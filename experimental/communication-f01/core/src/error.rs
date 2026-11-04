//! Error model of the F0/F1 profile (Design02 §4 state table).
//!
//! Input/profile rejections, authorization/discovery/instance rejections,
//! resource and storage failures, deadline/disconnect states (which carry an
//! explicit `execution` qualifier), and provider domain rejections are
//! distinct codes. A reply that "succeeded" from the broker's point of view
//! must never be re-labeled as a deterministic non-execution.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ErrorCode {
    /// Malformed frame: zero length, bad UTF-8, broken JSON syntax.
    InvalidFrame,
    /// Well-formed JSON that violates the profile: duplicate key, unknown
    /// field, wrong/oversized/undersized typed value, unknown operation,
    /// duplicate active request_id.
    SchemaMismatch,
    /// Protocol version or codec id has no common profile with the peer.
    UnsupportedVersion,
    /// Token unknown, role not allowed for the operation, or cross-principal
    /// access attempt.
    PermissionDenied,
    /// Discover/call target service currently not registered.
    ServiceNotFound,
    /// ServiceRef binds a stale broker epoch or a retired instance.
    StaleInstance,
    /// Another active provider owns the service (or descriptor changed on the
    /// same connection).
    AlreadyRegistered,
    /// A declared finite limit was hit (frame size, depth, quotas, capacity).
    ResourceExhausted,
    /// Domain storage failed (readonly/corrupt DB, IO error).
    StorageError,
    /// Deadline passed. Carries `execution`.
    DeadlineExceeded,
    /// Connection lost. Carries `execution`.
    Disconnected,
    /// Provider refused an already-validated request; carries `domain_code`.
    DomainError,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ExecutionState {
    /// The broker can prove the request never entered forwarding/handler.
    NotDispatched,
    /// The request was forwarded; its effect is unknown.
    MayHaveRun,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommError {
    pub code: ErrorCode,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub message: String,
    /// Present only for `deadline_exceeded` / `disconnected`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub execution: Option<ExecutionState>,
    /// Present only for `domain_error`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub domain_code: Option<String>,
}

impl CommError {
    pub fn new(code: ErrorCode, message: impl Into<String>) -> Self {
        CommError {
            code,
            message: message.into(),
            execution: None,
            domain_code: None,
        }
    }

    pub fn with_execution(mut self, e: ExecutionState) -> Self {
        self.execution = Some(e);
        self
    }

    pub fn with_domain(mut self, code: impl Into<String>) -> Self {
        self.domain_code = Some(code.into());
        self
    }

    pub fn invalid_frame(m: impl Into<String>) -> Self {
        Self::new(ErrorCode::InvalidFrame, m)
    }
    pub fn schema(m: impl Into<String>) -> Self {
        Self::new(ErrorCode::SchemaMismatch, m)
    }
    pub fn version(m: impl Into<String>) -> Self {
        Self::new(ErrorCode::UnsupportedVersion, m)
    }
    pub fn permission(m: impl Into<String>) -> Self {
        Self::new(ErrorCode::PermissionDenied, m)
    }
    pub fn not_found(m: impl Into<String>) -> Self {
        Self::new(ErrorCode::ServiceNotFound, m)
    }
    pub fn stale(m: impl Into<String>) -> Self {
        Self::new(ErrorCode::StaleInstance, m)
    }
    pub fn resource(m: impl Into<String>) -> Self {
        Self::new(ErrorCode::ResourceExhausted, m)
    }
    pub fn storage(m: impl Into<String>) -> Self {
        Self::new(ErrorCode::StorageError, m)
    }
    pub fn deadline(execution: ExecutionState, m: impl Into<String>) -> Self {
        Self::new(ErrorCode::DeadlineExceeded, m).with_execution(execution)
    }
    pub fn disconnected(execution: ExecutionState, m: impl Into<String>) -> Self {
        Self::new(ErrorCode::Disconnected, m).with_execution(execution)
    }
    pub fn domain(code: impl Into<String>, m: impl Into<String>) -> Self {
        Self::new(ErrorCode::DomainError, m).with_domain(code)
    }
}

impl std::fmt::Display for CommError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.code)?;
        if let Some(e) = &self.execution {
            write!(f, "({:?})", e)?;
        }
        if let Some(d) = &self.domain_code {
            write!(f, "({})", d)?;
        }
        if !self.message.is_empty() {
            write!(f, ": {}", self.message)?;
        }
        Ok(())
    }
}

impl std::error::Error for CommError {}
