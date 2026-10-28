# AutoOS 应用产品化调研资料库

初版：2026-10-22。状态：初期研究与范围建议，供下一阶段设计/Plan 使用；不是已冻结路线、实现验收或发版承诺。

2026-10-29 当前工作：[应用基础能力盘点](basic-capability-audit-20261029.md)。按用户明确的两层要求，先分“已有基本实现 / 先补基本应用”：28 个登记入口中 A 类 19 个、B 类 9 个；Notes 属实现已申报完成但待最终核验，不当作纯 UI demo。源码、范围限制和固定快照见[结构化盘点](basic-capability-audit-20261029.json)。

当前会话先分类并优先补齐 B 类基础能力；A 类可以优先考虑 Linux 支持，后续增强另开会话讨论。基础能力通过后，Linux 与增强可并行。前期[Linux/产品分期建议](linux-mvp-priorities.md)保留作背景，不替代本次基本能力判定。

## 先读结论

现行工作次序以基础能力盘点为准。以下产品组合、竞品和增强路线是前期研究，不表示已经达到基本可用，也不作为本会话新增功能清单。

下一版应支持一套能完成日常工作的应用组合，并按能力成熟度发布。现有应用继续纳入产品盘点；优先把文件操作、阅读摘录、知识整理、创作开发三类流程做实。不能用“窗口能打开”代表应用可用，也不必为每个桌面图标复制一个完整商业产品。

建议采用三种交付方式：**原生产品化**（Auto 的交互与领域逻辑）、**复用成熟引擎**（排版/解码/索引等）、**集成外部应用**（浏览器/办公等）。这三者都属于 AutoOS 支持应用，发布时明确标注方式和范围。

第一批研究和交付候选：文件管理器、Launcher、Notes、Reader；已有 AutoShell、AutoMusk、编辑器/Jade 方向保留原主线，本文不重新裁定“四大主打”的完整名单。音乐作为已有可用基础上的日用样板。待办、日历、图库、视频在共同能力明确后推进。Blog 已在既有路线中，保留一个“本地草稿→预览→导出”闭环，不随此建议撤项。

这里的“最佳设计”指**适合 AutoOS 目标用户、可维护且可验证的设计组合**。候选方案要经源码核查、真实任务试用和 AutoUI 双端探针后再冻结；初期研究不能证明某款产品客观最好。

## 阅读入口

| 文件 | 回答的问题 |
|---|---|
| [基础能力两类盘点](basic-capability-audit-20261029.md) / [证据快照](basic-capability-audit-20261029.json) | 哪 19 个已有基本实现，哪 9 个先补基本应用？各自最低合格线、源码证据和缺口是什么？ |
| [基本可用与 Linux 首批排序](linux-mvp-priorities.md) | 哪四个应用先做？28 个入口如何分批？Linux 哪些风险应提前验证？ |
| [应用组合与优先级](portfolio.md) | 当前 28 个注册入口如何处置？下一版自研什么、集成什么、分几批？ |
| [文件管理器](file-manager.md) | Strata / Dolphin / COSMIC Files / Yazi：哪些设计值得吸收，如何接入现有 Explorer？ |
| [阅读器](reader.md) | Readest / Foliate / KOReader / calibre：阅读、标注、书库如何组合？ |
| [知识与工作管理](knowledge-productivity.md) | Notes、Todo、Calendar、Kanban、Blog、Chat 的市场、设计与实现候选 |
| [媒体与创作](media-creative.md) | Music、Video、Photos、Paint 的日用边界及引擎路线 |
| [系统与开发工具](system-developer.md) | Shell、Term、编辑器、Musk、设置、系统工具和轻量应用 |
| [实现路线与验证](implementation.md) | 共同能力、双端接缝、实现选项、依赖、可行性探针与发布门 |
| [研究方法与后续任务](methodology.md) | 如何持续补充、评价竞品，把研究转为可执行 Plan？ |
| [来源索引](sources.md) / [结构化目录](sources.json) | 来源、核查日期、源码抽样版本、许可与证据深度 |

## 研究口径

- **事实 A**：官方产品页、仓库描述或 README 声明的能力；没有独立验证其效果。
- **事实 B**：抽样读到的源码、依赖清单、类型或许可文本；只覆盖所读部分。
- **实测 C**：安装竞品或运行 AutoOS 探针的可复现证据。本轮尚无新增 C 级证据。
- **建议 D**：基于上述材料的产品判断、设计组合和实现假设，均需后续验证。

本轮覆盖全部 28 个 manifest 入口，并补充 AutoShell、知识/编辑器、浏览器/办公等必要产品方向。竞品来源包括 62 个 GitHub 仓库及官方产品/标准文档；阅读器、文件管理器做了源码入口抽样，其余主要为功能与架构初筛。没有收集市场份额、付费转化、真实留存或可靠的行业规模数据；“市场情况”是产品分层、用户需求与进入空间分析，不伪造排名。

## 与仓库规范的关系

`apps.manifest` 仍是运行注册事实源；本库不增删入口、不改端口、不改 app 名称。`docs/specs/` 描述已实现行为；本库描述调研与目标设计，不能把本库直接写入 current-state Spec。进入实现时遵循 auto-plan 流程，在所属 `apps/<目录>` 工作；框架缺口另在 auto-lang 立项。本轮仅文档研究，沿仓库既有战略参考文档方式保存，不分配实施 Plan 号，不改既有 Plan 状态。

关联：[既有应用/系统基础设施分期](../../design/strategy/v0.6-foundations-roadmap.md)、[通信 RFC](../../design/strategy/auto-app-communication-v0.6.md)、[桌面组织参考](../../design/strategy/desktop-organization-deskbox-reference.md)、[App 子模块工作流程](../../design/strategy/app-submodule-workflow.md)。
