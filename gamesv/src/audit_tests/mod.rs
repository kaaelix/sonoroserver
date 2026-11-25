mod part1;
mod part2;

use std::fs;
use std::path::PathBuf;
use std::sync::{Arc, OnceLock};
use std::time::{SystemTime, UNIX_EPOCH};

use sonoro_common::fs::FileSystem;
use sonoro_gamesv_data::Assets;
use sonoro_gamesv_logic::scene::Scene;
use sonoro_proto::PbMessage;
use tokio::sync::{Notify, mpsc};

use crate::network::connection::Connection;
use crate::network::message::{Header, Message};
use crate::network::state::{PlayerComponentStorage, State};

pub struct TempDirGuard {
    pub path: PathBuf,
}

impl TempDirGuard {
    pub fn new(prefix: &str) -> Self {
        let nanos = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_or(0, |d| d.as_nanos());
        let path = std::env::temp_dir().join(format!("{prefix}_{}_{}", std::process::id(), nanos));
        fs::create_dir_all(&path).expect("create temp dir");
        Self { path }
    }
}

impl Drop for TempDirGuard {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.path);
    }
}

pub fn shared_assets() -> Arc<Assets> {
    static ASSETS: OnceLock<Arc<Assets>> = OnceLock::new();
    Arc::clone(ASSETS.get_or_init(|| {
        let bindata_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../assets/BinData")
            .to_string_lossy()
            .into_owned();
        Arc::new(Assets::load(&bindata_dir).expect("load workspace assets/BinData"))
    }))
}

pub struct TestHarness {
    pub tmp: TempDirGuard,
    pub rx: mpsc::Receiver<Vec<u8>>,
    pub state: State,
}

impl TestHarness {
    pub fn new(prefix: &str, map_id: Option<i32>) -> Self {
        let tmp = TempDirGuard::new(prefix);
        let assets = shared_assets();
        let fs = Arc::new(FileSystem::open(&tmp.path).expect("open test fs"));
        let (tx, rx) = mpsc::channel(16_384);
        let conn = Connection::new(tx, Arc::new(Notify::new()));
        let pcs = PlayerComponentStorage::init(&fs, &assets, 1).expect("init pcs");
        let mut state = State::new(fs, conn, assets, pcs, 1);
        if let Some(mid) = map_id {
            state.scene = Some(Scene::init(&state.fs, 1, mid).expect("init scene"));
        }
        Self { tmp, rx, state }
    }

    pub fn drain_count(&mut self) -> usize {
        let mut count = 0;
        while self.rx.try_recv().is_ok() {
            count += 1;
        }
        count
    }

    pub fn drain_push_msg_ids(&mut self) -> Vec<u16> {
        let mut ids = Vec::new();
        while let Ok(frame) = self.rx.try_recv() {
            if frame.len() < 3 {
                continue;
            }
            if let Ok(msg) = Message::decode(&frame[3..])
                && matches!(msg.header, Header::Push(_))
            {
                ids.push(msg.header.message_id());
            }
        }
        ids
    }

    pub fn drain_decoded<M: PbMessage>(&mut self, msg_id: u16) -> Vec<M> {
        let mut out = Vec::new();
        while let Ok(frame) = self.rx.try_recv() {
            if frame.len() < 3 {
                continue;
            }
            if let Ok(msg) = Message::decode(&frame[3..])
                && msg.header.message_id() == msg_id
                && let Ok(decoded) = M::decode(&msg.payload)
            {
                out.push(decoded);
            }
        }
        out
    }
}
