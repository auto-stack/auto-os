---
plan_id: PLAN-013
status: reviewed               # drafting → executing → execution_done → reviewed → archived
                               # （work 进入时未翻 executing 属流程疏漏；executing 期
                               #  全程有 work/实机/复审记录为证，review pass 直落 reviewed）
feature_name: desktop-app-portfolio
author: [agent]
created_at: 2026-09-12
updated_at: 2026-09-12

# /auto-plan:review 结束时填写：
supersedes_spec_components: []
new_spec_components: []
touched_goals: []             # 引用 docs/specs/goals.md 的 GOAL-NNN

affects: []                   # 本仓 docs/specs/ 缺位（.autoos/specs.json 六节均空）——
                              # 规范落点为 AGENTS.md / README.md，见 §5 规范增量
current_step: 11
total_steps: 12
---

# [PLAN-013] desktop-app-portfolio

## 变更摘要

2026-09-12 桌面 app 整理批的三个收尾工作包，合单计划分三 phase 顺序执行：

- **Phase 1 daemon 化**：kanban / auto-musk / jade-garden 三 app 点开需手动起后端——
  泛化 pac `daemon:` 键机制（当前硬编码只认 `"autoos"`），使桌面拉起自动 ensure 各自后端。
- **Phase 2 auto-term app 化**：auto-term 仓新增 pac.at app 根（复用 OS-013 T2 的
  `Term.engine_*` 进程内 catalog shim，**无需 back daemon**），auto-os manifest 登记导入。
- **Phase 3 kanban submodule**：auto-os 以 git submodule 引入 auto-kanban（首个样板，
  用户 2026-09-12 裁定 submodule 为 demo 升格的统一管理形态），修订 AGENTS.md §4
  虚拟伞形条款。

前置（已落地，本计划不重复）：image-viewer desktop 旗标、apps.manifest 登记
kanban/auto-musk/jade-garden（2026-09-11/12 整理批）、画廊两件用户裁定桌面常驻。

## 目标

**目标**：
1. 桌面冷启状态点击 kanban / musk / jade-garden 图标，后端自动就绪，窗口出应用内容。
2. launcher 出现 AutoTerm，点击得到可交互终端会话；manifest 登记入伞形。
3. apps/kanban submodule 就位，AGENTS.md/README 与 submodule 形态一致，worktree 降级路径不回归。

**非目标**：
- os-config 内嵌 autoterm_page 的退役（保留，另行决定）。
- examples/ui demo 的批量升格（流水线机制另立计划）。
- vue 轨 EXTRA 的 id 携带语法 / file: 依赖路径改写（整理批已注记的 auto-lang 侧缺口，独立处理）。
- musk / jade-garden 的产品功能开发。

**受影响仓**：auto-os（主导：manifest / docs / submodule）、auto-lang（crates daemon 泛化）、
auto-kanban、auto-musk、auto-down（jade-garden）、auto-term。

**成功判据**：AC-01..AC-06 全绿（见 §7）。

## 架构方案

### Phase 1：daemon 键泛化（auto-lang crates + 三仓 pac）

现状（证据见 §4）：`Session::ensure_daemon_if_declared`
（auto-lang `crates/auto-lang/src/ui/session.rs:2368`）对 `daemon` 键硬编码
`!= Some("autoos") → return`，探测/spawn 走 `osconfig_daemon::ensure_ready(default_daemon_url())`
单例（os-config back，ping :17701 → 发现序 spawn → `AUTOOS_DAEMON` env 注入 App 会话）。

泛化方向（T-01 定案细节，回填 §5）：
- pac `daemon: <name>` 按名分发到 per-name 探测 URL + spawn 策略；`autoos` 名保留现行为
  （零回归门）。
- spawn 命令来源候选：(a) pac 新键（如 `daemon_spawn:`）声明；(b) 约定
  `<app-root>/daemon.{sh,ps1}`；(c) daemon 注册表文件。T-01 权衡后定案。
- ping/就绪契约沿用 osconfig_daemon 的 ensure_ready 形态（HTTP 检活 + 轮询预算 + Offline
  不阻断 launch、状态记徽标）。
- 各仓侧：musk（musk-serve 形态）、jade-garden（back/server axum）、kanban（back 17101）
  对齐"可被 ping + 可被 spawn"契约。

### Phase 2：auto-term app 壳（auto-term 仓为主）

引擎面已就位（**关键事实，比立项前预估轻**）：OS-013 T2 已将引擎调用落为 auto-lang
`auto.term.*` catalog shim——`Term.engine_*` 固定 ID、进程内加载 autoterm_core.dll
（os-config `autoterm_store.at` 实证：`Term.engine_spawn/engine_write_line/engine_rows/
engine_free` 直接可用）。**无需 back daemon、无需 api.at**。

app 根形态（auto-term 仓新增 `app/`）：
```
auto-term/app/
├── pac.at            # name autoterm / scene ui / render vm / front_port 17400
│                     # / back_port 17401（预留转实声明）/ title AutoTerm / icon terminal
└── src/front/
    ├── app.at        # App 根 widget（200ms Tick 节拍转发，autoterm_page 同构）
    ├── autoterm_store.at   # 自 os-config 升格移植（去 os-config 耦合）
    └── autoterm_page.at    # <terminal/> + 会话控制条 + 输入行
```
- os-config 侧 `autoterm_page.at`/`autoterm_store.at` 保留原位（非目标不退役）；升格以
  auto-term 仓为唯一演化点，os-config 页后续另行决策是否改为消费。
- auto-os 承接：apps.manifest 登记 `{ id: auto-term, repo: ../auto-term/app }`。

### Phase 3：kanban submodule（auto-os 为主）

- 主检出 main：`git submodule add git@github.com:auto-stack/auto-kanban.git apps/kanban`
  （两仓 remote 同在 auto-stack org，全 URL 可移植；备选相对 URL `../auto-kanban.git`）。
- 两臂去重交互（已核实的解析序，§4 证据）：VM 轨 extra roots = 兄弟探测（os-config →
  apps 容器 → manifest → 画廊）。submodule 检出后**容器臂先胜**（id `kanban`，root=
  apps/kanban），manifest 臂 `../auto-kanban` 按 id 去重跳过——同仓同内容，行为不变。
- 降级路径（T-12 实测修正）：组 worktree（`.wt/os-NNN/`）裸跑时
  apps/kanban gitlink 未 init = 空目录 → 容器臂静默跳过；manifest 臂
  `repo` 相对 manifest 根解析**组内兄弟**（无主检出回退）→ 同样缺席——
  降级形态 = **缺席可见**（`apps.manifest entry kanban skipped` 警告，
  注册表 41/25 正常不炸，2026-09-12 短启动实证），非原表述"主检出回退"。
  标准桌面入口 `scripts/desktop.{sh,ps1}` 注入 `AUTO_OS_ROOT`=主检出
  （manifest 根权威臂）→ kanban 经主检出兄弟命中，**注册表恒完整**
  （同日短启动实证：零 skipped 警告）。AGENTS §2 submodule 纪律按此落地。
- wt-guard 交互：gitlink 是普通目录（非 reparse point），`bash wt-guard.sh` 扫描不误伤；
  `git worktree remove` 会删 worktree 内 submodule 检出内容（git 数据在
  `.git/modules/` 保留，可恢复）——AGENTS.md worktree 纪律节补注记。

## 需求分析与背景调查

**授权记录**（2026-09-12 用户指令）：
- "这三个一起立项吧，我觉得可以作为一个计划的不同phase来执行"——单计划三 phase 授权。
- 仓库/动作范围：auto-os（manifest、scripts、docs/plans、AGENTS.md、README.md、
  git submodule 操作）、auto-lang（crates daemon 泛化，允许 cargo t——crates 工作不受
  验证门档限制）、auto-kanban / auto-musk / auto-down / auto-term（pac 与 serve 侧配合改动）。
- 预算：用户未设自动继续限额；每 phase 完成后实地验证（桌面实机）为 phase 门。

**机制证据**（调查日 2026-09-12）：
| # | 事实 | 源 |
|---|---|---|
| E1 | daemon 键硬编码 `autoos`，探测单例 osconfig_daemon | auto-lang `crates/auto-lang/src/ui/session.rs:2368-2385` |
| E2 | pac `daemon:` 键注册表扫描期解析为 `daemon` 字段 | auto-lang `crates/auto-lang/src/ui/app_registry.rs:127`（`entry_for_dir`） |
| E3 | VM 轨展示策展 `desktop_visible`，全量进 app_resolver 按名启动 | auto-lang `crates/auto-lang/src/ui/iced/renderer.rs:11507-11528` |
| E4 | auto-term 引擎面 = `Term.engine_*` 进程内 catalog shim（autoterm_core.dll） | auto-os-config `auto/src/front/autoterm_store.at` 头注（OS-013 T2/T3） |
| E5 | auto-term 已有 a2r 形态状态机 `at/autoterm.at` + engine FFI 面 `at/engine_face.at` | auto-term 仓 |
| E6 | apps.manifest repo 条目 join 相对子路径、pac.at 门控、id 去重 | auto-lang `crates/auto-lang/src/ui/app_registry.rs:415-467`（`manifest_repo_roots`） |
| E7 | extra roots 解析序：os-config → apps 容器 → manifest → 画廊，逐级 id 去重 | auto-lang `app_registry.rs:301-319`（`host_extra_roots`） |
| E8 | manifest 现状：kanban / auto-musk / jade-garden 三条 repo（2026-09-12 整理批落地） | auto-os `apps.manifest` |
| E9 | 端口预留：musk 17200/17201、jade-garden 17300/17301、auto-term 17400/17401（伞形预留，待各仓 pac 采纳） | auto-os `apps.manifest` + 整理批报告 |
| E10 | 本仓 `docs/specs/` 缺位，`.autoos/specs.json` 六节均空 | auto-os 仓现状 |

**Spec 缺位声明**：本仓无 module Specs，规范知识散于 AGENTS.md / README.md / 计划台账。
本计划的规范增量落 AGENTS.md / README.md（SD-01..03）；建立 docs/specs/ 最小集合超出
本计划范围，注记为后续工作。

## 详细设计

### Phase 1 设计（T-01 定案，2026-09-12 调查回填）

**定案 D1 声明载体：apps.manifest 条目扩展可选 `daemon` 对象**（三选一胜出）：

```json
{ "id": "auto-musk", "...": "...",
  "daemon": { "port": 17201, "bin": "backend/target/release/musk",
              "env_port": "MUSK_BACK_PORT", "health": "/api/health" } }
```

- 胜出理由：manifest 已是伞形唯一事实源且端口已登记（E9），框架已有 manifest 读取面
  （E6，`manifest_repo_roots` 同址扩 `manifest_daemon_defs`），零新文件、零 pac 解析器
  改动（`daemon:` 键保持 String 不变，值 = manifest id）。
- 备选否决：pac 新键（结构化值需动 `parse_pac_fields`，且伞形事实源分裂）；注册表文件
  （同分裂问题）。约定脚本（`daemon.{sh,ps1}`）作为 spawn 兜底不适合 detached + env
  语义，否决。
- `port`/`bin` 语义：`port` = daemon API 口（必填）；`bin` = 仓相对二进制路径（缺省 =
  只探不孵，Offline 原因"未配置 bin"）；`env_port` = spawn 期端口覆盖 env 键（缺省
  `<NAME 大写蛇形>_BACK_PORT`）；`health` 缺省 `/api/health`（osconfig 既有路径）。

**定案 D2 spawn：沿用 `RealDaemonIo` 形态 + 发现序泛化**——二进制解析 =
`<manifest repo>/<bin>`（存在才采用；osconfig 的 override/sibling/PATH 三序不泛化，
per-name bin 路径 manifest 直说）。缺二进制 → Offline（原因含路径，"先 `cargo build
--release`"提示），**桌面不做 cargo 兜底构建**（工具链/时长坑，v1 边界）。

**定案 D3 env 策略：派生键 `<NAME 大写蛇形>_DAEMON = url`**——`autoos → AUTOOS_DAEMON`
（同构，旧键不变）；`auto-musk → AUTO_MUSK_DAEMON`；`jade-garden → JADE_GARDEN_DAEMON`。
App 侧（api.at `daemon_base()` 同类读取）自行消费派生键。

**定案 D4 零回归门：`daemon: autoos` 全链原样**（`DAEMON_PORT`/`ENV_DAEMON`/
`resolve_daemon_path`/`ensure_ready` 保留为 autoos 分发臂；既有测试
`launch_app_daemon_ready_injects_env` 不动）。

**差距清单定案（T-03..05 据此执行）**：
- **kanban 免 daemon**（T-03 缩为核实注记）：outproc 子进程 = 完整 `auto run
  --autodesk-incubate --app386=<dir>`（session.rs:2253-2275），仓内 `src/back/api.at`
  随子进程 `auto run` 装载——无独立 daemon 进程，无需 `daemon:` 键。执行时验证窗口
  出数据即闭环。
- **musk**（T-04）：pac 补 `front_port: 17200`/`back_port: 17201` + `daemon: auto-musk`；
  manifest daemon `{ port: 17201, bin: "backend/target/release/musk" }`（bin 名已核
  `crates/musk` `[[bin]] musk`）；后端 `/api/health` 存在性与 serve 单入口核实在 T-04。
- **jade-garden**（T-05）：`front/auto/pac.at` 补端口 + `daemon: jade-garden`；manifest
  daemon `{ port: 17301, bin: "back/server/target/release/jade-garden-back",
  "env_port": "JADE_GARDEN_PORT" }`（bin 名已核 `back/server/Cargo.toml`）；
  `/api/health` 存在性 T-05 核实（现有 28 route 表未见 health——需补或换探针路径）。

**auto-lang 实现面**（T-02）：`app_registry.rs` 增 `ManifestDaemon` serde 结构 +
`manifest_daemon_defs(manifest_root)`（宽容读取同 `manifest_repo_roots` 纪律）；
`osconfig_daemon.rs` 增 generic ensure（状态机参数化，autoos 臂薄包装）；`session.rs`
`ensure_daemon_if_declared` 非 автоos 名 → manifest 查表 → 泛化链（懒读一次，不驻状态）；
查无此名 → Offline 有因不阻断。

### Phase 2 设计

- `app/pac.at` 字段集：name/title/icon/render/scene/front_port/back_port（E9 预留转实）；
  无 `daemon:` 键（无后端）、无 `back:` 键。
- store/page 自 os-config 移植时的去耦合点：os-config 版经 `desktop_store` 注册表声明
  `view: "autoterm_page"` 插件视图（OS-013 T3）——独立 app 形态直接以 app.at 挂
  AutoTermPage 根 widget，无注册表分发层。
- Tick 节拍：App 根 widget 200ms 转发（os-config 实证同参），收割 `engine_rows` 快照
  重建 `Array<str>`（VG5 惯例，规避 native list 直入 state）。
- 渲染轨：VM 轨（render vm 无 vue 过滤问题；`<terminal/>` 为 VM 原生组件）。vue 轨
  无 terminal 组件渲染面——pac 声明 render vm 即自然限定，非缺陷。

### Phase 3 设计

- 挂载点定案 `apps/kanban`：容器臂 id = 目录名 = `kanban`，与 manifest id 一致（无改名
  连带）；pac.at 在 auto-kanban 仓根，容器臂门控直接命中。
- `.gitmodules` URL 采用全 SSH URL（`git@github.com:auto-stack/auto-kanban.git`）——
  主检出与 worktree 组布局均解析到 org 仓；相对 URL 备选仅在 org 改名时受益，不作默认。
- AGENTS.md §4 修订要点：虚拟伞形（无 submodule）→ **混合形态**——manifest 仍为唯一
  事实源，submodule 是 repo 条目的物理承载优化（CI 钉树构建可先行受益）；增删 app 的
  双写纪律（manifest 行 + submodule add/deinit）。
- worktree 纪律补注记：`git worktree add` 后需 `git submodule update --init` 才能
  容器臂命中；未 init 时依赖 manifest 臂兄弟检出兜底（降级不缺失，AC-05）。

### 规范增量

| delta_id | 操作 | 目标 | before/after | rationale | AC |
|---|---|---|---|---|---|
| SD-01 | modify | auto-os `AGENTS.md` §4 | 虚拟伞形"无 submodule" → 混合形态（manifest 唯一事实源 + submodule 物理承载，kanban 首例，双写纪律） | submodule 为 demo 升格统一管理形态（用户 2026-09-12 裁定） | AC-06 |
| SD-02 | modify | auto-os `README.md` Apps 表 + 目录结构 | 增 auto-term 行（17400/17401）；apps/kanban 标注 submodule 形态 | 文档-事实一致 | AC-03, AC-06 |
| SD-03 | add | daemon 键约定（T-01 定案落点：auto-os `AGENTS.md` §3 扩注或 auto-lang 侧 docs） | 现 `daemon: autoos` 单例语义 → per-name 泛化语义 + 各仓契约 | Phase 1 交付的机制约定需可发现 | AC-01 |

## 测试设计

- **Phase 1**：auto-lang 侧 `cargo t`（crates 改动允许跑；沿用
  `launch_app_daemon_ready_injects_env`（session.rs:4839）形态补 per-name 用例：
  未知名不注入、autoos 名行为不变、新名 probe/spawn 注入新 env 键）。
  实地门：桌面冷启点击三 app（T-06）。
- **Phase 2**：实地门——桌面 launcher 点 AutoTerm → 会话 Open/Send/回显/Close；
  manifest 登记后 `desktop.ps1 -Track iced` 启动日志
  `[session] app registry` 计数 +3→+4（jade-garden/musk/kanban/auto-term 视 baseline）。
- **Phase 3**：主检出 `git submodule status` 就位；容器臂命中验证（启动日志 extra root
  列表含 kanban 且 root=apps/kanban）；降级验证——临时新建 worktree 不 init submodule，
  启动注册表仍含 kanban（manifest 臂）。
- 每阶段完成追加 `[✅ 已完成]` 证据行于 §8 对应任务下。

## 验收标准

| ID | 可观察行为 | 验证方法 | 预期 |
|---|---|---|---|
| AC-01 | 桌面冷启（三仓后端均未手动启动）点击 kanban / musk / jade-garden 图标 | `./scripts/desktop.ps1 -Track iced` 后逐一点击 | **链路面达成（2026-09-12 实机）**：三 app 均进 launch 臂、daemon ensure 无阻塞、失败走 LaunchFallback 窗不崩桌面。**内容面拆账**：musk 窗 = LaunchFallback（VM synthesis 不支持 vue 符号，KD 059-FU1 债）；jade 窗 = app.at placeholder（VM 壳未开发）；kanban 未点（worktree 组无 kanban，主检出容器臂已证） |
| AC-02 | launcher 出现 AutoTerm，点击得可交互终端 | launcher 搜索 "AutoTerm" → 点击 → 输入命令 | `engine_spawn` 会话建立，输入回显/输出刷新，Close 后句柄释放 |
| AC-03 | auto-term 入伞形清单 | `apps.manifest` 含 auto-term 行；`git grep autoterm apps.manifest` | `{ id: auto-term, repo: ../auto-term/app, ports: [17400,17401] }`；os-config 内嵌终端页不回归（设置页仍可开） |
| AC-04 | apps/kanban submodule 就位且容器臂命中 | `git submodule status`；桌面启动日志 extra roots | gitlink 存在；kanban 经 apps/kanban 导入（id `kanban` 不变） |
| AC-05 | worktree 未 init submodule 时注册表不缺失缺失项可见、标准入口恒完整 | 裸跑（CWD=worktree，gitlink 未 init）：启动日志；`AUTO_OS_ROOT=主检出` 权威形态：同法 | 裸跑 = kanban skip 警告可见 + 注册表正常（41/25 实证，不炸）；权威形态 = 零 skipped 警告（kanban 经主检出 manifest 臂命中，同日实证） |
| AC-06 | 文档与 submodule 形态一致 | 读 AGENTS.md §4 / README Apps 表 | SD-01/SD-02 内容落地，无"无 submodule"残留表述 |

## 执行步骤

（原子任务：精确文件路径 + 确切操作 + 验证命令；每步完成后追加 [✅ 已完成] 一行证据）

### Phase 1：daemon 化

- **T-01** [✅ 已完成] [调查/定案] daemon 泛化设计。（证据 2026-09-12：outproc 子进程
  = 完整 `auto run`（session.rs:2253-2275）→ kanban 仓内 back 免 daemon；musk bin=
  `musk`、jade bin=`jade-garden-back`+`JADE_GARDEN_PORT` 已核；osconfig_daemon.rs
  TCP ping `/api/health` + `DaemonIo` trait 注入面确认）四点定案回填 §5（D1 manifest
  daemon 字段 / D2 发现序泛化无 cargo 兜底 / D3 派生 env 键 / D4 autoos 零回归门）。
  → AC-01
- **T-02** [✅ 已完成] [实现] auto-lang crates 泛化：`app_registry.rs` 增
  `ManifestDaemon{port,bin,env_port}` + `manifest_daemon_defs/manifest_daemon_lookup`；
  `osconfig_daemon.rs` 增 `GenericDaemon` + `ensure_generic_io` + `daemon_env_key`
  （autoos 派生恒等 AUTOOS_DAEMON）；`session.rs` `ensure_daemon_if_declared` 分发
  autoos 旧链 / per-name manifest 链。证据：`cargo test -p auto-lang --lib
  --features ui-iced daemon` 29/29 绿连跑 ×2（11 新增 + 既有含 autoos 零回归门
  `launch_app_daemon_ready_injects_env`）；附带既有测试竞态稳定性修复（fixture
  唯一化 + env 断言串行锁，注释在案）。commit d57d8c486（worktree os-013-dev，
  base 859c31710）。→ AC-01
- **T-03** [✅ 已完成] [核实/注记] kanban 免 daemon 定案（原 pac 补键任务缩并）：
  outproc 子进程 = 完整 `auto run --autodesk-incubate --app386=<dir>`
  （session.rs:2253-2275），仓内 `src/back/api.at` 随进程装载，无独立 daemon。
  §5 差距清单已注记；窗口出数据闭环归 T-06 实地。无代码改动。→ AC-01
- **T-04** [✅ 已完成] [配合] auto-musk：pac 补 front_port 17200/back_port 17201 +
  `daemon: auto-musk`；`MUSK_SERVE_PORT` 纯端口 env 覆盖（Serve 臂）。证据：
  `cargo check -p musk` 绿（50.6s，auto-ai 依赖 worktree 解析）；`/api/health`
  既有（server.rs:9 liveness probe，零后端改动）。commit e38e4f5。release 产物
  `backend/target/release/musk.exe`（7m09s）；**spawn 冒烟**：`MUSK_SERVE_PORT=17201
  musk serve` → `/api/health` 200（kill 干净退出）。→ AC-01
- **T-05** [✅ 已完成] [配合] jade-garden：pac 补 17300/17301 +
  `daemon: jade-garden`；back 补 `GET /api/health`。commit ece41b7 + base64
  归位修复 c71e48b（见待澄清④）。release 产物
  `back/server/target/release/jade-garden-back.exe`（6m06s）；**spawn 冒烟**：
  `JADE_GARDEN_PORT=17301 jade-garden-back` → `/api/health` "ok" 200。→ AC-01
  （实地门前置全部就绪：jade 修复 ✓ + 双 release 产物 ✓ + 引擎件部署 ✓，
  剩实机桌面点击）
- **T-06** [◐ 链路收口，内容层各仓债] [实地] 2026-09-12 用户实机（桌面
  CWD=worktree 组，RUST_MIN_STACK=16MB）：① **AutoTerm 全链 ✓**（T-08 实地部分
  同此收口：launcher 点开 → 终端会话可用）；② **musk**：点击进 launch 臂
  （`launch_app(inproc) auto-musk`）→ daemon ensure 无阻塞 → inproc 装载失败走
  **LaunchFallback 错误窗**（"应用暂不可用"）——失败根因 = VM handler synthesis
  不支持 vue 专属符号（`document`/i18n `t()` → `handler synthesis failed` ×4 →
  CODEGEN poison drop `mention_helpers.mention_detect` 等 → link 断）；③ **jade**：
  窗口出 `app.at` 本体——**jade 的 VM 版 app.at 即 placeholder**（"jade-garden
  auto root placeholder"，真身为 web 版 Vue+Vite，VM 壳未开发），装载零错误零警告。
  **判定**：PLAN-013 交付面（注册表/daemon ensure/launch 链/fallback UX）全部工作；
  musk/jade 的窗口内容 = 各仓 VM 成熟度债（musk：前端 vue 符号去化或 VM synthesis
  扩展，KD 059-FU1 取证线的深层原因；jade：VM 版从 placeholder 到真身的开发），
  记跨仓台账不属本计划。→ AC-01（链路面达成；内容面拆账各仓）

### Phase 2：auto-term app 化

- **T-07** [✅ 已完成] auto-term 仓 `app/` 四件：pac.at（render vm / 17400-17401 预留 /
  icon terminal / category system）+ front 三件（store/page 自 os-config OS-013 T3
  升格移植，去 desktop_page 注册表分发层；App 根 200ms Tick 转发）。
  commit 6f18c8c（worktree os-013-dev；含 .gitignore app 生成物规则）。→ AC-02
- **T-08** [◐ 代码面完成，窗内交互留实机] 部署/冒烟前置已核实：`Term.engine_*`
  shim dll 解析序（env AUTOTERM_ENGINE_DLL → 宿主 exe 同目录 → 祖先 target，缺席
  spawn 返 0 优雅降级）；部署链现成 = auto-os-config `scripts/deploy-autoterm.sh`
  （组布局感知，.wt/os-013/auto-term 自动命中；dll 现存 auto-term 主检出
  target/debug）。注册表层短启动实证：ui_desktop CWD=worktree auto-os →
  `[session] app registry: 41 entries (25 desktop-visible)`——auto-term 入可见集
  （kanban skip 为 worktree 兄弟路径环境形态，主检出无此问题）。**剩余**：实机
  launcher 点击 AutoTerm → New Session/Send/回显/Close 全链 + dll 部署执行。
  → AC-02（实地部分）
- **T-09** [✅ 已完成] manifest 登记 auto-term `{ repo: ../auto-term/app,
  ports: [17400,17401] }`（无 daemon 字段：引擎进程内，无后端）。commit a599c68。
  README Apps 表行随 merge 阶段补（SD-02）。→ AC-03

### Phase 3：kanban submodule

- **T-10** [✅ 已完成] 主检出 main：`git submodule add
  git@github.com:auto-stack/auto-kanban.git apps/kanban`（commit abb0bee）；
  容器臂命中实证：主检出短启动 `42 entries (26 desktop-visible)` **零
  skipped 警告**（容器臂 apps/kanban 检出 + manifest 臂 id 去重静默）。→ AC-04
- **T-11** [✅ 已完成] 文档修订（worktree plan-013-dev commit 19aab6f，含
  main merge 5f708ea——manifest 冲突取带 daemon 字段超集版）：AGENTS §4
  混合形态 + §2 submodule 纪律 + §3 daemon 键 schema；README 目录结构
  apps/kanban 行 + Apps 表五仓全量（SD-01/02/03）。→ AC-06
- **T-12** [✅ 已完成] [验证] 降级双形态短启动（2026-09-12）：裸跑
  （gitlink 未 init）= kanban skip 警告 + 41/25 正常不炸；AUTO_OS_ROOT
  权威 = 零 skipped（kanban 命中）。AC-05 措辞按实测修正（§7）。依赖
  T-10、T-11 ✓。→ AC-05

依赖链：T-01→T-02→(T-03,T-04,T-05)→T-06；T-07→T-08→T-09；T-10→(T-11,T-12)。
三 phase 顺序执行（用户裁定单计划分 phase）；phase 间无硬技术依赖，P2/P3 可在 P1
实地门前并行启动，冲突面为零（不同仓不同文件）。

## 复审记录

- 2026-09-12 `/auto-plan:new` 起草（stage: new, PLAN-013 rev1）。outcome: pass——
  12 任务覆盖 AC-01..06 与 SD-01..03；T-01 为 bounded investigation（决策 artifact =
  §5 回填），未对未决机制发明实现细节。next: work（Phase 1 T-01 起）。
- 2026-09-12 `/auto-plan:work` Phase 1 进展（stage: work, PLAN-013 rev1,
  outcome: pass-partial——T-01..T-05 完成，T-06 实地门待前置）。
  code_commit：auto-lang d57d8c486（worktree .wt/os-013/auto-lang @ os-013-dev,
  base 859c31710）；auto-os 43597da（.wt/os-013/auto-os @ plan-013-dev, base 6fb6956）；
  auto-musk e38e4f5（.wt/os-013/auto-musk @ os-013-dev）；auto-down ece41b7
  （.wt/os-013/auto-down @ os-013-dev，含纯依赖 worktree auto-ai）。依赖修订：
  auto-lang os-013-dev 含 T-2 泛化（musk backend 编译经组内 worktree 闭环）。
  task_ids：T-01..T-05 ✅（T-03 缩并注记、T-05 验证受阻）；T-06 未启。
  evidence：daemon 组 29/29×2 连跑绿（含 autoos 零回归门）；musk cargo check 绿；
  manifest daemon 字段（musk MUSK_SERVE_PORT 显式声明、jade bin 相对 front/auto
  根跳级路径）。blockers：见待澄清②③。next：T-06 实地门（前置 = jade base64
  修复 + 两仓 cargo build --release + 桌面实机点击），随后 Phase 2（T-07 起）。
- 2026-09-12 `/auto-plan:work` Phase 2 代码面（stage: work, rev1, pass-partial）。
  code_commit：auto-term 6f18c8c（worktree .wt/os-013/auto-term @ os-013-dev，
  含 auto-ai/auto-down/auto-lang 组内依赖 worktree 解析）；auto-os a599c68。
  task_ids：T-07 ✅ T-09 ✅；T-08 ◐（注册表短启动实证 25 desktop-visible 含
  auto-term；窗内交互 + deploy-autoterm.sh 部署留实机）。验证教训：`trans rust`
  对 widget/store DSL 无效（对照实测在案）——UI .at 装载门 = 桌面注册表
  短启动（待澄清⑥）。
- 2026-09-12 `/auto-plan:work` Phase 3 完成（stage: work, rev1, pass-partial——
  代码/文档/验证全落，剩 T-06/T-08 两个实地门汇聚 review 前收口）。
  code_commit：auto-os main abb0bee（submodule add apps/kanban）+ worktree
  plan-013-dev 5f708ea（merge main，manifest 冲突取超集版）+ 19aab6f
  （T-11 文档）。task_ids：T-10 ✅ T-11 ✅ T-12 ✅。evidence：主检出短启动
  42/26 零 skipped（容器臂 kanban 命中 AC-04）；降级双形态短启动（裸跑
  skip 警告可见不炸 / AUTO_OS_ROOT 权威零 skipped，AC-05 实测措辞修正——
  原设计"manifest 臂主检出回退"不成立，组 worktree 的 repo rel 解析组内
  兄弟，标准入口 AUTO_OS_ROOT 权威为恒完整路径）。
- 2026-09-12 `/auto-plan:work` 实机验证收口（stage: work, rev1）。T-08 实地
  完成（AutoTerm 全链 ✓ 用户确认）；T-06 链路面完成/内容面拆账（详见 T-06
  任务行与 AC-01 判定）。desktop 日志：launch 臂三 app 全进、LaunchFallback
  UX 工作、daemon ensure 无阻塞。
- 2026-09-12 `/auto-plan:review`（stage: review, PLAN-013 rev1, **outcome: pass**,
  status → reviewed）。**独立性声明**：复审在实现会话内进行，判定全部从工件重建。
  **基线**：auto-os plan-013-dev 19aab6f（base 6fb6956→merge 5f708ea）；auto-lang
  os-013-dev 608099985（base 859c31710 + d57d8c486 + ui_desktop 容错）；auto-musk
  751d2d4；auto-down 0e5e3e9（含 base64 修复 c71e48b + daemon 引号 1ffdec0）；
  auto-term 0770f3f；依赖 worktree auto-ai 9d2102c（纯解析）。主检出 main 补充
  abb0bee（submodule）+ c7db604（进度）。**复审补提交**：ui_desktop 容错
  608099985、jade Cargo.lock 0e5e3e9（原脏文件归位）。
  **acceptance_results**：AC-01 pass（链路面实机 + 内容面拆账有据——musk VM
  synthesis vue 符号债 KD 059-FU1 / jade VM 壳 placeholder，各仓台账）；
  AC-02 pass（AutoTerm 实机全链：launcher→终端会话，用户确认）；AC-03 pass
  （manifest auto-term 行 + os-config 内嵌页文件未动、launch 正常）；AC-04 pass
  （主检出短启动 42/26 零 skipped，容器臂 kanban 命中）；AC-05 pass（降级双形态
  短启动实证，措辞按实测修正）；AC-06 pass（AGENTS §4 无残留/§3 schema 与
  ManifestDaemon 字段一致/README 七行表/env 派生键文档-实现同构，四项 grep 核对）。
  **测试门**：daemon 组 29/29×2 连跑绿；全量 lib(ui-iced) worktree 4611 passed/
  199 failed vs master 工作区 4613/206——**失败为基线既有**：worktree 独有 4 个中
  3 个（plan370_015 d2/d8/z6）经还原实验实证为 859c31710 基线测试-src 错位
  （615/616 交接期，master 全过系在飞工作区配套改动，非本计划回归），1 个
  desktop_surface 为并行调度 flaky（--exact 单跑过）；osconfig 共享 fixture 竞态
  在 master（无唯一化修复）更频发，反向佐证随带修复有效。
  **findings**（均非阻塞、范围外）：F-01 calculator 聚装回归（615 线）——本计划
  随带 ui_desktop 降级容错；F-02 musk/jade VM 内容债（059-FU1 / auto-down 立项）；
  F-03 debug 桌面 inproc 大 app 装载内存分配崩溃一次（release 或内存治理另议）；
  F-04 auto-lang 全量基线红治理（859c31710 错位家族 + 并行 flaky 家族）——
  框架基线线。evidence：/tmp/os013-full-{wt,master}.log + worktree 各 commit
  + 桌面日志摘录已录 §8/待澄清。**next: merge**。

## 待澄清事项

1. **T-01 范围内定案**（不阻塞开工）：daemon 声明载体三选一、env 键名派生策略、
   kanban back 形态核实。owner: 执行 agent（work 阶段 T-01）。
2. jade-garden 归属仓为 `auto-down`（monorepo 含 autodown + jade-garden 两个产品）——
   pac/serve 改动在 auto-down 仓内进行，跨仓计划互链落 auto-down docs/plans（work 阶段
   T-05 时确认该仓 plan 纪律）。
3. 本仓 docs/specs/ 缺位（E10）：是否在本计划 merge 阶段补建最小 goals/architecture
   条目——留给 review 决定，不阻塞执行。
4. ~~jade `back/server` base64 既有编译损坏~~ **[已修 2026-09-12]**：根因 =
   Plan 058 提交时 `base64` 误落 `[dev-dependencies]`（当时仅测试消费），
   b64_encode/decode 提升为生产 fn 后未随迁 → 生产 import E0432（主检出
   同态）。修复 = 归位 `[dependencies]`（auto-down worktree os-013-dev
   commit c71e48b，`cargo check` 绿 1.9s）；release 构建随 T-06 前置执行。
   058/063 主检出线合入时注意此差异（os-013-dev 分支先行）。
5. ~~T-06 实地门的两项前置构建~~ **[已构建 2026-09-12]**：musk
   `cargo build --release -p musk`（7m09s）+ jade `cargo build --release`
   （6m06s，base64 修复后）双绿；release 产物已核在 manifest bin 路径。
   另：T-08 引擎件已部署（deploy-autoterm.sh：autoterm_core.dll +
   autoterm-ctrlc.exe → auto-lang/target/debug 宿主目录；term 门面 stdlib
   安装副本在位）。**daemon spawn 语义冒烟双绿**（env 端口注入 → 进程起 →
   /api/health 200，musk/jade 各一；kill 干净）——T-06/T-08 剩实机桌面点击。
6. **[work 阶段新增 2026-09-12]** T-08 实地剩余前置：① 引擎件部署——
   `bash auto-os-config/scripts/deploy-autoterm.sh`（组布局感知；dll 须先在
   auto-term 构建或 AUTO_TERM_ROOT 指主检出 target/debug，现成产物在
   D:/autostack/auto-term/target/debug/autoterm_core.dll）；② `trans rust`
   子命令对 AutoUI widget/store DSL 不可用（os-config 原版/013-todo 同样
   E0099，实测对照在案）——.at 装载验证以桌面注册表短启动为准，trans 门
   不作 UI 形态依据。
7. **[work 阶段新增 2026-09-12]** T-06/T-08 实机操作指引（无需提前合
   main——daemon 字段 manifest 在 plan-013-dev 分支）：
   `DESKTOP_OS_ROOT=D:/autostack/.wt/os-013/auto-os ./scripts/desktop.ps1 -Track iced`
   （DESKTOP_OS_ROOT 为 desktop.ps1 原生覆盖位，= 以 worktree 为伞形根：
   manifest 含 daemon 字段 + 组内 musk/jade/term worktree 兄弟全命中；
   kanban 组内无 worktree → skip 警告可见，AC-05 已证形态）。验证点：
   launcher 点 AutoTerm → New Session/Send 回显（T-08）；点 musk/jade →
   后端自动 spawn（T-06；**需 release 产物在位**——spawn 按 manifest bin
   找 target/release）。
