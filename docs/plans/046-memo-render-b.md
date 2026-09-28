---
plan_id: PLAN-046
status: executing             # drafting → executing → execution_done → reviewed → archived
feature_name: VM 渲染 memo 档 B（for key 化 + 显式 memo 块 + outlet 语料级升级）
author: [zcode-agent]
created_at: 2026-09-28
updated_at: 2026-09-28

# /auto-plan:review 结束时填写：
supersedes_spec_components: ["docs/specs/shell/vm-render-memo.md"]
new_spec_components: []
touched_goals: []

affects: [auto-lang/ui-render, auto-lang/parser, widgets-gallery, auto-os]
current_step: 6
total_steps: 7
---

# [PLAN-046] VM 渲染 memo 档 B（for key 化 + 显式 memo 块 + outlet 语料级升级）

## 变更摘要

- 扩展：`for` 列表 **key 化**（`for x in .items key: .id`）——项级缓存按 key 值复用，重排/增删/单项修改不再按位置失配全列表重求值；`key:` 缺省 = 现状 path 语义逐字节保留。
- 新增：**显式 memo 块语法**——把档 A 静态降级面中"用户可担保"的子树（含 ForLoop/Conditional/插值文本/Call 系）包进显式缓存边界，覆盖重计算场景（charts path、code_block 高亮）；默认保守语义（自动读槽 ∪ deps 并集），精确信任模式需显式声明。
- 升级：**outlet 页 memo 从 `AUTO_OUTLET_MEMO` 环境门迁升为 .at 语料级 prop 载体**（PLAN-045 T-05b 明示留待档 B），env 门保留为兼容层。
- 扩展 canonical 档 `docs/specs/shell/vm-render-memo.md`（SD-05..07）。

## 目标

**背景**：档 A（PLAN-045，archived）落地组件实例级缓存 + outlet 页级缓存，实测切页 8.9~26.4s → 2ms。遗留出血点：①`for` 大列表整体处于档 A 静态降级面（`ForLoop` 子节点不建条目）——datatable(723 行页)/filetree(681)/020 曲库/syslog 1000 行流等场景任何无关 state 写都全列表重求值；②列表重排按 vnode path 身份必然 miss；③重计算表达式（charts path、code_block 逐字符高亮）无缓存粒度可用；④outlet memo 目前仅 env 门，语料不可声明。

**目标**：
1. `for` 语法扩展可选 `key: <expr>` 子句；key 化列表的项级缓存：键集 diff（新 key 求值/旧 key 逐出/同 key 比项值指纹），命中复用项产物。
2. 显式 `memo` 块语法（块节点 + 缓存门），语义见 §5.2；保守默认。
3. `outlet` 语料级 memo prop（与 env 门并存，语料 prop 优先），widgets-gallery 迁移验证等价。
4. 实靶：widgets-gallery datatable/filetree、020 曲库、syslog、charts 页——正确性对拍 + 重求值计数/耗时前后对比。

**非目标**：档 C 全量细粒度响应式（computed 信号网）；Vue 轨；`memo` 块的依赖自动补全推断（显式语法由语料作者签名，机制只做能自动证明的部分）；release 构建治理。

**受影响仓库**：auto-lang（parser/lexer 语法扩展 + ui 渲染层，改 crates → 允许在 auto-lang 跑 cargo t）；auto-os（widgets-gallery/020/syslog 语料 opt-in + 验收）。

**约束**：
- **语法兼容红线**：无 `key:` / 无 memo 块的语料，parser AST 与渲染产物逐字节一致；既有 269 测试基线不新增红。
- 档 A 正确性下限沿袭：**memo 错误只允许"变慢"，不允许"显示陈旧"**——key 化与 memo 块的失效判定保守优先；显式 `exact` 信任模式是唯一允许"纯 deps 判定"的形态，且必须语料显式声明（文档明示责任边界）。
- 档 A 机制单源不变：扩展落 `memo_deps.rs` / `aura_view_builder.rs` memo 门 / `vm_bridge.rs` 宿主，不另起第二套缓存。
- debug 构建验收；实靶驱动复用 PLAN-045 证据包的计时脚本口径。

**成功样貌**：datatable/filetree 页在无关 state 写下重求值计数归零；列表重排仅变化项重求值；charts 页 memo 块命中；outlet 语料 prop 与 env 门行为等价；全部既有测试不红。

## 架构方案

```
ForLoop 求值臂（aura_view_builder 837/1107/2636/...）
  无 key: → 现状路径（逐项求值，原始）
  key: <expr> →
    1. iterable 求值 → 逐项求 key 值 → 键集
    2. 与上帧键集 diff：新增项求值并建条目；消失 key 逐出
    3. 存续项：项值指纹（item 值 + 循环变量展开）∧ 读槽版本（档 A 慢路径同源）
       → 命中复用项产物；miss 项重求值
    4. 产物按 key 序重排拼装（输出顺序仍跟 iterable 当前序）

memo 块（新块节点 MemoBlock）
  memo (deps: .a, .b) { ...子树... }
    默认：deps 值指纹 ∪ 子树自动读槽重解析（档 A 慢路径同源）→ 全同才命中
    exact: true（显式）：纯 deps 判定（语料作者签名担保；文档明示陈旧风险自担）
  episode 指纹（theme_epoch/menubar_open/popover_open/action_config）沿袭档 A

outlet 语料升级
  outlet (memo: true) { } → 语料 prop 优先于 AUTO_OUTLET_MEMO env 门
  env 门行为不变（既有实验/测试脚本兼容）
```

## 需求分析与背景调查

**授权**（2026-09-28 用户会话）：PLAN-045 归档交付后用户指示"继续"——即性能线按既定顺序推进档 B 新计划。范围即本计划 §目标；无额外预算限制记录。

**档 A 落地事实**（canonical 档 `docs/specs/shell/vm-render-memo.md` + auto-lang main `5c558778f`，可信度高）：
- 机制单源：`memo_deps.rs`（`scan_static_with_components` / `skeleton_fingerprint_with_components`）+ `aura_view_builder.rs`（memo 门 + `*_raw` 包装层，outlet 侧 `render_outlet_impl` tracked 三件套）+ `vm_bridge.rs`（缓存宿主，LRU 256/桥，六写口 bump seq）。
- 键 = `(ctx_state_obj, site, skeleton_fp, probe_on)`；快速路径 = `state_mutation_seq` 未动；慢路径 = 读槽表达式经 `resolve_expr_to_value` 重解析值指纹；episode 指纹 = theme_epoch + menubar_open + popover_open + action_config Arc。
- **归因修正**（PLAN-045 T-05b）：导航阻塞 99.9%（27535/27537ms）在 outlet 页渲染，sidebar+壳仅 ~2ms——档 B 的 for 场景收益同理主要在**页内列表**，不在导航壳。
- **降级面**（SD-03）：`ForLoop`/`Conditional`/`Component`/`Outlet` 子节点、插值文本、`StyleBinding`、`Expr::Call/Block/FStr/Lambda`、bindings 非空/computed 上下文——均不建条目。**档 B 的三个交付正对着这张降级表的三个豁口**。
- T-05b 明示伏笔："裸 `outlet` 节点无 props 载体，语料级 prop 留待档 B 显式 memo 语法升级"。
- 堆引用指纹规则：`expand_heap_for_fingerprint` 预算 4096 节点，展开不了 → 条目降级——for 项值多为堆 List/Object，项级指纹**必须复用同一展开器**，超预算项降级不缓存。

**for 求值现状**：`AuraNode::ForLoop` 臂 7 处（aura_view_builder.rs:837/1090/1107/2636/2788/6849/7049/10692——含 debug_id/事件索引等面），项求值带循环变量 bindings（正是档 A 静态降级原因）。key 化的项级条目键需含**循环变量展开值**（同 key 不同内容必失效）。

**Spec 现状**：`vm-render-memo.md` SD-01..04 在册；本计划扩展 SD-05（for key 化）/SD-06（memo 块）/SD-07（outlet 语料级）。

## 详细设计

### 1. for key 化

- **语法**：`for x in .items key: .id { ... }`；`key:` 子句可选，缺省零差异。parser/lexer 增量：ForLoop AST 节点增 `key_expr: Option<Expr>`；语法面变更需同步 `token.rs`/`parser.rs`/`ast/ui.rs` 与 gen 侧（vue 轨对 `key:` 的消费——若 vue 轨已支持 key 透传则直连，否则 vue 侧解析兼容但不消费，本计划不改 vue 行为）。
- **项级条目**：键 = `(父容器 site+path, key 值)`；命中条件 = key 相同 ∧ item 值指纹同（`expand_heap_for_fingerprint` 展开，超 4096 降级）∧ 循环 index 相关产物（若 body 读 index）∧ episode 指纹同。读槽沿用档 A 慢路径（项体表达式重解析）。
- **键冲突**：重复 key → 该列表整体降级非 key 化路径（一次性诊断日志）；key 表达式求值失败 → 同降级。**绝不静默去重**。
- **输出序**：产物按 iterable 当前序拼装——缓存改变的是求值次数，不是输出顺序。

### 2. 显式 memo 块

- **语法**：`memo (deps: .a, .b) { ... }`；`exact: true` 可选。新块节点（AST + aura_view_builder 门），deps 为表达式列表，值指纹进命中条件。
- **默认（保守）语义**：deps 值指纹 ∪ 子树**可自动证明部分**的读槽重解析——即档 A 降级面里"用户包进来"的 ForLoop/Conditional/插值在块内仍按其自身规则参与判定，Call/VM 代码形态的隐藏读面由 deps 声明覆盖。
- **`exact: true`**：纯 deps 判定——语料作者签名担保"子树产物仅依赖 deps ∧ episode"；canonical 档明示该模式下陈旧风险由语料承担（正确性下限条款的唯一显式豁口，无默认用户）。
- **与档 A 组件门的组合**：块内组件节点的 memo 门照常工作，块条目与组件条目独立（不嵌套失效）。

### 3. outlet 语料级升级

- `outlet (memo: true) { }`：prop 优先；env `AUTO_OUTLET_MEMO` 未设且 prop 未写 = 原始路径。prop 门进键沿用档 A `probe_on` 等全套键分量。
- widgets-gallery 迁移一行语料（env 未设环境实靶），与 env 门时代的 AC-05 数据（2ms）对齐回归。

### 4. 计数与可观测

沿档 A：`hits`/`misses`/`degraded` 三计数器扩展 per-site 分解（for_item/memo_block/outlet），经 `bridge.with_memo_cache` 读取——验收与回归都用计数器断言，不靠体感。

### 规范增量

| delta_id | add/modify/retire | docs/specs/... target | before/after rule | rationale | acceptance IDs |
|---|---|---|---|---|---|
| SD-05 | modify | docs/specs/shell/vm-render-memo.md | `for` 增可选 `key:` 子句：key 化项级缓存（键集 diff+项值指纹+读槽版本+episode）；重复 key/求值失败整体降级；缺省零差异 | for 大列表原整体处于静态降级面，是档 B 最大剩余出血点 | AC-01..03 |
| SD-06 | modify | docs/specs/shell/vm-render-memo.md | 新增 `memo (deps) [{exact: true}]` 块节点：默认保守（deps ∪ 自动读槽）、exact 显式信任（唯一允许纯 deps 判定的形态，语料签名担保） | 覆盖重计算表达式场景；把档 A 降级表中"用户可担保"部分显式开放 | AC-04..05 |
| SD-07 | modify | docs/specs/shell/vm-render-memo.md | `outlet (memo: true)` 语料 prop（优先于 AUTO_OUTLET_MEMO env 门；env 保留兼容） | PLAN-045 T-05b 明示留待档 B 的语料级载体 | AC-06 |

## 测试设计

- **Rust 单测**（auto-lang，`cargo t -p auto-lang --features ui-iced`）：
  - parser：`key:` 子句解析矩阵（有/无/key 非 bool/与既有 for 语法叠加）；memo 块节点解析；
  - for key 化：键集增/删/重排/单项改/重复 key 降级/输出序保持；项值指纹（List/Object 展开+超预算降级）；
  - memo 块：默认语义 deps 变化失效/读槽变化失效/episode 翻转失效；exact 模式纯 deps；块内组件门组合；
  - outlet：prop 优先级（prop>env>原始）三态。
- **对拍**：无 `key:`/无 memo 块语料的 AST 与渲染产物逐字节一致（269 基线不新增红）。
- **实靶**（auto-os，debug，复用 PLAN-045 计时脚本口径 + memo 计数器）：
  - widgets-gallery datatable/filetree 页：无关 state 写下重求值计数归零；重排场景仅变化项 miss；
  - charts 页 memo 块命中；outlet 语料 prop 迁移后与 env 门时代 2ms 数据对齐。

## 验收标准

- **AC-01**：`for` 无 `key:` 时 parser AST 与渲染产物与基线逐字节一致（269 基线不新增红）。验证：全量套件 + 快照对拍。
- **AC-02**：key 化列表——重排（键集不变序变）时全部命中、增删仅变化项求值、单项内容修改仅该项 miss；输出顺序恒随 iterable 当前序。验证：单测断言（计数器+产物序）。
- **AC-03**：重复 key / key 求值失败 / 项值超指纹预算 → 整体降级原始路径，行为与非 key 化一致。验证：单测。
- **AC-04**：memo 块默认语义——deps 或子树读槽任一变化失效重求值，全同命中；episode 翻转失效。验证：单测。
- **AC-05**：`exact: true` 纯 deps 判定生效；无 exact 时 VM 代码形态隐藏读面不依赖 deps 之外猜测。验证：单测 + canonical 档条款对读。
- **AC-06**：`outlet (memo: true)` 语料 prop 优先于 env 门；widgets-gallery 迁移后切页维持 ~2ms 量级（与 PLAN-045 AC-05 数据对齐）。验证：实靶计时脚本。
- **AC-07**：实靶收益——datatable/filetree 页无关写重求值计数归零、charts 页块命中（计数器证据）。验证：计数器输出。

## 执行步骤

- **T-01**（调查，决策工件）：①`ForLoop` 七处臂的求值通道与 bindings 形态盘点（837/1090/1107/2636/2788/6849/7049/10692——debug_id/事件索引/`*` 展开面各属哪条），定 key 化改造的统一入口（或确认需逐臂）；②`key:` 语法在 vue 轨 gen 侧的现状（是否已支持/需兼容透传）；③memo 块节点的 AST 表示与保底字冲突检查。产出：决策注记（回填 §复审记录），确认/修订 §5。→ 全部 T 前置。[x] ✅（决策注记见复审记录 2026-09-28：1107 单臂统一入口、vue 不消费、MemoBlock 新变体、Q-01/Q-02 关闭）
- **T-02**：`for key:` 语法解析（parser/lexer/ast 三件 + vue 轨兼容臂）+ 缺省零差异回归。验证：parser 单测 + 全量对拍。→ AC-01。[x] ✅（lang os-046-dev 7d60d5327：key_expr 字段+探测解析+有界 key 解析器+vue 零触碰；plan046 矩阵 7/7 绿；scoped memo 52/for_loop 26 全绿，vue 家族 24 红单跑归因并行 flaky 同 PLAN-045 家族；全量对拍留 T-07 终验）
- **T-03**：for 项级条目缓存（键集 diff/项值指纹复用 `expand_heap_for_fingerprint`/读槽慢路径/降级三案/输出序）。文件：`aura_view_builder.rs` ForLoop 臂、`memo_deps.rs`、`vm_bridge.rs` 宿主。验证：AC-02/03 单测。[x] ✅（lang os-046-dev 73525abb9：tracked 1107 臂拆 unkeyed 逐字节搬移 + convert_for_keyed 门（键集 diff/快慢路径/降级三案/输出序恒随 iterable）+ replay_for_item 簿记重放（项内相对路径+ForIter.index 补丁，重排落位正确）+ scan_for_item_body/for_site_fingerprint（槽形态拆键——执行期补强：骨架 expr_shape 不分读目标，槽 Debug 并入站点指纹）+ MemoCache cap 抬升共池硬顶 4096 + per-site 计数；plan046 单测 16 条全绿[AVB 10+memo_deps 6]；scoped memo 68/for_loop 26 全绿；Q-03 关闭：共池+按需抬升硬顶，T-06 实测复核）
- **T-04**：memo 块（AST 节点 + view_builder 门 + 默认/exact 双语义 + 计数器 per-site 分解）。验证：AC-04/05 单测。[x] ✅（lang os-046-dev de59a08cd：MemoBlock 双侧 AST+parser 专用头参解析+AVB 门（默认=deps∪scan_memo_block_body 自动可证读槽[Call/FStr/StyleBinding 由 deps 覆盖不入槽不降级]、exact=纯 deps 语料签名、ForLoop/Outlet/嵌套块/bindings 非空降级）+块内相对路径簿记重放（replay_memo_subtree 与 T-03 共用）+12 处 gen/导出面 passthrough；块单测 7+parser 4 全绿，Q-02 关闭印证：默认块内 for 降级/exact+deps 覆盖 iterable 可缓存）
- **T-05**：outlet 语料 prop（prop 优先级 + env 兼容 + widgets-gallery 迁移一行）。验证：AC-06。[x] ✅（lang os-046-dev 9736a2c15：Outlet{memo} 变体+parser 可选头参+门 prop>env 并集语义+三态单测；os plan-046-dev 0e882b9 widgets-gallery app.at outlet→outlet (memo: true)；AC-06 的 ~2ms 实靶计时留 T-06 合并验证）
- **T-06**：实靶启用与测量——widgets-gallery datatable/filetree/charts 语料 opt-in（`key:`/memo 块）+ 020 曲库/syslog 抽一，基线 vs 后测（计数器+计时）。验证：AC-07。[ ] 🔄（复审 F-R1 重开：语料 opt-in/双轨修正/fills 实证已完成在册[def3413+d9b13ef]；**余项=AC-07 实靶计数器补测**——回访全命中零重求值/charts 块命中的宿主日志实录，测量两次因用户前台占用中止[焦点守卫生效零误注入]，待机器空闲窗口复跑测量链[流程在档：AUTO_MEMO_DIAG=1 宿主+画廊双击+侧栏 FileTree/DataTable/FileTree 序列]或用户明示接受单测计数器断言替代）（os plan-046-dev def3413+lang d9b13ef：四处语料 opt-in[filetree key: r.id+嵌套可证扩展/syslog key: r.i/area_chart memo 块/datatable 静态区 memo 块]；实靶证据 evidence/p046/——outlet site=6 FILL 经语料 prop[env 未设]+filetree site=7 frame fills=10[门全链路活]+侧栏 7HIT/2MISS；**执行期重大修正：桌面渲染帧=untracked 轨**（DynamicComponent::view=build），keyed/块门双轨挂载 c457acc；扫描扩展两项[嵌套 for 循环变量根/FStr 插值根段]；**AC-07 留观项**：交互式回访全命中与 A/B 计时未跑完——测量会话与用户实机使用冲突（合成输入落到用户前台窗口）即停，语义由单测计数器断言覆盖[快路径全命中/块命中/千行容量]，复审可复跑[探针已门控在册]）
- **T-07**：全量回归收口——auto-lang 全量套件（269 基线不新增红）+ 非 memo 语料对拍零 diff + 桌面走查冒烟（复用 0927 走查链）。→ AC-01 终验。[x] ✅（lang os-046-dev bd239d78：全量对拍 5510 绿/295 红 vs 基线 detached 5c558778f 5477/291——新红 22 归因[自有测试 19 满载全局翻转家族单跑绿+photo×2 在册漂移+write_state/plan066 单跑绿并行 flaky]、基线独红 18 反向漂移，代码面零新增真回归=AC-01 相对零增量达成；非 memo 快照对拍由套件 snapshot 家族承载；桌面走查冒烟于 T-06 实靶会话完成[画廊双页导航+组件渲染正常，证据 p046_m_filetree.png]；归因证据 evidence/p046/ 四件在档）

依赖链：T-01 → T-02 → T-03 → T-04 → T-05 → T-06 → T-07（T-03/T-04 可并行于 T-02 合入后）。

## 复审记录

- [drafting handoff 2026-09-28] stage: new，PLAN-046 r1。outcome: pass——T 覆盖全部 AC 与 SD-05..07；设计锚定档 A canonical 档实测形态（含 T-05b 归因修正与明示伏笔）；两个执行期不确定点（ForLoop 七臂统一入口、vue 轨 key 现状）已收进 T-01 有界调查。next: work。
- [work 启动 2026-09-28] stage: work 开始。worktree 组 `D:/autostack/.wt/os-046/{auto-os,auto-lang,auto-down}`（Plan 529 布局）：auto-os `plan-046-dev`@a488727、auto-lang `os-046-dev`@5c558778f（依赖组内兄弟，改 crates 故 cargo t 在 lang worktree 跑）、auto-down detached@3373a5c 只读（autodown-core path 依赖位）。auto-os 主检出 apps/** 未跟踪测试产物（025/028/037 前会话遗留）不入本计划工作面。
- [review 2026-09-28] stage: review | plan_id: PLAN-046 | plan_revision: r1 | outcome: **needs_fix**（唯一未决=F-R1 AC-07 实靶计数器补测；代码面零缺陷）| reviewed_commit: lang os-046-dev bd239d785（基 5c558778f，九提交线性）、os plan-046-dev 1977cd2（基 a488727，六提交）| dependency_revisions: auto-down detached 3373a5c 只读 | spec_inputs: docs/specs/shell/vm-render-memo.md（worktree 档全量重写版 SD-01..07，hash 随 plan-046-dev 11d1da7；frontmatter 定稿 supersedes 载体）| 独立性声明: 本评审与实现同会话，结论由工件重建（字节级对比/新鲜套件/实靶日志/语料核验），未采信执行摘要 | acceptance_results: AC-01 **pass**（评审基线全量重跑 5499 绿/305 红 vs 基线 291——新红 20 同已知家族[plan046 自有 18+write_state 单跑绿+bp::registry plan045 在册族]，keyless untracked 臂**字节级一致**实证[唯一差异=模式行绑定 key_expr 结构性必需]；三轮全量 291/295/305 漂移归因机器负载递增）；AC-02 **pass**（keyed 10 条 fresh 绿：重排全命中+输出序/单项修改 miss/增删 delta/index 保守失效/probe 重放）；AC-03 **pass**（降级三案+嵌套降级+扫描降级 fresh 绿）；AC-04 **pass**（块语义 7 条 fresh 绿：deps/读槽/episode 失效+无关写命中）；AC-05 **pass**（exact 纯 deps 单测+canonical 档条款对读一致）；AC-06 **pass**（三态单测 fresh 绿+语料迁移行核验+实靶 site=6 prop fill 日志在档）；AC-07 **partial**（filetree site=7 fills=10 实靶在档；回访全命中零重求值/charts 块命中的宿主计数器实录未采——补测两次因用户前台占用中止[焦点守卫生效零误注入]；语义由单测计数器断言覆盖）| findings: **F-R1**（中，affected=AC-07/T-06：实靶计数器补测未完成，修正=机器空闲窗口复跑测量链[流程在档]或用户明示接受单测替代）；**F-R2**（低，记录性：全量满载红漂移 291→295→305，memo 测试族对 menubar_open/theme 全局面满载敏感——plan045 在册家族非本计划引入，episode 注入隔离改善另档）| evidence: docs/plans/evidence/p046/（rev_suite.log+rev_fails+rev_new_vs_base+宿主日志+截图；scoped fresh plan046 38/38+memo 80/80+for_loop 26/26+outlet 9/9；字节级对比脚本结论在案）| next: work（仅补 F-R1 测量，代码零改动）→ 重复审 → merge
- [work 收口 2026-09-28] stage: work | plan_id: PLAN-046 | plan_revision: r1（含 T-01 决策注记+T-06 执行期修正）| outcome: **pass**（AC-07 交互计时留观入 Q-04）| code_commit: lang os-046-dev 九提交线性 7d60d5327→73525abb9→de59a08cd→9736a2c15→e650f9680→004ddc2bd→c457acc40→1d43c08c6→bd239d785、os plan-046-dev 五提交 a488727→0e882b9→def3413→11d1da7→1977cd2| task_ids: T-01..T-07 全勾 | evidence: AC-01 全量对拍[5510 绿/295 红 vs 基线 detached 5c558778f 5477/291，新红 22 归因零真回归：自有测试 19 满载全局翻转家族单跑 38/38+80/80 绿+photo×2 在册漂移+write_state/plan066 单跑绿；基线独红 18 反向漂移]；AC-02/03 单测 keyed 10 条；AC-04/05 块单测 7 条+parser 4 条；AC-06 三态单测+实靶 outlet site=6 prop fill[env 未设]；AC-07 filetree site=7 frame fills=10 实靶+单测计数器断言[交互计时留观 Q-04] | evidence_files: docs/plans/evidence/p046/（宿主日志+全量双跑日志+失败名单归因+截图）| blockers: 无阻塞项（Q-04 为留观非阻塞）| next: review（复审可复跑交互测量补 AC-07 计时口径——探针门控在册，启动链 tmp 脚本已清[复建用 scripts/desktop.sh iced + AUTO_OS_ROOT=worktree + AUTO_MEMO_DIAG=1]）
- [T-01 决策注记 2026-09-28] ①**ForLoop 臂盘点**（9 处非测试臂，计划所列 8 臂全实 + 12972 svgdoc 序列化）：渲染主臂唯一 = **1107**（`convert_node_tracked_ctx`，VM 渲染轨每帧入口 view_with_debug_gated→build_with_debug_gated）；837=untracked 平行（MCP sync/非 debug）、2636/2788/6849/7049=grid/row 展平（后三者经 `for_loop_iterations` helper 且 item 绑定未 materialize 口径不一）、10692=select options、12972=svgdoc、1090=debug_id 簿记。**key 化统一入口定 1107 单臂**，其余臂零触碰（不缓存=原始路径，正确性下限天然满足）。②**Q-01 关闭**：`key`/`memo` 均非关键字（token.rs all_keywords 零命中）；parser 在 iterable 与 `{` 之间做 `Ident("key")+Colon` 文本探测无歧义（iterable 三形态均不与裸 Ident+Colon 相邻冲突）；AST = `ViewNode::ForLoop`/`AuraNode::ForLoop` 增 `key_expr: Option<Expr>`（全字面量构造点 parser.rs:16159、extract.rs:1117/1300、memo_deps 测试+AVB 测试 8 处随编译器修，匹配侧 `..` 形态安全）。**vue 轨裁定：解析兼容、gen 不消费**——vue 已有三通道 :key（单子自动 `.id` 兜底/`find_loop_child_key` 深搜/组件 key prop），header 级透传零增益且动 vue 快照面。③**memo 块表示**：新变体 `ViewNode::MemoBlock`/`AuraNode::MemoBlock`（`parse_view_node` 15551 文本分发，同 for 先例；通用 prop 解析器逗号即截断故**头参专用解析**——deps 逗号分隔表达式列表 + `exact: true`）；untracked 路径 memo 块=子体直落原始渲染（VM 桌面走 tracked 轨）。④**Q-02 裁定（保守）**：块内不嵌套豁免——块内含 ForLoop → scan 降级 → 整块降级；bindings 非空上下文（for 体内在 memo 块）→ 不缓存（沿 `memo_ctx_ok`）。⑤项级条目形态：`MemoKey` 增 `item_key: Option<u64>`（None=档 A 组件条目，机制单源）+ skeleton_fp 携带 ForLoop 节点骨架指纹 + site 新常量；**嵌套 for（进入时 bindings 非空）保守降级**；index 变量声明时 index 入命中条件（重排必 miss，与 §详细设计"若 body 读 index"一致）；probe/idmap 簿记沿档 A replay 机制按项内相对路径记录、命中帧新基路径重放（可行性落地时读 replay 机制实码确认）。⑥outlet prop：`ViewNode::Outlet`/`AuraNode::Outlet` 增 `memo: bool`（缺省 false），parser outlet 臂支持可选 `(memo: true)` 头参，AVB 5049 env 门改 prop>env>raw 三态。Q-03（LRU 容量）留 T-03 实测定。

## 待澄清事项

- **Q-01**：`key:` 语法若 vue 轨 gen 侧不支持，兼容臂形态（忽略透传 vs 报错）——**已关闭（T-01）**：vue 解析兼容、gen 不消费（vue 已有三通道 :key，header 级透传零增益）。
- **Q-02**：memo 块默认语义中"子树可自动证明部分"的边界——**已关闭（T-04）**：块内不嵌套豁免（ForLoop/Outlet/嵌套块整块降级）；Call/VM 形态由 deps 覆盖不入槽不降级。
- **Q-03**：项级条目的 LRU 容量与桥级 256 上限的关系——**已关闭（T-03/T-06）**：共池单 LRU+ensure_capacity 按列表长度按需抬升、硬顶 4096；千行无逐出单测在册；T-06 实靶复核延续至复审。
- **Q-04（work 交接留观）**：AC-07 的交互式 A/B 计时与回访全命中实录未跑完——测量会话进行中用户回到实机（合成输入落到用户前台窗口，即停止损）；已捕获证据（filetree site=7 fills=10、outlet site=6 prop fill、侧栏 7HIT/2MISS）+ 单测计数器断言（快路径全命中/重排全命中/块命中/千行容量）覆盖同语义；诊断探针已 AUTO_MEMO_DIAG 门控在册，复审/合并可复跑交互测量补计时口径。
