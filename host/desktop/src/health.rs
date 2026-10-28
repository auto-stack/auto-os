//! boot 期只读自检（PLAN-876 T-04）。
//!
//! 目的：把"注册表装了几条、哪些 app 声明了原生产物但产物不在、哪些 app 目录
//! 是空壳（子模块未初始化/工作区被清空）、shell pack 从哪来、桌面 storage 里
//! 有没有失效 id、`auto` CLI 是否可达"一次说清，而不是让用户双击试错。
//!
//! 边界：只读——不写 storage、不改注册表、不构建任何产物。输出两形态：
//! 一行 `[desktop-health] key=value …`（脚本可 grep）+ 零到多行
//! `[desktop-health] hint=…`（只在有缺口时出现，内容必须是可执行动作）。

use std::path::{Path, PathBuf};

/// 自检快照（纯数据，便于单测与脚本断言）。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Snapshot {
    pub apps_dir: String,
    pub apps_dir_src: &'static str,
    /// 注册表条目数（与 `[session] app registry: N entries` 同源同算法）。
    pub registry: usize,
    /// 桌面可见条目数（pac `desktop:` 过滤后）。
    pub desktop_visible: usize,
    /// 声明了 `desktop_exe` 但产物不在的 app id。
    pub declared_exe_missing: Vec<String>,
    /// 空壳 app 目录数（无 `pac.at`，且除 `.git` 外无内容 = 子模块未初始化）。
    pub apps_empty: usize,
    pub shell_pack: Option<String>,
    pub shell_pack_src: &'static str,
    pub storage: Option<String>,
    /// storage `shell.desktop.icons` 里无注册表落点的 id 数。
    pub orphan_icons: usize,
    /// `auto` CLI 解析结果（PATH/同目录/同目录上两级）。
    pub auto_cli: Option<String>,
}

impl Snapshot {
    /// 单行形态（脚本 grep 面）。
    pub fn to_line(&self) -> String {
        format!(
            "[desktop-health] apps_dir={} apps_dir_src={} registry={} desktop_visible={} declared_exe_missing={} apps_empty={} shell_pack={} shell_pack_src={} storage={} orphan_icons={} auto_cli={}",
            self.apps_dir,
            self.apps_dir_src,
            self.registry,
            self.desktop_visible,
            join_or_dash(&self.declared_exe_missing),
            self.apps_empty,
            self.shell_pack.as_deref().unwrap_or("-"),
            self.shell_pack_src,
            self.storage.as_deref().unwrap_or("-"),
            self.orphan_icons,
            self.auto_cli.as_deref().unwrap_or("missing"),
        )
    }

    /// 缺口建议（空 = 全绿，不输出 hint 行）。
    pub fn hints(&self) -> Vec<String> {
        let mut out = Vec::new();
        if !self.declared_exe_missing.is_empty() {
            out.push(format!(
                "declared desktop_exe missing for [{}]: run `auto build -r rust` in each app, or set \"launch\": \"vm\" in apps.manifest",
                self.declared_exe_missing.join(",")
            ));
        }
        if self.apps_empty > 0 {
            out.push(format!(
                "{} app dir(s) empty (no pac.at, only .git or nothing): run `git submodule update --init --recursive`",
                self.apps_empty
            ));
        }
        if self.auto_cli.is_none() {
            out.push(
                "auto CLI not found: run `cargo build -p auto` (needed for rqhost daemon and native app spawn)"
                    .to_string(),
            );
        }
        if self.orphan_icons > 0 {
            out.push(format!(
                "desktop storage has {} id(s) without a registry entry: clean them from `shell.desktop.icons` or add an apps.manifest entry",
                self.orphan_icons
            ));
        }
        out
    }
}

/// 采样（只读）。
pub fn snapshot(apps_dir: &Path, apps_dir_src: &'static str) -> Snapshot {
    use auto_ui_runtime::ui::app_registry;
    let entries = app_registry::aggregate_scan(
        apps_dir,
        &app_registry::host_extra_roots(),
        &app_registry::ScanOptions::default(),
    );
    let desktop_visible = entries.iter().filter(|e| e.desktop_visible).count();
    let declared_exe_missing: Vec<String> = entries
        .iter()
        .filter_map(|e| {
            let decl = e.desktop_exe.as_deref()?;
            (!e.app_root().join(decl).is_file()).then(|| e.id.clone())
        })
        .collect();

    let apps_empty = count_empty_app_dirs(apps_dir);
    let shell = auto_ui_runtime::ui::shell::resolve_shell_pack_dir();
    let shell_pack_src = if std::env::var_os("AUTO_SHELL_PACK").is_some() {
        "env"
    } else if shell.is_some() {
        "resolved"
    } else {
        "embedded"
    };
    let storage = std::env::var_os("AUTO_VM_STORAGE_FILE").map(|p| PathBuf::from(p));
    let orphan_icons = orphan_icon_count(&entries);
    let auto_cli = auto_ui_runtime::ui::desktop_protocol::rqhost::resolve_auto_cli();

    Snapshot {
        apps_dir: apps_dir.display().to_string(),
        apps_dir_src,
        registry: entries.len(),
        desktop_visible,
        declared_exe_missing,
        apps_empty,
        shell_pack: shell.map(|p| p.display().to_string()),
        shell_pack_src,
        storage: storage.map(|p| p.display().to_string()),
        orphan_icons,
        auto_cli: auto_cli.map(|p| p.display().to_string()),
    }
}

/// 打印自检行 + 缺口建议。
pub fn report(apps_dir: &Path, apps_dir_src: &'static str) {
    let snap = snapshot(apps_dir, apps_dir_src);
    println!("{}", snap.to_line());
    for hint in snap.hints() {
        println!("[desktop-health] hint={hint}");
    }
}

/// 空壳 app 目录：存在但无 `pac.at`，且除 `.git` 外没有内容。
///
/// 这正是"子模块未初始化"与"工作区被清空"两种现场的共同签名；有 `src/` 之类
/// 内容的非 app 目录（如 `common/`）不会被计入。
fn count_empty_app_dirs(apps_dir: &Path) -> usize {
    let Ok(rd) = std::fs::read_dir(apps_dir) else {
        return 0;
    };
    rd.filter_map(|e| e.ok())
        .filter(|e| e.path().is_dir())
        .filter(|e| {
            let dir = e.path();
            if dir.join("pac.at").is_file() {
                return false;
            }
            match std::fs::read_dir(&dir) {
                Ok(inner) => inner
                    .filter_map(|i| i.ok())
                    .all(|i| i.file_name() == std::ffi::OsStr::new(".git")),
                Err(_) => false,
            }
        })
        .count()
}

/// storage `shell.desktop.icons` 中无注册表落点的 id 数。
fn orphan_icon_count(entries: &[auto_ui_runtime::ui::app_registry::AppRegistryEntry]) -> usize {
    let Some(icons) = auto_lang_core::vm::ffi::stdlib::storage_host_read("shell.desktop.icons")
    else {
        return 0;
    };
    let known: std::collections::HashSet<&str> = entries.iter().map(|e| e.id.as_str()).collect();
    icons
        .split(',')
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .filter(|id| !known.contains(id))
        .count()
}

fn join_or_dash(items: &[String]) -> String {
    if items.is_empty() {
        "-".to_string()
    } else {
        items.join(",")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn tmpdir(tag: &str) -> PathBuf {
        let dir = std::env::temp_dir().join(format!(
            "p876-health-{tag}-{}-{}",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .map(|d| d.as_nanos())
                .unwrap_or(0)
        ));
        std::fs::create_dir_all(&dir).unwrap();
        dir
    }

    /// V-03 判据：空壳目录（只有 .git）与"有内容的非 app 目录"必须被区分。
    #[test]
    fn empty_app_dir_signature_is_git_only() {
        let root = tmpdir("empty");
        std::fs::create_dir_all(root.join("apps/auto-calc")).unwrap();
        std::fs::write(root.join("apps/auto-calc/.git"), "gitdir: x").unwrap();
        std::fs::create_dir_all(root.join("apps/common/settings")).unwrap();
        std::fs::write(root.join("apps/common/settings/pac.at"), "name: \"s\"").unwrap();
        std::fs::create_dir_all(root.join("apps/auto-notes/src")).unwrap();
        assert_eq!(count_empty_app_dirs(&root.join("apps")), 1, "只数只有 .git 的 app 目录");
        std::fs::remove_dir_all(&root).ok();
    }

    /// 行形态可被脚本按 key=value 解析（不含空格，逗号分隔清单）。
    #[test]
    fn line_is_key_value_and_grep_able() {
        let snap = Snapshot {
            apps_dir: "D:/x/apps".into(),
            apps_dir_src: "cli",
            registry: 30,
            desktop_visible: 29,
            declared_exe_missing: vec!["036-tetris".into(), "038-minesweeper".into()],
            apps_empty: 0,
            shell_pack: Some("D:/x/shell".into()),
            shell_pack_src: "resolved",
            storage: Some("C:/u/.config/autoos/desktop-storage.json".into()),
            orphan_icons: 2,
            auto_cli: None,
        };
        let line = snap.to_line();
        assert!(line.starts_with("[desktop-health] "));
        assert!(line.contains("registry=30"));
        assert!(line.contains("declared_exe_missing=036-tetris,038-minesweeper"));
        assert!(line.contains("auto_cli=missing"));
        let hints = snap.hints();
        assert_eq!(hints.len(), 3, "缺产物/auto CLI/失效图标各一条建议：{hints:?}");
        assert!(hints.iter().any(|h| h.contains("auto build -r rust")));
        assert!(hints.iter().any(|h| h.contains("cargo build -p auto")));
    }

    /// 全绿时不出 hint（避免噪声淹没真实缺口）。
    #[test]
    fn no_hints_when_clean() {
        let snap = Snapshot {
            apps_dir: "D:/x/apps".into(),
            apps_dir_src: "cli",
            registry: 30,
            desktop_visible: 29,
            declared_exe_missing: vec![],
            apps_empty: 0,
            shell_pack: None,
            shell_pack_src: "embedded",
            storage: None,
            orphan_icons: 0,
            auto_cli: Some("D:/x/auto.exe".into()),
        };
        assert!(snap.hints().is_empty());
    }
}
