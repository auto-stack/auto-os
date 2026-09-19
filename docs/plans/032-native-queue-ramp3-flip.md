---
plan_id: PLAN-032
status: drafting               # drafting → executing → execution_done → reviewed → archived
feature_name: native-queue-ramp3-flip
author: [agent]
created_at: 2026-09-19
updated_at: 2026-09-19
plan_revision: 1

# /auto-plan:review 结束时填写：
supersedes_spec_components: []
new_spec_components:
  - auto-lang/docs/design/autoui/desktop-protocol-v1.md   # §1.12 v1.12 增量（review 定稿）
touched_goals: []

affects:
  - auto-lang/crates/auto-lang/src/ui/desktop_protocol/coverage.rs        # 六缺项判定/放行 + kinds/prefixes + 翻转仪器
  - auto-lang/crates/auto-lang/src/ui/desktop_protocol/native_projector.rs # tabs/hidden/样式 grid/定位族（依 D1 口径）投影臂
  - auto-lang/crates/auto-lang/src/ui/desktop_protocol/client_entry.rs    # 翻转点（Covered 臂 :128，门后）
  - auto-lang/crates/auto-lang/src/ui_gen/rust.rs                         # tabs/tab a2r 断裂映射（依 D2 定案）
  - auto-lang/docs/plans/KNOWN-DEBT-AND-RISKS.md                          # P026-D3 处置 + 新债随注
  - auto-os/docs/plans/autos-desktop-program.md                           # M7-a 批次行 + 裁定行
current_step: 0
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
| SD-01 | add | auto-lang/docs/design/autoui/desktop-protocol-v1.md（§1.12 v1.12 增量） | before：v1.11 覆盖下 judged 六缺项在册（tabs/hidden/style-grid/定位族/012 映射），auto 缺省 independent（"flip pending ramp v3"观测行）；after：五族口径入册（tabs kind 全链 + hidden 跳过语义 + 样式 grid 分岔 + 定位族分层[absolute 真渲 + fixed/sticky 降级随注——依 D1 定案]）+ 翻转裁定结果（缺省 queue 或显式不翻 + 数据行）——PROTOCOL_VERSION 仍 1（零 wire 变体） | 协议权威版本化收录 ramp v3 与缺省裁定 | AC-01/02/03 |
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

- **T-01 [lang] 深水调查与定案**
  文件：`coverage.rs`（token 表/prefixes/仪器）、`native_projector.rs`
  （walker/臂先例）、`view.rs`（Tabs）、`ui_gen/rust.rs`（tab 映射）、
  018/021/024/041/046/012 真源 + §5.1（写面）。
  动作：D1–D6 定案（定位族口径核心）。
  产物：`### 5.1 定案记录`（file:line 证据）。
  验证：定案完备；复审通过。
  → 全 AC 前置。新路径：定案产物。
- **T-02 [lang] 012 映射 + hidden**
  文件：`coverage.rs` + `native_projector.rs`。
  动作：§5.2 T-02；D3/D5。
  验证：单测 + golden（018/041/012 样本）绿。
  → AC-01。
- **T-03 [lang] tabs kind**
  文件：`coverage.rs` + `native_projector.rs` + `ui_gen/rust.rs`
  （依 D2）。
  动作：§5.2 T-03。
  验证：tabs golden + on_select 单测 + 046 a2r 编译（若同批）。
  → AC-01/03。
- **T-04 [lang] 样式 grid**
  文件：`coverage.rs` + `native_projector.rs`。
  动作：§5.2 T-04；D4。
  验证：024 golden + 与 View::Grid 同构断言。
  → AC-01。
- **T-05 [lang] 定位族**
  文件：`coverage.rs` + `native_projector.rs`。
  动作：§5.2 T-05；D1 口径落地。
  验证：018/021 golden（含降级随注断言）。
  → AC-01。
- **T-06 [lang] 复测 + 翻转（dual-exit）**
  文件：`client_entry.rs`（:128 门后）+ `coverage.rs`（assert 反转—
  若翻）+ 数据报告。
  动作：§5.3；D6 全清单。
  验证：仪器数据行 + 断言态一致 + 台账行。
  → AC-02。
- **T-07 [lang+os] e2e 与回归**
  文件：lang `stage3.rs`（六例腿 + 翻转抽样腿）+ assets/032/；os
  smoke 如需。
  动作：AC-01..05 逐条留痕。
  → AC-03/05。
- **T-08 [lang+os] 文档与台账收口**
  文件：lang `desktop-protocol-v1.md`（§1.12）+ KNOWN-DEBT；os 台账
  M7-a 行 + 互链。
  动作：SD-01..03 落笔。
  → AC-06。

## 9. 复审记录

- 2026-09-19 /auto-plan:new 起草交接：`stage: new`，PLAN-032 rev 1
  （M7-a 首件——台账终态批次 b2e9cce 登记）。`outcome: pass`（合同
  完整：六缺项实现面逐族 file:line 在案（token 表/walker/变体/
  断裂映射）、翻转点与防漏断言、仪器复现命令、工作量分层与深水
  预判齐备）；`next: work`（前置 030 ✅；建议 031 merge 后开工）。
  悬置决策 §10（①–④），①定位族口径为核心（分层为倾向），均不阻塞
  T-01。

## 10. 待澄清事项

- **①（T-01 D1）** 定位族口径：分层（absolute+offset 真渲 +
  fixed/sticky 降级放行随注——推荐）vs 全真渲（深水——fixed/sticky
  视口锚定牵帧语义/宿主层）vs 全放行（I3 边界过宽）。
- **②（T-01 D2）** tabs a2r 断裂同批修（推荐——M7-c② 前置 + 046
  为 a2r 样本）vs 仅 VM 轨（翻转门不受阻）。
- **③（T-01 D6）** 翻转判据沿承（judged ≥95% + 缺项全在册）；新例
  入场稀释不配平（仪器如实——026/029 口径，样本集口径出界）。
- **④（债登记）** fixed/sticky 真渲 + z 完整栈序（若分层口径）——
  新债随注，真渲另立。
