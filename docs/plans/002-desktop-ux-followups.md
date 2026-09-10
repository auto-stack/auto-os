---
plan_id: PLAN-002
origin: PLAN-535
status: executing              # drafting → executing → execution_done → reviewed → archived
feature_name: desktop-ux-followups
author: [zhaopuming]
created_at: 2026-09-04
updated_at: 2026-09-09

# /auto-plan:review 结束时填写：
supersedes_spec_components: []
new_spec_components: []
touched_goals: []             # 引用 docs/specs/goals.md 的 GOAL-NNN

affects: [ui/session, ui/iced, virtual_window.rs, popover.rs, code_editor]                   # 受影响的 specs 路径，如 [auto-lang/vm]
current_step: 3
total_steps: 5
---

> **随迁注记（Stage B P-1，2026-09-07，PLAN-001）**：本计划自 auto-lang
> `docs/plans/535-desktop-ux-followups.md`（origin PLAN-535）随迁重编为 PLAN-002——drafting 原状、
> 状态机续用，git 历史留 auto-lang 仓。设计依据
> [Design 01 §5](../design/01-stage-b-desktop-migration.md)；auto-lang 侧指针行见
> 其 `docs/plans/INDEX.md`。正文中的 auto-lang 相对路径/行号锚点，开工时按本仓
> AGENTS §2 解析序换算（env → `../auto-lang` → `D:/autostack/auto-lang`），
> 现文不回改。

# [PLAN-002] desktop-ux-followups

## 变更摘要

**滚动跟踪计划**（528 先例）：PLAN-526（桌面 Shell UX 十一波 39 任务，已归档
`docs/plans/archive/526-desktop-shell-ux-fixes.md`）复审后登记的未竟项与用户复核项，
统一在本计划继续跟踪。每个条目逐条转为执行步骤修复并验证；新的桌面问题亦可在此滚动登记。

承接自 526 的四组：
- **A 已知视觉债两项**（KNOWN-DEBT 🟢）：popover 首次打开横向锚点偏左；
  window_thumbnail 懒捕获前显示空。
- **B 公共基建一项**（KNOWN-DEBT 🟡，用户核准延后）：wrap_layout_onclick
  布局件级 hover/右键公共基建。
- **C 用户复核清单**（功能已实现+测试承载，真实手势/视觉待确认）：
  T20 跨窗首击、T24 最大化底缘、T28 三键 hover 盒、T31 icon 双击打开、
  T32 Ctrl+Tab 松手提交、T37 标题栏右键菜单、T38 launcher 滚动。
- **D 崩溃观察**：桌面进程常驻 RUST_BACKTRACE=1（8916 实例已带）——
  code_editor clamp（526 T21）与截图零尺寸（T39）两处已修；若再发同族
  panic 直接取全栈定位。

## 目标

1. 修复 popover 首开横向锚点偏左（首次打开时定位偏移，之后正常）。
2. 修复 window_thumbnail 懒捕获前空白显示。
3. wrap_layout_onclick 布局件级 hover/右键公共基建落地（或明确再延后并记录理由）。
4. 用户复核清单逐项确认（复核通过即勾销；发现问题转为新任务）。

## 架构方案

- A1（popover 锚点）：`popover.rs` Panel::layout 的 content 尺寸在首次
  layout 时可能为 0/未测量（首帧 bounds 未就绪）→ 首开定位偏左。方向：
  首帧后重定位（on_layout 二次定位）或缓存 anchor bounds 于 visible 翻转时
  刷新。
- A2（thumbnail 空显）：`snapshot.rs` request_capture 队列 + 渲染臂
  fallback icon 兜底已存在——空显发生在懒捕获回调前。方向：兜底态绘制
  skeleton（浅色占位块）替代空白，捕获完成后 ServiceTick 刷新。
- B（wrap_layout_onclick）：为 .at 布局件（row/col/grid/container）统一
  提供 hover 类消费与 oncontextmenu 挂点（当前仅 mouse-area/button 支持）。
  影响全示例回归面（526 待澄清③裁定独立立项）——本计划内先做设计草稿 +
  试点（launcher/桌面），全量铺开另立。
- C：无代码改动；用户反馈驱动。

## 需求分析与背景调查

- 前置：PLAN-526（11 波 39 任务，已归档）——本计划所有条目源自其复审记录
  与 KNOWN-DEBT-AND-RISKS.md 登记（🟡×1、🟢×2）。
- 用户复核项原文见 526 归档文件"复审记录"表与 T20/T24/T28/T31/T32/T37/T38
  各任务条目。

## 详细设计

### A1/A2 实施回填（2026-09-09）

**根因链（探针实测，scratch/p002/popover_debug6.log）**：dock 条目
popover 的 content 子树含 `window_thumbnail`（526 T8 单 popover 双态结构）。
builder 侧恒产 `View::WindowThumbnail`，但 VM 模式渲染前消息桥
`convert_view_messages`（renderer.rs）无该变体臂——落 `_ => Empty` 兜底。
iced 收到的 content=col([Empty])：面板 content 列被 p-1 chrome 撑成 8×8
（子节点 0×0），Top 放置按 content=8×8 定位=锚点右上 8×8 空壳亮点，
整段 hover 期可见；关→开翻转后 content 换型正常（菜单 152×112 首帧即
正位）。→ KNOWN-DEBT 双🟢（首开偏左+懒捕获空显）实为同根两症。

**修复四处**：
1. `renderer.rs convert_view_messages` 补 WindowThumbnail 显式臂（根修）
   + fence 测试（同坑第四例：Grid 319/menubar 422/496 MouseArea）。
2. `popover.rs panel_is_degenerate` 纯函数 + Panel::draw 跳绘（防御纵深：
   任何来源的退化 content 不再闪现空壳）+ 三单测。
3. `desktop.at` blank 菜单 col 归位 popover 标签内（T36 括号错位回归——
   修复过程中经金样对拍证实：master 金样即含散落结构）。
4. `shell.rs resolve_shell_pack_dir` ancestors nth(3) 组目录解析（worktree
   构建静默读主检出 pack——本次实机验证静默失效的根因，独立登记）。

**保留探针**：`AUTO_POPOVER_DEBUG=1` 门控两处（popover.rs Panel::layout
逐帧几何、renderer.rs WindowThumbnail fallback 臂触发），定位复验通道。

### B 实施回填（2026-09-09）

设计稿 = `docs/design/autoui/layout-interaction.md`（问题/设计/语义边界/
分期/后续项）。要点：

- **右键**：`View::Row/Column/Container` 增 `on_right_click`（命名对齐
  Button 402 先例）；aura `set_layout_onclick` → `set_layout_events`（同一
  提取点收 `onclick`/`click` 与 `oncontextmenu`，tracked/untracked 镜像）；
  `convert_view_messages` 显式穿透（D-GAP 第四例教训的护栏测试）。
- **hover**：新增 `crates/auto-lang/src/ui/iced/hover_area.rs`——HoverArea
  纯委托包装 widget（PointerArea/table_resize 先例），自持
  `State{hovered,cursor_position,bounds}`；与样式闭包共享
  `Arc<AtomicBool>` 标志（`layout_hover_flag` 仅在样式声明 `hover:` 变体时
  构造），`layout_style_fn` 按标志在 base 与
  `merged_with_variant(style, Hover)` 两套已构建样式间二选一；翻转只
  `shell.request_redraw()`（不重建 view、不发消息）。
- **包装点**：`wrap_layout_onclick` → `wrap_layout_events(el, onclick,
  on_right_click, hover)`——单个 mouse_area 承载左右两键 + 外层 HoverArea；
  inspect 捕获态自守卫不包（490 G4 同规则）。
- **试点站点**：launcher 网格格（既有 `hover:bg-primary/10` 生效，零 .at
  改动）+ 桌面图标格（`hover:bg-white/10`，`oncontextmenu` 自 icon button
  迁至格 col）。desktop.at 走 auto-os `scripts/shell-pack-sync.py --sync`
  同步 pin + `AUTO_LANG_UPDATE_GOLDEN=1` 金样再生。
- **语义边界**（设计稿 §3）：hover 类作用于布局件自身（不级联子件文本色）；
  布局类 hover 不重排；`View::Grid` 无事件槽（后续项）；`cursor-pointer`
  iced 适配器 no-op（后续项）。

## 测试设计

- A1：popover 首开定位 headless 单测（Simulator：visible 翻转后首帧 bounds
  断言不出锚定边）。
- A2：thumbnail 空显期 skeleton 断言（现有 snapshot 组测试扩展）。
- B：wrap_layout_onclick 试点组件单测 + 既有示例回归（全示例对拍另立）。
- C：实机截图/手势（用户路径）。

## 验收标准

- [x] A1 popover 首开锚点居正（headless 断言 + 实机目检）。
      [✅ 已完成] 2026-09-09。根因非定位几何，而是 A2 同根（见下）：空壳
      面板（content=[Empty]）以 0 尺寸公式定位=视觉偏移。修复①
      convert_view_messages 补 WindowThumbnail 臂（空壳源头消除）②
      Panel 退化内容跳绘护栏 panel_is_degenerate（防御纵深）。证据：
      headless 翻转两帧断言×2（首开=再开+Top/BottomStart 锚居中，layout_
      tests.rs）+ panel_is_degenerate 单测×3；实机 scratch/p002/
      popover_debug6.log——空壳 0 帧（修前 11-15 帧/次），hover 缩略
      200×120 整段在位 panel.x=174=254+(40−200)/2 精确居中，菜单
      152×112 @198 居中，电源面板 232×82.2 @1048 不变。
- [x] A2 thumbnail 空显消除（实机目检）。
      [✅ 已完成] 2026-09-09。真根因=convert_view_messages（VM 模式消息
      桥）缺 WindowThumbnail 臂——缩略节点落 `_ => Empty` 兜底，缩略在
      VM 模式从未渲染过（非 KNOWN-DEBT 猜的快照时序；builder 侧恒产
      thumb、iced 收 Empty，探针时序实锤 scratch/p002）。修复=显式臂 +
      fence 测试 test_convert_view_messages_preserves_window_thumbnail
      （Grid 319/menubar 422/496 MouseArea 后同坑第四例）。实机：
      thumb-fallback 臂复活（wid=1/2 各 21/25 帧），fallback icon 真渲染。
- [x] B wrap_layout_onclick 设计草稿 + 试点落地（或用户裁定再延后并更新
      KNOWN-DEBT 理由）。
      [✅ 已完成] 2026-09-09。原 `wrap_layout_onclick` → `wrap_layout_events`
      收拢布局件三臂（auto-lang os-002-dev `d5b345fb1`，auto-os `fa77bc9`）：
      ① onclick（490 G4 原样）② `oncontextmenu`——`View::Row/Column/Container`
      增 `on_right_click`，aura `set_layout_events` 双键提取（tracked/untracked
      镜像），`convert_view_messages` 显式穿透（D-GAP 护栏测试同坑第五例）
      ③ `hover:` 变体类消费——新增 `hover_area.rs`（HoverArea 自持 hover 态 +
      与样式闭包共享 `Arc<AtomicBool>` 标志，翻转只 request_redraw，零 view
      重建/零消息往返）。试点：launcher 网格格（既有 `hover:bg-primary/10`
      生效）+ 桌面图标格（`hover:bg-white/10` + 右键自 icon button 迁至格 col，
      格内全域可右键）。证据：设计稿 `docs/design/autoui/layout-interaction.md`；
      测试 7 枚（含 iced_test 端到端 flag 驱动、desktop 试点断言 rc=4/hv=4）；
      `cargo t` 全量失败集与 master 全等 24=24（零回归）、iced-layout-tests
      35/35、shell pack hash-lock 绿、a2vue 金样再生后绿；实机
      scratch/p002/p002b_{desktop,launcher}.ps1——桌面格 hover 像素差 6054/
      撤除 0、label 区右键菜单、launcher 网格 7/8 候选点命中 152×95 格
      （未命中的 1 号=选中态格，本就无 hover: 类）。KNOWN-DEBT 526 行结案，
      余项（Grid 事件槽/cursor-pointer/hover 文本级联/全示例 sweep）另立。
- [ ] C 用户复核清单七项逐项确认并勾销。
      [复核进展 2026-09-09] **T20 ✓**（首击=窗口置顶+按钮同时生效，空白区
      只聚焦无误触发；事件实录 scratch/p002/review_ue.log）。**T24 ✓**
      （最大化底缘与任务栏无遮挡，用户确认；附带产出 N4 居中+N5 圆角两
      优化）。**T28 ✓**（三键 hover 盒正方形、紧拢，用户确认）。
      **T31 核心链 ✓**（双击整格 80×80 含 chip/label/空隙全通 + 换 app
      复测通过；右键菜单暴露 N6a/N6b 两缺陷**已转 PLAN-010 专项**
      （popover-overlay-dismiss，含隔离取证与三步 bisect 计划），修复后
      复验勾销）。N1/N2 已修复并经用户确认。余 T32/T37/T38 待续。

## 执行步骤

（每项开工时从本清单转正；完成追加 [✅ 已完成] 证据。）

> **开工登记（2026-09-09）**：worktree 组 `D:/autostack/.wt/os-002/`——
> auto-os（分支 os-002-dev，基点 `6eb654c`）+ auto-lang（分支 os-002-dev，
> 基点 `7613e961f` = 当日 master，含并行会话 595 merge）。正文 auto-lang
> 路径/行号锚点按 AGENTS §2 解析序换算（本次解析 = `D:/autostack/auto-lang`
> 主检出读原文，改动落 auto-lang worktree）。
- [x] A1 popover 首开锚点偏左修复（popover.rs Panel::layout）。
      [✅ 已完成] 2026-09-09，auto-lang `efc7e64b9`。根因在
      convert_view_messages（见 A2），非 popover.rs 定位几何；popover.rs
      侧加 panel_is_degenerate 跳绘护栏（退化帧空壳不可见，纯函数+三
      单测）；headless 翻转两帧断言×2（含 shell.at 同构内容换型场景）。
      修 595 遗留 layout_tests.rs Value::Obj Box 化编译破损（master 存量）。
- [x] A2 thumbnail 空显兜底（snapshot.rs/渲染臂）。
      [✅ 已完成] 2026-09-09，auto-lang `efc7e64b9`。真根因=
      convert_view_messages（VM 模式消息桥，renderer.rs:5875）缺
      WindowThumbnail 臂——缩略落 `_ => Empty` 兜底，VM 模式从未渲染过
      （KNOWN-DEBT 的"快照时序"猜想不成立）。修复=显式臂+fence 测试；
      附带发现并修复 desktop.at T36 括号错位回归（blank 菜单散落桌面
      左下角常驻渲染，auto-os `56cc364` pack+pin+金样再生）与
      resolve_shell_pack_dir 组目录解析 bug（worktree 构建静默读主检出
      pack，ancestors nth(3) 修复）——两项均独立登记 KNOWN-DEBT。
- [x] B wrap_layout_onclick 设计草稿 + launcher/桌面试点。
      [✅ 已完成] 2026-09-09，auto-lang `d5b345fb1` + auto-os `fa77bc9`。
      设计草稿 `docs/design/autoui/layout-interaction.md`（问题/设计/语义
      边界/分期）；实现=`hover_area.rs`（新 widget）+ `wrap_layout_events`
      + `layout_hover_flag/layout_style_fn` + 三节点双路径臂 + aura
      `set_layout_events` + View 字段（含 convert_view_messages 穿透）；
      试点站点=launcher 网格格 + desktop.at 图标格（pack=pin 同步+金样再生）。
      证据见验收标准 B 项。
- [ ] C 用户复核七项逐项销账（T20/T24/T28/T31/T32/T37/T38）。
- [x] D 崩溃：铃铛二次开合通知中心 → 进程静默退出 code 1（复现 2/2：
      2026-09-03 21:38 用户实机 + 22:0x 验收通道 handler 双调；无 panic
      输出、无错误日志——疑似 VM 层 Process.exit 或未打印的 abort，
      需 RUST_BACKTRACE=full 实例复现取栈）。定位：`notes_toggle` →
      toggle_notification_center（renderer.rs:8184）二连击路径。
      ⚠ 归因备注（2026-09-04）：本机存在并行会话的 taskkill /F 强杀
      ui_desktop 的干扰源（强杀退出码恰为 1、无输出——与静默退出同
      signature），上述"复现"不能排除该干扰；复核时需先排除并行
      taskkill（或以 RUST_BACKTRACE=full + 独占环境复现）。
      ✅ 销账（2026-09-06，PLAN-575 T4b）：独占环境 + 退出审计三挂点
      （stdlib.rs exit_audit，零行为变更）下 N=20 轮二次开合归因运行
      20/20 存活零退出、审计零记录——本项负载未复现，按 D3 判据走
      不可复现分支：KNOWN-DEBT 526 行降档 🟡（疑外部击杀，049 同族），
      审计机制常驻，真实复现再启。台账 scratch/p575/ledger.jsonl。
- [ ] E 用户复核：通知中心开合（同 D 场景）确认修复。

## 复审记录

### work 交接记录（2026-09-09 · N3/N4/N5）

stage: work | PLAN-002 | rev 0 | partial（C 复核推进：T20/T24 ✓，N1/N2
已修复用户确认 ✓，N3/N4/N5 交付待验，保持 executing） | code_commit:
auto-lang os-002-dev `60d3efb75`（本仓无改动） | task_ids: N3+N4+N5
evidence: N3=execute_launch_app Ok 臂去通知（toast 逐面重复：412 双层
Stack 每动态视图面一份；失败臂保留）；N4=fit_aware_root 居中包裹（512
待澄清③定案，测量锚点口径不变）；N5=window_radius 四角全圆（T25 降级
撤销）+客户区底色同步+app 根节点默认 rounded-b-2xl（round_bottom_root_default
纯函数+单测，显式 radius 声明优先）；layout_tests 35/35+cargo t 3=3
零回归 | blockers: 无 | next: 用户验 N3/N4/N5 → 续 C3/T28、C4/T31、
C5/T32、C6/T37、C7/T38、E

### work 交接记录（2026-09-10 · N6b 隔离矩阵）

> **2026-09-10 移交注记**：N6a/N6b 已立专项 **PLAN-010**
> （popover-overlay-dismiss，本仓 docs/plans/010-popover-overlay-dismiss.md）
> ——取证结论、隔离矩阵与三步 bisect 计划以该计划"需求分析与背景调查/
> 架构方案"两节为准，下文为其过程记录。

stage: work | PLAN-002 | rev 0 | partial（N6b 隔离取证推进，保持
executing） | code_commit: auto-lang os-002-dev `ee986f0cb`（两隔离例）
| task_ids: N6b
evidence: **隔离矩阵**——A：独立 OS 窗口（iced::application，HostBackend
直跑，无 DM 包装）`ui_popover_probe`：外点自动关闭**正常**（用户手测 +
probe_standalone.log Toggle→Dismissed 循环实录）——popover widget 本体与
iced core overlay::Element::Map 映射（local_shell+merge，源码核对无误）
均健康；C：桌面 surface icon 菜单（daemon+深层嵌套）**坏**（前轮探针
实锤：发布 4 次未达 daemon update + 基础树全聋）；B：裸 daemon+DM 包装
`ui_popover_probe2` **无效测试床**——窗口全黑（裸 daemon 缺桌面主题/字体
接线，渲染不出内容；0×0 client 系未开窗所致已修，留窗即黑屏）。
**结论**：popover 外点关闭机制在健康 runtime 下工作正常；断点在桌面
daemon 宿主环境（多窗口 daemon/主题接线/深层 widget 树三者之一）。
**下轮 bisect 计划**：①把 probe B 补齐桌面同款 `.font(INTER)+.theme()`
接线使内容可见，外点测试即判 daemon 层；②若 B 仍正常 → 在真桌面
以 AUTO_POPOVER_DEBUG 探针对比 dock 菜单（shell.at 层）与 icon 菜单
（surface 层）收窄宿主层；③修复候选：dismiss 消息改走 base 树旁路
（subscription 全局键盘 Esc + 定时器兜底）或修 iced 集成断点。
blockers: 无（bisect 路径明确） | next: 专项回合继续 N6b；N6a 同回合查
evidence: **隔离矩阵**——A：独立 OS 窗口（iced::application，HostBackend
直跑，无 DM 包装）`ui_popover_probe`：外点自动关闭**正常**（用户手测 +
probe_standalone.log Toggle→Dismissed 循环实录）——popover widget 本体与
iced core overlay::Element::Map 映射（local_shell+merge，源码核对无误）
均健康；C：桌面 surface icon 菜单（daemon+深层嵌套）**坏**（前轮探针
实锤：发布 4 次未达 daemon update + 基础树全聋）；B：裸 daemon+DM 包装
`ui_popover_probe2` **无效测试床**——窗口全黑（裸 daemon 缺桌面主题/字体
接线，渲染不出内容；0×0 client 系未开窗所致已修，留窗即黑屏）。
**结论**：popover 外点关闭机制在健康 runtime 下工作正常；断点在桌面
daemon 宿主环境（多窗口 daemon/主题接线/深层 widget 树三者之一）。
**下轮 bisect 计划**：①把 probe B 补齐桌面同款 `.font(INTER)+.theme()`
接线使内容可见，外点测试即判 daemon 层；②若 B 仍正常 → 在真桌面
以 AUTO_POPOVER_DEBUG 探针对比 dock 菜单（shell.at 层）与 icon 菜单
（surface 层）收窄宿主层；③修复候选：dismiss 消息改走 base 树旁路
（subscription 全局键盘 Esc + 定时器兜底）或修 iced 集成断点。
blockers: 无（bisect 路径明确） | next: 专项回合继续 N6b；N6a 同回合查

### work 交接记录（2026-09-09 · N6 取证）

stage: work | PLAN-002 | rev 0 | partial（C 复核推进：T20/T24/T28 ✓，
T31 核心链 ✓ 但右键菜单 N6a/N6b 两缺陷，保持 executing） |
code_commit: auto-lang os-002-dev `fd2b85ea3`（取证插桩，无行为变更） |
task_ids: N6a+N6b（C4/T31 右键菜单复核发现）
evidence: 复现脚本 scratch/p002/n6b_repro.ps1 + n6b_repro.log；探针四挂点
（AUTO_POPOVER_DEBUG=1）实录——①DSL ondismiss 接线正确（aura widget 锚臂
extract ondismiss→View::Popover.on_dismiss=Some ✓）②overlay 注册正常
（每帧 pv-overlay ✓）③外点/ESC 时 Panel::update 收到事件且判定正确
（over_panel=false→dismiss 分支执行，pv-dismiss=4 次发布 ✓）④**消息未达
daemon update**（pv-update/pv-window 对 MenuClose=0，对 IconMenu=4——
DM::App 臂在位且工作）⑤菜单开启期间基础树全聋（外点连 BlankPress 都不
达；面板内点击正常=MenuOpen 到达 ✓）。断点定位：iced 0.14 overlay→
runtime 消息管线（非 DSL/非 popover widget 判定层） | blockers: N6b 需
iced runtime 专项定位（假设清单：overlay 消息 vec 与 base 分离/多窗口
UserInterface 路由/0.14 升级回归） | next: N6b 专项回合；N6a 样式盒
根因待一查（build_button_style/visual wrap 的 border 均 width=0 不可见，
截图方框来源未定）；续 C5/C6/C7/E

### 复核结论登记（2026-09-09 · 第二轮）

- **N1 ✓**：chip 全域双击启动，用户确认（方案 A 落地生效）。
- **N2 ✓**：开窗闪变消除，用户确认（fit 测量期越界隐藏生效）。
- **C2/T24 ✓**：最大化底缘与任务栏无遮挡，用户确认。
- **N3（已修复待验）**：启动成功通知去除（桌面+app 窗双份 Success 均
  消失；失败反馈保留）。顺带登记：toast 层逐视图面渲染的重复缺陷仍在
  （任何 notify 都会在 shell+每个 app 窗各弹一份）——用户裁定启动场景
  去通知后暂无消费面，登记 KNOWN-DEBT 候选，首个真实 notify 需求出现
  时系统化收口（单点渲染+跨窗去重）。
- **N4（已修复待验）**：最大化/放大 fit 窗内容居中（原左上角沉底，
  512 待澄清③由用户裁定关闭：居中语义定案）。
- **N5（已修复待验）**：窗框底部圆角系统化——①窗框四角全圆（撤销
  526 T25 底边方角降级；最大化仍方角=全屏惯例）②客户区底色同步底角
  ③vwin 托管 app 根节点默认 rounded-b-2xl（16px 与窗框对齐；应用显式
  radius 声明优先不覆盖；独立模式 OS 窗走系统原生圆角不涉及）。

### work 交接记录（2026-09-09 · N1/N2）

stage: work | PLAN-002 | rev 0 | partial（N1/N2 交付，C 复核进行中 T20 ✓，
E 待验，保持 executing） | code_commit: auto-lang os-002-dev `5119f1ab3`，
auto-os os-002-dev `844b7c8` | task_ids: N1（chip 双击死区，方案 A 定案）+
N2（fit 窗开窗闪变）
evidence: N1=desktop.at chip 降级 col+icon（pack sync 4e5bb4f1a9+金样再生
10/6 行 churn）+试点断言 rc=4/hv=4 绿；N2=vwin_fit_hidden 纯函数+单测
（越界 100k px 绘制偏移，布局原位锚点可测、Stack 零位移；ServiceTick
计数 5 tick 强制显形护栏）+layout_tests 35/35+a2vue 15/15；cargo t 失败
集与 master 全等 3=3 零回归；实机待用户复验（N1 双击/右键整格、N2 计算器
开窗一次成型） | blockers: 无（等用户复验） | next: 用户续验 C2-C7/E

### 阶段复审记录（2026-09-09 · A1/A2/B）

stage: review（阶段性，仅 A1/A2/B） | plan_id: PLAN-002 | plan_revision: 0
（frontmatter 无 plan_revision 字段，按交接记录记 rev 0） | outcome: **pass
（A1/A2/B 三项全 pass；C/E 未完，整体保持 executing，不授予终态 reviewed）**
| reviewed_commit: auto-os os-002-dev `fa77bc9`；auto-lang os-002-dev
`d5b345fb1`（A1/A2=`efc7e64b9`，debt 回写=`518479259`） | base_commit:
auto-os `6eb654c`，auto-lang `7613e961f` | dependency_revisions: master
auto-lang `18ab5d642`（回归对照基线，脏区仅 website/docs markdown） |
spec_inputs: auto-lang `docs/specs/autoui-skill/project.md`（22 行 stub，无
交互原语条目，无冲突）；`.autoos/specs.json` 本审未动 | worktree 双仓
`git status` 干净（实现全部已提交）。

**acceptance_results**（复审独立复跑，非沿用 work 证据）：

- **A1 pass**：layout_tests 35/35（`cargo test -p auto-lang --features
  ui-iced,iced-layout-tests --lib layout_tests`，含首开翻转两帧断言
  popover_first_open_after_content_swap/flip_matches_second_open）；
  panel_is_degenerate 三单测绿；实机探针日志复核：popover-debug 127 帧
  中 content=8×8 空壳 **0 帧**，电源面板 232×82.2 @x=1048 与记录一致。
- **A2 pass**：fence 测试
  test_convert_view_messages_preserves_window_thumbnail 绿；thumb-fallback
  探针臂 46 次触发（实机日志）；desktop.at T36 归位后 a2vue 金样
  test_a2vue_desktop_surface_asset 绿；pack==pin 逐字节一致（shell/
  desktop.at vs assets/desktop.at diff 空）；hash-lock parity 测试绿。
- **B pass**：7 枚测试全绿（hover_area::transition_fires_only_on_change /
  test_layout_oncontextmenu_becomes_right_click /
  test_convert_view_messages_preserves_layout_right_click /
  test_layout_hover_flag_only_with_hover_variant /
  test_layout_style_fn_selects_hover_on_flag /
  layout_hover_flag_tracks_cursor_over（端到端）/
  desktop_surface_at_loads_interactions_and_dispatch 试点断言 rc=4 hv=4）；
  实机像素差自留档 PNG 独立重算：桌面格 hover=**6054**/撤除=**0**（与交接
  记录一致），launcher l_hover_2..8 全命中（12364–14551）、1 号=选中态格
  =0（7/8 命中复现）；p002b_desktop.log 格 col IconMenu 事件链在案。
- **回归门 pass**：全量 `cargo t` 分支 1189 pass/3 fail vs master `18ab5d642`
  1192 pass/3 fail——失败集**逐名全等**（plan370_015_behavior_tests::
  d2_new_note_appends / d8_toggle_dark_mode / z6_export_prolog_alignment，
  存量）。交接记录"24=24"系对开工日基点 master 的对照；本次复审以当日
  master 重对照得 3=3，结论不变（零回归）。

**findings**（均不阻塞本阶段结论）：

- **F-1（P2，终审前必清）**：计划缺 `### 规范增量` 节，frontmatter 三 delta
  字段空且无书面说明。终审（C/E 完成后）必须补：现有 canonical specs 无
  UI 交互原语条目，B 的持久语义当前承载于设计稿
  `docs/design/autoui/layout-interaction.md`——终审时要么登记新 spec
  组件，要么写明空 impact 理由。
- **F-2（P3）**：待澄清 #1（A2 skeleton vs 降级 icon 视觉形态）已因根因
  修复失效（缩略直接渲染，无兜底形态需求），应标记"已失效"防空决策债。
- **F-3（P3）**：`518479259` 在 KNOWN-DEBT 🟢 表格中段引入空行，Markdown
  渲染表格被截断为两截，下个 docs 批顺手修复。
- **F-4（P3）**：两处 AUTO_POPOVER_DEBUG 探针代码注释写"定案后移除"，与
  详细设计"保留探针"决议不一致，措辞对齐（或清理批移除）。

**evidence**（worktree 清理后可复现命令均已内录上文；主检出留档
`D:/autostack/auto-lang/scratch/p002_review_master_fails.txt`=master 失败
集；分支失败集与像素差重算结果已全文内录本记录，scratch/p002 为
auto-lang worktree 临时证据，merge 清理后以上文内录为准）。

next: work 继续 C/E（用户实机复核七项清单 + 通知中心开合确认）；F-1 于
终审前清偿；整体计划保持 `executing`。

### C/E 复核进展 + 新发现登记（2026-09-09 · 第一轮）

**T20 ✓**（证据见验收标准 C 项进展注）。

- **N1（复核发现，阻塞 T31 收口）：桌面图标 chip（圆角徽标块）上双击无效**。
  根因已定位——chip 是 iced `Button`（desktop.at popover 锚内
  `button (icon: e.icon)`，无 onclick 无 on_right_click 纯视觉），在
  `mouse-area (ondblclick)` **内侧**：iced 命中测试最内层优先，Button 捕获
  press/release，祖先 MouseArea 的 on_double_click（496 M5）收不到按压——
  chip 上双击只发 VM 裸 `click` 事件（review_ue.log 实录 8 条
  `widget="" event="click"`，无 ActivateApp）；格内 label/padding 区双击
  正常（ActivateApp 实录 ✓）。**存量缺陷**（T30 起结构即如此，非 B 引入；
  526 T31 验证时用户双击落在 label 区未暴露）。修复方向待用户定夺双击
  目标语义：**A（推荐）**=chip 降级非交互视觉件（image/col），整格
  80×80 含 chip+label 统一双击/右键，hover 高亮框=可点边界（Windows
  惯例，目标最大）；**B**=仅 chip 可双击（用户第一直觉，目标缩至 40×40
  且 label 区失效，需 Button 双击原语或 chip 包 mouse-area + 撤格级
  ondblclick）。定夺后转 work 实施。
- **N2（复核发现，滚动登记）：app 窗打开瞬间偏大再缩至实际尺寸**。
  机制=Plan 504 fit 窗（pac.at `window: "fit"`）先以初始尺寸开窗，
  `__fit_measured` 测量回填后才缩到内容尺寸——首帧到测量帧间隙可见闪变。
  修复方向（work 阶段设计）：fit 窗首帧隐藏/透明直至测量落定，或持久化
  上次实测尺寸作初始值。独立于 C 清单，转 work 排期。

### work 交接记录（2026-09-09 · B）

stage: work | PLAN-002 | rev 0 | partial（B 交付，C/E 未完，保持
executing） | code_commit: auto-lang os-002-dev `d5b345fb1`，auto-os
os-002-dev `fa77bc9` | task_ids: B（+设计稿）
evidence: 设计稿 `docs/design/autoui/layout-interaction.md`；测试 7 枚
（hover_area 转移纯函数/标志门控/样式二选一/消息桥穿透 fence/aura
oncontextmenu 提取/iced_test 端到端 flag 驱动/desktop 试点 rc=4 hv=4）；
`cargo t` 全量失败集与 master 全等（24=24，零回归，三轮复跑仅
clipboard_native 一次 flake 单测隔离绿）+ iced-layout-tests 35/35 +
shell pack hash-lock 绿 + a2vue 金样再生绿 + desktop_protocol 仅基线红；
实机 scratch/p002/p002b_{desktop,launcher}.ps1（桌面格 hover 6054/撤除 0、
label 右键菜单、launcher 7/8 命中 152×95 格） | blockers: C/E 需用户实机
复核 | next: C/E 用户复核，或径入 review（A1/A2/B 可先行复审）

### work 交接记录（2026-09-09）

stage: work | PLAN-002 | rev 0 | partial（A1/A2 交付，B/C/E 未完，保持
executing） | code_commit: auto-lang os-002-dev `efc7e64b9`+`518479259`，
auto-os os-002-dev `56cc364` | task_ids: A1/A2+附带三修复 |
evidence: scratch/p002（探针日志 popover_debug6.log+截图 boot6/tb_first/
first5 等）；headless popover 套件 26/26+p7_loader 2/2+a2vue 金样/
desktop_surface 绿；desktop 套件 3 红与 master 集合全等（存量+flake，主
检出复现在案） | blockers: B 需设计草稿试点时段；C/E 需用户实机复核 |
next: B（wrap_layout_onclick 草稿+试点）或径入 review（A1/A2 可先行复审）

**随迁发现登记**：①convert_view_messages D-GAP 尾差 8 变体（Accordion/
NavigationRail/Overlay/Select/Sidebar/Slider/Tabs 等，KNOWN-DEBT 🟡候选）
②desktop.at T36 括号错位回归（已修，金样再生）③resolve_shell_pack_dir
worktree 解析失效（已修）——三项均入 auto-lang KNOWN-DEBT-AND-RISKS.md。

## 待澄清事项

1. A2 空显兜底的视觉形态（skeleton 块 vs 首帧降级 icon）需用户定夺。
2. ~~B 的铺开范围（仅桌面 vs 全示例）影响回归面，试点后定。~~ **已裁定
   （2026-09-09，B 交付）**：机制全量生效（渲染层全局），试点面 = launcher
   + 桌面；全示例 sweep（481+88 处 `hover:` 的布局件逐例目检 + 金样对拍）
   与余项（`View::Grid` 事件槽 / `cursor-pointer` 接线 / hover 文本色级联）
   另立计划——见 `docs/design/autoui/layout-interaction.md` §5/§6。
