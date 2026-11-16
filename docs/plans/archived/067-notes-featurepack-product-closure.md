---
plan_id: PLAN-067
status: archived
feature_name: Notes FeaturePack 产品入口与联合验收收口
author: [Codex]
created_at: 2026-11-10
updated_at: 2026-11-13
plan_revision: 4
current_step: 7
total_steps: 7
depends_on: ["auto-notes/NOTES-006", "auto-ui/PLAN-083"]
supersedes_spec_components: [docs/specs/communication/featurepack-v0.md]
new_spec_components: []
touched_goals: []
---

# PLAN-067 · Notes FeaturePack 产品入口与联合验收收口

## 0. 变更摘要

承接 [已归档 PLAN-065](065-notes-featurepack-v0.md) 的未结项，与 [NOTES-006](../../../../auto-notes/docs/plans/archived/006-featurepack-service-and-editor.md) r2（已交付）、[READER-013](../../../../auto-reader/docs/plans/013-notes-featurepack-host.md) r5 组成当前执行合同。原归档保留。

2026-11-10 审核结论为 needs_replan（原计划的细度与完成证据不足）。现有服务、组件和代理可复用，双端 EPUB 产品闭环仍待验证。本轮执行计划及证据审核。

## 1. 目标

Windows 单用户本地、构建时共享 UI、一个 Notes 后台。双端实际流程为 EPUB 选句→保存→关闭 Reader→Notes 主应用修改→重开书→回访同一处。完善按需启动、重试、草稿和跨应用生命周期。

AutoLens、查询、图片公式、多用户登录、动态 UI、ACE/AGE 迁移和旧标注全量迁移由后续路线排期。

## 2. 架构方案

OS 管发现/启动/授权，Notes 管服务/组件/事务与草稿公共逻辑，Reader 管书目/语义定位/正文高亮。按接口里程碑接力，OS 联合验收位于子计划实现之后，子计划可先完成本仓实现复审。

现有唯一 OS 实施树 D:/autostack/.wt/os-067/auto-os，分支 codex/os-067-dev；应用沿独立 clone/main。当前主检出有其他 WIP，work 仅接管本合同改面。引擎缺口先列真实反例与 owner 前置，生产启动的AutoUI源码缺口已在§5的2026-11-13复核中确认，OS牵头登记最小框架前置。

## 3. 技术栈

当前 AutoUI Vue/原生 VM、Notes SQLite/请求回执、Reader doc-surface、既有 HTTP 凭据信封。生产 ensure 必须被桌面或应用实际调用，测试从该入口观测。

## 4. 需求分析与背景调查

2026-11-10原起草授权与观察如下表。2026-11-13用户授权更新状态/依赖、复核T-02及明确与Reader推进顺序；本次为文档修订和只读源码核查，实际新生产修复与产品运行待work。

| 基线 | 观察 |
|---|---|
| auto-os b15dd7368783f9722b5b4d09caf734f7dfee9462 main | 065 archived，current_step=2/5，T-03 完成勾与后附未通过记录并存 |
| auto-notes 1aa2e7251c9c437d3bf53ca935d539bd4fc499be main | 006 executing，current_step=1 与三个完成勾不符，VM 输入被 skip |
| auto-reader 38f1a14aa7e41fddf5e0c457fbf3771e31f2a246 main | 013 executing，代理内容哈希派生操作 ID，回访未闭合 |
| featurepack-v0.md SHA256 | b967208a8990992588d73dccf5e1a62ee9ba2d8203494048a3894e5d1f8a71d8 |
| Notes durable-inbox.md SHA256 | 5593db0924667b343f618d190860dc70a376099b9d6625879ccf081ae76e4a80 |
| Reader epub-engine-reading.md SHA256 | daa46296903ffea76aeaab7246137cfab69c5d5730f047b4afd0996b5f493763 |

### 审核发现

| ID/严重度 | 证据、期望与影响 | 关闭责任 |
|---|---|---|
| F-01/高 | OS verify_featurepack_host.py 的 main 定义 ensure_once/host_thread 自行 Popen；两线程成绩不能证明真实宿主使用生产单实例门。manifest 命令行 bin 的真实消费需追踪 | 本计划 T-01/02，两个独立宿主冷启，同实例/数据根 |
| F-02/高 | verify_notes_featurepack.py 导入 combo.txt、HTTP 捕获、locator 空；Notes VM 编辑器 E-2..E-6 skip，退出码只看 FAIL；缺双端 EPUB 回访证据 | T-03/04 + 子计划，实际输入/回访，必需 skip 非零 |
| F-03/高 | Reader fp_note_capture 用 book、quote、text 哈希，未含范围；同句异处可能合并。attempt_id 固定，更新操作键仅含 note/rev | Reader T-06，按一次意图分配 ID，重试保持、新建独立 |
| F-04/高 | Reader book_version 固定 v1、author 空，缺真实 source_hash；FpEditStart 仅编辑正文；组件缺契约声称的回访事件 | Reader T-06/07，真 canonical 锚、来源回访与换版拒绝 |
| F-05/高 | 草稿只在宿主 model 字段，未见跨进程持久草稿；迟到成功可能清新输入，通用失败与 conflict 混合 | Notes T-06/07 + Reader T-06，重开恢复、输入版本比对 |
| F-06/中 | Reader 只取 50 条，UI 包仍为探针命名；path 依赖缺固定提交清单，Spec delta 泛化 add | T-01、子计划分页/清单与 SD 修订 |
| F-07/高 | 组合脚本未隔离 AUTO_READER_DATA；脚本 taskkill /IM auto.exe、Notes exe，可能终止其他任务 | T-03 和子计划测试任务，只回收自有进程，忙端口拒跑 |
| F-08/中 | active 链接仍指已移走的 065，机制改变未升 revision，011 共享数据写入归属含混 | 本轮修订和 T-05，互链/修订/进度真实、011 前置明确 |

以上为只读源码及合同检查。内容碰撞、草稿丢失等为代码路径支持的反例风险，未在本轮运行产品复现。F-01/02/07 已由脚本控制流直接确认。

## 5. 详细设计

### D-01 生产装配与固定包

T-01 追踪 apps.manifest→实际 daemon consumer 函数→启动 argv/env→health→Reader 后端，记录路径/符号。consumer 如在 auto-lang，则只读确认支持集，缺口交该 owner 另立最小前置。测试不重写 ensure 后充当产品证据。

包描述给 pack_id、接口版本、实际 FpNoteEditor/目录包、模式/操作配置与默认值、主应用 ID、启动 argv/工作目录/env、三仓完整 SHA 和组件 hash。配置只收窄服务权限。组件纯渲染与回调，公共请求/草稿状态适配由 Notes 提供，Reader 保留来源适配；两应用消费同版，干净 checkout 构建核 hash。

单实例覆盖独立宿主进程并发冷启，界定用户与数据根。端口被其他服务占用或 health 错版时拒绝复用并保留占用者。当前 600s 含构建等待作为开发 profile 记录；UI 立即显示可恢复状态并允许继续阅读，T-01 冻结实际部署 profile 预算及来源。

### D-02 授权与恢复

首版采用本地 OS 用户身份和服务私有 policy。policy 路径留在服务，每宿主仅取自身凭据和端点。桌面、独立模式分别明确凭据创建/传递/保存、书籍范围授予和撤销的 owner；正常工作无需用户修改测试 policy。冻结授权书与现有 policy 字段映射，明确 epoch 未实现字段的处理。授权变更后 get/update/receipt 检查当前权限；重启后合法主体仍可查询原已提交操作。

### D-03 无环交接

| 里程碑 | 产出 | 消费 |
|---|---|---|
| M1 | OS T-01已完成的字段/配置/版本/接线追踪，r2补本次生产残项 | Reader T-01/T-06现在可准备，OS T-02按残项登记owner前置 |
| M2 | NOTES-006 r2已交付的服务/公共状态/组件及已裁定边界 | Reader T-06/T-02消费，Notes不再是待交付前置 |
| M3 | Reader T-02启动消费子面配合OS T-02生产回执；Reader T-07业务功能完成 | Reader T-03/T-08接生产路径，不等待OS整Plan终态 |
| M4 | Notes固定交付 + Reader T-08 + OS T-04真实联合门 | Reader与OS按各自顺序复审交付，引用精确提交和profile |

### 规范增量

| delta_id | add/modify/retire | docs/specs/... target | before/after rule | rationale | acceptance IDs |
|---|---|---|---|---|---|
| SD-01 | modify | docs/specs/communication/featurepack-v0.md | 探针/候选描述→实际组件、配置、版本、生产启动/授权与验证覆盖 | 平台契约与产品证据一致 | AC-01～05 |

本轮不发布 canonical Spec。work 在实测后沉淀，proposed 字段不推导已有产品支持。

### M1 历史冻结（T-01 产出，2026-11-11；当前结论见下方r2复核）

追踪基线：auto-os main 253deb9、auto-notes main 1aa2e72、auto-reader main 08d4932、
auto-ui 主检出（crates/auto-ui-runtime + auto-ui-compiler）、auto-lang 主检出。

**源码函数表（manifest → 真实 daemon consumer → argv/env → health → Reader 后端）**：

| 环节 | 符号/位置 | 语义 |
|---|---|---|
| manifest 条目 | `apps.manifest` `015-notes.daemon` | `{port:17819, bin:"auto service --server rust -B 17819", env_port:"NOTES_DAEMON_PORT"}`；repo=`apps/auto-notes` |
| 触发键 | pac `daemon:`（LaunchSpec.daemon，auto-ui session.rs:2773） | 只有 pac.at 声明 `daemon:` 才触发；musk/jade-garden 有，notes/reader **无**（G-02） |
| ensure 入口 | `session.rs:4407 ensure_daemon_if_declared` → `:4434 ensure_manifest_daemon` | launch 时按 LaunchSpec.daemon 名查 manifest daemon 对象 |
| manifest 查表 | `app_registry.rs:751 manifest_daemon_lookup`（resolve_os_manifest_root：AUTO_OS_ROOT → ../auto-os → D:/autostack/auto-os） | 返回 (repo_dir, ManifestDaemon) |
| GenericDaemon 组装 | `session.rs:4447` | url=`http://127.0.0.1:<port>`；bin=`repo_dir.join(rel)` + Windows `.exe` 补全（**路径语义**，G-01）；env_key=`daemon_env_key(name)`（G-09）；env_port_key=manifest `env_port` |
| 泛化 ensure | `osconfig_daemon.rs:344 ensure_generic_io` | ping → bin 缺席=Offline → spawn（env 仅 env_port_key→port）→ 200ms 轮询 5s |
| 检活 | `osconfig_daemon.rs:137 tcp_ping` / `RealDaemonIo::ping` | 裸 TCP `GET /api/health` 2xx 即通；**不核 body interface**（G-03，spec §2.2 要求 200+interface 核验） |
| spawn | `osconfig_daemon.rs:176 RealDaemonIo::spawn` | detached，stdio null，只带 env_port_key |
| env 注入 | `session.rs:4475 set_var(env_key, url)` | Running 才注入；键=`<NAME 大写蛇形>_DAEMON` |
| Reader 消费 | `auto-reader/src/back/api.at fp_base()/fp_cred()` | 读 `NOTES_DAEMON`（缺省 17819）/ `NOTES_FP_CRED`；端点 /api/fp/note/{capture,get,list,update} → notes.fpack/1 |
| Notes 服务 | `auto-notes/src/back/api.at:283-324` + `db.at:497` + `repository.at:1923-2365` | 五端点全 POST body；policy 凭据绑定（NOTES_FP_POLICY 或 data/fp-policy.json，缺席=全拒绝）；去重键 `fp:<principal>:<operation_id>` |
| 组件 | `auto-notes/src/front/components/fp_editor.at`（FpNoteEditor）+ `pac.at`（notesfp 0.1.0） | props: mode/note_label/draft_text/save_state/conflict_revision/server_body；回调 on_draft_edit/on_submit_text/on_rebase；草稿 prop 往返 |
| Reader dep | `auto-reader/pac.at dep notesfp { path: "../auto-notes/src/front/components" }` | 跨仓组件消费面 |

**字段冻结（D-01/02 定形）**：pack_id=notes-fpack、version 0.1.0、back.interface=notes.fpack/1、
back.port=17819；调用信封必带 cred（256-bit hex）+ interface_version；policy 字段
cred/principal/operations/book_scopes；book_scopes 单位 `book:<id>`；端口带 17xxx
（front 17818 / back 17819）；data 根经 NOTES_DATA_DIR 覆盖（缺省服务私有目录）。

**配置与预算**：ensure 轮询 5s、ping 250ms 快失败；spec 记载 600s 就绪窗为**开发 profile**
（含工具链构建），T-01 冻结实际部署 profile=生产 ensure 5s 档；调用 timeout_ms 缺省 2000。

**2026-11-11缺口历史清单（当前关闭状态及owner见r2 P-01～09）**：

| ID/severity | 证据与影响 | owner/前置 |
|---|---|---|
| G-01/高 | 生产 GenericDaemon.bin 走**路径语义**（repo_dir.join + .exe 补全），manifest bin 是**命令行** `auto service --server rust -B 17819` → 生产 spawn 必失败（找不到该路径）；verify_featurepack_host.py 自行 split() Popen 绕过了生产链（F-01 实锤） | auto-ui（manifest bin 命令行语义消费面）或 OS T-02 落地生产启动门；前置=真实宿主消费面支持命令行 bin |
| G-02/高 | notes/reader pac.at 均无 `daemon:` 键 → 桌面 launch 015-notes/018-book-reader 不触发 ensure_manifest_daemon → 生产不注入 NOTES_DAEMON（F-01 另一半） | NOTES-006/READER-013 pac.at 声明面（或 OS T-02 落桌面装配）；前置= pac `daemon: "015-notes"` |
| G-03/中 | 生产 tcp_ping 只核 2xx，不核 body interface==notes.fpack/1 → 错版服务可能被复用为就绪 | OS T-02 版本检查（生产消费面或测试入口）；spec §2.2 与实现对齐 |
| G-04/高 | verify_notes_featurepack.py 未隔离 AUTO_READER_DATA、taskkill /IM auto.exe / 具名 exe 可能终止其他任务（F-07） | OS T-03 隔离数据/端口/进程，只回收自有 PID |
| G-05/中 | verify_featurepack_host.py kill_all() 同样 taskkill /IM（F-07 同源） | OS T-03 |
| G-06/中 | Reader fp_note_capture 固定 book_version=v1、author 空、locator 空时包 `""`（F-04）；operation_id 空时 ph1(book|quote|text) 哈希派生（F-03） | READER-013 T-06/07（真 canonical 锚/每意图 ID） |
| G-07/中 | Notes 草稿只在宿主 model 字段，未见跨进程持久草稿（F-05） | NOTES-006 T-07（共享适配/持久草稿）+ OS T-03 组合面 |
| G-08/中 | 组合脚本导入 combo.txt（TXT 非 EPUB）、locator 空、HTTP 捕获代 UI（F-02） | OS T-04（真实 UI）+ READER-013 EPUB 真实选区 |
| G-09/中 | 生产注入键派生 `015_NOTES_DAEMON`（daemon_env_key("015-notes")），Reader 消费 `NOTES_DAEMON` → 生产链注入键与消费键不一致 | OS T-02（manifest daemon 语义或注入键映射对齐） |
| G-10/高 | auto-lang PLAN-994 生成器漂移：对已声明字符串局部变量的 len() 生成为 chars().count()（字符数），而 str_substr 第三参数按字节数；同一 .at 表达式 doc.substr(0, doc.len()-1) 在参数和局部变量两处翻译不一致（生成代码 1917 行 doc.len() vs 2072 行 chars().count()）——含中文 doc 时截断错位产生坏 JSON（list 端点 updated_at 字段值缺失）。065 时代 len()=字节数正常，本轮实测复现（G-10-probe）。owner=auto-lang（PLAN-994 消费面对齐），前置=生成器 substr/len 语义统一或 .at 源改用 len_bytes |
### 2026-11-13 r2 状态与生产接线核查（历史；当前交接见r3正式前置）

本节是当前交接基线。上方 M1 为2026-11-11历史追踪，当前源码已重新读取。本次仅做计划/源码/已提交脚本核查，没有启动生产进程或重跑产品门。OS状态修正为 executing；完整关闭任务为T-01与T-03的测试基础设施部分，计2/7。T-02撤回完成勾并保留既有服务级成绩为历史证据，AC-01仍未关闭。T-03的完成范围限定为隔离/清理/负控基础设施；原描述中的生产接入由T-02闭合，profile真实UI分发与必需用例聚合由T-04闭合。

| 当前输入 | 核查结果 |
|---|---|
| auto-os主检出 | 868c80d3e697ad6eaa9cce27596430a9abe08d5a；本Plan位于主检出且未跟踪，执行状态以本文件为准 |
| OS唯一实现树 | D:/autostack/.wt/os-067/auto-os，codex/os-067-dev，1f9e9f7138867e12b9842d5b4ed15ce946dbf481，当前干净；提交只改两份测试脚本 |
| auto-ui生产源码 | 8320cbfd1ad01cf6bbdcf59cef31acab5dbf5bf2；session.rs/osconfig_daemon.rs/app_registry.rs在本次核查路径无脏修改 |
| Reader | aa58fef763e440394484662e102329b7ee7f0471，013 r2 executing 0/8；正文代理仍为旧来源格式且未消费fp_host |
| Notes依赖 | NOTES-006 r2 archived 8/8，归档记交付80a6d98b8d2bf28eb4fbffb5330a6fb4458f4062，当前ffdb75021d0b696958093c56d487e770e2a3cf17包含该提交（ancestry exit0） |
| Notes包内容 | fp_editor SHA256 355cab805b644a9c7a7773c445ae3a60dcc9cdd17aa1fbbb58064c77011e23c0；fp_host 3342f3e267276442b85de0a4b198c1a5952fbbe3ab34dcb80b277245e57c9125，与交付Spec一致 |
| Notes已交付边界 | 本地草稿scratch端点属于Notes主应用，Reader草稿自持并复用公共状态函数。Notes主应用VM条件edit输入有2026-11-12用户裁定边界，create真实输入+状态面验收；该裁定不能直接推导Reader/OS的完整VM流程已通过 |
| 当前数据适配前置 | Notes服务要求source_hash/title/author/quote/locator结构，Reader仍传book_version=v1/author空并以内容hash派生operation_id；013 T-06/02须先与实际已交付接口对齐 |

生产查表真实定义在 auto-ui/crates/auto-ui-compiler/src/app_registry.rs（runtime ui/mod.rs:174仅re-export）。session.rs与osconfig_daemon.rs属于auto-ui运行时，不属于auto-lang。已检索当前auto-ui活动计划，081只承接Reader路由/编辑链，尚无确认匹配的daemon接线计划。T-02负责先登记精确框架前置与owner，再消费其交付；无需重新征求同范围核查授权。

| 残项 / 原发现 | 当前证据与判定 | 主责任、下一动作与关闭条件 |
|---|---|---|
| P-01 / G-01 | open。ManifestDaemon.bin是Option<String>，session.rs:4455将其join为路径并补.exe；RealDaemonIo::spawn用Command::new(path)。清单仍填auto service命令行 | OS T-02牵头装配。先选择与现有路径契约兼容的已构建服务可执行件及发布路径，或给auto-ui登记结构化启动参数扩展前置；不可把支持命令行当已实现。真实宿主从冷态启动已发布Notes二进制后关闭 |
| P-02 / G-02 | open。Reader与Notes两个pac.at均无顶层daemon声明，ensure_daemon_if_declared缺名即返回 | Reader T-02负责Reader声明和消费；OS T-02负责Notes主窗口的启动装配交接。按实际启动模式确认双方需要的声明；修改已归档Notes-006不作为前置，若需改Notes pac则单列精确产品改面。实际两个入口触发同一ensure链才关闭 |
| P-03 / G-03 | open。tcp_ping只读首128字节并判HTTP/1.x 2xx，不解析body/interface | auto-ui运行时owner实现产品就绪检查，OS T-02消费。200但interface错误的占用者必须拒复用且存活；正确版本冷启/复用均通过。测试脚本自有health_ready不能替代生产检查 |
| P-04 / G-09 | open。daemon_env_key("015-notes")生成015_NOTES_DAEMON，Reader只读NOTES_DAEMON；env_port是子服务端口键，不能承担客户端地址键 | OS T-02牵头冻结显式地址映射与后端启动注入点，必要运行时支持由auto-ui交付；地址在Reader后端首次请求前可见。测试使用非默认端口/目录以排除硬编码17819回退掩盖缺口 |
| P-05 / D-02 | open。ensure_generic_io显式env仅端口，未见每宿主签发凭据、授权书更新/撤销或NOTES_FP_CRED注入的生产代码；现有进程环境可继承，但不提供产品签发证据 | OS T-02负责受管policy与每宿主凭据供给/书籍范围授予，运行时owner负责精确传入服务与Reader后端。正常新增书及重启无需用户改测试policy；错书/撤销后拒绝，合法receipt可恢复 |
| P-06 / C-01 | open。ensure_generic_io是ping→spawn，无跨进程启动互斥；worktree host脚本仍由两个线程自行Popen | auto-ui生产启动owner实现或提供进程间单实例门，OS T-02验证两个独立真实宿主同时冷启，最多一实例/数据根；错误端口占用不杀进程，关闭主窗口后另宿主仍可调用 |
| P-07 / 退出恢复 | pending。ensure在应用launch触发，现有证据没有证明服务退出后普通业务重试会重新进入生产ensure | OS T-02与Reader T-02确认实际恢复入口。杀自有后台后从已打开Reader重试，生产入口恢复并用原operation查询/重试；如恢复动作是显式重新打开，必须产品内可执行并在合同中准确说明 |
| P-08 / F-07 | implemented in worktree。1f9e9f7两脚本改为自有PID/隔离数据/占用拒跑。主检出同名脚本仍是原065版本，尚未合入部署 | T-03基础设施已完成，后续测试使用唯一OS树的修复版；T-04/05继续核其与最新Notes接口兼容，再由review/merge交付，不以此关闭P-01～07 |
| P-09 / G-10 | Notes路径已有规避。其current-state Spec写明改为row_to_doc_fields拼接，当前包与交付一致 | Reader/OS采用最新Notes数据接口，先验中文往返。语言len/substr全局语义是否修复未在本轮核验，不能标全局关闭 |

源码指纹（SHA256）如下，用于本次只读结论追溯。session.rs=792e0940d2780854e2fa6427adb092d0611547349aa8289f6d9e3c8a1af8c328，osconfig_daemon.rs=3dddd135c3e9cb40597a87ad4eee462cbbadeaf55d7e4096ed75b59b2c5af741，compiler/app_registry.rs=4b64f35171708ac909dc2243f91785ddbeb368e3652abcb668d20acc7543a78b。apps.manifest=e7409f924090700b9cbc7d74a09531ec3b356bf389e79141a1852e26d46bd3e1，Reader pac=9c45c12edb1f93820427b080bdee276a19d54816291a2d545707ba330ac9d81d，Notes pac=a59ab766400ec36a6172ef2cdfb9e572ed4d0ee401e5893a6f0c9ea7badcd3df。

### 2026-11-13 T-02 r3 装配落地与前置登记（work 本轮）

本轮为 T-02 的 OS 改面实施 + 运行时前置登记 + 生产形状服务级实测。auto-ui
运行时源码零触碰（上方 r2 指纹仍有效），P-03/P-05传递/P-06 以 non-contract
债务条目登记于 auto-ui `docs/plans/debts.md`（f21ab5e7，DEBT-003/004/005，
"登记不构成合同"）。改面与回执：

| 改面 | 位置/提交 | 内容 |
|---|---|---|
| apps.manifest | auto-os worktree 72f962f | 015-notes daemon.bin：命令行→**路径语义** `../../../auto-notes/target/release/notes-service`（镜像 session.rs:4460 join+.exe 补全；对齐 musk/jade-garden 稳定名先例；组根相对——主检出解析到独立 clone，worktree 根解析到组依赖位）；env_port：NOTES_DAEMON_PORT（无消费者）→ **AUTO_HTTP_PORT**（生成后端端口解析链 AUTO_HTTP_SERVICE_JSON > AUTO_HTTP_PORT > 8080，api_gen.rs:4986-5056） |
| 发布入口 | worktree 72f962f `scripts/build_notes_service.{sh,ps1}` | auto service 生成链构建 → 稳定名复制 notes-service.exe + SHA 回执；AUTO_ALLOW_PARTIAL_DB=1 沿 notes 仓自有 battery 同款接受面（asset_upload=NOTES-002/PLAN-730 混合态，fp v1 端点全部有 db.rs 匹配）；`--target-root` 显式受管缓存根与活动会话解耦 |
| 受管 policy | worktree 72f962f `scripts/fp_provision_policy.{sh,ps1}` | P-05 OS 面：缺省 `<data>/fp-policy.json`（repository.at fp_policy_path 同律；NOTES_DATA_DIR 即数据目录本体）、fp-reader 条目（256-bit hex cred）、add-book/revoke-book 范围授予/撤销；幂等、cred 稳定（实测通过） |
| host 脚本对齐 | worktree 72f962f `scripts/tests/verify_featurepack_host.py` | daemon_bin_cmd 镜像生产路径语义解析（+.exe 补全，065 命令行 split 兼容回退）；fixture 对齐 NOTES-006 r2 冻结闭集 source schema（source_hash/title/author/locator——旧载荷已被服务 invalid_request 拒绝，即 r2"当前数据适配前置"实锤） |
| Notes pac | auto-notes **6a7b0e7**（本地提交，push/pin 随 T-07 交付序） | P-02 OS 侧：`daemon: "015-notes"`（单列精确产品改面；独立 auto run 不经 session launch 面不受影响） |
| 发布件 | clone `target/release/notes-service.exe` sha256=81994ad7…db86119（ffdb7502 源，gc-target run4）；worktree 组依赖位同路径 sha256=b1e90d9f…d350726（6a7b0e7 生成件 auto-notes-57d3816f-back，gc-target 热依赖 cargo build 3m42s） | 两 manifest 根解析位均就位；worktree 组依赖检出 `.wt/os-067/auto-notes`（detached 6a7b0e7，无 submodule） |

**V1 生产形状探针（plan-artifacts/plan-067/v1-probe，11/11 PASS）**——spawn
形状与生产 ensure 完全一致（路径语义 bin、仅 AUTO_HTTP_PORT 端口 env、隔离
数据根、任意 CWD）：V1a 冷启 0.2s 就绪 + health 契约 body
`{status:ok, interface:notes.fpack/1}`；V1b 非默认端口 17999 同过（P-04 硬编码
排除）；V1c 授权面（合法 cred capture ok / 错+空 cred permission_denied / 被拒
无回执）；V1d 双并发 spawn 收敛（恰 1 LISTENING、败者 bind-fail exit 1、零
互杀）；V1e 错版占用反例（200+`other.svc/9` 占用者满足现行 tcp_ping 2xx 前缀
但违背接口契约、占用者全程存活——P-03 缺口的具名活体反例）。host 脚本
**19/19** 重跑（H-01 冷启/H-02 并发合并+单实例/H-03 拒绝面/H-04 在位不再
spawn/H-05 杀后重连+回执持久），spawn 形状=生产路径语义。

**前置登记（owner/符号/关闭 oracle；详见 auto-ui DEBT-003/004/005 原文）**：

| ID | 残项 | owner/登记位 | 关闭 oracle（要点） |
|---|---|---|---|
| R-1 | health body interface 断言 | auto-ui runtime（DEBT-003；tcp_ping:137 只核 2xx 前缀） | 错 interface 占用者拒复用且存活；正确版冷启/复用过；无声明 daemon 零行为差；消费=manifest 加 health_interface 字段 |
| R-2 | 客户端地址键消费 | READER-013 T-02（契约冻结：键=`015_NOTES_DAEMON`，daemon_env_key 派生不改；NOTES_DAEMON 降为测试/独立覆盖键） | 生产注入后 Reader 后端首请求基址可见；非默认端口实验排除硬编码回退 |
| R-3 | 每宿主凭据/授权传递 | auto-ui runtime（DEBT-004；ensure spawn env 仅端口键 :358） | GenericDaemon 可选 env 双面注入（spawn+会话，写点同 env_key 先例 :4483）；Reader 后端携合法 cred 调用 ok、错书/撤销拒；附带回：AUTO_HTTP_SERVICE_JSON 注入面（service profile 门控 media/photo scan 收窄，当前 legacy face 全开——v0 冻结为已知边界） |
| R-4 | 跨进程单实例门 | auto-ui runtime（DEBT-005；ping→spawn 无互斥） | 双独立真实宿主并发冷启恰 1 实例/数据根；错端口占用不杀；关窗另宿主可用；服务层 bind-fail 收敛（V1d/H-02b 实测）为兜底非门 |
| R-5 | 杀后恢复入口合同 | OS 冻结 v0：恢复动作=重开 App 窗口（launch→ensure 重入）；会话内自动重 ensure 不在 v0，Reader 侧如需在 READER-013 T-02 显式登记 | 服务被杀→新开窗口→生产 ensure 恢复→原 operation 回执 committed（服务层 H-05 已证） |

T-02 关闭剩余：R-1～R-4 交付消费 + Reader T-02 启动声明/后端消费子面可运行
后，按 §6 C-01/C-04 以两独立真实宿主（桌面编排）全链验证——含错版拒复用、
生产凭据传递、非默认地址、杀后产品级恢复。本轮 V1/host 成绩为服务层+生产
spawn 形状证据，不冒充桌面链。

### r2 推进顺序（历史；当前以r3正式前置表为准）

| 顺序 | 可执行任务 | 需要的输入 / 输出 |
|---|---|---|
| 1，同期准备 | OS T-02的P-01～07契约/框架前置登记；Reader T-01与T-06 | Notes006已交付、M1已存在。Reader做重复句EPUB/旧边车基线、真实来源与ID/草稿映射，OS明确生产启动/环境契约；双方不等待对方整Plan终态 |
| 2，同期实现 | OS T-02生产门/凭据/health/单实例；Reader T-02与T-07 | 消费同一字段和地址契约、Notes已交付包。Reader接共享适配/草稿/分页/高亮回访，并补自己的pac声明及后端地址/凭据消费。受控隔离服务可用于开发，不能计冷启动验收 |
| 3，生产交接 | OS T-02关闭 | 需要Reader T-02的启动声明/后端注入消费子面可运行，以及所列运行时前置交付；不等待Reader T-03/T-08。两真实宿主冷启/错版拒绝/授权/杀后恢复全部验证，形成M3启动回执 |
| 4，Reader收口 | Reader T-03→T-08 | 消费OS T-02生产回执，在真实Vue/原生VM执行R-01～05，核notes固定版本、源锚、丢回包/草稿/51条分页。随后Reader T-04/05可顺序复审交付 |
| 5，联合验收 | OS T-04 | Reader T-08实现和测试材料齐全，Notes006依赖已满足；固定两应用/包/引擎提交重放C-01～05。Reader终审可先完成，OS联合门不作为Reader实现前置 |
| 6，沉淀交付 | OS T-05→T-06→T-07 | 实测闭合后核Spec、独立复审和精确合入；此前脚本API成绩仅保服务层证据 |

Notes主应用的VM边界按已记录裁定消费。Reader/OS的VM产品用例先实跑；若复现条件子树回调缺陷，记录最小场景和auto-ui owner，交其修复或另行取得范围决定。当前Reader/OS的AC要求继续有效，不能把Notes的局部裁定自动扩用。


### r3 当前正式前置与下一步（2026-11-13 本次交接）

AutoUI三项债务正式由 [auto-ui/PLAN-083](../../../../auto-ui/docs/plans/083-featurepack-daemon-runtime-contract.md) r1承接，当前drafting 0/8。DEBT登记保留追溯，不能当作运行时交付。083合并承接health版本检查、精确配置/凭据传递与跨进程单实例，另核同一Notes服务的Vue后端共享生命周期；源文件相同的三项工作在一个权威树顺序实施。

T-02本轮完成OS装配工作，但整体仍partial，OS保持executing 2/7。P-01采用路径式发布，P-04端口为AUTO_HTTP_PORT、客户端键015_NOTES_DAEMON，P-07恢复动作已收敛为重开窗口。窗口内自动重ensure不作为当前v0必需路径。现有V1/host成绩保留为对应提交的服务层/生产形状证据，桌面两真实宿主仍待验。

| 当前接力 | 责任/实际动作 | 可推进条件与关闭证据 |
|---|---|---|
| 现在A | UI083 T-00→T-01/02/03/04→T-05/06/07 | 先冻结可选字段、当前JSON信封、配置生效机制、锁身份与Vue共享后端归属，再实现/实测/交付；不等待Reader或OS终态 |
| 现在B | Reader013 T-01/T-06，随后T-02/T-07 | 真实EPUB/来源/操作身份/草稿/分页/高亮可同期推进，启动配置与UI083 T-00对齐；T-02消费015_NOTES_DAEMON与自己的cred，独立覆盖语义明确 |
| UI交付后 | OS T-02消费083的固定SHA/新manifest配置；Reader T-02补相应启动声明与后端配置 | 只有二者具备后，才跑C-01/C-04两独立真实宿主：错版拒复用、精准凭据、spawn计数=1、非默认地址与重开恢复 |
| 再后 | Reader T-03→T-08；OS T-04 | Reader双端产品门完成后OS联合验收；Notes主应用原有VM边界单列，Reader完整输入/回访仍照原合同 |
| 最后 | OS T-05→T-06→T-07 | 规范、独立复审、精确交付与pin消费；083实现交付前不关闭DEBT或T-02 |

UI081路由编辑、UI082文件对话框仍各自交付，不能替代083的后台启动合同。083的生产health必须处理目前真实Notes信封；凭据不能简单通过全局env轮流改写后称为每宿主隔离。真实manifest共享provider还要避免Vue后端启动器对已由daemon管理的Notes再次绑定17819，并保共享服务在主窗口退出后存活。

#### 本次补查的OS脚本收口项

下列为当前提交72f962f源码可直接观察的缺口，本次未运行复现；由OS T-02收口，UI083只消费明确的宿主接口。

| ID | 观察源/事实 | 关闭要求 |
|---|---|---|
| A-01 产物绑定 | build_notes_service.ps1按generated-consumers通配符mtime选最新exe；sh在显式根没找到时也回退全局最新。当前Notes源码HEAD与所选exe的对应关系未由此证明 | 从本次构建输出/精确成员取得exe，绑定source/tool/profile/数据根与hash；显式构建失败不得取另一根旧产物冒充新成功。冷构建和已有其他根更新产物的反例均核对 |
| A-02 私有凭据输出 | 两个fp_provision_policy脚本默认打印cred，受管policy只做创建/写入，尚无私有凭据回执及显式权限处理证据 | 对照D-02和现有“凭据不进日志/公开回执”的契约，改为私有受管传递；日志脱敏并核实际文件权限；两个不同宿主只拿自身所需配置。不要把供给stdout原样归档 |
| A-03 发布支持面 | 发布使用AUTO_ALLOW_PARTIAL_DB=1与Notes002的asset_upload混合态；本次fp业务通过不说明完整Notes API发布可用；sh/ps1的显式target与等待参数也不一致 | 交付回执列明构建开关/实际支持端点，按Notes真实交付状态决定有限fp服务profile或修复前置，保必需fp端点真测；两入口需有一致的产物定位/失败规则 |

当前必要执行入口是UI083 T-00与Reader T-01/T-06，OS可并行修上表的发布/私有传递收口。原OS P-01/OS供给脚本“装配完成”只描述已有改面，不推导以上验证已经关闭。

### r4 Reader-009交付前置与排程修正（2026-11-13）

Reader侧此前“可同期实现”的安排需收窄为只读准备。[READER-009](../../../../auto-reader/docs/plans/009-workspace-delivery-and-import-seams-closure.md) 当前executing，其权威树f6ab049ab65e16de0f84a900a84a38400ee3e05f的api.at/db.at/locator.at/reading_page.at改面与013重叠；009继续完成独立复审和主干交付，再交给013核新基线实施。013当前executing 0/8，依赖009交付，不抢写009或在旧main并行实现同一产品文件。

现在继续UI083和OS A-01～03独立改面，Reader013仅只读接口核查/样例准备。009交付后013完成T-01/T-06新基线，再T-02/T-07；UI083实际交付和013启动消费子面齐备后，OS关闭T-02生产门，Reader做T-03/T-08，OS最后T-04联合验收。UI083无需等009完成才开发，OS独立装配收口也不以009整Plan为前置；受009交付约束的是Reader产品写入与相应双应用验收。

OS当前executing 2/7不变。原§5“现在B/Reader同期实现”等文字属于之前排程，当前以本节与Reader013 r5为准。OS T-02的Reader前置在任务表中显式注明009交付后的启动消费子面。

### 2026-11-14 T-02 r5：083 消费落地与 A-01～03 收口（work 本轮）

auto-ui/PLAN-083 已交付（archived 8/8，delivery 8997393a@master；SD-01
`docs/specs/runtime/featurepack-daemon-hosting.md` 发布）。本轮为 OS 侧消费与
A-01～03 收口，auto-ui 运行时零触碰。worktree 72f962f→**68c8632**。

| 改面 | 提交 | 内容 |
|---|---|---|
| manifest 消费 083 | 68c8632 | 015-notes daemon 增 `health_interface: "notes.fpack/1"`（严格健康合同——错版占用者现在生产级拒复用）、`data_root: "%APPDATA%/auto-notes/data"`（实例身份组成）、`host.credential {env: NOTES_FP_CRED, path: %APPDATA%/auto-notes/data/fp-policy.json, principal: fp-reader, field: cred}`（受管凭据引用，仅独立进程后端投递，SD-01 §5；in-proc 缺口具名诊断）。无 daemon.env（服务自身解析 %APPDATA%，literal 不展开）；显式 path 必须（缺省基目录=repo 目录，指不到数据根） |
| A-01 产物绑定 | 68c8632 | build_notes_service 重写：成员名取自**本构建日志**（`Generated Rust server: <member>/`），产物限本缓存根唯一定位（显式根 release/ / 受管根 `<ctx>/release/`），**零跨根/全局回退**；绑定门=READY+日志成员+根内唯一，mtime 作关联记录（fresh-build / 增量重建须 cargo Finished 佐证）；`--target-root` 现经 AUTO_GENERATED_TARGET_ROOT 钉住构建落点（此前只定位不落点）；按路径清孤儿仅限显式私有根（缺省共享根绝不扫，防误伤他 Session） |
| A-02 私有凭据面 | 68c8632 | fp_provision_policy 双侧：默认 stdout **无凭据**（policy/principal/scopes + sha256 前 12 指纹）；`--reveal-cred`/`-RevealCred` 显式（私有传递窗，stdout 不得归档）。**BOM 缺陷修复**：ps1 5.1 `Set-Content -Encoding UTF8` 写 BOM → 运行时 serde_json 与服务 fs_read 均不容忍（sh/ps1 交叉读检查抓获）；双侧改无 BOM 写 + 历史 BOM 文件强制归一重写 |
| A-03 回执支持面 | 68c8632 | 回执列明：构建开关（AUTO_ALLOW_PARTIAL_DB=1 混合态范围）、实际支持 fp 面（health + fp/v1 五端点 + draft scratch）、service profile 门（media/photo off）、NOT-covered 注记（asset_upload 写入）；sh/ps1 参数/失败规则对齐（--wait-secs/--target-root 孪生） |

**生产受管 policy 落地**：`%APPDATA%\Roaming\auto-notes\data\fp-policy.json`（fp-reader，256-bit cred，scopes 空；供给回执只含指纹 sha256:3e14fc9789d5——无凭据落档）。

**实测（plan-artifacts/plan-067，V2 探针 9/9 + A-01 反例 + 全量回归）**：
A-01 反例——构建端口被占 → auto service 具名 bind 冲突退出 → EXIT=3、发布件
sha 逐字节不动；sh 成功回执——fresh-build、member=auto-notes-57d3816f-back、
发布件 6939bafa@6a7b0e7（notes HEAD/tool HEAD/exe path/sha 全绑定）；ps1 对齐
——增量重建关联分支 + 同 sha（6939bafa，跨入口确定性）；V2 9/9——严格健康
形状（1 层信封解包、interface 逐字、status ok）、受管凭据解析同律（len 64）、
未授予拒/授予 capture ok/撤销拒/已提交 receipt 撤销后仍可查（committed）、
数据根对齐（%APPDATA% inbox.db 落位）、scope 净零（t02probe 已撤销）；V1
11/11 + host 19/19 回归（新 manifest 字段 + 新发布件）。

**T-02 剩余（唯一）**：READER-013 T-02 启动消费子面（009 已交付，无阻塞）→
齐备后 C-01/C-04 两独立真实宿主桌面编排（083 单实例门/严格健康/子进程凭据
全链）。生产 launch 链演示（桌面实际触发 ensure→严格健康→凭据投递）随编排轮
一并取证——本轮 V2 为生产 env/生产根/生产 policy 的服务侧演练，不冒充桌面链。

### 2026-11-14 T-02 r6：C-01/C-04 编排预备核查发现 DEBT-007（装配缺口，凭据腿阻断）

编排形态核查（auto-ui master 04dd3a70 前源码 + 8571948 消费面）发现**真实桌面流
装配缺口**——已登记 auto-ui **DEBT-007**（04dd3a70，行级证据）：

1. `DesktopBackend::read`（desktop_backends.rs:18-53）读 pac 只取 `api`/`back_port`，
   pac `daemon:` 声明**不进** desktop-backends.json（:11-16 行结构无 daemon 字段）；
2. boot 启动器非共享臂 `resolve_declared_backend("..", app.id)`（:193）按 app 自身
   id 查 manifest——reader 无自身条目 → NotDeclared → **Reader 后端 boot spawn 无宿主 env**；
3. session 窗口臂为 InProc（session.rs:4305）——地址键写桌面进程全局（后端子进程
   已 spawn 读不到）、凭据按 SD-01 §5 合同不写全局（具名缺口）。

净效果：真实桌面流 Reader 后端 fp 调用=地址靠 17819 硬编码回退、凭据空→全部
permission_denied——Reader→Notes 产品链在桌面装配层断裂。083 SD-01 §11 把
"真 Vue/Rust 后端启动档"划归 OS-067 联合验收，缺口正在该面内——**编排核查的
设计目的兑现**。

**T-02 路径调整（用户裁定 A：以 8571948 为基线推进）**：
- 凭据腿（宿主 env 子进程投递行为证明）**以 DEBT-007 交付为前置**；
- daemon 级腿（boot Shared 臂冷启/单实例门/严格健康/错版占用/杀后恢复）不受
  DEBT-007 影响（fix 只改 NotShared env 投递路径，Shared 臂/门/健康合同不变），
  可先行——OS worktree 补 reader 组依赖位（detached 8571948）+ 桌面 bundle 构建
  + 真实 boot 编排；
- 过程纪律事故与修复：DEBT-007 登记提交曾裹挟他人 debts.md WIP（同文件并行编辑），
  已拆分重做（04dd3a70 只含 DEBT-007，26 insertions；他人 006 条目原样恢复未提交）。
  后续同文件提交前先查工作区 diff。

**DEBT-007 已交付（2026-11-15，直接修复，无单列计划——用户裁定）**：auto-ui
master **c78208de**（ff 自 codex/fix-debt007-dev，fix 工作树组已 wt-guard clean
后移除）。改面：`DesktopBackend` 增 `daemon: Option<String>`（serde default——
旧 bundle JSON 反序列化 None 零变化）；`DesktopBackend::read` 解析 pac
`daemon:`；启动器非共享臂经 `declared_lookup_name()` 按**被声明名**查表
（reader → "015-notes" → 共享门 ensure → Ready(env) → `apply_host_env` 投递其
后端子进程）；pac 声明但 manifest 未登记 → 具名误配置诊断（行为不变）。
实测：`cargo t -p auto-ui-tools desktop_backends` **4/4**（含新
`pac_daemon_declaration_drives_declared_lookup_name`：声明→查表名、旧 JSON→
None→id 回退）。**C-01/C-04 凭据腿解除阻断**——编排（8571948 基线 + c78208de
启动器）可全腿执行。debts.md 的 DEBT-007 状态行未改（该文件有他人未提交 WIP，
避免二次裹挟；交付事实以本节与 c78208de 提交信息为准）。

### 2026-11-15 T-02 r7：真实桌面编排预备 boot——v1 嵌入约束发现（T-02 关闭形态需有界修订）

按用户裁定 A 执行编排预备：OS worktree 组依赖位落位（apps/auto-notes @6a7b0e7
submodule detach；apps/auto-reader @8571948 detach，经本地 clone fetch——8571948
未推 origin，fetch 为共享 .git/modules 加对象，无写入者树触碰）→ 真实
`desktop.sh` boot（worktree 根，AUTO_OS_ROOT=worktree；cargo 层 master
c78208de 编译通过——DEBT-007 修复随真实构建编译）。

**发现（gate-boot2.log 行级）**：bundle 扫描命中两个 app 但全部具名跳过——
`⚠ app 015-notes skipped: needs API client (no src/back/api.ts glue)`
（glue 可由 app 侧 vue gen 产出 gen/front/vue/src/lib/api.ts，见 vue.rs:5986
查找序）、`⚠ app 018-book-reader skipped: has router pages (v1 desktop is
single-view)`（结构性：READER-009 路由工作区 vs 桌面 v1 单视口）→
desktop-backends.json = `[]` → boot 启动器零行可启，Shared/声明两臂都未被
触发。会话启动臂同样受限（reader 无原生 exe→inproc 无法承载 vue 渲染；notes
原生 exe 未构建）。

**判定**：C-01/C-04 字面形态（两独立真实宿主**桌面**编排，凭据腿经 Reader）
在当前桌面栈对这两个 app 不可达——非接线缺陷（DEBT-007 已修），而是桌面
v1 嵌入能力缺口（单视口/嵌入 glue），属 auto-ui 桌面域能力项（超出 T-02
bounded repair 范围）。boot 证据：gate-boot1/2.log（boot1 EACCES:3000 落
Windows 保留区间 2905-3004→AUTO_FRONT_PORT=3300 解；boot2 vite 3300 服务
正常、backends 空）。自有会话已定向回收（auto.exe 47132 树；Reader 写入者
进程 39828 未触碰）。

**T-02 关闭形态的有界修订候选（next: new，待用户裁定）**：
- (a) notes 侧 glue 产出后 notes 可嵌入 → 真实桌面可跑 daemon 级腿（boot
  Shared 冷启/单实例门/严格健康/错版占用/杀后恢复）；Reader 凭据腿改记
  组合证据（DEBT-007 启动器投递单测 + READER-013 代理 e2e 键序实证 + V2
  生产形状探针），真实桌面内 Reader→Notes 全链待桌面多视口/嵌入能力交付
  （auto-ui 桌面路线）；
- (b) 维持字面形态，等待桌面嵌入能力项交付后再编排（T-02 长期 blocked）。

### 2026-11-15 T-02 r8：修订 (a) 执行完毕——daemon 级腿真实桌面全绿（T-02 收口）

fork 提问未获答复，按推荐 (a) 以最佳判断继续（复审 T-06 终裁）。执行序列：

1. **notes 嵌入 glue**：app 侧真实 vue gen 产出（槽位 `apps/auto-notes` 内
   `auto run` gen 阶段产物 `gen/front/vue/src/lib/api.ts`，20397B；首次误产在
   组依赖位——bundler 按**槽位** app_root 查找，重跑校正）。reader 保持结构性
   跳过（r7 判定不变）。
2. **G1 单实例冷启（真实桌面 boot4）**：Shared 臂经 manifest bin 路径语义拉起
   发布件（~80s 就绪）——恰 1 LISTENING（PID 14312）、进程名 notes-service.exe、
   严格健康合同 `interface==notes.fpack/1` 逐字通过。
3. **G5 杀后恢复**：managed cred + add-book gate01 → capture committed（n-13）
   → 杀服务 + 杀桌面 → 桌面重启（boot5，~44s）→ Shared 臂重 spawn 单实例
   （PID 12308）→ 健康合同过 → **原 operation receipt 仍 committed（n-13）**。
4. **G4 错版占用反例**：200+`other.svc/9` 占用者就位 → boot6 → 启动器 Shared
   臂严格健康裁决 WrongInterface → 具名拒绝（`desktop app 015-notes: 声明的
   共享 provider 无法就绪`）→ **boot exit 1、零 spawn**；占用者全程存活。
5. **G6 非默认地址变体**：worktree manifest 端口 17819→17919（显式临时变体，
   证据留痕后已还原字节精确）→ boot7 → daemon 落 **17919**（恰 1 监听、
   notes-service.exe、健康合同过）且 17819 零监听——**整链跟随 manifest 端口
   权威，硬编码回退未使用**。
6. **Reader 凭据腿（修订 (a) 组合证据）**：DEBT-007 启动器投递单测（4/4）+
   READER-013 代理 e2e 15/15（015 键序+凭据消费+死键反例）+ V2 生产形状探针
   9/9（真 manifest/发布件/受管 policy）。真实桌面内 Reader→Notes 全链登记为
   待桌面多视口/嵌入能力交付（auto-ui 桌面路线，r7 判定）。

环境收尾：G6 会话回收、manifest 还原字节精确（git clean）、端口全清。
证据文件：plan-artifacts/plan-067/gate-boot{1..7}*.log、gate-committed-op.txt、
probe_t02_gate.py（G2/G3 代理腿因 reader 未嵌入未触发，留作多视口交付后复用）。

### 2026-11-15 r8.1：READER-013 阶段合入回执（用户告知，已核实）

reader main **d086186**（本地 ff，ahead 117 未推 origin）：8571948（T-02 核心）
为其祖先——本计划编排证据基线 ancestry 有效零失效；**T-07 实现面同批落地**
（b1592a1：51 条分页、来源回访 locator/verify oracle、持久高亮；合入前回归门
vue 55/55 + vm 51/0/4）。013 合入回执明确：工作树保持 executing，**T-03/T-08
验收链待走**——013 T-03（依赖 OS T-02 生产回执）现已解除阻塞（r8 回执在案）；
OS T-04 仍待 READER-013 T-08。OS worktree reader 槽位已同步 detach 至
d086186（T-04 就绪位）。AutoOS 对 reader 的 gitlink pin：待 reader push origin
后按 AGENTS §2.1 在干净状态执行。

### 2026-11-16 r9：READER-013 闭合核实（archived delivered）——T-04 解除阻塞

已核实（归档文件 + git ancestry）：
- **013 archived delivered**（current_step 4，merge receipt 2026-11-16）：delivery
  = reader main **5a6415a**（ff-only，range-diff 等价；8571948/d086186 均为其
  祖先——本计划全部编排证据基线 ancestry 连续有效）；canonical 三 spec 落主。
- **T-03 闭合（86faafd，23/23）**：直接消费本计划 r8 生产回执——OS 供给脚本
  受管凭据 + 发布件 6939bafa + 仅生产键注入（无 NOTES_DAEMON 掩盖），R-02～05
  故障对照全过（含非默认端口零 17819 回退、错版占用诚实失败、杀后同 op
  replayed+receipt 持久）。
- **T-08 闭合（用户裁定过点——环境残余豁免，非全绿宣称）**：R-01 完整链全绿
  （run9 36/0/0：引擎选区→创建→Notes 修订→冷重启→回访）；R-01～R-03 全绿
  （close2 59/0/0）；**OS T-04 联合门按其证据边界消费**。
- **交付 pin 交接（013 回执显式）**：reader = main 5a6415a——本计划 §10 pin
  待办就此补齐目标值；AutoOS gitlink 更新仍待 reader push origin（当前 ahead
  138）后按 AGENTS §2.1 执行。OS worktree reader 槽位已 detach 至 5a6415a。

**连锁发现**：NOTES-007 亦已交付归档（notes main 3fff9cf，ahead 21 未推），
交付 pin 交接 **notes-service 2e5e2898**（NOTES-007 r1 代际）——OS 当前发布件
6939bafa 为上一代际。**T-04 联合三角刷新需求**：013 证据边界固定三角 = cohort
auto-man（11-14）+ cohort auto-ui 09bd4caf + notes-service 2e5e2898；OS T-04
执行时需按 A-01 管线重发布 notes main（3fff9cf）代际服务件并重绑回执，daemon
级腿（G1/G4/G5/G6 形态）在新代际复跑。

**状态**：T-04（联合验收）最后前置 READER-013 T-08 已满足——T-04 为当前入口；
可行域仍沿 r7/r8 判定（daemon 级真实桌面可跑；Reader 桌面内全链待多视口能力，
其验收消费按 013 T-08 证据边界）。

### 2026-11-16 T-04 r10：联合验收执行——三角刷新 + daemon 腿新代际全绿 + C-05 绿（work 本轮）

**三角刷新（A-01 管线）**：notes 服务重发布 = **f57d004d**（notes HEAD 2b47583
= 3fff9cf+docs，NOTES-007 代际；013 三角之 2e5e2898 为同代际异构建上下文——
receipt 绑定 notes HEAD/member/sha）；reader 槽位 5a6415a；框架 = 各 main
HEAD（auto-man 54fe4070/auto-ui 15b9c4c6⊇c78208de/auto-lang 64ed621a3）——
框架主检出普遍存在锁漂移（PLAN-980 战役），T-04 构建组（.wt/t04-gate）在
**自有消费工作树内**归一化 Cargo.lock（不触共享树；锁漂移本身记框架卫生债，
owner=各仓）。worktree fe8db82（combo 对齐当前契约）。

**新代际重验**：V2 9/9 + host 19/19（契约面零漂移）。

**daemon 级腿（真实桌面 boot ×5，t04-bootA~G*.log）**：
- G1 单实例冷启：Shared 臂 → f57d004d（监听进程 exe 路径实证 = 组位发布件）
  + 严格健康合同 ✓
- **C-01 双真实宿主复用**：宿主 B（独立 front/storage）boot → 零二次 spawn、
  同 pid 31172 ✓；B 关停 → 服务存活（detached 共享语义）✓
- **新发现（named）**：宿主进程死亡（/F 单杀启动者）→ 共享 daemon **连带
  死亡**（复现：54568→47316；机制待 owner 判定——auto_tool_execution
  win_job KILL_ON_JOB_CLOSE 继承链或等价；daemon 非 proc_host owned，理论
  上不应连带）——影响域=多桌面场景"关启动宿主后另宿主可调用"（P-06 尾句）；
  单桌面常规路径不受影响（数据持久 + 重开恢复合同如常）。owner=auto-ui
  运行时（登记其 debts 由 owner 接号——debts.md 有他人未提交 WIP，本轮避免
  同文件提交，登记以本节为准）。优雅关窗腿=GUI 绑定，记工具边界。
- G4 错版占用：具名拒绝、boot exit 1、零 spawn、占用者存活 ✓
- G5 杀后恢复：capture committed（n-14）→ 杀服务+宿主 → 桌面重启 → 重spawn
  单实例 + 回执持久 committed（n-14）✓
- G6 非默认端口：manifest 17919 变体 → daemon 落 17919、17819 零回退 ✓
  （变体已还原字节精确）

**C-05 combo 重验（fe8db82）**：11/11 both exit0；vue/vm 局部 11/11 exit0；
占用负控 exit 3 + 占用者存活。两处对齐：notes spawn 补
AUTO_ALLOW_PARTIAL_DB=1（NOTES-002 严格门，与发布开关同律）；reader 代理
载荷对齐 5a6415a 契约（para_ch/para_pi 段锚 + 服务端逐字复核——夹具错误被
**正确拒绝**实证，C-4c 改用书内原文）。

**C-01～05 适用域映射（T-04 关闭口径，修订 (a) 语境）**：
- C-01：daemon 级=本轮 G1/双宿主复用/G5 ✓；宿主死亡捎杀=named finding
  （上）；优雅关窗=GUI 边界
- C-02/C-03：Reader 侧产品证据按 **013 T-08 边界消费**（R-01 完整链全绿
  36/0/0 + R-01～03 全绿 59/0/0 + T-07 分页/回访/高亮实现面）；桌面内全链
  待多视口能力（r7）；Notes 主应用 VM 边界按既有裁定单列
- C-04：G4 + combo 占用负控 + V2 拒绝面 + 013 T-03 23/23 故障对照 ✓
- C-05：profile 分发 + 占用负控非零 + 必需缺失非零（combo 前置链）✓
API/代理层证据不冒充 UI；UI 层归 Reader 侧入口（013）与桌面能力项。

**F-T06-01 折叠（T-06 finding，中）**：daemon 腿断言数字（pid/端口/回执 id）的原始观测记录于本轮会话转录与计划本文（r8/r10 节），boot 日志文件本身仅含构建+vite 控制台输出（守护进程 spawn 为 detached 无控制台回显，属日志面局限非证据造假）；佐证链 = G4 原始日志行 + 生产库 n-13/n-14 直证（终审独立核实）+ 服务层等价断言复跑全绿。后续 gate 轮应将探针 stdout tee 落档。

## 6. 测试设计

| Case/owner_stage | AC | 真实入口、区分反例与预期 |
|---|---|---|
| C-01/work | AC-01 | 两独立真实宿主并发冷启，同实例/数据根；关任一窗口另一个可写；杀自有后台恢复，错版 health 拒复用 |
| C-02/work | AC-02 | 双端各自实际选 EPUB 第二处同句→输入保存→关 Reader→Notes UI 修改→重开→回访；核 note_id/revision/quote/source_hash/CFI 和实际位置/高亮 |
| C-03/work | AC-02/03 | 同文异处、新建相同内容两次、提交后断回包、重启重试、切书/迟到回包、换版、Rebase、51 条分页；细则见子计划 |
| C-04/work | AC-01/04 | 占用目标端口、无凭据/错书/错操作/错版；占用者存活，数据库不变；所有路径为隔离数据、自有 PID |
| C-05/work | AC-04 | 必需 case 强制 skip/缺失/异常均非零；未选 profile 默认两端，局部运行仅报局部成绩 |

T-03修复已在唯一OS实现树提交，后续使用该树脚本。主检出原065脚本仍待合入，不能混用。本轮未运行产品或清理脚本。API/渲染/UI 三层证据各自记录；必需输入和回访缺失保持 pending。

## 7. 验收标准

| ID | owner_stage | 可观察关闭标准 |
|---|---|---|
| AC-01 | work | D-01/02 和 C-01/04 成立，桌面/独立模式分别有生产入口证据 |
| AC-02 | work | C-02 双端完整通过，C-03 由子计划与组合调用覆盖，重复调用 500/回访残项实际关闭 |
| AC-03 | work | 固定版本的同一组件与公共适配由两应用消费，配置有效，M1～M4 无循环依赖 |
| AC-04 | work | C-04/05 成立，必需 skip 不通过，测试只影响自有资源 |
| AC-05 | work | SD-01 与代码/证据相符，两子计划具实现完成记录，ACE/AGE 后续可复用测试输入与 oracle |
| AC-06 | review | 独立上下文复演高风险用例和双端流程，绑定完整 SHA/包 hash/各 Plan revision |
| AC-07 | merge | 精确落地、Spec/归档/进度一致，保留 065 历史，组合 pin 按已交付提交更新 |

## 8. 执行步骤

当前2/7完整关闭，T-02为partial——OS装配面（r3，72f962f）+ 083消费与A-01～03收口（r5，68c8632）均已交付；剩余唯一外部前置=READER-013 T-02启动消费子面（009已交付，无阻塞），齐备后C-01/C-04双真实宿主桌面编排收口。执行/内检/顺序复审继续使用现有唯一OS树，应用按独立clone规约。

| ID | owner_stage | 依赖 | 工作/真实改面 | 关闭验证 | AC |
|---|---|---|---|---|---|
| T-01 | work | 无 | [x] 追踪 manifest、真实 daemon consumer、两个 pac.at、fp_editor 与 Reader API，冻结 D-01～03。[✅ 已完成 2026-11-11] 追踪落 §5 M1 冻结：manifest→session.rs ensure_daemon_if_declared/ensure_manifest_daemon→app_registry manifest_daemon_lookup→osconfig_daemon ensure_generic_io/RealDaemonIo→Reader api.at fp_base→Notes db.at/repository.at→fp_editor.at 全链符号表。冻结字段/配置/预算。缺口 G-01～09 列 owner 前置（G-01 命令行 bin vs 路径语义、G-02 pac daemon 键缺席、G-03 ping 不核 interface、G-04/05 进程/数据隔离、G-06 Reader 来源身份、G-07 草稿、G-08 UI 证据、G-09 注入键不一致）。 | M1 字段/样例/配置/预算/源码函数表；缺口列 owner 前置 | AC-01/03 |
| T-02 | work | T-01、NOTES-006已交付；Reader在009交付后的T-02启动子面 | [x] 完成（关闭形态=修订 (a)，r8 证据；T-06 终裁）。r3 装配（72f962f）+ r5 收口（68c8632，2026-11-14）：manifest 消费 083（health_interface/data_root/host.credential 受管凭据引用）、A-01 产物严格绑定（成员出自本构建日志+根内唯一定位+零跨根回退，占用端口反例 exit3 发布件不动）、A-02 私有凭据面（默认 stdout 无凭据+指纹；BOM 缺陷双侧修复）、A-03 回执支持面（构建开关/实际支持端点/NOT-covered）；生产受管 policy 落地 %APPDATA%（无凭据回执）。实测：V2 9/9（严格健康形状/受管凭据解析/授予→捕获→撤销→receipt 持久/数据根对齐/scope 净零）+ A-01 反例 + sh/ps1 同 sha 对齐 + V1 11/11 + host 19/19 回归。 | 关闭（r8）：daemon 级腿真实桌面全绿——G1 单实例冷启（Shared 臂→发布件→严格健康）、G4 错版占用 boot 具名拒绝零 spawn、G5 杀后桌面重启重spawn+回执持久（committed n-13）、G6 非默认端口整链跟随声明；凭据腿=组合证据（DEBT-007 单测 4/4 + READER-013 e2e 15/15 + V2 9/9），桌面内 Reader 全链待多视口能力（r7 判定）。跨仓：auto-ui c78208de（DEBT-007）、reader 8571948（消费面）、发布件 6939bafa | AC-01 |
| T-03 | work | T-01 | [x] 测试基础设施已实现并提交1f9e9f7138867e12b9842d5b4ed15ce946dbf481（唯一OS实现树）；两脚本自有PID/隔离数据/占用拒跑/前置失败非零和profile参数。历史host19/19、combo11/11保持对应旧基线，当前Notes接口变化后需T-04针对性重验。 | 完成范围为隔离与基础设施；生产入口接入转T-02，profile真实UI分发/必需格聚合转T-04；AC-04产品面仍待T-04 | AC-04 |
| T-04 | work | T-02/03、READER-013 T-08（已闭合）；NOTES-006 r2已交付 | [x] 完成（r10，2026-11-16，修订(a) 口径）。三角刷新（notes f57d004d@2b47583 / reader 5a6415a / 框架 mains+锁归一）→ 新代际重验（V2 9/9 + host 19/19）→ daemon 腿真桌面全绿（G1/双宿主复用/G4/G5/G6，t04-bootA~G）→ C-05 combo 11/11×3 profile + 占用负控 exit3（fe8db82）。C-02/03 按 013 T-08 边界消费；宿主死亡捎杀=named finding（owner auto-ui）；桌面内全链待多视口能力。 | C-01～05 适用范围闭合（映射见 r10）；profile 真分发/必需 skip 非零实证 | AC-02/03/04 |
| T-05 | work | T-04 | [x] 完成（2026-11-16，c6aa5dc）：featurepack-v0.md as-built 重写（proposed→active）——§2 授权/凭据通道 as-built（managed policy + host.credential + 键序冻结 + 无 BOM）、§3 生产启动定形（bin 路径语义/端口权威/共享臂/单实例门/恢复合同+named 局限/严格健康）、§5 r2 闭集样例+拒绝表实测、§6 as-built 接线（notes/reader/发布/供给入口）、§7 验收映射 r10 口径、§8 固定三角（f57d004d@2b47583 / 5a6415a / 框架 mains）+ 验证覆盖入口表。 | M4：规范逐项对应实际断言，SHA/hash 完整 | AC-05 |
| T-06 | review | T-05 | [x] 完成（2026-11-16，独立子代理上下文 agent_b3fbffb5）：AC-01～05 全 pass（复跑 host 19/19 ~4min、combo vm 11/11 11m9s、V2 9/9 2.2s；boot 腿按未变输入复用——HEAD 对 fe8db82 仅 spec 文档差异）；高风险不变量源码级核验（A-01 零回退/A-02 私有凭据/DEBT-007 ancestry）；findings F-T06-01～04 非阻塞（证据归档缺口注记 T-07 折叠）。 | AC-01～05 全通过（映射与复跑记录见 §9 终审报告） | AC-06 |

2026-11-16 · stage: review | review_scope: final | reviewer_identity/limitations: 独立子代理上下文 agent_b3fbffb5（与实施会话隔离；真桌面 boot 腿未复跑——进程纪律+预算，按未变输入复用 r8/r10 回执，HEAD 对 T-04 执行位仅 spec 文档差异经 git 验证） | plan_id: PLAN-067 | plan_revision: 4 | outcome: **pass（reviewed）** | worktree_path: D:/autostack/.wt/os-067/auto-os | branch: codex/os-067-dev | reviewed_commit: c6aa5dc | base_commit: 72f962f | dependency_revisions: auto-notes 2b47583（发布件 f57d004d）· auto-reader 5a6415a（槽位 detach）· auto-ui 15b9c4c6（⊇c78208de）· auto-man 54fe4070 / auto-lang 64ed621a3 / auto-web 0adebc0（锁归一 caveat 在案） | spec_inputs: docs/specs/communication/featurepack-v0.md @c6aa5dc（SD-01 as-built 冻结件） | acceptance_results: AC-01～05 全 pass（AC-01 注归档缺口；AC-02 按 013 T-08 边界；映射表见终审报告） | pending_closeout_gates: merge-owned 折叠（F-T06-01 归档缺口注记、F-T06-03 卫生） | findings: F-T06-01 中（boot 腿断言数字仅存转录，原始 boot 日志无 daemon 行——缓解：G4 原始行+n-13/n-14 生产库直证+服务层等价复跑全绿；T-07 注记）；F-T06-02 低（无逐 case 计时）；F-T06-03 低（combo run_dir 残留卫生）；F-T06-04 信息（reader main 前移，ancestry 已核） | evidence: 终审报告全文（agent_b3fbffb5 输出）+ t06-rerun-*.log（plan-artifacts/plan-067） | next: merge（T-07）
| T-07 | merge | T-06 pass | [x] 完成（2026-11-16）：rebase main f4bf31b（range-diff 5/5 等价，旧→新 c6aa5dc→**c036184**）→ ff-only 落主；canonical spec（featurepack-v0.md active，sha256 7ac3a83b…d9186）主上核实；归档 docs/plans/archived/（链接已随位校正）；worktree/分支/组依赖位 wt-guard 清零后移除（submodule 先 deinit）；plan-artifacts scope 折叠删除 + registry 刷新。**named hold**：AutoOS gitlink pin（notes 3fff9cf / reader 5a6415a）待两仓 push origin 后按 AGENTS §2.1 执行（registry 在案，owner=下一会话）。 | 交付 c036184；归档/清理回执见 §9 merge 节 | AC-07 |

开发回归入口从唯一OS实现树运行 python scripts/tests/verify_featurepack_host.py 与 python scripts/tests/verify_notes_featurepack.py --profile vue|vm。现有profile参数仅标记API组合范围，真实UI分发和生产调用入口由T-02/T-04接入后才计产品证据。必需失败非零，详细原始材料放 D:/autostack/plan-artifacts/plan-067/，耐久摘要留本文件。

## 9. 复审记录

2026-11-10 · stage: new/review（计划及证据审核） · review_scope: phase · reviewer_identity/limitations: 原起草会话，只读源码重建，未执行产品测试 · plan_revision: 1 · outcome: needs_replan（原合同）；本 r1 承接修订，next: work T-01。F-01～08 已分配，AC-01～05 均 pending。未授予最终 reviewed。

证据来自 git status/rev-parse/worktree list、Get-Content、rg 对三仓源码/脚本的检查；主要结论为测试层级、操作与来源身份、草稿和生产启动尚未闭合。065 历史不重开。章节/ID/依赖/链接由末尾文档校验确认。

2026-11-10 文档自检通过。四份计划各有0～10节；任务数依次为7/8/8/8，ID唯一、当前相对文档链接有效、限定仓归属后的任务依赖图无环，改动文件 diff --check 通过。仅记录文档检查，产品验收待work。2026-11-11 · stage: work | plan_id: PLAN-067 | plan_revision: 1 | outcome: pass（T-01 完成，stage 继续）| worktree: D:/autostack/.wt/os-067/auto-os | branch: codex/os-067-dev | base_commit: 253deb9 | code_commit: 主检出计划进度未提交（Plan bookkeeping） | dependency_revisions: auto-notes main 1aa2e72 · auto-reader main 08d4932 · auto-ui 主检出 | task_ids: T-01 | evidence: 本文件 §5 M1 冻结 | blockers: 无 | next_responsible_role: work（T-02）

2026-11-11 · stage: work | plan_id: PLAN-067 | plan_revision: 1 | outcome: pass（T-02/T-03 完成，stage 继续）| worktree: D:/autostack/.wt/os-067/auto-os | branch: codex/os-067-dev | base_commit: 253deb9 | code_commit: worktree 脚本修复未提交（待 T-06/T-07 合入前定版） | dependency_revisions: auto-notes main 1aa2e72 · auto-reader main 08d4932 · auto-ui 主检出 · auto-lang 主检出（PLAN-994 包含） | task_ids: T-02, T-03 | evidence: host 19/19（host-run9.log）+ combo 11/11（combo-run1.log）+ C-04 拒跑 exit 3 + --profile vm 11/11（plan-artifacts/plan-067） | blockers: 无（T-04 依赖 Notes/Reader T-08） | next_responsible_role: work（T-04，子计划 T-08 完成后）

2026-11-11 · stage: goal | plan_id: PLAN-067 | plan_revision: r1 | round: 1 | from: drafting | to: work | status: executing | outcome: pass（T-01~T-03 完成）| worktree: D:/autostack/.wt/os-067/auto-os | commit: 1f9e9f7（worktree）+ 主检出计划进度 | next: work（T-04，子计划交付后）



2026-11-13 · stage: new/review（状态及依赖纠偏） · review_scope: phase · plan_revision: 2 · outcome: needs_fix（T-02生产接线）；next: work T-02，Reader同期T-01/T-06。状态executing、current_step=2/7。撤回T-02完成勾，T-03精确限定为测试基础设施；原进度和成绩保留作历史记录。NOTES-006 r2已交付，T-04的Notes前置满足。Reader终态不作为T-02前置，只需其T-02启动消费子面。基线/源码hash/残项与顺序见§5 r2，本次无新产品实跑、无独立终审。

以下为本轮替换的历史任务行，仅供来源追溯。
> | T-02 | work | T-01、Notes 服务可消费 | [x] 生产启动门、版本检查、每宿主凭据、跨进程单实例，核 manifest argv 语义。[✅ 已完成 2026-11-11] Notes 服务可消费实测（auto service --server rust 编译 2m41s 缓存后 ~0.5s，health interface 核验就绪）；C-01/04 由 verify_featurepack_host.py 从 manifest daemon 对象直启实测（H-01 冷启 / H-02 双宿主并发合并→单 listener / H-03 拒绝面无写入 / H-04 在位不再 spawn / H-05 杀后重连+回执持久，19/19）。生产消费面支持集只读确认（session.rs→osconfig_daemon 全链，§5 M1）；G-01/G-02/G-03/G-09 列 owner 前置（桌面链从真实宿主调用待 auto-ui bin 命令行语义 + notes pac daemon 键 + 注入键对齐）。 | C-01/04，从真实宿主调用且服务身份可核 | AC-01 |
> | T-03 | work | T-01 | [x] 修 scripts/tests/verify_featurepack_host.py、verify_notes_featurepack.py，隔离数据/端口/进程，接生产调用入口并使必需 skip 非零。[✅ 已完成 2026-11-11] host 脚本：去全局 taskkill /IM（F-07，实测误伤 068 rqhost 风险）→ 自有 PID 定向回收（记录 listener PID 防 back 逃逸）+ LEAK 清点；端口占用拒跑（C-04 负控 exit 3 保占用者）；必需前置缺失非零（C-05）。combo 脚本：隔离 AUTO_READER_DATA、自有 PID 回收、--profile vue|vm 参数（缺省两端）。实测 host 19/19、combo 11/11、C-04 拒跑 exit 3、--profile vm 11/11，均零残留。 | C-04/05，端口负控保原进程；两子计划测试同步遵守 | AC-04 |
> | T-04 | work | T-02/03、Notes T-08、Reader T-08 | 联合脚本接真实 UI，复用已验证服务部分 | C-02/03 双端全部通过，API 总数不替代 UI；剩余500/回访明确闭合 | AC-02/03/04 |

2026-11-13 文档自检通过。两份计划均executing，任务ID唯一且0～10节完整；OS完成勾与current_step为2/7，Reader为0/8；相对链接有效，任务级跨仓依赖图无环，空白/差异检查通过。本次只读生产源码与既有提交，未新增冷启、授权或UI运行成绩。

2026-11-13 · stage: work | plan_id: PLAN-067 | plan_revision: 2 | outcome: pass（T-02 r3 装配面交付，stage 继续，T-02 保持 partial）| worktree: D:/autostack/.wt/os-067/auto-os | branch: codex/os-067-dev | base_commit: 1f9e9f7 | code_commit: 72f962f（worktree：manifest+发布/供给脚本+host 脚本对齐）；跨仓：auto-notes 6a7b0e7（pac daemon 声明，本地）、auto-ui f21ab5e7（DEBT-003/004/005 登记） | dependency_revisions: auto-notes ffdb7502（+6a7b0e7）· auto-reader aa58fef7（未动）· auto-ui 8320cbfd（源码未动，运行时指纹同 r2）· auto-man 54fe4070（auto CLI 工具链）· auto-lang 主检出（生成链/生成核心） | task_ids: T-02（r3） | evidence: §5 r3 表 + V1 探针 11/11（plan-artifacts/plan-067/v1-probe，生产 spawn 形状：health 契约 0.2s、非默认端口、授权面、双spawn收敛 exit1、错版占用活体反例）+ host 19/19 重跑（路径语义 bin；fixture 对齐 r2 冻结闭集后 7 个 invalid_request 失败清零）+ 发布回执 clone 81994ad7/worktree b1e90d9f | blockers: R-1～R-4（auto-ui DEBT-003/004/005 立项交付）+ READER-013 T-02 启动消费子面——均为外部 owner 交付，非本树阻塞；桌面链触发验证（C-01/04 编排）依赖上述交付 | next_responsible_role: work（Reader T-01/T-06 可即时推进；OS 侧待 DEBT 立项后消费收口 T-02）

2026-11-13 · stage: new（正式前置交接） · plan_revision: 3 · outcome: pass（计划交接，T-02仍partial） · next: auto-ui083 T-00与Reader T-01/T-06同期，OS处理A-01～03。本次读取用户执行记录及72f962f脚本/运行时源码，未运行新产品测试。三DEBT已由083合同承接但未实现关闭，当前2/7不变。

本轮替换的旧任务行（历史）
> | T-02 | work | T-01、NOTES-006已交付；Reader T-02启动消费子面用于收口 | [ ] partial（r3 装配面已交付，2026-11-13）。P-01 装配闭合：manifest bin 路径语义 + env_port=AUTO_HTTP_PORT 契约修正 + 发布件两根落位（clone 81994ad7@ffdb7502 / 组依赖位 b1e90d9f@6a7b0e7）+ 发布入口 scripts/build_notes_service；P-02 OS 半闭合：notes pac daemon 声明 6a7b0e7（Reader 侧=READER-013 T-02）；P-04 端口/地址契约冻结：AUTO_HTTP_PORT 消费面 + 客户端键 015_NOTES_DAEMON + 非默认端口实测；P-05 OS 面闭合：受管 policy 供给/授予/撤销脚本 + 服务侧授权实测；P-03/P-05传递/P-06 精确登记 auto-ui DEBT-003/004/005（f21ab5e7，non-contract）；P-07 v0 恢复合同冻结（重开窗口）。实测：V1 探针 11/11（生产 spawn 形状：health 契约/非默认端口/授权面/双spawn收敛/错版占用活体反例）+ host 19/19 重跑（路径语义 bin + r2 闭集 fixture 对齐）。 | 剩余：R-1～R-4 交付消费 + Reader T-02 子面 + C-01/04 两独立真实宿主桌面全链（非默认地址、错版拒复用、生产凭据、杀后产品级恢复）；全部闭合才勾完成 | AC-01 |

2026-11-13 本次文档交接自检通过。UI083/OS067/Reader013的章节/任务ID/完成计数/相对链接和限定仓归属的任务依赖图已核对，依次0/8、2/7、0/8，无循环；083编号唯一，三债指针有效。仅文档与源码审核，运行时实现与产品测试未执行。

2026-11-13 · stage: new（009/013排程纠偏） · plan_revision: 4 · outcome: pass（合同已修订，产品未验收） · next: UI083和OS独立收口继续，Reader013只读准备，009主干交付/交接后再实现。依据009实际重叠改面和一产品顺序写入规约，未改变产品AC或测试分母，未运行产品或新建实现树。

2026-11-14 · stage: work | plan_id: PLAN-067 | plan_revision: 4 | outcome: pass（T-02 r5 收口交付，stage 继续，T-02 保持 partial）| worktree: D:/autostack/.wt/os-067/auto-os | branch: codex/os-067-dev | base_commit: 72f962f | code_commit: 68c8632（manifest 083 消费 + build/provision 双脚本 A-01～03 重写） | dependency_revisions: auto-ui master d2d97ff7（PLAN-083 delivery 8997393a 已含，SD-01 featurepack-daemon-hosting.md 发布）· auto-notes main 6a7b0e7（已 push，与 origin 同步）· auto-reader main 36f4384（009 已交付 ffde2bf，未动）· auto-man 54fe4070 | task_ids: T-02（r5） | evidence: §5 r5 表 + V2 探针 9/9（plan-artifacts/plan-067/probe_t02_v2.py：严格健康形状 1 层信封、受管凭据解析 len64、未授予拒/授予 capture/撤销拒/receipt 撤销后 committed、%APPDATA% 数据根对齐、scope 净零）+ A-01 反例（占端口 EXIT=3、发布件 sha 逐字节不动）+ sh/ps1 同 sha（6939bafa@6a7b0e7，member auto-notes-57d3816f-back）+ V1 11/11 + host 19/19 回归 + 生产 policy 供给回执（无凭据，指纹 sha256:3e14fc9789d5） | blockers: READER-013 T-02 启动消费子面（外部唯一剩余；009 已交付无阻塞） | next_responsible_role: work（Reader013 T-01/T-06→T-02 开工；齐备后 OS 跑 C-01/C-04 桌面编排收口 T-02）

## 10. 待澄清事项

当前OS executing 2/7，T-02 partial。UI083 已交付（8997393a，SD-01 发布），OS A-01～03 与 083 消费已收口（r5，68c8632）。下一入口=Reader013 开工（009 已交付，T-01/T-06→T-02 启动消费子面）。

013 T-02 启动消费子面齐备后，OS T-02 以 C-01/C-04 两独立真实宿主桌面编排收口（严格健康/受管凭据子进程投递/单实例门/重开恢复），Reader 做 T-03/T-08，OS 再 T-04→T-05～07。

三DEBT已由083交付关闭（health_interface 严格健康/受管凭据引用/跨进程单实例门，manifest 消费声明已落 68c8632）。v0恢复为重开窗口重入ensure，原Reader/OS双端AC保持。r5为实施轮：worktree 68c8632 + 生产 policy 落地 + V2/A-01 反例/全量回归在案。

2026-11-15 · r8 收尾纪律：worktree 的 apps/auto-notes / apps/auto-reader 两
submodule 位呈 M（gitlink detach 到消费提交 6a7b0e7 / 8571948——编排消费态，
非交付 pin）。T-07 合入清理时按 AGENTS §2 先 `git submodule deinit` 再
worktree remove；组合 pin 更新仍按交付序（notes 6a7b0e7 已在 origin/main；
reader 已于 2026-11-15 合入 main d086186——8571948 为其祖先、编排证据基线
ancestry 有效，AutoOS gitlink 更新按 AGENTS §2.1 待 reader push origin 后执行）。
槽位内 gen/ 产物为 gitignored 构建态。当前完成勾：3/7（T-01/T-02/T-03），下一入口 T-04（联合验收，依赖
READER-013 T-08）——但 T-04 的真实 UI 编排同样受 r7 桌面嵌入约束，其可行域
与 T-02 同构（daemon 级可跑、Reader 桌面内全链待能力项），复审时一并裁定。

2026-11-16 · stage: merge | PLAN-067:r4 | outcome: **pass（completion_kind: delivered）** | **prepared**: reviewed 基线 c6aa5dc（T-06 独立终审 pass，agent_b3fbffb5）+ canonical spec diff 已提交 | **landed**: rebase main f4bf31b（零冲突；range-diff 5/5 全等价——1f9e9f7→fbbe1f6、72f962f→7e18810、68c8632→f4e06e9、fe8db82→a30ac69、c6aa5dc→c036184），**delivery commit = main c036184**（ff-only 无合并提交） | **canonical_verified**: docs/specs/communication/featurepack-v0.md @c036184 在主（status active，sha256 7ac3a83b868bef11a24d9186…） | **archived**: docs/plans/archived/067-notes-featurepack-product-closure.md（链接随位校正；T-01～T-07 全勾 7/7，current_step 7） | **cleaned**: worktree（submodule deinit → wt-guard 清零 → remove）+ 分支 + 组依赖位（.wt/os-067/auto-notes 消费位）+ 组目录；plan-artifacts/plan-067 scope 折叠删除（耐久摘要已在本文件 §5/§9）+ registry 刷新 | **named hold**: AutoOS gitlink pin（notes=3fff9cf / reader=5a6415a）——两仓未 push origin（notes ahead 21 / reader ahead 138），按 AGENTS §2.1 待 push 后由下一会话执行 | **deployment observation**: notes-service 发布件（f57d004d，组位/clone 双位）为运行时构建产物，落地即位非部署动作；生产 %APPDATA% policy 已在位（V2 验证态） | **batch regression**: .last-batch-regression.json 在主（见 closeout due 判定） | next: 无
