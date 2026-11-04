//! Run/test support: private per-run directories, access-config generation,
//! endpoint/provider-ready files and managed process launch helpers. This
//! manages only processes this controller started.

use autoos_comm_local::job::{ChildStdio, JobGuard, ManagedChild};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::time::{Duration, Instant};

pub const ROLE_PROVIDER: &str = "provider";
pub const ROLE_CALLER: &str = "caller";

/// Access policy entry as stored in the broker config (token never logged).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccessEntry {
    pub token: String,
    pub principal: String,
    pub role: String,
    #[serde(default)]
    pub allow: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccessConfig {
    pub entries: Vec<AccessEntry>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BrokerConfigFile {
    pub run_dir: String,
    /// None → broker listens on a fresh random pipe name.
    #[serde(default)]
    pub pipe_name: Option<String>,
    pub policy_file: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BrokerEndpoint {
    pub pipe_name: String,
    pub broker_epoch: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProviderReady {
    pub instance_id: String,
    pub broker_epoch: String,
    pub pid: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProviderServiceConfig {
    pub run_dir: String,
    pub token_file: String,
    pub endpoint_file: String,
    #[serde(default)]
    pub stop_file: Option<String>,
    #[serde(default)]
    pub barrier: Option<String>,
}

/// Credential handles for one run (token values stay in private files).
pub struct RunCredentials {
    pub tokens: HashMap<String, String>, // principal → token
    pub policy_file: PathBuf,
}

fn run_root() -> PathBuf {
    std::env::temp_dir().join("autoos-comm-f01")
}

/// Create a fresh private run directory: %TEMP%/autoos-comm-f01/<runid>.
pub fn create_run_dir(tag: &str) -> PathBuf {
    let dir = run_root().join(format!(
        "{}-{}-{}",
        tag,
        std::process::id(),
        autoos_comm_core::ids::new_id()
    ));
    std::fs::create_dir_all(&dir).expect("create run dir");
    dir
}

/// Best-effort removal of a finished run directory (tests only).
pub fn remove_run_dir(dir: &Path) {
    let _ = std::fs::remove_dir_all(dir);
}

/// Generate the access config: one token per principal, role + allowed
/// services, written to the run dir. Token files: token-<principal>.txt.
pub fn write_access_config(run_dir: &Path, entries: &[(&str, &str, Vec<&str>)]) -> RunCredentials {
    let mut list = Vec::new();
    let mut tokens = HashMap::new();
    for (principal, role, allow) in entries {
        let token = autoos_comm_core::ids::new_token();
        tokens.insert(principal.to_string(), token.clone());
        list.push(AccessEntry {
            token,
            principal: principal.to_string(),
            role: role.to_string(),
            allow: allow.iter().map(|s| s.to_string()).collect(),
        });
    }
    let cfg = AccessConfig { entries: list };
    let policy_file = run_dir.join("broker-access.json");
    std::fs::write(&policy_file, serde_json::to_vec(&cfg).unwrap()).expect("write access config");
    for (principal, token) in &tokens {
        let p = run_dir.join(format!("token-{principal}.txt"));
        std::fs::write(p, token).expect("write token file");
    }
    RunCredentials {
        tokens,
        policy_file,
    }
}

pub fn token_file(run_dir: &Path, principal: &str) -> PathBuf {
    run_dir.join(format!("token-{principal}.txt"))
}

pub fn endpoint_file(run_dir: &Path) -> PathBuf {
    run_dir.join("endpoint.json")
}

pub fn provider_ready_file(run_dir: &Path) -> PathBuf {
    run_dir.join("provider-ready.json")
}

pub fn stop_file(run_dir: &Path, role: &str) -> PathBuf {
    run_dir.join(format!("stop-{role}.flag"))
}

pub fn barrier_dir(run_dir: &Path) -> PathBuf {
    run_dir.join("barriers")
}

pub fn logs_dir(run_dir: &Path) -> PathBuf {
    let d = run_dir.join("logs");
    std::fs::create_dir_all(&d).ok();
    d
}

pub fn write_json<T: Serialize>(path: &Path, v: &T) {
    std::fs::write(path, serde_json::to_vec(v).unwrap()).expect("write json");
}

pub fn read_json<T: for<'de> Deserialize<'de>>(path: &Path) -> std::io::Result<T> {
    let bytes = std::fs::read(path)?;
    serde_json::from_slice(&bytes)
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))
}

/// Poll-read the broker endpoint file until present.
pub fn wait_endpoint(run_dir: &Path, deadline: Instant) -> std::io::Result<BrokerEndpoint> {
    let p = endpoint_file(run_dir);
    loop {
        if Instant::now() >= deadline {
            return Err(std::io::Error::new(
                std::io::ErrorKind::TimedOut,
                "endpoint wait",
            ));
        }
        if let Ok(v) = read_json::<BrokerEndpoint>(&p) {
            return Ok(v);
        }
        std::thread::sleep(Duration::from_millis(25));
    }
}

/// Poll for the provider-ready marker (service registered).
pub fn wait_provider_ready(run_dir: &Path, deadline: Instant) -> std::io::Result<ProviderReady> {
    let p = provider_ready_file(run_dir);
    loop {
        if Instant::now() >= deadline {
            return Err(std::io::Error::new(
                std::io::ErrorKind::TimedOut,
                "provider-ready wait",
            ));
        }
        if let Ok(v) = read_json::<ProviderReady>(&p) {
            return Ok(v);
        }
        std::thread::sleep(Duration::from_millis(25));
    }
}

/// Poll for a barrier marker; returns its kind (barrier evidence).
pub fn wait_barrier(
    run_dir: &Path,
    operation_id: &str,
    deadline: Instant,
) -> std::io::Result<String> {
    let dir = barrier_dir(run_dir);
    let p = dir.join(format!("barrier-{operation_id}.json"));
    loop {
        if Instant::now() >= deadline {
            return Err(std::io::Error::new(
                std::io::ErrorKind::TimedOut,
                "barrier wait",
            ));
        }
        if let Ok(bytes) = std::fs::read(&p) {
            #[derive(Deserialize)]
            struct Marker {
                barrier: String,
            }
            if let Ok(m) = serde_json::from_slice::<Marker>(&bytes) {
                return Ok(m.barrier);
            }
        }
        std::thread::sleep(Duration::from_millis(25));
    }
}

/// Request graceful exit of a role (broker/provider poll the stop file).
pub fn request_stop(run_dir: &Path, role: &str) {
    let _ = std::fs::write(stop_file(run_dir, role), b"stop");
}

pub fn clear_stop(run_dir: &Path, role: &str) {
    let _ = std::fs::remove_file(stop_file(run_dir, role));
}

/// Spawn a managed child (suspended → job → resume) with stderr redirected
/// into the run's log dir and stdout captured through an inheritable pipe.
/// Returns the child plus the pipe's read end (already at EOF-safe state).
pub fn spawn_managed(
    job: &JobGuard,
    exe: &Path,
    args: &[String],
    envs: &[(String, String)],
    cwd: &Path,
    stderr_log: &Path,
) -> std::io::Result<(ManagedChild, Option<std::fs::File>)> {
    let log_f = std::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(stderr_log)?;
    use std::os::windows::io::AsRawHandle;
    unsafe {
        use windows_sys::Win32::Foundation::{SetHandleInformation, HANDLE_FLAG_INHERIT};
        if SetHandleInformation(
            log_f.as_raw_handle() as _,
            HANDLE_FLAG_INHERIT,
            HANDLE_FLAG_INHERIT,
        ) == 0
        {
            return Err(std::io::Error::last_os_error());
        }
    }
    let inherit_stderr = log_f.as_raw_handle() as _;
    let (stdout_read, stdout_write) = autoos_comm_local::job::child_stdout_pipe()?;
    let args_ref: Vec<&str> = args.iter().map(|s| s.as_str()).collect();
    let spawned = job.spawn(
        exe,
        &args_ref,
        envs,
        Some(cwd),
        ChildStdio {
            stdin: None,
            stdout: Some(stdout_write),
            stderr: Some(inherit_stderr),
        },
    );
    // close our copy of the write end so EOF reaches the reader once the
    // child (and its job siblings) exit
    unsafe {
        windows_sys::Win32::Foundation::CloseHandle(stdout_write);
    }
    let child = spawned?;
    Ok((child, Some(stdout_read)))
}
