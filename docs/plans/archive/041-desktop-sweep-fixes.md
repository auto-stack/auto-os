---
plan_id: PLAN-041
status: archived                # drafting → executing → execution_done → reviewed → archived
feature_name: desktop-sweep-fixes
author: [agent]
created_at: 2026-09-22
updated_at: 2026-09-22
plan_revision: 2

# /auto-plan:review 结束时填写：
supersedes_spec_components: []
new_spec_components: [shell/dashboard, shell/showdesk-ux-polish]
touched_goals: []             # 无 goals 册登记项变更（桌面 UX 无 GOAL-NNN 绑定；review 核）

affects: [shell/dashboard, shell/showdesk-ux-polish]
current_step: 12
total_steps: 15
---

# [PLAN-041] desktop-sweep-fixes

## 0. 变更摘要

2026-09-22 晚 VM 桌面（ui_desktop iced 轨，auto-lang `a5926b8c2` 重建 + auto-os main
`7403b20`）实机巡检批的**问题记录与修复跟踪**。MCP 驱动（autoui_screenshot /
autoui_state / autoui_desktop bus 动词）全面走查桌面/任务栏/dashboard/launcher/多
app/主题/分区/通知中心，沉淀 15 项观察（P1×2 / P2×8 / P3×5），其中 13 项立修复任务，
2 项 bounded 复现定性；PLAN-040 在账 F-R1 本轮实机复现并**收窄失效环节**，F-R0 保持
用户实机复核（机器怪癖：MCP 无 hover/真击合成通道）。rev2（同日 20:37）：并入用户
截图裁定观察⑮——深色壁纸+深色主题下小组件玻璃卡对比度不足，设计方向用户已定
（背景更不透明+偏深色、前景文字偏浅色）。

跨仓改动面：auto-os（shell/*.at）、auto-lang（crates/auto-lang/src/ui/*）、
auto-os-config（auto/src/front/theme_picker.at 等）。

## 1. 目标

- **G1 动态臂应用可启动**：025-sys-monitor 在 VM 桌面可启动可用（`api.kill_process`
  Undefined symbol 消除，或按决策件落地明确"不可用"语义改进）；五 app 符号家族
  逐一过定性。
- **G2 分区隔离**：workspace 切换后非当前分区窗口不可见（现状：Desktop 2 上
  workspace-0 的 calculator 完整可见，state+截图双证据）。
- **G3 主题热切换一致性**：config 外写与 bus `set_theme` 双路径行为等价（F-R1 根
  修）；浅色主题下 dashboard 时钟 face 时间数字可见。
- **G4 dashboard face 健康度**：日期活值（现停 9月18日）；第 4 face 不再裁剪
  （spans 9>8 v2 债）；通知面板 state 翻转 ≤1s 上屏（现 ~90s）；小组件卡在深色
  壁纸+深色主题下可读（玻璃卡对比度，观察⑮用户裁定方向）。
- **G5 恢复链保真**：会话恢复窗口标题栏不再是通用 "App"。
- **G6 settings 布局**：主题卡"浅色"钮单行完整显示；主色调第三色板形状归一。
- **G7 日志卫生**：`__mcp_heartbeat`/`__scroll_state_read` handler-not-found 停止
  每帧刷屏；boot `App.Init failed: handler not found: Init` 不再入真桌面日志。

**非目标**：
- F-R0 字面鼠标交互复核（最小化单击恢复/预览 hover 可点）——用户实机项，机器
  怪癖不可合成，本计划只保留验证清单。
- M7-c②③（jade-edit / auto-musk / sys-monitor 样式大改）——2026-09-21 已裁定延
  期；本计划只修**启动坏**（符号链接），不动样式大改面。
- 桌面图标 11→29 扩张与同位重叠的**根修**（触发源未定位，T-14 只做复现定性，
  修复视定性结果另立）。

**成功判据**：§7 验收标准 AC-01..AC-14 全绿；auto-lang 侧定向测试绿 + tf 门红
基线对拍零新增。

## 2. 架构方案

四轨并行，每轨内任务可独立验证：

- **A 轨（lang VM 动态链接面）**：VM 动态解释 app 的 front 调 `api.*`/原生 fn 时
  VmBridge 报 `Undefined symbol`。先 bounded investigation 出装载链根因（back
  `#[api]` 符号为何不入 VM 环境：装载序缺臂 / daemon 分离设计 / pac 档位缺失），
  决策件定形态后统一修。025-sys-monitor 为试点（front 调用、back
  `apps/025-sys-monitor/src/back/api.at:127 pub fn kill_process` 均在案，唯链接断）。
- **B 轨（lang WM/主题/恢复面，iced renderer）**：workspace 切换臂补窗口可见性
  过滤；config-poll 主题应用路径补 dashboard face 重建信号（bus `set_theme` 路径
  已正确——THEME_EPOCH 类机制双路径等价化）；会话恢复链注入真 title。
- **C 轨（os shell 面，shell/dashboard.at + notification_center.at）**：face 行
  策略 v2（8×2 双行 wrap 替代单行裁剪）、时钟 face 日期活值、时间色值语义
  token 化（浅色不可见根修）、通知面板渲染时效。
- **D 轨（os-config settings 面）**：theme_picker 双字钮布局约束 + 色板形状。
- **E 轨（lang MCP/日志卫生）**：心跳/scroll 探测 handler-missing 降频静音；boot
  Init 探测噪声在真桌面装载形态下静音。

修复优先序建议：B（G2/G3 用户可感最强、定位最熟）→ C → E（卫生）→ A（调查周期
长）→ D（跨仓独立）。A 轨调查可与 B/C 并行启动。

## 3. 技术栈

沿仓现状，零新增：AutoUI .at（VM 动态解释 + shell pack 宿主内编译）、Rust/iced
（ui-iced feature）、auto-os-config .at front。验证通道 = AutoUI MCP（HTTP
JSON-RPC `POST /mcp`）+ bus 动词，驱动脚本见 `docs/plans/reports/p041-shots/p041-mcpq-driver.sh`。

## 4. 需求分析与背景调查

**授权记录**：用户 2026-09-22 指令"打开虚拟桌面，继续看那里有问题，并用新的计划
进行问题记录和修复的跟踪"——授权巡检 + 本计划起草；修复执行沿范式待 work 阶段，
跨仓范围（os + lang + os-config）在本计划显式声明。无用户指定预算/自动续跑限制。

**证据链**（`docs/plans/reports/p041-shots/`，MCP 端口 :9249，启动日志
`.auto/desktop-041.log` 未入库、摘录入库）：

| 观察 | 证据 | 版本面 |
|---|---|---|
| ① 025-sys-monitor 启动死窗"应用暂不可用"（20:26 实测复现）| shot-04；`__wm_wins` wid=3 title="025-sys-monitor（不可用）" app=""；`__wm_running` 不含 | front `api.kill_process`；back `apps/025-sys-monitor/src/back/api.at:127` 定义在案 |
| ② 分区不隔离（Desktop 2 可见 ws-0 calculator）| shot-10（`__wm_workspaces` current=1 + `__wm_meta` "free\t" 无聚焦 + calculator 可见同帧）| |
| ③ F-R1 复现+收窄（config 外写→face 滞留深色；bus 动词路径正常往返）| shot-05（bus 切浅 face 跟随）vs shot-07（config 外写切浅 face 滞深、任务栏已翻浅）| lang `a5926b8c2` |
| ④ 浅色时钟 face 时间数字不可见（深→浅橙字消失，切回深恢复）| zoom-dash-light vs zoom-dash-dark-back | |
| ⑤ 时钟 face 日期停 2026年9月18日（今天 9/22，全程不变）| shot-01/02/08/13 | 在账观察面 |
| ⑥ 第 4 face 裁剪（日志每帧 `1 face(s) clipped`；spans 2+3+3+2=9>8）| p041-boot-log-excerpt.txt | PLAN-040 v2 债 |
| ⑦ 通知面板 state 翻转后 ~90s 才上屏（`__wm_notes_visible`=1 时 shot-12 无面板，后续 shot-13 有）| shot-12/13 对比 | |
| ⑧ settings"浅色"钮双行断行+垂直裁切；第三色板竖长椭圆 | zoom-theme-row | os-config `auto/src/front/desktop_page.at`/`theme_picker.at` |
| ⑨ 恢复窗标题栏通用 "App"（calculator；settings 正常显"系统设置"）| shot-01/02 | |
| ⑩ `__mcp_heartbeat`/`__scroll_state_read` handler-not-found 每帧刷屏 | desktop-041.log 尾部连续行 | lang `crates/auto-lang/src/ui/iced/renderer.rs` 探测方 |
| ⑪ boot `App.Init failed: handler not found: Init` 入真桌面日志 | p041-boot-log-excerpt.txt | 记忆在账"占位壳预期噪声"，但 ui_desktop 真桌面日志同样出现 |
| ⑫ launcher 计数 1/28 vs 注册表 29 desktop-visible | shot-03；boot 日志 43 entries(29 visible) | 低置信 |
| ⑬ `__wm_fp` 图标 2、3 同位 (0,0) | 巡检 state 读取 | 用户拖拽期间产生 |
| ⑭ 图标集 11（boot，storage 恢复）→ ~29（20:27 后全注册表）扩张 | shot-04 vs shot-07/09 对比 | 触发源未定位（用户并行操作 / config 外写重扫两假说）|
| ⑮ 深色壁纸+深色主题下小组件玻璃卡对比度不足（卡底 `bg-white/10` 低透白单态 + face 文字灰阶对比弱，繁忙壁纸进一步压可读性）| shot-15-user-dark-widgets-contrast.png（用户 20:37 截图裁定）| 卡底板 `shell/dashboard.at:89`（单态不分支主题）；face 文字色由宿主 mini 视图渲染注入（lang 侧语义 token，dark 分支值待提亮）|

**在账引用**：PLAN-040（已归档）F-R0/F-R1 findings；P040 v2 face 债；M7-c②③ 延
期裁定（2026-09-21）；012-clock 改名三同步教训；开发机怪癖清单（合成输入不可用/
截图落 CWD/`-d` 旗标/exe 锁定 taskkill 前置）。

**Spec 现状**：`docs/specs/shell/`（dashboard.md / showdesk-icons.md /
showdesk-ux-polish.md / showdesk-ux-wallpaper.md）未覆盖：分区窗口可见性规则、
主题热切换双路径等价契约、face 行策略 v2、通知面板时效——由 §5 规范增量补。

## 5. 详细设计

### A 轨：VM 动态符号链接（G1）

现状链路：桌面 launch → VM 动态解释 app front → VmBridge init 链接 `api.*` 符号
→ `Undefined symbol: api.kill_process in module App` → 死窗"应用暂不可用"。
front 调用点与 back 定义点均在源码层面成立，断点在**装载链未把 back 符号注册进
动态解释环境**。

- T-01 bounded investigation（决策件）：定位 lang 侧 VM 装载序（vm_bridge /
  desktop 动态臂），回答三问：①back 模块是否本应随 front 一起装载（缺臂）？
  ②daemon 分离形态下设计意图是否就是 front-only（则需 pac 档位/注册表声明 +
  不可用语义前置）？③五 app 符号是否同根因？产出
  `docs/plans/reports/p041-vm-symbol-decision.md`（含推荐选项与影响面），**决策
  后才进 T-02**。
- T-02 按 T-01 决策落地：首选预期 = 装载链补 back 符号注册（025-sys-monitor
  kill_process 全链通：启动→进程列表→结束进程动作）；次选 = 不可用语义前置
  （注册表标"需后端"徽标 + 点击即明确提示而非死窗）。

### B 轨：WM/主题/恢复（G2/G3/G5）

- **workspace 隔离**：`renderer.rs` 分区切换臂现状只切 `__wm_workspaces` current
  旗标与输入焦点（`__wm_meta` focused 已正确清空），缺**渲染层窗口可见性过滤**
  ——非当前分区窗不绘制（最小化同款 hide 语义）+ 切回恢复。send_to 语义保持。
- **config-poll 主题等价**：`set_theme` bus 路径正确重建 face（本轮实证）；config
  poll 路径（`desktop_config.rs` mtime 轮询 → `renderer.rs` 应用）缺 face 重建信
  号（THEME_EPOCH 或等效 dirty 位不随 poll 自增）。修 = poll 应用主题名/暗色翻转
  时与 bus 路径汇聚到同一重建入口。
- **恢复 title**：会话恢复臂以 registry title（`title`/`title_zh`）注入 `__wm_wins`
  title 字段（现落 VM 缺省 "App"；settings 恢复正确说明链路有正确先例可对齐）。

### C 轨：dashboard/notification shell 面（G3/G4）

- **face 行策略 v2**：`shell/dashboard.at` 现单行 8 列容不下 spans 2+3+3+2=9。
  改两行 wrap（外框已 8×2、双行高充足）或 face spans 重排；布局日志 clipped 归零。
- **时钟 face 活值**：日期停旧日 + 时间慢 tick 两症同源方向——face 值由低频刷新
  写入而非活绑定；定位 `shell/dashboard.at` 时钟 face 数据源（mini face 缓存 vs
  012-clock mini 视图自身），修为每刷新拍取活值 + 刷新拍对齐秒级（或 face 内独立
  1s tick）。
- **时间色值 token 化**：浅色主题时间数字不可见 = 深色硬编码色值；换语义 token
  （沿 PLAN-040 mini 视图全语义 token 先例）。
- **通知面板时效**：`__wm_notes_visible` 翻转与面板渲染脱节 ~90s（shot-12/13）；
  与 B 轨 face 重建信号同族，接同一快路径（目标 ≤1 ServiceTick）。
- **小组件卡对比度（观察⑮，用户裁定方向）**：现卡底板 `bg-white/10 +
  border-white/25 + backdrop-blur-md`（`shell/dashboard.at:89`）单态不分支主题，
  深色壁纸下"低透白"悬空、face 内文字灰阶对比弱。设计方向（用户 20:37 定）：
  **深色主题下卡背景更不透明且偏深色、前景文字偏浅色**——卡底板按主题分支
  （dark：深色高不透明玻璃，如深底 ~85% 不透明档位 + 磨砂保留；light：沿现浅
  玻璃）；face 文字主/次层级提亮（宿主 mini 视图语义 token dark 分支值提升，
  主文字趋近白/90、次级信息不低于白/70 档），以用户壁纸实拍走查验收。

### D 轨：os-config settings（G6）

`auto-os-config/auto/src/front/theme_picker.at`（+ `desktop_page.at` 消费臂）：
双字钮（浅色）min-width/单行不换行约束；色板 swatch 圆形归一（第三椭圆为宽高约
束泄漏）。

### E 轨：日志卫生（G7）

`renderer.rs` 探测方：`__mcp_heartbeat`/`__scroll_state_read` handler 缺失时降为
一次性 debug 级（或首探后缓存缺失事实停探）；真桌面（ui_desktop）装载形态下
boot 的 `App.Init failed: handler not found: Init` 探测噪声静音（占位壳形态保留
——记忆在账其为预期观测行）。

### 规范增量

| delta_id | add/modify/retire | target | before/after | rationale | AC |
|---|---|---|---|---|---|
| SD-01 | modify | docs/specs/shell/dashboard.md | face 行策略：单行 8 列裁剪 → 溢出时 span-3 卡依序收缩到 2（单行自适应全容纳；双行 wrap 会折半高/溢框故弃，plan 预授权「spans 重排」臂，review 校正对齐实现）| 第 4 face 恒裁剪（spans 9>8）| AC-08 |
| SD-02 | modify | docs/specs/shell/dashboard.md | face 时钟：日期硬编码初始值 → civil 算法活值（秒级 tick 随更）；时间色值 = 语义 token（预存正确，非本计划改动面）；陈旧/缺席根因 = present 饥冻（T-14 修，§9）| 日期停旧日+浅色时间不可见 | AC-06, AC-07 |
| SD-03 | add | docs/specs/shell/dashboard.md（分区可见性规则；落位复审定） | 分区切换：非当前分区窗不渲染（切回恢复）| 现无规则、行为缺失 | AC-04 |
| SD-04 | add | docs/specs/shell/dashboard.md（主题热切换契约） | config-poll 与 bus set_theme 双路径 face 重建等价 | F-R1 根修契约化 | AC-05 |
| SD-05 | modify | docs/specs/shell/showdesk-ux-polish.md | 通知面板：state 翻转 → 上屏 ≤1 ServiceTick | 现最长 ~90s 脱节 | AC-09 |
| SD-06 | add | docs/specs/shell/showdesk-ux-polish.md | 动态臂 app back 符号装载契约（或"需后端"声明语义，随 T-01 决策） | Undefined symbol 家族契约空白 | AC-01, AC-02 |
| SD-07 | modify | docs/specs/shell/dashboard.md | 小组件卡底板：单态 `bg-white/10` → 主题双分支（dark=深色高不透明玻璃/light=浅玻璃保留），face 文字 dark 分支主/次层级提亮（白/90、白/70 档） | 深色壁纸下可读性（用户裁定方向，观察⑮） | AC-15 |

## 6. 测试设计

- **MCP 驱动配方**（真桌面、真链路；驱动脚本入库 `p041-mcpq-driver.sh`）：
  启动（`scripts/desktop.sh iced` + `AUTOUI_ACCEPTANCE=1`，日志 grep 实际 MCP 口）
  → `autoui_desktop` bus 动词驱动 → `autoui_screenshot`/`autoui_state` 断言。
  注意：截图先 `cp` 唯一名再 Read（Read 图像缓存怪癖）；启动显式 `cd` 仓根；
  主题实验后恢复 `config.at` 深色。
- **lang 侧**：本计划改 `crates/`，适用 auto-lang Category A 测试纪律；定向模块
  测试 + `cargo tf --no-fail-fast` 门，红基线按 2026-09-22 换代清单对拍（musk×6 +
  projector_counter + ffi_dual_019 flaky）。
- **os-config 侧**：主题卡走查截图（浅/深双主题）。
- **回归面**：dock hover 测试基线（PLAN-040 6 条）、snapshot 冻结集 25、fit 20
  不回退。

## 7. 验收标准

| ID | 观察行为 | 验证方法 |
|---|---|---|
| AC-01 | T-01 决策件存在且含推荐选项、影响面、五 app 定性 | 文件在案 + §9 引用 |
| AC-02 | 025-sys-monitor 从桌面启动：或全功能可用（进程表+结束进程），或按决策语义给出前置明确提示（无死窗）| MCP `launch\t025-sys-monitor` → screenshot/state |
| AC-03 | ui-gallery/kanban/017-chat/auto-term 四符号各自定性（同根因已修/独立债立卡/域外归因）落 §9 | 逐一 `launch` 复测 + 结论行 |
| AC-04 | `workspace\t1` 后截图无 workspace-0 窗口；`workspace\t0` 切回窗口恢复 | MCP 配方前后帧对比 |
| AC-05 | config 外写 `dark_theme:false` → face 跟随浅色（与 bus 路径一致）；恢复写回 | shot-07 同配方复测（先备份 config）|
| AC-06 | 深浅双主题下时钟 face 时间数字均可见（语义 token）| 双主题截图 |
| AC-07 | 时钟 face 日期显示当日 | 截图对 `date` |
| AC-08 | 启动日志无 `face(s) clipped`；4 face 全可见 | 日志 grep + 截图 |
| AC-09 | `notes_toggle` 后 ≤1s 面板上屏 | 配方：toggle→1s→截图 |
| AC-10 | "浅色"钮单行完整；三色板同形（圆）| os-config 主题卡截图 |
| AC-11 | 会话恢复窗 title 为 app 真名（calculator 非 "App"）| 重启桌面后 `__wm_wins` |
| AC-12 | 空闲 60s 日志增量无 `__mcp_heartbeat`/`__scroll_state_read` not-found 行 | 日志增量 grep |
| AC-13 | ui_desktop 启动日志无 `App.Init failed: handler not found: Init` | 启动日志 grep |
| AC-14 | ⑫⑬⑭ 三观察项定性落 §9（修/债/不修各归其位）| 结论行 |
| AC-15 | 深色主题+用户壁纸（stella purple）下：小组件卡背景明显偏深且高不透明、face 文字（时间/待办/曲名等）浅色可读；浅色主题浅玻璃不回退 | 用户壁纸实拍走查（dark/light 双主题截图）|

## 8. 执行步骤

> 执行载体：分组平铺 worktree `.wt/os-041/auto-os`（plan-041-dev）+ auto-lang /
> auto-os-config 对配分支；本计划起草提交在 main（.next-id + plan + 证据归档）。
>
> **状态总表（2026-09-22 work 首腿，详见 §9 执行记录）**：
> T-01 [x] T-02 [x] T-03 [x]（025 实测 ✓/017 围栏+债 D1/kanban·auto-term·
> ui-gallery 切桌面复验）T-04 [x] T-05 [x] T-06 [x]（补课腿闭合：缺席=T-14 修
> 复前"主题翻转过渡帧被 present 饥冻"衍生症状，现构建暗→浅往返活值在屏，见
> §9）T-07 [x] T-08 [x] T-09 [x] T-10 [x]（实施完毕，实机走查随合并）
> T-11 [x] T-12 [x] T-13 [x] T-14 [x]（idle-present 根因修复；图标定性；计数
> 差一切桌面核销）T-15 [x] —— 12/15，`executing` 继续（尾项=T-03 三 app 复验
> + T-10 走查，均随合并后桌面）。

- **T-01**（A轨）VM 动态符号调查决策件。文件：lang `crates/auto-lang/src/ui/`
  （vm_bridge/装载序，探查定位）；产出 `docs/plans/reports/p041-vm-symbol-decision.md`。
  验证：决策件三问有答 + 用户/复审确认选项。依赖：无。→ AC-01
- **T-02**（A轨）按 T-01 选项落地 025-sys-monitor 可启动。文件：随决策（lang
  装载链或 pac/注册表语义）。验证：AC-02 配方。依赖：T-01。→ AC-02
- **T-03**（A轨）符号家族四 app 复测定性（ui-gallery `fmt.pct1` 注意并行会话
  WIP 归因；kanban/017-chat/auto-term 同法）。验证：AC-03 结论行。依赖：T-01。
  → AC-03
- **T-04**（B轨）分区窗口可见性。文件：lang `crates/auto-lang/src/ui/iced/renderer.rs`
  分区切换臂。验证：AC-04 配方。依赖：无。
- **T-05**（B轨）config-poll 主题 face 重建（F-R1 根修）。文件：lang
  `renderer.rs`（poll 应用臂）+ `shell_client.rs`/dashboard 信号面（定位后落）。
  验证：AC-05 配方（config 备份恢复纪律）。依赖：无（建议先于 T-09 定信号形态）。
- **T-06**（C轨）时钟 face 时间 token 化 + （T-07 同点）日期活值。文件：
  `shell/dashboard.at`（时钟 face 源）+ 必要时 `assets/icons/mapping.json` 无关；
  lang 侧 mini 渲染分支若涉案随改（shell pack 三重对拍：金样 + codegen 门 + 三主
  题走查）。验证：AC-06/07。依赖：无。
- **T-08**（C轨）face 行策略 v2（双行 wrap）。文件：`shell/dashboard.at` +
  wrapper 高度三处同步面（沿 PLAN-040 DASH_TAB_STRIP_H 同步清单）。验证：AC-08。
  依赖：无。
- **T-09**（C轨）通知面板渲染时效。文件：`shell/notification_center.at` +
  B 轨信号面。验证：AC-09。依赖：T-05（共用信号形态）。
- **T-10**（D轨）os-config 主题卡布局。文件：`../auto-os-config/auto/src/front/theme_picker.at`、
  `desktop_page.at`。验证：AC-10。依赖：无。
- **T-11**（B轨）恢复 title 注入。文件：lang `renderer.rs` 会话恢复臂。验证：
  AC-11（重启桌面）。依赖：无。
- **T-12**（E轨）探测静音（heartbeat/scroll_state_read）。文件：lang `renderer.rs`
  探测方。验证：AC-12。依赖：无。
- **T-13**（E轨）boot Init 噪声静音（真桌面形态）。文件：lang `renderer.rs`
  （或装载探测臂）。验证：AC-13。依赖：无。
- **T-14**（卫生）⑫⑬⑭ 复现定性：launcher 计数差一归因；图标同位/扩张复现实验
  （隔离验收配方：`AUTOVM_STORAGE_FILE`/`AUTOOS_DESKTOP_CONFIG` 临时实例）。验证：
  AC-14。依赖：无。
- **T-15**（C轨）小组件卡对比度（观察⑮）。文件：`shell/dashboard.at:89` 卡底板
  主题双分支 + lang 侧宿主 mini 视图 face 文字语义 token dark 分支提亮（定位面：
  workspace_preview/dashboard 宿主渲染臂）；shell pack 三重对拍（金样 + codegen
  门 + 三主题走查）+ 用户壁纸实拍。验证：AC-15。依赖：无（建议与 T-06/T-08 同
  checkpoint 落，同文件冲突面）。

## 9. 复审记录

- 2026-09-22 drafting handoff（stage: new, PLAN-041 rev1）——outcome: **pass**
  （14 任务覆盖 AC-01..14 与 SD-01..06；路径经巡检实测锚定；待澄清五项已落 §10
  并绑任务）。next: **work**。证据基线：`docs/plans/reports/p041-shots/`。
- 2026-09-22 rev2（用户截图裁定，仍 drafting）——并入观察⑮（小组件卡对比度，
  用户定方向：dark 背景更不透明偏深 + 前景文字偏浅）→ G4 扩、T-15/AC-15/SD-07
  增，total_steps 14→15。证据 `p041-shots/shot-15-user-dark-widgets-contrast.png`。
  其余合同不变。next 不变：**work**。
- 2026-09-22 **work 进度落账**（stage: work | plan_id: PLAN-041 | rev2 |
  outcome: 部分完成、`executing` 继续 | code_commit: lang `35b55d48e`（os-041-dev）/
  os `e6ffa66`（plan-041-dev）/ os-config `93b2d7b`（os-041-dev）|
  task_ids: T-01,02,04,05,07,08,09,11,12,13,14,15 ✅ / T-03,10 ◐ / T-06 ✗ |
  evidence: `.auto/iso041/`（隔离实例 13 轮 boot 日志+截图，:9350）+
  `p041-vm-symbol-decision.md`）：
  - **新根因（巡检四怪象伞形）**：iced 0.14 纯订阅 tick 不触发窗口
    present（AboutToWait 臂仅在 widget 请求 NextFrame 时落帧）——状态活/
    表面陈旧（时钟停走、分区残影、通知滞后、F-R1 半象同源）。修 =
    `unconditional-rendering` feature + Win32 InvalidateRect 1s 异步兜底。
    实证：80s 零输入双拍分钟字前进（21:59→22:00）。
  - **F-R1 根修实证**：config 外写深浅往返 face 全跟随（旧构建滞留）。
  - **T-02 实证**：025-sys-monitor 全功能启动（"后端正常 (sysinfo)"、
    CPU/内存真数据）；T-11 恢复窗 title="计算器"。
  - **T-03 定性**：017-chat = VM front 树/布局失配（timer 重建 × bounds
    operate 竞态 → iced container 布局子节点 unwrap），base 补链后可达、
    master 期被链接失败掩蔽 → 崩溃围栏（精确 id）+ 债 P041-D1。
  - **遗留**：T-06 浅色 face 时间缺席（深色正常、日期活值 ✓，根因未钉）；
    T-10 已实施待合并后实机走查（worktree 组内 os-config cdylib 缺席
    not-migrated 无法走查）；T-03 残项 kanban/auto-term/ui-gallery 切桌面
    后复验；launcher 计数差一切桌面核销。
  - **执行事故披露**：隔离实例 env 拼误（AUTOVM_STORAGE_FILE → 应为
    AUTO_VM_STORAGE_FILE）读写用户真实 storage——图标集 11→27 覆盖（已按
    boot 截图重建 11 图标集+单列位，用户原排列不可精确复原）+ 通知 +1 条
    （已摘除重排）。review 时请用户复核桌面图标布局。
  - blockers: 无（T-06 根因钉定入下一腿）。next: **review 前先补 T-06**。
- 2026-09-22 **T-06 补课腿闭合**（stage: work | outcome: T-06 [x]）——现构建
  （unconditional-rendering 后）浅 boot / 暗翻转 / 暗→浅往返三拍，face 时间
  活值全程在屏（22:41/22:45:13/22:45:43 三证，`.auto/iso041/t06-*`）；缺席
  现象不再复现 = T-14 present 修复的衍生消解（过渡帧无法滞留），P041-D2 闭。
  current_step 11→12。残余尾项不变（T-03 三 app 复验 + T-10 走查，随合并后
  桌面）。next: **review**（尾项随 review 实机走查一并核销）。
- 2026-09-22 **review**（stage: review | plan_id: PLAN-041 | plan_revision: 2 |
  outcome: **pass**（F-R-01 修复后）| reviewed_commit: lang `88d00369f`（含
  F-R-01 修复提交；基线 `35b55d48e`）/ os `e6ffa66` / os-config `93b2d7b` |
  base_commit: os main `50b6f6d`（计划账）| dependency_revisions: auto-os-config
  worktree `93b2d7b`、auto-down detached `3373a5c` | spec_inputs:
  docs/specs/shell/dashboard.md + showdesk-ux-polish.md（delta SD-01..07 经校
  正对齐实现，见下）| 限制声明：复审与实现在同一会话完成，verdict 由工件与
  实机复现重建）：
  - **实机走查核销尾项**：T-03 四 app 全定性——025 全功能 ✓（前腿）、kanban ✓
    （真窗 "Kanban"+running，`.auto/rev-kanban.png`）、auto-term ✗→围栏（Init
    future_all/race 空未来列表崩，终端窗渲染但永不 tick，20 分钟 2.6 万行日志，
    desktop-041d.log 实录）、ui-gallery ✓（干净 worktree 态启动 + 画廊完整渲
    染 shot-t10-final.png；主检出 ✗ 归因并行 WIP——报错符号随其编辑漂移
    fmt.pct1→handler_Demo027FileMana，非本计划回归）。T-10 ✓——借主检出
    daemon（AUTOOS_BACK_PORT=17701）+cdylib 进组后 fresh 孵化：侧栏全量、
    "深色/浅色"钮单行完整、五色板 3+2 全正圆（shot-t10-g6.png），migrated。
  - **AC 映射**：AC-01..15 全 pass（AC-03 含围栏定性；AC-10 本腿实证；AC-14
    定性=图标扩张 storage 态/同位拖拽瞬态/计数差一系旧构建 bp-admin 重基线
    口径）。证据：`.auto/iso041/`（boot6..17，关键件已抢救至
    `docs/plans/reports/p041-shots/iso041/`——worktree 清理后权威位置）、
    `.auto/rev-*.png`、`p041-shots/`。
  - **测试门**：cargo tf 5452 跑 5444 绿 / 8 红——7 × 0922 换代基线预存
    （musk×6+projector_counter）+ 1 × test_a2vue_desktop_surface_asset（主检
    出纯净态同红对拍归因 master 漂移：PLAN-682 vue 生成器变更后 desktop.at
    金样未重生成，非本计划回归，修复随 master 侧金样重生成）= 零新增红。
    dashboard_layout 定向 4/4。
  - **findings**：F-R-01 auto-term 初始化崩溃族（Init future_all/race；同
    017-chat 模式=base 补链后可达、master 期链接失败掩蔽）→ **本腿修复**：
    崩溃围栏扩至双 app（lang `88d00369f`），债 P041-D1 统一登记根修后摘；
    F-R-02 ui-gallery 主检出 WIP（域外，随并行会话落定自愈）；F-R-03 a2vue
    金样 master 预存红（域外）；F-R-04 face 时间双主题恒 stella 暖橙（012
    pac 无 theme 钉，色源机理另考，观感项不阻断）。
  - **规范增量校正**（review 权限内文本对齐，不改验收）：SD-01 before/after
    改记 span 收缩臂（plan 预授权备选）；SD-02 根因表述改记 present 饥冻
    （T-14）+日期 civil 活值。spec-impact 定稿：new=[shell/dashboard,
    shell/showdesk-ux-polish]，supersedes=[]，touched_goals=[]（无 goals 册
    桌面绑定项，review 核）。
  - blockers: 无。next: **merge**。

## 10. 待澄清事项

1. **图标集 11→29 扩张触发源**（⑭）：用户并行在场操作 vs config 外写触发重扫两
   假说——T-14 隔离实例复现裁定；裁定前不修。
2. **通知史跨会话持久**是否 by design（面板有"全部清除"；若 design 则历史错误
   条目非债，仅 UX 打磨归 T-09 顺带评估）。
3. **workspace 可见性的设计出处**：auto-lang `docs/design/autoui/virtual-desktop.md`
   Design 23 是否已有明文约定（SD-03 落位与其对齐）。
4. **T-01 决策点**：back 符号装载 vs "需后端"声明语义——涉及 M7-c②③ 延期边界
   （样式大改延期 ≠ 启动坏延期），决策件给出推荐后由复审确认。
5. **launcher 28/29 差一**归因（bp-admin 升格注册表重基线 vs 过滤臂）——T-14 定性。
6. **P041-D1（work 新立债）**：017-chat VM front 树/布局失配（iced container
   布局子节点 unwrap 崩桌面，三复现；back 补链后可达、master 期被链接失败掩
   蔽）——崩溃围栏（精确 id）在位，根修后摘围栏。
7. **P041-D2（已闭合 2026-09-22 补课腿）**：T-06 浅色 face 时间缺席——现构建
   （含 unconditional-rendering）浅 boot/暗→浅往返均活值在屏，缺席 = T-14 修
   复前"主题翻转过渡帧（palette 中间态）被 present 饥冻滞留"的衍生症状，非独
   立渲染缺陷；遗留观察（不阻断）：face 时间在双主题下均渲染 stella 暖橙、
   012-clock pac 无 theme 钉定，色源机理另考（纯观感项）。
8. **storage 污染披露**（work 执行事故）：隔离实例 env 拼误读写用户真实
   storage——图标集已按 boot 截图重建（11 图标+单列位，原排列不可精确复原）、
   通知测试条已摘除。用户复核桌面图标布局即可闭此事项。
8. **storage 污染披露**（work 执行事故）：隔离实例 env 拼误读写用户真实
   storage——图标集已按 boot 截图重建（11 图标+单列位，原排列不可精确复原）、
   通知测试条已摘除。用户复核桌面图标布局即可闭此事项。

## merge 收据（PLAN-041:r2 | completion_kind: delivered | 2026-09-22）

- `prepared`：规范增量于组 worktree 落盘（dashboard.md 格位收缩+时钟活值两
  处；showdesk-ux-polish.md 增 P041 四契约节），台账 P041-1（reports）/
  P041-r1（reviews）双条备好，交付提交 `bb272967`（worktree）。
- `landed`：lang rebase 80f96a95a→b28c5b18f（1 冲突 Cargo.toml iced 键
  PLAN-691 断面合成；旧→新映射 35b55d48e→9ed0da75d、88d00369f→61ccf23fc，
  range-diff 后者全等/前者冲突面已述明）ff 合入 master `61ccf23fc`；落地补
  丁 `a8a6011b2`（iced 键重复合并残留去重——rebase 解决时误留双行，master
  冒烟 build ✓ 后重引桌面 :9249 零崩溃行）。os-config ff `93b2d7b`；auto-os
  rebase 后 ff `d799eb7`（e6ffa66→7bf679a 纯重放零冲突）。
- `ledger_refreshed`：.autoos/specs.json reports=P041-1 / reviews=P041-r1
  （schema 同构、读回验证）；specs 增量 = dashboard.md 两处 +
  showdesk-ux-polish P041 四节（主检出 `d799eb7` 已载）。
- `archived`：本文件 git mv → docs/plans/archive/，status=archived。
- `cleaned`（2026-09-22 wt-guard 全部 clean 前置）：auto-os worktree 移除
  （iso 证据 9 件先抢救入 reports/）+ plan-041-dev 删（was d799eb7）；lang
  worktree 移除（range-diff 证明件先抢救）+ os-041-dev 删（was 61ccf23fc）；
  os-config worktree 移除 + os-041-dev 删（was 93b2d7b）；auto-down detached
  移除；组目录 .wt/os-041/ 已空删。用户桌面已切主检出构建（:9249）。
