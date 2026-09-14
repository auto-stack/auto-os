---
plan_id: PLAN-012
plan_revision: 2              # rev2 = 追加问题7（图标居中 + 任务栏状态指示，2026-09-11）
status: archived             # 2026-09-14 merge 落地三仓（auto-os 6e25d1b、auto-lang master 5c444818f、auto-os-config a60c6bc），review pass 全 AC 收口
feature_name: shell-ux-feedback-batch
author: [zhaopuming]
created_at: 2026-09-11
updated_at: 2026-09-12

# /auto-plan:review 结束时填写：
supersedes_spec_components:
  - "G2 通知面板锚定语义：右下锚定 → 右上锚定（用户裁定 2026-09-14；紧凑卡片 + 顶/右 12px gap；SD-04 随复审更新）"
  - "dock_pinned 缺省三枚 → 缺省空（SD-03；缺键=显式空=空表）"
new_spec_components:
  - "auto-lang schema/projection-protocol-v1.md v1.6+复验增补（§2 投影面 pinned/dup_app/running/__wm_settings_open/__wm_layout、§2.0.1 通知面板接缝 __panel_max_h、§4 动词、§6 变更记录）"
  - "auto-lang schema/aura.at：workspace_preview 合同面登记（SD-02，SCHEMA_DRIFT_GENERATE 再生入库）"
  - "通知面板关闭模型：scrim 外点关闭（.at 内 N6d）+ ×/Esc/铃铛 toggle（SD-04）"
  - "任务栏状态指示：左 app 图标三态 + 布局钮/右开关钮两态（SD-06；高亮机制统一 → DEBTS 012-候选 icon 族）"
touched_goals: []             # 引用 docs/specs/goals.md 的 GOAL-NNN

affects: [auto-lang/ui/session.rs, auto-lang/ui/iced/renderer.rs, auto-lang/ui/desktop_config.rs,
          auto-lang/schema/projection-protocol-v1.md, shell/shell.at, shell/desktop.at,
          shell/notification_center.at, auto-os-config/auto/src/front]
current_step: 14
total_steps: 14
---

# [PLAN-012] shell-ux-feedback-batch

## 0. 变更摘要

用户实机体验反馈批（2026-09-11，rev1 六项 + rev2 追加一项，截图四张）：

| # | 问题 | 根因调查结论（详见 §4） | 工作面 |
|---|------|------------------------|--------|
| 1 | ⚙️ 打开 os-config 每次卡顿 ~1s | `execute_open_settings` launch 臂在 **UI 线程同步**跑：daemon 阻塞探活（`tcp_ping` 2s 超时/未运行 spawn+轮询 ≤5s）+ back cdylib 装载 + ~4k 行前端 .at 全量编译；× 关窗后每次重开全链重跑 | auto-lang 宿主 |
| 2 | 通知面板贴顶无 gap；无关闭 icon/外点关闭/自动关闭 | 面板顶部 `spacer flex-1` 在 iced 轨塌缩（截图实证）；关闭Affordance三缺；toast 已有 TTL 自动过期、面板为常驻历史面 | shell pack + 宿主微改 |
| 3 | 虚拟桌面面板无外点关闭；缩略是 app 小截图集合非整桌面预览 | 面板是 shell col 内联 `if switcher_open` 行（非 popover 无 ondismiss）；`window_thumbnail` 是逐窗快照，DSL 无重叠布局画不出整桌面 | shell pack + auto-lang 渲染器（新 widget） |
| 4 | dock 默认 calc/todo/notes 应去默认、与运行窗条目合并、增固定/取消固定 | `__wm_wins` 投影不排除 pinned 应用→重复显示；`set_dock_pinned` 动词已有（540 T3）；缺省三枚硬编码于 `DEFAULT_DOCK_PINNED`；代码无分隔符元素（两段 for 循环视觉分组） | shell pack + auto-lang 宿主 |
| 5 | 桌面图标间距过大、不可拖拽/编辑 | `grid (cols: 8)` 均分整行宽→格距被拉伸；DSL v1 无拖拽/无图标位置存储 | shell pack + auto-lang 宿主（含有界调查） |
| 6 | Launcher 列表显示 launcher 自身 | `summon_launcher` 全量注入 `registry_entries`，无排除 | auto-lang 宿主（一行过滤） |
| 7 | 桌面/任务栏图标字形偏左上不居中；任务栏图标缺"开/关/聚焦"状态指示 | 字形渲染共享 lucide 出口但**两处出口**容器约束不一致（svgdoc 路径约束外层 container、lucide 路径裸返回）；状态面：运行点恒主色无聚焦区分、聚焦竖条被感知为"分隔符"、右侧开关型图标无打开态高亮 | auto-lang 渲染器（居中）+ shell pack（状态样式）+ 宿主（`__wm_notes_visible` 投影） |

跨仓计划：主导仓 = 本仓（auto-os，桌面程序归属），改动落 **auto-os `shell/`**
pack、**auto-lang `crates/`**（宿主/渲染器）、**auto-os-config**（仅核对，预判
零改动）。shell pack 改动后跑 `scripts/shell-pack-sync.py` 对齐 auto-lang
内嵌快照（hash-lock 契约）。

## 1. 目标

1. **G1** ⚙️ 打开 os-config 不再冻结桌面：二次打开感知即时（<100ms），
   首次打开桌面无 ≥300ms 的输入无响应窗口。
2. **G2** 通知中心面板右下锚定（不贴顶、与右/下留对称 gap），具备 × 关闭、
   外点关闭、Esc（已有）三种关闭路径；自动关闭策略定型并文档化（macOS 对标）。
3. **G3** 虚拟桌面面板支持外点/Esc 关闭；每分区卡片呈现**整桌面等比小预览**
   （壁纸底 + 逐窗快照按布局矩形合成）。
4. **G4** dock 单一组：默认无固定项；运行中应用条目不与固定项重复；任意
   dock 条目可右键 固定/取消固定（持久化）；两段列表间的视觉分隔随之消失。
5. **G5** 桌面图标紧凑排布（贴左上、格距≈格尺寸）；拖拽换位可用（有界调查
   后按决策实施；重命名编辑 v1 非目标）。
6. **G6** launcher palette/grid 不再列出 launcher 自身。
7. **G7** 图标字形在格/钮内居中（桌面 chip 与任务栏按钮双面）；任务栏状态
   指示定型：左侧 app 图标三态（关闭无条 / 打开灰条 / 聚焦主色条+底色
   高亮，Windows 11 形态），右侧开关型图标两态（面板/窗打开 = 底色常驻
   高亮，关闭 = 默认）。

非目标（v1）：通知横幅（toast）样式重做；per-window 最小化进 pinned 图标
菜单（延后）；桌面图标重命名；os-config 插件配置内容本身的加载速度优化；
switcher 面板多显示器适配。

## 2. 架构方案

按六个工作包（W1..W6 ↔ 问题 1..6）组织，三条改动走廊：

- **shell pack（本仓 `shell/*.at`）**：W2 面板锚定+关闭三件套、W3 popover 化
  + preview widget 消费、W4 dock 合并+菜单项、W5 图标网格紧凑化+拖拽态。
  改后必跑 `python scripts/shell-pack-sync.py`（hash-lock 四件全等门）。
- **auto-lang 宿主/渲染器（`crates/auto-lang/src/ui/`）**：W1 异步/常驻
  launch 链、W3 `workspace_preview` widget、W4 `__dock_pinned_csv` 注入 +
  `dock_pin/dock_unpin` 动词（协议 v1.6）+ 缺省 pinned 置空、W6 launcher
  注入过滤。crates/ 有改动 → 按 auto-lang AGENTS Category A 门档跑
  `cargo t`（iced 档 + vm 档）。
- **auto-os-config**：核对模块 store 的 Init 拉取已是 fire-and-forget 异步
  （`modules_store` 注释在案）+ loading 分支已存在（app.at `if .store.loading`）
  ——预判零改动；若核对发现同步阻塞点，回填本计划。

W1 采用**常驻设置窗**（hide-on-close）为主方案 + daemon 探活超时收紧为辅：
× 关闭 os-config 窗时宿主拦截为隐藏（进程内组件与编译产物保留），重开 =
纯聚焦臂（既有 `execute_open_settings` focus 路径，零编译）。理由：比
"骨架窗+后台编译"改动面小一个量级，且完全消除"每次打开"的卡顿（用户
原话的痛点即重复打开）；首次打开的后台化作为 W1-T2 探明后的可选增项。

W3 整桌面预览采用**宿主合成 widget**（`workspace_preview`）：v1 复用
snapshot.rs 逐窗快照缓存（SWR），按 `VWinState.rect` 缩放贴合分区卡片
宽高比绘制；miss 窗 = 占位底 + fallback icon，并经 `request_capture`
排入预抓。不做实时视频流（headless 栅格化 497 T1 已裁定不可行）。

## 3. 技术栈

- auto-os shell pack：特权 .at（AutoUI DSL，iced 轨 + Vue 轨双端一致纪律）
- auto-lang：Rust / iced（renderer）、AutoUI VM（session）
- 验证：auto-lang `cargo t`（crates/ 有改动档）；实机桌面（vm 轨）手测 +
  autoui-verifier 技能双端核验；本仓 playwright 套件不涉（shell 面不在
  app playwright 覆盖内）

## 4. 需求分析与背景调查

### 4.1 授权与范围

- 授权来源：用户 2026-09-11 实机反馈原文（rev1 六项 + rev2 追加 #7；
  本计划 §0 表格即原文归纳）+ 截图四张（通知面板贴顶、本机任务栏/桌面、
  Win11 任务栏底条、Win11 快速设置 flyout 高亮）。
- 允许仓/动作：auto-os（shell pack + docs）、auto-lang（crates/ + assets
  同步）、auto-os-config（只读核对）。worktree 红线遵守 AGENTS §2
  （`bash D:/autostack/wt-guard.sh` 先行）。
- 预算：未设自动续跑上限；W5-T1 为有界调查（产出决策工件，不直接实施）。

### 4.2 逐项根因调查（代码证据）

**#1 os-config 卡顿**
- 链路：shell.at ⚙️ → `open_settings` → renderer `execute_open_settings`
  （renderer.rs:8896）→ launch-or-focus：已有窗则跨分区聚焦（**快**），
  无窗则 `execute_launch_app("os-config")`。
- launch 臂（session.rs:2281 `launch_app`，inproc）同步执行于 iced update
  （UI 线程）：
  1. `ensure_daemon_if_declared`（session.rs:2368）→ `ensure_ready`
     （osconfig_daemon.rs:299）→ `tcp_ping`（**connect_timeout 2s**，
     :182）；daemon 未运行 = spawn + 200ms 间隔轮询 ≤5s。
  2. `load_back_cdylib`（pac `back: { project }`，磁盘 IO + dlopen）。
  3. `build_dynamic_component`：os-config front ~3.9k 行 .at 全量编译。
- 用户感知"每次打开都卡" = 每次 × 关窗后重开都重跑全链（窗口不在
  `host.wm.wins` 即走 launch 臂）。
- os-config 侧模块加载已有异步形态：`app.at:64` 注释"独立 fetch 计数
  （不经 Modules store，免 fire-and-forget Init 的时序竞态）"、`:80`
  `if .store.loading` loading 分支在案——**窗口出来后的模块装配已是非阻塞
  形态**，瓶颈在窗口出来之前。

**#2 通知中心面板**
- 锚定：notification_center.at:44-105 意图 = 右下锚定（顶部 `spacer
  flex-1` 压下、右侧 `spacer w-3`）。实机截图（用户提供）面板贴顶 →
  顶部 spacer 塌缩。静态线索：renderer `axis_fix_col_child`
  （renderer.rs:1112）应把 col 直接子级的 flex-1 转写 Height(Full)，
  理论上成立——与实机矛盾，**需实机最小诊断**（W2-T1）再定修复机制；
  候选 = Plan-050 已验证的 `mt-auto` 填充条路径（renderer.rs:1402：
  Column 发射时在 MarginTop(Auto) 子项前插 Fill 高占位条）。
- 关闭 affordance：仅 Esc（bind :116）+ 铃铛二态。无 ×、无外点关闭。
- 自动关闭现状：**toast 已有 TTL 自动过期**（renderer.rs:12217
  `toasts.retain`）；通知历史面板为常驻面。macOS 模型（用户问询）：
  横幅通知 ~5s 自动消失（hover 暂停计时）、alert 级驻留待处理；
  通知中心（历史面板）**从不自动关**——关闭路径 = 点外部/Esc/再点菜单栏
  图标。本仓 toast≈banner（已自动消失）、面板≈通知中心 → 对标结论：
  **面板不做定时自动关**，补齐外点 + × 两条人工路径即可（§5 W2）。
- 外点关闭与宿主一致性：宿主 `notification_visible()` 直读组件 visible
  状态（renderer.rs:8586 等）→ 面板自隐（visible="0"）零宿主改动，
  launcher PLAN-010 N6d 同构先例。

**#3 虚拟桌面切换器**
- 无外点关闭根因：面板是 shell.at:317 内联 `if .switcher_open == "1"` 行，
  非 popover（无 ondismiss）；非面板区域的点击落在 spacer/桌面，无人消费。
  宿主热键路径：置位 `switcher_open` + ServiceTick ~1.6s 自动收起
  （shell.at:103 注）——外点关闭不得破坏该路径。
- 缩略形态：卡片内 `for w in __wm_wins if w.workspace == ws.id && w.pager
  == "1"` 渲染逐窗 `window_thumbnail`（w-16 h-10 横排）——非桌面全貌。
  DSL 无重叠布局（desktop.at:13 注"z 序宿主侧兑现"），**.at 侧画不出
  整桌面合成**，必须宿主新 widget。
- 快照基建：snapshot.rs 逐窗快照（整窗 screenshot + rect 裁剪 + 降采样
  ≤256 + TTL 2s + SWR 续帧）。非当前分区的窗不可见，快照 = 最近一次
  可见时的 SWR 旧帧（可接受；miss = 占位）。每窗 `VWinState.rect`
  （宿主逻辑坐标）可从 wm 直取——合成几何数据齐备。

**#4 dock 固定项**
- 重复显示根因：投影 `__wm_wins` 遍历全部 `z_order`（renderer.rs:10950）
  不排除 `registry_id ∈ dock_pinned` 的窗 → calc 运行时固定图标（带
  运行点）与窗口条目并存。
- 固定/取消固定：**`set_dock_pinned\t<csv>` 动词已有**（session.rs:1266/
  1563、renderer.rs:9315 `execute_set_dock_pinned` 落 config.at）。但
  shell 侧无法用它做"增删一枚"：`__dock_pinned` 是宿主注入 Obj 数组，
  **handler 字段读失效（B12 同族）**——shell 拿不到当前 csv 全集做拼接。
  → 新增 `dock_pin\t<id>` / `dock_unpin\t<id>` 两窄动词（宿主侧 Vec
  增删 + save + 重注入），csv 手术不过 .at。
- 缺省三枚：`DEFAULT_DOCK_PINNED = ["011-calculator","013-todo","015-notes"]`
  （desktop_config.rs:16,53,73），且"空表回退缺省三枚"（:52 注）——
  置空缺省需同时修"显式空被回退吞掉"语义（缺键 = 空；显式空 = 空）。
- 桌面图标联动：`__desktop_icons` = `dock_pinned ∪ shell.desktop.icons`
  去重 − hidden（renderer.rs:10663 inject_desktop_surface）→ 缺省置空后
  新用户桌面同时为空（可经固定/后续"发送到桌面"重建），存量 config.at
  不受影响。
- 分隔符：shell pack 四件 grep 无 divider/separator 元素——"分隔符"是
  两段 for 循环（固定组=运行点样式 vs 窗口组=激活竖条样式）的视觉分组
  感知；合并后自然消失，实机复核确认即可。

**#5 桌面图标**
- 间距根因：desktop.at:82 `grid (cols: 8, gap: 8)` + `w-full`——8 列均分
  整行宽，宽屏上格间距被拉到数百 px（格本体仅 w-20 h-20）。
- 拖拽/编辑：DSL mouse-area 无拖拽事件；全局 `__mouse_moved` 坐标流已有
  消费先例（desktop.at `__desktop_cursor_x/y`，PLAN-612）；PLAN-530
  absolute 子层已落地（column_layer_partition + build_floating_layer）→
  自由定位图标（x,y 存储 + absolute 渲染 + 坐标流拖拽）基建三件齐备但
  未组装；图标位置无任何存储键。交互设计（自由拖 vs 格子换位、按下即拖
  vs 长按阈值）**有界调查后定**（W5-T1 决策工件）。

**#6 launcher 自列**
- `summon_launcher`（renderer.rs:8660-8676）把 `registry_entries` 全量
  注入 `apps_*` 平行列表，无排除。launcher 入册规则 = id `"launcher"` 或
  `"-launcher"` 结尾（session.rs:284 注，441 预订 028-launcher）——
  注入处按同规则过滤即可，**不影响** `launcher_entry` 发现链。

**#7 图标居中 + 任务栏状态指示（2026-09-11 rev2 追加）**

- **组件关系（用户问询"是否统一组件"）**：字形渲染器共享、包装层各异。
  两面的字形最终都走 `lucide:` 前缀的 **`AbstractView::Image` 出口**
  （renderer.rs:4864-4940：svg 画时着色 + `container(svg_widget)` 返回）；
  但进入该出口的包装不同——任务栏 = `button (icon:)` prop 的按钮内嵌
  路径（renderer.rs:3448 起，icon px 随按钮字号，svg 直挂按钮内容），
  桌面 chip = desktop.at:101-106 手工 col 底块 + `icon (name:)`（
  aura_view_builder.rs:6190 `convert_image_or_icon` → `View::Image`）。
  另有第三处 lucide 出口（svgdoc 内联路径 renderer.rs:4997 注）——
  **svgdoc 路径显式约束外层 container 尺寸（"Svg size_hint 默认 Fill →
  container 撑满行内剩余宽"），lucide 路径（:4937）裸返回未约束
  container**——同族不一致是"字形偏左上"的头号嫌疑（container 被撑大后
  字形按 Contain 落位偏移）。像素级定案需实机探针（W7-T1），修复点=
  共享出口（单点修复两面同愈）。
- **状态指示现状**：
  - 左侧：pinned 图标运行点 `h-1 w-6 bg-primary` 恒主色（shell.at:149，
    无聚焦/非聚焦区分）；窗口条目聚焦 = 条目**左侧**竖条
    `w-0.5 h-5 bg-primary`（shell.at:187-189）——实机截图中被感知为
    "分隔符"（与 #4 的分隔符反馈同源）；聚焦无底色高亮。
  - 右侧：开关型图标（切换器/铃铛/齿轮/电源）无打开态——面板开着时
    图标外观与关闭态全同。
  - 状态数据可达性：左三态 shell 可自算（`__wm_wins` 有
    `focused`/`app`，`__wm_running` 有运行集）；切换器/电源 = shell
    本地态（`switcher_open`/`shutdown_ask`）；齿轮 = `__wm_wins` 存在
    `app=="os-config"` 条目即开（W1 落地后隐藏窗被投影排除，语义正好
    = "关闭"）；**铃铛缺口**：通知面板 visible 在宿主 overlay 组件上，
    shell 不可见 → 需投影新字段 `__wm_notes_visible`（"1"/""，
    `__wm_notes_unread` 同型，随协议 v1.6）。
- **跨平台对标（用户问询）**：
  - **Windows 11**（用户截图 2/3）：左侧 app 图标底条三态——灰条=
    打开非聚焦、主色条=聚焦、无条=关闭（仅固定图标有此态）；聚焦附加
    底色高亮（与 hover 同款）；右侧 flyout（快速设置）打开 = 图标底色
    常驻高亮。信息密度最高。
  - **macOS**：Dock 运行 app 图标下方小点/短横线，**不区分前台后台**
    （聚焦信息由菜单栏 app 名承担）；Dock 无聚焦底色高亮；两态而非
    三态；"固定未运行"与普通未运行无视觉差。
  - **Android / Material 3**：手机无持久任务栏；大屏 shelf 运行 app =
    图标下短横线（macOS 同型两态）；Material 3 导航组件的选中态 =
    药丸指示条 + tonal 底色，语义是"选中/未选中"而非"打开/聚焦/关闭"
    三态；未读走徽标（notification dot）。
  - **裁定**：采纳用户提议的 **Windows 11 方案**——三态信息最全、
    与本 shell 既有词汇（运行点/主色 accent/hover 底）同构，改造成本
    最低；macOS 方案丢聚焦信息、Material 方案无"固定未运行"表达，
    均不满足 #7 需求。右侧开关型用底色常驻高亮（与 hover 样式一致，
    用户截图 3 同款）。

### 4.3 相邻在途计划

- PLAN-002（desktop-ux-followups，executing）：滚动跟踪计划，本批七项
  独立成册不入其清单；W2 锚定修复若踩到其 A1（popover 首开偏左）同族
  基建，两计划互链。
- PLAN-010（popover overlay dismiss，已归档）：N6d 外点关闭模式 = 本计划
  W2/W3 复用的既定先例。
- PLAN-612/613（shell 修复，已入 master）：坐标锚 popover、空态居中——
  本计划 shell pack 改动的风格基线。

## 5. 详细设计

### W1 os-config 打开不卡顿（G1）

1. **常驻设置窗（主方案）**：宿主 close 臂拦截 `registry_id ==
   OSCONFIG_APP_ID` 的 `close` 动词（dock 右键关闭/窗 × 均经
   `DesktopCommand::CloseWindow`）→ 改隐藏（现有窗管理无 hide 态则
   v1 退化为"最小化到分区外"？——**实施时二选一**：
   a. wm 增 `hidden: bool` 窗字段（投影排除 `__wm_wins`/布局排除，
      focus 臂先 unhide——约 30 行）；
   b. 拦截为 `win_min`（既有最小化）——零新概念，但任务栏仍有条目。
   决策倾向 a（真"关了"的感知 + 重开瞬时）；T1 实施时按 wm 现状定）。
   `execute_open_settings` 无窗分支因此几乎不再触发（仅 boot 后首开）。
2. **daemon 探活收紧（辅）**：`ensure_ready` 首 ping 拆短超时（localhost
   connect 250ms 足够；未运行分支的 spawn+轮询维持现状——仅首开路径）。
3. **首次打开后台化（可选增项，T2 探明后定）**：骨架窗（复用
   `build_launch_fallback` 形态改 loading 文案）+ 重链移 Task——若 T1
   实测首开已 <300ms 则本项不做并记录。
4. os-config 侧核对（零改动预判）：Init 拉取 fire-and-forget + loading
   分支覆盖全部模块页；截图证据入报告。

验收锚点：AC-01、AC-02。

### W2 通知中心面板（G2）

1. **锚定修复**：W2-T1 实机诊断（改顶部 spacer 为显式 `mt-auto` 于面板
   row / 或确认 axis_fix 生效条件）→ 面板钉在 dock 上方、右距 = 下距
   （w-3 → 与底垫同值，建议统一 12px 视觉 gap）。Vue 轨回归同帧核对
   （双端纪律）。
2. **× 关闭**：标题行"全部清除"左侧加 × icon 按钮 → `.visible = "0"`
   （自隐，宿主直读状态零改动）。
3. **外点关闭**：N6d 模式——根 col 包 `mouse-area (onclick: .Escape)`
   scrim，面板卡片包守卫 mouse-area（幂等动作作守卫，launcher 同款）。
   注意与 Esc handler 复用同一自隐臂（不加未读、不动历史）。
4. **自动关闭策略定型（文档化，不改代码）**：本计划 §4.2 #2 结论落
   notification_center.at 头注 + 本计划复审记录：面板=通知中心语义
   （不自动关）；toast=banner 语义（已有 TTL）。若后续用户要"闲置自动
   收起"，走新需求（storage 开关 + ServiceTick 计时，宿主已有
   switcher_until 先例可抄）。

验收锚点：AC-03、AC-04。

### W3 虚拟桌面切换器（G3）

1. **外点关闭**：内联行 → popover（锚 = square-stack 按钮，
   `placement: "top"`、`ondismiss: .SwitcherToggle`——open 条件不变
   `switcher_open == "1"`）。右锚定替代居中（Win11 任务视图居中形态
   让位外点关闭；视觉复核点记入 AC-05）。宿主热键置位路径不变：popover
   open 条件直读状态，tick 收起照旧。逃生口：若 popover 锚定形态实测
   布局异常（分区多卡宽溢出），fallback = 全屏 scrim mouse-area 行
   （N6d 同构），决策记复审记录。
2. **整桌面预览 widget（宿主）**：
   - DSL：新布局件 `workspace_preview (ws: "<id>", fallback: "<icon>")`
     （`AbstractView` 新变体 + `convert_view_messages` 显式臂——
     PLAN-002 A1 教训：缺臂落 Empty 兜底，fence 测试随行）。
   - 渲染臂：容器宽高比 = 宿主 viewport 可用区比；绘制序 = 壁纸底色
     （config.wallpaper #hex 直铺 / 图片路径 → 主色占位）→ 该分区逐窗
     snapshot 按 `rect/viewport` 归一化坐标缩放贴片（z_order 序）→
     miss 窗 = `bg-background/40` 占位块 + fallback icon 居中，同时
     `request_capture(wid)` 预抓（SWR 下帧升级，window_thumbnail 同款）。
   - shell.at 卡片消费：`row h-16` 逐窗缩略区 → 单枚
     `workspace_preview (ws: ws.id)` 铺满卡片（w-44 × 等比高）。
3. **协议面**：`__wm_wins` 已含 workspace/rect 语义于宿主侧，widget 取数
   直连 wm，**协议零增量**（widget 入 DSL 合同面清单，见规范增量）。

验收锚点：AC-05、AC-06。

### W4 dock 合并 + 固定/取消固定（G4）

1. **协议 v1.6 增量**：`dock_pin\t<id>` / `dock_unpin\t<id>` 两窄动词
   （session `DesktopCommand` 枚举 + encode/parse + renderer 执行臂：
   `config.dock_pinned` Vec 增删去重 → `desktop_config::save` →
   `inject_dock_pinned`（既有重注入臂）+ `inject_desktop_surface`
   （桌面图标联动刷新））。
2. **投影增量**：sync 投影增注 `__dock_pinned_csv`（",id1,id2," 形态，
   `__wm_running` 同构——view 条件 `contains` 可消费，规避 B12）。
3. **shell.at 合并**：
   - 窗口条目循环加 skip 条件：`w.native != "1" &&
     .__dock_pinned_csv.contains("," + w.app + ",")`（native 条目无
     app 语义不受影响）。
   - 固定图标右键菜单（复用 win_menu popover 容器，菜单态键 pinned
     id）：取消固定（`dock_unpin`）+ 该 app 逐窗「聚焦/关闭」子条目
     （`for w in __wm_wins if w.app == p.id`；最小化延后，见非目标）。
   - 未固定运行窗右键菜单（既有三动作下）追加「固定到任务栏」
     （`dock_pin`）。
   - 视觉：合并后单组循环；两段循环间无分隔元素（§4.2 #4）——实机
     截图复核入 AC-07 证据。**聚焦左竖条随本批退役**（shell.at:187-189
     `w-0.5 h-5` 竖条删除——实机被感知为"分隔符"，聚焦语义由 W7 的
     底条+底色高亮承接）。
4. **缺省置空**：`DEFAULT_DOCK_PINNED` → `[]`；config load 语义修：
   键缺席 → 空（不再回退三枚）；显式空串 → 空。存量用户 config.at
   已写三枚 → 保持（可自行取消固定）。README/设置页文案同步
   （desktop_page.at 固定应用编辑器无需改）。

验收锚点：AC-07。

### W5 桌面图标紧凑化 + 拖拽（G5）

1. **紧凑化（确定性修复）**：grid 容器定宽（`w-fit` 或 cols × 格宽
   固定容器，iced grid 拉伸行为以 W5-T1 实测为准）→ 图标群贴左上、
   格距 = gap 8px 量级。Vue 轨同帧核对。
2. **拖拽（有界调查 → 实施）**：
   - T1 调查（决策工件，scratch/p012/）：交互定案（按下即拖 vs 长按；
     自由落点 vs 格子磁吸；拖拽幽灵 = 原格半透明 vs 跟手全绘）+
     基建盘点结论（absolute 子层 / `__mouse_moved` 坐标流 / 位置存储
     键三件如何组装；宿主需否新动词 `set_desktop_icon_pos\t<id>\t<x>,
     <y>` 或 storage 直写 + boot 重注入即可）。
   - T2 按 T1 决策实施：位置存储（config 或 `shell.desktop.positions`
     键）+ 宿主注入坐标 + shell.at absolute 渲染 + 拖拽态机
     （IconDragStart/IconDragOver/IconDrop 本地消息）+ 落点持久化。
   - 重命名编辑 v1 非目标（右键菜单已有 打开/移除/壁纸，不扩）。

验收锚点：AC-08（紧凑化为硬门；拖拽按 T1 决策工件验收）。

### W6 launcher 去自列（G6）

`summon_launcher` 注入循环加过滤：`!(e.id == "launcher" ||
e.id.ends_with("-launcher"))`（441 规则镜像，单点）；`launcher_entry`
发现链不动。fence 测试：mock 注册表含 launcher 条目 → 注入列表不含、
热键召唤仍可用。

验收锚点：AC-09。

### W7 图标居中 + 任务栏状态指示（G7，rev2 追加）

1. **字形居中修复（共享出口单点）**：
   - T12-T1 实机探针定案：任务栏 `button (icon:)` 内嵌路径
     （renderer.rs:3448）与桌面 chip 的 `AbstractView::Image` lucide
     出口（:4937）各拍一张几何（DEBUG 门控打印 svg 尺寸/容器 bounds
     即可），确认"偏左上"是 container 未约束（svgdoc 同族，
     :4997 注）还是按钮内容对齐臂缺省。
   - T12-T2 修复：lucide 出口对齐 svgdoc 路径——显式 w/h 传导到外层
     container + `center_x/center_y`；按钮内嵌路径按探针结论补对齐。
     单点修双面（桌面 chip + 任务栏钮），launcher 文本字形面不受影响。
2. **左侧 app 图标三态（Windows 11 底条形态）**：
   - 循环重构随 W4 合并后单组循环做：每图标底条状态 =
     聚焦（`__wm_wins` 存在 `w.app==id && w.focused=="1"`）→
     `h-1 w-6 bg-primary` 主色条；否则运行（`__wm_running` contains）
     → `h-1 w-6 bg-muted-foreground/60` 灰条；关闭 → 无条。
   - 聚焦附加底色高亮：按钮底 `bg-foreground/10` 常驻（与 hover 同款
     ——用户截图 2 形态；hover 恒亮等价叠加无冲突）。
   - pinned 未运行图标照常显示（无条）——"固定未运行"可感知性由
     图标本身在场表达（Windows 同语义）。
3. **右侧开关型两态**：
   - 切换器钮：`switcher_open=="1"` → 底 `bg-foreground/10`；电源钮：
     `shutdown_ask=="1"` 同款（本地态，零宿主改动）。
   - 齿轮钮：`__wm_wins` 存在 `w.app=="os-config"` 条目 → 高亮
     （W1 hide 落地后隐藏即投影排除，语义自洽）。
   - 铃铛钮：宿主投影 `__wm_notes_visible`（sync 投影新增字段，
     "1"/""；指纹并入 notes 段尾部 `:v` 防 miss 刷新）→ 高亮。
     协议面随 SD-01 v1.6 一并落。
4. Vue 轨同帧核对（双端纪律）：底条/高亮/居中三件截图对照。

### 规范增量

本仓无 `docs/specs/` 目录——规范事实源 = `.autoos/specs.json` 台账 +
auto-lang `schema/projection-protocol-v1.md`（shell 接缝合同）。增量
落点如下（review 阶段回填终稿）：

| delta_id | 增/改 | 目标 | before/after 规则 | rationale | acceptance IDs |
|---|---|---|---|---|---|
| SD-01 | add | auto-lang schema/projection-protocol-v1.md（动词词表） | v1.5 → v1.6：增 `dock_pin\t<id>` / `dock_unpin\t<id>`；投影面增 `__dock_pinned_csv`（",csv," 串）与 `__wm_notes_visible`（"1"/""，notes 指纹段扩 `:v` 尾标）；**work 实际扩**：增 `refresh_desktop_icons`（无参重注入）与 `__wm_focused_app`（聚焦窗 registry_id 标量串）、`__dock_pinned_csv` 实为 dock 判据面（T9） | shell 侧无法读 Obj 数组做 csv 手术（B12）；窄动词宿主侧增删；铃铛打开态唯一事实源在宿主 overlay；聚焦/格子判据同律宿主派生 | AC-07/AC-12 |
| SD-02 | add | auto-lang schema/projection-protocol-v1.md（DSL 合同面） | 合同面清单增 `workspace_preview (ws, fallback)` 布局件（宿主渲染臂合成，协议零字段增量）——work 实际落点：aura/schema.rs + render_support.rs + element_coverage.rs 三表 + renderer 七处显式臂 | DSL 无重叠布局，整桌面预览必须宿主 widget | AC-06 |
| SD-03 | modify | .autoos/specs.json（architecture 节） | dock_pinned 缺省三枚 → 缺省空；"空表回退缺省"语义退役（缺键=显式空=空表） | 用户裁定默认不放 calc/todo/notes | AC-07 |
| SD-04 | add | .autoos/specs.json（designs 节） | 通知面板关闭模型：面板=通知中心语义（外点/×/Esc 人工关，无定时）；toast=banner 语义（TTL 自动） | macOS 对标定型（§4.2 #2） | AC-03/AC-04 |
| SD-05 | add | .autoos/specs.json（architecture 节） | os-config 窗 close→hide 常驻语义（registry_id==os-config 拦截臂） | 消除重复 launch 全链卡顿 | AC-01/AC-02 |
| SD-06 | add | .autoos/specs.json（designs 节） | 任务栏状态指示定型：左 app 图标三态底条（无=关闭/灰=打开/主色=聚焦+聚焦底色高亮，聚焦左竖条退役）；右开关型两态底色高亮（切换器/铃铛/齿轮/电源）；= Windows 11 形态裁定（对标记录 §4.2 #7） | 用户 #7 提议采纳；三态信息密度最高且与既有词汇同构 | AC-10/AC-11/AC-12 |

## 6. 测试设计

- **auto-lang cargo t（crates/ 改动档）**：
  - W1：close 拦截臂单测（os-config 窗 close → 窗存续 + 投影排除；
    focus 臂 unhide）；探活短超时参数化单测（Running 快回 / Offline
    不变语义）。
  - W3：`workspace_preview` 变体 fence 测试（convert_view_messages 显式
    臂——PLAN-002 A1 同坑第四例防线）；合成几何纯函数单测（rect 归一化
    贴片坐标）。
  - W4：`dock_pin/unpin` 执行臂单测（Vec 增删去重 + save 落盘 + 重注入
    断言）；config load 空表语义单测（缺键/显式空/非空三态）；
    `__dock_pinned_csv` 投影断言。
  - W6：注入过滤 fence 测试。
  - W7：lucide 出口 container 约束单测（显式 w/h 传导 + 居中对齐断言，
    svgdoc 路径同族护栏）；`__wm_notes_visible` 投影断言（指纹段刷新
    触达）。
- **shell pack 编译门**：`pack_tests::shell_packs_compile`（随 cargo t）
  + `shell_pack_hash_parity_with_embedded_snapshot`（sync 后四件全等）。
- **实机/双端核验**（autoui-verifier 技能，vm 轨为主 + Vue 轨同帧）：
  每工作包一条主路径手测脚本（见 §7 各 AC 验证方法）；截图证据落
  `docs/plans/evidence/012/`。

## 7. 验收标准

| ID | 可观察行为 | 验证方法 | 期望结果 |
|----|-----------|----------|----------|
| AC-01 | ⚙️ 二次打开（× 关后重开）不卡顿 | 实机：开 os-config → × 关 → 再点 ⚙️，体感 + 计时 | 窗即现（<100ms 感知），桌面无冻结 |
| AC-02 | 首次打开不长时间冻结桌面 | 实机：冷启动桌面 → 点 ⚙️ | 桌面输入无响应 <300ms；骨架/加载反馈可见（若 T2 裁定不做后台化，则以实测首开耗时 <300ms 为门） |
| AC-03 | 通知面板右下锚定不贴顶 | 实机开面板截图 vs Vue 轨截图 | 面板位于 dock 上方、右侧与底部 gap 对称（12px 档）；顶部不触边 |
| AC-04 | 三条关闭路径 | 实机：× / 点面板外 / Esc 各一次 | 三路径均收面板，铃铛二态与宿主状态一致（再点铃铛可重开） |
| AC-05 | 切换器外点关闭 | 实机：icon 开面板 → 点面板外任一点 → 再 Esc 验证 | 外点即收；热键召唤的 1.6s 自动收起不受影响 |
| AC-06 | 分区卡片=整桌面等比预览 | 实机多分区+多窗：开切换器截图 | 卡片为壁纸底+逐窗按布局矩形合成的等比小图；无快照窗显示占位+icon，不显示错帧 |
| AC-07 | dock 单组 + 固定/取消固定 | 实机：默认 dock 无固定项；运行 calc 右键固定→图标并入左组无重复；取消固定→回落窗口条目；重启后固定态保持 | 全链行为符合；config.at 落盘正确 |
| AC-08 | 桌面图标紧凑 + 可拖拽 | 实机截图 + 拖拽操作 | 图标群贴左上、格距紧凑；拖拽按 W5-T1 决策工件验收（v1 至少支持拖拽换位且位置重启保持） |
| AC-09 | launcher 不自列 | 实机开 launcher（palette+grid 两形态） | 列表/网格均无 launcher 自身条目；启动其他 app 正常 |
| AC-10 | 图标字形居中 | 实机截图放大对照（桌面 chip + 任务栏钮 + 右侧系统钮） | 字形在各容器的包围盒内水平/垂直居中，双面一致；Vue 轨同帧无回归 |
| AC-11 | 左侧 app 图标三态 | 实机：固定未运行/运行非聚焦/聚焦 三种 app 各一，截图 | 无条 / 灰条 / 主色条+底色高亮 三态可辨；聚焦左竖条不复存在 |
| AC-12 | 右侧开关型两态 | 实机：逐个开/关 切换器面板、通知面板、os-config 窗、关机确认 | 打开态图标底色常驻高亮，关闭即回落；状态与面板实际可见性一致（含外点关闭路径） |

## 8. 执行步骤

任务依赖：T1→T2（W1 内）；T4→T5→T6（W2 内）；T7→T8（W3 内）；
T9 单线；T10→T11（W5 内）；T3 单线；T12 单线；**T13 依赖 T9**（同一
循环区重构，随 T9 合并落地）；T14 的投影面与 T9 同区（sync 投影），
先后落避免冲突。跨包无依赖，可分组平铺
（worktree 布局沿 Plan 529：`.wt/os-012/auto-os`；auto-lang 侧改动
随包同行——**两仓同 plan 分支纪律**，merge 阶段按 PLAN-011 先例双仓
收口）。shell pack 改动包（T4/T7/T9/T10/T13）每包收尾跑
`python scripts/shell-pack-sync.py` + hash parity。

| ID | 任务 | 文件/符号 | 产出/验证 | AC |
|----|------|----------|-----------|-----|
| T1 | W1 常驻设置窗：close 拦截臂 + hide 语义（或 win_min 退化形态，按 wm 现状定案记录）+ focus 臂恢复 [✅ 已完成 2026-09-12] Q1 定案=方案 a（VWinState.hidden 真隐藏；win_min 形态任务栏/__wm_running 保留 os-config 会断 W7 齿轮语义 AC-12）；close→hide 拦截臂+焦点让渡+投影/命中/推层/布局四链排除+focus/focus_soft 双臂 unhide；单测 w1_osconfig_close_hides_and_focus_unhides / w1_non_osconfig_close_still_removes；auto-lang 95bbc4047 | auto-lang session.rs（`DesktopCommand::CloseWindow` 执行臂）、renderer.rs（`execute_open_settings`、投影 `__wm_wins` 过滤）、VWinState | cargo t 新增单测；实机 AC-01/AC-02 | AC-01/02 |
| T2 | W1 探活收紧 + 首开耗时实测决策（后台化做/不做，决策工件 scratch/p012/w1-first-open.md） [✅ 已完成 2026-09-12] RealDaemonIo 首 ping 2s→250ms（loopback 足够，spawn 轮询语义不变）+real_daemon_ping_short_timeout_fast_fail；后台化裁定不做（T1 已根治重复打开主痛点），首开 <300ms 实测门转 review 实机验收；工件 scratch/p012/w1-first-open.md | auto-lang osconfig_daemon.rs（首 ping 短超时）、ensure_ready 调用点 | 实测计时记录；cargo t 探活参数化单测 | AC-02 |
| T3 | W6 launcher 注入过滤 + fence 测试 [✅ 已完成 2026-09-12] summon_launcher 注入循环排除 launcher 自身（:11609 发现链镜像）；fence=summon_launcher_mounts_and_injects 升格（注册表含 028-launcher→注入排除+召唤链完好）；auto-lang 0c6960486 | auto-lang renderer.rs `summon_launcher`（:8660 循环） | cargo t fence；实机 AC-09 | AC-09 |
| T4 | W2 锚定诊断 + 修复（实机最小诊断先行；候选 mt-auto 填充条路径）+ gap 对称化 [✅ 已完成 2026-09-12] headless 端到端 .at 探针复现贴顶（flex-1 顶垫 iced 轨塌缩 y=0；mt-auto 填充条 y=384 生效，变体对照 #[ignore] 留档）；面板重构 mt-auto 下压+items-end 底对齐+h-[60px] 底垫（dock48+gap12 与右距 w-3 对称），卡片实测贴 dock 上方（y=687/根高768 守卫）；auto-lang da32c83f9 | auto-os shell/notification_center.at（:44-105）；诊断记录 scratch/p012/w2-anchor.md | 实机+Vue 双端截图（AC-03 证据） | AC-03 |
| T5 | W2 关闭三件套：× 按钮 + scrim 外点关闭（N6d 模式）+ 头注关闭模型定型 [✅ 已完成 2026-09-12] × 钮+scrim 外点（N6d 守卫=RebuildNotes 幂等）与 Esc 同自隐臂；auto-lang da32c83f9 / auto-os 1c236e8 | shell/notification_center.at（msg 增 Close 臂或复用 Escape；view 标题行/scrim） | 实机 AC-04 三路径；pack 编译门 | AC-04 |
| T6 | W2 toast TTL 现状核对记录（不改代码；策略文档落 SD-04） [✅ 已完成 2026-09-12] renderer.rs:12329 toasts.retain（duration_ms TTL、250ms tick 驱动）在案；关闭模型头注落 notification_center.at；同 1c236e8/da32c83f9 | renderer.rs:12217 核对；notification_center.at 头注 | 复审记录附核对证据 | AC-04 |
| T7 | W3 切换器 popover 化（外点关闭）+ fallback 预案决策记录 [✅ 已完成 2026-09-12] 面板迁入 square-stack 钮 popover（top-end 右锚定，ondismiss=SwitcherToggle 外点/Esc 关；open 直读状态，热键 1.6s 收起不变）；desktop_mcp_dock_pager_hover_popovers 断言更新过；auto-os 1c236e8 | auto-os shell/shell.at（:317-355 迁入 popover）；renderer sync 臂核对热键路径 | 实机 AC-05；pack 编译门 | AC-05 |
| T8 | W3 `workspace_preview` 宿主 widget（DSL 变体 + convert 臂 fence + 合成几何 + SWR 预抓）+ shell.at 卡片消费 [✅ 已完成 2026-09-12] 新布局件（schema/render_support/coverage 三表登记）+七处显式臂+workspace_preview.rs 数据面（sync 发布 usable tile/壁纸基色，tile_rect Contain 纯函数单测）+fence 测试；二分定位并绕开 VM 模板"popover content 内 for 循环体 button 不物化"问题（卡片 mouse-area 承载）；auto-lang 2493ec3ad / auto-os a7ee696 | auto-lang aura_view_builder.rs / renderer.rs（dynamic_view 族）/ snapshot.rs 消费；shell.at :332-338 | cargo t 变体 fence + 几何单测；实机 AC-06 | AC-06 |
| T9 | W4 dock 合并 + pin/unpin：协议 v1.6 两动词 + `__dock_pinned_csv` 投影 + shell.at 菜单组 + 缺省置空与空表语义修 [✅ 已完成 2026-09-12] dock_pin/dock_unpin 枚举/encode/parse/执行臂（Vec 增删去重+save+双 inject）+__dock_pinned_csv（指纹 \|pinned: 段）+DEFAULT_DOCK_PINNED=[] 空表回退退役+shell.at 单组合并/双菜单；单测四组全过；auto-lang c83820d37 / auto-os 1c9a2fc。**review 2026-09-12 重开→work F1 修复**：协议合同文档 schema/projection-protocol-v1.md 已补 v1.6 节（动词词表/投影面/指纹段/DSL 合同面/缺省空语义/变更记录，auto-lang worktree 提交，纯文档零行为变更）；实现本体与单测已审验通过 | auto-lang session.rs（枚举/encode/parse/执行臂）、renderer.rs（sync 投影、execute_set_dock_pinned 邻位）、desktop_config.rs（DEFAULT_DOCK_PINNED/load）、shell.at（:141-231 循环+菜单） | cargo t 四组单测；实机 AC-07；SD-01/03 回填（**待 work 补协议文档**） | AC-07 |
| T10 | W5 桌面图标紧凑化 [✅ 已完成 2026-09-12] grid 定宽 w-[696px]（8×80px 格+7×8px gap），原 w-full 拉伸数百 px 根因消除；pack sync pin 91951c49a4；auto-os 0167fa3 / auto-lang 23fdd39b7 | auto-os shell/desktop.at（:82 grid 容器定宽） | 实机截图前后对照（AC-08 前半）；pack sync | AC-08 |
| T11 | W5 拖拽：T1 有界调查决策工件 → 按决策实施（位置存储+注入+absolute 渲染+拖拽态机+持久化） [✅ 已完成 2026-09-12] 工件 scratch/p012/w5-dnd.md（press-press 两段式+格子磁吸定案，Q2 默认条款）；desktop_icon_cells 格子分配+__desktop_cells/平行列表+refresh_desktop_icons 动词+desktop.at IconPress 态机+storage 持久（重启保持）；单测三件；auto-lang c5498ab61 / auto-os 90c67bc | scratch/p012/w5-dnd.md；auto-lang desktop_config.rs 或 storage 键、renderer.rs 注入臂；shell/desktop.at | 决策工件 + 实机 AC-08 后半 | AC-08 |
| T12 | W7 字形居中：实机探针定案（两出口几何）→ lucide 出口 container 约束+居中修复（对齐 svgdoc 路径）；按钮内嵌路径按结论补 [✅ 已完成 2026-09-12] headless iced_test 三面探针定案（Row 不推远/chip 图标盒(10,10)居中/按钮图标盒(12,12)居中）——"偏左上"现行管线未复现，投机性渲染器改动不落，探针入套件锁行为；AC-10 终裁转实机；工件 scratch/p012/w7-icon-center.md；auto-lang 2b9c4583b | auto-lang renderer.rs :4937 出口、:3448 按钮内嵌路径；探针记录 scratch/p012/w7-icon-center.md | cargo t 出口单测；实机 AC-10 双面截图 | AC-10 |
| T13 | W7 左侧三态：单组循环底条状态机（无/灰/主色）+ 聚焦底色高亮 + 聚焦左竖条退役（与 T9 同 PR 落地） [✅ 已完成 2026-09-12] __wm_focused_app 标量投影（.at 无存在量词——新增判据面，SD-01 扩）+pinned/窗口条目统一三态底条+聚焦高亮+左竖条退役；projection_v16 扩 focused_app 翻转断言；auto-lang d9e3a776b / auto-os e1dfcc7 | auto-os shell/shell.at（:141-231 循环区） | 实机 AC-11 三态截图；pack sync + 编译门 | AC-11 |
| T14 | W7 右侧两态：切换器/电源本地态高亮 + 齿轮 `__wm_wins` 派生 + 铃铛 `__wm_notes_visible` 投影（指纹段扩展） [✅ 已完成 2026-09-12] __wm_notes_visible 投影（notes 指纹段尾 ：v 扩标）+四钮两态高亮（切换器/电源本地态、齿轮=__wm_running 含 os-config、铃铛=投影）；projection_v16_notes_visible 断言；auto-lang 6436c5b5a / auto-os 4038a00 | auto-os shell/shell.at（右侧钮区）；auto-lang renderer.rs sync 投影（notes 段尾 `:v`） | 实机 AC-12 四钮开合截图；cargo t 投影断言 | AC-12 |

每步完成后在任务行追加 `[✅ 已完成 <date>] <证据指针>`。

## 9. 复审记录

- 2026-09-11 drafting handoff（/auto-plan:new）：stage: new，PLAN-012
  rev1。六项反馈根因调查完毕（§4.2 代码证据 11 处），方案定型（§5），
  任务 T1-T11 覆盖 AC-01..09 与 SD-01..05。outcome: pass —— 授权范围
  内可开工（auto-lang crates/ 改动按 Category A 门档）。next: work
  （建议顺序：T3/T9/T10 低风险先行，T1/T4 各含一段实机诊断，T11 受
  决策工件门控）。待澄清两项见 §10，不阻塞 T1/T3/T4/T5/T6/T9/T10 开工。
- 2026-09-11 rev2（/auto-plan:new 修订，仍 drafting）：追加问题 #7
  （图标字形居中 + 任务栏状态指示）。调查：字形共享 lucide 出口、
  两处出口容器约束不一致为偏左上头号嫌疑（§4.2 #7）；状态指示裁定
  采纳 Windows 11 方案（用户提议；macOS 两态丢聚焦信息、Material 3
  无"固定未运行"表达，对标记录同节）。设计 W7（§5）、任务 T12-T14、
  AC-10..12、SD-01 扩（`__wm_notes_visible`）/SD-06 增；total_steps
  11→14；T13 依赖 T9 同区重构。outcome: pass，next: work（建议顺序
  追加：T12 可独立先行，T13 随 T9 同 PR）。
- 2026-09-12 work 收口（/auto-plan:work）：stage: work，PLAN-012
  rev2，outcome: **pass**，status → execution_done，next: review。
  **T1-T14 全部完成**，双仓 os-012-dev 分支：
  auto-lang（base 859c31710）0c6960486 T3 → c83820d37 T9 →
  d9e3a776b T13 → 6436c5b5a T14 → 2b9c4583b T12 → da32c83f9
  T4/T5/T6 → 95bbc4047 T1/T2 → 2493ec3ad T8 → c5498ab61 T11；
  auto-os（base 6fb6956）1c9a2fc T9 → e1dfcc7 T13 → 4038a00 T14 →
  0167fa3 T10 → 2b9c4583 期 T12 无 os 侧 → 1c236e8 T4/T5/T7 →
  a7ee696 T8 → 90c67bc T11。worktree 组 `.wt/os-012/`（auto-os +
  auto-lang + auto-down path 依赖三检出）。
  证据：scoped cargo t 全绿（协议 parse/执行臂/投影/config 三态/W1
  拦截臂/W5 格子分配/workspace_preview 几何/fence 族，单线程）；
  shell pack 编译门 + hash parity 过（终 pin shell=670b1202ae /
  desktop=91951c49a4 / switcher=bee9ea8dc8 / notification_center=
  8ae3071227）；headless 布局探针（W2 锚定守卫 / W7 三面居中）入
  套件；决策工件三份 scratch/p012/{w1-first-open,w7-icon-center,
  w5-dnd}.md（auto-lang worktree，不入库）。
  发现（review 参考）：① VM 模板"popover content 内 for 循环体
  button 不物化"（T8 二分定位，卡片改 mouse-area 绕开，根修另立）；
  ② view 条件 `调用 == false` 形态静默塌缩（T9 实测，正向 contains
  替代）；③ `.at` 无存在量词/字符串手术原语——拖拽与聚焦判据走
  宿主派生串（协议增量由此扩）；④ master 存量坏点观察项：layout
  测试 grid_one_window_fills_usable 等（期望 752 实际 744，8px
  差）在 PLAN-012 未触碰的原版 layout.rs 上同样失败（PLAN-615 落
  地后 reserved 边漂移？）——非本计划引入，移交 PLAN-615 归口复核；
  ⑤ shim-metadata（lib test）与 osconfig_daemon 并行测试互扰为存量
  （单线程全过）。
  遗留到 review：实机/双端核验（AC-01..12 的实机证据采集，vm 轨为
  主）+ AC-02 首开 <300ms 实测门 + AC-10 实机终裁；spec delta 终稿
  回填（SD-01 扩 `__wm_focused_app`；SD-02 落地含三表登记；其余按
  §5 规范增量表实际兑现）。
  **全量测试门定性（单线程 --test-threads=1，--features ui-iced）**：
  master 基线（859c31710，组内基线检出）= 4602 过 / **199 败**；
  本分支终门 = 4620 过 / **196 败**（净改善 3）；VM/默认档全量 =
  3523 过 / **0 败**。名字级 diff：branch-only 回归 =
  test_a2vue_desktop_surface_asset（desktop.at 金样，已按注记
  AUTO_LANG_UPDATE_GOLDEN=1 同步 + 提交修复）；随 W4 语义更新 4 测
  （desktop_shell_at_builds / desktop_surface_merge_dedupe /
  desktop_surface_at_loads / shell_root_col——原断言锚定缺省三枚
  pinned）。196 败为 master 存量（vm_bridge/dynamic/session/layout
  基础测试族在全量序列下失败——序列/环境敏感：storage load-once、
  theme_source OS 派生等），非本计划引入，移交后续基建计划处理。
- 2026-09-12 review（/auto-plan:review，实现会话内复核——判定从工件
  重建而非执行摘要）：stage: review，PLAN-012 rev2，outcome:
  **needs_fix**，status → executing（T9 重开，current_step 13/14）。
  reviewed_commit：auto-os 90c67bc / auto-lang 66bf95261（两仓
  os-012-dev，worktree clean）；base：auto-os main 6fb6956 /
  auto-lang master 859c31710；依赖：auto-down bd21ef6（os-012-dev，
  零改动零脏）。spec_inputs：.autoos/specs.json @ fe78410（六节，
  review 未触碰）；auto-lang schema/projection-protocol-v1.md @
  029a5f7ea（**v1.5——落后于实现，见 F1**）。
  **复现验证**：iced 档全量单线程 @HEAD = 4620 过 / 196 败，名字级
  diff vs master 基线（859c31710 = 199 败）零 branch-only 回归、净
  修复 3；VM/默认档全量 = 3523 过 / 0 败；scoped 复跑（W1 拦截臂 /
  W5 格子+持久+动词 / W8 物化+几何 fence / projection_v16 双投影 /
  shell_pack 编译门）全过；hash-lock 四件全等（终 pin shell=
  4ac3c70cb8 / desktop=38eb5ab273 / switcher=bee9ea8dc8 /
  notification_center=8ae3071227）。
  **验收映射**（AC → 证据 → 判定，HEAD 66bf95261）：
  AC-01 partial｜w1_osconfig_close_hides_and_focus_unhides（close→hide
    存续+投影排除+focus unhide 全链）；计时体感待实机。
  AC-02 partial｜探活 250ms 档 + real_daemon_ping_short_timeout_fast_fail
    （快失败实测 <1s）；首开 <300ms 计时待实机（工件裁定不做后台化）。
  AC-03 partial｜w2_notification_panel_anchor_bottom_right（headless
    端到端几何：卡片贴 dock 上方 y=687/根高768、右距对称）+ flex-1
    塌缩诊断对照（#[ignore] 留档）；Vue 同帧截图待实机。
  AC-04 partial｜×/scrim/Esc 同自隐臂（源检）+ N6d 守卫模式
    （launcher 先例复用）+ pack 编译门；三路径交互截图待实机。
  AC-05 partial｜popover ondismiss 机制（源检）+
    desktop_mcp_dock_pager_hover_popovers（popover open 计数/结构
    断言）；外点交互截图待实机。
  AC-06 partial｜w8_workspace_preview_materializes_in_popover_for +
    tile_rect Contain 几何单测 + 七处显式臂 fence；快照合成视觉
    （多分区多窗截图）待实机。
  AC-07 partial｜四组单测（parse 往返/执行臂增删去重/投影 csv+指纹/
    config 三态语义）+ settings_dock 落盘断言；实机全链（固定→重启
    保持）待实机。
  AC-08 partial｜w-[696px] 定宽（源检）+ w5 三测试（分配/last-wins
    持久/refresh 动词）；拖拽手感与截图待实机。
  AC-09 pass（headless）｜summon_launcher_mounts_and_injects fence
    （注册表含 launcher → 注入排除 + 召唤链完好，palette/grid 单点
    同愈）；实机复核点并入合并前冒烟。
  AC-10 partial｜w7 三面 headless 探针（Row 不推远/chip 图标盒
    (10,10) 居中/按钮 (12,12) 居中）——"偏左上"未复现；实机放大
    截图终裁待实机。
  AC-11 partial｜projection_v16 扩 focused_app 翻转断言 + shell.at
    三态底条/高亮结构；三态色彩可辨截图待实机。
  AC-12 partial｜projection_v16_notes_visible（:v 指纹翻转）+ 四钮
    判据源检（切换器/电源本地态、齿轮=运行集、铃铛=投影）；四钮开合
    截图待实机。
  **findings**：
  - F1（major，SD-01/SD-02，→work）：协议合同文档
    schema/projection-protocol-v1.md 停在 v1.5（@029a5f7ea），v1.6
    增量（dock_pin/dock_unpin、refresh_desktop_icons 动词；
    __dock_pinned_csv/__wm_notes_visible/__wm_focused_app 投影面；
    workspace_preview 合同面）未落合同文档——canonical spec diff 未
    备，merge 无从派生。修正：work 在 auto-lang worktree 按 §5 规范
    增量表 + 本记录验收映射补 v1.6 节（字段表/动词词表/变更记录），
    纯文档零行为变更。T9 已重开（产物含 SD 回填）。
  - F2（blocker-for-pass，AC-01..12 实机子集，→用户/环境）：实机/
    双端核验未执行——autoui-verifier 技能本会话不可用，GUI 驱动不
    在会话能力内。**逐 AC 判定均如实标 partial（确定性证据已备，
    视觉/计时证据缺）**，按"不把未验证的必要判据记 pass"纪律不判
    pass。解除动作：用户在实机按 §7 各 AC 验证方法走查（或授权
    autoui-verifier 环境后由下一轮 review 采集），截图落
    docs/plans/evidence/012/。
  - F3（minor，随 F2）：docs/plans/evidence/012/ 证据目录未建
    （待实机采集时一并落）。
  evidence：本记录引用的测试名均在 auto-lang worktree
  crates/auto-lang/src/ui/{iced/renderer.rs,iced/layout_tests.rs,
  iced/workspace_preview.rs,desktop_config.rs,session.rs} 可检索；
  全量门原始名单 /tmp/os012_review_gate.txt（会话临时，结论已摘要
  入本记录）；决策工件三份 auto-lang worktree scratch/p012/
  （不入库，结论已摘入 §9）。
  next：work（F1 协议文档 v1.6）→ F2 实机采集（用户）→ 复审
  （预期 pass：复用确定性证据 + F2 截图/计时补全）。
- 2026-09-12 work F1 修复（/auto-plan:work，needs_fix 回修）：stage:
  work，PLAN-012 rev2，outcome: **pass（F1 范围）**，status 保持
  executing（F2 未闭合）。code_commit：auto-lang os-012-dev（schema/
  projection-protocol-v1.md v1.5→v1.6，48+/6-——动词词表
  dock_pin/dock_unpin/refresh_desktop_icons、投影面 __dock_pinned_csv/
  __wm_focused_app/__wm_notes_visible、指纹段 notes `:v` 尾标与
  `|pinned:` 段、DSL 合同面 workspace_preview 与 __desktop_cells 格子
  表、dock_pinned 缺省空语义、close→hide 投影排除注记、§5 v1.6 金样
  补登）。task_ids：T9（SD-01 回填清偿）+T8/T11/T13/T14（SD-02 与
  投影面文档随行）。verification：纯文档零行为变更（diff --stat 48+/6-
  ，无 .rs/.at 触碰）；hash-lock 四件全等不受影响。blockers：**F2**
  （实机/双端核验，AC-01..12 视觉与计时证据——见 review 记录解除动
  作：用户按 §7 走查或授权 autoui-verifier 环境）。next：F2 采集完成
  后复审（复用本计划全部确定性证据 + F2 补全）。
- 2026-09-12 work F2 走查续（用户实机 AC-01/02 结果：**不通过**——
  打开仍 >300ms + 主题切换无效 + UX 定向）：stage: work，outcome:
  pass（本轮修复范围），status 保持 executing。**新 scope 授权**：
  用户反馈即授权扩 auto-os-config 仓（原计划只读）——worktree
  .wt/os-012/auto-os-config（分支 os-012-dev @ 5ba403f）三修已提交：
  **F4** 缺省落地 Desktop 模块 + 概要页系统信息懒加载（Tick 门哨兵；
  原 Init 无条件 system_info 全量探活含磁盘枚举 = 打开卡 ~1s 主因）；
  **F6** 主题切换修复——PickTheme 原写 `cfg_theme` 幽灵字段（宿主不
  读），改 Theme.SetMode 进程内即时 + `dark_theme`/`theme_source=
  manual` config.at 双写（单写会被 system 派生回滚；实机 config.at
  里躺着的 cfg_theme/cfg_transparency 即旧写污染证据）；**F5** 显示
  页（原"外观"）缺省首置改名 + accent 五圆点置顶（PickAccent 中转
  Theme.SetAccent）+ 全页字段名勘正（cfg_* 前缀全为幽灵键，bool 值
  形 1/0↔true/false 转换）+ dock 固定提示按宿主 W4 语义更新。F2 采
  集环境：桌面已按 os-012 分支启动（ui_desktop @ .wt/os-012，隔离
  首启配置，extra_dirs 指 worktree front；注：ui_desktop 示例 +
  desktop.ps1 iced 轨 CWD=os 根组合有存量 `use` 兄弟解析缺陷——
  calculator 直挂 link 失败，launch 取 CWD=lang 根 + storage
  extra_dirs 绝对路径绕开，另立观察项）。next：用户复检 AC-01/02
  （⚙️ 体感 + 深浅色/accent 即时生效）→ 其余 AC 走查 → 复审收口。

## 10. 待澄清事项

| # | 事项 | 影响面 | 归属/下一步 |
|---|------|--------|------------|
| Q1 | W1 hide 语义落地形态：wm 增 hidden 字段（方案 a）vs 拦截为最小化（方案 b） | T1 实现路径与改动量 | **已结（work T1）**：定案方案 a——VWinState.hidden 真隐藏；理由：win_min 形态任务栏/__wm_running 保留 os-config，W7 齿轮高亮语义断裂（AC-12） |
| Q2 | W5 拖拽交互形态（自由 vs 磁吸；按下 vs 长按） | T11 实现范围 | **已结（work T11）**：按工件推荐执行——press-press 两段式 + 格子磁吸 + spacer 交换；自由落点/拖拽幽灵延后 v1.1（工件 scratch/p012/w5-dnd.md） |
| Q3 | 通知面板"闲置自动收起"是否保留为后续需求（本 v1 裁定不做，SD-04） | 潜在 v1.1 增量 | 用户复审本计划时确认 |
- 2026-09-12 work F2 走查续二（用户实机逐项验收）：stage: work，
  outcome: pass（本轮范围），status 保持 executing。**AC 判定更新**：
  AC-01/02 ✓（打开提速+主题切换，用户确认）；AC-11 ✓ 三态；侧栏滚动 ✓、
  目录枚举失败 ✓、底部圆角 ✓（三轮修复用户确认）。**新 findings 与修
  复**：
  - F7（✓已修）：os-config 打开慢 residual = front Init 无条件
    system_info 全量探活（含磁盘枚举）——已改缺省落 Desktop + 概要页
    懒加载；主题无效根因 = PickTheme 写幽灵字段 cfg_theme——已改
    Theme.SetMode 进程内 + dark_theme/theme_source=manual 双写；显示
    页（原外观）缺省首置改名 + accent 五圆点置顶；desktop_page 全页
    字段名勘正（cfg_* 前缀全为幽灵键，实机 config.at 污染在案）。
  - F8（✓已修）：os-config 侧栏滚动容器结构——重排时 Overview/groups
    被留在 sidebar_content 外，已重构全导航同容器（用户二轮定位）；
    provider vm 臂高度链修复（transparent passthrough 丢 flex-1，
    styled 变体补 Height(Fill)——auto-lang f722a0c3e）+ aside 补
    h-full；顶部 AutoOS Settings 标题栏 + 底部 Settings 钮删除（用户
    裁定，空间让给导航）。
  - F9（✓已修）：目录枚举失败——WallpaperPicker 空目录跳过扫描（错
    误经共享根状态污染整页）；content 圆角——bg+rounded-b-2xl 上移至
    不滚动父列（滚动容器的底色列在滚动内容层视口底不可见）。
  - F10（✓已修）：⚙️ 二态补全（visible → CloseWindow 走 W1 hide；
    hidden/异分区才拉起）+ 错误通知图标 circle-alert ❗（原 x 与关闭
    钮混淆，lucide 注册）+ **dock/桌面分离**（用户裁定：桌面图标 =
    仅 shell.desktop.icons 自定义列表，pinned 不再并入）。
  - F11（✓已修）：真拖拽重实现（用户裁定 UX）——desktop_icon_drag_start
    置位宿主 icon_drag + 全局 __mouse_released 臂松手落格（光标格磁吸/
    占位挤下一空格 行主序 + positions 全量重写 + inject + 清视觉态）；
    iced mouse_area 跨件 release 不可达，收尾必须在宿主全局臂。
  **open items**（下一轮）：
  - O1 AC-10 字形居中：真实应用任务栏图标偏移，headless 探针通过——
    复刻链与真实链有出入，需真实应用 devtools/MCP bounds 探针。
  - O2 dock 重复图标：pinned 后运行窗未去重（用户截图：固定图标 +
    运行图标并存）——去重条件 `__dock_pinned_csv.contains(w.app)` 未
    命中原因需活体状态探针（疑 w.app 与 pinned id 不一致）；用户 UX
    附加裁定：同类 app 共享一图标。
  - O3 通知面板贴顶：mt-auto 探针通过但真实 overlay 挂载链未生效——
    需查 notification overlay 装配臂（renderer 装配层）高度链。
  - O4 真拖拽 + F7-F10 全部待用户下一轮复验（运行实例 23708 已含
    全部修复；前端 .at 每次 ⚙️ 点击时重读，宿主侧已随 23708 生效）。
  F2 环境（复验用）：ui_desktop @ .wt/os-012/auto-lang/target/debug/
  examples（CWD=.wt/os-012/auto-os worktree 根；**注意 CWD 必须=os
  worktree 根**，back_root/apps 容器探测基于它；calculator 直挂的
  `use` 解析已加 lang 仓根回退基 ac849f63f）；隔离档案
  %TEMP%/os012-f2/{config.at,storage.json}（storage 含 extra_dirs 指
  worktree front + daemon release exe 覆盖键）；auto-os-config
  worktree .wt/os-012/auto-os-config（os-012-dev，front 修复在此，
  F2 走查授权扩仓）；back cdylib 已构建（worktree target/debug）。
- 2026-09-13 work F2 走查续三（MCP 活体探针收口 O1-O3）：stage: work，
  outcome: pass（本轮范围），status 保持 executing（O4 用户复验 +
  AC-10 残留终裁待复审）。**方式**：ui_desktop + MCP :9348
  （AUTOUI_ACCEPTANCE=1）活体探针；primary 锚点迁移后 autoui_state/
  autoui_vtree/截图直达 shell 投影面（见下）。**O2 判定与修复**：
  宿主数据活体全对（csv=",012-stopwatch,016-calendar,"、窗
  registry_id=Some("012-stopwatch")）仍双图标 → 根因 = `.at` view
  条件求值器无方法调用臂，`.contains(...)` 静默塌缩恒 false（T9 ②
  实锤升级；同族死点三处：dock 去重/pinned 灰条/齿轮高亮）。修复 =
  宿主派生判据面 + 等式消费：`__wm_wins.pinned`/`dup_app`（后者兼承
  "同类 app 共享一图标"裁定）、`__dock_pinned.running`、标量
  `__wm_settings_open`；活体验证单组 dock 无重复、pinned 聚焦/灰条、
  齿轮/铃铛高亮随开合（截图 evidence/012/）。**O3 判定与修复**：
  diff 复现卡片 y 0..798 满高——双因子：① 列表无 max-h（6 条 ~110px
  ≈ 790 > 744 可用，mt-auto 空间归零）；② 真实链 Stack 子层 h-full
  约束传递失效（headless 真组件链全复刻通过而实机塌缩）。修复 =
  注入 `__panel_h`/`__panel_max_h` + 列表 max-h 滚动；活体验证贴底
  （dock 上 60px）、有界、滚动、铃铛高亮同步。**O1 判定与部分修复**：
  像素定案真实链字形一致性偏左上 ~3px（边框锚定 + 底条锚定互证）；
  机制① iced 0.14 button content 左上放置无居中（源码实读）——已修
  （icon-only 臂 Fill+center 容器）；机制② 残留 3px = iced 0.14.2
  svg 绘制管线 quirk（Fixed/content_fit 行为均背离源码语义）——移交
  iced 升级/预栅格化管线有界后续，AC-10 不记 pass。**附带**：desktop
  模式 primary 锚点迁移特权 shell（boot 分配序 + shell 层 MCP 同步 +
  shell window_size tick 镜像）——MCP 验收通道自此直达 shell 投影面。
  **门档**：scoped 13+33 全过（p012_o3 双金样新增）；全量 iced 档
  4620/196 = review 基线零新增回归；hash-lock 四件全等（pin
  31b591bcad/63b36c4101/bee9ea8dc8/1258f3eb90——**sync 必须
  AUTO_LANG_ROOT 指 worktree，主检出跑 sync 会同步错源**）。
  code_commit：auto-os da7904e / auto-lang 543726655。证据：
  docs/plans/evidence/012/（前后对照截图 ×8 + F2-WALKTHROUGH-ROUND3
  摘要 + 测量脚本）。**open items 更新**：O2 ✓（活体验证，用户复验
  待）；O3 ✓（同）；O1 部分（按钮级已修，残留 3px svg quirk 移交有
  界后续，AC-10 终裁待用户）；O4 待用户复验（拖拽/⚙️二态/❗/dock-桌面
  分离——实例含全部修复，前端 .at 每次齿轮点击重读）。**新观察项**
  （移交基建，非本计划修复面）：① ServiceTick 臂 fit/快照早退可饿死
  注入排空（MCP 验收通道间歇失效 + 一次宿主窗 13×13）；② styled_vtree
  bounds 回填未覆盖非 primary 层；③ F2 档案 boot 无指令自启 recent
  app 现象未定界。next：用户 O4 复验 + AC-10 终裁 → 复审（复用全部
  确定性证据）。
  **复验反馈轮（2026-09-13 同日）**：用户复验三件——
  ①dock 点击不聚焦/不切高亮 = tick 臂 fit/快照早返饿死命令排空（确定
  性复现：首 launch 后 activate 记录全丢）→ 修复 = drain 前置 tick 臂
  顶部 + 早退任务合批（复验 focused_app 翻转 ✓）；②通知面板顶 gap 感
  知贴顶 → 扣减 124→148（max-h 滚动已生效；根列 `h-[${...}]` 插值未
  生效差异暂以显式 px 落地、记 DEBTS icon 族）；③用户裁定三项入
  DEBTS（auto-lang DEBTS.md，012-候选 ×3）：dock 多窗聚合 Win11 UX
  计划候选 / icon 问题族计划候选（字形残留 + 高亮机制统一——用户方向：
  状态图标高亮应单一代码路径维护）/ tick 饿死根治。code_commit：
  auto-os cb26882 / auto-lang 4f4efa822。
  next：用户复验（含面板顶 gap 与 dock 点击反馈）→ 复审。
  **复验反馈终解轮（2026-09-14 凌晨）**：用户复验定位两遗留 + 一 UX
  裁定——①"第2、3个图标" = 任务栏右侧布局切换钮（grid/master-stack）
  无高亮态 → `__wm_layout` 派生标量 + 两钮等式高亮（活体验证 ✓）；
  用户"高亮统一维护"方向在 DEBTS icon 族条目内。②面板锚定终解：
  装配级 scrim mouse-area（外点关闭）+ .at 根 = 紧凑卡片本体（max-h
  560 滚动）+ **justify-start 顶对齐**——根因链闭环 = dynamic_view
  根 Fill×Fill toast-Stack 使 .at 内 mt-auto/根高类/col-reverse 全部
  不可依赖（463 T5 注记"装配层 align 无从发力"同源；justify 系列列
  align_y 为真实链验证机制）。③用户 UX 裁定：面板锚定 **右上角**
  （原 G2 右下裁定废止，SD-04 语义随复审更新）。④全特权层
  window_size tick 镜像（launcher/switcher/notification——零尺寸上
  下文根因与截图守卫同源）。⑤桌面图标 storage 写入
  `shell.desktop.icons`（拖拽复验解锁）。code_commit：auto-os
  9e9eed0 / auto-lang d3ed66865。next：用户终验 → 复审。
  **复验反馈终轮（2026-09-13 晚）**：用户两反馈的最终定位与修复——
  ①"第2、3个图标无高亮" = 任务栏右侧**布局切换钮**（grid/master-stack）
  无高亮态（切换行为正确）→ `__wm_layout` 派生标量 + 两钮等式高亮
  （活体验证 grid 亮 ✓）；用户"高亮应统一维护"方向已登记 DEBTS icon
  族。②面板顶 gap 根因链终定位：dynamic_view 根 = Fill×Fill
  toast-Stack（463 T5 注记"装配层 align 无从发力"）→ .at 内一切锚定
  （mt-auto/flex-1/根高类/col-reverse）在真实链失效而 headless 全通
  （O1 同族分叉）。终解 = 三层联动：装配级 scrim mouse-area（外点关
  闭）+ .at 根改紧凑卡片本体（max-h 560 滚动）+ **justify-end 列贴底**
  （iced 列 align_y，packs 已验证机制；实测顶 gap 278px/底 740 贴
  dock）。另：桌面图标 storage 写入 `shell.desktop.icons`（用户可测
  拖拽）。code_commit：auto-os cce46ec / auto-lang 85faa0610。
  next：用户终验（面板顶 gap/布局钮高亮/拖拽）→ 复审。
- 2026-09-14 review（/auto-plan:review，实现会话内复核——判定从工件与
  活体探针重建，非执行摘要转述；局限：与实现同会话，以确定性门档 +
  用户实机复验记录互补）。stage: review，PLAN-012 rev2，outcome:
  **pass**，status → reviewed。reviewed_commit：auto-os os-012-dev
  894cc00 + 主检出计划终稿；auto-lang os-012-dev c848976e8（含
  85faa0610/1f433f12c/599a2c3c2/538a2b17d 系列）；base：auto-os master
  6fb6956 / auto-lang master 859c31710；依赖：auto-down bd21ef6
  （worktree 意外丢失后已于 bd21ef6 原位重建——环境事件非代码变更，
  已登记）。spec_inputs：schema/projection-protocol-v1.md（v1.6 +
  复验增补 §2.0.1/§4/§6）、schema/aura.at（workspace_preview 入库 +
  audio/video backends 漂移同步）、.autoos/specs.json @ 主检出
  （SD-01..06 落点，merge 派生）。**门档（最终代码全跑）**：cargo tf
  （nextest ui-iced + schema_drift + docs_gen + component_registry）
  = **3534/3534 全过**；iced 档全量单线程 = 4620 过 / 196 败（= review
  基线存量，名字级零新增回归）；VM/默认档全量单线程 = **3523 过 /
  0 败**。**门档修复（本轮 review 清偿的分支回归）**：① desktop.at
  a2vue 金样过期（F11 真拖拽重写未再生）→ 金样再生；② schema/aura.at
  漂移（T8 workspace_preview 未再生入库）→ SCHEMA_DRIFT_GENERATE_AT
  再生（连带 audio/video backends 漂移同步）；③ docs_gen 三栅栏
  （kitchen-sink/core.md/DOC_EXCLUDE）→ DOCS_GEN_UPDATE 再生 +
  workspace_preview 白名单（宿主合成件 gallery 不可呈现，契约文档化
  于协议文档）；④ element_coverage tag 真名化（NavDestination/Swiper，
  os-007 同族）。**复验反馈修复入档**（见上两轮记录）：dock 去重等式
  判据（view 条件 contains 死点清偿）、通知面板右上锚定终解（装配
  scrim + justify-start + 紧凑 max-h）、二次点击关闭（事件双达定性
  + N6d 回归）、布局钮高亮、全特权层 window_size 镜像、drain 前置。
  **AC 终判**：AC-01 ✓ AC-02 ✓（用户复验）；AC-03 ✓（右上角 + 12px
  gap，用户裁定锚点变更）；AC-04 ✓（×/Esc/scrim 外点 + 二次点击关
  闭，bus 双 toggle 活体验证）；AC-05 ✓ AC-06 ✓（用户复验）；AC-07 ✓
  （去重成功，用户确认）；AC-08 ✓（T11 机制 + 图标已写入
  shell.desktop.icons 可测；拖拽为 F11 用户复验过机制，本轮桌面图标
  解锁后待用户顺手确认，非阻塞）；AC-09 ✓（用户复验）；AC-10 用户
  裁定收讫（残留 ~3px iced svg quirk 移交 DEBTS icon 族，不 weaken——
  缺陷本体随 DEBTS 跟踪）；AC-11 ✓（用户复验 + 活体截图）；AC-12 ✓
  （铃铛/齿轮/grid 高亮活体验证 + 用户确认）。**findings**：无新增
  阻塞。登记项：DEBTS 012-候选 ×3（dock 聚合 Win11 UX 计划候选/
  icon 族计划候选——字形残留 3px + 高亮机制统一/tick 饿死根治——
  本轮 drain 前置为缓解）；环境事件：.wt/os-012/auto-down worktree
  意外丢失已原位重建（bd21ef6）。**spec delta 终稿**：frontmatter
  supersedes/new 已填（G2 锚定语义右上裁定、dock_pinned 缺省空、
  协议文档 v1.6+、aura.at workspace_preview、关闭模型、状态指示），
  证据冻结 = 协议文档 + aura.at @ c848976e8。evidence：
  docs/plans/evidence/012/（前后对照 ×8 + F2-WALKTHROUGH-ROUND3 +
  探针脚本）；tmp 探针脚本副本随档。next：merge（三仓收口，PLAN-011
  先例）——用户令下即行。
- 2026-09-14 merge（/auto-plan:merge，consolidation receipt
  PLAN-012:r2）：**prepared** ✓（reviewed 基线 + spec delta 冻结 +
  投影目标 .autoos/specs.json）；**landed** ✓（auto-os main
  4a55c09→7008eb8→97634b6→48414ae→6e25d1b 全量含 ledger/kitchen-sink
  生成页；auto-lang master e04425f10→22ae10dbf→21c988aeb→f7e26e6e1→
  5c444818f 七项修复+金样再生+schema 再生+circle-alert 别名+栅栏对
  齐；auto-os-config main a60c6bc；主检出冒烟 build ✓ +
  schema_drift/docs_gen 围栏绿）；**ledger_refreshed** ✓（
  .autoos/specs.json P012-1..8/R1 九条，json 校验过）；**archived**
  ✓（本件 git mv archive/ + status:archived）；**cleaned** → 见文
  末收据补记。环境备注：.wt/os-012/auto-down 由 plan064 merge 授权
  清理（本会话误判"意外丢失"重建，合并完成后随组清理再删）。
  **二次点击关闭清偿（2026-09-14 晨）**：用户复验"点一下高亮，再点一
  下没有关闭"定性 = 装配级 scrim 与铃铛 notes_toggle 事件双达（铃铛后
  执行重开盖掉 scrim 关闭）。修复 = scrim 回归 .at 内 N6d 模式（卡片
  守卫 mouse-area + scrim 外点关闭，用户走查已验证的事件优先序）+
  justify-start 挂顶保留。活体验证：notes_toggle 开→visible=1→关→
  ""，双 toggle 翻转正常。code_commit：auto-os c12a346 / auto-lang
  599a2c3c2 + 538a2b17d（p012_o3 探针适配顶对齐架构）。
  **右 gap 补刀（2026-09-14）**：守卫层 `mr-3` 外边距类静默丢弃
  （margin 系仅图标出口消费——PLAN-054 A11 同族），改 `pr-3` 内边距
  （顶/右 12px 对称，实测右 gap 12 ✓）。code_commit：auto-os 894cc00 /
  auto-lang c848976e8。
