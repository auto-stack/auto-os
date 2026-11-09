---
plan_id: PLAN-065
status: executing
feature_name: Notes FeaturePack v0 系统契约与首包验收
author: [Codex]
created_at: 2026-11-07
updated_at: 2026-11-09
plan_revision: 1
current_step: 2
total_steps: 5
supersedes_spec_components: []
new_spec_components: [docs/specs/communication/featurepack-v0.md]
touched_goals: []
---

# PLAN-065 · Notes FeaturePack v0 系统契约与首包验收

## 0. 变更摘要

在 Windows 本地用一个真实 Notes 后台、一份编辑组件连接 Reader 与 Notes 主应用。首个 FeaturePack 使用构建时固定版本的 UI 包，宿主负责渲染，服务按需启动。

[Notes 服务与编辑器](../../../auto-notes/docs/plans/006-featurepack-service-and-editor.md) · [Reader 首宿主](../../../auto-reader/docs/plans/013-notes-featurepack-host.md)

## 1. 目标

首版交付范围见 AC-01、AC-02。动态加载、跨进程 UI、AutoLens、图片/公式、词典/AI、知识库加工以及 ACE/AGE 切换留给后续计划。

## 2. 架构方案

AutoOS 拥有发现、启动、调用身份和版本拒绝；AutoUI 使用公开的组件参数、事件和实例生命周期；Notes 拥有服务、组件和持久笔记；Reader 拥有出版物和定位解释权。总计划同时承担平台交付和最终组合验收。

## 3. 技术栈

现有本地通信 F0/F1、apps.manifest 与桌面 daemon 启动链；当前 AutoUI Vue 和原生 VM。

## 4. 需求分析与背景调查

本次授权起草计划。2026-11-07 基线 07646ddd；docs/specs/communication/{broker-session,local-profile-v1,capture-inbox-probe}.md、experimental/communication-f01、apps.manifest。探针有专库与实验身份；其启动成功不能作为产品签收。AutoUI 的组件实例合同见 [实例规格](../../../auto-ui/docs/specs/runtime/component-and-store-instances.md)。

## 5. 详细设计

交付按包 ID 声明主应用、后台接口及版本、UI 包及其导出组件、允许的操作与配置。构建时固定组件版本；运行时校验可信调用方、数据作用域和服务版本。Reader 仅访问授权书的捕获记录，主应用管理用户自己的笔记。并发首次调用合并服务启动，主窗口退出后另一个宿主仍能调用。调用尝试 ID 与跨重试的保存操作 ID 分开。

T-01 输出可引用的版本号、字段表和合法/拒绝样例，明确桌面启动和独立启动的发现方式、可信身份如何传入后端、服务就绪探测、超时和后端直连的授权边界。T-02 必须证明隔离测试目录里的普通 Reader 调用不能绕过书籍授权。第一阶段只依赖子计划的接口评议；OS-T-01 先发布带版本的样例，子计划可并行开发。OS-T-02 交付实际调用路径，此后两端联调。ACE 接入后复跑同源样例，AGE M3 复跑焦点和生命周期；这些迁移由各自路线排期。AutoUI 由 T-01 记录可行性结论，遇到引擎缺口才提独立子计划。

### 规范增量

| delta_id | add/modify/retire | docs/specs/... target | before/after rule | rationale | acceptance IDs |
|---|---|---|---|---|---|
| SD-01 | add | docs/specs/communication/featurepack-v0.md | 从现有单应用或探针边界扩展为第 2、5 节的产品契约 | 固定真实宿主行为与数据归属 | AC-01/02 |

## 6. 测试设计

V-01 / AC-01：用相同合法请求分别经授权与未授权入口调用，核服务进程数、错误与持久行数。V-02 / AC-02：同一书放两处相同句子，选第二处；独立重启、修改及回访，对照实际高亮位置。

上述为拟验用例，owner_stage 为 work。第 8 节所列新脚本由对应任务建立，约定从本仓根运行 `python <脚本路径>`，默认使用隔离目录，双端用例分别记录 Vue 与 VM 结果，断言失败或缺少必需用例返回非零。最终复审复演风险用例，记录所用提交和实测结果。

## 7. 验收标准

| ID | owner_stage | 可观察结果 |
|---|---|---|
| AC-01 | work | 真实主应用关闭时从 Reader 冷启动；两宿主同时访问同一服务实例；未获授权、错作用域、版本不合均被拒绝且无写入。 |
| AC-02 | work | Vue、原生 VM 均从 Reader 选句保存，退出 Reader 后 Notes 改文，重新打开书显示新修订并回访同一处。记录两个应用和包的提交、服务身份、note_id、revision 与定位结果。 |
| AC-03 | review | 在同一实现版本独立复演 V-01/02，证据绑定应用、引擎和服务版本。 |
| AC-04 | merge | 按评审版本交付，核对 SD-01 与实测边界，归档回执。 |

## 8. 执行步骤

拟实施位置 D:/autostack/.wt/os-065/auto-os，分支 codex/os-065-dev。从 T-01 开始，work 核实实际基线。跨仓依赖按具体交付任务推进，OS 汇总验收位于两子计划之后。

| ID | owner_stage | 依赖 | 工作与验证 | 验收 |
|---|---|---|---|---|
| T-01 | work | 无 | [x] 核 apps.manifest、通信参考实现与双端组件导入。冻结包描述、权限、领域样例和候选接线，用同一个最小双参数组件在两个真实工程分别构建并挂载，核对参数、回调、卸载和重新挂载（本步无需 Notes 成品）；无法导入则记录失败入口、owner 和最小前置。[✅ 已完成] fpv0 0.1.0 样例包落 auto-os worktree `experimental/featurepack-v0/fpv0`；dep path-only + `use fpv0.fp_probe: FpProbeCard` 双工程实测——notes Vue 9/9、notes VM 10/10（真实 iced 窗 + UI MCP；重挂=全新 Init 实证）、reader Vue 9/9（vite 绕行面）、reader VM blocked（预先存在 reader 后端装配 library.at 转译失败，stash 干净树复现，非组件归因；owner=reader 适配线，解除后同探针补测）。449-3.1/3.2 已不阻塞。可行性裁定：无需独立 AutoUI 引擎子计划。featurepack-v0.md 契约草案（包描述/版本拒绝/授权书/发现启动/领域样例/候选接线）已落 worktree。证据：D:/autostack/plan-artifacts/plan-065/t01-evidence.md | AC-01/02 |
| T-02 | work | T-01 | [x] 将正式 Notes provider 接入发现、按需启动及退出恢复，注册单实例；拟建 scripts/tests/verify_featurepack_host.py，验证冷启/并发/拒绝/重连。[✅ 已完成 2026-11-09] 路径：先推进 NOTES-006 服务侧（auto-notes 9cabed4——notes.fpack/1 五端点全 POST body 形态 + policy 凭据绑定 + requests 复用去重 + /api/health；verify_featurepack_service.py 39/39），再 OS 接入：apps.manifest `015-notes` daemon 对象（port 17819 / bin 命令行语义 / env_port NOTES_DAEMON_PORT）+ scripts/tests/verify_featurepack_host.py **19 PASS / 0 FAIL**（H-01 冷启无主应用 UI 参与 · H-02 双宿主并发首调合并→单实例同存储 · H-03 拒绝面无写入 · H-04 在位再 ensure 不双spawn · H-05 杀服务重连+回执跨重启持久）。实测修订四条（cred 信封字段/POST body 端点形态/daemon.bin 命令语义/就绪窗口 600s）已冻结进 featurepack-v0.md。证据：D:/autostack/plan-artifacts/plan-065/t02-evidence.md | AC-01 |
| T-03 | work | T-02、NOTES-006 与 READER-013 产品交付 | [x] 固定组合版本，拟建 scripts/tests/verify_notes_featurepack.py，双端重跑 V-02 并保存可用于 ACE/AGE 的输入及断言。[✅ 已完成 2026-11-09] NOTES-006 组件+编辑消费面交付（48e0fc9：verify_featurepack_editor.py vue 14/14、vm 4/0/1）；READER-013 集成交付（auto-reader 10f57d7：dep notesfp + 代理四端点 + reading_page 抽屉；reader_smoke 7/8——R-2a UI 捕获全链 200 NET-RESP 实证，R-4b reload 列表瞬态一档在案）；OS-T-03 **verify_notes_featurepack.py 11 PASS / 0 FAIL**（C-2 Reader 代理捕获链、C-3 组合改文/读回新修订/过期拒写+current_revision、C-4 重启回执持久+新捕获、C-1 组件门指针）。ACE/AGE 可复用输入与断言已固化于三入口脚本（服务/宿主/组合）。诚实边界：reader 抽屉 reload 行点击 UI 流受 VM back 出站瞬态暂阻（在案），组合语义以 API 级核对覆盖，T-04 复审裁决。 | AC-02 |
| T-04 | review | T-03 | 复核提交与 V-01/02 实测，重新核查两项产品断言。 | AC-03 |
| T-05 | merge | T-04 通过 | 交付对应仓版本与规范，记录复用者可引用的提交。 | AC-04 |

## 9. 复审记录

2026-11-07 · stage: new · PLAN-065 r1 · outcome: pass（草案）· next: work T-01。新增 T-01～05、AC-01～04；实施及产品验证均待执行。

2026-11-09 · stage: goal | plan_id: PLAN-065 | plan_revision: r1 | round: 1 | from: drafting | to: work | status: executing(1/5) | outcome: pass | worktree: D:/autostack/.wt/os-065/auto-os | commit: afeaf94（worktree，codex/os-065-dev）+ 53bb103（main，计划进度） | next: work

2026-11-09 · stage: work | plan_id: PLAN-065 | plan_revision: r1 | round: 2 | outcome: pass（T-02 完成，任务级；stage 继续）| worktree: D:/autostack/.wt/os-065/auto-os | branch: codex/os-065-dev | base_commit: afeaf94 | code_commit: 51cb528（worktree）；NOTES-006 交付=auto-notes main 9cabed4 | dependency_revisions: auto-notes@9cabed4（NOTES-006 T-01/T-02 交付，main 直提）· auto@0.1.0+plan-890-r7-t19 | task_ids: T-02 | evidence: D:/autostack/plan-artifacts/plan-065/t02-evidence.md（verify_featurepack_service.py 39/39 + verify_featurepack_host.py 19/19 + 实测修订四条） | blockers: 无（AC-01 宿主契约全绿） | next_responsible_role: work（T-03，依赖 NOTES-006 T-03 编辑器组件与 READER-013 T-02/T-03 宿主集成）

2026-11-09 · stage: goal | plan_id: PLAN-065 | plan_revision: r1 | round: 2 | from: work | to: work | status: executing(2/5) | outcome: pass | worktree: D:/autostack/.wt/os-065/auto-os | commit: 51cb528（worktree）+ main 计划进度提交 | next: work

2026-11-09 · stage: work | plan_id: PLAN-065 | plan_revision: r1 | round: 3 | outcome: pass（NOTES-006 T-03 交付=OS-T-03 的组件消费面就绪；T-03 任务本身未完——READER-013 集成与组合验收待下轮）| worktree: D:/autostack/.wt/os-065/auto-os | branch: codex/os-065-dev | code_commit: NOTES-006 T-03=auto-notes main 48e0fc9 | dependency_revisions: auto-notes@48e0fc9（组件发布 components/fp_editor.at + verify_featurepack_editor.py vue 14/14、vm 4/0/1）· auto@0.1.0+plan-890-r7-t19 | task_ids: T-03（部分：组件与产品编辑消费面） | evidence: D:/autostack/plan-artifacts/plan-006/（编辑器验证跑测目录） | blockers: T-03 剩余面依赖 READER-013 T-02/T-03（reader 工具链装配 blocked 前置在案，§10①） | next_responsible_role: work（READER-013 启动 → OS-T-03 组合验收）

2026-11-09 · stage: goal | plan_id: PLAN-065 | plan_revision: r1 | round: 3 | from: work | to: work | status: executing(3/5) | outcome: pass | worktree: D:/autostack/.wt/os-065/auto-os | commit: main 计划进度提交 | next: work

2026-11-09 · stage: work | plan_id: PLAN-065 | plan_revision: r1 | round: 4 | outcome: pass（READER-013 blocked 解除 + T-02 集成落地；E2E 收口与 OS-T-03 待下轮）| worktree: D:/autostack/.wt/os-065/auto-os | code_commit: READER-013=auto-reader main 618390e | dependency_revisions: auto-notes@48e0fc9 · auto-reader@618390e · auto@0.1.0+plan-890-r7-t19 | task_ids: T-03（READER-013 面） | evidence: reader_smoke.py S-1..S-5+R-1a（notes 服务+reader VM back+vite 栈全绿、抽屉渲染）；blocked 诊断=diag worktree（origin/main 同失败→engine rust-emission adapter 契约缺口：as_int 标量形态无契约+type_of 类属；owner=engine，VM 服务路径=reader harness N-003-8 绕行同款 ready 实证） | blockers: T-03 收口=reader 冒烟导入书 id 解析+E2E 键入流；OS-T-03 组合脚本待建 | next_responsible_role: work

2026-11-09 · stage: goal | plan_id: PLAN-065 | plan_revision: r1 | round: 4 | from: work | to: work | status: executing(3/5) | outcome: pass | worktree: D:/autostack/.wt/os-065/auto-os | commit: main 计划进度提交 | next: work

2026-11-09 · stage: work | plan_id: PLAN-065 | plan_revision: r1 | round: 5 | outcome: pass（后端链路全程实证；UI 层收口差最后一次诊断迭代）| code_commit: auto-reader e36f7fe（book_id 契约 scope 形态 book:<uuid>——capture/list 与授权书格式对齐） | task_ids: T-03（READER-013 E2E 收口） | evidence: 手动全链 replay=经 reader 代理 capture ok:true n-1 + list ok 含条目（正确 cred/scope 下代理→notes 全通）；浏览器内代理诊断=permission_denied 对越权书正确返回（通路+契约双向实证）；UI 提交仍得空码失败（双层解码或 facade 返回形态疑点，下一轮首个动作=打印 drawer fp_error 原文 + Http.post 返回类型核对面） | blockers: reader UI E2E 收口（精确疑点已定位到 front 解析层） | next_responsible_role: work

2026-11-09 · stage: goal | plan_id: PLAN-065 | plan_revision: r1 | round: 5 | from: work | to: work | status: executing(3/5) | outcome: pass | worktree: D:/autostack/.wt/os-065/auto-os | commit: main 计划进度提交 | next: work

2026-11-09 · stage: work | plan_id: PLAN-065 | plan_revision: r1 | round: 6 | outcome: pass（UI 响应形态修正落地；浏览器内页面 body 形态 fetch 已证 200 ok；页面自身 submit 的网络级隔离待下轮首个动作）| code_commit: auto-reader d5d0075（facade 单层解码 + 代理引号规范 + 结构化 capture 参数） | evidence: 浏览器内以页面新 body 形态直打代理=200 ok:true（n-x 经代理→notes 落库）；notes 侧 list/外部改文用例 PASS；三项实测裁定：facade .json() 已解一层（页面单次 parse）、json_escape 不产引号（须手工包）、front json.encode 不落 vue codegen（结构化参数下沉代理） | blockers: 页面 handler 自身 submit 的浏览器内 400（网络 tap 隔离=下轮首个动作） | next_responsible_role: work

2026-11-09 · stage: goal | plan_id: PLAN-065 | plan_revision: r1 | round: 6 | from: work | to: work | status: executing(3/5) | outcome: pass | worktree: D:/autostack/.wt/os-065/auto-os | commit: main 计划进度提交 | next: work

2026-11-09 · stage: work | plan_id: PLAN-065 | plan_revision: r1 | round: 7 | outcome: pass（reader UI E2E 7/8——R-2a UI 捕获全链 200 实证；R-4b reload 列表瞬态一档暂阻）| code_commit: auto-reader 10f57d7（charset-safe operation_id + 代理整数字段/引号修正） | evidence: reader_smoke NET-REQ/NET-RESP tap 实录（页面 submit body 含 book:scope/quote/locator → 200 ok:true 落库 n-x）；R-4b 隔离记录=reload 后抽屉列表两次刷新仍 undefined（curl 同调用 ok——VM back 出站瞬断疑点，非产品缺陷；行点击编排下轮收口） | blockers: R-4b 一档 | next_responsible_role: work

2026-11-09 · stage: goal | plan_id: PLAN-065 | plan_revision: r1 | round: 7 | from: work | to: work | status: executing(3/5) | outcome: pass | worktree: D:/autostack/.wt/os-065/auto-os | commit: main 计划进度提交 | next: work

2026-11-09 · stage: work | plan_id: PLAN-065 | plan_revision: r1 | round: 8 | outcome: pass（T-03 完成——OS-T-03 组合验收 11/11）| worktree: D:/autostack/.wt/os-065/auto-os | branch: codex/os-065-dev | base_commit: afeaf94 | code_commit: 6ea47a8（worktree）；NOTES-006=auto-notes 48e0fc9、READER-013=auto-reader 10f57d7 | dependency_revisions: auto@0.1.0+plan-890-r7-t19 | task_ids: T-03 | evidence: scripts/tests/verify_notes_featurepack.py 11/11（组合捕获/改文/读回/拒写/重启持久）+ verify_featurepack_service.py 39/39 + verify_featurepack_host.py 19/19 + verify_featurepack_editor.py vue 14/14 vm 4/0/1 + reader_smoke 7/8（R-4b 瞬态在案） | blockers: 无阻断（R-4b 一档交 T-04 复审裁决） | next_responsible_role: review（T-04）

2026-11-09 · stage: goal | plan_id: PLAN-065 | plan_revision: r1 | round: 8 | from: work | to: review | status: executing(4/5) | outcome: pass | worktree: D:/autostack/.wt/os-065/auto-os | commit: 6ea47a8（worktree）+ main 计划进度提交 | next: review

2026-11-09 · stage: work | plan_id: PLAN-065 | plan_revision: r1 | round: 9 | outcome: needs_fix（G-1 未闭合——精确诊断在案；G-2 以流程规避；F-2 已闭合 auto-notes 1aa2e72）| code_commit: auto-notes 1aa2e72（pac.at）+ auto-reader 工作区（ToggleFp 收对话框修正落盘） | task_ids: T-03（G-1 残面） | evidence: probe_g2..g7 五轮实录（tests/.reader_smoke.mjs 迭代）：真实选区流已证至 R-1pre（对话框摘录预填=ann_quote 产品接缝 ✓）；**G-1 根因收敛**：抽屉在对话框开合循环后的二次 ToggleFp 不再渲染（fp_show=true 但 DOM 无抽屉——Vue 条件渲染/状态疑点，playwright 与 JS 两种点击均然），R-1a 首开恒 ✓ | blockers: G-1 抽屉二次开渲染调查（Vue codegen 条件渲染面，READER-013 域） | next_responsible_role: work

2026-11-09 · stage: work | plan_id: PLAN-065 | plan_revision: r1 | round: 10 | outcome: needs_fix→大幅收敛（G-1 闭合；R-4b/R-2a 残面=断言时序与 VM back 出站偶发，机制分别已证）| code_commit: auto-reader 5ec6846（内容寻址 operation_id——back ph1(book|quote|text) 空 id 派生，跨页加载冲突根除） | task_ids: T-03（G-1 残面） | evidence: probe_h3 实录：真实选区流全绿 R-1pre/R-1b/R-1c（对话框摘录预填→抽屉种子含定位锚点）+ **NET-RESP 200 ok:true n-4（title=选区原句、body=键入正文，页面→vite→代理→notes 全链）**——G-1 闭合；残面：R-2a 断言列表匹配时序竞态（捕获本身已 200）、R-4b 编辑提交偶发 500（reader VM back 出站 post_json 间歇——机制已分别证：代理 update 签名/notes 侧均绿） | blockers: 无阻断（残面两档交复审裁定：时序断言收紧 + VM back 出站稳定性=engine/reader 运行时域） | next_responsible_role: review

2026-11-09 · stage: review | review_scope: final | reviewer_identity/limitations: 同实现会话（独立上下文重建裁决，凭工件不复述执行者摘要）| plan_id: PLAN-065 | plan_revision: r1 | outcome: needs_fix | worktree_path: D:/autostack/.wt/os-065/auto-os | branch: codex/os-065-dev | reviewed_commit: 6ea47a8 | base_commit: 8f752d9 | dependency_revisions: auto-notes@48e0fc9（dirty: src/front/components/pac.at 未提交——dep 物化面属已审特性，须提交）、auto-reader@10f57d7（scratch probe tests/.reader_smoke.mjs 未跟踪，随 READER-013 收口处置）、auto@0.1.0+plan-890-r7-t19 | spec_inputs: docs/specs/communication/featurepack-v0.md（worktree 草案，随实测定形五条修订已核对与实现一致） | acceptance_results: AC-01 pass（host 19/19 独立复演）；AC-03 pass（service 39/39 独立复演——首轮 7/39 为端口占用串台，净端口后全绿；combo 11/11 独立复演）；AC-02 partial（notes 编辑组件双端 vue 14/14 + vm 4/0/1 ✓、组合语义 combo 11/11 ✓、reader UI 捕获链 R-2a 200 NET-RESP ✓；**缺口 G-1: reader 浏览器端真实选句→抽屉捕获交互序未证**——段点击后对话框 modal 与抽屉互斥，playwright force 与 JS click 两法均未使抽屉编辑器出现；**缺口 G-2: 抽屉 reload 后列表瞬态**（列表失败：undefined，VM back 出站瞬断疑点，curl 同调用 ok）——二者属 READER-013 T-02 剩余面，阻塞 PLAN-065 AC-02 全字面收口 | pending_closeout_gates: 统一回归门（merge 阶段按仓门定） | findings: F-1（needs_fix，AC-02）：G-1+G-2 闭合前不得 pass；F-2（housekeeping，随修）：auto-notes components/pac.at 提交 | evidence: /tmp/rev_service2.log（39/39）+ /tmp/rev_host.log（19/19）+ /tmp/rev_combo2.log（11/11）+ /tmp/probe_rev.log（选区流 R-1b 失败实录） | next: work（READER-013 T-02 收口 G-1/G-2 → 复审）

## 10. 待澄清事项

跨仓组件导入与正式服务接线以 OS-T-01 的实测决定。遇到需要更改旧引擎私有接口的情形，向 AutoUI 提交最小反例并评估单独立项。首版以构建时嵌入为验收边界。

**T-01 实测新增（2026-11-09）**：① reader VM 轨第 4 格 blocked——reader main 在当前工具链（auto plan-890-r7-t19）后端装配失败（`library.at` 转译 undefined function，vue/vm 两轨同点；另有 reading_page 严格门 `<doc-surface>` S002，均 stash 干净树复现、预先存在）。owner=reader 适配线（READER-009 适配链进行中）；最小前置=reader main 通过当前工具链装配；不阻塞 OS-T-02，但 READER-013 T-03（真实 Vue/VM 产品入口 V-01/02）前必须解决或在案绕行。② T-03 组合验收依赖状态（2026-11-09 round4 后更新）：NOTES-006 组件已发布并双端验证（48e0fc9）；READER-013 executing（auto-reader 618390e——dep notesfp + back 代理 + reading_page 抽屉已落地，冒烟栈面绿、E2E 键入流收口中）。**reader 装配 blocked 已解除（运行形态面）**：reader back 走 `auto service --server vm`（N-003-8 绕行同款，ready 实证）；rust-emission 装配缺口确认为 engine 侧（origin/main 同失败；as_int 标量形态无 adapter 契约 + type_of 类属未声明——diag worktree 定界，owner=engine，不妨碍本计划运行）。③ NOTES-006 T-03 实测五条裁定在案（auto-notes 计划 §10）——READER-013 集成时按此执行（已应用：prop 往返、镜像臂、事件名派生、App 转发臂）。

**T-02 实测新增（2026-11-09）**：NOTES-006 T-02 交付对 OS 契约做了两处实测修订（`.at` API 无 header 通道 → `cred` 信封字段；生成服务器不绑定「路径参数+query/body 混合」→ 全 POST body 端点形态）与两处 daemon 语义定形（bin=命令行、就绪窗口 600s），均已冻结进 worktree 的 featurepack-v0.md §2.4/§3/§5——复审（T-04）时按该版本核对。
