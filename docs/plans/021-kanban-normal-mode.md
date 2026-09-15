---
plan_id: PLAN-021
status: reviewed             # drafting → executing → execution_done → reviewed → archived
feature_name: auto-kanban 普通模式——手动看板（卡片 CRUD/列移动/排序/持久化），与只读计划板并存为双模式
author: [zhaopuming, ZCode]
created_at: 2026-09-15
updated_at: 2026-09-15
plan_revision: 2

# /auto-plan:review 结束时填写：
supersedes_spec_components: []
new_spec_components: [P021-1 apps/kanban.md（app spec 新增：双模式架构/泛化 Card 契约/manual 无状态文件后端契约）]
touched_goals: []

affects: [docs/specs/apps/kanban.md]
current_step: 10
total_steps: 10
---

# [PLAN-021] auto-kanban 普通模式（手动看板）——双模式并存

## 0. 变更摘要

auto-kanban 目前只有一种形态：对项目计划的只读展示（lang_plans 板，Plan 579
交付）。用户裁定（2026-09-15）添加**普通模式**：与常见看板 app（Trello/
Notion board 一类）同型的手动看板——可手动添加卡片、编辑卡片细节（标题/
详情/优先级）、在列间移动、列内排序、删除，且数据持久化。

本计划交付（全部变更落在 auto-kanban 仓，计划文件在 auto-os 主导仓）：

1. **双模式架构**：模式 = 看板（kind）。`boards.json` 注册第二个看板条目
   `manual`（普通看板）；header 板切换按钮组（配置驱动，已有）即模式切换；
   页面按当前板 kind 条件渲染——`lang_plans` → 现有只读四列板（零功能改动），
   `manual` → 新手动三列板。
2. **泛化 Card 扩展**：`Card` 增加 `detail` / `priority` / `order` 三字段；
   lang_plans 适配器填默认值（""/""/0），维持"所有 source 填全字段"的泛化契约。
3. **manual 数据源**：新适配器 `src/back/manual_store.at`——无状态文件后端
   （每请求读 `data/manual.json`，每次变更全量重写），JSON 手工序列化 +
   动态 JsonValue 导航读取（VM 约束下的既有先例写法）。
4. **写 API 四端点**：POST 创建 / PUT 编辑 / PUT move（列移动+排序合一）/
   DELETE 删除，经 boards_registry 按 kind 分发（沿用读路径的 seam）。
5. **前端手动板页**：三列（待办/进行中/完成）+ 快速添加行 + 卡片编辑面板
   （标题 input / 优先级单选 / 详情 textarea）+ 按钮移动与 ▲▼ 排序（双端
   可用）+ Vue 拖拽增强 + 删除两击确认。
6. **测试**：fixture 注入 + playwright 新用例（~9 条）+ 现有 8 条只读板
   回归全绿 + VM 轨一致性一轮。

**明确不做**（v1 边界，详见 §1 非目标）：多手动看板管理、自定义列、标签、
截止日期、搜索筛选、手动板归档、多人协作、卡片详情路由页。

## 1. 目标

1. 用户可在 header 一键切换「计划（只读）」与「普通看板（可编辑）」两种
   模式；只读板行为与现状完全一致（现有 8 测零回归）。
2. 普通模式下可完成完整卡片生命周期：添加（快速添加 + 列内添加）→ 编辑
   细节（标题/多行详情/优先级）→ 移动列（按钮 + Vue 拖拽）→ 列内排序
   （▲▼ + Vue 拖拽）→ 删除（两击确认）。
3. 数据持久化：所有变更落 `data/manual.json`，重启 `auto run` 后卡片与
   顺序原样恢复；数据文件路径可被 `AUTO_KANBAN_DATA` 环境变量覆盖（测试
   注入用，沿 `AUTO_KANBAN_BOARDS` 先例）。
4. 配置驱动不回退：manual 板由 boards.json 条目驱动——去掉条目即从 UI
   消失；lang_plans 板同理（Plan 579 目标 3 的延续）。
5. 双端可用：Vue 轨（`auto run`）全功能；VM 轨（`auto run -r vm`）按钮
   通道全功能（拖拽为 Vue 增强，见 §10-1）。
6. 成功样貌：playwright 全套（旧 8 + 新 ~9）绿；VM 轨 MCP 快照见三列与
   卡片计数链；真实数据只读板对账不破坏（C3 沿用）。

### 非目标（v1 边界外，出现真实需求再立项）

- 多手动看板（建板/改名/删板）——boards.json 加条目即可扩展第二块手动板
  （每板一数据文件），但 UI 不提供管理界面。
- 自定义列（增删改列名/列排序）——列集硬编码 todo/doing/done。
- 标签、截止日期、卡片成员。
- 搜索/筛选。
- 手动板归档（archived 折叠区是只读板语义）。
- 多人协作/并发合并（单用户本地 app；全量重写即够）。
- 卡片详情独立路由页（023-realworld 有 `:slug` 路由先例，但 v1 用页内
  条件面板——015-notes 先例——更省路由接线）。

## 2. 架构方案

### 双模式拓扑（= 板 = kind）

```
boards.json                                  前端
{ "boards": [                                header 按钮组（配置驱动，已有）
  { id: "lang-plans", kind: "lang_plans",…},    [计划] [普通看板] [↻ 刷新]
  { id: "manual",     kind: "manual", … }    ↓ SelectBoard → store.current_kind
] }                                          board 页按 kind 条件渲染：
                                              lang_plans → 只读四列（现状不动）
后端 boards_registry.load_cards(id)            manual     → 手动三列（新）
  kind == "lang_plans" → lang_plans.scan()      写操作 → 新四端点
  kind == "manual"     → manual_store.load()   （registry 按 kind 分发）
```

**模式切换即板切换**：不引入独立的"模式"状态机——板条目的 kind 决定页面
形态与可写性。这保住 Plan 579 的配置驱动架构（新看板类型 = 新适配器 +
一条配置），普通模式只是第二个 kind。

### 数据流（手动板）

```
[front: manual 板页]  ←HTTP←  GET  /api/boards/manual/cards          （已有端点，复用）
        │                       POST /api/boards/:id/cards            （新：创建）
        ├─HTTP→                 PUT  /api/boards/:id/cards/:cid       （新：编辑）
        │                       PUT  /api/boards/:id/cards/:cid/move  （新：移动/排序）
        │                       DEL  /api/boards/:id/cards/:cid       （新：删除）
        ↓ 每次变更后重拉 cards（022 k1 模式：mutate → all_cards()）
[back: manual_store.at]  无状态文件后端：
    load():  read_text(data_path) → json.parse → 动态导航 → []Card（排序后返回）
    save():  []Card → 手工 JSON 序列化（json_escape）→ write_text 全量重写
    data_path: Env.get("AUTO_KANBAN_DATA") → 默认 "data/manual.json"（相对 CWD）
```

无状态选型理由（对比模块级 var 缓存，015-notes db.at 先例）：与 lang_plans
每请求重扫同构（Plan 579"每请求全量重扫"裁定的小语料延续）；重启天然安全；
测试隔离免 reset 端点（换文件即换库）；零 VM 模块 var 状态风险。语料量级
（个人看板 < 数百卡）下每次全量读写开销可忽略。

### 手动板看板列与卡片（UI 规格）

```
┌ 顶部快速添加行：[输入卡片标题……] [＋ 添加]（默认进「待办」列）
├ 三列：待办(todo) / 进行中(doing) / 完成(done)，列头 + 计数徽章，列尾「＋」
├ 卡片：[P0 徽章] 标题（截断）…… [◀][▶] [▲][▼] [×]
│        详情首行预览（灰小字，截断）· updated_at
└ 编辑面板（点击卡片展开，板下方）：标题 input / 优先级 P0|P1|P2 单选按钮组 /
   详情 textarea（多行）/ [保存] [取消]
```

- **优先级**：`P0`（紧急，红）/ `P1`（高，橙）/ `P2`（普通，灰，默认）。
  徽章颜色由视图字段比较条件渲染（无函数调用，PLAN-616 约束安全）。
- **删除两击确认**：首击 `×` → 该卡变红色「确认」按钮（store.
  confirm_delete_id 标记），再击执行；点其他卡的 × 切换确认目标。不依赖
  modal/timer。
- **移动双通道**：`◀▶` 按钮移动相邻列（双端可用，playwright 断言通道）；
  Vue 拖拽（draggable/ondrop，022 先例）为增强通道（见 §10-1）。
- **排序**：`▲▼` 与相邻卡交换 order；跨列移动落目标列尾部。

## 3. 技术栈

- **app**：AutoUI .at，双端（Vue `auto run` :17100 / VM `auto run -r vm`
  iced），端口 17100/17101 不变。
- **写端点先例**：examples/ui/022-kanban `src/back/api.at`（POST/PUT/DELETE
  `#[api]` 形态）与 015-notes（POST/PUT/DELETE/PATCH，`auto run -r vm` 实跑
  ——VM HTTP 轨写端点可用的实证）。
- **前端件先例**：`input{value,oninput,placeholder}`（022 board.at:23-29）、
  `textarea`（015-notes editor.at:151）、条件面板（015-notes app.at:71-73，
  widget 调用可被 if 包裹）、拖拽（022 board.at:52-53 draggable/ondragstart/
  ondragover.prevent/ondrop）。
- **stdlib 能力**（已核对签名）：`file.write_text(path,content) int`、
  `file.read_text`、`file.exists`、`fs.join`、`fs.create_dir`、`str.replace
  (from,with)`、`str.split/find/sub(END 语义)/trim`、`.to_int()/.str()`、
  `json.parse` + JsonValue 动态导航（`parsed.cards[i].field`）、`time.now()`。
- **VM 轨约束**（Plan 579 实测 + PLAN-616 示例侧 DSL/VM 约束表，
  015-notes tests/acceptance.atd:189-202）：
  - 无 `List[T]` 泛型容器——裸数组 `[]T` + push；
  - `json.decode[T]`/`encode[T]` 泛型实例化调用 VM codegen 不支持
    （codegen.rs:9713，Plan 579 T8 探针定案）→ 读取动态导航、写入手工序列化；
  - 后端模块级 `.field = []` / 局部 `[]T` 赋回状态字段会 `Assignment to
    complex LHS` panic → 类型化局部量构建后整体赋值或重建列表整体替换
    （022 db.at delete_card 重建模式）；前端 store `.archived_recent = []`
    在现 app 已实测可用（T16 证据），store 上下文不属此约束；
  - 视图条件不能调方法（`x.contains(y)` 恒假）→ 过滤下沉 store / 字段
    相等比较（现行 `card.column == "drafting"` 同型，安全）；
  - VM view 不能调函数（Plan 402）→ 列计数/标签一律 handler 预格式；
  - `flex-wrap` VM 降级单行 → 多行容器拆分；按钮显式 `w-auto`；
  - `text "${x}"` 插值两端都渲染字面量 → 只用 `text <ref>`；
  - JsonValue 元素先物化（`""+val`）再调 str 方法（探针 a6a/mat 定案）。
- **测试**：playwright（tests/ 现有 run.mjs 编排器复用，加 `AUTO_KANBAN_DATA`
  注入）；VM 轨一致性 autoui-verifier（`test_vm_mcp.py`，Plan 579 T16 先例）。
- **验证门档**：不改 auto-lang `crates/`——严禁 auto-lang `cargo t`/`docs_gen`
  （Category A）。

## 4. 需求分析与背景调查

（调查于 2026-09-15 本会话；代码基线 auto-kanban main fc0434f——src/back
三模块 + src/front 三文件 + boards.json 单条目 + tests 8 用例全绿）

1. **用户需求原文**（2026-09-15）："现在我们的看板应用只支持一种模式：对
   项目中的计划的只读展示。请添加普通模式：和普通的看板app类似的模式
   （可以手动添加计划，可以编辑细节等）。请先按照常见的看板app，进行详细
   的设计，然后立项用 auto-plan-new 写下实施计划。"——先设计后立项，本
   计划 §2/§5 即该设计。
2. **授权记录**：范围 = auto-kanban 添加普通模式（双模式并存）；允许仓库 =
   auto-kanban（全部实现）+ auto-os（本计划文件、spec 增量）；未指定预算
   与自动续行限额；不改 auto-lang（Category A 沿用）。
3. **常见看板 app 功能测绘**（Trello/Notion board/Linear/Jira board 共性）：
   看板=列+卡片；卡片=标题+详情+优先级+时间戳；交互=快速添加、详情编辑、
   拖拽/按钮移动、排序、删除确认；持久化。v1 取其并集的核心子集（§1），
   排除协作/筛选/多板管理（单用户本地工具无此紧迫性）。
4. **读写能力实证**：022-kanban 证明 `#[api(POST/PUT/DELETE)]` + input +
   拖拽全链路；015-notes 证明同型后端在 **VM 轨**实跑（README "auto run -r
   vm" + 19 条 MCP 场景）且 textarea/条件面板可用——普通模式无框架缺口。
5. **配置驱动 seam 现状**：boards_registry.load_cards 已按 kind 分发
   （boards_registry.at:85-88）；BoardsStore.SelectBoard 已含 kind 可判
   （store.boards[i].kind）；写路径只需在同一分发点扩一条臂 + api.at 加
   四端点。BoardDef.root_env 对 manual 板填 ""（字段保留，契约不破）。
6. **持久化写法约束**：手工 JSON 序列化必须过转义（`"`/`\`/换行/制表），
   `str.replace` 链可用；换行字面量 `"\n"` 在 lang_plans split 用例已证。
   `fs.create_dir` 兜底 data/ 目录不存在（首创建即建目录再写）。
7. **spec 现状**：auto-os `docs/specs/apps/` 已有 klondike/minesweeper/
   tetris 三个 app spec 先例，无 kanban spec——本计划新增（§5 规范增量）。
8. **历史计划查重**：auto-os 活跃 014/016/019/020 无看板需求；auto-lang
   归档 579 是本 app 的创建计划（只读定位），其"明确不做写回"边界针对
   lang_plans 数据源（计划状态翻转归 auto-plan 四技能）——本计划的写
   操作作用于**新的独立数据文件**，不写回 auto-lang 计划文件，与 579
   边界不冲突；README 的"只读仪表盘"定位表述将随本计划更新为双模式。

## 5. 详细设计

### 5.1 数据模型（api.at）

```auto
/// 泛化 Card 扩展（+3 字段，两 source 契约：全字段必填）
pub type Card = {
    id: str            // manual 板 = 递增十进制串 "1","2",…
    title: str
    column: str        // manual 板 ∈ todo|doing|done；lang_plans 五态
    badge: str         // manual 板恒 ""（parked 是 lang_plans 语义）
    current: int       // manual 板恒 0
    total: int         // manual 板恒 0
    progress: str      // manual 板恒 "—"
    updated_at: str
    archived: bool     // manual 板恒 false
    file: str          // manual 板恒 ""（无源文件）
    detail: str        // ★新：多行详情；lang_plans 恒 ""
    priority: str      // ★新：P0|P1|P2；lang_plans 恒 ""
    order: int         // ★新：列内序（升序渲染）；lang_plans 恒 0
}
```

lang_plans.at 构造 Card 处补三个默认值字段（一处字面量，零逻辑变化）。

### 5.2 数据文件（data/manual.json）

```json
{
  "cards": [
    { "id": "1", "title": "写周报", "detail": "含上线清单", "column": "todo",
      "priority": "P2", "order": 0, "updated_at": "2026-09-15T10:00:00" }
  ]
}
```

- 路径解析：`Env.get("AUTO_KANBAN_DATA")` 非空 → 用之；默认
  `data/manual.json`（相对运行 CWD，与 boards.json 同基准）。
- **无 created_at 字段**：Card 模型无此字段（只读板无此语义），v1 只记
  updated_at，减少双轨字段。多记需求出现时再加（schema 版本化 v2 课题）。
- id 分配：`max(existing.to_int()) + 1`（加载时推导，文件不存计数器——
  删除后 id 不复用由 max+1 自然保证……注意：删除最大 id 后 max 会回退、
  新卡复用已删 id。可接受（id 只作本文件内主键，无外部引用），不改。

### 5.3 manual_store.at（新文件 src/back/manual_store.at）

```auto
// 无状态文件后端。公开面：
pub fn load() CardsResult          // 读+解析+排序（column 序 todo<doing<done 内按 order 升序）
pub fn create(title str, column str) Card        // 追加目标列尾部，updated_at=now，save 后返回
pub fn update(id str, title str, detail str, priority str) Card
pub fn move_card(id str, column str, order int) Card   // 列移动+排序合一（前端给目标序号）
pub fn remove(id str) bool

// 内部：
fn data_path() str                 // env 覆盖 → data/manual.json
fn load_cards_only() []Card        // 文件缺失/解析失败 → []（空态，不算错误）
fn save(cards []Card) int          // 全量重写（先 fs.create_dir("data") 兜底）
fn card_to_json(c Card) str        // 手工序列化单卡（json_escape 各 str 字段）
fn json_escape(s str) str          // \\ → \\\\ 、 " → \" 、\n → \\n、\t、\r
fn next_id(cards []Card) str       // max(to_int)+1
fn resort(cards []Card) []Card     // 列权重 + order 双键稳定排序（选择排序即可，小语料）
```

- 读取：`json.parse(text)` → `parsed.cards` 动态导航（boards_registry.at:41
  先例）；元素字段经 `.get`/点导航取 `as_string`/`as_int`，str 拼接物化。
- 写入：整文件 = `"{\"cards\":["` + join(map(card_to_json)) + `"]}"`；
  无数组 join 原语 → for 循环拼 `var out str`，卡间补逗号（首卡旗标）。
- **排序稳定性**：resort 用双键（列权重 0/1/2、order），同键保持原序
  （选择排序交换会破坏稳定性 → 用"逐轮取最小+移出"的稳定选择排序或插入
  排序；小语料 O(n²) 无所谓，注释写明稳定性要求）。
- **无模块级可变状态**：全部函数局部量进、构造结果出（规避 §3 后端
  `.field = []` panic 族；022 db.at 重建模式同型）。

### 5.4 API 端点（api.at，registry 分发）

```auto
#[api(method = "POST",   path = "/api/boards/:id/cards")]
pub fn create_card(id str, title str, column str) Card
#[api(method = "PUT",    path = "/api/boards/:id/cards/:cid")]
pub fn update_card(id str, cid str, title str, detail str, priority str) Card
#[api(method = "PUT",    path = "/api/boards/:id/cards/:cid/move")]
pub fn move_card(id str, cid str, column str, order int) Card
#[api(method = "DELETE", path = "/api/boards/:id/cards/:cid")]
pub fn delete_card(id str, cid str) bool
```

- boards_registry 各加"按 id 找 def → kind=="manual" → manual_store.xxx，
  其余 kind 返回零值 Card{…,"id":"",…}/false"分发臂（load_cards 同型扩写）。
- 写操作非法参（空 title / 未知 column / 未知 cid）：create 空标题落零卡
  （前端本就拦空）；update/move 未命中返回零卡（id=""）；remove 返回 false
  ——前端以重拉 cards 为准（响应值仅诊断用，不驱动 UI 断言）。

### 5.5 boards.json

```json
{ "boards": [
  { "id": "lang-plans", "title": "计划", "kind": "lang_plans", "root_env": "AUTO_LANG_ROOT" },
  { "id": "manual", "title": "普通看板", "kind": "manual", "root_env": "" }
] }
```

### 5.6 前端 store（boards_store.at 扩展）

新 model 字段：

```auto
var current_kind str = ""          // SelectBoard/Init 时随 current_id 一并置位
var confirm_delete_id str = ""     // 两击删除的当前确认目标（""=无）
// 编辑面板表单缓冲（Open 时从卡拷入，Save 时提交）
var editing_id str = ""
var edit_title str = ""
var edit_detail str = ""
var edit_priority str = ""
// 手动板三列预格式计数（VM view 零函数）
var c_todo str = "0" / var c_doing str = "0" / var c_done str = "0"
```

新 msg：`AddCard(str), OpenEditor(str), CloseEditor, SaveEditor, TitleInput(str),
DetailInput(str), SetPriority(str), RemoveCard(str), AskRemove(str), MoveCard(str, str),
ShiftCard(str, int), DragCard(str), DropCol(str)`。

handler 要点：

- `LoadCurrent` 分流：current_kind=="manual" 时预格式 c_todo/c_doing/c_done
  （复用现计数循环加三分支），.empty 仅对 lang_plans 板置位（手动板空=引导
  添加，不是错误空态）。
- `AddCard(title)`：空拦 → `create_card(id, title, "todo")` → 重拉。
- `OpenEditor(cid)`：从 .cards 查卡拷入三个缓冲字段。
- `SaveEditor`：`update_card(...)` → 清 editing_id → 重拉。
- `AskRemove(cid)`：confirm_delete_id 置/换；`RemoveCard(cid)`：确认且
  cid 相符 → `delete_card` → 清确认位 → 重拉。
- `MoveCard(cid, col)`：目标列 = 该列现卡数（追加尾部 order）→ `move_card`。
- `ShiftCard(cid, ±1)`：与相邻卡交换 order（一次 move_card 两调用或后端
  重排——**定案：后端 move 语义按"目标位插入"实现**：前端给目标 order，
  后端把同列 order≥目标的卡顺移 +1 后落位，天然支持交换与插入两种用法；
  ▲▼ = 与前/后卡交换两次 move 调用，简单直白）。
- 拖拽：DragCard 记 id，DropCol(col) → MoveCard（022 模式）。

**move 后端插入语义**（manual_store.move_card）：
```
目标列内：order >= order_in 的卡 order += 1 → 目标卡 {column, order_in}
最后统一"压紧"（该列按序重编号 0..n-1）再 save——防长期交换后 order 稀疏膨胀
```

### 5.7 前端页面（board.at 拆分 + 新部件）

- `pages/board.at` 收薄：按 `.store.current_kind` 条件渲染——只读分支保持
  现四列+折叠区（**代码原样搬进分支，零行为变化**）；manual 分支渲染新部件
  `ManualBoard()`。
- 新文件 `src/front/pages/manual_board.at`（widget ManualBoard）：
  - 快速添加行（input + ＋按钮，022:23-29 同型）；
  - 三列（列头中文名+计数徽章；`for card in .store.cards` + `if card.column
    == "todo"` 字段比较过滤，现行同型）；
  - 卡片行：P0/P1/P2 徽章（条件样式）+ 标题 + 详情首行预览（detail 按换行
    split 的首段，**store 预计算**存入？——Card.detail 直接传给 `text`，
    多行文本在 VM text 的渲染未证 → **设计：卡片上不显详情预览，只在编辑
    面板显**，规避多行 text 渲染风险；卡片显示 `[P0] 标题 · 09-15`）；
  - 卡片操作行：◀ ▶ ▲ ▼ ×（`w-auto` 显式宽度，PLAN-616）；× 确认态红底
    "确认"（store.confirm_delete_id 比较）；
  - 卡片整体 onclick → OpenEditor（拖拽通道用 draggable 分离，022 同型）；
  - 编辑面板（板下方条件渲染）：标题 input / P0|P1|P2 三按钮单选（选中态
    高亮，字段比较）/ 详情 textarea / 保存+取消。
- `app.at`：header 板切换按钮加**当前板高亮**（current_id 比较换样式，
  两模式共用的小改进）；其余不动。

### 5.8 测试基建

- fixture：`tests/testdata/manual/manual.test.json`——预置 5 卡（todo 3 /
  doing 1 / done 1，含 P0×1、带引号与换行 detail×1、不同 order）。
- run.mjs env 注入 `AUTO_KANBAN_DATA=<repo>/tests/testdata/manual/
  manual.test.json`（与 AUTO_LANG_ROOT/AUTO_KANBAN_BOARDS 并列）。
- **测试隔离**：无状态文件后端的红利——spec 文件级 `beforeAll` 用 node fs
  重写 fixture 模板到该路径（服务器每请求重读，立即生效；无需 reset 端点）。
  模板存 `tests/testdata/manual/manual.template.json`，运行时拷贝为
  `manual.test.json`（后者入 .gitignore）。
- 真实数据保护：AUTO_KANBAN_DATA 不设时写 `data/manual.json`（仓根
  data/ 入 .gitignore——用户本机数据不入库）。

### 5.9 规范增量

| delta_id | add/modify/retire | docs/specs/… target | before/after rule | rationale | acceptance IDs |
|---|---|---|---|---|---|
| SD-01 | add | auto-os `docs/specs/apps/kanban.md` | before：无 kanban app spec（app 仅有 Plan 579 归档叙述）。after：双模式架构契约——模式=板=kind；泛化 Card 十三字段全填契约；manual 无状态文件后端（AUTO_KANBAN_DATA 解析序/全量重写/JSON 转义）；写四端点契约；VM 约束适配清单引用 | 首个 app 侧双模式数据源，行为契约需可引用的 spec 载体（merge 时落盘） | AC-01..AC-08 |
| SD-02 | modify | auto-kanban `README.md`（非 specs 目录，随任务更新；此处登记口径） | before："只读仪表盘：不做写回"。after："双模式：只读计划板 + 手动普通看板；写操作仅作用于 manual 板独立数据文件，不写回数据源" | 定位表述与新行为一致；写回禁令细化为主体限定（lang_plans 源仍禁写） | AC-01 |

frontmatter 对应：`new_spec_components: [docs/specs/apps/kanban.md]`、
`affects: [docs/specs/apps/kanban.md]`（review 时定稿）。

## 6. 测试设计

| 层 | 内容 | 判定 |
|---|---|---|
| P1 API 语义（curl 或 playwright request） | GET 空库/有库；POST 建 todo 尾卡；PUT 改三字段（含引号/换行 detail 往返转义）；move 插入语义（中位插入后序号压紧）；DELETE；未知 cid 行为 | 响应 JSON 断言 + 落盘文件内容断言（node fs 读） |
| P2 playwright（Vue 轨，~9 新测） | M1 模式切换（三列标题"待办/进行中/完成"+ 计数徽章）；M2 快速添加→卡片出现+计数+1；M3 点卡开面板/改标题详情优先级/保存生效；M4 ◀▶ 列移动+计数迁移；M5 ▲▼ 排序交换；M6 两击删除+计数-1；M7 持久化（变更后 page.reload 卡片仍在）；M8 特殊字符往返（含 " 与换行的 detail 重开面板一致）；M9 零 console error | 全绿；fixture 重置策略见 §5.8 |
| P3 回归 | 现有 8 测（只读板四列/计数/进度/parked/折叠/刷新/console/双板切换）在双板配置下全绿 | `cd tests && npm test` 一把过 |
| P4 双端一致 | VM 轨 `auto run -r vm` + autoui-verifier `test_vm_mcp.py`：快照见双板按钮+普通板三列与计数链；按钮通道加卡/移动一轮 MCP 操作 | 快照+操作断言，证据入本文件复审节 |
| 门档纪律 | auto-lang 零 cargo/docs_gen；不写 auto-lang 仓任何文件 | 复审核对 |

## 7. 验收标准

- **AC-01 双模式切换**：header 有「计划」「普通看板」两按钮，当前板高亮；
  切到普通看板见三列（待办/进行中/完成）可编辑界面；切回计划板为现有四列
  只读界面。验证：P2-M1 + 现有 T1/T8 回归绿。
- **AC-02 添加卡片**：快速添加行输入标题点「＋添加」→ 待办列尾出现新卡、
  计数 +1；空标题不添加。验证：P2-M2。
- **AC-03 编辑细节**：点卡片开编辑面板，标题/多行详情/优先级（P0/P1/P2
  单选）修改保存后卡片徽章与重开面板内容一致；含英文引号、换行的详情
  往返无损。验证：P2-M3 + M8。
- **AC-04 移动与排序**：◀▶ 按钮在双端可用并迁移计数；▲▼ 交换相邻卡；
  Vue 拖拽到目标列生效（冒烟）。验证：P2-M4/M5 + 人工拖拽一次截图。
- **AC-05 删除**：两击确认删除卡片，计数 -1，确认态可被切换目标打断。
  验证：P2-M6。
- **AC-06 持久化**：所有变更落 `AUTO_KANBAN_DATA` 指向的 JSON 文件（结构
  合法、转义正确）；reload 页面后卡片/列/顺序恢复。验证：P2-M7 + P1 落盘
  断言。
- **AC-07 只读板零回归**：现有 8 测全绿；真实数据对账不变（卡片数 ==
  auto-lang plans 目录 .md 计数）。验证：P3 + C3 沿用命令。
- **AC-08 双端与配置驱动**：VM 轨普通板按钮通道可用（P4）；boards.json
  去掉 manual 条目则按钮与板消失、其余板不受影响。验证：P4 快照 + 临时
  改配置 curl `/api/boards` 复验后还原。

## 8. 执行步骤

（原子任务：精确文件路径 + 确切操作 + 验证命令；每步完成后追加
[✅ 已完成] 一行证据。执行仓：D:/autostack/auto-kanban；worktree 裁定见
§10-2）

- [x] **T-01 后端模型扩展**（AC-01/03 前置）
  api.at Card 加 detail/priority/order 三字段；lang_plans.at 卡构造处补
  ""/""/0 默认值（约 177 行字面量 +1 行）。boards_registry.at 零改动。
  验证：`auto gen` 退出码 0；`cd tests && npm test` 现有 8 测全绿
  （Card 扩字段不破坏只读板）。
  [✅ 已完成 2026-09-15] api.at Card 十三字段（两 source 全字段必填契约入
  注释）；lang_plans.at:177 字面量补 detail:""/priority:""/order:0。验证：
  `auto gen` EXIT=0；`cd tests && npm test` 8 passed (3.0s)。提交
  f4cb452（base fc0434f）。附带发现：auto gen 产出 rust-workspace/（生成
  物，未跟踪未 ignore）——随 T-08 一并入 .gitignore。
- [x] **T-02 manual_store 无状态文件后端**（AC-02/03/04/05/06 后基座）
  新建 `src/back/manual_store.at`（§5.3 全函数面：load/create/update/
  move_card/remove + json_escape/card_to_json/resort/next_id/data_path/
  load_cards_only/save）。挂进 boards_registry.load_cards 分发
  （kind=="manual" → manual_store.load()）。
  验证：`auto gen` 过；起服后 `AUTO_KANBAN_DATA=<tmp>/m.json curl
  :17101/api/boards/manual/cards` 返回 `{cards:[],meta:…}`（空库）；
  手写种子 JSON 后 curl 返回排序后卡片。
  [✅ 已完成 2026-09-15] manual_store.at 全函数面 + registry 分发臂。
  VM 实测约束四条（证据记入，已适配）：①后端编译入口（api.at）Plan 550
  门 E5501 禁裸 nil/null 字面量（use 引入的模块不受门控）→ api.at 零
  nil；②`""+JsonValue`（json.get 结果）得 JSON 编码形态带引号，取原始
  内容必须 json.as_string；③结构体字面量字段位内联函数调用误求值
  （source_root:data_path() → 0），须先 let 绑定再入字面量；④VM HTTP
  位置参数制：路径参数按值绑定（可解析 i32 的推 int）+ 请求体为单个
  原始 JSON 串（http_server.rs build_handler_args，Plan 346）→ id 采用
  "m"+十进制串（§5.2 id 格式偏差记录），body 解包下沉 registry。
  验证：`auto gen` EXIT=0；GET 有库返回 resort 排序后五卡
  （todo 2,5,8 / doing 0 / done 3），quote+newline detail 往返无损。
  提交 e33fd9c。
- [x] **T-03 写四端点 + 分发**（AC-02/03/04/05）
  api.at 加 create_card/update_card/move_card/delete_card 四端点（§5.4
  签名）；boards_registry 各加 manual 分发臂（非法参回零卡/false）。
  验证：curl 全链路——POST 建卡→GET 见；PUT 改 detail（含 `"` 与换行）
  →落盘文件 node 读回转义正确；move 中位插入→GET 序号压紧 0..n-1；
  DELETE→GET 无此卡；未知 cid PUT 回零卡。
  [✅ 已完成 2026-09-15] §5.4 签名按 VM HTTP 实况修订（见 T-02 证据②④）：
  四端点签名 (path_params…, body str)，registry 增 create_card/update_card/
  move_card/delete_card 分发臂 + is_manual kind 门 + body_str/body_int
  解包。move 定案落地为"用户可见序（order 升序）目标位插入 + 全列压紧"
  （初版按文件序插入被实测纠正：头卡移 0 位乱序 → 改 resort 后序列插入，
  move/delete 统一 resort+renumber_columns 存盘规范序）。create 空标题拒
  创建（落零卡不落盘，AC-02）。curl 全链路实测：POST→GET 见（doing 尾
  order=1，P2 默认）；PUT 含 `"`/换行 detail → 落盘文件 python json 读回
  转义正确；move 头/尾/跨列插入序号压紧 0..n-1；DELETE true→GET 无此卡；
  未知 cid m999 PUT 回零卡 id=""；lang-plans 板写拒绝（POST 零卡/DELETE
  false，Plan 579 禁写边界保持）。提交 e33fd9c。
- [x] **T-04 boards.json 注册 + 模式切换 + header 高亮**（AC-01/08）
  boards.json 加 manual 条目（§5.5）；boards_store 加 current_kind 置位
  （Init/SelectBoard）+ LoadCurrent 手动板计数分流（c_todo/c_doing/c_done
  预格式，.empty 仅 lang_plans）；app.at 板按钮 current_id 比较高亮；
  board.at 拆只读分支（原代码原样入 if）。
  验证：`auto run` 切换两板——计划板四列原样、普通板三列空态（引导文案
  "点击上方添加第一张卡片"）；截图 `screenshots/021-t04-mode.png`。
  [✅ 已完成 2026-09-15] boards.json §5.5 落盘；store 增 current_kind
  （Init/SelectBoard/Refresh 三路置位——Refresh 后 boards 整体重建需按 id
  恢复 kind，计划外小加固）+ c_todo/c_doing/c_done_m 预格式（c_done 名被
  lang_plans execution_done 计数占用，manual done 列计数记 c_done_m，命名
  偏差记录）+ manual_empty 旗标（视图零函数判空）+ .empty 仅 lang_plans；
  app.at 沿 015-notes chip 两态样式先例（style 命名 + `style: if b.id ==
  .store.current_id {tab_btn_on} else {tab_btn}`）；board.at 只读树原样搬进
  lang_plans 分支 + manual 分支渲染 ManualBoard()。布局裁定：组件 widget 放
  `src/front/manual_board.at`（非 pages/——pages/ 生成路由页，front 根生成
  components/，015-notes editor/sidebar 先例；初放 pages/ 致 vite 导入解析
  失败 8 测红，移动后绿）。验证：`auto gen` EXIT=0；`npm test` 8 passed
  (2.5s)（只读板零回归）；playwright 双板截图：计划板四列原样（真实语料
  扫描 ~3s 后）+ 普通板三列（待办/进行中/完成）计数徽章 0/0/0 + 空态引导
  + 当前板高亮（screenshots/021-t04-mode-langplans.png、021-t04-mode.png）。
  提交 dfde67e。注意：后端单 worker，首屏 lang_plans 扫描期间 manual 请求
  排队（测试/截图需等首轮 LoadCurrent 完成——计数标签出现为就绪信号）。
- [x] **T-05 手动板页：三列 + 快速添加**（AC-01/02）
  新建 `src/front/pages/manual_board.at`（widget ManualBoard，§5.7）：
  快速添加行 + 三列渲染 + 计数徽章 + AddCard handler 链（store）。
  验证：添加两张卡出现在待办列、计数 2；`npm test` P2-M1/M2 绿。
  [✅ 已完成 2026-09-15] 三列壳随 T-04 落地；T-05 增快速添加行
  （input value/oninput + ＋添加，022 同型）+ store AddCard 链（空拦 →
  create_card → LoadCurrent）+ esc() body JSON 串构造。布局：组件在
  `src/front/manual_board.at`（见 T-04）。**框架契约重要实证（记入
  spec 增量素材）**：生成客户端按 .at 形参名打包非路径参数
  （(id,body) → 请求体 {"body":"<store 构造的 JSON 串>}，双层编码），
  registry body 解包兼容双形态（curl 顶层直发 / 客户端 body 包装）；
  缺字段判定必须 json.has_key——json.get 缺失键返回非 nil Null（实测，
  == nil 判不中）。验证：playwright 前端实测加两卡 → 待办列两卡可见
  + 计数徽章 2 + 零 console error + data/manual.json 落盘正确（截图
  screenshots/021-t05-add.png）；`npm test` 8 passed (2.9s) 只读回归绿
  （P2-M1/M2 正式断言随 T-08 manual.spec.ts 固化——T-05 先以脚本实测
  等价验证）。提交 57f59bc。环境教训（记录）：本轮多台 --server vm
  调试服并存导致响应串台假象（TaskStop 只杀 shell 不杀 detached
  子进程；17103 曾双进程同听）——验证一律先按端口清场。
- [x] **T-06 编辑面板**（AC-03）
  store 加 editing_id/edit_* 缓冲 + OpenEditor/CloseEditor/SaveEditor/
  TitleInput/DetailInput/SetPriority handlers；manual_board.at 加条件
  编辑面板（input/textarea/三优先级按钮/保存取消）。
  验证：P2-M3/M8 绿（特殊字符往返）；截图 `021-t06-editor.png`。
  [✅ 已完成 2026-09-15] store 缓冲四字段 + 六 handler；面板含标题
  input/详情 textarea/优先级三按钮单选（选中态高亮，style if 比较）/
  保存取消；卡片整体 onclick → OpenEditor。实现修正：store 字段 input
  绑定为单向（:value + @input），新值必须经带参 handler 回写（msg
  EditTitle(str) → .EditTitle(v str) → store.EditTitle(v)，015-notes
  editor.at:26,162 先例）——空参 handler 写法致编辑不生效，已修正。
  验证：playwright 往返实测——点卡面板开、改标题（含英文双引号）/
  多行详情（换行+引号）/P0 保存 → 卡片标题更新 + P0 徽章出现 + 面板
  收起；重开面板三字段往返无损（title/detail JSON.stringify 比对一致，
  P0 高亮 class 含 border-red-400）；零 console error。截图
  screenshots/021-t06-editor.png、021-t06-editor-reopen.png。
  提交 17384e3。
- [x] **T-07 移动/排序/删除/拖拽**（AC-04/05）
  store 加 AskRemove/RemoveCard（两击确认）/MoveCard/ShiftCard/DragCard/
  DropCol；卡片操作行 ◀▶▲▼× + Vue draggable/ondrop（022 同型）+ 确认态
  样式。
  验证：P2-M4/M5/M6 绿；人工拖拽一次成功（截图 `021-t07-drag.png`）。
  [✅ 已完成 2026-09-15] store 六 handler + widget 按钮/拖拽接线。
  实现修正：卡片 onclick 收敛至**信息行**（徽章+标题+时间），操作行按钮
  不再冒泡触发编辑面板（实测全卡 onclick 时按钮点击冒泡开面板——
  Vue 事件冒泡，生成端无 .stop 通道；计划"卡片整体 onclick"按此收敛，
  交互语义不变）。▲▼ 定案落地：目标位插入语义下与邻卡换位 = 一次
  move_card 调用（邻卡 order 即目标位；边界无邻卡为安全 no-op）。
  验证：playwright 实测——▶ 卡A todo→doing 计数迁移 ✓；▲ 卡C 与卡B
  换位（todo 序 C<B）✓；两击删除（× → 确认红钮 → 卡消失 + 服务端
  落盘重编号压实）✓；HTML5 拖拽 todo→done 成功 ✓（playwright dragTo，
  截图 screenshots/021-t07-drag.png、021-t07-actions.png）；零 console
  error；`npm test` 8 passed (2.8s)。提交 ac8d90e。环境记录：本轮
  auto-lang docs/plans/415 在被并发写入时扫描可致 VM 后端崩溃（撕裂
  读，框架层既有暴露，非本计划引入；语料稳定后自愈）。
- [x] **T-08 fixture + 套件 + 隔离**（AC-06 测试面）
  `tests/testdata/manual/manual.template.json`（§5.8 五卡分布）；
  board.spec.ts 旁新建 `manual.spec.ts`（M1-M9 + beforeAll 模板拷贝）；
  run.mjs 注入 AUTO_KANBAN_DATA；`.gitignore` 加 data/ 与 manual.test.json。
  验证：`cd tests && npm test` 全绿（8 旧 + ~9 新）。
  [✅ 已完成 2026-09-15] template 五卡（m1 P0 含引号+换行 detail /
  m2 / m3 todo；m4 doing；m5 done）；boards.test.json 增 manual 条目
  （board.spec 共用一份配置，T8 双板断言不受第三钮影响——实测通过）；
  boards.with-manual.json（T-03 临时验证用）删除；manual.spec.ts M1-M9
  以 card() 定位器（div.shadow-sm hasText）作用域化按钮点击。隔离策略
  微调：模板重拷放 **beforeEach**（计划为 beforeAll——九测均变更数据，
  每测重置保独立性，偏差记录）；M7 reload 后需重切普通看板（回到首板）。
  路径基准 process.cwd()（import.meta 在 playwright 转译管道不可用，实测）。
  .gitignore 增 data/、tests/testdata/manual/manual.test.json、
  rust-workspace/（生成产物；与用户侧 screenshots 悬置修改分属两 hunk，
  仅本计划行入库）。验证：`npm test` **17 passed (5.5s)**（8 旧 + 9 新）。
  提交 957b990。
- [x] **T-09 双端一致 + 真实数据回归**（AC-07/08）
  autoui-verifier：VM 轨起服 → test_vm_mcp.py 快照（双板按钮/三列/计数
  链）+ 按钮通道加卡/移动一轮；真实数据轮（AUTO_LANG_ROOT 不设）对账
  C3 沿用。
  验证：快照与断言记录入本文件复审节；对账数字相等。
  [✅ 已完成 2026-09-15] **结构修正（先于验证）**：ManualBoard 独立组件
  在 iced 路由页不渲染（跨文件组件调用从 pages/ 页 widget 发起时
  AURA 组件解析失败——Vue 轨正常、015-notes 因无 pages/ 不受影响；
  最小复现 + 同文件 widget 对照实测定位）→ **ManualBoard 视图整体内联
  board.at**（计划 §5.7 结构偏差记录：不拆组件、单 widget 双分支，
  022 同型），内联前后 Vue 轨 17 测全绿。VM 轨（`auto run -r vm` +
  AUTOUI_MCP_PORT + autoui-verifier 驱动）：初屏双板按钮（计划/普通看板）
  + 只读四列 ✓；press 普通看板 → 三列（待办/进行中/完成）+ 快速添加行
  ✓；type+press 按钮通道加卡 → state c_todo "0"→"1" ✓；press ▶ 移动 →
  c_doing "1"/c_todo "0" ✓（零拖拽依赖，§10-1 双端差异不触发）。截图
  plan021_vm_manual 存 tests/screenshots。VM 环境注记：iced 进程内
  后端对相对路径 boards.json 读取失败（CWD 与预期不符）→ 部署/验证
  用绝对路径 env（桌面标准入口注入绝对路径，语义一致）。
  **真实数据对账（C3）**：AUTO_LANG_ROOT 指真实 auto-lang →
  cards 627 == docs/plans 4 + archive 623 = 627 ✓（总数相等）；拆分
  active 3 / archived 624 vs 目录 4/623 的差异为 corpus 侧：4 个无
  status frontmatter 的 tracker 文件（242/415/INDEX/KNOWN-DEBT-AND-
  RISKS，非 auto-plan 计划文件）按 Plan 579 未知态映射入 archived 折叠
  ——扫描逻辑本计划零改动。AC-07 总数对账达成。`npm test` 17 passed。
  提交 e712eb5。
- [x] **T-10 收尾：README + spec 增量 + 健康检查**（SD-01/SD-02）
  README.md 定位改双模式（§5.9 SD-02 口径）+ API 表补四端点 + 运行节补
  AUTO_KANBAN_DATA；spec 增量草案文本写入本计划（merge 时落
  docs/specs/apps/kanban.md）；`grep -rn "console.log\|debugger\|print("
  src/` 零残留；两仓 git status 核对。
  验证：三条命令输出贴入证据。
  [✅ 已完成 2026-09-15] README 双模式定位（SD-02 口径：写操作仅作用于
  manual 板独立数据文件，lang_plans 源仍禁写）+ 看板注册示例双条目 +
  API 表补写四端点（含 kind 门说明）+ 结构节补 manual_store.at。
  健康检查：`grep -rn "console.log\|debugger\|print(" src/` 零残留；
  auto-kanban git status 仅剩 .gitignore 用户侧悬置 hunk（本计划 3 行
  已入库）；auto-os 本计划仅动 docs/plans/021（仓内其余改动为用户侧
  并行工作，不并入）。提交 5f3b4ff。

## 9. 复审记录

- 2026-09-15 · stage: new · plan_revision: 1 · ZCode（/auto-plan:new 起草）
  - outcome: **pass**（授权范围内可交 /auto-plan:work；无阻断决策点）
  - next: **work**（T-01 起）；待澄清 §10-1（VM 拖拽）与 §10-2（worktree
    形态）为执行期裁定项，不阻断 T-01..T-06。
- 2026-09-15 · stage: work · plan_revision: 2 · ZCode（/auto-plan:work）
  - outcome: **pass** — T-01..T-10 全部完成，验收映射齐全：
    AC-01（M1/T1/T8）✓ · AC-02（M2）✓ · AC-03（M3/M8）✓ ·
    AC-04（M4/M5 + playwright dragTo 拖拽冒烟截图）✓ · AC-05（M6）✓ ·
    AC-06（M7 + P1 落盘断言）✓ · AC-07（8 旧测全绿 + 627==627 对账）✓ ·
    AC-08（VM 轨 MCP 全链 + boards.json 配置驱动架构不变）✓。
  - code_commit（auto-kanban main）：f4cb452（T-01）→ e33fd9c（T-02/03）→
    dfde67e（T-04）→ 57f59bc（T-05）→ 17384e3（T-06）→ ac8d90e（T-07）→
    957b990（T-08）→ e712eb5（T-09）→ 5f3b4ff（T-10）。base fc0434f。
  - 执行裁定与偏差（均已记录于任务证据）：无 worktree（沿 579 先例）；
    ManualBoard 不拆组件、视图内联 board.at（iced 路由页跨文件组件
    调用不渲染）；manual done 列计数记 c_done_m（c_done 名被占用）；
    id 采用 "m"+十进制串（VM 路径参 i32 强制适配）；写端点签名为
    (path_params…, body str)（VM HTTP 位置参数制）；fixture 重拷放
    beforeEach（测间独立性）。
  - VM 约束新实证（spec 增量素材）：后端编译入口禁 nil（E5501，use
    模块豁免）；json.get 缺失键返回非 nil Null（须 has_key）；
    ""+JsonValue 得 JSON 编码形态（取原始值须 as_string）；结构体字面量
    字段位内联函数调用误求值（须先 let 绑定）；生成客户端按形参名打包
    body（registry 双形态解包）；fs 路径在 iced 进程内需绝对路径。
  - blockers: 无。
  - next: **review**（/auto-plan:review；worktree 保留不适用——本计划
    无 worktree，主检出即执行现场，复审直接对 auto-kanban main）。
- 2026-09-15 · stage: work · spec 增量草案（SD-01 定稿素材，merge 时落
  docs/specs/apps/kanban.md）：
  - 双模式架构：模式 = 板 = kind（boards.json 条目驱动；header 按钮组
    即切换，当前板高亮；去掉条目即从 UI 消失）。
  - 泛化 Card 契约：十三字段全填（lang_plans 填
    badge=""、current=0、total=0、progress="—"、archived、file 与
    detail=""、priority=""、order=0；manual 填 badge=""/current=0/
    total=0/progress="—"/archived=false/file=""）。
  - manual 无状态文件后端：每请求读数据文件、变更全量重写 + 全列
    压紧重编号；路径解析 env AUTO_KANBAN_DATA → data/manual.json；
    JSON 手工序列化（str 字段全过转义，先反斜杠后引号再控制字符）；
    id = "m"+递增十进制（本文件内主键，允许复用）。
  - 写四端点契约：POST/PUT/PUT move/DELETE，经 boards_registry 按
    kind 分发；非 manual 板落零卡/false；lang_plans 源禁写不变；
    move = 用户可见序目标位插入 + 全列压实；create 空标题拒创建。
  - VM HTTP/渲染约束适配清单（引用本文件 T-02/T-03/T-05/T-09 证据）：
    位置参数制 body 串、E5501 入口禁 nil、has_key 判缺、as_string
    物化、let 绑定入字面量、客户端 body 包装双形态、iced 相对路径、
    路由页跨文件组件调用不渲染（ManualBoard 内联）。
- 2026-09-15 · stage: review · plan_revision: 2 · ZCode（/auto-plan:review，
  **与会话内执行同会话复核——独立性受限，已按合同从工件重建裁决：
  全部验证在 reviewed commit 上独立重跑，不采信执行期摘要**）
  - outcome: **pass**
  - reviewed_commit: auto-kanban main `5f3b4ff2a9e6854e6c226f0048aeaec948c598b2`
    · base_commit: `fc0434f3de60c5cbb06a6df3865c201e65fb1724`（祖先关系
    验证 ✓，9 提交，无 worktree——计划 §10-2 裁定的主检出执行）
  - dependency_revisions: auto `0.1.0+v0.4.2-753-g4e26b3237-dirty`
    （auto-lang target/debug，会话中经历 690→753 两次并发重建，复审
    全链以 753 重跑）；无其他依赖仓变更。
  - dirty inventory: 仅 .gitignore 用户侧 hunk（screenshots/图片 ignore，
    非本计划，未入库——HEAD 版已核对只含本计划 3 行）；无未提交实现。
  - scope: diff fc0434f..HEAD 共 14 文件，全部落 auto-kanban 授权仓；
    auto-os 侧仅计划文件（2dd68da 单文件）✓。
  - spec_inputs: docs/specs/apps/kanban.md 尚不存在（SD-01 = new，merge
    落盘）；§9 增量草案经复核——描述即当前行为与持久决策，无执行日记
    残留；supersedes=[]、new=P021-1（docs/specs/apps/kanban.md，与既有
    apps/klondike 等 spec 布局一致）、affects 一致。
  - acceptance_results（独立重跑证据）：
    AC-01 ✓ M1+T1/T8 · AC-02 ✓ M2 · AC-03 ✓ M3 · AC-04 ✓ M4/M5+
    dragTo 拖拽冒烟 · AC-05 ✓ M6 · AC-06 ✓ M7+P1 落盘断言 ·
    AC-07 ✓ 17/17（5.7s，复审重跑）+ 真实对账 cards 629 == 磁盘
    5+624（语料较执行时新增 2 文件后总数仍相等；无 status tracker
    文件入 archived 为 579 既有映射，扫描逻辑本计划零改动）·
    AC-08 ✓ P1 curl 全链（GET 排序/PUT 引号换行落盘转义/move 中位
    插入压实 0..n-1/DELETE true/未知 cid 零卡/lang-plans 拒写零卡）
    + 配置驱动实测（临时移除 manual 条目 → /api/boards 消失，还原
    后恢复，git 工作区零残留）。
  - findings（均 info 级非阻断，框架/语料侧，不在本计划授权范围）：
    R-01 iced 进程内相对路径读配置失败（绝对路径 env 可用，桌面标准
    入口即绝对注入）；R-02 语料并发撕裂读可致 VM 后端崩溃（579 既有
    暴露，执行期偶发两次，语料稳定后自愈）；R-03 statusless tracker
    文件映射入 archived 折叠（579 既有映射）。
  - evidence: tests/tests/{board,manual}.spec.ts（17 测，复审重跑
    17 passed 5.7s）；P1 curl 链（fixture
    %TEMP%/review021/m.json，落盘断言 python 读回）；AC-08 配置移除
    实测（已还原）；screenshots/021-t04…t07*.png；§9 work 记录及各
    任务证据行。复审后工作区核对：boards.test.json 已还原（零 diff）。
  - next: **merge**（/auto-plan:merge；spec 增量按 §9 草案落
    docs/specs/apps/kanban.md）。

## 10. 待澄清事项

1. **VM 轨拖拽支持度未证实**（不阻断）：022 的 draggable/ondrop 在 Vue 轨
   有先例；VM (iced) 轨是否分发拖拽事件无实证。v1 裁定：按钮通道为双端
   主通道（playwright 断言面），拖拽为 Vue 增强；T-09 VM 轮顺带观测，若
   VM 拖拽可用则补注记、不可用则登记为已知双端差异（不修）。
2. **worktree 形态**（已裁定 2026-09-15，/auto-plan:work 入口）：**不建
   worktree 组**——沿 Plan 579 先例（其复审记录明载"无 worktree，产出于两
   新建仓，复验直接对两仓与主检出"），全部实现落在 auto-kanban 主检出
   （main @ fc0434f）；auto-os 侧仅本计划文件回写。main 检出现有 .gitignore
   一处他因悬置修改（screenshots/图片 ignore 扩展，用户侧），保留不并入本
   计划提交。wt-guard 红线本轮不触发（无 worktree 建/删）。
3. **manual 板卡片量级**：无状态全量重写在数百卡内无感；若未来上万卡需
   缓存层，属 v2 课题（现无此需求）。
4. **id 复用**（§5.2 已注记）：删除最大 id 后新卡复用该 id——本文件内主键
   语义下可接受；若未来卡片被外部引用（如深链）则需单调计数器，v2 课题。
