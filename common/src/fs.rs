//! Thread-safe cached atomic filesystem abstraction over a state directory.

use std::collections::HashMap;
use std::fs::{self, File};
use std::io::{self, Write};
use std::path::{Component, Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, Condvar, Mutex, RwLock};
use std::thread::JoinHandle;
use std::time::SystemTime;

/// Operation forwarded to an optional persistence sync hook (e.g., SQLite mirroring).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SyncOp {
    Write,
    DeleteFile,
    DeleteTree,
}

/// Callback invoked after mutating filesystem operations succeed.
pub type SyncHook = Arc<dyn Fn(SyncOp, &str, &[u8]) + Send + Sync>;

/// Entry returned by [`FileSystem::read_dir`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DirEntry {
    pub path: String,
    pub is_dir: bool,
}

impl DirEntry {
    /// Returns the final path segment (basename).
    pub fn basename(&self) -> &str {
        self.path.rsplit('/').next().unwrap_or(self.path.as_str())
    }
}

#[derive(Default)]
struct FileEntry {
    content: Option<Vec<u8>>,
    mtime: Option<SystemTime>,
    dirty: bool,
    revision: u64,
}

struct PendingWrite {
    full_path: PathBuf,
    content: Vec<u8>,
    revision: u64,
}

#[derive(Default)]
struct WritebackState {
    pending: HashMap<String, PendingWrite>,
    flushing: bool,
    closed: bool,
}

struct WritebackCoordinator {
    state: Mutex<WritebackState>,
    work_cv: Condvar,
    idle_cv: Condvar,
}

/// Cross-platform cached state filesystem rooted at `state_dir`.
pub struct FileSystem {
    root_dir: PathBuf,
    entries: Arc<RwLock<HashMap<String, Arc<Mutex<FileEntry>>>>>,
    sync_hook: RwLock<Option<SyncHook>>,
    writeback: Arc<WritebackCoordinator>,
    worker: Mutex<Option<JoinHandle<()>>>,
}

static TMP_SEQ: AtomicU64 = AtomicU64::new(0);

impl FileSystem {
    /// Opens an existing `root_path` directory as the state root.
    pub fn open(root_path: impl AsRef<Path>) -> io::Result<Self> {
        let root_dir = root_path.as_ref().to_path_buf();
        let meta = fs::metadata(&root_dir)?;
        if !meta.is_dir() {
            return Err(io::Error::new(
                io::ErrorKind::NotADirectory,
                "state path is not a directory",
            ));
        }
        let entries = Arc::new(RwLock::new(HashMap::new()));
        let writeback = Arc::new(WritebackCoordinator {
            state: Mutex::new(WritebackState::default()),
            work_cv: Condvar::new(),
            idle_cv: Condvar::new(),
        });
        let worker_wb = Arc::clone(&writeback);
        let worker_entries = Arc::clone(&entries);
        let handle = std::thread::Builder::new()
            .name("fs-writeback".to_owned())
            .spawn(move || writeback_loop(worker_wb, worker_entries))?;
        Ok(Self {
            root_dir,
            entries,
            sync_hook: RwLock::new(None),
            writeback,
            worker: Mutex::new(Some(handle)),
        })
    }

    /// Waits until all queued write-behind disk writes have been written.
    pub fn flush(&self) {
        self.flush_writeback();
    }

    fn enqueue_write(&self, key: String, full_path: PathBuf, content: Vec<u8>, revision: u64) {
        let mut st = self
            .writeback
            .state
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner());
        st.pending.insert(
            key,
            PendingWrite {
                full_path,
                content,
                revision,
            },
        );
        self.writeback.work_cv.notify_one();
    }

    fn flush_writeback(&self) {
        let mut st = self
            .writeback
            .state
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner());
        if st.pending.is_empty() && !st.flushing {
            return;
        }
        if !st.pending.is_empty() {
            self.writeback.work_cv.notify_one();
        }
        while !st.pending.is_empty() || st.flushing {
            st = self
                .writeback
                .idle_cv
                .wait(st)
                .unwrap_or_else(|poisoned| poisoned.into_inner());
        }
    }

    /// Returns the state root path.
    pub fn root_dir(&self) -> &Path {
        &self.root_dir
    }

    /// Registers a hook invoked on writes and deletes.
    pub fn set_sync_hook(&self, hook: SyncHook) {
        if let Ok(mut guard) = self.sync_hook.write() {
            *guard = Some(hook);
        }
    }

    fn notify_hook(&self, op: SyncOp, rel_path: &str, content: &[u8]) {
        if let Ok(guard) = self.sync_hook.read()
            && let Some(hook) = guard.as_ref()
        {
            hook(op, rel_path, content);
        }
    }

    fn resolve_relative(&self, rel_path: &str) -> io::Result<(String, PathBuf)> {
        if rel_path.contains('\0') {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "path contains NUL byte",
            ));
        }

        let normalized = rel_path.replace('\\', "/");
        let path = Path::new(&normalized);
        if path.is_absolute() {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "absolute paths are not allowed",
            ));
        }

        let mut clean_parts = Vec::new();
        let mut full = self.root_dir.clone();
        for comp in path.components() {
            match comp {
                Component::Normal(seg) => {
                    let s = seg.to_string_lossy();
                    clean_parts.push(s.into_owned());
                    full.push(seg);
                }
                Component::CurDir => {}
                Component::ParentDir | Component::RootDir | Component::Prefix(_) => {
                    return Err(io::Error::new(
                        io::ErrorKind::InvalidInput,
                        "path traversal is not allowed",
                    ));
                }
            }
        }

        if clean_parts.is_empty() {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "empty relative path",
            ));
        }

        Ok((clean_parts.join("/"), full))
    }

    fn get_entry(&self, key: &str) -> Option<Arc<Mutex<FileEntry>>> {
        self.entries
            .read()
            .ok()
            .and_then(|map| map.get(key).cloned())
    }

    fn get_or_create_entry(&self, key: &str) -> Arc<Mutex<FileEntry>> {
        if let Some(existing) = self.get_entry(key) {
            return existing;
        }
        let mut map = self
            .entries
            .write()
            .unwrap_or_else(|poisoned| poisoned.into_inner());
        map.entry(key.to_string())
            .or_insert_with(|| Arc::new(Mutex::new(FileEntry::default())))
            .clone()
    }

    /// Reads a file relative to the state root, using an mtime-validated in-memory cache.
    pub fn read_file(&self, rel_path: &str) -> io::Result<Option<Vec<u8>>> {
        let (key, full_path) = self.resolve_relative(rel_path)?;
        let entry_arc = match self.get_entry(&key) {
            Some(entry) => entry,
            None => match fs::metadata(&full_path) {
                Ok(_) => self.get_or_create_entry(&key),
                Err(err) if err.kind() == io::ErrorKind::NotFound => return Ok(None),
                Err(err) => return Err(err),
            },
        };

        let mut entry = entry_arc
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner());

        if entry.dirty && entry.content.is_some() {
            return Ok(entry.content.clone());
        }

        match fs::metadata(&full_path) {
            Ok(meta) => {
                let disk_mtime = meta.modified().ok();
                let is_fresh = match (&entry.content, entry.mtime, disk_mtime) {
                    (Some(_), Some(cached_t), Some(disk_t)) => disk_t <= cached_t,
                    (Some(_), None, None) => true,
                    _ => false,
                };

                if is_fresh {
                    return Ok(entry.content.clone());
                }

                match fs::read(&full_path) {
                    Ok(bytes) => {
                        entry.mtime = disk_mtime;
                        entry.content = Some(bytes.clone());
                        Ok(Some(bytes))
                    }
                    Err(err) if err.kind() == io::ErrorKind::NotFound => {
                        entry.content = None;
                        entry.mtime = None;
                        Ok(None)
                    }
                    Err(err) => Err(err),
                }
            }
            Err(err) if err.kind() == io::ErrorKind::NotFound => {
                entry.content = None;
                entry.mtime = None;
                Ok(None)
            }
            Err(err) => Err(err),
        }
    }

    /// Queues `content` for coalesced atomic write-behind, skipping identical writes.
    pub fn write_file(&self, rel_path: &str, content: &[u8]) -> io::Result<()> {
        let (key, full_path) = self.resolve_relative(rel_path)?;
        let entry_arc = self.get_or_create_entry(&key);
        let mut entry = entry_arc
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner());

        if let Some(existing) = entry.content.as_deref()
            && existing == content
        {
            return Ok(());
        }

        validate_writable_target(&full_path)?;

        let owned = content.to_vec();
        entry.content = Some(owned.clone());
        entry.mtime = Some(SystemTime::now());
        entry.dirty = true;
        entry.revision = entry.revision.wrapping_add(1);
        let revision = entry.revision;
        drop(entry);

        self.enqueue_write(key.clone(), full_path, owned, revision);
        self.notify_hook(SyncOp::Write, &key, content);
        Ok(())
    }

    /// Atomically reads, optionally updates, and queues write-behind while holding its entry lock.
    pub fn modify_file<R, F>(&self, rel_path: &str, f: F) -> io::Result<R>
    where
        F: FnOnce(Option<&[u8]>) -> io::Result<(Option<Vec<u8>>, R)>,
    {
        let (key, full_path) = self.resolve_relative(rel_path)?;
        let entry_arc = self.get_or_create_entry(&key);
        let mut entry = entry_arc
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner());

        let current: Option<Vec<u8>> = if entry.dirty && entry.content.is_some() {
            entry.content.clone()
        } else {
            match fs::metadata(&full_path) {
                Ok(meta) => {
                    let disk_mtime = meta.modified().ok();
                    let is_fresh = match (&entry.content, entry.mtime, disk_mtime) {
                        (Some(_), Some(cached_t), Some(disk_t)) => disk_t <= cached_t,
                        (Some(_), None, None) => true,
                        _ => false,
                    };
                    if is_fresh {
                        entry.content.clone()
                    } else {
                        match fs::read(&full_path) {
                            Ok(bytes) => {
                                entry.mtime = disk_mtime;
                                entry.content = Some(bytes.clone());
                                Some(bytes)
                            }
                            Err(err) if err.kind() == io::ErrorKind::NotFound => {
                                entry.content = None;
                                entry.mtime = None;
                                None
                            }
                            Err(err) => return Err(err),
                        }
                    }
                }
                Err(err) if err.kind() == io::ErrorKind::NotFound => {
                    entry.content = None;
                    entry.mtime = None;
                    None
                }
                Err(err) => return Err(err),
            }
        };

        let (next_opt, ret) = f(current.as_deref())?;
        if let Some(next_bytes) = next_opt
            && entry.content.as_deref() != Some(next_bytes.as_slice())
        {
            validate_writable_target(&full_path)?;
            entry.content = Some(next_bytes.clone());
            entry.mtime = Some(SystemTime::now());
            entry.dirty = true;
            entry.revision = entry.revision.wrapping_add(1);
            let revision = entry.revision;
            drop(entry);
            self.enqueue_write(key.clone(), full_path, next_bytes.clone(), revision);
            self.notify_hook(SyncOp::Write, &key, &next_bytes);
        }
        Ok(ret)
    }

    /// Deletes a single file relative to the state root, ignoring missing files.
    pub fn delete_file(&self, rel_path: &str) -> io::Result<()> {
        let (key, full_path) = self.resolve_relative(rel_path)?;
        self.flush_writeback();
        if let Some(entry_arc) = self.get_entry(&key) {
            let mut entry = entry_arc
                .lock()
                .unwrap_or_else(|poisoned| poisoned.into_inner());
            entry.content = None;
            entry.mtime = None;
            entry.dirty = false;
            entry.revision = entry.revision.wrapping_add(1);
        }

        match fs::remove_file(&full_path) {
            Ok(()) => {}
            Err(err) if err.kind() == io::ErrorKind::NotFound => {}
            Err(err) => return Err(err),
        }

        self.notify_hook(SyncOp::DeleteFile, &key, &[]);
        Ok(())
    }

    /// Recursively deletes a directory tree relative to the state root.
    pub fn delete_tree(&self, rel_path: &str) -> io::Result<()> {
        let (key, full_path) = self.resolve_relative(rel_path)?;
        self.flush_writeback();
        let matching: Vec<Arc<Mutex<FileEntry>>> = {
            let prefix = format!("{key}/");
            let map = self
                .entries
                .read()
                .unwrap_or_else(|poisoned| poisoned.into_inner());
            map.iter()
                .filter(|(k, _)| *k == &key || k.starts_with(&prefix))
                .map(|(_, v)| v.clone())
                .collect()
        };

        for entry_arc in matching {
            let mut entry = entry_arc
                .lock()
                .unwrap_or_else(|poisoned| poisoned.into_inner());
            entry.content = None;
            entry.mtime = None;
            entry.dirty = false;
            entry.revision = entry.revision.wrapping_add(1);
        }

        match fs::remove_dir_all(&full_path) {
            Ok(()) => {}
            Err(err) if err.kind() == io::ErrorKind::NotFound => {}
            Err(err) => return Err(err),
        }

        self.notify_hook(SyncOp::DeleteTree, &key, &[]);
        Ok(())
    }

    /// Recursively lists files and directories under `rel_path`.
    pub fn read_dir(&self, rel_path: &str) -> io::Result<Option<Vec<DirEntry>>> {
        let (key, full_path) = self.resolve_relative(rel_path)?;
        self.flush_writeback();
        match fs::metadata(&full_path) {
            Ok(meta) if meta.is_dir() => {}
            Ok(_) => {
                return Err(io::Error::new(
                    io::ErrorKind::NotADirectory,
                    "target is not a directory",
                ));
            }
            Err(err) if err.kind() == io::ErrorKind::NotFound => return Ok(None),
            Err(err) => return Err(err),
        }

        let mut entries = Vec::new();
        walk_dir(&full_path, &key, &mut entries)?;
        Ok(Some(entries))
    }
}

impl Drop for FileSystem {
    fn drop(&mut self) {
        {
            let mut st = self
                .writeback
                .state
                .lock()
                .unwrap_or_else(|poisoned| poisoned.into_inner());
            st.closed = true;
            self.writeback.work_cv.notify_all();
        }
        if let Some(handle) = self
            .worker
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner())
            .take()
        {
            let _ = handle.join();
        }
    }
}

fn validate_writable_target(full_path: &Path) -> io::Result<()> {
    match fs::symlink_metadata(full_path) {
        Ok(meta) if meta.is_dir() => Err(io::Error::new(
            io::ErrorKind::IsADirectory,
            "target path is a directory",
        )),
        Ok(_) => Ok(()),
        Err(err) if err.kind() == io::ErrorKind::NotFound => Ok(()),
        Err(err) => Err(err),
    }
}

fn writeback_loop(
    wb: Arc<WritebackCoordinator>,
    entries: Arc<RwLock<HashMap<String, Arc<Mutex<FileEntry>>>>>,
) {
    loop {
        let batch = {
            let mut st = wb.state.lock().unwrap_or_else(|p| p.into_inner());
            while st.pending.is_empty() && !st.closed {
                st = wb.work_cv.wait(st).unwrap_or_else(|p| p.into_inner());
            }
            if st.pending.is_empty() && st.closed {
                wb.idle_cv.notify_all();
                break;
            }
            st.flushing = true;
            std::mem::take(&mut st.pending)
        };

        for (key, op) in batch {
            if atomic_write(&op.full_path, &op.content).is_ok()
                && let Some(entry_arc) = entries.read().ok().and_then(|m| m.get(&key).cloned())
            {
                let mut entry = entry_arc.lock().unwrap_or_else(|p| p.into_inner());
                if entry.revision == op.revision {
                    entry.dirty = false;
                }
            }
        }

        let mut st = wb.state.lock().unwrap_or_else(|p| p.into_inner());
        st.flushing = false;
        if st.pending.is_empty() {
            wb.idle_cv.notify_all();
        }
    }
}

fn walk_dir(abs_dir: &Path, rel_prefix: &str, out: &mut Vec<DirEntry>) -> io::Result<()> {
    for item in fs::read_dir(abs_dir)? {
        let item = item?;
        let file_name = item.file_name();
        let name = file_name.to_string_lossy();
        let child_rel = format!("{rel_prefix}/{name}");
        let is_dir = item.file_type()?.is_dir();
        out.push(DirEntry {
            path: child_rel.clone(),
            is_dir,
        });
        if is_dir {
            walk_dir(&item.path(), &child_rel, out)?;
        }
    }
    Ok(())
}

fn atomic_write(target: &Path, content: &[u8]) -> io::Result<()> {
    let parent = target
        .parent()
        .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidInput, "target path has no parent"))?;
    fs::create_dir_all(parent)?;

    let file_name = target.file_name().and_then(|s| s.to_str()).unwrap_or("tmp");
    let seq = TMP_SEQ.fetch_add(1, Ordering::Relaxed);
    let pid = std::process::id();
    let tmp_path = parent.join(format!(".{file_name}.{pid}.{seq}.tmp"));

    let write_res = (|| -> io::Result<()> {
        let mut file = File::create(&tmp_path)?;
        file.write_all(content)?;
        Ok(())
    })();

    if let Err(err) = write_res {
        let _ = fs::remove_file(&tmp_path);
        return Err(err);
    }

    if let Err(err) = fs::rename(&tmp_path, target) {
        let _ = fs::remove_file(&tmp_path);
        return Err(err);
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::{SystemTime, UNIX_EPOCH};

    fn temp_dir(label: &str) -> PathBuf {
        let nanos = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("clock")
            .as_nanos();
        let dir = std::env::temp_dir().join(format!("sonoro_fs_test_{label}_{nanos}"));
        fs::create_dir_all(&dir).expect("create temp dir");
        dir
    }

    #[test]
    fn writes_reads_and_blocks_traversal() {
        let dir = temp_dir("basic");
        let fs_state = FileSystem::open(&dir).expect("open fs");

        fs_state
            .write_file("account/123", b"hello")
            .expect("write file");
        assert_eq!(
            fs_state.read_file("account/123").expect("read file"),
            Some(b"hello".to_vec())
        );
        assert!(fs_state.read_file("../etc/passwd").is_err());
        assert!(fs_state.read_file("missing").expect("missing").is_none());

        fs_state.delete_file("account/123").expect("delete file");
        assert!(
            fs_state
                .read_file("account/123")
                .expect("after del")
                .is_none()
        );

        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn coalesces_write_behind_and_flushes_latest_state_to_disk() {
        let dir = temp_dir("writeback");
        let fs_state = FileSystem::open(&dir).expect("open fs");

        fs_state
            .write_file("player/100000001/scene", b"v1")
            .expect("write v1");
        fs_state
            .write_file("player/100000001/scene", b"v2")
            .expect("write v2");
        assert_eq!(
            fs_state
                .read_file("player/100000001/scene")
                .expect("read cached"),
            Some(b"v2".to_vec())
        );

        fs_state.flush();
        let disk_bytes = fs::read(dir.join("player/100000001/scene")).expect("disk bytes");
        assert_eq!(disk_bytes, b"v2");

        let _ = fs::remove_dir_all(&dir);
    }
}
