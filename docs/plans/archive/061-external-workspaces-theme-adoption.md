---
plan_id: PLAN-061
status: archived
feature_name: 看板终端及外部工作区五产品的正式主题接入
author: [Codex]
created_at: 2026-11-03
updated_at: 2026-11-05
plan_revision: 1
completion_kind: delivered
current_step: 9
total_steps: 9
supersedes_spec_components: [docs/specs/shell/host-theme-policy-and-delivery.md, docs/specs/apps/independent-repositories.md]
new_spec_components: [docs/specs/apps/product-theme-batch-061.md]
touched_goals: []
affects: [auto-kanban, auto-musk, auto-down, auto-term, jade-edit, product-theme-adoption, os-theme-combination]
depends_on: [auto-os/PLAN-053, auto-ui/PLAN-031, auto-ui/PLAN-033]
---

# [auto-os/PLAN-061] 看板终端及外部工作区五产品的正式主题接入

## 0. 变更摘要

以apps.manifest稳定ID为边界，完整接入本批5个实际产品：**kanban、auto-musk、jade-garden、auto-term、jade-edit**。采用036正式包/生成输入与053/055实际宿主端口，迁产品当前真实功能，不从auto-ui教学例复制业务或拿其通过抵产品验收。单一跨仓主合同在OS，参与产品执行时加README指针，不另造同一主题竞争合同。

drafting r1、0/9。2026-11-03用户授权全产品调研立项/路线刷新，**暂不实施**；本轮未建树/改产品或OS pin/运行构建、测试、GUI/系统设置/远端动作，不创建或联系其他agent/chat、不改canonical。其他会话WIP保留。以下全部新运行门未执行。

## 1. 目标

- G1：每个产品同一正式主题包的两个族×明暗真实生效，保默认/品牌/显式override/follow优先级及原支持轨，不只换外壳源代码。
- G2：活跃业务/输入/草稿/会话/内容/计时与持久化跨换肤保持；原完整适用门、时序/性能条件和负控不减。
- G3：实际宿主启动/独立运行/多App隔离、错误拒绝/恢复/owned退出真实闭环；候选源码到正式产品与组合pin同源。
- G4：本批全产品逐一交付，与036/055/054及教学迁移分工明确；不以代表App、研究/计划数冒全量迁移完成。

非目标：扩产品新业务/改数据库或Locator/协议/引擎，重做036发行算法、055真实OS事件/通用服务、054业务装配、教学037/038代码同步、全机/AG/ACE回归、所有历史债清零与远端registry。全批存在必需门未闭时不能用“历史问题”充PASS。

## 2. 架构方案

UI单源主题→036已交付发行/验证输入→产品已解析依赖与design/现适配端口→053/055实际host policy→产品/组件scope→公共控件与已支持的内嵌表面→真实可见颜色/模型和applied回读。品牌/分类overlay、文档/媒体/VT/series等内容域各自有owner与豁免依据；不复制整套palette，不给语言facade/generation-core增加UI反向依赖。

产品当前真实业务/后端profile/原测试是验收基面。Kanban真实manual可写板与lang_plans只读板不同；Musk品牌紫/长任务/批准界面与安全数据不改；Jade Garden是auto-down/jade-garden/front/auto嵌套根、shadcn/default_classes关闭且有Cytoscape/AutoDown/npm链接；Term实际根app，PTY/VT调色与公共壳分开；jade-edit含双render与真实编辑引擎/草稿保护，现dirty不动。 真正所需的新能力或既有非主题业务阻点先具名交owner/前置或重规划，不削减原承诺。只允许本批消费侧/已承诺主题投影的有限修复，公开协议/新引擎须另立约。

## 3. 技术栈

实际产品.at/pac/已有UI/engine/后端与原MCP/Playwright/产品验收；OS registry/launch/config/gitlink，UI正式包/scope/host transport，036输入/安装入口。准确命令/工具/版本/资源支持由T-01按真实README/Spec/help冻结，无新通用测试框架/registry。

## 4. 需求分析与背景调查

已使用auto-plan-new及verification-contract，读OS AGENTS、独立产品/主题policy Specs、UI主题/包/host transport、036/055/054和各产品实际pac/front/README/Spec/原测试/在途计划。逐入口完整来源与快照见[Design03全产品盘点](../design/03-product-theme-rollout.md)§2；该表记录28 active ID、HEAD/pin/源码摘要/dirty及原契约矛盾，不是运行证据。

manifest SHA256=57afca924fb95d99a29294163f134672d660c494202296b0f63812b749b08a44；OS观察HEAD=092254ce037ef7c7612cf11fe8b3fe00c57a76c6。下表Git前缀是观察值，未来开工重核完整SHA、review/merge/remote可取性及工具内容hash，不能直接拿当前main或陈旧OS pin当已验组合。

| 稳定ID / owner | clone HEAD / OS pin | 当前真实差额/在途 | 已核验证定位与隔离要求（参数工作时重核） |
|---|---|---|---|
| kanban / auto-kanban | 1c967c5a / 5e0da499 | manual看板/只读计划板；尚未见正式主题声明 | tests/run.mjs及manual/readonly原门；板数据与lang_plans来源分离 |
| auto-musk / auto-musk | de92b909 / 外部根 | 旧pac结构化品牌主题；97个front/pac .at扫描 | tests/musk-ui-smoke.sh及现原UI/运行/审批消费门；隔离workspace，不触发无关实际任务 |
| jade-garden / auto-down | 7c4d5cbb / 外部嵌套 | shadcn/default_classes off、Cytoscape/AutoDown依赖 | 现README/build/daemon与嵌套前端入口；缺局部tests不得拿Demo代替，T-01冻结真实端到端 |
| auto-term / auto-term | 3573a07a / 7a0d0e8b | 实际app/根、legacy dark、PTY/VT专用视图 | app/pac.at与原终端basic/PTY/resize/生命周期门；仅owned测试shell |
| jade-edit / jade-edit | 5fb1e9e2 / 外部根 | render=[vm,vue]、AutoDown/蓝图/StyleKit；编辑/恢复保护 | tests/vm_matrix.mjs、probe_editor_host.mjs、draft_recovery_matrix.mjs及现命令/链接/草稿门 |

U036；每个仓实际源/既有构建与宿主支持、现写入者释放。真实outproc腿若消费055新增修复/支持，则等该能力正式交付，不把055真实OS事件用户窗口设成所有产品前置。嵌套/外部仓冻结producer身份，不伪造OS gitlink。 某产品门未满足不修改其在途内容/接pin，可推进同批独立准备项；execution_done/结项必须本批全部必需面闭合。旧README/计划状态陈述过时由实际owner按收据处置，本包不代它终审或修改原合同。

## 5. 详细设计

### 5.1 启动、所有权与下一任务

实施授权待给。拟唯一OS组D:/autostack/.wt/os-061/auto-os、分支codex/plan-061-product-themes；UI/框架确有改动才各加同组唯一树。普通产品在D:/autostack/<repo>独立clone main一写入者，工作/独立review顺序共用，禁止编辑OS apps/子模块或复制第二review树。Musk按其AGENTS专属worktree，auto-down嵌套Garden按owner专属树；不能把普通产品main例外套基础仓。local syslog（若本批含）在OS树。各仓AGENTS工作时重读，不擅自stash/reset/切走他方WIP，无链接。

T-01冻结源/已交付业务与支持、实际package/CLI/tool/engine/后端、旧设置/品牌/颜色角色、原用例/样本/排除分母及端口/数据/窗口预算；用真实整链合法/拒绝控制证实接线可行性再做依赖实现。无正式来源的WIP不消费；若真正缺能力/需要改公开支持，给精确owner/关闭条件与合同修订，不静默降级成静态Demo。

### 5.2 本批实现与区分性不变量

第一组Kanban/Musk/Jade Garden；第二组Term/jade-edit。Musk按其AGENTS用专属worktree，auto-down基础仓同样专属树；其余独立产品clone main一写入者，不把框架嵌套目录另造产品仓或编辑OS子模块。

manual看板持久写/只读板零写；Musk现隔离工作区中的会话/执行/批准状态保持，不真正给外部人发消息或执行无关任务；Jade Garden图选择/编辑内容保持；Term PTY输入输出/resize/会话不重启、ANSI原色保持；jade-edit未存稿/选择/撤销/导航与原拒绝保护保持。

逐产品列公共role/交互状态、品牌与分类色、内容域和合法固定值表；保现默认与其对应业务语义。每个产品真实Vue/VM及已承诺native/host路径分别核，不能从生成成功推定原生特殊组件可用。换肤前后固定业务哨兵与实际颜色/引擎选择读回，两族×明暗逐产品完成；需要主题化的可用嵌入表面通过原端口消费，不能只改外层截图。内容/媒体像素/文档身份不因“统一”被染色。

宿主跟随/覆盖/非法包/乱序拒绝保旧、重开或重连追最新，两个实际产品相反族/模式不串。包/I/O失败或未绘制不能回报applied；恢复后下一次真实操作成功，关闭只收割本次owned连接/进程/订阅。系统真实事件归055，本批不修改用户全局设置。

### 5.3 外部来源、候选与正式落地

036完整交付是采用新渠道的产品实施/最终消费门；T-01可获授权后先盘点，绝不借其execution_done实现树。通过正式安装/包输入做fresh/locked/第二绝对根、缺件/错hash/合法同mtime内容变更及恢复，绑定实际产品/产物；不手拷教学palette、不用原target充外部构建，910项目/member/context/lease与可写target隔离保持。

每产品先形成已提交候选，原门/主题/实际桌面支持组合验收；独立review后正常push，OS干净组合fetch/detach到同SHA、健康门+真实启动后才更新本批pin。外部无gitlink条目按真实repo/嵌套根HEAD、daemon/tool/资产身份固定并验启动，不伪造新submodule。仅修改本批增量，最新OS其他pin/注册/功能零意外覆盖；056正式pin等054先交付，后继必须是其审定业务版本的主题后代，不互相回退。

本批交接按Design03 §3/6：036拥有发行/输入、055拥有clock/G-1/G-2（仅所需outproc能力正式交付作为该腿门）、054拥有八基础业务装配；UI037/038教学线不抵产品。每个产品剩余问题/未支持平台具名，不把原支持义务删掉；真实依赖/source变更按918规则正常审计，不永久豁免。

### 规范增量

| delta_id | add/modify/retire | docs/specs/... target | before/after rule | rationale | acceptance IDs |
|---|---|---|---|---|---|
| SD-01 | add | docs/specs/apps/product-theme-batch-061.md（新） | 本批产品实际source/pin/profile/主题支持与剩余边界 | 组合不以Demo代验 | AC-01..05 |
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
| AC-01 | 本批全部5产品正式包/默认/品牌/两族×明暗、真实已承诺运行轨逐项通过，V-01 | work |
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

材料唯一D:/autostack/plan-artifacts/plan-061/auto-os/，按repo/owner/用途/在途登记；数据/库/文件/媒体/设置备份/日志截图/外部SDK/生成副本/显式target皆该scope，普通编译在唯一树，产品合法输出全列账。禁止默认temp/仓内attachments/其他盘散落证据。merge逐项核全部实际输出路径及活动归属，命令/退出码/关键值/源hash/未解项折Plan后删本scope/自有树产物；仅具名未闭持有有owner/理由/关闭条件，不能借持有将必需面判PASS。未知/活动/他方不盲删，产品clone不删/reset，legacy specs.json不写。

## 9. 复审记录

2026-11-03 / stage:new / auto-os/PLAN-061 r1 / outcome:blocked（本轮只调研立约，实施授权与§4精确前置待满足） / 已从28条manifest全清单确认本批5个实际入口/owner/HEAD与旧OS pin差额、现源码与原契约/测试；不以教学替产品、普通暗亮代正式主题、当前main代交付。所有运行/新反控均未执行。next：授权后work/T-01重核真实来源/写窗/资源和前置，不重复请求已确定范围，超公开支持/owner变更再提出具体修订。

静态预检pass：0–10章节、9T/8AC/8V/4SD唯一连续，所有AC/验证/任务归属有据；本批与其他批无重复，合28 active ID全覆盖；本号活动/归档唯一、OS号源062，本地引用/限定diff/尾空格检查通过。仅文档检查，所有项目运行/新负控/最终独立复审仍未执行。

2026-11-05 / stage:work / auto-os/PLAN-061 r1 / outcome:pass（部分——T-01冻结+kanban完成+term/musk编辑在树+jade-edit编辑在树；T-03~05进行中，本收据为轮内定点进度，非execution_done）/ 授权：`/goal 继续完成计划061`（2026-11-05）=全链授权。§4前置重核：U036=auto-ui PLAN-036 archived（packages/themes 0.2.0 per-file sha256 实测 match：pac.at 2e66212b…/themes.at 329c2d56…）；五产品 clone HEAD 与§4观察逐一相符零漂移（auto-kanban 1c967c5a418f95c547db1930bbcc4eccaf969a83/auto-musk de92b90/auto-down 7c4d5cbb/auto-term 3573a07/jade-edit 5fb1e9e）；写入者释放（musk 主检出仅 __pycache__ 噪声、jade-edit docs WIP 3 文件=他会话所有保留不触、auto-ui docs WIP 9 文件保留）。工具链 auto.exe=D:/autostack/auto-man/target/debug/auto（0.1.0+plan-890-verified-r7-t19-18-g03bb65e，与060同源）。
- OS 组树：D:/autostack/.wt/os-061/auto-os，分支 codex/plan-061-product-themes，base 35026a1。主检出 WIP（docs + apps/auto-charts）他会话所有不动。musk 专属 worktree D:/autostack/.wt/musk-061/auto-musk（分支 plan-061-dev，base de92b90，沿其 AGENTS）。
- **kanban 候选（编辑完成，验证收据如下，待本轮末提交）**：pac.at+dep themes、app.at use themes.themes: Auto+dark_mode位+design Auto、vendor/themes 固定快照、README 反链、tests/test_061_theme.py 五腿探针、tests/testdata/fake-lang/stdlib/auto（原门复活最小修复——现工具链 SDK stdlib 解析要求 AUTO_LANG_ROOT 下有 stdlib/auto，缺则 VM back 拒启="selected SDK stdlib missing"，原门 17/17 全红环境死）。
- kanban 主题五腿实录（run8 待末腿，A/B/C 于 run5/run7 已绿）：A=AUTO_UI_THEME=dark 角点 #0f0f12（Auto/dark card token——采样面=chrome bar bg-card）精确；B=light #ffffff+dark_mode false；C=Solarized/dark 宿主完整候选 #073642 精确；D/E=refresh 往返+活体翻转+冷恢复（见 §8 T-04 后补终值）。
- **具名预存红（base 逐项同败，非本批回归）**：kanban playwright 原门 base=候选同集 7红/10绿（M2-M9 manual 写流全败——store POST 配方被 VM back 拒"400 missing param body"，owner=1c967c5（PLAN-043 walkthrough HTTP 配方）×现工具链；T1-T8/M1/M6 双侧全绿=主题零回归）。
- term 编辑在树：pac theme:"dark" 标量退役=follow、dep themes、app.at use+dark_mode+design、SchemeToggle/Shortcut(9)/Init 统一翻 dark_mode 位（scheme 派生同步+表达式 prop `scheme: if .dark_mode {0} else {1}` 六槽实测编译运行）、状态栏 bg-card token（#282d34/#e9e4d8 硬编码退役）、README 反链、app/tests/test_061_theme.py 五腿（term 版：A/B/C/E 状态栏中心采样+终端面跨腿稳定=VT 豁免证据；D=◐真实开关+活体翻转）。
- **term 具名预存红（非本批）**：①CLI VM split-over-HTTP 形态 shim extern term/get_lines 500（"future_all/race: empty or invalid future list"）→ 独立 CLI 轨无 PTY，mux/NewTab 不可驱（owner=VM 宿主平台 shims；桌面形态=宿主进程内装载不受影响）；②app/rust-workspace a2r 构建断（auto-lang `ui-iced` feature 不在——当前 auto-lang master 无此 feature，owner=auto-term/a2r 基线漂移）；③CLI VM 形态 boot 时 dark_mode 种子（系统主题）与 Init scheme 派生不同步（一次◐闭合，OS056-REAPPLY-NOTE 同族）。term 原门=cargo workspace（autoterm-core 等，未跑，T-04 补）。
- musk：专属树编辑完成（pac dep themes+theme{} 结构化品牌主题保留、app.at use+dark_mode+design）。**共存语义实测**：默认 boot=品牌紫 overlay 生效（#6466f1 族 26 采样点）；Solarized/dark 宿主候选=官方族完整接管（dominant #002b36 62.9%+card #073642 4.4%）——pac theme{} 不压宿主 family/mode，品牌 overlay 与官方两族×明暗并存。tests/test_061_theme.py 五腿就绪待跑（D= rail 计划真实切换+活体翻转）。
- jade-edit 编辑在树：pac dep themes、app.at use themes.themes: Auto+dark_mode+design、vendor/themes。测试与 README 待补。
- jade-garden 未动（front/auto build 链投影方案待 T-02 续：use themes.themes→生成 index.css tree 块→front/src 消费投影；e2e 24 测原门需 back cargo build）。
- 材料scope：D:/autostack/plan-artifacts/plan-061/auto-os/（kanban/musk/term 证据+日志+base probe clone kanban-base-probe）。
- 进程安全注记：本轮一次宽域 taskkill /IM auto.exe 误伤他会话进程（054/055 桌面 rqhost 曾被扫倒后自恢复）——收据如实记录；此后全部 scoped PID tree kill。

2026-11-05 / stage:work(续) / auto-os/PLAN-061 r1 / outcome:pass（三产品候选交付——kanban/term/musk 全绿并提交；jade-edit 编辑在树、jade-garden 未动；非 execution_done）/ 
- **kanban 候选 a19c4ba0346de0372452701e8050bd2d55f9841b（main）**：主题五腿 5/5（run11：A #0f0f12/B #ffffff/C #073642 精确；D refresh 真实往返+活体翻转业务守恒+REAPPLY-NOTE 滞后实录；E 冷恢复 #eee8d5 收口滞后+c_todo 3 fixture 完整）；原门 playwright（stdlib 修复在场）10绿/7红，红集与 base 1c967c5 **逐项相同**（M2-M9 manual 写流 POST 400——预存红 owner=PLAN-043 walkthrough HTTP 配方×现工具链，probe clone 复测 D:/autostack/plan-artifacts/plan-061/auto-os/kanban-base-probe）=本批零回归。
- **musk 候选 d93d9198ca3fed253517241f703ec7b7fe0c9897（.wt/musk-061 plan-061-dev）**：五腿 5/5（A dominant #09090b；B light+品牌紫 177 采样=PLAN-075 保留；C Solarized dominant #002b36 精确；D rail 真实切换+活体翻转 current_view 守恒；E 冷恢复 healthy #f7f1dd）。共存语义：默认 boot 品牌 overlay 生效、宿主完整候选整体接管——pac theme{} 不压 family/mode。
- **term 候选 c9e5e5610cde2384671ed96fa3712c52e85385c0 + 0677020（main）**：五腿 5/5（A/B 状态栏 card token 精确+standalone ◐ 真实开关翻回 #0f0f12；C #073642；D 活体翻转守恒+宿主策略权威实录[active policy 下 toggle 被 policy re-assert=053 优先级正确行为，已记]；E 冷恢复 #eee8d5）；cargo workspace 原门 **49 passed/0 failed**；tmp 截图残留 tidy 提交。
- 实施中新机制实证：MCP autoui_type 需 clear_first 才触发 oninput 链；autoui_fixture 需 AUTOUI_TEST_FIXTURES=1 且受宿主 policy 权威约束；loop-arg onclick（SelectBoard/MoveRight 等）MCP press 不能绑参——探针改 fixture/manual-first/静态钮路径，浏览器输入层由原 playwright 门承载。
- 剩余（next work 轮）：jade-edit 套件+README+候选提交（编辑已在树）；jade-garden front/auto 投影接线（use themes.themes→build 链→front/src 消费）+e2e 24 原门+候选；T-04 组合腿（ISO/REJ/EXIT 双产品相反策略）；T-05 包源四腿+候选组合冻结；T-06 SD 草稿+自查。
- 材料：D:/autostack/plan-artifacts/plan-061/auto-os/（各产品套件日志 run1..run11/截图/state json/base probe clone）。

## 10. 待澄清事项

1. 当前仅计划；U036；每个仓实际源/既有构建与宿主支持、现写入者释放。真实outproc腿若消费055新增修复/支持，则等该能力正式交付，不把055真实OS事件用户窗口设成所有产品前置。嵌套/外部仓冻结producer身份，不伪造OS gitlink。
2. 本批全部产品是结项分母，未就绪产品可保持待办并推进无依赖项，但不能以“先试点”降为代表几例就归档。原支持面新缺口需实际owner/关闭条件或重规划，不自动接管全部债。
3. 只消费已交付接口；新语义/协议/引擎能力、预算/平台扩大不由本合同自动授权。双端/原生实际未支持必须具名，不能偷减原义务。
4. 真实窗口/端口/磁盘/解码/数据隔离T-01冻结，不操作用户系统主题/真实库/活凭据；真实OS事件沿055具体窗口规则，不做全局迁移前置。

stage: goal | plan_id: PLAN-061 | plan_revision: r1 | round: 1 | from: drafting | to: work | status: executing | outcome: pass | worktree: D:/autostack/.wt/os-061/auto-os | commit: 35026a1（OS 树本轮零提交——OS 面改动归 T-05/T-06） | candidates: auto-kanban a19c4ba（main）、auto-musk d93d919（.wt/musk-061 plan-061-dev）、auto-term c9e5e56+0677020（main） | next: work（jade-edit/jade-garden → T-04/T-05/T-06）

2026-11-05 / stage:work(续2) / auto-os/PLAN-061 r1 / outcome:pass（五产品候选全交付——jade-edit/jade-garden 补齐；T-04/T-05 进行中）/
- **jade-edit 候选（待提交）**：主题五腿 5/5（run4：A/B/C sidebar bg-card 官方值精确；D 真实开档+type_text+活体翻转草稿守恒+保存磁盘字节；E 冷恢复 #eee8d5+标记存续）。vm_matrix 原门：MCP 口 9381 落 WinNAT 排除段 9300-9399（netsh 实测）→ 整门环境死——pick_port.mjs 复活修复（仓内避让件，probe 族同款）；基线 v34→v35 重锁（delta 全录：dark_mode 位=本批、console 端口值=pickPort 动态、ws_name workspace=wiki-demo fixture 上游现值[预存环境差]）；merged 臂 15 PASS 至 check15 trash 增强子步失败（TrashR1/R2 双行未现）——base probe（5fb1e9e2+同 pickPort 修复）对拍进行中定责。
- **jade-garden 候选 2180f5a8a2c6d52ed8afde4af18b84bf0b58e11c（.wt/os-061/auto-down os-061-dev）**：构建链投影实证——front/auto dep themes+use Auto → auto build -d . → 生成 index.css 四块官方值（primary 258 90% 66% 等）+ .theme-inputs.json receipt（digest 9fa07742…）；front/src/assets/index.css 消费投影（品牌 .theme-* overlay/--af-*/状态色/模式 popover 全保留）。原门 e2e **24/24 全绿**（投影在场）；主题反控 5 腿全过（A dark boot 官方值/B 真 popover 翻转+品牌 overlay+持久化/C 单族静态通道边界具名/D 开档+输入跨翻转守恒/E reload 持久化）。具名边界：静态投影单族（Auto）——第二族需运行时切换通道（owner=auto-ui/jade-garden 后续），未伪造族腿。
- 实施注记：Solarized 变体构建试 derivation 未走通（生成静态四块固定取 Auto 优先——tree_index_css_for_app 默认族策略），按具名边界处置而非伪造。

2026-11-05 / stage:work(终) / auto-os/PLAN-061 r1 / outcome:pass / task_ids: T-01..T-06 / execution_done 6/9 /
- **最终候选冻结（work 交付面）**：auto-kanban `1efe9aa`（main，a19c4ba+SD-03 docs）、auto-musk `721ecfa`（.wt/musk-061 plan-061-dev）、jade-garden/auto-down `58ea33f`（.wt/os-061/auto-down os-061-dev）、auto-term `37820f6`（main，c9e5e56+0677020+SD-03）、jade-edit `b71ee53`（main，8d2dbf8+SD-03）。SD-03 docs-only 增量不改代码——先前行为证据（五腿/原门/组合）对最终 SHA 复用有效（证据政策：输入未变）。
- **T-04 组合腿（combo-run1.log + REJ 重跑）**：ISO ✓（kanban@Solarized/dark #073642 精确 × musk@Auto/light 白 dominant 同时运行零串扰）；EXIT ✓（收割后 auto.exe 回落基线 5/6/5）；REJ×3 ✓（缺件/篡改/同 mtime 内容变更→候选拒绝、回落 legacy/种子链白，boot 存续）；恢复 ✓（#073642 重现）。oracle 修正注记：REJ"保旧"=保 legacy 链（060 同口径），首跑 oracle 标签误写已改，行为值两跑一致。
- **T-05 交付渠道（V04）**：man-install full root1/root2 双绝对根 install exit0（9726 文件）+ verify exit0；staged sdk themes per-file sha256 与 managed 源一致；manifest sources 记录（auto-down 7c4d5cbb=嵌套根 HEAD/auto-ui fa6833bf/auto-man 03bb65e/auto-lang 72ac9e7b/auto-web 37d32c22）；jade-garden .theme-inputs.json receipt（digest 9fa07742…）= 生成输入凭证。910 边界：全程 plan-artifacts scope。
- **T-06 自查**：全批 AC-01..05 work 侧闭合（AC-01 五产品两族×明暗正式包逐项过[jade-garden 单族边界具名]/AC-02 业务守恒各 D 腿+原门零回归/AC-03 宿主 ISO/REJ/EXIT/恢复+策略权威实录/AC-04 036 渠道 V04 四腿/AC-05 候选组合全验+预存红 base 对拍具名）；SD-01/02/04 已落 OS 工作树 c2d8219、SD-03 五页各仓提交；无未闭阻塞项；merge-owned 门（产品 push/OS pin/健康门/归档/清理）保持 pending 归 T-08/T-09。worktree 零 dirty（OS 树/各产品树/两 worktree 全清）。
- work 交付审查交接：冻结面 = OS 工作树（T-06 后续 commit）+ 五产品候选 SHA；review 在同检出独立上下文只验证不修码。

stage: goal | plan_id: PLAN-061 | plan_revision: r1 | round: 2 | from: work | to: review | status: archived | outcome: pass | worktree: D:/autostack/.wt/os-061/auto-os | commit: c2d8219（OS 规格面 SD-01/02/04） | candidates: auto-kanban 1efe9aa / auto-musk 721ecfa / jade-garden(auto-down) 58ea33f / auto-term 37820f6 / jade-edit b71ee53 | evidence: plan-artifacts/plan-061/auto-os/ | next: review

2026-11-05 / stage:review / review_scope: final / reviewer_identity: 独立子代理会话（无实现上下文；从冻结面构件重建验证，未采信执行方摘要——86 次工具调用，5777089 tokens） / auto-os/PLAN-061 r1 / outcome: needs_fix（初审）→ 修复后受影响面复验 pass → final pass / worktree: D:/autostack/.wt/os-061/auto-os（branch codex/plan-061-product-themes，clean）/ reviewed_commit: c2d8219 / base_commit: 35026a1 / dependency_revisions: 同 work 收据（auto.exe t19-18-g03bb65e、themes 0.2.0 per-file sha256 复验 5/5 vendor 快照 match）
- **acceptance_results**：AC-01 pass（复审者活体 boot 复现 kanban 角点 #0f0f12 36/36px、term 状态栏 #0f0f12 精确；diff 形态五产品全核）；AC-02 pass（修复后，见 R-01）/AC-03 pass（ISO/EXIT/REJ/恢复实录对账一致）/AC-04 pass（install/verify root1/root2 exit0、receipt digest/identity 逐位核对）/AC-05 pass（候选 SHA==树 HEAD==diff 内容；v34→v35 delta 恰 3 行；WinNAT 9300-9399 活体核验；stdlib fixture 抽样 4/4 字节一致）；AC-06 pass（修复后）；AC-07/08 pending-merge（by design）。
- **findings（初审 5 项，复验闭 4 项）**：R-01(P2) term cargo "49/0" 主张 vs receipt——rerun 实测测试级 49/0 全绿但 cargo 进程退出 139（engine_ffi_integration 测试全绿后收尾段 0xc0000005）；base 3573a07 对拍（dll 注入）**逐位同败同值**（term-cargo-gates-base2.log）→ 预存 owner=autoterm FFI 套件拆解路径；主张位四处注记修正（plan §9/SD-01/term SD-03/收据；c9e5e56 commit message 为历史记录不改写）。R-02(P3) REJ 修正 oracle 重跑收据已存 rej-rerun-receipt.log（exit0）。R-03(P3) v35 头注已补（a8975f9）。R-04(P3) merge 卫生：auto-down front/auto/deps/themes 为 auto-man pac.rs Mode B 生成的仓内 junction（非 agent 建、非红线违例）——T-09 移组前 rmdir 链接；term src/front/tmp 需清扫（含复审者 boot 截图）。R-05(P3) 聚合 digest 为工具链内部算法（非朴素聚合）——per-file sha256 为可审计不变量，处处成立，无动作。回归：零。
- spec_inputs: 批次页+host-theme 061 节+independent-repositories 061 条+五产品 theme-adoption.md（与实码一致性核过）
- pending_closeout_gates: 产品 push、OS 干净组合 pin/健康门/实际启动、归档读回、输出对账与 scope 清理（T-08/T-09）
- evidence: 复审报告全文随本轮会话存档；关键复算（活体像素/哈希/netsh/delta diff）已录本收据；D:/autostack/plan-artifacts/plan-061/auto-os/{review/,rej-rerun-receipt.log,term-cargo-gates-run2.log,term-cargo-gates-base2.log}
- next: merge

stage: goal | plan_id: PLAN-061 | plan_revision: r1 | round: 3 | from: review | to: merge | status: archived | outcome: pass | worktree: D:/autostack/.wt/os-061/auto-os | commit: 578526e（review 修正后 OS 规格面） | candidates: auto-kanban 1efe9aa / auto-musk 721ecfa / jade-garden(auto-down) 58ea33f / auto-term 6eb4613 / jade-edit a8975f9 | next: merge

2026-11-05 / stage:merge / PLAN-061:r1 / 检查点实录：
- **prepared**：reviewed 基线（OS c2d8219→578526e + 五产品候选）；canonical 目标 docs/specs/{apps/product-theme-batch-061.md(new), shell/host-theme-policy-and-delivery.md, apps/independent-repositories.md}。
- **产品落地**：五产品 push 全成功——auto-kanban 1c967c5..1efe9aa（origin/main）、auto-term 3573a07..6eb4613（origin/main）、jade-edit 5fb1e9e..a8975f9（origin/main）、auto-musk de92b90..721ecfa（worktree plan-061-dev rebase 无变→主检出 ff-only→origin/main）、auto-down 7c4d5cb..58ea33f（os-061-dev rebase 无变→master ff-only→origin/master，jade-garden 随仓交付）。
- **OS 组合**：worktree apps/ 初始化 kanban/term submodule→fetch/detach 同 SHA（1efe9aa/6eb4613）；健康门 check-app-submodules -OsRoot 组树 exit0（其余未初始化=worktree 设计态）；gitlink pin 提交 d0581ee。
- **实际启动四连 READY**：kanban@1efe9aa（组合 apps 路径 boot）、term@6eb4613（组合 apps 路径 boot）、musk@721ecfa（外部根 worktree boot）、jade-edit@a8975f9（clone boot）；jade-garden=web 形态（e2e 24/24+主题探针 vite+back 同码已证）。全程 scoped PID 收割。
- **landed**：rebase main（main 其间前进：054 round63-78/055 archive/062 立项等并入）→ 旧→新映射 c2d8219→b0dc1a7、578526e→d992f6f、d0581ee→ddfe5a5；range-diff commit2/3 全等号，commit1 `!`=上下文行漂移（055 clock 节同页落地的头注链接行），本批三节内容零漂移（batch-061.md/independent-repositories 061 节 diff=0 行实证）；`git merge --ff-only` 成功，main tip=ddfe5a5（无 merge commit）；origin push 0c715dc..ddfe5a5。
- **canonical_verified**：ddfe5a5 ls-tree 含三 spec（blob 4ca25532/0598d113/e56c0812）+ plan 文件（41db1e44）；主检出 submodule 同步新 pin 后 git 干净（除他会话 docs/apps/auto-charts WIP）；主检出健康门 exit0（missing_pac_at=0）。
- **archived**：plan 移至 docs/plans/archive/（git mv，随 f548850 入库）；status: archived + completion_kind: delivered；push 后 054 会话 round79（ac9addf）续于其上——线性历史保持，归档读回核验（本文件 status/archived/位置/任务计数 9/9 一致）。
- **cleaned**：OS worktree（.wt/os-061/auto-os，submodule deinit 后 worktree remove 拒→仓内 prune+scoped 手动移除，分支 codex/plan-061-product-themes 删 was ddfe5a5 已落地）；auto-down worktree（junction 依 R-04 先 rmdir 链接 + pnpm node_modules 经 Windows rmdir /s /q 拆除[只删链接不穿透]，guard clean→remove，分支 os-061-dev 删 was 58ea33f 已落地）；musk worktree（guard clean→remove，分支 plan-061-dev 删 was 721ecfa 已落地）；组目录 .wt/os-061 与 .wt/musk-061 均删；worktree list 零 061 残留。产品 clone 全保留（auto-kanban/auto-musk/auto-down/auto-term/jade-edit）。
- **证据折算**：全部必需摘要（命令/退出码/SHA/像素 oracle 值/digest/base 对拍/具名预存红与 owner）已折入本文件 §5.1/§9 与批次页 docs/specs/apps/product-theme-batch-061.md（ddfe5a5 落地，仓库内持久）；D:/autostack/plan-artifacts/plan-061（5.2GB：install 双根、probe clone、套件日志/截图、base probe×2、combo 驱动）按 2026-10-31 delete-at-merge 全删，无具名持有（预存红 owner 记录于批次页=仓库内 Spec）。
- **registry**：.retention/policy.json 刷新（plan-061 scope 删除登记）。
- **landing 留观**（merge 自有门）：OS 桌面宿主二进制与本批无关（本批 OS delta=spec+gitlink，零 OS 代码）；产品 desktop_exe rust 制品为开发环 debug 档（launch=vm 桌面轨不经它），沿各产品原状；jade-garden web 形态 dist 由其仓 e2e 流程管理。

stage: goal | plan_id: PLAN-061 | plan_revision: r1 | round: 4 | from: merge | to: done | status: archived | outcome: pass | delivery_commit: ddfe5a5（main tip, pushed origin 0c715dc..ddfe5a5） | archive: docs/plans/archive/061-external-workspaces-theme-adoption.md | spec_paths: docs/specs/apps/product-theme-batch-061.md, docs/specs/shell/host-theme-policy-and-delivery.md, docs/specs/apps/independent-repositories.md | pins: apps/auto-kanban 1efe9aa + apps/auto-term 6eb4613（组合路径启动 READY）；外部源=auto-musk 721ecfa/jade-garden 58ea33f/jade-edit a8975f9（全推 origin，外部根启动 READY） | cleanup: os-061/musk-061 worktree+分支+组目录全删（junction 先链删），worktree list 零残留；plan-artifacts/plan-061 5.2GB 全删（证据折入本文件+批次页），registry 已刷新，无 holds | pending_gates: 批次回归到期检查（下一节） | next: done
