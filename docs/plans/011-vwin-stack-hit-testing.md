---
plan_id: PLAN-011
status: reviewed               # drafting → executing → execution_done → reviewed → archived
feature_name: vwin-stack-hit-testing
author: [zhaopuming]
created_at: 2026-09-11
updated_at: 2026-09-11
plan_revision: 2

# /auto-plan:review 结束时填写：
supersedes_spec_components: []
new_spec_components: ["P011-1"]  # ui/overview.md §已知坑：mouse_area 命中带规则（SD-01，merge 落账）
touched_goals: []              # 引用 docs/specs/goals.md 的 GOAL-NNN

affects: [auto-lang/docs/specs/auto-lang/ui/architecture.md, ui/iced/virtual_window.rs, ui/iced/renderer.rs]
current_step: 5
total_steps: 5
---

> **来源（PLAN-010 待澄清④ 移交，2026-09-11 立案）**：P010-F1——桌面空白区
> 基础树点击不可达（无菜单开启时 BlankPress/BlankMenu 从不入 VM）。PLAN-010
> work 期新登记，遮蔽其 T5 空白腿实机验收与 PLAN-002 C4/T31 勾销；登记原文
> 见 PLAN-010 `## 待澄清事项` ④。取证细节见 auto-lang `scratch/p010/`
> （t5b_blank.log / t5c_nofloat.log / n6b_repro.log）与本计划 §4。
> 用户裁定（2026-09-11）：单独立项为 vwin/Stack 命中测试专项（否决并入
> PLAN-002 收尾选项）。两计划互链：本计划交付后 PLAN-010 T5 空白腿与
> PLAN-002 C4 解蔽续验。**【复审定案 2026-09-11】**levitate/悬垂假设证伪，
> 真根因与修复见"详细设计回填"；outcome: pass → reviewed。

# [PLAN-011] vwin-stack-hit-testing

## 变更摘要

桌面空白区点击不可达专项：①bisect 定位空白点击死区的根因层（**终局：
levitate/悬垂假设证伪**——真根因=iced mouse_area 命中带为内容盒，desktop.at
空白菜单 popover 锚件的 style 尺寸类落外层容器对命中几何 no-op，命中带仅
图标条带高）；②desktop.at 命中带内容件 Fill 改造修复（锚件几何/放置语义/
视觉零变化）；③解蔽下游——PLAN-010 T5 空白菜单腿（外点/Esc 实机）与
PLAN-002 C4/T31 复验得以进行。非 popover 域缺陷（PLAN-010 断点已修且与本
缺陷独立：B1 无菜单态零事件、n6b_repro 旧基线同病=存量）。

## 目标

1. **空白可达**：无菜单开启时，左击桌面空白 → BlankPress 入 VM（到达
   daemon update）；右击空白 → BlankMenu 入 VM 且空白菜单开启。实机自动化
   判据（沿 t5b_blank.ps1 模式）。
2. **根因定案并修复**：悬垂层/部件 bisect 判定表落档；修复使 vwin 层的
   interaction 报告围栏在窗 rect 内（或悬垂源根治），levitate 不再越 rect
   致下层失焦。
3. **零回归**：vwin 交互面（拖拽/八向缩放/标题栏菜单/客户区按钮输入）、
   图标格双击（PLAN-002 C4 链）、任务栏/launcher 点击全部不受累；回归门
   失败集与基线全等零新增。

**非目标**：popover dismiss 语义与同击双投递（PLAN-010 已定案）、N6a 样式、
iced 版本升级、菜单开启期事件语义重设计、dock 条目存量发丝框（PLAN-010
待澄清② 已另登记）。

## 架构方案

三步 bisect（沿 PLAN-010 逐环收窄范式，每步一个可判定实验）：

1. **T1 悬垂层定位**：`AUTO_STACK_PROBE` env 门控 daemon 层装配
   （renderer.rs:14966-15151 五配置矩阵：surface-only / +shell / +launcher
   / +vwin / 全量），每配置跑 t5b_blank.ps1 同款自动化空白点击，以
   BlankPress 可达性逐配置判定。vwin 配置聋且 sans-vwin 配置可达 → 悬垂源
   在 vwin 层；全配置聋 → 常驻层（shell/launcher/scrim）另行走查。
   辅助证据：`__bounds_collected`/layout_collector 导出各层布局盒，直找
   测量超出窗 rect 的部件（越界悬垂直接证据）。
2. **T2 悬垂部件定位**（T1 指 vwin 层时）：win_stack 内逐层 bounds dump
   找报非 None interaction 且越 rect 的部件；区分两候选——fit 常驻 Shrink
   链（renderer.rs:15941 toast-Stack 于 fit_pending||fit_enabled 时
   Shrink×Shrink，内容自然尺寸可宽于 rect）与内容 min 宽超出。
3. **T3 修复定稿+落地**（按 T1/T2 证据择一或组合）：
   - **命中围栏**（保底）：desktop_root / virtual_window_element 以
     iced `opaque()` 包 win_stack（layout bounds 外 interaction=None；
     本仓同病先例：renderer.rs:18994 abs 层 opaque、PLAN-051 P2 空层
     不入栈）。
   - **悬垂根治**：fit Shrink 链或越界部件布局修正（按 T2 定位）。
   修复不锁死于预研候选，以 opaque 语义核对（待澄清②）与 bisect 证据
   为准；不破坏拖拽/缩放/焦点环/标题栏菜单浮层。

## 需求分析与背景调查

**授权记录**：用户 2026-09-11 授权本计划立案（scope=vwin/Stack 命中测试
专项起草；否决 PLAN-010 待澄清④ 的"并入 PLAN-002 收尾"选项）。允许仓库/
动作：auto-os（计划与证据台账）、auto-lang（crates 修复 + scratch 取证；
遵守本仓 AGENTS §3 验证门档）。未设专项预算上限；执行（work）需用户另行
触发 `/auto-plan:work`。

**症状实录**（PLAN-010 work 期，证据 auto-lang `scratch/p010/`）：

| # | 实验 | 结果 |
|---|---|---|
| 1 | t5b B1：干净态左击空白(1100,400) | ✗ 零事件——全日志 grep BlankPress/BlankMenu/BlankClose/pv-panel-press = 0 命中 |
| 2 | t5b B2-B4：右击空白开 blank 菜单 | ✗ BlankMenu 不入 VM（截图见 t5b_blank_open.png） |
| 3 | 旧基线 n6b_repro.log | ✗ BlankPress 亦从未到达（存量，非 PLAN-010 引入） |
| 4 | t5 S3：icon 格右击穿透 | ✓ IconMenu 到达（t5_acceptance.log:462）——图标格可点 |

→ 阻塞呈**位置相关**：icon 格（窗带外）可达、空白（悬垂带内）全聋，与
"vwin 内容越界悬垂带"假设自洽。

**机制源码**（已核对）：

- `iced_widget-0.14.2/src/stack.rs:231-277`：Stack::update 自顶向下逐层
  update，下层 update 前若 `is_over && !cursor.is_levitating()` 且上层
  `mouse_interaction != None` → `cursor = cursor.levitate()`。
- `iced_core-0.14.0/src/mouse/cursor.rs:57`：Levitating 光标使下层
  `is_over` 恒 false → mouse_area 命中全失效（全聋机制）。
- `crates/auto-lang/src/ui/iced/renderer.rs:15153` desktop_root =
  container(Stack::with_children(layers))；层序 14966-15151：壁纸 → scrim
  → surface(AppId(4)) → vwin×N → native slot → drag-over → shell →
  launcher → switcher/notification（后四者按态）。
- `crates/auto-lang/src/ui/iced/virtual_window.rs:437-454`：win_stack
  Fixed(rect.width×height) + 定位容器；:327-341 客户区 mouse_area
  `.interaction(Idle)`（层内合法兜底，但 mouse_interaction 沿子树下探，
  越界子件可越 rect 报点）；:365-386 win_box `clip(true)` **只裁绘制**
  （container.rs:351 viewport 求交），布局/命中越界不受限——悬垂通道。
- `crates/auto-lang/src/ui/iced/renderer.rs:15941`：每 app 视图根
  toast-Stack 于 fit_pending||fit_enabled 时整链 Shrink×Shrink（Plan
  504/512 注记，fit 窗常驻 Shrink 以支持动态重测）——内容自然尺寸测量
  可宽于窗 rect。

**关键反证排除**：t5c_nofloat（"无浮动窗"腿）**无效**——C0 两击未关窗
（t5c_nofloat.log:109 仅 `WindowFocused(Id(1))`；thumb-fallback 25 次、
AppId(1)/AppId(2) 热重载 45 次贯穿全程=两层 vwin 始终在场）。"无窗仍聋"
不成立，悬垂假设未被反证；T1 需补真关窗腿（点 × 后以 thumb-fallback
消失为证）。

**已排除**：PLAN-010 断点（parser on* 分桶 → ondismiss 路由）已修
（auto-lang `8873772ec`），与本缺陷独立；B1 无菜单态零事件即在本修复线
上复测仍聋。

## 详细设计（T1/T2 回填定稿，2026-09-11：levitate 假设证伪，几何根因）

**bisect 判定链（scratch/p011/ 全套实录）**：

1. **T1 层矩阵**（AUTO_STACK_PROBE 五配置 × 空白点击，探针已撤）：FULL 复现
   P010-F1（BlankPress/BlankMenu 零到达，icon 正控 IconMenu×6 可达=管线健
   康）；**no_vwin / no_launcher / no_shell / surface_only 全配置同样聋**——
   vwin/shell/launcher 层全部排除，levitate/越界悬垂假设**证伪**（立会话
   勘察的 t5c nofloat 腿未真关窗的疑点随之消解）。
2. **T2 几何微测**（判定位相关）：格条带内空白 (700,40) 左击→BlankPress ✓
   右击→BlankMenu ✓（t2_probe_FULL.log:132/154）；条带下 (1100,400) 零事件。
   → 阻断面=图标条带以下全桌面，命中带在 surface 视图**内部**。
3. **根因**（转换层机制，renderer.rs MouseArea 臂实证）：DSL `mouse-area`
   的 style（w-full h-full）由 build_container 落在**外层包装 container**
   （Fill×Fill，命中透明），而 iced `mouse_area` 命中带=**内容盒**——
   desktop.at 空白菜单 popover（T36）以全桌面 mouse-area 为锚件，锚的命中
   实际仅图标网格条带高，条带以下即死区。popover 开启期其全屏遮罩接住点外
   （BlankClose 可达）掩盖了死区（PLAN-010 同击语义⑤自洽）。P007-1"Popover
   锚 shrink 上下文 Fill 解析零高"为同族已知约束（本例实为内容盒效应）。

**修复定稿**（desktop.at 结构改造，最小根治）：

- mouse-area 内容包 `col (style: "w-full h-full")`——内容件 Fill 使命中带
  随之全屏；锚件（外层包装）几何不变→popover 放置语义不变；grid/图标格
  结构不动→视觉与交互零变化。auto-os `46a07cf`；pin 快照+金样镜像
  auto-lang `51eb14b1c`（desktop.at=36c7413a0b）。
- **否决的候选**（记录防复辟）：a) opaque 围栏——源码核对（待澄清②）证其
  不围子树越界 interaction 报告，且非本缺陷机制；b) 转换器级 mouse_area
  尺寸类语义修复——正解但波及全部 app 命中面（36 app 回归面），转登记
  KNOWN-DEBT 候选（见规范增量 SD-01）。

**实机验证矩阵**（AC-01/02/05 判据，scratch/p011/）：

| 腿 | 证据 | 结果 |
|---|---|---|
| 死区点 BlankPress（1100,400） | t3_verify.log:134 | ✓ 修复前零事件→修复后到达 |
| 死区点 BlankMenu+菜单开启 | t3_verify.log:156 + t3_v2_blank_menu.png | ✓ 菜单左下开启，ghost 项无方框 |
| blank 菜单外点关闭 | t3_verify.log:238（BlankClose） | ✓ |
| blank 菜单 Esc 关闭 | t3b_esc.log:192→245（BlankMenu→BlankClose） | ✓ |
| 条带内 BlankPress 不回退 | t3c_strip.log:195 | ✓ |
| icon 正控 | t3b_esc.log:273 / t1_matrix_FULL IconMenu×6 | ✓ |
| 撤探针后死区点复证 | t4_sanity.log:208 | ✓ |
| 不回归：双击链 ActivateApp | t4_interact.log:180-184 | ✓ |
| 不回归：dock/launcher（SummonLauncher） | t4_interact.log:233-237 | ✓ |
| 不回归：vwin titlebar 按压（WindowFocused） | t4_interact.log:178 | ✓ |

### 规范增量

| delta_id | 操作 | 目标（auto-lang 仓相对路径） | before/after 规则 | rationale | acceptance IDs |
|---|---|---|---|---|---|
| SD-01 | add | docs/specs/auto-lang/ui/overview.md §已知坑（L247 节；复审定稿） | 新增持久规则："iced mouse_area 命中带=内容盒；DSL mouse-area 的显式尺寸类（w-full/h-full 等）落在外层包装 container，对命中几何 no-op——需要大命中带时必须让内容件自身 Fill（desktop.at P010-F1 先例）；转换器级修复（style 尺寸类进命中带）为 KNOWN-DEBT 候选" | 转换层陷阱属持久规则级知识（P007-1 零高规则同族）；levitate 围栏原案随假设证伪作废 | AC-01, AC-02, AC-04 |

（spec 权威目录在 auto-lang 仓；本计划落 auto-os，delta 复审定稿时同步
auto-lang 侧 specs 台账。）

## 测试设计

- **实机自动化判据**（沿 t5b_blank.ps1 模式，落 scratch/p011/）：
  - 干净态（真关窗，thumb-fallback 消失为证）左击空白 → 日志出现
    `DM::App(AppId(4), BlankPress)`；右击空白 → BlankMenu 到达+菜单截图。
  - 悬垂回归判据：带窗态（vwin 在场）重复 B1/B2——修复前后对照实录。
- **headless 回归**：desktop_surface 试点断言（BlankPress handler 闭环
  锚点 renderer.rs:23979-23985、BlankMenu :24781 已有，试点扩展）；若落
  命中围栏，新增围栏单测（rect 外 interaction=None 谓词）。
- **回归门**：cargo t 失败集与 os-010 线基线（22=22，见 PLAN-010 验收）
  全等零新增；iced-layout-tests 35/35；a2vue 金样再生绿；shell-pack
  hash-lock 绿。
- **不回归面实机清单**：图标格双击开 app（PLAN-002 C4 链）、vwin 拖拽/
  八向缩放/标题栏三键与右键菜单（PLAN-526 T37）、任务栏 dock 点击、
  launcher 召唤/点击、blank 菜单开启后外点关闭（PLAN-010 语义）。

## 验收标准

- [x] **AC-01** 无菜单开启态左击桌面空白 → BlankPress 入 VM。验证：
      自动化脚本实录日志含 `DM::App(AppId(4), BlankPress)`；预期结果=
      到达 daemon update 且 VM handler 执行。
      [✅ 已完成] t3_verify.log:134（死区点 1100,400，修复前零事件）+
      t3c_strip.log:195（条带内不回退）+ t4_sanity.log:208（撤探针后复证）。
- [x] **AC-02** 无菜单开启态右击桌面空白 → BlankMenu 入 VM 且空白菜单
      开启。验证：日志 BlankMenu 到达 + 截图菜单可见。
      [✅ 已完成] t3_verify.log:156 + t3_v2_blank_menu.png（菜单实机首开，
      ghost 项无方框）+ t3b_esc.log:192。
- [x] **AC-03** 悬垂根因定案：bisect 判定表落档（哪层/哪部件、机制确认
      实录）。验证：scratch/p011/ 含五配置矩阵结果 + 越界部件 bounds
      证据。
      [✅ 已完成] 判定表见本计划"详细设计回填"：五配置矩阵（t1_matrix_*.log
      ×5）证伪 levitate→几何微测（t2_probe_FULL.log:132/154/条带下零事件）
      →转换层机制定案（MouseArea 臂 style 落外层容器、命中带=内容盒）。
- [x] **AC-04** 修复零回归：回归门四项全绿（失败集全等/layout 35/35/
      a2vue/hash-lock）+ 不回归面实机清单逐项实录通过。
      [✅ 已完成] 日常档 --no-fail-fast 失败集 22 项与 os-010 线基线（同命令
      在 .wt/os-010 重跑）**名单级全等零新增**（plan370×3/plan492/plan055/
      coverage/lucide/layout×14/charts_gallery，全为存量环境红、与 desktop.at
      零交集；序号差异=并行调度抖动）+
      iced-layout-tests 35/35 + headless 子集 13/13（含金样再生后
      a2vue_desktop_surface）+ hash-lock 四件全等绿；实机不回归面：双击链
      （t4_interact.log:180）+dock/launcher（:233）+vwin titlebar 按压
      （:178）。拖拽/缩放全程断言对合成光标不可靠且代码路径零交集，留
      用户实机复核（沿 PLAN-010 T5 先例）。
- [x] **AC-05** 下游解蔽：blank 菜单开启 → 外点左击 → 关闭（BlankClose/
      MenuClose 到达）、ESC → 关闭——PLAN-010 T5 空白腿判据在本线实测
      通过（PLAN-010 残余验收可继续）。
      [✅ 已完成] 外点 t3_verify.log:238（BlankClose 到达+VM handler OK）；
      ESC t3b_esc.log:245。PLAN-010 T5 空白腿与 C4/T31 的用户实机复核
      随本计划交付解蔽。
- [x] **AC-06** PLAN-002 C4/T31 复验路径打通：icon 右键菜单全判据（开启/
      外点关/Esc 关/菜单项样式）实机实录交付，用户复核勾销可进行。
      [✅ 已完成] icon 菜单开启 IconMenu 到达（t3b_esc.log:273/t1_matrix
      IconMenu×6）+外点关/Esc 关判据同 t5_acceptance 沿用（PLAN-010 已录）
      +本线空白腿补齐；用户实机复核清单见 T5。

## 执行步骤

（原子任务：精确文件路径 + 确切操作 + 验证命令；每步完成后追加
[✅ 已完成] 一行证据）

- [x] **T1 悬垂层 bisect 定位** → AC-03。
      [✅ 已完成] AUTO_STACK_PROBE env 门控五配置矩阵（探针代码判定后已撤，
      回归门在撤探针构建上复跑）；判定：全配置（含 surface_only）空白均聋
      +FULL icon 正控可达→上层全排除、levitate 假设证伪；真关窗腿被矩阵
      设计取代（no_vwin 直接无 vwin 层，强于坐标关窗）。判定表落档
      scratch/p011/t1_matrix_*.log ×5。
- [x] **T2 悬垂部件定位**（依赖 T1）→ AC-03。
      [✅ 已完成] 定位改为几何微测（bounds dump 不需要）：条带内 (700,40)
      可达/条带下 (1100,400) 零事件（t2_probe_FULL.log）→命中间=surface
      视图内部；机制定案=MouseArea 转换臂 style 落外层容器、命中带=内容盒
      （renderer.rs 4265/6323 两臂+apply_container_style 实读）。
- [x] **T3 修复落地**（依赖 T1，T2 证据辅助定案）→ AC-01/02。
      [✅ 已完成] desktop.at mouse-area 内容包 w-full h-full col（auto-os
      `46a07cf`）；opaque 围栏候选经源码核对否决（helpers.rs:577-708）、
      转换器级修复转 KNOWN-DEBT（SD-01）；pin 同步 auto-lang `51eb14b1c`
      （desktop.at=36c7413a0b）。六判据实录绿（详见 AC 表）。
- [x] **T4 回归门+不回归面**（依赖 T3）→ AC-04。
      [✅ 已完成] 撤探针重建→headless 子集 13/13（金样再生后）+iced-layout
      -tests 35/35+日常档 --no-fail-fast 失败集 22 与 os-010 线基线逐一
      全等（/tmp/f011.txt vs /tmp/f010.txt，全存量环境红）+hash-lock 四件
      全等；实机不回归面双击/dock/launcher/titlebar 按压实录绿（拖拽全程
      断言合成光标不可靠，代码路径零交集，留用户实机）。
- [x] **T5 解蔽下游+证据包**（依赖 T4）→ AC-05/06。
      [✅ 已完成] blank 菜单外点/Esc 关闭实测通过（t3_verify.log:238/
      t3b_esc.log:245）——PLAN-010 T5 空白腿判据在本线全绿；证据归档
      scratch/p011/（t1_matrix×5/t2_probe/t3_verify/t3b_esc/t3c_strip/
      t4_sanity/t4_interact + 截图×4）；用户实机复核清单：①空白左/右击
      手感 ②blank 菜单外点/Esc ③icon 菜单外点/Esc→勾销 PLAN-002 C4/T31
      ④vwin 拖拽/缩放手感。PLAN-010/PLAN-002 回填注记见主检出计划。

## 复审记录

### 复审（2026-09-11，同会话复审——独立性限制与工件重构声明）

stage: review | plan_id: PLAN-011 | plan_revision: 2 | outcome: **pass** |
reviewed_commit: auto-os os-011-dev `15f90be`（HEAD；本计划 scope=46a07cf，
其上 15f90be 为 PLAN-010 N6d 并发件不在本计划 scope）+ auto-lang os-011-dev
`51eb14b1c` | base_commit: auto-os `eb88c86` / auto-lang `39ce8d789` |
dependency_revisions: auto-down `afc1cc8`（os-011 组，未改动）|
spec_inputs: docs/specs/auto-lang/ui/overview.md §已知坑（SD-01 落锚，
merge 时落账）；SD-01 规则文本对 renderer.rs MouseArea 双臂
（4265/6323→build_container→apply_container_style）实读复核一致

**独立性声明**：复审与执行同会话，判定按技能要求重构自工件而非执行者
摘要——六项证据行号逐一独立复核命中（t1_matrix_surface_only=0 事件证伪腿、
t2_probe_FULL.log:132/154 条带判别、t3_verify.log:134/156/238、
t3b_esc.log:192/245、金样 51eb14b1c 内容 diff 目检=内包 col 镜像、
46a07cf diff 目检=最小结构改造无夹带）。

acceptance_results: AC-01 ✓pass / AC-02 ✓pass / AC-03 ✓pass / AC-04
✓pass / AC-05 ✓pass / AC-06 ✓pass（6/6）。复审加值门：**cargo tf 复审档
3480 测试唯 1 红=test_charts_gallery_compiles（P007-6 在案 master 存量，
零新增）**；tf 档不带 ui-iced（Plan 507 惯例），iced/layout 面由日常档
--no-fail-fast 22=22 名单级全等收口（f010n/f011n diff 空），组合门完整。

findings:
- F-R1（非阻塞，范围外）：blank 菜单落屏幕左下（锚=全域+BottomStart 既有
  语义）——已在待澄清④ 登记 KNOWN-DEBT 候选。
- F-R2（非阻塞）：拖拽/缩放全程断言合成光标不可靠——t4b/chrome 自动化无
  判定产物；代码路径零交集（auto-lang diff 仅 assets+金样）+ base 线
  PLAN-526/002 用户核准沿用，列入用户实机复核清单④。
- F-R3（记录）：探针脚手架 AUTO_STACK_PROBE 已撤（回归门在撤探针构建上
  复跑：t4_sanity.log:208 死区 BlankPress 复证），判定表以日志+计划回填
  为持久档案。

evidence: scratch/p011/（t1_matrix×5/t2_probe/t3_verify/t3b_esc/t3c_strip/
t4_sanity/t4_interact 日志+截图×4；worktree 移除后以本记录行号摘要为持久
档案）；回归门命令与结果摘录见 AC 表与详细设计回填节。

next: merge（/auto-plan:merge——合并时先落 PLAN-010 链：os-011-dev 同时
携带 N6c/N6d 与本计划根修；SD-01 随 merge 落 auto-lang specs 台账）

### work 交接（2026-09-11）

stage: work | plan_id: PLAN-011 | plan_revision: 2 | outcome: pass |
code_commit: auto-os os-011-dev `46a07cf`（shell/desktop.at 根修），auto-lang
os-011-dev `51eb14b1c`（pin 快照+a2vue 金样镜像）| task_ids: T1,T2,T3,T4,T5

evidence: 根因=iced mouse_area 命中带为内容盒、DSL mouse-area 的 style
尺寸类落外层包装 container 对命中几何 no-op——desktop.at 空白菜单 popover
锚件命中带仅图标条带高，条带以下全桌面 BlankPress/BlankMenu 死区（bisect
三段实证：AUTO_STACK_PROBE 五配置矩阵证伪 levitate/vwin 假设〔t1_matrix_*.log
×5〕→格条带几何微测〔t2_probe_FULL.log:132/154〕→转换层机制实读定案）。
修复=mouse-area 内容包 w-full h-full col（锚件几何/放置语义/视觉零变化）。
实机六判据全绿（死区 BlankPress/BlankMenu、菜单外点关、Esc 关、条带不回退、
icon 正控；t3_verify/t3b_esc/t3c_strip/t4_sanity）+不回归面（双击链 ActivateApp/
dock SummonLauncher/titlebar 按压，t4_interact）+回归门四项（失败集 22=22
名单级全等〔与 .wt/os-010 同命令基线 diff 空〕/iced-layout-tests 35/35/
headless 子集 13/13 含金样再生/hash-lock 四件全等）。plan_revision 1→2：
设计随 bisect 证据 pivot（levitate→几何），目标/验收/范围不变。

blockers: 无（拖拽/缩放全程断言受合成光标限制，与修复代码路径零交集，
随用户实机复核清单交付）

concurrent: auto-os os-011-dev 于本计划 work 期间出现用户侧并发提交
`15f90be`（PLAN-010 N6d launcher 外点关闭，stacked on 46a07cf，祖先关系
已验证）——本计划全部回归门/实机证据产生于 `46a07cf`+`51eb14b1c` 构建，
N6d 不触及 desktop.at/desktop_surface 面；合并时 os-011-dev 同时携带
PLAN-010 N6d 与本计划根修，先落 PLAN-010 链。

next: review（用户实机复核清单：①空白左/右击手感 ②blank 菜单外点/Esc
③icon 菜单外点/Esc→勾销 PLAN-002 C4/T31 ④vwin 拖拽/缩放手感；worktree
保留供复审）

### 起草交接（2026-09-11）

stage: new | plan_id: PLAN-011 | plan_revision: 1 | outcome: pass（起草
完成，待 work 触发；授权范围=§4 授权记录，无越权）

背景勘察证据：t5b_blank.log 零事件实锤、t5c nofloat 腿无效判定（未真关
窗）、levitate 机制源码双端核对（iced stack.rs:231 + cursor.rs:57）、
desktop_root 层序与 vwin 组合锚点、clip 不裁命中通道确认。悬垂假设自洽
未被反证；修复候选 a/b 以 bisect 证据定夺，未预锁实现。

next: work（建议 worktree 组 `.wt/os-011/` 三仓，auto-os/auto-lang base
取 os-010 线 tip——本计划验证依赖 PLAN-010 修复提交 8873772ec/58b90b2，
见待澄清③）

## 待澄清事项

1. ~~T1 若 sans-vwin 配置亦聋~~ **已定案（2026-09-11 work）**：矩阵实测
   全配置皆聋→阻断在 surface 视图内部（非上层），支线成为主干——根因
   几何定案见详细设计回填。
2. ~~opaque() 语义前置核对~~ **已定案（2026-09-11 work 源码核对）**：iced
   0.14.2 Opaque（helpers.rs:577-708）只在"子树报 None 且 cursor 在
   bounds 内"补 Idle/捕获 press；**越界悬垂子件自报的非 None interaction
   原样穿透**——opaque 围不住本缺陷病源。候选 a 升级为自写命中围栏件
   （HitFence：bounds 外 mouse_interaction 强制 None + update 不转发，
   同时防致聋与防悬垂子件截获点击）；候选 b 不变，T1/T2 后定夺。
   **（终局）**：levitate 假设整体证伪，HitFence 候选一并作废，修复走
   desktop.at 命中带改造（详细设计回填）。
3. ~~worktree 线基~~ **已定（2026-09-11 work 启动）**：os-011 三仓组
   （.wt/os-011/）base 取 os-010 线 tip——auto-lang `39ce8d789`（含
   8873772ec N6b 根修+N6c）、auto-os `eb88c86`、auto-down `afc1cc8`；
   分支 os-011-dev。合并时先落 PLAN-010 链防重复。
4. **blank 菜单放置位**（work 新登记，非本计划 scope）：blank 菜单面板
   落屏幕左下（锚=全桌面+BottomStart 的既有语义，t3_v2_blank_menu.png；
   经 overlay 绘制不被 dock 遮挡、ghost 项无方框）。Windows 惯例是菜单
   随点击处弹出——需 BlankMenu 事件携带坐标→popover at_point 锚，涉
   mouse-area 事件签名扩展，另立计划/KNOWN-DEBT 候选。无用户基线，不阻
   本计划验收。
