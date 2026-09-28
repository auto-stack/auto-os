---
plan_id: PLAN-047
status: drafting               # drafting → executing → execution_done → reviewed → archived
feature_name: VM 渲染 memo 档 C（动态读拦截 + per-path 版本 + computed 信号网）
author: [zcode-agent]
created_at: 2026-09-28
updated_at: 2026-09-28
plan_revision: 1

# /auto-plan:review 结束时填写：
supersedes_spec_components: ["docs/specs/shell/vm-render-memo.md"]
new_spec_components: []
touched_goals: []             # 引用 docs/specs/goals.md 的 GOAL-NNN

affects: [auto-lang/ui-render, auto-lang/vm, widgets-gallery]
current_step: 0
total_steps: 8
---

# [PLAN-047] VM 渲染 memo 档 C（动态读拦截 + per-path 版本 + computed 信号网）

## 0. 变更摘要

- 新增：**动态依赖录制**（信号录制器 Recorder）——memo 门激活期间，状态读经桥读通道与 view-builder 读通道自动记录 `(state_obj_id, field_path)` 依赖边；静态扫描证明不了的动态读面（Call/computed/深容器）首次由**运行时事实**覆盖。
- 新增：**写点字段归因 + per-path 版本表**——engine 突变臂与 Rust 桥写口从"只 bump 全局 seq"升级为带 `(heap_id, path)` 归因的定点 bump；清偿 PLAN-045 T-01 裁定的"版本表不完备（容器原地突变无字段归因）"欠账。
- 新增：**computed 信号节点**——`.at` computed 属性升级为信号网节点（缓存值 + 动态 dep 集 + 脏传播），inline 表达式与 block 体隐藏 VM fn 双通道接入；computed 每帧重算痛点闭合。
- 升级：**memo 门 check 路径**——条目带动态 dep 集时走纯版本比对命中（`version_fast`，零重解析）；既有静态扫描+值指纹慢路径保留为降级回退。computed 上下文"整体不 memo"排除解除（降级面收敛）。
- 扩展 canonical 档 `docs/specs/shell/vm-render-memo.md`（SD-08..11）。

## 1. 目标

**背景**（沿 PLAN-045/046 archived 实证）：档 A/B 落地组件实例级/页级/keyed-for 项级/memo 块缓存后，切页 8.9~26.4s → ~2ms。遗留结构性出血点：
1. **慢路径本身就是重求值**——每次 check 帧对全部读槽经 `resolve_expr_to_value` 逐表达式重解析再指纹比对；条目越多、读槽越宽，check 成本线性涨（正是"变慢"上限的来源）。
2. **静态降级面 = 动态读面盲区**——Call/FStr/块体 computed/容器深读等 VM 代码形态静态不可证 → 整体不缓存；档 B 只能靠语料作者显式 `memo (deps: …, exact)` 手工签名兜住（charts path、codeblock 高亮）。
3. **computed 是 memo 禁区**——widget 声明 computed → builder 上下文整体不 memo（`memo_deps` 扫描裁定），computed 本体每帧重算。
4. **写点无字段归因**——任何写都翻全局 seq，粗粒度快速路径在活跃写场景（如 fire_timer 周期写、拖拽高频写）退化。

**目标**：
1. 框架层新增动态依赖录制：memo 门/computed 求值激活期间，读状态即录依赖；录制盲区保守降级。
2. 写点归因 + per-path 版本表：engine 突变臂与桥写口定点 bump；无关 path 版本不动。
3. computed 信号节点：动态 dep 发现 + 值缓存 + 脏传播（写→dep→信号→消费条目）。
4. memo 门 check 升级 `version_fast`：动态 dep 集版本全同 → 零重解析命中；降级面收敛（computed 排除解除）。
5. 实靶：widgets-gallery（charts/codeblock/filetree）、syslog 千行流——check 帧重解析计数归零 + per-site 计数器口径验收。

**非目标**：
- 不做订阅推送式渲染调度（仍 pull-per-frame；信号网改变的是"命中判定的成本与覆盖面"，不是"何时重绘"）。
- 不动 Vue 轨；不做 release 构建治理；不改 memo opt-in 默认（非 memo 语料零行为零开销延续）。
- 不改 `.at` 语法（档 C 无 parser/lexer 面变更；computed 语法既有）。
- 带 bindings（循环变量）的 computed 求值不入信号网（v1 保守边界，keyed-for 项内 computed 沿项级条目承载）。

**受影响仓库**：auto-lang（`crates/auto-lang/src/ui/` + `crates/auto-lang/src/vm/`，改 crates → 允许在 auto-lang 跑 cargo t）；auto-os（widgets-gallery/syslog 语料验收 + 证据包，无语料语法改动、仅 opt-in 维持/微调）。

**约束**：
- **非 memo 零行为零开销红线**（045 用户硬性要求延续）：recorder 未激活分支直落原始路径；非 memo 语料产物与现状逐字节一致，269 测试基线零新增红。
- **正确性下限沿袭**：memo/信号错误只允许"变慢"，绝不允许"显示陈旧"。版本表盲区（拦截不到的读面）一律保守降级回既有路径。
- **机制单源不变**：扩展 `memo_deps.rs` / `aura_view_builder.rs` memo 门 / `vm_bridge.rs` 宿主 + `vm/engine.rs` 写读臂，不另起第二套缓存。
- 全局 episode 指纹（theme_epoch/menubar_open/popover_open/action_config）保留不动——信号网不替代它。
- debug 构建验收；计数器断言口径（046 Q-04 先例：单测计数器替代交互式计时）。

**成功样貌**：含动态读面（Call prop、computed、深容器）的 memo 子树在无关写帧下 check 零重解析（version_fast 计数）；含 computed 的 widget 上下文可 memo 且产物与非 memo 逐字节一致；engine 无关 path 写不翻条目版本；全部既有测试不红。

## 2. 架构方案

```
读面（求值期，recorder 激活时录依赖）
  桥通道    VmBridge::read_state / read_state_as_vec / materialize_obj_ref
  builder   resolve_expr_to_value(Ident/Dot→read_state 臂) / eval_computed
  engine    VM fn 执行期堆读（state obj 字段读/列表迭代）——recorder
            激活位经 Arc 句柄下探（block computed call_computed_fn 场景）
        │
        ├─ Recorder（VmBridge RefCell 宿主，帧级 guard 激活）
        │    记录 (state_obj_id, path_key) → 当前站点集
        │    未激活 = Option 分支直落（零开销红线）
        ▼
写面（写点归因）
  engine 突变臂（set 系/列表写/对象写/字符串池——T-02 普查清单）
  Rust 桥六写口（write_state/write_or_insert_state/write_state_vec/
  sync_busy_flag/ensure_child_state）
        │
        ├─ per-path 版本表：path_key=(heap_id, path) → u64 版本
        │    归因可写 → 定点 bump；归因不了 → 全局 seq 兜底 bump
        ▼
信号网
  memo 条目（site 1..8）：fill 时收编动态 dep 集 → check 先版本比对
  computed 信号节点 (widget_name, prop_name)：值缓存 + dep 集 + 脏传播
        │
        └─ 盲区/异常 → 降级回既有静态扫描+指纹慢路径 → 原始路径
```

## 3. 技术栈

- auto-lang（Rust，crates/auto-lang）：`ui/memo_deps.rs`（Recorder/版本表/信号节点）、`ui/vm_bridge.rs`（宿主 + 写口归因）、`ui/aura_view_builder.rs`（check 门升级 + computed 信号接入）、`vm/engine.rs`（写臂归因 + 读臂拦截钩子）。
- 验证：`cargo test -p auto-lang --features ui-iced --lib`（lang worktree）；auto-os 侧 widgets-gallery/syslog 桌面实靶（MCP acceptance :9471 + per-site 计数器）。

## 4. 需求分析与背景调查

**授权记录**：用户 2026-09-28 会话明示"继续推一档"（即 PLAN-045/046 非目标清单里的"档 C：全量细粒度响应式（computed 信号网）"），授权起草本计划。执行授权走 `/auto-plan:work` 流程；允许仓库 auto-lang（crates/ 改动 → AGENTS Category A 红线不适用）与 auto-os（语料验收）；debug 构建口径；无用户特别指定的自动续跑限制。

**代码事实**（2026-09-28 主检出勘察）：
- canonical 档：`auto-os docs/specs/shell/vm-render-memo.md`（SD-01..07，档 A/B 全量契约）。
- `memo_deps.rs`（1412 行）：MemoKey/条目/LRU、骨架指纹、静态扫描（scan_static/scan_for_item_body/scan_memo_block_body）、值指纹（4096 预算）、episode 指纹、SiteCounts。
- `aura_view_builder.rs`（22616 行）：组件 memo 门（~5190-5366）、raw 包装 check 路径（~20181+，慢路径 `resolve_expr_to_value` + `fingerprint_value`）、`eval_computed`（inline resolve / block 体 `call_computed_fn` 回退，~11634）、`resolve_expr_to_value`（Ident/Dot/store 别名/bindings→computed→read_state 链，~11658）。
- `vm_bridge.rs`（4290 行）：`read_state`(~678)/`read_state_as_vec`(~853)/`write_state`(~720) 系六写口（已补 bump 全局 seq，~764/788/998/1028/1306/1312/1555）、`call_computed_fn`(~1823，AutoTask 同步驱动)、`with_memo_cache` 宿主。
- `vm/engine.rs`：`state_mutation_seq: AtomicU64`（~401/757）；bump 位点 ≈10 处（字符串池 intern ~1197/1214、insert_heap_object ~1271、VM set/列表写臂 ~5313/5352/5426/5946/5969/6002/6038）；堆 = `heap_objects: DashMap<u64, Arc<RwLock<dyn HeapObject>>>`（~385）。**全部 bump 无字段归因**——per-path 版本表的写点普查是 T-02 的决策工件。
- computed 现状：`AuraComputed { name, expr }`（aura/types.rs ~731）；block 体经 handler_codegen 合成隐藏 VM fn `__computed_<Widget>_<Prop>`；含 computed 的 widget 上下文整体不 memo（memo_deps 扫描裁定，canonical 档 SD-03）。

**Spec 偏差**：无。canonical 档 SD-01..07 与代码一致（046 merge 刚读回核验）；本计划为纯扩展（SD-08..11）。

## 5. 详细设计

### 5.1 依赖录制器（Recorder，SD-08）

- 宿主：`VmBridge` 新增 `RefCell<Option<RecState>>`（与 memo 缓存同款宿主理由：渲染期 `&self`）。`RecState { deps: BTreeMap<DepKey, ()>, budget: usize }`；`DepKey = (u64 heap_id, path_key)`（path_key 为字段链的紧凑编码——字符串入 intern 表或直接 String，T-02 定）。
- 激活面：RAII guard（`with_recorder(|…|)`），memo 门 fill/check 与 computed 信号求值三处开启；guard 嵌套 = dep 并集（外层条目收编内层 computed 的 dep——脏传播链的静态基础）。
- 录制点（先桥/builder 通道，engine 通道 T-04）：`read_state`/`read_state_as_vec`（root 态字段读 → `(state_obj_id, field)`）；`materialize_obj_ref` 展开（容器读 → `(heap_id, "@vec")` 级粒度）；`resolve_expr_to_value` 的 Dot 链逐段（`.a.b` → 两条：根字段 + 子字段读）。
- **录制预算**：单次求值 dep 数上限（初值 256，超限 → 该条目弃动态 dep 集，落回静态扫描路径——宁缺勿错）。
- 盲区声明（写入 canonical 档）：NativeCall/FFI 内部读、engine 直读未挂钩臂（T-04 普查兜底前）——有盲区嫌疑的条目保留静态扫描联合判定（动态 ∪ 静态取并集进 check）。

### 5.2 写点字段归因与 per-path 版本表（SD-09）

- 版本表宿主：`AutoVM`（engine）侧 `DashMap<DepKey, u64>`（与 `state_mutation_seq` 同生命周期；桥/门经 `vm` 句柄访问）。全局 `state_mutation_seq` 保留：归因不了的写点兜底 bump（翻全局 → 全部条目落慢路径，正确性不受损）。
- T-02 普查（bounded investigation，决策工件）：枚举 engine 全部 bump 位点 + 桥六写口 → 每位点归因分类（A=可定点 `(heap_id, path)` / B=可定点到 heap_id 不可到 path（记 `(heap_id, "*")` 粗粒度） / C=归因不了（全局兜底））。产物入 plan 附录 + canonical 档。
- 归因升级：A 类位点改调 `bump_path(heap_id, path)`（同时 bump 全局 seq——全局语义不变，定点表是增量）；桥写口同构升级（`write_state(field)` → `(state_obj_id, field)` 定点）。
- 容器原地突变（T-01 欠账）：ListData/ObjectData 的 push/set/insert 写臂按 B 类记 `(heap_id, "*")`——依赖该容器的条目版本比对必失效（保守，不追求字段级）。

### 5.3 memo 门 check 路径升级（SD-11 前半）

- fill 时：门求值全程 recorder 开启 → 产物条目收编动态 dep 集（`MemoEntry.dyn_deps: Option<BTreeSet<DepKey>>`）。
- check 时三级判定（序保守）：
  1. `seq_fast`：全局 seq 未动 ∧ episode 同 → 命中（现状快速路径，不动）。
  2. `version_fast`（新增）：条目有 dyn_deps → 逐 dep 版本比对，全同 ∧ episode 同 → 命中，**零重解析**。
  3. `fp_slow`：既有静态扫描读槽重解析 + 值指纹比对（降级回退，判定序与现状一致）。
- 计数器扩展：`SiteCounts` 增 check-kind 分解（`seq_fast/version_fast/fp_slow/degraded` per-site）——验收与回归口径。
- keyed-for 项级与 memo 块站点同构接入（项级 dyn_deps 另含项值 dep：`(state_obj_id, iter_field)` + 项指纹兜底不变）。

### 5.4 computed 信号节点（SD-10）

- 信号表宿主：`VmBridge`（与 memo 缓存同池不同表）；节点键 `(widget_name, prop_name)`；节点体 `{ cached: Value, deps: BTreeSet<DepKey>, version: u64, bindings_free: bool }`。
- 求值通道：
  - inline computed：`eval_computed` 的 `resolve_expr_to_value` 臂，recorder 开启录制 → dep 集 = 录制结果；
  - block 体 computed：`call_computed_fn` 执行期 recorder 句柄下探 engine（T-04 拦截位在 VM 堆读臂生效）→ dep 集同样录制。
- 命中判定：bindings_free（求值时 bindings 为空或仅含信号无关名——保守：非空一律不入网）∧ deps 版本全同 ∧ episode 同 → 复用 `cached`；任一失效 → 重求值并刷新 dep 集。
- 脏传播：写点定点 bump 后，check 序沿条目 dep 集走版本比对即隐式传播（**不建主动订阅图**——pull 式级联，机制简单且与"只允许变慢"下限同构：漏传播 = 落重求值，不陈旧）。
- computed 值进 memo 条目：条目 dep 集自动收编所读 computed 信号的 dep（5.1 guard 嵌套并集）→ computed 失效即推条目失效。

### 5.5 降级面收敛（SD-11 后半）

- memo_deps 扫描的 computed-exclusion 臂改判：上下文含 computed → 不再整体不 memo，改"该子树条目依赖经信号网录制兜底"；block 体/Call 形态读面由动态录制覆盖。
- 产物逐字节对拍：收敛前后同语料同操作渲染产物 hash 相等（AC-05）。
- 保守边界：bindings 非空上下文（for 体内）computed 沿现状降级；信号网盲区嫌疑条目保留联合判定（5.1）。

### 5.6 规范增量

| delta_id | add/modify/retire | docs/specs/... target | before/after rule | rationale | acceptance IDs |
|---|---|---|---|---|---|
| SD-08 | add | docs/specs/shell/vm-render-memo.md | before: 读依赖仅静态扫描（动态形态降级）；after: memo 门/computed 求值期动态录制 dep 集，预算 256，盲区联合判定 | 静态降级面=动态读盲区的结构性收敛 | AC-02/04/07 |
| SD-09 | add | docs/specs/shell/vm-render-memo.md | before: 写点只 bump 全局 seq 无归因；after: A/B/C 三类归因（定点 path/粗粒度 heap/全局兜底），per-path 版本表宿主 AutoVM | 清偿 PLAN-045 T-01"版本表不完备"欠账 | AC-03 |
| SD-10 | add | docs/specs/shell/vm-render-memo.md | before: computed 每帧重算且是 memo 禁区；after: computed 信号节点（bindings-free 保守面）值缓存+dep 集+pull 式级联失效 | computed 每帧重算痛点 + memo 禁区解除 | AC-04/05 |
| SD-11 | modify | docs/specs/shell/vm-render-memo.md | before: check=seq 快路径∨指纹慢路径；after: 三级判定 seq_fast→version_fast→fp_slow；computed-exclusion 改判为信号网兜底 | 慢路径本身即重求值的成本收敛 | AC-02/05/06 |

## 6. 测试设计

- **单测（auto-lang，lang worktree 跑）**：
  - `ui::memo_deps::plan047_recorder_tests`：录制内容正确（桥读/Dot 链逐段/容器展开）、guard 嵌套并集、预算超限弃集、退出清理。
  - `vm::plan047_attribution_tests`：A 类写点定点 bump（无关 path 版本不动）、B 类容器写 `(heap_id,"*")`、C 类全局兜底、桥六写口归因。
  - `ui::memo_deps::plan047_signal_tests`：computed 信号命中/失效/pull 级联链（state→computed→条目）、bindings 非空不入网、block 体通道 dep 录制。
  - `aura_view_builder::plan047_gate_tests`：check 三级判定序（seq_fast 优先/version_fast 零重解析[resolve_expr_to_value 调用计数断言]/fp_slow 回退）、keyed-for 与 memo 块站点同构、computed-exclusion 改判产物对拍。
- **门**：`cargo test -p auto-lang --features ui-iced --lib` 全绿（含 269 基线 + p045/p046 全套件——AC-01 相对零增量口径）。
- **实靶（auto-os，debug 同构建）**：widgets-gallery charts/codeblock 页（exact memo 块场景与信号网等价性回访）、filetree（keyed-for × version_fast 叠加）、syslog 千行流（周期写活跃场景 version_fast 存活）；per-site check-kind 计数器 + dep 表内存口径；证据包 `docs/plans/evidence/p047/`。

## 7. 验收标准

- **AC-01 非 memo 零回归**：269 基线相对零增量 + p045/p046 测试套件全绿；非 memo 语料渲染产物与现状逐字节一致。验证：lang worktree `cargo test -p auto-lang --features ui-iced --lib`，门全绿 + 差集归因归零。
- **AC-02 version_fast 零重解析**：带动态 dep 集的条目在无关写帧 check 命中时 `resolve_expr_to_value` 调用计数 = 0。验证：`plan047_gate_tests` 计数断言（per-site check-kind 分解）。
- **AC-03 写点归因定点性**：A 类写点 bump 后仅依赖该 path 的条目版本失效，无关 path 版本不动；B/C 类保守语义正确。验证：`plan047_attribution_tests`。
- **AC-04 computed 信号网**：inline/block 双通道 computed 命中复用与失效重算正确；state→computed→条目 pull 级联计数断言。验证：`plan047_signal_tests`。
- **AC-05 降级面收敛等价**：含 computed 的 widget 上下文 memo 化后渲染产物与非 memo 逐字节一致。验证：`plan047_gate_tests` 产物 hash 对拍 + 实靶页目检。
- **AC-06 实靶计数器**：charts/filetree/syslog 实靶 per-site 计数器在档——无关写帧 version_fast 命中、活跃写场景正确降级不陈旧。验证：桌面实靶日志 + `docs/plans/evidence/p047/` 证据包。
- **AC-07 正确性下限守护**：recorder 盲区注入（NativeCall 内读/预算超限）自动落回既有路径，产物正确无陈旧。验证：`plan047_recorder_tests` 盲区臂 + 对拍。

## 8. 执行步骤

> 工作布局沿 046：worktree 组 `D:/autostack/.wt/os-047/{auto-os,auto-lang}`（auto-os `plan-047-dev`，auto-lang `os-047-dev` 依赖组内兄弟——改 crates 故 cargo t 在 lang worktree 跑）。每步完成追加 `[✅ 已完成]` 证据行。

- **T-01 Recorder 基建**（→AC-02/07 基座）：`memo_deps.rs` 新增 `RecState/DepKey/guard`；`vm_bridge.rs` 宿主 + `read_state`/`read_state_as_vec`/`materialize_obj_ref` 三通道录制；预算与清理。验证：`plan047_recorder_tests` 前 4 条绿。
- **T-02 写点普查（bounded investigation，决策工件）**：engine ~10 bump 位点 + 桥六写口逐一归因分类（A/B/C）；per-path 版本表宿主与 path_key 编码选型裁定；产物（位点清单表）入本计划附录。验证：表覆盖全部现存 bump 位点，无遗漏臂。
- **T-03 版本表 + 写点归因升级**（→AC-03）：`engine.rs` `bump_path` + `DashMap<DepKey,u64>`；A 类位点与桥六口升级；B/C 类保守语义。验证：`plan047_attribution_tests` 绿。
- **T-04 engine 读臂拦截**（→AC-04 block 通道）：recorder 句柄下探（`Arc<Option<…>>` 或 vm 字段直挂）；state obj 字段读/列表迭代臂录制；未激活零开销分支。验证：`call_computed_fn` 场景 dep 录制单测绿。
- **T-05 memo 门 check 三级判定**（→AC-02）：`MemoEntry.dyn_deps` + fill 采编 + check 升级 + check-kind 计数器；keyed-for/memo 块站点同构。验证：`plan047_gate_tests` 判定序与零重解析计数断言绿。
- **T-06 computed 信号节点**（→AC-04）：信号表 + inline/block 双通道 + bindings 保守面 + pull 级联。验证：`plan047_signal_tests` 绿。
- **T-07 降级面收敛**（→AC-05/07）：computed-exclusion 改判 + 产物逐字节对拍 + 盲区臂。验证：对拍 hash 相等、`plan047_gate_tests` 收敛臂绿。
- **T-08 语料实靶 + 证据包**（→AC-06）：widgets-gallery 三页 + syslog 实测；check-kind/内存口径采集；证据包 `docs/plans/evidence/p047/`。验证：AC-06 计数器在档，lang 门全绿复跑。

依赖序：T-01 → T-02 → T-03 → (T-04 ∥ T-05) → T-06 → T-07 → T-08。

## 9. 复审记录

- [new 2026-09-28] stage: new，PLAN-047 rev1。outcome: pass（drafting → 待 /auto-plan:work 执行）。next: work。SD-08..11 待 review 终审定稿；T-02 普查产物回填附录。

## 10. 待澄清事项

- **Q-01（T-02 决策工件覆盖）**：engine 读臂拦截面的完备性（NativeCall/native.rs 内部读是否可达）——普查裁定；不可达面按盲区联合判定保守兜住，不阻塞主链。
- **Q-02（T-08 度量裁定）**：per-path 版本表内存上界（千行列表 × path 基数）——实测超预算则版本表自身 LRU 化（版本丢失 = 落 fp_slow，正确性不受损）。
- **Q-03（边界沿袭确认）**：keyed-for 项内 computed（bindings 参与）v1 不入信号网，沿项级条目承载——如实靶显示增益不足，档后续批次再议。
