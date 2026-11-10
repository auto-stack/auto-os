# rq-final-acceptance — OS066 时序/呈现观测驱动（T-02 提供）

用途：Windows 完整 RQ 最终验收的时序/present 观测与重建身份核验。
全部命令在 **隔离基座** 环境运行（独占 broker 井盖 + 探针 root），
不触碰用户生产桌面/数据；性能独占窗口另行约定。

## 两种观测方案（T-02 定案，均现有时序/present 通道）

1. **frame_bench/frame_segments**（`AUTO_FRAME_BENCH=1`）：
   `[P725-FRAME]` 行随帧吐出——`begin`/`present`（泵消费时刻，R-1 序障
   合法代理）/`draw_end_ms`（呈现真相，wgpu flush 同步先行于 compositor
   present）+ 五段成本（s1..s5，µs）。进程起点单调毫秒。
   呈现终点口径：present/draw_end 是**呈现完成代理**；物理上屏（显示器
   实际刷新）本机无采集设备，具名 pending（V-04 如实分列）。
2. **外部 ms 时间戳管道 + 桌面 Host 帧合成日志**（`[plan060] frame compose
   wid=.. frame_id=.. slot=..`，session.rs 现有行）：帧合成节奏（wid 分布、
   frame_id 递增、间距 p50/p95），与 [P725-FRAME] 对读。

GPU 观测：wgpu adapter 查询（名字/后端/能力）走 host 侧现有 boot 报告；
无独立 GPU 计时器，以 draw_end（呈现真相）为 GPU 提交完成代理，独占 GPU
时间与物理上屏同列 pending。

## 命令

```powershell
# 1. 启动 iced 桌面并采集观测日志（阻塞至退出）
pwsh -File scripts/rq-final-acceptance/observe.ps1 -Out <dir> [-Seconds 30]

# 2. 解析日志 → 指标摘要（p50/p95/max 帧间距、present 延迟、wid 分布）
python scripts/rq-final-acceptance/metrics.py <log> --json <summary.json>

# 3. 负控：错误模式/缺 Host 必须具名失败（V-04/V-06 反控）
pwsh -File scripts/rq-final-acceptance/negative.ps1
```

输出目录（默认 `scripts/rq-final-acceptance/out/`，git-ignored）：
- `desktop-<ts>.log`：ui-desktop stderr（每行前缀进程单调 ms 时间戳）
- `summary-<ts>.json`：metrics.py 产物

## 隔离基座环境（继承 OS063 配方）

- `AUTOOS_BROKER_PIPE` = 独占井盖名（如 `autodesk-broker-os066-<pid>`）
- `AUTO_386_APP_ROOT` = 探针 386 root（App 装载源）
- `AUTO_DESKTOP_APPS_EXTRA` 按需注入 manifest 臂
- 结束 taskkill 自有 PID 并核 `rqhost --pipe` 进程=0（无泄漏收尾）
