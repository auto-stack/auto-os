---
plan_id: PLAN-034
status: archived              # drafting → executing → execution_done → reviewed → archived（终态）
feature_name: rqhost-maturity
author: [agent]
created_at: 2026-09-20
updated_at: 2026-09-20
plan_revision: 1
completion_kind: delivered

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
current_step: 8
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

### 5.1 定案记录（T-01 产物，2026-09-20；lang worktree = .wt/lang-034/auto-lang@plan-034-dev，base master 4cbc810eb）

**核验法**：本人精读协议核心六文件（shm/endpoint/client_runtime/broker_surface/rqhost/message/mod）+ 三个只读探索代理（内存仪器/五 kind 消费面/债账锚点）。下列 file:line 均为 worktree 实证。

- **D1 release 复测口径（定案）**：
  - 产出：worktree 内 `cargo build --release -p auto --bin auto`（`crates/auto` default 特性已含 ui-iced——Cargo.toml:26，无需 --features；仓内先例 aavm_native_gen_check.sh:32 / examples/ui/031 perf_release.ps1:80）。不新增脚本文件，命令行留痕于本记录与 reports。
  - e2e 定位器规避：`e2e_exe::locate_with_stale_guard`（mod.rs:136-167，`#[cfg(all(test, ui-iced))]` 测试门控模块）探测序 `["debug","release"]`（:152）且无任何 release env（现存仅 AUTO_FRESH_EXE=重建 debug）。**定案：加 env `AUTO_E2E_PROFILE`**（值 release/debug，缺省不变维持 debug 优先——零行为差，四个调用方 stage3.rs:2811/4857/5328、remote.rs:606 零改动）；desktop.ps1:45-53 release 优先先例同型。
  - 采样：`sample_process_memory`（stage3.rs:202，K32 双字段 working_set/private）直接复用；落盘沿 `AUTO_034_ASSETS=1` → `docs/plans/reports/assets/034/` 门（p031 :5001-5014 / p033 :5488-5504 同型）。
- **D2 归因矩阵（定案）**：12 格 = {debug,release} × {wgpu,tiny-skia} × {1,2,5 窗}；数据行格式沿 inventory.txt 同型 `{build} {backend} windows={n}: rqhost pid=… working_set=…KB private=…KB`。观测法：build 对隔离嫌疑①（debug 构建本身）；backend 对隔离②（wgpu 设备/表面驻留）——**daemon spawn Command 加 `.env("ICED_BACKEND","tiny-skia")` 即可，零代码**（fallback.rs:276-278 env 原生；tiny-skia 认 "tiny-skia"/"tiny_skia"；rqhost 走 iced::daemon 标准链 rqhost.rs:999-1002 无手动 renderer 构造，实证可切）；窗边际斜率隔离②'（每窗 surface）；残余归③fontdb/④缓存（不够分离时加 handle_cache 清零对照腿）。判定序：release×wgpu×1 窗先行单测 → 达标即结论 + D5 优化转可选；不达标 → 全矩阵 + D5 执行。注意：tiny-skia 腿 wgpu-only primitive 降级为 warn 不渲（fallback.rs:437-459；mpv widget.rs:236 硬 wgpu）——矩阵载体选 003/001 无 mpv 面，零撞。
- **D3 位图通道（定案，核心）**：
  - **段策略 = 候选 A 专用第二段**：段名 `autodesk-shm-{pid}-{surface}-bm`，宿主侧 `SharedFrameBuffer::create`（与主段同向建段，rqhost.rs:386-387 同则）；**槽数 2、slot_size = ceil(w)×ceil(h)×16 + 4**（**执行期修正**（043 冒烟实测超档：canvas coords 560×360 in 表面 480×320 → 806400 > 614404 拒收）：表面档 ×4 字节余量 = 2× 线性——DPI 缩放与适度超面画布；commit-on-touch 使未写页不计驻留，三宿主 rqhost/session/host 同则；超档上传 = 观测弃置，v1 显式边界维持）。
  - **协商面 = BufferAlloc 消息级尾追**（先例 = shm 字段本体 message.rs:546 + Welcome frame_mode :512-517）：新尾追字段 `bm: Option<BitmapBuffer{shm,slots,slot_size}>`——**None 不写字节**（既有 golden 零漂移）；decode 侧 `remaining()>0` 条件读（信封 decode 强制 finish，message.rs:1309——尾追必须在变体 decode 内消化，Welcome 同律）。
  - **wire 变体**：计划简称"tag 10（BitmapReady/Ack）"实占两 tag——**BitmapReady = tag 10**（app→host：`{wid, id: String, w, h, stride, slot, len}`；wid 路由与 tag 4/7/8 同律——endpoint surface_for(wid) endpoint.rs:478-484，宿主自解 surface；len 显式镜像 FrameReadyShared）+ **BitmapAck = tag 11**（host→app：`{wid, slot}`——Ack 归还槽，FrameAck :651-656 同纪律）。下一可用 tag 此后 = 12。
  - **槽纪律镜像 FrameReadyShared**：app 单写者（选非前台槽 → write_slot `[u32 len][RGBA]` → BitmapReady → 宿主 read_slot → Handle::from_rgba → handle_cache 翻新 → BitmapAck 归还）；app 复用槽必在 Ack 后 = 宿主已读完旧载荷，**同 id 覆盖的无版本号时序由 Ack 纪律钉死**。
  - **id 空间**：id 为 app 侧字符串；`bitmap://{id}` 直拼 src。跨 app 撞名由 **app 侧唯一化**消化——上传 API 与组件共用助手 `bitmap_src(局部 id)` 生成 `bitmap://{pid}-{局部 id}`（进程内单源）；宿主在 ReclaimWindow/断连清该 client 的 bitmap 键（防 pid 复用串扰 + 段随 shm.remove 同步释放）。
  - **上传 API（接口最小化）**：`FrameSource::drain_bitmap_uploads(&mut self) -> Vec<BitmapUpload{id,w,h,stride,rgba}>`（缺省空——**033 drain_desktop_commands 同型缝** endpoint.rs:97-103）+ `Component::drain_bitmap_uploads`（缺省空，canvas 组件实现）；泵排水点 = 输入派发后 + 周期拍后（drain_desktop_bus 同点位 client_runtime.rs:555-572）；泵持位图段 + 槽位簿记，超槽数在途 = 丢本次 + 观测行（**高频流背压 v1 not-yet 随注**——消费面 canvas 快照级低频）。
  - **宿主消费**：endpoint `(Active, BitmapReady)` → `HostAction::BitmapReady{…}`（endpoint.rs:487 on_message 加臂）；消费面四处（rqhost apply_actions / session.rs broker_apply_actions :4069 同段 / host.rs :191 / stage3 测试臂）共用助手：读槽 → `Handle::from_rgba(w,h,rgba)` → `handle_cache.insert("bitmap://{id}", Some)` → 回 BitmapAck。
  - **resolve 前缀臂**（broker_surface.rs:104-155 分派序）：thumbnail/workspace/lucide 同级加 `bitmap://` 臂——命中 handle_cache 直出；**miss 不落负缓存**（位图可能后于首帧到达，负缓存会 pin 死后到位图——与本地族语义的差异点）+ 观测去重行 + None 占位。paint 臂（:444-462）零改动。
- **D4 像素原生族裁定（修正版草案，提请用户确认）**——T-01 核验修正两处计划草案事实：①**imagesurface"零消费者"不成立**（031-image-viewer[P-RQ-PIX 量化门载体]+027-file-manager 真实在用，`image_surface` 拼写；a2r 臂 ui_gen/rust.rs:3191 + inproc 渲染 renderer.rs:5981/image_surface.rs 齐全；queue 侧显式 not-yet coverage.rs:171/192-193 + P033-D4 element 表脱钩在册红）→ "裁缩"前提失效；②canvas 消费者 = 043+049 两例（049-canvas-graph PLAN-661 图元三表族样板，草案遗漏）；另 video `.at` 消费实为 019/020/030 三例（vue 主轨，native 样板唯一 = 030）、P032-D3 锚实为 KNOWN-DEBT:2284、terminal 撞面叙事挂 P-RQ-PIX ④（非 P032-D3——其名单只有 018 truncate/041 codeeditor）。**修正版五 kind 处置**：
  - video = inproc/独立窗专属（mpv wgpu-only widget.rs:236/403 + §4.1 SW 裁定 030 文档:98-112 + 24fps shm 流量级不经济）；
  - terminal = M7-c 撞面立项（a2r 直构臂 ui_gen/rust.rs:2669-2823 / inproc 完整 renderer.rs:4419 / queue 无臂——撞面结构；位图快照 + 命中坐标回传路径预留）；
  - canvas = 位图快照过线（CanvasPainter::draw renderer.rs:7661-7709 现成注入点，scene 结构化纯数据双表契约 043 app.at:7-10；样板验证载体 = 043，049 随注同律）；
  - code_editor = not-yet 家族维持（041 唯一消费；P032-D3 :2284 在册）；
  - **imagesurface = queue 臂 not-yet 家族维持 + M7-c 撞面裁定**（替代原"裁缩"：显示面归一到 image src 引用形态的迁移与交互回调采集面归 M7-c 撞面批裁定；P033-D4 element 表脱钩归 PLAN-656 线收口——显式边界非静默）。
- **D5 优化项取舍（定案）**：序 = release 复测先行（可能零优化达标）；不达标执行清单按嫌疑分摊序：④handle_cache LRU（**bitmap 键豁免淘汰**——app 不重传则淘汰即永久占位；容量定标依矩阵数据）→ ⑤canvas Cache 局部化（15ms tick 全量几何重建 broker_surface.rs:330-355 + :10-11 在册遗留）→ ②每窗 surface 归因处置；自观测（rq_update 周期采样——**轻量观测行形态**，不接 mem_guard 全套冻结语义：daemon 是共享宿主，冻结语义不适用）无论如何落一行（达标 = 观测面 only）。
- **D6 TS 面（定案）**：messages.ts Frame 通道现仅 decode tag 4/9（:199-224），BufferAlloc 不经 TS decode（host→app 方向不过远程端）——**义务收窄为 tag 10 decode 分支**（返回 typed 桩对象，消费面忽略；防 unknown-tag throw 破坏 WS 会话）+ 占位渲染维持（render.ts image 臂零改动）+ golden 新帧（tag 10 编码样张双侧对拍）。tag 11（BitmapAck）同批加分支（host→app 方向同理防线下）。

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

- **T-01 [lang] 深水调查与定案** [✅ 已完成 2026-09-20]
  文件：`rqhost.rs`、`broker_surface.rs`、`message.rs`、`shm.rs`/
  `endpoint.rs`、`stage3.rs`（仪器）、iced fallback/`ICED_BACKEND`
  语义、043/030-video-player/auto-term 消费面（读）+ §5.1（写面）。
  动作：D1–D6 定案（D3 位图通道 + D4 裁定草案为核心）。
  产物：`### 5.1 定案记录`（file:line 证据）+ D4 裁定草案提请用户
  确认。
  验证：定案完备；D4 获用户确认。
  → 全 AC 前置。新路径：定案产物。
  证据：定案记录落 §5.1（本人精读六核心文件 + 三探索代理全量核验）；
  两处草案事实修正（imagesurface 零消费者不成立→031/027 在用、
  canvas 消费 = 043+049）；D4 修正版经用户确认采纳（2026-09-20
  会话 AskUserQuestion 实录"采纳修正版（推荐）"）。
- **T-02 [lang] release 复测 + 归因矩阵** [✅ 已完成 2026-09-20]
  文件：release 产出脚本/文档行 + e2e 定位器规避 + `stage3.rs`
  （p034 度量腿）+ reports/ 数据文件。
  动作：§5.2 T-02；达标判定（dual-exit）。
  验证：矩阵数据行 + 结论句落盘。
  → AC-01/02。
  证据：13 格矩阵（default+12）落 assets/034/memory-matrix.txt——
  wgpu 驻留 ≈223MB 大头（234580 vs 11236KB）；每窗 wgpu ≈42MB vs
  tiny-skia ≈3.3MB；AUTO_E2E_PROFILE 定位器规避落地（mod.rs）；
  release 产出 = worktree cargo build --release（命令行留痕）。
- **T-03 [lang] 优化项（门后按需）** [✅ 已完成 2026-09-20——数据驱动裁定]
  文件：`broker_surface.rs`（LRU/Cache）、`rqhost.rs`（自观测）。
  动作：§5.2 T-03；依 T-02 数据取舍。
  验证：复测闭环数据行。
  → AC-01。
  证据：矩阵数据指认 wgpu 驻留为绝对大头（缓存/fontdb 微小——LRU/Cache
  零收益）→ 执行项 = **daemon 缺省切 tiny-skia**（run_daemon 缺省
  ICED_BACKEND，显式 env 胜出；合法性 = D4 video 独立窗专属裁定）+ 
  自观测面（rq_update 300 拍节流内存行）；复测 release×default×1窗
  **11260KB ≤ 102400KB 达标**（余量 10×）。LRU/Cache 转可选留痕
  （数据不支持执行）。
- **T-04 [lang] 位图通道 wire 与段** [✅ 已完成 2026-09-20]
  文件：`message.rs`（tag 10 + 依 D3 的段声明面）、`shm.rs`/
  `endpoint.rs`（槽生命周期）、TS `messages.ts`/`fixtures` + Rust
  ts_fixtures。
  动作：§5.3 T-04。
  验证：round-trip/golden/段生命周期单测绿。
  → AC-03。
  证据：tag 10/11 + BufferAlloc.bm 尾追（message.rs）+ 尾追向后兼容
  测试（bm=None 与旧线字节恒等）+ TS decode 桩 + BITMAP_READY_HEX
  双侧对拍（remote.rs ts_fixtures ↔ codec.test.ts）+ TS 29/29 绿 +
  Rust p034 全绿；段生命周期随 T-05 三宿主落地（commit 727007e9c）。
- **T-05 [lang] 两端 API 与消费**（核心已落 2026-09-20，样板腿随 T-07）
  文件：`native_projector.rs`（上传 API）、`broker_surface.rs`
  （前缀臂 + 缓存键）、合成生产者测试组件。
  动作：§5.3 T-05；canvas 快照样板（依 D4）。
  验证：全链单测 + 进程内装配端到端。
  → AC-03/04。
  证据：上传 API（FrameSource/Component drain_bitmap_uploads +
  produce_bitmap）+ 三宿主 BitmapReady 臂（rqhost/session/host）+
  resolve bitmap:// 前缀臂 + 进程内端到端（p034_bitmap_channel_
  inproc_roundtrip：上传→缓存→resolve→重传翻新）+ canvas 臂
  （tiny_skia 栅格化孪生 + coverage 入册 + p034_canvas_snapshot_arm
  单测）全绿（commit 727007e9c，17 文件 +1471）；043 真进程 -q 冒烟
  归 T-07 e2e 腿。
- **T-06 [lang] 裁定入册** [✅ 已完成 2026-09-20]
  文件：协议 §1.15 草案、台账/债账条目。
  动作：§5.4 T-06（D4 用户确认后）。
  验证：文档交叉引用。
  → AC-04/05。
  证据：§1.15 v1.15 全文（位图通道语义 + 五 kind 裁定表 + 门判定回填
  11260KB 达标）+ 顶表 v1.15 行；KNOWN-DEBT：P028-D1 核销 / P028-D3
  边界更新 / P-RQ-PIX ③④✅ / P034-D1..D3 新债；os 台账 PLAN-034 交付
  行 + 副线债行更新（三 pending → 两✅+门判定指针）。
- **T-07 [lang+os] e2e**
  文件：lang `stage3.rs`（p034_rqhost_maturity_arm）+ assets/034/。
  动作：AC-01..04 逐条留痕。
  → AC-01/02/03/04。
  证据：p034_rqhost_maturity_arm canvas 样板腿绿（043 -q：覆盖门放行
  + 开窗 + 首帧 + bitmap 观测行 + 零弃置）；p034_memory_matrix_leg
  矩阵腿绿（门达标行）；assets/034/ 四件（memory-matrix.txt /
  canvas-arm.txt / canvas-child-stderr.log / daemon-stderr.log）；
  截图腿沿 P031-R2 改道先例（ToDesk 覆盖层环境事实——观测行 + stderr
  即环境无关留痕）。
- **T-08 [lang+os] 文档与台账收口** [✅ 已完成 2026-09-20]
  文件：lang `desktop-protocol-v1.md`（§1.15 + 顶表）+ KNOWN-DEBT
  处置；os 台账行 + 互链。
  动作：SD-01..03 落笔。
  → AC-05。
  证据：SD-01/03 见 T-06；SD-02 os 台账（main 提交）；回归门：
  ①scoped（desktop_protocol/session/stage3）= 分支 44 红 ⊆ master
  基线 44 红（**红集差空**——detached 4cbc810eb 全量对照实证；红族 =
  VmBridge 018 并行族 + covered_elements_within_target_set 在册）；
  ②cargo t 全量日常档 = 红仅 P028-D4 在册族（p053 worktree 特有，
  债账 :2341 明文）；③TS vitest 29/29；④os smoke-034 六腿全过
  （生产 well-known daemon + 043 样板 + 自观测行——debug 双窗
  private=17584KB）；⑤auto-man rust_ui（收据见下）。

## 9. 复审记录

- 2026-09-20 /auto-plan:merge `PLAN-034:r1` 五 checkpoint（prepared →
  landed → ledger_refreshed → archived → cleaned）：
  - **prepared**：复审基线 b43d6ca53（r1 pass + 冻结哈希 120bcdfa/
    00132f91）；canonical Spec（协议 §1.15/顶表 + KNOWN-DEBT）随实现
    提交已在 lang 分支；台账投影五件（P034-1×4 + P034-r2）于 os
    worktree 备妥（c78e4fc，rebase 后 0fd56d0）。
  - **landed**：lang rebase onto master 4aadc1f57（并发 077/662 推进）
    零冲突，range-diff 四对全等（727007e9c→c4facc584 / 15619a910→
    e87bbd487 / ceb2d6d01→79950df42 / **b43d6ca53→4c3a440e6=交付提交**）
    ——ff-only 落地 master tip=4c3a440e6；组合态刷新验证 10/10 + 成熟
    腿 e2e 1/1（同 SHA）。os rebase onto main 4ec4f88（963d492→fe8131f
    等价 + 台账后裔 0fd56d0=projection-only 交付件）——ff-only 落地
    main tip=0fd56d0。
  - **ledger_refreshed**：.autoos/specs.json 读回实证——P034 条目
    五件（reports/architecture/designs/tests P034-1 + reviews P034-r2），
    总数 131→136。
  - **archived**：本文件 docs/plans/archive/034-rqhost-maturity.md +
    status: archived + `completion_kind: delivered`。
  - **cleaned**：wt-guard 三 worktree——lang-034 首跑 BLOCKED（pnpm
    node_modules junction 54 处，本会话 TS 测试 pnpm install 所置——
    661 先例同型；按闸门指引 cmd rmdir 逐链接摘除[只删链接本身]后
    过闸 exit 0）/os-034 clean/auto-down clean；移除：lang-034
    （worktree 注销后残留目录 rm -rf——闸净后无穿透风险）+ 分支
    plan-034-dev（lang，was 4c3a440e6=master；os，was 0fd56d0=main）；
    auto-down worktree 移除但**分支保留**（auto-down 本地分支
    2026-09-19 既存，非本计划所建——属主规则）；组目录 lang-034/
    os-034 双移除实证；worktree prune 零残留。os-035/lang-653/
    term-024 = 他方会话组未触碰。
  主检出注记：lang 主检出 .next-id 脏（并行会话取号簿记，未触碰）；
  os 主检出 ui-gallery/widgets-gallery 他方 WIP 与本分支零叠。

- 2026-09-20 /auto-plan:review r1：`stage: review | PLAN-034 | rev 1 | pass |
  b43d6ca53 | 4cbc810eb | auto-down@84c9897（只读）| 协议文档@tip
  120bcdfa03、KNOWN-DEBT@tip 00132f91（冻结哈希）| AC-01..06 全 pass
  | findings F-034-R1..R4（均非阻塞）| 证据见下 | next: merge`。
  **独立性声明**：实施会话内复审——结论自工件重建（diff/测试重跑/
  文档锚点），不采信执行期叙述。
  - **AC 复验**（全部于 tip b43d6ca53 重跑）：①AC-01 门达标
  assets/034/memory-matrix.txt（release×default 11260KB≤102400KB——
  T-03 缺省软光栅，dual-exit"优化执行至达标"支 + D4 video 裁定授权，
  留痕链 定案/§1.15/commit 完整；app 6460KB）②AC-02 13 格 + 分摊
  结论句（wgpu≈223MB 大头/每窗 42MB vs 3.3MB）③AC-03 p034 套件
  11/11 绿（wire/尾追兼容 golden 零漂移/TS 对拍/前缀臂/进程内端到端/
  canvas 臂/矩阵腿/成熟腿）+ TS vitest 29/29 + PROTOCOL_VERSION=1
  实核 ④AC-04 §1.15 五 kind 表 + D4 用户确认记录 + canvas 样板腿 +
  smoke ⑤AC-05 §1.15/顶表/P028-D1 核销/P028-D3 边界/P-RQ-PIX ③④/
  台账行锚点全中 ⑥AC-06 回归门：scoped 红集差空（分支 44=detached
  master 基线 44，VmBridge 018 并行族+covered_elements 在册）；cargo t
  红=P028-D4 族；cargo tf 红=mouse_area/autodown 在册族（2551/2553）；
  rust_ui 25/25；smoke-034 六腿。
  - **findings**：F-034-R1（low 债）§6"段释放/键逐出"无专断言（行为
  已实现三宿主在案；建议补测或随 M7-c 批）；F-034-R2（low）
  AUTO_E2E_PROFILE 无单测（4 行门控助手，矩阵/成熟腿间接覆盖）；
  F-034-R3（观察）位图入缓存后重绘依赖 15ms Tick 周期重绘（最坏一帧
  占位延迟，非正确性）；F-034-R4（注记）daemon 缺省后端切换为产品级
  行为变化——授权链完整（dual-exit 文本 + D4 + 数据），env 可回切。
  - **规范增量核验**：SD-01/02/03 目标在位、before/after 与实现一致；
  frontmatter new_spec_components 已定型；touched_goals=[] 说明——
  specs.json goals 节为空（库内惯例，进度由台账承载）。shm.rs 零改动
  （affects 预测 vs 实际——段机制全复用，非缺口）。
  - **脏树清点**：复审前 lang worktree 有 015-notes 生成物两件（cargo t
  已知红族再生副产物，非实现改动）——已复位；两 worktree 现净。

- 2026-09-20 /auto-plan:work 全量收口：`stage: work`，PLAN-034 rev 1，
  T-01..T-08 全闭环。`outcome: pass`（AC-01 内存门达标 11260KB≤
  102400KB[缺省软光栅档，dual-exit 走"优化执行至达标"支]；AC-02 13 格
  矩阵 + 四嫌疑分摊结论[wgpu≈223MB 大头]；AC-03 位图全链[codec/golden/
  TS/进程内端到端/043 真进程]；AC-04 五 kind 裁定入册[用户确认修正版]
  + canvas 样板；AC-05 §1.15+顶表+P028-D1 核销+台账；AC-06 回归门
  [红集差空对照 + 在册红归因]）。`code_commit`: lang plan-034-dev
  727007e9c/15619a910/夹具修复/os plan-034-dev smoke 件；`task_ids`:
  T-01..T-08；`evidence`: §5.1 定案记录 + assets/034/ 四件 + 各任务
  证据行；`blockers`: 无。`next`: review（/auto-plan:review）。
  环境注记：worktree 组 lang-034（+auto-down 只读依赖）+ os-034；
  os 主检出他方 WIP（ui-gallery demo 文件）未触碰。

- 2026-09-20 /auto-plan:work T-01：`stage: work`，PLAN-034 rev 1。
  `outcome: pass`（D1–D6 全定案于 §5.1 定案记录，file:line 证据在案；
  两处草案事实修正防错误裁定——imagesurface 消费现实、canvas 双消费）。
  `next: T-02`（release 复测 + 归因矩阵）。D4 已获用户确认（采纳修正版）。
  blockers: 无。

- 2026-09-20 /auto-plan:new 起草交接：`stage: new`，PLAN-034 rev 1。
  `outcome: pass`（合同完整：314MB 口径与采样基建、debug 优先序规避
  点、ICED_BACKEND 原生开关、位图通道三候选与词汇前缀最小面、五
  kind 消费现实、债账四条 pending 全部 file:line 在案）；`next:
  work`（**无硬前置**——033 已归档；D4 裁定草案在 T-01 提请用户
  确认，不阻塞 T-02 内存臂先行）。悬置决策 §10（①–④），D3 段策略
  为核心。

## 10. 待澄清事项

（T-01 全部定案闭环：①D3 段策略 = 专用第二段 + BufferAlloc 尾追
[§5.1 定案记录，执行期槽档修正一处]；②D4 五 kind 裁定 = 用户确认
修正版；③D1 = env 覆盖（AUTO_E2E_PROFILE）+ 命令行留痕；④D5 =
release 先行 → 数据驱动裁 daemon 缺省 tiny-skia。原悬置四项原文
如下存档。）

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
