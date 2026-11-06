//! PLAN-054: finite desktop ownership of explicitly declared native HTTP services.
//! Uses the existing service CLI and owned process facility; no service adoption/retry.
use auto_tool_execution::{ExecutionSpec, OwnedProcess, StdioPolicy, TreeScope};
use serde_json::Value;
use std::{
    io::{BufRead, BufReader, Read, Write},
    net::{SocketAddr, TcpStream},
    path::{Path, PathBuf},
    sync::mpsc,
    time::{Duration, Instant},
};

const GRACE: Duration = Duration::from_secs(5);

#[derive(Debug)]
struct Declaration {
    id: String,
    root: PathBuf,
    port: u16,
}

// pac scalar fields are deliberately restricted to one top-level literal.
fn scalar<'a>(pac: &'a str, key: &str) -> Result<&'a str, String> {
    let mut values = pac.lines().filter_map(|line| {
        if line.starts_with(char::is_whitespace) || line.starts_with('#') {
            return None;
        }
        let (name, value) = line.split_once(':')?;
        (name.trim() == key).then_some(value.trim())
    });
    let value = values
        .next()
        .ok_or_else(|| format!("missing literal pac {key}"))?;
    if values.next().is_some() {
        return Err(format!("duplicate pac {key}"));
    }
    Ok(value)
}

fn declarations(apps_dir: &Path) -> Result<Vec<Declaration>, String> {
    use auto_ui_runtime::ui::app_registry as registry;
    let Some(os_root) = registry::resolve_os_manifest_root(apps_dir.parent().unwrap_or(apps_dir))
    else {
        return Ok(Vec::new());
    };
    let manifest: Value = serde_json::from_slice(
        &std::fs::read(os_root.join("apps.manifest")).map_err(|e| e.to_string())?,
    )
    .map_err(|e| format!("native HTTP manifest: {e}"))?;
    let apps = manifest["apps"]
        .as_array()
        .ok_or("native HTTP manifest apps missing")?;
    let entries = registry::aggregate_scan(
        apps_dir,
        &registry::host_extra_roots(),
        &registry::ScanOptions::default(),
    );
    let mut result = Vec::new();
    for app in apps {
        if app.get("native_http").is_none() {
            continue;
        }
        let enabled = app["native_http"]
            .as_bool()
            .ok_or("native_http must be boolean")?;
        if !enabled {
            continue;
        }
        let id = app["id"].as_str().ok_or("native HTTP id missing")?;
        if app["status"] != "active" || app["launch"] != "native" {
            return Err(format!("{id}: native_http requires active native launch"));
        }
        if result.iter().any(|d: &Declaration| d.id == id) {
            return Err(format!("duplicate native HTTP app {id}"));
        }
        let entry = entries
            .iter()
            .find(|e| e.id == id)
            .ok_or_else(|| format!("{id}: native HTTP registry source missing"))?;
        let root = entry
            .entry
            .ancestors()
            .find(|p| p.join("pac.at").is_file())
            .ok_or_else(|| format!("{id}: native HTTP pac root missing"))?
            .to_path_buf();
        let pac = std::fs::read_to_string(root.join("pac.at")).map_err(|e| e.to_string())?;
        if scalar(&pac, "api")?.trim_matches('"') != "rust" {
            return Err(format!("{id}: native HTTP requires pac api rust"));
        }
        let port: u16 = scalar(&pac, "back_port")?
            .parse()
            .map_err(|_| format!("{id}: invalid back_port"))?;
        if port == 0
            || !app["ports"]
                .as_array()
                .is_some_and(|ps| ps.iter().any(|p| p.as_u64() == Some(port.into())))
        {
            return Err(format!("{id}: back_port not in manifest ports"));
        }
        if result.iter().any(|d| d.port == port) {
            return Err(format!("{id}: duplicate native HTTP port"));
        }
        result.push(Declaration {
            id: id.to_owned(),
            root,
            port,
        });
    }
    Ok(result)
}

/// A finite owner, held until the desktop event loop returns.
#[derive(Default)]
pub struct DesktopBackends {
    services: Vec<Service>,
}

impl DesktopBackends {
    pub fn start(apps_dir: &Path) -> Result<Self, String> {
        let declarations = declarations(apps_dir)?;
        let mut owner = Self::default();
        if declarations.is_empty() {
            return Ok(owner);
        }
        let cli = auto_ui_runtime::ui::desktop_protocol::rqhost::resolve_auto_cli()
            .ok_or("native HTTP auto CLI missing")?;
        for d in declarations {
            let mut spec = ExecutionSpec::new([
                cli.as_os_str().to_owned(),
                "service".into(),
                "--server".into(),
                "rust".into(),
                "--dir".into(),
                d.root.as_os_str().to_owned(),
                "-B".into(),
                d.port.to_string().into(),
            ]);
            spec.cwd = Some(d.root.clone());
            spec.tree_scope = TreeScope::ProcessTree;
            spec.stderr = StdioPolicy::Inherit;
            spec.env_remove = [
                "AUTO_HTTP_BASE",
                "AUTO_HTTP_PORT",
                "AUTO_HTTP_SERVICE_JSON",
                "CHAT_TEST_MODE",
                "CHAT_LEGACY_ADOPT",
            ]
            .into_iter()
            .map(Into::into)
            .collect();
            owner
                .services
                .push(Service::start(d, spec, Duration::from_secs(610))?);
        }
        Ok(owner)
    }

    pub fn close(&mut self) -> Result<(), String> {
        let mut errors = Vec::new();
        for mut service in self.services.drain(..).rev() {
            if let Err(e) = service.close() {
                errors.push(format!("{}: {e}", service.id));
            }
        }
        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors.join("; "))
        }
    }
}

impl Drop for DesktopBackends {
    fn drop(&mut self) {
        if let Err(e) = self.close() {
            eprintln!("[native-http] teardown failed: {e}");
        }
    }
}

struct Service {
    id: String,
    port: u16,
    ready: Value,
    process: OwnedProcess,
    closed: bool,
    reader: Option<std::thread::JoinHandle<()>>,
}

impl Drop for Service {
    fn drop(&mut self) {
        if !self.closed {
            if let Err(e) = self.process.cancel_within(GRACE) {
                eprintln!(
                    "[native-http] id={} owned cancellation failed: {e}",
                    self.id
                );
            }
            if let Err(e) = self.reclaim_reader() {
                eprintln!("[native-http] id={} {e}", self.id);
            }
        }
    }
}

impl Service {
    fn start(d: Declaration, spec: ExecutionSpec, budget: Duration) -> Result<Self, String> {
        let addr: SocketAddr = ([127, 0, 0, 1], d.port).into();
        if TcpStream::connect_timeout(&addr, Duration::from_millis(300)).is_ok() {
            return Err(format!(
                "{}: native HTTP port {} occupied; occupant preserved",
                d.id, d.port
            ));
        }
        let process = auto_tool_execution::spawn(&spec)
            .map_err(|e| format!("{}: service spawn: {e}", d.id))?;
        let mut service = Self {
            id: d.id,
            port: d.port,
            ready: Value::Null,
            process,
            closed: false,
            reader: None,
        };
        let stdout = service
            .process
            .take_stdout()
            .ok_or("native HTTP stdout missing")?;
        let (tx, rx) = mpsc::channel();
        // The reader owns no process handle; stdout closes when this owned tree ends.
        service.reader = Some(
            std::thread::Builder::new()
                .name("desktop-service-ready".into())
                .spawn(move || {
                    for line in BufReader::new(stdout).lines() {
                        match line {
                            Ok(line) => {
                                if let Some(json) = line.strip_prefix("AUTO_SERVICE_READY ") {
                                    let _ = tx.send(
                                        serde_json::from_str::<Value>(json)
                                            .map_err(|_| "invalid READY JSON"),
                                    );
                                }
                            }
                            Err(_) => {
                                let _ = tx.send(Err("service stdout read failed"));
                                break;
                            }
                        }
                    }
                })
                .map_err(|e| format!("service reader spawn: {e}"))?,
        );
        let deadline = Instant::now() + budget;
        let ready = loop {
            if let Some(status) = service.process.try_wait().map_err(|e| e.to_string())? {
                return Err(format!(
                    "{}: service exited before READY: {status}",
                    service.id
                ));
            }
            if Instant::now() >= deadline {
                return Err(format!("{}: service READY deadline exceeded", service.id));
            }
            match rx.recv_timeout(Duration::from_millis(50)) {
                Ok(value) => break value.map_err(|e| format!("{}: {e}", service.id))?,
                Err(mpsc::RecvTimeoutError::Timeout) => (),
                Err(mpsc::RecvTimeoutError::Disconnected) => {
                    return Err(format!("{}: service stdout ended before READY", service.id))
                }
            }
        };
        service.ready = ready;
        service.check_identity()?;
        eprintln!(
            "[native-http] owner=desktop id={} cli_pid={} port={} ready=true",
            service.id,
            service.process.id(),
            service.port
        );
        Ok(service)
    }

    fn check_identity(&mut self) -> Result<(), String> {
        if self
            .process
            .try_wait()
            .map_err(|e| e.to_string())?
            .is_some()
        {
            return Err("owned service already exited".into());
        }
        let expected_bound = format!("127.0.0.1:{}", self.port);
        if self.ready["bound"] != expected_bound
            || self.ready["profile"] != "development"
            || self.ready["instance_id"].as_str().is_none_or(str::is_empty)
        {
            return Err("owned READY profile/bound/instance mismatch".into());
        }
        let (code, current) = request(self.port, "GET", "/__auto/health/ready")?;
        if code != 200
            || !["instance_id", "config_hash", "assembly_fingerprint"]
                .iter()
                .all(|key| {
                    self.ready[*key].as_str().is_some_and(|s| !s.is_empty())
                        && current[*key] == self.ready[*key]
                })
        {
            return Err("owned service identity changed; listener preserved".into());
        }
        Ok(())
    }

    fn close(&mut self) -> Result<(), String> {
        self.check_identity()?;
        let (code, body) = request(self.port, "POST", "/__auto/shutdown")?;
        if code != 202 || body["draining"] != true {
            return Err("owned shutdown refused".into());
        }
        match self
            .process
            .wait_deadline(GRACE)
            .map_err(|e| e.to_string())?
        {
            Some(status) if status.success() => {
                self.reclaim_reader()?;
                self.closed = true;
                eprintln!("[native-http] id={} graceful_exit=0", self.id);
                Ok(())
            }
            Some(status) => Err(format!("owned service exit {status}")),
            None => Err("owned shutdown deadline exceeded; own tree cancellation required".into()),
        }
    }

    fn reclaim_reader(&mut self) -> Result<(), String> {
        if let Some(reader) = &self.reader {
            let deadline = Instant::now() + GRACE;
            while !reader.is_finished() && Instant::now() < deadline {
                std::thread::sleep(Duration::from_millis(20));
            }
            if !reader.is_finished() {
                return Err("service stdout reader teardown deadline exceeded".into());
            }
        }
        if let Some(reader) = self.reader.take() {
            reader
                .join()
                .map_err(|_| "service stdout reader panicked")?;
        }
        Ok(())
    }
}

fn request(port: u16, method: &str, path: &str) -> Result<(u16, Value), String> {
    let mut stream = TcpStream::connect_timeout(&([127, 0, 0, 1], port).into(), GRACE)
        .map_err(|e| e.to_string())?;
    stream
        .set_read_timeout(Some(GRACE))
        .map_err(|e| e.to_string())?;
    stream
        .set_write_timeout(Some(GRACE))
        .map_err(|e| e.to_string())?;
    write!(stream, "{method} {path} HTTP/1.1\r\nHost: 127.0.0.1:{port}\r\nContent-Length: 0\r\nConnection: close\r\n\r\n").map_err(|e| e.to_string())?;
    let mut response = Vec::new();
    stream
        .take(65537)
        .read_to_end(&mut response)
        .map_err(|e| e.to_string())?;
    if response.len() > 65536 {
        return Err("service admin response too large".into());
    }
    let response = std::str::from_utf8(&response).map_err(|_| "invalid service response")?;
    let (headers, body) = response
        .split_once("\r\n\r\n")
        .ok_or("incomplete service response")?;
    let code = headers
        .split_whitespace()
        .nth(1)
        .and_then(|c| c.parse().ok())
        .ok_or("invalid service status")?;
    Ok((
        code,
        serde_json::from_str(body).map_err(|_| "invalid service admin JSON")?,
    ))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::net::TcpListener;

    // Executed as a separate owned process by the tests below. No production switch.
    #[test]
    fn native_backend_fixture() {
        let Ok(mode) = std::env::var("P054_BACKEND_FIXTURE") else {
            return;
        };
        if mode == "early" {
            return;
        }
        let port: u16 = std::env::var("P054_BACKEND_PORT").unwrap().parse().unwrap();
        let listener = TcpListener::bind(("127.0.0.1", port)).unwrap();
        if mode == "never" {
            std::thread::sleep(Duration::from_secs(300));
            return;
        }
        let ready = serde_json::json!({"bound":format!("127.0.0.1:{port}"),"profile":"development","instance_id":"owned-fixture","config_hash":"fixture-config","assembly_fingerprint":"fixture-source"});
        if mode == "malformed" {
            println!("AUTO_SERVICE_READY {{bad");
        } else {
            println!("AUTO_SERVICE_READY {ready}");
        }
        std::io::stdout().flush().unwrap();
        for conn in listener.incoming() {
            let mut conn = conn.unwrap();
            conn.set_read_timeout(Some(GRACE)).unwrap();
            let mut bytes = Vec::new();
            while !bytes.ends_with(b"\r\n\r\n") {
                let mut byte = [0];
                conn.read_exact(&mut byte).unwrap();
                bytes.push(byte[0]);
                assert!(bytes.len() < 2048);
            }
            let req = String::from_utf8_lossy(&bytes);
            let shutdown = req.starts_with("POST /__auto/shutdown ");
            let mut body = ready.clone();
            if mode == "mismatch" {
                body["instance_id"] = "foreign-listener".into();
            }
            if shutdown {
                body = serde_json::json!({"draining":true});
            }
            let body = body.to_string();
            write!(conn,"HTTP/1.1 {} OK\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}",if shutdown {202}else{200},body.len(),body).unwrap();
            if shutdown {
                break;
            }
        }
    }

    fn free_port() -> u16 {
        TcpListener::bind(("127.0.0.1", 0))
            .unwrap()
            .local_addr()
            .unwrap()
            .port()
    }
    fn fixture(mode: &str, port: u16) -> (Declaration, ExecutionSpec) {
        let mut spec = ExecutionSpec::new([
            std::env::current_exe().unwrap().into_os_string(),
            "--exact".into(),
            "native_backends::tests::native_backend_fixture".into(),
            "--nocapture".into(),
            "--format=terse".into(),
        ]);
        spec.env = vec![
            ("P054_BACKEND_FIXTURE".into(), mode.into()),
            ("P054_BACKEND_PORT".into(), port.to_string().into()),
        ];
        spec.stderr = StdioPolicy::Inherit;
        spec.tree_scope = TreeScope::ProcessTree;
        (
            Declaration {
                id: "fixture".into(),
                root: PathBuf::new(),
                port,
            },
            spec,
        )
    }
    #[test]
    fn p054_owned_backend_ready_and_graceful_close() {
        let port = free_port();
        let (d, s) = fixture("normal", port);
        let mut service = Service::start(d, s, Duration::from_secs(5)).unwrap();
        assert!(service.process.try_wait().unwrap().is_none());
        service.close().unwrap();
        assert!(service.closed);
        assert!(TcpStream::connect(("127.0.0.1", port)).is_err());
    }
    #[test]
    fn p054_owned_backend_failures_cancel_only_owned_service() {
        for mode in ["early", "never", "malformed", "mismatch"] {
            let observer = TcpListener::bind(("127.0.0.1", 0)).unwrap();
            let port = free_port();
            let (d, s) = fixture(mode, port);
            let error = Service::start(d, s, Duration::from_millis(1200))
                .err()
                .expect("bad service must fail");
            assert!(!error.is_empty(), "{mode}");
            assert!(
                TcpStream::connect(("127.0.0.1", port)).is_err(),
                "{mode}: own listener leaked"
            );
            assert!(
                TcpStream::connect(observer.local_addr().unwrap()).is_ok(),
                "{mode}: observer lost"
            );
        }
    }
    #[test]
    fn p054_occupied_port_preserves_occupant_without_spawn() {
        let occupant = TcpListener::bind(("127.0.0.1", 0)).unwrap();
        let port = occupant.local_addr().unwrap().port();
        let (d, mut s) = fixture("normal", port);
        s.argv[0] = "nonexistent-must-not-spawn".into();
        let error = Service::start(d, s, Duration::from_secs(1)).err().unwrap();
        assert!(error.contains("occupied"), "{error}");
        assert!(TcpStream::connect(occupant.local_addr().unwrap()).is_ok());
    }
    #[test]
    fn p054_literal_profile_rejects_missing_duplicate_or_nested_fields() {
        assert_eq!(
            scalar(
                "api: \"rust\"\n  back_port: 1234\nback_port: 17823\n",
                "back_port"
            )
            .unwrap(),
            "17823"
        );
        assert!(scalar("back_port: 17823\nback_port: 17824", "back_port").is_err());
        assert!(scalar("dep x {\n    api: \"rust\"\n}", "api").is_err());
    }
}
