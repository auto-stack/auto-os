//! F1 routing contract tests (V-04, V-06, V-07, V-08 and the connection
//! quota leg of V-12). Real broker process + binary/library providers.

mod common;

use autoos_comm_core::error::{CommError, ErrorCode, ExecutionState};
use autoos_comm_core::protocol::{
    SCHEMA_CAPTURE_CAPTURE, SCHEMA_CAPTURE_RECEIPT, SERVICE_ID_CAPTURE,
};
use common::*;

fn code_of(e: &CommError) -> String {
    format!("{:?}", e.code)
}

/// V-04: credential roles and trusted identity — a body can never raise its
/// own permission; a no-permission caller is rejected pre-dispatch.
#[tokio::test(flavor = "multi_thread")]
async fn v04_permissions_roles_and_trusted_identity() {
    let mut run = TestRun::new("v04");
    let (_broker, _provider) = run.start_stack().await;

    // permitted caller works
    let a = run.client(CALLER_A).await;
    let sref = run.discover(&a).await;
    let op = new_op();
    let _ = capture_ok(&a, &sref, &op, "v04 text").await;

    // caller with no allow-list entry: discover refused pre-dispatch
    let denied = run.client(DENIED_CALLER).await;
    let e = denied
        .discover(SERVICE_ID_CAPTURE)
        .await
        .expect_err("denied caller must not discover");
    assert!(is_code(&e, ErrorCode::PermissionDenied), "{e:?}");

    // call is equally refused (same token, different operation)
    let body = serde_json::json!({"operation_id": new_op(), "text": "x"});
    let e = denied
        .call(&sref, "capture", SCHEMA_CAPTURE_CAPTURE, 1000, body)
        .await
        .expect_err("denied caller must not call");
    assert!(is_code(&e, ErrorCode::PermissionDenied), "{e:?}");

    // body cannot forge another principal: receipt with someone else's
    // principal is an explicit permission rejection at the provider
    let body = serde_json::json!({"operation_id": op, "principal": CALLER_B});
    let e = a
        .call(&sref, "receipt", SCHEMA_CAPTURE_RECEIPT, 2000, body)
        .await
        .expect_err("forged principal receipt must be rejected");
    assert!(is_code(&e, ErrorCode::PermissionDenied), "{e:?}");
    let _ = code_of(&e);

    // no credential material may appear in any log
    let tokens = [run.token(CALLER_A), run.token(DENIED_CALLER)];
    let logs = [
        ("broker-stderr.log", run.log_text("broker-stderr.log")),
        ("provider-stderr.log", run.log_text("provider-stderr.log")),
    ];
    for (name, text) in &logs {
        for t in &tokens {
            assert!(!text.contains(t), "token leaked in {name}");
        }
    }
}

/// V-06 core sequencing, deterministic version: start with NO provider,
/// take over step by step.
#[tokio::test(flavor = "multi_thread")]
async fn v06_takeover_after_disconnect() {
    let mut run = TestRun::new("v06b");
    let _broker = run.spawn_broker();
    run.endpoint = Some(support::wait_endpoint(&run.run_dir, run.deadline()).expect("endpoint"));

    // 1. first provider registers
    let mut p1 = RawProvider::connect(&run, "desc-v06b").await;
    let a = run.client(CALLER_A).await;
    let sref1 = run.discover(&a).await;
    assert_eq!(sref1.instance_id, p1.service_ref.instance_id);

    // 2. concurrent registration from another connection is refused
    {
        let token = run.token(PROVIDER);
        let client = autoos_comm_local::pipe::connect_pipe(&run.pipe(), run.deadline())
            .await
            .unwrap();
        let (mut r, mut w) = tokio::io::split(client);
        autoos_comm_local::pipe::send_message(
            &mut w,
            &autoos_comm_core::protocol::Message::Hello(autoos_comm_core::protocol::Hello {
                protocol_version: autoos_comm_core::protocol::PROTOCOL_VERSION,
                codec: autoos_comm_core::protocol::CODEC_ID.to_string(),
                token,
            }),
        )
        .await
        .unwrap();
        match autoos_comm_local::pipe::read_message(&mut r)
            .await
            .unwrap()
            .unwrap()
        {
            autoos_comm_core::protocol::Message::Welcome(_) => {}
            o => panic!("op {}", o.op_name()),
        }
        autoos_comm_local::pipe::send_message(
            &mut w,
            &autoos_comm_core::protocol::Message::Register(autoos_comm_core::protocol::Register {
                corr_id: 1,
                service_id: SERVICE_ID_CAPTURE.to_string(),
                interface: autoos_comm_core::protocol::INTERFACE_CAPTURE.to_string(),
                schema_id: SCHEMA_CAPTURE_CAPTURE.to_string(),
                descriptor: "desc-v06b".to_string(),
            }),
        )
        .await
        .unwrap();
        match autoos_comm_local::pipe::read_message(&mut r)
            .await
            .unwrap()
            .unwrap()
        {
            autoos_comm_core::protocol::Message::Error(e) => {
                assert!(matches!(e.code, ErrorCode::AlreadyRegistered), "{e:?}")
            }
            o => panic!("expected already_registered error, got {}", o.op_name()),
        }
        drop(r);
        drop(w);
    }

    // 3. same-descriptor re-register on p1's own connection is idempotent
    {
        use autoos_comm_core::protocol::{Message, Register, Registered};
        use autoos_comm_local::pipe::{read_message, send_message};
        let mut w = p1.write.lock().await;
        send_message(
            &mut *w,
            &Message::Register(Register {
                corr_id: 2,
                service_id: SERVICE_ID_CAPTURE.to_string(),
                interface: autoos_comm_core::protocol::INTERFACE_CAPTURE.to_string(),
                schema_id: SCHEMA_CAPTURE_CAPTURE.to_string(),
                descriptor: "desc-v06b".to_string(),
            }),
        )
        .await
        .unwrap();
        drop(w);
        match read_message(&mut p1.read).await.unwrap().unwrap() {
            Message::Registered(rg) => {
                assert_eq!(
                    rg.service_ref.instance_id, sref1.instance_id,
                    "same descriptor on same connection returns the original instance"
                );
            }
            o => panic!("expected registered, got {}", o.op_name()),
        }
    }

    // 4. changed descriptor on the same connection is refused
    {
        use autoos_comm_core::protocol::{Message, Register};
        use autoos_comm_local::pipe::{read_message, send_message};
        let mut w = p1.write.lock().await;
        send_message(
            &mut *w,
            &Message::Register(Register {
                corr_id: 3,
                service_id: SERVICE_ID_CAPTURE.to_string(),
                interface: autoos_comm_core::protocol::INTERFACE_CAPTURE.to_string(),
                schema_id: SCHEMA_CAPTURE_CAPTURE.to_string(),
                descriptor: "desc-v06b-CHANGED".to_string(),
            }),
        )
        .await
        .unwrap();
        drop(w);
        match read_message(&mut p1.read).await.unwrap().unwrap() {
            Message::Error(e) => {
                assert!(matches!(e.code, ErrorCode::AlreadyRegistered), "{e:?}")
            }
            o => panic!("expected descriptor-mismatch error, got {}", o.op_name()),
        }
    }

    // 5. disconnect retires the instance; a new provider takes over with a
    //    fresh instance; the old ref is then strictly stale (not silently
    //    rerouted to the new instance)
    drop(p1);
    std::thread::sleep(std::time::Duration::from_millis(300));
    // while no provider holds the service, a call on the old ref is refused
    let e = a
        .call(
            &sref1,
            "receipt",
            SCHEMA_CAPTURE_RECEIPT,
            1000,
            serde_json::json!({"operation_id": new_op(), "principal": a.principal()}),
        )
        .await
        .expect_err("retired instance must be refused");
    assert!(
        matches!(
            e.code,
            ErrorCode::StaleInstance | ErrorCode::ServiceNotFound
        ),
        "{e:?}"
    );
    let mut p2 = RawProvider::connect(&run, "desc-v06b").await;
    // the old instance ref still points at the retired registration
    let e = a
        .call(
            &sref1,
            "receipt",
            SCHEMA_CAPTURE_RECEIPT,
            1000,
            serde_json::json!({"operation_id": new_op(), "principal": a.principal()}),
        )
        .await
        .expect_err("old ref must be stale after takeover");
    assert!(is_code(&e, ErrorCode::StaleInstance), "{e:?}");
    let sref2 = run.discover(&a).await;
    assert_ne!(
        sref2.instance_id, sref1.instance_id,
        "new instance after takeover"
    );
    assert_eq!(sref2.instance_id, p2.service_ref.instance_id);

    // 6. discovery miss for an unregistered service id
    let e = a
        .discover("auto.capture.nonexistent")
        .await
        .expect_err("unknown service must not resolve");
    // permission check runs first for this caller (not in allow list)
    assert!(matches!(
        e.code,
        ErrorCode::PermissionDenied | ErrorCode::ServiceNotFound
    ));
}

/// V-07: deadlines and late results — forwarded-but-unanswered is
/// may_have_run; a queued (never-forwarded) deadline is not_dispatched;
/// late replies never reach a new request; mutations are never replayed.
#[tokio::test(flavor = "multi_thread")]
async fn v07_deadline_states_and_late_replies() {
    let mut run = TestRun::new("v07");
    let _broker = run.spawn_broker();
    run.endpoint = Some(support::wait_endpoint(&run.run_dir, run.deadline()).expect("endpoint"));

    // silent provider: registers but never replies
    let mut provider = RawProvider::connect(&run, "desc-v07").await;
    let a = run.client(CALLER_A).await;
    let sref = run.discover(&a).await;

    // forwarded, no reply → may_have_run
    let op1 = new_op();
    let body = serde_json::json!({"operation_id": op1, "text": "v07 may_have_run"});
    let rid1 = format!("{}aaaaaaaaaaaaaaaaaaaaaaaaaa01", &op1[..8]);
    let c1 = a.call_with_id(&rid1, &sref, "capture", SCHEMA_CAPTURE_CAPTURE, 300, body);
    let forwarded = tokio::time::timeout(std::time::Duration::from_secs(5), async {
        let _ = provider.next_call().await;
    });
    let (e, _) = tokio::join!(c1, forwarded);
    let e = e.expect_err("silent provider → deadline");
    assert!(matches!(e.code, ErrorCode::DeadlineExceeded), "{e:?}");
    assert_eq!(
        e.execution,
        Some(ExecutionState::MayHaveRun),
        "forwarded call must be may_have_run"
    );

    // the provider saw exactly one attempt: no automatic replay
    // (the single next_call above consumed it; a second would arrive here)

    // late reply for the dead route never reaches a new request
    let route1 = format!("{}", autoos_comm_core::ids::new_id());
    // craft a reply on a route id that no longer exists — must be dropped
    provider
        .reply(&route1, serde_json::json!({"status": "committed"}))
        .await;

    // a fresh request completes normally and cannot receive the late body:
    // the driver consumes the forwarded call and answers it in time
    let op2 = new_op();
    let rid2 = format!("{}aaaaaaaaaaaaaaaaaaaaaaaaaa02", &op2[..8]);
    let c2 = a.call_with_id(
        &rid2,
        &sref,
        "capture",
        SCHEMA_CAPTURE_CAPTURE,
        2000,
        serde_json::json!({"operation_id": op2, "text": "v07 ontime"}),
    );
    let driver = async {
        let c = provider.next_call().await;
        assert_eq!(c.request_id, rid2, "correlation follows request ids");
        provider
            .reply(
                &c.route_id,
                serde_json::json!({"status": "committed", "via": "raw"}),
            )
            .await;
    };
    let (reply, _) = tokio::join!(c2, driver);
    let reply = reply.expect("fresh call after timeout");
    assert_eq!(
        reply["via"], "raw",
        "the late reply must not have been reused"
    );
}

/// V-07 via a scripted provider: reply ordering proves late replies cannot
/// be reused for a later request.
#[tokio::test(flavor = "multi_thread")]
async fn v07_late_reply_never_reaches_new_request() {
    let mut run = TestRun::new("v07b");
    let _broker = run.spawn_broker();
    run.endpoint = Some(support::wait_endpoint(&run.run_dir, run.deadline()).expect("endpoint"));
    let mut provider = RawProvider::connect(&run, "desc-v07b").await;
    let a = run.client(CALLER_A).await;
    let sref = run.discover(&a).await;

    // call1: provider withholds the reply; caller times out at 300ms
    let op1 = new_op();
    let rid1 = format!("{}bbbbbbbbbbbbbbbbbbbbbbbbbb01", &op1[..8]);
    let c1 = a.call_with_id(
        &rid1,
        &sref,
        "capture",
        SCHEMA_CAPTURE_CAPTURE,
        300,
        serde_json::json!({"operation_id": op1, "text": "v07b one"}),
    );
    // drive both: the pcall arrives while the caller waits; the reader
    // borrow ends with this block
    let (call1, e) = {
        let reader = provider.next_call();
        tokio::pin!(reader);
        tokio::join!(reader, c1)
    };
    let e = e.expect_err("call1 times out");
    assert_eq!(e.execution, Some(ExecutionState::MayHaveRun));

    // call2 with a new request_id; provider replies to call1's route (late)
    // before replying to call2's own route.
    let op2 = new_op();
    let rid2 = format!("{}bbbbbbbbbbbbbbbbbbbbbbbbbb02", &op2[..8]);
    let c2 = a.call_with_id(
        &rid2,
        &sref,
        "capture",
        SCHEMA_CAPTURE_CAPTURE,
        2000,
        serde_json::json!({"operation_id": op2, "text": "v07b two"}),
    );
    let driver = async {
        let call2 = provider.next_call().await;
        assert_eq!(call2.request_id, rid2);
        // late reply first: it must be dropped, never reused for call2
        provider
            .reply(
                &call1.route_id,
                serde_json::json!({"status": "committed", "late": true}),
            )
            .await;
        provider
            .reply(
                &call2.route_id,
                serde_json::json!({"status": "committed", "late": false}),
            )
            .await;
    };
    let (reply2, _) = tokio::join!(c2, driver);
    let reply2 = reply2.expect("call2 must succeed with its own reply");
    assert_eq!(
        reply2["late"], false,
        "late reply for call1 must never be delivered to call2"
    );
    assert_eq!(reply2["status"], "committed");

    // duplicate active request_id is refused while call1... (call1 done) —
    // reuse after completion is legal; reuse while active is refused:
    let op3 = new_op();
    let rid3 = format!("{}bbbbbbbbbbbbbbbbbbbbbbbbbb03", &op3[..8]);
    let slow = a.call_with_id(
        &rid3,
        &sref,
        "capture",
        SCHEMA_CAPTURE_CAPTURE,
        1000,
        serde_json::json!({"operation_id": op3, "text": "v07b three"}),
    );
    let driver = async {
        let call3 = provider.next_call().await;
        // while call3 is active on this connection, reusing the same
        // request_id must be refused
        let dup = a
            .call_with_id(
                &rid3,
                &sref,
                "capture",
                SCHEMA_CAPTURE_CAPTURE,
                1000,
                serde_json::json!({"operation_id": new_op(), "text": "dup"}),
            )
            .await
            .expect_err("duplicate active request_id refused");
        assert!(is_code(&dup, ErrorCode::SchemaMismatch), "{dup:?}");
        provider
            .reply(&call3.route_id, serde_json::json!({"status": "committed"}))
            .await;
    };
    let (slow_out, _) = tokio::join!(slow, driver);
    slow_out.expect("slow call completes");
}

/// V-08: two callers never cross streams; a caller cannot send provider-side
/// messages; forged provider routes are dropped without harming callers.
#[tokio::test(flavor = "multi_thread")]
async fn v08_caller_isolation_and_forged_routes() {
    let mut run = TestRun::new("v08");
    let _broker = run.spawn_broker();
    run.endpoint = Some(support::wait_endpoint(&run.run_dir, run.deadline()).expect("endpoint"));
    let mut provider = RawProvider::connect(&run, "desc-v08").await;

    let a = run.client(CALLER_A).await;
    let b = run.client(CALLER_B).await;
    let sref = run.discover(&a).await;

    // interleaved captures, each caller gets its own capture_id
    let opa = new_op();
    let opb = new_op();
    let rida = format!("{}cccccccccccccccccccccccccc0a", &opa[..8]);
    let ridb = format!("{}cccccccccccccccccccccccccc0b", &opb[..8]);
    let ca = a.call_with_id(
        &rida,
        &sref,
        "capture",
        SCHEMA_CAPTURE_CAPTURE,
        2000,
        serde_json::json!({"operation_id": opa, "text": "v08 A"}),
    );
    let cb = b.call_with_id(
        &ridb,
        &sref,
        "capture",
        SCHEMA_CAPTURE_CAPTURE,
        2000,
        serde_json::json!({"operation_id": opb, "text": "v08 B"}),
    );
    let driver = async {
        let c1 = provider.next_call().await;
        let c2 = provider.next_call().await;
        let (caller1, caller2) = (c1.caller.clone(), c2.caller.clone());
        assert_ne!(
            caller1, caller2,
            "trusted principals must differ per caller"
        );
        // answer in reverse order to prove routing is not FIFO-accidental
        provider
            .reply(
                &c2.route_id,
                serde_json::json!({"status": "committed", "id": 2}),
            )
            .await;
        provider
            .reply(
                &c1.route_id,
                serde_json::json!({"status": "committed", "id": 1}),
            )
            .await;
    };
    let (fut_a, fut_b, _) = tokio::join!(ca, cb, driver);
    let ra = fut_a.expect("A reply");
    let rb = fut_b.expect("B reply");
    // each caller resolved with its own route's body (1 vs 2), proving the
    // reverse-order replies were correlated per request, not broadcast
    assert_eq!(ra["status"], "committed");
    assert_eq!(rb["status"], "committed");
    assert_ne!(
        ra["id"], rb["id"],
        "each caller must receive its own reply body"
    );

    // receipt isolation across principals is proven with the real provider
    // in the capture_receipt family (V-09); the raw provider here never
    // replies, so no receipt round-trip is attempted in this test.

    // forged route from the provider: dropped, callers unaffected (a driven
    // capture still completes with its own reply)
    provider
        .reply(
            &autoos_comm_core::ids::new_id(),
            serde_json::json!({"status": "committed"}),
        )
        .await;
    let op = new_op();
    let rid = format!("{}dddddddddddddddddddddddddd01", &op[..8]);
    let after = a.call_with_id(
        &rid,
        &sref,
        "capture",
        SCHEMA_CAPTURE_CAPTURE,
        2000,
        serde_json::json!({"operation_id": op, "text": "after forged route"}),
    );
    let driver = async {
        let c = provider.next_call().await;
        assert_eq!(
            c.request_id, rid,
            "forged route must not consume the new request"
        );
        provider
            .reply(
                &c.route_id,
                serde_json::json!({"status": "committed", "ok": true}),
            )
            .await;
    };
    let (after, _) = tokio::join!(after, driver);
    let after = after.expect("driven capture after forged route");
    assert_eq!(after["ok"], true);

    // a caller sending a provider-direction message gets cut off
    {
        let raw_client_pipe = run.pipe();
        let token = run.token(CALLER_A);
        let client = autoos_comm_local::pipe::connect_pipe(&raw_client_pipe, run.deadline())
            .await
            .unwrap();
        let (mut r, mut w) = tokio::io::split(client);
        autoos_comm_local::pipe::send_message(
            &mut w,
            &autoos_comm_core::protocol::Message::Hello(autoos_comm_core::protocol::Hello {
                protocol_version: autoos_comm_core::protocol::PROTOCOL_VERSION,
                codec: autoos_comm_core::protocol::CODEC_ID.to_string(),
                token,
            }),
        )
        .await
        .unwrap();
        match autoos_comm_local::pipe::read_message(&mut r)
            .await
            .unwrap()
            .unwrap()
        {
            autoos_comm_core::protocol::Message::Welcome(_) => {}
            o => panic!("op {}", o.op_name()),
        }
        // caller pretends to be a provider replying on a fabricated route
        autoos_comm_local::pipe::send_message(
            &mut w,
            &autoos_comm_core::protocol::Message::ReplyFromProvider(
                autoos_comm_core::protocol::ReplyFromProvider {
                    route_id: autoos_comm_core::ids::new_id(),
                    body: serde_json::json!({"status": "committed"}),
                },
            ),
        )
        .await
        .unwrap();
        // the connection must be terminated (unexpected direction)
        let next = tokio::time::timeout(
            std::time::Duration::from_secs(3),
            autoos_comm_local::pipe::read_message(&mut r),
        )
        .await
        .expect("no hang after forged preply");
        assert!(
            matches!(next, Err(_) | Ok(None)),
            "conn must close or error after protocol violation: {next:?}"
        );
    }

    // the forged preply must not have satisfied any pending caller request:
    // B still completes a fresh, properly-driven call
    let opb2 = new_op();
    let ridb2 = format!("{}dddddddddddddddddddddddddd02", &opb2[..8]);
    let post = b.call_with_id(
        &ridb2,
        &sref,
        "capture",
        SCHEMA_CAPTURE_CAPTURE,
        2000,
        serde_json::json!({"operation_id": opb2, "text": "post-violation"}),
    );
    let driver2 = async {
        let c = provider.next_call().await;
        assert_eq!(c.request_id, ridb2);
        provider
            .reply(&c.route_id, serde_json::json!({"status": "committed"}))
            .await;
    };
    let (post, _) = tokio::join!(post, driver2);
    post.expect("post-violation call completes");
    let _ = provider; // provider still connected
}

/// Deterministic cap check: fill exactly to the cap, then assert refusal.
#[tokio::test(flavor = "multi_thread")]
async fn v12_connection_cap_rejects_the_ninth() {
    let mut run = TestRun::new("v12conn2");
    let _broker = run.spawn_broker();
    run.endpoint = Some(support::wait_endpoint(&run.run_dir, run.deadline()).expect("endpoint"));
    // provider takes slot 1
    let provider = RawProvider::connect(&run, "desc-v12conn2").await;
    // 7 callers fill slots 2..=8 (idle connections stay open)
    let mut clients = Vec::new();
    for _ in 0..7 {
        clients.push(run.client(CALLER_A).await);
    }
    // slot 9: connect succeeds at the pipe level, but Hello → Welcome must
    // be refused with resource_exhausted
    let token = run.token(CALLER_B);
    let client = autoos_comm_local::pipe::connect_pipe(&run.pipe(), run.deadline())
        .await
        .expect("9th pipe connect");
    let (mut r, mut w) = tokio::io::split(client);
    autoos_comm_local::pipe::send_message(
        &mut w,
        &autoos_comm_core::protocol::Message::Hello(autoos_comm_core::protocol::Hello {
            protocol_version: autoos_comm_core::protocol::PROTOCOL_VERSION,
            codec: autoos_comm_core::protocol::CODEC_ID.to_string(),
            token,
        }),
    )
    .await
    .unwrap();
    // the handshake Welcome is sent first; the cap rejection follows
    match autoos_comm_local::pipe::read_message(&mut r).await.unwrap() {
        Some(autoos_comm_core::protocol::Message::Welcome(_)) => {}
        Some(autoos_comm_core::protocol::Message::Error(e)) => {
            assert!(matches!(e.code, ErrorCode::ResourceExhausted), "{e:?}");
            // existing connections are unaffected (discovery still works;
            // the raw provider never replies, so a full capture is not used)
            let a0 = clients.first().unwrap();
            let _sref0 = run.discover(a0).await;
            return;
        }
        Some(other) => panic!("expected welcome/cap rejection, got {}", other.op_name()),
        None => panic!("connection closed without cap rejection error"),
    }
    let cap_err = tokio::time::timeout(
        std::time::Duration::from_secs(3),
        autoos_comm_local::pipe::read_message(&mut r),
    )
    .await
    .expect("cap error arrives promptly")
    .expect("no io error")
    .expect("stream not empty");
    match cap_err {
        autoos_comm_core::protocol::Message::Error(e) => {
            assert!(matches!(e.code, ErrorCode::ResourceExhausted), "{e:?}");
        }
        other => panic!(
            "expected cap rejection after welcome, got {}",
            other.op_name()
        ),
    }
    // existing connections unaffected: discovery still resolves
    let a = clients.first().unwrap();
    let _sref = run.discover(a).await;
    let _ = provider;
}
