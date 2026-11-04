//! autoos-comm-core — F0/F1 wire: DTOs, strict codec, admission, errors.
//!
//! Zero IO: this crate never touches sockets, pipes, files or clocks; it only
//! defines and validates the finite wire profile frozen for PLAN-049
//! (Design02 §§4–6, `json-f01/1`).

pub mod admission;
pub mod codec;
pub mod error;
pub mod ids;
pub mod protocol;

pub use admission::Limits;
pub use error::{CommError, ErrorCode, ExecutionState};
pub use protocol::{
    Call, CallToProvider, Discover, Discovered, Error, Hello, Message, Register, Registered, Reply,
    ReplyFromProvider, ServiceRef, Welcome, CODEC_ID, INTERFACE_CAPTURE, SCHEMA_CAPTURE_CAPTURE,
    SCHEMA_CAPTURE_RECEIPT, SERVICE_ID_CAPTURE,
};
