---
plan_id: PLAN-059
status: archived
feature_name: 正式Reader产品的主题与阅读表面接入
author: [Codex]
created_at: 2026-11-03
updated_at: 2026-11-04
plan_revision: 1
current_step: 9
total_steps: 9
supersedes_spec_components: [docs/specs/shell/host-theme-policy-and-delivery.md, docs/specs/apps/independent-repositories.md]
new_spec_components: [docs/specs/apps/product-theme-batch-059.md]
touched_goals: []
affects: [auto-reader, product-theme-adoption, os-theme-combination]
depends_on: [auto-os/PLAN-053, auto-ui/PLAN-031, auto-ui/PLAN-033, auto-reader/READER-007]
---

# [auto-os/PLAN-059] 正式Reader产品的主题与阅读表面接入

## 0. 变更摘要

以apps.manifest稳定ID为边界，完整接入本批1个实际产品：**018-book-reader**。采用036正式包/生成输入与053/055实际宿主端口，迁产品当前真实功能，不从auto-ui教学例复制业务或拿其通过抵产品验收。单一跨仓主合同在OS，参与产品执行时加README指针，不另造同一主题竞争合同。

drafting r1、0/9。2026-11-03用户授权全产品调研立项/路线刷新，**暂不实施**；本轮未建树/改产品或OS pin/运行构建、测试、GUI/系统设置/远端动作，不创建或联系其他agent/chat、不改canonical。其他会话WIP保留。以下全部新运行门未执行。

## 1. 目标

- G1：每个产品同一正式主题包的两个族×明暗真实生效，保默认/品牌/显式override/follow优先级及原支持轨，不只换外壳源代码。
- G2：活跃业务/输入/草稿/会话/内容/计时与持久化跨换肤保持；原完整适用门、时序/性能条件和负控不减。
- G3：实际宿主启动/独立运行/多App隔离、错误拒绝/恢复/owned退出真实闭环；候选源码到正式产品与组合pin同源。
- G4：本批全产品逐一交付，与036/055/054及教学迁移分工明确；不以代表App、研究/计划数冒全量迁移完成。

非目标：扩产品新业务/改数据库或Locator/协议/引擎，重做036发行算法、055真实OS事件/通用服务、054业务装配、教学037/038代码同步、全机/AG/ACE回归、所有历史债清零与远端registry。全批存在必需门未闭时不能用“历史问题”充PASS。

## 2. 架构方案

UI单源主题→036已交付发行/验证输入→产品已解析依赖与design/现适配端口→053/055实际host policy→产品/组件scope→公共控件与已支持的内嵌表面→真实可见颜色/模型和applied回读。品牌/分类overlay、文档/媒体/VT/series等内容域各自有owner与豁免依据；不复制整套palette，不给语言facade/generation-core增加UI反向依赖。

产品当前真实业务/后端profile/原测试是验收基面。当前Reader007 executing2/13，clone有未提交变更；不能用旧BookReader教学例/旧pin上的纯文本页顶替正式EPUB产品。主题适用公共壳与阅读引擎明确支持的配色，出版物原内容/图片/批注/选择颜色语义各归owner；不扩新格式或修改Locator/书库协议。 真正所需的新能力或既有非主题业务阻点先具名交owner/前置或重规划，不削减原承诺。只允许本批消费侧/已承诺主题投影的有限修复，公开协议/新引擎须另立约。

## 3. 技术栈

实际产品.at/pac/已有UI/engine/后端与原MCP/Playwright/产品验收；OS registry/launch/config/gitlink，UI正式包/scope/host transport，036输入/安装入口。准确命令/工具/版本/资源支持由T-01按真实README/Spec/help冻结，无新通用测试框架/registry。

## 4. 需求分析与背景调查

已使用auto-plan-new及verification-contract，读OS AGENTS、独立产品/主题policy Specs、UI主题/包/host transport、036/055/054和各产品实际pac/front/README/Spec/原测试/在途计划。逐入口完整来源与快照见[Design03全产品盘点](../design/03-product-theme-rollout.md)§2；该表记录28 active ID、HEAD/pin/源码摘要/dirty及原契约矛盾，不是运行证据。

manifest SHA256=57afca924fb95d99a29294163f134672d660c494202296b0f63812b749b08a44；OS观察HEAD=092254ce037ef7c7612cf11fe8b3fe00c57a76c6。下表Git前缀是观察值，未来开工重核完整SHA、review/merge/remote可取性及工具内容hash，不能直接拿当前main或陈旧OS pin当已验组合。

| 稳定ID / owner | clone HEAD / OS pin | 当前真实差额/在途 | 已核验证定位与隔离要求（参数工作时重核） |
|---|---|---|---|
| 018-book-reader / auto-reader | 2fc496af / fa6185eb | 007 executing2/13，正式EPUB表面/Locator改造中 | READER-007正式交付的阅读/Locator/输入/恢复完整入口；旧tests/real-book.spec.ts不得代新引擎验收 |

U036，以及READER-007正式交付的阅读表面/Locator v2/来源支持合同；原计划所需UI029接口按Reader正式组合接受，不借Reader主检出WIP。可先静态核查/映射，产品修改和最终组合等读者基线/写窗释放；不阻塞媒体058。 某产品门未满足不修改其在途内容/接pin，可推进同批独立准备项；execution_done/结项必须本批全部必需面闭合。旧README/计划状态陈述过时由实际owner按收据处置，本包不代它终审或修改原合同。

## 5. 详细设计

### 5.1 启动、所有权与下一任务

实施授权待给。拟唯一OS组D:/autostack/.wt/os-059/auto-os、分支codex/plan-059-product-themes；UI/框架确有改动才各加同组唯一树。普通产品在D:/autostack/<repo>独立clone main一写入者，工作/独立review顺序共用，禁止编辑OS apps/子模块或复制第二review树。Musk按其AGENTS专属worktree，auto-down嵌套Garden按owner专属树；不能把普通产品main例外套基础仓。local syslog（若本批含）在OS树。各仓AGENTS工作时重读，不擅自stash/reset/切走他方WIP，无链接。

T-01冻结源/已交付业务与支持、实际package/CLI/tool/engine/后端、旧设置/品牌/颜色角色、原用例/样本/排除分母及端口/数据/窗口预算；用真实整链合法/拒绝控制证实接线可行性再做依赖实现。无正式来源的WIP不消费；若真正缺能力/需要改公开支持，给精确owner/关闭条件与合同修订，不静默降级成静态Demo。

### 5.2 本批实现与区分性不变量

先公共书架/工具栏/设置，再经007正式引擎端口接阅读配色/现高亮对比度；全部在Reader一写入者的独立clone顺序完成，OS只接固定产物/pin。

真实EPUB→目录/图片/脚注→精确选区批注→换主题/字号重排→回访/冷恢复，位置/引用原文/批注ID与数据不丢；旧TXT/Markdown和Locator不匹配拒绝原门保留，不能退回纯文本假绿。

逐产品列公共role/交互状态、品牌与分类色、内容域和合法固定值表；保现默认与其对应业务语义。每个产品真实Vue/VM及已承诺native/host路径分别核，不能从生成成功推定原生特殊组件可用。换肤前后固定业务哨兵与实际颜色/引擎选择读回，两族×明暗逐产品完成；需要主题化的可用嵌入表面通过原端口消费，不能只改外层截图。内容/媒体像素/文档身份不因“统一”被染色。

宿主跟随/覆盖/非法包/乱序拒绝保旧、重开或重连追最新，两个实际产品相反族/模式不串。包/I/O失败或未绘制不能回报applied；恢复后下一次真实操作成功，关闭只收割本次owned连接/进程/订阅。系统真实事件归055，本批不修改用户全局设置。

### 5.3 外部来源、候选与正式落地

036完整交付是采用新渠道的产品实施/最终消费门；T-01可获授权后先盘点，绝不借其execution_done实现树。通过正式安装/包输入做fresh/locked/第二绝对根、缺件/错hash/合法同mtime内容变更及恢复，绑定实际产品/产物；不手拷教学palette、不用原target充外部构建，910项目/member/context/lease与可写target隔离保持。

每产品先形成已提交候选，原门/主题/实际桌面支持组合验收；独立review后正常push，OS干净组合fetch/detach到同SHA、健康门+真实启动后才更新本批pin。外部无gitlink条目按真实repo/嵌套根HEAD、daemon/tool/资产身份固定并验启动，不伪造新submodule。仅修改本批增量，最新OS其他pin/注册/功能零意外覆盖；056正式pin等054先交付，后继必须是其审定业务版本的主题后代，不互相回退。

本批交接按Design03 §3/6：036拥有发行/输入、055拥有clock/G-1/G-2（仅所需outproc能力正式交付作为该腿门）、054拥有八基础业务装配；UI037/038教学线不抵产品。每个产品剩余问题/未支持平台具名，不把原支持义务删掉；真实依赖/source变更按918规则正常审计，不永久豁免。

### 规范增量

| delta_id | add/modify/retire | docs/specs/... target | before/after rule | rationale | acceptance IDs |
|---|---|---|---|---|---|
| SD-01 | add | docs/specs/apps/product-theme-batch-059.md（新） | 本批产品实际source/pin/profile/主题支持与剩余边界 | 组合不以Demo代验 | AC-01..05 |
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
| AC-01 | 本批全部1产品正式包/默认/品牌/两族×明暗、真实已承诺运行轨逐项通过，V-01 | work |
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
| T-01 | 授权后核本批全部source/原合同/写入者/前置，唯一组/owner clone、scope，准确命令/资源/表面映射与整链合法/失败探针；阻点owner有据 [✅ 已完成]（2026-11-04：U036 archived 10/10、READER-007 archived 12/13、reader main@4c93d1da clean 写窗释放——§4 两前置全部满足；os-059 组冻结 lang 2f558fc93/ui e06b65b2/web 3de5524/down 7c4d5cbb + auto-man 2bbd9542 安装版 auto.exe r7-t19-17-g2bbd954 + 组树重建 auto-ui-tool.exe；包源 0.2.0 逐文件 sha256 全 match、聚合 digest 9fa07742…838e 逐位一致；基线 boot 探针 backend 10s + front html PASS，`plan-artifacts/plan-059/auto-os/t01-freeze.md`） | 授权已给（/goal 完成计划059）；§4各产品门；AC-01..05 | work |
| T-02 | U036后按§5.2第一职责段接产品正式包/默认/品牌/可用内嵌主题端口，保原业务，新增定向反例 [✅ 已完成 2026-11-04]（reader 独立 clone：pac `dep themes{vendor/themes}` 固定快照 0.2.0 digest 9fa07742…838e 逐文件校验、`use themes.themes: Auto`+App `design{theme:"Auto"}`+`dark_mode` App 级保留名位（notes 先例）、壳面语义 token 零改动随包；**框架投影修复**：vue 轨 design.theme 注入门控 accent/theme_name 双臂静默截断→三臂扩（auto-ui 组树 `ui_gen/vue/script.rs`，分支 codex/plan-059-vue-design-theme-gate@ad944951，回归测 2+compiler lib 1138/0，tools lib 红集==base e06b65b2 逐一同败零新红）；腿 A/B 像素精确 #09090b/#ffffff） | T-01+U036及对应产品门；AC-01/02/03 | work |
| T-03 | 完成本批其余职责段/产品公共及专用支持面，宿主采纳/独立跟随、原交互/恢复，有限已有主题缺陷有反例 [✅ 已完成 2026-11-04]（阅读配色经 007 端口接入：显式书级选择>应用模式派生>缺省 light，`/api/reader/command` op:settings 下发+`__reader.appliedSettings` 读回；**既有缺口修复有反例**：冷恢复不向引擎端口补发主题→Init 末段补发；引擎文档 html 背景精确 oracle=shadow root 递归下钻（#1f2430 dark/#f4ecd8 sepia）；VM 轨 standalone follow 臂实证（跟随臂仅 iced 路径——vue 轨无此臂，F/FB/FR/S2 全走 VM 轨：Solarized/dark #002b36 精确、坏 digest 具名拒绝保旧 #ffffff、重开追最新 #09090b、sepia 显式>follow；dark_mode/theme 状态哨兵过） | T-02；AC-01..03 | work |
| T-04 | 本批每产品真实两族明暗/原业务/多App/错误/owned退出完整并集，记录首红/条件排除 [✅ 已完成 2026-11-04]（七腿套件全绿 exit 0（vue：A 暗覆盖/B 明默认/S1 sepia 显式+持久；vm：F Solarized 跟随/FB 坏包拒绝保旧/FR 重开追最新/S2 显式>跟随）；原门并集：auto test 7/7、verify_epub 22/0、verify_standalone vue 28/0（含 verify_local vue 50/50+R-06 毁损反控）、vm 27 checks/2 fail=原生驱动既有崩（pristine 4c93d1da 逐一复现同败，非本批引入，具名）；V-03 双产品相反腿：reader@Solarized/dark #002b36 + photos@Auto/light #ffffff 并行零串扰、reader 重开 Auto/dark #09090b、photos 零位移） | T-03；AC-01..03/05，V-01..03/05 | work |
| T-05 | 外部包/输入/移根/失效恢复，全部已提交产品候选/候选组合健康与实际启动，同SHA冻结；056组合门等U054 [✅ 已完成 2026-11-04]（V-04 包源消费腿：vendor 快照同 mtime 篡改→boot 具名阻断不发半成品、还原→恢复真实消费、`.theme-inputs.json` receipt digest=9fa07742…838e 精确（036 §3 inputs 身份）；第二绝对根=跟随臂 AUTOOS_THEME_PACKAGE_DIR 组树包源已证；**候选冻结：auto-reader main `17de181`**（main，7 文件 +588/−1，工作树 clean）；OS pin 更新+组合健康/实际启动=T-08/merge owning（§5.3 独立 review 后 push→fetch/detach 同 SHA→健康门+实际启动）） | T-04、§4精确前置；AC-04/05，V-04/06 | work |
| T-06 | 自查健康/遗漏/原义务/全批AC/SD草稿、source/pin/其他计划交接冻结；execution_done6/9 | T-05；AC-01..05 | work |
| T-07 | 同组/同clone冻结、独立上下文final review，发现返write owner并失效相应证据，不造第二实现树 | T-06；AC-06，V-07 | review |
| T-08 | 冻结必要职责并集、依赖序落地/产品正常push/OS干净组合pin或外部源读回、健康/实际启动/canonical/路线/归档；不覆盖054/其他新提交 | T-07 pass+§4组合门；AC-07，V-08 | merge |
| T-09 | 全输出/设置/owned进程对账，摘要折Plan删统一scope/自有树产物，各树guard clean后移除/分支/空组；保产品clone，archived9/9 | T-08；AC-08，V-08 | merge |

材料唯一D:/autostack/plan-artifacts/plan-059/auto-os/，按repo/owner/用途/在途登记；数据/库/文件/媒体/设置备份/日志截图/外部SDK/生成副本/显式target皆该scope，普通编译在唯一树，产品合法输出全列账。禁止默认temp/仓内attachments/其他盘散落证据。merge逐项核全部实际输出路径及活动归属，命令/退出码/关键值/源hash/未解项折Plan后删本scope/自有树产物；仅具名未闭持有有owner/理由/关闭条件，不能借持有将必需面判PASS。未知/活动/他方不盲删，产品clone不删/reset，legacy specs.json不写。

## 9. 复审记录

2026-11-03 / stage:new / auto-os/PLAN-059 r1 / outcome:blocked（本轮只调研立约，实施授权与§4精确前置待满足） / 已从28条manifest全清单确认本批1个实际入口/owner/HEAD与旧OS pin差额、现源码与原契约/测试；不以教学替产品、普通暗亮代正式主题、当前main代交付。所有运行/新反控均未执行。next：授权后work/T-01重核真实来源/写窗/资源和前置，不重复请求已确定范围，超公开支持/owner变更再提出具体修订。

静态预检pass：0–10章节、9T/8AC/8V/4SD唯一连续，所有AC/验证/任务归属有据；本批与其他批无重复，合28 active ID全覆盖；本号活动/归档唯一、OS号源062，本地引用/限定diff/尾空格检查通过。仅文档检查，所有项目运行/新负控/最终独立复审仍未执行。

- stage: work | plan_id: auto-os/PLAN-059 | plan_revision: r1 | outcome: pass（execution_done 6/9）| 执行位: 产品=D:/autostack/auto-reader main `17de181`（唯一写入者，树 clean）；OS=组树 `.wt/os-059/auto-os` 分支 codex/plan-059-product-themes（SD-01/02/04 草稿 commit）；UI=组树 `.wt/os-059/auto-ui` 分支 codex/plan-059-vue-design-theme-gate `ad944951`（树 clean）| base_commit: reader 4c93d1da / ui e06b65b2（036 交付线）/ os 0c715dc | dependency_revisions: auto-man 2bbd9542（安装版 auto.exe r7-t19-17-g2bbd954）、auto-lang 2f558fc93、auto-web 3de5524e、auto-down 7c4d5cbb（组树 detached） | task_ids: T-01..T-06 | evidence: ①前置重核 U036 archived 10/10+READER-007 archived 12/13（§4 两前置满足，写窗释放）②七腿反控全绿 exit 0（`tests/test_059_theme.py`：vue A/B/S1 像素精确 #09090b/#ffffff/sepia + 引擎端口读回 + 引擎文档 html 精确 #1f2430/#f4ecd8（shadow 递归 oracle）；vm F/FB/FR/S2 #002b36/#ffffff/#09090b + dark_mode/theme 状态哨兵 + 具名拒绝日志行）③原门并集：auto test 7/7、verify_epub 22/0、verify_standalone vue 28/0（内含 verify_local vue 50/50+R-06 毁损反控双向）、vm 27/2（2 fail=原生驱动 UI-find 崩，**pristine 4c93d1da 逐一复现同败，非本批引入，owner=reader 环境/框架**）④V-03 双产品相反腿 PASS（reader+photos 并行零串扰+重开恢复）⑤V-04 包源腿 PASS（同 mtime 篡改具名阻断/还原恢复/receipt digest 9fa07742…838e）⑥框架投影修复：ui `ad944951` 门控三臂扩+回归测 2+compiler lib 1138/0，tools lib 红==base e06b65b2 零新红（036 交付归因的既有红集）⑦被测==交付：候选提交后自冻结 ui 树重建 tool 复跑 A/F 全绿 ⑧材料 `plan-artifacts/plan-059/auto-os/`（冻结记录/腿日志/截图/探针/v03/v04 证据）| blockers: 无 work-owned 阻塞；具名移交 merge：原生驱动既有崩（pristine 同败）与 007 引擎面 CDP GAP 维持 owner 注记 | next_responsible_role: review（T-07 独立上下文 final review，同组同 clone 冻结源：reader 17de181 + ui ad944951 + os 工作树）

- stage: review | review_scope: final | reviewer_identity/limitations: **实现同会话复审**（计划未指定独立 agent/model；按 review skill 如实记录该限制，裁决自工件与独立重执行重建——非仅采信执行摘要）| plan_id: auto-os/PLAN-059 | plan_revision: r1 | outcome: **pass**（修复轮后）| worktree_path: reader main clone + os-059 组树（auto-os/auto-ui） | branch: reader main / os codex/plan-059-product-themes / ui codex/plan-059-vue-design-theme-gate | reviewed_commit: **reader 6d65224**（17de181+R-1/R-2 修复）/ ui ad944951 / os acc86006 | base_commit: reader 4c93d1da / ui e06b65b2 / os 0c715dc | dependency_revisions: man 2bbd9542 / lang 2f558fc93 / web 3de5524e / down 7c4d5cbb（全 clean） | spec_inputs: SD-01=docs/specs/apps/product-theme-batch-059.md（os acc86006）、SD-02=host-theme-policy-and-delivery.md PLAN-059 节、SD-03=reader docs/specs/theme-adoption.md（6d65224）、SD-04=independent-repositories.md 批次行 | acceptance_results: AC-01 **pass**（两族×明暗全矩阵像素逐项：Auto #ffffff/#09090b + Solarized #fdf6e3/#002b36，SL 腿=R-1 补全后实测；显式 override/默认/follow 优先级 A/S1/S2/F 实证）｜AC-02 **pass**（原门并集 verify_standalone vue 28/0 含 verify_local 50/50+毁损反控、verify_epub 22/0、auto test 7/7——review 现场复跑后两项同绿；业务哨兵全腿在位：真实书卡/阅读态持久 S1→S2/批注链=007 原门保留）｜AC-03 **pass**（follow/fb 拒绝保旧/FR 重开追最新/V-03 双产品零串扰；outproc 重连腿=N/A——reader 非 outproc app，载体闭环=055 G-2 具名）｜AC-04 **pass**（V-04：同 mtime 篡改具名阻断/还原恢复/receipt digest 9fa07742…838e 精确；第二绝对根=跟随臂组树包源）｜AC-05 **pass**（前置逐一重核 U036/READER-007 archived、被测==交付（冻结树重建 tool 复跑 A/F）、pristine 对照甄别 vm 2 fail=既有、无借 WIP/Demo 顶替）｜AC-06 **pass**（本审：AC 映射逐项证据化、断言为精确 oracle 非 exit 码、负控在位、SD 四篇与实际行为一致） | pending_closeout_gates: AC-07/08=merge owning（正常 push→OS fetch/detach 同 SHA→check-app-submodules 健康门+真实桌面启动→canonical/归档读回→输出对账清理）；批量回归不 due（本批未触发 5 倍数窗口） | findings: R-1（coverage，AC-01）Solarized×light 腿缺席→**已闭**（SL 腿实测 #fdf6e3±2 PASS，6d65224）；R-2（doc，准确性）app.at 注释误记 design 丢失根因为"位置/codegen 丢块"→**已闭**（改记真实根因=编译器注入门控截断，指认 ui ad944951）；非阻塞具名：vm 原生驱动崩（pristine 同败，owner=reader 环境/框架，后继处置）、007 引擎面 CDP GAP 维持、056 todo vue 轨像素门从未执行之事实由 SD-02 注记 | evidence: 本记录内联命令/数值；现场复跑（auto test 7/7、verify_epub 22/0、SL 腿、A/F 冻结复跑）；diff 全文审查（reader 4 文件+5 新件、ui 门控三处、os SD 三篇）；legacy ledger 零写入（.autoos 只读未触） | next: merge（T-08 冻结并集+push+OS pin+健康门+实际启动→T-09 对账清理归档）

- stage: merge | plan_id: auto-os/PLAN-059:r1 | reviewer/operator: 主会话（auto-plan-merge skill 全流程） | outcome: pass（delivered；cleanup 见 T-10/closeout） | checkpoints: **prepared**=reviewed 三树（reader 6d65224 / ui ad944951 / os acc86006）全 clean、canonical delta=已复审 SD-01..04、依赖=man 2bbd9542/lang 2f558fc93/web 3de5524/down 7c4d5cbb；**landed**=①auto-ui：分支 rebase master（plan-041 交付 7471b7c4 基）→ **cfe04ce1**（range-diff `ad944951=cfe04ce1` 全等）→ ui master ff-only 实测（主检出他方 WIP docs 零触碰零重叠）+ 重绑定 compiler lib 1139/0+回归测 2/2；②reader：`git push origin main`（46cb31f..**6d65224**）——独立 review pass 后按 §5.3 正常 push（含 007 已交付 21 提交补发）；③auto-os：os 分支（acc8600 SD 落树+f250c57 pin）rebase main（0c715dc 基未动）→ main **ff-only f250c57**（零 merge commit；主检出他方 WIP：docs/design|plans|research+apps/auto-charts gitlink 全部非重叠零触碰） | **canonical_verified**=docs/specs/apps/product-theme-batch-059.md（新）+ shell/host-theme-policy-and-delivery.md（PLAN-059 消费面节）+ apps/independent-repositories.md（059 批次行）@f250c57 与复审树逐字一致（git-tracked）；reader SD-03 theme-adoption.md+README 反链@6d65224（已随 push 在 origin） | **combination gates（AC-07）**=apps/auto-reader gitlink fa6185eb→**6d652249**（fetch/detach 同 SHA 实证：worktree `git submodule update --init`+`checkout --detach 6d65224`）；健康门 check-app-submodules.ps1：worktree 形态=25 未初始化（AGENTS 纪律设计态）+reader detached 候选；**落地后主检出门**=reader 工作树实体化于 6d652249（`git submodule status` 同步干净）、`missing_pac_at=0`（27 manifest 注册源全覆盖）；**真实桌面启动**=desktop.sh vue（os-059 组树 lang/ui/man 源+desktop-target 构建）→ desktop 进程 `auto.exe run --desktop` PID 32040（15:54:34 起）存活、apps-registry `(1 apps)`=pinned reader、vite :3000 就绪——证据后按 owned 进程收割 | **landing≠deployment 观察**：主检出无共享生产桌面二进制消费 master（桌面按需自源构建；预存 058 rqhost 实例 PID 28976 属他方会话世界未触碰）；ui master 推送=仓主常规同步流（本计划未 push ui master——其上还有他方未推送提交，不代发） | **archived**=本文件移 docs/plans/archived/（status: archived 9/9，completion_kind=delivered）并随归档 commit 进 default branch | **cleaned**=见 T-09/closeout 收尾记录（guard→树/分支/组目录→plan-artifacts scope 删除→registry 刷新） | next: closeout（清理+对账）

## 10. 待澄清事项

1. 当前仅计划；U036，以及READER-007正式交付的阅读表面/Locator v2/来源支持合同；原计划所需UI029接口按Reader正式组合接受，不借Reader主检出WIP。可先静态核查/映射，产品修改和最终组合等读者基线/写窗释放；不阻塞媒体058。
2. 本批全部产品是结项分母，未就绪产品可保持待办并推进无依赖项，但不能以“先试点”降为代表几例就归档。原支持面新缺口需实际owner/关闭条件或重规划，不自动接管全部债。
3. 只消费已交付接口；新语义/协议/引擎能力、预算/平台扩大不由本合同自动授权。双端/原生实际未支持必须具名，不能偷减原义务。
4. 真实窗口/端口/磁盘/解码/数据隔离T-01冻结，不操作用户系统主题/真实库/活凭据；真实OS事件沿055具体窗口规则，不做全局迁移前置。
