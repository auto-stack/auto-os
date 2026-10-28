---
plan_id: PLAN-051
status: archived
feature_name: 三仓迁移后的桌面启动脚本与候选产物入口
author: [Codex]
created_at: 2026-10-28
updated_at: 2026-10-28
plan_revision: 1
current_step: 6
total_steps: 7
supersedes_spec_components: []
new_spec_components: [docs/specs/shell/desktop-bootstrap.md]
touched_goals: []
affects: [desktop-bootstrap, scripts]
depends_on: []
---

# [PLAN-051] 三仓迁移后的桌面启动入口

## 0. 变更摘要

修改scripts/desktop.ps1与desktop.sh：显式识别auto-lang/auto-ui/auto-man，构建统一目录的CLI/UI工具/VM桌面宿主；新增候选产物目录启动模式。保留vue默认轨、iced、fullscreen、dry-run和OS/apps/mpv等原环境行为，失败非零，禁止失败后启动旧产物。

## 1. 目标

用户2026-10-28明确请求修改原脚本以支持迁移后的auto-ui/auto-man。本次授权覆盖该局部实现及定向验证，不需再次询问已明确的功能；不改框架Rust/公开协议，不启动真实桌面，不做完整应用兼容性验收、不推送。独立review和merge收尾另有owner。

## 2. 架构方案

两模式：源码模式按显式参数→对应AUTO_*_ROOT→OS兄弟→主检出解析三仓；显式无效覆盖立即失败。同级布局是现有Cargo相对依赖的前置，错误组合拒绝。按目标仓CWD构建到统一专用target：Man完整CLI，UI工具，iced另编ui_desktop。脚本不从PATH或旧语言target选可执行文件。

CandidateDir模式不构建、不从旧目录兜底：iced要求平铺ui_desktop/auto/auto-ui-tool三个真实文件；vue要求auto/auto-ui-tool并解析语言desktop-host工程根。设置AUTO_UI_TOOL_BIN指向本轮工具，子进程CLI按同目录/父目录既有发现机制命中同组产物。脚本不因文件存在而声称版本匹配或真实桌面已验收。

## 3. 技术栈

PowerShell、Bash、Cargo；Python unittest仅用于脚本回归。保留原脚本runtime依赖，不增加Python启动依赖。构建仍在三仓各自CWD读取Cargo配置，统一target可覆盖输出目录。

## 4. 需求分析与背景调查

依据AGENTS.md、docs/specs/shell/desktop-app-launch.md及语言仓repository-boundaries/man-three-repository-delivery.md。OS无specs/overview，当前模块页为入口。源码锚：main b21ef25efb1290ef0164ceec8e148fb058ab8a20；原desktop.ps1/sh的iced臂只构建语言example，vue臂PATH→语言target；ui_desktop已直接use auto_ui_runtime，rqhost_auto_binary/outproc_auto_binary从当前exe向上寻auto而非PATH。迁后CLI owner是Man，tool owner是UI，故要统一产物和显式工具来源。OS主检出其他会话仅docs WIP，本次不覆盖；框架三仓只读。

## 5. 详细设计

新增PS参数LangRoot/UiRoot/ManRoot/BuildRoot/CandidateDir，对应Bash --lang-root/--ui-root/--man-root/--build-root/--candidate-dir。BuildRoot默认OS/.auto/desktop-runtime，target在其中；CandidateDir与BuildRoot互斥。保留Track、Fullscreen、DryRun与DESKTOP_OS_ROOT。源码模式显式CARGO_TARGET_DIR与构建命令、选定仓/二进制路径均打印；成功后校验应有文件。所有外部构建/启动exit code传播，临时环境与CWD在成功/失败后恢复（子进程继承选定根、工具、OS/apps环境）。DryRun只打印，不创建目录/调用Cargo/启动GUI，不依赖候选exe已存在。

### 规范增量

| delta_id | 操作 | target | before/after | 理由 | AC |
|---|---|---|---|---|---|
| SD-01 | add | docs/specs/shell/desktop-bootstrap.md | 旧单仓路径假设→三仓来源/统一target/候选目录/失败与dry-run合同 | 消费迁后公开owner | AC-01..04 |

## 6. 测试设计

只对启动脚本的外部路径/副作用/退出码做定向测试，真实产品运行保持pending。小型受控cargo/launch替身只验证脚本调度，不冒称真实构建或桌面首帧。PS/Bash入口均执行相同语义案例。

| V | owner_stage | 入口/输入 | 区分性断言 | AC |
|---|---|---|---|---|
| V-01 | work | 临时同级三仓、空格/Unicode路径、iced/vue dry-run | 选新owner与统一target，旧PATH无效；无文件写入/无外部命令 | AC-01,04 |
| V-02 | work | 真脚本调度受控cargo与launch | Man/UI/宿主顺序和CWD、target及工具env一致；子进程CLI可沿统一target邻接发现 | AC-01,03 |
| V-03 | work | 预留旧产物但cargo故意失败 | 非零传播且launch未发生；不静默使用旧exe | AC-03 |
| V-04 | work | 显式坏root、非同级、缺产物、候选模式、环境恢复 | 非零分类准确；候选不构建/不查PATH，退出后宿主环境/CWD恢复 | AC-02..04 |

## 7. 验收标准

| ID | owner_stage | 条件 |
|---|---|---|
| AC-01 | work | 源码模式识别三仓，按实际owner构建所需包到统一target，旧参数有效 |
| AC-02 | work | CandidateDir显式消费对应exe与工具，缺失/无效目录拒绝，绝不回退旧目录 |
| AC-03 | work | 外部失败码传播，失败不启动旧exe，子进程根/工具身份与路径明确 |
| AC-04 | work | PS/Bash调度与dry-run语义一致，空格路径/环境/CWD/MPV配置保持，README说明支持与限制 |
| AC-05 | review | 独立上下文验AC/V/SD及实际diff，明确真实三仓构建和GUI未执行的边界 |
| AC-06 | merge | 定向门通过后有序合入，SD/ledger/归档/guard收尾；无Cargo整档/docs_gen |

## 8. 执行步骤

| 状态 | ID | owner_stage | 依赖 | 交付 | AC |
|---|---|---|---|---|---|
| [x] | T-01 | work | 用户本次功能授权 | 原树/来源核验、专用OS工作树；冻结参数与路径/失败合同 | AC-01..04 |
| [x] | T-02 | work | T-01 | 修改PS/Bash两入口的三仓构建、候选与环境恢复 | AC-01..03 |
| [x] | T-03 | work | T-02 | 定向脚本回归与语法检查，失败/旧产物/Unicode路径反例闭合 | AC-01..04 |
| [x] | T-04 | work | T-03 | README用法和SD proposal/限制、兼容说明 | AC-04 |
| [x] | T-05 | work | T-04 | scoped diff自检、提交代码、handoff，真实GUI仍pending | AC-01..04 |
| [x] | T-06 | review | T-05冻结提交 | 同一工作树独立上下文review；不以替身证明桌面可用 | AC-05 |
| [ ] | T-07 | merge | T-06 pass | 定向门/合入/SD与ledger/归档/逐树guard清理 | AC-06 |

实施组D:/autostack/.wt/os-051/auto-os，分支codex/plan-051-dev；不需要改框架仓，不创建依赖链接/树。所有脚本修改与验证在该树，计划簿记留main。若要实际三仓构建，另按固定依赖worktree和资源窗口执行，不在本任务偷偷开重负载或GUI。

## 9. 复审记录

2026-10-28 / stage:merge / PLAN-051:r1 / completion_kind:delivered / checkpoint:archived（cleaned待下一步真实执行） / prepared:reviewed d8dbb046fe0124f52f51a660c67c9a36b9ac94f6与SD-01冻结；rebase d8dbb04→8b6ae21，range-diff精确等价；脚本/测试三文件逐字节未变，13测试证据复用 / landed:main ff-only→3088dea1db735c294ec9a6fe22f5b3522ee82209 / canonical:docs/specs/shell/desktop-bootstrap.md及README入口已读回 / ledger_refreshed:guarded offline projection（无本会话store工具或已知本地store监听），原ledger可解析、六节保留；新增P051-DESKTOP-BOOTSTRAP/P051-REVIEW-R1/P051-MERGE-R1/P051-LAUNCHER-REGRESSION四项，source/file/tags齐，原条目和其他字段逐字节未扰动；原文档无version字段，不伪造版本；backup=D:/autostack/.verify/os-051-20261028/ledger-before.json / gates:doc/语法/diff检查通过，无Rust整档或docs_gen；非用户运行验证已通过声明 / next:fresh guard后清理唯一OS开发树、更新T-07完成，随后按用户额外授权实际构建并启动VM桌面。


2026-10-28 / stage:review / review_scope:final / reviewer_identity:本会话基于已提交工件重新审计，未另开独立模型/会话，明确上下文独立性限制；不是把work汇报当证据 / plan_id:PLAN-051 / revision:1 / outcome:pass / worktree:D:/autostack/.wt/os-051/auto-os / branch:codex/plan-051-dev / reviewed_commit:d8dbb046fe0124f52f51a660c67c9a36b9ac94f6 / base:841f46d / dependency_revisions:脚本回归无框架二进制依赖，源码owner按各仓Cargo清单只读核对 / acceptance_results:AC-01三仓解析/同级约束/统一target逐调用验证pass；AC-02候选不构建/缺件拒绝/无PATH回退pass；AC-03三阶段构建失败停止、真实native cargo exit23与launch exit31传播pass；AC-04PS/Bash direct入口/Unicode/空格/dry-run无目录写入/环境CWD恢复及README边界pass；AC-05独立验收步骤完成，所述上下文限制保留 / evidence:定向unittest13/13 exit0，22.761s，代码与提交干净重核；实际源码diff、根解析/程序发现全路径与SD对照 / SD-01 reviewed proposal sha256:6a32cdc3f9b07e86e67f60c86e73d24fdc288cc0e34e7f51c65dfea56d274514 / findings:none / pending_closeout_gates:规范落地与ledger受管刷新/线性合入/归档/guard清理；无Rust/Cargo整档/docs_gen要求 / next:auto-plan-merge T-07。用户本轮另行授权合入后真实三仓构建与VM桌面检查，该运行结果另记报告，不把脚本替身当产品正控。


2026-10-28 / stage:work / revision:1 / outcome:pass（work完成，待独立review，未merge） / worktree:D:/autostack/.wt/os-051/auto-os / branch:codex/plan-051-dev / base:841f46d / code_commit:d8dbb046fe0124f52f51a660c67c9a36b9ac94f6 / task_ids:T-02..T-05 / AC-01..04:脚本级验证通过 / readonly_source_heads:lang13057957c、UI7095f53、Man873fa23（仅路径/清单观察，非真实组合构建证明） / next:review T-06同树冻结提交，T-07合入沉淀/guard收尾仍pending。

验证：`python -m unittest discover -s scripts/tests -p test_desktop_launcher.py -v` exit0，13/13、17.885s，PS/Bash均执行，包括直接public入口dry-run和native cargo exit23、三构建阶段exit17与残留旧exe、缺件、不命中PATH、空格中文、候选不需源、覆盖序、环境/CWD恢复、启动exit31。首次测试发现PS null环境恢复变为空字符串，改为null时删除Env项后反例及同族通过。PowerShell Language.Parser零错误、`bash -n scripts/desktop.sh` exit0、`git diff --check` exit0。设置DESKTOP_OS_ROOT=主OS根后，两脚本真实目录dry-run均选Man/UI/lang且共同target，无目录写入或外部构建。

SD-01草案见[spec-delta.md](../attachments/051/spec-delta.md)，README已解释新owner/候选入口与限制；原始替身trace在测试临时目录生命周期内，仅控制调度，不充当产品正控。永久测试即复跑入口。`scripts/desktop.ps1` sha256 `eb6d0ac62a6c9828991ab47fe45e8ac7c9a390908017ed96161f6bd91254d12c`；`scripts/desktop.sh` sha256 `aa051130f14978373e34590c261ea5735d93ca340251658a45f8fb30b795ff37`；`scripts/tests/test_desktop_launcher.py` sha256 `2eefaf64fb829ebd2cd32b20c0d79c550cd69c736ea982352bb31b929b21f3f0`。

剩余边界：没有运行真实Cargo build/整档/docs_gen/GUI；匹配版本、资源完整性、桌面首帧和app交互仍需接入实测，不计为本Plan已验证。无产品Rust/协议改动，无远端写入，OS主检出他方docs WIP未触碰。当前代码工作树干净，保留供独立review，不清理、不自行合入。


2026-10-28 / stage:work / T-01 pass / tree:D:/autostack/.wt/os-051/auto-os / branch:codex/plan-051-dev / base:841f46d / next:T-02。根仓只有他方docs WIP，本次未动；不建依赖链接、不运行真实构建/GUI。

2026-10-28 / stage:new / revision:1 / outcome:pass（合同可实施，非实现验收） / next:work T-01 / scope:用户本次脚本修改请求；不merge/push，不运行真实GUI。

## 10. 待澄清事项

无阻塞用户决策。库/工具二进制版本匹配和真实桌面可用性不能靠路径测试证明，另行接入实测。脚本仅消费既有公开owner，不设计新发布包协议。
