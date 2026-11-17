---
plan_id: PLAN-070
status: reviewed
feature_name: AutoIME Windows 日常试用版
author: [Codex]
created_at: 2026-11-17
updated_at: 2026-11-17
plan_revision: 2
depends_on: [PLAN-069]
current_step: 5
total_steps: 5
supersedes_spec_components: []
new_spec_components: []
touched_goals: []
---

# PLAN-070 · AutoIME Windows 日常试用版

## 交付目标与依据

在现有 Windows 11 x64 TSF 输入法上交付一套可日常使用的离线拼音方案。行为以 [Design 05](../design/05-autoime-windows.md) 为准。[PLAN-069](archived/069-autoime-windows-x64-tsf-preview.md) 已交付记事本上屏原型（代码 `94f1101`），构成本计划唯一且已满足的计划依赖。AutoOS 仓主导现有 Windows 原生实现；新版 AGE/ACE、Linux PLAN-062 以及 AutoUI 新功能均不在关键路径。

069 的「甲你好乙」与双窗无串扰是回归基线。它们没有覆盖选中旧词后取消、重复 Test 回调、Shift 混输、非数字候选标签或新方案离线升级。已归档的 069 继续保留其原始验收结果。

## 执行步骤

路径相对于 [Windows 原型目录](../../experimental/autoime/windows/README.md)。下列工作逐项记录改动版本、验证入口与结果。

| 步骤 | 实施面和操作 | 通过条件 |
|---|---|---|
| T-01 ✅ · 隐私与按键 | 在 `ime/dllmain.cpp`、`ime/textservice.cpp`、`ime/ime.h` 移除输入内容诊断，处理 TSF 探询、实际派发、重复消息和修饰键。在 `ime/rime_engine.{h,cpp}` 传入真实修饰状态。 | 自动重放 Test→Test→Down、仅 Down、长按与失焦迟到事件；每个有效键只产生预期次数的操作。Shift 单击切模式、Shift＋字母保留大小写，Ctrl+S 由宿主处理。敏感输入直通，无学习或候选；诊断中无键码序列、预编辑、选区或提交正文。 |
| T-02 ✅ · 编辑会话 | 在 `ime/textservice.cpp` 记录原选区及恢复信息，维护焦点代次，处理宿主主动终止、写锁失败、异步失效与部分提交；富文本采用设计中的可逆路径。 | 对「甲旧词乙」选中「旧词」后输入并取消，正文、选区恢复；确认「你好」只替换一次。A 切 B 时预编辑无串写，宿主接管后的旧回调不改正文；含 emoji 的前后缀保持完整。 |
| T-03 ✅ · 方案与候选窗 | 新增 `autoime_pinyin` 方案及通用中文、项目、中英、显式短语等资源；登记实际依赖、版本、校验值和来源。扩展引擎快照，在 `ime/candidate_window.cpp` 消费真实标签、页内索引和页版本；修正 `GetTextExt` 输出参数，验证缩放与回退。 | 在临时用户目录回读新会话的方案 ID；回放 nh、nih、nihao、项目词、中英混输及自定义短语增删。选词与所示标签一致，过期点击无效。基础方案离线工作，既有显式方案选择及朙月回退均可用。 |
| T-04 ✅ · 部署与恢复 | 更新 `scripts/install.cmd`、`scripts/uninstall.cmd` 和引擎加载路径，分离首次部署和首键处理。候选资源编译成功后切换版本，失败保留最近可用版本；处理双宿主同时冷启及已加载旧 DLL。 | 从全新测试用户目录做离线冷启；分别验证资源损坏回退、引擎完全不可用时键盘直通、升级后用户短语和学习数据保留。记录首次部署时间和就绪后按键到候选延迟的中位数、P95。 |
| T-05 ✅ · 集成人工验收 | 用同一交付构建在记事本、浏览器文本框与 [041 AutoEdit](../../../auto-ui/examples/ui/041-auto-edit/README.md) 完成一轮真实键入。更新原型 README 的构建、部署、恢复说明，附合成用例和实际结果。 | 三宿主分别记录选区替换与取消、快捷键、中英混输、半截拼音切焦和候选跟随。AutoEdit 如确因宿主不可用，记待测原因及复验入口，不能计作通过；可复现的 AutoIME 自身缺陷在本计划修复。 |

## 验证方式和范围

- **开发阶段**沿用 `ime/build.cmd` 和 `probe/build.cmd`；从各自目录构建。扩展 `probe/rime_probe.cpp` 的引擎回放，并为 TSF 回调序列补独立自动化夹具。两者分别验证词库行为与宿主路由。以合成文字和隔离用户目录运行，不读取真实输入历史。初期选型沿用源码比较结果，无人工竞品试打。
- **集成后**用「你好，今天在 AutoOS 里重新编译内核」串起真实输入。人工验收确认窗口、焦点与手感；只复验修复影响的路径。原生 Windows 结果和 AutoEdit 兼容结果分别记录。
- **后续可选**为万象式整句模型、使用已提交的宿主上下文重排、共享引擎服务和跨平台移植另作决策。本次使用 librime 的基础组句、简拼和用户学习。

## 复审记录

2026-11-17 按 Design 05 r1 建立草案，待执行。T-01 至 T-05 的结果均由后续 work 记录。

### 执行记录

- **T-01 ✅（2026-11-17，worktree `D:/autostack/.wt/os-070/auto-os`，分支
  `plan-070-dev`）** 按键路由抽为 `ime/key_router.{h,cpp}`：Test 只判定、
  不推进引擎；匹配的 KeyDown 执行且仅执行一次。重复 Test 命中缓存、仅 Down
  直接处理、长按逐次处理、切焦代次过期的迟到回调丢弃。修饰位真实传入引擎；
  Ctrl/Alt/Win 与 AltGr 归宿主；Shift 单击切中英（组合中先取消）；Shift＋
  字母在中文模式保留大写与 Shift 修饰；密码/敏感输入范围
  （`GUID_PROP_INPUTSCOPE`=IS_PASSWORD 或 `GUID_COMPARTMENT_KEYBOARD_DISABLED`）
  与引擎不可用时全键直通。诊断改走 `ime/diag.{h,cpp}` 闭集 stage/code/计数
  接口，按键路径不再记录键码、预编辑、选区或提交正文；结构日志仅保留身份、
  路径、矩形与 HRESULT。自动化夹具 `ime/tests/key_router_test.cpp`
  （`ime/test.cmd`）复放上列回调序列：124 checks 全过，含隐私日志扫描
  （合成串不出现在日志）与源码审计（无 payload 型 ImeLog），warm 1.5s。
  证据 `D:/autostack/plan-artifacts/plan-070/t01/fixture.log`
  （sha256 `2D645C675493E579347DCE8B6B8AFFEBAF76E4BAEBA9A32C0A42450E3659F2FE`）。
  真实宿主中的行为随 T-05 集成人工验收复验。

- **T-02 ✅（2026-11-17，worktree 同上）** 组合起点快照（原选区 range 克隆、
  原文、是否非空选区）在启动组合前捕获；取消、Esc、失焦、切窗、Deactivate
  统一走 `RestoreComposition` 写回原文并按原样恢复选区；宿主强制终止走
  `OnCompositionTerminated` 只释放状态并作废旧代次；写锁/宿主拒绝写入时保留
  组合与快照、不向其他目标重放；提交与残余预编辑由
  `edit_model::PlanCompositionApply` 判定（先写一次前缀再开新组合）；
  UTF-8→UTF-16 转换保留 emoji 代理对。Fixture 新增部分提交与 emoji 用例，
  136 checks 全过，warm 1.5s。证据 `plan-artifacts/plan-070/t02-fixture.log`
  （sha256 `1CB1DE888A3B00AF66EA1D92C43D891E6BFDC445572D4AF9311B9CEB4ABD6AC4`）。
  真实宿主中的选区恢复随 T-05 复验。
- **T-03 ✅（2026-11-17，worktree 同上）** 新增 `resources/`：`autoime_pinyin`
  方案（全拼/简拼/全简混输/术语/中英/简体）、原创术语表、中英别名表、两个
  部署用依赖方案、用户短语模板，及依赖台账 `resources/README.md`（本机
  Weasel 0.17.4 + librime 1.13.1 的路径、字节、SHA-256 与许可来源）。
  依赖方案使部署器编译两张附加词表（librime 只为 `schema/dependencies`
  中方案的主 translator 准备词典）。数字保持为选词键（1–5）；别名改用纯
  字母码（`ros`→`ROS 2`、`autoos`→`AutoOS`），与设计示例 `ros2` 的差异与
  理由记入资源 README（触发条件由方案定义）。引擎快照只保留当前页候选并
  携带真实标签/页号/版本；候选窗按引擎标签与每显示器 DPI 绘制，鼠标点击
  携带页内索引+页版本，过期点击拒绝；`GetTextExt` 第四参数改为有效
  `BOOL clipped`。Probe 新增 10 用例（新会话回读方案 ID、nh 歧义与标签
  选词、nih、项目词、中英混输、短语增删后重部署、朙月回退、半截拼音），
  全过，warm 3.2s，deploy_ms≈2960。证据
  `plan-artifacts/plan-070/t03/probe-t03-final.log`
  （sha256 `5E40DB109FFA719CCFBA5FEDDB74329DB2EFEC66939C0063CEB6BC8328269F70`）。

- **T-04 ✅（2026-11-17，worktree 同上，code `b1ab2a5`）** 新增
  `ime/deployment.{h,cpp}`：资源按内容哈希定版（`.autoime-resources`）、命名
  互斥量跨进程串行化整段冷启、替换前备份 `.autoime-backup`、失败回滚并丢弃
  `build/`；`custom_phrase.txt` 与 `autoime_user.userdb` 升级时保留。安装期由
  `autoime_deploy.exe` 同步预部署，运行期在 `Activate` 起工作线程部署，就绪前
  按键直通。Schema 校验同时要求 schema id、显示名≠id 与真实组句候选，避免
  损坏 YAML 或陈旧 build 蒙混；失败序列：上一可用资源 → 朙月 → 禁用直通。
  安装/卸载脚本复制 `resources\`、处理加载中的旧 DLL（改名停放）、清理原型
  明文日志 `%TEMP%\autoime-ime.log`；卸载默认保留用户数据（`--purge` 删除）。
  T-04 套件 `ime/tests/run-t04.ps1` 32 checks 全过：A 全新冷启、B 未变更
  重部署、C 引擎缺失 exit 2、D 损坏无旧版→朙月 exit 5、E 损坏升级→回滚最后
  可用版、F 升级保留短语与学习库、G 双宿主并发冷启、H 延迟。实测：首次
  部署 2870 ms（全新用户目录），就绪后按键到候选 median 149 µs / P95 164 µs
  （50 次）；各用例 3.2–6.7 s，套件总 23.8 s。证据
  `plan-artifacts/plan-070/t04/run-t04-3.log`
  （sha256 `B00AD90DD748454F5285D9FAF95FE20BDC594F9BE18844EF81E2A9333449BCAC`）、
  `deploy-measure.log`
  （sha256 `08C8F18E54CB514A9AFE0416328BA23C8BBE4B3F1DD84C3C35141A1E221A7A18`）。

- **T-05 ✅（2026-11-17，交付构建=安装目录 DLL，code `268611d`；AutoEdit 单列
  待测）** 验收脚本先判定目标窗口当前 TIP：起一段组合并枚举宿主进程是否出现
  AutoIME 自带候选窗类 `AutoIMECandidateWindow`（小狼毫不产生该窗口），判定
  失败即中止、不向其他窗口发键；此前的误判（实际是小狼毫）已由此校验纠正。
  记事本（WinUI）：选区替换「甲旧词乙」→「甲你好乙」；Esc 取消恢复原文；
  emoji 前后缀「甲😀你好乙」；`autoos`/`ros`/`tsf` 首候选分别上屏
  AutoOS/ROS 2/TSF；半截拼音 `ni` 切窗后在另一窗提交 `好`，旧窗无串写且
  预编辑被清空；单击 Shift 切英文后 `abc` 直通，Ctrl+S 由宿主打开“另存为”、
  Esc 关闭后正文不变。浏览器（Edge/Chromium）：替换、取消（宿主终止组合时
  按保存的原选区回写）、`tsf`→`TSF` 均通过。候选窗为纵向原生窗口，按引擎
  标签与每显示器 DPI（本次 192）绘制、随光标移动，日志含每页标签与矩形。
  **AutoEdit 待测**：验收时 AutoOS 桌面/AutoEdit 未运行（仅有 `auto-ui-tool`
  进程、无 AutoEdit 窗口），故不计作通过；复验入口=启动 AutoOS 桌面并打开
  041 AutoEdit，重复替换/取消、中英混输与半截拼音切焦。
  证据：`plan-artifacts/plan-070/t05/notepad-verified-2.log`
  （sha256 `66E61E3B0C4D04F05B8F9AB77B3354E19B164CEB1D1066A6E03584A61BAEA880`）、
  `edge-verified.log`
  （sha256 `C8EBCE15FC8491142346DD1B33424B621A5B3BD70A348B4FD3D47B8F905C3720`）、
  `shortcut-autoedit.log`
  （sha256 `BE7BD6A98D4D621B8A65DA8298C1D28491F918BDDB591BC6D1CCD2EF7815EBBF`）、
  `t04-final.log`
  （sha256 `CC1B7DF34F8A7BA07D1F8583222C3136E98C46EB7D6E745C51B76BF9BD7EB97D`）。
  回归：fixture 136 checks、probe 10 用例、T-04 套件 32 checks 全过。

stage: work | plan_id: PLAN-070 | plan_revision: r1 | outcome: pass（T-01 完成，T-02..T-05 续）| worktree_path: D:/autostack/.wt/os-070/auto-os | branch: plan-070-dev | base_commit: 61237ba | code_commit: b130c61 | dependency_revisions: rime.dll=librime 1.13.1 (Weasel 0.17.4, 系统安装未改动) | task_ids: T-01 | evidence: D:/autostack/plan-artifacts/plan-070/t01/fixture.log (sha256 2D645C67…F2FE) | blockers: 无 | next_responsible_role: work (T-02 编辑会话、T-03 方案/候选、T-04 部署)

stage: goal | plan_id: PLAN-070 | plan_revision: r1 | round: 1 | from: work | to: work | status: executing | outcome: pass | worktree: D:/autostack/.wt/os-070/auto-os | commit: b130c61 | next: work

stage: work | plan_id: PLAN-070 | plan_revision: r1 | outcome: pass（T-02/T-03 完成，T-04..T-05 续）| worktree_path: D:/autostack/.wt/os-070/auto-os | branch: plan-070-dev | base_commit: 61237ba | code_commit: 243dd47 | dependency_revisions: rime.dll=librime 1.13.1 (Weasel 0.17.4, 系统安装未改动) | task_ids: T-02, T-03 | evidence: D:/autostack/plan-artifacts/plan-070/t02-fixture.log (sha256 1CB1DE88…6AC4), t03/probe-t03-final.log (sha256 5E40DB10…9F70) | blockers: 无 | next_responsible_role: work (T-04 部署与恢复、T-05 集成人工验收)

stage: goal | plan_id: PLAN-070 | plan_revision: r1 | round: 2 | from: work | to: work | status: executing | outcome: pass | worktree: D:/autostack/.wt/os-070/auto-os | commit: 243dd47 | next: work

stage: work | plan_id: PLAN-070 | plan_revision: r1 | outcome: pass（T-04 完成；T-05 待人工验收）| worktree_path: D:/autostack/.wt/os-070/auto-os | branch: plan-070-dev | base_commit: 61237ba | code_commit: b1ab2a5 | dependency_revisions: rime.dll=librime 1.13.1 (Weasel 0.17.4, 系统安装未改动) | task_ids: T-04 | evidence: D:/autostack/plan-artifacts/plan-070/t04/run-t04-3.log (32 checks, sha256 B00AD90D…BCAC), deploy-measure.log (deploy 2870ms, median 149us/P95 164us) | blockers: 无（T-05 需管理员安装新构建并在真实宿主键入）| next_responsible_role: work (T-05 安装与三宿主验收)

stage: goal | plan_id: PLAN-070 | plan_revision: r1 | round: 3 | from: work | to: work | status: executing | outcome: pass | worktree: D:/autostack/.wt/os-070/auto-os | commit: b1ab2a5 | next: work

stage: work | plan_id: PLAN-070 | plan_revision: r1 | outcome: pass（T-01..T-05 完成；AutoEdit 单列待测，非阻塞）| worktree_path: D:/autostack/.wt/os-070/auto-os | branch: plan-070-dev | base_commit: 61237ba | code_commit: 268611d | dependency_revisions: rime.dll=librime 1.13.1 (Weasel 0.17.4, 系统安装未改动) | task_ids: T-01..T-05 | evidence: plan-artifacts/plan-070/{t01/fixture,t02-fixture,t03/probe-t03-final,t04/run-t04-3,t05/{notepad-verified-2,edge-verified,shortcut-autoedit,t04-final}}.log | blockers: 无（AutoEdit 宿主未运行→待测，复验入口见待澄清事项）| next_responsible_role: review

stage: goal | plan_id: PLAN-070 | plan_revision: r1 | round: 4 | from: work | to: review | status: executing | outcome: pass | worktree: D:/autostack/.wt/os-070/auto-os | commit: 268611d | next: review

stage: review | review_scope: final | reviewer_identity: 独立子代理 reviewer（独立上下文；同会话执行者不参与判定）| plan_id: PLAN-070 | plan_revision: r1 | outcome: needs_fix | worktree_path: D:/autostack/.wt/os-070/auto-os | branch: plan-070-dev | reviewed_commit: 268611da495f1d7c8fd646aa6084905bf0b5b92c | base_commit: 61237ba7910bb54e0ba06119f8e43c547777bddf | dependency_revisions: rime.dll=librime 1.13.1（Weasel 0.17.4，sha256 2D8F1BC3…C45B）；resources/README.md 的 6 个资源与 8 个共享数据哈希全符 | spec_inputs: docs/design/05-autoime-windows.md @ a6f2373（r1）；无 docs/specs 变更、无 `.autoos/specs.json` 写入 | acceptance_results: T-01 pass · T-02 pass（F2 契约待调和）· T-03 pass · T-04 pass · T-05 partial（记事本+Edge pass；AutoEdit 待测不计通过）| pending_closeout_gates: AutoEdit 宿主复验；Design 05 §4.2 调和；候选窗 DPI-192 视觉；安装构建非逐字节可复现（MSVC link）| findings: P070-F1（major，测试纪律）+ F2..F5（minor）+ F6..F9（informational）| evidence: plan-artifacts/plan-070/**（11/11 声明哈希复核一致）+ review 轮三套件实跑 | next: work

### 复审发现（needs_fix）

- **P070-F1 · major · 测试纪律**：`ime/test.cmd` 先构建再运行（`build.cmd` 内嵌），warm 2031ms、删 `build/` 后 5026ms（24 步编译）；fixture 二进制本身 43ms 但无 per-case 计时，仅一条 SUMMARY。违反 2026-11-14 测试纪律（日常测试不得隐式编译前置；>10s 新增用例不合格）。修复面：拆出显式准备步骤、`test.cmd` 只运行缺失即具名报错、fixture 输出每例耗时。
- **P070-F2 · minor · 契约冲突**：`RestoreTerminatedOrigin` 在宿主终止组合时按保存的原选区回写，与 Design 05 §4.2“宿主已接管范围以宿主终态为准，禁止按历史偏移回写”冲突，但 T-05 的 Edge 取消又依赖该恢复。需在 Design 05 §4.2 写明有界例外（仅经仍有效的 origin range + 终止 cookie，折叠/无选区路径带内容守卫），或改代码；两处文本不能同时生效。
- **P070-F3 · minor**：快照未携带预编辑光标/选区（`RimeComposition.cursor_pos/sel_start/sel_end` 可用），`UpdateCompositionText` 恒为 end，未满足 Design 05 §3/§4.2“光标跟随引擎位置”。
- **P070-F4 · minor**：候选窗无预编辑行；DPI 只缩放行高/边距，字体仍按 `GetDpiForSystem()` 创建一次；无移动/缩放/滚动重定位钩子，README“翻页/缩放后重新定位”描述超前。
- **P070-F5 · minor**：`verify_registered.exe` 已构建但安装/卸载脚本未调用，未落实 Design 05 §6“升级核对实际注册路径与文件版本”。
- F6 · informational：T-04 D/E 用例实测 8.0–9.3s，10s 门内但余量仅约 0.7s（执行记录 6.6–6.7s，review 两次 8.0–9.3s）；冷缓存超 10s 即升级为 needs_fix。
- F7 · informational：部署/延迟/套件总时长与计划记录同量级但偏大（deploy 2870→3176/4030ms，median 149→446µs/P95 957µs，套件 23.8→28.9/30.8s），均远在断言阈值内。
- F8 · informational：`probe/rime_probe.cpp:918` 注释仍写 `ros2`（代码为 `ros`，偏差已记入 resources/README）；README 的 run-t04 约 24s 实测 29–38s。
- F9 · informational（待测，不计通过）：敏感范围直通/写锁保留仅经代码+夹具；过期点击仅代码；候选窗 DPI-192 与矩形跟随只在未保存的 `%TEMP%\autoime-host.log`。

stage: goal | plan_id: PLAN-070 | plan_revision: r1 | round: 5 | from: review | to: work | status: executing | outcome: needs_fix | worktree: D:/autostack/.wt/os-070/auto-os | commit: 268611d | next: work

### r2 修复与复验（2026-11-17，code `fae70a2`，plan r2 / Design 05 r2）

- **F1 ✅ 测试纪律**：`ime/test.cmd` 只运行预构建夹具，缺件时具名报错
  `preparation missing` 并 exit 2（实测），不再内嵌 `build.cmd`；准备步骤=
  `ime/build.cmd`。fixture 输出每例耗时并汇总（17 cases/146 checks，
  总 7.3ms，最慢 diagnostics 7.6ms）；probe 的 RESULT 也带每例 ms
  （10 用例，最慢 t8 134ms）。无新增 >10s 用例。
- **F2 ✅ 契约调和**：Design 05 → r2，§4.2 增加“有界例外”——仅当宿主在
  取消/切窗时终止组合且仍持有原选区快照时，经终止 `ecWrite` 与仍有效的
  origin range 回写，折叠路径带内容守卫；不得按历史偏移回写或跨上下文。
  计划 r2 记录该修订。
- **F3 ✅ 预编辑位置**：引擎快照新增 `preedit_utf16/preedit_cursor/
  preedit_sel_start/preedit_sel_end`（UTF-16，含代理对换算）；组合光标/选区
  跟随 `RimeComposition.cursor_pos/sel_*`，不再恒在末尾；换算函数抽到
  `edit_model` 并由夹具 `offset_mapping` 覆盖（ASCII、甲😀乙、越界、空串）。
- **F4 ✅ 候选窗**：显示预编辑行（候选行下移、点击索引扣减），字体按窗口 DPI
  重建，候选可见时每 250ms 重查光标并重定位；README 的缩放/重定位描述同步更正。
- **F5 ✅ 安装校验**：`verify_registered` 增加 InprocServer32 路径/文件/版本与
  期望路径比对（正例 exit 0、错误期望 exit 4），`autoime.rc` 增加 0.1.0.0
  版本资源；安装脚本复制并调用它，失败 exit 4；卸载删除该工具。实测安装输出
  `version=0.1.0.0 path_matches_expected=1`。
- **F6 ✅ 恢复余量**：回滚只丢弃 AutoIME 自身编译产物、保留朙月/共享表复用；
  T-04 D/E 从 9.3/9.5s 降到 4.0/3.9s（全 32 checks 通过，套件 19.0s）。
- **F7/F8/F9**：F7 为测量差异，记录不改断言；F8 注释与 README 时长已更正；
  F9 敏感直通/写锁/过期点击仍以代码+夹具为证，保留为待测项。
- **回归**：fixture 17/146 ✅（7.3ms）、probe 10/10 ✅（每例 ≤134ms）、
  T-04 32/32 ✅（A 3.2s/B 0.6s/D 4.0s/E 3.9s/F 3.2s/G 4.1s）。
- **宿主复验（安装构建 0.1.0.0）**：记事本 A/B/G、autoos/ros/tsf 首候选、
  半截拼音切窗后旧窗清空；Edge 新进程 A/B/C 全过；候选窗结构日志
  `ped=1 dpi=192` 且矩形随光标变化（`host-candidate.log`）。
- 证据（sha256）：`t06/fixture-final.log` `D2B7035D…05EC`、
  `t06/probe-final.log` `3510A19C…E2A1`、`t06/t04-r2.log` `04B818D2…3775`、
  `t06/notepad-r2.log` `0D528CE0…7E81`、`t06/edge-r2.log` `9A4E9AB6…F792`、
  `t06/host-candidate.log` `E9AB8BC7…9D67`、`t06/install-4.log` `A7B3CDD5…BB19`。

stage: work | plan_id: PLAN-070 | plan_revision: r2 | outcome: pass（F1–F6 修复，F7–F9 记录/待测）| worktree_path: D:/autostack/.wt/os-070/auto-os | branch: plan-070-dev | base_commit: 61237ba | code_commit: fae70a2 | dependency_revisions: rime.dll=librime 1.13.1 (Weasel 0.17.4) | task_ids: T-01..T-05 | evidence: D:/autostack/plan-artifacts/plan-070/t06/{fixture-final,probe-final,t04-r2,notepad-r2,edge-r2,host-candidate,install-4}.log | blockers: 无（AutoEdit 仍待宿主可用）| next_responsible_role: review

stage: goal | plan_id: PLAN-070 | plan_revision: r2 | round: 6 | from: work | to: review | status: execution_done | outcome: pass | worktree: D:/autostack/.wt/os-070/auto-os | commit: fae70a2 | next: review

stage: review | review_scope: final | reviewer_identity: 独立复-review 子代理 reviewer2（独立上下文，未参与 r2 修复；未重跑 GUI/宿主，复用 t06 宿主日志；librime 1.13.1 上游语义核对）| plan_id: PLAN-070 | plan_revision: r2 | outcome: pass | worktree_path: D:/autostack/.wt/os-070/auto-os | branch: plan-070-dev | reviewed_commit: fae70a2527cb0a1a886c47e5839f701fdd39f7e4 | base_commit: 61237ba7910bb54e0ba06119f8e43c547777bddf | dependency_revisions: rime.dll=librime 1.13.1（Weasel 0.17.4，sha256 2D8F1BC3…C45B）；安装 DLL=审阅构建（sha256 ACFC00F0…8F17），FileVersion 0.1.0.0 | spec_inputs: docs/design/05-autoime-windows.md @ 57e0315（r2 §4.2 有界例外）；无 docs/specs 变更、无 .autoos/specs.json 写入 | acceptance_results: T-01 pass · T-02 pass · T-03 pass · T-04 pass（32/32；D 4036ms / E 3612ms）· T-05 partial（记事本+Edge pass；AutoEdit 待测，不计通过）| pending_closeout_gates: AutoEdit 宿主复验；候选窗/手感人工确认；安装 DLL 非逐字节可复现（MSVC link）；P070-R1/R4 + R5 归并记录 | findings: F1–F9 全闭合（F9 仅记录性待测）；新增 P070-R1（minor，潜在不可达路径）+ R2..R5（informational）| evidence: D:/autostack/plan-artifacts/plan-070/t06/**（7/7 sha256 复核一致）+ 本轮实跑（fixture 6.736ms、probe 10/10 deploy 3025ms、T-04 32/32 suite 18504ms）| next: merge

### 复-review 结论（r2，pass）

- **F1–F6 全闭合**：F1 缺前置→具名错误 exit 2、正常 17 cases/146 checks/6.7ms 且每例计时、
  probe 无隐式构建；F2 Design r2 文本与 `RestoreTerminatedOrigin` 语义一致；F3 快照字段与
  `offset_mapping` 通过；F4 候选窗实现与 README/宿主日志（ped=1 dpi=192、矩形 356→372→390）
  一致；F5 校验器正/负例 0/4、安装日志 version 0.1.0.0；F6 T-04 32/32，D 4036ms/E 3612ms。
- **F7–F9**：F7 同量级测量差异、F8 注释/时长已修、F9 敏感直通/写锁/过期点击仍以代码+夹具
  为证（记录性待测，不计通过）。
- **新增非阻塞项**：P070-R1（minor，潜在不可达路径：多段/半匹配预编辑的选区起点构造与
  librime 字节偏移语义；交付拼音流程为单段、光标在末尾，验收不受影响）；R2（Design r2
  可补一句“TSF 不提供终止原因”）；R3（README 仍写 136 checks / ~30–40s）；R4（回滚选择性
  清理未含依赖方案文件，依赖资源损坏路径未验）；R5（空规范增量建议在 merge 补一句理由）。

stage: goal | plan_id: PLAN-070 | plan_revision: r2 | round: 7 | from: review | to: merge | status: reviewed | outcome: pass | worktree: D:/autostack/.wt/os-070/auto-os | commit: fae70a2 | next: merge

## 待澄清事项

- **（待测，非阻塞）AutoEdit 兼容复验**：T-05 验收时 AutoOS 桌面/AutoEdit
  未运行（仅有 `auto-ui-tool` 进程、无 AutoEdit 窗口），按计划记待测、不计作
  通过。复验入口：启动 AutoOS 桌面并打开
  [041 AutoEdit](../../../auto-ui/examples/ui/041-auto-edit/README.md)，重复
  选区替换/取消、中英混输与半截拼音切焦；AutoIME 自身缺陷另立修复。
- 人工手感（候选窗视觉、点击选词、缩放）建议由用户在最终验收时确认；本计划
  已用宿主真实输入链验证功能，不替代主观手感。
- **P070-R1（minor，非阻塞，随 merge 归档为跟进项）**：多段/半匹配预编辑的选区起点
  构造与 librime 字节偏移语义在交付拼音流程（单段、光标在末尾）不可达；关闭条件=
  后续计划修正 `ShiftStartToRange` 起点/字节→UTF-16 换算，并补非零 `sel_start` 夹具。
- **P070-R2..R5（informational）**：Design 05 补“TSF 不提供终止原因”一句；README
  计数/时长；回滚选择性清理补依赖方案文件并验证依赖损坏路径；merge 时为空的规范增量
  写明“experimental 原型、无 docs/specs 目标”的理由。以上均不阻塞交付。
