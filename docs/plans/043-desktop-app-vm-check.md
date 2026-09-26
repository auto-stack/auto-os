---
plan_id: PLAN-043
status: reviewed               # drafting → executing → execution_done → reviewed → archived
                               # （2026-09-25 会话收口：Part 1+2 merge、桌面终验、
                               #   Phase 3/4、027/018/017/ui-gallery/036/037/
                               #   jade-edit/图标/壁纸/主题/围栏全空等本轮全部
                               #   修复与记录收口；剩余项=下方遗留清单+待澄清，
                               #   移交下个会话的新计划跟踪——见"移交下会话清单"）
                               # （2026-09-27 全计划收口复审 pass——见复审记录末条）
feature_name: desktop-app-vm-check
author: []
created_at: 2026-09-23
updated_at: 2026-09-24

# /auto-plan:review 结束时填写：
supersedes_spec_components: []
new_spec_components: [P043-1, P043-2, P043-R1, P043-3, P043-4, P043-R2]   # 1/2/R1 已沉积；
                               # 3/4（launch 字段机制/分主题壁纸双槽）+R2（收口复审
                               # 收据）2026-09-27 merge 沉积
touched_goals: []             # 引用 docs/specs/goals.md 的 GOAL-NNN

affects: [auto-lang/examples/ui, auto-lang/crates/ui, auto-lang/crates/auto-man, apps/]
current_step: 2
total_steps: 4
---

# [PLAN-043] desktop-app-vm-check

虚拟桌面（VM 轨，ui_desktop）各 app 打开情况检查与修复跟踪。
2026-09-23 实机走查启动，每个 app 一个 Part 记录：现象 → 根因 → 修复 → 验证。

## 走查环境

- 桌面宿主：`D:/autostack/auto-lang/target/debug/examples/ui_desktop.exe`，
  CWD=`D:/autostack/auto-os`，`AUTO_OS_ROOT`=本仓根，
  `AUTO_VM_STORAGE_FILE`=隔离档案（`tmp/desktop-vm-check/storage.json`），
  `AUTOUI_ACCEPTANCE=1`，MCP `:9471`。
- boot 注册表：43 entries / 29 desktop-visible（主根 examples/ui opt-in +
  容器臂 apps/* + manifest repo 臂 auto-kanban/auto-musk/jade-garden/auto-term
  + os-config + 双画廊，去重后口径）。
- 驱动：MCP acceptance 通道（bus 动词 `launch\t<id>` / `close\t<wid>` +
  `autoui_screenshot` 留痕）。

## 变更摘要

## 目标

## Part 1：029-photo-gallery 照片图库——展示内容非实时扫描（已定位根因）

**现象（用户报告）**：图库展示的照片集合与预览不是最新的；照片目录
（`C:\Users\zhaop\Pictures`）内容已变化，图库显示不变。

**根因（已证实）**：该 app **没有任何后端**，运行期零目录扫描。照片集是
2026-09-14 一次性离线烘焙进前端的死数据：

- 流水线（`examples/ui/029-photo-gallery/scripts/`）：
  1. `prepare_gallery.py`：扫一遍 `C:\Users\zhaop\Pictures`（+ Screenshots
     子目录前 15 张）→ 生成 260px JPEG 缩略图 + `gallery_data.json`
     （元数据 + base64 缩略图，821KB，9/14 16:43）。
  2. `generate_at.py`：读 json 生成 `var p_ids/p_titles/p_albums/p_dates/
     p_fulls/p_thumbs...` 数组源码。
  3. `assemble_app.py`：`app_head.at` + 数组 + 固定尾巴 → `src/front/app.at`。
- `src/front/app.at` 模型里硬编码 24 张（注释写"68 张"也是假的）；
  `p_thumbs` = 烘焙缩略图**绝对路径**；`p_fulls` = 原图绝对路径
  （`C:/Users/zhaop/Pictures/...`——原图还活着所以大图能开，但集合
  本身冻结在 9/14）。
- **脚本路径还指向已删除的 worktree** `D:\autostack\.wt\lang-628\...`
  （lang-628 组已折叠）——烘焙链在主检出上重跑会直接失败；主检出
  `app.at` 里的缩略图路径已被手工改写为主检出路径，与生成器脱节。
- pac.at 注释仍写 picsum.photos 固定 seed（Plan 537 旧描述），与
  Plan 628 改写后的本地照片实现不符。

**修复方向（2026-09-23 用户裁定）：A——加真后端，HTTP 供给图片素材**
（浏览器/Vue 端不能读本地文件，缩略图与原图一律走后端 HTTP）。

### Part 1 详细设计（方向 A 定案）

架构 = **复刻 Plan 617 `media_root` 能力臂模式**，做照片专用的
`photo_root` 平行件（不发 app 级 `src/back`，框架能力臂随 pac 声明激活，
与 020 同构）：

1. **`crates/auto-lang/src/ui/photo_service.rs`（新模块）**——镜像
   `media_service.rs` 形态：递归索引（jpg/jpeg/png/webp，symlink 跳过、
   深度帽）、blake3 token id（绝对路径不出后端）、自然排序；增量字段
   `width/height`（image::image_dimensions 头解析）、`date`（mtime）、
   `album`（rel_dir 首段，根文件归 "photos"）。缩略图端点语义：
   `thumb/{id}?w=260` 按需生成（`image` crate 解码→EXIF 朝向规范化
   （kamadak-exif，image-pipeline 既有依赖）→imageops 变换→thumbnail
   →JPEG），磁盘缓存 `%TEMP%/autoos-photo-thumbs/<id>-<mtime>-<w>.jpg`
   （mtime 参与缓存键，目录变化自动失效）；`full/{id}` 原图字节 +
   image content-type。整体 `cfg(feature = "image-pipeline")` 门控
   （恰为该特性语义；宿主 ui-iced 与生成后端均已含）。
2. **`crates/auto-man/src/pac.rs`**：`photo_root: "C:\\Users\\zhaop\\Pictures"`
   解析入 pac config（media_root 同款单源）。
3. **`crates/auto-man/src/api_gen.rs`**：pac 含 photo_root → 生成后端
   发射 `PHOTO_SERVICE_HANDLERS`（`/api/photos/scan|thumb/{id}|full/{id}`，
   axum 形态镜像 MEDIA_SERVICE_HANDLERS）；**scan 的 url 字段发绝对地址**
   `http://127.0.0.1:{back_port}/api/photos/...`（VM native 渲染器
   `load_image_bytes` 对非 http src 走本地文件臂，相对 URL 不可用——
   020 能用相对值是因 mpv 契约特判，image widget 无此臂；Vue 端绝对值
   经 dev proxy 照常工作）。
4. **`crates/auto-lang/src/back_proxy.rs`**：桌面懒启 proxy 增
   `try_native_photos` 原生路由（`/apps/<id>/api/photos/*`，绝对 base
   镜像 media_scan 的 §5.3 裁定）。
5. **`crates/auto-lang/src/ui/back_provision.rs`**：能力判定/plan 增
   photo_root 臂（media_root 平行）。
6. **app 改写**：`pac.at` 删烘焙痕迹、加 `photo_root:`；`src/front/app.at`
   删全部 p_* 硬编码数组，Init `Http.get_json("/api/photos/scan")` 拉取，
   photos/view_list 由响应构建；相册侧栏按响应 album 动态分组（不再写死
   photos/screenshots 两段）；favorites 落 app storage（重开保留）。
   烘焙三脚本（prepare_gallery/generate_at/assemble_app）删除。
7. **安全边界**：sd 同 media_service——后端只出 token，路径永不序列化；
   thumb/full 按 token 反查索引，越界请求 404。

**验收（修复后）**：
- 独立形态 `auto run -r vm 029-photo-gallery`：scan 200、条目数 =
  当前目录实数、缩略图/原图 HTTP 200 + Content-Type 正确；
  目录增删照片 → 重启 app 后集合跟随。
- 桌面形态：launch 图库（懒启 proxy 臂）→ 网格实渲染、大图可开、
  MCP 截图留痕。
- 回归：020 的 media 路由零触碰（新模块平行，不并 SUPPORTED_EXTENSIONS）；
  `cargo check -p auto-lang` + 作用域测试过门。

**执行仓/分支**：auto-lang 侧改动（crates + examples/ui/029）走
`D:/autostack/.wt/lang-043/auto-lang`（分支 plan-043-dev，Plan 529 组
布局）；计划簿记本文件留 auto-os main。跨仓互链按 AGENTS §1。

## 执行步骤

- [✅ 已完成] 2026-09-23 走查启动：boot 43/29，MCP :9471 驱动臂验证可用。
- [✅ 已完成] Part 1 根因定位（烘焙流水线实证，见上）。
- [✅ 已完成] Part 1 设计定案（方向 A；photo_root 能力臂五件 + app 改写，
  见「Part 1 详细设计」）。
- [✅ 已完成] T1 lang worktree `.wt/lang-043/auto-lang`（plan-043-dev）+
  组兄弟 `.wt/lang-043/auto-down`（autodown-core path 依赖，纯检出零改动）。
- [✅ 已完成] T2 photo_service.rs 新模块（索引 + thumb 磁盘缓存 + full
  流 + EXIF 朝向；8/8 单测绿）。
- [✅ 已完成] T3 pac.rs photo_root 解析 + api_gen.rs PHOTO_SERVICE_HANDLERS
  发射（scan/thumb/full 三路由，绝对 URL）+ AUTO_PHOTO_ROOT env 注入
  （main.rs）+ 注册表/LaunchSpec photo_root 透传全链。
- [✅ 已完成] T4 back_proxy try_native_photos（镜像 media 臂）+
  back_provision plan 臂（native_photos）；back_proxy e2e 33/34 绿
  （1 个重跑即过，真 TCP 环境抖动；photo_service/app_registry/api_gen
  作用域测试绿）。
- [✅ 已完成] T5 app 改写：pac.at（photo_root + api/back_port）+
  最小 src/back/api.at（status 控制面，后端进程因它而在）+
  app.at 全量重写（Init 拉 scan 建网格 / 动态相册分组 / 收藏落
  Storage `photo-gallery.favs`）+ 烘焙脚本/数据/缩略图删除 + SPEC.md
  重写。
- [✅ 已完成] T6 收尾（独立形态全过）：Part 2 修复后网格实渲染截图
  （迷你根 3 张 + 真实根实机照片）、查看器原图/上下张环绕、收藏跨
  会话恢复、目录增删跟随（3→4 张实证）、加载更多分批；020 对照
  item_count=393 恢复（框架修复红利，020 仅改配方）。
- [✅ 已完成] **导航模型修订**（2026-09-24 用户裁定：非递归）：
  `index_directory`（递归平铺）→ `list_directory`（只列当前目录 +
  子目录导航条目带直属计数）；`?dir=` 相对段 `sanitize_rel_dir` 拒
  穿越/盘符/反斜杠；token→路径注册表按 root 分桶；thumb/full 走
  token 反查。前端：页签=全部/收藏，子目录 chips 导航 + GoUp 返回，
  收藏改全记录 JSON 往返（跨目录聚合）。独立形态实机：根 15 直属 +
  7 子目录 chips、Screenshots 339 导航/返回全链。photo_service 10/10
  单测重写绿；提交 lang worktree `plan-043-dev`。
- [✅ 已完成] **分批渲染闸**（性能边界）：VM native image 同步阻塞
  加载实测 412 张冻死 UI——网格只画前 60 张 +「加载更多」分批
  （render_cap=60，过滤/排序/导航归位）。lang 层异步图片加载记为
  后续债。
- [✅ 已完成] **桌面形态终验**（2026-09-24 补验，主检出重建 ui_desktop
  @lang master 5af53ff3f）：MCP acceptance 召唤 029 → back-proxy 懒启
  （:3358）→ 窗口明示「后端实时扫描 · 根目录」，15 项真实内容 + 真实
  相册分组（PixPin 6 / Saved Pictures 0 等），截图留痕
  `tmp/autoui-screenshot-1790227582516.png`。原"worktree 重编译"路径
  已无必要（代码已在 master，主检出构建等价）。
- [✅ 已完成] Phase 3（分主题壁纸）：双槽+迁移+SetTheme/SetWallpaper 臂
  落地（lang 5ff4646fc），19/19 单测 + 实机深浅切换跟随全过——深槽=
  剑士图、浅槽=songyu.png（用户钦定值已设）。
- [✅ 已完成] Phase 4（图标列主序）：分析修正（自由填充本已列主序；
  真因=apply_drop 补位臂行主序分裂 + 底稿遗产数据）——apply_drop 对
  齐 + 清 14 个 positions 键，实机列主序截图过。

## Phase 2 执行记录（get_json 回归修复，2026-09-24 收口）

**根因（实锤）**：PLAN-080 F-2③（a4d48faa6，09-21）把 UI 路径裸
`Http.get_json` 编译期改写为 `auto.http.get_json` + `auto.json.to_value`
（返回**解析产物**而非 body 字符串）——旧配方 `json.to_value(Http.get_json(..))`
（stdlib 注释层文档化、020/029/030 在用）变成双重解析：内层已产
`__json_object`，外层 `shim_json_to_value` 走 `String` 弹栈的
`{:?}` 兜底 → **NanoValue 调试串（~20 位数字）**，`json.to_value` 解析
为 number → `data.entries` 恒空 → 静默空态。020（零改动 app）同症状
实证为框架回归；桌面形态同中招（今早主检出构建 boot 即报曲库空）。

**修复**：① `shim_json_to_value` 幂等臂——object/list/bool/null 实参
原样透传（stdlib.rs，注释全链）；② 020/029/030 前端配方同步修订
（去冗余外包）；③ p080 测试家族补幂等回归钉
（`json_to_value_idempotent_on_parsed_object`）；④  stale 配方注释
（shim_http_get 文档块）修订。

**验证**：新钉过；p080 家族 5/5；json 族 63/63；020 独立形态
item_count=393 恢复；029 端到端全链绿。th 档 back_proxy 17 红 =
Hyper-V 端口保留段轮转覆盖硬编码测试端口（干净树同败，环境性）。

## Part 2：split 形态 VM 前端 `Http.get_json` 返回解码坏（预存框架回归）——
## ✅ 2026-09-24 Phase 2 已修复收口（见「Phase 2 执行记录」）

**~~处置（待用户裁定）~~ → 用户裁定：本计划内立 Phase 2 修复（已收口）**。
原分析存档（现象/影响面/证据链）保留如下——根因最终定位与初判
（Plan 027 内存修复窗口）不同：**真凶是 PLAN-080 F-2③ 的编译期改写
变更了 get_json 返回协议**（字符串→解析值），旧配方双重解析触发
`{:?}` 兜底。初判证据链中"回归窗口 09-21/09-22"为误导（080 同处
09-21，a4d48faa6 在窗口内但方向不同），最终以 Phase 2 执行记录为准。

**现象（已解决）**：app 前端旧配方拿到的返回值是 20 字符十进制数
（NanoValue 调试串），`data.entries` 恒空、静默无错。curl 直查同端点
数据完整。020（零改动 app）同症状双形态中招。

- [ ] 继续走查其余 app（027-file-manager、030-video-player、031-image-viewer、
  031-paint、036-tetris、037-klondike、038-minesweeper、041-auto-edit、
  auto(os-config)、ui-gallery、widgets-gallery、kanban、auto-musk、
  jade-garden、auto-term）——每个 app 补一个 Part。

## 走查中已发现的其它问题（待逐 app 立 Part）

- **[宿主 panic（最高优先）]** 连续启动 app 过程中宿主进程崩溃：
  `iced_widget-0.14.2 container.rs:291 Option::unwrap()` on None，
  桌面整体死亡（MCP 断连）。崩溃前最后 launch 的是 026-database
  （该 app 有 use 模块解析失败：use 模块 `{ package` 解析失败——多行
  import 语法被 P-15 单行星系解析打断，"引用其符号的面将落空"）。
  疑似坏 app 的残缺 view 触发宿主布局 unwrap。待：最小复现 + 宿主
  渲染兜底（panic 边界不应杀死整个桌面）。
- **017-chat**：launch 解析失败，弹「应用暂不可用 无法启动: 017-chat」。
- **018-book-reader**：✅ **2026-09-24 修复收口**（lang master
  4dc4d581a）。两层根因：①front 三文件 `use back.api` 直接导入 VM 轨
  P-15 静默跳过 → Init 调 undefined → VmBridge init 崩 → 无法启动——
  改 HTTP 配方（book_store/book_detail/reading 全部
  Http.get_json/post/put/delete 打既有 #[api] 端点）；②CRUD back 不建
  proxy session（back_needs_session 旧判据只认 ~Stream/~Promise/
  use auto.，"inproc CALL 面已通"在桌面轨失效）——谓词增 `#[api]`
  命中，CRUD back 一律 proxy session 供给。实测：launch → lazy-start
  :3358 → Library 真数据渲染（Rivers of Time 33% 书卡）。
  小观察：书架计数 f-string `.len()` 渲染空（cosmetic，在账）。
  **✅ 2026-09-25 UI 修正**（用户反馈书架单列满宽 + Continue 横幅占屏）：
  书架改原生 grid 部件 `cols:3`（Tailwind 响应式类 md:/lg: 原生渲染器
  不吃 → 塌单列；原生 grid cols 支持状态表达式绑定，宽度信号注入后可
  升级自适应 2/3 列）；Continue Reading 大横幅降级标题栏按钮（▶ 书名 ·
  进度）；书架计数 f-string len() 渲染空改 book_count 管线；头两按钮
  whitespace-nowrap。实机截图三列真数据过（lang master 后续提交）。
- **017-chat**：解析层已修（chat_store/app.at 同款 HTTP 改造，@4dc4d581a）
  ——但 launch 即崩桌面：iced_widget container.rs:291 unwrap
  （**P041-D1 债本尊，今日围栏放行实验复现**，front build/back 供给全过、
  崩在布局竞态）。**✅ 2026-09-25 根治收口**：iced_widget 本地补丁
  （`patches/iced_widget`，[patch.crates-io] 挂载）——container.rs 五处
  operate/update/mouse_interaction/draw/overlay 的布局空窗 unwrap 改良性
  降级（至多丢一拍操作/绘制，不崩进程）。017 摘围栏实机验证：双实例并
  开 + 秒表计时 + back HTTP 数据全通 + 零 panic。auto-term 留围栏观察
  （无独立实证）。宿主 panic 族（最高优先遗留）获同款缓解——其余部件
  同型 unwrap ~25 处未补，再现同型崩溃按需扩展（patches/README.md）。
- **auto-term**：✅ **2026-09-25 修复收口**：围栏放行实验发现其走
  **outproc-native**（有 desktop_exe——PLAN-041 围栏自始拦的其实是原生
  exe 形态）；spawn 失败两层：①exe 为 9/21 旧协议构建（PLAN-693/694
  desktop 方言之前）→ 附着握手不认识即退；②auto-term/app/rust-workspace
  的 app Cargo.toml 残留 ui-gpui 声明（PLAN-691 退役家族第三例，a2r
  重生成前置的 resolve 即拒）。修复：摘 ui-gpui 声明（rust-workspace 系
  gitignore 生成物，盘上修法即归属；脚手架模板已随 PLAN-691 清理）+
  app 根 `auto build -r rust` 再生成 main.rs（含 View::Terminal history
  字段——a2r 模板已跟上）+ 重编 exe。实测：outproc-native 附着 rqhost
  合成器，**活的 cmd 会话渲染在桌面内**（版本横幅+提示符+分屏工具栏，
  截图在案）。**围栏全空**（VM_LAUNCH_FENCE=[]：017 iced 补丁 +
  auto-term 协议对齐双双根治；若再现旧崩形按需恢复围栏）。
- **per-app 主启动方式声明（2026-09-26 用户需求落地，lang 1d597e3f4）**：
  apps.manifest 条目增 `launch` 字段（"vm"|"native"；缺席=既有 exe 存在
  即原生的向后兼容）——manifest_launch_lookup 单名查表（daemon lookup
  同款独立读取），launch_app 门按声明分派：声明 vm 的 app 恒 inproc
  解释（编译产物存在也不走原生附着）。本版 10 条目全声明 vm；tetris
  条目 id 顺修 036-tetris（manifest id 对齐注册表 id）。实测：tetris/
  klondike inproc VM 解释渲染可玩（.at 游戏 UI 桌面内完整）。
- **boot 直挂计算器退役 + 围栏全空（lang 提交链 70bcd5246→86b56884b）**：
  ui_desktop 入口的 011 boot 直挂组件装配拆除（用户报告"每次启动都开
  计算器"——无 autostart 机制，是入口硬编码直挂）+ run_dynamic_iced_multi
  空组件表守卫降级日志（boot 窗全部经注册表 launch 链）。
- **跨会话教训（2026-09-25）**：共享主检出多会话并行期间，`taskkill /IM
  auto.exe` 类按映像名清扫会误杀他会话的构建/运行进程（PLAN-701 T-01
  F-RV6 根因勘定在案：其会话基线 2/5 与下游同率、watcher 实捕同秒双进程
  蒸发=外部 TerminateProcess 形）——并行期清扫一律 /PID 精确制导
  （tools/perf/README.md:65 指导在案）。
  **✅ 2026-09-25 续：按用户裁定切回 VM 集成形态**（"本版本只做 VM
  集成；RQHost 下个版本"）——约定产物（target/debug/auto-term.exe）
  摘除 → convention 落空 → launch 落回 inproc VM 解释渲染。实测：
  **工具栏干净渲染（无乱码——乱码是 outproc 原生 exe 形态的图标字体
  缺口，随 RQHost 形态退场）**；**shell 面板空**（P041-D1 崩溃签名
  精确复现：handler_App_Init/Tick `future_all/race: empty or invalid
  future list` 每拍崩——back 模块合并成功（mux_/term_ 符号全在），
  崩在 mux tick 的 future 组合器空列表 = 引擎链（autoterm_core.dll
  已部署宿主 exe 同目录 → mux_init → PTY spawn → 内容馈送）在桌面
  内嵌 VM 形态未驱动起来，与 PLAN-041"渲染但永不 tick"同根）。
  **剩余工程**：引擎 DLL 链诊断（dll 加载确认/PTY spawn 失败点/
  mux future 列表构建）——auto-term VM 集成的核心缺口，独立立项；
  终端窗暂不可用（空面板），建议关闭该窗至引擎链修复（每秒 Tick
  错误日志有量）。
- **ui-gallery**：✅ **2026-09-25 修复收口**（根因=0922 遗留未提交 WIP）：
  主工作树里 ui-gallery 的 d027 flat-module 重构 WIP（0922 19:22 遗留，
  三天未动）用了 **handler 直调语句**（`.NavTo(h)`/`.NavTo(up2)`）——VM
  链接器不解析 handler 间直调符号 → `link failed: Undefined symbol:
  handler_Demo027FileManager_NavTo` → 无法启动。处置：整块 WIP 带标签
  stash 保全（`ui-gallery d027 flat-module WIP…恢复 git stash pop`），
  画廊回 HEAD 态——实测构建过、窗口全 UI 渲染（demo 列表+内嵌视口）。
  WIP 恢复前提：handler 直调语法需先上 VM 链接支持（新符号面，lang 侧
  待议）。widgets-gallery 不涉（独立目录，本就正常）。
- **ui-gallery / jade-edit / boot 直挂退役（2026-09-25 会话伴随改动）**：
  ①桌面"玉圃"位改开 **jade-edit**（../jade-edit，PLAN-081 单工程双轨）
  ——apps.manifest 注册 repo 臂（ports [4181]）+ README 表同步 + icons
  白名单 jade-garden→jade-edit 换位；实机 launch 即开（vm 解释渲染，
  26 actions/8 toolbar，back 会话经 CRUD 供给懒启，文件树/工作区全活）。
  **submodule 收编候选在案**（用户：将来 git submodule 引入
  apps/jade-edit，kanban 先例）。jade-garden 旧版仍在 manifest/launcher
  未裁撤。②**boot 直挂计算器退役**（lang 70bcd5246）：ui_desktop 入口
  原把 011-calculator 作 boot 直挂组件装配（APP_B include_str!，用户
  报告"每次启动都开计算器"）——拆除 + run_dynamic_iced_multi 空表守卫
  降级为日志；boot 窗全部经注册表 launch 链。③020-music-player 的
  SwitchMode handler not found（按钮点了没反应）——观察面待查。
- **025-sys-monitor 表格桌面内嵌不渲染（2026-09-25 深挖未决，最高优先
  之一）**：桌面内嵌形态进程表**表头+计数徽标渲染、行区全空**（用户
  截图 286/实测 249-257）；**独立 `auto run -r vm` 正常**。证据链：
  ①back 侧无虞——会话 back 经 proxy 直拉
  `/apps/025-sys-monitor/api/system/snapshot` HTTP 200，**procs 257 条
  全字段**（curl 实证）；②summary 链活——KPI 卡实时真值
  （procCount=257 来自 snap.summary.proc_count，与 procs 同源同对象）；
  ③front 数据两种提取形态均失败：原 `use back.api` 直导（跨 VM 句柄，
  疑调用栈弹出即回收→句柄悬垂→for 迭代空；独立单 VM 无跨桥所以正常
  ——musk [VM-IDX] no-heap-object 同族）与 HTTP 配方
  （Http.get_json + ?? [] 提取 + while 索引物化，029 验证形态）均
  行区空；④排序状态（storage sysmon.sort_dir=asc 遗留）重置 cpu/desc
  走直拷分支亦空——嵌套 .Sort 调用假设排除；⑤HTTP 配方破坏独立形态
  （独立 VM 轨相对 /api 无路由 → backend_ok=false + Refresh 阻塞），
  **已回滚**至 use back.api 原形态（独立可用；桌面回到已知症状）。
  **剩余 suspects**：(a) 桌面内嵌解释器对 HTTP JSON 值的字段/迭代语义
  （029 同配方在 029 桌面内嵌却工作——差异待查：029 数据经 photo
  native 臂 vs 025 经 lazy session）；(b) table-row 部件在内嵌 vwin
  的渲染（行高塌陷/裁剪）；(c) 跨 VM 句柄返回值 stake/物化（call_vm_fn
  返回侧无 stake——PLAN-053 只修了参数侧）。**诊断工具缺口**：MCP
  vtree/state 对 app 窗口内容全盲（只读桌面壳层，最小化时全盲）——
  app 窗口状态检查通道需补（否则此类问题只能盲诊）。025 独立运行的
  Tick 预算告警（handler_App_Tick busy 2s）为既有性能面（249 行
  HTTP+重排每秒）非本缺陷。
- **036-tetris / 037-klondike**：✅ **2026-09-25 修复收口**（两层根因）。
  ①**桌面实例缺 rqhost 端点**（主根因）：outproc-native 原生 exe 经
  `--desktop-endpoint` 附着 rqhost 合成器渲染——直启 exe 不带
  `AUTO_DESKTOP_RQHOST=1`（desktop.ps1 标准入口默认开）时 spawn 即退
  （进程无窗）；带端点重启后双游戏 outproc-native 附着渲染全通（纸牌
  接龙全屏截图在案）。②**构建层三件**（盘上修，rust-workspace 系
  gitignore 生成物）：tetris workspace 摆脱已退役 ui-gpui feature 声明
  （PLAN-691 后 resolve 即拒）+ front aria-label 摘除（a2r 词表门）+
  main.rs 按现行模板再生（ClientOpts 增 remote 字段）；klondike front
  ondblclick 摘除（VM/native 走 store「再点一次」路径零损失）+ 生成
  main.rs bool 条件位 as_bool 直修 + Cargo.lock hyper-util 降 0.1.20
  （aliyun 镜像缺 0.1.21）。auto-kanban 桌面图标修复：白名单 id 写了
  目录名 auto-kanban，而 manifest 臂注册 id=manifest id `kanban`（映射
  表键原生在）——id 对不上整条查询落空；白名单改正后位图+正式名
  "Kanban" 渲染 ✓（icon_file 临时诊断已还原）。
  **lang 侧记账债**：a2r 视图 if 条件位对 record 字段访问按字符串降链
  （应 bool→as_bool）——codegen 根修待立项；另旧 exe（9/14 构建）与
  新桌面二进制合成协议不匹配（进程活窗不出），原生 exe 需随桌面协议
  重建。
  **✅ 2026-09-25 17:53 修正（rqhost 端点脱钩）**：用户裁定本版本纯
  VM 加载（AUTO_DESKTOP_RQHOST 不设）后实测——**tetris/klondike 照常
  渲染**：原生 exe 走 Plan 020 血统的 **SHM 像素桥**（协议稳定，与
  rqhost 端点无关），此前"缺端点致游戏打不开"的判断有误（该端点服务
  的是更新的 desktop 方言路线，非游戏所需）。VM-only 桌面终态：全部
  .at app + 双游戏 + 025 摘要（行区债在账）正常。
  **❌ 0925 深挖定位：klondike 原生 exe 明牌牌面全空**（白底无点数
  无花色；VM 解释形态正常）。vtree 实证：明牌容器/rank 文本/花色在
  渲染树中 **0 出现**，视图渲染的是七个空列占位——**发牌只落了 down
  计数（col0..6_down=0..6 ✓）与 stock=24，七个列数组在视图期为空**：
  a2r 生成 store 的发牌数组填充静默失败（down 计数同源赋值却生效——
  半途断裂形态）。suspect=a2r store codegen 的数组 push 落地（auto-man
  rust_ui 生成链）——lang/auto-man 深水债，独立立项。附着形态本身无恙
  （SHM 桥照常出图：牌背/卡底/工具栏全渲染）。
- **027-file-manager**（2026-09-24 本会话实机两次复现 + 行级定位）：
  VM 轨 boot **fatal**（plan-446 C1 起 fatal at boot）——
  `components/tree_icon.at` 头部 `use stylekit.styles: icon_base` 在
  **组件文件**里解析失败 → `icon_base` undefined（报错 17:53 实落第 18 行
  `style: icon_base` 引用处，解析器行号偏 1），后续 19 个
  `Expected term, got RBrace` 全为级联误报。**关键鉴别证据**：同款
  `use stylekit.styles: <item>` 在 app.at（029/031 实测能开）解析正常、
  组件文件失败——机制嫌疑=组件文件的 use 解析上下文（与 024/026 的
  `{ package` 块形同族）。连坐面：018 的 tree_icon.at 与 027 字节级同
  md5；024-charts 四个图表组件带同款 use（高危未测）。修法模板：026
  拷贝已内联 class 字符串（无 use 行）；四胞胎拷贝（018/026/027）需
  同步。vue 轨不受影响（用户实机 027 正常）。
  **2026-09-25 更新：027 实测可启动**（用户实机 launch 无 fatal 无
  toast——stylekit 组件 use 解析链在此期间的解释器演进中恢复，018 的
  tree_icon 同源 use 亦不再 fatal）。029/031-image-viewer/031-paint 亦
  实测可启动（031-paint 首次走查通过）。
- **024-charts**：use `{ package: ... from "components" }` 多行形解析失败
  （页面仍出，但引用的符号面落空）。
- **016-calendar**：use `datetime` 模块解析失败（静默跳过）+ `flex-wrap`
  native 降级（Plan 412 在案）。
- **launcher 覆盖层**：summon 后 launcher 覆盖层长时间驻留，bus Esc/
  重 summon 未关闭；启动 app 的窗口开在其后（z 序/聚焦问题待查）。
- **Windows 端口保留段（环境）**：Hyper-V 保留 8251-8850 等段——020
  pac `back_port: 8320` 恰在段内，独立形态本机起不来（8429 实测
  PermissionDenied）；029 已改 4429（4776 以下空闲）。020 的独立形态
  验证需 `-B` 覆盖或 pac 改端口（待澄清，可能与 Part 2 修复同批做）。
- **桌面快捷方式白名单（2026-09-24 三轮裁定，终态 29 = launcher 镜像）**：
  用户裁定**桌面 = launcher 列表**。权威口径（app_registry 实证）：主根
  examples/ui 为策展 C 档 20（PLAN-552 清单，demo 001/003/004 从来不在
  其中——此前手工枚举误判其 visible）；外部根缺省 visible（apps 容器
  036/037/039 + manifest 四仓含 auto-kanban + os-config + 双画廊
  ui-gallery/widgets-gallery）= 30；launcher 注入再滤 `-launcher` 后缀
  （028 自身不入列）= 29。桌面终态 = launcher 29：去 demo 三件与启动器
  自身，补 ui-gallery/widgets-gallery/auto-kanban，实机截图全过
  （auto-kanban 无图标素材回退 lucide 属预存语义）。语义级"新 app 自动
  上榜（= launcher 列表）"仍属 DEFAULT_DESKTOP_ICONS registry 驱动候选。
  **执行期教训**：`desktop-storage.json` 仍是 load-once + 整文件覆盖写
  （PLAN-044 键级合并只护 config.at）——外部改写必须在无桌面实例运行
  时做，否则被运行实例内存态整体写回（本轮实证一次）。

## Phase 3：分主题壁纸——深/浅主题各记一张（2026-09-24 用户并入；✅ 同日执行收口）

**需求（用户裁定，09-24 两轮修正）**：深色主题壁纸 =
`D:/Down/stella-os/wallpapers/purple.png`（用户经设置 picker 自选；
执行期曾被执行臂误覆盖为剑士图、又因反斜杠 bug 落过坏路径，终态已
恢复用户选值）；浅色主题壁纸 =
`D:/Down/stella-os/wallpapers/songyu.png`（用户钦定——
`D:\Down\stella-os\wallpapers` 即机器缺省壁纸目录，
`wallpapers_dir` 留空时代码探测兜底就是它，picker 扫描同源）。

**现状（已证实）**：无此功能——`DesktopConfig` 单 `wallpaper_path`，
`set_theme` 动词只切 `dark_theme`/`theme_source` 不触碰壁纸。执行期
另有实证：多实例写竞态把活跃壁纸冲回 `#101014`（本 Phase 落地后槽值
走字段级合并护栏，此类丢失不再复现）。

**实现（lang plan-043-dev @ 5ff4646fc）**：
1. `desktop_config.rs`：DesktopConfig 增 `wallpaper_path_dark` /
   `wallpaper_path_light` 双槽；serialize/parse/`apply_field`（字段级
   合并）同步；`load()` 尾部挂 `apply_per_theme_wallpaper_slots`——
   ①存量单值迁移（当前主题槽空且单值非空 → 播种，幂等）②boot 按生效
   主题取槽（槽非空 → 单值跟随，内存生效不回写）。活跃值语义不变
   （`wallpaper_path` 仍是渲染层读的单源，槽 = 记忆面）。
2. renderer `execute_set_wallpaper`：写**当前主题**槽（另一槽不动）。
3. renderer `execute_set_theme`：切主题后目标槽非空且 ≠ 活跃值 →
   `execute_set_wallpaper(slot)`（旧布局快照/落盘/快照全撤/格子重注入
   同链复用）。
4. os 侧零改动（picker 语义宿主收口——shell 只发 `set_wallpaper`，槽
   路由在宿主按当前主题完成）。

**验证**：
- 单测：desktop_config 作用域 **19/19 绿**（含 4 个新 per_theme 用例：
  槽往返 / 单值播种迁移 / boot 按主题取槽 / 播种不碰另一主题槽；
  `save_merges_external_field_changes` 合并回归带新字段过）。
- 实机（MCP acceptance 驱动）：深色设剑士图 → config 落
  `wallpaper_path_dark`=剑士；`set_theme light`（浅槽空 → 壁纸保持）→
  设 songyu → `wallpaper_path_light`=songyu 且活跃值=songyu；
  `set_theme dark` → **活跃值自动跟回槽值**（config 实录）。
  终态（用户两轮修正后）：dark_theme=true、深槽=purple.png、
  浅槽=songyu.png、活跃=purple.png。
- **执行期修正 @ 8cb2a4db1**：实机 picker 选图暴露反斜杠路径被
  `__desktop_cmd` VM 字符串管道吃掉（`D:\Down\...` 落盘成
  `D:Downstella-os...`，靠目录首图兜底假活、选非首图必错）——
  `execute_set_wallpaper` 入口统一规整正斜杠。终态浅槽 =
  `D:/Down/stella-os/wallpapers/songyu.png`（用户改正的路径）实录。

## Phase 4：桌面图标默认排布列主序（2026-09-24 用户并入；✅ 同日执行收口，分析修正）

**需求（用户裁定）**：默认排列 = 最左列起从上往下、满列再排下一列。

**执行期分析修正**（起草时两处预设都不准，实测修正）：
- ~~`layout.rs:219` Grid 翻列主序~~——该函数是**窗口平铺**（WM 布局
  动词 grid 档），与桌面图标无关，不动。
- ~~自由填充改列主序~~——`desktop_icon_cells` ② 的未定位补位**已经是
  列主序**（2026-09-15 用户裁定在案：`k/rows, k%rows` 纵向优先）。

**真因（两个）**：
1. **口径分裂**：`desktop_icon_apply_drop` 的无位补位仍是**线性行主序**
   扫描（`s=0,1,2…`），注释却写"与 desktop_icon_cells 同口径"——
   09-15 改展示网格列主序时漏了拖拽臂，拖拽一发生补位图标横插。
2. **缺省底稿遗产数据**：`shell.desktop.positions`（+13 个 wp 桶）为
   旧时代产物，其 `c:r` 值经现行读卡器解读成横向条带（用户截图顶行
   计算器/时钟/待办/天气/备忘录横排即此）——新壁纸 fallback 到底稿
   即被污染。

**修复**：
1. lang `desktop_icon_apply_drop`：补位循环改列主序（rows 口径照抄
   desktop_icon_cells；占位跳过同构）——plan-043-dev @ 5ff4646fc。
2. 数据修复：备份 `desktop-storage.json` + `config.at` 至
   `~/.config/autoos/backup-p043-phase34/` 后，清除全部 14 个
   `shell.desktop.positions*` 键——所有壁纸语境走纯列主序默认流
   （用户拖拽按壁纸键落新桶，互不污染）。

**验证**：desktop_icon 作用域 3/3 绿；清键后重启实机截图——第 1 列
自上而下 = icons 清单序前 9（001→003→004→计算器→时钟→待办→天气→
备忘录→日历），满列换列，与需求逐项吻合
（`tmp/autoui-screenshot-1790239488832.png`；中段图标为会话恢复窗
遮挡，非缺失）。

**遗留观察**：切主题 dashboard face 渲染滞留（F-R1，在案）不在本
Phase 范围。
- **单实例桌面防御（2026-09-24 用户裁定方向，未来立项）**：同一系统
  只允许共用**一个**桌面实例——桌面本体可切换多个虚拟桌面（工作区），
  多开 OS 实例冗余且正是历史状态互踩（壁纸/图标丢失三症状）的祸根。
  现阶段用户不会多实例并跑；落地候选：boot 单实例守卫（命名互斥/锁
  文件 + 二次启动转激活已有实例），与 PLAN-044 字段合并护栏互补。

## 复审记录

**2026-09-24 work 复审（Part 1 + Phase 2，lang plan-043-dev 双提交）— pass**：
- **验收对账**：Part 1 独立形态验收标准逐项重验（scan 三态/thumb 缓存/full
  Content-Type/目录跟随/导航全链/收藏跨会话）——全过；桌面形态 launch
  终验为**合并后剩余项**（不阻塞代码 merge，验证面非代码面）。
- **遗漏/债扫描**：分批渲染闸为 documented 性能边界（lang 层异步图片
  加载记后续债）；str[i] 字符码陷阱与 text 表达式两坑已修并注记；
  030 配方修订未单独实机验（幂等臂回归钉覆盖 + 020/029 实机绿，
  残余风险登记）。
- **健康检查**：fmt/警告零新增（diff 面）、无调试印残留。
- **合并实况**：master 已含 PLAN-042 T-08 同 bug 收敛修复（object/list
  直通）——冲突消解取并集（043 扩 bool/null + 幂等回归钉 + 029/030
  配方），020 取 master 插桩版；合并缝修 master 新 LaunchSpec 字面量
  缺 photo_root 字段。合并树 photo_service 10/10、p080 5/5、
  cargo check 零 error。
- **merge receipt**：`auto-lang master 5af53ff3f`（合 plan-043-dev
  @ 5778b1b55）；worktree 移除前 wt-guard 拦截 360 个 pnpm junction
  → 逐一枚举 rmdir 清链后复扫 clean → 移除零残留；分支 plan-043-dev
  已删；组目录 `.wt/lang-043` 已清（auto-down 兄弟检出同步移除）。
- spec 沉积：`.autoos/specs.json` +3（P043-1 photo_service 能力臂 /
  P043-2 to_value 幂等臂 / P043-R1 本复审）。

**2026-09-27 review（全计划收口复审，实现会话内独立 artifact 重构）— pass**：
- **基线**：plan @ os main `1ae80c6`（HEAD）；依赖=auto-lang master——
  5af53ff3f / 4dc4d581a / 70bcd5246 / 86b56884b / 1d597e3f4 merge-base
  祖先实证 OK；**哈希漂移勘正**：plan 引用的 5ff4646fc/8cb2a4db1 为
  rebase 前哈希，内容以 `d50c61ff8`（Phase 3/4）/`3e3e1e297`（反斜杠
  修正）落 master（git log -S 实证）。specs 输入=.autoos/specs.json
  P043-1/2/R1（内容与计划一致）。lang 主检出 clean；os tracked 树 clean。
  worktree 已清（无 os-043 组），按 commit 祖先+记录史验证落点。
- **局限声明**：复审在实现会话内进行——判定由盘上代码/数据/截图
  artifact 重构，不依赖执行摘要。
- **代码面六件 master 实证**：launch 查表门（session.rs:3241 +
  app_registry.rs:645 manifest_launch_lookup）；VM_LAUNCH_FENCE=[]（renderer.rs:13946）；
  patches/iced_widget + [patch.crates-io] 挂载（Cargo.toml:105）；
  back_needs_session `#[api]` 谓词（back_provision.rs:29-32）；018 三
  文件 HTTP 配方（lang examples/ui/018 src/front 全中）；desktop_config
  wallpaper 双槽 + apply_field + 迁移（desktop_config.rs:64/168）。
- **数据/状态面对拍**：apps.manifest 10/10 `launch:"vm"` + tetris id
  036-tetris 对齐 ✓；config.at 浅槽 songyu.png + dark_theme=true ✓；
  README jade-edit 行 ✓；icons mapping kanban 键在/auto-kanban 死键无 ✓；
  025 sys_store `use back.api` 回滚完整（Http.get_json 零命中）✓；
  klondike 无 ondblclick / tetris 无 aria-label / auto-term rust-workspace
  无 ui-gpui ✓；stash@{0} ui-gallery WIP 保全标签在案 ✓；positions
  遗产键零残留 ✓。
- **运行时证据复用理由**：同日同内容提交（SHA 绑定）的实机验收记录
  （VM-only 桌面终态/双游戏/壁纸深浅跟随/图标列主序，截图
  tmp/autoui-screenshot-1790{3227582516→330037788}.png 族在盘）+
  后续 PLAN-044 全量 5495 套件对拍覆盖同内容面；桌面当前未运行，
  不做重复实机驱动（避免扰动用户环境）。
- **发现 F-R1（状态回归，非代码缺陷）——已修复**：09-26 08:22→23:17
  间（并行会话窗口）desktop-storage.json 被某运行实例整体写回近空态
  （shell.desktop.icons 29 白名单 + wp 桶全失，仅剩 vm.* 两键）且
  config.at 深槽被冲回 #101014——即本计划在案的 load-once+整文件覆盖
  × 多实例症状族复发（08:22 截图仍 29 健生态、23:17 截图已自愈播种
  11 默认图标 + 纯色壁纸，实证窗口）。代码防线上游均已在位（config
  字段合并=PLAN-044 已 merge；storage 键级合并写与单实例防御=移交
  清单 9/10 候选）。处置：桌面离线窗口备份后修复——icons 29 白名单
  重写（序=08:22/17:53 截图证实老序 + jade-edit 换位 + ui-gallery/
  widgets-gallery/kanban 补入尾部）、config 深槽/活跃=purple.png；
  复读校验过。备份：`~/.config/autoos/backup-p043-review/`。
- **发现 F-R2（知识面，转 merge 处置）**：09-25/26 新增持久决策
  （per-app launch 字段机制、分主题壁纸双槽契约）未入 specs.json
  （现存三条为 09-24 面）——merge 阶段补沉积 designs 两条
  （P043-3/P043-4，frontmatter 已增补）；iced 补丁政策以
  patches/iced_widget/README.md 为仓内持久载体，引链即可。
- **结论：pass**（全部代码/行为判据过；F-R1 状态面已修复、F-R2 非
  code 阻塞转 merge 知识沉积）。next=merge。

## 待澄清事项

- 宿主 panic 是否单独立 plan（auto-lang crates/ 改动 Category A/B 门档），
  还是在本 plan 内做 os 侧复现 + lang 侧修复协同。

## 移交下会话清单（2026-09-25 会话收口；新计划从这里起）

已根治/收口（本 plan 内闭环，勿重复立项）：029 实时扫描、get_json 幂等、
017/018 HTTP 配方+iced 补丁、027/029/031/031-paint 走查恢复、036/037
游戏启动（SHM 桥+构建三件）、ui-gallery（WIP stash 后恢复）、jade-edit
注册（manifest+README+图标换位）、图标 29=launcher 镜像、分主题壁纸
双槽、图标列主序、boot 直挂退役、围栏全空、auto-kanban 图标。

移交清单（新计划的候选工作面，按优先序）：
1. **025-sys-monitor 表格桌面内嵌不渲染**（深挖未决，证据链+三 suspects
   +诊断工具缺口见条目）——最高优先之一。
2. **宿主 panic 族兜底扩展**（patches/iced_widget 已缓解 container 一族；
   其余部件同型 ~25 处未补 + 最小复现待做）。
3. **auto-term VM 集成引擎链**（autoterm_core.dll 已部署宿主 exe 同目录、
   mux tick future 空列表崩溃每拍、shell 面板空——引擎 DLL 链诊断）。
4. **a2r codegen 两债**：视图 if 条件位 bool 降链（klondike main.rs 直修
   在案，再生会回退）；工作区脚手架 ui-gpui 残留复核（tetris/auto-term
   盘上已清，模板已净，再生场景复核）。
5. **016-calendar**：use `datetime` 解析失败 + flex-wrap native 降级
   （Plan 412 在案）。
6. **024-charts**：use `{ package }` 多行形（引用符号面落空）。
7. **launcher 覆盖层驻留 + z 序**（summon 后不关、启动窗开在其后）。
8. **Windows 端口保留段**：020 back_port 8320 在段内起不来（改口或 -B）。
9. **DEFAULT_DESKTOP_ICONS registry 驱动播种 + demo 排除**（图标白名单
   语义化；本会话数据级 29 已稳）。
10. **jade-edit / auto-kanban 的 submodule 收编**（用户：将来考虑；
    kanban 先例）。
11. **RQHost 路线（下版本）**：desktop 方言统一（旧 exe 重编）、中文渲染
    /图标字体、auto-term 引擎链在 outproc 形态的完善。

跨会话纪律（并行会话在案）：共享主检出多会话并行常态——清扫用 /PID、
外部 exe 产物重建前先对协议、gitignore 生成物的修复要登记（再生会回退）。
