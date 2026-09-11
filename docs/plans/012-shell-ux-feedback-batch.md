---
plan_id: PLAN-012
status: drafting               # drafting → executing → execution_done → reviewed → archived
feature_name: shell-ux-feedback-batch
author: [zhaopuming]
created_at: 2026-09-11
updated_at: 2026-09-11

# /auto-plan:review 结束时填写：
supersedes_spec_components: []
new_spec_components: []
touched_goals: []             # 引用 docs/specs/goals.md 的 GOAL-NNN

affects: [auto-lang/ui/session.rs, auto-lang/ui/iced/renderer.rs, auto-lang/ui/desktop_config.rs,
          auto-lang/schema/projection-protocol-v1.md, shell/shell.at, shell/desktop.at,
          shell/notification_center.at, auto-os-config/auto/src/front]
current_step: 0
total_steps: 11
---

# [PLAN-012] shell-ux-feedback-batch

## 0. 变更摘要

用户实机体验反馈批（2026-09-11，六项，含截图一）：

| # | 问题 | 根因调查结论（详见 §4） | 工作面 |
|---|------|------------------------|--------|
| 1 | ⚙️ 打开 os-config 每次卡顿 ~1s | `execute_open_settings` launch 臂在 **UI 线程同步**跑：daemon 阻塞探活（`tcp_ping` 2s 超时/未运行 spawn+轮询 ≤5s）+ back cdylib 装载 + ~4k 行前端 .at 全量编译；× 关窗后每次重开全链重跑 | auto-lang 宿主 |
| 2 | 通知面板贴顶无 gap；无关闭 icon/外点关闭/自动关闭 | 面板顶部 `spacer flex-1` 在 iced 轨塌缩（截图实证）；关闭Affordance三缺；toast 已有 TTL 自动过期、面板为常驻历史面 | shell pack + 宿主微改 |
| 3 | 虚拟桌面面板无外点关闭；缩略是 app 小截图集合非整桌面预览 | 面板是 shell col 内联 `if switcher_open` 行（非 popover 无 ondismiss）；`window_thumbnail` 是逐窗快照，DSL 无重叠布局画不出整桌面 | shell pack + auto-lang 渲染器（新 widget） |
| 4 | dock 默认 calc/todo/notes 应去默认、与运行窗条目合并、增固定/取消固定 | `__wm_wins` 投影不排除 pinned 应用→重复显示；`set_dock_pinned` 动词已有（540 T3）；缺省三枚硬编码于 `DEFAULT_DOCK_PINNED`；代码无分隔符元素（两段 for 循环视觉分组） | shell pack + auto-lang 宿主 |
| 5 | 桌面图标间距过大、不可拖拽/编辑 | `grid (cols: 8)` 均分整行宽→格距被拉伸；DSL v1 无拖拽/无图标位置存储 | shell pack + auto-lang 宿主（含有界调查） |
| 6 | Launcher 列表显示 launcher 自身 | `summon_launcher` 全量注入 `registry_entries`，无排除 | auto-lang 宿主（一行过滤） |

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

- 授权来源：用户 2026-09-11 实机反馈原文六项（本计划 §0 表格即原文归纳）
  + 截图一（通知面板贴顶实拍）。
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

### 4.3 相邻在途计划

- PLAN-002（desktop-ux-followups，executing）：滚动跟踪计划，本批六项
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
     截图复核入 AC-07 证据。
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

### 规范增量

本仓无 `docs/specs/` 目录——规范事实源 = `.autoos/specs.json` 台账 +
auto-lang `schema/projection-protocol-v1.md`（shell 接缝合同）。增量
落点如下（review 阶段回填终稿）：

| delta_id | 增/改 | 目标 | before/after 规则 | rationale | acceptance IDs |
|---|---|---|---|---|---|
| SD-01 | add | auto-lang schema/projection-protocol-v1.md（动词词表） | v1.5 → v1.6：增 `dock_pin\t<id>` / `dock_unpin\t<id>`；投影面增 `__dock_pinned_csv`（",csv," 串） | shell 侧无法读 Obj 数组做 csv 手术（B12）；窄动词宿主侧增删 | AC-07 |
| SD-02 | add | auto-lang schema/projection-protocol-v1.md（DSL 合同面） | 合同面清单增 `workspace_preview (ws, fallback)` 布局件（宿主渲染臂合成，协议零字段增量） | DSL 无重叠布局，整桌面预览必须宿主 widget | AC-06 |
| SD-03 | modify | .autoos/specs.json（architecture 节） | dock_pinned 缺省三枚 → 缺省空；"空表回退缺省"语义退役（缺键=显式空=空表） | 用户裁定默认不放 calc/todo/notes | AC-07 |
| SD-04 | add | .autoos/specs.json（designs 节） | 通知面板关闭模型：面板=通知中心语义（外点/×/Esc 人工关，无定时）；toast=banner 语义（TTL 自动） | macOS 对标定型（§4.2 #2） | AC-03/AC-04 |
| SD-05 | add | .autoos/specs.json（architecture 节） | os-config 窗 close→hide 常驻语义（registry_id==os-config 拦截臂） | 消除重复 launch 全链卡顿 | AC-01/AC-02 |

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

## 8. 执行步骤

任务依赖：T1→T2（W1 内）；T4→T5→T6（W2 内）；T7→T8（W3 内）；
T9 单线；T10→T11（W5 内）；T3 单线。跨包无依赖，可分组平铺
（worktree 布局沿 Plan 529：`.wt/os-012/auto-os`；auto-lang 侧改动
随包同行——**两仓同 plan 分支纪律**，merge 阶段按 PLAN-011 先例双仓
收口）。shell pack 改动包（T4/T7/T9/T10）每包收尾跑
`python scripts/shell-pack-sync.py` + hash parity。

| ID | 任务 | 文件/符号 | 产出/验证 | AC |
|----|------|----------|-----------|-----|
| T1 | W1 常驻设置窗：close 拦截臂 + hide 语义（或 win_min 退化形态，按 wm 现状定案记录）+ focus 臂恢复 | auto-lang session.rs（`DesktopCommand::CloseWindow` 执行臂）、renderer.rs（`execute_open_settings`、投影 `__wm_wins` 过滤）、VWinState | cargo t 新增单测；实机 AC-01/AC-02 | AC-01/02 |
| T2 | W1 探活收紧 + 首开耗时实测决策（后台化做/不做，决策工件 scratch/p012/w1-first-open.md） | auto-lang osconfig_daemon.rs（首 ping 短超时）、ensure_ready 调用点 | 实测计时记录；cargo t 探活参数化单测 | AC-02 |
| T3 | W6 launcher 注入过滤 + fence 测试 | auto-lang renderer.rs `summon_launcher`（:8660 循环） | cargo t fence；实机 AC-09 | AC-09 |
| T4 | W2 锚定诊断 + 修复（实机最小诊断先行；候选 mt-auto 填充条路径）+ gap 对称化 | auto-os shell/notification_center.at（:44-105）；诊断记录 scratch/p012/w2-anchor.md | 实机+Vue 双端截图（AC-03 证据） | AC-03 |
| T5 | W2 关闭三件套：× 按钮 + scrim 外点关闭（N6d 模式）+ 头注关闭模型定型 | shell/notification_center.at（msg 增 Close 臂或复用 Escape；view 标题行/scrim） | 实机 AC-04 三路径；pack 编译门 | AC-04 |
| T6 | W2 toast TTL 现状核对记录（不改代码；策略文档落 SD-04） | renderer.rs:12217 核对；notification_center.at 头注 | 复审记录附核对证据 | AC-04 |
| T7 | W3 切换器 popover 化（外点关闭）+ fallback 预案决策记录 | auto-os shell/shell.at（:317-355 迁入 popover）；renderer sync 臂核对热键路径 | 实机 AC-05；pack 编译门 | AC-05 |
| T8 | W3 `workspace_preview` 宿主 widget（DSL 变体 + convert 臂 fence + 合成几何 + SWR 预抓）+ shell.at 卡片消费 | auto-lang aura_view_builder.rs / renderer.rs（dynamic_view 族）/ snapshot.rs 消费；shell.at :332-338 | cargo t 变体 fence + 几何单测；实机 AC-06 | AC-06 |
| T9 | W4 dock 合并 + pin/unpin：协议 v1.6 两动词 + `__dock_pinned_csv` 投影 + shell.at 菜单组 + 缺省置空与空表语义修 | auto-lang session.rs（枚举/encode/parse/执行臂）、renderer.rs（sync 投影、execute_set_dock_pinned 邻位）、desktop_config.rs（DEFAULT_DOCK_PINNED/load）、shell.at（:141-231 循环+菜单） | cargo t 四组单测；实机 AC-07；SD-01/03 回填 | AC-07 |
| T10 | W5 桌面图标紧凑化 | auto-os shell/desktop.at（:82 grid 容器定宽） | 实机截图前后对照（AC-08 前半）；pack sync | AC-08 |
| T11 | W5 拖拽：T1 有界调查决策工件 → 按决策实施（位置存储+注入+absolute 渲染+拖拽态机+持久化） | scratch/p012/w5-dnd.md；auto-lang desktop_config.rs 或 storage 键、renderer.rs 注入臂；shell/desktop.at | 决策工件 + 实机 AC-08 后半 | AC-08 |

每步完成后在任务行追加 `[✅ 已完成 <date>] <证据指针>`。

## 9. 复审记录

- 2026-09-11 drafting handoff（/auto-plan:new）：stage: new，PLAN-012
  rev1。六项反馈根因调查完毕（§4.2 代码证据 11 处），方案定型（§5），
  任务 T1-T11 覆盖 AC-01..09 与 SD-01..05。outcome: pass —— 授权范围
  内可开工（auto-lang crates/ 改动按 Category A 门档）。next: work
  （建议顺序：T3/T9/T10 低风险先行，T1/T4 各含一段实机诊断，T11 受
  决策工件门控）。待澄清两项见 §10，不阻塞 T1/T3/T4/T5/T6/T9/T10 开工。

## 10. 待澄清事项

| # | 事项 | 影响面 | 归属/下一步 |
|---|------|--------|------------|
| Q1 | W1 hide 语义落地形态：wm 增 hidden 字段（方案 a）vs 拦截为最小化（方案 b） | T1 实现路径与改动量 | T1 开工首日按 VWinState/布局排除现状定案，记录于任务证据 |
| Q2 | W5 拖拽交互形态（自由 vs 磁吸；按下 vs 长按） | T11 实现范围 | T11-T1 决策工件产出后用户可选复核；默认按工件推荐执行 |
| Q3 | 通知面板"闲置自动收起"是否保留为后续需求（本 v1 裁定不做，SD-04） | 潜在 v1.1 增量 | 用户复审本计划时确认 |
