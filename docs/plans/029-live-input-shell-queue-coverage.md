---
plan_id: PLAN-029
status: drafting               # drafting → executing → execution_done → reviewed → archived
feature_name: live-input-shell-queue-coverage
author: [agent]
created_at: 2026-09-18
updated_at: 2026-09-18
plan_revision: 1

# /auto-plan:review 结束时填写：
supersedes_spec_components: []
new_spec_components:
  - auto-lang/docs/design/autoui/desktop-protocol-v1.md   # §1.10 v1.10 增量（review 定稿）
touched_goals: []

affects:
  - auto-lang/crates/auto-lang/src/ui/session.rs                        # 键盘/滚轮/IME live 接线（desktop_window_events 扩臂 + 映射）
  - auto-lang/crates/auto-lang/src/ui/desktop_protocol/native_projector.rs  # popover/thumbnail/preview/mouse-area 臂
  - auto-lang/crates/auto-lang/src/ui/desktop_protocol/coverage.rs      # 四 kind 入册 + Popover 子树扫描递归 + 防漏钉
  - auto-lang/crates/auto-lang/src/ui/iced/broker_surface.rs            # lucide: 栅格化词汇扩展
  - auto-lang/crates/auto-lang/src/ui/iced/renderer.rs                  # lucide 栅格化机具接驳（lucide_svg_doc_with/resvg）
  - auto-lang/crates/auto-lang/src/ui/desktop_protocol/client_entry.rs  # 翻转数据门复测点（门后）
  - auto-lang/docs/design/autoui/desktop-shell-a2r.md                   # 前置序列更新（第一件已交付 → 本件）
  - auto-lang/docs/plans/KNOWN-DEBT-AND-RISKS.md                        # P025-D1 核销
  - auto-os/docs/plans/autos-desktop-program.md                         # 台账 3c2 行
current_step: 0
total_steps: 9
---

# [PLAN-029] live-input-shell-queue-coverage

## 0. 变更摘要

B 形态 shell 前置序列（desktop-shell-a2r.md §10-①）第一件（图像
DrawOp 通道，PLAN-028）已收官。本计划交付**第二件 = ②⑤ 键盘 live
接线与真机实测 + ② 覆盖二批收尾（shell queue 面五 kind）**，为
shell outproc client（B 程序主体）清场：

**①键盘/滚轮/IME live 接线**——普查实锤"真机实测"的前置是真实接线
工作：`desktop_window_events` 订阅零键盘/滚轮臂（session.rs:7086-7102），
`broker_key_event/char/ime_*/scroll` 六函数备好但**无事件源**（P025-D1
在册债）。本计划接线（iced 事件映射 Key::Named→VK u32 /
WheelScrolled→Scroll / Key text→CharTyped / IME 事件）+ 真机证据
（D2 定案口径；⑤协议级兜底延续）。
**②shell queue 面覆盖收尾**——shell pack 五件经 027 codegen 产出的
View 树含**五个 native queue 缺失 kind**：popover（×9，open 态在
View 字段——应用态驱动，投影器免状态机）、mouse-area（×13 元素）、
window_thumbnail（×3）/ workspace_preview（×1，解析侧 028 已备
`thumbnail://`，缺投影器桥接臂）、icon（→`lucide:` image，宿主
resvg 栅格化机具齐备——词汇扩展即真渲）。**③翻转数据门复测**
（026 数据 76.2% < 95% 维持 independent——四 kind 入册后复测，
dual-exit 同 026 口径）。

零 wire 变体（popover 覆盖序渲染复用 select 先例 paint-order 置顶，
零 overlay 协议语义；lucide/thumbnail 解析全在宿主侧词汇表内）。

## 1. 目标

- **G1 键盘/滚轮/IME live 接线**：宿主 iced 键盘/滚轮/IME 事件 →
  映射 → `broker_key_event/char/ime_commit/ime_preedit/ime_cancelled/
  scroll` → 焦点窗/命中窗 child 的**生产链真接线**（P025-D1 核销）；
  DesktopMessage 扩展与订阅位按 D1 定案。
- **G2 真机证据**：真 OS 事件到 child 状态变化的端到端证据（D2 定案
  口径：SendInput FFI 真机合成（键盘无坐标问题——P020-D4 阻断只伤
  点击）/ acceptance channel 扩 key verb / 用户手动留痕，⑤协议级
  兜底延续）。
- **G3 native popover 臂**：`View::Popover`（open=View 字段）——开态
  覆盖序渲染（select 先例 native_projector.rs:235-259：主块后追加
  ops，paint order 置顶）+ placement 解析（shell 实用集 + 全 14 枚举
  的覆盖/随注按 D3）+ Point/Widget 双锚 + 命中语义（外点/Esc →
  `on_dismiss` 消息派发）+ Modal scrim；`scan_native_node` 补
  Popover 子树递归（coverage.rs:562 缺口）。
- **G4 thumbnail/preview 桥接 + MouseArea 臂**：
  `View::WindowThumbnail{wid}` → `DrawOp::Image{src:"thumbnail://
  {wid}"}`（028 解析侧直用）+ miss fallback；`WorkspacePreview{ws}`
  形态按 D4（workspace:// 宿主合成 vs 网格 refs vs not-yet）；
  `View::MouseArea` 透传渲染 + 事件命中路由（shell ×13 click/
  contextmenu）。
- **G5 lucide: 词汇真渲**：宿主 `resolve_drawlist_image` 扩 `lucide:`
  前缀——`lucide_svg_doc_with`（全量表 1401 图标）+ resvg/tiny-skia
  栅格化（依赖已在、先例在测试 renderer.rs:29031）→ RGBA Handle
  缓存 + tint 着色 → `frame.draw_image`——dock/任务栏/桌面图标在
  queue 臂真渲（028 not-yet"字形线"的 lucide 半边收口）。
- **G6 覆盖收口 + 翻转复测 + 文档**：四 kind 入册 + 防漏钉双向更新；
  **shell pack 五件 View 树装载期 native 判定 Covered**（B 程序
  覆盖门的硬前置验证）；翻转数据门复测（dual-exit）；desktop-
  protocol-v1.md **§1.10 v1.10 增量**；desktop-shell-a2r.md 前置
  序列更新（第一件已交付标注 + 本件交付）；P025-D1 核销；台账
  3c2 行。

**非目标**（明确出界）：

- **shell outproc client 本体 + 启动序/看门兵**（B 程序主体——本计划
  是其直接前置；ShellProjection 过线/desktop.* wire 化随其立项）。
- **OS 级点击自动化**（P020-D4 维持——键盘无坐标问题不在阻断面，
  点击仍受阻）。
- **解释态投影臂扩 popover/thumbnail**（解释态 target_set 维持缺项，
  分表纪律 I4；解释态 inproc 直挂已有 popover widget 臂不受影响）。
- **svgdoc: 通用 SVG 词汇**（lucide: 同机具顺带与否按 D5-b 定；通用
  内联 SVG 独立线）。
- auto 缺省翻转执行（复测数据 dual-exit 落盘；翻转裁定维持数据门
  ——达标即翻的同口径仅当数据过门，见 AC-07）。
- 双投影器统一（P020-D1）；位图过线通道；web 真位图（028 not-yet
  三项维持）。
- IME 候选窗定位下发（ImeCursor 形态定案归 B 程序——shell 输入面
  需求驱动时再立）。

## 2. 架构方案

```text
┌─ live 输入臂（session.rs）─────────────────────────────────────────┐
│ desktop_window_events 扩键盘/滚轮/IME 臂（现仅 Window 事件        │
│   :7086-7102）：                                                    │
│   Event::Keyboard(KeyPressed{key,text}) → text 可打印 = broker_    │
│     char(CharTyped)；Key::Named → VK u32 映射 = broker_key_event   │
│   Event::Keyboard(Ime*) → broker_ime_commit/preedit/cancelled      │
│   Event::Mouse(WheelScrolled{delta}) → Lines×LINE_H 像素化/Pixels  ││   → broker_scroll(hit_test 命中窗)                                 │
│ 路由语义零新增：键盘/IME = session.focused 焦点窗（broker_* 既有  │
│   :3379-3482 六函数直用）；DesktopMessage 扩展位按 D1              │
│ 真机证据（D2）：SendInput FFI e2e（推荐）/ acceptance key verb /   │
│   手动留痕；⑤协议级兜底延续                                         │
└──────────────────────────────────────────────────────────────────┘
┌─ shell queue 面臂（native_projector.rs + coverage.rs）─────────────┐
│ Popover：open=View 字段（:931）——开态 = 主块后覆盖序 ops（select  │
│   先例 :235-259）+ placement 解析 + Point{x,y}/Widget 双锚命中 +   │
│   外点/Esc → on_dismiss 派发（零投影器开合状态机）+ Modal scrim    │
│ WindowThumbnail{wid} → Image{src:"thumbnail://{wid}"} + miss       │
│   fallback（fallback_icon 语义 D4）；WorkspacePreview{ws} 形态 D4  │
│ MouseArea：透明透传渲染 + 事件命中项（click/contextmenu 族）       │
│ coverage：kinds + popover/mousearea/windowthumbnail/               │
│   workspacepreview；scan_native_node 补 Popover 子树递归（:562）   │
└──────────────────────────────────────────────────────────────────┘
┌─ lucide 词汇臂（broker_surface.rs + renderer.rs 机具）─────────────┐
│ resolve_drawlist_image 扩 lucide: 前缀：lucide_svg_doc_with(name)  │
│   （:6122-6135，lucide_generated 1401 全量表）+ tint 着色 → resvg/ │
│   tiny_skia 栅格化（真依赖已备 :57/:221-222）→ RGBA Handle 缓存    │
│   → frame.draw_image；未知名 → 降级占位 + 观测（I3）               │
└──────────────────────────────────────────────────────────────────┘
```

**不变式**：

- **I1 追加式协议**：`PROTOCOL_VERSION` 维持 1，零新 wire tag——
  popover 覆盖序 = 既有 ops 语义；lucide/thumbnail = 宿主侧词汇表
  扩展（src 字符串空间，§1.9 词汇表增量入册）。
- **I2 零回归**：broker_* 六函数与路由语义零改动（只接事件源）；
  既有 kind/golden/防漏钉零漂移；解释态直挂 popover widget 臂
  （renderer.rs:4638/:6991）零牵连。
- **I3 not-yet 纪律**：lucide 未知名/未定案 placement/workspace
  形态 = 显式降级或随注，禁静默。
- **I4 双轨分表**：解释态 target_set 不加四 kind（I4 纪律）；
  native 表扩容独立。

**关键风险**：iced 0.14 daemon 形态下键盘事件的订阅可达性（iced
daemon 的 keyboard 事件流面——T-01 实勘，widget 级 PUA 处理
renderer.rs:8722 与全局订阅的边界）；SendInput FFI 的会话完整性
（合成事件进真实 OS 队列——e2e 门控 + 焦点窗前置断言）；popover
14 placement 的空间翻转几何（锚点+边距推导）；lucide 栅格化的
性能（每图标首渲栅格化 + 缓存命中后零成本——缓存 key 含
name+color+size）。

## 3. 技术栈

Rust / iced 0.14（daemon 事件流）；resvg 0.45 + tiny-skia 0.11
（ui-iced 真依赖已备）；lucide_generated 全量表（1401 图标）；028
交付的图像词汇/解析基建（resolve_drawlist_image/thumbnail://
snapshot SWR）；select 覆盖序先例；Windows SendInput FFI（如 D2
采纳——shm.rs 手写 FFI 先例同型，e2e 门控）；验收载体 = shell
pack 五件（auto-os shell/，View 树扫描断言）+ 003-converter/fixture
（键盘链）+ p026/p028 帧内定位法扩展。

## 4. 需求分析与背景调查

**授权记录**：用户 2026-09-18 会话（PLAN-028 收官交接）明确"下一件
按 desktop-shell-a2r 前置序列推进（025 键盘真机实测 → 覆盖二批），
需要时以 /auto-plan:new 立项"——本计划即该授权的立项（前置序列第
二件）。**本轮仅规划，未授权实施**。涉及仓：auto-lang（接线/投影
臂/词汇/文档）+ auto-os（shell pack 扫描载体/台账/e2e 腿）。无
预算/自动续跑约束声明。**无前置计划依赖**（028 已 merge 即基线）。

**现状事实**（已核，2026-09-18 master @ 02c04ad42 含 028，探索代理
全量普查）：

- **键盘 live 接线缺口（P025-D1 在册）**：`desktop_window_events`
  仅 Window 事件（session.rs:7086-7102）——零键盘/滚轮/IME 臂；
  broker_* 六函数实现完备无生产调用点（`broker_key_event` :3379
  `broker_char` :3399 `broker_ime_*` :3421-3482 `broker_scroll`
  :3487——键盘/IME 路由 `session.focused` 焦点窗、滚轮 hit_test
  命中窗）；KNOWN-DEBT :2266 P025-D1 原文"live 接线需 DesktopMessage
  扩展 + iced 事件映射另立"。
- **真机证据通道**：协议级先例 = p025_native_input_arm 经
  `broker_char` 宿主生产函数驱动（stage3.rs:1702-1710）；OS 级合成
  仓内零先例（无 SendInput/enigo）；acceptance channel 仅
  bus/handler 两动词（mcp_server.rs:1116-1126）——key verb 缺；
  ⑤口径原文（desktop-protocol-v1.md §1.7/§1.8）协议级承载兜底。
- **View::Popover**（view.rs:927-933）：anchor（Widget/Point
  :1007-1013）/content/placement（14 枚举含 Modal/Pointer
  :1018-1046）/**open: bool（应用态驱动，:924-925 注释）**/
  on_dismiss: Option<M>；a2r 裸 popover 臂（027 T-02，rust.rs:
  4529-4649）——shell 九处全显式带 open，x/y 双全 = Point 锚
  （缺省 BottomStart）、否则首子锚件（缺省 Bottom）。
- **native queue 缺口四 kind**：`native_kind_of` 归一
  windowthumbnail/workspacepreview/popover/mousearea
  （coverage.rs:473-478）均不在 kinds（:175-203）→ ensure_covered
  拒绝/auto 降级；投影器全落 `other` 占位盒（native_projector.rs:
  1078-1094）；`scan_native_node` 不递归 Popover 子树（:562——
  MouseArea content 递归在场 :561）。
- **select 覆盖序先例**（可直套）：SelectOverlay + 主块后追加 ops
  + 命中互斥 + Esc（native_projector.rs:90-98/:235-259/:319-325/
  :375-382）。
- **thumbnail/preview**：028 解析侧已备——`resolve_drawlist_image`
  的 `thumbnail://{wid}` 虚拟引用（broker_surface.rs:102-104，
  snapshot SWR 解析 :140-163）；缺投影器臂（`View::WindowThumbnail
  {wid, fallback_icon}` view.rs:874-880 → Image op 桥接）。
  WorkspacePreview{ws}（:890）无既有语义参照（D4 定案）。
- **lucide 机具**：iced 直挂臂走 SVG 光栅化非字体（renderer.rs:
  5471-5547）——`lucide_svg_doc_with`（:6122-6135，
  lucide_generated 1401 全量表）+ svg::Handle 缓存（:6207-6221）+
  tint（:5519-5530）；**resvg/tiny-skia 直用栅格化先例在测试**
  （renderer.rs:29030-29059，resvg 0.45/tiny-skia 0.11 = ui-iced
  真依赖 Cargo.toml:57/:221-222）；`hicon:` 前缀→RGBA Handle 同型
  先例（:5452-5455）；028 宿主词汇表 `lucide:` 显式 not-yet
  （desktop-protocol-v1.md §1.9）。
- **shell pack 用量（View 层缺口面）**：popover ×9（shell.at ×5 +
  desktop.at ×4，其中坐标锚 3 处）、mouse-area 元素 ×13、
  window_thumbnail ×3（shell.at:369/switcher.at:80/88）、
  workspace_preview ×1（shell.at:423）、icon ×7（→`lucide:`
  image_styled，rust.rs:3213）；零 slider/select/checkbox 用量。
- **auto 翻转现状**：Covered 臂仍返 Pixels（client_entry.rs:122-131，
  "flip pending ramp v3 data gate"）；026 数据行 = judged 16/21 =
  76.2% < 95% 阈值维持不翻（reports/p026-native-flip-data-row.md，
  缺项面表把 popover 归 shell S1 范围）。
- **文档同步债**：desktop-shell-a2r.md §10-① 前置序列文本（:271-274）
  仍写"图像 DrawOp 通道立项"未标交付（交付态在 os 台账 3c1 行
  :106）——本计划 SD-02 同步更新。
- **协议版本纪律**：v1.9 = 028 现行；PROTOCOL_VERSION=1 全程。

## 5. 详细设计

### 5.1 T-01 深水调查定案（决策产物）

- **D1 live 接线形态**：iced 0.14 **daemon** 形态下键盘/滚轮/IME
  事件的订阅位（iced daemon 的 subscription 面 vs 窗口事件流——
  实勘 iced 0.14 daemon 事件可达性；widget 级 PUA 处理
  renderer.rs:8722 与全局订阅的边界）；DesktopMessage 扩展变体
  形态（Key(Char)/KeyVk(u32)/Wheel{dx,dy}/ImeCommit…——与既有
  ServiceTick 同型泵入）；映射表（Key::Named→VK u32：BACK=8/
  ESC=27/RETURN=13/方向键…；Key text→CharTyped；WheelScrolled
  Lines×LINE_H/Pixels——025 D4 定案口径复用）。
- **D2 真机证据口径**：候选 A = **SendInput FFI e2e**（推荐——真
  OS 事件进真实 iced 循环，键盘无坐标问题，shm.rs 手写 FFI 先例
  同型、AUTO_DESKTOP_E2E 门控）；B = acceptance channel 扩 key
  verb（进程内半真机——MCP 驱动生产函数）；C = 用户手动实测留痕
  （截图/日志）。兜底 = ⑤协议级承载（p025 先例延续）。定案含
  e2e 焦点前置（SendInput 前断言目标窗聚焦）。
- **D3 popover 臂设计**：placement 覆盖集 = shell 实用子集
  （top/top-end/bottom/bottom-start + Modal/Pointer 如实用）全渲
  vs 全 14 枚举渲染 vs 子集+其余随注（**倾向**：shell 实用集 +
  Modal（通知/对话面）+ 其余 not-yet 随注）；锚几何（Widget 锚 =
  锚件 rect 翻转推导；Point 锚 = 视口坐标直用 + 边距）；scrim
  （Modal = 半透明全屏 Quad 先于面板 ops + scrim 命中 = 只关）；
  命中语义（开态互斥：面板项 > scrim/外点 → on_dismiss 派发 +
  rev++；Esc → on_dismiss）；**开合零投影器状态**（open 随帧）。
- **D4 thumbnail/preview 桥接**：WindowThumbnail{wid} →
  `Image{src:"thumbnail://{wid}"}` + miss fallback（fallback_icon
  lucide 名 → D5 词汇解析；缺省 `app-window`）；WorkspacePreview
  候选 A = `workspace://{ws}` 宿主合成虚拟引用（028 词汇表扩
  scheme——宿主按 ws 窗列表拼网格缩略图，**倾向**：语义对齐
  shell 使用面）/ B = 投影器侧不做、kind not-yet 随注 / C = 占位。
- **D5 lucide 栅格化**：`resolve_drawlist_image` 扩 `lucide:`
  前缀——name 查全量表 → `lucide_svg_doc_with`（stroke width 按
  size 推导）→ tint（style 颜色 or 缺省前景——svg 文档 currentColor
  替换 vs 栅格后像素替换，按 iced 直挂臂 :5519-5530 先例取舍）→
  resvg 栅格化目标尺寸（size 缓存 key：name+color+size）→ Handle
  缓存；未知名 → 降级占位 + 观测。b：svgdoc: 同机具顺带与否
  （shell 零用量——倾向 not-yet 维持）。
- **D6 覆盖扫描与防漏钉**：kinds 四项入册 + `scan_native_node`
  Popover 子树递归（anchor+content）+ 防漏钉矩阵更新（四 kind ×
  投影臂双向）；shell pack 五件 View 树扫描 = Covered 断言载体
  （B 程序覆盖门预演）。
- **D7 翻转复测口径**：026 数据门口径复用（judged ≥95% 阈值），
  样本集不变（examples 全量）+ 增 shell pack 五件行；dual-exit
  （达标即翻 + 台账入册 / 不达标数据留痕）。

定案记录追加 `### 5.1 定案记录`，作为 T-02..T-08 依据。

### 5.2 live 接线与真机证据（T-02/T-03）

- **T-02 接线**：desktop_window_events 扩臂（或 D1 定案的订阅位）→
  D1 映射表 → broker_* 六函数（零改动直用）；DesktopMessage 扩展
  与泵入点；单测 = 映射表全覆盖 + 路由断言（焦点窗/命中窗）。
- **T-03 真机证据**：D2 口径落地——SendInput FFI（如采纳）或
  acceptance key verb 或手动留痕脚本；e2e `p029_live_input_arm`
  （真 OS 键入 → 焦点窗 native app input 值变；IME commit 腿；
  滚轮腿）；⑤协议级兜底腿保留。

### 5.3 shell queue 面臂（T-04/T-05/T-06）

- **T-04 popover 臂**：覆盖序渲染（select 先例）+ D3 全案；golden
  （开/闭态 × 锚型 × placement 子集）+ 命中单测（外点/Esc/项）+
  on_dismiss 派发断言；Popover 子树扫描递归。
- **T-05 桥接臂**：WindowThumbnail/WorkspacePreview（D4）→ Image
  op；golden + snapshot 命中/miss 三路径（028 测试形态复用）。
- **T-06 MouseArea 臂**：透明透传 + 子树渲染 + 事件命中项
  （click/contextmenu 族——HitEntry 扩型）；golden + 命中单测。

### 5.4 lucide 词汇（T-07）

D5 全案：词汇扩展 + 栅格化 + 缓存 + tint；golden（代表图标 ink
非零断言——plan619 测试先例）+ 缓存命中单测 + 未知名降级。

### 5.5 覆盖收口与翻转复测（T-08）

kinds 四项 + 防漏钉双向 + shell pack 五件 View 树 Covered 断言
（e2e/单测双载体）；翻转数据门复测（D7 dual-exit）。

### 5.6 e2e 与收口（T-09）

`p029_*` e2e（live 输入腿 + shell 面渲染腿——合成 client 发
popover/thumbnail/mousearea 视图帧断言）；截图 assets/029/；文档：
desktop-protocol-v1.md **§1.10 v1.10 增量**（live 接线 + 四 kind +
lucide 词汇 + D4 词汇增量）；desktop-shell-a2r.md 前置序列更新
（§10-①：图像件 ✅ → 本件 ✅ → 剩 shell outproc client）；
P025-D1 核销；os 台账 3c2 行；两仓互链。

### 规范增量

| delta_id | add/modify/retire | docs/specs/... target | before/after rule | rationale | acceptance IDs |
|---|---|---|---|---|---|
| SD-01 | add | auto-lang/docs/design/autoui/desktop-protocol-v1.md（§1.10 v1.10 增量） | before：键盘/滚轮/IME 生产路径 = broker_* 函数无事件源（§1.7 live 壳接线缺口在册）；native 覆盖缺 popover/mousearea/windowthumbnail/workspacepreview；`lucide:` 词汇 not-yet（§1.9）；after：live 接线两端入册（iced 事件映射 + DesktopMessage 泵入 + broker 路由）+ 四 kind 渲染/命中语义（popover 覆盖序/open=View 态/on_dismiss；thumbnail 桥接）+ lucide: 词汇真渲入册 + workspace://（如 D4-A）；PROTOCOL_VERSION 仍 1（零 wire tag） | 协议权威版本化收录 B 前置第二件 | AC-01/03/04/05 |
| SD-02 | modify | auto-lang/docs/design/autoui/desktop-shell-a2r.md | before：§10-① 前置序列"图像 DrawOp 通道立项（先行）→ 025 键盘真机实测 → 覆盖二批 → shell outproc client…"（图像件未标交付）；after：序列更新——图像通道 ✅（PLAN-028）+ 本件 ✅（live 输入 + shell queue 面）→ 剩余 = shell outproc client + 启动序/看门兵 | 设计文档前置序列状态收口（清偿同步债） | AC-08 |
| SD-03 | modify | auto-os/docs/plans/autos-desktop-program.md | before：3c1 行 = 图像通道交付；after：增 3c2 行（live 接线 + shell queue 面 + lucide 真渲 + 翻转复测结论） | 桌面程序台账 | AC-08 |
| SD-04 | modify | auto-lang/docs/specs/auto-lang/ui/（review 期按目录实况定） | before：028 provisional 无 live 接线/shell 面条目；after：session live 接线 + native popover/thumbnail/mousearea 臂 + lucide 词汇条目（provisional） | 模块 spec 对齐实现 | AC-01/03/04/05 |

零 spec 影响的变更不存在（live 链路/覆盖集/词汇为协议级知识）；
ledger（auto-lang `.autoos/specs.json`）随 merge 沉淀。

## 6. 测试设计

- **单测（session）**：映射表全覆盖（Named 键→VK/text→CharTyped/
  Wheel 两型/IME 三态）；DesktopMessage 泵入 → broker_* 路由断言
  （焦点窗/命中窗）；零改动的 broker_* 既有测试回归。
- **单测（native_projector）**：popover golden（开/闭 × Point/Widget
  锚 × placement 子集 + Modal scrim）+ 命中（外点/Esc/面板项 →
  on_dismiss 派发 + rev）+ Popover 子树扫描递归；thumbnail/
  preview 桥接 golden + 命中/miss/fallback 三路径；MouseArea 透传
  + click/contextmenu 命中派发。
- **单测（broker_surface）**：lucide: 词汇（代表图标栅格化 ink 非零
  ——plan619 先例 + 缓存命中零重栅格化 + tint 颜色断言 + 未知名
  降级占位 + 观测去重）。
- **覆盖单测**：四 kind 入册判定 + 防漏钉双向更新 + **shell pack
  五件 View 树 Covered**（装载期扫描断言——B 程序覆盖门预演）。
- **e2e（AUTO_DESKTOP_E2E）**：`p029_live_input_arm`（D2 口径——
  真 OS 键入/IME/滚轮 → 焦点窗 native app 状态变 + ⑤兜底腿）；
  `p029_shell_face_arm`（合成 client 发 shell 面视图 → 帧断言
  popover/thumbnail/mousearea/icon 渲染 + 命中闭环）；截图
  assets/029/。
- **翻转复测**：examples 全量 + shell pack 五件数据行（D7 dual-exit）。
- **回归门**：desktop_protocol/session/stage3/dual_mode + ts_fixtures
  + auto-os 桌面 smoke。

## 7. 验收标准

- **AC-01 live 接线全链**：真 iced 键盘/滚轮/IME 事件 → 映射 →
  broker_* → 焦点窗/命中窗 child——单测（映射表+路由）绿 +
  P025-D1 核销落笔。验证：单测 + KNOWN-DEBT 核销行。
- **AC-02 真机证据**：D2 定案口径的端到端证据落盘（SendInput 真机
  腿或 acceptance verb 或手动留痕；不可达时 ⑤协议级承载 + 留痕
  ——dual 口径同 025 先例）。验证：e2e 输出/留痕文件。
- **AC-03 popover 全链**：开态覆盖序渲染（placement 子集 + Modal
  scrim）+ Point/Widget 双锚 + 外点/Esc → on_dismiss 派发 + 命中
  互斥；闭态零 ops（open 随帧）。验证：golden + 命中单测 + e2e。
- **AC-04 thumbnail/preview 桥接**：WindowThumbnail → 缩略图 Image
  op（snapshot 命中真渲/miss fallback）；WorkspacePreview 按 D4
  定案形态（或显式 not-yet 随注）。验证：三路径单测 + e2e。
- **AC-05 lucide 真渲**：`lucide:` src → 宿主栅格化真图标（ink 非
  零 + tint 正确 + 缓存命中）；未知名降级占位 + 观测。验证：单测
  + e2e 帧断言。
- **AC-06 覆盖与防漏钉**：四 kind 入册 + Popover 子树扫描递归 +
  防漏钉双向绿 + **shell pack 五件 View 树 Covered**（B 程序覆盖
  门预演断言）。验证：覆盖单测 + 防漏钉。
- **AC-07 翻转复测 dual-exit**：数据行（examples + shell pack 行）
  落盘——达标即翻（缺省 queue + 台账入册）或显式不翻留痕；禁无
  数据翻转。验证：数据报告 + client_entry 缺省断言。
- **AC-08 文档与回归门**：§1.10 + 前置序列更新 + P025-D1 核销 +
  台账 3c2 行落盘互链；§6 回归门全绿（在册既有红除外）。

## 8. 执行步骤

依赖序：T-01 → {T-02, T-04 并行} → {T-03, T-05, T-06, T-07 并行}
→ T-08 → T-09。lang worktree `D:/autostack/.wt/lang-029/auto-lang`；
os `D:/autostack/.wt/os-029/auto-os`。**无前置计划依赖**（028 已
merge 即基线）。

- **T-01 [lang] 深水调查与定案**
  文件：`ui/session.rs`（desktop_window_events/订阅面）、iced 0.14
  daemon 事件 API 面（registry 源）、`native_projector.rs`、
  `broker_surface.rs`、`ui/iced/renderer.rs`（lucide/snapshot 机具，
  读）、shell pack 五件（auto-os，读）+ 本计划 §5.1（写面）。
  动作：D1–D7 定案。
  产物：`### 5.1 定案记录`（file:line 证据）。
  验证：定案完备；复审通过。
  → 全 AC 前置。新路径：是。
- **T-02 [lang] live 接线**
  文件：`ui/session.rs`（订阅扩臂 + 映射 + DesktopMessage 扩展）。
  动作：§5.2 T-02；broker_* 零改动直用。
  验证：映射/路由单测绿 + broker_* 既有测试回归。
  → AC-01。
- **T-03 [lang] 真机证据 e2e**
  文件：`stage3.rs`（p029_live_input_arm）+ D2 通道（SendInput FFI
  模块如采纳 / acceptance verb / 脚本）。
  动作：§5.2 T-03；⑤兜底腿。
  验证：e2e 留痕（AUTO_DESKTOP_E2E 门）。
  → AC-02。
- **T-04 [lang] popover 臂**
  文件：`native_projector.rs`（覆盖序渲染 + 锚/placement/scrim +
  命中）、`coverage.rs`（kind + 子树递归）。
  动作：§5.3 T-04；D3 全案。
  验证：golden + 命中单测 + 扫描递归测试。
  → AC-03/06。
- **T-05 [lang] thumbnail/preview 桥接臂**
  文件：`native_projector.rs`（两变体 → Image op）；如 D4-A 采
  workspace:// 另扩 `broker_surface.rs` 词汇。
  动作：§5.3 T-05。
  验证：三路径单测 + golden。
  → AC-04/06。
- **T-06 [lang] MouseArea 臂**
  文件：`native_projector.rs`（透传 + 命中项）。
  动作：§5.3 T-06。
  验证：golden + 命中单测。
  → AC-06。
- **T-07 [lang] lucide 词汇真渲**
  文件：`broker_surface.rs`（词汇扩展）、`renderer.rs`（机具接驳
  ——栅格化/tint 函数化复用）。
  动作：§5.4；D5 全案。
  验证：ink/tint/缓存/降级单测绿。
  → AC-05。
- **T-08 [lang] 覆盖收口 + 翻转复测**
  文件：`coverage.rs`（防漏钉）、`client_entry.rs`（门后翻转点）、
  数据报告（lang reports/）。
  动作：§5.5；D7 dual-exit。
  验证：防漏钉 + shell pack Covered 断言 + 数据行落盘。
  → AC-06/07。
- **T-09 [lang+os] e2e 与收口**
  文件：lang `stage3.rs`（p029_shell_face_arm）+ 截图 assets/029/ +
  `desktop-protocol-v1.md`（§1.10）+ `desktop-shell-a2r.md`（前置
  序列）+ KNOWN-DEBT（P025-D1 核销）；os 台账 3c2 行 + 互链。
  动作：AC 逐条留痕；SD-01..04 落笔。
  → AC-08。

## 9. 复审记录

- 2026-09-18 /auto-plan:new 起草交接：`stage: new`，PLAN-029 rev 1。
  `outcome: pass`（合同完整：live 接线缺口（P025-D1 原文）、broker_*
  六函数与路由语义、View::Popover open 态归属、四 kind 覆盖缺口、
  select 覆盖序先例、lucide 栅格化机具与依赖、028 词汇/解析基建、
  翻转数据门现状全部 file:line 在案）；`next: work`（**无前置计划
  依赖**，T-01 可即行）。悬置决策登记 §10（①–⑤），均不阻塞 T-01
  开工。

## 10. 待澄清事项

- **①（T-01 D2）** 真机证据口径：SendInput FFI e2e（推荐——真 OS
  事件、键盘无坐标问题、FFI 先例同型）vs acceptance channel 扩
  key verb vs 手动留痕；⑤协议级兜底延续。
- **②（T-01 D4）** WorkspacePreview 形态：`workspace://{ws}` 宿主
  合成虚拟引用（推荐——语义对齐使用面）vs kind not-yet 随注 vs
  占位。
- **③（T-01 D3）** placement 覆盖集：shell 实用子集 + Modal（推荐）
  vs 全 14 枚举——其余 not-yet 随注。
- **④（T-01 D5-b）** svgdoc: 通用词汇顺带（shell 零用量——倾向
  not-yet 维持）。
- **⑤（T-08）** 翻转 dual-exit 口径同 026（数据门 ≥95% judged）；
  shell pack 行是否计入分母（倾向单列——shell 非 examples 样本）。
