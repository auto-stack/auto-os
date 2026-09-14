# Tetris AutoUI 应用规范

> **Status**: active
> **Owner**: auto-os / Plan 005
> **Revision**: 1
> **Last updated**: 2026-09-13

## 定位与边界

俄罗斯方块是 AutoOS 的单人游戏 app。应用源码位于 `apps/036-tetris`，由同一套
`.at` 前端/Store 与后端 API 生成 Vue、VM/Iced 和 Rust UI 产物。游戏规则在前端
Store 本地运行；后端只负责最高分读取与保存。每次 Tick、移动、旋转和落块都不得
访问 HTTP 或磁盘。

不包含多人、云排行榜、账户、Hold、多块 next 队列、音效、SRS 踢墙、T-spin、
combo/B2B、会话续玩或手写 Vue/JS/Rust 业务替身。

## 运行模式与集成

| 模式 | 前端 | 后端 | 部署 | 端口/入口 |
|---|---|---|---|---|
| M1 | Vue | Rust | HTTP | `17500` / `17501` |
| M2 | Vue | VM | HTTP | `17500` / `17501` |
| M3 | VM | VM | merged | 原生 AutoUI |
| M4 | VM | VM | no-merge HTTP | 原生 AutoUI + `17501` |
| M5 | VM | Rust | no-merge HTTP | 原生 AutoUI + `17501` |
| M6 | Rust | Rust | merged | 原生 AutoUI |
| M7 | Rust | Rust | no-merge HTTP | 原生 AutoUI + `17501` |

Vue 始终通过 HTTP 调用后端；VM 与 Rust 必须分别支持 `merged` 和 `no-merge`。
Vue 的进程内合并不属于本应用范围。桌面登记 id 为 `tetris`，标题为“俄罗斯方块”，
场景为 `ui`，类别为 `game`，默认深色主题、`indigo` accent、`fit` 窗口。

## 游戏规则

- 棋盘为 10×20，按自上而下行主序存储 200 个整数；0 为空，1–7 为固定方块色号。
- 七种方块各有四种顺时针旋转，共 28 个形态。v1 使用确定性的 LCG 独立抽样，
  不声称 7-bag；无效移动/旋转保持原状，无踢墙。
- active 由 `piece/rotation/px/py` 表示，ghost 由相同碰撞函数计算，不写入 locked board。
  绘制优先级为 locked、ghost、active，active 覆盖其他层。
- 重力为 `max(120, 600 - (level - 1) * 60)` ms；App 只声明一个 20ms Tick，Store
  按累计时间推进。软降每格 +1，硬降每格 +2；硬降即使距离为 0 也只锁定一次。
- 同时消除完整行，保留行次序不变并在顶部补空行。1/2/3/4 行分别得
  `100/300/500/800 × 消行前等级`；`level = 1 + floor(lines / 10)`。
- 状态为 `ready → playing ↔ paused → over`，帮助、失焦和重开确认使用独立 overlay
  状态；暂停/帮助/确认时棋盘和 active 保持，背景游戏输入被拦截。

## 输入与生命周期

`←/→` 移动，`↑` 旋转，`↓` 软降，`Space` 硬降，`P` 暂停/继续。屏上按钮与键盘
分派同一组 Store 消息。ready 和 over 不把任意键当开始；Vue 阻止方向键/Space
滚页和重复触发。左右/软降首步立即、随后按 160ms 延迟和 60ms 周期重复；旋转、
硬降、暂停每次按下只处理一次。暂停、失焦、隐藏、重开和销毁清理 Tick 与按住状态，
恢复不得追补积压时间。

## UI 规范

- 默认内容区约 640×760 逻辑像素，外边距 24、分区间距 16；棋盘严格 1:2，默认
  280×560，10 列×20 行，格间距 1px。
- 棋盘使用深色稳定底、低对比网格线；单格必须 `min-w-0 min-h-0 rounded-none p-0`，
  不得被 Button 默认圆角/内边距撑出轨道。active 为亮实心，locked 为较暗实心，
  ghost 为空心描边。
- I/O/T/S/Z/J/L 的颜色分别为青、黄、紫、绿、红、蓝、橙，三渲染固定一致，
  不随 accent 重映射。accent 只影响标题、主按钮等品牌控件。
- 右侧依次显示 4×4 居中的 next、得分/最高纪录、等级/消行/升级进度与最近反馈。
  next 按实际外接矩形居中；得分使用等宽数字；反馈固定位置短暂显示，不阻塞游戏。
- `ready`、`paused`、`help`、`confirm`、`over` 五类状态统一使用 AutoUI `dialog`
  与 `dialog-content`。Vue 为视口居中 Modal，VM/Rust 为原生 Modal Popover；无触发器
  的 dialog 仍须注册 overlay，不得退回普通流或落到窗口角落。
- 内容宽度小于 560 时，next/摘要移到棋盘上方，棋盘格缩至 20px，控制分两行；360×720
  主操作可达，320px 宽不得横向溢出，过矮窗口允许纵向滚动。点击目标至少 44×44，
  正文对比度目标为 4.5:1。

## 持久化 API

后端由 `src/back/api.at` 与 `src/back/db.at` 定义，HTTP 路径固定：

- `GET /api/tetris/record` 返回当前最高分标量；缺少记录视为 0。
- `POST /api/tetris/score` 接收 `{ "score": "<非负整数>" }`，以最大值更新并返回
  JSON `true/false`。非法或负分返回 false 且不修改记录；重复提交幂等。
- `records.json` 至少包含 `{ "schema_version": 1, "best": <int> }`，并兼容早期裸整数
  读取。保存成功前 UI 显示“新纪录待保存”，收到 true 后才显示“已保存”；失败显示
  “纪录未保存 · 重试”，游戏继续可玩。
- Init 只读一次 best；刷新、重启和跨模式读取以后端记录为唯一事实源。未收到 ACK 的
  分数不得宣称已持久化；前端 storage 不得建立第二数据源。

## 验收门

规则验收须覆盖 7×4 形态、墙/地板/堆叠碰撞、ghost、软硬降、1/2/3/4 行消除、
等级 1→2、120ms 下限、出生冲突与重开。交互验收须覆盖所有状态、按钮/键盘等价、
暂停/失焦冻结、重复键、空格不双触发、焦点和背景输入拦截。部署验收须逐项运行
M1–M7，记录进程/业务端口/数据路径和重启结果；M8 不适用。视觉验收须覆盖
640×760、360×720、320 宽，next 居中、棋盘 1:2、零圆角单格、ghost、深浅主题和
Dialog 居中。桌面入口与 `05-games` 画廊的发现、打开、开局也属于集成验收。

该规范描述产品合同；各项是否已通过，以 Plan 005 的复审记录和可复跑证据为准。
