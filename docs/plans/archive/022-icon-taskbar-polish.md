---
plan_id: PLAN-022
status: archived             # drafting → executing → execution_done → reviewed → archived（2026-09-17 r1 pass merge 清偿）
completion_kind: delivered
feature_name: icon-taskbar-polish
author: [ZCode 会话 2026-09-15]
created_at: 2026-09-15
updated_at: 2026-09-17
plan_revision: 1              # 回补计划无 revision，复审定基线 r1

# /auto-plan:review 结束时填写：
supersedes_spec_components: []  # showdesk-wallpaper 与标题配色零重叠（grep 实证，无退役）
new_spec_components:
  - docs/specs/shell/showdesk-icons.md   # 新增：桌面图标网格/任务栏图标消费/切换器预览发布契约
  - apps/028-launcher/SPEC.md            # 修改：5 列网格+限高滚动+120% 行高布局语义节
touched_goals: []             # 本仓无 docs/specs/goals.md（PLAN-023 同例）；空影响说明见规范增量节

affects: [auto-os/shell, auto-os/apps/028-launcher, auto-os/assets/icons, auto-lang/ui-iced-renderer]
current_step: 10
total_steps: 10
---

# [PLAN-022] icon-taskbar-polish

> **性质说明**：本计划为**回补记录**——2026-09-15 会话中用户以截图逐条直接驱动
> 修改（未走 /auto-plan:new 先行），工作完成后按用户要求补记全部需求与修改。
> 所有改动**均未提交**（auto-os 与 auto-lang 两仓工作区）。

## 变更摘要

桌面/任务栏/launcher 三处图标体系的整轮打磨：真位图图标全量接入（任务栏 9 钮 +
launcher + 切换器）、桌面图标 48px 满幅 + 标题回归、拖拽交互修复（误跳格阈值、
拖拽幽灵、落点格高亮）、列主序排布、暗壁纸标题配色、launcher 网格 5 列 + 限高滚动、
切换器预览刷新 + 新建桌面入口、DualApp 验收窗退役。涉及 auto-os（.at/.at 资产/mapping）
与 auto-lang（renderer.rs/session.rs/ui_desktop.rs，已重建 ui_desktop）。

## 目标（用户需求，按提出顺序）

| # | 需求 | 来源 | 状态 |
|---|------|------|------|
| R1 | launcher 图标满幅渲染，去 chip 外框 | 截图驱动 | ✅ |
| R2 | 任务栏 7 钮（launcher/虚拟桌面切换/双布局/通知/设置/电源）换真位图图标 | 文字需求 | ✅ |
| R3 | `D:\autostack\assets\{light,dark}` 图标按内容重命名为 `<stem>-<theme>.png` | 文字需求 | ✅ |
| R4 | term→terminal、os-config→config 接入；kanban/musk 用户自理 | 文字需求 | ✅（kanban/musk 用户后续自行重制） |
| R5 | 桌面图标 64px（后改判 48px）+ 下方标题回归 + hover 半透明底 + 去黑边框 | 截图+文字 | ✅ |
| R6 | 拖拽修复：单击/双击不跳格、拖拽幽灵跟随、落点格高亮 | 截图+文字 | ✅（逻辑已落地，手感待用户实测） |
| R7 | 桌面图标列主序排布（纵向优先） | 文字需求 | ✅ |
| R8 | launcher 应用网格 5 列 + 卡片限高滚动 | 截图+文字 | ✅ |
| R9 | launcher 候选行行高 120% + 左距/图标间距紧凑 | 截图+文字 | ✅ |
| R10 | 单击/双击第一击不闪拖拽副本（幽灵门控到位移阈值后） | 截图+文字 | ✅ |
| R11 | 暗壁纸标题不可读 → 亮白标签 + 深色底片（图片壁纸）/亮度自适应（纯色） | 截图+文字 | ✅ |
| R12 | 任务栏高亮：去描边 → hover 底放大一号（44px）→ 亮度两轮调参（white/25→50） | 截图+文字 | ✅ |
| R13 | 激活条带出现时图标不得被抬升 | 截图+文字 | ✅ |
| R14 | 切换器：预览图空 + 缺"新建虚拟桌面"入口 | 截图+文字 | ✅ |
| R15 | DualApp 残留窗退役 | 问答确认 | ✅ |
| R16 | 切换器："+"新建卡 + 预览瓦片数据/刷新链修复（publish 独立化 + 400ms 节拍直发 + 快照补抓） | 截图+文字 | ✅（深色主题下深色窗缩略对比度低，见待澄清 5） |
| R17 | 任务栏激活条带下移 1px（`mt-0.5`→`mt-[3px]`，pinned 三态 + 窗口条目二态五臂同值）——2px 时条带视觉贴住高亮框，条带下方尚余 1-2px 空隙可用 | 截图+文字 | ✅（2026-09-15 二次会话，3x 放大截图实测缝隙回归） |

## 架构方案

- **iconfile 通道**：`icon (name: "iconfile:<stem>")` 双主题位图臂（renderer 既有
  能力），资产根 = `AUTO_OS_ROOT/assets/icons/{light,dark}/<stem>.png`，主题跟随
  `dark_mode()`。任务栏按钮以 `variant: "ghost"`（无边框预设）+ icon 属性消费。
- **满幅 tile 语义**（沿 PLAN-018-FU2 用户裁定「保留原图圆角板、外框不要」）：
  位图自带圆角板 → 满幅渲染不套任何底框/描边；lucide 字标才保留品牌色 chip。
- **拖拽阈值**：拾起记录起点光标，松手位移 < 6px 视为点击原样落回；两条落格动词
  （全局 `__mouse_released` 兜底臂 + BlankDrop `desktop_icon_drop_at`）统一受
  `icon_drag_moved` 门。
- **切换器预览刷新**：快照抓取为异步 SWR——面板开着时由 400ms ServiceTick 逐可见窗
  补抓 + 重新发布预览数据 + 置 shell 重建；面板收起即停止（零常态开销）。

## 需求分析与背景调查

- 现有图标体系：PLAN-018（iconfile 位图臂 + mapping.json + 28 stem）、PLAN-018-FU2/3
  （桌面满幅 tile + 边框高亮）、PLAN-018-FU5/6（全量双主题图标）。
- 任务栏按钮 default 变体带"发丝描边"（PLAN-571 预设 `border border-border`）——
  位图外的圆角框来源。
- `D:\autostack\assets\{light,dark}` = 用户图标生成源目录（ChatGPT 原图，1254×1254），
  与 `assets/icons` 28 stem 一一对应 + 5 枚任务栏新 stem + 用户后续自制的
  kanban/musk/tetris/klondike。
- iced 限制实测：button 内容行把子件高度钳到文本行高（图标必须套定尺寸 col 或用
  plain row 承载）；col 自适应高会被 grid 拉伸成整行高（格子须固定高）。

## 详细设计（文件级修改清单）

### auto-os 仓

| 文件 | 修改 |
|------|------|
| `shell/shell.at` | 任务栏 9 钮（launcher/desktop-switch/layout-grid/layout-stack/notification/config/shutdown + pinned/window 条目）`icon` 换 `iconfile:<stem>`、`variant: "ghost"`（去描边）、`text-4xl`（36px 图标）、`h-11 w-11`（hover/激活底 44px）；切换器 popover 加"新建虚拟桌面"+"卡（`WorkspaceAdd`）" |
| `shell/desktop.at` | 图格：图标 48px 满幅（`w-12 h-12`）、格高固定 `h-[72px]`（标题回归、杀行拉伸）、hover `bg-white/10` 整组底、FU3 边框退役；标题三级配色（图片壁纸=白字+`bg-black/30` 底片、纯色壁纸按亮度） |
| `apps/028-launcher/src/front/app.at` | palette 行 button→mouse-area+row（iced button 内容行压扁子件，矩阵探针 A-E 实证）、满幅臂 40px 定尺寸 col、行高 `py-[18px]`、px-3/gap-2 紧凑；grid 5 列 + `max-h-[440px] overflow-y-auto` |
| `assets/icons/{light,dark}/` | 新增 stem：`terminal`、`config`、`desktop-switch`、`notification`、`layout-grid`、`layout-stack`、`shutdown`、`tetris`、`klondike`、`auto-robot`；重制覆盖：`kanban`、`auto-musk`、`auto-term`、`os-config`（512px 处理管线：alpha>32 内容框 → 方形化 → LANCZOS 512） |
| `assets/icons/mapping.json` | `auto-term`→`terminal`、`os-config`→`config`、+`auto-os-config-front`→`config`、+`036-tetris`/`tetris`→`tetris`、+`037-klondike`→`klondike` |
| `D:\autostack\assets\{light,dark}`（用户目录，仓外） | 33+33 张生成图按内容重命名为 `<stem>-<theme>.png`（28 stem 全量 + 5 任务栏 stem）；`klondite`→`klondike` 笔误纠正、`auto-musk.png`→`auto-musk-light.png` 规范化 |

### auto-lang 仓

| 文件 | 修改 |
|------|------|
| `crates/auto-lang/src/ui/iced/renderer.rs` | ① 桌面图标落格栅距 X/Y 分离（88/80）+ 两路落格动词 `icon_drag_moved` 门 + 6px 位移阈值；② 拾起记录起点 + `drag_icon`/`drop_c`/`drop_r` 数据面 + `__mouse_moved` 阈值判定与逐节拍补抓；③ `desktop_icon_cells` 加 `rows` 参数、未定位图标**列主序**填充（rows 按视口高/80px 行距，扣任务栏预留）；④ `__desktop_label_dark` 壁纸亮度判定（#hex 直算 luma；图片解码 32×32 均值，按路径缓存）+ `publish_workspace_previews` 独立化；⑤ ServiceTick：切换面板开着时逐可见窗 `request_capture` + 重发布 + shell 重建；⑥ `w5_desktop_icon_cells_assignment` 测试同步列主序语义 |
| `crates/auto-lang/src/ui/session.rs` | `icon_drag: Option<(String, (f32, f32))>`（附拾起起点）+ `icon_drag_moved: bool` |
| `crates/auto-lang/examples/ui_desktop.rs` | 459-dual-app demo boot 直挂窗退役（双窗隔离验收走专用 example `ui_dual_app`） |

### 规范增量（2026-09-17 复审补立，merge 时落 canonical）

| delta_id | 类型 | 目标 | before/after | rationale | acceptance |
|---|---|---|---|---|---|
| SD-01 | add | `docs/specs/shell/showdesk-icons.md` | before：桌面图标网格/任务栏图标消费/切换器预览发布无成文契约（PLAN-018 只钉 iconfile 协议族与资产管线，语义散见注释）；after：①桌面格 48px 满幅 tile（沿 018-FU2 满幅裁定）+未定位图标列主序填充（rows=视口高/80px 扣任务栏预留）+拖拽 6px 位移阈值门双落格动词+标题三级配色（图片壁纸白字+`bg-black/30` 底片、纯色按亮度）；②任务栏 `iconfile:`+`ghost` 变体消费+44px hover/激活底+激活条带 `mt-[3px]` 五臂同值；③切换器预览独立发布（`publish_workspace_previews`+面板开时 400ms ServiceTick 逐可见窗 `request_capture` SWR 补抓、收起零常态开销） | R1-R17 十七轮用户裁定的持久语义钉死，防后续计划回归 | AC-1, AC-2, AC-3, AC-5, AC-6 |
| SD-02 | modify | `apps/028-launcher/SPEC.md`（新增布局语义节） | before：SPEC 只有接缝/排序规则/键盘流/已知边界/验收入口五节，网格布局无契约；after：5 列网格（`cols: 5` 用户裁定）+`max-h-[440px] overflow-y-auto` 限高滚动+palette 行 `py-[18px]`（120% 行高）+行容器 mouse-area+row（iced button 内容行钳高实测裁定） | R8/R9 布局语义钉死 | AC-4 |

空影响说明：`touched_goals` 为空——本仓不存在 `docs/specs/goals.md`（PLAN-023 同例），goals 记录归 `.autoos/specs.json` goals 节，merge 时派生。

## 测试设计

- **矩阵探针**（tmp/icon-matrix-probe，已删）：同结构 5 变体一次定案"iced button
  内容行钳子件高度、plain row 完美"→ palette 行容器改 mouse-area+row。
- **独立 MCP 实例**（ui_desktop + AUTOUI_MCP_PORT=9531 + 存储副本）：桌面图格 48px/
  标题/无黑边、launcher palette/grid 截图逐轮验收；切换器预览发布日志
  （`publish: 1 ws entries, tiles per-ws [("0", 1)]`）。
- **vue 轨**：`vue_verify.mjs` [1][2] 通过（[3] 为 PLAN-613 前的存量坏定位器，与本计划无关）。
- **诊断日志**：`[ws-preview] publish/tick refresh` 两条 eprintln 留存在构建中
  （定位用，提交前可拆）。

## 验收标准

1. 桌面图标 48px 满幅 + 下方标题 + 无黑边框，列主序排列（左列自上而下）。✅ 实测
2. 任务栏 9 钮位图渲染、无描边框、hover/激活底 44px；激活条带出现不挤动图标。✅ 实测
3. term=terminal 位图、os-config=config 位图（桌面格 + dock 均确认）。✅ 实测
4. launcher：5 列、palette 行高 120%、图标 40px 无缝、候选文本纵向居中、28 应用全注册。✅ 实测
5. 切换器：预览数据每拍发布（日志证实）、"+"卡可见。✅ 实测（真实缩略图依赖快照入缓存）
6. 单击/双击不再跳格或闪副本；拖拽幽灵/落点高亮逻辑就位。逻辑+构建验证，手感待用户实测。

## 执行步骤

1. [✅ 已完成] launcher 满幅臂 + `full` 旗标（R1）——app.at 8 处；vue_verify [1][2] 过、[3] 存量失败经基线对比排除
2. [✅ 已完成] 任务栏 7 钮 + dock 条目位图接线（R2）——shell.at；截图确认 dock "Auto Term" 终端位图
3. [✅ 已完成] D:\autostack\assets 66 文件重命名（R3）——脚本断言 33+33；用户新增 8 张（kanban/musk 重制中）未动
4. [✅ 已完成] 7 新 stem 处理安装 + mapping 3 键 + 旧 stem 覆盖（R4）——矩阵对照图目检
5. [✅ 已完成] 桌面图标 80→64→48px 两轮 + 标题回归 + hover 底 + 去边框（R5）——desktop.at 4+4 处
6. [✅ 已完成] 拖拽阈值/幽灵/落点高亮 + 落格栅距修正（R6）——renderer.rs/session.rs；矩阵探针 + 构建验证
7. [✅ 已完成] 列主序排布（R7）——desktop_icon_cells 加 rows；测试同步语义
8. [✅ 已完成] 暗壁纸标签配色（R11）——亮度判定 + 三级配色；期间修 u16 亮度累加溢出 panic
9. [✅ 已完成] launcher 5 列/滚动/行距（R8/R9）——vue 轨截图 + 桌面截图
10. [✅ 已完成] 切换器 "+"卡 + 预览刷新（R14）——发布日志确认每拍直发；DualApp 退役（R15）——ui_desktop.rs，boot 日志无 DualApp

## 复审记录

- 2026-09-17（独立新会话复审）：`stage: review` | plan_id PLAN-022 |
  plan_revision r1（回补计划缺 revision，本次定基线 r1）| `outcome: pass` |
  reviewed_commit auto-os `d616fa7`（base `65bb9cb`）+ auto-lang `94bc69c22`
  （主体：拖拽阈值/列主序/459 直挂退役，base `1450d5e`）+ `9bd26d884`
  （ws-preview 诊断拆除=待澄清①提交前置项清偿）| dependency_revisions：
  两 lang 提交 merge-base 实证均为 master 合并 `768e8fcd3` 与 023 r1 所测
  `46fd09dd6` 之祖先（最终 022 lang 状态在 023 r1 回归树内）| spec_inputs：
  `docs/specs/shell/showdesk-wallpaper.md`（标题配色与其零重叠，无冲突）、
  `apps/028-launcher/SPEC.md` | acceptance_results：AC-1..AC-5 pass（执行期
  用户逐条截图实测在案 + HEAD 标记复核：shell.at 7 枚 iconfile 按钮全 ghost
  变体、`mt-[3px]` 五臂+1 注释、desktop.at `w-12 h-12`+`h-[72px]`+
  `bg-black/30`、launcher `cols: 5`+`max-h-[440px]`+`py-[18px]`、mapping 5 键、
  WorkspaceAdd 新建卡 shell.at:416）；AC-6 pass（按其约定口径=逻辑+构建验证：
  `icon_drag_moved` 门 renderer×5+session×2、6px 阈值、双落格动词同门；手感
  反馈通道保持开放，见待澄清 4）| findings：F-01（已清偿）回补计划缺
  plan_revision/规范增量节——本次复审补立 r1 与规范增量（SD-01/SD-02）；
  非阻塞计划外项：切换器深色对比度（待澄清 5）、kanban/musk 旧图取舍
  （待澄清 2，用户自理）| evidence：lang 回归门复用（显式理由：被审提交
  已深居 main 历史，现跑全量测的是 HEAD 非基线）——023 r1 全量 tv
  3743/3743+集成 13/13+定向 44/45（树含 022 双 lang 提交）+ 023 work 收口
  desktop_mcp 58/0+vue 零 TS 错（auto-os 侧在 `d616fa7` 之后）+ a2vue 金样
  对拍重生成（auto-lang `0407a9f9b`，desktop.at 022 网格终版双端同源）|
  局限声明：运行时行为未现场重演（ui_desktop 未运行），结论重建自工件：
  计划内执行期用户实测记录 + HEAD 代码标记 + 祖先覆盖回归 | `next: merge`

- 2026-09-17（merge 清偿）：`stage: merge` | PLAN-022:r1 | `outcome: pass` |
  **prepared**：reviewed 基线 f357db7；实现已先行落库（回补形态，
  auto-os `d616fa7`/auto-lang `94bc69c22`+`9bd26d884`，复审 merge-base
  祖先实证）→ 按 legacy 规则建专属沉淀 worktree `.wt/os-022/auto-os`
  （分支 `plan-022-dev`，自 f357db7）；frozen delta=SD-01/SD-02。
  **landed**：worktree `088bc6e`（docs/specs/shell/showdesk-icons.md 新增
  71 行 + apps/028-launcher/SPEC.md 布局语义节 +14 行 + 台账五条）ff 落地
  **main 088bc6e**——delivery_commit=088bc6e（reviewed f357db7 的文档/
  投影纯增量 +145 行，实现/依赖零变化，免复审条件核对过）；落地后校验：
  spec 文件在位、台账 JSON 读回 80 条。**ledger_refreshed**：
  .autoos/specs.json 五条（reports/architecture/designs/tests 各 P022-1 +
  reviews PLAN-022-r1），75→80 条读回过（reports 15/architecture 22/
  designs 11/tests 15/reviews 17/goals 0——goals 空影响有说明）。
  **archived**：docs/plans/archive/022-icon-taskbar-polish.md（git mv），
  completion_kind: delivered。**cleaned**（同日清偿）：worktree 树净 +
  `plan-022-dev` 已并入 main 核验 → wt-guard 过闸（零 reparse point）→
  worktree `.wt/os-022/auto-os` 移除，分支 `plan-022-dev` 删（was
  088bc6e=main 已含），组目录 `.wt/os-022` 移除，`.wt` 零残留。五检查点
  prepared/landed/ledger_refreshed/archived/cleaned 全 closed。

## 新会话交接（2026-09-15 收尾快照）

**当前状态**
- 桌面实例：ui_desktop（最新构建，含本计划全部改动）由上一会话拉起运行中。
- 改动全部**未提交**：
  - auto-os：`shell/shell.at`、`shell/desktop.at`、`apps/028-launcher/src/front/app.at`、
    `assets/icons/`（10 新 stem + 4 重制 + mapping.json）、`docs/plans/022-*`；
  - auto-lang：`renderer.rs`、`session.rs`、`examples/ui_desktop.rs`；
  - 用户目录 `D:utostackssets\{light,dark}`（仓外，已重命名规范化）。
- 诊断残留：renderer.rs 两条 `[ws-preview]` eprintln（提交前拆）。

**重建 / 运行**
- `cd D:utostackuto-lang && cargo build -p auto-lang --features ui-iced --example ui_desktop`
- 运行：`AUTO_OS_ROOT=D:/autostack/auto-os`（+ `AUTO_VM_STORAGE_FILE=%USERPROFILE%/.config/autoos/desktop-storage.json`）
  `target/debug/examples/ui_desktop.exe`；shell/*.at 与 assets 均运行时直读主检出。

**验证手段（本会话实测有效）**
- 独立 MCP 实例：`AUTOUI_MCP_PORT=9531` 起 ui_desktop → `autoui_find/autoui_state/autoui_screenshot`
  （9247 留给主桌面）；autoui_find 按 label 子串匹配（label 含  PUA 包装）。
- `window::screenshot` 在桌面窗口**最小化时必失败**（含 MCP autoui_screenshot 前置护栏）——
  验证截图类功能前先还原窗口。

**待用户反馈/后续项**
- 拖拽手感（阈值/幽灵/落点高亮）实测；切换器预览深色主题对比度（待澄清 5）；
  kanban/musk 新旧图取舍（待澄清 2）；诊断日志拆除后分组提交。

## 待澄清事项

1. **提交**：全部改动未提交（auto-os：shell/*.at、launcher app.at、assets/icons、
   mapping.json、docs/plans/.next-id+本文件；auto-lang：renderer.rs、session.rs、
   ui_desktop.rs）。两条 `ws-preview` eprintln 诊断日志建议提交前移除。
2. `D:\autostack\assets` 内 `kanban-light/dark.png`（01:49 旧对）与 `kanban.png`
   （02:55 新对，已采用）并存——旧对未删，取舍归用户。
3. 切换器预览的真实窗缩略图依赖快照抓取入缓存（SWR）；若某些窗口始终出占位块，
   下一步排查 snapshot 抓取臂。
4. 拖拽幽灵/落点高亮为逻辑实现 + 结构验证，交互手感待用户实测反馈。
5. 切换器预览（R16）：数据/抓取/重建链已验证（日志 `tiles=[("0",1)]` + widget 收到 `tiles=1`）；深色主题下深色窗缩略贴深色底对比度低，后续可给预览底换浅色 surface 或给瓦片加 1px 描边提升辨识度。
6. 后续新需求追加至本表（R17+），实施走 /auto-plan-work。

## 2026-09-15 二次核查：桌面启动链 + file-manager 实测（用户问询驱动）

**用户问题**：桌面里的 File-manager 还是旧版？为什么不会自动更新？——要求核查
虚拟桌面链接/启动 app 的机制。

**启动链结论（源码定位）**
- **发现（boot 期一次定格）**：`app_registry::aggregate_scan`（renderer.rs:12810
  起）——主根 = auto-lang `examples/ui` + 外部根（storage `shell.apps.extra_dirs`
  + `../auto-os-config/auto` + `../auto-os/apps` 容器 + apps.manifest repo 条目
  ，`host_extra_roots()`）；按 id 去重**主根优先** → `027-file-manager` 恒解析
  到框架 demo（本机实测 45 entries / 29 desktop-visible）。
- **启动（每次点击现场链接）**：`LaunchApp(id)` → `app_resolver` 闭包**点击时**
  `read_to_string` 入口 .at → `build_dynamic_component` VmBridge 进程内链接成窗
  。零缓存、零编译产物——源码改动下次打开即生效。本会话实证：015-notes 在旧
  实例 storage 留有 `api.update_note` Undefined 历史 error，新实例现场链接正常。
- **不会"自动更新"的三件事**：① 注册表快照 boot 期定格——新增 app/新 mapping
  要重启桌面才入册；② **已开着的窗不热更**——旧代码驻留到关窗为止（用户看到
  "旧版"的最可能来源：03:22 PLAN-618/631 提交前开着的 file-manager 窗一直驻留
  ）；③ renderer/shell 本体（auto-lang crates）改动要重建 exe + 重启。
- **file-manager 现状实测（更正）**：桌面/独立启动均解析**主检出 master 源**
  ，其 file-manager 止步 PLAN-618/631（TreeView 无"此电脑"）。用户所指新版
  （侧栏"此电脑"列 C:/D:）= **PLAN-016 r2 Phase 2 批次，提交在 `os-016-dev`
  分支**（worktree `.wt/os-016/auto-lang`，末批 2026-09-14 19:06 beb1ad124
  + 修复轮至 6c488c136，已同步过一次 master）——**工作零丢失**，PLAN-016
  （docs/plans/016-file-manager-revamp.md）状态 `executing` 10/18，本就未到
  合并步。桌面显旧 = 分支未合并，非 worktree 事故。合并时 crates 有改动
  （renderer/session/app_registry/vm native +197 行）→ 须重建 ui_desktop。

**R17 候选（iced VM 轨 popover 闭合态漏染，两处独立实证）**
- **已立 R17 为激活条带下移**（用户截图裁定"条条贴住高亮框、下方尚余 1-2px，
  建议下移 1px"）：shell.at 五处条带臂 `mt-0.5`→`mt-[3px]`（pinned 焦点/运行/
  透明占位三态 + 窗口条目二态），注释在案；重启实测 3x 放大截图确认缝隙。
- popover 漏染两条待用户裁定后立 R18/R19 并走 /auto-plan-work。（2026-09-15
  追注：PLAN-016 修复轮 5 `35712be35` 在 os-016-dev 已做 popover trigger/content
  拆分重构——漏染疑在分支已解，合并后桌面复核再定 R18 立废。）
- file-manager 窗底漏染"新建文件夹 / 确认删除项目？"文本：app.at:1108/:1140
  两枚坐标锚 popover（`open` 门控为假）内容散落渲染到窗底左（仅文本，无容器
  底/输入件）。
- 桌面左下角漏染"更换壁纸…"：desktop.at blank/icon 右键菜单 popover 同族——
  PLAN-002 曾修过"闭合括号错位 → view 级散落子树、左下角常驻"同款（注释在案
  ），疑复发或另一菜单臂。
- 归因方向：renderer 坐标锚 popover **closed 态渲染臂**（修复面在 auto-lang
  renderer.rs，非 app 侧结构）；复验路径：AUTOUI_ACCEPTANCE=1 + MCP
  autoui_desktop bus activate / autoui_screenshot（本节实例日志
  tmp/fm-check/desktop-accept.log）。
