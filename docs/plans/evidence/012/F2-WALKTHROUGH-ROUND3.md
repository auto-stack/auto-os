# PLAN-012 F2 走查第三轮证据摘要（2026-09-13，MCP 活体探针）

环境：ui_desktop @ .wt/os-012/auto-lang（debug，os-012-dev），
CWD=.wt/os-012/auto-os，隔离档案 %TEMP%/os012-f2（已还原用户底稿），
MCP :9348 + AUTOUI_ACCEPTANCE=1。截图均 2560×1600（2× 逻辑坐标）。

## O2 dock 去重（AC-07）

- **活体定性**：pinned=["012-stopwatch","016-calendar"]、csv=",012-stopwatch,
  016-calendar,"、stopwatch 窗 registry_id=Some("012-stopwatch")——宿主数据
  完全一致仍双图标（o2-dock-duplicates-before.png：pinned 时钟 + 窗口条目
  时钟并存）。根因 = `.at` view 条件求值器（eval_condition_with_inner）
  **无方法调用臂**，`.contains(...)` 静默塌缩恒 false（T9 发现②实锤升级；
  同族死点：shell.at `__wm_running.contains` pinned 灰条 ×1、齿轮高亮 ×1）。
- **修复**：宿主派生 `__wm_wins.pinned`/`dup_app` + `__dock_pinned.running`
  字段，shell 等式消费；`dup_app` 同步兑现"同类 app 共享一图标"（z_order
  首见之外不重复出条目）。
- **活体验证**（o2-dock-fixed.png / -zoom.png）：单组 dock 无重复；pinned
  时钟聚焦态（主色条+底色高亮）、pinned 日历运行灰条、三窗口条目各灰条。

## O3 通知面板贴顶（AC-03）

- **活体复现**（diff bbox）：面板卡片 y 0..798 满高——非布局塌缩单因：
  ① 历史列表无 max-h，6 条 ~110px 条目 ≈ 790px > 744 可用（mt-auto 空间
  归零）；② 真实链 Stack 子层 `h-full` 约束传递失效（headless 全链复刻
  ——真 .at 源 + 真组件管线 + Stack 装配——通过而实机塌缩）。
- **修复**：宿主注入 `__panel_h`（根列显式像素高）+ `__panel_max_h`
  （列表 max-h 滚动）；o3-panel-bottom-anchored-fixed.png：卡片贴底
  （dock 上方 ~60px）、有界、列表滚动截断、铃铛高亮同步。
- **金样**：layout_tests p012_o3_notification_layer_in_stack_anchor +
  p012_o3_notification_real_component_in_stack（少条目贴底 / 多条目
  max-h 有界双场景）。

## O1 图标字形居中（AC-10，部分修复 + 定案）

- **像素定案**（边框锚定 + 底条锚定互证）：真实链任务栏图标字形一致性
  偏左上 dx≈-2.7~3.0 / dy≈-3.0 逻辑 px（o1-glyph-bias-crosshair.png）。
- **机制定位**：① iced 0.14 button 布局 = padded 后 content **左上放置
  无居中**（iced_widget-0.14.2 button.rs 实读）——已修：icon-only 臂
  Fill×Fill+双向 center 容器（headless 与 auto-size 按钮均 no-op/正确）；
  ② 残留 ~3px = iced 0.14.2 svg 绘制管线 quirk（Fixed 尺寸与 content_fit
  实际行为均背离源码语义，探针 Fixed(40) 仍绘 intrinsic 24 偏左上）——
  **移交 iced 升级或预栅格化图标管线的有界后续**，本轮不投机改渲染器。
- AC-10 终裁维持 partial（残留 3px 可感知），不 weaken。

## 齿轮/铃铛两态（AC-12）

- gear-highlight-missing-before.png：os-config 运行、齿轮无高亮（:414
  contains 死点）。gear-and-bell-highlight-fixed.png：os-config 开 →
  齿轮高亮 ✓，通知面板开 → 铃铛高亮 ✓，电源/切换器关态未亮 ✓。

## 附带修复（验收通道基建）

- desktop 模式 primary 锚点迁移：特权 shell 先于直挂 comps 分配 + shell
  层 MCP 同步开启——autoui_state/autoui_vtree/截图自此观测 shell 投影面
  （O1 真实链探针前置条件）。
- shell_fields.window_size 随 tick 镜像宿主 viewport（层 App 无窗事件，
  MCP 截图曾被零尺寸守卫整体拒死）。
- **观察项（移交基建）**：① ServiceTick 臂 fit/快照早退可饿死注入排空
  （实测注入通道死亡 + 一次宿主窗 13×13 事件，7996/14304 两实例复现轨迹）；
  ② styled_vtree bounds 回填未覆盖非 primary 层；③ F2 档案 boot 自动
  拉起 recent/图标 app 现象未定界（001-helloworld/015-notes 等无指令自启，
  失败通知入史）。

## 门档

- scoped（--test-threads=1）：p012_o3 ×2 + projection_v16 + w1_/w2_/w5_/
  w7_/w8_ + dock_pin/unpin + shell_pack 族 = **13+33 全过**。
- 全量 iced 档：4620 过 / 196 败 = review 基线（master 存量，零新增回归）。
- shell-pack hash-lock：pin shell=31b591bcad desktop=63b36c4101
  switcher=bee9ea8dc8 notification_center=1258f3eb90（worktree 内嵌快照
  经 AUTO_LANG_ROOT 显式指向 worktree 同步——**勿从主检出跑 sync**）。
