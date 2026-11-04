//! Resource-boundary tests (V-05 pipe security bits, V-12 in-flight and
//! provider-queue quotas). The V-05 DACL equality oracle lives in the
//! lifecycle t01 gate; here the first-instance and reject-remote bits are
//! checked on real handles, and the quota rejections are proven
//! pre-dispatch.

mod common;

use autoos_comm_core::error::ErrorCode;
use autoos_comm_core::protocol::{SCHEMA_CAPTURE_CAPTURE, SERVICE_ID_CAPTURE};
use common::*;

/// V-05: real-handle transport security — explicit DACL membership,
/// reject-remote state bit, first-instance protection.
#[tokio::test(flavor = "multi_thread")]
async fn v05_pipe_security_bits_on_real_handles() {
    use std::os::windows::io::AsRawHandle;
    use std::time::Duration;

    let name = autoos_comm_local::pipe::PipeName::new_random();
    let server = autoos_comm_local::pipe::create_pipe_server(&name, true).expect("first instance");

    // a second first-instance bind on the same name must fail (V-05)
    let second = autoos_comm_local::pipe::create_pipe_server(&name, true);
    let err = second.expect_err("second first-instance bind must fail");
    assert_eq!(
        err.raw_os_error(),
        Some(5), // ERROR_ACCESS_DENIED
        "first-instance failure surfaces as access denied: {err}"
    );
    // additional non-first instances are allowed (broker accept pattern)
    let extra = autoos_comm_local::pipe::create_pipe_server(&name, false);
    assert!(extra.is_ok(), "non-first instance binds fine");

    // connect a client; both ends observe the real object state
    let client = autoos_comm_local::pipe::connect_pipe(
        &name,
        std::time::Instant::now() + Duration::from_secs(3),
    )
    .await
    .expect("client connect");

    // DACL: exactly the current user + SYSTEM, nobody else
    let sids = autoos_comm_local::pipe::pipe_dacl_sids(client.as_raw_handle()).expect("dacl");
    let me = autoos_comm_local::pipe::current_user_sid_string().unwrap();
    assert_eq!(sids.len(), 2, "no Everyone/Anonymous ACEs: {sids:?}");
    assert!(
        sids.contains(&me) && sids.contains(&"S-1-5-18".to_string()),
        "{sids:?}"
    );

    // reject-remote is set on the actual server-side handle state
    let rejects_server = autoos_comm_local::pipe::pipe_rejects_remote(server.as_raw_handle())
        .expect("server pipe state");
    assert!(
        rejects_server,
        "PIPE_REJECT_REMOTE_CLIENTS must be active on the listening end"
    );

    drop(client);
    drop(server);
    drop(extra);
}

/// V-12: per-connection in-flight cap (8) and provider queue cap (16) are
/// rejected pre-dispatch with not_dispatched; a slow-but-reading provider
/// plus abandonment isolates the quota exactly.
#[tokio::test(flavor = "multi_thread")]
async fn v12_inflight_and_provider_queue_quota() {
    let mut run = TestRun::new("v12quota");
    let _broker = run.spawn_broker();
    run.endpoint = Some(support::wait_endpoint(&run.run_dir, run.deadline()).expect("endpoint"));
    // silent provider: registers, never replies (holds all routes open)
    let _provider = RawProvider::connect(&run, "desc-v12quota").await;

    let a = run.client(CALLER_A).await;
    let sref = run.discover(&a).await;

    // abandon 8 calls: fully written, routes stay open
    for i in 0..8u32 {
        let op = autoos_comm_core::ids::new_id();
        let rid = format!("{}eeeeeeeeeeeeeeeeeeeeeeee{:02}", &op[..8], i);
        a.call_abandon(
            &rid,
            &sref,
            "capture",
            SCHEMA_CAPTURE_CAPTURE,
            5000,
            serde_json::json!({"operation_id": op, "text": "quota fill"}),
        )
        .await
        .expect("abandoned call written");
    }

    // the 9th on the same connection is refused pre-dispatch
    let op = autoos_comm_core::ids::new_id();
    let body = serde_json::json!({"operation_id": op, "text": "over per-conn quota"});
    let e = a
        .call(&sref, "capture", SCHEMA_CAPTURE_CAPTURE, 1000, body)
        .await
        .expect_err("per-conn in-flight cap must reject");
    assert!(matches!(e.code, ErrorCode::ResourceExhausted), "{e:?}");
    assert_eq!(
        e.execution,
        Some(autoos_comm_core::error::ExecutionState::NotDispatched),
        "quota rejection is provably not dispatched"
    );

    // second caller fills the provider queue to 16 (8+8)
    let b = run.client(CALLER_B).await;
    let srefb = run.discover(&b).await;
    for i in 0..8u32 {
        let op = autoos_comm_core::ids::new_id();
        let rid = format!("{}ffffffffffffffffffffffff{:02}", &op[..8], i);
        b.call_abandon(
            &rid,
            &srefb,
            "capture",
            SCHEMA_CAPTURE_CAPTURE,
            5000,
            serde_json::json!({"operation_id": op, "text": "queue fill"}),
        )
        .await
        .expect("second caller abandoned call");
    }

    // a third caller now hits the provider-queue cap (16)
    // (the denied caller cannot call; use a fresh caller-a connection)
    let c = run.client(CALLER_A).await;
    let srefc = run.discover(&c).await;
    let op = autoos_comm_core::ids::new_id();
    let body = serde_json::json!({"operation_id": op, "text": "queue full"});
    let e = c
        .call(&srefc, "capture", SCHEMA_CAPTURE_CAPTURE, 1000, body)
        .await
        .expect_err("provider queue cap must reject");
    assert!(matches!(e.code, ErrorCode::ResourceExhausted), "{e:?}");
    assert_eq!(
        e.execution,
        Some(autoos_comm_core::error::ExecutionState::NotDispatched)
    );

    // a service that nobody registered still yields not_found (not quota)
    let e = c
        .discover("auto.capture.nonexistent")
        .await
        .expect_err("unknown service");
    assert!(matches!(
        e.code,
        ErrorCode::ServiceNotFound | ErrorCode::PermissionDenied
    ));
    let _ = SERVICE_ID_CAPTURE;
}
