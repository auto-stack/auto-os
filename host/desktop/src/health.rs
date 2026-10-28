//! boot 期只读自检（PLAN-876 T-04）。
//!
//! 目的：把"注册表装了几条、哪些 app 声明了原生产物但产物不在、哪些 app 目录
//! 是空壳（子模块未初始化/工作区被清空）、shell pack 从哪来、桌面 storage 里
//! 有没有失效 id、`auto` CLI 是否可达"一次说清，而不是让用户双击试错。
//!
//! 边界：只读——不写 storage、不改注册表、不构建任何产物。输出两形态：
//! 一行 `[desktop-health] key=value …`（脚本可 grep）+ 零到多行
//! `[desktop-health] hint=…`（只在有缺口时出现，内容必须是可执行动作）。

use std::collections::HashSet;
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
    /// apps.manifest 的 repo 条目里缺 `pac.at` 的数量（注册表跳过面）。
    pub skip_no_pac: usize,
    /// 声明了 `desktop_exe` 但产物不在的 app id。
    pub declared_exe_missing: Vec<String>,
    /// 空壳 app 目录数（无 `pac.at`，且除 `.git` 外无内容 = 子模块未初始化）。
    pub apps_empty: usize,
    pub shell_pack: Option<String>,
    pub shell_pack_src: &'static str,
    pub storage: Option<String>,
    /// storage `shell.desktop.icons` 里"既无注册表落点、也不在 apps.manifest"
    /// 的 id 数（REV2：manifest 落点也算已知，避免子模块未初始化时误报）。
    pub orphan_icons: usize,
    /// `auto` CLI 解析结果（env/PATH/同目录及上两级）。
    pub auto_cli: Option<String>,
}

impl Snapshot {
    /// 单行形态（脚本 grep 面）。
    pub fn to_line(&self) -> String {
        format!(
            "[desktop-health] apps_dir={} apps_dir_src={} registry={} desktop_visible={} skip=no_pac_at:{} declared_exe_missing={} apps_empty={} shell_pack={} shell_pack_src={} storage={} orphan_icons={} auto_cli={}",
            self.apps_dir,
            self.apps_dir_src,
            self.registry,
            self.desktop_visible,
            self.skip_no_pac,
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
    ///
    /// REV2：`orphan_icons` 的"清理"建议只在**没有空壳 app 目录**时给——
    /// 子模块未初始化的检出里，合法 id 会因扫描不到而像 orphan，照 hint 删除
    /// 会丢掉桌面快捷方式。
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
        if self.orphan_icons > 0 && self.apps_empty == 0 {
            out.push(format!(
                "desktop storage has {} id(s) with neither a registry entry nor an apps.manifest entry: clean them from `shell.desktop.icons`",
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

    // manifest 落点面（REV2/F-876-R2-01）：**直读 apps.manifest**——registry 的
    // `manifest_repo_roots` 会在返回前把"缺 pac.at"的条目过滤掉，用它做统计源
    // 会让 `skip` 结构性恒 0、并让合法 id 在 known 集里缺席。
    let manifest_parent = apps_dir.parent().unwrap_or(apps_dir).to_path_buf();
    let (skip_no_pac, manifest_known) = app_registry::resolve_os_manifest_root(&manifest_parent)
        .map(|root| manifest_state(&root))
        .unwrap_or((0, HashSet::new()));

    let apps_empty = count_empty_app_dirs(apps_dir);
    let shell = auto_ui_runtime::ui::shell::resolve_shell_pack_dir();
    let shell_pack_src = if std::env::var_os("AUTO_SHELL_PACK").is_some() {
        "env"
    } else if shell.is_some() {
        "resolved"
    } else {
        "embedded"
    };
    let storage = std::env::var_os("AUTO_VM_STORAGE_FILE").map(PathBuf::from);
    let mut known: HashSet<String> = entries.iter().map(|e| e.id.clone()).collect();
    known.extend(manifest_known);
    let orphan_icons = auto_lang_core::vm::ffi::stdlib::storage_host_read("shell.desktop.icons")
        .map(|icons| orphan_ids(&icons, &known).len())
        .unwrap_or(0);
    let auto_cli = auto_ui_runtime::ui::desktop_protocol::rqhost::resolve_auto_cli();

    Snapshot {
        apps_dir: apps_dir.display().to_string(),
        apps_dir_src,
        registry: entries.len(),
        desktop_visible,
        skip_no_pac,
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

/// 直读 `apps.manifest`：返回（缺 `pac.at` 的 active repo 条目数，全部 active id）。
///
/// 刻意不用 `app_registry::manifest_repo_roots`——它在返回前就把缺 `pac.at`
/// 的条目过滤掉了；把它当统计源会让 `skip` 结构性恒 0，并让 known 集缺 id
/// （REV2/F-876-R2-01 实证）。
fn manifest_state(manifest_root: &Path) -> (usize, HashSet<String>) {
    let Ok(raw) = std::fs::read_to_string(manifest_root.join("apps.manifest")) else {
        return (0, HashSet::new());
    };
    let Ok(parsed) = serde_json::from_str::<serde_json::Value>(&raw) else {
        return (0, HashSet::new());
    };
    let mut skip = 0usize;
    let mut known = HashSet::new();
    for app in parsed
        .get("apps")
        .and_then(|a| a.as_array())
        .into_iter()
        .flatten()
    {
        let Some(id) = app.get("id").and_then(|i| i.as_str()) else {
            continue;
        };
        if app
            .get("status")
            .and_then(|s| s.as_str())
            .unwrap_or("active")
            != "active"
        {
            continue;
        }
        known.insert(id.to_string());
        if app.get("kind").and_then(|k| k.as_str()).unwrap_or("repo") != "repo" {
            continue; // local 形态由容器展开，不算 skip
        }
        if let Some(repo) = app.get("repo").and_then(|r| r.as_str()) {
            if !manifest_root.join(repo).join("pac.at").is_file() {
                skip += 1;
            }
        }
    }
    (skip, known)
}

/// 空壳 app 目录：存在但无 `pac.at`，且除 `.git` 外没有内容。///
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

/// storage id 里既无注册表落点、也无 apps.manifest 落点的清单。
fn orphan_ids(icons: &str, known: &HashSet<String>) -> Vec<String> {
    icons
        .split(',')
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .filter(|id| !known.contains(*id))
        .map(str::to_string)
        .collect()
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

    fn base_snapshot() -> Snapshot {
        Snapshot {
            apps_dir: "D:/x/apps".into(),
            apps_dir_src: "cli",
            registry: 30,
            desktop_visible: 29,
            skip_no_pac: 0,
            declared_exe_missing: vec![],
            apps_empty: 0,
            shell_pack: None,
            shell_pack_src: "embedded",
            storage: None,
            orphan_icons: 0,
            auto_cli: Some("D:/x/auto.exe".into()),
        }
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
        assert_eq!(
            count_empty_app_dirs(&root.join("apps")),
            1,
            "只数只有 .git 的 app 目录"
        );
        std::fs::remove_dir_all(&root).ok();
    }

    /// 行形态：**完整键集**必须都在（REV2：缺键要能被抓，不再只断言子串）。
    #[test]
    fn line_carries_full_key_set() {
        let mut snap = base_snapshot();
        snap.declared_exe_missing = vec!["036-tetris".into(), "038-minesweeper".into()];
        snap.orphan_icons = 2;
        snap.auto_cli = None;
        let line = snap.to_line();
        assert!(line.starts_with("[desktop-health] "));
        for key in [
            "apps_dir=",
            "apps_dir_src=",
            "registry=",
            "desktop_visible=",
            "skip=no_pac_at:",
            "declared_exe_missing=",
            "apps_empty=",
            "shell_pack=",
            "shell_pack_src=",
            "storage=",
            "orphan_icons=",
            "auto_cli=",
        ] {
            assert!(line.contains(key), "行缺键 {key}: {line}");
        }
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
        assert!(base_snapshot().hints().is_empty());
    }

    /// V-03（REV2）：空壳 app 目录要给出"跑子模块初始化"的可执行建议。
    #[test]
    fn hint_when_apps_empty() {
        let mut snap = base_snapshot();
        snap.apps_empty = 1;
        let hints = snap.hints();
        assert!(
            hints.iter().any(|h| h.contains("git submodule update --init --recursive")),
            "空壳目录要提示子模块初始化：{hints:?}"
        );
    }

    /// REV2（F-876-R2-01）：`skip` 必须反映**非零**真实缺口——直读 manifest，
    /// 不受 registry 的 pac.at 过滤影响；known 集含全部 active id。
    #[test]
    fn manifest_state_counts_missing_pac_at() {
        let root = tmpdir("manifest");
        std::fs::create_dir_all(root.join("apps/present")).unwrap();
        std::fs::write(root.join("apps/present/pac.at"), "name: \"p\"").unwrap();
        std::fs::create_dir_all(root.join("apps/missing")).unwrap();
        let manifest = r#"{
          "apps": [
            {"id": "present-app", "repo": "apps/present", "kind": "repo", "status": "active"},
            {"id": "missing-app", "repo": "apps/missing", "kind": "repo", "status": "active"},
            {"id": "local-app", "repo": "apps/none", "kind": "local", "status": "active"},
            {"id": "gone-app", "repo": "apps/gone", "kind": "repo", "status": "retired"}
          ]
        }"#;
        std::fs::write(root.join("apps.manifest"), manifest).unwrap();
        let (skip, known) = manifest_state(&root);
        assert_eq!(skip, 1, "只有 active repo 且缺 pac.at 的条目计入 skip");
        assert!(known.contains("present-app"));
        assert!(known.contains("missing-app"), "缺 pac.at 也要进 known（否则误判 orphan）");
        assert!(known.contains("local-app"));
        assert!(!known.contains("gone-app"), "非 active 不入 known");
        std::fs::remove_dir_all(&root).ok();
    }

    /// REV2（F-876-R-02）：manifest 落点也算已知——子模块未初始化时不得把
    /// 合法 id 判成 orphan，且此场景**不许**给"清理 storage"的破坏性建议。
    #[test]
    fn orphan_requires_absence_from_registry_and_manifest() {
        let mut known: HashSet<String> = HashSet::new();
        known.insert("011-calculator".into());
        known.insert("038-minesweeper".into()); // 只在 manifest 落点
        let icons = "011-calculator,038-minesweeper,041-auto-edit";
        assert_eq!(
            orphan_ids(icons, &known),
            vec!["041-auto-edit".to_string()],
            "manifest 落点算已知，只有两边都没有的才算 orphan"
        );

        let mut snap = base_snapshot();
        snap.orphan_icons = 24;
        snap.apps_empty = 24;
        assert!(
            !snap.hints().iter().any(|h| h.contains("clean them")),
            "空壳目录在场时不得建议清理 storage（会丢快捷方式）"
        );
    }
}
