---
plan_id: PLAN-042
status: execution_done               # drafting → executing → execution_done → reviewed → archived
feature_name: desktop-syslog（系统日志 app）
author: [agent]
created_at: 2026-09-23
updated_at: 2026-09-23
plan_revision: 1
current_step: 9
total_steps: 9

# /auto-plan:review 结束时填写：
supersedes_spec_components: []
new_spec_components:
  - auto-os docs/specs/shell/syslog.md     # 宿主环/log 动词/注入节流契约
  - auto-os docs/specs/apps/syslog-app.md  # viewer app spec
touched_goals: []             # 引用 docs/specs/goals.md 的 GOAL-NNN

affects:
  - auto-lang crates/auto-lang/src/ui/         # syslog 环 + trap + 注入
  - auto-lang crates/auto-lang/src/ui/session.rs  # DesktopCommand::Syslog
  - auto-os apps/039-syslog/                   # 新 app（front-only）
  - auto-os apps.manifest + README Apps 表
---

# [PLAN-042] desktop-syslog（系统日志 app）

## 0. 变更摘要

桌面「系统日志」常驻工具：宿主侧有界环形缓冲统一采集三层日志（宿主
`log` crate 记录 + `[session]` 等 eprintln 诊断家族 + VM App 级 handler
失败），新增 `log` 出向动词（notify 姊妹、联合排空分段归因 registry_id），
独立注册表 app `apps/039-syslog` 以普通虚拟窗展示（**非 overlay**——
设计共识），宿主 500ms 攒批下行注入、窗关零常驻开销。首个实机验证场景 =
P041-D6 音乐 scan 链（插桩三钉 + VM get_json 修复腿，后者可分离）。

## 1. 目标

1. **可见性**：今晚 D4/D5/D6 那类排查（供给链 spawn 抢答、冷索引超时、
   VM 回传静默空）所需的诊断行，实时滚在桌面内的一个窗口里，不再靠
   stderr 文件 + MCP 探针黑盒抠。
2. **三层采集归一**：宿主自身（主题/供给/face 装载失败）、各 app（经
   `log` 动词，归因到 registry_id）、VM 运行时错误（handler 崩溃/
   Undefined symbol 家族）——一环一序（seq 单调）。
3. **性能红线**（与通知面板教训同源）：
   - 环有界 FIFO 1000 条，超限淘汰最旧；
   - **绝不逐行触发 view 重建**——注入面至多 500ms 一拍、seq 无变化
     零注入、全量快照替换（不做增量协议 v1）；
   - 窗口关闭时 ring 照转、注入泵零工作（常驻开销仅环本身）。
4. **互补不重复**：通知中心=用户通知、F12 DevTools=单 app 内省、
   stderr 文件=事后归档（保留不退役，`syslog!` 宏双写）、本窗=开发者
   系统日志——四者各守一层。
5. D6 随腿互验：日志窗作为 P041-D6 修复的验证工具；修复本身（VM
   get_json 回传 20 位数字串）为**可分离腿**。

### 非目标

- 不做 stderr 全量拦截（Windows GUI 进程无控制台，兜底靠双写宏逐点
  覆盖，不做全局 dup2 重定向）。
- 不做日志持久化落盘/跨 boot 历史（ring 进程内；落盘另立）。
- 不做 per-entry 增量同步协议、不做远程（rqhost 侧）日志聚合。
- 不动通知中心与 F12 DevTools 的既有形态。

## 2. 架构方案

```
┌─ 采集（三面）────────────────────────────────────────────┐
│ ① 宿主 log crate trap：log::set_boxed_logger → 环        │
│    （error/warn 恒收；info 默认收——环 1000 条容量内可控）  │
│ ② eprintln 诊断家族：`syslog!` 宏双写（ring + 原 eprintln）│
│    ——renderer.rs [session]/[dashboard]/[p036] 载重站点    │
│ ③ App `log` 动词：`log␟level␟text` → DesktopCommand::     │
│    Syslog → 联合排空分段执行，source=notify_source 归因    │
│    （notify 完全同型，PLAN-014 W-08 既有机制零新增）       │
└────────────────┬─────────────────────────────────────────┘
                 ▼
   ui/syslog.rs 全局环（Mutex<VecDeque<SyslogEntry>>，cap 1000，
   seq 单调分配器；snapshot(from_seq) / dirty_seq()）
                 ▼
┌─ 视图 ───────────────────────────────────────────────────┐
│ apps/039-syslog（front-only 注册表 app，普通虚拟窗）       │
│ 宿主 ServiceTick 帧泵：wm.wins 按 registry_id=="syslog"    │
│ 定位（设置窗 OSCONFIG_APP_ID 先例 renderer.rs:12004）→     │
│ seq 有变化且距上次注入 ≥500ms → __syslog_entries 全量      │
│ 快照下行注入（launcher 下行注入通道同形，renderer.rs:19674）│
│ 窗不在 = 零注入零扫描开销                                   │
└───────────────────────────────────────────────────────────┘
```

关键裁定点（沿用先例，零新机制）：
- **动词通道**：`__desktop_cmd` 三段词面 `log\u{1F}level\u{1F}text`，
  parse 臂与 notify 同型（session.rs:2039-2047）；执行期分段归因沿
  `notify_source`（renderer.rs:12064）。协议记 v1.9。
- **窗口形态**：普通注册表窗（launcher/settings 同类），**不占 overlay
  槽**——避开面板层叠坑（设计共识 2026-09-23）。
- **归因链**：app → registry_id；宿主内部 → `host`；VM 错误 →
  `vm:<app-id 或 face id>`。

## 3. 技术栈

- auto-lang：Rust（iced 宿主 + VM natives；`log` 0.4 已在依赖
  auto-lang/Cargo.toml:164，根 workspace 另有 simplelog 0.12——T-02
  排查桌面轨初始化现状）。
- auto-os：app 仓形态 `apps/039-syslog`（.at 前端 + pac.at；
  launcher 先例 kind:local）。
- 双仓 worktree 组：`.wt/os-042/{auto-os, auto-lang}`（Plan 529 布局；
  P035 双仓先例）。

## 4. 需求分析与背景调查

**授权记录**：2026-09-23 用户提案（桌面常驻 log 查看窗）+ 上一会话设计
共识达成（环形缓冲 trap / log 动词族 / 独立 app 窗口非 overlay / 攒批
刷新红线 / 命名「系统日志」）；本会话按自主模式起草。**遗留用户裁定点**：
「先修 D6 还是先做日志窗」未获明示——本计划默认**窗口先行、D6 随腿
互验**（T-08 可分离），review 时可翻案。无预算约束声明。

**背景事实**（file:line 锚，起草时核于两仓 master）：
- 通知中心先例（Plan 479）：`NOTES_CAP=50` FIFO（session.rs:240）、
  `notify_source` registry_id 归因（session.rs:361-363）、`notify`
  动词 parse 臂（session.rs:2039-2047）。
- 联合排空泵：`drain_and_execute_desktop_commands`
  （renderer.rs:11980；注册表窗按 app 分段 + minis 孵化段
  12037-12052；执行期置 notify_source 12063-12066）。
- 设置窗按 registry_id 定位先例：renderer.rs:12000-12006
  （OSCONFIG_APP_ID，wm.wins 线性查）。
- launcher 下行注入先例：renderer.rs:19674（召唤时注入平行字符串列表）；
  平行 Obj 数组注入已证形态：`__wm_notes`/`__dock_pinned`
  （session.rs shell_write_vec）。
- eprintln 载重站点（trap 面）：renderer.rs 10506/10565/10643/10788/
  10874/11616/11714/13290/15566/16033/16045/16315（[session]/
  [dashboard]/[p036] 家族 + shell/desktop surface load failed）。
- hover 零重建原语已有：`hover_area.rs`（P002 B 腿交付）——暂停滚动
  的 hover 态消费它，不逐行重建。
- D6：音乐 app = auto-lang `examples/ui/020-music-player`（scan 走框架
  `/api/media/scan` 自动供给，api.at:3）；根因钉死=get_json 回传 .at
  20 位数字串非 body（ScanProbe.at 最小复现——**原 scratch 件已不在
  磁盘，T-08 首步重建**）；get_json = VM native `auto.http.get_json`
  （native_catalog.rs:1343，opcode 3102，Void 型=异步延续）。瘦身分支
  fix-music-slim 在途（305KB→~110KB），插桩不依赖其合入。
- 端口带：17xxx 中 17800 空位（apps.manifest 现占 17100-17600/4028/
  4038）；viewer 无后端无 daemon（auto-term 先例：无 daemon 声明合法）。
- apps/ 编号：现 025/028/036/037/038 → 取 **039**（登记时复查目录防
  并行会话撞号）。

## 5. 详细设计

### 5.1 宿主环（T-01）

`crates/auto-lang/src/ui/syslog.rs`：

```rust
pub struct SyslogEntry { pub seq: u64, pub ts_ms: u64,
    pub level: SyslogLevel /* Error|Warn|Info */, pub source: String,
    pub msg: String }
// 全局：Mutex<VecDeque<SyslogEntry>>，cap = SYSLOG_CAP = 1000，满弹头。
// push(level, source, msg)：分配 seq（AtomicU64 单调）+ now 时间戳。
// dirty_seq() -> u64；snapshot() -> Vec<SyslogEntry>（全量克隆）。
#[macro_export] macro_rules! syslog {
    ($level:expr, $source:expr, $($arg:tt)*) => {{
        eprintln!("[syslog][{}][{}] {}", $level, $source, format!($($arg)*));
        $crate::ui::syslog::push($level, $source, format!($($arg)*));
    }}
}
```

双写是刻意的：stderr 文件层保留（事后归档），环是桌面内实时层。

### 5.2 log trap 与诊断家族（T-02/T-03）

- 桌面轨 boot（ui_desktop 入口）`log::set_boxed_logger` 装环 logger
  （error/warn/info 三级入环，source=`host`，target 透传进 msg 前缀）。
  **第一步排查**：desktop 轨是否已初始化 logger（simplelog 见
  auto/main.rs、compile.rs——桌面轨大概率未装；已装则改为
  `log::set_logger` 失败时仅 syslog! 宏面，不强拆既有链）。
- T-03 trap 面：App handler/face rebuild 失败（10506/10643/10788/
  11714 站点族 + T-03 排查补全 msg pump dispatch 错误路径与未捕获 VM
  错误出口），源标 `vm:<app>` 或 `host:<face>`。

### 5.3 log 动词（T-04）

- session.rs DesktopCommand 枚举 + parse 臂（notify 同型三段；未知
  level 兜底 info 不弃单——notify kind 兜底同款）；执行臂：
  `syslog::push(level, notify_source.unwrap_or("privileged"), text)`。
- 词表白名单注释 + 协议 v1.9 版记（shell_projection 契约文档所在处
  随 T-04 定位补一行）。

### 5.4 viewer app（T-05）

`apps/039-syslog/`：pac.at（name: "syslog"，scene: "ui"，render: "vue"，
front_port: 17800，icon: "scroll-text"，title: "System Log"，title_zh:
"系统日志"，category: "system"，desktop: "true"）+ src/front/（app.at +
store + 页面）+ README + tests/。

store 消费 `__syslog_entries`（Vec<Obj> 平行数组：seq/ts/level/source/
msg 五字段，注入面 session 写 store、.at 只读渲染）：
- 行形态：`HH:MM:SS  级别点  来源  文本`；error 红/warn 黄/info 灰；
- 暂停滚动：`hover_area` 原语置 pause 态 + 手动暂停钮（pause 期间注入
  照收 store 数组，仅不重渲染列表尾随——一拍全量快照替换天然支持）；
- 过滤：级别三选 checkbox + 关键字输入框（store 侧过滤视图数组）；
- 单条复制：点行复制文本到剪贴板（复用既有剪贴板能力；.at 侧无此
  原语则降级为选中高亮 + 详态面板，T-05 实作时按仓库现状定）。

### 5.5 注入泵（T-06）

renderer ServiceTick 帧泵（400ms 既有节律）内追加 syslog 段：
`wm.wins` 线性查 `registry_id=="syslog"`（查不到即返回——零开销门）；
`syslog::dirty_seq() != last_injected_seq` 且 `now - last_inject ≥500ms`
→ `snapshot()` 注入 `__syslog_entries` + 置脏 + 记 seq。窗关闭=
查找落空，环与泵皆零额外工作。

### 5.6 music 插桩（T-07）

020-music-player front scan 调用链三钉（`log` 动词）：scan 发起
（info）/响应到达（info，字节数）/entries 解析计数（=0 时 error 行，
非 0 info）。插桩行带 `music-scan` 前缀便于过滤。

### 5.7 D6 修复腿（T-08，可分离）

重建 ScanProbe.at 最小复现（get_json 异步延续收到的值=20 位数字串
而非 body）→ 单测钉红 → 定位 VM 异步响应回传站点（get_json native/
task 完成续体路径）→ 修复使延续收到 body → 单测转绿。与窗口互验：
修复前窗内 entries 计数=0 行可见，修复后非空。

### 规范增量

| delta_id | add/modify/retire | docs/specs/... target | before/after rule | rationale | acceptance |
|---|---|---|---|---|---|
| SD-01 | add | auto-os docs/specs/shell/syslog.md | 无 → 宿主 syslog 环契约（cap 1000/seq 单调/双写宏）、`log` 动词词面（协议 v1.9，分段归因 registry_id）、注入节流红线（≥500ms/seq 门/窗关零开销） | 三层采集与节流红线需跨计划稳定 | AC-02/04/06 |
| SD-02 | add | auto-os docs/specs/apps/syslog-app.md | 无 → 039-syslog app spec（front-only/无 daemon/`__syslog_entries` 消费/暂停-过滤-复制面/端口 17800） | app 登记与消费契约 | AC-05/07 |

## 6. 测试设计

- **单测（auto-lang）**：环容量淘汰/seq 单调/snapshot；DesktopCommand
  parse `log` 三段臂（含未知 level 兜底）；注入泵 seq 门（无变化零注入，
  假时钟）；ScanProbe 复现红→修复绿（T-08）。
- **headless（desktop_mcp / 无头会话）**：注册表 app 发 `log` 动词 →
  环内 source=registry_id；构造坏 handler app → 环内 error 行；开
  syslog 窗 → `__syslog_entries` 非空；关窗 → 泵空转零注入。
- **实机**：`auto run --desktop`（iced 轨）launch 系统日志 → 三级别行
  可见/暂停/过滤操作截图；020-music 曲库页 scan → 窗内 ≥3 行
  music-scan 事件；D6 修复前后对比截图。证据入 `docs/plans/evidence/p042/`。
- 门档：改 auto-lang `crates/` → 允许 `cargo t`（AGENTS.md Category
  判定：本计划改 crates，门合法）；基线对拍沿在册红归因口径。

## 7. 验收标准

- **AC-01** 环语义：容量 1000 满弹头、seq 严格单调、snapshot 全序——
  单测绿。
- **AC-02** 宿主 trap：log crate 三级 + `[session]` 家族载重站点 +
  shell/surface load failed 入环——headless 断言环内容含对应行。
- **AC-03** VM 错误可见：坏 handler app 的失败行入环（源含 app 标识）
  ——headless 断言。
- **AC-04** `log` 动词：三段词面 parse、registry_id 归因、未知 level
  兜底 info——单测 + headless 双证。
- **AC-05** 窗口可用：注册表可见「系统日志」、launch 开普通虚拟窗
  （非 overlay 槽）——实机/desktop_mcp 证据。
- **AC-06** 节流红线：注入频率 ≤2Hz、seq 无变化零注入、窗关后泵零
  工作——单测（假时钟）+ headless 断言。
- **AC-07** 视图功能：级别着色/来源列/时间戳/暂停滚动/级别+关键字
  过滤/单条复制——实机截图证据。
- **AC-08** music 插桩：scan 后窗内 ≥3 行 `music-scan` 事件（实机）。
- **AC-09** D6（可分离腿）：ScanProbe 复现单测红→修复绿；实机曲库
  entries 非空（若该腿被裁撤，本条随撤并记录于 §10）。

## 8. 执行步骤

（原子任务；每步完成后追加 [✅ 已完成] 一行证据）

- **T-01 宿主环模块**（auto-lang `crates/auto-lang/src/ui/syslog.rs`
  新文件 + 模块挂载）：SyslogEntry/环/seq/snapshot/`syslog!` 宏；
  单测容量淘汰+seq 单调。验证：`cargo test -p auto-lang syslog` 绿。
  → AC-01。依赖：无。
  [✅ 已完成] lang os-042-dev commit（T-01 单独提交）：syslog.rs 295 行
  （环 cap1000 满弹头/AtomicU64 seq/dirty_seq/snapshot 全序/injection_due
  500ms 节流纯函数/HostLogger log::Log trap+install 降级链/syslog! 双写宏
  语句宏语义）；ui/mod.rs 零 feature 门挂载；`cargo test -p auto-lang
  --lib syslog` 6/6 绿（并发进程内唯一 source 过滤口径；dirty_seq 精确
  相等断言改相对不变量——同进程并发写入者实证）。
- **T-02 log trap + 诊断家族双写**：①排查桌面轨 logger 初始化现状
  （bounded，产物=一行结论进本节证据）；②boot 装环 logger（或已装
  降级路径）；③§4 所列 eprintln 载重站点加 `syslog!` 双写（renderer.rs
  12 站点起步，补全以 grep 复核为准）。→ AC-02。依赖：T-01。
  [✅ 已完成] ①结论：桌面 iced 轨真入口 = ui_desktop **example 进程**
  （`cargo run --example ui_desktop`），本无任何 logger 初始化；`auto`
  CLI 的 simplelog（auto/src/main.rs:174）只在 CLI 子命令进程，与桌面
  进程无涉——装环即全量，降级链仅保险。②examples/ui_desktop.rs main()
  顶部 install_host_logger()。③renderer.rs 16 站点（session 诊断家族+
  shell/surface load failed+outproc spawn+fallback 臂+fit 重试+registry
  scan 等含 info 级）+ back_provision×2/shell-pack/hot_reload 共 20 站点
  双写；11685 dashboard refresh 与 9481 msg-pump 每帧 trace **刻意不入环**
  （刷屏红线）。与 T-03/T-04 同 commit（e996c3393）。
- **T-03 VM 错误 trap**：排查 handler/face 失败与未捕获 VM 错误出口
  （10506/10643/10788/11714 族 + msg pump dispatch 路径），统一
  `syslog!(error, "vm:<app>", ...)`。→ AC-03。依赖：T-01。
  [✅ 已完成] dynamic.rs 两臂（on_with_input_for Err 臂 + 路由派发 Err
  臂）入环；face 归因 = source_path 父目录/文件干（`vm:<dir>/<stem>`，
  无路径回退根 widget 名）；timer 回调失败既有 log::warn 经 HostLogger
  自流入环；vm_bridge:1744 exports/字节 dump 噪声站点不入环（renderer
  调用点已收口 Err）。T-08 探针实证：坏 handler 行落环 source=
  `vm:scan_probe/app`。同 commit e996c3393。
- **T-04 `log` 动词**：session.rs DesktopCommand::Syslog + parse 臂
  （notify 同型）+ 执行臂入环（notify_source 归因）+ 协议 v1.9 版记；
  单测 parse/兜底/归因。→ AC-04。依赖：T-01。
  [✅ 已完成] DesktopCommand::Syslog(SyslogLevel, String)；encode/parse
  臂（\u{1F}/\t 双轨；未知 level parse 兜底 info；空 text 弃单）；
  roundtrip 词表全量测试加样例（52→53 变体）；执行臂 notify_source 归因
  （None → "privileged"）；协议 v1.9 版记落 shell_projection.rs 头。
  单测 `syslog_verb_parse_semantics` + roundtrip + session::tests 84 全绿。
  同 commit e996c3393。
- **T-05 viewer app**：auto-os `apps/039-syslog/` 全套（pac.at/src/
  front/tests/README）+ apps.manifest 行（ports [17800]，无 daemon）+
  本仓 README Apps 表同步（AGENTS.md §4）。→ AC-05/AC-07（窗体部分）。
  依赖：无（可与 T-01..04 并行，store 以 mock 注入开发）。
  [✅ 已完成] auto-os plan-042-dev：pac.at（name syslog/17800/scroll-text/
  系统日志/system/desktop true/无 daemon 无 desktop_exe=解释态 launch）+
  src/front/app.at 单组件（launcher 约束同款——store 子组件 vue TS 生成
  损坏；平行字符串列表接缝 `__syslog_seq/time/level/source/msg` + hosted/
  级别三选 chip + 关键字过滤 + 暂停积压条 + 行选择详情 + clipboard_
  set_text 复制[剪贴板 native 2926 在册，§10.3 定为真复制] + 独立模式
  mock 自证面）+ tests/desktop_mcp.py 冒烟 5 断言 + README + manifest 行
  + README Apps 表。执行修正：registry_id=目录名 `039-syslog`（§10.2
  复核后钉死；泵匹配用）。
- **T-06 注入泵**：renderer ServiceTick 段（registry_id 定位 + seq 门
  + 500ms 节流 + `__syslog_entries` 注入）；单测假时钟。→ AC-06。
  依赖：T-01/T-05。
  [✅ 已完成] ServiceTick 段（sync_dash_dark_bit 邻位）+ syslog_inject_
  tick()：wm.wins 线性查 registry_id=="039-syslog"（窗不在零扫描门）→
  injection_due(dirty,last,500ms) → snapshot 全量 → 五平行字符串列表
  write_state_vec（**执行修正：Vec<Obj> → 平行字符串列表**——B12 家族
  宿主注入 Obj 数组 handler 字段读失效，launcher apps_* 先例；plan §5.4
  "Vec<Obj> 平行数组"按已证形态落）→ 显式 call_handler("Rebuild")（宿主
  写状态不触发 handler，RebuildNotes 同规）→ 簿记后置（写失败下拍重试）。
  DesktopState 增 syslog_inject_seq/syslog_last_inject。节流判定单测 =
  T-01 injection_due 假时钟 6 断言；段本体薄（查+门+写），headless 断言
  随 T-09。iced example 构建绿。
- **T-07 music 插桩**：020-music-player scan 链三钉（`music-scan`
  前缀）。→ AC-08。依赖：T-04。
  [✅ 已完成] app.at Init 臂四记录累积写 `__desktop_cmd`（\n 分隔多记录
  ——parse_records 按行切；钉1 发起 info / 钉2 响应到达[store __scan_ok]
  / 钉3 entries 计数[=0 error]）；player_store.at `__scan_ok` 旗标。
  **执行修正：字节数钉退役**——T-08 勘定 `Http.get_json` 表达式值 = 已
  解析 Obj（PLAN-080 F-2③ 编译期内联 to_value），原始 body 字节在 .at
  层不可观测；到达+解析由旗标与计数钉承载（AC-08 ≥3 行不受影响：发起/
  到达或失败/计数 = 3-4 行）。与 T-08 同 commit。
- **T-08 D6 修复腿（可分离）**：ScanProbe.at 复现重建 → 单测钉红 →
  VM 异步回传修复 → 绿。→ AC-09。依赖：T-04（复现件用 `log` 动词自证
  可见）；与 T-01..07 无序耦合。
  [✅ 已完成] **根因翻案（证据钉死）**：VM 异步延续**无 bug**——探针
  实证延续推值正确（tagged string NV）。真根因 = **PLAN-080 F-2③ 编译
  期改写 × PLAN-617 T-10 文档配方相戗**：`Http.get_json(url)` 自此编译
  为 `get_json + json.to_value`（web 轨 fetch().json() 语义），文档配方
  `json.to_value(Http.get_json(url))`（音乐 030-video 存量同形）成二次
  转换——旧 to_value 把 Obj 强转 String（NV 位型显示即「20 位数字串」
  实体）解析失败 → 静默 null → `data.entries ?? []` 恒空 = entries 0。
  根修 = **to_value 幂等**（TAG_OBJECT/TAG_LIST 接收者直通；stdlib.rs
  shim_json_to_value），存量 .at 消费方零改绿。回归钉 = plan042_scan_
  probe_tests.rs 双测（裸形态 + 文档配方）：进程内真 HTTP 端点 × 生产
  装载管线 build_dynamic_component × on_with_input_for；配方测关修复
  实测红（entries=0）、开修复绿（entries=1）。json 63 测 + plan083 桥
  5 测绿。探针勘定副产品：mount 自发 Init + 显式 Init 双派发形态、
  http 失败体 `{"error","status"}` Obj 形态。音乐 .at 零改动（幂等根修
  直接治愈）。
- **T-09 收口验证 + spec 沉淀**：headless+实机全量 AC 过台、证据入
  `docs/plans/evidence/p042/`、SD-01/02 落笔、双仓门（auto-lang
  `cargo t` 对拍基线、auto-os app 侧 e2e）。→ AC 全量。依赖：T-01..08。
  [✅ 已完成] **headless**：p042_app_at_parses（viewer 源解析门）+
  p042_log_verb_ring_attribution（AC-04 执行臂归因：privileged/registry_id
  双证）+ p042_bad_handler_error_lands_in_ring（AC-03：source=
  vm:scan_probe/bad 行入环）全绿；p042_syslog_window_launch_and_pump
  （#[ignore]+P042_APPS_DIR 门，lang 测试跨仓扫 os worktree 注册表）：
  launch→窗在册 registry_id 归因→环 3 行基线注入→平行列表可读→Rebuild
  consumed=3→新行 500ms 攒批拒→过节流追平——AC-05/06 泵全程绿。
  **app 侧 e2e**：tests/desktop_mcp.py 8/8（VM 轨：mock 种子/级别过滤/
  关键字/暂停积压-恢复/计数）。**实机**（ui_desktop iced 真桌面 ×3 轮，
  证据 docs/plans/evidence/p042/）：bus 真排空臂 launch 039-syslog → 普
  通虚拟窗开（AC-05，非 overlay）→ 窗内**真环行**（boot registry 行
  host 归因+真时刻，AC-02 实机半证）→ music launch → back-proxy 懒启
  3358 → 窗内 3 行 music-scan（initiated/arrived/entries parsed: 393，
  registry_id 归因，AC-08；393=P037 真机同数——D6 修复实机互验，AC-09）
  → 选择详情条/复制钮可见（AC-07 截图）。实机勘定两处泵修正（顶层优先
  窗定位+hosted="1" 泵写）随 lang 8f1785733。**双仓门**：auto-lang 全量
  lib 套件对拍——基线 ca880b0e0（无本计划改动）243 failed/5226 passed
  vs 本分支 238 failed/5240 passed（+14 passing 恰=本计划新增测试数；
  基线红集不增，属在册 flaky 域）；json 63/stdlib 26/dynamic 89/session
  114 定向全绿。**spec 沉淀**：SD-01（docs/specs/shell/syslog.md）+
  SD-02（docs/specs/apps/syslog-app.md）plan-proposed 落盘（review 门随
  计划，未入台账）。

## 9. 复审记录

- 2026-09-23 r1 起草 handoff（/auto-plan:new）：stage: new；
  outcome: pass（授权面=用户提案+设计共识，D6 优先级裁定记录于 §4，
  默认随腿可分离）；next: work（`.wt/os-042/` 双仓组开工）。
- 2026-09-23 r2 work handoff（/auto-plan:work）：stage: work；
  plan_id: PLAN-042 | plan_revision: 1 | outcome: pass（execution_done）|
  code_commit: auto-os plan-042-dev 3e4da88（base 01f1e88）；auto-lang
  os-042-dev 8f1785733（base ca880b0e0）| task_ids: T-01..T-09 全链 |
  evidence: §8 各任务 [✅ 已完成] 行 + docs/plans/evidence/p042/ + 双仓
  提交链（lang 3931b5b5a→e996c3393→46c6ed30e→8f1785733；os 80e5185→
  3e4da88）| blockers: 无 | next: review。AC-01..09 全量过台（headless
  单测/app e2e 8/8/实机三轮截图）；基线对拍红不增（243→238，含 14 新测
  绿）；组 `.wt/os-042/{auto-os,auto-lang,auto-down,baseline-lang}` 留
  组待 review/merge（merge 技能负责 wt-guard 清组）。

## 10. 待澄清事项

1. **D6 优先级裁定**（用户）：默认窗口先行、D6 随腿（T-08 可分离）；
   若裁定 D6 单独先行，本计划 T-08 移交独立腿，AC-09 随撤。
   → **已按默认执行**（窗口先行 + D6 随腿完成，AC-09 全证）。
2. apps/039 编号登记时复查（并行会话撞号防线——launcher 4028/038
   4038 先例说明 apps 编号与 examples 编号可能交错）。
   → **已复查**：登记时 apps/ 无 039 目录、manifest 无 17800 占用。
3. .at 剪贴板能力现状（T-05 单条复制的实作形态：真复制 vs 选中详态
   降级）——T-05 实作时按仓库现状定，不阻塞。
   → **已定**：`clipboard_set_text`（native 2926）在册，真复制落地。
4. 桌面轨 logger 初始化现状（T-02 ①产物）：simplelog 若已占位，
   trap 改宏面覆盖——已在 T-02 内 bounded 处置。
   → **已勘定**：桌面 iced 轨（ui_desktop example 进程）本无 logger，
   装环即全量；CLI simplelog 异进程无涉；降级链留保险。
5. **执行期新增（勘定记录，非阻塞）**：
   - 注入形态修正：plan §5.4 `Vec<Obj> 平行数组` → **平行字符串列表**
     （B12 家族已证形态，launcher apps_* 先例）。
   - viewer 单组件化（plan §5.4 "store + 页面" → 状态内聚 App——store
     子组件 vue TS 生成损坏，launcher 约束同款）。
   - registry_id = 目录名 `039-syslog`（plan §2 词面 "syslog" 修正）。
   - T-07 字节数钉退役：`Http.get_json` 表达式值 = 已解析 Obj
     （PLAN-080 F-2③ 编译期内联 to_value），raw body 字节 .at 层不可
     观测；AC-08 ≥3 行以钉 2 拆双行满足。
   - **D6 根因翻案**：VM 异步延续无 bug（探针实证）；真根因 =
     PLAN-080 编译期改写 × PLAN-617 T-10 文档配方二次转换；根修 =
     to_value 幂等（T-08，全量证据见 §8 T-08 行）。plan §5.7/§4 的
     "VM 异步回传站点修复"表述按实证翻案，AC-09 以新根因口径验收。
