---
plan_id: PLAN-017
status: drafting               # drafting → executing → execution_done → reviewed → archived
feature_name: minesweeper-ui-revamp
author: [agent]
created_at: 2026-09-14
updated_at: 2026-09-14

# /auto-plan:review 结束时填写：
supersedes_spec_components: []
new_spec_components: []
touched_goals: []             # 引用 docs/specs/goals.md 的 GOAL-NNN

affects:
  - auto-os/apps/038-minesweeper/src/front/app.at
  - auto-os/apps/038-minesweeper/src/front/minesweeper_store.at
  - auto-os/apps/038-minesweeper/tests/desktop_mcp.py

current_step: 0
total_steps: 4
---

# [PLAN-017] minesweeper-ui-revamp

> **执行环境**：主导仓 = `auto-os`，目标 app = `apps/038-minesweeper`。
> 沿用 auto-plan 范式：`/auto-plan:new` → `/auto-plan:work` → `/auto-plan:review` → `/auto-plan:merge`。
> 约束：保持纯 AutoLang/AutoUI 实现（零 TS / 零手写 Rust），确保 AutoVM 与 Vue 双端完全同构。

## 0. 变更摘要

`apps/038-minesweeper` 是 AutoUI 跨端运行（AutoVM 原生渲染与 Vue Web 渲染）的典型示例。经过实机运行与截图评测，当前版本存在严重的可读性与交互体验缺陷：
1. **数字隐形致命缺陷**：VM 模式下 `button` 内部单个 `text` 节点的内联 `style: cell.number_class` 在 VM View Builder 构建期被丢弃，按钮退化为深色模式默认的亮白文字；配合单元格翻开后的纯白背景（`bg-white`），导致**白字白底**，翻开后的周围地雷数字完全不可见，游戏丧失可玩性；
2. **状态栏暗底暗字**：剩余雷数与用时硬编码了浅色模式的 `text-gray-800`，在深色背景下呈深黑色，数字几乎隐形；
3. **难度选择器排版折行**：初级 9×9 棋盘较窄，右侧“高级 30×16”发生文本折行，三枚按钮高矮不一；且未激活按钮为刺眼的大白色块，层级反客为主；
4. **棋盘网格缺乏统一质感**：单元格带独立圆角，导致格与格之间存在黑色四角孔洞与缝隙，缺乏经典扫雷规整紧凑的地砖质感；
5. **核心 UX 体验缺失**：缺少经典扫雷必备的 Chord（双击/快速探测已标旗邻格）操作、缺少表情状态指示器（🙂/😵/😎），触雷时无法辨认哪颗是致命引爆雷。

本计划对 `038-minesweeper` 进行全面的 UI/UX 现代化深色重构，彻底解决数字与状态栏可读性，打造精致复古仪表板质感，并补齐 Chord 探测与表情交互机制。

## 1. 目标

- **G1（核心可读性与深色重构）**：彻底修复翻开单元格的数字颜色丢失问题，全面升级为深色现代质感（未翻开凸起按键 `bg-zinc-700`，翻开平坦凹陷 `bg-zinc-900`，高对比数字色 1 蓝、2 绿、3 红、4 紫、5 橙等），确保 VM 与 Vue 双端数字 100% 清晰可辨。
- **G2（复古 LCD 仪表板状态栏）**：剩余雷数与计时器改用独立数码管/仪表板风格深色内嵌盒（`bg-zinc-950 font-mono text-red-500`），文字对比度达标；重开按钮升级为经典表情指示器（🙂 游戏中、😵 触雷、😎 获胜）。
- **G3（分段控件式难度选择器）**：改用紧凑胶囊分段底座（Segmented Control），固定按钮宽度，消除 9×9 下的文本折行；消除刺眼白色块，使未激活项保持克制暗色，激活项高亮聚焦。
- **G4（精致无缝网格与触雷反馈）**：单元格取消独立大圆角，消除中间孔洞；踩雷时精准红色高亮触雷引爆点（`bg-rose-900 border-rose-600`）；优化结算呈现，消除底部文本插入导致的窗口尺寸抖动。
- **G5（Chord 快速探测 UX）**：为已翻开的数字格增加快速探测点击逻辑：当周围插旗数已等于该格数字时，点击该数字格自动翻开周围所有未标旗邻居，大幅提升中高级别游玩操作体验。

**非目标**：
- 不引入外部图片资源或手写 JS/Rust 后端（保持纯 `.at` 编写、双端同源）。
- 不改变 pac.at 的 `front_port: 4038` 与 `window: "fit"` 契约。

## 2. 架构方案

纯 AutoUI 层与 Store 层升级，完全遵循双端约束（VM 不支持视图层函数调用，所有样式与状态由 Store 计算后以字段形式暴露）：

```
[MinesweeperStore (minesweeper_store.at)]
  ├── 单元格 cell_class 升级:
  │     未翻开: bg-zinc-700 hover:bg-zinc-600 border-zinc-600 rounded-none
  │     翻开(数字): bg-zinc-900 border-zinc-800 text-{color}-400 font-bold rounded-none
  │     引爆雷: bg-rose-900 border-2 border-rose-500 rounded-none
  ├── 响应式仪表板字段:
  │     mines_str / timer_str (纯数字字符串，供 LCD 盒展示)
  │     face_icon (🙂 游戏中 / 😵 触雷 / 😎 获胜)
  └── 交互逻辑扩充:
        Chord(x, y) 快速探测翻开邻居
        
[App Widget (app.at)]
  ├── 顶部 LCD 仪表板: [ 💣 010 ]  ( 🙂 / 😵 / 😎 )  [ ⏱ 000 ]
  ├── 胶囊分段难度切换栏: [ 初级 9×9 | 中级 16×16 | 高级 30×16 ] (等宽无折行)
  └── 棋盘卡片容器: 消除孔洞贴合网格，点击未翻开格揭开，点击已翻开数字格触发 Chord
```

## 3. 技术栈

- **DSL**：AutoUI (`scene: "ui"`)，纯 `.at` 实现
- **渲染后端**：双端同构（AutoVM Native 桌面 + Vue 3 Web）
- **验证工具**：Python MCP 自动化交互套件 (`autoui_snapshot`, `autoui_action`, `autoui_screenshot`)

## 4. 需求分析与背景调查

- **授权范围**：用户明确要求“先用 /auto-plan-new 做一个改进计划，然后再实施”。
- **现有实现调研**：
  - `apps/038-minesweeper` 基于 Plan 402 与 Plan 506，支持 `window: "fit"` 窗口自动收缩特性。
  - 实机测试截图表明：
    1. 翻开数字为白色，背景为白色，文字对比度极低（不可读）；
    2. 状态栏 `text-gray-800` 在 VM 暗黑背景下近乎全黑；
    3. 初级 9×9 下难度栏由于容器宽度仅约 320px，导致“高级 30×16”折行为两行；
    4. 单元格按钮各自携带 `rounded` 属性，拼合成网格后四周凹陷形成密集孔洞。
- **底层机制调查**：
  - `crates/auto-lang/src/ui/aura_view_builder.rs:8535` 处理 `button` 时，如果 `children` 只有单一 `AuraNode::Text(_)`，则只提取其文本作为 button label，丟弃了 Text 自身的 `style`。
  - 因此将文本颜色写在 `button` 自身的 `class:`（即 `cell.cell_class` 中的 `text-blue-400 font-bold`）可同时在 AutoVM 和 Vue 双端完美生效。

## 5. 详细设计

### 5.1 色彩系统与双端适配

- **单元格状态与类名设计**：
  - 未翻开覆盖状态（`revealed == false && flagged == false`）：
    `w-8 h-8 flex items-center justify-center select-none bg-zinc-700 hover:bg-zinc-600 active:bg-zinc-800 border border-zinc-600 text-zinc-100 rounded-none text-sm`
  - 标记旗帜状态（`flagged == true`）：
    `w-8 h-8 flex items-center justify-center select-none bg-zinc-700 hover:bg-zinc-600 border border-zinc-600 text-rose-400 rounded-none text-sm`
  - 翻开空白状态（`adjacent == 0`）：
    `w-8 h-8 flex items-center justify-center select-none bg-zinc-900 border border-zinc-800/80 text-transparent rounded-none text-sm`
  - 翻开数字状态（`adjacent > 0`）：
    - 1: `w-8 h-8 flex items-center justify-center select-none bg-zinc-900 border border-zinc-800/80 text-blue-400 font-bold rounded-none text-base`
    - 2: `w-8 h-8 flex items-center justify-center select-none bg-zinc-900 border border-zinc-800/80 text-emerald-400 font-bold rounded-none text-base`
    - 3: `w-8 h-8 flex items-center justify-center select-none bg-zinc-900 border border-zinc-800/80 text-rose-400 font-bold rounded-none text-base`
    - 4: `w-8 h-8 flex items-center justify-center select-none bg-zinc-900 border border-zinc-800/80 text-purple-400 font-bold rounded-none text-base`
    - 5: `w-8 h-8 flex items-center justify-center select-none bg-zinc-900 border border-zinc-800/80 text-amber-400 font-bold rounded-none text-base`
    - 6: `w-8 h-8 flex items-center justify-center select-none bg-zinc-900 border border-zinc-800/80 text-cyan-400 font-bold rounded-none text-base`
    - 7: `w-8 h-8 flex items-center justify-center select-none bg-zinc-900 border border-zinc-800/80 text-zinc-300 font-bold rounded-none text-base`
    - 8: `w-8 h-8 flex items-center justify-center select-none bg-zinc-900 border border-zinc-800/80 text-pink-400 font-bold rounded-none text-base`
  - 致命触雷格（`exploded == true`）：
    `w-8 h-8 flex items-center justify-center select-none bg-rose-950 border-2 border-rose-500 text-rose-200 rounded-none text-base animate-pulse`
  - 其余揭示地雷格（`mine == true`）：
    `w-8 h-8 flex items-center justify-center select-none bg-zinc-900 border border-zinc-800 text-zinc-200 rounded-none text-base`

### 5.2 状态栏 LCD 仪表板与表情设计

- **左侧雷数显示盒**：
  - 容器：`row { col { text "MINES"; text .store.mines_str } }`
  - 样式：`bg-zinc-950 border border-zinc-800 px-3 py-1 rounded-lg items-center`
  - 标签：`text-[10px] text-zinc-500 font-mono tracking-wider`
  - 数值：`font-mono text-xl font-bold text-rose-500`（带 0 前缀，如 `010`、`008`）
- **中间状态表情按钮**：
  - 按钮绑定 `.Reset`，展示 `.store.face_icon`
  - 游戏中为 `🙂`，踩雷时为 `😵`，通关胜利为 `😎`
  - 样式：`text-2xl px-3 py-1 bg-zinc-800 hover:bg-zinc-700 active:scale-95 rounded-xl border border-zinc-700 shadow-md`
- **右侧计时器显示盒**：
  - 容器：`row { col { text "TIME"; text .store.timer_str } }`
  - 样式：`bg-zinc-950 border border-zinc-800 px-3 py-1 rounded-lg items-center`
  - 标签：`text-[10px] text-zinc-500 font-mono tracking-wider`
  - 数值：`font-mono text-xl font-bold text-amber-400`（带 0 前缀，如 `000`、`023`）

### 5.3 胶囊分段难度切换器（Segmented Control）

- 外层容器底座：`row { style: "bg-zinc-900/90 border border-zinc-800 p-1 rounded-xl gap-1 items-center" }`
- 选项按钮：
  - 标签精简为 `初级 9×9`、`中级 16×16`、`高级 30×16`，字号设为 `text-xs`，内边距统一 `px-2.5 py-1`，杜绝 9×9 容器下的折行；
  - 激活态类：`bg-zinc-700 text-white font-medium text-xs px-2.5 py-1 rounded-lg shadow-sm border border-zinc-600/50`
  - 未激活态类：`text-zinc-400 hover:text-zinc-200 text-xs px-2.5 py-1 rounded-lg hover:bg-zinc-800`

### 5.4 快速双击探测机制（Chord）

- 当玩家点击已翻开的数字单元格（`adjacent > 0`）时：
  1. 统计周围 8 个相邻格子中 `flagged == true` 的总数；
  2. 若 `flag_count == cell.adjacent`，则对周围 8 个格子中所有 `revealed == false && flagged == false` 的邻居调用揭开逻辑；
  3. 若邻居中有未标旗的地雷，则立即踩雷游戏结束；若均为安全格，则正常翻开并递归进行空白蔓延（Flood Fill）。

### 规范增量（Spec Delta Table）

| delta_id | add/modify/retire | docs/specs/... target | before/after rule | rationale | acceptance IDs |
|---|---|---|---|---|---|
| SD-01 | modify | `docs/specs/examples/038-minesweeper.md` | 按钮文字颜色与背景样式由子节点 `text` 的 inline `style:` 迁移至 `button` 自身的 `class:` | 解决 VM View Builder 丢弃内联 text 样式的机制限制，达成 VM 与 Vue 双端色彩一致 | AC-01, AC-02 |
| SD-02 | add | `docs/specs/examples/038-minesweeper.md` | 新增已翻开数字单元格的快速探测（Chord）交互语义及表情状态指示器 | 补齐经典扫雷进阶操作体验与情感化微反馈 | AC-04, AC-05 |

## 6. 测试设计

- **自动化 MCP 交互测试套件**：更新 `tests/desktop_mcp.py`：
  1. 验证初始状态：Snapshot 中包含 LCD 数字串、表情 `🙂`、分段按钮；
  2. 验证翻开状态：首击揭开后，数字单元格正确展示且带有对应的深色高对比文字颜色类；
  3. 验证 Chord 操作：标旗后点击数字单元格，周围邻居能够正确被连带翻开；
  4. 验证触雷流程：触雷后表情变为 `😵`，触雷格精准高亮红底；
  5. 验证三种难度切换：初级 9×9、中级 16×16、高级 30×16 下均能正常渲染且窗口无异常拉伸。
- **视觉回归截图**：重新抓取 VM 模式下的渲染截图并归档入 `src/front/tests/screenshots/`。

## 7. 验收标准

- **AC-01**：翻开单元格后的数字在 VM 模式下清晰可见，色彩鲜明（对比度 > 4.5:1），彻底告别“白字白底隐形”。
- **AC-02**：状态栏剩余雷数与计时器在深色背景下高亮易读，呈现整齐的数码仪表质感，不再出现暗底暗字。
- **AC-03**：初级 9×9 难度下难度选择按钮无折行破框，未激活按钮不再是刺眼的大白块。
- **AC-04**：单元格取消独立大圆角，棋盘无缝拼接且外层有整齐卡片阴影包裹；触雷时红底高亮标出引爆的那颗雷。
- **AC-05**：支持 Chord 快速探测操作：周围旗帜数正确时点击数字可一键翻开安全邻格。
- **AC-06**：`desktop_mcp.py` 自动化测试全绿通过，VM 模式运行零 panic 零崩溃。

## 8. 执行步骤

- **T-01**：重构 `minesweeper_store.at` 的色彩与样式模型：
  - 将高对比文本颜色与深色背景直接写入 `cell_class`；
  - 增加 `face_icon` 表情状态机与 `exploded_idx`；
  - 实现 `mines_str` 与 `timer_str` 格式化；
  - 实现 `Chord(x, y)` 快速探测逻辑。
- **T-02**：重构 `app.at` 视图布局：
  - 打造 LCD 仪表板信息栏与表情重开按钮；
  - 重构胶囊底座分段式难度选择栏（防止折行）；
  - 优化棋盘外层容器卡片与无缝网格，去除单元格孤立圆角；
  - 单元格点击事件区分：未翻开揭开、已翻开触发 Chord。
- **T-03**：更新 `tests/desktop_mcp.py`，增加深色文字对比度、Chord 操作以及表情状态测试用例。
- **T-04**：运行 VM 自动化测试与实机验证，捕获全新优化截图，更新测试截图基线。

## 9. 复审记录

- stage: new
- plan_id: PLAN-017
- plan_revision: 1
- outcome: pass
- next: work (T-01)

## 10. 待澄清事项

无阻塞事项，随时可进入执行。
