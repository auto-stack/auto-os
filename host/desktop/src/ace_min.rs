//! PLAN-064 M2-A 最小（MIN-A-OS）：真实 OS 候选启动/注册/激活/窗口会话链。
//!
//! 契约：本 Plan §5「MIN-A-OS真实见证与精确迁交」与 §5「T-01 A最小冻结」；
//! 980（ace-m2-autoui.md §12）已交 MIN-A-UI 通用 Node 桌面入口。边界：
//! - 隔离候选注册表**只**冻结 980 已交 Hello 与 Counter Node 见证（测试
//!   App ID；不新增生产 App、不动 apps.manifest / 生产 gitlink）。
//! - 启动链 = 读源 → 身份核（blake3/engine/provider，逐项具名拒）→
//!   `ensure_ui980_provider`（SDK 身份）→ `NodeDesktopBridge::load`
//!   （Schema/verify/AV 真值）→ `open`（真 FFI）→ 可选真窗
//!   （`AceNodeDesktopWindow`：真 Iced 窗口、真输入、tick 受控事件循环）
//!   → 关闭回收观察（closed / late_dropped / stale_rejected / 关后 poll
//!   = SESSION_CLOSED）→ 证据 JSON。
//! - 拒绝腿：错车道标签（BAD_OP）/ 错句柄（UNKNOWN_HANDLE）/ 过期
//!   session（SESSION_CLOSED）/ provider 撤权（Denied → readmit 恢复），
//!   全部**零 UI 效果**（无窗口帧、无 applies）后原入口恢复。
//! - 全链无旧 VM 解释调用：feature 关 = `--ace-min` 具名拒，绝不回退
//!   旧 VM；错 SDK/ABI/协议/授权一律具名拒后由正确入口恢复。
//!
//! 桌面 shell / 注册表启动完整业务仍归 B 波（HOST 完整余量）；本模块只
//! 承载 A 最小候选会话服务。
//!
//! CLI（挂在 `ui-desktop` 上；boot 真迹保留，隔离生产 daemon/注册表）：
//! ```text
//! ui-desktop --ace-min <id> [选项]
//!   --ace-min <id>         候选见证 id（ace.hello.min | ace.counter.min）
//!   --ace-instances <n>    覆盖实例数（1..=4，MIN-A 打包界）
//!   --ace-script <spec>    受控事件脚本（980 同格式：t:click:i:g,..|close|late|reopen）
//!   --ace-window           追加真窗腿（真 Iced 窗口；记录腿配 --hold-ms）
//!   --hold-ms <ms>         真窗腿到点存证退出（交互档 = 手动关窗）
//!   --ace-evidence <path>  证据 JSON 输出路径
//! ```

/// 候选见证源相对 auto-ui 根的路径（T-01 冻结）。
const HELLO_REL: &str = "examples/ui/ace-hello-minimal/src/front/app.at";
const COUNTER_REL: &str = "examples/ui/ace-node-counter/src/front/app.at";

/// 候选注册表冻结项（B-PROOF-I：既有候选注册表，非生产 manifest）。
#[derive(Clone, Debug)]
pub struct CandidateSpec {
    /// 稳定 App id（激活/证据/注册表键）。
    pub id: &'static str,
    /// 源相对 auto-ui 根路径。
    pub source_rel: &'static str,
    /// 源字节 blake3 hex（T-01 冻结；失配 = 来源变化，具名拒）。
    pub blake3: &'static str,
    /// 唯一合法引擎（`parse_ace_engine` 只认 av/ace-av；其余具名拒）。
    pub engine: &'static str,
    /// provider 注册名（错名 = 错 SDK，具名拒）。
    pub provider: &'static str,
    /// 缺省实例数（hello 静面 1；counter 两实例定向 2:1）。
    pub instances: usize,
    /// 缺省受控脚本（headless 业务腿；980 冻结轨迹同源）。
    pub script: &'static str,
}

/// 既有候选注册表（只读冻结；扩充须具名裁定，不在这里悄悄长）。
pub const CANDIDATES: &[CandidateSpec] = &[
    CandidateSpec {
        id: "ace.hello.min",
        source_rel: HELLO_REL,
        // T-01 冻结（p064_blake3_pins_match_frozen_sources 守恒）。
        blake3: "a5fcb1d19fc729500ab8ba54d327477090f2d607c07b1e3294bebba16aad4927",
        engine: "av",
        provider: "p980_ui980_native",
        instances: 1,
        script: "",
    },
    CandidateSpec {
        id: "ace.counter.min",
        source_rel: COUNTER_REL,
        blake3: "790d6fcc52634b5a4b4c7d940d73ffecedf7e792d84e034ccb644e4314d6f9e7",
        engine: "av",
        provider: "p980_ui980_native",
        instances: 2,
        // 980 冻结轨迹同源：i0 Inc,Dec,Reset,Inc,Inc + i1 Inc → 2:1；
        // 关闭 → late → 重开（世代递增、业务状态存活）。
        script: "1:click:0:2,2:click:0:0,3:click:0:1,4:click:0:2,5:click:0:2,\
                 6:click:1:2,8:close,9:late,11:reopen",
    },
];

/// blake3 hex（源/产物身份；与 provider_digest 同族）。
fn blake3_hex(bytes: &[u8]) -> String {
    let mut h = blake3::Hasher::new();
    h.update(bytes);
    h.finalize().to_hex().to_string()
}

/// auto-ui 根解析序：`AUTO_UI_ROOT` → 编译期锚（`<仓根>/../auto-ui`——
/// worktree 组兄弟 / 主检出兄弟两形态同解）。
pub fn resolve_ui_root() -> Result<std::path::PathBuf, String> {
    if let Some(p) = std::env::var_os("AUTO_UI_ROOT") {
        let root = std::path::PathBuf::from(p);
        if root.join("crates/auto-ui-runtime").is_dir() {
            return Ok(root);
        }
        return Err(format!(
            "ace-min: AUTO_UI_ROOT={} 缺 crates/auto-ui-runtime（错 SDK 根）",
            root.display()
        ));
    }
    // panel: <repo>/host/desktop → <repo>/.. → 兄弟 auto-ui
    let anchor = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("..")
        .join("..");
    let root = anchor.join("auto-ui");
    if root.join("crates/auto-ui-runtime").is_dir() {
        return Ok(root);
    }
    Err(format!(
        "ace-min: auto-ui 根不可达（env AUTO_UI_ROOT 或兄弟检出 {}）",
        root.display()
    ))
}

fn lookup_candidate(id: &str) -> Result<&'static CandidateSpec, String> {
    CANDIDATES.iter().find(|c| c.id == id).ok_or_else(|| {
        format!(
            "ace-min: 未注册候选 id `{id}`（既有：{}）",
            CANDIDATES
                .iter()
                .map(|c| c.id)
                .collect::<Vec<_>>()
                .join(", ")
        )
    })
}

/// 候选静态校验：engine/provider/instances/脚本界（不触源，纯逻辑）。
fn validate_spec(c: &CandidateSpec) -> Result<(), String> {
    if c.engine != "av" && c.engine != "ace-av" {
        // dispatch 须 ACE：旧 VM 引擎名一律具名拒（无回退）。
        return Err(format!(
            "ace-min: {} 引擎 `{}` 非 ACE AV（launch=vm 兼容名不适用于候选见证）",
            c.id, c.engine
        ));
    }
    if c.provider != "p980_ui980_native" {
        return Err(format!(
            "ace-min: {} provider `{}` 非 980 冻结 SDK 身份",
            c.id, c.provider
        ));
    }
    if c.instances == 0 || c.instances > 4 {
        return Err(format!(
            "ace-min: {} instances={} 越界（MIN-A 界 1..=4）",
            c.id, c.instances
        ));
    }
    Ok(())
}

/// 受控脚本解析（980 exe 同格式）：`t:click:i:g` / `close` / `late` /
/// `reopen`，逗号分隔；tick 从 1。坏 token 具名拒。
fn parse_script(spec: &str) -> Result<Vec<(usize, ScriptKind)>, String> {
    let mut out = Vec::new();
    if spec.is_empty() {
        return Ok(out);
    }
    for token in spec.split(',') {
        let token = token.trim();
        if token.is_empty() {
            continue;
        }
        let (tick, rest) = token
            .split_once(':')
            .ok_or_else(|| format!("ace-min: 脚本 token `{token}` 缺 tick"))?;
        let tick: usize = tick
            .parse()
            .map_err(|_| format!("ace-min: 脚本 tick `{tick}` 非法"))?;
        if tick == 0 {
            return Err("ace-min: 脚本 tick 从 1 起".into());
        }
        if rest == "close" {
            out.push((tick, ScriptKind::Close));
        } else if rest == "late" {
            out.push((tick, ScriptKind::Late));
        } else if rest == "reopen" {
            out.push((tick, ScriptKind::Reopen));
        } else if let Some(args) = rest.strip_prefix("click:") {
            let mut it = args.split(':');
            let (Some(i), Some(g), None) = (it.next(), it.next(), it.next()) else {
                return Err(format!("ace-min: click token `{token}` 须 i:g"));
            };
            let i: usize = i
                .parse()
                .map_err(|_| format!("ace-min: click instance `{i}` 非法"))?;
            let g: usize = g
                .parse()
                .map_err(|_| format!("ace-min: click target `{g}` 非法"))?;
            out.push((tick, ScriptKind::Click(i, g)));
        } else {
            return Err(format!("ace-min: 脚本 token `{token}` 未知"));
        }
    }
    Ok(out)
}

/// 解析后的脚本步（feature 无关层；ace-min 臂映射到 980 `ScriptStep`）。
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ScriptKind {
    Click(usize, usize),
    Close,
    Late,
    Reopen,
}

// ---------------------------------------------------------------------------
// CLI 面（feature 无关：关 = 具名拒；开 = 真链）
// ---------------------------------------------------------------------------

/// `--ace-min` 参数是否在列（main.rs 早分流用）。
pub fn requested(args: &[String]) -> bool {
    args.iter().any(|a| a == "--ace-min")
}

/// 取旗标值（缺值具名拒）。
fn next_flag_value<'a>(
    it: &mut std::slice::Iter<'a, String>,
    flag: &str,
) -> Result<&'a str, String> {
    it.next()
        .ok_or_else(|| format!("ace-min: {flag} 缺值"))
        .map(|s| s.as_str())
}

/// 执行 `--ace-min` 候选见证链。返回 Err = 具名拒（宿主以码 1 退）。
pub fn run_cli(args: &[String]) -> Result<(), String> {
    let mut id: Option<&str> = None;
    let mut instances: Option<usize> = None;
    let mut script: Option<&str> = None;
    let mut window = false;
    let mut hold_ms: Option<u64> = None;
    let mut evidence: Option<&str> = None;
    let mut it = args.iter();
    while let Some(a) = it.next() {
        match a.as_str() {
            "--ace-min" => id = Some(next_flag_value(&mut it, "--ace-min")?),
            "--ace-instances" => {
                let v = next_flag_value(&mut it, "--ace-instances")?;
                instances = Some(
                    v.parse()
                        .map_err(|_| format!("ace-min: --ace-instances `{v}` 非法"))?,
                );
            }
            "--ace-script" => script = Some(next_flag_value(&mut it, "--ace-script")?),
            "--ace-window" => window = true,
            "--hold-ms" => {
                let v = next_flag_value(&mut it, "--hold-ms")?;
                hold_ms = Some(
                    v.parse()
                        .map_err(|_| format!("ace-min: --hold-ms `{v}` 非法"))?,
                );
            }
            "--ace-evidence" => evidence = Some(next_flag_value(&mut it, "--ace-evidence")?),
            _ => {}
        }
    }
    let id = id.ok_or_else(|| "ace-min: 缺 --ace-min <id>".to_string())?;
    let spec = lookup_candidate(id)?;
    validate_spec(spec)?;
    if let Some(n) = instances {
        if n == 0 || n > 4 {
            return Err(format!("ace-min: --ace-instances {n} 越界（1..=4）"));
        }
    }
    let script_spec = script.unwrap_or(spec.script);
    let parsed = parse_script(script_spec)?;

    #[cfg(feature = "ace-min")]
    {
        let ev = imp::activate(
            spec,
            instances.unwrap_or(spec.instances),
            &parsed,
            window,
            hold_ms,
            evidence,
        )?;
        if let Some(path) = evidence {
            let json = serde_json::to_vec_pretty(&ev).map_err(|e| format!("ace-min: {e}"))?;
            if let Some(dir) = std::path::Path::new(path).parent() {
                let _ = std::fs::create_dir_all(dir);
            }
            std::fs::write(path, json).map_err(|e| format!("ace-min: 证据落盘 {path}: {e}"))?;
            eprintln!("[ace-min] evidence -> {path}");
        } else {
            eprintln!(
                "[ace-min] evidence:\n{}",
                serde_json::to_string_pretty(&ev).unwrap_or_default()
            );
        }
        Ok(())
    }
    #[cfg(not(feature = "ace-min"))]
    {
        let _ = (window, hold_ms, evidence);
        Err(format!(
            "ace-min: 本 ui-desktop 构建未开 `ace-min` feature（980 ace-bridge）；\
             A 最小候选链具名拒——请以 `cargo build -p os-desktop --features ace-min` 构建。\
             旧 VM 解释回退被 PLAN-064 合同拒绝，不执行。candidate={}",
            spec.id
        ))
    }
}

// ---------------------------------------------------------------------------
// A 最小真链（feature = ace-min）
// ---------------------------------------------------------------------------

#[cfg(feature = "ace-min")]
mod imp {
    use super::{CandidateSpec, ScriptKind};
    use auto_ui_runtime::ui::ace_node_desktop::{
        self as and, install_bridge, with_bridge, AceNodeDesktopWindow, NodeDesktopBridge,
        ScriptStep, StepOutcome,
    };
    use auto_ui_runtime::ui::iced::run_app;

    /// 激活一个候选：身份核 → SDK → 拒绝腿（零效果）→ 业务腿（headless）
    /// → 可选真窗腿 → 回收观察 → 证据。
    pub fn activate(
        spec: &CandidateSpec,
        instances: usize,
        script: &[(usize, ScriptKind)],
        window: bool,
        hold_ms: Option<u64>,
        evidence: Option<&str>,
    ) -> Result<serde_json::Value, String> {
        // ---- 身份核（缺源/错 hash/错 SDK 具名拒，零副作用）----
        let ui_root = super::resolve_ui_root()?;
        let source_path = ui_root.join(spec.source_rel);
        let source = std::fs::read(&source_path).map_err(|e| {
            format!(
                "ace-min: {} 源不可读（{}）: {e}",
                spec.id,
                source_path.display()
            )
        })?;
        let digest = super::blake3_hex(&source);
        if digest != spec.blake3 {
            return Err(format!(
                "ace-min: {} 源身份失配——blake3 期望 {} 实得 {}（来源变化使本候选失效；\
                 先具名修订注册表再重试）",
                spec.id, spec.blake3, digest
            ));
        }
        let source =
            String::from_utf8(source).map_err(|_| format!("ace-min: {} 源非 UTF-8", spec.id))?;

        // ---- SDK 身份（ACE 宿主 + ui980 provider 注册/构建，幂等）----
        auto_lang_ace_host::install().map_err(|e| format!("ace-min: ace host install: {e}"))?;
        let frames_at_start = and::NODE_DESKTOP_FRAMES.load(std::sync::atomic::Ordering::Relaxed);
        let cdylib = and::ensure_ui980_provider()
            .map_err(|e| format!("ace-min: ui980 provider（SDK 身份）: {e}"))?;
        and::install_ui980_lane().map_err(|e| format!("ace-min: ui980 lane install: {e}"))?;

        // ---- 拒绝腿（未 open 任何业务 session：零 UI 效果）----
        let probes = run_probes(spec)?;

        // ---- headless 业务腿（装载/open/受控事件/关闭回收/重开）----
        let headless = run_headless(spec, instances, script, &source)?;

        // ---- 可选真窗腿（真 Iced 窗口；记录腿 = hold-ms 存证退出）----
        let window_ev = if window {
            Some(run_window(
                spec, instances, &source, script, hold_ms, evidence,
            )?)
        } else {
            None
        };

        let frames_delta =
            and::NODE_DESKTOP_FRAMES.load(std::sync::atomic::Ordering::Relaxed) - frames_at_start;
        Ok(serde_json::json!({
            "plan": "PLAN-064/MIN-A-OS",
            "candidate": {
                "id": spec.id,
                "source": source_path.display().to_string(),
                "blake3": digest,
                "engine": spec.engine,
                "provider": spec.provider,
                "provider_cdylib": cdylib.display().to_string(),
                "instances": instances,
            },
            "ui_root": ui_root.display().to_string(),
            "probes": probes,
            "headless": headless,
            "window": window_ev,
            "frames_rendered_total": frames_delta,
        }))
    }

    /// AV 车道原始调用（969 facade；与 980 `av_call` 同一受控边界——宿主
    /// 直呼用于拒绝腿与关后回收观察，业务事件循环仍由桥的受控 tick 承载）。
    fn lane(body: &str) -> Result<i32, String> {
        let src = format!("use.rs ui980_native\nfn main() int {{ return {body} }}");
        let mut request = auto_lang_core::AceRequest::simple(&src);
        request.provider = Some(and::UI980_PROVIDER_NAME);
        let report = auto_lang_core::run_ace(auto_lang_core::AceEngineChoice::AceAV, &request);
        match report.outcome {
            auto_lang_core::AceOutcome::Success {
                value: auto_lang_core::AceValue::I32(v),
                ..
            } => Ok(v),
            auto_lang_core::AceOutcome::Denied { code, message } => {
                Err(format!("denied {code}: {message}"))
            }
            auto_lang_core::AceOutcome::Success { .. } => Err("ace-min: 车道结果非 int".into()),
            other => Err(format!("ace-min: 车道终态 {other:?}")),
        }
    }

    /// 拒绝腿：错标签/错句柄/撤权——每腿断言零 UI 效果（无窗帧），错形态
    /// 逐一具名后由正确入口恢复（错协议恢复 = V-OS-A-MIN 判据之一）。
    fn run_probes(spec: &CandidateSpec) -> Result<Vec<serde_json::Value>, String> {
        let frames = || and::NODE_DESKTOP_FRAMES.load(std::sync::atomic::Ordering::Relaxed);
        let mut probes: Vec<serde_json::Value> = Vec::new();
        let mut probe =
            |name: &'static str, refusal: String, before: usize| -> Result<(), String> {
                let frames_delta = frames() - before;
                if frames_delta != 0 {
                    return Err(format!(
                        "ace-min: 拒绝腿 {name} 产生了窗口帧（{frames_delta}）——零 UI 效果破约"
                    ));
                }
                probes.push(serde_json::json!({
                    "name": name,
                    "refusal": refusal,
                    "frames_delta": frames_delta,
                }));
                Ok(())
            };
        // ① 错车道标签（BAD_OP=-5；标签先于句柄核）。
        let before = frames();
        let wrong_tag = lane(r#"ui_poll_event("pol", 1, 0)"#);
        if wrong_tag != Ok(and::ui980_status::BAD_OP) {
            return Err(format!("ace-min: 错标签腿未得 BAD_OP：{wrong_tag:?}"));
        }
        probe(
            "wrong_lane_tag",
            format!("lane code {}", and::ui980_status::BAD_OP),
            before,
        )?;
        // ② 错句柄（UNKNOWN_HANDLE=-1）。
        let before = frames();
        let wrong_handle = lane(r#"ui_poll_event("poll", 424242, 0)"#);
        if wrong_handle != Ok(and::ui980_status::UNKNOWN_HANDLE) {
            return Err(format!(
                "ace-min: 错句柄腿未得 UNKNOWN_HANDLE：{wrong_handle:?}"
            ));
        }
        probe(
            "wrong_handle",
            format!("lane code {}", and::ui980_status::UNKNOWN_HANDLE),
            before,
        )?;
        // ③ 撤权（Denied 先于任何 dispatch）→ readmit 恢复。
        let before = frames();
        if !auto_lang_ace_host::revoke_provider(spec.provider) {
            return Err("ace-min: 撤权腿 revoke_provider 失败".into());
        }
        let revoked = lane(r#"ui_poll_event("poll", 424242, 0)"#);
        let revoked_refusal = match &revoked {
            Err(e) if e.contains("denied") => format!("{e}"),
            other => {
                auto_lang_ace_host::readmit_provider(spec.provider);
                return Err(format!(
                    "ace-min: 撤权腿未得 Denied（实得 {other:?}）——授权边界破约"
                ));
            }
        };
        if !auto_lang_ace_host::readmit_provider(spec.provider) {
            return Err("ace-min: readmit_provider 失败（撤权后无法恢复）".into());
        }
        let recovered = lane(r#"ui_poll_event("poll", 424242, 0)"#);
        if recovered != Ok(and::ui980_status::UNKNOWN_HANDLE) {
            return Err(format!(
                "ace-min: 撤权恢复腿未回到 UNKNOWN_HANDLE：{recovered:?}"
            ));
        }
        probe(
            "provider_revoked_then_readmitted",
            format!("{revoked_refusal} -> recovered UNKNOWN_HANDLE"),
            before,
        )?;
        Ok(probes)
    }

    fn to_script_steps(parsed: &[(usize, ScriptKind)]) -> Vec<(usize, ScriptStep)> {
        parsed
            .iter()
            .map(|(t, k)| {
                let step = match k {
                    ScriptKind::Click(i, g) => ScriptStep::Click {
                        instance: *i,
                        target: *g,
                    },
                    ScriptKind::Close => ScriptStep::Close,
                    ScriptKind::Late => ScriptStep::LateClick,
                    ScriptKind::Reopen => ScriptStep::Reopen,
                };
                (*t, step)
            })
            .collect()
    }

    /// headless 业务腿：load → open → 受控步进（脚本点击/close/late/reopen）
    /// → 关后回收观察（SESSION_CLOSED / late_dropped）→ 证据。
    fn run_headless(
        spec: &CandidateSpec,
        instances: usize,
        script: &[(usize, ScriptKind)],
        source: &str,
    ) -> Result<serde_json::Value, String> {
        let mut bridge = NodeDesktopBridge::load(source, spec.source_rel, instances)
            .map_err(|e| format!("ace-min: {} 装载具名拒: {e}", spec.id))?;
        bridge.install_script(to_script_steps(script));
        let session1 = bridge
            .open()
            .map_err(|e| format!("ace-min: {} open 具名拒: {e}", spec.id))?;
        // 受控步进直到脚本收口（界限 4096 防死转；越界具名拒）。
        let mut outcomes: Vec<serde_json::Value> = Vec::new();
        for _ in 0..4096 {
            if bridge.script_done() {
                break;
            }
            match bridge.step() {
                StepOutcome::Idle => {}
                StepOutcome::Dispatched {
                    instance,
                    ref msg,
                    ref states,
                } => outcomes.push(serde_json::json!({
                    "kind": "dispatched", "instance": instance, "msg": msg,
                    "states": states,
                })),
                StepOutcome::DispatchRefused(e) | StepOutcome::LaneError(e) => {
                    outcomes.push(serde_json::json!({"kind": "refused", "detail": e}));
                    return Err(format!(
                        "ace-min: {} 受控步具名拒: {e}（脚本/装载错，不静默）",
                        spec.id
                    ));
                }
            }
        }
        if !bridge.script_done() {
            return Err(format!("ace-min: {} 脚本未收口（4096 步界）", spec.id));
        }
        let opened_sessions = evidence_sessions(&bridge); // 全 session 快照（脚本段）
                                                          // 脚本段关闭观察：脚本含 close 时，late 已投向关闭 session
                                                          // （late_dropped 计数）——从证据回收，不依赖进程级 stale 靶。
        let closed_rows: Vec<&serde_json::Value> = opened_sessions
            .as_array()
            .map(|rows| rows.iter().filter(|r| r["closed"] == true).collect())
            .unwrap_or_default();
        let late_dropped_script: u64 = closed_rows
            .iter()
            .filter_map(|r| r["late_dropped"].as_u64())
            .sum();
        let stale_rejected_script: u64 = closed_rows
            .iter()
            .filter_map(|r| r["stale_rejected"].as_u64())
            .sum();
        // 宿主自有收尾关闭（A 最小会话服务生命周期）：脚本未关/重开仍开的
        // session 由宿主显式 close，随后 poll 必得 SESSION_CLOSED——
        // "关闭回收"对静态面（hello，无脚本）同样成立。
        let mut host_closed: Option<i32> = None;
        if let Some(open_id) = bridge.session {
            bridge.close_session()?;
            let rc = lane(&format!(r#"ui_poll_event("poll", {open_id}, 0)"#));
            if rc != Ok(and::ui980_status::SESSION_CLOSED) {
                return Err(format!(
                    "ace-min: 宿主关闭后 poll 未得 SESSION_CLOSED：{rc:?}"
                ));
            }
            host_closed = Some(open_id);
        }
        let _ = and::ui980_status::OK;
        // 脚本末尾 reopen（若有）：新世代 + 业务状态存活。
        let final_states = (0..instances)
            .map(|i| {
                let row = bridge.states(i).unwrap_or_default();
                (
                    format!("i{i}"),
                    row.into_iter()
                        .map(|(n, v)| (n, format!("{v:?}")))
                        .collect::<Vec<_>>(),
                )
            })
            .collect::<Vec<_>>();
        let evidence = bridge.evidence();
        let reopen_ev = evidence
            .get("sessions")
            .and_then(|s| s.as_array())
            .map(|rows| {
                rows.iter()
                    .max_by_key(|r| r["id"].as_i64().unwrap_or(0))
                    .cloned()
                    .unwrap_or(serde_json::Value::Null)
            })
            .unwrap_or(serde_json::Value::Null);
        Ok(serde_json::json!({
            "session_first": session1,
            "sessions": opened_sessions,
            "outcomes": outcomes,
            "script_close": {
                "closed_sessions": closed_rows.iter().map(|r| r["id"].clone()).collect::<Vec<_>>(),
                "late_dropped": late_dropped_script,
                "stale_rejected": stale_rejected_script,
            },
            "host_close": {
                "closed_session": host_closed,
                "poll_after_close": and::ui980_status::SESSION_CLOSED,
            },
            "reopen_last": reopen_ev,
            "final_states": final_states,
            "engine_face": bridge.engine_face(),
            "buttons": bridge.buttons().iter().map(|b| serde_json::json!({
                "label": b.label, "msg": b.msg,
            })).collect::<Vec<_>>(),
            "errors": bridge.errors,
        }))
    }

    fn evidence_sessions(bridge: &NodeDesktopBridge) -> serde_json::Value {
        bridge
            .evidence()
            .get("sessions")
            .cloned()
            .unwrap_or_default()
    }

    /// 真窗腿：新桥（新世代）→ open → install_bridge → `run_app` 真窗
    /// （真 Iced 输入/tick）；hold-ms = 记录腿到点存证退出；无 hold =
    /// 手动关窗（交互档）。关窗后桥 close + 证据。
    fn run_window(
        spec: &CandidateSpec,
        instances: usize,
        source: &str,
        script: &[(usize, ScriptKind)],
        hold_ms: Option<u64>,
        evidence_path: Option<&str>,
    ) -> Result<serde_json::Value, String> {
        let mut bridge = NodeDesktopBridge::load(source, spec.source_rel, instances)
            .map_err(|e| format!("ace-min: 真窗腿装载具名拒: {e}"))?;
        bridge.install_script(to_script_steps(script));
        let session = bridge
            .open()
            .map_err(|e| format!("ace-min: 真窗腿 open 具名拒: {e}"))?;
        install_bridge(bridge);
        let before = and::NODE_DESKTOP_FRAMES.load(std::sync::atomic::Ordering::Relaxed);
        if let Some(ms) = hold_ms {
            // 记录腿：到点存证退出（980 window exe 同型）。存证线程关 session
            // → 汇总证据 → 落盘 evidence_path（有则）→ exit(0)。
            let spec = spec.clone();
            let path = evidence_path.map(std::string::ToString::to_string);
            std::thread::spawn(move || {
                std::thread::sleep(std::time::Duration::from_millis(ms));
                let sessions = with_bridge(|b| {
                    let _ = b.close_session();
                    b.evidence()
                })
                .map(|ev| ev.get("sessions").cloned().unwrap_or_default());
                let frames =
                    and::NODE_DESKTOP_FRAMES.load(std::sync::atomic::Ordering::Relaxed) - before;
                let full = serde_json::json!({
                    "plan": "PLAN-064/MIN-A-OS",
                    "candidate": {
                        "id": spec.id,
                        "source_rel": spec.source_rel,
                        "blake3": spec.blake3,
                        "engine": spec.engine,
                        "provider": spec.provider,
                        "instances": instances,
                    },
                    "window": {
                        "real_window": true,
                        "frames_rendered": frames,
                        "hold_ms_exit": ms,
                        "sessions": sessions,
                    },
                    "note": "hold-ms 记录腿：真窗 tick 驱动脚本 → 到点关 session 存证退出",
                });
                if let Some(p) = &path {
                    let json = serde_json::to_vec_pretty(&full).unwrap_or_default();
                    if let Some(dir) = std::path::Path::new(p).parent() {
                        let _ = std::fs::create_dir_all(dir);
                    }
                    let _ = std::fs::write(p, json);
                    eprintln!("[ace-min] hold-ms 到点，存证 -> {p}（frames={frames}）");
                } else {
                    eprintln!(
                        "[ace-min] hold-ms 到点，存证（frames={frames}）：\n{}",
                        serde_json::to_string_pretty(&full).unwrap_or_default()
                    );
                }
                std::process::exit(0);
            });
        }
        let run =
            run_app::<AceNodeDesktopWindow>().map_err(|e| format!("ace-min: 真窗运行失败: {e:?}"));
        let frames = and::NODE_DESKTOP_FRAMES.load(std::sync::atomic::Ordering::Relaxed) - before;
        let run = run?;
        let _ = session;
        Ok(serde_json::json!({
            "frames_rendered": frames,
            "real_window": true,
            "run": format!("{run:?}"),
        }))
    }

    #[cfg(test)]
    mod tests {
        use super::super::*;
        use super::*;

        #[test]
        fn p064_candidate_registry_freeze_is_well_formed() {
            assert_eq!(CANDIDATES.len(), 2, "A 最小只冻结 Hello 与 Counter");
            let ids: Vec<_> = CANDIDATES.iter().map(|c| c.id).collect();
            assert_eq!(
                ids.len(),
                std::collections::HashSet::<_>::from_iter(ids.iter().copied()).len(),
                "稳定 id 须唯一"
            );
            for c in CANDIDATES {
                validate_spec(c).unwrap_or_else(|e| panic!("{e}"));
                assert_eq!(c.engine, "av");
                assert_eq!(c.provider, "p980_ui980_native");
                parse_script(c.script).unwrap_or_else(|e| panic!("{}: {e}", c.id));
            }
        }

        #[test]
        fn p064_blake3_known_vector() {
            assert_eq!(
                blake3_hex(b""),
                "af1349b9f5f9a1a6a0404dea36dcc9499bcb25c9adc112b7cc9a93cae41f3262"
            );
        }

        #[test]
        fn p064_blake3_pins_match_frozen_sources() {
            let root = resolve_ui_root().expect("auto-ui 根可达");
            for c in CANDIDATES {
                let bytes = std::fs::read(root.join(c.source_rel))
                    .unwrap_or_else(|e| panic!("{}: {e}", c.source_rel));
                let actual = blake3_hex(&bytes);
                assert_eq!(actual, c.blake3, "{} 来源身份漂移——先具名修订注册表", c.id);
            }
        }

        #[test]
        fn p064_unknown_engine_refused() {
            let mut c = CANDIDATES[0].clone();
            let spec = CandidateSpec { engine: "vm", ..c };
            let err = validate_spec(&spec).unwrap_err();
            assert!(err.contains("非 ACE AV"), "{err}");
            let _ = &mut c;
        }

        #[test]
        fn p064_provider_mismatch_refused() {
            let spec = CandidateSpec {
                provider: "other_sdk",
                ..CANDIDATES[0].clone()
            };
            let err = validate_spec(&spec).unwrap_err();
            assert!(err.contains("SDK 身份"), "{err}");
        }

        #[test]
        fn p064_instances_bound_refused() {
            for n in [0usize, 5] {
                let spec = CandidateSpec {
                    instances: n,
                    ..CANDIDATES[0].clone()
                };
                let err = validate_spec(&spec).unwrap_err();
                assert!(err.contains("越界"), "{err}");
            }
        }

        #[test]
        fn p064_script_parser_accepts_and_refuses() {
            let ok = parse_script("1:click:0:2, 2:close,3:late,4:reopen").unwrap();
            assert_eq!(
                ok,
                vec![
                    (1, ScriptKind::Click(0, 2)),
                    (2, ScriptKind::Close),
                    (3, ScriptKind::Late),
                    (4, ScriptKind::Reopen),
                ]
            );
            assert!(parse_script("0:click:0:0").is_err(), "tick 从 1");
            assert!(parse_script("1:boom").is_err());
            assert!(parse_script("1:click:0").is_err());
            assert!(parse_script("x:close").is_err());
        }

        #[test]
        fn p064_unknown_candidate_refused() {
            let err = lookup_candidate("nope.app").unwrap_err();
            assert!(err.contains("未注册候选"), "{err}");
        }

        #[test]
        fn p064_cli_requires_id() {
            let err = run_cli(&["ui-desktop".into()]).unwrap_err();
            assert!(err.contains("缺 --ace-min"), "{err}");
        }

        /// 双 App 同宿主进程：会话单调、关闭 session 不复活、业务不串
        /// （B-HOST-I 双App反例的进程内形态；跨进程形态见 tests/ace_min.rs）。
        #[test]
        fn p064_dual_app_one_host_process_isolation() {
            let root = resolve_ui_root().expect("auto-ui 根可达");
            let read = |c: &CandidateSpec| -> String {
                let bytes = std::fs::read(root.join(c.source_rel)).expect("源可读");
                String::from_utf8(bytes).expect("UTF-8")
            };
            let (hello, counter) = (CANDIDATES[0].clone(), CANDIDATES[1].clone());
            let h1 = run_headless(&hello, 1, &[], &read(&hello)).expect("hello 腿");
            let script = parse_script(counter.script).expect("counter 脚本");
            let h2 = run_headless(&counter, 2, &script, &read(&counter)).expect("counter 腿");
            let s1 = h1["session_first"].as_i64().unwrap();
            let s2 = h2["session_first"].as_i64().unwrap();
            assert!(s2 > s1, "同宿主会话 id 单调（{s1} -> {s2}）");
            // hello 已关闭 session 不被 counter 复活：poll 仍 SESSION_CLOSED。
            let hello_closed = h1["host_close"]["closed_session"].as_i64().unwrap();
            assert_eq!(
                lane(&format!(r#"ui_poll_event("poll", {hello_closed}, 0)"#)),
                Ok(and::ui980_status::SESSION_CLOSED),
                "前 App 关闭 session 被复活——隔离破约"
            );
            let h1_text = h1.to_string();
            assert!(!h1_text.contains("count"), "hello 静面无 counter 字段");
            assert!(
                h2.to_string().contains("Int(2)"),
                "counter 自 0 起轨不受 hello 影响"
            );
        }
    }
}
