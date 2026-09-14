# PLAN-015 T7 实证记录（2026-09-14）

## VM 轨双 locale 窗标题实机（worktree 二进制 `target/debug/auto.exe`，样本 041-auto-edit）

| locale | stdout | 判定 |
|---|---|---|
| 缺省（未设 AUTO_LOCALE） | `VM window title: 编辑器 (from pac.at)` | zh 链命中 title_zh ✅ |
| `AUTO_LOCALE=en` | `VM window title: AutoEdit (from pac.at)` | en 链取 title ✅ |
| `AUTO_LOCALE=zh_CN` | `VM window title: 编辑器 (from pac.at)` | zh 前缀命中 ✅ |

命令（auto-lang 组 worktree）：

```
cd .wt/os-015/auto-lang/examples/ui/041-auto-edit
timeout 25 ../../../target/debug/auto.exe run -r vm          # 缺省 zh
AUTO_LOCALE=en  timeout 25 ../../../target/debug/auto.exe run -r vm
AUTO_LOCALE=zh_CN timeout 25 ../../../target/debug/auto.exe run -r vm
```

## 定点测试（worktree）

- `cargo t -p auto-man --lib title_zh / display_title / pac_title`：
  `test_plan015_title_zh_never_enters_cargo_toml`、
  `test_title_zh_parsing_and_display_chain`、`pac_display_title_locale_chain`、
  `pac_title_overrides_document_title` 全绿（294 filtered）。
- `cargo t -p auto-lang --features ui-iced --lib app_registry::`：
  **23 passed / 0 failed**（含新增 `scan_temp_dir_title_zh_and_display_chain`、
  策展集恰等、manifest 解析、launch resolver e2e）。
- `cargo check -p auto`（main.rs 注入臂）：绿。

## 策展集既有红修齐（非本计划引入）

`scan_examples_ui_curation_set` 在 base（master 71ed7ea90）即红：9c6c27e86
（2026-09-12，031-image-viewer 进桌面裁定）补了 pac `desktop:"true"` 但漏更
want 清单（16 缺 image-viewer）。T6 按"多一个=悄悄上架"双向语义把 want 补到
17 并注明溯源；断言语义未放宽。

## pac.at 覆盖核行（AC-5）

git diff 计数：auto-lang 33 / auto-os 5 / auto-kanban 1 / auto-musk 1 /
auto-term 2（app+at-app）/ auto-os-config 1 / auto-down(jade-garden) 1
——**44 个 pac.at 全部 `title`+`title_zh` 双全**；`name`/`exe_name` 语义零改动。

## 手段调整说明

AC-6 的"launcher/dock/任务栏"桌面格视觉截图未采——VM 独立窗标题（任务栏
标签同源：LaunchSpec.title 即 display_title 注入，renderer 桌面格读点同函数
族）已三 locale 实机 + 23 单测绿，按 PLAN-012 先例以 headless 等价成文；
整桌面 zh/en 截图留 /auto-plan:review 实机门复核。
