---
plan_id: PLAN-070
status: executing
feature_name: AutoIME Windows 日常试用版
author: [Codex]
created_at: 2026-11-17
updated_at: 2026-11-17
plan_revision: 1
depends_on: [PLAN-069]
current_step: 4
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
| T-05 · 集成人工验收 | 用同一交付构建在记事本、浏览器文本框与 [041 AutoEdit](../../../auto-ui/examples/ui/041-auto-edit/README.md) 完成一轮真实键入。更新原型 README 的构建、部署、恢复说明，附合成用例和实际结果。 | 三宿主分别记录选区替换与取消、快捷键、中英混输、半截拼音切焦和候选跟随。AutoEdit 如确因宿主不可用，记待测原因及复验入口，不能计作通过；可复现的 AutoIME 自身缺陷在本计划修复。 |

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

stage: work | plan_id: PLAN-070 | plan_revision: r1 | outcome: pass（T-01 完成，T-02..T-05 续）| worktree_path: D:/autostack/.wt/os-070/auto-os | branch: plan-070-dev | base_commit: 61237ba | code_commit: b130c61 | dependency_revisions: rime.dll=librime 1.13.1 (Weasel 0.17.4, 系统安装未改动) | task_ids: T-01 | evidence: D:/autostack/plan-artifacts/plan-070/t01/fixture.log (sha256 2D645C67…F2FE) | blockers: 无 | next_responsible_role: work (T-02 编辑会话、T-03 方案/候选、T-04 部署)

stage: goal | plan_id: PLAN-070 | plan_revision: r1 | round: 1 | from: work | to: work | status: executing | outcome: pass | worktree: D:/autostack/.wt/os-070/auto-os | commit: b130c61 | next: work

stage: work | plan_id: PLAN-070 | plan_revision: r1 | outcome: pass（T-02/T-03 完成，T-04..T-05 续）| worktree_path: D:/autostack/.wt/os-070/auto-os | branch: plan-070-dev | base_commit: 61237ba | code_commit: 243dd47 | dependency_revisions: rime.dll=librime 1.13.1 (Weasel 0.17.4, 系统安装未改动) | task_ids: T-02, T-03 | evidence: D:/autostack/plan-artifacts/plan-070/t02-fixture.log (sha256 1CB1DE88…6AC4), t03/probe-t03-final.log (sha256 5E40DB10…9F70) | blockers: 无 | next_responsible_role: work (T-04 部署与恢复、T-05 集成人工验收)

stage: goal | plan_id: PLAN-070 | plan_revision: r1 | round: 2 | from: work | to: work | status: executing | outcome: pass | worktree: D:/autostack/.wt/os-070/auto-os | commit: 243dd47 | next: work

stage: work | plan_id: PLAN-070 | plan_revision: r1 | outcome: pass（T-04 完成；T-05 待人工验收）| worktree_path: D:/autostack/.wt/os-070/auto-os | branch: plan-070-dev | base_commit: 61237ba | code_commit: b1ab2a5 | dependency_revisions: rime.dll=librime 1.13.1 (Weasel 0.17.4, 系统安装未改动) | task_ids: T-04 | evidence: D:/autostack/plan-artifacts/plan-070/t04/run-t04-3.log (32 checks, sha256 B00AD90D…BCAC), deploy-measure.log (deploy 2870ms, median 149us/P95 164us) | blockers: 无（T-05 需管理员安装新构建并在真实宿主键入）| next_responsible_role: work (T-05 安装与三宿主验收)

stage: goal | plan_id: PLAN-070 | plan_revision: r1 | round: 3 | from: work | to: work | status: executing | outcome: pass | worktree: D:/autostack/.wt/os-070/auto-os | commit: b1ab2a5 | next: work
