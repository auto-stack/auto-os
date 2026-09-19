---
plan_id: PLAN-034
status: drafting               # drafting → executing → execution_done → reviewed → archived
feature_name: rqhost-maturity
author: [agent]
created_at: 2026-09-20
updated_at: 2026-09-20
plan_revision: 1

# /auto-plan:review 结束时填写：
supersedes_spec_components: []
new_spec_components:
  - auto-lang/docs/design/autoui/desktop-protocol-v1.md   # §1.15 v1.15 增量（review 定稿）
touched_goals: []

affects:
  - auto-lang/crates/auto-lang/src/ui/desktop_protocol/message.rs        # FrameMsg tag 10（BitmapReady/Ack）+ BufferAlloc 尾追（依 D3）
  - auto-lang/crates/auto-lang/src/ui/desktop_protocol/{shm,endpoint,client_runtime}.rs # 位图槽/段 + 泵扩展
  - auto-lang/crates/auto-lang/src/ui/iced/broker_surface.rs            # bitmap:// 前缀臂 + handle_cache 上限（依 D5）
  - auto-lang/crates/auto-lang/src/ui/desktop_protocol/rqhost.rs        # daemon 自观测 + 位图消费 + release 复测配合
  - auto-lang/crates/auto-lang/src/ui/desktop_protocol/{native_projector,mod}.rs # RqProjector 位图引用消费面（依 D3）
  - auto-lang/packages/drawlist-renderer/                                # TS decode tag 10（占位维持）
  - auto-lang/crates/auto-lang/src/ui/desktop_protocol/stage3.rs        # p034 e2e（内存矩阵 + 位图腿）
  - auto-lang/docs/design/autoui/desktop-protocol-v1.md                  # §1.15 + 顶表 v1.15
  - auto-lang/docs/plans/KNOWN-DEBT-AND-RISKS.md                         # P028-D1/D3、P-RQ-PIX 处置
  - auto-os/docs/plans/autos-desktop-program.md                          # M7 副线债行更新
current_step: 0
total_steps: 8
---

# [PLAN-034] rqhost-maturity

## 0. 变更摘要

**"RQHost + RqProjector 完整化"第二件**（033 之后；消费端 + 协议词汇
面），三件事：

**①rqhost 内存达标（量化门 ≤100MB）**——"先测再优化"：release 复测
零代码（度量/采样/落痕基建全在，唯一缺口 = e2e 载体定位器 debug 优先
序 mod.rs:152-153 需规避）→ 归因矩阵（debug/release × wgpu/
tiny-skia[A/B 实验，`ICED_BACKEND` 环境开关 iced 原生现成] × N 窗边际
[1/2/5]）→ 按需优化（handle_cache 无界加 LRU / daemon 自观测面
[mem_guard 未接 rq_update] / canvas Cache 局部化降 15ms tick 每帧分配）。
**②位图过线通道（P028-D1 兑现）**——普查定案最小面：`bitmap://{id}`
src 前缀走**既有 Image op 词汇表**（零 DrawOp 新 tag，宿主解析单点加
前缀臂）+ FrameMsg **tag 10**（BitmapReady/Ack——元数据过管道、位图
块在 shm 槽/专用段，镜像 FrameReadyShared 槽纪律）；app 生成像素内容
（canvas/终端/动态图）从此经图像算子流过协议。**③像素原生族裁定
（P-RQ-PIX 前置④）**——基于消费现实的五 kind 处置：video 裁
"inproc/独立窗专属"（mpv-widget wgpu-only + 24fps shm 流量级不经济）、
imagesurface 裁缩（零消费者）、terminal 留 M7-c 撞面裁定（auto-term
真需求，位图快照+命中回传路径）、code_editor 维持 not-yet 家族
（P032-D3 在册）、canvas 按位图快照过线（样板级验证）。

`PROTOCOL_VERSION` 维持 1（全部追加式：tag 10 + 词汇前缀 + 可选
BufferAlloc 尾追）。

## 1. 目标

- **G1 内存量化门**：daemon release private ≤100MB（031 debug 实测
  314MB）；app ≤10MB 沿用复核（033 实测 003 -q=8MB / 027 -q=20MB——
  027 超门按 P033-D3 另裁口径复核）。**dual-exit 数据门**：release
  复测达标 → 优化项转可选；不达标 → 归因矩阵定位 + 优化项执行至
  达标（或显式重定标留痕）。
- **G2 归因矩阵（资产，无论达标与否）**：{debug, release} × {wgpu,
  tiny-skia} × {1/2/5 窗} 的 daemon private 数据行落盘——wgpu 设备
  驻留/每窗 surface/cosmic-text fontdb/缓存四嫌疑位分摊。
- **G3 位图过线通道**：FrameMsg tag 10（BitmapReady{surface,id,w,h,
  stride,slot/len}/BitmapAck）+ shm 位图槽/段（D3 定案：专用第二段
  vs BufferAlloc 尾追）+ `bitmap://` 词汇前缀（宿主 resolve 单点臂 +
  handle_cache 键）+ app 侧上传 API（RqProjector/组件面喂图接口）；
  合成位图生产者全链验证（动画位图 app `-q` 真渲）；TS decode tag 10
  必达 + 占位维持（web 真位图 not-yet 边界更新）。
- **G4 像素原生族裁定入册**：五 kind 处置行（§0 ③）落协议 §1.15 +
  台账 + KNOWN-DEBT——P-RQ-PIX 前置④达成；canvas 位图快照过线作为
  通道的首个真实消费样板（043-canvas-paint 级验证，依 D4 取舍）。
- **G5 收口**：desktop-protocol-v1.md **§1.15 v1.15 增量**（位图通道
  全语义 + 五 kind 裁定 + 内存门结论）；P028-D1 核销 / P028-D3 边界
  更新 / P-RQ-PIX 条件③④达成注记；台账 M7 副线债行更新。

**非目标**（明确出界）：

- **native pixels 臂本体的删除**（P-RQ-PIX 门后执行——本计划交付
  前置③④与量化门，删除另立收尾件）。
- **terminal 过线实现**（M7-c 特殊线撞面裁定+立项——本计划只裁路径
  与通道就位）；**video 过线**（裁"独立窗专属"后不做）；code_editor
  家族扩展。
- **web 真位图渲染**（TS 侧 fetch/data 通道——not-yet 边界维持，
  decode 防线义务除外）。
- M7-b（shell 编译面）/M7-c（app 批量化）——并行波次；**e2e 载体定位
  器的 release 缺省化**（只为本计划加规避参数/env，全局改另议——
  debug 优先序是 031 以来的构建时间权衡）。
- a2r codegen 为 canvas/video 补臂（canvas 快照生产者走宿主外路径依
  D3/D4 定案，不扩 codegen 面）。

## 2. 架构方案

```text
┌─ 内存臂（rqhost.rs + stage3.rs 仪器）───────────────────────────────┐
│ release 复测：cargo build --release 产 target/release/auto.exe      │
│   + e2e 定位器规避（env 覆盖优先序，desktop.ps1 release 优先先例）  │
│ 归因矩阵：ICED_BACKEND=tiny-skia A/B（iced 0.14 fallback 原生       │
│   env，零代码）× N 窗边际采样（压测 harness 先例）× debug/release   │
│ 按需优化（依数据）：handle_cache LRU / rq_update 接 mem_guard 自    │
│   观测 / canvas Cache 局部化（Stage 5 在册遗留 broker_surface:11）  │
└──────────────────────────────────────────────────────────────────┘
┌─ 位图通道臂（message.rs + shm/endpoint + 两端）─────────────────────┐
│ app 侧：bitmap 生产 API（RqProjector/组件面）→ 写 shm 位图槽        │
│   → FrameMsg tag 10 BitmapReady{surface,id,w,h,stride,slot,len}     │
│   （元数据过管道——镜像 FrameReadyShared 槽纪律）                    │
│ 宿主侧：BitmapAck 归还槽 → 位图入 handle_cache（键 bitmap://{id}）  │
│ DrawList 侧：Image{src:"bitmap://{id}"} ——零新 op tag，             │
│   resolve_drawlist_image 前缀臂单点；失效/更新 = 同 id 重上传覆盖   │
│ TS：decode tag 10 必达（unknown throw 防线）+ 占位渲染维持          │
└──────────────────────────────────────────────────────────────────┘
┌─ 裁定臂（文档面）────────────────────────────────────────────────────┐
│ 五 kind 处置行：video=inproc/独立窗专属（wgpu-only+带宽）｜         │
│ terminal=M7-c 撞面（auto-term 真需求；位图快照+命中回传）｜         │
│ canvas=位图快照过线（043 样板验证）｜code_editor=not-yet 家族维持｜ │
│ imagesurface=裁缩（零消费者，P032-D3 关联）                         │
└──────────────────────────────────────────────────────────────────┘
```

**不变式**：

- **I1 追加式协议**：`PROTOCOL_VERSION` 仍 1——tag 10 新变体（旧端
  不产不识则按未知拒收，会话隔离）+ 词汇前缀（src 字符串空间）+
  BufferAlloc 尾追（消息级尾追合法先例 :542-546）；既有 1-9 tag 与
  golden 零漂移。
- **I2 零回归**：RqProjector/-q 两轨/桌面/rqhost 机器零行为变化
  （位图通道是新增能力，无既有路径改写）；release 复测只加观测。
- **I3 降级纪律**：TS 位图占位维持成文；未实现 kind 处置全部显式
  入册（裁定的"专属/裁缩"也是显式边界，非静默）。
- **I4 数据门纪律**：内存达标判定仅凭 release 复测数据行；
  dual-exit（达标 → 优化转可选 / 不达标 → 归因+优化至达标或重定标
  留痕），禁拍脑袋定标。

**关键风险**：release 复测仍超标的可能性（debug 314 → release 若只
降到 ~150MB，需归因优化真做——wgpu 设备驻留是大头嫌疑，tiny-skia
常驻又砍 mpv 视频面，两难留给数据说话）；位图段的槽策略（专用第二
段 vs BufferAlloc 尾追——D3 定案，段名/尺寸协商面）；高频位图流
（视频级 24fps）的背压语义 v1 不做（D3 随注——本计划消费面是 canvas
快照级低频）；`bitmap://` 失效语义（同 id 覆盖 + 宿主缓存即时翻新，
无版本号——T-01 钉死）。

## 3. 技术栈

Rust / iced 0.14（fallback 链 + `ICED_BACKEND` env 原生开关）；既有
shm/槽/泵机器（FrameReadyShared 同构）；K32GetProcessMemoryInfo 采样
器（stage3.rs:114-212）+ p031/p033 e2e 度量臂；desktop.ps1 release
优先解析序先例；验收载体 = 合成位图生产者（动画 gradient/clock 级
测试组件）+ 043-canvas-paint（canvas 快照样板，依 D4）+ 003/001
（内存矩阵陪跑）。

## 4. 需求分析与背景调查

**授权记录**：用户 2026-09-20 会话明确"计划 033 正在归档，现在规划
计划 034"（按既定路线：RQHost+Native 臂完善第二件 = rqhost 成熟化；
量化门 app ≤10MB / rqhost ≤100MB 为用户 2026-09-19 定标）。**本轮
仅规划，未授权实施**。涉及仓：auto-lang（协议/通道/宿主/文档）+
auto-os（台账）。无预算/自动续跑约束声明。

**前置依赖**：033 已归档 ✅（RqProjector 在 master、AppProjector 已
退役、§1.14 现行）；无其他硬前置（与 M7-b/M7-c① 文件面错开可并行）。

**现状事实**（已核，2026-09-20 master 含 033，探索代理全量普查）：

- **314MB 出处与口径**：`docs/plans/reports/assets/031/inventory.txt`
  —— rqhost `working_set=200504KB / private=314136KB`（**PrivateUsage
  口径**，debug 构建）；两 app 8532/6876KB。033 对照行
  assets/033/memory-comparison.txt：003 直挂 private 225MB vs -q
  8MB；027 -q 20.4MB（P033-D3 另裁口径在册）。
- **采样与 e2e 基建**：`sample_process_memory`（stage3.rs:114-212，
  K32 FFI 双字段）；p031_rqhost_arm :4843-5015（度量腿 :4980-4999）；
  p033_rq_unify_arm :5315-5444+；**e2e 载体定位器 debug 优先**
  （e2e_exe::locate_with_stale_guard mod.rs:152-153——release 复测
  需规避：产出 release 产物 + 探测序覆盖；desktop.ps1:45-49 有
  release 优先解析序先例）；release 无自动化构建脚本（手工
  `cargo build --release` 先例为零——本计划补最小脚本/文档行）。
- **后端开关（零代码）**：iced 0.14 fallback 链 = wgpu primary →
  tiny-skia 兜底（iced_renderer fallback.rs:267-322）；**`ICED_
  BACKEND` env 原生支持**（fallback.rs:275-276；tiny-skia 认
  "tiny-skia"/"tiny_skia"）→ A/B 归因实验开关现成。注意：mpv 视频面
  是 wgpu-only primitive（ui/mpv/widget.rs:236）——tiny-skia 常驻会
  砍视频直挂（rqhost 现不渲视频，无现实冲突，但裁定时记）。
- **daemon 内存嫌疑位**：①debug 构建本身（同引擎 app 直挂 225MB vs
  -q 8MB——渲染栈只在 daemon）；②wgpu 设备/适配器驻留 + 每窗
  surface；③cosmic-text fontdb；④handle_cache **无界无 LRU**
  （broker_surface.rs:55-61 裸 HashMap，lucide 键含尺寸发散面）；
  ⑤15ms tick 每帧全量几何重建无 canvas Cache（broker_surface.rs:330-
  355 + :10-11 在册遗留）；daemon **零自观测面**（mem_guard 只接
  inproc 渲染器 renderer.rs:25106-25137，rq_update 未接）。
- **DrawOp/Image 现状（032/033 零触碰）**：`Image{rect, src, fit}`
  （message.rs:103-111；**op 定长不可尾追原文 :107-110**——未来呈现
  参数 = 新 tag）；tag 6 已用，**下一可用 = 7**（但本计划免用——
  `bitmap:` 走 src 词汇）；src 前缀分派 resolve_drawlist_image
  （broker_surface.rs:104-155：thumbnail://→workspace://→lucide:→
  缓存→http 后台→本地族）；not-yet 词汇：svgdoc:（§1.10 成文）。
- **shm/泵可复用面**：SharedFrameBuffer（槽数 2/Commands 槽 16384/
  段名 autodesk-shm-<pid>-<surface>）；FrameReadyShared 槽循环
  （endpoint.rs:225-252 产 → 宿主 ComposeFrameShared 读+FrameAck
  归还）；超槽回退管道内联先例（client_runtime.rs:653-673）；
  BufferAlloc `shm: Option<String>` 即尾部追加字段先例
  （message.rs:542-546——消息级尾追合法）。**FrameMsg 下一可用
  tag = 10**（1-9 已用，message.rs:599-608）；ControlMsg 下一可用
  = 15。
- **像素原生五 kind 消费现实**（裁定基础）：terminal——auto-term
  真需求（M7-c 特殊线，台账 :104），a2r 有直构臂（rust.rs:2669-
  2823），inproc 渲染器完整；video——030-video-player 唯一消费
  （设计 §4.1 SW 渲染裁定），a2r **无臂**，mpv-widget wgpu-only；
  canvas——唯一消费 043-canvas-paint 样板，a2r 无臂，inproc =
  CanvasPainter（"不经 DrawOp 线协议 v1 无路径 op" renderer.rs:4809-
  4811 在册）；code_editor——唯一消费 041，015-notes 已弃，P032-D3
  not-yet 家族在册；imagesurface——**零消费者**（grep .at 零命中）。
- **queue 臂现状**：五 kind 全不在 native_queue_set（coverage.rs:
  175-231）→ 覆盖门拒绝（vm -q 硬编 Commands 无 auto 降级
  rqhost.rs:591-611 + client_entry.rs:102-108）或 a2r auto 降级
  independent（client_entry.rs:201-209）。
- **TS 面**：Image op 占位渲染 + web 真位图 not-yet 成文
  （render.ts:50-55 / §1.9 D5）；unknown tag throw 防线（messages.ts
  decode）——新 tag 10 TS decode 义务。
- **债账现状**：P028-D1 位图过线 not-yet（KNOWN-DEBT :2338）；
  P028-D3 web 真位图（:2340）；P-RQ-PIX 四条件 + 量化门（:2273）；
  P033-D1..D4（:2413-2420）；台账 :118/:120 副线行（033 后已更新，
  "release 复测+归因优化 = PLAN-034 前置"在册）。协议版本表 :30 =
  v1.14；**§1.14 后即 §2——034 落 §1.15 + 顶表行**。

## 5. 详细设计

### 5.1 T-01 深水调查定案（决策产物）

- **D1 release 复测口径**：release 产物产出方式（构建脚本/文档行）+
  e2e 定位器规避（env 覆盖 vs 临时探测序参数——倾向 env：零侵入，
  desktop.ps1 先例同型）；采样矩阵脚手架（p034 度量腿复用
  sample_process_memory）。
- **D2 归因矩阵设计**：{debug,release} × {wgpu,tiny-skia} × {1,2,5
  窗} 数据行格式；fontdb/缓存分摊的观测法（若 tiny-skia A/B 不够
  分离——加缓存清零对照腿）；达标判定序（release 单测 → 不达标再
  矩阵 → 优化项取舍 D5）。
- **D3 位图通道定案**（核心）：段策略——候选 A = **专用第二段**
  `autodesk-shm-<pid>-<surface>-bm`（BufferAlloc 之外新开，槽数/
  尺寸协商随首个 BitmapReady 惰性 or Welcome 尾追声明；**倾向**：
  与主段解耦，尺寸不受 16KiB Commands 档约束）/ B = BufferAlloc
  尾追 `bm_shm` 字段（生命周期复用但三处宿主消费面全动）；
  BitmapReady/Ack 语义（tag 10：{surface, id, w, h, stride, slot,
  len}；Ack 归还槽同 FrameAck 纪律）；`bitmap://{id}` 失效/覆盖
  语义（同 id 重上传 → 宿主缓存即时翻新——无版本号，T-01 钉死
  单写者时序）；app 侧上传 API 形态（RqProjector 公开 bitmap 上传
  方法 vs 组件 trait 钩子——消费面是 VM canvas 快照/a2r 自绘，接口
  最小化）；高频流背压 v1 not-yet 随注。
- **D4 像素原生族裁定草案**（用户确认后入册）：video = inproc/
  独立窗专属（wgpu-only + 24fps 带宽）；terminal = M7-c 撞面立项
  （位图快照 + 命中坐标回传路径预留）；canvas = 位图快照过线（043
  样板验证——VM 轨 CanvasScene → 宿主外快照生产者依 D3 API）；
  code_editor = not-yet 家族维持；imagesurface = 裁缩（零消费者 +
  P032-D3 关联）。
- **D5 优化项取舍**：handle_cache LRU（容量定标——达标则观测面
  only）/ rq_update 接 mem_guard 或周期自采样 / canvas Cache 局部化
  （几何 Cache 的重建语义与 15ms tick 的收益评估）。
- **D6 TS 面**：decode tag 10 分支（占位渲染维持）+ golden 新帧。

定案记录追加 `### 5.1 定案记录`，作为 T-02..T-06 依据。

### 5.2 内存臂（T-02/T-03）

- **T-02 release 复测 + 归因矩阵**：release 产出 + p034 内存度量腿
  （矩阵数据行落 reports/）+ 达标判定（dual-exit）。
- **T-03 优化项（门后按需）**：依 D5 与 T-02 数据——LRU/自观测/
  Cache 三项按需执行 + 复测闭环。

### 5.3 位图通道臂（T-04/T-05）

- **T-04 wire 与段**：FrameMsg tag 10（BitmapReady/Ack）+ codec +
  golden（Rust 双侧）+ 段策略落地（D3）+ TS decode（D6）。
- **T-05 两端 API 与消费**：app 侧上传 API + 宿主 resolve 前缀臂 +
  handle_cache 位图键 + 合成位图生产者全链（`-q` 真渲断言）+
  canvas 快照样板（043 级，依 D4）。

### 5.4 裁定与收口（T-06/T-07/T-08）

- **T-06 裁定入册**：五 kind 处置行（用户确认 D4 后）——协议 §1.15
  + 台账 + KNOWN-DEBT（P028-D1 核销 / P028-D3 边界更新 / P-RQ-PIX
  前置③④达成注记）。
- **T-07 e2e**：`p034_rqhost_maturity_arm`——内存矩阵腿（T-02 仪器
  e2e 化）+ 位图合成腿（动画位图真渲 + 帧更新）+ 截图 assets/034/。
- **T-08 文档台账**：§1.15 增量 + 顶表 v1.15 行；台账 M7 副线债行
  更新（位图过线 ✅ / 裁定 ✅ / 量化门结论）；两仓互链。

### 规范增量

| delta_id | add/modify/retire | docs/specs/... target | before/after rule | rationale | acceptance IDs |
|---|---|---|---|---|---|
| SD-01 | add | auto-lang/docs/design/autoui/desktop-protocol-v1.md（§1.15 v1.15 增量 + 顶表行） | before：v1.14 下位图过线 not-yet（P028-D1）、像素原生五 kind 无处置、rqhost 内存门未达标（debug 314MB）；after：**位图通道全语义入册**（FrameMsg tag 10 槽纪律 + `bitmap://{id}` 词汇 + 失效/覆盖语义 + 高频流 not-yet 边界）+ **五 kind 裁定行**（video 专属/terminal M7-c/canvas 快照过线/code_editor 维持/imagesurface 裁缩）+ 内存量化门结论行——PROTOCOL_VERSION 仍 1（追加式） | 协议权威收录通道词汇与裁定（P-RQ-PIX 前置③④兑现） | AC-03/04/05 |
| SD-02 | modify | auto-os/docs/plans/autos-desktop-program.md | before：M7 副线债 :120（位图过线/像素原生裁定/内存达标三 pending）；after：三件达成注记 + 交付行（034）+ pixels 退役门剩余条件清单更新（只剩覆盖收口[M7-c] + 退役执行件） | 桌面程序台账 | AC-05 |
| SD-03 | modify | auto-lang/docs/plans/KNOWN-DEBT-AND-RISKS.md | before：P028-D1（位图过线 not-yet）/P028-D3（web 真位图）/P-RQ-PIX（四条件）；after：P028-D1 核销、P028-D3 边界更新（通道在、web 端仍 not-yet）、P-RQ-PIX 条件③④达成注记 + P034 新债随注 | 债账收口 | AC-05 |

零 spec 影响的变更不存在（通道词汇/裁定/量化门为协议级知识）；ledger
随 merge 沉淀。

## 6. 测试设计

- **单测（message）**：tag 10 round-trip + golden 新帧 + 既有 1-9
  零漂移 + 未知 tag 拒收维持。
- **单测（shm/endpoint）**：位图段/槽生命周期（上传→Ack→复用→段
  释放）；同 id 覆盖时序（单写者钉死）；超槽拒绝（沿 write_slot
  纪律）。
- **单测（broker_surface）**：`bitmap://` 前缀臂（命中缓存/未上传
  降级占位 + 观测去重）；handle_cache LRU（若 D5 采）。
- **单测（rqhost）**：daemon 自观测（若采）；位图消费端到端
  （进程内装配）。
- **e2e**：`p034_rqhost_maturity_arm`——内存矩阵腿（debug/release ×
  后端 × N 窗）+ 位图合成腿（生产者组件 `-q`：首帧位图真渲 + 动画
  更新帧翻新断言）+ canvas 快照样板腿（043 级，依 D4）+ 截图
  assets/034/。
- **TS**：decode tag 10 单测 + golden 双侧对拍。
- **回归门**：desktop_protocol（scoped + rqhost）/session/stage3 +
  `cargo t -p auto-man rust_ui` + auto-os 桌面 smoke。

## 7. 验收标准

- **AC-01 内存量化门（dual-exit）**：release 复测数据行落盘——
  daemon private ≤100MB 达标（优化项转可选留痕）**或**不达标 →
  归因矩阵 + 优化执行后复测达标（或用户显式重定标留痕）；app
  ≤10MB 沿用复核（027 超门按 P033-D3 口径另裁说明）。验证：数据
  行 + e2e 度量腿。
- **AC-02 归因矩阵资产**：{debug,release}×{wgpu,tiny-skia}×{1,2,5
  窗} 数据行落盘（四嫌疑位分摊结论句）。验证：reports/ 数据文件。
- **AC-03 位图通道全链**：合成位图生产者 `-q`（两轨任一）——
  upload→`bitmap://` 引用→宿主真渲 + 动画更新帧翻新；codec
  round-trip/golden 绿；TS decode 不 throw；`PROTOCOL_VERSION` 仍 1。
  验证：单测 + e2e 位图腿 + assets/034/。
- **AC-04 像素原生族裁定入册**：五 kind 处置行（D4 经用户确认）落
  §1.15 + 台账 + 债账；canvas 快照样板验证（若 D4 采）。验证：文档
  交叉引用 + 样板腿（如有）。
- **AC-05 文档与债账**：§1.15 + 顶表 v1.15 + P028-D1 核销 + P028-D3
  边界更新 + P-RQ-PIX ③④注记 + 台账行落盘互链。验证：文档检查。
- **AC-06 回归门**：§6 回归门全绿（在册既有红除外）。

## 8. 执行步骤

依赖序：T-01 → T-02 → {T-03[门后按需], T-04 并行} → T-05 → T-06 →
T-07 → T-08。lang worktree `D:/autostack/.wt/lang-034/auto-lang`；os
`D:/autostack/.wt/os-034/auto-os`。**无硬前置**（033 已归档；可与
M7-b/M7-c① 并行——文件面错开）。

- **T-01 [lang] 深水调查与定案**
  文件：`rqhost.rs`、`broker_surface.rs`、`message.rs`、`shm.rs`/
  `endpoint.rs`、`stage3.rs`（仪器）、iced fallback/`ICED_BACKEND`
  语义、043/030-video-player/auto-term 消费面（读）+ §5.1（写面）。
  动作：D1–D6 定案（D3 位图通道 + D4 裁定草案为核心）。
  产物：`### 5.1 定案记录`（file:line 证据）+ D4 裁定草案提请用户
  确认。
  验证：定案完备；D4 获用户确认。
  → 全 AC 前置。新路径：定案产物。
- **T-02 [lang] release 复测 + 归因矩阵**
  文件：release 产出脚本/文档行 + e2e 定位器规避 + `stage3.rs`
  （p034 度量腿）+ reports/ 数据文件。
  动作：§5.2 T-02；达标判定（dual-exit）。
  验证：矩阵数据行 + 结论句落盘。
  → AC-01/02。
- **T-03 [lang] 优化项（门后按需）**
  文件：`broker_surface.rs`（LRU/Cache）、`rqhost.rs`（自观测）。
  动作：§5.2 T-03；依 T-02 数据取舍。
  验证：复测闭环数据行。
  → AC-01。
- **T-04 [lang] 位图通道 wire 与段**
  文件：`message.rs`（tag 10 + 依 D3 的段声明面）、`shm.rs`/
  `endpoint.rs`（槽生命周期）、TS `messages.ts`/`fixtures` + Rust
  ts_fixtures。
  动作：§5.3 T-04。
  验证：round-trip/golden/段生命周期单测绿。
  → AC-03。
- **T-05 [lang] 两端 API 与消费**
  文件：`native_projector.rs`（上传 API）、`broker_surface.rs`
  （前缀臂 + 缓存键）、合成生产者测试组件。
  动作：§5.3 T-05；canvas 快照样板（依 D4）。
  验证：全链单测 + 进程内装配端到端。
  → AC-03/04。
- **T-06 [lang] 裁定入册**
  文件：协议 §1.15 草案、台账/债账条目。
  动作：§5.4 T-06（D4 用户确认后）。
  验证：文档交叉引用。
  → AC-04/05。
- **T-07 [lang+os] e2e**
  文件：lang `stage3.rs`（p034_rqhost_maturity_arm）+ assets/034/。
  动作：AC-01..04 逐条留痕。
  → AC-01/02/03/04。
- **T-08 [lang+os] 文档与台账收口**
  文件：lang `desktop-protocol-v1.md`（§1.15 + 顶表）+ KNOWN-DEBT
  处置；os 台账行 + 互链。
  动作：SD-01..03 落笔。
  → AC-05。

## 9. 复审记录

- 2026-09-20 /auto-plan:new 起草交接：`stage: new`，PLAN-034 rev 1。
  `outcome: pass`（合同完整：314MB 口径与采样基建、debug 优先序规避
  点、ICED_BACKEND 原生开关、位图通道三候选与词汇前缀最小面、五
  kind 消费现实、债账四条 pending 全部 file:line 在案）；`next:
  work`（**无硬前置**——033 已归档；D4 裁定草案在 T-01 提请用户
  确认，不阻塞 T-02 内存臂先行）。悬置决策 §10（①–④），D3 段策略
  为核心。

## 10. 待澄清事项

- **①（T-01 D3）** 位图段策略：专用第二段（推荐——与 16KiB
  Commands 档解耦）vs BufferAlloc 尾追 `bm_shm`；失效/覆盖语义
  （同 id 即时翻新）与上传 API 最小面。
- **②（T-01 D4）** 像素原生族裁定（用户确认项）：video=inproc/
  独立窗专属（推荐）、terminal=M7-c 撞面立项（推荐）、canvas=位图
  快照过线（推荐，043 样板）、code_editor=维持 not-yet（推荐）、
  imagesurface=裁缩（推荐，零消费者）。
- **③（T-01 D1）** e2e 定位器规避形态：env 覆盖（推荐，零侵入）vs
  探测序参数；release 产出的最小自动化（脚本 vs 文档行）。
- **④（T-01 D5）** 优化项取舍序：release 复测先行（推荐——可能
  零优化达标）；LRU/自观测/Cache 三项为不达标时的执行清单。
