# 037-klondike — 经典纸牌接龙 (Klondike Solitaire)

基于 AutoUI 标准架构打造的完整 52 张经典 Windows 纸牌接龙游戏，支持 Vue 轨与 AutoVM 原生双端运行，包含自绘矢量排版与外部 SVG 皮肤包双模热插拔、单步撤销（Undo）、双击快捷归位、一键自动收牌以及战绩持久化。

## 特性

- **经典规则与完整闭环**：
  - 标准 52 张扑克牌，Fisher-Yates 伪随机洗牌（确定性 LCG 种子支撑测试回归）；
  - 7 列牌桌（Tableau），1..7 张阶梯式发牌，每列仅顶牌翻开；
  - 发牌堆（Stock）单张翻入弃牌堆（Waste），空堆后点击重置循环；
  - 4 个基础堆（Foundations ♠, ♥, ♦, ♣），A 到 K 同花色递增收集；
  - 牌桌红黑交替、降序堆叠，支持连贯序列整体移动；空列仅允许放置 K；
  - 自动翻牌机制：移走顶牌后下一张盖牌自动翻开（`face_down` 动态维护）；
  - 获胜判定：4 个基础堆各集满 13 张（总计 52 张）触发胜利横幅与用时/步数结算。
- **高效与人性化交互**：
  - **点击流保底**：单击选中高亮 → 点击目标列/堆执行移动，全平台（Web + 原生桌面 VM）一等保底；
  - **双击快捷归位**：双击任意明牌（弃牌堆或牌桌顶牌），若基础堆有合法槽位即刻自动飞入；
  - **一键全自动收牌（Auto）**：扫描所有可上基础堆的明牌，一键完成通关残局收割；
  - **单步撤销（Undo）**：精确保存每一步移动前的位置、分数与盖牌翻转状态，支持逆向还原。
- **双皮肤渲染模式**：
  - **自绘矢量排版 (Vector)**：纯 AutoUI 原生声明式 DSL（Row / Col / Text / Icon / Border / Shadow），无需任何外部图片资源，清晰锐利、自适应主题；
  - **SVG 皮肤包插槽 (SVG)**：支持标准外部 SVG 皮肤包（如 `svg-cards`），提供一键热插拔切换。
- **前后端分层与战绩落盘**：
  - 前端专注渲染与帧交互，后端 `src/back/api.at`、`records.at` 与 `db.at` 契约定义；
  - 持久化追踪最少步数（Best Moves）、最快用时（Best Time）、累计胜场与总盘数（落盘至 `records.json`）。

## 代码结构

```text
apps/037-klondike/
├── pac.at                          # 清单：ports=[17600, 17601], render="vue", theme="dark"
├── README.md                       # 应用使用与开发说明
├── records.json                    # 战绩数据文件（最佳步数/用时/胜场）
├── src/front/
│   ├── app.at                      # 主桌面壳、顶部状态栏、7列牌桌布局与胜利庆祝横幅
│   ├── klondike_store.at           # 核心 Store 引擎（52牌洗牌发牌、状态机、撤销栈、自动收牌）
│   └── components/
│       ├── card_face.at            # 卡牌组件（自绘矢量角标/花色与外部 SVG 插槽双模）
│       ├── card_suit.at            # 扑克花色图形组件（♠, ♥, ♦, ♣）
│       └── court_badge.at          # J / Q / K 人头牌艺术花体徽章
├── src/back/
│   ├── api.at                      # 战绩 DTO 与 #[api] 端点定义
│   ├── records.at                  # 战绩更新与防回退校验
│   ├── db.at                       # 数据库代理层
│   └── api.ts                      # Web 客户端双模持久化胶水（API + localStorage）
└── tests/
    ├── rules_golden.cjs            # 100% 规则覆盖确定性测试（花色/移动判定/洗牌/翻牌/胜利）
    ├── test_klondike.cjs           # Playwright 全流程端到端自动化测试
    └── test_klondike_moves.cjs     # Playwright 牌桌列间交互、双击上基础与撤销测试
```

## 运行方式

### 1. Web 轨 (Vue)

```bash
cd apps/037-klondike
auto build -r vue

# 运行前端开发预览（默认端口 17600）
pnpm --prefix gen/front/vue run preview --port 17600
# 或通过 auto nexus 直接拉起
auto run
```

浏览器访问：`http://localhost:17600/`

### 2. 桌面 VM 轨 (AutoVM 原生窗口)

```bash
cd apps/037-klondike
auto run -r vm
```

## 自动化测试与验收

本应用提供完整的自动化测试套件：

```bash
# 1. 规则纯函数确定性 Golden 测试 (100% 规则覆盖)
node tests/rules_golden.cjs

# 2. Playwright 全流程自动化回归 (含开局、抽牌、撤销、必胜局、自动收牌、战绩断言与皮肤切换)
$env:NODE_PATH="<playwright node_modules 路径>"; node tests/test_klondike.cjs

# 3. Playwright 牌桌列间移动、双击飞入基础堆与自动翻牌/撤销回归
$env:NODE_PATH="<playwright node_modules 路径>"; node tests/test_klondike_moves.cjs
```
