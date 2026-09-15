---
plan_id: PLAN-022
status: execution_done        # 回补记录：工作先行于计划（直接用户请求驱动），复审未做
feature_name: icon-taskbar-polish
author: [ZCode 会话 2026-09-15]
created_at: 2026-09-15
updated_at: 2026-09-15

# /auto-plan:review 结束时填写：
supersedes_spec_components: []
new_spec_components: []
touched_goals: []             # 引用 docs/specs/goals.md 的 GOAL-NNN

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

- 未复审（回补记录；如需 review 走 /auto-plan:review）。

## 待澄清事项

1. **提交**：全部改动未提交（auto-os：shell/*.at、launcher app.at、assets/icons、
   mapping.json、docs/plans/.next-id+本文件；auto-lang：renderer.rs、session.rs、
   ui_desktop.rs）。两条 `ws-preview` eprintln 诊断日志建议提交前移除。
2. `D:\autostack\assets` 内 `kanban-light/dark.png`（01:49 旧对）与 `kanban.png`
   （02:55 新对，已采用）并存——旧对未删，取舍归用户。
3. 切换器预览的真实窗缩略图依赖快照抓取入缓存（SWR）；若某些窗口始终出占位块，
   下一步排查 snapshot 抓取臂。
4. 拖拽幽灵/落点高亮为逻辑实现 + 结构验证，交互手感待用户实测反馈。
