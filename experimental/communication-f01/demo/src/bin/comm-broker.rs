//! comm-broker — the independent broker process. Never embeds a domain
//! handler; owns session epochs, the registry, routes and resource caps.

use autoos_comm_demo::support::{self, BrokerConfigFile};
use autoos_comm_local::broker::{Broker, BrokerConfig};
use autoos_comm_local::eventlog::EventLog;
use autoos_comm_local::pipe::PipeName;
use autoos_comm_local::policy::AccessPolicy;
use std::sync::{Arc, Mutex};
use tokio::sync::watch;

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
        eprintln!("usage: comm-broker --config <path> [--log <path>]");
        return std::process::ExitCode::from(2);
    };
    let cfg: BrokerConfigFile = match support::read_json(std::path::Path::new(&config_path)) {
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
    let policy_json = match std::fs::read_to_string(&cfg.policy_file) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("cannot read policy: {e}");
            return std::process::ExitCode::from(2);
        }
    };
    let policy = match AccessPolicy::from_json(&policy_json) {
        Ok(p) => p,
        Err(e) => {
            eprintln!("bad policy: {e}");
            return std::process::ExitCode::from(2);
        }
    };
    let pipe_name = cfg.pipe_name.clone().unwrap_or_else(PipeName::new_random);
    let (shutdown_tx, shutdown_rx) = watch::channel(false);
    let broker = Broker::new(
        BrokerConfig {
            pipe_name: pipe_name.clone(),
            log: log.clone(),
        },
        policy,
        shutdown_rx,
    );
    let epoch = broker.epoch();
    support::write_json(
        &support::endpoint_file(&run_dir),
        &support::BrokerEndpoint {
            pipe_name: pipe_name.clone(),
            broker_epoch: epoch,
        },
    );
    log.event("endpoint_published", &[("pipe", pipe_name.as_str())]);

    // graceful-stop watcher (controller writes the stop flag)
    let stop_path = support::stop_file(&run_dir, "broker");
    let stop_handle = Arc::new(Mutex::new(shutdown_tx));
    std::thread::spawn(move || loop {
        if stop_path.exists() {
            if let Ok(tx) = stop_handle.lock() {
                let _ = tx.send(true);
            }
            return;
        }
        std::thread::sleep(std::time::Duration::from_millis(100));
    });

    broker.serve().await;
    std::process::ExitCode::SUCCESS
}
