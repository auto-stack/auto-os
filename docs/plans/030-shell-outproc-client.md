---
plan_id: PLAN-030
status: drafting               # drafting → executing → execution_done → reviewed → archived
feature_name: shell-outproc-client
author: [agent]
created_at: 2026-09-19
updated_at: 2026-09-19
plan_revision: 1
current_step: 0
total_steps: 10

# /auto-plan:review 结束时填写：
supersedes_spec_components: []
new_spec_components:
  - auto-lang/docs/design/autoui/desktop-protocol-v1.md   # §1.11 v1.11 增量（review 定稿）
touched_goals: []

affects:
  - auto-lang/crates/auto-lang/src/ui/desktop_protocol/message.rs        # 投影下行推/命令上行执行/表面 z 平面声明（ControlMsg 增量）
  - auto-lang/crates/auto-lang/src/ui/session.rs                         # broker DesktopBus 执行臂/投影推送泵/看门兵/壳孵化
  - auto-lang/crates/auto-lang/src/ui/iced/renderer.rs                   # 壳层合成改双表面消费 + 启动序 + 降级门
  - auto-lang/crates/auto-lang/src/ui/shell.rs                           # 壳产物形态/装配入口（编译 exe 轨）
  - auto-lang/crates/auto-lang/src/ui/shell_projection.rs                # 027 休眠 typed 载体激活（wire 编解码消费）
  - auto-lang/crates/auto-lang/src/ui/desktop_protocol/client_entry.rs   # 壳客户端入口（--autodesk-shell 装配位）
  - auto-lang/docs/design/autoui/desktop-protocol-v1.md                  # §1.11 v1.11 增量
  - auto-lang/docs/design/autoui/desktop-shell-a2r.md                    # §10-① 前置序列收口（第三件/终件）
  - auto-os/docs/plans/autos-desktop-program.md                         # 台账 3c3 行
---

# [PLAN-030] shell-outproc-client

## 0. 变更摘要

B 形态前置序列前两件已收官（图像 DrawOp 通道 PLAN-028 / live 输入 +
shell queue 面 PLAN-029），B 演进门三前置全绿。本计划交付**B 程序主体
= shell outproc client + 启动序 + 看门兵**：

**①协议增量**：投影下行推（027 休眠 typed 载体 = ShellProjection/
五面快照/ShellEvent/ShellClock 转 wire——`ControlMsg` 追加式变体）
+ 命令上行执行（`ControlMsg::DesktopBus` broker 路径落地——现状
实锤被丢弃，desktop.* wire 化清偿）+ 表面 z 平面声明（壳双表面：
background[壁纸上/窗口下] + chrome[置顶]——复用既有 per-client 多
表面基建）。**②壳编译客户端**：a2r 生成轨（027 五件词汇门产物）+
`DesktopBusHandle`/`HostStorage` 装配激活（027 "T-08 编译壳装配用"
休眠接缝清偿）+ NativeProjector queue 臂（029 五件 Covered 前提）。
**③宿主侧**：壳孵化（boot 序：宿主起→broker→spawn 壳→attach→
chrome 就绪；失败降级链）+ 双表面合成层位 + 输入路由（chrome 置顶
命中优先/壳焦点键盘/live 输入 029 直用）+ 投影泵改推（指纹门保留）。
**④看门兵**：壳死亡检测（EOF 泵既有）→ 重启（spawn 退避预算）→
全量投影重推 → 恢复；**⑤双轨开关**：`shell.apps.shell_model:
inproc|outproc`（ProcessModel 先例）——**缺省维持 inproc**，outproc
为实证形态（缺省翻转随实机浸润另立裁定，3a 行先例同口径）。
**⑥e2e + 文档**：p030 真子进程壳全链 + 桌面 smoke + SendInput 真机
腿（启动序基建就绪后激活——029 挂账清偿）+ §1.11 + 台账 3c3。

launcher 懒挂载面**维持注册表 in-proc 现状**（SHELL_MANIFEST 排除
launcher——registry path，027 D5 口径）；懒 overlay 三面
（switcher/notification/dashboard）v1 归属由 T-01 D6 定夺。

## 1. 目标

- **G1 协议增量（追加式）**：投影下行推通道（ShellProjection 五面
  载体 + ShellEvent + ShellClock 编解码，指纹/节拍语义入册）+ 命令
  上行执行臂（broker DesktopBus → parse_records → 52 动词执行复用
  + registry_id 归因）+ 表面 z 平面声明（background/chrome 双表面
  协商）；`PROTOCOL_VERSION` 维持 1（ControlMsg 追加式纪律）。
- **G2 壳编译客户端**：shell pack 五件 a2r 产物 + 装配（ShellProjection
  apply/ DesktopBusHandle 上行 / HostStorage / NativeProjector）+
  `--autodesk-shell` 入口；渲染 = queue 臂 DrawList。
- **G3 宿主合成与输入**：boot 孵化序 + 双表面层位（background
  z-bottom / chrome z-top）+ chrome 命中优先路由 + 壳焦点键盘
  （live 输入直用）+ 投影泵改推（事件驱动 + 指纹门保留）。
- **G4 看门兵**：壳进程死亡检测 → 退避重启 → 全量投影重推 → 表面
  恢复；降级容忍（重启预算耗尽 = 降级观测，桌面不炸）。
- **G5 双轨开关**：`shell.apps.shell_model` 配置位 + 双轨并存
  （inproc 解释轨零回归——开发态 fallback 常驻，a2r §10-① 双轨
  裁定原文）；缺省 inproc。
- **G6 e2e 与文档**：p030_shell_outproc_arm（真子进程壳：五面渲染 /
  任务栏点击→窗口操作联动 / 投影推送→dock 帧变 / 杀壳→看门兵→恢复）
  + 桌面 smoke 真机腿 + SendInput 真机腿（前台断言 + 观察 channel）
  + §1.11 v1.11 增量 + a2r 前置序列收口 + 台账 3c3 行 + 029 挂账
  （SendInput/IME 候选窗定位随注核对）。

**非目标**（明确出界）：

- **缺省翻转**（outproc 实证达标后的 default 裁定另立——实机浸润
  数据驱动，3a 行 process_model 先例同口径）。
- **compositor 迁移**（终态图景"桌面进程含 compositor"远期——本
  计划宿主仍为 compositor + WM 权威，Design 23 R1 修订语义不变）。
- **launcher outproc 化**（注册表路径 .at 懒挂载维持现状——SHELL_
  MANIFEST 排除口径）。
- **懒 overlay 三面 outproc 化**（若 T-01 D6 裁定 v1 仅常驻双面，
  则 switcher/notification/dashboard 维持 in-proc 懒挂载，outproc
  化随缺省翻转计划）。
- **第三方 shell / shell 独立发布**（B 演进门第四条"真实需求"维持
  观察）。
- 既有 in-proc 解释壳行为回归（I1：双轨下解释轨零漂移——投影写序/
  命令排空/事件管线全部保留）。

## 2. 架构方案

```text
┌─ 协议增量（message.rs，追加式）───────────────────────────────────┐
│ ControlMsg 追加：                                                   │
│   ShellProjectionPush { face, payload }   # 宿主→壳：五面 typed    │
│                                           # 载体 wire 编码（027    │
│   ShellClockTick { .. }                   # ShellProjection 家族） │
│   （ShellEvent 随 Push 载体随行）         # 独立脏帧通道（027 D1） │
│ DesktopBus（既有变体）broker 路径落地：    # 壳→宿主：52 动词上行   │
│   broker_apply_actions 增臂 → parse_records → execute_desktop_     │
│   commands 复用 + registry_id 归因       # desktop.* wire 化清偿   │
│ 表面 z 平面：Hello/Welcome 协商扩平面声明（background|chrome|       │
│   window 既有）——复用 per-client 多表面（wid_surface BTreeMap      │
│   基建）                                 # T-01 D1 实勘定形       │
└──────────────────────────────────────────────────────────────────────┘
┌─ 壳编译客户端（client_entry.rs --autodesk-shell）──────────────────┐
│ a2r 五件产物（027 词汇门）+ 装配：                                    │
│   ShellSurface 编译壳 impl：apply_projection（027 D2 候选 B 同款）  │
│   DesktopBusHandle（027 D3 单方法枚举载荷）→ ControlMsg::DesktopBus │
│   HostStorage ← ShimHostStorage 同后端                               │
│   NativeProjector queue 臂（ensure_covered 门——029 五件 Covered）   │
│ 双表面：background + chrome 各持 DrawList（z 平面声明随 Hello）      │
└──────────────────────────────────────────────────────────────────────┘
┌─ 宿主侧（renderer.rs/session.rs）───────────────────────────────────┐
│ boot 序：宿主窗→open_desktop→broker→[shell_model=outproc?] spawn    │
│   壳 exe→attach→表面注册→chrome 就绪；失败→降级链（D7 定）          │
│ 合成：background 表面帧铺 z-bottom（替代壁纸+desktop face 层）→     │
│   虚拟窗 z_order → chrome 表面帧 z-top（替代 taskbar/overlay 层）   │
│ 输入：WM z 序含两伪表面（chrome 置顶命中优先）；键盘 = 焦点表面      │
│   （壳面聚焦）+ live 输入 029 直用 + 热键宿主保留                    │
│ 投影泵：sync_shell_windows 指纹门保留 → 命中 outproc 壳时改推       │
│   ShellProjectionPush（事件驱动 + ServiceTick 兜底——现节拍不变）    │
└──────────────────────────────────────────────────────────────────────┘
┌─ 看门兵（session.rs 新模块）────────────────────────────────────────┐
│ pump_broker_clients 壳死亡检出（EOF 既有）→ respawn 队列：           │
│   退避（1s/2s/5s 封顶，预算 N 次/窗口）→ spawn_exe_child 复用 →     │
│   attach → 全量投影重推（指纹强制失效）→ 表面恢复；预算耗尽 =        │
│   降级观测 + 桌面不炸（I5）                                          │
└──────────────────────────────────────────────────────────────────────┘
```

**不变式**：

- **I1 双轨零回归**：`shell_model=inproc`（缺省）路径字节级零变化
  ——投影写序/命令排空/事件管线/层位全部保留；outproc 为纯增量臂。
- **I2 追加式协议**：新 ControlMsg 变体追加；既有变体语义零改动；
  `PROTOCOL_VERSION` 维持 1。
- **I3 投影语义零漂移**：ShellProjection 过线 = 027 typed 载体 wire
  编码（叶面保形 D1 原则——"1"/"" lowering 单点）；指纹门/节拍语义
  与 in-proc 轨同册（I9 投影唯一事实：宿主 WM 仍为唯一事实源）。
- **I4 命令单源**：DesktopCommand 52 动词 parse_records/encode 既有
  单点直用——上行只换通道不改词表（027 I2 延续）。
- **I5 降级容忍**：壳 spawn/重启失败 = 降级桌面（现状 boot 容忍语义
  延伸），桌面不炸（catch_unwind 边界 + 降级观测）。

**关键风险**：双表面 z 平面与 WM 命中序的交互（chrome 伪窗与真窗
z_order 交织——D3 实勘）；a2r 壳产物对投影载体的编译期类型契合
（027 词汇门保障但装配面新写）；投影推送的帧率/带宽（31 字段 ×
update 排空点节拍——指纹门 + 分面载荷限幅）；看门兵重启风暴
（退避 + 预算 + 观测）；launcher/overlay 混合层位（in-proc overlay
与 outproc chrome 同屏——D6 裁定 v1 边界规避）。

## 3. 技术栈

Rust / iced 0.14 daemon（宿主）；desktop_protocol wire（ControlMsg
追加式）；027 typed 载体（ShellProjection 家族 + DesktopBusHandle +
HostStorage——休眠激活）；a2r 生成轨（027 五件词汇门产物）；029
交付（live 输入 route_live_input / NativeProjector 四 kind / 词汇表）；
broker 孵化（spawn_exe_child/incubation/attach 既有）；mem_guard
watcher 线程先例（看门兵线程形态参照）；ProcessModel 配置位先例
（shell_model 双轨）；e2e 载体 = 真子进程壳（AUTO_DESKTOP_E2E 门，
p025/p029 harness 形态）+ 桌面 smoke（os scripts）+ SendInput FFI
（029 sendinput.rs——前台断言 + 观察 channel）。

## 4. 需求分析与背景调查

**授权记录**：用户 2026-09-19 会话（PLAN-029 merge 收官后）明确
"OK，推进立项"——本计划即 B 程序主体立项（a2r §10-① 前置序列第
三件/终件）。**本轮仅规划，未授权实施**。涉及仓：auto-lang（协议/
宿主/客户端/文档）+ auto-os（台账/smoke/可能 pack 微调）。无预算/
自动续跑约束声明。**无前置计划依赖**（029 已 merge 即基线 lang
56d1596f2 / os 82bbc7c）。

**现状事实**（已核，2026-09-19，探索代理三路普查）：

- **壳现形态 = in-proc DynamicComponent**：boot 期 `build_shell_
  component()` 编译装载（renderer.rs:14399-14416，五面 builder 于
  ui/shell.rs——`shell_source` 解析序：override→AUTO_SHELL_PACK→
  兄弟 auto-os→硬编码→嵌入 pin 快照 + hash parity 测试）；渲染走
  解释 iced 层栈（view_desktop_fn 18574：壁纸→desktop face→虚拟窗
  →taskbar→launcher/switcher/notification overlay→desktop_root）。
- **宿主→壳状态注入面（~50 `__` 变量）**：主泵 `sync_shell_windows`
  （13672-13708）经 `build_shell_projection`→`ShellProjection::
  interpreted_writes()`（shell_projection.rs:305-341，**17 键指纹门
  控 `__wm_fp`**）——**typed 派生已生产在用**；其余分面 ad-hoc：
  clock（update_shell_clock 9507 分钟门）/dock 三键（apply_dock_
  edges_now 10681）/desktop surface 八键（inject_desktop_surface
  13354）/cursor 双键（__mouse_moved 18333 无 view_dirty）/wallpaper
  picker 八键（11340）/switcher·notification·dashboard·launcher
  召唤注入（9950/10019/10386/9807）。节拍 = **事件驱动（每 update
  排空点邻位 17365）+ ServiceTick 400ms 兜底**。
- **壳→宿主命令**：`__desktop_cmd` 状态排空（drain_desktop_commands
  session.rs:2802）→ `DesktopCommand`（session.rs:1342-1499，**52
  变体**）→ `execute_desktop_commands`（renderer.rs:10807，52 臂）；
  记录格式 `verb\u{1F}arg`（encode/parse_records 单点）。
- **027 休眠接缝（B 的 wire 词汇）**：`ShellProjection`（96-126，
  派生生产在用）、`ShellManifest`（344-393，五件清单——launcher
  排除）、`DesktopBusHandle/DesktopBusQueue`（session.rs:4980-5019，
  "T-08 编译壳装配用"零生产消费）、`HostStorage/ShimHostStorage`
  （5025-5045）、五面快照 + ShellEvent + ShellClock（dormant）；52
  动词 roundtrip 测试在册（5053-5146，词表规模守门 ≥46）。
- **outproc 基建**：孵化握手（broker.rs 191-217/91-130）+ attach
  （session.rs:3344-3412）+ 帧合成（broker_composed → DrawListPainter
  /pixels 上传）+ 输入下行（broker_*/route_live_input 029）全链在
  册；**per-client 多表面基建存在**（BrokerClient.wid_surface
  BTreeMap + SurfaceStore）。
- **三缺口实锤**：①`ControlMsg::DesktopBus`（壳→宿主）在 broker
  路径**被丢弃**（endpoint.rs:565-570 透传 + broker_apply_actions
  无该臂——desktop.* wire 化缺口）；②宿主→子进程**无持续状态
  推送**（StateSnapshot L3 v2a 接收端在册但零生产发送方；attach 后
  仅 Input 下行）；③**无子进程自愈**（死亡→wm_remove_win 回收，
  ReconnectPolicy 仅子侧 30s 重连；outproc_children 注释"显式生命
  周期管理非 v1 面"）。
- **boot 鸡蛋序**：宿主对壳**零依赖**（WM 自 open_desktop 起存在；
  壳装载失败 = 无任务栏退化桌面容忍注释 14398；dock 预留边来自
  config 非壳）——启动序无硬阻断。
- **看门兵先例**：mem_guard watcher（独立线程 1s 采样 + FREEZE_
  HOOK，mem_guard.rs:226）为唯一 in-tree watchdog；daemon ensure
  链（ping→spawn→poll，osconfig_daemon.rs）为拉起先例；子进程
  respawn 零先例（本计划新增面）。
- **设计裁定链**：Design 23 R1（特权 App 拥有窗口语义/宿主只管合成）
  → a2r §5 B 裁定（2026-09-18 用户裁定）+ §10-① 前置序列（028 ✅
  029 ✅ → 本件）→ 027 r2（S1/S2 形态无关资产 = B wire 词汇；S3 已
  改道 B）→ 台账 3c/3c1/3c2 + 行 5/6（R8-R12/I7-I9：shell 无几何
  操作/表面双端同源/投影唯一事实）。
- **029 挂账（本计划清偿/核对）**：SendInput 真机腿（依赖启动序 +
  child 观察——启动序交付后激活）；IME 候选窗定位（ImeCursor 形态
  归 B——shell 输入面需求驱动，本计划随注核对不强制交付）；桌面
  smoke 真机腿（"B 程序立项后随启动序一并"）。

## 5. 详细设计

### 5.1 T-01 深水调查定案（决策产物）

- **D1 表面模型**：双表面 z 平面（background[壁纸上/窗口下全屏] +
  chrome[置顶全屏]）vs 单表面 + 宿主层序钩子 vs 五面各一客户端。
  **倾向**：双表面单客户端（z 平面声明随 Hello/surface 协商——
  per-client 多表面基建直用；chrome 全屏透明命中由 DrawList 空
  区直通实现——宿主命中序按 z 平面查 chrome 伪窗→真窗→background
  伪窗）。实勘：wid_surface/SurfaceStore 多表面消费面、Welcome
  协商扩展形态、DrawListPainter 空区命中语义。
- **D2 投影下行通道**：`ControlMsg::ShellProjectionPush{face,
  payload}`（027 载体 wire 编码——叶面保形 D1）+ ShellClock 独立
  变体 + ShellEvent 随行 vs StateSnapshot 复用（raw KV——类型面弱
  且 50 变量面散）。**倾向**：typed Push 新变体（027 SD-02 "wire
  payload 词汇基础"原文兑现）；指纹门宿主侧保留（未变不推）；节拍
  = 现事件驱动 + tick 兜底不变。实勘：payload 尺寸/编码黄金、分面
  增量（仅脏面推）可行性。
- **D3 输入路由**：chrome 伪窗置顶命中优先（WM z_order 插序）+
  壳面聚焦键盘（live 输入 029 焦点窗语义直用——chrome 伪窗可为
  focused）+ 热键宿主保留（desktop_hotkey_subscription 不动）+
  background 伪窗承接桌面空白点击（BlankPress 语义）。实勘：WM
  hit_test 伪窗插序形态、GlobalPress 与 chrome 命中的序、
  `__mouse_moved/__desktop_cursor` 投影推送节流（cursor 无
  view_dirty 现语义——过线节流定案）。
- **D4 命令上行**：broker_apply_actions 增 DesktopBus 臂 → record
  解析（parse_records 单点）→ execute_desktop_commands 复用（渲染
  线程归属/任务合成衔接实勘）+ registry_id 归因（a2r c4 缺口行）；
  acceptance 通道适配（029 DesktopInject::Key/Bus 走 in-proc
  `__desktop_cmd` 语义——outproc 壳下注入路径定案）。
- **D5 看门兵形态**：独立 watcher 线程（mem_guard 先例）vs
  ServiceTick 臂内联。**倾向**：ServiceTick 臂内联（死亡检出已在
  pump_broker_clients tick 内——零新线程；respawn 队列 + 退避
  1s/2s/5s 封顶 + 预算 3 次/60s 窗 + 全量重推（指纹强制失效）+
  预算耗尽降级观测）；与 mem_guard FREEZE_HOOK 的关系随注。
- **D6 v1 面边界**：SHELL_MANIFEST 五件全 outproc 单 exe（懒态 =
  投影可见性驱动，chrome 表面统一承载 overlay 态）vs 仅常驻双面
  （shell taskbar + desktop surface）outproc + 懒 overlay 三面维持
  in-proc 懒挂载。**倾向**：五件全 outproc（避免混合层位）——
  前置实勘：in-proc launcher overlay 与 outproc chrome 同屏层序
  冲突与否；若冲突则 v1 收常驻双面，overlay 随缺省翻转计划。
- **D7 双轨开关与降级**：`shell.apps.shell_model: inproc|outproc`
  存储键（ProcessModel::from_storage 先例）+ boot 分叉 + outproc
  装载失败降级链（spawn 失败→回退 inproc + 观测 vs 无壳桌面——
  **倾向**回退 inproc，开发友好）；a2r 产物生成/装配组织（壳 exe
  构建产物路径、`--autodesk-shell` 参数面、五件单 binary）。
- **D8 e2e 与真机腿口径**：p030 harness（真子进程壳——p029 t3
  native 档扩展 vs 独立壳 exe spawn）；SendInput 真机腿（前台断言
  + child 观察 channel——启动序就绪后激活；观察 = 壳投影推送侧效
  日志 or acceptance 通道）；桌面 smoke（os 仓 scripts 形态）。

定案记录追加 `### 5.1 定案记录`，作为 T-02..T-09 依据。

### 5.2 协议增量（T-02）

ControlMsg 追加变体（ShellProjectionPush/ShellClockTick）+ DesktopBus
broker 执行臂 + 表面 z 平面声明（Hello/Welcome 协商扩展）+ 编解码
round-trip/golden + TS 侧 decode 对拍（若 Control 面 TS 在册）。

### 5.3 壳编译客户端（T-03）

a2r 五件产物装配（ShellSurface 编译壳 impl——apply_projection
[027 D2 候选 B]/DesktopBusHandle 上行/HostStorage 接驳）+
NativeProjector 双表面 + `--autodesk-shell` 入口 + ensure_covered 门
（029 Covered 前提下应过——门内实体为编译产物视图）。

### 5.4 宿主合成与输入（T-04）

boot 孵化序（shell_model 分叉 + spawn + attach + 表面注册 + 降级
链）+ 双表面层位（background 替壁纸/desktop-face 层、chrome 替
taskbar/overlay 层——inproc 轨层位代码保留双轨）+ WM 伪窗插序 +
chrome 命中优先 + 焦点键盘 + 投影泵改推（指纹门保留 + 分面增量）。

### 5.5 看门兵（T-05）

respawn 队列 + 退避/预算 + 全量重推 + 恢复断言 + 降级观测 +
acceptance/观测通道适配。

### 5.6 投影与命令全链（T-06）

52 动词上行过 broker 执行（逐族抽样断言）+ 投影推送→壳帧变全链
（窗口开关→dock 列表帧变）+ registry_id 归因。

### 5.7 双轨与回归（T-07）

shell_model 配置位 + inproc 轨零回归门（既有 shell 测试族全绿）+
双轨并存断言。

### 5.8 e2e 与真机（T-08）

p030_shell_outproc_arm（五面渲染/任务栏点击联动/投影推送帧变/
杀壳→看门兵→恢复四腿）+ 桌面 smoke + SendInput 真机腿（前台断言）
+ 帧留痕 assets/030/。

### 5.9 文档收口（T-09）

§1.11 v1.11 增量 + a2r §10-① 前置序列终件 ✅ + 台账 3c3 行 +
overview provisional + 029 挂账核对（SendInput 清偿/IME 随注）+
KNOWN-DEBT 新债登记（如降级/预算边界）。

### 规范增量

| delta_id | add/modify/retire | docs/specs/... target | before/after rule | rationale | acceptance IDs |
|---|---|---|---|---|---|
| SD-01 | add | auto-lang/docs/design/autoui/desktop-protocol-v1.md（§1.11 v1.11 增量） | before：投影下行/命令上行无协议通道（ShellProjection 派生仅进程内消费；DesktopBus broker 路径丢弃）；after：投影下行推（typed 载体 wire 编码 + 指纹/节拍语义）+ DesktopBus 上行执行（52 动词 + registry_id 归因）+ 表面 z 平面声明（background/chrome）+ 看门兵语义入册；PROTOCOL_VERSION 仍 1（ControlMsg 追加式） | B 程序主体协议面权威版本化 | AC-01/02/03/06 |
| SD-02 | modify | auto-lang/docs/design/autoui/desktop-shell-a2r.md | before：§10-① 前置序列"剩余 = shell outproc client + 启动序/看门兵（可立项）"；after：终件交付标注（B 形态落地 = outproc 实证形态；缺省翻转另立；launcher/overlay 边界按 D6 定案随注） | 设计文档前置序列收口 | AC-07 |
| SD-03 | modify | auto-os/docs/plans/autos-desktop-program.md | before：3c2 行 = 前置序列第二件；after：增 3c3 行（B 程序主体交付：协议增量/编译客户端/启动序/看门兵/双轨开关/e2e 结论） | 桌面程序台账 | AC-07 |
| SD-04 | add | auto-lang/docs/specs/auto-lang/ui/overview.md（provisional 节） | before：无 shell outproc 条目；after：B 形态 provisional 指针节（028/029 同款） | 模块 spec 对齐实现 | AC-01/02/03 |

零 spec 影响的变更不存在（协议通道/表面模型/生命周期为协议级知识）；
ledger（os `.autoos/specs.json`）随 merge 沉淀。

## 6. 测试设计

- **单测（wire）**：新 ControlMsg 变体 round-trip + golden bytes +
  载体编码叶面保形（对拍 interpreted_writes 17 键语义）；DesktopBus
  上行 52 动词逐族抽样过 broker 执行；坏 tag/未知 face 拒收。
- **单测（宿主）**：投影推送指纹门（未变不推/变了推/强制失效）；
  分面增量；WM 伪窗插序命中（chrome 优先/背景承接/真窗 Sandwich）；
  看门兵状态机（检出→退避序列→预算耗尽降级→恢复重推）；shell_model
  双轨分叉；降级链（spawn 失败→回退/无壳）。
- **单测（客户端装配）**：编译壳 apply_projection 行为对拍（对拍
  in-proc interpreted_writes 等价——I3 零漂移）；DesktopBusHandle
  上行队列；ensure_covered 过门。
- **e2e（AUTO_DESKTOP_E2E）**：`p030_shell_outproc_arm`——真子进程
  壳四腿（五面渲染帧断言 / 任务栏按钮点击→窗口操作全链 / 投影推送
  →dock 列表帧变 / kill 壳→看门兵退避重启→投影重推恢复）；帧留痕
  assets/030/。
- **真机腿**：桌面 smoke（os scripts）+ SendInput 腿（前台断言 +
  观察 channel——D8 定案形态；不可达时 acceptance 通道承载 + 留痕
  dual 口径）。
- **回归门**：desktop_protocol/session/stage3/shell（inproc 轨
  shell 测试族）/dual_mode + ts_fixtures（若 TS 面触碰）+ auto-os
  桌面 smoke。

## 7. 验收标准

- **AC-01 协议增量**：投影下行推 + 命令上行执行 + z 平面声明
  wire 在册（round-trip/golden 绿 + PROTOCOL_VERSION 仍 1）。
  验证：单测 + §1.11。
- **AC-02 壳编译客户端**：五件 a2r 产物经 `--autodesk-shell` 孵化
  为 outproc 客户端，双表面 queue 臂渲染 + ensure_covered 过门。
  验证：装配单测 + e2e 腿 1。
- **AC-03 宿主合成与输入**：boot 孵化序（含降级链）+ 双表面层位 +
  chrome 命中优先 + 焦点键盘 + 投影改推全链绿。验证：单测 + e2e
  腿 1/2/3。
- **AC-04 命令全链**：52 动词上行过 broker 执行（逐族抽样）+
  registry_id 归因。验证：单测 + e2e 腿 2。
- **AC-05 看门兵**：杀壳→检出→退避重启→全量重推→恢复；预算耗尽
  降级观测。验证：状态机单测 + e2e 腿 4。
- **AC-06 双轨零回归**：`shell_model=inproc`（缺省）既有 shell 测试
  族全绿 + 双轨并存断言。验证：回归门。
- **AC-07 文档与台账**：§1.11 + a2r 序列收口 + 台账 3c3 + overview
  provisional + 029 挂账核对（SendInput 真机腿清偿或 dual 口径
  留痕；IME 随注）落盘互链。验证：文档锚点 + 交叉引用可解析。
- **AC-08 回归门**：§6 回归门全绿（在册既有红除外）。

## 8. 执行步骤

依赖序：T-01 → {T-02, T-03 并行} → {T-04, T-05 并行} → T-06 →
T-07 → T-08 → T-09。lang worktree `D:/autostack/.wt/lang-030/
auto-lang`（+ 组内 auto-down 只读依赖）；os `D:/autostack/.wt/
os-030/auto-os`。**无前置计划依赖**（029 merge 即基线）。

- **T-01 [lang] 深水调查与定案**
  文件：`message.rs`（多表面/Control 面）、`session.rs`（broker/
  pump/看门兵落点）、`renderer.rs`（boot/层位/投影泵）、`shell.rs`/
  `shell_projection.rs`（产物与载体）、`client_entry.rs`、a2r 生成
  轨（ui_gen/rust.rs 壳产物组织）、mem_guard.rs（watcher 先例）。
  动作：D1–D8 定案。
  产物：`### 5.1 定案记录`（file:line 证据）。
  验证：定案完备；复审通过。
  → 全 AC 前置。新路径：是。
- **T-02 [lang] 协议增量**
  文件：`message.rs`（变体 + 编解码）、`endpoint.rs`/`session.rs`
  （DesktopBus 执行臂）。
  动作：§5.2。
  验证：wire 单测绿。
  → AC-01/04。
- **T-03 [lang] 壳编译客户端**
  文件：`client_entry.rs`（--autodesk-shell）、`shell.rs`（装配）、
  a2r 产物组织（D7 定）。
  动作：§5.3。
  验证：装配单测 + ensure_covered。
  → AC-02。
- **T-04 [lang] 宿主合成与输入**
  文件：`renderer.rs`（boot/层位/投影泵）、`session.rs`（WM 伪窗/
  路由）。
  动作：§5.4。
  验证：层位/命中/投影单测。
  → AC-03。
- **T-05 [lang] 看门兵**
  文件：`session.rs`（respawn 队列/退避/重推）。
  动作：§5.5。
  验证：状态机单测。
  → AC-05。
- **T-06 [lang] 全链整合**
  文件：T-02..T-05 交界面。
  动作：§5.6。
  验证：52 动词/投影帧变全链单测。
  → AC-03/04。
- **T-07 [lang] 双轨与回归**
  文件：`session.rs`（shell_model）、boot 分叉。
  动作：§5.7。
  验证：inproc 轨 shell 测试族全绿。
  → AC-06。
- **T-08 [lang+os] e2e 与真机**
  文件：lang `stage3.rs`（p030）+ os scripts（smoke）。
  动作：§5.8。
  验证：e2e 四腿 + smoke/SendInput 留痕。
  → AC-02/03/04/05/07。
- **T-09 [lang+os] 文档收口**
  文件：§1.11 + a2r + 台账 3c3 + overview + KNOWN-DEBT。
  动作：§5.9。
  验证：AC 逐条留痕；SD-01..04 落笔。
  → AC-07/08。

## 9. 复审记录

- 2026-09-19 /auto-plan:new 起草交接：`stage: new`，PLAN-030 rev 1。
  `outcome: pass`（合同完整：三缺口实锤[DesktopBus broker 丢弃/
  StateSnapshot 零发送方/无 respawn]、027 休眠接缝与 52 动词单源、
  boot 零依赖鸡蛋序、投影泵指纹门、看门兵先例、双轨先例全部
  file:line 在案；任务覆盖全部 AC 与规范增量）；`next: work`
  （**无前置计划依赖**，T-01 可即行）。悬置决策登记 §10（①–⑥），
  均不阻塞 T-01 开工。

## 10. 待澄清事项

- **①（T-01 D1）** 表面模型：双表面单客户端（推荐——多表面基建
  直用）vs 单表面+层序钩子 vs 五面各客户端。
- **②（T-01 D6）** v1 面边界：SHELL_MANIFEST 五件全 outproc（推荐，
  若 launcher in-proc overlay 与 outproc chrome 同屏层序无冲突）
  vs 仅常驻双面 outproc（overlay 维持 in-proc 懒挂载）。
- **③（T-01 D5）** 看门兵形态：ServiceTick 臂内联（推荐——零新
  线程）vs mem_guard 式独立 watcher 线程。
- **④（T-01 D7）** outproc 装载失败降级：回退 inproc（推荐——开发
  友好）vs 无壳桌面（现状容忍语义直延）。
- **⑤（T-01 D8）** SendInput 真机腿观察 channel 形态：壳投影推送侧效
  日志 vs acceptance 通道扩展（029 verb 族延续）。
- **⑥（T-07 后）** 缺省翻转口径：本计划缺省维持 inproc——翻转
  裁定（实机浸润数据门）留待另立，非本计划面。
