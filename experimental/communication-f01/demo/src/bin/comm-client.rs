//! comm-client — one-shot CLI caller: explicit discover, then capture /
//! receipt calls. Results are JSON lines on stdout; exit 0 when every
//! operation succeeded, 1 when any failed, 2 on usage errors. Credentials
//! are read from files, never passed on the command line or logged.

use autoos_comm_core::protocol::{
    SCHEMA_CAPTURE_CAPTURE, SCHEMA_CAPTURE_RECEIPT, SERVICE_ID_CAPTURE,
};
use autoos_comm_local::client::CommClient;
use serde::Serialize;
use std::time::{Duration, Instant};

#[derive(Serialize)]
struct ResultLine {
    kind: &'static str,
    operation_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    status: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    capture_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    latency_ms: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    error_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    error: Option<String>,
}

fn usage() -> std::process::ExitCode {
    eprintln!(
        "usage: comm-client --endpoint <endpoint.json> --token-file <f> --cmd <capture|receipt|batch> [options]\n\
         capture: --op <128bit-hex> --text <s> | --text-file <f> [--title <s>] [--source-uri <s>] [--timeout-ms <1..5000>]\n\
         receipt: --op <128bit-hex> [--timeout-ms]\n\
         batch:   --file <jsonl> [--timeout-ms]  (lines: {{\"operation_id\",\"text\",\"title\"?,\"source_uri\"?}})"
    );
    std::process::ExitCode::from(2)
}

struct Opts {
    endpoint: String,
    token_file: String,
    cmd: String,
    op: Option<String>,
    text: Option<String>,
    text_file: Option<String>,
    title: Option<String>,
    source_uri: Option<String>,
    timeout_ms: u32,
    batch_file: Option<String>,
}

fn parse_args() -> Option<Opts> {
    let args: Vec<String> = std::env::args().collect();
    let mut o = Opts {
        endpoint: String::new(),
        token_file: String::new(),
        cmd: String::new(),
        op: None,
        text: None,
        text_file: None,
        title: None,
        source_uri: None,
        timeout_ms: autoos_comm_core::Limits::DEFAULT_TIMEOUT_MS,
        batch_file: None,
    };
    let mut i = 1;
    while i < args.len() {
        let get = |i: usize| args.get(i).cloned();
        match args[i].as_str() {
            "--endpoint" => o.endpoint = get(i + 1)?,
            "--token-file" => o.token_file = get(i + 1)?,
            "--cmd" => o.cmd = get(i + 1)?,
            "--op" => o.op = get(i + 1),
            "--text" => o.text = get(i + 1),
            "--text-file" => o.text_file = get(i + 1),
            "--title" => o.title = get(i + 1),
            "--source-uri" => o.source_uri = get(i + 1),
            "--timeout-ms" => o.timeout_ms = get(i + 1)?.parse().ok()?,
            "--file" => o.batch_file = get(i + 1),
            _ => return None,
        }
        i += 2;
    }
    Some(o)
}

fn emit(line: &ResultLine) {
    println!("{}", serde_json::to_string(line).expect("serialize result"));
}

fn err_line(op: &str, e: &autoos_comm_core::error::CommError) -> ResultLine {
    ResultLine {
        kind: "error",
        operation_id: op.to_string(),
        status: None,
        capture_id: None,
        latency_ms: None,
        error_code: Some(format!("{:?}", e.code)),
        error: Some(e.to_string()),
    }
}

#[tokio::main(flavor = "multi_thread")]
async fn main() -> std::process::ExitCode {
    let Some(o) = parse_args() else {
        return usage();
    };
    let token = match std::fs::read_to_string(&o.token_file) {
        Ok(t) => t.trim().to_string(),
        Err(e) => {
            eprintln!("cannot read token file: {e}");
            return std::process::ExitCode::from(2);
        }
    };
    let endpoint: autoos_comm_demo::support::BrokerEndpoint =
        match autoos_comm_demo::support::read_json(std::path::Path::new(&o.endpoint)) {
            Ok(e) => e,
            Err(e) => {
                eprintln!("cannot read endpoint: {e}");
                return std::process::ExitCode::from(2);
            }
        };
    let deadline = Instant::now() + Duration::from_secs(5);
    let client = match CommClient::connect(&endpoint.pipe_name, &token, deadline).await {
        Ok(c) => c,
        Err(e) => {
            emit(&err_line("(connect)", &e));
            return std::process::ExitCode::from(1);
        }
    };
    let service_ref = match client.discover(SERVICE_ID_CAPTURE).await {
        Ok(s) => s,
        Err(e) => {
            emit(&err_line("(discover)", &e));
            return std::process::ExitCode::from(1);
        }
    };
    let principal = client.principal().to_string();
    let ok = match o.cmd.as_str() {
        "capture" => {
            let Some(op) = o.op.clone() else {
                return usage();
            };
            let text = match (o.text.clone(), o.text_file.clone()) {
                (Some(t), _) => t,
                (None, Some(f)) => match std::fs::read_to_string(&f) {
                    Ok(t) => t,
                    Err(e) => {
                        eprintln!("cannot read text file: {e}");
                        return std::process::ExitCode::from(2);
                    }
                },
                _ => return usage(),
            };
            one_capture(
                &client,
                &service_ref,
                &op,
                &text,
                o.title.as_deref(),
                o.source_uri.as_deref(),
                o.timeout_ms,
            )
            .await
        }
        "receipt" => {
            let Some(op) = o.op.clone() else {
                return usage();
            };
            one_receipt(&client, &service_ref, &op, &principal, o.timeout_ms).await
        }
        "batch" => {
            let Some(file) = o.batch_file.clone() else {
                return usage();
            };
            let content = match std::fs::read_to_string(&file) {
                Ok(c) => c,
                Err(e) => {
                    eprintln!("cannot read batch file: {e}");
                    return std::process::ExitCode::from(2);
                }
            };
            let mut all_ok = true;
            for line in content.lines() {
                if line.trim().is_empty() {
                    continue;
                }
                #[derive(serde::Deserialize)]
                struct BatchItem {
                    operation_id: String,
                    text: String,
                    title: Option<String>,
                    source_uri: Option<String>,
                }
                let item: BatchItem = match serde_json::from_str(line) {
                    Ok(v) => v,
                    Err(e) => {
                        eprintln!("bad batch line: {e}");
                        return std::process::ExitCode::from(2);
                    }
                };
                if !one_capture(
                    &client,
                    &service_ref,
                    &item.operation_id,
                    &item.text,
                    item.title.as_deref(),
                    item.source_uri.as_deref(),
                    o.timeout_ms,
                )
                .await
                {
                    all_ok = false;
                }
            }
            all_ok
        }
        _ => return usage(),
    };
    if ok {
        std::process::ExitCode::SUCCESS
    } else {
        std::process::ExitCode::from(1)
    }
}

async fn one_capture(
    client: &CommClient,
    service_ref: &autoos_comm_core::protocol::ServiceRef,
    op: &str,
    text: &str,
    title: Option<&str>,
    source_uri: Option<&str>,
    timeout_ms: u32,
) -> bool {
    let started = Instant::now();
    let mut body = serde_json::json!({
        "operation_id": op,
        "text": text,
    });
    if let Some(t) = title {
        body["title"] = serde_json::Value::String(t.to_string());
    }
    if let Some(u) = source_uri {
        body["source_uri"] = serde_json::Value::String(u.to_string());
    }
    match client
        .call(
            service_ref,
            "capture",
            SCHEMA_CAPTURE_CAPTURE,
            timeout_ms,
            body,
        )
        .await
    {
        Ok(reply) => {
            let line = ResultLine {
                kind: "capture_result",
                operation_id: op.to_string(),
                status: reply
                    .get("status")
                    .and_then(|v| v.as_str())
                    .map(String::from),
                capture_id: reply
                    .get("capture_id")
                    .and_then(|v| v.as_str())
                    .map(String::from),
                latency_ms: Some(started.elapsed().as_millis() as u64),
                error_code: None,
                error: None,
            };
            emit(&line);
            line.status.as_deref() == Some("committed")
        }
        Err(e) => {
            emit(&err_line(op, &e));
            false
        }
    }
}

async fn one_receipt(
    client: &CommClient,
    service_ref: &autoos_comm_core::protocol::ServiceRef,
    op: &str,
    principal: &str,
    timeout_ms: u32,
) -> bool {
    let started = Instant::now();
    let body = serde_json::json!({ "operation_id": op, "principal": principal });
    match client
        .call(
            service_ref,
            "receipt",
            SCHEMA_CAPTURE_RECEIPT,
            timeout_ms,
            body,
        )
        .await
    {
        Ok(reply) => {
            let line = ResultLine {
                kind: "receipt_result",
                operation_id: op.to_string(),
                status: reply
                    .get("status")
                    .and_then(|v| v.as_str())
                    .map(String::from),
                capture_id: reply
                    .get("capture_id")
                    .and_then(|v| v.as_str())
                    .map(String::from),
                latency_ms: Some(started.elapsed().as_millis() as u64),
                error_code: None,
                error: None,
            };
            emit(&line);
            true // receipt answered (committed or not_observed are both answers)
        }
        Err(e) => {
            emit(&err_line(op, &e));
            false
        }
    }
}
