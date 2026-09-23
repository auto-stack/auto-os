---
plan_id: PLAN-043
status: executing               # drafting → executing → execution_done → reviewed → archived
feature_name: desktop-app-vm-check
author: []
created_at: 2026-09-23
updated_at: 2026-09-23

# /auto-plan:review 结束时填写：
supersedes_spec_components: []
new_spec_components: []
touched_goals: []             # 引用 docs/specs/goals.md 的 GOAL-NNN

affects: [auto-lang/examples/ui, apps/]
current_step: 1
total_steps: 1
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
- [✅ 已完成] T6 半闭环：**后端半全过**——独立形态 curl 级断言：scan 200
  / 411 条目（实目录全量，旧烘焙 24 张对照）/ root_missing 三态；
  thumb 200 image/jpeg 173×260 等比 19KB，二次请求 0.2s（磁盘缓存命中）；
  full 200 3.4MB Content-Type 正确。**前端半被预存框架回归阻塞**（见
  Part 2——非本计划改动引入）。
- [ ] T6 收尾：Part 2 框架回归修复后 → 独立形态网格实渲染截图 + 桌面
  形态 launch + 目录增删跟随性实证。

## Part 2：split 形态 VM 前端 `Http.get_json` 返回解码坏（预存框架回归，
阻塞 Part 1 前端验收 + 020 生产坏）

**现象**：app 前端 `Http.get_json("/api/photos/scan")`（同 `/api/media/scan`）
拿到的返回值不是响应体，而是一个 20 字符十进制数（实测
`18443647848969734819`，顶部位 0xFFF2——疑似 NaN-boxed 指针位被当整数
读出）。`json.to_value` 将其解析为 number → `data.entries ?? []` → 空表
→ **静默空态，无错误抛出**。curl 直查同一端点数据完整（029 scan 411 条 /
020 media scan 393 条）。

**影响面**：020-music-player（本计划零改动的 app）同症状——独立 split
形态 item_count=0/library_error=""（后端 393 条）；桌面形态同样中招
（2026-09-23 主检出 09:47 构建 ui_desktop boot 即报「本地曲库为空」，
E:\Music 实有 393 首）。即**双形态下所有靠 get_json 拉后端数据的 app
全部静默空数据**。

**证据链**：
- Rust 层响应正确：`AUTO_LANG_HTTP_TRACE=1` 实证 status=200 +
  body 前缀正确。
- 值在入 .at 前已坏：前端把 raw 落盘（`File.write_text`）实证 20 字节
  数字串；与响应体大小无关（3 张迷你根目录同症状）。
- 回归窗口：Plan 027 内存修复两提交恰好改该路径消费语义——
  `e5a3e85d5`（09-21，ASYNC_RESULTS 消费即删键）与 `69420195f`
  （09-22，Err 条目映射 + worker 池化 + entry.or_insert）；
  Plan 037 时代（≤09-15）AC-04 曾实证 020 split 出曲库正常。
- 嫌疑链：`shim_http_get_json` re-entry → `check_async_http_result` →
  `push_string_result`（`vm.add_string` + `rc_push_str_idx`）→ .at 侧
  读取解码。修复方向 = 该链路的 tag/索引位约定核对（lang 侧 bisect
  e5a3e85d5^..69420195f 可分钟级定位）。

**处置（待用户裁定）**：lang 侧单独立 plan 修复（Category B 门档；
修复后 Part 1 T6 收尾 + 020 自动恢复）。本计划内不再展开。

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
- **018-book-reader**：`use back.api`（get_book/list_chapters...）解析
  失败（PLAN-664 静默跳过）→ 同样「无法启动」占位。
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

## 待澄清事项

- 宿主 panic 是否单独立 plan（auto-lang crates/ 改动 Category A/B 门档），
  还是在本 plan 内做 os 侧复现 + lang 侧修复协同。
