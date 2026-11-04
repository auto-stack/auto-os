//! Broker process core: authenticated connections, service registry, route
//! table, budgets and resource caps (Design02 §§3–4, 6).
//!
//! Invariants the implementation is bound to:
//! - The broker never embeds a domain handler; it only routes.
//! - Identity comes from the Hello token, never from message bodies.
//! - A forwarded call is conservatively `may_have_run`; only rejections
//!   decided before the forward attempt are `not_dispatched`.
//! - Late replies never reach a new request; routes are removed at deadline
//!   or terminal error; duplicate active request_id is rejected.
//! - Mutations are never replayed automatically.
//!
//! Lock discipline: the state mutex is a `std::sync::Mutex`; guards are
//! dropped before every `.await`.

use crate::eventlog::EventLog;
use crate::frame_io::read_frame;
use crate::pipe::{create_pipe_server, send_message, server_handshake};
use crate::policy::{AccessPolicy, Session};
use autoos_comm_core::error::{CommError, ErrorCode, ExecutionState};
use autoos_comm_core::ids;
use autoos_comm_core::protocol::{
    self, Call, CallToProvider, Discovered, Error as ErrorMessage, Message, Registered,
};
use autoos_comm_core::Limits;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use tokio::io::ReadHalf;
use tokio::net::windows::named_pipe::NamedPipeServer;
use tokio::sync::{watch, Mutex as AsyncMutex};
use tokio::time::{sleep_until, Instant as TokioInstant};

type WriteHalfArc = Arc<AsyncMutex<tokio::io::WriteHalf<NamedPipeServer>>>;

#[derive(Clone)]
pub struct BrokerConfig {
    pub pipe_name: String,
    pub log: EventLog,
}

struct ServiceEntry {
    conn_id: u64,

    service_ref: protocol::ServiceRef,
    descriptor: String,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum RouteState {
    /// Created, forward not yet attempted: a deadline here is provably
    /// `not_dispatched`.
    Pending,
    /// Forward write attempted: any deadline/disconnect is `may_have_run`.
    Forwarded,
}

struct Route {
    caller_conn: u64,
    provider_conn: u64,
    request_id: String,
    state: RouteState,
}

struct ConnCtx {
    session: Session,
    write: WriteHalfArc,
}

struct BrokerInner {
    epoch: String,
    registry: HashMap<String, ServiceEntry>,
    routes: HashMap<String, Route>,
    inflight_per_conn: HashMap<u64, usize>,
    inflight_global: usize,
    auth_count: usize,
    next_conn_id: u64,
    conns: HashMap<u64, Arc<ConnCtx>>,
}

impl BrokerInner {
    fn take_route(&mut self, route_id: &str) -> Option<Route> {
        let route = self.routes.remove(route_id)?;
        self.inflight_global = self.inflight_global.saturating_sub(1);
        if let Some(n) = self.inflight_per_conn.get_mut(&route.caller_conn) {
            *n = n.saturating_sub(1);
            if *n == 0 {
                self.inflight_per_conn.remove(&route.caller_conn);
            }
        }
        Some(route)
    }
}

pub struct Broker {
    cfg: BrokerConfig,
    policy: Arc<AccessPolicy>,
    inner: Arc<Mutex<BrokerInner>>,
    shutdown: watch::Receiver<bool>,
}

impl Broker {
    pub fn new(cfg: BrokerConfig, policy: AccessPolicy, shutdown: watch::Receiver<bool>) -> Broker {
        let epoch = ids::new_id();
        Broker {
            cfg,
            policy: Arc::new(policy),
            inner: Arc::new(Mutex::new(BrokerInner {
                epoch,
                registry: HashMap::new(),
                routes: HashMap::new(),
                inflight_per_conn: HashMap::new(),
                inflight_global: 0,
                auth_count: 0,
                next_conn_id: 1,
                conns: HashMap::new(),
            })),
            shutdown,
        }
    }

    pub fn epoch(&self) -> String {
        self.inner.lock().unwrap().epoch.clone()
    }

    /// Accept loop. Resolves when shutdown fires or the listen fails fatally.
    pub async fn serve(self) {
        let log = self.cfg.log.clone();
        let mut shutdown = self.shutdown.clone();
        log.event("broker_start", &[("pipe", self.cfg.pipe_name.as_str())]);
        let mut current = match create_pipe_server(&self.cfg.pipe_name, true) {
            Ok(s) => s,
            Err(e) => {
                // first-instance failure: another broker holds this name (V-05)
                log.event(
                    "broker_listen_failed",
                    &[("error", format!("{e:?}").as_str())],
                );
                return;
            }
        };
        loop {
            tokio::select! {
                _ = shutdown.changed() => {
                    log.event("broker_shutdown", &[]);
                    return;
                }
                r = current.connect() => {
                    if r.is_err() {
                        log.event("accept_failed", &[]);
                        match create_pipe_server(&self.cfg.pipe_name, false) {
                            Ok(s) => current = s,
                            Err(e) => {
                                log.event("recreate_failed", &[("error", format!("{e:?}").as_str())]);
                                shutdown.changed().await.ok();
                                return;
                            }
                        }
                        continue;
                    }
                    let connected = current;
                    current = match create_pipe_server(&self.cfg.pipe_name, false) {
                        Ok(s) => s,
                        Err(e) => {
                            log.event("recreate_failed", &[("error", format!("{e:?}").as_str())]);
                            // serve the already-connected client, then stop accepting
                            spawn_conn(Arc::new(self.clone_for_conn()), connected, log.clone());
                            shutdown.changed().await.ok();
                            log.event("broker_shutdown", &[]);
                            return;
                        }
                    };
                    spawn_conn(Arc::new(self.clone_for_conn()), connected, log.clone());
                }
            }
        }
    }

    fn clone_for_conn(&self) -> Broker {
        Broker {
            cfg: self.cfg.clone(),
            policy: self.policy.clone(),
            inner: self.inner.clone(),
            shutdown: self.shutdown.clone(),
        }
    }
}

fn spawn_conn(broker: Arc<Broker>, mut server: NamedPipeServer, log: EventLog) {
    tokio::spawn(async move {
        // authenticated session (bounded handshake)
        let session = match server_handshake(
            &mut server,
            &broker.epoch(),
            &broker.policy,
            Limits::HANDSHAKE_DEADLINE,
        )
        .await
        {
            Ok(s) => s,
            Err(e) => {
                log.event(
                    "handshake_failed",
                    &[("code", format!("{:?}", e.code).as_str())],
                );
                return;
            }
        };
        // connection cap: reject after auth, before registration into state
        let over_cap = {
            let inner = broker.inner.lock().unwrap();
            inner.auth_count >= Limits::MAX_AUTH_CONNS
        };
        if over_cap {
            let e = CommError::resource("authenticated connection quota reached")
                .with_execution(ExecutionState::NotDispatched);
            let _ = send_message(&mut server, &error_msg(None, None, None, &e)).await;
            log.event(
                "conn_cap_reject",
                &[("principal", session.principal.as_str())],
            );
            return;
        }
        let conn_id = {
            let mut inner = broker.inner.lock().unwrap();
            let conn_id = inner.next_conn_id;
            inner.next_conn_id += 1;
            inner.auth_count += 1;
            conn_id
        };
        let (r, w) = tokio::io::split(server);
        {
            let mut inner = broker.inner.lock().unwrap();
            inner.conns.insert(
                conn_id,
                Arc::new(ConnCtx {
                    session,
                    write: Arc::new(AsyncMutex::new(w)),
                }),
            );
        }
        log.event("conn_open", &[("conn", format!("{conn_id}").as_str())]);
        let task = ConnTask {
            broker,
            conn_id,
            log,
        };
        task.read_loop(r).await;
    });
}

fn error_msg(
    corr: Option<u32>,
    request_id: Option<String>,
    route_id: Option<String>,
    e: &CommError,
) -> Message {
    Message::Error(ErrorMessage::from_comm(
        corr,
        request_id,
        route_id,
        e.clone(),
    ))
}

struct ConnTask {
    broker: Arc<Broker>,
    conn_id: u64,
    log: EventLog,
}

impl ConnTask {
    fn inner(&self) -> std::sync::MutexGuard<'_, BrokerInner> {
        self.broker.inner.lock().unwrap()
    }

    fn is_provider_conn(&self) -> bool {
        let inner = self.inner();
        inner
            .conns
            .get(&self.conn_id)
            .map(|c| c.session.is_provider())
            .unwrap_or(false)
    }

    /// Provider reply on a broker-private route: complete the pending route
    /// to its caller, or drop it when the route is already gone (late reply
    /// never reaches a new request).
    async fn handle_provider_reply(&self, rp: protocol::ReplyFromProvider) {
        let route = {
            let mut inner = self.inner();
            inner.take_route(&rp.route_id)
        };
        match route {
            Some(route) => {
                let write = {
                    let inner = self.inner();
                    inner.conns.get(&route.caller_conn).map(|c| c.write.clone())
                };
                if let Some(w) = write {
                    let msg = Message::Reply(protocol::Reply {
                        request_id: route.request_id,
                        body: rp.body,
                    });
                    send_error_to(&w, msg).await;
                }
            }
            None => {
                self.log.event("late_reply_dropped", &[]);
            }
        }
    }

    async fn read_loop(self, mut r: ReadHalf<NamedPipeServer>) {
        loop {
            let frame = match read_frame(&mut r).await {
                Ok(Some(f)) => f,
                Ok(None) => break,
                Err(e) => {
                    self.log.event(
                        "conn_read_error",
                        &[("code", format!("{:?}", e.code).as_str())],
                    );
                    break;
                }
            };
            let msg = match autoos_comm_core::codec::decode(&frame) {
                Ok(m) => m,
                Err(e) => {
                    // structural rejection: connection terminates with a
                    // structured local forensic event (no credentials).
                    self.log
                        .event("bad_frame", &[("code", format!("{:?}", e.code).as_str())]);
                    self.send_err(None, None, None, &e).await;
                    break;
                }
            };
            let is_provider = self.is_provider_conn();
            match msg {
                Message::ReplyFromProvider(rp) => {
                    if is_provider {
                        self.handle_provider_reply(rp).await;
                    } else {
                        self.log.event("unexpected_direction", &[("op", "preply")]);
                        break;
                    }
                }
                Message::Hello(_)
                | Message::Welcome(_)
                | Message::Registered(_)
                | Message::Discovered(_)
                | Message::Reply(_)
                | Message::CallToProvider(_) => {
                    self.log
                        .event("unexpected_direction", &[("op", msg.op_name())]);
                    break;
                }
                Message::Register(reg) => {
                    if !is_provider {
                        self.log
                            .event("unexpected_direction", &[("op", "register")]);
                        break;
                    }
                    if !self.handle_register(reg).await {
                        break;
                    }
                }
                Message::Discover(d) => {
                    self.handle_discover(d).await;
                }
                Message::Call(call) => {
                    self.handle_call(call).await;
                }
                Message::Error(err) => {
                    self.handle_error(err).await;
                }
            }
        }
        self.teardown().await;
    }

    async fn handle_register(&self, reg: protocol::Register) -> bool {
        let session = self.session();
        if !session.may_register(&reg.service_id) {
            self.send_err(
                reg.corr_id.into(),
                None,
                None,
                &CommError::permission("credential may not register this service"),
            )
            .await;
            return true;
        }
        if reg.interface != protocol::INTERFACE_CAPTURE
            || reg.schema_id != protocol::SCHEMA_CAPTURE_CAPTURE
        {
            self.send_err(
                reg.corr_id.into(),
                None,
                None,
                &CommError::schema("interface/schema mismatch for capture profile"),
            )
            .await;
            return true;
        }
        if reg.descriptor.is_empty() || reg.descriptor.len() > Limits::MAX_DESCRIPTOR {
            self.send_err(
                reg.corr_id.into(),
                None,
                None,
                &CommError::schema("descriptor out of bounds"),
            )
            .await;
            return true;
        }
        let outcome = {
            let mut inner = self.inner();
            match inner.registry.get(&reg.service_id) {
                Some(entry) if entry.conn_id == self.conn_id => {
                    if entry.descriptor == reg.descriptor {
                        Ok(entry.service_ref.clone())
                    } else {
                        Err(CommError::new(
                            ErrorCode::AlreadyRegistered,
                            "descriptor changed on same connection",
                        ))
                    }
                }
                Some(_) => Err(CommError::new(
                    ErrorCode::AlreadyRegistered,
                    "service owned by another provider",
                )),
                None => {
                    let instance_id = ids::new_id();
                    let service_ref = protocol::ServiceRef {
                        broker_epoch: inner.epoch.clone(),
                        instance_id,
                        service_id: reg.service_id.clone(),
                        interface: reg.interface.clone(),
                    };
                    inner.registry.insert(
                        reg.service_id.clone(),
                        ServiceEntry {
                            conn_id: self.conn_id,
                            service_ref: service_ref.clone(),
                            descriptor: reg.descriptor.clone(),
                        },
                    );
                    Ok(service_ref)
                }
            }
        };
        match outcome {
            Ok(service_ref) => {
                self.log
                    .event("registered", &[("service", reg.service_id.as_str())]);
                self.send_msg(Message::Registered(Registered {
                    corr_id: reg.corr_id,
                    service_ref,
                }))
                .await;
            }
            Err(e) => {
                self.log.event(
                    "register_rejected",
                    &[
                        ("service", reg.service_id.as_str()),
                        ("code", format!("{:?}", e.code).as_str()),
                    ],
                );
                self.send_err(reg.corr_id.into(), None, None, &e).await;
            }
        }
        true
    }

    async fn handle_discover(&self, d: protocol::Discover) {
        let session = self.session();
        if !session.may_call(&d.service_id) {
            self.send_err(
                d.corr_id.into(),
                None,
                None,
                &CommError::permission("credential may not access this service"),
            )
            .await;
            return;
        }
        let service_ref = {
            let inner = self.inner();
            inner
                .registry
                .get(&d.service_id)
                .map(|e| e.service_ref.clone())
        };
        match service_ref {
            Some(service_ref) => {
                self.send_msg(Message::Discovered(Discovered {
                    corr_id: d.corr_id,
                    service_ref,
                }))
                .await;
            }
            None => {
                self.send_err(
                    d.corr_id.into(),
                    None,
                    None,
                    &CommError::not_found("service not registered"),
                )
                .await;
            }
        }
    }

    async fn handle_call(&self, call: Call) {
        let recv_start = TokioInstant::now();
        let session = self.session();
        let request_id = call.request_id.clone();

        // 1. permission
        if !session.may_call(&call.service_ref.service_id) {
            self.send_err(
                None,
                request_id.into(),
                None,
                &CommError::permission("credential may not call this service"),
            )
            .await;
            return;
        }
        // 2. reference resolution
        let epoch = self.inner().epoch.clone();
        let entry = {
            let inner = self.inner();
            inner
                .registry
                .get(&call.service_ref.service_id)
                .map(|e| (e.conn_id, e.service_ref.clone()))
        };
        let (provider_conn, entry_ref) = match entry {
            Some((conn_id, service_ref)) => (conn_id, service_ref),
            None => {
                let e = if call.service_ref.broker_epoch != epoch {
                    CommError::stale("broker epoch retired; rediscover explicitly")
                } else {
                    CommError::not_found("service not registered")
                };
                self.send_err(None, request_id.into(), None, &e).await;
                return;
            }
        };
        if entry_ref.instance_id != call.service_ref.instance_id
            || entry_ref.broker_epoch != call.service_ref.broker_epoch
        {
            self.send_err(
                None,
                request_id.into(),
                None,
                &CommError::stale("instance retired; rediscover explicitly"),
            )
            .await;
            return;
        }
        // 3. schema/method pair + body shape (pre-dispatch, no handler runs)
        let body_shape = match (call.method.as_str(), call.schema_id.as_str()) {
            ("capture", s) if s == protocol::SCHEMA_CAPTURE_CAPTURE => {
                autoos_comm_core::admission::validate_capture_body(&call.body).map(|_| ())
            }
            ("receipt", s) if s == protocol::SCHEMA_CAPTURE_RECEIPT => {
                autoos_comm_core::admission::validate_receipt_body(&call.body).map(|_| ())
            }
            _ => Err(CommError::schema("unknown method/schema for this profile")),
        };
        if let Err(e) = body_shape {
            self.send_err(None, request_id.into(), None, &e).await;
            return;
        }
        // 4. timeout profile
        if let Err(e) = autoos_comm_core::admission::validate_timeout_ms(call.timeout_ms) {
            self.send_err(None, request_id.into(), None, &e).await;
            return;
        }
        // 5. duplicate active request_id on this connection
        let duplicate_active = {
            let inner = self.inner();
            inner
                .routes
                .values()
                .any(|r| r.caller_conn == self.conn_id && r.request_id == call.request_id)
        };
        if duplicate_active {
            self.send_err(
                None,
                request_id.into(),
                None,
                &CommError::schema("request_id already active"),
            )
            .await;
            return;
        }
        // 6. remaining budget on the broker-local monotonic clock
        let elapsed_ms = recv_start.elapsed().as_millis() as u32;
        let remaining = call.timeout_ms.saturating_sub(elapsed_ms);
        if remaining == 0 {
            self.send_err(
                None,
                request_id.into(),
                None,
                &CommError::deadline(
                    ExecutionState::NotDispatched,
                    "deadline expired before dispatch",
                ),
            )
            .await;
            return;
        }
        // 7. quotas + route creation
        let route_id = ids::new_id();
        let deadline = TokioInstant::now() + std::time::Duration::from_millis(remaining as u64);
        let quota_reject = {
            let mut inner = self.inner();
            let per_conn = inner
                .inflight_per_conn
                .get(&self.conn_id)
                .copied()
                .unwrap_or(0);
            let provider_queue = inner
                .routes
                .values()
                .filter(|r| r.provider_conn == provider_conn)
                .count();
            let reject = per_conn >= Limits::MAX_INFLIGHT_PER_CONN
                || inner.inflight_global >= Limits::MAX_INFLIGHT_GLOBAL
                || provider_queue >= Limits::MAX_PROVIDER_QUEUE;
            if reject {
                None
            } else {
                inner.routes.insert(
                    route_id.clone(),
                    Route {
                        caller_conn: self.conn_id,
                        provider_conn,
                        request_id: request_id.clone(),
                        state: RouteState::Pending,
                    },
                );
                inner.inflight_global += 1;
                *inner.inflight_per_conn.entry(self.conn_id).or_default() += 1;
                Some(())
            }
        };
        if quota_reject.is_none() {
            let e = CommError::resource("in-flight/queue quota exceeded")
                .with_execution(ExecutionState::NotDispatched);
            self.send_err(None, request_id.into(), None, &e).await;
            return;
        }
        // 8. deadline watchdog (decides not_dispatched vs may_have_run by the
        //    forward state recorded at step 9)
        {
            let inner = self.broker.inner.clone();
            let route_id_w = route_id.clone();
            let request_id_w = request_id.clone();
            let log = self.log.clone();
            tokio::spawn(async move {
                sleep_until(deadline).await;
                let route = {
                    let mut g = inner.lock().unwrap();
                    g.take_route(&route_id_w)
                };
                if let Some(route) = route {
                    let exec = match route.state {
                        RouteState::Forwarded => ExecutionState::MayHaveRun,
                        RouteState::Pending => ExecutionState::NotDispatched,
                    };
                    log.event(
                        "route_deadline",
                        &[("execution", format!("{exec:?}").as_str())],
                    );
                    let write = {
                        let g = inner.lock().unwrap();
                        g.conns.get(&route.caller_conn).map(|c| c.write.clone())
                    };
                    if let Some(w) = write {
                        let e = CommError::deadline(exec, "call deadline exceeded");
                        send_error_to(&w, error_msg(None, request_id_w.into(), None, &e)).await;
                    }
                }
            });
        }
        // 9. forward attempt (state recorded before the write so that any
        //    deadline/disconnect during the write is may_have_run)
        let pcall = CallToProvider {
            route_id: route_id.clone(),
            request_id: request_id.clone(),
            caller: session.principal.clone(),
            method: call.method.clone(),
            schema_id: call.schema_id.clone(),
            deadline_ms: remaining,
            body: call.body.clone(),
        };
        let write = {
            let inner = self.inner();
            inner.conns.get(&provider_conn).map(|c| c.write.clone())
        };
        match write {
            Some(w) => {
                {
                    let mut inner = self.inner();
                    if let Some(r) = inner.routes.get_mut(&route_id) {
                        r.state = RouteState::Forwarded;
                    }
                }
                let mut w = w.lock().await;
                if let Err(e) = send_message(&mut *w, &Message::CallToProvider(pcall)).await {
                    drop(w);
                    self.log.event(
                        "forward_failed",
                        &[("code", format!("{:?}", e.code).as_str())],
                    );
                    let removed = {
                        let mut inner = self.inner();
                        inner.take_route(&route_id).is_some()
                    };
                    if removed {
                        // the forward write started: delivery unknown
                        self.send_err(
                            None,
                            request_id.into(),
                            None,
                            &CommError::disconnected(
                                ExecutionState::MayHaveRun,
                                "provider connection lost",
                            ),
                        )
                        .await;
                    }
                } else {
                    drop(w);
                }
            }
            None => {
                let removed = {
                    let mut inner = self.inner();
                    inner.take_route(&route_id).is_some()
                };
                if removed {
                    self.send_err(
                        None,
                        request_id.into(),
                        None,
                        &CommError::disconnected(
                            ExecutionState::NotDispatched,
                            "provider vanished before dispatch",
                        ),
                    )
                    .await;
                }
            }
        }
    }

    async fn handle_error(&self, err: ErrorMessage) {
        let Some(route_id) = err.route_id.clone() else {
            self.log.event(
                "error_without_route",
                &[("code", format!("{:?}", err.code).as_str())],
            );
            return;
        };
        let route = {
            let mut inner = self.inner();
            inner.take_route(&route_id)
        };
        match route {
            Some(route) => {
                let write = {
                    let inner = self.inner();
                    inner.conns.get(&route.caller_conn).map(|c| c.write.clone())
                };
                if let Some(w) = write {
                    let msg = error_msg(None, Some(route.request_id), None, &err.to_comm());
                    send_error_to(&w, msg).await;
                }
            }
            None => {
                // late or unknown route: dropped, never reused
                self.log.event("late_error_dropped", &[]);
            }
        }
    }

    async fn send_msg(&self, msg: Message) {
        let write = {
            let inner = self.inner();
            inner.conns.get(&self.conn_id).map(|c| c.write.clone())
        };
        if let Some(w) = write {
            send_error_to(&w, msg).await;
        }
    }

    async fn send_err(
        &self,
        corr: Option<u32>,
        request_id: Option<String>,
        route_id: Option<String>,
        e: &CommError,
    ) {
        self.log
            .event("error_sent", &[("code", format!("{:?}", e.code).as_str())]);
        self.send_msg(error_msg(corr, request_id, route_id, e))
            .await;
    }

    fn session(&self) -> Session {
        let inner = self.inner();
        inner
            .conns
            .get(&self.conn_id)
            .expect("conn ctx exists")
            .session
            .clone()
    }

    async fn teardown(self) {
        let notifications: Vec<(WriteHalfArc, Message)> = {
            let mut inner = self.inner();
            inner.registry.retain(|_, e| e.conn_id != self.conn_id);
            let route_ids: Vec<String> = inner
                .routes
                .iter()
                .filter(|(_, r)| r.caller_conn == self.conn_id || r.provider_conn == self.conn_id)
                .map(|(id, _)| id.clone())
                .collect();
            let mut notifs = Vec::new();
            for id in route_ids {
                if let Some(route) = inner.take_route(&id) {
                    if route.provider_conn == self.conn_id && route.caller_conn != self.conn_id {
                        // it was forwarded => provider-side loss => may_have_run
                        if let Some(ctx) = inner.conns.get(&route.caller_conn) {
                            let msg = error_msg(
                                None,
                                Some(route.request_id.clone()),
                                None,
                                &CommError::disconnected(
                                    ExecutionState::MayHaveRun,
                                    "provider disconnected",
                                ),
                            );
                            notifs.push((ctx.write.clone(), msg));
                        }
                    }
                }
            }
            inner.conns.remove(&self.conn_id);
            inner.auth_count = inner.auth_count.saturating_sub(1);
            notifs
        };
        for (w, msg) in notifications {
            send_error_to(&w, msg).await;
        }
        self.log.event(
            "conn_closed",
            &[("conn", format!("{}", self.conn_id).as_str())],
        );
    }
}

async fn send_error_to(w: &WriteHalfArc, msg: Message) {
    let mut guard = w.lock().await;
    let _ = send_message(&mut *guard, &msg).await;
}
