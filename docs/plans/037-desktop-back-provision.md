---
plan_id: PLAN-037
status: executing               # drafting → executing → execution_done → reviewed → archived（R1 blocked 回位——AC-01 真机段待用户确认）
feature_name: desktop-back-provision
author: [agent]
created_at: 2026-09-20
updated_at: 2026-09-20
plan_revision: 1

# /auto-plan:review 结束时填写：
supersedes_spec_components: []
new_spec_components: [auto-lang/vm/back-proxy.md「运行期增删与生命周期」节（SD-01）, autos-desktop-program.md 后端供给裁定行（SD-02）]
touched_goals: []

affects: [auto-lang/vm, auto-lang/ui, auto-os/desktop-ledger]
current_step: 7
total_steps: 7
---

# [PLAN-037] desktop-back-provision

## 0. 变更摘要

VM 桌面（iced/inproc 直挂形态）launch app 时**后端零供给**：020-music-player
打开即"本地曲库为空"——前端 `Http.get_json("/api/media/scan")` 无进程应答
（player_store.at:90-122 catch 兜底）。PLAN-658 建成的多后端 proxy
（单端口子 URL 路由 + VM session + 宿主原生 media 路由）只挂在画廊
（ui-gallery）启动路径（rust_ui.rs:3095-3100 仅 `file_name()=="ui-gallery"`
或 gallery_mode 时 `start_gallery_back_proxy`），桌面宿主不在覆盖面。

本计划把 658 的 proxy **承载层原样引进桌面宿主**，生命周期从画廊的
"boot 期全量静态注册"翻转为"**launch 时按需装载、窗关卸载**"：

1. `launch_app`（session.rs:3072）插显式后端供给步骤，四臂决策树：
   manifest daemon（既有 ensure 链，零变化）/ pac capability（`media_root:`
   → proxy 原生 media 路由）/ .at back 特形（`~Stream`/`~Promise`/
   `use auto.*` → proxy VM session）/ 无后端（维持现状）。
2. `back_proxy` 补**运行期增删 API**（现仅 `start` 一次性注册，
   back_proxy.rs:150）。
3. 前端相对 URL 经**内存态字面量前缀化**指向 proxy（画廊
   `prefix_api_url_literals`（vue.rs:6046）同律，桌面在
   `build_dynamic_component` 前对 spec.code 变换，不落盘不改语料）。
4. 生命周期绑定窗：app_key 单例 + AppId→app_key 引用计数，最后一个窗
   关闭即卸载（drop sender → session 线程 `for req in rx` 自然退出，
   back_proxy.rs:753）；proxy 进程内懒启常驻（listener 空闲近零成本）。

## 1. 目标

- **G-1 020 桌面曲库可见**：VM 桌面打开 020-music-player 出真实曲库
  （E:\Music 经 proxy 原生 media 路由），流媒体 URL 可播。
- **G-2 决策树统一**：后端供给成为 launch 流程的显式步骤，四臂
  （daemon/capability/session/none）单一裁决点，不再散落。
- **G-3 按需生命周期（内存纪律）**：boot 零常驻（无供给则 proxy 不
  启动）；launch 时只装载该 app 的后端；全窗关闭即卸载；双窗同 app
  共享一份后端。对齐 M7-c app ≤10MB 内存门方向。
- **G-4 零回归**：manifest daemon 臂（auto-musk/jade-garden）、无后端
  app（001 族）、独立形态（`auto run -r vm` 020 单跑）、画廊形态
  （ui-gallery，658 路径含谓词/前缀化函数迁移）全绿。
- **非目标**：vue 轨桌面（iframe/Vue embed 形态）变更；跨机/远程部署；
  运行中热插拔/热换后端；manifest daemon app 收编进 proxy（一等公民
  进程带健康契约，保持 outproc 原样）；`AUTO_HTTP_BASE` 语义变更。
- **受影响仓库**：auto-lang（crates/auto-lang/src：back_proxy.rs、
  ui/session.rs、ui/app_registry.rs；crates/auto-man/src：谓词/前缀化
  函数迁移复用）+ auto-os（本计划 + 台账行）。

## 2. 架构方案

承 2026-09-20 会话两轮架构咨询裁定（用户认可推荐形态）：

**承载层复用，生命周期翻转。** 658 的 proxy（路由/崩溃隔离/原生 media/
SSE 面）是画廊与桌面共用的库层，不动语义；桌面侧新的是"何时给谁装载"
的决策与生命周期。明确拒绝面（会话中已裁定，复审不再议）：

- ❌ `AUTO_HTTP_BASE` 做 per-app base——进程单值 env，运行期读
  （stdlib.rs:6766-6770），多 app 各需各前缀无法表达（658 §5.3 同结论）。
- ❌ 每 app 独立进程 re-exec 跑完整 `auto run -r vm`——PLAN-033 已裁
  （解释态两合法形态 = inproc 直挂 / `-q` 经 native 臂，2026-09-19），
  且直挂 225MB vs `-q` 8MB 实测差。
- ❌ 运行中动态热换后端——后端绑定在 launch 时刻（进程模型语义），
  飞行中不换。
- ❌ manifest daemon app 进 proxy——真进程带 `GET /api/health` 契约，
  加中间人零收益。

**进程/线程形态**：proxy 懒启（首个需要供给的 launch 时 `start`，
port 0 自动）；listener 线程常驻（accept 空闲近零成本，避免拆
accept 循环的复杂度）；每 session 专线程（658 形态不变）；
`DesktopState` 持 `RunningProxy` 句柄至进程终。

## 3. 技术栈

- Rust：`crates/auto-lang/src/back_proxy.rs`（运行期增删 API +
  ProxyShared 并发化）；`crates/auto-lang/src/ui/session.rs`
  （launch_app 接线、DesktopState 字段、关窗钩子、LaunchSpec 扩展）；
  `crates/auto-lang/src/ui/app_registry.rs`（pac `media_root:` 解析、
  back 入口探测）；`crates/auto-man/src/vue.rs`（session 臂谓词与
  `prefix_api_url_literals` 迁至 auto-lang 后改为复用——依赖方向
  auto-man→auto-lang 成立）。
- 测试：`crates/auto-lang/src/tests/back_proxy_tests.rs` 扩展；
  session 级 headless 测试；smoke 脚本（scripts/ 同 025/033 先例）。
- 验证门：auto-lang `cargo t`（Category A——crates 改动必须过）；
  画廊/独立形态手动回归各一次。

## 4. 需求分析与背景调查

### 4.1 授权记录

- 用户 2026-09-20（问题诊断轮）：确认 VM 桌面 020 曲库空 = 前端
  inproc 直挂、后端零供给（本会话代码实证）。
- 用户 2026-09-20（架构裁定轮）：就"画廊式静态 proxy vs proxy+动态
  加载"征询，采纳推荐——"launch 决策树四臂 + 按需 proxy session +
  窗生命周期绑定（app_key 单例+引用计数）+ 内存态前缀化"。
- 用户 2026-09-20（立项授权）："根据你的推荐，做 launch 决策树 +
  按需 proxy session 机制。请新建一个 plan 来实施它。"
- 预算/续跑限制：无特别声明（auto-plan 常规节奏）。

### 4.2 关键实证（起草期核实，行号为 2026-09-20 主检出）

| 事实 | 证据 |
|---|---|
| 020 前端空曲库兜底链 | player_store.at:92 `Http.get_json("/api/media/scan")`；catch → :99 "后端服务未启动"、:112 "本地曲库为空"、:118 "连接服务失败（端口 8320）" |
| 桌面 launch 决策树现状 | session.rs:3072 `launch_app`：spawner → native exe（:3088 `outproc_native_exe`）→ inproc 直挂（:3119 `build_dynamic_component(&spec.code)`）；:3101 `ensure_daemon_if_declared` 仅 manifest daemon 臂 |
| 解释态形态裁定 | session.rs:3073-3077 PLAN-033 注记："两合法形态 = inproc 直挂 / `-q` 经 native 臂"（re-exec 臂已退役） |
| proxy 现仅画廊挂接 | rust_ui.rs:3095-3100：`file_name()=="ui-gallery"` ‖ gallery_mode 才 `start_gallery_back_proxy`（vue.rs:6513） |
| proxy 启动期一次性注册 | back_proxy.rs:150 `start(config)`；RunningProxy 仅 `has_session`（:233），无增删口 |
| session 线程优雅退出已成立 | back_proxy.rs:753 `for req in rx`——sender 全 drop 即退出 |
| 画廊前缀化先例 | vue.rs:6046 `prefix_api_url_literals`：行级，同行含 `Http.` 且含 `"/api/` 才改写（020 语料实证单行调用） |
| session 臂谓词先例 | vue.rs:6525-6531：back api.at 含 `~Stream`/`~Promise`/行首 `use auto.` → 建_session |
| AUTO_HTTP_BASE 运行期进程 env | stdlib.rs:6766 `resolve_http_base_url` 读 env（多 app 不可用） |
| 020 后端形态 | pac.at：`api:"rust"`、back_port 8320、`media_root:"E:\Music\"`、无 `daemon:`/`desktop_exe:`；658 §5.2 裁定画廊内 020 不建 session（.at back 仅 `status()` 桩），真实曲库走宿主原生 media 路由（NativeMediaApp） |
| 020 在桌面注册表 | desktop-host gen/front/vue/src/apps/ 含 020-music-player；session.rs:5506 DashboardPin 测试锚 |
| 桌面 020 无 native exe | rust-workspace 仅 `020-music-player-back`（目录名不匹配发现序 :2945）→ 恒走 inproc |
| 关窗摘 App 站点 | session.rs:3633-3638（broker 断连）、:4150-4156（ReclaimWindow）两处 `wm_remove_win + apps.remove`；inproc CloseApp 臂路径 T-00 实证定位 |
| LaunchSpec 装配点 | session.rs:2130（code/source_path/daemon/back_root/exe/render_decl，无 media_root） |
| 658 规范文件在册 | auto-lang docs/specs/auto-lang/vm/back-proxy.md |

### 4.3 约束

- auto-lang AGENTS.md Category A：crates/ 改动必须过 `cargo t`
  （本仓工作严禁在 auto-lang 跑 `cargo t`——桌面验证经 smoke/e2e，
  lang 侧测试在 lang worktree 内跑）。
- M7-c 内存纪律：boot 零常驻、按需装载（G-3）。
- Stage B 搬迁（桌面壳 auto-lang→auto-os）在册债：本计划全部新代码
  放 auto-lang 库层，搬迁时整体带走。

## 5. 详细设计

### 5.1 launch 后端供给决策树（T-03）

新模块 `crates/auto-lang/src/ui/back_provision.rs`（或并入 session.rs，
体量定）：

```
ensure_backend(spec) -> ProvisionDecision:
  ① spec.daemon 声明          → Daemon 臂：既有 ensure_daemon_if_declared（零变化）
  ② pac media_root 声明       → Capability 臂：proxy.add_native_media(app_key, root)
  ③ back api.at 存在且特形谓词命中 → Session 臂：proxy.add_session(SessionSpec)
  ④ 其余                       → None（纯前端，零变化）
```

- app_key = App 目录名（与 658 子 URL 段同源，`/apps/<key>/api/*`）。
- ②③可叠加（020 现状：②命中；017/031 形：③命中）。
- ①独立（真进程，不进 proxy）。
- ③谓词 = vue.rs:6525-6531 同律（迁 auto-lang，auto-man 改引用）；
  普通 `#[api]` CRUD back 不建 session——inproc 合并编译 CALL 面现状
  由 T-00 实证记录（不扩大本计划范围）。

### 5.2 back_proxy 运行期增删 API（T-01）

- `ProxyShared.{sessions, native_media}` 改 `Mutex<HashMap<..>>`
  （listener 线程与宿主线程双写）。
- `RunningProxy` 新增：
  - `add_session(spec: SessionSpec) -> io::Result<()>`——spawn session
    线程（16MB 栈同 start 形）+ 插表；同 app_id 重复 add = 幂等拒绝。
  - `add_native_media(app: NativeMediaApp)`——resolve_root + 插表。
  - `remove_app(app_id)`——摘表（drop sender → session 线程退出）；
    JoinHandle 保留在 SessionHandle 供测试 join 断言退出。
  - `base_url_for(app_id) -> String`——`http://127.0.0.1:{port}/apps/{id}`
    （前缀化 root 唯一供给源）。
- listener 常驻不拆（§2）；`AUTO_BACK_PROXY_TRACE=1` 打点沿用。

### 5.3 前端 URL 内存态前缀化（T-04）

- `prefix_api_url_literals`（vue.rs:6046）迁 auto-lang（供桌面与
  auto-man 双消费；迁移 vs 复制去重方式待澄清④，默认迁移）。
- 桌面接线：`launch_app` inproc 臂，`build_dynamic_component` 前对
  `spec.code` 变换：仅当该 app 有 proxy 供给（②或③命中）且 code 含
  改写面时，`root = proxy` origin（`http://127.0.0.1:{port}`）行级改写。
  **不落盘、不改语料**；热重载重新编译同律（重编译路径同样过此变换）。
- 改写面边界沿 658 实证注记：多行调用形态（字面量与 `Http.` 不同行）
  不在改写面，语料出现时回补。
- **执行期修正（T-05 冒烟实证，d88d377f0）**：
  ① 前缀 root 为 **origin-only**——prefix 函数自拼 `/apps/{id}/api/`
  段，传 `base_url_for`（含完整子前缀）产生双重前缀 404（658 画廊
  `set_gallery_proxy_root` 同语义印证）；
  ② **改写面扩至模块文件**——Http 调用点多在 store 模块（020
  player_store.at:92）而非入口源，UI 合并编译管线 11 个读点重读同文件
  （collect_module_imports/子模块读/孙组件/visited 扫×3/adapter/
  qualifier/dir-walk）——新增 un-gated `back_prefix.rs`（进程级 launch
  作用域 overlay：PrefixSpec{front_dir,root,app_key} + RAII guard，
  build 返回即清）包全部读点，`apply` 按 front_dir 谓词过滤（back
  目录零改写 + 行级 `Http.` 规则双保险）。async HTTP 的 base 解析在
  **独立线程**做（`resolve_http_base_url` 进程 env 架构）——请求期
  per-app base 架构性不可行，源级变换为唯一同时满足 per-app 隔离与
  async 线程的机制（§2 拒绝面之外的架构实证）。

### 5.4 生命周期与关窗钩子（T-04）

- `DesktopState` 增：`back_proxy: Option<RunningProxy>`（懒启）、
  `back_refs: HashMap<String /*app_key*/, usize>`（引用计数）、
  `app_back: HashMap<AppId, String>`（AppId→app_key 归属）。
- launch：ensure_backend 命中 ②③ → 懒启 proxy（首个命中时）→
  add_* → `back_refs[key] += 1`；`app_back[app_id] = key`。
- 关窗：**三站点统一挂 `release_backend(app_id)`**：① inproc CloseApp
  命令臂——renderer.rs `execute_desktop_commands` 的 `DC::CloseWindow`
  分支（os-config close→hide 拦截之后的回收段，`state.apps.remove(&app)`
  于 renderer.rs:11478-11479）；② broker 断连站点 session.rs:3633-3635；
  ③ `HostAction::ReclaimWindow` 站点 session.rs:4153-4155。
  `release_backend`：`back_refs[key] -= 1`，归零 → `remove_app(key)` +
  清归属。outproc/daemon app 不参与（无记录，release 为 no-op）。
  **不可挂在 `wm_remove_win` 本体**——`detach_surface_to_os_window`
  表面翻转臂（renderer.rs:4217）同样调它但 App 存续，挂彼处会误卸。
- 双窗同 app：第二次 launch 命中已存在 → 计数 +1，不重复装载
  （一份后端共享，2026-09-20 会话默认裁定，待澄清②）。

### 5.5 规范增量

| delta_id | add/modify/retire | target | before/after | rationale | AC |
|---|---|---|---|---|---|
| SD-01 | modify | auto-lang docs/specs/auto-lang/vm/back-proxy.md | before：proxy 为画廊 boot 期一次性注册（start 配置全量）；after：增「运行期增删与生命周期」节——add_session/add_native_media/remove_app/base_url_for、懒启常驻、drop-sender 优雅退出、桌面供给决策树四臂引用 | 桌面复用承载层的契约面 | AC-02/03 |
| SD-02 | add | auto-os docs/plans/autos-desktop-program.md | 台账增行：desktop back provision（launch 决策树 + 按需 proxy + 窗生命周期绑定；PLAN-037） | 桌面程序机制裁定登记 | AC-01 |

### 5.6 内存与可观测

- 懒启门：boot 后无 back-proxy-listener 线程；首个供给命中才启动
  （AC-06 断言面）。session 线程名 `back-proxy-session:<id>` 沿用；
  `[back-proxy:<id>]` 日志通道沿用。

## 6. 测试设计

- **单测（auto-lang，cargo t）**：
  - back_proxy 增删：add_session 后路由 200 → remove_app 后 404；
    session 线程 join 退出断言；base_url_for 格式；同 id 重复 add 拒绝。
  - 前缀化函数：对齐 vue.rs 既有单测同型（`Http.`+`"/api/` 行改写、
    注释行/属性行不动）——迁移后 auto-man 侧测试同步改引用。
  - LaunchSpec/AppRegistryEntry：pac `media_root:` 解析、back 入口探测。
- **集成（headless session，session.rs 测试先例 :5506）**：
  launch 020（测试根合成或 examples 直引）→ proxy scan 200 + entries
  非空 → close 窗 → 路由 404 → 复 launch 重建可用；双窗计数/单卸载。
- **smoke 脚本**：`scripts/smoke-037-desktop-back.sh`（025/033 先例）：
  起 VM 桌面（headless 可行段）→ launch 020 → curl proxy scan 断言。
- **真机**：用户验收 VM 桌面 020 曲库 + 播放（ToDesk 合成输入不可用，
  真机归用户——开发机环境怪癖清单）。
- **回归门**：auto-lang `cargo t`（Category A）；画廊形态 ui-gallery
  跑一次（658 路径零漂移，尤其函数迁移后）；独立形态 020 `auto run -r vm`
  跑一次（零触及断言）。

## 7. 验收标准

- **AC-01** VM 桌面打开 020-music-player：曲库列表非空（E:\Music 实际
  条目数），条目 url 为 proxy 绝对地址，流 URL 经 proxy 可取字节
  （curl 200 + Content-Type）。验证：真机 + smoke 脚本输出。
- **AC-02** 生命周期：关闭 020 全部窗后 `GET /apps/020-music-player/
  api/media/scan` → 404/连接语义（路由摘除）；复 launch 重建可用；
  双窗同 app 仅一份后端、全关才卸。验证：集成测试 + smoke。
- **AC-03** 决策树零回归：001-helloworld launch 零变化且 proxy 不启动；
  manifest daemon 臂（apps.manifest 声明 app）ensure 链照旧。
  验证：集成测试 + 既有 daemon 单测绿。
- **AC-04** 形态零回归：独立 `auto run -r vm`（020）出曲库照旧；画廊
  ui-gallery 形态（658 三 demo）照旧出真实数据。验证：手动回归留痕。
- **AC-05** session 臂：特形谓词命中载体（合成测试根或 017 经
  AUTO_DESKTOP_APPS_EXTRA——待澄清①）经桌面 launch 可装载可服务，
  remove_app 后线程退出。验证：集成测试。
- **AC-06** 内存纪律：boot 后无 back-proxy 线程（懒启门）；首 launch
  后仅该 app 的 session/路由存在。验证：线程枚举断言（测试）+ smoke
  日志。

## 8. 执行步骤

（原子任务：精确文件路径 + 确切操作 + 验证命令；每步完成后追加
[✅ 已完成] 一行证据。工作树：lang 侧 worktree 承 auto-plan-work 节奏，
os 侧计划/台账在主检出。）

- **T-00 基线实证**（bounded investigation，产出决策记录回填 §5）：
  ① inproc app CloseApp 命令臂的 `apps.remove` 精确站点定位
  （已知锚：session.rs:3633/:4155 两站点 + shell close verb 编码 :1597）；
  ② 桌面 inproc 臂 merged CALL 现状（`use back.api:` front 是否已通，
  013 载体一试）；③ 020 桌面现状取证（"本地曲库为空" log/截图）。
  验证：回填本节 + 关窗站点行号入 5.4。
  [✅ 已完成] 2026-09-20 worktree（基 92355a5a3）：①三站点定位——
  inproc CloseApp 臂在 renderer.rs `execute_desktop_commands`
  DC::CloseWindow（`state.apps.remove` @ renderer.rs:11478-11479）+
  broker 断连 session.rs:3633-3635 + ReclaimWindow session.rs:4153-4155，
  已回填 §5.4（含 wm_remove_win 不可挂注记——detach 表面翻转臂共用）；
  ②merged CALL **已通**：`use back.api:` 经 lib.rs 模块映射
  （`back.api` → `src/back/api.at`，lib.rs:2880-2970）在
  build_dynamic_component 合并编译内解析，既有测试
  launch_three_real_apps_via_registry_resolver（app_registry.rs:1340，
  实跑 013-todo——front todo_store.at:4 即 `use back.api:` 形）master 绿
  ——证实 §5.1 裁定：普通 #[api] CRUD back 不建 session、CALL 面
  进程内已通，不扩范围；③020 前端兜底链行号核实（player_store.at:93
  `Http.get_json("/api/media/scan")` 单行且同行含 `Http.`——在 658
  前缀化改写面内；:99/:112/:118 三段兜底文案），诊断轮实证已在 §4.1。
- **T-01 back_proxy 运行期增删 API**：
  crates/auto-lang/src/back_proxy.rs（ProxyShared Mutex 化；RunningProxy
  四方法；JoinHandle 保留）+ back_proxy_tests.rs 扩展。
  验证：`cargo test -p auto-lang back_proxy`（lang worktree）绿。
  AC-02/05。
  [✅ 已完成] 2026-09-20 commit 63c41ccb6（plan-037-dev）：ProxyShared
  sessions/native_media → Mutex（SessionHandle{tx,join} 形）；四方法
  add_session（同 id AlreadyExists 幂等拒绝）/add_native_media
  （resolve_root+覆盖重插）/remove_app（双表摘除，返 JoinHandle 供测试
  join；drop sender 退出语义实证）/base_url_for（`http://127.0.0.1:
  {port}/apps/{id}`）；route_request 锁内克隆 sender 即放（reply 等待
  不持锁）。测试 +2：`http_e2e_back_proxy_runtime_add_remove_and_join_
  exit`（懒启门基态/200→404/join 干净退出/复 add 重建/重复 add 拒绝/
  base_url_for 格式）+ `http_e2e_back_proxy_runtime_native_media_add_
  remove`（未注册 404→注册 scan/stream 字节保真→摘除 404）。验证跑
  `cargo nextest run -p auto-lang --lib --features ui-iced,test-http-e2e
  back_proxy`：13/13 绿（含 658 既有 11 件零回归）。
- **T-02 注册表与谓词扩展**：
  ui/app_registry.rs（pac `media_root:`、back 入口探测进
  AppRegistryEntry）+ session.rs LaunchSpec 透传 + 谓词 fn 迁 auto-lang
  （vue.rs:6525 改引用）。验证：`cargo test -p auto-lang app_registry`
  + `cargo test -p auto-man vue` 绿。
  [✅ 已完成] 2026-09-20 commit e213d2148（plan-037-dev）：
  AppRegistryEntry/LaunchSpec 增 `media_root`（pac 原值透传，与画廊
  pac_media_root 同语义——引号剥、不反转义）+ `back_entry`
  （`src/back/api.at` is_file 探测）；boot resolver（renderer.rs）真接线
  两字段；41 处既有字面量机械补 None（括号深度扫描脚本）。新模块
  `ui/back_provision.rs`（cfg ui-iced）：`back_needs_session`（~Stream/
  ~Promise/行首 use auto. 谓词，658 迁移零语义变化）+
  `prefix_api_url_literals`（行级 Http.+`"/api/` 改写，658 迁移）+ 谓词
  矩阵/改写面单测 ×2。auto-man vue.rs 删本地实现改 `use auto_lang::ui::
  back_provision::`（待澄清④按默认裁定：迁移而非复制）。验证：
  app_registry+back_provision 27/27 绿；auto-man vue 79/79 绿。
- **T-03 桌面供给层**：
  ui/back_provision.rs（ensure_backend 四臂 + 懒启）+ DesktopState 三
  字段。验证：供给决策单测（四臂各一例）。AC-03。
  [✅ 已完成] 2026-09-20 commit 9cc15a451（plan-037-dev）：BackendPlan
  （②native_media/③session 双 Option，①daemon 不进计划④=空）+
  `plan_backend(spec, app_key)` 纯决策（app_key=launch 名，与 658 子
  URL 段同形自洽；back_entry 读失败容错为空计划）；DesktopSession 三
  方法 `ensure_backend`（懒启 `BackProxyConfig::default()` port 0、
  首装才装载、计数 +1、启动失败降级返回 None 不阻断 launch）/
  `bind_app_backend`（allocate_app 后归属绑定——ensure 在编译前、
  app_id 在编译后才存在，两段式）+ `release_backend`（计数归零
  remove_app，JoinHandle drop=分离线程；listener 常驻不拆）。DesktopState
  增 `back_proxy: Option<RunningProxy>`/`back_refs: HashMap<String,
  usize>`/`app_back: HashMap<AppId, String>` 三字段（构造初始化齐）。
  验证：`plan_backend_four_arms` 单测绿（①daemon 零计划/②capability/
  ③session/④空 + CRUD 不建 session + ②③叠加 + 幽灵入口容错）。
- **T-04 launch/关窗接线**：
  session.rs launch_app inproc 臂（ensure_backend → 前缀化 →
  build_dynamic_component；前缀化函数迁移 + auto-man 复用）+ 三站点
  release_backend 挂钩。验证：集成测试（AC-01/02/06 段）。
  [✅ 已完成] 2026-09-20 commit 4076f4d09（plan-037-dev）：launch_app
  inproc 臂序 ensure_backend → prefix_api_url_literals（内存态，不落盘）
  → build（**编译失败 release_app_key 回滚计数**——app 未诞生无窗可
  release，防泄漏）→ allocate → bind_app_backend。关窗释放挂**四站点**
  （T-00 锚三 + 实勘补第四：renderer.rs `WmCommand::Close` 标题栏 × 的
  WM 主关闭路径——漏挂则主关闭路径计数泄漏）：DC::CloseWindow 命令臂 /
  WmCommand::Close / broker 断连 / ReclaimWindow；`release_backend` 与
  `release_app_key` 共享卸载核。桌面无独立重编译路径（独立模式 500ms
  泵不属桌面域；重开窗即 relaunch 再过前缀化变换——§5.3 热重载注记
  以此兑现）。集成测试 `launch_provision_lifecycle_via_resolver` 绿：
  懒启门（boot 零 proxy）→ launch 后 scan 200 + entries + 绝对 url →
  双窗计数 2 同一 proxy → 关一窗计数 1 路由仍 200 → 全关 404 →
  复 launch 重建 200（listener 常驻复用）。回归：session 111/111 +
  desktop_protocol 36/37（coverage `imagesurface` 红为在册既红——
  stash 干净基线同红实证，非本计划引入）。
- **T-05 e2e + 回归门**：
  scripts/smoke-037-desktop-back.sh + 画廊/独立形态手动回归 +
  auto-lang `cargo t` 全量。验证：smoke 输出 + 回归留痕。AC-01..06。
  [✅ 已完成] 2026-09-20：smoke 脚本（os 侧 commit 3092bd0）实跑 PASS
  ——boot 懒启门（零 back-proxy 行）→ MCP bus launch 020 真消费臂 →
  日志抓 lazy-start 端口 → scan 200 **真实曲库 393 条目** + 条目 url
  proxy 绝对地址 → 流端 206 + Content-Type: audio/mpeg → 截图留痕
  （侧栏徽标 393——数据入 store）。**冒烟实证两修**（lang commit
  d88d377f0）：① 前缀 root 必须为 **origin-only**（base_url_for 含
  /apps/{id} 会双重前缀——[p037-http] 实证 /apps/x/apps/x/api/ 404）；
  ② Http 调用点在 **store 模块**非入口（player_store.at:92）——UI 合并
  编译管线 11 个读点重读同文件，新增 un-gated `back_prefix.rs`（launch
  作用域 overlay + RAII guard）包全部读点；async HTTP base 解析在独立
  线程（进程 env 架构）——请求期 per-app base 架构性不可行，源级变换
  为终选（§2 拒绝面之外的架构实证）。回归门：cargo t 全量 **零新增红**
  （我 72 红 ⊆ master 74 红——对拍法：checkout master -- crates/ 同
  worktree 跑，master 多出 external_config_poll flaky ×2；36 摘要红为
  master 谱系既红：musk ×3/layout 族/ui_gen ×2/coverage 等，09-19 在册
  清单外的新红代——**未归本计划**）；back_proxy 13/13
  （ui-iced,test-http-e2e）；auto-man vue 79/79；**画廊 VM 臂**照旧
  （proxy 33 apps/2 sessions——迁移谓词选出与 658 相同的 017+031 会话
  集；020 scan 真数据 + 017 chat 种子数据 + player/status 404 = 658
  语义原样）；**独立形态** 020 `auto run -r vm` 照旧（split 模式 + 自有
  8320 后端链零触及；rust-workspace Cargo.toml 成员裁剪漂移已还原——
  在册怪癖）。观察注记：020 "本地曲库为空" 系统通知 = app.at Init 与
  async 扫描时序竞态（**语料既有行为**，画廊/独立形态同律，非本计划
  回归——候选后续语料修缮）。环境事故两笔记档：①aliyun sparse 镜像
  redox_users 0.5.3 失同步——门期用 worktree 本地 .cargo/config.toml
  ustc 覆盖（已还原未提交）；②裸 stash 对误弹他会话 "026-final" 栈
  （032 期同款在册事故重演）——已按在册法恢复（对方栈完好）。
- **T-06 真机验收 + 规范收尾**：
  用户真机验收 020 曲库/播放；SD-01（back-proxy.md 增补节）+ SD-02
  （台账行）落地。验证：用户确认 + 文档 diff。
  [✅ 已完成·规范面] 2026-09-20：SD-01 = lang commit 5f88992a2
  （back-proxy.md「运行期增删与生命周期」节——四方法 API/Mutex 并发/
  桌面供给生命周期/前缀化 origin-only 与模块粒度 overlay 架构实证；
  Status 行与落地节同步）；SD-02 = autos-desktop-program.md M7 副线
  裁定交付行（PLAN-037 段，033/034 行同型）。**真机验收归用户**
  （§10.3 默认裁定 + ToDesk 合成输入不可用约束）：管道面已由
  smoke-037 承载（真曲库 393 入 store + 流 206 audio/mpeg + 截图
  留痕）；用户真机播放确认随 review 入口跟进，AC-01 真机段不视为
  已闭环——复审时若用户未确认则翻回。

依赖链：T-00 → T-01 → T-02 → T-03 → T-04 → T-05 → T-06
（T-01/T-02 可并行）。

## 9. 复审记录

- 2026-09-20 draft handoff（/auto-plan:new）：stage `new`，PLAN-037
  revision 1。outcome `pass`——授权（§4.1）覆盖全部任务；待澄清①-④
  均有默认裁定不阻塞开工（T-05 前可定①，T-04 前可定②④）。
  next: `work`。
- 2026-09-20 work handoff（/auto-plan:work）：stage `work`，PLAN-037
  revision 1。outcome `pass`——7/7 任务勾（T-00..T-06；T-06 真机段
  归用户随 review 跟进）。code_commit：lang plan-037-dev
  63c41ccb6→e213d2148→9cc15a451→4076f4d09→d88d377f0→5f88992a2
  （基 92355a5a3；组 .wt/lang-037/{auto-lang, auto-down}）+ os 侧
  8aee904/3092bd0/1f014cd（计划/smoke/台账）。
  evidence：smoke-037 真管道 PASS（boot 懒启门/scan 200 真曲库 393/
  流 206 audio/mpeg/截图徽标 393）；cargo t 全量零新增红（master
  对拍 72⊆74）；back_proxy 13/13 + back_provision 族 + auto-man vue
  79/79；画廊 VM 臂（proxy 33 apps/2 sessions 同 658 会话选择 + 020/
  017 数据面）与独立形态 020 照旧。执行期两设计修正入 §5.3（origin-
  only root；模块粒度 overlay——async 线程架构实证）。待澄清落定：
  ①=合成根集成测试承载（未注册 017；AC-05 段由 back_proxy 真语料
  017/031 测试面承载）；②=app_key 单例+引用计数（默认裁定落地，
  集成测试双窗计数实证）；④=迁移（auto-man 改引用，vue 79/79 零抖动）；
  ③=真机归用户（随 review）。blockers：无阻塞项；AC-01 真机段=
  用户确认跟进（复审未确认则翻回）。next: `review`。

- 2026-09-20 R1 review（/auto-plan:review，实施会话内复审——独立性
  限制在案，结论自工件重建：提交链/测试复跑/规范 diff，不采信执行期
  自述）。stage `review`，PLAN-037 revision 1。
  **baseline**：reviewed_commit = lang plan-037-dev 5f88992a2（基
  92355a5a3，六提交 63c41ccb6→5f88992a2，工作树净）；os 997c97d；
  依赖 auto-down fba6563ed（组内 detached）；spec 输入 =
  docs/specs/auto-lang/vm/back-proxy.md@5f88992a2（SD-01 冻结于该
  提交）。
  **acceptance_results**：AC-02 ✅（lifecycle 集成 + T-01 join 退出，
  复跑绿）；AC-03 ✅（组合证：四臂①零计划单测 + boot 懒启门断言 +
  launch_three_real_apps 真实纯 app launch 绿——013 带 CRUD back_entry
  即③负例实证）；AC-04 ✅（复审复跑画廊 VM 臂：33 apps/2 sessions +
  020 真数据 + 017 种子数据；独立形态执行期留痕）；AC-05 ✅（按待澄清
  ①落定载体：T-01 合成根 proxy 级集成 add/serve/remove/join 退出绿 +
  谓词/四臂单测；组合注记见 F-R1）；AC-06 ✅（懒启门 = back_proxy
  None 状态断言 + smoke boot 零行——技术手段等价：start() 为 listener
  唯一 spawn 点）；**AC-01 partial**——smoke 段 ✅✓（执行期+复审双复
  现：scan 200 真曲库 393/流 206 audio/mpeg/绝对 url），**真机段待
  用户确认（§10.3 归属；问询无应答——在册怪癖，按预登记"未确认则
  翻回"执行）**。
  **findings**：F-R1（P3 非阻塞改进）：ensure_backend ③臂 add_session
  组合路径无桌面 launch 级集成测试（决策单测+proxy 级集成已覆盖，缺
  组合腿）——后续计划可补；F-R2（P3 观察注记）：020"本地曲库为空"
  通知=Init/async 时序竞态（语料既有，候选语料修缮）。零阻塞 find。
  **gates**：cargo tf 全量对拍零新增红（我 2 唯一红 ui_gen::rust ×2 =
  master 既红；master 侧多 ffi_dual_019 在册 flaky）；cargo tv
  3815/3817 同两既红（master 基线复证）；定向 42/42+79/79；smoke-037
  复跑 PASS。aliyun 镜像 redox_users 失同步持续——门期本地 ustc 覆盖
  已还原（怪癖清单在案）。
  **outcome `blocked`**：命名前置 = 用户真机确认 AC-01（VM 桌面 020
  曲库+播放）。plan 回 `executing`（任务勾全保留——阻塞非任务返工）；
  确认后重入 review（代码/依赖/测试配置未变，本记录证据可复用，仅补
  AC-01 真机段）；若真机发现问题 → needs_fix/needs_replan 按面重开。
  next: 用户确认 → review 补录 → merge。

## 10. 待澄清事项

1. **session 臂验收载体**（影响 AC-05/T-05）：017-chat/031-image-viewer
   未在桌面注册表（desktop-host apps 清单无此项）。默认 = 测试根合成
   载体（AUTO_DESKTOP_APPS_EXTRA 注入临时根）；备选 = 注册 017 入桌面。
2. **双窗共享语义**（影响 5.4）：默认 app_key 单例 + 引用计数（一份
   后端共享，2026-09-20 会话建议）；备选 = 每窗独立 session。复审可翻。
3. **真机验收归属**：ToDesk 合成输入不可用（环境怪癖清单），AC-01
   真机段归用户。
4. **函数迁移 vs 复制**（5.3/T-04）：`prefix_api_url_literals` 与
   session 臂谓词默认迁 auto-lang（auto-man 改引用，依赖方向成立）；
   若迁移致 auto-man 面抖动超预期，回退为复制 + 注记去重债。
