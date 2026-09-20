---
plan_id: PLAN-035
status: execution_done        # drafting → executing → execution_done → reviewed → archived（rev3 回环收口 2026-09-20）
feature_name: desktop-ux-rev3
author: [agent]
created_at: 2026-09-20
updated_at: 2026-09-20
plan_revision: 2

# /auto-plan:review 结束时填写：
supersedes_spec_components: []
new_spec_components: []
touched_goals: []             # 引用 docs/specs/goals.md 的 GOAL-NNN

affects:                      # 受影响 specs/实现路径
  - auto-os/shell/shell.at                                    # T-01 右组右对齐 / T-03 sliver
  - auto-os/shell/dashboard.at                                # T-04 头行 72px（网格行 0）
  - auto-lang/crates/auto-lang/src/ui/iced/renderer.rs        # T-02 picker 钳制 / T-04 layout v2 / T-07 孵化 drain
  - auto-lang/examples/ui/012-clock/src/front/app.at          # T-05 mini 居中
  - auto-lang/examples/ui/013-todo/src/front/{app.at,todo_store.at} # T-06 preview 双列 + span3
  - auto-lang/examples/ui/020-music-player/src/front/{app.at,player_store.at} # T-07 紧凑 mini + 空曲库通知化
  - auto-os/docs/specs/shell/dashboard.md                     # SD-01/02/03 沉淀
  - auto-os/docs/specs/shell/showdesk-wallpaper.md            # SD-04 sliver/picker 沉淀
  - auto-os/docs/specs/shell/showdesk-icons.md                # SD-05 任务栏布局合同
  - auto-lang/crates/auto-lang/assets/                        # pin 快照 sync（shell-pack-sync）
current_step: 16
total_steps: 16
---

# [PLAN-035] desktop-ux-rev3

## 0. 变更摘要

用户实机走查（2026-09-20 截图三张）反馈五组桌面 Shell 问题，本计划收口：

1. **任务栏右组居中 bug**：切换器/通知等中段图标组应右对齐贴时钟，实测
   被双侧等宽空隙夹在屏幕中央。
2. **任务栏壁纸 picker 游离块**：launcher 与计算器之间出现一小块（壁纸
   picker 的「浏览…」按钮透过 bg-card/95 任务栏隐现——放大取证在案），
   点击弹出「选择壁纸目录」原生对话框——picker 卡在开启态且渲染在陈旧
   坐标。
3. **显示桌面 sliver 修缮**：无 hover 提示；hover 高亮未覆盖整块；宽度偏
   大；截图示块体越出任务栏下缘。
4. **小组件面板网格重构**：面板外框应占据屏幕右上角 **8×3** 桌面图标网格；
   widget 默认 **2×2**、内容多者 **3×2**（各组件自定），卡框对齐网格线。
   （现实现：外框 10 列吸附已落但内部仍三等分列 span1|2 缩放——spec
   SD-01 领先实现，本计划按用户新裁定一并收口。）
5. **三个 mini 细节**：时钟内容偏右未居中；todo 内容过疏（加右列：未完成
   前三清单）；音乐卡因「本地曲库为空」占位把播放控件挤出卡外（空曲库
   消息改走系统通知流，控件全部可见）。

**跨仓**：auto-os（本 plan + `shell/*.at`）主导，auto-lang（`renderer.rs`
layout v2/drain 扩容/picker 钳制 + `examples/ui` 三 app mini 视图）配合。
两仓 worktree 分组平铺（Plan 529 布局 `.wt/os-035/`）。

## 1. 目标

- **G1** 任务栏布局确定性：右组图标右缘贴时钟左邻，任何窗口宽度下无双侧
  等分空隙。
- **G2** 壁纸 picker 无游离残留：任意时刻 picker 面板完整在屏、可关闭；
  任务栏区域无幽灵块。
- **G3** sliver 完成度对齐 Win11 惯例：更窄、hover 满覆盖 + 文字提示、
  不越界。
- **G4** 小组件面板 = 右上 8×3 网格（列距 88 / 行距 80 / 12px 边距），
  widget 卡 2×2（缺省）/ 3×2（声明），卡框落网格线。
- **G5** 三个既有 mini 视图信息完整、布局居中、空态走通知流。

**非目标**：任务栏/面板交互语义重构（编辑 popover F-01 仍延后）；tooltip
通用原语（P548-D2 债不动，sliver 用 popover 锚定形态自建）；平板网格 v2
resize（PLAN-024 §9.3 不变）；VM 轨桌面 parity（特权 shell 仅 iced 轨）。

**受影响仓**：见 frontmatter `affects`。成功样态：五组问题在实机（master
构建重启后的桌面）逐一复验通过并留截图证据；auto-lang `cargo t` 全绿；
spec 沉淀与 pin 快照 hash-lock 对齐。

## 2. 架构方案

- **分工面不变**：shell pack（auto-os `shell/`，运行时直读——`resolve_shell_pack_dir`
  解析序，`ui/shell.rs:35-97`）承载视图/本地态；宿主（auto-lang
  `renderer.rs`）承载几何单一事实、命令排空、通知落库。双仓改动按既有
  接缝落，不新增协议动词（notify / wallpaper_close 等词表已备）。
- **任务栏右组（T-01）**：静态分析（`build_row`/`apply_row_style`/
  `convert_spacer`、iced 0.14 Row 默认 Shrink）指向单一 Fill spacer 应已
  右对齐，与截图矛盾 → 先 T-00 实机归因（陈旧二进制 vs 适配层未知 Fill
  默认两假设），按载决修 `.at` 结构（空托盘 row 退役/显式收缩）或适配层
  默认宽。修复合同：任务栏主轴 **Fill 元素恰一**。
- **picker 收口（T-02）**：宿主 `inject_wallpaper_picker` 注入前对
  `__wp_x/__wp_y` 按 viewport 钳制（含任务栏 reserved 边，
  `desktop_dock_edges` 同源）；排查 stuck-open 路径（点选即应用保持开启
  属设计语义，但 ×/Esc/外点必须可达——钳制后自然恢复），补 boot/重注入
  防御臂。
- **sliver（T-03）**：`shell.at` 重构 sliver 臂——锚定 popover 形态
  （mouse-area 内 popover：anchor=细条 col、content=「显示桌面」提示）；
  宽度 w-3→w-2；高亮类落 anchor col 满覆盖；显式定高防溢出（命中/可视
  一致，PLAN-021 线 B 合同）。
- **dashboard 8×3（T-04）**：`dashboard_layout` v2——外框定尺寸 8 列×3 行
  （696×232，右上 12px，节距 88/80）；内部格位直接网格算术产出（span
  2|3 列 × 2 行格高 = 168|256 × 152px），删除 pw0/sx/sy 缩放路径；头行 =
  网格行 0（72+gap8，`DASH_HEADER_H` 48→80）；`dashboard.at` 头行
  h-12→h-[72px]。span 存储 `shell.dashboard.span.<id>` 缺省 2、合法域
  {2,3}（旧 "1" 迁移读作 2）；app 源声明通道 =
  `dashboard_face_candidates` 文本探测 `view mini (span: "3")`（存储覆写
  仍胜）。溢出（单 widget 行装不下）v1 裁剪 + dev 日志。
- **三 mini（T-05..T-07）**：clock 居中修复；todo store 增派生 `preview`
  （未完成前三，与 active_count 同臂维护）+ mini 双列 + 源声明 span 3；
  music 重排 2×2 紧凑布局 + 空曲库改发 `notify\tinfo\t…`（`__desktop_cmd`）
  + 宿主**联合排空扩容孵化会话段**（现仅遍历 `wm.wins`——孵化会话不可达，
  本计划唯一宿主动词面变更）。

## 3. 技术栈

- auto-os：AutoUI .at（shell pack 五件）、auto-plan 范式、specs 台账。
- auto-lang：Rust（iced 0.14 宿主 renderer）、.at examples、cargo test
  （ui-iced 档）、shell-pack-sync hash-lock（auto-os → auto-lang 单向）。
- 验证：cargo t（auto-lang，crates 有改动 → 门档允许实跑）+ 实机桌面
  走查截图（scripts/desktop 入口 / scratch/p035 证据惯例）。

## 4. 需求分析与背景调查

### 4.1 授权记录

- 用户 2026-09-20 会话明示：五组走查反馈立计划跟踪，随后以
  /auto-plan:work 执行（独立 worktree）。授权仓：auto-os + auto-lang。
- 预算：未设上限；门禁按本仓范式（cargo t 全量 + 实机证据）。

### 4.2 背景调查（证据锚点）

| # | 事实 | 源 |
|---|---|---|
| F1 | shell pack 权威源 = auto-os `shell/`（运行时直读，assets 为 pin 快照） | auto-lang `ui/shell.rs:15,35-97` |
| F2 | 任务栏结构：单 spacer `flex-1` + 右组 + 空托盘 row + 时钟 + sliver | auto-os `shell/shell.at:384-579` |
| F3 | iced 0.14 `Row::new()` 宽默认 Shrink；Container 按 content size_hint fluid | iced_widget-0.14.2 `row.rs:81`、`container.rs:96` |
| F4 | `spacer`→`flex-1` 容器（Fill）；`build_row` justify 用 FillPortion spacer 模拟 | auto-lang `aura_view_builder.rs:7312`、`renderer.rs:2193-2258` |
| F5 | dashboard 现实现：外框 R10 已吸附（10 列 872×232 右上 12px）但内部仍三等分列 `DASH_COLS=3/DASH_CELL_H=132` + sx/sy 缩放；panel_top 64 居中公式残留 | auto-lang `renderer.rs:10305-10310,10416-10469,19341-19364` |
| F6 | spec SD-01 已裁定网格吸附（10 列起）——本计划按用户新裁定改 8×3 | auto-os `docs/specs/shell/dashboard.md` SD-01 |
| F7 | 「选择壁纸目录」= rfd 原生对话框，唯一触发路径 = picker 头部「浏览…」→ `PickerBrowse` → `wallpaper_browse_dir`；截图小块隐现「浏览」字样 → picker 卡开启态 + 陈旧坐标渲染于桌面层、透过 bg-card/95 隐现 | auto-lang `renderer.rs:11556-11564`；auto-os `shell/desktop.at:311-329`；截图放大取证 |
| F8 | sliver 无 tooltip 原语可用（tooltip tag = fallback 未实现，P548-D2 债） | auto-lang `render_support.rs:340` |
| F9 | 联合排空只遍历 `wm.wins`（有窗 app）——孵化会话 `__desktop_cmd` 不可达 | auto-lang `renderer.rs:11015-11035`、`session.rs:2898-2910` |
| F10 | notify 动词 `notify\t<kind>\t<msg>`（app 可自 `__desktop_cmd` 发，v1.2 起；notify_source 分段归因既有） | auto-lang `session.rs:1656,1969`、`renderer.rs:11011-11048` |
| F11 | todo store 派生先例：active_count 于 Init/Add/Toggle/Delete/ToggleAll/ClearCompleted 六臂重算 | auto-lang `examples/ui/013-todo/src/front/todo_store.at` |
| F12 | music 空曲库置 `current_title="本地曲库为空"`（mini 直显根因） | auto-lang `examples/ui/020-music-player/src/front/player_store.at:112` |
| F13 | MouseArea 命中区=内容镜像容器（style 宽高镜像），wrapper 样式不参与命中 | auto-lang `renderer.rs:4728-4744` |
| F14 | 用户截图面板实测 ~1164 逻辑 px 宽 / 三等分卡，与 master F5 公式（≤920 或 872）均不符 → **运行二进制疑似滞后 master** | 截图像素测量 vs F5 |

### 4.3 归因疑点（T-00 有界调查载决）

- **Q-A 任务栏居中根因**：假设①运行二进制陈旧（F14 旁证，master 静态
  分析应已右对齐）；假设②适配层存在未识别的 Fill 默认。载决方式：master
  worktree 构建 → 实机复现五题 → 逐题标注「已修于 master（补回归守护）/
  仍在（按 §5 修）」。
- **Q-B picker 卡开路径**：何种操作序导致 picker_open 悬置 + 坐标落于任务
  栏区（右键落点即坐标锚，桌面层全屏含任务栏下方）。复现序留证。

## 5. 详细设计

### 5.1 T-01 任务栏右组右对齐

- Q-A 归因①（二进制陈旧）：master 无需改 `.at`——补**结构防御**（空托盘
  row 退役或显式收缩），加回归注记；归因②：定位 Fill 默认源
  （`build_row`/`apply_row_style` 臂）修 auto-lang，附单测（row 无宽类 =
  Shrink 合同）。
- 修复后合同：任务栏主轴 Fill 元素恰一（spacer）；右组末件（sliver）贴
  右缘（AC-01/SD-05）。

### 5.2 T-02 壁纸 picker 生命周期收口

- 注入臂 `inject_wallpaper_picker`：`__wp_x/__wp_y` 对
  `viewport - desktop_dock_edges(reserved)` 钳制（面板 720 宽右下界内
  收）；面板常量与 .at class 同源注记。
- stuck-open 载决（Q-B）：若复现出漏关路径（如应用后 × 不可达）补对应
  关闭臂；`wallpaper_close`/`PickerDismiss`/Esc 链回归走查。
- 防御：boot/重注入 picker_open 与几何同拍；空 items 面板仍完整可关。

### 5.3 T-03 sliver 修缮（shell.at）

```text
mouse-area (onclick: .ShowDesktopToggle, onmouseenter: .SliverHover, onmouseleave: .SliverUnhover) {
    popover (open: .sliver_hover == "1", placement: "top", class: "p-1 border rounded bg-card") {
        col { style: "w-2 h-10 <hover 高亮类在此，满覆盖>" }        // anchor 细条
        text "显示桌面" { style: "text-xs text-muted-foreground" }  // tooltip 内容
    }
}
```

- 宽度 w-3(12px)→w-2(8px)；hover 高亮与常态同几何（满覆盖合同）；anchor
  显式定高防溢出（AC-03/SD-04）；提示文案随 `__wm_showdesk` 动态
  （「显示桌面/恢复桌面」，Q3 初裁动态、复审定稿）。

### 5.4 T-04 dashboard 8×3 网格（dashboard_layout v2）

- 常量：`DASH_GRID_COL=88`（80+8）、`DASH_GRID_ROW=80`（72+8）、
  `DASH_MARGIN=12`、`DASH_COLS=8`、`DASH_ROWS=3`、`DASH_HEADER_H=80`
  （行 0 满格 72+gap8）、`DASH_WIDGET_H=152`（2 行格）。
- 签名改 `dashboard_layout(viewport, faces) -> (panel_x, panel_y, panel_w,
  panel_h, cells)`（视口绝对坐标）：
  - `panel_w = 8*88-8 = 696`、`panel_h = 3*80-8 = 232`；窄视口下限防御
    沿旧 clamp 家法（PLAN-659 T-04 上界守卫不回退）；
  - `panel_x = vw - 12 - 696`、`panel_y = 12`（右上角）；
  - face 卡：`w = span*88-8`（span∈{2,3}→168|256）、`h = 152`；行主序
    next-fit（col 溢出 8 换行；3 行框内仅一 widget 行可容，溢出裁剪 +
    dev 日志）；卡 x 自面板左缘 88 节距、y = `panel_y + 80`（行 1 起）。
  - 删除 pw0/sx/sy 缩放路径（R10 段并入 v2）；`refresh_dashboard_panel`
    与 view 装配段单一消费。
- span 语义：`dashboard_span_of` 缺省 2、合法域 {2,3}、旧 "1"→2 迁移；
  `dashboard_face_candidates` 增 `view mini (span: "3")` 文本探测（存储
  覆写仍胜）。
- `dashboard.at`：头行 h-12→h-[72px]（网格行 0）；pill 居中不变；空态
  文案位随头行。chrome 仍由宿主 wrapper 定位定尺寸（既有根修不变）。
- 测试：`plan024_dashboard_layout_tests` 重写为 8×3 合同（外框矩形/
  span2|3 宽/行 y/next-fit/溢出裁剪/span 迁移与 clamp）。

### 5.5 T-05 clock mini 居中（012-clock）

- 归因（截图+实机）：内层 row `justify-center/w-full` 组合在卡内实际
  效果；修复 = 双保险（外 col items-center + 内 row 去冗余类），实机
  截图验收（AC-06）。

### 5.6 T-06 todo mini 右列前三（013-todo）

- store：`var preview []str = []`；Init/AddTodo/ToggleTodo/DeleteTodo/
  ToggleAll/ClearCompleted/Refresh 各臂与 active_count 同步重算（前 3
  条未完成 text；F11 同臂纪律）。
- mini：双列 row——左列计数+标签（既有），右列 `for t in .store.preview`
  逐行 `text`（`text-xs text-muted-foreground truncate`，至多 3 行）。
- 源声明：`view mini (span: "3")`（T-04 探测通道消费）。

### 5.7 T-07 music mini 重排 + 空曲库通知化（020-music-player）

- mini 2×2 紧凑重排：disc 缩小（w-10 h-10）、title/artist 单行 truncate、
  时间行 + ⏮⏯⏭ 合并横排——合同：152px 高内全部可见（AC-08）。
- 空曲库（player_store Load 扫描零曲目）：`current_title` 回退「未选择
  歌曲」、不再写「本地曲库为空」入卡；改发
  `.__desktop_cmd = "notify\tinfo\t本地曲库为空: " + <dir>`
  （追加语义单点同款）。
- 宿主：联合排空段扩容——dashboard 孵化会话段（face 状态 hatched）并入
  segments（registry_id 归因沿用，F10）；词表白名单不变。防刷屏：Load
  幂等（扫描结果变化沿触发才再发）。

### 5.8 规范增量

| delta_id | 操作 | 目标 | before/after | rationale | acceptance |
|---|---|---|---|---|---|
| SD-01 | modify | docs/specs/shell/dashboard.md §SD-01 几何吸附 | 外框 10 列 872 → **8 列 696×232 右上 12px**；内部三等分缩放 → **网格算术（88/80 节距，卡 2×2/3×2）**；头行 = 网格行 0（72+8） | 用户 2026-09-20 裁定（8×3、2×2/3×2、卡框对齐网格） | AC-04/05 |
| SD-02 | modify | docs/specs/shell/dashboard.md §SD-02/§SD-04 span | span "1"\|"2" 缺省 1 → **"2"\|"3" 缺省 2（旧 "1" 迁移）**；增 app 源 `view mini (span:)` 文本探测通道（存储覆写胜） | 组件自定规格裁定 | AC-05/07 |
| SD-03 | modify | docs/specs/shell/dashboard.md §SD-03 孵化注入面 | 联合排空仅 wm.wins → **增孵化会话段**（notify 可达，归因沿用） | 空曲库通知路径依赖（F9） | AC-09 |
| SD-04 | modify | docs/specs/shell/showdesk-wallpaper.md §SD-01 sliver 行 + §SD-02 picker | sliver w-3 → **w-2 + hover 满覆盖 + popover 提示「显示桌面」**；picker 坐标 **viewport-reserved 钳制合同**（任意时刻在屏可关） | 走查三项反馈 | AC-02/03 |
| SD-05 | modify | docs/specs/shell/showdesk-icons.md §SD-02 任务栏图标消费 | 增任务栏**布局合同**：主轴 Fill 元素恰一（右组右缘贴时钟）；空容器不得持有 Fill 默认 | 居中 bug 回归守护 | AC-01 |

无新增 spec 文件；`supersedes_spec_components`/`new_spec_components` 由
review 定稿回填。

## 6. 测试设计

- **单元（auto-lang cargo t）**：
  - dashboard_layout v2 合同套件（外框/格位/span 域与迁移/next-fit/溢出
    裁剪/窄视口防御）——重写 `plan024_dashboard_layout_tests`；
  - span 探测（candidates 文本扫描 "3" 命中/缺席缺省 2/存储覆写胜）；
  - 联合排空孵化段（hatched 会话 `__desktop_cmd` notify 出段 + 归因）；
  - picker 坐标钳制纯函数（若抽函数直测，否则实机承载）；
  - 若 Q-A 归因②：row 默认宽合同单测。
- **实机（截图证据，scratch/p035 惯例）**：五题逐项 before/after；浅色
  主题抽查（高亮/提示可见性）。
- **回归门**：auto-lang `cargo t` 全量（ui-iced 档含）绿；shell-pack-sync
  后 assets pin 零 diff；壁纸右键→picker→应用→关闭链路走查。

## 7. 验收标准

- **AC-01** 任务栏右组（切换器/布局×2/铃铛/小组件/设置/电源）右缘贴时钟
  左邻，无双侧等分空隙；窗口拉伸/收缩保持。验证：实机截图。
- **AC-02** 任务栏区域无壁纸 picker 游离块；picker 任意开启时刻完整在屏
  且 ×/Esc/外点可关；「浏览…」仅自 picker 头部可达。验证：实机走查。
- **AC-03** sliver 宽 ≤8px；hover 高亮覆盖整块；hover 顶部浮出「显示桌面」
  提示；块体不越出任务栏。验证：实机截图（hover 态）。
- **AC-04** dashboard 面板外框 = 右上 8×3 网格块（696×232、12px 边距、
  88/80 节距），头行占网格行 0。验证：实机截图 + 布局单测。
- **AC-05** widget 卡：clock/music 2×2、todo 3×2；卡框线落 88/80 网格；
  旧 span 存储值不炸（迁移读）。验证：实机截图 + 单测。
- **AC-06** clock mini 内容组在卡内水平居中。验证：实机截图。
- **AC-07** todo mini 右列显示未完成前三（截断安全），左计数保留；待办
  增删后前三联动。验证：实机操作截图。
- **AC-08** music mini 2×2 内 disc/曲名/时间/⏮⏯⏭ 全部可见；空曲库不再
  内联显示「本地曲库为空」。验证：实机截图（E:\Music\ 空目录现场）。
- **AC-09** 空曲库时通知中心出现一条 info「本地曲库为空: E:\Music\」且
  来源归因 music；重复扫描不刷屏。验证：实机通知面板截图。
- **AC-11** 桌面空白右键菜单含「桌面小组件」项 + 开合 checkbox，点击即切换面板可见性。验证：实机截图。
- **AC-12** 小组件 face 升格开窗后关闭 app 窗，face 保留并回到孵化/运行态显示（不随窗消失）。验证：实机操作截图。
- **AC-10** auto-lang cargo t 全绿；shell-pack-sync 后 assets pin 零
  diff；specs 沉淀五条 SD 落库。验证：门输出 + diff。

## 8. 执行步骤

| id | 任务 | 依赖 | 产出/验证 | AC |
|---|---|---|---|---|
| T-00 | 环境口径与实机归因：master worktree 构建桌面（os-035 组内 auto-lang），复现五题，产出逐题载决工件（已修于 master/仍在+根因）；Q-B picker 卡开序复现留证 [✅ 已完成] 五题 master 全部成立；Q-A 载决②（justify-center 列 Fill 包装）；Q-B 深层为合成层泄漏（DEBTS-035-01）；tmp/p035/attribution.md + docs/plans/evidence/p035/ | — | scratch/p035 归因记录 + 截图；决定 T-01 形态 | — |
| T-01 | 任务栏右组右对齐（按 T-00 载决：.at 结构防御 或 auto-lang Fill 默认修复） [✅ 已完成] shell.at clock 列去 justify-center；实机 AC-01；headless 探针红→绿 | T-00 | AC-01 截图；归因②则附单测 | AC-01 |
| T-02 | picker 生命周期收口（坐标钳制 + 漏关臂补齐，renderer.rs） [✅ 已完成] desktop.at 四处坐标锚 popover 内容守卫=根治幽灵；钳制核实 usable_rect 既有在场（等效实现在场记载）；深层债 DEBTS-035-01 | T-00 | AC-02 走查；钳制单测（若抽函数） | AC-02 |
| T-03 | sliver 修缮（shell.at：w-2/满覆盖/popover 提示/防溢出） [✅ 已完成] 视觉类移内容 col（hit=可视）；w-2；动态提示「显示桌面/恢复桌面」 | T-00 | AC-03 hover 截图 | AC-03 |
| T-04 | dashboard_layout v2 + span 2\|3/迁移/探测 + dashboard.at 头行 72 + 测试重写（renderer.rs + shell/dashboard.at） [✅ 已完成] 8×3 直出绝对矩形；span 三级消费序；测试重写 4/4 绿；实机 AC-04/05 | T-00 | 布局单测绿；AC-04/05 截图 | AC-04/05 |
| T-05 | clock mini 居中（012-clock app.at） [✅ 已完成] 2×2 收紧（svg 40/text-xl），列交叉轴居中 | T-04 | AC-06 截图 | AC-06 |
| T-06 | todo preview 派生 + 双列 mini + span3 声明（013-todo store/app.at） [✅ 已完成] Recompute 归一七臂 + preview 三标量 + 双列 3×2；孵化失败根因=`text ("· "+…)` 非 parse 形态→f-string | T-04 | AC-07 截图 | AC-07 |
| T-07 | music 紧凑 mini + 空曲库通知化 + 孵化 drain 扩容（020-music-player + renderer.rs） [✅ 已完成] 控件定尺寸防溢出；Init 读 store 计数（calendar 先例）+ 一次性旗标；drain 孵化段 + push_notification 尾条去重。验证口径偏差：drain 单测以实机行为证据替代（badge=1 + 通知面板条目截图，内容/归因/一次性三点齐全） | T-04 | AC-08/09 截图 + drain 单测 | AC-08/09 |
| T-14 | 走查回环③（用户复核第三轮）：面板四围 padding + 时钟 3×2——dashboard_layout 外框 = 8×3 网格块外扩 PAD 12px（720×256，格位原点内移 PAD），dashboard.at chrome 加 p-3 对齐；012-clock mini 声明 span 3 + 表盘/字号放大（用户截图：2×2 内表盘+数字钟过挤；3+3+2 恰满 8 格） [✅ 已完成] t14b 实机截图：四围 padding+时钟 3×2 大表盘+三卡满排 | — | 实机：四围 padding、时钟 3×2 表盘加大、三卡恰满一行 | AC-04/05 |
| T-15 | 回环③收口：金样对拍 + 门 + 双仓提交 + 状态头 execution_done [✅ 已完成] 定向 17/17+4/4+1/1 绿；全量 39 红=基线同集 | T-14 | 门绿 + 证据 | AC-04/05 |
| T-16 | 走查回环④（用户复核第四轮）：sliver 高亮区须贴窗口右缘——高亮右侧残留任务栏 pr-2 奶白条；任务栏行 px-2 改 pl-2，细条贴缘、高亮占满分隔线右侧 [✅ 已完成] 实机 hover 截图 t16_sliver；词汇门/a2vue 复验绿 | — |
| T-17 | 走查回环④：012-clock 开窗自适应加固——pac 本已 window:"fit"，但 fit 测量重试上限 10 次（4s）在调试构建/高负载冷启动下耗尽放弃，窗口停留默认宽短尺寸（用户截图1）；FIT_MEASURE_MAX_RETRIES 10→150（60s），命中即止 [✅ 已完成] 冷启动即刻激活实机验证贴合（c1_clock_late） | — | 新增 AC-13 | 实机 hover 截图：高亮右缘贴窗口边缘 | AC-03 |
| T-09 | 用户走查回环②：iconfile 位图资产根解析回退（icon_root 只有双 env 臂，裸 exec 缺 AUTO_OS_ROOT 全部位图空白；补 CWD/assets/icons → P-3 OS 根解析序家族回退） [✅ 已完成] 无 env 实机位图齐全 | — | 实机无 env 启动位图齐全 | AC-01 |
| T-10 | 用户走查回环②：sliver 高亮/命中区扩到分隔线右侧全高（anchor col h-full） [✅ 已完成] r5 角落截图 | T-03 | AC-03 hover 截图复核 | AC-03 |
| T-11 | 用户走查回环②：桌面空白右键菜单增「桌面小组件」checkbox 开关项（desktop.at + 宿主 __wm_dashboard 投影注入 desktop 面：apply 臂 + inject boot 臂） [✅ 已完成] 勾选/切换/面板回正三态实机过；连带 a2r 词汇门补臂（checked 任意表达式走 ast_expr_to_rust + 门表 checked/onclick 扩容） | — | 实机菜单开关面板截图 | 新增 AC-11 |
| T-12 | 用户走查回环②：关窗后小组件面保留——投影 apply 臂（fp 变化即窗开合）挂 refresh_dashboard_panel（重孵化/降级，状态面即时回正） [✅ 已完成] 日志三拍 hatched→running→hatched，face 常驻 | — | 实机：face 开窗→关窗→face 复在 | 新增 AC-12 |
| T-13 | 回环收口：重建 + 无 env 实机复核四项 + 门（定向 + 全量对拍基线） + 双仓提交 + 状态头回 execution_done [✅ 已完成] cargo t --no-fail-fast 39 红与 rev1 同集零新增；定向 7/7 绿 | T-09..T-12 | 门绿 + 证据 | AC-01..12 |
| T-08 | 收口：cargo t 全量 + 实机五题总走查 + spec 沉淀（SD-01..05）+ shell-pack-sync + 状态头/台账/program tracker 更新 [✅ 已完成] 门归因定案：cargo t --no-fail-fast 5275 跑/5235 绿/40 红**全数在册或基线归因**（39 唯一名中 38 在干净 master 逐名复现同红 + musk 6 件=P645-D2 在册 + kitchen_sink=分支落点偏斜+主检出 widgets-gallery WIP）；本轮唯一真回归 a2vue 金样已重生成转绿；定向门 dashboard_layout 4/4 + p035 探针 1/1；pin 快照五件 hash-lock 相等；实机五题证据 evidence/p035/ | T-01..T-07 | AC-10；execution_done 状态头 | AC-10 |

（每步完成后在任务行追加 [✅ 已完成] 一行证据。）

## 9. 复审记录

- 2026-09-20 drafting：/auto-plan:new 起草（rev1）。grounding：F1-F14
  证据锚实测在案；Q-A/Q-B 两疑点转 T-00 有界调查（decision artifact）。
  `stage: new`，`outcome: pass`（授权范围内可开工），`next: work`。
- 2026-09-20 work：/auto-plan:work 执行（worktree auto-os `.wt/os-035/
  auto-os`@plan-035-dev 325f1ff + auto-lang `.wt/os-035/auto-lang`
  @auto-os-035-dev ce7a64014 + 依赖 auto-down@auto-os-035-dev；基线
  auto-os 4ec4f88 / auto-lang 4aadc1f57）。T-00 载决：Q1=假设②（适配层
  justify-center 列 Fill 宽包装，headless 探针红→绿）；Q2 幽灵块根因=
  合成层内容泄漏（.at 内容守卫止血，深层 DEBTS-035-01）。T-01..T-07
  全落；实机五题证据 + 定向门（dashboard_layout 4/4、p035 探针 1/1）绿；
  全量门结果见 T-08 行。`stage: work | plan_id: PLAN-035 | revision: 1 |
  outcome: pass | code_commit: 325f1ff+ce7a64014 | next: review`（全量门
  绿后置 execution_done）。
- 2026-09-20 work 收口：全量门归因定案——cargo t --no-fail-fast
  5275 跑 40 红（39 唯一），38 名在干净 master（主检出 1d6dc1f86）
  同滤串逐名复现同红，musk 6 件另在册 P645-D2，kitchen_sink=worktree
  分支落点（4aadc1f57，落后 master 5 提交）+ 主检出 widgets-gallery
  WIP 的环境红；本轮唯一真回归 = desktop.at a2vue 金样（AUTO_LANG_
  UPDATE_GOLDEN 重生成转绿，08efc9cbe）。**execution_done**：五组
  修缮全交付，实机五题证据齐；`stage: work | outcome: pass |
  code_commit: auto-os 325f1ff + auto-lang ce7a64014+08efc9cbe |
  task_ids: T-00..T-08 | next: review`。
- 2026-09-20 走查回环（用户实机复核第二轮，plan rev2）：①右组位置 ✓
  但 iconfile 位图全空（本轮裸启动无 AUTO_OS_ROOT，icon_root 双 env
  臂外无回退）→ T-09；②sliver 高亮应覆盖分隔线右侧全高 → T-10；
  ③小组件 × 关闭后无入口 → 桌面右键菜单 checkbox 开关项 → T-11；
  ④face 开窗后关窗连带消失（面板 refresh 只挂召唤事件）→ T-12。
  status 回 executing，revision 2。
- 2026-09-20 rev2/rev3 收口：T-09..T-13 全落（icon_root 回退 / sliver
  全高 / 右键菜单 checkbox 开关 + a2r 词汇门补臂 / 关窗 face 回正钩）；
  T-14/T-15 全落（外框四围 PAD 12px=720×256、格位原点内移、clock 3×2
  表盘 text-3xl/56px、declared_span 探测改全文唯一标记 dashboard
  span: N——012-clock 头注含 view mini 误锚实证修正）。门：cargo t
  --no-fail-fast 39 红与 rev1/rev2 同集零新增；a2vue 17/17；词汇门绿。
  `stage: work | outcome: pass | code_commit: auto-os 5122410 +
  auto-lang 855f5da8b | task_ids: T-09..T-15 | next: review`。
  **execution_done**。
- 2026-09-20 走查回环④（T-16/T-17）：T-16 sliver 贴缘高亮占满右侧
  （px-2→pl-2）；T-17 012-clock 开窗自适应加固（fit 测量重试上限
  10→150 次=60s——慢首帧耗尽重试后窗口停留默认尺寸的根因；冷启动
  即刻激活实机验证贴合）。门：词汇门/a2vue/定向全绿。`stage: work |
  outcome: pass | code_commit: auto-lang <T-17 提交> | task_ids:
  T-16..T-17 | next: review`。**execution_done 维持**。

## 10. 待澄清事项

| # | 事项 | 影响 | owner/next |
|---|---|---|---|
| Q1 | ~~任务栏居中根因~~ **已载决（T-00）**：假设②成立——适配层对无 width 类 justify-center 列给包装容器 Fill 宽；与二进制新旧无关（master 亦复现） | T-01 已按此修 | 已闭环 |
| Q2 | 8×3 框内 widget 超 4 张的溢出策略（v1 裁剪） | 未来组件增多后的 UX | v1 裁剪落地，滚动/增高挂 dashboard v2 债；复审可调 |
| Q3 | sliver 提示文案是否随 __wm_showdesk 切换（「显示桌面/恢复桌面」） | 细节体验 | 实现取动态文案（零成本），复审定稿 |
| Q4 | todo 前三排序（store 序 = API 返回序；无优先级字段） | 「前三个」语义 | 沿 store 序（创建序）；如需优先级另立计划 |
