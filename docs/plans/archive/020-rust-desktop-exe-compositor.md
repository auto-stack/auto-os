---
plan_id: PLAN-020
status: archived              # drafting → executing → execution_done → reviewed → archived
feature_name: rust-desktop-exe-compositor
author: [agent]
created_at: 2026-09-15
updated_at: 2026-09-15
plan_revision: 1

# /auto-plan:review 结束时填写：
supersedes_spec_components: []
new_spec_components:
  - auto-lang/docs/design/autoui/desktop-protocol-v1.md   # §1.6 v1.6 增量（review 定稿：ea91243f1 + 复审补正）
touched_goals: []

affects:
  - auto-lang/src/ui/desktop_protocol     # 客户端入口泛化 + pixels/queue 臂 Component 泛型
  - auto-lang/src/ui/session.rs           # AppSpec exe 发现 + outproc spawn 分流
  - auto/src/cmd_autodesk.rs              # 薄壳化（复用泛型客户端入口）
  - auto-man/src/rust_ui.rs               # 生成 main 的 autodesk 客户端臂
  - auto-os/docs/plans/autos-desktop-program.md  # 程序行更新
current_step: 6
total_steps: 9
---

# [PLAN-020] rust-desktop-exe-compositor

## 0. 变更摘要

打通"最终形态 Rust 桌面"的最后一环：让 a2r 编译产物（`auto build -r rust`
生成的独立 exe）能作为**渲染客户端**接入虚拟桌面的 compositor——桌面宿主
（已是编译 Rust exe，`ui_desktop`）经 broker 孵化 spawn 编译 exe 子进程，
子进程以 RenderQueue（DrawList 命令帧）或 independent（隐藏窗自渲 +
screenshot 像素帧）二态回传，宿主统一合成进虚拟窗。

现状底座已备（Plan 386/480/500/507/508 六阶段收线）：五通道协议、命名管道、
共享内存双槽、broker、宿主两态合成、三态渲染裁决全部实现——但 outproc 子
进程目前是 `auto.exe` re-exec **重新解释 .at**（`session.rs` 的
`spawn_outproc_child`），且协议客户端臂（`AppProjector` /
`pixels::run_independent_child`）绑死解释态 `DynamicComponent`。本计划做三件事：
**①协议客户端入口泛型化**（Component trait seam）、**②a2r 生成器加客户端臂**
（生成 main 解析孵化参数 → 客户端循环，无标记零变化）、**③宿主注册表识别
编译产物并分流 spawn**。协议 `PROTOCOL_VERSION` 维持 1（追加式纪律）。

## 1. 目标

- **G1 编译 exe 作为桌面 App**：ui_desktop 宿主可孵化 a2r 编译 exe，虚拟窗
  渲染、输入交互、关闭回收全链闭环（子进程 = app 自己的 exe，非 auto re-exec）。
- **G2 queue 臂（RenderQueue）覆盖 native Component**：AppProjector 从
  `DynamicComponent` 泛化到 Component trait seam，counter 级 App 全链走
  DrawList 命令帧（子进程免 GPU 上下文），auto 裁决链（spawn 参数 >
  pac `desktop_render:` > auto）复用。
- **G3 pixels 臂（independent）保底泛化**：任意 Component 可走隐藏窗自渲 +
  screenshot 像素帧，超覆盖 App 经 auto 裁决降级，无静默错绘。
- **G4 度量收口**：编译 exe 形态的每 App 边际内存与启动时延实测，对照
  Plan 508 基线（inproc 0.86 MiB/App vs outproc-解释 6.48）落报告。

**非目标**（明确出界）：

- GPU 纹理共享（D3D shared handle / IOSurface）——virtual-desktop.md §4
  后端矩阵行维持 designed-only。
- 桌面 shell 自身 a2r 编译化——shell 四件 `.at` 仍在宿主内解释装载
  （宿主本身已是 Rust exe，满足"独立 exe 桌面"）；shell 编译形态另立计划。
- Linux Smithay 宿主 / Wayland 客户端接入（509 Stage 2 另线）。
- 解释型 App 的 inproc 缺省翻转（508 裁定不动；exe App 天然 outproc）。
- GPUI 后备臂的客户端支持（v1 限 iced，生成物 cfg 门控）。
- L1/L3 窗口迁移语义扩展（既有不动，仅回归验证）。

## 2. 架构方案

三臂改造，全部在既有 Desktop Protocol v1 骨架内：

```text
┌─ 宿主臂（auto-lang/ui/session.rs + renderer.rs 装配）──────────┐
│ AppSpec 增 exe 发现（pac desktop_exe: > rust-workspace 约定路径）│
│ launch_app_outproc 分流：有 exe → spawn <exe> --autodesk-        │
│   incubate --app386=<name> --autodesk-broker=<pipe>（同参数面）  │
│ 认领/registry_id 回填/daemon ensure 链/stage3 合成：零改动复用   │
└──────────────────────────────────────────────────────────────────┘
┌─ 客户端臂（auto-lang/ui/desktop_protocol/）─────────────────────┐
│ client_for_component（新）：cmd_autodesk.rs 的 run_client_entry │
│   抽壳——三态裁决 + broker 孵化 + 主循环骨架不变，组件装载泛型化 │
│ AppProjector / run_independent_child：DynamicComponent →        │
│   Component seam（视图/文本/条件/状态/命中 五解析点抽象）        │
└──────────────────────────────────────────────────────────────────┘
┌─ 生成器臂（auto-man/src/rust_ui.rs）────────────────────────────┐
│ main 模板增孵化参数解析：--autodesk-incubate / -broker / -render │
│   → client_for_component（iced 臂内）；无标记 = run_app_devtools │
│   现行行为零变化（I1 零删除不变式）                              │
└──────────────────────────────────────────────────────────────────┘
```

**不变式**：

- **I1 零删除**：解释型三形态（inproc 缺省 / outproc-解释 / Standalone）与
  生成 exe 独立形态（直跑开自己的窗）行为零变化；既有测试全绿是回归门。
- **I2 追加式协议**：`PROTOCOL_VERSION` 维持 1；若 native 命中表需要新 wire
  变体，走追加 tag（v1.6 增量记录），禁止改义既有 tag。
- **I3 not-yet 纪律**：queue 臂未覆盖的 widget 在覆盖表显式 not-yet，
  auto 裁决降级 independent 并留观测痕——禁止静默错绘（500/507 既定）。

**关键风险**：native `View` 树与解释态 `AuraNode` 的表示/解析差异（T-01
深水调查定案）；pixels 臂子进程自带 iced/wgpu 的内存税（G4 度量 + queue 臂
为目标形态缓解）；tick/subscription 语义在 queue 臂的泵驱动（复用
`Component::tick_interval_ms` 配方，run_app_devtools 已有先例）。

## 3. 技术栈

Rust / iced 0.14（宿主与 pixels 臂子进程运行时）；既有 desktop_protocol
（tokio named pipe、`CreateFileMappingW` shm 双槽、APDL 二进制信封）；a2r
生成器（auto-man `rust_ui.rs`，`auto build/run -r rust` 驱动）；验收载体 =
`ui_desktop` 编译宿主（auto-os `scripts/desktop.sh` iced 臂）+ apps 仓
rust-workspace 产物（036-tetris 等参照 623 验证链）；度量 =
`K32GetProcessMemoryInfo`（480 先例，零新依赖）。

## 4. 需求分析与背景调查

**授权记录**：用户 2026-09-15 会话明确要求"规划出一个新的计划"（含实现
Rust 版 RenderQueue 路径的步骤拆解）——**本轮仅规划，未授权实施**。涉及
仓：auto-lang（框架/协议/生成器）+ auto-os（桌面程序台账/验收装配）。
无预算/自动续跑约束声明。

**现状事实**（已核）：

- 协议底座全量实现：`crates/auto-lang/src/ui/desktop_protocol/`（mod/
  message/transport/shm/broker/stage3/client_runtime/pixels/coverage/
  dual_mode），设计文档 `auto-lang/docs/design/autoui/desktop-protocol-v1.md`
  （v1.0–v1.5 增量史）。
- outproc 子进程 = `auto run --autodesk-incubate`（`session.rs:2368-2390`
  spawn_outproc_child；2570-2620 launch_app_outproc 认领链）——解释 .at，
  非编译产物。
- 客户端臂绑死解释态：`AppProjector{ component: DynamicComponent }`
  （client_runtime.rs:100-101）；`pixels::run_independent_child(component:
  DynamicComponent, …)`（pixels.rs:252-259）。
- a2r 产物 = 原生 Rust Component：`auto_lang::ui::{Component, View}`（trait
  定义 ui/component.rs:34；范例 apps/036-tetris/rust-workspace/…/main.rs：
  AppMsg 枚举 + 状态结构体 + `view() -> View<Self::Msg>`，FStr/条件/状态在
  Rust 侧已解析，无 VM）。
- 生成 main 只分派 `run_app_devtools::<C>()`（iced）或 GPUI（rust_ui.rs
  1780-1815 模板），`--autodesk-*` 零感知。
- 三态渲染裁决链已就位：`RenderMode::resolve`（spawn 参数 > pac
  `desktop_render:` > auto）+ `coverage::effective_frame_mode`（auto 探测
  降级）（cmd_autodesk.rs:75-94）。
- 508 裁定：解释型缺省 inproc；outproc（auto re-exec 实测 6.48 MiB/App、
  启动 25-250ms）为显式隔离选项——exe App 不受此裁定约束（天然 outproc），
  度量对齐口径见待澄清④。
- auto-os 台账：`docs/plans/autos-desktop-program.md`（程序行登记点）；
  `.autoos/specs.json` 六节 ledger（merge 期沉淀）。

**specs 现状**：auto-lang `docs/specs/auto-lang/ui/` 为 UI 模块 spec 源；
协议权威文档 = `docs/design/autoui/desktop-protocol-v1.md`（版本化增量式）。
本计划规范增量见 §5 规范增量表。

## 5. 详细设计

### 5.1 T-01 深水调查：native View 投影 seam（决策产物）

a2r 的 `View<Msg>` 与解释态 AuraNode 的差异面（节点种类、prop 值形态、
handler 载荷、条件块、状态插值）是泛化的唯一未知量。调查产出** seam 设计
定案**，回答：

- **D1' 命中→动作策略**（本计划核心决策）：
  - **策略 A（生成期静态表）**：rust_ui.rs 生成 `hit_actions()` 映射
    （widget 位 → Msg 构造），投影器消费与解释态同形的交互区表；
  - **策略 B（运行期 View 投影）**：泛型投影器直接遍历 native `view()`
    产物（prop 已解析、handler 已是 Msg 载荷），不依赖生成期信息。
  - 倾向 B（零生成器耦合、与解释态 `flatten_visible` 同构），A 作 B 不可
    行时的回退——以 tetris（store 嵌套 + tick）与 counter 级两样本定案。
- **D2'** 文本/条件/状态五解析点在 native 侧的对应面（直接值 vs 求值闭包）。
- **D3'** queue 臂 tick 配方（`Component::tick_interval_ms/tick_msg` 在
  ClientPump 的驱动位）与 pixels 臂 subscription 的既通路确认。

定案记录追加进本节（`### 5.1 定案记录`），复审时作为 T-03/T-04 的依据。

### 5.1 定案记录（T-01，2026-09-15）

- **D1' = 策略 B（运行期 View 投影），策略 A 否决**。证据：
  - `View<M>` 是 iced 无关类型 IR（`ui/view.rs:428`，~35 变体、typed 命名字段
    prop、无 HashMap），已有三消费端（iced `iced/renderer.rs:3052`、GPUI
    `gpui/renderer.rs:60`、headless/VTree `vnode_converter.rs:67`）——投影器
    即第四后端，架构先例充分。
  - 零参 handler 在 View 里已是**物化 `M` 值**（`Button.onclick: M`
    view.rs:452；builder 构建期急切求值闭包 view.rs:1409-1415）→ 泛型命中表
    `Vec<(WRect, C::Msg)>` 直接 clone 派发。先例：`extract_handler_from_view<M>`
    （renderer.rs:22002-22028）+ rust 模式 MCP 通道已按 view→path→handler→
    `component.on(m)` 派发（renderer.rs:21924-21931）。
  - 解释态同样产 View（`AuraViewBuilder::build(&AuraNode) -> View<DynamicMessage>`
    aura_view_builder.rs:375，DynamicComponent::view dynamic.rs:1154）——View
    投影器天然具备双轨统一潜力。
  - 策略 A 否决依据：静态表无法表达状态依赖视图（tetris `cells.iter().map`
    每格按钮 / `if save_pending` 条件按钮，main.rs:112 一表达式内并存），且
    重复运行期树已携带的信息。
  - **泛型而非对象安全**：`Component: Sized`（component.rs:34）→
    `NativeProjector<C: Component>` 单态化（每个 child exe 自带具体 App，
    无动态分发需求）。
- **D2' 五解析点 native 对应面**：视图清单 = 直接遍历 `view()` 产物（复用
  `extract_children_ref<M>` vnode_converter.rs:581、`find_view_by_path_generic`
  renderer.rs:21981、`VNode.path` 稳定身份 vnode.rs:310）——解释态所需的
  flatten_visible/eval_condition/interpolate（client_runtime.rs:717/732/1807）
  在 View 已天然消解（条件/循环/插值在 view() 构建期求值完毕）；文本/prop =
  typed 字段直读；状态读 = `Component::state_snapshot()`（component.rs:87，
  生成器已覆写，tetris main.rs:383-409——顺带对齐 L3 StateSnapshot 控制面）；
  样式 = View 持已解析 `Option<Style>`（StyleClass typed 枚举 style/mod.rs:246）
  vs 现投影器 NodeStyle 吃字符串（client_runtime.rs:223）→ 需
  StyleClass→BoxLayout/颜色小适配器；命中派发 = 泛型表 + `component.on(msg)`，
  **输入载荷侧信道**：生成 on() 经 `last_input_text()` 读 typed 文本
  （renderer.rs:1065-1069），派发前须设同一 thread-local（MCP 先例
  renderer.rs:21926）。
- **D3' tick/键**：trait 方法全后端无关（tick_interval_ms/tick_msg/key_message/
  key_bindings，component.rs:41-72）；ClientPump 现无 tick 源（client_runtime.rs:
  2044/2171）→ native 泵自带 timer 循环（devtools_update 派发同型
  renderer.rs:21881-21886）。
- **v1 边界（入覆盖表 not-yet）**：payload handler 族（Slider `fn(f32)->M`、
  Select/PointerMove/Scroll `Arc<dyn Fn>` 新类型 view.rs:46-418）不入命中表；
  `EventRouter`/`view_to_vtree_with_events` 不可复用（DynamicMessage 专属
  unsafe transmute，vnode_converter.rs:1383-1397）。
- **架构裁定（附加）**：v1 双投影器并存——`AppProjector`（解释态，零改动，
  I1）+ `NativeProjector<C>`（新，View 泛型）；解释态投影器改写为 View 基
  （双轨统一）留后续债，不在本计划。

### 5.2 客户端入口泛型化（T-02）

`desktop_protocol` 新增 `client_for_component`（名字执行期可调）：把
cmd_autodesk.rs 的 `run_client_entry` 中"装载 .at → DynamicComponent"之外
的骨架（参数解析、三态裁决、broker 孵化/直连、Commands/Pixels 分派）抽为
接收已构造组件的入口；cmd_autodesk.rs 改薄壳。组件参数以 T-01 定案的 seam
类型（解释态 DynamicComponent 实现同一 seam，编译态 Component 经适配）。
行为零变化由既有测试守护。

### 5.3 pixels 臂泛化（T-03）

`run_independent_child` 的组件参数泛型化（或同型 twin 函数）：子进程隐藏
窗 iced 运行时驱动 native Component 的 update/view 循环，输入经协议通道
转交（reader 线程 → 主线程，Rc 不跨线程纪律同既型）。

### 5.4 queue 投影臂泛化（T-04）

AppProjector 的 DynamicComponent 五解析点（视图清单、文本插值、条件求值、
状态读、命中派发）换 seam 实现；覆盖表对 native 视图清单的静态扫描同型
（auto 裁决）。首次交付覆盖集 = counter 级（text/button/input/线性堆叠
族，Stage 5 覆盖表既有集合的 native 等价），其余显式 not-yet。

**实现设计钉（2026-09-15 work 中段，代码未落）**：
- **接缝已核实**：`AppEndpoint<S: FrameSource>`（endpoint.rs:82-95，
  trait 四方法 revision/render_frame/on_input/on_control，对象安全形态）
  ——`ClientPump` 字段钉死 `AppProjector`，需泛型化为 `ClientPump<S:
  FrameSource>`（解释态单态化零行为差；`run_client` 公签名不动，加
  native 包装）。tick 源：FrameSource 追加缺省空方法 `poll_tick(&mut
  self)`，泵循环每轮调用（NativeProjector 实现为 interval 到期 →
  `component.on(tick_msg)` + revision 递增）。
- **投影器**：新文件 `desktop_protocol/native_projector.rs`
  `NativeProjector<C: Component>`——render_frame 每帧调 `component.view()`
  走 View 树（无模板缓存语义，条件/循环已求值）；**布局走平行walker
  `layout_view_block`**（镜像 client_runtime `layout_block` 块流语义，
  节点面换 View 枚举——I1 零改动解释臂，重复 ~150 行记债归"双轨统一"
  后续）；样式：View 持 typed `Option<Style>`（StyleClass 枚举）vs
  NodeStyle 吃字符串——写 StyleClass→BoxLayout 小适配器（不复刻字符串
  解析）。命中表 `Vec<(WRect, C::Msg)>`（HitKind 泛型化或平行枚举）。
- **v1 覆盖集**：text/button + col/row + 布局子集（padding/gap/margin/
  尺寸/圆角底/前景色）；input/slider/select 等 payload 族 not-yet——
  **native 显式 queue 遇未覆盖 = 拒绝退出留痕**（非静默错绘，AC-04）；
  native auto 缺省 = independent（待澄清③推荐落地）。
- **client_entry native 臂**：`run_native_client<C>(component, opts,
  target)`——Commands → NativeProjector + 泛型泵；Pixels → T-03 入口
  （T-05 生成 main 消费）。

### 5.5 生成器客户端臂（T-05）

rust_ui.rs main 模板：iced 臂入口前解析 `std::env::args()`，见
`--autodesk-incubate` / `--autodesk-client=` → `client_for_component`
（组件 = 生成的 `{main_widget}::default()`）；无标记 = `run_app_devtools`
零变化。`--autodesk-render` 透传。GPUI 臂不接（cfg 下孵化参数时报错退出
并留痕）。

### 5.6 宿主注册表与 spawn 分流（T-06）

AppSpec 增 `exe: Option<PathBuf>`：发现序 = pac `desktop_exe:` 字段 >
`<app-root>/rust-workspace/<id>/target/(release|debug)/<id>.exe` 约定。
`launch_app_outproc` 分流：有 exe → spawn exe（参数面同现行
`--autodesk-incubate --app386=<目录名> --autodesk-broker=<pipe>`，不再注入
`AUTO_386_APP_ROOT` 解释根）；无 exe → 现行 auto re-exec 臂零变化。认领/
registry_id 回填、`ensure_daemon_if_declared`、broker serve/attach 泵全部
复用。exe 缺失（声明了但未构建）→ toast 错误语义与 inproc 臂一致。
落位 (16,16)/480×320 已知差异本轮不新做（见待澄清⑤）。

### 5.7 e2e 验收与度量（T-07/T-08）

验收载体：auto-os `scripts/desktop.sh` iced 臂 + 编译产物 App。样本 =
counter 级（queue 臂全链）+ 036-tetris（auto 裁决降级 + 复杂视图）。
度量沿 480/508 方法（K32GetProcessMemoryInfo，N=1/3/5 边际），报告落
auto-lang `docs/plans/reports/020-rust-exe-compositor-metrics.md`。

### 规范增量

| delta_id | add/modify/retire | docs/specs/... target | before/after rule | rationale | acceptance IDs |
|---|---|---|---|---|---|
| SD-01 | add | auto-lang/docs/design/autoui/desktop-protocol-v1.md（§1.6 v1.6 增量） | before：客户端臂仅解释态 DynamicComponent（经 auto re-exec 装载）；after：协议客户端入口泛型化至 Component seam，a2r 编译 exe 为一等客户端，命中表 native 策略与 auto 裁决缺省入册（PROTOCOL_VERSION 仍 1，追加式） | 协议权威文档版本化收录 exe-client 形态 | AC-02/03/04 |
| SD-02 | modify | auto-os/docs/plans/autos-desktop-program.md | before：RenderQueue 线终态 = "outproc 为隔离/远程形态的 App 载体选项"（解释态）；after：登记 PLAN-020 行——编译 exe App 为 compositor 一等客户端，宿主孵化分流 exe 臂 | 桌面程序台账收录新形态 | AC-02 |
| SD-03 | modify | auto-lang/docs/specs/auto-lang/ui/（具体文件 review 期按目录实况定） | before：UI 模块 spec 无编译 exe 客户端面；after：补 desktop_protocol 客户端 seam 的模块 spec 条目（provisional） | 模块 spec 对齐实现 | AC-02/03/04 |

零 spec 影响的变更不存在（协议/桌面形态为 spec 级知识）；ledger
（auto-lang `.autoos/specs.json` designs/tests 节）随 merge 沉淀。

## 6. 测试设计

- **单测（auto-lang）**：seam 泛化后全消息 round-trip / golden bytes 零漂移；
  native 投影 golden（counter 级 View → DrawList 期望帧）；覆盖表 not-yet
  判定；AppSpec exe 发现序（pac 字段 > 约定路径 > None）；spawn 参数面。
- **集成（两进程）**：dual_mode 同型扩展——re-exec **编译产物 exe**（非
  auto）：孵化 → Active → 帧递增（Commands 与 Pixels 各一型）→ 协议点击
  → Close → 退出码。
- **e2e（auto-os）**：desktop.sh iced 宿主 + 编译 App 冒烟（虚拟窗出现、
  点击闭环、Close 回收、kill 子进程回收）；生成 exe 脱离桌面直跑 = 独立窗
  （AC-01 回归）。
- **parity**：counter 级三臂金样（独立窗直挂 / queue 帧重放 / pixels 帧重放）
  同源对拍，扩入既有 I4' 纪律。
- **度量**：N=1/3/5 内存边际 + 启动时延，双口径（Private/WS），对照
  508 报告数字行。

## 7. 验收标准

- **AC-01 独立形态零回归**：`auto build -r rust` 产物 exe 无孵化参数直跑 =
  现行独立窗行为。验证：重生成 036-tetris + counter 级样本直跑冒烟；
  既有 rust 轨测试全绿（`cargo t -p auto-man rust_ui` 相关 + 623 遗留套件）。
- **AC-02 宿主孵化编译 exe**：ui_desktop 宿主 launch 声明了 exe 的 App →
  子进程为该 exe（进程名/路径断言，非 auto.exe），虚拟窗渲染出内容，点击
  → 状态变化 → 新帧（counter: 0→1）。验证：e2e 冒烟脚本 +
  `docs/plans/reports/assets/020/` 截图留痕。
- **AC-03 queue 臂全链（RenderQueue）**：counter 级 App 以 DrawList 命令帧
  渲染进虚拟窗，子进程无 GPU 上下文（无 iced window 建立，以观测通道
  Log/孵化记录帧模式断言）。验证：集成测试 + e2e。
- **AC-04 auto 裁决与 not-yet 纪律**：tetris（超覆盖）auto 模式降级
  independent 像素臂，孵化记录带 `pixels:auto` 留痕；显式 `--autodesk-render=
  queue` 对未覆盖项不静默错绘（覆盖表 not-yet → 拒绝或显式降级，行为与
  解释态一致）。验证：裁决单测 + e2e 观测断言。
- **AC-05 生命周期回收**：宿主 Close → 子进程退出（退出码 0）；kill 子
  进程 → 宿主回收虚拟窗（EOF 路径）。验证：e2e 两向用例。
- **AC-06 度量报告**：`docs/plans/reports/020-rust-exe-compositor-metrics.md`
  落盘，含 queue/pixels 两臂 × N=1/3/5 内存边际 + 启动时延 + 与 508 基线
  对照结论句。验证：报告存在且数字可复现（脚本入 repo）。
- **AC-07 既有零回归**：`cargo t -p auto-lang desktop_protocol --features
  ui-iced`、session/stage3/dual_mode 相关、auto-os 桌面 smoke
  （desktop_mcp 链）全绿。

## 8. 执行步骤

依赖序：T-01 → T-02 → {T-03, T-04 并行} → T-05 → T-06 → T-07 → T-08 → T-09。
auto-lang 侧工作在 lang worktree（`D:/autostack/.wt/lang-020/auto-lang`，
623 同型）；auto-os 侧在 os 组 worktree（`D:/autostack/.wt/os-020/auto-os`）。

- **T-01 [lang] 深水调查与 seam 定案**
  文件：`crates/auto-lang/src/ui/component.rs`、`ui/dynamic.rs`、
  `desktop_protocol/client_runtime.rs`（读面）+ 本计划 §5.1（写面）。
  动作：对 counter 级与 tetris 生成代码做 View/AuraNode 差异扫描，定案
  D1'/D2'/D3'。产物：`### 5.1 定案记录`。
  验证：定案记录含两样本证据引用；复审通过。
  [✅ 已完成 2026-09-15] 定案 = 策略 B（§5.1 定案记录）；两样本证据 =
  counter 级 builder 物化 + tetris main.rs:112 状态依赖视图；`View<M>`
  三消费端/`extract_handler_from_view<M>` 先例/五解析点对应面/边界均落
  file:line 证据。代码零改动（纯调查），无 commit。
  → AC-03/04。新路径：是（调查产物）。
- **T-02 [lang] 客户端入口泛型化**
  文件：`crates/auto-lang/src/ui/desktop_protocol/`（新入口 + mod 导出）、
  `crates/auto/src/cmd_autodesk.rs`（薄壳化）。
  动作：按 §5.2 抽壳；解释态行为零变化。
  验证：`cargo t -p auto-lang --features ui-iced desktop_protocol` 全绿 +
  `cargo t -p auto`（autodesk 入口相关）。
  [✅ 已完成 2026-09-15] worktree `D:/autostack/.wt/lang-020/auto-lang`
  （plan-020-dev，基线 auto-lang fcf4b1092 + 组内 auto-down 140775f 兄弟
  worktree）。代码：新增 `desktop_protocol/client_entry.rs`
  （ClientOpts/ClientTarget/connect/run_dynamic_client），mod.rs 注册，
  `crates/auto/src/cmd_autodesk.rs` 薄壳化（装载+三态裁决留壳）。门：
  ①`cargo check -p auto` exit=0；②`cargo t -p auto-lang --features
  ui-iced desktop_protocol --no-fail-fast` = 120 跑 117 绿 / 3 败，逐条
  归因（均非 020 代码回归）：`coverage::covered_elements_within_target_set`
  = **主检出既有红**（stash 基线同败复证：element 表登记 imagesurface
  covered / 投影器能力表缺失）；`p508_g2_outproc_arm` 与
  `t3_independent_pixels_and_dual_mode` = **环境交互**——e2e_exe 硬编码
  `<worktree>/target` 寻址 × 验证时 CARGO_TARGET_DIR 组目录重定向 →
  构建落点/寻址点分裂（os error 3），无重定向复跑收口（见 §9）。
  → AC-07。
- **T-03 [lang] pixels 臂泛化**
  文件：`desktop_protocol/pixels.rs`（+ client_runtime 需要处）。
  动作：按 §5.3 组件参数泛型化；双态分派接 T-02 入口。
  验证：既有 pixels 测试全绿 + 新增 native Component 像素臂两进程集成
  （exe 形态，AC-05 前置）。
  [✅ 已完成 2026-09-15] 同 worktree。代码：`pixels.rs` 增
  `run_independent_native_child<C: Component>`（桥/launch slot/PIXELS_POLL
  装配与解释臂同源）+ `poll_transport` pub(crate)；`iced/renderer.rs` 增
  NativePixelsMsg/NativePixelsHost（from_launch 发 Hello）/update
  （Proto/Shot 两臂镜像 session 解释态同名臂；App/Tick 驱动
  `inner.on` 后截图）/view（View→iced，`Element<'_>` 生命周期跟输入——
  iced 0.14 ViewFn HRTB）/协议轮询 + tick 两个泛型 subscription recipe/
  `run_native_iced_pixels`（隐藏单窗 `visible:false`，`window::latest()`
  取窗 id——0.14 无 Id::MAIN，`.run().map_err(into)`）；`iced/mod.rs`
  再导出；`examples/native_pixels_counter.rs` + Cargo.toml `[[example]]`
  （required-features = ui-iced）。**范式发现（T-05/T-07 依赖）**：winit
  Windows 拒绝非主线程 EventLoop，libtest 恒在工作线程跑测试 → 窗口化
  像素臂 e2e 载体必须是生产形态二进制（stage3 t3 用真 `auto run` 同理；
  本轮以 example 二进制最小同构落地；测试二进制 re-exec 仅适用 queue 臂）。
  门：`AUTO_DESKTOP_E2E=1 cargo t ... native_pixels_child_two_process`
  **PASS 25.6s**——真隐藏窗 + 真截图：Hello → Welcome(Pixels)+BufferAlloc
  → FrameReadyPixels 首帧（shm 槽满幅非零、64×32 逻辑尺寸对齐）→
  Input 触发第二帧（frame_id 递增；v1.3 输入边界对齐：无 handler 派发）
  → Close → ExitRequest → 宿主 BufferRelease → Detached → 子进程干净
  退出（**宿主必须走完回收步**——Close 不自足）。边界（§5.1 同册）：
  L3 StateSnapshot 注入传 None（native not-yet）。
  → AC-02/04/05。
- **T-04 [lang] queue 投影臂泛化**
  文件：`desktop_protocol/client_runtime.rs`（AppProjector seam）、
  `desktop_protocol/coverage.rs`（native 视图清单扫描）。
  动作：按 §5.4 + T-01 定案；counter 级覆盖集 + not-yet 表。
  验证：投影 golden 单测 + 覆盖判定单测。
  [✅ 已完成 2026-09-15] 同 worktree，commit `98a4cd502`。代码：
  ①新文件 `native_projector.rs`——`NativeProjector<C: Component>`
  （策略 B：render_frame 每帧调 `component.view()` 走 View 树；平行
  walker `layout_view_block/layout_view_node` 镜像 `layout_block` 块流
  语义含两遍法居中；typed 样式适配器 `node_style_of`（盒模复用
  `BoxLayout::from_style`，装饰/对齐/字号直填 `NodeStyle`——
  `NodeStyle`/共享 helpers/consts 已 `pub(crate)` 化零复刻）；命中表
  `Vec<(WRect, C::Msg)>` 物化消息 clone 派发；禁用态压暗无命中区；
  覆盖门 `ensure_covered()` + 门后动态分支 `uncovered_seen` 占位留痕；
  `poll_tick` interval→`component.on(tick_msg)`+rev 前进）。
  ②`endpoint.rs` FrameSource 追加缺省空方法 `poll_tick`（解释态零变化）；
  `ClientPump` 泛型化 `ClientPump<S: FrameSource = AppProjector>`（缺省
  类型参数保全部既有调用点；空拍臂 + `poll_session_tick` revision 对账
  产帧；`run_client` 公签名不动，新增泛型 `run_client_session`）。
  ③`coverage.rs` 增 `Coverage::native_queue_set()`（v1 = text/button +
  col/row/container/list + 布局样式子集；payload/display 族 not-yet）+
  `scan_native_view`（View<M> 树 → ViewScan，handler 物化故
  param_handlers 恒空）+ `native_style_token`（StyleClass→代表 token，
  judge 复用）+ `native_kind_of`（34 变体→归一 kind）。
  ④`client_entry.rs` 增 `resolve_native_frame_mode`（**待澄清③落地：native
  auto 缺省 independent + 降级观测行**；显式 queue 不在此裁决）+
  `run_native_client<C>`（Commands→覆盖门拒绝留痕+泛型泵；Pixels→T-03
  入口）。
  门：①`cargo check -p auto-lang/-p auto --features ui-iced` exit=0；
  ②新增 8 测全绿——golden 帧形（typed 链路 text-slate-200/text-lg 断言）/
  命中派发+盒外不派发/禁用压暗无命中/覆盖门 payload 族拒绝（slider 缺项）/
  不支持样式缺项（shadow）/门后动态分支占位留痕/poll_tick 相位对齐+到期
  派发/**真命名管道全循环**（native 帧入宿主 462 会话合成 → 协议点击 →
  count:0→1 → L2Detach 出口 revision 连续）；③回归：desktop_protocol
  套件 129 跑 128 绿（唯一败 = 在册既有红
  `coverage::covered_elements_within_target_set`，主检出 stash 基线同败
  ——blocker ① 维持）；client_runtime 38/38、stage3 12/12、endpoint
  9/9 全绿（ClientPump 泛型化解释态零回归实证）。
  → AC-03/04。
- **T-05 [lang] 生成器客户端臂**
  文件：`crates/auto-man/src/rust_ui.rs`（main 模板 1780-1815 区域）。
  动作：按 §5.5 加参数解析与分派；重生成样本验证。
  验证：counter 样本重生成 diff 仅含客户端臂；直跑回归（AC-01）；
  `cargo t -p auto-man` rust_ui 套件。
  [✅ 已完成 2026-09-15] 同 worktree，commit `d2991337d`。代码：
  `wrap_example` main 模板 iced 臂注入 `native_client_gate`（参数解析
  `--autodesk-incubate` / `--autodesk-client=` 直连 / `--autodesk-broker=`
  / `--app386=` / `--autodesk-render=` 透传 → `RenderMode::resolve`（spawn
  参数 > auto；pac 档由宿主 spawn 侧透传，生成物无 pac 位置感知）→
  `resolve_native_frame_mode` → `run_native_client(main_widget::default(),
  …)`；无标记 = `run_app_devtools` 零变化（I1）；GPUI 臂孵化参数在册 =
  报错退出留痕（v1 限 iced）。边界随注：async-init App 经孵化臂以
  default() 态起。验证：①`cargo t -p auto-man rust_ui` 22/22 绿；②仓外
  scratch counter（`.wt/lang-020/scratch020/002-counter`，项目本地
  rust-workspace）重生成 + 编译 2m56s 过——生成 main 含 gate 且真编译；
  ③孵化冒烟：`--autodesk-incubate` 无 broker → **native auto→independent
  降级观测行打印 + broker 失败优雅 Error 退出**（门/臂端到端实证）；
  ④直跑冒烟：无标记 exe 独立窗存活（"Running with Iced backend"，6s 存活
  后 kill 回收）= AC-01 独立形态。**惯例发现**：`test_gen_015_notes_rust`
  会再生成仓内 `examples/rust-workspace/015-notes`（模板变更触发）——
  9cac4fd96 在案该生成物不入库，已 `git checkout --` 还原，提交只含
  rust_ui.rs。
  → AC-01/02。
- **T-06 [lang] 宿主注册表与 spawn 分流**
  文件：`crates/auto-lang/src/ui/session.rs`（AppSpec/launch_app_outproc/
  spawn_outproc_child）、`crates/auto-man/src/pac.rs`（`desktop_exe:` 解析，
  若采 pac 字段——按待澄清②定案）、renderer.rs 装配处（exe 发现）。
  动作：按 §5.6；解释态臂零变化。
  验证：发现序/spawn 参数单测 + 既有 session/stage3 测试全绿。
  [✅ 已完成 2026-09-15] 同 worktree，commit `ffd2ff9bb`。代码（待澄清②
  按推荐落地 = pac 字段为主 + 约定兜底）：①`AppRegistryEntry.desktop_exe:
  Option<String>`（pac `desktop_exe:` 原值入册）+ `LaunchSpec.exe:
  Option<PathBuf>`（boot resolver 相对 App 根解析，缺席 None；23 处既有
  构造点补 `exe: None` 零行为差）；②`outproc_native_exe` 发现序——声明
  即信（缺失在 spawn 臂报错转 toast，不静默回退解释臂）> rust-workspace
  约定路径 `<root>/rust-workspace/<dir>/target/{release,debug}/<exe>.exe`
  （release 先 debug；exe 名 pac name 蛇形先、目录名兜底——scratch counter
  实测生成物 = 蛇形包名）；③`spawn_exe_child`（native 臂：无 `run` 子
  命令、不注入 `AUTO_386_APP_ROOT`，`--app386=<dir>` 在 native 侧为 Hello
  app_name 覆盖——宿主认领按目录名匹配同源；NEXTEST_* 剥除同款）；
  ④`launch_app_outproc` 三级分流：注入 spawner（测试机件不动）> native
  exe 臂 > 现行 auto re-exec 臂（零变化，I1）。
  门：①`native_exe_discovery_order`（声明优先/release>debug/蛇形>目录名/
  全缺 None/内联 spec 无发现面五断言）+ `scan_picks_up_desktop_exe_
  declaration`（注册表入册）绿；②回归：session 73/73、stage3 12/12、
  app_registry 24/24、desktop_protocol 129 跑 128（唯一败 = 在册既有红）
  全绿——解释态臂零回归实证。
  → AC-02。
- **T-07 [os+lang] e2e 验收**
  文件：auto-os `scripts/desktop.sh`（如需旗标）+ 新增冒烟脚本（对齐
  `apps/028-launcher/tests` 先例）；截图留痕目录
  `docs/plans/reports/assets/020/`（lang 侧）。
  动作：AC-01..05 逐条跑通留痕。
  验证：见各 AC 验证句。
  [✅ 已完成 2026-09-15] lang 侧 commit `6780307f7` + 路由补丁 +
  真机证 `b6f7238a5`；os 侧 `scripts/smoke-020-native-exe.sh`（os worktree
  plan-020-dev）。真机链（ui_desktop `--apps-dir` 载体注册表）：
  ①DesktopBus `launch	002-counter`（真消费臂）→ 宿主**孵化 counter.exe
  （编译产物进程实证，非 auto.exe）**——含 T-07 补路由（inproc 缺省下
  "exe App 天然 outproc"，G1/非目标节裁定，session 73/73 回归绿）；
  ②queue 档虚拟窗渲染 native View 投影帧（"Counter: 0" + 三按钮，
  `020-native-launch-queue.png`）；③auto 档（去声明重启）宿主 broker 链
  打印 `[render] 002-counter: auto -> independent (coverage downgrade)` +
  pixels 帧入合成器（`020-native-auto-pixels.png` + 宿主日志
  `020-desktop-host-log.txt`）——**AC-04 tetris 样本以 counter-auto 模式
  等价替代**（降级机制 app 无关；tetris 冷构建 ~10min 不增机制覆盖，
  调整随注）；④协议级点击闭环 0→1 + kill EOF 回收 + Close 退出码 0 =
  p020_native_exe_arm（真 broker/endpoint 栈）。**已知缺口（P020-D4
  入债）**：窗内点击/× 关闭的 OS 级自动化未打通（DPI 2x + 画布缩放变换
  未文档化，acceptance channel 无 pointer verb）——GUI 级点击/回收由
  协议级证据承载；smoke 脚本随注。
  → AC-01✅ AC-02✅ AC-03✅ AC-04✅（等价样本）AC-05✅（协议级）。
- **T-08 [lang] 度量报告**
  文件：`docs/plans/reports/020-rust-exe-compositor-metrics.md` + 度量脚本
  （480 先例同型，入 repo）。
  动作：§5.7 度量；结论句对照 508。
  → AC-06。
  [✅ 已完成 2026-09-15] commit（metrics 臂 + 报告同提）。代码：
  `p020_metrics_native_arm`（N=1/3/5 阶梯 + K32 双口径 + attach/首帧
  时延 + 点击交互 stats，`AUTO020-METRICS-*` 输出行）。实测：**queue 臂
  边际 ≈2.42 MiB/App**（Private，N=1→5 线性）vs 508 解释 outproc 6.48
  （≈2.7×，距 inproc 0.86 的差值 = 进程/协议固有税）；attach 24.6–37.5ms、
  首帧 25.3–39.6ms（508 口径 25–250ms 贴下界）；点击往返 median 1.501ms
  / p95 1.559ms。报告含复现命令与口径注记（同 exe 五实例 vs 508 五不同
  App 差异随注）。
  → AC-06 ✅。
- **T-09 [lang+os] 文档与台账收口**
  文件：auto-lang `docs/design/autoui/desktop-protocol-v1.md`（§1.6）、
  `docs/plans/KNOWN-DEBT-AND-RISKS.md`（如产生新边界）、auto-os
  `docs/plans/autos-desktop-program.md`（SD-02 行）+ 两仓互链。
  动作：SD-01..03 落笔。
  → AC 全体的可追溯性。
  [✅ 已完成 2026-09-15] SD-01 = §1.6 v1.6 增量（ea91243f1）；SD-03 =
  overview.md provisional 指针（7d6ba9cb3）；P020-D1..D4 入 KNOWN-DEBT；
  SD-02 = 台账 3a 行已在 os worktree（plan-020-dev）落笔——canonical
  发布随 merge。两仓互链：计划 affects/程序行/冒烟脚本互指已落。
## 9. 复审记录

- 2026-09-15 /auto-plan:new 起草交接：`stage: new`，PLAN-020 rev 1。
  `outcome: pass`（合同完整，任务覆盖全部 AC 与规范增量，路径/命令已对
  仓核验）；`next: work`（T-01 起步——深水定案先行，无需用户解锁）。
  悬置决策已在 §10 登记（①②③④⑤），均不阻塞 T-01/T-02 开工。
- 2026-09-15 /auto-plan:work 中段记录（T-01..T-03 完成，T-04 未启）：
  `stage: work | PLAN-020 | rev 1 | outcome: executing（未到交接门）|
  code_commit: auto-lang plan-020-dev 876782838(T-02) → c6c7988f5(T-03)
  （基线 fcf4b1092；组 = .wt/lang-020/{auto-lang, auto-down@140775f}）|
  task_ids: T-01✓ T-02✓ T-03✓ | evidence: §5.1 定案记录（策略 B）+
  各任务 [✅] 行；native 像素臂 e2e PASS 25.6s | blockers: ①主检出既有红
  coverage::covered_elements_within_target_set（stash 基线同败——
  element 表 imagesurface 登记/能力表脱钩，非 020 引入，**建议上报
  plan624/在册线处理**）；②p508_g2/t3_independent 环境交互（e2e_exe
  硬编码 worktree 默认 target × CARGO_TARGET_DIR 重定向；无重定向复跑
  收口中——后续验证一律不重定向 target 或先修 e2e_exe 尊重 env）|
  next: T-04（NativeProjector<C> over View，§5.1 定案 + 既有
  AppProjector 块流布局复用）。
  补（同日晚）：②已收口——`p508_g2_outproc_arm` 无重定向复跑 **PASS
  132.6s**（真 auto.exe outproc 链，cmd_autodesk 薄壳化零回归实证）；
  套件在册唯一失败 = 主检出既有红 coverage（blocker ① 维持，转用户
  上报）。工作面重定向经验入册：lang-020 组验证一律用 worktree 默认
  target（e2e_exe/stage3 寻址前提）。
- 2026-09-15 /auto-plan:work 中段记录（T-04 完成，T-05 未启）：
  `stage: work | PLAN-020 | rev 1 | outcome: executing（未到交接门）|
  code_commit: auto-lang plan-020-dev 98a4cd502(T-04；前序 c6c7988f5=T-03)
  | task_ids: T-04✓ | evidence: §8 T-04 [✅] 行（8 新测含真管道全循环
  PASS；client_runtime 38/stage3 12/endpoint 9 零回归；desktop_protocol
  129 跑 128 绿）| blockers: ①（维持——既有红 coverage imagesurface，
  非 020 面）| next: T-05（rust_ui.rs main 模板孵化参数解析 →
  client_entry::run_native_client 分派；重生成 counter 样本 diff 仅客户端臂
  + 直跑回归）。
- 2026-09-15 /auto-plan:work 中段记录（T-05 完成，T-06 未启）：
  `stage: work | PLAN-020 | rev 1 | outcome: executing（未到交接门）|
  code_commit: auto-lang plan-020-dev d2991337d(T-05；前序 98a4cd502=T-04)
  | task_ids: T-05✓ | evidence: §8 T-05 [✅] 行（scratch counter 生成编译过
  + 孵化/直跑双冒烟；rust_ui 22/22）| blockers: ①（维持）| next: T-06
  （session.rs AppSpec `exe:` 发现序=pac `desktop_exe:` > rust-workspace
  约定——待澄清②按推荐落地；launch_app_outproc spawn 分流；scratch
  counter exe 为 T-07 e2e 载体留存 `.wt/lang-020/scratch020`）。
- 2026-09-15 /auto-plan:work 收尾记录（T-01..T-06 + T-08 全闭环，
  T-07/T-09 各余 os 侧一腿）：
  `stage: work | PLAN-020 | rev 1 | outcome: executing（T-07 os 侧 GUI
  留痕腿 + T-09 SD-02 canonical 行未落，未到交接门）|
  code_commit: auto-lang plan-020-dev 98a4cd502(T-04) → d2991337d(T-05)
  → ffd2ff9bb(T-06) → 6780307f7(T-07 lang 侧) → T-08(metrics) → T-09
  lang 侧(docs) 共 6 提交（基线 fcf4b1092）| task_ids: T-04✓ T-05✓
  T-06✓ T-08✓ T-09◐(SD-02 归 merge) T-07◐(os 侧未启) | evidence:
  §8 各 [✅]/[◐] 行；关键实证 = p020_native_exe_arm（生产 spawn 链孵化
  编译 exe → queue 帧 19 ops → 点击 0→1 → kill/Close 双向回收 exit=0）
  + 度量 2.42 MiB/App + v1.6 入册 + KNOWN-DEBT P020-D1..D3 | blockers:
  ①（维持，既有红 coverage）②盘空间（D 盘 681G 用满清出 11G，lang-020
  target 增量缓存为祸源——建议计划外清理）| next: work 续 T-07 os 侧
  （建 `.wt/os-020/auto-os` 组 worktree → desktop.sh iced 真机冒烟 +
  截图留痕 `docs/plans/reports/assets/020/` + tetris 降级载体），随后
  execution_done → review（SD-02 行文案已备，merge 期落 autos-desktop-
  program.md）。
- 2026-09-15 /auto-plan:work 中段记录（T-06 完成，T-07 未启）：
  `stage: work | PLAN-020 | rev 1 | outcome: executing（未到交接门）|
  code_commit: auto-lang plan-020-dev ffd2ff9bb(T-06；前序 d2991337d=T-05)
  | task_ids: T-06✓ | evidence: §8 T-06 [✅] 行（发现序五断言 + 注册表
  入册单测；session 73/stage3 12/app_registry 24/desktop_protocol
  129 跑 128 回归绿）| blockers: ①（维持）| next: T-07（os 组 worktree
  建 `.wt/os-020/auto-os`；desktop.sh iced 宿主 + counter exe 冒烟：
  虚拟窗/点击闭环/双向回收/截图留痕 `docs/plans/reports/assets/020/`）。

- 2026-09-15 /auto-plan:work 收口记录：
  `stage: work | PLAN-020 | rev 1 | outcome: pass | code_commit:
  auto-lang plan-020-dev 98a4cd502(T-04) → d2991337d(T-05) → ffd2ff9bb(T-06)
  → 6780307f7(T-07 lang) → T-08(82e9205fd) → T-09 lang(7d6ba9cb3) → 路由
  补丁+D4 → 真机留痕 b6f7238a5（基线 fcf4b1092）；auto-os plan-020-dev
  （worktree .wt/os-020/auto-os）冒烟脚本+SD-02 行 1 提交 |
  task_ids: T-01..T-09 全闭环（T-04✓ T-05✓ T-06✓ T-07✓ T-08✓ T-09✓；
  current_step 9/9）| evidence: §8 各 [✅] 行——协议级
  p020_native_exe_arm（孵化/queue 帧/点击 0→1/kill+Close 双向回收 exit 0）
  + 真机桌面链（bus launch → counter.exe 孵化 → queue 帧渲染截图 +
  auto→independent 降级行 + pixels 入合成器，assets/020/）+ 度量
  2.42 MiB/App + v1.6 入册 + KNOWN-DEBT P020-D1..D4 + SD-01..03 |
  blockers: 无阻断面（在册既有红 coverage=plan624 线；P020-D4 GUI 自动化
  缺口为债非 AC 阻断——AC-02/05 点击/回收由协议级证据承载）| next:
  review（auto-plan-review；全量套件门随 review 跑）。

- 2026-09-15 /auto-plan:review（同会话复审——独立性受限已声明，裁决由
  工件重建，不采信执行摘要）：
  `stage: review | PLAN-020 | rev 1 | outcome: pass |
  reviewed_commit: auto-lang plan-020-dev b187ef7d0（+复审提交：SD-01
  补正）| base_commit: fcf4b1092 | dependency_revisions: auto-down
  140775f（组兄弟 worktree）；载体 scratch020（regen 产物不入库）|
  spec_inputs: desktop-protocol-v1.md §1.6（ea91243f1+复审补正）/
  ui/overview.md 指针/KNOWN-DEBT P020-D1..D4；wire（message.rs）基线零
  diff 实证，PROTOCOL_VERSION 仍 1 |
  acceptance_results: AC-01✅（counter+**tetris 重生成直跑独立窗存活
  6s**——复审补证；rust_ui 22/22）AC-02✅（真机 bus launch→counter.exe
  进程孵化→虚拟窗渲染 queue 帧，截图 assets/020/；点击闭环协议级
  p020_native_exe_arm 0→1）AC-03✅（19 ops 命令帧真管道+真机桌面双证）
  AC-04✅（覆盖门拒绝单测+真机降级观测行+**tetris 孵化探针降级行复审
  补证**+pixels 帧入合成器）AC-05✅（kill→EOF 回收+Close→exit 0 协议级
  双向；GUI 级缺口=P020-D4 债）AC-06✅（报告复审重跑：N1 2.41→N5
  12.24MiB，边际≈2.46 与报告 2.42 同噪；点击 median 1.505ms）AC-07✅
  （desktop_protocol 131 跑 130+在册红；session 73/73；stage3 14/14；
  dual_mode 3/3；app_registry 24/24；auto-man rust_ui 22/22；auto bins
  编译过；**os 025 desktop_mcp 链 13/13**）|
  findings: R-01 info=在册既有红 covered_elements_within_target_set
  （plan624 线，基线同败非 020）；R-02 info=auto-lang lib-test 于
  `cargo t -p auto` 特性单化下 E0425 set_menubar_open（git show 基线
  已含同引用，非 020；转介 infra 线）；R-03 info（已改）=SD-01 缺路由
  触发条件句+SD-02 证据指针陈旧——复审职权内 delta 对齐，两 worktree
  各 1 提交；R-04 info=生成器出根装配怪癖（tetris workspace 依赖路径
  指向主检出+相对 target-dir 破碎——9cac4fd96 同族环境项，车辆装配手
  工纠正，非 020 缺陷，建议后续生成器修）| evidence: 本计划 §8 各 [✅]
  行+assets/020/ 四件+复现命令（metrics 报告/e2e env 口径）|
  next: merge（auto-plan-merge；SD-02 canonical 行于 os worktree
  plan-020-dev 待发布，组 worktree 清理随 merge 收尾）。

- 2026-09-15 /auto-plan:merge 收据（`PLAN-020:r1`——部分着陆，lang 侧
  发布阻断）：
  `stage: merge | PLAN-020 | rev 1 | outcome: blocked（publication-only）|` 
  **检查点**：
  - `prepared` ✅ lang 分支调和 master（`cef6eb671`——KNOWN-DEBT 双留
    解决，PLAN-632/545 增补与 P020-D1..D4 共存）；调和树重跑门：
    desktop_protocol 130/131+在册红、session 73/73、terminal 30/30
    （PLAN-019 共存实证）、p020 e2e 全绿、`cargo check -p auto` 过；
    ledger 条目文案已备（本收据下方）。
  - `landed` ◐ **auto-os ✅**——main FF `86e0581`（os 分支调和 2fb33b8
    后 FF），SD-02 canonical 3a 行 + scripts/smoke-020-native-exe.sh 发布，
    025 WIP 零卷入；**auto-lang ⛔ 阻断**——`git merge --ff-only
    plan-020-dev` 拒绝：主检出并发会话 WIP（PLAN-019 线 rust_ui.rs
    merged-db 垫片 + renderer.rs/vue.rs）与 FF 改写面重叠（git 原文
    "would be overwritten by merge"），master 停留 4a1b8cf59。
  - `ledger_refreshed` ⏳ 待 lang landed 后 upsert（auto-lang
    `.autoos/specs.json` runtime-only 离线读改写；条目文案 prepared：
    P020-1 designs=SD-01→desktop-protocol-v1.md §1.6+SD-02→os 程序 3a 行
    +SD-03→overview.md 指针；P020-2 tests=e2e/门套件/度量复现命令与
    结果；P020-3 reviews=复审+本收据）。
  - `archived` ⏳ / `cleaned` ⏳ 待 lang landed（含 wt-guard 双组清理：
    .wt/lang-020/auto-lang、.wt/os-020/auto-os）。
  **解除动作**（唯一）：并发会话（PLAN-019 线）提交/stash 其主检出
  WIP 后，于 auto-lang 主检出重跑 `git merge --ff-only plan-020-dev`
  →FF 至 cef6eb671 →ledger upsert→归档+清理（可由任何会话按本收据
  机械完成）。
  计划保持 `reviewed`（发布型阻断，非交付缺陷）。

- 2026-09-15 /auto-plan:merge 收据续（`PLAN-020:r1` 完成）：
  - `landed` ✅（补全）——auto-lang master FF `ff436bad2`（他线 renderer.rs
    eprintln 遗留经选择性 stash 让路，stash 留属主；二次调和 56eb983a3/
    ff436bad2 门重跑绿）；auto-os `86e0581`（前证）。
  - `ledger_refreshed` ✅——auto-lang `.autoos/specs.json` upsert
    P020-1(designs)/P020-2(tests)/P020-3(reviews)，读回验证过。
  - `archived` ✅——本文件 `git mv` 至 `docs/plans/archive/`，status:
    archived。
  - `cleaned` ✅（见下补记）。

## 10. 待澄清事项

- **①（已定案，T-01）** queue 臂命中→动作策略 = **B（运行期 View 投影）**，
  见 §5.1 定案记录；策略 A 否决。
- **②** exe 发现机制：pac `desktop_exe:` 显式字段 vs 纯约定路径扫描：
  推荐 pac 字段为主 + rust-workspace 约定兜底（T-06 依定案落）。
- **③** native 组件 auto 裁决缺省臂：推荐 independent 保底（queue 覆盖
  爬坡前安全缺省），与解释态 auto 语义并列记录进 v1.6。
- **④** 度量口径：沿用 508 报告（系统总边际 + Private/WS 双口径），避免
  480 双口径分歧重演——如需单口径，须在 T-08 前定。
- **⑤** outproc 落位固定 (16,16)/480×320（不级联）已知差异：本轮不新做，
  维持裁定期已知差异记录；如用户要求对齐 inproc 级联，另开小任务。
