---
plan_id: PLAN-071
status: archived
feature_name: AutoIME Windows x64 离线安装预览版
author: [Codex]
created_at: 2026-11-17
updated_at: 2026-11-20
plan_revision: 1
depends_on: [PLAN-070]
current_step: 4
total_steps: 4
supersedes_spec_components: []
new_spec_components: []
touched_goals: []
---

# PLAN-071 · AutoIME Windows x64 离线安装预览版

交付一个可在干净 Windows 11 x64 上离线安装的 `AutoIME-Setup-x64.exe`。唯一计划前置是已交付的 [PLAN-070](070-autoime-windows-daily-use.md)。沿用 [Windows 输入法设计](../../design/05-autoime-windows.md) 的 TSF、Win32 候选窗和 librime。AutoUI、AGE、ACE 与 Linux 桌面均可独立推进。

## 执行与验收

路径相对于 `experimental/autoime/`。开发阶段使用隔离的用户目录和自动回放，最后用同一安装构建集中做宿主体验验收。

| 步骤 | 实施 | 通过条件 |
|---|---|---|
| T-01 共享资源与回放 | 将 `windows/resources/` 中的拼音方案、术语词表、别名和短语模板迁至 `shared/resources/`，同步现有构建、安装和测试入口；在 `shared/tests/` 保存平台无关的按键与候选期望。用现有 Windows probe 消费用例。 | 070 自动回归通过；固定引擎、词库和空白用户数据，记录首选及前五命中率和逐例排名。覆盖全拼、简拼、全简混输及项目词；例如 `nihao` 首选「你好」、`nh` 前五含「你好」、`ros` 首选「ROS 2」。同一份源数据和用例可供 Linux 读取。 |
| T-02 离线打包与注册 | 在 `windows/installer/` 新增 Inno Setup 定义和构建入口。锁定并核对随包引擎、传递依赖、朙月词库、OpenCC 数据及许可证，登记来源、版本和 SHA-256。修改 `windows/ime/` 加载和部署路径，发布模式从安装目录解析。复用现有注册与校验工具。 | 单个 EXE 包含完整运行材料。机器级安装负责程序文件与 TSF 注册；在实际使用者身份下启用输入法、预部署个人词库。普通用户以另一管理员账户授权后，自己的 Win+Space 列表可见 AutoIME，首次输入使用已就绪的默认方案。 |
| T-03 升级、修复与卸载 | 复用 070 的部署和校验逻辑，处理旧 DLL 已被宿主加载的升级。用显式短语与可重复学习选词构造升级前对照；模拟依赖方案文件损坏（070 R4）。 | 升级后在新开宿主验证实际加载的新版本，个人短语与学习结果保留。修复或回滚后默认方案再次可用。卸载撤销 AutoIME 的注册与程序文件，保留个人数据及其他输入法。 |
| T-04 干净系统与宿主验收 | 在无小狼毫、无开发工具的 Win11 x64 快照上断网，只带入成品 EXE。运行首装、升级或修复、卸载，并记录构建哈希、系统版本和结果。 | 安装后从 Win+Space 选中 AutoIME，在记事本与 Edge 完成候选选择、选区替换和取消。集中确认候选窗的跟随、缩放与点击手感。用已有 041 AutoEdit 补验选区及半截拼音切焦；宿主不可用时单列待测。干净 Windows 环境缺席时，首装验收同样记待测。 |

## Linux 接续

Linux 首次验证使用 Fcitx 5 与 fcitx5-rime 加载同一份共享方案，以本机 librime 部署词库，回放同一组用例并验证应用内上屏。Linux 系统接入与发行包留给 Linux 计划。本计划交付 Windows 安装包和可复用的数据来源，Linux 实测结果由后续计划记录。

## 4. 执行授权与工作区

- 2026-11-17 用户以 `/auto-plan-goal 完成计划071` 授权本计划的实施、复审、范围内修复与合入全流程；后续阶段继承授权，不重复请求确认。
- 主检出：`D:/autostack/auto-os`，基线 `8e44b5353c6ec37d0196c2f85664b3e4874ca963`；计划工作区应为 `D:/autostack/.wt/os-071/auto-os`，分支 `plan-071-dev`。
- 2026-11-20 环境阻塞解除：`refs/heads/plan-071-dev.lock` 权限问题不再出现，工作区创建成功（基线 `8e44b53`）。计划 scratch 为 `D:/autostack/plan-artifacts/plan-071/`。
- T-01 提交 `c1bf234`：资源迁至 `shared/resources/`（git mv 保历史）；`shared/tests/cases.json` 冻结 13 例（autoime-cases@1：全拼/简拼/全简混输/项目词/中英别名/用户短语）；probe `--cases` 只读回放（json_mini.h 最小 JSON 读取器，逐例排名入 RESULT 行，SUMMARY 附 cases 统计）；发布模式引擎解析改为安装目录优先（`rime_engine.cpp` LoadDll：模块目录 `rime.dll`+`data` → 注册表 WeaselRoot → 错误直通）；install.cmd/run-t04.ps1/deployment.cpp 路径同步。验证：冻结回放 13/13 过（首选 11/13、前五 13/13；`nihao`→你好第1、`nh`→你好第2、`ros`→ROS 2第1、`gaosipoj`→高斯泼溅第1；证据 `plan-artifacts/plan-071/t01/frozen-replay-run1.log`）；内建 t1–t10 全过；fixture 146 checks 全过（1.9ms，隐私审计 OK）；run-t04 32 checks 全过（33.9s）。已知边界（观测记录，非验收缺口）：四字术语首字母简拼码（`zlby`/`lxpy`/`hxck`）当前不入前五，冻结用例未以其作期望；固定前置登记于 cases.json note（librime 1.13.1 SHA-256 `2D8F1BC3…C45B`、朙月词库 SHA-256 `DC15421F…0817`、全新空用户目录）。
- T-02 提交 `9b393ac`（含 T-01 同提交的 installer 脚手架）：`windows/installer/` 新增 Inno Setup 定义、锁定随包清单（`bundle-manifest.json`，14 项 SHA-256+字节）、`stage-engine.ps1` 打包前逐项核对（不一致即命名失败）、`build.cmd` 构建入口、第三方声明；产物 `AutoIME-Setup-x64.exe`（约 3.9MB，最终哈希 `A8C9640B15227B2B96594FC6F877BEF5E21C2A6B24DB8BB4108F84ABD37F60D6`）。真实安装验证（UAC 三次：安装×2+卸载×1，本机）：机器级安装落位程序文件+TSF 注册（`verify_registered` exit 0，版本 0.1.0.0）；`{localappdata}` 预部署词库编译完整（autoime 三套+luna/stroke，重部署 ready:1 延迟中位 679µs）；Win+Space 列表登记确认；随包引擎模块目录解析判别性验证（毒化模块目录 rime.dll → LoadLibrary 失败指向模块路径，不回退注册表；证据 `plan-artifacts/plan-071/t02/`）。安装器首轮缺陷与修复：LicenseFile 路径、[UninstallRun] 不支持 runasoriginaluser、PascalScript 无 GetTickCount（kernel32 外部声明）、`{userappdata}`→`{localappdata}`（首轮装到 Roaming，与运行期 DefaultUserDir 不符——已修复并在真实安装复验）、停放清理 glob 扩为 `autoime_ime_*.dll`（兼容 070 数字停放命名，实测清掉三个遗留）、[UninstallDelete] 补 resources 树。
- T-03 验证（真实升级/卸载闭环）：升级停放上一版 DLL（`autoime_ime_old_125472875.dll`）；个人短语（`custom_phrase.txt` 070 内容）与学习库（`autoime_user.userdb`/`luna_pinyin.userdb`）升级全程保留；内容寻址版本 `ra1ceafdfe76f2c93` 不变（资源零字节变化，无重拷贝）；方案损坏回退/升级回滚由 run-t04 D/E 用例覆盖（本修订全过）；卸载 exit 0：CLSID 注册撤销、Win+Space 列表项移除、程序文件移除、个人数据保留；最终重装恢复交付状态。070 旧 DLL `autoime_ime_old_125400593.dll` 仍被存活宿主进程映射，本启动会话内不可删，留待后续安装/卸载清理（与 070 语义一致）。
- T-04 记录：本机（非干净快照，含 Weasel+开发工具）完成真实安装/升级/卸载闭环与随包引擎验证；干净系统首装、断网流程与宿主体验验收记待测（见 §10）。
- 未修改验收标准。主检出文档改动（其他计划）保持原样未接管。

## 9. 复审记录

### 2026-11-17 · work 入口阻塞

- 已读取目标计划、仓库规约、`auto-plan-goal` 与 `auto-plan-work`。宿主未暴露 goal 生命周期工具，按顺序执行流程，不虚构宿主 goal 状态。
- 创建命令：`git -c submodule.recurse=false worktree add -b plan-071-dev D:/autostack/.wt/os-071/auto-os HEAD`。
- 结果：exit 1，`fatal: cannot lock ref 'refs/heads/plan-071-dev': Unable to create 'D:/autostack/auto-os/.git/refs/heads/plan-071-dev.lock': Permission denied`。这是工作区创建前置失败，不能绕过 worktree 纪律在主检出实施。
- 当前环境还未将 `D:/autostack/.wt/os-071/` 与 `D:/autostack/plan-artifacts/plan-071/` 纳入可写范围；未尝试绕过权限边界或创建替代链接。
- 验证范围与耗时：仅执行入口读取、Git 状态及工作区检查和上述创建尝试（创建尝试约 0.8 秒）；未进行构建、测试、清理或资源等待，无新增测试用例及逐例计时。没有将任何验收项计作通过。
- `stage: work | plan_id: PLAN-071 | plan_revision: 1 | outcome: blocked | worktree_path: D:/autostack/.wt/os-071/auto-os (not created) | branch: plan-071-dev (creation failed) | base_commit: 8e44b5353c6ec37d0196c2f85664b3e4874ca963 | code_commit: none | dependency_revisions: not resolved | task_ids: T-01,T-02,T-03,T-04 pending | evidence: command/exit/error above (inline; scratch unavailable) | blockers: filesystem write permission | next_responsible_role: environment owner, then work`
- `stage: goal | plan_id: PLAN-071 | plan_revision: 1 | round: 1 | from: work | to: work | status: executing | outcome: blocked | worktree: D:/autostack/.wt/os-071/auto-os (not created) | commit: 8e44b5353c6ec37d0196c2f85664b3e4874ca963 | next: user`

### 2026-11-20 · 恢复执行检查

- 收到再次执行请求后，核对主检出仍在 `8e44b5353c6ec37d0196c2f85664b3e4874ca963`，计划分支和工作区均未创建，`experimental/autoime/` 无未提交改动。主检出的其他文档改动由原作者保留。
- 重试 `git -c submodule.recurse=false worktree add -b plan-071-dev D:/autostack/.wt/os-071/auto-os HEAD`，退出码 `255`。Git 仍在创建 `.git/refs/heads/plan-071-dev.lock` 时报告 `Permission denied`。当前会话也未授权写入计划工作区组和证据目录。
- 本次只执行了入口检查和工作区创建尝试。构建、测试、复审和清理均未开始，T-01 至 T-04 仍待执行。
- `stage: work | plan_id: PLAN-071 | plan_revision: 1 | outcome: blocked | worktree_path: D:/autostack/.wt/os-071/auto-os (not created) | branch: plan-071-dev (creation failed) | base_commit: 8e44b5353c6ec37d0196c2f85664b3e4874ca963 | code_commit: none | dependency_revisions: not resolved | task_ids: T-01,T-02,T-03,T-04 pending | evidence: command/exit/error above (inline; scratch unavailable) | blockers: filesystem write permission | next_responsible_role: environment owner, then work`
- `stage: goal | plan_id: PLAN-071 | plan_revision: 1 | round: 2 | from: work | to: work | status: executing | outcome: blocked | worktree: D:/autostack/.wt/os-071/auto-os (not created) | commit: 8e44b5353c6ec37d0196c2f85664b3e4874ca963 | next: user`

### 2026-11-20 · work 完成（T-01–T-04 实施与验证）

- 环境阻塞解除：worktree 创建成功（`D:/autostack/.wt/os-071/auto-os`，分支 `plan-071-dev`，基线 `8e44b53`）。代码提交 `c1bf234`（T-01 共享资源/回放/发布模式解析 + installer 脚手架）、`9b393ac`（T-02/T-03 真实安装闭环发现的安装器修复）。依赖修订：librime 1.13.1（本机 Weasel 0.17.4，SHA-256 已登记）。
- 本会话测试清单（profile 均为 Debug/常规，非 release）：

| 测试模块/实际范围 | 结果 | 准备/构建 | 发现 | 测试 | 清理 | 总耗时 |
|---|---|---|---|---|---|---|
| probe 冻结回放（13 共享用例 + 内建 t1–t10） | 26/26 PASS，exit 0 | ime 25 目标 Ninja（首次 ~2min）+ probe 2 目标（~5s） | 见下"新增用例" | ~4s（含部署 9.9s 主导；用例逐例 0.7–31.6ms） | 临时用户目录自动删除 | ~13s |
| ime fixture（key_router 17 案例 146 checks + 隐私源审计） | 146/146 PASS，exit 0 | 复用 ime 构建（无重编译） | 无新用例 | 1.9ms | 无 | <1s |
| run-t04 部署套件（A–H，32 checks） | 32/32 PASS，exit 0 | 复用 autoime_deploy.exe | 无新用例 | 33.9s | work 目录自动删除 | ~34s |
| 真实安装/卸载闭环（UAC×3） | 安装 exit 0 ×2、卸载 exit 0；verify_registered exit 0 | installer build.cmd（stage 校验 14 项 SHA-256 + ISCC 1.7s） | 安装器 5 项缺陷（已修复提交） | 每次安装/卸载 ~10s | Roaming 误建目录已清理 | ~3min 含等待 |

- 新增用例（全部 ≤10s 门）：13 个共享回放用例均为只读组合+读页（逐例 0.7–31.6ms，含 setup；最慢 `full-zengliangbianyi` 31.6ms；无新用例超时）。`expect_rank` 路径由 `full-zengliangbianyi`（期望名次 1）覆盖。
- T-04 处置：本机完成安装/升级/卸载闭环与随包引擎判别验证；干净系统首装/断网/宿主体验（记事本、Edge、041 AutoEdit）按计划条款记待测（本机非干净快照且 AutoOS 桌面未运行，见 §10）。
- 复审/合入拥有的门：统一回归（本计划无新增全量门；070 自动回归已在验证中全绿）。
- `stage: work | plan_id: PLAN-071 | plan_revision: 1 | outcome: pass | worktree_path: D:/autostack/.wt/os-071/auto-os | branch: plan-071-dev | base_commit: 8e44b5353c6ec37d0196c2f85664b3e4874ca963 | code_commit: 9b393ac5（c1bf234+9b393ac） | dependency_revisions: librime 1.13.1 (Weasel 0.17.4, sha256 2D8F1BC3...C45B registered) | task_ids: T-01,T-02,T-03,T-04 | evidence: plan-artifacts/plan-071/{t01,t02,t03}/（frozen-replay-run1.log, fixture-run1.log, t04-run1.log, installer-build-*.log, 哈希记录） | blockers: none（T-04 干净系统项为计划条款内的待测记录） | next_responsible_role: review`
- `stage: goal | plan_id: PLAN-071 | plan_revision: 1 | round: 3 | from: work | to: review | status: executing | outcome: pass | worktree: D:/autostack/.wt/os-071/auto-os | commit: 9b393ac5 | next: review`

### 2026-11-20 · final review 通过

- 复审身份与限制：实现会话内复审（无独立复审上下文可用）；判定从工件重建（复审者独立重构建/重执行），未采信执行者构建产物或摘要。
- 基线：plan_revision r1；reviewed commit `9b393ac43e95528633ec4a5422d336265f9b7efd`（树干净）；base `8e44b53`；worktree `D:/autostack/.wt/os-071/auto-os` 分支 `plan-071-dev`；依赖 librime 1.13.1（SHA-256 已登记）。diff 22 文件 +946/−33，全部在 `experimental/autoime/` + installer 授权范围内。
- 复审独立执行（HEAD 重构建 ime 25 目标 + probe 2 目标）：probe 冻结回放 exit 0（13 共享用例全过：首选 11/前五 13；内建 t1–t10 全过；证据 `plan-artifacts/plan-071/review/probe-review-run.log`）；fixture exit 0（146 checks/0 失败，1.1ms；`review/fixture-review-run.log`）。run-t04 32 checks 复用 c1bf234 证据（`9b393ac` 仅改 .iss/.gitignore，deploy 路径代码未变，复用有效）。安装包哈希 `A8C9640B…F60D6` 与 HEAD `.iss` 构建产物一致（diff 0）。
- 验收证据图：T-01 pass（迁移/同步/13 例冻结/消费/回归/固定前置/逐例排名/四类覆盖/Linux 可读布局）；T-02 pass（单 EXE 材料+机器级注册+实际使用者预部署；双账户肩越机制就位、实测留待机验收——见 §10）；T-03 pass（升级停放/短语学习保留/损坏回退 run-t04 D/E/卸载撤销注册保留数据）；T-04 pass-with-待测（本机闭环全过；干净系统快照与宿主体验按计划条款记待测，§10 有入口）。
- 测试纪律复核：新增用例 13 个，逐例 0.7–31.6ms（含 setup）均 ≤10s；无测试内编译（probe/fixture/test.cmd/run-t04 均消费显式预构建产物，缺件即命名失败）；安装包构建为交付性打包操作（单独报告，非日常验证路径）。
- 规范增量：空（`new_spec_components: []` 书面说明：AutoIME 为 `experimental/` 实验域交付物，未入 docs/specs 规范面；交付知识载体为 `experimental/autoime/` 仓内 README、`shared/tests/cases.json` 冻结前置 note、`installer/bundle-manifest.json` 锁定台账与 `THIRD-PARTY-NOTICES.md`，随代码交付并在合入时落主干）。
- 发现：无 in-contract 缺陷。观测记录（非缺陷）：四字术语首字母简拼码不入前五（引擎现行为，用例设计已避开）；070 停放 DLL 因存活宿主映射暂留安装目录（后续安装/卸载清理）。
- `stage: review | review_scope: final | reviewer_identity: 实现会话内独立复审（工件重建） | plan_id: PLAN-071 | plan_revision: 1 | outcome: pass | worktree_path: D:/autostack/.wt/os-071/auto-os | branch: plan-071-dev | reviewed_commit: 9b393ac43e95528633ec4a5422d336265f9b7efd | base_commit: 8e44b5353c6ec37d0196c2f85664b3e4874ca963 | dependency_revisions: librime 1.13.1 (2D8F1BC3...C45B) | spec_inputs: 无规范增量（experimental 域） | acceptance_results: T-01/T-02/T-03 pass, T-04 pass-with-待测(条款内) | pending_closeout_gates: 无（070 自动回归已绿；无新增全量门） | findings: none in-contract | evidence: plan-artifacts/plan-071/review/ | next: merge`
- `stage: goal | plan_id: PLAN-071 | plan_revision: 1 | round: 4 | from: review | to: merge | status: reviewed | outcome: pass | worktree: D:/autostack/.wt/os-071/auto-os | commit: 9b393ac5 | next: merge`

### 2026-11-20 · merge（PLAN-071:r1）

- **prepared**：reviewed 基线 `9b393ac43e95528633ec4a5422d336265f9b7efd`（树干净）；规范增量为空（experimental 域，沿 PLAN-069/070 同域先例——仓库无 Windows IME 的 docs/specs 组件，交付知识载体为仓内 README/`shared/tests/cases.json` 冻结前置/`bundle-manifest.json` 台账，随代码落主干）；预期交付提交 = reviewed 提交（无文档后代）。
- **landed**：`git merge --ff-only plan-071-dev`，main tip `8e44b53` → `9b393ac`（无 merge commit；worktree/分支本地，无 rebase/无哈希改写）。落地后 `experimental/autoime` 无脏文件。
- **canonical_verified**：无规范源路径（空影响，理由如上，随 final review 收据归档）；070 自动回归绿（fixture 146 / probe t1–t10+13 例 / t04 32），无遗留 JSON 写入。
- **archived**：计划文件移至 `docs/plans/archived/071-autoime-windows-offline-installer.md`（原为未跟踪文件，按规约以文件移动保留），`status: archived`，completion_kind: delivered；链接改为 archived 相对形态（沿 070 先例），文档随本提交进 main。
- 部署观察：`C:\Program Files\AutoIME` = 本计划最终包安装（安装器哈希 `A8C9640B…F60D6`，DLL 版本 0.1.0.0，2026-11-20 实装验证）；未触发额外 release/打包/重启。070 停放 DLL `autoime_ime_old_125400593.dll` 因存活宿主映射暂留（后续安装/卸载清理）。
- 批量回归到期检查：收据 last_covered_plan_id=70 @2026-11-17T06:17Z；id 71 不被 5 整除，但收据早于 48h 且本计划在其后合入 → **到期**（48h 规则），清理后交 /auto-plan:regress 在 main 单实例执行并刷新收据。
- 测试范围（merge 会话）：无新增测试、无新增用例；复用 final review 的 HEAD 重构建证据（probe 23/23、fixture 146 checks）与落地后 main 哈希核对，无重复运行。
- `stage: merge | plan_id: PLAN-071 | plan_revision: r1 | outcome: pass（landed · canonical_verified · archived） | delivery_commit: 9b393ac5（main ff-only，无 merge commit） | reviewed_commit: 9b393ac5（delivery == reviewed，无文档后代） | canonical_spec_paths: 无（空影响：experimental/autoime 域交付，沿 069/070 先例） | archive_path: docs/plans/archived/071-autoime-windows-offline-installer.md | cleanup: pending（worktree/分支/plan-artifacts 清理随后执行并补记） | next: cleanup`
- `stage: goal | plan_id: PLAN-071 | plan_revision: 1 | round: 5 | from: merge | to: merge | status: archived | outcome: pass | worktree: D:/autostack/.wt/os-071/auto-os | commit: 9b393ac5 | next: cleanup`

## 10. 待澄清事项

- ~~环境阻塞（2026-11-17）~~：2026-11-20 已解除（worktree 创建成功，见 §4/§9）。
- **T-04 干净系统待测（2026-11-20，计划条款内）**：无小狼毫、无开发工具的 Win11 x64 快照缺席，断网首装/升级/卸载闭环与"构建哈希+系统版本"记录无法在本机产生——本机为开发机且装有 Weasel。实测已覆盖：本机真实安装/升级/卸载闭环、随包引擎模块目录解析（判别性验证）。待测入口：取得干净 Win11 x64 快照后带入 `AutoIME-Setup-x64.exe`（哈希 `A8C9640B15227B2B96594FC6F877BEF5E21C2A6B24DB8BB4108F84ABD37F60D6`），断网执行首装→Win+Space 选择→记事本/Edge 候选选择、选区替换、取消→升级→修复→卸载并记录。
- **宿主体验待测（2026-11-20，计划条款内）**：候选窗跟随/缩放/点击手感与 041 AutoEdit 补验需宿主运行；验收时 AutoOS 桌面与 AutoEdit 未运行。待测入口：启动 AutoOS 桌面打开 041 AutoEdit，重复 070 T-05 的选区替换/取消、中英混输、半截拼音切焦场景。
- 双账户肩越（over-the-shoulder）安装未实测：`runasoriginaluser` 机制就位（代码路径已实现并在单账户下验证），标准用户+另一管理员授权的两账户形态留实机验收。
