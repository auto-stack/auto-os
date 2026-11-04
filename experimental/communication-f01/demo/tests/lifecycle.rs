//! Lifecycle / full-path integration tests (T-01 gate + V-06/V-13 cases).
//!
//! The T-01 feasibility gate runs the entire F1 path across three real
//! processes: broker, capture provider, client binaries. Every process is
//! Job-Object managed (suspended → assign → resume); failure never leaves an
//! unmanaged run.

use autoos_comm_demo::support::{self, BrokerConfigFile, ProviderServiceConfig};
use autoos_comm_local::job::JobGuard;
use std::io::Read;
use std::path::{Path, PathBuf};
use std::time::{Duration, Instant};

fn bin(name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_BIN_EXE_comm-broker"))
        .parent()
        .unwrap()
        .join(name)
}

struct TestRun {
    run_dir: PathBuf,
    job: JobGuard,
}

impl TestRun {
    fn new(tag: &str) -> TestRun {
        let run_dir = support::create_run_dir(tag);
        let job = JobGuard::new().expect("job object");
        TestRun { run_dir, job }
    }
}

impl Drop for TestRun {
    fn drop(&mut self) {
        support::request_stop(&self.run_dir, "broker");
        support::request_stop(&self.run_dir, "provider");
        std::thread::sleep(Duration::from_millis(300));
        // triage hook: set COMM049_KEEP_RUN=1 to inspect run artifacts
        if std::env::var("COMM049_KEEP_RUN").is_err() {
            support::remove_run_dir(&self.run_dir);
        }
    }
}

fn write_broker_config(run_dir: &Path) -> PathBuf {
    let cfg = BrokerConfigFile {
        run_dir: run_dir.to_string_lossy().to_string(),
        pipe_name: None,
        policy_file: run_dir
            .join("broker-access.json")
            .to_string_lossy()
            .to_string(),
    };
    let p = run_dir.join("broker-cfg.json");
    support::write_json(&p, &cfg);
    p
}

fn write_provider_config(run_dir: &Path, barrier: Option<&str>) -> PathBuf {
    let cfg = ProviderServiceConfig {
        run_dir: run_dir.to_string_lossy().to_string(),
        token_file: support::token_file(run_dir, "capture-provider")
            .to_string_lossy()
            .to_string(),
        endpoint_file: support::endpoint_file(run_dir)
            .to_string_lossy()
            .to_string(),
        stop_file: Some(
            support::stop_file(run_dir, "provider")
                .to_string_lossy()
                .to_string(),
        ),
        barrier: barrier.map(String::from),
    };
    let p = run_dir.join("provider-cfg.json");
    support::write_json(&p, &cfg);
    p
}

fn spawn_broker(run: &TestRun) -> autoos_comm_local::job::ManagedChild {
    let cfg = write_broker_config(&run.run_dir);
    let (child, _) = support::spawn_managed(
        &run.job,
        &bin("comm-broker.exe"),
        &["--config".to_string(), cfg.to_string_lossy().to_string()],
        &[],
        &run.run_dir,
        &support::logs_dir(&run.run_dir).join("broker-stderr.log"),
    )
    .expect("spawn broker");
    child
}

fn spawn_provider(run: &TestRun, barrier: Option<&str>) -> autoos_comm_local::job::ManagedChild {
    let cfg = write_provider_config(&run.run_dir, barrier);
    let (child, _) = support::spawn_managed(
        &run.job,
        &bin("capture-inbox.exe"),
        &["--config".to_string(), cfg.to_string_lossy().to_string()],
        &[],
        &run.run_dir,
        &support::logs_dir(&run.run_dir).join("provider-stderr.log"),
    )
    .expect("spawn provider");
    child
}

/// Spawn a one-shot client and collect its stdout JSON lines.
fn run_client(run_dir: &Path, job: &JobGuard, principal: &str, extra: &[String]) -> (i32, String) {
    let mut args = vec![
        "--endpoint".to_string(),
        support::endpoint_file(run_dir)
            .to_string_lossy()
            .to_string(),
        "--token-file".to_string(),
        support::token_file(run_dir, principal)
            .to_string_lossy()
            .to_string(),
    ];
    args.extend_from_slice(extra);
    let (child, stdout) = support::spawn_managed(
        job,
        &bin("comm-client.exe"),
        &args,
        &[],
        run_dir,
        &support::logs_dir(run_dir).join("client-stderr.log"),
    )
    .expect("spawn client");
    let code = child
        .wait_for(Duration::from_secs(10))
        .expect("client wait")
        .expect("client exit within 10s");
    let mut out = String::new();
    if let Some(mut f) = stdout {
        let _ = f.read_to_string(&mut out);
    }
    (code as i32, out)
}

/// T-01 full-path gate: three real processes, one capture/receipt round,
/// explicit DACL on the pipe handle, bad-token rejection, persistence across
/// broker restart, idempotent retry, managed shutdown.
#[tokio::test(flavor = "multi_thread")]
async fn t01_full_path_three_processes_capture_receipt_restart() {
    let run = TestRun::new("t01");
    let deadline = Instant::now() + Duration::from_secs(15);

    // access config: provider + one permitted caller + one denied caller
    let creds = support::write_access_config(
        &run.run_dir,
        &[
            (
                "capture-provider",
                support::ROLE_PROVIDER,
                vec!["auto.capture.inbox"],
            ),
            ("caller-a", support::ROLE_CALLER, vec!["auto.capture.inbox"]),
            ("denied-caller", support::ROLE_CALLER, vec![]),
        ],
    );

    // 1. broker
    let broker = spawn_broker(&run);
    let endpoint = support::wait_endpoint(&run.run_dir, deadline).expect("endpoint");
    assert!(!endpoint.pipe_name.is_empty());
    assert!(
        endpoint.pipe_name.contains("autoos-comm-f01-"),
        "random experimental pipe name: {}",
        endpoint.pipe_name
    );

    // 2. explicit DACL on the actual pipe object (V-05 evidence)
    {
        let client = autoos_comm_local::pipe::connect_pipe(&endpoint.pipe_name, deadline)
            .await
            .expect("probe connect");
        use std::os::windows::io::AsRawHandle;
        let h = client.as_raw_handle();
        let sids = autoos_comm_local::pipe::pipe_dacl_sids(h).expect("dacl");
        let me = autoos_comm_local::pipe::current_user_sid_string().unwrap();
        assert_eq!(
            sids.len(),
            2,
            "exactly two ACEs (user+SYSTEM), got {sids:?}"
        );
        assert!(sids.contains(&me), "current user granted: {sids:?}");
        assert!(
            sids.contains(&"S-1-5-18".to_string()),
            "SYSTEM granted: {sids:?}"
        );
        drop(client);
    }

    // 3. provider
    let provider = spawn_provider(&run, None);
    let ready = support::wait_provider_ready(&run.run_dir, deadline).expect("provider ready");
    assert_eq!(ready.broker_epoch, endpoint.broker_epoch);

    // 4. real client binary: capture one text (three distinct PIDs by now)
    let op1 = autoos_comm_core::ids::new_id();
    let text1 = "PLAN-049 首验文本\nline two — UTF-8 preserved\n";
    let (code, out) = run_client(
        &run.run_dir,
        &run.job,
        "caller-a",
        &[
            "--cmd".to_string(),
            "capture".to_string(),
            "--op".to_string(),
            op1.clone(),
            "--text".to_string(),
            text1.to_string(),
        ],
    );
    assert_eq!(code, 0, "capture via real client failed: {out}");
    let capture_id = parse_field(&out, "capture_id").expect("capture_id");

    // 5. receipt via real client: same committed content
    let (code, out) = run_client(
        &run.run_dir,
        &run.job,
        "caller-a",
        &[
            "--cmd".to_string(),
            "receipt".to_string(),
            "--op".to_string(),
            op1.clone(),
        ],
    );
    assert_eq!(code, 0, "receipt via real client failed: {out}");
    assert_eq!(parse_field(&out, "status").as_deref(), Some("committed"));
    assert_eq!(
        parse_field(&out, "capture_id").as_deref(),
        Some(capture_id.as_str())
    );

    // 6. independent read-only DB verification from this process
    {
        let db = run.run_dir.join("inbox-db").join("capture.db");
        let conn =
            rusqlite::Connection::open_with_flags(&db, rusqlite::OpenFlags::SQLITE_OPEN_READ_ONLY)
                .expect("independent readonly open");
        let (n, t): (i64, Vec<u8>) = conn
            .query_row(
                "SELECT COUNT(*), COALESCE(text, X'') FROM captures",
                [],
                |r| Ok((r.get(0)?, r.get(1)?)),
            )
            .expect("independent count");
        assert_eq!(n, 1, "exactly one row after one capture");
        assert_eq!(String::from_utf8_lossy(&t), text1, "raw content preserved");
    }

    // 7. retry same key+payload via library client: same id, no new row
    {
        let client = autoos_comm_local::client::CommClient::connect(
            &endpoint.pipe_name,
            &creds.tokens["caller-a"],
            deadline,
        )
        .await
        .expect("library client");
        let sref = client
            .discover("auto.capture.inbox")
            .await
            .expect("discover");
        let body = serde_json::json!({ "operation_id": op1, "text": text1 });
        let reply = client
            .call(
                &sref,
                "capture",
                autoos_comm_core::protocol::SCHEMA_CAPTURE_CAPTURE,
                2000,
                body,
            )
            .await
            .expect("retry capture");
        assert_eq!(
            reply["capture_id"],
            capture_id.as_str(),
            "same key+payload → same id"
        );
    }

    // 8. bad credential is rejected at Hello, before any business routing
    {
        let bad = autoos_comm_core::ids::new_token();
        let r = autoos_comm_local::client::CommClient::connect(&endpoint.pipe_name, &bad, deadline)
            .await;
        match r {
            Err(e) => assert!(
                matches!(e.code, autoos_comm_core::error::ErrorCode::PermissionDenied),
                "bad token → permission_denied, got {e:?}"
            ),
            Ok(_) => panic!("bad token must not authenticate"),
        }
    }

    // 9. broker restart: new epoch, provider re-registers against it
    broker.kill();
    let broker2 = spawn_broker(&run);
    let deadline2 = Instant::now() + Duration::from_secs(15);
    // the endpoint file may briefly hold the old broker's data
    let mut endpoint2 = support::wait_endpoint(&run.run_dir, deadline2).expect("endpoint2");
    loop {
        if endpoint2.broker_epoch != endpoint.broker_epoch {
            break;
        }
        if Instant::now() > deadline2 {
            panic!("new epoch never published");
        }
        std::thread::sleep(Duration::from_millis(25));
        endpoint2 = support::wait_endpoint(&run.run_dir, deadline2).expect("endpoint2");
    }
    assert_ne!(
        endpoint2.broker_epoch, endpoint.broker_epoch,
        "fresh broker epoch after restart"
    );
    loop {
        let r = support::wait_provider_ready(&run.run_dir, deadline2).expect("re-register");
        if r.broker_epoch == endpoint2.broker_epoch {
            break;
        }
        if Instant::now() > deadline2 {
            panic!("provider never re-registered on new epoch");
        }
        std::thread::sleep(Duration::from_millis(50));
    }

    // 10. old ServiceRef is stale; explicit discover against the new broker
    //     succeeds and the old row persists (receipt from before the restart)
    {
        let client = autoos_comm_local::client::CommClient::connect(
            &endpoint2.pipe_name,
            &creds.tokens["caller-a"],
            deadline2,
        )
        .await
        .expect("client after restart");
        let stale_ref = autoos_comm_core::protocol::ServiceRef {
            broker_epoch: endpoint.broker_epoch.clone(),
            instance_id: ready.instance_id.clone(),
            service_id: "auto.capture.inbox".to_string(),
            interface: "capture.inbox/1".to_string(),
        };
        let body = serde_json::json!({ "operation_id": op1, "principal": client.principal() });
        let e = client
            .call(
                &stale_ref,
                "receipt",
                autoos_comm_core::protocol::SCHEMA_CAPTURE_RECEIPT,
                2000,
                body,
            )
            .await
            .expect_err("old epoch ref must be stale");
        assert!(
            matches!(e.code, autoos_comm_core::error::ErrorCode::StaleInstance),
            "{e:?}"
        );
        let sref = client
            .discover("auto.capture.inbox")
            .await
            .expect("rediscover");
        assert_eq!(sref.broker_epoch, endpoint2.broker_epoch);
        assert_ne!(
            sref.instance_id, ready.instance_id,
            "new instance after restart"
        );
        let body = serde_json::json!({ "operation_id": op1, "principal": client.principal() });
        let reply = client
            .call(
                &sref,
                "receipt",
                autoos_comm_core::protocol::SCHEMA_CAPTURE_RECEIPT,
                2000,
                body,
            )
            .await
            .expect("receipt after restart");
        assert_eq!(reply["status"], "committed");
        assert_eq!(reply["capture_id"], capture_id.as_str());
        assert_eq!(reply["text"], text1);
    }

    // 11. managed shutdown: stop flags → clean exits
    support::request_stop(&run.run_dir, "provider");
    support::request_stop(&run.run_dir, "broker");
    assert!(
        broker2
            .wait_for(Duration::from_secs(4))
            .expect("wait broker")
            .is_some(),
        "broker exits after stop flag"
    );
    assert!(
        provider
            .wait_for(Duration::from_secs(4))
            .expect("wait provider")
            .is_some(),
        "provider exits after stop flag"
    );
}

fn parse_field(out: &str, field: &str) -> Option<String> {
    for line in out.lines() {
        if let Ok(v) = serde_json::from_str::<serde_json::Value>(line) {
            if let Some(s) = v.get(field).and_then(|f| f.as_str()) {
                return Some(s.to_string());
            }
        }
    }
    None
}
