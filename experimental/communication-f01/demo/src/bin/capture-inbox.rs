//! capture-inbox — the first service: an independent capture provider with a
//! dedicated private SQLite database. It owns the database and the domain
//! dedup key; the broker never sees a receipt.

use autoos_comm_core::protocol::{INTERFACE_CAPTURE, SCHEMA_CAPTURE_CAPTURE, SERVICE_ID_CAPTURE};
use autoos_comm_demo::capture::{handle_call, BarrierPosition, CaptureService, DESCRIPTOR};
use autoos_comm_demo::store::Store;
use autoos_comm_demo::support::{self, ProviderServiceConfig};
use autoos_comm_local::eventlog::EventLog;
use autoos_comm_local::provider::ProviderConn;

#[tokio::main(flavor = "multi_thread")]
async fn main() -> std::process::ExitCode {
    let args: Vec<String> = std::env::args().collect();
    let mut config_path: Option<String> = None;
    let mut log_path: Option<String> = None;
    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "--config" => config_path = args.get(i + 1).cloned(),
            "--log" => log_path = args.get(i + 1).cloned(),
            other => {
                eprintln!("unknown argument `{other}`");
                return std::process::ExitCode::from(2);
            }
        }
        i += 2;
    }
    let (Some(config_path), log_path) = (config_path, log_path) else {
        eprintln!("usage: capture-inbox --config <path> [--log <path>]");
        return std::process::ExitCode::from(2);
    };
    let cfg: ProviderServiceConfig = match support::read_json(std::path::Path::new(&config_path)) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("bad config: {e}");
            return std::process::ExitCode::from(2);
        }
    };
    let run_dir = std::path::PathBuf::from(&cfg.run_dir);
    let log = match &log_path {
        Some(p) => {
            EventLog::to_file(std::path::Path::new(p)).unwrap_or_else(|_| EventLog::stderr())
        }
        None => EventLog::stderr(),
    };
    let token = match std::fs::read_to_string(&cfg.token_file) {
        Ok(t) => t.trim().to_string(),
        Err(e) => {
            eprintln!("cannot read token: {e}");
            return std::process::ExitCode::from(2);
        }
    };
    let barrier = BarrierPosition::from_config(cfg.barrier.as_deref());
    let stop_path = cfg
        .stop_file
        .clone()
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|| support::stop_file(&run_dir, "provider"));

    // dedicated database must be owned before registration: a second writer
    // fails here, before the broker ever hears about it
    let db_dir = run_dir.join("inbox-db");
    let store = match Store::open(&db_dir) {
        Ok(s) => s,
        Err(e) => {
            log.event("store_open_failed", &[("error", format!("{e:?}").as_str())]);
            return std::process::ExitCode::from(3);
        }
    };
    log.event(
        "store_open",
        &[("db", store.db_path.to_string_lossy().as_ref())],
    );
    let service = std::sync::Arc::new(CaptureService { store, barrier });
    let bdir = std::sync::Arc::new(support::barrier_dir(&run_dir));
    std::fs::create_dir_all(bdir.as_ref()).ok();

    // reconnect loop: provider re-registers with a new instance after any
    // broker connection loss (V-06)
    loop {
        if stop_path.exists() {
            log.event("provider_stop", &[]);
            let _ = std::fs::remove_file(support::provider_ready_file(&run_dir));
            return std::process::ExitCode::SUCCESS;
        }
        let endpoint = match support::wait_endpoint(&run_dir, instant_in(2)) {
            Ok(e) => e,
            Err(_) => continue,
        };
        match ProviderConn::connect_register(
            &endpoint.pipe_name,
            &token,
            SERVICE_ID_CAPTURE,
            INTERFACE_CAPTURE,
            SCHEMA_CAPTURE_CAPTURE,
            DESCRIPTOR,
            std::time::Instant::now() + std::time::Duration::from_secs(2),
        )
        .await
        {
            Ok(conn) => {
                support::write_json(
                    &support::provider_ready_file(&run_dir),
                    &support::ProviderReady {
                        instance_id: conn.service_ref().instance_id.clone(),
                        broker_epoch: conn.service_ref().broker_epoch.clone(),
                        pid: std::process::id(),
                    },
                );
                log.event(
                    "registered",
                    &[
                        ("service", SERVICE_ID_CAPTURE),
                        ("instance", conn.service_ref().instance_id.as_str()),
                    ],
                );
                let handler = {
                    let service = service.clone();
                    let bdir = bdir.clone();
                    move |call: autoos_comm_core::protocol::CallToProvider| {
                        let service = service.clone();
                        let bdir = bdir.clone();
                        async move { handle_call(&service, &call, (*bdir).clone()).await }
                    }
                };
                let _ = conn.serve(handler).await;
                log.event("broker_connection_lost", &[]);
                let _ = std::fs::remove_file(support::provider_ready_file(&run_dir));
                // reconnect after a short beat
                tokio::time::sleep(std::time::Duration::from_millis(200)).await;
            }
            Err(e) => {
                log.event(
                    "register_failed",
                    &[("code", format!("{:?}", e.code).as_str())],
                );
                // the registration may be permanently refused (second writer,
                // permission); retry a few times, then exit non-zero
                static FAILS: std::sync::atomic::AtomicU32 = std::sync::atomic::AtomicU32::new(0);
                let n = FAILS.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                if n >= 60 {
                    return std::process::ExitCode::from(4);
                }
                tokio::time::sleep(std::time::Duration::from_millis(300)).await;
            }
        }
    }
}

fn instant_in(secs: u64) -> std::time::Instant {
    std::time::Instant::now() + std::time::Duration::from_secs(secs)
}
