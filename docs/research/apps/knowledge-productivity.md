# 知识与工作管理：六类应用的初期研究

日期：2026-10-22。公开功能资料主要为 A 级，许可抽样为 B 级；综合设计与实现均为 D 级建议。市场判断是需求分层，不含份额/营收。各 app 当前实现需在所属子模块另做基线审计。

<a id="notes"></a>
## 1. Notes 与知识库

**市场**：快速便笺、个人本地知识库、团队协作工作区是三个不同产品面。[Obsidian](https://obsidian.md/) 是本地知识工作的重要体验对照，[Notion](https://www.notion.com/product) 强调整合与协作；两者此处作为商业产品参考，不作为可直接复用的开源依赖。

| 开源/源码产品 | 已核查材料与优点 | 代价/取舍 |
|---|---|---|
| [Joplin](https://github.com/laurent22/joplin) | README：离线优先、Markdown、附件/搜索/导入导出、同步/剪藏；根 LICENSE 默认 AGPL，部分目录另有许可 | 借鉴可靠数据与导出；同步服务、插件和多端不是 Notes 首次捕获的前置 |
| [Logseq](https://github.com/logseq/logseq) | 仓库定位隐私优先知识管理，AGPL 元数据；后续研究大纲与关系工作流 | 本轮未核对其当前存储版本，不把历史 Markdown 模型强加于当前产品 |
| [Trilium Notes](https://github.com/TriliumNext/Trilium) | README：层级知识库、同一笔记多处组织、富文本；AGPL 元数据 | 树与链接可借鉴；应用内部数据库及脚本机制增加迁移成本 |
| [AppFlowy](https://github.com/AppFlowy-IO/AppFlowy) | 仓库定位项目/wiki/团队的协作工作区；AGPL 元数据 | 可研究结构化内容；首批不复制完整 Notion 式工作区 |
| [Outline](https://github.com/outline/outline) | README：团队知识库，React/Node；已读 LICENSE 为 BSL 1.1 | 可参考产品分工；**当前不是 OSI 开源许可**，不可按 MIT 等宽松依赖处理 |

**建议设计**：Notes 做统一收件箱：文字/链接/图片/附件带来源进入，快速搜索、少量标签、Markdown/资源目录导出；重型文档、知识关系与长篇编辑由 AutoJade 等既有产品承接。先明确对象身份、编辑归属与导出格式，再讨论是否共用数据库。

本地只读 `D:/autostack/jade-edit/README.md` 已确认：产品名 AutoJade，定位 AutoDown `.ad` 文档与本地知识库，已有链接/反链、搜索、标签、文件组织、草稿恢复等文档记载；它明确与面向代码/文本的 AutoEdit 区分。manifest 的 `jade-garden` 又是另一个旧定位，不能因名称接近就合并数据。上述是文档事实，未做运行复验。

**实现候选**：Notes 的 AutoDown 依赖在[独立应用 Spec](../../specs/apps/independent-repositories.md)已有记录，先复核现存引擎、schema 与 pin，再决定是否补 SQLite 索引。文件或数据库只指定一个真值来源，全文索引可重建。捕获以幂等 request ID + 持久回执落地；附件先完整落盘再提交记录；退出重启后可查询。跨 app 通信沿现有 F0/F1/F2 设计，不在 Notes 单独发明 broker/权限体系。

**验证闭环**：Reader 摘录→Notes 收件箱→检索→交给 AutoJade/导出；中途崩溃、重复提交、附件失败、断网后仍可解释。持久回执不能只代表 UI 显示了新一行。中日韩文分词/子串搜索需实际语料，不能仅接 FTS 就宣称中文检索质量达标。

<a id="tasks"></a>
## 2. Todo

**市场**：个人快速执行与团队项目管理分层明显。[Todoist](https://www.todoist.com/) 官方强调自然输入、Today/Upcoming 和过滤等工作流。AutoOS 应先解决“记下来→今天处理→可靠提醒”，避免复制项目管理平台。

**开源参考**：[Planify](https://github.com/alainm23/planify)（GPL-3.0 元数据，Vala/GTK）README 列明项目/分区、搜索、重复任务、提醒、Todoist/Nextcloud 连接与离线模式；适合研究低门槛任务流。[Vikunja](https://vikunja.io/) 官方提供列表、看板、甘特等视图和自托管，声明 AGPL；本次 GitHub 路径查询失败，改以官网为准，未审源码。其多视图价值伴随更大服务/协作面。

**综合设计**：Inbox、Today、Upcoming、项目、标签；快速添加、截止日期、优先级、子任务、完成/恢复。自然语言日期只是录入辅助，必须显示解释后的绝对日期、时区及可修改项。重复任务显式区分“按计划日期重复”和“完成后间隔”；不能把所有重复算成秒数相加。

**实现候选**：Auto 应用层 + 本地事务存储；候选模型含稳定 ID/rev、due、完成记录、recurrence 和外部关联。提醒由系统调度能力承接。与 Kanban 可共享身份/链接，但是否共用任务实体须先审计两个现存 schema；本轮不迁移数据。初期 JSON/ICS 导出与备份先于云同步。

**验证**：离线添加、重启、重复任务一次完成、撤销、月末/夏令时、通知去重与窗口关闭后的行为；从 Launcher 添加的任务与 UI 同一真值。体验门为用真实一周任务跑流程，不只测按钮。

<a id="calendar"></a>
## 3. Calendar

**市场**：个人安排、组织会议、账号协作是不同复杂度；日历真正难处是时间模型与互通。[Nextcloud Calendar](https://github.com/nextcloud/calendar) 提供 WebCal、与任务/其他应用集成、邀请、提醒等（README A）；仓库 AGPL。[GNOME Calendar](https://github.com/GNOME/gnome-calendar) 是桌面个人日程的轻量产品对照（GPL 元数据，本轮主要核对定位）。

**综合设计**：日/周/月 + 议程，快速创建，多日历颜色，全天与有时区事件，重复与单次例外，ICS 导入导出。Todo 的截止日期投影到日历，但事件和任务不是同一概念。多人邀请、CalDAV 双向同步和组织可用性晚于本地闭环。

**实现候选**：参考 [RFC 5545](https://www.rfc-editor.org/info/rfc5545/) 的事件/重复模型与后续更新，不自创字符串时间协议。Nextcloud README 指向 ical.js、fullcalendar、c-dav library，可作候选入口；这些浏览器或服务组件不等于 VM 原生 UI 现成能力。先复用经过验证的 recurrence/时区库接缝，再用 AutoUI 做视图。

**验证**：全天日期不因时区漂移、跨午夜/夏令时、月末重复、例外修改/取消、导入重复去重、ICS 往返、睡眠恢复后的提醒。账号认证/冲突处理需要额外服务合同，不能借“支持 ICS”宣称已支持 CalDAV。

<a id="kanban"></a>
## 4. Kanban

**市场**：[Trello](https://trello.com/) 是看板与工作流整合的商业参照；个人卡片与团队权限/审计/自动化的成本不同。已有 `apps/auto-kanban` 应沿原计划增强。

**候选**：[WeKan](https://github.com/wekan/wekan) README 定位自托管协作看板，MIT；可借鉴卡片、列表、协作与数据可控性，代价是独立服务和团队运维。[PLANKA](https://github.com/plankanban/planka) 当前 README 自称 open source，但实际 [LICENSE.md](https://github.com/plankanban/planka/blob/master/LICENSE.md) 为 Community License 并区分 Pro/Enterprise；本轮仅列设计参考与许可待审，不作为宽松开源代码来源。

**综合设计**：项目→列→卡片→清单/标签/附件，键盘与拖放等价，撤销和持久化真实；支持筛选与导出。Todo 做个人执行视图，Kanban 做项目状态视图，通过稳定链接与可选映射相连，避免两份不一致任务。协作/离线编辑冲突、权限与自动化独立立项。

**实现候选与门**：先核查[现有 Kanban Spec](../../specs/apps/kanban.md)，复用现有领域层；顺序键与事务移动避免重排丢卡。验证移动中断/重复请求、同时修改、附件失败、导出回读与键盘完成同一流程；此处不是要求当前版本立即实现多人离线协作。

<a id="blog"></a>
## 5. Blog / 发布工作台

**市场**：个人静态发布、独立作者出版/订阅、内容社区并非一种产品。[Ghost](https://github.com/TryGhost/Ghost)（MIT 元数据）覆盖独立出版、会员/订阅与自托管/托管服务；[Zola](https://github.com/getzola/zola)（EUPL-1.2 元数据）是单二进制静态站生成器，README 列出模板、资源、目录/分页、静态搜索等。前者是完整服务，后者是发布引擎，不混称两个桌面编辑器。

**综合设计**：Auto Blog 首版是写作发布工作台：草稿、本地预览、元数据/附件、导出不可变快照、发布状态和失败重试。优先复用 AutoJade/编辑器的文档输入，保留原 `023-realworld` 入口与历史来源。社区关注/评论/推荐、邮件列表/收费内容不会因做了发布按钮就具备，另行论证。

**实现候选**：Markdown/AutoDown→中间文档→HTML/静态资源；优先导出 Zola 等引擎可用目录，或通过显式 provider 调用外部发布 API。记录草稿 rev、导出 hash、目标及发布回执；资源引用在导出快照内完整，外链失效可诊断。网络发布与本地预览两条动作分开。

**验证**：无网写作、崩溃恢复、附件相对路径、引用/代码/图片导出、导出后草稿修改不污染旧快照；重复点击发布不产生未知重复内容。需要凭据/外部写入的试验用专用测试站点。

<a id="chat"></a>
## 6. Chat

**市场与首要决策**：AI 助手、个人 IM 和团队/联邦通信是三条产品线。当前 `017-chat` 名称与界面不能替代产品身份决定。AutoOS 已有 auto-ai 与 Musk，若 Chat 是普通 AI 对话，应共享模型/凭据/使用量基础；若是人与人通信，需要现成通信协议与身份生态。

**开源参考**：[Element Web](https://github.com/element-hq/element-web) 为 Matrix 客户端（AGPL 元数据）；当前 [Synapse](https://github.com/element-hq/synapse) 为服务端（AGPL），旧 `matrix-org/synapse` 已归档。本轮只确认架构/定位，不声称已审密码协议、完整 E2EE 实现或可直接裁剪。

**综合设计**：先定 AI/IM；共通最小能力是会话、真实历史、搜索、取消/重试、附件状态与可导出。AI 分支需要流式回复、上下文/成本可见、工具执行回执；IM 分支需要账户/设备、离线发送、去重及送达状态。二者可以共享外壳但不能伪造相同协议。

**实现候选与门**：AI 接已有 auto-ai；IM 集成成熟 Matrix 客户端或经审 SDK，避免自研加密协议。在产品身份确定前只做隔离探针。验证断网/重连/重复消息、历史恢复、取消后最终状态；端到端加密声明必须有对应实现与验证，不能由使用 HTTPS 推导。
