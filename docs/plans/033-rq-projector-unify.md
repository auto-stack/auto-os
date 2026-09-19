---
plan_id: PLAN-033
status: drafting               # drafting → executing → execution_done → reviewed → archived
feature_name: rq-projector-unify
author: [agent]
created_at: 2026-09-19
updated_at: 2026-09-19
plan_revision: 1

# /auto-plan:review 结束时填写：
supersedes_spec_components: []
new_spec_components:
  - auto-lang/docs/design/autoui/desktop-protocol-v1.md   # §1.14 v1.14 增量（review 定稿）
touched_goals: []

affects:
  - auto-lang/crates/auto-lang/src/ui/desktop_protocol/client_entry.rs   # 改接 seam（Commands 臂）+ run_client 泛型收口
  - auto-lang/crates/auto-lang/src/ui/desktop_protocol/client_runtime.rs # AppProjector 本体退役 + ClientPump 缺省泛参收口
  - auto-lang/crates/auto-lang/src/ui/desktop_protocol/native_projector.rs # 更名 RqProjector + VM 输入/计时补丁消费面
  - auto-lang/crates/auto-lang/src/ui/dynamic.rs                         # on() input_state_map 回写（借 on_with_input_for）+ tick override（依 D2）
  - auto-lang/crates/auto-lang/src/ui/desktop_protocol/remote.rs         # 宿主孪生处置（D4——裁定外第二消费者）
  - auto-lang/crates/auto-lang/src/ui/session.rs                         # 解释 re-exec 臂/process_model 选项拔除
  - auto-lang/crates/auto/src/cmd_autodesk.rs                           # .at 装载臂处置
  - auto-lang/crates/auto-lang/src/ui/desktop_protocol/{rqhost,shell_client,stage3,mod}.rs # 更名/测试迁移
  - auto-lang/docs/design/autoui/desktop-protocol-v1.md                  # §1.14 + 更名 canonical 同步（~11 行）
  - auto-os/docs/plans/autos-desktop-program.md                          # M7 行 + 副线债裁定落定
current_step: 0
total_steps: 8
---

# [PLAN-033] rq-projector-unify

## 0. 变更摘要

**032 之后"RQ 渲染体系完整化"第一件**（用户 2026-09-19 澄清定调：`-q` =
`--render-queue` 对两轨一视同仁——VM 版经 `-q` 走 **NativeProjector**
获得省内存[免每 app iced/wgpu 后端] + native 保真）。三件事：

**①`-q` VM 轨改接 native 臂**——普查实锤最小接缝 = `client_entry.rs`
Commands 臂一处（`AppProjector::new + run_client` → `NativeProjector::new +
ensure_covered + run_client_session`），模板三件齐备（native 臂
client_entry.rs:206-229 / DynamicComponent 实例化 shell_client.rs:91-100 /
保真门仪器 coverage.rs:1172）；030 shell outproc 已是生产先例（解释装载 +
View 全展开）。**②VM 轨三项补齐**（普查新发现的坑）：零参 oninput 的
`input_state_map` 绑定字段回写在 native 臂缺失（003-converter 实证受害，
补丁可借 dynamic.rs:2013-2021——a2r 生成物同构 rust.rs:1270-1314）；
VM 定时器不拍（`tick_msg/tick_interval_ms` 双默认 None，VM timers 靠
iced 订阅驱动）；`__desktop_cmd` 上行缺读走（shell_client.rs:201-222
child 化可借）。**③AppProjector 退役 + 统一更名 `RqProjector`**（用户
授权更名决策，2026-09-19；P020-D1 统一即主线载体、销账）——拔根四件
（解释 re-exec 臂/process_model 选项、解释 pixels 臂、本体+walker、
**remote.rs:218 宿主孪生[裁定外第二生产消费者，显式处置]**）+ 测试
迁移重基线；更名波及 = 代码 123 处 + canonical 文档 ~20 行 + 两侧
specs.json 派生 4 处（归档 6+3 件按 AGENTS 纪律不动）。

## 1. 目标

- **G1 `-q` VM 轨改接**：`run_dynamic_client` Commands 臂改走
  NativeProjector（View 全展开投影 + `ensure_covered` 门）——003-converter
  / 027-file-manager `-q` 渲染正确（build_dynamic_component 全展开口径
  保真门 Covered + e2e 帧断言），调用面（cmd_autodesk/rqhost/main/
  lib.rs）零改动。
- **G2 VM 轨三项补齐**：①`input_state_map` 回写——单参与零参 oninput
  两形态都闭环（003 双向换算实证）；②VM 定时器 native 泵可拍（方案
  依 D2）；③`__desktop_cmd` child 化读走 + `DesktopBus` 上行（普通
  VM app 的 desktop.* 命令在 `-q` 下闭环；shell_client 实现泛化）。
- **G3 AppProjector 退役**：生产消费者清零——①解释 re-exec 臂
  （session.rs:3344 spawn_outproc_child + cmd_autodesk .at 装载臂）+
  `process_model=outproc` 选项拔除（用户裁定：解释态两合法形态 = 内嵌
  直挂 / `-q` 经 native 臂）；②解释 pixels 臂（client_entry.rs:99
  run_independent_child）同废；③本体 + 块流 walker 删除，ClientPump
  缺省泛参（:2095）与 run_client 固签名（:2409）泛化收口；④remote.rs
  宿主孪生显式处置（依 D4）。
- **G4 更名 `RqProjector`**：代码 123 处（Native 68 + App 55）+ canonical
  文档 ~20 行 + 两侧 specs.json 派生 4 处同步；归档零改动（纪律断言）；
  P020-D1 销账（统一兑现 + 更名落定）。
- **G5 验收与收口**：VM 内存对照行（`auto run -r vm` 直挂 vs `-q`——
  免 iced/wgpu 后端的收益实证，app ≤10MB 量化门沿用）；desktop-
  protocol-v1.md **§1.14 v1.14 增量**（VM -q 经 RqProjector + AppProjector
  退役 + 更名入册）；台账 M7 副线债裁定落定；KNOWN-DEBT P020-D1 销账。

**非目标**（明确出界）：

- **rqhost 内存达标与位图过线**（下一件 PLAN-034：release 复测/归因
  优化 + 位图过线通道 + 像素原生族裁定——消费端/协议词汇面，另行立项）。
- **native pixels 臂退役**（`run_independent_native_child`——四条件门
  后，本计划不动 a2r 轨保底）。
- 桌面解释态 inproc 缺省（不动——VM app 在桌面仍内嵌直挂）。
- M7-b（shell 编译面/overlay outproc）与 M7-c（app 批量化）——并行波次。
- native 覆盖扩容（032 六缺项 + M7-c 面为本计划保真前提，不在本计划
  内拓词）。
- 解释态投影器的几何级 parity 精调（退役方向而非精调方向）。

## 2. 架构方案

```text
┌─ 改接臂（client_entry.rs，最小 seam）───────────────────────────────┐
│ run_dynamic_client Commands 臂（:108-124）：                        │
│   AppProjector::new + run_client                                   │
│   → NativeProjector::new(comp, w, h) + ensure_covered +            │
│     run_client_session（模板 = native 臂 :206-229 + shell_client   │
│     :91-100——DynamicComponent 天然满足 Component[Sized+Debug]，    │
│     030 已实证）                                                    │
│ 调用面零改动：cmd_autodesk:121 / rqhost:609 / main:1211 / lib:4846  │
└──────────────────────────────────────────────────────────────────┘
┌─ VM 补齐臂（dynamic.rs + native_projector.rs 消费面）───────────────┐
│ ①input_state_map 回写：on() 拆 Typed 后、handler 执行前补绑定字段  │
│   写回（借 on_with_input_for :2013-2021；INPUT_TEXT 注入链         │
│   :1803-1814 ↔ native 代写 store_input_text :679-683 已同线程接通）│
│ ②VM 定时器：D2 定案（override tick 面 vs 泵侧 VM timer 驱动）      │
│ ③__desktop_cmd 上行：drain_commands child 化泛化（shell_client    │
│   :201-222 可借）→ ControlMsg::DesktopBus                          │
└──────────────────────────────────────────────────────────────────┘
┌─ 退役臂（client_runtime.rs + session.rs + cmd_autodesk.rs）─────────┐
│ 解释 re-exec 臂拔除（spawn_outproc_child:3344 + .at 装载臂         │
│   :72-121 + process_model 选项）→ 解释 pixels 臂同废 →             │
│ AppProjector 本体 + 块流 walker 删 → ClientPump/run_client 泛型    │
│ 收口 → remote.rs:218 孪生处置（D4）→ 测试迁移重基线（几何断言分层）│
└──────────────────────────────────────────────────────────────────┘
┌─ 更名臂 ────────────────────────────────────────────────────────────┐
│ NativeProjector→RqProjector（68 处）；AppProjector 名随本体消亡     │
│ canonical 同步：desktop-protocol-v1.md ~11 行 / desktop-shell-a2r  │
│ 2 行 / KNOWN-DEBT 3 行 / specs overview+plans 3 行 / os 台账 4 行 / │
│ 两侧 specs.json 4 处；归档 6+3 件不动（AGENTS:148-152）             │
└──────────────────────────────────────────────────────────────────┘
```

**不变式**：

- **I1 追加式协议**：零 wire 变体——改接/退役/更名全部是客户端臂内部
  演进，`PROTOCOL_VERSION` 仍 1。
- **I2 零回归**：a2r 轨（run_native_client/rust -q/桌面 exe 臂）零改动；
  桌面解释态 inproc 缺省零改动；shell outproc（030）零牵连；rqhost
  机器零改动（`vm_typing_loop_over_pipe` :1689-1738 天然成为新投影器
  断言载体）。
- **I3 退役非静默**：process_model=outproc 拔除留配置面迁移注记（storage
  键读到的处置——报错 or 忽略留痕，D6 定）；remote 孪生处置显式留痕。
- **I4 测试纪律**：迁移 = 重基线（AuraNode 块流 vs View walk 两布局
  引擎几何不同——文本断言平移、几何断言按 RqProjector 重录，逐条归因）。

**关键风险**：`input_state_map` 回写位选择错会双写冲突（on() 与
on_with_input_for 两路径——D1 定案须钉"单写点"）；VM timers 驱动方案
影响泵节拍（D2）；测试重基线工作量（client_runtime climb 族 ~11 处 +
stage3 :271/:567 + session P508 :6625——分层处置）；**auto-lang 主检出
`.autoos/specs.json` UU 冲突未清**（本计划开工的环境前置——属主会话
或随 032 merge 收口）。

## 3. 技术栈

Rust / iced 0.14；既有 NativeProjector/ClientPump/client_entry 机器
（020/025/026/029 爬坡面即新保真面）；030 shell_client 装配先例；
coverage 保真门仪器（native_flip_coverage_data_row 机制复用为 VM -q
保真门——build_dynamic_component 全展开口径，032 T-07
native_gate_runtime_views_of_six 钉法可循）；验收载体 = 003-converter
（零参 oninput 实证）/ 027-file-manager（用户原始痛点）/ 001-helloworld
（031 既有断言迁基线）。

## 4. 需求分析与背景调查

**授权记录**：用户 2026-09-19 会话定调"032 完成之后完善 RQHost +
Native 臂"，并授权更名决策（"看看你要不要改名 NativeProjector 成
RqProjector"——裁定：**改**，随本计划统一落定，P020-D1 注记在册）；
前轮澄清（`-q` VM 轨 = 解释组件物化 View 直喂 NativeProjector）为
本计划 G1 的直接授权。**本轮仅规划，未授权实施**。涉及仓：auto-lang
（投影器/装载/文档）+ auto-os（台账）。无预算/自动续跑约束声明。

**前置依赖**：①PLAN-032 merge（worktree plan-032-dev @47b86d730 已
execution_done 收尾中——T-06 缺省翻转已执行，canonical §1.13 随其
落地）；②auto-lang 主检出 `.autoos/specs.json` UU 合并冲突清理
（环境前置，非本计划工作面）。

**现状事实**（已核，2026-09-19 master b69c7344c 含 031，探索代理全量
普查；032 事实取 worktree plan-032-dev@47b86d730）：

- **改接 seam 与模板**：`run_dynamic_client`（client_entry.rs:89-126）
  Commands 臂 :108-124 = `AppProjector::new(:116) + run_client(:118)`；
  native 臂模板 :206-229（`NativeProjector::new + ensure_covered +
  run_client_session`）；shell_client.rs:91-100 = DynamicComponent 实例化
  先例（`NativeProjector<DynamicComponent>`，030 生产在役）。调用面
  零改动（cmd_autodesk.rs:121 / rqhost.rs:609 / main.rs:1211 /
  lib.rs:4846-4849）。
- **DynamicComponent ↔ Component**：`impl Component`（dynamic.rs:1782-
  1853；`type Msg = DynamicMessage`，on :1795/view :1838）；trait 约束
  仅 Sized+Debug（component.rs:30）——天然满足，无适配器。
- **输入编辑三轨对照（G2 深水）**：①AppProjector 臂 = `type_into`
  （client_runtime.rs:454-510，读绑定字段→编辑→写回→零参派发）；
  ②VM 自开窗轨 = INPUT_TEXT（renderer.rs:4283-4288）+
  `on_with_input_for`（dynamic.rs:1977——`input_state_map` 绑定字段
  回写 :2013-2021 + 文本实参 :2047-2070）；③native 臂 = 编辑 buffer +
  `dispatch_input_edit`（native_projector.rs:679-683，
  store_input_text+on）。**缺口**：`DynamicComponent::on()`
  （:1795-1822）只有 INPUT_TEXT 空实参注入（:1803-1814），**无
  input_state_map 回写**——零参内联闭包 oninput（003-converter
  app.at:38/:49）在 native 臂下绑定字段不落。单参 handler 形态兼容。
- **VM 定时器缺口**：native `poll_tick` 走 trait
  `tick_interval_ms()/tick_msg()`（native_projector.rs:571-586），
  DynamicComponent 双默认 None（未 override）→ VM timers
  （fire_timer dynamic.rs:1146 族）不拍。
- **`__desktop_cmd` 上行缺口**：现 AppProjector 泵无读走
  （client_runtime.rs 零命中）——shell_client.rs:201-222 的
  drain_commands + DesktopBus 上行是 shell 特有实现，可借泛化。
- **AppProjector 消费面**：生产 = client_entry:116（两调用方）+
  **remote.rs:206-236 宿主孪生**（remote_twin_hits：resolver 同源编译
  + AppProjector 同布局断言——裁定清单外的第二生产消费者，必须显式
  处置）；桌面解释 outproc 链 = session.rs:3314 launch_app_outproc →
  spawn 三分流 :3331-3346（:3344 spawn_outproc_child re-exec）→
  main.rs:961 run_if_client_entry → cmd_autodesk.rs:37-122（:82 装载
  .at → :121 run_dynamic_client）；ProcessModel::Outproc 配置位
  session.rs:2100-2138；解释 pixels 臂 client_entry.rs:99。
- **测试依赖**：dual_mode.rs 零依赖（自有 FrameSource）；stage3.rs
  10 处全 `#[cfg(test)]`（:271/:567 + 029 已有 native 对照臂
  run_native_t3_child）；client_runtime 内部 ~11 处（climb_001-005/
  project/click/管道全循环——两布局引擎几何不同，断言需重基线非直迁）；
  session.rs:6605-6627（P508 outproc 测试体）；rqhost.rs 测试
  （vm_typing_loop_over_pipe :1689-1738 = 改接后天然断言载体）。
- **更名波及**：代码 NativeProjector 68 处 / AppProjector 55 处；
  canonical = desktop-protocol-v1.md 11 行（:19/:22/:50/:63/:73/:75/
  :178/:530/:555-556/:635）+ desktop-shell-a2r.md 2 行 + KNOWN-DEBT
  3 行（:2272 P020-D1/:2379/:2383）+ specs ui/overview.md 2 行 +
  plans.md 1 行 + os 台账 4 行（:103/:109/:121/:127）+ 两侧 specs.json
  4 处；归档 6+3 件不动（AGENTS.md:148-152）。RqProjector 代码零
  命中（名空闲）。
- **保真门仪器**：coverage.rs:1172-1256（examples 全量 → build →
  scan_native_view × judge）——机制可复用为 VM -q 保真门（口径走
  build_dynamic_component 全展开，032 T-07 钉法先例）；shell 五件
  同构 shell_pack_native_covered :1290-1373。
- **032 状态**：worktree lang-032 @47b86d730 execution_done（T-06
  翻转已执行——Covered 臂 Pixels→Commands；T-07 六例 e2e；T-08 文档
  §1.13）——本计划前置就绪待 merge。

**specs 现状**：协议权威 v1.12 现行（031 §1.12；032 落 §1.13 在途）；
P020-D1 更名注记 + 主线载体澄清（2026-09-19，KNOWN-DEBT 工作区在案
——随主检出冲突收尾落账）；os 台账 M7 副线债 :109 裁定原文。

## 5. 详细设计

### 5.1 T-01 深水调查定案（决策产物）

- **D1 input_state_map 回写位（G2 核心）**：候选 A = `on()` 内补
  （拆 Typed 后、handler 前——与 INPUT_TEXT 注入同位；**倾向**：单
  写点在组件侧，投影器/pixel 等所有消费端免费受益）/ B = native
  dispatch_input_edit 前置回写（投影器侧——但 on() 语义分裂两处）。
  以 003 双 input 双向换算（零参闭包读他字段）+ 单参 handler 两样本
  定案；**双写冲突钉死**（on 与 on_with_input_for 单写点声明）。
- **D2 VM 定时器驱动**：候选 A = `impl Component` 补 override
  （tick_interval_ms/tick_msg 自 VM 声明提取——多 timer 语义如何取
  "主"间隔，实勘 fire_timer 家族）/ B = NativeProjector 侧 VM timer
  泵（view_mount 帧簿记驱动——dynamic.rs mount 面实勘）。以带 timer
  样本（clock/todo 类）定案。
- **D3 `__desktop_cmd` 上行泛化**：shell 专属 child 化推广到普通
  VM -q client（泵内双读走点 vs 帧后单点——shell_client :404-448
  先例）；上行语义对齐宿主全窗联合排空（renderer 侧既有）。
- **D4 remote.rs 宿主孪生处置**：候选 A = 迁 RqProjector 重基线
  （孪生命中表继续服务远程镜像）/ B = 孪生面随 508 远程线另裁
  （本计划留痕不阻塞）。以 remote_twin_hits 的消费语义（WS 镜像端
  hitTest）定。
- **D5 测试重基线口径**：文本断言平移 / 几何断言重录分层清单；
  climb_001-005 处置（迁 RqProjector golden 重录 vs 删——历史爬坡
  证据价值 vs 维护成本）。
- **D6 process_model 拔除面**：storage 键 `shell.apps.process_model`
  读到 outproc 的处置（报错提示迁移 vs 忽略留痕）；`AUTO_SHELL_MODEL`
  （030 shell 专属）不混淆。

定案记录追加 `### 5.1 定案记录`，作为 T-02..T-07 依据。

### 5.2 改接与 VM 补齐（T-02/T-03）

- **T-02 改接**：client_entry Commands 臂换 NativeProjector 装配 +
  ensure_covered；001/003 冒烟（保真门 Covered 断言）；
  `vm_typing_loop_over_pipe` 复跑（改接后天然断言）。
- **T-03 VM 三补**：①on() input_state_map 回写（D1 位 + 单写点断言
  + 003 换算闭环）；②定时器（D2 方案 + 样本闭环）；③__desktop_cmd
  上行（D3 + 命令闭环用例）。

### 5.3 退役与孪生（T-04/T-05）

- **T-04 AppProjector 拔根**：解释 re-exec 臂 + process_model 选项 +
  解释 pixels 臂 + 本体/walker + ClientPump/run_client 泛型收口
  （D6 处置面）。
- **T-05 remote 孪生 + 测试迁移**：D4 处置落地；测试重基线（D5 清单
  ——stage3/client_runtime climb 族/session P508/rqhost）。

### 5.4 更名与收口（T-06/T-07/T-08）

- **T-06 更名**：代码 123 处 + canonical ~20 行 + 两侧 specs.json
  4 处；归档零改动断言（grep 归档区零新增）。
- **T-07 e2e 与度量**：`p033_rq_unify_arm`——003/file-manager `-q`
  全链（帧断言 + 输入换算 + 截图 assets/033/）+ VM 内存对照行
  （直挂 vs -q：app 进程 Private 对照，≤10MB 门沿用）+ 桌面 inproc
  解释回归（I2）。
- **T-08 文档台账**：§1.14 增量（VM -q 经 RqProjector/AppProjector
  退役/更名/process_model 拔除）；P020-D1 销账；os 台账 M7 副线债
  裁定落定 + M7 行更新；KNOWN-DEBT（P033 新债随注）。

### 规范增量

| delta_id | add/modify/retire | docs/specs/... target | before/after rule | rationale | acceptance IDs |
|---|---|---|---|---|---|
| SD-01 | add | auto-lang/docs/design/autoui/desktop-protocol-v1.md（§1.14 v1.14 增量 + 全文更名同步） | before：v1.13 双投影器并存（NativeProjector + AppProjector），`-q` VM 轨走 AppProjector 降级投影；after：**单投影器 RqProjector**（VM 经物化 View 入投影[a2r/解释同律]、input_state_map 回写/VM timer/desktop_cmd 上行补齐入册）、AppProjector 退役 + 解释 outproc 选项（process_model）拔除、更名 canonical 全同步（~11 行）——PROTOCOL_VERSION 仍 1 | 协议权威收录统一形态与用户裁定（2026-09-19 澄清） | AC-01/02/03/04 |
| SD-02 | modify | auto-os/docs/plans/autos-desktop-program.md | before：M7 副线债 :109 裁定原文（-q VM 改接 + AppProjector 退役 + 归一 RqProjector）；after：落定为交付行（033）+ P020-D1 销账指针 + M7 波次注记 | 桌面程序台账 | AC-05 |
| SD-03 | modify | auto-lang/docs/plans/KNOWN-DEBT-AND-RISKS.md | before：P020-D1 在册（双投影器统一 + 更名意向 + 主线载体注记）；after：销账（统一兑现）+ P033 新债随注（测试重基线残留/remote 孪生处置结果） | 债账收口 | AC-05 |

零 spec 影响的变更不存在（投影器统一/退役/更名为协议级知识）；ledger
随 merge 沉淀。

## 6. 测试设计

- **单测（client_entry/dynamic）**：改接装配（Commands 臂 →
  RqProjector 泵）；`input_state_map` 回写单写点（003 双向换算两形态
  ——单参/零参闭包）；VM timer 拍发（D2 方案样本）；__desktop_cmd
  上行（drain+DesktopBus 编码）。
- **单测（退役面）**：解释 re-exec 臂拔除断言（spawn 三分流二分）；
  process_model 键处置（D6）；ClientPump 泛型收口编译面 +
  run_client 泛化签名。
- **保真门**：VM -q 保真仪器（build_dynamic_component 全展开 ×
  scan × judge——003/file-manager/001 全 Covered 断言）。
- **测试迁移重基线**（D5 清单）：stage3 :271/:567（queue 档腿迁
  native 形）、client_runtime climb 族处置、session P508 :6625、
  rqhost vm_typing 复跑——逐条归因留痕。
- **e2e**：`p033_rq_unify_arm`（§5.4 T-07）+ 截图 assets/033/ +
  内存对照行。
- **回归门**：desktop_protocol（scoped + rqhost 13）/session/
  stage3/dual_mode + `cargo t -p auto-man rust_ui` + auto-os 桌面
  smoke（解释 inproc 零回归[I2] + 桌面 exe 臂零回归）。

## 7. 验收标准

- **AC-01 `-q` VM 改接全链**：003-converter/027-file-manager/
  001-helloworld 经 `-q` 以 RqProjector 渲染——保真门全展开口径
  Covered + e2e 帧断言 + 截图留痕；file-manager 的 popover/图标/图像
  面真渲（对照 031 AppProjector 降级形态的修复实证）。验证：保真门
  + e2e + assets/033/。
- **AC-02 VM 输入/计时/命令三补**：003 键入双向换算闭环（零参闭包
  回写实证）；timer 样本在 `-q` 下拍发；desktop.* 命令经 VM `-q`
  client 上行执行闭环。验证：单测 + 集成。
- **AC-03 VM 内存收益**：`auto run -r vm` 直挂 vs `-q` 的 app 进程
  Private 对照数据行（-q 免 iced/wgpu 后端收益；≤10MB 门沿用）。
  验证：度量行 + 进程清单留痕。
- **AC-04 AppProjector 退役**：生产消费者清零（含 remote.rs 孪生按
  D4 显式处置）；解释 outproc 选项/pixels 臂拔除；归档区零改动；
  全量编译零 AppProjector 残留（除归档/历史注记）。验证：grep 清单
  + 编译门。
- **AC-05 更名与文档**：RqProjector 全替换（代码+canonical+specs.json
  派生）；P020-D1 销账；§1.14 + 台账落定互链。验证：grep + 文档交叉
  引用。
- **AC-06 零回归**：a2r 轨/桌面 inproc 解释/shell outproc/rqhost 机器
  全绿；§6 回归门通过（在册既有红除外）。

## 8. 执行步骤

**前置**：①PLAN-032 merge（execution_done 收尾中）；②auto-lang 主检出
specs.json UU 冲突清理。依赖序：T-01 → T-02 → T-03 → {T-04, T-05
并行} → T-06 → T-07 → T-08。lang worktree
`D:/autostack/.wt/lang-033/auto-lang`；os `D:/autostack/.wt/os-033/
auto-os`。

- **T-01 [lang] 深水调查与定案**
  文件：`client_entry.rs`、`dynamic.rs`（on/on_with_input_for/mount/
  timer 面）、`native_projector.rs`（poll_tick/dispatch 面）、
  `remote.rs`（孪生）、`session.rs`（spawn 三分流/process_model）、
  shell_client.rs（drain 先例）+ §5.1（写面）。
  动作：D1–D6 定案（D1 回写位为核心）。
  产物：`### 5.1 定案记录`（file:line 证据）。
  验证：定案完备；复审通过。
  → 全 AC 前置。新路径：定案产物。
- **T-02 [lang] 改接**
  文件：`client_entry.rs`（Commands 臂 seam）。
  动作：§5.2 T-02；调用面零改动验证。
  验证：001/003 冒烟 + vm_typing 复跑 + 保真门 Covered。
  → AC-01。
- **T-03 [lang] VM 三补**
  文件：`dynamic.rs`（on 回写/timer override 依 D2）、
  `native_projector.rs`（timer 消费/desktop_cmd 读走依 D3）。
  动作：§5.2 T-03。
  验证：003 换算/timer/命令三闭环单测绿。
  → AC-02。
- **T-04 [lang] AppProjector 拔根**
  文件：`client_runtime.rs`（本体/泛型收口）、`session.rs`（re-exec
  臂/process_model）、`cmd_autodesk.rs`（装载臂）、`client_entry.rs`
  （解释 pixels 臂）。
  动作：§5.3 T-04；D6 处置面。
  验证：编译门 + 拔除 grep 清单 + spawn 二分断言。
  → AC-04。
- **T-05 [lang] remote 孪生 + 测试迁移**
  文件：`remote.rs`（D4 处置）、`stage3.rs`/`client_runtime.rs`/
  `session.rs`/`rqhost.rs`（D5 重基线清单）。
  动作：§5.3 T-05；逐条归因留痕。
  验证：迁移后套件绿（重基线清单核）。
  → AC-04/06。
- **T-06 [lang] 更名 RqProjector**
  文件：代码 123 处 + canonical 文档 + 两侧 specs.json。
  动作：§5.4 T-06；归档零改动断言。
  验证：grep 清单（RqProjector 全替换/归档区零新增）。
  → AC-05。
- **T-07 [lang+os] e2e 与度量**
  文件：lang `stage3.rs`（p033_rq_unify_arm）+ assets/033/；os smoke
  如需。
  动作：AC-01..03 逐条留痕 + 内存对照行。
  → AC-01/02/03。
- **T-08 [lang+os] 文档与台账收口**
  文件：lang `desktop-protocol-v1.md`（§1.14 + 更名同步）+ KNOWN-DEBT
  （P020-D1 销账 + 新债）；os 台账落定 + 互链。
  动作：SD-01..03 落笔。
  → AC-05。

## 9. 复审记录

- 2026-09-19 /auto-plan:new 起草交接：`stage: new`，PLAN-033 rev 1。
  `outcome: pass`（合同完整：改接 seam/三模板/输入三轨对照与回写缺口/
  timer 与 desktop_cmd 缺口/AppProjector 消费面含 remote 孪生/更名
  波及 123+20+4/测试依赖与重基线口径全部 file:line 在案）；`next:
  work`——**前置 = 032 merge + 主检出 specs.json 冲突清理**。悬置决策
  §10（①–④），D1 回写位为核心，均不阻塞 T-01。

## 10. 待澄清事项

- **①（T-01 D1）** input_state_map 回写位：on() 内补（推荐——单写
  点）vs 投影器前置；双写冲突钉死。
- **②（T-01 D2）** VM 定时器：Component override 面 vs 泵侧驱动
  （多 timer 取主间隔语义实勘）。
- **③（T-01 D4）** remote.rs 宿主孪生：迁 RqProjector 重基线（推荐）
  vs 随远程线另裁留痕。
- **④（T-01 D5）** climb_001-005 历史爬坡测试处置：迁 RqProjector
  重录 golden vs 删（历史证据价值 vs 维护成本）。
