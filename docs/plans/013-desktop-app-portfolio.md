---
plan_id: PLAN-013
status: drafting               # drafting → executing → execution_done → reviewed → archived
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
current_step: 5
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
- 降级路径：新 worktree（`.wt/os-NNN/`）未 `git submodule update --init` 时 apps/kanban
  为空目录 → 容器臂 pac.at 门静默跳过 → manifest 臂兄弟检出 `../auto-kanban` 兜底，
  注册表完整（AC-05）。
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
| AC-01 | 桌面冷启（三仓后端均未手动启动）点击 kanban / musk / jade-garden 图标 | `./scripts/desktop.ps1 -Track iced` 后逐一点击 | 后端自动 ensure（ping/spawn 链），窗口出应用内容；Offline 时不挂死、徽标有因 |
| AC-02 | launcher 出现 AutoTerm，点击得可交互终端 | launcher 搜索 "AutoTerm" → 点击 → 输入命令 | `engine_spawn` 会话建立，输入回显/输出刷新，Close 后句柄释放 |
| AC-03 | auto-term 入伞形清单 | `apps.manifest` 含 auto-term 行；`git grep autoterm apps.manifest` | `{ id: auto-term, repo: ../auto-term/app, ports: [17400,17401] }`；os-config 内嵌终端页不回归（设置页仍可开） |
| AC-04 | apps/kanban submodule 就位且容器臂命中 | `git submodule status`；桌面启动日志 extra roots | gitlink 存在；kanban 经 apps/kanban 导入（id `kanban` 不变） |
| AC-05 | worktree 未 init submodule 时注册表不缺失 | 新建 `.wt/os-013/auto-os` 不跑 submodule init，启动桌面 | kanban 经 manifest 臂 `../auto-kanban` 兜底，注册表完整 |
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
  既有（server.rs:9 liveness probe，零后端改动）。commit e38e4f5。→ AC-01
- **T-05** [✅ 已完成（验证受阻见待澄清②）] [配合] jade-garden：pac 补 17300/17301
  + `daemon: jade-garden`；back 补 `GET /api/health`。代码 commit ece41b7。
  编译验证受阻：`cargo check` 4 errors（vm_dispatch.rs base64 unresolved）——
  **主检出同态既有损坏**（非本计划引入；Plan 058 base64 信封线与 063 在飞交叉，
  不代修）。→ AC-01（实地门前置含此项）
- **T-06** [实地验证] 桌面冷启逐一点击三 app（AC-01 全链）。依赖 T-02..T-05。→ AC-01

### Phase 2：auto-term app 化

- **T-07** auto-term 仓新建 `app/pac.at` + `app/src/front/{app.at, autoterm_store.at,
  autoterm_store 移植, autoterm_page.at}`（§5 Phase 2 设计；移植源 = auto-os-config
  `auto/src/front/autoterm_{store,page}.at`，去 desktop_store 注册表分发层）。→ AC-02
- **T-08** [实地验证] 桌面 launcher 点 AutoTerm 全链（Open/Send/回显/Close）。依赖 T-07。→ AC-02
- **T-09** auto-os `apps.manifest` 登记 auto-term（17400/17401 转实）；README Apps 表
  （SD-02 前半）。依赖 T-08。→ AC-03

### Phase 3：kanban submodule

- **T-10** 主检出 main：`git submodule add git@github.com:auto-stack/auto-kanban.git
  apps/kanban`；启动桌面验证容器臂命中（AC-04）。注意 main 主检出运行（new-plan.sh 同纪律）。→ AC-04
- **T-11** 文档修订：`AGENTS.md` §4 混合形态（SD-01）+ worktree 纪律 submodule 注记；
  README 目录结构 Apps 表（SD-02 后半）；daemon 键约定落点（SD-03）。→ AC-06
- **T-12** [验证] worktree 降级：`git worktree add D:/autostack/.wt/os-013/auto-os
  -b plan-013-dev`（不 init submodule）→ 启动注册表 kanban 兜底命中（AC-05）；验证后
  按 wt-guard 纪律清理（`bash D:/autostack/wt-guard.sh` 先扫再 remove）。依赖 T-10、T-11。→ AC-05

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

## 待澄清事项

1. **T-01 范围内定案**（不阻塞开工）：daemon 声明载体三选一、env 键名派生策略、
   kanban back 形态核实。owner: 执行 agent（work 阶段 T-01）。
2. jade-garden 归属仓为 `auto-down`（monorepo 含 autodown + jade-garden 两个产品）——
   pac/serve 改动在 auto-down 仓内进行，跨仓计划互链落 auto-down docs/plans（work 阶段
   T-05 时确认该仓 plan 纪律）。
3. 本仓 docs/specs/ 缺位（E10）：是否在本计划 merge 阶段补建最小 goals/architecture
   条目——留给 review 决定，不阻塞执行。
4. **[work 阶段新增 2026-09-12]** jade `back/server` base64 既有编译损坏
   （vm_dispatch.rs E0432×2/E0433×2；主检出 D:/autostack/auto-down 同态复现，
   非 PLAN-013 引入）——归 Plan 058（base64 信封）/063 在飞线修复；T-06 实地门前
   需其先绿（jade daemon spawn 链的 release 构建依赖可编译的 server）。
5. **[work 阶段新增 2026-09-12]** T-06 实地门的两项前置构建：musk
   `cargo build --release -p musk`（backend/，产物 backend/target/release/musk.exe）、
   jade（base64 修复后）`cargo build --release`（back/server/）——桌面 spawn 发现序
   按 manifest bin 路径找 release 产物，debug 产物不命中（D2 定案）。
