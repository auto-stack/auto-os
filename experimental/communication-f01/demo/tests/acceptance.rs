//! Acceptance workload (V-14, AC-09): 50×1KiB captures across two callers
//! (25 operations each) plus duplicate-key traffic; latency percentiles,
//! error distribution and resource facts are reported — a baseline, never a
//! superiority claim. A repeated run against the same directory must not
//! manufacture data; the oracle is the independent read-only database, not
//! the service's own replies.

mod common;

use autoos_comm_core::protocol::{SCHEMA_CAPTURE_CAPTURE, SCHEMA_CAPTURE_RECEIPT};
use common::*;
use std::time::{Duration, Instant};

fn one_kib_text(tag: &str, i: u32) -> String {
    // deterministic exactly-1KiB payload (ASCII filler: byte length is exact)
    let mut base = format!("v14 {tag} item {i:04} - ");
    let filler = "content-payload.";
    while base.len() < 1024 {
        base.push_str(filler);
    }
    base.truncate(1024);
    assert_eq!(base.len(), 1024, "payload must be exactly 1KiB");
    base
}

#[tokio::test(flavor = "multi_thread")]
async fn v14_acceptance_workload_50_items_repeat_is_idempotent() {
    let mut run = TestRun::new("v14");
    let (_broker, _provider) = run.start_stack().await;
    let a = run.client(CALLER_A).await;
    let b = run.client(CALLER_B).await;
    let sref_a = run.discover(&a).await;
    let sref_b = run.discover(&b).await;

    // 25 unique operations per caller, 1KiB each
    let mut ops_a = Vec::new();
    let mut ops_b = Vec::new();
    for i in 0..25u32 {
        ops_a.push((autoos_comm_core::ids::new_id(), one_kib_text("caller-a", i)));
        ops_b.push((autoos_comm_core::ids::new_id(), one_kib_text("caller-b", i)));
    }

    let t0 = Instant::now();
    let mut lat_a = Vec::new();
    let mut errors = 0usize;
    for (op, text) in &ops_a {
        let started = Instant::now();
        let body = serde_json::json!({"operation_id": op, "text": text});
        match a
            .call(&sref_a, "capture", SCHEMA_CAPTURE_CAPTURE, 2000, body)
            .await
        {
            Ok(reply) => {
                assert_eq!(reply["status"], "committed");
                lat_a.push(started.elapsed().as_millis() as u64);
            }
            Err(_) => errors += 1,
        }
    }
    let mut lat_b = Vec::new();
    for (op, text) in &ops_b {
        let started = Instant::now();
        let body = serde_json::json!({"operation_id": op, "text": text});
        match b
            .call(&sref_b, "capture", SCHEMA_CAPTURE_CAPTURE, 2000, body)
            .await
        {
            Ok(reply) => {
                assert_eq!(reply["status"], "committed");
                lat_b.push(started.elapsed().as_millis() as u64);
            }
            Err(_) => errors += 1,
        }
    }
    let workload_wall = t0.elapsed();

    // duplicate-key traffic: every caller retries 5 of its own keys
    for (op, text) in ops_a.iter().take(5) {
        let id = capture_ok(&a, &sref_a, op, text).await;
        let _ = id;
    }
    for (op, text) in ops_b.iter().take(5) {
        let _ = capture_ok(&b, &sref_b, op, text).await;
    }

    // receipt spot checks: both callers read their own results
    let (op0, _) = &ops_a[0];
    let r = a
        .call(
            &sref_a,
            "receipt",
            SCHEMA_CAPTURE_RECEIPT,
            2000,
            serde_json::json!({"operation_id": op0, "principal": a.principal()}),
        )
        .await
        .expect("receipt");
    assert_eq!(r["status"], "committed");
    assert_eq!(r["text"], ops_a[0].1);

    // latency percentiles (baseline facts, not a performance claim)
    let mut all: Vec<u64> = lat_a.iter().chain(lat_b.iter()).copied().collect();
    all.sort_unstable();
    let p = |q: usize| -> u64 {
        if all.is_empty() {
            0
        } else {
            all[((all.len() - 1) * q).min(all.len() - 1)]
        }
    };
    println!(
        "v14 workload: 50 items, wall={:?}, p50={}ms p95={}ms max={}ms, caller_errors={errors}",
        workload_wall,
        p(50),
        p(95),
        all.last().copied().unwrap_or(0)
    );

    // independent read-only DB: exactly 50 rows, contents byte-exact
    let conn = rusqlite::Connection::open_with_flags(
        run.run_dir.join("inbox-db").join("capture.db"),
        rusqlite::OpenFlags::SQLITE_OPEN_READ_ONLY,
    )
    .expect("independent readonly open");
    let count: i64 = conn
        .query_row("SELECT COUNT(*) FROM captures", [], |r| r.get(0))
        .expect("count");
    assert_eq!(count, 50, "50 logical captures, dup traffic added nothing");
    let mut total_bytes: i64 = conn
        .query_row(
            "SELECT COALESCE(SUM(LENGTH(text)), 0) FROM captures",
            [],
            |r| r.get(0),
        )
        .expect("sum");
    assert_eq!(total_bytes, 50 * 1024, "every capture is 1KiB on disk");
    let _ = total_bytes;

    // REPEAT RUN: same directory, same keys+payloads — must not add data
    drop(a);
    drop(b);
    support::request_stop(&run.run_dir, "provider");
    support::request_stop(&run.run_dir, "broker");
    std::thread::sleep(Duration::from_millis(600));

    support::clear_stop(&run.run_dir, "broker");
    support::clear_stop(&run.run_dir, "provider");
    let _broker2 = run.spawn_broker();
    // poll past the stale endpoint file of the first broker
    let deadline = run.deadline();
    let old_epoch = run.endpoint.as_ref().unwrap().broker_epoch.clone();
    let mut ep2 = support::wait_endpoint(&run.run_dir, deadline).expect("endpoint2");
    while ep2.broker_epoch == old_epoch {
        assert!(Instant::now() < deadline, "new endpoint never published");
        std::thread::sleep(Duration::from_millis(25));
        ep2 = support::wait_endpoint(&run.run_dir, deadline).expect("endpoint2");
    }
    run.endpoint = Some(ep2.clone());
    let _provider2 = run.spawn_provider(None);
    loop {
        match wait_provider_ready_async(&run.run_dir, deadline).await {
            Ok(r) if r.broker_epoch == ep2.broker_epoch => break,
            _ => {}
        }
    }
    let a2 = run.client(CALLER_A).await;
    let sref2 = run.discover(&a2).await;
    for (op, text) in ops_a.iter().take(10) {
        // identical body fields — the digest binds operation_id+text here
        let body = serde_json::json!({"operation_id": op, "text": text});
        let reply = a2
            .call(&sref2, "capture", SCHEMA_CAPTURE_CAPTURE, 2000, body)
            .await
            .expect("repeat capture");
        assert_eq!(reply["status"], "committed");
    }

    let count2: i64 = conn
        .query_row("SELECT COUNT(*) FROM captures", [], |r| r.get(0))
        .expect("count2");
    assert_eq!(count2, 50, "the repeat run must not manufacture data");
    let total2: i64 = conn
        .query_row(
            "SELECT COALESCE(SUM(LENGTH(text)), 0) FROM captures",
            [],
            |r| r.get(0),
        )
        .expect("sum2");
    assert_eq!(total2, total_bytes, "byte volume unchanged after repeat");
}
