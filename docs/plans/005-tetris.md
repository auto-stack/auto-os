---
plan_id: PLAN-005
origin: PLAN-557
status: reviewed
feature_name: tetris
author: [zhaopuming]
created_at: 2026-09-05
updated_at: 2026-09-14
plan_revision: 3
supersedes_spec_components: []
new_spec_components: [docs/specs/apps/tetris.md]
touched_goals: []
affects: [auto-os/apps/036-tetris]
current_step: 6
total_steps: 6
---

> **迁移与修订历史**：本计划由 auto-lang `557-tetris.md` 随 PLAN-001 于
> 2026-09-07 迁入 auto-os，保留 origin 与 drafting 状态。2026-09-12 首次建立
> revision 1：按用户要求重新调查标准 AutoUI 架构与 UI。旧版无已完成任务或
> 验证记录；编号 036 保留为历史示例编号，不再把填补框架空号当作产品目标。
> 同日 revision 2：用户明确“Vue 使用 HTTP；VM/Rust 支持 merged 和 no-merge”，
> 据此将 M8 标为不适用，移除模式语义阻断；其余设计与六任务保留。

# [PLAN-005] Tetris — AutoOS 俄罗斯方块

## 0. 变更摘要

**需要更新原计划。** 保留经典单人玩法，改为前后端分层的标准 AutoUI app，
重点完善棋盘可读性、操作反馈、落点预览、状态切换与窄窗布局。

| 原计划问题 | 本次修订 |
|---|---|
| 仍写 `auto-lang/examples/ui/036-tetris`，迁移后归属未处理 | 按真实 app 规约建议新建兄弟仓 `auto-tetris`，由 auto-os 登记；与 PLAN-013 集成方案在 T1 对齐 |
| 全部逻辑塞 App，没有标准后端，best 只写前端 storage | App / Store / 规则函数 / 页面组件 / `#[api]` / 持久化分层，游戏帧计算留前端 |
| “双端全绿”混淆渲染器、后端实现与部署方式 | 列出 Vue、VM、Rust 及 HTTP / merged 实际矩阵，能力缺口显式保留 |
| `running = "1"` 与 Vue Tick 的 `"true"` 门控不符 | 独立 phase 状态机 + 固定 Tick，避免保留名门控 |
| UI 只有左信息栏、200 格和底部按钮 | 棋盘主导，右侧 next/成绩/升级进度；明确宽窄布局、主题、暂停/结算/保存失败 |
| ghost 可选；“7 bag”又允许独立抽样 | ghost 纳入必验；v1 明确为 LCG 独立抽样、无踢墙 |
| 用“格子总数守恒”代验消行 | 固定棋盘夹具，直接断言 1/2/3/4 行消除后的完整棋盘、分数和等级 |

## 1. 目标

1. 同一套 `.at` 源码提供 Vue、VM/Iced、转译 Rust UI，通过标准
   `use back.api` 与 `#[api]` 连接前后端，部署要求按 §2 矩阵逐项验收。
2. 完整规则：10×20 棋盘、7 种四格块、移动/旋转/软降/硬降、重力、锁定、
   消行计分、等级加速、next、ghost、暂停、重开、出生碰撞结束、best 跨重启保留。
3. 桌面默认窗口及窄窗均能阅读棋盘、操作并识别当前状态；三种渲染使用相同
   信息层级、色号和按钮语义。VM 通过不能代表转译 Rust 通过。
4. AutoOS 桌面可发现并打开应用；保留游戏画廊 `05-games` 收录要求。

**非目标**：多人、云排行榜、账户、Hold、多块 next 队列、音效、完整 SRS、
T-spin/combo/B2B、7-bag、会话续玩、手写 Vue/JS 或 Rust 业务替身、框架大改。
若需要框架修复，单列 auto-lang 前置 Plan，不在 app 中硬编码特例。

## 2. 架构方案

### 2.1 仓库与源代码边界

建议应用根为 `../auto-tetris/`（新仓，尚未创建），遵循 AGENTS §3；本次执行
因目标仓不存在且迁移补丁被自动审查拒绝，先在 `apps/036-tetris/` 完成可发现的
实现，迁移留在 B3。主计划仍在本文件，应用 README 反链。端口暂拟 17400 / 17401，T1 根据
最新 manifest 与实际占用复核，当前不视为已预留。PLAN-013 正在提出
submodule 机制，其草案不等同于当前规约已修改；T1 按生效规约选择登记方式，
不顺带改造组织机制，也不在 auto-lang 复制第二份应用。

以下均为**拟新增**应用路径：

```text
auto-tetris/
├── pac.at
├── README.md
├── src/front/
│   ├── app.at                  # 生命周期、唯一 Tick、键盘路由、布局
│   ├── tetris_store.at         # 状态、命令、派生视图、API 同步状态
│   ├── game_rules.at           # 形状/碰撞/消行/计分/ghost 确定性函数
│   └── pages/
│       ├── game.at             # 棋盘与信息栏组合
│       ├── board.at            # 200 格，active/ghost/locked 区分
│       ├── stats.at            # next、score、best、level、lines
│       └── controls.at         # 屏上操作与状态面板
├── src/back/
│   ├── api.at                  # 共享 DTO + #[api]
│   └── records.at              # 校验、最高分更新、文件持久化
└── tests/
    ├── testdata/               # 固定种子、棋盘、API 数据与预期结果
    ├── rules.at                # 调用真实规则，不复制游戏算法
    ├── smoke.spec.ts           # Vue Playwright
    ├── desktop_mcp.py          # VM / Rust 原生交互
    ├── run_matrix.py           # 拟新增统一模式/夹具/证据入口
    └── package.json            # Playwright 依赖与 test 入口
```

所有模式由 Store 调用同名 API。碰撞、输入、重力、ghost 在前端本地完成，
**每 Tick、每次移动不得发 HTTP 或写磁盘**。后端负责纪录读取与成绩保存。
不能一套 Vue 规则、一套 VM 规则；生成产物不作为手改业务源码。

### 2.2 模式矩阵与已知缺口

命令在应用根执行。merged 指后端业务在原生宿主进程内执行，无需独立业务
HTTP 服务；同端口代理、同时启动两个服务不算 merged，MCP 端口不属于业务服务。

| ID | 前端 / 后端与部署 | 启动入口 | 重点 / 当前证据 |
|---|---|---|---|
| M1 | Vue / Rust，HTTP | `auto run -r vue --server rust --no-merge` | 标准 Web 模式，真实 API 与重启恢复 |
| M2 | Vue / VM，HTTP | `auto run -r vue --server vm --no-merge` | 与 M1 同 API、同持久化 |
| M3 | VM / VM，merged | `auto run -r vm --merged` | 当前直接链接 .at 后端，无独立业务服务 |
| M4 | VM / VM，HTTP | `AUTO_BACKEND_IMPL=vm auto run -r vm --no-merge` | 前端 VM + AutoVM HTTP；`--server vm` 是仅启动后端服务的入口 |
| M5 | VM / Rust，HTTP | `auto run -r vm --server rust --no-merge` | Rust 后端与 M3/M4 结果一致 |
| M6 | Rust / Rust，merged | `auto run -r rust --server rust --merged` | scalar API 已生成 db 吸收委托；仍需原生实机证明真实落盘与重启 |
| M7 | Rust / Rust，HTTP | `auto run -r rust --server rust --no-merge` | 转译 UI 真实验证，不能以 VM 截图替代 |
| M8 | Vue / 后端，merged | 不适用（用户已裁定） | Vue 使用 HTTP；不列交付腿，不以传 --merged 声称支持 |

**2026-09-12 用户裁定原文：“Vue 使用 HTTP；VM/Rust 支持 merged 和 no-merge”。**
因此交付矩阵为 M1–M7，M8 不适用，不需要 Vue 进程内合并方案。
M6 必须实测最高分校验与跨进程重启落盘，不能用前端 storage 或模拟 CRUD 绕过。
完整交付仍受 M6 能力约束，但不阻止 T1 调查及独立的规则/UI 工作。

## 3. 技术栈

- Auto `.at`：widget / store / 普通函数 / `use back.api` / `#[api]`。
- AutoUI：Vue 与原生 Iced，Rust UI 和 HTTP 后端由工具链生成。
- 已验证的 row/col/grid、文本、按钮、语义主题；不引入独立 Canvas 游戏引擎、
  DOM 游戏逻辑或只在 Vue 有效的动画作为功能基础。
- 后端以 .at 实现记录文件；数据根不依赖启动 CWD。文件与 JSON 能力须在
  目标模式探针验证，不能按 notes 注释推定已支持。
- Playwright + AutoUI MCP；实施验证复用 auto-lang
  `.agents/skills/autoui-verifier/` 的驱动与视觉检查方法。

## 4. 需求分析与背景调查

### 4.1 授权与范围

2026-09-12 用户授权分析、更新 PLAN-005，参考 notes/minesweeper，要求标准
AutoUI 及上述模式，重点优化 UI；随后明确 Vue 使用 HTTP、VM/Rust 均支持
merged/no-merge，已纳入 revision 2。前段只修订计划并提供 UI 草图，未指示执行
应用开发、创建远程仓、修改框架、提交或发布；无用户指定预算。随后用户授权实施
更新后的计划，执行在专用 worktree 进行，未修改 auto-lang crates、未创建链接或远程仓。
后续 work 使用专用 worktree，跨仓 env → 兄弟检出 → 主检出，禁止 junction/symlink。
已有其他任务改动不属于本次修订范围。

### 4.2 证据基线（源码调查，未运行验证）

auto-os HEAD：`66ac35597af67335ac56473783113608f5c48479`；
auto-lang HEAD：`3b9eae6711ac196a646d73d8112aaf06de6a2e45`。
工作区含未提交改动，HEAD 不能代表所有当前文件内容。

| 来源 | 结论 |
|---|---|
| 本仓 `AGENTS.md`、`README.md`、`apps.manifest`、`docs/design/01-stage-b-desktop-migration.md` §1/5 | 真实 app 独立仓及 17xxx 端口；minesweeper 是随迁资产，不意味着新产品放回 examples |
| 本仓 `docs/plans/013-desktop-app-portfolio.md` | 有在途集成机制变更，T1 对齐，不覆盖其草案 |
| auto-lang `docs/specs/overview.md`、`docs/specs/auto-lang/ui/overview.md`、`docs/specs/auto-lang/ui/design/app-generation.md` | 生成和 app 能力背景；早期设计的能力缺项与当前实现分开判断 |
| auto-lang `examples/ui/015-notes/{pac.at,src/front/app.at,src/front/notes_store.at,src/back/api.at,src/back/db.at}` | Store、组件和带实现的 #[api] 可参考；README 示例过时，db 虽注释 JSON 持久化，所读实现是内存集合，不能作为落盘证据 |
| 本仓 `apps/038-minesweeper/{pac.at,src/front/app.at,tests/desktop_mcp.py}` | App→Store、grid、确定性原生驱动先例；没有标准后端，不能用它证明全栈矩阵 |
| auto-lang `crates/auto/src/main.rs` Run 与分派 | render/server/merged/no-merge 分属不同维度；有效显式开关是 --merged，不发明 --merge |
| auto-lang `crates/auto-man/src/{vue.rs,rust_ui.rs,pac.rs}` | Vue HTTP；VM merged 链接；Rust generate_merged_api_client 通用分支构造 API_DATA CRUD；pac 有 scene/api/ports/window |
| auto-lang `crates/auto-lang/src/ui_gen/vue.rs` grid 分支与 Aura schema | Vue 生成器用 `cols` 生成 `grid-cols-N`，而 schema 仍登记 `columns`；这是已记录的契约漂移。本 app 使用生成器实际支持的静态 `cols`，框架别名修复另立前置 Plan，不在 app 内改 crates |
| auto-lang `crates/auto-lang/src/ui_gen/vue.rs` Tick 生成段 | interval 编译期取值；有 running 字段时只 watch 字符串 "true"，旧版 "1" 不成立 |
| auto-lang `crates/auto-man/src/vue.rs::gallery_apps_dir` | 画廊默认从一个示例根收割，manifest 登记不等于自动进入画廊 |

框架路径按本仓解析序换算，本次命中 `../auto-lang`。SHA-256 内容锚：
旧 PLAN-005 `1bb6586501cf22113ee8379d1fd0d0ee1029439d1dd0e7d30f50cca0955efedc`；
notes Store `5e0cbd5491837324b15c9516e2306234681c33600cabdf9264b7d8fe473318a3`；
notes API `ac99b3fd3d6a98010d5cb3ef47a2ab74a5e83a7fddee88ee7aa50d510fe65a5c`；
minesweeper App `d524ed46d11df8457738197a502272a5fbe2d5f8a5bf380798414a5684c5f4d4`。

本仓 **docs/specs/ 尚不存在**，但 .autoos/specs.json 已有历史索引，不能误写
“台账为空”。旧 GOAL-010 保留为历史关联，不伪造本仓不存在的 goal Spec。
本计划提出新增 app Spec，review/merge 时沉淀，本轮不预写权威规范。

## 5. 详细设计

### 5.1 状态、规则与计时

- board 是 200 个整数，0 空、1–7 色号，只存 locked；active 保存
  piece/rotation/x/y，ghost 由同一碰撞函数派生，不写 board。
- `ready → playing ↔ paused → over`，暂停原因另存 manual/focus/help。
  启动停 ready，点击“开始游戏”才计时；重开进入新局 playing。
- 7×4×4 个 (dx,dy) 形状；平铺时每旋转 8 个整数，偏移
  `((piece * 4) + rotation) * 8`，替换旧版量纲不清的 piece*16 + rot*4。
- v1 **LCG 独立抽样，不称 7-bag**。T1 选定跨 JS 精确整数与 Rust/VM
  整数范围不溢出的公式，用注入种子钉定 golden；默认种子取时间。
- 顶部居中出生；顺时针旋转，无踢墙，无效操作原状。重力/软降遇阻立即锁定；
  硬降到 ghost 落点后只锁一次，距离为零仍锁定；不额外增加锁定延迟。
- 满行同时删除，余行次序保持，顶部补空。1/2/3/4 行得
  `100/300/500/800 × 消行前 level`；软降成功 +1/格，硬降 +2/格。
  `level = 1 + floor(lines/10)`，下一块使用新等级。
- 落速保持 `max(120, 600 - (level - 1) * 60)` ms。App 唯一固定 Tick
  暂定 **20ms**，Store 按累计时间分频；所有落速可被 20 整除，不用 100ms
  近似导致 540/120ms 失真。App 不声明 running 保留名，Tick 按 phase 门控。
- T1 验证三渲染的首拍、时间源和周期；掉帧至多补一个重力步，暂停/恢复、
  失焦清除积压时间与按键，禁止恢复瞬间连落。仅棋盘变化时重建格视图。
  不以帧数冒充真实毫秒；计时能力不足先记录探针结果。

### 5.2 API、持久化与失败反馈

拟定 DTO 与接口由 T1 验证可表达性，业务都在 records.at：

| API | 语义 |
|---|---|
| list_scores() / GET /api/tetris/record | 返回当前 best 标量 int；缺文件为 0；版本化 JSON 中读取 `best` |
| create_score(score str) / POST /api/tetris/score | 前端以字符串跨过 Rust UI 的 i32/i64 边界，后端解析非负分，落盘 max(old_best, score) 并返回 bool；重复提交幂等 |

records.json 至少含 schema_version、best。当前实现以 app 工作目录为文件根；数据根
优先级与跨模式统一目录仍是框架前置项，不能把 CWD 当产品契约。T1 验证 .at
路径/文件替换与写入串行化，多实例不得用旧值覆盖更高分。

Init 读一次；锁定后若本局最高分超过已确认 best，合并提交；结束或确认重开
时提交尚未保存的新纪录。每时刻最多一个请求，ACK 才显示“已保存”，晚到的小值
不使 best 回退，无新高不写盘。失败仍可玩，提示“纪录未保存 · 重试”，恢复后
重试当前会话最高值。未 ACK 的值不宣称跨崩溃保存，不用前端 storage 建第二
真相源。刷新/重启以后端文件为准；定位为单机成绩记录，不宣称防作弊榜单。

### 5.3 UI：棋盘主导、操作与状态可见

沿 notes 的语义主题与分区，但以棋盘为中心。窗口内不重复系统标题栏；
页头“俄罗斯方块”+状态+暂停。默认深色，支持浅色；外围使用 AutoOS
bg-background/bg-card/text-foreground/text-muted-foreground，
棋盘保持稳定深色游戏面，按钮跟随系统 accent。

```text
  俄罗斯方块                            进行中  [暂停 P]
  ┌──────────────────────┐  ┌────────────────────┐
  │                      │  │ 下一个              │
  │       当前块         │  │   4×4 居中预览       │
  │                      │  ├────────────────────┤
  │      10 × 20         │  │ 得分        2,480   │
  │       主棋盘         │  │ 最高纪录    8,600   │
  │                      │  │ 等级 03   消行 24  │
  │       空心落点       │  │ 升级还需 6 行      │
  │       已锁定堆叠     │  │ 消行 +300          │
  └──────────────────────┘  └────────────────────┘
  [左移] [旋转] [右移] [软降]        [硬降 Space]
  ← → 移动 · ↑ 旋转 · ↓ 软降 · Space 硬降 · P 暂停
```

| 区域 | 约束 |
|---|---|
| 默认内容区 | 约 640×760 逻辑像素，外边距 24、分区间距 16；棋盘 280×560（格 28），信息栏约 184 宽，整体居中 |
| 棋盘 | 严格 1:2，10×20，格间距 1px；网格线 1px 低对比；active 亮实心、locked 稍暗实心、ghost 空心描边；绘制 locked→ghost→active，active 优先；棋盘格使用零内边距、零圆角和 `min-w-0/min-h-0`，避免 Button 默认样式撑出轨道 |
| 七色 | I 青、O 黄、T 紫、S 绿、Z 红、J 蓝、L 橙，三渲染一致；不随 accent 重映射，形状/边界/空心实心辅助识别 |
| 信息 | 得分最大、best 次级；等宽数字防抖，为七位以上留宽；level/lines 并列，升级进度 lines%10，文字“还需 N 行” |
| next | 一个 4×4 预览，按实际外接矩形居中；I/O 不因锚点偏移而贴边 |
| 反馈 | 侧栏固定位置显示最近消行与加分约 600ms；棋盘即时消行，反馈不阻塞规则；不依赖闪烁/粒子/音效 |
| 窄窗 | 内容宽 <560 时 next+得分摘要移到棋盘上方；格 20（200×400），控制两行；360×720 中棋盘/主操作可见，320 宽无横溢出，过矮窗口允许整页纵滚 |
| 实现能力 | 不假定 aspect-square、任意断点或绝对定位在原生有效；T1 验证视口信号与尺寸属性，同一布局状态控制三轨。手动“紧凑布局”可辅助，但不能代验自动窄窗 |
| 可读性 | 正文 14–16、次级 ≥12，点击目标 ≥44×44；硬降有文字，不只用生僻符号；正文对比目标 ≥4.5:1，焦点可见 |

状态切换保持棋盘外框尺寸，不挤动信息栏。覆盖容器须跨端验证；若覆盖不可靠，
三端统一用棋盘区域内等尺寸状态面板。不能 Vue 遮罩、VM 页尾文字；面板阻止
背景游戏输入。面板位置和结构优先于阴影/动画精度。

| 状态 | 棋盘区域与操作 |
|---|---|
| ready | “准备好了吗”、简短键位、开始游戏；显示 next/best，无重力 |
| playing | 完整棋盘，硬降为高频主操作，暂停为次级 |
| paused | 弱化棋盘，“已暂停”；继续游戏/重新开始，board/active/next 保持 |
| help / 失焦 | 游戏暂停；帮助说明本版规则；失焦显示“已暂停，点击继续”，恢复焦点不自动续玩 |
| over | 游戏结束、本局分数、消行/等级、best；新高显示“新纪录”；再来一局，保存状态独立显示 |
| 保存失败 | 信息栏轻提示+重试，不阻断单人游戏，不伪报已保存 |
| 中途重开 | 暂停并确认“重新开始本局？”；继续/重开，取消恢复原状态 |

### 5.4 输入与生命周期

- ←/→ 移动、↑ 顺时针旋转、↓ 软降、Space 硬降、P 暂停/继续；
  屏上按钮分派相同 Store 命令，ready/over 不把任意键当开始。
- 只作用于当前聚焦游戏窗口，帮助/弹窗拦截游戏命令。Vue 阻止方向键/Space
  滚页，避免按钮 Space 默认点击又触发一次硬降。
- 左右/软降按住：首步立即，延迟 160ms，以后每 60ms 重复；旋转/硬降/暂停
  每次按下仅一次。T1 核验 keyup/repeat/blur，不依赖平台 OS 重复速率。
- Tick/按键走串行状态更新；暂停、结束、隐藏、销毁清理按住状态。重复 Init、
  重开、桌面重开不能叠加 Tick。原生输入能力不足须记录阻断，不默默降级。

### 5.5 pac 与集成

拟定 pac：name="tetris"、title="俄罗斯方块"、version="1.0.0"、
scene="ui"、render="vue"、api="rust"、desktop="true"、category="game"、
theme="dark"、accent="indigo"、front_port=17400、back_port=17401。
优先 window="fit" 与稳定首帧；数值起窗须验证三轨，不能假定 VM 专属配置
在 Rust 也有效。blocks 图标先验证闭集，必要时用已有图标配标题。

本仓登记 apps.manifest、README Apps 表、docs/plans/autos-desktop-program.md。
独立运行通过后仍需真实 Vue/VM 桌面打开，验证后端就绪、路由/代理、焦点/暂停。
画廊 05-games 必须可打开；外部源发现不足列框架依赖，不能复制源码回 examples
或创建链接绕过。

### 规范增量

| delta_id | add/modify/retire | docs/specs/... target | before/after rule | rationale | acceptance IDs |
|---|---|---|---|---|---|
| SD-01 | add | docs/specs/apps/tetris.md | 无 → 规则、状态、计时、输入、UI 尺寸/主题 | 固化可复验玩法与视觉标准 | AC-02, AC-03, AC-04, AC-05 |
| SD-02 | add | docs/specs/apps/tetris.md | 无 → DTO、纪录唯一源、幂等保存、模式矩阵与裁定边界 | 同名 API 不等于业务等价 | AC-01, AC-06 |
| SD-03 | add | docs/specs/apps/tetris.md | 无 → 应用仓、端口、桌面/画廊入口、复验方式 | 产品根集成事实与 app README 互链 | AC-07, AC-08 |

同一新 Spec 汇总三项。review 固定最终路径/版本，merge 更新派生 ledger；
当前不把拟定能力写成已实现规范，不改框架 Specs。

## 6. 测试设计

### 6.1 确定性规则夹具

测试入口注入 seed/board/piece/rotation/x/y/score/lines/phase 及手动时钟；
调用真实 .at 规则，不在 Python 复制算法。夹具仅测试启动路径启用，不给
普通用户开放任意棋盘/成绩接口。T1 验证跨端注入方式。

- 7×4 形状每块四个互异格，O 旋转占格不变；左右墙/地板/堆叠碰撞无非法
  重叠，无效移动/旋转保持状态。
- 重力 600/540/120ms 边界；暂停任意时间 board/score 不变，恢复无补落。
- 硬降距离 d：落点等于 ghost，加分 2d，只锁一次；软降成功 +1，遇阻不多加。
- 1/2/3/4 行：底部 n 行各缺同列，以竖 I 填洞；四组明确列出完整预期 board；
  另含非相邻满行与上方标记格，验证压缩次序、顶部补空、消行 n 与计分。
  “空格+占格=200”不能替代这些断言。
- level=1、lines=9 单行消除得 100（不含降落分），变 lines=10、level=2、
  period=540；高等级下限 120；同 seed/输入序列三渲染最终状态一致。
- 出生冲突 over，无越界写；重开 score/lines/level 重置而 best 保留；
  ghost 不贡献锁定格数，暂停/结束无计分副作用。

### 6.2 API、部署与故障

M1–M7 各用隔离目录：best=0 → 提交800 → 提交300 → 重复800 → 完全退出
前后端 → 重启读800；负分拒绝，损坏文件不静默重置，交错请求仍保留最大值。
抓取 HTTP/进程/业务端口证明实际模式；M3/M6 必须执行真实 records.at 校验/
落盘，不能只看 UI 数字。跨模式顺序共用数据根验证可读性。
断后端/只读目录时提示保存失败，游戏继续；恢复后单次重试成功。
完整矩阵指 M1–M7；M8 按用户裁定记不适用，不纳入通过数。

### 6.3 交互与视觉

Vue 用 Playwright，VM/Rust 用原生驱动。Rust MCP 若不可用，T1 确定可行原生
替代驱动，不能以 skip 记通过。按 autoui-verifier 检查几何/字号/颜色/层级，
不要求字体抗锯齿逐像素完全相同。

每渲染截图覆盖 ready/playing（ghost）/paused/over/保存失败；同夹具测试
640×760、360×720、320 宽与矮窗，深浅色可读性。实际按键/点击验证按住、
焦点、空格不双触发、失焦暂停、重开确认、恢复和单 Tick。
截图与状态断言均须通过，不以 DOM 存在代验可见性，也不只截图不测规则。
按模式记录启动命令、revision/commit、快照、截图、数据路径和通过项。

## 7. 验收标准

| ID | 可观察的通过条件 | 验证方法 |
|---|---|---|
| AC-01 | 同一 .at 标准分层，M1–M7 业务等价；Vue HTTP，VM/Rust 均通过 merged/no-merge | 源码、进程/网络、完整矩阵，缺口不计 pass；M8 不适用 |
| AC-02 | 7块/28旋转、碰撞、软硬降、消行/计分/升级/结束符合 §5 | §6.1 真实规则夹具与跨端 golden 零差异 |
| AC-03 | 600/540/120ms 正确，暂停/失焦冻结，恢复无追帧，按住/单次一致 | 手动时钟断言 + 三渲染真实按键/计时 |
| AC-04 | 1:2 棋盘、next 居中、ghost 等于硬降，主题/宽窄布局符合 §5.3 | 同局截图/几何；360×720 主操作可达、320 宽无横溢出 |
| AC-05 | 全状态、帮助、重开确认完整，焦点正确、背景输入不穿透 | 三渲染交互脚本，状态切换棋盘外框不变 |
| AC-06 | best 跨重启/模式保留，幂等且只增不减，失败/重试如实 | M1–M7 持久化、故障、交错写测试，不接受前端 storage 替代 |
| AC-07 | 桌面 Vue/VM 发现、打开并玩一局；画廊05-games可打开；登记一致 | 桌面/画廊真实交互，核对 id/端口/路径，缺能力则 blocked |
| AC-08 | 测试可复跑，Spec delta 与实现/模式裁定一致，不污染框架仓 | T6 证据审计、git diff/status、review 沉淀清单 |

### 7.1 本次交付范围与后置债务

用户在 2026-09-14 确认 M1–M7 七种模式均可运行，基本移动、旋转、软降、硬降、
暂停和消行流程满足本次交付目标，并同意将剩余的扩展验收项移交 DEBTS，不阻塞
本计划 review/merge。后置项登记在仓根 [DEBTS.md](../../DEBTS.md)：

- `DEBT-005-01`：长按/keyup/失焦的原生物理输入矩阵，以及窄窗口、主题和稳定像素夹具。
- `DEBT-005-02`：桌面发现、打开、开局与 `05-games` 画廊真实验收。
- `DEBT-005-03`：merged 原生重启、损坏/只读、并发最大值和跨模式数据根的持久化故障矩阵。

这些债务保留原规范中的目标，不改变已交付功能；后续专项计划应按 `docs/specs/apps/tetris.md`
的规则和 UI 契约补齐证据。

## 8. 执行步骤

保留 T1–T6 ID，按执行切片记录进度。下列 run_matrix.py 及参数是 **T1/T6 要新建并验证
的入口**，不表示已有工具。先以当前 auto --help / auto run --help 确认 CLI，
记录实际二进制路径。工作在专用 worktree，计划簿记留主检出。

- [x] **T1 能力探针与阻断清单**（依赖无；AC-01/03/04/05/06/07/08）
  读取 §4 锚，新建 tests/probes/、run_matrix.py --probe，输出
  tests/evidence/capabilities.md。覆盖 M1–M7 真业务/落盘、20ms Tick/时间源、
  keyup/repeat/blur、视口/覆盖/尺寸/图标、Store/函数、夹具、原生自动化、
  桌面/画廊发现；对齐 PLAN-013 落点与端口。
  验证：各适用模式跑最小 API，写入后退出重启读取，输出 supported/blocked
  与证据。最多一轮最小复现+一轮源码接口核对；缺口形成精确前置清单回填，
  不无限试错。M8 已按用户裁定不适用；M6 失败转框架前置，不直接扩大代码范围。
  已完成 `--probe`/`--all-modes`：源码、CLI、前后端生成物、Playwright 与 Rust
  后端均为 supported；M1–M7 的模式矩阵已由用户完成基本操作验收。原生扩展输入
  和稳定像素夹具转入 `DEBT-005-01`。
- [x] **T2 标准骨架与 UI 基线**（依赖 T1 对应 UI/目录能力；AC-01/04/05）
  新增 §2.1 pac/App/Store/pages/API DTO 骨架，以固定局面实现 ready/playing/
  paused/over、宽窄与深浅色。测试夹具开关不出现在产品页面。
  验证：auto build -r vue、auto build -r rust、M3 启动无编译错误；
  python tests/run_matrix.py --suite visual --fixture midgame，三轨可见截图。
  已实现：`apps/036-tetris` 的 pac/App/Store/API 骨架、棋盘主导布局、next/成绩
  信息栏、状态 overlay、触控按钮和窄窗 flex 换行；Vue 与 Rust 生成构建均通过。
  截图回归后将棋盘网格改为 `cols: 10/4`（Vue 生成 `grid-cols-10/4`），网格占满
  棋盘区域；所有棋盘按钮显式使用 `min-w-0 min-h-0 rounded-none p-0`，并设置 1px
  间距，消除默认 Button 的圆角和内边距重叠。Playwright 已断言 10/4 列及 200 个
  棋盘按钮；M3 实机窗口和视觉夹具转入 `DEBT-005-01`。
  状态提示统一使用 AutoUI 标准 `dialog`/`dialog-content`，不再用
  `absolute inset-0` 模拟遮罩。Vue 生成到标准 Dialog（fixed 视口居中），
  VM/Rust 生成到 `PopoverPlacement::Modal`；这样原生渲染器不会把提示降级成
  普通流式内容而落到窗口角落。Playwright 已加入 `role=dialog` 的视口中心断言。
- [x] **T3 块表与移动族**（依赖 T2；AC-02/04）
  新增 game_rules.at，完善 Store、tests/rules.at、testdata：
  形状/LCG/碰撞/旋转/软硬降/ghost，分离 board 与视图格。
  验证：python tests/run_matrix.py --suite rules，确定性夹具通过；
  真实输入与 ghost 落点一致，测试不重写游戏算法。
  已实现：Store 内 7×4 形状表、LCG、碰撞、ghost、移动/旋转/软降/硬降及
  统一 render cell 字段；Rust 与 VM fixture golden 已覆盖 7×4 形态及 1/2/3/4
  行消除计分/压缩。
  消行后的行压缩已按棋盘自上而下存储修正为“顶部补空行、保留行顺序”，避免
  消除底部满行后上方方块停留在原坐标。
- [x] **T4 完整局面与输入**（依赖 T3；AC-02/03/05）
  完成 Tick、锁定/消行/计分/升级/出生结束、键盘与按钮、按住重复、
  暂停/失焦/重开、状态面板及反馈。
  验证：python tests/run_matrix.py --suite gameplay，§6.1 全 golden、
  生命周期及完整一局通过，无双 Tick/越界写/后台继续落块。
  已实现：20ms Tick 状态机、锁定/消行/计分/升级/出生结束、键盘绑定、暂停、
  帮助、重开确认和反馈文案；Playwright 已验证开始/暂停/恢复、硬降与说明层。
  ready/paused/help/confirm/over 五种状态现在复用标准 AutoUI Dialog，状态切换
  不再依赖原生不稳定的绝对定位；Vue 首屏已验证提示框居中。
  受控 Vue 局面已验证消除 1 行后上方标记格下落 1 行；按住重复和失焦物理矩阵
  转入 `DEBT-005-01`。
- [x] **T5 持久化与部署等价**（依赖 T4，T1 后端前置解决；AC-01/06）
  完成 src/back/{api,records}.at 与 Store 保存状态，各模式执行相同业务。
  验证：python tests/run_matrix.py --suite persistence --all-modes，逐腿
  重启读800、故障/重试/最大值通过；保留进程/网络/文件证据。
  M6 未解决则 blocked，不把 HTTP 结果充当 merged。
  已实现：`#[api]` GET/POST、负分保护、最大值更新入口、保存状态与重试按钮；
  Rust HTTP 后端真实 GET/POST 已通过，`records.json` 现在写入带
  `schema_version`/`best` 的 JSON，并已用 `42 → 低分 10 → 高分 99 → 重启` 回归。
  负分与非法字符串现在明确返回 false，且不会改变已有最高分。
  merged Rust 的 scalar API 已生成 db 吸收委托；损坏/只读/并发及跨模式数据根转入
  `DEBT-005-03`，不影响本次已验收的基本持久化行为。
- [x] **T6 集成、验证与回写**（依赖 T1–T5；AC-01..08；SD-01..03）
  完善测试入口与依赖、应用 README，更新本仓 manifest/README/桌面台账、
  画廊入口；相关框架前置完成后实测。
  验证：python tests/run_matrix.py --all-modes，npm test（应用 tests 目录），
  完成 §6 视觉检查、桌面/画廊真实开局及最终版本证据；准备 Spec delta 供
  独立 review，不提前宣称 reviewed。
  未改 auto-lang crates 时遵守 Category A，**不运行 cargo t/docs_gen**。
  已实现：应用 README、测试包、Playwright 冒烟、desktop MCP 探针、可复跑的
  `run_matrix.py` 与 `tests/evidence/capabilities.md`，以及 `apps.manifest`/本仓
  README 登记；基础实现提交 `dae3951`，验证补丁另有 worktree 提交。
  真实桌面/画廊打开转入 `DEBT-005-02`；本次功能验收、Spec delta 和 review 已完成。

## 9. 复审记录

### 2026-09-12 — 草案修订交接（非实现复审）

- stage: new
- plan: PLAN-005 / revision 2（revision 1 首建契约，revision 2 纳入用户模式裁定；
  保留六任务 ID；M8 从待定转为不适用）
- outcome: blocked（完整交付受 M6 业务能力约束；目前是源码调查，
  尚无运行探针证明修复已完成）
- next: T1 最小能力探针，重点核验 M6；后续 work 获授权后执行。
- changed: T1–T6；建立 AC-01..08、SD-01..03；补分层、矩阵、确定性测试、
  持久化、ghost/状态/宽窄/主题/输入契约。
- 该条记录对应修订阶段：当时只做源码对照、计划结构/覆盖与差异检查，未构建运行
  app；UI 草图表示设计目标，不作为当时的 AutoUI 能力或游戏实现证据。

### 2026-09-12 — execution slice / worktree `plan-005-dev`

- worktree：`D:/autostack/.wt/os-005/auto-os`；实现提交：`dae3951`。
- 交付：新增 `apps/036-tetris`，包含 pac、AutoUI App/Store、Rust `.at` API/DB、
  Playwright 冒烟与 desktop MCP 探针；同步 `apps.manifest` 与本仓 README。
- UI：棋盘主导的 10×20 深色游戏面、active/locked/ghost 三态、4×4 next、得分/
  best/等级/消行/升级进度、ready/paused/help/confirm/over overlay、触控与键盘
  共用消息、窄窗 flex 换行。`accent: indigo` 只作用于品牌/主按钮，方块七色保持
  固定语义，避免主色改变棋盘识别。
- 验证证据：Vue `auto build --render vue --gen-only`、`vue-tsc`、Vite production
  build、Playwright 3/3；Rust UI `auto build --render rust --gen-only` + Cargo
  build；后端 `trans db.at --ai`、`cargo check`，真实 Rust HTTP GET/POST（17401）
  返回并写入 `records.json`。未运行 auto-lang `cargo t/docs_gen`。
- 边界：测试当前用路由 mock API；desktop MCP 未设置 `AUTOUI_MCP_URL` 时只输出
  skip；完整 M1–M7、视觉夹具、按住/失焦与桌面/画廊真实开局仍未验收。

### 2026-09-12 — execution follow-up / persistence and matrix evidence

- worktree：`D:/autostack/.wt/os-005/auto-os`；后续提交 `5f47f8b` 记录版本化记录
  文件、兼容旧裸整数读取，并加入 `tests/run_matrix.py` 与能力证据。
- 验证：`python -B tests/run_matrix.py --all-modes`；Rust HTTP 后端在
  `42 → 低分 10 保持 42 → 高分 99 → 进程重启仍为 99` 后通过，文件内容含
  `schema_version` 与 `best`；`cli.help`、`backend.transpile`、Vue/Rust 生成物、
  Playwright 包与后端可执行文件均 supported。
- 边界：`vm.mcp` 与 native/gameplay/visual 仍因未连接驱动 blocked；这不把 skip
  计入通过，也不宣称 M6 merged 已等价。

### 2026-09-12 — execution follow-up / gameplay and API alignment

- worktree：`D:/autostack/.wt/os-005/auto-os`；提交 `63e1556` 修正重力不计分、
  首屏读取 best、仅新纪录进入保存状态，并让保存成功/失败反馈保持真实。
- API：GET 纪录改为跨渲染器可消费的标量；POST 的 score 在 UI 边界序列化为字符串，
  后端统一解析后复用最大值落盘逻辑。Rust merged 已生成 scalar db 委托并通过
  Cargo 构建，Vue `vue-tsc`/Vite 与 Playwright 3/3 通过。
- 复验：`python -B tests/run_matrix.py --all-modes` 仍报告源代码、转译、生成物、
  Playwright、后端与 monotonic/cross-process persistence supported；`vm.mcp` 和
  native/gameplay/visual 继续明确 blocked。

### 2026-09-12 — execution follow-up / input validation

- worktree：`D:/autostack/.wt/os-005/auto-os`；提交 `3c9e39f` 为 `create_score` 增加
  非负数字解析，负分和非法字符串返回 false，避免无效请求伪报保存成功。
- 复验：Vue 生成、`vue-tsc`/Vite、Rust UI 生成/Cargo、Playwright 3/3 及
  `python -B tests/run_matrix.py --all-modes`；持久化套件新增 `-1`/`oops` 拒绝断言，
  仍通过 42→10→99→重启序列。

### 2026-09-12 — execution follow-up / grid cell geometry

- worktree：`D:/autostack/.wt/os-005/auto-os`；提交 `9783016` 修正截图暴露的棋盘格
  几何；随后提交 `ada3c94` 将盒模型断言固化到冒烟套件。根因分两层：Vue 生成器从
  `cols` 生成 `grid-cols-N`，旧 `columns` 不产生
  列类；列轨道正确后，Button 默认 `px-4/py-2 rounded-md` 又以最小内容宽度撑出
  轨道，初始化空格还保留旧 class。应用现在统一补上 `cols`、`h-full`、1px gap，
  以及 `min-w-0 min-h-0 rounded-none p-0`。
- 证据：浏览器计算样式为棋盘 10 列、预览 4 列；每个棋盘按钮宽度等于轨道且
  `border-radius=0/padding=0`；Playwright 几何（含盒模型）+交互 3/3 通过。最终 Vue
  `vue-tsc`/Vite、Rust UI 生成/Cargo（含 no-merge 启动编译）和
  `python -B tests/run_matrix.py --all-modes` 均通过；原生窗口视觉仍受 MCP 缺失阻断。

### 2026-09-12 — execution follow-up / row compaction

- worktree：`D:/autostack/.wt/os-005/auto-os`；提交 `7ef2880` 修正消行后的棋盘压缩。
  原实现把保留行后的空单元追加到数组尾部；棋盘按自上而下索引时，这会让上方方块
  留在原行。现在先补入 `cleared * 10` 个顶部空单元，再按原顺序追加保留行。
- 受控 Vue 局面：底行预填 6 格、当前横向 I 补齐整行并执行 Tick，结果为
  `lines=1`、`score=100`、原第 18 行清空、标记格由第 18 行移至第 19 行，棋盘
  长度保持 200。Vue Playwright 3/3、Vue/Rust 构建、Rust merged/no-merge 编译及
  `python -B tests/run_matrix.py --all-modes`（含持久化回归）均通过；原生视觉和
  完整 VM/Rust 实机交互仍受 B1 阻断。

### 2026-09-12 — execution follow-up / standard Dialog overlays

- worktree：`D:/autostack/.wt/os-005/auto-os`；提交 `4759a25`。未改变游戏规则，
  只将五个状态层从 `if + col(style: "absolute inset-0 ...")` 改为标准 `dialog`
  根和 `dialog-content` 面板。
- 根因：原生 Iced/VM 的绝对定位降级路径要求额外定位元数据，旧写法在首屏会成为
  普通流式内容并落到左下角。标准 Dialog 由 Vue 生成器映射到 centered Dialog，
  Rust/VM 生成器映射到 Modal Popover，三轨使用同一状态绑定与按钮事件。
- 证据：Vue 完整生成（含 Dialog 组件物化）、`vue-tsc`/Vite、Playwright 3/3
  （含 `role=dialog` 视口中心断言）通过；Rust UI 重新生成并 Cargo 构建通过，
  `rust+rust merged` 原生窗口启动且 MCP 监听独立端口 9248；`vm+vm merged` 原生窗口启动，
  `tests/desktop_mcp.py` 返回 `Tetris VM MCP snapshot OK`。原生像素截图和完整
  M1–M7 交互矩阵仍留给 B1 驱动，不把结构快照当作像素验收。

### 2026-09-12 — execution follow-up / Rust keyboard dispatch

- worktree：`D:/autostack/.wt/os-005/auto-os`；应用预览修复提交 `d6d31d0`；
  AutoUI 框架按键桥接提交位于隔离工作树
  `D:/autostack/.wt/lang-tetris`（`4a5680cb1`、`80c8ea32f`、`873e6c72d`、
  `5dda39f6b`）。主检出 `D:/autostack/auto-lang` 保留其他任务的未提交改动，
  本次只叠加相同的四个按键桥接文件修改，未重置或覆盖它们。
- 根因：Rust 端生成了 `bind` 声明，却没有把 `key_bindings` 发布到共享 MCP
  状态，且原生 MCP 事件只停在字符串事件层；所以工具返回“Key sent”而
  `store.px` 不变。修复后，`Component` 生成 `key_bindings`/`key_message`，
  Iced 使用 `keyboard::listen()` 接收未被输入控件消费的物理键，MCP 同时发布
  映射并将 handler 名称回解析为同一 typed message。输入法组合事件被 Iced
  过滤，方向键、Space、P 不依赖 IME。
- 证据：Rust UI 重新编译并以 `AUTOUI_MCP_PORT=9250` 启动；首屏状态为
  `phase=ready`，点击“开始游戏”后 `phase=playing`，调用
  `autoui_keyboard ArrowLeft` 后 `store.px: 3 → 2`，随后 ArrowRight 恢复为
  3；Space 使方块落到底部，P 使 `phase: playing → paused`。生成的 Rust
  源码包含 ArrowLeft/Right/Up/Down、Space、P/p 的绑定与消息映射。Vue 构建与
  Playwright 3/3（含键盘硬降、Dialog 居中、10/4 列与零圆角盒模型）继续通过。
- 限制：当前环境没有可供 Computer Use 直接注入物理键的桌面窗口，因此物理
  按键已完成源码/生成物路径核验，Rust MCP 已完成实际状态变化核验；B1 仍保留
  原生窗口截图和完整按键重复/失焦矩阵。

### 2026-09-12 — execution follow-up / triggerless VM modal centering

- 根因：`dialog` 没有触发器时，标准转换会生成
  `PopoverAnchor::Widget(View::Empty)`；Iced 的零尺寸 flex 子项不会注册 overlay，
  面板因此从模态层退回普通布局并落到窗口角落。
- 修复：Modal Popover 对 Empty anchor 保留 1×1 的隐形基座，并由 generic/dynamic
  两条原生渲染路径传递 `anchor_is_empty`；有真实触发器的普通 Popover 保持原有锚点
  尺寸与定位。应用五个 `dialog` 声明无需增加绝对定位或应用专用 workaround。
- 证据：`cargo test -p auto-lang --features iced-layout-tests popover_modal_ -- --nocapture`
  通过，覆盖 Empty anchor 居中、常规居中、遮罩外点、Esc 关闭和面板点击；新构建的
  VM merged 在 9255 端口启动，`tests/desktop_mcp.py` 返回 `Tetris VM MCP snapshot OK`，
  首屏“开始游戏”动作使 `phase: ready → playing`。截图接口在当前沙箱会产生窄长窗口，
  仅作为启动证据，不替代原生窗口像素验收。

### 2026-09-13 — review / revision 2

- stage: review
- plan_id: PLAN-005
- plan_revision: 2
- outcome: blocked
- reviewed_commit: `d6d31d022a2b6dd420a94efc4a644478be57daef`
- base_commit: `6fb69564d81591c8666b1485de2aaaa97342d77a`
- dependency_revisions: `auto-lang` main `012b30832c59cc9bc34b25fa017b3276535a60ab`（工作区含未提交改动）；
  Rust 键盘隔离工作树 `5dda39f6b8362ba0165f2372d1d4f885623ff506`（Popover/renderer/layout_tests 仍有未提交改动）
- spec_inputs: Plan SHA-256 `369EA9834D8607E738F5C34098E0A8221B821ECB4FE4A0563F5DF5E69C50A6CD`；
  `.autoos/specs.json` SHA-256 `F7B87F8E1B313FA8B264E53AED024C709345EC4A987A42F53BBB43B1BF97BD85`；
  `docs/specs/apps/tetris.md` 缺失；应用能力证据
  `apps/036-tetris/tests/evidence/capabilities.md`（worktree hash
  `E9FE9B510033B4D1DDACB910BF6135D85FFA4252B48D2C06AC768CCA57BF52DF`）
- acceptance_results:
  - AC-01: partial — 应用源代码、Vue/Rust 生成物及若干构建通过；M1–M7 的真实进程/网络/业务等价矩阵未完成，M6 merged 未证明。
  - AC-02: partial — 已有实现和 Vue 受控消行回归；`rules.at`、完整 1/2/3/4 行 golden 与三轨零差异尚未建立。
  - AC-03: partial — 20ms Tick、状态机和绑定已实现；按住重复、keyup/失焦、三轨真实计时及原生输入仍未验收。
  - AC-04: partial — Vue 棋盘几何、Dialog 居中和 Iced 五项 Popover 布局测试通过；VM/Rust 原生窗口截图、窄窗和深浅主题夹具缺失。
  - AC-05: partial — Vue 交互与 VM MCP 首屏动作有证据；完整 VM/Rust merged/no-merge 状态覆盖、焦点和背景输入拦截未证明。
  - AC-06: partial — Rust HTTP 的低分保护、版本化 JSON 和重启读取有证据；M6 原生 merged、损坏/只读/并发及跨模式数据根未证明。
  - AC-07: blocked — manifest 登记已存在，但桌面/05-games 画廊真实打开和开局未验收。
  - AC-08: blocked — canonical `docs/specs/apps/tetris.md` 尚不存在；应用 worktree 有未跟踪运行产物，依赖 worktree 有未提交实现改动。
- findings:
  - `R-001` blocker（AC-01/03/04/05/06/07；T1/T5/T6，B1/B2）：计划自己仍记录完整 VM/Rust 矩阵、原生视觉、失焦/重复键和 M6 持久化为 blocked。当前唯一新增原生证据是 VM merged MCP 结构冒烟和 Iced 布局单测，不能替代这些用户可观察验收。修复：接入可复跑的原生驱动，逐项运行 M1–M7，保存进程/网络/窗口/文件证据后复审。
  - `R-002` blocker（AC-08）：reviewed behavior 依赖 auto-lang 的 Popover/renderer/layout 浮动改动，当前既未落在依赖主线提交，也未形成可绑定的依赖 SHA；应用 worktree 还含未跟踪截图和嵌套构建产物。修复：在依赖仓单独完成 review/landing，清理或隔离运行产物，重新绑定应用与依赖提交并复跑回归。
  - `R-003` blocker（SD-01..03/AC-08）：提议的 `docs/specs/apps/tetris.md` 不存在，不能在 merge 阶段验证 frozen Spec delta。修复：在专用 worktree 准备三项规范增量，按当前实现和已批准裁定复审后再合并。
  - `R-004` needs_fix（AC-08）：本次复跑 `python -B tests/run_matrix.py --all-modes`/`--probe` 因 `tests/evidence/capabilities.md` 写入被拒而失败；`pnpm exec playwright test --list` 在当前工作树未找到可执行的 Playwright。修复：解除文件/运行时锁定并确认测试依赖入口，再提交新的可复跑证据。
- evidence:
  - 应用分支 `plan-005-dev` 的实现提交链为 `dae3951`、`5f47f8b`、`63e1556`、`3c9e39f`、`9783016`、`ada3c94`、`7ef2880`、`4759a25`、`d6d31d0`。
  - `cargo test -p auto-lang --features iced-layout-tests popover_modal_ -- --nocapture` 已通过 5 项；该结果仅覆盖 Iced Popover 几何/关闭语义。
  - 已提交的 `capabilities.md` 明确 `vm.mcp: blocked`；当前复跑还暴露证据文件写入阻断。
  - 应用 worktree：`D:/autostack/.wt/os-005/auto-os`；依赖 worktree：`D:/autostack/.wt/lang-tetris`。
- review_record_persistence: 复审记录已写入主检出 `docs/plans/005-tetris.md`；尝试仅提交该计划时，Git 因无法创建
  `D:/autostack/auto-os/.git/index.lock`（Permission denied）失败，未触碰其他工作区改动。
- next: 保持 Plan `executing`，先处理 `R-001` 至 `R-004`，完成依赖落地、Spec 准备和 M1–M7 原生/持久化证据，再重新运行 `/auto-plan:review`；本次不进入 `/auto-plan:merge`。

### 2026-09-13 — execution follow-up / completeness acceptance

- 本轮证据以应用提交 `d6d31d022a2b6dd420a94efc4a644478be57daef` 为基线，
  运行产物放在隔离验收副本 `D:/autostack/auto-os/tetris-vm-check`，未回写应用
  或框架源码；依赖仍以 `D:/autostack/.wt/lang-tetris` 的
  `5dda39f6b8362ba0165f2372d1d4f885623ff506` 为参考，工作区仍有未提交改动。
- Vue HTTP（M1）：Rust HTTP 后端 `17411` 与 Vite `17410` 联调，Playwright
  `3 passed (3.8s)`；`python -B tests/run_matrix.py --suite persistence` 在
  隔离副本通过单调更新及跨进程重启读取。
- VM：`auto run -r vm --server vm --no-merge` 已启动真实 HTTP API `17401`，
  `GET /api/tetris/record` 返回 200；merged 日志确认后端进程内运行。两条 VM
  原生腿均无法取得 MCP/窗口交互证据，原因是本机回环监听返回 WinError 10013，
  Computer Use 也没有可控制的原生桌面窗口。
- Rust merged（M6）：已用 Iced MCP 实测 ready→playing、方向键、旋转、空格硬降、
  P 暂停、玩法说明/返回；重试保存写入 `records.json`（`schema_version=1`）并在
  重启后读回最高分。该结果证明状态与落盘路径可工作，但仍不替代原生像素/失焦矩阵。
- Rust no-merge（M7）：隔离 `CARGO_TARGET_DIR` 的生成前端 Cargo build 已通过；
  手工启动生成前端 + Rust HTTP `17401` 后，MCP 实测开始、方向键、旋转、空格硬降、
  P、帮助层及重启读分数均通过。标准 `auto run -r rust --server rust --no-merge`
  仍在工具链强制使用 `D:/autostack/auto-lang/target` 时因 `os error 5` 失败，
  因此官方命令腿仍不能记为 pass。
- 本轮还发现生成 Rust no-merge API 客户端的 `create_score` 只返回本地 JSON，
  没有把后端 bool 结果传回 Store；后端实际已写入分数，但界面会保留
  “纪录未保存 · 重试”。这是 M7/AC-06 的生成器契约缺口，需在框架前置修复或
  明确降级前重新复验，不能用手工后端写入替代保存反馈验收。
- 本轮结论：AC-01、AC-04、AC-05、AC-06、AC-07、AC-08 仍为 partial/blocked；
  Vue、Rust merged 和手工 Rust no-merge 的可观察状态证据已增加，但完整 M1–M7
  官方命令矩阵、原生窗口截图、按住/失焦、损坏/只读/并发持久化、桌面/画廊开局、
  canonical `docs/specs/apps/tetris.md`、依赖落地与干净工作树尚未完成。
- 新增执行阻断：`R-005`（VM/native MCP 回环权限与无原生窗口）；`R-006`（Rust
  no-merge 官方 CLI target 权限及生成客户端保存返回值契约）。Plan 保持
  `executing`，先处理 `R-001`–`R-006` 后再重新运行 `/auto-plan:review`，本轮不进入
  `/auto-plan:merge`。

### 2026-09-13 — execution follow-up / Rust no-merge save return fix

- 依赖工作树：`D:/autostack/.wt/lang-tetris`；提交 `ddb5cda98` 修复 Rust UI
  生成器的布尔 HTTP API 返回值。`bool` 端点现在生成同步请求并解析
  `json::<bool>()`，不再返回本地占位 JSON；`auto-man` 定向单测通过（1 passed，
  292 filtered）。依赖工作树中 Popover/renderer/layout 浮动改动仍未合并，故该
  提交只作为可绑定的前置修复证据，不改变本 Plan 的依赖落地阻断。
- 官方 no-merge 命令已使用修复后的 CLI 重新生成源码；生成物确认
  `create_score(score: String) -> bool` 且同步读取后端布尔响应。官方命令仍因工具链
  强制写入 `D:/autostack/auto-lang/target` 而触发 Windows `os error 5`，R-006 的
  target 权限部分保持阻断。
- 在隔离验收副本 `D:/autostack/auto-os/tetris-vm-check` 使用独立 Cargo target
  编译生成前端/后端并启动 Rust no-merge：MCP 实测 ready→playing、ArrowLeft、
  硬降、保存和重启读取；将记录置为 0 后硬降得到 36 分，点击“重试保存”使
  `save_pending: true → false`、`save_label: 新纪录待保存 → 已保存`、
  `best_label: 0 → 36`，HTTP 记录为 36，重启后仍读回 36。该证据关闭生成客户端
  保存返回值这一子缺口，但不能替代官方命令、原生像素、按住/失焦及 VM 回环权限
  证据。
- 本轮不改变应用 worktree；Plan 仍为 `executing`。R-005（VM/native MCP 与
  原生窗口不可用）及 R-001–R-004、R-006 的 CLI target 权限和其余完整性验收项
  继续交由后续复审处理，本轮不进入 `/auto-plan:review` 或 `/auto-plan:merge`。

### 2026-09-13 — execution follow-up / dependency and Spec cleanup

- 依赖工作树 `D:/autostack/.wt/lang-tetris` 已将三个 Iced Modal 文件提交为
  `6cefb21cf`，与布尔 API 生成器提交 `ddb5cda98` 分开可审；两项均保留独立
  变更边界，工作树已干净。`popover_modal_` 回归为 5 passed，覆盖无触发器居中、
  普通居中、遮罩外点、Esc 关闭和面板点击。
- 应用 worktree `D:/autostack/.wt/os-005/auto-os` 已清理截图临时目录和误生成的
  嵌套 `apps/autostack` target，当前工作树干净；新增 canonical Spec
  `docs/specs/apps/tetris.md`，提交 `31caca2`，覆盖 SD-01（规则/状态/输入/UI）、
  SD-02（DTO/纪录/幂等保存/模式矩阵）和 SD-03（应用路径、端口、桌面/画廊入口、
  复验门）。该 Spec 是产品合同，实际通过项仍以本节证据为准。
- R-002 的“依赖未提交/应用产物污染”与 R-003 的“Spec 缺失”已具备可绑定的
  修复提交；它们仍需独立 `/auto-plan:review` 绑定依赖 SHA 后才能进入 merge，
  不在本轮直接合并。R-001/R-005 原生驱动与回环权限、R-004 可复跑入口、R-006
  官方 CLI target 权限以及 M1–M7 完整性矩阵继续阻断，Plan 保持 `executing`。

### 2026-09-13 — execution follow-up / clean matrix rerun

- 在干净应用 worktree `D:/autostack/.wt/os-005/auto-os/apps/036-tetris` 重跑
  `python -B tests/run_matrix.py --all-modes`，能力证据文件可正常写入；源码、
  转译、Vue/Rust 生成、Playwright 包、Rust 后端和持久化均为 supported。
  `pnpm exec playwright test --list` 正常列出 3 个冒烟用例。之前 R-004 的证据
  写入/测试入口问题不再复现。
- 同一轮仍明确报告 `vm.mcp: blocked`（未配置 `AUTOUI_MCP_URL`）及
  `native/gameplay/visual: blocked`（缺少可连接的 VM MCP/浏览器），因此不把
  R-001/R-005 的原生交互、视觉、失焦/重复键和 M1–M7 完整矩阵误记为通过。
  R-004 可标记为 needs_fix 已处理；Plan 继续保持 `executing`，等待原生驱动和
  依赖独立复审后再进入 `/auto-plan:review`。

### 2026-09-14 — execution follow-up / official Rust no-merge recovery

- 依赖工作树 `D:/autostack/.wt/lang-tetris` 新增提交 `6b228524e`：Rust UI
  运行器的共享 Cargo target 现在尊重调用方显式设置的 `CARGO_TARGET_DIR`，不再
  强制覆盖到不可写的 `auto-lang/target`。与 `ddb5cda98`、`6cefb21cf` 一样，
  该提交保持独立依赖边界，工作树已干净。
- 使用该修复版 CLI 执行官方
  `auto run -r rust --server rust --no-merge`，隔离 target 下前端/后端均成功
  编译并启动（业务 HTTP `17401`、Rust UI MCP `9254`）。实测 `ready → playing`、
  `ArrowLeft`（`px: 3 → 2`）、两次硬降，以及新纪录保存（分数 `66`：
  `save_pending: true → false`、`新纪录待保存 → 已保存`、后端记录 `66`）均通过。
  这同时关闭了 R-006 的 CLI target 权限和生成客户端 bool 返回值两个子阻断。
- 官方命令腿现可复跑，但 M1–M7 的完整业务矩阵、原生窗口像素/物理按键、按住与
  失焦、故障/并发持久化和桌面/画廊开局仍受 R-001/R-005 的驱动与环境条件限制。
  Plan 继续保持 `executing`；依赖提交和 Spec 需独立复审绑定后，才可进入
  `/auto-plan:merge`。

### 2026-09-14 — review / revision 2（复审重建）

- stage: review
- plan_id: PLAN-005
- plan_revision: 2
- outcome: blocked
- reviewed_commit: `31caca2caea61586df7bd363f62151ded6eddfb5`
- base_commit: `6fb69564d81591c8666b1485de2aaaa97342d77a`
- dependency_revisions: `auto-lang` 隔离工作树
  `D:/autostack/.wt/lang-tetris`=`6b228524ed6eddccfc8f5e7017f33916f4d5f180`，
  包含 `ddb5cda98`（Rust bool HTTP 返回值）、`6cefb21cf`（无触发器 Modal
  overlay）和 `6b228524e`（尊重显式 `CARGO_TARGET_DIR`）；依赖工作树干净。
- spec_inputs: `docs/specs/apps/tetris.md` 已在 reviewed commit 中存在，SHA-256
  `596B7E0884F53B56C2834E22F7E2CA14B98CE6D06BDA614AAF0B27A09432AB70`；本次仅
  复核既有 Spec，不在复审阶段发布新的规范内容。
- independence: 本次复审在同一执行会话内完成，非独立审查会话；结论由当前
  worktree、提交、生成物和测试工件重建。
- acceptance_results:
  - AC-01: partial — Vue HTTP、Rust merged/no-merge 官方命令和核心状态动作均有
    实测；`run_matrix.py --all-modes` 的源代码/转译/生成/Cargo/持久化能力均为
    supported，但完整 M1–M7 真实进程、VM 两腿和三轨业务等价矩阵仍不完整。
  - AC-02: partial — 10×20 棋盘、ghost、消行上方方块下落及受控 Vue 回归已验证；
    7×4 形态与 1/2/3/4 行 golden 及三轨零差异仍未建立。
  - AC-03: partial — 20ms Tick、键盘映射和 Rust MCP 单键动作通过；按住重复、
    keyup/失焦清理、三轨真实计时和物理键仍没有可复跑证据。
  - AC-04: partial — Vue 棋盘 10/4 列、零圆角盒模型和 Dialog 居中，及 Iced
    Modal 五项布局回归通过；VM/Rust 原生窗口像素、窄窗/深浅主题夹具仍缺失。
  - AC-05: partial — Vue Playwright 冒烟实际 `3 passed (3.2s)`，Rust no-merge
    MCP 已验证开始、方向键、硬降和暂停；VM/native MCP、焦点和背景输入拦截
    的完整覆盖仍缺失。
  - AC-06: partial — HTTP 与 Rust no-merge 已验证低分保护、版本化记录、保存 ACK
    和跨进程重启读取；merged 原生重启、损坏/只读、并发最大值及跨模式数据根
    尚未完成。
  - AC-07: blocked — manifest 条目存在，但桌面和 `05-games` 画廊的真实发现、
    打开与开局仍没有可控环境证据。
  - AC-08: partial — Spec、依赖提交、干净 app/dependency worktree、能力矩阵和
    Playwright 可复跑入口已具备；完整 M1–M7 与桌面/原生视觉门仍未闭合。
- findings:
  - `R-001` blocker（AC-01/02/03/04/05/06/07）：完整 VM/Rust merged/no-merge
    实机矩阵、规则 golden、按住/失焦、原生截图和桌面/画廊门仍未完成。现有 MCP
    状态冒烟与 Iced 几何单测不能替代用户可观察的全量验收。下一步需要可复跑的
    原生驱动，逐项保存进程、业务端口、窗口、输入和数据文件证据。
  - `R-005` blocker（AC-01/03/04/05/07）：本机 VM/native MCP 回环仍受
    WinError 10013/无可控制原生窗口限制，因而无法完成 VM 两腿物理交互和像素
    验收。需在具备 loopback 权限和原生窗口驱动的环境复验，不能将 HTTP 或结构
    快照升级为原生通过。
  - `R-002` needs_fix（AC-08）：Popover/renderer、bool API 和 target 目录修复已
    形成独立依赖提交并通过定向回归，但尚未进入依赖主线并绑定到本仓可合并提交；
    merge 前必须完成依赖侧独立 review/landing 与应用最终 SHA 复跑。
  - `R-003` resolved-for-review（AC-08）：canonical Spec 已在 reviewed commit
    中落地并覆盖 SD-01..03；进入 merge 时仍须把该文件随应用提交沉淀到主线。
  - `R-004` resolved（AC-08）：干净 worktree 的 `run_matrix.py --all-modes` 和
    Playwright 列表均可复跑，实际 Vue Playwright 三用例已通过；本次测试生成物
    已清理。
  - `R-006` resolved（AC-01/06）：显式 Cargo target 修复后，官方
    `auto run -r rust --server rust --no-merge` 已在隔离 target 成功编译/启动，
    并完成方向键、硬降和分数保存 ACK 验证。
- evidence: app worktree `D:/autostack/.wt/os-005/auto-os`（clean，reviewed
  commit `31caca2`）；dependency worktree `D:/autostack/.wt/lang-tetris`
  （clean，HEAD `6b228524e`）；`python -B tests/run_matrix.py --all-modes`、
  `.\\node_modules\\.bin\\playwright.cmd test --reporter=line`（tests 目录，
  `3 passed (3.2s)`）、`popover_modal_`（5 passed）及官方 Rust no-merge MCP
  记录均已在执行记录中留存。
- next: Plan 保持 `executing`，先解除 `R-001`/`R-005`，完成依赖独立 landing 和
  M1–M7/桌面/视觉/持久化全量复验；复审未通过，不进入 `/auto-plan:merge`。

### 2026-09-14 — execution follow-up / auto-lang dependency landing

- `R-002` 已完成：从当前 auto-lang 主线新建独立 landing worktree，干净应用
  `ddb5cda98` 与 `6b228524e`，并复跑 auto-man 定向测试 `1 passed`；Iced 主线
  现有 Modal 实现（`ee2fafa76`）连同 Tetris 所需无触发器回归复跑为 `5 passed`。
  两个生成器修复已落到 auto-lang 主线提交 `697718962`（bool HTTP ACK）和
  `af28b48fa`（尊重显式 `CARGO_TARGET_DIR`）。
- 以 auto-lang 主线为 `AUTO_LANG_ROOT` 在 app worktree 重跑
  `python -B tests/run_matrix.py --all-modes`：source/transpile/Vue/Rust/Cargo/
  persistence 均为 `supported`，仅 `vm.mcp` 与 native/gameplay/visual 保持环境
  阻断；Playwright 冒烟实际 `3 passed (3.1s)`。测试生成物已清理，app worktree
  保持干净。
- `R-002` 状态改为 resolved；`R-001` 与 `R-005` 仍是完整性验收的阻断项。主线
  其它计划在本轮并行推进，依赖最终提交以当前 auto-lang `master` 的包含关系为准。

### 2026-09-14 — execution follow-up / final cross-renderer matrix

- 应用最终修复已提交 `989c0be`：记录路径通过 `AUTO_PROJECT_DIR` 与
  `AUTO_RENDER` 在 Vue、VM、Rust 间保持一致；VM 前端在 `src/front` 运行时使用
  `../../records.json`，Rust/Vue 使用应用根路径。成绩解析拒绝负数和非法输入，
  Rust/VM/Vue 共用版本化 `records.json`；RetrySave 直接消费 typed `bool` 回执。
- auto-lang 主线已提交 `183e358cc`，在既有 `697718962`（bool HTTP 回执）、
  `af28b48fa`（显式 Cargo target）等修复之上，补齐 `AUTO_RENDER`/非扩展路径注入，
  并使 `a2r_std` 生成前缀幂等；Rust no-merge 官方入口可从源码重新生成并编译。
- 最终可复跑证据：Vue HTTP Playwright `3 passed (2.3s)`；VM merged 与 no-merge
  原生 MCP 均完成 ready→playing、左移、硬降，no-merge 完成重试保存并读回
  `records.json`；Rust merged 与 no-merge 原生 MCP 均完成开始、左移、硬降、重试
  保存，生成后端可编译；持久化矩阵通过低分保护、负分/非法分数拒绝、版本化写入
  与跨进程重启读取。VM no-merge 原生键盘实测 `ArrowLeft`（`px: 3→2`）、
  `ArrowRight`（恢复 3）和 `P`（`playing→paused`）。
- `AUTOUI_MCP_PORT` 高端口回环现可用；VM 初始原生截图已取得，`准备好了吗`
  Dialog 位于窗口中央。截图接口仍受当前窗口捕获尺寸影响，无法替代窄窗、深浅
  主题和逐像素 golden。M4 的实际前端+HTTP 入口是上表命令；`--server vm` 仅用于
  启动 AutoVM 后端服务，避免后续复验把服务模式误当完整 UI 模式。
- 阻断收窄但不关闭：`R-001` 仍缺 7×4 形态及 1/2/3/4 行规则 golden、三轨零差异
  全量矩阵、按住/keyup/失焦和物理输入；`R-005` 仍缺可控物理窗口驱动、桌面与
  `05-games` 画廊真实发现/开局及稳定像素夹具。MCP 结构/状态证据不升级这些门。
  `R-002` 已 resolved，依赖主线与应用提交均有可绑定 SHA。
- Plan 继续保持 `executing`；只有在具备上述原生/桌面验收能力并完成独立复审后，
  才能转 `reviewed` 并进入 `/auto-plan:merge`。

### 2026-09-14 — review / revision 2（fit 自动尺寸复审）

- stage: review
- plan_id: PLAN-005
- plan_revision: 2
- outcome: blocked
- reviewed_commit: `66da4c78c46e1df0d6747ded0999f825ab724551`
- base_commit: `6fb69564d81591c8666b1485de2aaaa97342d77a`
- dependency_revisions: `auto-lang` master `34f86fa424e7c6f12d3b7e1fe7d62a301d12cf44`
  （fit 测量容器改用隐藏 scrollbar；`cargo build -p auto` 通过）。主检出另有
  其他计划的既有未提交文件，本次只提交并验证 renderer 单文件改动。
- spec_inputs: Plan SHA-256 `CF6EA8A86932A29D27EB9772E65B6F319B692E5512D39689F3927E3F8E841B64`；
  `docs/specs/apps/tetris.md`（应用 worktree）SHA-256
  `596B7E0884F53B56C2834E22F7E2CA14B98CE6D06BDA614AAF0B27A09432AB70`；
  `.autoos/specs.json` SHA-256
  `CDE7340E2D51703BBEB85FA6DFFFBA9D893F706FB69A8E6C73416B4EB6F2818D`。
- independence: 本次复审在同一执行会话内完成，非独立审查会话；结论由当前
  应用 worktree、依赖提交、生成物和可复跑命令结果重建。
- acceptance_results:
  - AC-01: partial — `run_matrix.py --all-modes` 的源码、转译、生成、Cargo、
    后端和持久化能力为 supported；VM MCP/native/gameplay/visual 仍有 blocked，
    M1–M7 完整真实进程、网络和业务等价矩阵未闭合。
  - AC-02: partial — `rules_golden` 为 2 passed，覆盖 opening/lock 与 1/2/3/4
    行计分、压缩；7×4 形态/28 旋转全量 golden 与 VM 夹具注入仍缺失。
  - AC-03: partial — 20ms Tick、键盘单键和暂停动作有证据；按住重复、keyup、
    失焦清理、三轨真实计时和物理输入仍没有可复跑证据。
  - AC-04: partial — 新运行时 `Scrollbar::hidden()` 修复后，VM merged 首屏截图
    观察到自动收缩、中央 Dialog 且无 fit 测量滚动条；Iced Modal 定向测试 5 passed。
    360×720/320 宽、深浅主题和稳定逐像素夹具仍未完成。
  - AC-05: partial — Rust/VM MCP 已有 ready→playing、方向键、硬降和暂停状态证据；
    Playwright 入口可列出 3 个用例，但当前环境实际运行因 Chromium 可执行文件缺失，
    完整三轨状态、焦点和背景输入拦截仍未证明。
  - AC-06: partial — HTTP 与 no-merge 持久化低分保护、版本化写入、保存 ACK 和
    跨进程重启读取通过；merged 重启、损坏/只读、并发最大值和跨模式数据根仍缺证据。
  - AC-07: blocked — `gallery_contract.py` 仍报告 `05-games` 画廊缺失且产品 app
    不在当前 gallery scan root；桌面发现、打开和开局没有真实可控证据。
  - AC-08: partial — 应用 worktree 已恢复干净，fit 运行时提交可绑定，能力矩阵与
    Spec 输入可复核；当前环境的浏览器二进制缺失以及完整原生/桌面证据仍未闭合。
- findings:
  - `R-001` blocker（AC-01/02/03/04/05/06/07）：规则全量 golden、三轨 M1–M7
    完整矩阵、按住/keyup/失焦、物理输入、窄窗/主题夹具仍未完成。`rules_golden`
    与 MCP 状态冒烟只能证明已有子集，不能升级为完整通过。
  - `R-005` blocker（AC-01/03/04/05/07）：高端口 VM MCP 当前可连接且 fit 截图已验证，
    但仍没有可接管的真实顶层窗口/物理输入驱动，也没有桌面与 `05-games` 画廊真实
    发现/开局和稳定像素验收；不能以 MCP 结构快照替代这些门。
- evidence:
  - `python -B tests/run_matrix.py --all-modes`：持久化 supported、Rust rules golden
    supported；VM rules fixture 与 native/gameplay/visual 明确 blocked。
  - `node_modules/.bin/playwright.cmd test --list`：列出 3 个用例；实际运行因缺少
    Chromium 可执行文件未启动，既有 `3 passed` 记录仍保留为历史证据。
  - `cargo test -p auto-lang --features iced-layout-tests popover_modal_ -- --nocapture`：
    5 passed；VM merged MCP `autoui_snapshot` 返回 `Tetris VM MCP snapshot OK`，
    `autoui_screenshot` 首屏观察到中央 Dialog、自动收缩且无可见 fit 滚动条。
  - 应用 worktree `D:/autostack/.wt/os-005/auto-os`：clean；依赖 renderer 修复提交
    `34f86fa424e7c6f12d3b7e1fe7d62a301d12cf44` 已提交。
- next: Plan 保持 `executing`；先解除 `R-001`/`R-005`，完成完整规则与模式矩阵、
  原生物理/像素和桌面画廊验收后，再重新复审；本次不进入 `/auto-plan:merge`。

### 2026-09-14 — execution follow-up / native VM focus and input gate

- stage: work
- plan_id: PLAN-005
- plan_revision: 2
- outcome: partial
- code_commits: app `df4ee1f`/`7091fa6`, evidence refresh `44f6933`/`20bef67`; auto-lang prerequisite
  `86dae6ba4` (`fix(ui): route opt-in app blur lifecycle events`)
- task_ids: T1, T5, T6
- evidence:
  - `cargo check -p auto-lang` and full `cargo build -p auto` pass on the
    dependency revision.
  - A real VM merged window on MCP `9273` passed
    `python -B tests/native_focus_probe.py --mcp-url http://127.0.0.1:9273/mcp`:
    six physical key-down packets during a 750 ms hold, explicit key-up,
    `px=3→0`, and focus transfer to another visible window resulting in
    `phase="paused"`. Focus restoration did not resume play.
  - The refreshed native sequence on MCP `9274` starts from the ready Dialog,
    sends `ArrowLeft`, `ArrowRight`, `ArrowDown`, `Space`, and `P` with
    key-down/key-up pairs, and records the focus transition in
    `tests/evidence/native-input-live.json`.
  - `tests/evidence/vm-ready-modal-live.png` shows the ready Dialog centered
    over the dimmed board; `tests/evidence/capabilities.md` binds the live MCP
    endpoint and native driver.
  - Rust `rules_golden` now passes 3 tests, including all seven pieces × four
    rotations and one through four line-clear score/compaction cases.
- blockers:
  - VM fixture injection is still unavailable through AutoUI MCP, so the same
    full rules golden cannot yet be run against the VM implementation.
  - The complete M1–M7 cross-mode business matrix, persistence fault/concurrency
    cases, and stable narrow-window/theme pixel fixtures remain unexecuted.
  - Desktop discovery and the `05-games` gallery are still external to the
    current app scan root/category and remain blocked by `gallery_contract.py`.
- next: keep Plan `executing`; independently review/land auto-lang `86dae6ba4`,
  then run the remaining mode and gallery gates before `/auto-plan:review`.


### 2026-09-14 — execution follow-up / port allocation

- stage: work
- plan_id: PLAN-005
- plan_revision: 2
- outcome: partial
- code_commits: e18b791 (runtime/test port fix) and 81dcdf8 (manifest/README port alignment)
- change: Tetris pac.at now uses 17500/17501; README, Playwright default,
  persistence probe default, and the canonical app Spec matrix use the same
  ports. Repository search found no existing 17500/17501 registration.
- evidence: git diff --check passed; all checked-in Tetris port references are
  aligned. This removes the port collision only; R-001/R-005 native and gallery
  acceptance blockers remain unchanged.
- next: continue Plan 005 execution and re-review after the remaining acceptance
  evidence is complete.


### 2026-09-14 — execution follow-up / partial code landing

- stage: work
- plan_id: PLAN-005
- plan_revision: 2
- outcome: partial
- delivery_commit: 73413c5 (merge of integration commit a4bb893)
- landed: apps/036-tetris, docs/specs/apps/tetris.md, manifest entry, README
  registration, and the corrected 17500/17501 port allocation are now on
  auto-os main; ancestry to a4bb893 verified.
- acceptance: focused manifest/port checks and Python syntax checks passed.
  This is a phase landing only; the Plan remains executing and is not archived.
- blockers: R-001 and R-005 remain unchanged (complete cross-renderer/native
  and desktop/gallery acceptance still required).
- next: finish the remaining acceptance evidence, then run an independent
  revision-2 review before merge/archive.

### 2026-09-14 — execution follow-up / VM fixture rules golden

- stage: work
- plan_id: PLAN-005
- plan_revision: 2
- outcome: partial
- code_commits: `2bf5050` and `31ad06c` on `plan-005-dev`
- task_ids: T1, T5, T6
- delivered: added `tests/vm_rules_golden.py`, wired it into `run_matrix.py`,
  fixed the Windows gallery audit path crash, and refreshed the rules/capability
  evidence. The runner uses the opt-in AutoUI MCP `autoui_fixture` channel and
  drives the public `App` handlers; it does not add a debug control to Tetris.
- evidence: live merged VM MCP `http://127.0.0.1:9292/mcp` passed opening/lock,
  all seven pieces × four rotations (via collision checks of every occupied
  coordinate), and 1/2/3/4-line score/compaction cases. Python AST parsing and
  `git diff --check` passed. `gallery_contract.py --json` now emits explicit
  findings instead of crashing.
- blockers: native physical keydown/keyup, long-press/repeat, blur/focus,
  narrow-window/theme pixels, desktop discovery, and `05-games` gallery remain
  manual/framework acceptance items. Rust Cargo was rerun successfully with an
  explicit writable `CARGO_TARGET_DIR` and is no longer blocked. A fresh
  VM no-merge launch in this environment stops in the AutoVM server with
  `Module not found: auto.env`; the no-merge cross-mode leg therefore remains
  open. The physical driver also found no visible `俄罗斯方块` window to take
  over, while the merged MCP endpoint itself was reachable.
- next: perform the native and desktop/gallery manual matrix and request an
  independent revision-2 review;
  keep Plan status `executing` until those gates close.

### 2026-09-14 — execution follow-up / Rust native startup repair

- stage: work
- plan_id: PLAN-005
- outcome: partial
- change: the native Iced view no longer uses the styled-column → nested-row/
  nested-column level-card shape that caused `STATUS_STACK_OVERFLOW` during
  startup. The level/line summary is now a two-column `grid`, and unsupported
  `btn*`/`flex-wrap` classes were removed from the shared view source.
- evidence: `auto build -r rust --gen-only` regenerated the UI source; offline
  Cargo build passed; the generated Rust binary stayed alive for eight seconds
  and its MCP snapshot passed `tests/desktop_mcp.py` on port 9442. The startup
  log no longer contains the flex-wrap or button-style warnings.
- remaining: native physical input/window takeover, narrow-window and gallery
  acceptance, and VM no-merge environment validation remain separate gates.

### 2026-09-14 — functional acceptance reported by user

- stage: work
- plan_id: PLAN-005
- outcome: functional-complete
- acceptance: user manually verified that all seven modes (M1–M7) start and
  support the basic Tetris operations without functional issues. M5 VM/Rust
  no-merge, M6 Rust merged, and M7 Rust no-merge now start without the earlier
  layout stack overflow or button-style warnings.
- scope: this closes the planned feature behavior and the user-visible basic
  operation gate. Formal review evidence for long-press/keyup/blur, narrow
  window pixels, and desktop/gallery discovery remains a separate evidence
  concern if those gates are required for archival.

### 2026-09-14 — review / functional acceptance phase

- stage: review (phase-only)
- plan_id: PLAN-005
- plan_revision: 2
- outcome: pass
- reviewed_commit: `a4533a9296ac9c93f78c474df69706c5420dd525`
- base_commit: `31ad06c`
- dependency_revisions: auto-lang `a899ec8967fd05e0147889d01e18540ead62d31b`;
  renderer prerequisite remains recorded in the preceding execution entry
- spec_inputs: `docs/specs/apps/tetris.md` revision 1 and this Plan revision 2
- acceptance_results:
  - M1–M7: pass for the user-visible functional phase. The user manually
    confirmed that all seven Vue/VM/Rust merged/no-merge modes start and that
    the basic move, rotate, soft-drop, hard-drop, pause and line-clear flows
    work. M5, M6 and M7 also start without the earlier native layout overflow
    or style warnings.
  - implementation regression: pass. The reviewed worktree is clean at
    `a4533a9`; generated Rust source builds offline, the native process remains
    alive, and the MCP snapshot probe succeeds.
- findings:
  - no functional findings remain for the accepted M1–M7 basic-operation
    phase.
  - `R-001`/`R-005` remain open only for the separate archival evidence gates
    already named in prior records (long-press/keyup/blur, narrow-window and
    theme pixel fixtures, and desktop/gallery discovery). This phase verdict
    does not silently convert those unexecuted criteria into passes.
- evidence:
  - user acceptance in the 2026-09-14 functional test session (M1–M7).
  - `auto build -r rust --gen-only`, offline Cargo build, direct native
    process liveness probe, and `tests/desktop_mcp.py` snapshot probe as
    recorded in the preceding execution entry.
  - `git diff --check` passes; reviewed implementation worktree is clean.
- next: keep the overall Plan `executing` because this is a phase-only pass.
  The accepted functional delivery is ready for the remaining archival review
  gates; run `/auto-plan:review` again after those gates if final
  `reviewed`/merge status is required.

### 2026-09-14 — review / final scope amendment and merge approval

- stage: review
- plan_id: PLAN-005
- plan_revision: 3
- outcome: pass
- reviewed_commit: `a2eab26dc6f25821b48d5f051ef14d6121651b38`
- base_commit: `a8e65531015384b1c1658b42b56a6ec68111a41e`
- dependency_revisions: auto-lang `a899ec8967fd05e0147889d01e18540ead62d31b`
- spec_inputs: `docs/specs/apps/tetris.md` revision 1; no canonical behavior
  change, only the user-approved deferral ledger in `DEBTS.md`
- acceptance_results:
  - AC-01, AC-02, AC-05 and the basic portions of AC-03/04/06: pass. M1–M7
    start and the user verified the basic move, rotate, soft-drop, hard-drop,
    pause and line-clear flows in all seven modes.
  - AC-08: pass. The app is registered, the Spec is present, the generated
    targets and evidence are recorded, and the reviewed implementation is on
    the main tree.
  - deferred sub-gates: long-press/keyup/blur and visual fixtures →
    `DEBT-005-01`; desktop/gallery → `DEBT-005-02`; merged persistence fault
    matrix → `DEBT-005-03`. These are approved follow-up scope and do not block
    this delivery.
- findings: no remaining finding blocks the approved Plan 005 delivery.
- evidence:
  - user functional acceptance for M1–M7 on 2026-09-14;
  - VM fixture and Rust rules golden evidence recorded above;
  - current main tree contains the verified renderer-safe `app.at` and the
    canonical Tetris Spec; `DEBTS.md` preserves the deferred acceptance work.
- next: merge and archive Plan 005 with `completion_kind: delivered`; future
  debt work should use a new Plan and reference the three DEBT IDs.

### 2026-09-14 — merge receipt PLAN-005:r3

- stage: merge
- plan_id: PLAN-005
- plan_revision: 3
- outcome: in_progress
- prepared: reviewed baseline `a2eab26dc6f25821b48d5f051ef14d6121651b38`,
  canonical Spec `docs/specs/apps/tetris.md` revision 1, and approved debt
  projection `DEBTS.md`; delivery preparation committed as `091e722`.
- landed: the reviewed implementation tree and canonical Spec are ancestors of
  the current main branch; the renderer-safe Tetris view is present and the
  prepared Plan/debt/ledger delta is committed in `091e722`.
- ledger_refreshed: `.autoos/specs.json` now contains `P005-1`, `P005-2`,
  `P005-3`, `P005-4` and `P005-R1`, all pointing to the Plan and archived
  evidence target.
- archived: pending final `git mv` and status update.
- cleaned: pending `wt-guard.sh` verification and removal of the dedicated
  `plan-005-dev` worktree/branch.
- next: archive the Plan, clean its worktree, then append the final archive and
  cleanup receipts.

## 10. 待澄清事项

| ID | 问题 / 当前建议 | 责任人与下一步 |
|---|---|---|
| B1（已转债务） | 原生长按/keyup/失焦、窄窗/主题像素和完整物理输入证据尚未形成 | 见 `DEBT-005-01` |
| B2（已转债务） | merged 原生重启、损坏/只读、并发最大值和跨模式数据根矩阵尚未形成 | 见 `DEBT-005-03` |
| B3（范围偏差） | 真实 app 独立兄弟仓建议未执行；自动审查拒绝把 manifest 指向尚不存在的 `../auto-tetris`，当前实现保留在 `apps/036-tetris` 以保持可发现/可运行 | 若要迁移，先创建并验证目标仓，再单独提出 manifest/README 迁移变更 |
| Q1（已解决） | 用户明确 Vue 使用 HTTP；VM/Rust 支持 merged 和 no-merge | 2026-09-12 用户回复，revision 2 纳入；M8 不适用，不再请求确认 |
| Q2 | merged Rust 的通用 CRUD 仍不是本应用的持久化证明；当前已验证 scalar API 可生成 db 委托，但复杂 DTO/原生重启仍需实机证据 | 后续 T1/T6 继续用真实模式核验；缺口转 auto-lang 前置 Plan，保留 AC-01/06，不降为内存记录 |
| Q3（已转债务） | 端口、PLAN-013 集成、画廊外部源、三轨视口/键盘能力 | 端口已调整并验证；桌面/画廊后续见 `DEBT-005-02` |
| Q4（框架观察） | `grid` schema 的 `columns` 与 Vue 生成器的 `cols` 存在别名漂移；本次用 `cols` 规避，未改 auto-lang | 若其他 app 需要 `columns` 在 Vue 生效，另立 auto-lang 前置 Plan 同步 schema/生成器；本 app 以已验证产物为准 |

旧待澄清项已明确：v1 LCG 独立抽样、无踢墙保留；ghost 升为必验 UI 改善。

