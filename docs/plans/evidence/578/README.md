# PLAN-008 (origin 578) 证据索引 — 2026-09-11 work 轮

执行环境：worktree 组 `.wt/os-008/{auto-os,auto-lang,auto-down}`（分支均
`os-008-dev`；auto-os base `a8d7450` / auto-lang base `622edfdd9`）。
提交：auto-lang `45ae96897`、auto-os `2c012a3`。

## T6 iced 轨（VM 桌面）

| 文件 | 内容 |
|---|---|
| `iced-baseline-master.log` | master exe × 主检出 CWD：`38 entries (22 desktop-visible)`（os-config+kanban 在组） |
| `iced-baseline-os008group.log` | **受控基线**：master exe × os-008 组 CWD：`36 entries (20 desktop-visible)`（组内无 auto-os-config/auto-kanban 检出） |
| `iced-new-os008.log` | **新代码** × 同组 CWD：`38 entries (21 desktop-visible)` |
| `iced-new-fullscreen.log` | 全屏冒烟同上（boot 正常） |
| `iced-desktop-galleries.png` | 窗口态桌面截图（boot 未崩，DualApp+calculator 正常渲染） |
| `iced-fullscreen-galleries.png` | 全屏桌面截图（壁纸+dock+boot 窗） |

**验收 1 对照（同组受控）**：36 (20) → 38 (21) = **+2 entries / +1
desktop-visible**（画廊 +2/+2，022-kanban 退策展 −1），与计划推演精确一致。
主检出对照（38/22 → 38/21）另受组构成差影响（os-config 根与 auto-kanban
检出不在 os-008 组，日志含 `kanban skipped` 行为证）。

## T6 vue 轨（desktop-host）

| 文件 | 内容 |
|---|---|
| `vue-run-os008-full.log` | worktree `auto` CLI + 脚本 env 全量生成日志 |

关键行：
- `✓ extra root: ui-gallery (from D:/autostack/.wt/os-008/auto-os/ui-gallery)`
  ——验收 3 前半（生成输出）精确命中。
- `⚠ extra root widgets-gallery skipped: no entry .at`——实为 render 过滤
  （vm 声明被 vue 过滤排除，设计行为）；⚠ 文案 "no entry .at" 为既有措辞
  混用（scan_app_root 对无入口/render 过滤同型返回 None），PLAN-008 未改。
- `⚠ app ui-gallery skipped: needs ext files`——**验收 3 后半受挫**：
  Plan 465 登记 v1 限制（`@/ext/` 导入者跳过，ui-gallery 的 settings dep
  即此形态），最终 `apps-registry.ts` 无 ui-gallery 条目。属先于本计划的
  生成器边界，非扫描面回归。
- `⚠ app 022-kanban skipped: needs API client`——022-kanban 因 v1 跳过
  从未上过 vue 桌面（先于 T7），验收 6 vue 腿成立。
- `✓ Desktop host: src/App.vue + src/apps-registry.ts (23 apps)`——生成
  完成标志；产物 `.wt/os-008/auto-lang/examples/desktop-host/gen/front/vue/
  src/apps-registry.ts`（grep 无 ui-gallery/widgets-gallery/022-kanban）。

## 实机交互腿（待用户复核，沿 472/478 先例 headless 指针成文）

- 图标格/launcher 中 UI Gallery（image 图标）/Widgets Gallery（layout-grid
  图标）标题非裸 id 的目视确认。
- launch `widgets-gallery`（VM 原生）可用；launch `ui-gallery` 不崩桌面。
- `shell.apps.scan_galleries=false` 关断实机（单测已过：
  `gallery_extra_roots_scan_galleries_off`）。
