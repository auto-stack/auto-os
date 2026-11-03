---
plan_id: PLAN-049
status: drafting
feature_name: 本地应用通信F0/F1与文本捕获最小闭环
author: [Codex]
created_at: 2026-10-16
updated_at: 2026-10-16
plan_revision: 1
current_step: 0
total_steps: 8
supersedes_spec_components: []
new_spec_components:
  - docs/specs/communication/project.md
  - docs/specs/communication/local-profile-v1.md
  - docs/specs/communication/broker-session.md
  - docs/specs/communication/capture-inbox-probe.md
touched_goals: []
affects: [communication, capture-inbox-probe]
---

# [AutoOS PLAN-049] 本地应用通信F0/F1与文本捕获最小闭环

## 0. 变更摘要

复用[既有通信RFC](../design/strategy/auto-app-communication-v0.6.md)与[F0–F5分期](../design/strategy/v0.6-foundations-roadmap.md)，按[Design02](../design/02-local-app-communication-f01.md)在auto-os新增独立Rust实验workspace。Windows本地命名管道连接broker、capture-service、caller三个真实进程，验证发现/短调用/结果、持久领域回执、超时和实例更替。首服务是专用文本捕获收件箱，不修改Notes或旧VM。

本轮仅起草；未授权实施、开worktree或运行功能测试。未来work须先取得本r1实施确认，再核对实际基线与附件指纹。

## 1. 目标

1. F0：可解释且严格的版本/Schema/有限JSON wire、身份/权限/错误及资源合同，配人工golden正反例。
2. F1：三个独立PID、真实IPC注册/发现/转发，返回可核对的capture/receipt；broker不内置领域服务。
3. 短调用截止/断线后保持确定未转发与结果未知的区别，旧引用/晚到结果不得串入新请求。
4. SQLite事务提供本地去重/持久回执，以独立读库证明重试没有新增记录；不泛化exactly-once。
5. 首例有受管进程、输入/队列/时间/输出配额及可重复展示，从零桌面/零语言依赖构建运行。

非目标：修改Auto语法/TaskRegistry/Value、Auto binding、真实Notes/Launcher或任一app接线、AI调用/MCP桥、F2任务取消/事件/stream、Linux/远程/ROS2/Batom v2、通用IDL生成器、生产broker自启/安装与完整OS权限系统；机器人算法排除。F0的未来扩展描述不代表已支持。

## 2. 架构方案

`caller --公开wire--> broker --独立provider连接--> capture-service --事务--> 专用SQLite`；回包按broker私有route回到原caller。core→数据/验证；local→core与命名管道；demo→core/local与SQLite；无反向依赖auto-lang/UI/man/AI/renderer。

broker拥有会话epoch、连接绑定principal、注册表、路由和有界资源；provider拥有数据库/领域去重；caller拥有显式发现/尝试与结果核对。重启后旧ServiceRef拒绝，显式发现后才取新实例。不把实验凭证当安装app身份，不把receipt存进broker响应cache。
设计的wire/语义单源为Design02 §§3–6，计划通过附件指纹绑定；若改变目标、预算、错误承诺或边界，须修订plan_revision及受影响验收，不让T-01生成第二份冲突规范。

## 3. 技术栈

Rust standalone workspace（新 `experimental/communication-f01/Cargo.toml`、Cargo.lock）；Tokio、serde/serde_json、rusqlite、必要Windows安全/Job API薄封装，版本由T-01记录与锁定。至少三个bin，名字拟为`comm-broker`、`capture-inbox`、`comm-client`。

新crate/module路径是拟建，不声称当前存在。现有 `scripts/new-plan.sh`仅取号；拟建 `scripts/verify-comm-049.ps1`用于协议/IPC/领域/生命周期/完整验收，`scripts/demo-comm-049.ps1`只展示已验证行为。不得安装工具/依赖服务、修改全局配置或拉起当前桌面；缺工具记infra阻断。

## 4. 需求分析与背景调查

授权：用户已选择先做通信最小闭环，要求复用设计、明确架构/首服务/语言/验收/拆分边界，起草计划，**先不写代码**。允许本轮只读相关仓并在主导auto-os写设计/计划/索引及取号簿记；不授权实施、创建agent/会话、推送或改变现有产品。用户未指定实现人/模型、时间/token预算或自动继续规则。

主导仓auto-os main取材HEAD `2b210066ef6855a5f584b118a020015b4146a3d7`，起始工作区干净，active/archive最大048、next049；mutex+独占allocation锁覆盖重扫/脚本/唯一性检查，已领取049、next050。auto-lang的路线文档提交`195f7650c`是讨论输入；其在途Plan/ledger改动不归本计划。取材文件SHA见[planning-inputs.json](attachments/049/planning-inputs.json)。

| 证据入口 | 观察与采用边界 |
|---|---|
| auto-os AGENTS.md、README、design/00-intro、scripts/new-plan.sh | OS产品/服务归本仓，主检出簿记与单计划worktree；首验无需app submodule |
| docs/specs/{apps/independent-repositories,shell/desktop-app-launch,shell/state-files}.md | 现行模块分散，无overview/project/goals及通用通信Spec；新的数据库位于独立实验目录，不改用户config写安全合同 |
| 通信RFC、system-devtools RFC、F0–F5 | 自有语义/外部桥、typed call/receipt、独立样板先行；不把长期全量范围装入F1 |
| auto-lang vm/task_system.rs | mpsc::Sender/Receiver<Value>与进程内TaskRegistry；不能序列化句柄扩成远程Task |
| auto-lang ui/desktop_protocol/{transport,broker}.rs | 现有命名管道/分帧及关闭IO教训，broker是桌面孵化；新应用服务不依赖/改其消息枚举 |
| auto-ai-daemon Cargo.toml | Rust异步服务已有生态经验；不引入其配置/模型服务作为F1前置 |
| auto-os scripts/smoke-037-desktop-back.sh、auto-lang experimental/ac-core/src/link.rs | 前者启动重桌面且直接kill不适合新最小实验；后者私有JobGuard/SUSPENDED-before-assign经验可复用，不能依赖整个ac-core拉入Cranelift |
| apps/015-notes | 本次未见可读docs，真实app协议未知；用独立capture服务，后续adapter另勘 |

本轮不读取无关计划历史；未找到匹配的活跃通信Plan。当前Spec缺口由SD-01–04补齐，起草不写canonical Spec。系统协议是跨消费者新合同，先有Design02再立本Plan，不分配auto-lang编号或将其算语言实现任务。

## 5. 详细设计

### 5.1 F0/F1必须保持的规则

- 依Design02：64KiB帧上限、16层容器、有限JSON profile；先admission再分配/handler，重复键/未知字段/版本与不支持类型明确拒绝；head/body错误和资源错误分开。
- 两种尝试身份：request_id用于关联，principal+operation_id用于领域去重；ServiceRef绑定broker_epoch与provider_instance。body伪造principal/app_id不能改变可信身份，provider回包不能指定其他caller。
- broker转发前验证权限/Schema/引用并扣预算；进入转发即保守may_have_run，timeout/断线不能伪报确定失败；不自动重放写操作，晚到结果丢弃且不可串请求。
- capture保存原文字与来源元数据；源码URI不被执行/读取；note与receipt同事务，成功回复在commit之后。相同key/payload返回同capture_id，不同payload冲突；满库仍允许旧key查询/重试；坏库不重置。
- 实验pipe/数据目录隔离；显式DACL、remote拒绝、凭证policy与角色限制真实验证；旧桌面、用户文件、app配置和真实令牌不作测试输入。

### 5.2 新路径与实施前决策

拟建workspace的core/src/{protocol,codec,admission,error}.rs、local/src/{transport,session,registry,route,client}.rs、demo/src/{capture,store}.rs及bin；tests/{f0_contract,f1_route,capture_receipt,lifecycle,resources}.rs和fixtures。名称可等价调整，不能因此隐藏不完整路径。

T-01是必经全路径可行性门，先验证3进程一条capture/receipt、DACL/坏token、SQLite重启与完整回收，再冻结Schema字段/golden、ID/可选字段规范、SQL设置、版本/lock、预算实现与测试support复用决定。通过前不得平铺整个引擎。若不能满足合同，给出反例和needs_replan，不删回执/安全/预算或改成HTTP/同进程fallback。
固定数据配额、时间/队列/输出上限见Design02 §6；本轮数字为拟议首验profile，不是用户给定性能承诺。T-01只选择可满足本r1的方法，改变这些限额须修订计划并呈现。

### 规范增量

| delta_id | add/modify/retire | docs/specs/... target | before/after rule | rationale | acceptance IDs |
|---|---|---|---|---|---|
| SD-01 | add | docs/specs/communication/project.md | 无通用通信现行模块→独立F0/F1支持集、包图、入口与非目标 | 不从桌面协议推断已有系统总线 | AC-01,AC-02,AC-09 |
| SD-02 | add | docs/specs/communication/local-profile-v1.md | 无冻结wire→窄JSON/framing/版本/错误/预算与golden；注明实验版本 | 后续SDK可复用且不依赖Value | AC-03,AC-04,AC-08 |
| SD-03 | add | docs/specs/communication/broker-session.md | 无服务会话合同→principal/epoch/实例/路由/截止/断线与接入控制 | 不串引用、不伪报副作用状态 | AC-02,AC-04,AC-05,AC-08 |
| SD-04 | add | docs/specs/communication/capture-inbox-probe.md | 无样板→私有DB、幂等事务/receipt、恢复与配额；明确非Notes产品 | 可核对真实领域效果与局限 | AC-06,AC-07,AC-09 |

## 6. 测试设计

所有下列新命令/用例尚不存在、未执行；由T-01/T-02建立入口与判据。实现人负责定向测试；独立最终review在同一权威worktree、独立上下文核验，不修代码。用户未指定角色/模型，本轮不创建agent或发消息。merge只负责冻结门与归档，不接收未完成正确性。

| Case ID | AC/不变量 | 输入/状态 | entrypoint（拟建） | 区分错误实现的oracle | work owner/边界 |
|---|---|---|---|---|---|
| V-01 | AC-01/02 | 从空目录起3bin，未启动AutoOS/auto | f1_route + demo脚本 | 三个不同PID；provider实际收到trace，DB在provider目录；broker进程不存在capture handler | T-01/T-03；真实Windows路径 |
| V-02 | AC-03 | 手写合法golden、UTF-8中文/换行、拆/粘帧 | f0_contract + 原始wire fixture入口 | 已核定字段/值保真；非仅共享encoder→decoder往返 | T-02；有限profile |
| V-03 | AC-03/08 | 长度0/65536/65537、残帧/坏UTF-8、深度16/17、dup key/unknown/type溢出 | f0_contract/resources | shape/resource分类正确，超长头拒绝前不分配payload/不触handler；临界合法控制仍过 | T-02/T-05；各公开reader一致 |
| V-04 | AC-03/04 | 错protocol/codec/schema、坏token/正确token无角色；伪造principal | 原始IPC客户端+f1_route | 业务前拒绝；handler计数与DB均无变化；body不能提权；日志无凭证 | T-02/T-03 |
| V-05 | AC-04 | 实际监听handle/同名第二broker/remote选项 | Windows transport tests | DACL只有允许SID、first instance失败、remote拒绝位；不以只测应用token代替ACL | T-01/T-05；无第二账号不冒称跨用户攻击实测 |
| V-06 | AC-02/05 | provider断线、重连、重复注册；broker重启 | lifecycle | 原注册被撤销；新instance/epoch；旧ref拒绝，显式discover后成功 | T-03/T-05 |
| V-07 | AC-05 | 前置队列超时、转发后timeout/迟到reply、复用活动request_id | f1_route/lifecycle故障点 | not_dispatched与may_have_run区分；晚结果不串新request；已转发写不自动重放 | T-03/T-05 |
| V-08 | AC-02/04 | 两caller并发、provider伪造route/reply | f1_route | 每个caller只收到自身结果；伪造route拒绝/关闭，不能吞成其他request成功 | T-03/T-05 |
| V-09 | AC-06 | 同key顺序/并发重试、同key不同payload、不同principal | capture_receipt | 同capture_id且独立SQLite读库仅一行；冲突不覆盖，跨principal查询拒绝 | T-04；实际跨进程与落盘 |
| V-10 | AC-06/07 | kill-before-commit；commit-after-before-reply；重启 | capture_receipt/lifecycle明确barrier | 前者无半条note/receipt；后者查询committed且重试不增行；杀进程时刻由barrier证据确定 | T-04/T-05；不拿sleep猜事务阶段 |
| V-11 | AC-06/07/08 | 只读/坏DB、第二writer、1024/1025条；满库旧key查询/重试 | capture_receipt/resources | 注册或写失败明确；原数据保留；旧key继续可用，坏库不变空库 | T-04/T-05 |
| V-12 | AC-08 | 超连接/在途/队列、slow reader/不发完Hello/残帧 | resources/lifecycle | 配额拒绝或有界超时；其他合法连接恢复；不无界spawn/内存增长 | T-03/T-05 |
| V-13 | AC-08 | 正常退出/启动失败/timeout/子孙持stdout管道/containment失败 | 最小test support/lifecycle | 实际全树PID退出、pipe/锁可再用、IO读者回收；失败不降级裸跑 | T-01/T-05；只管理自己进程/目录 |
| V-14 | AC-09 | 50条1KiB，两caller各25操作；再跑同目录 | acceptance/demo + 独立读库 | receipt/内容/来源/行数与预期一致，重复运行不制造数据；报告时延/资源和失败项 | T-06；不是性能优越性结论 |

验证入口拟为 `pwsh -NoProfile -File scripts/verify-comm-049.ps1 -Scope <scope>`；scope取admission、routing、receipt、lifecycle、resources或acceptance之一（T-01定义实际CLI，后续不复制独立decoder）；稳定测试family对应上表。领域oracle由人工expected与Python标准库sqlite3在service停止后只读核对，不以服务自身的“成功”JSON作为唯一判据。补充反例为必验，固定golden不能替代V-06..13。
健康检查拟为 `cargo fmt --manifest-path experimental/communication-f01/Cargo.toml --all -- --check` 与 `cargo check --manifest-path experimental/communication-f01/Cargo.toml --locked`；均在实施树运行，当前尚无manifest，未执行。
有限测试输入的wall/队列配额不是承诺无限输入。未支持Linux、网络对手、全Atom、真实app/Auto binding、掉电耐久性和任意外部副作用exactly-once；kill进程只证明所声明SQLite故障模型。

## 7. 验收标准

| ID | owner_stage | 可观察结果/方法 |
|---|---|---|
| AC-01 | work | 新standalone workspace锁定依赖、零auto-lang/UI/man/Cranelift闭包，独立构建；metadata/包图+V-01 |
| AC-02 | work | 3真实进程注册/发现/调用/路由；双caller不串包、重复注册策略与失效正确；V-01/06/08 |
| AC-03 | work | F0严格版本/framing/有限Schema与人工golden；所有读入口有相同admission；V-02/03/04 |
| AC-04 | work | 实际DACL、remote/first-instance、凭证角色和可信principal/route控制；V-04/05/08 |
| AC-05 | work | 截止/断线/epoch更替不重路由、不自动重放；may_have_run/迟到结果正确；V-06/07 |
| AC-06 | work | capture/receipt原值保真、事务/持久去重、冲突/主体隔离，独立查库成立；V-09/10/11 |
| AC-07 | work | crash前后、坏库/只读/多writer不丢原数据/不伪报成功；V-10/11 |
| AC-08 | work | Design02首验限额与完整进程/IO/pipe/锁回收成立；V-03/05/11/12/13；超限失败明确 |
| AC-09 | work | 可重复CLI展示、50条工作负载、阶段/延迟/资源报告及明确未覆盖面；V-14，并保持禁止写面零变化 |
| AC-10 | review | 在冻结代码/依赖/配置上独立核验AC-01..09与SD-01..04、必验反例、fmt/告警/遗漏债；pass标明merge-owned门pending |
| AC-11 | merge | final review后冻结提交跑独立workspace完整门、必要复验；Spec/ledger/索引沉淀、归档与guard清理有真实收据；没有auto-lang Rust改动时不跑其cargo t/docs_gen |

## 8. 执行步骤

唯一实施树拟为 `D:/autostack/.wt/os-049/auto-os`，分支`plan-049-dev`；尚未创建。work先确认主导仓/实际基面/依赖与本r1授权，主检出只记计划进度。同一树依次实施/独立复审/merge；无junction/symlink，移除前wt-guard。跨仓均只读，不建语言兄弟树或改app submodule。

| ID | owner_stage | 依赖 | 写面/结果 | AC与定向验证（拟建命令，结果应通过） |
|---|---|---|---|---|
| T-01 | work | r1实施确认 | 新workspace/lock、最小3bin、test support与attachments/049/contract-decision.md；通过全路径门并冻结profile细节/复用决定 | AC-01/02/04/06/08；`pwsh -NoProfile -File scripts/verify-comm-049.ps1 -Scope admission`首验含V-01/04/05/10/13最小腿；不成立先回规划 |
| T-02 | work | T-01 | core DTO/codec/admission、手写fixtures/descriptor，形成严格F0 | AC-03/08；`cargo test --manifest-path experimental/communication-f01/Cargo.toml --locked -p autoos-comm-core`，V-02/03/04 |
| T-03 | work | T-02 | local transport/session/registry/route/client，Hello/register/discover/call/reply与配额/失效 | AC-02/04/05/08；`pwsh -NoProfile -File scripts/verify-comm-049.ps1 -Scope routing`，V-04/06/07/08/12 |
| T-04 | work | T-03 | demo capture/store、SQLite事务和receipt查询、故障barrier；只用私有数据 | AC-06/07；`pwsh -NoProfile -File scripts/verify-comm-049.ps1 -Scope receipt`，V-09/10/11与独立读库 |
| T-05 | work | T-04 | 有界IO/测试收尾、资源与综合故障反例，不造通用管理框架 | AC-04/05/07/08；`pwsh -NoProfile -File scripts/verify-comm-049.ps1 -Scope lifecycle`及`-Scope resources`，V-03/05..13受影响组合 |
| T-06 | work | T-05 | demo脚本、README/支持矩阵、完整验收报告/身份、拟议Spec delta；只将work项完成送review | AC-01..09；`pwsh -NoProfile -File scripts/verify-comm-049.ps1 -Scope acceptance`及`pwsh -NoProfile -File scripts/demo-comm-049.ps1`重复运行；全部V-01..14无遗漏；fmt/check见§6 |
| T-07 | review | T-06、写入冻结 | 独立review真实代码/反例/SD/债与健康；不修实现，不合入 | AC-10；记录reviewed commit/依赖/输入指纹，pass仍保留AC-11 pending |
| T-08 | merge | T-07 pass | 同冻结提交完整workspace门、沉淀SD/ledger/索引、合入归档与guard清理；变代码/依赖须重验受影响review | AC-11；`cargo test --workspace --manifest-path experimental/communication-f01/Cargo.toml --locked`及冻结验收重放/合法复用，禁止无关lang整档 |

开发只做当前必要check/定向反例；不因单条finding或提交反复跑整档。冻结merge门只覆盖新独立workspace；若实现发现必须改lang/UI/man或用户数据，先记录边界冲突并修订，不能沿用本计划许可扩大写面。

## 9. 复审记录

### 起草交接 r1（2026-10-16）

`stage: new`；`plan: auto-os/PLAN-049`；`revision: 1`；`outcome: pass`（合同已具备供实施确认的范围，非代码验收）；`next: 用户确认r1后 /auto-plan:work`。本轮无实现授权，不运行T-01，不新建代理/会话/实施树。
已审：复用来源/主导仓、无活跃匹配Plan/唯一取号、8个任务与owner、AC-01..11/SD-01..04/V-01..14对应、设计单源与未决项、资源/事务/生命周期反例和禁止写面。规划输入指纹/静态检查见attachments/049；新增命令均为未来入口。
后续发现或语义修订在此保留历史，不通过删除验收换通过。最终独立review、冻结门、merge和清理均未执行。

## 10. 待澄清事项

没有必须阻断起草的用户信息缺失。建议选择已在合同中明确：Windows首验、Rust、命名管道、3进程、独立capture服务与专用SQLite；用户确认实施时即确认本r1范围和拟议profile预算。
T-01拥有实际版本/SQL设置/可选字段golden、资源策略及独立测试helper可行性的有界决策；不允许改变首服务/持久性/接入边界或删强制反例。未来跨平台、Auto binding、F2任务/事件/流、Notes/Launcher及ROS2各另立阶段，机器人算法在通信框架之后再议。
