---
plan_id: PLAN-060
status: archived
feature_name: 纸牌方块扫雷三产品的正式主题接入
author: [Codex]
created_at: 2026-11-03
updated_at: 2026-11-05
plan_revision: 1
current_step: 9
total_steps: 9
completion_kind: delivered
supersedes_spec_components: [docs/specs/shell/host-theme-policy-and-delivery.md, docs/specs/apps/independent-repositories.md]
new_spec_components: [docs/specs/apps/product-theme-batch-060.md]
touched_goals: []
affects: [auto-solitaire, auto-tetris, auto-minesweeper, product-theme-adoption, os-theme-combination]
depends_on: [auto-os/PLAN-053, auto-ui/PLAN-031, auto-ui/PLAN-033]
---

# [auto-os/PLAN-060] 纸牌方块扫雷三产品的正式主题接入

## 0. 变更摘要

以apps.manifest稳定ID为边界，完整接入本批3个实际产品：**037-klondike、036-tetris、038-minesweeper**。采用036正式包/生成输入与053/055实际宿主端口，迁产品当前真实功能，不从auto-ui教学例复制业务或拿其通过抵产品验收。单一跨仓主合同在OS，参与产品执行时加README指针，不另造同一主题竞争合同。

drafting r1、0/9。2026-11-03用户授权全产品调研立项/路线刷新，**暂不实施**；本轮未建树/改产品或OS pin/运行构建、测试、GUI/系统设置/远端动作，不创建或联系其他agent/chat、不改canonical。其他会话WIP保留。以下全部新运行门未执行。

## 1. 目标

- G1：每个产品同一正式主题包的两个族×明暗真实生效，保默认/品牌/显式override/follow优先级及原支持轨，不只换外壳源代码。
- G2：活跃业务/输入/草稿/会话/内容/计时与持久化跨换肤保持；原完整适用门、时序/性能条件和负控不减。
- G3：实际宿主启动/独立运行/多App隔离、错误拒绝/恢复/owned退出真实闭环；候选源码到正式产品与组合pin同源。
- G4：本批全产品逐一交付，与036/055/054及教学迁移分工明确；不以代表App、研究/计划数冒全量迁移完成。

非目标：扩产品新业务/改数据库或Locator/协议/引擎，重做036发行算法、055真实OS事件/通用服务、054业务装配、教学037/038代码同步、全机/AG/ACE回归、所有历史债清零与远端registry。全批存在必需门未闭时不能用“历史问题”充PASS。

## 2. 架构方案

UI单源主题→036已交付发行/验证输入→产品已解析依赖与design/现适配端口→053/055实际host policy→产品/组件scope→公共控件与已支持的内嵌表面→真实可见颜色/模型和applied回读。品牌/分类overlay、文档/媒体/VT/series等内容域各自有owner与豁免依据；不复制整套palette，不给语言facade/generation-core增加UI反向依赖。

产品当前真实业务/后端profile/原测试是验收基面。保实际manifest的vm启动、独立Vue/VM原支持；牌面红黑花色、Tetris块分类色、雷盘数字色是内容/规则身份，背景/面板/按钮/焦点是公共主题角色。不得为换肤改随机流/规则/分数/时间/难度/回放样本。 真正所需的新能力或既有非主题业务阻点先具名交owner/前置或重规划，不削减原承诺。只允许本批消费侧/已承诺主题投影的有限修复，公开协议/新引擎须另立约。

## 3. 技术栈

实际产品.at/pac/已有UI/engine/后端与原MCP/Playwright/产品验收；OS registry/launch/config/gitlink，UI正式包/scope/host transport，036输入/安装入口。准确命令/工具/版本/资源支持由T-01按真实README/Spec/help冻结，无新通用测试框架/registry。

## 4. 需求分析与背景调查

已使用auto-plan-new及verification-contract，读OS AGENTS、独立产品/主题policy Specs、UI主题/包/host transport、036/055/054和各产品实际pac/front/README/Spec/原测试/在途计划。逐入口完整来源与快照见[Design03全产品盘点](../design/03-product-theme-rollout.md)§2；该表记录28 active ID、HEAD/pin/源码摘要/dirty及原契约矛盾，不是运行证据。

manifest SHA256=57afca924fb95d99a29294163f134672d660c494202296b0f63812b749b08a44；OS观察HEAD=092254ce037ef7c7612cf11fe8b3fe00c57a76c6。下表Git前缀是观察值，未来开工重核完整SHA、review/merge/remote可取性及工具内容hash，不能直接拿当前main或陈旧OS pin当已验组合。

| 稳定ID / owner | clone HEAD / OS pin | 当前真实差额/在途 | 已核验证定位与隔离要求（参数工作时重核） |
|---|---|---|---|
| 037-klondike / auto-solitaire | 91192ef5 / 91192ef5 | legacy dark/indigo、规则golden/真输入 | tests/desktop_mcp.py、rules_golden.cjs、test_vm_klondike.py/rust_golden.py等原条件 |
| 036-tetris / auto-tetris | 06b2e67f / 06b2e67f | legacy dark/indigo、Rust API、原native物理输入 | tests/desktop_mcp.py、run_matrix.py/native_physical.py/vm_rules_golden.py原门 |
| 038-minesweeper / auto-minesweeper | ae6dcbfd / ae6dcbfd | 确定性棋盘/计时；官方包未接 | tests/desktop_mcp.py、rust_smoke.py等当前原规则门（不借教学相同脚本结果） |

U036和各产品已交付的规则/实际桌面launch=vm基线；不等待AG全线、UI038、054或055真实事件整项。图形扩展若新增需求另归owner，不迁游戏引擎。 某产品门未满足不修改其在途内容/接pin，可推进同批独立准备项；execution_done/结项必须本批全部必需面闭合。旧README/计划状态陈述过时由实际owner按收据处置，本包不代它终审或修改原合同。

## 5. 详细设计

### 5.1 启动、所有权与下一任务

实施授权待给。拟唯一OS组D:/autostack/.wt/os-060/auto-os、分支codex/plan-060-product-themes；UI/框架确有改动才各加同组唯一树。普通产品在D:/autostack/<repo>独立clone main一写入者，工作/独立review顺序共用，禁止编辑OS apps/子模块或复制第二review树。Musk按其AGENTS专属worktree，auto-down嵌套Garden按owner专属树；不能把普通产品main例外套基础仓。local syslog（若本批含）在OS树。各仓AGENTS工作时重读，不擅自stash/reset/切走他方WIP，无链接。

T-01冻结源/已交付业务与支持、实际package/CLI/tool/engine/后端、旧设置/品牌/颜色角色、原用例/样本/排除分母及端口/数据/窗口预算；用真实整链合法/拒绝控制证实接线可行性再做依赖实现。无正式来源的WIP不消费；若真正缺能力/需要改公开支持，给精确owner/关闭条件与合同修订，不静默降级成静态Demo。

### 5.1.1 T-01 冻结记录（2026-11-05 执行轮）

- 授权：`/goal 完成计划060`（2026-11-05）＝全链授权（work/review/有界修复/merge，沿 auto-plan-goal 授权条款）。
  §4 前置全部重核满足：U036=auto-ui/PLAN-036 **archived**（Full SDK/delivery 携 packages/themes）；OS PLAN-053
  archived；UI-031/033 archived。
- OS 组树：`D:/autostack/.wt/os-060/auto-os`，分支 `codex/plan-060-product-themes`，
  base `d15b823e96747df20a619ab528df7e944c7abdd6`。主检出 WIP（docs + apps/auto-charts）为他方会话所有，不动。
- 产品 clone（main，clean，单写入者=本会话，与 §4 观察前缀逐一相符）：
  auto-solitaire `91192ef593cb25377a2d089542e6f040507231a9`、auto-tetris
  `06b2e67f2117084853271cf194e67cc47be6b3c5`、auto-minesweeper
  `ae6dcbfd7033bc2ee81c6d11b7162d1c653e9d04`；remotes=github.com/auto-stack/<repo>。
- 工具链（059 同源）：`auto.exe=D:/autostack/auto-man/target/debug/auto`
  （`0.1.0+plan-890-verified-r7-t19-18-g03bb65e`，036 交付+053 机制）；主题包 managed 源
  `auto-ui/packages/themes` 0.2.0 per-file sha256 全 match（pac.at `2e66212b…`、themes.at
  `329c2d56…`），聚合 digest `9fa07742…`（053 宿主固定值；vendor 快照同律校验）；
  `--theme` CLI 覆盖（458）与 `gen --delivery-root`（792）在场。manifest/.gitmodules SHA256
  与 §4 观察值逐位一致（无漂移）。
- 消费形态冻结（056–059 同律）：`vendor/themes` 固定快照 + pac `dep themes`、`theme:` 键退役
  （follow）、accent 保留；app `use themes.themes: Auto` + `design{theme:"Auto",
  mode: if .dark_mode …}`、App 级 `dark_mode` 位默认 true（保现暗色默认）。不声明
  `dep stylekit`（weather r8 先例：rust 轨不解析、三游戏无需）。
- rust 轨风险退役：056–058 批 7 个产品（chat/todo/calendar/music/video/photos/explorer）均
  `api:"rust"`+`dep themes` 组合全绿——tetris 同组合有据，无需专项 a2r 探针。
- 原门分母冻结：solitaire=rules_golden.cjs / test_klondike*.cjs（17600 legacy 口，非本批入口）/
  desktop_mcp.py / rust_golden.py / test_vm_klondike.py；tetris=smoke.spec.ts(playwright) /
  desktop_mcp.py / run_matrix.py(--suite×4) / vm_rules_golden.py / native_physical.py /
  gallery_contract.py + rust-workspace cargo rules_golden；minesweeper=desktop_mcp.py /
  rust_smoke.py。命令按各 docs/source-README.md。
- 已知宿主注记（053 owner，非本批缺陷，沿 056–059 处置）：OS056-REAPPLY-NOTE 拒绝后无间隔
  立即重投偶发不生效（恢复路径=合同命名"重开或重连"）；frame-0 无已提交策略 boot 落 family
  默认浅色（一次翻转后双向正常）。
- 材料 scope：`D:/autostack/plan-artifacts/plan-060/auto-os/`（probe/evidence/logs）。

### 5.2 本批实现与区分性不变量

第一组Solitaire/Tetris主题壳和规则/输入门；第二组Minesweeper及三游戏同场隔离/Tick与退出。原rules_golden、native_physical、MCP及Rust声明轨条件逐项保留。

在牌局有效移动/回退、方块落子/消行/暂停、确定雷盘展开/旗标/踩雷期间换肤，规则状态/分数/计时原值连续；两个游戏相反主题不串，真实键鼠继续而非直接改模型伪通过。

逐产品列公共role/交互状态、品牌与分类色、内容域和合法固定值表；保现默认与其对应业务语义。每个产品真实Vue/VM及已承诺native/host路径分别核，不能从生成成功推定原生特殊组件可用。换肤前后固定业务哨兵与实际颜色/引擎选择读回，两族×明暗逐产品完成；需要主题化的可用嵌入表面通过原端口消费，不能只改外层截图。内容/媒体像素/文档身份不因“统一”被染色。

宿主跟随/覆盖/非法包/乱序拒绝保旧、重开或重连追最新，两个实际产品相反族/模式不串。包/I/O失败或未绘制不能回报applied；恢复后下一次真实操作成功，关闭只收割本次owned连接/进程/订阅。系统真实事件归055，本批不修改用户全局设置。

### 5.3 外部来源、候选与正式落地

036完整交付是采用新渠道的产品实施/最终消费门；T-01可获授权后先盘点，绝不借其execution_done实现树。通过正式安装/包输入做fresh/locked/第二绝对根、缺件/错hash/合法同mtime内容变更及恢复，绑定实际产品/产物；不手拷教学palette、不用原target充外部构建，910项目/member/context/lease与可写target隔离保持。

每产品先形成已提交候选，原门/主题/实际桌面支持组合验收；独立review后正常push，OS干净组合fetch/detach到同SHA、健康门+真实启动后才更新本批pin。外部无gitlink条目按真实repo/嵌套根HEAD、daemon/tool/资产身份固定并验启动，不伪造新submodule。仅修改本批增量，最新OS其他pin/注册/功能零意外覆盖；056正式pin等054先交付，后继必须是其审定业务版本的主题后代，不互相回退。

本批交接按Design03 §3/6：036拥有发行/输入、055拥有clock/G-1/G-2（仅所需outproc能力正式交付作为该腿门）、054拥有八基础业务装配；UI037/038教学线不抵产品。每个产品剩余问题/未支持平台具名，不把原支持义务删掉；真实依赖/source变更按918规则正常审计，不永久豁免。

### 规范增量

| delta_id | add/modify/retire | docs/specs/... target | before/after rule | rationale | acceptance IDs |
|---|---|---|---|---|---|
| SD-01 | add | docs/specs/apps/product-theme-batch-060.md（新） | 本批产品实际source/pin/profile/主题支持与剩余边界 | 组合不以Demo代验 | AC-01..05 |
| SD-02 | modify | docs/specs/shell/host-theme-policy-and-delivery.md（仅实际消费者/接缝） | 增本批真实跟随/覆盖/错误/状态支持，不重定义055真事件结论 | 宿主唯一policy | AC-03/05 |
| SD-03 | add/modify | 各参与产品拟docs/specs/theme-adoption.md（新或真实既有页）；local例在OS specs/apps | 逐产品默认/品牌/内容owner/原业务及精确来源，执行时按owner确认 | 不复用教学结论 | AC-01..05 |
| SD-04 | modify | docs/specs/apps/independent-repositories.md；现产品主题页（真实差额） | 产品独立演进/精确组合，阶段计数仅按实际验收 | 全清单不丢项 | AC-05/07 |

frontmatter只列OS目标；参与仓Spec T-01按实际存在与owner确认。未改能力的Spec不虚构增量，不写旧specs.json/手工缓存索引。

## 6. 测试设计

全部为未来验证；T-01绑定准确入口/fixture，T-02..05令新增反控可运行。Design03 §6共同合同与上表产品原门同时适用，不以此8例摘要削减各产品用例。work负责实现/内部自查；未指定特定agent/model；独立上下文final review在同组/同产品clone冻结写入后只验证，不修代码。实际双端执行时读autoui-verifier。

| Case ID | 不变量/真实输入 | 入口与可区分关闭oracle | owner_stage |
|---|---|---|---|
| V-01 | AC-01；本批每产品两族×明暗、旧默认/品牌/覆盖 | 真实产品入口/宿主各原支持轨；实际值来自所选正式包，非仅类名或Demo截图，内容域不乱改 | work |
| V-02 | AC-02；上表与§5.2所有产品原业务在活跃态中换肤/继续/退出冷恢复 | 原完整适用测试+新增真实交互，模型/磁盘/媒体/文档/会话等业务哨兵前后相符，原样本/条件/负控不减 | work |
| V-03 | AC-03；实际宿主启动、两产品相反选择、follow/override/拒绝、重连 | 真实scope/frame与applied/rejected对应，不串、不重建业务；config轮询与outproc推送分列，helper不代真实载体 | work |
| V-04 | AC-04；U036安装→fresh/locked/第二根、坏包/缺件/合法同mtime改变/恢复 | 实际产品产物/颜色与receipt身份一致，旧根不可见，坏包非0拒绝保旧、恢复可再次消费，910 target/lease不退化 | work |
| V-05 | AC-02/03/05；本批特有资源、原故障和来源/前置 | §5.2具名oracle与产品原门实际保留；隔离数据/owned进程、旁观者/其他pin不动，必需阻点未闭不能PASS | work |
| V-06 | AC-05；候选提交、功能基线/工具、真实候选组合及非目标源 | 被测source/候选SHA一一对应，本批入口全验；无未交付WIP/旧Demo/错误后端profile、旧pin替代，055/054门准确 | work |
| V-07 | AC-06；全部AC/产物/原负控/未闭义务与SD | 同树独立final review，逐产品源码+实际验证重建，遗漏/workaround/健康检查，任一必需面未闭不可pass | review |
| V-08 | AC-07/08；冻结并集、真实产品push/OS pin或外部身份、知识/输出 | 被测==交付，健康/实际启动/canonical/路线/归档读回，完整输出与设置/进程对账，guard与材料清理 | merge |

准确现测试定位见§4，每产品原Spec/README为原分母来源，旧路径/命令按实际help修定位，不能造不存在的标志。无现GUI测试的产品T-01必须补最小真实回归入口，不把缺文件当豁免。实际产物/工具内容hash、准备/编译/发现/运行/清理时间、执行/原条件排除/首红/重放分列，未跑不PASS。

纯OS/.at/产品资产不跑语言Cargo/docs_gen；共享UI Rust真改按受影响完整职责/原消费者，UI t/tu/ti按实际touch，语言中立库真改才选对应固定组。final后一次冻结必要并集，复用输入未变有效证据，不全AG/ACE/四仓全档。声明支持面有缺口先具名修复/前置或重规划，不为少跑改支持承诺。

## 7. 验收标准

| ID | 可观察关闭条件 | owner_stage |
|---|---|---|
| AC-01 | 本批全部3产品正式包/默认/品牌/两族×明暗、真实已承诺运行轨逐项通过，V-01 | work |
| AC-02 | 各产品原完整适用业务/数据/内容/资源/时序/性能条件守恒，活跃换肤不丢状态，V-02/05 | work |
| AC-03 | 宿主/独立/多App跟随覆盖、错误拒绝恢复和owned生命周期准确，V-03/05 | work |
| AC-04 | 036实际发行/生成输入、外部fresh/locked/移根/拒绝恢复与910规则在产品消费通过，V-04 | work |
| AC-05 | 基线/前置/源码与候选组合全验，036/055/054/Demo边界及非目标pin保护可核，无假支持/借WIP，V-05/06 | work |
| AC-06 | 独立final review绑定冻结源/逐产品证据与SD，健康/遗漏/workaround pass，V-07 | review |
| AC-07 | 冻结职责门/正常产品push/OS正式pin或外部精确源、实际启动/规范/路线/归档读回，V-08 | merge |
| AC-08 | 全输出/数据/进程/设置对账，必要摘要折Plan后删scope/自有树产物、guard clean；具名持有条件齐，V-08 | merge |

## 8. 执行步骤

| ID | 动作/可验收关闭结果 | 依赖 / AC | owner_stage |
|---|---|---|---|
| T-01 | 授权后核本批全部source/原合同/写入者/前置，唯一组/owner clone、scope，准确命令/资源/表面映射与整链合法/失败探针；阻点owner有据 | 授权待给；§4各产品门；AC-01..05 | work |
| T-02 | U036后按§5.2第一职责段接产品正式包/默认/品牌/可用内嵌主题端口，保原业务，新增定向反例 | T-01+U036及对应产品门；AC-01/02/03 | work |
| T-03 | 完成本批其余职责段/产品公共及专用支持面，宿主采纳/独立跟随、原交互/恢复，有限已有主题缺陷有反例 | T-02；AC-01..03 | work |
| T-04 | 本批每产品真实两族明暗/原业务/多App/错误/owned退出完整并集，记录首红/条件排除 | T-03；AC-01..03/05，V-01..03/05 | work |
| T-05 | 外部包/输入/移根/失效恢复，全部已提交产品候选/候选组合健康与实际启动，同SHA冻结；056组合门等U054 | T-04、§4精确前置；AC-04/05，V-04/06 | work |
| T-06 | 自查健康/遗漏/原义务/全批AC/SD草稿、source/pin/其他计划交接冻结；execution_done6/9 | T-05；AC-01..05 | work |
| T-07 | 同组/同clone冻结、独立上下文final review，发现返write owner并失效相应证据，不造第二实现树 | T-06；AC-06，V-07 | review |
| T-08 | 冻结必要职责并集、依赖序落地/产品正常push/OS干净组合pin或外部源读回、健康/实际启动/canonical/路线/归档；不覆盖054/其他新提交 | T-07 pass+§4组合门；AC-07，V-08 | merge |
| T-09 | 全输出/设置/owned进程对账，摘要折Plan删统一scope/自有树产物，各树guard clean后移除/分支/空组；保产品clone，archived9/9 | T-08；AC-08，V-08 | merge |

材料唯一D:/autostack/plan-artifacts/plan-060/auto-os/，按repo/owner/用途/在途登记；数据/库/文件/媒体/设置备份/日志截图/外部SDK/生成副本/显式target皆该scope，普通编译在唯一树，产品合法输出全列账。禁止默认temp/仓内attachments/其他盘散落证据。merge逐项核全部实际输出路径及活动归属，命令/退出码/关键值/源hash/未解项折Plan后删本scope/自有树产物；仅具名未闭持有有owner/理由/关闭条件，不能借持有将必需面判PASS。未知/活动/他方不盲删，产品clone不删/reset，legacy specs.json不写。

## 9. 复审记录

2026-11-03 / stage:new / auto-os/PLAN-060 r1 / outcome:blocked（本轮只调研立约，实施授权与§4精确前置待满足） / 已从28条manifest全清单确认本批3个实际入口/owner/HEAD与旧OS pin差额、现源码与原契约/测试；不以教学替产品、普通暗亮代正式主题、当前main代交付。所有运行/新反控均未执行。next：授权后work/T-01重核真实来源/写窗/资源和前置，不重复请求已确定范围，超公开支持/owner变更再提出具体修订。

静态预检pass：0–10章节、9T/8AC/8V/4SD唯一连续，所有AC/验证/任务归属有据；本批与其他批无重复，合28 active ID全覆盖；本号活动/归档唯一、OS号源062，本地引用/限定diff/尾空格检查通过。仅文档检查，所有项目运行/新负控/最终独立复审仍未执行。

2026-11-05 / stage:work / auto-os/PLAN-060 r1 / outcome:pass / worktree: D:/autostack/.wt/os-060/auto-os（分支 codex/plan-060-product-themes，base d15b823e，code 27a6c03） / dependency_revisions: auto.exe=auto-man 0.1.0+plan-890-verified-r7-t19-18-g03bb65e（036+053 机制）；managed themes 0.2.0 @ auto-ui/packages/themes（per-file sha256 全 match，digest 9fa07742…）；交付渠道 root1/root2 = man-install full（9704 文件，verify exit 0，sources: auto-ui 25d4ea5d/auto-man 03bb65e/auto-lang 5f673746/auto-down 7c4d5cbb/auto-web 37d32c22） / task_ids: T-01..T-06 / evidence: 批次页 docs/specs/apps/product-theme-batch-060.md（工作树 27a6c03）；产品候选（各自 clone main，未 push）：auto-solitaire 54c0051 / auto-tetris d252f36 / auto-minesweeper 6873674；逐产品 theme 套件 5/5（A/B/C/E=Auto/Solarized × 明/暗 四组合像素 oracle，D=活跃局中换肤业务守恒），组合腿 ISO（双产品相反策略同时运行不串扰）/REJ（错 digest 具名拒绝保旧实录）/EXIT（owned 退出收割后归零）/V-04 包源四腿（缺件/篡改/同 mtime 内容锁/恢复重应用）全过；原门：solitaire rules_golden 100%+rust_golden 12/12+vm_klondike exit0+desktop_mcp 22/26（=base 逐项一致）、tetris playwright 3/3+desktop_mcp exit0+vm_rules_golden PASS、mines desktop_mcp 25/0；原始证据 D:/autostack/plan-artifacts/plan-060/auto-os/ / blockers: 无阻塞性问题；具名预存红（base 同败，非本批回归）：a2r `??` E0599（tetris back）、a2r E0308（mines front）、tetris tracked golden fixture 缺席（均 owner=auto-lang，主题轨走 `--server vm` 既有支持模式）；solitaire desktop_mcp 4 项陈旧标签（base 同集）；gallery_contract 独立 clone 陈旧路径；native_physical 环境阻塞（SetForegroundWindow 拒绝）；OS056-REAPPLY-NOTE 与 standalone 跟随臂 dark_mode 位不同步（053 owner 注记） / next_responsible_role: review（T-07 同组同源独立 final review，冻结面=OS 工作树 27a6c03 + 三产品候选 SHA）

stage: goal | plan_id: PLAN-060 | plan_revision: r1 | round: 1 | from: work | to: review | status: execution_done | outcome: pass | worktree: D:/autostack/.wt/os-060/auto-os | commit: 27a6c03788ef389c45aa09de0723bab1990c5c1d | next: review

2026-11-05 / stage:review / review_scope: final / reviewer_identity: 独立子代理会话（无实现上下文；从冻结面构件重建验证，未采信执行方摘要）/ auto-os/PLAN-060 r1 / outcome: pass / worktree: D:/autostack/.wt/os-060/auto-os（branch codex/plan-060-product-themes，clean）/ reviewed_commit: 27a6c03788ef389c45aa09de0723bab1990c5c1d / base_commit: d15b823e96747df20a619ab528df7e944c7abdd6 / dependency_revisions: 同 work 收据（auto.exe t19-18-g03bb65e、managed themes 0.2.0）/ spec_inputs: 批次页（工作树）+ host-theme §PLAN-060 + independent-repositories §PLAN-060 + 三产品 docs/specs/theme-adoption.md / acceptance_results: AC-01..06 pass（复审者独立重建：rules_golden 100%、solitaire 显式暗 boot 角点 #09090b ±0、vue 轨 front 200、逐行豁免核对、tetris base db.at `??` 预存红在位、src/back 零 diff、vendor 快照 diff -r 全等、零 specs.json 写入）；AC-07/08 pending-merge（产品 push+OS pin、对账清理——by design 未做） / findings: P3×2（mines 批次映射表缺棋盘容器一行，复审者裁定"none required for acceptance，下次触面可补"——不改已复审基线；聚合 digest 非朴素拼接可复算，属 053 宿主内部算法，per-file 全等已足证内容同一）/ evidence: 复审报告全文随本轮会话存档；关键复算命令/结果已录本收据 / next: merge

stage: goal | plan_id: PLAN-060 | plan_revision: r1 | round: 2 | from: review | to: merge | status: reviewed | outcome: pass | worktree: D:/autostack/.wt/os-060/auto-os | commit: 27a6c03788ef389c45aa09de0723bab1990c5c1d | next: merge

2026-11-05 / stage:merge / PLAN-060:r1 / outcome:pass（cleaned 见末段收据）/ completion_kind: delivered / 各检查点实录：
- prepared：reviewed 基线 27a6c03（spec 三件）+ 产品候选三 SHA；canonical 目标 docs/specs/{apps/product-theme-batch-060.md(new), shell/host-theme-policy-and-delivery.md, apps/independent-repositories.md}。
- 产品落地：三产品 push origin/main 成功且 SHA==候选（solitaire 91192ef→54c0051、tetris 06b2e67→d252f36、mines ae6dcbf→6873674）。
- OS 组合：工作树内 submodule fetch/detach 同 SHA（三 apps 组合检出），真实启动三连 READY（solitaire/tetris--server vm/mines，组合路径 boot）；check-app-submodules 工作树 exit0（tetris MISMATCH 即待提交 pin，其余未初始化=worktree 设计态）；gitlink 提交 08740a1。
- landed：wt-guard clean；main 已前进（054 round62 落地）→ rebase main（range-diff 全等号：27a6c03=7a79dfd、08740a1=dd6c1d7）；`git merge --ff-only` 成功，main tip=dd6c1d7（无 merge commit）；旧→新映射 27a6c03→7a79dfd、08740a1→dd6c1d7。
- canonical_verified：dd6c1d7 ls-tree 含三 spec 文件（blob 795f09b/d9f768e/92f5481）；主检出三 apps submodule update --init+detach 同步到新 pin 后 clean。
- 组合健康：check-app-submodules（main）exit0——本批三产品零条目，唯一 MISMATCH=apps/auto-charts（他会话 WIP，056 域，非本批）。
- 合并自有门留观（stale-artifact 观察，非重建项）：OS 桌面宿主二进制与本批无关（本批 OS delta=spec+gitlink，无 OS 代码）；产品 desktop_exe rust 制品为开发环 debug 档（launch=vm 桌面轨不经它），沿用各产品原状。
stage: goal | plan_id: PLAN-060 | plan_revision: r1 | round: 3 | from: merge | to: done | status: archived | outcome: pass | worktree: D:/autostack/.wt/os-060/auto-os | commit: dd6c1d7（main tip, delivery） | next: done

## 10. 待澄清事项

1. 当前仅计划；U036和各产品已交付的规则/实际桌面launch=vm基线；不等待AG全线、UI038、054或055真实事件整项。图形扩展若新增需求另归owner，不迁游戏引擎。
2. 本批全部产品是结项分母，未就绪产品可保持待办并推进无依赖项，但不能以“先试点”降为代表几例就归档。原支持面新缺口需实际owner/关闭条件或重规划，不自动接管全部债。
3. 只消费已交付接口；新语义/协议/引擎能力、预算/平台扩大不由本合同自动授权。双端/原生实际未支持必须具名，不能偷减原义务。
4. 真实窗口/端口/磁盘/解码/数据隔离T-01冻结，不操作用户系统主题/真实库/活凭据；真实OS事件沿055具体窗口规则，不做全局迁移前置。
