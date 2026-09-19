---
plan_id: PLAN-031
status: reviewed                 # drafting → executing → execution_done → reviewed → archived
feature_name: rqhost-native-windows
author: [agent]
created_at: 2026-09-19
updated_at: 2026-09-19
plan_revision: 1

# /auto-plan:review 结束时填写：
supersedes_spec_components: []
new_spec_components:
  - auto-lang/docs/design/autoui/desktop-protocol-v1.md   # §1.11 v1.11 增量（rendezvous/采纳/生命周期语义，review 定稿）
  - auto-lang/docs/design/autoui/virtual-desktop.md      # Design 23 §4 后端矩阵增行（第四运行形态）
touched_goals: []

affects:
  - auto-lang/crates/auto-lang/src/ui/desktop_protocol/rqhost.rs      # 新模块：rendezvous + 采纳 + 多窗 daemon（新路径）
  - auto-lang/crates/auto-lang/src/ui/desktop_protocol/mod.rs         # 模块注册
  - auto-lang/crates/auto-lang/src/ui/desktop_protocol/client_entry.rs # 采纳助手 + exit-on-EOF 策略档
  - auto-lang/crates/auto-lang/src/ui/iced/broker_surface.rs          # DrawListPainter 每窗挂载接驳（复用面）
  - auto-lang/crates/auto/src/main.rs                                # Run -q 旗标 + rqhost 子命令
  - auto-lang/crates/auto/src/cmd_autodesk.rs                        # -q gate（run_if_client_entry 同位分岔）
  - auto-lang/crates/auto-man/src/{automan.rs,rust_ui.rs}            # vm 装载链分岔 + rust 轨注入
  - auto-lang/docs/design/autoui/{desktop-protocol-v1.md,virtual-desktop.md} # 协议增量 + 形态入册
  - auto-os/docs/plans/autos-desktop-program.md                      # 台账行
current_step: 8
total_steps: 8
---

# [PLAN-031] rqhost-native-windows

## 0. 变更摘要

给 AutoUI 增加**第四运行形态**：`auto run -r vm -q`（或 `-r rust -q`）
把 app 以**宿主 OS 普通原生窗口**形态启动——不启动虚拟桌面，app 进程
只产 RenderQueue/DrawList 帧，一个**共享的后台合成器进程（rqhost）**
为每个客户端开一个真实 OS 窗、栅格化、转发输入；多 app 共享单
rqhost 实例（compositor 架构，虚拟桌面同型）。用户裁定：不做
standalone 形态（单 app 时 shared 自动孵化即等价，多 app 时省
N-1 渲染进程）。

普查证实零件九成在库：broker 的**上门通道已存在**（serve_once 不校验
来源，broker.rs:91-130——缺的只是"客户端权威"采纳臂：绕过
`app_resolver` 本地编译，resolver 桩形态在测试装配双先例）；多可见窗
daemon 已生产（`auto run` 多 App 即是）；输入按窗分派 + LiveInput
映射族（029）已在 master；直连宿主最小面（listen + HostEndpoint +
手工泵）有双先例。**新设计面三件**：①rendezvous 采纳协议（well-known
管道 + per-app 管道先行 listen + 无源 attach）；②多窗 rqhost daemon
（每客户端一窗：Hello 的 title/icon/尺寸 → 开窗；canvas 栅格化；
window_id→client 输入路由；末窗退出）；③生命周期补缺（rqhost 死 →
app **退出**而非 30s 空等——exit-on-EOF 策略档）。CLI：`Run` 加
`-q/--render-queue`（-q 短名空闲已核；`--render` 撞名前科 → 孵化语义
旗标带名前缀或用 -q）+ `auto rqhost` 子命令（current_exe 自孵化
寻址先例 S:2952-2978）。**零 wire 变体**（Hello 无 frame_mode 请求位
→ 直连缺省 Commands 正合 queue 语义；rendezvous 记录是传输层管道串
约定，非 codec 面）。

## 1. 目标

- **G1 rendezvous 与采纳**：well-known 管道（用户会话域固定名）——
  客户端 `adopt` 记录 → rqhost 分配 per-app 管道（先行 listen，broker
  serve_once 同型）→ 客户端直连（`ClientTarget::Direct` 零改动）→
  HostEndpoint 握手（Hello 的 title/icon/width/height = 开窗凭据）；
  **客户端权威 attach**（resolver 桩，宿主零装载）。
- **G2 多窗 rqhost daemon**：每客户端一枚 `HostEndpoint`（单客户端
  状态机，endpoint.rs:421/:574-577）+ 一个真实 OS 窗（`iced::
  window::open` + register_window，detach_surface_to_os_window 先例
  S:3837-3864）；窗内容 = DrawListPainter canvas（broker_surface
  复用）；resize → `ControlMsg::Resize`；**末窗关闭 → daemon 退出**
  （iced 空窗不自动退出 R:14165-14167——自建）。
- **G3 输入按窗路由**：`listen_with` 的 window_id → (endpoint, wid)
  映射（替代桌面 WM hit_test）；LiveInput 族（029 在 master：
  named_key_vk/live_inputs_from_keyboard/IME/wheel）→ `InputMsg` 下发；
  修饰位 wire_modifiers。
- **G4 CLI 两轨对称**：`-q/--render-queue`——vm 轨在 `run_if_client_
  entry` 同位分岔（ensure rqhost → build_dynamic_component →
  `client_entry::run_dynamic_client(Direct)`，装载链零改）；rust 轨
  在 run_rust_ui 前解析 rendezvous、注入 `--autodesk-client=<pipe>
  --autodesk-render=queue`（args 透传已通 rust_ui.rs:2839-2853）。
- **G5 生命周期**：①app 退出 → EOF → 窗回收（pump_broker_clients
  回收臂搬用 S:3444-3472）；②关窗 → Close → app 退出码 0（端点状态机
  既有）；③**rqhost 死 → app exit-on-EOF**（-q 客户端策略档——区别
  于桌面 30s 重连；缺省退出 + 观测行）；④末窗退出 = daemon 退出。
- **G6 发现与孵化**：`ensure_rqhost`——探测（连上即关模板
  broker.rs:56-59）→ 不在则 spawn `auto rqhost`（current_exe 寻址
  先例）→ 就绪退避重连；**并发冷启动竞态**（双 -q 同时到达 → 单
  实例）有测试。
- **G7 验收与入册**：e2e（单/多 app、输入闭环、生命周期双向、竞态、
  降级显式）；desktop-protocol-v1.md **§1.11**（rendezvous/采纳/
  生命周期语义）；**Design 23 §4 后端矩阵增行**（第四运行形态——
  共享合成器原生窗）；保真清单（-q 下全保真 examples 名单 + 超覆盖
  降级显式）；台账行；smoke 脚本。

**非目标**（明确出界）：

- **B 形态桌面本体与其 ad-hoc attach**（`--rq-host=desktop` 参数面
  预留、实现归 PLAN-030 线——采纳协议设计与其同源，SD 入册时互链）。
- **standalone 形态**（用户裁定砍除——单 app 由 shared 自动孵化覆盖）。
- Hello 扩 frame_mode 请求位（直连缺省 Commands 正合 v1 语义；将来
  pixels-on-rqhost 需求时按追加式另立）。
- rqhost 内 WM 语义（虚拟窗/任务栏/布局——原生窗自带 OS 窗口管理）；
  多表面单客户端（一 app 多窗）；截图/缩略图（`window::latest()` 单窗
  假设链本计划不触碰）。
- job object/宿主亡 OS 级兜底（KD 债登记；exit-on-EOF 协议级兜底
  v1 够用）。
- 缺省形态翻转（inproc/pixels 缺省不动——-q 是显式 opt-in）。

## 2. 架构方案

```text
auto run -r vm -q 01-helloworld
  └─ ensure_rqhost：连 well-known 管道（连上即关探测）
       ├─ 在 → adopt 记录 → 收 per-app 管道名
       └─ 不在 → spawn `auto rqhost`（current_exe）→ 退避重试 → adopt
  └─ vm 轨：装载 .at（build_dynamic_component 既有链）
     → client_entry::run_dynamic_client(comp, opts, Direct(per-app pipe))
     （AppProjector 产帧；exit-on-EOF 策略档）
                    │ Direct 直连（既有，零改动）
                    ▼
┌─ rqhost daemon（auto rqhost 子命令；iced::daemon）──────────────────┐
│ well-known 管道 serve：adopt 记录 → 分配 per-app 管道先行 listen    │
│   （broker serve_once 同型）→ 回名                                 │
│ 每客户端：HostEndpoint（resolver 桩——客户端权威，零装载）          │
│   + shm 开段（autodesk-shm-<pid>-<surface>，槽 2/16KiB Commands 档）│
│   + Welcome/BufferAlloc（activate 既有）                           │
│   + 一个真实 OS 窗（Hello 的 title/icon/wh；register_window）       │
│ 泵循环（pump_broker_clients 形态搬用）：帧→SurfaceStore compose；   │
│   EOF→关窗回收；Close→端点状态机→app 退出                          │
│ view 按窗：DrawListPainter canvas（broker_surface 复用）            │
│ 输入：listen_with window_id → (endpoint,wid) → LiveInput 映射       │
│   （029 族复用）→ InputMsg 下发；resize → ControlMsg::Resize        │
│ 末窗关闭 → iced::exit（自建——空窗不退出的反面）                    │
└──────────────────────────────────────────────────────────────────┘
```

**不变式**：

- **I1 零 wire 变体**：codec/golden 零漂移（rendezvous 记录 = 管道串
  约定；Hello/Welcome/BufferAlloc 原样复用）；`PROTOCOL_VERSION` 仍 1。
- **I2 零回归**：`--autodesk-*` 既有旗标语义、桌面 broker 孵化链、
  双投影臂、直连测试装配全部零改动（-q 是新增 gate，不碰既有分岔）。
- **I3 not-yet 显式**：-q 跑超覆盖 demo = 占位盒/降级观测行可见
  （既有纪律），保真清单文档化。
- **I4 形态正交**：rqhost 不引入 WM/桌面语义；与 PLAN-030（shell
  outproc）的采纳协议同源不同宿主——接口设计互链不自缚。

**关键风险**：并发冷启动竞态（双 -q 同时探测失败双双 spawn → 双
实例——退避 + 第二实例退出策略，D2 定案）；rendezvous 与 per-app
listen 的时序窗（spawn 后就绪前 adopt 失败重试）；iced daemon 多窗
输入焦点门控的既有假设（desktop 模式绑 host.window 过滤 R:17486-
17492——rqhost 新形态需自己的 per-window 装配，不复用该过滤）；
exit-on-EOF 与 ReconnectPolicy 的策略分叉点（client_entry 参数面）。

## 3. 技术栈

Rust / iced 0.14（daemon 多窗 + listen_with window_id 事件流）；
desktop protocol 既有机器（transport listen/connect、HostEndpoint、
SurfaceStore、SharedFrameBuffer、五通道）；LiveInput 映射族（029 在
master）；DrawListPainter（broker_surface）；clap（Run 旗标 + rqhost
子命令）；验收载体 = examples/ui 01-helloworld / 003-converter（vm）+
004-profile-card / counter 级（rust，a2r 重生成）；e2e 门
AUTO_DESKTOP_E2E 同族 + 进程数/窗口句柄断言。

## 4. 需求分析与背景调查

**授权记录**：用户 2026-09-19 会话逐轮定形后明确"OK，按照这个定位来
立项吧"——定位 = 共享合成器原生窗运行时（不做 standalone；单 app
shared 自动孵化等价、多 app 省进程），`auto run -q`（vm/rust 两轨）。
**本轮仅规划，未授权实施**。涉及仓：auto-lang（协议模块/CLI/装载链/
文档）+ auto-os（台账/smoke）。无预算/自动续跑约束声明。

**前置依赖**：029 已归档 ✅（LiveInput 映射族在 master——本计划直接
复用）。PLAN-030（shell-outproc-client，并行会话在册）与本计划同
touch `desktop_protocol/` 模块树——**建议 030 merge 后执行**（同模块
串行；若并行须 worktree 调和，非硬前置）。

**现状事实**（已核，2026-09-19 master@086861a2b 含 029，探索代理
全量普查；路径缩写 P=desktop_protocol、S=ui/session.rs、
R=ui/iced/renderer.rs）：

- **上门通道已存在**：broker `serve_once`（P/broker.rs:91-130）
  listen broker 管道 → accept → 读孵化记录（探测 ping 吞掉 :94-98）
  → 分配 `<broker>-app-<n>` 管道**先行 listen** → 回名 →
  `Incubation{pipe_name, end, render}`——**不校验来源**（任何进程
  `request_incubation` :191-217 都受理）。真正缺口 = 认领逻辑：
  生产 `broker_apply_actions`（S:3674-3815）用 `desktop.app_resolver`
  在宿主本地重新编译一份组件（:3689-3696），MISS 静默弃连——"宿主是
  内容权威"假设。rqhost 需要"客户端即权威、宿主无源"attach 臂：
  resolver 桩（ProtocolHost resolver 注入闭包 P/host.rs:121 可传
  空桩——dual_mode.rs:112-120 / native_projector.rs:3216-3325 双
  先例即此形态）。
- **直连宿主最小面（现成模板）**：`transport::listen` +
  `PendingServer::wait_connect`（P/transport.rs:188-209，命名管道
  `\\.\pipe\<name>` :235-237）+ `ProtocolHost::new(session, resolver)` +
  手工泵；shm 宿主建（P/host.rs:157-165，`autodesk-shm-<pid>-<surface>`
  :161-162，槽 2/Commands=16384 档 S:3710-3716）；Welcome 由
  `endpoint.activate` 发（:166-181 + BufferAlloc）。**HostEndpoint 单
  客户端**（P/endpoint.rs:421 注释、:574-577 Active 拒第二条 Hello）
  → rqhost 每客户端一枚端点（BrokerClient 先例 S:2209-2214）。
- **Hello/Welcome 字段**（P/message.rs:308-325/:384-396）：Hello
  {version, app_name, **title, icon:Option<Vec<u8>>, width, height**,
  fonts}——开窗凭据齐备；**Hello 无 frame_mode 请求位**，直连缺省
  Commands（cmd_autodesk.rs:97-99 注释）——正合 -q queue 语义。
- **多窗 daemon 先例**：`auto run` 多 App = boot 逐 App
  `iced::window::open` + register_window（R:14633-14672）；运行期开窗
  = `detach_surface_to_os_window`（S:3837-3864）；daemon view 按窗
  （R:18574-18594，未登记窗回退占位）；**事件流带 window id**
  （`listen_with(|e,status,window_id|)` R:19288-19363 /
  `Event::Interaction{window,..}` R:8828-8837）；per-App 键盘订阅
  按窗+焦点过滤先例（R:8794-8837）。**iced 空窗不自动退出**
  （R:14165-14167 注释）——末窗退出要自建。
- **输入映射族（029 已入 master）**：LiveInput 六型（S:2316-2333）、
  `named_key_vk`（S:2358-2393）、`live_inputs_from_keyboard`
  （S:2395-2416）、IME/滚轮（S:2418-2439）、`wire_modifiers`
  （S:2341-2356）、统一入口 `route_live_input`（S:3648-3672）与
  broker_pointer_down hit_test→本地坐标（S:3476-3510）。
- **CLI 面**：`Commands::Run`（main.rs:401-434，`-r/--render` 已占用
  ——孵化旗标曾因撞名改 `--autodesk-render=`，cmd_autodesk.rs:13-16）；
  **`-q` 短名空闲**（grep 核）；客户端 gate = `run_if_client_entry`
  （main.rs:929-943 第一分岔 + cmd_autodesk.rs:37-44）——**-q gate 的
  落位先例**；vm 装载链 Automan::run→run_vm_ui（automan.rs:1484-
  1515）→run_file（rust_ui.rs:2956/3131）→run_dynamic_iced
  （lib.rs:4833-4844）；rust 轨 run_rust_ui（automan.rs:1466-1469）
  `cargo run` **args 逐个透传**（rust_ui.rs:2839-2853）+ 生成 main
  client gate（:1824-1897，`--autodesk-client=<pipe>` 直连已支持）。
- **客户端直连面**：`ClientTarget = Direct(String) | Broker{..}`
  （client_entry.rs:36-59）；`run_dynamic_client/run_native_client`
  （:64-98/:156-201）；重连 `ReconnectPolicy{30s,50ms}`（:89-90）——
  **无"宿主死即退"语义**（G5③ 策略档缺口）；`ensure_covered` 启动
  覆盖门（native 轨）。
- **回收语义**：EOF→wm_remove_win+释放（S:3444-3472）；Close→
  ExitRequest→ReclaimWindow+BufferRelease 端点状态机
  （P/endpoint.rs:302-310/:352-357/:619-630）；宿主 Drop 收割仅亲生
  outproc_children（S:2173-2179/:3331-3336）——**上门客户端无 Child
  句柄**，孤儿判定只能 EOF；无 job object。
- **自孵化先例**：`outproc_auto_binary`（S:2952-2978，current_exe/
  向上寻兄弟 auto.exe）+ `spawn_outproc_child` re-exec（S:2985-3007）；
  探测-孵化序模板 broker.rs:56-59。子命令结构：单 bin `auto`，
  Commands 枚举 main.rs:365 起 + match :788 起——`auto rqhost` 落位
  先例同族（cmd_autodesk 是 Run 臂内 gate 的另一形态先例）。
- **桌面 attach 面（出界）**：`--rq-host=desktop` 仓内零字样；桌面
  唯一上门 = 默认 broker 管道（`--autodesk-incubate` 不带
  `--autodesk-client` 即向其请求，cmd_autodesk.rs:101-104），attach
  仍要求 resolver 可解析——PLAN-030 线的 ad-hoc attach 与本计划采纳
  协议同源。
- **B 前置序列现状**：029 已归档（live 输入 + shell 面覆盖 ✅）；
  030-shell-outproc-client 并行在册（B 程序主体）——本计划是其
  平行受益件（采纳协议打样），非其前置。

**specs 现状**：协议权威 desktop-protocol-v1.md（v1.10 = 029 现行）；
Design 23（virtual-desktop.md §4 后端矩阵）无第四形态行——SD-02
增行；029 module spec provisional 在册。

## 5. 详细设计

### 5.1 T-01 深水调查定案（决策产物）

- **D1 采纳协议形态**：rendezvous 记录语法（沿 broker `incubate␟<name>
  ␟<mode>` 管道串约定族——`adopt␟<app_name>`？含 render 档位与否）；
  per-app 管道命名（`<wellknown>-app-<n>` 同型）；well-known 名
  （用户会话域：`autodesk-rqhost`？多会话/多用户隔离——Windows 管道
  命名空间语义核实）；resolver 桩形态（闭包 vs 新 attach 臂——
  **倾向**：rqhost 内新 `adopt_attach`（ProtocolHost 侧 resolver 桩
  直用，不碰 broker_apply_actions——桌面链零牵连））。
- **D2 发现/孵化/竞态**：探测（连上即关 broker.rs:56-59 模板）→
  失败则 spawn（`auto rqhost --listen`，current_exe 寻址）→ 就绪
  退避（50ms×n 上限）；**双 -q 并发冷启动**（双双探测失败双双
  spawn 的竞态）——候选 A = 第二实例启动时探测已在则自杀退出
  （**倾向**，简单）vs 管道名原子注册（Windows 命名管道
  FILE_FLAG_FIRST_INSTANCE 语义核实）。
- **D3 多窗宿主形态**：窗口注册表（window_id ↔ surface/wid/endpoint）；
  开窗凭据 = Hello(title/icon/wh)——icon 字节 → iced 窗口 icon 支持面
  核实（不支持则 v1 忽略随注）；view 按窗（未登记窗占位——R:18574
  先例）；DrawListPainter 挂载（broker_surface 复用 + per-surface
  front 缓冲取帧）；resize 双向（OS resize→ControlMsg::Resize→
  客户端重排→新帧）。
- **D4 输入路由**：listen_with window_id → (endpoint,wid) 映射表；
  鼠标本地坐标（窗内坐标即表面坐标——免 hit_test）；LiveInput 族
  复用（029 函数直调 or 提升为 pub 复用面）；焦点门控（OS 窗焦点
  = 事件流天然绑定，免 WM focus）。
- **D5 生命周期语义**：末窗关闭 → `iced::exit`（窗口注册表空判）；
  app EOF → 关窗 + 端点/表面/shm 释放；关窗 → Close 状态机既有；
  **rqhost 死 → app 退出**：client_entry 增策略参数（-q 档 =
  exit-on-EOF + 观测行；桌面档 = 既有 30s 重连不变——参数面默认值
  保持桌面行为，I2）。
- **D6 CLI 面**：`-q/--render-queue`（Run 旗标）+ `--rq-host=<target>`
  预留（缺省 well-known；desktop/显式管道 = 出界占位）；vm 轨分岔点
  （main.rs:931 旁——build_dynamic_component 后走 client_entry，
  装载链零改）；rust 轨注入点（run_rust_ui 前 resolve rendezvous →
  args 重写 `--autodesk-client=<pipe> --autodesk-render=queue`）；
  `auto rqhost` 子命令形态（Commands 变体 vs Run 隐藏旗标——**倾向**
  子命令，`--listen` 起服）。
- **D7 保真清单口径**：-q 全保真 examples 名单（解释态 target_set
  Covered 判定逐例跑）+ 超覆盖降级演示位（popover 等占位可见）——
  文档行 + smoke 演示。

定案记录追加 `### 5.1 定案记录`，作为 T-02..T-07 依据。

### 5.1 定案记录（T-01 产物，基线 lang-031@08526fda8 全量 file:line 重核）

**D1 采纳协议形态**：
- well-known 管道名 **`autodesk-rqhost`**（`autodesk-broker` broker.rs:22 同族）。
  测试缝 = env `AUTO_RQHOST_WELLKNOWN`（pid 后缀管道——P489 `adjudicate_on`
  可测性缝同型，生产行为零变化）。多会话隔离：Windows `\\.\pipe\` 命名
  空间机器级全局，v1 单实例=单机器域（与 `autodesk-broker` 同口径），
  多用户终端服务器隔离随 §1.11 注记为已知边界。
- adopt 记录语法 **`adopt␟<app_name>`**（DesktopBus 管道串约定族，verb=
  adopt），应答 `adopt␟<per-app pipe>`。**不含 render 档位字段**：-q
  唯一档=queue，Hello 无 frame_mode 请求位（I1 零 wire 变体）；将来
  pixels-on-rqhost 按 broker `incubate␟<name>␟<mode>` 第三字段先例追加式扩展。
- per-app 管道命名 `<wellknown>-app-<n>`（broker.rs:121 同型）。
- **resolver 桩形态定案：不引入 ProtocolHost**——rqhost 自建 `RqClient`
  （HostEndpoint + SurfaceStore + shm 直用，stage3.rs:38-55 `BrokerClient`
  同型轻装；动作臂=session.rs:3677-3815 `broker_apply_actions` 同构改写）：
  ResolveAndAttach 直接以 Hello 凭据 activate（app_id/wid = rqhost 自有
  计数器，rect=(0,0,w,h) 窗口本地坐标），**根本不设 resolver 闭包**——
  "客户端权威"落在适配层结构上，桌面链 broker_apply_actions 零牵连（I4）。
  理由：ProtocolHost 绑 `&mut DesktopSession`（host.rs:105-122——462
  虚拟窗 WM 对象，rqhost I4 出界）；计划 §2 架构盒本就枚举
  HostEndpoint+shm+activate 而非 DesktopSession。

**D2 发现/孵化/竞态**：
- 探测：`transport::connect(wellknown, timeout)` 连上即关（broker.rs:56-59
  模板）。connect 内建 FILE_NOT_FOUND(2)/PIPE_BUSY(231) deadline 重试
  （transport.rs:222-229）——spawn 后就绪前窗口期由探测超时预算吸收。
- `ensure_rqhost`：探测（~500ms 短超时）→ 失败 → spawn `auto rqhost`
  （current_exe 寻址，outproc_auto_binary 同位复用 session.rs:2956-2978）
  → 就绪退避重试（间隔 100ms 起指数退避，预算 ~8s）→ adopt。
- **并发冷启动竞态定案 = 锁管道原子注册（候选 B，推翻原倾向 A）**：
  tokio 1.53.1 `ServerOptions::first_pipe_instance`（FILE_FLAG_FIRST_PIPE_
  INSTANCE；注册源 tokio-1.53.1/src/net/windows/named_pipe.rs:1990-2005
  实证——"ensure that they are the only process listening…subsequent
  instances will fail with PermissionDenied"）。rqhost 启动先在锁管道
  `<wellknown>-lock` 以 first_instance create：成功=唯一实例（句柄驻留
  进程寿命）；PermissionDenied=已有实例 → 干净退出（码 0）。锁管道永不
  accept/永不重建（避开 serve_once 重听循环与 first_instance 多实例互斥
  ——adopt 管道本身沿用 broker 重听模式）。理由：候选 A 的探测-自杀窗
  非原子（双 spawn 同时探测失败双双驻留），锁管道 OS 级原子零窗口。
  transport 增 `listen_first_instance` API（pipe mod 内，非 Windows 空）。
- **新事实（影响 T-02 设计）**：broker `serve_once` 第二个 `wait_connect`
  （broker.rs:128）阻塞整个 serve 环——rust 轨 cargo build 分钟级延迟
  不得阻塞 adopt 环路 → rqhost 的 per-app `wait_connect` **线程化**
  （每次采纳一枚线程，连接后推 pending 队列，daemon tick 消费）。

**D3 多窗宿主形态**：
- 窗口注册表：daemon State 内 `iced::window::Id ↔ RqClient`（BTreeMap）。
  开窗在 update 内 `iced::window::open(Settings{size: Hello.wh,…})`，Task
  随 update 返回派发（detach_surface_to_os_window session.rs:3837-3864
  drop-task 先例证明登记即刻生效，正道仍走 Task 返回）；title 经 daemon
  `.title(TitleFn)` 按注册表解析（iced-0.14.0/src/daemon.rs:189-203）。
- Hello.icon：iced Settings 有 `icon: Option<Icon>`（iced_core-0.14.0/
  src/window/settings.rs:85-86 实证）但 Icon 需 RGBA+尺寸元数据，Hello
  携带的是编码字节 → **v1 忽略 + 观测行一次**，解码入册 §1.11 边界注记。
- view 按窗：注册表命中 → `drawlist_element(client.composed())`；未命中
  → 占位文本（renderer.rs:18574-18594 先例）。DrawListPainter 挂载 =
  broker_surface.rs:39/:465-472 `DrawListPainter`/`drawlist_element` 泛型化
  `<M>`（现绑死 DesktopMessage；canvas `Program<M>` 本就泛型 + PhantomData，
  既有调用点类型推断零改动）。
- resize：OS `Event::Window(Resized)` → `ControlMsg::Resize` 下发（AppEndpoint
  Active 接受 endpoint.rs:331-335）→ 客户端重排新帧；宿主侧表面尺寸簿记更新。
- 帧泵节奏：daemon 订阅 `iced::time::every(15ms)`（桌面 400ms ServiceTick
  对原生窗输入→帧响应太钝；15ms≈60fps 上限，空泵成本=N 管 try_recv 可忽略）。

**D4 输入路由**：
- `listen_with` 闭包（renderer.rs:19288-19363 模板）产 `Message::Input
  {window, …}` → 注册表反查 client → InputMsg 下发。鼠标坐标=窗内坐标
  即表面坐标（broker_pointer_down session.rs:3476-3510 的 hit_test+平移
  在原生窗形态退化为恒等——免 hit_test）。
- LiveInput 族直调：`live_inputs_from_keyboard`/`live_input_from_input_
  method`/`live_input_from_wheel` + `wire_modifiers`（session.rs:2341-2439，
  pub 纯函数，029 在 master）——rqhost 自己的 listen_with 闭包内调用，
  Ignored 门同 desktop_window_events（session.rs:7258-7289）；零提升零改签名。
- 焦点门控：OS 窗焦点天然绑定事件流（事件自带 window_id）——键盘路由=
  发生窗（替代桌面 wm.focused 焦点窗语义）；无 WM focus 语义引入（I4）。
- PointerPressed/Released：rqhost listen_with 增 Mouse 事件臂直订（桌面
  仅 CursorMoved/ButtonReleased——rqhost 需按下事件映射 InputMsg::

  PointerPressed/Released）。

**D5 生命周期**：
- 末窗关闭：`Event::Window(Closed)` → 注册表移除 → 空且曾有窗 → 
  `iced::exit()`。上游实证：daemon "will not stop running when all its
  windows are closed"（iced-0.14.0/src/daemon.rs:22-24——计划预判 R:14165
  注释正确）；boot 零窗待命不退（有窗标记门）。
- app EOF → pump 检出（is_eof/Err）→ `window::close` + 端点/表面/shm
  释放（pump_broker_clients session.rs:3417-3474 回收臂同构）。
- 用户关窗 → CloseRequested/Closed → `ControlMsg::Close` 下发 → app
  ExitRequest → ReclaimWindow（endpoint.rs:547-553 既有状态机）→ app
  退出码 0（ClientExit::Closed）。
- **rqhost 死 → app exit-on-EOF**：`ClientTarget` 增 `Rqhost { pipe }`
  变体——connect 走 Direct 同型；Commands 臂 reconnect=**None**
  （client_runtime.rs:2341-2345：无策略即 `ClientExit::HostLost` 即退——
  语义现成，缺口仅在 client_entry.rs:89-90/:189-190 桌面档硬编
  Some(30s/50ms)）+ HostLost 出口观测行。桌面档（Direct/Broker）默认
  30s 重连不变（I2）。枚举加变体对旧生成物源兼容（生成 gate 只构造
  不穷尽匹配，rust_ui.rs:1833-1897 实证）。

**D6 CLI 面**：
- `Run` 增 `-q/--render-queue`（clap bool；`-q` 短名空闲实证——Run 现占
  d/p/B/F/r，main.rs:402-433）；`--rq-host=<target>` **未实现预留**（出现
  即显式报错"预留"，desktop 实现归 030 线）。
- vm 轨分岔（对计划的微调，记录在案）：-q gate 落 main.rs Run 臂
  run_if_client_entry（:929-943）**同位之后、pac/api 侦测后**（~:1005，
  保证 vm/rust 判定与主流程同源）——ensure_rqhost+adopt → **env
  `AUTO_RQHOST_PIPE=<pipe>` 注入** → 主流程照常 am.run() → run_vm_ui
  （后端/主题/CWD 装载链零改，rust_ui.rs:2956-3144）→ lib.rs 
  run_file_dynamic_ui_inner（build_dynamic_component 后、run_dynamic_iced
  前，lib.rs:4834+）读 env 分岔 → `client_entry::run_dynamic_client
  (Rqhost)`。Hello 凭据：title=`AUTO_VM_TITLE` env 缺省 app 名；
  wh=`AUTO_VM_WINDOW` 解析（renderer.rs:6399-6413 同式；fit/缺席→480×320
  缺省，broker 子同款）。理由：main.rs:931 直叉需复刻 run_vm_ui 的
  CWD/主题/后端序——env 门+装载链内分岔才是"装载链零改"的字面兑现。
- rust 轨：同 gate（is_rust_api/render=rust）→ args 前注
  `--autodesk-client=<pipe> --autodesk-render=queue --autodesk-rqhost`
  （第三标记使生成 gate 选 Rqhost 目标——生成器 rust_ui.rs:1833-1897 增
  解析臂；旧生成物不识标记 → Direct+queue 渲染通但宿主死 30s 挂等，
  重生成后全语义——e2e 用新鲜生成物；注入点在 run_if_client_entry gate
  之后防 gate 劫持）→ cargo run 透传（rust_ui.rs:2839-2853 既有）。
- `auto rqhost` 子命令（Commands 枚举新变体 main.rs:365 族）：`auto rqhost
  [--pipe <name>]`（缺省 well-known；测试 pid 后缀）；match 臂 → 
  `rqhost::run_daemon(pipe)`。
- `-q` × vue/tauri/jet/arkts render：显式报错退出（出界组合）。

**D7 保真清单口径**：
- -q 渲染面 = DrawListPainter（与桌面 broker 窗同一栅格化器）→ 保真集合
  与桌面 outproc 队列臂同源：**vm 轨解释态全保真**（AppProjector 全
  vocabulary 投影，无覆盖门）；超覆盖演示 = popover 等 not-yet 词汇处
  占位盒 + 观测行（I3 既有纪律，broker_surface.rs:441-457 占位臂现成）。
- native 轨 -q 走 `ensure_covered` 既有门（client_entry.rs:179-182）。
- 清单落盘：desktop-protocol-v1.md §1.11 附注行（引 029 覆盖数据面，
  不重跑全量）+ os 仓 smoke 演示位（跑一个超覆盖 demo 验占位可见）。
- e2e 载体：vm=01-helloworld/003-converter（examples/ui 在册）；rust=
  a2r 重生成（examples/rust-workspace/counter 级）。

**调查修正与增量事实**（vs §4 普查）：
- 行号漂移（086861a2b→08526fda8）：pump_broker_clients 3444→**3417**；
  broker_apply_actions 3674→**3677**；outproc_auto_binary 2952→**2956**；
  broker serve_once :91-130/**:128 第二 wait_connect 阻塞**（新事实，D2
  线程化依据）；LiveInput 族 :2316-2439 精确一致。
- "宿主是内容权威"缺口确认：broker_apply_actions resolver MISS `continue`
  静默弃连（session.rs:3691-3696）——rqhost 适配层无此臂（D1 结构性根除）。
- iced 0.14 daemon API 全实证：`iced::daemon(boot,update,view)` +
  `.title/.subscription` + `.run()`（daemon.rs:27-31）；`iced::exit`
  （iced_runtime lib.rs:124）；window::open/close（iced_runtime window.rs:
  272/284）。
- ClientExit/ReconnectPolicy：`reconnect=None` 即 exit-on-EOF 现成语义
  （client_runtime.rs:2341-2345）——G5③ 实现面比计划预估更薄。

### 5.2 rqhost 核心（T-02/T-03/T-04）

- **T-02 rendezvous + 采纳端点**：rqhost 模块——well-known serve
  （adopt 记录 → per-app 管道先行 listen → 回名）+ per-client 装配
  （HostEndpoint + resolver 桩 + shm 开段 + activate）；单测（多
  客户端并发接纳 / resolver 桩零装载断言 / 探测 ping 吞掉）。
- **T-03 多窗 daemon**：`auto rqhost` 子命令 + iced::daemon 装配
  （boot 空窗待命 / 接纳后 window::open + register / view 按窗 /
  DrawListPainter / resize / 末窗退出）；泵循环（帧 compose + EOF
  回收 + Close 状态机驱动）。
- **T-04 输入路由**：window_id 映射 + LiveInput→InputMsg + 修饰位；
  单测（按窗分派不串扰）。

### 5.3 客户端与 CLI（T-05/T-06）

- **T-05 客户端接入**：`adopt_rqhost` 助手（探测/记录/收名）+
  client_entry 策略参数（exit-on-EOF 档）；vm 轨 -q 分岔；单测
  （Direct 连接复用既有 round-trip）。
- **T-06 CLI 两轨 + ensure_rqhost**：Run -q 旗标；vm 轨 gate；rust
  轨注入；`auto rqhost` 子命令注册；并发冷启动竞态处理（D2）。

### 5.4 验收与收口（T-07/T-08）

- e2e `p031_rqhost_arm`（AUTO_DESKTOP_E2E 门）：单 app（vm：01/
  003 键入换算；rust：004/counter 级重生成）→ 原生窗渲染 + 关窗
  双向退出；多 app 共享（进程数断言 = 1 rqhost + N app；双窗独立
  输入）；kill 双向（app→窗回收；rqhost→app 退出码非挂等）；
  竞态（并发双 -q 冷启动 → 单实例）；降级演示（超覆盖 demo 占位
  可见 + 观测行）。截图/进程清单留痕 assets/031/。
- 度量轻行：rqhost + 2 app 总内存 vs 2×inproc 直挂（数据行）。
- 文档：desktop-protocol-v1.md **§1.11 v1.11 增量**（rendezvous
  采纳协议/生命周期语义/与桌面 attach 的同源注记）；**Design 23
  §4 后端矩阵增行**（Win/Mac·原生窗·AutoUI App = 共享合成器
  rqhost 形态）；保真清单；台账行；smoke 脚本（os 仓）。

### 规范增量

| delta_id | add/modify/retire | docs/specs/... target | before/after rule | rationale | acceptance IDs |
|---|---|---|---|---|---|
| SD-01 | add | auto-lang/docs/design/autoui/desktop-protocol-v1.md（§1.11 v1.11 增量） | before：直连/ broker 双 target，孵化 = 宿主驱动 + resolver 内容权威；无宿主随客户端退出语义；after：增 rendezvous 采纳协议（well-known 管道 + adopt 记录 + per-app 管道 + 客户端权威 attach——宿主零装载）、rqhost 生命周期语义（末窗退出/app EOF 回收/宿主死 app exit-on-EOF 策略档）、与桌面 ad-hoc attach（PLAN-030 线）的同源注记——传输层管道串约定，零 codec 变体，PROTOCOL_VERSION 仍 1 | 协议权威收录第四形态的连接与生命周期语义 | AC-01..06 |
| SD-02 | modify | auto-lang/docs/design/autoui/virtual-desktop.md（Design 23 §4 后端矩阵） | before：三形态（直挂 inproc/桌面虚拟窗/独立窗）；after：增第四形态行——宿主 OS 原生窗·共享合成器（rqhost，RenderQueue 统一渲染词汇的第四消费面），用户裁定记录（standalone 否决：单 app shared 自动孵化等价） | 架构文档收录运行形态 | AC-07 |
| SD-03 | modify | auto-os/docs/plans/autos-desktop-program.md | before：无 rqhost 行；after：登记第四运行形态交付行（-q 两轨/共享实例/生命周期/保真清单） | 桌面程序台账 | AC-07 |
| SD-04 | modify | auto-lang/docs/specs/auto-lang/ui/（review 期按目录实况定） | before：无 rqhost 条目；after：rqhost 模块（rendezvous/采纳/多窗/输入路由）+ client_entry 策略参数 + CLI gate 条目（provisional） | 模块 spec 对齐实现 | AC-01..05 |

零 spec 影响的变更不存在（连接协议/运行形态为 spec 级知识）；ledger
（auto-lang `.autoos/specs.json`）随 merge 沉淀。

## 6. 测试设计

- **单测（rqhost 模块）**：rendezvous（adopt 记录解析/per-app 管道
  分配/探测吞掉）；多客户端并发接纳（N 端点 N 表面，shm 段名不撞）；
  resolver 桩零装载断言；泵循环（帧 compose/EOF 回收/Close 状态机）；
  末窗退出判定。
- **单测（客户端/CLI）**：adopt_rqhost（在/不在两态 + 退避）；
  exit-on-EOF 策略档（EOF → 退出 + 观测行 vs 既有 30s 重连默认不
  变——I2 断言）；vm 轨分岔参数面；rust 轨注入 args 重写。
- **集成（两进程）**：rqhost + 单客户端全循环（握手→帧→输入→
  resize→Close→退出码 0——native_client_full_cycle_over_pipe 形态
  ×rqhost 装配）；多客户端（双 app 双窗独立交互不串扰）。
- **e2e（AUTO_DESKTOP_E2E）**：`p031_rqhost_arm`（§5.4 全景）+
  竞态腿 + 降级演示腿；截图/进程清单留痕。
- **回归门**：desktop_protocol（含 broker/dual_mode/native 全循环）/
  session/stage3 + codec golden 零漂移（I1）+ `cargo t -p auto-man
  rust_ui` + auto-os 桌面 smoke（-q 不影响既有形态）。

## 7. 验收标准

- **AC-01 单 app 原生窗**：`auto run -r vm -q`（01-helloworld/
  003-converter）→ rqhost 自动孵化 → 原生窗 queue 帧渲染（标题/
  尺寸来自 Hello）；003 键入→换算联动帧变；关窗 → app 退出码 0 +
  窗回收。验证：e2e + 截图。
- **AC-02 多 app 共享**：两个 -q app → **单 rqhost 进程**（进程数
  断言）+ 双窗独立渲染与输入，互不串扰。验证：e2e 多客户端腿。
- **AC-03 发现与竞态**：冷启动自动孵化 + 就绪退避；实例在时直连
  零重复孵化；并发双 -q 冷启动 → 恰一实例。验证：单测 + e2e 竞态腿。
- **AC-04 生命周期双向**：kill app → EOF 窗回收；kill rqhost →
  app **干净退出**（exit-on-EOF，非 30s 挂等）+ 观测行；末窗关闭 →
  rqhost 退出。验证：e2e 双向腿。
- **AC-05 两轨对称**：`auto run -r rust -q`（生成 exe 直连注入）同
  AC-01 链；生成物独立窗直跑行为零变化（I2）。验证：e2e rust 腿 +
  AC-07 回归。
- **AC-06 resize 闭环**：OS 窗 resize → ControlMsg::Resize → 客户端
  重排新帧。验证：集成 + e2e。
- **AC-07 降级显式与回归**：超覆盖 demo 占位可见 + 观测行；保真
  清单文档落盘；codec golden 零漂移；§6 回归门全绿（在册红除外）。
- **AC-08 文档入册**：§1.11 + Design 23 矩阵行 + 台账行 + smoke
  落盘互链。验证：文档交叉引用可解析。

## 8. 执行步骤

**前置**：029 已归档 ✅；**建议 030 merge 后执行**（desktop_protocol
同模块串行；非硬前置）。依赖序：T-01 → T-02 → T-03 → {T-04, T-05
并行} → T-06 → T-07 → T-08。lang worktree
`D:/autostack/.wt/lang-031/auto-lang`；os `D:/autostack/.wt/os-031/
auto-os`。

- **T-01 [lang] 深水调查与定案** [x] [✅ 已完成 2026-09-19]
  文件：`desktop_protocol/{broker,host,endpoint,transport,client_
  entry}.rs`、`ui/session.rs`（LiveInput/泵/开窗先例，读）、
  `crates/auto/src/main.rs`（CLI）、iced 0.14 daemon 多窗/管道命名
  语义（registry 源）+ §5.1（写面）。
  动作：D1–D7 定案。
  产物：`### 5.1 定案记录`（file:line 证据）。
  验证：定案完备；复审通过。
  → 全 AC 前置。新路径：是（rqhost 模块）。
  证据：§5.1 定案记录 D1–D7 全落（基线 lang-031@08526fda8 重核；
  D2 竞态翻案锁管道原子注册[tokio first_pipe_instance 注册源实证]；
  D1 裁定不引入 ProtocolHost 自建 RqClient；D6 vm 轨分岔点微调
  env 门+装载链内分岔；§10 ①–⑤ 全闭）。
- **T-02 [lang] rendezvous + 采纳端点** [x] [✅ 已完成 2026-09-19]
  文件：新 `desktop_protocol/rqhost.rs` + `mod.rs` 注册。
  动作：§5.2 T-02；resolver 桩（桌面链零牵连）。
  验证：单测绿（并发接纳/零装载/探测）。
  证据：commit c33cebe38——单测 6/6 绿（rendezvous 往返/探测吞掉、锁管道
  双声明+serve 级 AlreadyRunning、未知 app 名零装载采纳到 Active、3 客户端
  并发 surface 不撞、帧合成 Ack/回收/EOF、ensure 退避全序替身孵化器）；
  transport/broker 既有测试零回归（9/9+3/3）。D1 裁定不引入 ProtocolHost
  （自建 RqClient 复用 BrokerClient）；D2 定案锁管道原子注册
  （tokio first_pipe_instance，CLAIM_DENIED_MARKER 免疫 OS 消息本地化）。
  → AC-01/02/03。
- **T-03 [lang] 多窗 daemon + `auto rqhost`** [x] [✅ 已完成 2026-09-19]
  文件：`rqhost.rs`（daemon 装配/泵/view/末窗退出）、`crates/auto/
  src/main.rs`（子命令）。
  动作：§5.2 T-03。
  验证：单测 + 集成（单客户端全循环）绿。
  证据：commit 801ed51fb——broker_surface 泛型化（DrawListPainter<M>，
  唯一消费面类型推断承接）；rq_update/rq_view/rq_subscription + 15ms 帧泵
  + 末窗退出门（无在册窗∧无待定∧曾有窗→iced::exit）+ resize 下发；
  集成 rqhost_full_cycle_over_pipe：真 ClientPump 全循环（采纳→握手→帧
  →resize→Close→Reclaim→BufferRelease→ClientExit::Closed）7/7 绿；
  auto rqhost [--pipe] 子命令 + auto 构建通过。
  → AC-01/04/06。
- **T-04 [lang] 输入按窗路由** [x] [✅ 已完成 2026-09-19]
  文件：`rqhost.rs`（window_id 映射 + LiveInput 复用接驳）。
  动作：§5.2 T-04。
  验证：分派单测（不串扰）。
  证据：commit 3c8363e68——Live/CursorMoved/PointerPressed/Released 四臂
  + live_input_msgs 六型映射（029 纯函数直调）+ last_cursor 窗级簿记 +
  Ignored 门订阅；input_routes_by_window_without_crosstalk：双客户端双窗
  四型输入 A 端按序全收 wid 随行正确/B 端静默（8/8 绿）。
  → AC-01/02。
- **T-05 [lang] 客户端接入 + 策略档** [x] [✅ 已完成 2026-09-19]
  文件：`desktop_protocol/client_entry.rs`（adopt 助手 + 策略参数）、
  `crates/auto/src/cmd_autodesk.rs`（vm 轨 -q 分岔）。
  动作：§5.3 T-05；默认行为零变化（I2）。
  验证：单测（两态/退避/策略档）。
  证据：commit 8b1602838——ClientTarget::Rqhost{wellknown,app_name}
  （rendezvous 采纳内建）+ reconnect_for 策略档（Rqhost=None exit-on-EOF
  + 观测行；Direct/Broker=30s/50ms 不变）+ lib.rs run_file_dynamic_ui_
  inner env 门分岔（AUTO_RQHOST_PIPE=wellknown——装载链零改，D6 微调
  落位）+ run_vm_rqhost_client 凭据 env 消费；单测 +3（策略档 I2 断言/
  ensure 就绪退避/vm fork 凭据+生命周期）10/10 绿。**设计修正**（测试
  暴露）：原 ensure 采纳后弃端会烧掉一次性 per-app 管道实例——ensure
  改只探活（ensure_rqhost_ready），采纳内建到 ClientTarget::Rqhost 与
  各轨客户端；serve 兼容 incubate 动词（broker 族记录——旧生成物零改
  接 rqhost）。
  → AC-01/04。
- **T-06 [lang] CLI 两轨 + ensure_rqhost + 竞态** [x] [✅ 已完成 2026-09-19]
  文件：`crates/auto/src/main.rs`（-q 旗标）、`crates/auto-man/
  src/{automan,rust_ui}.rs`（vm 分岔/rust 注入）。
  动作：§5.3 T-06；D2 竞态处理。
  验证：单测 + 竞态用例绿。
  证据：commit 149d299c7——Run -q/--render-queue + gate（am.run 前：
  ensure→双信号并发注入[vm env + rust args 前注]各轨只认各的；出界
  render 显式报错；--rq-host 预留位拒绝）；生成 gate --autodesk-rqhost
  标记臂（旧生成物安全忽略退化 broker 档）；run_rust_ui cargo `--`
  分隔符前置修复（`- 开头透传参数原被 cargo 吞）；单测×3（gate 校验
  档位/注入形状/生成内容 rqhost 臂）+ CLI help 冒烟；D2 竞态 = 锁管道
  原子注册（T-02 已落 lock_pipe_second_claim_fails——serve 级
  AlreadyRunning 断言）。
  → AC-03/05。
- **T-07 [lang+os] e2e 与度量** [x] [✅ 已完成 2026-09-19（含 R1..R4 与 R5 修复轮）]
  文件：lang `stage3.rs`（p031_rqhost_arm）+ 截图 assets/031/；os
  smoke 脚本。
  动作：AC-01..06 逐条留痕 + 度量行。
  证据：e2e p031_rqhost_arm 六腿真机 PASS（①003-converter 采纳→开窗
  →首帧 ②001-helloworld 共享双窗 + 第二 daemon 锁管道码 0 退 ③
  SetWindowPos→resize 观测行 ④kill 双向[app→EOF 窗回收观测/daemon→
  host lost 观测行+码 0] ⑤未解析 image→[drawlist-image] 占位观测行
  ⑥度量行[daemon 314MB + 每 app 7-9MB private] + Win32 PrintWindow
  BMP 截图/进程清单/daemon-stderr 留痕 reports/assets/031/）；
  **附带根修**：e2e 首跑暴露 client_runtime push_frame 超槽帧静默弃帧
  冻结（既有缝，桌面 broker 同益）→ 回退管道内联 FrameReady + 合成
  20KB 帧回归钉；os 侧 smoke-031-rqhost.sh 生产 well-known 全链演示
  PASS（含 -d 旗标只喂 pac 的既有怪癖记录 + exec 形态 spawn 工程坑）。
  **修复轮（P031-R1..R4，commit 34bb52532）**：R3 末窗门单测
  last_window_close_exits_daemon（rq_update 真行为四态）+ e2e 关窗 X 腿
  （WM_CLOSE→app 码 0）+ 末窗自退腿（降级窗关→daemon3 自退码 0+观测行）；
  R2 键入闭环改集成承载 vm_typing_loop_over_pipe（真管道×真 003 源×真
  ClientPump×rq_update 输入臂：键入 100→宿主合成帧文本 212[p025 同级]；
  hello 帧文本不变+revision 零前进=不串扰）——e2e 真机合成输入撤腿=本机
  ToDesk 输入钩子类环境对合成输入不生效（SendInput 零送达×2 + winit 0.30
  WM_POINTER 路径忽略 legacy 投递消息；native_dock_e2e T4 同款环境事实
  先例）；R4 自动孵化 e2e 腿（不预起 daemon→子进程自 spawn→锁持有→末窗
  自退→锁让出）；R1 rust 轨 e2e 腿（counter 强制重生成→cargo -- 注入→
  采纳开窗+首帧→WM_CLOSE 码 0；**根修=子进程 stderr PIPE 必须排水
  [LineTail]——cargo 警告超 4KB 缓冲阻塞写端=构建假死**）。e2e 复跑全绿
  23.56s（七腿）；desktop_protocol 183 绿+在册红×2（基线同红）；rqhost
  13/13；rust_ui 复跑 23/23。
  → AC-01..06。
- **T-08 [lang+os] 文档与台账收口** [x] [✅ 已完成 2026-09-19（含 R6 新鲜度修复轮）]
  文件：lang `desktop-protocol-v1.md`（§1.11）、`virtual-desktop.md`
  （Design 23 §4 增行 + 裁定记录）；os 台账行 + 互链。
  动作：SD-01..04 落笔。
  证据：SD-01 协议顶表 v1.11 行 + §1.11 全节（PROTOCOL_VERSION 仍 1，
  I1）；SD-02 Design 23 §4 后端矩阵 Win/Mac·rqhost B 形态行 + 内存
  注记（314MB+7-9MB 实测）+ standalone 裁定记录；SD-03 os 台账裁定
  登记簿 3d 行（030 预订 3c3 不撞）；SD-04 ui overview.md rqhost
  provisional 指针节（§1.10 节式）。交叉引用：§1.11 ↔ Design 23 §4 ↔
  台账 3d ↔ PLAN-030 同源注记互链可解析。
  → AC-07/08。

## 9. 复审记录

- 2026-09-19 /auto-plan:review 第三轮（R5/R6 闭环，终审）：`stage:
  review | PLAN-031 | rev 1 | pass | reviewed_commit: lang af3bd9fd2 /
  os e0cd1ce | base: lang 08526fda8 / os 3cd6b12 | deps: auto-down
  dep-031@a615d69 | spec_inputs: desktop-protocol-v1.md@§1.11（含输入
  闭环承载裁定段）+ virtual-desktop.md@Design23§4 + specs/auto-lang/ui/
  overview.md + autos-desktop-program.md@3d（七腿）| acceptance:
  AC-01..08 全 pass | findings: 无新发现（R5/R6 修复验证通过）|
  evidence: **本轮增量**——e2e 复审员复跑 PASS 20.00s（rust 腿"零残留"
  断言在场[代码 ×2 处]且腿后 rust-workspace git 0 脏实证）+ §1.11/
  overview/台账 3d 计数与裁定段落核验新鲜 + scoped desktop_protocol
  183 绿+基线红×2（同前）；**复用（重复复审条款，理由：自 34bb52532
  以来 diff = stage3.rs[纯 e2e 门控测试码]+两文档，R1..R4 实现与
  gate 外测试面字节不变）**——rqhost 13/13@af3bd9fd2、tf 全量门
  3639/3643（4 红=master 谱系在册转交：ui_gen::rust×2/kitchen_sink/
  ffi_dual_019，022 旧树全绿+diff 零重叠归因链在第二轮记录）、
  session 71/rust_ui 23、smoke 全链 | next: merge。
  **独立性声明：实施会话内三轮复审，结论均自工件重构。**
- 2026-09-19 /auto-plan:work 修复轮（R5/R6）：`stage: work | PLAN-031 |
  rev 1 | pass | code: lang af3bd9fd2 + os 台账七腿提交 | task_ids:
  T-07/T-08 | evidence: e2e 复跑 PASS 37.87s——rust 腿带"零残留"清洁
  断言通过 + rust-workspace git 状态 0 脏实证（断言首跑自证抓出
  Drop 时序盲区后显式 restore.run() 修正）；rqhost 13/13 复跑绿；
  §1.11/overview/台账 3d 计数与裁定同步 | blockers: 无 | next: review
  第三轮（可按重复复审条款复用第二轮未变证据——本轮仅动 e2e 卫生
  断言与文档，R1..R4 相关实现与测试零变更）。
- 2026-09-19 /auto-plan:review 第二轮（R1..R4 闭环）：`stage: review |
  PLAN-031 | rev 1 | needs_fix | reviewed_commit: lang 34bb52532 |
  base: 08526fda8 | deps: auto-down dep-031@a615d69 | spec_inputs: §1.11
  + Design23§4 + ui overview + 台账 3d | acceptance: AC-01..06 pass；
  AC-07 pass（回归门：rqhost 13/13 复跑绿 + desktop_protocol 183 绿 +
  session/rust_ui 沿用 + **cargo tf 全量门 3639/3643**——4 红全数转交
  在册：ui_gen::rust ×2 / docs_gen kitchen_sink / ffi_dual_019，归因链 =
  lang-022 旧 worktree 四测全绿 + 本计划 diff 零重叠（ui_gen/schema/
  ffi_dual/kitchen-sink 皆未触碰）+ 失败夹具内聚 → master 谱系回归
  （012-clock/scroll-schema 世代合并线，属主 session 处理））；
  AC-08 partial（R6）| findings: P031-R5/R6（下）| evidence: e2e 七腿
  复跑 PASS 28.37s（关窗 X/末窗自退/自动孵化/rust 腿四新腿全过）+
  rqhost 13/13 + vm_typing 212 联动 + tf --no-fail-fast 清单
  /tmp 同步计划 | next: work 修复 R5/R6（轻量）。
  **独立性声明：实施会话内复审，结论自工件重构（复跑+归因链）。**

  **P031-R5**（needs_fix→work；T-07）：e2e rust 腿生成污染 workspace
  根 Cargo.toml（members 增 "002-counter"）——RestoreFiles 只护 counter
  成员两件，漏护工作区清单；**实证击穿本复审的 tf 首跑**（脏文件在场
  →2 红烟幕+1864 测未跑）。修正：restore 列表加
  examples/rust-workspace/Cargo.toml + 腿末 git status 清洁断言。
  **P031-R6**（needs_fix→work；T-08）：spec delta 新鲜度——§1.11 验证面
  计数陈旧（"11 项"→13 项、"六腿"→七腿）+ 修复轮持久裁定未记（键入
  闭环集成承载 + ToDesk 合成输入环境裁定；stderr 排水根修入证据注）；
  台账 3d 行"六腿"同步。
- 2026-09-19 /auto-plan:work 修复轮（R1..R4）：`stage: work | PLAN-031 |
  rev 1 | pass | code: lang 34bb52532（+观测行/排水根修含其中）|
  task_ids: T-07 | evidence: e2e p031_rqhost_arm 七腿全绿 23.56s（新增
  关窗 X/末窗自退/自动孵化/rust 轨四腿）；rqhost 13/13（+末窗门四态
  单测 +键入闭环集成测试[真 003 源 212 联动+零串扰]）；desktop_protocol
  183 绿+在册红×2 基线同红；rust_ui 23/23 | blockers: 无 | next: review
  （R1..R4 闭环复审）。执行期环境裁定记录：真机合成输入（SendInput/
  PostMessage）在本机 ToDesk 钩子类环境不可用——键入闭环按 486 两级
  退路先例改集成承载；屏幕截图留痕裁撤（agent 桌面覆盖层入镜两轮实证
  ——留痕=进程清单+daemon stderr+revision 观测行）。
- 2026-09-19 /auto-plan:review 第一轮：`stage: review | PLAN-031 | rev 1 |
  needs_fix | reviewed_commit: lang 5a64bd474 / os d096fb5 | base: lang
  08526fda8 / os 3cd6b12 | deps: auto-down dep-031@a615d69（零内容改动）|
  spec_inputs: desktop-protocol-v1.md@§1.11 + virtual-desktop.md@Design23§4
  + specs auto-lang/ui/overview.md + autos-desktop-program.md@3d 行 |
  acceptance: AC-01 partial / AC-02 partial / AC-03 partial / AC-04 partial /
  AC-05 fail / AC-06 pass / AC-07 pass / AC-08 pass |
  findings: P031-R1..R4（下）| evidence: 复审员重跑 rqhost 11/11 绿 +
  session 71/71 绿 + cmd_autodesk 2/2 绿 + e2e p031_rqhost_arm 复跑 PASS
  3.10s（AUTO_DESKTOP_E2E=1，真机）；diff 范围与 affects 清单逐一对应
  （17 文件，无越界）；spec delta 四落点锚定核验；在册红×2 基线对照
  沿用工作期同提交同配置记录（复审期不再 stash——栈跨 worktree 共享
  已实证交叉风险）；cargo tf 全量门推迟至 pass 轮（needs_fix 代码将
  变更，重跑浪费）| next: work 修复 R1..R4（全在 T-07 范围）。
  **独立性声明：本轮在实施会话内复审——结论自工件重构（测试重跑 +
  测试体重读枚举腿位），未采信执行摘要。**

  **P031-R1**（fail；AC-05/T-07）：rust 轨 e2e 腿缺席——`-r rust -q`
  全链运行时零执行（生成 gate `--autodesk-rqhost` 解析臂、cargo `--`
  透传、a2r 产物上下文的 Rqhost 采纳仅编译级/内容断言）。修正：T-07
  补 rust 腿（counter 级重生成 → -q → 原生窗首帧 → 关窗退出；首跑
  cargo build 分钟级，加预算）。
  **P031-R2**（partial；AC-01/AC-02/T-07）：e2e 缺"003 键入→换算
  联动帧变"与"双窗输入互不串扰"腿——输入闭环现仅单测（真管道
  dispatch）+vm_fork 协议级；AC-01/02 明文 e2e 口径。修正：SendInput
  腿（029 sendinput.rs 组装层 + 前台化）或协议级注入腿入 e2e 文；
  帧变观测 = PrintWindow 前后像素差或 daemon 帧计数观测行。
  **P031-R3**（partial；AC-01/AC-04/T-07）：用户关窗（X）→app 退出码
  0 路径无 e2e；**AC-04"末窗关闭→rqhost 退出"门（无窗∧无待定∧曾有
  窗→iced::exit）零测试（单测亦无）**。修正：PostMessage WM_CLOSE 关
  converter 窗→app 码 0；再关末窗→daemon 进程退出（wait_pid）；
  末窗门补 rq_update 级单测（条件判定提取可测或 serve stop 旗标副作用）。
  **P031-R4**（partial；AC-03/T-07）：真实自动孵化 e2e 缺席——e2e/
  smoke 均预起 daemon；ensure spawn 真 `auto rqhost` 仅替身孵化器单测。
  修正：冷启动腿（不预起 daemon → 直接 spawn -q 子进程 → 窗/首帧 →
  daemon 由子进程孵化断言）。
- 2026-09-19 /auto-plan:work 执行收口：`stage: work | PLAN-031 | rev 1 |
  pass | code: lang plan-031-dev 08526fda8..5a64bd474（T-07 2feca90da/T-08 5a64bd474；T-02 c33cebe38/
  T-03 801ed51fb/T-04 3c8363e68/T-05 8b1602838/T-06 149d299c7/T-07 e2e/
  T-08 文档）+ os plan-031-dev（smoke + 台账 3d）| T-01..T-08 全勾 |
  证据：rqhost 单测/集成 11 绿；e2e p031_rqhost_arm 六腿真机 PASS
  （AUTO_DESKTOP_E2E）+ 留痕 reports/assets/031/（BMP 截图×2/进程清单
  /daemon-stderr）；os smoke 生产管道全链 PASS；回归门 desktop_protocol
  181 绿 + 在册红×2（coverage target_set/demo parity——merge-base 基线
  全量并行同红，豁免口径 AC-07）+ session 71 绿 + auto-man rust_ui 23
  绿 + codec golden 零漂移（suite 内含）；§5.1 定案 D1-D7 全兑现（D2
  竞态锁管道原子注册/T-05 设计修正[ensure 只探活，采纳内建
  ClientTarget::Rqhost——原设计会烧一次性管道实例，测试暴露]记录在案）|
  blockers: 无 | next: review（/auto-plan:review）。
- 2026-09-19 /auto-plan:new 起草交接：`stage: new`，PLAN-031 rev 1
  （030 号被并行会话取用于 shell-outproc-client，本计划顺延 031——
  取号脚本守卫核过无撞）。`outcome: pass`（合同完整：broker 上门
  通道/resolver 权威假设缺口、直连宿主最小面双先例、多窗 daemon 与
  window_id 事件流、LiveInput 族在 master、CLI 侵入点与撞名前科、
  回收语义缺口全部 file:line 在案）；`next: work`（前置 029 ✅；
  建议 030 merge 后开工——同模块串行）。悬置决策 §10（①–⑤）均不
  阻塞 T-01。

## 10. 待澄清事项

- **（执行期发现，转交）examples 生成物漂移**：lang-031 worktree 测试
  跑动偶发物化 `examples/rust-workspace/015-notes/src/main.rs` +
  `Cargo.toml` 重生成 diff（lucide icon 映射/012-clock 成员剔除——与
  主检出 656 会话的同类未提交 WIP 内容**不同源**；master 已提交
  main.rs 相对现行生成器已陈旧）。已在 031 worktree 恢复 HEAD 不夹带；
  归属建议 = 生成物刷新批次（656 会话或独立小批）。
- **（执行期发现，技能面）git stash 栈跨 worktree 共享**：本组
  stash/pop 与并行会话存在交叉风险（本轮实证主检出 WIP 未受损）——
  后续并行会话建议弃用 stash 改 worktree 内 commit 分支。

- ~~①（T-01 D2）~~ 并发冷启动竞态：**已定案（T-01）**——锁管道原子
  注册（tokio `first_pipe_instance`，OS 级原子零窗口；推翻第二实例
  探测自杀案——其窗口非原子）。见 §5.1 D2。
- ~~②（T-01 D1）~~ well-known 管道名与多会话隔离：**已定案（T-01）**——
  `autodesk-rqhost` 固定名 + 测试缝 env `AUTO_RQHOST_WELLKNOWN`；
  单实例=单机器域（broker 同口径），多用户终端服务器随 §1.11 注记。
  rendezvous 记录 = `adopt␟<app_name>`（无 render 字段，I1）。见 §5.1 D1。
- ~~③（T-01 D5）~~ rqhost 死时 app 策略：**已定案（T-01）**——
  exit-on-EOF（`ClientTarget::Rqhost` 变体 + reconnect=None 现成语义）；
  桌面档 30s 重连默认不变。见 §5.1 D5。
- ~~④（T-01 D6）~~ `auto rqhost` 子命令：**已定案（T-01）**——子命令
  （`--pipe` 参数化）；Hello.icon → iced Settings 有 icon 位但需 RGBA
  尺寸元数据，v1 忽略 + 观测行。见 §5.1 D6/D3。
- ~~⑤（T-01 D7）~~ 保真清单口径：**已定案（T-01）**——vm 轨解释态
  全保真 + not-yet 词汇占位/观测行（I3 现成纪律）；native 轨走
  ensure_covered 既有门；`--rq-host=` 参数面预留未实现（显式报错）。
  见 §5.1 D7。
