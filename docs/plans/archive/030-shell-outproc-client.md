---
plan_id: PLAN-030
status: archived                # drafting → executing → execution_done → reviewed → archived（终态）
feature_name: shell-outproc-client
author: [agent]
created_at: 2026-09-19
updated_at: 2026-09-19
plan_revision: 2
current_step: 10
total_steps: 11

# /auto-plan:review 结束时填写：
supersedes_spec_components: []
new_spec_components:
  - auto-lang/docs/design/autoui/desktop-protocol-v1.md   # §1.11 v1.11 增量（SD-01）
  - auto-lang/docs/design/autoui/desktop-shell-a2r.md     # §10-① 终件收口 + v1 形态注记（SD-02）
  - auto-os/docs/plans/autos-desktop-program.md           # 台账 3c3 行（SD-03）
  - auto-lang/docs/specs/auto-lang/ui/overview.md         # B 形态 provisional 节（SD-04）
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
- **G2 壳 outproc 客户端（rev 2 形态裁定：v1 = 解释面）**：双常驻面经
  shell_source 同源解释装载 + NativeProjector View 全展开渲染（load 期
  ensure_covered 过门）+ `--autodesk-shell` 入口 + ShellProjection child
  侧 lowering + `__desktop_cmd` 读走上行；渲染 = queue 臂 DrawList。
  **a2r 编译面轨（shell-lib 组件库生成模式）移出为独立计划**（用户
  裁定 2026-09-19：run_rust_ui 为 app 工程形——生成器需新组件库模式，
  与缺省翻转计划同族排布；P030-D1 在册）。
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
三件/终件）。**rev 2（2026-09-19 复审后用户裁定）**：v1 壳形态 =
解释面 outproc child 入契约（AC-02/G2 措辞已改），a2r 编译面轨
（shell-lib 生成模式）另立独立计划；T-10 看门兵单测随轮补齐。涉及仓：auto-lang（协议/
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

### 5.1 定案记录（T-01，2026-09-19，四路并行实勘）

**D1 表面模型 = 单连接双表面（Hello 尾追协商）+ 宿主 Stack 层槽 + 双伪窗**
- 实勘纠偏：per-client 多表面仅在容器层成立（`BrokerClient.wid_surface/surfaces/shm`
  均 map，stage3.rs:38-55）；端点状态机单值（`HostEndpoint{app_id,wid,surface}`
  endpoint.rs:422-430，Active 帧一律 `self.surface.expect()` :479/500/525）+
  Active 拒第二 Hello（:447-465）+ 三处 `client.wid == Some(wid)` 单值路由
  （session.rs:3504-3508/3528-3531/3819-3824、broker_surface.rs:494-497）——
  "基建直用"不成立，需扩。**定案**：Hello 尾部追加 `surfaces:
  Vec<SurfaceDecl{role,w,h}>`（`frame_mode` 尾追先例 message.rs:389-394
  `remaining()>0` 判定；role ∈ background|chrome|window，缺省/旧端线 =
  单 window 声明——旧客户端零变化）；Welcome 尾部逐表面
  `{wid,surface,rect}`；`ResolveAndAttach`/`activate()` 扩多表面（endpoint
  Active 帧按消息 wid 路由到对应 surface——FrameReady 本就带 wid）。
- **z 平面落位 = 宿主合成 Stack 层槽而非 z_order 窗**：background 槽 =
  壁纸层上、vwin 循环下（替代 desktop face 槽 renderer.rs:18603-18619）；
  chrome 槽 = vwin 循环上、launcher overlay 下（替代 shell 任务栏槽
  :18891-18909）。宿主壁纸层（:18598-18602，config 驱动）维持宿主侧
  不随壳走。
- **命中 = 双伪窗承载**（WM 域插序不改序——hit_test 逆序纯遍历
  session.rs:923-942，伪窗经 wm_add_win 尾插即置顶命中，D3-1 结论）：
  background 伪窗全屏**垫底**（`wm_add_win` 需增 insert(0) 垫底臂——现
  仅 push 尾 :801）；chrome 伪窗置顶。**chrome 表面 v1 = 任务栏带矩形
  （非全屏）**——hit_test 无透明直通（:938-940 纯矩形，D1-3 实锤），
  全屏 chrome 伪窗会吞掉带外全部点击；带矩形天然规避，overlay 态仍归
  in-proc 层（见 D6）。background 伪窗全屏垫底语义正确：空白点击本就
  归 desktop face 消费。

**D2 投影下行 = typed Push 新变体 ×3（含 cursor），指纹门转宿主侧缓存**
- `ControlMsg` 追加：`ShellProjectionPush{face:u8, payload:bytes}`(tag 12)
  + `ShellClockTick{time,date}`(13) + `ShellCursorMove{x,y}`(14)——
  StateSnapshot 同形模板（message.rs:867/936-940 payload-bytes 模式）。
  cursor 独立变体依据：逐事件字段按设计不入快照（shell_projection.rs:17-19），
  空白菜单坐标锚（renderer.rs:18333-18342）需事件级通道；**节流 = 仅
  blank_menu 开/拖拽中推 + ServiceTick 400ms 兜底**（复用 view_dirty 门
  消费语义 :18314-18317）。
- 载体 encode/decode 零底子新写（全仓无调用；底子 = plain-data 载体族
  shell_projection.rs:13-19 + codec.rs LE 原语 :67-100）：叶面保形
  （bool 载体 bool，"1"/"" lowering 单点留在解释轨）；分面增量 = face
  各自 payload + 宿主侧 per-face 指纹缓存（fp 字段已随载体 :124-125，
  设计预留）；尺寸量级低个位数 KB << 16KiB shm 槽（session.rs:3711-3716），
  走管道 payload 无压力。
- 节拍不变：事件驱动（每 update 排空点 :17365-17369 邻位）+ ServiceTick
  400ms 兜底（:19244）；outproc 臂改推 Push，inproc 臂 byte 级零变化（I1）。
- StateSnapshot 不复用（标量袋 Nil 占位 client_runtime.rs:2002-2006，
  无 face/指纹维度——只借形态）。

**D3 输入路由 = pointer 族补接线 + 键盘 live 直用 + 热键宿主保留**
- **pointer 生产接线缺口实锤**：`broker_pointer_down`（session.rs:3479-3510）
  全仓仅测试调用；029 live 输入只覆盖键盘/字符/IME/滚轮六型
  （:2319-2333）。**定案**：GlobalPress 臂（renderer.rs:18081-18104）/
  release（:18418-18422 邻位）/CursorMoved 臂（:18299-18386）hit_test
  命中 broker wid 时路由 `broker_pointer_down/新增 _release/_moved`
  （press 既有 wm_focus+坐标平移 :3500-3501 语义保留）。chrome 伪窗
  focus 接管键盘 = live 路由按 focused 焦点窗直用（broker_key_event
  :3518-3534 去单值化后自动通）。
- GlobalPress 不看捕获状态（:19335-19339）——chrome 带内"按钮"点击宿主
  无 widget 感知，命中裁决全落 WM 伪窗（本质差异入册）。
- 热键 `desktop_hotkey_subscription`（:8879-8909）纯宿主订阅不动；
  `__mouse_moved/__desktop_cursor` 见 D2 cursor 变体。

**D4 命令上行 = 端点拆臂 + session 收集 + renderer 同拍执行**
- 丢弃点实锤：endpoint.rs:565-570 `let _ = control;` 假透传。**定案**：
  ①端点 DesktopBus 拆出产 `HostAction::DesktopBus{wid,record}`（enum
  加变体 ≈5 行）；②`broker_apply_actions` 加臂（session.rs:3688 match，
  ObserveUp 同级）→ `parse_records`（:1725 单点直用）→ registry_id 归因
  （wid → `wm.wins[wid].registry_id`，壳伪窗 = "shell"/"desktop-face"，
  仿 renderer.rs:10786-10789 notify_source）→ push 进 session 新
  `desktop_bus_inbox: Vec<(source, DesktopCommand)>`；③renderer
  ServiceTick 臂 **pump（:17643）之后** drain inbox → 直调
  `execute_desktop_commands`（:10807 私有 fn，renderer 内可见、task 可
  上缴）——同拍生效，绕开 drain(:17581) 先于 pump 的次序问题。
- acceptance `DesktopInject::Bus` 臂改道（:13545-13559 绑 in-proc
  shell_app，壳出走后失效）：outproc 时 push 同一 inbox（验收与生产
  同臂）；`Key` 臂零改动（route_live_input 已通）。
- 52 动词零词表变化（I4）；client 侧发送 API 新增（D4-1 全仓无发送方）。

**D5 看门兵 = ServiceTick 臂内联 + respawn 队列（短预算分拍）**
- 死亡检出复用 pump EOF（session.rs:3445 `is_eof`，400ms tick 内）；
  respawn 不内联 `launch_app_outproc`（UI 线程同步阻塞 ~25s 实锤
  :3236-3240）——**登记意图 + 到期 Instant**（`switcher_until` 先例
  session.rs:442-444），ServiceTick 到期消费；壳孵化用**短受理预算**
  （3s 受理 + 2s attach，boot 期与 respawn 期同预算）。退避 1s/2s/5s
  封顶、预算 3 次/60s 窗；耗尽 = 降级观测（eprintln + 状态标志）+
  **回退 inproc 装载**（与 D7 降级链同臂）。全量重推 = respawn attach
  成功后宿主指纹缓存强制失效（None）→ 下一拍 sync 推全量。
- mem_guard watcher 线程不采用（桌面宿主不跑 sample_and_guard，
  renderer.rs:24417 仅 devtools 轨；ServiceTick 内联零新线程零锁）。

**D6 v1 面边界 = 仅常驻双面 outproc（shell taskbar + desktop surface），
overlay 三面 + launcher 维持 in-proc**（计划预留的冲突回落分支）
- 实锤冲突：①dashboard 渲染带在 App 窗**之下**（renderer.rs:18627-18791
  + R3 注释 :18620-18623）与置顶 chrome 表面矛盾；②dashboard face 卡
  = 宿主侧 mini 会话活渲染（split_ref_face :18737-18751、hatch_mini_app
  :10404-10417）无投影材料；③launcher 留 in-proc 保 iced 聚焦链
  （:9826-9847）与键盘独占仲裁（:19148-19219）。queue 臂 outproc 是
  宿主合成层非 OS 窗，launcher 盖 taskbar 语义照旧（层序 chrome 槽在
  launcher 层之下）。switcher/notification 材料最齐（SwitcherSnapshot/
  NotesSnapshot + popover wire 命中已证）但 v1 不动——overlay 三面
  outproc 化随缺省翻转计划另立（非目标一致）。
- **outproc 壳渲染面 = shell 面（taskbar 带）+ desktop 面（全屏 icons）**
  两 resident face；SHELL_MANIFEST 五件清单不变（overlay 三面 in-proc
  懒挂载现状零变化）。

**D7 双轨开关与降级 = `shell.apps.shell_model` + boot 序调整 + 回退 inproc**
- 配置位镜像 ProcessModel 三件套（session.rs:2049-2070 先例）：新
  `ShellModel{Inproc 缺省,Outproc}` + `from_storage`（坏值回退 inproc，
  472 同型）+ `load_shell_model()`（renderer.rs:12746 邻位）+ DesktopState
  字段（:384 邻位）；同步改"shell 特权面恒 inproc"注释（:2052-2054）。
  存储键定名 `shell.apps.shell_model`。不进 config.at（v1 最小面，与
  hole/remote_token 同族）。
- **boot 序倒挂实锤**（shell 装载 :14399 在 enable_broker :14623 之前）：
  outproc 臂 spawn 挪到 enable_broker 之后；shell_model 读取前移到壳
  装载点之前（现 process_model 读点 :14584 太晚且在 apps_dir 块内）。
  降级链 = spawn/attach 失败（短预算，见 D5）→ 回退
  `build_shell_component()` in-proc 装载 + stderr 留痕（:14403-14405
  Err 臂降级模板同源），桌面不炸（I5）。
- **壳 exe 产物组织**：a2r 生成 = rust-workspace member 形态
  （rust_ui.rs:462-474 每成员 src/main.rs + Cargo.toml + 共享 target）；
  生成代码入仓先例 = examples/rust-workspace/015-notes。壳客户端 crate
  落 auto-os `shell/rust-workspace/`（pack 源同根，resolve_shell_pack_dir
  解析序 shell.rs:35-61 兄弟臂自命中）；宿主发现 = pack 目录下
  `rust-workspace/<member>/target/{release,debug}/<exe>.exe`（约定扫描
  形态同 outproc_native_exe :2901-2913，非 pac 声明——shell 不经注册表）。
  `--autodesk-shell` 旗标 = client_entry 新 `ClientTarget` 分支（组装
  壳装配而非单 app 投影器）。hash parity 契约不动（.at 源仍单源；
  生成物由 a2r 从源派生）。storage.* 直调 shim 的换线需求随 T-03 实勘
  shell.at 实际 storage 用点定（若 v1 壳面无 storage 读写则零接线）。

**D8 e2e 与真机口径 = 新 arm 复用 t3 真子进程 + smoke/SendInput 双口径**
- `p030_shell_outproc_arm`：stage3.rs 新测试（AUTO_DESKTOP_E2E 门），
  宿主接线照抄 p029 五件套（resolver 注入/process_model=Outproc/
  outproc_spawner/enable_broker，:1688-1714）；child = `spawn_t3_child`
  re-exec + `t3_child_body` 新分支（:510-518 加语料惯例）组装**壳装配
  客户端**（双表面 + apply_projection + DesktopBus 上行）。四腿：双表面
  首帧 DrawList 断言 / 任务栏按钮坐标点击 → DesktopBus 上行 → 窗口
  操作断言 / 投影变更推送 → dock 列表帧变断言 / kill child（outproc_
  children 句柄 :391）→ 看门兵退避重启 → 全量重推恢复断言。断言 =
  状态轮询 + DrawOp 谓词（wait_frame :1716-1762 惯例）；留痕
  `assets/030/`（AUTO_030_ASSETS）。
- 编译壳 exe 真身：ensure_covered 门（029 `shell_pack_native_covered`
  扩展）+ os smoke 腿（scripts/smoke-030-shell-outproc.sh，acceptance
  起桌面 + MCP 截图留痕惯例）。SendInput 真机腿：前台断言
  `foreground_window()`（sendinput.rs:67-70）+ `send()==len` 断言
  （:104-117）+ acceptance key verb 观察（route_live_input → outproc
  壳连接）；协议级断言由 p030 e2e 承载，真机腿 dual 口径留痕（029
  挂账形态）。宿主自身 HWND 取用面缺失（iced window::Id 无 raw
  handle）→ 前台断言按窗口标题 FindWindow（`Auto - {name}`，
  renderer.rs:19029-19039）或宿主未聚焦时跳过留痕（skipped 口径）。

**证据基线**：lang worktree plan-030-dev @ 6934e2839（cargo check 绿，
29.5s/181 存量警告）；四路实勘代理 D1-D8 全域 file:line 在案（本节
引注）。悬置 ①-⑤ 全部落定（⑥ 缺省翻转维持另立，非本计划面）。

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
- **AC-02 壳 outproc 客户端（rev 2）**：双常驻面经 `--autodesk-shell`
  孵化为 outproc 客户端（解释装载 + NativeProjector），双表面 queue 臂
  渲染 + ensure_covered 过门。验证：装配单测 + e2e 腿 1。（编译面轨
  移出另立——rev 2 用户裁定，R-1 闭环。）
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

**work 基线（2026-09-19 /auto-plan:work 进入）**：lang worktree
`D:/autostack/.wt/lang-030/auto-lang` @ `6934e2839`（master 快照，
含 029 基线 56d1596f2 + 后续 docs-only 提交），branch `plan-030-dev`；
依赖 `D:/autostack/.wt/lang-030/auto-down` @ `b1c88def9`（detached
只读）；os worktree `D:/autostack/.wt/os-030/auto-os` @ `3cd6b12`
（main HEAD），branch `plan-030-dev`。auto-lang 主检出自带
`examples/rust-workspace/015-notes` 外方 WIP（`.wt/foreign-wip-rust-
workspace.patch` 同名踪迹在案）——非本计划路径，不纳入。

- **T-01 [lang] 深水调查与定案** [x]
  文件：`message.rs`（多表面/Control 面）、`session.rs`（broker/
  pump/看门兵落点）、`renderer.rs`（boot/层位/投影泵）、`shell.rs`/
  `shell_projection.rs`（产物与载体）、`client_entry.rs`、a2r 生成
  轨（ui_gen/rust.rs 壳产物组织）、mem_guard.rs（watcher 先例）。
  动作：D1–D8 定案。
  产物：`### 5.1 定案记录`（file:line 证据）。
  验证：定案完备；复审通过。
  [✅ 2026-09-19：四路并行实勘（D1+D2/D3+D4/D5+D7/D6+D8）全域
  file:line 在案；关键纠偏——多表面基建"直用"不成立（端点单值+
  三处路由）、hit_test 无透明直通（chrome 定带矩形）、dashboard
  z 带冲突（D6 收常驻双面）、pointer 生产接线缺口、boot 序倒挂、
  a2r 壳 crate 不存在（产物组织定 auto-os shell/rust-workspace）；
  悬置 ①-⑤ 落定；详见 §5.1]
  → 全 AC 前置。新路径：是。
- **T-02 [lang] 协议增量**
  文件：`message.rs`（变体 + 编解码）、`endpoint.rs`/`session.rs`
  （DesktopBus 执行臂）。
  动作：§5.2。
  验证：wire 单测绿。
  [✅ 2026-09-19：lang 104de2655——ControlMsg 三变体（12/13/14，golden
  + round-trip）；Hello/Welcome 尾段多表面协商（空不写尾段=字节级不变）；
  HostEndpoint 多表面路由 + activate_multi；DesktopBus 端点拆臂 +
  session inbox（归因 wid→registry_id）；shell_projection wire 全家族 +
  DesktopSurfaceSnapshot + lowering。desktop_protocol 171/172（唯一红
  = 基线预存 plan624，stash 复核）；endpoint 11/11；broker 15/15]
  → AC-01/04。
- **T-03 [lang] 壳 outproc 客户端** [x]（rev 2 形态裁定转正：v1 = 解释面 outproc child——R-1 闭环，编译面轨另立；证据保留如下）
  文件：`client_entry.rs`（--autodesk-shell）、`shell.rs`（装配）、
  a2r 产物组织（D7 定）。
  动作：§5.3。
  验证：装配单测 + ensure_covered。
  [✅ 2026-09-19：lang 2a3a83369——shell_client.rs（ShellFaces 双常驻面
  + ShellPump 双表面协商/投影消费/命令上行/内联帧，--autodesk-shell
  入口 + spawn_shell_outproc）。**形态调整（复审项）**：v1 交付 = 解释面
  outproc child（shell_source 同源装载——AppProjector 接缝即编译面轨
  替换点）；a2r 编译面轨未接——实锤：run_rust_ui 为 app 工程形
  （rust_ui.rs:2777 resolve_rust_workspace_dir/front_member_name），壳
  pack 组件库形需新生成模式；词汇门测试自注"真实编译门 = T-07
  shell-lib crate cargo build——crate 尚不存在"（ui_gen/rust.rs:8557）。
  029 shell_pack_native_covered 五件 Covered 门维持有效。shell_client
  3/3]
  → AC-02（编译面轨半句随注）。
- **T-04 [lang] 宿主合成与输入** [x]
  文件：`renderer.rs`（boot/层位/投影泵）、`session.rs`（WM 伪窗/
  路由）。
  动作：§5.4。
  验证：层位/命中/投影单测。
  [✅ 2026-09-19：lang 0a3618807——boot 分叉 + broker 后 spawn + 降级
  回退；壳 ResolveAndAttach 分支（垫底/置顶伪窗 + activate_multi +
  归因）；投影泵壳臂（per-face 指纹门 + DesktopSurfaceSnapshot 单源
  重构）；视图双表面槽位 + vwin 跳过 + 伪窗投影排除；pointer
  press/release 生产接线（owns_wid 八处去单值化）。**cursor 推送
  （ShellCursorMove）wire 已册未接泵**（blank 菜单锚——随 T-06/T-08
  补）。desktop_protocol 176/177（唯一红预存）；shell 26/26；broker 15/15]
  → AC-03。
- **T-05 [lang] 看门兵** [ ]（rev 复审 R-2 重开：预算耗尽/降级支路状态机单测缺席——随修订轮补；e2e 腿 4 happy path 证据保留）
  文件：`session.rs`（respawn 队列/退避/重推）。
  动作：§5.5。
  验证：状态机单测。
  [✅ 2026-09-19：lang 0a3618807——schedule_shell_respawn（退避
  1s/2s/5s、预算 3 次/60s 窗）+ ServiceTick 分拍 shell_watchdog_step +
  attach 指纹强制失效（全量重推）+ 耗尽降级观测/一次性回退 inproc。
  **状态机单测未单独落**（respawn 逻辑经 e2e 腿 4 覆盖设计——T-08
  未竟，复审补）]
  → AC-05（e2e 腿 4 随 T-08）。
- **T-06 [lang] 全链整合** [x]
  文件：T-02..T-05 交界面。
  动作：§5.6。
  验证：52 动词/投影帧变全链单测。
  [✅ 2026-09-19：lang c150c3c61——acceptance Bus 臂 outproc 改道同
  inbox；drain_desktop_bus_inbox 抽取（同拍执行 + 归因）；cursor 接泵
  （命中 background 伪窗/拖拽门——D3 节流）；shell_spawner 注入位 +
  AUTO_SHELL_MODEL env。单测：inbox 全链（布局/通知/壁纸族抽样 +
  registry_id 归因 + 幂等）+ 指纹门三态（MockPipe）。52 动词清单
  roundtrip 既有单测在册；逐族执行断言 = 布局/通知/壁纸三族单测 +
  e2e 腿 2 真按钮动词（SummonLauncher 族实跑）]
  → AC-03/04。
- **T-07 [lang] 双轨与回归** [x]
  文件：`session.rs`（shell_model）、boot 分叉。
  动作：§5.7。
  验证：inproc 轨 shell 测试族全绿。
  [✅ 2026-09-19：shell_model 配置位 T-04 落地（env AUTO_SHELL_MODEL >
  storage）；双轨断言单测（from_storage 缺省 inproc + add_win_bottom
  z 序/焦点/MRU）；inproc 零回归门 108/108（ui::session + shell 族 +
  dual_mode + stage3 + ts 对拍）+ 全量 261/262（唯一红 = 基线预存
  imagesurface）——lang dc4ef0677]
  → AC-06。
- **T-08 [lang+os] e2e 与真机** [x]
  文件：lang `stage3.rs`（p030）+ os scripts（smoke）。
  动作：§5.8。
  验证：e2e 四腿 + smoke/SendInput 留痕。
  [✅ 2026-09-19：lang dc4ef0677——p030_shell_outproc_arm 四腿 PASS
  （2.0s：attach+双伪窗 / 双表面首帧 / 投影推送→任务栏帧变[假窗
  registry_id=p030-fake 入 dock] / 真按钮点击→DesktopBus→shell 归因→
  drain 执行 / kill→看门兵检出→1s 退避 respawn→指纹失效→全量重推恢
  复）；帧留痕 assets/030/（chrome 21 行含假窗图标+运行条）。投影器
  定轨 NativeProjector（AppProjector 队列臂 ForLoop/Conditional no-op
  实锤——P030-D5 在册）+ load 期 ensure_covered 过门（AC-02）。os
  32075c8——smoke-030-shell-outproc 实跑 PASS（shell attached
  dual-surface bg=2 chrome=3 真机 boot 分叉实证）；SendInput 真机腿
  dual 口径（协议级 = p030 e2e；真机注入受 P020-D4 约束，KNOWN-DEBT
  核销行在案）]
  → AC-02/03/04/05/07。
- **T-09 [lang+os] 文档收口** [x]
  文件：§1.11 + a2r + 台账 3c3 + overview + KNOWN-DEBT。
  动作：§5.9。
  验证：AC 逐条留痕；SD-01..04 落笔。
  [✅ 2026-09-19：lang 3b403d0cf——§1.11 v1.11 增量 + 顶表行；a2r
  §10-① 前置序列终件 ✅ + v1 交付形态注记；overview provisional 节；
  KNOWN-DEBT P030-D1..D5 + 029 SendInput 挂账核销行。os——台账 3c3 行
  落盘。SD-01..04 全部落笔（ledger 沉淀随 merge）]
  → AC-07/08。
- **T-10 [lang] 看门兵预算耗尽支路单测（rev 2 新增，R-2 闭环）**
  文件：`session.rs` tests（schedule_shell_respawn 状态机）。
  动作：预算窗口内第 3 次登记 → `shell_degraded = true` + respawn 清空
  + 后续登记早退（degraded 门）；renderer 降级回退臂与 T-04 既有接线
  对拍。
  验证：`cargo t -p auto-lang --features ui-iced shell_respawn_budget`。
  [✅ 2026-09-19：lang 4653d890a——shell_respawn_budget_exhaustion_
  degrades 单测（inproc 早退/首登/窗口内第 3 次耗尽 → degraded 置位
  + respawn 清空/degraded 门恒早退不自愈）；ui::session 82/82]
  → AC-05。

## 9. 复审记录

- 2026-09-19 /auto-plan:new 起草交接：`stage: new`，PLAN-030 rev 1。
  `outcome: pass`（合同完整：三缺口实锤[DesktopBus broker 丢弃/
  StateSnapshot 零发送方/无 respawn]、027 休眠接缝与 52 动词单源、
  boot 零依赖鸡蛋序、投影泵指纹门、看门兵先例、双轨先例全部
  file:line 在案；任务覆盖全部 AC 与规范增量）；`next: work`
  （**无前置计划依赖**，T-01 可即行）。悬置决策登记 §10（①–⑥），
  均不阻塞 T-01 开工。
- 2026-09-19 /auto-plan:work 完成记录（T-06..T-09 续跑收口）：`stage:
  work | PLAN-030 | rev 1 | outcome: pass（全部 10 任务完成，待 review）|
  code_commit: lang plan-030-dev 104de2655→2a3a83369→0a3618807→c150c3c61
  →dc4ef0677→3b403d0cf→095702d39；os plan-030-dev 32075c8 |
  task_ids: T-06..T-09 ✅（前五任务见上行交接）| evidence: p030 e2e
  四腿 PASS（2.0s）+ 帧留痕 assets/030/；os smoke 实跑 PASS（shell
  attached dual-surface bg=2 chrome=3）；回归 261/262（唯一红 = 基线
  预存 imagesurface——stash 复核在案）+ inproc gate 108/108；cargo
  check ui-iced --tests 全绿 | blockers: 无。复审关注项：①T-03 v1 形态
  = 解释面 outproc child（a2r 编译面轨另立——P030-D1 实锤依据在案）
  ②AppProjector 队列臂 ForLoop no-op 为既有缺口（P030-D5 新登记）
  ③SendInput 真机腿 dual 口径（P020-D4 约束如实记录）| next: review`
- 2026-09-19 /auto-plan:work 中程交接（会话上下文耗尽，T-06..T-09 未竟）：
  `stage: work | PLAN-030 | rev 1 | outcome: blocked（继续执行） |
  code_commit: lang plan-030-dev 104de2655（T-02）+ 2a3a83369（T-03）+
  0a3618807（T-04/T-05）| task_ids: T-01..T-05 ✅（见各任务证据行）|
  evidence: cargo check ui-iced --tests 全绿；desktop_protocol 176/177
  （唯一红 = 基线预存 covered_elements imagesurface，stash 复核在案）；
  shell 26/26；broker 15/15；endpoint 11/11；shell_client 3/3 |
  blockers: 无外部阻塞；剩余 T-06（52 动词逐族抽样全链单测 + acceptance
  Bus 臂改道[renderer.rs apply_desktop_injects Bus 臂绑 in-proc shell_app，
  壳 outproc 时失效]）、T-08（p030_shell_outproc_arm e2e 四腿 + os
  smoke + SendInput 真机腿）、T-09（§1.11/a2r §10-①/台账 3c3/overview/
  KNOWN-DEBT）| next: 继续 T-06（worktree/分支原样沿用）`

- 2026-09-19 /auto-plan:review（实施会话内复审——结论自工件重建）：
  `stage: review | PLAN-030 | rev 1 | outcome: needs_replan |
  reviewed_commit: lang plan-030-dev 095702d39（104de2655→2a3a83369→
  0a3618807→c150c3c61→dc4ef0677→3b403d0cf→095702d39）；os plan-030-dev
  32075c8 | base: lang 6934e2839 / os 3cd6b12 / dep auto-down b1c88def9 |
  spec_inputs: desktop-protocol-v1.md[+§1.11+顶表 v1.11] /
  desktop-shell-a2r.md[§10-① 终件] / overview.md[provisional] /
  KNOWN-DEBT[P030-D1..D5] / 台账 3c3（os main docs/plans） |
  acceptance: AC-01 pass / AC-02 partial / AC-03 pass / AC-04 pass（R-3
  注）/ AC-05 partial / AC-06 pass / AC-07 pass / AC-08 pass |
  findings: R-1[major|AC-02/T-03] AC-02 子句"五件 a2r 产物"未达——
  v1 交付 = 解释面 outproc child（实锤：run_rust_ui app 工程形
  rust_ui.rs:2777；词汇门自注 shell-lib crate 不存在 ui_gen/rust.rs:
  ~8557；P030-D1 在册）——合同假设失效，须有界修订（或改契约接受
  v1 形态 + 编译面轨另立计划，或补编译面轨任务）；R-2[minor|AC-05/
  T-05] 看门兵预算耗尽/降级支路无验证（AC-05 验证款"状态机单测"缺席；
  e2e 腿 4 只覆盖 happy path）——随修订轮补单测；R-3[info|AC-04]
  逐族抽样 = 3 族单测 + e2e 真动词 1 族——可接受样本，加宽为非阻塞
  改进；R-4[info] frontmatter new_spec_components 仅列协议文档，
  SD-02/03/04 目标随修订/merge 终化 | evidence: 全量 cargo tf --no-
  fail-fast 3640/3643（2 红基线预存[kitchen_sink/mouse_area——基线
  detached 树复现同败]+display_family/ffi_dual_018/019 双侧隔离绿=
  负载偶发，无回归）；终 HEAD 验收件 9/9（p030 e2e 四腿 + inbox 全链
  + 指纹门 + 双轨断言 + 垫底窗 + golden/多表面/拆臂）；desktop_protocol
  176/172（唯一红基线预存 imagesurface）；inproc gate 108/108；os
  smoke 实跑 PASS（shell attached dual-surface bg=2 chrome=3）；帧留痕
  assets/030/ | next: /auto-plan:new 有界修订（R-1 形态裁定 + R-2 补
  测 + R-4 前言终化），T-03/T-05 重开、current_step 7`

- 2026-09-19 /auto-plan:new 有界修订交接（rev 2）：`stage: new |
  PLAN-030 | rev 2 | outcome: pass | 变更：G2/AC-02 形态措辞（v1 =
  解释面 outproc child；编译面轨移出另立——用户裁定）+ T-03 转正
  重勾 + 新增 T-10（R-2 看门兵预算耗尽单测）+ frontmatter spec 清单
  终化（R-4）| task_ids: T-03 重勾/T-10 新增 | next: work（仅 T-10）`

- 2026-09-19 /auto-plan:work rev2 收口：`stage: work | PLAN-030 | rev 2 |
  outcome: pass | code_commit: lang 4653d890a（T-10）| task_ids: T-10 ✅
  （T-03 rev2 转正重勾；T-01..T-09 前轮在案）| evidence: 新单测 PASS +
  ui::session 82/82 | blockers: 无 | next: review（rev 2 复审——重点
  R-1/R-2 闭环核验）`

- 2026-09-19 /auto-plan:review rev 2 复审（实施会话内——结论自工件重建）：
  `stage: review | PLAN-030 | rev 2 | outcome: pass | reviewed_commit:
  lang plan-030-dev 4653d890a（rev1 复审点 095702d39 之后仅 +45 行
  测试[session.rs T-10]）；os plan-030-dev 32075c8 | base: lang
  6934e2839 / os 3cd6b12 / dep auto-down b1c88def9 | spec_inputs:
  同 rev1 复审（docs 提交 3b403d0cf 未变）+ frontmatter 终化清单 |
  acceptance: AC-01..AC-08 全 pass（AC-02 按 rev2 契约核验：--autodesk-
  shell 孵化 ✓/双表面 queue 臂 ✓/ensure_covered ✓/装配单测 ✓/e2e 腿 1
  ✓；AC-05 = 状态机单测[T-10] + e2e 腿 4 双全 ✓）| findings:
  R-1 ✅ 闭环（rev2 用户裁定入契约 + 授权记录在案）；R-2 ✅ 闭环（T-10
  4653d890a）；R-4 ✅ 闭环（frontmatter 四目标全列）；R-3 维持 info 级
  非阻塞改进（动词抽样加宽）| evidence: 终 HEAD 验收批 11/11（p030
  e2e 四腿 + T-10 + inbox/指纹门/双轨/垫底窗/golden/多表面/拆臂/
  wire round-trip）；全量 cargo tf --no-fail-fast 3640/3643——三红与
  rev1 复审同一组且基线复现在案（mouse_area/kitchen_sink 基线预存 +
  display_family 负载偶发双侧隔离绿），此后代码增量仅测试文件 →
  基线证据复用（理由显式）；ui::session 82/82；os smoke 实跑 PASS
  （rev1 在案，os 侧零变更）| next: merge（rev 2 全绿）`

- 2026-09-19 /auto-plan:merge 收据（PLAN-030:r2）：`stage: merge |
  PLAN-030 | rev 2 | outcome: pass | prepared: 复审基线 4653d890a + 冻结
  delta（docs 3b403d0cf）+ 分支 re-sync master 480bddf94（KNOWN-DEBT
  尾部并集解）+ 全量复验（验收批 4/4 + tf 3640/3643 同归因）|
  landed: lang master 6adf57099（merge 提交——二次冲突并集解后 amend，
  marker 清零实证 + master 冒烟 5/5 + ancestry OK）；os main 9a84265
  （FF，ancestry OK）| ledger_refreshed: .autoos/specs.json P030-1×4
  [reports/architecture/designs/tests] + P030-r1（JSON 校验过，5 条目）
  | archived: docs/plans/archive/030-shell-outproc-client.md |
  completion_kind: delivered | cleaned: wt-guard 三树 clean（lang-030/auto-lang + auto-down +
  os-030/auto-os）→ 三 worktree 移除 + 双 plan-030-dev 分支删除
  （lang 480bddf94 / os 9a84265 落地 ancestry 在案）+ 组目录移除实证
  （.wt 下 030 组零残留）`

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
