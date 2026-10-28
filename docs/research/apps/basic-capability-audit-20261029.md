# 应用基本能力盘点：哪些可优先 Linux 化，哪些先补成应用

日期：2026-10-29。范围：apps.manifest 的 28 个登记入口，按产品去重后为 27 个应用；另列 AutoShell / AutoEdit，不混入登记数量。

## 结论

- **A：18 个已有真实基本功能实现，可优先进入 Linux 兼容性候选队列。** 先保留现有基本任务，不在本会话扩展功能。
- **B：9 个应先完成基本应用。** 其中 4 个为界面/协议/组件演示（Calendar、Chat、Database、Charts），4 个为已有部分真实功能但基础缺口未闭合（Todo、Paint、Reader、Blog），1 个为实现已申报完成、待最终核验（Notes）。B 不等于都要从零重写。

先达到每个 app 的基础必要能力；达到后，Linux 支持与后续增强可以并行。增强与竞品综合设计另开会话；本轮只分类和确定基础补齐清单。

## 产品去重修正

用户于 2026-10-29 明确：Jade Garden 是 AutoJade 的旧名，两者是同一应用。manifest 当前保留 `jade-garden` 与 `jade-edit` 两条接入记录，盘点按一个 AutoJade 统计。因此登记入口仍为 28 条，独立产品为 27 个：A 类 18 个，B 类 9 个。两个位置的源码证据在 JSON 中保留，不能合并推导所有形态/版本都已通过；本次不擅自删除入口、移动仓库或合并数据。

## 判定口径与证据边界

“基础必要能力”是用户能用自己的数据完成该应用的一个真实核心任务；需要保存的数据能正常保存/重开，错误不冒充成功。真实任务不等于功能清单已经很长，也不等于必须实现成熟竞品全部能力。

本次读取应用源码、Spec、计划和已有证据记录，没有启动应用或重跑功能套件。因此 A 表示代码与现有记录支持已有基础实现，**不是本轮保证全部后端、全部功能和发行包均已验收**。后续移植前应运行该 app 的现有平台基线。B 中的明确实现缺口有直接源码证据；已知失败不因界面可打开而降为扩展需求。

已实际运行子模块健康闸门：exit 0 / healthy，27 个 active repo 条目缺 pac.at 为 0。该检查只证明组合/检出健康，不证明 27 个去重产品可用。源码 pin、选读文件哈希、具体行号和 WIP 快照见 [结构化盘点](basic-capability-audit-20261029.json)。没有切换 app 分支、修改 app、更新 pin 或更改他人 WIP。

## A：已有基本实现，优先考虑 Linux

| 应用 | 已有的基本闭环 | 本轮必须保留的边界 | 证据 |
|---|---|---|---|
| AutoKanban | 已有真实计划扫描与配置驱动的只读看板 | 当前定位是只读计划板，不要求拖拽写回才能算基本可用 | [README.md](../../../apps/auto-kanban/README.md) / [lang_plans.at](../../../apps/auto-kanban/src/back/lang_plans.at) |
| AutoMusk | 已有项目、聊天、工具执行和 Plan/Spec 工作流实现 | 依赖 AI 服务、配置与工具链；本轮未重新验证全流程 | [README.md](../../../../auto-musk/README.md) / [tools.rs](../../../../auto-musk/backend/crates/musk/src/tools.rs) |
| AutoTerm | 已有 PTY/终端引擎、Auto 前端及会话 API | 已有支持声明以 Windows 为准；Unix 代码路径不等于 Linux 已验收 | [README.md](../../../apps/auto-term/README.md) / [term.rs](../../../apps/auto-term/app/term.rs) |
| AutoSolitaire | 已有接龙规则与真实游戏状态机 | 发行包与平台输入仍需验；不以新玩法为前置 | [klondike_store.at](../../../apps/auto-solitaire/src/front/klondike_store.at) / [desktop_mcp.py](../../../apps/auto-solitaire/tests/desktop_mcp.py) |
| AutoTetris | 已有规则/计时循环与最高分保存实现 | 历史脚本不等于本轮通过；平台计时/输入另验 | [tetris_store.at](../../../apps/auto-tetris/src/front/tetris_store.at) / [db.at](../../../apps/auto-tetris/src/back/db.at) |
| AutoMinesweeper | 已有布雷、邻雷计数、展开和胜负逻辑 | 不要求新增联网排行榜或关卡才能基本可用 | [minesweeper_store.at](../../../apps/auto-minesweeper/src/front/minesweeper_store.at) / [minesweeper.md](../../specs/apps/minesweeper.md) |
| AutoLauncher | 宿主覆盖注册列表并消费 desktop 命令；有实际搜索/动作路径 | 独立模式显式 mock，判定仅限桌面宿主应用启动器；尚非完整独立全盘搜索工具 | [app.at](../../../apps/auto-launcher/src/front/app.at) / [001-query-provider-core.md](../../../apps/auto-launcher/docs/plans/001-query-provider-core.md) |
| AutoSyslog | 宿主写入 __syslog_*，app 消费并过滤 | 独立 Vue 模式是 mock；基本能力仅按桌面宿主日志查看器判定 | [README.md](../../../apps/auto-syslog/README.md) / [renderer.rs](../../../../auto-ui/crates/auto-ui-runtime/src/ui/iced/renderer.rs) |
| AutoJade（旧称 Jade Garden） | 已有文件/文档编辑及基础读写验收记录；两个名字属于同一应用 | RELEASE 仍 preparation_only，功能已有不等于独立安装包/正式发布已完成 | [wsys.at](../../../../jade-edit/src/back/wsys.at) / [RELEASE.md](../../../../jade-edit/docs/RELEASE.md) |
| AutoCalc | 已有表达式求值、科学/程序员运算实现 | 不以新单位库或更多模式为基础门 | [app.at](../../../apps/auto-calc/src/front/app.at) / [acceptance.atd](../../../apps/auto-calc/tests/acceptance.atd) |
| AutoClock | 使用 Time.now_sec，真实 Tick 计时及闹钟列表存储 | 固定世界时区偏移；闹钟仅窗口内 Tick/横幅，不能宣称关窗后台可靠提醒。A 仅覆盖基础时钟/前台计时 | [app.at](../../../apps/auto-clock/src/front/app.at) / [desktop_mcp.py](../../../apps/auto-clock/tests/desktop_mcp.py) |
| AutoWeather | 后端实际请求 Open-Meteo/QWeather，城市与设置写文件 | 仍有显式演示降级；演示数据不能算实时取数成功。A 只指已有真实 API 路径 | [api.at](../../../apps/auto-weather/src/back/api.at) / [app.at](../../../apps/auto-weather/src/front/app.at) |
| AutoVideo | 已真实扫描及受控 video；Spec 记录 Vue 播放闭环 | 不同后端解码依赖有条件，VM 本地选文件会降级；A 不等于原生/Linux 视频已验收 | [player_store.at](../../../apps/auto-video/src/front/player_store.at) / [viewport.at](../../../apps/auto-video/src/front/viewport.at) |
| AutoMusic | 真实 /api/media/scan 数据和受控媒体状态，并非静态歌单 | 基本播放成立不表示已有完整音乐库、媒体键或跨重启续播；本轮不增加这些增强 | [player_store.at](../../../apps/auto-music/src/front/player_store.at) / [viewport.at](../../../apps/auto-music/src/front/viewport.at) |
| AutoPhotos | 扫描/缩略图/原图真实服务与目录浏览、收藏存储 | A 是基础看图，不代表照片管理/编辑器已实现；尚无独立完整测试套件证明所有功能 | [app.at](../../../apps/auto-photos/src/front/app.at) / [pac.at](../../../apps/auto-photos/pac.at) |
| AutoMonitor | 后端实际调用 sys.cpu_usage/processes/disks 等 | A 先以只读监视器判定；结束进程的身份/权限安全另验 | [api.at](../../../apps/auto-monitor/src/back/api.at) / [SPEC.md](../../../apps/auto-monitor/SPEC.md) |
| AutoExplorer | 已有真实文件服务和批量操作/搜索/预览实现 | README 001–006 统一复审未在本轮重跑；启动模式和大目录边界要复核 | [app.at](../../../apps/auto-explorer/src/front/app.at) / [api.at](../../../apps/auto-explorer/src/back/api.at) |
| AutoOS Config | 已有统一配置服务与实际配置编辑前后端 | 现有应用配置中心不等于 Linux 网络/电源/音频控制面板 | [README.md](../../../apps/auto-os-config/README.md) / [api.at](../../../apps/auto-os-config/auto/src/back/api.at) |

## B：先补齐/验收基本应用

| 应用 | 已有内容与明确缺口 | 最小合格线 | 下一步 | 证据 |
|---|---|---|---|---|
| AutoTodo | 已有 CRUD，但 db.at 明确内存列表与 seed 初始化；任务重启丢失；缺持久化与失败回执，尚不足以做日用任务应用 | 创建/编辑/完成/删除真实任务→保存→重启恢复 | 首批补持久化、重启恢复和写入失败反馈 | [db.at](../../../apps/auto-todo/src/back/db.at) / [api.at](../../../apps/auto-todo/src/back/api.at) |
| AutoNotes | 已有真实仓储、恢复和 UI；NOTES-001 r8 execution_done，35/35 work tasks；AC-01–05 仍要求绑定修订的最终复审；不能仅凭 execution_done 判已基本可用 | 录入→真实保存→重启回读→检索/删除恢复，保存失败不丢编辑 | 先完成现有 001 的最终核验；通过即可升 A，不重做 UI | [001-durable-inbox.md](../../../apps/auto-notes/docs/plans/001-durable-inbox.md) / [repository.at](../../../apps/auto-notes/src/back/repository.at) |
| AutoCalendar | 目前只有月份导航和日期选择；today 固定 2026-06-17；无事件模型/事件持久化 | 真实今日/月视图→创建/编辑/删除事件→保存重开 | 先接真实日期并补本地事件 CRUD 与恢复 | [calendar_store.at](../../../apps/auto-calendar/src/front/calendar_store.at) / [app.at](../../../apps/auto-calendar/src/front/app.at) |
| AutoChat | 有消息 API/SSE 演示，但联系人和消息为内存种子，bot_reply 固定文本；既不证明实际 AI 调用，也不证明真实多人通信；重启无可靠历史 | 明确 AI 或人际聊天→真实发送/回复→会话隔离→历史恢复 | 先确定基本产品身份，再接真实通道与会话存储；不把加密/联邦等增强加入本轮 | [db.at](../../../apps/auto-chat/src/back/db.at) / [db.at](../../../apps/auto-chat/src/back/db.at) |
| AutoReader | 已有 TXT/Markdown 书库、导入和恢复实现；READER-001 r7 最终裁决 blocked，基础合同仍有具名框架缺项；不能称完整 EPUB/PDF 阅读器 | 一种明确支持格式的真实导入→正文/目录→保存位置→重开续读 | 沿现有 001 关闭基础阻塞并验收；本轮不追加 Readest 全功能复刻 | [001-real-library.md](../../../apps/auto-reader/docs/plans/001-real-library.md) / [real-library.md](../../../apps/auto-reader/docs/specs/reader/real-library.md) |
| AutoBlog | 已有作者模式、真实草稿仓储与 UI；当前 001 r6 复审有输入丢失、恢复/迁移及虚假 saved 等未闭合项，旧 merge pass 已被后续复审取代 | 本地创作→保存→重启恢复→继续编辑；保存失败不假报成功 | 沿现有 001 修复基本草稿正确性；预览/完整发布包仍按既有后续合同，不在本轮扩围 | [001-durable-drafts.md](../../../apps/auto-blog/docs/plans/001-durable-drafts.md) / [drafts.at](../../../apps/auto-blog/src/back/drafts.at) |
| AutoPaint | 已有真实 16×16 像素格铅笔/橡皮/填充/吸色和 storage 存取；未达到通用画图工具；现有 SPEC 登记全同色快照 Redo/Load 塌缩，且未见图片文件导出闭环 | 绘制一张作品→撤销→可靠保存/重开→可导出使用 | 先明确像素画板最小范围，修存取/撤销正确性并做文件输出，不扩专业画布 | [SPEC.md](../../../apps/auto-paint/SPEC.md) / [app.at](../../../apps/auto-paint/src/front/app.at) |
| AutoDatabase | 界面具备对象树/结果表，但 RunSql 按字符串返回预置列表；CommitTx/RollbackTx 仅清计数和改提示，无 SQLite 连接/事务 | 打开用户 SQLite→真实表/SQL→真实结果/错误；写操作若开放须真实事务 | 先做真实 SQLite 连接与读查询，事务按钮必须真实或明确不提供 | [app.at](../../../apps/auto-database/src/front/app.at) / [app.at](../../../apps/auto-database/src/front/app.at) |
| AutoCharts | 已有真正图表渲染与切换，但数据是内嵌 monthly/seed 与模拟 Tick；缺用户数据输入、图表规格保存和结果导出，是图表组件演示 | 导入用户数据→选一种图表→正确呈现→保存/导出结果 | 先补一个 CSV→图表→导出的最小闭环 | [app.at](../../../apps/auto-charts/src/front/app.at) / [SPEC.md](../../../apps/auto-charts/SPEC.md) |

## 两个容易误判的例子

**Kanban** 当前产品定位就是只读计划看板。真实扫描计划并显示状态已是它的基本任务；不能以没有可写卡片/拖拽作为不合格证据。

**Clock、Launcher、Syslog、媒体应用** 要按实际形态判断：Clock 的前台计时可用不代表后台闹钟可靠；Launcher/Syslog 的独立 fixture 不能冒充宿主功能，但它们已有真实宿主接入；Video 的 Web 播放不能推导原生端解码已完成。表格中的 A 均保留这些范围限制。

**Notes** 不应与纯 UI demo 混为一谈。现有 001 为 execution_done/r8，实际仓储和修复已写出；之所以暂留 B，是当前基本合同仍待最终核验。通过现有验收即可升 A，不再额外加功能作为升级条件。

## 本会话后续补齐顺序

1. **Notes：先收尾已有基础验收**，可能最快从 B 升 A；不重开一套产品设计。
2. **Todo：优先补真实持久化和重启恢复**。已有 CRUD 可沿用，目标窄、完成后立刻能日用。
3. **Calendar / Paint：分别补事件与持久化、作品保存正确性与文件输出**。先定最小范围，不加协作、专业笔刷等增强。
4. **Reader / Blog：沿已有 001 关闭基础阻塞与正确性问题**，不再开重复计划；框架缺项需在主改面仓明确处理。
5. **Database / Charts：分别跑通 SQLite 真实查询、用户数据到图表输出**。先完成一个真实端到端任务。
6. **Chat：产品身份是基本任务的前置**。真实 AI 对话或真实人际通信选其一后再实施，不能把固定 bot 回复改文案当补齐功能。

这是基于缺口大小和现有工作的建议队列，不是已启动九个应用实现。每个基本能力合同进入对应 app 的 Plan，并以真实保存/重启/错误与当前支持端验收；达到基础合格线后再进入 A。暂不为这些 app 增加长期扩展路线。

## 主表外的生态应用

- **AutoShell / ASH**：已有真实命令/结构化管道/外部程序能力，作为 A 候选另列；见 [README](../../../../auto-shell/README.md)。不把 AutoTerm 和 AutoShell 合并为一个应用。
- **AutoEdit**：已有真实文件编辑/保存与会话实现，作为 A 候选另列；当前应用入口在 [specs/auto-edit](../../../../auto-edit/specs/auto-edit/README.md)，尚未在本仓 manifest 登记。登记/打包是集成工作，不据此否认已有编辑功能。

前期竞品和改进讨论保留为研究背景；当前基础能力判定与分工以本盘点为准。Linux/发行形态的真实支持范围仍须在相应任务中单独验收。
