# Spec: 037-klondike — 经典纸牌接龙 (Klondike Solitaire)

本规范定义 AutoOS 体系下 037-klondike 经典纸牌接龙应用的核心规则模型、交互契约、前后端分层与桌面集成标准。

## 1. 规则与状态模型 (SD-01)

### 1.1 扑克牌与花色编码
- **扑克总数**：标准 52 张牌，编码 `card = 0..51`；
- **花色计算**：
  - `0` = ♠ 黑桃 (Spade, 黑色)
  - `1` = ♥ 红桃 (Heart, 红色)
  - `2` = ♦ 方块 (Diamond, 红色)
  - `3` = ♣ 梅花 (Club, 黑色)
  - *计算约束*：严格规避 JS 浮点除法陷阱，采用区间判定：
    `if card >= 39 { suit = 3 } else if card >= 26 { suit = 2 } else if card >= 13 { suit = 1 } else { suit = 0 }`
- **点数计算**：`rank = (card % 13) + 1`，取值 `1 (A) .. 13 (K)`；
- **颜色判定**：`is_red = (suit == 1 || suit == 2)`。

### 1.2 牌桌布局与容量安全约束
- **发牌堆 (Stock)**：初始 24 张未翻开牌，单张翻入弃牌堆；空堆时点击重新循环；
- **弃牌堆 (Waste)**：存放已翻开弃牌，仅顶牌可操作；
- **基础堆 (Foundations)**：4 个槽位（♠, ♥, ♦, ♣），同花色从 A..K 递增收集，各集满 13 张即通关；
- **牌桌列 (Tableau)**：
  - 7 列阶梯发牌：列 `c` 包含 `c + 1` 张牌（总计 28 张）；
  - 每列初始仅末尾 1 张翻开，前 `c` 张为盖牌；
  - 堆叠规则：红黑交替（`diff_color`）、点数降序（`rank == target_rank - 1`）；
  - 空列约束：仅允许放置 K（`rank == 13`）；
  - 容量安全：采用 140 格平铺（7 列 × 最大 20 深度），规避动态列表越界。

## 2. 交互流与双端支持 (SD-02)

- **全平台保底通道（点击流）**：
  - 单击明牌：设置 `sel_source` / `sel_col` / `sel_idx`，触发卡牌高亮（`ring-2 ring-primary`）；
  - 单击目标槽位/列：校验规则有效性，合法则执行移动并解除选中；非法则静默取消选中；
  - 双端一致性：保证在无 DOM 拖放的原生桌面 AutoVM (Iced) 端 100% 完整可玩。
- **Vue 轨增强通道（拖拽流）**：
  - 支持 HTML5 原生拖拽，在卡牌上声明 `draggable="true"`，目标列容器声明 `ondragover.prevent` 与 `ondrop`；
  - 拖拽状态统一桥接至 Store 内部字段，禁止直接耦合浏览环境 `dataTransfer` 对象。

## 3. 核心动作与状态转移契约 (SD-03)

- **自动翻开盖牌 (Auto-Flip)**：
  - 当列顶牌移走导致明牌清空，且存在剩余盖牌时（`col_len == col_down && col_down > 0`），自动执行 `col_down -= 1` 翻开新顶牌。
- **单步撤销 (Undo)**：
  - 维护快照栈 `undo_stack`，记录卡牌位置、盖牌翻开计数与移动步数；
  - 执行 Undo 时，逆向恢复卡片原列、将自动翻开的新顶牌精确回翻为盖牌，并还原步数。
- **快捷上基础 (Auto-Send)**：
  - 双击任意明牌（弃牌堆或列顶牌）：检查 4 基础堆是否存在同花色下一个递增槽位，存在则自动归位；
  - 一键全自动收牌（`AutoSendAll`）：扫描全盘可归位明牌，快速完成终盘收割。

## 4. 前后端分层与持久化规范 (SD-04)

- **端口分配**：前端 `17600`，后端 `17601`（遵循 AGENTS.md §3 真实应用 17xxx 端口带）；
- **DTO 契约**：
  ```auto
  pub type GameRecord = {
      best_moves: int
      best_time_s: int
      games_won: int
      games_played: int
  }
  ```
- **API 端点**：
  - `GET /api/records`：获取历史最佳战绩；
  - `POST /api/records/win`：胜利局上报并防回退更新（仅当步数更少或用时更短时覆盖纪录）；
  - `POST /api/records/start`：对局启动盘数计数。
- **本地落盘**：持久化至 `records.json`，Web 客户端具备 `localStorage` 离线容灾。
