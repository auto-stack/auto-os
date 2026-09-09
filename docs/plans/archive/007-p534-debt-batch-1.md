---
plan_id: PLAN-007
origin: PLAN-577
status: archived          # drafting → executing → execution_done → reviewed → archived
feature_name: P534 债务清偿批一期（avatar 家族 + schema 滞留 + breadcrumb 栈溢出）
author: [zhaopuming, ZCode]
created_at: 2026-09-07
updated_at: 2026-09-07

# /auto-plan:review 结束时填写：
supersedes_spec_components: []
new_spec_components:
  - ".autoos/specs.json#P007-1 avatar 家族 VM 渲染语义（三臂+props desugar+默认尺寸）"
  - ".autoos/specs.json#P007-2 widget 子件渲染环守卫（自名折叠递归防护）"
  - ".autoos/specs.json#P007-3 远程图源同步抓取 3s 超时上界"
  - ".autoos/specs.json#P007-4 schema 全量再生成滞留清偿形态（真名登记+P3 显式拆册）"
touched_goals: []             # 本仓（auto-os）无 goals.md 注册表;桌面域谱系
                              # 隶 auto-lang GOAL-010 应用轨道,台账走 specs.json sections

affects: [auto-lang/ui]       # 受影响的 specs 路径
current_step: 9
total_steps: 9
---

> **随迁注记（Stage B P-1，2026-09-07，PLAN-001）**：本计划自 auto-lang
> `docs/plans/577-p534-debt-batch-1.md`（origin PLAN-577）随迁重编为 PLAN-007——drafting 原状、
> 状态机续用，git 历史留 auto-lang 仓。设计依据
> [Design 01 §5](../design/01-stage-b-desktop-migration.md)；auto-lang 侧指针行见
> 其 `docs/plans/INDEX.md`。正文中的 auto-lang 相对路径/行号锚点，开工时按本仓
> AGENTS §2 解析序换算（env → `../auto-lang` → `D:/autostack/auto-lang`），
> 现文不回改。
> 编号消歧：origin 为 p534 债批一（577-p534-debt-batch-1），与 auto-lang
> archive 既有 577-emitter-gaps-batch（trans 域，已归档）无关。

# [PLAN-007] P534 债务清偿批一期——avatar 家族补齐 + schema 再生成滞留清偿 + breadcrumb 栈溢出归因修复

## 变更摘要

PLAN-534 执行/复审期登记的 P534-D1..D6 中,三个可独立交付、互不阻塞的项组成
本批（债务批先例:505 一期/515 二期）:

1. **P534-D4 avatar 家族补齐**:`convert_avatar` 现仅渲染裸 avatar 标签灰圆
   占位,`avatar-image`/`avatar-fallback` 零臂（unknown fallback）,子件整体
   弃置——gallery /hovercard 页触发器 hit area 零高（534 实机 hover 只能借
   语料工程完成）、/avatar 页缺图。
2. **schema 再生成滞留清偿**:534 做最小补丁时暴露的全量再生成滞留——
   sidebar 族 7 元素+NavDestination/Swiper 未入 element_coverage 登记、
   P3 `sidebar_menu_sub_button` 档位不一致（静态 full vs schema partial）。
   谁下次跑 `SCHEMA_DRIFT_GENERATE_AT=1` 必撞围栏红。
3. **P534-D5 breadcrumb 栈溢出归因修复**:534 全站扫描三次确定性死于
   Tooltip→Breadcrumb 边界。复审期探针已定性=**页本体即崩**（新会话直达
   Breadcrumb 必现,与访问序列无关）,即 **P530-D1 已在案存量**（"导航到
   breadcrumb 页必现 overflow,疑 ForLoop/嵌套 link 链,debug 构建抓栈
   bisect"）——本批偿还之。

不在本批（维持债务）:P534-D1（VM 定时器原语/delay）、D2（拖拽手势）、
D3（vue 基线运行对照,依赖 npm 环境建设）、MCP overlay 注入通道
（P533-D2 扩展）。

## 目标

1. avatar 家族:`avatar`（有子件）渲染子件组合,`avatar-image`（src→图）、
   `avatar-fallback`（→文本）独立臂;/avatar 页观感补齐。
2. hovercard 触发器在 gallery 页**直接**可 hover（hit area 非零）,534 验收
   G3 的 gallery 形态补全,不再借语料工程。
3. schema 全量再生成入账:滞留 12 元素登记,P3 档位对齐,三围栏绿,无新增
   豁免争议。
4. breadcrumb 页可直接导航进入（不崩）,全站 68 页扫描一遍全绿。
5. KNOWN-DEBT:P534-D4/D5 结案回写,D5 关联 P530-D1 一并结算。

## 架构方案

三件独立可交付,共享 534 已验证的流程资产:

| 件 | 落点 | 复用 |
|---|---|---|
| avatar 家族 | aura_view_builder 双镜像臂+四表同步 | convert_image_or_icon（img/image/icon 先例）、534 四表流程（schema.rs+aura.at+render_support+baseline） |
| schema 滞留 | element_coverage.rs+SCHEMA_DRIFT_GENERATE_AT 全量再生成 | 534 围栏清偿流程（P533 dialog 同型） |
| breadcrumb 溢出 | 病源定位（页面内容二分+debug 构建）→修复 | P530-D1 归因留档（scratch/p530/） |

## 技术栈

Rust（auto-lang crate）+ AutoLang .at（gallery 页面）+ 既有测试基建
（headless iced_test/plan 探针/autoui-verifier MCP）。

## 需求分析与背景调查

（2026-09-07 对码+实机探针核实）

- **avatar 现状**:`convert_avatar`（aura_view_builder.rs,untracked :1842/
  tracked :3405 两臂）只产灰圆占位容器,**子件参数被丢弃**;
  `avatar-image`/`avatar-fallback` 全仓零臂。`img|image|icon` →
  `convert_image_or_icon`（src prop 图源）可直接复用;537 已实证 image
  fit cover/contain 双端。
- **schema 滞留清单**（534 重生成实测暴露）:NavDestination、Swiper、
  sidebar-group-action、sidebar-inset、sidebar-menu-action、
  sidebar-menu-badge、sidebar-menu-sub、sidebar-menu-sub-item、
  sidebar-separator、sidebarinput、sidebarmenuskeleton、sidebarrail
  （12 元素未入 element_coverage）;P3 `sidebar_menu_sub_button`
  render_support 静态=full vs schema=partial。
- **breadcrumb 溢出三事实**（534 复审期探针,scratch/p534_review/）:
  ①三次全站扫描确定性死于 Tooltip→Breadcrumb 边界;②死亡前快照指标平坦
  （无累积,同页 96 次循环零崩溃零增长）;③**新会话直达 Breadcrumb 即崩**
  （与序列无关）。P530-D1 在案先验:master@96586cca 对照构建同复现,疑
  ForLoop/嵌套 link 链。breadcrumb 家族五标签全仓零臂（全 unknown
  fallback）,页面 63 行有限树——病源疑在页内特定构造形态（breadcrumb-link
  带 href prop？unknown tag+特定 prop 组合触发的通用路径递归）,执行期
  二分定位。
- **边界**:534-D3/D1/D2/MCP 注入不在本批;avatar 圆形裁剪受 VM 无 clip
  原语限制（见待澄清③）。

## 详细设计

### D1 avatar 家族臂（双镜像）

- `avatar`（有子件）:容器（rounded-full bg-muted+尺寸类）+子件转换
  （avatar-image/avatar-fallback）；无子件保持现灰圆占位。
- `avatar-image`:复用 `convert_image_or_icon`（src prop 同 img 形态;
  网络图源加载失败/未载时的占位沿 image 既有语义）。
- `avatar-fallback`:text 臂（children text/text prop → convert_text_element,
  居中类注入）。
- 双镜像（untracked+tracked）接线两处分发区,镜像 534 sheet/drawer 臂模式。
- **四表同步**（复用 534 流程）:新拼写入 schema.rs elements+aura.at
  （avatar-image/avatar-fallback backends.iced 回填 native）+render_support
  +schema_drift_baseline。

### D2 schema 滞留清偿

- element_coverage.rs 登记 12 元素（NotConsumed"shadcn web 长尾"注释,
  对齐 drawer_close 行风格）。
- `SCHEMA_DRIFT_GENERATE_AT=1` 全量再生成→aura.at 差异复核（对照 534 期
  /tmp 重生成产物已知形态）→P3 `sidebar_menu_sub_button` 档位对齐
  （先诊断 render_support 静态表与 schema backends.iced 哪边为意图值,
  对齐另一边）→baseline 裁剪（漂移消除项）人工复核。
- 三围栏验收:schema_drift+queue_coverage+docs_gen。

### D3 breadcrumb 溢出（P530-D1 偿还）

- **T 定位**（二分法,每步留痕 scratch/p577/）:
  a. debug 构建直接导航 breadcrumb 页固定复现;
  b. 页面内容二分:preview-card 外壳/table 块/各 breadcrumb-* 子块逐块
     注释（页面 63 行,块级 ≤6 步）——锁定病源构造;
  c. 若 a/b 指向通用路径（疑:unknown tag+href prop→link/路由形态递归）,
     以最小 .at 复现（≤10 行）固定病源。
- **T 修复**（按病源二选一,预设出口）:
  - 页面构造病源（如特定 prop 组合触发通用路径环）→builder/解析路径修复
    （环检测或深度防御）+最小复现回归测试;
  - 深层运行时缺陷（修复面超出本批）→归因报告（scratch/p577/+最小复现
    入语料）+独立计划立项,KNOWN-DEBT P534-D5 标"归因在案,偿还另立"。
- 全站扫描回归:68/68 页一遍全绿（breadcrumb 可进,无崩溃尾部）。

## 测试设计

（作用域:Category B 局部 ui 改动;`cargo t iced` 局部+`cargo tv`;fold 前
pre-fold `cargo tf`;不触 aavm 路径零 taa 触发）

- avatar:builder 探针（children 渲染/无子件占位保持/fallback 文本可见/
  hit area 非零）+layout_tests 或 plan 探针文件;实机 /avatar+/hovercard
  截图（autoui-verifier MCP）。
- schema:三围栏（schema_drift_fence/queue_coverage_drift_fence/
  docs_coverage_fence）。
- breadcrumb:最小复现回归测试（病源构造 .at→build 不崩）;全站扫描脚本
  复用 scratch/p534_gallery_scan.py（68/68 全绿判据）。
- 双端:avatar vue 轨对照（shadcn Avatar 语义——图源/回退文本），537 image
  先例口径。

## 验收标准

- [x] /avatar 页 avatar-image/avatar-fallback 渲染（截图落账 scratch/p577/）。
      （t4_avatar_vm.png+像素密度 0.79;props 形态 desugar 补臂后两 40x40 圆）
- [x] /hovercard 页 avatar 触发器 hit area 非零,真 hover 进/出**直接通过**
      （不借语料工程;534 G3 gallery 形态补全）。
      （@rect(781,496,40,40) h=40;OS 级真 hover 进→true/出→false,
      t4hc_summary.md PASS+三截图）
- [x] schema 三围栏绿;滞留 12 元素登记;全量再生成入账无未审豁免。
      （T2/T5/T6 期完成:2/2+4/4+7/7）
- [x] breadcrumb 页可直接导航进入不崩;全站扫描 68/68 全绿。
      （环守卫 889d81fb1;fullscan_report.md visited=68 anomalies=0 died=False）
- [x] KNOWN-DEBT P534-D4/D5+P530-D1 结案回写（D5 若走"归因报告+另立"出口,
      本项改判该出口产物在案）。
      （三条全 ✅ 偿还;D5 根因同 P530-D1 非另立——扫描序列首踩病源页实证）
- [x] `cargo t iced`+`cargo tv` 全绿（唯一红允许=charts gallery 存量）;
      pre-fold `cargo tf` 与基线一致。
      （iced 唯一红=lucide 存量/master 集合全等;tv 唯一红=charts 存量;
      tf no-fail-fast 唯一红=charts,master 同命令集合全等）

## 执行步骤

（原子任务;每步完成后追加 [✅ 已完成] 一行证据。行号为 2026-09-07
master 基点,执行时以 grep 重新定位。worktree=`D:/autostack/.wt/lang-577/
auto-lang`;依赖仓不涉及。）

1. [✅ 已完成] **avatar-image/fallback 臂**——`d9bb87ef9`：双镜像三臂
   （avatar 容器转换子件/avatar-image→convert_image_or_icon/avatar-fallback
   →convert_text_element 带 events/children）；check 0 错。
2. [✅ 已完成] **四表同步**——schema.rs elements（avatarimage/avatarfallback）
   +render_support 三臂 full+aura.at 全量再生成+baseline 复核 8+/5-（全
   avatar 拼写/消漂维度：canvas 行消解+sub_button 入册消漂为改善项）；
   **三围栏 2/2+4/4+7/7 绿**。
3. [✅ 已完成] **avatar 探针**——`tests/plan577_avatar_tests.rs` 2/2：
   有子件（avatar-image 图节点 src 入树+avatar-fallback 文本 "CN" 可见）
   /无子件容器占位保持。
4. [✅ 已完成] **avatar 实机**——`023b6a987`（T4 顺带两根因修复：①props
   形态 desugar——gallery /avatar 页用 `avatar (src/fallback)` props 形态,
   对齐 vue 端 AvatarImage/AvatarFallback 展开补臂；②**触发器零高根因**——
   centering 容器无显式宽高被 apply_container_style 设 Fill×Fill,shrink
   上下文（Popover 锚）解析为零高,对齐 vue 端恒注入 w-10 h-10 修复；
   ③远程图源裸 `reqwest::blocking::get` 挂死渲染线程→加 3s 超时）。
   实机证据（scratch/p577/）：/avatar 两 40x40 圆渲染（网络图 avatar
   content 密度 0.79、CN 灰圆 0.79/gray 0.70,截图 t4_avatar_vm.png）；
   /hovercard 触发器 @rect(781,496,40,40) h=40>0（t4hc_hovercard_bounds.txt,
   152 rects）→ OS 级真 hover 进 `__dlg_open_1=true`/出 `=false`
   （t4hc_summary.md PASS,三截图在案）——不借语料工程,534 G3 补全。
   探针 3/3（增 props 形态用例）。
5. [✅ 已完成] **element_coverage 登记**——实测修正：534 调查 12 项中
   sidebar 系 10 项 561 期已按折叠名在册；全量再生成把命名翻转为连字符/
   Pascal/整词真名（sidebar-group-action…/NavDestination→nav-destination/
   Swiper→swiper/sidebarinput…）——真名登记+旧下划线孪生与 Pascal 行退役，
   双向净+计数一致（414=414）。
6. [✅ 已完成] **schema 全量再生成**——aura.at 419 元素（413→419）人工
   复核（再生成滞留全清：drawer/hover_card/nav 组/sheet/sidebar 进
   builtin 层）；**P3 消解**=sidebar_menu_sub_button 独立立册（vue 同件但
   iced 档不同——561 真值 sub=full/button=partial，regen 别名合并丢该
   区分，显式产物元素拆开）；kitchen-sink 38→50 节+core.md 随生
   （auto-os worktree `84ff922`）；DOC_EXCLUDE+2（折叠键）；
   schema_drift 2/2+docs_gen 4/4+queue 1/1 绿。
7. [✅ 已完成] **breadcrumb 病源二分**——3 步锁定（≤6 预算,留痕
   scratch/p577/bisect_s*.log）：s1 注释 `breadcrumb-page "Breadcrumb"`
   一行即活（页面其余全在）→ s2 页面剥至最小形态（10 行体,仅含
   breadcrumb-page tag）仍 100% 栈溢出 → s3 对照 tag 改 `breadcrumb-pag`
   （折叠键不再命中组件名）活。**机制**：未知 tag 经 widget_registry
   折叠兜底（P435 P8-6：剥 `-`/`_`+小写）命中页面组件自身名——
   `breadcrumb-page`→`breadcrumbpage`==fold(`BreadcrumbPage`)→
   render_child_widget 无限自递归。疑点修正：与 unknown tag+href prop
   无关（breadcrumb-link 无恙）,是自名折叠命中。
8. [✅ 已完成] **breadcrumb 修复**——`889d81fb1`：通用环守卫
   （AuraViewBuilder 增 active_child_widgets 进行中集合,按分支克隆
   传递,双胎 render_child_widget 入口命中环渲染 Empty;A→B→A 类互递
   归同防）。回归测试 plan577_breadcrumb_cycle（红灯验证:摘守卫跑测
   STATUS_STACK_OVERFLOW 实测在案）。验证：最小复现不崩+直达
   breadcrumb 页 ALIVE（MCP snapshot len=33598 正常返回,截图
   t8_breadcrumb_alive.png+快照 t8_breadcrumb_snapshot.txt 落账）。
9. [✅ 已完成] **收口簿记**——KNOWN-DEBT 三条结案回写（auto-lang worktree
   提交：P530-D1 ✅889d81fb1 环守卫/P534-D4 ✅023b6a987 家族+根因/P534-D5
   ✅根因同 P530-D1,扫描 ~55 次首踩 breadcrumb 页）；gallery README avatar
   行更新（auto-os worktree `d6a48d2`）；全站扫描 **68/68 全绿零异常应用
   存活**（scratch/p577/fullscan_report.md，适配版脚本 p577_fullscan.py）；
   pre-fold `cargo tf` no-fail-fast 唯一红=charts 存量，master 同命令对账
   **集合全等零新增红**；`cargo t iced` 唯一红=lucide 存量（master 同命令
   集合全等）；`cargo tv` 唯一红=charts 存量（计划允许口径内）。

## 复审记录

### work 阶段收尾记录（2026-09-09）

stage: work | PLAN-007 | rev 1（随迁重编后首版执行完毕） | pass |
code_commit: auto-lang os-007-dev `023b6a987`（T1-T4）+`889d81fb1`（T7/T8）
+KNOWN-DEBT 提交;auto-os os-007-dev `84ff922`（T6 kitchen-sink）+`d6a48d2`
（T9 README/gitignore） | task_ids: T1-T9 全 ✅（current_step 9/9） |
evidence: scratch/p577/（T4 实机三件套+T7 二分四步+T8 alive 快照/截图+
fullscan_report.md 68/68+tf/tv/iced 对账） | blockers: 无 |
next: /auto-plan:review 007

执行期偏差记录（对原设计的修正，均在授权语义内）：
1. T4 发现 props 形态缺口（/avatar 页 `avatar (src/fallback)` 用法不在
   T1 臂覆盖内）→ 对齐 vue 端 desugar 补臂（构造修正）。
2. T4 根因修复超出原"实机验证"字面：触发器零高根因（centering 容器
   Fill×Fill 在 shrink 上下文解析为零高）与远程图源同步抓取挂死渲染线程
   均为 P534-D4 验收的必要修复面（"hover 直接通过"判据的构成性工作）。
3. T7 病源与计划疑点修正：非 unknown tag+href prop 组合，而是自名折叠
   递归（breadcrumb-page → fold 命中 BreadcrumbPage）。P534-D5 根因
   同此（扫描 ~55 次导航首踩病源页），非累积深度问题——原"归因另立"
   出口未启用，直接修复结案。
4. 验收第 6 条口径说明：iced 档唯一红=lucide 存量（用户基线清单内），
   tv/tf 档唯一红=charts 存量（计划允许口径）——master 同命令对账集合
   全等，零新增红。

### 规范增量（review 定稿,merge 时沉积 .autoos/specs.json）

- **P007-1 avatar 家族 VM 渲染语义**：`avatar`（容器,有子件经同表分发转换/
  无子件灰圆占位）、`avatar-image`（复用图臂）、`avatar-fallback`（文本臂）
  三臂；props 形态 `avatar (src/alt/fallback)` 对齐 vue 端 desugar 为
  AvatarImage+AvatarFallback；默认注入 w-10 h-10（对齐 vue 恒注入语义）。
  持久规则：**centering 容器（center_x/center_y）无显式宽高时被
  apply_container_style 设为 Fill×Fill,shrink 上下文（Popover 锚/行内）
  解析为零高**——新增 centering 容器组件必须带默认尺寸类（534 触发器
  零高根因）。圆形裁剪以 bg+rounded-full+尺寸类近似（VM 无 clip 原语）。
  验收：AC1/AC2。
- **P007-2 widget 子件渲染环守卫**：widget_registry 折叠兜底（P435 P8-6
  剥 `-`/`_`+小写）使未知 tag 可命中组件自身名（`breadcrumb-page`→
  BreadcrumbPage）→ 无守卫无限自递归栈溢出。规则：AuraViewBuilder 持
  active_child_widgets 进行中集合（按分支克隆传递,兄弟复用不受影响）,
  双胎 render_child_widget 入口命中环渲染 Empty（A→B→A 互递归同防）。
  命名约束沉淀：**路由页 widget 名与 demo 内 shadcn tag 折叠同名时触发**,
  页面命名避免与所演示组件的折叠名相撞。验收：AC4。
- **P007-3 远程图源同步抓取超时**：load_image_bytes 远程 URL 同步抓取
  （渲染线程上）加 3s 超时——裸 reqwest::blocking::get 网络不可达时挂死
  渲染线程（bounds 不更新/截图超时/导航无响应）。结果按 URL 进程内缓存,
  阻塞每 URL 至多一次。验收：AC1/AC2 前置。
- **P007-4 schema 再生成滞留清偿形态**：全量再生成（SCHEMA_DRIFT_GENERATE_AT=1）
  的入账口径——regen 真名登记（连字符/Pascal 翻转）、旧下划线孪生退役、
  P3 档位冲突显式拆册（vue 同件 iced 档不同时,regen 别名合并丢区分）、
  kitchen-sink 随生、DOC_EXCLUDE 折叠键。验收：AC3。


### merge 收据（PLAN-007:r1, 2026-09-09）

stage: merge | PLAN-007 | rev 1 | pass |
prepared: reviewed 基线 af07c9d37（auto-lang）/d6a48d2（auto-os）+冻结规范增量
P007-1..4;投影目标 .autoos/specs.json（P007-1..6+P007-R1 落位:reports/
architecture×4/tests/reviews）;delivery 经 review 后仅增沉积/台账/格式提交 |
landed: auto-lang master `0e7198699`（merge,plan577 4/4+check 0 错冒烟）;
auto-os main `0b3bd23`（merge）——两仓 ancestry 经 merge 提交实证 |
ledger_refreshed: .autoos/specs.json（worktree 内预备 a54c75e → merge 落
main）,item IDs P007-1..6+P007-R1,file 指向 archive 路径,JSON 校验通过;
台账 docs/plans/autos-desktop-program.md 007 行翻 📦;auto-lang
docs/plans/INDEX.md 随迁表行翻 📦 |
archived: docs/plans/archive/007-p534-debt-batch-1.md（git mv）;completion_kind:
delivered |
cleaned: 待填（拆除后回填） |

备注:并行会话期间 auto-lang master 前移（594 merge/595 review),merge 零冲突
（重叠文件无并行改动）;master 未提交的 v05/website 改动为并行会话所有,
原样保留未触碰。

### review 记录（2026-09-09）

stage: review | PLAN-007 | rev 1 | pass |
reviewed_commit: auto-lang os-007-dev `af07c9d37`（T1-T9+review 格式修正,
6 提交）;auto-os os-007-dev `d6a48d2`（T6 kitchen-sink+T9 README/gitignore） |
base_commit: auto-lang `03a72b9f8`;auto-os `2afd1a8` |
dependency_revisions: auto-down 零改动（纯路径依赖位） |
spec_inputs: 无 canonical specs 被修改（.autoos/specs.json 沉积属 merge） |
acceptance_results: AC1 pass/AC2 pass/AC3 pass/AC4 pass/AC5 pass/AC6 pass |
findings: F1（已修）——新增代码 3 处 rustfmt 偏好差异（净漂移 637→638）,
review 中按 rustfmt 形态修正后与 master 持平（637）,行为零变化
（plan577 4/4+hover/breadcrumb 实机对最终 HEAD 重验） |
evidence: 三围栏 2/2+4/4+7/7（AUTO_OS_ROOT 三仓围栏,worktree HEAD 复跑）;
plan577 探针+回归 4/4（含红灯 STATUS_STACK_OVERFLOW 验证在案）;AC2 行为
双轮实机 ALL PASS（hit area @rect(781,496,40,40) h=40,hover 进
__dlg_open_1=true/出=false,scratch/p577/t4hc_*）;AC1 像素密度复测
（content 0.79/0.79,gray 0.70）;AC4 fullscan_report.md 68/68 零异常存活
+t8 直达 ALIVE（len=33598,对最终 HEAD 重验）;AC5 KNOWN-DEBT 三条 ✅ 已偿还
（worktree 提交 52a417be0）;AC6 tf no-fail-fast 唯一红=charts,master 同
命令集合全等;iced 唯一红=lucide 存量（master 集合全等）;tv 唯一红=charts
存量 |
next: /auto-plan:merge 007

独立性声明：本次 review 在执行会话内进行（无独立角色）——按技能要求以
工件重建判定（diff 审读+围栏/探针/实机复跑+像素/快照复核），未采信
执行摘要。

## 待澄清事项

1. **P534-D3（vue 基线运行对照）与 MCP overlay 注入通道不在本批**——
   前者依赖 npm 环境建设,后者偏基础设施,均维持债务;若用户希望并入,
   本批扩为二期再议。
2. **breadcrumb 修复出口边界**:病源若在深层运行时（如 link 路由解析环
   牵动 router 全链）,浅修（页面构造规避）与根治的取舍在 T7 定位后裁定,
   超边界走"归因报告+独立计划"预设出口（验收第 5 条已含该出口形态）。
3. **avatar 圆形裁剪**:VM 无 clip 原语,圆形以 bg-muted+rounded-full+尺寸
   类近似（528 期 VM 形状能力既有口径）,不做真圆像素裁剪;图源加载失败
   占位沿 image 既有语义。
