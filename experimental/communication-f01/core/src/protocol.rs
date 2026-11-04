//! Message DTOs of the `json-f01/1` profile (Design02 §4).
//!
//! One flat JSON object per frame, discriminated by the `op` field. Unknown
//! fields, duplicate keys, non-u32 numbers and depth violations are rejected
//! by the strict codec (`codec::decode`) — the `deny_unknown_fields` data
//! here is documentation, not the enforcement point. All wire integers are
//! u32. Caller-supplied correlation: `request_id` correlates one attempt,
//! `corr_id` correlates register/discover exchanges; the domain dedup key is
//! `operation_id` inside capture bodies — the three must never be conflated.
//!
//! Wire shapes (frozen at T-01, see plan-artifacts/plan-049/contract-decision.md):
//! - caller→broker: `hello welcome discover discovered call reply error`
//! - broker→provider (private): `pcall preply` + `welcome error`
//! - provider→broker: `register registered` + `hello error`

use serde::{Deserialize, Serialize};

pub const PROTOCOL_VERSION: u32 = 1;
pub const CODEC_ID: &str = "json-f01/1";
pub const SERVICE_ID_CAPTURE: &str = "auto.capture.inbox";
pub const INTERFACE_CAPTURE: &str = "capture.inbox/1";
pub const SCHEMA_CAPTURE_CAPTURE: &str = "capture.inbox.capture/1";
pub const SCHEMA_CAPTURE_RECEIPT: &str = "capture.inbox.receipt/1";

/// Opaque reference to one concrete service instance.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ServiceRef {
    pub broker_epoch: String,
    pub instance_id: String,
    pub service_id: String,
    pub interface: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Hello {
    pub protocol_version: u32,
    pub codec: String,
    /// Credential text; bound to a principal by the broker-side policy.
    /// Never logged, never echoed in receipts.
    pub token: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Welcome {
    pub broker_epoch: String,
    /// Server-derived principal label; client body claims never matter.
    pub principal: String,
    pub role: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Register {
    pub corr_id: u32,
    pub service_id: String,
    pub interface: String,
    /// Descriptor of this profile's body schema pair (frozen text).
    pub schema_id: String,
    /// Opaque service self-description text (≤4096 bytes); registry compares
    /// equality only.
    pub descriptor: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Registered {
    pub corr_id: u32,
    pub service_ref: ServiceRef,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Discover {
    pub corr_id: u32,
    pub service_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Discovered {
    pub corr_id: u32,
    pub service_ref: ServiceRef,
}

/// Caller → broker invocation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Call {
    pub request_id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub trace_id: Option<String>,
    pub service_ref: ServiceRef,
    pub method: String,
    pub schema_id: String,
    pub timeout_ms: u32,
    /// Must be a JSON object; profile-checked before forwarding.
    pub body: serde_json::Value,
}

/// Broker → provider forwarded invocation (private shape on the provider
/// connection). `caller` is the trusted principal bound at Hello; `route_id`
/// is broker-private and the provider may only reply on routes it received.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CallToProvider {
    pub route_id: String,
    pub request_id: String,
    pub caller: String,
    pub method: String,
    pub schema_id: String,
    /// Remaining budget computed by the broker on its local monotonic clock.
    pub deadline_ms: u32,
    pub body: serde_json::Value,
}

/// Provider → broker → caller successful reply.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Reply {
    pub request_id: String,
    pub body: serde_json::Value,
}

/// Provider → broker reply correlated by private route.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReplyFromProvider {
    pub route_id: String,
    pub body: serde_json::Value,
}

/// Structured error. At least one correlation field must be present.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Error {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub corr_id: Option<u32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub route_id: Option<String>,
    pub code: crate::error::ErrorCode,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub message: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub execution: Option<crate::error::ExecutionState>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub domain_code: Option<String>,
}

impl Error {
    pub fn from_comm(
        corr: Option<u32>,
        request_id: Option<String>,
        route_id: Option<String>,
        e: crate::error::CommError,
    ) -> Self {
        Error {
            corr_id: corr,
            request_id,
            route_id,
            code: e.code,
            message: e.message,
            execution: e.execution,
            domain_code: e.domain_code,
        }
    }

    pub fn to_comm(&self) -> crate::error::CommError {
        let mut e = crate::error::CommError::new(self.code, self.message.clone());
        e.execution = self.execution;
        e.domain_code = self.domain_code.clone();
        e
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "op")]
pub enum Message {
    #[serde(rename = "hello")]
    Hello(Hello),
    #[serde(rename = "welcome")]
    Welcome(Welcome),
    #[serde(rename = "register")]
    Register(Register),
    #[serde(rename = "registered")]
    Registered(Registered),
    #[serde(rename = "discover")]
    Discover(Discover),
    #[serde(rename = "discovered")]
    Discovered(Discovered),
    #[serde(rename = "call")]
    Call(Call),
    #[serde(rename = "pcall")]
    CallToProvider(CallToProvider),
    #[serde(rename = "reply")]
    Reply(Reply),
    #[serde(rename = "preply")]
    ReplyFromProvider(ReplyFromProvider),
    #[serde(rename = "error")]
    Error(Error),
}

impl Message {
    pub fn op_name(&self) -> &'static str {
        match self {
            Message::Hello(_) => "hello",
            Message::Welcome(_) => "welcome",
            Message::Register(_) => "register",
            Message::Registered(_) => "registered",
            Message::Discover(_) => "discover",
            Message::Discovered(_) => "discovered",
            Message::Call(_) => "call",
            Message::CallToProvider(_) => "pcall",
            Message::Reply(_) => "reply",
            Message::ReplyFromProvider(_) => "preply",
            Message::Error(_) => "error",
        }
    }
}
