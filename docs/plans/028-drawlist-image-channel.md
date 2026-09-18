---
plan_id: PLAN-028
status: drafting               # drafting → executing → execution_done → reviewed → archived
feature_name: drawlist-image-channel
author: [agent]
created_at: 2026-09-18
updated_at: 2026-09-18
plan_revision: 1

# /auto-plan:review 结束时填写：
supersedes_spec_components: []
new_spec_components:
  - auto-lang/docs/design/autoui/desktop-protocol-v1.md   # §1.9 v1.9 增量 + 版本表回填（review 定稿）
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
current_step: 0
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
| SD-03 | modify | auto-lang/docs/specs/auto-lang/ui/（review 期按目录实况定） | before：026/027 provisional 无图像面条目；after：broker_surface Image 臂 + 两投影臂真渲 + 词汇/降级纪律条目（provisional） | 模块 spec 对齐实现 | AC-02/03/04 |

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

- **T-01 [lang] 深水调查与定案**
  文件：`desktop_protocol/message.rs`、`ui/iced/broker_surface.rs`、
  `ui/iced/renderer.rs`（load_image_bytes/缓存/snapshot 面，读）、
  `ui/iced/snapshot.rs`（读）、iced canvas Frame API 面（registry
  源）+ 本计划 §5.1（写面）。
  动作：D1–D6 定案（op 参数/词汇/thumbnail/降级/TS/尺寸上限）。
  产物：`### 5.1 定案记录`（file:line 证据）。
  验证：定案完备；复审通过。
  → AC-01..04 前置。新路径：是。
- **T-02 [lang] wire op + codec + golden（Rust 侧）**
  文件：`desktop_protocol/message.rs`（+ 测试）。
  动作：§5.2；tag 6 追加式。
  验证：round-trip + golden 单测绿；既有 golden 零漂移。
  → AC-01。
- **T-03 [lang] 宿主栅格化 + 缓存 + 降级**
  文件：`ui/iced/broker_surface.rs`（Image 臂 + 解析序 + 缓存 +
  观测）；renderer.rs 缓存面复用/接驳。
  动作：§5.3 T-03；`Frame::draw_image` 仓内首用。
  验证：分派/缓存/降级单测绿（缓存替身）。
  → AC-02/03。
- **T-04 [lang] thumbnail:// 虚拟引用**
  文件：`broker_surface.rs`（解析臂）、`snapshot.rs`（复用接驳，
  零新机制）。
  动作：§5.3 T-04；命中/miss/重抓三路径。
  验证：三路径单测 + 合成 client 集成。
  → AC-04。
- **T-05 [lang] 两投影臂真图升级**
  文件：`client_runtime.rs`（layout_image）、`native_projector.rs`
  （Image 臂）、`coverage.rs`（随注）。
  动作：§5.4；golden 改写。
  验证：两臂 golden 绿 + 注释核销清单。
  → AC-05。
- **T-06 [lang] TS 渲染器臂**
  文件：`packages/drawlist-renderer/src/{messages,render}.ts` +
  `test/fixtures.golden.ts` + Rust 对向 `remote.rs` ts_fixtures。
  动作：§5.5。
  验证：TS 测试 + 双侧 golden 对拍绿。
  → AC-01/06。
- **T-07 [lang+os] e2e 与度量**
  文件：lang `stage3.rs`（p028_image_arm + p026 断言改写）+ 截图
  `docs/plans/reports/assets/028/`；os smoke 腿（如需）。
  动作：AC-02..05 逐条跑通留痕 + 度量行。
  → AC-02/03/04/05。
- **T-08 [lang+os] 文档与台账收口**
  文件：lang `desktop-protocol-v1.md`（§1.9 + 顶表回填）、
  `KNOWN-DEBT-AND-RISKS.md`（P026-D1 半句核销 + not-yet 三项）；
  os `autos-desktop-program.md`（台账行）+ 两仓互链。
  动作：SD-01..03 落笔。
  → AC-07。

## 9. 复审记录

- 2026-09-18 /auto-plan:new 起草交接：`stage: new`，PLAN-028 rev 1。
  `outcome: pass`（合同完整：DrawOp/codec tag 位、canvas draw_image
  能力链、宿主解析/缓存/snapshot 基建、两投影臂占位现状、TS 对拍
  义务全部 file:line 在案；src 引用路线避开位图过线的 v1 判断有
  普查依据）；`next: work`（**无前置计划依赖**，T-01 可即行）。
  悬置决策登记 §10（①–⑤），均不阻塞 T-01 开工。

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
