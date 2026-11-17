---
plan_id: PLAN-064
status: executing
feature_name: ACE桌面宿主与全应用分批迁移
plan_revision: 6
author: [Codex]
created_at: 2026-11-07
updated_at: 2026-11-17
current_step: 1
total_steps: 97
depends_on: [auto-lang/PLAN-959, auto-lang/PLAN-969, auto-lang/PLAN-970, auto-lang/PLAN-971, auto-lang/PLAN-973, auto-lang/PLAN-974, auto-lang/PLAN-975, auto-lang/PLAN-978, auto-lang/PLAN-980, auto-lang/PLAN-981, auto-lang/PLAN-982, auto-lang/PLAN-983, auto-os/PLAN-054, auto-os/PLAN-063]
supersedes_spec_components: [docs/specs/shell/desktop-app-launch.md, docs/specs/apps/independent-repositories.md]
new_spec_components: [docs/specs/shell/ace-desktop-and-app-compatibility.md]
touched_goals: []
affects: [desktop-host, app-assembly, ace-migration]
---

# [PLAN-064] ACE桌面宿主与全应用分批迁移

## 0. 变更摘要

2026-11-12 r5：按用户要求以963同等标准补本包具体输入/独立预期、类型与代码职责、旧AutoVM/A2R及真实消费者到ACE验证映射；原任务/AC/V/依赖/预算/阶段与进度保留。本轮只修订规划，不实施/建树/生成fixture/运行产品；旧历史记录不作当前完成证明。

历史修订：2026-11-10 r4当前执行版：跨M2-A/B。本轮把阶段安排落实到本Plan的开工前置、执行顺序和A/B交接；原任务/AC/范围保留，所有任务仍未执行。以§5的最小接口和§8的分阶段清单为准，不按后文历史轮次的旧总顺序施工。

历史修订：2026-11-09 r3当前：用户正式拆M2为A语言/新库完善、B全产品替换；本包归属**跨M2-A/B**，以§5阶段合同为当前启动/验收边界。旧日期范围/义务不删，所有任务仍未执行；仅规划，不授权施工。

由语言[PLAN-984](../../../auto-lang/docs/plans/984-ace-m2-os-apps.md)拆出OS域实施正文。HOST、28个单App、ASSEMBLY及A期PROOF共31个可独立实施/验证/复核批次，97任务，全部未执行。语言984保通用嵌入/非桌面消费者/全分母交接；OS054/063原合同不变。
历史修订：2026-11-09 r2：用户明确AGE M2结果须可用ACE直接承接AGE游戏、传统App及AutoOS桌面本身。本包补桌面`.at`/普通Node根的AV与A2R真实组合验收，复用原HOST/ASSEMBLY，新增AC-SHELL-AGE/V-SHELL-AGE；不新增Plan或任务，仍drafting、0/94，仅规划。

2026-11-16 r6当前：HOST/PROOF A只交MIN-A-OS真实启动/激活/窗口链的Node Hello/Counter隔离见证；原shell/Notes/Clock全旅程、28项App、完整Node桌面与双Host/模式迁到三波后OS波。97任务/0完成保持，用户仅授权规划。

## 1. 目标

现有桌面/系统宿主及全部实际应用在ACE下保持原行为、数据与支持模式，生产调用无旧VM/A2X回退。28条是当前apps.manifest快照，不是全生态最终上限；959发现新增可达产品必须追加具名批次、owner与AC，不能以manifest外免验。Reader不因OS054局部排除而退出M2。

非目标：新增业务、新平台、重写compositor、替代UI/AGE底层实现、提前发布M2或批量升级非目标gitlink。本轮仅计划，不实施/测试/启动桌面/提交/推送。

## 2. 架构方案

宿主消费969/970公开执行session、UI980/AGE981领域接口；OS持启动策略/注册表/窗口/daemon组合，不自行实现共享语言语义。App先在原归属仓验证，再固定来源组装候选，最后验真实桌面。frontmatter为最终依赖并集，各批按下文最小前置推进。

OS064不依赖984整包或985–987；984/CLOSURE依赖OS064正式交付。HOST直接接969/970/978/980已交接口，避免984↔OS循环。981只约束使用AGE的产品与最终组合，不阻普通App批次。

## 3. 技术栈

现有host/desktop、Vue桌面、UI direct/适用RQ、AutoMan与各App原栈。OS唯一树拟为D:/autostack/.wt/os-064/auto-os，分支codex/plan-064-dev；其他引擎仓消费各自Plan唯一树/显式SDK，不建第二权威树或junction/symlink。

App按OS AGENTS的独立clone主分支规约在D:/autostack/<app>实施，先核当地AGENTS/在途计划/写窗；本计划登记迁移子任务，超出迁移的业务变更另立当地合同。复用clone、不覆盖WIP；apps子模块只作固定组合，local syslog在OS专属树。

## 4. 需求分析与背景调查

本次授权只含设计、路线与相关Plan修订，未含源码/测试实现、生成fixture/SDK、构建、产品运行或merge。阶段依据为用户本轮正式决定；取材当前959冻结分母、TestCase/固定组/ACE源码重放规范及现存owner入口，旧overview的物理路径/别名不覆盖当前Cargo。

2026-11-07用户授权细化977/980–984，必要时拆计划，先不施工。OS AGENTS明确桌面/应用组合计划归OS取号；scripts/new-plan.sh已原子领取064。读取apps/independent-repositories.md、shell/desktop-app-launch.md、apps.manifest、host/desktop/Cargo.toml、scripts/desktop.ps1及check-app-submodules.ps1。OS无docs/specs/overview.md；旧launch Spec的auto-lang物理路径落后于UI迁仓，交付时按真实owner更正Spec，不以旧路径伪造新实现。

调查OS HEAD b4ccc8dbf6cdfee8d12b5711d0c3b9a5b8266a56，manifest28条。OS054在途7/10，其八App冻结基线保留；OS063 r3负责RQ双Host原能力，本包只承ACE迁移后受影响重验。既有主题/游戏义务保留，主题烟测不充业务验收。
上段为11-07历史快照。11-09用户进一步明确传统AutoUI/完整AGE语法及桌面产品交付目标，本轮只修订本包与984/领域路线，未授权实现/启动桌面/构建/测试。实际补读`host/desktop/src/`、`shell/{shell,desktop,switcher,notification_center,dashboard}.at`及`shell/desktop-app-launch` Spec；桌面UI已是特权AutoUI pack，不可将28 App数量当桌面源码已迁证据。

### 2026-11-12 本轮规划取材与授权

用户要求细化其余ACE M2计划，复用已有研究/合同，仍只规划；本轮不创建实施树/集成分支、不执行T-01探针或产品/正式性能测试、不提交/合入/tag/发布、不改canonical Specs或legacy索引。945–948原M1-TAG及各A/B正式来源门均不因本次文档静态取材放行；既有实施证据/暂停/等待由原owner维护。

取材语言主线读取基面 `f9fb3854beb3886338156cfa61d16f7f84893cc9`（不是所有外仓同SHA；各源hash如下，开工重核正式来源）；本包焦点：真实桌面/28App与普通Node根迁移。已读当前规范入口：[auto-os/docs/specs/shell/desktop-app-launch.md](../specs/shell/desktop-app-launch.md)。下列是已核路径/结构的规划取材，不宣称逐符号全审或运行通过；旧函数同名/目录存在/历史组绿都不等于新ACE支持。

- [auto-os/host/desktop/src](../../host/desktop/src)：本仓独占桌面/窗口/启动/系统接线；目录职责已核，确切符号/闭包开工再核

- [auto-os/shell/desktop.at](../../shell/desktop.at)：真实桌面pack/通知/图标协议源；读取SHA256前16 `376b156d4f458699`

- [auto-os/apps.manifest](../../apps.manifest)：注册分母快照，新增consumer需补而非排除；读取SHA256前16 `e7409f924090700b`

本轮用户授权：仅修订现有980、981、1068、OS064、985–987及相关路线，不新立项、不启动实施。采用A最小真实交付→985/A M2A-08→B完整UI/Widgets的AV→AC→A2R三波→AutoOS全量迁移→候选/安装/退役。读取语言主线aa257fa1425c31fab48bd544cbd116b2fdce7dcd及现有Specs/已交HELLO、AXR0、SOURCE；UI示例真实源在auto-ui/examples/ui，Widgets源在auto-os/widgets-gallery/src/front/{pages,components}，ui-gallery副本需和规范源去重。新增接面/用例/收据仅是合同，不代表已运行。语言/标准库/非图形FFI的既定基础与测试守恒仍保留，不能借UI延期把普通语言必需能力全部后移。

OS域唯一主责仍为本Plan；App改动只在原独立clone，组合gitlink更新沿本仓AGENTS，不在UI波的测试副本里冒产品迁移。

## 5. 详细设计

### M2阶段执行合同（本轮当前）

| 阶段 | 唯一任务归属/交付 |
|---|---|
| A准备 | T-01冻结实际登记App/host/源码/原oracle与A/B承接，不改生产manifest/gitlink |
| A最小 | HOST I/V/R与PROOF I/V/R各自MIN-A-OS部分；正式phase交IF-OS064-A-MIN给985/A，原任务全量部分不提前勾 |
| B-OS | UI-W1-AV→UI-W2-AC→UI-W3-A2R正式齐后，HOST/PROOF完整余量、A01..A28、ASSEMBLY与原T-05/06/07；实际登记增量有owner/源/期望不漏 |

A只需980 MIN-A-UI、969/970/978及真实所用库/宿主接口，不等980全包、981完整LANGUAGE/RUST/PARITY、28App或完整OS054业务。现有OS054/063所用平台/协议约束保留，不把完整产品迁移置为最小见证前置。完整OS原required义务全部留B，原独立oracle不缩；一Plan一仓唯一tree保留至整包完成，分期交接不是归档。

#### MIN-A-OS真实见证与精确迁交

真实OS启动/注册/activate/窗口会话服务通过隔离候选登记980最小Node Hello与Counter（拟测试App ID，由B-PROOF-I在既有候选注册表冻结；不新增生产App），经真正AV UI-FFI/Iced输入、双实例隔离、稳定ID激活与关闭回收；错SDK/ABI/协议/授权拒绝，不fallback旧VM。保持源码/producer/host来源可核，不以DryRun/空host/额外Rust小窗或进程存活通过。A不要求新桌面外观、完整shell Node根、Notes/Clock全业务/持久化/完整双Host/A2R。

旧A要求逐项迁交：Notes保存/搜索/重开/坏写保旧→原B-A15-I/V/R；Clock启动暂停恢复/timer释放→B-A12-I/V/R；原完整shell与桌面Node根/28App/双Host/配置/daemon/原模式→B-HOST完整余量＋B-ASSEMBLY-I/V/R；桌面AV/AC/A2R真实guest与数据/包身份→V-SHELL-AGE及最终ASSEMBLY。现有Widgets/ui-gallery的组件/示例由980 UI波以原源消费，两个完整gallery容器、桌面注册/导航/安装来源归本包ASSEMBLY，不重复一套组件实现或要求OS先迁完才能签W1。

OS波开始先消费三波正式来源、源/SDK/ABI与实际后端选择；按原A01..28最小业务依赖迁移，可成组安排但原每App I/V/R与数据/资源oracle保持。桌面公共Node组件/三端host先准备就绪，再逐组App，最后所有R与原额外登记项齐→ASSEMBLY；各App按本版公开支持模式实际验，不能用“本App只跑AV”隐藏公共三端能力缺失。OS产品迁移不另加“每App必须同时实现三端”的门，公共桌面根/能力三端与App声明模式分别核。

不等待985/B最终签或986/987安装；OS交产品候选结果给984 CLOSURE/985 B，986再验安装、987退役。不能形成OS→985最终→OS或UI波→OS全量→UI波环。

### HOST：宿主与启动服务

最小前置：L969/L970/L973/L978、980/STATE及所验NATIVE或WEB模式；RQ另需UI061–063/OS063适用正式接口。改面：host/desktop/src/main.rs、host/desktop/Cargo.toml、scripts/desktop.ps1、apps.manifest，UI注册表/session实现仍归980。替旧core执行依赖、host install hook/storage调用、App factory和daemon生命周期；launch=vm可保兼容名字，但dispatch须ACE。冻结源/候选SDK/host/协议版本，错二进制拒绝，失败不回退旧VM。

独立oracle：真desktop打开两个隔离App，不串store/焦点/任务；关闭重开、daemon重连、错版本拒绝，主题/pin/launch ID保持。HOST用一个已支持原App作host fixture，无需先等28项全迁完。

桌面作者层与原生宿主分工：上述shell/desktop等`.at` pack及其props/model/on/绑定/主题/通知/切换业务进入980/981共同前门；OS仍持窗口/系统服务/启动与授权边界，不能用Rust重写桌面业务来假称AGE执行。原widget源可走正式兼容投影，不强制全文件改Node；ASSEMBLY另用普通AGE Node桌面根复用这些真实组件/协议证明新作者入口，保持原功能，不做新桌面设计。HOST基础接线不等待28 App/完整Node终验，避免阻塞并行。

### 单App批次

每行Axx独立I实现→V验证→R复核；family仅排程分组。所有行做冷启动→表内操作→适用持久化重开→坏输入/失败更新恢复→关闭资源；旧数据不可删除。原支持矩阵继承959，加入已承诺RQ格；非适用须有原合同依据，缺环境为未执行。

| 批次/manifest ID | owner与开发来源 | OS组合路径 | 最小前置 | 独立操作oracle | 现有测试种子（相对开发来源） |
|---|---|---|---|---|---|
| A01 / kanban (DATA) | auto-kanban：`D:/autostack/auto-kanban` | `apps/auto-kanban` | HOST；980适用模式/978；L973/L975适用库 | 新建/移动卡片、列顺序、刷新与旧数据重开 | `tests/run.mjs` |
| A02 / auto-musk (SERVICE) | auto-musk：`D:/autostack/auto-musk` | `../auto-musk` | HOST；980适用模式/978；L974/L975及983/CONSUMERS服务腿 | 隔离daemon读取真实计划、修改隔离记录后重开，不改生产任务 | `tests/musk-ui-smoke.sh` |
| A03 / jade-garden (SERVICE) | auto-down：`D:/autostack/auto-down` | `../auto-down/jade-garden/front/auto` | HOST；980适用模式/978；L974/L975及983/CONSUMERS服务腿 | 文章/集合读写、真实后端错误与重连、关闭daemon回收 | `jade-garden` |
| A04 / auto-term (SYSTEM) | auto-term：`D:/autostack/auto-term` | `apps/auto-term/app` | HOST；980适用模式/978；L973/L975适用库 | PTY输入输出、中文/resize、双终端隔离与关闭子进程 | `拟补永久产品回归（本轮未创建）` |
| A05 / 037-klondike (GAME) | auto-solitaire：`D:/autostack/auto-solitaire` | `apps/auto-solitaire` | HOST；980适用模式/978；980/STATE/NATIVE及原生成腿980/RUST | 固定牌局发牌/合法非法移动、撤销重启，牌面/得分独立oracle | `tests/rules_golden.cjs` |
| A06 / 036-tetris (GAME) | auto-tetris：`D:/autostack/auto-tetris` | `apps/auto-tetris` | HOST；980适用模式/978；980/STATE/NATIVE及原生成腿980/RUST | 固定序列移动/旋转/消行、真实键盘焦点与停止tick | `tests/gallery_contract.py` |
| A07 / 038-minesweeper (GAME) | auto-minesweeper：`D:/autostack/auto-minesweeper` | `apps/auto-minesweeper` | HOST；980适用模式/978；980/STATE/NATIVE及原生成腿980/RUST | 固定雷盘揭开/标记/胜负/重开，坏动作不改变棋盘 | `tests/rust_smoke.py` |
| A08 / 028-launcher (SYSTEM) | auto-launcher：`D:/autostack/auto-launcher` | `apps/auto-launcher` | HOST；980适用模式/978；L973/L975适用库 | 搜索/点选真实App、稳定ID单实例与失败启动恢复 | `tests/click_input_smoke.mjs` |
| A09 / 039-syslog (SYSTEM) | auto-os(local)：`D:/autostack/auto-os/apps/auto-syslog` | `apps/auto-syslog` | HOST；980适用模式/978；L973/L975适用库 | 日志订阅/过滤/滚动、断流重连与窗口关闭取消 | `拟补永久产品回归（本轮未创建）` |
| A10 / jade-edit (EDIT) | jade-edit：`D:/autostack/jade-edit` | `../jade-edit` | HOST；980适用模式/978；L973/L975适用库 | 打开/编辑/保存/草稿恢复，坏保存保旧文件，多文档隔离 | `tests/draft_recovery_matrix.mjs` |
| A11 / 011-calculator (BASIC) | auto-calc：`D:/autostack/auto-calc` | `apps/auto-calc` | HOST；980适用模式/978；L973/L975适用库 | 输入/优先级/清除/错误恢复，数值精度按原oracle | `tests/acceptance.atd` |
| A12 / 012-clock (BASIC) | auto-clock：`D:/autostack/auto-clock` | `apps/auto-clock` | HOST；980适用模式/978；L973/L975适用库 | 时间/计时启动暂停恢复，关闭窗口释放timer | `tests/t3_smoke.py` |
| A13 / 013-todo (DATA) | auto-todo：`D:/autostack/auto-todo` | `apps/auto-todo` | HOST；980适用模式/978；L973/L975适用库；OS054该App冻结基线 | 增改完成/删除、筛选与旧任务持久化重开 | `tests/run_autotest.py` |
| A14 / 014-weather (SERVICE) | auto-weather：`D:/autostack/auto-weather` | `apps/auto-weather` | HOST；980适用模式/978；L974/L975及983/CONSUMERS服务腿 | 地点查询、真实HTTP失败重试与late响应隔离 | `tests/vm_smoke.py` |
| A15 / 015-notes (DATA) | auto-notes：`D:/autostack/auto-notes` | `apps/auto-notes` | HOST；980适用模式/978；L973/L975适用库；OS054该App冻结基线 | 编辑/搜索/保存/重开，旧便笺可读、坏写保旧 | `tests/acceptance.atd` |
| A16 / 016-calendar (DATA) | auto-calendar：`D:/autostack/auto-calendar` | `apps/auto-calendar` | HOST；980适用模式/978；L973/L975适用库；OS054该App冻结基线 | 跨月/闰日、事件增改删除与持久化恢复 | `tests/basic` |
| A17 / 017-chat (SERVICE) | auto-chat：`D:/autostack/auto-chat` | `apps/auto-chat` | HOST；980适用模式/978；L974/L975及983/CONSUMERS服务腿；OS054该App冻结基线 | 双会话消息/重连/旧历史，断线和关闭后late消息隔离 | `tests/acceptance.atd` |
| A18 / 018-book-reader (EDIT) | auto-reader：`D:/autostack/auto-reader` | `apps/auto-reader` | HOST；980适用模式/978；L973/L975适用库 | EPUB装载/翻页/目录/批注保存重开、坏书恢复 | `tests/epub` |
| A19 / 023-realworld (SERVICE) | auto-blog：`D:/autostack/auto-blog` | `apps/auto-blog` | HOST；980适用模式/978；L974/L975及983/CONSUMERS服务腿；OS054该App冻结基线 | 真实文章读写/转义/评论原支持格，会话与失败事务 | `tests/byte_text_roundtrip.py` |
| A20 / 030-video-player (MEDIA) | auto-video：`D:/autostack/auto-video` | `apps/auto-video` | HOST；980适用模式/978；L973/L975适用库 | 真实媒体播放/暂停/seek/结束、关闭释放解码和音频资源 | `tests/package.json` |
| A21 / 020-music-player (MEDIA) | auto-music：`D:/autostack/auto-music` | `apps/auto-music` | HOST；980适用模式/978；L973/L975适用库 | 播放/暂停/切曲/seek，列表恢复与关闭音频资源 | `tests/test_020_vm.py` |
| A22 / 029-photo-gallery (MEDIA) | auto-photos：`D:/autostack/auto-photos` | `apps/auto-photos` | HOST；980适用模式/978；L973/L975适用库 | 目录图片装载/切换/缩放、坏图恢复与旧主题数据 | `tests/test_058_theme.py` |
| A23 / 031-paint (EDIT) | auto-paint：`D:/autostack/auto-paint` | `apps/auto-paint` | HOST；980适用模式/978；L973/L975适用库；OS054该App冻结基线 | 绘制/撤销/保存重开，独立像素oracle与坏保存保旧 | `tests/basic` |
| A24 / 025-sys-monitor (SYSTEM) | auto-monitor：`D:/autostack/auto-monitor` | `apps/auto-monitor` | HOST；980适用模式/978；L973/L975适用库 | 真实采样刷新/图表，取消采样与两实例隔离 | `tests/package.json` |
| A25 / 026-database (DATA) | auto-database：`D:/autostack/auto-database` | `apps/auto-database` | HOST；980适用模式/978；L973/L975适用库；OS054该App冻结基线 | 隔离库schema/查询/事务、坏SQL回滚与旧连接配置 | `tests/basic` |
| A26 / 027-file-manager (SYSTEM) | auto-explorer：`D:/autostack/auto-explorer` | `apps/auto-explorer` | HOST；980适用模式/978；L973/L975适用库 | 隔离目录列举/复制移动/重命名、权限拒绝保源文件 | `tests/plan023_check.py` |
| A27 / 024-charts (DATA) | auto-charts：`D:/autostack/auto-charts` | `apps/auto-charts` | HOST；980适用模式/978；L973/L975适用库；OS054该App冻结基线 | 原数据装载/系列切换/绘图，导出与坏数据恢复 | `tests/golden` |
| A28 / os-config (SYSTEM) | auto-os-config：`D:/autostack/auto-os-config` | `apps/auto-os-config/auto` | HOST；980适用模式/978；L973/L975适用库 | 隔离配置/主题/策略读写，拒绝非法值且保留原设置 | `package.json` |

测试种子仅证明已有入口，不表示覆盖上述全部oracle。各I读取原runner/README/package.json固定完整命令、隔离端口/数据与expected；缺业务断言补永久case再V。主题截图/首帧/存在进程不能代业务。auto-down区分前后端，musk只用隔离数据，不能写生产计划。缺provider能力交语言owner修复后回本批，不删业务绕过。

### ASSEMBLY：组合、MIX与候选交接

前置：HOST、A01–A28全部R通过；OS054/063和980–983适用正式来源。App先review/交付成可获取commit，OS才更新目标gitlink为detached同SHA；非目标pins固定，外部repo/local也逐行记身份。双Host沿OS063原合同；MIX-05用981融合场景，MIX-06用隔离安装SDK的文件/后台/动态配置真实App，验证同PID兼容更新、坏更新保旧、关闭资源归零。986/987负责最终安装发行，本包只交候选产品结果。

收据必须含repo/commit/Plan revision、原源hash、SDK/stdlib/host/模式、完整命令与子终态、旧数据fixture、独立expected、差额；来源变化使受影响收据失效。单批不单独归档，唯一树保留至全计划完成。

ASSEMBLY补桌面本体实证：消费980传统UI、981完整Node/融合正式来源，对同一桌面作者源/资源在AV开发运行和新A2R生成/编译的候选分别验证。普通Node桌面根及装配fixture由B-ASSEMBLY-I在本仓既有测试owner下新增，接原桌面组件/投影协议；不只开一个附带AGE窗口。真实验证桌面图标/启动器、dock/pin、通知/工作区/窗口切换、App启动/聚焦/关闭/重开、主题/配置和daemon/资源寿命；按既有功能实际分母核，不增新UX或删原功能。AV合法body更新无Cargo/rustc，A2R实际执行生成桌面逻辑、不内嵌VM解释该逻辑；错误源/错SDK拒绝，坏更新保旧、修复后同入口真实运行。

原28 App各自原模式/数据/业务门与双Host/direct/RQ条件仍全部保留；新Node桌面例不替它们，反之全部App绿不替桌面`.at`与Node入口。OS064收据由984总闭合核并交AGE M2的M2-OS-REAL联合门；980/981不反向依赖OS064终验，OS064也不等待AGE M2/ACE M2最终标签，防阶段环。


A期候选入口：PROOF优先消费978/980正式构建的显式CandidateDir与os-desktop真实host，I固定AV/A2R启动命令/源与SDK身份。实际需要最小CLI接线时只消费982/PROOF的A接口，不等982全目标命令或B安装；不得静默启动机器旧auto/旧VM以求桌面首帧。

### T-01 A最小冻结（执行口径，2026-11-17，goal"完成计划064/A"）

授权：用户 `/goal 完成计划064/A`（/auto-plan-goal 承接）= M2-A 组 work/独立复核/有界修复/A 期合入的授权；不含 B 波、不改验收阈值/范围。B 任务保持不勾（A独立R/fold不勾整任务），唯一树保留至整包完成。

- **唯一树**：`D:/autostack/.wt/os-064/auto-os` 分支 `plan-064-dev`（§3 的 `codex/` 前缀按本仓 worktree 规约收敛为 `plan-064-dev`，树路径不变；bounded 记录）；base=main@2383d72。依赖组兄弟（detached 消费，零修改；收敛为 **980 fold#7 交付时点状态**——desktop.ps1 `--locked` 组锁一致性裁定，非 master 追新）：auto-lang@7f431bd5f、auto-ui@403c936d、auto-web@0adebc0、auto-man@11fe79c、auto-down@5af11a3。裁定记录：ui/lang 后续 master（981/983 在途）使 man/ui 锁漂移，按"消费已交交付态"回 pin；后续批次如需追新由该批具名重钉。
- **A 前置核验（已核）**：969/970/973/974/975/978 已归档交；980 MIN-A-UI 已交（B-NATIVE-I/V/R ✅，phase fold #7 落主，spec §12）。981/982/983 非 A 前置。
- **源与协议身份冻结（blake3 hex，与 provider_digest 同族；sha256 参考：Hello `7f9f79a7…`、Counter `6016eb86…`）**：Hello 字面量 `auto-ui/examples/ui/ace-hello-minimal/src/front/app.at` = `a5fcb1d19fc72950…`；Counter `auto-ui/examples/ui/ace-node-counter/src/front/app.at` = `790d6fcc52634b5a…`；provider 头 `ui980_native/1 target=x86_64-pc-windows-msvc profile=ui-native.v1`（名 `p980_ui980_native`）；车道标签 `open/apply-int/apply-bool/poll/close`；打包位 `instance*1000+slot`（实例≤4、槽<1000）；引擎仅 `av`（`parse_ace_engine` 具名拒旧名）；ui980 状态码 OK=0/UNKNOWN_HANDLE=-1/SESSION_CLOSED=-2/BAD_SLOT=-3/BAD_OP=-5/HOOKS_MISSING=-100。源解析序：`AUTO_UI_ROOT` → 组兄弟 `../auto-ui` → 主检出 `D:/autostack/auto-ui`；失配/缺源具名拒。
- **改面（A）**：`host/desktop/src/main.rs`、新模块 `host/desktop/src/ace_min.rs`、`host/desktop/Cargo.toml`（feature `ace-min` = `auto-ui-runtime/ace-bridge` + `auto-lang-ace-host`，缺省关——关=CLI 具名拒，绝不落旧 VM）、`scripts/desktop.ps1`（`-ExtraArgs` 透传给 iced 轨）。`apps.manifest` 与生产 gitlink 不动（A）。
- **候选注册（B-PROOF-I 冻结）**：既有候选注册表=os-desktop 内冻结常量（非生产 manifest）：`ace.hello.min`→Hello 源、`ace.counter.min`→Counter 源；稳定 id 唯一、source+sha256+engine+provider+instances 逐项冻结；load 时逐项核对，错项具名拒（缺源/错 hash/错 engine/错 provider 不 launch）。
- **宿主会话链（B-HOST-I）**：读源→hash 核→`ensure_ui980_provider`（SDK 身份）→`NodeDesktopBridge::load`（Schema/verify）→`open`（真 FFI）→`install_bridge`→`run_app::<AceNodeDesktopWindow>`（真 Iced 窗/真输入/tick 受控）→关窗后回收观察（closed/stale_rejected/late_dropped/queue 排空）→证据 JSON。拒绝腿：错车道标签（BAD_OP）/错句柄（UNKNOWN_HANDLE）/过期 session（SESSION_CLOSED）/provider 撤权（revoke→拒→readmit→恢复），全部零 UI 效果后原入口恢复；全链无旧 VM 调用（no-fallback 结构性成立）。
- **命令与判据**：`cargo test -p os-desktop`（逻辑层：注册表冻结/错 engine/缺源/错 hash 拒，≤10s/例）；`cargo test -p os-desktop --features ace-min`（桥层：counter 0→1→0→0、两实例 2:1、close/reopen late/stale、错标签/错句柄/撤权零效果+恢复、hello 静面）；`AUTO_OS_ACE_MIN_E2E=1`+预备 provider→真窗 e2e（frames_rendered≥1）；V-OS-A-MIN 正式腿=真 ui-desktop 经 `desktop.ps1 -CandidateDir -ExtraArgs --ace-min … --ace-evidence <scope>/vos-a-min.json` 跑通激活/计数/两实例/关闭回收/错协议恢复并留证据。e2e 门档沿 980 r11 口径（交互档环境门控）。

### PROOF：A最小宿主见证、B完整业务旅程（新增未实施）

最小前置：HOST的MIN-A-OS接面、L969/L970/L978与980/NATIVE MIN-A-UI及实际使用的库；不要求980/RUST、完整LANGUAGE/PARITY或Notes/Clock全迁移。原完整PROOF旅程在B-OS按上述A12/A15/ASSEMBLY承接。实际改面：`D:/autostack/auto-os/shell/shell.at`；`D:/autostack/auto-os/host/desktop/src/main.rs`；`D:/autostack/auto-notes`；`D:/autostack/auto-clock`（均核存在）；永久ACE fixture由B-PROOF-I在原owner建立，临时材料仍归本Plan的plan-artifacts scope，不新增第二runner/语义实现。

I：A从980最小Node见证经真实OS启动/注册/activate/窗口链建立隔离AV候选；原shell与Notes/Clock源的完整输入/保存重开/计时/两App和AV/A2R旅程迁三波后HOST B、A12/A15与ASSEMBLY；不更新28App gitlink或机器默认，不冒单App全业务。
V：便笺文本原样保存重开，计时器按确定调度推进，窗口/store/任务不串，关闭资源归零；缺源/错协议拒绝；真实桌面不以空host通过。
R：独立核普通源→新producer→真实consumer、原expected/资源/退出及source/SDK身份；本批只关闭RG-A义务，不冒全部产品迁移。

### 具体输入、类型/代码设计与旧用例映射（2026-11-12，r5当前）

**具体输入与期望。** 原件[auto-os/shell/desktop.at](../../shell/desktop.at)（下列节选L28–L75；完整源/模块/资源闭包仍是验收输入，不把截断程序当可运行fixture）：

```auto
widget DesktopSurface {
    msg {
        Init,
        ActivateApp(str),
        IconMenu(str),
        // PLAN-012 W5：拖拽态机（press-press 两段式——onclick 即 iced
        // press；按下拾起 → 按到目标卡落下交换；按回原卡 = 取消）。
        IconPress(str),
        BlankPress, BlankDrop,
        MenuOpen,
        MenuRemove,
        MenuWallpaper,
        // PLAN-526 T10：空白处右键（mouse-area oncontextmenu——布局件
        // 右键臂缺位的替代挂点；菜单态本地门控，IconMenu 同型）。
        BlankMenu,
        // PLAN-019 v1.7：壁纸 picker（栅格点选即应用 / 预览二级态 /
        // 浏览换目录 / 关闭——语义全宿主收口，本面零分支）。
        PickerApply(str),
        // PLAN-035 T-11：桌面右键菜单「桌面小组件」开关（dashboard_toggle
        // v1.8 词面；可见性判据 = 宿主投影 __wm_dashboard）。
        ToggleDashboardBlank,
        PickerPreview(str),
        PickerBack,
        PickerNav(str),
        PickerBrowse,
        PickerDismiss,
        // PLAN-014 W-05'：空白菜单「恢复默认图标」（清 hidden 单源 +
        // refresh_desktop_icons 即时重注入）。
        ResetIconsBlank,
        // PLAN-014 W-02：总线写单点（同 shell.at SendCmd——追加语义，
        // 同排空周期多命令不互相覆盖）。
        SendCmd(str)
    }

    model {
        var __desktop_cmd str = ""
        var __desktop_bg str = ""
        var __desktop_icons = []
        var __desktop_hidden str = ""
        // PLAN-012 W5：格子化图标表（宿主注入——拖拽换位渲染面 + 平行
        // 字符串列表供 handler 下标读，B12 规避）。
        var __desktop_cells = []
        var __desktop_cell_ids = []
        var __desktop_cell_cs = []
        var __desktop_cell_rs = []
        var menu_id str = ""
        // PLAN-012 W5：拖拽态（按下拾起的图标 id；"" = 无拖拽）。
        var drag_id str = ""
```

本包所需独立结果：桌面activate同原应用id，Notes编辑保存重开数据一致、Clock倒计时不因多view倍速；真实桌面窗口/通知/workspace/配置/direct/RQ与AV/A2R均按原支持格核。 本轮未运行；这些断言是实施目标，不是已观察PASS。

**类型/代码设计。** 原shell/desktop与各App普通Auto源/旧数据→公共ACE/980/981→双host/实际桌面组件→独立真实业务/输入/数据→ASSEMBLY汇合。沿已有owner扩展，不因下面的名字另建平行模型/执行器。字段草图为拟设计，不是现存API或可编译声明；等价复用当前类型允许在原有界决定任务冻结，不能改变字段所表达的不变量：

```rust
// Proposed semantic fields; not existing/compiled API.
struct DesktopRuntimeBinding { shell_source: SourceIdentity, host: HostIdentity, app_registry: AppRegistry, engine: EngineIdentity }
struct AppMigrationReceipt { app: AppIdentity, mode: HostMode, original_data: FixtureIdentity, operations: ObservationSet, sources: SourceClosure }
```

| 实际代码owner | 本包具体职责与改面 |
|---|---|
| [auto-os/host/desktop/src](../../host/desktop/src) | 本仓独占桌面/窗口/启动/系统接线 |
| [auto-os/shell/desktop.at](../../shell/desktop.at) | 真实桌面pack/通知/图标协议源 |
| [auto-os/apps.manifest](../../apps.manifest) | 注册分母快照，新增consumer需补而非排除 |

**状态、错误与资源。** 缺app/新consumer、错gitlink/source/ABI、旧VM fallback、posted事件冒native、空桌面/进程存在冒业务均拒；未知原生格不填PASS。 HOST+PROOF属A，28App/ASSEMBLY按M2A-08后B；工作区/window/App/store/mount分别owner，双Host和普通Node根保完整；每App原数据/回滚/关闭资源不减。 沿原数值宽度/长度字符串/平台适用性/受管预算，不从Rust/C默认行为猜Auto语义，不以timeout/缺工具冒语言反例。

**旧输入到新检查的映射。**

| 原源码/测试/研究来源 | 保留的公开义务与新ACE验证 | 原期望/证据形态与读取身份 | 新ACE检查映射与原owner任务 |
|---|---|---|---|
| [auto-os/shell/desktop.at](../../shell/desktop.at) | 原桌面pack与普通Node桌面根新增consumer要同时验 | 无邻接expected；沿原loader/合同核，不补假绿；SHA256前16 `376b156d4f458699` | 拟ace.m2.p64.source01：desktop-av/desktop-a2r及原direct/RQ格；负控/资源见V-DETAIL-02/03；原任务 B-HOST-I/B-HOST-V/B-PROOF-I/B-PROOF-V/B-ASSEMBLY-I/B-ASSEMBLY-V |
| [auto-notes/src/front/app.at](../../../auto-notes/src/front/app.at) | 独立clone真实Notes源/持久数据，不把子模块目录当开发树 | 无邻接expected；沿原loader/合同核，不补假绿；SHA256前16 `19f2c7af0e39a66a` | 拟ace.m2.p64.source02：desktop-av/desktop-a2r及原direct/RQ格；负控/资源见V-DETAIL-02/03；原任务 B-HOST-I/B-HOST-V/B-PROOF-I/B-PROOF-V/B-ASSEMBLY-I/B-ASSEMBLY-V |
| [auto-clock/src/front/app.at](../../../auto-clock/src/front/app.at) | 真实clock/多view/计时 | 无邻接expected；沿原loader/合同核，不补假绿；SHA256前16 `84388e5a481552ca` | 拟ace.m2.p64.source03：desktop-av/desktop-a2r及原direct/RQ格；负控/资源见V-DETAIL-02/03；原任务 B-HOST-I/B-HOST-V/B-PROOF-I/B-PROOF-V/B-ASSEMBLY-I/B-ASSEMBLY-V |
| [auto-ui/crates/auto-ui-runtime/tests/desktop_behavior.rs](../../../auto-ui/crates/auto-ui-runtime/tests/desktop_behavior.rs) | 现桌面/宿主协议与行为 | Rust断言来源，具体test/check由原测试模块核；SHA256前16 `3eac91d0d5e221c4` | 原具名test/check或合同规则→V-DETAIL-01..04对应断言；T-01冻结完整ID，不当独立可运行源；原任务 B-HOST-I/B-HOST-V/B-PROOF-I/B-PROOF-V/B-ASSEMBLY-I/B-ASSEMBLY-V |

上表拟ID为可追踪规划键，尚未注册/执行；已有同义case/check优先复用并绑定，不重复累计；原支持域外的新语义先具名裁定。直接复用旧源码/公开行为断言与有效独立expected；旧VM opcode/对象布局、旧A2R生成全文金样保留给旧实现，不照搬为新后端唯一语义oracle。错误期望、空expected、仅parse/emit、句柄/内部显示编码分别登记；已知旧bug与规范冲突具名裁定，不能静默改expected或将全旧行为当新规范。表中hash为本轮读取快照（同轮修订的Plan按最终revision再核），不是永久固定接口；表为已核代表/已有研究入口，不是覆盖上限；沿原完整分母逐条映射，不以几份例子关闭本包全部原义务。

未来永久用例沿现TestCase/ACE worker或当地既有测试owner补充；语言侧可在`crates/auto-lang/test/cases/ace-m2/`以本Plan具名子目录承载，UI/Man/Web/OS真实消费者仍在对应仓既有tests/fixture，不能语言fixture代产品。准确新文件/命令由下列有界决定任务登记，当前不生成源码/测试资产、不另建runner/checker。

**原任务内的决定和交接。** T-01把每个B-A01..28映射实际独立clone/manifest id/source/原功能数据/命令/oracle/模式，新发现consumer交本owner；场景不是语言测试替身，不重复984生态实施。 必须补齐旧case/test/check ID＋source/hash＋原断言/期望种类→新case/后端/独立oracle→原任务/AC/组合owner；已定部分按本节执行，只有具名未决交原有界决定任务，不再把全部设计留空。不存在等价旧用例的目标/安装/可信来源/性能不变量明确标“无旧同形”，复用现机制并补区分性正反控，不伪造旧覆盖。

本节由原`T-01`冻结来源/未决，`B-HOST-I`、`B-HOST-V`、`B-PROOF-I`、`B-PROOF-V`、`B-ASSEMBLY-I`、`B-ASSEMBLY-V`按各自原owner_stage接实现/验证（纯分析包只审计），对应AC-BASE/AC-HOST/AC-ASSEMBLY/AC-SHELL-AGE/AC-PHASE；原review/merge任务分别核结论与正式落地，不能用规划检查提前勾选。范围外能力回原owner，真实新增依赖须改DAG，不补假的整包反向等待。

### 规范增量

| delta_id | add/modify/retire | docs/specs target | before/after rule | rationale | acceptance IDs |
|---|---|---|---|---|---|
| SD-01 | add | docs/specs/shell/ace-desktop-and-app-compatibility.md | 无ACE组合事实→已验host/桌面`.at`及Node根/App/数据/模式/AV与A2R来源 | OS唯一领域知识 | AC-HOST、AC-A01..A28、AC-ASSEMBLY、AC-SHELL-AGE |
| SD-02 | modify | docs/specs/shell/desktop-app-launch.md | 旧物理owner/VM执行→UI owner/ACE dispatch，保launch语义 | 名字不等实现 | AC-HOST |
| SD-03 | modify | docs/specs/apps/independent-repositories.md | 原组合→已review固定来源与结果 | 不自动升级 | AC-ASSEMBLY/AC-CLOSE |
| SD-PHASE | add | `docs/specs/shell/ace-desktop-and-app-compatibility.md`（沿原owner；仅实际交付后沉淀） | 单一M2口径→已验A/B能力/consumer及来源与旧新义务映射；未交B保持pending | 阶段不减原范围 | AC-PHASE/AC-A-PROOF |


| SD-DETAIL | add | `docs/specs/shell/ace-desktop-and-app-compatibility.md`（沿原owner，交付后沉淀） | 仅范围→实际已验输入/类型/状态/旧新check映射与限制 | 不以旧emit/规划代能力 | AC-BASE/AC-HOST/AC-ASSEMBLY/AC-SHELL-AGE/AC-PHASE |
| delta_id | add/modify/retire | target | before/after rule | rationale | acceptance IDs |
|---|---|---|---|---|---|
| SD-OS-WAVES | modify | docs/specs/shell/ace-desktop-and-app-compatibility.md及desktop-app-launch.md（交付后） | A全Notes/Clock AV-A2R→A最小真实Node OS宿主；完整桌面/App在三UI波后，三端公共能力与App模式分账 | 用户本轮三波与A最小出口裁定 | AC-OS-WAVES、AC-PHASE |

## 6. 测试设计

### 阶段适用验证（新增合同、未执行）

| Case | AC | 入口 | 独立判据 |
|---|---|---|---|
| V-PHASE | AC-PHASE | 各批V/I与原最终work验证及本包原固定组/真实owner入口，按§5阶段表选取 | 该阶段原required义务与新check逐行可追溯；普通源/独立expected、适用后端/平台/错误/资源真实执行；A仅RG指定产品，B全范围。删行/错producer/零选择/缺工具不能过；B-pending不改NA。 |


全部未执行。cwd=OS专属树，roots=冻结owner来源。scope=D:/autostack/plan-artifacts/plan-064/auto-os/，开工登记owner/用途/在途；外置target/安装/数据/截图按App放scope，普通构建留专属树，不散落。

| Case | AC | 入口与独立结果 |
|---|---|---|
| V-HOST | AC-HOST | `cargo test -p os-desktop`；`pwsh -File scripts/desktop.ps1 -Track iced -DryRun -LangRoot <lang> -UiRoot <ui> -ManRoot <man> -BuildRoot <scope>/build`，Vue换Track。DryRun仅验来源，正式去DryRun完成两App操作；候选腿用`-CandidateDir <scope>/candidate`。 |
| V-A01..A28 | AC-A01..A28 | 行内种子+I新增永久反例，V记录冻结命令与真实桌面操作；原源/旧数据对独立expected，Vue/原生/生成/RQ原支持格分别结果。 |
| V-COMBINE | AC-ASSEMBLY | `pwsh -File scripts/check-app-submodules.ps1 -OsRoot <candidate>`及`-SelfTest`；candidate全部子模块初始化，外部/local身份另核。健康门不能代业务。 |
| V-MIX | AC-ASSEMBLY | 执行981/MIX-05及Design55/MIX-06原场景，scope仓外候选manifest同源、旧新代隔离、坏更新保旧、资源归零。 |
| V-NEG | AC-ASSEMBLY | 缺一个App来源、错SDK/协议、旧dispatch残留、坏gitlink各负控红，邻近完整候选绿。 |
| V-SHELL-AGE | AC-SHELL-AGE | B-ASSEMBLY-I建普通Node桌面根/原pack同源fixture与命令，V消费980/981/978正式接口，经现有desktop候选入口真实AV和A2R编译运行；核图标/launcher/dock/pin/通知/工作区/窗口、双App/主题/旧配置/daemon/资源、改源新结果及失败修复；独立状态/事件/画面oracle，原数据不变。dispatch无旧VM fallback，A2R桌面业务本身实际执行生成代码；各App仍按原声明使用ACE AV/编译形态，不要求所有App强改同模式。DryRun/空桌面/附加小窗/进程存在不通过；窗口/GPU错峰，缺条件pending。 |

语言无改动不跑lang cargo；共享IR/执行改动由owner跑完整受影响固定组交收据。AGE只受影响文件/case，窗口/GPU单实例；不重跑全AG。发现/执行/排除、准备/运行/清理/总时与子终态分列，缺环境/首次红/timeout不能PASS。


### A期PROOF验证（本轮未运行）

| Case | AC | 实际/拟补入口 | 独立oracle |
|---|---|---|---|
| V-A-PROOF | AC-A-PROOF | A为V-OS-A-MIN：原os-desktop与真实desktop.ps1候选启动入口，隔离Node Hello/Counter经AV真实输入/激活/关闭；后端参数由I核实 | 原Notes保存重开/Clock计时/双Host/AV-A2R旅程原oracle保留并在B-OS A15/A12/HOST/ASSEMBLY关闭；DryRun/空host不能通过A本身 |

### 本轮细化的可区分验证（拟实施；纯分析包只核推理/来源）

| Case ID | AC/原任务 | 输入→入口→独立关闭oracle | 边界/错误近邻 |
|---|---|---|---|
| V-DETAIL-01 | AC-BASE/AC-HOST/AC-ASSEMBLY/AC-SHELL-AGE/AC-PHASE；B-HOST-I/B-HOST-V/B-PROOF-I/B-PROOF-V/B-ASSEMBLY-I/B-ASSEMBLY-V | 桌面activate同原应用id，Notes编辑保存重开数据一致、Clock倒计时不因多view倍速；真实桌面窗口/通知/workspace/配置/direct/RQ与AV/A2R均按原支持格核。 | 按上述原完整输入/真实consumer验证；原旧检查继续保留，未运行不PASS |
| V-DETAIL-02 | AC-BASE/AC-HOST/AC-ASSEMBLY/AC-SHELL-AGE/AC-PHASE；B-HOST-I/B-HOST-V/B-PROOF-I/B-PROOF-V/B-ASSEMBLY-I/B-ASSEMBLY-V | 缺app/新consumer、错gitlink/source/ABI、旧VM fallback、posted事件冒native、空桌面/进程存在冒业务均拒；未知原生格不填PASS。 | 配合法相邻输入；infra/timeout/缺工具不充语言负例 |
| V-DETAIL-03 | AC-BASE/AC-HOST/AC-ASSEMBLY/AC-SHELL-AGE/AC-PHASE；B-HOST-I/B-HOST-V/B-PROOF-I/B-PROOF-V/B-ASSEMBLY-I/B-ASSEMBLY-V | HOST+PROOF属A，28App/ASSEMBLY按M2A-08后B；工作区/window/App/store/mount分别owner，双Host和普通Node根保完整；每App原数据/回滚/关闭资源不减。 | 源/ABI/target/配置变化按实际读取失效；失败与修复/关闭各观察，不自报布尔 |
| V-DETAIL-04 | AC-BASE/AC-HOST/AC-ASSEMBLY/AC-SHELL-AGE/AC-PHASE；B-HOST-I/B-HOST-V/B-PROOF-I/B-PROOF-V/B-ASSEMBLY-I/B-ASSEMBLY-V | 按本节旧新表逐原case/check与完整分母对账；新后端/新不变量有独立oracle及原件/来源 | 删必需检查、错candidate/source或仅emit时必须被发现；该包非运行职责不得冒产品PASS |

真实入口沿本Plan原§6/当地owner命令，原固定组/平台/ignore/正式性能/AG定向规则不变；新增fixture只由原实施任务建立。发现/执行/原条件排除、准备/运行/清理、首次红/重放与缺证分列。新表不代原case/check、不新增任务号、不让原review/merge阶段提前通过。

### A最小/B-OS迁交验证（拟议，未运行）

| V / 原owner | 输入与关闭谓词 |
|---|---|
| V-OS-A-MIN / HOST、PROOF I/V/R A部分 | 从真实desktop候选入口加载正式980最小Node源/SDK，激活/点击计数/两实例/关闭与错协议恢复；独立值/实例/任务回收，AV真正dispatch且无旧fallback。仅DryRun/空host/硬编码Rust窗红；A2R/W2/W3/App全量pending不阻这个A最小。 |
| V-OS-WAVE-ENTRY / HOST B部分、各App I | 正式三波收据含所有UI/Widgets源/后端/ABI；错源/漏一波/把AXR0当W3必须拒绝开始产品迁移。冻结真实桌面guest选择与模式，不虚构当前有AC桌面CLI。 |
| V-OS-WAVE-FINAL / ASSEMBLY I/V/R | 三波后的真实shell/Node桌面根三端、A01..28和登记增量各声明模式/旧数据/独立操作、双Host/daemon/错误/关闭；Notes/Clock原V-A-PROOF完整旅程迁此及A12/A15。删App/模式/AC域、旧VM回退或拿UI截图代OS业务须红。 |

V-A-PROOF/AC-A-PROOF旧文字保留其完整业务oracle，但当前A仅V-OS-A-MIN；其Notes/Clock/完整桌面部分由上表具名原任务在B关闭，不能遗漏，也不让其反卡A。未来原B-PROOF-I核scripts/desktop.ps1与candidate注册接口真实参数，拟见证App由该任务创建；已有cargo test -p os-desktop与desktop.ps1真实非DryRun入口复用。新source/后端case未实施；Windows不跑Linux/WSL，真实平台/窗口单实例与原条件保持。

## 7. 验收标准

| ID | owner_stage | 可观察标准 | 对应任务 |
|---|---|---|---|
| AC-BASE | work | 28项及959增量/原模式/旧数据/真实owner来源/原门完整 | T-01 |
| AC-HOST | work | §5 HOST与V-HOST全过，无旧执行回退/主检出误读 | B-HOST-I/V |
| AC-A01 | work | kanban共同生命周期及新建/移动卡片、列顺序、刷新与旧数据重开；原模式/数据保持，ACE身份可证 | B-A01-I/V |
| AC-A02 | work | auto-musk共同生命周期及隔离daemon读取真实计划、修改隔离记录后重开，不改生产任务；原模式/数据保持，ACE身份可证 | B-A02-I/V |
| AC-A03 | work | jade-garden共同生命周期及文章/集合读写、真实后端错误与重连、关闭daemon回收；原模式/数据保持，ACE身份可证 | B-A03-I/V |
| AC-A04 | work | auto-term共同生命周期及PTY输入输出、中文/resize、双终端隔离与关闭子进程；原模式/数据保持，ACE身份可证 | B-A04-I/V |
| AC-A05 | work | 037-klondike共同生命周期及固定牌局发牌/合法非法移动、撤销重启，牌面/得分独立oracle；原模式/数据保持，ACE身份可证 | B-A05-I/V |
| AC-A06 | work | 036-tetris共同生命周期及固定序列移动/旋转/消行、真实键盘焦点与停止tick；原模式/数据保持，ACE身份可证 | B-A06-I/V |
| AC-A07 | work | 038-minesweeper共同生命周期及固定雷盘揭开/标记/胜负/重开，坏动作不改变棋盘；原模式/数据保持，ACE身份可证 | B-A07-I/V |
| AC-A08 | work | 028-launcher共同生命周期及搜索/点选真实App、稳定ID单实例与失败启动恢复；原模式/数据保持，ACE身份可证 | B-A08-I/V |
| AC-A09 | work | 039-syslog共同生命周期及日志订阅/过滤/滚动、断流重连与窗口关闭取消；原模式/数据保持，ACE身份可证 | B-A09-I/V |
| AC-A10 | work | jade-edit共同生命周期及打开/编辑/保存/草稿恢复，坏保存保旧文件，多文档隔离；原模式/数据保持，ACE身份可证 | B-A10-I/V |
| AC-A11 | work | 011-calculator共同生命周期及输入/优先级/清除/错误恢复，数值精度按原oracle；原模式/数据保持，ACE身份可证 | B-A11-I/V |
| AC-A12 | work | 012-clock共同生命周期及时间/计时启动暂停恢复，关闭窗口释放timer；原模式/数据保持，ACE身份可证 | B-A12-I/V |
| AC-A13 | work | 013-todo共同生命周期及增改完成/删除、筛选与旧任务持久化重开；原模式/数据保持，ACE身份可证 | B-A13-I/V |
| AC-A14 | work | 014-weather共同生命周期及地点查询、真实HTTP失败重试与late响应隔离；原模式/数据保持，ACE身份可证 | B-A14-I/V |
| AC-A15 | work | 015-notes共同生命周期及编辑/搜索/保存/重开，旧便笺可读、坏写保旧；原模式/数据保持，ACE身份可证 | B-A15-I/V |
| AC-A16 | work | 016-calendar共同生命周期及跨月/闰日、事件增改删除与持久化恢复；原模式/数据保持，ACE身份可证 | B-A16-I/V |
| AC-A17 | work | 017-chat共同生命周期及双会话消息/重连/旧历史，断线和关闭后late消息隔离；原模式/数据保持，ACE身份可证 | B-A17-I/V |
| AC-A18 | work | 018-book-reader共同生命周期及EPUB装载/翻页/目录/批注保存重开、坏书恢复；原模式/数据保持，ACE身份可证 | B-A18-I/V |
| AC-A19 | work | 023-realworld共同生命周期及真实文章读写/转义/评论原支持格，会话与失败事务；原模式/数据保持，ACE身份可证 | B-A19-I/V |
| AC-A20 | work | 030-video-player共同生命周期及真实媒体播放/暂停/seek/结束、关闭释放解码和音频资源；原模式/数据保持，ACE身份可证 | B-A20-I/V |
| AC-A21 | work | 020-music-player共同生命周期及播放/暂停/切曲/seek，列表恢复与关闭音频资源；原模式/数据保持，ACE身份可证 | B-A21-I/V |
| AC-A22 | work | 029-photo-gallery共同生命周期及目录图片装载/切换/缩放、坏图恢复与旧主题数据；原模式/数据保持，ACE身份可证 | B-A22-I/V |
| AC-A23 | work | 031-paint共同生命周期及绘制/撤销/保存重开，独立像素oracle与坏保存保旧；原模式/数据保持，ACE身份可证 | B-A23-I/V |
| AC-A24 | work | 025-sys-monitor共同生命周期及真实采样刷新/图表，取消采样与两实例隔离；原模式/数据保持，ACE身份可证 | B-A24-I/V |
| AC-A25 | work | 026-database共同生命周期及隔离库schema/查询/事务、坏SQL回滚与旧连接配置；原模式/数据保持，ACE身份可证 | B-A25-I/V |
| AC-A26 | work | 027-file-manager共同生命周期及隔离目录列举/复制移动/重命名、权限拒绝保源文件；原模式/数据保持，ACE身份可证 | B-A26-I/V |
| AC-A27 | work | 024-charts共同生命周期及原数据装载/系列切换/绘图，导出与坏数据恢复；原模式/数据保持，ACE身份可证 | B-A27-I/V |
| AC-A28 | work | os-config共同生命周期及隔离配置/主题/策略读写，拒绝非法值且保留原设置；原模式/数据保持，ACE身份可证 | B-A28-I/V |
| AC-ASSEMBLY | work | 全候选来源固定、隔离/双Host/MIX/负控齐，unknown零 | B-ASSEMBLY-I/V |
| AC-SHELL-AGE | work | 桌面本体`.at`及普通AGE Node根在ACE AV与A2R编译候选真实承载桌面业务/原功能；与原28 App组合、启动/焦点/窗口/主题/旧数据/资源/恢复正确，来源/协议/运行模式可核，无旧引擎fallback；新Node入口不要求原App源码全部改写 | B-ASSEMBLY-I/V；V-SHELL-AGE及原HOST/App/ASSEMBLY结果 |
| AC-REVIEW | review | 每批及全包独立核AC/差额/反例/健康/Spec，无遗漏 | 各R/T-05 |
| AC-CLOSE | merge | owner正式交付、Spec/归档/清理与组合来源齐 | T-06/07 |
| AC-A-PROOF | work | A只MIN-A-OS真实启动/激活/窗口链的Node见证/双实例/关闭/错协议；原便笺保存重开、计时隔离及全部业务模式在B-OS具名原任务关闭 | B-PROOF-I/V/R的A部分＋V-OS-A-MIN；完整余量A12/A15/HOST B/ASSEMBLY |
| AC-PHASE | work | §5任务阶段/实际前置及原义务→新检查/独立oracle映射齐；对应阶段required闭合，B开工已核A正式收据，无scope缩减 | V-PHASE、各批V/I与原最终work验证；原独立review复核 |

| ID | owner_stage | 标准 | 核验 |
|---|---|---|---|
| AC-OS-WAVES | work | A的真实OS最小Node见证正式可交；三波后原Notes/Clock/完整桌面/登记App、双Host/数据/资源/模式原义务逐行在A12/A15/HOST/ASSEMBLY关闭；不以UI波通过冒OS已迁 | V-OS-A-MIN/WAVE-ENTRY/FINAL，原R与T-05独立final |

## 8. 执行步骤

当前共97任务，0完成。下面是唯一当前执行清单；原ID/AC保留。任务按A/B分组，组内按§5实际前置选择就绪任务，不把行号当依赖。完整Plan只有原终态归档，A部分完成不清唯一树。

### M2-A：能力与必要真实验证

- [x] **T-01｜M2-A准备｜owner_stage: work｜AC-BASE**：先核§5的A最小前置、真实owner/源/接口/命令及本包实施授权，冻结A能力或具名PROOF；B部分只登记原输入/oracle与缺口，正式实现须M2A-08及其批次前置，不把整包最终depends_on当A开工前置。 [✅ 2026-11-17 A最小冻结落§5「T-01 A最小冻结」块：前置已交核验（969/970/973/974/975/978 归档、980 MIN-A-UI 落主）、源/协议身份 sha256 冻结、候选注册表/宿主链/命令判据齐；B 登记维持§5 原表+三波前置，无新增缺口]

- [ ] **B-HOST-I｜M2-A最小/B-OS完整｜owner_stage: work｜AC-HOST**：在§5宿主文件接新公开接口、版本拒绝及双App永久反例。 本轮A仅MIN-A-OS/V-OS-A-MIN，原Notes/Clock全旅程与完整桌面/模式按具名迁交表在B-OS关闭；A独立R/fold不勾整任务，不等图形A2R。

- [ ] **B-HOST-V｜M2-A最小/B-OS完整｜owner_stage: work｜AC-HOST**：执行V-HOST真实腿，记录dispatch/资源/来源。 本轮A仅MIN-A-OS/V-OS-A-MIN，原Notes/Clock全旅程与完整桌面/模式按具名迁交表在B-OS关闭；A独立R/fold不勾整任务，不等图形A2R。

- [ ] **B-HOST-R｜M2-A最小/B-OS完整｜owner_stage: review｜AC-REVIEW**：停写独立核host边界/双App oracle，不冒全产品通过。 本轮A仅MIN-A-OS/V-OS-A-MIN，原Notes/Clock全旅程与完整桌面/模式按具名迁交表在B-OS关闭；A独立R/fold不勾整任务，不等图形A2R。

- [ ] **B-PROOF-I｜M2-A最小/B-OS完整｜owner_stage: work｜AC-A-PROOF**：在真实OS候选启动/注册/激活/窗口服务接980 MIN-A Node Hello/Counter源，创建隔离测试App ID与永久V-OS-A-MIN/命令；不改生产manifest/gitlink。原Notes/Clock/完整shell/AV-A2R旅程由B-OS A15/A12/HOST/ASSEMBLY承接，原oracle保留。

- [ ] **B-PROOF-V｜M2-A最小/B-OS完整｜owner_stage: work｜AC-A-PROOF**：A只实跑V-OS-A-MIN：真实AV输入/计数/激活/双实例/关闭/错协议恢复与source/SDK/host身份；不是空host/DryRun。完整原V-A-PROOF业务在三波后的B-OS具名任务核，不等待其结果才交A。

- [ ] **B-PROOF-R｜M2-A最小/B-OS完整｜owner_stage: review｜AC-A-PROOF**：同树停写独立核A最小真实宿主/typed窗口与迁交账，正式fold交IF-OS064-A-MIN给985/A；不提前勾本任务完整B余量/整包，也不签Notes/Clock/全桌面迁移。

A交接：本组work/原正反例齐→独立复核相应A切片→按owner流程正式合入并绑定source/SDK/接口/实际结果→985/A消费。若本包还有B，不提前勾全包review/归档；同树保留/resync。

### M2-B：全产品接管与完整收口

本组HOST完整余量、各App和ASSEMBLY的产品实施先核M2A-08及UI-W1-AV、UI-W2-AC、UI-W3-A2R三波均已正式交付，再核各App所用精确服务/库输入；不能只凭M2A-08跳过三波启动全量迁移。

所有本组实现任务先核M2A-08已合入收据，再核本批§5前置；只读准备不充B执行结果。A接口变化须回能力owner修复并定向更新A收据，不静默沿用旧证据。

- [ ] **B-A01-I｜M2-B｜owner_stage: work｜AC-A01**：在`D:/autostack/auto-kanban`迁移kanban工具/执行/配置接线；从`tests/run.mjs`补§5业务/失败/旧数据永久fixture并固定命令，不改非目标功能。

- [ ] **B-A01-V｜M2-B｜owner_stage: work｜AC-A01**：执行V-A01原支持格：新建/移动卡片、列顺序、刷新与旧数据重开及共同生命周期；核source/SDK/expected/资源/子终态。

- [ ] **B-A01-R｜M2-B｜owner_stage: review｜AC-REVIEW**：独立核该App全部AC/反例/实际dispatch，缺陷回本I；原义务保持，同版本收据交ASSEMBLY。

- [ ] **B-A02-I｜M2-B｜owner_stage: work｜AC-A02**：在`D:/autostack/auto-musk`迁移auto-musk工具/执行/配置接线；从`tests/musk-ui-smoke.sh`补§5业务/失败/旧数据永久fixture并固定命令，不改非目标功能。

- [ ] **B-A02-V｜M2-B｜owner_stage: work｜AC-A02**：执行V-A02原支持格：隔离daemon读取真实计划、修改隔离记录后重开，不改生产任务及共同生命周期；核source/SDK/expected/资源/子终态。

- [ ] **B-A02-R｜M2-B｜owner_stage: review｜AC-REVIEW**：独立核该App全部AC/反例/实际dispatch，缺陷回本I；原义务保持，同版本收据交ASSEMBLY。

- [ ] **B-A03-I｜M2-B｜owner_stage: work｜AC-A03**：在`D:/autostack/auto-down`迁移jade-garden工具/执行/配置接线；从`jade-garden`补§5业务/失败/旧数据永久fixture并固定命令，不改非目标功能。

- [ ] **B-A03-V｜M2-B｜owner_stage: work｜AC-A03**：执行V-A03原支持格：文章/集合读写、真实后端错误与重连、关闭daemon回收及共同生命周期；核source/SDK/expected/资源/子终态。

- [ ] **B-A03-R｜M2-B｜owner_stage: review｜AC-REVIEW**：独立核该App全部AC/反例/实际dispatch，缺陷回本I；原义务保持，同版本收据交ASSEMBLY。

- [ ] **B-A04-I｜M2-B｜owner_stage: work｜AC-A04**：在`D:/autostack/auto-term`迁移auto-term工具/执行/配置接线；从`拟补永久产品回归（本轮未创建）`补§5业务/失败/旧数据永久fixture并固定命令，不改非目标功能。

- [ ] **B-A04-V｜M2-B｜owner_stage: work｜AC-A04**：执行V-A04原支持格：PTY输入输出、中文/resize、双终端隔离与关闭子进程及共同生命周期；核source/SDK/expected/资源/子终态。

- [ ] **B-A04-R｜M2-B｜owner_stage: review｜AC-REVIEW**：独立核该App全部AC/反例/实际dispatch，缺陷回本I；原义务保持，同版本收据交ASSEMBLY。

- [ ] **B-A05-I｜M2-B｜owner_stage: work｜AC-A05**：在`D:/autostack/auto-solitaire`迁移037-klondike工具/执行/配置接线；从`tests/rules_golden.cjs`补§5业务/失败/旧数据永久fixture并固定命令，不改非目标功能。

- [ ] **B-A05-V｜M2-B｜owner_stage: work｜AC-A05**：执行V-A05原支持格：固定牌局发牌/合法非法移动、撤销重启，牌面/得分独立oracle及共同生命周期；核source/SDK/expected/资源/子终态。

- [ ] **B-A05-R｜M2-B｜owner_stage: review｜AC-REVIEW**：独立核该App全部AC/反例/实际dispatch，缺陷回本I；原义务保持，同版本收据交ASSEMBLY。

- [ ] **B-A06-I｜M2-B｜owner_stage: work｜AC-A06**：在`D:/autostack/auto-tetris`迁移036-tetris工具/执行/配置接线；从`tests/gallery_contract.py`补§5业务/失败/旧数据永久fixture并固定命令，不改非目标功能。

- [ ] **B-A06-V｜M2-B｜owner_stage: work｜AC-A06**：执行V-A06原支持格：固定序列移动/旋转/消行、真实键盘焦点与停止tick及共同生命周期；核source/SDK/expected/资源/子终态。

- [ ] **B-A06-R｜M2-B｜owner_stage: review｜AC-REVIEW**：独立核该App全部AC/反例/实际dispatch，缺陷回本I；原义务保持，同版本收据交ASSEMBLY。

- [ ] **B-A07-I｜M2-B｜owner_stage: work｜AC-A07**：在`D:/autostack/auto-minesweeper`迁移038-minesweeper工具/执行/配置接线；从`tests/rust_smoke.py`补§5业务/失败/旧数据永久fixture并固定命令，不改非目标功能。

- [ ] **B-A07-V｜M2-B｜owner_stage: work｜AC-A07**：执行V-A07原支持格：固定雷盘揭开/标记/胜负/重开，坏动作不改变棋盘及共同生命周期；核source/SDK/expected/资源/子终态。

- [ ] **B-A07-R｜M2-B｜owner_stage: review｜AC-REVIEW**：独立核该App全部AC/反例/实际dispatch，缺陷回本I；原义务保持，同版本收据交ASSEMBLY。

- [ ] **B-A08-I｜M2-B｜owner_stage: work｜AC-A08**：在`D:/autostack/auto-launcher`迁移028-launcher工具/执行/配置接线；从`tests/click_input_smoke.mjs`补§5业务/失败/旧数据永久fixture并固定命令，不改非目标功能。

- [ ] **B-A08-V｜M2-B｜owner_stage: work｜AC-A08**：执行V-A08原支持格：搜索/点选真实App、稳定ID单实例与失败启动恢复及共同生命周期；核source/SDK/expected/资源/子终态。

- [ ] **B-A08-R｜M2-B｜owner_stage: review｜AC-REVIEW**：独立核该App全部AC/反例/实际dispatch，缺陷回本I；原义务保持，同版本收据交ASSEMBLY。

- [ ] **B-A09-I｜M2-B｜owner_stage: work｜AC-A09**：在`D:/autostack/auto-os/apps/auto-syslog`迁移039-syslog工具/执行/配置接线；从`拟补永久产品回归（本轮未创建）`补§5业务/失败/旧数据永久fixture并固定命令，不改非目标功能。

- [ ] **B-A09-V｜M2-B｜owner_stage: work｜AC-A09**：执行V-A09原支持格：日志订阅/过滤/滚动、断流重连与窗口关闭取消及共同生命周期；核source/SDK/expected/资源/子终态。

- [ ] **B-A09-R｜M2-B｜owner_stage: review｜AC-REVIEW**：独立核该App全部AC/反例/实际dispatch，缺陷回本I；原义务保持，同版本收据交ASSEMBLY。

- [ ] **B-A10-I｜M2-B｜owner_stage: work｜AC-A10**：在`D:/autostack/jade-edit`迁移jade-edit工具/执行/配置接线；从`tests/draft_recovery_matrix.mjs`补§5业务/失败/旧数据永久fixture并固定命令，不改非目标功能。

- [ ] **B-A10-V｜M2-B｜owner_stage: work｜AC-A10**：执行V-A10原支持格：打开/编辑/保存/草稿恢复，坏保存保旧文件，多文档隔离及共同生命周期；核source/SDK/expected/资源/子终态。

- [ ] **B-A10-R｜M2-B｜owner_stage: review｜AC-REVIEW**：独立核该App全部AC/反例/实际dispatch，缺陷回本I；原义务保持，同版本收据交ASSEMBLY。

- [ ] **B-A11-I｜M2-B｜owner_stage: work｜AC-A11**：在`D:/autostack/auto-calc`迁移011-calculator工具/执行/配置接线；从`tests/acceptance.atd`补§5业务/失败/旧数据永久fixture并固定命令，不改非目标功能。

- [ ] **B-A11-V｜M2-B｜owner_stage: work｜AC-A11**：执行V-A11原支持格：输入/优先级/清除/错误恢复，数值精度按原oracle及共同生命周期；核source/SDK/expected/资源/子终态。

- [ ] **B-A11-R｜M2-B｜owner_stage: review｜AC-REVIEW**：独立核该App全部AC/反例/实际dispatch，缺陷回本I；原义务保持，同版本收据交ASSEMBLY。

- [ ] **B-A12-I｜M2-B｜owner_stage: work｜AC-A12**：在`D:/autostack/auto-clock`迁移012-clock工具/执行/配置接线；从`tests/t3_smoke.py`补§5业务/失败/旧数据永久fixture并固定命令，不改非目标功能。

- [ ] **B-A12-V｜M2-B｜owner_stage: work｜AC-A12**：执行V-A12原支持格：时间/计时启动暂停恢复，关闭窗口释放timer及共同生命周期；核source/SDK/expected/资源/子终态。

- [ ] **B-A12-R｜M2-B｜owner_stage: review｜AC-REVIEW**：独立核该App全部AC/反例/实际dispatch，缺陷回本I；原义务保持，同版本收据交ASSEMBLY。

- [ ] **B-A13-I｜M2-B｜owner_stage: work｜AC-A13**：在`D:/autostack/auto-todo`迁移013-todo工具/执行/配置接线；从`tests/run_autotest.py`补§5业务/失败/旧数据永久fixture并固定命令，不改非目标功能。

- [ ] **B-A13-V｜M2-B｜owner_stage: work｜AC-A13**：执行V-A13原支持格：增改完成/删除、筛选与旧任务持久化重开及共同生命周期；核source/SDK/expected/资源/子终态。

- [ ] **B-A13-R｜M2-B｜owner_stage: review｜AC-REVIEW**：独立核该App全部AC/反例/实际dispatch，缺陷回本I；原义务保持，同版本收据交ASSEMBLY。

- [ ] **B-A14-I｜M2-B｜owner_stage: work｜AC-A14**：在`D:/autostack/auto-weather`迁移014-weather工具/执行/配置接线；从`tests/vm_smoke.py`补§5业务/失败/旧数据永久fixture并固定命令，不改非目标功能。

- [ ] **B-A14-V｜M2-B｜owner_stage: work｜AC-A14**：执行V-A14原支持格：地点查询、真实HTTP失败重试与late响应隔离及共同生命周期；核source/SDK/expected/资源/子终态。

- [ ] **B-A14-R｜M2-B｜owner_stage: review｜AC-REVIEW**：独立核该App全部AC/反例/实际dispatch，缺陷回本I；原义务保持，同版本收据交ASSEMBLY。

- [ ] **B-A15-I｜M2-B｜owner_stage: work｜AC-A15**：在`D:/autostack/auto-notes`迁移015-notes工具/执行/配置接线；从`tests/acceptance.atd`补§5业务/失败/旧数据永久fixture并固定命令，不改非目标功能。

- [ ] **B-A15-V｜M2-B｜owner_stage: work｜AC-A15**：执行V-A15原支持格：编辑/搜索/保存/重开，旧便笺可读、坏写保旧及共同生命周期；核source/SDK/expected/资源/子终态。

- [ ] **B-A15-R｜M2-B｜owner_stage: review｜AC-REVIEW**：独立核该App全部AC/反例/实际dispatch，缺陷回本I；原义务保持，同版本收据交ASSEMBLY。

- [ ] **B-A16-I｜M2-B｜owner_stage: work｜AC-A16**：在`D:/autostack/auto-calendar`迁移016-calendar工具/执行/配置接线；从`tests/basic`补§5业务/失败/旧数据永久fixture并固定命令，不改非目标功能。

- [ ] **B-A16-V｜M2-B｜owner_stage: work｜AC-A16**：执行V-A16原支持格：跨月/闰日、事件增改删除与持久化恢复及共同生命周期；核source/SDK/expected/资源/子终态。

- [ ] **B-A16-R｜M2-B｜owner_stage: review｜AC-REVIEW**：独立核该App全部AC/反例/实际dispatch，缺陷回本I；原义务保持，同版本收据交ASSEMBLY。

- [ ] **B-A17-I｜M2-B｜owner_stage: work｜AC-A17**：在`D:/autostack/auto-chat`迁移017-chat工具/执行/配置接线；从`tests/acceptance.atd`补§5业务/失败/旧数据永久fixture并固定命令，不改非目标功能。

- [ ] **B-A17-V｜M2-B｜owner_stage: work｜AC-A17**：执行V-A17原支持格：双会话消息/重连/旧历史，断线和关闭后late消息隔离及共同生命周期；核source/SDK/expected/资源/子终态。

- [ ] **B-A17-R｜M2-B｜owner_stage: review｜AC-REVIEW**：独立核该App全部AC/反例/实际dispatch，缺陷回本I；原义务保持，同版本收据交ASSEMBLY。

- [ ] **B-A18-I｜M2-B｜owner_stage: work｜AC-A18**：在`D:/autostack/auto-reader`迁移018-book-reader工具/执行/配置接线；从`tests/epub`补§5业务/失败/旧数据永久fixture并固定命令，不改非目标功能。

- [ ] **B-A18-V｜M2-B｜owner_stage: work｜AC-A18**：执行V-A18原支持格：EPUB装载/翻页/目录/批注保存重开、坏书恢复及共同生命周期；核source/SDK/expected/资源/子终态。

- [ ] **B-A18-R｜M2-B｜owner_stage: review｜AC-REVIEW**：独立核该App全部AC/反例/实际dispatch，缺陷回本I；原义务保持，同版本收据交ASSEMBLY。

- [ ] **B-A19-I｜M2-B｜owner_stage: work｜AC-A19**：在`D:/autostack/auto-blog`迁移023-realworld工具/执行/配置接线；从`tests/byte_text_roundtrip.py`补§5业务/失败/旧数据永久fixture并固定命令，不改非目标功能。

- [ ] **B-A19-V｜M2-B｜owner_stage: work｜AC-A19**：执行V-A19原支持格：真实文章读写/转义/评论原支持格，会话与失败事务及共同生命周期；核source/SDK/expected/资源/子终态。

- [ ] **B-A19-R｜M2-B｜owner_stage: review｜AC-REVIEW**：独立核该App全部AC/反例/实际dispatch，缺陷回本I；原义务保持，同版本收据交ASSEMBLY。

- [ ] **B-A20-I｜M2-B｜owner_stage: work｜AC-A20**：在`D:/autostack/auto-video`迁移030-video-player工具/执行/配置接线；从`tests/package.json`补§5业务/失败/旧数据永久fixture并固定命令，不改非目标功能。

- [ ] **B-A20-V｜M2-B｜owner_stage: work｜AC-A20**：执行V-A20原支持格：真实媒体播放/暂停/seek/结束、关闭释放解码和音频资源及共同生命周期；核source/SDK/expected/资源/子终态。

- [ ] **B-A20-R｜M2-B｜owner_stage: review｜AC-REVIEW**：独立核该App全部AC/反例/实际dispatch，缺陷回本I；原义务保持，同版本收据交ASSEMBLY。

- [ ] **B-A21-I｜M2-B｜owner_stage: work｜AC-A21**：在`D:/autostack/auto-music`迁移020-music-player工具/执行/配置接线；从`tests/test_020_vm.py`补§5业务/失败/旧数据永久fixture并固定命令，不改非目标功能。

- [ ] **B-A21-V｜M2-B｜owner_stage: work｜AC-A21**：执行V-A21原支持格：播放/暂停/切曲/seek，列表恢复与关闭音频资源及共同生命周期；核source/SDK/expected/资源/子终态。

- [ ] **B-A21-R｜M2-B｜owner_stage: review｜AC-REVIEW**：独立核该App全部AC/反例/实际dispatch，缺陷回本I；原义务保持，同版本收据交ASSEMBLY。

- [ ] **B-A22-I｜M2-B｜owner_stage: work｜AC-A22**：在`D:/autostack/auto-photos`迁移029-photo-gallery工具/执行/配置接线；从`tests/test_058_theme.py`补§5业务/失败/旧数据永久fixture并固定命令，不改非目标功能。

- [ ] **B-A22-V｜M2-B｜owner_stage: work｜AC-A22**：执行V-A22原支持格：目录图片装载/切换/缩放、坏图恢复与旧主题数据及共同生命周期；核source/SDK/expected/资源/子终态。

- [ ] **B-A22-R｜M2-B｜owner_stage: review｜AC-REVIEW**：独立核该App全部AC/反例/实际dispatch，缺陷回本I；原义务保持，同版本收据交ASSEMBLY。

- [ ] **B-A23-I｜M2-B｜owner_stage: work｜AC-A23**：在`D:/autostack/auto-paint`迁移031-paint工具/执行/配置接线；从`tests/basic`补§5业务/失败/旧数据永久fixture并固定命令，不改非目标功能。

- [ ] **B-A23-V｜M2-B｜owner_stage: work｜AC-A23**：执行V-A23原支持格：绘制/撤销/保存重开，独立像素oracle与坏保存保旧及共同生命周期；核source/SDK/expected/资源/子终态。

- [ ] **B-A23-R｜M2-B｜owner_stage: review｜AC-REVIEW**：独立核该App全部AC/反例/实际dispatch，缺陷回本I；原义务保持，同版本收据交ASSEMBLY。

- [ ] **B-A24-I｜M2-B｜owner_stage: work｜AC-A24**：在`D:/autostack/auto-monitor`迁移025-sys-monitor工具/执行/配置接线；从`tests/package.json`补§5业务/失败/旧数据永久fixture并固定命令，不改非目标功能。

- [ ] **B-A24-V｜M2-B｜owner_stage: work｜AC-A24**：执行V-A24原支持格：真实采样刷新/图表，取消采样与两实例隔离及共同生命周期；核source/SDK/expected/资源/子终态。

- [ ] **B-A24-R｜M2-B｜owner_stage: review｜AC-REVIEW**：独立核该App全部AC/反例/实际dispatch，缺陷回本I；原义务保持，同版本收据交ASSEMBLY。

- [ ] **B-A25-I｜M2-B｜owner_stage: work｜AC-A25**：在`D:/autostack/auto-database`迁移026-database工具/执行/配置接线；从`tests/basic`补§5业务/失败/旧数据永久fixture并固定命令，不改非目标功能。

- [ ] **B-A25-V｜M2-B｜owner_stage: work｜AC-A25**：执行V-A25原支持格：隔离库schema/查询/事务、坏SQL回滚与旧连接配置及共同生命周期；核source/SDK/expected/资源/子终态。

- [ ] **B-A25-R｜M2-B｜owner_stage: review｜AC-REVIEW**：独立核该App全部AC/反例/实际dispatch，缺陷回本I；原义务保持，同版本收据交ASSEMBLY。

- [ ] **B-A26-I｜M2-B｜owner_stage: work｜AC-A26**：在`D:/autostack/auto-explorer`迁移027-file-manager工具/执行/配置接线；从`tests/plan023_check.py`补§5业务/失败/旧数据永久fixture并固定命令，不改非目标功能。

- [ ] **B-A26-V｜M2-B｜owner_stage: work｜AC-A26**：执行V-A26原支持格：隔离目录列举/复制移动/重命名、权限拒绝保源文件及共同生命周期；核source/SDK/expected/资源/子终态。

- [ ] **B-A26-R｜M2-B｜owner_stage: review｜AC-REVIEW**：独立核该App全部AC/反例/实际dispatch，缺陷回本I；原义务保持，同版本收据交ASSEMBLY。

- [ ] **B-A27-I｜M2-B｜owner_stage: work｜AC-A27**：在`D:/autostack/auto-charts`迁移024-charts工具/执行/配置接线；从`tests/golden`补§5业务/失败/旧数据永久fixture并固定命令，不改非目标功能。

- [ ] **B-A27-V｜M2-B｜owner_stage: work｜AC-A27**：执行V-A27原支持格：原数据装载/系列切换/绘图，导出与坏数据恢复及共同生命周期；核source/SDK/expected/资源/子终态。

- [ ] **B-A27-R｜M2-B｜owner_stage: review｜AC-REVIEW**：独立核该App全部AC/反例/实际dispatch，缺陷回本I；原义务保持，同版本收据交ASSEMBLY。

- [ ] **B-A28-I｜M2-B｜owner_stage: work｜AC-A28**：在`D:/autostack/auto-os-config`迁移os-config工具/执行/配置接线；从`package.json`补§5业务/失败/旧数据永久fixture并固定命令，不改非目标功能。

- [ ] **B-A28-V｜M2-B｜owner_stage: work｜AC-A28**：执行V-A28原支持格：隔离配置/主题/策略读写，拒绝非法值且保留原设置及共同生命周期；核source/SDK/expected/资源/子终态。

- [ ] **B-A28-R｜M2-B｜owner_stage: review｜AC-REVIEW**：独立核该App全部AC/反例/实际dispatch，缺陷回本I；原义务保持，同版本收据交ASSEMBLY。

- [ ] **B-ASSEMBLY-I｜M2-B-OS最终｜owner_stage: work｜AC-ASSEMBLY/AC-SHELL-AGE**：建立隔离候选、gitlink及外部repo身份，补缺项/错版本/MIX与V-SHELL-AGE；普通Node根复用真实桌面pack/协议，固定AV/A2R命令和业务/状态/画面独立预期，不重写宿主或通用语言。 三UI波正式后接V-OS-WAVE-FINAL：包括公共桌面Node根的真实AC及AV/A2R、gallery容器、全部原App与登记增量；不等待985/B或986/987。

- [ ] **B-ASSEMBLY-V｜M2-B-OS最终｜owner_stage: work｜AC-ASSEMBLY/AC-SHELL-AGE**：执行V-COMBINE/MIX/NEG、V-SHELL-AGE与全候选真实桌面原模式，逐格结果/源和候选身份交984/985及AGE M2联合门；缺桌面或App结果不通过。 三UI波正式后接V-OS-WAVE-FINAL：包括公共桌面Node根的真实AC及AV/A2R、gallery容器、全部原App与登记增量；不等待985/B或986/987。

- [ ] **B-ASSEMBLY-R｜M2-B-OS最终｜owner_stage: review｜AC-REVIEW**：独立核组合来源/旧数据/双Host/MIX与各App收据适用性，unknown阻通过。 三UI波正式后接V-OS-WAVE-FINAL：包括公共桌面Node根的真实AC及AV/A2R、gallery容器、全部原App与登记增量；不等待985/B或986/987。

- [ ] **T-05｜M2-B｜owner_stage: review｜AC-REVIEW**：全包独立复核绑定revision/SHA，核遗漏/延期/workaround、warning/fmt/debug与Spec。

- [ ] **T-06｜M2-B｜owner_stage: merge｜AC-CLOSE**：按当地流程先交App再固定OS组合；核正式来源/沉淀Spec，全齐归档交984。

- [ ] **T-07｜M2-B｜owner_stage: merge｜AC-CLOSE**：摘要折计划、实际输出逐目录对账，清scope/刷新登记，wt-guard clean后清OS树；不删除App clone或其他计划树。

B收口：所有B work、原全产品/组合及A证据适用性齐→原全包独立review→正式合入/Spec/归档→输出对账/清理；各批R与全包final分别执行，不能只数批次替原AC。

### 工作位置与材料



沿§3/§5既定唯一树与原owner规约实施；材料仅归本Plan的plan-artifacts按仓/批次scope，未知归属不删除，清树前必须wt-guard clean。本轮未创建树或材料scope。


本轮细化交接：原任务ID/阶段/进度保持；T-01须在依赖实现前把本轮具体输入/类型/旧新check与未决来源冻结；B-HOST-I/B-HOST-V/B-PROOF-I/B-PROOF-V/B-ASSEMBLY-I/B-ASSEMBLY-V分别落实对应V-DETAIL-01..04与原所有验证。分析/格式/性能/发行包沿各自职责，不把例子复制成另一执行器或新框架。未决改变既有公共语义/预算/来源门则具名修订，不静默缩分母。

## 9. 复审记录

2026-11-12 stage:new / PLAN-064 r5：按963标准补本包具体输入/独立预期、类型与模块职责、旧源/check映射及V-DETAIL-01..04。保原97任务/全部AC与V/依赖/阶段/预算和drafting 0/97；只做文档与源码静态核对，不实施/建树/生成fixture/运行/派agent/合入或发布。已有研究/已归档成果复用，新增草图不冒已有API或产品支持。

2026-11-17 stage: work（M2-A goal round 1：用户 `/goal 完成计划064/A`，/auto-plan-goal 承接 = M2-A work/独立复核/有界修复/A期合入授权）| plan_id: PLAN-064 | plan_revision: r6 | outcome: pass（A 切片 work 可交付独立复核；B 相任务框按 A 交接规则留开）| worktree: D:/autostack/.wt/os-064/auto-os @ plan-064-dev | base_commit: 2383d72 | code_commit: fb1192c | dependency_revisions: 组兄弟 detached = 980 fold#7 交付时点（lang 7f431bd5f / ui 403c936d / web 0adebc0 / man 11fe79c＋锁 1 行随动 / down 5af11a3；man/ui 锁一致性裁定见 §5 T-01 块）| task_ids: T-01 ✅；B-HOST-I/V、B-PROOF-I/V 的 A 切片实现＋真跑 | evidence: §5「T-01 A最小冻结」；plan-artifacts/plan-064/auto-os/（vos-a-min-hello-headless / vos-a-min-counter-headless / vos-a-min-counter-window.json）。V-OS-A-MIN 实跑：desktop.ps1 -Track iced -AceMin 全源 --locked 构建 → 真 ui-desktop 真窗 **372 帧**，冻结轨迹 0→1→0→0＋两实例 2:1（headless 与真窗腿同证），拒绝腿 错标签-5/错句柄-1/撤权 denied→readmit 恢复全零帧，close/late_dropped=1/关后 poll=-2/reopen 世代递增状态存活，双 App 进程内+跨进程隔离；套件 plain 29（18 基线+11 纯逻辑）、feature 30+4、真窗 e2e（AUTO_OS_ACE_MIN_E2E）372/373 帧、feature-off `--ace-min` 具名拒 exit 1（无旧 VM 调用）| blockers: 无 | next_responsible_role: review（B-HOST-R/B-PROOF-R A 切片独立核）。

2026-11-17 stage: review（B-HOST-R+B-PROOF-R A 切片；独立 subagent 同树停写核 fb1192c）| outcome: needs_fix | findings: **P1**＝T-01 冻结的 缺源/错 hash 具名拒缺永久判别负例，且纯逻辑测试未达 T-01 冻结命令 `cargo test -p os-desktop`；**P2**（fold 时修/记 B）＝artifact 索引 pin 陈旧、resolve_ui_root 编译期锚、ace-av 别名与 4096 步界无案例（无合同条目要求）。核过绿面：注册表冻结/身份链/真 FFI/真窗 372 帧/2:1/世代/回收/零效果拒绝/结构性无旧 VM 回退（feature 双态实测）/改面恰 6 文件/apps.manifest 零触碰/基线 18 不动/每例 ≤10s/证据与主张逐项相符 | repair: **06d3a0b**——身份核抽离为 feature 无关纯函数 `load_verified_source`，逻辑测试迁至 feature 无关层（T-01 冻结命令直跑 11 例），补 p064_missing_source_named_refusal / p064_pin_mismatch_named_refusal（错 hash 负例直穿唯一身份门，门删必红）；P2 索引 pin 已修 | re-run: plain 29 绿、feature 30+4 绿、fmt --check 干净 | blockers: 无 | next_responsible_role: 复核人确认 → A 交接 fold。

2026-11-17 stage: review（A 切片修复复核 @06d3a0b，同独立复核人）| outcome: **pass** | 复核实据：修复仅 ace_min.rs（163+/116−）；`load_verified_source` 行为逐字节等价且 hash 门全文件唯一（行146）；两负例分别直穿读门与唯一 pin 门（门删必红）；逻辑层 11 例达 T-01 冻结命令直跑（取两方案中更强的"迁移"解）；plain 29 / feature 30+4 实跑绿；health.rs 漂移确认为 base 既有（计划区间零触碰）| 结论：**A 切片具备 IF-OS064-A-MIN 正式 fold 条件**——source=os-desktop@06d3a0b（plan-064-dev）、消费身份=auto-ui 403c936d / auto-lang 7f431bd5f（detached 组兄弟）、冻结注册表 pin、V-OS-A-MIN 证据在 plan-artifacts/plan-064/auto-os/；整包 review/归档明确不在本审范围（B 波留，任务框留开，唯一树保留）。

2026-11-17 stage: goal（/auto-plan-goal 完成计划064/A 收口）| plan_id: PLAN-064 | plan_revision: r6 | round: 1 | from: work→review→merge(A fold) | to: user（A 交接完成，B 波待指令）| status: executing 1/97（任务框按 A 交接规则：T-01 勾，双相任务留开）| outcome: pass | worktree: D:/autostack/.wt/os-064/auto-os（保留，同树续 B/resync）| commit: 落主 **06d3a0b**（ff-only，无 merge commit，2383d72→fb1192c→06d3a0b 直线史）+ 簿记 **484c432**（本计划文件入册）| **IF-OS064-A-MIN 交接账（交 985/A 消费）**：交付面=os-desktop `ace_min` 候选见证链（注册表 ace.hello.min/ace.counter.min，blake3 冻结；feature `ace-min` 缺省关=具名拒；真 FFI open→真 Iced 窗→宿主关闭回收→证据 JSON；拒绝腿 错标签/错句柄/撤权零效果+恢复；全链无旧 VM）；消费身份=auto-ui 403c936d（980 §12 MIN-A-UI）/ auto-lang 7f431bd5f（969 facade/ace-host）；V-OS-A-MIN 证据=真窗 372 帧 desktop.ps1 -AceMin 正式腿 + headless 双 App + e2e 372/373 帧，均在 D:/autostack/plan-artifacts/plan-064/auto-os/ | landing 观察：落主 SHA 与实跑验证树同源（ff-only 无重写）；主检组（masters 追新至 ui 2f42d661 等）与落主锁 pin 已漂移——主检出 `--locked` 构建被拒（--locked 只读探测未改动任何文件）；此为 fold#7 后上游 master 移动的既有组态（同样堵主组 desktop.ps1 --locked），非本批引入；续用 pin 组树或下批具名重钉 | 遗留（不阻 A 完成）：B 波全部原义务（B-HOST/B-PROOF 完整余量、A01–A28、ASSEMBLY、V-SHELL-AGE）；P2 记 B（resolve_ui_root 编译期锚、ace-av 别名、4096 步界）；plan-artifacts scope 清理归整包 merge。

2026-11-12 本轮规划静态核验：32份合同原505任务ID/顺序/进度、全部旧AC/V/Spec-delta及原依赖/阶段/预算保留；新增128条V-DETAIL待实施验证，未生成任何测试源码。505处本地引用/围栏/0–10章节/原任务映射核对通过，跨四仓可达222份Plan的显式依赖无缺失/环。只证明规划结构与取材来源，不是旧VM/新ACE运行或独立产品review PASS；959–962归档和963r5未改。

2026-11-10 stage=new / r4：直接整理§2与§8当前执行合同，A/B清单分组、最小前置与阶段交接明示，消除旧单阶段总顺序/旧批次数量的冲突。保全部97个原任务ID/原AC、drafting与0进度；未新增实现任务/用例/编号。仅文档核对，未施工/运行产品/独立实现review/阶段fold或归档。

2026-11-09 stage=new / r3：正式按跨M2-A/B拆任务归属、补AC-PHASE/V-PHASE/SD-PHASE与A期PROOF I/V/R，当前0/97；原AC/T及历史记录保留。只做文档静态预检，未施工/跑测试/独立实现review/合入或签阶段。

2026-11-09 r2 / stage:new：桌面产品目标补到原HOST/ASSEMBLY，新增AC-SHELL-AGE/V-SHELL-AGE，原94任务/30批/原AC/V和App分母全保留，drafting、0/94。984只核产品收据，不复制OS正文；OS消费980/981、上游不反等OS。仅规划，实施handoff仍blocked（未授权实施/真实接口条件未齐），无桌面运行或独立final PASS。

规划静态核验：94任务/34AC/6验证行，原ID全保留、无重复、状态/0进度不变；与984可达205份跨仓Plan，无缺失引用/依赖环，新链接可解析。ASSEMBLY由原HOST/各App接新桌面实证，联合门只收结果，不反置上游依赖；未运行桌面/构建/产品测试。

2026-11-07 stage=new，r1 drafting 0/94。静态核来源/manifest/路径与结构，无实施/产品测试/独立实现review/merge/push。984原T-02/03/04/08的桌面与应用、原AC-02/03/06→本计划HOST/A01–A28/ASSEMBLY；984保总闭合。OS054/063进度未改。

2026-11-07文档核验：28条manifest ID与A01–A28逐项一致、每项I/V/R齐，30批94任务与frontmatter相符；真实开发来源和所列已有测试种子均存在，本地链接/章节/命名唯一性通过；跨仓DAG无环。没有执行所列命令，此结论不代表产品验收。

2026-11-16 stage:new/revision | OS064 r6 | outcome:pass（规划可执行合同，非实现/认证PASS） | 97原任务ID/勾选/0完成保留；A精确最小HOST/PROOF，旧Notes/Clock/桌面业务分别迁A12/A15/HOST/ASSEMBLY；B-OS待三UI波正式，无新增App/生产配置变更 | next:由原owner依新阶段/真实来源接续；本轮只规划，不实施/建树/测试/合入/清理/恢复目标。

本轮规划静态预检：14份计划/设计/路线文档，8份Plan共230个当前任务；仅980新增B-AV-I/V/R三项并按完整W3范围重开原RUST三项，旧AXR0收据保留；981仅核正原SOURCE三项已交的滞后计数。全部原任务ID/其余勾选、各Plan状态与整包依赖列表保留；新增本地链接/锚点15处、章节/表格/元数据计数/验收任务映射及行尾检查通过，A最小→A出口→W1→W2独立交接→W3→OS→候选/安装/退役阶段图16节点18边无环。原GUI/Widgets/Notes-Clock/语言/库/产品义务有具名阶段与原owner，不以延期冒PASS。仅文档预检，未实施/建树/运行测试或窗口/签阶段/合入/清理/恢复目标，未写源码、测试资产、canonical Specs或取号计数器；他会话并发修改不由本轮回退。

## 10. 待澄清事项

原支持格、工具平台/外部服务由T-01/各I有界冻结；缺环境阻该批V，不阻其他就绪批。新增产品先追加owner/命令/oracle批次再实施。当前仅规划授权，不含施工/生产数据/发布；后续实施按指令进入work。
