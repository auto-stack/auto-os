# Windows RQ 最终验收（PLAN-066）

本页是 AutoOS 桌面域 **Windows 完整 RQ 最终验收当前结论** 的 canonical 规范：
完整分母（能力/Demo/产品）、六格/任务/性能的逐 ID 实证状态、具名缺口与
989 消费入口。来源：PLAN-066（r1）实施与实测（2026-11-10），承接
[rqhost 装配与保真](rqhost-assembly-and-fidelity.md) 与 UI077
[产品投影覆盖](windows-rq-product-coverage.md) 的 OS 消费面。
未在本页写明的行为不视为已实现。

## 1. 完整分母（冻结）

- manifest active **28** 产品（24 gitlink pin + 3 兄弟仓 auto-musk/jade-garden/
  jade-edit + local 039-syslog；反控 jade-garden/syslog 在列）。
- examples/ui **38** 真实工程（35 编号 Demo + remote-frame-e2e/stylekit/themes；
  **041-auto-edit 具名必含**）。README 043/044 载具不属本树分母。
- UI061 能力 ID **9** 项（fidelity/pathfill、glyphrun、svg-raster、image-params、
  blur-sdf + resource/generation、release + pipeline/backpressure + ack/fence-split）。

## 2. 逐 ID 实证状态（2026-11-10 实盘）

### 2a. 六格机制面（复用 UI073/077 交付，身份未变重跑绿）

- `p077_*` + `ui::desktop_protocol::coverage::` 族：**28/28 PASS**（2.25s）。
- `p073_t05_media_two_host_semantics_and_readback`：**PASS 16.66s**（standalone
  rqhost + desktop 两 Host 媒体六格 + 独立读回）。
- 覆盖门（native_queue_set/ensure_covered）权威拒绝语义在册。

### 2b. 真实运行面（隔离基座探针）

| 面 | 结果 | 证据 |
|---|---|---|
| calc attach（queue） | adopt surface 480x320 + frame compose wid=1 | T-02 run-142646 |
| Todo attach（queue，UI077 交接） | **adopt + frame compose**，无 NotCovered——覆盖门拒绝已关闭 | T-04 td1 |
| Video attach（remote） | **adopt + frame compose**（remote 真渲）；queue 臂拒绝=契约保持（vd1） | T-04 vd1/vd2 |
| auto-edit attach | App Init + 14 actions + **frame_id 121..126 连续帧流** | T-03 ae1 |
| 客户端故障隔离 | **calc attach 确认后 kill 客户端 → 桌面存活并继续 [P725-FRAME]**（t6b 实证：adopt + frame compose → kill → ui-desktop 继续帧） | T-06 t6b |
| 无泄漏收尾 | 桌面关闭 leftover=0（多轮） | observe/n-sweep |
| 负控 | 错误端点 exit=1 具名错误 | negative.ps1 |

### 2c. 性能（N=1/4/8，debug 档，AUTO_FRAME_BENCH=1）

| N | attach | 帧 | gap p50/p95 | 成本 | 备注 |
|---|---|---|---|---|---|
| 1 | calc | 383 | 103.5/228.95ms | 3 proc/341MB | present p50=96ms |
| 4 | 4/4 | wid1 398+wid4 252 连续 | ~88-103ms | 4 proc/183MB | 2 静态低帧（无饥饿） |
| 8 | 5/8 | 779 | 88/172ms | 7 proc/1230MB | 3 未 adopt 具名（018/025/029） |
- M-02/M-05/M-07 实证（ms 帧间距/内存/公平切片）；M-01/03/04/06/08 debug 初步，
  release 档终值已由 PLAN-068 T-09 交付（见
  [Windows 完整 RQ 交付](windows-rq-complete-delivery.md) §3）。
- 物理上屏/GPU 独占计时器/高刷适用面：066 时为具名 pending（无采集设备、
  本机 59/60Hz），代理=present/draw_end（呈现真相）已证；**PLAN-068 §0 修约
  后**物理上屏以 ETW DWM 呈现节奏关闭（60Hz vsync 实证）、高刷按当前环境
  60Hz 承诺、光子终点光学采集不作必需项。

## 3. 具名缺口/待办（不缩分母）

| 项 | 状态 | owner |
|---|---|---|
| 7 个 Demo 工程构建失败（023/024/026/027/029/032/stylekit）——源缺陷 | **已关闭（PLAN-068 T-04/T-06）**：023/024/026/029/032 修复+027 环境消解+stylekit 库工程归类；T-06 矩阵 35/35 应用格绿 | UI Demo 源域（auto-ui 仓） |
| 041-auto-edit 运行期 `use 模块解析失败`（P-15） | **已关闭（PLAN-068 T-04/T-05）**：VM use-loader 识别 `{ package` 块（非文件系统形态静默）；041 实机无 AUTO-USE-DIAG | UI Demo 源域 |
| N=8 中 018/025/029 未 adopt（VM-PARKED/后端等待） | **已关闭 adopt（PLAN-068 T-05/T-08）**：N8 8/8 adopt（release）；Init 后端就绪链 park→resumed 实证（Todo 真实 API 后端 850ms 恢复）；业务数据面深测绑定 T-07/T-08 | OS068 战役后继 |
| 017-chat native HTTP 后端组树生成失败（auto-web-contract links 冲突） | **已关闭（PLAN-068 T-05）**：source_owner 保守卫（异组 AUTO_WEB_SOURCE_ROOT 具名拒绝）+ 单测；组 env 正控生成成功 | 生成器/auto-web 解析域 |
| auto-edit 编辑/IME/弹层/保存、Todo 业务/Video 播放深测 | Todo 业务闭环已交（PLAN-068 T-08：AddTodo→Toggle→SQLite 落盘→冷重开）；041 保存/重开闭环已交（T-07）；041 编辑/IME 真实字符插入=canvas MCP 边界具名（T-11 裁定机制面充分）；Video 按 PLAN-068 §10.11 用户修约=机制面+具名边界关闭，auto-video 产品后端合同具名移交 owner | 已关闭（PLAN-068 修约口径） |
| release 档性能终值、物理上屏/GPU 独占/高刷 | release P/M 数据已交（PLAN-068 T-09 N1/N4/N8）；物理上屏按 PLAN-068 §0 修约以 ETW DWM 呈现节奏关闭（60Hz vsync 实证）；高刷按当前环境 60Hz 承诺（探针历史：WPT present 需提权、NVML 不可用、无采集设备） | 已关闭（PLAN-068 修约口径） |

## 4. 989 消费入口

- 命令/形态：Todo=queue/Auto（`auto run --render-mode=desktop --desktop-endpoint
  <broker> --autodesk-incubate --app386=013-todo`）；Video=remote（同上 +
  `--autodesk-render=remote`）；观测=`AUTO_FRAME_BENCH=1` + `[P725-FRAME]` +
  observe.ps1（ms 管道）。
- 驱动：`scripts/rq-final-acceptance/{observe,metrics,n-sweep,negative}.ps1/py`。
- 证据：plan-artifacts/plan-066/auto-os/（t01-freeze、run-142646、td1/vd1/vd2、
  ae1、sweep/N1-N8、t6）+ 本 Plan §9 收据。
- 限制：物理上屏/高刷按 PLAN-068 §0 修约关闭（ETW 60Hz 实证/60Hz 承诺）；
  release 档终值已交（PLAN-068 T-09）；989 治理+RQ 联合验收消费
  本页逐 ID 状态，不冒全通过。
