# shell/syslog —— 宿主系统日志环（PLAN-042 SD-01）

**状态**：plan-proposed（PLAN-042 执行期落笔；review 门随计划，未入台账）

## 契约

### 1. 环（auto-lang `crates/auto-lang/src/ui/syslog.rs`）

- 进程内**有界 FIFO 环**，容量 `SYSLOG_CAP = 1000` 条，满弹头（淘汰最旧）。
- `seq` 全局单调分配（AtomicU64）；`SyslogEntry { seq, ts_ms, level, source, msg }`；
  `ts_ms` = UNIX epoch 毫秒。
- `push(level, source, msg) -> u64`（三层采集唯一写入口）；
  `dirty_seq()`（注入面脏判据：最新已分配 seq，0 = 无写入）；
  `snapshot()`（全量克隆，环序 = 最旧→最新）。
- **性能红线**：入环 O(1)、零 view 重建；视图刷新唯一通道 = 注入泵
  （§3）。

### 2. 三层采集

| 层 | 入口 | source 归因 |
|---|---|---|
| ① 宿主 `log` crate | `HostLogger`（ui_desktop boot `install_host_logger()`；error/warn/info 入环，Debug/Trace 不入） | `host`（target 前缀入 msg） |
| ② eprintln 诊断家族 | `syslog!` 宏**双写**（环 + 原 eprintln——stderr 归档层不退役） | `host`（载重站点）；`vm:<dir>/<stem>`（VM handler 失败，face 归因） |
| ③ App `log` 动词 | `log␟level␟text`（协议 v1.9；notify 同型三段） | notify_source 分段归因 registry_id；特权面 None → `privileged` |

- `syslog!` 宏：`syslog!(SyslogLevel::X, source, fmt...)`，语句宏语义（恒
  `()`）；未知 level（动词词面）兜底 `info` 不弃单。
- 刻意不入环：每帧 trace（dashboard refresh / msg-pump dispatch）——防刷
  屏（1000 条容量预算面）。
- 安装降级链：`install_host_logger()` false（已有 logger 占槽）= 仅宏面，
  不强拆既有链（`auto` CLI 的 simplelog 在 CLI 子命令进程，与桌面进程无涉
  ——桌面真入口 = ui_desktop example，本无 logger）。

### 3. 注入泵（renderer ServiceTick 段，400ms 节拍）

- 定位：`wm.wins` 查 `registry_id == "039-syslog"`（**顶层优先**：z_order
  逆序首个非隐藏匹配；全隐藏回退首个在册窗）。**窗不在 = 本段零扫描零注
  入**（环照转，泵零工作）。
- 节流：`injection_due(dirty, last_injected, last_at, now)`——seq 无变化零
  注入；距上次注入 `< 500ms` 攒批拒绝（`MIN_INJECT_INTERVAL`；注入频率
  ≤2Hz）；首拍（无簿记）立即放行基线。
- 下行形态：`snapshot()` 全量快照 → `__syslog_seq/time/level/source/msg`
  **平行字符串列表**（`write_state_vec`；time 宿主预格式化 HH:MM:SS 本地
  时区）+ `hosted="1"` → **显式 `call_handler("Rebuild")`**（宿主写状态不
  触发 handler）→ 置脏一拍重建。
- 簿记后置：注入写完成才更新 `syslog_inject_seq`/`syslog_last_inject`（写
  失败下拍重试）。
- v1 全量快照替换，不做 per-entry 增量协议（红线）。

### 4. 协议版记

- **v1.9**（PLAN-042）：上行新增 `log` 动词（词表白名单 = parse 臂；text
  单行，可含分隔符——parse 取首分符尾部保留；空 text 弃单）。下游投影载
  体面零改动（下行注入走 app state 直写）。

## 验收映射

AC-01（环语义单测）/ AC-02（宿主 trap headless+实机 boot 行）/ AC-03
（VM 错误入环，源含 face 标识）/ AC-04（动词三段+归因+兜底，单测+headless
双证）/ AC-06（≤2Hz+seq 门+窗关零开销：假时钟单测 + headless 泵测）。
