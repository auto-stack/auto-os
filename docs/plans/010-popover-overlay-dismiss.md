---
plan_id: PLAN-010
status: drafting               # drafting → executing → execution_done → reviewed → archived
feature_name: popover-overlay-dismiss
author: [zhaopuming]
created_at: 2026-09-10
updated_at: 2026-09-10

# /auto-plan:review 结束时填写：
supersedes_spec_components: []
new_spec_components: []
touched_goals: []             # 引用 docs/specs/goals.md 的 GOAL-NNN

affects: [ui/iced, virtual_window.rs, popover.rs]   # 受影响的 specs 路径
current_step: 0
total_steps: 5
---

> **来源（PLAN-002 复审发现移交，2026-09-10）**：本计划承接 PLAN-002 C 复核
> 登记的 N6b（桌面壳弹层外点不关闭+开启期基础树事件整吞）与 N6a（弹层菜单
> 项方框样式）。取证细节见 PLAN-002 `## 复审记录` 三条 work 交接记录
> （2026-09-09/10）与 auto-lang `scratch/p002/`（n6b_repro.ps1、探针插桩
> `fd2b85ea3`、隔离例 `ee986f0cb`）。两计划互链：修复落地后 PLAN-002 C4
> （T31 右键菜单）凭本计划交付复验勾销。

# [PLAN-010] popover-overlay-dismiss

## 变更摘要

桌面壳弹层（popover/overlay）**外点关闭失效**专项：①定位 iced 0.14
overlay→runtime 消息管线在桌面 daemon 宿主环境中的断点；②修复外点/Esc
关闭并消除"菜单开启期基础树事件整吞"（BlankPress 不可达）；③顺带修复
PLAN-002 N6a（弹层菜单项方框样式）。已完成的隔离取证与三步 bisect 计划
见"需求分析与背景调查"。

## 目标

1. 桌面壳内 popover 外点点击自动关闭（含 Esc），菜单开启期不再整吞基础树
   事件（BlankPress 等正常可达）。
2. 弹层菜单项样式与常规右键菜单一致（N6a：去除每项方框）。
3. PLAN-002 C4（T31 桌面图标右键菜单）凭本计划交付复验勾销。

## 架构方案

三步 bisect（每步一个可判定实验，依次收窄）：

1. **补齐探针 B**：`ui_popover_probe2`（daemon+DM 包装最小例，`ee986f0cb`）
   补桌面同款 `.font(INTER_FONT_*)+.theme(shadcn_theme)` 接线使内容可见
   ——外点一测即判：失败=daemon 层即可复现（消息包装/多窗口运行时），
   正常=断点在桌面更深宿主结构。
2. **真桌面宿主层对比**：步骤 1 若正常，则在真桌面以 AUTO_POPOVER_DEBUG
   探针对比 **dock 菜单（shell.at 层）vs icon 菜单（desktop.at surface
   层）** 的外点行为，收窄到具体宿主层（surface 层/vwin 层/Stack 分层）。
3. **修复**（视 bisect 结果二选一或组合）：
   - **旁路线**（保底）：dismiss 改走 base 树外旁路——subscription 全局
     键盘 Esc 直发 MenuClose + 菜单开启期定时器/交互兜底；外点整吞另以
     scrim 层（base 树内全屏 mouse-area，菜单开启时挂）承接点外。
   - **断点线**（根治）：按 bisect 结果修 iced 集成断点（如 overlay 消息
     在 DM 包装链中的路由缺口）。

## 需求分析与背景调查

**症状**（PLAN-002 C4 复核，2026-09-09）：桌面图标右键菜单开启后，点击
菜单外任意处（空白/其他格）菜单不消失；菜单开启期间整个桌面基础层对输入
全聋（空白点击的 BlankPress 也不触发）；ESC 同样无效。唯一可关闭路径=
点面板内菜单项。

**取证实锤**（探针四挂点 + 自动化点击脚本，`fd2b85ea3`/`ee986f0cb`，
scratch/p002/n6b_repro.* 与 probe_*）：

| # | 环节 | 结果 |
|---|---|---|
| 1 | DSL 接线（aura widget 锚臂 extract ondismiss→View::Popover） | ✓ 正确 |
| 2 | overlay 注册（每帧 pv-overlay 探针） | ✓ 正常 |
| 3 | Panel::update 收点外事件（over_panel=false 判定+dismiss 分支执行） | ✓ 正常 |
| 4 | on_dismiss 发布（pv-dismiss 探针，含 ESC 路径 ×4） | ✓ 已发布 |
| 5 | **消息到达 daemon update（DM::App/DM::Window 臂探针）** | ✗ **0 次** |
| 6 | 菜单开启期基础树事件（BlankPress） | ✗ 整吞 |

**隔离矩阵**：

- **A 独立窗口**（iced::application 直跑、无 DM 包装，`ui_popover_probe`）：
  外点自动关闭**正常**——用户手测确认 + probe_standalone.log
  Toggle→Dismissed 循环实录。popover widget 本体健康。
- **C 桌面壳 icon 菜单**（daemon+深层嵌套）：如上表，坏。
- **B 裸 daemon+DM 包装**（`ui_popover_probe2`）：无效测试床——裸 daemon
  缺主题/字体接线窗口全黑（0×0 client 系未开窗所致已修，留窗即黑屏）。

**已排除**：popover.rs 判定层（over_panel/over_anchor/modal 分支）、
iced core `overlay::Element::Map`（local_shell+merge 映射，源码核对无误）、
DSL 提取层。

**残局假设**（bisect 目标）：daemon 多窗口运行时消息排空路径 / DM 包装链
在 overlay 场景的路由缺口 / 桌面深层 widget 树（Stack 分层）对 overlay
事件派发的干扰。

## 详细设计

（bisect 步骤 1/2 完成后回填：断点定位 + 修复方案定稿。旁路线预设计——

- **Esc 旁路**：桌面已有 hotkey 订阅体系（Ctrl+Space/Ctrl+Tab 先例），
  增"菜单开启态 + Esc → MenuClose"订阅臂，绕开 overlay 消息面。
- **点外旁路**：菜单开启时在 surface 根挂全屏透明 mouse-area（z 序置于
  菜单面板之下、其余内容之上），on_press → MenuClose；面板自身点击仍由
  overlay 层承接。此层属 base 树——但基础树全聋问题若不先解，旁路同样
  不可达；故点外旁路以 bisect 查明"基础树全聋"根因为前提，或改由
  global subscription 层（win32 钩子先例 native_dock_event_subscription）
  承接。）

## 测试设计

- **实机判据**（自动化脚本，沿 n6b_repro.ps1 模式）：右键开菜单 → 脚本
  点空白 → 日志须出现 MenuClose（且 BlankPress 可达）；ESC → MenuClose。
- **headless 回归**：desktop_surface_at_loads_interactions_and_dispatch
  试点断言（rc=4/hv=4）+ layout_tests 35/35 + 全量 cargo t 失败集与
  master 全等（当前基线 3=3，见 PLAN-002 复审记录 2026-09-09）。
- **N6a 目检**：菜单项无方框、hover 提亮与任务栏菜单一致（截图留档）。

## 验收标准

- [ ] 桌面壳内 popover 外点点击自动关闭（icon 菜单+任务栏菜单+空白菜单
      三面实测），日志实录 MenuClose/WinMenuClose 到达。
- [ ] 菜单开启期基础树事件可达（BlankPress 在菜单开启时触发）。
- [ ] ESC 关闭弹层（icon 菜单实测）。
- [ ] N6a：弹层菜单项样式与常规右键菜单一致（无逐项方框）。
- [ ] 回归门：cargo t 失败集与 master 全等（零新增失败）+ 实机像素/事件
      证据留档 scratch。

## 执行步骤

（原子任务：精确文件路径 + 确切操作 + 验证命令；每步完成后追加
[✅ 已完成] 一行证据）

- [ ] T1 补齐探针 B 渲染接线（`.font`+`.theme`，对齐 renderer.rs:15214
      daemon 装配），外点测试判 daemon 层。
- [ ] T2 真桌面 dock vs icon 菜单外点对比（收窄宿主层）。
- [ ] T3 断点定位 + 修复方案定稿（旁路线/断点线二选一或组合）。
- [ ] T4 修复落地 + N6a 样式修复 + 回归门。
- [ ] T5 实机验收三面（icon/任务栏/空白菜单外点+Esc）+ PLAN-002 C4/T31
      复验勾销。

## 复审记录

（无）

## 待澄清事项

1. 修复路线（旁路线 vs 断点线）视 T1/T2 bisect 结果定，T3 回填。
2. N6a 方框来源未定（build_button_style/visual wrap 的 border 均
   width=0 不可见，截图方框来源待一查）——T4 一并定位。
3. 若断点确认为 iced 0.14 上游问题且短期不可修，是否接受旁路线为长期
   方案（用户裁定）。
