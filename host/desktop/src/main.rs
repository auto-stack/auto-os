//! AutoOS 桌面宿主入口（PLAN-876 T-02，自 auto-lang `examples/ui_desktop.rs` 迁入）。
//!
//! 形态：一个 OS 窗口承载多 App 虚拟窗口（拖拽/缩放/聚焦/× 关闭）+ 全屏桌面
//! shell（任务栏/热键/注册表启动）。渲染/会话/投影由 `auto-ui-runtime` 提供，
//! 本 crate 只做 **AutoOS 产品装配**：注册表目录、storage 政策、rqhost 装配、
//! boot 自检与足迹桩。
//!
//! 运行：
//! ```text
//! cd D:/autostack/auto-os            # apps 兄弟探测依赖 CWD
//! cargo run -p os-desktop -- --apps-dir apps
//! ```
//! 全屏：追加 `--fullscreen`。
//! 兼容 env：`AUTO_OS_ROOT`（apps 缺省根）、`AUTO_VM_STORAGE_FILE`（已设不覆盖）、
//! `AUTO_DESKTOP_RQHOST=1`（孵化合成器 daemon）、`AUTO_DESKTOP_HOLE=1`（真洞）。

use auto_ui_runtime::ui::iced::{
    run_dynamic_desktop_fullscreen, run_dynamic_desktop_with_options, DesktopOptions,
};
use std::path::PathBuf;

mod health;

/// apps 目录缺省解析序：`AUTO_OS_ROOT/apps` → `CWD/apps` → 仓根（编译期锚）。
///
/// 与旧 example 的差别：旧缺省指向框架仓 `examples/ui`（教学 demo 集），
/// 迁入 auto-os 后缺省必须是**本仓 apps**（AutoOS 产品集）。
fn resolve_apps_dir() -> (PathBuf, &'static str) {
    if let Some(root) = std::env::var_os("AUTO_OS_ROOT") {
        return (PathBuf::from(root).join("apps"), "auto_os_root");
    }
    if let Ok(cwd) = std::env::current_dir() {
        let candidate = cwd.join("apps");
        if candidate.is_dir() {
            return (candidate, "cwd");
        }
    }
    // panel: <repo>/host/desktop → <repo>
    let repo = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("..");
    (repo.join("apps"), "repo_root")
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    auto_lang_core::set_ui_host_install_hook(auto_ui_runtime::install_ui_host_capabilities);
    auto_ui_runtime::install_ui_host_capabilities();
    // 桌面轨宿主 log crate trap：本入口无 logger，装环即全量收 error/warn/info。
    auto_ui_runtime::ui::syslog::install_host_logger();
    // 桌面在场标记：App 可用 Env.get 探测自身是否运行在虚拟桌面内。
    std::env::set_var("AUTO_UI_IN_DESKTOP", "1");
    // storage 缺省对齐既有桌面（确定性 per-user 文件）；已设 env（脚本/测试隔离）
    // 不覆盖——否则直接 exec exe 时会落到按 CWD 哈希的临时库，快捷方式/shell
    // 配置全部"消失"。
    if std::env::var_os("AUTO_VM_STORAGE_FILE").is_none() {
        if let Some(home) = std::env::var_os("USERPROFILE").or_else(|| std::env::var_os("HOME")) {
            let path = PathBuf::from(home)
                .join(".config")
                .join("autoos")
                .join("desktop-storage.json");
            if let Some(dir) = path.parent() {
                let _ = std::fs::create_dir_all(dir);
            }
            std::env::set_var("AUTO_VM_STORAGE_FILE", &path);
        }
    }

    let args: Vec<String> = std::env::args().collect();
    let fullscreen = args.iter().any(|a| a == "--fullscreen");
    let explicit_apps_dir = args
        .iter()
        .position(|a| a == "--apps-dir")
        .and_then(|i| args.get(i + 1))
        .map(PathBuf::from);
    let (apps_dir, apps_dir_src) = match explicit_apps_dir {
        Some(dir) => (dir, "cli"),
        None => resolve_apps_dir(),
    };

    // 足迹桩（死亡判位面）：start/alive/exit 三行 + panic backtrace。
    install_traces(fullscreen, &apps_dir);

    // boot 自检（只读）：一次说清注册表/exe 产物/空壳 app/storage/rqhost 可达性。
    health::report(&apps_dir, apps_dir_src);

    // boot 组件表恒空（PLAN-043）：窗口全部经注册表 launch 链。
    let comps = Vec::new();
    let opts = DesktopOptions {
        fullscreen,
        apps_dir: Some(apps_dir),
        hole_mode: std::env::var("AUTO_DESKTOP_HOLE").as_deref() == Ok("1"),
        ..Default::default()
    };
    let result = if fullscreen {
        run_dynamic_desktop_fullscreen(comps, opts)
    } else {
        run_dynamic_desktop_with_options(comps, opts)
    };
    match &result {
        Ok(_) => eprintln!("[ui-desktop] exit ok (code 0)"),
        Err(e) => eprintln!("[ui-desktop] exit err (code 1): {e}"),
    }
    result.map(|_| ())
}

fn install_traces(fullscreen: bool, apps_dir: &std::path::Path) {
    let boot = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0);
    eprintln!(
        "[ui-desktop] start pid={} fullscreen={fullscreen} apps_dir={} t={boot}",
        std::process::id(),
        apps_dir.display()
    );
    std::panic::set_hook(Box::new(|info| {
        eprintln!(
            "[ui-desktop] panic {info}\nbacktrace:\n{}",
            std::backtrace::Backtrace::force_capture()
        );
    }));
    std::thread::spawn(move || loop {
        std::thread::sleep(std::time::Duration::from_secs(30));
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_secs())
            .unwrap_or(0);
        eprintln!("[ui-desktop] alive pid={} t={now}", std::process::id());
    });
}
