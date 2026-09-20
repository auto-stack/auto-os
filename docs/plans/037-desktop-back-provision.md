---
plan_id: PLAN-037
status: executing               # drafting → executing → execution_done → reviewed → archived
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
current_step: 0
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
  改写面时，`root = proxy.base_url_for(app_key)` 行级改写。**不落盘、
  不改语料**；热重载重新编译同律（重编译路径同样过此变换）。
- 改写面边界沿 658 实证注记：多行调用形态（字面量与 `Http.` 不同行）
  不在改写面，语料出现时回补。

### 5.4 生命周期与关窗钩子（T-04）

- `DesktopState` 增：`back_proxy: Option<RunningProxy>`（懒启）、
  `back_refs: HashMap<String /*app_key*/, usize>`（引用计数）、
  `app_back: HashMap<AppId, String>`（AppId→app_key 归属）。
- launch：ensure_backend 命中 ②③ → 懒启 proxy（首个命中时）→
  add_* → `back_refs[key] += 1`；`app_back[app_id] = key`。
- 关窗：inproc CloseApp 臂（T-00 定位）与 3633/4155 两站点统一挂
  `release_backend(app_id)`：`back_refs[key] -= 1`，归零 →
  `remove_app(key)` + 清归属。outproc/daemon app 不参与（无记录，
  release 为 no-op）。
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
- **T-01 back_proxy 运行期增删 API**：
  crates/auto-lang/src/back_proxy.rs（ProxyShared Mutex 化；RunningProxy
  四方法；JoinHandle 保留）+ back_proxy_tests.rs 扩展。
  验证：`cargo test -p auto-lang back_proxy`（lang worktree）绿。
  AC-02/05。
- **T-02 注册表与谓词扩展**：
  ui/app_registry.rs（pac `media_root:`、back 入口探测进
  AppRegistryEntry）+ session.rs LaunchSpec 透传 + 谓词 fn 迁 auto-lang
  （vue.rs:6525 改引用）。验证：`cargo test -p auto-lang app_registry`
  + `cargo test -p auto-man vue` 绿。
- **T-03 桌面供给层**：
  ui/back_provision.rs（ensure_backend 四臂 + 懒启）+ DesktopState 三
  字段。验证：供给决策单测（四臂各一例）。AC-03。
- **T-04 launch/关窗接线**：
  session.rs launch_app inproc 臂（ensure_backend → 前缀化 →
  build_dynamic_component；前缀化函数迁移 + auto-man 复用）+ 三站点
  release_backend 挂钩。验证：集成测试（AC-01/02/06 段）。
- **T-05 e2e + 回归门**：
  scripts/smoke-037-desktop-back.sh + 画廊/独立形态手动回归 +
  auto-lang `cargo t` 全量。验证：smoke 输出 + 回归留痕。AC-01..06。
- **T-06 真机验收 + 规范收尾**：
  用户真机验收 020 曲库/播放；SD-01（back-proxy.md 增补节）+ SD-02
  （台账行）落地。验证：用户确认 + 文档 diff。

依赖链：T-00 → T-01 → T-02 → T-03 → T-04 → T-05 → T-06
（T-01/T-02 可并行）。

## 9. 复审记录

- 2026-09-20 draft handoff（/auto-plan:new）：stage `new`，PLAN-037
  revision 1。outcome `pass`——授权（§4.1）覆盖全部任务；待澄清①-④
  均有默认裁定不阻塞开工（T-05 前可定①，T-04 前可定②④）。
  next: `work`。

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
