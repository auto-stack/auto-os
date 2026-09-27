# Spec: VM 渲染组件级 memo（vm-render-memo）

> Source of truth for VM 轨组件实例级 memo 缓存契约（PLAN-045 第一批：
> 菜单族四件 menubar/toolbar/dropdownmenu/contextmenu 族 + sidebar nav 块）。
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
  ~2ms；0927"侧栏主导"归因推翻），AC-05 非页级缓存不可达。载体 =
  `AUTO_OUTLET_MEMO=1` 环境门（裸 `outlet` 节点无 props 载体，语料级
  prop 留待档 B 显式 memo 语法升级；env 未设 = 原始路径零变化）。命中帧
  重放产物无关簿记（mounted/path sink、callback routes、state prep）；
  **Init 身份变化帧弃缓存全量渲染**（Init 派发语义逐字节保持）。

## SD-02 缓存键与命中条件

- 键 = `(ctx_state_obj, site, skeleton_fp, probe_on)`：
  - `ctx_state_obj`：子 widget 状态作用域（`override_state_obj_id` 或根态）。
  - `skeleton_fp`：子树静态骨架指纹（tag/prop 键/字面量值/文本/结构）——
    纯 AST 漫步零 VM 零分配。同一桥生命周期内 view 模板不可变（hot-reload
    走新建 VmBridge，缓存随桥弃置），骨架同 ⇒ 结构同。
  - `site`：convert 家族判别（防跨家族骨架撞键）。
  - `probe_on` 进键：probe-off 先填充、probe-on 后命中会吞 acceptance 事件
    索引——两态各存各的条目（AC-06）。
- 命中 = **快速路径**（全局 `state_mutation_seq` 自 fill 未动 ∧ 全局
  episode 指纹同 → 零重解析复用产物）或 **慢路径**（读槽表达式经同一求值
  通道 `resolve_expr_to_value` 重解析、值指纹比对同 ∧ episode 指纹同）。
- 全局 episode 指纹分量：`theme_epoch` + `menubar_open` +
  `popover_open` + `action_config` Arc 身份（任一翻转 → miss，重求值）。
- 值指纹规则：堆引用（`VmRef`/`Int≥4M`）先展开为纯值再哈希
  （ObjectData/GenericInstance→Obj、ListData→Array；`expand_heap_for_fingerprint`）；
  **展开不了的堆引用 → 整条目降级**（绝不按含堆 id 的指纹命中——原地突变
  会漏检成陈旧）；展开预算 4096 节点，超限降级。
- 计数器语义：`hits` 只在产物确被复用时递增（快速/慢路径命中）；
  `misses` = 有条目但判定失效；`degraded` = 降级不入缓存。

## SD-03 正确性论证与降级面（宁缺勿错）

- 论证：转换器是确定式函数 `f(AST 子树, prop 表达式解析值, 转换器隐藏读面,
  全局 episode 态)`。AST 在桥生命周期内不可变；prop 表达式值由慢路径重解析
  覆盖；episode 态入指纹。故命中 ⇒ 产物必同，无陈旧。
- 静态降级（不建条目走原始路径）：`Expr::Call`/`Block`/`FStr`/Lambda/
  Closure/借用系等 VM 代码形态；`ForLoop`/`Conditional`/`Component`/`Outlet`
  子节点；插值文本；`StyleBinding` prop。绑定门：bindings 非空或 widget 声明
  computed → 上下文整体不 memo（循环变量与 computed 走 VM 代码，静态不可证）。
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
- 容量：每桥 LRU 256 条目，溢出逐最旧（PLAN-045 Q-03：首批量级 ≪ 上限，
  执行期按内存实测调）。计数器经 `bridge.with_memo_cache` 读取（验收与
  度量通道）。

## 边界与非目标

- memo 范围限同步求值产物；含图片节点/异步产物子树不缓存（保守排除）。
- interpreter 字节码级动态读集拦截不在本机制（T-01 裁定：值指纹已闭合
  正确性，版本表不完备[容器原地突变无字段归因]且需 engine 侵入）。
- contextmenu（popover 坐标锚形态）与 outlet 页级 memo 留待后续批次
  （机制通用，键面/粒度另议）。
- Vue 轨不适用（DOM diff 天然增量）。

## 验证

- lang 单测：`ui::memo_deps`（指纹确定性/堆展开/骨架键/扫描降级/LRU/
  episode 翻面 9 条）+ `aura_view_builder::plan045_memo_tests`（menubar
  命中与失效、off 惰性、动态降级、sidebar nav 组派生键、outlet 页门
  快/慢路径 6 条）。
- 门：`cargo test -p auto-lang --features ui-iced --lib` 全绿（非 memo
  路径零回归——AC-01；与基线 detached c0a52de7b 对拍，差集归因归零）。
- 实靶（debug 同构建 A/B，`docs/plans/evidence/p045/` 数据在档）：
  - **AC-05**：widgets-gallery 切页 memo OFF 8.9~26.4s → memo ON 回访
    **block_ms=2**（`[VM-VIEW] widget=App build_ms` 口径，即 VM 整树重
    解释主线程阻塞；较 0927 基线降三个数量级）。首访为 FILL 全价（机制
    必然），收益在回访导航。MEMO-DIAG 实机：侧栏 nav 块每帧 6 HIT +
    1 MISS（仅 active 翻转组重求值），outlet 页门回访全 HIT slow。
  - **AC-02**：jade-edit menubar（memo:true worktree 语料）文件/视图菜
    单展开渲染与切换 Console 勾选态：memo ON 与 OFF 逐项一致（勾选
    glyph 经状态写失效重渲正确呈现）；截图在 `tmp/p045_J_*.png`。
  - 计数器口径：路由切换仅 active 翻转组 `misses` 递增、其余组 `hits`
    递增（MEMO-DIAG 实录）。
