---
plan_id: PLAN-031
status: drafting               # drafting → executing → execution_done → reviewed → archived
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
current_step: 0
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

- **T-01 [lang] 深水调查与定案**
  文件：`desktop_protocol/{broker,host,endpoint,transport,client_
  entry}.rs`、`ui/session.rs`（LiveInput/泵/开窗先例，读）、
  `crates/auto/src/main.rs`（CLI）、iced 0.14 daemon 多窗/管道命名
  语义（registry 源）+ §5.1（写面）。
  动作：D1–D7 定案。
  产物：`### 5.1 定案记录`（file:line 证据）。
  验证：定案完备；复审通过。
  → 全 AC 前置。新路径：是（rqhost 模块）。
- **T-02 [lang] rendezvous + 采纳端点**
  文件：新 `desktop_protocol/rqhost.rs` + `mod.rs` 注册。
  动作：§5.2 T-02；resolver 桩（桌面链零牵连）。
  验证：单测绿（并发接纳/零装载/探测）。
  → AC-01/02/03。
- **T-03 [lang] 多窗 daemon + `auto rqhost`**
  文件：`rqhost.rs`（daemon 装配/泵/view/末窗退出）、`crates/auto/
  src/main.rs`（子命令）。
  动作：§5.2 T-03。
  验证：单测 + 集成（单客户端全循环）绿。
  → AC-01/04/06。
- **T-04 [lang] 输入按窗路由**
  文件：`rqhost.rs`（window_id 映射 + LiveInput 复用接驳）。
  动作：§5.2 T-04。
  验证：分派单测（不串扰）。
  → AC-01/02。
- **T-05 [lang] 客户端接入 + 策略档**
  文件：`desktop_protocol/client_entry.rs`（adopt 助手 + 策略参数）、
  `crates/auto/src/cmd_autodesk.rs`（vm 轨 -q 分岔）。
  动作：§5.3 T-05；默认行为零变化（I2）。
  验证：单测（两态/退避/策略档）。
  → AC-01/04。
- **T-06 [lang] CLI 两轨 + ensure_rqhost + 竞态**
  文件：`crates/auto/src/main.rs`（-q 旗标）、`crates/auto-man/
  src/{automan,rust_ui}.rs`（vm 分岔/rust 注入）。
  动作：§5.3 T-06；D2 竞态处理。
  验证：单测 + 竞态用例绿。
  → AC-03/05。
- **T-07 [lang+os] e2e 与度量**
  文件：lang `stage3.rs`（p031_rqhost_arm）+ 截图 assets/031/；os
  smoke 脚本。
  动作：AC-01..06 逐条留痕 + 度量行。
  → AC-01..06。
- **T-08 [lang+os] 文档与台账收口**
  文件：lang `desktop-protocol-v1.md`（§1.11）、`virtual-desktop.md`
  （Design 23 §4 增行 + 裁定记录）；os 台账行 + 互链。
  动作：SD-01..04 落笔。
  → AC-07/08。

## 9. 复审记录

- 2026-09-19 /auto-plan:new 起草交接：`stage: new`，PLAN-031 rev 1
  （030 号被并行会话取用于 shell-outproc-client，本计划顺延 031——
  取号脚本守卫核过无撞）。`outcome: pass`（合同完整：broker 上门
  通道/resolver 权威假设缺口、直连宿主最小面双先例、多窗 daemon 与
  window_id 事件流、LiveInput 族在 master、CLI 侵入点与撞名前科、
  回收语义缺口全部 file:line 在案）；`next: work`（前置 029 ✅；
  建议 030 merge 后开工——同模块串行）。悬置决策 §10（①–⑤）均不
  阻塞 T-01。

## 10. 待澄清事项

- **①（T-01 D2）** 并发冷启动竞态：第二实例自杀退出（推荐——简单
  可测）vs 管道名原子注册（Windows 命名空间语义 T-01 核）。
- **②（T-01 D1）** well-known 管道名与多会话隔离（单用户单实例 vs
  会话域实例）；rendezvous 记录语法（沿 broker 管道串约定族）。
- **③（T-01 D5）** rqhost 死时 app 策略：exit-on-EOF（推荐——原生
  app 心智）vs 重找宿主（将来 `--rq-host` 生态）；桌面档 30s 重连
  默认不变。
- **④（T-01 D6）** `auto rqhost` 子命令（推荐）vs Run 隐藏旗标；
  Hello.icon → 窗口 icon 支持面（不支持则 v1 忽略随注）。
- **⑤（T-01 D7）** 保真清单口径：-q 全保真 examples 名单逐例判定
  文档化（smoke 演示位）；`--rq-host=` 参数面预留（desktop 实现出界
  归 030 线，接口先留）。
