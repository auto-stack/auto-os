---
plan_id: PLAN-039
status: drafting               # drafting → executing → execution_done → reviewed → archived
feature_name: m7c1-near-apps-a2r
author: [agent]
created_at: 2026-09-20
updated_at: 2026-09-20
plan_revision: 1
current_step: 0
total_steps: 10

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
M7-b ✅036 → M7-c① 本件）。**本轮仅规划，未授权实施**。涉及仓：
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

## 8. 执行步骤

**前置**：M7-b ✅（036）；038 merge（kanban 视觉对拍面硬前置——T-08
前到位即可；codegen 面无冲突可先行）。依赖序：T-01 → {T-02, T-03} →
{T-04, T-05, T-06 并行} → T-07 → T-08 → T-09 → T-10。lang worktree
`D:/autostack/.wt/lang-039/auto-lang`（组内 auto-down 依赖位同 036 型）；
os `D:/autostack/.wt/os-039/auto-os`（组内 auto-kanban 依赖 worktree
`.wt/os-039/auto-kanban`）。

- **T-01 [lang] 深水定案**
  文件：R（五臂锚点 §4）/view.rs（D1-B 面）/session.rs（D2 面：spawn_
  launcher_outproc + desktop_exe 消费链）。
  动作：D1/D4/D5 实施期自裁定案；D2/D3 用户确认项呈报。
  产物：`### 5.1 定案记录`（file:line 证据）。
  验证：定案完备；D2/D3 获用户确认。
  → 全 AC 前置。
- **T-02 [lang] klondike/minesweeper codegen 臂**（§5.2）
  验证：五臂之三单测绿。
  → AC-01。
- **T-03 [lang] kanban codegen 臂**（§5.2）
  验证：折平/textarea/拒绝门单测绿。
  → AC-01。
- **T-04 [os+lang] klondike 全轨**（§5.3）
  验证：生成门 + 三轨 + back 对照。
  → AC-02。
- **T-05 [os+lang] minesweeper 全轨**（§5.3）
  验证：生成门 + VM/Rust 轨。
  → AC-03。
- **T-06 [os+lang] tetris 收口**（§5.3）
  验证：rules_golden + run_matrix。
  → AC-04。
- **T-07 [lang+os] launcher 编译轨**（§5.4，依 D2）
  验证：编译轨召唤链 + p036 e2e 回归。
  → AC-05。
- **T-08 [kanban+lang] kanban a2r**（§5.5，依 D3 + 038 merge）
  验证：生成门矩阵（诚实红）+ 对拍。
  → AC-06。
- **T-09 [os] 伞形与台账**（§5.6）
  验证：manifest/README/台账/债册。
  → AC-07。
- **T-10 [lang+os] 回归收口**（§5.6）
  验证：§6 回归门全绿矩阵。
  → AC-07。

## 9. 复审记录

- 2026-09-20 /auto-plan:new 起草交接：`stage: new`，PLAN-039 rev 1
  （.next-id 039——037/038 已被并行会话取用，取号无撞）。M7-c① 近邻
  梯队（台账 :105）。`outcome: pass`（合同完整：五 app 拓扑/生成链现状/
  八缺口 file:line/desktop_exe 消费链/038·037 协调面/P666-D1 波及面/
  三轨基建全部在案；正面确认清单[icon/grid/input/dialog 族已覆盖]划出
  真实工作量边界）；`next: work`——前置 = 038 merge（仅 T-08 对拍面
  硬前置；A/B 批次可先行）。悬置决策 §10（①–③），D2 launcher 编译轨/
  D3 kanban drag 为用户确认项，不阻塞 T-01/T-02/T-04 先行。

## 10. 待澄清事项

- **①（T-01 D2，用户确认项）launcher 编译轨深度**：A = 本波核销
  P036-D2+D4（生成 exe + 召唤链消费 + shell_key 编译臂——分水岭收益，
  推荐）vs B = 仅生成 + desktop_exe 声明（普通链可用，双债留册）。
- **②（T-01 D3，用户确认项）kanban drag 语义**：A = ondragover 家族
  显式 not-yet 拒绝 + P039 债在册（生成门唯一诚实红；推荐——drag
  家族另立）vs B = 最小 no-op 臂（违 I1 静默红线，需升真语义才可行）
  vs C = 改 board.at 移除 drag（触 038 收敛面）。
- **③（T-01 D1/D4/D5，实施期自裁呈报）**：button 双击原语形态
  （A=MouseArea 包裹降级 推荐 vs B=View 原语三端涟漪）；grid 动态
  cols 运行期 vs 静态近似（minesweeper 实证恒定性定）；单路由折平
  边界（多路由显式拒）。
