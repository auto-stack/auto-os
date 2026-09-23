---
plan_id: PLAN-042
status: drafting               # drafting → executing → execution_done → reviewed → archived
feature_name: desktop-syslog（系统日志 app）
author: [agent]
created_at: 2026-09-23
updated_at: 2026-09-23
plan_revision: 1
current_step: 0
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
- **T-02 log trap + 诊断家族双写**：①排查桌面轨 logger 初始化现状
  （bounded，产物=一行结论进本节证据）；②boot 装环 logger（或已装
  降级路径）；③§4 所列 eprintln 载重站点加 `syslog!` 双写（renderer.rs
  12 站点起步，补全以 grep 复核为准）。→ AC-02。依赖：T-01。
- **T-03 VM 错误 trap**：排查 handler/face 失败与未捕获 VM 错误出口
  （10506/10643/10788/11714 族 + msg pump dispatch 路径），统一
  `syslog!(error, "vm:<app>", ...)`。→ AC-03。依赖：T-01。
- **T-04 `log` 动词**：session.rs DesktopCommand::Syslog + parse 臂
  （notify 同型）+ 执行臂入环（notify_source 归因）+ 协议 v1.9 版记；
  单测 parse/兜底/归因。→ AC-04。依赖：T-01。
- **T-05 viewer app**：auto-os `apps/039-syslog/` 全套（pac.at/src/
  front/tests/README）+ apps.manifest 行（ports [17800]，无 daemon）+
  本仓 README Apps 表同步（AGENTS.md §4）。→ AC-05/AC-07（窗体部分）。
  依赖：无（可与 T-01..04 并行，store 以 mock 注入开发）。
- **T-06 注入泵**：renderer ServiceTick 段（registry_id 定位 + seq 门
  + 500ms 节流 + `__syslog_entries` 注入）；单测假时钟。→ AC-06。
  依赖：T-01/T-05。
- **T-07 music 插桩**：020-music-player scan 链三钉（`music-scan`
  前缀）。→ AC-08。依赖：T-04。
- **T-08 D6 修复腿（可分离）**：ScanProbe.at 复现重建 → 单测钉红 →
  VM 异步回传修复 → 绿。→ AC-09。依赖：T-04（复现件用 `log` 动词自证
  可见）；与 T-01..07 无序耦合。
- **T-09 收口验证 + spec 沉淀**：headless+实机全量 AC 过台、证据入
  `docs/plans/evidence/p042/`、SD-01/02 落笔、双仓门（auto-lang
  `cargo t` 对拍基线、auto-os app 侧 e2e）。→ AC 全量。依赖：T-01..08。

## 9. 复审记录

- 2026-09-23 r1 起草 handoff（/auto-plan:new）：stage: new；
  outcome: pass（授权面=用户提案+设计共识，D6 优先级裁定记录于 §4，
  默认随腿可分离）；next: work（`.wt/os-042/` 双仓组开工）。

## 10. 待澄清事项

1. **D6 优先级裁定**（用户）：默认窗口先行、D6 随腿（T-08 可分离）；
   若裁定 D6 单独先行，本计划 T-08 移交独立腿，AC-09 随撤。
2. apps/039 编号登记时复查（并行会话撞号防线——launcher 4028/038
   4038 先例说明 apps 编号与 examples 编号可能交错）。
3. .at 剪贴板能力现状（T-05 单条复制的实作形态：真复制 vs 选中详态
   降级）——T-05 实作时按仓库现状定，不阻塞。
4. 桌面轨 logger 初始化现状（T-02 ①产物）：simplelog 若已占位，
  trap 改宏面覆盖——已在 T-02 内 bounded 处置。
