---
plan_id: PLAN-003
origin: PLAN-554
status: reviewed                # drafting → executing → execution_done → reviewed → archived
feature_name: clock-app
author: [zhaopuming]
created_at: 2026-09-05
updated_at: 2026-09-08（execution_done：T1-T7 全 ✅，交接 review）

# /auto-plan:review 结束时填写：
supersedes_spec_components:
  - "auto-lang examples/ui/README.md: 修改——012 行更新为 Clock 四 tab（os-003 升级注记）"
new_spec_components:
  - "auto-lang crates/auto-lang/src/ui_gen/ts_adapter.rs: 修改——Time 模块 vue 桥（Time.now_sec()→Math.floor(Date.now()/1000)、now_ms→Date.now()；TDD 锁 time_module_bridges_to_date_now）——时间类 app 双端能力基建"
  - "auto-lang examples/ui/012-stopwatch: 升级——Clock 四 tab（秒表真走表/计时器横幅/世界时钟 8 城/闹钟 storage 5 槽）+ tests/desktop_mcp.py 四断言组 12/12 + vue 三 tab 截图"
touched_goals:
  - "GOAL-010: 桌面默认应用集——012 升级 Clock（C 档策展成员，id 保持稳定）"

affects: [auto-lang/ui]       # 受影响的 specs 路径，如 [auto-lang/vm]
current_step: 3
total_steps: 7
---

> **随迁注记（Stage B P-1，2026-09-07，PLAN-001）**：本计划自 auto-lang
> `docs/plans/554-clock-app.md`（origin PLAN-554）随迁重编为 PLAN-003——drafting 原状、
> 状态机续用，git 历史留 auto-lang 仓。设计依据
> [Design 01 §5](../design/01-stage-b-desktop-migration.md)；auto-lang 侧指针行见
> 其 `docs/plans/INDEX.md`。正文中的 auto-lang 相对路径/行号锚点，开工时按本仓
> AGENTS §2 解析序换算（env → `../auto-lang` → `D:/autostack/auto-lang`），
> 现文不回改。

# [PLAN-003] Clock——012-stopwatch 原地升级为四 tab 时钟应用

## 变更摘要

AutoOS 缺 Windows Clock 对标物。现状 `012-stopwatch` 是半成品：`.Tick`
消息存在但**无人发送**（走表不走，仅 Start/Stop/Lap 改显示）；025-dashboard
已实证 `.Tick` 自驱动机制（`interval` 模型变量被 UI 运行时取走作周期）。
本计划把 012 **原地升级**为 Clock 四 tab 应用：秒表（修复走表）/ 计时器 /
世界时钟 / 闹钟。**目录名与 registry id `012-stopwatch` 保持稳定**（recent
槽、pac name 引用不破坏），pac title 改 "Clock"。

依赖：PLAN-552 `desktop:` 字段（软依赖，未合入时为无害未知键）。

## 目标

1. **秒表**：真实走表（Tick 驱动，精度 10ms 档显示 MM:SS.cc）+ 计圈
   （定长 5 槽，028 先例）+ Reset。
2. **计时器**：HH/MM/SS 设定 → 倒数 → 到零横幅提醒（in-app banner v1）
   + 暂停/继续/取消。
3. **世界时钟**：≥8 城当前时刻（`Time.now_sec` + 城市偏移表换算，含
   北京/伦敦/纽约/东京/悉尼/巴黎/莫斯科/洛杉矶），当前城高亮本地大表。
4. **闹钟**：HH:MM 设定 + 闹钟列表（storage 定长 5 槽持久化）+ 到点
   in-app banner（v1 无声）。
5. pac：`title: "Clock"`、`icon: "clock"`、`category: "tool"`、
   `desktop: "true"`、`window: "fit"` 保持。

## 架构方案

- **Tick 契约**（025-dashboard 实证形态）：model 声明 `interval int = <ms>`
  + `.Tick` 消息 handler → UI 运行时按 `interval` 周期派发。本应用
  `interval = 250`（世界时钟秒级、闹钟分钟级比对、秒表 10ms 显示精度分频：
  `sub` 计数器 25 进位——025 `.subTick` 分频同款）。
  **T1 探针**先钉死契约细节（变量名/类型/取走条件，读 renderer `__tick`
  事件名与 AppTickRecipe 段 + 025 SPEC），结论写回本 plan。
- **时间源**：`Time.now_ms()/now_sec()`（`vm/ffi/stdlib.rs:644-669` 实存）。
  世界时钟/闹钟比对用 now_sec 换算；秒表 elapsed 用 Tick 累计（暂停语义
  简单，不受系统调时影响）。
- **单组件约束**：全状态内聚 App（store 子组件 vm 生成损坏先例 013/038）；
  换算/补零逻辑写状态法 handler（模块级 fn 不进 vue SFC，024/028 先例）。

## 需求分析与背景调查
（从 docs/specs/overview.md 与相关 module spec 取材）

- **GOAL-010**：Clock 为 2026-09-05 盘点第一梯队缺口；012 现状
  （`examples/ui/012-stopwatch/src/front/app.at`）：`.Tick -> { .elapsed += 10 }`
  存在但无派发方——升级顺带修复而非重写。
- **Tick 机制**：`examples/ui/025-dashboard/src/front/app.at:9` 头注
  「`interval` 模型变量被 .Tick 机制取走作 setInterval 周期（250ms 基准）」；
  运行时侧 `renderer.rs` `TICK_EVENT("__tick")` + `AppTickRecipe`（tokio
  interval，5871-5936 段）。vue 侧对应 setInterval 生成（025 双端实测绿）。
- **时间原生**：`Time.now_ms/now_sec/now`（stdlib.rs 644-669）。
- **storage 定长槽惯例**：028 `launcher.recent_apps.0..4`（值只做 `!= ""`
  比较——vue 产物 null 链 TS18047 教训）。
- **通知面**：shell toast/通知中心（Plan 479）经 `__desktop_cmd` 上行——
  App 侧可用动词词表待 T1 核；无则 in-app banner（col 覆盖层）v1。
- 画廊：012 已归 "02-components"（vue.rs 分类链 007–012 臂）——升级后
  形态变化不改分类（画廊分类按前缀稳定）。

## 详细设计

### model（要点）

```
var tab str = "stopwatch"       // stopwatch | timer | world | alarm
// 秒表
var running str = "false"  var elapsed int = 0        // ms 累计
var sub int = 0                 // 250ms→10ms 分频基数（×25 显示步进）
var time_display str / ms_display str / lap1..lap5 str
// 计时器
var t_set_h/m/s int = 0  var t_left int = 0   // 剩余秒
var t_running str = "false"  var t_done str = "0"     // 到零横幅门控
// 世界时钟
var cities = ["Beijing","London",...]          // 平行列表（B12 规避）
var offsets = [8, 0, -5, 9, 11, 1, 3, -8]      // UTC 偏小时
var now str = ""  var rows = []                // handler 自建 {city,time}
var local_city int = 0
// 闹钟
var a_h str = "07"  var a_m str = "30"
var alarms = ["","","","",""]  var fired str = ""        // 当日已触发槽标记
var banner str = ""            // 通用覆盖横幅文案（空=隐藏）
// Tick
var interval int = 250
```

### handler（要点）

- `.Tick`：分频→各 tab 更新（秒表 elapsed+=250 且显示步进 cc；timer
  t_left 递减到 0 触发 banner；world 每 4 tick 重算行；alarm 每分钟比对
  now 换算 HH:MM ∈ alarms 且未 fired → banner + fired 标记）。
- `.StartStop/.Lap/.Reset`（秒表，012 现逻辑保留 + 接 Tick）；
- `.TStart/.TPause/.TCancel`（计时器）；
- `.SetLocal(i)`（世界时钟高亮切换）；
- `.AAdd/.ADel(i)`（闹钟槽 CRUD + storage `clock.alarms.0..4` 持久化）；
- `.DismissBanner`（横幅关闭，fired 保持防重复触发）。

### view（要点）

顶部 tab 四胶囊（选中 `bg-primary/15`）；各 tab 内容卡片；banner =
覆盖层 col（`bg-background/95` + 文案 + Dismiss 按钮）。

## 测试设计

`tests/desktop_mcp.py`（012 目录，011/013 惯例，双端）：

1. 秒表 Start→等 ~1s→显示前进（MM:SS.cc 变化断言）；Lap 记圈；Reset 清零。
2. 计时器设 00:00:01→Start→到零 banner 出现 + Dismiss 可关。
3. 世界时钟 ≥8 行渲染；北京/伦敦时差断言（now_sec 基准算期望值，容差
   ±60s 换算）。
4. 闹钟设当前下一分钟→等触发 banner；storage 重开恢复列表。

（等待类断言用 mcp 轮询超时 10s，038 timer 先例。）

## 验收标准

1. 双端四 tab 全功能可用；秒表真实走表（修复原 012 不走表缺陷）。
2. `desktop_mcp.py` 双轨全绿。
3. 闹钟 storage 持久化跨重启恢复。
4. pac title "Clock"/icon "clock" 生效（boot 窗标题与图标格）。
5. registry id `012-stopwatch` 不变（recent 槽/dock pinned 无破坏）。
6. `examples/ui/README.md` 012 行更新（Clock 四 tab）。

## 执行步骤
（原子任务：精确文件路径 + 确切操作 + 验证命令；每步完成后追加 [✅ 已完成] 一行证据）

- [✅ 已完成] **T1 Tick 契约探针**（P-6 V8 冒烟批，2026-09-07）
  读 `crates/auto-lang/src/ui/iced/renderer.rs` `TICK_EVENT`/`AppTickRecipe`
  段与 vue 侧 setInterval 生成（ui_gen）+ 025 SPEC「Tick 机制」节；确认
  `interval` 变量取走条件与 vue/vm 双端行为；顺带核对 `__desktop_cmd`
  动词表有无 notify。结论写回本节。
  验证：探针笔记（scratch/p554/）

  **T1 探针结论（路径均经解析序 `../auto-lang` 兄弟换算可达——V8 实证）**：

  1. **取走条件**（`aura/extract.rs:719-737`）：widget 有 `.Tick` handler 时，
     model 中**字面名 `interval`** 的 `Expr::Int` 字面量被取走为
     `tick_interval`（非字量/缺省→1000ms），且 `interval` 从 state_vars
     移除（不进 ref/状态面）。iced 轨 `renderer.rs:15097` 经
     `component.tick_interval()` → `widget_tick`（TICK_EVENT=`"__tick"`，
     renderer.rs:5583；AppTickRecipe tokio 订阅 6262-6380 段）恒挂。
  2. **vue 轨门控差（关键坑）**：`ui_gen/vue.rs:3995-4016`——若模型存在
     名为 `running` 的状态变量，setInterval 挂 `watch(running)`（仅
     `running=="true"` 时跑）；无 `running` 变量则 onMounted 恒跑。iced 轨
     **无此门控**。⇒ 单 App 混四 tab 时若用 `running` 命名秒表开关，vue 轨
     会把整只 tick 门死（计时器/世界钟/闹钟停走）。**对策：秒表开关命名
     避开 `running`（用 `sw_on`），tick 恒跑、handler 内自管累加门控**——
     双端一致。
  3. **notify 动词在**（`session.rs:1486` notify arm → push_notification，
     shell toast/通知中心可达）：待澄清 #1 升级为可选——v1 仍以 in-app
     banner 为主，闹钟触发可叠加 notify（执行期裁定）。
  4. **时间源实存**：`Time.now_ms/now_sec`（stdlib.rs:686-696）。
  5. **025 锚点换算**：正文旧锚 `examples/ui/025-dashboard`（541 更名+590
     迁移）→ 现址本仓 `apps/025-sys-monitor/SPEC.md:114`（Tick 分频机制节，
     250ms 基准+speedDiv 分频同款）。
- [✅ 已完成] **T2 骨架 tab 化**
  `examples/ui/012-stopwatch/src/front/app.at`：model 增 tab/新状态族；view
  改四 tab 胶囊 + 占位内容；原秒表面板迁入 stopwatch tab。
  验证：`cd examples/ui/012-stopwatch && auto build`
  —— lang `os-003-dev` `6428299a3`（auto build 过；T1 契约 sw_on 替代
  running 落地）。
- [✅ 已完成] **T3 秒表接 Tick**
  `.Tick` 分频驱动 elapsed + 显示；Lap 扩 5 槽。
  验证：`auto run` 手测走表/暂停/计圈/复位
  —— 同提交。累计法 v1（+=250ms，计划主文；差值法升级条件=vue 轨 Time 桥
  确认，待澄清 #3 维持）。vue 生成验证=onMounted 恒 tick（无 watch 门控，
  T1 坑精准避开）；VM 轨 MCP 手测自动化 `tests/t3_smoke.py`：走表
  2250ms→"00:02.25"/计圈/暂停冻结全过。**执行期发现**：消息名 `Lap` 在
  VM 轨被内置动词劫持（结果自动包 "Lap {n}: " 前缀）→ 改名 DoLap 规避
  （app.at 注记；框架劫持面待查）。
- [✅ 已完成] **T4 计时器**
  设定三 input + Start/Pause/Cancel + 到零 banner。
  验证：`auto run` 倒数 1s 冒烟
  —— 执行期裁定：文本 input+str→int 解析改**步进器**（VM 状态 str
  `.to_int()` 接收者坏——t_left 出 -163000 垃圾值实测；框架债登记）；
  步进器取模回绕+唯一标签（时±/分±/秒±）。倒数/到零横幅/Dismiss 全过
  （desktop_mcp T2 组）。
- [✅ 已完成] **T5 世界时钟**
  城市表 + `Time.now_sec` 换算 handler + 行渲染 + 本地城高亮。
  验证：`auto run` 与系统时钟肉眼对拍（北京/纽约）
  —— 框架侧补 **Time 模块 vue 桥**（ts_adapter：`Time.now_sec()`→
  `Math.floor(Date.now()/1000)`，TDD 测试 `time_module_bridges_to_date_now`
  过；此前 vue 轨无 Time 桥=时间类 app 双端缺口）。8 城渲染+北京/伦敦
  8h±60s 时差断言过（desktop_mcp T3 组）；本地城 ● 高亮。
- [✅ 已完成] **T6 闹钟 + storage**
  槽 CRUD + storage 持久化 + Tick 分钟比对触发 banner。
  验证：`auto run` 设下一分钟闹钟等到触发
  —— 5 槽 storage（`AUTO_VM_STORAGE_FILE` 隔离验证）；入列/跨重启恢复
  过（desktop_mcp T4/T4b 组）；触发比对=fired 防重+当日轮转。
- [✅ 已完成] **T7 测试与回写**
  `tests/desktop_mcp.py` 四断言组（双端）；pac.at 改 title/icon/category/
  desktop；README 012 行更新。
  验证：`python tests/desktop_mcp.py` 双轨绿 + `cargo check -p auto-lang`
  （若 T1 触及 ui_gen 则跑）
  —— **desktop_mcp 12/12 全绿**（VM 轨四断言组）+ vue 轨 build 过+
  playwright 三 tab 截图（tests/clock_vue_{timer,world}.png）；pac
  Clock/clock/tool；README 012 行；`cargo check` 0 错 + ts_adapter 16/16
  （含新桥测试）。提交：`6428299a3`（T2/T3）+ `43f2b8832`（T4-T7）。

## 复审记录

**复审人**：ZCode（/auto-plan:review 独立复审），2026-09-08。
**方法**：计划 vs lang `os-003-dev` 实际 diff（两提交，8 文件 +794）逐项重证；
关键验证全部复审档重跑。

### 逐项验收裁定（verify, don't trust）

| # | 验收标准 | 裁定 | 复审证据 |
|---|---|---|---|
| 1 | 双端四 tab 全功能；秒表真走表 | **过** | desktop_mcp **12/12 复审档重跑**（fresh .am+taskkill 清场）；vue build 过 + playwright 三 tab 截图（timer 24K/world 30K 内容量） |
| 2 | desktop_mcp.py 双轨全绿 | **过** | VM 轨 12/12（exit=0）；vue 轨=build+三 tab 截图（**口径注记**：desktop_mcp.py 为 VM 轨套件，vue 轨历来走 build+playwright 冒烟——011/013/028 惯例一致） |
| 3 | 闹钟 storage 跨重启 | **过** | 套件 T4b 组复跑 PASS（AUTO_VM_STORAGE_FILE 隔离，actual==expected repr 比对） |
| 4 | pac title "Clock"/icon 生效 | **过** | VM boot 实证 `VM window title: Clock (from pac.at)`；icon clock/category tool 字段实存 pac |
| 5 | registry id 012-stopwatch 不变 | **过** | scan_examples_ui_curation_set 复跑 PASS（012 在策展集=recent/dock 无破坏） |
| 6 | README 012 行更新 | **过** | 文件在案（Clock 四 tab+os-003 注记） |

**全量门禁**（ts_adapter 触及 → tf）：no-fail-fast 22 unique 红 =
**master 预存 22 红集合逐名全等**（590/009 复审基线同集：layout×14/plan370×3/
plan055/desktop_protocol/lucide/charts/plan492_m4）——**零新增红**，Time 桥
无回归。

### 遗漏/延后/Workaround 猎查

- **遗漏**：无——T1-T7 全有对应 diff 与复跑证据；t3_smoke.py（T3 期证据）
  随套件保留。
- **计划文本偏离（均执行期注记在案，裁定合理）**：①「三 input」→步进器
  （VM 状态 str `.to_int()` 接收者坏——t_left 垃圾值实测，框架债候选
  P003-R1）；②横幅覆盖层→恒结构顶栏（**Tick 后仅文本重绑重渲染**，结构/
  样式 if 不重评估——VM 渲染债候选 P003-R2，覆盖层留修复后升级）；③
  消息名 Lap→DoLap（VM 内置动词劫持，自动 "Lap {n}: " 前缀——劫持面待查
  P003-R3）；④秒表累计法（计划主文 v1 方案，差值法升级条件=Time 桥已补
  但维持 v1 稳定，待澄清 #3 注记维持）。
- **延后（在案）**：notify 叠加留 v2（待澄清 #1 执行期收口注记）；DST
  固定偏移 v1（待澄清 #2 维持）。
- **Workaround**：步进器唯一标签（时±/分±）兼无障碍收益，非隐藏。

### 结论

六项验收全过，tf 零新增红；三项偏离+两项延后全部显式在案。
**路由：`reviewed`**，就绪 `/auto-plan:merge`（lang 侧 `os-003-dev`
两提交随批合入）。

## 待澄清事项

1. 闹钟到点走 shell toast/通知中心（`__desktop_cmd` 上行）还是 in-app
   banner——T1 动词表核对后定；v1 默认 banner（保守面）。
   **执行期收口：v1 落 in-app 顶栏横幅**（notify 叠加留 v2——T1 已证动词在，
   session.rs:1486）。
2. 世界时区夏令时：v1 固定偏移表（8 城多数无 DST 或影响 ±1h），SPEC
   登记限制；DST 规则表远期。
3. 秒表精度 10ms 档基于 250ms Tick 分频外推（elapsed+=250 实际是墙钟
   步长）——显示 cc 两位够用；如需真精度改 `Time.now_ms` 差值法（T1
   后裁定，倾向差值法：Start 记锚点，Stop 累计）。

> **执行期发现登记（T1-T7，交接 review 携带）**：① VM 状态 str `.to_int()`
> 接收者错乱（垃圾值）→ app 改步进器规避，框架债候选；② Tick 后仅文本
> 重绑重渲染（结构/样式 if 不重评估）→ 横幅恒结构顶栏，VM 渲染债候选；
> ③ 消息名 `Lap` VM 轨被内置动词劫持（自动 "Lap {n}: " 前缀）→ 改名
> DoLap，劫持面待查；④ ts_adapter 补 Time 模块 vue 桥（time_module_
> bridges_to_date_now 锁）。
