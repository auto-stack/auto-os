---
plan_id: PLAN-028
status: reviewed              # drafting → executing → execution_done → reviewed → archived
feature_name: drawlist-image-channel
author: [agent]
created_at: 2026-09-18
updated_at: 2026-09-18
plan_revision: 1

# /auto-plan:review 结束时填写：
supersedes_spec_components: []
new_spec_components:
  - auto-lang/docs/design/autoui/desktop-protocol-v1.md   # §1.9 v1.9 增量 + 顶表 v1.5–v1.9 回填（review 复核定稿）
touched_goals: []

affects:
  - auto-lang/crates/auto-lang/src/ui/desktop_protocol/message.rs        # DrawOp::Image（tag 6 追加式）+ codec
  - auto-lang/crates/auto-lang/src/ui/iced/broker_surface.rs            # 宿主栅格化 Image 臂 + 缓存 + 降级
  - auto-lang/crates/auto-lang/src/ui/desktop_protocol/client_runtime.rs # 解释态 layout_image 真图升级
  - auto-lang/crates/auto-lang/src/ui/desktop_protocol/native_projector.rs # native Image 臂真图升级
  - auto-lang/crates/auto-lang/src/ui/desktop_protocol/coverage.rs       # 保真口径随注更新
  - auto-lang/crates/auto-lang/src/ui/desktop_protocol/remote.rs         # ts_fixtures golden 扩
  - auto-lang/packages/drawlist-renderer/                                # TS decode/render 臂 + golden
  - auto-lang/docs/plans/KNOWN-DEBT-AND-RISKS.md                         # P026-D1 图像半句核销
  - auto-os/docs/plans/autos-desktop-program.md                          # 台账行（B 前置第一件交付）
current_step: 8
total_steps: 8
---

# [PLAN-028] drawlist-image-channel

## 0. 变更摘要

DrawList（queue 臂命令帧）现只有 Quad/Text/TextStyled/Scissor 五算子，
**无图像算子**——image/icon/avatar 在两投影臂均为"占位 Quad 保真"（
P026-D1 在册债）；而 shell a2r 化已裁定 **B 形态**（outproc shell 经
RenderQueue 渲染，desktop-shell-a2r.md 裁定落定），**壁纸/窗口缩略图/
壁纸预览全是图像，图像 op 缺失是 B 硬阻断**（前置序列第一件）。本计划
交付图像通道 v1，**核心路线 = src 引用 + 宿主侧解析**（普查证实零新
依赖、零位图过线）：

**①wire**：`DrawOp::Image{rect, src, ...}`（tag 6 追加式，
`PROTOCOL_VERSION` 仍 1）——src 复用宿主既有图像词汇（本地文件/
`builtin:`/`data:`/`http(s)://`，`load_image_bytes` 现成）+ 保留
scheme 命名空间（`thumbnail://{wid}` 虚拟引用 = 宿主快照解析）；
**②宿主栅格化**：`canvas::Frame::draw_image`（iced 0.14 本仓 feature
链已通，零新依赖——仓内首用）+ `get_or_create_image_handle` 缓存 +
未解析降级占位 Quad + 观测行；**③两投影臂真图升级**（解释态
layout_image + native Image 臂同刻度，占位保真三处核销）；
**④缩略图虚拟引用**（`thumbnail://{wid}` → snapshot 缓存/SWR/
request_capture 既有基建——B 形态 shell 前置能力，本期即可测）；
**⑤TS 远程渲染器**（decode 必达防 throw + 占位渲染，web 真位图
not-yet 成文）+ 双侧 golden 对拍。

## 1. 目标

- **G1 wire 算子**：`DrawOp::Image`（tag 6）——rect + src（既有词汇）
  + 最小呈现参数（D1 定案：fit/filter/border_radius 子集）；codec
  round-trip + Rust↔TS 双侧 golden bytes；未知 tag 拒收纪律维持。
- **G2 宿主栅格化**：DrawListPainter 消费 Image op——`load_image_bytes`
  （file/`builtin:`/`data:`/`http(s)`）→ Handle 缓存 →
  `Frame::draw_image`；**未解析/加载失败降级 = 占位 Quad + 观测行**
  （I3 not-yet 纪律，禁静默错绘）；解码与缓存在宿主侧（child 免解码，
  "轻 child"哲学延续）。
- **G3 两投影臂真图升级（parity 同刻度）**：解释态 `layout_image`
  （client_runtime.rs:1340-1356，src 现显式丢弃）与 native `Image` 臂
  （native_projector.rs:890-898 占位）发 Image op；占位保真口径三处
  核销（两臂 + coverage 随注）；降级路径保留占位为兜底语义。
- **G4 缩略图虚拟引用**：`thumbnail://{wid}` → 宿主快照解析
  （snapshot.rs TTL/SWR/降采样 + screenshot×rect 裁剪既有链）渲染进
  DrawList 消费位——B 形态 shell（showdesk/workspace preview）的前置
  能力，本期以合成 client 直测。
- **G5 TS 与收口**：drawlist-renderer decode 臂（unknown tag throw
  防线）+ renderFrame 占位渲染（web 真位图 not-yet 成文——TS 无 shm/
  跨源约束）；desktop-protocol-v1.md **§1.9 v1.9 增量 + 版本表回填**
  （v1.5–v1.8 欠账）；KNOWN-DEBT P026-D1 图像半句核销（字形半句
  维持）；台账行；e2e 真图断言 + 度量（帧字节增量 = src 串长；宿主
  解码/缓存命中成本行）。

**非目标**（明确出界）：

- **位图字节过线通道**（shm 位图上传/内嵌载荷）——app 运行时生成
  位图（图表 canvas/data URL 之外的动态图）与 TS 真位图的共同前置，
  独立增量另立（src 引用形态覆盖文件/网络/内嵌既有词汇，v1 够用）。
- **lucide:/svgdoc: 字形真渲**（P026-D1 后半维持 not-yet——字形/
  矢量栅格化独立线；icon 现经 `View::image_styled("lucide:…")` 降级
  形态，本计划不扩其解析）。
- **imagesurface 交互族**（026 D5 not-yet 维持）。
- **shell outproc client 本体**（B 程序——本计划是其前置第一件）。
- **fit 高级语义组合**（cover/contain 裁剪编排——投影器 Scissor 可
  组合出，v1 = D1 定案最小集，精化随需）。
- **web 远程端真位图渲染**（fetch/跨源/data 通道——TS not-yet 成文）。
- 解释态/native 两臂的保真差分精调（对拍口径 = op 序列同源，视觉级
  精调另立）。

## 2. 架构方案

```text
┌─ wire 臂（desktop_protocol/message.rs）────────────────────────────┐
│ DrawOp::Image{rect: WRect, src: String, fit: ImageFit}（tag 6，    │
│   追加式；ImageFit 最小枚举 D1 定案，缺省值向后兼容旧端 = 占位）   │
│ codec encode/decode 对称 + 未知 tag 拒收维持（:194）               │
│ golden bytes：Rust 单测 + ts_fixtures↔fixtures.golden.ts 双侧钉    │
└──────────────────────────────────────────────────────────────────┘
┌─ 宿主栅格化臂（ui/iced/broker_surface.rs）─────────────────────────┐
│ paint_ops 增 Image 臂：                                            │
│   src 解析序 = thumbnail:// 虚拟引用（快照缓存/SWR/request_        │
│   capture 触发——G4）> load_image_bytes 词汇（file/builtin:/data:   │
│   /http(s)://，renderer.rs:5862-5899 现成）                        │
│   → get_or_create_image_handle 缓存（renderer.rs:5908-5925 同型，  │
│     key=src#wxh，防每帧重解码）→ Frame::draw_image（bounds, Image{ │
│   handle, filter_method, border_radius}——iced 0.14 feature 链已通）│
│ 未解析（路径缺/超时/方案未识）→ IMAGE_PLACEHOLDER Quad + 观测行    │
└──────────────────────────────────────────────────────────────────┘
┌─ 投影臂（client_runtime.rs + native_projector.rs，同刻度）─────────┐
│ 解释态 layout_image：src 丢弃 → 发 Image op（rect 沿既有尺寸推导   │
│   w=style.fixed_w 或 min(avail,96)——布局语义零变化）              │
│ native Image 臂：占位 Quad → Image op（同 rect 推导；026 §1.8     │
│   占位保真注释核销，降级占位转兜底语义）                           │
│ coverage 随注更新（image kind 真渲；占位保真边界条目改降级语义）   │
└──────────────────────────────────────────────────────────────────┘
┌─ TS 臂（packages/drawlist-renderer）───────────────────────────────┐
│ messages.ts：DrawOp 镜像类型 + decode 分支（tag 6 必达，否则       │
│   unknown tag throw 破坏 WS 会话）                                 │
│ render.ts：renderFrame Image → 占位 fillRect（web 真位图 not-yet   │
│   成文注释）                                                       │
│ fixtures.golden.ts ↔ remote.rs ts_fixtures 新 golden 帧双侧钉      │
└──────────────────────────────────────────────────────────────────┘
```

**不变式**：

- **I1 追加式协议**：`PROTOCOL_VERSION` 维持 1；tag 1–5 语义冻结零
  改动；Image = tag 6 新增，旧端（不识 6）行为 = 拒收会话（与既有
  unknown tag 纪律一致，无静默漂移）。
- **I2 零回归**：既有 op golden bytes 零漂移；两投影臂既有布局语义
  （rect 推导/占位尺寸）零变化——升级是"占位 Quad → Image op"的同位
  替换 + 宿主侧新增消费，帧结构对既有断言（p026 帧内定位法的
  quads/texts 谓词）保持兼容（image 占位 Quad 断言改写为 Image op
  断言，随注归因）。
- **I3 not-yet 降级纪律**：未解析 src = 占位 + 观测行，禁静默错绘；
  TS 真位图/字形真渲/位图过线三项 not-yet 显式成文。
- **I4 双臂同刻度**：解释态/native 同日同口径升级，parity 纪律沿
  026（I4 双轨分表不牵连——覆盖表两处各自随注）。

**关键风险**：`Frame::draw_image` 仓内零先例（feature 链已通但需首用
验证——wgpu/tiny_skia 双后端均实现，风险低仍需保真验证）；http src
在 e2e 的网络依赖（004 是远程 URL——离线降级断言兜底 + 029 本地文件
主载体）；命令帧 shm 槽 16KiB vs src 串长（URL 极端长度的截断语义——
D6 核）；宿主侧解码线程（load_image_bytes http 3s 超时阻塞 canvas
绘制——缓存先行 + miss 异步/同步取舍 D1）。

## 3. 技术栈

Rust / iced 0.14（canvas image feature 链已通——iced_graphics
`Frame::draw_image` frame.rs:96-100，wgpu/tiny_skia 双后端实现）；既有
`load_image_bytes` 词汇与 `get_or_create_image_handle` 缓存
（renderer.rs）；snapshot 基建（snapshot.rs：request_capture/TTL/SWR/
thumbnail_from_screenshot + screenshot×rect 裁剪链 renderer.rs:17343-
17357）；TS/Canvas2D drawlist-renderer；e2e 载体 = 004-profile-card
（http URL 宿主代取）+ 029-photo-gallery（本地文件真图源）+ 合成
client（thumbnail 引用）；断言形态 = p026 帧内定位法（quads_of/
texts_of 谓词轮询 composed()）扩展 image op 断言。

## 4. 需求分析与背景调查

**授权记录**：用户 2026-09-18 会话明确"计划 027 已经收口，请继续起草
下一个计划"——下一个计划按 B 形态前置序列（desktop-shell-a2r.md
裁定落定节）= **图像 DrawOp 通道**（上轮已向用户预告"下一步起草图像
通道计划"，本轮即其授权）。**本轮仅规划，未授权实施**。涉及仓：
auto-lang（wire/栅格化/投影臂/TS/文档）+ auto-os（台账/e2e 腿）。无
预算/自动续跑约束声明。**无前置计划依赖**（026/027 已 merge，两投影
臂与覆盖表现状即基线）。

**现状事实**（已核，2026-09-18 master @ 2bdb8bb7d，探索代理全量普查）：

- **DrawOp/codec**：5 变体 Quad/Text/TextStyled/Scissor/ScissorPop
  （message.rs:76-103），tag 1–5（encode :118-149/decode :163-194），
  **下一可用 tag = 6**；decode 未知 tag 报错拒收（:194，测试 :1576-
  1601 钉死）；DrawList = clear + ops 先到先画（:67-73）；三个编号域
  独立（op tag / 载荷 kind / FrameMsg tag——协议文档 :25-26）。
- **canvas 画图能力（零新依赖）**：iced 0.14（Cargo.lock 4165）；
  `iced = {features=[…,"image",…]}` 已启用（auto-lang Cargo.toml:199）；
  `Frame::draw_image(bounds, Image)` 存在（iced_graphics frame.rs:96-100，
  cfg image 门控），feature 链已通（iced_widget→iced_renderer→wgpu/
  tiny_skia→iced_graphics），双后端 `Backend::draw_image` 均实现；
  canvas `Image<H>` 带 filter_method/rotation/border_radius/opacity
  （iced_core image.rs:14-68）。**仓内零 canvas 画图先例**（唯一
  draw_image 是 code_editor advanced renderer API widget.rs:503）。
- **宿主图像解析/缓存现成**：`load_image_bytes`（renderer.rs:5840-
  5904）支持 `builtin:`/`data:` URI/`http(s)://`（reqwest 3s 超时）/
  本地文件；`get_or_create_image_handle`（:5908-5925，key=URL#radius#
  wxh 缓存防每帧重解码）；Handle 先例 from_rgba/from_bytes 多处。
- **缩略图源能力齐备**：snapshot.rs（request_capture :72/take_capture_
  requests :88/TTL+SWR :94-116/thumbnail_from_screenshot :141 降采样）；
  宿主窗整幅 screenshot × `wm.wins[wid].rect` 裁剪任意虚拟窗位图
  （renderer.rs:17343-17357）；`View::WindowThumbnail` 消费臂 :5146
  （快照命中 from_rgba 直绘 + 过期静默重抓 + miss fallback）。
- **两投影臂占位现状**：解释态 layout_image（client_runtime.rs:
  1340-1356，**src 显式丢弃** `let _ = props.get("src")`，占位 Quad +
  IMAGE_PLACEHOLDER :43-45）；native Image 臂（native_projector.rs:
  890-898，026 占位同口径，v1.8 保真注释 :891-893）。
- **View/codegen 面**：`View::Image{src, style}`（view.rs:818-822，无
  fit 字段——尺寸经 style fixed_w/h）；a2r codegen image 臂
  `View::image/image_styled`（rust.rs:3017-3062，src 绑定形状容差 =
  026 T-07 交付）；icon 降级 `image_styled("lucide:{name}")`（:3213）。
- **TS 渲染器**：messages.ts DrawOp 镜像 + decode tag 1–5（:134-162，
  **未知 tag throw**）+ render.ts renderFrame（:21-61，fillRect/
  clip/restore/fillText）+ golden 双侧钉（remote.rs ts_fixtures
  p508_ts_crosscheck_golden_bytes ↔ fixtures.golden.ts）；Canvas2D
  drawImage 可用但 TS 无 shm 通道。
- **保真口径在册**：desktop-protocol-v1.md §1.8 已知边界（:318-320）
  "位图真渲/图像 DrawOp 算子 not-yet（…图像通道归 shell a2r 设计
  §4-B 独立线）"；KNOWN-DEBT P026-D1（:2262，图像/字形真渲）。
- **版本表债**：desktop-protocol-v1.md 顶表停在 v1.4，§1.5–§1.8 已
  存在未回填（:14-20）——本计划 SD-01 顺带清偿。
- **协议版本纪律**：PROTOCOL_VERSION=1（mod.rs:67），v1.1–v1.8 追加式
  零翻版先例；027 落点在 projection-protocol（v1.10）非本协议。
- **e2e 载体**：004-profile-card（app.at:18 **远程 URL** src，:33
  image w-20 h-20=80×80——p026 断言其占位 80×80）；029-photo-gallery
  （app.at:104-125 本地文件 + 仓内 thumbnails 绝对路径）；043-
  clipboard-bridge（运行时剪贴板路径）；p026_native_display_arm 帧内
  定位法（stage3.rs:870-1139，quads_of/texts_of 谓词轮询）可直接扩
  image op 断言。
- **命令帧槽**：Commands 档 shm 槽 16KiB（session.rs:3524）——src 串
  为帧内增量，正常路径零压力（D6 极端截断语义核）。

**specs 现状**：协议权威 = desktop-protocol-v1.md（v1.8 现行 + 顶表
欠账）；B 形态裁定与前置序列 = desktop-shell-a2r.md（裁定落定节，
图像 op 为 [硬阻断] 首件）；模块 spec 026/027 provisional 在册。

## 5. 详细设计

### 5.1 T-01 深水调查定案（决策产物）

- **D1 op 形态与呈现参数**：`Image{rect, src}` + 最小集候选——fit
  （v1 = Stretch 拉伸至 rect（与占位尺寸盒语义同位）**倾向**；
  Cover/Contain = 投影器 Scissor+等比 rect 组合，另立精化）/
  filter_method（Nearest/Linear 缺省 Linear）/ border_radius（复用
  style rounded 的投影器推导 or op 字段）。同步语义定案：http src
  的 miss 处理（同步阻塞 3s vs 占位先行 + 缓存就绪后帧翻——**倾向
  后者**：观测行 + 下帧翻真，禁 UI 卡顿）。
- **D2 src 词汇与解析映射**：入册词汇 = `load_image_bytes` 四形态
  （file/builtin:/data:/http(s)://）+ `thumbnail://{wid}` 虚拟引用；
  **not-yet 词汇显式成文**（lucide:/svgdoc:/iconfile:/hicon:——字形
  真渲线）；未知 scheme = 未解析降级（I3）。
- **D3 thumbnail:// 解析形态**：宿主快照缓存命中 → draw_image；miss
  → fallback 占位 + `request_capture` 静默重抓（SWR 语义同
  WindowThumbnail 臂 :5146）+ 下帧翻——shell showdesk 前置语义直接
  对齐。
- **D4 降级与观测**：未解析 src（缺文件/超时/未知 scheme）→
  IMAGE_PLACEHOLDER Quad + 观测行（Log 通道 vs 宿主 stderr——按宿主
  观测面现状定）；同一 src 重复未解析的去重观测（防每帧刷行）。
- **D5 TS 策略**：decode tag 6 必达（否则 unknown throw 破会话）；
  renderFrame = 占位 fillRect（灰底）+ 注释成文 not-yet（web 真位图
  需 fetch/跨源/data 通道，独立增量）；golden 帧新增（含 Image op
  的最简帧）。
- **D6 帧尺寸与 src 上限**：16KiB 槽下 src 串上限语义（超长 URL =
  生成期/投影器侧截断 or 拒绝——定案入册）；极端场景度量行。

定案记录追加 `### 5.1 定案记录`，作为 T-02..T-07 依据。

### 5.1 定案记录（T-01，2026-09-18，lang worktree @ 64f157f33 实证）

**锚点重核**（§4 锚点取证于 2bdb8bb7d，本表为 64f157f33 复核结果——
全部成立，仅行号微漂者已更新）：DrawOp 五算子 message.rs:76-103、
encode :106-152 / decode :154-198 / 未知 tag 拒收 :194、下一可用
tag = 6；paint_ops broker_surface.rs:74-156；load_image_bytes
renderer.rs:5840-5904（`/api/__auto/media/` 票据 :5844 / builtin:
:5862 / data: :5869 / http 3s :5884-5896 / 本地文件 :5898-5899，
**含负缓存** :5902）；service_snapshot_requests renderer.rs:9290-9323
（drain → 整窗 screenshot Task → SnapshotShot 回调）；WindowThumbnail
SWR 消费臂 renderer.rs:5146-5210（stale → request_capture + 旧图续帧
+ `Handle::from_rgba` **每帧重建**）；快照编排泵挂 update drain 臂
renderer.rs:17429；桌面订阅面 renderer.rs:19015-19132（toast tick
250ms 条件订阅 / MCP 心跳 / listen_with 鼠标事件）；snapshot.rs 全套
（request_capture :72 / TTL 2s :32 / 冷却 500ms :61 / stale 读口
:116 / cache_put :107 / thumbnail_from_screenshot :141）；layout_image
client_runtime.rs:1340-1356（src 丢弃 :1350）；native Image 臂
native_projector.rs:890-898；IMAGE_PLACEHOLDER client_runtime.rs:45；
coverage 两表（解释态 :61/:74、native_queue_set :163-192）；ts_fixtures
remote.rs:420-552 五锚点；TS readDrawList messages.ts:129-166（未知
throw :162）/ render.ts :21-61 / fixtures.golden.ts / `vitest run`；
shm write_slot 超槽显式拒绝 shm.rs:225-233 + Commands 槽 16KiB
session.rs:3543；iced 0.14 `Frame::draw_image(bounds, impl Into<Image>)`
（iced_graphics frame.rs:96-100 cfg image）+ `Image{handle,
filter_method, rotation, border_radius, opacity}`（iced_core image.rs
:14-68，Handle→Image 缺省 Linear）；p026_native_display_arm stage3.rs
:883+（AUTO_DESKTOP_E2E 门 :884、quads_of :1029 / texts_of :1037、
004 占位 80×80 断言 :1052-1060）；载体 004（avatar_url 远程 URL
app.at:18、w-20 h-20 :33）+ 029（本地文件绝对路径 app.at:120-125，
`.at` 源已有 `fit:"cover"/"contain"` 但 View 层无 fit 字段——投影器
无 fit 信息可发）。

- **D1 op 形态与呈现参数**：`DrawOp::Image { rect: WRect, src: String,
  fit: ImageFit }`（tag 6）；`ImageFit` v1 仅 `Stretch = 1`（拉伸至
  rect——与占位尺寸盒同位），`from_u8` 未知值 UnknownTag 拒收
  （FrameMode/PixelFormat 纪律同款）。**op 字段定长，尾部追加不可行**
  （DrawOp decode 共享 Reader 无载荷尾判据——v1.3 Welcome 尾部追加
  先例仅适用载荷末字段），故 filter_method/border_radius **不入 wire**
  （宿主缺省 Linear、方形填充，与现占位零视觉差）；未来需要 = 新 tag
  （tag 语义冻结）。Cover/Contain 精化另立时同规（新 fit 值被旧端拒收
  → 届时新 tag）。**http miss 同步语义 = 占位先行 + 后台解码 + 下帧
  翻真**：paint 路径禁阻塞——file/builtin:/data:（本地字节）同步快
  路径当帧直绘；http(s):// miss 当帧占位 + spawn 后台线程
  load_image_bytes（自带进程缓存含负缓存，无重试风暴）→ Handle 缓存
  → 翻真由宿主任一后续重绘兑现（鼠标/250ms toast tick/MCP 心跳/帧泵
  驱动——**零新增触发器**，v1 语义成文；桌面 update 必经 view 重建 →
  canvas 重画）。
- **D2 src 词汇与解析映射**：入册 = load_image_bytes 词汇四形态
  （本地文件 / `builtin:ricepaper|inkwash` / `data:[mediatype][;base64],`
  / `http(s)://`）+ `/api/__auto/media/` 票据前缀随注（宿主本地管线
  词汇，投影器不特判）+ `thumbnail://{wid}` 虚拟引用（D3）。**not-yet
  词汇显式成文**：`lucide:`/`svgdoc:` = 字形真渲独立线（P026-D1 后半
  维持），本计划经 I3 未解析降级（icon 经 codegen 降级形态
  `image_styled("lucide:…")` 走 Image 臂 → 宿主解析不了 → 占位，行为
  与现占位口径连续）；未知 scheme = 未解析降级（I3）。
- **D3 thumbnail:// 解析形态**：**每帧 `snapshot_window_stale(wid)`
  直查，不进永久 Handle 缓存**（缩略需 SWR 刷新，冻结句柄锁死旧图；
  WindowThumbnail 臂 from_rgba 每帧重建同律）：命中（含过期）→
  `Handle::from_rgba` → draw_image；过期 → `request_capture` 静默重抓
  （SWR）；真 miss → request_capture + 占位当帧，重抓 cache_put 落地
  后下帧翻真。wid 非数字解析失败 = 未解析降级（D4）。
- **D4 降级与观测**：未解析（缺文件/负缓存/未知 scheme/字形词汇/wid
  非法）→ IMAGE_PLACEHOLDER 同色占位 Quad（client_runtime.rs:45 常量
  复用，视觉连续）+ 观测行 `[drawlist-image]` 前缀 eprintln +
  ui_console_push 双落（remote.rs enable_remote_ws 先例同款）；
  **去重 = 进程级 dedup 集**（src 首败打一行，后续静默；命中翻真时
  打一行 hit）。http 后台抓取失败入负缓存自然止血。
- **D5 TS 策略**：messages.ts DrawOp 联合增 `{kind:'image', rect, src,
  fit}`（tag 6 分支——不增即 unknown throw 破会话，防线必达）；fit u8
  镜像枚举（未知值 throw，与 Rust from_u8 对称）；render.ts image 臂
  = 灰底占位 fillRect（IMAGE_PLACEHOLDER 同值 rgb(60,60,70)）+
  **not-yet 注释成文**（web 真位图需 fetch/跨源/data 通道，TS 无 shm
  ——独立增量）；golden 新帧 `IMAGE_FRAME_HEX` 双侧钉（remote.rs
  ts_fixtures 增第六锚点）。
- **D6 帧尺寸与 src 上限**：**拒绝语义已在册，v1 不设投影器侧截断**
  ——Commands 槽 16KiB（session.rs:3543）+ shm `write_slot` 超槽显式
  报错（shm.rs:228-233 `payload N exceeds slot size`，无静默截断）。
  极端超长 src（超大 data: URL）= 帧编码超槽 → 响亮失败（I3 精神）；
  §1.9 成文 + T-07 度量行记帧字节增量 = src 串长。

**波及面普查**（新增变体的穷尽 match 点，编译期兜底）：paint_ops 外层
match（broker_surface.rs:78，T-03 增臂）、drawlist_to_text 测试 helper
（client_runtime.rs:2626，T-05 增臂）、message encode/decode（T-02）；
其余 DrawOp 消费点均为 filter_map/matches!（quads_of/texts_of/
scissor 计数）零波及。

### 5.2 wire 与 codec（T-02）

`DrawOp::Image` 变体 + encode/decode（tag 6）+ round-trip 单测 +
golden bytes（Rust 侧新帧样本：含 Image op 的最简帧）+ ts_fixtures
双侧钉（T-06 TS 侧对向）。

### 5.3 宿主栅格化与缩略图（T-03/T-04）

- **T-03**：broker_surface paint_ops Image 臂——src 解析序（thumbnail:
  > load_image_bytes 词汇）→ Handle 缓存（宿主侧全局缓存复用
  get_or_create_image_handle 同型，key 含 fit/radius 派生）→
  `Frame::draw_image`（bounds=rect，filter/border_radius 按 D1）；
  未解析降级占位 + 观测去重（D4）；http miss 异步翻帧（D1）。
- **T-04**：`thumbnail://{wid}` 解析——snapshot 缓存查询 + miss
  fallback/重抓（D3，复用 snapshot.rs 全套，不新建机制）；合成
  client 直测（发 Image{src:"thumbnail://N"} → 宿主缓存注入快照 →
  断言 draw_image 消费 + miss 降级路径）。

### 5.4 两投影臂升级（T-05）

解释态 `layout_image`（src 丢弃 → Image op，rect 推导零变化）与
native `Image` 臂（占位 → Image op）同刻度；三处占位保真注释核销
（client_runtime :43-45/:1340-1356、native_projector :886-893、
coverage.rs :164-191 随注）；golden 更新（既有占位断言改 Image op
断言——p026 帧内定位法样本改写归因随注）；降级占位转兜底语义。

### 5.5 TS 臂（T-06）

messages.ts 类型 + decode 分支；render.ts 占位渲染 + not-yet 注释；
fixtures.golden.ts 新帧 ↔ remote.rs ts_fixtures 对向；`pnpm test`
（或仓内 TS 测试命令按现状）。

### 5.6 e2e 与收口（T-07/T-08）

- e2e：`p028_image_arm`（AUTO_DESKTOP_E2E 门，p026 同型）——004
  （http URL 宿主代取，80×80 Image op 断言 + 离线降级腿）、029/
  fixture（本地文件真图，帧内 image op 定位 + 截图留痕 assets/
  028/）、thumbnail 合成 client 腿（T-04 集成）；既有 p026 断言
  改写回归。
- 度量：帧字节增量（src 串）数据行 + 宿主解码/缓存命中成本行
  （首帧 vs 后续帧时延）。
- 文档：desktop-protocol-v1.md **§1.9 v1.9 增量**（Image 算子/词汇
  表/降级纪律/TS not-yet）+ **顶表回填 v1.5–v1.8**；KNOWN-DEBT
  P026-D1 图像半句核销（字形半句维持 + 本计划 not-yet 三项入册）；
  os 台账行（B 前置第一件交付）；两仓互链。

### 规范增量

| delta_id | add/modify/retire | docs/specs/... target | before/after rule | rationale | acceptance IDs |
|---|---|---|---|---|---|
| SD-01 | add | auto-lang/docs/design/autoui/desktop-protocol-v1.md（§1.9 v1.9 增量 + 顶表回填 v1.5–v1.8） | before：DrawOp 五算子无图像，image/icon/avatar 占位保真（§1.8 边界"图像通道归独立线"）；after：`DrawOp::Image`（tag 6）入册——src 词汇表（file/builtin:/data:/http(s)/thumbnail://）+ 解析序与降级纪律（未解析=占位+观测）+ 两投影臂真渲口径 + not-yet 边界（位图过线/字形真渲/web 真位图）；PROTOCOL_VERSION 仍 1；顶表 v1.5–v1.8 欠账顺带清偿 | 协议权威版本化收录图像算子（B 形态硬阻断解除的第一件） | AC-01/02/03/05 |
| SD-02 | modify | auto-os/docs/plans/autos-desktop-program.md | before：B 前置序列"图像通道立项先行"未交付；after：登记交付行（tag 6/词汇/两臂真渲/thumbnail 能力 + not-yet 三项） | 桌面程序台账 | AC-07 |
| SD-03 | modify | auto-lang/docs/specs/auto-lang/ui/overview.md（review 期钉定——026/027 落地条目所在卷档；条目文本随 merge 沉淀，review 期不改 canonical） | before：026/027 provisional 无图像面条目；after：broker_surface Image 臂 + 两投影臂真渲 + 词汇/降级纪律条目（provisional） | 模块 spec 对齐实现 | AC-02/03/04 |

零 spec 影响的变更不存在（wire 算子/词汇/降级纪律为协议级知识）；
ledger（auto-lang `.autoos/specs.json`）随 merge 沉淀。

## 6. 测试设计

- **单测（message.rs）**：Image op round-trip（多 src 形态/fit 缺省/
  空 src）；golden bytes 新帧；未知 tag 拒收维持；src 上限语义
  （D6）。
- **单测（broker_surface）**：Image 臂分派（file/data: 解析 → Handle
  缓存命中断言）；未解析降级（缺文件/未知 scheme → 占位 Quad +
  观测去重）；http miss 异步翻（D1 定案后按形态）；thumbnail://
  命中/miss/重抓路径（缓存注入替身）。
- **单测（投影臂）**：解释态/native image golden 更新（占位 Quad →
  Image op，rect 零变化断言）；降级兜底路径（宿主未解析时端到端
  帧仍合法）。
- **TS**：decode tag 6 单测 + renderFrame 占位渲染 + golden 双侧
  对拍（fixtures.golden.ts ↔ ts_fixtures）。
- **集成/e2e**：`p028_image_arm`（004 http 主载 + 离线降级腿；029/
  fixture 本地文件；thumbnail 合成腿）；p026 既有断言改写回归；
  AUTO_DESKTOP_E2E 门 + 截图留痕 assets/028/。
- **度量**：帧字节增量 + 首帧/后续帧解码成本数据行。
- **回归门**：desktop_protocol/session/stage3/dual_mode + remote
  ts_fixtures + auto-os 桌面 smoke（desktop_mcp 链）。

## 7. 验收标准

- **AC-01 wire 与 golden**：Image op codec round-trip 绿；Rust↔TS
  golden bytes 双侧钉绿；既有五算子 golden 零漂移；未知 tag 拒收
  维持；`PROTOCOL_VERSION` 仍 1。验证：单测 + 双侧对拍。
- **AC-02 宿主真图渲染**：queue 臂 004-profile-card（http src 宿主
  代取）与本地文件样本（029/fixture）帧含 Image op，宿主画出真图
  （首帧 miss 翻真语义符合 D1）；截图留痕。验证：e2e 帧内定位 +
  assets/028/ + 单测缓存断言。
- **AC-03 降级纪律（I3）**：未解析 src（缺文件/超时/未知 scheme）=
  占位 Quad + 观测行（去重），无静默错绘、无 UI 卡顿。验证：单测
  + e2e 离线降级腿。
- **AC-04 缩略图虚拟引用**：`thumbnail://{wid}` 命中渲染 / miss
  降级+静默重抓+下帧翻真（SWR 语义）——合成 client 直测三路径。
  验证：单测（缓存替身）+ 集成。
- **AC-05 两臂 parity 与口径核销**：解释态/native 同刻度发 Image
  op（golden 同步更新）；占位保真三处注释核销、降级转兜底语义；
  p026 既有断言改写并归因。验证：golden + 注释核销清单。
- **AC-06 TS 面**：decode 不 throw（tag 6 帧 WS 会话可用）+ 占位
  渲染 + not-yet 成文 + golden 对拍绿。验证：TS 测试 + 双侧对拍。
- **AC-07 文档与回归门**：§1.9 + 顶表回填 + P026-D1 图像半句核销 +
  台账行 + 度量行落盘互链；§6 回归门全绿（在册既有红除外）。

## 8. 执行步骤

依赖序：T-01 → T-02 → {T-03, T-05 并行} → T-04 → T-06 → T-07 →
T-08。lang worktree `D:/autostack/.wt/lang-028/auto-lang`；os
`D:/autostack/.wt/os-028/auto-os`。**无前置计划依赖**（026/027 已
merge 即基线）。

- **T-01 [lang] 深水调查与定案** [x]
  文件：`desktop_protocol/message.rs`、`ui/iced/broker_surface.rs`、
  `ui/iced/renderer.rs`（load_image_bytes/缓存/snapshot 面，读）、
  `ui/iced/snapshot.rs`（读）、iced canvas Frame API 面（registry
  源）+ 本计划 §5.1（写面）。
  动作：D1–D6 定案（op 参数/词汇/thumbnail/降级/TS/尺寸上限）。
  产物：`### 5.1 定案记录`（file:line 证据）。
  验证：定案完备；复审通过。
  → AC-01..04 前置。新路径：是。
  [✅ 已完成] 2026-09-18：锚点 64f157f33 全量重核（§5.1 证据表）+
  D1–D6 六定案落笔（§5.1）；关键新事实 = op 字段定长不可尾部追加
  （共享 Reader 无载荷尾判据）→ filter/radius 不入 wire；http miss =
  占位先行+后台解码+零新增触发器翻真（订阅面 19015-19132 证据）；
  thumbnail 不进永久缓存（SWR 刷新语义）；write_slot 显式超槽拒绝
  = D6 现成。波及面 = 3 处穷尽 match（编译期兜底）。
- **T-02 [lang] wire op + codec + golden（Rust 侧）** [x]
  文件：`desktop_protocol/message.rs`（+ 测试）。
  动作：§5.2；tag 6 追加式。
  验证：round-trip + golden 单测绿；既有 golden 零漂移。
  → AC-01。
  [✅ 已完成] `DrawOp::Image{rect, src, fit}` + `ImageFit::Stretch(1)`
  （from_u8 拒收）encode/decode（message.rs）；`image_op_round_trip_
  and_golden`（src 四形态/空 src/直编 golden/未知 fit 拒收，载荷长锚点
  80）PASS；既有五算子 golden 零漂移（per_channel/scissor/textstyled
  均绿）。commit lang f89a0c5eb。
- **T-03 [lang] 宿主栅格化 + 缓存 + 降级** [x]
  文件：`ui/iced/broker_surface.rs`（Image 臂 + 解析序 + 缓存 +
  观测）；renderer.rs 缓存面复用/接驳。
  动作：§5.3 T-03；`Frame::draw_image` 仓内首用。
  验证：分派/缓存/降级单测绿（缓存替身）。
  → AC-02/03。
  [✅ 已完成] paint_ops Image 臂（draw_image(bounds, &handle) /
  IMAGE_PLACEHOLDER 占位）；`resolve_drawlist_image`（Handle 缓存
  key=src + 负缓存 + http_inflight 后台线程 + observed_unresolved
  去重，eprintln+ui_console 双落）；load_image_bytes 提权 pub(crate)
  单源复用。t028_* 四单测 PASS（data:/builtin:/负缓存/not-yet 词汇/
  http 占位先行后台落缓存）。
- **T-04 [lang] thumbnail:// 虚拟引用** [x]
  文件：`broker_surface.rs`（解析臂）、`snapshot.rs`（复用接驳，
  零新机制）。
  动作：§5.3 T-04；命中/miss/重抓三路径。
  验证：三路径单测 + 合成 client 集成。
  → AC-04。
  [✅ 已完成] resolve_thumbnail（snapshot_window_stale 每帧直查不进
  永久缓存 + 过期 request_capture SWR + miss 占位/request_capture）；
  snapshot.rs 增 `__test_backdate` 测试替身（__test_session 先例）。
  `t028_thumbnail_miss_hit_and_swr` 三路径 PASS；p028_image_arm 腿③
  cache_put 注入实驱 PASS。
- **T-05 [lang] 两投影臂真图升级** [x]
  文件：`client_runtime.rs`（layout_image）、`native_projector.rs`
  （Image 臂）、`coverage.rs`（随注）。
  动作：§5.4；golden 改写。
  验证：两臂 golden 绿 + 注释核销清单。
  → AC-05。
  [✅ 已完成] layout_image（src 字面量/绑定 read_state 代入，rect
  推导逐字零变化）+ native `View::Image{src, ..}` 臂同刻度；三处
  注释核销（IMAGE_PLACEHOLDER 转降级兜底语义/client_runtime+native
  臂随注/coverage 两表随注）。golden 改写归因：climb_004_*_image_op、
  display_family_placeholder_golden（Image op 断言）、t1_display
  金样（仅 quad→image 行，几何逐字节不变）、stage3 images_of 定位器
  ——全绿。
- **T-06 [lang] TS 渲染器臂** [x]
  文件：`packages/drawlist-renderer/src/{messages,render}.ts` +
  `test/fixtures.golden.ts` + Rust 对向 `remote.rs` ts_fixtures。
  动作：§5.5。
  验证：TS 测试 + 双侧 golden 对拍绿。
  → AC-01/06。
  [✅ 已完成] messages.ts tag 6 分支 + `{kind:'image', rect, src,
  fit}`；render.ts 占位灰底 + web 真位图 not-yet 成文；IMAGE_FRAME_HEX
  ↔ remote.rs 第六锚点双侧钉；未知 tag 拒收测试维持。`pnpm test`
  27/27 绿；`p508_ts_crosscheck_golden_bytes` 全量档绿。
- **T-07 [lang+os] e2e 与度量** [x]
  文件：lang `stage3.rs`（p028_image_arm + p026 断言改写）+ 截图
  `docs/plans/reports/assets/028/`；os smoke 腿（如需）。
  动作：AC-02..05 逐条跑通留痕 + 度量行。
  → AC-02/03/04/05。
  [✅ 已完成] `p028_image_arm`（AUTO_DESKTOP_E2E=1 PASS）：①004 真子
  进程 queue 帧 80×80 cravatar op；②p028 语料（capability-tests 新
  件——029 源解释态编译器不可 parse，探针实证 20 错，按 §5.6
  "029/fixture" 措辞由 fixture 承载）五 src 形态一帧全数入帧；③宿主
  侧三路径实驱（thumbnail 注入命中/本地文件解析+缓存/离线负缓存）；
  度量行 frame_bytes=365 src_bytes=245 ops=5 + 解码成本行；AUTO_028_
  ASSETS 帧留痕 assets/028/ 两件落盘。真像素截图腿（ui_desktop 桌面
  进程 canvas 栅格）留 merge 后 smoke 承载（smoke-026 同款脚本形态，
  本期 draw_image 首用保真由双后端 feature 链 + e2e 帧断言背书）。
- **T-08 [lang+os] 文档与台账收口** [x]
  文件：lang `desktop-protocol-v1.md`（§1.9 + 顶表回填）、
  `KNOWN-DEBT-AND-RISKS.md`（P026-D1 半句核销 + not-yet 三项）；
  os `autos-desktop-program.md`（台账行）+ 两仓互链。
  动作：SD-01..03 落笔。
  → AC-07。
  [✅ 已完成] §1.9 六条 + 顶表 v1.5–v1.9 回填（v1.5–v1.8 日期 git
  pickaxe 取证）+ §1.8 边界核销随注；KNOWN-DEBT P026-D1 图像半句
  核销 + 增补五（P028-D1..D4 入册）；os 台账 3c1 行（os plan-028-dev
  1eed4ed）；模块 spec（SD-03 provisional）留 review 期定稿落笔。

## 9. 复审记录

- 2026-09-18 /auto-plan:new 起草交接：`stage: new`，PLAN-028 rev 1。
  `outcome: pass`（合同完整：DrawOp/codec tag 位、canvas draw_image
  能力链、宿主解析/缓存/snapshot 基建、两投影臂占位现状、TS 对拍
  义务全部 file:line 在案；src 引用路线避开位图过线的 v1 判断有
  普查依据）；`next: work`（**无前置计划依赖**，T-01 可即行）。
  悬置决策登记 §10（①–⑤），均不阻塞 T-01 开工。
- 2026-09-18 /auto-plan:work 进入：`stage: work`，授权 = 用户本轮
  "计划028 实施"。worktree 落位：os `D:/autostack/.wt/os-028/auto-os`
  （branch `plan-028-dev` @ os main 2de26fc）+ lang
  `D:/autostack/.wt/lang-028/auto-lang`（branch `plan-028-dev` @
  lang master 64f157f33）。主检出 WIP 预检：auto-os 主检出 ui-gallery
  demos/registry + widgets-gallery 缓存有他属未提交改动；auto-lang
  主检出 `examples/rust-workspace/Cargo.toml` 有他属未提交改动——
  均**不建其上不并收**，本计划全部编辑在 worktree 内。基线漂移注：
  §4 锚点取证于 lang 2bdb8bb7d，现基线 64f157f33（646 archived），
  T-01 深水调查全部重核。
- 2026-09-18 /auto-plan:work 收口：`stage: work`，PLAN-028 rev 1。
  `outcome: pass`。code_commit：lang plan-028-dev **f89a0c5eb**
  （T-02..T-07 实现+e2e+文档）+ os plan-028-dev **1eed4ed**（台账
  3c1 行）。task_ids：T-01..T-08 全勾（§8 逐条证据）。
  evidence：①AC-01 message.rs tag 6 round-trip/golden/未知 fit 拒收
  绿 + IMAGE_FRAME_HEX 双侧对拍绿 + 既有 golden 零漂移 + PROTOCOL_
  VERSION 仍 1；②AC-02 p028_image_arm 腿①②（004 http 80×80 op +
  语料五形态帧）+ resolve 单测缓存断言 + assets/028/ 留痕；③AC-03
  负缓存/未解析占位/观测去重单测 + e2e 离线腿（不可达 http 占位
  当帧）；④AC-04 thumbnail 三路径单测（__test_backdate 替身）+
  e2e cache_put 注入实驱；⑤AC-05 两臂 golden 改写全绿 + 三处注释
  核销 + p026 断言改写归因；⑥AC-06 TS 27/27 + decode 防线 + not-yet
  成文；⑦AC-07 §1.9/顶表回填/KNOWN-DEBT/台账 3c1 落笔 + 回归门
  全绿（全量失败集与 master 基线 40 项逐一全等——ui-iced 组合既有
  红族含 musk p053×4，P645-D2 同族在册；gated p028_image_arm 另跑
  PASS）。环境/路线记录：029 源解释态编译器不可 parse（探针 20 错）
  → 本地文件腿按 §5.6 "029/fixture" 由 capability-tests/p028-image-
  channel 语料件承载；依赖组补 auto-down detached worktree
  （lang-022 先例）；真像素截图腿（ui_desktop canvas 栅格）留 merge
  后 smoke 脚本承载。blockers：无。next：review。
- 2026-09-18 /auto-plan:review 复审：`stage: review`，PLAN-028 rev 1。
  `outcome: pass` → status **reviewed**。**独立性声明**：复审在实施
  同会话内进行——结论从工件重建（提交 diff / 测试复跑 / 基线对拍），
  不依赖执行者摘要。
  reviewed_commit：lang f89a0c5eb（worktree plan-028-dev，净树）/
  os 1eed4ed（worktree plan-028-dev，净树）。base_commit：lang
  64f157f33（master）/ os 2de26fc（main）。dependency_revisions：
  auto-down 65279a2（detached 组内 worktree，零修改）。
  **diff 审查**：message.rs **零删除行**（git diff 64f157f33..f89a0c5eb
  实证）——tag 1–5 encode/decode 逐字节冻结，Image = 纯追加（I1）；
  两投影臂 rect 推导行逐字不动（I2）；renderer.rs 仅可见性提权。
  **acceptance_results**（逐条复跑）：AC-01 pass（message scoped 绿；
  p508_ts_crosscheck 复跑 PASS；PROTOCOL_VERSION=1 在码）；AC-02
  pass（p028_image_arm 复跑 PASS——004 真 80×80 op + 语料五形态；
  assets/028/ 留痕随 f89a0c5eb 入库）；AC-03 pass（负缓存/降级单测
  + e2e 离线腿）；AC-04 pass（thumbnail 三路径单测 + e2e 注入实驱）；
  AC-05 pass（改写 golden 全绿 + 三处核销随注 + native Image op
  出帧由 display_family golden 承载）；AC-06 pass（TS 27/27 复跑）；
  AC-07 pass（§6 回归门见下）。**回归门**（复审档）：日常档 ui-iced
  全量 5141 测 = 5101 绿 + 40 红与 master 基线**双向全等**
  （comm 双向空，最终轮 /tmp 对拍）；cargo tf（全配置无 ui-iced）
  wt 2 红 ⊆ master 3 红（差集空）；cargo tt wt 6 红 = master 6 红
  （双向全等——a2r 真编译门/codegen fixture 族为无 ui-iced 特性
  组合双方共有红，非本计划回归）；cargo tv/tb 豁免（改动面零
  VM/book——全部实现居 ui-iced 门内，无 ui-iced 构建不含本 diff）。
  **spec_inputs**：desktop-protocol-v1.md §1.9 + 顶表 v1.5–v1.9
  （随 f89a0c5eb；六条主张逐一对码核验——词汇表/3s 超时/TTL 2s/
  冷却 500ms/16KiB write_slot 拒绝/占位色同值三方一致）；SD-02
  台账行随 os 1eed4ed；SD-03 目标钉定 overview.md（merge 期沉淀）。
  **findings**：无阻断项。备注：covered_elements_within_target_set
  为 master 基线既有红（/tmp/m-u.txt 在册），非本计划面。evidence
  持久化：f89a0c5eb / 1eed4ed 提交本体 + assets/028/ 帧留痕 + 本节
  汇总行。next：**merge**。

## 10. 待澄清事项

- **①（T-01 D1）** fit 语义 v1：Stretch 拉伸至 rect（推荐——与占位
  尺寸盒同位）vs Cover/Contain 组合（投影器 Scissor+等比 rect，另立
  精化）；http miss 异步翻帧（推荐）vs 同步阻塞。
- **②（T-01 D2）** not-yet 词汇边界：lucide:/svgdoc: 字形真渲维持
  P026-D1 后半（推荐——独立线）；未知 scheme = 降级（I3）。
- **③（T-01 D3）** thumbnail:// 本期交付（推荐——宿主机具齐备、B
  硬前置能力可先行直测）vs 留 shell B 程序。
- **④（T-01 D5）** TS 策略：decode 必达 + 占位渲染（推荐）——web
  真位图（fetch/跨源）not-yet 成文。
- **⑤（T-01 D6）** src 串上限语义（16KiB 槽内极端 URL：截断 vs
  拒绝）——定案入册。
