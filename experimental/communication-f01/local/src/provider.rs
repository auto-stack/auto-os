//! Provider-side session: register one service and serve forwarded calls on
//! broker-private routes. A provider can only complete routes it received on
//! its own connection; spoofed route ids are rejected by the broker, and the
//! provider never learns other callers' connections.

use crate::frame_io::read_frame;
use crate::pipe::{connect_pipe, send_message};
use autoos_comm_core::error::{CommError, ExecutionState};
use autoos_comm_core::protocol::{
    CallToProvider, Error as ErrorMessage, Hello, Message, Register, ServiceRef, Welcome, CODEC_ID,
    PROTOCOL_VERSION,
};
use autoos_comm_core::Limits;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use tokio::io::{ReadHalf, WriteHalf};
use tokio::net::windows::named_pipe::NamedPipeClient;
use tokio::sync::Mutex as AsyncMutex;
use tokio::time::timeout;

struct ProviderInner {
    write: AsyncMutex<WriteHalf<NamedPipeClient>>,
    read: AsyncMutex<Option<ReadHalf<NamedPipeClient>>>,
    inflight: AtomicUsize,
}

#[derive(Clone)]
pub struct ProviderConn {
    inner: Arc<ProviderInner>,
    welcome: Welcome,
    service_ref: ServiceRef,
}

impl ProviderConn {
    /// Connect, Hello, and register exactly one service.
    pub async fn connect_register(
        pipe_name: &str,
        token: &str,
        service_id: &str,
        interface: &str,
        schema_id: &str,
        descriptor: &str,
        deadline: std::time::Instant,
    ) -> Result<ProviderConn, CommError> {
        let client = connect_pipe(pipe_name, deadline).await.map_err(|e| {
            CommError::disconnected(
                ExecutionState::NotDispatched,
                format!("connect failed: {e}"),
            )
        })?;
        let (r, w) = tokio::io::split(client);
        let inner = Arc::new(ProviderInner {
            write: AsyncMutex::new(w),
            read: AsyncMutex::new(Some(r)),
            inflight: AtomicUsize::new(0),
        });
        {
            let mut gw = inner.write.lock().await;
            send_message(
                &mut *gw,
                &Message::Hello(Hello {
                    protocol_version: PROTOCOL_VERSION,
                    codec: CODEC_ID.to_string(),
                    token: token.to_string(),
                }),
            )
            .await?;
        }
        let welcome = {
            let mut gr = inner.read.lock().await;
            let gr = gr.as_mut().expect("read half present until serve");
            match crate::pipe::read_message(gr).await? {
                Some(Message::Welcome(w)) => w,
                Some(Message::Error(e)) => return Err(e.to_comm()),
                Some(other) => {
                    return Err(CommError::schema(format!(
                        "expected welcome, got `{}`",
                        other.op_name()
                    )))
                }
                None => {
                    return Err(CommError::disconnected(
                        ExecutionState::NotDispatched,
                        "broker closed during handshake",
                    ))
                }
            }
        };
        {
            let mut gw = inner.write.lock().await;
            send_message(
                &mut *gw,
                &Message::Register(Register {
                    corr_id: 1,
                    service_id: service_id.to_string(),
                    interface: interface.to_string(),
                    schema_id: schema_id.to_string(),
                    descriptor: descriptor.to_string(),
                }),
            )
            .await?;
        }
        let service_ref = {
            let mut gr = inner.read.lock().await;
            let gr = gr.as_mut().expect("read half present until serve");
            match crate::pipe::read_message(gr).await? {
                Some(Message::Registered(reg)) => reg.service_ref,
                Some(Message::Error(e)) => return Err(e.to_comm()),
                Some(other) => {
                    return Err(CommError::schema(format!(
                        "expected registered, got `{}`",
                        other.op_name()
                    )))
                }
                None => {
                    return Err(CommError::disconnected(
                        ExecutionState::NotDispatched,
                        "broker closed during register",
                    ))
                }
            }
        };
        Ok(ProviderConn {
            inner,
            welcome,
            service_ref,
        })
    }

    pub fn service_ref(&self) -> &ServiceRef {
        &self.service_ref
    }

    pub fn principal(&self) -> &str {
        &self.welcome.principal
    }

    /// Serve forwarded calls until the broker connection drops. Each call is
    /// executed under its remaining budget; the provider never replies on a
    /// route other than the one it received.
    pub async fn serve<F, Fut>(&self, handler: F) -> Result<(), CommError>
    where
        F: Fn(CallToProvider) -> Fut + Send + Sync + 'static,
        Fut: std::future::Future<Output = Result<serde_json::Value, CommError>> + Send + 'static,
    {
        let mut r = self
            .inner
            .read
            .lock()
            .await
            .take()
            .expect("serve() called at most once");
        loop {
            match read_frame(&mut r).await {
                Ok(Some(frame)) => {
                    let msg = match autoos_comm_core::codec::decode(&frame) {
                        Ok(m) => m,
                        Err(_) => break,
                    };
                    let call = match msg {
                        Message::CallToProvider(c) => c,
                        Message::Error(_) => continue, // broker-origin notices
                        _ => break,                    // unexpected direction
                    };
                    // bounded admission: never spawn unbounded handlers
                    if self.inner.inflight.load(Ordering::Relaxed) >= Limits::MAX_PROVIDER_QUEUE {
                        let e = CommError::resource("provider queue full")
                            .with_execution(ExecutionState::NotDispatched);
                        let _ = provider_error(&self.inner.write, &call.route_id, &e).await;
                        continue;
                    }
                    self.inner.inflight.fetch_add(1, Ordering::Relaxed);
                    let inner = self.inner.clone();
                    let fut = handler(call.clone());
                    tokio::spawn(async move {
                        let outcome = timeout(
                            std::time::Duration::from_millis(call.deadline_ms as u64),
                            fut,
                        )
                        .await;
                        let _ = match outcome {
                            Ok(Ok(body)) => {
                                provider_reply(&inner.write, &call.route_id, body).await
                            }
                            Ok(Err(e)) => provider_error(&inner.write, &call.route_id, &e).await,
                            Err(_) => {
                                // the provider budget expired: effect unknown
                                let e = CommError::deadline(
                                    ExecutionState::MayHaveRun,
                                    "provider budget expired",
                                );
                                provider_error(&inner.write, &call.route_id, &e).await
                            }
                        };
                        inner.inflight.fetch_sub(1, Ordering::Relaxed);
                    });
                }
                _ => {
                    return Err(CommError::disconnected(
                        ExecutionState::MayHaveRun,
                        "broker connection closed",
                    ))
                }
            }
        }
        Err(CommError::disconnected(
            ExecutionState::MayHaveRun,
            "serve loop ended",
        ))
    }
}

/// Reply helper used by the demo service loop.
pub async fn provider_reply(
    write: &AsyncMutex<WriteHalf<NamedPipeClient>>,
    route_id: &str,
    body: serde_json::Value,
) -> Result<(), CommError> {
    let mut w = write.lock().await;
    send_message(
        &mut *w,
        &Message::ReplyFromProvider(autoos_comm_core::protocol::ReplyFromProvider {
            route_id: route_id.to_string(),
            body,
        }),
    )
    .await
}

/// Structured-error reply helper (domain_error, storage_error, ...).
pub async fn provider_error(
    write: &AsyncMutex<WriteHalf<NamedPipeClient>>,
    route_id: &str,
    e: &CommError,
) -> Result<(), CommError> {
    let mut w = write.lock().await;
    send_message(
        &mut *w,
        &Message::Error(ErrorMessage::from_comm(
            None,
            None,
            Some(route_id.to_string()),
            e.clone(),
        )),
    )
    .await
}
