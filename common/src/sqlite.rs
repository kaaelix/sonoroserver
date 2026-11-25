//! Cross-platform bundled SQLite wrapper matching the original `common.sqlite` pragmas.

use std::path::Path;

use rusqlite::{Connection, OpenFlags};
use thiserror::Error;

/// Errors returned by SQLite operations.
#[derive(Debug, Error)]
pub enum SqliteError {
    #[error("sqlite error: {0}")]
    Rusqlite(#[from] rusqlite::Error),
}

/// Thin wrapper around a `rusqlite::Connection` configured with SonoroServer defaults.
pub struct Db {
    conn: Connection,
}

impl Db {
    /// Opens (or creates) a read-write SQLite database with WAL mode and busy timeout.
    pub fn open(path: impl AsRef<Path>) -> Result<Self, SqliteError> {
        let conn = Connection::open(path)?;
        conn.execute_batch(
            "PRAGMA journal_mode=WAL; PRAGMA synchronous=NORMAL; PRAGMA busy_timeout=5000; PRAGMA cache_size=-16384; PRAGMA temp_store=MEMORY; PRAGMA mmap_size=67108864;",
        )?;
        Ok(Self { conn })
    }

    /// Opens an existing SQLite database in read-only mode with cache and mmap tuning.
    pub fn open_read_only(path: impl AsRef<Path>) -> Result<Self, SqliteError> {
        let conn = Connection::open_with_flags(path, OpenFlags::SQLITE_OPEN_READ_ONLY)?;
        conn.execute_batch(
            "PRAGMA cache_size=-16384; PRAGMA temp_store=MEMORY; PRAGMA mmap_size=67108864;",
        )?;
        Ok(Self { conn })
    }

    /// Executes one or more SQL statements without parameters.
    pub fn exec(&self, sql: &str) -> Result<(), SqliteError> {
        self.conn.execute_batch(sql)?;
        Ok(())
    }

    /// Exposes the underlying `rusqlite::Connection` for prepared statements and transactions.
    pub fn connection(&self) -> &Connection {
        &self.conn
    }

    /// Exposes a mutable reference to the underlying `rusqlite::Connection`.
    pub fn connection_mut(&mut self) -> &mut Connection {
        &mut self.conn
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn opens_in_memory_and_runs_sql() {
        let db = Db::open(":memory:").expect("open in-memory db");
        db.exec(
            "CREATE TABLE t (id INTEGER PRIMARY KEY, name TEXT); INSERT INTO t VALUES (1, 'a');",
        )
        .expect("exec sql");
        let name: String = db
            .connection()
            .query_row("SELECT name FROM t WHERE id = 1", [], |row| row.get(0))
            .expect("query row");
        assert_eq!(name, "a");
    }
}
