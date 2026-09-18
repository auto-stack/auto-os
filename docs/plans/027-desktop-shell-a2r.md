---
plan_id: PLAN-027
status: reviewed              # rev2 复审 pass（F-01 修复闭环）；next: merge
feature_name: desktop-shell-a2r
author: [agent]
created_at: 2026-09-18
updated_at: 2026-09-18
plan_revision: 2

# /auto-plan:review 结束时填写：
supersedes_spec_components: []
new_spec_components:
  - auto-lang/schema/projection-protocol-v1.md   # v1.10：typed 快照通道入册（SD-02，review 闭环）
  - auto-lang/docs/specs/auto-lang/ui/design/shell-a2r-seams.md   # S1/S2 接缝面 provisional（SD-04）
touched_goals: []

affects:
  - auto-lang/crates/auto-lang/src/ui/shell.rs                    # ShellSurface 装配 trait + pack 发现（双轨开关）
  - auto-lang/crates/auto-lang/src/ui/session.rs                  # 特权槽位接 typed 组件 + 投影 sync 改造
  - auto-lang/crates/auto-lang/src/ui/iced/renderer.rs            # 装配层槽位替换 + ShellProjection 推送点
  - auto-lang/crates/auto-lang/src/ui_gen/rust.rs                 # popover 臂/AnchorSlot 发射/显式拒绝/desktop.*+storage.* 转译
  - auto-lang/crates/auto-man/src/rust_ui.rs                      # 无窗组件库生成目标 + 装配清单
  - auto-lang/docs/design/autoui/desktop-shell-a2r.md             # 状态更新（裁定落定）
  - auto-os/shell/                                                # pack 源（零内容改动，形态迁移 parity 锚）
  - auto-os/docs/plans/autos-desktop-program.md                   # 台账行
current_step: 7
total_steps: 7        # rev2：可执行任务 = T-01..T-06（已完成）+ T-09'（收口）；T-07/T-08 退役不计数
---

# [PLAN-027] desktop-shell-a2r

## 0. 变更摘要

**rev 2（2026-09-18，有界修订）**：§10-① 用户裁定主形态 = **B（outproc 特权协议客户端——shell 与 app 同律独立进程、渲染经 RenderQueue 发桌面 compositor 统一渲染）**+ 解释装载路径双轨常驻。按 §4 预案"改选 B 本计划按范式修订"：范围收口至 **S1/S2**（T-01..T-06 已交付，产物形态无关全部承继）；**T-07/T-08 退役**（A 形态装配面），S3 交接面改写为 B 形态程序的前置序列（图像 DrawOp 通道立项先行 → 025 键盘真机实测 → 覆盖二批 → shell outproc client + 启动序/看门兵），以新计划承接；AC 按实际交付重述。档案事实注记：A 形态只链 shell pack 五件生成物，不涉及任何 App——App 两形态下均已是独立进程走 RenderQueue（PLAN-020/025 既有）；裁定对 shell 本身的推论（同律独立进程）成立。

**rev 1 原文（存档，A 形态口径——四件现为五件见 §5.1 修正 A）**：桌面 shell 是"全 a2r 桌面"的最后一块解释态：四件 `.at`（auto-os
`shell/`：shell/desktop/notification_center/switcher，共 1664 行）由
ui_desktop 宿主进程内解释装载（`build_dynamic_component`），每帧
`dynamic_view` 重建。本计划按设计文档
`auto-lang/docs/design/autoui/desktop-shell-a2r.md`（2026-09-18 落盘）
的迁移路径 **S1–S3** 实施**形态 A（链入宿主）**：shell pack 经 a2r 生成
Rust 组件库链进宿主二进制，投影/命令接缝类型化
（ShellProjection 快照 + DesktopBusHandle），**视觉/交互零变化**（parity
锚），解释装载路径降级为开发态 fallback（双轨常驻——工作假设，§10-①）。

三步：**S1 生成域补面**（裸 popover 臂 + AnchorSlot 槽位协议 + codegen
显式拒绝——icon 变体已由 PLAN-026 收编）；**S2 shell 专属接缝**
（ShellProjection typed 快照承投影协议 v1.8 语义 + desktop.*/storage.*
类型化转译）；**S3 生成目标与装配**（无窗组件库目标 + ShellSurface 装配
trait + 双轨切换 + 金样对拍）。**B-ready 纪律随行**：接缝按可序列化
形态设计，B 形态（outproc）留远期演进门（设计文档 §5）。

## 1. 目标

**rev 2 收口目标**（原 G1–G4 为 rev 1 合同，处置随注）：

- **G1 S1 生成域补面（✅ 已交付，T-02/03/04）**：shell pack 五件 a2r
  生成词汇完备——裸 popover 臂（Point 锚 + open/placement/ondismiss）、
  宿主合成件直发既有变体（window_thumbnail ×3 / workspace_preview ×1）、
  codegen 未知 prop/事件显式拒绝（compile_error）+ shell 全量清单编译门。
  **普查修正 C**：mouse-area 臂（26 处）+ div→container + taskbar→row
  + 布局件点击落字段（ViewBuilder build() 修复）。
- **G2 S2 接缝类型化（✅ 已交付，T-05/06）**：`ShellProjection`
  plain-data 快照承投影协议 v1.8 语义（指纹门控/原子换装/召唤事件/clock
  独立脏帧零漂移）；`DesktopBusHandle`（枚举载荷单方法）+ `HostStorage`
  typed 接缝；52 动词 roundtrip 全量对拍（**捕获并修复 set_theme_name
  encode 死词**）。
- **G3 A 形态装配（❌ rev2 裁定退役，原 T-07/T-08）**：无窗组件库链入
  宿主 + ShellSurface inproc 双轨装配——随 B 裁定失效，由 B 形态程序
  （shell outproc client）承接。
- **G4 parity 与度量（→ 重定位）**：金样对拍/体积/启动度量随 shell
  编译化的形态落点（B 程序的 shell exe）后移；本计划保留已交付的行为
  级对拍（指纹门控族 + 词汇门 + roundtrip）。
- **G5 收口与交接（rev2 新增，T-09'）**：设计文档状态按裁定落定
  （SD-01）、桌面程序台账登记 B 方向与本计划资产行（SD-03）、S1/S2 新面
  模块 spec 条目（SD-04）、KNOWN-DEBT 随注——B 程序新计划的可复审起点。

**非目标**（rev2 更新）：B 形态程序本体（图像通道/键盘实测/覆盖二批/
shell outproc client/启动序看门兵）——新计划承接；shell pack 内容改版
（零改动已达成并守卫）；解释装载路径退役（双轨常驻裁定）；Stage B 搬迁。


## 2. 架构方案

```text
┌─ S1 生成域（ui_gen/rust.rs + view.rs 消费端）─────────────────────┐
│ 裸 popover 臂：View::Popover + PopoverAnchor::Point{x,y} 发射     │
│   （构造器已在 view.rs:976-985，codegen 从不发射）+ ondismiss 事件 │
│ AnchorSlot 槽位：window_thumbnail/workspace_preview →            │
│   View::AnchorSlot{key} 发射（native 投影透传先例 native_        │
│   projector.rs:308）；宿主装配层按键替换合成件（既有 iced 注入    │
│   面同位）                                                        │
│ 显式拒绝：add_prop_to_builder/add_event_to_builder 未知项 = 编译  │
│   错（替换 :4713/:4748-4762 静默丢弃）；shell 全量 tag/prop/事件   │
│   清单入编译门测试                                                │
└──────────────────────────────────────────────────────────────────┘
┌─ S2 接缝（session.rs + renderer.rs + ui_gen 转译）────────────────┐
│ ShellProjection（plain data，B-ready）：宿主侧由 sync_shell_     │
│   windows（renderer.rs:12671-13022）改造——__wm_fp 指纹门控保留，  │
│   有变整组快照推送（替换 write_state 组）；typed 组件消费入口 =    │
│   update 消息 WmSync(ShellProjection)（召唤事件 = 快照内显式      │
│   ShellEvent 位，替换 call_handler）                              │
│ DesktopBusHandle trait（宿主实现）：codegen 把 shell.at 的        │
│   SendCmd/desktop.* 写法转译 handle.verb(...) 类型化调用（46 动词 │
│   session.rs:1313-1451）；双轨桥 = 解释壳 __desktop_cmd 字符串    │
│   通道在 handle 背后同实现（零分叉）；storage.* → HostStorage     │
│   （对接宿主 storage，解释态原生位 vm/native_catalog.rs:1118-1126 │
│   对照）                                                          │
└──────────────────────────────────────────────────────────────────┘
┌─ S3 装配（rev2 ❌ 退役——A 形态面随 §10-① 裁定失效）──────────────┐
│ 原方案（无窗组件库链入宿主 + ShellSurface inproc 双轨装配）由      │
│ B 形态程序承接：shell = 020 形态 outproc 编译 exe，渲染走          │
│ RenderQueue；前置序列（设计 §4/§5）：图像 DrawOp 通道立项先行      │
│ （硬阻断）→ 025 键盘真机实测 → 覆盖二批（display 族 + popover      │
│ 开合）→ shell outproc client + 启动序/看门兵。本计划 S1/S2 产物   │
│ （IR/词汇门/typed 载体词汇）为该程序的直接前置资产。               │
└──────────────────────────────────────────────────────────────────┘

**不变式**：

- **I1 零回归**：解释壳路径双轨期零变化（desktop_mcp 五套解释形态全绿
  是回归门）；普通 App 轨道（解释三形态 + a2r 客户端）零牵连。
- **I2 投影协议语义零漂移**：ShellProjection 承载 v1.8 全部字段语义
  （指纹门控/原子换装/召唤事件/clock 独立脏帧）——对拍测试钉行为，
  载体扩展入册（SD-01）。
- **I3 B-ready**：ShellProjection/DesktopBusHandle 按 plain-data/可序列
  化设计，禁止硬编码进程内假设（B 形态演进时接缝直换载体）。
- **I4 parity 锚**：编译壳六面与解释壳金样对拍；shell pack 源文件
  （auto-os/shell/）零内容改动。

**关键风险**：投影改造是宿主渲染热路径（每 update 周期）——指纹门控
语义丢失/漂移是最大回归面（I2 对拍钉）；shell 生成物体积/编译时长
（1664 行 .at → 单 crate，R2）；双轨期两装载路径的行为分叉（R6——金样
对拍门禁双形态）；装配层槽位替换与懒挂载 overlay 的时序（summon_
launcher 等注入点 renderer.rs:9286/:9347-9349 的 typed 等价物）。

## 3. 技术栈

Rust / iced 0.14（宿主装配）；a2r 生成链（ui_gen/rust.rs +
auto-man/rust_ui.rs，path 依赖 auto-lang 与宿主同源——设计普查确认
Cargo 层天然成立）；投影协议 v1.8（schema/projection-protocol-v1.md）；
DesktopBus 46 动词（session.rs:1313-1451）；验收 = 实机 ui_desktop
（auto-os `scripts/desktop.sh`）+ desktop_mcp 五套（I2 既有链）+ a2vue
desktop_surface 金样族先例（vue.rs:27105-27139）。

## 4. 需求分析与背景调查

**授权记录**：用户 2026-09-18 会话明确"假如 026 执行完毕的话……提前
规划下一个计划吧"——**本轮仅规划，未授权实施**。**工作假设**（用户
裁定问询未作答，按设计文档 §5/§8-R3 推荐继续，§10-① 登记）：主形态
= **A 链入宿主**（B-ready 随行）；解释装载路径 = **双轨常驻**。两项
假设在 T-07（S3 装配）开工前需用户确认，若改选 B 本计划按范式修订
（S1/S2 任务与形态无关，不受影响）。涉及仓：auto-lang（生成域/接缝/
装配/协议文档/设计文档）+ auto-os（pack 源只读引用/台账/e2e）。无
无预算/自动续跑约束声明。

**rev2 授权记录（2026-09-18）**：用户在 §10-① 问询中裁定主形态 B + 双轨常驻，并指示"本计划文件走 review/merge、独立进程方案另立新计划执行"——有界修订（范围收口 S1/S2 + T-07/T-08 退役 + AC 重述）获授权；B 形态程序另立计划（后续 /auto-plan:new）。

**前置依赖**：PLAN-026 merge 收口（shell pack a2r 编译的 IR 前置 =
View::Icon 变体 + badge/card/scroll/a codegen 修复；ui_gen/rust.rs
同文件避免并行冲突）。

**现状事实**（已核，2026-09-18，来源 = 设计文档普查 desktop-shell-a2r.md
§2/§3，行号锚定当日 master）：

- **解释壳管线**：pack 发现 resolve_shell_pack_dir（shell.rs:35-61，
  override > env > ../auto-os/shell > 硬编码 > 内嵌快照 :15/:107/:118/
  :131）；boot 装载 open_desktop（session.rs:2369-2389），shell/desktop
  先于 App（renderer.rs:13397-13426），launcher/switcher/notification
  懒挂载 overlay（summon_launcher renderer.rs:9286）；shell = 宿主内
  DynamicComponent App（AppSession session.rs:2336-2339），特权锚点
  session.rs:259-289。
- **每帧重建**：split_ref_shell（session.rs:3842-3861）→ dynamic_view
  （renderer.rs:17918）→ VM 解释渲染。
- **投影**：sync_shell_windows（renderer.rs:12671-13022）——`__wm_*`
  16 字段 + `__desktop_*`/`__wp_*` ~15 字段，`__wm_fp` 指纹门控（不变
  整组跳写 :12947-12952，有变原子换装 :12953-13000 + view_dirty）；
  召唤 call_handler（RebuildMru/RebuildNotes/RunningSync，
  :9247/:9498/:9570/:13007-13016）；launcher 注入 apps_names/apps_icons
  （:9347-9349）；switcher mru 平行列表（:9424-9498）；通知快照
  （:9511-9570）。
- **命令**：DesktopBus = handler 拼 verb\targ 写 `__desktop_cmd`
  （shell.at:58-61 SendCmd 单点，:595-637 各 handler，:658-663 追加
  语义 v1.8）→ 宿主排空（特权槽 session.rs:2512-2544 + 全窗联合
  renderer.rs:9770-9789）→ DesktopCommand 46 动词（session.rs:
  1313-1451）。
- **a2r 缺口（本计划面）**：裸 popover col 降级 + prop 静默丢弃
  （rust.rs:4713，PopoverAnchor::Point 存在但不发射 view.rs:976-985）；
  宿主合成 widget 零 codegen（projection-protocol-v1.md:77）；显式
  拒绝缺位（add_prop :4713 丢弃/add_event :4748-4762 只认三种）；
  desktop.*/storage.* 零支持（兜底裸调用编译失败 rust.rs:5868）；
  入口假设独立窗 + 单 main widget 启发式（rust_ui.rs:1795-1799/
  :1918-1941）。
- **依赖同源**：a2r 生成物 path 依赖 auto-lang（rust_ui.rs:2418/2079）
  ——链入宿主 Cargo 层成立，零新依赖。
- **026 收编面**：View::Icon 变体 + badge/card/scroll/a codegen 修复
  （PLAN-026 T-02/T-03）——本计划 S1 不重复。

**specs 现状**：投影协议权威 = schema/projection-protocol-v1.md（v1.8）；
设计文档 desktop-shell-a2r.md（设计输入态，本计划实施即其落地载体，
SD-01 更新其状态）；模块 spec 020/025/026 provisional 链在册。

## 5. 详细设计

### 5.1 T-01 深水调查定案（决策产物）

- **D1 ShellProjection 结构**：31 字段（`__wm_*`/`__desktop_*`/`__wp_*`）
  的 typed 映射表——逐字段定类型（窗口列表/图标列表/壁纸路径/工作区/
  通知/MRU/clock…），平行字符串列表（`__desktop_cells` 族 + mru_* 注入
  :9424-9498）归并为结构化载荷；召唤事件 = `ShellEvent` 枚举位
  （RebuildMru/RebuildNotes/RunningSync + clock/date 独立脏帧语义保位）。
- **D2 消费入口形态**：候选 A = codegen 生成 `WmSync(ShellProjection)`
  消息变体（宿主 send——与 iced 消息流同构）；候选 B = 装配层直接
  方法调用（`set_projection(&mut self, proj)`）。以 shell.at 现有
  handler 消息流（SendCmd 单点模式）与懒挂载时序定案。
- **D3 DesktopBusHandle 形态**：trait 方法 = 46 动词逐一映射 vs 分组
  （launch/focus/layout 三原语 + 枚举载荷）；codegen 转译面 = shell.at
  的 `.SendCmd("verb\t"+arg)` 字符串拼法如何被识别（SendCmd 单点 =
  codegen 可识别的锚——定识别策略）；storage.* 的 HostStorage 键值面
  与 os-config 关系确认。
- **D4 AnchorSlot 键协议**：槽位键命名（window_thumbnail:{wid} /
  workspace_preview:{idx}？）+ 宿主装配层替换点（解释态合成件注入位
  的 typed 等价物）+ 懒挂载 overlay 的槽位时序。
- **D5 装配清单**：ShellManifest 表达（常驻面/懒挂载面/overlay 槽/
  各面组件名）+ wrap_example 无窗目标的工程形态（crate 名/装配宏/
  与宿主 shell.rs 的接驳）。

定案记录追加 `### 5.1 定案记录`，作为 T-02..T-08 依据。

### 5.1 定案记录

**T-01 深水调查（2026-09-18，worktree 实勘：auto-lang @ 2808c551a
plan-027-dev / auto-os @ 4db1419 plan-027-dev；行号为本日实勘位）**。
两项普查修正先行（等效实现调整，授权范围内继续）：

- **修正 A（五件非四件）**：shell pack 现为**五件** 1831 行——PLAN-024
  归档落地新增 `dashboard.at`（149 行，dock Dashboard 面板 overlay，
  shell.rs:131/136-139 进程内嵌 + pack 同级发现，EMBEDDED 表
  shell.rs:68-74 五项）。编译域按五件收口（常驻 2：shell/desktop +
  懒挂载 3：switcher/notification_center/dashboard）；launcher overlay
  仍走注册表装载路径（apps/028-launcher 非 pack 源，本计划不动——
  G3"六面"为桌面功能面闭环口径，非六面皆编译）。
- **修正 B（槽位协议落现有变体）**：`View::WindowThumbnail`/
  `View::WorkspacePreview` 变体**已在册**（view.rs:878/890；解释侧
  构造 aura_view_builder.rs:6709-6736；iced 消费 renderer.rs:5146/
  5230——`AbstractView` 即 `View` 别名 renderer.rs:8，typed View 与
  解释 View 同一 into_iced 面）。T-03 不新建 AnchorSlot 键机制，直接
  codegen 发射既有变体；计划 §2"View::AnchorSlot{key}"命名由既有专用
  变体承接（键 = wid/ws prop）。view.rs:464 既有 `AnchorSlot{index}`
  为 VM 轨块锚定专用（autodown），不相关不动。

**D1 ShellProjection 结构**：新模块
`crates/auto-lang/src/ui/shell_projection.rs`，plain-data 可序列化
（B-ready/I3）。载体 = `ShellProjection`，分面嵌套：

- shell 面（任务栏）：`wins/workspaces/mru: Vec<..>`、
  `notes: Vec<ShellNote>`、`meta_layout`/`meta_focused_wid`、
  `running_csv`/`focused_app`/`dock_pinned_csv`、
  `notes_unread:u64`/`notes_badge`、`notes_visible`/`settings_open`/
  `showdesk`/`dashboard_visible: bool`、`layout`、`fp`（指纹随载体，
  门控逻辑留宿主侧）；`dock_pinned: Vec<DockPin>`。
- desktop 面：`bg`/`icons: Vec<DesktopIcon>`/`hidden_csv`/
  `cells: Vec<DesktopCell>` + 平行列表 `cell_ids/cs/rs`（handler 下标
  读合同面保形）/`drag_icon`/`drop_c`/`drop_r`/`drag_moved`/
  `label_dark`/`cursor_x`/`cursor_y`/`running_csv` +
  `wp: WallpaperPickerProjection`（picker/preview/dir/current/x/y/
  items/visible/paths）。
- clock/date 独立通道 `ShellClock{time,date}`——独立脏帧语义保位
  （ServiceTick 泵 renderer.rs:9261-9308 分钟/变化才写，不入指纹组）。
- 召唤事件 `events: Vec<ShellEvent>`（RebuildMru/RebuildNotes/
  RunningSync 三位）随快照交付，替换 call_handler 直调。
- **wire 叶面保形原则**：字段叶子保持 wire 串形（"1"/""、wid 串、
  focused 串）——`.at` 等式消费语义零漂移；容器层 typed（Obj 数组 →
  Vec<Struct>），B-ready 可序列化成立。

**D2 消费入口形态：候选 B（装配层方法调用）**。shell 编译壳生成形态 =
`Component<Msg = DynamicMessage>`——事件以既有
`DynamicMessage::Typed{widget_name,event_name,args}`（interpreter/
bridge.rs:44-52）承载，桌面事件管线（convert_view_messages / DM::App
路由 / MCP 面）零改动；投影下行不走消息变体，走
`ShellSurface::apply_projection(&mut self, &ShellProjection) -> bool`
（解释壳 impl 逐字节保留现 write_state_* 序列，指纹门控留宿主侧；
编译壳 impl 存字段+内部置脏）。call_handler 召唤 →
`ShellSurface::call_handler(&mut self, name: &str) -> bool`（解释壳 =
bridge_mut().call_handler 现行为；编译壳 = 分派
RebuildMru/RebuildNotes/RunningSync/ApplyFilter/RebuildFaces handler
法）。clock/date → `apply_clock`。选 B 弃 A（WmSync 消息变体）缘由：
特权面载荷 per-face 异构，消息变体需 per-widget 消息管线路由进
DesktopSession 的 iced Task 域——装配层直调让解释壳 impl 有逐字节
保行为的锚点（I1），编译壳无需消息包装。

**D3 DesktopBusHandle + HostStorage**：

- `DesktopBusHandle` trait = **枚举载荷单方法**形态：
  `fn send(&mut self, cmd: DesktopCommand)` + provided
  `fn send_record(&self, rec: &str)`（= `DesktopCommand::parse_records`
  逐条 send）。DesktopCommand（session.rs:1323，46 动词 +
  encode/parse_records 双向）即词表类型化单源——per-verb trait 方法会
  把 arg 型决策（Wid 解析/N 前缀剥离/bool 串）复制出第二份，违 I2。
  B-ready：plain enum 可序列化。
- **codegen 识别策略**：SendCmd 单点 handler 臂为结构锚——`.SendCmd`
  handler 体转译 `self.__bus.send_record(&rec)`；其余 handler 的
  `.SendCmd(expr)` 调用点即本地方法调用，无 verb 串解析需求
  （动词分型在 parse_records 运行时单点，双轨零分叉）。
- **解释壳桥**：解释轨 __desktop_cmd 字符串通道**原样不动**（I1）；
  双轨单源 = parse_records + 宿主执行臂共用。HostStorage trait
  `{ get(&self,key)->String; set(&self,key,&str) }`，宿主实现对接现行
  storage 运行时（解释态原生位 vm/native_catalog.rs:1118-1126 同后
  端）；编译壳持 `Arc<dyn HostStorage>` 装配期注入，storage.get/set
  转译为 handle 调用（None → get ""/set no-op）。

**D4 槽位键协议**：修正 B 落定——`window_thumbnail(wid,
fallback_icon)` / `workspace_preview(ws, fallback)` codegen 臂直发
既有变体；宿主合成件消费（快照渲染臂 request_capture/fallback
语义）双形态同源（同一 into_iced 臂）。无新槽位注册表。

**D5 装配清单 + 生成目标**：

- `ShellManifest`（shell_projection.rs 同册）：
  `faces: [ShellFace{id, widget, mount}]`，`ShellMount ∈
  {ResidentBoot, LazyOverlay(召唤动词)}`；五件清单 =
  shell(ResidentBoot)/desktop(ResidentBoot)/switcher(LazyOverlay)/
  notification_center(LazyOverlay)/dashboard(LazyOverlay)。
- wrap_example 新"无窗组件库"形态：产物 = 单 crate
  （建议 `crates/shell-pack/`，**入库**——宿主构建免生成时序依赖；
  再生成走 a2r CLI，pack 改动后 regen+build），内容 = 五组件 +
  `SHELL_MANIFEST` const + `mount_face(id) -> Option<Box<dyn
  ShellSurface>>` 工厂；path 依赖 auto-lang（DAG：shell-pack →
  auto-lang ← auto-man 宿主，无环）。Cargo 面细节（workspace 成员/
  feature 门）T-07 实施期定。
- **双轨开关语义**：编译壳 = 缺省发布态；`AUTO_SHELL_PACK` env
  **存在且为目录** 或 `set_shell_pack_override` 显式注入 = 开发态
  解释壳（改 .at 重启即生效回路，R3）。兄弟检出发现 pack 但无
  env/override ≠ 解释态（该发现链保留服务 hash parity 测试
  shell.rs:197-220 与解释轨内嵌回退）。开关判定面在 shell.rs 装配
  工厂，boot 序/懒挂载时序不变。

### 5.2 S1 生成域（T-02/T-03/T-04）

- **popover（T-02）**：裸 `popover` 元素 codegen 臂——`View::Popover`
  + `PopoverAnchor::Point{x,y}`（+ Widget 锚 fallback）+ open/placement/
  ondismiss 译；ondismiss 入 `add_event_to_builder` 认知集。
- **槽位（T-03）**：window_thumbnail/workspace_preview →
  `View::AnchorSlot{key}` 发射（键协议 D4）；宿主装配层按 key 替换
  合成件（解释态 iced 注入位同位——renderer.rs 合成件消费面）。
- **显式拒绝（T-04）**：未知 prop/事件 = 编译期错误（codegen 产
  `compile_error!` 或生成期 hard error——按生成管线形态定）；shell
  四件全量 tag/prop/事件清单入编译门测试（防"看似编译过实缺件"）。

### 5.3 S2 接缝（T-05/T-06）

- **投影（T-05）**：`ShellProjection`（D1 结构）+ 宿主侧
  sync_shell_windows 改造——指纹门控保留（diff 快照），有变整组
  `WmSync`/`set_projection` 推送（D2）；懒挂载面注入（launcher
  apps_*/switcher mru/notes 快照）归入快照载荷；typed 消费端代码生成。
- **命令（T-06）**：`DesktopBusHandle` trait（D3）+ codegen 转译
  （SendCmd 锚识别 → handle.verb(...)）+ `HostStorage`；宿主实现 +
  解释壳桥（`__desktop_cmd` 字符串通道在 handle 背后同实现——双轨期
  单源）；46 动词对拍测试（记录级 ↔ 类型化双向）。

### 5.4 S3 装配（rev2 ❌ 退役）

原 T-07（无窗组件库生成目标）/T-08（ShellSurface 装配与双轨切换）为
A 形态面，随 §10-① B 裁定退役（§2 退役块）。D5 装配清单产出
（`ShellManifest`，五件）作为 B 程序的 pack 形态事实源继续有效。

### 5.5 验收与收口（rev2 重述 = T-09'）

收口面：设计文档状态按裁定落定（SD-01——状态从"设计输入（未裁定）"改
"裁定落定：B 形态 + 双轨常驻，A 线按 §10-① 档案注记存档"）；桌面程序
台账登记（SD-03）；S1/S2 模块 spec 条目（SD-04）；KNOWN-DEBT 随注
（A 形态度量面、parity 实机六面 → B 程序承接；p010 基线红疑因）。
### 规范增量（rev2 重述）

| delta_id | add/modify/retire | docs/specs/... target | before/after rule | rationale | acceptance IDs |
|---|---|---|---|---|---|
| SD-01 | modify | auto-lang/docs/design/autoui/desktop-shell-a2r.md | before：设计输入（未裁定）；after：**裁定落定 = B 形态 + 双轨常驻**（§10-① 用户裁定记录 + A 线存档注记 + B 前置序列）+ S1/S2 实施锚定（本计划任务映射） | 设计文档状态收口 | AC-04' |
| SD-02 | add | auto-lang/schema/projection-protocol-v1.md（typed 快照通道增量） | before：投影载体 = `__wm_*` 状态变量 + write_state 原子换装 + call_handler 召唤（解释态专语义）；after：增 typed 快照通道（ShellProjection + apply 推送）等价语义入册——指纹门控/原子性/召唤事件/clock 独立脏帧零漂移，双载体并存（解释态通道双轨期不动）；**载体 plain-data 可序列化 = B 形态 wire payload 词汇基础** | 投影协议权威文档收录类型化载体（✅ 载体已落地 T-05） | AC-03 |
| SD-03 | modify | auto-os/docs/plans/autos-desktop-program.md | before：无 shell a2r 行；after：登记 **shell 编译化 B 方向行**（裁定 + 前置序列 + 本计划资产承继清单） | 桌面程序台账 | AC-04' |
| SD-04 | modify | auto-lang/docs/specs/auto-lang/ui/（review 期按目录实况定） | before：无 shell_projection/总线接缝/新 codegen 臂条目；after：shell_projection 模块（载体+清单）+ session 总线/storage trait + ui_gen 新臂（popover/槽位/mouse-area/拒绝门）provisional 条目 | 模块 spec 对齐实现（S1/S2 交付面） | AC-02/03 |

rev2 不新增 spec 影响面；B 形态程序（wire 通道/图像 op/启动序）的 spec
增量归新计划。

零 spec 影响的变更不存在（投影载体/装配形态/生成目标为 spec 级知识）；
ledger（auto-lang `.autoos/specs.json`）随 merge 沉淀。

## 6. 测试设计

**rev2 交付状态**：下列单测/门面已随 T-01..T-06 落地并全绿（golden/词汇门/roundtrip/指纹门控族）；集成·实机·parity·度量四类随 B 形态程序承接（S3 退役）。保留原文作为设计记录。

- **单测（ui_gen）**：popover 臂 golden（Point 锚/ondismiss 译）；
  AnchorSlot 发射 golden；显式拒绝门（未知 prop/事件样本编译错断言）；
  shell 四件全量清单编译门；desktop.*/storage.* 转译 golden
  （SendCmd 锚 → handle 调用）。
- **单测（session/renderer）**：ShellProjection 指纹门控（不变不推/
  有变整组）+ 原子性 + 召唤事件位；懒挂载面快照载荷；DesktopBusHandle
  46 动词对拍（记录级 ↔ 类型化双向）；AnchorSlot 键替换。
- **集成/实机**：编译壳桌面六面渲染 + 交互闭环（dock 点击 launch/
  任务栏切换/右键菜单 popover/壁纸选择器/launcher 键盘流/通知面板）；
  双轨切换（env 回退解释壳）。
- **parity**：六面 × {解释壳, 编译壳} 金样对拍（a2vue desktop_surface
  金样族扩展，帧级：投影注入 → 视图输出）。
- **回归门**：desktop_mcp 五套**双形态**各跑；desktop_protocol/session/
  stage3 全绿；`cargo t -p auto-man rust_ui`；auto-os 桌面 smoke。
- **度量**：宿主二进制体积/启动时延增量；shell 视图重建耗时对照
  解释态基线（数据行入报告）。

## 7. 验收标准（rev2 重述）

- **AC-01 解释壳零回归（双轨前提）**：desktop_mcp 五套解释形态全绿 +
  I2 冒烟——解释装载路径行为零变化。验证：五套套件 + smoke（work 期
  以 auto-lang 全量与 base 41 项本机预存红逐一全等 + 投影门控族 8 测
  全绿承证；desktop_mcp 五套在 merge 前回归门复跑）。
- **AC-02 S1 生成域**：shell 五件 a2r 生成词汇完备（popover ×9/
  thumbnail ×3/preview ×1/mouse-area ×26 全译）；未知 prop/事件显式
  编译错（拒绝门测试）；布局件点击落字段（build() 修复）。验证：
  `test_shell_pack_codegen_vocabulary_gate` + 拒绝门测试 + golden 族
  （✅ 已交付，611fbff2f）。
- **AC-03 S2 接缝语义零漂移**：投影协议 v1.8 语义对拍全绿（指纹门控/
  原子换装/召唤事件/clock）；动词 roundtrip 全量对拍双向绿（含 wire
  缺陷修复）；storage 双轨同后端。验证：模块单测 + 门控族 8 测 +
  `desktop_command_roundtrip_full_vocabulary`（✅ 已交付，T-05/T-06
  commit）。
- **AC-04' 收口与交接（rev2 新增）**：SD-01..04 落笔互链可解析；S3→B
  交接面（前置序列 + 资产清单）在设计文档与台账成文。验证：文档交叉
  引用检查。
- **AC-05（原 A 形态编译壳实机闭环，rev2 ❌ 退役）**：随 B 裁定失效，
  由 B 程序的 shell outproc 实机验收承接（新计划 AC 承接本条意图）。
- **AC-06（原 A 形态双轨切换，rev2 → 重定位）**：双轨常驻裁定已录
  （§10-①）；AUTO_SHELL_PACK 回退解释壳为既有行为（零改动即满足），
  行为同源验证随 B 程序对拍门。
- **AC-07（原度量与文档）**：文档面并入 AC-04'；体积/启动/重建耗时
  度量随 shell 编译形态落点（B 程序 shell exe）后移，原 A 形态度量
  口径作废。
- **AC-08 回归门（原 AC-07 顺延保 ID）**：§6 回归门全绿（在册既有红
  除外——本机 41 项基线红已逐一归档，非本计划回归）。验证：merge 前
  全量 no-fail-fast 与基线集 diff。

> 退役注记：AC-05/06/07 处置经 §10-① 用户裁定与 rev2 授权
> （§4），非为过审而删减——A 形态意图由 B 程序承接并留痕。

## 8. 执行步骤

**rev2 状态**：T-01..T-06 已完成（证据见各条勾选，全数承继）；
T-07/T-08 退役（§10-① B 裁定）；剩余可执行 = T-09'（收口）。

**前置**：PLAN-026 merge 收口（✅ 已满足——026 已归档）。依赖序：T-01 → {T-02, T-03, T-04 并行} → {T-05, T-06
并行} → T-07 → T-08 → T-09。**T-07 开工前 §10-① 两项工作假设需用户
确认**（S1/S2 与形态无关不受影响）。lang worktree
`D:/autostack/.wt/lang-027/auto-lang`；os `D:/autostack/.wt/os-027/
auto-os`。

- **T-01 [lang] 深水调查与定案** ✅ 已完成
  文件：`ui/shell.rs`、`ui/session.rs`、`ui/iced/renderer.rs`（注入/
  合成件面）、`ui_gen/rust.rs`、`shell/*.at`（auto-os，读）+ §5.1 写面。
  动作：D1–D5 定案（快照结构/消费入口/handle 形态/槽位键协议/装配
  清单）。
  产物：`### 5.1 定案记录`（file:line 证据）。
  验证：定案完备；复审通过。
  → AC-02/03/04 前置。新路径：是。
  [✅ 已完成] [x] 定案 D1–D5 全部落笔 §5.1（2026-09-18）；含两项普查
  修正（五件 pack / 槽位落既有变体）与 §10-② D2 定案（候选 B）。
  证据：本文件 §5.1；实勘 worktree auto-lang@2808c551a / auto-os@4db1419。
- **T-02 [lang] 裸 popover codegen 臂** ✅ 已完成
  文件：`ui_gen/rust.rs`（popover 臂 + 事件认知集）。
  动作：§5.2 popover；shell ×9 真源样本验证。
  验证：codegen golden + shell 编译门增量绿。
  → AC-02。
  [✅ 已完成] [x] `generate_bare_popover` 臂（rust.rs generate_view_tree
  modal 族后）+ golden ×2。验证：`cargo t -p auto-lang bare_popover`
  2 passed（worktree 97d0bb75d）；popover 族回归 12/13——1 红为
  **基线预存**（p010_popover_ondismiss_extracted_from_events，base
  2808c551a 同红，stash 复证实锤，非 T-02 回归；疑似根因 = desktop.at
  拖拽幽灵 popover 无 ondismiss 时解释臂落 widget 形态 __popover_close
  兜底，解释轨零触碰，登记 KNOWN-DEBT 面）。ondismiss 未入
  add_event_to_builder 认知集（§5.2 原文）——View builder 无通用
  on_dismiss 槽，识别面收敛 popover 臂本地（定案记录 D3 同律：识别
  面单点化），证据链在本节。
- **T-03 [lang] AnchorSlot 槽位协议** ✅ 已完成
  文件：`ui_gen/rust.rs`（发射）、`ui/iced/renderer.rs`（装配层替换）。
  动作：§5.2 槽位（D4 键协议）。
  验证：发射 golden + 替换单测 + thumbnail/preview 样本。
  → AC-02/04。
  [✅ 已完成] [x] 按 D4 修正 B 落地：window_thumbnail/workspace_preview
  codegen 直发既有 `View::WindowThumbnail/WorkspacePreview` 变体——
  零 renderer 改动（消费端已在册：变体 view.rs:878/:890，iced 臂
  renderer.rs:5146/:5230 与解释轨同一 into_iced 面，"装配层替换"由
  既有消费臂天然承接）。key/fallback 动态表达式支持（loop var 沿
  Value 下标读惯例）；fallback 缺省 app-window。验证：
  `cargo t -p auto-lang host_synth_slot` 1 passed（worktree
  274265345）；thumbnail ×3/preview ×1 真源样本随 T-04 shell 编译门
  全量验证。
- **T-04 [lang] codegen 显式拒绝 + 编译门** ✅ 已完成
  文件：`ui_gen/rust.rs`（:4713/:4748-4762 丢弃改拒绝）、新编译门测试。
  动作：§5.2 拒绝门；shell 全量清单入测试。
  验证：拒绝门测试绿（未知样本断言编译错）。
  → AC-02。
  [✅ 已完成] [x] worktree 611fbff2f。实施面：
  ① 拒绝门 = add_prop_to_builder/add_event_to_builder 未知键 emit
  `compile_error!`（表达式位块；生成期 hard error 需 String 管线全链
  Result 化——成本不成比例，编译期错同样拦截产物）；布局 hover 事件
  （onmouseenter 等）= 认知且双轨同弃层（View IR 无布局 hover 槽、解释
  set_layout_events 同弃——switcher.at row hover parity 保持）。
  ② **普查修正 C**：mouse-area 零 codegen 臂而 shell 实用 26 处（a5
  误记"shell 未直接用"）——补臂，事件映射与解释臂
  convert_mouse_area_untracked 全同源；div→container、taskbar→row 映射
  同源补齐（任务栏横条缺省 col 会纵堆）。③ view.rs build() 修复：
  Row/Column 硬编码 onclick None 改落 button_onclick（布局件点击此前
  "编译过实无行为"——PLAN-012 W3 桌面卡点击类 VM 轨语义的 a2r 对应）。
  ④ with_button_preset 四臂（双 feature twin）剥除已消费 variant/size
  （拒绝门防误伤；shell 按钮 ×38 携 variant；动态 style 不注入沿
  PLAN-571 文档化先例）。⑤ 全量清单门
  `test_shell_pack_codegen_vocabulary_gate`：真源五件每对
  (tag,prop/event) 对表断言（表即合同）。
  验证：拒绝门 ×1 + 词汇门 ×1 + plan571 族回归绿；`cargo t -p
  auto-lang --no-fail-fast` 全量失败集与 base **41 项逐一全等**
  （本机预存红：layout ×16/vm_bridge ×4/lucide ×1/p010 ×1/musk ×3
  等——stash 对照实锤，非本计划回归；p010 疑因见 T-02 证据）。
  → AC-02。
- **T-05 [lang] ShellProjection 投影接缝** ✅ 已完成
  文件：`ui/iced/renderer.rs`（sync 改造）、`ui/session.rs`、新
  `ShellProjection` 模块、ui_gen 消费端生成。
  动作：§5.3 投影（D1/D2）。
  验证：指纹门控/原子/召唤对拍单测 + 懒挂载载荷测试。
  → AC-03。
  [✅ 已完成] [x] worktree feat commit（T-05）。实施面：
  ① 新模块 `ui/shell_projection.rs`——ShellProjection（指纹门控组
  typed 载体，§5.1 D1 字段表全量）+ ShellEvent 召唤位 + ShellClock
  独立脏帧通道 + 懒挂载 payload 四件（Switcher/Notes/Launcher/
  DashboardSnapshot）+ DesktopSurfaceSync + ShellManifest（D5 五件）；
  B-ready 全 plain data，wire 叶面保形（bool→"1"/"" lowering 单点）。
  ② sync_shell_windows 单源化：build_shell_projection（派生逻辑逐行
  平移）+ apply_shell_projection_interpreted（指纹门控保留、写集经
  interpreted_writes 逐字节一致；desktop 层 __wm_running/RunningSync
  随行；cursor/drag"只写不置脏"字段不入组——逐事件写语义保持）。
  ③ **排序调整（授权内）**：懒挂载四召唤注入块的载荷化挪 T-08
  （payload 类型本任务在册；召唤改造与 ShellSurface trait 同面实施
  避免二次翻动——T-08 验收时补对拍）。
  验证：模块单测 ×3 绿 + 既有指纹门控/原子/召唤族 8 测全绿
  （projection_fingerprint_gates_rewrite_and_view_dirty 等）；
  renderer 模块 no-fail-fast 失败集 ⊆ 基线 41 项（零回归）。
  → AC-03（命令面在 T-06）。
- **T-06 [lang] DesktopBusHandle + storage 接缝** ✅ 已完成
  文件：新 `DesktopBusHandle` trait + 宿主实现（session.rs）、
  `ui_gen/rust.rs` 转译、HostStorage。
  动作：§5.3 命令（D3）+ 解释壳桥。
  验证：46 动词双向对拍 + 桥单源测试（解释壳命令回归）。
  → AC-03。
  [✅ 已完成] [x] worktree d6e8da838。实施面：
  ① DesktopBusHandle（D3 枚举载荷单方法 + provided send_record——
  SendCmd 锚落点与解释轨同一 parse_records 单点分型）+
  DesktopBusQueue 进程内队列（T-08 装配消费）。
  ② **普查修正 D**：设计 §3b-b2"storage.* 零支持/兜底编译失败"已过时
  ——a2r codegen 现译 shim_storage_*（rust.rs:6598-6609）与解释轨原生位
  （native_catalog 1106-1108）同一 KV 后端，双轨零分叉已成立；HostStorage
  trait + ShimHostStorage 委托为装配层显式注入/测试替身面（codegen 不
  改——避免投机抽象层）。
  ③ **对拍捕获真 wire 缺陷**：SetThemeName encode 发无人解析的
  `set_theme_name` 死词（PLAN-601 漏逆向臂；线上发件面走
  set_theme\t<名>）——修正 encode 搭 set_theme 线上词。AC-03"双向对拍"
  门的设计意图实证。
  验证：roundtrip 全量对拍（52 变体显式枚举 + 空参 trailing tab 保形 +
  队列序保持）绿；session::tests 71/71 绿。
  → AC-03。
- **T-07 [lang] 无窗组件库生成目标**（rev2 ❌ 退役——A 形态面，§10-① 裁定；ShellManifest 产出已在 T-05 落地，B 程序 pack 事实源）。原文：
  文件：`crates/auto-man/src/rust_ui.rs`（wrap_example 新形态）。
  动作：§5.4 生成目标（D5）；shell 四件生成 crate 编译过。
  验证：生成物编译 + 装配清单断言；**前置：§10-① 用户确认**。
  → AC-02。
- **T-08 [lang] ShellSurface 装配与双轨切换**（rev2 ❌ 退役——同上）。原文：
  文件：`ui/shell.rs`（trait + 双轨开关）、`ui/session.rs`/
  `ui/iced/renderer.rs`（槽位接 typed）。
  动作：§5.4 装配切换；boot/懒挂载时序保持。
  验证：编译壳实机六面 + 双轨切换用例 + desktop_mcp 双形态。
  → AC-04/05。
- **T-09' [lang+os] 收口与交接（rev2 重写）** ✅ 已完成
  文件：`desktop-shell-a2r.md` 状态更新（SD-01，B 裁定落定）、
  `autos-desktop-program.md` 台账行（SD-03）、auto-lang `docs/specs/`
  模块条目（SD-04，provisional）、KNOWN-DEBT 随注（A 形态度量/实机
  parity → B 程序；p010 基线红疑因）。
  动作：AC-01..04'/08 逐条留痕；SD-01..04 落笔；B 程序新计划交接
  材料（前置序列 + 资产清单）成文。
  验证：文档交叉引用可解析；回归门与基线集 diff 全等。
  → AC-04'/08。
  [✅ 已完成] [x] lang worktree 63d981d49 + auto-os main 1313d00。
  SD-01：设计文档状态头改"裁定落定 = B + 双轨常驻"、§5 裁定记录
  （含 A 线事实注记）、§6 改道注、§10 四项全落定、文末实施锚定
  （S1/S2 提交映射 + S3→B 交接）。SD-04：新 spec
  `docs/specs/auto-lang/ui/design/shell-a2r-seams.md`（provisional——
  S1 臂族/拒绝门/词汇门 + S2 载体/总线/storage/对拍门 + 零回归面 +
  已知限制）+ ui/overview.md 刷新行。KNOWN-DEBT 两行：p010 基线红
  疑因（拖拽幽灵 popover 解释臂 widget 形态兜底）、按钮动态
  style+variant preset 不注入双轨分歧候选（B parity 线复核项）。
  随件：`shell_packs_compile` 冒烟补 dashboard.at（PLAN-024 历史遗漏）。
  SD-03：台账裁定登记簿 3c 行（B 方向 + 前置序列 + 资产承继清单；
  初版插入缺换行与行 4 拼接，已修复为干净单行新增）。AC-08 回归门：
  `cargo t -p auto-lang --no-fail-fast` 全量失败集 41 项与改前基线
  逐一全等（BASELINE-EQUAL）。
→ AC-06/07。

## 9. 复审记录

- 2026-09-18 /auto-plan:review 复审（F-01 修复闭环后终审）：
  `stage: review`，PLAN-027 rev 2。`outcome: pass` → **status =
  reviewed**。`reviewed_commit`：lang plan-027-dev F-01 修复提交
  （v1.10，63d981d49 之后）+ 全实现链 97d0bb75d/274265345/611fbff2f/
  T-05/d6e8da838；`base_commit` 2808c551a；`dependency_revisions`
  auto-down detached fae21d9（只读）。`spec_inputs`：SD-01（设计文档
  裁定落定 ✅ 63d981d49）、SD-02（协议 v1.10 ✅ 本轮修复——typed 通道
  载体/apply 语义/懒挂载 payload/清单/总线接缝/双载体声明/对拍记录
  入册，与实现逐面一致）、SD-03（台账 3c 行 ✅ 1313d00）、SD-04
  （shell-a2r-seams provisional ✅ 63d981d49）；front matter
  new_spec_components 已定稿。`acceptance_results`：AC-01 pass（五套
  实跑 + 全量基线全等；028 五败项与 011 flake 均归因预存——master
  对照在案，F-02 随 merge 注记）、AC-02 pass（18 测复现）、AC-03
  pass（F-01 修复后 delta 与实现闭环）、AC-04' pass（SD-01..04 落笔
  互链）、AC-05/06/07 处置留痕（§7 退役注记）、AC-08 pass（41 项
  BASELINE-EQUAL）。`findings`：F-01 fixed-in-review；F-02 minor
  不阻（launcher 套件锚数漂移属桌面维护线）。`next`：merge
  （auto-plan-merge——land plan-027-dev，沉淀 ledger，归档）。

- 2026-09-18 /auto-plan:review：`stage: review`，PLAN-027 rev 2。
  `outcome: needs_fix`（F-01）——**status 回 executing**。复审基线：
  reviewed_commit = lang plan-027-dev 63d981d49（工作树零 dirty），
  base = 2808c551a，依赖 = auto-down detached fae21d9（只读）。
  **已验面**：AC-02/03 复现 = 18 测全绿（词汇门/拒绝门/golden/投影门控
  族/roundtrip/载体单测，审定提交上复跑）；AC-08 全量失败集 41 项与
  基线全等（T-09' 期）；AC-01 desktop_mcp 五套解释形态实跑 =
  025-sys-monitor 13/13 ✅、038-minesweeper 25/25 ✅、013-todo 22/22 ✅、
  011-calculator 2×17/17 ✅（首跑掉线与 master 同签名双侧复现=预存
  环境 flake）、028-launcher 16 pass/5 fail——败项 master 基线二进制
  逐项全同（注册表内容/聚焦流随后续计划漂移，**预存非 027 回归**，
  在册 KNOWN-DEBT 面）；AC-04' 文档面已验（SD-01/03/04 互链可解析）。
  **findings**：
  - **F-01（block，AC-03/SD-02）**：规范增量 SD-02（add——
    `schema/projection-protocol-v1.md` typed 快照通道入册）canonical
    文本未落笔（rev2 T-09' 任务重写时遗漏，增量表仍保留该行）——
    协议权威文档零 typed 通道内容（grep 实证）。修正：work 补写
    typed 通道节（ShellProjection 载体 + apply 推送语义：指纹门控/
    原子性/召唤事件/clock 独立脏帧双载体并存）→ 复审转 pass。
  - **F-02（minor，不阻 pass，随 merge 注记）**：028-launcher 套件
    5 败项为套件期望漂移（基线二进制同败）——登记 KNOWN-DEBT 面
    （套件锚数更新属桌面程序维护线，非本计划合同）。
  `next`：work（F-01 修复，单周期间）→ 复审转 pass。

- 2026-09-18 /auto-plan:work 收口交接：`stage: work`，PLAN-027 rev 2。
  `outcome: pass` → **status = execution_done**。`code_commit`：
  lang plan-027-dev 63d981d49（T-09' 文档收口；此前实现链
  97d0bb75d/274265345/611fbff2f/T-05/d6e8da838）+ auto-os main
  1313d00（SD-03 台账）。`task_ids`：T-01..T-06 + T-09'（全数）；
  T-07/T-08 退役（§10-①）。`evidence`：各任务勾选证据；AC-01..03/08
  已验（✅ 注记），AC-04' 本任务落笔（SD-01..04 互链），AC-05/06/07
  处置留痕；AC-08 回归门 BASELINE-EQUAL（41 项）。`blockers`：无。
  `next`：/auto-plan:review（复审批次；desktop_mcp 五套在复审回归门
  复跑）→ merge；B 形态程序另立新计划（图像通道立项先行）。

- 2026-09-18 /auto-plan:new rev2 起草交接：`stage: new`，PLAN-027
  rev 2（有界修订，同一计划文件）。`outcome: pass`——修订授权已录
  （§4 rev2 授权记录；§10-① 裁定）；T-01..T-06 证据承继有效（形态
  无关），T-07/T-08 退役，T-09' 重写，AC 重述（AC-04' 新增、AC-05/06/
  07 处置随裁定留痕）、SD-01..04 重述。`next: work`（T-09' 收口 →
  execution_done → review → merge）；B 形态程序另立新计划（图像通道
  立项先行）。

- 2026-09-18 /auto-plan:work 交接：`stage: work`，PLAN-027 rev 1。
  `outcome: needs_replan`（§10-① 用户裁定主形态 = B——T-07/T-08 为
  A 形态面随裁定失效，S1/S2 形态无关全部承继）。
  `code_commit: d6e8da838`（plan-027-dev，base 2808c551a；依赖 worktree
  .wt/lang-027/auto-down @ fae21d9 只读）；`task_ids: T-01..T-06`。
  `evidence`：本文件各任务勾选证据 + §10-① 裁定记录；auto-lang 全量
  失败集与 base 41 项预存红逐一全等（零回归）；对拍捕获
  set_theme_name wire 缺陷已修（d6e8da838）。
  `blockers`：无（裁定已获，修订材料齐备）。
  `next`：/auto-plan:new 有界修订——S3 重排为 B 形态程序
  （图像通道立项先行 → 025 键盘实测 → 覆盖二批 → shell outproc client +
  启动序/看门兵；双轨常驻沿裁定）。worktree 保留供复审/merge。
  status 保持 `executing`（needs_replan 不改状态机位）。

- 2026-09-18 /auto-plan:work 中程记录：`stage: work`，PLAN-027 rev 1。
  T-01..T-06 完成（current_step 6/9）：S1 生成域补面三件 + 拒绝门 +
  词汇门 + mouse-area/div/taskbar 扩面（普查修正 C）落 ui_gen @
  97d0bb75d/274265345/611fbff2f；ShellProjection 模块 + sync 单源化 @
  T-05 commit；DesktopBusHandle/HostStorage + set_theme_name wire 缺陷
  修正（对拍捕获）@ d6e8da838。全部在 worktree
  `D:/autostack/.wt/lang-027/auto-lang`（plan-027-dev，base 2808c551a）；
  依赖 worktree `.wt/lang-027/auto-down`（detached @ fae21d9，只读）。
  回归归因：auto-lang 全量 no-fail-fast 失败集与 base 41 项本机预存红
  逐一全等。**下一步 = §10-① 用户确认（T-07 硬门）→ T-07/T-08/T-09。**
  阻塞：无（等待用户裁定问询）。

- 2026-09-18 /auto-plan:new 起草交接：`stage: new`，PLAN-027 rev 1。
  `outcome: pass`（合同完整：设计文档普查事实全量 file:line 在案，
  S1–S3 任务覆盖全部 AC 与规范增量）；`next: work`——**前置 = PLAN-026
  merge**；**T-07 前需用户确认 §10-① 两项工作假设**（形态 A + 双轨
  常驻；S1/S2 任务无假设依赖可先行）。悬置决策 §10（①–③），①为
  用户裁定项。
- 2026-09-18 补注：起草时用户裁定问询（AskUserQuestion 两问：A/B 形态、
  解释路径去留）未获作答——按设计文档 §5/§8-R3 推荐作为**工作假设**
  继续（形态 A + 双轨常驻），非用户显式授权；T-07 开工前的确认问询
  为硬门（见 §8 T-07 前置注记）。

## 10. 待澄清事项

- **①（用户裁定，T-07 前硬门）✅ 已裁定（2026-09-18 work 中）**：
  **主形态 = B（outproc 特权协议客户端）**；解释装载路径 = **双轨常驻**
  （推荐项获确认）。裁定原文要旨："A 岂不是要把所有 app 的 Rust 代码
  组合进桌面二进制？那显然不是操作系统的做法。我们不是有基于
  RenderQueue 的渲染机制吗？用普通操作系统的机制：桌面是独立进程
  （且包含 compositor），打开的 app 也是独立进程，渲染经 RenderQueue
  发给桌面进程统一渲染——这不是之前就规划好的吗？"
  **档案事实注记**：A 形态只链 shell pack 五件（~1800 行 .at 的生成物），
  不涉及任何 App——App 两形态下均已是独立进程走 RenderQueue
  （PLAN-020 客户端 + 025 覆盖爬坡既有）；裁定所引终态图景对 shell 的
  推论 = shell 也应同律独立进程，与设计文档"桌面=RenderHost、其余皆
  上层 App"一致，成立。**B 前置件序列**（设计 §4/§5，随裁定生效）：
  图像 DrawOp 通道（硬阻断——壁纸/缩略图皆图像，DrawList 现仅
  Quad/Text/TextStyled/Scissor；独立立项）→ 025 键盘路由真机实测 →
  覆盖二批 display 族 + popover 开合 → shell outproc client（复用 020
  client gate/ClientOpts 面）+ 启动序/看门兵。
  **S3 重排走范式修订**：T-07"无窗组件库+宿主装配"与 T-08"ShellSurface
  inproc 装配"为 A 形态面，随裁定失效；next = /auto-plan:new 有界修订
  （S1/S2 产物形态无关全部承继——IR/词汇门/typed 载体即 B 的 wire
  payload 词汇）。
- **②（T-01 D2）** 消费入口形态：WmSync 消息变体 vs 装配层方法调用
  （以 shell.at 消息流与懒挂载时序定案）。
- **③（低风险默认）** Stage B 搬迁与编译化先后：默认编译化先行
  （§1 非目标注记）；如需先搬迁，本计划 affects 面调整。
