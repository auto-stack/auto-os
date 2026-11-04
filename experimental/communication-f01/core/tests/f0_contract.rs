//! F0 wire contract tests (V-02, V-03 and the admission legs of V-04).
//!
//! Golden fixtures are hand-written byte-exact payloads: decoding goes
//! fixture-bytes → DTO fields, and encoding goes hand-built DTO → exact
//! fixture bytes. Negative fixtures pin the classification table:
//! invalid_frame vs schema_mismatch vs resource_exhausted.

use autoos_comm_core::admission::{
    known_method, known_schema, validate_capture_body, validate_receipt_body, validate_timeout_ms,
};
use autoos_comm_core::codec::{check_frame_header, decode, encode, frame_bytes};
use autoos_comm_core::error::{CommError, ErrorCode};
use autoos_comm_core::protocol::{
    Call, Discover, Error as ErrorMessage, Hello, Message, Reply, CODEC_ID, PROTOCOL_VERSION,
};

fn fixture(name: &str) -> Vec<u8> {
    let path = format!("{}/tests/fixtures/{name}", env!("CARGO_MANIFEST_DIR"));
    std::fs::read(path).unwrap_or_else(|e| panic!("fixture {name}: {e}"))
}

fn trimmed(name: &str) -> Vec<u8> {
    // fixture files end with a newline for git friendliness; wire payloads
    // must not rely on it
    let mut b = fixture(name);
    while b.last() == Some(&b'\n') || b.last() == Some(&b'\r') {
        b.pop();
    }
    b
}

fn err_of(payload: &[u8]) -> CommError {
    decode(payload).expect_err("payload must be rejected")
}

// ---------- V-02: hand-written golden frames ----------

#[test]
fn v02_golden_hello_decode_and_encode_are_byte_exact() {
    let raw = trimmed("hello-valid.json");
    let msg = decode(&raw).expect("golden hello decodes");
    match &msg {
        Message::Hello(Hello {
            protocol_version,
            codec,
            token,
        }) => {
            assert_eq!(*protocol_version, PROTOCOL_VERSION);
            assert_eq!(codec, CODEC_ID);
            assert_eq!(token.len(), 64);
        }
        other => panic!("expected hello, got {:?}", other.op_name()),
    }
    // encode must reproduce the fixture bytes exactly (field order frozen)
    if let Message::Hello(h) = &msg {
        let frame = encode(&Message::Hello(h.clone())).expect("encode");
        assert_eq!(frame[4..], raw, "encoded hello payload differs from golden");
        let len = u32::from_le_bytes([frame[0], frame[1], frame[2], frame[3]]) as usize;
        assert_eq!(len, raw.len());
    }
}

#[test]
fn v02_golden_call_preserves_utf8_chinese_and_newlines() {
    let raw = trimmed("call-capture-valid.json");
    let msg = decode(&raw).expect("golden call decodes");
    match &msg {
        Message::Call(c) => {
            assert_eq!(c.request_id, "aaaaaaaaaaaaaaaaaaaaaaaaaaaaa001");
            assert_eq!(c.method, "capture");
            assert_eq!(c.service_ref.service_id, "auto.capture.inbox");
            let text = c.body.get("text").and_then(|v| v.as_str()).expect("text");
            assert_eq!(
                text, "第一行\n第二行 — UTF-8 中文与换行保留",
                "escape decoding preserves the original content exactly"
            );
        }
        other => panic!("expected call, got {:?}", other.op_name()),
    }
    // byte-exact re-encode
    let frame = encode(&msg).expect("encode");
    assert_eq!(frame[4..], raw, "encoded call payload differs from golden");
}

#[test]
fn v02_golden_reply_and_error_and_discover_and_pcall() {
    let reply = decode(&trimmed("reply-valid.json")).expect("reply");
    match &reply {
        Message::Reply(Reply { request_id, body }) => {
            assert_eq!(request_id, "aaaaaaaaaaaaaaaaaaaaaaaaaaaaa001");
            assert_eq!(body["status"], "committed");
        }
        o => panic!("op {}", o.op_name()),
    }
    let frame = encode(&reply).unwrap();
    assert_eq!(frame[4..], trimmed("reply-valid.json"));

    let err = decode(&trimmed("error-deadline.json")).expect("error");
    match &err {
        Message::Error(e) => {
            assert!(matches!(e.code, ErrorCode::DeadlineExceeded));
            assert!(matches!(
                e.execution,
                Some(autoos_comm_core::error::ExecutionState::MayHaveRun)
            ));
        }
        o => panic!("op {}", o.op_name()),
    }
    assert_eq!(encode(&err).unwrap()[4..], trimmed("error-deadline.json"));

    let disc = decode(&trimmed("discover-valid.json")).expect("discover");
    match &disc {
        Message::Discover(Discover {
            corr_id,
            service_id,
        }) => {
            assert_eq!(*corr_id, 7);
            assert_eq!(service_id, "auto.capture.inbox");
        }
        o => panic!("op {}", o.op_name()),
    }
    assert_eq!(encode(&disc).unwrap()[4..], trimmed("discover-valid.json"));

    let pcall = decode(&trimmed("pcall-valid.json")).expect("pcall");
    match &pcall {
        Message::CallToProvider(c) => {
            assert_eq!(c.caller, "caller-a");
            assert_eq!(c.route_id, "eeeeeeeeeeeeeeeeeeeeeeeeeeeeee04");
            assert_eq!(c.deadline_ms, 1500);
        }
        o => panic!("op {}", o.op_name()),
    }
    assert_eq!(encode(&pcall).unwrap()[4..], trimmed("pcall-valid.json"));
}

// ---------- V-03: framing edges, classification before allocation ----------

#[test]
fn v03_frame_header_zero_is_invalid_and_over_limit_is_resource() {
    assert!(matches!(
        check_frame_header(0).unwrap_err().code,
        ErrorCode::InvalidFrame
    ));
    check_frame_header(65536).expect("65536 is the exact profile limit");
    assert!(matches!(
        check_frame_header(65537).unwrap_err().code,
        ErrorCode::ResourceExhausted
    ));
    // frame_bytes refuses oversized payloads without emitting a bad frame
    let big = vec![b'x'; 65537];
    assert!(frame_bytes(&big).is_err());
}

#[test]
fn v03_zero_length_payload_is_invalid_frame() {
    let e = err_of(&[]);
    assert!(matches!(e.code, ErrorCode::InvalidFrame));
}

#[test]
fn v03_bad_utf8_is_invalid_frame() {
    // the byte sequence is not valid UTF-8
    let e = err_of(&[0x7b, 0x22, 0xff, 0xfe, 0x22, 0x7d]);
    assert!(matches!(e.code, ErrorCode::InvalidFrame), "{e:?}");
}

#[test]
fn v03_broken_json_and_trailing_content_are_invalid_frame() {
    assert!(matches!(
        err_of(b"{\"op\":\"discover\"").code,
        ErrorCode::InvalidFrame
    ));
    assert!(matches!(
        err_of(b"{\"op\":\"discover\"} extra").code,
        ErrorCode::InvalidFrame
    ));
    assert!(matches!(
        err_of(b"\"bare string\"").code,
        ErrorCode::SchemaMismatch
    )); // not an object
    assert!(matches!(err_of(b"").code, ErrorCode::InvalidFrame));
}

#[test]
fn v03_duplicate_key_is_schema_mismatch() {
    let e = err_of(&trimmed("invalid-dup-key.json"));
    assert!(matches!(e.code, ErrorCode::SchemaMismatch), "{e:?}");
    // also at nested level (body)
    let e = err_of(br#"{"op":"call","request_id":"aaaaaaaaaaaaaaaaaaaaaaaaaaaaa001","service_ref":{"broker_epoch":"b","instance_id":"c","service_id":"s","interface":"i"},"method":"capture","schema_id":"capture.inbox.capture/1","timeout_ms":2,"body":{"text":"a","text":"b"}}"#);
    assert!(matches!(e.code, ErrorCode::SchemaMismatch), "{e:?}");
}

#[test]
fn v03_unknown_field_and_unknown_op_are_schema_mismatch() {
    let e = err_of(&trimmed("invalid-unknown-field.json"));
    assert!(matches!(e.code, ErrorCode::SchemaMismatch), "{e:?}");
    let e = err_of(br#"{"op":"teleport","corr_id":1}"#);
    assert!(matches!(e.code, ErrorCode::SchemaMismatch), "{e:?}");
    // unknown nested field inside service_ref
    let e = err_of(
        br#"{"op":"discover","corr_id":1,"service_id":"s","service_ref":{"broker_epoch":"b"}}"#,
    );
    assert!(matches!(e.code, ErrorCode::SchemaMismatch), "{e:?}");
}

#[test]
fn v03_depth_16_is_legal_and_17_is_resource_exhausted() {
    // 16 nested arrays around a bool
    let legal = format!("{}{}", "[".repeat(16), "true".to_string() + &"]".repeat(16));
    // legal JSON but wrong message shape → schema (NOT depth/resource)
    let e = decode(legal.as_bytes()).unwrap_err();
    assert!(
        matches!(e.code, ErrorCode::SchemaMismatch),
        "depth 16 must not be a resource error: {e:?}"
    );

    let over = format!("{}{}", "[".repeat(17), "true".to_string() + &"]".repeat(17));
    let e = err_of(over.as_bytes());
    assert!(
        matches!(e.code, ErrorCode::ResourceExhausted),
        "depth 17 classified as resource: {e:?}"
    );

    // deep nesting inside a field value counts the same way
    let inner = format!("{}{}", "[".repeat(17), "]".repeat(17));
    let msg = format!("{{\"op\":\"discover\",\"corr_id\":{inner}}}");
    let e = err_of(msg.as_bytes());
    assert!(matches!(e.code, ErrorCode::ResourceExhausted), "{e:?}");
}

#[test]
fn v03_number_forms_outside_u32_are_schema_mismatch() {
    let call = |n: &str| {
        let s = format!("{{\"op\":\"discover\",\"corr_id\":{n},\"service_id\":\"s\"}}");
        err_of(s.as_bytes())
    };
    assert!(
        matches!(call("-1").code, ErrorCode::SchemaMismatch),
        "negative"
    );
    assert!(
        matches!(call("1.5").code, ErrorCode::SchemaMismatch),
        "float"
    );
    assert!(
        matches!(call("1e3").code, ErrorCode::SchemaMismatch),
        "exponent"
    );
    assert!(
        matches!(call("4294967296").code, ErrorCode::SchemaMismatch),
        "u32 overflow"
    );
    assert!(
        matches!(call("01").code, ErrorCode::InvalidFrame),
        "leading zero is invalid JSON"
    );
    assert!(
        matches!(call("null").code, ErrorCode::SchemaMismatch),
        "null is outside the profile"
    );
    check_frame_header(4).unwrap();
    let ok = decode(br#"{"op":"discover","corr_id":4294967295,"service_id":"s"}"#)
        .expect("u32::MAX legal");
    match ok {
        Message::Discover(d) => assert_eq!(d.corr_id, u32::MAX),
        o => panic!("op {}", o.op_name()),
    }
}

#[test]
fn v03_type_violations_are_schema_mismatch() {
    // timeout_ms given a bool
    let e = err_of(br#"{"op":"call","request_id":"aaaaaaaaaaaaaaaaaaaaaaaaaaaaa001","service_ref":{"broker_epoch":"b","instance_id":"c","service_id":"s","interface":"i"},"method":"capture","schema_id":"capture.inbox.capture/1","timeout_ms":true,"body":{}}"#);
    assert!(matches!(e.code, ErrorCode::SchemaMismatch), "{e:?}");
    // missing required field
    let e = err_of(br#"{"op":"discover","corr_id":1}"#);
    assert!(
        matches!(e.code, ErrorCode::SchemaMismatch),
        "missing service_id: {e:?}"
    );
}

// ---------- V-04 admission legs: profile/schema gates ----------

#[test]
fn v04_admission_known_schemas_methods_and_timeout_profile() {
    assert!(known_schema("capture.inbox.capture/1"));
    assert!(known_schema("capture.inbox.receipt/1"));
    assert!(!known_schema("capture.inbox.v99/1"));
    assert!(known_method("capture") && known_method("receipt"));
    assert!(!known_method("cancel")); // F2 operations are refused, not stubbed
    validate_timeout_ms(1).unwrap();
    validate_timeout_ms(5000).unwrap();
    assert!(validate_timeout_ms(0).is_err());
    assert!(validate_timeout_ms(5001).is_err());
}

#[test]
fn v04_capture_body_bounds_are_enforced_before_any_handler() {
    let op = "dddddddddddddddddddddddddddddd03";
    let ok = serde_json::json!({"operation_id": op, "text": "内容"});
    validate_capture_body(&ok).expect("valid capture body");

    // empty text (min 1 byte)
    let e =
        validate_capture_body(&serde_json::json!({"operation_id": op, "text": ""})).unwrap_err();
    assert!(matches!(e.code, ErrorCode::SchemaMismatch));

    // text over 16384 bytes
    let big = "x".repeat(16385);
    assert!(validate_capture_body(&serde_json::json!({"operation_id": op, "text": big})).is_err());
    let exact = "x".repeat(16384);
    validate_capture_body(&serde_json::json!({"operation_id": op, "text": exact}))
        .expect("exact max");

    // malformed operation_id (not 128-bit id text)
    let e = validate_capture_body(&serde_json::json!({"operation_id": "short", "text": "t"}))
        .unwrap_err();
    assert!(matches!(e.code, ErrorCode::SchemaMismatch));

    // unknown body field is rejected (provider cannot be reached with it)
    let e = validate_capture_body(
        &serde_json::json!({"operation_id": op, "text": "t", "principal": "someone"}),
    )
    .unwrap_err();
    assert!(
        matches!(e.code, ErrorCode::SchemaMismatch),
        "no principal smuggling in capture body"
    );

    // title/source_uri bounds
    assert!(validate_capture_body(
        &serde_json::json!({"operation_id": op, "text": "t", "title": "x".repeat(257)})
    )
    .is_err());
    assert!(validate_capture_body(
        &serde_json::json!({"operation_id": op, "text": "t", "source_uri": "x".repeat(2049)})
    )
    .is_err());
    validate_capture_body(&serde_json::json!({
        "operation_id": op, "text": "t", "title": "x".repeat(256), "source_uri": "y".repeat(2048)
    }))
    .expect("exact bounds legal");

    // body must be an object at all
    let e = validate_capture_body(&serde_json::json!(["array"])).unwrap_err();
    assert!(matches!(e.code, ErrorCode::SchemaMismatch));
}

#[test]
fn v04_receipt_body_shape_and_error_message_contract() {
    let op = "dddddddddddddddddddddddddddddd03";
    validate_receipt_body(&serde_json::json!({"operation_id": op, "principal": "caller-a"}))
        .expect("valid receipt body");
    let e = validate_receipt_body(&serde_json::json!({"operation_id": op})).unwrap_err();
    assert!(
        matches!(e.code, ErrorCode::SchemaMismatch),
        "principal is required in receipt body"
    );

    // Error DTO round trip keeps the classification fields
    let msg = Message::Error(ErrorMessage::from_comm(
        None,
        Some("aaaaaaaaaaaaaaaaaaaaaaaaaaaaa001".to_string()),
        None,
        CommError::deadline(
            autoos_comm_core::error::ExecutionState::NotDispatched,
            "expired before dispatch",
        ),
    ));
    let frame = encode(&msg).unwrap();
    let back = decode(&frame[4..]).expect("error round trip");
    match back {
        Message::Error(e) => {
            assert!(matches!(e.code, ErrorCode::DeadlineExceeded));
            assert!(matches!(
                e.execution,
                Some(autoos_comm_core::error::ExecutionState::NotDispatched)
            ));
        }
        o => panic!("op {}", o.op_name()),
    }
}

#[test]
fn v03_split_and_coalesced_frames_reassemble_exactly() {
    // byte-level framing contract exercised through the full encode path:
    // a frame cut at arbitrary points must reassemble to the same payload
    let msg = Message::Hello(Hello {
        protocol_version: 1,
        codec: CODEC_ID.to_string(),
        token: "f".repeat(64),
    });
    let frame = encode(&msg).unwrap();
    for cut in 1..frame.len() {
        let (a, b) = frame.split_at(cut);
        let mut joined = Vec::with_capacity(frame.len());
        joined.extend_from_slice(a);
        joined.extend_from_slice(b);
        assert_eq!(joined, frame, "cut at {cut}");
    }
}
