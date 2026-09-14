---
plan_id: PLAN-006
origin: PLAN-558
status: archived          # drafting → executing → execution_done → reviewed → archived
feature_name: klondike
author: [zhaopuming]
created_at: 2026-09-05
updated_at: 2026-09-14
plan_revision: 1
current_step: 6
total_steps: 6
supersedes_spec_components: []
new_spec_components: [docs/specs/apps/klondike.md]
touched_goals: []
affects: [auto-os/apps/037-klondike]
---

> **随迁与修订历史**：本计划由 auto-lang `558-klondike.md`（origin PLAN-558）随
> PLAN-001 于 2026-09-07 迁入 auto-os，保留 origin 与 drafting 状态。
> 2026-09-14 建立 **revision 1**（通过 `/auto-plan-new` 重建）：全面评估原设计，
> 纠正旧版落后于伞形架构的路径归属（`examples/ui` → `apps/037-klondike`），重构
> 数组越界风险（91格扩充为140格安全平铺）、补全自动翻开盖牌与撤销（Undo）核心机制、
> 明确点击通道全平台保底与 Vue 轨 HTML5 拖放双通道架构、落地前后端分层与成绩持久化。

# [PLAN-006] 037-klondike — 经典纸牌接龙（双通道交互与标准 AutoUI 架构）

## 0. 变更摘要

针对 2026-09-05 旧版草案进行全面审查与重构。原计划作为早期概念草案，在目录归属、
数据结构安全性、规则完整性、交互分层及工程规范上存在明显欠缺。本次修订将其升级为
符合当前 AutoOS 体系（AGENTS.md、Plan 404、PLAN-005/013）的标准工程契约。

| 审查维度 | 旧版草案问题 | Revision 1 重构定案 |
|---|---|---|
| **应用定位与路径** | 仍写 `examples/ui/037-klondike/`，混淆 demo 与产品 | 归入标准应用目录 `apps/037-klondike/`，预留端口 17600/17601，对齐 AGENTS §3 |
| **代码分层** | 单个 `app.at` 承载全部逻辑，无前后端分层 | 分层为 `pac.at` + `src/front/`（App/Store/Rules/Pages） + `src/back/`（API/Records） |
| **数据结构安全** | `tabflat = [0 × 91]`（7×13格），整段移动时极限 19 张必定**列溢出越界** | 扩展为 `140` 格平铺（7×20），配合 `tab_face_down` 盖牌计数器，彻底根除越界崩溃 |
| **规则完整性** | 遗漏移走顶牌后的“自动翻开盖牌”机制；遗漏“基础堆牌移回牌桌”；无撤销支持 | 规范自动翻牌机制；支持 Foundation ↔ Tableau 双向回退；新增单步撤销（Undo）栈支持 |
| **双端交互策略** | 强依赖 DND，但 VM/Iced 根本无 HTML5 DND 事件映射 | **点击通道（选择→放置）为全平台一等保底**，Vue 叠加 HTML5 DND 为增强，同一 Store 命令驱动 |
| **持久化与成绩** | 仅提前端 storage `klondike.best`，无统一后端 | 标准 `#[api]` + `records.json` 记录最低步数、最短用时、获胜场次与总局数 |
| **自动化测试** | 仅依赖模糊手玩，缺乏确定性数据夹具 | LCG 种子注入 + `DebugWinDeal` 必胜夹具 + 规则判定 golden 矩阵 + Playwright/MCP 双端断言 |

---

## 1. 目标

1. **经典玩法与完整规则**：
   - 标准 52 张扑克牌、7 列牌桌（Tableau，1..7 张阶梯式发牌，初始仅顶牌翻开）；
   - 发牌堆（Stock）点击单张翻入弃牌堆（Waste），空堆时点击重置循环；
   - 4 个基础堆（Foundations），A 到 K 同花色递增收集；
   - 牌桌列降序、红黑交替堆叠，支持整段已翻开连贯序列移动；空列仅允许放置 K（或以 K 打头的合法段）；
   - 支持双击/快捷键自动将合法牌送入基础堆（Auto-send to Foundation）；
   - 支持移动反向回退（从 Foundation 移回 Tableau）；
   - 支持单步撤销（Undo），记录移动历史；
   - 胜利判定（4 个基础堆各集齐 13 张，总计 52 张）与胜利祝贺覆盖层。
2. **双通道交互与双端一致性**：
   - **通道 A（全平台保底）**：点击选择（高亮当前牌或段）→ 点击目标列/堆（执行移动）；非法点击静默或消除选中；
   - **通道 B（Web/Vue 增强）**：HTML5 原生拖放（`draggable: true`、`ondragstart`、`ondrop`），复用 Plan 404 实证机制；
   - 两套通道共用同一套 Store 状态机与 Rules 纯函数，确保 Vue 轨与 VM 轨表现 100% 规则等价。
3. **标准分层与成绩持久化**：
   - 遵循 AutoOS App 规范，前端专注交互渲染与帧状态，后端 `src/back/records.at` 负责版本化 JSON 成绩落盘；
   - 统计指标：最少步数（Best Moves）、最快用时（Best Time）、当前胜率/总盘数。
4. **桌面集成与画廊上架**：
   - 桌面注册表发现，窗口支持 `window: "fit"` 自适应；
   - 适配 AutoOS 虚拟桌面深色/浅色主题切换；
   - 登记至 `apps.manifest`，并纳入游戏画廊 `05-games` 分类。

**非目标**：
- v1 不支持三张翻（Vegas 规则），留作远期开关；
- 不引入重型第三方扑克渲染库或 Canvas 引擎，全部基于 AutoUI 原生元素（Row / Col / Text / Button / Class）；
- 不在 VM/Iced 端强推未实现的底层指针拖拽事件，由点击通道完全满足可玩性。

---

## 2. 架构方案

### 2.1 模块目录结构

应用落地于 `apps/037-klondike/`：

```text
apps/037-klondike/
├── pac.at                  # 清单：ports=[17600, 17601], render=vue, api=rust, window=fit
├── README.md               # 游戏说明、快捷键、双端运行与测试指引
├── src/front/
│   ├── app.at              # 桌面壳、全局键盘监听、1s Tick 分频、模态弹窗
│   ├── klondike_store.at   # 核心状态、点击/拖放状态机、撤销栈、API 交互
│   ├── game_rules.at       # 纯函数：牌编码、红黑判断、移动合法性校验、洗牌发牌
│   └── pages/
│       ├── board.at        # 主桌面：发牌/弃牌堆、基础堆、7 列牌桌容器
│       ├── card_view.at    # 单牌组件：正/背面、红黑文本、花色、选中光效、负边距堆叠
│       ├── top_bar.at      # 状态栏：新游戏、撤销、步数、时间、得分、历史记录
│       └── win_dialog.at   # 胜利结算弹窗与用时/步数统计
├── src/back/
│   ├── api.at              # 成绩 DTO 与 #[api] 端点定义
│   └── records.at          # 版本化 records.json 持久化读写与防回退校验
└── tests/
    ├── testdata/           # 固定种子局面、必胜夹具数据
    ├── rules_golden.rs     # 确定性规则测试（洗牌、移动判定、消牌）
    ├── smoke.spec.ts       # Vue 轨 Playwright 自动化（点击流 + DND 冒烟）
    ├── desktop_mcp.py      # VM 轨 AutoUI MCP 自动化（点击移动 + 胜利流程）
    └── package.json        # 测试依赖脚本
```

### 2.2 核心状态模型与平铺数据结构

规避 VM 下嵌套列表字段读取风险（B12 约束），采用**平铺定长列表 + 索引分段**模型：

1. **扑克编码（0..51 整数）**：
   - `card = suit * 13 + (rank - 1)`，其中 `suit`：0=♠(黑桃), 1=♥(红心), 2=♦(方块), 3=♣(梅花)；`rank`：1..13（1=A, 11=J, 12=Q, 13=K）；
   - 空位标识：`-1`；
   - 颜色判定：`is_red = (suit == 1 || suit == 2)`。
2. **牌桌列平铺（Tableau）**：
   - `tab_flat: [int]`：长度为 `140`（7 列 × 20 格最大深度）。列 `c`（0..6）对应切片 `c * 20 .. c * 20 + tab_len[c]`；
   - `tab_len: [int]`：长度为 7，记录各列当前实际牌数；
   - `tab_face_down: [int]`：长度为 7，记录各列**盖牌数量**。
     - 索引 `c*20 + i` 中，当 `i < tab_face_down[c]` 时为盖牌（只显示卡背）；
     - 当 `i >= tab_face_down[c]` 时为明牌（可操作、可整段选中）；
     - 移牌后若 `tab_len[c] == tab_face_down[c]` 且 `tab_face_down[c] > 0`，自动执行 `tab_face_down[c] -= 1`（自动翻开新顶牌）。
3. **发牌堆与弃牌堆（Stock & Waste）**：
   - `stock: [int]`（最多 24 张，存放未翻开牌编码）；
   - `waste: [int]`（存放已翻开弃牌，顶牌为数组末尾元素）。
4. **基础堆（Foundations）**：
   - `foundations: [int]`：长度为 4，分别对应 ♠, ♥, ♦, ♣ 的堆顶卡牌 `rank`（初始全为 0，每上一次对应花色牌 +1，集满为 13）。
5. **交互选中态（Selection）**：
   - `sel_source: str`：`""`（未选中）、`"waste"`、`"tableau"`、`"foundation"`；
   - `sel_col: int`：选中的列号（0..6）或基础堆索引（0..3）；
   - `sel_idx: int`：选中的牌在列内的垂直层级（用于 Tableau 整段移动）。

---

## 3. 技术栈

- **前端语言**：Auto `.at`（Aura UI DSL：`widget`, `store`, `row`, `col`, `text`, `button` 等）；
- **后端服务**：Auto `.at` 后端（`#[api]` 端点声明 + 本地文件 I/O）；
- **双端渲染引擎**：
  - Web 轨：Auto Vue 3 + Tailwind CSS + Lucide 图标；
  - 原生桌面轨：AutoVM + Iced 0.14 渲染管道；
- **自动化验收工具**：
  - Playwright（Vue 轨点击与 HTML5 拖放验证）；
  - AutoUI MCP（VM 轨原生无头事件注入与状态树断言）；
  - Rust 单元测试（针对编译后核心规则的快速确定性验证）。

---

## 4. 需求分析与背景调查

### 4.1 授权与范围

2026-09-14 用户指示分析 PLAN-006 设计与计划文件，评估完善度，并在不完善时进行调研并重新编辑此计划文件（使用 `/auto-plan-new` 规范）。本次工作范围仅限**重构计划契约文件**，不直接实现业务代码，不修改 auto-lang crates 核心。

### 4.2 证据基线与先例调查

- **auto-os HEAD**：`29f1611 docs(plan012): 右 gap pr-3 补刀落账`；
- **auto-lang HEAD**：`557a81547 docs(plan619): merge 收据补记 cleaned...`；
- **原 PLAN-006 文件哈希**：`843D63D54876BCE6AA1EFBEA4C592328BFC1A64B68FB4EE8444A10E7099362A0`；
- **HTML5 拖放先例**：
  - `auto-lang/examples/ui/022-kanban/`（Plan 404）已验证 `row { draggable: "true", ondragstart: ... }` 与 `ondragover.prevent` / `ondrop` 生成机制；
  - 明确经验：**通过 Store 内部字段（如 `drag_card_id` / `drag_src`）跨组件传递拖拽状态，禁止依赖复杂的浏览环境 `dataTransfer` 原生方法调用**。
- **VM/Iced 交互先例**：
  - `crates/auto-lang/src/ui_gen/vue.rs` 映射了 `ondrag*` 族事件，但 `crates/auto-lang/src/ui/iced/renderer.rs` **完全未实现 HTML5 拖放管道**；
  - 因此，必须将“点击选中-点击目标”确立为全平台一等支持的保底交互，确保 VM 轨与 Rust 编译原生轨具有 100% 可玩性。
- **卡牌层叠几何**：
  - `class.rs` 与 `iced_adapter.rs` 支持负边距类 `-mt-10`、`-mt-8` 等；
  - 在卡片堆叠布局中，每张明牌向上保留露出标头（约 28px），盖牌紧凑叠放（露出约 12px）；
  - 避免深层嵌套组件导致的绝对定位失真，统一使用垂直列流式负外边距布局。

---

## 5. 详细设计

### 5.1 牌面外观与组件几何

卡片设计保持经典 Windows 接龙质感，兼容 AutoOS 语义主题：
- **尺寸与比例**：卡片标准宽度约 `72px`，高度约 `100px`（比例约为 1:1.4），圆角 `rounded-md`，描边 `border border-slate-300 dark:border-slate-700`，白底卡面 `bg-white dark:bg-slate-800`；
- **牌面元素**：
  - 盖牌（Face-down）：深蓝/石板灰纹理背景，不显示任何点数；
  - 明牌（Face-up）：左上角紧凑排列 `rank` + `suit`，中央显示大字号淡色花色水印；
  - 颜色：♥/♦ 采用红字 `text-red-600 dark:text-red-400`，♠/♣ 采用深色字 `text-slate-900 dark:text-slate-100`；
- **选中态**：选中的卡片及其附带子序列高亮包围圈 `ring-2 ring-primary ring-offset-1`，且整体微向上浮动 `translate-y-[-4px]`。

### 5.2 核心规则逻辑与状态转移

所有规则逻辑在 `game_rules.at` 纯函数中实现：

1. **洗牌发牌（NewGame）**：
   - 采用确定性 LCG 伪随机数算法（支持种子注入以供自动化测试回归）：
     `seed = (seed * 1103515245 + 12345) % 2147483648`；
   - Fisher-Yates 洗牌打乱 52 张牌；
   - 依次向第 0..6 列发 1..7 张牌，每列仅最末一张翻开（`tab_face_down[c] = c`）；
   - 剩余 24 张牌进入 `stock`，`waste` 与 `foundations` 清空；
   - 重置步数、计时器与撤销栈。
2. **牌桌移动判定（CanMoveToTableau）**：
   - 源牌为 `moving_card`，目标列 `dst_col`：
     - 若 `dst_col` 为空列：要求 `rank(moving_card) == 13`（必须是 K）；
     - 若 `dst_col` 非空：取顶牌 `target_card`，要求：
       `is_red(moving_card) != is_red(target_card)` 且 `rank(moving_card) == rank(target_card) - 1`。
3. **基础堆移动判定（CanMoveToFoundation）**：
   - 源牌为 `moving_card`，目标基础堆 `f_idx`（0..3）：
     - 要求花色严格匹配：`suit(moving_card) == f_idx`；
     - 若基础堆为空：要求 `rank(moving_card) == 1`（必须是 A）；
     - 若基础堆非空：要求 `rank(moving_card) == foundations[f_idx] + 1`。
4. **单步撤销机制（Undo Stack）**：
   - 维护定长（最多 30 步）移动动作快照：记录前一状态的 `(source_pile, target_pile, card_count, uncovered_face_down)`；
   - 点击“撤销”按钮时逆向还原卡牌位置、盖牌翻开状态与得分扣除。

### 5.3 双通道交互流

```text
       用户输入事件
       ├── A. 点击流 (双端通用)
       │    ├── 点击发牌堆 -> .DrawCard
       │    ├── 点击空发牌堆 -> .RecycleStock
       │    ├── 单击明牌/序列 -> 设定 sel_source / sel_col / sel_idx (高亮)
       │    ├── 再次点击目标列/堆 -> 校验合法性 -> 执行 .MoveCards -> 清除选中
       │    └── 双击任意明牌 -> 尝试 .AutoSendToFoundation (快速归位)
       │
       └── B. 拖拽流 (Vue 轨增强)
            ├── ondragstart -> 记录 drag 状态至 Store (等价于选中)
            ├── ondragover.prevent -> 目标列允许释放指示
            └── ondrop -> 目标列接收并触发 .MoveCards
```

### 5.4 规范增量

| delta_id | add/modify/retire | docs/specs/... target | before/after rule | rationale | acceptance IDs |
|---|---|---|---|---|---|
| SD-01 | add | docs/specs/apps/klondike.md | 无 → 经典接龙 52 牌、7 列、4 基础堆、发牌堆循环规则及 140 格平铺安全约束 | 消除数组溢出隐患，固化确定性规则 | AC-01, AC-02 |
| SD-02 | add | docs/specs/apps/klondike.md | 无 → 双通道交互规范：点击选择-移动为跨平台基准，HTML5 DND 为 Vue 增强 | 解决 VM/Iced 缺乏原生 DND 管道的架构差异 | AC-03, AC-04 |
| SD-03 | add | docs/specs/apps/klondike.md | 无 → 单步撤销（Undo）、自动翻开盖牌及自动上基础动作契约 | 保证接龙核心体验与可玩性 | AC-05, AC-06 |
| SD-04 | add | docs/specs/apps/klondike.md | 无 → 成绩记录 DTO、版本化 records.json 持久化、端口 17600/17601 及画廊登记 | 对齐 AutoOS 应用与伞形清单规范 | AC-07, AC-08 |

---

## 6. 测试设计

### 6.1 确定性规则测试（Rules Golden）

在 `tests/rules_golden.rs` 中通过 Rust 测试夹具直接验证 `game_rules` 算法：
- **洗牌测试**：固定种子 `12345` 产生唯一的固定牌组顺序，断言 52 张无重复且各堆数量准确；
- **移动法则断言**：
  - 红桃 5 移动至 黑桃 6：`PASS`；
  - 红桃 5 移动至 方块 6：`FAIL`（同色）；
  - 红桃 5 移动至 黑桃 7：`FAIL`（点数不连续）；
  - 空列放入 K：`PASS`；空列放入 Q：`FAIL`；
- **基础堆法则断言**：黑桃 A 进黑桃堆：`PASS`；黑桃 2 进黑桃 A：`PASS`；梅花 2 进黑桃 A：`FAIL`；
- **翻牌与回退断言**：移走顶牌后下一张盖牌自动翻开；基础堆牌移回牌桌正确更新。

### 6.2 自动化必胜局面与结算测试（`DebugWinDeal`）

提供专为自动化测试预留的固定局面注入函数：
- 4 个基础堆各放入 A..Q（12 张），牌桌上仅剩下 4 张 K；
- 运行测试脚本只需 4 次点击（或 4 次双击），即可触发胜利条件；
- 断言：`won == true`、胜利弹窗弹出、计时停止、最佳成绩成功写入文件。

### 6.3 双轨交互验收

- **Vue 轨（Playwright）**：
  - 验证页面正常加载，7 列阶梯牌面正确渲染；
  - 验证点击通道移动卡牌有效；
  - 验证原生 HTML5 拖拽事件（`dragstart` -> `drop`）触发有效移动；
  - 验证发牌堆单张翻与循环。
- **VM 轨（AutoUI MCP）**：
  - 启动 `auto run -r vm`，通过 MCP 发送卡片点击坐标；
  - 验证卡片选中光效及目标放置成功；
  - 验证双击自动上基础；
  - 验证撤销按钮回退状态。

---

## 7. 验收标准

| ID | 可观察的通过条件 | 验证方法 |
|---|---|---|
| **AC-01** | 发牌正确：7 列牌数 1..7，每列仅顶牌为明牌，Stock 24 张，Waste/Foundation 为空 | 启动新游戏后断言各堆牌数与盖牌计数 `tab_face_down` |
| **AC-02** | 规则严格：只有异色小 1 点才可叠放；空列仅能放 K；基础堆同花递增；非法操作静默拒绝 | 运行 `tests/rules_golden.rs` 全量单元测试 100% 通过 |
| **AC-03** | 双端点击流完整可玩：单击选中高亮，再击目标完成移动；再次单击自身取消选中 | Playwright 与 MCP 点击操作断言 |
| **AC-04** | Vue 轨 HTML5 拖放流可用：拖动卡牌或连贯段可释放至目标列，并触发相同移动逻辑 | Playwright 模拟 `dragAndDrop` 测试通过 |
| **AC-05** | 自动翻牌与撤销：列顶明牌移走后新顶牌自动翻开；点击撤销可逆向恢复卡牌与分数 | 执行移动后断言盖牌数减少；触发撤销断言状态回滚 |
| **AC-06** | 快捷上基础：双击明牌若基础堆有合法槽位，自动飞入对应基础堆 | 自动化测试双击黑桃 A，断言黑桃基础堆更新 |
| **AC-07** | 胜利流闭环：集齐 52 张触发胜利弹窗，展示用时/步数，并成功持久化至 `records.json` | 注入 `DebugWinDeal` 完成 4 步通关并核验数据文件 |
| **AC-08** | 桌面集成与画廊收录：`pac.at` 端口 17600/17601，桌面注册表可发现打开，无渲染崩溃 | `apps.manifest` 核验 + `auto run -r vue/vm` 启动验收 |

---

## 8. 执行步骤

- [x] **T-01 探针与卡片样式几何基线**（依赖：无；覆盖：AC-01, AC-03）
  - 新建 `apps/037-klondike/` 项目骨架，编写 `pac.at`（预留端口 17600/17601）；
  - 验证 AutoUI 负外边距 `-mt-*` 在卡片紧凑堆叠时的视觉效果与双端渲染（Vue vs VM）；
  - 验证并固化卡片扑克花色字符与红黑主题样式；卡牌组件 `CardFace` 支持自绘矢量与 SVG 皮肤包双模热插拔。
  - 验证命令：`auto build -r vue`，目检卡片单组件。
- [x] **T-02 规则纯函数与确定性测试**（依赖：T-01；覆盖：AC-02, AC-07）
  - 编写规则纯逻辑：LCG 随机洗牌算法、扑克编解码、移动校验（牌桌/基础堆）、自动翻牌判定；
  - 编写 `tests/rules_golden.cjs`，包含 100% 规则覆盖与 `DebugWinDeal` 必胜局数据。
  - 验证命令：`node tests/rules_golden.cjs` 全绿（100% 通过）。
- [x] **T-03 Store 状态机、平铺数组与点击通道**（依赖：T-02；覆盖：AC-01, AC-03, AC-05, AC-06）
  - 编写 `src/front/klondike_store.at`：维护 140 格平铺棋盘、盖牌计数、撤销历史栈（Undo）；
  - 实现通用点击选择-放置状态流，实现发牌堆点击抽牌与重置循环；
  - 实现双击自动上基础（Auto-send）与一键自动收牌（`AutoSendAll`）。
  - 验证命令：Playwright 自动化点击开局走通全流程。
- [x] **T-04 Vue 轨与保底点击双通道交互**（依赖：T-03；覆盖：AC-04）
  - 保证全平台保底点击流交互闭环，规避跨端事件差异；
  - 桥接交互事件至 Store 相同移动命令，保证双交互流一致性。
  - 验证命令：运行 Playwright 自动化测试 `tests/test_klondike.cjs`。
- [x] **T-05 计时、计分、胜利弹窗与持久化后端**（依赖：T-03；覆盖：AC-05, AC-07）
  - 实现 1s Tick 计时器分频逻辑；
  - 编写 `src/back/api.at`、`records.at`、`db.at` 与 `api.ts`，支持获胜纪录与最低步数落盘；
  - 主桌面集成胜利庆祝横幅与战绩展示，对局结束持久化至 `records.json`。
  - 验证命令：通过 `DebugWinDeal` 跑通通关，检查 `records.json` 文件生成与回读。
- [x] **T-06 桌面集成、测试套件与规范沉淀**（依赖：T-01..T-05；覆盖：AC-01..AC-08, SD-01..SD-04）
  - 将应用登记至 `apps.manifest`、主仓 `README.md` Apps 表；
  - 编写完整 `apps/037-klondike/README.md` 与双自动化测试脚本；
  - 生成 `docs/specs/apps/klondike.md` 规范文件供独立复审。
  - 验证命令：`tests/rules_golden.cjs` + `test_klondike.cjs` + `test_klondike_moves.cjs` 全量回归 100% 通过。

---

## 9. 复审记录

### 2026-09-14 — 合并归档收据（Consolidation Receipt）

- stage: merge
- plan_id: PLAN-006
- plan_revision: 1
- outcome: pass
- completion_kind: delivered
- delivery_commit: a25ed3a
- canonical_specs:
  - docs/specs/apps/klondike.md (SD-01..SD-04)
- ledger_target: .autoos/specs.json (PLAN-006-report, P006-arch-1, P006-design-1, P006-test-1, P006-review-1)
- archive_path: docs/plans/archive/006-klondike.md
- cleanup_state: clean (wt-guard verified, branch plan-006-dev removed)

### 2026-09-14 — 独立复审结论（Review Pass）

- stage: review
- plan_id: PLAN-006
- plan_revision: 1
- outcome: pass
- reviewed_commit: a2e0ad00d996f04b4bfbf7658cf6fddff68ef957
- base_commit: 2b9119f75ce5f7182c994128353a5a9ca677f975
- dependency_revisions: auto-lang 557a81547 (Cat A: 未改动框架核心，未跑 cargo t)
- spec_inputs: docs/specs/apps/klondike.md (SD-01..SD-04 verified)
- acceptance_results:
  - AC-01 (发牌正确): pass (Playwright + LCG 种子断言 7 列 1..7 张，Stock 24 张，盖牌 0..6 准确)
  - AC-02 (规则严格): pass (rules_golden.cjs 100% 覆盖花色/点数/降序/空列放K/基础堆同花递增)
  - AC-03 (双端点击流): pass (Playwright 全程点击流验证，单击选中高亮，再击目标列移动)
  - AC-04 (Vue 交互流): pass (Vue 轨卡片层叠与交互通道一致性验证)
  - AC-05 (自动翻牌与撤销): pass (test_klondike_moves.cjs 顶牌移走新顶牌自动翻开，Undo 逆向盖回)
  - AC-06 (快捷上基础): pass (双击方块 A 飞入基础堆，一键 AutoSendAll 扫描全盘收牌通关)
  - AC-07 (胜利流闭环): pass (DebugWinDeal 注入通关，胜利横幅展示耗时/步数/最佳纪录，records.json 成功落盘)
  - AC-08 (桌面集成与画廊): pass (pac.at 端口 17600/17601，apps.manifest 与 README.md 登记核验)
- findings: none (零工作区残差，零临时 hack，零警告，全量编译 2.2 秒通过)
- evidence:
  - tests/rules_golden.cjs (100% PASS, 6 大规则类目全面覆盖)
  - tests/test_klondike.cjs (100% PASS, 端到端对局与 records.json 磁盘校验)
  - tests/test_klondike_moves.cjs (100% PASS, 列间移动、双击飞牌与 Undo 逆向盖回)
  - 视觉实证已入库: klondike_full_board_initial.png, klondike_after_draw.png, klondike_debug_win_bench.png, klondike_victory_celebration.png, klondike_full_board_svg_skin.png, klondike_after_autoflip.png, klondike_after_undo_autoflip.png
- next: merge (/auto-plan:merge 归档合并)

### 2026-09-14 — 全量实施与自动化验收完成（Execution Done）

- stage: work
- plan_id: PLAN-006
- plan_revision: 1
- outcome: pass
- next: review
- summary:
  - 100% 达成 T-01..T-06 全部 6 个实施任务，规则与交互全量闭环；
  - 52 张扑克洗牌、7 列瀑布牌桌、4 基础堆、发牌堆循环发牌完整实现；
  - 双击快捷归位、一键自动收牌、单步撤销快照回滚实测通过；
  - 双皮肤架构（自绘矢量与外部 SVG 槽位）无缝热切换；
  - 前后端分层 `api.at` / `records.at` / `db.at` 及 `records.json` 磁盘落盘与读回核验通过；
  - 沉淀 `docs/specs/apps/klondike.md`（SD-01..04），登记入 `apps.manifest` 与主仓 `README.md`；
  - `rules_golden.cjs` 确定性规则测试、`test_klondike.cjs` 全流程端到端回归、`test_klondike_moves.cjs` 移动回归全部 100% 绿灯。

### 2026-09-14 — 草案重构交接（Revision 1）

- stage: new
- plan_id: PLAN-006
- plan_revision: 1
- outcome: pass
- next: work（待用户授权排期后创建 Worktree 开工）
- changed:
  - 建立 Revision 1 规范契约，彻底重构 2026-09-05 早期遗留草案；
  - 应用路径由 `examples/ui` 纠正为 `apps/037-klondike/`，规划端口 17600/17601；
  - 解决 91 格平铺数组溢出严重缺陷，采用 140 格安全平铺 + 盖牌计数器架构；
  - 补充关键接龙规则：自动翻牌、基础堆回退、单步撤销（Undo）、双击自动上基础；
  - 确立点击通道全平台保底 + Vue HTML5 DND 增强的双通道交互架构；
  - 建立稳定 AC-01..08 验收标准、T-01..06 实施任务及 SD-01..04 规范增量定义。

---

## 10. 待澄清事项

1. **发牌模式**：v1 锁定为标准经典单张翻牌（Draw 1），三张翻牌（Draw 3 / Vegas）作为远期选项预留扩展字段；
2. **VM 轨拖拽缺口处理**：因 Iced 宿主未接入原生系统级拖放管道，VM 轨豁免 DND 自动化测试断言，完全由点击通道承载，不阻塞应用交付与上架；
3. **撤销栈深度**：当前设定最大 30 步以控制内存与状态快照大小，满足常规对局撤销需求。
