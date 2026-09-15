---
plan_id: PLAN-021
status: drafting               # drafting → executing → execution_done → reviewed → archived
feature_name: auto-kanban 普通模式——手动看板（卡片 CRUD/列移动/排序/持久化），与只读计划板并存为双模式
author: [zhaopuming, ZCode]
created_at: 2026-09-15
updated_at: 2026-09-15
plan_revision: 1

# /auto-plan:review 结束时填写：
supersedes_spec_components: []
new_spec_components: [P021-1 apps/kanban.md（app spec 新增：双模式架构/泛化 Card 契约/manual 无状态文件后端契约）]
touched_goals: []

affects: [docs/specs/apps/kanban.md]
current_step: 0
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

- [ ] **T-01 后端模型扩展**（AC-01/03 前置）
  api.at Card 加 detail/priority/order 三字段；lang_plans.at 卡构造处补
  ""/""/0 默认值（约 177 行字面量 +1 行）。boards_registry.at 零改动。
  验证：`auto gen` 退出码 0；`cd tests && npm test` 现有 8 测全绿
  （Card 扩字段不破坏只读板）。
- [ ] **T-02 manual_store 无状态文件后端**（AC-02/03/04/05/06 后基座）
  新建 `src/back/manual_store.at`（§5.3 全函数面：load/create/update/
  move_card/remove + json_escape/card_to_json/resort/next_id/data_path/
  load_cards_only/save）。挂进 boards_registry.load_cards 分发
  （kind=="manual" → manual_store.load()）。
  验证：`auto gen` 过；起服后 `AUTO_KANBAN_DATA=<tmp>/m.json curl
  :17101/api/boards/manual/cards` 返回 `{cards:[],meta:…}`（空库）；
  手写种子 JSON 后 curl 返回排序后卡片。
- [ ] **T-03 写四端点 + 分发**（AC-02/03/04/05）
  api.at 加 create_card/update_card/move_card/delete_card 四端点（§5.4
  签名）；boards_registry 各加 manual 分发臂（非法参回零卡/false）。
  验证：curl 全链路——POST 建卡→GET 见；PUT 改 detail（含 `"` 与换行）
  →落盘文件 node 读回转义正确；move 中位插入→GET 序号压紧 0..n-1；
  DELETE→GET 无此卡；未知 cid PUT 回零卡。
- [ ] **T-04 boards.json 注册 + 模式切换 + header 高亮**（AC-01/08）
  boards.json 加 manual 条目（§5.5）；boards_store 加 current_kind 置位
  （Init/SelectBoard）+ LoadCurrent 手动板计数分流（c_todo/c_doing/c_done
  预格式，.empty 仅 lang_plans）；app.at 板按钮 current_id 比较高亮；
  board.at 拆只读分支（原代码原样入 if）。
  验证：`auto run` 切换两板——计划板四列原样、普通板三列空态（引导文案
  "点击上方添加第一张卡片"）；截图 `screenshots/021-t04-mode.png`。
- [ ] **T-05 手动板页：三列 + 快速添加**（AC-01/02）
  新建 `src/front/pages/manual_board.at`（widget ManualBoard，§5.7）：
  快速添加行 + 三列渲染 + 计数徽章 + AddCard handler 链（store）。
  验证：添加两张卡出现在待办列、计数 2；`npm test` P2-M1/M2 绿。
- [ ] **T-06 编辑面板**（AC-03）
  store 加 editing_id/edit_* 缓冲 + OpenEditor/CloseEditor/SaveEditor/
  TitleInput/DetailInput/SetPriority handlers；manual_board.at 加条件
  编辑面板（input/textarea/三优先级按钮/保存取消）。
  验证：P2-M3/M8 绿（特殊字符往返）；截图 `021-t06-editor.png`。
- [ ] **T-07 移动/排序/删除/拖拽**（AC-04/05）
  store 加 AskRemove/RemoveCard（两击确认）/MoveCard/ShiftCard/DragCard/
  DropCol；卡片操作行 ◀▶▲▼× + Vue draggable/ondrop（022 同型）+ 确认态
  样式。
  验证：P2-M4/M5/M6 绿；人工拖拽一次成功（截图 `021-t07-drag.png`）。
- [ ] **T-08 fixture + 套件 + 隔离**（AC-06 测试面）
  `tests/testdata/manual/manual.template.json`（§5.8 五卡分布）；
  board.spec.ts 旁新建 `manual.spec.ts`（M1-M9 + beforeAll 模板拷贝）；
  run.mjs 注入 AUTO_KANBAN_DATA；`.gitignore` 加 data/ 与 manual.test.json。
  验证：`cd tests && npm test` 全绿（8 旧 + ~9 新）。
- [ ] **T-09 双端一致 + 真实数据回归**（AC-07/08）
  autoui-verifier：VM 轨起服 → test_vm_mcp.py 快照（双板按钮/三列/计数
  链）+ 按钮通道加卡/移动一轮；真实数据轮（AUTO_LANG_ROOT 不设）对账
  C3 沿用。
  验证：快照与断言记录入本文件复审节；对账数字相等。
- [ ] **T-10 收尾：README + spec 增量 + 健康检查**（SD-01/SD-02）
  README.md 定位改双模式（§5.9 SD-02 口径）+ API 表补四端点 + 运行节补
  AUTO_KANBAN_DATA；spec 增量草案文本写入本计划（merge 时落
  docs/specs/apps/kanban.md）；`grep -rn "console.log\|debugger\|print("
  src/` 零残留；两仓 git status 核对。
  验证：三条命令输出贴入证据。

## 9. 复审记录

- 2026-09-15 · stage: new · plan_revision: 1 · ZCode（/auto-plan:new 起草）
  - outcome: **pass**（授权范围内可交 /auto-plan:work；无阻断决策点）
  - next: **work**（T-01 起）；待澄清 §10-1（VM 拖拽）与 §10-2（worktree
    形态）为执行期裁定项，不阻断 T-01..T-06。

## 10. 待澄清事项

1. **VM 轨拖拽支持度未证实**（不阻断）：022 的 draggable/ondrop 在 Vue 轨
   有先例；VM (iced) 轨是否分发拖拽事件无实证。v1 裁定：按钮通道为双端
   主通道（playwright 断言面），拖拽为 Vue 增强；T-09 VM 轮顺带观测，若
   VM 拖拽可用则补注记、不可用则登记为已知双端差异（不修）。
2. **worktree 形态**（/auto-plan:work 执行期裁定）：变更全在 auto-kanban
   仓（D:/autostack/auto-kanban 主检出当前 clean，仅 .gitignore 一处他因
   修改）；沿 Plan 579 先例可直接主检出执行；若 work 技能强制 worktree，
   则建组目录 `.wt/os-021/auto-kanban`（AGENTS.md §2 布局）并遵守 wt-guard
   红线。auto-os 仓侧仅有本计划文件与 .next-id 变更。
3. **manual 板卡片量级**：无状态全量重写在数百卡内无感；若未来上万卡需
   缓存层，属 v2 课题（现无此需求）。
4. **id 复用**（§5.2 已注记）：删除最大 id 后新卡复用该 id——本文件内主键
   语义下可接受；若未来卡片被外部引用（如深链）则需单调计数器，v2 课题。
