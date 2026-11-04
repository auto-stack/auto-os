//! Caller-side client connection: Hello/discover/call with correlation and
//! deadline handling. The client never trusts its own body claims — the
//! broker binds identity from the token alone.

use crate::frame_io::read_frame;
use crate::pipe::{connect_pipe, send_message};
use autoos_comm_core::error::{CommError, ExecutionState};
use autoos_comm_core::protocol::{
    Call, Discover, Discovered, Hello, Message, Reply, ServiceRef, Welcome, CODEC_ID,
    PROTOCOL_VERSION,
};
use autoos_comm_core::Limits;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use tokio::io::WriteHalf;
use tokio::net::windows::named_pipe::NamedPipeClient;
use tokio::sync::{oneshot, Mutex as AsyncMutex};
use tokio::time::timeout;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum PendingKey {
    Corr(u32),
    Request(String),
}

struct ClientInner {
    write: AsyncMutex<WriteHalf<NamedPipeClient>>,
    next_corr: std::sync::atomic::AtomicU32,
    pending: Mutex<HashMap<PendingKey, oneshot::Sender<Result<Message, CommError>>>>,
}

#[derive(Clone)]
pub struct CommClient {
    inner: Arc<ClientInner>,
    welcome: Welcome,
}

impl CommClient {
    /// Connect, say Hello, and wait for the Welcome (all bounded).
    pub async fn connect(
        pipe_name: &str,
        token: &str,
        deadline: std::time::Instant,
    ) -> Result<CommClient, CommError> {
        let client = connect_pipe(pipe_name, deadline).await.map_err(|e| {
            CommError::disconnected(
                ExecutionState::NotDispatched,
                format!("connect failed: {e}"),
            )
        })?;
        let (r, w) = tokio::io::split(client);
        let inner = Arc::new(ClientInner {
            write: AsyncMutex::new(w),
            next_corr: std::sync::atomic::AtomicU32::new(1),
            pending: Mutex::new(HashMap::new()),
        });
        // handshake
        let hello = Message::Hello(Hello {
            protocol_version: PROTOCOL_VERSION,
            codec: CODEC_ID.to_string(),
            token: token.to_string(),
        });
        {
            let mut gw = inner.write.lock().await;
            crate::pipe::send_message(&mut *gw, &hello).await?;
        }
        let (tx, rx) = oneshot::channel();
        let _ = inner
            .pending
            .lock()
            .unwrap()
            .insert(PendingKey::Corr(0), tx); // corr 0 reserved for Welcome
        spawn_read_loop(inner.clone(), r);
        let welcome = timeout(
            deadline.saturating_duration_since(std::time::Instant::now()),
            rx,
        )
        .await
        .map_err(|_| CommError::deadline(ExecutionState::NotDispatched, "welcome deadline"))?
        .map_err(|_| CommError::disconnected(ExecutionState::NotDispatched, "read loop ended"))??;
        let welcome = match welcome {
            Message::Welcome(w) => w,
            Message::Error(e) => return Err(e.to_comm()),
            other => {
                return Err(CommError::schema(format!(
                    "expected welcome, got `{}`",
                    other.op_name()
                )))
            }
        };
        Ok(CommClient { inner, welcome })
    }

    pub fn principal(&self) -> &str {
        &self.welcome.principal
    }

    pub fn broker_epoch(&self) -> &str {
        &self.welcome.broker_epoch
    }

    pub fn role(&self) -> &str {
        &self.welcome.role
    }

    async fn request(&self, key: PendingKey, msg: Message) -> Result<Message, CommError> {
        let (tx, rx) = oneshot::channel();
        {
            let mut pending = self.inner.pending.lock().unwrap();
            if pending.contains_key(&key) {
                // never displace an in-flight correlation: the duplicate is
                // rejected client-side and the original stays resolvable
                return Err(CommError::schema("correlation key already active"));
            }
            pending.insert(key, tx);
        }
        {
            let mut w = self.inner.write.lock().await;
            send_message(&mut *w, &msg).await?;
        }
        // broker-side deadlines answer every request; the cap is a hang guard
        let answer = timeout(Duration::from_secs(30), rx)
            .await
            .map_err(|_| CommError::deadline(ExecutionState::MayHaveRun, "client wait cap"))?
            .map_err(|_| {
                CommError::disconnected(ExecutionState::MayHaveRun, "connection closed")
            })??;
        Ok(answer)
    }

    pub async fn discover(&self, service_id: &str) -> Result<ServiceRef, CommError> {
        let corr = self
            .inner
            .next_corr
            .fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        let answer = self
            .request(
                PendingKey::Corr(corr),
                Message::Discover(Discover {
                    corr_id: corr,
                    service_id: service_id.to_string(),
                }),
            )
            .await?;
        match answer {
            Message::Discovered(Discovered { service_ref, .. }) => Ok(service_ref),
            Message::Error(e) => Err(e.to_comm()),
            other => Err(CommError::schema(format!(
                "unexpected `{}` reply to discover",
                other.op_name()
            ))),
        }
    }

    /// Issue one call with an explicit request_id (used by the duplicate
    /// request_id counterexample) and wait for the reply or structured error.
    pub async fn call_with_id(
        &self,
        request_id: &str,
        service_ref: &ServiceRef,
        method: &str,
        schema_id: &str,
        timeout_ms: u32,
        body: serde_json::Value,
    ) -> Result<serde_json::Value, CommError> {
        let call = Message::Call(Call {
            request_id: request_id.to_string(),
            trace_id: None,
            service_ref: service_ref.clone(),
            method: method.to_string(),
            schema_id: schema_id.to_string(),
            timeout_ms,
            body,
        });
        let answer = self
            .request(PendingKey::Request(request_id.to_string()), call)
            .await?;
        match answer {
            Message::Reply(Reply { body, .. }) => Ok(body),
            Message::Error(e) => Err(e.to_comm()),
            other => Err(CommError::schema(format!(
                "unexpected `{}` reply to call",
                other.op_name()
            ))),
        }
    }

    /// Issue one call with a fresh random request_id.
    pub async fn call(
        &self,
        service_ref: &ServiceRef,
        method: &str,
        schema_id: &str,
        timeout_ms: u32,
        body: serde_json::Value,
    ) -> Result<serde_json::Value, CommError> {
        self.call_with_id(
            &autoos_comm_core::ids::new_id(),
            service_ref,
            method,
            schema_id,
            timeout_ms,
            body,
        )
        .await
    }

    /// Issue a call and return immediately after the frame is fully written
    /// (used for late-reply and abandonment counterexamples).
    pub async fn call_abandon(
        &self,
        request_id: &str,
        service_ref: &ServiceRef,
        method: &str,
        schema_id: &str,
        timeout_ms: u32,
        body: serde_json::Value,
    ) -> Result<(), CommError> {
        let call = Message::Call(Call {
            request_id: request_id.to_string(),
            trace_id: None,
            service_ref: service_ref.clone(),
            method: method.to_string(),
            schema_id: schema_id.to_string(),
            timeout_ms,
            body,
        });
        let mut w = self.inner.write.lock().await;
        send_message(&mut *w, &call).await
    }
}

fn spawn_read_loop(inner: Arc<ClientInner>, mut r: tokio::io::ReadHalf<NamedPipeClient>) {
    tokio::spawn(async move {
        loop {
            match read_frame(&mut r).await {
                Ok(Some(frame)) => {
                    let msg = match autoos_comm_core::codec::decode(&frame) {
                        Ok(m) => m,
                        Err(_) => break, // protocol violation: fail pending
                    };
                    let key = match &msg {
                        Message::Welcome(_) => Some(PendingKey::Corr(0)),
                        Message::Discovered(d) => Some(PendingKey::Corr(d.corr_id)),
                        Message::Reply(rp) => Some(PendingKey::Request(rp.request_id.clone())),
                        Message::Error(e) => {
                            if let Some(c) = e.corr_id {
                                Some(PendingKey::Corr(c))
                            } else if let Some(rid) = &e.request_id {
                                Some(PendingKey::Request(rid.clone()))
                            } else {
                                // uncorrelated: handshake-phase rejection
                                // (before any Welcome) resolves the
                                // reserved Welcome slot
                                Some(PendingKey::Corr(0))
                            }
                        }
                        _ => None,
                    };
                    if let Some(key) = key {
                        let tx = inner.pending.lock().unwrap().remove(&key);
                        let _ = tx.map(|tx| tx.send(Ok(msg)));
                    }
                }
                // clean EOF or read error: fail all pending below
                _ => break,
            }
        }
        // fail all pending with a conservative may_have_run
        let pending: Vec<_> = inner.pending.lock().unwrap().drain().collect();
        for (_, tx) in pending {
            let _ = tx.send(Err(CommError::disconnected(
                ExecutionState::MayHaveRun,
                "connection closed",
            )));
        }
    });
}

/// Access the write half for raw-protocol counterexamples (V-04/V-08).
pub async fn raw_send(client: &CommClient, msg: &Message) -> Result<(), CommError> {
    let mut w = client.inner.write.lock().await;
    send_message(&mut *w, msg).await
}

/// Generous bound helper for tests (single-case budget is 20s).
pub fn case_budget() -> Duration {
    Limits::SINGLE_CASE_BUDGET
}
