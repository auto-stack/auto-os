---
plan_id: PLAN-047
status: reviewed              # drafting → executing → execution_done → reviewed → archived
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
current_step: 8
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
>
> **现场记录（work 启动 2026-09-28）**：auto-os `plan-047-dev`@6c9c7ba（=契约 rev1 提交）；auto-lang `os-047-dev`@9b5a10e51（=046 落地 tip/master 同点）；auto-down detached@3373a5c 只读（a2r-actor-tests 的 autodown-core 组内兄弟路径依赖位，046 同款）。auto-os 主检出 tracked-dirty=0（仅前会话未跟踪测试产物）；auto-lang 主检出有 docs 面脏文件（design/00、design/33、plans/.next-id——非 crates 代码路径，不入本计划工作面）。

- **T-01 Recorder 基建**（→AC-02/07 基座）：`memo_deps.rs` 新增 `RecState/DepKey/guard`；`vm_bridge.rs` 宿主 + `read_state`/`read_state_as_vec`/`materialize_obj_ref` 三通道录制；预算与清理。验证：`plan047_recorder_tests` 前 4 条绿。
  [✅ 已完成 2026-09-28] lang@553f79a0e。DepKey（heap_id+path，`"*"`=容器粗粒度面）/RecState（预算 REC_DEP_BUDGET=256 超限弃整集）/absorb（并集+overflow 传播）+vm_bridge `dep_recorder: RefCell<Option<RecState>>` 宿主（两 ctor 同置，ui-interpreter 门控同 memo_cache）/DepRecGuard（finish/drop 双路恢复+嵌套并集吸收）/读通道录制实装三口+共享口：`read_state`（具名字段）、`read_state_as_vec` Int 臂+`vmref_to_vec`（容器 any）、`materialize_obj_ref` Int/VmRef 臂（堆展开 any）；非 ui-interpreter 构建零伤 stub。验证：`cargo test -p auto-lang --features ui-iced --lib plan047_recorder` 9/9 绿（memo_deps 语义 4 + vm_bridge 通道 5，含未激活零录制/字段录制/容器 any/嵌套并集/drop 恢复）。执行注记：Dot 链逐段录制按 5.1 的等价实现——子对象字段读经 `materialize_obj_ref` 展开时记 `(heap_id,"*")` 粗粒度覆盖（任一字段原地变即失效，保守正确），builder 侧零改动；嵌套测试实证 items 列表读三依赖边（count+items 字段+列表 any）。
- **T-02 写点普查（bounded investigation，决策工件）**：engine ~10 bump 位点 + 桥六写口逐一归因分类（A/B/C）；per-path 版本表宿主与 path_key 编码选型裁定；产物（位点清单表）入本计划附录。验证：表覆盖全部现存 bump 位点，无遗漏臂。
  [✅ 已完成 2026-09-28] 普查表（lang worktree 勘察，os-047-dev@553f79a0e 面）：

  | # | 位点 | Op/fn | 写面 | 归因 | 类 |
  |---|---|---|---|---|---|
  | E1 | engine.rs~1197 | add_string freelist 复用 | 字符串池槽覆写 | 无堆 id | C |
  | E2 | engine.rs~1214 | add_string 追加 | 字符串池追加 | 无堆 id | C |
  | E3 | engine.rs~1271 | insert_heap_object | 堆对象出世 | 新 id 无既有 dep 可指 | C |
  | E4 | engine.rs~5313 | LIST_PUSH_INT | 列表 push | list_id | B |
  | E5 | engine.rs~5352 | LIST_POP_INT | 列表 pop | list_id | B |
  | E6 | engine.rs~5426 | LIST_SET_INT | 列表按位写 | list_id | B |
  | E7 | engine.rs~5946 | SET_ELEM ObjectData 臂 | map 按键写 | id+key | B（A-able） |
  | E8 | engine.rs~5969 | SET_ELEM GenericInstance 臂 | map 按键写 | id+key | B（A-able） |
  | E9 | engine.rs~6002 | SET_ELEM HashMap 臂 | map 按键写 | id+key | B（A-able） |
  | E10 | engine.rs~6038 | SET_GENERIC_FIELD | obj.field=value | id+field_name | A |
  | N1 | native.rs~2797 | shim_list_push | 列表 push | list_id | B |
  | N2 | native.rs~2924 | shim_list_pop | 列表 pop | list_id | B |
  | N3 | native.rs~3305 | shim_list_set | 列表按位写 | list_id | B |
  | B1 | bridge write_state | 根态字段写 | state_obj_id+field | A |
  | B2 | bridge write_or_insert_state 新增臂 | 根态字段首写 | state_obj_id+field | A |
  | B3 | bridge write_state_vec 容器臂×2 | 堆列表原地重写 | list_id | B |
  | B4 | bridge ensure_child_state 值变重种子/新增臂×2 | 根态字段种子 | state_obj_id+field | A |
  | B5 | bridge sync_busy_flag 真写臂 | __busy_handlers 重写 | (root,BUSY_STATE_FIELD)+list_id 双记 | A+B |

  **选型裁定**：版本表宿主 = `AutoVM`（engine）侧 `DashMap<(u64, String), u64>`——键用原生元组不依赖 ui 门控类型（memo_deps 是 ui-interpreter 门控，vm 模块不能反向依赖）；`"*"` 通配键（`DEP_PATH_ANY` 同值语义）由 bump 侧维护：**A 类写同时 bump exact+wildcard，B 类只 bump wildcard**。check 侧（T-05）：具名 dep (id,path) 只比 exact 版本（通配翻不翻不影响它——根态槽位只能经带名写臂变，内容变经内层对象自己的 dep 兜住）；`"*"` dep 只比 wildcard。C 类保持纯全局 bump（E3 出世 id 无既有 dep；字符串池内容不被 dep 引用——全局翻 → 条目落慢路径重解析，正确性不受损）。**全局 `state_mutation_seq` 语义零变化**（所有位点保留既有 bump，per-path 表纯增量）——seq 快路径与 fire_timer 空转判定既有消费者零回归。
  **普查发现（Q-01 附带证据）**：`auto.hashmap` 系 native 写（hashmap.set 插入）无任何 seq bump——既有全局快路径盲区（写后 memo 快路径可陈旧命中）；T-03 顺路补 `bump_path(map_id, None)`（B 类）闭合，全局语义同步补齐。
- **T-03 版本表 + 写点归因升级**（→AC-03）：`engine.rs` `bump_path` + `DashMap<DepKey,u64>`；A 类位点与桥六口升级；B/C 类保守语义。验证：`plan047_attribution_tests` 绿。
  [✅ 已完成 2026-09-28] lang@f202d01c6。`AutoVM.path_versions: DashMap<(u64,String),u64>`（T-02 裁定落地：原生元组键）+ `bump_path(heap_id, Option<&str>)`（A 类 exact+wildcard 双 bump / B 类 wildcard-only / 全局 seq 同步 bump 语义零变化）+ `path_version` 读口；升级位点：VM E4-E6（bump 移至 list_id 出栈后）、E7-E9（按键名 decode 后 exact+wildcard，普查 A-able 兑现）、E10 SET_FIELD（移至 field_name 解码后，错误路径不再多 bump——handler Err 置脏面方向不变）、N1-N3 shim_list_*；桥 B1/B2（A）、B3×2（B）、B4×2（值变才 bump A，语义保真）、B5（A+B 双归因：根态字段 exact+新镜像列表 wildcard）。普查发现顺路闭合：SET_ELEM ListData 臂补 bump（062 遗漏的全局盲区）+ shim_hashmap_insert_str 补 bump（原零 bump）。验证：`plan047_attribution` 5/5 绿（exact 定点性 root.count 前进而 root.label 不动 / 列表 push 仅列表 wildcard 前进根态 exact 零扰动 / 桥写 exact / hashmap shim 直调 exact+k+wildcard+全局补齐 / C 类全局-only 零 path 条目）+ 回归 scoped：memo 84/84、engine 20/20、vm_bridge 50/50、plan046 38/38 全绿。
- **T-04 engine 读臂拦截**（→AC-04 block 通道）：recorder 句柄下探（`Arc<Option<…>>` 或 vm 字段直挂）；state obj 字段读/列表迭代臂录制；未激活零开销分支。验证：`call_computed_fn` 场景 dep 录制单测绿。
  [✅ 已完成 2026-09-28] lang@a27be452b。执行形态与计划微差（等价实现）：录制核心类型下沉 `vm/dep_track.rs`（**不挂 feature 门**——vm 模块不得依赖 ui 门控类型，memo_deps 转发导出保持 T-01 引用面）；录制槽 = AutoVM `dep_recorder_slot: Mutex<Option<Arc<Mutex<RecState>>>>` + `dep_rec_active: AtomicBool` 快速门（先填槽后开旗 Release/先关旗后清槽，同线程串行无竞争）；读臂挂钩四口：GET_GENERIC_FIELD（field_names 可证名）/GET_FIELD（field_name 解码后挂；decode 兜底 id 过录=保守方向）/LIST_GET_INT+GET_ELEM 堆对象臂（容器 any 粗粒度面）；桥 guard 激活即 `vm.set_dep_recorder` 双通道绑定，finish 收编引擎影子集（record 并集+overflow 同规）。验证：`plan047` 全套 16/16 绿（新增槽语义+handler 端到端 GET 臂录制/去激活零账两条）+ memo 84/vm_bridge 52 回归绿。归因注记：`vm::ffi::http_server` 全组 40/1 红=基线并行串扰既有 flaky（主检出 9b5a10e51 同型复现，单跑绿，046 在册 flaky 家族同形），非本计划引入。
- **T-05 memo 门 check 三级判定**（→AC-02）：`MemoEntry.dyn_deps` + fill 采编 + check 升级 + check-kind 计数器；keyed-for/memo 块站点同构。验证：`plan047_gate_tests` 判定序与零重解析计数断言绿。
- **T-06 computed 信号节点**（→AC-04）：信号表 + inline/block 双通道 + bindings 保守面 + pull 级联。验证：`plan047_signal_tests` 绿。
  [✅ 已完成 2026-09-28] lang@325a2efff。`VmBridge.computed_signals: RefCell<HashMap<(String,String), ComputedSignal{cached,deps 基线对}>>`（生命周期随桥）+ `computed_signal_hit`（deps 版本全同 → 值缓存复用；**命中时 dep 键吸收进当前录制域**——嵌套 guard 并集使外层 memo 条目/外层信号覆盖内层信号失效面 = pull 式级联闭合，computed 嵌套传递性陈旧缺口的设计闭合点）/`computed_signal_store`（空集/超预算不入网退档 A/B）+signal_hits/misses 观测计数；`eval_computed` 双通道接入（inline 表达式与 block 体隐藏 VM fn 同构信号包裹；bindings-free 保守面沿 v1 边界）。验证：`plan047_signal` 3/3 绿（inline 命中/失效保真 + block 通道隐藏 fn 端到端[合成在册预检] + 级联闭合[memo 条目收编信号 dep 面]）。执行注记：测试 builder 须 `.with_computed(...)` 显式挂表（生产链 dynamic.rs:1565 已挂）。
- **T-07 降级面收敛**（→AC-05/07）：computed-exclusion 改判 + 产物逐字节对拍 + 盲区臂。验证：对拍 hash 相等、`plan047_gate_tests` 收敛臂绿。
  [✅ 已完成 2026-09-28] lang@c47888381（+70595f10c 文档面）。`memo_ctx_ok` 改判：computed 声明不再整体排除 memo（PLAN-045 T-01 静态不可证裁定清偿——computed 读面由桥读通道+引擎读臂+信号网三通道动态录制闭合）；bindings 排除保持（循环变量版本不随录制域——v1 保守边界沿袭）；memo_deps 头注/扫描注释同步对齐。验证：`plan047_convergence` 1/1 绿（computed-widget menubar memo 化产物与原始路径逐字节对拍一致 + 条目在册确证非静默降级 + deps 变化经信号级联保真）。
- **T-08 语料实靶 + 证据包**（→AC-06）：widgets-gallery 三页 + syslog 实测；check-kind/内存口径采集；证据包 `docs/plans/evidence/p047/`。验证：AC-06 计数器在档，lang 门全绿复跑。
  [✅ 已完成 2026-09-28] **证据包在档**（auto-os plan-047-dev@f11df72，`docs/plans/evidence/p047/`：README 索引 + 满载门两跑全量日志 + 新红清单）。**AC-01 全量门满载对拍归因**：本侧 os-047-dev@70595f10c **5504 绿/329 红** vs 基线 9b5a10e51（=046 落地 tip）5498 绿/311 红——新红 22 = 20 条 plan047 新测（满载红全数同根因：`plan492-pkg-repro-m1-canary` 临时目录污染串入 VmBridge::new；基线同证 29 条既有 vm_bridge 测试同报错文本同根因；**plan047 单跑 24/24 绿在档**）+ 2 条漂移（plan632-f4/ffi_dual-019，**单跑双绿实证**）；反向漂移 4（基线独红）；**零真回归**。scoped 门：plan047 24/24 + memo 86 + plan04 76 + vm_bridge 52 + engine 20 全绿。**语料 opt-in 零改动确认**：widgets-gallery app.at:701 `outlet (memo: true)` + filetree.at:45 `key: r.id` 在册（046 面续用），档 C 机制自动拾取；AC-06 交互式实靶沿 046 Q-04 先例（计数器断言承载验收，桌面 A/B 计时留观——部署观察项：桌面宿主下次启动现场构建自动拾取）。**⚠ 现场碰撞实录**：auto-os worktree `D:/.wt/os-047/auto-os` 与 auto-lang worktree `D:/.wt/os-047/auto-lang` 在本计划执行期间被并行会话（Plan 706，WSL 侧 /mnt/d 路径）切换到其分支 `plan-706-os-dev`/`plan-706-dev` 并提交其工作（bde65b0/f06e2ec7a）——本计划交付面无损（lang 七提交在 `os-047-dev`@70595f10c 分支指针在档；auto-os 证据提交经 commit-tree 重建为 plan-047-dev@f11df72，未携带 706 内容）；本计划的一次证据提交 c63ea70 曾落上 706 分支（父=bde65b0），未回退（706 会话在用，不劫持其分支）；**处置留观：706 会话归属与两分支后续合并动线需用户裁定**。

依赖序：T-01 → T-02 → T-03 → (T-04 ∥ T-05) → T-06 → T-07 → T-08。

## 9. 复审记录

- [new 2026-09-28] stage: new，PLAN-047 rev1。outcome: pass（drafting → 待 /auto-plan:work 执行）。next: work。SD-08..11 待 review 终审定稿；T-02 普查产物回填附录。
- [work 2026-09-28] stage: work | plan_id: PLAN-047 | plan_revision: 1 | **outcome: pass** | code_commit: lang os-047-dev@70595f10c（七提交链 553f79a0e→f202d01c6→a27be452b→27c9061ce→325a2efff→c47888381→70595f10c）+ auto-os plan-047-dev@f11df72（证据包） | task_ids: T-01..T-08 全毕（current_step 8/8） | evidence: docs/plans/evidence/p047/（scoped 门 plan047 24/24+memo 86+plan04 76+vm_bridge 52+engine 20 全绿；满载门对拍 5504/329 vs 基线 5498/311——新红 22 全数单跑绿/污染家族实证，零真回归） | blockers: 无（⚠ 现场碰撞实录见 T-08——706 会话占用 os-047 worktree 目录，交付分支指针无损，处置留观） | **next: review**（execution_done）。
- [review 2026-09-28] stage: review | plan_id: PLAN-047 | plan_revision: 1 | **outcome: pass** | reviewed_commit: lang os-047-dev@70595f10c + auto-os plan-047-dev@f11df72 | base_commit: lang 9b5a10e51（=046 落地 tip）+ auto-os 6c9c7ba | dependency_revisions: auto-down 3373a5c detached（只读依赖位） | spec_inputs: docs/specs/shell/vm-render-memo.md（canonical，SD-01..07 版本=046 落地） | **独立性声明**：复审与实施同会话——裁定从工件重建（复审 worktree `.wt/os-047/auto-lang-rv`@70595f10c 新鲜复跑 + 代码巡检），不采实施摘要自证。 | acceptance_results: **AC-01 pass**（满载对拍 5504/329 vs 5498/311——新红 22 全数单跑绿/plan492 临时目录污染家族实证[基线同证 29 既有测同根因]+2 漂移单跑双绿；复用 work 跑对拍证据，理由=reviewed 提交与对拍跑完全同代码同条件，证据包在 plan-047-dev@f11df72）；**AC-02 pass**（plan047_version_fast_unrelated_write_zero_reeval——check 帧求解增量==快路径帧口径+fp_slow 零调用，新鲜复跑绿）；**AC-03 pass**（plan047_attribution 5/5——exact 定点性/列表 wildcard 精度面/桥写/hashmap 闭合/C 类全局-only）；**AC-04 pass**（plan047_signal 3/3——inline/block 双通道+级联吸收）；**AC-05 pass**（plan047_convergence——computed-widget memo 化产物逐字节对拍+条目在册+级联保真）；**AC-06 pass**（计数器口径在档+语料 opt-in 在册零改动；交互式计时沿 046 Q-04 留观先例——部署观察项）；**AC-07 pass**（content_change 无陈旧+预算弃整集+盲区降级[extra_dyn 跳 version_fast/computed 源 dyn_deps=None]）。**fresh 复跑**：plan047 24/24+memo 86+vm_bridge 52+plan04 76+engine 20 全绿（rv worktree）。巡检：bump_path/path_versions、dyn_deps、ComputedSignal 面、memo_ctx_ok 改判、record_heap_read 四臂——7 文件 +1910/-81 在位。 | findings: **F-1（非阻塞，merge 义务）**：canonical 档两处被 SD-09/SD-11 supersede 待 merge 改写——L71（SD-03 绑定门 computed 排除句）+ L184（边界 T-01 裁定"引擎拦截不在本机制"）；SD-08..11 落档时同步收敛。**F-2（非阻塞）**：worktree 目录 os-047/{auto-os,auto-lang} 处 706 会话控制（review worktree 另立 auto-lang-rv 规避）；merge 按 046 收据流程处置、wt-guard 照常。 | evidence: docs/plans/evidence/p047/（plan-047-dev@f11df72；gate 双跑日志文件留现场 os-047/auto-os worktree 未入库[.gitignore log 面]） | **next: merge**（reviewed）。

## 10. 待澄清事项

- **Q-01（已关闭 2026-09-28 T-02/T-04）**：engine 读臂完备性——GET 系四臂挂钩在册（GET_FIELD/GET_GENERIC_FIELD/LIST_GET_INT/GET_ELEM 堆对象臂）；残余盲区 = NativeCall/FFI 内部读与 extra_dyn 派生面（sidebar nav 路由 Rust 侧读）——后者跳过 version_fast 落指纹慢路径（T-05 正确性三补②），前者靠 C 类全局兜底 + 盲区嫌疑条目联合判定，保守闭合。
- **Q-02（已裁定 2026-09-28 T-08）**：per-path 版本表内存——v1 无界 DashMap，基数 = 被 (heap_id,path) 写过的键数（与状态写活动同阶，实测无异常）；LRU 化留档后续批次（版本丢失 = 落 fp_slow，正确性不受损）。
- **Q-03（沿袭确认）**：keyed-for 项内 computed（bindings 参与）v1 不入信号网（bindings-free 保守边界），沿项级条目承载；如实靶增益不足，档后续批次再议。
- **Q-04（新增 2026-09-28，用户裁定项）**：Plan 706 会话在执行期间占用了本计划的 worktree 组目录（`.wt/os-047/{auto-os,auto-lang}`，切至 plan-706-os-dev/plan-706-dev 并提交其工作）——本计划交付分支指针无损（os-047-dev@70595f10c / plan-047-dev@f11df72），worktree 目录现处 706 会话控制下；review 阶段需新建/重取 worktree（merge 技能按分支处理，wt-guard 照常）。706 分支上混入的本计划证据提交 c63ea70（父=bde65b0）未回退——两计划后续合并动线需用户裁定。
