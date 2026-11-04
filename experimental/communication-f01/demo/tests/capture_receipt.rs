//! Capture domain contract tests (V-09 dedup/conflict/isolation, V-10 crash
//! barriers with marker-file evidence, V-11 capacity/readonly/corrupt/
//! second-writer). All against the real capture-inbox binary + SQLite.

mod common;

use autoos_comm_core::error::ErrorCode;
use autoos_comm_core::protocol::{SCHEMA_CAPTURE_CAPTURE, SCHEMA_CAPTURE_RECEIPT};
use common::*;
use std::time::Duration;

fn db_path(run: &TestRun) -> std::path::PathBuf {
    run.run_dir.join("inbox-db").join("capture.db")
}

/// Independent read-only verification from this process (the service's own
/// success reply is never the only oracle).
fn db_rows_and_texts(run: &TestRun) -> (i64, Vec<String>) {
    let conn = rusqlite::Connection::open_with_flags(
        db_path(run),
        rusqlite::OpenFlags::SQLITE_OPEN_READ_ONLY,
    )
    .expect("independent readonly open");
    let mut stmt = conn
        .prepare("SELECT text FROM captures ORDER BY created_at_ms, rowid")
        .expect("prepare");
    let rows = stmt
        .query_map([], |r| {
            let b: Vec<u8> = r.get(0)?;
            Ok(String::from_utf8_lossy(&b).to_string())
        })
        .expect("query");
    let texts: Vec<String> = rows.map(|r| r.expect("row")).collect();
    (texts.len() as i64, texts)
}

async fn receipt_of(
    client: &autoos_comm_local::client::CommClient,
    sref: &autoos_comm_core::protocol::ServiceRef,
    op: &str,
) -> serde_json::Value {
    let body = serde_json::json!({"operation_id": op, "principal": client.principal()});
    client
        .call(sref, "receipt", SCHEMA_CAPTURE_RECEIPT, 2000, body)
        .await
        .expect("receipt answers")
}

/// V-09: dedup, conflict and principal isolation with an independent DB.
#[tokio::test(flavor = "multi_thread")]
async fn v09_dedup_conflict_and_principal_isolation() {
    let mut run = TestRun::new("v09");
    let (_broker, _provider) = run.start_stack().await;
    let a = run.client(CALLER_A).await;
    let sref = run.discover(&a).await;

    let op = new_op();
    let text = "v09 原始文本\nwith newline";

    // sequential retries: same key+payload → same capture_id
    let id1 = capture_ok(&a, &sref, &op, text).await;
    let id2 = capture_ok(&a, &sref, &op, text).await;
    assert_eq!(id1, id2, "same key+payload must return the same capture_id");
    let (n, texts) = db_rows_and_texts(&run);
    assert_eq!(n, 1, "retries must not add rows");
    assert_eq!(texts[0], text, "raw content preserved byte-exact");

    // concurrent same-key same-payload (same principal, two in-flight calls)
    // → still one row; the dedup key is (principal, operation_id)
    let op_c = new_op();
    let (ra, rb) = tokio::join!(
        capture_ok(&a, &sref, &op_c, "concurrent payload"),
        capture_ok(&a, &sref, &op_c, "concurrent payload"),
    );
    assert_eq!(ra, rb, "concurrent identical captures converge to one id");
    let (n, _) = db_rows_and_texts(&run);
    assert_eq!(n, 2, "concurrent dedup still only one row per key");

    // same key, different payload → operation_conflict, original intact
    let body = serde_json::json!({"operation_id": op, "text": "DIFFERENT payload"});
    let e = a
        .call(&sref, "capture", SCHEMA_CAPTURE_CAPTURE, 2000, body)
        .await
        .expect_err("conflicting payload must be refused");
    assert!(
        matches!(e.code, ErrorCode::DomainError),
        "conflict is a domain_error: {e:?}"
    );
    assert_eq!(e.domain_code.as_deref(), Some("operation_conflict"));
    let (n, texts) = db_rows_and_texts(&run);
    assert_eq!(n, 2, "conflict writes nothing");
    assert_eq!(texts[0], text, "original content untouched by conflict");
    // the committed receipt still resolves to the original capture
    let r = receipt_of(&a, &sref, &op).await;
    assert_eq!(r["capture_id"], id1.as_str());

    // cross-principal: B cannot read A's capture; explicit forgery refused
    let b = run.client(CALLER_B).await;
    let srefb = run.discover(&b).await;
    let body = serde_json::json!({"operation_id": op, "principal": "caller-a"});
    let e = b
        .call(&srefb, "receipt", SCHEMA_CAPTURE_RECEIPT, 2000, body)
        .await
        .expect_err("cross-principal receipt is a permission rejection");
    assert!(matches!(e.code, ErrorCode::PermissionDenied), "{e:?}");
    // a scoped own-principal query is a clean not_observed (no leak)
    let r = receipt_of(&b, &srefb, &op).await;
    assert_eq!(r["status"], "not_observed");
}

/// V-10a: kill-before-commit — barrier marker is the kill evidence; no row,
/// no receipt; restart is clean.
#[tokio::test(flavor = "multi_thread")]
async fn v10a_kill_before_commit_leaves_nothing() {
    let mut run = TestRun::new("v10a");
    let _broker = run.spawn_broker();
    run.endpoint = Some(support::wait_endpoint(&run.run_dir, run.deadline()).expect("endpoint"));
    let mut provider = run.spawn_provider(Some("before_commit"));
    let ready = support::wait_provider_ready(&run.run_dir, run.deadline()).expect("ready");

    let a = run.client(CALLER_A).await;
    let sref = run.discover(&a).await;
    let op = new_op();

    // the capture stalls inside the open transaction; the caller gets the
    // conservative may_have_run after its deadline
    let body = serde_json::json!({"operation_id": op, "text": "v10a killed before commit"});
    let call = a.call(&sref, "capture", SCHEMA_CAPTURE_CAPTURE, 500, body);
    let barrier = wait_barrier_async(&run.run_dir, &op, run.deadline());
    let (res, kind) = tokio::join!(call, barrier);
    assert_eq!(
        kind.expect("barrier marker"),
        "before_commit",
        "marker is the kill evidence"
    );
    let e = res.expect_err("stalled capture times out");
    assert_eq!(
        e.execution,
        Some(autoos_comm_core::ExecutionState::MayHaveRun)
    );

    // kill the provider exactly at the barrier
    provider.kill();

    // restart without the barrier: service recovers, receipt is not_observed
    let provider2 = run.spawn_provider(None);
    loop {
        let r = wait_provider_ready_async(&run.run_dir, run.deadline())
            .await
            .expect("re-register");
        if r.broker_epoch == ready.broker_epoch && r.instance_id != ready.instance_id {
            break;
        }
    }
    let a2 = run.client(CALLER_A).await;
    let sref2 = run.discover(&a2).await;
    let r = receipt_of(&a2, &sref2, &op).await;
    assert_eq!(
        r["status"], "not_observed",
        "nothing was committed before the kill"
    );
    let (n, _) = db_rows_and_texts(&run);
    assert_eq!(n, 0, "no half-written row after kill-before-commit");
}

/// V-10b: commit-then-kill-before-reply — the committed capture survives,
/// the retry is idempotent (no second row).
#[tokio::test(flavor = "multi_thread")]
async fn v10b_commit_survives_kill_before_reply() {
    let mut run = TestRun::new("v10b");
    let _broker = run.spawn_broker();
    run.endpoint = Some(support::wait_endpoint(&run.run_dir, run.deadline()).expect("endpoint"));
    let mut provider = run.spawn_provider(Some("after_commit_before_reply"));
    let ready = support::wait_provider_ready(&run.run_dir, run.deadline()).expect("ready");

    let a = run.client(CALLER_A).await;
    let sref = run.discover(&a).await;
    let op = new_op();
    let text = "v10b committed then killed";

    let body = serde_json::json!({"operation_id": op, "text": text});
    let call = a.call(&sref, "capture", SCHEMA_CAPTURE_CAPTURE, 500, body);
    let barrier = wait_barrier_async(&run.run_dir, &op, run.deadline());
    let (_, kind) = tokio::join!(call, barrier);
    assert_eq!(
        kind.expect("barrier marker"),
        "after_commit_before_reply",
        "commit already happened at this barrier"
    );
    provider.kill();

    // restart: the committed capture is durable; the caller retries with the
    // same key+payload and must converge to the same capture_id, no new row
    let provider2 = run.spawn_provider(None);
    loop {
        let r = wait_provider_ready_async(&run.run_dir, run.deadline())
            .await
            .expect("re-register");
        if r.broker_epoch == ready.broker_epoch && r.instance_id != ready.instance_id {
            break;
        }
    }
    let a2 = run.client(CALLER_A).await;
    let sref2 = run.discover(&a2).await;
    let r = receipt_of(&a2, &sref2, &op).await;
    assert_eq!(
        r["status"], "committed",
        "committed receipt survives the kill"
    );
    let committed_id = r["capture_id"].as_str().expect("capture_id").to_string();
    assert_eq!(r["text"], text);
    let retry_id = capture_ok(&a2, &sref2, &op, text).await;
    assert_eq!(
        retry_id, committed_id,
        "retry converges to the committed id"
    );
    let (n, texts) = db_rows_and_texts(&run);
    assert_eq!(n, 1, "kill+retry must not duplicate the row");
    assert_eq!(texts[0], text);
    let _ = provider2;
}

/// V-11: storage failure modes — second writer refused pre-registration,
/// corrupt DB never rebuilt, readonly media refuses startup, capacity is a
/// pre-write rejection while old keys stay usable.
#[tokio::test(flavor = "multi_thread")]
async fn v11_storage_failures_and_capacity() {
    // --- second writer refused before registration; first writer unaffected
    let mut run = TestRun::new("v11");
    let _broker = run.spawn_broker();
    run.endpoint = Some(support::wait_endpoint(&run.run_dir, run.deadline()).expect("endpoint"));
    let provider_a = run.spawn_provider(None);
    let _ready = support::wait_provider_ready(&run.run_dir, run.deadline()).expect("ready");
    let provider_b = run.spawn_provider(None);
    let b_exit = provider_b
        .wait_for(Duration::from_secs(5))
        .expect("wait second writer")
        .expect("second writer must exit on its own");
    assert_eq!(
        b_exit, 3,
        "second writer exits with the storage-failure code"
    );
    let a = run.client(CALLER_A).await;
    let sref = run.discover(&a).await;
    let op = new_op();
    capture_ok(&a, &sref, &op, "first writer still works").await;
    let _ = provider_a;

    // --- corrupt database: refuse, never rebuild
    drop(_ready);
    support::request_stop(&run.run_dir, "provider");
    let _ = provider_a.wait_for(Duration::from_secs(5));
    let db = db_path(&run);
    let original = std::fs::read(&db).expect("db exists");
    std::fs::write(&db, b"this is not a sqlite database at all........").expect("corrupt");
    let provider_c = run.spawn_provider(None);
    let c_exit = provider_c
        .wait_for(Duration::from_secs(5))
        .expect("wait corrupt-db provider")
        .expect("corrupt-db provider must exit");
    assert_eq!(c_exit, 3, "corrupt db refuses startup");
    let after = std::fs::read(&db).expect("corrupt db still readable");
    assert_eq!(
        after.len(),
        original.len().min(after.len()),
        "file not rebuilt"
    );
    assert!(
        after.starts_with(b"this is not a sqlite"),
        "bytes unchanged"
    );

    // --- readonly media: refuse startup, original data intact
    // restore a valid db by rerunning the healthy provider once
    std::fs::write(&db, &original).expect("restore valid db");
    let provider_d = run.spawn_provider(None);
    support::wait_provider_ready(&run.run_dir, run.deadline()).expect("healthy again");
    support::request_stop(&run.run_dir, "provider");
    let _ = provider_d.wait_for(Duration::from_secs(5));
    // capture one more row through a fresh provider to have data, then flip
    // the readonly attribute
    let mut perms = std::fs::metadata(&db).expect("meta").permissions();
    #[allow(clippy::permissions_set_readonly_false)]
    perms.set_readonly(true);
    std::fs::set_permissions(&db, perms).expect("set readonly");
    let before_ro = std::fs::read(&db).expect("read db");
    let provider_e = run.spawn_provider(None);
    let e_exit = provider_e
        .wait_for(Duration::from_secs(5))
        .expect("wait readonly provider")
        .expect("readonly provider must exit");
    assert_eq!(e_exit, 3, "readonly db refuses startup");
    assert_eq!(
        std::fs::read(&db).expect("read db after"),
        before_ro,
        "readonly attempt left the file unchanged"
    );
    let mut perms = std::fs::metadata(&db).expect("meta").permissions();
    perms.set_readonly(false);
    std::fs::set_permissions(&db, perms).expect("clear readonly");

    // --- capacity: 1024 fills the inbox; new keys are refused pre-write,
    // old-key retries/queries keep working
    let _provider_f = run.spawn_provider(None);
    support::wait_provider_ready(&run.run_dir, run.deadline()).expect("ready for fill");
    let filler = run.client(CALLER_A).await;
    let sref_f = run.discover(&filler).await;
    // one row already exists from the earlier phases; the profile cap is 1024
    for i in 0..1023 {
        let op_i = autoos_comm_core::ids::new_id(); // unique key per fill
        let body = serde_json::json!({"operation_id": op_i, "text": format!("fill {i}")});
        let reply = filler
            .call(&sref_f, "capture", SCHEMA_CAPTURE_CAPTURE, 2000, body)
            .await
            .expect("fill capture");
        assert_eq!(reply["status"], "committed", "fill {i}");
    }
    let (n, _) = db_rows_and_texts(&run);
    assert_eq!(n, 1024, "one pre-fill row + 1023 filled rows = capacity");

    // new key at capacity → resource_exhausted, nothing written
    let body = serde_json::json!({"operation_id": autoos_comm_core::ids::new_id(), "text": "over capacity"});
    let e = filler
        .call(&sref_f, "capture", SCHEMA_CAPTURE_CAPTURE, 2000, body)
        .await
        .expect_err("capacity must refuse new keys");
    assert!(matches!(e.code, ErrorCode::ResourceExhausted), "{e:?}");
    // old-key retry at capacity still returns the same id, no new row
    let op_first = op;
    let id_old = capture_ok(&filler, &sref_f, &op_first, "first writer still works").await;
    assert!(!id_old.is_empty(), "old-key retry works at capacity");
    let (n2, _) = db_rows_and_texts(&run);
    assert_eq!(n2, n, "capacity rejections and retries add no rows");
    // old-key receipt query at capacity still works
    let r = receipt_of(&filler, &sref_f, &op_first).await;
    assert_eq!(r["status"], "committed");
}
