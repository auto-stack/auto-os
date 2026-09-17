---
plan_id: PLAN-025
status: reviewed              # drafting → executing → execution_done → reviewed → archived
feature_name: native-queue-coverage-ramp
author: [agent]
created_at: 2026-09-17
updated_at: 2026-09-17
plan_revision: 1

# /auto-plan:review 结束时填写：
supersedes_spec_components: []
new_spec_components:
  - auto-lang/docs/design/autoui/desktop-protocol-v1.md   # §1.7 v1.7 增量（review 定稿）
touched_goals: []

affects:
  - auto-lang/crates/auto-lang/src/ui/desktop_protocol/native_projector.rs  # form/payload 渲染臂 + 分型命中表 + 键入/右键/滚轮
  - auto-lang/crates/auto-lang/src/ui/desktop_protocol/coverage.rs          # native_queue_set 扩容
  - auto-lang/crates/auto-lang/src/ui/desktop_protocol/client_runtime.rs    # 借鉴面（解释态先例）；HitKind 邻接对齐
  - auto-lang/crates/auto-lang/src/ui/session.rs                            # broker_key/char/scroll 宿主生产路径
  - auto-lang/crates/auto-lang/src/ui/iced/renderer.rs                      # INPUT_TEXT thread-local（读面；写点定案见 T-01 D2）
  - auto-lang/crates/auto-lang/src/ui_gen/rust.rs                           # slider/select codegen 臂
  - auto-lang/crates/auto-man/src/rust_ui.rs                                # 如 fixture 生成链需壳侧配合
  - auto-os/docs/plans/autos-desktop-program.md                             # 程序台账 3b 行
current_step: 8
total_steps: 8
---

# [PLAN-025] native-queue-coverage-ramp

## 0. 变更摘要

PLAN-020 交付的 `NativeProjector<C>`（a2r 编译 exe 经 RenderQueue 接入
compositor）v1 覆盖集仅 text/button + 线性布局；payload handler 族
（input/slider/select 等）与键盘/滚轮/右键输入路由显式 not-yet（KNOWN-DEBT
P020-D2；native `auto` 缺省降级 independent 兜底）。本计划做覆盖爬坡与输入
路由收口，四臂改造**全部零 wire 变体**（`InputMsg` 已含
KeyPressed/CharTyped/Scroll/Right 全套，message.rs:636-649）：

**①投影器 form 族臂**（input/textarea/checkbox/switch/radio——渲染/命中/
聚焦/CharTyped 编辑/INPUT_TEXT 回写通道）；**②投影器 payload 族臂**
（slider `fn(f32)->M` + select `SelectCallback`，含 a2r codegen 补臂与验收
fixture）；**③输入路由**（右键 `on_right_click`、滚轮 `Scrollable
on_scroll` + Scissor 裁剪镜像 515 G1、宿主侧 broker_key/char/scroll 生产
路径——解释态臂同册受益）；**④覆盖表与 parity 同步**（native_queue_set
扩容 + 防漏钉 + 003-converter 三臂金样）+ e2e 真机链 + v1.7 增量与台账
收口（P020-D2 键/滚轮/右键半句核销）。`PROTOCOL_VERSION` 维持 1。

## 1. 目标

- **G1 native form 族 queue 臂全链**：input/textarea/checkbox/switch/radio
  渲染（INPUT_BG/边框/焦点描边/placeholder/值文本/勾选图形——Quad+Text
  镜像解释态视觉规格）、命中、聚焦、CharTyped/退格编辑、INPUT_TEXT 回写
  （a2r 生成 `on()` 兼容）、view() 重入帧闭环。
- **G2 native payload 族**：slider（track/fill/knob 渲染 + 轨道命中几何 →
  f32 → `fn(f32)->M` 派发）；select（闭态盒 + 开态覆盖序 ops + 选项命中 →
  `SelectCallback(usize,&str)` 派发 + 投影器侧开合状态）；a2r codegen 补
  slider/select 臂 + 验收 fixture。
- **G3 输入路由**：右键（Button/Row/Column/Container `on_right_click` 命中
  派发）；滚轮（`Scrollable on_scroll` 回调派发 + Scissor/ScissorPop 溢出
  裁剪）；宿主生产路径（`broker_key_event`/`broker_char`/`broker_scroll` +
  WM 焦点窗路由——ui_desktop 真机键盘/滚轮可达 child；解释态臂既有
  CharTyped/退格消费 :445-457 同册受益）。
- **G4 覆盖表与 parity**：`native_queue_set` 扩容（kinds + layouts
  `scroll`）；auto 缺省仍 independent（翻转三闸之 T-覆盖复测数据随报告
  落盘，**裁定不动**）；防漏钉测试（native 覆盖矩阵 × 投影器臂双向钉）；
  三臂金样扩 input 级（003-converter：独立窗 / queue 重放 / pixels 重放）。
- **G5 e2e 与收口**：003-converter a2r exe 真机全链（键入 → 换算联动帧
  变）+ slider/select fixture 同链；smoke 扩展；desktop-protocol-v1.md
  §1.7 v1.7 增量；程序台账 3b 行；KNOWN-DEBT P020-D2 核销半句。

**非目标**（明确出界）：

- L3 `StateSnapshot` 注入 native（typed 组件字段写回通道另立——P020-D2
  后半笔维持债）。
- 双投影器统一（P020-D1：解释态投影器改写 View 基——覆盖爬坡后另立；
  本计划 native 臂继续平行 walker 追加，重复量随注入 D1 账）。
- 解释态 slider/select 投影臂（解释态 target_set 不加 slider/select，
  分表纪律 I4；解释态 queue 臂仅被动受益 G3 宿主生产路由，其右键/滚轮
  消费 not-yet 维持并留痕）。
- IME 闭环（ImePreedit/Commit 维持 not-yet——CharTyped 口径对齐 v1.3
  解释态裁定）。
- grid 布局 native 臂；scrollable 投影器内建偏移缓存（偏移语义 = app
  状态经 on_scroll 消息重入 view()，投影器只裁剪不缓存）。
- 键盘通用 KeyPressed→`key_message` 路由（桌面级热键线，另行）。
- GUI 级 OS 点击/键入自动化（P020-D4 维持债；e2e 输入证据协议级承载）。
- GPU 纹理共享（designed-only 维持）。

## 2. 架构方案

四臂改造，零 wire 变体，全部在既有 Desktop Protocol v1 骨架内：

```text
┌─ 投影器臂（desktop_protocol/native_projector.rs）────────────────┐
│ 命中表扩容：Vec<(WRect, C::Msg)> → 分型命中表（零参消息 / Input   │
│   {rect, on_change msg, identity} / Slider{rect, range, fn} /     │
│   Select 开合项 / 右键消息项 / Scroll 回调项）——T-01 D1/D3 定形态 │
│ 渲染臂追加：layout_view_input/textarea/checkbox/switch/radio/     │
│   slider/select（镜像解释态视觉规格；宿主 op 级泛型零改动）       │
│ 聚焦态：focused input 身份跨帧稳定（D1）；CharTyped/退格 →        │
│   投影器侧 buffer → INPUT_TEXT thread-local → component.on(       │
│   on_change msg)（a2r 生成 on() 兼容，零生成器改动——D2）          │
│ Scissor：scrollable 溢出 push/pop（镜像 client_runtime 515 G1）   │
└──────────────────────────────────────────────────────────────────┘
┌─ 生成器臂（auto-lang/src/ui_gen/rust.rs）─────────────────────────┐
│ slider/select codegen 臂（.at 元素 → View 构造 + on() 侧载荷      │
│   处理——checkbox 直接构造先例 :3043-3107）＋仓内 fixture           │
└──────────────────────────────────────────────────────────────────┘
┌─ 宿主路由臂（ui/session.rs + host.rs）────────────────────────────┐
│ broker_key_event/broker_char/broker_scroll（镜像 broker_pointer_  │
│   down :3174-3205 收尾）：WM 焦点窗 → InputMsg 下发命中窗 client  │
│ ui_desktop iced 键盘/滚轮订阅 → InputMsg 映射（D4 调查映射面）    │
└──────────────────────────────────────────────────────────────────┘
┌─ 覆盖/文档臂（coverage.rs + desktop-protocol-v1.md + os 台账）────┐
│ native_queue_set 扩容 + 防漏钉；v1.7 增量；程序台账 3b 行         │
└──────────────────────────────────────────────────────────────────┘
```

**不变式**：

- **I1 零删除**：PLAN-020 交付面零回归（text/button/线性布局臂、queue/
  pixels 双态、独立窗直跑、解释态三形态、宿主孵化分流）；既有测试全绿
  是回归门。
- **I2 追加式协议**：`PROTOCOL_VERSION` 维持 1，零新 wire tag——全部
  复用 InputMsg/ControlMsg 既有变体。
- **I3 not-yet 纪律**：本期未覆盖面（IME/grid/解释态 slider 等）显式
  留痕不静默；显式 `queue` 遇 not-yet 拒绝退出留痕行为延续（500/507/
  020 既定）。
- **I4 双轨分表**：`native_queue_set` 与解释态 `target_set` 分表扩容互
  不牵连（native 只加本期交付 kind；解释态 slider/select 维持缺项）。

**关键风险**：native View 无稳定节点身份（解释态靠绑定 field 身份
refocus）——聚焦 input 跨帧身份是唯一结构性未知量（T-01 D1 定案）；
宿主 iced 键盘事件 → CharTyped 映射（text 字段可用性/VK 码口径）需
调查（D4）；select 开合的命中互斥与覆盖序帧语义为本期最重交互件
（D3）；INPUT_TEXT thread-local 代写的线程边界（ClientPump 与
component.on 同线程——020 定案记录已核，执行期复核）。

## 3. 技术栈

Rust / iced 0.14（宿主）；既有 desktop_protocol（tokio named pipe、shm
双槽、APDL 信封）；`INPUT_TEXT` thread-local（iced/renderer.rs:28，读面
`last_input_text()` :1065-1069）；ui_gen codegen（auto-lang
src/ui_gen/rust.rs，auto-man rust_ui.rs 装配壳）；验收载体 =
`examples/ui/003-converter`（input×2 + 内联换算，app.at:37-49）+ 新
slider/select fixture（仓内 test fixture .at + a2r scratch 载体，020
scratch 先例——形态 T-04 定）；e2e = auto-os `scripts/desktop.sh` iced
宿主 + smoke 扩展（scripts/smoke-020-native-exe.sh 增 025 腿或新脚本）。

## 4. 需求分析与背景调查

**授权记录**：用户 2026-09-17 会话明确"按照你说的，用 auto-plan-new
起草下一阶段的任务"（下一阶段 = 前轮结论：queue 臂覆盖爬坡 payload 族
入覆盖集 + P020-D2 输入路由补齐）——**本轮仅规划，未授权实施**。涉及
仓：auto-lang（投影器/生成器/宿主路由/协议文档）+ auto-os（台账/e2e/
smoke）。无预算/自动续跑约束声明。

**现状事实**（已核，2026-09-17 探索代理全量扫描 + 主检复读）：

- **wire 全量在册，零缺口**：`InputMsg` 含
  PointerPressed{button: Left/Right/Middle}、KeyPressed/KeyReleased{key:
  u32, modifiers}、CharTyped{ch}、Scroll{dx,dy}、Ime*（message.rs:636-649，
  encode 覆盖 :667+）；`MouseButton::Right = 2`（:618-622）。缺口全在
  两端：投影器消费（native_projector.rs:129-145 仅 Left-Pressed）与宿主
  生产（仅 `broker_pointer_down` session.rs:3174-3205 /
  `ProtocolHost::pointer_down` host.rs:255-275；键盘仅测试注入先例
  stage3.rs:620-633）。
- **NativeProjector 现状**：命中表 `Vec<(WRect, C::Msg)>`（:46）；其余
  View 变体走占位盒臂 `not-rendered` + uncovered 留痕（:397-412）；布局
  无 Scissor；覆盖门 `ensure_covered`（:72-82）调用点 client_entry.rs:
  154-157。
- **解释态先例可镜像**：layout_input（client_runtime.rs:1047-1126：
  INPUT_BG Quad + 1px 边框 + 焦点蓝描边 :1070-1081 + placeholder/
  值文本）；聚焦 `focused_input` + 点击聚焦（:104/:414-416）+
  CharTyped/退格（:445-457）+ `type_into` 保型回写（:483-521）+ refocus
  （:184-190）；checkbox/switch/radio（:1134-1259，`register_toggle` 命
  中 + handler 在场 = handler 拥有状态变更）；layout_scroll Scissor
  push/pop（:953-1020，滚动偏移交互 v1 不载 :950-952 随注——native 侧
  语义 = on_scroll 全交 app，口径差入册 v1.7）。
- **INPUT_TEXT 通道**：thread-local 由 iced 窗口路径独写（renderer.rs:
  4047/4143/4148），queue 臂（无 iced widget）恒空读；a2r 生成 `on()`
  依赖 `last_input_text()` 写绑定字段（ui_gen/rust.rs:1270-1314）——
  投影器同线程代写可行（ClientPump 与 component.on 同线程，020 §5.1
  先例：MCP 派发前设同一 thread-local renderer.rs:21926）。
- **View handler 形态**（view.rs）：Input builder `.on_change(M)`（占位
  payload + 真文本经 thread-local，:1618+ 生成侧 :2668-2729）；textarea
  同构（:1651/:2732-2777）；Checkbox `on_toggle: Option<M>` 零参物化
  （ui_gen :3043-3107 直接构造）；`slider(range, value, fn(f32)->M)`
  （:1726，**fn 指针非闭包**）；`select(options)` +
  `SelectCallback{Arc<dyn Fn(usize,&str)->M>}`（:15-39/:1687）；
  `Scrollable{on_scroll: Option<ScrollCallback{Arc<dyn Fn(
  ScrollMetrics)->M}>}`（:283-301）；`on_right_click: Option<M>` 在
  Button/Row/Column/Container（:454/:479/:490/:651）。
- **codegen 缺口**：input/textarea/checkbox 臂已有；slider/select 无
  codegen 臂（仅 KNOWN_TAGS :4510-4619 登记）——**examples/ui 001-045
  无 slider/select 真源**（020-music-player 的 "slider" 仅 icon 名
  controls.at:155）。
- **宿主合成零改动成立**：`DrawListPainter` op 级泛型（broker_surface.rs:
  39-158）——投影器扩臂产 Quad/Text/Scissor 即可显示。
- **测试先例**：`native_client_full_cycle_over_pipe`（native_projector.rs:
  886-996，真管道全循环）；`coverage_gate_refuses_payload_family`（:747-772，
  Slider 构造缺项断言——扩容后此测试语义反转需改写）；`p020_native_
  exe_arm`（stage3.rs:1967-2191，AUTO_DESKTOP_E2E 门 + AUTO_020_*
  载体 env）；`parity_matrix_covers_target_set` 防漏钉（client_runtime.rs:
  2698-2723）；editor_frame.rs:170-205 = InputMsg 全变体→FrameSource 消费
  模板。
- **specs 现状**：协议权威 = desktop-protocol-v1.md（v1.6 现行，§1.6 =
  020 交付）；模块 spec provisional 指针已立（020 SD-03）。本计划规范
  增量见 §5 规范增量表。

## 5. 详细设计

### 5.1 T-01 深水调查定案（决策产物）

- **D1 聚焦身份机制**（本计划核心结构决策）：native View 无解释态的
  field 绑定名。候选：A = 命中表槽位序 + 帧后重定位（解释态 refocus
  :184-190 同型——首帧记 idx，重渲染后按几何最近邻/首 Input 槽对位）；
  B = View 树路径身份（vnode_converter `VNode.path` 稳定身份先例
  vnode.rs:310，投影器自记 path）；C = 值指纹匹配。以 003-converter 双
  input（同屏两框、值互算触发重渲染）+ 动态增删 input 样本定案。
- **D2 INPUT_TEXT 回写定案**：A = 投影器同线程代写 thread-local（零
  生成器改动，倾向——与 iced 窗口路径写入点语义等价：写入 = 焦点框
  当前文本）；B = 显式载荷通道（生成器配合改 on() 签名）。定案记录须
  含 a2r 生成 on() 读面兼容性证据（ui_gen/rust.rs:1270-1314）与
  textarea/parse 保型（Int/Double 字段）边界。
- **D3 select 开合语义**：开态 = 主块渲染后追加覆盖序 ops（DrawList
  paint order 天然置顶，无 overlay 协议语义）+ 投影器侧 `open: bool`
  （外点/Esc 关闭；开态命中表互斥：仅选项项 + 关闭区）；选项命中 →
  `SelectCallback(idx, label)` 物化 M 派发。CharTyped 字母跳项等键盘
  导航 v1 不做（not-yet 随注）。
- **D4 宿主键盘/滚轮映射**：iced 0.14 键盘事件（KeyPressed
  `text`/SmolStr 可用性、物理键 → Windows VK u32 口径）→
  InputMsg::KeyPressed/CharTyped；iced 滚轮 subscription →
  Scroll{dx,dy}。WM 焦点窗语义核实（现桌面 = 最近点击窗？）。
- **D5 滚轮语义**：Scrollable `on_scroll` 在场 = `ScrollCallback(
  ScrollMetrics)` 全交 app 状态（投影器不缓存偏移，Scissor 只管裁剪）；
  不在场 = 不路由留痕。与解释态"滚动偏移 v1 不载"的口径差入册 v1.7。
- **slider 拖拽**：v1 最小 = 点击定位（PointerPressed 一次派发）；
  按住拖拽连续派发（PointerMoved 按住态）若 D1 命中表形态天然支持则
  顺带，否则 not-yet 随注（T-01 附带定案）。

定案记录追加 `### 5.1 定案记录`，作为 T-02..T-05 依据。

### 5.1 定案记录（T-01，2026-09-17，worktree lang-025 基线 5ceeac30c）

- **D1 聚焦身份 = 候选 A（命中表槽位序 + 帧后重定位）**。证据：
  native `View::Input` 无任何身份/key 字段（view.rs:494-503——placeholder/
  value/on_change/on_submit/width/password/style），候选 B 的 path 身份
  需投影器自记 VNode.path 但 native 无 VNode（vnode.rs:310 先例属解释
  态轨），候选 C 值指纹在双 input 同值时（003 两框初始 "0"/"32" 尚异，
  但编辑后可同值）歧义。定案：投影器侧 `focused_input: Option<usize>`
  = **Input 槽位序**（确定性树序——view() 全物化、a2r 生成按源序构造，
  命中表登记序即源序）；点击聚焦记槽位；每帧 render 后重定位（槽位
  越界 → 置 None 留痕）。动态增删 input 致焦点前移的结构变化为 v1 边界
  （越界即失焦，不猜测——解释态 field 名 refocus client_runtime.rs:184-190
  的原语 native 不具备，债务随注 §1.7）。
- **D2 INPUT_TEXT 回写 = 候选 A（投影器同线程代写 thread-local）**。
  证据链：a2r 生成 on() 对 input 登记变体读
  `auto_lang::ui::iced::last_input_text()`（ui_gen/rust.rs:1280）并按
  绑定字段类型 parse（:1290-1313——f64/i32/…；003-converter 双 double
  字段保型由 parse+unwrap_or 承担，投影器侧不重复保型）；ClientPump
  单线程泵（client_runtime.rs:2135 step→dispatch→FrameSource::on_input
  同调用线程），thread-local 写（投影器）与读（生成 on()）同线程成立；
  020 先例 renderer.rs:22688 同线程代写先例在场。定案：投影器持聚焦框
  编辑 buffer（聚焦时自 view value 初始化；非聚焦框恒直显 view value）；
  CharTyped/退格 → buffer 编辑 → `INPUT_TEXT.with(set)` →
  `component.on(on_change)` → rev++。**on_submit/Enter 不派发**（not-yet
  随注——解释态臂同边界，client_runtime.rs:445-457 只消费 CharTyped/
  VK_BACK）。物化零参纪律下 props/events 不需登记（on_change 已是 M 值，
  覆盖表 props/events 对 native 维持空表——judge 只查 kinds/layouts/
  style）。
- **D3 select 开合 = 覆盖序 ops + 投影器侧 `select_open: Option<usize>`
  槽位**。闭态 = 值盒 + ▾ + 点击开（无消息派发）；开态 = 主块渲染后
  **追加**选项列 ops（DrawList paint order 天然置顶）+ 命中互斥：开态
  PointerPressed 只查选项项——命中 → `SelectCallback.call(idx,label)`
  物化派发 + 关闭 + rev++；未命中（外点）→ 仅关闭（吞掉，不下穿主块）
  + rev++（关闭也是状态变化——帧回闭态）。Esc（KeyPressed 27）关闭。
  选项消息在布局期物化（回调 Arc clone 廉价），命中表存 `(rect, M)` 零
  新类型。键盘跳项 not-yet（随注）。高亮当前项 = BUTTON_BG Quad。
- **D4 宿主键盘/滚轮映射**。iced 0.14 面：`keyboard::Event::KeyPressed{
  key, text, .. }` 携 `text: Option<SmolStr>`（可打印字符 → CharTyped
  直接取）；特殊键（Backspace/Escape/方向）以 `Key::Named` → Windows
  VK u32 映射（BACK=8/RETURN=13/ESC=27——解释态消费口径
  client_runtime.rs:450）；滚轮 `mouse::Event::WheelScrolled{delta}` →
  `Lines{x,y}×LINE_H 像素化 / Pixels 直取` → Scroll{dx,dy}。WM 焦点 =
  `session.focused: Option<Wid>`（session.rs:629；wm_focus :2408）——
  broker_key/char/scroll 路由目标 = 焦点窗（区别于 pointer_down 的
  hit_test）。**生产接线面**：ui_desktop iced 壳层（auto-lang
  session.rs iced::daemon 侧 :6347 desktop_window_events 同源订阅面）
  把键盘/滚轮事件映射后调 session broker_*（真机可达性 e2e 期核，
  ⑤口径：不可达时协议级 CharTyped 注入承载）。
- **D5 滚轮语义 = on_scroll 全交 app**。Scrollable 命中表项 =
  `{viewport rect, offset 快照(view.offset), viewport/content 尺寸,
  ScrollCallback}`；派发时 `offset' = clamp(快照 + (dx,dy), 0..=content-
  viewport)`——镜像 iced 侧 `vp.absolute_offset()` 为**滚动后**偏移的
  语义（renderer.rs:2353 证据），回调收新偏移，app 写状态重入 view()
  （投影器不缓存偏移，Scissor 只裁剪）。on_scroll 不在场 = 滚轮不路由
  留痕（I3）。
- **slider 拖拽附带定案 = v1 点击定位**（PointerPressed 一次派发）；
  按住拖拽连续派发 not-yet 随注（分型命中表无按住态追踪，强加需
  PointerMoved 按住态机——债面入 §1.7）。
- **调查新证据（执行期修订，不扩合同）**：
  1. **View 无 Switch 变体**（view.rs 全枚举核对——native_kind_of
     coverage.rs:374-413 亦无 switch 臂）：§5.2/§5.5 的 "switch" 臂
     **落空**，native form 族交付 = input/textarea/checkbox/radio
     四型（switch 为解释态 aura 标签专属——024 台账 t1_form 夹具经
     解释态投影，native 轨无可产该 kind 的 View 构造，not-yet 无对象
     即无缺口）。native_queue_set kinds 不含 switch（I4 分表）。
  2. **003-converter gate 阻断 token = flex-1 / shadow-sm**（app.at
     复核——field_col "flex-1 gap-1.5"、卡片 "shadow-sm"）：两 token
     解释态 target_set 均放行（coverage.rs:133/136），native 渲染面
     已按解释态同款边界静默降级（native_projector.rs:531-534——shadow
     no-op、flex 族自然宽）。定案：native_queue_set style_prefixes 补
     "flex-1"/"shadow"（**降级放行**——语义 = 解释态同款保真边界，
     入册 §1.7；非静默扩权，覆盖表随注先行）。
  3. ScrollMetrics offset 语义 = 滚动后绝对偏移（renderer.rs:2353），
     已并入 D5；slider `on_change: fn(f32)->M` 为 fn 指针（view.rs:724，
     Copy）——命中表零 Arc 存储直接内联。

### 5.2 投影器 form 族臂（T-02）

`layout_view_node` 追加 Input/Textarea/Checkbox/Switch/Radio 臂：视觉
镜像解释态（INPUT_BG Quad + 1px 边框 + 焦点蓝描边 + placeholder/值
Text + 勾选图形 Quad 族；textarea 多行框复用 input 命中/编辑闭环）；
命中表分型（Input{rect, on_change msg, identity} / Toggle 零参消息直入
既有消息表）；`on_input` 扩 CharTyped/KeyPressed{key:8}——聚焦编辑
buffer + INPUT_TEXT 写入（D2）+ `component.on(msg)` + rev 前进；渲染轮
转后聚焦重定位（D1）。覆盖门同步：native_queue_set kinds 扩容 + input
值面口径（物化零参纪律下 props/events 是否需要登记——按 D2 定案记录）。

### 5.3 投影器 slider/select 臂 + codegen（T-03/T-04）

- **slider（T-03）**：track 底 Quad + fill Quad + knob Quad（值比例
  几何）；命中 = 轨道 rect → `f32 = start + clamp((x-x0)/w) × range` →
  `on_change(value)` 物化 M 派发。codegen 臂：`View::slider(range,
  value, |v| Msg::Variant(v))` + on() 侧 f32 载荷写绑定字段（fn 指针
  闭包物化，回写通道比 input 干净——定案记录确认为零 thread-local）。
- **select（T-04）**：闭态 = 值盒 + ▾；开态 = 覆盖序选项列（高亮当前，
  D3）；codegen 臂：`View::select(options)` + on_select 回调 + on() 侧
  usize/&str 载荷写绑定字段。fixture：仓内 test fixture .at（codegen
  golden 单测源）+ a2r scratch 载体（020 scratch020 先例，e2e 用）。

### 5.4 输入路由与宿主生产路径（T-05）

- 投影器：PointerPressed-Right → 右键命中表项（Button/Row/Column/
  Container `on_right_click` 物化 M）派发；Scroll → 命中窗内 Scrollable
  `on_scroll` 派发（ScrollMetrics 组装，D5）+ Scissor 溢出裁剪帧。
- 宿主：session.rs 增 `broker_key_event`/`broker_char`/
  `broker_scroll`（镜像 broker_pointer_down :3174-3205 收尾——WM 命中/
  焦点窗 → 命中窗 client 管道下发）；ui_desktop iced 键盘/滚轮订阅 →
  InputMsg 映射（D4）。解释态臂同册受益：既有 CharTyped/退格消费
  （:445-457）接通生产路径；解释态右键/滚轮消费 not-yet 维持（生产
  路径就位、消费端留痕——I3）。

### 5.5 覆盖表/parity/裁决（T-06）

`native_queue_set` 扩容：kinds + input/textarea/checkbox/switch/radio/
slider/select；layouts + scroll；样式前缀按新臂需要审（焦点描边色等
投影器内置常量不入表）。auto 缺省维持 independent（508/020 裁定不动；
翻转三闸之 T-覆盖复测 = examples 001-045 可 queue 化比例数据行入
T-08 报告）。防漏钉：native 覆盖矩阵 × 投影器臂双向钉测试
（parity_matrix_covers_target_set 先例）；`coverage_gate_refuses_
payload_family` 语义反转改写（slider 由拒转收，新拒样本换 grid/IME）。
三臂金样扩 input 级：003-converter 独立窗 / queue 重放 / pixels 重放
同源对拍（I4' 纪律扩条目）。

### 5.6 e2e/文档/台账（T-07/T-08）

- e2e：`p025_native_input_arm`（AUTO_DESKTOP_E2E 门 + 025 载体 env，
  p020_native_exe_arm 同型）——003-converter a2r exe（scratch 载体重
  生成）孵化 → queue 帧双 input → 宿主生产路径键入（真机）或协议级
  CharTyped 注入（P020-D4 债未清前的承载口径，⑤）→ 换算联动帧断言；
  slider/select fixture 同链。截图留痕
  `docs/plans/reports/assets/025/`（lang 仓）。
- 文档：desktop-protocol-v1.md **§1.7 v1.7 增量**（native form/payload
  族入册 + 输入路由两端 + INPUT_TEXT 回写通道 + 边界：IME/grid/拖拽/
  解释态 slider/select/滚动偏移口径差）；KNOWN-DEBT P020-D2 键/滚轮/
  右键半句核销（StateSnapshot 半句保留另立）+ 新债随注（如 D1 身份
  机制的债面）；os 程序台账 3b 行（canonical 随 merge）。
- 度量增量（轻）：覆盖翻转样本数据行（不入 020 度量口径重跑）。

### 规范增量

| delta_id | add/modify/retire | docs/specs/... target | before/after rule | rationale | acceptance IDs |
|---|---|---|---|---|---|
| SD-01 | add | auto-lang/docs/design/autoui/desktop-protocol-v1.md（§1.7 v1.7 增量） | before：v1.6 native 覆盖 = text/button + 线性布局，键/滚轮/右键不路由，INPUT_TEXT 回写通道缺失；after：form/payload 族入册（渲染/命中/聚焦/回写），输入路由两端（投影器消费 + 宿主 broker_key/char/scroll 生产 + WM 焦点路由），边界显式（IME/grid/拖拽/解释态 slider/select/滚动口径差）——PROTOCOL_VERSION 仍 1（零 wire 变体） | 协议权威文档版本化收录覆盖爬坡 | AC-02/03/04/05 |
| SD-02 | modify | auto-os/docs/plans/autos-desktop-program.md | before：3a 行 native 覆盖 v1 最小集 + 输入路由 not-yet；after：增 3b 行（覆盖爬坡 + 输入路由收口；P020-D2 键/滚轮/右键半句核销、StateSnapshot 半句另立） | 桌面程序台账收录 | AC-06 |
| SD-03 | modify | auto-lang/docs/specs/auto-lang/ui/（review 期按目录实况定） | before：020 provisional 指针；after：coverage/native_projector 模块 spec 条目对齐 v1.7 覆盖集与输入路由 | 模块 spec 对齐实现 | AC-02/03/04/05 |

零 spec 影响的变更不存在（覆盖集/输入路由为协议级知识）；ledger
（auto-lang `.autoos/specs.json`）随 merge 沉淀。

## 6. 测试设计

- **单测（auto-lang native_projector/coverage）**：form 臂渲染 golden
  （input 值/placeholder/焦点描边、checkbox 勾选、switch/radio）；聚焦
  编辑闭环（CharTyped/退格 → buffer → INPUT_TEXT → on → 帧值更新；003
  换算保型 Int 样本）；slider 几何 golden + f32 派发（轨道点击 x → 期望
  值）；select 闭/开态 golden + 选项命中 + 外点关闭；右键/滚轮派发 +
  Scissor 帧断言（溢出子裁剪、栈平衡）；覆盖门扩容判定（input/slider/
  select 入、grid/IME 仍拒 + `coverage_gate_refuses_payload_family`
  反转改写）。
- **单测（auto-lang ui_gen/auto-man）**：slider/select codegen golden
  （fixture .at → View 构造 + on() 载荷写绑定字段）；`cargo t -p
  auto-man rust_ui` 回归。
- **单测（session）**：broker_key/char/scroll → 焦点窗管道路由（镜像
  broker_pointer_down 测试形态）。
- **集成（真管道全循环）**：`native_client_full_cycle_over_pipe` 同型
  扩展——form App（双 input）CharTyped 联动；slider/select App 两型。
- **e2e（AUTO_DESKTOP_E2E）**：`p025_native_input_arm` + smoke 脚本
  （os 仓）+ 截图留痕 assets/025/。
- **parity**：003-converter 三臂金样对拍（独立窗 / queue / pixels）。
- **回归门**：desktop_protocol / session / stage3 / dual_mode /
  app_registry / client_runtime 套件全绿（AC-01/07）。

## 7. 验收标准

- **AC-01 既有零回归**：PLAN-020 交付面全绿——native queue counter 全
  循环、pixels 臂 e2e、独立窗直跑（AC-01 of 020）、解释态三形态、
  `p020_native_exe_arm`。验证：`cargo t -p auto-lang --features ui-iced
  desktop_protocol` + session/stage3/dual_mode 全绿（在册既有红
  coverage::covered_elements_within_target_set 除外——plan624 线）。
- **AC-02 form 族 queue 全链**：a2r 003-converter exe 显式 queue 档孵化，
  双 input 渲染进虚拟窗；键入（宿主生产路径，真机不可达时协议级
  CharTyped 注入承载 + 留痕）→ INPUT_TEXT → on_change → 换算联动帧变
  （输入值与换算结果断言）。验证：集成 + e2e + 帧断言证据。
- **AC-03 slider/select queue 全链**：fixture App slider 轨道点击 →
  f32 派发 → 值帧变；select 开 → 选项命中 → SelectCallback → 值帧变 →
  关闭。验证：集成两型 + golden + e2e 载体。
- **AC-04 输入路由两端**：投影器侧右键/滚轮派发（单测 + Scissor 帧）；
  宿主侧 broker_key/char/scroll 单测 + 一次真机（或协议级承载，⑤口
  径）链路证据；解释态臂键入经新生产路径可用（同册受益实证一条）。
- **AC-05 覆盖表与裁决**：003-converter + slider/select fixture 装载期
  native 判定 Covered（显式 queue 不再拒绝）；grid/IME 未覆盖显式
  not-yet 维持；native auto 缺省仍 independent + 降级留痕；防漏钉双向
  测试在册。验证：覆盖单测 + 防漏钉。
- **AC-06 台账与文档**：desktop-protocol-v1.md §1.7、os 程序台账 3b 行
  （worktree 落笔，canonical 随 merge）、KNOWN-DEBT P020-D2 半句核销
  落盘且互链。验证：文档存在 + 交叉引用可解析。
- **AC-07 回归门**：`cargo t -p auto-lang --features ui-iced
  desktop_protocol`、session/stage3/dual_mode/app_registry、`cargo t
  -p auto-man rust_ui`、auto-os 桌面 smoke（desktop_mcp 链）全绿。

## 8. 执行步骤

依赖序：T-01 → T-02 → {T-03, T-04 并行} → T-05 → T-06 → T-07 → T-08。
auto-lang 侧在 lang worktree（`D:/autostack/.wt/lang-025/auto-lang`，
020 同型）；auto-os 侧在 os 组 worktree（`D:/autostack/.wt/os-025/
auto-os`）。

- **T-01 [lang] 深水调查与 seam 定案** ✅
  文件：`src/ui/view.rs`、`src/ui/desktop_protocol/native_projector.rs`、
  `client_runtime.rs`、`session.rs`、`iced/renderer.rs`（读面）+ 本计划
  §5.1（写面）。
  动作：D1–D5 + slider 拖拽附带定案；003-converter 双 input 与动态
  增删样本扫描。
  产物：`### 5.1 定案记录`（file:line 证据）。
  验证：定案记录完备（五决策 + 附带项）；复审通过。✅ [✅ 已完成]
  [2026-09-17 work] 五决策+拖拽附带全落 §5.1；新证据三项（View 无
  Switch 变体→native form 族=四型；003 gate 阻断=flex-1/shadow-sm→
  降级放行定案；ScrollMetrics offset=滚动后偏移）已录。证据
  lang-025 @5ceeac30c 读面核对。
  → AC-02/03/04 前置。新路径：是（调查产物）。
- **T-02 [lang] 投影器 form 族臂 + 键入闭环** ✅
  文件：`native_projector.rs`（渲染臂/分型命中表/聚焦/编辑 buffer/
  INPUT_TEXT 写入）、`coverage.rs`（native kinds 扩容）。
  动作：按 §5.2 + D1/D2 定案。
  验证：§6 form 单测 + 集成（双 input CharTyped 联动）全绿；回归门
  （client_runtime/stage3 不动面全绿）。
  [2026-09-18 work] 分型命中表 HitEntry + Input/Textarea/Checkbox/Radio
  臂 + 聚焦槽位（D1-A）+ store_input_text 代写闭环（D2-A）；测试
  12/12 + native_form_full_cycle_over_pipe。lang plan-025-dev 734f34ad5。
  → AC-02/05。
- **T-03 [lang] slider 臂 + codegen + fixture** ✅
  文件：`native_projector.rs`（slider 渲染/命中/f32 派发）、
  `ui_gen/rust.rs`（slider codegen 臂）、`auto-man` fixture 链。
  动作：按 §5.3 slider。
  验证：slider golden/派发单测 + codegen golden + scratch 载体编译过。
  [2026-09-18 work] HitEntry::Slider + track/fill/knob 几何 + step 取整
  派发；codegen fn 指针臂 + fixture slider.at + golden（e2e 期编译反馈
  修 f32 字面量/math 降级/input value 容差——T-07 一并入册）。e9545479b。
  → AC-03/05。
- **T-04 [lang] select 臂 + codegen + fixture** ✅
  文件：`native_projector.rs`（闭/开态 + 命中互斥 + SelectCallback）、
  `ui_gen/rust.rs`（select codegen 臂）。
  动作：按 §5.3 select + D3 定案。
  验证：select golden/命中/关闭单测 + codegen golden + 集成。
  [2026-09-18 work] SelectOverlay 覆盖序 + 命中互斥 + Esc/外点关闭；
  on_choose 物化闭包 codegen（载荷数自适应）+ fixture select.at +
  golden；usize 非法 .at 载荷类型证据 → fixture 用 str。5a1270bac。
  → AC-03/05。
- **T-05 [lang] 右键/滚轮路由 + 宿主生产路径** ✅
  文件：`native_projector.rs`（Right/Scroll 消费 + Scissor）、
  `session.rs`（broker_key_event/broker_char/broker_scroll + WM 焦点
  路由）、`host.rs`（如需 ProtocolHost 侧同构）。
  动作：按 §5.4 + D4/D5 定案。
  验证：路由单测 + Scissor 帧断言 + 解释态同册受益实证。
  [2026-09-18 work] Right/Scroll 消费 + Scissor 溢出裁剪 + broker_key_
  event/broker_char（焦点窗）/broker_scroll（命中窗）落册；t3 e2e 003
  键入改走 broker_char（解释态同册受益实证）；D4 边界：live 壳无键盘
  订阅通道 → 新债 P025-D1 随注（⑤口径协议承载）。89b3616db。
  → AC-04。
- **T-06 [lang] 覆盖表收口 + parity** ✅
  文件：`coverage.rs`（native_queue_set/防漏钉/反转测试改写）、
  金样体系（003-converter 三臂）。
  动作：按 §5.5。
  验证：防漏钉 + parity 金样对拍绿。
  [2026-09-18 work] flex-1/shadow 降级放行（§5.1 定案 2）+ 003 全
  token 放行单测；双向防漏钉（矩阵 × 投影器臂）；native queue 金样
  （test/parity/native/003-converter.expected.txt 两阶段全精度锁）；
  gate 反转改写（T-03 已并）+ underline 样本换防。83e2720f3。
  → AC-05。
- **T-07 [lang+os] e2e 验收** ✅
  文件：lang `stage3.rs`（p025_native_input_arm）+ 截图留痕
  `docs/plans/reports/assets/025/`；os `scripts/`（smoke 扩展）。
  动作：AC-02..05 逐条跑通留痕。
  验证：见各 AC 验证句。
  [2026-09-18 work] p025_native_input_arm 三腿真 exe PASS：003 queue 档
  孵化 + broker_pointer_down 聚焦 + broker_char 键入 "100"→212 联动
  （AC-02）；slider 75% 点击→75 + select 开→Medium 命中→闭（AC-03）；
  帧留痕 assets/025/（帧 dump 代截图——⑤口径/020 同边界）；os
  smoke-025-native-input.sh（os plan-025-dev 4284bc7）。d871f8e50。
  → AC-02/03/04。
- **T-08 [lang+os] 文档与台账收口** ✅
  文件：lang `desktop-protocol-v1.md`（§1.7）、`KNOWN-DEBT-AND-RISKS.md`
  （P020-D2 半句核销 + 新债）、覆盖翻转数据行；os
  `autos-desktop-program.md`（3b 行）+ 两仓互链。
  动作：SD-01..03 落笔。
  [2026-09-18 work] §1.7 六节增（SD-01）；KNOWN-DEBT P020-D2 半句核销
  + P025-D1/D2 新债；os 台账 3b 行（SD-02，os plan-025-dev b4c2c1b）；
  SD-03 模块 spec 对齐按计划措辞留 review 期。870ee1574。
  → AC-06。

## 9. 复审记录

- 2026-09-17 /auto-plan:new 起草交接：`stage: new`，PLAN-025 rev 1。
  `outcome: pass`（合同完整：背景事实全量 file:line 在案，任务覆盖全部
  AC 与规范增量，路径/命令对两仓核验）；`next: work`（T-01 起步——
  深水定案先行，无需用户解锁）。悬置决策登记 §10（①–⑤），均不阻塞
  T-01 开工。
- 2026-09-18 /auto-plan:work 收执：`stage: work | PLAN-025 | rev 1 |
  outcome: pass | lang code_commit 5ceeac30c..870ee1574（plan-025-dev，
  T-02 734f34ad5 / T-03 e9545479b / T-04 5a1270bac / T-05 89b3616db /
  T-06 83e2720f3 / T-07 d871f8e50 / T-08 870ee1574；os plan-025-dev
  4284bc7 + b4c2c1b）| task_ids T-01..T-08 全勾（8/8）| evidence：
  AC-01 desktop_protocol+stage3+dual_mode+app_registry 172/173（唯一红
  = covered_elements_within_target_set 在册既有红 plan624 线，基线
  5ceeac30c 复核同红）+ client_runtime/native_projector 61/61 +
  auto-man rust_ui 22/22 + p020_native_exe_arm 绿；AC-02/03
  p025_native_input_arm 三腿真 exe PASS（003 queue 孵化 broker_char 键入
  100→212 联动 / slider 75% / select Medium，帧留痕 assets/025/）；
  AC-04 broker_input_production_routes + t3 e2e 走新生产路径（解释态
  同册受益）；AC-05 native_coverage_matrix_pinned_to_projector 双向钉 +
  native_gate_accepts_003_style_tokens + native queue 金样；AC-06 §1.7
  + 台账 3b 行 + KNOWN-DEBT 核销/新债互链可解析；AC-07 回归门全绿（除
  在册既有红）| blockers 无 | next: review（execution_done，worktree
  lang-025/os-025 保留）。合同内偏差（证据驱动，均留痕）：View 无
  Switch 变体 → native form 族四型；flex-1/shadow 降级放行；ScrollMetrics
  offset=滚动后语义；live 壳键盘订阅缺口 → P025-D1；截图→帧 dump 代留痕
  （⑤口径）。
- 2026-09-18 /auto-plan:review 复审：`stage: review | PLAN-025 | rev 1 |
  outcome: pass | reviewed_commit lang plan-025-dev 870ee1574 + os
  plan-025-dev b4c2c1b | base_commit lang 5ceeac30c / os 4b974c2 |
  dependency_revisions auto-down 3a052558（detached 兄弟，只读清单）|
  spec_inputs desktop-protocol-v1.md（§1.7 增量随 T-08 提交）+ KNOWN-DEBT
  （P020-D2 核销 + P025-D1/D2）+ os 台账 3b 行 | acceptance_results：
  AC-01 pass（172/173；唯一红 covered_elements_within_target_set = 在册
  既有红——测试本体与 target_set 依赖零改动 + 基线复核；p020 臂绿）；
  AC-02 pass（p025 converter 腿：真 exe queue 孵化→broker_char 100→212
  联动，复审独立复跑）；AC-03 pass（slider 75%/select Medium 腿复跑 +
  46 项单测/golden）；AC-04 pass（broker_input_production_routes wire 级
  断言 + t3 e2e 改走 broker_char 同册受益 + 右键/滚轮/Scissor 单测）；
  AC-05 pass（双向防漏钉 + 003 token 放行 + 金样内容抽查=全精度帧 dump
  非平凡断言）；AC-06 pass（§1.7 断言与代码抽查一致：view.rs Switch=0/
  store_input_text/broker×3；互链可解析）；AC-07 pass（client_runtime+
  native_projector 61/61 + auto-man rust_ui 22/22 + p020 无回归；os diff
  纯 docs+script 零运行时面 → desktop_mcp 不受影响）| findings：
  R-1 info rust-workspace 成员表/旧 converter 生成件随构建变更（生成器
  副作用，T-07 提交说明留痕；015-notes 副作用已还原）；R-2 info SD-03
  目标钉定 = docs/specs/auto-lang/ui/overview.md:30 provisional 指针行
  （v1.6→v1.7 + 覆盖集/输入路由随册一句），canonical 编辑不随 review
  发布、merge 期落笔；R-3 info AC-07 os 侧 desktop_mcp 未跑（零运行时
  变更，不构成缺失）；R-4 info 帧 dump 代截图（⑤口径/P020-D4 同边界，
  已授权）| evidence：复审在 lang-025/os-025 worktree HEAD 独立复跑
  p025_native_input_arm + broker_input_production_routes +
  t3_examples_queue_end_to_end + 173 项协议套件 + 46 项投影器/覆盖/
  codegen 套件（本记录即复现命令）；wire 不变量= message.rs 零 diff +
  PROTOCOL_VERSION 未触；I4=解释态 target_set 零 diff | next: merge
  （SD-03 一行指针 delta 文本随本记录，merge 期落 canonical）。

## 10. 待澄清事项

- **①（T-01 D1）** ✅ 已定案：槽位序+帧后重定位（候选 A）——§5.1 D1。
- **②（T-01 D2）** ✅ 已定案：投影器同线程代写 thread-local（候选 A，
  零生成器改动）——§5.1 D2。
- **③（T-01 附带）** ✅ 已定案：v1 点击定位；拖拽 not-yet 随注（§5.1
  附带定案 + P025-D2 归册）。
- **④（T-03/T-04）** ✅ 已按推荐落地：仓内 fixture
  `tests/fixtures/025-native-input/{slider,select}.at`（codegen golden
  真源）+ scratch025 载体（不入库，.gitignore，020 scratch020 先例）。
- **⑤（T-07）** ✅ 已按 D4 调查结果定：真机 iced 事件注入面缺席
  （P025-D1——live 壳无键盘订阅通道）→ 协议级承载（p025_native_input_arm
  经 broker_char/broker_scroll **新宿主生产路径**，非直注 client.end），
  留痕 assets/025/。
