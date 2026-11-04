//! Frozen first-verification profile limits (Design02 §6) and body-shape
//! admission for the capture schemas. These numbers are the proposed
//! first-verification profile frozen by T-01; loosening them to make a test
//! pass is forbidden without a plan revision.

use crate::error::CommError;
use crate::ids;
use std::time::Duration;

pub struct Limits;

#[allow(dead_code)]
impl Limits {
    // wire
    pub const MAX_FRAME: u32 = 65536;
    pub const MAX_DEPTH: usize = 16;
    // fields
    pub const MAX_TEXT: usize = 16384;
    pub const MAX_TITLE: usize = 256;
    pub const MAX_SOURCE_URI: usize = 2048;
    pub const MAX_DESCRIPTOR: usize = 4096;
    pub const MAX_PRINCIPAL: usize = 128;
    // services & connections
    pub const MAX_AUTH_CONNS: usize = 8;
    pub const MAX_INFLIGHT_PER_CONN: usize = 8;
    pub const MAX_INFLIGHT_GLOBAL: usize = 32;
    pub const MAX_PROVIDER_QUEUE: usize = 16;
    // time
    pub const HANDSHAKE_DEADLINE: Duration = Duration::from_secs(2);
    /// Once a frame's length header has arrived, its body must complete
    /// within this bound. Idle gaps *between* frames are not adversarial and
    /// are not bounded (F1 has no keepalive).
    pub const FRAME_BODY_DEADLINE: Duration = Duration::from_secs(2);
    pub const WRITE_DEADLINE: Duration = Duration::from_secs(2);
    pub const DEFAULT_TIMEOUT_MS: u32 = 2000;
    pub const MIN_TIMEOUT_MS: u32 = 1;
    pub const MAX_TIMEOUT_MS: u32 = 5000;
    pub const SHUTDOWN_DEADLINE: Duration = Duration::from_secs(2);
    // inbox
    pub const MAX_CAPTURES: i64 = 1024;
    pub const MAX_DB_BYTES: i64 = 32 * 1024 * 1024;
    // test/demo budgets
    pub const SINGLE_CASE_BUDGET: Duration = Duration::from_secs(20);
    pub const TOTAL_BUDGET: Duration = Duration::from_secs(300);
    pub const MAX_PROC_OUTPUT: u64 = 1024 * 1024;
}

/// Validate a caller-requested call timeout.
pub fn validate_timeout_ms(t: u32) -> Result<(), CommError> {
    if (Limits::MIN_TIMEOUT_MS..=Limits::MAX_TIMEOUT_MS).contains(&t) {
        Ok(())
    } else {
        Err(CommError::schema(format!(
            "timeout_ms {t} outside [{}, {}]",
            Limits::MIN_TIMEOUT_MS,
            Limits::MAX_TIMEOUT_MS
        )))
    }
}

/// Validated `capture` request body.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CaptureInput {
    pub operation_id: String,
    pub text: String,
    pub title: Option<String>,
    pub source_uri: Option<String>,
}

/// Validated `receipt` request body.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReceiptInput {
    pub operation_id: String,
    /// Must equal the trusted caller principal (cross-principal access is a
    /// `permission_denied` rejection, never a scoped silent miss).
    pub principal: String,
}

fn body_object(
    body: &serde_json::Value,
) -> Result<&serde_json::Map<String, serde_json::Value>, CommError> {
    body.as_object()
        .ok_or_else(|| CommError::schema("body must be a JSON object"))
}

fn body_str(
    obj: &serde_json::Map<String, serde_json::Value>,
    key: &str,
    min_bytes: usize,
    max_bytes: usize,
) -> Result<String, CommError> {
    let s = obj
        .get(key)
        .ok_or_else(|| CommError::schema(format!("missing body field `{key}`")))?
        .as_str()
        .ok_or_else(|| CommError::schema(format!("body field `{key}` must be a string")))?;
    let n = s.len();
    if n < min_bytes || n > max_bytes {
        return Err(CommError::schema(format!(
            "body field `{key}` length {n} outside [{min_bytes}, {max_bytes}]"
        )));
    }
    Ok(s.to_string())
}

fn body_opt_str(
    obj: &serde_json::Map<String, serde_json::Value>,
    key: &str,
    max_bytes: usize,
) -> Result<Option<String>, CommError> {
    match obj.get(key) {
        None => Ok(None),
        Some(v) => {
            let s = v
                .as_str()
                .ok_or_else(|| CommError::schema(format!("body field `{key}` must be a string")))?;
            if s.len() > max_bytes {
                return Err(CommError::schema(format!(
                    "body field `{key}` length {} exceeds {max_bytes}",
                    s.len()
                )));
            }
            Ok(Some(s.to_string()))
        }
    }
}

fn check_body_keys(
    obj: &serde_json::Map<String, serde_json::Value>,
    allowed: &[&str],
) -> Result<(), CommError> {
    for k in obj.keys() {
        if !allowed.contains(&k.as_str()) {
            return Err(CommError::schema(format!("unknown body field `{k}`")));
        }
    }
    Ok(())
}

/// Validate a `capture` body against the frozen `capture.inbox.capture/1`
/// schema. Absent optional fields and explicit empty strings carry the same
/// digest semantics (T-01 freeze): both encode as the empty string.
pub fn validate_capture_body(body: &serde_json::Value) -> Result<CaptureInput, CommError> {
    let obj = body_object(body)?;
    check_body_keys(obj, &["operation_id", "text", "title", "source_uri"])?;
    let operation_id = body_str(obj, "operation_id", ids::ID_LEN, ids::ID_LEN)?;
    if !ids::is_id_text(&operation_id) {
        return Err(CommError::schema("operation_id must be 128-bit id text"));
    }
    let text = body_str(obj, "text", 1, Limits::MAX_TEXT)?;
    let title = body_opt_str(obj, "title", Limits::MAX_TITLE)?;
    let source_uri = body_opt_str(obj, "source_uri", Limits::MAX_SOURCE_URI)?;
    Ok(CaptureInput {
        operation_id,
        text,
        title,
        source_uri,
    })
}

/// Validate a `receipt` body against `capture.inbox.receipt/1`.
pub fn validate_receipt_body(body: &serde_json::Value) -> Result<ReceiptInput, CommError> {
    let obj = body_object(body)?;
    check_body_keys(obj, &["operation_id", "principal"])?;
    let operation_id = body_str(obj, "operation_id", ids::ID_LEN, ids::ID_LEN)?;
    if !ids::is_id_text(&operation_id) {
        return Err(CommError::schema("operation_id must be 128-bit id text"));
    }
    let principal = body_str(obj, "principal", 1, Limits::MAX_PRINCIPAL)?;
    Ok(ReceiptInput {
        operation_id,
        principal,
    })
}

/// Known schema ids of this profile; anything else is a pre-dispatch reject.
pub fn known_schema(schema_id: &str) -> bool {
    matches!(
        schema_id,
        crate::protocol::SCHEMA_CAPTURE_CAPTURE | crate::protocol::SCHEMA_CAPTURE_RECEIPT
    )
}

/// Known method names of this profile.
pub fn known_method(method: &str) -> bool {
    matches!(method, "capture" | "receipt")
}
