---
plan_id: PLAN-014
status: executing              # drafting → executing → execution_done → reviewed → archived（2026-09-17 review needs_fix：F-01..F-04 复开 T-02/T-05/T-09/T-10）
feature_name: shell-ux-polish-v2
author: [zhaopuming]
created_at: 2026-09-14
updated_at: 2026-09-17
plan_revision: 2              # rev1 = 2026-09-14 起草（14 项 W-01..W-14）；rev2 = 2026-09-17 重基线修订（见 §0 disposition 表）

# /auto-plan:review 结束时填写：
supersedes_spec_components: []
new_spec_components: [docs/specs/shell/showdesk-ux-polish.md]
touched_goals: []             # 引用 docs/specs/goals.md 的 GOAL-NNN

affects: [shell/shell.at, shell/desktop.at, shell/switcher.at, shell/notification_center.at,
          auto-lang/schema/projection-protocol-v1.md,
          auto-lang 宿主注入/通知臂（session.rs / renderer.rs / notify 投影面）,
          docs/specs/shell/showdesk-ux-polish.md（新增）]
current_step: 6
total_steps: 10
---

# [PLAN-014] shell-ux-polish-v2（rev2）

## 0. 变更摘要

虚拟桌面 shell UX 打磨第二批。rev1 起草于 2026-09-14（14 项 W-01..W-14），
其后同日至 09-17 间大量计划落地（PLAN-012/015/016/018/019/022/023 全部
merge 归档），rev1 的基线假设（012 未合并、任务栏 lucide 字形、协议 v1.6
空位、桌面图标紧凑版式）全部失效。2026-09-17 会话逐项复核实码后用户裁定
rev2 disposition，本计划按其重基线。

### rev1 → rev2 disposition 表（用户裁定 2026-09-17）

| rev1 项 | 内容 | rev2 处置 | 依据（实码/计划实证） |
|---|---|---|---|
| W-01 | 死消息清理 + 头注补 color | **保留** | 五组死消息原样在 shell.at（:58-61,:81 声明，:571-624 handler）；`LayoutFree` 的按钮已被 526 T35 退役，死实；desktop.at 头注仍缺 `color` |
| W-02 | `__desktop_cmd` 追加语义 | **保留**（含 Q1 spike 退化路径） | 四 pack 写点仍全部直接覆盖赋值 |
| W-03 | 任务栏字形区分 | **删除——被取代** | 2026-09-15 用户裁定任务栏全量换 iconfile 位图（launcher/desktop-switch/layout-grid/layout-stack/notification/config/shutdown，shell.at:172-497），字形混淆问题不复存在 |
| W-04 | 桌面选中态 + 启动中反馈 | **保留，mock 重画** | `sel_id`/`launching` 未落；但 022 已重做图标（48px 满幅位图/80px 格/PITCH 88/列主序），rev1 mock「紧凑左上」版式作废，语义保留视觉参数按 022 网格重定 |
| W-05 | 空白菜单扩充 + hidden 去重 | **拆分保留**：hidden 去重 + 恢复默认图标；**砍「显示桌面」** | MenuRemove 仍裸追加（desktop.at:416-424）；空白菜单已被 019 重构（更换壁纸…）；「显示桌面」与 019 负一屏语义冲突且入口同为一个空白菜单——由 019 接管（019 待澄清 #1 就此清偿；spec 归属 docs/specs/shell/showdesk-wallpaper.md） |
| W-06 | 时钟两行 + 日期 + 点击跳日历 | **保留两行+日期；砍点击臂** | 仍单行 `__wm_clock`（shell.at:528）；注册表实查无日历 app（apps/ 全量 pac.at 零 calendar 命中，003-clock 四 tab 无日历）——点击无落点，日历 app 缺位记 §10-Q3 |
| W-07 | 未读角标圆形化 | **保留** | 仍裸红字 `text-xs text-error`（shell.at:470-472）；铃铛已换 notification 位图，badge 视觉参数随位图语境微调 |
| W-08 | 通知跳来源 | **保留** | `__wm_notes` 仍 `{id,kind,msg,at}` 四字段（schema v1.2 行），行不可点；B12 规避期平行列表同型加 `note_apps` |
| W-09 | scrim 外点关闭核对 | **删除——已消化** | 012 已落；其后 016（popover trigger/content 拆分）、019（Esc 链）多轮重构，rev1 的「核对 012 实现」对象已不存在 |
| W-10 | switcher 预选第 2 项 + hover | **保留** | `RebuildMru` 仍 `sel = 0`（switcher.at:132）、无 onmouseenter；022 给切换器加的是预览刷新+新建桌面入口，未触此二点 |
| W-11 | 窗口标题兜底 | **删除——被 015 覆盖** | PLAN-015 delivered：pac.at `title`/`title_zh` 展示名链（title_zh→title→name→目录名）；022 又退役 DualApp 验收窗 |
| W-12 | 协议汇总升版 | **保留，改登记 v1.8** | v1.6/v1.7 编号已被 012（dock 面）/016+019（open_with、wallpaper 动词）占用，现协议 v1.7；本计划字段面内容（除编号外）均未落 |
| W-13 | 托盘占位撤除 | **删除——已消化** | 空 row 已转为 Plan 497 G2「状态图标挂载点容器」（shell.at:520-526 有注释有归属），不再是真空占位缺陷 |
| W-14 | B12 债务清偿（auto-lang VM） | **移出——转独立 auto-lang 计划挂账** | 平行列表规避仍在（switcher/notification_center 实码注释自证）；auto-lang KNOWN-DEBT 的「B12」条目系另案（编码伪证）；维持 rev1 Q3 倾向另立，本计划只保 `note_apps` 消参挂账（§10-Q5） |

**净效果**：保留 9 项（W-01/02/04/05'/06'/07/08/10/12'），删除 5 项
（W-03/09/11/13 被取代或消化；W-14 转独立挂账）；协议增量零新动词
（`show_desktop` 随 W-05 砍除，v1.8 纯字段 + 总线语义）。

mock 现状（`docs/plans/evidence/014/` 四张）：panels 两张（⑧⑨⑩语义）仍可
用；desktop 两张版式作废（022 后图标网格为 48px 位图/80px 格/列主序），
选中态/启动反馈语义保留，rev2 mock 于 T-05 执行期重画入
`evidence/014/rev2/`。双主题基调（stella light token 映射）不变。

跨仓：主导仓 = 本仓（auto-os shell pack + spec）；auto-lang 承载协议
schema v1.8 与宿主注入/通知臂（Category A 门档：不在 auto-lang 跑
`cargo t`/`docs_gen`）。shell pack 改动后跑 `scripts/shell-pack-sync.py`
对齐 auto-lang 内嵌快照（hash-lock 契约）。工作按 AGENTS.md 走 worktree
组 `.wt/os-014/`（移除前过 wt-guard）。

## 1. 目标

1. **G1（清理）** shell pack 无死消息/死 handler；`__desktop_cmd` 多命令
   同排空周期不丢失（或 spike 定案的单槽约束明确化）。
2. **G2（桌面）** 桌面图标单击有选中反馈、双击启动有进行中反馈（022 网格
   形态下）；hidden 串写入去重；空白菜单具备恢复默认图标入口。
3. **G3（任务栏）** 时钟两行（HH:MM + `M月D日 周X`）分钟/日期独立刷新；
   未读数为圆形角标压 notification 位图右上角。
4. **G4（面板）** 通知整行点击跳来源 app（未运行启动/运行聚焦两臂）；
   switcher 召唤预选 MRU 第 2 项、行 hover 跟随选中且与键盘推进互洽。
5. **G5（协议）** 投影协议升 v1.8（字段/总线语义增量一次落），shell pack
   hash-lock 同步零 diff，`__desktop_icons` color 漏记补审计。

## 2. 架构方案

延续 WM-as-app / shell=用户态定案（Design 23 R1 + I7-I9）：全部 shell 侧
改动仍是 pack 内数据驱动（宿主注入、shell 发动词），不引入 shell 侧几何
操作。**rev2 零词表增量**——新增注入一律走协议字段扩展，由宿主解析注入，
保持 I9 单一事实。

关键复用：

- `__wm_running`（协议 v1 即有，`",id1,id2,"` 派生串，schema:35）已存在
  ——W-04 只是把注入面扩到 desktop 层（现仅注入 shell 层），非新字段。
- `__wm_notes` 消费走 B12 规避期平行列表（`note_ids/kinds/msg/ats` +
  `RebuildNotes`，schema:36 在案）——W-08 同型加 `note_apps`，W-14 落地后
  消参（挂账不阻塞）。
- 空白菜单/负一屏/更换壁纸臂为 019 既有结构，W-05' 只在其上追加项，不动
  019 组合簿记。

## 3. 技术栈

- shell pack：auto-os `shell/*.at`（数据驱动 DSL，双端同源）。
- auto-lang：`schema/projection-protocol-v1.md` + 宿主注入/通知臂
  （Category A 门档内，不跑 `cargo t`）。
- 同步：`scripts/shell-pack-sync.py`（hash-lock）。
- 验证：autoui-verifier 双轨（Vue + VM）× 双主题（stella light token 映射）
  + `scripts/desktop.{sh,ps1}` 实机装配冒烟。

## 4. 需求分析与背景调查

### rev1 基线（2026-09-14 评审，历史保留）

代码+设计评审产出两轮建议（视觉缺陷/交互闭环/信息架构/协议债），实证包
括：死消息五组、`__desktop_cmd` 单槽覆盖、字形同形双钮、裸红字 badge、
通知无来源字段、switcher sel 复位 0、B12 平行列表 ~120 行重复、
`__desktop_icons` color 漏记。详见 rev1（git 历史）与当日评审记录。

### rev2 重核（2026-09-17，逐项实码验证）

- **死消息**：`shell/shell.at:58-61`（`LayoutFree`/`WorkspaceNext`/
  `WorkspaceClose` 声明）、`:81`（`HoverWs/HoverWsEnd`）；handler 在
  `:571`/`:591`/`:594`/`:623-624`。pack 内 grep 无发送者（仅 handler 行
  自身命中）；宿主派发路径 T-01 前置核查。
- **`__desktop_cmd`**：四 pack 写点全部直接赋值（`.__desktop_cmd = "…"`），
  协议头「多条以 \n 连接」承诺与实现不符照旧。
- **桌面**：desktop.at 无 `sel_id`/`launching`；`__wm_running` 仅注入
  shell 层（shell.at:91）。022 网格形态：48px 满幅位图、格 80px、PITCH 88、
  列主序、拖拽阈值/幽灵修复在案——W-04 视觉参数以此为基。
- **hidden 去重**：`MenuRemove`（desktop.at:416-424）仍逗号裸追加；
  空白菜单现有「更换壁纸…」（019）与「显示设置」（旧通道），无恢复入口。
- **时钟**：单行 `text .__wm_clock`（shell.at:528），无 `__wm_date`。
- **badge**：裸 `text-xs text-error font-semibold`（shell.at:470-472），
  非圆形、无 9+ 截断；铃铛已位图化（`iconfile:notification`）。
- **通知**：`__wm_notes` 四字段无来源 app（schema:36）；平行列表
  `note_ids/kinds/msg/ats` 消费在案。
- **switcher**：`RebuildMru` 置 `sel = 0`（switcher.at:132），行无 hover。
- **协议**：现 v1.7（016 open_with + 019 wallpaper 组）；oncontextmenu
  坐标事件臂 schema 无登记，`__desktop_cursor_x/y` 宿主泵仍在
  （desktop.at:80-81，且 022 拖拽幽灵臂依赖该泵——坐标臂改造见 §10-Q2）。
- **注册表**：apps/ 全量 pac.at 无 calendar——W-06 点击臂无落点，砍除。

### 授权记录

- 2026-09-17 用户批准 rev2 disposition（本会话）：删 W-03/09/11/13、
  W-05 砍显示桌面、W-12 改 v1.8、W-04 mock 重对齐 022、W-06 核实日历入口
  （结果：无，砍点击臂）、重写开工核对；W-14 转独立挂账。
- 允许仓/动作：auto-os `shell/` + `docs/specs/shell/`（新增 spec）、
  `scripts/shell-pack-sync.py`；auto-lang schema + 宿主臂（Category A）。
- 无特别预算/自动续跑限制授权；执行仍需走 `/auto-plan:work`。

## 5. 详细设计

### W-01 死消息清理（shell.at）

删除无发送者的 msg + handler：`LayoutFree`、`WorkspaceNext`、
`WorkspaceClose`、`HoverWs`/`HoverWsEnd` 五组。**前置**（T-01）：grep
auto-lang 宿主消息派发确认无热键直投路径（宿主热键走直写状态/直发命令，
预判全死；若有投递路径则保留该条并注释）。顺手修 desktop.at 头注补
`__desktop_icons[].color` 字段登记（v1.4 漏记，与 W-12' schema 审计呼应）。

### W-02 `__desktop_cmd` 追加语义（四 pack）

写总线 handler 统一追加形态：
```
if .__desktop_cmd != "" { .__desktop_cmd = .__desktop_cmd + "\n" }
.__desktop_cmd = .__desktop_cmd + "<verb>\t<arg>"
```
涉及 shell.at / desktop.at / switcher.at / notification_center.at 全部写
点。若追加形态实测触发宿主排空指纹问题（多行命令序），退化为：修注释为
「单槽，每周期至多一条，重复动作合并」——T-03 开工 spike 定案（§10-Q1）。

### W-04 桌面选中态 + 启动中反馈（desktop.at + 宿主注入面）

- `sel_id str`：图标 mouse-area `onclick` → `.Select(e.id)`，选中格包
  白/10 圆角高亮块；`BlankPress` 清空。**与 022 拖拽互洽**：onclick 选中
  不得破坏 022 拖拽阈值判定（单击 vs 拖拽以既有阈值臂为准，选中态只在
  非拖拽路径置位）。
- 启动中：`ActivateApp` 后置 `launching = id`（chip 半透明 + 右上角灰点）；
  宿主把既有 `__wm_running` 派生串扩注到 desktop 层，handler 读入：running
  含 launching id 即清 launching 态。
- 防御：launching 无 ack 超时自愈——`Init`/重注入时 launching 与 running
  求差清残（宿主启动失败场景 `__wm_running` 不含该 id）。
- 视觉参数执行期按 022 网格实测定稿（格 80px/图标 48px 位图/列主序），
  mock 重画入 `evidence/014/rev2/`。

### W-05' hidden 去重 + 恢复默认图标（desktop.at）

- `MenuRemove` 追加前去重（`"," + id + ","` 域串包裹 contains 判断）。
- 空白菜单（019 后形态）追加**恢复默认图标**项 =
  `storage.set("shell.desktop.hidden","")` + 本地 `__desktop_hidden` 清空；
  生效路径按现机制（boot 合并臂排除 hidden——重启全量回；若宿主支持
  即时重注入则即时回，T-04 实测定案，验收以重启全量回为准）。
- **不加「显示桌面」**——负一屏语义归 019（`ShowDesktopToggle` sliver +
  空白菜单簿记已在其域内）。

### W-06' 时钟两行 + 日期注入（shell.at + 宿主）

- 宿主注入扩为两字段：分钟变化写 `__wm_clock`（`"HH:MM"`，不变），日期
  变化写 `__wm_date`（`"M月D日 周X"`）——两字段独立脏帧，稳态零重建口径
  不变。
- shell.at 托盘组时钟区（现单行 text，:528）改两行 col：上行 HH:MM、
  下行日期；**无点击臂**（注册表无日历 app，§10-Q3）。

### W-07 未读角标圆形化（shell.at）

badge 改绝对定位圆形：`bg-error text-primary-foreground rounded-full`
16px 圆压 notification 位图钮 `-top-1 -right-1`；数字 >9 显示 "9+"。
iced/Vue 双端同 class（505 B1 数据驱动口径）；浅色主题角标 #EF4444 不变。

### W-08 通知跳来源（协议 v1.8 + notification_center.at + 宿主）

- `notify` 落库时记录来源 app id；`__wm_notes` 增 `app` 字段（合同面）；
  B12 规避期平行列表同型加 `note_apps`（W-14 落地后消参，挂账 §10-Q5）。
- 通知行 onclick → `activate\t<r.app>`（宿主臂：未运行启动/运行聚焦，
  复用既有 activate 两臂零新动词）。

### W-10 switcher 预选第 2 项 + hover 跟随（switcher.at）

- `RebuildMru`：`sel = nres > 1 ? 1 : 0`（Alt-Tab 惯例；MRU 第 1 项 =
  当前聚焦窗，Enter 聚焦 = 无操作）。
- 行 mouse-area 增 `onmouseenter: .HoverSel(r.i)` → `sel = i`；键盘
  Advance/Back 从当前 sel 继续（天然满足，无需特判）。

### W-12' 协议 v1.8 汇总（auto-lang schema + 宿主 + hash-lock）

一次升版承载本计划全部增量（**纯字段 + 总线语义，零新动词**）：

- 入向字段：`__wm_running` 注入面扩 desktop 层（字段本身 v1 已有，登记
  注入面扩展）；`__wm_date` 新字段；`__wm_notes[].app` / `note_apps`。
- 总线语义：`__desktop_cmd` 追加语义（W-02 spike 定案结果）写入协议。
- 字段表审计：`__desktop_icons[].color` 补登记（v1.4 漏记）。
- oncontextmenu 坐标事件臂：**默认后置**（§10-Q2——022 拖拽幽灵臂依赖
  `__desktop_cursor_x/y` 泵，改造需先裁定交互归属），不在 v1.8 强捆。
- `scripts/shell-pack-sync.py` 同步 hash-lock 零 diff。

### 规范增量

| delta_id | add/modify/retire | 目标 | before/after | rationale | acceptance |
|---|---|---|---|---|---|
| SD-01 | add | docs/specs/shell/showdesk-ux-polish.md（新） | 无 → 桌面选中/启动反馈语义、hidden 去重与恢复、时钟/日期注入面、badge 形态、通知来源跳转、switcher 预选/hover 的行为合同 | rev2 保留九项 UX 行为此前无 spec 归属；沿 019/022 先例一计划一 spec | AC-02..04 |
| SD-02 | add | auto-lang schema/projection-protocol-v1.md §6 v1.8 节（跨仓，review 定稿路径） | v1.7 → v1.8：`__wm_running` 注入面扩展、`__wm_date`、`__wm_notes[].app`/`note_apps`、`__desktop_cmd` 追加语义、color 补记 | 承载 G5；字段表与 pack model 声明逐一比对 | AC-05 |

## 6. 测试设计

- **双端纪律**：每个改动的验收 = auto-lang autoui-verifier 双轨（Vue +
  VM）× **深/浅双主题**截图对照 mock（desktop 组按 rev2 新 mock，
  `evidence/014/rev2/`；panels 组沿旧 mock 语义）+ 实机装配冒烟
  （`bash scripts/desktop.sh` / `desktop.ps1`）。浅色一律按 stella light
  token 验收（选中块 #E3DDD1 半透明底 / 预选行 #6466F1 底白字 / hover 8%
  黑底 / 角标 #EF4444 与语义色不变）。
- **宿主改动**（`__wm_date` 注入 / `__wm_running` desktop 扩面 / notify
  来源记录）：auto-lang 侧 hand 测试或既有宿主冒烟脚本；不在 auto-lang 跑
  `cargo t`（Category A 门档）。
- **W-02**：同周期双命令场景（双击 + 右键快连击）宿主排空日志验证两命令
  均执行；或 spike 退化定案记录。
- **W-04**：启动失败残态自愈用例（running 不含 launching id 时清残）+
  与 022 拖拽互洽（选中不误触拖拽、拖拽不误置选中）。
- **W-05'**：重复移除同 id 不产生重份；恢复默认图标后重启全量回。
- **W-06'**：跨天日期刷新用例；分钟/日期独立脏帧（另一字段不变时稳态
  零重建口径维持）。
- **W-08**：未运行启动/运行聚焦两臂各一用例。
- **回归面**：dock 三动作菜单、桌面双击激活、**022 拖拽行为族**、
  **019 壁纸 picker 与负一屏**（desktop.at 动过的邻接面）、热键召唤三
  overlay、通知 badge 计数——每 W 完成后过一遍。

## 7. 验收标准

1. **AC-01（G1）**：grep shell pack 无死消息/死 handler；同排空周期双
   命令零丢失（或 spike 单槽约束定案记录在案）。
2. **AC-02（G2）**：选中态/启动反馈 rev2 mock 对拍通过（双轨双主题）；
   启动失败残态自愈；hidden 去重（重复移除同 id 无重份）；恢复默认图标
   重启后全量回；022 拖拽族回归零红。
3. **AC-03（G3）**：时钟两行显示、分钟/日期独立刷新（跨天用例过）；
   未读数为圆形角标压位图钮右上、9+ 截断。
4. **AC-04（G4）**：通知行点击跳来源两臂（未运行启动/运行聚焦）；外点
   关两面板回归绿；switcher 召唤预选第 2 项、Enter 即切最近其他窗、
   hover 跟随与键盘推进互洽。
5. **AC-05（G5）**：schema v1.8 节齐四增量（running 扩面/date/notes.app/
   cmd 语义）+ color 补记；`shell-pack-sync.py` hash 对齐零 diff；字段表
   与 pack model 声明逐一比对记录。

## 8. 执行步骤

（原子任务：精确文件 + 确切操作 + 验证命令；每步完成后追加 [✅ 已完成]
一行证据。**前置基线**：012/015/016/018/019/022/023 已全部 merge 归档，
无在飞 auto-os 计划 worktree 冲突面；工作在 `.wt/os-014/auto-os` +
`.wt/os-014/auto-lang` 组进行（AGENTS.md 布局，移除前过 wt-guard）。）

1. **[x] T-01 开工核对（W-00'）**：grep auto-lang 宿主确认 W-01 五消息无宿主
   投递路径；确认协议头现版本 v1.7 与 §6 节结构（v1.8 落点）；从
   docs/specs/shell/showdesk-icons.md + desktop.at 抄录 022 网格常量
   （格/距/图标径）作 W-04 mock 参数。证据存 `evidence/014/rev2/`。
   [✅ 已完成] 2026-09-17 五消息全死实证（热键直发 WmCommand/命令总线变
   体/注释命中，pack 内零发送者；ws_hover 属死组随删）；协议 v1.7 :1-331
   v1.8 落点定位；022 网格常量抄录（cols 8/gap 8/格 80×72/图标 48 满幅/
   行距 80/列主序/6px 阈值）；附带：parse_records 多记录排空在案（Q1 倾
   向追加可行）、__desktop_icons.color 与 __desktop_cells.full 漏记双实
   证、__wm_running 注入点 renderer.rs:12840。证据
   `evidence/014/rev2/T01-kaiming-hecha.md`（worktree plan-014-dev）。
2. **[ ] T-02 W-01**（review needs_fix 复开：F-02 fixture 修复）：删 shell.at 五组死 msg+handler；desktop.at 头注补
   color。验证：pack 冒烟（desktop.sh 起桌面，dock/热键回归）+ grep 零残留。
   [✅ 已完成] 2026-09-17 五组全删（ws_hover 随死组退役）；grep 零残留；
   pack 编译冒烟 `shell_packs_compile` PASS（真管线 compile+Init）。
   实机 dock/热键回归 → review 门（双轨截图通道）。
3. **[x] T-03 W-02**：spike 宿主排空指纹（10 分钟盒）→ 四 pack 写点统一追加
   语义（或按 Q1 退化注释修正）。验证：同周期双命令日志。
   [✅ 已完成] spike 定案=追加语义可行（parse_records 按 \n/REC_SEP 切分
   session.rs:1645 + drain 每 update 周期读+清 renderer.rs:9652）；Q1 就此
   清偿（退化路径未动用）。实现调整：两行内联形态 → **SendCmd(rec) 单点
   handler**（四 pack 统一，语义等价、单点维护，32 写点全改）；既有
   `parse_records("show_desktop\nwallpaper_pick")` 双记录金样 PASS + scope
   套件 34/34。
4. **[x] T-04 W-05'**：MenuRemove 去重；空白菜单加「恢复默认图标」。验证：
   去重用例 + 重启全量回 + 019 菜单臂回归（更换壁纸/负一屏）。
   [✅ 已完成] 去重 = 逗号域串包裹 contains（handler 侧合法，012 v1.6
   在案）+ `ResetIconsBlank` = hidden 单源清空 + `refresh_desktop_icons`
   即时重注入（v1.6 动词，storage 已清故即时臂=重启全量回同效）；019 菜单
   臂（更换壁纸/显示设置）未动。a2vue 金样重生成含该按钮（vue 产物可见）。
   重启全量回/菜单臂实机回归 → review 门。
5. **[ ] T-05 W-04**（review needs_fix 复开：F-03 hover 提取回归）：desktop.at `sel_id`/选中块/`launching`；宿主 `__wm_running`
   扩注 desktop 层；rev2 mock 重画并对拍；自愈用例 + 拖拽互洽用例。
   [✅ 已完成] sel_id（IconPress 置位/BlankPress 清空，拖拽阈值臂不受
   影响——选中只置本地态）+ launching（ActivateApp 置位，绝对定位灰点
   + 半透明；class.rs absolute/-top/-right 白名单在案）+ 宿主
   `__wm_running` desktop 扩注 + `RunningSync` 召唤（写状态不触发 handler
   律）+ Init/重注入求差自愈；rev2 mock 双主题重画入
   `evidence/014/rev2/`（022 常量：cols 8/gap 8/格 80×72/图标 48 满幅）。
   对拍/拖拽互洽/自愈实机用例 → review 门（宿主投影/scoped 套件已绿）。
6. **[x] T-06 W-06'**：宿主 `__wm_date` 注入；shell.at 时钟两行（无点击臂）。
   验证：跨天刷新用例 + 脏帧独立性。
   [✅ 已完成] `update_shell_clock` 双字段独立变化才写（date_text/clock_text
   各自去重，独立脏帧——另一字段不变时零写入零 dirty）；中文周几宿主
   格式化（chrono Datelike）；shell.at 两行 col 无点击臂。脏帧独立性=
   代码结构保证（两字段各自 changed 位）；跨天实机用例 → review 门。
7. **[x] T-07 W-07**：badge 圆形绝对定位 + 9+ 截断。验证：双轨双主题截图。
   [✅ 已完成] 实现调整：9+ 截断宿主派生 `__wm_notes_badge`（>9→"9+"/
   0→""——.at 视图无数值比较原语，I9 单点；`__wm_notes_unread` 计数合同
   不变）；badge = relative col + absolute 16px 圆压 notification 位图
   右上（-top-1 -right-1，双端同 class）。双轨双主题截图 → review 门。
8. **[x] T-08 W-08**：宿主 notify 记录来源 app；`__wm_notes.app` +
   `note_apps`；notification_center.at 行 onclick activate。验证：两臂
   用例。
   [✅ 已完成] notify_source 分段归因（联合排空泵注册表窗段按 app 分段
   执行——段前置置 source，命令顺序与原扁平 concat 逐一相同，Shutdown
   短路按段传递；特权段恒 None）；NotificationEntry.app + persist/
   restore（旧库缺键→""）+ 召唤/活更新双注入点 note_apps；行
   `OpenSource` = activate 两臂复用（app=="" no-op，跳转即收面板）。
   发现并修复：召唤注入点漏 note_apps → RebuildNotes IndexError
   （note_apps 域外防御读 + 双注入点补齐）；`notif_center_summon_headless`
   等全绿。两臂实机用例 → review 门。
9. **[ ] T-09 W-10**（review needs_fix 复开：F-04 fixture 断言更新）：switcher.at sel 预选第 2 项 + 行 onmouseenter 跟随。
   验证：键盘/鼠标混合推进用例。
   [✅ 已完成] RebuildMru `nres>1 → sel=1`（Alt-Tab 惯例）+ HoverSel(int)
   行悬停置 sel（visible 门控；键盘 Advance/Back 从当前 sel 天然互洽）。
   pack 编译冒烟 PASS；混合推进实机用例 → review 门。
10. **[ ] T-10 W-12'**（review needs_fix 复开：F-01 SD-01 spec 撰写）：schema §6 v1.8 节（四增量 + color 补记；oncontextmenu
    坐标臂按 Q2 裁定记录）+ 宿主注入面收口 + `shell-pack-sync.py`
    hash 对齐。验证：字段表与 model 声明逐一比对记录。
    [✅ 已完成] schema v1.8（§6 节 + §2 行：running 扩面/date/notes.app+
    note_apps/badge/cmd 追加语义 + §2.1 color/full 审计补记 + Q2 后置
    记录）；`shell-pack-sync.py --sync` pin 四件（shell=0c185aa354/
    desktop=331cca880f/switcher=789f4c4f17/notification_center=52993bd461，
    校验模式四件全等）；字段表↔model 逐一比对：shell.at（__wm_date/
    __wm_notes_badge）desktop.at（__wm_running）notification_center.at
    （note_apps）全登记，无漏。a2vue desktop 金样重生成（17/17 a2vue
    套件绿）。schema_drift fence PASS。

## 9. 复审记录

- 2026-09-17 stage:review PLAN-014 rev2 outcome:**needs_fix** —
  reviewed_commit: auto-os `5c2f5c2b`（plan-014-dev）/ auto-lang
  `bcc7f6c87`（auto-os-dev）；base: auto-os 325095b（main）/ auto-lang
  eefb5d84d（master）；dependency: auto-down detached b37b08e（纯构建依赖，
  零改动）。两 worktree 提交时点零脏区。**独立性声明**：评审与 work 同会
  话执行，结论从工件重建（完整 diff 清点 + 套件实跑 + 失败逐个归因），
  已发现 work 阶段自检未覆盖的问题（见 F-02..F-03），非橡皮章。
  spec_inputs: auto-lang schema v1.8（worktree 已提交，待 merge 发布）；
  **SD-01 目标文件缺失（F-01）**。
  **acceptance_results**：AC-01 pass（grep 零残留复跑；parse_records 双
  记录金样 + drain read/clear + SendCmd 单点，scoped 34/34）；AC-02/03/04
  partial（代码落码 + scoped 绿，但 F-02/03/04 套件红 + 实机运行时门未
  过）；AC-05 partial（schema v1.8 落盘、hash-lock 四件全等、字段表↔model
  比对在案；SD-01 spec 缺）。
  **findings**：
  - **F-01（blocking，SD-01/AC-02..04）**：规范增量 SD-01 目标
    `docs/specs/shell/showdesk-ux-polish.md` 未在 worktree 准备（work 阶
    段漏项——SD-02 已落、SD-01 无文件）。修正：work 按计划 §5 SD-01 行撰
    写行为合同（桌面选中/启动反馈、hidden 去重与恢复、时钟/日期注入面、
    badge 形态、通知来源跳转、switcher 预选/hover）提交 auto-os worktree。
  - **F-02（blocking，T-02/W-01）**：`ui::iced::renderer::tests::
    desktop_shell_at_builds_with_dock_defaults` 以 call_handler 调用已删
    的 `WorkspaceClose` pack handler → HandlerNotFound（renderer.rs:26940）。
    定性：fixture 陈旧于已批准的 W-01 删除合同（T-01 宿主路径核查只覆盖
    生产派发路径，测试 fixture 调用点漏查——`grep` 输出 head 截断所致）。
    修正：fixture 改走 bus 动词臂或删该调用段。
  - **F-03（blocking，T-05/W-04，含 022 邻接面）**：`desktop_surface_at_
    loads_interactions_and_dispatch` hover 变体类 **0/2**（renderer.rs:27888
    「每格一枚 hover: 变体类」）——格 col 样式链由三分支加深为五分支
    （drag→drop→launching→selected→hover）后，VM 视图对布局件 hover 类
    的提取归零。dbl/clk/rc 断言均过、唯样式 hover 面归零——疑似
    aura_view_builder 样式 if 表达式求值深度限制或静默失败，**须修实现
    形态或 builder，非仅改测试**（影响 VM 轨桌面格 hover/样式面）。
  - **F-04（blocking，T-09/W-10）**：`switcher_summon_advance_confirm_
    roundtrip` 断言 RebuildMru 后 `sel == 0` 旧合同；W-10 已批准语义 =
    nres>1 预选 1。修正：fixture 断言更新为新合同。
  - **F-05（non-blocking，验证基建）**：worktree vue 轨 gen/front/vue
    依赖不全（vue-sonner/reka-ui/@vueuse/core/class-variance-authority
    unresolved，vite 启动失败）——运行时门 unblock = gen 目标 `pnpm
    install` 装齐（或自主检出 gen 等价补齐，禁 junction）。
  - **环境类观察（非本计划债，已归因排除）**：master 基线自身 24 失败
    （layout 几何 ×11/vm_bridge calendar ×4/aura_view_builder ×3/iced
    renderer ×2/desktop_protocol ×1 等；主检出同套件实跑在案
    /tmp/master_ui_failures.txt 结论已誊入本记录）；`external_config_poll_
    hot_apply_loopsafe` 在 desktop 进程并发运行时被外部 config 主题翻转
    污染假红（desktop 停后单跑 PASS 已证）；`chart_geom` 反向不对称
    （master 红分支绿）同属波动。上述四类均先于/独立于本计划存在。
  **evidence**：完整套件日志 /tmp/wt_ui_full.log（分支 2060 跑：2033 pass
  /27 fail——24 基线 + 3 真回归）与 /tmp/master_ui_failures.txt（master
  2060 跑：2036 pass/24 fail）；失败归因逐条在案；work 阶段 scoped 证据
  与 rev2 mock 见 `docs/plans/evidence/014/rev2/`。
  **next**：`/auto-plan:work`（repair cycle 1/3）：F-01 撰写 SD-01 spec、
  F-02/F-04 fixture 合同更新、F-03 实现修复（hover 提取），完成后 re-review
  （重审门 = 全 ui 套件零新增红 + vue 轨运行时门补齐）。
- 2026-09-17 stage:work PLAN-014 rev2 outcome:**pass** — T-01..T-10 全执
  行（代码实现完整、scoped 验证绿、双 worktree 已提交）。worktree 组
  `.wt/os-014/`：auto-os `plan-014-dev`（基线 325095b，shell pack + rev2
  证据）；auto-lang `auto-os-dev` bcc7f6c87（基线 master eefb5d84d，宿主
  臂 + schema v1.8 + pin 同步 + a2vue 金样）+ auto-down 兄弟（detached
  b37b08e，纯构建依赖）。验证记录：`shell_packs_compile`（四 pack 真管
  线 compile+Init）PASS；scoped 套件 drain/notif/projection/shell_pack
  34/34 PASS（含修复 notif_center_summon_headless 召唤注入点漏 note_apps
  后全绿）；`shell-pack-sync.py` 校验四件全等；a2vue desktop 金样重生成
  后 17/17 绿；schema_drift PASS；W-01 grep 零残留。实现调整三项（语义
  等价，均在授权内）：W-02 两行内联→SendCmd(rec) 单点 handler；W-07 9+
  截断→宿主派生 `__wm_notes_badge`（I9）；Q1 spike 经代码实证提前定案
  （parse_records 换行切分 + 每周期排空，追加语义无指纹冲突）。**遗留至
  review 门（环境所限非代码缺失）**：双轨（Vue+VM）×双主题截图对拍
  rev2 mock、实机装配冒烟（desktop.sh 起桌面 dock/热键/022 拖拽族/019
  菜单臂回归）、W-04 自愈/拖拽互洽、W-05' 重启全量回、W-06' 跨天、
  W-08 两臂、W-10 混合推进的实机用例——沿 022 先例执行期用户逐条截图
  验收通道。next: review（`/auto-plan:review`，worktree 组保留）。
- 2026-09-17 stage:new PLAN-014 **rev2** outcome:**pass** — 重基线修订
  完成（disposition 全部用户裁定，见 §0 表与 §4 授权记录）：删 W-03/09/
  11/13（022 位图、012+016+019 外点重构、015 命名链、497 G2 挂载容器
  实证取代）；W-14 移出转独立 auto-lang 计划挂账；W-05 砍显示桌面（019
  负一屏接管，其待澄清 #1 就此清偿）；W-06 砍点击跳日历（注册表实查无
  日历 app）；W-12 改协议 v1.8（v1.6/1.7 已被占用）；W-04 mock 重对齐
  022 网格形态。净保留九项，T-01..T-10 覆盖 AC-01..05 与 SD-01/02；
  基线 = main（019/022 merge 后），无冲突面。next: work
  （`/auto-plan:work`，worktree `.wt/os-014/`）。

## 10. 待澄清事项

- **Q1（W-02，owner: T-03 spike）**：**已清偿（2026-09-17 work T-03）**
  ——spike 经代码实证定案：`parse_records` 按 \n/REC_SEP 切分多记录
  （session.rs:1645）+ 排空点每 update 周期读+清（renderer.rs），追加
  语义与宿主排空零冲突；退化路径未动用，四 pack 已落 `SendCmd` 追加
  单点（含既有双记录金样覆盖）。
- **Q2（oncontextmenu 坐标事件臂，owner: 用户，默认后置）**：rev1 W-12
  曾计划事件带 `(x,y)` 并删 `__desktop_cursor_x/y` 宿主泵；022 拖拽幽灵
  臂现依赖该泵（desktop.at:229），改造前需裁定交互归属。默认不在 v1.8
  强捆，泵与坐标锚照旧。
- **Q3（日历 app 缺位，owner: 用户）**：W-06' 点击臂已砍。若要时钟点击
  跳日历，需先立项日历 app（注册表无此 app；ui-gallery 016-calendar 仅为
  demo），另行计划。
- **Q4（双轨默认主题统一，owner: 用户）**：rev1 Q5 沿承——VM 轨缺省
  stella、Vue 轨缺省 scaffold；本计划验收两轨各按本轨实际主题对拍，统一
  默认主题另行裁定。
- **Q5（W-14 B12 转独立，owner: 用户/另会话）**：「宿主注入 Obj 数组
  handler 字段读失效」修复在 auto-lang VM（Category A），另立 auto-lang
  侧计划（编号待取）；落地后本计划 `note_apps`/平行列表族消参。
