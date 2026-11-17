//! PLAN-064 M2-A 最小（MIN-A-OS）：永久回归（V-OS-A-MIN 各腿）。
//!
//! 命令（冻结）：
//! ```text
//! cargo test -p os-desktop --features ace-min --test ace_min -- --test-threads=1
//! ```
//! 真 Windows 窗腿需 `AUTO_OS_ACE_MIN_E2E=1`（980 r11 同档：真窗/交互档
//! 环境门控）；provider 由首次 `ensure_ui980_provider` 预备（缓存在
//! target/debug/ace-node-desktop，命中后测试零 rustc）。
//!
//! 全局 statics（980 HOST/BRIDGE 单例）要求串行：子进程驱动天然隔离，
//! 静态互斥再保 e2e 腿独占。
//!
//! feature 关的行为（`--ace-min` 具名拒、绝不落旧 VM）由 ace_min 模块
//! cfg 臂承担，验收以无 feature 构建直跑 exe 的具名拒输出为证（V 腿），
//! 不设同义反复的单测。
#![cfg(all(windows, feature = "ace-min"))]

use std::process::Command;
use std::sync::Mutex;

static SERIAL: Mutex<()> = Mutex::new(());

fn exe_path() -> std::path::PathBuf {
    env!("CARGO_BIN_EXE_ui-desktop").into()
}

fn temp_evidence(name: &str) -> std::path::PathBuf {
    let dir = std::env::temp_dir().join("p064-ace-min-evidence");
    let _ = std::fs::create_dir_all(&dir);
    dir.join(name)
}

fn spawn_witness(args: &[&str], evidence: &std::path::Path) -> serde_json::Value {
    let _guard = SERIAL.lock().unwrap();
    let out = Command::new(exe_path())
        .args(args)
        .arg("--ace-evidence")
        .arg(evidence)
        .output()
        .expect("spawn ui-desktop");
    assert!(
        out.status.success(),
        "witness run failed: {}",
        String::from_utf8_lossy(&out.stderr)
    );
    let bytes = std::fs::read(evidence).expect("evidence written");
    serde_json::from_slice(&bytes).expect("evidence JSON")
}

#[test]
fn p064_hello_static_face_headless() {
    let ev = spawn_witness(
        &["--ace-min", "ace.hello.min"],
        &temp_evidence("hello-headless.json"),
    );
    assert_eq!(ev["candidate"]["id"], "ace.hello.min");
    assert_eq!(ev["candidate"]["engine"], "av");
    assert_eq!(ev["candidate"]["blake3"].as_str().unwrap().len(), 64);
    // 拒绝腿：三腿全拒且零窗帧。
    let probes = ev["probes"].as_array().unwrap();
    assert_eq!(probes.len(), 3, "wrong_tag/wrong_handle/revocation");
    for p in probes {
        assert_eq!(p["frames_delta"], 0, "{p}");
    }
    assert!(probes[2]["refusal"].as_str().unwrap().contains("denied"));
    // 静面：无事件引擎、无按钮、宿主关闭回收 poll=-2。
    let h = &ev["headless"];
    assert_eq!(h["engine_face"], false);
    assert!(h["buttons"].as_array().unwrap().is_empty());
    assert_eq!(h["host_close"]["poll_after_close"], -2);
    assert!(h["host_close"]["closed_session"].is_i64());
    assert!(h["errors"].as_array().unwrap().is_empty());
    assert_eq!(ev["frames_rendered_total"], 0, "headless 不冒真窗");
}

#[test]
fn p064_counter_trajectory_two_instances_close_reclaim_reopen() {
    let ev = spawn_witness(
        &["--ace-min", "ace.counter.min"],
        &temp_evidence("counter-headless.json"),
    );
    let h = &ev["headless"];
    assert_eq!(h["engine_face"], true);
    // 980 冻结按钮序（DEC=0/RESET=1/INC=2）。
    let buttons: Vec<&str> = h["buttons"]
        .as_array()
        .unwrap()
        .iter()
        .map(|b| b["msg"].as_str().unwrap())
        .collect();
    assert_eq!(buttons, vec!["Dec", "Reset", "Inc"]);
    // 冻结轨迹：i0 Inc,Dec,Reset,Inc,Inc → 0→1→0→0→1→2；i1 Inc → 1（2:1）。
    let dispatched: Vec<(&str, u64, &str)> = h["outcomes"]
        .as_array()
        .unwrap()
        .iter()
        .filter(|o| o["kind"] == "dispatched")
        .map(|o| {
            (
                o["msg"].as_str().unwrap(),
                o["instance"].as_u64().unwrap(),
                o["states"][0][1].as_str().unwrap(),
            )
        })
        .collect();
    assert_eq!(
        dispatched,
        vec![
            ("Inc", 0, "1"),
            ("Dec", 0, "0"),
            ("Reset", 0, "0"),
            ("Inc", 0, "1"),
            ("Inc", 0, "2"),
            ("Inc", 1, "1"),
        ],
        "dispatch 轨迹须与 980 冻结同源"
    );
    // 脚本 close → late 丢弃计数；宿主收尾关闭 → poll -2。
    assert_eq!(h["script_close"]["closed_sessions"], serde_json::json!([1]));
    assert_eq!(h["script_close"]["late_dropped"], 1);
    assert_eq!(h["host_close"]["poll_after_close"], -2);
    // 重开：新世代、业务状态存活（2:1）。
    assert_eq!(
        h["final_states"],
        serde_json::json!([["i0", [["count", "Int(2)"]]], ["i1", [["count", "Int(1)"]]]])
    );
    let reopen_gen = h["reopen_last"]["generation"].as_u64().unwrap();
    assert!(reopen_gen >= 2, "重开世代递增：{reopen_gen}");
    assert!(h["errors"].as_array().unwrap().is_empty());
    assert_eq!(ev["frames_rendered_total"], 0);
}

#[test]
fn p064_dual_app_sequential_isolation() {
    // 双 App 反例（每 App 独立宿主进程——隔离面同真实桌面 outproc 形态）：
    // 各自 oracle 独立成立、业务互不串（hello 无 count 字段、counter 自
    // 0 起轨至 2:1）。同宿主进程的双 App 会话单调/不复活腿见
    // ace_min::imp::tests::p064_dual_app_one_host_process_isolation。
    let hello = spawn_witness(
        &["--ace-min", "ace.hello.min"],
        &temp_evidence("dual-hello.json"),
    );
    let counter = spawn_witness(
        &["--ace-min", "ace.counter.min"],
        &temp_evidence("dual-counter.json"),
    );
    let hello_text = serde_json::to_string(&hello["headless"]["final_states"]).unwrap();
    assert!(
        !hello_text.contains("count"),
        "hello 静面不得出现 counter 业务字段: {hello_text}"
    );
    let counter_text = serde_json::to_string(&counter["headless"]["final_states"]).unwrap();
    assert!(
        counter_text.contains("Int(2)"),
        "counter 轨迹自 0 起不受 hello 影响: {counter_text}"
    );
    assert_eq!(
        hello["headless"]["host_close"]["poll_after_close"], -2,
        "hello 关闭回收独立成立"
    );
    assert_eq!(
        counter["headless"]["host_close"]["poll_after_close"], -2,
        "counter 关闭回收独立成立"
    );
}

#[test]
fn p064_window_e2e_real_input_and_frames() {
    let Ok(gate) = std::env::var("AUTO_OS_ACE_MIN_E2E") else {
        eprintln!("skip: AUTO_OS_ACE_MIN_E2E 未设（真窗档环境门控）");
        return;
    };
    if gate.is_empty() {
        eprintln!("skip: AUTO_OS_ACE_MIN_E2E 空");
        return;
    }
    let _guard = SERIAL.lock().unwrap();
    let evidence = temp_evidence("counter-window.json");
    let _ = std::fs::remove_file(&evidence);
    let out = Command::new(exe_path())
        .args([
            "--ace-min",
            "ace.counter.min",
            "--ace-window",
            "--hold-ms",
            "9000",
        ])
        .arg("--ace-evidence")
        .arg(&evidence)
        .output()
        .expect("spawn windowed witness");
    assert!(
        out.status.success(),
        "{}",
        String::from_utf8_lossy(&out.stderr)
    );
    let ev: serde_json::Value =
        serde_json::from_slice(&std::fs::read(&evidence).expect("window evidence")).unwrap();
    let frames = ev["window"]["frames_rendered"].as_u64().unwrap();
    assert!(frames >= 4, "真窗渲染帧数（Iced 实绘）: {frames}");
    let sessions = ev["window"]["sessions"].as_array().unwrap();
    let applied: Vec<String> = sessions
        .iter()
        .flat_map(|s| s["applied"].as_array().unwrap().iter())
        .map(|a| a["value"].as_str().unwrap_or("").to_string())
        .collect();
    assert!(
        applied.iter().any(|v| v == "2"),
        "真窗腿提交轨迹含终值 2（2:1）: {applied:?}"
    );
}
