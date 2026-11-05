---
plan_id: PLAN-054
status: executing
feature_name: 八个基础应用统一组合与桌面任务验收
author: [Codex]
created_at: 2026-11-03
updated_at: 2026-11-05
plan_revision: 1
current_step: 4
total_steps: 10
base_branch: main
base_commit: 65fc70de8b505657db660d91024b2cf4425b7759
depends_on: [PLAN-050, PLAN-051]
supersedes_spec_components: [docs/specs/apps/independent-repositories.md, docs/specs/shell/desktop-app-launch.md]
new_spec_components: [docs/specs/apps/basic-app-assembly.md]
touched_goals: []
affects: [apps-gitlinks, apps-manifest, desktop-app-launch, backend-profile-consumption, app-data-isolation]
---

# PLAN-054：八个基础应用统一组合与桌面任务验收

## 0. 变更摘要

承接各app独立基本能力与运行收口，统一接入Notes、Todo、Calendar、Paint、Database、Chat、Blog、Charts。先核验真实交付，再在专用候选OS组合更新8个gitlink及必要注册/后端声明、完成真实桌面用户任务和多app共存；不以批量更新SHA代替运行验收。

Reader按独立会话Readest路线**排除**，其pin与代码不在本包；非目标还包括Linux适配、功能扩展、主题/计算器053、通用OS通讯049、服务supervisor重写、框架大迁移与全机缓存清理。本次只立项，不更新任何子模块。

## 1. 目标

- G1：8个app各自实现、最终复审、归档与远端提交可追溯，选择当前功能交付基线，不混入未交付WIP；同一SDK/CLI/tool/桌面宿主组合可重建。
- G2：候选组合全部既有注册入口保持完整；8个目标按稳定ID唯一注册，实际源码等于固定pin，app前后端按已验profile工作。
- G3：Vue桌面与iced桌面各自从启动器完成8个app基本用户任务、关闭/重开/继续修改；多app同时运行时数据/身份/路由互不混淆，关闭只回收自己资源。
- G4：失败可诊断且不破坏旧用户数据、旁观进程与原组合；交付后主检出8个子模块clean/detached到审定SHA，保留可恢复的旧版本记录。

## 2. 架构方案

主责auto-os；Plan在main登记，未来唯一实施组目录 `D:/autostack/.wt/os-054/auto-os`，分支 `codex/plan-054-dev`，现在不创建。实施/内部检查/独立最终复审顺序共用该树。app独立clone保持main且只读，不切走他人WIP，不在子模块内开发产品功能；确需app源码/pac变更由对应app owner沿独立Plan交付后消费。

依赖不是“另几个仓main最新即完成”。T-00只读盘点/技术探针可先行；**T-01交付门未全部关闭前，不得执行T-02的pin升级或候选桌面验收**。缺项沿现有Todo-002/Database-002/Blog-005补齐，不改旧AC/伪造归档，不用旧001/004提交绕过本轮已约定缺口。

候选工作树默认空gitlink不能证明组合完整：需要把所有已登记子模块初始化到各自记录的pin（非目标不升级），并准备manifest真实外部repo依赖。候选运行显式 `DESKTOP_OS_ROOT=<本候选OS>`，由051脚本同时设正确 `AUTO_OS_ROOT/AUTO_DESKTOP_APPS`；不能注册表读主检出旧app、截图却声称候选已验。不建junction/symlink。

继续复用现有registry/launch、按app后端供给与051启动脚本。公共框架若缺按app后端profile/HTTP路由能力，T-00定位现有端口后交实际auto-ui/auto-web/auto-man owner最小前置，独立交付后再消费；本计划不无授权改共享框架或建另一个服务管理器。

## 3. 技术栈

现有.at app、git submodule、apps.manifest、PowerShell/Bash桌面入口、AutoUI注册与会话/后端供给、Rust服务或VM HTTP、Playwright/MCP与各app现有数据oracle。UI源码位置已迁移：registry在auto-ui-compiler，session/back_provision在auto-ui-runtime；不沿旧Spec指向auto-lang路径直接修补。

## 4. 需求分析与背景调查

用户2026-11-03要求检查最近计划并规划下一轮；沿既有安排，先补app单独可用，再用统一OS计划集成。本轮授权为核对、立计划与文档提交push；不实现、不更新OS子模块、不代替其他会话做最终review或推送尚待审实现，不扩大Reader/功能增强/Linux范围。

详细事实见[2026-11-03交付核对](reports/20261103-basic-app-delivery-audit.md)。当前Notes/Calendar/Paint/Chat/Charts有交付回执；Todo-002仍drafting/0/7，Database-002实施6/8但review/merge未完且8个本地提交未推，Blog-005仍dev门阻塞。Chat归档计数7/9与未勾项、Database frontmatter/README滞后均保留为对账问题，不能据文件路径或用户概述推定已验。

只读子模块健康检查仍发现Charts实际checkout与父pin不符（27 repo，missing_pac_at=0）；本轮不纠正。manifest部分app只列旧前端端口；现桌面back_provision对一般#[api]默认走VM proxy，而Chat凭据生成只在已验Rust轨安全可用。独立通过不等于可不加区分地嵌入桌面。

本轮源观察的UI HEAD为 `cfe325d41e9c98b9d877fe071db04b42b687880c`；未来work重新冻结实际公共组合，不把此观察值当兼容性认证。

| 已观察源 | SHA256 |
|---|---|
| `D:\autostack\auto-os\apps.manifest` | `57afca924fb95d99a29294163f134672d660c494202296b0f63812b749b08a44` |
| `D:\autostack\auto-os\.gitmodules` | `789e6307f470a07e0da14762277de28ead8b332f0aae3531efb29366db9d9119` |
| `D:\autostack\auto-os\docs\specs\apps\independent-repositories.md` | `1fd1f626b08ce2d278f17d72183499d295c9d8a62eef56ee731ab8aece5e7450` |
| `D:\autostack\auto-os\docs\specs\shell\desktop-app-launch.md` | `32316db755d035b798f32b85e02525c2d39e6b83ace55a6156078345ca24fe6d` |
| `D:\autostack\auto-os\docs\specs\shell\desktop-bootstrap.md` | `63ba8243c96177794c6c6488b1532b63deba6f35ae691e5a2a3b64722b2d8882` |
| `D:\autostack\auto-os\scripts\desktop.ps1` | `ffc5fd34f4caec1025dbb7f957437d571e377a60583c7a53b0a6060a35dc2670` |
| `D:\autostack\auto-os\scripts\desktop.sh` | `f095c1cff9a1908c376576ef030630135bf41e2fc43021e650c06fc68892f1eb` |
| `D:\autostack\auto-os\scripts\check-app-submodules.ps1` | `af0bad9960e487325a8e3377cd8f7add3256547c2337770e69b6fe997401ce2c` |
| `D:\autostack\auto-ui\crates\auto-ui-compiler\src\app_registry.rs` | `7f1029651ad0432171a7b436ab83b26ab97ec97bd71d4a34f298ef09ea9bdb80` |
| `D:\autostack\auto-ui\crates\auto-ui-runtime\src\ui\session.rs` | `d87c86613cc9cafcc8c488853fbf32a976b8c1de6562c85ce106e64d48f30dca` |
| `D:\autostack\auto-ui\crates\auto-ui-runtime\src\ui\back_provision.rs` | `417e5a4605612c71a8f85960852eb68bcd05b8634a70fbeab0986b9c4fbed325` |

## 5. 详细设计

### 交付门与支持矩阵

T-00产出拟新增 `docs/plans/evidence/054/assembly-decision.md`，T-01产出拟新增 `docs/plans/evidence/054/delivery-lock.md`。后者逐app列完整SHA/远端、review/merge/归档和源相等证明、Spec/hash、选择理由、前后端profile、数据根、实际端口/路由、CLI/tool/宿主hash及本轮支持范围；不是新增产品注册表或手工Spec索引。

| app / 稳定manifest id | 必需前序与当前状态 | 后端下限/数据根 | 组合用户任务 |
|---|---|---|---|
| Notes / 015-notes | 001+004已交付，owner对齐失效归档链接 | Rust；NOTES_DATA_DIR，测试门缺省关 | 新建中文笔记→编辑/保存→重开/下一次保存 |
| Todo / 013-todo | 001+002；002未实施，继续原计划 | Rust；AUTO_TODO_DATA | 创建/编辑/完成→重开→继续编辑 |
| Calendar / 016-calendar | 001+002已交付 | Rust；AUTO_CALENDAR_DATA，非测试墙钟 | 新建跨日事件→编辑/删除→重开，恢复凭据/删除事实保持 |
| Paint / 031-paint | 001+002已交付 | 现有VM/VM HTTP；独立KV与Vue同源profile | 画图→Save/Load→SVG→重开逐格一致；不要求跨profile同步 |
| Database / 026-database | 001+002；002待最终review/merge/push | Rust；本次关闭状态SQLite夹具 | 真输入/树选择/只读SQL/拒写→切库/关闭，原件不改 |
| Chat / 017-chat | 001+002有review/交付，owner对账7/9 | Rust secure_hex；CHAT_DATA_DIR，测试/legacy接管缺省关 | 桌面A＋独立合法客户端B互发/重开；C不可读私聊，凭据不入证据 |
| Blog / 023-realworld | 001..004+005；005上游/dev门待关闭 | Rust；AUTO_BLOG_DATA | 真作者页挂载→编辑/自动保存/预览→重开再存，非demo feed |
| Charts / 024-charts | 001+002已交付 | VM HTTP；AUTO_CHARTS_DATA | 中文路径CSV/小数柱图→工程保存重开→独立SVG |

**门的关闭**：每项前序必须有实际已交付/固定远端SHA和对应review/merge证据，内容与Spec一致；元数据滞后由owner根据真实回执修正，不补虚假勾选。共用工具组合通过所选源码的受影响独立门后才能签署delivery-lock。不能用不含005功能的新旧Blog混搭、Database旧Init注入、Chat VM弱随机或Todo未隔离旧门绕过。

### 候选与启动链

1. 记录原8个pin和全部非目标pin/注册集。候选中8个submodule只fetch并detached到已推送SHA，其他submodule只初始化不升级；URL仍实际远端，branch=main不代替固定pin。任何脏/有人写入的子模块拒绝切换，回到app owner处理。
2. 启动链为：稳定ID→真实候选pac/源→声明的后端profile和数据作用域→正确路由→真实UI→独立磁盘/HTTP oracle。不能只验证图标、首页或健康200。保留正常VM前端承载，**前端VM不意味着后端一律VM**；Rust服务不得静默解释降级。
3. 补齐必要ports/launch/daemon或现有受支持profile接线；只有使用现有daemon机制时才按其固定GET /api/health契约接入，缺health的app不得配置成必然可用。具体接入/复用选择由T-00全路径探针冻结，不预设8个都需新daemon。
4. 数据与进程归属按app和本次run绑定，不借启动全机环境修改串到下一app；固定端口冲突要命名失败，不杀占用者/静默换路由。共享后端最后窗口关闭的释放/继续服务选择须在T-00逐profile说明，测试只能回收本次拥有的进程/连接。
5. 分别验Vue桌面与iced桌面。若某app独立profile与桌面承载不兼容，提出实际最小接缝并保持该必需门未完成，不退为从桌面另开外部窗口来冒称嵌入成功；确需改变承载目标须明确用户决定。

### 同场、失败与回退

至少同时打开Notes/Todo/Calendar三个有持久数据的不同app，再加入Chat/Charts或Paint覆盖Rust＋VM混合；为每个app写不同内容哨兵，逐窗关闭/重开/修改仍各归各库。Chat二客户端只用本次测试身份；原始日志不包含活凭据。Database使用关闭的隔离夹具，不对用户库试写。

失败场景包括候选tool缺失/身份不符、端口被旁观服务占用、后端未就绪/中途退出、写入拒绝、子模块缺pac/指针错与路由误绑。失败可见且不出现演示成功，已提交数据保留，解除后下一次实际操作成功。回退只还原候选8个源码pin和声明，不倒写迁移后的用户数据；数据变更需app已验证备份/兼容合同或本次隔离目录，不能许诺git回退能逆转数据迁移。

053主题/计算器与本包共享manifest/宿主入口时按实际文件窗口协调，T-00冻结接口/基线后串行合入；不整体等待053或改其WIP。未来主main前进时重放只含本包的8个pin/声明增量，对新main的非目标pin做差集；不覆盖其他owner合法升级。

### 规范增量

| delta_id | add/modify/retire | docs/specs/... target | before/after rule | rationale | acceptance IDs |
|---|---|---|---|---|---|
| SD-01 | modify | docs/specs/apps/independent-repositories.md | 补8app交付/组合版本与app/OS责任边界，不复制app业务Spec | pin需与独立交付/真实组合绑定 | AC-01/02/06 |
| SD-02 | modify | docs/specs/shell/desktop-app-launch.md | 当前owner路径、稳定ID与前端/后端profile分离，候选根与失败规则 | 防默认VM后端/旧根假验 | AC-02/03/04 |
| SD-03 | add | docs/specs/apps/basic-app-assembly.md | 实际支持矩阵、共用运行身份、同场/隔离/恢复和未支持边界 | 只发表真实组合结果 | AC-01..06 |

## 6. 测试设计

复用 `scripts/check-app-submodules.ps1`（含-SelfTest/-OsRoot）、`scripts/verify-app-links.py`、051的 `scripts/tests/test_desktop_launcher.py` 和各app已交付门；T-06使拟新增 `scripts/tests/verify_basic_app_assembly.py --profile vue|iced --os-root <候选根>` 可执行，旧工具参数先按源码核对，不能假造现有选项。只跑实际改面和必要公共owner的固定组/消费验证；不因OS/app工作在语言仓跑Cargo全量/docs_gen。

| Case ID | AC/不变式 | 真实入口/输入 | 区分性oracle与关闭条件 | owner_stage / 边界 |
|---|---|---|---|---|
| V-01 | AC-01/02 | T-01的8app交付与共用运行lock；坏SHA/未push/待review/错tool反例 | 每个选择可追溯，错误具名阻止T-02；合法项可继续。文件存在或frontmatter单项不能作唯一oracle | work；不补造旧review |
| V-02 | AC-02 | 候选全部submodule/manifest与显式DESKTOP_OS_ROOT，删pac/错pin/旧根反控 | 健康门检测错误；正常候选完整注册集、8目标唯一ID且源SHA正确；非目标不升级不消失 | work；Reader pin不动 |
| V-03 | AC-03 | 两种桌面启动器实际点击8个app，执行§5用户任务后重开/再操作 | 窗口/页面中真实内容＋独立库/文件/几何oracle一致；Chat使用安全Rust凭据，Blog真作者页而非壳 | work；无API预写替代UI |
| V-04 | AC-04 | Notes/Todo/Calendar同场＋Rust/VM混合，关闭一个/重开/恢复 | 不同哨兵分别在正确库，路由/端口/凭据不串，旁观进程和非选定Clock/扫雷基线入口保持 | work；不接管Reader/主题功能 |
| V-05 | AC-04/05 | 后端终止/拒写、端口占用、坏tool/缺pac，恢复后继续实际操作 | 失败非零/可见且已确认数据不损坏，不清旁观/共享target；解除后下一次成功，自有子树/资源释放符合合同 | work；仅隔离故障 |
| V-06 | AC-05 | 将实际候选路由/源身份指错或抹掉真实UI结果的受控变异 | 对应断言红、总门非零；还原合法对照通过，无必需skip/pending/零场景 | work；证明变异确实作用于被测候选 |
| V-07 | AC-06 | 新main上只重放本包固定组合，受控候选回退/重新启动 | gitlink/声明集合精确、non-target不变、健康绿；回退不倒写用户数据，恢复后任务仍可执行 | work；正式merge复用绑定基线证据或按漂移重验 |

work内部自检只表明可交最终review；未指定其他模型/agent，本计划不自动创建或联系新会话。最终review以独立上下文、同一实施树核验交付身份及两桌面的真实任务，声明证据复用和局限。原始日志/报告/截图仅本地，Plan记命令/退出码/关键结果/hash，不新增非Markdown证据入Git。Spec提案本轮不发布，旧ledger只读。

## 7. 验收标准

- AC-01（owner_stage=work）：8app交付门与公共运行身份完整，Todo/Database/Blog缺口实际关闭；V-01通过后才可升候选pin。
- AC-02（owner_stage=work）：候选源等于固定SHA、完整注册集健康、根来源正确、后端profile分离、Reader/非目标不升级，V-02通过。
- AC-03（owner_stage=work）：Vue和iced两桌面均从入口完成8app真实用户任务及重开后再操作，V-03全过，无壳/假数据/外窗顶替。
- AC-04（owner_stage=work）：多app同场的路由/数据/身份与进程归属正确，失败可恢复，V-04/05通过。
- AC-05（owner_stage=work）：永久汇总门完整执行，V-06能检测真实错误，必需skip/pending/零场景为非零；SD候选与实际支持矩阵一致。
- AC-06（owner_stage=work）：落main前重放/候选回退正确、非目标不受影响、数据不逆迁移，V-07通过；源码/依赖漂移重核。
- AC-07（owner_stage=review）：冻结组合/源码/tool，独立最终复审AC-01..06及SD-01..03，不把实现门留给merge放行。
- AC-08（owner_stage=merge）：只合审定8app pin/必要声明及规范/文档；主检出对应子模块clean/detached、健康门绿、提交push可核验；Spec单源、Plan归档/计数/恢复收据和候选树deinit＋wt-guard后清理真实完成。非目标owner的WIP/分支不接管。

## 8. 执行步骤

- [x] T-00（owner_stage=work；依赖050/051；AC-01/02）：只读复核交付/公共工具和registry/backend全路径，输出assembly-decision；对尚缺前置写具体owner/关闭条件，此任务不需升级pin。[✅ 已完成 2026-11-04] 证据：7/8 交付门关闭（Todo-002、Database-002 已于 11-03 后归档交付并 push；Notes/Calendar/Paint/Chat/Charts 门在 056 基线上 intact）；Blog 唯一硬缺口，其 auto-ui 前置 PLAN-050 已 merge pass（master 024fe57a，vue.rs 发射 optimizeDeps.include+host 127.0.0.1+strictPort），BLOG-005 owner 会话已获授权路线①待 resume 归档。registry/backend 四臂全路径探针冻结 T-03 接线边界：Paint/Charts 走 inproc ③ VM session（其已验 profile）；Rust 六雄首选 outproc 孵化臂（auto run --autodesk-incubate，api:"rust" 自动起 Rust 服务），daemon 臂因 8 app 均无 /api/health 暂不可用（退路=app owner 补 health+release 二进制）。数据根六 env 逐仓 grep 核实。主检出 apps/auto-charts 检出 d5f5535≠gitlink 77b6f82 记录待 T-02 校正。全文见 plan-artifacts/plan-054/assembly-decision.md（sha256 06575569…48d8）。工作树 D:/autostack/.wt/os-054/auto-os（codex/plan-054-dev@f34edf1）已建，T-00 零代码改动。
- [x] T-01（owner_stage=work；依赖T-00及8app实际交付；AC-01）：关闭§5全部交付门并冻结delivery-lock；Todo002/Database002/Blog005未闭合时不能勾选。[✅ 已完成 2026-11-05] BLOG-005 于 2026-11-05 归档交付（832b71f，T-06 53/53 双实证），8/8 门关闭。Pin 锁：7 app 取 PLAN-056 hold 移交基线（=当前 remote tip，database 280ddac 与 tip b3499d5 src 字节一致），Blog 取其 005 交付提交 832b71f（6769d6e 主题基线为其祖先）；8/8 均逐仓实证为 origin/main 祖先（已 push）。逐 app Spec tree hash / 前后端 profile / 数据根 / pac 实值端口 / 路由前缀 / 工具链冻结（auto-lang 70b4a1de7、auto-ui b3414750、auto-man 03bb65e）全部入锁。全文见 plan-artifacts/plan-054/delivery-lock.md（sha256 3ecb71d4…202d）。
- [x] T-02（owner_stage=work；依赖T-01；AC-02）：建立唯一OS候选树，完整初始化所需组合，只把8app detached到已push交付SHA；验证健康/ID/源身份及错误反控，Reader和其他pin保持。[✅ 已完成 2026-11-05] 候选树 `.wt/os-054/auto-os`（codex/plan-054-dev @ fe53a04，基点 main cd90099）：24 gitlink 全初始化（含 notes 嵌套 vendor/auto-down @ c39ac7f9）；8 目标 fetch+detach 到锁定 SHA 并提交（8 files changed 纯 gitlink 增量）；16 非目标含 Reader 保持基线（逐条程序比对 pass）；组内补依赖树 auto-musk@de92b90 / jade-edit@5fb1e9e2 / auto-down@7c4d5cb（detached，manifest 臂兄弟解析命中）。V-02：健康门绿（exit=0，27 repo 条目，missing_pac_at=0）；反控三例全红且具名——删 pac.at（missing_pac_at=1）、charts 退回旧 pin（MISMATCH 具名）、旧根 main 检出（exit=1 报告其 d5f5535 漂移）；ID 唯一（28 条目）、8 目标 active 注册。**教训/决策**：`git submodule update --recursive` 会把刻意的 detach 重置回索引 pin——先递归初始化、后 detach 目标、再提交；`verify-app-links.py` 是 import 时期快照门（v0.6-app-import.json 停在 Plan 775，main 上已因 reader/calc pin 演进红），其断言恰作 pin 分歧检测器，不作候选绿门（无 override 参数，源码已核对）。日志在 plan-artifacts/plan-054/（health-gate-t02.log、v02-*.log、baseline-pins.txt、candidate-submodule-status.txt）。
- [x] T-03（owner_stage=work；依赖T-02及实际最小owner前置；AC-02/03）：在现有声明/接入机制收口后端profile、端口/路由和数据作用域；不开发app功能或重建服务生态。[✅ 已完成 2026-11-05] 声明增量 commit 3cd4585：calendar/paint/database/charts 补 back 端口（17821/17835/17839/17851，pac 实值）、Rust 六雄 launch:"native"（PLAN-910 发布 exe→Outproc 原生孵化，缺产物=StrictMissing 具名失败）、paint/charts launch:"vm"（其交付 profile，显式消假警报）；28 条目端口唯一程序核验；健康门增量后仍绿。数据作用域逐 app 核实：env 覆盖+按 app 子目录缺省回退，测试/legacy/probe env 全缺省关；Database 桌面任务用关闭状态夹具。机制全为现有交付能力，零 app/框架源码变更。详见 plan-artifacts/plan-054/wiring-decision.md（sha256 见同目录 .sha256）。
- [ ] T-04（owner_stage=work；依赖T-03；AC-03）：两桌面从真实入口跑8app用户任务、关闭/重开与下一次操作，独立核对数据/图形。
- [ ] T-05（owner_stage=work；依赖T-04；AC-04）：跑混合同场/旁观者/失败恢复及资源最后owner释放，保全用户数据和其他应用。
- [ ] T-06（owner_stage=work；依赖T-05；AC-05/06）：使聚合入口可执行，跑V-01..07及受影响旧门、变异反控、候选回退/重放，准备SD和真实支持矩阵。
- [ ] T-07（owner_stage=work；依赖T-06；AC-01..06）：核对最新main/共用SDK及053共享窗口、源/工具漂移和文档，提交execution_done。
- [ ] T-08（owner_stage=review；依赖T-07；AC-07）：同树独立上下文最终复审，强制区分单独与桌面、前端VM与后端profile的证据。
- [ ] T-09（owner_stage=merge；依赖T-08 pass；AC-08）：合入审定组合、规范/归档/push；主检出clean/detached/健康绿和回退收据验证后，deinit候选子模块、wt-guard再清理自有树。

## 9. 复审记录

- stage: new | plan_id: PLAN-054 | plan_revision: 1 | outcome: pass（仅T-00只读准备就绪；T-01交付门尚不满足，禁止pin升级） | next: work/T-00（等待后续实施指令）；TODO-002→work、DATABASE-002→final review/merge/push、BLOG-005→实际owner前置消费分别沿原计划。
- 本轮只查Plan/Spec/源码/提交祖先与远端，未构建/运行应用或桌面、未创建工作树、未修改app或升级OS子模块。053预留号和在途文件保留；不自动执行这份草案。
- stage: work | plan_id: PLAN-054 | plan_revision: r1 | outcome: pass（T-00 完成；T-01 被外部前置阻塞，保持 executing） | worktree_path: D:/autostack/.wt/os-054/auto-os | branch: codex/plan-054-dev | base_commit: f34edf1 | code_commit: f34edf1（T-00 零代码改动） | dependency_revisions: auto-ui master 024fe57a（PLAN-050 已并）/auto-lang 9f38bbf69/auto-man 2bbd954（观察值，T-01 重冻结） | task_ids: T-00 | evidence: 7/8 交付门关闭（TODO-002 archived delivery 4c51230、DATABASE-002 archived delivery f07d86e 均 push；056 具名 hold OS056-U054-PINS 移交 8/8 基线 SHA）；Blog 门未关闭=唯一硬缺口，其上游 auto-ui 前置已交付，owner=BLOG-005 会话 | blockers: T-01 需 BLOG-005 archived+push（auto-blog） | next: work/T-01（等 BLOG-005）
- stage: goal | plan_id: PLAN-054 | plan_revision: r1 | round: 1 | from: drafting | to: work | status: executing | outcome: pass | worktree: D:/autostack/.wt/os-054/auto-os | commit: f34edf1 | next: work/T-01（ Blog-005 外部前置；未关闭前禁止 T-02 pin 升级与候选桌面验收）

## 10. 待澄清事项

无须再次确认“先独立可用、后统一OS集成”的方向。不得通过降低Blog dev门、跳过Database原生review或沿旧Todo基线使前置看似满足。若某app确无法在声明的桌面承载运行，给最小反例与owner前置，受影响任务保持未完成。
- **T-01 硬前置（2026-11-04 T-00 结论）**：BLOG-005 未归档 = 唯一关闭缺口。其上游 auto-ui 前置已于 2026-11-04 交付（PLAN-050 merge pass @ master 024fe57a：optimizeDeps.include+strictPort+host 发射）；BLOG-005 目标会话已获用户路线①授权，resume→冷缓存 vue-dev-rust 至绿→T-02..T-07→归档。**关闭条件 = auto-blog docs/plans/archived/005-*.md status: archived 且 delivery commit 已 push**。此前禁止 T-02 pin 升级与候选桌面验收（§2/AC-01 红线）。Blog pin 不得用 056 移交的 6769d6e（仅主题基线、不含 005 作者页功能）。
- 主检出 apps/auto-charts 检出 d5f5535 ≠ gitlink 77b6f82：T-02 候选树按 pin 重建校正，本轮不动（与 11-03 核对一致）。
- **框架 owner 前置 ×2（2026-11-05 T-04 实证定界，阻断 AC-03 双桌面×8app）**：①iced 轨 Rust 六雄嵌载——`auto build -r rust` 生成码 68 编译错误为 app 源特化（002-counter 锁定组合对照 PASS），owner=auto-ui/auto-lang codegen——**已在上游立草案 auto-ui/PLAN-052**（.wt/ui-052/auto-ui 分支 codex/plan-052-dev @ 9faa5707，含 68 错误分族与 counter 对照证据），关闭条件=052 交付后 6 app exe 可构建且用户任务可跑；②vue 轨——desktop-host v1 单 api surface（8 app 仅 1 可嵌）+ run_ui 栈溢出（64MB 栈 workaround 实证）+ 3000 端口排除段（AUTO_FRONT_PORT 可绕），owner=auto-ui host 演进——**已在上游立草案 auto-ui/PLAN-053**（同分支，含单 api surface 与栈溢出证据），关闭条件=053 交付后 8 app 可嵌。按 §5.5 两轨受影响任务保持未完成、不退化冒称；其余可验证面（iced VM 双雄任务/同场/失败可见/注册健康）证据已备，恢复 /goal resume 续作。技术选择由T-00形成有界可行性/责任决策，只有实际变更目标/验收/跨仓写入范围才需新用户决定；不得通过降低Blog dev门、跳过Database原生review或沿旧Todo基线使前置看似满足。若某app确无法在声明的桌面承载运行，给最小反例与owner前置，受影响任务保持未完成。
- stage: goal | plan_id: PLAN-054 | plan_revision: r1 | round: 2 | from: work | to: work | status: executing | outcome: waiting（第 1 次，0/3） | worktree: D:/autostack/.wt/os-054/auto-os | commit: 0260a5e | next: work/T-01（外部前置未变：BLOG-005 仍 drafting，owner 会话正在 main 上实施 src/back；无阶段工作可执行）
- stage: goal | plan_id: PLAN-054 | plan_revision: r1 | round: 3 | from: work | to: work | status: executing | outcome: waiting（第 2 次，1/3） | worktree: D:/autostack/.wt/os-054/auto-os | commit: fafe548 | next: work/T-01（外部前置未变：BLOG-005 仍 drafting、owner main 实施中；无阶段工作可执行）
- stage: goal | plan_id: PLAN-054 | plan_revision: r1 | round: 4 | from: work | to: user | status: executing | outcome: blocked（连续第 3 次 waiting 后按协议标记） | worktree: D:/autostack/.wt/os-054/auto-os | commit: ad6365f | next: user（解除动作：BLOG-005 归档交付——owner=BLOG-005 会话；其上游 auto-ui PLAN-050 前置已于 2026-11-04 交付 @ master 024fe57a，owner 已获用户路线①授权，关闭条件=auto-blog 005 status: archived 且 delivery commit push 到 main。关闭前 T-01 不得勾选、T-02+ 禁止执行。已核验事实：7/8 交付门关闭且 push；T-00 决策文档在 plan-artifacts/plan-054/assembly-decision.md；工作树与分支已备。）
- stage: work | plan_id: PLAN-054 | plan_revision: r1 | outcome: pass（T-01 完成，交付门 8/8 关闭，delivery-lock 冻结） | worktree_path: D:/autostack/.wt/os-054/auto-os | branch: codex/plan-054-dev | base_commit: f34edf1 | code_commit: 未变（T-01 为只读锁定+文档） | dependency_revisions: auto-lang 70b4a1de7 / auto-ui b3414750 / auto-man 03bb65e（锁冻结值） | task_ids: T-01 | evidence: BLOG-005 archived @832b71f（2026-11-05，T-06 53/53）；8 pin 均 origin/main 祖先实证；锁全文 plan-artifacts/plan-054/delivery-lock.md（sha256 3ecb71d4…202d） | blockers: 无 | next: work/T-02（建候选树+init 组合+8 pin fetch/detach）
- stage: goal | plan_id: PLAN-054 | plan_revision: r1 | round: 5 | from: work（blocked 恢复） | to: work | status: executing | outcome: pass | worktree: D:/autostack/.wt/os-054/auto-os | commit: 831734b（阻塞解除：BLOG-005 归档交付） | next: work/T-02
- stage: work | plan_id: PLAN-054 | plan_revision: r1 | outcome: pass（T-02 完成，候选组合建成且健康门绿、V-02 反控全红） | worktree_path: D:/autostack/.wt/os-054/auto-os | branch: codex/plan-054-dev | base_commit: cd90099 | code_commit: fe53a04（纯 8-gitlink 增量） | dependency_revisions: 组内依赖树 auto-musk@de92b90 / jade-edit@5fb1e9e2 / auto-down@7c4d5cb（detached） | task_ids: T-02 | evidence: 健康门 exit=0 healthy；24 gitlink 程序比对全过；V-02 三反控红且具名；日志 plan-artifacts/plan-054/ | blockers: 无 | next: work/T-03（后端 profile/端口/路由/数据作用域收口）
- stage: goal | plan_id: PLAN-054 | plan_revision: r1 | round: 6 | from: work | to: work | status: executing | outcome: pass | worktree: D:/autostack/.wt/os-054/auto-os | commit: fe53a04 | next: work/T-03
- stage: work | plan_id: PLAN-054 | plan_revision: r1 | outcome: pass（T-03 完成，后端 profile/端口/路由/数据作用域声明收口） | worktree_path: D:/autostack/.wt/os-054/auto-os | branch: codex/plan-054-dev | base_commit: cd90099 | code_commit: 3cd4585 | dependency_revisions: 同 T-01 锁 | task_ids: T-03 | evidence: manifest 增量（launch 分离 native×6/vm×2 + back 端口×4）；健康门绿；端口唯一核验；wiring-decision.md | blockers: 无 | next: work/T-04（两桌面真实用户任务）
- stage: goal | plan_id: PLAN-054 | plan_revision: r1 | round: 7 | from: work | to: work | status: executing | outcome: pass | worktree: D:/autostack/.wt/os-054/auto-os | commit: 3cd4585 | next: work/T-04
- stage: work | plan_id: PLAN-054 | plan_revision: r1 | outcome: in_progress（T-04 中期实证记录，任务保持未完成） | worktree_path: D:/autostack/.wt/os-054/auto-os | branch: codex/plan-054-dev | code_commit: 3cd4585（未变） | task_ids: T-04 | evidence: ①工具链三件套按锁定 SHA 组内兄弟建成（auto/tool/ui-desktop 全 exit=0；补 auto-web 兄弟 37d32c22——T-01 锁表遗漏已修正）；②**iced 轨原生 exe 门红**：6/6 `auto build -r rust` cargo 101（生成码 68 编译错误，如 Option<serde_json::Value> 型别失配）；对照探针 002-counter 进行中以定界（app 源特化 vs 生成器广泛破损）；③**关键对照事实**：NOTES-004 归档回执显示各 app 交付的"原生"轨=VM+Rust split（auto run -r vm，MCP 实证），非全原生 exe——T-03 选的 PLAN-910 exe 缝对这些 app 非交付已验形态；④**vue 轨三障碍定位**：v1 desktop-host shell 不触发 per-app 生成（要求仓内已有 gen glue→正为 6 app 补生成）；shell 端口 3000 落在 Windows excluded range（EACCES，AUTO_FRONT_PORT 可覆盖）；run_ui 线程栈溢出（64MB RUST_MIN_STACK 下不复现=深递归型，疑宿主缺陷，主检出对照待做）。按 §5.5：具名失败保持门开，不退 VM 冒称；受影响最小接缝=框架 owner 前置（auto-ui/auto-man）。 | blockers: iced Rust 六雄嵌载缝（待 counter 对照定界后具名路由框架 owner）；vue shell 栈溢出根因 | next: work/T-04（续）
- stage: work | plan_id: PLAN-054 | plan_revision: r1 | outcome: in_progress（T-04 第二次中期记录） | worktree_path: D:/autostack/.wt/os-054/auto-os | code_commit: 3cd4585（未变） | task_ids: T-04 | evidence: ①**对照定界**：002-counter 原生 exe 在锁定组合 PASS（counter.exe 发布成功，0 错误）→ 6 app 的 rust exe 68 错误为 **app 源特化**（a2r/rust-ui 生成器对这些 app 的 back 源 constructs 失配），具名 owner 前置=auto-ui/auto-lang codegen，054 无授权修复；②**iced 运行面绿**：desktop-health 30 注册/29 可见/0 skip/候选根/隔离 storage；ACCEPTANCE 通道驱动建立——bus `launch\t024-charts` 走真实臂 inproc 启动（launch 声明生效）、per-app surface 快照/状态/截图证据在 plan-artifacts/plan-054/t04/（含 31 apps launcher + Charts 窗 PNG）；Charts CSV 导入 UI 可无对话框驱动；③per-app 输入投递链最后一环（handler 排队后状态未变）待解，见 t04/driving-notes.md；④vue 轨三障碍具名（单 api surface 为硬限制：8 app 仅 1 可嵌；栈溢出 workaround 64MB；端口 3000 排除段 AUTO_FRONT_PORT 可绕）。 | blockers: 同前（两框架 owner 前置）+ per-app 输入投递 | next: work/T-04（续）
- stage: goal | plan_id: PLAN-054 | plan_revision: r1 | round: 12 | from: work | to: user | status: executing | outcome: blocked（默认 12 轮上限到达；T-04 余量受两个框架 owner 前置硬阻） | worktree: D:/autostack/.wt/os-054/auto-os | commit: 4f6440d | next: user（解除动作见 §10：①auto-ui/auto-lang 修 a2r/rust-ui 生成器对 8 app back 源的 68 编译错误（002-counter 对照 PASS 证组合无恙）；②auto-man/auto-ui 演进 desktop-host 离 v1 单 api surface/栈溢出——或用户裁量 AC-03 承载目标。已核验可复用：T-00..T-03 全过、候选树健康、iced 运行面绿、验收驱动链、全部证据 plan-artifacts/plan-054/。恢复方式：/goal resume 同目标续 T-04。）
- stage: goal | plan_id: PLAN-054 | plan_revision: r1 | round: 13 | from: work | to: work | status: executing | outcome: pass | worktree: D:/autostack/.wt/os-054/auto-os | commit: 531b435 | next: 上游前置修复中（auto-ui/PLAN-052 已开工：T-00 五族定界完成+F4 已修实测在飞，ui-052 树 codex/plan-052-dev @ 400613e9；052 交付后回 T-04 iced Rust 六雄）
- stage: goal | plan_id: PLAN-054 | plan_revision: r1 | round: 14 | from: work | to: work | status: executing | outcome: pass | worktree: D:/autostack/.wt/os-054/auto-os | commit: 4ff2207 | next: 上游 PLAN-052 T-01 推进中——F4+F2 已修并实测 notes 50→27 错误（E0609 族全消），F3（dep style 常量注入）已实现待实测（ui-052 @ 392b2325）
