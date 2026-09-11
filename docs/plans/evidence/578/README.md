# PLAN-008 (origin 578) 证据索引 — 2026-09-11 work + review 轮

执行环境：worktree 组 `.wt/os-008/{auto-os,auto-lang,auto-down}`（分支均
`os-008-dev`；auto-os base `a8d7450` / auto-lang base `622edfdd9`）。
提交：auto-lang `45ae96897`、auto-os `2c012a3`。
复审轮：同基线自主复现（AUTOUI_MCP_PORT=9348 + AUTOUI_ACCEPTANCE=1 +
AUTO_VM_STORAGE_FILE 存储隔离），`cargo tf --no-fail-fast` 3506/3506 全绿。

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

## 实机腿（复审轮已自主复现 + 残余用户快核）

复审轮经验收注入通道（autoui_desktop bus/handler）自主完成：
- `launcher-open-21apps.png`——launcher 全列表徽标 **"1 / 21 apps"**
  （16 C 档 + 3 容器 + 2 画廊；含 kanban 应为 22——视觉面算术铁证）。
- `launch-widgets-gallery-vm.png`——bus `launch\twidgets-gallery`：VM
  原生窗完整渲染（Overview/Layout 侧栏 + 61 Widgets hero）。AC2 ✓
- `launch-ui-gallery-vm.png`——bus `launch\tui-gallery`：真内容页渲染
  （UI Gallery 标题 + 002-counter 教程卡片，非占位页），桌面存活。AC2 ✓
- `iced-scan-off.log`——seed `shell.apps.scan_galleries=false` 后 boot
  `36 entries (19 desktop-visible)`（画廊在时 38/21）。AC4 实机 ✓

残余用户快核腿（秒级，非阻塞）：
- launcher 列表滚动至尾部，目视两画廊行（UI Gallery / Widgets Gallery
  标题行）——MCP 输入/键盘/ApplyFilter handler 三条注入途径均作用域
  受限（根组件），行级像素留用户。
- 桌面图标格（非 launcher）中 image/layout-grid 图标形态目视。

（原始 work 轮实机交互腿记录见下，已被复审轮自主复现取代大半。）
