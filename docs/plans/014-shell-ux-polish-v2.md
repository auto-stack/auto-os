---
plan_id: PLAN-014
status: drafting               # drafting → executing → execution_done → reviewed → archived
feature_name: shell-ux-polish-v2
author: [zhaopuming]
created_at: 2026-09-14
updated_at: 2026-09-14

# /auto-plan:review 结束时填写：
supersedes_spec_components: []
new_spec_components: []
touched_goals: []             # 引用 docs/specs/goals.md 的 GOAL-NNN

affects: [shell/shell.at, shell/desktop.at, shell/switcher.at, shell/notification_center.at,
          auto-lang/schema/projection-protocol-v1.md, auto-lang/ui/iced/renderer.rs,
          auto-lang/ui/desktop_config.rs, apps/003-clock（时钟日期点击目标）]
current_step: 0
total_steps: 14
---

# [PLAN-014] shell-ux-polish-v2

## 变更摘要

虚拟桌面 shell UX 打磨第二批。2026-09-14 代码+设计评审产出两轮建议（视觉缺陷
/交互闭环/信息架构/协议债），mock 定稿四张（`docs/plans/evidence/014/`，
**深/浅双主题同版式**——浅色按项目实际默认主题 **stella light** 实测 token
映射（`auto-lang design_tokens/registry.rs`：背景 #F5F1E8 暖奶油纸色 /
card #FBF8F2 / foreground #2A2723 / primary #6466F1 indigo / destructive
#EF4444；深色 primary #9394F5 同色相提亮），形式与深色完全统一；用户
2026-09-14 审定，浅色第一轮误画 shadcn 粉紫通用浅色已纠偏）：

| 图 | 主题 | 标注 | 内容 |
|----|------|------|------|
| `014-mock-desktop.png` | 深色 | ①-⑦ | ①桌面图标紧凑左上+单击选中态 ②启动中指示 ③空白菜单扩充（含恢复入口） ④launcher 独立字形 ⑤运行/聚焦指示 ⑥未读角标圆形化 ⑦时钟+日期两行 |
| `014-mock-panels.png` | 深色 | ⑧-⑩ | ⑧通知整行点击跳来源 ⑨时间戳/分组 ⑩切换器预选第 2 项 |
| `014-mock-desktop-light.png` | 浅色 | ①-⑦ | 同上（stella 浅色映射：选中块 #E3DDD1 半透明底/预选行 indigo 底白字/hover 8% 黑底/角标 #EF4444 不变） |
| `014-mock-panels-light.png` | 浅色 | ⑧-⑩ | 同上 |

与 **PLAN-012（shell-ux-feedback-batch）** 的关系：**串行，本计划排在 012 之后**
（012 work 已于 2026-09-12 收口、status=execution_done 待 review/merge；与本
计划同改 `shell/shell.at` dock 区段与 `shell/desktop.at` 图标网格——012 的
G4 dock 合并/G5 图标紧凑+拖拽/G7 状态指示定型先落，本计划在其定型结论上
继续做；冲突面以 012 实现为准）。012 已覆盖项本计划不重复：os-config 卡顿（G1）、通知
面板 gap/关闭三路径（G2）、切换器面板外点关闭/整桌面预览（G3）、dock 合并
pin（G4）、图标紧凑/拖拽（G5）、launcher 自列（G6）、图标居中/三态指示（G7）。

本计划覆盖 012 之外的 14 项（W-01..W-14），按面归四组：

- **清理组**（W-01/02/03/13）：死消息、`__desktop_cmd` 单槽覆盖丢命令、
  字形冲突、真空托盘占位。
- **桌面组**（W-04/05）：单击选中态 + 启动中反馈；空白菜单扩充
  （恢复默认图标/显示桌面/整理图标）+ hidden 去重。
- **任务栏组**（W-06/07/11）：时钟两行+日期+点击跳日历；未读角标圆形化；
  窗口标题兜底（消灭 "App"/"DualApp"）。
- **面板+协议组**（W-08/09/10/12/14）：通知行跳来源 + scrim 外点关闭；
  switcher 预选第 2 项 + hover 跟随；**投影协议 v1.6** 汇总增量
  （desktop 注入 `__wm_running`、notes 注入 app 字段、时钟/日期、
  oncontextmenu 事件带坐标、命令总线追加语义）；B12 平行列表债务清偿。

跨仓计划：主导仓 = 本仓（auto-os）；改动落 **auto-os `shell/`** pack、
**auto-lang**（协议 schema + 宿主/渲染器，Category A 门档：不在 auto-lang 跑
`cargo t`）、**apps/003-clock**（仅核对日历 app id，预判零改动）。shell pack
改动后跑 `scripts/shell-pack-sync.py` 对齐 auto-lang 内嵌快照（hash-lock 契约）。

## 目标

1. **G1（清理）** shell pack 无死消息/死 handler；`__desktop_cmd` 多命令同
   排空周期不丢失；任务栏无同形双钮；无恒空 UI 占位。
2. **G2（桌面）** 桌面图标单击有选中反馈、双击启动有进行中反馈（mock ①②）；
   空白菜单具备恢复默认图标与显示桌面入口，hidden 串写入去重（mock ③）。
3. **G3（任务栏）** 时钟两行（HH:MM + 日期）且点击打开日历 app；未读数为
   圆形角标压铃铛右上角；所有虚拟窗标题来自注册表（无泛化名）（mock ④⑥⑦）。
4. **G4（面板）** 通知整行点击跳来源 app、外点可关；switcher 预选 MRU 第 2
   项、鼠标 hover 跟随选中（mock ⑧⑨⑩）。
5. **G5（协议）** 投影协议升 v1.6（五处增量一次落），shell pack 同步
   hash-lock；B12 workaround 三处收敛（可拆独立批次，允许后置）。

## 架构方案

延续 WM-as-app / shell=用户态定案（Design 23 R1 + I7-I9）：本计划全部 shell
侧改动仍是 pack 内数据驱动（宿主注入、shell 发动词），不引入 shell 侧几何
操作。新增注入一律走协议字段扩展（仅字段、不增动词词表——除 W-05
`show_desktop` 一个动词），由宿主解析注入，保持 I9 单一事实。

`show_desktop`（W-05）是唯一的词表增量：宿主臂 = 最小化当前分区全部虚拟窗，
语义与 Win「显示桌面」一致，与既有 `win_min` 同族。

## 需求分析与背景调查

评审证据（2026-09-14，详见当日评审对话与截图）：

- `docs/reports/p6-v1/v1_1_desktop_dock.png` 实证：桌面图标被 `grid (cols:8)`
  + `w-full` 拉满全屏（012 G5 已接，本计划不做）；任务栏 launcher 钮与
  布局钮同为 `layout-grid` 字形（`shell/shell.at:133` vs `:244`）；未读 badge
  为铃铛旁裸红字（`shell/shell.at:264-268`）。
- 死消息：`LayoutFree`（shell.at:381，T35 后无发送者）、`WorkspaceNext`
  （:400）、`WorkspaceClose`（:403）、`HoverWs/HoverWsEnd`（:428-429，T18 后
  无发送者）。
- `__desktop_cmd`：协议头注释承诺"多条以 \n 连接"（shell.at:11），全部
  handler 直接覆盖赋值——同排空周期两命令后者覆盖前者，前者无声丢失。
- 桌面移除：`shell.desktop.hidden` 逗号串追加无去重（desktop.at:198-208），
  且无任何恢复 UI 入口。
- 空白菜单坐标锚依赖宿主持续泵 `__desktop_cursor_x/y`（desktop.at:48-49）——
  oncontextmenu 事件不带坐标的协议缺口 workaround。
- 通知条目无来源 app 字段（`__wm_notes` v1.2 四字段），整行不可点。
- switcher `RebuildMru` 复位 `sel=0`（switcher.at:129）——MRU 第 1 项 =
  当前聚焦窗，Enter 聚焦 = 无操作；行无 hover 跟随。
- 窗口标题泛化：实机截图虚拟窗标题为 "App"/"DualApp"——注册表 title 缺失
  且无宿主兜底。
- B12：宿主注入 Obj 数组的 handler 字段读失效 → switcher/notification_center/
  launcher 三处平行字符串列表 + while 重建样板（switcher.at:29-39/129-140、
  notification_center.at:32-40/127-137），共 ~120 行重复。
- desktop.at 头注（:14-16）声明注入 `{id,icon,label,src}`，view 实际消费
  `e.color`（:102）——协议 v1.4 字段表漏记 color。

## 详细设计

### W-01 死消息清理（shell.at）

删除无发送者的 msg + handler：`LayoutFree`、`WorkspaceNext`、
`WorkspaceClose`、`HoverWs/HoverWsEnd`。
**开工前置**：grep auto-lang 宿主消息派发，确认宿主热键不直投这些消息名
（宿主热键走直写状态/直发命令，预判全死；若 `WorkspaceNext` 有宿主键盘
投递路径则保留该条并注释说明）。顺手修 desktop.at 头注补 `color` 字段。

### W-02 `__desktop_cmd` 追加语义（四个 pack）

写总线 handler 统一追加形态：
```
if .__desktop_cmd != "" { .__desktop_cmd = .__desktop_cmd + "\n" }
.__desktop_cmd = .__desktop_cmd + "<verb>\t<arg>"
```
涉及 shell.at / desktop.at / switcher.at / notification_center.at 全部写点。
若追加形态实测触发宿主排空指纹问题（多行命令序），退化为：修注释为
"单槽，每周期至多一条，重复动作合并"——开工 spike 10 分钟定案。

### W-03 任务栏字形区分（shell.at）

launcher 钮 `layout-grid` → `search`（备选 `app-window`，开工核对注册表
lucide 字形可用性后定）；切换器钮 `square-stack` → `layout-dashboard`；
布局钮保留 `layout-grid`（九宫格=布局语义唯一）。mock 图一④。

### W-04 桌面选中态 + 启动中反馈（desktop.at + 协议 v1.6）

- `sel_id str`：图标 mouse-area `onclick` → `.Select(e.id)`（单击选中），
  chip 外包白/10 圆角选中块（mock ①）；BlankPress 清空。
- 启动中：`ActivateApp` 后置 `launching = id`（chip 半透明 + 右上角灰点，
  mock ②）；协议 v1.6 给 desktop.at 注入 `__wm_running`（与 shell.at 同形
  派生串），handler 读入：running 含 launching id 即清 launching 态。
- 防御：launching 态无 ack 超时自愈（宿主臂启动失败场景）——宿主启动失败
  时保证 `__wm_running` 不含该 id，desktop.at `Init`/重注入时 launching
  与 running 求差清残。

### W-05 空白菜单扩充 + hidden 去重（desktop.at + 宿主）

- `MenuRemove` 追加前去重（`.contains("," + id + ",")` 域串包裹判断）。
- 空白菜单增两项：**恢复默认图标** = `storage.set("shell.desktop.hidden","")`
  （本地态即生效，下次注入自然全量回）；**显示桌面** = 新动词
  `show_desktop`（宿主臂最小化当前分区全部虚拟窗）。**整理图标** v1 不做
  （依赖拖拽定案，012 G5/Q2 挂账），菜单不放灰化项。
- mock 图一③。词表增量仅此一个，协议 v1.6 登记。

### W-06 时钟两行 + 点击跳日历（shell.at + 宿主）

- 宿主 `__wm_clock` 注入扩为两行数据源：分钟变化时写 `"HH:MM"`（不变），
  日期变化时写 `__wm_date`（`"M月D日 周X"`）——两字段独立脏帧，稳态零重建
  口径不变。
- shell.at 时钟区改两行 col；时钟区 onclick → `activate\t<calendar_app_id>`
  （复用 472 activate 两臂，零新动词）。calendar_app_id 开工从注册表核对
  （003 clock / 画廊 calendar 之一，以注册表实际 id 为准）。

### W-07 未读角标圆形化（shell.at）

badge 改绝对定位圆形：`bg-error text-primary-foreground rounded-full`
16px 圆压在铃铛钮 `-top-1 -right-1`；数字 >9 显示 "9+"。iced/Vue 双端同
class（505 B1 数据驱动口径）。mock 图一⑥。

### W-08 通知跳来源（协议 v1.6 + notification_center.at + 宿主）

- `notify` 落库时记录来源 app id；`__wm_notes` 增 `app` 字段
  （B12 规避期平行列表同型加 `note_apps`，W-14 收敛后消参）。
- 通知行 onclick → `activate\t<r.app>`（宿主臂：未运行启动/运行聚焦）。
  mock 图二⑧。

### W-09 scrim 外点关闭（switcher.at + notification_center.at）

012 已落两面板外点关闭（popover ondismiss 模式：switcher 面板锚
square-stack 钮、通知面板补 ×+外点）。**本任务退化为核对**：开工 diff
012 的实现，确认覆盖 switcher.at / notification_center.at 两 App 的
外点路径；有死角（如 App 内 scrim 未包守卫）才补，无则记录核对结论销项。

### W-10 switcher 预选第 2 项 + hover 跟随（switcher.at）

- `RebuildMru`：`sel = nres > 1 ? 1 : 0`（mock 图二⑩，Alt-Tab 惯例）。
- 行 mouse-area 增 `onmouseenter: .HoverSel(r.i)` → `sel = i`；键盘
  Advance/Back 从当前 sel 继续（天然满足，无需特判）。

### W-11 窗口标题兜底（auto-lang 宿主 + 注册表核对）

- 注册表 pac.at 逐 app 核对 `title`/`name` 字段（003/025/028/038/画廊两件）。
- 宿主侧 fallback：VirtualWindow 标题源缺失时回退注册表 app name（不回退
  字面 "App"）。跨 auto-lang，改动面 = 标题注入臂，Category A 门档内。
- 验收：实机截图无泛化标题。

### W-12 协议 v1.6 汇总（auto-lang schema + 宿主 + hash-lock）

一次升版承载本计划全部字段/词表增量：
- 入向字段：`__wm_running` 注入 desktop.at；`__wm_notes[].app` /
  `note_apps`；`__wm_date`；`__desktop_cmd` 追加语义写入协议（W-02
  定案结果）。
- 事件臂：oncontextmenu 带 `(x, y)` 参数——desktop.at 删
  `__desktop_cursor_x/y` 两变量与宿主泵送臂（PLAN-012 坐标锚数据源改
  事件参数直读）。
- 词表：`show_desktop`（W-05）。
- `schema/projection-protocol-v1.md` 字段表全量登记（含 v1.4 漏记的
  `__desktop_icons[].color`）；`scripts/shell-pack-sync.py` 同步 hash-lock。

### W-13 托盘占位处置（shell.at）

`shell.at:303-305` 恒空 tray row：v1 撤除（注释留 tray API 立项回挂点）；
若 012 G4 落地后右簇重排则随其布局一并处理。

### W-14 B12 债务清偿（auto-lang VM，可后置/可拆独立 plan）

修复"宿主注入 Obj 数组 handler 字段读失效"，switcher / notification_center /
launcher 三处平行列表 + while 重建收敛为直接消费合同面（`__wm_mru` /
`__wm_notes`）。涉及 launcher 的改动与 028-launcher 仓协同。本任务与 W-01..13
无依赖，允许后置为独立计划；W-08 的 `note_apps` 平行列表在 W-14 后消参。

## 测试设计

- **双端纪律**：每个改动的验收 = auto-lang autoui-verifier 双端（Vue 轨 +
  VM 轨）× **深/浅双主题**截图对照 mock（`evidence/014/` 四张——浅色一律
  按 stella light token 验收，语义态浅色映射口径：选中块 #E3DDD1 半透明
  底 / 预选行 #6466F1 底白字 / hover 8% 黑底 / 角标 #EF4444 与语义色不变）
  + 实机装配冒烟（`bash scripts/desktop.sh` / `desktop.ps1`）。
  注：Vue 轨运行时默认主题为 scaffold（白底），VM 轨默认 stella（奶油底）
  ——双端截图各按本轨实际主题对拍，若要两轨统一默认主题另行裁定。
- **宿主改动**（W-05 显示桌面臂 / W-06 日期注入 / W-08 app 记录 / W-11
  标题兜底 / W-12 注入面）：auto-lang 侧 hand 测试或既有宿主冒烟脚本，
  不在 auto-lang 跑 `cargo t`（Category A）。
- **协议 v1.6**：schema 文档 diff + 指纹门控核对（宿主注入面字段表与
  shell pack model 声明逐一比对）。
- **W-02**：构造同周期双命令场景（双击+右键菜单快连击）宿主排空日志
  验证两命令均执行。
- 回归面：dock 三动作菜单、桌面双击激活、热键召唤三 overlay、
  通知 badge 计数——每 W 完成后过一遍。

## 验收标准

1. G1：grep shell pack 无死消息；宿主排空日志同周期双命令零丢失；
   任务栏 launcher/布局/切换器三钮字形两两不同；无恒空容器。
2. G2：mock 图一①②③ 逐点实机对拍通过；hidden 串写入去重（重复移除
   同 id 不产生重份）；恢复默认图标后重启桌面全量回。
3. G3：时钟两行显示、分钟/日期独立刷新；点击时钟打开日历 app；未读
   数为圆形角标且 9+ 截断；全仓窗口无 "App"/"DualApp" 泛化标题。
4. G4：通知行点击跳来源（未运行启动/运行聚焦两臂）；外点关两面板；
   switcher 召唤预选第 2 项、Enter 即切最近其他窗、hover 跟随与键盘
   推进互洽。
5. G5：`schema/projection-protocol-v1.md` v1.6 节齐五增量；
   `shell-pack-sync.py` hash 对齐零 diff。

## 执行步骤

（原子任务：精确文件路径 + 确切操作 + 验证命令；每步完成后追加
[✅ 已完成] 一行证据。**前置：PLAN-012 已 execution_done（2026-09-12 收口，
待 review/merge）**；开工时核对其 work diff 与本计划的冲突面（dock 区段/
图标网格/两面板外点关闭），冲突处以 012 实现为准。）

1. **W-00 开工核对**：grep auto-lang 宿主消息派发确认 W-01 五消息无宿主
   投递路径；核对注册表 lucide 字形（search/app-window/layout-dashboard）
   与 calendar app id；确认 012 状态。验证：grep 输出存档 evidence/014/。
2. **W-01** 删 `shell/shell.at` 五组死 msg+handler；修 `shell/desktop.at`
   头注补 `color`。验证：pack 冒烟（desktop.sh 起桌面，dock/热键回归）。
3. **W-02** 四 pack 写总线 handler 追加语义统一（或按 spike 退化为注释
   修正）。验证：同周期双命令日志。
4. **W-03** `shell/shell.at` launcher/切换器 icon 换字形。验证：截图对拍
   mock 图一④。
5. **W-04** desktop.at 加 `sel_id`/选中块/`launching` 态；宿主注入
   `__wm_running` 至 desktop 层（协议 v1.6 字段先行落）。验证：mock ①②
   对拍 + 启动失败残态自愈用例。
6. **W-05** MenuRemove 去重；空白菜单加"恢复默认图标/显示桌面"；宿主
   落 `show_desktop` 臂。验证：mock ③ 对拍 + hidden 去重用例。
7. **W-06** 宿主加 `__wm_date` 注入；shell.at 时钟两行 + onclick
   activate 日历。验证：mock ⑦ 对拍 + 跨天日期刷新用例。
8. **W-07** shell.at badge 圆形绝对定位 + 9+ 截断。验证：双端截图。
9. **W-08** 宿主 notify 记录来源 app；协议字段 + notification_center.at
   行 onclick。验证：mock ⑧ 对拍（未运行/运行两臂）。
10. **W-09** 核对 012 已落的两面板外点关闭（popover ondismiss）覆盖度，
    有死角才补 App 内 scrim 守卫。验证：外点关闭用例 + 核对结论记录。
11. **W-10** switcher.at sel 预选第 2 项 + 行 onmouseenter 跟随。验证：
    mock ⑩ 对拍 + 键盘/鼠标混合推进用例。
12. **W-11** 注册表 title 核对 + 宿主标题兜底臂。验证：实机无泛化标题
    截图。
13. **W-12** schema/projection-protocol-v1.md v1.6 节（五增量）+ 宿主注入面
    + `scripts/shell-pack-sync.py` hash 对齐。验证：字段表与 model 声明
    逐一比对记录。
14. **W-13/14** 撤真空 tray row（或随 012 G4 布局处理）；B12 清偿实施或
    拆独立 plan 挂账（本步结束时二选一并记录裁定）。

## 复审记录

（空——/auto-plan:review 时填写）

## 待澄清事项

- Q1（W-03）：launcher 新字形 `search` vs `app-window`，开工按注册表
  可用字形定（mock 按 search 画）。
- Q2（W-02）：追加语义若与宿主排空指纹冲突，是否接受退化为注释修正？
  倾向：冲突则退化为注释 + 单槽约束明确化（UI 节奏下两命令同周期概率
  极低）。
- Q3（W-14）：B12 在 auto-lang 属 VM 核心修复（Category A），是否本计划
  内做还是另起 auto-lang 侧 plan？倾向：本计划只落 shell 侧消参准备，
  修复另立 plan。
- Q4（W-09）：~~若 PLAN-012 先行落地外点 dismiss~~ **已澄清**——012
  execution_done 时外点关闭已落（popover ondismiss 模式），W-09 按退化
  执行（核对补死角）。
- Q5（主题基调，2026-09-14 主题 token 调查新增）：双轨默认主题不一致——
  VM 轨缺省 **stella**（theme/mod.rs:42-45），Vue 轨运行时缺省 **scaffold**
  （ui_gen/vue.rs:16480，白底）。本计划验收两轨各按本轨实际主题对拍；是否
  统一默认主题（如 Vue 轨也归 stella）不在本计划范围，留用户裁定。
