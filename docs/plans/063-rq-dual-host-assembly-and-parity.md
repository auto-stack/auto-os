---
plan_id: PLAN-063
status: executing
feature_name: RQ独立Host与虚拟桌面Host完整装配验收
author: [Codex]
created_at: 2026-11-05
updated_at: 2026-11-08
plan_revision: 4
current_step: 3
total_steps: 7
depends_on: [auto-ui/PLAN-061, auto-ui/PLAN-062, auto-ui/PLAN-063, auto-os/PLAN-054, auto-ui/PLAN-073]
supersedes_spec_components: []
new_spec_components: [docs/specs/shell/rqhost-assembly-and-fidelity.md]
touched_goals: []
affects: [renderqueue, rqhost, rendering-fidelity, resource-lifecycle, rq-performance]
---

# PLAN-063：RQ独立Host与虚拟桌面Host完整装配验收

## 0. 变更摘要

承接2026-11-05用户提出的RQ完整功能要求。设计见[完整设计](../design/04-rqhost-platform-and-compositor.md)，[战略/跨仓边界](../../../auto-ui/docs/design/autoui/renderqueue-refactoring-roadmap.md)。本Plan仅drafting，尚未实施。交付限定为本合同结果，不以单个阶段完成冒整条RQ路线全部完成。

## 1. 目标

OS主导Host装配/窗口策略和整体验收；UI通用功能由UI062/063提供，Man发行发现走其正式接口。只读产品源码/独立clone，运行用隔离数据；不自动授权产品业务修改、系统主题全局改动、全量gitlink升级或自有Linux compositor。

工作结果以§7可观察AC为准；同源direct是兼容基准，禁止删除能力/用隐藏自窗或整帧自绘降级来达标。原生窗口标题栏与App内容区分别验，不要求Host系统装饰像素相同。

## 2. 架构方案

复用UI精确Replay和平台服务，不另建OS绘图原语解释器；OS负责虚拟桌面策略和产品组合。全能力以UI061清单为准，产品清单从apps.manifest/真实来源生成，含外部/嵌套/OS-local。每个产品核VM/Rust可构建状态和两RQ出口；原direct/Rust缺陷须具名交owner并保持本任务未完，不能据不支持免验。规模大时可按已冻结清单批次执行，最终分母不可缩减。所需产品新增能力超范围时提出具体后继合同；不自行写产品业务或假归档。

实现→独立review→merge依次使用同一权威实施树。拟用`D:/autostack/.wt/os-063/auto-os`、分支`codex/plan-063-dev`；本轮不创建。需要写入的协作仓在同组各一个树；产品只读/隔离数据，新增业务写面须另定具体合同。UI/Man/OS原owner接口保留，语言基础无UI反依赖。

## 3. 技术栈

Rust、现有iced/wgpu、desktop_protocol二进制消息/共享资源；当前Windows本地双进程与两种Host。测试沿仓内现有Cargo入口及真实公开CLI。Linux transport/共享映射归OS062；标准Wayland compositor和ACE/AGE仅在设计中留接缝，不在本Plan实现。

## 4. 需求分析与背景调查

### 授权

2026-11-08 r4：用户授权从063起更新RQ后续计划，并允许M1前新增修复计划；仅设计/合同修订，暂不实施。062已归档r5/7-7（cae1632e/a1cdfaa7），UI060 r8/6-6、UI061 r3/6-6、OS054 r3/10-10已交付。旧源码观察/“在途、Spec缺席”段落仅为r3历史，当前行为以两份RQ canonical及实际源码为准。

2026-11-08 r4/goal：用户 `/goal 完成计划063` = 全链授权（本goal会话经 /auto-plan-goal：在
专属worktree实施、独立review、有界修复、merge归档清理）。r4 receipt的blocked（执行授权
未到）由此解除；UI063已正式交付（archived r4 7/7，交付96236725，UI master 4ebc51ce），
T-02/03进入门满足；UI073仍executing 0/8，T-04/05进入门保持。

2026-11-05 r3：用户要求复核四份RQ计划范围/依赖、与UI060/OS062去重并明确启动顺序；仅修订草案，不实施。

2026-11-05 r2：用户明确授权将性能分析/建议补入设计、战略和相关Plan，仍暂不实施。本次仅修订合同，不新增计划或接管其他会话。

用户明确要求检查设计/实现、建立设计/战略、纳入拆分重构路线并考虑新计划；本轮获准调研、文档和立约，未获本轮实施授权。不自动开agent/聊天，不接管UI053/060/OS054/061/062在途合同，不提交他方WIP。无额外预算/时限授权。

### 源码事实

host/desktop为薄入口，主要Host实现仍在UI的rqhost/session/broker_surface；OS054双轨八产品、UI060运行修复尚在途。源码依赖与虚拟桌面输入/画面必须消费正式版本，不可拿独立窗或inproc VM替代VM RQ。OS062仅Linux两Rust应用准入，本计划当前Windows完整双Host，Linux扩面另签。

基面UI `426c1f708e7c966f243c86b1f38029ae47b0fa0a`，OS `7968a68bfd15853a907f8b21f4dc106a2c216d39`；文件SHA256与逐项来源在主设计§2。UI Specs从`docs/specs/README.md`进入，相关`ui/structure-closure.md`、`runtime/component-and-store-instances.md`、`runtime/native-vm-app-validation.md`；OS从`docs/specs/shell/desktop-app-launch.md`与`desktop-bootstrap.md`进入。完整保真Spec当前缺席，本Plan提出新增，不提前改canonical。headless/rqhost/client_entry/message/endpoint/shm/transport及iced/broker_surface是现有路径；RenderPacket/ReplayCore与新测试路径是拟议，不冒已实现接口。

立项时取号扫描主检出、archive/archived及全部注册worktree；当时UI060仅树内亦计入占号。r3时UI060合同已恢复主检出、仍executing；UI053已有正式交付与清理收据。锁内取号、不覆盖已有Plan；UI无new-plan.sh使用独占分配，OS使用仓内脚本。实现时重新冻结真实源与依赖，不能依此快照借WIP。

## 5. 详细设计

完整不变量和跨阶段约束沿主设计§3–8；本Plan工作集中于§8唯一任务表。T-01建立`D:/autostack/plan-artifacts/plan-063/auto-os/`并登记owner、用途、在途状态；探针/截图/日志/外部构建只在统一scope，不落docs/plans/evidence或attachments。正常编译target在专属树；外置target也只在scope并登记可删。

未决接缝由对应前置任务做有界探针并形成可审阅决定，不能一边猜一边切生产；源与已交付依赖不变时复用有效证据。三Host控制面可扩展但没有未来支持假承诺。能力丢失/非法帧必须具名失败；资源上限须T-01按真实测试环境和UI061决策冻结单位、边界、失败表现，不新设语法/控件限制。

### 范围与启动门复核（r3）

- **可先准备**：T-01只读枚举manifest、产品来源、Demo与六格缺口；UI061正式能力/预算未到只能是草表，不能勾完。T-02～05等UI062/063正式交付及OS054正式组合；UI060经UI063传递，不重复设修复任务。
- **补实际写面**：OS host/desktop是薄入口；桌面逻辑部分仍在UI的crates/auto-ui-runtime/src/ui/session.rs和ui/iced/renderer/shell_state.rs。T-03允许同组唯一UI协作树修改所需桌面策略配置/接线（placement/z/clip/可见性/focus与通用adapter连接）。通用输入/服务/调度属UI063、绘图资源属UI062，不再造机制；缺通用能力则具名差额/修订或后继修复合同，不静默扩成第二个UI063。双仓review冻结同版，先落地UI接线后固定OS组合。
- **产品边界**：OS054八产品原门保持；OS063消费其正式组合扩完整RQ能力、两Host和manifest全分母，同身份有效证据可复用，不借WIP。T-01给每产品VM/Rust/direct/RQ缺口标owner、真实失败及关闭合同；业务新增/语言重大能力不无限吸收，必需格未闭仍阻塞完整出口。28为当前观察，不是永久分母。
- **发行/Linux边界**：本Plan验Windows App/Host/字体资源/后端运行闭包，消费UI063/Man发现接口；缺闭包明确失败。OS062保Linux两Rust应用及Nix安装，不追加28产品/六格要求；Linux全能力扩面、Wayland compositor、ACE/AGE仍另有进入门，不阻塞当前Windows交付。

### r4：062正式交付后的承接与M1边界

本Plan保持drafting0/7。本轮不重开062，不以归档通过外推全部能力。已读runtime/rq-fidelity-contract.md及runtime/rq-render-packets-and-resources.md；规划来源：rq-fidelity-contract.md SHA256=57f9d191c3718979716e286c103b901a60496224360ab50250675a1cf868d2ff；rq-render-packets-and-resources.md SHA256=bc496c0109716f986098089f45ebb2ad4000f81cf33ae1dbe875a4bc1c3108fe。这些是设计来源，不是本轮运行结果。

**分阶段进入门**：T-01正式清单冻结可先行（UI061已交）；T-02/03装配/策略等UI063正式交付，UI062/OS054已满足。T-04全能力/全产品最终运行及T-05闭合额外等UI073正式交付并重建组合；只读盘点与准备不因frontmatter新增073整项停摆。UI073不依赖本Plan全产品结果，无环。

| 062交接项 | 本Plan责任与关闭要求 | 原任务/验收 |
|---|---|---|
| M-02/M-06/M-07 | UI063实现事件/deadline/present、冷热启动/恢复通路及公平调度，最小真实两Host验证；OS063测最终组合/1、4、8 App及完整成本。061/062代理与未测项不能抵实际present/响应/进展证据 | T-01/04/05；AC-07；V-07/08/10 |
| 四既有红 | autocenter_translates_hits_with_ops、stage3_multi_app_stress_n3/n5、t3_examples_queue_end_to_end逐项列当前来源/输入/断言/基线对照/候选适用/owner/关闭证据；命中本路线必需能力不能基线红豁免 | T-01/05；AC-01/04；V-11 |
| 连续媒体/绘制残差/重排负控 | 生产修复及最小六格由UI073负责，UI063提供真实接口；OS063做最终产品消费，不另造第二套replay。静态poster、文字重排、2%阈值不能关闭完整媒体/精确画面义务 | T-01挂接、T-04/05正式消费；AC-01/02/04；V-02/09/12；[UI073](../../../auto-ui/docs/plans/073-rq-m1-media-fidelity-and-resource-closure.md) |
| r5计时与真正隔离 | 062 replay p50设门/p95只记录是局部门，不等最终端到端尾延迟、deadline通过。T-01先冻结真实负载/样本/时序终点/硬预算与污染规则，禁止测后宽限；isolated只是标签，需实际CPU/GPU/外来进程负载记录，不具测量条件保pending | T-01/04/05；AC-07；V-10 |
| 旧制品与删除证据 | 重建本候选App/Host/工具/SDK，绑source/dep/config/hash/实际PID，无旧exe或开发根fallback。归档摘要足以证明的证据可复用，仅缺失或变更的case重验，删除原始scope不自动全跑/假绿 | T-01/02/05；AC-01/04；V-12 |

默认客户区exact及结构/业务oracle目标保持。062 r5的逐族2%/非exact与p95只记录不得自动变成本Plan最终通过口径；本Plan相关绘制尚未修复则具名标“等待UI073”，不声称M1完整图像已过。最终窄容差只允许原合同所述真正平台采样非确定性并有区域/理由/敏感负控；实质支持承诺或预算改变须另行裁定/修约。

生产修复不混进只读归因：四红中输入/调度根因归UI063，绘图/资源根因归UI073，产品业务根因交实际owner具名合同；无正式修复合同/证据的必需缺口阻断本Plan相关AC与M1。不能仅按名字判断，更不要求无关历史红全部清零。

### 性能合同（r2继承）

遵循[主设计§7.1～7.3](../../../auto-ui/docs/design/autoui/renderqueue-complete-fidelity.md)的P-01异步批量、P-02资源复用、P-03呈现调度、P-04多App隔离和M-01～08指标。本Plan的关闭条件是AC-07、V-05～08与§8任务，设计链接不代替实际证据。

- RQ不是逐绘图RPC：steady-state不逐图元等待reply，也不强制每帧等待上一帧呈现后才准备；容量不足须背压，传输ACK与GPU资源寿命分开。
- 驻留且未变font/image/mesh资源重复帧不重复上传payload/创建GPU资源；合法失效、驱逐、重建单列，不重整形App已定位的文字。片段复用先实证，不默认重做全局增量场景。
- 及时响应输入/资源完成并按deadline/present调度，固定15ms轮询不能是唯一驱动；在途帧/字节有上限，可合并过期画面但不能丢输入、资源依赖或业务副作用。
- per-App上传/资源/CPU工作预算与公平调度必须有轻重混合负控；长任务分批/限流，不能假设任意GPU命令可抢占。Host/device loss仍是共享故障域，不冒完全隔离。
- 数值预算由UI061先量原direct基线、在候选比较前冻结：逐workload列单位/统计量/绝对与相对上限/样本/owner/失败处理。本轮无数值或测量结论；后继缺预算或必要测量不能签性能PASS，超标不得测后改阈值。

### 任务落点

T-01读取`apps.manifest`与正式产品源；T-02/03修改`host/desktop/src/{main,health}.rs`、`scripts/desktop.{ps1,sh}`的实际装配差额，UI的ui/session.rs与ui/iced/renderer/shell_state.rs桌面策略接线按§5纳入同组UI协作树；通用渲染/输入回UI正式owner，不能复制到OS。T-04/05新增`scripts/rq-parity/`，复用UI061 oracle和真实产品原门。

### 规范增量

| delta_id | add/modify/retire | target | before/after rule | rationale | acceptance IDs |
|---|---|---|---|---|---|
| SD-01 | add | docs/specs/shell/rqhost-assembly-and-fidelity.md | 当前有限分散实现/历史设计 → 本Plan已验范围、062r5承接、073交接、旧红适用/关闭、重建与真实计时条件及P/M性能要求 | 不再以历史“无漂移”或编译绿替代运行证明 | AC-01..04、AC-07 |

如实际改动还影响已有canonical条目，T-01具名补充并按语义变化修订Plan；不制造无关Spec条目，review核真实增量。

## 6. 测试设计

以下均为拟执行，不是本轮通过证据。主矩阵VM/Rust×direct/standalone-RQ/desktop-RQ；同source/主题/font/viewport/scale/时钟/事件序列。所有V行owner_stage=work。结构/glyph/资源、客户区像素和业务读回同时核验；旧direct金样不因共用新代码而失效。默认exact像素比较，真正平台采样非确定性只有具名区域/原因/敏感反控经审查后可设窄容差；不允许全局阈值吞漏图。

| Case | AC | 输入/真实入口与负控 | 可观察oracle/关闭条件 | 责任任务/边界 |
|---|---|---|---|---|
| V-01 | AC-01 | manifest全部active ID、UI所有Demo/能力ID、正式pin/工具/资源；反控遗漏外部Jade Garden或syslog | 双向核无遗漏，源码/运行版本同一，六格和平台状态分列；清单缺项驱动失败 | T-01/04；真实全分母 |
| V-02 | AC-02 | 同VM输入或Rust产物的direct/rq/desktop，包含实际编辑/媒体/图形能力；缺工具/错Host负控 | 客户区同源视觉/交互一致；rq App无GPU/隐藏窗；desktop内容真实在虚拟桌面，缺件不启动陈旧产物 | T-02/04；六格与发布身份 |
| V-03 | AC-03 | 桌面移动/缩放/遮挡/最小化、capture越界、IME、多个同名store App、真实产品写后重开 | 画面clip/z序与焦点一致，事件命中正确，实际API/持久数据正确且不串App；按完整能力表闭合 | T-03/04；窗口策略/产品任务 |
| V-04 | AC-04 | 并发孵化、pending时关末窗、关闭其一、HostLost、后端失败、隔离运行目录的打包启动 | 无多余Host/资源泄漏，旁观App不受损，错误明确可恢复；闭包不依赖源码/旧缓存，缺资源明确失败 | T-05；生命周期与部署 |
| V-05 | AC-07 | 公开入口真实App、两Host、延迟ACK/慢Host；错版本或模式反控 | 实际装配保持异步批量/有界槽信用，无隐藏direct降级；M-01/M-04链路可追 | T-02/T-04/T-05；work；本Plan职责，机制相同且身份未变证据可复用 |
| V-06 | AC-07 | 真实产品重复打开资源、稳定帧/单项更新、Host冷/热和session重建 | M-04/M-05/M-06区分缓存驻留与重传；统计App+Host全部成本且共享去重，不隐藏Host开销 | T-02/T-04；work；本Plan职责，机制相同且身份未变证据可复用 |
| V-07 | AC-07 | 同源direct与两RQ Host六格，空闲/滚动/IME/动画/媒体与声明刷新率 | M-01～06/M-08按冻结定义/预算报告，提交与呈现终点分开，分布/样本/丢帧齐全、保原功能 | T-03/T-04；work；本Plan职责，机制相同且身份未变证据可复用 |
| V-08 | AC-07 | 1/4/8冻结规模的轻重混合负载、重App过载/断连与两Host | M-07公平性及资源上限通过，轻App/重App均有进展，旁观App不受损；未测规模不得宣称支持 | T-03/T-05；work；本Plan职责，机制相同且身份未变证据可复用 |
| V-09 | AC-02、AC-03、AC-04 | UI063 adapter与正式OS054；错工具/漏资源/焦点路由/可见性反控 | 实际OS+UI接线与正式版本一致，缺闭包失败；双仓落地顺序可核，不挪用Linux/Nix结果 | T-02/T-03/T-05；组合兼容 |


| V-10 | AC-07 | 两Host真实输入→App→提交→可观测present、冷/热启动/恢复，先冻结预算；忙机/缺present仅代理/仅isolated标签的负控 | 终点/样本/机器负载有实证，尾延迟/deadline/冷热恢复按实际职责闭合；环境污染、p95仅打印或缺测不PASS | T-01/04/05；通用机制归UI063，最终规模归OS063 |
| V-11 | AC-01、AC-04 | 四旧红逐case原输入与断言、原基线与当前候选；遗漏case/仅写“known red”反控 | 当前影响/owner/正式关闭证据可核，命中必需面仍失败则不能通过相关AC；范围外有真实理由，不按名称一律重做或免责 | T-01/05；不扩全历史债 |
| V-12 | AC-01、AC-02、AC-04 | 冻结producer/Host/工具/SDK重建后真实启动；故意同名旧exe/缺资源；摘要复用与UI073交接表 | 运行hash/来源/配置/PID吻合；旧制品失败，删除附件不自动丢有效摘要，未交媒体/图像能力明确阻断其最终完整验收 | T-01/02/05；OS063最终T-04另核 |

性能实测口径：M-01响应延迟p50/p95/p99/max及分段；M-02帧间隔/目标deadline失约、合并/丢弃/画面年龄；M-03 App/Host/总CPU与可测GPU时间、空闲唤醒；M-04命令/资源字节、消息/往返、复制与上传/创建计数；M-05每进程/总内存与共享去重、GPU驻留和队列峰值；M-06冷/热启动及恢复；M-07轻重App公平；M-08保真/业务守恒。各职责分别取证，OS063做最终组合，未执行不得被上游局部测试抵扣。

同执行器原direct、候选direct及两RQ Host同机器/source/build profile/GPU/字体/DPI/分辨率/主题/事件/资源/刷新率比较；冷/热分列。完整多App成本为N个App加Host，direct为N进程；桌面壳基准单列且不得隐藏其成本。event/frame ID配对、单调时钟校准偏差；无法量物理上屏只标实际present代理终点。UI061先冻结样本长度/重复/运行顺序/污染规则，成对或交错运行；保失败/缺测/异常与分布，不择优挑样本，p99样本不足不给通过结论。性能测量仅在获准实施后的独立窗口，不与其他会话争用GPU。

测量入口由UI061 T-04的`tests/rq_fidelity/`原型补计时/计数并交付准确命令，后继T-01读回；OS063由`scripts/rq-parity/`编排真实组合。两路径仍是拟新增，本轮不创建脚本或运行基准。不得新增一个与现有oracle无关的泛化跑分系统。

### 验证入口

OS实施树：`cargo check --locked -p os-desktop --bin ui-desktop`及实际现有`cargo test --locked -p os-desktop`适用目标；`pwsh -File scripts/check-app-submodules.ps1`核正式组合。`scripts/smoke-031-rqhost.sh`、`scripts/smoke-034-rqhost.sh`只作历史入口参考，T-01核是否仍可用，不以旧样例抵全量。T-02新增`scripts/rq-parity/`编排（新路径），消费UI061真实驱动；产品沿各自原owner门，语言零改不跑语言cargo t。

T-01确认真实test namespace/feature/平台条件；新增入口由标明任务提供。列用例/执行/条件排除，准备与运行时间分列；非零/超时/skip不充PASS。按实际触面完整职责族和必要消费者运行，最终review后merge只跑冻结并集；不叠跑全仓或AG里程碑。真实窗口验证不抢占其他会话，不能通过提高资源阈值隐藏功能缺口。

### 角色与证据

work负责实现/自检；独立上下文review只检查冻结同一树，不另建reviewer树；merge落地与清理。未指定模型/代理，不自动创建。每条证据绑Plan revision、代码/依赖提交、实际profile/进程/资源/产物，失败/修复后结果分列。用户未授权实施前不进入上述运行。

## 7. 验收标准

| ID | owner_stage | 可观察行为/判据 |
|---|---|---|
| AC-01 | work | 全部manifest产品、全部Demo与UI能力矩阵逐项可追踪，未支持/未执行不冒PASS或缩分母 ；按r4承接表及V-10/11/12中关联本AC的义务逐项核验，无隐式豁免|
| AC-02 | work | 六格在真实Host呈现且内容/行为一致，App无自绘；发布/模式/依赖身份可证明 |
| AC-03 | work | 桌面窗口策略、坐标/输入/IME/焦点和产品真实任务正确，独立Host仍保持原生窗合同 |
| AC-04 | work | 共享Host生命周期/故障与干净闭包启动通过，恢复与资源终态可核，无旁观进程损伤 ；按r4承接表及V-10/11/12中关联本AC的义务逐项核验，无隐式豁免|
| AC-05 | review | 独立上下文逐项验证当前修订AC/V、真实源和规范增量，所有必需项有可复核证据；不能只读完成勾选 |
| AC-06 | merge | 通过后按依赖顺序合入、canonical读回、路线回填，归档/guard/材料清理与恢复摘要完整；交付边界如实记录 |
| AC-07 | work | 独立与桌面Host实证P-01～04及六格/多App M-01～08预算，App+Host总成本与原direct/候选direct同口径；轻App响应与重App进展均成立，缺失测量/超预算不计PASS，不外推X11/Linux或全产品性能 ；按r4承接表及V-10/11/12中关联本AC的义务逐项核验，无隐式豁免|

## 8. 执行步骤

| 完成 | ID | owner_stage | 依赖/进入门 | 文件/操作与结果 | 关闭方式/AC |
|---|---|---|---|---|---|
| [x] | T-01 | work | 可先只读枚举；冻结等UI061；T-02起等UI062/063、OS054正式结果 | 冻结apps.manifest所有active ID（观察28）、全部教学Demo与能力对应表、真实产品源/工具/资源版本；列缺产物/旧红，禁止缩分母；冻结同环境direct与两Host性能工作负载、可用刷新率、1/4/8实际规模和产品映射；消费UI061/062现合同，单列代理/未测及r5局部门；冻结真实最终预算，不改变功能分母 ；按r4承接表冻结四旧红/计时真实环境/重建身份与UI073分工| V-01；AC-01；AC-07/V-05～08（按§6分工） ；V-10/11/12|
| [x] | T-02 | work | T-01完成及UI062/063、OS054正式交付 | 在host/desktop、scripts/desktop与UI通用adapter正式接口中闭合独立/桌面Host装配、入口和资源发现；不复制ReplayCore或改产品业务；装配不额外引入同步RPC/重复资源上传；消费UI062有界队列与持久资源正式合同 | V-02；AC-02；AC-07/V-05～08（按§6分工）；V-09 |
| [x] | T-03 | work | T-02 | 消费UI063 adapter；在OS及§5限定UI策略写面补齐placement/z序/clip/resize/DPI/focus/capture及所有权接线（V-09），独立窗口profile保持；按实际能力接系统服务；核虚拟桌面焦点/遮挡/可见性和present节奏与UI063调度接口，避免重窗口垄断；隐藏/恢复策略须保业务时钟语义 | V-03；AC-03；AC-07/V-05～08（按§6分工）；V-09 |
| [ ] | T-04 | work | T-03 ；UI073正式交付且同版消费产物已重建| 运行全能力六格及Demo全部需求；按manifest逐产品跑既有主要任务和运行形态；不足归具名owner且维持待办；按同源同画质测M-01～08；direct N进程对比RQ N App+Host总成本，桌面壳成本与共享内存分列，产品样本不冒全产品性能结论 ；逐073关闭连续媒体/RemoteFrame、文字/阴影/残差，不能静态帧抵连续播放| V-01/02/03；AC-01..03；AC-07/V-05～08（按§6分工） ；V-10/12|
| [ ] | T-05 | work | T-04 | 多App、pending采纳/末窗、HostLost/取消与资源回收；验证固定发布闭包无开发根依赖，整理Spec/完整交付矩阵；两Host真实混合负载、慢Host/ACK、重App过载/断连及轻App输入；核M-07预算与资源终态、发布态性能差额 ；闭合r4适用旧红/资源终态/制品与计时交接，缺必需项不交review为PASS| V-04；AC-04；AC-07/V-05～08（按§6分工）；V-09 ；V-10/11/12|
| [ ] | T-06 | review | T-05 | 独立上下文核实际OS/UI双仓接线、进程/画面/产品数据/源身份、未执行和全部分母，冻结组合；独立核P/M口径、冻结预算、性能负控和完整分布，不能用平均FPS或实现者摘要替代 | AC-05；复核AC-07 |
| [ ] | T-07 | merge | T-06 pass | 先落地实际UI策略接线，再固定OS正式组合、健康门、canonical与路线回填，按产品工作流仅必要已交付pin，归档guard清理；沉淀性能支持边界/预算与实测差额，不外推未测平台 | AC-06 |

work项完成才进入execution_done；review/merge项仍待办，不能为交接伪勾。每个任务读取§4对应实际路径及主设计证据；新增接口先由有界决策任务确认。全部原始材料merge折摘要进本Plan§9后删自有scope；移除树先wt-guard clean，不穿透链接、不清未知owner目录。

## 9. 复审记录

2026-11-05 / stage:new / plan_id:auto-os/PLAN-063 / revision:r1 / outcome:pass（草案合同，不是实现验收） / next:待用户启动后work/T-01，按§8正式依赖核门。

草案静态预检：0–10章节齐全，任务数与total_steps一致，AC/V/SD映射及新文档本地链接有效；主检出/两类归档/注册worktree中的本号唯一。已做源码/设计/相关Spec与重叠计划检查；本轮未实现、构建、跑运行门或独立final review。当前task/AC/V/SD为执行合同；实际通过证据后续填写。设计前置的不确定性保留为任务/阻塞，不能声明已解。

### 2026-11-05 r2性能合同修订

- stage:new / plan_id:auto-os/PLAN-063 / revision:r2 / outcome:pass（文档修订，非性能验收） / next:待用户启动后按§8依赖进入work。
- 变更：新增AC-07、V-05～08；§5增加P-01～04，§6补M-01～08及direct配对/样本/冻结预算口径；§8原任务增补性能职责，SD-01同步扩展。总任务数不变，current_step仍0，status仍drafting；r1记录保留为历史。
- 用户授权仅文档，未实现、编译、运行或测量；没有性能PASS或节省承诺。r2静态检查只核章节/引用/任务与AC/V映射，不是独立final review。

### 2026-11-05 r3范围与依赖复核

- stage:new / plan_id:auto-os/PLAN-063 / revision:r3 / outcome:合同修订，静态核查 / next:仍drafting，按§5/§8依赖等待用户启动。
- 修订发现：拆开绘图/运行/产品六格以消除反向依赖；UI060独占索引/输入/刷新/API修复，OS062保Linux通道/平台/安装；补接口交付/兼容门及OS063实际UI策略写面。P/M性能要求及全部能力/产品分母不减少。
- 来源：UI060主检出executing r1/2-6、扩围及续接记录；UI053归档6/6及UI 9a94d568 / lang 61207576b正式交付（当前主线祖先）；OS062 drafting r1/0-13的T-02～05/09～11；headless、UI session/shell_state与OS薄Host入口。启动时重新冻结源版本/单写符号，不将快照永久当准入。
- 保留r1/r2历史；步数、AC数量和current_step不变，补V-09接口负控。未改UI060/OS062/OS054原合同，未建树/实现/构建/运行或签final review；运行证据待后续work。

- r3静态检查：四份合同0～10章节、27项任务（6/7/7/7）、各7条AC/9条V及引用一致；正式depends_on图无环；RQ与本轮现行路线共155条本地链接有效；三仓本轮文档范围diff --check退出0。只证明合同结构/链接，不证明任何运行、性能或Linux支持。


### 2026-11-08 r4后续承接修订（仅规划）

- stage:new；revision:4；outcome:blocked（执行授权未到；UI063/UI073正式结果待交付）；next:按§5分阶段门进入T-01，当前drafting0/7。
- 保留7任务/7 AC及历史；新增V-10/11/12，增强T-01/04/05及AC-01/04/07、SD-01。062 r5局部门/具名遗留不自动降低M1目标；UI073独占剩余绘制/连续媒体/资源反例，OS063独占最终产品组合性能，UI063负责通用运行机制。
- 本轮未实施/运行/建树/改canonical、未独立审查通过；既有r1～r3审查仅历史，不转作r4 PASS。
- 本轮规划静态自检：0～10章、T/AC/V/SD=7/7/12/1，所有任务未勾选、drafting0步；本轮四合同依赖图无环（解析80个真实Plan），本轮当前文档共82条本地链接及表格/围栏有效。语义合同SHA256=`1b6c31a45798f3b90ad768326abe6c755c67a21da8090716667cab3d6ab12909`（UTF-8/LF，排除§9）。仅合同自检，非独立设计复审或运行PASS；本轮未实施。

### 2026-11-08 work 进展（goal round 1：T-01 冻结闭合）

- stage:work / plan_id:auto-os/PLAN-063 / plan_revision:r4 / outcome:pass（T-01） / worktree:D:/autostack/.wt/os-063/auto-os / branch:codex/plan-063-dev / base_commit:07646dd / dependency_revisions:auto-ui 4ebc51ce（master，UI063交付96236725）、auto-lang 1f05effab、auto-web 2d698413b / task_ids:T-01 / evidence:plan-artifacts/plan-063/auto-os/t01-freeze.md（十节冻结清单） / blockers:UI073 executing 0/8 阻塞 T-04/05（不阻T-02/03） / next_responsible_role:work（T-02）
- T-01 冻结内容：manifest active=28 全列（24 gitlink pin + 3 兄弟仓 + 反控 jade-garden/syslog 在列）；Demo 36 目录；9 项 ALL_FIDELITY_CAPS；六格门（2/255、producer 基线、39 View 26≤0.1%/13族具名/2% glitch cap）；性能预算全套（W-A/B/C p95、硬零上传、256MiB/2帧/1MiB/8MiB·s、replay p50 局部门具名非端到端）；本机环境（3840×2560@60Hz RTX 4060 Ti、toolchain 1.98.0、auto-lang 1f05effab）；1/4/8 产品映射（§5 冻结：N1=calc，N4=calc/clock/todo/charts，N8=+reader/monitor/photos/video）；四旧红 V-11 适用表（全部 UI063 r4 已根因闭合 290/0，无 known-red 豁免；落地主线复验归 T-04/05，UI073 C-04 同律）；UI073 分工（C-01..04 归其生产修复，本 Plan 最终消费）；重建身份基（07646dd/4ebc51ce/1f05effab/2d698413b/1.98.0）。
- 环境事实：worktree apps/ 容器臂未检出（gitlink 空目录，AGENTS.md 预期）；T-04 运行产品走 §2.1 独立 clone/组合检出 + 隔离数据目录。`scripts/rq-parity/` 为 T-04/05 新增。本 receipt 仅 T-01 关闭，非实现验收。

### 2026-11-08 work 进展（goal round 1 续：T-03 桌面泵接线落地，组 UI 树）

- stage:work / plan_id:auto-os/PLAN-063 / plan_revision:r4 / outcome:pass（T-03 实施面） / worktree:D:/autostack/.wt/os-063/auto-os + 组UI树 D:/autostack/.wt/os-063/auto-ui / branch:codex/plan-063-dev + auto-os-dev / base_commit:07646dd(os)+4ebc51ce(ui) / code_commit:UI 076167aa（os-plan-063 T-03 wiring） / dependency_revisions:auto-lang 1f05effab、auto-web 2d698413b、auto-down 58ea33f、auto-man 347c29f（组内只读树） / task_ids:T-03 / evidence:plan-artifacts/plan-063/auto-os/t02-t03-evidence.md / blockers:无（T-04 起等 UI073） / next_responsible_role:work（T-02 boot 探针 → 阻塞面登记）
- 实施内容（§5 r3 授权的同组唯一 UI 协作树写面）：`PUMP_MSG_BUDGET_PER_TICK` 转单源 pub（rqhost.rs）；session.rs 抽 `pump_one_broker_client`（预算切片+viewport 同步+主题推送，日常/即时共用）+ `pump_broker_client_for_wid` 即时泵 + 7 个输入/焦点发送臂后联 + `broker_pump_tick`/`DesktopEvent::BrokerPump`；host.rs 订阅装配（broker 客户端在场 → 15ms 活跃档专泵，daemon `rq_subscription` 同律同值；零客户端 400ms 待命档兜底）。消费 UI063 已交付合同，无新机制。
- 陈旧测试对账：`keyboard_mapping_shapes` 旧断言与 UI063 G-02 已交付语义相悖——stash 对照证实 pre-existing red（4ebc51ce 亦红）；断言对齐为 `KeyReleased{key:13}`，doc 同步。非机制变更，不扩成第二个 UI063。
- 受影响族门（最终修订）：`ui::session` **108/0**（含新增 2 测）；`ui::desktop_protocol` 冻结滤程（单线程）**290/0**（284.59s，UI063 基线复现）。
- 新增线级证据：`p063_os_desktop_pump_budget_slices_flood`（64/拍切片、余量 FIFO 续排、不误杀）；`p063_os_desktop_input_immediate_pump_same_call`（KeyPressed+Resize 同调用上线，无 400ms 档等待）。
- 本 receipt 仅 T-03 实施面关闭；T-02 boot 探针与 work 阶段整体收口证据后随。

### 2026-11-08 work 进展（goal round 1 收口：T-02 装配闭合；T-04/05 阻塞登记）

- stage:work / plan_id:auto-os/PLAN-063 / plan_revision:r4 / outcome:pass（T-02）/ worktree:D:/autostack/.wt/os-063/auto-os / branch:codex/plan-063-dev / base_commit:07646dd / code_commit:7b3946b（Cargo.lock 随 UI063 交付图最小重解析 +38 行 accesskit 族） / dependency_revisions:UI auto-os-dev 076167aa、auto-lang 1f05effab、auto-web 2d698413b、auto-man 347c29f / task_ids:T-02 / evidence:plan-artifacts/plan-063/auto-os/{t02-t03-evidence.md,boot-probe.log,boot-probe2.log} / blockers:T-04/05 等 UI073（executing 0/8） / next_responsible_role:user（交付 UI073 后续接 T-04）
- `cargo check --offline -p os-desktop --bin ui-desktop` PASS；`cargo test -p os-desktop` **18/0**；smoke-031/034 判定历史入口（era-031/034 布局，不抵全量）。
- boot 探针（真实启动）：冷一次性 lang consumers 构建 2m38s 后装配链全绿——健康报告 registry=26/可见 25（双臂发现：manifest 臂对 worktree 空容器具名 skip、容器臂经 CLI apps-dir 命中主检出）、私 rqhost `owner=desktop pid=35756 endpoint=autodesk-rqhost-desktop-26560 ready=true`、主题包 themes v0.2.0 装载、syslog host logger 在线；收尾 taskkill 后 `rqhost --pipe` 进程=0 无泄漏。探针身份三二进制 SHA256 前 16 绑定（t02-t03-evidence.md）。
- **阻塞（§10 同步）**：T-04 进入门"UI073正式交付且同版消费产物已重建"未满足（UI073 executing r1 0/8）；T-05 依赖 T-04。T-01/02/03 work 面已闭；T-06/07 待 work 全闭后。
- work 阶段在本轮的诚实边界：已达 UI073 之外的可用终态；恢复动作 = UI073 正式交付归档后，重建同版消费产物（本树 pull/copy UI master 交付 commit）→ T-04 六格与全产品运行。

### 2026-11-08 goal round 1 收据（work 阶段：T-01/02/03 闭，T-04/05 阻塞于 UI073）

- stage:goal / plan_id:auto-os/PLAN-063 / plan_revision:r4 / round:1 / from:work / to:work / status:executing / outcome:blocked（T-04 进入门"UI073正式交付且同版消费产物已重建"未满足——UI073 executing r1 0/8；非用户决策，外部计划交付依赖） / worktree:D:/autostack/.wt/os-063/auto-os（+组内 auto-ui/auto-lang/auto-web/auto-down/auto-man 五树） / commit:OS codex/plan-063-dev 7b3946b、UI auto-os-dev 076167aa、main 计划文档 5fcc024+1f2cd14 / next:user（UI073 正式交付归档后，本组 UI 树同步其交付 commit、重建同版消费产物 → 恢复 T-04 六格与全产品运行 → T-05 → review → merge）
- 本轮完成：T-01 冻结清单（t01-freeze.md 十节）；T-02 装配闭合（build+18/0 测试+真实 boot 探针，身份哈希绑定，无 daemon 泄漏）；T-03 桌面泵接线（UI 树 076167aa：预算切片单源+7 臂即时泵+15ms 活跃档订阅；ui::session 108/0、desktop_protocol 冻结滤程 290/0；pre-existing 陈旧断言对齐 G-02 并 stash 对照留痕）。计划状态 drafting→executing，current_step 3/7。
- 无需用户裁定的待决；等待项为 UI073 交付（他仓他计划）。本 receipt 不签任何最终验收（AC-01..03/07 的最终证据归 T-04/05）。

### 2026-11-08 goal round 2（waiting：无工作无检查）

- stage:goal / plan_id:auto-os/PLAN-063 / plan_revision:r4 / round:2 / from:- / to:- / status:executing / outcome:waiting（等待轮1/3） / worktree:同 round 1（保留不动） / commit:不变（OS 7b3946b / UI 076167aa / main c17a4c8） / next:user（UI073 交付后恢复 T-04）
- 持久事实核查：UI073 仍 executing r1 但已推进 1/8（T-01 媒体容量探针、T-03 A5 持久 tiny_skia recorder 落其 worktree，rq_ 族 46/46 绿）——积极信号但交付门未满足。另：auto-ui master 新增 3 个 docs 提交（7bc8f5cb 等，含 062 direct-draw replay 路径用户裁定）——T-04 恢复时组 UI 树需同步 master 后再重建同版产物。
- 本轮按等待轮协议：无 stage work、无检查、无树变更。

### 2026-11-08 goal round 3（waiting：无工作无检查）

- stage:goal / plan_id:auto-os/PLAN-063 / plan_revision:r4 / round:3 / from:- / to:- / status:executing / outcome:waiting（等待轮2/3） / worktree:同 round 1（保留不动） / commit:不变（OS 7b3946b / UI 076167aa / main c17a4c8+） / next:user（UI073 交付后恢复 T-04）
- 持久事实核查：UI073 仍 executing r1 current_step 1/8，worktree tip 未变（20590168）；auto-ui 本地 master tip 仍 4ebc51ce（round 2 所见 7bc8f5cb 系 origin/master 前向引用，本地未并——组树与本地 master 一致）。交付门未满足。
- 本轮按等待轮协议：无 stage work、无检查、无树变更。

### 2026-11-08 goal round 4（waiting 3/3 → goal blocked）

- stage:goal / plan_id:auto-os/PLAN-063 / plan_revision:r4 / round:4 / from:- / to:- / status:executing / outcome:blocked（连续三等待轮满，goal 标记 blocked；非计划内可解） / worktree:同 round 1（五组树+两 dev 分支+plan-artifacts 全保留，恢复零重建） / commit:不变（OS 7b3946b / UI 076167aa / main 0a078a0+） / next:user
- 持久事实核查（第三轮同向）：UI073 仍 executing r1 current_step 1/8、worktree tip 20590168 未变、未归档。交付门未满足，等待轮协议用尽。
- **确切解除动作（唯一）**：auto-ui/PLAN-073 正式交付归档（其会话完成 work→review→merge）→ 本 goal 恢复：①组 UI 树 `.wt/os-063/auto-ui` 同步其交付 commit；②重建同版消费产物（三库）；③T-04 六格与全产品运行 → T-05 → T-06 独立 review → T-07 merge 归档。
- 在此之前本 goal 不再消耗等待轮；所有检查点已固化（§9 round 1 收据）。

### 2026-11-09 work 进展（goal round 5：UI073 交付后恢复——同步/F-1 关闭/族门重绑）

- stage:work / plan_id:auto-os/PLAN-063 / plan_revision:r4 / outcome:pass（F-1 关闭 + 同步 + 门重绑；T-04 战役 running） / worktree:D:/autostack/.wt/os-063/auto-os + 组UI树 / branch:codex/plan-063-dev + auto-os-dev / base_commit:UI master bcceb419 / code_commit:UI 71150e75（T-03 重放）→ 17880ccb（F-1）；lang 重冻结 d057ae5ea / dependency_revisions:auto-web 2d698413b、auto-down 58ea33f（=UI073 谱系） / task_ids:T-04（部分） / evidence:plan-artifacts/plan-063/auto-os/{t04-gates-context.md,t04-campaign-runbook.md,t04-desktop.log,t04-calc*.log,dp-filter-*.log} / blockers:无 / next_responsible_role:work（T-04 产品战役）
- UI073 交付核验（archived r1 8/8，交付 82b61084，canonical 3 文件落地）；组 UI 树 rebase 后 T-03 重放、**F-1 scale hold 关闭**（OS063 域）：`DesktopState.device_scale` 双源落簿（boot 一次性 scale_factor Task + Rescaled 事件）宿主窗门控 + `broker_client_content` v2 臂消费 `broker_v2_device_scale`（rqhost 同律）——boot 探针实测 `[ui-desktop] host device scale 2`（本机 scale=2，HiDPI 软字关闭）。
- 族门重绑（谱系变更后）：`ui::session` **109/0**（含新 F-1 测）；rq_ nextest sanctioned **50/51**（唯一 = capability_rows 撞 profile 120s 悬崖：solo 我的 tip 114.41s PASS / 纯基线 144.04s PASS——机器速度边界非 oracle 失败）；cargo 顺序上下文双 lineage 对照（基 3 红/tip 5 红，红项 nextest 全绿）判为不承载门（nextest.toml 隔离窗语义自证）。**p073_t05/t02 媒体语义（desktop 臂经本 Plan 泵接线）nextest 全 PASS——泵接线与媒体门无交互回归。**
- 产品 pin 对账：worktree gitlink 与主检出一字节一致（OS054 交付组合）——同身份复用前提成立。产物重建三件哈希绑定（runbook §身份）。三产物 id：auto `215a1354…`、tool `a8cc4d74…`、ui-desktop `3984d8bd…`。
- T-04 战役情报固化（runbook）：两宿主两管道分野（broker `autodesk-broker`=虚拟窗嵌载 vs 私 rqhost=standalone）；实测外部客户端无 incubation 动词不被 broker 收纳、incubate 名字解析走框架路径——**产品运行正道 = 桌面注册表 launch 链**；探针进程已全清。战役剩余 7 步清单化。

### 2026-11-09 work 进展（goal round 6：T-04 证据落定 + 具名差额登记 attach 门）

- stage:work / plan_id:auto-os/PLAN-063 / plan_revision:r4 / outcome:blocked（T-04 产品嵌入面：外部 CLI 客户端→broker attach 未达 Active——具名差额，owner=UI063 后继修复合同；六格/媒体/生命周期面已按可落口径取证） / worktree:同前 / code_commit:UI 17880ccb、OS 7b3946b（无新代码） / task_ids:T-04（部分：六格机制面✓ 产品嵌入面✗具名） / evidence:plan-artifacts/plan-063/auto-os/{t04-campaign-runbook.md,t04-calc{2,3,4,-r,-r2,-r3}.log,t04-desktop*.log} / blockers:见§10 具名差额 DG-1 / next_responsible_role:user（UI063 后继合同修复 attach 后恢复产品战役）
- **DG-1（具名差额，三形态全参数复现）**：外部 CLI 客户端 `auto run --render-mode=desktop
  --desktop-endpoint=autodesk-broker --autodesk-incubate --app386=<id>
  --autodesk-broker=autodesk-broker`（registry argv 同参；+`--autodesk-render=remote`
  +`AUTO_UI_IN_DESKTOP=1` 全 parity；release 档；隔离基座）：broker 收纳 incubation →
  宿主 `attach_pending_incubations(5000)` 预算内未达 Active（无 codec/on_message 错误行
  = **宿主零收到客户端消息**）→ 弃连 → 客户端 `exit=HostLost revision=0/1`（连接后
  静默挂起，无编译/装载日志）。**in-test 程序化路径全绿**（t3/stress 真子进程 +
  nextest rq_ 族本谱系 50/51 含 p073 媒体 desktop 臂）——缺口定位在外部 CLI 客户端
  形态的 attach 机械（通用客户端/attach 域 = **UI063 责任面**；r4 生产修复路由同律：
  输入/调度根因归 UI063）。本 Plan 不修（§5 r3：缺通用能力具名差额/后继修复合同，
  不静默扩成第二个 UI063）。
- **T-04 可落证据（本谱系实测）**：①六格机制面 = rq_ 族（rq_wire 传输六格/rq_cells
  7 fixtures/rq_matrix 负控/rq_media 两宿主四相）nextest sanctioned **50/51**（唯一 =
  capability_rows 120s 配置悬崖，solo 双 lineage PASS——t04-gates-context.md）；②V-11
  四旧红 = UI073 落地主线两轮绿 + 本谱系 rq_ 族绿（复用成立）；③F-1 scale hold 关闭
  （boot 实测 scale 2 落簿）；④V-12 身份：release 三件哈希（runbook）+ boot PIDs；
  ⑤隔离基座 boot 绿（registry 8/8、私 rqhost、主题包、无泄漏收尾）。
- **T-04 具名待办（DG-1 解除后即恢复，不缩分母）**：逐产品桌面嵌入运行（N=1/4/8 冻结
  映射）、M-02/06/07 终值、V-08/V-10 终值、V-01 全分母运行态复核——全部挂 DG-1 关闭
  条件；M-01/M-04/M-05/M-08 按交付证据复用（机制同、身份未变）。

### 2026-11-09 goal round 7（PLAN-074 开立 + T-01 红线闭合）

- stage:goal / plan_id:auto-os/PLAN-063 / plan_revision:r4 / round:7 / from:- / to:- / status:executing / outcome:waiting（DG-1 修复已开工：auto-ui/PLAN-074 T-01 完成） / worktree:不变（UI auto-os-dev 树续用） / commit:UI 80f0c3bf（074 合同）→ c2254800（T-01 红线测试） / next:PLAN-074 T-02（transport 层修复）
- 用户裁定"开修复计划修复"→ **auto-ui/PLAN-074**（074-rq-desktop-broker-attach-closure.md，master 80f0c3bf）：DG-1 → 外部 CLI 客户端 attach 闭合；5 任务（红线→修复→真桌面端到端→review→merge）；树延用 os-063 组 UI 树（auto-os-dev，谱系连续）。
- **T-01 完成（c2254800）**：红线测试 `p074_broker_incubate_vm_client_attach_active` 5s 确定性复现 DG-1；根因经逐层插桩缩定**传输配对层**（客户端 Hello 已发出、宿主 pending end 零读到——tokio pipe serve 端配对/读线程启动序嫌疑）；T-02 续作断点固化于 runbook"PLAN-074 T-02 续作断点"节。

### 2026-11-09 work 进展（goal round 8：DG-1 关闭——074 交付消费实测成功，N=1 落地）

- stage:work / plan_id:auto-os/PLAN-063 / plan_revision:r4 / outcome:pass（DG-1 关闭确认 + N=1 落地） / worktree:同前（UI auto-os-dev == master 31e532e4） / code_commit:UI 5d8056f3（074 交付，含我方 F-1/红线 rebase 续含） / task_ids:T-04（产品嵌入面恢复） / evidence:plan-artifacts/plan-063/auto-os/{t04-desktop-iso.log,t04-calc-iso.log,t04-n1-iso.png,t04-campaign-runbook.md} / blockers:无 / next_responsible_role:work（N=4/8 扫 + M 终值）
- PLAN-074 交付核验（archived r1 5/5 @ 5d8056f3，根因 = 孵化 resolve 失败臂）；组树同步；release 产物重建绑定（auto f2c2abfe/tool a3a6bf22/ui-desktop 993f85ee）。
- **N=1 实测**：隔离 broker 井盖（AUTOOS_BROKER_PIPE——共享 well-known 曾短暂触达用户生产桌面，已纠）+ 全方言外部客户端 → 客户端全程存活无 HostLost → 我的桌面日志 `adopt surface 480x320` + `frame compose wid=1 frame_id=1`（虚拟窗帧合成）。**零拒绝/零 budget 失败。战役配方冻结于 runbook（N 扫描直接复用）。**
- 运行配方情报：宿主 attach 按 manifest id 解析（app_resolver）；AUTO_386_APP_ROOT=probe-386root 供客户端装载；桌面注册表经默认 extra-roots 吸入主检出臂（注册表隔离不完整，管道隔离已足够）；V-02 视觉证据一律按 PID PrintWindow。

## 10. 待澄清事项

- **DG-1（2026-11-09 登记的唯一未决）**：外部 CLI 客户端→broker attach 5s 预算未达
  Active（详见 §9 round 6 receipt：三形态复现、宿主零收消息、in-test 程序化路径绿）。
  owner = **UI063 后继修复合同**（通用 attach/客户端机械域）。解除动作 = 该合同交付
  attach 修复并归档 → 本 Plan 恢复 T-04 产品嵌入战役（runbook 战役剩余 7 步）→
  T-05 → T-06 → T-07。此前 T-04 的产品面/M 终值/V-08/10 具名待办维持待办（不缩分母）。
- **T-04/05 阻塞（2026-11-08 goal round 1 登记）**：唯一未决前置 = auto-ui/PLAN-073 正式交付（当前 executing r1 0/8：连续媒体/精确绘制/资源乱序负控生产修复）。解除动作：UI073 归档后在本组 UI 树同步其交付 commit、重建消费产物，即可恢复 T-04。T-01/02/03 已闭不受影响。
  【已解除 2026-11-09：UI073 archived r1 8/8 @ 82b61084，组树已同步】
- r4当前未决：T-01冻结实际入口/预算/旧红当前适用及必要关闭合同；媒体/绘制/乱序资源详UI073。不把062的阶段性豁免挪作M1豁免。
- 当前正式来源UI060/061/062和OS054均已交付，旧r3“在途/未授权研究/Spec缺席”仅历史观察，不作为本轮阻塞；本轮未实施。
- Linux全平台/compositor、ACE/AGE独立进入门保持；无必须重新询问才能完成本次草案的问题。
