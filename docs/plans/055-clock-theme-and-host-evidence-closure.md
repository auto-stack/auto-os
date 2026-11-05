---
plan_id: PLAN-055
status: executing
feature_name: auto-clock正式主题接入与宿主真实事件及跨进程验收收口
author: [Codex]
created_at: 2026-11-03
updated_at: 2026-11-05
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

2026-11-05用户 `/goal 完成计划055` = 全链授权（work→review→有界修复→merge，沿 auto-plan-goal；含建树/产品clone开发/构建/真实运行）。U036=auto-ui/PLAN-036 已 archived 10/10，T-02 前置已满足；真实OS事件腿（T-06）仍须届时单独窗口许可。

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

### T-04 VM 轨四 tab oracle 全绿 + 产品候选落（2026-11-05，work 执行中）

- **四 tab oracle 15/15 全绿**（`plan-artifacts/plan-055/auto-os/fourtab-vm.log`）：T0 表盘（含 dial_mode 切换）/T1 秒表（走表≥1000ms、MM:SS.cc、计圈、复位）/T2 计时器（1s 到零 banner、Dismiss）/T3 世界时钟（8 城、北京/伦敦 ±60s 原容差，b=790 l=310）/T4 闹钟入列+storage 跨重启——**主题采纳落地后的完整原业务门，原条件未放宽**。组工具链（AUTO_BIN=组 auto.exe + 组 tool），`tests/desktop_mcp.py` 就绪判定修为状态非空（MCP 端口先于首帧 view 同步打开——空 State 竞态株）。
- **产品候选提交**：auto-clock main `3495671`（= pac.at dep themes + app.at use/design + README 反链 + 测试修 + vendor/themes 快照 + .gitignore deps/）；树 clean；**push 留待 review 后**（T-09）。独立默认腿（Auto/light 无宿主）由本 oracle 即证（VM 轨无宿主 env = design 默认生效，业务/渲染双绿）。
- 待办：载体 exe MCP 四 tab 腿、clock/calc 同场腿、T-05 组合健康/pin。

### T-03 真载体整链实证（2026-11-05，work 执行中）

**G-2 四腿之三条真腿已通**（隔离桌面 probe，desktop-9 + clock exe，日志 `plan-artifacts/plan-055/auto-os/probe/{desktop-9,clock-13,clock-14}.log`）：
- **attach 推送腿**：`[PLAN-053] adopt attach OK: Some("012-clock") caps=["theme-policy/1"]` → push rev=1 family=Auto mode=dark → `outproc 主题策略结果 app=012-clock rev=1 status=applied`（宿主日志行）+ 载体端 `[native-theme] push rev=1 …` 应用行。
- **变更广播腿**：probe config 外写 mode=dark→light → 宿主 poll 提交 → 广播 push rev=2 → `rev=2 status=applied`。
- **重连腿**：杀时钟重连 → attach 即推**最新**快照 rev=2（非旧 rev）→ applied。
- 载体面：a2r exe 嵌入官方包（owner/package/version/digest/families/themes.at 原文）→ `NativeThemeCarrier` 校验（family/version/digest/mode，具名拒绝保旧）→ `set_theme_tree`+`set_dark_mode` 应用 → 帧后 applied 读回。

**过程中修的三层根因**（均组 auto-ui 树，分支 `codex/plan-055-clock-theme-ui`）：
1. **协议帧歧义（PLAN-053 遗留真 bug，408af4fd 后第二个 UI 提交）**：Hello 双"缺席=不写"尾段（v1.11 surfaces + PLAN-053 theme_caps）——surfaces 空而 caps 非空时，解码端把 caps 段长度前缀当 surfaces 段读 → TooShort。修复=任一尾段在场则 surfaces 段恒显式（计数可0），双空旧端线字节不变；回归 `hello_trailing_segments_caps_without_surfaces_round_trip`。
2. **HeadlessFrameSource 缺主题转发**：remote/v2 臂是 a2r 载体主产线，缺省 theme_capable=false 压掉 caps 门（`caps=[]` 株）→ 补三方法转发。
3. **broker 管道 env 覆盖**（`AUTOOS_BROKER_PIPE`）：同名多实例服务端使客户端连入任意桌面实例（含用户桌面/僵尸桌面）——probe 隔离必需；无 env 零行为差。**probe 教训：管道名必须每次启动唯一**。

**未竟腿**：无——T-03 全部收口（2026-11-05）。载体端拒绝路径单测 3 绿（四拒绝臂+应用/帧翻+裸包装）；stale 判定锚 session 级单测绿（迟到 rev<推送 → stale 行不覆盖 / 同 rev 原样落行，受控注入常驻回归）。坏 family 外写实腿已通（见下）。probe 诊断打点（`[probe-broker]/[probe-client]` eprintln、LAST_BAD_FRAME static、carrier 注册/推送行）为临时件，收口前清理为常驻语义或摘除。

**拒绝/恢复真腿（desktop-10 + clock-15）**：外写 `theme_family: "NoSuchFamily"` → 宿主具名拒绝、**零提交零推送**（无 rev=3、config 不回写、载体保持 rev=2 保旧）→ 再写合法 Auto/dark → **rev=3 applied**（恢复后下一次实际操作成功，V-07 恢复语义）。`__theme_status` 具名面 MCP 读回未走通（9304 连接拒绝——待查 MCP bind），具名拒绝面由 053 单测+Spec 承担，probe 补充为状态不变形证。

**运行环境事故（如实记）**：probe 期间 ①D 盘曾 100% 满（本计划隔离 target 42G，cargo clean 后释放 41G）；②`taskkill //IM ui-desktop.exe` 误杀了用户自启桌面（PID 23968，10:00:38 启动）——配置/存储未触碰，需用户自行重启其桌面；③另一会话的 ace-native 测试负载与本 probe 并行（未触碰其进程）。



### T-01 冻结与组建立（2026-11-05，work 执行中）

- **授权**：`/goal 完成计划055`（auto-plan-goal 全链：work/review/有界修复/merge）。
- **组**：`.wt/os-055/`——auto-os=`codex/plan-055-clock-theme`@517779b（base=main HEAD）；auto-ui=`codex/plan-055-clock-theme-ui`@b3414750（沿 053 `-ui` 后缀先例）；auto-lang=detach@70b4a1de7、auto-man=detach@03bb65e（只读兄弟源，Cargo path 依赖要求同级布局；lang 主检出有他方 docs WIP 不涉及）。产品 clone D:/autostack/auto-clock main@551d05e clean（=apps gitlink，单写入者）。
- **U036**：=auto-ui/PLAN-036（正式主题包发行与生成输入收口）已 **archived 10/10**（实证 os `docs/specs/apps/product-theme-batch-059.md:31`、archive/058/059 收据）；残余「U036 SDK 通道待办」owner=Man 发行，batch-057 定性非本类阻塞。
- **053 终态**：archived 10/10 @os d9de141；G-1=真实OS事件 hold（owner user/merge，closure=授权窗口）；G-2=OS053-OUTPROC-CARRIER（KNOWN-DEBT OS053-OUTPROC-CARRIER 行：闭=真载体 probe-t05 四腿 attach推/applied回读/重连重推/stale分列；053 时"自采纳Hello未达broker"因无 outproc 二进制）。
- **载体机制（auto-ui 主检出 b3414750 冻结）**：broker=ui-desktop 进程内 DesktopSession（pipe `autodesk-broker`）；Hello 尾段 `theme_caps`（`theme-policy/1`）；帧17/18=`ThemePolicyPush{revision,record}`/`ThemePolicyResult{revision,status,reason}`，record=`rev␟family␟version␟digest␟mode`；attach/重连/commit 三路推全快照；stale 锚=`theme_pushed_rev`（ack rev < 推送 rev）；宿主状态面=`theme_outproc_status` 行 + `__theme_status` + `[PLAN-053] outproc 主题策略结果` 日志行。真实 exe 门：`--render-mode=desktop --desktop-endpoint=<pipe>`（缺端点具名报错）。
- **clock**：pac render=vue、front_port 17812、无 daemon；业务=单 `src/front/app.at` 四tab（sw_*、t_*、cities 8+offsets、alarms storage `clock.alarms.N`、Tick 250ms）；`tests/desktop_mcp.py`=VM轨 oracle（MCP 9257，storage 隔离 TEMP/pid，AUTO_BIN 可覆写）；现无任何 theme/包消费代码。
- **正式包渠道**：`auto-ui/packages/themes` v0.2.0（owner=auto-ui schema=1，families Auto/Solarized）；产品采纳先例（calc）=`vendor/themes` 快照（`publish_theme_package_snapshot`）+ `dep themes{path:"vendor/themes"}` + `use themes.themes`；宿主 catalog 注入序 `AUTOOS_THEME_PACKAGE_DIR`→`AUTO_UI_ROOT/packages/themes`；配置 env `AUTOOS_DESKTOP_CONFIG`。
- **manifest**：28 active；auto-clock 行=id `012-clock`、ports[17812]、无 daemon。
- **构建 scope**：`CARGO_TARGET_DIR=.wt/os-055/auto-os/.auto/desktop-runtime/target`（隔离，三产物 auto/auto-ui-tool/ui-desktop）；组树构建日志 `plan-artifacts/plan-055/auto-os/build-t01.log`。
- **主检出他方 WIP（不动不收编）**：`apps/auto-charts` gitlink 前移（CHARTS-001）、docs/design+docs/research 若干修改、未跟踪 plans 055/060/061 与 research 文件。


2026-11-03 / stage:new / auto-os/PLAN-055 r1 / outcome:blocked（只调研立约；实施授权待给、包接入等U036、真实事件届时等窗口许可） / 已核053终态/G-1/G-2、054边界、clock source=gitlink且clean与四tab原测试；选择clock而非扩大八产品装配。未执行探针/项目测试/真实系统事件。next：授权后work/T-01；不借协议单测或standalone轮询宣称G-2闭。

静态预检pass：0–10章节、11T/10AC/10V/4SD唯一连续，work/review/merge职责及G-1/G-2真实oracle明确；055跨活动/归档唯一、OS号源056；本地合同/设计引用、限定diff/尾空格检查通过。未创建树/产品改动/运行测试，053两项仍未关闭。

## 10. 待澄清事项

1. 本轮没有系统设置窗口权限；T-06具体申请并等待许可，期间可完成无依赖任务，未许可不做该腿/不声明完整交付。
2. 真实clock生成/自采纳能力T-01须整链实证；需要新后端/协议/平台则具名前置或重规划，不换helper满足AC-02。
3. U036未满足不提前用草案接口发布产品；037/038/054不是整项前置，clock旧源码/053合法快照不被本轮动。
4. 产品clone/pin按OS AGENTS；有他方WIP/并发先归owner，不擅自stash/reset。真实资源/数据隔离T-01冻结，Windows范围不冒跨平台。
