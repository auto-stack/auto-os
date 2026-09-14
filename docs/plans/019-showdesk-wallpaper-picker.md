---
plan_id: PLAN-019
status: executing             # drafting → executing → execution_done → reviewed → archived
feature_name: showdesk-wallpaper-picker
author: [zhaopuming]
created_at: 2026-09-14
updated_at: 2026-09-14

# /auto-plan:review 结束时填写：
supersedes_spec_components: []
new_spec_components: [docs/specs/shell/showdesk-wallpaper.md]
touched_goals: []             # 引用 docs/specs/goals.md 的 GOAL-NNN

affects: [shell/desktop.at, shell/shell.at,
          auto-lang/schema/projection-protocol-v1.md,
          auto-lang/crates/auto-lang/src/ui/session.rs,
          auto-lang/crates/auto-lang/src/ui/iced/renderer.rs,
          docs/specs/shell/showdesk-wallpaper.md]
current_step: 7
total_steps: 7
---

# [PLAN-019] showdesk-wallpaper-picker

## 0. 变更摘要

虚拟桌面壁纸切换方案重设计（替换现有「设置面板手输路径 + 文件名按钮」的繁琐流程）。
三个自足能力 + 一个组合入口：

1. **显示桌面（负一屏）**：任务栏右缘 sliver toggle——切换到保留的空工作区
   （无 App 窗、图标与壁纸照常），再点返回 origin 分区。语义与 Win11「显示桌面」
   一致，但实现不用最小化全部（用户裁定否定 win_min 同族），用既有多分区机制。
2. **壁纸选择 carousel**：负一屏上浮现的候选壁纸栅格（宿主注入目录扫描结果），
   点击即应用热生效、当前壁纸高亮、大图预览二级态、←/→ 轮换对比、Esc 逐级退回。
3. **组合入口**：桌面右键两处「更换壁纸…」= `wallpaper_pick`（切负一屏 + 开
   picker + return_on_close）；关闭时自动回 origin。归属规则用户裁定定案：
   **谁切屏，谁负责切回**——组合调用自动返回；用户自己经 sliver 进入负一屏后
   开 picker，关闭只关 picker 不代管返回。
4. **每壁纸图标布局记忆**：`shell.desktop.positions` 按壁纸键控，切壁纸布局
   跟随（配合「图标不挡壁纸主体」的真实选壁纸需求）。

协议增量：projection-protocol v1.6 → v1.7（§4 动词 +5、§2 字段 +5）。
与本仓 PLAN-014 W-05「显示桌面」（win_min 同族最小化语义）冲突，本计划接管
该项语义（见 §10-1）。

## 1. 目标

- G1 显示桌面：sliver 点击 ⇄ 负一屏，origin 往返无损（窗口/焦点/分区原样）。
- G2 壁纸 carousel：负一屏上点击缩略图立即换壁纸（真桌面即合成预览——图标
  在新壁纸上实时可见），大图预览与 ←/→ 轮换支撑构图对比。
- G3 组合入口与返回归属：`wallpaper_pick` / `wallpaper_close` 两臂实现
  「谁切屏谁负责切回」。
- G4 目录浏览：picker 内「浏览…」弹原生目录对话框，单目录语义
  （`wallpapers_dir` 单源不变）。
- G5 布局记忆：壁纸 A/B 各自的图标摆布互不覆盖、切换跟随、重启保持。

**非目标**（记录债务，不实施）：
- vue 轨对拍（v1.7 协议文档承载 vue 端后续实现合同，沿 v1.6 先例注记）。
- iced Swiper/carousel 后端（负一屏全屏空间下栅格形态已够用，见 §5-3）。
- os-config 设置面板改造（既有 WallpaperPicker 保留为高级路径）。
- 图标排布自动适应壁纸内容（显著度分析 + 网格求解，另立 plan）。
- 多显示器。

## 2. 架构方案

分层不变（WM-as-app，桌面 shell 是特权 AutoUI app），改动三个层面：

- **宿主（auto-lang crates/auto-lang/src/ui/）**：`WmState` 增负一屏簿记
  （保留分区下标 + origin + picker 开关 + return 归属旗标）；DesktopBus 动词
  执行臂 +5；注入面 +5（负一屏态/sliver 判据/picker 五面）；全局键臂消费
  ←/→/Esc（picker 开时）。
- **shell 层（auto-os shell/）**：`shell.at` 任务栏右缘 sliver（toggle 判据
  `__wm_showdesk`）；`desktop.at` picker 全屏层（栅格态/预览态状态机 + 菜单臂
  改线）+ 每壁纸布局消费面零改动（格子注入面不变）。
- **合同（auto-lang schema/projection-protocol-v1.md）**：升 v1.7，§2 字段表
  与 §4 动词表增量，§6 changelog。

**两个操作一个组合**（用户定案）：

```
sliver 点击        → show_desktop / showdesk_return（WM 自足操作）
桌面右键更换壁纸   → wallpaper_pick（= show_desktop 幂等 + picker 开 + return_on_close=1）
picker 关闭        → wallpaper_close（picker 关 + 若 return_on_close → showdesk_return）
选壁纸/换目录      → 复用既有 set_wallpaper / set_wallpapers_dir（热生效通道现成）
目录对话框         → wallpaper_browse_dir（宿主 rfd pick_folder 后走同一目录写臂）
```

关键时序裁定：`wallpaper_pick` 到达时若 current 已是负一屏（用户自入），
**不覆盖 origin 且 return_on_close=0**——归属规则在宿主单点收口，.at 侧无分支。

## 3. 技术栈

纯 .at（shell 层）+ auto-lang crates Rust（宿主层）。目录对话框用 rfd
（`ui-dialog` feature 已有，Plan 418 先例；`vm/native.rs` FileDialog 用点在案），
无新依赖。壁纸缩略图/大图经既有 `image` 元素 + `load_image_bytes` 本地路径
读取（renderer.rs:5595-5598），无新管线。

## 4. 需求分析与背景调查

**授权记录**（2026-09-14 会话裁定，真实授权）：
- UX 形态：负一屏方案（否定底部面板方案与全屏最小化方案）；显示桌面与
  carousel 为两个操作、更换壁纸组合调用；return 归属规则「谁切屏谁负责切回」。
- 范围：含每壁纸布局记忆；不含 vue 对拍/自动避让（见非目标）。
- 仓库/动作授权：auto-lang `crates/` 改动在本计划范围内（Category A 验证门档
  对改 crates 的工作**允许** cargo test）。

**证据清单**（已核实，文件:行）：

| 事实 | 位置 |
|---|---|
| 桌面右键两处「更换壁纸…」现发 `open_settings`（改线点） | auto-os shell/desktop.at:145,171,224-260 |
| `WmState` 分区表/current/add_workspace（负一屏落点） | auto-lang session.rs:605,625,627,895 |
| `SetWallpaper`/`SetWallpapersDir` 动词与执行臂（复用） | session.rs:1276,1310；renderer.rs:9987 附近 |
| boot 壁纸解析回退链（#hex/builtin/路径→目录首图→ricepaper） | renderer.rs:11181 `load_desktop_wallpaper` |
| 壁纸目录解析链 config→env→D:\Down\stella-os\wallpapers | renderer.rs:11224 `wallpapers_dir_or_default` |
| 目录扫描现成能力（picker 注入数据源） | renderer.rs:9426 `scan_wallpapers_dir` |
| 图片本地路径直读（缩略图 src 可用绝对路径） | renderer.rs:5560-5598 `load_image_bytes` |
| 图标位置存储 `shell.desktop.positions` csv 读/全量重写 | renderer.rs:11255,11346,11390 |
| 内置壁纸 include_bytes（缺省链不动） | renderer.rs:1476-1477 |
| 协议 v1.6 现状（§2 字段表/§4 动词表/§6 changelog） | auto-lang schema/projection-protocol-v1.md（300 行） |
| config.at 单源 + mtime 热轮询（set_wallpaper 热生效通道） | auto-lang desktop_config.rs:100,122 |
| 任务栏右端结构（sliver 落点：时钟右侧） | auto-os shell/shell.at:415-445 |
| activate 切到窗所在分区的既有行为（负一屏需先 return） | renderer.rs:24827 测试在案 |
| rfd 原生对话框既有依赖（`ui-dialog` feature） | auto-lang Cargo.toml:62-68,216 |
| **PLAN-014 W-05「显示桌面」= win_min 同族（语义冲突）** | auto-os docs/plans/014-shell-ux-polish-v2.md:87,159,283 |

**参考实现**：stella-os os-simulator（D:\Down\stella-os\os-simulator/index.html
:246-248,700-710,952-961）——底部滑上面板 + 缩略图条 + 点击即应用 + active
高亮 + 外点关闭。本计划吸收其「点击即应用/高亮/外点关」交互语义；呈现位置
按用户裁定演进为负一屏。

**仓库影响**：auto-lang（crates + schema 文档）、auto-os（shell/ + docs）。
auto-os-config 不改。

## 5. 详细设计

### 5-1 WM 负一屏（session.rs + renderer.rs）

`WmState` 增簿记字段（执行时若收拢为 `ShowdeskState` struct 允许，协议面不变）：

- `showdesk_ws: Option<usize>`——保留分区下标（懒建：首次 `show_desktop` 时
  `add_workspace()` 并记录；命名 "Desktop"（不给数字标签，投影中被过滤））。
- `showdesk_origin: Option<usize>`——进入前的分区；`show_desktop` 时
  current 已是负一屏则**不覆盖**。
- `picker_open: bool` + `picker_return_on_close: bool`——carousel 簿记。

动词执行臂（DesktopBus §4 词表，shell/desktop 发出、宿主 update 壳层消费）：

- `show_desktop`：origin 记录 → current = showdesk_ws。幂等（重复到达不迁移
  origin）。
- `showdesk_return`：picker 若开着先关（幂等清簿记）→ current = origin →
  origin/return_on_close 清零。
- `wallpaper_pick`：`show_desktop` 幂等臂 → `picker_open=1` →
  `picker_return_on_close = (进入前 current != showdesk_ws)`（归属规则单点）
  → 触发 picker 注入。
- `wallpaper_close`：`picker_open=0` → 若 `picker_return_on_close` 则走
  `showdesk_return` 臂。
- `wallpaper_browse_dir`：宿主 rfd `pick_folder`（父窗口句柄绑定沿
  ui-dialog feature；若宿主构建未启用该 feature，T-03 内的有界调查裁定启用
  方式）→ 选定后调 `SetWallpapersDir` 同一执行臂（目录写路径单一）。

排除规则（负一屏躲开常规分区导航）：

- `__wm_workspaces` 投影过滤 showdesk_ws 条目（pager 切换条不可见）。
- `workspace_next` 环切跳过 showdesk_ws。
- `workspace_close`/`send_to` 目标为 showdesk_ws → 拒绝（no-op + 忽略）。
- `activate`（桌面图标双击）：current == showdesk_ws 时**先 return 再
  activate**（否则新窗开在负一屏穿帮；renderer.rs:24827 既有行为是切到窗所在
  分区，负一屏上无窗，须先回 origin）。

### 5-2 协议 v1.7（schema/projection-protocol-v1.md）

§4 动词表 +5：`show_desktop` / `showdesk_return` / `wallpaper_pick` /
`wallpaper_close`（均无参）、`wallpaper_browse_dir`（无参；宿主弹原生目录
对话框）。`set_wallpaper\t<path>` / `set_wallpapers_dir\t<dir>` 复用不变。

§2 字段表 +5（宿主写，shell/desktop 只读消费）：

| 字段 | 类型 | 语义 |
|---|---|---|
| `__wm_showdesk` | str `"1"/""` | 当前分区 = 负一屏（sliver 高亮/toggle 判据） |
| `__wp_picker` | str `"1"/""` | picker 层可见性（desktop.at 条件渲染唯一事实） |
| `__wp_dir` | str | 当前壁纸目录（picker 头部展示） |
| `__wp_current` | str | 当前壁纸路径（缩略图高亮等式判据） |
| `__wp_items` | Obj 数组 `{name:str}` | 候选清单合同面；handler 侧消费走伴随平行字符串列表 `wp_paths`（下标读 path，B12 规避——464 `apps_*`/notes 同型）；由 `scan_wallpapers_dir`（renderer.rs:9426）供源 |

§6 changelog 增 v1.7 节；vue 端注记沿 v1.6 先例（本版实现 = vm 端，vue 端
后续按文档对拍）。

### 5-3 desktop.at picker 层（shell/desktop.at）

宿主注入五面 + 本地态 `preview_ix str`（"" = 栅格态；非空 = 大图预览态，
值为 `wp_paths` 下标串）。栅格态优先，全屏层结构：

```
if .__wp_picker == "1" {
    mouse-area (onclick: .PickerDismiss)          # 遮罩：点击 = 关闭
      col（半透明底 bg-black/60 w-full h-full）
        col（居中面板 w-[880px] bg-card rounded-xl border p-4）
          头部 row：目录路径 text（.__wp_dir）+「浏览…」钮（wallpaper_browse_dir）
                    + 关闭钮（wallpaper_close）
          栅格态（preview_ix == ""）：
            grid (cols: 4, gap: 12) for __wp_items →
              image（src = wp_paths[i]，B12 规避：view for 消费 Obj 数组 ok，
                     onclick 闭包参数走平行列表）+ 当前壁纸描边（path==.__wp_current）
          预览态（preview_ix != ""）：
            大图 + ‹ › 钮 + Esc/关闭回栅格
}
```

- 点击缩略图 → `__desktop_cmd = "set_wallpaper\t<path>"`（热生效；config.at
  mtime 轮询既有，壁纸层瞬时换纹理，图标不动——真桌面即合成预览）。
- 消息：`PickerPick(i)` / `PickerPreview(i)` / `PickerPrev` / `PickerNext` /
  `PickerDismiss` / `PickerBrowse`；Esc 链 = 预览态回栅格态、栅格态发
  `wallpaper_close`。键盘 ←/→/Esc 由**宿主**全局键臂消费（picker_open 时；
  iced 焦点在宿主侧，.at 无键盘原语——switcher 热键同层）：栅格态 ←/→ =
  逐张 `set_wallpaper` 轮换（flip 对比）；预览态 = 切游标。
- 菜单改线：`.MenuWallpaper` / `.MenuWallpaperBlank` 由发 `open_settings`
  改发 `wallpaper_pick`（desktop.at:255-231 两臂）；「显示设置」臂不动。
- 不用 Swiper：iced 轨无该后端（stdlib/aura 仅 ark/jet/vue spec），且负一屏
  全屏栅格一屏尽收候选，构图对比优于单张轮播（设计裁定，见非目标）。

### 5-4 shell.at sliver（任务栏右缘）

时钟之后、任务栏最右端：细长 mouse-area（`w-3 h-full`，左缘 `border-l`
分隔线；hover 高亮本地态 `sliver_hover`，dock_hover 同型）。onclick:
`.ShowDesktopToggle` → handler 按 `.__wm_showdesk` 等式发
`showdesk_return` / `show_desktop`。hover 提示（「显示桌面」小 popover）v1
先落 hover 底色 + 分隔线，文本 tooltip 作 stretch（不设验收门）。

### 5-5 每壁纸图标布局记忆（renderer.rs 桌面图标臂）

- 键控：缺省底稿 `shell.desktop.positions`（现键，兼容回退）+ 新键
  `shell.desktop.positions.wp/<fp>`；`fp` = 归一化壁纸路径的稳定十六进制
  指纹（算法执行期定，T-06；规则：大小写/分隔符归一 + FNV-1a，落 spec）。
- 写时机（`SetWallpaper` 执行臂内，单一收口）：apply 前把当前格子表落盘到
  **旧壁纸**键（当前壁纸若无键控记录则先补写）→ 读**新壁纸**键（缺席回退
  缺省底稿）→ 重注入 `__desktop_cells` 全组。
- 拖拽落格（`desktop_icon_drop_at`，renderer.rs:11390 全量重写处）改写
  当前壁纸键（缺省底稿不再被拖拽更新——仅无键控壁纸时兜底写）。
- flip 对比语义：picker 连续点选时布局跟随各壁纸记忆跳位 = 所见即所得
  （每张壁纸的构图即其记忆布局）。

### 5-6 规范增量

| delta_id | add/modify/retire | docs/specs/... target | before/after rule | rationale | acceptance IDs |
|---|---|---|---|---|---|
| SD-01 | add | auto-lang schema/projection-protocol-v1.md §4 | 动词词表 +show_desktop/showdesk_return/wallpaper_pick/wallpaper_close/wallpaper_browse_dir | 两操作一组合协议化，跨端合同 | AC-01..04,07 |
| SD-02 | add | auto-lang schema/projection-protocol-v1.md §2 | 字段表 +__wm_showdesk/__wp_picker/__wp_dir/__wp_current/__wp_items(+wp_paths 平行面) | sliver/picker 投影面 | AC-03,09 |
| SD-03 | add | auto-os docs/specs/shell/showdesk-wallpaper.md | 新模块 spec：负一屏语义（排除规则/origin 簿记）、picker 交互合同、return 归属规则、每壁纸布局键控（fp 规则） | 沉淀本计划全部用户裁决 | AC-01..08 |

本计划无 retire 项；`open_settings` 动词保留（「显示设置」臂仍用）。

## 6. 测试设计

**宿主单测（auto-lang crates，Category A 允许）**——沿 renderer.rs 既有
`projection_v1*` / `workspace_v11_*` 测试家族扩展：

- showdesk 状态机：origin 记录/幂等（重复 show_desktop 不覆盖）/return 迁移/
  picker 开着时 return 先关。
- 排除规则：workspaces 投影过滤、workspace_next 跳过、close/send_to 拒、
  负一屏上 activate 先 return（对照 renderer.rs:24827 既有语义）。
- wallpaper_pick/close 组合簿记：return_on_close 两分支（他分区进入=1、
  负一屏进入=0）。
- 每壁纸 positions：fp 键控 roundtrip、apply 换键迁移（旧键补写/新键回退
  底稿）、拖拽写当前键。
- picker 注入面：items/paths/current/dir 四面投影 + 指纹门控（窗态翻转刷新）。

**合同对拍**：schema_drift / v1.7 字段表与 sync 实现一致（投影测试家族内）。

**实机冒烟**（desktop MCP 截图脚本，沿 025-sys-monitor tests 套路）：
sliver→负一屏→picker→点选→自动回 origin 全链截图；双主题各一轮。

## 7. 验收标准

| ID | 可观察行为 | 验证方法 | 期望 |
|---|---|---|---|
| AC-01 | sliver 点击切到负一屏（无窗空桌面，图标+壁纸在）；再点返回原分区，窗口/焦点原样 | 实机冒烟脚本 + 状态机单测 | 往返无损，无窗丢失/焦点漂移 |
| AC-02 | 负一屏不出现在 pager 切换条与 workspace_next 环；close/send_to 拒绝；负一屏上双击图标 → 回 origin 并在 origin 启动 | 宿主单测（排除规则组） | 全部按 §5-1 规则 |
| AC-03 | 桌面右键「更换壁纸…」→ 切负一屏 + picker 浮现；点缩略图立即应用（config.at `wallpaper_path` 落盘、桌面即时换装、图标在新壁纸上可见）；选完/Esc/遮罩关闭自动回 origin | 单测（组合臂）+ 实机冒烟 | 全链成立；config 单源写 |
| AC-04 | 用户 sliver 自入负一屏后再开 picker → 关闭只关 picker，**不**切回 | 单测（return_on_close=0 分支） | 归属规则成立 |
| AC-05 | 预览钮/双击进大图预览，‹›切换，Esc 逐级退回（预览→栅格→关闭） | 实机冒烟 + .at 状态机走查 | Esc 链每级成立 |
| AC-06 | picker 开时 ←/→：栅格态逐张应用轮换（flip 对比）；预览态切图 | 宿主键臂单测 | 两态各自成立 |
| AC-07 | 「浏览…」弹原生目录对话框；选定后列表重扫、`__wp_dir` 更新、`wallpapers_dir` 落盘热生效 | 有界调查结论 + 冒烟 | 对话框可达、写路径单一 |
| AC-08 | 壁纸 A 摆布局 → 切 B 摆另一布局 → 来回切换布局跟随各自记忆；重启保持 | 单测（键控迁移）+ 手动验证 | 记忆互不覆盖 |
| AC-09 | 协议 v1.7 文档（§2/§4/§6）与实现一致；投影/schema_drift 测试绿 | `cargo test -p auto-lang`（在 auto-lang；本计划改 crates 允许） | 全绿 |
| AC-10 | vue 轨对拍债务显式记录（v1.7 文档 vue 端注记） | 文档评审 | 注记在案 |

## 8. 执行步骤

（原子任务；每步完成后追加 [✅ 已完成] 一行证据。T-06 可与 T-03..T-05 并行。）

### T-01 协议 v1.7 文档增量
auto-lang `schema/projection-protocol-v1.md`：§4 动词 +5、§2 字段 +5
（含 wp_paths 平行面注记）、§6 v1.7 changelog + vue 端注记。
依赖：无。→ AC-09,10。

[x] [✅ 已完成] 提交 8580ef29e（§4 动词 +7：实现期增补 wallpaper_nav/
wallpaper_preview 已回填；§2 +__wm_showdesk + __wp_* 五面；§6 v1.7 节 +
vue 注记）。

### T-02 WM 负一屏 + 动词执行臂
session.rs `WmState` 簿记字段；renderer.rs 执行臂 `show_desktop` /
`showdesk_return`（排除规则四条 + activate 先 return）+ 投影过滤 +
`__wm_showdesk` 注入。单测：状态机组 + 排除规则组。
依赖：T-01。→ AC-01,02。

[x] [✅ 已完成] 提交 11b6ab81e。WmState showdesk/picker 七簿记字段；
排除规则五条全落（环切/删除守卫+压实跟随/发送拒/activate 先回）；测试
showdesk_state_machine_roundtrip / showdesk_exclusion_rules /
showdesk_projection_filter_and_flag 全绿。

### T-03 wallpaper_pick/close 组合臂 + picker 注入面 + 目录浏览臂
renderer.rs：组合簿记（归属规则单点）、`__wp_*` 五面注入（复用
`scan_wallpapers_dir`）、`wallpaper_browse_dir`（**有界调查**：宿主构建
`ui-dialog` feature 启用方式与 rfd 父窗口绑定，决策工件落 T-03 证据；回退
路径 = 头部目录输入行，os-config 先例）。
单测：组合两分支 + 注入面投影。
依赖：T-02。→ AC-03,04,07。

[x] [✅ 已完成] 提交 11b6ab81e。组合簿记归属规则单点（AC-04 两分支测试
wallpaper_pick_close_return_ownership）；inject_wallpaper_picker 五面
直写；browse_dir = rfd pick_folder（调查结案：ui-dialog 已随 ui-iced 启用，
v1 无父窗绑定）。

### T-04 desktop.at picker 层 + 键臂
shell/desktop.at：五面消费、栅格/预览状态机、遮罩/Esc 链、菜单两臂改线
`wallpaper_pick`；renderer.rs 全局键臂（←/→/Esc，picker_open 门控；执行时
核实与 switcher 热键注册点不冲突）。
依赖：T-03。→ AC-03,05,06。

[x] [✅ 已完成] 提交 0fd896a（auto-os .at）+ aedf73f35（auto-lang 键臂 +
assets pin 双写 + vue 金样重生成）。键臂 = 订阅层 PICKER_KEYS_OPEN 原子
门控 + picker_key_message 纯函数（关态完全穿透不吞键）；DesktopEvent
+WallpaperKeyNav/WallpaperKeyEscape。测试 wallpaper 族 5 绿 +
desktop_surface 4 绿（含真 pack 编译）。

### T-05 shell.at sliver
shell/shell.at 时钟后右缘细条 + toggle handler + hover 态。
依赖：T-02。→ AC-01。

[x] [✅ 已完成] 提交 0fd896a。w-3 细条 + border-l 分隔线 + sliver_hover
高亮；ShowDesktopToggle 按 __wm_showdesk 等式直发；shell pack 编译测试
14 绿（真 shell.at 装载）。

### T-06 每壁纸布局记忆
renderer.rs `SetWallpaper` 臂换键迁移 + `desktop_icon_drop_at` 写当前键 +
fp 算法（定案后落 SD-03 spec）。单测：键控组。
依赖：T-01（独立于 T-02..T-05）。→ AC-08。

[x] [✅ 已完成] 提交 820cdfde8。fp = FNV-1a 64hex + `\`→`/` + ASCII
小写折叠（落 SD-04）；键控读缺席回退缺省底稿；apply_drop 写当前壁纸键；
set_wallpaper 迁移臂（切前快照旧键/切后重注入）。测试
wallpaper_layout_key_and_csv_roundtrip + wallpaper_layout_memory_migration
绿；w5/desktop_injects 回归绿。

### T-07 对拍 + 实机冒烟收口
投影/schema_drift 全量绿；desktop MCP 冒烟脚本（双主题全链截图）；SD-03
spec 定稿（docs/specs/shell/showdesk-wallpaper.md）。
依赖：T-04,T-05,T-06。→ AC-09（终验）。

[x] [✅ 已完成] 提交 ef270c4c3（p010 预存红顺带修正）+ c77d9b7（SD-03
spec 草案）。全量日常档 --no-fail-fast = 4885/4903；18 失败全部基线实证
预存红（layout 族 14 环境几何 + c2_param/plan606/plan055/desktop_protocol
coverage + p010——后者过期期望已修正）。范围调整：shell 宿主无 VM/MCP
通道（per-app harness 不适用），实机冒烟以 SD spec「验证」节人工清单承载，
待复审/用户走查（AC-01/03/04/05/06/07/08 实机侧）。

## 9. 复审记录

- 2026-09-14 stage:new PLAN-019 rev1 outcome:pass — 起草完成。范围经会话
  三轮收敛（UX 形态→两操作拆分→return 归属规则），全部用户裁决已录入
  §4 授权记录与 §5 设计。任务覆盖全部 AC 与 SD；路径/行号经本仓与 auto-lang
  实地核实。next: work（`/auto-plan:work`，worktree
  `D:/autostack/.wt/os-019/auto-os -b plan-019-dev`）。
- 2026-09-14 stage:work PLAN-019 rev1 outcome:pass — 执行完成
  （T-01..T-07，current_step 7/7）。
  - **worktree/分支**：auto-os `D:/autostack/.wt/os-019/auto-os`
    （plan-019-dev，基线 a657a4e）；auto-lang `D:/autostack/.wt/os-019/auto-lang`
    （auto-os-dev，基线 a9d3b8c67）+ 依赖 auto-down（detached 140775f）。
  - **提交**：auto-lang 8580ef29e（T-01 协议 v1.7）→ 11b6ab81e（T-02/T-03
    WM+组合臂）→ aedf73f35（T-04/T-05 键臂+pack 双写）→ 820cdfde8（T-06
    布局记忆）→ ef270c4c3（T-07 p010 修正）；auto-os 0fd896a（.at 层）→
    c77d9b7（SD-03 spec 草案，备 merge 发布）。
  - **证据**：showdesk×3 + wallpaper×5（pick-close 归属两分支/nav 游标
    flip/Esc 链/布局键控迁移）新测全绿；shell 14 绿 + desktop_surface 4 绿
    （含真 pack 编译）+ w5/desktop_injects 回归绿；vue a2vue 金样重生成
    （AUTO_LANG_UPDATE_GOLDEN）；全量日常档 --no-fail-fast = 4885/4903，
    18 失败**全部基线实证预存红**（layout 族 14 = dock 几何环境依赖、
    c2_param/plan606/plan055/desktop_protocol coverage/p010 = 本机
    fixture 漂移——p010 过期期望 4 已顺带修正为 2+PickerDismiss 断言）。
  - **有界调查结案**（§10-2/3/4）：ui-dialog 已随 ui-iced 启用（rfd 直接
    可用，v1 无父窗绑定）；picker 键臂用订阅层原子门控（PICKER_KEYS_OPEN，
    关态完全穿透）；fp 算法定案 FNV-1a 64hex + `\`→`/` + ASCII 小写折叠
    （已落 SD-04）。协议 v1.7 实现期增补 `wallpaper_nav`/`wallpaper_preview`
    两动词与 `__wp_preview`（路径载荷）——设计细化已回填文档。
  - **残余（review 门）**：实机冒烟清单已落 SD spec「验证」节（shell 宿主
    无 VM/MCP 通道，per-app harness 不适用——范围调整记录在案）；AC-01/03/
    04/05/06/07/08 的实机侧待复审/用户走查确认。
  - next: review（`/auto-plan:review`）。

## 10. 待澄清事项

1. **PLAN-014 W-05 语义冲突**（owner: 用户，下次触碰 014 时裁定）：014 的
   「显示桌面」= win_min 同族最小化语义（014:87,159,283），与本计划负一屏
   语义冲突且入口同为空白菜单。建议 014 修订时移除该项、由本计划接管；
   本计划先行不阻塞（两 plan 分 worktree，合并序先 019 后 014 可免冲突）。
2. rfd 宿主启用方式（feature 门/父窗口句柄）——T-03 有界调查裁决，回退路径
   已备（目录输入行）。
3. ←/→ 键臂与既有全局热键表（switcher 等）的注册点关系——T-04 执行时核实，
   冲突则 picker_open 门控天然隔离。
4. 壁纸路径 fp 算法细节（归一化规则）——T-06 执行期定案并回写 SD-03 spec。
