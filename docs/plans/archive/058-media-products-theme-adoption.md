---
plan_id: PLAN-058
status: archived
feature_name: 音乐视频图库三产品的正式主题接入
author: [Codex]
created_at: 2026-11-03
updated_at: 2026-11-03
plan_revision: 1
current_step: 6
total_steps: 9
supersedes_spec_components: [docs/specs/shell/host-theme-policy-and-delivery.md, docs/specs/apps/independent-repositories.md]
new_spec_components: [docs/specs/apps/product-theme-batch-058.md]
touched_goals: []
affects: [auto-music, auto-video, auto-photos, product-theme-adoption, os-theme-combination]
depends_on: [auto-os/PLAN-053, auto-ui/PLAN-031, auto-ui/PLAN-033]
---

# [auto-os/PLAN-058] 音乐视频图库三产品的正式主题接入

## 0. 变更摘要

以apps.manifest稳定ID为边界，完整接入本批3个实际产品：**020-music-player、030-video-player、029-photo-gallery**。采用036正式包/生成输入与053/055实际宿主端口，迁产品当前真实功能，不从auto-ui教学例复制业务或拿其通过抵产品验收。单一跨仓主合同在OS，参与产品执行时加README指针，不另造同一主题竞争合同。

drafting r1、0/9。2026-11-03用户授权全产品调研立项/路线刷新，**暂不实施**；本轮未建树/改产品或OS pin/运行构建、测试、GUI/系统设置/远端动作，不创建或联系其他agent/chat、不改canonical。其他会话WIP保留。以下全部新运行门未执行。

## 1. 目标

- G1：每个产品同一正式主题包的两个族×明暗真实生效，保默认/品牌/显式override/follow优先级及原支持轨，不只换外壳源代码。
- G2：活跃业务/输入/草稿/会话/内容/计时与持久化跨换肤保持；原完整适用门、时序/性能条件和负控不减。
- G3：实际宿主启动/独立运行/多App隔离、错误拒绝/恢复/owned退出真实闭环；候选源码到正式产品与组合pin同源。
- G4：本批全产品逐一交付，与036/055/054及教学迁移分工明确；不以代表App、研究/计划数冒全量迁移完成。

非目标：扩产品新业务/改数据库或Locator/协议/引擎，重做036发行算法、055真实OS事件/通用服务、054业务装配、教学037/038代码同步、全机/AG/ACE回归、所有历史债清零与远端registry。全批存在必需门未闭时不能用“历史问题”充PASS。

## 2. 架构方案

UI单源主题→036已交付发行/验证输入→产品已解析依赖与design/现适配端口→053/055实际host policy→产品/组件scope→公共控件与已支持的内嵌表面→真实可见颜色/模型和applied回读。品牌/分类overlay、文档/媒体/VT/series等内容域各自有owner与豁免依据；不复制整套palette，不给语言facade/generation-core增加UI反向依赖。

产品当前真实业务/后端profile/原测试是验收基面。只操作scope内小合法音视频/图像；AUTO_MEDIA_ROOT/AUTO_PHOTO_ROOT等实际配置由T-01核，不扫描用户E盘/图片库。包只改变公共壳/播控/队列表面，媒体像素、viewport黑/透明、专辑/图片内容色不被重染。Photos默认入口与references旧image-viewer分别对待，不把references全算在当前产品功能。 真正所需的新能力或既有非主题业务阻点先具名交owner/前置或重规划，不削减原承诺。只允许本批消费侧/已承诺主题投影的有限修复，公开协议/新引擎须另立约。

## 3. 技术栈

实际产品.at/pac/已有UI/engine/后端与原MCP/Playwright/产品验收；OS registry/launch/config/gitlink，UI正式包/scope/host transport，036输入/安装入口。准确命令/工具/版本/资源支持由T-01按真实README/Spec/help冻结，无新通用测试框架/registry。

## 4. 需求分析与背景调查

已使用auto-plan-new及verification-contract，读OS AGENTS、独立产品/主题policy Specs、UI主题/包/host transport、036/055/054和各产品实际pac/front/README/Spec/原测试/在途计划。逐入口完整来源与快照见[Design03全产品盘点](../design/03-product-theme-rollout.md)§2；该表记录28 active ID、HEAD/pin/源码摘要/dirty及原契约矛盾，不是运行证据。

manifest SHA256=57afca924fb95d99a29294163f134672d660c494202296b0f63812b749b08a44；OS观察HEAD=092254ce037ef7c7612cf11fe8b3fe00c57a76c6。下表Git前缀是观察值，未来开工重核完整SHA、review/merge/remote可取性及工具内容hash，不能直接拿当前main或陈旧OS pin当已验组合。

| 稳定ID / owner | clone HEAD / OS pin | 当前真实差额/在途 | 已核验证定位与隔离要求（参数工作时重核） |
|---|---|---|---|
| 020-music-player / auto-music | 6b73a417 / 6b73a417 | legacy dark/emerald、真实scan/播放 | tests/test_020_vm.py及原扫描/媒体门；受管小媒体根、真实back profile |
| 030-video-player / auto-video | b6efa4ef / b6efa4ef | legacy dark、media扫描/mpv原生/队列 | tests/smoke.spec.ts、vm-smoke.mjs及原native播放门；受管小媒体根 |
| 029-photo-gallery / auto-photos | 5eec01b8 / 5eec01b8 | 真实scan/thumb/full；旧image-viewer在references不等新入口 | 当前README的AUTO_PHOTO_ROOT与实际scan/thumb/full；无tests根不能冒已有GUI门，T-01补最小真实回归 |

U036和三个产品实际声明的扫描/播放/图像支持基线；不等Reader007、059、UI038研究或054整项。真实媒体链若阻断必需门，交实际owner修复/前置，不以空视图或404代PASS。 某产品门未满足不修改其在途内容/接pin，可推进同批独立准备项；execution_done/结项必须本批全部必需面闭合。旧README/计划状态陈述过时由实际owner按收据处置，本包不代它终审或修改原合同。

## 5. 详细设计

### 5.1 启动、所有权与下一任务

实施授权待给。拟唯一OS组D:/autostack/.wt/os-058/auto-os、分支codex/plan-058-product-themes；UI/框架确有改动才各加同组唯一树。普通产品在D:/autostack/<repo>独立clone main一写入者，工作/独立review顺序共用，禁止编辑OS apps/子模块或复制第二review树。Musk按其AGENTS专属worktree，auto-down嵌套Garden按owner专属树；不能把普通产品main例外套基础仓。local syslog（若本批含）在OS树。各仓AGENTS工作时重读，不擅自stash/reset/切走他方WIP，无链接。

T-01冻结源/已交付业务与支持、实际package/CLI/tool/engine/后端、旧设置/品牌/颜色角色、原用例/样本/排除分母及端口/数据/窗口预算；用真实整链合法/拒绝控制证实接线可行性再做依赖实现。无正式来源的WIP不消费；若真正缺能力/需要改公开支持，给精确owner/关闭条件与合同修订，不静默降级成静态Demo。

### 5.2 本批实现与区分性不变量

第一组Music/Video真实scan、播放/暂停/seek/队列；第二组Photos实际scan/thumb/full/收藏导航。原native解码/性能/资源约束保留，无需引入新媒体格式。

主题往返时播放进度/音量/队列/正在选媒体和图像session不变，扫描/解码失败真实可见且恢复可再播放；不能仅验证空库布局、假进度条或截图。

逐产品列公共role/交互状态、品牌与分类色、内容域和合法固定值表；保现默认与其对应业务语义。每个产品真实Vue/VM及已承诺native/host路径分别核，不能从生成成功推定原生特殊组件可用。换肤前后固定业务哨兵与实际颜色/引擎选择读回，两族×明暗逐产品完成；需要主题化的可用嵌入表面通过原端口消费，不能只改外层截图。内容/媒体像素/文档身份不因“统一”被染色。

宿主跟随/覆盖/非法包/乱序拒绝保旧、重开或重连追最新，两个实际产品相反族/模式不串。包/I/O失败或未绘制不能回报applied；恢复后下一次真实操作成功，关闭只收割本次owned连接/进程/订阅。系统真实事件归055，本批不修改用户全局设置。

### 5.3 外部来源、候选与正式落地

036完整交付是采用新渠道的产品实施/最终消费门；T-01可获授权后先盘点，绝不借其execution_done实现树。通过正式安装/包输入做fresh/locked/第二绝对根、缺件/错hash/合法同mtime内容变更及恢复，绑定实际产品/产物；不手拷教学palette、不用原target充外部构建，910项目/member/context/lease与可写target隔离保持。

每产品先形成已提交候选，原门/主题/实际桌面支持组合验收；独立review后正常push，OS干净组合fetch/detach到同SHA、健康门+真实启动后才更新本批pin。外部无gitlink条目按真实repo/嵌套根HEAD、daemon/tool/资产身份固定并验启动，不伪造新submodule。仅修改本批增量，最新OS其他pin/注册/功能零意外覆盖；056正式pin等054先交付，后继必须是其审定业务版本的主题后代，不互相回退。

本批交接按Design03 §3/6：036拥有发行/输入、055拥有clock/G-1/G-2（仅所需outproc能力正式交付作为该腿门）、054拥有八基础业务装配；UI037/038教学线不抵产品。每个产品剩余问题/未支持平台具名，不把原支持义务删掉；真实依赖/source变更按918规则正常审计，不永久豁免。

### 规范增量

| delta_id | add/modify/retire | docs/specs/... target | before/after rule | rationale | acceptance IDs |
|---|---|---|---|---|---|
| SD-01 | add | docs/specs/apps/product-theme-batch-058.md（新） | 本批产品实际source/pin/profile/主题支持与剩余边界 | 组合不以Demo代验 | AC-01..05 |
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
| T-01 | ✅ 冻结+前置全核实（U036/053/031/033 archived）；os-058 组树冻结（man 2bbd954/lang 2f558fc93/ui 3c46104b/web 3de5524/down 7c4d5cbb，PATH 陈旧二进制退役重建）；manifest sha=57afca92…未漂移；三产品 clone clean=§4 观察值；主题包源 per-file sha 校验；video 双链例外发现（冻结线解析红 A/B） | T01-freeze.md；AC-01..05 | work |
| T-02 | ✅ music `4b9e446`（vendor 0.2.0 快照逐字节校验+pac dark 退役+use/design；反控两腿像素精确 #09090b/#ffffff+真实 scan/播放哨兵；原门 exit 0） | T-01+U036及对应产品门；AC-01/02/03 | work |
| T-03 | ✅ video `0922b44`（双腿 #0f0f12/#ffffff 精确；vm-smoke ALL PASS；**具名红①** media 能力臂后端生成 A/B 双链同败 owner=auto-lang）；photos `7ce8bd2`（本仓首个永久回归门：真实 scan+full 链 /api/photos/full/:id+双腿精确） | T-02；AC-01..03 | work |
| T-04 | ✅ V-03 五腿全过：music@Solarized/dark #002b36 精确、双产品相反策略零串扰、坏 digest 具名拒绝保旧（实录）、改写→重开追最新 #09090b 精确；OS056-REAPPLY-NOTE 复现（live 臂滞留，053 owner 维持，非本批回归） | T-03；AC-01..03/05，V-01..03/05 | work |
| T-05 | ✅ V-04 全过（music：篡改→具名拒绝 `主题包生成准入失败` 非零中止、恢复 sha 等值干净 boot、第二绝对根零拒绝）；组合 pin=组树 `3e84b43`（3 gitlink detached 候选+健康门批内臂 clean+锁同型加法刷新 036 先例）；实际桌面 boot（组树 ui-desktop iced：theme 包 digest 行+registry=7 desktop_visible=7+alive 50s+ 四心跳，隔离 storage/config） | T-04、§4精确前置；AC-04/05，V-04/06 | work |
| T-06 | ✅ SD-01/02/04 草备随 `3e84b43`（SD-03 三页在各产品仓候选内）；自查 AC-01..05：主题面三产品全闭、业务哨兵全绿（video 真实媒体腿=具名红① held，原 vm-smoke 合同过）；命名 hold 3 项（①media 能力臂 owner=auto-lang、②冻结线子模块导入解析 owner=auto-lang、③REAPPLY-NOTE owner=053）均 A/B 同源非本批回归；统一回归门（冻结受影响并集）留 review/merge；execution_done 6/9 | T-05；AC-01..05 | work |
| T-07 | 同组/同clone冻结、独立上下文final review，发现返write owner并失效相应证据，不造第二实现树 | T-06；AC-06，V-07 | review |
| T-08 | 冻结必要职责并集、依赖序落地/产品正常push/OS干净组合pin或外部源读回、健康/实际启动/canonical/路线/归档；不覆盖054/其他新提交 | T-07 pass+§4组合门；AC-07，V-08 | merge |
| T-09 | 全输出/设置/owned进程对账，摘要折Plan删统一scope/自有树产物，各树guard clean后移除/分支/空组；保产品clone，archived9/9 | T-08；AC-08，V-08 | merge |

材料唯一D:/autostack/plan-artifacts/plan-058/auto-os/，按repo/owner/用途/在途登记；数据/库/文件/媒体/设置备份/日志截图/外部SDK/生成副本/显式target皆该scope，普通编译在唯一树，产品合法输出全列账。禁止默认temp/仓内attachments/其他盘散落证据。merge逐项核全部实际输出路径及活动归属，命令/退出码/关键值/源hash/未解项折Plan后删本scope/自有树产物；仅具名未闭持有有owner/理由/关闭条件，不能借持有将必需面判PASS。未知/活动/他方不盲删，产品clone不删/reset，legacy specs.json不写。

## 9. 复审记录

2026-11-04 / stage:work / auto-os/PLAN-058 r1 / outcome:executing（T-01..T-03 完成，T-04..T-06 进行中） / goal授权=用户 `/goal 完成计划058` 全链（解除首收据授权 blocked）。前置核实：U036 archived（UI e06b65b2 交付）、053/031/033 archived——T-02 前置满足。T-01 冻结（plan-artifacts/plan-058/auto-os/T01-freeze.md）：OS 31bdcfd、manifest sha=57afca92…（与 §4 观察值一致）、三产品 clone clean main 6b73a41/b6efa4e/5eec01b（=§4）、主题包源 per-file sha 校验（pac 2e66212b/themes 329c2d56）。工具链裁定：**os-058 组树**（man 2bbd954@.wt/os-058/auto-man + lang 2f558fc93@auto-lang + ui 3c46104b@auto-ui + web 3de5524 + down 7c4d5cbb，PATH 陈旧 47d8f7b 二进制退役重建）；video 例外跑主检出链（冻结线解析不了其子模块 dep-style 导入——具名红②）。

T-02/T-03 三产品候选提交（057 模板：vendor/themes 0.2.0 快照逐字节校验 + pac theme 键退役=follow 宿主 + use themes.themes: Auto + design{Auto, dark_mode 派生} + SD-03 页 + README 反链）：**music 4b9e446**（反控两腿像素精确：AUTO_UI_THEME=dark→主区 #09090b、未设→OS 跟随 #ffffff；真实 scan/播放哨兵受管夹具 3 首；原门 test_020_vm.py exit 0）、**video 0922b44**（队列面板 #0f0f12/#ffffff 双腿精确；vm-smoke.mjs ALL PASS 诚实降级；**具名预存红①** media 能力臂后端生成两链同败 "backend Cargo.toml missing"——真实 scan/播放腿与 smoke.spec.ts 阻，owner=auto-lang 后端生成）、**photos 7ce8bd2**（本仓首个永久回归门：真实 scan 哨兵+full 链 OpenPhoto→/api/photos/full/:id→回网格+双腿像素精确；photo_service 能力臂 17833 实跑正常）。

关键机制实录（plan §2 落地语义）：dark_mode 播种链=Plan 458 env（AUTO_UI_THEME：CLI>os-config>pac.at）+ PLAN-615 T-04 OS 系统主题回退；design.mode 每帧求值、宿主策略完整快照 full_host 接管（031 语义）。夹具全部 plan-artifacts scope 自制（WAV 3/MP4 2 含嵌套/PNG 5），零触用户 E 盘/图片库。证据：plan-artifacts/plan-058/auto-os/{music,video,photos}/（双腿截图+state+mcp-tools.json）+ 各产品 tests/test_058_theme.py。产品 push 与 OS pin/落地留 merge（T-08）；T-04 宿主跟随腿与 T-05 组合健康进行中。

2026-11-04 / stage:work / auto-os/PLAN-058 r1 / outcome:pass（execution_done 6/9） / goal授权=用户 `/goal 完成计划058` 全链 / worktree=D:/autostack/.wt/os-058/auto-os 分支 codex/plan-058-product-themes；base 31bdcfd；code_commit 3e84b43（3 gitlink 候选 pin 4b9e446/0922b44/7ce8bd2+SD-01/02/04 草备+Cargo.lock 同型加法刷新）；dependency_revisions：man 2bbd954/lang 2f558fc93/ui 3c46104b/web 3de5524/down 7c4d5cbb（组树 detached clean）；工具链身份 组树 auto.exe g2bbd954(491d568b…)+主检出链 auto.exe（video 线）；task_ids T-01..T-06 / AC-01 主题面：三产品正式包双腿（显式 dark/OS 跟随）像素逐位精确（#09090b、#0f0f12、#ffffff、#002b36 Solarized 宿主腿）；AC-02 业务守恒：music 受管夹具真实 scan(3首)/播放/队列哨兵跨策略环境保持、photos 真实 scan+full 链+回网格、video vm-smoke 诚实降级 ALL PASS（真实媒体腿=具名红① held：media 能力臂后端生成 A/B 双链同败 owner=auto-lang，关闭条件=生成完整工作区+smoke.spec.ts 绿）；AC-03 V-03 五腿（相反策略隔离/拒绝保旧实录/重开追最新/零串扰）；AC-04 V-04 三腿（篡改具名拒绝非零/恢复 sha 等值/第二根零拒绝）；AC-05 冻结与候选：候选 SHA=被测源、基线 A/B 双臂、055/054/Demo 边界零触碰、无 WIP 消费 / evidence：plan-artifacts/plan-058/auto-os/（T01-freeze.md、music/video/photos 双腿截图+state、hosted/ V-03 逐腿截图+state、v04/ 三腿日志、hosted-desktop/ boot storage） / blockers：无新增（3 命名 hold 均 A/B 同证预存、owner 域不在本批） / pending gates：统一回归（冻结受影响并集）owner=review/merge、产品 push+OS 落地 owner=merge / next_responsible_role=review（T-07 独立上下文 final review）。

静态预检pass：0–10章节、9T/8AC/8V/4SD唯一连续，所有AC/验证/任务归属有据；本批与其他批无重复，合28 active ID全覆盖；本号活动/归档唯一、OS号源062，本地引用/限定diff/尾空格检查通过。仅文档检查，所有项目运行/新负控/最终独立复审仍未执行。

stage: goal | plan_id: PLAN-058 | plan_revision: r1 | round: 1 | from: work | to: review | status: execution_done | outcome: pass | worktree: D:/autostack/.wt/os-058/auto-os | commit: 3e84b43（组树）；产品候选 4b9e446/0922b44/7ce8bd2 | next: review（T-07 独立上下文 final review；产品 push/OS 落地/归档留 merge）

2026-11-04 / stage:review / review_scope:final / reviewer_identity:独立上下文 subagent（general-purpose agent，无实现会话记忆；limitations=music反控与原门、video反控与vm-smoke未重跑改核测试源码断言+state JSON+日志一致性、V-04三腿未重跑核t1/t2/t3日志+探针脚本、hosted-desktop boot未重跑核storage/config与提交声明、冻结链video解析红未直接复现核记录）/ plan_id:PLAN-058 / plan_revision:r1 / outcome:pass / worktree_path:D:/autostack/.wt/os-058/auto-os / branch:codex/plan-058-product-themes / reviewed_commit:3e84b430ceea…（全clean复核；产品 4b9e446/0922b44/7ce8bd2 与 gitlink/SD-01 三方一致）/ base_commit:31bdcfd / dependency_revisions:man 2bbd954/lang 2f558fc93/ui 3c46104b/web 3de5524/down 7c4d5cbb（组树 detached clean）/ spec_inputs:SD-01/02/04（worktree 3e84b43 冻结副本）+SD-03×3（产品候选内）/ acceptance_results:AC-01..05 pass（AC-02 video 真实媒体腿=具名红① held，vm-smoke 诚实合同过）；评审自主复跑：photos 反控全程两次 exit 0（证据逐字节同哈希）、video 主检出链 service 探针复现红①一字不差、hosted 11 张 PNG 像素全实测（#002b36/#ffffff/#09090b/#051a21 REAPPLY-NOTE 滞留帧）；pending_closeout_gates:统一回归（冻结受影响并集）owner=merge、产品 push+OS 落地 owner=merge / findings:R-01 minor（music原门/video vm-smoke exit 日志未存档——merge 前补跑补存或折行注明证据形态）、R-02 minor（config-sol-dark.at 被腿E就地改写，T-09 清理注明）、R-03 minor（auto-music 未跟踪截图残留，push 前清理）、R-04 minor（SD-02 "60s 不落定"数值宜引 flip probe 60s 轮询实录收敛措辞）、R-05 observation（v03-d 拒绝语义=fresh boot 拒绝形态，与措辞自洽，备查）/ evidence:本记录+plan-artifacts/plan-058/auto-os/（评审复跑落盘同目录）/ next:merge（T-08/T-09；R-01/R-03/R-04 在 merge 顺手处置）。

stage: goal | plan_id: PLAN-058 | plan_revision: r1 | round: 2 | from: review | to: merge | status: reviewed | outcome: pass | worktree: D:/autostack/.wt/os-058/auto-os | commit: 3e84b43（reviewed_commit，独立上下文 subagent 复核零漂移+AC-01..05 pass+自主探针 photos exit 0 字节级可复现/红①复现/hosted 11 PNG 实测） | next: merge（T-08 产品 push/OS 落地/归档 + T-09 对账清理；R-01/R-02/R-03/R-04 merge 顺手处置）

2026-11-04 / stage:merge / plan_id:PLAN-058:r1 / reviewer/operator:主会话（auto-plan-merge skill全流程） / outcome:pass（delivered） / checkpoints: prepared=组树 c2e9c5a（reviewed 3e84b43 的 doc-only 后代：R-04 措辞单行收敛，implementation/deps 零变更，exact delta 核对）；产品处置=R-03 清 music 残留（src/front/tests/ 删除）、R-01 补存 music test_020_vm exit 0 与 video vm-smoke ALL PASS exit 0 日志（plan-artifacts 后随 scope 折叠）；landed=main ff-only `31bdcfd..c2e9c5a`（零 merge commit；rebase up-to-date，range-diff 3e84b43 `=`+新增 c2e9c5a），OS push `31bdcfd..c2e9c5a → origin/main`；产品 push 全 ff：music `6b73a41..4b9e446`、video `b6efa4e..0922b44`、photos `5eec01b..7ce8bd2`；main 检出三子模块同步 detached 到落地 pin（0 dirty）；canonical_verified=main@c2e9c5a：SD-01 docs/specs/apps/product-theme-batch-058.md 在树、SD-02 PLAN-058 节（R-04 措辞落定）、SD-04 批次行、三 gitlink=4b9e446c…/0922b44d…/7ce8bd25… 逐一核实 / landing≠deployment：OS 桌面运行时产物（ui-desktop 等）按需重建——T-05 实际启动已在组树 c2e9c5a 同源工具链实证（theme 包 digest 行+registry=7+alive 50s+），主检出 ui-desktop 无既有生产二进制=记录观察；batch_regression：.last-batch-regression.json 缺失=due→交接 auto-plan-regress 主检出单实例执行 / next: closeout（wt-guard→组树/分支删除→plan-artifacts 折叠删除→registry 刷新→cleaned commit）

2026-11-04 / stage:merge / closeout / cleaned（部分 pending 项见尾） / wt-guard：六树全清（auto-os/lang/ui/web/down/man 逐树 "无任何 reparse point"）；worktree 移除：lang/ui/web/down/man 五树+git 元数据 prune 完毕、分支 codex/plan-058-product-themes 已删（@c2e9c5a）；auto-os 树内容全部删除（python 逐文件 0 locked），唯空目录句柄被外部进程占用（他方 cargo 构建会话 CWD），rmdir pending——guard 已清、git 已 prune、内容已空，仅剩目录壳，句柄释放后 rmdir 即净；plan-artifacts/plan-058 全 scope 折叠后删除（R-01 两腿日志=exit 0 摘要已折入本页 merge 收据；R-02 config 就地改写注明于 hosted 收据；所有像素数值/命令/退出码已折入 §9 各收据）；registry 刷新：新增 hold ①PLAN-058-MEDIA-BACK-GEN（video media 能力臂后端生成两链同败，owner=auto-lang 后端生成，关闭条件=生成完整工作区+video smoke.spec.ts 真实 scan/播放门绿）、②PLAN-058-FROZEN-LINE-CHILD-IMPORT（冻结线子模块 dep-style 导入解析 fatal，owner=auto-lang 解析线，关闭条件=冻结线兼容或批次例外线文档化维持）；既有 hold ③OS056-REAPPLY-NOTE（owner=053）维持并在本批复现实录；完成态 completion_kind: delivered；batch_regression due（.last-batch-regression.json 缺失）→ 交接 auto-plan-regress 主检出单实例执行，非本计划门。

## 10. 待澄清事项

1. 当前仅计划；U036和三个产品实际声明的扫描/播放/图像支持基线；不等Reader007、059、UI038研究或054整项。真实媒体链若阻断必需门，交实际owner修复/前置，不以空视图或404代PASS。
2. 本批全部产品是结项分母，未就绪产品可保持待办并推进无依赖项，但不能以“先试点”降为代表几例就归档。原支持面新缺口需实际owner/关闭条件或重规划，不自动接管全部债。
3. 只消费已交付接口；新语义/协议/引擎能力、预算/平台扩大不由本合同自动授权。双端/原生实际未支持必须具名，不能偷减原义务。
4. 真实窗口/端口/磁盘/解码/数据隔离T-01冻结，不操作用户系统主题/真实库/活凭据；真实OS事件沿055具体窗口规则，不做全局迁移前置。
