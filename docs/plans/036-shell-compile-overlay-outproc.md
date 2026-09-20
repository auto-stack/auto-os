---
plan_id: PLAN-036
status: executing             # drafting → executing → execution_done → reviewed → archived
feature_name: shell-compile-overlay-outproc
author: [agent]
created_at: 2026-09-20
updated_at: 2026-09-20
plan_revision: 1

# /auto-plan:review 结束时填写：
supersedes_spec_components: []
new_spec_components:
  - auto-lang/docs/design/autoui/desktop-protocol-v1.md   # §1.16 v1.16 增量（review 定稿）
touched_goals: []

affects:
  - auto-lang/crates/auto-man/src/rust_ui.rs                            # 无窗组件库生成目标（shell-lib crate + mount_face 工厂）
  - auto-lang/crates/auto-lang/src/ui/desktop_protocol/shell_client.rs  # 编译组件替换点 + overlay 面装配 + 聚焦 child 化
  - auto-lang/crates/auto-lang/src/ui/desktop_protocol/shell_projection.rs # 四张 overlay 快照载体激活（推送+消费）
  - auto-lang/crates/auto-lang/src/ui/desktop_protocol/message.rs       # surface_role 扩档（追加式，依 D2）
  - auto-lang/crates/auto-lang/src/ui/session.rs                        # overlay 懒挂载槽 outproc 化 + launcher 装载 + 焦点语义
  - auto-lang/crates/auto-lang/src/ui/iced/renderer.rs                  # Stack 层槽 per-surface 化 + 推送泵 + face 栅格化（依 D1）
  - auto-lang/crates/auto-lang/src/ui/desktop_protocol/broker_surface.rs # face:// 虚拟引用（依 D1-C+）
  - auto-os/shell/                                                       # pack 源（035 修缮后基线；pin 快照 sync）
  - auto-os/docs/plans/autos-desktop-program.md                         # M7-b 行
current_step: 4
total_steps: 10
---

# [PLAN-036] shell-compile-overlay-outproc

## 0. 变更摘要

**M7-b 波次**（B 形态彻底化收官，承接 P030-D1/P030-D3 两债——032 翻转
解锁的"随缺省翻转计划另立"件）。三批交付：

**A. shell-lib 组件库生成模式（P030-D1）**：wrap_example 增"无窗组件
库"目标——shell pack 五件生成 lib crate（`SHELL_MANIFEST` 同形 const +
`mount_face` 工厂，027 D5 设计被裁件复活），child 侧 `ShellFaces::load`
的 `RqProjector<DynamicComponent>` 替换为编译组件（同 trait 面，接缝
注记在案）；收益 = 省 boot 期 .at 解释装载 + 摆脱 auto.exe re-exec
依赖；双轨保留解释 fallback（R3 裁定）。
**B. overlay 面 outproc 化（P030-D3，渐进三批）**：协议机器普查证实
**几乎全备**——一 exe 多表面协商/分面帧/指纹门/按 face 寻址的
ShellProjectionPush（tag 12）全链在役；027 交付的
Switcher/Notes/Launcher/Dashboard 四张快照载体"编码在册零推送零消费"
（shell_projection.rs wire 模块，`wire_round_trip_full_family` 注释明示
D6 边界）。**B1 switcher + 通知中心**（载体全齐，纯缺推送泵激活 +
child 装配 + Esc/键盘路由 + 缩略图 `thumbnail://` 下行引用）→ **B2
dashboard**（两硬点：z 新档——surface_role 现只 background/window/
chrome 三档，dashboard 需"background 上/window 下"中间档；face 卡
材料——宿主活渲染的 mini 视图无投影载体，四候选含"`face://{appid}`
宿主栅格化 + 虚拟引用下行"零 wire 方案）→ **B3 launcher**（注册表
装载 → outproc + 聚焦链 child 化[029 焦点窗路由够字符面，缺伪窗可
focused 的 WM 语义 + `__focus_input` child 化 + 热键宿主保留边界]）。
**C. 终态验收**：**五面全 outproc 桌面**（shell/desktop 常驻 + 三
overlay + launcher 全部经 RenderQueue）+ watchdog/parity + 度量。

与 035（desktop-ux-rev3，executing 中，动 dashboard.at/renderer.rs/
shell.at 内容面）的协调：**本计划以 035 merge 后基线起草执行**（形态
vs 内容边界；pin 快照 hash-lock 先后约定）。

## 1. 目标

- **G1 shell-lib 生成模式（P030-D1 核销）**：wrap_example"无窗组件库"
  目标（五件 → 一个 lib crate + `SHELL_MANIFEST` const + `mount_face`
  工厂）；词汇门升级真编译门（`test_shell_pack_codegen_vocabulary_gate`
  自注"真实编译门 = shell-lib crate cargo build"）；child 替换点落地
  （编译组件 ↔ `RqProjector<DynamicComponent>` 同 trait 面互换）；双轨
  常驻（解释 fallback 开发态）；度量行（boot 时延差 / 解释器依赖消除）。
- **G2 B1 switcher + 通知中心 outproc**：两张快照载体激活（宿主推送
  泵 + child 分面装配 + 召唤动词联动）；键盘路由（Esc 关闭/上下键/
  Enter——029 焦点窗路由 + RqProjector on_input 既有面）；switcher 行
  缩略图经 `thumbnail://` DrawOp 引用（028 通道，零 wire）；伪窗 z =
  置顶带矩形（chrome 先例）。
- **G3 B2 dashboard outproc**：z 新档（依 D2：surface_role 追加
  DASHBOARD 档 or 宿主槽位矩形化——追加式）；face 卡材料方案落地
  （依 D1 四候选定案）；`__dashboard_cmd` 上行 + face 点击→focus 闭环。
- **G4 B3 launcher outproc**：装载形态（依 D3：注册表 exe 一面一 exe
  vs 并入壳 exe 多表面）；聚焦链 child 化（`__focus_input` 语义 /
  summon 时 input 预登记等价物 / 伪窗可 focused 的 WM 语义）；热键
  （Ctrl+Space/⊞）宿主保留边界成文。
- **G5 终态验收与收口**：五面全 outproc e2e（桌面 chrome/壁纸/任务栏/
  switcher/通知/dashboard/launcher 全经 RenderQueue——进程边界断言）；
  watchdog 覆盖 overlay 面（面崩溃不连坐判定按 D3 形态）；parity 对拍
  （outproc 各面 × in-proc 形态金样）；§1.16 增量 + P030-D1/D3 核销 +
  台账 M7-b 行。

**非目标**（明确出界）：

- **035 的内容修缮面**（dashboard 8×3 网格/任务栏 bug 修/mini 细节
  ——形态工作以其落地为基线，不重复不回改）。
- M7-c（app 批量化）/ M7-d（收口）——并行波次；**pixels 退役执行件**
  （条件②覆盖收口随 M7-c，删除另立）。
- 位图过线新方向（034 child→host 上行已备；face:// 若采 D1-C+ 为
  **宿主解析虚拟引用**，零 wire——不扩通道本体）。
- L3 StateSnapshot native 注入；518 色彩；Stage B 搬迁；rqhost 生态。
- 解释 fallback 退役（双轨常驻裁定维持——R3）。

## 2. 架构方案

```text
┌─ A 编译面（rust_ui.rs + shell_client.rs 替换点）────────────────────┐
│ wrap_example 无窗组件库目标：五件 → lib crate（shell-pack）         │
│   + SHELL_MANIFEST const + mount_face(id)->组件 工厂                │
│ child：ShellFaces::load = RqProjector<编译组件>（同 trait 面：      │
│   component_mut/write_state/on_input/render_frame/revision）        │
│ 双轨：缺省编译 exe？/解释 re-exec fallback（shell_model 配置面扩展）│
└──────────────────────────────────────────────────────────────────┘
┌─ B overlay（一 exe 多表面机器复用 + 三段渐进）──────────────────────┐
│ 机器面（在役/在册）：SurfaceDecl 协商 + activate_multi 路由 +       │
│   分面帧 sync_frames + ShellProjectionPush 按 face 寻址（tag12）    │
│   + shell_face 常量 SHELL=1..DASHBOARD=5（message.rs:328-345）      │
│ B1 switcher/通知：SwitcherSnapshot/NotesSnapshot 推送泵激活         │
│   （renderer summon/inject 点 → child）+ child 两面装配 +           │
│   Esc/键盘路由 + thumbnail:// 行缩略图                              │
│ B2 dashboard：surface_role 扩 DASHBOARD 档（z：bg 上/window 下）    │
│   + 宿主 Stack 槽 per-surface 化 + face 材料（D1 定案——四候选）     │
│ B3 launcher：装载 outproc（D3 形态）+ 聚焦链 child 化 +             │
│   热键宿主保留边界                                                   │
└──────────────────────────────────────────────────────────────────┘
┌─ C 验收（e2e/parity/watchdog/度量）──────────────────────────────────┐
│ 五面全 outproc 桌面 e2e + 各面 parity 金样 + watchdog 面崩溃粒度    │
│   判定 + boot 时延/内存对照行                                        │
└──────────────────────────────────────────────────────────────────┘
```

**不变式**：

- **I1 追加式协议**：surface_role 扩档/新词汇（face:// 若采）全部
  追加式；`PROTOCOL_VERSION` 仍 1；既有 golden 零漂移。
- **I2 零回归**：in-proc 形态（shell_model=inproc 缺省）全行为不变——
  desktop_mcp 五套 inproc 档回归门；035 落地的 UX 基线不回改。
- **I3 降级纪律**：面崩溃 watchdog 粒度按 D3 形态（连坐 or 独立），
  耗尽回退 in-proc 语义沿 030 链；未覆盖面/未定案项显式随注。
- **I4 双轨纪律**：解释 fallback（AUTO_SHELL_PACK）常驻；编译/解释
  两形态 parity 对拍进验收（非只验 outproc）。

**关键风险**：dashboard z 档与 WM 命中的耦合（全屏面板伪窗命中收窄
——chrome 带矩形先例需推广）；face 材料四候选的实时性/复杂度权衡
（D1 核心）；launcher 聚焦链 child 化的 iced 语义（focus Task/
`__focus_input` 重试机制是宿主内发明，child 化需等价重设计）；与
035 的 renderer.rs 冲突（前置时序钉死）；shell-lib 生成物的构建
集成（宿主构建链是否引 shell-pack crate——双轨缺省方向依 D4）。

## 3. 技术栈

Rust / iced 0.14；v1.11 多表面机器（SurfaceDecl/activate_multi/
ShellProjectionPush/shell_face 常量）；027 typed 快照载体四件
（shell_projection.rs wire 模块）；028 `thumbnail://` 虚拟引用；
034 bitmap://（对偶参考）；a2r 生成链（wrap_example + 词汇门）；
030 watchdog/双伪窗/降级链；验收 = p030 e2e 扩展 + desktop_mcp 双
形态 + assets/036/。

## 4. 需求分析与背景调查

**授权记录**：用户 2026-09-20 会话明确"计划 034 已经完成，正在
merge。下一步是什么计划？需要 auto-plan-new 来写新计划"——按台账
M7 序（M7-a ✅032 → {M7-b, M7-c①} 并行带），下一件取 **M7-b**
（shell a2r 编译面 + overlay 四面 outproc 化——P030-D1/D3 显式移交
债；"随缺省翻转计划另立"的解锁条件[032 翻转]已满足）。**本轮仅
规划，未授权实施**。涉及仓：auto-lang（生成/协议/装配/文档）+
auto-os（pack 基线/台账）。无预算/自动续跑约束声明。

**前置依赖**：①034 merge ✅（master 含 c4facc584——bitmap 通道已
在）；②**035-desktop-ux-rev3 merge**（executing 中，动
dashboard.at[8×3 网格 layout v2]/renderer.rs[dashboard_layout]/
shell.at——B2 批次与其基线强耦合，A/B1 批次文件面错开但 pin 快照
hash-lock 先后需约定；**建议整体以 035 merge 后开工**）。

**现状事实**（已核，2026-09-20 master 含 033/034，探索代理全量
普查；路径缩写 SC=shell_client.rs、SP=shell_projection.rs、
R=renderer.rs、S=session.rs）：

- **030 v1 现役机器（033 后）**：child 双表面协商（Hello 尾段
  background+chrome 两枚 SurfaceDecl SC:320-340；Welcome extra_
  surfaces 建 routes SC:375-387）；RqProjector 双面（SC:70-79/84-101，
  **overlay 三面 v1 显式忽略留痕 SC:115-116**）；ShellProjectionPush
  消费（apply_projection SC:123-164，ControlMsg tag 12
  message.rs:1093-1100/1237）；__desktop_cmd 读走+DesktopBus 上行
  （SC:201-222/:411-448）；分面增量帧+双槽轮转（sync_frames
  SC:457-495，免 shm 内联帧）；spawn = re-exec auto + env
  （S:3049-3066，AUTO_SHELL_GEOM/PACK）；宿主 = 模型分叉
  （env > storage，缺省 Inproc，R:14845-14866）+ 壳臂 attach 双伪窗
  （background 垫底 wm_add_win_bottom/chrome 置顶带矩形 S:3946-3955
  + activate_multi endpoint.rs:774-803）+ 投影推送泵（SHELL 17 键/
  DESKTOP_SURFACE 9 键指纹门 R:14074-14144）+ watchdog（EOF→退避
  1s/2s/5s/预算 3 次→shell_degraded 一次性回退 in-proc
  S:3480-3542/R:18266-18290）。
- **overlay 四面 in-proc 装配**：懒挂载槽 S:284-311（launcher/
  switcher/notification/dashboard 四槽 + hatched_minis）；召唤动词 =
  summon_launcher R:10036-10138（Ctrl+Space/⊞ 热键）/summon_switcher
  R:10177-10254（Ctrl+Tab，mru 四平行列表+thumbs+RebuildMru）/
  toggle_notification_center R:10264-10327（note_* 五列表+RebuildNotes）/
  toggle_dashboard R:10778+（boot 常驻 R:15147-15151，face_* 注入+
  RebuildFaces R:10680+）；launcher 装载 = boot 注册表扫描
  launcher_entry（R:15044-15051/S:323-326）+ 召唤时 read_to_string+
  build_dynamic_component 懒挂载（R:10044-10063）+ apps 七平行列表
  注入（R:10071-10115）。
- **z 序现状**（desktop_root Stack 底→顶，R:19337-19747）：壁纸 →
  background 表面 → **dashboard 常驻层（R:19369-19530，z 高于图标
  低于 App 窗——R3 语义）** → 图标/虚拟窗 z_order → 槽位 chrome →
  chrome 表面（任务栏）→ launcher overlay → switcher → 通知中心。
  **surface_role 只三档**（WINDOW=0/BACKGROUND=1/CHROME=2，
  message.rs:308-324）——无"bg 上/window 下"中间档。
- **投影材料缺口**（载体 vs 消费对照，普查表）：switcher/通知 =
  **载体全齐零推送**（SwitcherSnapshot SP:152-163/NotesSnapshot
  SP:168-181 wire 编解码在册，`wire_round_trip_full_family` 注释
  "overlay 三面 v1 不下行（D6 边界）但编码在册"——027 交付四件、
  030 激活 2/6）；dashboard face 卡 = **宿主活渲染**（split_ref_face
  mini 视图拆借渲染 R:19479-19500 + DM::App 直达 R:10330-10334；
  mini 会话 = 宿主内 windowless hatched R:10687+；DashboardSnapshot
  只有 face_* 清单+几何 SP:216-225——**无 mini 视图内容载体**）；
  launcher = LauncherSnapshot 齐（SP:186-199）+ 聚焦链
  （iced focus Task R:10116-10137 + `__focus_input` 重试
  R:10106-10109 + 宿主窗订阅独占 R:19943-19960——**child 化最深**）。
- **键盘路由可用性**：029 live 输入焦点窗路由齐（route_live_input
  S:3864-3888 → broker_key/char/ime 按 wm.focused；RqProjector
  on_input 已收 CharTyped/Ime/Key8/27/Scroll native_projector.rs:
  563-625）——B1/B3 字符面够用；缺 = 伪窗可 focused 的 WM 语义 +
  聚焦 child 化 + 热键宿主保留。
- **shell-lib 生成需求面**：wrap_example 现状 = app 工程形（单 main
  启发式+独立窗入口+client gate 含 Rqhost 档 rust_ui.rs:1752-1931）；
  设计缺口原文（desktop-shell-a2r.md:104 d1 + :220-229 S3——T-07
  被裁件）；`SHELL_MANIFEST` const 在（SP:1044-1072，crate
  "shell-pack" 五面清单 + mount_face 注释锚点零实现）；**词汇门在**
  （test_shell_pack_codegen_vocabulary_gate rust.rs:8901-8969+，自注
  "真实编译门 = shell-lib crate cargo build"——030 复审 R-1 同引）；
  child 替换点注记在案（SC:10-14）；收益 = 省每次 boot .at 解释装载
  + 摆脱 auto.exe re-exec（smoke 需双构建 smoke-030:34-38）。
- **034 对偶参考**：bitmap:// = child→host 上行（app 产位图宿主合
  成）；face 材料若采"宿主栅格化+虚拟引用下行"= **零 wire**（028
  thumbnail:// 同型——src 词汇表加 face:// 前缀臂）。
- **035 协调面**：035 affects = shell.at（①③）/dashboard.at（④
  8×3 网格）/renderer.rs（T-02/T-04/T-07）/examples 012/013/020/
  pin 快照——状态 executing 0/9。冲突文件 = renderer.rs +
  dashboard.at + pin hash-lock；**先后 = 035 先行落地**。
- **e2e 基线**：p030_shell_outproc_arm 四腿在（stage3.rs:1015-1194，
  AUTO_DESKTOP_E2E 门 + AUTO_030_ASSETS）；smoke-030（缺省 lang 根
  = .wt/lang-030 已清理，路径需注意）；shell_pack_native_covered
  仪器在（coverage.rs:1534+）。

**specs 现状**：协议 v1.15 现行（034 §1.15——merge 在途）；P030-D1
（KNOWN-DEBT :2383）/P030-D3（:2385）在册；台账 M7-b 行（:103）；
Design 23 §4 矩阵 + desktop-shell-a2r.md S3 改道注。

## 5. 详细设计

### 5.1 T-01 深水调查定案（决策产物）

- **D1 face 卡材料（本计划核心决策）**：候选 A = mini 会话随面搬
  child（会话孵化链大改——hatched_minis child 化）；B = 宿主保留
  face 合成层、仅 chrome outproc（半 outproc——z 问题只解一半）；
  C = face 位图化过线（034 通道反向/实时性受限）；**C+ = 宿主栅格
  化 mini 视图 + `face://{appid}` 虚拟引用下行**（child 的 DrawList
  引用、宿主解析——028 thumbnail:// 同型零 wire，SWR 语义随 face
  revision 翻新；**倾向**）。以 035 落地后的 dashboard.at face 消费
  面定案。
- **D2 dashboard z 档形态**：候选 A = surface_role 追加 DASHBOARD=3
  （"background 上/window 下"新档 + 宿主 Stack 插层 + WM 命中窗口
  带化推广[chrome 带矩形先例]——追加式）；B = 宿主槽位矩形化不动
  协议（但 outproc 表面的插入层语义无协议锚——复用 chrome 档 + 宿
  主插层 hack）。**倾向 A**（协议语义显式）。
- **D3 overlay exe 拓扑**：候选 A = **壳 exe 多表面扩展**（五面一
  exe——机器全复用，看门兵连坐粒度 = 整壳）；B = overlay 独立 exe
  （隔离粒度好，四份装配成本）；C = 混合（switcher/通知/dashboard
  并壳 + launcher 一面一 exe[本走注册表]）。**倾向 C**——以 D1/D2
  复杂度与崩溃粒度权衡定案。
- **D4 编译/解释双轨缺省**：缺省编译 exe（发布态）vs 维持解释缺省
  （保守）——考虑 shell-lib crate 的构建集成（宿主构建链是否引
  用/独立 exe 发现序）与 030 smoke 双构建负担消除收益。
- **D5 launcher 聚焦 child 化等价物**：`__focus_input` 重试机制
  （R:10106-10109）的 child 等价（投影下发 focus 请求位 vs child
  自主聚焦首 input）；伪窗可 focused 的 WM 语义（chrome 先例推广）；
  热键宿主保留边界（Ctrl+Space/⊞ 在宿主层截获→投影/命令下发）。
- **D6 B1 键盘语义清单**：switcher（Esc/Tab/Enter/方向键）与通知
  （Esc/清除）的 InputMsg 消费面 vs 宿主热键边界。

### 5.1 定案记录

**T-01 调查基线**：lang-036 worktree @ master 608399943（含 034 merge
c4facc584）；§4 事实锚逐点复核在案——SC=shell_client.rs（578 行，
`ShellFaces` 双面 `RqProjector<DynamicComponent>` SC:70-79、解释装载
`ShellFaces::load` SC:84-102、overlay 忽略留痕 SC:115-116、替换点注记
SC:10-14）；SP=**`src/ui/shell_projection.rs`**（§4 路径缩写实指，非
desktop_protocol/ 下）——`SHELL_MANIFEST` 五件 SP:1044-1072（crate
"shell-pack"，027 D5 注释锚点零实现）；词汇门 ui_gen/rust.rs:8901+
（认知表比对，:8899 自注"真实编译门 = T-07 shell-lib crate cargo
build"）；RqProjector 泛型 `C: Component`（native_projector.rs:301，
a2r typed 组件原生可用）；`Component` trait 已有 defaulted
`drain_desktop_commands`（033 泛化，component.rs:96-100）与
`apply_state_snapshot`（L3 注入位，:125-133——a2r 缺省 false）；a2r
状态字段类型映射（ui_gen/rust.rs:466-482）：`var x = []` →
`Vec<serde_json::Value>`、`str` → `String`——与 ShellWrite 载荷
（auto_val::Value）经 `Value::deserialize::<T>()` 互转（auto-val
de.rs:98）；入口归属 `crates/auto/src/cmd_autodesk.rs:70`
（run_shell_outproc）；spawn 注入面 session.rs:3043-3067（**现状无条件
注入 AUTO_SHELL_PACK**——兄弟发现也注入，D4 需改）；shell_model 分叉
renderer.rs:14840-14866（env AUTO_SHELL_MODEL > storage
shell.apps.shell_model，缺省 Inproc）；027 归档 §5.1 D5 原始设计
（archive/027-desktop-shell-a2r.md:315-332：lib crate 入库
crates/shell-pack + mount_face 工厂 + 双轨开关语义）。

- **D1 face 卡材料（用户确认项，悬置 → T-06 前置）**：维持倾向 **C+**
  （宿主栅格化 mini + `face://{appid}` 虚拟引用下行——028 thumbnail://
  同型零 wire + SWR 翻新）；以 035 落地后 dashboard.at face 消费面
  最终定案。不阻塞批次 A/B1。
- **D2 dashboard z 档（悬置 → T-05 前置）**：维持倾向 **A**
  （surface_role 追加 DASHBOARD=3 档——追加式协议显式 + 宿主 Stack
  插层 + WM 命中带化推广）。
- **D3 overlay exe 拓扑（用户确认项，悬置 → T-07 前置）**：维持倾向
  **C 混合**（switcher/通知/dashboard 并壳 exe 多表面 + launcher 一面
  一 exe 走注册表）；以 D1/D2 落地复杂度与崩溃粒度权衡终裁。
- **D4 编译/解释双轨缺省（✅ 定案 2026-09-20）**：**outproc child 缺省
  编译**（auto.exe 链入 shell-pack——省 boot 期 .at 解释装载）；显式
  `AUTO_SHELL_PACK` env（存在且为目录）或宿主 override 注入 = 开发态
  解释 child（027 D5 双轨开关语义原文承袭：兄弟检出发现 pack 但无
  env/override ≠ 解释态）。配套改造：spawn_shell_outproc 注入面收窄
  （仅显式 override/env 命中时注入 AUTO_SHELL_PACK——现状无条件注入
  会使编译轨在开发检出永不生效）；in-proc 宿主（shell_model=inproc
  缺省）解释装载零变化（I2）。构建集成 = shell-pack 入库 auto-lang
  workspace 成员（每次 cargo build 即真编译门；宿主/auto.exe 构建免
  生成时序依赖——027 D5 入库裁定原文）。发布态专用 shell exe（彻底
  摆脱 re-exec）另立注记，不在本计划强制面。
- **D7 shell-lib 生成目标形态（✅ 定案，批次 A 承接 D4）**：产物 =
  `crates/shell-pack/`（lib crate，入库 workspace 成员，path 依赖
  auto-lang[ui-iced]——DAG: shell-pack → auto-lang ← auto-man/auto，
  无环）。内容 = 五组件（a2r 编译产物）+ `SHELL_MANIFEST` const
  （shell_projection.rs 同形 ShellManifest）+ `mount_face(id, w, h)
  -> Option<Box<dyn ShellSurface>>` 工厂 + 每组件 ShellStateAccess
  生成实现（string-key 写态 lowering[auto_val→serde_json 字段赋值]/
  召唤事件 dispatch[event 名 → Msg variant on()]/命令读走）。生成入口
  auto-man rust_ui.rs `generate_shell_pack_lib`（pack 解析与词汇门
  同序 resolve_os_top_dir）；真编译门 = workspace 成员编译（每次
  build）+ **freshness 字节对拍测试**（重生成 vs 入库物，solo 检出
  跳过——词汇门保留为快速诊断，非升级替代而是三件套）。
- **D8 child 替换点形态（✅ 定案，批次 A 承接）**：`ShellFaces` 字段
  换 `Box<dyn ShellSurface>`（装配 trait 定于 shell_client.rs 同册：
  apply_writes/dispatch_event/write_scalar/read_state_str/clear_state/
  on_input/render_frame/revision/hit_rects——SC:70-247 消费面的 trait
  化）。解释臂 = 现行 `RqProjector<DynamicComponent>` 适配器
  （write_state/call_handler/read_state 桥——行为逐字节不变）；编译臂
  = `RqProjector<编译组件>` 适配器（经 D7 ShellStateAccess 生成实现）。
  auto-lang 不引 shell-pack（无环）——装配点 = crates/auto
  cmd_autodesk（`run_shell_outproc_with(broker, faces_factory)` 工厂
  注入；`--autodesk-shell` 分派按 D4 开关选臂）。字节级 parity 对拍
  入 T-08 验收（B 批次后五面金样）；T-03 冒烟 = 结构性（装载/投影
  apply/命令 drain/渲染非空/revision 前进）+ **boot 时延度量行**
  （interpreted 解释装载 vs compiled mount_face 对拍，落 shell-pack
  测试输出，T-09 汇总）。
- **D5 launcher 聚焦 child 化（悬置 → T-07 前置）**：候选面复核在案
  （iced focus Task R:10116-10137 / `__focus_input` 重试 R:10106-10109
  / 宿主窗订阅独占 R:19943-19960）；B3 批次细化（投影下发 focus 请求
  位 vs child 自主聚焦首 input + 伪窗可 focused WM 语义 + 热键宿主
  保留边界三件套）。
- **D6 B1 键盘语义清单（悬置 → T-04 前置）**：switcher
  （Esc/Tab/Enter/方向键）与通知（Esc/清除）InputMsg 消费面（029
  route_live_input S:3864-3888 + native_projector on_input
  native_projector.rs:563-625 在役证据复核）；B1 批次落清单。

**批次时序裁定（本会话开工面）**：035 merge 前置对 B2 硬阻塞维持；
批次 A（T-02/T-03，rust_ui.rs/shell_client.rs/session.rs spawn 面
与 035 affects[renderer.rs/dashboard.at/examples/pin] 文件错开）先行
——唯一交叠 = pin 快照 hash-lock（A 批次不动 assets/ 五件内容，
freshness 门以 auto-os/shell 权威源对拍，035 merge 后随其 T-08 pin
sync 重跑一次 freshness 确认零漂移）。


### 5.2 批次 A：shell-lib 生成（T-02/T-03）

- **T-02 生成目标**：wrap_example 无窗组件库形态（lib crate
  shell-pack + `SHELL_MANIFEST` const + `mount_face` 工厂）；词汇门
  升级真编译门（crate cargo build 断言）；生成物构建集成（依 D4）。
- **T-03 child 替换 + 双轨**：ShellFaces::load 编译组件替换点（同
  trait 面互换 + parity 冒烟）；shell_model 配置面扩展（compiled |
  interpreted | outproc 组合语义澄清）；度量行（boot 时延/解释装
  载消除）。

### 5.3 批次 B1：switcher + 通知（T-04）

推送泵激活（summon/toggle 注入点 → SwitcherSnapshot/NotesSnapshot
下行——030 双面泵推广）+ child 两面装配（SurfaceDecl + 分面帧 +
Esc/键盘路由依 D6）+ thumbnail:// 行缩略图 + 置顶带矩形伪窗 +
parity 对拍。

### 5.4 批次 B2：dashboard（T-05/T-06）

- **T-05 z 档与槽位**：D2 落地（role 扩档 + Stack 插层 + 命中带
  化）。
- **T-06 face 材料**：D1 落地（C+ = face:// 前缀臂 + 宿主 mini 栅
  格化 + SWR 翻新；或定案候选）；face 点击→focus 闭环 +
  __dashboard_cmd 上行。

### 5.5 批次 B3：launcher（T-07）

D3 装载形态 + D5 聚焦链 child 化 + LauncherSnapshot 推送（apps 七
平行列表）+ 热键宿主保留 + 键盘流 e2e（搜索/导航/Enter 启动）。

### 5.6 验收与收口（T-08/T-09/T-10）

- **T-08 e2e**：`p036_all_faces_outproc_arm`——五面全 outproc（进程
  边界断言 + 各面渲染/交互闭环 + watchdog 面崩溃粒度[依 D3]）+
  parity 双形态对拍 + 截图 assets/036/。
- **T-09 度量**：boot 时延（编译 vs 解释）、五面 outproc 内存对照
  （对照 034 量化门口径）、交互往返。
- **T-10 文档台账**：§1.16 增量（overlay 面语义/z 档/face:// 词汇/
  编译面轨）+ P030-D1/D3 核销 + 台账 M7-b 行 + desktop-shell-a2r.md
  S3 终态注记。

### 规范增量

| delta_id | add/modify/retire | docs/specs/... target | before/after rule | rationale | acceptance IDs |
|---|---|---|---|---|---|
| SD-01 | add | auto-lang/docs/design/autoui/desktop-protocol-v1.md（§1.16 v1.16 增量 + 顶表行） | before：v1.15 下 overlay 三面+launcher in-proc（P030-D3 D6 边界）、shell child = 解释装载（P030-D1）、surface_role 三档、四张 overlay 快照载体零推送；after：overlay 面全 outproc 语义（exe 拓扑依 D3/z 档依 D2/face 材料词汇依 D1）+ shell-lib 编译面轨（生成目标/双轨）+ 快照载体全激活——PROTOCOL_VERSION 仍 1（追加式） | 协议权威收录 B 形态彻底化 | AC-02..05 |
| SD-02 | modify | auto-os/docs/plans/autos-desktop-program.md | before：M7-b 行 = 待执行；after：交付行（P030-D1/D3 核销指针 + 五面 outproc 终态 + 度量结论） | 桌面程序台账 | AC-06 |
| SD-03 | modify | auto-lang/docs/plans/KNOWN-DEBT-AND-RISKS.md | before：P030-D1（:2383）/P030-D3（:2385）在册；after：双债核销 + P036 新债随注（聚焦 child 化边界/face 实时性边界等） | 债账收口 | AC-06 |
| SD-04 | modify | auto-lang/docs/design/autoui/desktop-shell-a2r.md | before：S3 改道注（B 程序承接）；after：终态注记（五面 outproc + 编译面轨兑现——B 形态彻底化收官） | 设计文档闭环 | AC-06 |

零 spec 影响的变更不存在（进程拓扑/z 档/词汇为协议级知识）；ledger
随 merge 沉淀。

## 6. 测试设计

- **单测（生成/编译面）**：shell-lib crate 编译门（词汇门升级）；
  mount_face 工厂五面断言；child 替换点 trait 面互换（编译组件 ×
  RqProjector 装配单测）。
- **单测（投影/装配）**：四张快照载体推送-消费闭环（指纹门/召唤
  联动）；surface_role 新档编解码 + golden（追加式零漂移）；face://
  前缀臂（命中/SWR 翻新/未命中降级——依 D1）；launcher 聚焦 child
  化路由（依 D5）。
- **单测（宿主装配）**：Stack 槽 per-surface 化插层序；命中带化
  （dashboard 面板矩形/overlay 召唤窗）；watchdog 面崩溃粒度
  （依 D3——连坐 or 独立 respawn）。
- **e2e**：`p036_all_faces_outproc_arm`（§5.6）+ p030 四腿回归 +
  desktop_mcp 五套**双形态**（inproc 回归 + outproc 新基线）+ 截图
  assets/036/。
- **parity**：五面 × {in-proc, outproc} 金样对拍（035 落地后基线）。
- **回归门**：desktop_protocol/session/stage3/rqhost + rust_ui +
  auto-os 桌面 smoke（inproc 缺省零回归[I2]）。

## 7. 验收标准

- **AC-01 shell-lib 编译面**：五件生成 lib crate 编译过（真编译门）；
  child 编译组件替换点在役（同 trait 面）；双轨可切（解释 fallback
  行为不变）；boot 时延数据行（编译 vs 解释）。验证：编译门 + 度量
  行 + parity 冒烟。
- **AC-02 B1 switcher/通知 outproc 全链**：两面 outproc 渲染 +
  召唤动词联动投影 + Esc/键盘路由闭环 + switcher 行缩略图
  （thumbnail://）显示。验证：单测 + e2e 腿 + parity。
- **AC-03 B2 dashboard outproc 全链**：z 档正确（App 窗遮挡/图标
  层之下——R3 语义保持）；face 卡经定案材料方案渲染（D1）+ 点击
  focus 闭环；面板 chrome outproc。验证：单测 + e2e 腿 + parity。
- **AC-04 B3 launcher outproc 全链**：召唤（热键宿主保留）+ 键盘流
  （搜索/导航/Enter 启动 app）+ 聚焦 child 化闭环（依 D5）。
  验证：e2e 键盘流腿 + parity。
- **AC-05 五面终态**：全 outproc 桌面 e2e（进程边界断言——shell/
  desktop/switcher/notification/dashboard/launcher 面全部经
  RenderQueue）；watchdog 面崩溃粒度按 D3 验证。验证：
  p036_all_faces_outproc_arm + 截图。
- **AC-06 文档与回归门**：§1.16 + P030-D1/D3 核销 + 台账/设计文档
  注记落盘互链；§6 回归门全绿（在册既有红除外；inproc 缺省零回
  归[I2]）。

## 8. 执行步骤

**前置**：034 merge ✅（master 含 c4facc584）；**035 merge**（B2
批次硬前置；A/B1 文件错开但 pin hash-lock 先后约定——建议整体 035
后开工）。依赖序：T-01 → {T-02, T-04 并行} → T-03 → {T-05, T-06
串行} → T-07 → T-08 → T-09 → T-10。lang worktree
`D:/autostack/.wt/lang-036/auto-lang`；os `D:/autostack/.wt/os-036/
auto-os`。

- **T-01 [lang] 深水调查与定案** [x]
  文件：SC/SP/R/S 四处（§4 事实锚）、message.rs（role/shell_face）、
  rust_ui.rs（wrap_example）、035 落地后 dashboard.at（读）+
  §5.1（写面）。
  动作：D1–D6 定案（D1 face 材料/D2 z 档/D3 拓扑为用户确认项）。
  产物：`### 5.1 定案记录`（file:line 证据）。
  验证：定案完备；D1-D3 获用户确认。
  → 全 AC 前置。
  [✅ 已完成 2026-09-20] §5.1 定案记录落盘（lang-036 worktree 复核
  §4 全锚点 + 新定案 D4/D7/D8 三件；D1/D2/D3 维持倾向悬置为 T-05/
  T-06/T-07 批次前置——不阻塞批次 A；D5/D6 复核在案待 B1/B3 细化]。
- **T-02 [lang] shell-lib 生成目标** [x]
  文件：`crates/auto-man/src/rust_ui.rs`（无窗组件库形态）+ 词汇门
  升级。
  动作：§5.2 T-02；构建集成依 D4。
  验证：crate cargo build 编译门绿。
  → AC-01。
  [✅ 已完成 2026-09-20] generate_shell_pack_lib 生成器 + 入库物
  `crates/shell-pack/`（workspace 成员 = 每次 cargo build 即真编译门）
  + freshness 字节对拍门（auto-man test_shell_pack_lib_freshness 绿；
  生成器确定性实证：prop/event 迭代序确定化后两次生成逐字节同）+
  #[ignore] regen_shell_pack 重生成入口。真编译门清偿 a2r 七族缺口
  （button icon PUA/裸点条件下划线/.Variant 自消息派发/Eq-Neq 跨型
  降串三臂/单支 if-expr else 补全/String += 分型/WorkspacePreview
  数值键降串/contains Pattern）+ truncate 真渲⑫（018 债清偿）。
  lang-036 @ 243bc5bd。
- **T-03 [lang] child 替换 + 双轨** [x]
  文件：`shell_client.rs`（替换点）+ shell_model 配置面。
  动作：§5.2 T-03。
  验证：互换 parity 冒烟 + 度量行。
  → AC-01。
  [✅ 已完成 2026-09-20] ShellFaces 接缝 trait 化（ShellSurface/
  ShellStateAccess + FaceProjector 泛型适配体——解释臂行为逐字节
  不变）；编译装配点 cmd_autodesk（缺省编译轨 + 显式 AUTO_SHELL_PACK
  = 解释双轨[027 D5 承袭]）；spawn 注入收窄（仅显式命中注入——
  session.rs resolve_shell_pack_explicit）。结构性 parity 冒烟绿
  （shell-pack tests 4/4：五面 mount/写臂全覆盖/wire 投影 roundtrip/
  命令 drain 幂等/渲染非空）；boot 时延度量行：**interpreted 25.7ms
  vs compiled 0.8ms（约 32×）**。字节级 parity 对拍入 T-08 验收。
  lang-036 @ 243bc5bd。
- **T-04 [lang] B1 switcher + 通知** [x]
  文件：R（推送泵/注入点）、SC（两面装配）、S（伪窗/召唤联动）。
  动作：§5.3；D6 键盘语义。
  验证：两面闭环单测 + e2e 腿 + parity。
  → AC-02。
  [✅ 已完成 2026-09-20] D6 定案落地（宿主键盘截获 → ShellEvent 扩档
  tag6-9 随快照 events 下行 → child dispatch 派发同名 Msg；指纹门
  放行 = fp 变化 || events 非空）；两载体 interpreted_writes + 键序
  单测；child 四面装配（OVERLAY role 追加档 + 声明序映射 + faces-map
  懒装/预装 + 投影臂命令排水）；宿主七处分叉（in-proc 零变化 I2）+
  可见性镜像位 + 层序全屏贴层 + attach overlay 伪窗（registry_id
  归因 + 投影/渲染过滤）；thumbnail:// 零新 wire（既有臂直用）。
  门：97/97 + 14/14 + shell-pack 4/4 + auto-man 312/312 +
  desktop_protocol 183/184（imagesurface 在册红）+ auto check 0 错。
  随注：分区预览自隐计时 outproc 失效（载体无 switcher_open 键）；
  字节级金样对拍归 T-08。lang-036 @ 95c4ff066（rebase 后批次 A =
  b777e6532 + 035-sync regen 提交）。
- **T-05 [lang] B2 z 档与槽位**
  文件：`message.rs`（role 扩）、R（Stack 插层/命中带化）、S（伪窗）。
  动作：§5.4 T-05；D2 落地。
  验证：role golden + 插层序单测。
  → AC-03。
- **T-06 [lang] B2 face 材料**
  文件：R（mini 栅格化/face 解析依 D1）、`broker_surface.rs`
  （face:// 前缀臂依 D1-C+）、SC（face 点击/命令上行）。
  动作：§5.4 T-06；D1 落地。
  验证：face 渲染/focus 闭环单测 + e2e 腿。
  → AC-03。
- **T-07 [lang] B3 launcher**
  文件：S/R（装载形态依 D3）、SC（聚焦 child 化依 D5）、热键边界。
  动作：§5.5。
  验证：键盘流 e2e 腿 + parity。
  → AC-04。
- **T-08 [lang+os] e2e 五面终态**
  文件：lang `stage3.rs`（p036_all_faces_outproc_arm）+ assets/036/。
  动作：AC-02..05 逐条留痕。
  → AC-02..05。
- **T-09 [lang] 度量**
  文件：度量行（boot 时延/内存对照/交互往返）落 reports/。
  动作：§5.6 T-09。
  → AC-01/05 佐证。
- **T-10 [lang+os] 文档与台账收口**
  文件：lang `desktop-protocol-v1.md`（§1.16）+ KNOWN-DEBT（双债
  核销）+ `desktop-shell-a2r.md`（终态注记）；os 台账 M7-b 行 +
  互链。
  动作：SD-01..04 落笔。
  → AC-06。

## 9. 复审记录

- 2026-09-20 /auto-plan:new 起草交接：`stage: new`，PLAN-036 rev 1
  （035 号被并行会话取用于 desktop-ux-rev3，本计划顺延 036）。M7-b
  波次（P030-D1/D3 承接）。`outcome: pass`（合同完整：030 v1 机器/
  四面 in-proc 装配/z 序与 role 三档/投影材料缺口表/shell-lib 生成
  需求与词汇门/035 协调面全部 file:line 在案）；`next: work`——
  **前置 = 034 ✅ + 035 merge**（B2 硬前置；建议整体 035 后开工）。
  悬置决策 §10（①–④），D1 face 材料/D3 拓扑为用户确认项，不阻塞
  T-02/T-04 先行。
- 2026-09-20 /auto-plan:work 批次 A 执行记录（中间执行——计划整体
  保持 executing）：worktree lang-036 @ master 608399943 分叉、
  提交 243bc5bd（16 文件 +2587/-170）；组内 auto-down 依赖 worktree
  `.wt/lang-036/auto-down` @ detach d1a83b6（a2r-actor-tests manifest
  解析所需，034 组同型）。T-01 定案（D4/D7/D8 定案 + D1/D2/D3 悬置
  批次前置）→ T-02 生成目标（真编译门绿：词汇表超记实臂缺位清偿
  a2r 七族 + truncate 真渲⑫[018 债清偿翻 Covered] + 生成器确定性
  实证）→ T-03 替换+双轨（接缝 trait 化 + cmd_autodesk 编译轨缺省 +
  spawn 注入收窄 + 度量行 25.7ms→0.8ms）。门：shell-pack 4/4 +
  freshness 绿 + auto check 0 错 + ui_gen 812/814（2 在册红基线
  stash 实证同红：mouse_area/autodown_panel）+ desktop_protocol
  182/183（imagesurface 在册红基线同红）+ session/shell 93/93 +
  auto-man 312/313（修前 freshness 过期红已转绿）。B1（T-04）起
  挂 035 merge 前置（§8 依赖序维持；035 状态 executing——其 §9 已
  记 work 完成待 review）。`stage: work | PLAN-036 | rev 1 |
  outcome: pass（批次 A）| code_commit: lang-036 243bc5bd |
  task_ids: T-01,T-02,T-03 | evidence: 上述门 | blockers: 035 merge
  （B1-B3 前置）| next: 035 merge 后 T-04`。随注三件：①auto-man
  测试运行有 examples 再生成副作用（worktree 内曾误裁 rust-workspace
  成员——278f71f35 同型反模式，已 amend 回滚；015-notes main.rs
  再生成保留[反映本提交生成器修复]）；②smoke-030/p030 e2e 腿后续
  重跑将默认吃到编译轨 child（行为预期变化，T-08 对拍面）；③专用
  shell exe（彻底摆脱 re-exec）维持 D4 注记另立。
- 2026-09-20 /auto-plan:work T-04 执行记录（B1；035 merge 解锁后）：
  前置同步 = lang-036 rebase master 899db807f（批次 A 落位 b777e6532）
  + shell-pack regen（035 pack 变化 +21/-3 流入）+ os-036 ff 24aa01f
  + 组内 auto-down 依赖 worktree 重建（并行清理误删——detached b422385）。
  交付 = B1 全链（D6 事件位方案 + 两载体写集 + child 四面 + 宿主七处
  分叉 + 伪窗 + 层序 + thumbnail 既有臂）。提交 95c4ff066（10 文件
  +800/-81）。门见 T-04 行。p030 e2e 腿0 断言已放宽（2/4 伪窗——
  smoke 重跑时生效）。`stage: work | PLAN-036 | rev 1 | outcome:
  pass（B1）| code_commit: lang-036 95c4ff066 | task_ids: T-04 |
  evidence: 上述门 | blockers: D1 face 材料用户确认（T-06 前置）+
  D3 拓扑用户确认（T-07 前置）——D2 可按倾向 A 先行 | next: T-05
  （D2 落地）→ T-06（D1 裁定后）`。

## 10. 待澄清事项

- **①（T-01 D1，用户确认项）** face 卡材料：C+ 宿主栅格化 +
  `face://{appid}` 虚拟引用下行（推荐——028 同型零 wire + SWR 翻
  新）vs A mini 会话搬 child（大改）vs B 半 outproc（z 只解一半）
  vs C 位图过线反向（实时性受限）。
- **②（T-01 D2）** dashboard z 档：surface_role 追加 DASHBOARD 档
  （推荐——协议显式）vs 宿主槽位 hack（零协议但无锚）。
- **③（T-01 D3，用户确认项）** overlay exe 拓扑：混合（推荐——
  三 overlay 并壳 exe 多表面 + launcher 一面一 exe）vs 全并壳 vs
  四独立 exe（崩溃粒度 vs 装配成本）。
- **④（T-01 D4）** 编译/解释双轨缺省方向：缺省编译（发布态收益）
  vs 维持解释缺省（保守）——shell-lib 构建集成面定。
