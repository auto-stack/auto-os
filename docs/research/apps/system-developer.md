# 系统与开发工具应用

日期：2026-10-22。公开产品/README 为 A，局部许可为 B；本文设计为 D。AutoOS 当前能力以已实现 Spec 和实际 app pin 为准。

<a id="launcher"></a>
## 1. Launcher：统一检索与动作入口

**市场**：传统启动菜单、键盘启动器和可扩展命令面板逐渐覆盖同一入口。商业 [Raycast](https://www.raycast.com/) 官方展示搜索、扩展与 AI 等；开源 [Flow Launcher](https://github.com/Flow-Launcher/Flow.Launcher)（MIT，Windows/.NET）强调文件/应用搜索与插件；[Vicinae](https://github.com/vicinaehq/vicinae)（GPL 元数据，C++）README 列出文件、剪贴板、窗口、计算和 React/TS 扩展；[Albert](https://github.com/albertlauncher/albert) 是键盘启动器候选，许可聚合为 NOASSERTION，后续逐组件核对。

**取其优点**：Raycast 的结果→动作可发现性、Flow 的 provider 分工、Vicinae 的桌面入口整合值得研究。扩展生态意味着版本/权限/维护工作，不能照搬一串插件作为首版承诺；已知生态兼容也不等于任何插件在 AutoOS 可运行。

**综合设计**：先 app、文件、命令、计算、Notes 捕获五类 provider；结果含类型、来源、匹配理由、次动作与可用状态。支持中文/拼音是否有价值要用真实查询验证；近期/频次可辅助排序，但危险动作不因高频就自动执行。搜索与调用分开：读到结果不是授权发布/删文件/执行任意代码。

**实现路线**：AutoUI 界面 + 可取消并发 provider + 有限 top-k 聚合 + query generation 丢弃过时结果；文件名索引与全文索引分层，统一按身份/权限暴露小投影。app 动作沿已有通信 capability/receipt 设计；尚无通信层时可显式 open_with 或导出/import。剪贴板历史默认不开启全量采集，范围/保留/排除项由产品合同明确。

**门**：快速连续输入、慢 provider/崩溃/无权限、失效文件、中文/同名结果、键盘/鼠标同任务、重复动作与退出后后台工作。测首个有用结果和最终结果 p50/p95，而非仅测过滤函数；验证 stale query 不能污染新结果。

<a id="terminal-shell"></a>
## 2. AutoTerm 与 AutoShell

**产品分工**：AutoTerm 是终端模拟/会话与交互表面；AutoShell/ASH 是命令、结构化管道与脚本；桌面 Shell 是窗口/工作区/任务栏。三者分别设计、分别验收。

| 开源参考 | 优点/事实 | 应吸收的原则与边界 |
|---|---|---|
| [WezTerm](https://github.com/wezterm/wezterm) | Rust 的终端与 multiplexer；主体 MIT 的 LICENSE 已读，字体等另许可 | 研究终端/会话分层、跨平台集成；Rust crate 不自动具备稳定嵌入合同 |
| [Ghostty](https://github.com/ghostty-org/ghostty) | MIT 元数据；README 描述原生 UI/GPU 与 libghostty 嵌入库 | 可做终端引擎比较候选；各平台支持和 API 稳定性需针对所需版本实测 |
| [Nushell](https://github.com/nushell/nushell) | MIT；结构化 pipeline、插件/数据处理的 README 入口 | 学习类型流/表格操作；不要求兼容所有 Nu 语法 |
| [fish](https://github.com/fish-shell/fish-shell) | README.rst：补全、建议、高亮与低配置成本；COPYING 以 GPLv2 为主并含其他许可 | 学习可发现/可纠错的交互；不直接复制 GPL 组件为宽松许可代码 |
| [PowerShell](https://github.com/PowerShell/PowerShell) | MIT；README 明确跨平台自动化、结构化数据/对象模型 | 学习对象管道、模块和管理接口；不把“有结构化数据”说成 ASH 独有能力 |

[AutoShell 当前 README](https://github.com/auto-stack/auto-shell) 已记载 ash-core/命令层/终端或 GUI 的分层、结构化管道、Auto 脚本与 Agent CLI。它也有 marketing 对照表；本文不复述其中把 PowerShell 当纯文本 shell 的不准确简化。用户给出的“综合多家优点”的方法适用，差异应由任务验证。

**综合设计**：保留 ASH 结构化管道与 Auto 语言优势；外部程序文本流、stderr、退出码与取消保持真实。AutoTerm 把可靠 PTY、中文 IME/组合字符/宽字、选择复制、滚屏、resize、会话退出作为基础；分屏/标签/远端在基础之上。终端快捷键与运行程序快捷键边界明确。

**实现路线与门**：先审计 `apps/auto-term` 既有引擎与会话，避免为追热门替换已验证内核。单独比较 libghostty/既有引擎的嵌入、许可、平台覆盖与维护成本。验证交互程序、全屏 TUI、ANSI/颜色、中文与 emoji、长输出/背压、Ctrl+C、子进程退出/孤儿清理；ASH 验证结构化类型往返、错误/取消、路径/编码与外部命令互通。两者性能与数据正确性分别报告。

<a id="flagships"></a>
## 3. Musk、AutoEdit、AutoJade（旧称 Jade Garden）

**市场**：代码编辑、AI 编程执行、文档知识库相互协作，但不是同一应用。[VS Code](https://code.visualstudio.com/) 是编辑/扩展生态参考，产品分发许可与源码需区分；[Zed](https://github.com/zed-industries/zed) README 描述高性能多人编辑，仓库聚合许可不能当统一 MIT；[Helix](https://github.com/helix-editor/helix)（MPL-2.0 元数据）提供多选区、LSP、Tree-sitter 的可组合编辑参照。[Codex CLI](https://github.com/openai/codex) 与 [Aider](https://github.com/Aider-AI/aider)（均 Apache-2.0 元数据）是 Agent 的公开实现参照，不能据开源 CLI 推导相关托管产品全部开源。

**本地事实**：`D:/autostack/auto-edit/specs/auto-edit/README.md` 记载多 tab、动作/快捷键、打开保存/脏文档、原生编辑接缝及框架限制；`D:/autostack/jade-edit/README.md` 定位 AutoJade 文档与本地知识库；`D:/autostack/auto-down/jade-garden/README.md` 记载基于 AutoDown 的知识编辑器及 Web/VM 两种形态。上述是只读定位，不按这些物理兄弟路径启动新开发，不迁移或切换他人 WIP。

**设计组合**：AutoEdit 专注可靠代码/文本编辑、语言诊断和运行；Musk 承担变更计划、执行、验证与沉淀；AutoJade 承担文档与知识关系，Jade Garden 是其旧名（2026-10-29 用户确认），只计一个产品。两个历史接入位置的兼容/迁移仍需核对，本次不修改入口或数据。Notes 为快速捕获面而非再造完整编辑器。

**Musk 的进入空间**：已有 [README](https://github.com/auto-stack/auto-musk) 强调 Plan+Spec、分阶段工作和可见验证，比复制一个聊天面板更有价值。Aider 的仓库上下文/Git 操作、Codex CLI 的本地 Agent 工程边界可继续深读；本轮尚未做算法/执行可靠性比较。下一步应比较同一小变更的补丁正确率、恢复能力、成本与人参与次数，而非比较支持多少模型。

**实现路线与门**：沿现有 auto-ai 模型/工具设施、AutoEdit 编辑接缝和 AutoDown 文档引擎；共享项目身份、内容引用与变更回执，不共享未经合同定义的内部表。Musk README 明确运行中 Relay 状态目前在内存、重启不恢复，此项是现实边界；如果下一版要承诺恢复需独立持久状态方案。验收打开→编辑→保存冲突/崩溃恢复，Plan→代码→实际测试→Spec 沉淀；权限/中途退出/模型失败不被当成功。

<a id="system"></a>
## 4. Settings、Monitor、Syslog

**市场与参照**：系统设置的价值在可理解/可还原，而非菜单数量；Monitor 对照任务管理器类产品，开源 [btop](https://github.com/aristocratos/btop)（Apache-2.0）README 提供进程排序/过滤/树、资源图与进程操作；Syslog 更适合与现有[系统 DevTools](../../design/strategy/system-devtools-v0.6.md)衔接。GNOME 的[应用目录](https://apps.gnome.org/)与 [XDG Desktop Portal](https://flatpak.github.io/xdg-desktop-portal/docs/) 可作系统整合参照；不是把另一套控制面板照搬进来。

| app | 综合设计 | 实现候选/关键门 |
|---|---|---|
| os-config | 搜索设置、解释范围/默认值、修改后生效状态、还原；模型/角色/技能延续既有职责 | 现有配置服务为真值；schema/version/原子写/错误恢复。无效配置不覆盖可用版本；凭据显示脱敏。Linux 系统权限改动经专门 adapter，不能写入应用设置就声称系统生效 |
| sys-monitor | CPU/内存/磁盘/网络/进程真实采样、趋势/排序/过滤、采样间隔；不可用有原因 | 复用现有系统采集模块；候选原生采样服务。识别 PID 重用、进程已退出/权限拒绝；终止前验证 PID+启动身份；图表不把缺失采样画成 0 |
| syslog | 按 app/任务/级别/时间筛选，关联 trace/receipt，有限保留，导出诊断 | app 日志→系统有限队列/轮转→投影；复用 DevTools schema 与脱敏，trace 缺失显式。验证日志洪水、丢失计数、磁盘满、敏感信息和导出可读性 |

后台任务、通知、快捷键与权限应由 OS/宿主能力供给；用户态 UI 不需要默认管理员身份。Linux Portal 用于对接宿主生态，现有 AutoOS broker 用于产品域调用，不能把二者当相互替代。

<a id="data-tools"></a>
## 5. Database 与 Charts

**Database 市场**：专业通用客户端与轻量 SQLite 浏览分层。[TablePlus](https://tableplus.com/) 是原生数据库 UI 商业对照；开源 [DBeaver](https://github.com/dbeaver/dbeaver)（Apache-2.0）README 覆盖 SQL/数据/模型、JDBC/ODBC 驱动，Java/Eclipse/插件体系；[DB Browser for SQLite](https://github.com/sqlitebrowser/sqlitebrowser) README 强调表格式浏览编辑、建表与导入导出，具体许可聚合未判定。前者生态广但维护面大，后者聚焦更适合首版。

**设计/实现建议**：先做好 SQLite 文件与只读查询，schema 树、SQL 编辑、分页结果、历史、CSV 导出；写操作进入显式事务，展示影响范围和提交/回滚。接现有 SQLite/Auto 标准库服务；跨数据库留驱动接口，PostgreSQL/MySQL/SSH 之后另选。查询异步取消、有行数/时间/内存预算；结果单元格类型/NULL/大整数不丢失。验证锁冲突、长查询、磁盘满、错误回滚、大表分页和导出一致性。只读开关需真实限制数据库连接而非仅隐藏按钮。

**Charts 市场**：可视化应用对照 [Datawrapper](https://www.datawrapper.de/) 的数据→映射→发布流程；[ECharts](https://github.com/apache/echarts)（Apache-2.0）和 [Vega-Lite](https://github.com/vega/vega-lite)（BSD-3-Clause）是开源**库/规格体系**，不能把它们算两个完整竞品应用。ECharts 图形能力丰富，Vega-Lite 声明式规格利于可复现和 Agent 调用；两者均有 Web 渲染接缝成本。

**设计/实现建议**：CSV/查询结果→类型和空值预览→图表字段映射→主题/标注→保存数据来源与规格→SVG/PNG/数据导出。首批柱/折/散点/表格；native 用 AutoUI 图表能力子集，Web 可评估库 adapter；如要一个规格双端执行，先固定公共子集和明确 unsupported。验证日期/小数/NULL/中文字段、坐标单位/截断、离线重开/导出；不要做只有随机 demo 数据的图表画廊。

<a id="utilities"></a>
## 6. Calculator、Clock、Weather

**市场**：高频但低复杂度的系统入口，可信赖比“功能全家桶”更有价值。每类选一个成熟开源实现与系统快捷入口做初筛，后续再补同平台横评。

| app | 开源/官方参考及优点 | 综合设计与实现候选 | 必须验证 |
|---|---|---|---|
| Calculator | [Qalculate/libqalculate](https://github.com/Qalculate/libqalculate)，GPL-2.0 元数据；README：单位、精度、函数/符号计算、CLI/库 | 先表达式/括号/百分比/单位、结果历史/复制；与 Launcher 共享计算内核；库绑定或受控 CLI 为候选，是否值得引入完整依赖先探针 | 小数/负数/除零/超范围/单位不兼容、表达式解析而非任意代码 eval；汇率若加则标日期与离线状态 |
| Clock | [GNOME Clocks](https://github.com/GNOME/gnome-clocks)，GPL-2.0 元数据；README：世界时钟/闹钟/秒表/计时器 | 沿[已有 Clock Plan](../../plans/archive/003-clock-app.md)增强；秒表用单调时钟，闹钟用时区/日历；提醒由后台服务或宿主接缝负责 | 时钟回拨/夏令时/睡眠/重启、窗口关闭后的提醒语义、重复触发；不把 UI Tick 当可靠系统调度 |
| Weather | [GNOME Weather](https://github.com/GNOME/gnome-weather)，GPL-2.0 元数据；[Open-Meteo](https://open-meteo.com/en/docs) 为数据 API 参考 | 城市搜索/定位选择、当前/逐时/多日、单位、来源与更新时间；provider + 缓存，不与 UI 绑定单一服务。API 数据/商用条款不等于代码许可证 | 时区/单位/数据过期/无网/请求失败、城市歧义；地理位置由用户选择，不默认后台精确采集。服务配额/地区覆盖另审 |

<a id="games"></a>
## 7. Tetris、Klondike、Minesweeper

**市场**：轻休闲适合验证交互与完整规则，需求与商业游戏发行不同。已有三个 app 的计划/Spec 应优先尊重；不要重新按静态 Demo 估算。[GNOME Mines](https://github.com/GNOME/gnome-mines)、[Aisleriot](https://github.com/GNOME/aisleriot) 是成熟规则/辅助功能的后续参照；[2048](https://github.com/gabrielecirulli/2048) 是开源轻游戏结构对照，不是 Tetris 的规则替代。Aisleriot 聚合许可显示 CC0，本轮未取得游戏统一许可，不将其推广到所有代码/牌面。

**综合设计**：保留完整规则、键盘/鼠标、暂停/重启、撤销（适用时）、存档和战绩；无网络亦可玩，清楚显示胜负。规则引擎与 UI 分离，确定种子/局面方便复现。Tetris 的规则与图形/名称资产独立确认来源，不能借一个别的开源游戏许可证推导。

**门**：沿现有 [Tetris](../../specs/apps/tetris.md)、[Klondike](../../specs/apps/klondike.md)、[Minesweeper](../../specs/apps/minesweeper.md) 的验证范围，补暂停/恢复、输入焦点、保存损坏、DPI/主题/可访问性真实场景。没有新规则变更就不为本轮文档重跑整套应用测试。
