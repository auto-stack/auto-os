---
plan_id: PLAN-043
status: executing               # drafting → executing → execution_done → reviewed → archived
feature_name: desktop-app-vm-check
author: []
created_at: 2026-09-23
updated_at: 2026-09-23

# /auto-plan:review 结束时填写：
supersedes_spec_components: []
new_spec_components: []
touched_goals: []             # 引用 docs/specs/goals.md 的 GOAL-NNN

affects: [auto-lang/examples/ui, apps/]
current_step: 1
total_steps: 1
---

# [PLAN-043] desktop-app-vm-check

虚拟桌面（VM 轨，ui_desktop）各 app 打开情况检查与修复跟踪。
2026-09-23 实机走查启动，每个 app 一个 Part 记录：现象 → 根因 → 修复 → 验证。

## 走查环境

- 桌面宿主：`D:/autostack/auto-lang/target/debug/examples/ui_desktop.exe`，
  CWD=`D:/autostack/auto-os`，`AUTO_OS_ROOT`=本仓根，
  `AUTO_VM_STORAGE_FILE`=隔离档案（`tmp/desktop-vm-check/storage.json`），
  `AUTOUI_ACCEPTANCE=1`，MCP `:9471`。
- boot 注册表：43 entries / 29 desktop-visible（主根 examples/ui opt-in +
  容器臂 apps/* + manifest repo 臂 auto-kanban/auto-musk/jade-garden/auto-term
  + os-config + 双画廊，去重后口径）。
- 驱动：MCP acceptance 通道（bus 动词 `launch\t<id>` / `close\t<wid>` +
  `autoui_screenshot` 留痕）。

## 变更摘要

## 目标

## Part 1：029-photo-gallery 照片图库——展示内容非实时扫描（已定位根因）

**现象（用户报告）**：图库展示的照片集合与预览不是最新的；照片目录
（`C:\Users\zhaop\Pictures`）内容已变化，图库显示不变。

**根因（已证实）**：该 app **没有任何后端**，运行期零目录扫描。照片集是
2026-09-14 一次性离线烘焙进前端的死数据：

- 流水线（`examples/ui/029-photo-gallery/scripts/`）：
  1. `prepare_gallery.py`：扫一遍 `C:\Users\zhaop\Pictures`（+ Screenshots
     子目录前 15 张）→ 生成 260px JPEG 缩略图 + `gallery_data.json`
     （元数据 + base64 缩略图，821KB，9/14 16:43）。
  2. `generate_at.py`：读 json 生成 `var p_ids/p_titles/p_albums/p_dates/
     p_fulls/p_thumbs...` 数组源码。
  3. `assemble_app.py`：`app_head.at` + 数组 + 固定尾巴 → `src/front/app.at`。
- `src/front/app.at` 模型里硬编码 24 张（注释写"68 张"也是假的）；
  `p_thumbs` = 烘焙缩略图**绝对路径**；`p_fulls` = 原图绝对路径
  （`C:/Users/zhaop/Pictures/...`——原图还活着所以大图能开，但集合
  本身冻结在 9/14）。
- **脚本路径还指向已删除的 worktree** `D:\autostack\.wt\lang-628\...`
  （lang-628 组已折叠）——烘焙链在主检出上重跑会直接失败；主检出
  `app.at` 里的缩略图路径已被手工改写为主检出路径，与生成器脱节。
- pac.at 注释仍写 picsum.photos 固定 seed（Plan 537 旧描述），与
  Plan 628 改写后的本地照片实现不符。

**修复方向（待用户裁定）**：
- A. 加真后端：pac `back` + `src/back/api.at` `#[api]` scan 端点
  （port 4029 带），运行期扫 `C:\Users\zhaop\Pictures`，缩略图按需生成/
  缓存；前端 Init fetch。——与 020-music-player 的 media scan 先例同构。
- B. 保守：把烘焙脚本修到主检出可重跑（路径参数化 + 相对路径），
  文档注明"重新生成"操作；不解决实时性。
- 倾向 A（app 是桌面常驻成员，"照片目录变了图库不变"是产品级缺陷）。

**验收（修复后）**：照片目录增删文件 → 重启 app（或刷新）后集合跟随
变化；桌面 VM 轨实机截图留痕。

## 执行步骤

- [✅ 已完成] 2026-09-23 走查启动：boot 43/29，MCP :9471 驱动臂验证可用。
- [✅ 已完成] Part 1 根因定位（烘焙流水线实证，见上）。
- [ ] Part 1 修复（方向待裁定）+ 双端验证。
- [ ] 继续走查其余 app（027-file-manager、030-video-player、031-image-viewer、
  031-paint、036-tetris、037-klondike、038-minesweeper、041-auto-edit、
  auto(os-config)、ui-gallery、widgets-gallery、kanban、auto-musk、
  jade-garden、auto-term）——每个 app 补一个 Part。

## 走查中已发现的其它问题（待逐 app 立 Part）

- **[宿主 panic（最高优先）]** 连续启动 app 过程中宿主进程崩溃：
  `iced_widget-0.14.2 container.rs:291 Option::unwrap()` on None，
  桌面整体死亡（MCP 断连）。崩溃前最后 launch 的是 026-database
  （该 app 有 use 模块解析失败：use 模块 `{ package` 解析失败——多行
  import 语法被 P-15 单行星系解析打断，"引用其符号的面将落空"）。
  疑似坏 app 的残缺 view 触发宿主布局 unwrap。待：最小复现 + 宿主
  渲染兜底（panic 边界不应杀死整个桌面）。
- **017-chat**：launch 解析失败，弹「应用暂不可用 无法启动: 017-chat」。
- **018-book-reader**：`use back.api`（get_book/list_chapters...）解析
  失败（PLAN-664 静默跳过）→ 同样「无法启动」占位。
- **024-charts**：use `{ package: ... from "components" }` 多行形解析失败
  （页面仍出，但引用的符号面落空）。
- **016-calendar**：use `datetime` 模块解析失败（静默跳过）+ `flex-wrap`
  native 降级（Plan 412 在案）。
- **launcher 覆盖层**：summon 后 launcher 覆盖层长时间驻留，bus Esc/
  重 summon 未关闭；启动 app 的窗口开在其后（z 序/聚焦问题待查）。

## 待澄清事项

- Part 1 修复方向（A 真后端 / B 保守重烘焙）待用户裁定。
- 宿主 panic 是否单独立 plan（auto-lang crates/ 改动 Category A/B 门档），
  还是在本 plan 内做 os 侧复现 + lang 侧修复协同。
