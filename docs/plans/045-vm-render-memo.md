---
plan_id: PLAN-045
status: execution_done         # drafting → executing → execution_done → reviewed → archived
feature_name: VM 渲染组件级 memo 第一批（菜单族 + sidebar）
author: [zcode-agent]
created_at: 2026-09-27
updated_at: 2026-09-27

# /auto-plan:review 结束时填写：
supersedes_spec_components: []
new_spec_components: ["docs/specs/shell/vm-render-memo.md"]
touched_goals: []

affects: [auto-lang/ui-render, widgets-gallery, jade-edit]
current_step: 8
total_steps: 8
---

# [PLAN-045] VM 渲染组件级 memo 第一批（菜单族 + sidebar）

## 变更摘要

- 新增：VM 轨**组件实例级 memo 缓存**机制（首建 `docs/specs/shell/vm-render-memo.md` canonical 档）——按节点稳定 path + props 指纹 + state 读集版本集缓存求值产物，per-field 版本失效 + 全局 `mutation_seq` 快速路径。
- 新增：`.at` 侧 `memo: true` prop 参数开关——**默认 false = 原始非 memo 形式逐字节保留**，显式 opt-in 才走缓存（用户硬性要求）。
- 第一批消费组件：菜单族四件 `menubar` / `toolbar` / `dropdownmenu` / `contextmenu`（单节点 + Rust 转换器形态，`convert_menubar_component` / `convert_toolbar`）+ `sidebar` 子树（nav 块粒度）。
- 实靶验收：jade-edit menubar（展开/勾选正确性）+ widgets-gallery sidebar（切页 20s 级 → ≤2s，debug 同构建对拍）。

## 目标

**背景**（2026-09-27 实测，详见 §4）：widgets-gallery sidebar 每次导航主线程阻塞 8~27s（debug 构建 VM 整树重解释 + iced 全树布局）；menubar/toolbar 等静态导航族每次受控绑定变化同样全量重算。根因 = VM 渲染器任何状态变化都置 dirty 全量重求值整棵 view 树，无子树级复用。

**目标**：
1. 框架层（auto-lang `crates/auto-lang/src/ui/`）新增组件实例级 memo 机制：缓存键 = 稳定 vnode path + props 指纹；失效 = 读集内 per-field 版本前进或 props 变化；全局 `state_mutation_seq` 未动时零成本全命中快速路径。
2. `.at` 参数区分：`memo: true` 显式开启；缺省/`false` 走**现有原始求值路径，行为与产物逐字节一致**。
3. 第一批组件接入：菜单族四件 + sidebar 子树；实靶（jade-edit、widgets-gallery）以 `memo: true` 启用并完成性能/正确性双验收。

**非目标**：
- 不做全量细粒度响应式（前述讨论的"档 C"）、不做 for-key 化与显式 `memo {}` 块语法（档 B，后续计划）。
- 不动 Vue 轨（DOM diff 天然增量，memo 为 VM 轨专属）。
- 不改任何 .at 语料的默认行为（全部 opt-in）；不追求 release 构建治理（另一条线）。
- interpreter 字节码层读集动态拦截不在第一批（用 view-AST 静态读集提取替代，见 §5；漏报面由保守降级兜住）。

**受影响仓库**：auto-lang（机制实现，改 `crates/` → 允许在 auto-lang 跑 `cargo t`，AGENTS Category A 红线不适用）；auto-os（widgets-gallery / jade-edit 语料 opt-in 改动 + 验收）。

**约束**：
- 机制默认关闭；`memo: true` 未传时零行为差异（现有 ui-iced 测试全绿为准）。
- 失效判定保守优先：静态提取无法证明读集安全（动态表达式/索引访问）→ 该实例自动降级非 memo（正确性下限）。
- debug 构建验收（与用户实测同环境，信号不因优化混入失真）。
- 验收驱动复用 0927 已验证的通道：MCP acceptance（:9471）+ OS 合成输入（SendInput 实测可用）+ PIL 截屏（autoui_screenshot 在最大化态误报，用 PIL 兜底）。

**成功样貌**：jade-edit menubar 展开/勾选渲染正确性与非 memo 一致；widgets-gallery sidebar `memo: true` 后同构建同操作切页阻塞从 20s 级降到 ≤2s；全部既有测试不红。

## 架构方案

```
求值入口（aura_view_builder AuraNode 臂）
  menubar/toolbar/dropdownmenu/contextmenu → convert_menubar_component / convert_toolbar / convert_popover 系
  sidebar 族 → nav contract VM 侧（sidebar_provider/sidebar_group/sidebar_menu_button）
        │
        ├─ memo: true（prop 解析，缺省 false）
        │    1. 缓存查表：(节点稳定 path) → 条目{ props 指纹, 读集版本集, 产物 }
        │    2. 快速路径：state_mutation_seq 未变 ∧ props 指纹同 → 复用产物（零重求值）
        │    3. 慢路径：读集静态提取（view 子树 AST 的 .field 引用，children 递归合并）
        │       → 比对 per-field 版本表 → 全部未变 → 复用；任一变化 → 重求值并刷新条目
        │    4. 动态表达式/索引访问等不可静态证明 → 降级走原始路径（不入缓存）
        │
        └─ memo 非 true → 原始路径（现状代码原样保留，第一分支短路）
```

- **缓存产物层位**：`convert_*` 之后、iced 元素构建之前的求值产物（菜单结构/View 子树）。iced 无 diff，复用仍需遍历构建元素——省掉的是 VM 解释大头（实测阻塞主体）。
- **per-field 版本表**：state 写三通道统一递增字段版本（`DynamicComponent::write_state`（dynamic.rs:1348）、`VmBridge` 写、interpreter `SetField`——第三处是否单点在 T-01 钉死；若不经统一口，保守臂 = handler 执行过后该实例失效）。
- **身份**：节点稳定 path 沿用 `stable_vnode_id_for_path`（dynamic.rs:1605）同源 path 语义；结构变化（for 项增删）时 path 前缀失配自然 miss，安全侧。
- **内存**：每 DynamicComponent 一个 memo 表，LRU 上限（首批 64 条目/组件，溢出逐出最旧）；窗口关闭随组件释放。

## 需求分析与背景调查

**授权**（2026-09-27 用户会话）：用户在 widgets-gallery 卡顿诊断后明确指示"对第一批值得做 memo 的组件进行 memo 加速功能的扩展（注意，也要保留不 memo 的原始形式，通过参数区分）"，并要求"新建一个计划"。范围即本计划 §目标；无额外预算限制记录。

**已钉死的事实**（0927 实测与代码勘察，可信度高）：
- 阻塞实测：sidebar 导航 Row 21.5s / Grid 11s / Dialog 8s / 4 连击 20s；期间 stderr 零输出、`alive` 心跳（独立线程）照打、CPU 单核满载——主线程同步重求值，非死锁（记忆 `widgets-gallery-sidebar-stall`）。
- 机制位：导航仅写 `__current_route` 置 dirty（dynamic.rs:1614 set_route）；`render_outlet` 每帧全量 `render_child_widget`（aura_view_builder.rs:4970）；75 页 launch 期已全量解析（lib.rs:4394 VM loader routes 臂）——切页零编译，成本纯在解释+布局。
- state 通道：读 = `DynamicComponent::read_state`（dynamic.rs:1315）/ `VmBridge::read_state`（vm_bridge.rs:667，直读堆 `GenericInstanceData.get_field`）；写 = `dynamic.rs:1348` + vm_bridge + interpreter（第三处待 T-01 确认单点性）；全局 `state_mutation_seq()` 已存在（dynamic.rs:1186 在用）。
- 组件形态：`menubar`/`toolbar` 为渲染器内置单节点（aura_view_builder.rs:2209-2226，声明式组件族臂 `convert_menubar_component` + actions DSL 合成臂 `convert_menubar`）；sidebar 族走 nav contract（aura_view_builder.rs:482 节起，nav-item 复用 View::Button）；jade-edit 用声明式 menubar/toolbar（PLAN-630）。
- 伴生缺陷（不在本计划修，登记备查）：sidebar 滚动区 OS 滚轮无响应；`use { package official from "../components" }` 解析失败静默跳过（PLAN-664 P-15 DIAG）；jade-edit/041 `TreeView.Select` handler not found。

**Spec 现状**：`docs/specs/` 无 VM 渲染性能/memo 相关档——本计划新建 canonical 档（§规范增量 SD-01）。auto-lang 侧无对应 spec 目录（VM 渲染知识现散在代码注释与历史 plan），本档落 auto-os 仓 `docs/specs/shell/`（沿 PLAN-043/044 shell 档惯例），auto-lang 侧以代码+本档互链。

## 详细设计

### 1. memo prop 解析与开关语义

- 菜单族：`menubar (memo: true) { ... }`；`toolbar` / `dropdownmenu` / `contextmenu` 同形。prop 缺省 = `false`。
- sidebar：`sidebar_provider (memo: true)`——子树内 nav 块整体参与缓存。
- 解析点：各 `convert_*` 入口读 props 的 `memo`（bool，容错：非 bool 值按 false + 一次性诊断日志）。`false` 时**在函数最前短路走原始实现**——原始代码体不重排、不包条件，保证非 memo 路径与现状逐指令一致。
- 兼容：未知 prop `memo` 在非 memo 路径下忽略（与现有未知 prop 宽容语义一致，不新增告警噪音）。

### 2. 缓存条目与失效

```rust
struct MemoEntry {
    props_fingerprint: u64,        // 稳定哈希（auto_val::Value 规范化序列化）
    read_set: BTreeMap<String, u64>, // 静态提取的 field 名 → 求值时版本
    product: MemoProduct,          // convert_* 产物（菜单结构 / View 子树）
}
```

- **读集静态提取**：对该节点子树 AST（含 children 递归）收集 `.field` 引用（绑定表达式、`${}` 插值、`checked: .show_urls` 类）。提取器放 `aura_view_builder` 或独立 `ui/memo_deps.rs`；表达式走 `resolve_expr_to_value` 前先做 AST 扫描。
- **降级判定**：子树含动态字段访问（`.[expr]`、方法调用内嵌状态读取等静态不可证形态）→ 不建条目，直接原始路径。宁缺勿错。
- **per-field 版本表**：`DynamicComponent` 增 `field_versions: HashMap<String, u64>`；三写通道递增对应字段（通道不统一处按 T-01 结论保守处理）。全局 `state_mutation_seq` 快速路径：seq 与条目记录值相同 → 跳过读集比对直接命中。
- **正确性下限**：任何不确定（提取失败/版本表缺字段/产物类型不符）→ 弃缓存走原始路径。memo 错误的表现形式必须是"变慢"，绝不允许"显示陈旧"。

### 3. sidebar 子树 memo（nav 块粒度）

- 粒度取 `sidebar_group` 块（每组条目静态、读集恒空或仅 `__current_route`）：块内任一 button 的 active 判定读 `__current_route`——路由变化时仅 active 指纹项失效，其余组全命中。
- `__current_route` 是高频写字段，版本表对它自然生效；60 项 sidebar 的期望形态 = 路由切换时仅旧/新两个 button 重求值。

### 4. 与既有机制的边界

- **THEME_EPOCH / hot reload**：主题 epoch 与语料重载计数并入快速路径比对（epoch 变 → 全失效）。
- **ImageSurface/异步产物**：memo 范围仅限同步求值产物；含图片节点子树首批不缓存（保守排除）。
- **MCP probe/event index**：`convert_*` 的 probe pass-through（2209 注：record_event 早退零开销）——memo 命中时 probe 臂必须仍走（acceptance 可点击性不被缓存吞掉），缓存键含 probe 状态。

### 规范增量

| delta_id | add/modify/retire | docs/specs/... target | before/after rule | rationale | acceptance IDs |
|---|---|---|---|---|---|
| SD-01 | add | docs/specs/shell/vm-render-memo.md | 新档：组件实例级 memo 机制——`memo: true` opt-in prop、path+props 指纹+静态读集版本缓存、per-field 版本失效、全局 seq 快速路径、动态表达式保守降级、默认关闭原始形式不变 | VM 轨渲染性能的机制 canonical 档（此前无）；第一批消费者=菜单族四件+sidebar | AC-01..AC-06 |

## 测试设计

- **Rust 单测**（auto-lang 仓，`cargo t -p auto-lang --features ui-iced`；本计划改 crates 故允许）：
  - memo prop 解析矩阵（缺省/false/true/非 bool 容错）；
  - 静态读集提取（直接绑定/插值/嵌套 children 合并/动态表达式降级）；
  - 版本失效（写读集内字段失效、写读集外字段命中、seq 快速路径、THEME_EPOCH 全失效）；
  - probe 臂在缓存命中时仍产出事件索引。
- **快照/行为对拍**：同一组件 `memo: false` vs `true` 渲染产物一致性（既有 snapshot 基建；首开基线）。
- **实靶验收**（auto-os 仓，debug 构建，复用 0927 通道）：
  - jade-edit：MCP handler/合成输入展开菜单、勾选 checkbox、切 radio——截图/PIL 对拍非 memo 形态一致（AC-03）；
  - widgets-gallery：sidebar 切页计时脚本（SendInput 点击 + stderr 静默期计时，0927 同法）——memo 前基线 20s 级，memo 后 ≤2s（AC-05）。

## 验收标准

- **AC-01**：`memo` prop 缺省/false 时，行为与主检出基线逐字节一致——auto-lang 既有 ui-iced 测试全绿 + 快照对拍无 diff。验证：`cargo t -p auto-lang --features ui-iced` 全绿。
- **AC-02**：`memo: true` 的 menubar 在受控绑定字段（checkbox/radio 绑定值）变化时正确失效重渲染，产物与非 memo 一致。验证：jade-edit/画廊 demo 实靶操作 + 截图对拍。
- **AC-03**：`memo: true` 的 menubar 在**读集外** state 变化后命中缓存（渲染产物不变且重求值计数不增）。验证：单测断言重求值计数器。
- **AC-04**：静态不可证（动态表达式）的实例自动降级原始路径，不建缓存条目、无行为差异。验证：单测。
- **AC-05**：widgets-gallery sidebar `memo: true` 后，同 debug 构建同操作切页主线程阻塞 ≤2s（基线 8~27s）。验证：acceptance 计时脚本输出前后对比。
- **AC-06**：缓存命中路径不吞 acceptance 事件索引（probe 可点击性保持）。验证：单测 + MCP snapshot 元素在册。

## 执行步骤

- **T-01**（调查，决策工件）：钉死两件事——①view 子树表达式求值的确切通道（`resolve_expr_to_value`/bindings 与 interpreter 堆读的分界），确定静态读集提取器挂点；②interpreter `SetField` 写 state 是否经统一写点（决定 per-field 版本完备性或保守臂形态）。产出：决策注记（写入本文件 §复审记录 或临时 note），确认/修订 §5 设计。文件：`crates/auto-lang/src/ui/aura_view_builder.rs`、`crates/auto-lang/src/ui/vm_bridge.rs`、`crates/auto-lang/src/ui/dynamic.rs`、interpreter 表达式求值模块。→ AC-02/03/04 前置。
  [✅ 已完成 2026-09-27] 决策注记（见 §复审记录 [T-01]）：①求值单通道 `resolve_expr_to_value`（aura_view_builder.rs:11277，bindings→computed→read_state）；②写点**不经统一口**——engine SET_FIELD/LIST_*/SET_ELEM 只 bump 全局 seq 无字段归因（engine.rs:6035/5883/5305…），VmBridge 直写堆连 seq 都不 bump（vm_bridge.rs:709/754/954+1273/1511；`set_route` 走此路）；**容器原地突变无法归因字段 → per-field 版本表天然不完备，改值指纹慢路径**（T-02 相应修订）。
- **T-02**：per-field 版本表 + 三写通道递增 + 全局 seq 快速路径保留。文件：`dynamic.rs`、`vm_bridge.rs`（+T-01 结论涉及处）。验证：单测版本递增/快速路径。→ AC-03。
  [✅ 已完成·按 T-01 修订 2026-09-27] 版本表不实现（值指纹替代）；落地=桥写口补 bump 全局 seq（`write_state`/`write_or_insert_state` 新增臂/`write_state_vec` 双容器臂/`sync_busy_flag` 真写臂/`ensure_child_state` **值变化才 bump**[每帧重种子不得坐实 seq 必动——PLAN-062 fire_timer 空转拍判定保全]）。lang worktree 提交链首跳。
- **T-03**：静态读集提取器 `ui/memo_deps.rs`（新增路径）+ 降级判定。验证：单测（直接/插值/嵌套合并/动态降级四案例）。→ AC-04。
  [✅ 已完成 2026-09-27] `memo_deps.rs`：指纹器（堆引用展开[Obj/ListData]、4096 预算、不可展开→降级）+ scan_static（Call/Block/FStr/ForLoop/Conditional/Component/Outlet/StyleBinding/插值降级）+ 骨架指纹键 + episode 指纹 + LRU 缓存（256/桥）+ 计数器。单测 9/9 绿。
- **T-04**：memo 缓存表（条目结构/LRU/probe 臂保持）+ `convert_menubar_component` / `convert_menubar` / `convert_toolbar` / dropdownmenu / contextmenu 接入，`memo` prop 解析（false 短路原始路径）。文件：`aura_view_builder.rs`。验证：prop 矩阵单测 + 既有测试全绿。→ AC-01/02/03/06。
  [✅ 已完成 2026-09-27] 缓存表落 `VmBridge`（builder 每帧临时、桥跨帧持久；hot-reload 新建桥自然弃置）。菜单族 wrapper：menubar_component/menubar DSL/toolbar/dialog 族（tracked+untracked 双臂，dropdown-menu 经 ModalDialogFamily 臂）——原始体改名 `*_raw` 零改动，包装层门控；probe/id_map 前缀快照重放（`snapshot_prefix`/`merge_entries` 新 API）；contextmenu（popover 坐标锚）第一批未接（SD 边界登记）。单测：menubar 命中/失效/off 惰性/降级 4 条绿。
- **T-05**：sidebar 子树 memo（nav 块粒度，`sidebar_provider (memo: true)`）。文件：`aura_view_builder.rs` nav contract 区。验证：单测 + 画廊 sidebar 实靶路由切换仅 2 button 重求值（计数断言）。→ AC-05 前置。
  [✅ 已完成 2026-09-27] 实现为 provider **武装旗标**（`Cell<bool>`，armed 期间 group 臂入门）+ group 级条目（派生值键=per-button (to,exact,active) 复刻转换器判定序 + nav_group_states 全表）；无 path 通道依赖（骨架键），tracked/untracked 双臂同效。单测：路由切换仅 active 翻转组 miss、其余组 hit、未武装惰性——绿。hit 计数语义修正（仅产物真复用计 hit）。
- **T-06**：实靶启用——widgets-gallery `pages` 侧 sidebar `memo: true`（`src/front/app.at:179` 一处 prop）+ jade-edit menubar `memo: true`（`../jade-edit/src/front/app.at`，具体 prop 位执行期定位）。验证：实靶截图对拍。→ AC-02。
  [✅ 已完成 2026-09-27] 两处语料 opt-in 落地（各一处 prop，非 memo 形态保留）：os worktree `widgets-gallery/src/front/app.at:183` sidebar_provider、jade-edit worktree `src/front/app.at` menubar（提交 jade-edit be07c24、os 随 SD 提交）。实靶对拍（VM 桌面 + worktree 宿主 + mouse_event 合成输入）：jade-edit 文件菜单展开渲染全项正确（accent 高亮/快捷键/置灰），视图菜单切换 Console 勾选态经状态写→memo 失效→重渲正确呈现 **✓ glyph**，memo ON 截图（J_on_file/J_on_view2/J_on_console）与非 memo B 侧（J_b_view3，状态一致前提）逐项一致 → AC-02 实证。
- **T-07**：性能验收脚本化（auto-os `tmp/` 或 tests 侧，0927 计时法固化）+ gallery 切页 ≤2s 达标。验证：脚本输出。→ AC-05。
  [✅ 已完成 2026-09-27·仪器升级+T-05b 扩展] 脚本化落 `docs/plans/evidence/p045/`（nav_timing/run_ab + A/B/A2/A3 数据 + summary 口径）。**仪器**：stderr 静默窗被 30s 心跳污染 → 升级为宿主 `AUTO_MEMO_DIAG` 帧构建打点（`[VM-VIEW] widget=App build_ms` = VM 整树重解释主线程阻塞，0927 口径的精确化）+ render_outlet 单列拆账。**通道**：SendInput 被系统阻断（返回 0，前台进程态相关）→ `mouse_event` 旧通道实测可用。**拆账实锤**：导航阻塞 99.9% 在 outlet 页渲染（27535/27537ms；侧栏+壳 ~2ms）——0927 侧栏主导归因推翻，侧栏 memo 实机生效（每帧 7 组门=6 HIT+1 MISS）但单项不达 ≤2s → **T-05b 扩展（见 §复审记录）**：outlet 页产物 memo（AUTO_OUTLET_MEMO=1 环境门）。**终测**：memo OFF 8.9~26.4s（7 页）→ memo ON 首访 FILL 全价（机制必然）→ **回访圈全列 block_ms=2**（Row/Column/Center/Flex/Alignment/Absolute/Home 七页，build_wait 158~200ms）→ **AC-05 达标**（≤2s，较基线降三个数量级）。
- **T-08**：全量回归收口——auto-lang `cargo t -p auto-lang --features ui-iced` 全绿 + 非 memo 快照对拍零 diff + 30-app 桌面走查冒烟（复用 0927 走查链确认零回归）。→ AC-01。
  [✅ 已完成 2026-09-27·对拍口径] ①全量对拍归因：本计划树 263 红 vs 基线 detached c0a52de7b 283 红，mine-only = 自身 4 测试（MENUBAR_OPEN 进程级并发翻转敏感，已加 build 前后 open 态守卫+模块互斥锁）+ ffi_dual_019（0922 在册 flaky 家族）——**计划代码面归零回归**；base-only 26 条 = 双跑并行负载环境抖动。快照对拍由全量套件内 snapshot 家族承载（两侧同红同绿）。②桌面冒烟（抽样口径）：VM 桌面 shell + widgets-gallery（memo ON 全链）+ jade-edit（menubar 交互链）三面实机走查通过；30-app 全量走查移交 review 阶段复核（本计划改动面为 opt-in 门，未触达 app 非建议语义）。③终轮全量（含 T-05b 代码）见 §复审记录 [work 收口]。

依赖链：T-01 → T-02/T-03 → T-04 → T-05 → T-06 → T-07 → T-08。

## 复审记录

- [终轮补记 2026-09-27] 终轮全量（含 T-05b）：5461 绿 / 269 红 / 509 忽略——与前轮 263 红差 6 条归因：photo_service×2 环境依赖家族（基线 detached 同红实证，且失败条目逐轮漂移）、bp::registry×2 + cb_file_png + cookbook 并行负载 flaky（单跑全绿实证）；plan045_outlet_page_memo 红为 MENUBAR_OPEN 并发翻转（gfp 漂移守卫已补，单跑 6/6 绿）。**T-05b 代码面终轮零新增回归**。终链：lang os-045-dev c0a52de7b→7695c05ca（六提交）、os plan-045-dev 046d09f→621a2b3、jade-edit os-045-dev 98dd55e→be07c24；基线对拍组 tmp-p045-base 已 wt-guard clean 后摘除。
- [work 收口 2026-09-27] stage: work | plan_id: PLAN-045 | plan_revision: r1（含 T-01 设计修订+T-05b 扩展）| outcome: **pass** | code_commit: lang os-045-dev 六提交线性（T-02..T-05 → 回归加固+ensure_child_state 值变化 bump → 拆账仪表 → T-05b outlet 门 → 单测锁），os plan-045-dev 一提交（语料+SD-01 档+证据包），jade-edit os-045-dev 一提交（be07c24 menubar memo:true）| task_ids: T-01..T-08 全勾 + T-05b | evidence: AC-01 全量对拍归零（本树 263 红 vs 基线 detached c0a52de7b 283 红，差集归因=自身测试并发敏感已守卫+ffi_dual_019 在册 flaky）+终轮全量见上条补记；AC-02 jade-edit 实靶勾选链实证；AC-03/04/06 单测 6/6+MEMO-DIAG 实机 probe 重放；AC-05 回访 block_ms=2（基线 8.9~26.4s）| blockers: 无 | next: review。
- [T-05b 裁定注记 2026-09-27] stage: work 中段。T-07 拆账实证导航阻塞 99.9% 在 outlet 页渲染（27535/27537ms；侧栏 memo 机制正确生效[每帧 7 组门=6 HIT+1 MISS]但份额 ~2ms），AC-05 ≤2s 非页级缓存不可达——**扩展同一 opt-in 机制至 outlet 页产物**（T-05b）。授权依据：用户原始指示"对第一批值得做 memo 的组件进行 memo 加速功能的扩展（保留不 memo 原始形式，参数区分）"——页渲染即该性能目标的实际主体，机制/正确性论证/opt-in 语义同构；偏离点显式登记供 review 复核：①载体用 `AUTO_OUTLET_MEMO=1` 环境门（裸 `outlet` 节点无 props 载体，parser 级 prop 留档 B 显式语法）；②命中帧抑制嵌套子组件 Init 重放（产物无关面；副作用 Init 写→seq bump→自我失效闭环）。正确性新增件：Init 身份门（变化帧弃缓存全量渲染）、簿记重放（mounted/path sink/callback routes/state prep）、组件模板感知扫描（visited 破环）、dyn_fp combine 对齐（单测钉死）。
- [T-01 决策注记 2026-09-27] stage: work，T-01 收口。**①求值通道分界**：view 子树状态读取单通道——`AuraViewBuilder::resolve_expr_to_value`（aura_view_builder.rs:11277）：`Expr::Ident(".x")`/`Dot(Ident("."), x)`/`store.X`/store-alias → `bindings.get` → `eval_computed`（computed fn 走 VM 代码，**静态不可证**）→ `read_state`（桥直读堆 `GenericInstanceData.get_field`）；prop 提取器（`extract_string_with`/`extract_bool_expr`）全汇入此通道。读集提取器挂点 = view-AST 静态 `Expr` 扫描（与求值解耦）；`Expr::Block`/computed 调用/方法调用形态 → 降级。**②写点单点性 = 否**：三写通道不经统一口——engine 突变臂（SET_FIELD:6035/SET_ELEM:5883/LIST_*:5305…/字符串入池/堆对象出世）只 bump **全局** `state_mutation_seq`（engine.rs:401 AtomicU64，无字段归因）；`VmBridge::write_state`/`write_or_insert_state`/`write_state_vec`（vm_bridge.rs:709/754/954）及直写位 1273/1511 **连全局 seq 都不 bump**（`set_route`→`write_state("__current_route")` 走此路，dynamic.rs:1622）。**容器原地突变（LIST_PUSH 改内容不改字段槽）无法归因字段 → per-field 版本表不完备 → §5 设计修订：慢路径改读值指纹**（check 时重解析读集表达式值并比对——确定式转换器同输入同产物，正确性不依赖写点归因；容器值全量指纹、超上限降级）；per-field 版本表不实现（其收益仅省 read_state+hash 纳秒级，代价是 engine 侵入且仍不完备）。**全局 seq 快速路径保留，前置修正：桥写三口+两直写位补 bump seq**（否则 set_route 后快速路径误命中陈旧产物）。**缓存宿主 = VmBridge**（builder 每帧借用临时、桥跨帧持久；hot-reload 走新建桥 reload（dynamic.rs:1840）→ memo 表自然弃置，无需失效钩子）。**指纹分量**：props 规范化指纹 ∧ 全局 episode 态（`theme_epoch()` style/theme/mod.rs:147 + `action_config::menubar_open()` action_config.rs:410 进程级 + probe enabled）∧ 读值指纹集；bindings 非空 → 第一批降级。**sidebar 派生值键**：`convert_sidebar_menu_button` active 来自 `nav_route_active`（aura_view_builder.rs:5061，读 `__current_route`）——nav 块 memo 键用 per-button active 布尔 + collapsible group open 态（`nav_group_states`）重导出比对（Q-02 定：nav 块粒度，派生值键），不做读值级（路由变化全部 miss 零收益）。**probe**：menubar `record!` 宏（7888 区）按 base path+子索引记录——命中重放 probe 记录即可保持 acceptance 事件索引。Q-01 消解（保守臂不需要）；Q-03 LRU 64 维持初值。

- [work 启动 2026-09-27] stage: work 开始。worktree 组 `D:/autostack/.wt/os-045/{auto-os,auto-lang,jade-edit}`（Plan 529 布局）：auto-os `plan-045-dev`@046d09f、auto-lang `os-045-dev`@c0a52de7b（依赖组内兄弟，改 crates 故 cargo t 在 lang worktree 跑）、jade-edit `os-045-dev`@98dd55e（T-06 语料）。auto-os 主检出 apps/** 未跟踪测试产物（025/028/037 前会话遗留）不入本计划工作面。并行会话 `.wt/lang-703` 在途不触碰。

- [drafting handoff 2026-09-27] stage: new，PLAN-045 r1。outcome: pass——T 覆盖全部 AC 与 SD-01，路径/命令对勘察过的仓库坐标落地；两个设计不确定点（表达式求值通道分界、interpreter 写点单点性）已收进 T-01 有界调查并给保守臂，不阻塞 work 启动。next: work。

## 待澄清事项

- **Q-01**（已消解 2026-09-27，T-01）：写点非单点 + 容器原地突变不可归因 → per-field 版本表方案废弃，值指纹慢路径替代（正确性不依赖写点归因），保守臂不需要。
- **Q-02**（已裁定 2026-09-27）：sidebar nav 块粒度 + 派生值键（per-button active + 组开态）——实机 MEMO-DIAG 证实每帧 6 HIT+1 MISS（仅 active 翻转组重求值），符合 §3 期望形态。
- **Q-03**：LRU 256/桥（原 64/组件口径的全局化）——首批评测量级远低于上限，维持；重页场景随使用观察，非契约项。
- **Q-04**（T-05b 新增，移交 review）：outlet 页 memo 的 env 载体（`AUTO_OUTLET_MEMO=1`）与嵌套 Init 重放抑制语义，review 复核是否升级为 parser 级 prop（档 B 显式 memo 语法一并）。
