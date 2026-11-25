use std::io;

use sonoro_common::fs::FileSystem;

pub fn next(fs: &FileSystem, counter_file: &str) -> io::Result<i32> {
    next_starting_at(fs, counter_file, 1)
}

pub fn next_starting_at(
    fs: &FileSystem,
    counter_file: &str,
    start_id: i32,
) -> io::Result<i32> {
    fs.modify_file(counter_file, |existing| {
        if let Some(bytes) = existing {
            let text = std::str::from_utf8(bytes)
                .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
            let id_str = text
                .split_whitespace()
                .next()
                .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "empty counter file"))?;
            let id: i32 = id_str
                .parse()
                .map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "invalid counter int"))?;
            let next_bytes = format!("{}\n", id + 1).into_bytes();
            Ok((Some(next_bytes), id))
        } else {
            let next_bytes = format!("{}\n", start_id + 1).into_bytes();
            Ok((Some(next_bytes), start_id))
        }
    })
}
