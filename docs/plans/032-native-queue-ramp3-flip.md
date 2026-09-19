---
plan_id: PLAN-032
status: execution_done               # drafting → executing → execution_done → reviewed → archived
feature_name: native-queue-ramp3-flip
author: [agent]
created_at: 2026-09-19
updated_at: 2026-09-19
plan_revision: 1

# /auto-plan:review 结束时填写：
supersedes_spec_components: []
new_spec_components:
  - auto-lang/docs/design/autoui/desktop-protocol-v1.md   # §1.13 v1.13 增量（031 先占 §1.12——drafting 时预计 v1.12，依 merge 序实取 §1.13）
touched_goals: []

affects:
  - auto-lang/crates/auto-lang/src/ui/desktop_protocol/coverage.rs        # 六缺项判定/放行 + kinds/prefixes + 翻转仪器
  - auto-lang/crates/auto-lang/src/ui/desktop_protocol/native_projector.rs # tabs/hidden/样式 grid/定位族（依 D1 口径）投影臂
  - auto-lang/crates/auto-lang/src/ui/desktop_protocol/client_entry.rs    # 翻转点（Covered 臂 :128，门后）
  - auto-lang/crates/auto-lang/src/ui_gen/rust.rs                         # tabs/tab a2r 断裂映射（依 D2 定案）
  - auto-lang/docs/plans/KNOWN-DEBT-AND-RISKS.md                          # P026-D3 处置 + 新债随注
  - auto-os/docs/plans/autos-desktop-program.md                           # M7-a 批次行 + 裁定行
current_step: 8
total_steps: 8
---

# [PLAN-032] native-queue-ramp3-flip

## 0. 变更摘要

**M7-a 波次**（台账终态批次第一波，承接 P026-D3）：native queue 覆盖
**ramp v3** 补齐 029 复测的 judged 六缺项，随后**缺省翻转复评**——
"app 缺省走 RenderQueue"的最后一门。

029 复测（reports/p029-native-flip-retest-row.md）：judged 16/22 =
72.7% < 95% 阈值，六缺项全在册：**018-book-reader**（fixed/hidden/
z——定位族+hidden）、**021-blog-viewer**（sticky/top/z——定位族）、
**024-charts**（style-grid——样式版 grid）、**041-auto-edit**
（hidden）、**046-tabs-variants**（tag:tabs——整 kind）、**012-clock**
（native-unstyled——映射表兜底漏类）。补齐分五族，工作量分层明确
（映射小改 → kind 臂 → 布局标志 → walker 分岔 → **定位族深水**——
T-01 定口径：absolute+offset 真渲[覆盖序先例] + fixed/sticky 降级放行
随注[opacity/overflow 先例]的分层方案为倾向）。

翻转 = `resolve_native_frame_mode` Covered 臂 one-line
（client_entry.rs:128 返 Pixels → Commands）+ 仪器防漏断言反转
（native_flip_coverage_data_row 的 `assert!(!flip)` coverage.rs:
1270-1273——达标即红逼翻转的向下任）+ 台账裁定行。**dual-exit**：
过门翻 / 不过显式不翻，禁无数据翻转。

## 1. 目标

- **G1 六缺项五族补齐**：
  ①012 映射兜底（`native_style_token` 补臂——最小改）；
  ②**tabs 整 kind**（kinds 入册 + `View::Tabs` 投影臂[标签托盘 +
  内容区 + on_select 命中，Popover/MouseArea 臂先例结构] + a2r
  tab/tab 断裂映射修复[依 D2]——046 TabsVariant default/enclosed 两态）；
  ③**hidden**（display:none 语义——prefixes 放行 + NodeStyle 布尔 +
  各布局臂跳过零尺寸）；
  ④**样式 grid**（`SC::Grid/GridCols/GridRows` token 放行 +
  `node_style_of` 解析 GridCols → Container 臂分岔复用既有 Grid
  walker——`View::Grid` 变体臂已在，只缺 style 路径）；
  ⑤**定位族**（依 D1 定案——倾向分层：absolute + offset 真渲[覆盖
  序 ops，Popover 先例] + fixed/sticky 降级放行随注[opacity/overflow
  先例，真渲另立债]）。
- **G2 缺省翻转复评（dual-exit）**：复测仪器 `native_flip_coverage_
  data_row` 复跑——judged ≥95% 过门 → 翻转（Covered 臂 one-line +
  防漏断言反转 + 观测行语义更新 + 台账 M7-a 裁定行 + 全 examples
  抽样 auto 档 e2e）；不过门 → 显式不翻留痕（缺项清单更新）。
- **G3 收口**：desktop-protocol-v1.md **§1.12 v1.12 增量**（五族
  口径 + 翻转裁定结果）；P026-D3 处置（翻则核销、不翻则数据行更新）；
  新债随注（fixed/sticky 真渲——若 D1 采分层）；台账 M7-a 行更新
  （M7-c② 的 tabs 依赖解锁注记）；shell pack 五件 Covered 维持。

**非目标**（明确出界）：

- **fixed/sticky 视口锚定真渲**（若 D1 采分层——帧语义/宿主层牵动，
  债登记另立）；z-index 完整栈序（absolute 覆盖序内的相对层级即可）。
- table/canvas/video/terminal/imagesurface kind（M7-c③ sys-monitor
  的 table 捆绑立项；terminal 归 auto-term 特殊线——本计划不碰）。
- M7-b（shell 编译面/overlay outproc）与 M7-c（app 批量化）——同批
  次并行波次，各自立项。
- 解释态投影臂的对应扩容（I4 分表——native 表扩容不牵解释态
  target_set）。
- 样本集口径变更（维持 examples 目录运行时扫描 + judged 剔除仪器桶
  的 026 D3 口径；新例稀释属仪器如实反映，不配平）。

## 2. 架构方案

```text
┌─ 判定面（coverage.rs）──────────────────────────────────────────────┐
│ ②kinds + "tabs"；③prefixes + "hidden"；④token 放行 style-grid；    │
│ ⑤prefixes + absolute/top-/left-/right-/bottom-/z-（fixed/sticky    │
│   依 D1）；①native_style_token 补 012 新类臂（:762 兜底前）        │
│ 防漏钉矩阵更新（新 kind×投影臂双向钉）                              │
└──────────────────────────────────────────────────────────────────┘
┌─ 投影面（native_projector.rs）─────────────────────────────────────┐
│ ②View::Tabs 臂：标签托盘（Row of 按钮态 Quad/Text）+ 选中态 +     │
│   内容区子树 + on_select 命中项（Popover 臂先例结构）              │
│ ③hidden：NodeStyle.hidden 标志 → layout_view_node 各臂跳过返零尺寸 │
│ ④样式 grid：node_style_of 解析 GridCols → Container 臂分岔复用    │
│   Grid walker（:1160 两遍网格既有）                                │
│ ⑤定位族（依 D1）：absolute+offset = 覆盖序 ops（主块后追加，      │
│   Popover 先例）；fixed/sticky = 降级放行（in-flow 渲染 + 随注）    │
└──────────────────────────────────────────────────────────────────┘
┌─ 翻转面（client_entry.rs + coverage.rs 仪器）───────────────────────┐
│ 复测 → 过门：Covered 臂 :128 Pixels→Commands + assert(!flip) 反转  │
│ + 观测行改"flipped@ramp3" + 台账裁定行 + 抽样 e2e                   │
│ 不过门：数据行更新 + 显式不翻（dual-exit）                          │
└──────────────────────────────────────────────────────────────────┘
```

**不变式**：

- **I1 追加式协议**：零 wire 变体——五族全部是投影器/覆盖表/宿主侧
  演进，`PROTOCOL_VERSION` 仍 1。
- **I2 零回归**：既有 kinds/golden/防漏钉零漂移；shell pack 五件
  Covered 维持（shell_pack_native_covered 回归）；解释态分表不动。
- **I3 not-yet 纪律**：fixed/sticky（若分层）与 z 完整栈序 = 显式
  降级/随注，禁静默错绘。
- **I4 数据门纪律**：翻转仅凭复测数据（judged ≥95% + 缺项全在册），
  dual-exit，禁无数据翻转（026/029 口径沿承）。

**关键风险**：定位族口径选择影响 018/021 两例能否 Covered（分层口径
下 018 的 fixed×2/z-40、021 的 sticky×2 走"放行+随注"——判定翻绿但
渲染 in-flow，保真边界必须随注明示）；tabs 的 a2r 断裂半句（§1.8
在册"tab 映射断裂维持"）若不修，046 经 VM 轨可判但 a2r 轨不可编译
——翻转门样本是 VM 轨扫描不受阻，但 M7-c② 依赖 a2r 侧（D2 定同批
与否）；hidden 跳过对布局聚合（flex-1/尺寸推导）的连锁——逐臂
golden 钉。

## 3. 技术栈

Rust / iced 0.14；既有投影器/覆盖表/翻转仪器（native_flip_
coverage_data_row @ coverage.rs:1171-1278——cargo 测试产物即数据行，
复现命令在 p029 报告头）；Popover 覆盖序/MouseArea 命中先例（029
交付）；Grid walker（:1160）；opacity/overflow 降级放行先例；验收
载体 = 046-tabs-variants / 024-charts / 018-book-reader / 021-blog-
viewer / 041-auto-edit / 012-clock 六例（queue 显式档全链 + golden）。

## 4. 需求分析与背景调查

**授权记录**：用户 2026-09-19 会话确认 M7 排序（"① 翻转两件 → ②
app 批量化 → ③ 收口"）并问询三波合一——裁定落地 = 台账 M7 终态批次
（commit b2e9cce：不合成单一执行计划，程序级规划归台账）+ 本计划为
M7-a 首件。**本轮仅规划，未授权实施**。涉及仓：auto-lang（覆盖/
投影/翻转/文档）+ auto-os（台账）。无预算/自动续跑约束声明。

**前置依赖**：030 已归档 ✅（投影器/覆盖基线）；**建议 031 merge 后
执行**（client_entry.rs 同文件小面 + 同 crate 串行习惯——031 在
review，非硬前置）。

**现状事实**（已核，2026-09-19 master，探索代理全量普查）：

- **复测数据与六缺项**（reports/p029-native-flip-retest-row.md）：
  overall 16/36 = 44.4%、judged 16/22 = 72.7%（剔 parse-fail ×10 +
  bridge-fail ×3 + no-widget ×1）；阈值 ≥95%；judged 缺项表 = 018
  （fixed/hidden/z）/021（sticky/top/z）/024（style-grid）/041
  （hidden）/046（tag:tabs）/012（native-unstyled）；shell 五件单列
  Covered 不入分母。
- **缺项实现面**：
  - ①012：`native_style_token` 兜底 `_ => "native-unstyled"`
    （coverage.rs:762）——012 重做后新类未入 StyleClass→token 表。
  - ②tabs：`View::Tabs{labels,contents,selected,position,on_select,
    style,variant}`（view.rs:796-805；TabsVariant default/enclosed =
    PLAN-641 view.rs:192-217）；`native_kind_of => "tabs"`
    （coverage.rs:482）但 kinds 无（:176-212）→ 缺项；投影器落
    `other` 兜底占位盒（native_projector.rs:1447-1460）；VM 轨
    convert_tabs 在场（aura_view_builder.rs:1750，tabslist 族子件
    折叠 :1398-1433）；a2r codegen tab 直通映射（rust.rs:5680-5681）
    且 §1.8 在册"tab 映射断裂维持"。
  - ③hidden：`SC::Hidden => "hidden"`（coverage.rs:745，:737-743
    注释归"语义承载未实现面"）；投影器零消费（node_style_of
    :1799- 不映射）；真源 018×2 / 041×3；prefixes :230-271 未放行。
  - ④样式 grid：`SC::Grid|GridCols|GridRows => "style-grid"`
    （coverage.rs:744）；`View::Grid` 变体臂已存在且入册 layouts
    "grid"（:222 + walker 两遍网格 native_projector.rs:1160）——缺
    style 路径（容器带 `grid-cols-2` 类走不到 Grid 臂）；真源 024。
  - ⑤定位族：`SC::Absolute/Fixed/Sticky` + `Top/Left/Right/
    BottomOffset` + `ZIndex` token（coverage.rs:746-753；`Relative`
    已放行 :733 注释明示"absolute+offset 族仍 not-yet"）；投影器
    单遍流式 (x,y) walker（layout_view_node :913 起）无定位通道；
    §1.8 已知边界在册；真源 018（fixed×2/z-40）、021（sticky×2/
    top-0/z-30）。
  - 对照已清偿：opacity 已放行（:699/:757 + prefixes :266-270）；
    popover/mousearea/windowthumbnail/workspacepreview 四 kind 已入册
    （:200-209，029）。
- **翻转点与仪器**：`resolve_native_frame_mode`（client_entry.rs:
  113-147）——Auto → scan×judge（:122-123），Covered 臂 :124-134 返
  `FrameMode::Pixels`（:128）+ 观测行 "queue-covered, default flip
  pending ramp v3 data gate"（:130-133）；翻转 = :128 一行。仪器 =
  `native_flip_coverage_data_row`（coverage.rs:1171-1278；eprintln
  数据行 :1258-1263；**防漏断言 `assert!(!flip)` :1270-1273——达标
  即红，逼翻转向下任**）；样本 = examples 目录运行时扫描（:1175-
  1230，当前 42 目录 36 样本）；shell 单列仪器 shell_pack_native_
  covered（:1288-1372）。复现命令：`cargo test -p auto-lang
  --features ui-iced --lib native_flip_coverage_data_row --
  --nocapture`。
- **M7 批次上下文**：台账 M7-a 行（b2e9cce）——M7-c②（jade-garden
  tab×27 / auto-musk tab×16）依赖本计划 tabs；M7-d 收口以翻转为前提
  之一。

**specs 现状**：协议权威 v1.11 现行（031 §1.11——031 在 review，其
canonical 随 merge；本计划 SD-01 增量号为 §1.12 以 merge 序为准）；
P026-D3 在册（KNOWN-DEBT :2278）。

## 5. 详细设计

### 5.1 T-01 深水调查定案（决策产物）

- **D1 定位族口径**（本计划核心决策）：候选 A = **分层**（absolute +
  offset 真渲 = 覆盖序 ops 锚定父容器坐标[Popover Point 锚先例] +
  fixed/sticky 降级放行随注[in-flow 渲染，opacity/overflow 先例，
  真渲债登记]——**倾向**：018/021 判定翻绿且 absolute 语义正确，
  fixed/sticky 保真边界显式）；B = 全真渲（fixed/sticky 视口锚定 =
  帧语义/宿主层牵动——深水，可能拖垮波次）；C = 全降级放行（最快
  翻转但 absolute 也错位——I3 边界过宽）。以 018/021 真源逐元素
  核对语义面定案。
- **D2 tabs 的 a2r 断裂半句**：同批修（tab/tab 直通映射断裂 →
  `View::Tabs` 构造 + on_select——**倾向**，M7-c② 前置且 046 为
  a2r 轨样本）vs 仅 VM 轨（翻转门不受阻——扫描走 VM 轨；a2r 侧留
  M7-c）。定案含 046 a2r 重生成编译验证。
- **D3 hidden 语义**：display:none = 子树整体不渲染不占位（NodeStyle
  布尔 + 各布局臂跳过返零尺寸 + 聚合连锁逐臂核——flex-1/尺寸推导/
  居中两遍法）；非 visibility:hidden（占位留白）——语义以真源用例
  核对（018/041 的 hidden 用法）。
- **D4 样式 grid 分岔**：GridCols(n) 解析 → Container 臂检测样式
  grid → 复用 Grid walker（:1160）；GridRows/gap 语义子集（024
  真源核对）；与 `View::Grid` 变体臂的互斥/优先级。
- **D5 012 映射臂**：012 新类枚举实况（重做后用类）→ native_style_
  token 补臂或确认 parser 类缺口（落 parser 线则记债）。
- **D6 翻转执行口径**：判据 = judged ≥95% + 缺项全在册（026 D3
  沿承）；翻转动作清单（:128 一行 + assert 反转 + 观测行文案 +
  台账 M7-a 裁定行 + 抽样 e2e[翻转后 auto 档 examples 抽样真机/
  集成]）；不过门出口的数据行更新义务。

定案记录追加 `### 5.1 定案记录`，作为 T-02..T-06 依据。

### 5.1 定案记录（T-01，2026-09-19，lang-032 worktree @ master 0c6b03fd3）

**基线实测**（仪器复跑，含随附测试面两修复——见本节末）：overall
16/37 = 43.2%（047-bp-admin 新例入场 36→37）、**judged 16/22 = 72.7%**
（仪器桶 15 = parse-fail×10 + bridge-fail×4 + no-widget×1）、六缺项与
p029 报告逐字一致：018（fixed/hidden/z-1）/021（sticky/top-1/z-1）/
024（style-grid）/041（hidden）/046（tag:tabs）/012（native-unstyled）。
补齐后 judged 预期 22/22 = 100% ≥ 95% 过门。

- **D1 定位族 = A 分层**（采倾向）。absolute+offset 真渲：
  `layout_view_block`/`layout_view_container` 内流循环完成后（父块尺寸
  已量出）延迟放置——absolute 子级不入流不占 cursor，临时 ctx 走线量
  尺寸（Popover tmp 先例 native_projector.rs:1336-1352）→ 锚点 = 父
  内容盒原点 + offsets（top-/left- 直加；bottom-/right- 按父盒尺寸
  反算——018 bookshelf.at:61 "absolute bottom-0 left-0 right-0" 徽条
  即此形态）→ `shift_draw_op`/hit 平移后**追加主序之后**（覆盖序，
  Popover 平移臂 :1370-1383 先例）。z 相对层级 = 同层追加序内按 z
  排序；完整栈序 out of scope（I3 随注）。fixed/sticky 降级放行：
  prefixes +"fixed"/"sticky" + 渲染 in-flow no-op（opacity/overflow
  先例 :266-270/:702-712）；真源核对 018 app.at:24（"hidden md:flex
  flex-col fixed inset-y-0 z-40" 桌面侧栏）、021 app.at:27（"sticky
  top-0 z-30" 导航条）——判定翻绿、渲染保真边界显式随注；真渲债
  §10-④ 登记。
- **D2 tabs a2r 断裂 = 同批修**（采倾向）。断裂证据：`tag_to_view_fn`
  "tabs"→"tabs" / "tab"→"tab"（rust.rs:5717-5718）发射 `View::tabs()`
  （缺 `Vec<String>` 实参）/`View::tab()`（不存在）——046 a2r 轨不可
  编译。修复：`generate_view_tree` 专属 `tag == "tabs"` 臂（input/
  slider/select 先例 rust.rs:2870/3706/3763），按 convert_tabs 契约
  （aura_view_builder.rs:1410-1439：tabslist 透明 / tabstrigger 标签+
  value / tabscontent 内容 / tab 平铺）发射
  `View::tabs(vec![..]).contents(vec![..]).selected(运行时 position
  表达式).variant(..).on_select(闭包物化载荷消息——select 臂 :3757
  -3762 先例).build()`；"tabs"/"tab" 断裂映射移除（026 T-02 link/a
  先例，残余落 `_ => "col"` 兜底）。VM 轨扫描面补：`scan_native_node`
  对 `View::Tabs` 落 `_ => {}`（coverage.rs:586）——contents 子树递归
  缺口同批清偿（防漏钉②纪律）。
- **D3 hidden = display:none + 响应式覆盖规则**。SC::Hidden → 子树
  整体不渲染不占位。覆盖规则：parser 剥响应式前缀（class.rs:748
  sm/md/lg/xl/2xl → rest）——"hidden md:flex" 解析为 [Hidden, Flex]
  并存，CSS 桌面档 md:flex 胜（Tailwind 生成序 + 桌面目标假设；Inline
  文档注释 class.rs:191 即此模式在案）→ 规则 = Hidden 在场且**无**
  display 族类（Flex/FlexRow/FlexCol/FlexColReverse/Block/Inline/
  InlineBlock/InlineFlex）才跳过。实现：NodeStyle.hidden 布尔 +
  `layout_view_node` 单一 choke（:911 style 取出后、arm 分派前返
  `Laid{(0,0)}` 零 ops）——全布局臂聚合连锁（flex-1/居中两遍法）零
  贡献自然成立。真源：018 app.at:24（hidden md:flex=可见）/:65
  （md:hidden=隐藏）；041 status_bar.at:8,32,33 + app.at:123-129。
- **D4 样式 grid = 堆叠族/容器单 choke 分岔**。024 真源 =
  `col (style: "grid grid-cols-2 gap-2")`（app.at:138）——**Column**
  带类非 Container：分岔 choke = `layout_view_group`（Row/Column/List）
  + `layout_view_container` 入口检测 grid_cols → 复用 Grid walker
  （:1160-1208 提取 helper，与 View::Grid 变体臂共用；变体臂字段优先，
  互斥不冲突）。语义子集：GridCols(n) 主驱（等宽列 row-major、行高
  = 行内最大、gap 通道既有）；GridRows(m) → cols=ceil(cells/m)；裸
  Grid 无 cols → cols=1。token "style-grid" 维持 + prefixes 精确放行。
- **D5 012 映射 = SC::SelfCenter 补臂**。实测缺口类 = `self-center` →
  SC::SelfCenter（class.rs:1321；iced 适配器 :1197 消费交叉轴自对齐）
  ——coverage token 表无臂落兜底 "native-unstyled"（coverage.rs:764）；
  012 App 视图 unit_label/冒号分隔标签用（app.at:257/348 等）。其余
  新类（drop-shadow-md/uppercase/tabular-nums）parser 无解析臂整类
  跳过 → 不产 token 非缺口。修复：token 表补臂 + prefixes "self-"
  放行 + 投影器消费 = NodeStyle.center_children（layout_view_block
  :857 子级居中通道既有——真渲轻量，非降级放行）。
- **D6 翻转判据 = judged ≥95% + 缺项全在册**（026/029 沿承）。仪器
  需升级：当前只算 overall（coverage.rs:1257-1259）——补 judged 率
  （剔除仪器桶 parse-fail/extract-fail/bridge-fail/no-widget）。过门
  翻转清单：①client_entry.rs:156 Covered 臂 Pixels→Commands（行号较
  计划 drafting 时 :128 漂移，031 合并所致）；②观测行 → flipped@ramp3
  文案（:158-161）；③coverage.rs:1271-1275 防漏断言反转 `assert!
  (!flip)` → `assert!(flip)`（翻转后跌破门即红——降级需显式裁定，
  非静默回归）；④台账 M7-a 裁定行 + p032 数据行报告；⑤auto 档抽样
  e2e（缺省 queue 生效断言）。不过门出口：数据行更新 + 显式不翻留痕。

**随附 master 测试面修复**（本计划验证面解锁——p029 记录的仪器命令
`--features ui-iced --lib` 在 master 编译失败 E0432：022 合入的
terminal/iced/widget.rs:1236 `use iced_test::simulator` 未挂特性门
（iced_test 属 `iced-layout-tests` 集 Cargo.toml:71）→ 补
`#[cfg(feature = "iced-layout-tests")]` 门；`--features
iced-layout-tests` 亦断：layout_tests.rs:2176/2328/2435 三处
`View::Scrollable` 字面量缺 PLAN-656 新字段（view.rs:739-744
axes/scrollbar_policy/controller）→ 补缺省（ScrollAxes::Y/Auto/
None，vnode_converter.rs:1029 full-path 先例）。归因 = 022/656 合并
门未跑对方特性集；随注 KNOWN-DEBT T-08 收口）。

### 5.2 五族补齐（T-02/T-03/T-04/T-05）

- **T-02 小改族**：①012 映射臂 + ③hidden（prefixes 放行 + NodeStyle
  标志 + 布局臂跳过 + golden——018/041 样本）。
- **T-03 tabs kind**：kinds 入册 + `View::Tabs` 投影臂（托盘/选中态/
  内容区/on_select 命中——Popover 臂结构先例）+ a2r 断裂修复（依
  D2）+ 046 golden + a2r 重生成编译（若 D2 同批）。
- **T-04 样式 grid**：token 放行 + node_style_of 解析 + Container
  臂分岔复用 Grid walker + 024 golden。
- **T-05 定位族**：依 D1 口径（分层倾向）——absolute+offset 覆盖序
  真渲 + fixed/sticky 放行随注 + z 相对层级（覆盖序内）；018/021
  golden（含降级随注断言）。

### 5.3 复测与翻转（T-06）

仪器复跑 → dual-exit：过门翻转全清单（§5.1 D6）+ 抽样 e2e；不过门
数据行更新 + 显式不翻。

### 5.4 e2e 与收口（T-07/T-08）

- e2e：六例 queue 显式档全链（046/024/018/021/041/012——帧断言 +
  交互闭环[tabs on_select/样式 grid 布局]）；翻转腿（若翻）：auto
  档抽样 examples 真机/集成（缺省 queue 生效断言）。
- 文档：§1.12 增量；P026-D3 处置；新债（fixed/sticky 真渲——若分
  层）；台账 M7-a 行更新（M7-c② 解锁注记）。

### 规范增量

| delta_id | add/modify/retire | docs/specs/... target | before/after rule | rationale | acceptance IDs |
|---|---|---|---|---|---|
| SD-01 | add | auto-lang/docs/design/autoui/desktop-protocol-v1.md（§1.13 v1.13 增量——复审定稿：031 先占 §1.12，drafting 时预计号依 merge 序实取） | before：v1.12 覆盖下 judged 六缺项在册（tabs/hidden/style-grid/定位族/012 映射），auto 缺省 independent（"flip pending ramp v3"观测行）；after（已验证实现态）：五族口径入册（tabs kind 全链 + hidden 跳过语义[display 族响应式覆盖] + 样式 grid 分岔[GridCols 优先/GridRows 反推/裸 Grid 等价堆叠] + 定位族分层[absolute 真渲 + fixed/sticky 降级随注——依 D1 定案]）+ D5 族运行时面（SelfCenter/Inset/LineClamp/FlexWrap）+ 翻转裁定（缺省 queue，judged 22/22 = 100%）+ 数据门口径升级（judged 计算 + 防漏断言反转）+ 运行时口径差（component.view() vs 静态壳——018/041 真 not-yet 拒收留痕）+ 保真边界随注——PROTOCOL_VERSION 仍 1（零 wire 变体） | 协议权威版本化收录 ramp v3 与缺省裁定 | AC-01/02/03 |
| SD-02 | modify | auto-os/docs/plans/autos-desktop-program.md | before：M7-a 行 = 待执行；after：M7-a 行更新（五族口径 + 翻转结果 + M7-c② tabs 解锁注记）+ 裁定行（若翻） | 桌面程序台账 M7 批次 | AC-06 |
| SD-03 | modify | auto-lang/docs/plans/KNOWN-DEBT-AND-RISKS.md | before：P026-D3 = 维持 independent 复测未达标；after：处置（翻则核销；不翻则数据行更新）+ 新债（fixed/sticky 真渲——若分层） | 债账 | AC-06 |

零 spec 影响的变更不存在（覆盖集/缺省裁定为协议级知识）；ledger 随
merge 沉淀。

## 6. 测试设计

- **单测（coverage）**：五族判定（tabs/hidden/style-grid/定位族
  [依 D1] 放行 + 012 映射）；防漏钉矩阵更新（tabs × 投影臂双向）；
  既有 golden 零漂移。
- **单测（native_projector）**：tabs 臂 golden（default/enclosed 两
  变体 × 选中态切换 on_select 命中）；hidden 跳过（零尺寸 + 布局
  聚合连锁——flex-1/居中两遍法）；样式 grid（GridCols 分岔布局
  golden——与 View::Grid 变体同构断言）；定位族（依 D1：absolute
  覆盖序坐标锚 + fixed/sticky in-flow 降级随注）。
- **仪器**：native_flip_coverage_data_row 复跑（数据行 + 防漏断言
  行为按翻转态）；shell_pack_native_covered 回归（五件 Covered 维持）。
- **e2e**：六例 queue 显式档全链 + 翻转腿（auto 档抽样——若翻）；
  a2r 重生成编译（046——若 D2 同批）。
- **回归门**：desktop_protocol/session/stage3 + `cargo t -p auto-man
  rust_ui` + auto-os 桌面 smoke。

## 7. 验收标准

- **AC-01 五族补齐**：六例（046/024/018/021/041/012）native 判定
  **Covered**；渲染断言按 D1-D5 定案（真渲 golden 或显式降级随注
  ——fixed/sticky 分层口径下 in-flow + 随注可验）。验证：单测 +
  golden + 复测仪器数据行。
- **AC-02 翻转 dual-exit**：复测数据行落盘——judged ≥95% 过门：
  `resolve_native_frame_mode` Covered 臂 = Commands + 防漏断言反转
  + 台账裁定行 + auto 档抽样 e2e（缺省 queue 生效）；不过门：显式
  不翻 + 缺项清单更新。两出口均 pass，禁无数据翻转。验证：仪器 +
  client_entry 断言 + e2e 腿。
- **AC-03 tabs 全链**（依 D2）：VM 轨判定+投影+on_select 闭环；a2r
  断裂若同批——046 重生成编译过 + tab 构造 golden。验证：单测 +
  e2e。
- **AC-04 防漏钉与零回归**：防漏钉矩阵含新 kind/放行项双向钉；既有
  golden/套件零漂移；shell 五件 Covered 维持；解释态分表不动
  （I4）。验证：防漏钉 + 回归门。
- **AC-05 e2e 全链**：六例 queue 显式档渲染/交互断言（tabs 切换/
  样式 grid 布局/hidden 不渲染）留痕。验证：e2e + assets/032/。
- **AC-06 文档台账**：§1.12 + P026-D3 处置 + 新债 + M7-a 行更新落盘
  互链。验证：文档交叉引用可解析。

## 8. 执行步骤

依赖序：T-01 → {T-02, T-03, T-04 并行} → T-05 → T-06 → T-07 →
T-08。lang worktree `D:/autostack/.wt/lang-032/auto-lang`；os
`D:/autostack/.wt/os-032/auto-os`。**建议 031 merge 后开工**（同
crate 串行，非硬前置）。

- **T-01 [x] [lang] 深水调查与定案**
  文件：`coverage.rs`（token 表/prefixes/仪器）、`native_projector.rs`
  （walker/臂先例）、`view.rs`（Tabs）、`ui_gen/rust.rs`（tab 映射）、
  018/021/024/041/046/012 真源 + §5.1（写面）。
  动作：D1–D6 定案（定位族口径核心）。
  产物：`### 5.1 定案记录`（file:line 证据）。
  验证：定案完备；复审通过。
  → 全 AC 前置。新路径：定案产物。
  [✅ 已完成] §5.1 定案记录六条全落（D1 分层/D2 同批修/D3 display:none+
  响应式覆盖/D4 堆叠族单 choke/D5 SelfCenter/D6 judged 口径+仪器升级）；
  基线仪器复跑实测（overall 16/37=43.2%，judged 16/22=72.7%，六缺项
  逐字对齐 p029）；随附 master 测试面两修复（022 iced_test 特性门 +
  656 Scrollable 字面量）解锁仪器命令——commit plan-032-dev。
- **T-02 [x] [lang] 012 映射 + hidden**
  文件：`coverage.rs` + `native_projector.rs`。
  动作：§5.2 T-02；D3/D5。
  验证：单测 + golden（018/041/012 样本）绿。
  → AC-01。
  [✅ 已完成] D5=SelfCenter 补臂+center_children 真渲；D3=prefixes⑧
  +NodeStyle.hidden 单一 choke + 块流 gap 序整段跳过 + display 族响应式
  覆盖（hidden md:flex 桌面可见/md:hidden 隐藏）。测试：hidden_display_
  none_golden（对照金样：布局与 B 不存在逐坐标等价）/hidden_responsive_
  override_visible/self_center_aligns_golden/native_gate_examples_012_
  041_covered 全绿；防漏钉 coverage_gate 翻样 truncate；desktop_protocol
  模块 194 过 2 红——两红（covered_elements_within_target_set 的
  imagesurface 登记漂移 + demo parity 隔离序）经 stash 基线核验均为
  master 既有红，非本任务引入（解释态域，I4 出界，随注记录）。仪器
  judged 16/22 → 18/22（012/041 翻绿，018 缺项收敛 fixed/z）。commit
  plan-032-dev。
- **T-03 [x] [lang] tabs kind**
  文件：`coverage.rs` + `native_projector.rs` + `ui_gen/rust.rs`
  （依 D2）。
  动作：§5.2 T-03。
  验证：tabs golden + on_select 单测 + 046 a2r 编译（若同批）。
  → AC-01/03。
  [✅ R1-F2 已修复] 孪生收窄 button-only（commit 06243bdb3）
  ——tabs variant 发射 tf/tt 双档恢复绿；顺带治愈 master 既有红
  display_family（icon size 传导恢复）；046 编译复跑过（22s）；ui-iced
  档回归绿。tf/tt 定稿清单 = mouse_area + a2r×4（master 基线既有）+
  ffi_dual_019（4 跑 2 红 2 绿、隔离×3/模块整组绿 = 重负载 flaky 谱系
  在册，非回归）。
  [原完成记录] kinds+tabs + View::Tabs 投影臂（等宽托盘/选中 bg/fg 差分/
  default+enclosed[选中下划线]两变体/Top+Bottom/TabSelect 命中 →
  TabsSelectCallback.call(index) 物化——VM 轨 value 串在回调内包装）+
  scan contents 递归补漏。a2r：generate_view_tree 专属臂（labels/
  contents 折叠、value 绑定运行时 position、variant full-path、onselect
  闭包物化载荷——select 臂先例）+ tag_to_view_fn tabs/tab 断裂映射移除。
  测试：tabs_tray_golden_and_select_loopback（default/enclosed × 点击
  切换闭环）+ test_tabs_codegen_view_tabs_folding（发射形状）+
  **test_tabs_codegen_046_compiles（真源 046 全量生成 cargo build 过
  209s，#[ignore] e2e 档本会话显式跑）** + 046 门 Covered + 防漏钉矩阵
  夹具补 Tabs。回归：desktop_protocol 195 过 2 既有红；ui_gen 791 过
  24 红 = master 基线同数（零新回归）。仪器 judged 19/22。commit
  plan-032-dev。
- **T-04 [x] [lang] 样式 grid**
  文件：`coverage.rs` + `native_projector.rs`。
  动作：§5.2 T-04；D4。
  验证：024 golden + 与 View::Grid 同构断言。
  → AC-01。
  [✅ 已完成] prefixes⑩ style-grid + NodeStyle.grid_cols/grid_rows +
  layout_view_block 入口单一分岔 choke（group/container/scrollable 全路
  径经此——较 D4 原案双 choke 更收敛）复用 Grid walker（:1160 提取为
  layout_grid_cells 单源，View::Grid 变体臂同构零变化）。解析序：
  GridCols(n) 优先 / GridRows(m)→ceil(cells/m) / 裸 Grid 不记档（无模
  板单列与纵向堆叠视觉等价）。测试：style_grid_golden_isomorphic_to_
  variant（同构断言[含 gap-2=Tailwind Fixed(2)→8px 刻度注记] + grid-
  rows-2×4=2 列）+ 024 门 Covered。回归：desktop_protocol 196 过 2 既
  有红。仪器 judged 20/22（024 翻绿）。commit plan-032-dev。
- **T-05 [x] [lang] 定位族**
  文件：`coverage.rs` + `native_projector.rs`。
  动作：§5.2 T-05；D1 口径落地。
  验证：018/021 golden（含降级随注断言）。
  → AC-01。
  [✅ 已完成] absolute+offset 真渲：place_absolute_children 延迟放置
  （脱离流零占位不贡献父高/left-top 优先·right-bottom 按父盒尺寸反算
  [堆叠族自身 fixed 高为既有丢弃边界——锚定按内容盒自洽随注]/同层 z
  稳定排序/覆盖序追加主序后——Popover 平移臂先例/detached_ctx 快照
  透传）；fixed/sticky 降级放行（in-flow no-op——真渲债 §10-④ 另立）；
  prefixes⑪ 定位族八项。golden：absolute_overlay_golden（bottom 反算/
  过约束 left 胜/z 序/覆盖序/零流内占位）+ fixed_sticky_degraded_
  inflow_golden（对照等价——降级非错绘钉）。018/021 门 Covered——
  **六缺项全清，仪器 judged 22/22 = 100%**（overall 22/37 = 59.5%，
  防漏断言维持绿——judged 口径升级属 T-06）。desktop_protocol 198 过
  2 既有红。commit plan-032-dev 1a2062771。
- **T-06 [x] [lang] 复测 + 翻转（dual-exit）**
  文件：`client_entry.rs`（:128 门后）+ `coverage.rs`（assert 反转—
  若翻）+ 数据报告。
  动作：§5.3；D6 全清单。
  验证：仪器数据行 + 断言态一致 + 台账行。
  → AC-02。
  [✅ 已完成·过门翻转] 复测 judged 22/22 = 100% ≥ 95% → 翻转：①
  Covered 臂 Pixels→Commands（client_entry.rs:156）；②观测行 native
  auto -> queue … flipped@ramp3；③仪器 judged 口径升级（剔除 parse-
  fail/extract-fail/bridge-fail/no-widget 桶）+ 防漏断言反转 assert!
  (flip)（跌破门即红——降级需显式裁定）；④p032 数据行报告落盘
  （reports/p032-native-flip-row.md：对差表/保真边界随注/翻转清单；
  台账 M7-a 裁定行随 T-08 SD-02 落笔）；⑤auto 档抽样 e2e = T-07
  翻转抽样腿。翻转态钉测试 native_auto_default_flipped_to_queue
  （Auto×Covered=Commands + NotCovered 降级路径不变）。回归：199 过
  2 既有红；shell 五件 Covered 维持（I2）。commit plan-032-dev。
- **T-07 [x] [lang+os] e2e 与回归**
  文件：lang `stage3.rs`（六例腿 + 翻转抽样腿）+ assets/032/；os
  smoke 如需。
  动作：AC-01..05 逐条留痕。
  → AC-03/05。
  [✅ 已完成] p032_ramp3_flip_arm（AUTO_DESKTOP_E2E=1 实跑全绿）：四
  进程腿（012[auto 翻转抽样——子进程 resolve 裁决 Commands +
  flipped@ramp3 观测行]/021/024/046[queue 档]）首帧钩子 + frame_mode
  =Commands + 046 tabs on_select 交互闭环（Beta 点击 → panel 切换）+
  024 grid 几何（Line/Bar 同行异列）+ assets/032/ 四帧留痕；018
  （truncate）/041（codeeditor）运行时视图真 not-yet → 拒收留痕腿
  （ensure_covered Err 载荷逐字断言——I3/AC-04 e2e 面）。**运行时口径
  发现**（native_gate_runtime_views_of_six 钉）：生产 auto 裁决消费
  component.view()（路由解析后）——较仪器静态 App 壳扫描宽；D5 族
  运行时面补臂（Inset 四槽真渲组合 absolute/LineClamp 放行随注/
  FlexWrap token 归 flex- 前缀）清偿 021/024 两例；018/041 家族出界
  留债（T-08 KNOWN-DEBT）。回归：desktop_protocol 201 过 2 既有红 +
  auto-man rust_ui 25/25 + 仪器 judged 22/22 维持 + shell 五件
  Covered 维持；os smoke 以 DesktopSession 级 e2e 承载（os 侧本计划
  零代码变更——仅台账文档）。commit plan-032-dev 9c33707cf。
- **T-08 [x] [lang+os] 文档与台账收口**
  文件：lang `desktop-protocol-v1.md`（§1.12）+ KNOWN-DEBT；os 台账
  M7-a 行 + 互链。
  动作：SD-01..03 落笔。
  → AC-06。
  [✅ 已完成] SD-01 = §1.13 v1.13 增量（031 先占 §1.12——drafting 时
  预计依 merge 序，实取 §1.13）+ 版本表行：五族口径/翻转裁定/数据门
  口径升级/运行时口径差/保真边界随注（lang 47b86d730）。SD-03 =
  KNOWN-DEBT：P026-D3 核销（翻转收束）+ 新债 P032-D1（定位族真渲
  分层）/P032-D2（line-clamp·flex-wrap no-op）/P032-D3（运行时口径
  缺口 truncate/codeeditor→M7-c）/P032-D4（特性集互盲合并门——022/
  656 随附修复归因）。SD-02 = os 台账 M7-a 行 ✅ 交付裁定 + P026-D3
  核销注记 + **M7-c② tabs 解锁注记**（jade-garden/auto-musk 可开工）
  + 裁定行（judged 22/22、p032 报告/§1.13 互链——os plan-032-dev
  12775b8）。回归门补录：session:: 86/86 绿。

## 9. 复审记录

- 2026-09-19 /auto-plan:work R1-F2 修复收口：`stage: work`，PLAN-032
  rev 1（needs_fix 重入）。`outcome: pass`——F-2 单点修复：
  with_button_preset 的 no-`ui` 孪生收窄 button-only（非 button 恒等
  返回；button 的 variant/size preset 剥除保留——与 ui 孪生范围及
  自身"恒等"文档契约对齐）。`code_commit`: lang plan-032-dev
  47b86d730→R1-F2 修复提交。`task_ids`: T-03（重开→闭合）。`evidence`:
  ①tabs 发射测试无 ui 档（nextest lib 裸跑）绿；②tf 全量 no-fail-fast
  两轮：1-2 红 = mouse_area（master 基线既有）± ffi_dual_019（2/4 轮
  现、隔离×3 绿、ffi_dual 模块整组 23/23 绿——重负载 flaky，master
  在册红谱系含其）；③tt 全量 5 红 = master 基线 6 红的子集
  （**display_family 被本修复治愈**——icon size 传导恢复的正向效应，
  净 -1）；④ui-iced 档回归（tabs golden/发射/门/仪器）绿；⑤046 真源
  a2r 编译复跑过（22s）。`blockers`: 无。`next: review`（R2——增量
  审 F-2 闭合 + 双档清单；其余 AC R1 已 pass 且代码未动）。**随注**：
  ffi_dual_019 的批内非确定性（同码 2 红 2 绿）建议随 B 程序 CI 化时
  定 attribution——本计划不立项。

- 2026-09-19 /auto-plan:review R1（实施会话内复审——独立性受限已在
  裁定前声明；裁定从工件重建：代码态/测试复跑/双基线对拍，不采信执行
  者摘要）：`stage: review`，PLAN-032 rev 1。`outcome: needs_fix`。
  `reviewed_commit`: lang plan-032-dev 47b86d730（base 0c6b03fd3，worktree
  干净复核）；os plan-032-dev 12775b8（base 9d6941a）；计划文档 os main
  bf05d05。`acceptance_results`：AC-01 pass（仪器 judged 22/22 = 100%
  复跑 + 六例门钉绿）/AC-02 pass（Covered 臂 Commands :157 + 观测行
  flipped@ramp3 + assert!(flip) 反转态 + 翻转态钉 + p032 报告在案）/
  AC-03 **partial**（VM 轨投影 golden + 046 真源 a2r cargo build 复跑
  70s 过；但发射测试 test_tabs_codegen_view_tabs_folding 在 cargo tf/tt
  档红——见 F-2）/AC-04 pass（防漏钉矩阵 + shell 五件 + 八 family
  golden 复跑绿；零回归对拍见下）/AC-05 pass（e2e 二跑全绿；一跑红
  归因 = 与后台全量门并发 CPU 争抢的环境因素——串行绿，随注）/AC-06
  pass（§1.13 锚点 + 版本表行 + P032-D×4 + P026-D3 核销 + 台账裁定行
  互链全解析；SD-01 表 §号失配已复审定稿修正）。
  `findings`：**F-2 [major·AC-03/T-03]**——tabs `variant` prop 发射在
  无 `ui` 特性档丢失：with_button_preset 的 no-`ui` 恒等孪生
  （ui_gen/rust.rs:5166）无视 tag 无条件 remove("variant")/remove
  ("size")——ui 孪生（:5097）只对 button 合并 preset；PLAN-641 起
  tabs 亦持 variant 词表 → tf（无 ui-iced）/tt（test-trans）档下
  `tabs variant:"enclosed"` 生成物丢形态。证据：tf 分支 4 红 vs master
  2 红（净增 = 本测试）；tt 7 vs 6（同）；隔离复跑 + 生成码 diff
  （仅 .variant 缺席）。修正：孪生收窄 `if tag != "button" { return
  Borrowed }`（对齐其文档契约"恒等"），tf/tt 复绿 + 046 编译复跑为
  完成判据。**F-1 [bookkeeping·已复审定稿]**——SD-01 增量表残留
  drafting 时 §1.12 号（031 先占 §1.12，实落 §1.13）+ after 规则未含
  D5 族运行时面——已修正。`evidence`: 双基线对拍（branch vs master
  @0c6b03fd3 组内 detached worktree）——tf 净增 1（tabs 发射）、tt 净增
  1（同）；ffi_dual_019 分支全量档红但隔离双跑绿（37s/7s）= 负载
  flake 非回归；既有红清单（mouse_area/display_family/a2r×4）两基线
  同集。**运维事件披露**：复审中一次 `git stash pop` 误弹他会话 stash
  （stash 列表全仓共享——"On plan-637-dev: 026-final"）入本 worktree，
  已 `git reset`+`checkout` 完整恢复（0 脏文件复核）；对方 stash 因
  pop 冲突不删除机制完好未损（stash@{0} 复核在案）。master 期间前移
  0c6b03fd3→b69c7344c（他会话合入）——merge 时调和。`next: work`（携
  F-2 单点修复；修复后 tf/tt 复绿 + 本记录补 R2）。

- 2026-09-19 /auto-plan:work 执行收口：`stage: work`，PLAN-032 rev 1。
  `outcome: pass`——T-01..T-08 全闭环：D1–D6 定案（§5.1）→ 五族补齐
  （T-02 012/hidden、T-03 tabs 全链 + a2r 断裂修复、T-04 样式 grid、
  T-05 定位族分层）→ 复测 **judged 22/22 = 100% ≥ 95% 过门翻转**
  （T-06：Covered 臂 Commands + flipped@ramp3 + 仪器 judged 口径 +
  防漏断言反转）→ 六例 e2e + 运行时口径发现与 D5 族补臂（T-07）→
  SD-01..03 收口（T-08）。`code_commit`: lang plan-032-dev
  87ed66ebd→47b86d730（七提交）；os plan-032-dev 12775b8。`task_ids`:
  T-01..T-08。`evidence`: p032-native-flip-row.md + assets/032/ 四帧 +
  防漏钉矩阵/门/golden/翻转态钉/运行时口径钉测试集（desktop_protocol
  201 过 2 既有红[stash 基线核验]；ui_gen 791 过 24 红 = master 基线
  同数；auto-man rust_ui 25/25；session 86/86；046 a2r 真源 cargo
  build 过 209s）。`blockers`: 无。`next: review`（六 AC 就绪：AC-01
  仪器+六例门钉/AC-02 翻转三证+钉/AC-03 tabs 双轨+046 编译/AC-04 防漏
  钉矩阵+零回归+shell 五件/AC-05 e2e+assets/AC-06 §1.13+债+台账互链）。
  悬置决策 §10 ①–④ 已随 T-01 定案销项；运行时口径差（新发现）与
  D5 族补臂按"等价局部实现调查调整在案"路径处置并全留痕（§5.1/复审
  本条/p032 报告/KNOWN-DEBT P032-D3）。

- 2026-09-19 /auto-plan:new 起草交接：`stage: new`，PLAN-032 rev 1
  （M7-a 首件——台账终态批次 b2e9cce 登记）。`outcome: pass`（合同
  完整：六缺项实现面逐族 file:line 在案（token 表/walker/变体/
  断裂映射）、翻转点与防漏断言、仪器复现命令、工作量分层与深水
  预判齐备）；`next: work`（前置 030 ✅；建议 031 merge 后开工）。
  悬置决策 §10（①–④），①定位族口径为核心（分层为倾向），均不阻塞
  T-01。

## 10. 待澄清事项（T-01 定案销项——原四问全决，见 §5.1）



- **①（T-01 D1）** 定位族口径：分层（absolute+offset 真渲 +
  fixed/sticky 降级放行随注——推荐）vs 全真渲（深水——fixed/sticky
  视口锚定牵帧语义/宿主层）vs 全放行（I3 边界过宽）。
- **②（T-01 D2）** tabs a2r 断裂同批修（推荐——M7-c② 前置 + 046
  为 a2r 样本）vs 仅 VM 轨（翻转门不受阻）。
- **③（T-01 D6）** 翻转判据沿承（judged ≥95% + 缺项全在册）；新例
  入场稀释不配平（仪器如实——026/029 口径，样本集口径出界）。
- **④（债登记）** fixed/sticky 真渲 + z 完整栈序（若分层口径）——
  新债随注，真渲另立。
