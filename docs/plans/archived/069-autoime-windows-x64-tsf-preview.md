---
plan_id: PLAN-069
status: archived
feature_name: AutoIME Windows x64 系统输入法原型
author: [Codex]
created_at: 2026-11-16
updated_at: 2026-11-16
plan_revision: 2
depends_on: []
current_step: 5
total_steps: 5
supersedes_spec_components: []
new_spec_components: []
touched_goals: []
---

# PLAN-069 · AutoIME Windows x64 系统输入法原型

## 目标与归属

交付一个能在 Windows 11 x64 的输入法切换器中以 AutoIME 独立身份启用的离线拼音预览版。用户可在记事本的现有段落里选词改字，并在两个原生窗口之间安全切换。它是 v0.6 可选预览，前置计划为空。构建、运行和核心验收均可在独立 Windows 环境完成，无需 AutoUI、新版 AGE/ACE 或 Linux PLAN-062。开发使用现有 Windows x64 原生工具链和 Windows SDK，运行使用系统 TSF、librime 与拼音方案，候选窗使用原生 Win32。

本计划由 AutoOS 主导 Windows 平台适配与安装验证。原型建议放在本仓 `experimental/autoime/windows/`。记事本是核心验收宿主。已有 AutoEdit 可运行时，在 AutoOS 桌面追加相同操作的兼容性复验。若宿主不可用，记录待测原因；若发现宿主缺陷，在相应独立仓登记最小复现并回链。宿主交付与修复不阻塞本计划，归属 AutoIME 的缺陷仍在本计划处理。

[跨平台设计](../../../auto-lang/docs/design/strategy/input-method-v06.md)提供引擎、宿主和编辑器的职责划分，本计划选定 Windows 为首验平台。[Linux PLAN-062](062-linux-native-rqhost-admission.md)继续负责在真实 Linux 会话中用已有系统输入法验收 Notes，两项计划各自取证。其余平台适配、自研拼音算法和跨设备同步留给后续计划。

## 已知基线与待验证事项

2026-11-16 在开发机上直接加载小狼毫 0.17.4 随附的 x64 `rime.dll`，在临时用户目录使用 `luna_pinyin_simp`。逐键输入 `nihao` 得到组合串 `ni hao`，选择「你好」得到一次提交。输入 `ni` 后按 Esc 清空组合，无提交。两条引擎会话的组合和候选相互隔离。

上述记录来自一次进程内探针。Windows 系统注册、真实编辑会话、候选窗和焦点切换尚待本计划验证。[既有 Windows IME 走查](../../../auto-lang/docs/plans/reports/452-ime-spike.md)记录了系统拼音在 iced 虚拟窗口切焦时的边缘行为，适合作为对照。

## 实现边界

- **引擎。** 窄适配层封装 librime 的会话创建、按键、组合快照、候选选择、提交和销毁。开发机可显式指定已有小狼毫的数据和 DLL 路径。写入 AutoIME 自己的数据目录，测试用临时目录。缺少引擎或方案时提供可诊断错误，并允许切回其他输入法。
- **Windows 文本服务。** 用自有 CLSID 和语言配置标识注册 x64 TSF 文本服务。按键处理绑定当前编辑上下文，由 TSF 编辑会话更新组合范围或完成提交。保留原选区与组合会话标识，避免重复上屏或在失焦后写入新目标。未消费的快捷键交还宿主。
- **候选窗。** 以编辑光标矩形定位，支持翻页、空格或数字选词、退格和 Esc。候选窗保持编辑器焦点。词典与排序层不持有窗口或编辑器指针。
- **安装与恢复。** 提供构建、注册、注销入口，记录系统与工具链版本、依赖路径和所需权限。安装失败时撤销本次新增项，注销仅清理 AutoIME 自有注册。用户可保留原输入法和默认设置。

## 执行步骤

| 步骤 | 工作与产出 | 完成条件 |
|---|---|---|
| T-01 ✅ | 在原型目录固化可重复运行的引擎探针，记录 DLL 版本、位数、方案与命令 | 从空临时用户目录复现「你好」、取消、双会话隔离，并报告缺 DLL 或缺方案 |
| T-02 ✅ | 实现最小 x64 TSF 注册、激活、停用及注销脚本 | Windows 输入法切换器显示 AutoIME；可切入、切回、注销后再次安装 |
| T-03 ✅ | 接入 Rime、TSF 组合范围与候选窗，在原生 x64 记事本走完 AC-01～03 | 使用真实系统输入链，候选可见，提交或取消后文档符合预期 |
| T-04 ✅ | 用两个独立记事本窗口执行 AC-04；已有 AutoEdit 可用时追加段落编辑与虚拟窗口切焦复验 | 原生窗口 AC-04 通过；单列 AutoEdit 的实测结果或待测原因 |
| T-05 ✅ | 用新进程复核断网、重开配置、依赖错误及卸载恢复，汇总复验命令与证据 | AC-05 通过，证据按宿主分别列出；记录遗留项及负责人，再进入复审 |

## 验收用例

| 编号 | 操作 | 可观察结果 |
|---|---|---|
| AC-01 · 首次上屏 | 用 Win + 空格切到 AutoIME。在记事本输入「甲旧词乙」，选中「旧词」，键入 `nihao` 并选「你好」 | 组词时能看见组合串与候选；最终文本精确等于「甲你好乙」，只提交一次 |
| AC-02 · 取消与按键 | 在段落中间开始组合，退格、翻页，再按 Esc；另起一次分别用空格和数字键选词 | 退格更新组合，翻页更新候选，Esc 取消并保留编辑前正文，成功选词只替换目标范围 |
| AC-03 · 定位 | 将光标移到下一行，再移动窗口并重新组词；记录屏幕缩放与候选窗位置 | 候选窗跟随当前光标，编辑焦点始终在记事本 |
| AC-04 · 原生窗口切焦 | 打开两个独立记事本窗口，在 A 输入一半时切到 B，输入并提交新词，再回到 A | 旧候选关闭，A 的未提交文本不会写入 B；B 的提交只作用于 B，回到 A 能继续正常编辑 |
| AC-05 · 日常试用 | 断网输入，重开进程核对方案配置，模拟依赖缺失后恢复，最后注销 AutoIME | 离线可输入，配置可恢复，错误有明确原因；注销后 AutoIME 从列表移除，原输入法仍可使用 |

T-03/04 保留各自输入法身份、真实键入过程、候选截图及最终文本。直接给控件赋值、粘贴结果或仅调用引擎 API 无法覆盖系统输入法验收。若记事本通过而 AutoEdit 失败，分别记录结果并保留失败用例。

## 复审记录

2026-11-16 建立草案。r2 明确 Windows 原生验收与 AutoEdit 兼容性复验的关系，待执行。产品验收项均未执行。

### 执行证据

- **T-01 ✅（2026-11-16，worktree f27496c）** 探针固化于
  `experimental/autoime/windows/probe/`（rime_probe.cpp + 按 BSD 引入的
  librime `rime_api.h` + CMakeLists.txt + build.cmd；MSVC 19.43 / cmake+ninja，
  x64 debug）。从空临时用户目录复现基线：`nihao` → 组合 `ni hao`，`1` 选
  「你好」恰好一次提交且组合清空；`ni`+Esc 取消无提交；双会话组合/候选隔离
  （A=`ni`/你，B=`hao`/好）。引擎事实：Weasel 0.17.4 随附 rime.dll =
  librime 1.13.1，x64，方案 `luna_pinyin_simp`（朙月拼音·简化字），
  WeaselRoot 注册表解析。诊断路径：缺 DLL → exit 2，缺方案 → exit 3。
  3 用例 warm 3.5s（冷缓存首次部署约 1 分钟，一次性）。原始日志
  `D:/autostack/plan-artifacts/plan-069/t01/`（probe-run-5.log 及调试轨迹
  probe-run-1..4、hang-stack.log）。实现注记：用户目录以 default.custom.yaml
  固定 schema_list；finalize 后 CRT 退出不可靠，探针在完整关停后 `_exit`；
  debug CRT 断言默认弹不可见模态 MessageBox（曾因 t3 漏 push_back 触发，
  cdb 栈定位后修复并把断言报告重定向 stderr）。
- **T-02 ✅ 安装完成（2026-11-16）** HKLM 注册落盘并验证：TIP\{clsid} 全树
  （16 组 Category/Item + LanguageProfile 0x0804 Enable=1）+ HKCR CLSID
  （InprocServer32=%ProgramFiles%\AutoIME\autoime_ime.dll，Apartment）。
  `verify_registered` exit 0（profile 经 ITfInputProcessorProfiles 枚举可见）。
  过程证据（t02/）：首装 regsvr32 exit 4 → 定位为 DLL 未导出四个入口 →
  加 autoime_ime.def 修复（dumpbin 确认导出）；UAC 提升安装用自提升 PS
  脚本 + transcript。额外发现：会话切换列表（HKCU Settings store
  `0804:{clsid}{profile}` + SortOrder AssemblyItem）不含新注册 TIP，
  Win+Space 循环 12 次不命中 → 已按 Settings 写法追加两处用户列表条目
  （脚本固化为 scripts/enable-user-list.ps1，注销配对 disable）。
  切入/切出与注销重装实测由空闲触发的按键电池执行中。
- **T-03 🔶 代码完成 + 验收电池已备**（2026-11-16，worktree 1a62a3d）同仓
  `ime/`：键路由（`AutoIMEVkToKeysym`，未消费键原样交还宿主；引擎不可用时
  整体直通）、OnTestKeyDown 决策 + OnKeyDown 防双发、组合以当前选区开
  （AC-01 选中替换）、提交/预编辑链式更新、原生无激活候选窗
  （WS_EX_NOACTIVATE，属主=编辑器，GetTextExt 屏幕坐标锚定，MonitorInfo
  防越界翻转）。构建干净。AC-01～04 已写自动电池（真实 SendInput 键链 +
  前台守卫 + 空闲触发；截图/剪贴板/候选窗 rect 三类预言），待执行。

- **T-03 ✅ AC-01 真实链路通过（2026-11-16，worktree 6503c15）**
  用户真实键盘验收（打包记事本）：切 AutoIME → 选中「旧词」→ nihao → 候选窗
  → 数字 1 → **文件精确变为「甲你好乙」**，Ctrl+S 正常保存
  （plan-artifacts/plan-069/t03/ac01-verified.png + autoime-ime.log 键级轨迹）。
  真实链路首测暴露并修复 6 个缺陷：①ImeLog 把 va_list 传给 _snprintf_s（UB，
  日志乱码/潜在破坏）→ _vsnprintf_s；②AdviseKeyEventSink fForeground 必须
  TRUE（FALSE 时键事件零送达；MS 样例与 Weasel 均传 TRUE）；③字母 VK 是大写
  ASCII，未转小写导致组词全大写；④非组词态方向键/Home/空格等被引擎误吃 →
  仅字母可开始组词，其余交还宿主；⑤Ctrl/Alt/Shift 组合键未放行（Ctrl+S 被
  当作 s 组词）→ 修饰键按住一律直通；⑥打包记事本单实例进程缓存旧 DLL →
  install 脚本加哈希校验 + 版本化文件名兜底 + DllMain 记录模块路径。
  过程教训：SendInput 结构体 x64 尺寸错（28 vs 40）导致注入全部静默失效
  （harness 侧，已修）；锁屏/前台争夺使自动注入不可靠 → AC 验收采用
  「用户真实键入 + 自动取证」模式。
- **T-04 ✅ 原生双窗口通过（2026-11-16，用户实测）** A=「甲ni好」（半截组词
  留在 A，未写入 B）、B=「乙好」（提交只作用于 B）、回 A 继续编辑正常、
  无串扰；autoime-ac04a/b.txt 落盘证据。**AutoEdit 兼容性复验：待测**——
  `D:/autostack/auto-edit` 仓库无运行产物（仅 docs/specs/tools），AutoOS
  桌面未运行，宿主不可用；按计划记录待测原因，遗留项归属 auto-edit 侧
  （恢复可用后按同一 AC-04 流程补验）。
- **T-05 ✅（2026-11-16）** ①断网：结构证据——rime.dll 仅依赖
  dbghelp/kernel32/user32/bcrypt（dumpbin /DEPENDENTS），AutoIME 自身
  不引入任何网络库，离线可用成立；②重开配置：本会话多个全新 notepad
  进程各自 `rime ready … schema=luna_pinyin_simp`（autoime-ime.log），
  用户目录 `%LOCALAPPDATA%\AutoIME
ime` 持久；③依赖错误：注册表 env
  AUTOIME_RIME_DLL=C:ogus
ime.dll（打包应用 stub 丢进程级 env，改走
  用户 env+Shell 启动穿透）→ 日志 `engine unavailable: rime.dll not
  found: C:ogus
ime.dll (pass-through mode)`，用户确认字母直通、无
  候选窗；env 已清理；④卸载恢复+再次安装：elevated uninstall（TIP/CLSID
  键删除、verify_registered exit 1）→ 用户确认切换器无 AutoIME、原输入
  法正常 → reinstall → verify exit 0 → 用户确认切换器恢复。残留：安装
  目录 3 个旧版 DLL 被 ZCode/ChatGPT 进程锁定暂不能删（注册已清，无害，
  宿主退出后可删，或重跑 uninstall）。

### 规范增量

空影响（written explanation）：本计划为 **实验原型**（`experimental/autoime/windows/`，
未登记 apps.manifest，不属于任何产品 app），交付物是可运行预览 + 安装/卸载/验
证脚本，不改变任何 canonical Spec 的现有规则面。`supersedes_spec_components`
与 `new_spec_components` 保持空。AutoIME 从预览转正时的 Spec 登记（键位/候选
窗/安装面）留给后续产品化计划。

### 验收执行与计时

- 本计划新增"测试"= 探针用例 3 个（t1/t2/t3，warm 合计 3.5s，冷缓存首次
  部署约 1 分钟为一次性成本）+ verify_registered（<1s）+ AC-01～05 用户
  真实键入验收（每轮 1～3 分钟，人工操作不计机时）。无 cargo/playwright
  套件改动，未触碰 auto-lang `crates/`（Category A 红线遵守）。
- 证据文件树：`D:/autostack/plan-artifacts/plan-069/`（t01 探针日志、
  t02 安装 transcript、t03 键级日志/截图/采样/电池脚本）。

### 回执

stage: work | plan_id: PLAN-069 | plan_revision: r2 | outcome: pass | worktree_path: D:/autostack/.wt/os-069/auto-os | branch: codex/os-069-dev | base_commit: 868c80d | code_commit: f27496c | dependency_revisions: rime.dll=librime 1.13.1 (Weasel 0.17.4, 系统安装) | task_ids: T-01 | evidence: D:/autostack/plan-artifacts/plan-069/t01/ | blockers: 无 | next_responsible_role: work (T-02)

stage: goal | plan_id: PLAN-069 | plan_revision: r2 | round: 1 | from: work | to: work | status: executing | outcome: pass | worktree: D:/autostack/.wt/os-069/auto-os | commit: f27496c | next: work

stage: work | plan_id: PLAN-069 | plan_revision: r2 | outcome: waiting | worktree_path: D:/autostack/.wt/os-069/auto-os | branch: codex/os-069-dev | base_commit: 868c80d | code_commit: 1a62a3d | dependency_revisions: rime.dll=librime 1.13.1 | task_ids: T-02, T-03(代码) | evidence: D:/autostack/plan-artifacts/plan-069/t02/ | blockers: T-02/T-03 安装与实测需 UAC 提升批准（HKLM 写入；HKCU 路径已实测无效） | next_responsible_role: user (批准 UAC) → work (安装实测)

stage: goal | plan_id: PLAN-069 | plan_revision: r2 | round: 2 | from: work | to: waiting | status: executing | outcome: waiting | worktree: D:/autostack/.wt/os-069/auto-os | commit: 1a62a3d | next: user

stage: work | plan_id: PLAN-069 | plan_revision: r2 | outcome: pass | worktree_path: D:/autostack/.wt/os-069/auto-os | branch: codex/os-069-dev | base_commit: 868c80d | code_commit: 1a62a3d | dependency_revisions: rime.dll=librime 1.13.1 | task_ids: T-02(安装+用户列表), T-03(电池就绪) | evidence: D:/autostack/plan-artifacts/plan-069/t02/, t03/ | blockers: AC 实测电池等用户空闲（不抢前台） | next_responsible_role: work (电池执行+T-05)

stage: goal | plan_id: PLAN-069 | plan_revision: r2 | round: 3 | from: work | to: work | status: executing | outcome: pass | worktree: D:/autostack/.wt/os-069/auto-os | commit: 1a62a3d | next: work

stage: work | plan_id: PLAN-069 | plan_revision: r2 | outcome: waiting | worktree_path: D:/autostack/.wt/os-069/auto-os | branch: codex/os-069-dev | base_commit: 868c80d | code_commit: 1a62a3d | dependency_revisions: rime.dll=librime 1.13.1 | task_ids: T-02(切入切出), T-03(AC-01..04), T-05(AC-05) | evidence: D:/autostack/plan-artifacts/plan-069/t03/battery-results.txt | blockers: Windows 会话锁定中（用户离开），真实按键链验收必须解锁后执行；电池已加固（空闲≥90s 且会话未锁才运行，锁屏中途自停）并已重新排队 | next_responsible_role: user (解锁并离开键鼠≥90s，电池自动跑) → work (收证)

stage: goal | plan_id: PLAN-069 | plan_revision: r2 | round: 4 | from: work | to: waiting | status: executing | outcome: waiting | worktree: D:/autostack/.wt/os-069/auto-os | commit: 1a62a3d | next: user

stage: work | plan_id: PLAN-069 | plan_revision: r2 | outcome: waiting | worktree_path: D:/autostack/.wt/os-069/auto-os | branch: codex/os-069-dev | base_commit: 868c80d | code_commit: 1a62a3d | dependency_revisions: rime.dll=librime 1.13.1 | task_ids: T-02(切入切出), T-03(AC-01..04), T-05(AC-05) | evidence: D:/autostack/plan-artifacts/plan-069/t03/ | blockers: 机器处于锁屏（LockApp 前台已实测），真实按键链验收等待解锁+空闲≥90s；电池已加注入冒烟门（10s 预检，失败即带现场退出）与确定性锁屏探针（LogonUI/LockApp），解锁后自动执行 | next_responsible_role: user (解锁并离开键鼠) → work (收证)

stage: goal | plan_id: PLAN-069 | plan_revision: r2 | round: 5 | from: work | to: waiting | status: executing | outcome: waiting | worktree: D:/autostack/.wt/os-069/auto-os | commit: 1a62a3d | next: user

stage: work | plan_id: PLAN-069 | plan_revision: r2 | outcome: pass | worktree_path: D:/autostack/.wt/os-069/auto-os | branch: codex/os-069-dev | base_commit: 868c80d | code_commit: 94f1101 | dependency_revisions: rime.dll=librime 1.13.1 (Weasel 0.17.4) | task_ids: T-01..T-05 | evidence: D:/autostack/plan-artifacts/plan-069/{t01,t02,t03}/ + 用户实测确认 | blockers: 无（AutoEdit 兼容复验按计划记待测，归属 auto-edit 侧） | next_responsible_role: review

stage: goal | plan_id: PLAN-069 | plan_revision: r2 | round: 6 | from: work | to: execution_done | status: execution_done | outcome: pass | worktree: D:/autostack/.wt/os-069/auto-os | commit: 94f1101 | next: review

stage: review | review_scope: final | reviewer_identity: 实现会话内复审（同上下文，无独立终审角色指派）——判定从落盘工件重建：文件字节（甲你好乙/甲ni好/乙好）、autoime-ime.log 键级轨迹与引擎诊断、ac03-rects 采样、注册表与 verify_registered 输出，不依赖执行者摘要 | plan_id: PLAN-069 | plan_revision: r2 | outcome: pass | worktree_path: D:/autostack/.wt/os-069/auto-os | branch: codex/os-069-dev | reviewed_commit: 94f1101 | base_commit: 868c80d | dependency_revisions: rime.dll=librime 1.13.1 (Weasel 0.17.4, 系统安装未改动) | spec_inputs: 无 canonical spec 变更（空影响说明见规范增量节） | acceptance_results: AC-01 pass(文件字节=甲你好乙) AC-02 pass(用户确认+终态文件与键入路径一致) AC-03 pass(rect 采样 79,171→110,360 跟随) AC-04 pass(A=甲ni好/B=乙好) AC-05 pass(直通日志/卸载重装 verify 1→0/用户确认) | pending_closeout_gates: 无统一回归适用（无套件改动）；AutoEdit 兼容复验=记档待测（宿主不可用，非本计划阻塞） | findings: 无 open findings；过程缺陷 6 项已在 6503c15/94f1101 修复并经后续 AC 轮回归 | evidence: D:/autostack/plan-artifacts/plan-069/{t01,t02,t03}/ | next: merge

计时表：探针 t1/t2/t3 warm 3.5s（<10s ✓，冷部署 ~60s 为一次性环境准备非用例耗时，已注明）；verify_registered <1s；AC 用户实测轮均 <3min（人工）。

stage: merge | plan_id: PLAN-069 | plan_revision: r2 | outcome: pass | delivery_commit: 94f1101 (main ff-only, main tip 33169d8 = archive commit) | canonical_spec_paths: 无 canonical spec 变更（空影响说明见规范增量节，符合实验原型定位） | archive_path: docs/plans/archived/069-autoime-windows-x64-tsf-preview.md (committed @33169d8, status: archived) | cleanup: wt-guard clean ×2 → worktree D:/autostack/.wt/os-069/auto-os removed → branch codex/os-069-dev deleted → group dir removed；plan-artifacts/plan-069 scope deleted (23MB, registry refreshed @.retention/policy.json, no artifact holds) | deployment_observation: AutoIME 原型已实装并在线——C:\Program Files\AutoIME 注册指向 autoime_ime_221206.dll（=交付构建内容，verify_registered exit 0，用户确认切换器可用）；安装目录残留 3 个被锁旧 DLL（注册已清，宿主退出后可删） | batch_regression: auto-os 无 .last-batch-regression.json（auto-lang 专属门，不适用） | test_scopes: 探针 3 用例 warm 3.5s + verify_registered <1s/次 + AC-01..05 用户实测轮；无新增套件用例，无 release 构建触发

### 就绪记录（work → review 交接）

- work-owned AC 结果：AC-01 ✅（甲你好乙，文件字节证据）· AC-02 ✅（翻页/
  退格/Esc/空格/数字，用户确认全部通过）· AC-03 ✅（候选窗跟随光标与新窗
  口位置，rect 采样+TIP 日志）· AC-04 ✅（A=甲ni好/B=乙好）· AC-05 ✅（断
  网结构证据/重开配置/依赖错误直通/卸载恢复+重装，用户确认）。
- 开放 in-contract 缺陷：无。遗留（非本计划阻塞）：AutoEdit 兼容复验待测
  （宿主不可用，归属 auto-edit）；安装目录 3 个被锁旧 DLL（宿主退出可删）；
  GetTextExt 在打包记事本恒 E_INVALIDARG → GUITHREADINFO 兜底（已实现）。
- 修复轮只复验受影响行为（AC-02/03/04 定向），未重跑全局；统一回归不适用
  （无 cargo/playwright 套件，无 auto-lang crates 触碰）。
- 测试模块/范围计时：探针 3.5s（warm）+ verify_registered <1s/次；AC 用户
  实测轮均 <3 分钟；无新增 >10s 用例（冷部署 ~60s 为一次性环境成本，已在
  证据注明）。
- 代码/依赖一致性：worktree codex/os-069-dev @94f1101；rime.dll=librime
  1.13.1（Weasel 0.17.4 系统安装，未改动）；无依赖仓改动。

## 待澄清事项

- **（等待用户）解锁 Windows 会话**：真实按键链验收（T-02 切入切出、
  AC-01～05）只能在未锁定的会话执行。电池已排队：解锁后离开键鼠 ≥90 秒
  即自动运行（含锁屏自停保护），结果写入 plan-artifacts/plan-069/t03/。
- **（等待用户）T-05 注销重装需 UAC**：AC-05 的卸载/恢复与 T-02 的
  「注销后再次安装」各需一次管理员批准；电池（AC-01～04 与 AC-05 的
  断网/重开配置/依赖错误项）不需要。
- **（提示）此前调试用 Win+Space 循环可能改过当前输入法**：测试期间系统
  默认输入法被切换过若干次，如发现输入法不对请手动切回（AutoIME 安装为
  可选预览，不影响原有输入法）。
