//! Dedicated private SQLite store of the capture inbox (Design02 §5).
//!
//! One service instance owns one data directory: an OS file lock is taken at
//! startup and held for the whole service lifetime; a second writer is
//! rejected before it can register. note+digest+receipt live in one
//! transaction; the success reply is produced only after COMMIT. A corrupt
//! or readonly database is never silently rebuilt as an empty store.

use autoos_comm_core::admission::CaptureInput;
use autoos_comm_core::error::CommError;
use autoos_comm_core::Limits;
use rusqlite::{params, Connection, OpenFlags, OptionalExtension};
use sha2::{Digest, Sha256};
use std::io::Write;
use std::path::{Path, PathBuf};
use std::sync::Mutex;

/// Kill-position barrier for crash counterexamples (V-10). The marker file
/// is the barrier evidence: the test kills the process only after the marker
/// exists, never by guessing with sleeps.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Barrier {
    /// No barrier: normal operation.
    None,
    /// Pause inside the open transaction, before COMMIT (kill → rollback →
    /// no row, no receipt).
    BeforeCommit,
}

#[derive(Debug)]
pub enum StoreError {
    CapacityExhausted,
    OperationConflict,
    Corrupt(String),
    Readonly(String),
    Io(String),
}

impl StoreError {
    pub fn to_comm(&self) -> CommError {
        match self {
            StoreError::CapacityExhausted => CommError::resource("capture capacity exhausted")
                .with_execution(autoos_comm_core::ExecutionState::NotDispatched),
            StoreError::OperationConflict => {
                CommError::domain("operation_conflict", "same key with different payload")
            }
            StoreError::Corrupt(m) => CommError::storage(format!("corrupt database: {m}")),
            StoreError::Readonly(m) => CommError::storage(format!("readonly database: {m}")),
            StoreError::Io(m) => CommError::storage(format!("storage io: {m}")),
        }
    }
}

pub struct CaptureOk {
    pub capture_id: String,
    /// True when an identical (principal, operation_id, payload) row already
    /// existed and its original capture_id was returned without a new write.
    pub dedup: bool,
}

pub struct ReceiptHit {
    pub capture_id: String,
    pub text: String,
    pub digest: String,
    pub title: Option<String>,
    pub source_uri: Option<String>,
}

pub struct Store {
    conn: Mutex<Connection>,
    _lock: std::fs::File,
    pub db_path: PathBuf,
}

/// Deterministic payload digest over the raw UTF-8 content with explicit
/// field-length encoding (T-01 freeze). Absent optional fields and explicit
/// empty strings encode identically.
pub fn payload_digest(input: &CaptureInput) -> String {
    let mut h = Sha256::new();
    h.update(b"f01-capture/1\x00");
    h.update(input.operation_id.as_bytes());
    h.update(b"\x00");
    let title = input.title.clone().unwrap_or_default();
    let uri = input.source_uri.clone().unwrap_or_default();
    h.update((title.len() as u32).to_le_bytes());
    h.update(title.as_bytes());
    h.update((uri.len() as u32).to_le_bytes());
    h.update(uri.as_bytes());
    h.update((input.text.len() as u32).to_le_bytes());
    h.update(input.text.as_bytes());
    hex32(&h.finalize())
}

/// Deterministic capture id: derived from the dedup key so the same
/// key+payload maps to the same id across restarts and processes.
pub fn capture_id_for(principal: &str, operation_id: &str, digest_hex: &str) -> String {
    let mut h = Sha256::new();
    h.update(b"f01-capture-id/1\x00");
    h.update(principal.as_bytes());
    h.update(b"\x00");
    h.update(operation_id.as_bytes());
    h.update(b"\x00");
    h.update(digest_hex.as_bytes());
    let full = hex32(&h.finalize());
    full[..32].to_string()
}

fn hex32(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{b:02x}")).collect()
}

impl Store {
    /// Open (or create) the dedicated database in `db_dir`, taking the
    /// exclusive lifetime file lock. Corruption and readonly media are
    /// reported as errors; the store never recreates a damaged file.
    pub fn open(db_dir: &Path) -> Result<Store, StoreError> {
        std::fs::create_dir_all(db_dir).map_err(|e| StoreError::Io(e.to_string()))?;
        let db_path = db_dir.join("capture.db");
        // lifetime lock before any SQLite work
        let lock_path = db_dir.join("inbox.lock");
        let lock = std::fs::OpenOptions::new()
            .create(true)
            .write(true)
            .open(&lock_path)
            .map_err(|e| StoreError::Io(format!("lock file: {e}")))?;
        use std::fs::TryLockError;
        if let Err(e) = lock.try_lock() {
            match e {
                TryLockError::WouldBlock => {
                    return Err(StoreError::Io(
                        "another writer holds the inbox lock".to_string(),
                    ))
                }
                TryLockError::Error(err) => return Err(StoreError::Io(err.to_string())),
            }
        }
        let exists = db_path.exists();
        let conn = Connection::open_with_flags(
            &db_path,
            OpenFlags::SQLITE_OPEN_READ_WRITE
                | OpenFlags::SQLITE_OPEN_CREATE
                | OpenFlags::SQLITE_OPEN_NO_MUTEX,
        )
        .map_err(|e| StoreError::Io(e.to_string()))?;
        // probe first: a damaged file is reported, never rebuilt
        if exists {
            conn.execute_batch("SELECT count(*) FROM sqlite_master;")
                .map_err(|e| StoreError::Corrupt(e.to_string()))?;
        }
        // pragmas: WAL + FULL sync (T-01 freeze); fails fast on readonly media
        conn.execute_batch(
            "PRAGMA journal_mode=WAL; PRAGMA synchronous=FULL; PRAGMA busy_timeout=2000;",
        )
        .map_err(|e| StoreError::Readonly(e.to_string()))?;
        if exists {
            let size = std::fs::metadata(&db_path).map(|m| m.len()).unwrap_or(0);
            if size as i64 > Limits::MAX_DB_BYTES {
                return Err(StoreError::Io(format!(
                    "database file {size} exceeds profile limit {}",
                    Limits::MAX_DB_BYTES
                )));
            }
        }
        conn.execute_batch(
            "CREATE TABLE IF NOT EXISTS captures (
                principal      TEXT NOT NULL,
                operation_id   TEXT NOT NULL,
                capture_id     TEXT NOT NULL,
                payload_digest TEXT NOT NULL,
                title          TEXT,
                source_uri     TEXT,
                text           BLOB NOT NULL,
                created_at_ms  INTEGER NOT NULL,
                PRIMARY KEY (principal, operation_id)
            );
            CREATE UNIQUE INDEX IF NOT EXISTS idx_capture_id ON captures(capture_id);",
        )
        .map_err(|e| StoreError::Corrupt(e.to_string()))?;
        Ok(Store {
            conn: Mutex::new(conn),
            _lock: lock,
            db_path,
        })
    }

    /// Insert-or-lookup one capture in a single transaction.
    pub fn capture(
        &self,
        principal: &str,
        input: &CaptureInput,
        digest_hex: &str,
    ) -> Result<CaptureOk, StoreError> {
        let mut conn = self.conn.lock().unwrap();
        let tx = conn
            .transaction()
            .map_err(|e| StoreError::Io(e.to_string()))?;
        let count: i64 = tx
            .query_row("SELECT COUNT(*) FROM captures", [], |r| r.get(0))
            .map_err(|e| StoreError::Corrupt(e.to_string()))?;
        let existing: Option<(String, String)> = tx
            .query_row(
                "SELECT capture_id, payload_digest FROM captures WHERE principal = ?1 AND operation_id = ?2",
                params![principal, input.operation_id],
                |r| Ok((r.get(0)?, r.get(1)?)),
            )
            .optional()
            .map_err(|e| StoreError::Corrupt(e.to_string()))?;
        match existing {
            Some((capture_id, existing_digest)) => {
                if existing_digest == digest_hex {
                    tx.commit().map_err(|e| StoreError::Io(e.to_string()))?;
                    Ok(CaptureOk {
                        capture_id,
                        dedup: true,
                    })
                } else {
                    // conflict never overwrites; the transaction is read-only
                    tx.rollback().map_err(|e| StoreError::Io(e.to_string()))?;
                    Err(StoreError::OperationConflict)
                }
            }
            None => {
                if count >= Limits::MAX_CAPTURES {
                    tx.rollback().map_err(|e| StoreError::Io(e.to_string()))?;
                    return Err(StoreError::CapacityExhausted);
                }
                let capture_id = capture_id_for(principal, &input.operation_id, digest_hex);
                let now_ms = std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .map(|d| d.as_millis() as i64)
                    .unwrap_or(0);
                tx.execute(
                    "INSERT INTO captures (principal, operation_id, capture_id, payload_digest, title, source_uri, text, created_at_ms)
                     VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
                    params![
                        principal,
                        input.operation_id,
                        capture_id,
                        digest_hex,
                        input.title,
                        input.source_uri,
                        input.text.as_bytes(),
                        now_ms
                    ],
                )
                .map_err(|e| StoreError::Corrupt(e.to_string()))?;
                tx.commit().map_err(|e| StoreError::Io(e.to_string()))?;
                Ok(CaptureOk {
                    capture_id,
                    dedup: false,
                })
            }
        }
    }

    /// Commit-after barrier variant: performs the full transaction
    /// (committing) and then blocks forever before returning. Used to prove
    /// a committed capture survives a kill-before-reply.
    pub fn capture_with_after_commit_barrier(
        &self,
        principal: &str,
        input: &CaptureInput,
        digest_hex: &str,
        barrier_dir: &Path,
        operation_id: &str,
    ) -> Result<CaptureOk, StoreError> {
        self.capture(principal, input, digest_hex)?;
        write_barrier_marker(barrier_dir, operation_id, "after_commit_before_reply");
        block_forever()
    }

    /// Pause inside the open transaction right before COMMIT (barrier).
    pub fn capture_with_before_commit_barrier(
        &self,
        principal: &str,
        input: &CaptureInput,
        digest_hex: &str,
        barrier_dir: &Path,
    ) -> Result<CaptureOk, StoreError> {
        let mut conn = self.conn.lock().unwrap();
        let tx = conn
            .transaction()
            .map_err(|e| StoreError::Io(e.to_string()))?;
        let count: i64 = tx
            .query_row("SELECT COUNT(*) FROM captures", [], |r| r.get(0))
            .map_err(|e| StoreError::Corrupt(e.to_string()))?;
        let existing: Option<String> = tx
            .query_row(
                "SELECT capture_id FROM captures WHERE principal = ?1 AND operation_id = ?2",
                params![principal, input.operation_id],
                |r| r.get(0),
            )
            .optional()
            .map_err(|e| StoreError::Corrupt(e.to_string()))?;
        if existing.is_some() {
            tx.rollback().map_err(|e| StoreError::Io(e.to_string()))?;
            return Err(StoreError::OperationConflict);
        }
        if count >= Limits::MAX_CAPTURES {
            tx.rollback().map_err(|e| StoreError::Io(e.to_string()))?;
            return Err(StoreError::CapacityExhausted);
        }
        let capture_id = capture_id_for(principal, &input.operation_id, digest_hex);
        let now_ms = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_millis() as i64)
            .unwrap_or(0);
        tx.execute(
            "INSERT INTO captures (principal, operation_id, capture_id, payload_digest, title, source_uri, text, created_at_ms)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
            params![
                principal,
                input.operation_id,
                capture_id,
                digest_hex,
                input.title,
                input.source_uri,
                input.text.as_bytes(),
                now_ms
            ],
        )
        .map_err(|e| StoreError::Corrupt(e.to_string()))?;
        // barrier: marker flushed BEFORE kill; commit never runs
        write_barrier_marker(barrier_dir, &input.operation_id, "before_commit");
        block_forever()
    }

    /// Query the current committed result for (principal, operation_id).
    pub fn receipt(
        &self,
        principal: &str,
        operation_id: &str,
    ) -> Result<Option<ReceiptHit>, StoreError> {
        let conn = self.conn.lock().unwrap();
        let row = conn
            .query_row(
                "SELECT capture_id, payload_digest, title, source_uri, text FROM captures
                 WHERE principal = ?1 AND operation_id = ?2",
                params![principal, operation_id],
                |r| {
                    let text_bytes: Vec<u8> = r.get(4)?;
                    Ok((
                        r.get::<_, String>(0)?,
                        r.get::<_, String>(1)?,
                        r.get::<_, Option<String>>(2)?,
                        r.get::<_, Option<String>>(3)?,
                        text_bytes,
                    ))
                },
            )
            .optional()
            .map_err(|e| StoreError::Corrupt(e.to_string()))?;
        Ok(
            row.map(|(capture_id, digest, title, source_uri, text_bytes)| {
                let text = String::from_utf8_lossy(&text_bytes).to_string();
                ReceiptHit {
                    capture_id,
                    text,
                    digest,
                    title,
                    source_uri,
                }
            }),
        )
    }

    /// Row count (used by independent verification helpers in tests).
    pub fn count(&self) -> Result<i64, StoreError> {
        let conn = self.conn.lock().unwrap();
        conn.query_row("SELECT COUNT(*) FROM captures", [], |r| r.get(0))
            .map_err(|e| StoreError::Corrupt(e.to_string()))
    }
}

/// Write the barrier marker file (flushed to disk before the block).
fn write_barrier_marker(dir: &Path, operation_id: &str, kind: &str) {
    let path = dir.join(format!("barrier-{operation_id}.json"));
    let body = format!("{{\"barrier\":\"{kind}\",\"pid\":{}}}", std::process::id());
    if let Ok(mut f) = std::fs::File::create(&path) {
        let _ = f.write_all(body.as_bytes());
        let _ = f.sync_all();
    }
}

/// Block the calling thread forever (the test controller kills the process).
fn block_forever() -> ! {
    loop {
        std::thread::park();
        // if spuriously unparked, keep blocking
        std::thread::sleep(std::time::Duration::from_secs(3600));
    }
}
