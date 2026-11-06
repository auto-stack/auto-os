//! PLAN-054: own only the product desktop's private rqhost instance.
//! DirectChildOnly intentionally preserves the existing no-cascade behavior
//! when an external termination skips the desktop's destructors.
use auto_tool_execution::{ExecutionSpec, OwnedProcess, StdioPolicy, TreeScope};
use std::time::{Duration, Instant};

const GRACE: Duration = Duration::from_secs(5);

#[derive(Default)]
pub struct DesktopRqhost {
    process: Option<OwnedProcess>,
    pipe: String,
}

impl DesktopRqhost {
    pub fn start(enabled: bool) -> Result<Self, String> {
        if !enabled {
            return Ok(Self::default());
        }
        use auto_ui_runtime::ui::desktop_protocol::{rqhost, transport};
        let cli = rqhost::resolve_auto_cli().ok_or("private rqhost auto CLI missing")?;
        let pipe = format!("{}-desktop-{}", rqhost::RQHOST_PIPE, std::process::id());
        let mut spec = ExecutionSpec::new([
            cli.into_os_string(),
            "rqhost".into(),
            "--pipe".into(),
            pipe.clone().into(),
        ]);
        spec.stdout = StdioPolicy::Inherit;
        spec.stderr = StdioPolicy::Inherit;
        spec.tree_scope = TreeScope::DirectChildOnly;
        // Match the existing framework launcher: test-runner control variables
        // belong to the parent, not to the actual rqhost CLI.
        spec.env_remove = std::env::vars_os()
            .map(|(key, _)| key)
            .filter(|key| key.to_string_lossy().starts_with("NEXTEST_"))
            .collect();
        let endpoint = pipe.clone();
        Self::spawn_checked(
            spec,
            pipe,
            move || transport::connect(&endpoint, 100).is_ok(),
            GRACE,
        )
    }

    pub fn endpoint(&self) -> Option<&str> {
        self.process.as_ref().map(|_| self.pipe.as_str())
    }

    fn spawn_checked(
        spec: ExecutionSpec,
        pipe: String,
        mut probe: impl FnMut() -> bool,
        budget: Duration,
    ) -> Result<Self, String> {
        if spec.tree_scope != TreeScope::DirectChildOnly {
            return Err("private rqhost requires DirectChildOnly compatibility scope".into());
        }
        if probe() {
            return Err(format!(
                "private rqhost endpoint {pipe} already occupied; occupant preserved"
            ));
        }
        let process =
            auto_tool_execution::spawn(&spec).map_err(|e| format!("private rqhost spawn: {e}"))?;
        let mut owner = Self {
            process: Some(process),
            pipe,
        };
        let deadline = Instant::now() + budget;
        let failure = loop {
            if let Some(status) = owner
                .process
                .as_mut()
                .unwrap()
                .try_wait()
                .map_err(|e| e.to_string())?
            {
                break format!("private rqhost exited before ready: {status}");
            }
            if probe() {
                if let Some(status) = owner
                    .process
                    .as_mut()
                    .unwrap()
                    .try_wait()
                    .map_err(|e| e.to_string())?
                {
                    break format!("private rqhost exited during ready: {status}");
                }
                eprintln!("[private-rqhost] owner=desktop pid={} endpoint={} ready=true scope=DirectChildOnly",
                    owner.process.as_ref().unwrap().id(), owner.pipe);
                return Ok(owner);
            }
            if Instant::now() >= deadline {
                break "private rqhost ready deadline exceeded".into();
            }
            std::thread::sleep(Duration::from_millis(20));
        };
        match owner.close() {
            Ok(()) => Err(failure),
            Err(cleanup) => Err(format!("{failure}; cleanup: {cleanup}")),
        }
    }

    pub fn close(&mut self) -> Result<(), String> {
        let Some(mut process) = self.process.take() else {
            return Ok(());
        };
        let pid = process.id();
        if let Some(status) = process.try_wait().map_err(|e| e.to_string())? {
            eprintln!("[private-rqhost] pid={pid} already_ended=true status={status}");
            return if status.success() {
                Ok(())
            } else {
                Err(format!(
                    "private rqhost {pid} exited unexpectedly: {status}"
                ))
            };
        }
        let status = process
            .cancel_within(GRACE)
            .map_err(|e| format!("private rqhost {pid} cancellation/reap: {e}"))?;
        // This is an intentional idle-helper cancellation, never a graceful-0 claim.
        eprintln!("[private-rqhost] pid={pid} controlled_cancel=true reaped=true status={status}");
        Ok(())
    }
}

impl Drop for DesktopRqhost {
    fn drop(&mut self) {
        if let Err(e) = self.close() {
            eprintln!("[private-rqhost] teardown failed: {e}");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{
        io::{Read, Write},
        net::{TcpListener, TcpStream},
    };

    fn connects(port: u16) -> bool {
        TcpStream::connect(("127.0.0.1", port)).is_ok()
    }
    fn free_port() -> u16 {
        TcpListener::bind(("127.0.0.1", 0))
            .unwrap()
            .local_addr()
            .unwrap()
            .port()
    }
    fn fixture_spec(mode: &str, port: u16) -> ExecutionSpec {
        let mut spec = ExecutionSpec::new([
            std::env::current_exe().unwrap().into_os_string(),
            "--exact".into(),
            "private_rqhost::tests::private_rqhost_fixture".into(),
            "--nocapture".into(),
            "--format=terse".into(),
        ]);
        spec.env = vec![
            ("P054_RQ_FIXTURE".into(), mode.into()),
            ("P054_RQ_PORT".into(), port.to_string().into()),
        ];
        spec.stdout = StdioPolicy::Null;
        spec.stderr = StdioPolicy::Null;
        spec.tree_scope = TreeScope::DirectChildOnly;
        spec
    }
    fn start_fixture(port: u16) -> DesktopRqhost {
        DesktopRqhost::spawn_checked(
            fixture_spec("server", port),
            "fixture".into(),
            || connects(port),
            Duration::from_secs(3),
        )
        .unwrap()
    }

    #[test]
    fn private_rqhost_fixture() {
        let Ok(mode) = std::env::var("P054_RQ_FIXTURE") else {
            return;
        };
        if mode == "early" {
            return;
        }
        let port: u16 = std::env::var("P054_RQ_PORT").unwrap().parse().unwrap();
        if mode == "owner" {
            let owner = start_fixture(port);
            std::mem::forget(owner);
            // Real process exit closes kernel handles while deliberately skipping Drop.
            std::process::exit(0);
        }
        let listener = TcpListener::bind(("127.0.0.1", port)).unwrap();
        listener.set_nonblocking(true).unwrap();
        let deadline = Instant::now() + Duration::from_secs(8);
        while Instant::now() < deadline {
            if let Ok((mut client, _)) = listener.accept() {
                client
                    .set_read_timeout(Some(Duration::from_millis(50)))
                    .unwrap();
                let mut bytes = [0; 4];
                if client.read(&mut bytes).ok() == Some(4) && &bytes == b"quit" {
                    break;
                }
            } else {
                std::thread::sleep(Duration::from_millis(10));
            }
        }
    }

    #[test]
    fn p054_private_normal_close_and_drop_preserve_observer() {
        let observer = TcpListener::bind(("127.0.0.1", 0)).unwrap();
        for via_drop in [false, true] {
            let port = free_port();
            let mut owner = start_fixture(port);
            assert!(connects(port));
            if via_drop {
                drop(owner);
            } else {
                owner.close().unwrap();
                owner.close().unwrap();
            }
            assert!(!connects(port), "own listener leaked");
            assert!(
                TcpStream::connect(observer.local_addr().unwrap()).is_ok(),
                "observer lost"
            );
        }
    }

    #[test]
    fn p054_private_occupied_endpoint_refuses_before_spawn() {
        let observer = TcpListener::bind(("127.0.0.1", 0)).unwrap();
        let port = observer.local_addr().unwrap().port();
        let mut spec = fixture_spec("server", port);
        spec.argv[0] = "nonexistent-must-not-spawn".into();
        let error = DesktopRqhost::spawn_checked(
            spec,
            "occupied".into(),
            || connects(port),
            Duration::from_secs(1),
        )
        .err()
        .unwrap();
        assert!(error.contains("occupied"));
        assert!(connects(port));
    }

    #[test]
    fn p054_private_start_failures_reap_owned_listener() {
        let port = free_port();
        let error = DesktopRqhost::spawn_checked(
            fixture_spec("early", port),
            "early".into(),
            || false,
            Duration::from_secs(3),
        )
        .err()
        .unwrap();
        assert!(error.contains("exited before ready"), "{error}");
        let port = free_port();
        let mut saw_listener = false;
        let error = DesktopRqhost::spawn_checked(
            fixture_spec("server", port),
            "never-ready".into(),
            || {
                saw_listener |= connects(port);
                false
            },
            Duration::from_secs(2),
        )
        .err()
        .unwrap();
        assert!(
            saw_listener,
            "must observe actual owned listener before the timeout"
        );
        assert!(error.contains("deadline exceeded"), "{error}");
        assert!(!connects(port), "observed owned listener not reclaimed");
    }

    #[test]
    fn p054_private_owner_exit_without_drop_does_not_cascade() {
        let port = free_port();
        let mut owner = auto_tool_execution::spawn(&fixture_spec("owner", port)).unwrap();
        let status = owner
            .wait_deadline(Duration::from_secs(4))
            .unwrap()
            .expect("fixture owner did not exit");
        assert!(status.success());
        assert!(
            connects(port),
            "direct child must survive owner exit that skips destructors"
        );
        let mut client = TcpStream::connect(("127.0.0.1", port)).unwrap();
        client.write_all(b"quit").unwrap();
        drop(client);
        let deadline = Instant::now() + Duration::from_secs(2);
        while connects(port) && Instant::now() < deadline {
            std::thread::sleep(Duration::from_millis(20));
        }
        assert!(!connects(port), "test-owned orphan fixture did not stop");
    }

    #[test]
    fn p054_private_disabled_has_no_process_or_endpoint() {
        let mut owner = DesktopRqhost::start(false).unwrap();
        assert!(owner.endpoint().is_none());
        assert!(owner.process.is_none());
        owner.close().unwrap();
    }
}
