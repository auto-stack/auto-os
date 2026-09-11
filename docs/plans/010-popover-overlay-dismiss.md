---
plan_id: PLAN-010
status: executing              # drafting → executing → execution_done → reviewed → archived
feature_name: popover-overlay-dismiss
author: [zhaopuming]
created_at: 2026-09-10
updated_at: 2026-09-10

# /auto-plan:review 结束时填写：
supersedes_spec_components: []
new_spec_components: []
touched_goals: []             # 引用 docs/specs/goals.md 的 GOAL-NNN

affects: [ui/iced, virtual_window.rs, popover.rs]   # 受影响的 specs 路径
current_step: 4
total_steps: 5
---

> **来源（PLAN-002 复审发现移交，2026-09-10）**：本计划承接 PLAN-002 C 复核
> 登记的 N6b（桌面壳弹层外点不关闭+开启期基础树事件整吞）与 N6a（弹层菜单
> 项方框样式）。取证细节见 PLAN-002 `## 复审记录` 三条 work 交接记录
> （2026-09-09/10）与 auto-lang `scratch/p002/`（n6b_repro.ps1、探针插桩
> `fd2b85ea3`、隔离例 `ee986f0cb`）。两计划互链：修复落地后 PLAN-002 C4
> （T31 右键菜单）凭本计划交付复验勾销。
>
> **worktree 组（2026-09-10 work 登记）**：`D:/autostack/.wt/os-010/` 三仓
> ——auto-os（os-010-dev，base=844b7c8=os-002-dev tip）+ auto-lang
> （os-010-dev，base=ee986f0cb=os-002-dev tip，含 fd2b85ea3 探针插桩与隔离
> 例——取证态一致）+ auto-down（os-010-dev，base=afc1cc8=组内依赖）。
> 注意：两仓 os-010-dev 与 os-002-dev 共享历史（base 取其 tip），merge 时
> 须先落/同溯 PLAN-002 链，防重复合入。work 交付：auto-lang `8873772ec`、
> auto-os `58b90b2`。
>
> **用户实机复核轮（2026-09-11）**：icon 菜单外点关闭 ✓ 用户确认；新增
> **N6c**（dock 菜单被 hover-leave 秒关、菜单项不可达）——已修：auto-os
> `eb88c86` + auto-lang `39ce8d789`（shell.at pin c207f048e5），dock 菜单
> 统一为 icon 菜单同构（WinMenuClose/HoverLeave 拆分），实机四判据实录
> n6c_dock_menu.log。新版桌面已交付用户续复核。

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

## 详细设计（T3 回填定稿：断点线）

**断点定位（bisect 三步实录，2026-09-10）**：

1. **T1 探针 B**（`ui_popover_probe2` 补 `.font(INTER_FONT_*)×3+
   .default_font+.theme(Dark)` 接线，shadcn_theme 为私有 fn 以 Dark 达
   "内容可见"目的——主题不参与消息路由不影响判据）：外点→`[pv-dismiss]
   publishing`→`DM::App reached update`→`Dismissed #1`；ESC→`Dismissed #2`。
   **判定：daemon+DM 包装层健康，断点在桌面更深宿主结构**。
2. **T2 dock vs icon**：icon 腿（surface 层）复现——`[pv-panel-press]
   over_panel=false`→publish 后无到达、BlankPress 同期不可达；dock 腿被
   hover-leave 遮蔽（合成光标移动即触发 onmouseleave→HoverEnd 先关菜单，
   真实用户外点同样被 hover-leave 兜住）——dock 腿对"外点 dismiss"不可
   测，改判据为 ESC。
3. **T3 定位**：`AUTO_DEBUG_KEYS` 入口全量探针实锤——**消息一直都在到达
   daemon update，但事件名是 `__popover_close` 而非 `MenuClose`**！链路
   逐环核对（iced 0.14.0 `UserInterface::update` overlay 分支/
   `overlay::Nested`/`Group`/`Element::Map`/`Shell::merge`，源码全查无误，
   与 T1 互证）。真断点在 **aura 解析层的 prop 分桶**：`parser.rs:15561`
   把一切 `on*` 键升格为 ViewEvent（events 桶），`ondismiss` 从不入
   props；`convert_popover` 只查 props→恒 miss→合成 `__popover_close`；
   该事件的处理臂（renderer.rs:12317）只清 menubar 自管开合全局
   （`action_config::popover_open`），对 VM 态 popover（desktop.at 的
   `menu_id`/`blank_menu` 状态驱动 open）是 no-op。menubar 时代未暴露：
   自管模式恰好被该回退臂服务；桌面 VM 态 popover 是首个显式 ondismiss
   消费者。**旁路线否决**：无需 scrim/subscription 兜底——路由本身健康，
   修消息内容即可（断点线）。

**修复（8873772ec）**：`convert_popover` 增 `events` 参数（两调用点同
传），ondismiss 提取顺序 props→events（`aura_events_get_base` 基名大小写
不敏感兜底，覆盖 `ondismiss.prevent` 修饰形态）；`__popover_close` 回退保
留（无显式 ondismiss 的自管形态仍走原语义）。headless 回归
`p010_popover_ondismiss_extracted_from_events`：MenuClose×4（icon）+
BlankClose×1（blank）提取、convert_view_messages 后存活、MenuClose
handler 闭环（menu_id 清位）。

**N6a 定案（同提交）**：方框来源=PLAN-571 default 按钮预设的发丝描边
（`bg-muted border border-border …`），无 variant 按钮全中（用户类
bg-transparent 只覆盖填充不覆盖描边；build_button_style/visual wrap 的
width=0 不可见——PLAN-002 待澄清②的谜底）。不动预设表（有互锁测试锚定，
dock 条目等处的存量发丝框另行登记），菜单项按钮显式 `variant: "ghost"`
（关闭/退出按语义 primary/ghost），iced/vue 双端 cva 同步去框。

## 测试设计

- **实机判据**（自动化脚本，沿 n6b_repro.ps1 模式）：右键开菜单 → 脚本
  点空白 → 日志须出现 MenuClose（且 BlankPress 可达）；ESC → MenuClose。
- **headless 回归**：desktop_surface_at_loads_interactions_and_dispatch
  试点断言（rc=4/hv=4）+ layout_tests 35/35 + 全量 cargo t 失败集与
  master 全等（当前基线 3=3，见 PLAN-002 复审记录 2026-09-09）。
- **N6a 目检**：菜单项无方框、hover 提亮与任务栏菜单一致（截图留档）。

## 验收标准

- [x] 桌面壳内 popover 外点点击自动关闭：icon 菜单 ✓（t5 实录
      `[pv-dismiss]`→`MenuClose` 到达+截图菜单消失）、任务栏 dock 菜单 ✓
      （ESC→HoverEnd 到达；外点由 hover-leave 天然兜住，合成光标无法分
      离两者）、空白菜单 headless 验证 ✓（ondismiss=BlankClose 提取+
      handler 闭环）实机待用户复核（空白右键被 P010-F1 遮蔽，见待澄清）。
- [~] 菜单开启期基础树事件可达：✓ 实证一路——icon 菜单开启期右键另一
      图标 `IconMenu 015-notes` 穿透到达并切换菜单（t5 S3）；✗ BlankPress
      本体——空白左/右击在"无菜单开启"时同样不可达（t5b B1），归因
      P010-F1（vwin 越界悬垂 levitate，menu 无关的独立存量），旧基线
      n6b_repro.log 中 BlankPress 亦从未到达过（非本计划引入）。
- [x] ESC 关闭弹层（icon 菜单实测）：t5 S2 `MenuClose` 到达+菜单消失。
- [x] N6a：弹层菜单项样式与常规右键菜单一致（无逐项方框）——icon/dock
      两菜单修复后截图（t5_n6a_icon_zoom.png / t5_s7_dock_menu_n6a.png
      对照 t2_icon_menu_zoom.png），a2vue 金样再生双端一致。
- [x] 回归门：cargo t 失败集 22=22 与 os-002 线基线全等（逐一 stash 对照
      零新增；master 基线 3 为 PLAN-002 记录值，本线 base 已含 19 个存量
      环境红——layout×15/lucide/coverage/plan055/plan492/plan370——与本次
      改动无关，已逐一验证在 base 同样失败）+ a2vue 金样再生绿 +
      iced-layout-tests 35/35 + popover/menubar/desktop_surface 子集全绿
      + shell-pack hash-lock 绿。实机像素/事件证据留档
      auto-lang `scratch/p010/`（t1/t2/t3/t5/t5b/t5c 日志+截图）。

## 执行步骤

（原子任务：精确文件路径 + 确切操作 + 验证命令；每步完成后追加
[✅ 已完成] 一行证据）

- [x] T1 补齐探针 B 渲染接线（`.font`+`.theme`，对齐 renderer.rs:15214
      daemon 装配），外点测试判 daemon 层。
      [✅ 已完成] ui_popover_probe2 补 .font×3+.default_font+.theme(Dark)；
      实录外点→Dismissed #1、ESC→Dismissed #2 均达 daemon update——
      **daemon 层健康，断点在桌面宿主结构**（scratch/p010/t1_probe_b.log）。
- [x] T2 真桌面 dock vs icon 菜单外点对比（收窄宿主层）。
      [✅ 已完成] icon(surface 层)复现：publish 后零到达+BlankPress 不可
      达（t2_desktop.log）；dock 腿被 hover-leave 遮蔽不可测（t2c_dock.log
      无 pv-panel-press，HoverEnd 先至）——改判据 ESC；配合
      `AUTO_DEBUG_KEYS` 入口探针收窄完成（t3_entry_probe.log）。
- [x] T3 断点定位 + 修复方案定稿（旁路线/断点线二选一或组合）。
      [✅ 已完成] 入口探针实锤 `DM::App(AppId(4), __popover_close)` 到达
      ——消息内容错，非路由丢；根修断点线定稿（parser on* 分桶→
      convert_popover 只查 props→回退事件对 VM 态 popover no-op；详见
      详细设计回填）。
- [x] T4 修复落地 + N6a 样式修复 + 回归门。
      [✅ 已完成] auto-lang `8873772ec`（convert_popover events 兜底+
      headless 回归+探针 B 接线+pack pin+金样再生）+ auto-os `58b90b2`
      （desktop.at/shell.at 菜单项 variant）；回归门：失败集 22=22 零新增
      （stash 逐项对照）+ iced-layout-tests 35/35 + a2vue 15/15。
- [~] T5 实机验收三面（icon/任务栏/空白 外点+Esc）+ PLAN-002 C4/T31
      复验勾销。
      [部分完成] icon 外点/ESC ✓、dock ESC ✓、N6a 双菜单截图 ✓
      （t5_acceptance.log + 截图）；空白腿与 PLAN-002 C4/T31 用户复核待
      用户实机进行——空白右键开菜单在本环境被 P010-F1 遮蔽（见待澄清），
      修复机制本身已由 headless（BlankClose 提取+闭环）与 icon/dock 实机
      同构验证。

## 复审记录

（无）

### work 交接记录（2026-09-10）

stage: work | plan_id: PLAN-010 | plan_revision: 0 | outcome: partial
（T1-T4 完成，T5 空白腿+C4 勾销待用户实机；保持 `executing`） |
code_commit: auto-lang os-010-dev `8873772ec`，auto-os os-010-dev
`58b90b2` | task_ids: T1,T2,T3,T4,T5(部分)

evidence: 根修断点=parser on* 分桶使 ondismiss 落 events 桶而
convert_popover 只查 props（t3_entry_probe.log 实录 `__popover_close`
到达=no-op 臂；入口探针 AUTO_DEBUG_KEYS 定案）；修复后 t5_acceptance.log：
icon 外点/ESC→MenuClose 到达、icon 菜单开启期右键另一图标 IconMenu
穿透到达（基础树可达实证）、dock ESC→HoverEnd 到达、面板内 打开→
MenuOpen 到达；N6a 修复前后截图（t2_icon_menu_zoom.png 有框 →
t5_n6a_icon_zoom.png / t5_s7_dock_menu_n6a.png 无框）；headless
p010_popover_ondismiss_extracted_from_events 绿；回归门 cargo t 失败集
22=22（os-002 线 base 存量，stash 逐项对照零新增）+ iced-layout-tests
35/35 + a2vue 15/15 + hash-lock 绿

blockers: P010-F1（新登记，见待澄清）遮蔽空白菜单实机腿与 BlankPress
判据；PLAN-002 C4/T31 勾销与空白菜单外点/Esc 需用户实机复核

next: 用户实机复核（清单：①右键空白开 blank 菜单→外点左击→关闭？
②blank 菜单 ESC→关闭？③icon 菜单外点/Esc 手感确认→勾销 PLAN-002
C4/T31）；复核通过后径入 /auto-plan:review

## 待澄清事项

1. ~~修复路线（旁路线 vs 断点线）~~ 已定：断点线（T3 实证路由健康，无需
   旁路），详见详细设计回填。
2. ~~N6a 方框来源~~ 已定：PLAN-571 default 预设发丝描边
   （`border border-border`），无 variant 按钮全中。菜单项已显式
   ghost/primary 去框；**dock 条目/启动钮等处的存量发丝框**不在本计划
   范围，留 KNOWN-DEBT 候选（预设表变更需连带互锁测试，宜专项）。
3. ~~旁路线为长期方案~~ 失效：断点线已落地，无长期旁路。
4. **P010-F1（work 新登记，阻塞 T5 空白腿实机验收）**：桌面空白区域的
   基础树点击不可达——无菜单开启时左击/右击空白（BlankPress/BlankMenu）
   均不入 VM（t5b_blank.log B1 零事件；旧基线 n6b_repro.log 中
   BlankPress 亦从未到达，非本计划引入）。嫌疑机制：vwin 内容部件越界
   悬垂（组件测量宽于窗 rect）使 iced Stack::update 的 levitate 生效
   （上层报交互→下层光标 Levitating→mouse-area 全部失焦；源码
   iced_widget-0.14.2/stack.rs:231），与 S3（图标格穿透成功）、B1（无
   菜单同样聋）全部自洽。归属：vwin/Stack 命中测试专项（非 popover 域），
   建议单独立项或在 PLAN-002 收尾时合并处置。
5. 同击语义注记：外点关闭的那一次点击，MenuClose 于 press 期发布→视图
   重建吃掉同击 release，BlankPress 不与 MenuClose 同击触发（菜单已关=
   用户目标达成）；BlankPress 需下一次独立点击。此为 dismiss+重建时序的
   固有形态，如需"一次点击双投递"须改 overlay 捕获语义（不建议）。
