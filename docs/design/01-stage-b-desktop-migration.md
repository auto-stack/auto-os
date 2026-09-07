# Design 01 — Stage B 桌面域搬迁定案（auto-lang → auto-os）

> 元信息：定案 2026-09-07 · 来源 Plan 584（auto-lang）· 前序 Plan 579
> （Stage A 两轴分离：语言/框架轴 = auto-lang，产品/应用轴 = auto-os 伞形根）、
> Plan 583（执行期清障发现）· 注册于 [00-intro.md](00-intro.md)。
> **修订 2026-09-07（用户二次裁定）**：shell 四件由「留架 + Stage C 候选」
> 翻转为「随迁」——新增 §4-P7 路径化批次，§1/§2/§6/§7 同步修订。
> 路径约定：本文中 `框架仓` = auto-lang 主检出（解析序末位
> `D:/autostack/auto-lang`）；`本仓` = auto-os。

## §0 定调与范围

- **两轴分离**（Plan 579 Stage A 裁定）：语言/框架轴 = auto-lang（VM/编译器/
  AutoUI 宿主运行时，单一真相源）；产品/应用轴 = auto-os 伞形根（桌面程序
  治理、shell 表面、应用轨道、app 仓策展）。
- **Stage B = 桌面域资产归位本仓**：治理资产（台账/计划）、shell 四件
  （§1-A4，经 §4-P7 路径化）、launcher 与应用轨道 apps、apps.manifest 对接。
- **总原则**：框架单一真相源留 auto-lang；跨仓依赖一律走解析序
  （env → 兄弟检出 → 主检出），**零 junction/symlink**（2026-09-03 三仓事故
  红线沿袭）；归档/历史文档不回改，以指针与跨仓引用衔接。
- **窗口约束**：唯一双写冲突面 = auto-lang 侧 541/582 两个在飞 worktree
  （详见 §5）——P-5 本体批必须等两者合并；其余批次无硬窗口。
- **实施载体**：§7 拆解表 P-1..P-7，每批一 plan 一 worktree；本设计即各批
  计划的取材源与验收基线。

## §1 资产清单（随迁）

三要素落表：源（auto-lang）/ 目标（auto-os）/ 框架依赖面。

**裁定链（两次，最终=随迁）**：Plan 584 草案曾把 shell 四件
`crates/auto-lang/assets/{shell,desktop,switcher,notification_center}.at` 列为
核心随迁资产；执行期实测发现它们被框架 **`include_str!` 编译期内嵌**
（`crates/auto-lang/src/ui/shell.rs:10/21/32/45` 四个 const），且框架代码与其
内容共演化（任务栏行高 56px 对齐、`verb\u{1F}arg` 记录编码、dock hover 语义
等源自 shell.at 的契约），遂先裁定「留架 + 路径化列 Stage C 候选」。
**用户二次裁定（2026-09-07）翻转：shell 是桌面核心组成，应当归位本仓**——
且引用面实测远小于预估（四 const 在 shell.rs 之外仅 **7 处**引用：
iced/renderer.rs ×6 + ui_gen/vue.rs ×1；`DesktopOptions` 已有 `apps_dir:
Option<PathBuf>` 先例，renderer.rs:10948）。**终局裁定：shell 四件随迁
（§1-A4），经 §4-P7「shell pack 路径化」解焊后落位本仓 `shell/`；框架侧
include_str! 内嵌副本转为 pin 快照回退（hash-lock 同步契约）。**路径化与
Design 23 R1「宿主=内核（框架）/shell=用户态（产品）」的架构本意一致。

### A. 桌面程序治理资产

| # | 资产 | 源（auto-lang） | 目标（auto-os） | 框架依赖面 |
|---|---|---|---|---|
| A1 | 桌面程序台账（活账：依赖图/入口仪表盘/裁定登记簿） | `docs/plans/autos-desktop-program.md` | `docs/plans/autos-desktop-program.md` | 无代码耦合；架构依据以跨仓路径引用 Design 23/24/25（留架，见 §2） |
| A2 | 桌面域在途计划 ×7（详 §5） | `docs/plans/{535,554,556,557,558,577,578}-*.md` | `docs/plans/` 重编 `os-NNN` + `origin: PLAN-5xx` 注记 | 无 |
| A3 | launcher 桌面（注册表型特权 app，Plan 464 交付） | `examples/ui/028-launcher/` | `apps/028-launcher/` | pac.at 注册表扫描（`--apps-dir`/`AUTO_DESKTOP_APPS`）、`desktop.*` builtin、投影协议 `__wm_*`、`LaunchApp` 接缝 |
| A4 | shell 四件（桌面核心表面：任务栏/dock、桌面本体、switcher、通知中心） | `crates/auto-lang/assets/{shell,desktop,switcher,notification_center}.at` | `shell/`（同名四件） | `ui/shell.rs` 加载器（§4-P7 路径化改造后消费）、投影协议 `__wm_*`、renderer/session/layout 侧记录编码契约（经 hash-lock pin 快照对齐） |

批次注记：A2 随 P-1「计划随迁批」先行（纯文件、零冲突面）；A1/A3 随 P-5
「本体批」——541/582 在 auto-lang 收口期间其 finish-plan 步骤仍要更新台账，
台账迁移必须等两者合并（§5 窗口约束）；A4 随 P-7「shell pack 批」——先落
框架加载器（§4-P7），再物理迁移四件并翻转权威。

### B. 应用轨道 apps（examples/ui 0xx 中有桌面域计划承载者）

原则：**应用轨道 = 产品轴**（GOAL-010「应用轨道出 examples——仓外首例」的
延伸）——有桌面域计划承载的 app 随迁；纯教学/演示 demo（001-helloworld、
002-counter、005-login 等）留架为框架示例（§2）。枚举边界以各随迁计划在
auto-os 开工时实测为准。

| # | app | 源（auto-lang） | 承载计划（随迁后执行） |
|---|---|---|---|
| B1 | 025 系（dashboard → sys-monitor） | `examples/ui/025-dashboard/`（541 合并后的形态） | 541 本仓收口 → 产出随迁 |
| B2 | minesweeper（游戏波基底） | `examples/ui/038-minesweeper/` | 556 games-wave1 |
| B3 | clock / tetris / klondike | 554/557/558 开工新落（现仅在计划中，无目录） | 554/557/558 |
| B4 | gallery 系 | `examples/ui/029-photo-gallery/` 等，集合以 578 定案为准 | 578 desktop-gallery-apps |
| B5 | notes explorer | ~~582 产出（examples/ui/ 下新目录）~~ **实测修正（P-5 立项 2026-09-07）：582 产出=website playground（packages/auto-playground-vue），非 examples/ui 桌面资产——无实物随迁** | 582 本仓收口（产出留 website 轴） |

公共资产：`examples/ui/common/` 中被随迁 app 引用者随迁（抽 `apps/common/`）；
`p493-color-check`、`041-auto-edit`、`043/044`（capability/dnd/clipboard 桥）
等无桌面域计划者留架。端口带：in-repo apps 沿 examples 的 30NN/80NN 带；未来
升格独立仓时改 17xxx 带（沿 auto-kanban 先例，本仓 AGENTS §3）。

### C. apps.manifest 对接面（清障二的消费端）

本仓 `apps.manifest`（伞形清单；现有 kanban 一例：repo 形态 17100/17101）在
Stage B 后聚合三类条目：① **repo 形态**（独立 app 仓，如 auto-kanban——
remote 窗或 extra root 注册）；② **local 形态**（本仓 `apps/` 目录，注册表
扫描）；③ **框架 demo**（auto-lang `examples/ui` 默认注册表——经清障二接入
聚合）。manifest 条目 schema 扩展由清障二计划（§4-P3）定稿。

**Schema 定稿（2026-09-07，PLAN-586 执行）**：条目 `{ id, name?, repo?,
kind: "repo"|"local"（缺省 repo）, ports?, status?（缺省 active）, added? }`
——宽容读取（未知字段忽略）；repo 形态 `repo` 相对 manifest 根解析、目标含
pac.at 才注册；local 形态由 `apps/` 容器展开覆盖（manifest 仅策展元数据）；
坏条目跳过 + 警告不阻断启动。**注册形态执行期修正**：repo 条目落
**extra root 原生挂载**（§1-C 候选①的 extra root 臂）——remote-apps 机制
（Plan 516 G4）实测为 WS 投影协议端点，装不下 http/原生 app 形态；纯 web
app iframe 嵌入列 Stage C 候选（依据与验证见 auto-lang PLAN-586 复审记录）。


## §2 留架清单（框架层不动）

原则：**桌面宿主运行时（WM/虚拟窗/合成/会话/投影/渲染）是框架能力，不是产品
资产**——462–479 落地时即以框架特性形态入库，3000+ 测试锚定；框架轴单一真相源
= auto-lang（Stage A 裁定）。Stage B 不动下列资产。

| # | 资产组 | 位置（auto-lang） | 留架理由 | 与桌面域的耦合点 |
|---|---|---|---|---|
| L1 | 桌面宿主运行时 | `crates/auto-lang/src/ui/`（iced/session/shell.rs/layout/vm_bridge/aura_view_builder 等） | WM/合成/会话/投影协议实现方，全量测试锚定 | `include_str!` 内嵌 shell 四件（`ui/shell.rs:10/21/32/45`）——§1 修正裁定的根因 |
| L2 | shell 四件 .at 的**框架内副本** | `crates/auto-lang/assets/{shell,desktop,switcher,notification_center}.at` | P-7 权威翻转后转为 **pin 快照回退**（权威副本随迁本仓 `shell/`，§1-A4；hash-lock 同步契约防双源漂移） | `include_str!` 编译期内嵌（`ui/shell.rs:10/21/32/45`）——外部引用仅 7 处（§4-P7 改造面）。**P-7 已落地（2026-09-07）**：四件已入本仓 `shell/`（内容与 pin 快照全等），`scripts/shell-pack-sync.py` 契约在案（校验红灯/`--sync` 单向同步双模式） |
| L3 | 语言核心与工具链 | `crates/` 全 workspace（auto-lang 九模块/auto-val/auto-man/auto-cli/auto-gen/auto-lsp/auto-cache…） | 语言/框架轴本体 | auto-man `rust_ui.rs` rust-server 产物链 = 清障一对象（§4） |
| L4 | auto-cosmic（Smithay 宿主线） | `auto-cosmic/`（host-smithay crate，509 线） | 框架侧宿主技术实验（Linux 向合成宿主） | 未来合成宿主演进线；归属可随 Stage C 重估 |
| L5 | Web/UI 生态包 | `packages/`（JS 四包）、`blocks/`、`website/`、`parity/`、`stdlib`（auto/lib） | 框架 UI 生态与语言运行库 | packages 被 app 轨道消费（vue 轨）——经版本引用，不随迁 |
| L6 | schema 合同 | `schema/aura.at`、`schema/projection-protocol-v1.md` | 协议合同由实现方持有（aura 组件声明/投影协议 v1 均框架实现） | 本仓设计文档以跨仓路径引用 |
| L7 | examples 非应用轨道 | `examples/`（godot/http/charts gallery/教学 demo/capability-tests/041-auto-edit 等） | 框架示例与能力测试 | examples/ui 教学 demo 留作默认注册表演示集（§1-B 原则） |
| L8 | 桌面程序历史设计文档 | `docs/design/autoui/{virtual-desktop(23), desktop-shell-and-launcher(24), desktop-shell(25), desktop-protocol-v1, examples-app-track(21)}.md` | 成文地与 specs 沉淀地；归档/历史不回改 | **新增**桌面程序设计一律落本仓 `docs/design/`（本 01 文档即首例）；本仓 00-intro 已注引用方式 |
| L9 | specs ledger 与计划体系 | `docs/specs/`、`docs/plans/`（INDEX/.next-id） | auto-lang 治理体系 | 台账 A1 迁移后在 auto-lang `docs/plans/` INDEX 留指针行（§5 迁移机制） |

规模注记：草案所称「框架层约 8 万行」以 §7 P-5 执行时实测（`tokei`/`cloc`）
归档为准，不作为本设计硬数字。


## §3 入口委托方式（ADR）

### 现状事实（2026-09-07 实测）

桌面**没有**统一 `auto desktop` 子命令，实为两条入口：

1. **iced/VM 轨**（验收宿主示例）：
   `cargo run -p auto-lang --features ui-iced --example ui_desktop [-- --fullscreen --apps-dir <path>]`
   （`crates/auto-lang/examples/ui_desktop.rs`；默认注册表目录 = 编译期
   `CARGO_MANIFEST_DIR` 锚定框架仓 `examples/ui`，`ui_desktop.rs:17-24`）。
2. **Vue 轨**：`auto run --desktop`（`crates/auto-man/src/vue.rs` 注入
   `AUTO_DESKTOP=1`；注册表目录经 `AUTO_DESKTOP_APPS` env 覆盖 / 默认
   `<root>/examples/ui`，`vue.rs:5480` `desktop_apps_dir`；extra 单 app 根经
   `AUTO_DESKTOP_APPS_EXTRA` 路径表 / 默认兄弟探测 `../auto-os-config/auto`
   ——Plan 559 W3 `desktop_extra_app_roots`；VM 轨 parity：
   `app_registry::extra_roots_from`）。

### 选项空间

- **a) 本仓薄包装**：auto-os 提供 `scripts/desktop.ps1`/`desktop.sh`（未来升格
  os CLI 子命令），按解析序定位 auto-lang，注入 `AUTO_DESKTOP_APPS`（本仓
  `apps/`）+ `AUTO_DESKTOP_APPS_EXTRA`（manifest 聚合根），再调上述既有两条
  入口。框架零新增改动（§4 清障除外）。
- **b) 框架一等子命令**：auto-lang CLI 新增 `auto desktop`，内部聚合
  manifest/env/默认注册表。体验最好，但框架改动面大（CLI + 双轨 + 测试），且把
  产品侧决策（manifest 聚合规则）倒灌回框架仓。
- **c) 双入口并存**：a 先行 + b 后补。

### 定案

**Stage B = 选项 a**；b 列 Stage C 候选（搬迁稳定、入口语义收敛后评估）；c 即
a→b 自然演进路径，不单列计划。

ADR 理由：① 改动面最小——Stage B 框架改动已锁定为 §4 两项清障，不扩面；
② Plan 579 已验证同形态路径（env 注入 + 伞形 manifest + vm 模式跳过）；
③ 本仓 AGENTS 解析序（env → sibling → 主检出）原生支持包装脚本定位框架；
④ 兼容性：028-launcher 及 examples 教学 demo 的既有启动方式零变化（a 只叠加
env，不改任何现有入口语义与默认值）。

（对 Plan 584 待澄清 #2 的修正收口：草案初值倾向「b：auto-lang 保留
`auto desktop` 入口」——实测该子命令不存在，"保留"对象落空；选项重定义后
a 仍是最小改动方，与原倾向「框架少动」的精神一致。）


## §4 技术清障与适配方案（三项）

P2/P3 为 Stage B **硬前置**（框架仓改动，Category B 作用域），P7 为二次裁定
提级的适配批次——三者各立小 plan；P2/P3/P7 均不依赖搬迁时机，P7 依赖 P-3
（同族解析序机制先行）。R-D1 状态修正（较 Plan 584 草案）：583 合并提交
`2856158e7` 已载明「R-D1 顺偿：rust-server 编译过 53.2s + API 200，E0432
消失」——**功能正确性已证**；Stage B 清障点因此收窄为**落点归属**，非功能修复。

### P2 清障一：rust-server 产物落点可配

- **现状锚点**：`crates/auto-man/src/rust_ui.rs:1979`
  `pub fn ensure_shared_workspace(project_dir)`——内部经 `get_rust_workspace_dir()`
  固定解析到框架仓 `examples/rust-workspace/`，扫描既有 member、重写 workspace
  `Cargo.toml`（`compute_auto_lang_rel_path` 注入框架路径依赖）；调用点：
  `rust_ui.rs:354`（generate_rust_ui）+ `api_gen.rs:636/752/754/3179`。仓外
  项目走 rust 后端时，产物 member 仍写进框架仓（579 执行期 kanban-back 两度
  生成/清理的根因；本仓工作区 `examples/rust-workspace/auto-musk-back` 的在飞
  改动即同链路产物）。
- **方案**：落点按解析序解析——
  `AUTO_RUST_WORKSPACE`（新 env）→ **项目仓内** `<project_dir>/rust-workspace/`
  （project_dir 即 pac.at 所在仓根）→ 框架仓 `examples/rust-workspace/`（默认，
  向后兼容）。`get_rust_workspace_dir`/`ensure_shared_workspace` 依此改造；
  `compute_auto_lang_rel_path` 对任意落点重算框架相对路径（该函数已存在，正是
  为跨仓相对定位而生）。框架仓内既有项目（auto-musk-back 等）行为零变化。
- **验收**：在 auto-os（或任一仓外 app）跑 rust 后端，member 目录落项目仓内，
  auto-lang `git status` 干净；框架仓内既有 rust-workspace 项目回归不变。
- **门档**：Category B——`cargo check -p auto-man` + rust_ui/api_gen 模块测试；
  折叠前 `cargo tf`。

> **P-2 落地注记（2026-09-07，auto-lang PLAN-587 已执行复审）**：
> `resolve_rust_workspace_dir(project_dir)` 单点解析——`AUTO_RUST_WORKSPACE`
> env（设置即权威）→ 框架内项目=共享工作区（canonicalize+小写归一前缀检测，
> 零变化）→ 仓外项目=`<project>/rust-workspace/`；rel 锚点
> （auto-lang path 依赖/target-dir）从实际落点重算。单一漏斗
> `ensure_shared_workspace` + 三处直调点（start_api_server/build/run）全走
> resolve。验证：仓外 helloworld fixture 完整生成链 member 落 project-local +
> 框架共享工作区 Cargo.toml 字节不变（V4）；框架内 015-notes 既有生成测试
> 零回归；tf 唯红=charts 预存。KNOWN-DEBT P584-D1 结案。

### P3 清障二：桌面注册表指向仓外（三源聚合）

- **现状锚点**：① iced 轨 `crates/auto-lang/examples/ui_desktop.rs:17-37`
  ——默认注册表目录编译期锚定框架仓 `examples/ui`（`--apps-dir` 可覆盖）；
  ② Vue 轨 `crates/auto-man/src/vue.rs:5480` `desktop_apps_dir`
  （`AUTO_DESKTOP_APPS` env 覆盖 / 默认 `<root>/examples/ui`）+ `:5512`
  `desktop_extra_app_roots`（`AUTO_DESKTOP_APPS_EXTRA` 路径表 / 默认兄弟探测
  `../auto-os-config/auto`——Plan 559 W3）；③ VM 轨 parity
  `app_registry::extra_roots_from`；④ 本仓 `apps.manifest` 已有 repo 形态条目
  （kanban，17100/17101）；⑤ 远程窗机制 `remote-apps.json`（Plan 516 G4）已在。
- **方案**（复用既有机制，零新概念）：
  1. **extra roots 兄弟探测泛化**：默认探测列表由 `[../auto-os-config/auto]`
     扩为 `[../auto-os-config/auto, ../auto-os/apps]`（apps/ 每个含 pac.at 的
     子目录 = 一个 local app root；缺失兄弟静默跳过——solo 检出不炸，既有语义）。
  2. **apps.manifest 聚合**：按解析序读本仓 `apps.manifest`，repo 形态条目经
     既有 remote/URL 机制注册为远程窗（端口已在 manifest；kanban 即首例）。
     manifest 条目 schema 扩展（local 形态、apps/ 目录）在本计划定稿。
  3. **三轨 parity**：vue（vue.rs）/ vm（app_registry）/ iced（DesktopOptions
     传参链）注册表语义同源同步，缺一即红。
- **验收**：经 §3-a 包装脚本从本仓启动双轨桌面，注册表呈现三类条目——框架
  demo（默认注册表）+ 本仓 `apps/` + kanban（repo 形态远程窗）；在框架仓内
  原样启动零回归（028-launcher/教学 demo 启动方式不变）。
- **第一验收用例**：auto-kanban（583 已有 vm 模式 586 卡对账基线）。
- **门档**：Category B——涉 auto-man + auto-lang（app_registry/DesktopOptions），
  `cargo check -p auto-man -p auto-lang` + 模块测试；折叠前 `cargo tf`。

> **P-3 落地注记（2026-09-07，auto-lang PLAN-586 已执行复审）**：三步全落
> （extra roots 容器泛化 / manifest 框架侧直读〔用户裁定；`AUTO_OS_ROOT`
> env 设置即权威不回落，兼关断开关〕/ 三轨 parity 锚 + iced
> `DesktopOptions.extra_app_roots`）。**执行期修正**：方案 2 的 repo 条目
> 由「remote/URL 机制注册为远程窗」修正为 **extra root 原生挂载**——
> remote-apps.json（Plan 516 G4）实测为 WS 投影协议端点（连另一桌面实例
> 投影面），http/原生 app 形态装不进；repo 仓本身即 pac.at+src/front/
> app.at 单 app 根（os-config 先例同型）。V5 的「kanban 远程窗」措辞随修
> 正为「kanban 原生挂载呈现」；交互级启动验收归 P-5 V1/V2。§3-a 包装脚本
> 随 P-6 落地（用户裁定；P-3 以 env 注入等价形态验证）。

### P7 shell pack 路径化与权威翻转（二次裁定提级，原 Stage C 候选）

- **现状锚点**：`crates/auto-lang/src/ui/shell.rs:10/21/32/45` 四个
  `include_str!` const（SHELL_AT/DESKTOP_AT/SWITCHER_AT/NOTIFICATION_CENTER_AT）；
  shell.rs 之外引用仅 **7 处**（`ui/iced/renderer.rs` ×6 + `ui_gen/vue.rs` ×1，
  2026-09-07 实测）；`DesktopOptions`（renderer.rs:10948）已有 `apps_dir:
  Option<PathBuf>` 同型先例。
- **方案**（三步）：
  1. **加载器**：shell.rs 四 const 改为运行时解析——`AUTO_SHELL_PACK`（env，
     目录含四件同名 .at）→ 兄弟仓 `../auto-os/shell/` → **内嵌 const 回退**
     （include_str! 保留）。7 处外部引用改走加载器；`DesktopOptions` 增
     `shell_pack: Option<PathBuf>`（沿 apps_dir 先例）供宿主显式传入。无 pack
     时行为与现状**逐字节一致**（回退零变化，既有 3000+ 测试零重锚）。
  2. **物理迁移 + 权威翻转**：四件 git 迁入本仓 `shell/`；加载器兄弟探测即命中
     （本仓与 auto-lang 为 D:/autostack 兄弟检出）。auto-lang `assets/` 内嵌
     副本降级为 **pin 快照**。
  3. **hash-lock 同步契约**：本仓 `shell/` 为唯一真相源；auto-lang 快照经显式
     同步步骤更新（脚本比对四件 hash，方向单向 auto-os → auto-lang，同步提交
     注记 pin 版本）。防止双源漂移；契约测试锁「回退快照可编译可挂载」。
- **验收**：①无 pack 环境实机桌面五面交互与现状一致（V1）；②`AUTO_SHELL_PACK`
  或兄弟命中本仓 `shell/` 时加载成功且实机不回归；③hash-lock：两仓四件 hash
  一致或差异仅存在于显式 pin 提交之后、同步脚本红灯提示；④`cargo t` 桌面/ui
  模块零红（回退路径全量保真）。
- **门档**：Category B——`cargo check -p auto-lang` + ui 模块测试；折叠前
  `cargo tf`。
- **排序**：P-3 之后（同族解析序改造先立先复用）；与 P-5 并行无冲突（shell
  四件与 541/582 改动面零交集，无窗口硬约束；建议先于 P-5 完成以让本体批
  一次收拢）。


## §5 在途计划处置裁定

**裁定人：用户（zhaopuming），2026-09-07。原文逐字入档：**

> 我建议把已经动工的两个（541和另一个）完成，其他还没开始的，可以整个计划文件
> 一起搬迁过去，搬迁完毕后再在auto-os仓库执行。

（「另一个」经实测指认 = **582**：与 541 同为仅有的两个已动工（executing 起）且
worktree 在飞的计划——`git worktree list`：`lang-541`、`lang-582`。T6 复测时
582 已被并行会话推进至 `reviewed`——处置不变，收口语义收窄为 merge+archive。）

### 处置表（T6 复测 2026-09-07：frontmatter 状态 + worktree）

| 计划（auto-lang） | 域 | 状态 | worktree | 处置 |
|---|---|---|---|---|
| 541-025-sys-monitor | 桌面 | executing | lang-541 在飞 | **本仓收口**（走完 review/merge/archive 终态，不随迁；产出 B1 随本体批资产随迁） |
| 582-playground-notes-explorer | 桌面/examples | **reviewed**（起草时 executing，并行推进） | lang-582 在飞 | **本仓收口**（仅余 merge+archive；产出 B5 随迁） |
| 535-desktop-ux-followups | 桌面 | drafting | 无 | **随迁 → os-002**（P-1 已执行 2026-09-07，PLAN-001） |
| 554-clock-app | 桌面 | drafting | 无 | **随迁 → os-003**（P-1 已执行 2026-09-07） |
| 556-games-wave1 | 桌面 | drafting | 无 | **随迁 → os-004**（P-1 已执行 2026-09-07） |
| 557-tetris | 桌面 | drafting | 无 | **随迁 → os-005**（P-1 已执行 2026-09-07） |
| 558-klondike | 桌面 | drafting | 无 | **随迁 → os-006**（P-1 已执行 2026-09-07） |
| 577-p534-debt-batch-1 | 桌面债批 | drafting | 无 | **随迁 → os-007**（P-1 已执行 2026-09-07；编号消歧：与 auto-lang archive 既有 577-emitter-gaps-batch 无关） |
| 578-desktop-gallery-apps | 桌面 | drafting | 无 | **随迁 → os-008**（P-1 已执行 2026-09-07） |
| 545-use-namespace-semantics | 语言域 | drafting | 无 | 留守 auto-lang |
| 570-py-subclass-factory | 语言域 | drafting | 无 | 留守 auto-lang |

推论（搬迁窗口定义）：**唯一双写冲突面 = 541/582 两个在飞 worktree**——P-5
资产搬迁执行批必须等两者合并；P-1 计划随迁批纯文件、零冲突，设计定案后即可行。

### 迁移机制

1. **批次**：P-1 = A2 七项计划文件（先行）；P-5 = A1 台账 + A3 launcher + B 批
   apps（本体批，窗口内）。
2. **编号映射**（Plan 584 待澄清 #1 默认裁定）：auto-os 侧 `scripts/new-plan.sh`
   重编 `os-NNN`，frontmatter 增 `origin: PLAN-5xx` 溯源注记；auto-lang
   `docs/plans/` INDEX 与本表留去向一行；两仓正文互链。
3. **状态保持**：随迁计划以 drafting 原状迁移（状态机续用不重置）；其
   needs-analysis 中的 auto-lang 相对路径，在 auto-os 开工时按解析序换算。
4. **台账接棒**：A1 台账迁入后，桌面域计划状态变更只记 auto-os 侧台账；
   auto-lang INDEX 留指针行（防腐：单一事实源）。

**P-1 执行注记（2026-09-07，PLAN-001）**：七项已随迁为 `docs/plans/` os-002..
os-008（drafting 原状，frontmatter 带 `origin`，正文 auto-lang 相对路径未回改、
开工时换算）；auto-lang 侧 `docs/plans/INDEX.md` 指针行在案，七源文件已
`git rm`（git 历史留档）。批次计划 os-001 已执行完毕。


## §6 验证矩阵

| # | 维度 | 基线/方法 | 通过标准 | 适用批次 |
|---|---|---|---|---|
| V1 | 实机桌面交互（Windows 首选） | `ui_desktop --fullscreen` 五面交互（dock/任务栏/switcher/pager/通知中心/launcher 启动），与 462–479 实机验收同型 | 与搬迁前行为一致，无回归 | P-5 |
| V2 | I2 desktop_mcp 五套基线 | 每 app `tests/desktop_mcp.py`（在装 12 app；I2 五套锚数 14/11/11/19/26——472/478/479 台账），经 autoui-verifier `test_vm_mcp.py` 执行 | 搬迁前后同数**零漂移** | P-5 |
| V3 | 双端一致性 | autoui-verifier（`test_vue_playwright.mjs` + `test_vm_mcp.py`）抽查随迁 app（至少 launcher + minesweeper） | vue/vm 双端一致 | P-5 |
| V4 | 产物零污染 | auto-os/app 仓跑 rust 后端，两仓 `git status` | 框架仓零新增产物 | P-2 |
| V5 | 注册表三源聚合 | §3-a 包装脚本启动双轨桌面 | 注册表呈现框架 demo + 本仓 `apps/` + kanban（repo 形态）三类；框架仓内原样启动零回归 | P-3 |
| V6 | 框架回归 | `cargo check -p auto-man -p auto-lang` + 模块测试；折叠前 `cargo tf` | 全绿（预存红按 564-Q6 台账豁免） | P-2/P-3 |
| V7 | 搬迁零残留 | 框架仓 grep 028-launcher/七计划名引用；INDEX/台账指针行核对 | 引用清零（指针行除外）；apps.manifest 与台账在本仓就位 | P-1/P-5 |
| V8 | 随迁计划可执行性 | 七项计划在 auto-os 开工冒烟一项（建议 554 clock 或 577 债批） | 解析序换算后路径可达、流程可续 | P-1 后 |
| V9 | shell pack 路径化保真 | §4-P7：无 pack 回退路径与现状逐字节一致；`AUTO_SHELL_PACK`/兄弟命中本仓 `shell/` 加载生效；hash-lock 契约（同步脚本比对四件 hash） | 回退零变化 + pack 生效 + 双源无静默漂移 | P-7 |

## §7 实施拆解

排序原则：治理先行、纯文件先行、框架改动小面先行、资产搬迁移入窗口。每批
一 plan 一 worktree（本仓组 `.wt/os-NNN/auto-os`；框架仓改动沿 `lang-NNN` 组）。

| 批次 | 内容 | 主仓 | 前置 | 窗口约束 |
|---|---|---|---|---|
| **P-1** | 计划随迁批：§1-A2 七项 → `os-NNN` 重编 + `origin` 注记 + INDEX 指针行 | auto-os | 本设计定案 | 无（可与 541/582 收口并行） |
| **P-2** | 清障一：rust-server 落点可配（§4-P2）——**✅ 已落地（2026-09-07，auto-lang PLAN-587）** | auto-lang | 无 | 无 |
| **P-3** | 清障二：注册表三源聚合（§4-P3，含 manifest schema 定稿）——**✅ 已落地（2026-09-07，auto-lang PLAN-586；执行期修正见 §4-P3 注记）** | auto-lang | 无 | 无 |
| **P-4** | VM 债族修复（source_root=0 / m12/m16；583 台账 + `scratch/p583` 复现器）——**✅ 已落地（2026-09-07，auto-lang PLAN-588；根因=静态模块白名单缺 file 的占位 receiver 漏槽，单点修+m16b 对账 319890==319890）** | auto-lang | 无 | 无（**建议**先于 P-5——shell.at 是该形态高密度用户，搬迁回归前修比回归中踩雷便宜） |
| **P-7** | shell pack 批：§4-P7 加载器 + §1-A4 四件物理迁入 `shell/` + 权威翻转 + hash-lock 同步契约——**✅ 已落地（2026-09-07，auto-lang PLAN-589 + 本仓 shell/ 四件 + sync 脚本）** | auto-lang 改造 + auto-os 落位 | P-3 落地 ✅（同族解析序机制复用） | 无硬窗口；已先于 P-5 完成 ✓ |
| **P-5** | 资产搬迁本体批：A1 台账 + A3 launcher + B 批 apps + common 抽取 + L8 指针登记（L2 处置见 P-7 pin 快照）+ 框架层行数实测归档（tokei/cloc） | 两仓 | P-2/P-3 落地（搬完即可跑）；P-4/P-7 建议先行 | **541/582 合并后**（唯一硬窗口） |
| **P-6** | 随迁计划开工：七项在 auto-os 逐个执行（首个建议 554 或 577——小面验证解析序与流程） | auto-os | P-1 + P-5 | — |
| Stage C 候选 | `auto desktop` 一等子命令（§3-b）；auto-cosmic 归属重估（L4）；app 升格独立仓 17xxx 带规约化 | — | Stage B 稳定后 | — |

P-2/P-3/P-4 互不依赖、不依赖 P-1/P-5/P-7 时机，设计定案后即可并行立项；
P-7 依赖 P-3（同族机制先行复用），其余批次按上表前置即可。

