# Minesweeper AutoUI 应用规范

> **Status**: active
> **Owner**: auto-os / Plan 017
> **Revision**: 1
> **Last updated**: 2026-09-14

## 定位与边界

扫雷（038-minesweeper）是 AutoOS 核心演示与实用游戏之一。应用源码位于 `apps/038-minesweeper`，由纯 `.at` 前端与 Store 实现（零 TS / 零手写 Rust），原生跨 AutoVM 桌面渲染与 Vue Web 渲染。

游戏规则完全在前端 `MinesweeperStore` 本地运行，具有 `window: "fit"` 窗口自动收缩特性（Plan 506）。

## 运行模式与集成

- **渲染双端同构**：AutoVM Native 桌面 (`auto run -r vm`) 与 Vue 3 Web (`auto run`)。
- **端口契约**：`front_port: 4038`，场景 `scene: "ui"`，类别 `game`。
- **窗口策略**：`window: "fit"`，窗口尺寸根据难度棋盘自适应贴合。

## 核心交互与视觉规范

### 1. 色彩系统与双端一致性 (SD-01)
- **底层机制规约**：VM View Builder 在构建单一文本子节点的 `button` 时会丢弃 Text 自身的内联样式。为确保 VM 与 Vue 双端高对比色彩完全同构，所有文本前景色、字体粗细与背景色一律统一挂载在 `button` 自身的 `class:`（通过 Store 的 `cell.cell_class` 暴露）。
- **单元格状态色彩映射**：
  - 未翻开：`bg-zinc-700 hover:bg-zinc-600 border-zinc-600 text-zinc-100 rounded-none`；
  - 标记旗：`bg-zinc-700 hover:bg-zinc-600 border-zinc-600 text-rose-400 rounded-none`；
  - 翻开空白：`bg-zinc-900 border-zinc-800/80 rounded-none`；
  - 翻开数字：凹陷暗底 `bg-zinc-900 border-zinc-800/80 font-bold rounded-none`，数字高对比色系：
    - 1: `text-blue-400`
    - 2: `text-emerald-400`
    - 3: `text-rose-400`
    - 4: `text-purple-400`
    - 5: `text-amber-400`
    - 6: `text-cyan-400`
    - 7: `text-zinc-300`
    - 8: `text-pink-400`
  - 触雷引爆格：精准醒目高亮 `bg-rose-950 border-2 border-rose-500 text-rose-200`；
  - 揭示其余地雷：`bg-zinc-900 border-zinc-800 text-zinc-200`。
- **无缝紧凑网格**：消除独立圆角（`rounded-none`），使单元格整齐贴合，外层采用卡片底座与精致投影包裹，告别四角孔洞与视觉割裂。

### 2. 状态栏 LCD 仪表板与表情指示器 (SD-02)
- **LCD 仪表显示盒**：
  - 剩余雷数：`bg-zinc-950 border border-zinc-800`，标签 `MINES`（`text-zinc-500 text-[10px] font-mono`），数值 `text-rose-500 font-mono text-xl font-bold`，三位等宽展示（如 `010`）。
  - 游戏计时：`bg-zinc-950 border border-zinc-800`，标签 `TIME`（`text-zinc-500 text-[10px] font-mono`），数值 `text-amber-400 font-mono text-xl font-bold`，三位等宽展示（如 `000`）。
- **表情重开按钮**：
  - 居中放置，绑定 `.Reset` 动作；
  - 经典三态表情：游戏中 `🙂`、触雷失败 `😵`、通关胜利 `😎`。

### 3. 分段控件式难度切换器 (Segmented Control)
- 外层胶囊底座：`bg-zinc-900/90 border border-zinc-800 p-1 rounded-xl`；
- 选项按钮：`初级 9×9`、`中级 16×16`、`高级 30×16`，紧凑设计（`text-xs px-2.5 py-1`），初级 9×9 窄容器下杜绝文本折行；
- 激活态 `bg-zinc-700 text-white`，未激活态 `text-zinc-400 hover:text-zinc-200 hover:bg-zinc-800`。

### 4. 快速双击探测机制 (Chord) (SD-02)
- **交互语义**：点击已翻开的有效数字单元格（`adjacent > 0`）时，系统自动统计其九宫格内已标旗数量；
- **连带揭开**：当 `flag_count == cell.adjacent` 时，自动翻开周围所有未标旗邻居；若有暗藏地雷未标旗则立即判定踩雷失败，若全为安全格则正常揭开并自动触发零雷空白蔓延（Flood Fill）。

## 验收门

- **自动化测试**：`tests/desktop_mcp.py` 覆盖 UI 结构快照、初始状态与表情、首击揭开与数字渲染、触雷高亮与失败表情、重开复位、Fit 窗口收缩等 25 项断言全绿通过。
- **视觉对比度**：翻开数字与底色对比度达标，LCD 数码盒清晰易读，无白字白底或暗底暗字缺陷。
