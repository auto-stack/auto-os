---
plan_id: PLAN-049
status: archived
feature_name: 本地应用通信F0/F1与文本捕获最小闭环
author: [Codex]
created_at: 2026-10-16
updated_at: 2026-10-16
plan_revision: 1
current_step: 8
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
| T-01 | work | r1实施确认 | 新workspace/lock、最小3bin、test support与contract-decision.md；通过全路径门并冻结profile细节/复用决定 | AC-01/02/04/06/08；`pwsh -NoProfile -File scripts/verify-comm-049.ps1 -Scope admission`首验含V-01/04/05/10/13最小腿；不成立先回规划 |
| T-02 | work | T-01 | core DTO/codec/admission、手写fixtures/descriptor，形成严格F0 | AC-03/08；`cargo test --manifest-path experimental/communication-f01/Cargo.toml --locked -p autoos-comm-core`，V-02/03/04 |
| T-03 | work | T-02 | local transport/session/registry/route/client，Hello/register/discover/call/reply与配额/失效 | AC-02/04/05/08；`pwsh -NoProfile -File scripts/verify-comm-049.ps1 -Scope routing`，V-04/06/07/08/12 |
| T-04 | work | T-03 | demo capture/store、SQLite事务和receipt查询、故障barrier；只用私有数据 | AC-06/07；`pwsh -NoProfile -File scripts/verify-comm-049.ps1 -Scope receipt`，V-09/10/11与独立读库 |
| T-05 | work | T-04 | 有界IO/测试收尾、资源与综合故障反例，不造通用管理框架 | AC-04/05/07/08；`pwsh -NoProfile -File scripts/verify-comm-049.ps1 -Scope lifecycle`及`-Scope resources`，V-03/05..13受影响组合 |
| T-06 | work | T-05 | demo脚本、README/支持矩阵、完整验收报告/身份、拟议Spec delta；只将work项完成送review | AC-01..09；`pwsh -NoProfile -File scripts/verify-comm-049.ps1 -Scope acceptance`及`pwsh -NoProfile -File scripts/demo-comm-049.ps1`重复运行；全部V-01..14无遗漏；fmt/check见§6 |
| T-07 | review | T-06、写入冻结 | 独立review真实代码/反例/SD/债与健康；不修实现，不合入 | AC-10；记录reviewed commit/依赖/输入指纹，pass仍保留AC-11 pending |
| T-08 | merge | T-07 pass | 同冻结提交完整workspace门、沉淀SD/ledger/索引、合入归档与guard清理；变代码/依赖须重验受影响review | AC-11；`cargo test --workspace --manifest-path experimental/communication-f01/Cargo.toml --locked`及冻结验收重放/合法复用，禁止无关lang整档 |

### 执行进度（work 阶段追加）

- **T-01 ✅ 已完成（2026-11-04，commit 4ddf251，worktree `.wt/os-049/auto-os`，branch `plan-049-dev`）**：
  独立workspace `experimental/communication-f01/`（core/local/demo，3 bin：comm-broker/capture-inbox/comm-client），
  Cargo.lock 冻结（tokio 1.54.1/rusqlite 0.32.1 bundled/windows-sys 0.59.0，见
  plan-artifacts/plan-049/contract-decision.md §1）。全路径门
  `cargo test -p autoos-comm-demo --locked --test lifecycle`（t01：3真实PID一条capture/receipt、
  实handle DACL oracle 恰两条ACE（用户+SYSTEM）、坏token拒绝、broker重启新epoch+旧ref stale拒绝+
  receipt持久、同key重试同capture_id、独立只读库对账、stop-flag受管退出）全过（0.81s）。
  冻结决定：op族/allowed-key表、digest与capture_id推导、receipt显式principal跨主体拒绝、
  Pending/Forwarded→not_dispatched/may_have_run 判定、WAL+FULL+生命周期文件锁、
  显式DACL（ConvertSidToStringSidW oracle）、SUSPENDED→assign→resume 遏制。goal授权链：用户
  `/goal 完成计划049` = r1实施确认，继承 work→review→repair→merge 全程。
- **T-02 ✅ 已完成（commit 7d48ef6）**：core/tests/f0_contract.rs 16 测试——字节级人工 golden
  双向核对（hello/call/reply/error/discover/pcall，UTF-8 中文与换行保真）、0/65536/65537 帧分类、
  坏UTF-8/断JSON/尾随、重复键（含嵌套）、未知字段/op、深度16合法/17 resource、非u32数字形态、
  类型违例、capture/receipt body 边界、error 往返；local frame_io 6 测试（单字节撕裂重组、粘帧、
  截断有界、2s残帧截止、帧间空闲不受限）。修正缺陷2则并固化回归：read_frame 部分头覆盖首字节、
  深度按容器层计数。AC-03/08。
- **T-03 ✅ 已完成（commit b799445）**：demo/tests/f1_route.rs 6 测试 + common/mod.rs 共享harness
  （RawProvider 故障注入）：V-04 权限/伪造主体/日志零凭证；V-06 抢占拒绝/同连接幂等/descriptor
  变更拒/断线易主新instance/旧ref stale；V-07 Forwarded→may_have_run、晚到结果永不入新请求、
  活动 request_id 拒绝、无自动重放；V-08 双caller逆序应答各取其身、caller发provider向帧被断开、
  伪造route不吞请求；V-12 连接帽第9连接 resource_exhausted。修正缺陷：client pending表重复insert
  顶掉在途关联。AC-02/04/05/08。
- **T-04 ✅ 已完成（commit 2af04d5）**：demo/tests/capture_receipt.rs 4 测试（真实provider+SQLite）：
  V-09 顺序+并发同id/独立只读库单行/conflict不覆盖/跨主体显式拒绝/not_observed；V-10a barrier标记
  证据+kill-before-commit 零行；V-10b commit后kill回执持久+重试同id；V-11 第二writer exit(3)先于
  注册、坏库不重建、只读拒绝、容量1024新key拒绝旧key可用。AC-06/07。
- **T-05 ✅ 已完成（commit ddb2ff9）**：resources.rs 2 测试（V-05 真句柄DACL={user,SYSTEM}/
  reject-remote位/first-instance拒绝；V-12 8/16配额 not_dispatched）+ lifecycle 新增3测试
  （V-13 孙进程持stdout被job收割、启动失败不裸跑、job drop可回收、定名pipe与inbox.lock可复用）。
  修正缺陷：process_gone 缺 SYNCHRONIZE；JobGuard 补 Drop（PID枚举+TerminateJobObject+
  kill-on-close）。AC-04/05/07/08。
- **T-06 ✅ 已完成（commit 141d6ed）**：`scripts/verify-comm-049.ps1`（六scope统一验证入口）+
  `scripts/demo-comm-049.ps1`（job-contained CLI演示：50×1KiB双caller批次、p50/p95事实、
  python stdlib sqlite3 只读对账、同目录幂等重跑——run1/run2 均 50行/51200字节/全1KiB）+
  acceptance::v14 负载测试 + README + SD-01..04 proposed（docs/specs/communication/，
  worktree内备置，随merge发布）。**验收门：verify-comm-049.ps1 -Scope acceptance 全绿
  （7族39测试）**，fmt/check 0告警。AC-01..09 全部有测试/命令证据——完整证据图见
  plan-artifacts/plan-049/acceptance-report.md。AC-10/AC-11 保持 pending（review/merge所有）。

开发只做当前必要check/定向反例；不因单条finding或提交反复跑整档。冻结merge门只覆盖新独立workspace；若实现发现必须改lang/UI/man或用户数据，先记录边界冲突并修订，不能沿用本计划许可扩大写面。

## 9. 复审记录

### work 交接 r1（2026-11-05）

`stage: work` | `plan_id: PLAN-049` | `plan_revision: 1` | `outcome: pass` | `status: execution_done`
worktree: `D:/autostack/.wt/os-049/auto-os` | branch: `plan-049-dev` | base_commit: `831734b`
code_commits: `4ddf251..141d6ed`（T-01..T-06，工作树干净）| dependency_revisions:
Cargo.lock 冻结（tokio 1.54.1 / serde 1.0.229 / serde_json 1.0.154 / rusqlite 0.32.1 bundled /
sha2 0.10.9 / getrandom 0.2.17 / windows-sys 0.59.0）| task_ids: T-01..T-06 | next_responsible_role: review（T-07，`review_scope: final`）

就绪记录：
- work-owned AC-01..09 全部有绑定测试/命令证据（证据图：
  `D:/autostack/plan-artifacts/plan-049/acceptance-report.md`；冻结决定：
  `D:/autostack/plan-artifacts/plan-049/contract-decision.md`）。
- 验收门：`pwsh -NoProfile -File scripts/verify-comm-049.ps1 -Scope acceptance`
  —— 7 族 39 测试全绿；`cargo fmt --all -- --check` 等价（fmt 已跑，0 diff）、
  `cargo check --workspace --locked` 0 告警。
- 未决 in-contract findings：无。未决合同决定：无（T-01 有界决定已冻结并留痕）。
- 已修实现缺陷5则均有回归测试（见 acceptance-report §已修正实现缺陷）。
- 已知边界（非缺陷，已声明）：demo脚本 assign-after-start 微秒窗口、
  无第二账号不做跨用户攻击实测、kill进程级SQLite故障模型——与 Design02/计划
  声明一致。
- 统一回归（冻结后完整门）pending，owner：review/merge（AC-10/AC-11），
  不阻塞本次 work-complete 交接。
- Spec delta：SD-01..04 已在 worktree `docs/specs/communication/` 备置
  （proposed），发布归 merge。

### review final r1（2026-11-05）

`stage: review | review_scope: final | reviewer_identity: 同会话复核（限制：与work同上下文；按技能要求以独立重执行+代码审读+变异探针重建结论，不采信执行摘要）| plan_id: PLAN-049 | plan_revision: 1 | outcome: needs_fix | worktree_path: D:/autostack/.wt/os-049/auto-os | branch: plan-049-dev | reviewed_commit: 141d6ed7421bdc375fac10d4d965444e66f93cf8 | base_commit: 831734b | dependency_revisions: Cargo.lock 冻结（同work交接）| spec_inputs: docs/specs/communication/{project,local-profile-v1,broker-session,capture-inbox-probe}.md（worktree内proposed，SD-01..04）| next: work（F-1修复后目标复review）

独立核验（@141d6ed，树干净）：
- 验收门新鲜重执行 ×2 + 资源族隔离重跑 ×15：除 F-1 外全绿；变异探针
  （codec MAX_DEPTH 16→24）立即使 v03_depth 测试失败——套件oracle有效性证实，
  探针已还原、树保持 reviewed commit 不变。
- 高风险路径独立审读：admission顺序（权限→ref→schema→timeout→dup→预算→配额）、
  信任边界（caller字段不在call允许键表、身份仅来自Hello）、路由生命周期
  （Pending/Forwarded两态与watchdog）、store事务/barrier位置、DACL构造——
  与 Design02/本计划合同一致。
- SD-01..04 与实现对读：op族/限额/编译指示/digest/receipt语义/配额数字均与
  代码事实一致；目标路径与 frontmatter new_spec_components 一致。

**F-1（needs_fix，minor，测试侧竞态，非产品缺陷）**
- 位置：`experimental/communication-f01/demo/tests/resources.rs` v12_inflight_
  and_provider_queue_quota 第137行断言（provider队列帽探测）。
- 现象：约1/10重跑（及1/4全门负载运行）得到 DeadlineExceeded(may_have_run)
  而非 ResourceExhausted(not_dispatched)。负载下可稳定复现（12连跑1败，
  20连跑2败均此断言）。
- 根因（已定位，非假设）：三个caller是三条独立连接，broker按连接并发处理
  帧；测试假设 b 的8条被弃呼叫在 c 探测前已全部成为路由。竞态窗口内队列
  确有空位，broker放行转发（Forwarded→1s超时→may_have_run）——**该行为
  本身符合合同**（配额未满时不拒绝是正确的）；错在测试的同步假设。
- 影响：AC-08 的 V-12 队列腿证据不稳定；其余断言与族内其他测试不受影响。
- 修复方向（供work）：确定性观察队列饱和——探测重试直至观察到
  ResourceExhausted（每次被放行的探测消耗一个队列槽位，帽必然到达），
  或等价同步（如让 RawProvider 排读部分pcall计数）。修复后重跑
  resources 族 + 全门一次确认。
- 复审影响面：仅此测试与其 AC-08 证据；其余 AC 与 SD 判定保留。

状态处理：execution_done → executing；current_step 6→5（T-05 重开，
其族内其余证据保留）。

### review final r2 —— F-1 修复复验（2026-11-05）

`stage: review | review_scope: final | reviewer_identity: 同会话（限制同r1；以修复diff+闭包行+重执行重建结论）| plan_id: PLAN-049 | plan_revision: 1 | outcome: pass | worktree_path: D:/autostack/.wt/os-049/auto-os | branch: plan-049-dev | reviewed_commit: 4be813d | base_commit: 831734b | spec_inputs: 同r1（SD-01..04，无改动）| next: merge

closure（check → finding/invariant）：
- 修复diff仅为测试harness（RawProvider::spawn_call_counter 计数读task +
  read半 Option<tak> 化 + v12 探测前等待16路由全部到达provider）；产品代码
  零改动，其余AC证据不失效。
- F-1 闭包：resources 族 20/20 连跑通过（修复前 ~1/10 失败率，P(20连过|仍
  竞态)≈0.12）；断言本体未弱化（仍要求 ResourceExhausted + not_dispatched）。
- 同级路径回归：f1_route（新Option API使用点）6/6 绿。
- 冻结基线统一门：`verify-comm-049.ps1 -Scope acceptance` 两次执行
  7族39测试全绿、0 FAILED（本轮为 merge 复用留证，AC-11 可在
  4be813d 上复用/重放）。
- 变异探针（r1）已还原，树保持 reviewed commit。

**final pass（AC-10）**：AC-01..09 全 pass（证据图 plan-artifacts/
plan-049/acceptance-report.md，V-12 队列腿更新为本修复版本）；SD-01..04
proposed 与实现一致（worktree docs/specs/communication/）。
**merge-owned 门 pending（AC-11）**：冻结提交完整 workspace 门重放/复用、
Spec 发布（SD-01..04 落 canonical）、归档、wt-guard 清理。

状态：executing → reviewed；current_step 6。

### goal 回执（2026-11-05，round 3 —— 终态）

`stage: goal | plan_id: PLAN-049 | plan_revision: r1 | round: 3 | from: merge | to: done | status: archived | outcome: pass | delivery_commit: 4be813d | archive: docs/plans/archived/049-local-communication-f01.md | next: done`

完成审计（对实际盘上状态逐项核验）：① main tip=4be813d，ff-only 线性落地，
无 merge commit；② canonical specs 四文件在落地提交上（哈希见 merge 回执）；
③ 本计划 archived 且已提交 main（1f791e6），status: archived；④ wt-guard
clean ×2 后 worktree/分支/组目录全部移除（盘上 0 残留）；⑤ plan-artifacts/
plan-049/ 已折叠删除、registry 刷新、无未决 hold；主检出本计划写面干净
（其余未提交改动属其他计划/用户 WIP，非本计划触碰）。T-01..T-08 全闭环
（8/8）；AC-01..09 pass（work）、AC-10 pass（review final）、AC-11 pass
（merge：冻结门重放/复用、Spec 落地、归档、清理）。

### merge 回执（2026-11-05）—— `PLAN-049:r1`

`stage: merge | plan_id: PLAN-049 | plan_revision: 1 | outcome: pass | delivery_commit: 4be813d072857560b1a1bde4c0ad96422d6f4430 | worktree: D:/autostack/.wt/os-049/auto-os（已清理）| branch: plan-049-dev（已删除）`

- `prepared`：reviewed 基线 4be813d（=main ff 前基 831734b 的后代，无需
  rebase）；SD-01..04 已随交付提交在树（141d6ed 引入，review 验证一致）。
- `landed`：`git merge --ff-only plan-049-dev` → main tip = 4be813d，无 merge
  commit，线性历史。主检出 landing 冲突核查：incoming 路径
  （experimental/**、scripts/*-comm-049.ps1、docs/specs/communication/**）
  与主检出既有未提交改动零交集。
- `canonical_verified`：docs/specs/communication/{project 42d76175…,
  local-profile-v1 b36061b8…, broker-session 7d723651…, capture-inbox-probe
  698ed3ae…} 于落地提交 4be813d（source 验证；应用索引自动派生，非门）。
  main known-good 复用依据：ff 后 main 树 == 4be813d == 验收门运行树
  （verify-comm-049.ps1 -Scope acceptance 7族39测试全绿 ×2 + 修复后 20/20
  族稳定跑），输入/依赖/配置未变。
- `archived`：docs/plans/archived/049-local-communication-f01.md，
  status: archived，**已提交到 main**（K13：归档文件入版本控制）。
- `cleaned`：wt-guard clean ×2（落地前+删除前）→ worktree 已删除 →
  分支 plan-049-dev 已删除 → 组目录 .wt/os-049 已删除（空）；
  plan-artifacts/plan-049/ 证据已折叠至本文件后整体删除，registry 已刷新；
  构建中间产物随 worktree 删除，未回收。
- `landing ≠ deployment`：落地内容为 experimental/ 实验workspace与验证/
  演示脚本，无生产消费面（apps.manifest 未动、无 daemon 注册、桌面/后端
  不引用）——无重建/重启项，本条为显式观察记录。
- 批量回归 due 检查：PLAN-049 不被 5 整除、auto-os 仓无
  .last-batch-regression.json 到期条件——不 due。

### 归档证据折叠（merge closeout，2026-11-05；原 plan-artifacts/plan-049/ 内容的持久摘要）

**冻结决定（contract-decision.md 摘要）**：依赖锁定 tokio 1.54.1 / serde 1.0.229 /
serde_json 1.0.154 / rusqlite 0.32.1(bundled) / sha2 0.10.9 / getrandom 0.2.17 /
windows-sys 0.59.0；op 族 caller面 hello/welcome/register/registered/discover/
discovered/call/reply/error + provider私有面 pcall/preply；allowed-key 表为未知
字段拒绝单源；ID=128bit hex 文本、凭证=256bit hex 文本仅经文件；digest=
SHA256("f01-capture/1 "+op+ +u32le(title)+title+u32le(uri)+uri+u32le(text)+text)，
capture_id=SHA256("f01-capture-id/1 "+principal+ +op+ +digest_hex)前32hex；
receipt 显式 principal 跨主体=permission_denied；路由 Pending/Forwarded 两态判定
not_dispatched/may_have_run；SQL WAL+synchronous=FULL+busy_timeout=2000、
生命周期 inbox.lock；DACL={user,SYSTEM}+SE_DACL_PROTECTED+reject-remote+
first-instance（ConvertSidToStringSidW oracle）；遏制=SUSPENDED→assign→resume；
read 模型=握手2s+帧体2s、帧间空闲不受限、写2s。实现期修正5缺陷（均有回归）：
partial-header 首字节覆盖、深度按容器计数、client pending 顶替、process_gone 缺
SYNCHRONIZE、JobGuard 缺 Drop。

**AC/V 证据图（acceptance-report.md 摘要）**：
AC-01 独立workspace/闭包 ✕ Cargo.lock；AC-02 t01+v06+v08+v12；AC-03
f0_contract×16+frame_io×6；AC-04 t01 DACL oracle+resources::v05+f1_route::v04；
AC-05 f1_route::v07×2+v06；AC-06 capture_receipt::v09+v14；AC-07 v10a/v10b/v11；
AC-08 f0 限额+frame_io+resources::v12+资源回收 lifecycle::v13；AC-09
demo-comm-049.ps1（run1 committed=50/50、python oracle 50行/51200字节/全1KiB、
run2 幂等）+ acceptance::v14（140ms wall、p50 3ms）。V-01..V-14 全映射见
acceptance-report.md（本轮折叠后原件随 scope 删除）。复审变异探针：MAX_DEPTH
16→24 立即使 v03_depth 失败（oracle 有效，已还原）。已知边界：demo脚本
assign-after-start 微秒窗口、无第二账号不做跨用户攻击实测、kill进程级
SQLite 故障模型、无掉电耐久——与 Design02 §7/§8 声明一致。

### goal 回执（2026-11-05，round 2）

`stage: goal | plan_id: PLAN-049 | plan_revision: r1 | round: 2 | from: review | to: merge | status: reviewed | outcome: pass | worktree: D:/autostack/.wt/os-049/auto-os | commit: 4be813d | next: merge`

### goal 回执（2026-11-05，round 1）

`stage: goal | plan_id: PLAN-049 | plan_revision: r1 | round: 1 | from: work | to: review | status: execution_done | outcome: pass | worktree: D:/autostack/.wt/os-049/auto-os | commit: 141d6ed | next: review`

### work 交接 r1（2026-11-05）

### 起草交接 r1（2026-10-16）

`stage: new`；`plan: auto-os/PLAN-049`；`revision: 1`；`outcome: pass`（合同已具备供实施确认的范围，非代码验收）；`next: 用户确认r1后 /auto-plan:work`。本轮无实现授权，不运行T-01，不新建代理/会话/实施树。
已审：复用来源/主导仓、无活跃匹配Plan/唯一取号、8个任务与owner、AC-01..11/SD-01..04/V-01..14对应、设计单源与未决项、资源/事务/生命周期反例和禁止写面。规划输入指纹/静态检查见attachments/049；新增命令均为未来入口。
后续发现或语义修订在此保留历史，不通过删除验收换通过。最终独立review、冻结门、merge和清理均未执行。

## 10. 待澄清事项

没有必须阻断起草的用户信息缺失。建议选择已在合同中明确：Windows首验、Rust、命名管道、3进程、独立capture服务与专用SQLite；用户确认实施时即确认本r1范围和拟议profile预算。
T-01拥有实际版本/SQL设置/可选字段golden、资源策略及独立测试helper可行性的有界决策；不允许改变首服务/持久性/接入边界或删强制反例。未来跨平台、Auto binding、F2任务/事件/流、Notes/Launcher及ROS2各另立阶段，机器人算法在通信框架之后再议。
