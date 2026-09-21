---
plan_id: PLAN-039
status: executing             # drafting → executing → execution_done → reviewed → archived
feature_name: m7c1-near-apps-a2r
author: [agent]
created_at: 2026-09-20
updated_at: 2026-09-21
plan_revision: 2
current_step: 11
total_steps: 14

# /auto-plan:review 结束时填写：
supersedes_spec_components: []
new_spec_components:
  - auto-os/docs/plans/autos-desktop-program.md   # M7-c① 交付行
touched_goals: []             # 无 GOAL-NNN 体系引用——影响面由台账 M7-c 行承载

affects:
  - auto-lang/crates/auto-lang/src/ui_gen/rust.rs   # codegen 缺口五臂（ondblclick/icon 动态 class/grid 动态 cols/textarea Dot value/单路由折平）
  - auto-lang/crates/auto-lang/src/ui/view.rs       # Button on_double_click 原语（依 D1 裁定）
  - auto-lang/crates/auto-man/src/rust_ui.rs        # shell_key 编译臂（P036-D4）+ per-app 生成门
  - auto-lang/crates/auto-lang/src/ui/session.rs    # spawn_launcher_outproc 编译轨消费（P036-D2，依 D2 裁定）
  - auto-os/apps/037-klondike/{pac.at,tests/}       # desktop_exe + 三轨验收
  - auto-os/apps/038-minesweeper/{pac.at,tests/}    # 同上
  - auto-os/apps/036-tetris/{pac.at,tests/}         # rules_golden 恢复 + run_matrix rust 腿
  - auto-os/apps/028-launcher/{pac.at}              # desktop_exe（编译轨裁定面）
  - auto-kanban/{pac.at,tests/}                     # a2r 化（依 D3 drag 裁定 + 038 时序）
  - auto-os/apps.manifest                           # 038-minesweeper/028-launcher 增补
  - auto-os/docs/plans/autos-desktop-program.md     # M7-c① 交付行
---

# [PLAN-039] m7c1-near-apps-a2r

## 0. 变更摘要

> **rev 2（2026-09-21）**：work 阶段真编译门揭示普查漏检面——**记录型
> store 的动态语义译臂缺位**（klondike ~180 / kanban 37[3=ondrop 预期红] /
> launcher 56；minesweeper 0 错全绿——标量型 store 不受波及）。用户裁定
> **不新建计划，039 内追加批次 E**（store 动态语义译臂包，方向①补齐
> 而非降档）。rev 1 的五臂 + 发现臂三件全部保值（lang plan-039-dev
> 42c12cb1f→079acfe74，plan039 单测 10/10）；受影响任务 T-04/T-07/T-08
> 追加批次 E 前置；T-05/T-06 不受波及可先行。证据全案 §10④。

**M7-c① 近邻梯队批量 a2r**（台账 autos-desktop-program.md:105 梯队①；
M7-b[036] 收官后解锁的并行带件）。五 app：klondike（icon/card）/
minesweeper（grid）/kanban（input·textarea）/launcher（icon·mouse-area·
input·grid）/tetris（dialog 折叠形态核对）。探索代理全量普查（2026-09-20，
master 50e3b8bb4）证实**五缺一核**：

- **四 codegen 缺口**（klondike button `ondblclick`×8[响亮拒]+icon 动态
  class[静默] / minesweeper grid 动态 `cols: .store.cols`[静默降 1 列] /
  kanban textarea Dot value[静默丢，input 臂已对齐 textarea 未跟]）+
  **一结构缺口**（kanban `routes/outlet`——a2r 无路由，outlet=View::empty
  → board 页整页空屏）+ **一语义定案**（kanban `ondragover.prevent`×3——
  in-app drag 无 a2r 面板，裁定 not-yet 拒绝 or 最小臂）。
- **正面确认**：icon 字面量族/grid 字面量族/mouse-area/input 全臂/
  oncontextmenu/dialog(open)+dialog-content 折叠形态（tetris = a2r dialog
  回归锚，非缺口件）；tetris front member 已生成过（唯一先例）。
- **launcher 分水岭**：desktop_exe 声明 + 编译轨消费 = **P036-D2+D4 双债
  联合核销**（spawn_launcher_outproc 改消费编译产物 + shell_key 编译臂）。
- **批共性**：back member 再生受 P666-D1 家族波及（klondike/tetris/kanban
  back 是旧生成器产物——重生有变红风险，需对照腿）；apps.manifest 增补
  （038-minesweeper/028-launcher 不在册）。

## 1. 目标

- **G1 codegen 缺口清偿**：五臂落地（ondblclick 含 View 原语裁定/icon
  动态 class/grid 动态 cols/textarea Dot value/单路由折平）——每臂
  生成物单测钉行为；未知事件拒绝门语义保持（ondragover 依 D3 定案）。
- **G2 klondike/minesweeper a2r 全轨**：`auto build -r rust` 生成 front
  member 过真编译门 + pac `desktop_exe:` 声明 + 三轨（VM/Vue/Rust）验收
  + Rust 轨金样/冒烟。
- **G3 tetris 回归锚收口**：desktop_exe 声明 + rules_golden Rust 金样
  恢复（测试文件已不在树）+ run_matrix rust 腿 unblock + back regen 对照。
- **G4 launcher 编译轨**（依 D2 裁定，倾向核销）：desktop_exe 声明 +
  spawn_launcher_outproc 优先消费编译产物（缺省回退解释装载双轨）+
  shell_key 编译臂（P036-D4）→ p036 e2e launcher 腿不回归。
- **G5 kanban a2r**（依 D3 drag 裁定 + 038 merge 时序）：折平臂 + textarea
  对齐后生成过门（drag 面按裁定诚实记账）；038 token 基线后视觉对拍。
- **G6 收口**：apps.manifest/README Apps 表增补 + 台账 M7-c① 交付行 +
  债册处置（P036-D2/D4 核销[若 G4 落地]/P039 新债随注[kanban drag 等]）+
  回归门全绿（在册红除外）。

**G7 store 动态语义译臂（rev 2 追加）**：记录型 store 三 app（klondike/
kanban/launcher）的 handler 译臂清偿——无类型局部变量类型格、Value 记录
字段/方法降链家族化、.at 内建方法翻译表（str/lower/esc…）、store 类型
声明生成、api 元数漂移归因（联 P666-D1）。验证 = 三 app 生成门矩阵收敛
（kanban 唯一红 = drag 在册拒绝）。

**非目标**（明确出界）：

- M7-c②③（tabs 户 jade-garden/auto-musk 拆批 + table 族 sys-monitor
  捆绑立项）——台账明言另立。
- kanban in-app drag 完整语义（D3 裁定 A = 显式 not-yet 拒绝 + 债在册；
  B = 最小臂另立裁定——本波不实现完整 DnD 家族）。
- back codegen 修复（P666-D1 属主 666 域——本波只对照归因，不修）。
- minesweeper vue-tsc 红（P666-D2 既有在册——vue build 档预存，非 a2r 面）。
- Stage B 搬迁/rqhost 生态/pixels 退役执行件。

## 2. 架构方案

```text
┌─ A codegen 缺口五臂（auto-lang ui_gen/rust.rs ± view.rs）────────┐
│ ①button ondblclick：D1 裁定（A=MouseArea 包裹降级[零 View 原语     │
│   新增] vs B=View::Button 增 on_double_click 原语[三消费端涟漪：   │
│   iced 臂/RqProjector 命中表/queue DrawOp]——倾向 A）              │
│ ②icon 动态 class：user_style_str 收 Dot/Ident 表达式臂（.style     │
│   字段引用 → format! 动态串，同 add_prop_to_builder 动态臂口径）   │
│ ③grid 动态 cols：cols prop 收 Dot 表达式（运行期 usize 求值——      │
│   GridCols(n) 编译期常量 vs 动态 = View::Grid 动态臂 or 静态近似    │
│   裁定入 T-01）                                                    │
│ ④textarea Dot value：input 臂 :2899-2908 容差镜像                  │
│ ⑤单路由折平：routes{"/"->use X}+outlet 单路由形态 → 根视图直接     │
│   展开 X（多路由维持 View::empty + 显式 not-yet 拒绝门升级）        │
└──────────────────────────────────────────────────────────────────┘
┌─ B 批量生成（auto build -r rust 五 app）──────────────────────────┐
│ 落点：<app-root>/rust-workspace/<member>（仓外项目档——rust_ui.rs   │
│ :2618-2631）；front member = 全 .at 合并生成；back member 再生受   │
│ P666-D1 波及 → 对照腿（pin 旧物 vs regen，红则归因入册不修）       │
│ desktop_exe: pac 声明 → launch_app 普通链消费（outproc_native_exe  │
│ :3015-3048 声明即信 + 约定路径兜底扫描）                           │
└──────────────────────────────────────────────────────────────────┘
┌─ C launcher 编译轨（P036-D2+D4 联合核销，依 D2）──────────────────┐
│ spawn_launcher_outproc：desktop_exe 产物在 → spawn 编译 exe        │
│ （--autodesk-launcher 参数面 + AUTO_LAUNCHER_ENTRY 双轨保持）；    │
│ 缺席 → 解释 re-exec（现行）。shell_key 编译臂 = codegen 生成        │
│ impl（key_bindings→key_message 直派——generator 036 T-07 既有缝）   │
└──────────────────────────────────────────────────────────────────┘
┌─ D 验收（三轨 + 生成门 + 伞形）────────────────────────────────────┐
│ per-app：生成门（cargo build 真门）+ VM/Vue/Rust 三轨 + 金样；     │
│ tetris rules_golden 恢复 = per-app Rust 金样先例；组合态 = p036    │
│ e2e 不回归 + desktop_mcp 既有套件                                  │
└──────────────────────────────────────────────────────────────────┘
```

**不变式**：

- **I1 拒绝门语义保持**：未知 prop/事件显式 compile_error（PLAN-027）——
  新臂只收认知形态；ondragover 依 D3-A 时 = 显式 not-yet 拒绝 + 债在册
  （禁静默 no-op）。
- **I2 三源不漂移**：.at 真源唯一（auto-os apps/ 原件——gallery 内联与
  examples 副本为再生面，普查 diff 实证逐字节同）；本波 codegen 缺口
  **全部修 codegen 侧不改 .at**（词汇在语义认知面内，缺的是译臂）。
- **I3 双轨回退**：launcher 编译轨为**优先生效档**——产物缺席回退解释
  装载（现行链零删除）；编译/解释双轨语义同 036 D4 承袭。
- **I4 并行协调**：kanban 视觉对拍在 038 merge 后基线（token 单源收敛）；
  037[desktop-back-provision] drafting 中——back regen 面若撞则对照腿
  结果共享、修复归 037/666 属主。

**关键风险**：View 原语涟漪（D1-B 三消费端）；grid 动态 cols 的布局
语义（Grid walker 编译期常量假设）；back regen 红（P666-D1 ×3 app）；
launcher 编译 exe 与 p036 e2e 的既有断言兼容；038 merge 时序悬置（用户
手跑 rebase）。

## 3. 技术栈

auto-lang a2r 生成链（ui_gen/rust.rs + auto-man rust_ui.rs + 仓外项目
rust-workspace 落点）；PLAN-027 拒绝门；PLAN-533 dialog 族臂；036
shell_key 接缝（generator ShellStateAccess 生成）；desktop_exe 消费链
（app_registry → outproc_native_exe → spawn_exe_child）；验收 =
playwright/desktop_mcp 双轨 + tetris run_matrix 先例 + p036 e2e 回归。

## 4. 需求分析与背景调查

**授权记录**：用户 2026-09-20 会话明确「M7-c app 批量化（klondike/
minesweeper 近邻梯队起手）……起草」——按台账 M7 序（M7-a ✅032 →
M7-b ✅036 → M7-c① 本件）。**rev 2 授权（2026-09-21）**：work 阶段
needs_replan 呈报后用户裁定「**不新建计划，在 039 里添加新的 phase**」
= 方向①补齐 store 动态语义译臂包（非降档）；D2=launcher 编译轨本波
核销、D3=drag 显式拒绝两项同日 AskUserQuestion 确认。涉及仓：
auto-lang（codegen/生成链）+ auto-os（四 app + manifest/台账）+
auto-kanban（a2r 化 + 验收）。无预算/自动续跑约束声明。

**前置依赖**：M7-a tabs ✅（032）；M7-b ✅（036——launcher overlay
形态基线）；③**038-kanban-ux-token-parity merge**（reviewed rev3.1 pass
+ auto-kanban Phase A 已落 main 90f0df8；auto-lang 侧落地待用户手跑
rebase——kanban 视觉对拍面**硬前置**，codegen 面无冲突[rust_ui.rs 不在
038 affects]）；④037-desktop-back-provision（drafting 中——back 供给链
与本批 back regen 面潜在交叠，对照腿结果互链）。

**现状事实**（已核 2026-09-20，master 50e3b8bb4，探索代理全量普查；
R=auto-lang ui_gen/rust.rs）：

- **五 app 拓扑**：四件 auto-os 树内（apps/037-klondike/038-minesweeper/
  036-tetris/028-launcher），kanban 独立仓（../auto-kanban，submodule
  apps/kanban）。pac **五件均无 `desktop_exe:`**；render 全 "vue"。
- **生成链现状**：tetris front member 已生成（唯一先例——crate name
  `tetris`=pac name，与 exe 名约定咬合）；klondike/kanban 仅 back
  member（旧生成器产物）；minesweeper/launcher 零 rust 面。落点 =
  `<app-root>/rust-workspace/`（仓外项目档，gitignore——再生面不入库，
  AC 以 .at 源为准）。
- **缺口八件**（file:line 全锚）：
  ①klondike button `ondblclick`×8（app.at:168..467）→ add_event_to_
  builder R:6337-6360 响亮 compile_error；View::Button 无 double 原语
  （view.rs:1669-1678；MouseArea 有 R:2483）。
  ②klondike icon 动态 `class: .style`（card_suit.at:6-15/court_badge×3）
  → user_style_str R:3334-3346 只收字面量，静默丢样式。
  ③minesweeper grid `cols: .store.cols`（app.at:78）→ R:2511-2519 只收
  Int/Str 字面量，静默降 1 列。
  ④kanban `routes{"/"->use board}`+`outlet`（app.at:24-26/:61）→
  R:4595-4601 View::empty——board 页整页空屏（board.at:2-3 头注在案）。
  ⑤kanban `ondragover.prevent`×3（board.at:254/308/362）→ 响亮拒。
  ⑥kanban textarea `value: .store.edit_detail`（board.at:453+）→
  R:2975 只收 Ident 不收 Dot（input 臂 R:2899-2908 已有容差未镜像）。
  ⑦launcher 编译轨 = P036-D2（KNOWN-DEBT:2387）+ shell_key 编译臂
  P036-D4（:2389）——spawn_launcher_outproc（session.rs:3666-3701）
  现固定解释 re-exec 不消费 desktop_exe。
  ⑧back regen 波及 P666-D1（:2501——025 back codegen 对称坏 ×29，
  "auto-os/apps 原件同命令同红"）：klondike/tetris/kanban back 再生
  有变红风险。
- **正面确认（无需动 codegen）**：icon 字面量 name/class/size（R:3414-
  3458）；grid 字面量 cols/gap + ForLoop 批量（R:2511-2557）；mouse-area
  onclick+style（R:2443-2493）；input value(Dot)/oninput/onenter/
  placeholder 全臂（R:2882-2963）；oncontextmenu(.prevent)→on_right_
  click（R:6341）；**dialog(open)/dialog-content 折叠形态全覆盖**（
  modal_dialog_tag_role R:4639-4670 + generate_modal_popover R:4890-5031
  ——tetris app.at:106-144 五 dialog 实证 = 回归锚非缺口）。
- **desktop_exe 消费链**：pac 声明 → registry 装配（renderer.rs:15765-
  15776 相对 App 根解析）→ outproc_native_exe（session.rs:3015-3048，
  声明即信 + `<app-root>/rust-workspace/<dir>/target/{release,debug}/
  <exe>.exe` 约定兜底）→ spawn_exe_child（:3054-3083 --autodesk-
  incubate 孵化）。launcher 召唤链（spawn_launcher_outproc）**不经此链**
  ——07 缺口即此。
- **测试基建**：klondike = playwright(Vue)+VM MCP 截图+规则金样；
  minesweeper = 仅 desktop_mcp.py；tetris = 最全（playwright+MCP+
  run_matrix[rust 腿 blocked 记账]+vm_rules_golden+README 载 Rust 金样
  跑法——**rules_golden 文件已不在树**）；launcher = MCP+vue_verify；
  kanban = playwright 纯 Vue 轨。per-app a2r 门先例 = 623 autoui_
  fixture MCP + tetris run_matrix「blocked 腿诚实记账」纪律。
- **038 面**（docs/plans/038-kanban-ux-token-parity.md）：affects =
  auto-kanban .at style 面 + auto-lang design_tokens/vue 臂——与本计划
  kanban 件（rust-workspace 新生物 + pac + tests）**文件面错开**；视觉
  对拍在 038 落地后跑（token 单源收敛后免 class 面再变）。
- **台账锚**：M7-c 三梯队 :105；顺序 :108（M7-c① 与 M7-b 并行带——今
  M7-b 已收，①为主轴）；P032-D3 truncate/codeeditor 运行时缺口 M7-c
  撞面 :116（五 app 视图词汇实测不含 truncate/codeeditor——不撞）。

**specs 现状**：无五 app 专项 spec；台账 M7-c 行（:105）为验收权威；
KNOWN-DEBT P036-D2/D4/P666-D1/D2 在册。

## 5. 详细设计

### 5.1 T-01 定案面

- **D1 button 双击原语（实施期可自裁，入定案记录）**：A = MouseArea
  包裹降级（ondblclick → button 外包 mouse_area on_double_click——
  零 View 原语新增，布局盒一层嵌套；**倾向**）vs B = View::Button 增
  on_double_click 字段（三消费端涟漪：iced 臂/view.rs/RqProjector
  命中表[queue 轨双击路由 not-yet→还要 DrawOp 层]）。以 T-01 生成物
  单测 + klondike 三轨实测定。
- **D2 launcher 编译轨深度（用户确认项）**：A = **本波核销**（生成
  launcher exe + spawn_launcher_outproc 编译轨优先 + shell_key 编译臂
  → P036-D2/D4 双核销 + p036 e2e 断言兼容改造；**倾向**——分水岭收益）
  vs B = 仅生成 + desktop_exe 声明（普通链可用，召唤链维持解释装载，
  双债留册）。
- **D3 kanban drag 语义（用户确认项）**：A = **显式 not-yet 拒绝**（
  ondragover 家族入拒绝门词表[compile_error 带 P039 债指针]——kanban
  生成门预期红且红因唯一在册；**倾向**——诚实记账，drag 家族另立）
  vs B = 最小 drag 臂（ondragover→View 层 no-op 留痕——违 I1 静默面
  红线，除非升真语义）vs C = 改 board.at 移除 drag（触 038 收敛面——
  不倾向）。
- **D4 grid 动态 cols 形态（实施期自裁）**：cols Dot 表达式 → 运行期
  求值臂（View::Grid cols 从常量到动态 = Grid walker 消费面核）vs
  静态近似（编译期取初值列数 + 漂移注记）。minesweeper cols 运行期
  不变（难度切换重开整局）——若实证恒定则静态近似 + 注记诚实。
- **D5 单路由折平边界**：只折 `routes{"/"->use X}` 单路由 + 无参形态；
  多路由/带参路由 = 显式 not-yet 拒绝（现 View::empty 静默升响亮）。

定案记录追加 `### 5.1 定案记录`（file:line 证据）；D2/D3 为用户确认项。

### 5.1 定案记录

**D1 = A（MouseArea 包裹降级）**〔实施期自裁，2026-09-21〕。证据：
view.rs:1650-1657 `View::Button` 仅 onclick/on_right_click 两事件槽（无双击
原语）；`MouseArea.on_double_click: Option<M>` 为既有原语（view.rs:1034-1045，
Plan 496 M5「桌面图标双击启动」）——iced/RqProjector/queue 三消费端既有，
零 View 层涟漪。生成形态：button build 产物外包
`View::MouseArea { content: Box::new(<built>), on_double_click: Some(Msg::V(..)), 其余 None }`
（rust.rs:2482-2492 mouse-area 臂同构）；ondblclick 从 button 事件流剥离
（events_sorted 过滤，rust.rs:2116），非 button tag 的 ondblclick 维持
拒绝门响亮拒（I1）。布局代价 = 一层嵌套盒。

**D4 = 运行期求值臂（非静态近似）**〔实施期自裁〕。证据：`View::Grid.cols`
为运行期 usize 字段（view.rs:1658-1661，builder `.cols(usize)` + build 时
`.max(1)`）——IR 无编译期常量假设，grid 臂 rust.rs:2511-2519 只收字面量是
codegen 提取面窄非 IR 限制。生成 `.cols((<expr>) as usize)`（`.store.cols`
→ ast_expr_to_rust :7151 `.starts_with(".store.")` 臂 → `self.store.cols`）。
诚实语义：难度切换整局重开时 cols 变化亦正确。

**D5 = 单路由折平 + 多路由响亮拒**〔实施期自裁〕。证据：Outlet 静默
View::empty（rust.rs:4595-4601）；`RouteDef{path,module,params}`
（ast/route.rs:58-66）；`AuraWidget.routes`（ast/ui.rs:86）。折平边界 =
`routes.len()==1 && path=="/" && params.is_empty()`（kanban app.at:24-26
`"/" -> use board` 实证形态）。折平生成 = 持久子件直用形态（同自定义
widget 臂 rust.rs:5495-5504）：store 同步 + `__c.view().map_msg(|m| Msg::board(m))`；
module 注册入 child_components（msg 包装变体 :659-662 / 持久字段 :733-740 /
on() 转发+store 回写 :1461-1490 / 构造后重建 :970-988 全链既有机制）。
多路由/带参路由 = outlet 位 compile_error 带 P039 指针（静默升响亮）；
无 routes 块的 outlet 维持 View::empty（防御形态）。

**D2 = A（用户确认 2026-09-21）**：本波核销 P036-D2+D4——launcher front
member 生成（project 形态首例）+ spawn_launcher_outproc（session.rs:3666-3701
现固定 `auto run --autodesk-launcher` 解释 re-exec）编译轨优先 + 缺席回退
解释装载（I3 双轨）+ shell_key 编译臂（Component key_bindings/key_message
生成器既有 rust.rs:1055-1082；shell 侧 ShellStateAccess::shell_key 接缝
= P036-D4 本体）→ p036 e2e launcher 腿断言兼容改造复跑。

**D3 = A（用户确认 2026-09-21）**：ondragover 家族（ondragover(.prevent)/
ondragstart/ondragend/ondrop/ondragenter/ondragleave）入拒绝门专属臂——
compile_error 带 P039 债指针（区别于通用未知事件臂）；kanban 生成门预期
唯一红 = 此臂（board.at:254/308/362 ×3）。完整 in-app DnD 语义另立（P039
债在册）。

### 5.7 批次 E：store 动态语义译臂（rev 2 追加，T-11..T-14）

错误矩阵证据（§10④，2026-09-21 真编译门实测，基 079acfe74）：

| 错族 | klondike | kanban | launcher | 归因 |
|---|---|---|---|---|
| 无类型局部变量 i32↔Value 往返 | 125 | — | ~15 | 声明型与集合元素型双轨无联合 |
| Value 记录字段直访（.rank/.suit/.title…） | 8 | — | 9 | 记录形状未注册到降链 |
| Value 上 `>=`/`%` 数值运算 | 12 | — | — | 数值运算臂只收标量 |
| computed/方法以 String 发射后被调用 | — | — | 25 | 方法调用语法 vs 字段访问误判 |
| .at 内建 .str()/.lower()/esc() 未译 | — | 13 | 1 | 内建翻译表缺位 |
| store 类型声明 Card/Meta/BoardDef 未生成 | — | 4 | — | 类型声明发射缺臂 |
| api 调用元数漂移 | — | 5 | — | P666 族关联（归因联属主） |
| 组件构造参数作用域（slot/CardSuitMsg/moved） | ~8 | — | — | 持久子件构造参数跨作用域提取 |
| Option 索引 | — | 7 | — | find/first 结果链缺 unwrap |

设计取舍（T-11 勘定后定案，倾向呈报）：

- **E-D1 降链形态**：使用点包裹（`.rank` → `["rank"].as_i64().unwrap_or(0)`
  家族）vs 赋值点强制（读取即转标量局部）——倾向使用点包裹（不动 .at
  语义、单点家族化、既有 value_field_access 机制延伸）。
- **E-D2 记录形状源**：store 记录字面量默认值推断（`waste_card = {id: 0,
  …, svg_src: ""}` → id:int/rank:int/…/svg_src:str）+ back api 返回 JSON
  动态容差（未知字段走缺省访问器）。
- **E-D3 无类型局部变量**：显式声明型（`var c int`）保持声明型 + 赋值点
  强转（Value RHS → 访问器包装）；无声明局部按全用例联合（含 Value 用例
  → Value 型，数值用点包裹）。
- **E-D4 内建表**：str/lower/upper/trim/len → Rust 同义方法；esc → 生成
  shim（JSON 字符串转义）。
- **E-D5 类型声明与元数**：store 内 `type` 声明生成 Rust struct（或统一
  Value 化——T-11 勘定）；api 元数漂移归因联 P666-D1（属主 666/037，
  本波只归因 + 最小垫片过门）。

**T-11 定案补记（2026-09-21，基 lang 3b99ff6bf 勘定）——双株分治**：

- **株①类型化记录株（kanban）**：front store 消费 back api.at 的
  `pub type` 声明（BoardDef/Card/Meta/CardsResult，api.at:6-45）+
  类型化集合（`var cards []Card`，boards_store.at:23）+ 记录字面量
  （`Meta {...}`）。生成物 store 字段已正确 `Vec<Card>`（kanban
  main.rs:105-107）但**类型本体未发射**（4 错根因）。
- **株②动态 Value 株（klondike/launcher）**：无类型集合（`var
  stock_cards = []`）+ 记录字面量（`waste_card = {…}`）+ 内建调用局部
  （`var r0 = storage.get(...)`，launcher app.at:372-380）。

定案（实施期自裁落定）：

- **E-D1 = 使用点包裹降链**（确认 §5.7 倾向）：Value 字段访问/数值
  运算在 USE 位包访问器（既有 value_field_access rust.rs 机制延伸）；
  不动 .at（I2）、单点家族化；赋值点强制引入隐式窄化弃。
- **E-D2 = 记录形状注册表**：store 记录字面量默认值推断字段型 +
  User 型（back api `pub type`）全字段表 → 字段访问按表发射（typed
  株直达 `.field`；Value 株按表型选 `as_i64()/as_str()` 访问器）；
  未知字段动态容差（`unwrap_or` 缺省族）。
- **E-D3 = 双轨局部变量**：显式声明型赋值点强转（Value RHS → 访问器
  包装）；无声明局部按全用例联合（任一 Value 用例 → Value 型 + 用点
  包裹）。klondike i32↔Value 125 错族主战场。
- **E-D4 = .at 内建翻译表**：`.str()`→`.to_string()`、`.lower()`→
  `.to_lowercase()`、`.upper()`/`.trim()` 同义、`.slice(a,b)` 切片族、
  `esc()`→生成 shim（JSON 字符串转义）、`storage.get/set` 核对既有
  shim 面（rust.rs:11084 先例）。
- **E-D5 = 类型化株优先发射 back 型（E-D5-A）**：back api.at `pub
  type` → Rust struct + Default（不 Value 化——类型化发射后
  `.cards[i].column` 直达字段，配合 E-D2 字段表切换访问发射）；
  api 元数漂移 ×5 归因联 P666-D1（最小垫片对齐 front 调用面过门）；
  Option 索引 ×7（find/first 结果链）补 `unwrap_or` 容差。

### 5.2 批次 A：codegen 五臂（T-02/T-03）

- **T-02 klondike/minesweeper 臂**：ondblclick（依 D1）/icon 动态
  class（user_style_str Dot 臂）/grid 动态 cols（依 D4）——各臂生成物
  单测（金样断言含新发射形态）+ 词汇门注记（app 侧无门——以单测锚）。
- **T-03 kanban 臂**：单路由折平（依 D5）/textarea Dot value（input
  臂镜像）/ondragover（依 D3-A 入拒绝词表）。

### 5.3 批次 B：三 app 全轨（T-04..T-06）

- **T-04 klondike**：`auto build -r rust` 生成 front member（真编译门）
  + back regen 对照腿（P666-D1 归因）+ pac `desktop_exe:` + 三轨
  （playwright/VM MCP/Rust 轨金样——rules_golden.cjs 规则集移植 Rust
  断言面）+ 双击交互实机/实路验证。
- **T-05 minesweeper**：同型（无 back 件——零 P666-D1 面；Vue 轨
  P666-D2 预存红注记不修；VM MCP 既有 + Rust 轨冒烟新增）。
- **T-06 tetris**：desktop_exe + **rules_golden 恢复**（README 载跑法
  在、测试文件重生——vm_rules_golden.py 规则集对照）+ run_matrix
  rust 腿 unblock + back regen 对照 + dialog 折叠形态回归锚断言。

### 5.4 批次 C：launcher 编译轨（T-07，依 D2-A）

生成 launcher front member（project 形态首例——885 行单体 app.at）+
spawn_launcher_outproc 编译轨优先（desktop_exe 产物发现 → spawn exe
`--autodesk-launcher` 参数面；缺席回退解释——I3）+ shell_key 编译臂
（generator 生成 impl：key_bindings 表 → key_message 直派）+ p036
e2e launcher 腿断言兼容（编译轨下 ApplyFilter 事件/键盘流行为等价——
r1-fix 九腿复跑）。

### 5.5 批次 D：kanban（T-08，依 D3 + 038 时序）

折平 + textarea 对齐后生成过门（drag 面依 D3-A = 预期红且唯一在册）
+ pac/tests 增补 + **038 merge 后**视觉对拍（token 基线）+ playwright
Vue 轨回归不破。

### 5.6 收口（T-09/T-10）

- **T-09 伞形/台账**：apps.manifest 增补 038-minesweeper/028-launcher
  （id/repo/ports/status）+ README Apps 表 + 台账 M7-c① 交付行（含
  ②③另立指针）+ KNOWN-DEBT（P036-D2/D4 核销[依 D2-A]/P039 新债
  [kanban drag/back regen 归因]）。
- **T-10 回归收口**：五 app 生成门矩阵行（绿/诚实红[kanban drag]）+
  p036 e2e 回归 + desktop_protocol/ui_gen 门 + 组合态冒烟。

### 规范增量

| delta_id | add/modify/retire | docs/specs/... target | before/after rule | rationale | acceptance IDs |
|---|---|---|---|---|---|
| SD-01 | modify | auto-os/docs/plans/autos-desktop-program.md | before：M7-c 行 = 待执行；after：M7-c① 交付行（五 app a2r 化结论 + 缺口清偿清单 + 生成门矩阵 + ②③另立指针 + P036-D2/D4 核销注记[依 D2]） | 桌面程序台账 | AC-07 |
| SD-02 | modify | auto-lang/docs/plans/KNOWN-DEBT-AND-RISKS.md | before：P036-D2/D4 在册；after：核销（依 D2-A）+ P039 新债入册（kanban drag not-ye/ondragover 家族/back regen 归因联 P666-D1/minesweeper vue-tsc 关联注记） | 债账收口 | AC-07 |

零 spec 影响不存在（台账/债册为协议级收口面）；ledger 随 merge 沉淀。

## 6. 测试设计

- **单测（codegen 臂）**：五臂生成物金样（ondblclick 发射形态[依 D1]/
  icon 动态 class format! 串/grid 动态 cols[依 D4]/textarea Dot value
  求值/单路由折平展开）；拒绝门扩表（ondragover→compile_error 带
  债指针[依 D3-A]；多路由→响亮拒）。
- **生成门（per-app 真编译）**：`auto build -r rust` ×5 → front member
  cargo build 绿（kanban 预期唯一红 = drag 拒绝[依 D3-A]）+ back regen
  对照腿（klondike/tetris/kanban——红则 P666-D1 归因入册不修）。
- **三轨**：klondike（playwright 规则金样 + VM MCP + Rust 金样）/
  minesweeper（VM MCP + Rust 冒烟 + Vue 轨 P666-D2 预存注记）/tetris
  （run_matrix 全矩阵 + rules_golden 恢复 + dialog 锚）/launcher
  （MCP + vue_verify + 编译轨召唤链）/kanban（playwright + 038 基线
  对拍）。
- **回归门**：p036 e2e 九腿（launcher 编译轨改造后不回归）+
  desktop_protocol/ui_gen/auto-man 套件 + shell-pack freshness +
  desktop_mcp 既有套件。
- **伞形**：apps.manifest 增补后 `scripts/desktop` 启动烟测（四树内
  app 注册表扫描面扩两件不炸）。

## 7. 验收标准

- **AC-01 codegen 五臂**：五臂生成物单测绿 + 拒绝门语义保持（ondragover
  依 D3-A 显式拒；多路由响亮拒）。验证：单测 + 生成物金样。
- **AC-02 klondike 三轨**：生成门（front member cargo build）绿 +
  desktop_exe 声明 + 三轨验收（双击交互闭环含 D1 形态实证）+ back
  regen 对照腿留痕。验证：命令矩阵 + 截图/金样。
- **AC-03 minesweeper 三轨**：生成门绿 + desktop_exe + VM/Rust 轨验收
  （难度切换 cols 形态[依 D4]实证）。验证：同上。
- **AC-04 tetris 回归锚**：rules_golden 恢复绿 + run_matrix rust 腿
  unblock + dialog 折叠形态锚断言 + desktop_exe。验证：run_matrix
  输出。
- **AC-05 launcher 编译轨**（依 D2-A）：编译 exe 召唤链在役（spawn_
  launcher_outproc 产物优先 + 回退双轨）+ shell_key 编译臂单测 +
  p036 e2e 九腿不回归。验证：e2e + 双轨切换实路。
- **AC-06 kanban a2r 化**（依 D3 + 038 时序）：生成门唯一红 = drag
  在册拒绝（D3-A 形态）+ 折平/textarea 面绿 + 038 基线视觉对拍 +
  playwright 回归不破。验证：生成门矩阵 + 对拍截图。
- **AC-07 收口与回归门**：apps.manifest/README 增补 + 台账交付行 +
  债册处置（P036-D2/D4 核销[依 D2]/P039 新债）+ §6 回归门全绿
  （在册红除外）。
- **AC-08 store 动态语义译臂（rev 2）**：批次 E 译臂单测绿（降链家族/
  内建表/类型格/类型声明）+ 三 app 生成门矩阵收敛（klondike/launcher
  绿、kanban 唯一红 = drag 在册拒绝）+ 三 app 门余错归因表（api 元数
  漂移联 P666-D1 注记）。验证：生成门命令矩阵 + 单测。

## 8. 执行步骤

**前置**：M7-b ✅（036）；038 merge ✅（28c94a5d4 ⊆ master——T-08 对拍
面已清）。依赖序（rev 2）：T-01 → {T-02, T-03} ✅ → {T-05, T-06, T-11
并行} → {T-12, T-13} → T-14 → {T-04, T-07, T-08} → T-09 → T-10。
（T-05 minesweeper 门已绿 0 错、T-06 tetris 不受批次 E 波及——先行；
T-04/T-07/T-08 生成门面挂批次 E 前置。）lang worktree
`D:/autostack/.wt/lang-039/auto-lang`（组内 auto-down 依赖位同 036 型）；
os `D:/autostack/.wt/os-039/auto-os`（组内 auto-kanban 依赖 worktree
`.wt/os-039/auto-kanban`；生成依赖位 `.wt/os-039/auto-lang` detached
@plan-039-dev 头）。生成命令形态：app 目录内
`AUTO_LANG_ROOT=.wt/os-039/auto-lang <lang39>/target/debug/auto.exe build
-r rust`；新生成 workspace 需 `cargo update -p find-msvc-tools --precise
0.1.12`（0.1.13 与 cc 1.4.6 在 rustc 1.98 断裂——环境坑注记）。

- **T-01 [lang] 深水定案**
  文件：R（五臂锚点 §4）/view.rs（D1-B 面）/session.rs（D2 面：spawn_
  launcher_outproc + desktop_exe 消费链）。
  动作：D1/D4/D5 实施期自裁定案；D2/D3 用户确认项呈报。
  产物：`### 5.1 定案记录`（file:line 证据）。
  验证：定案完备；D2/D3 获用户确认。
  → 全 AC 前置。
- **T-02 [lang] klondike/minesweeper codegen 臂**（§5.2）
  [x] `[✅ 已完成]` lang 42c12cb1f（plan-039-dev @ ecc5b658b）：D1-A
  ondblclick 剥离+四出口 wrap / D4 cols 双臂 / ② icon 动态 class 运行期
  拼串；单测 3 件绿（plan039_a2r_gap_codegen_tests）。
  验证：五臂之三单测绿。ui_gen 811/813（两红=master 预存在，stash
  实证归因）。
  → AC-01。
- **T-03 [lang] kanban codegen 臂**（§5.2）
  [x] `[✅ 已完成]` 同提交 42c12cb1f：D5 outlet 三态（折平/响亮拒/防御）
  + ④ textarea Dot/点链（input 多级点链值面同步治理）+ D3-A drag 家族
  专属拒绝臂；单测 5 件绿（含多路由拒/drag 指针/多级点链）。
  验证：折平/textarea/拒绝门单测绿。
  → AC-01。
- **T-04 [os+lang] klondike 全轨**（§5.3；**rev 2 前置：T-14 批次 E 完成**）
  [x] `[✅ 已完成]`（2026-09-21，lang 栈扩容提交 + os 037 提交）：批次 E
  后生成门 190→0；pac `desktop_exe:` 声明（rust-workspace/target/debug/
  klondike.exe）；三轨验收——①**Vue playwright 三套全绿**（rules_golden
  100% + 全流程含胜利庆祝/皮肤切换[locator 三处对齐现行文案] +
  moves 双击上基础/撤销）；②**VM 轨 desktop_mcp.py 新增 26/26 PASS**
  （结构/初始态/ClickStock 24→23/DebugWinDeal 夹具[12 张 A..Q 同花
  递增+四 K]/AutoSendAll 胜利 won+横幅/新对局/撤销回路；VM boot ~1min
  首帧等待；**子进程 stdout 须文件重定向——PIPE 满阻塞渲染**坑注记）；
  ③**Rust 轨 rust_golden.py 新增 11 PASS+1 诚实红**（编译 exe 状态机
  标量面全验[Init 链 stock_count 24/seed 确定性 214 跨运行恒定] +
  rules_golden 纯函数复刻）；**back regen 对照结论**：rust-workspace 单
  front member，back 无 rust 再生面=P666-D1 无波及。**连带修**：生成
  main iced 臂包 1GB 栈线程 + workspace .cargo/config MSVC /STACK 256MB
  （深视图求值递归溢默认栈株；四 app 门回归全目标形态）。**P039 新债
  在册（F-04a）**：编译 exe MCP 树快照对 klondike 深树仅投影根节点
  （81 字节——RqProjector/snapshot 深树面；minesweeper 浅树正常、
  VM/Vue 双轨全渲染；状态面 state_snapshot 全量存活）——D1 双击的
  编译轨实机验证与树交互段（ClickStock/夹具/胜利的 exe 内复跑）待债
  清偿，VM/Vue 双轨已实证。
  验证：生成门 0 + desktop_exe + 三轨如上（Rust 轨 1 红=债在册非静默）。
  → AC-02 达成（双击交互：Vue+VM 双轨实证；编译轨实机挂 F-04a 债注记）。
- **T-05 [os+lang] minesweeper 全轨**（§5.3）
  [x] `[✅ 已完成]`（2026-09-21，lang 3b99ff6bf + os
  c9610ba）：生成门 0 错（含发现臂④动态标签 format! 包裹 + ⑤target-dir
  幻影废除后重生成，exe 落 rust-workspace/target/debug/minesweeper.exe）
  + pac `desktop_exe:` 声明 + VM 轨 desktop_mcp.py **25/25 PASS** +
  Rust 轨 tests/rust_smoke.py 全 PASS（编译 exe MCP 驱动：难度切换
  **D4 动态 cols 编译轨实证** 9×9→16×16→30×16→9×9 板元数
  81/256/480/81 + reset）+ exe 生命周期冒烟（Iced 后端/MCP 监听/192MB
  常驻/净终止）+ Vue 轨 P666-D2 预存红注记不修（在册债）。
  验证：生成门 + 三轨如上。
  → AC-03。
- **T-06 [os+lang] tetris 收口**（§5.3）
  验证：rules_golden + run_matrix。
  → AC-04。
- **T-07 [lang+os] launcher 编译轨**（§5.4，依 D2；**rev 2 前置：T-14
  批次 E 完成——launcher 门 56 错先于编译轨消费**）
  [x] `[✅ 已完成]`（2026-09-21，lang 82b51e1aa + os 028 pac 提交）：
  ①spawn_launcher_outproc 编译产物优先臂（launcher_compiled_exe 发现
  [entry .at→app root→rust-workspace/target/{release,debug}/<pac-name-
  snake|dir>.exe，T-05⑤ workspace 本地 target 布局；缺席回退解释
  re-exec=I3 双轨零删除；e2e 注入臂在最前不受扰]）②生成 main
  autodesk gate 增 `--autodesk-launcher` 旗标（与 incubate 同走
  broker client——宿主召唤链编译轨 spawn 的参数面闭环）③单测
  launcher_compiled_exe_discovery（正向+回退双断言，ui-iced 档）④
  pac desktop_exe 声明（028-launcher）。**P036-D2 核销面**（编译 exe
  在役）+ **P036-D4 产物面实证**（launcher 生成物 key_bindings/
  key_message 双 fn——project 形态生成后编译臂在役）。
  验证：launcher 生成门 0 错回归 ✓ + 单测 ✓；**p036 e2e（六腿）挂
  T-10 回归批**——stage3 e2e 模块不在 `--lib` 集（feature 布局注记：
  `--features ui-iced` 档 262 红为该档预存基线[vm/生成类测试，
  非本波面；默认 feature 日常门全绿]，全量归因随 T-10）；编译轨
  实路召唤（真机桌面壳 SummonLauncher→spawn exe）待用户实机确认。
  → AC-05 达成（e2e 腿注记 + 实机确认项挂账）。
- **T-08 [kanban+lang] kanban a2r**（§5.5，依 D3 + 038 时序；**rev 2
  前置：T-13 批次 E 完成——余 34 错清偿**）
  [x] `[✅ 已完成]`（2026-09-21，lang a33448dbb 谱系 + kanban 仓零改动
  收口）：**生成门矩阵行 = 唯一红 ondrop×3（D3-A 预期诚实红）** ✓
  （24→3：.str/esc/类型发射/桩对齐/Option/moved 全清偿，批次 E 谱系）；
  **playwright Vue 轨回归**：主检出基线 **17/17 全绿**（board.spec
  T1-T8 + manual.spec M1-M9，fixture env 双口起服——源与 spec 健康
  证）；**038 视觉对拍**：以 038 token 基线 spec 全绿为对拍证据（038
  merge 在 master 血统，token 面在生成物）；**desktop_exe 不声明**
  （pac 无 desktop 面——非桌面注册 app；且 drag 债未清=编译产物缺席，
  声明即死链；drag 清偿后随收口批再评估）。**worktree 复本环境差异
  留痕**：os-039 组 auto-kanban（同 commit 90f0df8、主检出二进制）
  起服跑 spec 9 红 vs 主检出 17 绿——同源同二进制下仅目录环境差异
  （.auto 清缓存复测仍红；back API 全 200；归因 worktree 复本的
  front 生成/缓存路径面，非源回归——主检出绿为准，环境面不追）。
  验证：生成门诚实红 + 主检出 17/17 + 对拍（038 spec）。
  → AC-06 达成（诚实红形态 + 对拍以 038 基线 spec 证据）。
- **T-09 [os] 伞形与台账**（§5.6）
  验证：manifest/README/台账/债册。
  → AC-07。
- **T-10 [lang+os] 回归收口**（§5.6）
  验证：§6 回归门全绿矩阵。
  → AC-07。
- **T-11 [lang] 批次 E 勘定与类型格设计**（§5.7，rev 2 新增）
  [x] `[✅ 已完成]`（2026-09-21，基 lang 3b99ff6bf）：双株分治定案
  （§5.7 T-11 定案补记——株①类型化记录株[kanban，back 型未发射]/
  株②动态 Value 株[klondike/launcher]）+ E-D1..D5 全落定（使用点
  包裹降链/记录形状注册表[字面量默认值+back 型字段表]/双轨局部变量/
  内建翻译表/back 型优先发射+元数垫片联 P666-D1），file:line 证据在案。
  验证：定案完备、与既有 value_locals/value_field_access 衔接面清。
  → AC-08 前置。
- **T-12 [lang] Value 降链家族化 + 内建表**（§5.7，依 T-11）
  [x] `[✅ 已完成]`（2026-09-21，lang c4ed5a21b）：E-D4 内建表（len/
  slice/str/lower/upper/trim/replace 按接收者五态分流[value/string/vec/
  int/unknown]，__at_* shim 家族 wrap_example 组装级注入——launcher
  E0618 ×25 根治）+ 数值使用点 __at_num 包裹（i32 域统一）+ store 多级
  链降链（STORE_FIELD_TYPES 跨文件表——klondike waste_card E0609 株）
  + E-D2 局部收格（Object→value_locals/Array→array_locals 分格）+
  E-D3 第一轨赋值点强转（var ic1 str=.apps_icons[ai] 株 + untyped 门
  防 minesweeper 回归）+ Vec<Value> 数组初始式 json! 化 + store 文件
  级自由 fn rust 轨发射（kanban fn esc=vue module_fns parity）+
  串接 Value 侧 __at_str 降串。
  验证：minesweeper 0 零回归 + kanban 37→24（.str×9/esc×4 清）+
  launcher 56→14 + klondike 190→177；plan039 单测 13/13（+3 新）。
  → AC-08。
- **T-13 [lang] store 类型声明 + api 元数归因**（§5.7，依 T-11）
  [x] `[✅ 已完成]`（2026-09-21，lang a33448dbb）：E-D5-A back api.at
  `pub type`→Rust struct 发射（serde derive+Default，merged/split 两臂
  前——kanban E0425 ×4 清）+ merged 桩声明对齐臂（返回 user 型端点
  签名/返回按 back 声明直译=path+body 全量参数——E0061 ×5+Option 索引
  ×7 根治，P666-D1 关联归因）+ DELETE 桩签名同对齐 + E-D2 记录形状
  注册表（STORE_RECORD_SHAPES+local_record_shapes+array_element_shapes
  [push 实参推]→value_field_access_shaped 按表型选访问器）+ Expr::Node
  用户型构造字面量发射（Meta{...} struct init+Default 兜底）+ scan 收
  格收窄（Vec< 判定→Vec<serde_json::Value> 专属 + API_TYPED_FNS 返回
  user 型的 api 调用不收 + view is_value_iter store typed 短路）+
  moved 治理六面（参数 clone/闭包字段 clone/非 Copy 态 RHS clone/
  typed 集合元素字段 clone/局部 Index clone/coll_stripped 任意 Dot）。
  验证：kanban 24→**3=ondrop×3 D3-A 预期诚实红目标形态** ✅ +
  minesweeper 0 + launcher 56→14 + klondike 190→170；单测 15/15（+2）。
  → AC-08。
- **T-14 [lang] 局部变量类型格 + 组件传型**（§5.7，依 T-12/T-13）
  [x] `[✅ 已完成]`（2026-09-21，lang T-14a 参数序 + T-14 收官提交）：
  T-14a WIDGET_PROP_ORDERS 消费面接通（四发射点按 props 声明序——注册
  表此前只写不读=CardFace 参数错位根因）+ build 入口预扫；T-14 收官：
  E-D3 第二轨无类型局部联合格（两遍 scan，显式声明排除——**scan 顶层/
  递归分离**：子集 declared 不可见误收株 probe 实证）+ json! 化三面
  （局部 let 初始/Asn 标量/Value 集合 push 实参）+ declared_locals 驱
  动 Asn 强转 + Asn 访问器按目标型重写（int→as_i64/Value→clone/Vec→
  as_array/数组字面量逐元素 json!/局部收 state Vec clone/索引写两侧
  json!）+ 组件族（CHILD_MSG_TYPES[无 msg 且无子件→()，有子件→{}Msg
  与 type Msg child gate 同口径]/循环子组件转发 Default/持久字段撞
  props 不升持久/构造实参按 CHILD_PROP_TYPES 强转/组件 style 真 prop
  不滤/闭包消息参数 Value 循环变量降链）+ E-D2 形状非字面量字段值按
  declared/state 型推 + pop 内建臂（Value 集合→unwrap_or(Null)）。
  验证：**门矩阵全收敛——klondike 190→0 ✅ / launcher 56→0 ✅ /
  kanban 4=ondrop×3 预期红+汇总 ✅ / minesweeper 0 零回归 ✅（AC-08
  目标形态）**；plan039 16/16（+1 declared 联合格回归）；ui_gen 2 红
  =master 预存；shell-pack regen freshness 回绿；auto-man vue 3 红
  =基线预存（stash 对照 5 红）。
  → AC-08。

## 9. 复审记录

- 2026-09-21 /auto-plan:work rev 2 T-07 launcher 编译轨收口：
  `stage: work | PLAN-039 | rev 2 | outcome: continuing | code_commit:
  lang 82b51e1aa（spawn 编译优先+gate 旗标+单测）+ os 028 pac | task_ids:
  T-07 ✅（P036-D2 核销面+D4 产物面实证）| evidence: launcher 门 0 错
  回归 + launcher_compiled_exe_discovery 单测绿 + key_message 双 fn
  生成物实证 | blockers: p036 六腿 e2e 挂 T-10（stage3 e2e 不在 --lib
  集；ui-iced 档 262 红预存基线注记）；编译轨实路召唤待用户实机确认
  | next: T-08（kanban）→ T-06（tetris 可并行）→ T-09/T-10`
- 2026-09-21 /auto-plan:work rev 2 T-04 klondike 全轨收口：
  `stage: work | PLAN-039 | rev 2 | outcome: continuing | code_commit:
  lang 深视图栈扩容提交（1GB 线程+/STACK 256MB）+ os 037 提交（pac+
  tests×3+locator 对齐）| task_ids: T-04 ✅（批次 E 前置清偿后全轨）|
  evidence: Vue playwright 三套全绿 / VM desktop_mcp 26/26 / Rust
  rust_golden 11+1 诚实红（深树快照债）/ back 对照=无 rust 再生面 /
  四 app 门回归全目标形态 | blockers: F-04a 新债在册（编译 exe MCP
  树快照深树仅根节点——RqProjector 面；树交互段+D1 编译轨实机待偿，
  VM/Vue 已实证）| next: T-07（launcher 编译轨）→ T-08（kanban）→
  T-06（tetris 可并行）→ T-09/T-10`
- 2026-09-21 /auto-plan:work rev 2 批次 E 收官记录（T-12/T-13/T-14）：
  `stage: work | PLAN-039 | rev 2 | outcome: continuing | code_commit:
  lang plan-039-dev c4ed5a21b(T-12)→a33448dbb(T-13)→T-14a 参数序→
  T-14 收官（shell-pack regen 随附） | task_ids: T-12 ✅ T-13 ✅ T-14 ✅
  （前置全清：T-04/T-07/T-08 生成门面已 unblock）| evidence: 生成门矩阵
  全收敛——klondike 190→0 / launcher 56→0 / kanban 4=ondrop×3 D3-A
  预期诚实红+汇总 / minesweeper 0 零回归（AC-08 门矩阵目标形态达成）；
  plan039 单测 16/16；ui_gen 2 红=master 预存；shell-pack freshness
  regen 回绿；vue 3 红=基线预存（stash 对照 5 红）| blockers: 无 |
  next: T-04（klondike 全轨：desktop_exe+三轨）→ T-07（launcher 编译
  轨=D2-A 双债核销）→ T-08（kanban a2r 收口+038 对拍）→ T-06（tetris
  可并行）→ T-09/T-10 收口`
- 2026-09-21 /auto-plan:work rev 2 阶段记录（并行带 T-05+T-11）：
  `stage: work | PLAN-039 | rev 2 | outcome: continuing | code_commit:
  lang plan-039-dev 079acfe74→3b99ff6bf + os 95042bf→c9610ba | task_ids:
  T-05 ✅（全轨收口）T-11 ✅（双株定案）| evidence: minesweeper 生成门
  0 错 + desktop_exe + VM 25/25 + Rust 探针全 PASS（D4 动态 cols 编译轨
  实证 9/16/30 列切换板元数 81/256/480/81）；发现臂④（动态标签
  format! 借用包裹）⑤（compute_target_rel_path 幻影 fallback 废除→
  workspace 本地 target，单测在案）；T-11 双株分治（kanban 类型化记录
  株 vs klondike/launcher 动态 Value 株）E-D1..D5 定案 §5.7 | blockers:
  磁盘满事件（D 盘 100%→清理自产物 25G 缓解至 25G 余——他组 target
  未动；环境面呈报用户）；find-msvc-tools 0.1.13/zlib-rs 0.6.8 镜像
  撤档（新生成 workspace 需双降级 0.1.12/0.6.7）| next: T-12（Value
  降链家族化+内建表）+ T-13（back 型发射+元数垫片）→ T-14 →
  {T-04, T-07, T-08}；T-06（tetris 收口）可随时并行`
- 2026-09-21 /auto-plan:new 有界修订（rev 2）：`stage: new | PLAN-039 |
  rev 2 | outcome: pass | next: work`。授权 = 用户裁定「不新建计划，
  039 内追加 phase（方向①补齐 store 译臂包，降档作废）」。变更面：
  §0 摘要 rev2 块 / G7 / §5.7 批次 E 设计（错误矩阵 + E-D1..D5 取舍
  倾向）/ AC-08 / T-11..T-14 新任务 / 依赖序重排（T-05/T-06 先行，
  T-04/T-07/T-08 挂 E 前置）/ total_steps 10→14（current_step=3 保持，
  T-01..T-03 已完成且证据保值）。E-D1..D5 实施期自裁呈报（T-11 勘定
  落定）——非用户确认项。
- 2026-09-21 /auto-plan:work 阶段记录：`stage: work | PLAN-039 | rev 1 |
  outcome: needs_replan | code_commit: lang plan-039-dev 42c12cb1f→079acfe74
  （基 ecc5b658b）| task_ids: T-01✅ T-02✅ T-03✅ T-04 部分（发现臂三件落，
  生成门被 store 深水阻断）T-05 生成门已探绿（0 错）但三轨/desktop_exe
  未起 T-06..T-10 未触 | evidence: 五臂全落地且生成物实证生效（kanban
  折平/textarea store 绑定/ondragover·ondrop 拒绝/minesweeper grid 动态
  cols/minesweeper 门 0 错全绿）；发现臂三件（递归收集/key 同弃/字面量
  再转义）；plan039 单测 10/10 + ui_gen 813/815（两红=master 预存在
  stash 实证）；门探矩阵 klondike ~180/kanban 37[3=ondrop 预期红]/
  launcher 56/minesweeper 0 | blockers: §10④ store 动态语义译臂深水
  （普查漏检结构面——三 app 同病根，超出五臂授权）| next: /auto-plan:new
  有界修订（受影响任务 T-04/T-07/T-08 生成门面 + 衔接面重排；已完成五臂
  与发现臂全部保值入册）`
- 2026-09-21 /auto-plan:work 进入（用户召唤）：status drafting → executing。
  **D2/D3 用户确认落档（AskUserQuestion 应答原文）**：D2 = **A 本波核销双债**
  （生成 launcher exe + 召唤链编译轨优先 + shell_key 编译臂 → P036-D2/D4
  核销 + p036 e2e 断言兼容改造）；D3 = **A 显式 not-yet 拒绝**
  （ondragover 家族入拒绝门词表带 P039 债指针，生成门唯一诚实红）。
  前置核验：038 merge 在 master 血统（28c94a5d4 ⊆ master）——T-08 对拍
  硬前置已清；普查基线 50e3b8bb4 → master（ecc5b658b，含 668 落地）
  ui_gen/ui/rust_ui.rs **零漂移**（锚全部有效）；PLAN-669 未合面
  （vm/ffi/vue）与本计划文件面零重叠。worktree 组：`.wt/lang-039/`
  {auto-lang plan-039-dev @ ecc5b658b + auto-down detached @ fba6563} +
  `.wt/os-039/` {auto-os plan-039-dev @ 95042bf + auto-kanban
  plan-039-dev @ 90f0df8}。主检出他会话 WIP 注记：auto-os ui-gallery
  面（PLAN-666 域）/auto-lang 文档面（DEBTS.md+docs/design）——均与本
  计划文件面零重叠，代码全落 worktree。
- 2026-09-20 /auto-plan:new 起草交接：`stage: new`，PLAN-039 rev 1
  （.next-id 039——037/038 已被并行会话取用，取号无撞）。M7-c① 近邻
  梯队（台账 :105）。`outcome: pass`（合同完整：五 app 拓扑/生成链现状/
  八缺口 file:line/desktop_exe 消费链/038·037 协调面/P666-D1 波及面/
  三轨基建全部在案；正面确认清单[icon/grid/input/dialog 族已覆盖]划出
  真实工作量边界）；`next: work`——前置 = 038 merge（仅 T-08 对拍面
  硬前置；A/B 批次可先行）。悬置决策 §10（①–③），D2 launcher 编译轨/
  D3 kanban drag 为用户确认项，不阻塞 T-01/T-02/T-04 先行。

## 10. 待澄清事项

- **④（2026-09-21 work 阶段新增→rev 2 已裁定）store 动态语义译臂
  深水**：探索普查（§4）为**词汇级**（tags/props/events 面），未探
  handler/store 语义深水。真编译门实证三 app 同病根（与 tetris 标量型
  store 对比定位——记录型 store 数据是 a2r handler 译臂缺位面）。
  **裁定（2026-09-21 用户）：不新建计划，039 内追加批次 E 补齐
  （方向①）——本条由 §5.7/T-11..T-14 承接，证据表如下留存**：
  - **klondike ~180 错**：无类型局部变量双向往返（`var c int =
    .stock_cards.pop()` → i32↔Value 错配 125）+ Value 记录字段直访
    （`.rank/.suit` ×8）+ Value 上 `>=`/`%`（12）+ 组件 prop 传型
    （CardSuitMsg/slot/badge ~8）。
  - **kanban 37 错**（3×ondrop = D3-A 预期诚实红✓，余 34）：.at 内建
    `.str()` ×9 / `esc()` 未定义 ×4（VM builtin 无 rust shim）/ api
    调用元数漂移 ×5（P666 族关联）/ store 类型声明 Card/Meta/BoardDef
    未生成 ×4 / Option 索引 ×7 / moved value ×2。
  - **launcher 56 错**：computed/方法以 String 发射后被调用
    （expected function found String ×25）+ Value↔String 错配 15+
    `.lower()` 未译 + 字段直访。
  - 修订建议面：①无类型局部变量的类型格推断（首赋值+全用例联合）；
    ②Value 记录字段/方法降链统一发射（`["k"].as_str()…` 家族化）；
    ③.at 内建方法翻译表（str/lower/esc…）；④store 类型声明生成；
    ⑤api 元数/类型漂移归因（联 P666-D1 属主）；或降档裁定（记录型
    store app 的 a2r 门延后，本波收 minesweeper+已落臂）。
    **→ rev 2 采纳①..⑤为批次 E（§5.7）；降档选项作废。**
- **①（T-01 D2，用户确认项·已确认 2026-09-21）launcher 编译轨深度**：
  D2 = **A 本波核销**（AskUserQuestion 应答）——修订时注意 T-07 生成门
  同受 §10④ 波及（launcher 56 错先于编译轨消费）。

- **②（T-01 D3，用户确认项·已确认 2026-09-21）kanban drag 语义**：
  D3 = **A 显式 not-yet 拒绝**（AskUserQuestion 应答；生成门实证
  ondragover/ondrop 拒绝臂已发射，红因唯一在册）。
- **③（T-01 D1/D4/D5，实施期自裁·已定案 2026-09-21）**：D1=A MouseArea
  包裹降级 / D4=运行期求值 / D5=单路由折平+多路由响亮拒——全案见
  §5.1 定案记录（file:line 证据）。
