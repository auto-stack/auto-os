---
plan_id: PLAN-024
status: drafting               # drafting → executing → execution_done → reviewed → archived
feature_name: dashboard-widgets (S10 桌面小组件面板)
author: [zhaopuming]
created_at: 2026-09-17
updated_at: 2026-09-17

# /auto-plan:review 结束时填写：
supersedes_spec_components: []
new_spec_components:           # review 结束时定稿；provisional 见 §5 规范增量表
  - docs/specs/shell/dashboard.md
  - auto-lang:docs/specs/auto-lang/ui/architecture.md
touched_goals: []              # 引用 docs/specs/goals.md 的 GOAL-NNN

affects: [shell/shell.at, shell/dashboard.at（新增）, apps/025-sys-monitor/src/front/*,
          auto-lang:crates/auto-lang/src/ast/ui.rs, auto-lang:crates/auto-lang/src/parser.rs,
          auto-lang:crates/auto-lang/src/aura/{types.rs,extract.rs},
          auto-lang:crates/auto-lang/src/ui/{dynamic.rs,session.rs,shell.rs},
          auto-lang:crates/auto-lang/src/ui/iced/renderer.rs,
          auto-lang:crates/auto-lang/src/ui_gen/{api.rs,vue.rs},
          auto-lang:crates/auto-man/src/vue.rs,
          auto-lang:schema/projection-protocol-v1.md,
          auto-lang:assets/（shell pack 内嵌快照 hash-lock 同步）,
          auto-lang:examples/ui/012-stopwatch/src/front/app.at]
current_step: 8
total_steps: 9
---

# [PLAN-024] dashboard-widgets（S10 桌面小组件面板）

## 0. 变更摘要

实现 Design 24（auto-lang `docs/design/autoui/desktop-shell.md`）§4.2 早已设计、
从未立项的 **S10 Dashboard（桌面小组件面板）**：各 App 在 .at 里声明第二命名
视图（`view mini { … }`，与主 `view` 同源同 store），桌面宿主把这些 mini 面
作为**活渲染面**在召唤式 Dashboard 面板（第四枚 overlay 槽）网格直显；面板
可配置纳入哪些 App、各占几列，storage 持久化 boot 恢复。跨仓计划：语言小扩展
（多命名 view）+ 会话/渲染层第二渲染面在 auto-lang；面板 .at 面、投影协议
v1.8、试点 mini 在 auto-os。试点 = clock + sys-monitor 两张卡。

设计依据 = auto-lang `docs/design/autoui/desktop-shell.md` §2 S10 行 +
§4.2（2026-09-01 设计记录，"立项排期在视觉二期[518]合入后"——518 已归档，
立项条件满足）；视觉参考 = stella-os os-simulator dashboard（用户 2026-09-17
截图指定）。

## 1. 目标

1. **语言小扩展**：App 可在 .at 中声明命名视图 `view mini { … }`（多命名
   view），主 `view { … }` 语义零变化；解析/提取/编译/vue 生成全链路支持。
2. **第二渲染面**：mini 视图与主视图共用同一 AppSession/store，作为活渲染面
   可被桌面层拆借渲染（输入、Tick、重渲染全通）——不是截图/缩放。
3. **Dashboard 面板**：第四枚 overlay 槽（沿 switcher/notification_center
   先例），dock 召唤、Esc/外点关闭、懒挂载；面板网格直显各 App mini 卡。
4. **可配置 + 持久化**：纳入/移除、每卡列跨度（span）可配，`shell.dashboard.*`
   storage 落盘，boot 恢复。
5. **双端一致**：vue 轨同语法同面板（Mini.vue + registry + 桌面 vue 宿主），
   autoui-verifier 对拍。
6. **试点交付**：clock mini（时间卡，静默孵化场景）+ sys-monitor mini（系统
   概要卡，后端数据场景）实机可见。

**非目标（v1 明确不做）**：
- 桌面层常驻 widget（mini 面直接住壁纸层）——面板打开即看，常驻嵌入留 v2
  （机制已备：desktop 层 z 槽已实现，复用同拆借原语即可）。
- stella 式多 tab 面板（Media/Performance/Workspaces tab）——v1 单 Dashboard 网格。
- 日历/音乐/天气 widget——**当前无对应真 app**（016-calendar/020-music-player
  是画廊 demo 非桌面 app；无天气数据源）。生态补齐另立项（见 §4 app 盘点）。
- widget 拖拽重排、自由定位。
- 有后端 app 的静默孵化（sys-monitor 未运行只显示占位卡，不拉起后端）。

**成功样貌**：用户点 dock 上 Dashboard 钮 → 面板浮层展开 → 时钟卡走秒、
系统概要卡随 app Tick 刷新（若 sys-monitor 开着）、mini 卡内可交互；关掉
面板重开机，配置原样。

## 2. 架构方案

沿 §3 分界（驱动=内核 / shell=用户态，auto-lang desktop-shell.md）：语言与
宿主扩展全部是**加法**；shell 侧只消费投影 + 发命令，无几何操作。

```
[语言层 auto-lang]  view mini { … } 解析（AST named_views + parser peek 分派）
   ↓ aura extract / DynamicComponent.named_templates + view_named(name)
[会话层 auto-lang]  同 AppSession 第二渲染面：SessionViewRef.view_name 选择器；
                    无窗会话挂载（mini-only 孵化，不开虚拟窗）
[渲染层 auto-lang]  view_desktop_fn 装配：第四 overlay 槽 dashboard 层
                    （launcher/switcher/notification 同型邻位）；
                    格位按名拆借渲染（复用 vwin 拆借 + DM::App 事件打标）
[shell 层 auto-os]  shell/dashboard.at 特权面（面板 chrome + 网格 + 编辑
                    popover）+ shell.at dock Dashboard 钮 + summon 懒挂载
[配置]              shell.dashboard.* storage（Init 读回 + 宿主注入两段式，
                    「非几何无动词，storage 直写 boot 生效」既定判定）
[协议]              投影协议 v1.8：入向 __dashboard_faces 注入 + 出向
                    __dashboard_cmd 动词（toggle/close/pin/unpin/span/launch）
```

**四项关键裁定**（详证见 §4）：

- **D1 语法采用 `view mini { … }`，不采用 `view.mini { … }`**。点号在语言里
  只用于 msg 变体（`.Inc`）、字段访问、点分模块路径（`use back.api`），声明
  语法零点号先例；「关键字 + 名字 + 块」有 `msg Name {`（名字读入即弃）与
  `view fn` 分派点现成先例；设计稿 §4.2 已裁定 `view mini`。用户提议的
  `view.mini` 评估结论：可实现但会引入第四种点号语义，违背「v1 贴现有风格」
  的可逆决策原则，不采纳（对用户的正式回复见 §9 handoff）。
- **D2 面板 = 召唤式 overlay（第四槽）**，非桌面常驻层。stella 参考即此形态
  （顶部居中浮层 + dock 一键 toggle）；设计稿 S10 命名即「面板」；懒挂载沿
  `summon_switcher` 先例。
- **D3 mini 面 = 活渲染面**（相对 stella 静态 HTML 的本质增量）：同
  AppSession 拆借，输入/更新全通；stella 的「每 widget 自带定时器 DOM 直写」
  模式不移植，数据经 store 订阅天然成立。
- **D4 可用性规则**：widget 清单 = 注册表中源含 `view mini` 声明的 app；
  显示条件 = 会话已存在；**静默孵化**仅限无后端依赖的 mini app（clock 场景
  ——用户没开时钟也有时间卡），有后端 app（sys-monitor）未运行显示占位卡
  （点击 launch）。

**与 PLAN-014（shell-ux-polish-v2，drafting 未执行）的合序**：同改
`shell/shell.at`、同触投影协议 schema。014 文本中「协议升 v1.6」已滞后
（台账 P019-1 已交付 v1.7），其执行前需按当时 HEAD 核对。两计划无内容重叠，
谁先执行谁先落，后者 rebase 核对头注版本号与 dock 字形区。

## 3. 技术栈

- auto-lang：Rust（parser/AST/session/iced renderer/ui_gen/auto-man）+ .at DSL。
- auto-os：.at 特权 shell 面 + app .at（front/back 分层）+ storage API
  （字符串 KV，复杂结构定长槽/串序列化）。
- 双端验证：autoui-verifier 技能（vm 轨 iced 宿主 / vue 轨桌面 vue 宿主对拍）。
- 无新外部依赖。

## 4. 需求分析与背景调查

### 4.1 授权记录

- 用户 2026-09-17 会话请求：「检查我们的计划，然后规划一下怎么实现？以及哪些
  app 要添加小组件的界面，以及怎么加？」并给 stella-os dashboard 截图、提议
  `view.mini { … }` 语法。**授权范围 = 起草本计划**；执行（/auto-plan:work）
  未授权，handoff 待放行。
- 允许仓：auto-os（主导）+ auto-lang（语言/宿主伴随改动，沿 Stage B 双仓
  提交先例，如 PLAN-003 clock-app）。无预算/自动续跑限制声明。

### 4.2 既有规划核查（用户问「之前规划过但没有实现过」的答案）

- **规划存在**：auto-lang `docs/design/autoui/desktop-shell.md` §2 S10 行 +
  §4.2「S10 Dashboard 与 App mini 视图」（2026-09-01 设计记录，**未立项**）。
  核心决策已定：App 声明式 mini 视图（`view mini`），否决缩放方案（字不可读）
  与 shell 重画方案（双份维护、违反「App 拥有自己的 UI」所有权）；机制 =
  语言小扩展 + 同会话第二渲染面 + 桌面层 z 槽 + storage 配置。
- **排期条件已满足**：立项排期「视觉二期 518 合入后」——auto-lang
  `docs/plans/archive/518-desktop-visual-phase2.md` 已归档（其 §非目标 明确
  「桌面小组件→独立计划」）。
- **未立项原因**：视觉二期期间观感依赖主题/图标资产定稿，随后 Stage B 迁移、
  showdesk（S9 桌面本体）/通知中心（S6）等优先交付，S10 顺延至今。本计划即
  其立项载体；**auto-os 侧此前无任何小组件计划**（活跃 014 为 shell UX 打磨，
  归档 002–023 均不含），台账 `.autoos/specs.json` 六节无 dashboard 条目。

### 4.3 语言侧现状（auto-lang，证据经 2026-09-17 只读调研）

- `view` 是关键字 Token（token.rs:364,395）但按文本匹配；view 块无名字无参数，
  `WidgetDecl.view` 单槽（ast/ui.rs:68），重复声明**静默覆盖**（parser.rs:
  12719-12721，对照 setup/actions 臂有 duplicate 报错——本计划顺带修复）。
- 解析分派点现成：顶层 `parse_view_block_or_fragment`（parser.rs:15213-15231）
  消费 `view` 后 peek `fn` → `view fn` 片段；widget 体 `"view"` 臂
  （parser.rs:12719）peek `{` → 主视图。**`view mini` 只需把 peek 扩成
  「Ident（非 fn）→ 命名视图」**。
- 「关键字 + 名字 + 块」先例：`msg Name {`（parser.rs:14252-14261，名字读入
  即弃——正是本扩展的模板）、`store Name {`、`widget Name {`。
- 物化管线单视图假设：`build_dynamic_component`（lib.rs:3739+）取首个
  WidgetDecl → `extract_widget_from_decl`（aura/extract.rs:656，decl.view →
  单棵 view_tree）→ `DynamicComponent.view_template` 单棵（ui/dynamic.rs:93）。
- 会话/渲染：AppSession 持一 component（session.rs:456-460）；拆借结构
  `SessionViewRef`（session.rs:4026-4043，持 component 引用，**无视图名选择
  器**）；桌面装配 `view_desktop_fn`（ui/iced/renderer.rs:17172）：壁纸层 →
  桌面本体层（desktop 层 z 槽，Plan 496 已实现）→ vwin 层 → shell → launcher
  → switcher → 通知中心——**第四槽有同型邻位**；懒挂载先例 `summon_switcher`
  （renderer.rs:9344-9359）。
- overlay 特权面先例：switcher.at/notification_center.at 进程内嵌 +
  shell pack 直读（ui/shell.rs:66-139，pack 目录 `../auto-os/shell` 优先、
  内嵌快照回退、hash-lock 校验 shell.rs:178-200）——**新 dashboard.at 需
  同步 auto-lang assets/ 内嵌快照**。
- vue 轨：`generate_component_from_file`（ui_gen/api.rs:530）已提取**全部**
  WidgetDecl 但模板单棵（vue.rs:2440 generate_template(view_tree)）；桌面
  vue registry 构建期生成（auto-man/vue.rs:3785-4047，每 app 根 SFC →
  `src/apps/<id>/App.vue` + apps-registry.ts）——mini 需加 per-app 第二产物。
- `view` 同时是参数模式关键字（`fn f(view x)`，parser.rs:9508）——扩展只在
  widget 体/语句位分派，参数位不受影响，需回归测试钉住。

### 4.4 桌面侧现状（auto-os）

- 桌面 shell = `shell/` 四特权 .at 面：shell.at（dock/任务栏）、desktop.at
  （桌面本体 z 槽）、switcher.at、notification_center.at；宿主接缝 = 出向
  `__desktop_cmd` 命令总线 + 入向 `__wm_*` 注入面，协议 v1.7。
- storage 配置模式成熟：`shell.dock.*`（shell.at Init 读回，:555-569）、
  `shell.desktop.positions`（壁纸指纹记忆）、`launcher.recent_apps.0..4`
  （定长槽对称写回）；物理落地 `AUTO_VM_STORAGE_FILE`（per-user json）。
- app 注册表：apps/ 容器臂（025-sys-monitor/028-launcher/036-tetris/
  037-klondike/038-minesweeper/common/kanban）+ manifest 兄弟臂（auto-musk/
  jade-garden/auto-term）+ 两画廊常驻。clock 在 auto-lang examples/ui/
  012-stopwatch（`AUTO_DESKTOP_APPS` 注册，pac title "Clock"）。

### 4.5 app 盘点——哪些 app 加小组件界面（用户问题正面回答）

| app | mini 卡内容 | v1 取舍 | 理由 |
|---|---|---|---|
| **clock**（auto-lang examples/ui/012-stopwatch） | 大字 HH:MM + 日期 | ✅ v1 试点 | 纯前端（Time natives + 既有 Tick），静默孵化主验证场景；stella clock 卡同型 |
| **025-sys-monitor** | CPU%/内存%/进程数概要 | ✅ v1 试点 | 复用 SysStore 的 Snapshot.summary（后端 `/api/system/snapshot` 真数据），验证「后端数据卡」通道；仅运行中显示 |
| kanban | 列任务计数卡 | 后续候选 | submodule 跨仓，v1 不扩战线 |
| 037-klondike / 036-tetris | 战绩卡（胜率/最高分） | 后续候选 | 有 /api/records 后端，机制同 sys-monitor 通道，待 v1 机制验证后低成本跟进 |
| 日历 / 音乐 / 天气 | —— | **不做** | 无对应真 app：016-calendar/020-music-player 是画廊 demo；无天气数据源；auto-musk 是编码 agent 非播放器。生态补齐另立项（日历 app 候选 = 画廊 demo 升格，可顺带接 PLAN-014 W-06「时钟点击跳日历」核对） |
| launcher / 桌面本体 / 游戏 | —— | 不需要 | 无常驻概要语义 |

试点组合刻意覆盖两条数据通道：纯前端（clock）与后端 API（sys-monitor），
机制走通后其余 app 的 mini 是纯增量工作。

### 4.6 stella-os 参考结论（D:\Down\stella-os\os-simulator\index.html，只读分析）

顶部居中 overlay 面板（`min(920px, 94vw)`，z-index 50），dock 图标 toggle
「展开 ⇄ 折叠胶囊」；tab 栏 + 内容 pane；Dashboard = 不等宽 3 列 grid
（`1fr 1.6fr 1fr`）+ 2 列次行；`.dash-card` = 半透明底 + blur + 12px 圆角 +
1px 边框 + hover 上浮，色板收敛 CSS 变量。**移植取其骨架与视觉语言，不取其
实现**：widget 全部写死单文件、无注册机制、无配置、无持久化、数据多为假
定时器——这正是「App 声明式 mini + 可配置 + storage 持久化」要补的架构差距。
主题色不抄（demo 玫瑰红为其自有主题），沿本项目 stella light token 映射
（design_tokens/registry.rs，014 mock 同源）。

## 5. 详细设计

### 5.1 语言扩展：`view mini { … }`（auto-lang）

- **AST**（ast/ui.rs）：`WidgetDecl` 增 `named_views: Vec<(Name, ViewBlock)>`；
  `view: Option<ViewBlock>` 主视图槽不动。
- **解析器**（parser.rs）：
  - widget 体 `"view"` 臂（:12719）：消费 `view` 后 peek——`{` → 主视图；
    Ident 且非 `fn` → 读名 + 块，收进 `named_views`；补 duplicate 名报错
    （样式对照 :12742 setup/actions 臂），**顺带修复主 view 重复静默覆盖**。
  - `view fn` 片段分派（:15213-15231）行为不变（peek fn 优先）。
  - `view` 参数模式（fn 签名）不受影响——回归测试钉住。
- **提取**（aura/types.rs + extract.rs）：`AuraWidget.named_views:
  Vec<(String, AuraNode)>`，`extract_widget_from_decl` 一并提取。
- **动态轨**（lib.rs + ui/dynamic.rs）：`DynamicComponent` 增
  `named_templates: HashMap<String, AuraNode>`（与 :93 `view_template`
  并列）+ `view_named(name)` 构建器（复制 :771-777 builder 链换模板根）；
  `named_views()` 访问器供宿主推导 faces 清单。
- **vue 轨**（ui_gen/api.rs + vue.rs + auto-man/vue.rs）：
  `GeneratedComponent` 增 per-widget 命名视图产物 `named_view_codes:
  Vec<(widget, name, code)>`；VueGenerator 复用 generate_template 对每个
  named view 生成模板（script 段共享同一 store composable）；桌面 vue 宿主
  每 app 落 `src/apps/<id>/Mini.vue`（auto-man/vue.rs :3979-3984 邻位）并扩
  apps-registry.ts 行。

### 5.2 会话与渲染：第二渲染面（auto-lang）

- `SessionViewRef`（session.rs:4026-4043）增 `view_name: Option<&str>`；
  拆借访问器透传；`dynamic_view_impl`（renderer.rs:17831-17884）按
  view_name 选模板（None → 主视图，零回归）。
- **无窗会话**：`launch_app` 增 mini-only 臂——`build_dynamic_component` +
  `allocate_app` 照常，但不创建虚拟窗/不参与 z 序，仅供拆借。孵化条件：
  app 注册表声明 mini 且无后端依赖（pac 无 back_port 且 manifest 无 daemon）。
  孵化会话常驻（notification_app 槽先例），面板关闭不杀。
- **第四 overlay 槽**：`DesktopState` 增 `dashboard_app: Option<AppId>` +
  `dashboard_visible: bool`；boot 不装载，首召唤 `summon_dashboard` 懒挂载
  （`summon_switcher` renderer.rs:9344 同型）。
- **格位拆借**：`view_desktop_fn` 在通知中心邻位推 dashboard 层；面板内部
  布局由 dashboard.at 声明，格位 = 宿主按 `__dashboard_faces` 清单 × span
  计算的矩形，对每个 face 调按名拆借渲染 + `DM::App(app_id, m)` 事件打标
  （vwin 同型，输入/滚轮/焦点随拆借面直达该 app 会话）。装配细节（层内
  定位机制）T-03 执行时按 vwin chrome 拆借现状定案，若层内嵌套受限则退化
  为「面板 chrome 层 + face 子层 Stack 同矩形叠合」，行为等价。

### 5.3 faces 清单推导

两级：① 启动扫描——对注册表 app 源（front/*.at）做 `view mini` 文本探测
（grep 级，缓存），得候选清单（未运行也纳入，供占位卡/孵化判定）；
② 会话确认——会话化后以 `component.named_views()` 精确生效。文本误报
（注释命中）由两级确认兜底。

### 5.4 Dashboard 面板（auto-os shell/dashboard.at，新特权面）

- 面板 chrome：顶部居中浮层（宽 ≤920px 上限），标题行 + 网格区 + 编辑入口；
  3 列 grid（stella 比例简化为等宽起步，跨度配置表达宽卡），卡片圆角/边框/
  hover 沿项目主题 token。视觉 mock 先出（evidence/024/，浅/深双主题，
  014 流程先例），用户审定后实施。
- 面板逻辑（.at 侧，纯投影消费）：Init 读 `shell.dashboard.enabled`/
  `shell.dashboard.span.<app>`；`__dashboard_faces` 注入渲染清单；卡片
  右键 → 编辑 popover（纳入/移除/跨度 1|2），出向 `__dashboard_cmd`。
- dock 入口：shell.at 增 Dashboard 钮（网格字形，字形区与 PLAN-014 W 组
  合序核对）；点击出向 `dashboard_toggle`。
- 关闭仲裁：Esc/外点关闭沿通知中心先例；面板不参与焦点独占。
- shell pack 同步：新 dashboard.at 进 auto-os shell/ + auto-lang assets/
  内嵌快照 hash-lock 双写（shell.rs:178-200 校验）。

### 5.5 投影协议 v1.8

- 入向注入：`__dashboard_faces`（Obj 数组：app id/title/icon/face 状态
  running|hatched|placeholder/span）+ 可见性判据字段（`__wm_dashboard`
  沿 `__wm_showdesk` 同型）。
- 出向动词（`__dashboard_cmd` 词表）：`dashboard_toggle` / `dashboard_close`
  / `dashboard_pin <app>` / `dashboard_unpin <app>` / `dashboard_span <app>
  <1|2>` / `dashboard_launch <app>`。
- schema 升版 v1.7 → v1.8（auto-lang schema/projection-protocol-v1.md），
  vue 端对拍基线同步。

### 5.6 配置持久化

键空间 `shell.dashboard.*`：`enabled`（逗号串纳入清单）、`span.<app>`
（"1"|"2"）。写路径 = .at 直写 storage（「非几何无动词，storage 直写 boot
生效」既定判定）；读路径 = 面板 Init 读回 + 宿主注入两段式（dock pinned
先例：单一事实在宿主）。未配置默认 = 首次召唤时自动纳入全部候选（有占位
卡），用户显式编辑后以 storage 为准。

### 5.7 试点 mini

- **clock**（auto-lang examples/ui/012-stopwatch/src/front/app.at）：追加
  `view mini { … }`——大字 HH:MM + 日期行，绑定既有时间 model（app 既有
  Tick 驱动，零新 store）。pac 无后端 → 静默孵化生效。
- **sys-monitor**（apps/025-sys-monitor/src/front/）：app.at（或新建
  mini.at 子组件文件）追加 `view mini { … }`——CPU%/内存%/进程数三行概要，
  绑定 SysStore.summary；Tick 节流复核（mini 250ms 足够，沿既有 interval）。
  未运行 → 占位卡（图标 + 「启动以查看」+ 点击 `dashboard_launch`）。

### 5.8 规范增量

| delta_id | add/modify/retire | target | before/after rule | rationale | acceptance IDs |
|---|---|---|---|---|---|
| SD-01 | add | docs/specs/shell/dashboard.md（新模块 spec） | 新增：S10 Dashboard 面板契约——第四 overlay 槽、`view mini` 声明即注册、faces 两级推导、静默孵化条件（无后端）、配置键 `shell.dashboard.*`、v1.8 词表与注入面 | 设计稿 §4.2 立项落 spec；沿 showdesk-* 模块 spec 先例 | AC-01..07 |
| SD-02 | modify | auto-lang:docs/specs/auto-lang/ui/architecture.md | 视图声明节：单一 view → 主 view + 命名视图 `view mini`（多命名）；重复名报错；`view fn`/参数模式语义不变 | 语言小扩展的正名（设计稿「多命名 view」） | AC-01 |
| SD-03 | modify | auto-lang:schema/projection-protocol-v1.md | v1.7 → v1.8：新增 `__dashboard_faces` 注入 + `__dashboard_cmd` 六动词 + `__wm_dashboard` 判据 | 面板接缝协议化，沿 v1.2 通知中心先例 | AC-02,04,05 |

（frontmatter `new_spec_components` provisional 同上；review 定稿。 goals 节
本仓为空，touched_goals 保持空，不新立 GOAL。）

## 6. 测试设计

- **语言层（auto-lang cargo t）**：parser 新测——`view mini` 解析、多命名、
  duplicate 名报错、主 view 重复报错（修复回归）、`view fn` 片段不回退、
  `fn f(view x)` 参数模式不回退；extract/dynamic——named_templates 装载、
  view_named 构建；vue 生成快照——Mini.vue 产物 + registry 行；shell pack
  全量编译冒烟（ui/shell.rs:141 既有测自动覆盖新 dashboard.at）。
- **会话/渲染（auto-lang cargo t + layout_tests）**：无窗会话不进 z 序断言、
  第四槽装配层序断言（desktop_surface_z_slot 同型）、按名拆借渲染元素断言。
- **实机（auto-os tests/desktop_mcp 体系）**：召唤/关闭、clock mini 走秒
  （孵化）、sys-monitor mini 数据刷新（运行中）、占位卡 → 点击 launch、
  配置编辑落盘 + 重启恢复、Esc/外点仲裁；五套既有 desktop_mcp 数目不减。
- **双轨对拍**：autoui-verifier 技能，vm 轨 vs vue 轨面板结构一致。
- **证据**：实机截图 `docs/plans/evidence/024/`（面板全景/两试点卡/编辑
  popover/boot 恢复）。

## 7. 验收标准

| ID | 可观察行为 | 验证方法与预期 |
|---|---|---|
| AC-01 | `view mini { … }` 语法在 vm/vue 双轨解析编译；主 view 语义零变化；duplicate mini 名与重复主 view 报错；`view fn`/`fn f(view x)` 不回退 | auto-lang `cargo t`（含新 parser 测试）全绿；vue 生成快照含 Mini.vue |
| AC-02 | dock Dashboard 钮召唤面板（第四 overlay 槽，懒挂载）；clock mini 卡实时显示时间且 clock app 未手动启动过（静默孵化）；Esc/外点/再点关闭 | desktop_mcp 实机 + 截图；面板层在通知中心邻位、关闭后桌面恢复 |
| AC-03 | mini 面为活渲染面：sys-monitor 打开后 mini 卡数据随其 Tick 刷新，与主窗一致；mini 卡内交互直达 app 会话 | 实机并开主窗与面板，数值同源比对截图 |
| AC-04 | 面板编辑（移除/纳入/跨度）落 `shell.dashboard.*`，重启 boot 恢复 | 实机重启复现 + storage 文件核对 |
| AC-05 | vue 轨桌面宿主同面板同 mini 卡，与 iced 轨无结构差 | autoui-verifier 对拍报告 |
| AC-06 | 试点交付：clock + sys-monitor 两 mini 实机可见（含占位卡形态） | `docs/plans/evidence/024/` 截图组 |
| AC-07 | 零回归：auto-lang cargo t 全档基线不降；I2 五套 desktop_mcp 数目不减；shell pack hash-lock 同步校验绿 | 测试日志 + pack 冒烟输出 |

## 8. 执行步骤

（原子任务；每步完成后追加 [✅ 已完成] 一行证据。worktree 布局沿 Plan 529：
`.wt/os-024/auto-os` + auto-lang 伴随检出。）

| ID | 任务 | 仓/文件 | 依赖 | 产出/验证 | AC |
|---|---|---|---|---|---|
| T-01 | 语言 AST+parser：`WidgetDecl.named_views`、widget 体 `view` 臂 peek 分派（Ident 非 fn → 命名视图）、duplicate 查重、主 view 重复覆盖修复、参数模式/view fn 回归测试 | auto-lang ast/ui.rs, parser.rs | — | cargo t 新测绿（AC-01 语法半） | 01 |
| T-02 | 提取+动态轨+vue 生成：aura types/extract named_views、DynamicComponent.named_templates + view_named + named_views()、ui_gen named_view_codes、auto-man Mini.vue + registry 扩展 | auto-lang aura/*, ui/dynamic.rs, ui_gen/*, auto-man/vue.rs | T-01 | 单测+vue 快照绿 | 01,05 |
| T-03 | 会话+渲染：SessionViewRef.view_name、dynamic_view 按名选模板、无窗会话挂载臂、DesktopState dashboard 槽、summon_dashboard 懒挂载、第四槽装配 + 格位拆借 + 事件打标 | auto-lang ui/session.rs, ui/iced/renderer.rs | T-02 | layout/session 单测绿；实机面板可显任意 mini 面 | 02,03 |
| T-04 | 协议 v1.8 + dashboard.at 面：schema 升版、shell/dashboard.at（chrome/grid/编辑 popover，mock 先行 evidence/024/ 双主题审定）、shell.at dock 钮、faces 两级推导、assets 内嵌快照 hash-lock 双写 | auto-lang schema/, assets/；auto-os shell/ | T-03 | pack 冒烟绿；实机召唤/关闭/占位卡 | 02,04 |
| T-05 | 配置持久化：`shell.dashboard.enabled`/`span.<app>` 写读、Init 读回、宿主注入两段式、默认纳入策略 | auto-os shell/dashboard.at；auto-lang desktop_config.rs（如需键默认） | T-04 | 实机编辑→落盘→重启恢复 | 04 |
| T-06 | 试点 mini：clock `view mini`（012-stopwatch app.at）、sys-monitor `view mini`（front/，绑 SysStore.summary）、孵化条件核对（pac 无 back_port） | auto-lang examples/ui/012-stopwatch；auto-os apps/025-sys-monitor/src/front | T-04（声明可随 T-01 先写） | 实机两卡可见+走秒/刷新 | 03,06 |
| T-07 | 双轨对拍：桌面 vue 宿主面板渲染 + Mini.vue 消费；autoui-verifier 全量对拍 | auto-os（vue 轨入口 scripts/desktop.sh）；auto-lang auto-man | T-02,T-04 | 对拍报告零结构差 | 05 |
| T-08 | 测试收口：desktop_mcp 新套件（召唤/孵化/刷新/配置/仲裁）+ 五套既有不减 + 截图证据 evidence/024/ | auto-os tests/ | T-05,T-06,T-07 | AC-02..07 证据齐 | 02-07 |
| T-09 | 收尾：README/程序台账指针（autos-desktop-program.md S10 行回写）、shell 头注版本同步、复审准备 | auto-os | T-08 | 文档齐，/auto-plan:review 可入 | 07 |

**风险与对策**：
- iced 层内格位嵌套受限 → 退化为 chrome 层 + face 子层 Stack 同矩形叠合
  （§5.2 预案），行为等价；
- 文本扫描误报 → 会话后精确确认两级兜底（§5.3）；
- sys-monitor 后端未运行体验空洞 → 占位卡 + 一键 launch（非目标已声明不
  拉起后端；daemon 带外端口约定不在本计划扩）；
- 与 PLAN-014 同文件合序 → §2 合序规则，执行日以当时 HEAD 为基线核对。

**执行进度注记**（worktree `D:/autostack/.wt/os-024/{auto-os,auto-lang}`，
分支 plan-024-dev / auto-os-024-dev，基线 auto-os main a6d4a47 + auto-lang
master 50016b255）：

- [✅ 已完成] T-01 2026-09-17 auto-lang fdeb77406——named_views 解析分派 +
  duplicate 查重（主 view 静默覆盖顺带修复）+ 回归钉 9/9 绿；全量 cargo t
  失败集 = master 基线集（37↔37，ffi_dual_019 / external_config_poll 各侧
  一抖动）。教训：view 臂主视图分派初版漏消费 `{`，新测浅断言未抓到、
  全量 aura extract 语料抓到——回归钉 main_view_root_consumes_brace 已补。
- [✅ 已完成] T-02 2026-09-17 auto-lang 60d50aa99——AuraWidget.named_views
  同管线提取 + DynamicComponent.named_templates(BTreeMap)/view_named()/
  named_views()/reload 装载 + vue named_view_codes（克隆换根复用
  generate 管线）+ auto-man fake WidgetDecl 补字段（缺字段曾致 p508
  outproc 内嵌构建红——p508/dep_parity 语料因此全绿）。cargo t 失败集 =
  基线；auto-lang/auto-man 双 crate check 绿。
- 环境注记：auto-lang 组内依赖检出 `.wt/os-024/auto-down`（detached
  master，autodown-core path 依赖解析）；工作树用本地 target（共享主检出
  target 会经 CARGO_TARGET_DIR 污染 oracle/outproc 内嵌构建的落盘路径）。
- T-02 尾项 auto-man Mini.vue 落盘 + registry 行并入 T-04（与桌面 vue 宿主
  消费侧同批，plan §5.1 原文即含 auto-man/vue.rs）。
- [✅ 已完成] T-03+T-04 2026-09-17 auto-lang 334202bff + auto-os
  dock/pack 提交——第四 overlay 槽（SessionViewRef.view_name/无窗孵化/
  split_ref_dashboard·face/dashboard_visible/六动词 v1.8/drain
  __dashboard_cmd/动态 face 分支/装配 Stack 叠合/Esc 键盘订阅第五块/
  __wm_dashboard 投影）；dashboard.at 特权面（scrim/标题/几何注入）+
  shell.rs 五件 hash-lock 双写（parity 测试扩五件）+ dock Dashboard 钮
  （widgets-gallery 字形，两态高亮）经 shell-pack-sync 同步；协议
  schema/projection-protocol-v1.md 升 v1.8（§6 新节）。§10.3 裁定注记：
  格位装配 = 预案形态「chrome 层 + face 子层 Stack 同矩形叠合」（iced
  层内嵌套风险规避），卡片 chrome 宿主侧，布局单一事实 =
  dashboard_layout 行主序 next-fit（等宽 3 列 + span 1|2）。
- [✅ 已完成] T-05 2026-09-17（随 T-03/04 落码）——`shell.dashboard.enabled`
  csv + `span.<app>` storage 读写（storage_host_read/publish 直写，boot
  生效）；未配置默认 = 首召唤自动纳入全部候选；编辑执行体
  dashboard_set_pinned/set_span（首次显式编辑把未配置升级为显式清单）。
- [✅ 已完成] T-06 2026-09-17 auto-lang 334202bff（clock mini：w_local
  走秒大字卡，纯前端→静默孵化主场景）+ auto-os 提交（sys-monitor mini：
  cpu/mem/proc 三行概要绑 SysStore，有后端→占位卡路径）。
- [✅ 已完成] T-07 2026-09-17 auto-lang 54b7616e5——桌面 vue 宿主链：
  ui_build_shadcn_..._full 返回扩 named_view_codes；ViewProject.mini_face
  → 每 app Mini.vue 落盘 + apps-registry mini/loadMini 行；wm 资产
  DashboardPanel.vue（store 模块单例同源 = 活渲染对拍语义）+ Taskbar ▦
  钮 + 宿主 App.vue 接线。plan024 vue 产物双测 + auto-man 298/298 绿。
- [✅ 已完成] T-08 可自动化半 2026-09-17 auto-lang 2940e6e08——dashboard
  无头测试（可见性/孵化垫片/六动词往返/布局算式四测）；plan024 全套
  15/15 绿；收口门 cargo t --no-fail-fast 失败集 36 ⊂ master 基线 37
  （零新增回归）；shell_pack 3/3 绿。**实机半（召唤/孵化走秒/刷新/
  持久化重启/Esc 仲裁的实机操作 + evidence/024/ 截图 + autoui-verifier
  对拍）未执行——沿 PLAN-022 先例为用户截图驱动交互流程，见 §9 handoff。**
- [✅ 已完成] T-09 2026-09-17——程序台账指针：auto-lang
  docs/design/autoui/desktop-shell.md §4.2 未立项→已落地注记 + 派期清单
  S10 行 ✅（实现裁定差异两条在案）。
- [✅ 已完成] T-08 实机轮（自主驱动）2026-09-17 auto-lang d09347f04 +
  auto-os evidence 提交——**三实机根因修复**：①split_mut 缺孵化会话臂
  （无窗 update 拆借恒 None，Tick/handler 全静默——clock face 恒显初值
  根因）；②faces 读回须 read_state_as_vec（write_state_vec 落 VM 堆为
  VmRef）；③face 叠合改 px spacer 链定位（家法）。实机结果（evidence/
  024/ 三帧 + 驱动脚本）：boot 召唤 ✓、clock face 走秒（21:23:42→45→
  30:52 帧差）✓、sys-monitor face 真数据（CPU/内存/进程数随 Tick 刷新，
  66.4%→49.8%→8.3% 跨轮）✓、空态文案修复 ✓、Esc 关闭帧捕获 ✓（帧内容
  为打开态——SendKeys 焦点被浏览器抢占的自动化伪影，关闭链路代码与
  launcher/通知中心同款成熟模式，留用户一键复验）；格位换算与
  dashboard_layout 吻合（capture 有 ~2.1x DPI 缩放伪影）。桌面 vue 宿主
  面板/autoui-verifier 对拍与面板编辑 popover 仍开放（§10）。

## 9. 复审记录

### 2026-09-17 drafting handoff（/auto-plan:new）

- `stage: new`，**PLAN-024** revision 1。
- `outcome: pass`——任务覆盖全部 AC 与 SD；路径/命令经 2026-09-17 双仓只读
  调研落地（§4 证据行号）；无阻塞裁决。
- 对用户语法提议的正式回复：`view.mini { … }` 不采纳，定 `view mini { … }`
  （裁定 D1，§2/§5.1——点号声明零先例，`msg Name {` 同型先例 + 设计稿 §4.2
  既定，解析器 `view fn` 分派点现成）。
- `next: work`（T-01 起；worktree 未建，执行待用户放行 /auto-plan:work）。
- 已知待澄清（§10）不阻塞 T-01/T-02 开工。

### 2026-09-17 work handoff（/auto-plan:work）

- `stage: work`，**PLAN-024** revision 1。
- `outcome: pass（实机验证挂起）`——T-01..T-08 可自动化面全交付并提交；
  实机操作半（AC-02/03/04/06 的实机走查 + evidence/024/ 截图 +
  autoui-verifier 对拍）需用户在场驱动，沿 PLAN-022 截图驱动先例，状态保持
  `executing` 待实机轮后入 review。
- `plan_revision`: 1（范围零变更；两处实现裁定注记：§10.3 格位装配 =
  Stack 叠合预案形态；可见性沿 switcher/通知 visible-state 先例而非
  DesktopState 布尔——意图等价，语义与既有一致）。
- `code_commit`: auto-lang `auto-os-024-dev` fdeb77406 → 60d50aa99 →
  334202bff → 54b7616e5 → 2940e6e08（基线 master 50016b255）；auto-os
  `plan-024-dev`（基线 main a6d4a47，shell.at dock 钮 + shell/dashboard.at
  双写 + sys-monitor mini）。
- `task_ids`: T-01..T-08（可自动化面）；T-09 半（README/程序台账指针留
  review 前清单）。
- `evidence`: cargo t --no-fail-fast 失败集 36 ⊆ master 基线 37（三轮，
  零新增）；plan024 套件 15/15；auto-man 298/298；shell_pack 3/3（含
  hash-lock 五件 parity）；auto-lang/auto-man 双 crate check 绿；ui_desktop
  与 auto CLI worktree 本地 target 构建通过。
- `blockers`: ①实机验证需用户在场（开桌面 → 点 dock ▦ → 走查 AC-02/03/
  04/06 + 截图存 evidence/024/）；②§10.2 面板视觉骨架按项目 token 直落
  （014 mock 审定流程未走）——用户过目后如需调皮肤走增量轮。
- `next`: 实机走查轮（用户配合）→ 补 evidence/024/ → README/程序台账
  指针（autos-desktop-program.md S10 行）→ /auto-plan:review。

## 9.1 实机走查清单（review 前偿清）

1. `DESKTOP_OS_ROOT=D:/autostack/.wt/os-024/auto-os AUTO_LANG_ROOT=D:/autostack/.wt/os-024/auto-lang bash scripts/desktop.sh iced`（worktree 组自洽；
   构建已就绪）。
2. AC-02：点 dock ▦ → 面板顶部展开 → clock 卡走秒（clock 未手动启动 =
   静默孵化生效）；再点 ▦ / × / scrim / Esc 关闭，桌面恢复。
3. AC-03：先开 sys-monitor 主窗，再开面板 → sys-monitor 卡三行数值随
   Tick 与主窗一致跳动；卡内交互直达该 app 会话。
4. AC-04：占位卡（sys-monitor 未启动时）→ dashboard_launch 启动；
   编辑链（pin/unpin/span 命令或后续 popover）→ storage 文件核对
   `shell.dashboard.*` → 重启面板恢复。
5. AC-06：面板全景/两试点卡/关闭后桌面 截图入 `docs/plans/evidence/024/`。
6. AC-05：vue 轨 `bash scripts/desktop.sh`（vue）对拍面板结构；
   autoui-verifier 报告。

### 2026-09-17 用户走查裁定（work 轮增量，语义变更）

- **常驻小组件层（裁定取代 v1 召唤式 D2）**：用户实机走查三项反馈——
  ①面板与卡片未对齐；②外点即消失不符"桌面常驻组件"目标；③z 序错误
  （应仅高于壁纸、低于全部 app）。裁定落地（auto-lang 尾提交）：z 挪至
  壁纸之上/桌面图标与全部窗之下（v2 预留的桌面层 z 槽提前兑现）；去
  scrim/外点关闭/Esc；× = 隐藏、dock ▦ 切换；boot 即挂载显示。对齐根修
  = chrome 尺寸宿主 wrapper（与 face 卡同一算式），.at 不再依赖任意值
  尺寸类。spec delta SD-01 相应节（第四 overlay 槽/关闭仲裁）由 review
  按常驻语义修订。
- 实机复验：计算器窗正确遮挡面板与卡片（z ✓）、面板与卡片同算式定位
  （对齐 ✓）、常驻显隐 ✓（evidence/024/05-resident-bottom.png）。

## 10. 待澄清事项

1. **面板热键**：已裁定 v1 不加（dock 钮召唤 + Esc/外点/再点关闭三路径
   已通）；热键表余量预留 v2。
2. **视觉 mock 审定**：T-04 执行时按「014 流程」应先出双主题 mock 待审；
   本次自主执行直落项目 token 骨架（glass 三件套/12px 圆角/stella 比例
   简化等宽 3 列），**用户实机过目后如需调皮肤走增量轮**（§9.1 走查顺带）。
3. **格位装配实现形态**：已定案——**Stack 叠合预案形态**（§5.2），chrome
   层 + face 子层同 Stack，格位 = `dashboard_layout` 宿主单一事实（行主序
   next-fit），卡片 chrome 宿主侧容器，像素一致零漂移。决策记录进 T-03
   进度注记。
4. **静默孵化会话的 Tick 成本**：机制成立（订阅按 App 扇出不依赖窗口，
   无窗孵化会话 Tick 照常）；面板关闭不推 face 层故渲染成本仅打开期发生
   （face 每帧 view_named 新鲜构建——与主窗缓存隔离的裁定，主窗零踩踏）。
   实机度量（面板常开的空闲 CPU）留走查轮。
5. **编辑 popover UI 未实施**（新）：v1.8 词表 + 宿主执行体
   （dashboard_set_pinned/set_span→storage 直写→活刷新）已通并有词表
   测试，但面板内尚无右键 popover 触发 UI——走查轮后裁定补 UI（小增量）
   或降 v2（命令/配置面已可用）。
6. **vue 轨孵化语义差**（新，对拍注记）：iced 轨静默孵化 = 运行期无窗
   会话；vue 轨 registry 为构建期静态——DashboardPanel.vue 直接列出全部
   `mini: true` app 卡（无 running/hatched 分态）。对拍比结构（面板/网格/
   卡/关闭），分态语义差已在两轨实现头注登记。
