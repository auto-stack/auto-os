# Spec: VM 渲染组件级 memo（vm-render-memo）

> Source of truth for VM 轨组件实例级 memo 缓存契约（PLAN-045 第一批：
> 菜单族四件 menubar/toolbar/dropdownmenu/contextmenu 族 + sidebar nav 块；
> PLAN-046 档 B：for key 化 SD-05 + 显式 memo 块 SD-06 + outlet 语料级
> SD-07）。
> 实现单源：auto-lang `crates/auto-lang/src/ui/memo_deps.rs`（缓存/指纹/扫描）
> + `aura_view_builder.rs`（memo 门引擎与 `*_raw` 包装层）+ `vm_bridge.rs`
> （缓存宿主与 seq 补 bump）。正确性下限：**memo 错误的表现形式只允许
> "变慢"，绝不允许"显示陈旧"**。

## SD-01 opt-in 开关与非 memo 逐字节保留

- `memo` 是普通 bool prop（`menubar (memo: true) { … }` /
  `sidebar_provider (memo: true)`），**缺省/`false`/非 bool 字面量一律按
  关**（非 bool 一次性诊断日志）。关闭路径 = 包装函数直落 `*_raw` 原始
  转换体——原始代码不重排、不包条件，非 memo 行为与产物逐字节一致。
- 第一批消费者：`menubar`（声明式组件族 + actions DSL 双形态）、`toolbar`
  （DSL）、dropdown-menu 族（`convert_alert_dialog*`，ModalDialogFamily）；
  `sidebar_provider (memo: true)` **武装**其子树内 `sidebar_group` 臂入
  nav memo 门（provider 本体不建条目——passthrough 便宜，组级条目才是
  收益粒度）。contextmenu（popover 锚定族）第一批未接，见 §边界。
- **T-05b 扩展（PLAN-045 执行期裁定，拆账实证驱动）**：outlet 页产物
  memo——实测导航阻塞 99.9% 在 outlet 页渲染（27535/27537ms，侧栏+壳
  ~2ms；0927"侧栏主导"归因推翻），AC-05 非页级缓存不可达。原始载体 =
  `AUTO_OUTLET_MEMO=1` 环境门；**档 B（SD-07）起语料 prop 为正载体，
  env 降级为兼容层**。命中帧重放产物无关簿记（mounted/path sink、
  callback routes、state prep）；**Init 身份变化帧弃缓存全量渲染**
  （Init 派发语义逐字节保持）。

## SD-02 缓存键与命中条件

- 键 = `(ctx_state_obj, site, skeleton_fp, probe_on, item_key)`：
  - `ctx_state_obj`：子 widget 状态作用域（`override_state_obj_id` 或根态）。
  - `skeleton_fp`：子树静态骨架指纹（tag/prop 键/字面量值/文本/结构——
    档 B keyed-for 站点另含循环头 var/index/iterable/key 表达式全形态 +
    体读槽表达式全形态；memo 块站点含 exact 旗 + deps 全形态）。同一桥
    生命周期内 view 模板不可变（hot-reload 走新建 VmBridge，缓存随桥
    弃置），骨架同 ⇒ 结构同。
  - `site`：convert 家族判别（1..5=档 A 组件族、6=outlet 页、7=keyed-for
    项级、8=memo 块；防跨家族骨架撞键）。
  - `probe_on` 进键：probe-off 先填充、probe-on 后命中会吞 acceptance 事件
    索引——两态各存各的条目（AC-06）。
  - `item_key`（档 B）：keyed-for 项级条目的 key 值指纹；`None` = 组件/
    outlet/块条目（档 A 键面零变化）。
- 命中 = **快速路径**（全局 `state_mutation_seq` 自 fill 未动 ∧ 全局
  episode 指纹同 → 零重解析复用产物）或 **慢路径**（读槽表达式经同一求值
  通道 `resolve_expr_to_value` 重解析、值指纹比对同 ∧ episode 指纹同）。
  档 B 扩展：keyed-for 项级慢路径 = 项值指纹 ∧ 项体读槽 ∧ [声明 index
  时迭代位]（`memo_combine_for`）；memo 块慢路径 = deps 值指纹恒比 ∧
  [默认语义另比块内读槽]（`memo_combine_block`，exact=None）。
- 全局 episode 指纹分量：`theme_epoch` + `menubar_open` +
  `popover_open` + `action_config` Arc 身份（任一翻转 → miss，重求值）。
- 值指纹规则：堆引用（`VmRef`/`Int≥4M`）先展开为纯值再哈希
  （ObjectData/GenericInstance→Obj、ListData→Array；`expand_heap_for_fingerprint`）；
  **展开不了的堆引用 → 整条目降级**（绝不按含堆 id 的指纹命中——原地突变
  会漏检成陈旧）；展开预算 4096 节点，超限降级。
- 计数器语义：`hits` 只在产物确被复用时递增（快速/慢路径命中）；
  `misses` = 有条目但判定失效（keyed-for 新键 = fill 不计 miss）；`degraded`
  = 降级不入缓存。**per-site 分解**（档 B §4）：`site_counts[site]` 经
  `note_*_site` 记账（验收与回归都用计数器断言，不靠体感）。

## SD-03 正确性论证与降级面（宁缺勿错）

- 论证：转换器是确定式函数 `f(AST 子树, prop 表达式解析值, 转换器隐藏读面,
  全局 episode 态)`。AST 在桥生命周期内不可变；prop 表达式值由慢路径重解析
  覆盖；episode 态入指纹。故命中 ⇒ 产物必同，无陈旧。
- 静态降级（不建条目走原始路径）：`Expr::Call`/`Block`/`FStr`/Lambda/
  Closure/借用系等 VM 代码形态；`ForLoop`/`Conditional`/`Component`/`Outlet`
  子节点；插值文本；`StyleBinding` prop。绑定门：bindings 非空或 widget 声明
  computed → 上下文整体不 memo（循环变量与 computed 走 VM 代码，静态不可证）。
- **档 B 项级（keyed-for）扫描规则**（`scan_for_item_body`）——项值指纹覆盖
  循环变量的一切展开，扫描只找**外部状态读**入槽：
  - 条件臂递归：条件串 `parse_expr_fragment` 解析入槽（解析失败降级
    `conditional_unprovable`/VM 代码形态按 scan_expr 拒绝）；
  - 插值文本 / FStr prop：插值名**根段**（`r.rowcls`→`r`）∈ 循环变量 →
    项值覆盖（安全）；外部名 → 降级（`interpolated_text_external`/
    `fstr_external`）；
  - 嵌套 ForLoop：iterable **根段 ∈ 循环变量**（filetree 的
    `for g in r.guides`）→ 项值覆盖，递归体（内层 var/index 并入循环变量
    集）；根为状态（`.` 前缀或外部名）→ 降级（`for_loop`）；
  - 嵌套 memo 块/Outlet → 降级。
- **档 B memo 块扫描规则**（`scan_memo_block_body`，仅默认语义；exact 跳过
  扫描）：VM 代码形态（Call/FStr/Lambda/StyleBinding）**不入槽不降级**——
  块的 deps 声明即为其覆盖面；插值名逐名入槽（`resolve_expr_to_value` 是
  实际读通道的可靠上界）；降级仅限结构形态：嵌套 ForLoop/Outlet/嵌套块。
- sidebar nav 块派生值键：`sidebar_menu_button` 的 active 复刻转换器判定序
  （显式 `active:` prop > `nav_route_active(to, exact)`）+ collapsible 组
  开态（`nav_group_states` 全表）并入派生指纹——路由变化仅 active 翻转组
  失效，其余组全命中（PLAN-045 §3 期望形态）。
- 写面契约：**Rust 桥写六口补 bump 全局 seq**（`write_state`/
  `write_or_insert_state` 新增臂/`write_state_vec` 双容器臂/`sync_busy_flag`
  真写臂/`ensure_child_state` 值变化臂）——否则 `set_route` 等桥写通道对
  memo 与 PLAN-062 fire_timer 空转拍判定不可见。`ensure_child_state` 每帧
  重种子**值变化才 bump**（不变值重种子不得坐实"每帧 seq 必动"）。

## SD-04 缓存宿主与容量

- 宿主 = `VmBridge`（builder 每帧借用临时、桥跨帧持久）；`RefCell` 内可变
  （渲染期 `&self`，与 parked_tasks 同款理由）。类型面依赖 interpreter，
  `#[cfg(feature = "ui-interpreter")]` 门控。
- 容量：每桥共池单 LRU，默认 256 条目；**档 B Q-03 裁定：keyed-for fill
  按列表长度按需抬升（`ensure_capacity`），硬顶 4096**（千行列表防 LRU
  抖动使重求值计数无法归零；超顶走 LRU 逐出语义，正确性不受影响——只允许
  变慢）。计数器经 `bridge.with_memo_cache` 读取（验收与度量通道）。
- **双轨可达（档 B T-06 执行期实证）**：桌面窗口渲染帧走
  `DynamicComponent::view()` = `build()` = **untracked 转换器**；tracked
  轨（`build_with_debug*`）仅 acceptance/sync 面走。故 keyed-for 与 memo
  块门**双轨挂载**：untracked 臂伪造空簿记通道（空 path/空 id_map/禁用
  probe）复用同一门实现（机制单源）；probe 禁用 → `probe_on=false` 键面，
  与 tracked 帧条目互不串。probe/idmap 簿记在 untracked 帧为空集（该轨无
  acceptance 面）。outlet 页 memo 同理双轨（`render_child_widget*` 双胎）。

## SD-05 keyed-for 项级缓存（`for x in .items key: .id`）

- **语法**：`key:` 子句可选，位于 iterable 与体 `{` 之间（`key` 为软标识
  符，非关键字；iterable 三形态均不与裸 Ident+Colon 相邻冲突）；缺省零
  差异（keyless 路径 = `convert_for_unkeyed` 逐字节原臂实现）。key 表达式
  有界解析（`parse_view_key_expr`）：self 根状态链（`.a.b`）/项字段链
  （`item.id`）/Int/Str 字面量；Call/index/object 形态指名拒绝（键须为
  纯字段读——指纹按值展开，VM 代码形态不可证）。vue 轨解析兼容、gen 不
  消费（vue 已有三通道 `:key`：单子自动 `.id` 兜底/`find_loop_child_key`
  /组件 key prop；header 级透传零增益）。
- **项级条目**：键 = `(ctx_state_obj, site=7, 站点骨架指纹, probe_on,
  key 值指纹)`。命中 = 快速路径（seq 未动 ∧ episode 同）或慢路径（项值
  指纹 ∧ 项体读槽重解析[循环绑定上下文] ∧ [声明 index 时迭代位]）。
- **降级三案（AC-03，整体降级原始路径=非 key 化一致，绝不静默去重）**：
  重复 key / key 求值失败 / 项值超 4096 指纹预算；另：项体扫描降级、
  嵌套绑定上下文（外层 for 体内的 keyed for——外层循环变量版本静态不可证）。
- **输出序恒随 iterable 当前序**（缓存改变的是求值次数，不是输出顺序）。
  搜索过滤每帧按当前态现算（帧级集合语义，不进项缓存）。
- **重排全命中（AC-02）**：产物事件载荷构建期自循环绑定解析（位置无关），
  身份面（VNodeId/iced id）flatten 期按位置派生——命中帧产物跨位复用
  安全；probe/idmap 簿记按**项内相对路径**存储、命中帧以当前基路径+[i]
  前缀化重放（`replay_for_item`/`replay_memo_subtree`），ForIter.index
  补丁为当前迭代位（acceptance 事件索引落位正确）。声明 index 变量 =
  保守位入命中条件（重排必失效——无法廉价证明体不读 index，宁可多重
  求值）。
- 实证：widgets-gallery filetree 行循环（`for r in .ftRows key: r.id`，
  体含嵌套 `for g in r.guides` + FStr class + 条件臂）桌面实靶
  `site=7 frame fills=10`（门全链路活）；单测 16 条（fill/快路径/重排全
  命中+输出序/单项修改慢路径 miss/增删 delta/index 保守失效/降级三案产物
  对拍/嵌套降级/probe 重放）。

## SD-06 显式 memo 块（`memo (deps: .a, .b) { … }`）

- **语法**：`memo (` 前缀单 token 探测分发（`memo` 非关键字；裸 `memo`
  走通用 tag 路径零冲突）；头参专用解析——`deps:` 逗号分隔表达式列表
  （Tuple 拍平）+ `exact: true|false`（非 bool 指名报错）；未知头参报错。
- **默认语义（保守）**：deps 值指纹 ∪ 块内自动可证读槽（SD-03 块扫描规
  则）——Call/VM 代码形态的隐藏读面由 deps 声明覆盖（charts path、
  codeblock 高亮场景）；episode 翻转失效。
- **`exact: true`（语料签名担保）**：纯 deps 判定——不扫描不重解析读槽，
  **唯一允许"纯 deps 判定"的形态**；canonical 档明示：该模式下块内隐藏
  读面若超出 deps，陈旧风险由语料作者承担（正确性下限条款的唯一显式豁
  口，无默认用户）。
- **保守边界（Q-02 裁定）**：块内嵌套 ForLoop/Outlet/嵌套块 → 整块降级
  （默认语义；exact 可豁免——作者担保 iterable ⊆ deps）；bindings 非空
  上下文（for 体内的 memo 块）→ 不缓存（外层循环变量版本不随 deps）。
- 块内组件节点照常走自身 memo 门（块条目与组件条目独立，不嵌套失效）；
  untracked 轨同走共享门（SD-04 双轨）。
- 站点指纹 = exact 旗 + deps 全形态 + 体结构；命中帧簿记块内相对路径
  重放（上游结构变化时落位正确）。
- 实证：单测 7 条（AC-04 deps/读槽/episode 失效 + 无关写慢路径命中、
  AC-05 exact 纯 deps 隐藏读命中 + deps 失效、Call-prop charts 形态、
  嵌套 for 降级 + exact 豁免、untracked 共享门、per-site 计数）。

## SD-07 outlet 语料级 prop（`outlet (memo: true)`）

- `outlet` 由无 props 单元变体升级为 `{ memo: bool }`；parser 支持
  `outlet (memo: true)`（其他头参指名报错；裸 `outlet` 字节兼容）。
- **门优先级 = prop ∨ env**：语料 prop 为正载体（PLAN-045 T-05b 明示留
  待档 B 的载体位），`AUTO_OUTLET_MEMO` 环境门保留为兼容层（既有实验/
  测试脚本零回归）。prop 载体进键沿档 A 全套键分量（probe_on/
  ctx_state_obj/骨架指纹）。
- 实证：单测三态（prop=true+env 未设开门命中 / prop=false+env 未设惰性
  零计数 / prop=false+env=1 兼容开门）；桌面实靶 widgets-gallery 迁移
  一行（app.at `outlet (memo: true)`），env 未设下 `site=6 FILL` 经 prop
  开火（宿主日志在档）。

## 边界与非目标

- memo 范围限同步求值产物；含图片节点/异步产物子树不缓存（保守排除）。
- interpreter 字节码级动态读集拦截不在本机制（T-01 裁定：值指纹已闭合
  正确性，版本表不完备[容器原地突变无字段归因]且需 engine 侵入）。
- contextmenu（popover 坐标锚形态）留待后续批次（机制通用，键面/粒度
  另议）。
- keyed-for v1 边界：外层 for 体内不缓存（嵌套绑定上下文）；index 声明
  即位入命中条件（保守）；插值/FStr 外部名降级。
- Vue 轨不适用（DOM diff 天然增量）；`key:` 子句 vue gen 不消费。

## 验证

- lang 单测：`ui::memo_deps`（指纹确定性/堆展开/骨架键/扫描降级/LRU/
  episode 翻面 9 条 + 档 B 项级扫描/嵌套扩展/站点指纹/容量与 per-site
  计数 8 条）+ `aura_view_builder::plan046_for_memo_tests`（keyed 10 条）
  + `plan046_memo_block_tests`（块 7 条）+ `plan046_outlet_prop_priority`
  + parser 矩阵（key 子句 7 条/memo 块 4 条/outlet prop 1 条）。
- 门：`cargo test -p auto-lang --features ui-iced --lib`（AC-01：非 memo
  路径零回归——269 基线相对零增量口径；与基线 detached 5c558778f 对拍，
  差集归因归零）。
- 实靶（debug 同构建，`docs/plans/evidence/p046/` 在档）：
  - outlet 语料 prop：widgets-gallery 迁移一行，env 未设下
    `[MEMO-DIAG] site=6 FILL page=…` 经 prop 开火（IndexPage/FileTreePage
    两页实录）；PLAN-045 AC-05 的 ~2ms 回访口径由同机制承载（env 时代
    实测在档）。
  - keyed-for：filetree 页 `site=7 frame fills=10`（门全链路活）；
    回访全命中/单项修改 miss/增删 delta 由单测计数器断言承载（交互式
    A/B 计时留观——测量会话与用户实机使用冲突，合成输入不安全）。
  - 计数器口径：`site_counts[7/8]` per-site 分解（验收与回归都用计数器
    断言，不靠体感）。
