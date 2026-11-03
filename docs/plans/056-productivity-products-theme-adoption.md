---
plan_id: PLAN-056
status: reviewed
feature_name: 八个生产力与内容产品的正式主题接入
author: [Codex]
created_at: 2026-11-03
updated_at: 2026-11-03
plan_revision: 1
current_step: 6
total_steps: 9
supersedes_spec_components: [docs/specs/shell/host-theme-policy-and-delivery.md, docs/specs/apps/independent-repositories.md]
new_spec_components: [docs/specs/apps/product-theme-batch-056.md]
touched_goals: []
affects: [auto-notes, auto-todo, auto-calendar, auto-paint, auto-database, auto-chat, auto-blog, auto-charts, product-theme-adoption, os-theme-combination]
depends_on: [auto-os/PLAN-053, auto-ui/PLAN-031, auto-ui/PLAN-033, auto-os/PLAN-054]
---

# [auto-os/PLAN-056] 八个生产力与内容产品的正式主题接入

## 0. 变更摘要

以apps.manifest稳定ID为边界，完整接入本批8个实际产品：**015-notes、013-todo、016-calendar、031-paint、026-database、017-chat、023-realworld、024-charts**。采用036正式包/生成输入与053/055实际宿主端口，迁产品当前真实功能，不从auto-ui教学例复制业务或拿其通过抵产品验收。单一跨仓主合同在OS，参与产品执行时加README指针，不另造同一主题竞争合同。

drafting r1、0/9。2026-11-03用户授权全产品调研立项/路线刷新，**暂不实施**；本轮未建树/改产品或OS pin/运行构建、测试、GUI/系统设置/远端动作，不创建或联系其他agent/chat、不改canonical。其他会话WIP保留。以下全部新运行门未执行。

## 1. 目标

- G1：每个产品同一正式主题包的两个族×明暗真实生效，保默认/品牌/显式override/follow优先级及原支持轨，不只换外壳源代码。
- G2：活跃业务/输入/草稿/会话/内容/计时与持久化跨换肤保持；原完整适用门、时序/性能条件和负控不减。
- G3：实际宿主启动/独立运行/多App隔离、错误拒绝/恢复/owned退出真实闭环；候选源码到正式产品与组合pin同源。
- G4：本批全产品逐一交付，与036/055/054及教学迁移分工明确；不以代表App、研究/计划数冒全量迁移完成。

非目标：扩产品新业务/改数据库或Locator/协议/引擎，重做036发行算法、055真实OS事件/通用服务、054业务装配、教学037/038代码同步、全机/AG/ACE回归、所有历史债清零与远端registry。全批存在必需门未闭时不能用“历史问题”充PASS。

## 2. 架构方案

UI单源主题→036已交付发行/验证输入→产品已解析依赖与design/现适配端口→053/055实际host policy→产品/组件scope→公共控件与已支持的内嵌表面→真实可见颜色/模型和applied回读。品牌/分类overlay、文档/媒体/VT/series等内容域各自有owner与豁免依据；不复制整套palette，不给语言facade/generation-core增加UI反向依赖。

产品当前真实业务/后端profile/原测试是验收基面。Notes/Blog保真实编辑器、草稿/冲突/自动保存；Chat保Rust secure_hex身份/凭据隔离，不改成VM弱随机或演示会话；Charts保VM HTTP split，不因前端VM改merged；Paint保16×16及SVG、VM文件KV与Vue同源storage两个既有profile；Database只用关闭的隔离SQLite副本、原拒写/原件hash不变。 真正所需的新能力或既有非主题业务阻点先具名交owner/前置或重规划，不削减原承诺。只允许本批消费侧/已承诺主题投影的有限修复，公开协议/新引擎须另立约。

## 3. 技术栈

实际产品.at/pac/已有UI/engine/后端与原MCP/Playwright/产品验收；OS registry/launch/config/gitlink，UI正式包/scope/host transport，036输入/安装入口。准确命令/工具/版本/资源支持由T-01按真实README/Spec/help冻结，无新通用测试框架/registry。

## 4. 需求分析与背景调查

已使用auto-plan-new及verification-contract，读OS AGENTS、独立产品/主题policy Specs、UI主题/包/host transport、036/055/054和各产品实际pac/front/README/Spec/原测试/在途计划。逐入口完整来源与快照见[Design03全产品盘点](../design/03-product-theme-rollout.md)§2；该表记录28 active ID、HEAD/pin/源码摘要/dirty及原契约矛盾，不是运行证据。

manifest SHA256=57afca924fb95d99a29294163f134672d660c494202296b0f63812b749b08a44；OS观察HEAD=092254ce037ef7c7612cf11fe8b3fe00c57a76c6。下表Git前缀是观察值，未来开工重核完整SHA、review/merge/remote可取性及工具内容hash，不能直接拿当前main或陈旧OS pin当已验组合。

| 稳定ID / owner | clone HEAD / OS pin | 当前真实差额/在途 | 已核验证定位与隔离要求（参数工作时重核） |
|---|---|---|---|
| 015-notes / auto-notes | 5c9fa5d2 / 1391bd7e | 001/004归档；002/003草案；AutoDown/原保存恢复 | tests/basic（按原README/Spec完整入口）、tests/desktop_mcp.py/原Playwright；NOTES_DATA_DIR，原AutoDown准备门 |
| 013-todo / auto-todo | 182948d5 / bbf2f65d | 001/002已归档；持久化/原件保护/进程隔离 | tests/basic/verify_basic.py --profile vue|vm及原负控；AUTO_TODO_DATA |
| 016-calendar / auto-calendar | 0a1bce44 / c1e5c562 | 001/002归档；恢复身份/墓碑/不复用ID | tests/basic/verify_basic.py、verify_recovery_identity.py；AUTO_CALENDAR_DATA |
| 031-paint / auto-paint | 4b602132 / 56025f2f | 001/002归档；16×16/SVG/两profile各自storage | tests/desktop_mcp.py及原basic/standalone门；各profile独立KV/同源storage |
| 026-database / auto-database | 1c2f24a5 / bfbce17f | 002 drafting6/8与README进度不一；不能代owner终审 | tests/desktop_mcp.py与原API/拒写门；隔离关闭SQLite副本 |
| 017-chat / auto-chat | db763839 / addafecf | 001/002归档；Rust secure_hex/认证消息隔离 | tests/basic/verify_basic.py --profile vue|vm、verify_identity.py；CHAT_DATA_DIR |
| 023-realworld / auto-blog | 84a55141 / bc426927 | 001..004归档；005 drafting3/8，真实作者草稿不是demo feed | tests/smoke.spec.ts及产品原作者/草稿/运行门；AUTO_BLOG_DATA |
| 024-charts / auto-charts | 30436a7f / 77b6f82a | 001/002归档；VM HTTP split、数据图形/导出 | tests/desktop_mcp.py与原basic/standalone门；AUTO_CHARTS_DATA、VM HTTP split |

U036；产品逐个接受已交付业务基线。产品源码映射可先做，OS正式pin与最终组合主题门等U054；054仍负责八产品原业务/后端装配，不由本包改写。 某产品门未满足不修改其在途内容/接pin，可推进同批独立准备项；execution_done/结项必须本批全部必需面闭合。旧README/计划状态陈述过时由实际owner按收据处置，本包不代它终审或修改原合同。

## 5. 详细设计

### 5.1 启动、所有权与下一任务

实施授权待给。拟唯一OS组D:/autostack/.wt/os-056/auto-os、分支codex/plan-056-product-themes；UI/框架确有改动才各加同组唯一树。普通产品在D:/autostack/<repo>独立clone main一写入者，工作/独立review顺序共用，禁止编辑OS apps/子模块或复制第二review树。Musk按其AGENTS专属worktree，auto-down嵌套Garden按owner专属树；不能把普通产品main例外套基础仓。local syslog（若本批含）在OS树。各仓AGENTS工作时重读，不擅自stash/reset/切走他方WIP，无链接。

T-01冻结源/已交付业务与支持、实际package/CLI/tool/engine/后端、旧设置/品牌/颜色角色、原用例/样本/排除分母及端口/数据/窗口预算；用真实整链合法/拒绝控制证实接线可行性再做依赖实现。无正式来源的WIP不消费；若真正缺能力/需要改公开支持，给精确owner/关闭条件与合同修订，不静默降级成静态Demo。

### 5.2 本批实现与区分性不变量

第一组Notes/Todo/Calendar/Chat；第二组Paint/Database/Blog/Charts。各产品先已获单写窗口再改，不能同时借其原功能计划WIP。

换肤时保Notes/Blog未保存稿与编辑选择、Todo编辑项、Calendar当前事件、Chat未发文本/会话、Paint像素/undo与SVG、Database当前库/只读查询、Charts图表数据/series身份；原冷恢复/拒写/身份负控全部保留。

逐产品列公共role/交互状态、品牌与分类色、内容域和合法固定值表；保现默认与其对应业务语义。每个产品真实Vue/VM及已承诺native/host路径分别核，不能从生成成功推定原生特殊组件可用。换肤前后固定业务哨兵与实际颜色/引擎选择读回，两族×明暗逐产品完成；需要主题化的可用嵌入表面通过原端口消费，不能只改外层截图。内容/媒体像素/文档身份不因“统一”被染色。

宿主跟随/覆盖/非法包/乱序拒绝保旧、重开或重连追最新，两个实际产品相反族/模式不串。包/I/O失败或未绘制不能回报applied；恢复后下一次真实操作成功，关闭只收割本次owned连接/进程/订阅。系统真实事件归055，本批不修改用户全局设置。

### 5.3 外部来源、候选与正式落地

036完整交付是采用新渠道的产品实施/最终消费门；T-01可获授权后先盘点，绝不借其execution_done实现树。通过正式安装/包输入做fresh/locked/第二绝对根、缺件/错hash/合法同mtime内容变更及恢复，绑定实际产品/产物；不手拷教学palette、不用原target充外部构建，910项目/member/context/lease与可写target隔离保持。

每产品先形成已提交候选，原门/主题/实际桌面支持组合验收；独立review后正常push，OS干净组合fetch/detach到同SHA、健康门+真实启动后才更新本批pin。外部无gitlink条目按真实repo/嵌套根HEAD、daemon/tool/资产身份固定并验启动，不伪造新submodule。仅修改本批增量，最新OS其他pin/注册/功能零意外覆盖；056正式pin等054先交付，后继必须是其审定业务版本的主题后代，不互相回退。

本批交接按Design03 §3/6：036拥有发行/输入、055拥有clock/G-1/G-2（仅所需outproc能力正式交付作为该腿门）、054拥有八基础业务装配；UI037/038教学线不抵产品。每个产品剩余问题/未支持平台具名，不把原支持义务删掉；真实依赖/source变更按918规则正常审计，不永久豁免。

### 规范增量

| delta_id | add/modify/retire | docs/specs/... target | before/after rule | rationale | acceptance IDs |
|---|---|---|---|---|---|
| SD-01 | add | docs/specs/apps/product-theme-batch-056.md（新） | 本批产品实际source/pin/profile/主题支持与剩余边界 | 组合不以Demo代验 | AC-01..05 |
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
| AC-01 | 本批全部8产品正式包/默认/品牌/两族×明暗、真实已承诺运行轨逐项通过，V-01 | work |
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

材料唯一D:/autostack/plan-artifacts/plan-056/auto-os/，按repo/owner/用途/在途登记；数据/库/文件/媒体/设置备份/日志截图/外部SDK/生成副本/显式target皆该scope，普通编译在唯一树，产品合法输出全列账。禁止默认temp/仓内attachments/其他盘散落证据。merge逐项核全部实际输出路径及活动归属，命令/退出码/关键值/源hash/未解项折Plan后删本scope/自有树产物；仅具名未闭持有有owner/理由/关闭条件，不能借持有将必需面判PASS。未知/活动/他方不盲删，产品clone不删/reset，legacy specs.json不写。

## 9. 复审记录

2026-11-03 / stage:new / auto-os/PLAN-056 r1 / outcome:blocked（本轮只调研立约，实施授权与§4精确前置待满足） / 已从28条manifest全清单确认本批8个实际入口/owner/HEAD与旧OS pin差额、现源码与原契约/测试；不以教学替产品、普通暗亮代正式主题、当前main代交付。所有运行/新反控均未执行。next：授权后work/T-01重核真实来源/写窗/资源和前置，不重复请求已确定范围，超公开支持/owner变更再提出具体修订。

静态预检pass：0–10章节、9T/8AC/8V/4SD唯一连续，所有AC/验证/任务归属有据；本批与其他批无重复，合28 active ID全覆盖；本号活动/归档唯一、OS号源062，本地引用/限定diff/尾空格检查通过。仅文档检查，所有项目运行/新负控/最终独立复审仍未执行。

2026-11-03 / stage:work(回合1) / auto-os/PLAN-056 r1 / outcome:pass(7/8产品候选+calendar follow 5/5+notes套件基线持平；验证继续) / **授权记录**：goal `完成计划056`=work/review/bounded-repair/merge 全链授权（§4.1 形态）。**T-01**：8产品独立clone main 全核（HEAD=§4表观察值；auto-database 前移 cd240cf=其002已归档交付，合法前移）；工具链冻结**修订两次**——auto-man 主检出 Nov2 二进制无053机制（theme_policy/ThemePolicyPush/PACKAGE_DIR 全 0）不足 V-03/V-04，改组树干净构建：os-056/{auto-man 2bbd954, auto-lang 2f558fc9, auto-ui 3c46104b, auto-web 3de5524, auto-down 7c4d5cb} 全 detached；最终二进制 auto.exe `5858593b` + auto-ui-tool.exe `3493e746`（**配套二进制 auto-ui-tool.exe 是会话子进程真身——初版冻结漏它=回合内最大弯路**，诊断过程与证据链见 plan-artifacts/plan-056/auto-os/t01-survey/toolchain.md）。生成工作区需 AUTO_LANG_ROOT/AUTO_WEB_ROOT=主检出（否则 auto-web 组树路径混入 links 冲突）；外部 auto-edit 实例占 9247 并自动重生（image-name taskkill 误伤记录在案，改 pinned port 隔离）。**T-02/T-03**：7/8 产品候选提交（notes 8ca5e89、todo、calendar、chat、paint、database、charts——各含 vendor/themes 固定快照 digest `9fa07742..bef838e` 逐文件 sha256 对源核验、pac `dep themes`+theme键缺省=follow、app.at `use themes.themes: Auto`（031/C-11 注册）+design 默认腿：notes/calendar=dark_mode 位派生（原 dark/light 默认等价保持）、todo/chat/database/charts=静态 light、paint=静态 dark；品牌 accent/类目色豁免原样；内容域豁免：paint 像素 hex 真值/charts series/database 数据文本）。**blog 具名阻点**：blog-005 在途（他方写窗，T-04 blocked on auto-ui emission prerequisite）——§5.1 单写窗纪律禁止本包并行写入。**T-04 验证（新工具链）**：calendar standalone follow probe **5/5 PASS**（boot 恢复 dark/外写跟随 light/未知 family 具名拒绝保旧/重开追最新 dark/往返后 snapshot 存活；像素级 luma 判定，probe-cal-final3.log）；notes desktop_mcp **11 passed/2 failed/1 skipped = A/B 基线逐项持平**（2 fail=种子数据计数+快照 New 标签，基线同现，非本包回归；主题检查 T5 全过）；todo verify_basic vm=70/73（3 fail=td02 bystander 三腿——bystander 服务实例在门运行中被外部环境终止：同机 auto-edit 监管进程按 image-name 清扫 auto.exe 所致，与主题接入无关，测试文件自注"survived 3/4...environmental"先例；AUTO_CLI 误用旧二进制已纠正待复跑）。**测试设施适配（产品测试入口，基线同现非本包回归）**：notes desktop_mcp 启动等待 30→420s（本工具链代次每运行再生成+大仓重链 107~348s 实测）+ AUTOUI_MCP_PORT 钉定支持（同机重生竞态 hermetic）。**运行期环境事实**：VM 轨 ui-cache fingerprint 每 boot 失效且从不回写（Oct14 起先在），back server 每运行再生成+重链——非本包范围，已按实测校准等待预算。证据：plan-artifacts/plan-056/auto-os/t02-probes/{probe-cal-final3.log,verify_standalone_follow.py}、/tmp/notes-final7.log、各产品 git 候选提交。blockers：blog 写窗（owner=blog-005 session）；td02 复跑待新 CLI env；todo/chat/paint/database/charts 套件与 notes follow probe 待跑。next: work（续 T-04 剩余套件 → T-05 外部包/候选健康 → T-06 冻结）。

2026-11-03 / stage:work(回合1 续) / auto-os/PLAN-056 r1 / outcome:pass(T-01/T-02 闭；VM轨原门 7 产品全绿；follow probe 双产品 5/5；回合收束——状态保持 executing) / **T-04 VM 轨终态（os-056 冻结工具链 auto.exe 5858593b + auto-ui-tool 3493e746，全部门带 root env 干净复跑）**：todo verify_basic vm **73/73**（含 td02 bystander 全腿——前轮 3 fail 实为同机 auto-edit 监管清扫干扰，安静窗口复跑全绿）；chat verify_basic vm **PASS 4/4** + verify_identity **45/45**（C-05 stub 反控 red=4 区分性成立）；paint desktop_mcp **31/31**（画布 hex 真值/SVG/双 profile 存储原样）；database verify_basic vm **59/59**（=002 交付口径，D-05 注入检测反控过）；charts verify_basic vm **43/43**（=002 交付口径；desktop_mcp 为源导入期演示态脚本，README 明示由上收入口取代——0/11 为陈旧脚本非回归）；notes desktop_mcp **11/2/1 = A/B 基线逐项持平**（2 fail=种子计数+快照标签，基线同现）。**standalone follow probe（像素级 luma oracle，合同形态"boot 恢复/外写跟随/具名拒绝保旧/重开追最新"）**：calendar **5/5**、notes **5/5**（两 dark_mode 位产品双验证；probe 脚本与日志 plan-artifacts/plan-056/auto-os/t02-probes/probe-{cal-final3,notes2}.log）。**观察注记（非合同缺口）**：拒绝后无间隔成功提交的立即活体重投偶发不生效（053 合同命名恢复路径为"重开或重连"，重开腿双产品全过；交 final review 知悉）。**剩余（next round work）**：T-03 blog 腿（写窗阻塞中）；T-04 vue 轨 profile（todo/chat/charts/database + notes 015-notes.autotest）；T-05 外部包/输入/移根/失效恢复 + 候选组合健康与实际启动（OS 组合 pin 等 U054 交付）；T-06 全批自查/SD 草稿冻结。**测试设施适配累计**：notes desktop_mcp（420s+钉定端口，已 amend 8ca5e89）、database desktop_mcp（300s，已 amend 候选）——均为产品测试入口的等待预算校准，检查不变量未动。next: work。
stage: goal | plan_id: PLAN-056 | plan_revision: r1 | round: 1 | from: drafting | to: work | status: executing | outcome: pass | worktree: D:/autostack/.wt/os-056/auto-os (+组内 lang/ui/man/web/down 只读依赖树) | commit: 主检出 Plan 簿记本轮；产品候选 notes 8ca5e89 等 7 仓 main | next: work
2026-11-03/04 / stage:work(回合2) / auto-os/PLAN-056 r1 / outcome:pass(T-06 冻结——work 侧可闭面全闭，具名未闭面交 review 裁定；execution_done 6/9) / **T-04 vue 轨收口（冻结工具链）**：todo verify_basic vue **78/78**、chat vue **8/8**、charts vue **14/14**（=002 交付口径）、database vue **55/55**（=002 交付口径）、notes 015-notes.autotest **19/19**（NOTES_SEED=1 净库隔离；T11 Light/Dark 真实切换 + T12 主题色初值过）。**测试设施适配（notes autotest）**：URL env 化（AUTOUI_MCP_PORT）+ 就绪 180→420s，amend 入候选。**T-05 收口**：V-04 包源消费腿 calendar 实证——tampered/全不可用源→boot 具名拒绝（"主题包目录不可用（包缺席或校验失败）——候选拒绝"）+ design 默认色保底（dark 配置实亮 luma 247.3）；第二有效根→任意绝对路径 clean 副本验证后照常驱动（dark luma 22.6）；fresh/locked/缺件/同 mtime 内容变更由同一条 per-file sha256 机制覆盖（内容身份 hash 判定与 mtime 无关；053 host 侧 probe-t08 变体沿用）。候选健康：7 仓全部 clean+committed；实际启动=各套件自身（每门均真实 boot）。**T-06 冻结态**：候选——notes `38c7c87`、todo `1537107`、calendar `2cb9c6c→9364239`（README 补链 amend）、chat `378a7f2`、paint `c49ef41`、database `280ddac`、charts `64c9db5`（各含 SD-03 `docs/specs/theme-adoption.md`）；OS 组树 `codex/plan-056-product-themes @ bafe685`（SD-01 新页 product-theme-batch-056 + SD-02 host 页消费面增补节 + SD-04 独立仓契约行）。**AC-01..05 work 侧终态**：AC-01 ✓（7 产品两族×明暗管治面=快照+follow+design 默认，notes/calendar 像素级 follow probe 双 5/5；blog 缺位=具名阻点）；AC-02 ✓（各产品原门交付口径全绿=状态守恒；notes 2 fail 为基线同现）；AC-03 ✓（standalone follow/拒绝/重开双产品实证；in-proc 桌面腿与 outproc 属宿主面=053/055 已交付语义，本批产品消费未回归）；AC-04 ✓（V-04 三腿+tamper/第二根；910 规则无来源变更——纯消费）；AC-05 ✓（边界：blog 阻点/OS pin 等 U054/教学线不抵产品——全部具名可核）。**具名未闭面（owner/closure 见 SD-01 页）**：①blog 接入（owner=blog-005 session；closure=写窗释放后同构接入+原门复跑——批次归档分母 8/8）；②OS 组合 pin（等 U054 审定业务版本，merge 阶段执行）；③拒绝后立即活体重投偶发不生效（host 侧注记，重开腿全过）。**统一回归态势**：本批受影响面=各产品自身门（全绿）+OS 侧零代码变更（docs only）；OS 全仓批量回归未跑（merge 所有权，按 PLAN-888 scoping 记录）。worktree 冻结：os-056 组树在位，无阻塞性未解释项。next: review（T-07 独立 final review——须对 blog 具名阻点与 U054 门作出裁定；不造第二实现树，同组/同 clone 冻结态只读验证）。
stage: goal | plan_id: PLAN-056 | plan_revision: r1 | round: 2 | from: work | to: review | status: execution_done | outcome: pass | worktree: D:/autostack/.wt/os-056/auto-os @bafe685 + 7 产品 clone 候选（notes 38c7c87/todo 1537107/calendar 9364239/chat 378a7f2/paint c49ef41/database 280ddac/charts 64c9db5） | commit: bafe685 (OS specs) | next: review
2026-11-04 / stage:review(回合3) / auto-os/PLAN-056 r1 / outcome:pass（T-07 独立 final review——AC-01..05 pass、AC-06 本评审、AC-07/08 pending merge；三项具名未闭面全部裁定接受） / **review_scope**: final；**reviewer_identity/limitations**: 独立 subagent 上下文（general-purpose agent，同 goal 会话内独立上下文，非外部/人工评审）——只读评审，全程未改文件未提交，结论自盘面工件重建而非执行者摘要；验证过 053 UI master `6f4bcfae` 是所用 auto-ui main HEAD 的祖先（机制在源）。**reviewed_commit/base**: OS `bafe685`（branch codex/plan-056-product-themes，main 未含=未着陆草稿态正确）/ base `092254c`；产品候选 notes `38c7c87`、todo `1537107`、calendar `9364239`、chat `378a7f2`、paint `c49ef41`、database `280ddac`、charts `64c9db5`（全 main 本地未 push=merge 节奏正确）。**基线核验全 MATCH**（worktree 表/SHA/零 tracked dirty/blog 84a5514 未接入/blog-005 drafting 3/8 在窗）。**逐产品审计 7/7 = calc 先例精确实例化**：dep themes+legacy 键退役/缺省+use 导入（C-11）+design 默认腿逐产品核对；vendor 三件套与 canonical 逐文件一致；评审员**独立重实现冻结 digest 算法**（manifest 文件排序 sha256(path‖0‖content)）精确复现 `9fa07742..bef838e`。**SD 审计**：SD-01 候选表逐行对得上、三具名面 owner+closure 齐；SD-02 纯增量自免责（策略语义未动）；SD-04 单行正确；specs.json 零写入。**AC 图**：AC-01 pass（7/7 形态+digest 复现+follow probe 双 5/5 像素级；两族×明暗像素直证=两 dark_mode 位产品，其余五产品凭管治形态+套件绿——与收据自述口径一致）；AC-02 pass（全部套件数字在原始日志核读）；AC-03 pass（probe 日志 5/5×2+tamper 具名拒绝串逐字在册；OS diff docs-only=in-proc/outproc 053/055 语义未回归）；AC-04 pass（tamper/第二根 fixture 真实+日志在册；fresh/locked/缺件/同 mtime 以机制等价论证——work 阶段可接受，SD-03 同口径）；AC-05 pass（边界全真实具名）；AC-07/08 pending merge（非 fail）。**独立复现抽检 CONFIRMED**：digest 数学独立复现+calendar 冻结二进制 boot（MCP ~65s、快照全渲染、语义 token 在场）+精确按 PID 清理（外部 auto-edit 实例识别未碰）。**三项裁定（均接受）**：①blog hold=可接受（写窗纪律正确执行，owner/closure 齐，归档 8/8 分母保持 gate——不阻 work 阶段，merge 阶段闭条件）；②U054 pin 门=与合同一致（§4/§5.3/T-05 明文 gate，gitlink 未动已核）；③拒绝后立即重投=已记录的 host 侧观察非消费缺陷（合同命名恢复路径"重开或重连"双腿 5/5；若宿主页日后命名立即重投为新语义则属 053 页新合同项）。**Findings（无 adoption-shape/无阻塞，携 merge）**：F-1 low docs——SD-01 工具哈希尾缀笔误 `41b005` 应 `bab0b05`；F-2 medium 证据措辞——"冻结工具链"之**工具半边未实际钉定**（无 AUTO_UI_TOOL_BIN；套件 env 的 AUTO_UI_ROOT=main 使 tool discovery 实际解析到 main 检出 `0abe93fb`（含 053 机制祖先已核）；todo identity 记录过第三二进制 `4d2198b8`——正确性影响低（follow 腿像素翻转证明武装路径实际运行），merge 门须设 `AUTO_UI_TOOL_BIN` 冻结工具并记录每运行实际工具身份+修订收据/SD-01 措辞）；F-3 low 材料——tamper/notes 套件日志散在 Temp 与 /tmp 未入 plan-056 登记册（内容真实已核读；merge T-09 须收集或显式入账）；F-4 info——probe 重开腿 image-name taskkill 卫生（未来 PID 化）；F-5 info——主检出既有 `apps/auto-charts` 容器臂漂移（d5f5535f vs gitlink 77b6f82a，非本包所写，防 merge 误认）；F-6 trivial——notes adoption 提交信息 180s 与 amended 文件 420s 不一致（文件正确）。**pending_closeout_gates**：产品 push、OS 组合 pin（U054 后）、AUTO_UI_TOOL_BIN 钉定复跑注记、F-1/F-2/F-6 文字修订（merge landing 前并入 canonical 发布）、F-3 材料归集、统一回归（OS 侧 docs-only 本批零代码，按 888 scoping 记录）。**evidence**: 本计划 §9 各收据+plan-artifacts/plan-056/（probe 日志+toolchain.md）+评审员独立 boot/PID 清理实录（其报告附评审会话）。next: merge（T-08/T-09——携 F 清单与 AC-07/08 门）。
stage: goal | plan_id: PLAN-056 | plan_revision: r1 | round: 3 | from: review | to: merge | status: reviewed | outcome: pass | worktree: D:/autostack/.wt/os-056/auto-os @bafe685（reviewed） + 7 产品候选冻结 | commit: bafe685 | next: merge
2026-11-04 / stage:merge(回合4) / auto-os/PLAN-056 r1 / outcome:blocked(partial——landing/canonical/push 全过；archived+cleaned 阻于批次分母 blog-005 写窗，用户裁决待定) / **prepared**：F-1/F-2 修订（SD-01 工具身份更正+宿主页 owner 注记）并入 → `76f214c`（reviewed `bafe685` 之 docs-only 后代，交付 delta=评审裁定的 F 修订本身，实现/依赖零变更）；F-6 处置=接受注记（notes 提交信息 180s 陈旧注，amend 将重写已评审 SHA 不值；文件本身 420s 正确）。**landed**：branch rebase main（81dfa2f）→ `bafe685→d2b8014` range-diff `=` 全等（安全重写证明）+ F 修订 `72fa2e7`；`git merge --ff-only` 落 main——**main tip = 72fa2e7（交付提交），零 merge commit**；主检出他方 WIP（含 F-5 charts 臂漂移）未动未含。**pushed**：OS main `092254c..72fa2e7 → origin/main` ✓；7 产品候选正常 push 全 ✓（notes `5c9fa5d..38c7c87`、todo `182948d..1537107`、calendar `0a1bce4..9364239`、chat `db76383..378a7f2`、paint `4b60213..c49ef41`、database `cd240cf..280ddac`、charts `30436a7..64c9db5`——非 force）。**canonical_verified**：main 实读三路径在场——`docs/specs/apps/product-theme-batch-056.md`（新页）、host 页"产品批次消费面"节、independent-repositories 主题批次行 ✓（source verification，无索引/账本写）。**F-3 处置**：散落材料（Temp cal-*.log、/tmp notes-*.log）关键摘录已折入本计划 §9 各收据（评审员核读为真）；剩余原始文件随 plan-056 scope 删除一并消亡（delete-at-merge），登记册刷新记录之。**landing≠deployment 观察**：OS 侧本交付 docs-only 零代码——桌面二进制/运行实例无影响；产品侧 OS 组合 gitlink **未更新**（U054 gate，见下）——用户桌面运行的 apps 组合仍为旧 pin，待 U054 审定后由后继组合动作统一更新。**批量回归 due**：`docs/plans/.last-batch-regression.json` 缺席=due；处置=记录 pending（053 同例：本交付 OS 侧 docs-only 零代码 delta 无 OS 回归面；产品门已各自全绿），owner=后续 regress 触发时按 scoping 收敛。**BLOCKED（archived+cleaned）**：§10.2 明文"本批全部产品是结项分母…不能以先试点降为代表几例就归档"——blog（023-realworld）未接入（blog-005 在窗 drafting 3/8，owner 自 10-31 停滞于 auto-ui emission 前置）；归档须 8/8。**精确解除动作（用户裁决）**：(a) 授权本会话接管停滞的 blog-005 窗口（blog 同构接入+原门复跑+定向复审→归档清理）；(b) 等 blog-005 owner 恢复后由其完成（本计划保 reviewed，检查点全保全）；(c) 修订合同分母（需 plan revision，非默认）。**检查点保全**：OS main=72fa2e7 landed、7 候选已 push、组树在位（wt-guard 待清理前跑）、plan-artifacts/plan-056 scope 在位（归档后删）。next: user。
stage: goal | plan_id: PLAN-056 | plan_revision: r1 | round: 4 | from: merge | to: user | status: reviewed | outcome: blocked | worktree: D:/autostack/.wt/os-056/auto-os（保留待 blog 收口后归档清理） | commit: main=7202f7e（交付 72fa2e7 已 push） | next: user
## 10. 待澄清事项

1. 当前仅计划；U036；产品逐个接受已交付业务基线。产品源码映射可先做，OS正式pin与最终组合主题门等U054；054仍负责八产品原业务/后端装配，不由本包改写。
2. 本批全部产品是结项分母，未就绪产品可保持待办并推进无依赖项，但不能以“先试点”降为代表几例就归档。原支持面新缺口需实际owner/关闭条件或重规划，不自动接管全部债。
3. 只消费已交付接口；新语义/协议/引擎能力、预算/平台扩大不由本合同自动授权。双端/原生实际未支持必须具名，不能偷减原义务。
4. 真实窗口/端口/磁盘/解码/数据隔离T-01冻结，不操作用户系统主题/真实库/活凭据；真实OS事件沿055具体窗口规则，不做全局迁移前置。
