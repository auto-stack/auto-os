# Spec: apps/kanban — 配置驱动双模式看板（只读计划板 + 手动普通看板）

本规范定义 [auto-kanban](../../../auto-kanban)（伞形登记 id: kanban，端口
17100/17101）的核心契约：模式 = 板 = kind 的双模式架构、泛化 Card 全字段
契约、manual 无状态文件后端与写四端点。来源：PLAN-021（r2，reviewed
commit `5f3b4ff2a9e6854e6c226f0048aeaec948c598b2`）；创建计划 PLAN-0579
（auto-lang，只读形态）。

## 1. 双模式架构（模式 = 板 = kind）(SD-01)

- 看板由 `boards.json`（仓根；`AUTO_KANBAN_BOARDS` env 覆盖）注册：
  `{ id, title, kind, root_env }` 每条目一板；header 板切换按钮组即模式
  切换（配置驱动——当前板 id 比较高亮，去掉条目即从 UI 消失）。
- 页面按当前板 `kind` 条件渲染：`lang_plans` → 只读四列板（Drafting /
  Executing / Execution Done / Reviewed + Archived 折叠区，扫描 auto-lang
  `docs/plans` frontmatter，五态状态机映射，PLAN-0579 契约不变）；
  `manual` → 手动三列板（待办 todo / 进行中 doing / 完成 done）。
- 未知 kind → 空 cards + meta.source_root 携错误串（前端空态）。

## 2. 泛化 Card 全字段契约 (SD-02)

`Card` 十三字段，**两 source 全字段必填**（消费端零特判）：

| 字段 | lang_plans | manual |
|---|---|---|
| id | plan_id（缺省文件名 stem） | `"m" + 递增十进制`（本文件内主键，删除最大 id 后复用可接受） |
| title / column / updated_at | frontmatter 映射 / 五态 / frontmatter | 用户输入 / todo\|doing\|done / 每次写刷新 |
| badge / current / total | 状态派生 / step 计数 | `""` / `0` / `0` |
| progress | 预格式 `"3/7"` 或 `"—"` | 恒 `"—"` |
| archived / file | 列派生 / 源文件名 | 恒 `false` / `""` |
| detail / priority / order | 恒 `""` / `""` / `0` | 多行详情 / `P0|P1|P2`（默认 P2）/ 列内序升序 |

## 3. manual 无状态文件后端 (SD-03)

- **无状态**：每请求读数据文件、每次变更全量重写（与 lang_plans 每请求
  全量重扫同构）；全部函数局部量进、构造结果出（后端模块级可变状态在
  VM 上下文会 panic）。
- **路径解析序**：`AUTO_KANBAN_DATA` env → 默认 `data/manual.json`（相对
  运行 CWD）。**iced 进程内相对路径读取不可依赖**，部署/验证一律注入
  绝对路径（桌面标准入口即绝对注入）。
- **数据文件 schema**：`{"cards":[{id,title,detail,column,priority,order,
  updated_at}]}` 七字段/卡；手工 JSON 序列化（str 字段全过转义：先反斜杠、
  再引号、再 `\n`/`\t`/`\r`）；读取走 `json.parse` + 动态导航，str 字段
  `json.as_string` 物化、int 字段 `json.as_int`（禁 `""+JsonValue` 拼接
  ——得 JSON 编码形态；缺失键判定必须 `json.has_key`——`json.get` 对
  缺失键返回非 nil Null）。
- **排序**：读出即 resort（列权重 todo<doing<done、列内 order 升序，稳定
  插入排序）；每次 move/delete 后全列压紧重编号 `0..n-1` 并按规范序存盘
  （防 order 稀疏膨胀）。

## 4. 写四端点契约（经 boards_registry 按 kind 分发）(SD-04)

| Method | Path | body 字段 | 语义 |
|---|---|---|---|
| POST | `/api/boards/:id/cards` | title, column | 建卡落目标列尾（order=列卡数），空标题拒创建 |
| PUT | `/api/boards/:id/cards/:cid` | title, detail, priority | 编辑三字段并刷新 updated_at |
| PUT | `/api/boards/:id/cards/:cid/move` | column, order | 目标位插入（用户可见序），全列压紧；同列相邻交换 = 一次调用 |
| DELETE | `/api/boards/:id/cards/:cid` | — | 删卡 |

- **kind 门**：仅 `kind == "manual"` 的板可写；lang_plans 板写请求落零卡
  /false（PLAN-0579"不写回数据源"边界延续——auto-lang 计划状态翻转归
  auto-plan 四技能）。
- 非法参（未知 cid / 空标题 / 非法列名归一 todo）不落盘；前端以重拉
  cards 为准，响应值仅诊断用。
- **VM HTTP 位置参数制**（Plan 346）：路径参数按声明序绑定且纯数字按值
  强制 i32（故 id 带 `m` 前缀走 str 通道）；请求体以单个原始 JSON 串追加
  到末位形参，字段解包在 boards_registry（api.at 为后端编译入口，Plan 550
  门 E5501 禁 nil 字面量，use 模块豁免）。生成客户端按形参名打包非路径
  参数（`{body: <json 串>}` 双层编码），解包双形态兼容。

## 5. 手动板交互契约 (SD-05)

- **添加**：快速添加行（新卡进待办列尾）+ 列尾「＋」；空标题前后端双拦。
- **编辑**：点卡片信息行开面板（标题 input / 多行详情 textarea / P0-P1-P2
  单选按钮组，选中态高亮）；保存后面板收起、卡片与徽章即时更新。
- **移动/排序双通道**：◀▶ 列迁移与 ▲▼ 相邻换位为**双端保底通道**
  （Iced 无 DOM 拖放，100% 按钮可玩）；Vue 轨 HTML5 拖拽（draggable/
  ondrop → 列迁移）为增强通道。
- **删除**：两击确认——首击 × 该卡变红色「确认」钮（store 记确认目标，
  点其他卡 × 切换目标），再击执行；不依赖 modal/timer。
- **列计数/空态**：列计数徽章为 store handler 预格式（VM view 不能调
  函数，Plan 402）；manual 板空 = 引导添加（非错误空态，与 lang_plans
  空态文案区分）。
- **持久化**：全部变更即时落盘；重启 `auto run` 后卡片/列/顺序原样恢复
  （无状态后端天然保证）。

## 6. 测试与验证门 (SD-06)

- playwright 17 测（`tests/tests/`：board.spec 8 只读回归 + manual.spec
  9 手动板 M1-M9）；fixture 隔离：`manual.template.json` 每测前重拷为
  `manual.test.json`（后端每请求重读，换文件即换库），run.mjs 注入
  `AUTO_KANBAN_DATA`/`AUTO_KANBAN_BOARDS`/`AUTO_LANG_ROOT`。
- 真实数据对账（C3 延续）：lang_plans 卡片总数 == auto-lang
  `docs/plans` + `archive` 的 .md 文件数（statusless tracker 文件按未知
  态入 archived 折叠，579 既有映射）。
- VM 轨验证：`auto run -r vm` + autoui-verifier MCP（snapshot / type /
  press / state 计数链）；双端一致性以按钮通道为断言面。
- 门档纪律：不改 auto-lang `crates/`（严禁 auto-lang `cargo t`/`docs_gen`，
  Category A）。

## 7. UI token 纪律（SD-07 · PLAN-038 Phase A+B）

普通看板/计划板视图样式 **单源** `src/front/{app,pages/board}.at`，两端
（Vue codegen / VM Iced）消费同一 class 串。

- **核心 token 白名单（app 缺省）**：`bg-background` `text-foreground`
  `bg-card` `bg-muted` `text-muted-foreground` `border-border` `border-input`
  `bg-primary` `text-primary` `text-primary-foreground`
  `bg-secondary` `text-secondary-foreground` `bg-destructive` `text-destructive`
  `bg-accent` `text-accent-foreground` 及透明度/布局 utility。
- **禁用（始终）**：palette 硬编码（`bg-blue-500` 等）；**手改** gen 下
  SFC/CSS/tailwind 作为 app 纪律。
- **扩展色（Phase B 后平台能力）**：auto-lang registry + auto-man 脚手架已
  为 scaffold/cli-vue/tauri 生成 `success/warning/info/error` CSS 变量与
  tailwind 映射（与 VM stella 同值）；`auto build` 全路径写
  `vite-env.d.ts`/`auto-select/overlay.ts`/`auto-sources.ts`。`.at` **可以**
  安全使用 `text-success` 等扩展 class。**本 app 缺省**仍只使用核心 token
  （完成列 muted / P1 primary/10），直至产品侧显式恢复扩展色。
- **视觉约定（Phase A 缺省）**：进行中列标题/计数用 `primary`；完成列用
  `muted-foreground` 降噪；优先级 P0=`destructive`、P1=`primary/10`、
  P2=`muted`。
- **gen 产物非长期源**：`gen/front/vue` 由 codegen 重生。根 `index.html`
  仅作手机模拟器镜像，class 对齐同一核心 token 集。
- **交互文案不变**：PLAN-021 SD-05 测试可定位文案（列名/占位符/◀▶▲▼×/
  确认/保存等）与 CRUD 契约不在 token 收敛中变更。
