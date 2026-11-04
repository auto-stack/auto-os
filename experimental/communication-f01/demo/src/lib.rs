//! autoos-comm-demo — capture inbox service: private SQLite store, domain
//! logic, run/test support and the three experiment binaries.

pub mod capture;
pub mod store;
pub mod support;

pub use store::{Barrier, CaptureOk, ReceiptHit, Store, StoreError};
