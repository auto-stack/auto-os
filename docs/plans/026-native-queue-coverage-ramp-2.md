---
plan_id: PLAN-026
status: executing              # drafting → executing → execution_done → reviewed → archived
feature_name: native-queue-coverage-ramp-2
author: [agent]
created_at: 2026-09-18
updated_at: 2026-09-18
plan_revision: 1

# /auto-plan:review 结束时填写：
supersedes_spec_components: []
new_spec_components:
  - auto-lang/docs/design/autoui/desktop-protocol-v1.md   # §1.8 v1.8 增量（review 定稿）
touched_goals: []

affects:
  - auto-lang/crates/auto-lang/src/ui/view.rs                              # Icon 变体（T-01 定案后）
  - auto-lang/crates/auto-lang/src/ui/desktop_protocol/native_projector.rs  # display 族/grid/center/IME 消费臂
  - auto-lang/crates/auto-lang/src/ui/desktop_protocol/coverage.rs          # native_queue_set 扩容 + 防漏钉
  - auto-lang/crates/auto-lang/src/ui_gen/rust.rs                           # badge/card/scroll/icon codegen 断裂修复
  - auto-lang/crates/auto-lang/src/ui/iced/renderer.rs                      # View::Icon 消费臂 + IME 生产路由
  - auto-lang/crates/auto-lang/src/ui/gpui/renderer.rs                      # View 变体消费臂（cfg 门，最小）
  - auto-lang/crates/auto-lang/src/ui/desktop_protocol/client_entry.rs      # auto 缺省翻转点（T-06 门后）
  - auto-lang/crates/auto-lang/src/ui/session.rs                            # broker_ime 生产路由（025 D4 同型）
  - auto-os/docs/plans/autos-desktop-program.md                             # 3b 行更新
current_step: 0
total_steps: 8
---

# [PLAN-026] native-queue-coverage-ramp-2

## 0. 变更摘要

PLAN-025（复审中）把 native queue 臂覆盖爬到 form/payload 族
（input/textarea/checkbox/radio/slider/select + scroll 裁剪 + 键盘/滚轮/
右键输入路由两端）。本计划做**第二批覆盖爬坡 + IME 闭环 + auto 裁决翻转
评估**，四件事，**零 wire 变体**（`Ime*` 变体 v1.0 起在册，位图真渲明确
出界）：

**①display 族 native 臂**（image/avatar/progress/divider/separator/spacer
+ icon——按解释态**占位保真口径**镜像：解释态 queue 臂的 image/icon/
avatar 本就是占位 Quad，"位图内容归 Stage 5+"在册，DrawOp 无图像算子）；
**②布局族**（grid/center native walker + 语义容器经 codegen 降级验证）；
**③IME 闭环**（NativeProjector 消费 ImeCommit/Cancelled/preedit + 宿主
broker_ime 生产路由——中文输入在 queue 臂可用）；**④a2r 可编译性修复**
（badge/card/scroll/icon 映射到不存在的 View 构造器→编译失败的断裂）+
**auto 裁决翻转评估**（508 三闸 T-覆盖复测：全 examples 样本 Covered
比例，达标则 native `auto` 缺省 independent→queue，裁定入册）。

与 shell a2r 设计（`docs/design/autoui/desktop-shell-a2r.md` §10-③）的
拆分裁定落地：**icon 的 View IR 变体收进本计划**（display 覆盖的硬前置，
S1-a1 共享层），popover/slot/codegen 显式拒绝留 shell S1。

## 1. 目标

- **G1 display 族 native queue 覆盖**：image（占位 Quad 保真）/avatar
  （占位 + fallback 首字母）/progress/divider/separator/spacer 渲染臂 +
  icon 端到端（View::Icon 变体 + 四消费端臂 + codegen 臂）；覆盖表
  kinds 同步扩容。
- **G2 布局族**：grid（View::grid 已有 + 解释态 layout_grid 先例
  client_runtime.rs:831）/center native walker 臂；语义容器族（article/
  nav/ul/li…）经 codegen 降级到 container 后的覆盖语义验证（View 层
  已归一则 native 表零登记——T-01 定案口径）。
- **G3 IME 闭环（native）**：ImeCommit → 聚焦 input 编辑 buffer 追加 +
  INPUT_TEXT + on_change 派发；ImeCancelled 消解；ImePreedit 最小显示
  （尾串拼接——T-01 定案）；宿主侧 iced IME 事件 → broker_ime 生产
  路由（025 D4 键盘映射同型）。
- **G4 a2r 可编译性修复**：`tag_to_view_fn` 映射到不存在构造器的
  badge/card/scroll（rust.rs:4608/:4643-4658）与 icon（:4662）修复——
  a2r 生成物含 display 族元素可编译；003/004/010 等样本重生成编译过。
- **G5 翻转评估与收口**：T-覆盖复测数据行（examples/ui 全量 a2r 重生成
  样本 × native 覆盖判定 Covered 比例）+ 三闸评估（508 在册口径）；
  达标 → `resolve_native_frame_mode` native `auto` 缺省翻 queue + 台账
  裁定入册；不达标 → 数据留痕显式不翻。desktop-protocol-v1.md §1.8
  增量 + parity 金样（004 三臂）+ KNOWN-DEBT（图像真渲债与占位保真
  口径入册）。

**非目标**（明确出界）：

- **位图真渲 / 图像 DrawOp wire 扩展**——占位保真对齐解释态既有口径；
  真图渲染（shm 块/位图共享）呼应 shell a2r 设计 §4-B 图像通道，独立
  线另立，本计划债登记。
- popover 裸元素 / AnchorSlot 槽位 / codegen 显式拒绝策略（shell a2r
  设计 S1 范围，见该文档 §3a/§6-S1）。
- 双投影器统一（P020-D1，维持"覆盖爬坡后另立"）。
- 解释态臂扩展：解释态 queue 臂的 slider/select/IME/right/scroll 消费
  维持 not-yet（生产路径 025 已就位，消费端留痕）。
- imagesurface 交互族（渲染占位可顺带，事件/交互 not-yet 随注——T-01
  定边界）；switch View 变体（025 已注"无 View 变体不列"，维持）。
- 键盘通用路由扩展、GUI 级 OS 自动化（P020-D4）、GPU 纹理共享、GPUI
  臂功能扩展（仅补 View 变体最小消费臂保编译）。

## 2. 架构方案

三臂改造 + 翻转点，零 wire 变体：

```text
┌─ View IR 臂（ui/view.rs + 四消费端）──────────────────────────────┐
│ View::Icon 变体（T-01 定案：变体 vs 降级——icon 无既有承载形态，  │
│   倾向变体）；消费端 = iced renderer / gpui renderer(cfg) /       │
│   vnode_converter / native_projector 四臂全补（变体穷尽性）       │
│ 既有构造器直用：grid/center/image_styled/progress_bar/spacer/     │
│   divider/avatar（view.rs:1114/:1801/:1574/:1744/:1773/:1778/:1783）│
└──────────────────────────────────────────────────────────────────┘
┌─ 投影器臂（desktop_protocol/native_projector.rs）────────────────┐
│ display 族渲染臂：镜像解释态占位保真（layout_image :1339 占位     │
│   Quad / layout_icon :1363 字形占位方块 / layout_avatar :1415 占位 │
│   +首字母 / layout_badge :1383 / layout_progress :1452 /          │
│   layout_divider :1474 / spacer :1497）                           │
│ grid/center walker 臂（镜像 layout_grid :831 两遍网格）            │
│ IME 消费：ImeCommit/Cancelled/Preedit → 025 D1/D2 机制复用        │
│   （聚焦槽位 + INPUT_TEXT 代写 + on_change 派发）                 │
└──────────────────────────────────────────────────────────────────┘
┌─ 生成器/宿主臂 ───────────────────────────────────────────────────┐
│ ui_gen/rust.rs：badge/card/scroll/icon 映射断裂修复（降级或构造   │
│   臂——T-01 定案）；语义容器降级验证                               │
│ session.rs：broker_ime 生产路由（iced IME 事件映射，025 D4 同型）  │
│ client_entry.rs：native auto 缺省翻转点（T-06 三闸门后）          │
└──────────────────────────────────────────────────────────────────┘
```

**不变式**：

- **I1 零删除**：025 交付面（form/payload/scroll/输入路由）与 020 交付面
  全量零回归；既有测试全绿是回归门。
- **I2 追加式协议**：`PROTOCOL_VERSION` 维持 1，零新 wire tag——IME 用
  v1.0 在册的 `Ime*` 变体，display 族用既有 Quad/Text/TextStyled。
- **I3 not-yet 纪律**：占位保美（image/icon/avatar 位图内容）与未覆盖
  面（imagesurface 交互/解释态扩展）显式随注，禁静默；显式 `queue` 遇
  not-yet 拒绝行为延续。
- **I4 保真对齐**：native 臂的 display 族保真口径 = 解释态 queue 臂
  同级（占位），不引入单臂超集——三臂金样对拍是验收面。

**关键风险**：View 加 Icon 变体牵动四消费端（含 cfg 门 gpui 臂）——
编译面扩散但机械；iced 0.14 IME 事件面（`keyboard::Event` 家族对 IME
的暴露形态）需 T-01 核实（editor_frame 消费先例在，生产侧无先例）；
翻转是**行为裁定翻转**（缺省臂变化）——数据门 + 台账双锚，禁拍脑袋。

## 3. 技术栈

Rust / iced 0.14；既有 desktop_protocol（零 wire 变体）；ui_gen codegen；
验收载体 = `examples/ui/004-profile-card`（image + 渐变 col，Stage 4 解释
态爬坡同款样本）+ grid 样本（010-contact-form / 013-todo 按实况取）+
icon 样本（examples 含 icon 元素者或 scratch，025 scratch 先例）；IME =
协议级 `ImeCommit` 注入承载（P020-D4 GUI 自动化债未清前口径）；
e2e = auto-os `scripts/desktop.sh` iced 宿主 + smoke 026 腿（025 脚本
扩）。

## 4. 需求分析与背景调查

**授权记录**：用户 2026-09-18 会话明确"计划 25 已实施完毕正在 review，
可以着手下一步计划的书写"（按既定路线：覆盖爬坡第二批 + 翻转评估；
shell 设计 §10-③ 拆分裁定随本计划落地）——**本轮仅规划，未授权实施**。
涉及仓：auto-lang（View IR/投影器/codegen/宿主路由/协议文档）+ auto-os
（台账/e2e/smoke）。无预算/自动续跑约束声明。

**前置依赖**：PLAN-025 merge 收口（026 的 coverage/native_projector/
client_entry 基线 = 025 landed master；025 复审中，分支 plan-025-dev
@870ee1574，T-01..T-08 全勾）。**026 worktree 于 025 merge 后开**。

**现状事实**（已核，2026-09-18，lang-025 worktree = 025 落地态）：

- **025 落地态**：native_queue_set kinds = text/button/input/textarea/
  checkbox/radio/slider/select；layouts = col/row/container/list/empty/
  anchorslot/scroll；样式前缀含 flex-1/shadow 降级放行（003 真源 gate
  所需，非静默扩权——coverage.rs 注记在案）。D1 聚焦 = Input 槽位序 +
  帧后重定位；D2 = 投影器同线程代写 INPUT_TEXT（buffer 聚焦时自 view
  value 初始化）；D4 = iced 键盘 `text` → CharTyped、Named 键 → VK u32
  （BACK=8/ESC=27）、滚轮 Lines×LINE_H 像素化，WM 焦点 = session.focused
  路由；select 开合 = 覆盖序 ops + 投影器侧槽位 + Esc 关闭。
- **DrawOp 无图像算子**（message.rs:76-103：Quad/Text/TextStyled/Scissor/
  ScissorPop）；**解释态 queue 臂 display 族 = 占位保真**：
  `layout_image`"样式尺寸驱动的占位 Quad（结构/占位正确）"（
  client_runtime.rs:1339-1362）、`layout_icon`"字形占位方块"（:1363-1382，
  IMAGE_PLACEHOLDER :44）、`layout_avatar`"方块占位 + fallback 首字母
  （src 位图内容归 Stage 5+）"（:1415）、badge（:1383）/progress
  （:1452）/divider（:1474）/spacer（:1497）。
- **View IR 构造器实况**：grid()/image/image_styled/progress_bar(_styled)/
  spacer()/divider()/avatar()/center()/scrollable() 已有（view.rs:1114/
  1470/1574/1744/1773/1778/1783/1801/1814）；**icon()/link()/badge()/
  card() 不存在**。
- **codegen 断裂**：`tag_to_view_fn` 把 badge/card/scroll/tooltip/modal/
  spinner 映射到不存在构造器（rust.rs:4608/:4643-4658）、icon 同（:4662）
  → a2r 生成物编译失败；裸 popover 走 col 降级 + prop 静默丢弃（:4713，
  shell S1 范围）。grid/center/image/progress/spacer/divider/avatar/
  scrollable 构造路径可用（rust.rs:2418/:3229/:2916/:3019-3041/:3009-
  3016）。
- **解释态 target_set 差集**（coverage.rs:58-148 vs native 025 落地态）：
  kinds 缺 `image, a, img, icon, badge, avatar, progress, divider,
  separator, spacer`；layouts 缺 `center, grid` + 语义容器 24 项 + card
  族（语义容器在 codegen 降级到 container 后 View 层归一——native 表
  是否登记依 T-01 定案）。
- **IME 面**：wire `ImePreedit{text,cursor}/ImeCommit{text}/ImeCancelled`
  v1.0 在册（message.rs:645-648）；消费先例 = editor_frame.rs:170-205
  全变体转 EditorInput；解释态 queue 臂 CharTyped-only（IME not-yet）；
  025 D4 生产路由不含 IME（键盘/滚轮 only）——本计划补。
- **翻转点与判据**：`resolve_native_frame_mode`（client_entry.rs，025
  T-04 落地"native auto 缺省 independent + 降级观测行"）；508 三闸 =
  T-覆盖（507 合入后复测——本计划数据行）/T-稳定性/T-远程；025 T-08
  已落覆盖翻转数据行（examples 样本比例，报告在 lang 仓 025 报告内）。
- **View 变体消费端**（Icon 变体牵动面）：iced renderer.rs、gpui
  renderer.rs（cfg 门）、vnode_converter.rs、native_projector.rs——
  变体穷尽匹配，加变体须四端全补臂。
- **specs 现状**：协议权威 = desktop-protocol-v1.md（v1.7 现行 = 025）；
  shell a2r 设计文档（desktop-shell-a2r.md，2026-09-18 落盘）§10-③
  拆分裁定由本计划执行（icon 收编、popover/slot/拒绝留 S1）。

## 5. 详细设计

### 5.1 T-01 深水调查定案（决策产物）

- **D1 icon/a/badge/card 的 View 形态**：icon 无既有承载（View 无变体、
  解释态直挂绕过 View）——候选 A = 新增 `View::Icon{name,size,style}`
  变体（四消费端臂，S1-a1 共享；**倾向**）；候选 B = codegen 降级为
  styled 容器/占位（零 View 变体但语义糊）。a = text 承载 + 下划线
  样式（解释态口径"a{text/style}，下划线不载"）→ 倾向 codegen 降级
  text_styled；badge/card = 倾向 codegen 降级（badge → text + chip 样式
  bg/rounded；card → container styled），View IR 收敛不加变体。定案
  含 004/010 真源样本验证。
- **D2 IME 消费最小集**：Commit/Cancelled 必须；preedit 显示候选 A =
  聚焦框值尾串拼接 + 下划线色（Text op 直拼，**倾向**）vs B =
  Commit-only（preedit 丢弃随注）。宿主映射面：iced 0.14 IME 事件
  （`keyboard::Event` 家族 / window 事件对 IME 的暴露形态）→
  `InputMsg::Ime*` 的映射可行性核实（editor_frame 消费先例 :170-205，
  生产侧零先例——唯一调查未知量）。
- **D3 翻转判据口径**：样本集 = examples/ui 全量 a2r 重生成 ×
  `scan_native_view × judge(native_queue_set_扩容后)`；阈值定案（建议
  Tier1+2 样本 100% Covered 或 ≥95% + 缺项清单全在册 not-yet）；稳定
  性闸 = 025/026 套件基线全绿；远程闸 = 508 远程面零牵连确认。翻转
  语义 = `resolve_native_frame_mode` 缺省 independent→queue（降级观测
  行语义保留——探测不 Covered 仍降级留痕）。
- **D4 语义容器覆盖语义**：codegen 降级（article/nav/ul/li → container）
  后 View 层归一 container——native 表零登记即可覆盖（scan 见
  Container）；验证降级链完备（无语义容器走断裂映射）。
- **D5 imagesurface 边界**：渲染 = 占位 Quad 顺带（image 同口径）；
  事件/交互（on_move/on_zoom 族，rust.rs:2942-3006 发射面）= not-yet
  随注（命中表不登记）——或整 kind not-yet（T-01 按 scan 实况定，
  倾向渲染顺带 + 交互 not-yet）。

定案记录追加 `### 5.1 定案记录`，作为 T-02..T-06 依据。

### 5.1 定案记录

（2026-09-18 /auto-plan:work T-01；证据基线 = lang-026 worktree @ master
e352437b0，025 landed b9e9f6899。file:line 均为该基线。）

**D1 icon/a/badge/card View 形态——定案：全部 codegen 降级（候选 B），
零 View 变体**。推翻计划倾向（icon 变体）的关键证据：**icon 的 View IR
承载在 VM 轨既有且成契约**——AuraViewBuilder `convert_image_or_icon`
（aura_view_builder.rs:6455-6500）把 icon 降级为
`View::Image{src: "lucide:{name}"}`（PLAN-018 协议前缀 iconfile:/hicon:/
lucide: 透传 + `with_icon_size` 尺寸契约：显式 w-/h- 类 > size prop >
共享默认，注释明言"**两端必须一致**，与 ui_gen/vue.rs 的 icon 臂同一套"）。
新增 View::Icon 变体 = 在 View IR 制造第二个 icon 承载、破坏既有两端
一致契约，违背计划自身"View IR 收敛"原则（badge/card 降级的同一理由）。
a2r codegen icon 臂按同型降级。余者：

- a/link → text_styled 降级（下划线不载 = 解释态口径）；"link" => "link"
  断裂同修（同 tag 族）。
- badge → Row + shadcn 基类合并（convert_badge aura_view_builder.rs:9157
  同型：base "inline-flex items-center…" + variant preset + text prop
  子级兜底）。
- card → container styled 降级；语义 card 族（cardheader/cardtitle/
  cardcontent/cardfooter/cardaction/carddescription）同 container 降级。
- **D1'（display 族 View 承载形态——计划 §5.3 渲染臂落地口径修正）**：
  spacer/divider/avatar 的 View 构造器为占位 stub（spacer()→View::Empty
  view.rs:1773、divider()→View::Empty :1778、avatar()→col builder
  :1783），直用则 native 静默零渲/丢 prop。定案：**a2r codegen 对齐
  VM 轨既有降级形态**（VM 轨证据：convert_spacer :6813 container+flex-1/
  user style；convert_divider :6885 container "w-full h-1 bg-gray-200"；
  convert_sep :8508 orientation 双档 w-full h-px bg-border / w-px h-4
  中线 Container(center)；convert_avatar :8559 尺寸类缺省 w-10 h-10
  bg-gray-300 rounded-full + 子件组合；convert_progress :6750
  View::ProgressBar；convert_grid :6215 View::Grid）。View 构造器 stub
  本体不动（VM/iced 消费面零扰动）。image → View::image/image_styled
  （codegen :2936-2953 既有）。
- modal/tooltip/spinner/tab/option/toggle/radiogroup 映射断裂维持
  （tag_to_view_fn :4779-4789 映射目标构造器不存在——grep 核实），
  显式拒绝策略归 shell a2r 设计 S1，KNOWN-DEBT 随注；026 只修
  badge/card/scroll/icon/a（AC-05 口径）。

**D2 IME 消费最小集——定案：Commit/Cancelled 必须 + preedit 尾拼
（候选 A）**。机制复用面 = NativeProjector{focused_input, input_buffer,
dispatch_input_edit}（native_projector.rs:127-135/:421-455）：ImeCommit =
聚焦 buffer 追加 → INPUT_TEXT 代写 → on_change 派发 → rev++；
ImePreedit = 暂存字段 + input 臂渲染尾拼（buffer 后接 preedit 串，
独立 Text op 下划线色区分）；ImeCancelled = 暂存清；无聚焦 = 丢弃 +
uncovered 观测留痕。消费先例 editor_frame.rs:195-199（三变体全转）。
宿主生产 = session.rs broker_ime（broker_char :3233 同型：session.focused
→ ProtocolMsg::Input(InputMsg::Ime*)）+ stage3 单测（025 T-05 broker_
input_production_routes 同型）。**悬置⑤落定：协议级 ImeCommit 注入为
证据承载**（025 已定口径的延续——live iced 壳无键盘/滚轮订阅通道 =
P025-D1 在册债，IME 订阅缺口并入该债随注；iced 0.14 = "0.14.0"
crates/auto-lang/Cargo.toml:199，真机事件面调查随债清进行）。

**D3 翻转判据——定案**：样本集 = examples/ui 全量 app（.at 解析 →
AuraViewBuilder 构建 View<DynamicMessage> 树（VM 轨运行时构造器，
aura_view_builder.rs 模块头文档）→ scan_native_view × judge
(native_queue_set 扩容后)）。仪器选型注记：AuraViewBuilder 为 VM 轨
aura→View 既有机制，与 a2r codegen 同以"降级到 View IR"为口径——
T-02 把 codegen display 臂对齐 VM 形态后，两轨 View 层同构，scan 结果
即 a2r 语义投影（编译级验证另由 004/scratch exe e2e 承担）。阈值 =
**≥95% Covered 且缺项清单全在册 not-yet**（payload 族 table/tabs/
accordion/sidebar/… + imagesurface + 解释态扩展面）→ 翻；否则不翻
留痕（AC-06 双出口）。翻转语义 = resolve_native_frame_mode
（client_entry.rs:117-131）Auto 臂改 queue 优先——**覆盖判定前移进
裁决**：Auto → 扫描判定，Covered → (Commands, false, None)；
NotCovered → (Pixels, true, 观测行载荷=缺项清单)——降级观测行语义
保留（探测不 Covered 仍降级留痕）。coverage 表定案：native kinds +
**image, progress**；layouts + **grid**；"center" **不入册**（View 层
归一 Container——scan_native_view 无 "center" kind 产出点
（native_kind_of coverage.rs:431-479 无此映射），登记即违反防漏钉钉②
"表内 kind 无夹具/无臂即炸"）；其余 display 族（icon/badge/avatar/
divider/separator/spacer/a/img）**经降级归一不入册**（产出 kind =
image/text/row/container/empty，全在册）——025"switch 无 View 变体
不列——分表非缺口"同口径（I4 分表非缺口）。

**D4 语义容器——定案：零登记（计划口径维持）**：article/nav/ul/li/
section/header/footer/aside/main/figure/details/summary/dl/dt/dd/ol
在 AuraViewBuilder 走语义容器臂（块流纵排）→ View::Column/Container
（507 T6 既有）；a2r 侧 `_ => "col"` fallback（rust.rs:4814）同归。
验证 = 抽样 codegen 冒烟（article/nav/ul 三件）+ 翻转数据行全量覆盖。

**D5 imagesurface——定案：整 kind not-yet（kinds 不登记）**。渲染
顺带 = native catch-all 占位盒既有（native_projector.rs:931-941
"覆盖门后动态分支防线"臂——ImageSurface 落此，占位盒 + uncovered_seen
留痕，"渲染占位顺带"语义已达成）；登记 kind 会令 scan_native_view
（无事件采集面，coverage.rs:486-541 注记）静默放行 on_wheel/on_pan
fn 回调族——违背 I3；显式 queue 遇 imagesurface = ensure_covered 拒绝
留痕维持。与计划推荐（渲染顺带+交互 not-yet）实质等价、实现面最小。

**任务级影响（证据驱动偏差，均留痕）**：T-02 由"View::Icon 变体 +
四消费端臂"改为"**a2r codegen display 族降级臂**（对齐 VM 轨形态：
icon/image/badge/card/scroll/a/divider/spacer/avatar/separator + 语义
容器验证）"——零变体 ⇒ 零四端穷尽性牵动，`cargo check --features
ui-iced,ui-gpui` 门取消（无编译面变化）；icon 端到端验收口径不变
（a2r 样本含 icon 可编译 + queue 渲染占位——AC-02）。T-03 渲染臂
= Image/Progress（Grid 归 T-04）；Badge/Divider/Spacer/Avatar 臂取消
（codegen 降级后 View 层无此 kind）。total_steps 相应 8→8（T-02 内容
替换，任务数不变）。

### 5.2 View IR icon 臂（T-02）

`View::Icon{name: String, size: f32, style: Option<Style>}`（D1-A 定案
形态）+ 四消费端臂：iced renderer（占位方块保真——真字形 lucide 若
iced 侧有现成基建顺带，否则 not-yet 随注）、gpui renderer（cfg 门最小
臂）、vnode_converter（归一 kind "icon"）、native_projector（T-03 display
臂合流）；codegen icon 臂（`View::icon(name).size(s)` builder）。

### 5.3 display 族投影臂 + codegen 修复（T-03）

- 渲染臂（native_projector.rs `layout_view_node`）：Image（占位 Quad，
  bg = style.bg 或 IMAGE_PLACEHOLDER——常量 pub(crate) 化复用）/Avatar
  （占位 + 首字母 Text）/Progress（track + fill Quad 比例几何）/
  Divider（1px Quad）/Spacer（尺寸占位）/Badge（D1 定案形态：降级
  text+chip 或独立臂）/Icon（T-02 变体臂，占位方块）。
- codegen 修复（ui_gen/rust.rs）：badge/card/scroll 映射断裂 → D1/D4
  定案的降级目标；`a` → text_styled 降级；icon 臂（T-02）。修复纪律：
  降级映射不静默丢关键 prop（label/src/progress 值必达）。

### 5.4 布局臂（T-04）

grid：walker 两遍网格（镜像 layout_grid :831——cols/spacing/gap 语义，
交叉轴尺寸聚合）；center：主轴/交叉轴双居中（两遍法既有机制复用，
view.rs:1801 center 构造 + rust.rs:3229 codegen 臂既有）。native 表
layouts + `center, grid`；语义容器 D4 验证。

### 5.5 IME 闭环（T-05）

投影器：`on_input` 扩 `ImeCommit{text}`（聚焦 buffer 追加 → INPUT_TEXT
代写 → `component.on(on_change)` → rev++）、`ImeCancelled`（preedit
消解）、`ImePreedit`（D2-A：preedit 串暂存 + input 臂渲染尾拼）；无
聚焦 input = 丢弃留痕。宿主：session.rs `broker_ime`（025 D4 同型
映射：iced IME 事件 → InputMsg::Ime* → session.focused 窗路由）。

### 5.6 覆盖表收口 + 翻转（T-06）

native_queue_set kinds + `image, img, icon, a, badge, avatar, progress,
divider, separator, spacer`（+ D5 imagesurface 口径）；layouts +
`center, grid`；防漏钉更新（覆盖矩阵 × 投影器臂双向钉，新拒样本 =
imagesurface 交互/popover）；T-覆盖复测数据行（D3 口径）落报告；三闸
评估 → 达标翻转 `resolve_native_frame_mode` 缺省 + 台账裁定行；不达标
显式留痕（AC-06 双出口）。

### 5.7 e2e 与收口（T-07/T-08）

- e2e：`p026_native_display_arm`（AUTO_DESKTOP_E2E 门，025 同型）——
  004-profile-card a2r exe 显式 queue 孵化 → image 占位/渐变 col/button
  渲染断言；grid 样本同链；IME = 协议级 `ImeCommit` 注入 → 中文入值
  帧断言（真机 IME 生产路径证据尽力，P020-D4 口径协议级承载）；截图
  留痕 `docs/plans/reports/assets/026/`（lang 仓）；smoke 026 腿
  （os 仓，025 脚本扩展）。
- 文档：desktop-protocol-v1.md **§1.8 v1.8 增量**（display 族占位保真
  入册 + IME 闭环两端 + auto 缺省裁定 + not-yet 边界：位图真渲/
  imagesurface 交互/解释态扩展）；os 程序台账 3b 行更新（覆盖二批 +
  翻转裁定）；KNOWN-DEBT：图像真渲债登记（占位保真口径 + 呼应
  shell-a2r 设计 §4-B 图像通道独立线）+ 新债随注；parity 金样 004
  三臂（独立窗/queue 重放/pixels 重放）。

### 规范增量

| delta_id | add/modify/retire | docs/specs/... target | before/after rule | rationale | acceptance IDs |
|---|---|---|---|---|---|
| SD-01 | add | auto-lang/docs/design/autoui/desktop-protocol-v1.md（§1.8 v1.8 增量） | before：v1.7 native 覆盖 = form/payload + 线性/scroll，IME not-yet，auto 缺省 independent；after：display 族入册（占位保真口径显式）+ grid/center + IME 闭环两端（Ime* 既有变体消费 + broker_ime 生产）+ auto 缺省裁定（翻转或维持，数据门双出口）——PROTOCOL_VERSION 仍 1（零 wire 变体） | 协议权威文档版本化收录二批爬坡 | AC-02/03/04/06 |
| SD-02 | modify | auto-os/docs/plans/autos-desktop-program.md | before：3b 行 = 025 覆盖集 + auto 缺省 independent；after：3b 行更新（display/布局二批 + IME + 翻转裁定结果） | 桌面程序台账 | AC-06 |
| SD-03 | modify | auto-lang/docs/specs/auto-lang/ui/（review 期按目录实况定） | before：025 provisional 对齐 v1.7；after：coverage/native_projector/View icon 条目对齐 v1.8 | 模块 spec 对齐实现 | AC-02/03/05 |

零 spec 影响的变更不存在（覆盖集/IME/缺省裁定为协议级知识）；ledger
（auto-lang `.autoos/specs.json`）随 merge 沉淀。

## 6. 测试设计

- **单测（native_projector/coverage）**：display 臂 golden（image 占位
  尺寸/bg、avatar 首字母、progress 比例几何、divider/spacer、icon 占位
  方块、badge 形态）；grid/center 布局 golden + 命中；IME（Commit 入值
  帧变、Cancelled 消解、preedit 尾拼、无聚焦丢弃留痕）；覆盖判定
  （display/grid 入、imagesurface 交互/popover 拒）；防漏钉双向更新。
- **单测（View IR/消费端）**：Icon 变体四消费端臂单测（iced/gpui/vnode/
  native 各一）；codegen golden（icon/badge/card/scroll/a 修复映射——
  004/010 fixture .at 重生成 diff 钉）。
- **单测（session）**：broker_ime → 焦点窗管道路由（025 键盘路由测试
  同型）。
- **集成（真管道全循环）**：025 `native_client_full_cycle_over_pipe`
  同型扩展——display App（004 级）+ grid 样本 + IME 注入闭环。
- **e2e（AUTO_DESKTOP_E2E）**：`p026_native_display_arm` + IME 注入腿；
  smoke 026 腿（os 仓）；截图 assets/026/。
- **parity**：004-profile-card 三臂金样对拍。
- **翻转数据**：examples/ui 全量 a2r 重生成 × 覆盖判定比例表（D3 口径）
  落报告。
- **回归门**：desktop_protocol / session / stage3 / dual_mode /
  app_registry / `cargo t -p auto-man rust_ui` / auto-os 桌面 smoke。

## 7. 验收标准

- **AC-01 既有零回归**：025/020 交付面全绿（native form/payload/scroll/
  输入路由/pixels/独立窗/解释态三形态）；`cargo t -p auto-lang
  --features ui-iced desktop_protocol` + session/stage3 全绿（在册既有
  红 coverage::covered_elements_within_target_set 除外）。
- **AC-02 display 族 queue 覆盖**：004-profile-card a2r exe 显式 queue
  档孵化 Covered（不再拒绝），image 占位/渐变/button 渲染进虚拟窗；
  icon 端到端（a2r 样本含 icon 可编译 + queue 渲染占位）。验证：golden
  + e2e + 截图。
- **AC-03 grid/center**：grid 样本 queue 渲染 + 网格命中派发正确；
  center 布局 golden。验证：单测 + 集成。
- **AC-04 IME 闭环**：协议级 `ImeCommit{"中文"}` 注入 → 聚焦 input 值
  更新 + on_change 派发 + 帧变；Cancelled/preedit 行为符合 D2 定案；
  broker_ime 路由单测绿；真机 IME 证据尽力（不可达则协议级承载留痕）。
- **AC-05 a2r 可编译性**：含 badge/card/scroll/icon/a 的样本（004/010
  + fixture）a2r 重生成编译过；关键 prop（label/src/progress）不静默
  丢失。验证：codegen golden + scratch 编译冒烟。
- **AC-06 翻转裁定双出口**：T-覆盖复测数据行 + 三闸评估落报告——
  达标：`resolve_native_frame_mode` native auto 缺省 = queue（降级观测
  行保留）+ 台账裁定行入册 + 全 examples 样本 e2e 抽样验证；不达标：
  显式"不翻 + 缺项清单"留痕。两出口均为 pass 态，禁无数据翻转。
- **AC-07 文档与回归门**：§1.8 增量、台账 3b 行、KNOWN-DEBT 图像真渲
  债、parity 金样落盘互链；§6 回归门全绿。

## 8. 执行步骤

**前置**：PLAN-025 merge 收口（026 worktree 基于 025 landed master 开）。
依赖序：T-01 → T-02 → {T-03, T-04 并行} → T-05 → T-06 → T-07 → T-08。
lang 侧 worktree `D:/autostack/.wt/lang-026/auto-lang`；os 侧
`D:/autostack/.wt/os-026/auto-os`。

- **T-01 [lang] 深水调查与定案** ✅
  文件：`ui/view.rs`、`ui/desktop_protocol/{native_projector,client_
  runtime,coverage,client_entry}.rs`、`ui_gen/rust.rs`、`ui/session.rs`
  iced 事件面（读）+ 本计划 §5.1（写）。
  动作：D1–D5 定案（icon/a/badge/card 形态、IME 最小集与 iced 映射、
  翻转判据、语义容器口径、imagesurface 边界）。
  产物：`### 5.1 定案记录`（file:line 证据）。
  验证：定案完备；复审通过。
  → AC-02/04/06 前置。新路径：是（调查产物）。
  [2026-09-18 work] §5.1 定案记录落盘（D1 全降级定案——关键证据 =
  VM 轨 icon→View::Image lucide 承载契约 aura_view_builder.rs:6455
  "两端必须一致"；D1' 降级形态对齐 VM 轨；D2 preedit 尾拼 + 协议级
  注入口径；D3 ≥95% 阈值 + AuraViewBuilder 仪器 + kinds+image/progress/
  layouts+grid 且 center 不入册钉②口径；D4 零登记维持；D5 整 kind
  not-yet）。任务级偏差：T-02 改 codegen 降级臂、零 View 变体。
  → AC-02/04/06 前置就绪。
- **T-02 [lang] View::Icon 变体 + 四消费端臂 + codegen 臂**
  文件：`ui/view.rs`（变体 + builder）、`ui/iced/renderer.rs`、
  `ui/gpui/renderer.rs`（cfg 最小臂）、`ui/vnode_converter.rs`、
  `ui_gen/rust.rs`（icon 臂）。
  动作：按 §5.2 + D1 定案。
  验证：四消费端单测 + codegen golden 绿；`cargo check --features
  ui-iced,ui-gpui` 过。
  → AC-02/05。
- **T-03 [lang] display 族投影臂 + codegen 断裂修复**
  文件：`native_projector.rs`（Image/Avatar/Progress/Divider/Spacer/
  Badge/Icon 臂）、`ui_gen/rust.rs`（badge/card/scroll/a 修复）。
  动作：按 §5.3 + D1/D5 定案。
  验证：display golden + codegen golden（004/010 fixture）+ scratch
  编译冒烟。
  → AC-02/05。
- **T-04 [lang] grid/center 布局臂**
  文件：`native_projector.rs`（walker grid/center）。
  动作：按 §5.4 + D4 验证。
  验证：grid/center golden + 命中单测 + 语义容器降级链测试。
  → AC-03。
- **T-05 [lang] IME 闭环两端**
  文件：`native_projector.rs`（ImeCommit/Cancelled/Preedit 消费）、
  `session.rs`（broker_ime 生产路由）。
  动作：按 §5.5 + D2 定案。
  验证：IME 单测 + broker_ime 路由单测 + 集成注入闭环。
  → AC-04。
- **T-06 [lang] 覆盖表收口 + 翻转评估**
  文件：`coverage.rs`（扩容 + 防漏钉）、`client_entry.rs`（翻转点，
  门后）、翻转数据报告（lang `docs/plans/reports/`）。
  动作：按 §5.6 + D3 口径；三闸评估。
  验证：防漏钉绿；数据行 + 评估结论落盘；达标腿含抽样 e2e。
  → AC-06。
- **T-07 [lang+os] e2e 验收**
  文件：lang `stage3.rs`（p026_native_display_arm + IME 腿）+ 截图
  `docs/plans/reports/assets/026/`；os `scripts/`（smoke 026 腿）。
  动作：AC-02..05 逐条跑通留痕。
  → AC-02/03/04。
- **T-08 [lang+os] 文档与台账收口**
  文件：lang `desktop-protocol-v1.md`（§1.8）、`KNOWN-DEBT-AND-RISKS.md`
  （图像真渲债 + 新债）；os `autos-desktop-program.md`（3b 行更新）+
  两仓互链。
  动作：SD-01..03 落笔。
  → AC-07。

## 9. 复审记录

- 2026-09-18 /auto-plan:new 起草交接：`stage: new`，PLAN-026 rev 1。
  `outcome: pass`（合同完整：025 落地态/解释态占位保真口径/codegen 断裂
  清单/IME wire 与消费先例全部 file:line 在案；任务覆盖全部 AC 与规范
  增量）；`next: work`——**前置 = PLAN-025 merge 收口**（解封动作明确：
  025 merge 后开 lang-026/os-026 worktree，T-01 起步无需用户解锁）。
  悬置决策登记 §10（①–⑤），均不阻塞 T-01 开工。

## 10. 待澄清事项

- **①（T-01 D1）** icon/a/badge/card 的 View 形态：倾向 icon 加变体
  （S1-a1 共享）、a/badge/card 走 codegen 降级（View IR 收敛）——以
  004/010 真源样本定案。
- **②（T-01 D2）** IME preedit 显示范围：尾串拼接（推荐）vs
  Commit-only 最小；iced 0.14 IME 事件映射面为唯一生产侧未知量。
- **③（T-06）** 翻转执行口径：达标即翻（推荐——025 已埋路线、数据门
  双出口防拍脑袋）vs 数据留痕下期翻。
- **④（T-01 D5）** imagesurface：渲染占位顺带 + 交互 not-yet（推荐）
  vs 整 kind not-yet。
- **⑤（T-07）** 真机 IME e2e 口径：宿主生产路径真机证据（iced IME 事件
  可注入时）vs 协议级 ImeCommit 注入承载（P020-D4 既有口径）——按
  D2 调查结果定，两口径均留痕。
