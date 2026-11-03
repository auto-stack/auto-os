---
plan_id: PLAN-053
status: archived
feature_name: 宿主主题链与auto-calc真实产品纵向接入
author: [Codex]
created_at: 2026-11-02
updated_at: 2026-11-03
plan_revision: 1
current_step: 7
total_steps: 10
supersedes_spec_components: [docs/specs/shell/showdesk-ux-polish.md]
new_spec_components: [docs/specs/shell/host-theme-policy-and-delivery.md, docs/specs/apps/calculator-theme-adoption.md]
touched_goals: []
affects: [auto-os-shell, desktop-theme-config, auto-ui-host-adapter, desktop-protocol, auto-calc, apps-manifest-and-gitlink]
depends_on: [auto-ui/PLAN-031]
---

# [auto-os/PLAN-053] 宿主主题链与auto-calc真实产品纵向接入

## 0. 变更摘要

承接[UI031合同](../../../auto-ui/docs/plans/031-theme-scope-publication-and-package-closure.md)和[架构重审设计](../../../auto-ui/docs/design/autoui/post-extraction-architecture-and-theme-closure.md)的O包：在已有AutoOS桌面配置/bus/主题应用链上补正式包版本和family/mode策略、版本化通知/重连/拒绝/持久化，选择**auto-calc独立产品**做宿主内与独立进程、Vue/VM实际纵向接入，并闭合007遗留真实系统主题事件验证。

当前drafting r1、0/10。2026-11-02用户只授权立正式计划、暂不实施；无worktree/产品改动/子模块升级/系统设置变更、构建或测试。UI031仍在途，仅按合同定义接收门，不声称端口已交付。本计划主责OS，参与UI/产品只留定位，不另造竞争主合同。

## 1. 目标

- G1：宿主持有可持久化theme family/mode policy和有效来源，基于031选择/作用域端口完成桌面shell与已接入App准确应用；启动、bus修改和config外写同一规则。
- G2：同进程应用与独立产品进程按会话/版本/revision接受快照，重复/乱序/断线/重连/拒绝有明确结果，不把全机瞬时原子换肤作为承诺。
- G3：auto-calc消费固定正式主题包/接口，独立默认/显式override/跟随正确；现表达式/Scientific/Programmer业务、状态、焦点/窗口语义保持。
- G4：系统真实事件、重启配置恢复、包来源/错误恢复、产品clone交付与OS gitlink组合均实际验证；向其余产品给可复制的契约而非源码拷贝。

非目标：全产品接入、UI六教学示例迁移、重实现031 scope/原子发布/正式包、通用RPC/OS服务supervisor、Linux发行版/Reader/AG路线、全面状态HMR、重做计算器引擎、默认生产部署/远端建仓/强推、全机缓存清理。

## 2. 架构方案

OS拥有policy与存储/通知路由，UI拥有数据schema/包验证/单App原子选择和真实控件投影，auto-calc拥有自身业务及跟随声明；实施时产品README添加本OS主合同反链，本轮不在产品main顺手改文档。现桌面基础实现部分物理在UI仓，不因主责OS就复制到shell.at；参与UI树只做host adapter/配置/现协议的必要扩展，不反向把OS产品代码塞入语言基础。

流向：shell/bus/config/真实系统事件 → 宿主验证候选 → policy持久提交/分配revision → 按session发送完整快照 → UI031端口本地原子验证/应用 → applied/rejected状态回读 → 下次连接获取最新完整快照。每App本地整pair原子，多个进程可逐个追上最新版本；失败App保旧且宿主可知，不能伪造全局成功。

不另造通用RPC或服务；优先扩已有DesktopCommand/desktop_protocol连接与配置轮询。独立Vue预览如何读取宿主选择/覆盖需用已有合适本机transport受管接线；若现通道无法承载，T-01给最小版本化桥接合同和支持矩阵，不借UI033完成或未批准OS049泛通信当隐含前置。

## 3. 技术栈

AutoOS shell/desktop.at、apps.manifest与gitlink；UI `ui/session.rs`、`ui/desktop_config.rs`、`ui/desktop_protocol`、renderer host/snapshots；UI031正式包与host adapter、Man实际启动/安装输入；auto-calc pac.at/src/front与现tests。保owned进程、真实handler、输入测试入口，复用autoui-verifier/原MCP/Playwright，不造通用runner。

## 4. 需求分析与背景调查

### 4.1 授权与仓规则

已读本仓[AGENTS](../../AGENTS.md)、现[桌面主题规范](../specs/shell/showdesk-ux-polish.md)、[桌面台账](autos-desktop-program.md)，以及UI[应用迁移](../../../auto-ui/docs/specs/ui/theme-app-migration.md)/031/重审设计、产品pac/src和tests/acceptance.atd。用户明确本轮仅规划；本仓无docs/plans/README，不虚构索引，以autos-desktop-program新增定位入口。

本仓main规则：Plan通过scripts/new-plan.sh独占取号，archive终态；OS/UI源码按专属worktree。**产品例外沿本仓AGENTS §2.1**：开发/复审使用`D:/autostack/auto-calc`独立clone的main且一写入者，组合`auto-os/apps/auto-calc`只作detached固定pin。不得在组合检出编辑产品，不将整个App开发搬回教学例，不自动stash/checkout他方WIP；产品提交/正常push与组合fetch/pin按该显式流程在获实施授权后完成，本轮无远端动作。

快照：OS HEAD `65fc70de8b505657db660d91024b2cf4425b7759`；UI `27f1edcd3396d3d9a31aadb96431fed1a2d97153`（031未落地）；calc独立clone HEAD及OS当前gitlink均`e5247c549db375480f30b9eb1e417ad9b15191a3`，本次calc status clean。OS已有auto-charts gitlink/docs等WIP、UI dynamic_build/ui_tests等WIP不消费/提交/还原。执行时重核，不能据现在clean认领未来工作。

2026-11-03 goal 执行授权（回合1）：用户以 goal objective `完成计划053` 启动交付循环。按 auto-plan-goal 授权合同，该 goal 即本计划全链执行授权：建立/使用唯一实施组树 `.wt/os-053`、work、独立 final review、契约内有界修复、merge（canonical 沉淀/归档/清树）。§10.1 的「本轮只规划」由本授权解除；goal/AC/范围/预算/仓库集不变。启动门核验（2026-11-03 实测）：U031 已归档落地——auto-ui master HEAD `68a099ac` 含 031 交付提交 `b6c26186`（第八次独立 final review pass，canonical specs 与 `packages/themes/` 已落地）；auto-calc 独立 clone `D:/autostack/auto-calc` HEAD `e5247c5` clean 且等于 OS gitlink；`.wt/os-053` 无既有组、无竞争 relay/写入者。§10.2 真实事件窗口授权未取得——T-06 前如仍未授权，按 §5.5 停候（V-07 真实腿 pending，不冒 PASS）。

### 4.2 已有入口与边界

| 实物 | 已核事实 | 本包剩余 |
|---|---|---|
| shell/desktop.at | set_theme dark/light按钮与SendCmd真实通道 | 接统一policy/family/mode选择与可读拒绝；不把纯按钮改色当完整链 |
| UI ui/session.rs | DesktopCommand::SetTheme(bool)/SetThemeName(String)，encode/parse共用线上set_theme词 | 旧暗/亮和内置名兼容，正式包来源/协商/完整快照/revision需新版本明示，不能把新对象塞旧窄字符串骗兼容 |
| UI ui/desktop_config.rs | dark_theme/theme_source/theme_name，load/save和外写mtime应用；AUTOOS_DESKTOP_CONFIG显式路径 | 加policy/包来源/revision及安全schema迁移，未知字段/其他设置保持；system resolved不反写成manual |
| 现主题热切换规范 | bus与config外写两条链都须adapter切换、全场snapshot invalidate、App view_dirty/dark_mode回写 | 继续对齐face/缩略/pager/壁纸和shell面板，不能只主窗换肤；新跟随不直接覆写产品业务模型 |
| auto-calc真实源 | pac去显式theme以跟随，accent indigo、stylekit vendor，front_port17810；表达式、Scientific、32位Programmer原引擎/测试 | 新正式包接入/follow与override、真实host/独立启动、业务状态保持；不以UI011-calculator示例替代 |
| 031合同 | UI端完整选择/失败保旧、App/window隔离、正式包获取和受控host输入/结果 | 接最终版本/命名和支持表，OS负责通知/存储/重连/实际系统事件，不回传完整scope实现给031收尾 |

## 5. 详细设计

### 5.1 启动门、唯一上下文与接缝冻结

实施启动=本合同明确执行授权 ∧ 031独立review/合入/最终canonical/交接可接受 ∧ auto-calc单一写入者/来源可用 ∧ T-01资源窗口和所选通道/支持合法。031仅work完成不放行；[UI033](../../../auto-ui/docs/plans/033-basic-examples-theme-migration.md)不构成本包前置，二者都接同版031，可分别实施、共享写面短窗串行。

OS核心唯一`D:/autostack/.wt/os-053/auto-os`（`codex/plan-053-host-theme`），参与UI一树同组分支、lang/web/man/down按实际依赖补固定来源，不建链接；产品按§4.1唯一独立clone，复审与实现顺序共用它，不另造同步副本。计划唯一簿记在OS main。

T-01授权后才建组/做probe：接受031最终API/包版本/owner、冻结OS/shell/UI/protocol/产品的来源和exact argv/env/config/端口/原业务门，核同进程及独立Vue/VM现通道、回读/拒绝位置，最多两个有据传输候选择一，给完整纵向hello→snapshot→apply→ack→reconnect探针。若需扩大通用协议/服务/Core，先needs_replan，不把通信路线049/应用全线作为新全局前置。

所有测试配置先显式`AUTOOS_DESKTOP_CONFIG=<统一scope>/config/...`，App存储/端口/宿主rendezvous亦钉自有测试实例；真实系统事件腿单独窗口。不得修改用户真实桌面配置、壁纸或运行中其他会话来制造恢复。必要桌面启动前按AGENTS跑scripts/check-app-submodules.ps1（用实际-OsRoot指向所验组合）；缺失/指针/批量删除/manifest健康红先核归属处置，不静默省略，更不为主题跑所有App业务套件。

### 5.2 Policy、兼容与持久化

policy记录family的固定包标识/version/content digest、mode=light|dark|system、默认/显式来源、schema和主机分配的单调revision；resolved_mode是运行派生，不持久化为显式mode。每产品声明follow或明确override，优先级接受031：受控CLI/App override > follow宿主 > 独立App默认 > legacy回退。宿主不知道/不支持包或mode时拒绝候选，保旧policy/有效选择，不偷偷替成另一个族成功。

沿旧dark_theme/theme_source/theme_name加载，明确缺键的历史默认与兼容映射；legacy builtin stella/scaffold/旧accent仍按031兼容规则，不强迫所有旧产品切到Auto/light。新选官方Auto/Solarized只向已协商支持的会话发新包选择，未选产品保持旧支持范围并返回明确legacy/unsupported状态，不声称全产品已接入。后续真实App接入逐仓扩。

持久化采用完整候选校验→临时文件/原子替换提交→发布revision，写失败不改变内存policy、不发成功ack；保存成功但广播前崩溃后，从磁盘完整快照恢复并重发。保其他未知配置/壁纸/面板/用户设置，不因新增schema整文件重置。config外写与bus修改经过同一验证/应用入口，重复本机写不触发反馈循环。

### 5.3 通知、session与恢复

沿已有本机desktop通道定义版本化握手/完整快照/结果：请求者session/epoch/当前能力与包标识，宿主revision，App返回applied或rejected含有效版本和具名原因。具体字段/codec在T-01按031端口和现协议上限冻结，不先虚构已存在消息。只向当前已绑定会话发送，跨App隔离；过期session、重复/低revision忽略且结果可读，重连/新开窗先获得最新完整快照，不只监听未来增量。

单App调用031原子端口，失败整个family/mode保持旧；宿主展示每App pending/applied/rejected/unsupported，不把发送成功当渲染成功。转义/长度/消息版本/包来源/授权按现本机通道保护，外来字符串不作shell/路径执行。不为适配全部程序开公共网络服务；旧客户端握手不支持新帧时可用原暗亮兼容，不能静默吞掉新family请求。

App退出/断线清除其订阅/旧epoch，宿主重启/产品重连恢复；owned子进程/管道/reader和rendezvous释放后同地址可重开。不按映像名终止别人的auto进程，不触OS049的会话。shell/face/缩略/pager/壁纸/悬浮面板失效均按现P041规则，不能一处新theme另一处旧snapshot。

### 5.4 真实产品：auto-calc

产品源使用固定正式themes包+UI031控件/选择声明，以已锁来源/版本获取；不复制配色表、不更换包源冒fresh。接入点pac/dep及必要front design/model消息/旧SetTheme等（按实际代码T-01），UI可调用follow不写业务字段；原calc的Basic/Scientific/Programmer模型/消息/算法及窗口fit保持。

产品验收至少覆盖现tests/acceptance.atd原义务与test_011_vm.py/test_011_vue.mjs/desktop_mcp.py/test_512_fit_remeasure.py实际适用面（启动后列出精确ID/条件，不将脚本存在当已运行）。关键区分输入：2+3=5、2*(3+4)=14、科学sqrt9=3、Programmer HEX FF+1=100及错误态/清除原行为；切family/mode、host跟随/override、断线重连后表达式/内存/所选模式/错误和焦点保持。不能只用显示0静态截图替换整产品功能。

真实同进程宿主里打开calc/另一个不接入的legacy控制，独立calc VM进程和Vue独立运行/宿主适配都验证；同App两窗共享正确session选择、显式preview例外按031端口，两个独立App/会话不串。现auto-calc格局和类目运算符/科学键语义色按App命名空间映射或合法固定值记录，不破坏键位意义。

work冻结产品候选提交/待接收pin并验证固定来源组合；最终独立review通过后，merge按现App流程完成正常push，再在干净且无他人写入的OS组合子模块fetch并detached到已交付SHA，更新gitlink/必要manifest定位、运行健康闸门和组合实际启动，才称OS组合交付；不能仅在clone跑绿或把main分支名当pin。若执行时remote不可用如实阻塞相应交付，不强推/改远端URL/借本机未发布路径伪完成。

### 5.5 真实系统事件、资源与回滚

闭合007遗留真实系统主题事件：受控provider/注入事件与**真实操作系统偏好改变→监听/轮询→policy resolved→桌面/运行calc**分别实证。Windows指定验证窗口内先记录原用户设置，取得实际操作授权后改变并等待真事件，精确恢复原值/进程状态，finally失败也报告残留；没有授权/事件能力就保持V-07 pending，不把受控绿或临时skip充PASS，不宣称本包完整交付。Linux/Mac另列平台未测，不冒跨平台全绿。

回滚仅本计划owned桌面/产品实例、测试配置和已确认组合提交；停订阅/子树后逆向恢复OS/UI/产品版本与gitlink，从备份policy schema受控降级，不恢复陈旧绝对target或把用户新配置覆盖成旧值。新schema不可被旧工具误写，应明确拒绝/迁移诊断。用户真实配置未获授权不写，测试配置缺失/损坏走可恢复诊断；不全机清缓存。

### 5.6 交接与非全局门

| 对方 | 本包接受/交付 | 边界 |
|---|---|---|
| UI031 | 接最终正式包/选择端口/原子拒绝/多App生命周期支持表，版本不够先具名修复/重规划 | 本包只做host/config/transport必要UI改动，不替它补scope/正式包/控件必需验收 |
| [UI033](../../../auto-ui/docs/plans/033-basic-examples-theme-migration.md) | 同包/优先级/接口组合，互交实际SHA与消费说明 | 互不等待，六教学例不是calc产品，不能相互替验 |
| 其他产品 | 提供固定包/消息/schema/follow声明/重连/拒绝/独立启动及OS pin范本 | 仅calc交付，不改全部产品；各仓另约、组合gitlink明示 |
| OS049/052/Reader/AG等 | 按真实共享入口/协议/资源窗口协调 | 不接管泛通信/发行版/Reader/AG合同，不把全线完成追加前置 |

### 规范增量

| delta_id | add/modify/retire | docs/specs/... target | before/after rule | rationale | acceptance IDs |
|---|---|---|---|---|---|
| SD-01 | add | docs/specs/shell/host-theme-policy-and-delivery.md（新） | 旧暗亮/内置name链→正式包标识/完整policy与版本化通知/结果、重连/拒绝/系统派生 | OS域唯一事实，非新通用服务 | AC-01..04/06 |
| SD-02 | modify | docs/specs/shell/showdesk-ux-polish.md | P041双路径/快照失效保留，补新scope/版本的实际应用和失败边界 | 不让shell/face/pager保留旧颜色 | AC-02/04 |
| SD-03 | add | docs/specs/apps/calculator-theme-adoption.md（新）；calc仓拟docs/specs/theme-adoption.md（新） | 旧跟随链→正式包/follow/override与业务状态/双端/组合pin证据 | 独立产品不等于教学例 | AC-05/07 |
| SD-04 | modify | UI拟docs/specs/runtime/host-theme-transport.md（新，参与仓）及031主题页只按真实差额 | UI端选择合同不变，记录host adapter/protocol/config的实际新能力 | OS持主合同，不在compiler或语言侧造OS实现 | AC-01..04 |
| SD-05 | modify | docs/specs/shell/host-theme-policy-and-delivery.md（SD-01新页） | 无真实OS事件/恢复/pin交接→具名实证、平台边界与其余产品接入合同 | 历史pending不因归档自动关闭 | AC-06..09 |

frontmatter仅本仓路径；跨仓Spec实际差额T-01按owner冻结。当前无canonical/legacy JSON写入，应用派生索引。

## 6. 测试设计

下列全未执行，T-01产生准确命令/测试ID/协议字段/本机endpoint及合法负例；复用现桌面/protocol/config/产品真实测试设施，不造镜像实现checker。

| Case ID | AC / 输入与真实链 | 区分性oracle / 阶段 |
|---|---|---|
| V-01 | AC-01；031正式端口握手/包/version/capabilities，新旧客户端与错误消息/身份 | 固定来源/语义明确，错版本/包/模式/会话拒绝并可回读，不把旧dark/light帧误读新family。work/T-01/02 |
| V-02 | AC-02；bus选择及config外写同候选、保存失败、未知字段/旧schema加载 | 两路径同有效结果，保存失败无内存半更新/广播成功，未知其他字段保留；旧默认可恢复。work/T-02/03 |
| V-03 | AC-03；相反选择/override的同进程两App/两窗、独立VM/Vue calc会话 | 按真实owner不串色、输入/业务保持，拒绝App旧pair完整，host applied/rejected分列可读。work/T-03/04 |
| V-04 | AC-03；重复/乱序revision、断线、新session、宿主重启与最新snapshot | 不倒退/不把旧session ack给新App；重连取得完整最新选择，不需再改一次设置。work/T-03/05 |
| V-05 | AC-04；切族/模式时shell/face缩略/pager/壁纸/面板与运行App | 实际像素/快照/状态一致失效，无旧frame滞留；不是只测SetTheme字段。work/T-03/05 |
| V-06 | AC-05；calc原acceptance及关键求值/科学/HEX/错误态→换肤/跟随/override | 原全部适用业务门保留，表达式/内存/mode/错误/焦点与输出不变，真实handler/MCP/Vue运行。work/T-04/05 |
| V-07 | AC-06；受控system与真实OS偏好改变分别入现provider→宿主→运行calc | 真事件可定位、resolved变化不写成manual，重启仍system；真实用户设置原值恢复有证。无许可/真事件未测不PASS。work/T-06 |
| V-08 | AC-01/05；正式包外部fresh/locked/第二根、缺源/错hash/不支持模式及恢复 | 独立启动可消费固定source，拒绝保旧且恢复后下一次真实使用成功，无主检出fallback；沿906/910。work/T-04/05 |
| V-09 | AC-04/06；进程/reader断开、候选错误、UI关闭重开、测试config损坏及回滚 | owned树/订阅/lease/endpoint释放，其他会话不受影响；恢复后可再次启动/跟随，残留如实列。work/T-05/06 |
| V-10 | AC-07；work验证固定候选SHA的组合和健康/实际启动；review后merge完成正常remote→OS干净组合fetch/detach/gitlink再读回 | 所测来源==产品交付SHA==OS pin，旧产品/manifest ID与端口稳定；仅clone通过不算组合通过；candidate与正式pin分列。work/T-06/07、merge/T-09 |
| V-11 | AC-08；各AC/真实传输/错误/原子policy/产品/SD与后继表 | 独立final review绑定OS/UI/calc SHA及031来源，无未解释的必需gap，不能让作者自查代替。review/T-08 |
| V-12 | AC-09；冻结受影响门与实际合入/pin/知识/归档/全输出路径 | 真实落地＋guard清理、系统恢复与持有对账齐，不冒全产品或全平台完成。merge/T-09/10 |

OS .at/配置/manifest资产本身不触发语言cargo t/docs_gen。若参与UI Rust host/protocol/config实改，按其真实模块/旧协议roundtrip/config外写/生命周期/renderer定向反控和UI最终t、生成tu单进程/必要ti职责并集；不默认全AG/ACE。calc保本产品原acceptance完整适用义务及VM/Vue脚本；T-01列排除/真实执行数与命令，父exit0不替子项。旧环境flake/原红/未执行分列，必需面红不以既存归因当通过。

每次桌面启动/组合合入按AGENTS的子模块健康检查，示例脚本路径误指旧示例根时只修产品真实测试入口，不拿教学源替产品。同机输入/GPU/端口/磁盘资源窗明确，不占他方桌面；真实系统设置变更须实际窗口授权。review同组+同一产品clone独立上下文，修复回原写入者并使相关证据失效。

## 7. 验收标准

| ID | 可观察验收 | owner_stage |
|---|---|---|
| AC-01 | 031最终版本/正式包/host选择支持被接受，版本化消息/身份/错误和独立来源准确，V-01/08 | work |
| AC-02 | 完整policy校验/持久提交/内存发布、bus/config外写与旧schema兼容，失败保旧且不丢其他配置，V-02 | work |
| AC-03 | 同进程/独立Vue/VM的会话隔离、follow/override、applied/rejected回读和最新revision重连恢复正确，V-03/04 | work |
| AC-04 | shell/face/pager/面板/壁纸失效与真实App同步、退出/失败/回滚资源恢复，不泄漏/影响他方，V-05/09 | work |
| AC-05 | auto-calc正式包与现Basic/Scientific/Programmer全部适用业务、状态/焦点/原测试义务双端保持，V-06/08 | work |
| AC-06 | 真实OS事件链和system恢复实际验证、用户设置精确恢复，受控/真实/平台限制分列；缺真事件不完整通过，V-07/09 | work |
| AC-07 | 产品独立clone候选提交/OS拟接收pin和registry固定来源组合验证、健康门与实际启动同SHA，后继产品契约明确；正式remote与gitlink落地属AC-09，V-10 | work |
| AC-08 | 独立final review逐AC/错误/健康/遗漏/workaround/SD pass，绑定冻结OS/UI/calc与031组合，V-11 | review |
| AC-09 | 冻结门、实际各仓落地/产品pin/canonical/交接、归档/所有材料与树/输出guard清理，具名持有有owner/关闭条件，V-12 | merge |

## 8. 执行步骤

| ID | 动作/实际触面与关闭条件 | 依赖 / AC | owner_stage |
|---|---|---|---|
| T-01 | U031及实施授权后建OS/UI唯一组并认领干净产品clone；冻结源码/pin/包/端口/命令/支持/资源、protocol全链probe和用户真实事件窗口 | 启动门；全部work AC，V-01 | work |
| T-02 | OS shell入口＋UI desktop_config/session旧协议兼容，定义完整policy/schema、校验/持久化/拒绝和031adapter接线 | T-01；AC-01/02，V-01/02 | work |
| T-03 | 现desktop_protocol/bus通知/握手/结果、会话revision/快照重连、shell/face/pager失效；只做host必要UI变化 | T-02；AC-03/04，V-03..05 | work |
| T-04 | auto-calc独立clone pac/正式dep/design/follow/override接入，保持原业务/类目色，独立Vue/VM入口和包正负消费 | T-03；AC-01/05，V-06/08 | work |
| T-05 | 宿主内＋独立进程的完整产品/乱序拒绝/重连重启/多窗/快照/关闭回滚并集；非选legacy控制 | T-04；AC-01..05，V-01..06/08/09 | work |
| T-06 | 获准窗口实测真实系统事件及原值恢复；产品现业务/健康门全闭；冻结产品已提交候选SHA及OS待接收pin，以显式固定来源验证候选组合（不冒生产gitlink已升级） | T-05；AC-06/07，V-07/09/10 | work |
| T-07 | 健康/遗漏/workaround、全部work证据与SD/来源/pin/后继交接冻结，execution_done7/10 | T-06；AC-01..07 | work |
| T-08 | 同组及同产品clone独立final review，finding回write owner/失效对应证据，不复制第二实现树 | T-07；AC-08，V-11 | review |
| T-09 | 冻结受影响门，review后产品正常push、干净组合fetch/detach/健康与精确pin读回，各实际仓依赖序落地/产品gitlink正式合入、canonical/台账/后继来源读回 | T-08 pass；AC-09，V-12 | merge |
| T-10 | 原设置/进程/全输出路径对账，摘要折Plan后删统一scope/自有树产物，各worktree先guard clean再移除、产品clone保留不删除/重置，归archive10/10 | T-09；AC-09 | merge |

统一材料根`D:/autostack/plan-artifacts/plan-053/auto-os/`（跨仓含auto-calc以repo身份分区），登记owner/用途/在途。外部源/SDK/发行包/测试配置/日志截图/外置target都在统一scope，正常OS/UI构建在组树、产品本地编译沿clone授权自有路径并纳输出对账；不写attachments/第二套临时根，不扫清真实用户缓存。merge将命令/退出码/关键数字/源hash/节选折本Plan，删除本计划整个scope；仅具名未闭持有owner/理由/关闭条件。全部实际使用目录/设置备份逐项核，不盲删未知/他方目录或活动进程。产品clone与组合源码不是临时worktree，不随组清掉。

## 9. 复审记录

2026-11-02 / stage:new / auto-os/PLAN-053 r1 / outcome:blocked（U031正式交付/本包实施授权未满足） / 已核现SetTheme/config/poll/shell入口与calc独立clone/pin及原测试责任；静态预检pass：0–10章节、10T/9AC/12V/5SD唯一连续、本地引用有效、无尾空格、053跨活动/归档唯一且号源单调；定义完整host＋单产品纵向合同，真实OS事件为必需腿而非受控代理。next：明确实施、接031最终API/包版本后T-01冻结全链/资源与真实事件窗口。当前无新实现/测试/建树/产品或子模块修改/系统设置变更/远端动作。

2026-11-03 / stage:work(回合1) / auto-os/PLAN-053 r1 / outcome:pass(T-01全闭+T-02完成+T-03实现待outproc实证) / 授权记录见§4.1（goal `完成计划053` 即全链执行授权；U031已归档落地 b6c26186 核验）。T-01：组树五仓冻结（OS `65fc70de`、UI `cfe325d4` 含031、lang `9c81ee6d`/web `3de5524e`/man `47d8f7b3`/down `7c4d5cbb` detached 只读依赖、calc clone `e5247c54` clean=gitlink）；冻结文档 `plan-artifacts/plan-053/auto-os/T01-freeze.md`（传输裁定=候选A扩desktop_protocol+DesktopCommand，候选B仅standalone腿复用config轮询；policy schema/会话/恢复/calc接入面全冻结）；组构建三二进制通（`--locked` 因组兄弟版本前移降级为常规构建——auto-man/auto-os Cargo.lock 更新仅组树内未提交）；真实桌面 probe 8/8 PASS（catalog 注入 themes v0.2.0 digest=9fa07742..、bus 提交写盘 rev1、重复写 no-op、未知 family 具名拒绝保旧、config 外写同入口 rev2、legacy set_theme 回归、`__theme_*` 状态面 autoui_snapshot app=Desktop 可读）。基线修复：master `cfe325d4` mpv-widget 下 media.rs 裸引私有 thread_local E0425 → projection 只读访问器（ad883db4 内）。T-02：policy 核心（catalog 注入/stage→落盘→publish/boot 恢复/eval 注册表/shell 脏位，8 单测）+ config 策略组五键（roundtrip/merge 单测，test-lock 修 712/03 用例既有并行窗口）+ `DesktopCommand::SetThemePolicy` 两参动词（词表 roundtrip+负例）+ 执行臂/外写同入口/allocate_app 播种 + os-desktop catalog init/boot 恢复 + desktop.at 策略区六直选与状态/具名拒绝行。T-03：Hello `theme_caps` 尾段 + ControlMsg 17/18（push/result，连接级寻址）+ attach 即推（broker/legacy 双路）+ 提交臂广播 + pump 队列排水 + stale ack 具名分列 + client 侧 031 端口喂入/帧后读回/结果上行（协议 roundtrip 17 单测）。证据：auto-ui `ad883db4`（21 文件 +1525）、auto-os `53f50c9`、probe 脚本与日志 `plan-artifacts/plan-053/auto-os/probe-*`。worktree：`.wt/os-053/{auto-os,auto-ui}`；base/依赖 commit 见 T01-freeze §1。blockers：无（真实事件窗口授权仍待用户——T-06 前置，§10.2）。next：T-04（auto-calc clone 正式包接入）。

## 10. 待澄清事项

1. 本轮只规划，两包共同等031正式交付，UI033不是OS053前置。031未交付或真实支持端口不足，先给精确阻点/owner与合同修订，不偷做scope/泛通信全套。
2. 真实OS主题事件验证需实际窗口授权、备份与finally恢复；未获授权/无法验证时保AC-06 pending，不以031受控provider/007归档代替。无需再次批准已确定的auto-calc选择或旧业务保留方向。
3. 产品按OS AGENTS独立clone main一写入者、提交/push后组合fetch/pin，不在apps/组合编辑。执行时WIP/remote权限/并发不同，不能擅自切换他人内容或重置产品目录。
4. 旧policy默认/官方包版本/transport字段和拒绝语义在031实际出口基础上T-01冻结；若需公开兼容变化/远端新动作，给具体差额，不以宽泛架构方向当授权。
5. 053仅单产品，后续产品各仓独立验收；不因系统事件/通信交集接管OS049/发行版052、Reader或AG线。实际输入/GPU/存储预算开工确认，不清未知缓存补空间。

2026-11-03 goal 授权后：第 1 条「本轮只规划」已由 goal `完成计划053` 解除（授权记录见 §4.1）；第 2–5 条边界保持有效——第 2 条的真实 OS 事件窗口授权截至回合 1 未取得，T-06 到达时仍未授权则按 §5.5 停候。

2026-11-03 / stage:work(回合1 续) / auto-os/PLAN-053 r1 / outcome:pass(T-04 主体验证闭；standalone 跟随腿遗留) / T-04：calc 独立 clone 候选提交 `e7c067f8`（main 本地未 push——push 留 T-09）：`vendor/themes` 固定快照（digest 与源包/宿主 boot log 三方一致 `9fa07742dc68aa53...bef838e`）+ pac `dep themes` + `design {theme:"Auto", mode:dark_mode}` 独立默认腿 + README 主合同反链 + acceptance.atd T8-T10。**原业务门 21/21 PASS**（desktop_mcp.py 全量：2+3=5、1+2+3=6、3.5+1=4.5、2*(3+4)=14、2^10=1024、T9 结构负例 + 新增 sqrt 语义钉定/HEX FF+1=100（pacc 256 + 读出渲染"100"）/除零错误态）。**测试设施修复（产品真实测试入口，pin 基线同现故非 053 回归）**：press 结果为按键瞬间早读（服务端按下即读，VM handler 效果下一拍落地）→ settle 后权威读出；`autoui_find` 为子串匹配（"+"→"+/-"）不用于按压定位——精确匹配单键序列为可靠原语；vnode id 随 fit 重测换代，先存 id 后按会打陌生槽位。**具名缺口（非 053 范围）**：表达式引擎不识别函数 token——`sqrt(9)` 求值得 9（函数名被直通忽略，括号求值照常）；引擎函数能力属产品后继（非目标：重做计算器引擎），T8 已钉定当前语义。遗留：standalone `auto run -r vm` 窗口的 live 跟随腿（AUTOOS_DESKTOP_CONFIG 轮询喂 031 端口——运行链 run_entry_file→run_file_dynamic_ui_inner 深处，归 T-05 独立进程验证面一并实施）；桌面内 calc（in-proc vm）腿与 outproc 实证归 T-05。next: work（T-04 跟随腿补闭 + T-05 V-01..06/08/09）。

stage: goal | plan_id: PLAN-053 | plan_revision: r1 | round: 1 | from: drafting | to: work | status: executing | outcome: pass | worktree: D:/autostack/.wt/os-053/auto-os (+auto-ui 同组) | commit: 53f50c9(os)/ad883db4(ui)/e7c067f8(calc 候选) | next: work

2026-11-03 / stage:work(回合2) / auto-os/PLAN-053 r1 / outcome:pass(T-04 全闭 + T-05 主腿实证；outproc-attach 端到端具名遗留) / **T-04 全闭**：standalone `auto run -r vm` 跟随腿落地并 4/4 PASS（`verify-standalone-follow.py`：boot 策略恢复 Auto/dark、config 外写同入口跟随 light、未知 family 拒绝保旧 light——像素级判定）。实现：theme_policy 目录发现装载单源（宿主/standalone 共用；digest 算法迁 compiler 为包身份唯一实现、tools 委托）+ `arm_standalone_follow`（env 门：catalog + boot 恢复 + 400ms tick 武装）+ ServiceTick 订阅门 + `run_file_dynamic_ui_inner` 武装位。**采纳者 dark_mode 语义修正**（031 合同内）：family 链激活时 scoped 组件的 `dark_mode` 由**已提交策略派生 mode**驱动（视图类分支读它）——cfg.dark_theme 翻转（含被拒候选的 legacy 外写）不得越权改色（execute_set_theme/外写差异臂/allocate 播种三处同语义）；根因链实证：calc 视图类分支读 dark_mode，后者曾被 rejected 候选的 legacy 翻转污染。**031 合同补课**：`use themes.themes: Auto` 缺失 = 包主题不入收集链（C-11 未注册拒绝）——calc 候选 `a1f38842` 补上。**T-05 实证**：桌面 probe 8/8（回ities 01）+ calc 21/21（业务+三区分用例）+ standalone 4/4 全绿于同轮复跑。**outproc-attach 端到端具名遗留（非 053 回归）**：外部自采纳客户端（adopt→桌面 broker）的 Hello 从未到达 broker（Plan-500/694 代 spawn 机制；本环境无 outproc app 二进制——calc 为 vue/vm 单产品、其余子模块按 AGENTS 刻意未初始化）。已交付协议侧：adopt 零装载激活臂（Incubation.adopt→BrokerClient.adopt→registry 不查、簿记壳组件）+ daemon 采纳预算 2000→15000ms + adopt 结果/能力观测行——push 机制两端点均有单测，端到端留待 outproc 载体存在时闭环。证据：UI `f4b0cc72`+`c4486006`、calc `a1f38842`、probe `probe-t05.py`+日志。AC 进度：AC-01/02/05 实证闭；AC-03 in-proc+standalone 闭、outproc 腿具名遗留；AC-04 失效面随 probe 截图/状态面证据（快照撤路径 P041 契约未回归）；AC-06 待真实事件窗口授权；AC-07 候选组合（worktree apps/auto-calc @pin + clone 候选）待 T-06 冻结核。next: work（T-06 健康门+候选冻结；真实事件窗口授权仍未取得——未授权则 V-07 真实腿保 pending，AC-06 不完整通过将阻断 AC-08 全过，需用户裁决）。

stage: goal | plan_id: PLAN-053 | plan_revision: r1 | round: 2 | from: work | to: work | status: executing | outcome: pass | worktree: D:/autostack/.wt/os-053/auto-os (+auto-ui) | commit: f4b0cc72+c4486006(ui)/a1f38842(calc 候选) | next: work

2026-11-03 / stage:work(回合3) / auto-os/PLAN-053 r1 / outcome:waiting(§10.2 真实事件窗口授权——唯一未决用户输入) / **T-06 主体验收**：①健康门：worktree `check-app-submodules.ps1 -OsRoot`（25 未初始化=worktree 设计态、auto-calc 在位、无批量删除签名）+ `-SelfTest` detector OK；②候选组合冻结：worktree `apps/auto-calc` 本地 fetch+detach 到候选 `a1f38842`（**gitlink 保持 pin e5247c5 未动**——不冒生产升级），组合实际启动 `probe-t06.py`：desktop(catalog) → bus launch 011-calculator（in-proc vm，容器臂命中候选源）→ 采纳者行 pending→applied（§5.3 真态语义：发送≠渲染成功，pending 自续拍到 eval 落定）→ 策略切换 dark→light → `011-calculator=applied` + `Auto@light rev2` 持稳。③V-08 包源负腿：tampered 目录（内容篡改→per-file sha 不符）→ 注入校验拒绝（回退次选或降级）；无有效源（tampered + 无 AUTO_UI_ROOT）→ **具名拒绝** `rejected: 主题包目录不可用（包缺席或校验失败）` + 保旧（theme_rev 3 跨拒绝提交不变）+ 降级诊断行；**无主检出 fallback 移除**（V-08 契约：目录解析收紧为 env 双源，编译期锚删除——用户机上不静默换源主检出）。④adopt 语义补齐（回合2 遗留收尾）：桌面 broker adopt 零装载激活（D1 客户端权威）+ daemon 采纳预算 15s + 采纳者状态行 pending 自续拍。**验收台账**：AC-01/02/05 实证闭；AC-03 in-proc+standalone 闭（outproc 端到端具名遗留，回合2 记录）；AC-04 失效面证据随 probe/组合截图；AC-07 候选组合+健康门+固定来源全闭（remote push 属 AC-09/T-09）；AC-08 待独立 review；**AC-06 阻塞于 §10.2 真实事件窗口授权**（round 1 AskUserQuestion 未获答复；V-07 真实腿缺授权不冒 PASS——AC-06 不完整通过将阻断 AC-08 全过与 merge）。证据：UI `2dca968b`；probe `probe-t06.py`+`t05/t06/t08-*.log`。**停候**：等待用户对 §10.2 的裁决——(a) 授权真实事件窗口 → T-06 补 V-07 真实腿 → T-07 execution_done → review；(b) 明示不授权 → AC-06 以"真实腿 pending+受控腿实证+平台边界"具名交 review 裁定（合同条款本身预期此形态，§10.2 原文），goal 以 blocked 收场交用户。next: user。

stage: goal | plan_id: PLAN-053 | plan_revision: r1 | round: 3 | from: work | to: user | status: executing | outcome: waiting | worktree: D:/autostack/.wt/os-053/auto-os (+auto-ui) | commit: 2dca968b(ui)/a1f38842(calc 候选) | next: user

2026-11-03 / stage:work(回合4) / auto-os/PLAN-053 r1 / outcome:pass(§10.2 裁决=未获授权——V-07 真实腿具名 pending 交 review；T-07 execution_done) / **§10.2 裁决记录**：真实事件窗口授权经 AskUserQuestion 两轮询问（回合1、回合3 停候）均未获用户答复——**授权未成立**，按 §10.2/§5.5 合同原文处置：V-07 真实腿保 pending（真实用户设置一律未触碰），受控腿+独立进程跟随实证保持，AC-06 以不完整形态具名交独立 review 裁定；不冒 PASS、不受控绿代替、不宣称本包在 AC-06 维度完整交付。**T-07 证据冻结**（execution_done 7/10）：工作树冻结 = OS `codex/plan-053-host-theme` @ `2dca968b`（Cargo.lock 组树本地更新未提交——兄弟版本前移所致，不入交付）+ 子模块 apps/auto-calc detach 候选 `a1f38842`（gitlink pin e5247c5 未动——候选组合为已验证工作态，T-09 才正式 pin）；UI `codex/plan-053-host-theme-ui` @ `c4486006`；calc clone `main` @ `a1f38842`（本地未 push）。**AC 台账（work 侧终态）**：AC-01 ✓（V-01 单测+probe 版本化消息/具名拒绝/独立来源；V-08 三腿：tampered-sha 校验拒、无源具名拒+保旧、回退次选按候选序）；AC-02 ✓（V-02 单测+probe：bus/外写同入口、写失败零内存变更、未知字段保留、旧链回归）；AC-03 ✓*（in-proc 组合腿 applied 状态行+切换持稳、standalone 4/4 像素级、outproc 端到端具名遗留——Hello 未达 broker，Plan-500/694 代机制，协议侧两端点单测闭）；AC-04 ✓*（invalidate_all 共享路径+probe 状态/截图失效证据+P041 契约未回归；in-desktop 像素级未单独取证——standalone 像素判定覆盖同一 publish/失效路径）；AC-05 ✓（calc 21/21：原义务全量+三区分用例+T9 结构负例；类目色未动）；**AC-06 ◇具名 pending**（受控 provider 腿=031 交付+system_prefers_dark 消费路径实证；真实腿未授权未测；平台边界=Windows-only provider 如实分列——交 review 裁定）；AC-07 ✓（候选组合+健康门+固定来源+实际启动同 SHA；后继契约=SD 文本+README 反链）；AC-08/09 属 review/merge。**统一回归态势**：受影响单测组 46 项（theme_policy 8/config 20/词表 1/协议 17）+ calc 21/21 + 三 probe 反复复跑全绿；全仓级批量回归未跑（merge 所有权，按 PLAN-888 scoping 规则记录）。工作树零阻塞问题、无未解释 workaround。next: review（T-08 独立 final review，评审须对 AC-06 具名 pending 与 outproc 具名遗留作出裁定）。

stage: goal | plan_id: PLAN-053 | plan_revision: r1 | round: 4 | from: work | to: review | status: execution_done | outcome: pass | worktree: D:/autostack/.wt/os-053/auto-os (+auto-ui) | commit: 2dca968b(os)/c4486006(ui)/a1f38842(calc 候选) | next: review

2026-11-03 / stage:review(回合5) / auto-os/PLAN-053 r1 / outcome:needs_fix(F-1 交付绑定——独立 final review 实录) / 独立 subagent 上下文 final review（limitations：同会话独立上下文、绑定评审时点树态）。**评审员实测**：单测 46/46、calc 21/21（含三区分用例）、probe-t01 8/8、standalone 4/4、probe-t06 4/5、自建 V-08 tamper 腿 PASS、digest 三方复算一致、健康门复跑、核心代码逐文件读（theme_policy/message 17·18/shell_state 采纳者语义/session 零装载/aura eval 记录点/desktop.at/calc 三仓）。**AC 图**：AC-01/02/05/07 pass，AC-03/04 pass*（outproc 与像素两具名遗留），AC-06 partial-named，AC-08 本评审，AC-09 merge。**G-1/G-2/G-3 全部裁定接受**：G-1 = merge-owned hold（merge 收据与 canonical SD-05 必须携带具名 hold：owner=user/merge、closure=获授权真实事件窗口；交付声明不得称 AC-06 完整）；G-2 = spec/DEBT 后继闭环条件（载体存在时测试），非 053 缺陷；G-3 = 充分（applied 仅在 publish 成功后于帧构建记录——t06 C/D 即像素重建证据；E 为探测工效学 F-2 minor 非阻塞）。**F-1（major，交付绑定）**：OS host/desktop 目录收敛变更未提交 + 回合4 收据 OS HEAD 误记 `2dca968b`（实为 UI HEAD；OS 分支当时 HEAD `53f50c9e` 仍含旧内联 loader——恰为 V-08 移除的主检出 fallback）。**修复（work）**：`4b1dee14` 提交两文件（内容即评审员已审的 working-tree 态——全部 probe/单测均对该内容运行），新 HEAD 单测复验 8/8；Cargo.lock（组兄弟漂移）与 apps/auto-calc detach 候选（gitlink pin 未动）为记录在案的工作态非交付脏项。§9 追加式更正：**回合4 收据的 OS SHA 以 `4b1dee14` 为准**（2dca968b = UI HEAD，归属笔误不改写原收据）。next: review（同评审员机械重绑定——评审员原话：repair 后无需重执行）。

stage: goal | plan_id: PLAN-053 | plan_revision: r1 | round: 5 | from: review | to: work | status: executing | outcome: needs_fix | worktree: D:/autostack/.wt/os-053/auto-os | commit: 4b1dee14(os, F-1 修复)/2dca968b(ui)/a1f38842(calc 候选) | next: review

stage: goal | plan_id: PLAN-053 | plan_revision: r1 | round: 5 | from: review | to: work | status: executing | outcome: needs_fix(repaired in-round) | worktree: D:/autostack/.wt/os-053/auto-os | commit: 4b1dee14(os F-1 修复) | next: review

2026-11-03 / stage:review(回合6) / auto-os/PLAN-053 r1 / outcome:pass(独立 final review 重绑定后终裁) / F-1 重绑定全核验：`4b1dee14` 在分支（父 `53f50c9e`、基 `65fc70de`）、diff 恰为两份已审文件（host/desktop Cargo.toml −2 依赖移除 + main.rs 79 行收敛 hunk）、零残留 fallback 命中、worktree 干净（除记录在案的 Cargo.lock/detach 两非交付项）、theme_policy 8/8 评审员本人复跑、收据更正可接受。终裁 AC 图：AC-01/02/05/07 pass；AC-03/04 pass（具名遗留 G-2/G-3）；AC-06 partial-named（merge-owned hold G-1，约束 merge 收据与 SD-05）；AC-08 本评审闭环。**status: reviewed → merge（T-09/T-10）**，merge 义务：SD-05 携带 G-1 hold、spec/DEBT 记录 G-2 闭环条件、calc push、gitlink 正式 pin、归档清树。

stage: goal | plan_id: PLAN-053 | plan_revision: r1 | round: 6 | from: review | to: merge | status: reviewed | outcome: pass | worktree: D:/autostack/.wt/os-053/auto-os | commit: 4b1dee14(os)/2dca968b(ui)/a1f38842(calc 候选) | next: merge

2026-11-03 / stage:merge(回合7) / auto-os/PLAN-053 r1 / outcome:blocked(partial——os 全链落地+pinned+pushed；ui landing 阻于并发会话 WIP) / **canonical specs 落地**：SD-01/02/03(os)/05 → os `ab30847`（新页 host-theme-policy-and-delivery.md 含 G-1 hold 与 G-2 闭环条件 + P041 family 链条款 + apps 登记页；doc-only descendant of reviewed 4b1dee14 = delivery commit 前身）；SD-04 → ui `e7af85b7`；SD-03 → calc `3159a0f6`。**landing（rebase→range-diff 全等→ff-only）**：os 分支 rebase 到 main 新 tip（他计划 `3f777db` docs-only，零文件交集）——映射 53f50c9→`5928a11`、4b1dee14→`df286c3`、ab30847→`31e0a939`，range-diff `=` 全等；**os main = `31e0a93` ff-only**（无 merge commit）。**calc push**：`e5247c5..3159a0f main→origin/main`（github，正常 push 非 force）。**gitlink 正式 pin**：os main 子模块 fetch origin → detach `3159a0f`（内容核：pac/vendor/specs 在场）→ **`123364d`** 提交。**os push**：origin `3f777db..123364d main→main` ✓。**canonical_verified（os 侧）**：landed main 含 specs/shell/host-theme-policy-and-delivery.md、showdesk-ux-polish 修订、apps/calculator-theme-adoption.md；SD-05 携带 G-1 hold（owner user/merge、closure 获授权窗口）、G-2 闭环条件在页——review 条件满足。**BLOCKED（ui landing）**：auto-ui main checkout 有他会话（ui-029/相关）未提交 WIP，含 `crates/auto-ui-runtime/src/dynamic_build.rs`——与本分支（arm_standalone_follow 接线）文件交集，ff-only 被 git 拒（§4.1 禁动他方 WIP）。**精确解锁动作**：该 WIP 属主提交/清理 auto-ui main checkout 后，执行 `git -C D:/autostack/auto-ui merge --ff-only codex/plan-053-host-theme-ui`（分支已 rebase 到 master fa01bd79 之上：映射 ad883db4→`fb899a57`、f4b0cc72→`cdd5fe3c`、c4486006→`3a0d67a2`、2dca968b→`af179789`，range-diff 全等，tip `6f4bcfae` 含 SD-04）→ 之后归档+清树（worktrees os-053 组五树 + ui 树 wt-guard 后移除 + plan-artifacts 删除 + registry）。**stale-artifact 观察**（landing≠deployment）：用户桌面二进制以下次 desktop.sh 构建为准生效；运行中的桌面实例为旧代码。calc clone tmp 已清。checkpoints 保全：os landed/pinned/pushed、calc pushed、ui branch ready、worktrees 在位。

stage: merge | plan_id: PLAN-053 | plan_revision: r1 | round: 7 | outcome: blocked(partial) | delivery(os main): 31e0a93+pin 123364d (pushed) | calc: 3159a0f (pushed) | ui: 6f4bcfae ready-unlanded | next: user/unblock

2026-11-03 / stage:merge(回合8) / auto-os/PLAN-053 r1 / outcome:pass(ui landing 完成——用户授权 stash 协议) / **§10.2/UI-landing 处置记录**：ui landing 阻塞经 AskUserQuestion 获用户明确授权「stash 协议」。执行实录：①WIP 快照（status 全列表 + dynamic_build diff sha `6cc0491c40dba366`、70 行）；②带标签 stash -u（"PLAN-053 ui landing transit (user-authorized 2026-11-03)"）；③**ff-only landing：auto-ui master `fa01bd79` → `6f4bcfae`**（SD-04 + theme_policy.rs + 协议帧 17/18 + 采纳者语义 + 零装载采纳全量落地）；④立即 stash pop——零冲突；⑤共存校验：`arm_standalone_follow` 在位（count=1）+ 他方 hunks 逐行同集（排序比对空差）；diff sha 差异（6cc0…→0cb9…）纯系基线漂移（新 HEAD 含我方插入），他方 hunks 内容零损。他会话 WIP 完整恢复于新 master 之上。**交付链闭合**：os main = `31e0a93` + pin `123364d`（pushed origin 3f777db..123364d）；auto-ui master = `6f4bcfae`（本地 landing；push 属 auto-ui 仓节奏——master 推送由宿主仓常规流程处理，本收据记录 landed 状态）；calc origin/main = `3159a0f`。**stale-artifact 观察（landing≠deployment）**：用户桌面二进制以下次 `scripts/desktop.sh` 构建为准生效；运行中桌面实例（如有）为旧代码；组树内构建产物随 worktree 清理消亡。**G-1 hold 终态**：AC-06 真实事件腿 pending——owner user/merge、closure 获授权真实事件窗口；已写入 SD-05（canonical os main）与本归档页；交付声明不含 AC-06 完整。**G-2 终态**：闭环条件已入 SD-01 outproc 节 + KNOWN-DEBT-AND-RISKS 条目。archive + cleanup 见后续收据。

stage: merge | plan_id: PLAN-053 | plan_revision: r1 | round: 8 | outcome: pass(ui landed; user-authorized stash protocol, WIP restored verified) | delivery: os main 31e0a93+123364d (pushed) / auto-ui master 6f4bcfae (landed) / calc origin 3159a0f (pushed) | next: archive
