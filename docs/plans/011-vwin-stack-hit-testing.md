---
plan_id: PLAN-011
status: drafting               # drafting → executing → execution_done → reviewed → archived
feature_name: vwin-stack-hit-testing
author: [zhaopuming]
created_at: 2026-09-11
updated_at: 2026-09-11
plan_revision: 1

# /auto-plan:review 结束时填写：
supersedes_spec_components: []
new_spec_components: []        # 预填 SD-01（详见 §5 规范增量），复审定稿
touched_goals: []              # 引用 docs/specs/goals.md 的 GOAL-NNN

affects: [auto-lang/docs/specs/auto-lang/ui/architecture.md, ui/iced/virtual_window.rs, ui/iced/renderer.rs]
current_step: 0
total_steps: 5
---

> **来源（PLAN-010 待澄清④ 移交，2026-09-11 立案）**：P010-F1——桌面空白区
> 基础树点击不可达（无菜单开启时 BlankPress/BlankMenu 从不入 VM）。PLAN-010
> work 期新登记，遮蔽其 T5 空白腿实机验收与 PLAN-002 C4/T31 勾销；登记原文
> 见 PLAN-010 `## 待澄清事项` ④。取证细节见 auto-lang `scratch/p010/`
> （t5b_blank.log / t5c_nofloat.log / n6b_repro.log）与本计划 §4。
> 用户裁定（2026-09-11）：单独立项为 vwin/Stack 命中测试专项（否决并入
> PLAN-002 收尾选项）。两计划互链：本计划交付后 PLAN-010 T5 空白腿与
> PLAN-002 C4 解蔽续验。

# [PLAN-011] vwin-stack-hit-testing

## 变更摘要

桌面空白区点击不可达专项：①bisect 定位使 iced Stack levitate 命中机制越界
生效的悬垂层/悬垂部件（嫌疑=vwin 内容部件测量宽于窗 rect）；②修复命中语义
越 rect 传播（命中围栏或悬垂根治，以 bisect 证据定夺）；③解蔽下游——
PLAN-010 T5 空白菜单腿（外点/Esc 实机）与 PLAN-002 C4/T31 复验得以进行。
非 popover 域缺陷（PLAN-010 断点已修且与本缺陷独立：B1 无菜单态零事件、
n6b_repro 旧基线同病=存量）。

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

## 详细设计

（T1/T2 bisect 完成后回填定稿；以下为预研设计与决策点。）

**修复候选**：

- SD 候选 a（命中围栏）：`virtual_window_element` 的 win_stack 包
  `opaque()`（或 desktop_root 逐层围栏）。依据：iced opaque 语义=布局
  bounds 外不报 interaction（待澄清② 源码核对前置）；收益=一处围栏覆盖
  全部越界子件，不逐一追内容部件；风险=需验证 chrome 浮层（标题栏菜单
  T37 在 win_stack 内顶层）不受累。
- SD 候选 b（悬垂根治）：按 T2 定位修正 fit Shrink 链或越界部件（如给
  toast-Stack max 宽高钳制、越界子件改 Scrollable/裁剪容器）。收益=治本
  （绘制/布局/命中三面归位）；风险=动渲染管线回归面大，需金样+全量门。

**不变式**（修复须保持）：vwin 层内交互优先级（App 组件 > 客户区聚焦
mouse_area > 缩放把手）不变；层间 z 序与 GlobalPress 清菜单语义不变；
最小化/分区隐藏不推层语义不变（renderer.rs:15000-15008）。

### 规范增量

| delta_id | 操作 | 目标（auto-lang 仓相对路径） | before/after 规则 | rationale | acceptance IDs |
|---|---|---|---|---|---|
| SD-01 | modify | docs/specs/auto-lang/ui/architecture.md（VirtualWindow 候选 B 条目，~L153） | before：组合语义=Stack/clip/mouse_area，未约束命中越界；after：增补"vwin 层 interaction 报告必须围栏在窗 rect 内（opaque 或等价机制）；clip 仅裁绘制不裁命中" | levitate 越界即全桌面下层失焦（stack.rs:231 机制），命中语义须与绘制裁剪语义显式区分 | AC-01, AC-02, AC-04 |

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

- [ ] **AC-01** 无菜单开启态左击桌面空白 → BlankPress 入 VM。验证：
      自动化脚本实录日志含 `DM::App(AppId(4), BlankPress)`；预期结果=
      到达 daemon update 且 VM handler 执行。
- [ ] **AC-02** 无菜单开启态右击桌面空白 → BlankMenu 入 VM 且空白菜单
      开启。验证：日志 BlankMenu 到达 + 截图菜单可见。
- [ ] **AC-03** 悬垂根因定案：bisect 判定表落档（哪层/哪部件、机制确认
      实录）。验证：scratch/p011/ 含五配置矩阵结果 + 越界部件 bounds
      证据。
- [ ] **AC-04** 修复零回归：回归门四项全绿（失败集全等/layout 35/35/
      a2vue/hash-lock）+ 不回归面实机清单逐项实录通过。
- [ ] **AC-05** 下游解蔽：blank 菜单开启 → 外点左击 → 关闭（BlankClose/
      MenuClose 到达）、ESC → 关闭——PLAN-010 T5 空白腿判据在本线实测
      通过（PLAN-010 残余验收可继续）。
- [ ] **AC-06** PLAN-002 C4/T31 复验路径打通：icon 右键菜单全判据（开启/
      外点关/Esc 关/菜单项样式）实机实录交付，用户复核勾销可进行。

## 执行步骤

（原子任务：精确文件路径 + 确切操作 + 验证命令；每步完成后追加
[✅ 已完成] 一行证据）

- [ ] **T1 悬垂层 bisect 定位** → AC-03。
      操作：renderer.rs:14966-15151 层装配处加 `AUTO_STACK_PROBE` env
      门控（五配置逐层增减，env 缺省=全量零变化）；复制 t5b_blank.ps1
      到 scratch/p011/ 改造（补真关窗腿：点 × + thumb-fallback 消失
      断言）；逐配置跑空白点击采集 BlankPress 可达性。
      验证：五配置判定表落档 scratch/p011/t1_matrix.md；预期=vwin 在场
      聋、sans-vwin 可达（否则按待澄清① 走常驻层支线）。
- [ ] **T2 悬垂部件定位**（依赖 T1）→ AC-03。
      操作：layout_collector/`__bounds_collected` 导出带窗态各层布局盒；
      找测量超出 rect 的部件与报非 None interaction 的子树；区分 fit
      Shrink 链 vs 内容 min 宽超出。
      验证：越界部件 bounds 实录（数值 > rect 对应维）落档。
- [ ] **T3 修复落地**（依赖 T1，T2 证据辅助定案）→ AC-01/02。
      操作：按候选 a/b 择一（opaque 围栏首选落点
      crates/auto-lang/src/ui/iced/virtual_window.rs:437-454；根治候选
      落点 renderer.rs:15941 fit 链）；opaque 语义先经最小例/源码核对
      （待澄清②）；附 headless 回归。
      验证：自动化 B1/B2 实录绿（BlankPress/BlankMenu 到达）+ 新增
      headless 断言绿。
- [ ] **T4 回归门+不回归面**（依赖 T3）→ AC-04。
      验证：cargo t 失败集与基线全等（逐项 stash 对照）；iced-layout-
      tests 35/35；a2vue 金样；hash-lock；不回归面清单逐项实录
      （图标双击/拖拽/缩放/标题栏菜单/dock/launcher/blank 菜单外点）。
- [ ] **T5 解蔽下游+证据包**（依赖 T4）→ AC-05/06。
      操作：blank 菜单外点/Esc 判据实测（PLAN-010 T5 空白腿口径）；icon
      菜单全判据实录（PLAN-002 C4 口径）；证据归档 scratch/p011/；
      PLAN-010/PLAN-002 计划文件回填互链注记。
      验证：AC-05/06 判据日志+截图齐备；用户实机复核清单输出。

## 复审记录

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

1. **T1 若 sans-vwin 配置亦聋**（常驻层嫌疑成立）：修复面扩至
   shell/launcher/scrim 层（同围栏法），目标与 AC 不变——执行期裁定权
   在案，无需回炉改约。
2. **opaque() 语义前置核对**（T3 前置）：需以 iced_widget-0.14.2
   opaque 源码核对或最小例实证"layout bounds 外 mouse_interaction=None
   且不阻子树内命中"；若语义不符，改用等价围栏（层 bounds 交集包装/
   自绘围栏件），候选不锁死。
3. **worktree 线基**（work 启动时定）：PLAN-010 尚未合并，本计划实机
   验证依赖其修复提交；建议 os-011 组 base 取两仓 os-010-dev tip
   （8873772ec / 58b90b2），合并时先落 PLAN-010 链防重复——沿 PLAN-010
   worktree 组先例，待 work 启动时与用户确认。
