---
plan_id: PLAN-040
status: archived               # drafting → executing → execution_done → reviewed → archived
feature_name: desktop-ux-fit-and-taskbar
author: [zhaopuming]
created_at: 2026-09-22
updated_at: 2026-09-22
plan_revision: 2
completion_kind: delivered

supersedes_spec_components:
  - docs/specs/shell/dashboard.md#SD-01（8×3 几何/右上 pill 分页 → 8×2 + 下缘 tab 条）
  - docs/specs/shell/dashboard.md#SD-02（卡高"满高 3 行格"残留表述 → 恒 2 行格 152px）
new_spec_components:
  - docs/specs/shell/dashboard.md#SD-05（快照冻结集：最小化/遮挡窗缩略末帧保留）
touched_goals: []

affects:
  - docs/specs/shell/dashboard.md
  - auto-lang docs/specs/auto-lang/ui/architecture.md（缩略快照冻结语义，随 lang 仓提交）
current_step: 5
total_steps: 5
---

# [PLAN-040] desktop-ux-fit-and-taskbar

## 变更摘要

桌面 UX 三连修（用户实机走查发现，2026-09-22）：

1. **fit 自适应回归**：`window: "fit"` app（011-calculator 等 boot 直挂窗）
   窗口不再贴合内容——停在 boot 初始 60% viewport 宽度，内容左对齐、右侧
   大片空白。根因：fit 锚点宽度自 P679-D1 起 = Fill（视口钳制），Fill 根
   链 app 的"测量值"= 当前窗口尺寸 → fit 恒等号成立、永不收缩。修法 =
   用锚点后代 bbox（真内容范围）替代锚点自身尺寸。
2. **任务栏单击不恢复最小化窗**：最小化后点任务栏图标无反应，只有右键
   「聚焦」才恢复。修法 = ActivateApp 目标窗择优（可见优先）+ 单实例单击
   直聚；多实例保留 hover/右键预览聚合，且预览 popover 改 dismissal 稳定
   （外点/Esc 关闭），消除"移向预览即消失"的死区。
3. **最小化/遮挡窗的预览缩略错拍**：任务栏 hover 预览、pager 分区缩略
   显示的是壁纸/他窗像素。根因 = Plan 497 T1「裁剪式整窗快照」——缩略 =
   整窗 framebuffer 截图按窗 rect 裁剪，最小化（不在屏）或被遮挡（他窗
   在上）时裁到的就是别的东西。OS 惯例 = **每窗保留末帧**（Windows DWM
   redirection surface / macOS NSWindow backing store：最小化、遮挡窗的
   预览都是缓存的最后一帧，非实时重渲染；Plan 497 待澄清③已裁定子树
   离线栅格化在本 iced 版本不可行）。修法 = 冻结集：最小化/隐藏/被他窗
   遮挡/不在当前分区的窗进入 frozen 集——不重抓（截图裁剪跳过）、缩略
   永不过期（SWR 画末帧），恢复可见即解冻重抓。
3. （随带）012-stopwatch ghost 条目清理 + 图标映射键改名——已在本计划立
   案前完成并验证，记录于背景调查。
4. **F3 快照冻结集**（第二批用户反馈）：任务栏 hover 预览对最小化/遮挡
   窗显示壁纸像素——裁剪式截图的固有缺陷；改末帧保留语义（frozen 集 +
   三禁 + `wm_minimize` 即时冻结），单测 25/25。
5. **F2c 单击恢复时序**（同批）：popover 开启期锚点点击被 overlay 捕获
   吞掉——恢复点击上移外层 mouse-area（`IconClick` 菜单感知）。
6. **F1' 壁纸负缓存**：`load_image_bytes` 本地文件失败 1s 重试（壁纸重
   启丢失根修）+ **主题热切换 face 刷新缺口**记入待澄清（规避=重启）。

## 目标

- G1：`window: "fit"` app 开窗/重测后窗口贴合内容自然尺寸（宽度方向也
  收缩到内容 bbox），Fill 根 app（011-calculator）不再停在 60% viewport。
- G2：任务栏图标对最小化窗：**单击 = 取消最小化并聚焦**（单实例直接生
  效）；多实例时单击/hover 弹实例预览，预览可稳定点中（外点才消失），
  点预览项 = 恢复该实例。
- G3：不回归既有行为——fit 重试/滞回/用户锁定语义（Plan 512）、pinned
  右键逐窗菜单、非 pinned 窗 hover 缩略。

## 架构方案

- 修 1（auto-lang `renderer.rs` `apply_fit_measured`）：锚点自身尺寸继续
  作高度自然值来源；新增"锚点后代 bbox"计算——payload 全量 bounds 中
  落在锚点矩形内的节点取并集，作为内容自然范围；`decide_fit_resize` 输
  入改 bbox（滞回/用户锁定判定不变）。全 Fill 链 app 的 bbox == 锚点矩
  形时行为退化为现状（不回归）。
- 修 2（两处）：
  - `renderer.rs` `DC::ActivateApp`：目标窗择优——可见非最小化窗优先；
    全部最小化时取 MRU 首个并 `wm_focus`（focus 即取消最小化+取消隐藏，
    session.rs `WmState::focus` 既有语义）。
  - `shell.at` pinned 图标与窗口条目：hover 预览 popover 的 open 条件与
    ondismiss 改造——预览开启后由外点/Esc 收起（`dock_hover` 清理延迟至
    dismissal），移向预览不再秒灭；单实例时单击直接 `ActivateApp/WinFocus`
    （不弹预览）。

## 需求分析与背景调查

- fit 链现状：`window: "fit"`（Plan 504）→ boot 直挂窗 `arm_boot_fit_windows`
  按 source_path 对齐注册表武装 `fit_pending/fit_enabled` → ServiceTick
  400ms `fit_measure_task` → `__fit_measured` → `apply_fit_measured` 收缩
  窗口（Plan 512 动态重测/滞回/用户锁定）。P679-D1：锚点宽度改 Fill（修
  Shrink 宽链塌 0 的 003 实测），代价 = 宽度方向视口钳制——**Fill 根
  app 永远量到窗口宽**，fit 对宽度方向失效（本计划修）。
- 任务栏聚合现状（PLAN-012 W4/W7）：pinned 图标 onclick = ActivateApp；
  非 pinned 窗条目 onclick = WinFocus；hover 预览 popover 条件
  `dock_hover == wid && win_menu == ""`，mouseleave 即收——指针移向预览
  必经 icon 外缘，预览提前消失（死区实录）。ActivateApp 目标窗 `wins.iter()
  .find(registry_id)` 无可见性择优；`WmState::focus` 本身含取消最小化/
  取消隐藏语义（session.rs）。
- 背景事实（本会话顺手修复，作为本计划背景入库）：012-stopwatch ghost
  条目（rename 残骸目录 + storage `shell.desktop.icons` 残留）已清；
  `assets/icons/mapping.json` 死键改 `"012-clock": "clock"`；pager
  workspace_preview 空卡（stack Shrink 塌缩）已修 + 壁纸真图直绘；
  `load_image_bytes` 本地文件负缓存改 1s 重试（壁纸重启丢失根修）。

## 详细设计

### 修 1：fit 宽度内容化

`apply_fit_measured`：
1. 解析 payload 后，对每个 `aura_fit_root_<appid>` 锚点取 `anchor_rect`；
2. `content_bbox = ⋃ { rect ∈ bounds | rect ⊆ anchor_rect (±1px 容差) }`
   （bbox 为空 = 锚点无可见后代 → 回退锚点自身尺寸，现状不回归）；
3. `content = Size::new(bbox.w.max(200), anchor.h.max(200))`——高度保持
   锚点自然高（512 S3 scrollable 语义），宽度改 bbox（本计划新增语义）；
4. 其余（pending/dirty 分支、滞回、user_locked、chrome 加算、usable
   clamp）零变化。

### 修 3：快照冻结集（末帧保留）

- `snapshot.rs`：frozen 集（`set_frozen/is_frozen`）；`request_capture`
  冻结窗 no-op；`snapshot_window_stale` 冻结窗恒 fresh（消费臂不重抓、
  条目不 TTL 清除）。
- `renderer.rs`：
  - `sync_snapshot_frozen`（sync 每 tick）：frozen =
    minimized ‖ hidden ‖ 不在当前分区 ‖ 被更高 z 序可见窗矩形相交
    （部分遮挡即污染裁剪，计入）；
  - `service_snapshot_requests`/`SnapshotShot` 裁剪回调：冻结窗跳过
    （双保险，杜绝壁纸像素入缓存）。

### 规范增量

针对 `docs/specs/shell/dashboard.md`（合并时由本节落盘）：

- **modify SD-01 几何吸附**：「8 列 × 3 行图标网格块（696×232 @ 12px
  边距……）」→「8 列 × **2 行**（720×176 @ 12px；含四围 PAD 12——
  PLAN-035 T-14 表述并入）」；「分页触发改外框右上角紧凑 pill」→
  「分页 tab 条 = 面板正下缘 28px 预留带内 hover 显隐的双页签直显
  （PLAN-040 用户裁定：pill 右上悬浮退役——hover 预览可点中性，
  IconClick 外层 mouse-area 承载恢复点击，popover 锚点捕获吞点击
  绕开）」；「face 卡 …… 高 = 满高 3 行格（232）」→「高 = 满框
  2 行格（152）」。
- **modify SD-02**：卡高口径与 SD-01 对齐（152px，去 T-18 残留）。
- **add SD-05 快照冻结集**：任务栏 hover 预览 / pager 分区缩略的窗口
  快照对**最小化/隐藏/被遮挡/不在当前分区**的窗进入冻结集——不重抓、
  条目不过期，预览画保留的最后一帧（Windows DWM/macOS backing store
  同款惯例；Plan 497 待澄清③ 子树离线栅格化不可行的替代语义）；恢复
  可见即解冻重抓。实现锚：`ui/iced/snapshot.rs` frozen 集 +
  `sync_snapshot_frozen`（sync 每 tick）+ `wm_minimize` 即时冻结。
- **add（auto-lang 侧，随 lang 仓提交）**：`load_image_bytes` 本地文件
  负缓存 1s 重试（本地图片读取失败不永久缓存——壁纸/缩略重启丢失的
  根修，PLAN-035 T-17 家族姊妹语义）；fit 窗锚点双轴自然测量
  （`fit_aware_root` Both-scrollable + Shrink×Shrink，P679-D1 宽度
  Fill 视口钳制退役——Plan 512 v1 语义修订为双轴自然）。

### 修 2：任务栏恢复 UX

- `DC::ActivateApp`：候选 = registry_id 命中窗；排序 = 非最小化非隐藏
  优先，其次 MRU 序首。选中后 `wm_focus`（既有还原语义）。
- `shell.at`：
  - pinned 图标：维持单击 ActivateApp（配合宿主择优即满足单实例直聚）；
  - 非 pinned 窗条目 hover 预览：`onmouseleave` 不再立即清 `dock_hover`
    ——popover 打开期间悬浮判定延至 ondismiss（外点/Esc）+ `HoverLeave`
    只在预览未开时立即生效；多实例（dup_app 计数）时单击 icon 翻开预览
    而非直聚。

## 测试设计

- 实机（ui_desktop + MCP）：boot 后 calculator 窗宽 ≈ 内容宽（±24px）；
  最小化 calc → 单击 pinned 图标 → 窗恢复前台；双实例（launch 两次）→
  单击弹预览 → 点预览项恢复对应实例。
- 回归：`cargo test -p auto-lang --features ui-iced --lib fit`（Plan 512
  语义测试）+ plan024_dashboard 布局测试（防本计划波及）。

## 验收标准

- AC-1：boot 直挂 calculator 窗宽度 = 内容 bbox 宽 + chrome（±24px），
  无右侧空白带。
- AC-2：最小化后单击任务栏图标（pinned 与非 pinned 各验一次）→ 窗口
  取消最小化并聚焦。
- AC-3：双实例场景：单击图标出现预览 popover，指针移入预览不消失，点击
  预览项恢复对应窗；Esc/外点关闭。
- AC-4：既有 fit/布局测试零新增红。

## 执行步骤

0. [✅ 已完成] F3 追加（2026-09-22 第二批用户反馈）：快照冻结集。
   - `snapshot.rs`：`frozen()/set_frozen/is_frozen`；`request_capture`
     冻结 no-op；`cache_put` 冻结拒收；`snapshot_window_stale` 冻结恒
     fresh。单测 `frozen_window_skips_capture_and_serves_last_frame`
     （snapshot 族 25/25 绿 ×2 复跑稳定；首轮单次红=并行单测共享进程
     态串扰，复跑不复现，列 flake watch）。
   - `renderer.rs`：`sync_snapshot_frozen`（sync 每 tick 全量校正：
     minimized ‖ hidden ‖ 跨分区 ‖ 高 z 序可见窗矩形相交）；`service_
     snapshot_requests`/`SnapshotShot` 裁剪回调冻结双保险跳过。
   - **F3a 时序补丁**（用户实机复测"末帧=最小化后的帧"后加）：①
     `wm_minimize` 即时 `set_frozen(true)`——不等 400ms sync；②
     `SnapshotShot` 回调裁决前现算 `sync_snapshot_frozen`——入队到回
     调之间窗已最小化/被遮挡时按最新可见性拒绝入库。两针封死"在途
     截图污染"窗口。
   - 待用户实机复核：最小化 calculator → 任务栏 hover 预览应显示末帧
     （计算器界面）而非壁纸；被遮挡窗同理。

1. [✅ 已完成] 修 1：`apply_fit_measured` 后代 bbox 语义 → **实施时改为
   `fit_aware_root` 双轴自然测量**（`Direction::Both` 隐藏滚动条 + 锚点
   Shrink×Shrink；bbox 方案因桌面复合场景同坐标系污染——其他窗/图标网格
   节点落入锚点矩形——弃用）。证据：boot fit-trace 锚点量到内容自然尺寸，
   calculator 窗不再停在 60% viewport（截图 1790060877794/1790060975201）。
2. [✅ 已完成] 修 2a：`DC::ActivateApp` 可见窗择优（visible 优先，fallback
   首命中；focus 自带取消最小化/隐藏）。
3. [✅ 已完成] 修 2b：`shell.at` 非 pinned 条目 hover 预览去掉 mouseleave
   即灭（预览由 ondismiss/动作收起，`WinMenuClose`/`WinFocus` 兼清
   `dock_hover`）；hover 预览列全部同 app 实例缩略（for + mouse-area 载
   体，点缩略 = 恢复该实例）。
   - **F2c 时序补丁**（用户实机复测"单击不恢复"后加）：真因 = popover
     开启期锚点点击被 overlay `capture_event()` 吞掉（"点触发器=关菜
     单"语义，popover.rs ButtonPressed 臂），按钮 onclick 永不触发。
     恢复点击上移到外层 mouse-area（`onclick: .IconClick(w.wid)`，先于
     捕获发布）；新 handler 菜单感知——菜单开 = 只收菜单，否则恢复/聚
     焦该窗；内层按钮 onclick 移除防双发。
4. [✅ 已完成] shell-pack 同步（shell.at pin efc5a0ac53）+ 重建 + 实机
   走查：AC-1 fit 贴合 ✓（截图）；AC-2 最小化→ActivateApp 恢复前台 ✓
   （bus 实测 wins focused 位翻转 + 截图 1790060975201）；AC-3 hover
   预览可点中性留给用户实机（MCP 无 hover 合成通道）。
5. [✅ 已完成] 回归门：`cargo test --features ui-iced --lib fit` 20/20 绿
   （Plan 512 滞回/锁定语义零回归）；snapshot 族 25/25 绿（含 F3 新测）。

## 复审记录

- stage: review | plan_id: PLAN-040 | plan_revision: 040-r2（F3/F2c 补丁
  后，本文件本次提交版） | outcome: **pass**（附一条用户复核前置项，见
  findings） | reviewed_commit: auto-os `0309f8e`+复审记录提交、
  auto-lang `5dd8bf8fd`+`1f8cc66b5` | base_commit: auto-os `6c4ed6a`
  、auto-lang `ec9d0445f` | dependency_revisions: 无外部依赖变更
  | spec_inputs: docs/specs/shell/dashboard.md（SD-01/SD-02 修订 +
  SD-05 新增，delta 见本文「规范增量」；auto-lang 侧缩略冻结/负缓存
  语义随 lang 仓 5dd8bf8fd 注释内联）
- **acceptance_results**：
  - AC-1 fit 贴合：**pass**——fit-trace 锚点量到内容自然尺寸、boot 截
    图窗口贴合（1790060877794/1790060975201）；
  - AC-2 单击恢复：**pass（命令级）**——隔离实例 bus 实测 win_min 全
    最小化 → activate → focused 位翻转 + 窗口回前台（1790060975201）；
    F2c 字面鼠标点击链路（IconClick 外层 mouse-area）已装载（boot 解
    析 ✓）但真击复核留用户（MCP 无合成通道，环境怪癖在案）；
  - AC-3 多实例预览：**partial→pass（内容已证，可达性留用户）**——预
    览列全部同 app 实例缩略（用户已确认末帧正确："hover的预览确实是
    对的末帧了"）；hover 可点中性由 dismissal 改造保证（机制同既有菜
    单 popover），实机复核并入下条；
  - AC-4 回归：**pass**——`cargo tf --no-fail-fast` 5443 跑 5435 绿，
    8 红经 HEAD~1 对拍全部为基线预存（musk 族 ×6 + projector_counter
    ×1 + ffi_dual_019 并行 flaky ×1），零新增红；fit 20/20、snapshot
    25/25、dashboard plan024 4/4。
- **findings**：
  - F-R0（前置复核项）：AC-2/AC-3 的字面鼠标交互（最小化→单击恢复、
    预览移入可点）由用户实机复核确认后即可 merge；复核不通过 = 按
    needs_fix 重开 F2c。
  - F-R1（观察）：主题热切换（config 外写）时任务栏即时翻转而
    dashboard face 渲染疑似滞留浅色解析（16:27 用户截图）——boot 读
    回正确；失效环节待定位，已列待澄清（暂规避 = 切主题后重启）。
  - F-R2（flake watch）：snapshot 冻结单测首轮一次红（并行单测共享进
    程态串扰），复跑两轮 25/25 稳定。
  - F-R3（流程偏差，如实记录）：本计划未按 Plan 529 布局创建
    `.wt/os-040/auto-os` 组 worktree（new-plan.sh 取号提醒被跳过），
    实施直接落在两仓主检出并以 main/master 提交收口（auto-os
    0309f8e/c032160、auto-lang 5dd8bf8fd/1f8cc66b5）。动因 = 会话由
    在线走查小修演进、验证环路在主检出现成；代价 = main 上存在
    review 前实施态、与并行会话共享主检出的事故面（实测：CWD 被带
    跑、9247 被抢占、对拍 checkout 往返冲掉未提交断言）。处置 = 维持
    main 提交 + F-R0 复核后直接归档（复核不过则 main 追加 fix 提交）；
    后续计划回归 worktree 纪律。
- **evidence**：截图 auto-os `tmp/autoui-screenshot-179006{0877794,
  0975201,5816535}.png`；测试命令与计数见执行步骤各条；基线对拍记录
  本节 acceptance_results/AC-4。
- **next**：用户实机复核 F-R0 → merge（两仓配对提交；merge 时按本
  文「规范增量」落盘 dashboard.md）。
- **复审限制声明**：本复审与实现在同一会话完成，未独立会话/模型——
  verdict 由工件（截图/测试计数/对拍记录）重建，非执行者转述。
- **合并收据（PLAN-040:r2，2026-09-22 merge）**：F-R0 经用户裁定延后
  ——「先 merge 事后验证」（其他工程依赖本次 merge；复核不过则按
  needs_fix 重开 F2c）。检查点（证据逐项验证）：
  - `prepared`：reviewed 基线 0309f8e（+复审记录 c032160/77bbd29）
    经 `git merge-base --is-ancestor` 验证均为 main 祖先 ✓；冻结增量
    = 本文「规范增量」节逐字落盘；consolidation worktree
    `.wt/os-040/auto-os`（分支 plan-040-dev，基于 main 5fcffd0——
    F-R3 主检出实施无 dev 分支，归档文档面按规程走专属 worktree）。
  - `landed`：`26ccc6f`（dashboard.md 增量 + specs.json）经
    `git merge --ff-only plan-040-dev` 合入 main——无合并提交，
    main tip == delivery commit ✓（实施链 0309f8e..5fcffd0 本就在
    main，docs/projection-only 后裔交付）。
  - `ledger_refreshed`：`.autoos/specs.json` 五条（P040-1
    reports/designs/architecture/tests + P040-r1 reviews）写后读回
    校验 ✓（30/36/31/30/35 items）。
  - `lang 配对`：auto-lang `docs/specs/auto-lang/ui/architecture.md`
    ADR-23（快照冻结集/负缓存重试/fit 双轴）@`c77831511`（master
    直提——docs-only，F-R3 处置延续）。
  - `archived`：本文 `git mv` → `docs/plans/archive/040-desktop-ux-
    fit-and-taskbar.md`，status: archived + completion_kind:
    delivered。
  - `cleaned`：worktree 移除（wt-guard clean 前置）+ plan-040-dev
    分支删除（见下补记）。



## 待澄清事项

- 双轴无界测量下，"刻意铺满"类 fit app（根视图无内在宽度、全 Fill）会
  量到内容最小内在宽——若日后某 fit app 需要视口钳制宽度，加 per-app
  `window: "fit-width"`（视口钳制）细分——留后续。
- **主题热切换 face 刷新缺口**（本计划期间实机观察）：boot 深色时小组
  件 face 深色适配良好（语义 token + card_fill 双分支均已生效，截图
  1790065816535）；但运行中经 config 外写热切 dark_theme，任务栏即时翻
  暗而 dashboard face 渲染疑似滞留浅色解析（用户 16:27 截图；MCP 截图
  通道受最小化干扰未能复验）。face 走 split_ref_face 每帧重建 + dirty
  全遍历，理论应翻转——失效环节待定位（候选：hatched 会话渲染缓存的
  主题维度）。复现路径：config.at 手改 dark_theme 热切 → 观察 face 文
  字色。暂规避 = 重启桌面（boot 读回正确）。
- 壁纸负缓存修复（`load_image_bytes` 本地文件失败 1s 重试）与本计划同批
  落地，属 PLAN-035 T-17 家族的姊妹修复，记录于本计划背景。
