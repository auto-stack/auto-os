---
plan_id: PLAN-045
status: drafting               # drafting → executing → execution_done → reviewed → archived
feature_name: VM 渲染组件级 memo 第一批（菜单族 + sidebar）
author: [zcode-agent]
created_at: 2026-09-27
updated_at: 2026-09-27

# /auto-plan:review 结束时填写：
supersedes_spec_components: []
new_spec_components: ["docs/specs/shell/vm-render-memo.md"]
touched_goals: []

affects: [auto-lang/ui-render, widgets-gallery, jade-edit]
current_step: 0
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
- **T-02**：per-field 版本表 + 三写通道递增 + 全局 seq 快速路径保留。文件：`dynamic.rs`、`vm_bridge.rs`（+T-01 结论涉及处）。验证：单测版本递增/快速路径。→ AC-03。
- **T-03**：静态读集提取器 `ui/memo_deps.rs`（新增路径）+ 降级判定。验证：单测（直接/插值/嵌套合并/动态降级四案例）。→ AC-04。
- **T-04**：memo 缓存表（条目结构/LRU/probe 臂保持）+ `convert_menubar_component` / `convert_menubar` / `convert_toolbar` / dropdownmenu / contextmenu 接入，`memo` prop 解析（false 短路原始路径）。文件：`aura_view_builder.rs`。验证：prop 矩阵单测 + 既有测试全绿。→ AC-01/02/03/06。
- **T-05**：sidebar 子树 memo（nav 块粒度，`sidebar_provider (memo: true)`）。文件：`aura_view_builder.rs` nav contract 区。验证：单测 + 画廊 sidebar 实靶路由切换仅 2 button 重求值（计数断言）。→ AC-05 前置。
- **T-06**：实靶启用——widgets-gallery `pages` 侧 sidebar `memo: true`（`src/front/app.at:179` 一处 prop）+ jade-edit menubar `memo: true`（`../jade-edit/src/front/app.at`，具体 prop 位执行期定位）。验证：实靶截图对拍。→ AC-02。
- **T-07**：性能验收脚本化（auto-os `tmp/` 或 tests 侧，0927 计时法固化）+ gallery 切页 ≤2s 达标。验证：脚本输出。→ AC-05。
- **T-08**：全量回归收口——auto-lang `cargo t -p auto-lang --features ui-iced` 全绿 + 非 memo 快照对拍零 diff + 30-app 桌面走查冒烟（复用 0927 走查链确认零回归）。→ AC-01。

依赖链：T-01 → T-02/T-03 → T-04 → T-05 → T-06 → T-07 → T-08。

## 复审记录

- [drafting handoff 2026-09-27] stage: new，PLAN-045 r1。outcome: pass——T 覆盖全部 AC 与 SD-01，路径/命令对勘察过的仓库坐标落地；两个设计不确定点（表达式求值通道分界、interpreter 写点单点性）已收进 T-01 有界调查并给保守臂，不阻塞 work 启动。next: work。

## 待澄清事项

- **Q-01**：interpreter `SetField` 若不经统一写点，保守臂"handler 执行后该实例失效"会把页内高频 handler 场景的命中率打掉多少？——T-01 实测定；若保守臂代价过高，回本计划讨论是否将读集动态拦截（interpreter 挂钩）提前。
- **Q-02**：sidebar memo 粒度（nav 块 vs 整个 sidebar_provider 子树）——T-05 执行期以画廊实测命中率定，倾向 nav 块（设计默认）。
- **Q-03**：memo 条目 LRU 上限 64/组件为初值，重页（datatable 723 行）是否够——执行期按内存实测调，非契约项。
