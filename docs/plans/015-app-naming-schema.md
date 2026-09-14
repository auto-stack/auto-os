---
plan_id: PLAN-015
status: reviewed               # drafting → executing → execution_done → reviewed → archived（review pass；merge 六仓已落，auto-lang landing 待并行会话让位——见 §9 merge 收据）
feature_name: app-naming-schema
author: [zhaopuming]
created_at: 2026-09-14
updated_at: 2026-09-14

# /auto-plan:review 结束时填写：
supersedes_spec_components: []
new_spec_components: [auto-lang/docs/specs/auto-man/project.md §「pac.at 四名称契约（PLAN-015）」]
touched_goals: []

affects: [auto-lang/crates/auto-lang/src/ui/app_registry.rs, auto-lang/crates/auto-man/src/automan.rs,
          auto-lang/crates/auto/src/main.rs, auto-os/apps.manifest, auto-os/AGENTS.md,
          auto-os/apps/*/pac.at, auto-lang/examples/ui/*/pac.at]
current_step: 7
total_steps: 7
---

# [PLAN-015] app-naming-schema

## 0. 变更摘要

桌面 21 app 名称混乱（kebab 机器名 / `Aaa Bbb` / 中文混杂，且有第三处死字段在漂移）。
本计划把每个 AutoUI app 的名称定为 **pac.at 四字段契约**，与工程标识解绑：

| # | 字段 | 状态 | 语义 |
|---|------|------|------|
| 1 | `name` | 保留不动 | 工程标识（kebab-case）：rust 轨包名/exe 缺省名、os-config 配置查找键（Plan 504 S7）、注册表条目标识 |
| 2 | `exe_name` | 已有（auto-lang Plan 014，auto-term 在用），升格标准字段 | rust 轨（a2r）exe 产物名，`[[bin]]` 显式化；合法字符 `[A-Za-z0-9_-]`；vue/VM 轨无操作 |
| 3 | `title` | 语义定版 | **英文展示名**（现有字段原样，不迁移不改名） |
| 4 | `title_zh` | **新增** | **中文展示名**，自由文本 |

配套三件事：

1. **展示名解析链加 locale 臂**：沿既有 `AUTO_LOCALE` 约定（VM i18n 同款，缺省 zh）——
   zh 取 `title_zh → title → name → 目录名`，en 取 `title → name → 目录名`。
2. **apps.manifest `name` 字段删除**：解析侧本就 `#[allow(dead_code)]` 零消费
   （`app_registry.rs:379`），中文值"通用看板（v1 计划板）"是漂移放大器；manifest
   回归纯机器登记（id/repo/ports/daemon），AGENTS.md §4 schema 样例同步。
3. **pac.at 批量补齐**：examples/ui + auto-os apps + manifest 各仓 app 补
   `title`（英）/`title_zh`（中），name 一律不动。

跨仓计划：主导仓 = 本仓（auto-os，桌面程序归属）；auto-lang 侧改动（crates +
examples/ui 批量）经 auto-lang 仓 plan/工作区分账（互链见 §4.3），或按 auto-plan
work 的组内多仓模式执行。

## 1. 目标

1. **G1** pac.at 四名称契约成文并落地：每个桌面注册 app 的 pac.at 可独立配置
   `name`/`exe_name`/`title`/`title_zh`，展示名不再依赖工程标识。
2. **G2** 桌面（launcher 图标格、dock、任务栏、VM 窗口标题）在 `AUTO_LOCALE=zh`
   （缺省）下显示中文名，`AUTO_LOCALE=en` 显示英文名；未配置 `title_zh` 的 app
   回落英文 `title`，永兜底 `name`→目录名（链路不断）。
3. **G3** exe 产物名可配：rust 轨 app 声明 `exe_name` 即得该名产物（Plan 014
   既有行为标准化，auto-term 为样板）；缺省行为不变（包名 snake_case）。
4. **G4** 名称单一事实源：apps.manifest 不再承载任何展示名；同一 app 的名称
   只在 pac.at 一处声明。
5. **G5** 全量补齐后，桌面 21 app 无一以 kebab 机器名或半截名示人（zh/en 两
   locale 下展示名完整）。

**非目标**：
- 不改任何 app 的目录名、manifest id、pac `name`（launch/dock pinned/os-config
  `apps/<name>/config.at` 查找键全部挂在标识上，动即回归）。
- 不做注册表 id 重命名、不做运行期切语言（装载期定 locale，PLAN-050 既有边界）。
- 不给 vue 轨引入 exe 产物（轨形态决定，无操作不是缺口）。
- 不动 i18n/{lang}.json 内容文案机制（应用内文案层与 app 元数据层分离）。

## 2. 架构方案

**数据流不变，解析链加一臂**。注册表扫描（`app_registry.rs`）保持平铺
`key: value` 行读（`parse_pac_fields` 通用键值解析器，新键零成本）：

```
pac.at(title/title_zh) → AppRegistryEntry { title, title_zh: Option<String> }
                              │
   ┌──────────────────────────┼──────────────────────────┐
   │ launch 臂                │ 注入臂                    │ VM 独立窗臂
   │ LaunchSpec.title 填充    │ launcher/dock 列表注入     │ automan.pac_window_title
   │ （entry.display_title）  │ （renderer.rs ~8793/8915） │ + main.rs AUTO_VM_TITLE
   ▼                         ▼                           ▼
        locale 解析统一收口 AppRegistryEntry::display_title()
        zh: title_zh → title → name → 目录名；en: title → name → 目录名
```

- **为什么扁平键而不是嵌套对象**：注册表扫描刻意"仅平铺行读、不引入 .at 全量
  解析"（`app_registry.rs` 模块注释），嵌套 `title { en, zh }` 会同时破坏该解析
  器与 auto-man 的行级 pac 读取；`parse_pac_fields` 对任意 `key: value` 透明。
- **为什么不用 i18n/{lang}.json**：那是应用内内容文案的装载机制（front 根目录、
  AUTO_LOCALE 装载期定 locale）；app 元数据名称若依赖 front 根外文件，注册表
  "一目录一扫描"模型即破。展示名属于包元数据，进 pac.at。
- **exe_name 不加第二消费点**：唯一消费点保持 `rust_ui.rs`（生成 Cargo.toml
  `[[bin]]`，regen 按 pac 现值重写）；注册表不 launch 独立 exe（VM 桌面启动走
  `auto run --autodesk-incubate` re-exec），不加字段进 `AppRegistryEntry`（YAGNI）。
  本计划只做契约成文 + 校验行为文档化。

## 3. 技术栈

- auto-lang：Rust（crates/auto-lang 注册表、crates/auto-man pac 读取、crates/auto CLI）。
- pac.at：既有平铺 DSL（无 schema 文件，契约以 spec + 解析器为准）。
- 测试：cargo t（app_registry 单测 + automan 单测）；桌面实机走 headless 指针
  （PLAN-012 先例）。

## 4. 需求分析与背景调查

### 4.1 授权与范围

- 用户 2026-09-14 裁定：app 名称与原始工程解绑、在 pac.at 配置四名称
  （工程名/exe 文件名/英文名/中文名）；auto-term 的 exe_name 配置作参考、
  统一设计。已确认按本计划范围执行（调研结论经用户"OK"认可，2026-09-14）。
- 允许仓库/动作：auto-os（plan、apps.manifest、apps/*/pac.at、AGENTS.md）、
  auto-lang（crates 三处消费点 + examples/ui/*/pac.at 批量 + spec 补节）、
  auto-kanban / auto-musk / auto-down(jade-garden) / auto-term（各自 pac.at
  批量，随 work 阶段组内分账）。不改 crates 之外的 auto-lang 构建面，
  遵守 auto-os AGENTS.md §3 验证门档（不改 auto-lang crates 时严禁在
  auto-lang 跑 cargo t——本计划改 crates，属 Category A 例外面，在
  auto-lang 侧 worktree 跑定点测试）。

### 4.2 根因调查（代码证据）

| # | 现象 | 根因 | 证据 |
|---|------|------|------|
| 1 | 展示名三态混杂（kebab/`Aaa Bbb`/中文） | `title` 语义从未定版语言；`parse_pac_fields` 无 schema 约束 | `app_registry.rs:114`（title→name→目录名兜底链）；examples/ui 实测：`012-stopwatch` name=`stopwatch`/title=`Clock`（漂移实锤）、`系统监视器`（025-sys-monitor）、`DndBridge`/`RealWorld` 连写 |
| 2 | apps.manifest 中文名示人 | `name` 字段解析即弃（dead_code），零消费仍在文件里被维护 | `app_registry.rs:377-380` `#[allow(dead_code)]`；manifest 实文 `"name": "通用看板（v1 计划板）"` |
| 3 | exe 名不可控（历史痛点） | 缺省 exe = pac name snake_case；Plan 014 已补 `exe_name` 显式 `[[bin]]`，但仅 auto-term 一家在用、未成契约 | `rust_ui.rs:1917`（parse_pac_exe_name，字符白名单 `[A-Za-z0-9_-]`）、`rust_ui.rs:1961`（generate_cargo_toml bin_block）、`auto-term/at-app/pac.at:9` |
| 4 | 桌面 21 app 构成 | examples/ui 策展集 17（desktop_visible 恰等断言）+ auto-os 侧 sys-monitor/launcher/minesweeper/ui-gallery/widgets-gallery + manifest 各仓（kanban/musk/jade-garden/auto-term/os-config） | `app_registry.rs` `scan_examples_ui_curation_set`（want 恰 17）；`scripts/desktop.sh` EXTRA 注入链 |
| 5 | 展示名消费点全集 | 注册表条目 title → launcher 注入（renderer 8793）、窗表注入（8915）、LaunchSpec 填充（session.rs:2315）；VM 独立窗走 automan→AUTO_VM_TITLE | `automan.rs:350`、`main.rs:1020-1024`、`iced/renderer.rs:8793/8915/10821`、`session.rs:2315/4591` |
| 6 | locale 既有约定 | VM i18n 已用 `AUTO_LOCALE`（缺省 zh）装载期定 locale | `crates/auto-lang/src/ui/i18n_lookup.rs` 模块注释 |

### 4.3 相邻在途计划

- PLAN-012（executing，shell-ux-feedback-batch）：dock/launcher/任务栏消费面
  的渲染细节在改，但不动注册表字段与 title 语义——本计划解析链入口收口在
  entry/automan 层，与其改动画不相交；执行期若 012 尚在途，注入臂任务
  （T2）以 rebase 顺序协调。
- auto-lang 侧对应分账：执行时在 auto-lang 主检出登记同主题互链（其
  docs/plans 取号与否以 work 阶段实测为准——若仅 pac.at 批量 + 三文件小改，
  可依 auto-lang AGENTS 判定是否达开 plan 门槛；主导分账始终是本计划）。

## 5. 详细设计

### W1 注册表 schema 扩展（G1/G2）

`crates/auto-lang/src/ui/app_registry.rs`：

- `AppRegistryEntry` 新增 `pub title_zh: Option<String>`；
  `entry_for_dir` 从 `fields.get("title_zh")` 装配。
- 新增解析收口：

```rust
/// 展示名解析链（AUTO_LOCALE，缺省 zh）：zh = title_zh→title→name→目录名；
/// en = title→name→目录名。locale 只认 zh 前缀命中，其余一律 en 链。
pub fn display_title(&self) -> &str
```

- 兜底链保持现行为不变（`title.or(name).unwrap_or(目录名)`），`title_zh` 只是
  zh locale 下的头选——未声明时 zh/en 输出一致，**零配置零回归**。
- 单测：title_zh 解析、zh/en 两链、无 title_zh 回落、裸目录回退不变。

### W2 VM 独立窗标题 locale 臂（G2）

- `crates/auto-man/src/automan.rs:350` `pac_window_title` 保持不动（返回声明值）；
  新增 `pac_display_title(&self) -> Option<String>`：读 `AUTO_LOCALE`（缺省 zh），
  zh 且 pac 有 `title_zh` 则取之，否则 `title`。
- `crates/auto/src/main.rs:1020`：`AUTO_VM_TITLE` 注入改调 `pac_display_title`。
- automan 的 pac 读取是行级（与 parse_pac_exe_name 同族），`title_zh` 新键
  直读即可；注意行内 `#` 注释剥离语义对齐 `parse_pac_fields`。

### W3 注入/launch 臂换用 display_title（G2）

- `session.rs:2315` LaunchSpec.title 填充处、`iced/renderer.rs:8793`（launcher
  titles 列表）、`:8915`（窗表 titles）、`:10821`（dock 条目）——凡取
  `entry.title` 展示处改 `entry.display_title()`；`e.title.to_lowercase()`
  （8797，palette 过滤键）改为对 display 结果 lowercase，搜索中英两可。
- 范围纪律：只换**展示读点**；id 匹配、pinned 查表、name 键查找一律不动。

### W4 apps.manifest name 退役（G4）

- `app_registry.rs` `OsManifestApp` 删 `name` 字段（本就 dead_code）。
- 本仓 `apps.manifest` 四条目删 `"name"` 行；`AGENTS.md` §4 manifest 行样例
  同步去 `name` 键并补一句"展示名以 pac.at 为唯一事实源（PLAN-015）"。
- README Apps 表若引用 name 值则一并核（执行期确认，预判仅 id/ports）。

### W5 pac.at 批量补齐（G5，数据任务）

三组，全部只增 `title`/`title_zh` 两行（title 已正确的只补 zh），name 不动：

- **auto-lang examples/ui**（33 目录）：17 策展 + 画廊型非策展目录一并补齐
  （兜底链会吃到）；已知样例：012-stopwatch `title: "Clock"` 不改，补
  `title_zh: "时钟"`；011-calculator 补 `"计算器"`；026-database
  title=`Database Studio` → zh `"数据库工作室"` 等——执行期按目录逐个定名，
  命名风格：中文用简体、不带 app 后缀、与英文名语义对齐。
- **auto-os apps/**：025-sys-monitor（title 现"系统监视器"——中英对调：
  `title: "System Monitor"` + `title_zh: "系统监视器"`）、028-launcher
  （Launcher/启动器）、038-minesweeper（Minesweeper/扫雷）、ui-gallery
  （UI Gallery/UI 画廊）、widgets-gallery（Widgets Gallery/组件画廊）。
- **manifest 各仓**：apps/kanban（auto-kanban 仓，Kanban/看板）、auto-musk
  （`pac.at` 仓根，Auto Musk/Musk 音乐？——执行期按仓 README 定名）、
  jade-garden（`auto-down/jade-garden/front/auto/pac.at`，Jade Garden/玉圃——
  执行期与仓主确认）、auto-term（`app/pac.at`，AutoTerm/终端；`at-app/pac.at`
  的 `exe_name: "auto-term"` 原样保留作标准样板）。

### W6 契约成文（G1）

- `auto-lang/docs/specs/auto-man/project.md` 增补 **pac 四名称契约** 节：
  四字段语义表、locale 解析链、exe_name 字符白名单与告警忽略行为、
  "manifest 不承载展示名"边界。plan `new_spec_components` 预填该锚点，
  review 定稿。
- 本仓 AGENTS.md §4 的措辞同步（W4 内）。

### W7 测试对齐（G2/G5）

- `scan_examples_ui_finds_at_least_27_apps`（calculator title 断言）与
  `scan_examples_ui_curation_set`：补字段不动 `desktop_visible`，预期天然绿；
  执行期跑定点测试确认，红则按断言语义修数据（不得改 want 集语义）。
- `regenerate_code_only`/`generate_rust_ui` 既有 exe_name 单测（rust_ui.rs:3439
  邻域）保持绿；新增一例：`title_zh` 不参与 Cargo.toml 生成（名称与产物解耦的
  反向钉）。

### 规范增量

| delta_id | 类型 | 目标 | before/after | rationale | acceptance |
|---|---|---|---|---|---|
| SD-01 | add | auto-lang/docs/specs/auto-man/project.md §「pac.at 四名称契约（PLAN-015）」 | before：pac 名称字段无契约（name/title 语义未定版、exe_name 仅 Plan 014 局部注释）；after：四字段语义表 + locale 解析链 + exe_name 白名单成文 | 展示名/标识/产物名三概念解耦需成文契约 | AC-1, AC-3 |
| SD-02 | modify | auto-lang/docs/specs/auto-man/project.md §「pac.at 四名称契约（PLAN-015）」边界段（review 定稿：原"manifest 节"实际收编为同节边界段，不另立节） | before：apps.manifest 条目含 name（dead）；after：manifest 条目 = id/repo/kind/ports/status/daemon，无展示名 | 单一事实源 | AC-4 |

## 6. 测试设计

| 层 | 手段 | 断言 |
|---|---|---|
| 注册表解析 | cargo t（auto-lang，app_registry 单测） | title_zh 装配；display_title zh/en 两链；无 title_zh 回落 title；裸目录回退不变 |
| VM 标题 | cargo t（auto-man） | pac_display_title zh 取 title_zh / en 取 title / 缺省回落；AUTO_VM_TITLE 注入值跟随 |
| exe 契约 | cargo t（auto-man rust_ui） | 既有 exe_name 用例绿；title_zh 不进 Cargo.toml |
| 策展回归 | cargo t（scan_examples_ui_*） | ≥33 与 want 恰 17 两断言保持绿（字段增补零扰动） |
| 桌面实机 | headless 指针（PLAN-012 先例）+ 机会性实机截图 | zh 缺省下 launcher/dock/任务栏/窗标题显示中文；`AUTO_LOCALE=en` 显示英文 |

## 7. 验收标准

- **AC-1**：`AppRegistryEntry` 含 `title_zh`，`display_title()` 按 AUTO_LOCALE
  解析（zh：title_zh→title→name→目录名；en：title→name→目录名），单测绿。
  验证：`cargo t -p auto-lang app_registry`（auto-lang 侧 worktree）。
- **AC-2**：VM 独立窗 `AUTO_VM_TITLE` 取 `pac_display_title`，automan 单测绿。
  验证：`cargo t -p auto-man pac_display`。
- **AC-3**：pac 四名称契约在 auto-lang spec 成文；`exe_name` 行为与 Plan 014
  一致（白名单告警忽略、regen 即生效），既有单测绿。
- **AC-4**：`apps.manifest` 与 `OsManifestApp` 无 `name` 字段；AGENTS.md §4
  样例同步；manifest 解析单测绿。
- **AC-5**：桌面注册 app 的 pac.at 全部具备语义正确的 `title`+`title_zh`
  （examples/ui 33 + auto-os apps 5 + manifest 各仓 4）；name/exe_name 语义
  不变。验证：批量脚本核行 + 策展集测试绿。
- **AC-6**：实机桌面 zh 缺省显示中文名、`AUTO_LOCALE=en` 显示英文名；
  launcher palette 中英文均可搜到（headless 指针成文，机会性实机截图佐证）。

## 8. 执行步骤

| ID | 任务 | 依赖 | 落点 | 产出/验证 | AC |
|---|---|---|---|---|---|
| T1 | W1+W2：注册表 title_zh/display_title + automan pac_display_title + main.rs 注入臂换用，随带单测 | — | auto-lang crates 三文件 | `cargo t -p auto-lang app_registry`、`cargo t -p auto-man pac_display` 绿 | AC-1, AC-2 |
| T2 | W3：注入/launch 臂展示读点换 display_title（session/renderer 四处；与 PLAN-012 在途渲染改动 rebase 协调） | T1 | auto-lang session.rs/renderer.rs | 定点测试绿；palette 过滤中英两可 | AC-1, AC-6 |
| T3 | W4：OsManifestApp 删 name + apps.manifest 四行删 + AGENTS.md §4 同步 | — | auto-os + auto-lang 各一 | manifest 解析单测绿；`grep '"name"' apps.manifest` 空 | AC-4 |
| T4 | W5：pac.at 批量补齐（三组；执行期逐目录定名，中文命名风格见 W5） | T1（先有契约再填数据，顺序非硬依赖可并行） | auto-lang examples/ui、auto-os apps/、四 manifest 仓 | 批量核行脚本过；策展集测试绿 | AC-5 |
| T5 | W6：auto-lang spec 补 pac 四名称契约节 + plan new_spec_components 定稿 | T1 | auto-lang docs/specs/auto-man/project.md | spec diff 成文 | AC-3 |
| T6 | W7：测试对齐补齐（title_zh 反向钉用例等） | T1, T4 | auto-lang rust_ui.rs 测试区 | 全量定点 cargo t 绿 | AC-3, AC-5 |
| T7 | 桌面实机验收：zh/en 双 locale 桌面截图 + headless 指针成文 | T2, T4 | 本仓 docs/plans/evidence/ | AC-6 证据落盘 | AC-6 |

执行载体：按 Plan 529 布局开组 worktree
`D:/autostack/.wt/os-015/auto-os`（`-b plan-015-dev`）；auto-lang 侧改动在
auto-lang 仓对应分支/组内检出落地（Category A 门档：本计划改 crates，可在
auto-lang 侧跑定点 cargo t，仍禁全量 docs_gen）。

### 执行证据（2026-09-14，work 轮）

- **[x] T1** [✅ 已完成] pac.rs `title_zh` 字段/解析/构造 + `Pac::display_title_in`；
  automan `pac_display_title`（env 壳）；app_registry `title_zh` + `display_title(_in)`；
  main.rs `AUTO_VM_TITLE` 臂换用。auto-man 定点 4 用例绿（含 Plan-014 反向钉）。
- **[x] T2** [✅ 已完成] renderer 五处读点（launcher titles/lts、桌面格 label、
  LaunchSpec×2）换 `display_title`；session.rs 无需改（resolver 灌入即 display 值）。
  手段注记：计划原列 session.rs:2315，实际填充点全在 renderer resolver（evidence §T2）。
- **[x] T3** [✅ 已完成] `OsManifestApp` 删 `name`；apps.manifest 四行删；
  AGENTS.md §4 样例与"展示名唯一事实源"注记。manifest 解析单测随 app_registry 23 绿。
- **[x] T4** [✅ 已完成] 44 pac.at 双字段全补（examples/ui 33、auto-os 5、
  kanban 1、musk 1、term 2、os-config 1、jade-garden 1——jade-garden 原缺
  title，补 `Jade Garden`/`玉圃`；musk 中文名暂保留品牌 `Auto Musk`，Q2 可改）。
- **[x] T5** [✅ 已完成] auto-lang `docs/specs/auto-man/project.md` 增
  "pac.at 四名称契约"节（SD-01/SD-02 定稿锚）。
- **[x] T6** [✅ 已完成] 反向钉用例（title_zh 不进 Cargo.toml）绿；
  **策展集既有红修齐**：base 即红（9c6c27e86 漏更 want），按双向语义补
  031-image-viewer 入 want（16→17），断言语义未放宽。app_registry 模块 23/23 绿。
- **[x] T7** [✅ 已完成] VM 轨三 locale 实机（缺省=编辑器 / en=AutoEdit /
  zh_CN=编辑器），证据 `evidence/plan015-vm-title-locales.md`；手段调整：
  整桌面截图以 headless 等价成文（PLAN-012 先例），视觉复核留 review 实机门。

## 9. 复审记录

- 2026-09-14 /auto-plan:new 起草（stage: new，PLAN-015 rev1）：
  调研结论六条（§4.2）全部锚到代码行号；四字段契约、locale 解析链、
  manifest name 退役三决定经用户认可（"OK"，2026-09-14）。outcome: pass，
  next: work（T1 可即刻开工）。jade-garden 中文名与 auto-musk 展示名两处
  执行期定名项见 §10，不阻塞 T1–T3。
- 2026-09-14 /auto-plan:work（stage: work | plan_id: PLAN-015 | rev1 |
  outcome: pass | next: review）。组 worktree：`.wt/os-015/{auto-os,auto-lang,
  auto-kanban,auto-musk,auto-term,auto-os-config,auto-down}`（分支 os-015-dev，
  auto-down detach@67bb508）。T1–T7 全落，各仓已 commit（commit hash 见各仓
  os-015-dev 分支头）；定点验证全绿（证据 `evidence/plan015-vm-title-locales.md`）。
  发现并修齐 base 既有红：策展集 want 漏更 031-image-viewer（9c6c27e86，
  语义未放宽）。AC-6 以 headless 等价成文（VM 窗标题三 locale 实机 +
  注入链单测），整桌面视觉截图留 review 实机门。blockers: 无。
- 2026-09-14 /auto-plan:review（stage: review | plan_id: PLAN-015 | rev1 |
  outcome: **pass** | next: merge）。
  **独立性声明**：复审与实施同会话，结论全部从工件独立重建（重跑测试/
  重做实机/重读 diff），不采信实施轮日志。
  **reviewed_commit**：auto-lang `d0baea6be`、auto-os `74de56e`、
  auto-kanban `fc0434f`、auto-musk `0a8d5ac`、auto-term `1a7ac1e`、
  auto-os-config `e924fff`、auto-down `3320014`（七检出全 clean）。
  **base_commit**：auto-lang master `71ed7ea90`、auto-os main `d426fe6`。
  **AC 判定**：AC-1 pass（app_registry 模块 23/23 + display_title 两链单测）；
  AC-2 pass（automan 单测 + `AUTO_VM_TITLE` 臂换用）；AC-3 pass（spec 契约节
  在 commit 内 + 反向钉用例绿 + exe_name 行为不变）；AC-4 pass（OsManifestApp
  无 name + manifest 四行删 + AGENTS.md 同步 + 解析单测绿）；AC-5 pass
  （review 侧独立重计：44 pac.at title/title_zh 双全，HEAD~1 diff 口径）；
  AC-6 pass（headless 等价档：复审独立复现四 locale VM 实机——缺省=编辑器/
  en=AutoEdit/zh_TW=编辑器/fr=AutoEdit；整桌面视觉截图遗留 merge 后机会性
  补采，不阻塞）。
  **全量门（nextest 串行）**：`cargo tv` 3691/3691 绿；`cargo t` 快档 4857
  例 11 失败=10 `ui::layout::tests`（master 双侧复现，base 既有）+1
  `plan492_m4 c2_param_msg_declaration_both_tracks_alive`（master 复现，base
  既有）——零新增红；ui-iced 档 4844 例 8 失败全 layout 家族（base 既有）；
  auto crate 11/11；auto-man 287/287。
  **findings**：F1(info) 策展集 base 既有红修齐已核（want 16→17，语义未
  放宽）；F2(info) 复审自身两次并行 cargo 执行造成假红（plan609/vshow env
  竞态 + auto.exe 文件锁）与 VM 实验残留僵尸 auto.exe——串行重跑后全部
  消除，教训：本仓门禁必须 nextest 串行单链；F3(info) layout 家族 + m4
  base 既有红非本计划范围，建议另开 debt 计划，不阻塞本计划。
  **spec_inputs**：`auto-lang/docs/specs/auto-man/project.md` @ d0baea6be
  （§「pac.at 四名称契约（PLAN-015）」23 行，锚定 SD-01/SD-02；描述持久
  行为，无执行日记）；`new_spec_components` 已定稿为精确路径。
- 2026-09-14 /auto-plan:merge（**PLAN-015:r1**，收据键 PLAN-015:r1，
  outcome: **blocked**——仅 auto-lang landing 一项待并行会话让位，余六仓
  checkpoint 全落）。
  - `prepared` ✅：七仓 reviewed 基线核对；auto-os spec 面随 auto-lang
    commit 承载（SD-01/SD-02 唯一 canonical 目标 =
    auto-lang/docs/specs/auto-man/project.md）；交付提交 = 各仓 os-015-dev
    分支头。
  - `landed` ✅（6/7）：auto-os main `2152dc1`（merge 提交，含
    apps 5 pac.at + apps.manifest + AGENTS.md）；auto-kanban main
    （title_zh 看板）；auto-term main（app+at-app 两文件）；auto-down
    master（jade-garden title/title_zh）；auto-musk main、auto-os-config
    main（各自 merge 提交，真合并零冲突）。
  - `landed` ⏳（1/7）：**auto-lang master**——主检出被并行会话活跃占用
    （iced layout/renderer/terminal 域 5→9 文件在途脏区，13:40–14:06 持续
    演进，master 期间三度推进 0a3c9b26b→e0c404f57→0736b58e7），merge 会
    覆写其在途文件，三次有界重试（含 5+8 分钟等待）均不满足干净前提，到
    自动重试上限。**分支已保持 landing-ready**：os-015-dev `cdc3ed46d`
    已三轮并入最新 master（重叠文件 i18n_lookup.rs/renderer.rs 均自动
    合并且双侧改动存活验证：locale_prefers_zh+TABLES 共存、5×display_title
    +5×title_zh:None 全在），合并提交定点绿（title_zh 3 + display 1 +
    i18n F-02 5 + app_registry 64 + vshow 4）。
  - **精确解锁动作**：auto-lang 主检出 `git status` 干净后跑
    `git merge os-015-dev`（预期 ff；若 master 又进，先在
    `.wt/os-015/auto-lang` 重并 master 再落）；落地后按序完成
    `ledger_refreshed` → `archived` → `cleaned`（七 worktree + 分支 +
    组目录，wt-guard 前置）。
  - `ledger_refreshed` ⏳ / `archived` ⏳ / `cleaned` ⏳：依 auto-lang
    landing 解锁（canonical spec 在该提交内，台账/归档不得先于 canonical
    落地发布）。worktree 全部保留不清理。

## 10. 待澄清事项

| # | 事项 | 影响 | 处置 |
|---|---|---|---|
| Q1 | jade-garden 中文名（"玉圃"为预设） | T4 一行数据 | **work 轮已落预设值**（可随时改 pac.at 一行） |
| Q2 | auto-musk 英文展示名定型（`Auto Musk` vs `Musk`） | T4 一行数据 | **work 轮裁定**：英文 `Auto Musk`、中文暂保留品牌 `Auto Musk`（品牌不译，RealWorld 同款先例）；要改仅需 pac.at 一行 |
| Q3 | auto-lang 侧是否独立开 plan 分账 | 流程账 | **work 轮判定**：改动面（4 crates 文件 + examples 数据 + spec 一节）随本计划组内 os-015-dev 分支落地，auto-lang 侧未单开 plan；merge 时按其 AGENTS 归账 |
