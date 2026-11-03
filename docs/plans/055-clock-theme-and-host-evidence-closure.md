---
plan_id: PLAN-055
status: drafting
feature_name: auto-clock正式主题接入与宿主真实事件及跨进程验收收口
author: [Codex]
created_at: 2026-11-03
updated_at: 2026-11-03
plan_revision: 1
current_step: 0
total_steps: 11
supersedes_spec_components: [docs/specs/shell/host-theme-policy-and-delivery.md]
new_spec_components: [docs/specs/apps/clock-theme-adoption.md]
touched_goals: []
affects: [auto-clock, host-theme-policy, desktop-protocol-adoption, apps-clock-gitlink]
depends_on: [auto-os/PLAN-053, auto-ui/PLAN-031, auto-ui/PLAN-033]
---

# [auto-os/PLAN-055] auto-clock正式主题接入与宿主真实事件及跨进程验收收口

## 0. 变更摘要

接收053有限交付，以独立产品**auto-clock**作为第二主题消费者，接036正式包发行/生成输入；保时钟业务，以真实生成应用验证outproc attach→push→applied→重连/stale链，闭G-2载体差额。在执行时获准窗口实测真实OS主题事件并恢复设置，闭G-1 hold。不是UI012教学例、八产品装配或新的通用通信平台。

drafting r1、0/11。2026-11-03用户只授权后续调研立项，本轮暂不实施，无树/产品改动/GUI/构建测试/系统设置或子模块升级/远端操作。取号按本仓new-plan.sh并独占锁，未创建或联系其他agent/chat、提交/push、改canonical。

## 1. 目标

- G1：clock消费正式包，独立默认、follow/override、拒绝保旧与恢复真实生效；Vue/VM业务保持，外部来源不靠手拷教学包/主检出兜底。
- G2：真实clock outproc载体完成连接/Hello/caps、attach推送、帧后applied、重连最新快照及stale/旧session分列，不用协议helper或config轮询顶替。
- G3：真实OS事件→宿主→clock/calc运行帧闭环，重启policy仍system，原用户值精确恢复。
- G4：四tab、storage/Tick生命周期保持；产品候选/正常push/正式组合pin同源，保护054和全部非目标产品。

非目标：新时钟功能/天气数据、054八app装配、049通用通信、公开协议改版/新宿主平台或服务supervisor、B2/B3迁移、Linux/Mac新支持、全机测试/缓存清理或公网发布。

## 2. 架构方案

OS拥有policy/持久化/连接路由与组合，UI拥有031选择和053 transport，clock拥有业务/跟随声明与计时。036提供正式包渠道/输入，本包只消费，不复制palette/摘要算法。

沿现theme-policy/1、帧17/18、零装载adopt、arm_standalone_follow及结果状态面。T-01必须验证clock实际生成/启动→broker→Hello路径可行性；必要接缝仅限现adopt/连接/回读，不凭猜测承诺一个参数即可完成。若需新平台/后端/公开协议，先具名前置/重规划，不把G-2降成模拟客户端测试。

## 3. 技术栈

AutoOS desktop脚本、apps.manifest/gitlink、shell/config；UI ui/theme_policy.rs、desktop_protocol/session/standalone/Component与原生生成入口；clock .at/pac/tests，原MCP/Playwright及系统provider。没有新跨进程服务。

## 4. 需求分析与背景调查

已读auto-plan-new/verification-contract、OS AGENTS、shell/host-theme-policy-and-delivery、apps/calculator-theme-adoption、UI runtime/host-theme-transport及主题包/作用域，053归档/KNOWN-DEBT、054和036/037/038合同，详[第二阶段调研](../../../auto-ui/docs/design/autoui/theme-rollout-stage-two.md)。OS docs/specs/README.md不存在，以相关现Spec为源，不杜撰索引内容。

观察OS HEAD 092254ce037ef7c7612cf11fe8b3fe00c57a76c6、UI 56ed6ac33b364ae4045115561468bc6c4e1c4cea。clock独立clone D:/autostack/auto-clock HEAD=OS apps/auto-clock gitlink=551d05e065eb2f449c643b3fa4fd671a9ec7a307，观察时clean；pac端口17812、无业务后端、尚无正式themes依赖，tests/desktop_mcp.py含四tab业务oracle。README/测试头有旧编号子模块/路径，工作按实际定位修正，不当已执行证据。

选clock而非weather：weather008仍execution_done，其pac/README明确VM进程内真天气与Vue生成桩数据差异，主题包不应吞入其产品差额。054明确排除主题，拥有八app组合/pin，本包不重复立约或升级那八产品。

053 archived10/10已落地清理，但G-1真实OS事件仍hold（owner user/merge，需窗口许可），G-2协议已实现但缺真实载体、曾自采纳Hello未达broker。旧probe scope已删除，从归档摘要/现代码重建。**本包两项均为必需work验收，不能再仅列hold而称完整交付**。原归档历史不重开；按新证据关闭债务/持有。

## 5. 详细设计

### 5.1 启动与下一任务

尚未获实施授权。拟唯一组D:/autostack/.wt/os-055/{auto-os,auto-ui}、OS分支codex/plan-055-clock-theme，UI参与仓各一树/分支；lang/Man/Web/Down按需固定兄弟源，不建链接。工作/独立review顺序共用同树，review暂停写入且不修代码。

**产品遵OS AGENTS §2.1例外**：开发/验证/review共用D:/autostack/auto-clock独立真实clone main，一写入者；不在apps组合子模块开发、不建第二review树、不切走他人WIP。实施时产品README加本主合同反链。产品候选提交→验证→独立review→正常push→OS干净组合fetch/detach/pin，产品clone不是临时树，不随组清理。

授权后T-01冻结U053、036状态、源/tool/package、实际broker路径、准确CLI参数/端口/隔离数据/config与磁盘/原生窗口资源；整链可行性决定先于依赖实现。T-02和最终正式包腿等**U036**；T-01可在授权后先核，037/038/054均不是整项前置。若需新能力，给owner/具体关闭条件或重规划，不无界修新后端。

### 5.2 产品业务与主题

按036正式版本/渠道绑定包source/hash/输入，沿031/053 default/follow/override优先级，坏包/版本/模式拒绝保旧，恢复后真实消费。clock与既有calc相反选择同场，切clock override不改变calc/宿主，恢复follow追最新revision。

保四tab原门：秒表Start→走表→Lap→Stop/Reset；计时器00:00:01到零banner/Dismiss；世界时钟≥8行、北京/伦敦沿原now_sec和±60s容差；下一分钟闹钟及storage跨重启恢复。换肤/attach/reconnect不重置elapsed、不倍速/重复Tick、不重复触发闹钟，不放宽原容差或改算法凑绿。

### 5.3 两条通知腿与失败

standalone Vue/VM可沿现config轮询验证初始/外写/override/恢复。**outproc必须是真实clock生成exe**，现broker/adopt连接/Hello归该进程，push revision须在帧评估后applied；重连取最新完整snapshot，重复/乱序/旧session结果不覆盖当前。宿主连接ID/状态、应用实际颜色/帧和时钟业务一起证明，不能仅测roundtrip、只开独立窗口或读取config替代G-2。

拒绝/unsupported/stale/pending/applied分列，收到帧不等应用成功。关闭/失败只回收本次连接/订阅/owned进程，calc和旁观桌面保留。恢复后下一次实际操作成功，不以父exit0遮子项失败。源码/产物身份和910可写target/lease/锁规则保持。

### 5.4 真实OS事件窗口

本轮无修改系统设置权限。T-06到达时申请具体短窗口，说明备份/修改项/占用与恢复；许可前继续其他独立工作，依赖该腿的完整交付不放行。

记录原用户主题值与本次隔离AUTOOS_DESKTOP_CONFIG、system/follow状态；真实dark/light变更经过真实provider→宿主revision→clock/calc实际帧/applied，重启仍system policy。finally精确恢复原用户值、本次配置/进程；恢复失败报告并保可恢复备份，不归档称已净。provider注入只作控制，不替真实事件。Windows范围不外推Linux/Mac。

### 5.5 组合与交接

每次桌面启动/合入先运行现scripts/check-app-submodules.ps1，传-OsRoot指向候选；候选显式OS root/config/数据与全部注册集，不暗读主检出旧app。非目标仅初始化既定pin，不升级、不切脏子模块；保护054八pin及其他owner后续合法变更。

候选已提交clock SHA先验；review后正常push，OS干净组合fetch/detach到同SHA，健康门和实际启动后提交正式pin。候选通过不冒正式组合已交付。合入前按最新main重核只有本包clock/主题增量，不批量覆写manifest或其他产品。

036拥有包渠道/输入，037教学012不等产品clock，038只研究专用表面，918准入/审计规则照常；真实manifest变更由实际owner接续，不给永久豁免。新证据关闭G-1/G-2时刷新现Spec/债务/持有索引，保053原历史。

### 规范增量

| delta_id | add/modify/retire | docs/specs/... target | before/after rule | rationale | acceptance IDs |
|---|---|---|---|---|---|
| SD-01 | modify | docs/specs/shell/host-theme-policy-and-delivery.md | G-1/G-2待验→真实系统/载体闭环与失败/平台边界 | 具名义务实证关闭 | AC-02/03/05 |
| SD-02 | add | docs/specs/apps/clock-theme-adoption.md；产品拟docs/specs/theme-adoption.md | 正式包/default/follow/override/四tab/已验pin | 产品不等教学例 | AC-01/04/06 |
| SD-03 | modify | UI docs/specs/runtime/host-theme-transport.md（实际接缝） | 协议实现→真实载体采纳/状态/退出支持 | 唯一机制owner | AC-02/03 |
| SD-04 | modify | docs/specs/shell/host-theme-policy-and-delivery.md | 实际包渠道/后继产品入口与原限制读回，保054边界 | 可继续接入 | AC-06/07 |

frontmatter只列OS Spec，参与仓目标工作冻结；不写旧specs.json。

## 6. 测试设计

以下全部未执行，T-01冻结命令/资源，T-02..06使新增控制可执行。work负责实现/自查；独立上下文final review同组/同产品clone冻结验证。执行双端时读autoui-verifier，本轮不操作UI。

| Case ID | 不变量/输入 | 实际入口及可区分oracle | owner_stage |
|---|---|---|---|
| V-01 | AC-01；U036来源/默认/follow/override、坏包与恢复 | 真实安装工具启动clock Vue/VM，source/hash固定、非法拒绝保旧、恢复成功，无主检出兜底 | work |
| V-02 | AC-02；真实clock outproc attach/Hello/push | 进程/连接身份、clock帧/颜色和applied revision一致，helper/轮询不能满足 | work |
| V-03 | AC-02/03；断线/重连、重复乱序/旧session/stale、拒绝/恢复 | 真实进程重连/应用腿+现受控故障帧，宿主状态不倒退、拒绝保旧，关闭只回收本次资源 | work |
| V-04 | AC-03；相反选择clock/calc同场、关闭一方 | 真实scope/帧值与业务不串，另一App/Tick继续，follow追最新 | work |
| V-05 | AC-04；四tab完整原门中换肤/重连 | 原秒表/倒计时/世界时差/闹钟/storage实际oracle，不重置/倍速/重复触发，原条件保留 | work |
| V-06 | AC-05；获准真实OS事件、控制输入、system重启 | 真实设置→provider→宿主→clock/calc渲染/applied，policy仍system，原值恢复；未跑不PASS | work |
| V-07 | AC-01/03；包/权限/连接失败、移根/重试 | 具名失败保旧，恢复后真实再消费；910身份/锁/lease保持，无owned泄漏 | work |
| V-08 | AC-06/07；候选SHA/remote/pin/健康及非目标pin | 被测clock==正式产品SHA==OS pin，候选/正式分列，054与非目标保持 | work（候选）/merge（正式） |
| V-09 | AC-08；逐AC/真实事件/载体/恢复/遗漏/SD | 独立review绑定OS/UI/clock/tool SHA，G-1/G-2必需面未闭不可完整pass | review |
| V-10 | AC-09/10；冻结门/落地/知识/全输出 | 来源同一、canonical/债务持有按实证闭、guard与设置/进程/材料对账 | merge |

已核clock tests/desktop_mcp.py，其他原脚本T-01列完整适用清单；OS健康/desktop入口、053归档逻辑和UI theme_policy/desktop_protocol单测可复用。所有config/data/图像日志/外部安装/target显式scope，旧脚本temp默认须修，不用用户storage。原业务/容差/时序/样本/平台条件不减，未执行/首红/排除分列。

OS/.at资产本身不跑语言cargo t/docs_gen；UI Rust接缝实改才跑theme_policy/protocol/session/config/lifecycle及calc/B1必要消费者，UI t/tu/ti按真实触面，不全AG/ACE/四仓整档。final后一次冻结必要职责并集，复用未变有效证据；列准备/编译/运行/清理时间和真实执行数。

## 7. 验收标准

| ID | 可观察关闭条件 | owner_stage |
|---|---|---|
| AC-01 | 正式包/来源、独立Vue/VM default/follow/override与恢复实际通过，V-01/07 | work |
| AC-02 | 真实clock载体attach→push→帧后applied/重连/stale实证，G-2可闭，V-02/03 | work |
| AC-03 | 多App隔离/拒绝保旧/owned连接订阅退出和恢复正确，V-03/04/07 | work |
| AC-04 | 四tab原完整适用业务门，换肤不重置/重复Tick，V-05 | work |
| AC-05 | 获准真实OS事件/system恢复实际通过、原值精确恢复，G-1可闭，V-06 | work |
| AC-06 | 候选clock/036支持/053差额交接准确，健康/实际运行同候选SHA，V-08 | work |
| AC-07 | 054/037/038/918边界及非目标pin/业务保持，无通用新平台，V-08 | work |
| AC-08 | 独立final review逐AC/遗漏/恢复/SD pass，不借hold冒完整交付，V-09 | review |
| AC-09 | 冻结门/正常产品push/正式pin/参与仓落地、canonical/债务持有实证关闭、归档，V-08/10 | merge |
| AC-10 | 所有设置/输出/进程对账、摘要折Plan删scope/自有树、guard clean，产品clone保留，V-10 | merge |

## 8. 执行步骤

| ID | 动作与关闭结果 | 依赖 / AC | owner_stage |
|---|---|---|---|
| T-01 | 授权后唯一组/产品一写入者，冻结U053/036/源/tool/资源；真实载体到broker有界可行性/必要接缝决定 | U053已满足，授权待给；AC-01..07 | work |
| T-02 | U036后clock包/default/follow/override与独立来源接入、README反链/原测试定位 | T-01与U036；AC-01/04/06 | work |
| T-03 | 真实clock adopt/broker/Hello/主题结果、重连/stale必要接缝及反控 | T-02；AC-02/03 | work |
| T-04 | 四tab、clock/calc同场、standalone Vue/VM与真实载体、拒绝恢复完整并集 | T-03；AC-01..04，V-01..05/07 | work |
| T-05 | 产品候选提交/候选组合健康与真实运行、source及其他pin对账 | T-04；AC-06/07，V-08候选 | work |
| T-06 | 申请/等待具体窗口许可后真实系统事件，finally恢复原值/进程，G-1/G-2新证据 | T-04；AC-05，V-06 | work |
| T-07 | work AC/健康/遗漏/原门/SD与036/后继交接冻结；execution_done7/11 | T-05/06；AC-01..07 | work |
| T-08 | 同组/同clone独立上下文final review，finding回write owner/失效证据 | T-07；AC-08，V-09 | review |
| T-09 | 冻结门，review后正常产品push→干净组合fetch/detach/健康/精确pin，各仓落地 | T-08 pass；AC-09，V-08正式/10 | merge |
| T-10 | canonical/路线/053具名债务与持有实证关闭、后继交接/归档读回 | T-09；AC-09 | merge |
| T-11 | 全设置/输出/进程对账、摘要折Plan删scope/自有树产物，guard后清理；archived11/11 | T-10；AC-10 | merge |

材料唯一D:/autostack/plan-artifacts/plan-055/auto-os/，UI/clock按repo分区并登记owner/用途/在途。外部SDK/包/快照/源码副本/设置备份/截图日志/显式target与测试数据均在此；普通编译在组树，产品clone合法编译输出列账。merge核全部实际路径（含其他盘/安装/临时根），命令/退出码/关键值/源hash/恢复节选折Plan后删scope与自有树产物；具名未闭持有有owner/理由/关闭条件，G-1/G-2必需面不能转hold冒交付。未知/活动/他方不盲删，产品clone不删不reset。

## 9. 复审记录

2026-11-03 / stage:new / auto-os/PLAN-055 r1 / outcome:blocked（只调研立约；实施授权待给、包接入等U036、真实事件届时等窗口许可） / 已核053终态/G-1/G-2、054边界、clock source=gitlink且clean与四tab原测试；选择clock而非扩大八产品装配。未执行探针/项目测试/真实系统事件。next：授权后work/T-01；不借协议单测或standalone轮询宣称G-2闭。

静态预检pass：0–10章节、11T/10AC/10V/4SD唯一连续，work/review/merge职责及G-1/G-2真实oracle明确；055跨活动/归档唯一、OS号源056；本地合同/设计引用、限定diff/尾空格检查通过。未创建树/产品改动/运行测试，053两项仍未关闭。

## 10. 待澄清事项

1. 本轮没有系统设置窗口权限；T-06具体申请并等待许可，期间可完成无依赖任务，未许可不做该腿/不声明完整交付。
2. 真实clock生成/自采纳能力T-01须整链实证；需要新后端/协议/平台则具名前置或重规划，不换helper满足AC-02。
3. U036未满足不提前用草案接口发布产品；037/038/054不是整项前置，clock旧源码/053合法快照不被本轮动。
4. 产品clone/pin按OS AGENTS；有他方WIP/并发先归owner，不擅自stash/reset。真实资源/数据隔离T-01冻结，Windows范围不冒跨平台。
