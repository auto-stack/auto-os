//! Capture inbox domain logic: method dispatch, schema admission (re-checked
//! provider-side), digest computation, receipt isolation and crash barriers.

use crate::store::{Store, StoreError};
use autoos_comm_core::admission::{validate_capture_body, validate_receipt_body, CaptureInput};
use autoos_comm_core::error::CommError;
use autoos_comm_core::protocol::{
    CallToProvider, SCHEMA_CAPTURE_CAPTURE, SCHEMA_CAPTURE_RECEIPT, SERVICE_ID_CAPTURE,
};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// Static service descriptor registered with the broker.
pub const DESCRIPTOR: &str = "capture.inbox/1 ops=capture,receipt private-db=WAL,full cap=1024";

/// Service configuration (from the run's private directory).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CaptureServiceConfig {
    pub run_dir: String,
    /// File containing this provider's 256-bit credential text.
    pub token_file: String,
    /// Broker endpoint file (endpoint.json).
    pub endpoint_file: String,
    /// Stop flag file: when present, the process exits 0 gracefully.
    #[serde(default)]
    pub stop_file: Option<String>,
    /// Crash-barrier position for counterexample tests; never set in demos.
    #[serde(default)]
    pub barrier: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct CaptureCommitted {
    pub status: &'static str, // "committed"
    pub capture_id: String,
    pub digest: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct ReceiptBody {
    /// "committed" with the stored fields, or "not_observed" (which never
    /// proves an in-flight request will not commit later).
    pub status: &'static str,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capture_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub digest: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_uri: Option<String>,
}

pub struct CaptureService {
    pub store: Store,
    pub barrier: BarrierPosition,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum BarrierPosition {
    #[default]
    None,
    BeforeCommit,
    AfterCommitBeforeReply,
}

impl BarrierPosition {
    pub fn from_config(v: Option<&str>) -> BarrierPosition {
        match v {
            Some("before_commit") => BarrierPosition::BeforeCommit,
            Some("after_commit_before_reply") => BarrierPosition::AfterCommitBeforeReply,
            _ => BarrierPosition::None,
        }
    }
}

pub fn service_id() -> &'static str {
    SERVICE_ID_CAPTURE
}

/// Handle one forwarded call. The trusted caller principal comes from the
/// broker-forwarded `caller` field (bound at Hello), never from the body.
pub async fn handle_call(
    service: &CaptureService,
    call: &CallToProvider,
    barrier_dir: PathBuf,
) -> Result<serde_json::Value, CommError> {
    let body = match call.method.as_str() {
        "capture" if call.schema_id == SCHEMA_CAPTURE_CAPTURE => {
            let input = validate_capture_body(&call.body)?;
            exec_capture(service, call, &input, barrier_dir)?
        }
        "receipt" if call.schema_id == SCHEMA_CAPTURE_RECEIPT => {
            let input = validate_receipt_body(&call.body)?;
            exec_receipt(service, call, &input)?
        }
        _ => {
            return Err(CommError::schema(
                "unknown method/schema for capture.inbox/1",
            ))
        }
    };
    Ok(body)
}

fn exec_capture(
    service: &CaptureService,
    call: &CallToProvider,
    input: &CaptureInput,
    barrier_dir: PathBuf,
) -> Result<serde_json::Value, CommError> {
    let digest = crate::store::payload_digest(input);
    let result = match service.barrier {
        BarrierPosition::BeforeCommit => service.store.capture_with_before_commit_barrier(
            &call.caller,
            input,
            &digest,
            &barrier_dir,
        ),
        BarrierPosition::AfterCommitBeforeReply => service.store.capture_with_after_commit_barrier(
            &call.caller,
            input,
            &digest,
            &barrier_dir,
            &input.operation_id,
        ),
        BarrierPosition::None => service.store.capture(&call.caller, input, &digest),
    };
    match result {
        Ok(ok) => {
            let _ = ok.dedup;
            let body = CaptureCommitted {
                status: "committed",
                capture_id: ok.capture_id,
                digest,
            };
            serde_json::to_value(body).map_err(|e| CommError::storage(format!("reply encode: {e}")))
        }
        Err(StoreError::OperationConflict) => {
            // deterministic no-write: provider-validated domain rejection
            Err(CommError::domain(
                "operation_conflict",
                "same key, different payload",
            ))
        }
        Err(e) => Err(e.to_comm()),
    }
}

fn exec_receipt(
    service: &CaptureService,
    call: &CallToProvider,
    input: &autoos_comm_core::admission::ReceiptInput,
) -> Result<serde_json::Value, CommError> {
    // cross-principal receipt access is an explicit permission rejection
    if input.principal != call.caller {
        return Err(CommError::permission(
            "receipt queries are restricted to the trusted principal",
        ));
    }
    let hit = service
        .store
        .receipt(&call.caller, &input.operation_id)
        .map_err(|e| e.to_comm())?;
    let body = match hit {
        Some(h) => ReceiptBody {
            status: "committed",
            capture_id: Some(h.capture_id),
            text: Some(h.text),
            digest: Some(h.digest),
            title: h.title,
            source_uri: h.source_uri,
        },
        None => ReceiptBody {
            status: "not_observed",
            capture_id: None,
            text: None,
            digest: None,
            title: None,
            source_uri: None,
        },
    };
    serde_json::to_value(body).map_err(|e| CommError::storage(format!("reply encode: {e}")))
}

/// The barrier position never applies to receipt queries.
pub fn receipt_barrier_inert(_b: &BarrierPosition) -> bool {
    true
}
