# B类应用基本能力补齐：总跟踪表

更新：2026-10-29。性质：跨应用项目总表，**不是覆盖九个应用的巨型执行Plan**，不占用AutoOS/应用计划号，不复制子计划的任务状态。实施、修复、复审与交付以各应用独立计划为准。

## 当前范围与两路安排

产品统计沿[基础能力盘点](../research/apps/basic-capability-audit-20261029.md)：28个manifest登记入口，去重为27个应用。AutoJade旧称Jade Garden，两条历史入口只算一个产品。当前A类18个，B类9个；AutoShell/AutoEdit为登记外生态补表。

- **A路：Linux支持目前暂停启动**。统一状态 `WAIT_DISTRO_M1`，等待 [auto-os-distro](https://github.com/auto-stack/auto-os-distro) 完成第一阶段 **Nix＋COSMIC基础**。本表不启动Linux应用适配或以Windows结果代替Linux验收。
- **B路：现在准备基本能力补齐**。Notes、Reader、Blog复用原001；其他六个应用各有新计划。当前只完成计划编制，不把draft或源码存在当作基本功能已经通过。
- 基础能力通过后，Linux与后续增强可以分路进行；本会话不展开长期增强、全量竞品复刻或新的产品大改版。

## B类主表（9个应用）

“就绪”只表示可以接续对应阶段，不表示自动执行。状态取自当前Plan；后续更新本表须保留实际证据，禁止仅改总表就升A。

| 顺序 | 应用 | 基本合格线 | 独立计划 | 当前状态 / 下一步 | 前置或边界 |
|---|---|---|---|---|---|
| 1 | Notes | 真保存、重开、冲突/失败恢复，满足原AC-01–05 | [NOTES-001](../../apps/auto-notes/docs/plans/001-durable-inbox.md)（复用） | execution_done / r8，work 35/35；接最终复审，不重做主体 | final review必须绑定实际修订/提交和真实双端证据；本轮未复审 |
| 2 | Todo | 任务CRUD、持久化、重启恢复和失败反馈 | [TODO-001](../../apps/auto-todo/docs/plans/001-durable-todos.md)（新） | drafting / r1；next=T-01能力探针 | 空库不播种demo；单写者/损坏/写失败不可假成功 |
| 3 | Calendar | 真实今日、单次事件CRUD、保存与重开 | [CALENDAR-001](../../apps/auto-calendar/docs/plans/001-local-events.md)（新） | drafting / r1；next=T-01日期/存储探针 | 不含重复/同步/提醒；日期和本地时区语义先冻结 |
| 4 | Paint | 16×16作品编辑、可靠撤销/保存重开、导出图像 | [PAINT-001](../../apps/auto-paint/docs/plans/001-reliable-pixel-paint.md)（新） | drafting / r1；next=T-01恢复/输出探针 | 修P553-D1，全同色不可SKIP；首版SVG输出，不加专业画布 |
| 5 | Reader | 现有真实导入、阅读、位置恢复合同完成 | [READER-001](../../apps/auto-reader/docs/plans/001-real-library.md)（复用） | executing / r7；按原计划关闭前置并重验 | 当前终审blocked；F-12目标滚入pane、真实大文本/VM等原必需项不能用债务关闭；不追加EPUB/PDF全功能 |
| 6 | Blog | 原合同的真实草稿保存、恢复、迁移与编辑保护 | [BLOG-001](../../apps/auto-blog/docs/plans/001-durable-drafts.md)（复用） | executing / r6；按原修复任务接续 | 旧pass已被后续needs_fix取代；不增社区、远端发布或全部预览能力 |
| 7 | Database | 打开真实SQLite、schema、只读SQL与真实错误 | [DATABASE-001](../../apps/auto-database/docs/plans/001-real-sqlite-browser.md)（新） | drafting / r1；next=T-01 SQLite全路径探针 | 禁止打开失败回退内存冒充连接；只读能力和动态结果未证实前不做假SQL；写按钮不可装可用 |
| 8 | Charts | CSV用户数据→柱图→保存重开→独立SVG输出 | [CHARTS-001](../../apps/auto-charts/docs/plans/001-data-to-chart.md)（新） | drafting / r1；next=T-01输入/几何/输出探针 | 真实CSV和数值，不能split逗号冒充解析；不含实时仪表盘 |
| 9 | Chat | 人与人真实收发、会话隔离和历史恢复 | [CHAT-001](../../apps/auto-chat/docs/plans/001-real-conversations.md)（新） | drafting / r2；next=T-01真实人际通道探针 | 用户已确认人与人聊天、沿用联系人界面；先验证通道/身份，首版不做AI客户端 |

顺序是建议调度，不是强制串行依赖：Notes最终复审、Todo能力探针等互不依赖；Reader框架阻塞或Chat通道调查不拖住其他应用。一个app一个写入者，不擅自启动并行agent/会话。

## 基础门、状态与升类规则

1. 每个app先按自己的Plan固定最小任务，不以窗口/截图/按钮数量代替真实数据闭环。原合同内必需目标不能因表中摘要而缩减。
2. work阶段完成真实用户数据、保存/重开、失败/恢复和实际声明支持的Vue/VM任务；若有框架缺项，指明owner、前置计划和关闭结果，不用mock顶替。
3. execution_done只说明实现交接；最终复审、依赖/规范增量和对应门通过后交付main。父仓pin必须指向实际交付，canonical/ledger/归档及必要组合验证可追溯。
4. 完成后以交付提交、验收结果/链接和日期更新本表及基础盘点，将该app由B升A；升级只依据既定基础合同，不额外捆绑未来增强。
5. 新升A的Linux工作仍服从 `WAIT_DISTRO_M1`。本表状态不是独立于Plan的第二套工作流，不因某项出阻塞就把其他app标为整体阻塞。

## A类 Linux 前置（18个去重应用）

解除条件：发行仓提供其第一阶段Nix＋COSMIC基础完成的版本化验收回执，注明可用于应用验证的源码/产物版本与首验环境；届时再准备各应用Linux任务。仅建好仓库、放出ISO文件或完成[OS PLAN-052建仓](052-distro-repository-setup.md)不等于该基础已完成。当前按用户明确决定保持等待，不推断已可开工。

| 应用 | 当前状态 | 共同前置 |
|---|---|---|
| AutoKanban | WAIT_DISTRO_M1；未启动Linux适配 | auto-os-distro：Nix＋COSMIC基础完成 |
| AutoMusk | WAIT_DISTRO_M1；未启动Linux适配 | auto-os-distro：Nix＋COSMIC基础完成 |
| AutoJade | WAIT_DISTRO_M1；未启动Linux适配 | auto-os-distro：Nix＋COSMIC基础完成 |
| AutoTerm | WAIT_DISTRO_M1；未启动Linux适配 | auto-os-distro：Nix＋COSMIC基础完成 |
| AutoSolitaire | WAIT_DISTRO_M1；未启动Linux适配 | auto-os-distro：Nix＋COSMIC基础完成 |
| AutoTetris | WAIT_DISTRO_M1；未启动Linux适配 | auto-os-distro：Nix＋COSMIC基础完成 |
| AutoMinesweeper | WAIT_DISTRO_M1；未启动Linux适配 | auto-os-distro：Nix＋COSMIC基础完成 |
| AutoLauncher | WAIT_DISTRO_M1；未启动Linux适配 | auto-os-distro：Nix＋COSMIC基础完成 |
| AutoSyslog | WAIT_DISTRO_M1；未启动Linux适配 | auto-os-distro：Nix＋COSMIC基础完成 |
| AutoCalc | WAIT_DISTRO_M1；未启动Linux适配 | auto-os-distro：Nix＋COSMIC基础完成 |
| AutoClock | WAIT_DISTRO_M1；未启动Linux适配 | auto-os-distro：Nix＋COSMIC基础完成 |
| AutoWeather | WAIT_DISTRO_M1；未启动Linux适配 | auto-os-distro：Nix＋COSMIC基础完成 |
| AutoVideo | WAIT_DISTRO_M1；未启动Linux适配 | auto-os-distro：Nix＋COSMIC基础完成 |
| AutoMusic | WAIT_DISTRO_M1；未启动Linux适配 | auto-os-distro：Nix＋COSMIC基础完成 |
| AutoPhotos | WAIT_DISTRO_M1；未启动Linux适配 | auto-os-distro：Nix＋COSMIC基础完成 |
| AutoMonitor | WAIT_DISTRO_M1；未启动Linux适配 | auto-os-distro：Nix＋COSMIC基础完成 |
| AutoExplorer | WAIT_DISTRO_M1；未启动Linux适配 | auto-os-distro：Nix＋COSMIC基础完成 |
| AutoOS Config | WAIT_DISTRO_M1；未启动Linux适配 | auto-os-distro：Nix＋COSMIC基础完成 |

AutoShell/AutoEdit虽不在上述登记数量内，其Linux工作也等待相同发行版基础。本表不擅自改manifest的旧入口或合并AutoJade历史数据。

## 计划归属与执行位置

- 总表在AutoOS；每个app的Plan在 `apps/<产品仓名>/docs/plans/`，各仓独立取号。六个新计划均为本仓001，后续号源记录在各自 `docs/plans/.next-id`；不会占用OS/语言的全局号。
- app工作在相应 `auto-os/apps/<产品仓名>`，main上开发并同步，单app单写者；提交/推送app、更新父仓gitlink后显式detach。应用代码不另建外部worktree，不建junction/symlink。
- 框架/OS核心缺口在主改面仓独立立项并遵守worktree纪律。Plan里列的能力探针先于依赖实现，不预先声称其已经可用。当前三仓工具链定位沿既有解析序。
- Notes/Reader/Blog现有Plan原文、修订、勾选和历史记录不修改；复用入口已在主表记录，不能重建另一个“基本版001”绕开旧失败。

## 本次建表交付与验证

本次产物：本总表、六份drafting计划（五份r1，Chat r2）及对应app README反链；未写业务源码、未运行应用功能验收、未更新canonical Spec/ledger或将任何B类升A。文档组合提交会按子模块纪律更新六个app的gitlink；这不是功能交付。

文档检查：9个B类都有唯一主计划入口，A类按产品去重18个且均有发行版前置；六份新计划均有0..10章节、7个未完成任务、AC-01..06和规范增量；源码事实与拟议新路径/测试明确区分；三份复用Plan哈希保持。具体版本与检查回执见 [准备回执](attachments/b-apps/planning-receipt.json)。

关联：[基础能力盘点](../research/apps/basic-capability-audit-20261029.md)、[App子模块纪律](../design/strategy/app-submodule-workflow.md)。总表状态后续由实际工作与复审证据更新，不设置定时器或自动执行。
