# Windows 完整 RQ 交付（PLAN-068）

本页是 AutoOS 桌面域 **Windows 完整 RQ 最终交付** 的 canonical 规范：
冻结 C/工具/App/Host/数据与 P/M 预算/设备、38/28/能力逐 ID 最终收据、
重验入口与 989 消费边界。来源：PLAN-068（r1）实施与实测（2026-11-10/11），
承接 [Windows RQ 最终验收](windows-rq-final-acceptance.md)（PLAN-066 有限
实证）与 [RQHost 装配与保真](rqhost-assembly-and-fidelity.md)。未在本页写明的
行为不视为已实现。

## 1. 冻结身份与分母（T-01/T-06 收据）

- 源码组合：OS 主线 `253deb9f3d560e16aa84de35f09bfeccb4d32e57` + 本包组树
  `codex/plan-068-dev` 各仓（auto-lang `f9738f0fe`、auto-ui `6e3de861`+
  `35f1531b`+`69104765`、auto-man `347c29f`、auto-web `2d69841`、
  auto-down `58ea33f`），1003 正式 C 后 rebase 主基线（auto-lang
  `9e36d984e`+本包、auto-ui `d88e9c51`+本包）。
- 工具三件套（共享 target `.auto/desktop-runtime/target`，release 档）：
  auto.exe=`68A0CC11DDB309C4762476F3BF67085A7A5D7EB91DC14ED4449FAC0CAEC5D3A7`、
  auto-ui-tool.exe=`F039FE916E323A0AA71BD55E4BEA96DA98AA0CD290B1D2772321855AA03662FA`、
  ui-desktop.exe=`D7CA31EB095FC086DB158FCA47690B58DA14DCC3A47F9E2D6BAB04D2CCC42445`。
- 分母：28 产品（24 gitlink + 3 兄弟 + 039-syslog）、38 Demo 目录（36 含
  pac.at、35 编号工程 + stylekit 库工程）、9 能力
  （message.rs:1037）、六格（VM/Rust × direct/standalone-RQ/desktop-RQ）、
  1/4/8（N1=calc；N4=calc/clock/todo/charts；N8=+reader/monitor/photos/video）。

## 2. 逐 R 关闭收据（R-01..07）

| ID | 关闭证据 | 状态 |
|---|---|---|
| R-01 | 7 Demo 构建失败：T-04 修复 023/024/026/029/032 源+生成器（auto-ui `6e3de861`），027 环境消解，stylekit 归类库工程；T-06 矩阵 35/35 应用格绿（demo-build-t06.csv） | 关闭 |
| R-02 | 041 P-15 假阳性消除（auto-ui `35f1531b` 模块编译链）；TreeView 纯 emit 子件噪音修复（auto-ui `69104765`，041 实机点击零噪音） | 关闭 |
| R-03 | chat links 保守卫（auto-lang `f9738f0fe` source_owner.rs check_web_group_consistency + 单测）；组 env 正控生成成功 | 关闭 |
| R-04 | N8 8/8 adopt（桌面宿主 attach：018/025/029 由 VM-PARKED 转 adopt；Init 后端就绪链 park→resume 实证） | 关闭（业务数据面 T-07/08 深测） |
| R-05 | Todo 持久业务闭环（AddTodo→ToggleTodo→SQLite 落盘读回→冷重开一致）；Video mpv 深测待 T-11/R-06 门后窗口 | 部分关闭（Video 待） |
| R-06 | release P/M 终值（sweep-rel N1/N4/N8）；物理上屏/GPU 独占/高刷=代理 draw_end，物理终点子门 pending | pending（设备门） |
| R-07 | 驱动参数化（build-demos/observe/n-sweep 显式 roots/exe/out，os-066 硬编码清除）+ 自测 test_rq_complete_acceptance.py 全绿 | 关闭 |

## 3. 性能终值（T-09 release 实测，2026-11-11 独立显示环境）

| N | adopt | 帧 | frame_gap p50/p95 | present p50/p95 | 成本 |
|---|---|---|---|---|---|
| 1 | 1 | 580 | 77/239ms | 40/229ms | 3 proc |
| 4 | 4 | 1102 | 42/186ms | 40/176ms | 4 proc |
| 8 | 8/8 | 1420 | 47/146ms | 48/143ms | 10 proc ~1.36GB |

- draw_end（wgpu flush sync）= GPU 提交完成计时逐帧在案；真实 NVIDIA RTX
  4060 Ti@59Hz 3840x2560 驱动渲染。
- 预算判定：P-01..04/M-01..08 严格 pass/fail 由 T-11 独立 review 按
  budget-freeze.md 冻结协议裁定；物理上屏（显示器实际刷新终点）无采集
  设备/PresentMon/ETW present-photon 通道 → 保持 pending，不据时间流逝
  假通过；高刷适用面环境 60Hz 无法实证 >60Hz、原高刷格不因本机删。

## 4. 重验入口与 989 消费

- 驱动：`scripts/rq-final-acceptance/{build-demos,observe,n-sweep,metrics,
  negative}.ps1/py`（T-03 参数化版）+ `scripts/tests/
  test_rq_complete_acceptance.py` 自测。
- 运行命令/形态：`auto run --render-mode=desktop --desktop-endpoint=<broker>
  --autodesk-incubate --app386=<id>`；Todo=queue/Auto、Video=remote；
  观测=`AUTO_FRAME_BENCH=1` + `[P725-FRAME]` + observe.ps1。
- 证据：plan-artifacts/plan-068/auto-os/（demo-build-t06.csv、sweep-rel/、
  budget-freeze.md、ae14-fixed.err、td1..td3-*.log、observe-041/、r06-probe/）
  + 本 Plan §9 收据。
- 限制：物理上屏/高刷 pending（R-06 探针耗尽：WPT present 需提权、NVML 不可用、
  无采集设备）；Video 真实播放深测待媒体环境；041 真实字符插入受 canvas
  编辑器 MCP 边界限制（T-11 裁定证据充分性）。
- **989 C-01..05 逐渠道交接**：
  - C-01 UI 生成工程独立交付：本包交 T-06 同源重建三件套（release hash
    68A0CC11/F039FE91/D7CA31EB）+ demo 矩阵 35/35；独立移根/缺失依赖反控由
    989 按冻结来源核验（D-942-P1 语义契约缺口仍属语言交付链 owner）。
  - C-02 Web 服务成员独立导出：本包无新增——989/auto-web owner 守独立导出
    合同；017-chat 后端生成已由 T-05 source_owner 卫关闭（组 env 正控成功）。
  - C-03 Windows 完整 App/后端运行闭包：本包交真实运行证据——Todo 真实
    API 后端（Rust split HTTP 17815 + SQLite 持久化冷重开读回）、041
    desktop-RQ attach（registry 10、adopt 480x320、358 帧）、N8 8/8 adopt
    （release）；闭包/可移动运行终验随 T-11 窗口。
  - C-04 语言 SDK/Man/Web 消费：1003 正式 C 后同源重建（line-tables-only
    根 manifest 实证 023/013/015/017/018/019 等），auto-generation-core 33/33。
  - C-05 本里程碑包安装/升级/恢复：本包不新增安装器；可移动目录运行与
    保存恢复（041 AUTO_OPEN/SAVE_PATH 旁路、Todo SQLite 冷重开）为合成数据
    隔离证据；全版本升级/恢复归 989 §5.2.2 对象表。
- 989 消费：本页逐 ID 状态为 D-RQ 最终输入；C-01..05 渠道独立交付仍归
  989 及各自 owner，本页不冒全渠道通过。
