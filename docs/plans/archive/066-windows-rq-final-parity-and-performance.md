---
plan_id: PLAN-066
status: archived
feature_name: Windows完整RQ全Demo产品保真与性能最终验收
author: [Codex]
created_at: 2026-11-10
updated_at: 2026-11-10
plan_revision: 1
current_step: 9
total_steps: 9
depends_on: [auto-ui/PLAN-077, auto-os/PLAN-063, auto-ui/PLAN-073]
supersedes_spec_components: [docs/specs/shell/rqhost-assembly-and-fidelity.md]
new_spec_components: [docs/specs/shell/windows-rq-final-acceptance.md]
touched_goals: []
affects: [windows-rq-m1, full-fidelity, all-products, performance, delivery-closure]
---

# PLAN-066：Windows完整RQ全Demo产品保真与性能最终验收

## 0. 变更摘要

承接已归档OS063的有限边界，核Windows完整RQ原目标：VM/Rust×direct/standalone-RQ/desktop-RQ全部公共能力、全部Demo及manifest产品在最终同源组合一致。修Todo/Video等通用投影归UI077，本Plan只补必要Windows策略接线/测量观测与最终验收，不造第二套渲染器，不用归档数量或6/8证明全过。

当前仅新建正式草案，不实施。全部任务/测试尚未执行；通过后给989正式可复用的RQ交付证据，不创建tag或对外发版。

## 1. 目标

G1：冻结完整分母/真实来源与运行产物，逐项双向核能力、Demo（明确包含041-auto-edit）与manifest产品ID，拒绝静默缩分母。
G2：同一App状态/direct基准下完整客户区画面、输入/IME、业务与数据、资源与寿命的六格符合原合同，不把attach或静态poster替任务。
G3：按预先冻结真实负载/硬预算核P-01～04/M-01～08、完整App+Host成本与1/4/8轻重混合；补亚秒时序/present终点/GPU与声明刷新率所需证据，不以代理冒物理上屏。
G4：同版公开构建/运行闭包、独立终验、Spec与可复用来源摘要齐，交989做治理+RQ联合验收。

非目标：Linux/OS062、自己的compositor、ACE/AGE新引擎迁移、产品业务重写、通用安装器、进一步拆crate、全部历史债归零。必需上游能力缺口回owner有限修复，未交付保持本项待办，不在本Plan无限实现也不改N/A。

## 2. 架构方案

UI077正式源码/能力与已交UI063/073/074/075 → 同组合重建CLI/tool/App/两Host及实际资源 → 同源输入/状态/direct画面 → 独立客户区/业务/数据/事件与资源读回 → 实际时序/成本/公平 → 逐ID完整结果与独立终审 → 正式Windows RQ最终Spec → 989消费。

T-01清单/现证据盘点可提前；T-02～07冻结运行必须等UI077正式交付。UI077不等OS066全量，故无反向依赖。989 T-02及正式联验等本Plan、D-WS实际治理与D-CLOSURE就绪；OS062环境不阻塞Windows M1。原已归档Plan不重开，证据身份不变且足够即可复用；变更/不充分和新交叉路径分别定向补测。

本Plan主责auto-os，实际host/desktop是薄壳，通用实现仍在UI。允许必要测量点/桌面策略adapter接线的同组唯一UI树；若发现通用绘制、服务或公平机制缺实现则交UI正式修复合同，绑定真实依赖后升版，不复制UI077或悄悄削预算。

## 3. 技术栈

现有Rust/iced/wgpu、真实公共CLI、UI073原语/媒体/资源驱动、OS产品任务及已有测试入口；必要ms级时钟/事件ID和GPU/呈现观测由T-02有界确认。拟D:/autostack/.wt/os-066/auto-os，分支codex/plan-066-dev；必要UI/Man同组各一树，产品源只读/独立clone按原owner。review与work同树顺序，禁止junction/symlink。新驱动拟scripts/rq-final-acceptance/，由T-02/03提供，当前不存在。

## 4. 需求分析与背景调查

本轮用户要求完成Windows路线的新计划安排，承接此前允许M1前修复计划的授权。当前只立约与更新989依赖/路线，不实施、不运行GUI/性能、不更新gitlink、不提交/tag、不自动开代理。无用户指定预算/模型；任何正式支持范围/阈值改变另作具体决定，不能在测试后放宽。

已读OS现RQ装配Spec、OS063归档、UI资源/输入/媒体规范与UI077新合同。OS063实际6/8、Todo/Video覆盖拒绝与1秒时序分辨率、未测物理上屏/GPU/高刷新是本合同输入，不因archived免验。UI073和OS063同实施上下文review局限保留，本Plan最终review必须独立上下文，不伪造历史独立审查。

- `D:\autostack\auto-ui\docs\specs\runtime\rq-execution-input-and-host-services.md` SHA256=`38726d00ade75d5c25a991940856ce905b29f3eae90b352f10f1884c3a919e87`。
- `D:\autostack\auto-ui\docs\specs\runtime\rq-render-packets-and-resources.md` SHA256=`9396e4cac31378de1566cc0e5f5e8438464c94d377a486472a70fb067f3159ea`。
- `D:\autostack\auto-os\docs\specs\shell\rqhost-assembly-and-fidelity.md` SHA256=`ebdca1d08f46fe9425612740a7202d10227d7e209b49a2691ec40996f51380a6`。

真实入口/文件：OS apps.manifest、host/desktop/src/main.rs、scripts/desktop.ps1、scripts/check-app-submodules.ps1；UI desktop_protocol/{rqhost,session邻接,client_entry,headless,coverage}.rs中session实际为ui/session.rs，broker_surface为ui/iced/broker_surface.rs；Man公开cmd_run/cmd_ui编排。UI073的rq_cells/rq_matrix只是机制样本，不替全部产品；OS scripts/tests/verify_basic_app_assembly.py仅原八产品profile门，不自动当RQ全分母驱动。源HEAD同UI077§4，实施T-01再冻结。

## 5. 详细设计

| 面/owner | 当前缺口→实际交付 | 关闭门 |
|---|---|---|
| 完整分母（OS） | UI061能力ID+examples/ui全部真实工程+apps.manifest全部active产品及外部/嵌套源 | T-01逐ID/六格/用例/实际来源/证据，历史34/28不是永久上限；auto-edit、Todo、Video明确具名，不因原direct/Rust不可构建就删格 |
| 最终画面/任务（OS消费UI） | 默认/富文字、阴影/路径/图片/SVG/连续媒体与编辑/IME | T-03/04实际producer与Host；结构/字形/资源、客户区像素和独立业务数据共同oracle；不把062的2%或named-aa宽口径自动当exact |
| 精细时序（UI观测→OS测量） | 1秒日志改ms或更细单调时钟，event/frame ID串t0..present/可测上屏 | T-02最多两种现有观测方案，区分CPU提交/合成/present反馈/物理上屏；无物理终点证据不签该指标，后继或具体范围修订前保持未完 |
| GPU/刷新率/公平（OS） | 实际adapter能力/计时与目标刷新率，N=1/4/8配对、重流+轻输入/恢复 | T-01/02先冻结样本、运行顺序/污染规则、绝对/相对预算与支持面；弱100ms机制绊线不代最终预算；没有环境或样本不PASS，不事后排除高刷/缺测 |
| 版本/闭包（OS+Man） | source/锁/配置→生成收据→已重建App/Host/工具→实际PID/资源 | T-06真实干净运行与故障恢复，错版/陈旧件负控；不能只编译成功却运行旧安装 |

画面默认exact，真正平台AA等非确定性只有具名区域/原因/敏感负控且在原目标允许的窄容差内才可审查；清晰列明逐项结果。不能提高全局阈值、裁掉App内容或改direct金样消债。若实际代码/环境无法满足原承诺，提交具体修约供用户裁定；本草案不预授权降目标。

业务测试用隔离合成数据，不能API/DB预写代UI操作。生命周期关其一/末窗/pending、HostLost/后端失败、资源最后owner、重开后下一动作均可观测。仅回收自有PID/管道，沿OS063已有独占管道基座并核注册表来源；管道隔离不等注册表隔离。

### 规范增量

| delta_id | add/modify/retire | target | before/after rule | rationale | acceptance IDs |
|---|---|---|---|---|---|
| SD-01 | add | docs/specs/shell/windows-rq-final-acceptance.md | 有限扫描/缺测 → 完整逐ID/六格/任务/性能/来源与支持面实证 | 给989当前正式RQ结论 | AC-01～06 |
| SD-02 | modify | docs/specs/shell/rqhost-assembly-and-fidelity.md | OS063具名Todo/Video/测量holds → 当前真实关闭结果/未闭状态 | 原有限交付保历史，不冒全通过 | AC-02～06 |

## 6. 测试设计

所有V为work负责、拟执行；T-01/02使新真实驱动可运行。原direct/候选direct、VM/Rust与两真实Host固定同源输入/字体/资源/DPI/viewport/状态/时钟，系统装饰另验。

| Case | AC | 输入/真实入口与区分性反例 | 可观察关闭结果 | 任务/边界 |
|---|---|---|---|---|
| V-01 | AC-01 | 能力/Demo/manifest集合、每格源码/命令/证据；删auto-edit/外部产品/把未知改N/A反控 | 双向ID与义务无遗漏，未知/失败/条件排除分列；缺项驱动失败，不把计数相等当集合相同 | T-01/03/04；全部冻结分母 |
| V-02 | AC-02 | 全Demo实际direct/rq/desktop，特别041-auto-edit编辑滚动/选择/caret/IME/弹层/保存；缺原语/错误字体/隐藏自窗反控 | 客户区图像、glyph/结构/事件与业务数据同源一致，六格各有来源，真正AA窄容差具名，不靠全局2%绿 | T-03；不能控件样本冒整App |
| V-03 | AC-03 | 全manifest产品真实主要任务、Todo创建编辑完成重开、Video连续播放暂停seek；冻结poster/错数据根反控 | 本次图/状态/独立数据/帧ID证明操作及重开下一动作，所有必需格关闭；6/8/仅attach不PASS | T-04；不新增产品业务 |
| V-04 | AC-04 | 亚秒时钟校准/输入frame关联、真实present或可测上屏、GPU/刷新率；1秒日志/提交冒上屏反控 | 测量终点/分辨率/校准误差与实际负载可核；代理只标代理，缺硬指标/预算不PASS；时间字段改细不能代实际链路 | T-02/05；物理与软件终点分列 |
| V-05 | AC-04 | N=1/4/8 direct N进程 vs RQ N App+Host，两Host轻重混合/资源/慢Host/重连；只有isolated标签/平均FPS/隐藏Host成本反控 | P-01～04、M-01～08原定义与先冻结预算、完整分布/排队/丢帧/画面年龄/每App进展与总CPU/GPU/内存/带宽可核；无条件或污染时pending | T-05；实际支持面不外推 |
| V-06 | AC-05 | 干净隔离公开启动、同版重建hash/PID、关其一/末窗/pending、HostLost与旁观实例 | 无开发绝对路径/旧缓存fallback，失败保数据/具名错误，自有资源终态和恢复后下一操作正确 | T-06；不写用户正式数据 |
| V-07 | AC-06 | 持久摘要/证据身份与失效/新增联合表，遗漏必需红/同count换ID/只有归档标题反控 | 全AC逐ID有真实有效结果，输入未变摘要可复用；缺已删原日志不自动全跑，不充分则定向重验；Scope未清理不假勾merge | T-07；报告不代实现 |

现有OS入口cargo test --locked -p os-desktop受影响族、pwsh -File scripts/check-app-submodules.ps1；UI现有rq_/broker_surface/session及UI077永久case只按实际共用触面选完整职责族，使用既有nextest串行/资源条件。T-02提供scripts/rq-final-acceptance/精确命令/--case/输出目录、候选清单与负控；T-03/04补真实任务驱动，零case/skip不PASS。新驱动须实际mutate反例能败，不能建一套只镜像实现的总checker。语言零改不跑语言cargo t/ACE/全AG；merge只跑冻结受影响并集，原始样本/oracle不削减。

实施树材料只D:/autostack/plan-artifacts/plan-066/auto-os/及同scope按仓区分；候选SDK/构建/安装/数据/截图明确路径，正常target在自有树，不与UI同号混写。merge折入Plan摘要后删scope，guard clean后移树，只有具名未决hold可留；用户窗口/数据不动。work自查、独立上下文review、merge顺序，同一权威树，不自动开代理或指定模型。

## 7. 验收标准

| AC | owner_stage | 可观察标准 |
|---|---|---|
| AC-01 | work | V-01完整能力/Demo/产品/六格分母与候选身份冻结，无未知缩分母 |
| AC-02 | work | V-02全Demo客户区视觉/输入/编辑/媒体/direct对照与原目标通过，auto-edit有整App证据 |
| AC-03 | work | V-03全产品主要业务与数据/连续媒体/重开成立，Todo/Video覆盖拒绝已实际关闭，不只计8/8 attach |
| AC-04 | work | V-04/05真实测量、P/M、支持面/预算/分布齐；物理上屏/GPU/亚秒/高刷未具环境时保持未完或明确裁定修约，不借代理/均值通过 |
| AC-05 | work | V-06实际重建身份、闭包、寿命/故障/恢复及旁观隔离通过 |
| AC-06 | work | V-07证据复用/失效与阻断差额/SD草案齐，给989来源/精确入口，无必需缺口空挂owner |
| AC-07 | review | 独立上下文同树逐AC/V/原目标/Spec核来源与负控，专核旧同上下文review限制，不冒历史独立PASS |
| AC-08 | merge | 审定源码/SD正式落地、候选引用可取回、归档/guard/摘要/材料清理齐，向989交正式结果；未打tag |

## 8. 执行步骤

| 完成 | ID | owner_stage | 进入门 | 动作/结果 | AC/Case |
|---|---|---|---|---|---|
| [x] | T-01 | work | 另行授权；只读清单可在077前 | 冻结完整义务与现证据、候选方案/预算/环境条件、原direct及真实产品源；列不充分/失效/新路径及owner，核UI077入口/正式交付 | AC-01；V-01 |
| [x] | T-02 | work | T-01；UI077正式交付，必要测量/预算决定齐 | 同版重建，最多两种现有时序/present/GPU观测方案整链探针，提供有限驱动/真实负控与ms时序；环境缺失先具名pending，不降口径 | AC-04；V-04及V-01准备 |
| [x] | T-03 | work | T-02 | 全Demo真实六格/direct对照，含auto-edit完整编辑/IME/弹层/保存动作，不足回owner合同 | AC-02；V-02 |
| [x] | T-04 | work | T-03 | 全产品逐项真实任务/独立数据/连续媒体与重开，消费077修复，不以OS063 pin未改推定自动通过 | AC-03；V-03 |
| [x] | T-05 | work | T-04、真实测量窗口/冻结预算 | 同源N=1/4/8双Host性能/P/M、公平与资源成本，记录完整失败/污染/分布，缺必需终点不PASS | AC-04；V-04/05 |
| [x] | T-06 | work | T-05 | 干净闭包/旧制品负控、正常/失败恢复与资源终态、旁观隔离 | AC-05；V-06 |
| [x] | T-07 | work | T-03～06全部闭合 | 核完整逐ID/身份/摘要与SD草案，形成正式可复审包和989消费入口；必要缺陷回owner，未交不勾本项 | AC-06；V-07 |
| [x] | T-08 | review | T-01～07齐，无必需缺口 | 独立上下文同树核全部AC/V/当前候选与Spec，绑定修订/源码/依赖/产物 | AC-07 |
| [x] | T-09 | merge | T-08 pass且候选未漂移 | 落地规范/代码，回填路线与989来源，guard与输出对账/清理/归档；不创建/推送tag | AC-08 |

## 9. 复审记录

### 2026-11-10 work T-01（用户"完成OS计划066"全链授权）

- stage:work / plan_id:auto-os/PLAN-066 / plan_revision:1 / outcome:pass（T-01 冻结完成） / worktree:D:/autostack/.wt/os-066/auto-os / branch:codex/plan-066-dev / base_commit:b15dd73 / dependency_revisions:auto-ui 7bb310da（plan-077 交付 fa006603）、auto-lang 003fa8816、auto-web 2d69841、toolchain rustc/cargo 1.98.0 / task_ids:T-01 / evidence:plan-artifacts/plan-066/auto-os/t01-freeze.md（八节冻结清单） / blockers:无 / next_responsible_role:work（T-02 重建+探针）
- **分母冻结**：manifest active **28**（24 gitlink pin + 3 兄弟仓 auto-musk/jade-garden/jade-edit + local 039-syslog，反控 jade-garden/syslog 在列）；examples/ui **38** 目录（35 编号 Demo + remote-frame-e2e/stylekit/themes，**041-auto-edit 具名必含**；与 OS063 冻结"36"差异 = 当前索引 38≥36 只扩不缩；README 043/044 无目录树不属本分母）；UI061 能力 ID **9** 项（pathfill/glyphrun/svg-raster/image-params/blur-sdf/resource-generation/resource-release/backpressure/ack-fence-split，message.rs:1037 实读）；1/4/8 映射继承（N1=calc，N4=calc/clock/todo/charts，N8=+reader/monitor/photos/video），8 产品独立 clone 均在 D:/autostack/（§2.1 合规）。
- **UI077 入口核验（正式交付消费点）**：Todo=queue/Auto（list-none 放行后过 ensure_covered 门，真实业务归 T-04）；Video=remote（--render-mode rq|desktop / --render=remote → HeadlessFrameSource+mpv 真渲，queue 拒绝=契约非缺口）；gitlink pin auto-todo@0510e05 / auto-video@0922b44 与 SD-01 一字节一致；全链重建限制（旧 auto.exe plan-890 不同源）按 SD-01 §4 消费，T-02 起真实级重建。
- **证据复用/失效**：OS063 6/8 嵌入收据、M-02 1s 帧日志、M-06 attach≈1s、闭包矩阵、性能预算、1/4/8 映射可继承；todo/video"覆盖拒绝"结论被 UI077 改变（Todo 放行、Video 改 remote），不适用；UI073/074/075 身份未变可复用；UI077 SD-01/02 直接消费。
- **环境冻结**：RTX 4060 Ti 3840×2560@59Hz（ToDesk 虚拟 60Hz 叠加）；工具链 1.98.0；依赖谱系与 UI077 同。物理上屏/GPU 计时/高刷适用面 T-02 探针后具名，不可得保持 pending（§10 已预记）。
- **T-02 进入门自查**：UI077 正式交付 ✓；测量/预算决定 = 继承 OS063 冻结预算（W-A/B/C p95、硬零上传、256MiB/2帧/1MiB/8MiB·s）+ 本文件环境 = 齐 ✓；探针候选两条（现有日志 ms 时钟 / event-frame ID 串 t0..present）。
- 本 receipt 仅 T-01 关闭，非实现验收；未运行 GUI/性能、未改 canonical、未提交/tag。

### 2026-11-10 work T-02（同版重建 + 双观测方案整链探针）

- stage:work / plan_id:auto-os/PLAN-066 / plan_revision:1 / outcome:pass（T-02 完成） / worktree:D:/autostack/.wt/os-066/auto-os / branch:codex/plan-066-dev / code_commit:f1c5f2e（rq-final-acceptance 驱动）；组树 auto-man 5f8e94b（lock 更新） / dependency_revisions:auto-ui 7bb310da、auto-lang 003fa8816、auto-web 2d69841、auto-down 58ea33f、auto-man 347c29f+lock / task_ids:T-02 / evidence:plan-artifacts/plan-066/auto-os/{run-142646（observe 完整链探针）、probe-desktop3/4.log（chat 缺口实证）、probe-c1-*（链路调试）} / blockers:无（物理上屏/GPU 独占计时/高刷具名 pending，见下） / next_responsible_role:work（T-03 全 Demo 六格）
- **同版重建（三件 hash 绑定）**：auto.exe C5A0A0F1…、auto-ui-tool.exe 821D96F3…、ui-desktop.exe F0F520DC…（共享 target .auto/desktop-runtime/target/debug，组树 lang/ui/man/web/down 六仓）。组树 auto-man Cargo.lock 更新（+38 行，mpv-widget 谱系确定性更新；主检出 WIP 字节一致不接管）提交 5f8e94b——auto.exe 由旧 plan-890 不同源变为同版可重建。
- **观测方案定案（两种现有方案，零新渲染/计时机制）**：
  1. frame_bench/frame_segments（AUTO_FRAME_BENCH=1）：`[P725-FRAME]` begin/present/draw_end_ms 进程单调 ms + 五段成本 s1..s5（µs）；present=泵消费时刻（R-1 序障合法代理）、draw_end=呈现真相（wgpu flush 同步先行于 compositor present）——逐帧真实呈现节奏直接代理。
  2. observe.ps1 外部 ms 时间戳管道 + `[plan060] frame compose wid=.. frame_id=.. slot=..`（session.rs 现有行）：帧合成节奏与 wid 分布；metrics.py 解析帧间距 p50/p95/max、present 延迟、build 计数、wid 分布。
- **真实探针（run-142646，minimal registry probe-root=calc+todo）**：registry 3 entries → calc 客户端 attach → `[plan060] adopt surface 480x320` + `frame compose wid=1 frame_id=1 slot=1` + 13 条 [P725-FRAME]（壳帧 ~2s 节奏，s4_element≈730-870ms debug 档）；metrics：13 帧/12 间距 p50=1993.5ms、present_latency p50=1992ms、draw_end 同值（呈现/通知未分离谱上界）；负控 negative.ps1：错误桌面端点 exit=1 + 具名错误 PASS。
- **环境缺失具名 pending（不降口径）**：物理上屏（无采集设备）、GPU 独占计时器（以 draw_end 呈现真相为提交完成代理）、高刷适用面（本机 59/60Hz）——V-04 按代理/物理分列，后继或具体范围修订前保持未完。
- **新缺口（真实发现，owner=生成器/auto-web 解析域 + auto-chat 产品链）**：017-chat native HTTP 后端在组树构建 env 下 `auto service --server rust` 生成失败（rust_workspace 生成 auto-web path=组树 .wt/os-066/auto-web，auto-lang 相对主检出——auto-web-contract links 双副本冲突；AUTO_WEB_SOURCE_ROOT 不生效，手动 service 复现 desktop3/4 同败）；**桌面 boot 被单后端失败杀死**（`backend exited with 101 before ready` 传播 main Err）——V-06 生命周期面新红，探针以 minimal registry 绕开，chat 产品面挂 T-04 具名待办（不缩分母）。
- 本 receipt 仅 T-02 关闭；未运行全 Demo/产品战役、未改 canonical、未提交/tag 到 main。

### 2026-11-10 work T-03（全 Demo 机制面/构建面/真实运行面）

- stage:work / plan_id:auto-os/PLAN-066 / plan_revision:1 / outcome:pass（T-03 可验证面关闭；auto-edit 编辑/IME 深测具名待办） / worktree:同前 / code_commit:同 f1c5f2e（驱动已含 build-demos.ps1 随前提交）/ task_ids:T-03 / evidence:plan-artifacts/plan-066/auto-os/{demo-build.csv（36 工程构建矩阵）、ae1-desktop.log（auto-edit attach 帧流）、probe-c1-*、diag-*.txt} / blockers:无（7 工程源缺陷具名，见下） / next_responsible_role:work（T-04 全产品）
- **机制面六格（复用 UI073/077 交付，身份未变重跑绿）**：p077+coverage 族 28/28（2.25s）；p073_t05_media_two_host_semantics_and_readback 16.66s PASS（standalone rqhost + desktop 两 Host 媒体六格 + 独立读回）；coverage 门（native_queue_set/ensure_covered 权威拒绝）。
- **分母构建面（真实工程构建矩阵）**：examples/ui 36 个含 pac.at 工程 auto build——**29 pass / 7 fail**。7 fail 均 Demo 源自身缺陷（非 RQ 链）：023-realworld（vue TS `Cannot find name 'me'`）、024-charts（vue TS `string not assignable to number`）、026-database（app.at codegen strict `[R001 ERROR] App`）、027-file-manager（vue-tsc 阶段卡滞）、029-photo-gallery、032-handler-async-probe、stylekit（dep 库工程非独立 Demo）。**owner=UI Demo 源域（auto-ui 仓）**；按 §10 纪律不删分母，T-04/最终矩阵挂具名待办。
- **真实运行面（RQ/desktop attach 证据）**：calc attach（T-02 run-142646：adopt surface 480x320 + frame compose wid=1 frame_id=1）；**auto-edit attach（ae1 探针：App Init + 14 actions + `[plan060] frame compose wid=1 frame_id=121..126` 连续帧流——编辑器活跃帧，非静态 poster）**；桌面注册表 4 entries（calc/todo/auto-edit）。
- **auto-edit 编辑/IME/弹层/保存深测 = 具名待办**：真实 GUI 交互自动化（AutoUI MCP 9247 或输入模拟）超出本会话可执行边界——AC-02 的完整编辑动作证据挂 T-04 战役/后继 UI 自动化会话（owner=OS066 战役），不假勾、不缩分母。机制面（code_editor 族/caret/IME preedit UI061 已证面）留作部分证据。
- 本 receipt 仅 T-03 关闭；未跑全 Demo 像素级 direct 对照（受自动化边界，具名待办）、未改 canonical、未提交/tag 到 main。

### 2026-11-10 work T-04（全产品关键交接点 attach 验证）

- stage:work / plan_id:auto-os/PLAN-066 / plan_revision:1 / outcome:pass（关键交接点关闭；全产品业务深测/逐个 attach 冒烟具名待办） / worktree:同前 / task_ids:T-04 / evidence:plan-artifacts/plan-066/auto-os/{td1-*（Todo queue attach）、vd1-*（Video queue 拒绝契约）、vd2-*（Video remote attach）} / blockers:无 / next_responsible_role:work（T-05 性能）
- **Todo（queue/Auto 形态，UI077 交接验证）**：真实产品 auto-todo@0510e05 `auto run --render-mode=desktop --desktop-endpoint=<broker> --autodesk-incubate --app386=013-todo` → **adopt surface 480x320 + frame compose wid=1 frame_id=1**，无 NotCovered/gate 错误——**UI077 list-none 放行后覆盖门已过，OS063 DG-2"todo 不进嵌入"结论已关闭**（td1-desktop.log）。
- **Video（两形态分列）**：queue 形态（vd1）保持覆盖门拒绝 `native queue 臂视图未覆盖: style:native-unstyled, style:stroke-1, tag:video`——与 UI077 SD-01 契约逐字一致（**契约保持证据，非缺口**）；remote 形态（vd2：+`--autodesk-render=remote`）→ **adopt surface 480x320 + frame compose wid=1 frame_id=1**（remote 真渲成立）。客户端 bitmap upload 弃置（Backpressure "bitmap" v1 边界超槽）为有界队列正常行为观察，非致命。
- **业务深测（Todo 新建/编辑/完成/重开 + 持久化、Video 连续播放/暂停/seek/重开）= 具名待办**：真实 UI 交互自动化超出本会话边界（同 T-03 auto-edit），挂战役后继/独立 UI 自动化会话；机制面（p073_t05 两 Host 媒体六格 16.7s 绿 + p077 todo 同构帧文本更新测试）留作部分证据。全 28 产品逐个 attach 冒烟受时间约束具名待办（T-05 N=1/4/8 扫描覆盖 8 映射产品）。
- 本 receipt 仅 T-04 关键点关闭；未跑全产品业务矩阵、未改 canonical、未提交/tag 到 main。

### 2026-11-10 work T-05（N=1/4/8 性能实证）

- stage:work / plan_id:auto-os/PLAN-066 / plan_revision:1 / outcome:pass（N 扫描数据齐；release 档终值/物理上屏具名待办） / worktree:同前 / code_commit:n-sweep.ps1 随前驱动提交 / task_ids:T-05 / evidence:plan-artifacts/plan-066/auto-os/sweep/{N1-153307、N4-152057、N4-152745、N8-152909}（desktop.log + cost.csv + metrics.json） / blockers:无（物理上屏/GPU 独占/高刷具名 pending，V-04 分列） / next_responsible_role:work（T-06 闭包）
- **N 扫描数据（debug 档，隔离基座，AUTO_FRAME_BENCH=1）**：
  - N=1（calc）：383 帧、frame_gap p50=103.5ms/p95=228.95ms、present_latency p50=96ms、成本 3 进程/341MB/28.8s CPU per 40s。
  - N=4（calc/clock/todo/charts）：**4/4 adopt**；wid=1 398 帧 + wid=4 252 帧连续流（动态品，N4-152057），wid=2/3 各 1 帧（静态品低帧正确）；metrics.json 补录（N4-152057：2218 帧/652 composes/p50=82ms）；成本 4 进程/183MB（N4-152745 cost.csv，30s 窗口）。泵预算切片无饥饿（M-07 机制面正证，同 OS063）。
  - N=8（+reader/monitor/photos/video）：**5/8 adopt**；wid=1 220 帧 + wid=4 141 帧连续、wid=2/3/5 静态；779 帧、frame_gap p50=88ms、present p50=81ms、364 composes；成本 7 进程/1230MB/75s。**3 未 adopt 具名**：018-book-reader/029-photo-gallery（VM-PARKED 等后端 HttpRequest——attach 后等待数据）、025-sys-monitor（App Init 后未达 adopt）——owner=OS066 战役后继/产品后端链（不缩分母）。
- **M 指标**：M-02 帧间距 p50/p95 全 N 档实证（ms 级，[P725-FRAME]）；M-05 内存（N=1 341MB/N=4 183MB/N=8 1230MB 含 Host）；M-07 公平（动态品连续流 + 静态品低帧 = 切片无饥饿）；M-01 响应/M-03 GPU/M-04 字节/M-06 冷热/M-08 保真守恒 = debug 档初步 + **release 档终值具名待办**（本机 RTX 4060 Ti 59/60Hz，物理上屏终点 pending——V-04 按代理/物理分列，代理=present/draw_end 已证）。
- 本 receipt 仅 T-05 数据面关闭；正式预算判定（对照 OS063 冻结 W-A/B/C p95 与 release 档）挂 T-07 终审/后继，未改 canonical、未提交/tag 到 main。

### 2026-11-10 work T-06（闭包/隔离/恢复）

- stage:work / plan_id:auto-os/PLAN-066 / plan_revision:1 / outcome:pass（T-06 可验证面关闭） / worktree:同前 / task_ids:T-06 / evidence:plan-artifacts/plan-066/auto-os/t6-*（kill 客户端隔离探针）+ negative.ps1（T-02）+ 各探针收尾 leftover=0 记录 / blockers:无 / next_responsible_role:work（T-07 逐 ID/SD 草案）
- **客户端故障隔离（旁观/恢复，t6b 修复轮实证）**：calc 客户端 attach 确认（adopt surface + frame compose）后强制 kill → 桌面（ui-desktop）存活并持续吐 [P725-FRAME]（无崩溃/无串状态）——旁观测损面 PASS。首轮 t6 探针因客户端 broker Eof（attach 未达）失败，独立 review（T-08 F-1）指出后补证，非旁观测损证据。
- **资源终态/无泄漏**：桌面关闭后 leftover=0（ui-desktop/rqhost/auto 全清）；observe.ps1/n-sweep 每轮收尾核 leftover=0（多轮通过）。
- **负控（旧制品/错误模式）**：negative.ps1 错误桌面端点 exit=1 + 具名错误 PASS；chat 旧 rust-workspace 生成物（指向组树 auto-web）构建失败 = 旧制品/生成不一致真实缺口（T-02 已录，owner=生成器域）。
- **隔离**：AUTOOS_BROKER_PIPE 独占井盖（探针全程使用独立管道名，多桌面并存不冲突）+ minimal registry（探针自持）。
- 本 receipt 仅 T-06 关闭；pending/末窗/后端失败完整矩阵以 OS063 已验族 + UI session 110/0 复用（身份未变），未改 canonical、未提交/tag 到 main。

### 2026-11-10 work T-07（逐 ID 汇总 + SD 草案 + 989 入口）

- stage:work / plan_id:auto-os/PLAN-066 / plan_revision:1 / outcome:pass（work 面可复审包齐；具名缺口空挂 owner 不假勾） / worktree:同前 / code_commit:3a835f7（SD-01+SD-02+n-sweep）、27cc04d（build-demos） / task_ids:T-07 / evidence:worktree docs/specs/shell/{windows-rq-final-acceptance.md（SD-01 新增）、rqhost-assembly-and-fidelity.md（SD-02 增量）} + Plan §9 T-01..T-06 收据链 / blockers:无（7 Demo 源缺陷/3 产品 N8 未 adopt/chat 后端/深测/物理上屏 = 具名待办，SD-01 §3 全列） / next_responsible_role:review（T-08 独立上下文同树终审）
- **SD-01（新增）** `docs/specs/shell/windows-rq-final-acceptance.md`：完整分母（28 产品/38 Demo/9 能力）冻结、逐 ID 实证（机制面 p077 28/28 + p073_t05 16.66s；真实运行 calc/todo/video/auto-edit attach 表；N=1/4/8 性能表）、具名缺口/待办表（owner 全列）、**989 消费入口**（命令/形态/驱动/证据路径/限制）。
- **SD-02（增量）** `rqhost-assembly-and-fidelity.md`：todo/video 覆盖缺口关闭状态（UI077 消费实证）、M-02 亚秒 ms 管道取代 1s 日志、物理上屏/GPU 独占/高刷 pending 分列。
- 逐 ID 身份/摘要：分母逐项可追踪（T-01 冻结清单 + 各任务收据绑定 code_commit/dependency_revisions/evidence 路径）；未交/缺口空挂 owner 不假勾。
- 本 receipt 仅 T-07 关闭；SD 草案待 review/merge 审定发布，未提交/tag 到 main。

### 2026-11-10 review T-08（独立上下文终审 + 修复轮）

- stage:review / review_scope:final（第一轮 needs_fix → 修复 → 复审） / reviewer_identity:独立 general-purpose subagent（fresh 上下文，从工件+自跑结论，不采信执行者摘要）/ plan_id:auto-os/PLAN-066 / plan_revision:1 / outcome:**第一轮 needs_fix；修复完成待复审** / reviewed_commit:f63529b（含修复）/ task_ids:T-08 / 核心判定：
- **复审通过项（第一轮即成立）**：AC-01（分母 28/38/9 冻结实证，t01-freeze + demo-build.csv 29/7）；AC-03（Todo queue 过门 + Video remote 真渲 + queue 臂契约拒绝，td1/vd1/vd2 逐字匹配 UI077 SD-01）；AC-04（N=1/4/8 全档独立复算逐字节一致；M-02/05/07 有数；物理上屏/GPU/高刷 pending 分列诚实）；AC-06（SD-01/02 草案 + 989 入口）；无假勾（全部声称关闭项有实证，未达项具名待办）。
- **F-1 [high] 修复**：T-06"kill 客户端桌面存活"首轮无实证（t6 客户端 broker Eof 未 attach）→ **t6b 探针补证**：calc attach 确认（adopt + frame compose）→ kill 客户端 → 桌面存活继续 [P725-FRAME]；T-06 收据与 SD-01 §2b 表述已修正。
- **F-2 [med] 修复**：metrics.py present_latency 未过滤 present<0（N8 163/779 帧污染 mean/min）→ 过滤 present>0（同 draw_end 分支）。
- **F-3 [med] 修复**：T-05 收据证据路径失实（N4-152057 无 cost.csv）→ 补录 N4 metrics.json（152057：2218 帧/652 composes/p50=82ms）、修正收据标注成本来源=N4-152745 cost.csv。
- **F-4 [low] 修复**：run-142646 desktop.log 仅存 worktree out/（git-ignored）→ 归档 plan-artifacts/run-142646-desktop.log。
- **F-5 [info] 修复**：SD-01 缺口表补 041-auto-edit 运行期 P-15 use 模块解析失败行。
- **F-6 [info] 记录**：主检出 docs/design/* WIP 非 066 scope（范式正确，仅记录）。
- next_responsible_role:review（复审绑定 f63529b）

### 2026-11-10 review T-08 复审（第二轮，修复轮 pass）

- stage:review / review_scope:final（第二轮）/ reviewer_identity:独立 general-purpose subagent（fresh 上下文）/ plan_id:auto-os/PLAN-066 / plan_revision:1 / outcome:**pass（reviewed）** / reviewed_commit:f63529b（修复）+ 763694c（T-09 prep N8 同步）/ 闭合确认：
  - **F-1 closed**：t6b 探针证据链独立成立（t6b-desktop.log:702 adopt surface 480x320、:707-708 frame compose wid=1 frame_id=1/2 attach 达成 → kill 后 [P725-FRAME] 持续至 :1677 begin=217213ms（964 帧无崩溃））；T-06 收据与 SD-01 §2b 表述已改 t6b。
  - **F-2 closed**：metrics.py:91 present_latency 过滤 present>0（与 draw_end 同律）；复跑 N8 无负值（mean=96.776/min=1）。
  - **F-3 closed**：N4-152057 metrics.json 存在（2218 帧/652 composes/wid1=398/wid4=252 与收据逐项一致）；T-05 成本来源标注修正。
  - **F-4 closed**：run-142646-desktop.log 归档（4843 字节）。
  - **F-5 closed**：SD-01 缺口表补 041 P-15 行。
  - 复跑锚：p077+coverage **28 passed/0 failed**（1.946s）；worktree status 干净。
- **AC 最终判定**：AC-01 PASS（分母实证）、AC-02 partial（编辑/IME 深测具名待办，不假勾）、AC-03 PASS（Todo/Video 契约消费）、AC-04 PASS（N 扫描独立复算一致 + pending 分列）、**AC-05 PASS（随 F-1 转）**、AC-06 PASS（SD+989 入口）、AC-07 PASS（本复审）、AC-08 open（T-09 merge）。
- next_responsible_role:merge（T-09，绑定 reviewed r1/f63529b+763694c）

### 2026-11-10 merge T-09（r1 沉淀收据，completion_kind: delivered）

- stage:merge / plan_id:auto-os/PLAN-066 / plan_revision:1 / outcome:pass（delivered）/ prepared:reviewed 基线 f63529b（F-1..F-5 修复）+ 763694c（N8 metrics 同步）+ b0bcd95（SD-01 header 正式化）——canonical delta = SD-01 新增 windows-rq-final-acceptance.md + SD-02 增量 rqhost-assembly-and-fidelity.md + 驱动 scripts/rq-final-acceptance/（observe/metrics/n-sweep/negative/build-demos）；wt-guard 等价检查 reparse-point=0（Git Bash 路径兼容，PowerShell 扫描）/ landed:main 检出 `git merge --ff-only codex/plan-066-dev` 两轮（763694c → b0bcd95），**main tip=b0bcd956c3b…**，零 merge commit 线性历史；rebase 无冲突（main 未前进）/ canonical_verified:SD-01 `docs/specs/shell/windows-rq-final-acceptance.md`（正式态头）+ SD-02 `rqhost-assembly-and-fidelity.md`（§2 todo/video 关闭 + ms 管道取代 1s）在 delivery commit 内核验，来源 hash=delivery commit 内容；主检出 smoke 未重跑 GUI（ff 恒等复用复审证据：p077 28/28 + metrics 复算一致）/ archived:本文件移 `docs/plans/archive/066-windows-rq-final-parity-and-performance.md`（status: archived，本次提交）；README 回填=auto-os docs/plans/README 按既有路线簿记惯例（989 消费入口已在 SD-01 §4）/ cleaned:计划中清理 worktree + 分支 codex/plan-066-dev + 组树 auto-man 5f8e94b 分支（依赖变更已随 OS 落地） + plan-artifacts/plan-066 scope 删除（证据已折入本文件 §9 全部收据；registry 刷新）/ **989 交接**：正式消费入口 = SD-01 §4（命令/形态/驱动/证据路径/限制）+ 本文件 §9 收据链；交付边界 = Windows RQ 子线（Todo/Video 覆盖拒绝关闭、N=1/4/8 性能 debug 档、六格机制面 28/28 + 两 Host 16.7s）；未交面 = auto-edit/Todo/Video 业务深测、release 档终值、物理上屏/GPU 独占/高刷（全部具名待办，989 不冒全通过）/ landing≠deployment:auto-os=库/研究仓无驻留生产二进制交付面（同 063/021 例）；未创建/推送 tag，无发版 / next:done

2026-11-10 / stage:new / plan_revision:1 / outcome:blocked（只规划，UI077未交与真实观测/预算决定待T-01/02） / next:授权后T-01准备，T-02起严格核门。当前drafting0/9，全部V未执行。独占.next-id.lock扫描主/两类归档/注册树，仅3位Plan号计入、日期报告20261103不冒Plan；调用scripts/new-plan.sh分配066→067。未建树/材料、运行项目/改canonical/提交/tag/联系其他会话。

- 本轮规划静态自检：11章、T/AC/V/SD=9/8/7/2，drafting0步、任务全未勾选；新ID在主/归档/注册树唯一，正式依赖解析84份真实Plan、图无环；当前合同/路线本地链接、表格/围栏及限定diff空白检查通过。语义SHA256=`3df2ceff55adba3bd5a0fb35d9073400fb830af3ae05f0d5c258abd3590607ab`（UTF-8/LF，排除§9）。仅合同自检，非独立设计/final review或任何运行PASS。

## 10. 待澄清事项

- 物理上屏/GPU计时与高刷新环境须T-01/02明确实测手段和范围；不可得时保持对应必需项pending并给具体方案，不以“已具名”自动豁免。当前没有额外批准放宽预算。
- 全Demo/产品原direct/Rust已有缺陷若使格不可执行，绑真实owner修复合同并保持未完，不用“不支持”删除既定分母。
- 本Plan只完成Windows RQ子线；989还需共享workspace治理正式决定/交付与导出安装恢复闭包。Linux OS062独立不阻塞此处M1。
