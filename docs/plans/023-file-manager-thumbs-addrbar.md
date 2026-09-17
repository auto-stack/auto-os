---
plan_id: PLAN-023
status: drafting               # drafting → executing → execution_done → reviewed → archived
feature_name: file-manager-thumbs-addrbar
author: [agent]
created_at: 2026-09-17
updated_at: 2026-09-17
plan_revision: 1

# /auto-plan:review 结束时填写：
supersedes_spec_components: []
new_spec_components:
  - auto-lang/stdlib/auto/image.at §thumb（缩略图 stdlib 契约面）
  - auto-lang/examples/ui/027-file-manager/SPEC.md §2.5 网格缩略图
  - auto-lang/examples/ui/027-file-manager/SPEC.md §1.5 地址栏坍缩
touched_goals: []              # 引用 docs/specs/goals.md 的 GOAL-NNN

affects:
  - auto-lang/stdlib/auto/image.at           # thumb 契约声明
  - auto-lang/stdlib/auto/image.rs.at        # rust 轨声明
  - auto-lang/stdlib/auto/image.vm.at        # vm 轨声明
  - auto-lang/crates/auto-lang/src/ui/image_pipeline.rs   # queue_media_thumbnail
  - auto-lang/crates/auto-lang/src/vm/ffi/stdlib.rs       # shim_image_thumb + 注册
  - auto-lang/crates/auto-lang/src/ui_gen/ts_adapter.rs   # VM-only 白名单加 image
  - auto-lang/examples/ui/027-file-manager/src/front/app.at          # 网格缩略图 + 地址栏
  - auto-lang/examples/ui/027-file-manager/src/front/components/fs_util.at  # is_image_ext
  - auto-lang/examples/ui/027-file-manager/SPEC.md
  - auto-os/docs/plans/023-file-manager-thumbs-addrbar.md

current_step: 0
total_steps: 5
---

# [PLAN-023] file-manager-thumbs-addrbar

> 跨仓计划：**主导仓 = auto-os**（桌面程序域，沿 PLAN-013/015/016 先例）。
> 主战场在 auto-lang（027-file-manager app + image stdlib + 媒体管线），
> 执行 worktree 组：`.wt/os-023/auto-os` + `.wt/os-023/auto-lang`
> （Plan 529 布局；分支 plan-023-dev）。

## 0. 变更摘要

用户两项改进需求，一次计划收口：

| 面 | 内容 | 改动域 |
|----|------|--------|
| A. 大图标缩略图（W1–W2） | 网格视图中图片文件显示真实预览图：框架 `auto.image` stdlib 新增 `thumb(path, size) -> uri` 原语（走既有 Plan 547 媒体管线，`MediaPriority::Thumbnail` 档异步解码），027 网格卡接线 `ImageSurface`，非图片/未就绪回落 FileIcon | auto-lang stdlib + crates + 027 |
| B. 地址栏可伸缩（W3） | 面包屑容器平时占满标题栏可用宽（紧贴搜索框）；深路径自动坍缩为 `首段 › ... › 末2段`，点 `...` 展开全链；超长段名 truncate | 仅 027 app.at |

**预览图来源裁决（用户问题 1 的回答，D-3）**：v1 采用**自建管线**（Plan 547
媒体管线已建：异步 worker 池 + `MediaPriority::Thumbnail` 优先档 + EXIF 旋转
+ Lanczos 缩放 + 引用计数注册表 + 三端共享路由 `/api/__auto/media/`），
不接 Windows Shell API。理由：
1. **复用 Windows 的两条路线中，直接解析 `thumbcache_*.db` 不可取**——格式
   未公开、随版本变、失效通知要接 shell change notify；正路是 COM 接口
   `IShellItemImageFactory::GetImage`（内部自动读写系统缩略图缓存）。
2. **方便性**：Shell 臂要 windows-rs COM 依赖 + worker 线程 COM 初始化纪律
   + 仅覆盖 Windows；Linux 无论如何要自建（freedesktop `~/.cache/thumbnails`
   规范）→ 复用 = 永久双代码路径；自建 = 一条通路双端共用，且管线已存在，
   边际成本 ≈ 一个 stdlib 函数。
3. **性能**：冷文件（谁都没缩略过）两者都要读盘解码一次，同量级；热文件
   （资源管理器已浏览过）Shell API 命中 thumbcache 免解码，快一个量级——
   这是复用唯一的实质优势。但自建管线进程内有解码缓存（二次导航零解码）+
   异步渐进填充（250ms Tick 驱动逐张浮现），首次延迟不阻塞 UI。格式覆盖上
   Shell API 更广（HEIC 等系统编解码器），管线白名单现 jpg/jpeg/png/webp。
   **结论**：热缓存优势不足以抵偿双路径维护面；Shell 臂列为后续可选优化
   （封装在 `image.thumb` 内部对 app 透明，若实测冷解码不达标再加）。

## 1. 目标

1. **G1 网格缩略图**：大图标（grid）模式下，图片扩展名文件显示真实预览图
   （请求 256px rendition，cover 裁切入卡）；非图片文件、解码未就绪、不支持
   格式一律回落 FileIcon，零空白卡。列表视图不变（FileIcon）。
2. **G2 主线程零解码**：`image.thumb` 只入队（注册 + 优先级排队），解码全在
   媒体 worker 线程；目录导航不因缩略图卡顿；缩略图随 250ms Tick 渐进浮现。
3. **G3 地址栏占满**：面包屑胶囊平时占满 顶部栏 中 返回/前进/上一级 与 右侧
   操作区（搜索框起）之间的全部可用宽。
4. **G4 深路径坍缩**：面包屑段数 > 5 时自动坍缩为 `首段 › ... › 末2段`；
   点 `...` 就地展开全链（导航后重回坍缩态）；单段名超长 truncate 截断。
5. **G5 双轨一致**：VM 轨全功能；vue 轨编译通过（`image.*` 走 `__vmOnly`
   降级，缩略图回落 FileIcon），地址栏坍缩双轨同构（纯 model 逻辑）。

非目标：列表视图缩略图、Windows Shell API 臂、缩略图落盘持久缓存
（freedesktop 规范）、视频缩略图、HEIC 等扩展格式白名单扩充。

## 2. 架构方案

```
027 NavTo(dir)
  └─ 逐条目物化 files_view（既有循环内）
       └─ is_image_ext(ext) && kept < THUMB_CAP
            └─ image.thumb(path, 256) -> str        # 新 stdlib，只入队
                 │                                   # ""=不支持/失败
                 └─ row.thumb_src = uri
iced 渲染（每 Tick 重建 view）
  └─ ImageSurface(src: row.thumb_src, fit: "cover")
       └─ resolve_media_render(uri) → 已解码则出图；
            未就绪本帧空 → 下个 Tick（250ms）浮现
```

- **媒体管线复用**（零新增解码代码）：`queue_media_rendition(path, spec,
  MediaPriority::Thumbnail, gen)` —— `Thumbnail` 档（优先级 5 最低、
  `MediaPin::None` 可驱逐）正是为本场景设计；`RenditionSpec{256,256,
  Lanczos3}`；EXIF 旋转管线自带；iced 侧 per-asset Handle 缓存防闪（P547）。
- **URI 契约**：`/api/__auto/media/{id}/{rev}`，与 031-image-viewer 同路由，
  ImageSurface 三端（ark/iced/vue）组件已注册。
- **vue 轨**：`ts_adapter.rs` 对 VM-only natives 有 `__vmOnly` 降级白名单
  （现 `fs | File`，Plan 444 先例），`image` 加入即编译安全；thumb_src 恒
  ""，FileIcon 回落。
- **地址栏坍缩**（纯 model 逻辑，双轨同构）：NavTo 构建全链 crumbs 后按
  策略切为 `crumbs_head`（首段）/ `crumb_gap`（bool，中段坍缩存在）/
  `crumbs_tail`（末 2 段）三面；视图三片段平铺（规避 VM 循环内条件节点
  纵向堆叠债，R4-2 实证）；`...` 段为独立按钮（CrumbsExpand 消息）。

## 3. 技术栈

auto-lang（.at DSL + Rust crates）、Plan 547 媒体管线（`image` crate 解码）、
iced VM 轨 + vue 生成轨、auto-os 桌面注册表（验收载体）。

## 4. 需求分析与背景调查

- **授权**：用户 2026-09-17 提出两项改进 + 预览图方案问题；指示
  「改动时要用 worktree 形式（配合计划文件）」→ 本计划即授权范围：
  auto-os（计划 + 验收收据）+ auto-lang（stdlib/crates/027）双仓，
  worktree 组执行。无额外预算约束。
- **代码事实**（2026-09-17 主检出核对）：
  - 027 网格卡 = FileIcon(ext, 36) + 名称 + 大小（app.at L531–566）；
    面包屑 crumb 全部 `shrink-0`（L204–216），溢出即硬裁——用户截图实证。
  - `auto.image` stdlib 面：会话 API（open_session/request_view，viewer 用）
    + 逐文件 ticket API（queue/retain/release）；**VM 轨 queue 返回整型
    句柄而非含 uri 记录**（stdlib.rs shim_image_queue L4079），且 queue 走
    `MediaPriority::Current`（全尺寸解码 + Hard pin，不适合批量缩略图）→
    必须新增 thumb 原语而非复用 queue。
  - `request_media_view` 返回选中项 JSON 快照且**改变会话选择**——逐条目
    轮询语义扭曲，不采用（设计否决记录）。
  - `truncate` class 在 iced = 单行 + clip 容器裁（renderer.rs L3310 注），
    **无 "…" 字形**——VM 轨做不到 CSS 式省略号，故地址栏选确定性坍缩策略
    （段级），段名截断在 vue 轨得真省略号、VM 轨为裁切（可接受，记录）。
  - F-2 框架债：popover 不可入 mouse-area 子树（closed 态泄漏参与布局）→
    `...` 展开不用 popover，用就地展开（模型标志位），零框架风险。
  - SPEC（027）§1 数据层 cap 500；§2 主题/图标 FileIcon 分支——本计划
    与其正交，SPEC 增量见 SD 表。
- **前置计划**：PLAN-016（file-manager-revamp，archived）——027 现代化
  基座；Plan 547（媒体管线，auto-lang）——ImageSurface + worker 池。

## 5. 详细设计

### 5.1 `auto.image.thumb` stdlib（W1）

```
// stdlib/auto/image.at（三轨声明同步 image.rs.at / image.vm.at）
/// Queue a thumbnail rendition for an image file and return its opaque
/// media URI ("/api/__auto/media/{id}/{rev}"), or "" when the path is
/// missing, unreadable, or has an unsupported extension. Decode happens
/// asynchronously on the media worker pool at Thumbnail priority; the URI
/// renders blank until the rendition is published (progressive fill).
pub fn thumb(path str, size int) str;
```

- 实现 `image_pipeline.rs::queue_media_thumbnail(path, size) -> String`：
  `supported_media_path` 白名单门 → `RenditionSpec{ width: size, height:
  size, quality: 85, original_pixels: false }` → `queue_media_rendition(…,
  MediaPriority::Thumbnail, 1)` → `media_uri(ticket)`；门失败返回 `""`。
- VM shim `shim_image_thumb`（stdlib.rs，`register_shim_by_name(
  "auto.image.thumb", …)`）：弹栈 size、path → 调上者 → push uri 字符串
  （push_string_result 形态，同 shim_image_current_uri）。
- ts_adapter.rs VM-only 白名单 `"fs" | "File"` → 追加 `"image"`（vue 轨
  `image.thumb` 编译为 `__vmOnly('image.thumb',…)` 抛错降级——调用点在
  027 以 `in_desktop`/VM 守卫不达，见 5.2）。
- **驱逐卫生**：Thumbnail 档 `MediaPin::None`，注册表按容量/世代驱逐；
  027 侧不持 ticket 引用（URI 失效 = 回落 FileIcon，自愈），NavTo 不显式
  release（无句柄可放）。验证 T-01 含注册表统计冒烟（stats 不单调涨）。

### 5.2 027 网格接线（W2）

- `components/fs_util.at` 新增 `is_image_ext(ext) bool`（字面量分支：
  jpg/jpeg/png/webp/bmp/gif/ico——管线白名单的超集无害：管线侧二次门）。
- `app.at` 常量 `THUMB_CAP = 120`（8 列 × 15 屏，超出不排队；滚动外条目
  不常用）；物化循环内：`is_image_ext(ext) && kept < THUMB_CAP` 时
  `thumb_src = image.thumb(pp, 256)`，否则 `""`；row 记录加 `thumb_src` 字段。
- 调用守卫：thumb 调用仅当 `Env.get("AUTO_UI_IN_DESKTOP") != "" ||
  boot 后 VM 轨`——以 `in_desktop` 变量门（Init 已解析；独立 `auto run -r vm`
  窗口该 env 为空会误伤）→ **改用：vue 轨 `__vmOnly` 抛错由 VM 守卫规避**
  ——不可行（vue 编译期降级、运行时也不达）：实际以 `thumb_ok` 布尔门：
  Init 末尾 try 质探 `image.thumb` 代价高 → **定案**：ts_adapter 白名单化
  后 vue 轨调用点编译为 `__vmOnly`（运行时抛错），027 在调用外包
  `if .vm_track { … }`，`vm_track` 于 Init 以 `Env.get("AUTO_OS_ROOT") != ""
  || in_desktop` 判定（VM 桌面/独立窗均真，vue 轨恒假——env 在浏览器侧
  恒空）。执行时若发现更简判别（如编译期轨别常量）可 trivial 替换。
- 视图（grid 分支）：`thumb_src != ""` → 定尺寸容器
  （`h-24 w-full rounded-lg overflow-hidden bg-muted`）内 `ImageSurface
  (src, fit: "cover", width: 96, height: 96)`；否则 FileIcon(36) 原样。

### 5.3 地址栏坍缩（W3）

- model 增：`crumbs_head = []`、`crumbs_tail = []`、`crumb_gap bool =
  false`、`crumbs_full = []`（全链留存）、`crumbs_expanded bool = false`；
  msg 增 `CrumbsExpand`。
- NavTo 尾部：既有 cs 构建后，`.crumbs_full = cs`；`apply_crumb_policy()`
  等价内联：`cs.len() > 5 && !.crumbs_expanded` → head = cs[0..1]、
  tail = cs[len-2..len]、gap = true；否则 head = cs、tail = []、gap = false
  （`.crumbs` 保留全链供兼容/调试）。
- `CrumbsExpand -> { .crumbs_expanded = true; head = crumbs_full; tail=[];
  gap=false }`（就地展开，不导航）。NavTo 入口重置 `crumbs_expanded=false`
  （新目录回坍缩态）。
- 视图（非编辑态胶囊内，三片段平铺）：
  `for c in .crumbs_head {…}` + `if .crumb_gap { button "…" onclick:
  .CrumbsExpand + sep "›" }` + `for c in .crumbs_tail {…}`；
  crumb 按钮 style 追加 `max-w-[10rem] truncate`（vue 真省略号；VM 裁切）。
  胶囊行保持 `flex-1 min-w-0` + `w-full overflow-hidden`（占满 + 兜底裁），
  左侧导航钮簇与右侧操作区 `shrink-0`（现状已正确，验证不动）。

### 5.4 规范增量

| delta_id | add/modify/retire | target | before/after | rationale | AC |
|----------|-------------------|--------|--------------|-----------|----|
| SD-01 | add | auto-lang/stdlib/auto/image.at §thumb | 无 → `thumb(path,size)->str` 契约（URI/""，异步，Thumbnail 优先档） | 文件管理器批量缩略图缺逐文件原语；ticket API 无 uri 出口、会话 API 选择态扭曲 | AC-01/02 |
| SD-02 | modify | auto-lang/examples/ui/027-file-manager/SPEC.md（新增 §2.5 网格缩略图） | FileIcon 恒定 → 图片扩展名 thumb_src 优先、FileIcon 回落、THUMB_CAP=120 | 用户需求 1 | AC-01/05 |
| SD-03 | modify | auto-lang/examples/ui/027-file-manager/SPEC.md（新增 §1.5 地址栏坍缩） | 全链 shrink-0 硬裁 → >5 段坍缩 `首+...+末2`、可展开、段名 truncate | 用户需求 2（截图实证溢出） | AC-03/04/05 |

## 6. 测试设计

- **T-01 单元**：`image_pipeline.rs` tests 模块补 `queue_media_thumbnail`
  冒烟（不存在路径 → ""；testdata png → uri 形如 `/api/__auto/media/{32hex}/1`）；
  既有 `a2r_std_signature_parity` 等套件回归（stdlib 三轨声明同步则绿）。
- **T-02/T-03 双轨验证**：VM 轨 `auto run -r vm`（027 目录）实机：
  - 导航至含图片目录（tests/testdata/photo.png + 用户 Pictures），grid 模式
    截图：图片卡出缩略图、其余 FileIcon；
  - 深路径（C:\Users\zhaop\AppData\Local 一类 ≥6 段）截图：坍缩形态 +
    点 `...` 展开形态；
  - vue 轨 `auto run` 编译零 TS 错（白名单生效）。
  证据 PNG 存 auto-os `docs/plans/evidence/023/`。
- **桌面注册表链路**（用户主场景）：auto-os 桌面 `auto run`（025/027 注册表
  in-process）复验 027 打开 + grid 缩略图（desktop_mcp.py 截图臂）。

## 7. 验收标准

| ID | 可观察行为 | 验证方法 |
|----|-----------|----------|
| AC-01 | grid 模式图片文件卡显示真实缩略图（cover 裁切），非图片/未就绪回落 FileIcon 零空白 | VM 桌面截图（testdata + Pictures） |
| AC-02 | 导航响应不受缩略图影响（无主线程解码；目录切换即时，缩略图 250ms 级渐进浮现） | 实机操作观察 + image.thumb 实现评审（入队即返） |
| AC-03 | 地址栏胶囊平时占满导航钮与搜索框之间全部可用宽 | 截图（浅路径） |
| AC-04 | ≥6 段路径坍缩为 `首段 › … › 末2段`；点 `...` 就地展开；导航后重回坍缩 | 截图（深路径 ×2 形态） |
| AC-05 | vue 轨编译通过，行为同构（缩略图 FileIcon 回落、坍缩逻辑生效） | `auto run` 构建零错 |
| AC-06 | SPEC 增量三节落库（SD-01/02/03 对应文档） | 文件评审 |

## 8. 执行步骤

（原子任务；每步完成后追加 [✅ 已完成] 一行证据。worktree 组
`.wt/os-023/{auto-os,auto-lang}`，分支 plan-023-dev。）

### T-01 框架：auto.image.thumb stdlib（SD-01）
- 文件：`stdlib/auto/image.at` / `image.rs.at` / `image.vm.at`；
  `crates/auto-lang/src/ui/image_pipeline.rs`（queue_media_thumbnail +
  单测）；`crates/auto-lang/src/vm/ffi/stdlib.rs`（shim_image_thumb +
  注册）；`crates/auto-lang/src/ui_gen/ts_adapter.rs`（VM-only 白名单加
  "image"）。
- 操作：按 5.1 实现；三轨声明同步（a2r_std_signature_parity 门）。
- 验证：`cargo test -p auto-lang --lib image_pipeline`（新 2 用例绿）+
  `cargo test -p auto-lang --lib a2r_std`（parity 绿）。
- 关联：AC-01/02

### T-02 app：网格缩略图接线（SD-02）
- 文件：`examples/ui/027-file-manager/src/front/components/fs_util.at`
  （is_image_ext）、`src/front/app.at`（thumb_src 字段 + 物化守卫 + grid
  ImageSurface 分支 + vm_track 门）。
- 验证：`auto run -r vm`（027 目录）实机 grid 截图（AC-01/02）。
- 依赖：T-01。关联：AC-01/02/05

### T-03 app：地址栏坍缩（SD-03）
- 文件：`examples/ui/027-file-manager/src/front/app.at`（model 5 变量 +
  CrumbsExpand 消息/处理臂 + NavTo 策略内联 + 视图三片段 + 段名 truncate）。
- 验证：实机深路径截图 ×2 形态（AC-03/04）。
- 依赖：无（独立于 T-01/02，可并行）。关联：AC-03/04/05

### T-04 vue 轨构建验证
- 操作：027 目录 `auto run`（vue 轨）构建零 TS 错；mock 形态目测
  （FileIcon 回落 + 坍缩逻辑）。
- 依赖：T-01（白名单）/T-02/T-03。关联：AC-05

### T-05 SPEC + 证据落库
- 文件：`examples/ui/027-file-manager/SPEC.md`（§1.5/§2.5）；
  auto-os `docs/plans/evidence/023/`（截图收据）；本计划状态推进
  execution_done。
- 依赖：T-02/03/04。关联：AC-06

## 9. 复审记录

- 2026-09-17（new 起草）：`stage: new`，PLAN-023 r1。`outcome: pass`——
  任务覆盖全部 AC 与 SD；路径/命令经主检出核对（shim 注册形态
  shim_image_current_uri 同款、ts_adapter 白名单 Plan 444 先例、
  MediaPriority::Thumbnail 档既有）；无阻塞性待决（5.2 vm_track 门允许
  执行期 trivial 替换更优判别）。`next: work`。

## 10. 待澄清事项

- 无用户侧待决。执行期两点就地裁决权（记录于 5.2）：vm_track 判别式可
  trivial 替换；THUMB_CAP 可按实机性能调 120±。Windows Shell API 臂为
  后续可选计划（用户问 1 裁决记录见 §0）。
