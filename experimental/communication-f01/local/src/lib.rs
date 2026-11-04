//! autoos-comm-local — Windows named-pipe transport, broker, client and
//! provider session machinery for the F0/F1 profile (PLAN-049).

pub mod broker;
pub mod client;
pub mod eventlog;
pub mod frame_io;
pub mod job;
pub mod pipe;
pub mod policy;
pub mod provider;

pub use broker::BrokerConfig;
pub use client::CommClient;
pub use policy::{AccessPolicy, Session};
pub use provider::ProviderConn;
