//! Shared harness for integration test families (f1_route, capture_receipt,
//! resources, lifecycle). Every process is Job-Object managed; run dirs are
//! private per-test and removed unless COMM049_KEEP_RUN=1.

#![allow(dead_code)]

use autoos_comm_core::protocol::{
    CallToProvider, Error as ErrorMessage, Hello, Message, Register, Registered, ReplyFromProvider,
    Welcome, CODEC_ID, INTERFACE_CAPTURE, PROTOCOL_VERSION, SCHEMA_CAPTURE_CAPTURE,
    SERVICE_ID_CAPTURE,
};
pub use autoos_comm_demo::support;
use autoos_comm_demo::support::{BrokerConfigFile, ProviderServiceConfig};
use autoos_comm_local::job::{JobGuard, ManagedChild};
use autoos_comm_local::pipe::{connect_pipe, read_message, send_message};
use std::path::{Path, PathBuf};
use std::time::{Duration, Instant};

pub const CALLER_A: &str = "caller-a";
pub const CALLER_B: &str = "caller-b";
pub const DENIED_CALLER: &str = "denied-caller";
pub const PROVIDER: &str = "capture-provider";

pub fn bin(name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_BIN_EXE_comm-broker"))
        .parent()
        .unwrap()
        .join(name)
}

pub struct TestRun {
    pub run_dir: PathBuf,
    pub job: JobGuard,
    pub endpoint: Option<support::BrokerEndpoint>,
}

impl TestRun {
    /// Standard four-principal access config: provider, two permitted
    /// callers, one no-permission caller (V-04 counterexample).
    pub fn new(tag: &str) -> TestRun {
        let run_dir = support::create_run_dir(tag);
        let job = JobGuard::new().expect("job object");
        let _creds = support::write_access_config(
            &run_dir,
            &[
                (PROVIDER, support::ROLE_PROVIDER, vec![SERVICE_ID_CAPTURE]),
                (CALLER_A, support::ROLE_CALLER, vec![SERVICE_ID_CAPTURE]),
                (CALLER_B, support::ROLE_CALLER, vec![SERVICE_ID_CAPTURE]),
                (DENIED_CALLER, support::ROLE_CALLER, vec![]),
            ],
        );
        TestRun {
            run_dir,
            job,
            endpoint: None,
        }
    }

    pub fn deadline(&self) -> Instant {
        Instant::now() + Duration::from_secs(15)
    }

    pub fn spawn_broker(&self) -> ManagedChild {
        let cfg = BrokerConfigFile {
            run_dir: self.run_dir.to_string_lossy().to_string(),
            pipe_name: None,
            policy_file: self
                .run_dir
                .join("broker-access.json")
                .to_string_lossy()
                .to_string(),
        };
        let p = self.run_dir.join("broker-cfg.json");
        support::write_json(&p, &cfg);
        let (child, _) = support::spawn_managed(
            &self.job,
            &bin("comm-broker.exe"),
            &["--config".to_string(), p.to_string_lossy().to_string()],
            &[],
            &self.run_dir,
            &support::logs_dir(&self.run_dir).join("broker-stderr.log"),
        )
        .expect("spawn broker");
        child
    }

    /// Broker bound to a fixed pipe name (rebind/reuse counterexamples).
    pub fn spawn_broker_named(&self, pipe_name: &str) -> ManagedChild {
        let cfg = BrokerConfigFile {
            run_dir: self.run_dir.to_string_lossy().to_string(),
            pipe_name: Some(pipe_name.to_string()),
            policy_file: self
                .run_dir
                .join("broker-access.json")
                .to_string_lossy()
                .to_string(),
        };
        let p = self.run_dir.join(format!(
            "broker-cfg-{}.json",
            autoos_comm_core::ids::new_id()
        ));
        support::write_json(&p, &cfg);
        let (child, _) = support::spawn_managed(
            &self.job,
            &bin("comm-broker.exe"),
            &["--config".to_string(), p.to_string_lossy().to_string()],
            &[],
            &self.run_dir,
            &support::logs_dir(&self.run_dir).join("broker-stderr.log"),
        )
        .expect("spawn broker named");
        child
    }

    pub fn spawn_provider(&self, barrier: Option<&str>) -> ManagedChild {
        let cfg = ProviderServiceConfig {
            run_dir: self.run_dir.to_string_lossy().to_string(),
            token_file: support::token_file(&self.run_dir, PROVIDER)
                .to_string_lossy()
                .to_string(),
            endpoint_file: support::endpoint_file(&self.run_dir)
                .to_string_lossy()
                .to_string(),
            stop_file: Some(
                support::stop_file(&self.run_dir, "provider")
                    .to_string_lossy()
                    .to_string(),
            ),
            barrier: barrier.map(String::from),
        };
        let p = self.run_dir.join("provider-cfg.json");
        support::write_json(&p, &cfg);
        let (child, _) = support::spawn_managed(
            &self.job,
            &bin("capture-inbox.exe"),
            &["--config".to_string(), p.to_string_lossy().to_string()],
            &[],
            &self.run_dir,
            &support::logs_dir(&self.run_dir).join("provider-stderr.log"),
        )
        .expect("spawn provider");
        child
    }

    /// Start broker + binary provider and record the endpoint.
    pub async fn start_stack(&mut self) -> (ManagedChild, ManagedChild) {
        let broker = self.spawn_broker();
        let endpoint = support::wait_endpoint(&self.run_dir, self.deadline()).expect("endpoint");
        self.endpoint = Some(endpoint);
        let provider = self.spawn_provider(None);
        let ready = support::wait_provider_ready(&self.run_dir, self.deadline()).expect("ready");
        assert_eq!(
            ready.broker_epoch,
            self.endpoint.as_ref().unwrap().broker_epoch
        );
        (broker, provider)
    }

    pub fn pipe(&self) -> String {
        self.endpoint.as_ref().expect("endpoint").pipe_name.clone()
    }

    /// Library client for one of the configured principals.
    pub async fn client(&self, principal: &str) -> autoos_comm_local::client::CommClient {
        let token = self.token(principal);
        autoos_comm_local::client::CommClient::connect(&self.pipe(), &token, self.deadline())
            .await
            .expect("client connect")
    }

    pub fn token(&self, principal: &str) -> String {
        std::fs::read_to_string(support::token_file(&self.run_dir, principal))
            .expect("token file")
            .trim()
            .to_string()
    }

    /// Discover the capture service through `client`.
    pub async fn discover(
        &self,
        client: &autoos_comm_local::client::CommClient,
    ) -> autoos_comm_core::protocol::ServiceRef {
        client.discover(SERVICE_ID_CAPTURE).await.expect("discover")
    }

    pub fn log_text(&self, name: &str) -> String {
        std::fs::read_to_string(support::logs_dir(&self.run_dir).join(name)).unwrap_or_default()
    }
}

impl Drop for TestRun {
    fn drop(&mut self) {
        support::request_stop(&self.run_dir, "broker");
        support::request_stop(&self.run_dir, "provider");
        std::thread::sleep(Duration::from_millis(300));
        if std::env::var("COMM049_KEEP_RUN").is_err() {
            support::remove_run_dir(&self.run_dir);
        }
    }
}

/// Hand-rolled provider connection for fault-injection scenarios: registers
/// the capture service, then hands raw halves to the test.
pub struct RawProvider {
    pub write: std::sync::Arc<
        tokio::sync::Mutex<tokio::io::WriteHalf<tokio::net::windows::named_pipe::NamedPipeClient>>,
    >,
    pub read: tokio::io::ReadHalf<tokio::net::windows::named_pipe::NamedPipeClient>,
    pub service_ref: autoos_comm_core::protocol::ServiceRef,
}

impl RawProvider {
    pub async fn connect(run: &TestRun, descriptor: &str) -> RawProvider {
        let token = run.token(PROVIDER);
        let client = connect_pipe(&run.pipe(), run.deadline())
            .await
            .expect("raw provider connect");
        let (r, w) = tokio::io::split(client);
        let w = std::sync::Arc::new(tokio::sync::Mutex::new(w));
        {
            let mut gw = w.lock().await;
            send_message(
                &mut *gw,
                &Message::Hello(Hello {
                    protocol_version: PROTOCOL_VERSION,
                    codec: CODEC_ID.to_string(),
                    token,
                }),
            )
            .await
            .expect("raw hello");
        }
        let mut r = r;
        match read_message(&mut r)
            .await
            .expect("raw welcome")
            .expect("welcome frame")
        {
            Message::Welcome(_) => {}
            other => panic!("expected welcome, got {}", other.op_name()),
        }
        {
            let mut gw = w.lock().await;
            send_message(
                &mut *gw,
                &Message::Register(Register {
                    corr_id: 1,
                    service_id: SERVICE_ID_CAPTURE.to_string(),
                    interface: INTERFACE_CAPTURE.to_string(),
                    schema_id: SCHEMA_CAPTURE_CAPTURE.to_string(),
                    descriptor: descriptor.to_string(),
                }),
            )
            .await
            .expect("raw register");
        }
        let service_ref = match read_message(&mut r)
            .await
            .expect("registered")
            .expect("frame")
        {
            Message::Registered(reg) => reg.service_ref,
            Message::Error(e) => panic!("register rejected: {:?}", e.code),
            other => panic!("op {}", other.op_name()),
        };
        RawProvider {
            write: w,
            read: r,
            service_ref,
        }
    }

    /// Await the next forwarded call (with a test budget).
    pub async fn next_call(&mut self) -> CallToProvider {
        let deadline = Instant::now() + Duration::from_secs(5);
        loop {
            if Instant::now() > deadline {
                panic!("no forwarded call within budget");
            }
            match tokio::time::timeout(Duration::from_secs(5), read_message(&mut self.read)).await {
                Ok(Ok(Some(Message::CallToProvider(c)))) => return c,
                Ok(Ok(Some(_))) => continue,
                Ok(Ok(None)) => panic!("provider connection closed while waiting for call"),
                Ok(Err(e)) => panic!("provider read error: {e:?}"),
                Err(_) => panic!("next_call budget exceeded"),
            }
        }
    }

    /// Reply on a route with an arbitrary body.
    pub async fn reply(&self, route_id: &str, body: serde_json::Value) {
        let mut w = self.write.lock().await;
        send_message(
            &mut *w,
            &Message::ReplyFromProvider(ReplyFromProvider {
                route_id: route_id.to_string(),
                body,
            }),
        )
        .await
        .expect("provider reply");
    }

    /// Send a structured error on a route.
    pub async fn error(&self, route_id: &str, e: &autoos_comm_core::error::CommError) {
        let mut w = self.write.lock().await;
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
        .expect("provider error");
    }
}

/// Capture one text through a library client; returns the capture_id.
pub async fn capture_ok(
    client: &autoos_comm_local::client::CommClient,
    sref: &autoos_comm_core::protocol::ServiceRef,
    op: &str,
    text: &str,
) -> String {
    let body = serde_json::json!({ "operation_id": op, "text": text });
    let reply = client
        .call(sref, "capture", SCHEMA_CAPTURE_CAPTURE, 2000, body)
        .await
        .expect("capture ok");
    assert_eq!(reply["status"], "committed", "{reply}");
    reply["capture_id"]
        .as_str()
        .expect("capture_id")
        .to_string()
}

/// True when the error carries this code.
pub fn is_code(
    e: &autoos_comm_core::error::CommError,
    c: autoos_comm_core::error::ErrorCode,
) -> bool {
    e.code == c
}

pub fn new_op() -> String {
    autoos_comm_core::ids::new_id()
}

/// Strip a run dir path prefix for concise assertion messages.
pub fn disp(p: &Path) -> String {
    p.to_string_lossy().to_string()
}

/// Async barrier/endpoint/provider-ready waits for use inside join!.
pub async fn wait_barrier_async(
    run_dir: &Path,
    operation_id: &str,
    deadline: Instant,
) -> std::io::Result<String> {
    let dir = support::barrier_dir(run_dir);
    let p = dir.join(format!("barrier-{operation_id}.json"));
    #[derive(serde::Deserialize)]
    struct Marker {
        barrier: String,
    }
    loop {
        if Instant::now() >= deadline {
            return Err(std::io::Error::new(
                std::io::ErrorKind::TimedOut,
                "barrier wait",
            ));
        }
        if let Ok(bytes) = std::fs::read(&p) {
            if let Ok(m) = serde_json::from_slice::<Marker>(&bytes) {
                return Ok(m.barrier);
            }
        }
        tokio::time::sleep(Duration::from_millis(25)).await;
    }
}

pub async fn wait_provider_ready_async(
    run_dir: &Path,
    deadline: Instant,
) -> std::io::Result<support::ProviderReady> {
    let p = support::provider_ready_file(run_dir);
    loop {
        if Instant::now() >= deadline {
            return Err(std::io::Error::new(
                std::io::ErrorKind::TimedOut,
                "provider-ready wait",
            ));
        }
        if let Ok(v) = support::read_json::<support::ProviderReady>(&p) {
            return Ok(v);
        }
        tokio::time::sleep(Duration::from_millis(25)).await;
    }
}
