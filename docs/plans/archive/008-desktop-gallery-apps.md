---
plan_id: PLAN-008
origin: PLAN-578
status: archived               # drafting → executing → execution_done → reviewed → archived
feature_name: desktop-gallery-apps
author: [ZCode, zhaopuming]
created_at: 2026-09-07
updated_at: 2026-09-11

# /auto-plan:review 结束时填写：
supersedes_spec_components: []
new_spec_components: [P008-1, P008-2]
touched_goals: []             # auto-os 无 canonical goals 文档（goals 节空）；
                              #   正文 GOAL-009/010 引用为 origin 随迁指针，
                              #   非 auto-os 侧可触达成份——空 Impact 说明。
affects: [auto-os/desktop-registry, auto-lang/ui]
current_step: 7
total_steps: 7
---

> **随迁注记（Stage B P-1，2026-09-07，PLAN-001）**：本计划自 auto-lang
> `docs/plans/578-desktop-gallery-apps.md`（origin PLAN-578）随迁重编为 PLAN-008——drafting 原状、
> 状态机续用，git 历史留 auto-lang 仓。设计依据
> [Design 01 §5](../design/01-stage-b-desktop-migration.md)；auto-lang 侧指针行见
> 其 `docs/plans/INDEX.md`。正文中的 auto-lang 相对路径/行号锚点，开工时按本仓
> AGENTS §2 解析序换算（env → `../auto-lang` → `D:/autostack/auto-lang`），
> 现文不回改。

> **执行重锚定案（2026-09-11，/auto-plan:work 开工轮）**：计划起草后 P-5
> （2026-09-07，PLAN-590）已把画廊两件自 auto-lang `examples/` 迁 **auto-os
> 顶层**（auto-lang 侧 047743158 git rm；auto-lang 残留目录为未跟踪构建
> 产物）——§详细设计 1/3 的 `apps_dir.parent()` 兄弟锚定前提失效，按 §6
> 重锚条款改锚 **`resolve_os_top_dir` 解析序**（`AUTO_OS_ROOT` env → 兄弟
> → 主检出；P-5 为框架消费随迁资产既建机制，docs 管线同款）。语义三件套
> 不变（按名探测 / 缺席静默跳过 / storage `shell.apps.scan_galleries`
> 开关）。逐任务重锚：**T1** pac 落点=auto-os 顶层两 pac.at；**T2**
> `gallery_extra_roots()` 去 `apps_dir` 参 + 纯函数
> `gallery_extra_roots_from(storage_value, parent)` 参数化（测试设计 2
> 的 storage 注入两难按预案取参数化臂）；**T3** 接线折叠进
> `host_extra_roots()`（renderer.rs boot `None` 臂自动覆盖，renderer 零
> 改动——行号 ~11274 现漂移 11377 亦不再需要）；**T4** vue.rs
> `desktop_extra_app_roots(root_dir)` 现形态无 apps_dir 参，缺省臂在
> manifest 臂后追加 `gallery_extra_roots_from(None, parent)`（env
> `AUTO_DESKTOP_APPS_EXTRA` 全替换臂不动）；**加座**：`AUTO_DESKTOP_APPS_EXTRA`
> 全替换语义下脚本化 vue 轨的画廊供给 = `scripts/desktop.{ps1,sh}` EXTRA
> 追加两画廊根（widgets-gallery 由 render 过滤自然排除——设计行为）；
> 三轨 parity 测试组合面同步画廊臂（fixture 补画廊目录防主检出兜底泄入）。
> **T6** 实机走 `scripts/desktop.{ps1,sh}`（P-6 包装）双轨。证据目录沿
> 正文 `docs/plans/evidence/578/`。执行落点：worktree 组
> `.wt/os-008/{auto-os,auto-lang}`（分支均 `os-008-dev`；base=auto-os
> `a8d7450` / auto-lang master `622edfdd9`）。

# [PLAN-008] 桌面画廊上架——ui-gallery / widgets-gallery 注册进虚拟桌面缺省应用面

## 变更摘要

把 `examples/ui-gallery` 与 `examples/widgets-gallery` 两个画廊应用注册进
虚拟桌面的**缺省**应用注册表扫描面。机制沿用 Plan 501 多根聚合的相邻仓
探测惯用法：双轨宿主的 extra 根缺省探测各扩展两个**仓内**画廊根（双轨
同律锚定 `apps_dir.parent()/{ui-gallery,widgets-gallery}`——主根指哪
画廊跟哪，PLAN-579 对齐，见 §详细设计 6），外部自含根按
PLAN-552 语义缺省 `desktop_visible=true`（opt-out），无需 pac `desktop:`
字段即入 launcher/图标格/dock 消费面。两 pac.at 补 `title:`/`icon:`
展示字段。VM 轨新增 `shell.apps.scan_galleries` storage 开关（缺省开，
命名对齐既有 `shell.apps.scan_siblings`）。

另收一项（2026-09-11 用户裁定补入）：**022-kanban 教学 demo 退桌面策展**
——独立仓 auto-kanban（apps.manifest extra root 挂载，P-3 已落地）替位，
examples/ui 早期 demo 不再上桌面：删 pac `desktop:` 行 + 策展恰等断言
同步（C 档 17→16）；demo 本体留 examples/ui 作教学与收割语料
（§详细设计 7）。

不动：examples/ui 主根扫描机制与 Plan 552 boot 两分逻辑、
`extra_roots_from`/`desktop_extra_app_roots` 的 env 覆盖语义
（`AUTO_DESKTOP_APPS_EXTRA` 设置时整体替换缺省，保持不变）。

**与 PLAN-579（auto-os 立项）的对齐**（2026-09-07 追加，同日二次修订）：
579 裁定 auto-lang=语言/框架根、auto-os=产品根（伞形）、真实 app 走独立
仓；其 Stage A 只建伞形骨架+首个 app 仓，桌面 shell 迁移=Stage B（另行
立项）。**执行时序（用户裁定）：579 → Stage B（新虚拟桌面可用）→ 578
延期执行**——本计划不先行落地现行桌面，落点=届时桌面宿主（Stage B
迁移后的注册表装配处），执行前按届时代码位置重锚文件路径（设计不变，
仅重锚；见 §详细设计 6）。双轨画廊探测统一按**主根（apps_dir）兄弟
锚定**（§详细设计 1/§3）：届时 apps_dir 按 579 AGENTS.md 跨仓解析序
约定（`AUTO_LANG_ROOT` → 兄弟 `../auto-lang` → 主检出）仍指 auto-lang
`examples/ui`，画廊条目自动跟随。画廊与 examples/ui 同属 **auto-ui
项目**资产（因 cargo 管理约束现居 auto-lang；未来随 auto-ui 栈拆仓迁出
——方式待定，用户举例 git submodule；见 §详细设计 6/待澄清③）。

## 目标

1. **ui-gallery 上架**（用户裁定：替代 Plan 552 从桌面清退的十多个教学
   demo——demo 探索职能已收拢进画廊，画廊本身理应在桌面）：
   - Vue desktop-host 轨（其原生形态，`render: "vue"`）：生成注册表含
     `ui-gallery`，可 launch。
   - VM 轨：注册可见（boot 不过滤 render，Plan 463 T7 裁定）；launch
     兼容性以"不崩桌面"为验收口径（占位页兜底属设计内降级）。
2. **widgets-gallery 上架**（用户裁定：首个独立 UI app + AutoUI 文档与
   展示面）：
   - VM 轨（其原生形态，`render: "vm"`，Plan 411 VM 原生窗口）：注册
     可见、可 launch。
   - Vue 轨：因 pac 声明 `render: "vm"` 被 `Some("vue")` 过滤器排除
     ——设计行为（前端目标声明即过滤，Plan 559 双轨对齐语义），本计划
     不绕过。
3. **展示元数据完备**：两画廊 pac.at 补 `title:`（"UI Gallery" /
   "Widgets Gallery"）与 `icon:`（`image` / `layout-grid`——两者均在
   iced 渲染器 lucide 清单内，renderer.rs:5312-5313）。
4. **可关**：VM 轨 `shell.apps.scan_galleries=false` 可整体关闭画廊
   探测（对称 `shell.apps.scan_siblings`）；Vue 轨沿用
   `AUTO_DESKTOP_APPS_EXTRA` 整体覆盖既有语义，不新增开关。
5. **022-kanban 退策展**（用户裁定 2026-09-11）：`examples/ui/022-kanban/
   pac.at` 删 `desktop: "true"` 行 + `scan_examples_ui_curation_set`
   want 集同步（C 档 17→16）——app 升格独立仓后早期 demo 退桌面，
   独立 app 替位（auto-kanban 已挂载，零新增机制）。

## 架构方案

不改架构——纯扫描面扩展，复用两套既有机制：

- **多根聚合**（Plan 501 `aggregate_scan`：主根优先按 id 去重 + extra
  根 `scan_app_root` 自含模式）；
- **缺省探测**（相邻仓 `../auto-os-config/auto` 惯用法：目录缺席静默
  跳过，宿主在任意检出/worktree 形态下不失效）。

双轨落点：

| 轨道 | 注册表装配点 | 画廊根来源 | 可见性 |
|---|---|---|---|
| VM（iced ui_desktop） | `renderer.rs` boot `aggregate_scan`（~11274） | `host_extra_roots()` + 新 `gallery_extra_roots(apps_dir)` | 552 两分：策展集含画廊（外部根 opt-out 缺省 true） |
| Vue（desktop-host） | `vue.rs generate_desktop_host`（~3650） | `desktop_extra_app_roots(root_dir, apps_dir)` 缺省臂扩展（画廊按 **apps_dir 兄弟**锚定，579 对齐） | render 过滤：ui-gallery 入、widgets-gallery 排除 |

## 需求分析与背景调查
（从 docs/specs/goals.md GOAL-009/010 与代码实勘取材）

- **现状缺口**：虚拟桌面注册表扫描面 = 主根 `examples/ui` + 缺省相邻仓
  `../auto-os-config/auto`（VM 轨 `host_extra_roots`，Vue 轨
  `desktop_extra_app_roots`，Plan 559 W3 双轨对齐）。两画廊位于
  `examples/ui-gallery`、`examples/widgets-gallery`——主根的**兄弟目录**，
  不在任何扫描面内；master 实测策展集恰为 examples/ui 的 20 个
  `desktop: "true"` app，无画廊条目。Plan 549（画廊建设）/552（策展）
  均未规划画廊上架——整理工作的自然缺口，本轮补上。
- **用户裁定**（2026-09-07）：ui-gallery 替代已下架的十多个教学 demo
  （552 清退的 Tier1/2 教学示例职能已收拢进画廊），理应上架；
  widgets-gallery 是自研的第一个独立 UI app，兼 AutoUI 文档与展示面，
  也有理由上架。
- **用户裁定**（2026-09-11，策展规则补入）：app 一旦升格独立仓
  （apps.manifest 注册），其 examples/ui 早期教学 demo 即退桌面策展——
  demo 本体留框架仓继续作教学示例与 ui-gallery 收割语料，桌面消费面只
  呈现独立 app。首例 kanban：`examples/ui/022-kanban/pac.at` 仍带
  `desktop: "true"`（策展 C 档 17 之一，断言锚 `app_registry.rs:540`，
  want 数组 557 行），与 auto-kanban（manifest extra root 原生挂载，
  title 同为 "Kanban"）在新桌面双入口并存——本计划清此重叠；后续 app
  升格沿用此规则（同样仅退策展 + 断言同步，零迁移）。
- **PLAN-552 语义适配**：外部自含根 `desktop_visible` 缺省 true
  （opt-out）——画廊以 extra 根形态注册即上架，零 pac 改动（pac
  `desktop:` 显式值仍可覆盖）。策展恰等断言只扫 examples/ui 主根，
  不受影响。
- **Vue 轨无 desktop_visible 过滤**（实勘：vue.rs 零处引用）——其隐式
  过滤即 `ScanOptions.render = Some("vue")`（`generate_desktop_host`
  3639-3642）；widgets-gallery 声明 `render: "vm"` 自然排除。
- **入口探测兼容**（实勘）：两画廊均为 `src/front/app.at`
  （`probe_entry` 兜底形态），`scan_app_root` 直接可用。
- **图标格徽标色**：`badge_color_for` 按 id 哈希 8 色板（Plan 518 G4③），
  零配置面。
- **PLAN-579 组织重划**（2026-09-07 用户裁定，与本计划起草同期）：auto-lang
  =语言/框架根、auto-os=产品根（伞形组织根）、真实 app 走独立仓（首个
  auto-plans 计划看板）；桌面 shell 迁移 = Stage B 另行立项。另用户澄清
  UI 栈/示例属 auto-ui 项目（现居 auto-lang 系 cargo 管理约束，未来拆仓
  或以 submodule 形态迁出）。对本计划的影响与迁移姿态见 §详细设计 6
  （结论：落点照常、探测锚定加固为 apps_dir 相对、画廊与主根同树共置
  不变式覆盖 auto-ui 拆仓各形态）。
- 相关 GOAL：GOAL-009（虚拟桌面与桌面 Shell）、GOAL-010（示例应用轨道）。

## 详细设计

### 1. VM 轨：`crates/auto-lang/src/ui/app_registry.rs`

新公开函数（放 `host_extra_roots` 邻位）：

```rust
/// PLAN-578：仓内画廊根探测（ui-gallery / widgets-gallery）——
/// `apps_dir.parent()` 下按名探测，目录缺席静默跳过；
/// storage `shell.apps.scan_galleries=false` 可关（对称 scan_siblings）。
pub fn gallery_extra_roots(apps_dir: &Path) -> Vec<(String, PathBuf)> {
    if crate::vm::ffi::stdlib::storage_host_read("shell.apps.scan_galleries")
        .as_deref() == Some("false")
    {
        return Vec::new();
    }
    const GALLERIES: [&str; 2] = ["ui-gallery", "widgets-gallery"];
    GALLERIES
        .iter()
        .filter_map(|name| {
            let p = apps_dir.parent()?.join(name);
            p.is_dir().then(|| ((*name).to_string(), p))
        })
        .collect()
}
```

（实现期可微调形态，语义三件套不变：按名探测 `apps_dir` 兄弟目录 /
缺席跳过 / storage 开关。）

### 2. VM 轨接线：`crates/auto-lang/src/ui/iced/renderer.rs`（~11274）

```rust
let mut extra = crate::ui::app_registry::host_extra_roots();
extra.extend(crate::ui::app_registry::gallery_extra_roots(apps_dir));
let full = crate::ui::app_registry::aggregate_scan(apps_dir, &extra, &…);
```

`aggregate_scan` 按 id 去重主根优先，画廊 id 与主根无冲突；boot 计数
eprintln（`{} entries ({} desktop-visible)`）自动反映 +2/+2。

### 3. Vue 轨：`crates/auto-man/src/vue.rs` `desktop_extra_app_roots`（~5507）

签名增参 `apps_dir`（私有 fn，单调用点 + 测试）。os-config 兄弟仓探测
保持 root_dir 锚定不变；**画廊探测改按 apps_dir 锚定**——与 VM 轨 §1
同律（PLAN-579 对齐：Stage B 宿主迁 auto-os 后 apps_dir 经跨仓解析序
仍指 auto-lang examples/ui，画廊探测不随宿主搬迁失效；若锚定
root_dir/examples/ 则宿主一搬即探测空路径）：

```rust
fn desktop_extra_app_roots(root_dir: &Path, apps_dir: &Path) -> Vec<(String, PathBuf)> {
    // …env AUTO_DESKTOP_APPS_EXTRA 覆盖臂与 os-config 兄弟探测原样（root_dir 锚定）…
    // 缺省臂追加（PLAN-578）：主根兄弟画廊——apps_dir.parent() 锚定
    if let Some(parent) = apps_dir.parent() {
        for name in ["ui-gallery", "widgets-gallery"] {
            push_root(parent.join(name), &mut out); // 既有闭包：is_dir 才入，id 取目录名
        }
    }
    out
}
```

调用点 `generate_desktop_host`（~3650）传 `&apps_dir`。注意 `push_root`
入的是 `(目录名, 路径)`——id 即 `ui-gallery` / `widgets-gallery`，
与 VM 轨一致。env `AUTO_DESKTOP_APPS_EXTRA` 覆盖路径**早返回不动**
（整体替换语义保持）。`generate_desktop_host` 的 render 过滤随后自然
裁决：ui-gallery 入、widgets-gallery 剔除。

### 4. pac 展示字段：两画廊 `pac.at`

- `examples/ui-gallery/pac.at`：增 `title: "UI Gallery"`、`icon: "image"`
- `examples/widgets-gallery/pac.at`：增 `title: "Widgets Gallery"`、
  `icon: "layout-grid"`

条目字段解析链：`title:` → `name:` → 目录名；`icon:` → 回退
`app-window`（`entry_for_dir` 114-123）。

### 5. 已知边界（验收口径，非债）

- VM 轨 launch ui-gallery：前端为 Vue 形态，解释器路径可能降级
  （Plan 463 T7 panic 边界 + 占位页兜底为设计内降级）——验收只要求
  "不崩桌面"；全功能体验走 Vue 轨。
- Vue 轨 widgets-gallery 缺席：render 声明过滤，设计行为。

### 6. 与 PLAN-579（auto-os 立项）的关系与迁移姿态

- **时序**（用户裁定 2026-09-07 二次修订：**延期执行**）：579 →
  Stage B（新虚拟桌面可用）→ 578。本计划不先行落地现行桌面——现行桌面
  有生之年无画廊（非回归：画廊从未上过现行桌面），换取避免迁移动窗口内
  双轨双冒烟、以及 Stage B 搬迁改写丢线的风险。579 Stage A 对 auto-lang
  零代码改动（其边界声明），与本计划无机制依赖。
- **落点=届时宿主**：执行时桌面注册表装配代码位于 Stage B 迁移后的宿主
  仓——**设计/锚定不变，文件路径与接线点按届时代码重锚**（执行前刷新
  本计划 T2-T6 的路径/行号引用，机械工作）。锚定不变式保证重锚可行：
  apps_dir 兄弟探测 + 579 解析序指向 auto-lang `examples/ui`，画廊条目
  自动跟随主根。
- **迁移零适配**：双轨画廊探测统一 apps_dir 兄弟锚定（§1/§3）。Stage B
  后宿主在 auto-os，apps_dir 按 579 AGENTS.md 跨仓解析序约定（env
  AUTO_LANG_ROOT → 兄弟 ../auto-lang → D:/autostack/auto-lang 主检出）
  解析回 auto-lang examples/ui——画廊条目自动跟随主根，无需改 578 代码。
- **双画廊与示例的长期归属**（用户 2026-09-07 澄清）：widgets-gallery /
  ui-gallery 与 examples/ui 同属 **auto-ui 项目**资产——auto-ui 仓
  （`D:/autostack/auto-ui`，2026-02 起休眠于 Plan 096 时代，末提交
  f7d92ee）历史上即承载 UI 栈 + auto-examples；现因 Rust cargo 管理约束
  整体居于 auto-lang。未来 auto-ui 栈拆仓迁出时（方式待定，用户举例
  git submodule；579 Stage A 对伞形仓否决 submodule 是 auto-os 组织面
  决策，与此处框架仓拆分不冲突——同属"按需再议"口径），示例/画廊随
  项目迁置（去向两读见待澄清③）。578 的**锚定不变式 = 画廊与主根示例
  同树共置**（同项目资产共进退）——无论未来整体留 auto-lang、迁 auto-ui
  独立仓或 submodule 挂载形态，`apps_dir.parent()` 兄弟探测始终成立；
  若拆仓后画廊与主根分置两树，以 storage `shell.apps.extra_dirs` 注册
  画廊根即可（机制已备，零返工）。

### 7. 022-kanban 退策展（2026-09-11 用户裁定补入）

- `examples/ui/022-kanban/pac.at`：删 `desktop: "true"` 行——沿 552
  opt-in 语义（主根 demo 缺省不上桌面），examples/ui 既有非桌面 demo
  均为无字段形态，不引入显式 false 惯例；留一行注释指位替位方
  auto-kanban（pac `//` 注释形态，沿 auto-kanban pac.at 先例）。
- `crates/auto-lang/src/ui/app_registry.rs` `scan_examples_ui_curation_set`
  （:540）：want 数组去 `"022-kanban"`（:557），断言文案与计数注释
  C 档 17→16，注记 PLAN-008 退策展缘由（沿 551/553/590 既有计数注记
  形态）。
- 替位方零改动：auto-kanban 经 apps.manifest（P-3 落地）extra root
  原生挂载已入 VM/Vue 双轨注册面，本项不触 manifest/挂载机制。
- 022-kanban 目录本体不迁不删：`auto run` 开发形态、ui-gallery 收割
  语料、`scan_examples_ui_finds_at_least_27_apps`（≥33）口径均不变。

### 规范增量（2026-09-11 review 轮定稿；ledger 条目 merge 时落账）

- **P008-1（architecture）桌面注册表画廊根契约**：画廊两件
  （ui-gallery/widgets-gallery，auto-os 顶层随迁资产）经
  `resolve_os_top_dir` 解析序（`AUTO_OS_ROOT` env 设置即权威 → 兄弟
  `parent/auto-os` → 主检出 `D:/autostack/auto-os`）按名探测入桌面注册
  面 extra roots，双轨同律（VM `host_extra_roots`/vue
  `desktop_extra_app_roots` 缺省臂）；目录缺席静默跳过（solo 检出不
  炸）；VM 轨 storage `shell.apps.scan_galleries=false` 整体关断（对称
  `scan_siblings`），vue 轨关断 = `AUTO_DESKTOP_APPS_EXTRA` 全替换既有
  语义（desktop 脚本 EXTRA 显式注入画廊两件）；外部自含根 opt-out 缺省
  可见（PLAN-552 语义），展示字段 pac `title:`/`icon:`。锚定不变式：
  画廊与主根示例的共置由解析序承载（P-5 随迁后 apps_dir 兄弟锚退役）。
- **P008-2（tests）注册表画廊验证矩阵注记**：真实材料门控四单测
  （probes/off/entries/render_filter）+ vue 轨缺省臂/env 全替换两腿 +
  三轨 parity（fixture 含画廊目录防主检出兜底泄入）；boot 计数受控基线
  语义（worktree 组内 36/20→38/21=+2/+1；组构成差——auto-os-config/
  auto-kanban 检出缺席——须同组对照，禁跨组直比）；策展恰等断言 16 即
  退策展回归载体（022-kanban 出、不增不减）。

（P008-1/P008-2 落 `.autoos/specs.json` architecture/tests 节，merge 轮
执行；本计划不动 live ledger。）

## 测试设计

全部新增测试紧贴既有形态（app_registry.rs 真实仓库材料门控单测 +
vue.rs 同文件 tests mod）：

1. `gallery_extra_roots_probes_repo_galleries`（app_registry）：以
   `repo_examples_ui()` 为 apps_dir，断言产出恰 `(ui-gallery, …)`、
   `(widgets-gallery, …)` 两根（真实仓材料，与既有
   `scan_examples_ui_finds_at_least_27_apps` 同款门控）。
2. `gallery_extra_roots_scan_galleries_off`：storage 开关注入
   `shell.apps.scan_galleries=false` → 空表。（storage_host_read 注入
   方式参照 extra_roots_from 既有测试 576-592 行形态；若 storage 不可
   注入则把开关判定提为纯函数参数化——实现期定，语义不变。）
3. `scan_gallery_roots_entries_visible`：`scan_app_root` 对两画廊目录
   产出条目——`desktop_visible == true`（外部根 opt-out）、`title` 取
   pac `title:`、`icon` 取 pac `icon:`、`render` 字段原样。
4. `gallery_render_filter_matrix`：widgets-gallery 目录条目在
   `ScanOptions{render: Some("vue")}` 下 None、default（None）下 Some
   ——钉死 Vue 轨排除/VM 轨收录的双轨语义。
5. `desktop_extra_app_roots_default_includes_galleries`（vue.rs tests
   mod）：tmp 造 `ws/examples/ui`（apps_dir）+ `ws/examples/ui-gallery`
   空目录，断言缺省臂含该根且路径锚定 `apps_dir.parent()`——**root_dir
   故意取独立 tmp 目录**（异根臂，钉死 579 对齐锚定不依赖 root_dir）；
   env 覆盖臂断言 galleries 不在（整体替换语义）。（env 测试需串行/
   隔离——参照 vue.rs 既有 env 测试的处理形态。）

策展恰等断言（`scan_examples_ui_curation_set`）**随退策展同步**（2026-09-11
补入）：want 去 `"022-kanban"`（C 档 17→16）；其双向恰等特性（多一/少一
即红）本身即钉死退策展回归，无需新增测试。扫描数断言（≥33）不变——
退策展只动 desktop_visible，不动扫描面。

## 验收标准

1. VM 桌面 boot：注册表日志相对现状 **+2 entries / +1
   desktop-visible**（画廊 +2/+2，022-kanban 退策展 -1）；launcher/图标格
   可见 UI Gallery 与 Widgets Gallery（图标 `image`/`layout-grid`、
   标题非裸 id）。
2. VM 桌面实机：launch `widgets-gallery` 可用（VM 原生形态）；launch
   `ui-gallery` 不崩桌面（占位页或可用均可，见 §详细设计 5）。
3. Vue desktop-host 生成：输出含 `extra root: ui-gallery`，注册面登记
   ui-gallery（extra root 日志 + 单测断言）；widgets-gallery 不在（render
   过滤，设计行为）。
   **（2026-09-11 用户裁定修正）**：原后半"注册表含 ui-gallery 且可
   launch"按待澄清④裁定 amend——vue desktop-host 前台 materialize 受
   Plan 465 登记 v1 限制（`@/ext/` 导入者跳过）约束，ui-gallery 不入
   apps-registry.ts；画廊全功能体验走 VM 轨（实机已证 launch 后真渲染
   内容页，非占位页）。ext 物化立 KNOWN-DEBT 候选（详见复审记录 F3）。
4. `shell.apps.scan_galleries=false` 后 VM 轨两画廊消失（单测 +
   可选实机）。
5. 门禁：`cargo check -p auto-lang -p auto-man` 触及文件零新警告；
   `cargo t gallery_extra`、`cargo t scan_gallery`、
   `cargo t desktop_extra` 全绿；`cargo t app_registry` 全绿。
6. 022-kanban 退策展生效（2026-09-11 补入）：双轨桌面 launcher/图标格/
   注册表无 022-kanban 条目，Kanban 入口唯一（auto-kanban）；
   `cargo t scan_examples_ui`（含策展恰等 + ≥33 扫描两断言）绿。

## 执行步骤
（原子任务：精确文件路径 + 确切操作 + 验证命令；每步完成后追加 [✅ 已完成] 一行证据）

- [x] **T1 pac 展示字段**：`examples/ui-gallery/pac.at` 增
  `title: "UI Gallery"` + `icon: "image"`；`examples/widgets-gallery/pac.at`
  增 `title: "Widgets Gallery"` + `icon: "layout-grid"`。
  验证：`grep -n "title:\|icon:" examples/ui-gallery/pac.at examples/widgets-gallery/pac.at`
  [✅ 已完成] 重锚落点=auto-os 顶层两 pac.at（worktree os-008-dev）；grep
  四行在案（icon:/title: 各 12/13 行）；lucide 在册 renderer.rs:5338-5339；
  `scan_gallery_roots_entries_visible` 断言 title/icon 透传过（app_registry
  19/19）。
- [x] **T2 app_registry 画廊根探测**：`crates/auto-lang/src/ui/app_registry.rs`
  增 `gallery_extra_roots`（§详细设计 1）+ 测试设计 1/2/3/4 四单测。
  验证：`cargo t gallery_extra && cargo t scan_gallery && cargo t gallery_render_filter`
  [✅ 已完成] 重锚：`gallery_extra_roots()`（去 apps_dir 参）+ 纯函数
  `gallery_extra_roots_from(storage_value, parent)` 走 `resolve_os_top_dir`
  解析序；四单测 probes_repo_galleries / scan_galleries_off（storage 注入
  两难按预案取参数化臂）/ entries_visible / render_filter_matrix；
  nextest `cargo t app_registry` 19/19 绿（os-008 worktree，auto-lang
  `os-008-dev`）。
- [x] **T3 VM 轨 boot 接线**：`crates/auto-lang/src/ui/iced/renderer.rs`
  ~11274 处 extra 拼接 `gallery_extra_roots(apps_dir)`（§详细设计 2）。
  验证：`cargo check -p auto-lang`（零新警告）
  [✅ 已完成] 重锚：接线折叠进 `host_extra_roots()` 末尾
  `roots.extend(gallery_extra_roots())`——renderer.rs boot `None` 臂自动
  覆盖，**renderer.rs 零改动**；`cargo check -p auto-lang -p auto-man`
  过（触及文件零新警告，vue.rs:5561 mutable 警告=master 5556 既有位移）。
- [x] **T4 Vue 轨缺省臂扩展**：`crates/auto-man/src/vue.rs`
  `desktop_extra_app_roots` 增 `apps_dir` 参 + 缺省臂按主根兄弟追加两
  画廊探测（§详细设计 3，579 对齐锚定）+ 测试设计 5 单测（含 root_dir
  异根锚定臂）。
  验证：`cargo t desktop_extra_app_roots`
  [✅ 已完成] 重锚：缺省臂 manifest 臂后追加
  `gallery_extra_roots_from(None, parent)`（P-5 解析序，签名不动）；
  测试设计 5 重锚为 `desktop_extra_app_roots_default_includes_galleries`
  （tmp 异根 fixture + manifest 占位防兜底泄入 + env 全替换腿）；三轨
  parity 测试组合面同步画廊臂（fixture 补画廊目录）；nextest
  `-p auto-man desktop_extra` 2/2 + `three_track` 1/1 绿。**加座**：
  `scripts/desktop.{ps1,sh}` vue 轨 EXTRA 追加两画廊根（env 全替换语义下
  脚本化轨道的画廊供给；widgets-gallery 由 render 过滤自然排除）。
- [x] **T5 局部门禁**：`cargo check -p auto-lang -p auto-man` +
  `cargo t app_registry` + 新增滤串复跑。
  验证：命令全绿、触及文件零新警告
  [✅ 已完成] check 过（警告全既有，无触及文件新警告）；nextest：
  `app_registry` 19/19、`scan_examples_ui` 2/2、`-p auto-man
  desktop_extra` 2/2、`three_track` 1/1、`extra_roots` 双 crate 5/5
  全绿（os-008 worktree）。
- [x] **T6 双轨实机冒烟**：VM 轨
  `cargo run -p auto-lang --features ui-iced --example ui_desktop -- --fullscreen`
  ——boot 日志画廊 +2/+2（T7 已先行则 desktop-visible 净 +1，验收 1
  口径）、图标格两画廊可见、launch widgets-gallery 可用、launch
  ui-gallery 不崩；Vue 轨 `examples/desktop-host` 下
  `auto run --desktop`——生成日志含 `extra root: ui-gallery`、可 launch。
  证据：日志摘录 + 截图存 `docs/plans/evidence/578/`（可选用
  autoui-verifier 脚本）。
  [✅ 已完成] 证据集+索引存 `docs/plans/evidence/578/`（README.md 为入口）。
  **iced 轨**：受控对照（同 os-008 组 CWD）master exe `36 (20)` → 新 exe
  `38 (21)` = **+2 entries / +1 desktop-visible**（验收 1 精确命中）；
  全屏/窗口冒烟 boot 未崩（截图两张）。**vue 轨**：worktree `auto` CLI +
  脚本 env——`✓ extra root: ui-gallery` 精确命中（验收 3 前半）；
  widgets-gallery render 过滤缺席（设计行为）；`apps-registry.ts (23 apps)`
  落地。**受挫项（验收 3 后半）**：ui-gallery 被生成器 v1 限制
  `needs ext files`（Plan 465 登记，`@/ext/` 导入者跳过）拦在
  apps-registry.ts 之外——扫描面登记 ✓、前台 materialize ✗，先于本计划
  的生成器边界，非本计划回归；处置见待澄清④。**交互腿**（图标格目视/
  launch 两画廊/scan_galleries=false 实机）沿 472/478 先例 headless 指针
  成文，待用户实机复核。
- [x] **T7 022-kanban 退策展**（2026-09-11 补入；建议先于 T6 执行使冒烟
  覆盖终态，无依赖可任意位次）：`examples/ui/022-kanban/pac.at` 删
  `desktop: "true"` 行（留注释指位 auto-kanban）；
  `crates/auto-lang/src/ui/app_registry.rs` 策展断言 want 去
  `"022-kanban"` + 计数注释 C 档 17→16（§详细设计 7）。
  验证：`cargo t scan_examples_ui`（策展恰等 + ≥33 两断言）绿；桌面
  Kanban 入口唯一（auto-kanban）。
  [✅ 已完成] worktree 两处落地（auto-lang examples/ui/022-kanban/pac.at
  注释替位 + want 数组去 022-kanban，断言文案/注释 17→16 注记 PLAN-008）；
  nextest `scan_examples_ui` 2/2 绿（恰等 16 + ≥33）；桌面入口唯一性
  归 T6 实机核对。

## 复审记录

### 复审记录（2026-09-11，/auto-plan:review 轮）

`stage: review | plan_id: PLAN-008 | plan_revision: r2（执行重锚定案 +
验收 3 修正，2026-09-11 用户裁定）| outcome: pass | reviewed_commit:
auto-os 2c012a3 / auto-lang 45ae96897（两 worktree 复核时点干净无脏区）
| base_commit: auto-os a8d7450 / auto-lang 622edfdd9 |
dependency_revisions: auto-down 1557a39（os-008-dev，零改动）|
spec_inputs: .autoos/specs.json sections@562a7cd（goals 节空）|
acceptance_results: AC1 pass / AC2 pass / AC3 pass（r2 修正后口径）/
AC4 pass / AC5 pass / AC6 pass | findings: F1-F3 全 info 级非阻塞
（见下）| evidence: docs/plans/evidence/578/（复审轮自主复现 + 新增
launcher-open-21apps / launch-widgets-gallery-vm / launch-ui-gallery-vm
三截图与 iced-scan-off.log）| next: /auto-plan:merge`

**独立性声明**：复审与实现在同一会话承载（无独立复审会话授权）——结论
按技能要求自工件重构，非执行者摘要转述：全量门禁 `cargo tf
--no-fail-fast` 复审档重跑 **3506/3506 全绿**（worktree，commit 后重跑）；
iced 轨受控对照、AC4 存储关断、双 launch 腿均复审轮**新boot 实机**复现
（AUTOUI_MCP_PORT=9348 + AUTOUI_ACCEPTANCE=1 验收注入通道 + 存储隔离）；
vue 轨日志复用自 work 轮，理由=运行时点与提交内容逐字节一致（先跑后
commit，无中间改动）。

**验收逐条**：
- **AC1 pass**：受控基线 `36 (20)` → 新 `38 (21)` = +2/+1 精确
  （iced-baseline-os008group.log vs iced-new-os008.log）；launcher 实机
  截图徽标 **"1 / 21 apps"**（=16 C 档 + 3 容器 + 2 画廊，含 kanban 应
  为 22——计数徽标即视觉面算术铁证）。行级目视（两画廊行在列表中的
  像素）→ 用户快核腿：launcher 覆盖层输入/键盘/handler 三条注入途径
  均作用域受限（MCP 输入定位根组件；ApplyFilter 入队未改过滤态），沿
  472/478 headless 指针先例成文。
- **AC2 pass（超预期）**：bus `launch\twidgets-gallery` → VM 原生窗
  **完整渲染**（Overview/Layout 侧栏 + "v1.0 — 61 Widgets" hero，截图）；
  bus `launch\tui-gallery` → **真内容页渲染**（"UI Gallery" 标题 +
  002-counter 教程卡片，非占位页——原设计按 §详细设计 5 只要求不崩），
  桌面存活（state 可读、launcher/dock 正常）。
- **AC3 pass（r2 修正口径）**：`✓ extra root: ui-gallery` 精确命中 +
  `desktop_extra_app_roots_default_includes_galleries` 单测；widgets-
  gallery render 过滤缺席=设计行为；前台 materialize 受 Plan 465 v1
  限制——按 ④ 裁定 amend 并立 KNOWN-DEBT 候选（F3）。
- **AC4 pass**：单测（参数化开关语义）+ **实机**：seed
  `shell.apps.scan_galleries=false` → boot `36 (19)`（画廊在时 38/21）
  ——iced-scan-off.log。
- **AC5 pass**：scoped 滤串全绿（work 轮）+ 复审档 `cargo tf
  --no-fail-fast` 3506/3506 全绿 + `desktop.sh` bash -n / `desktop.ps1`
  PSParser 双语法门过 + 触及文件零新警告（vue.rs:5561 mutable 警告=
  master 5556 既有位移，主检出对照实证）。
- **AC6 pass**：策展恰等断言 16（恰等特性双向钉死）+ `≥33` 扫描数不
  变（scan_examples_ui 2/2）；vue 注册表无 022-kanban（v1 skip 既有）；
  VM 轨 022 退策展（boot 计数 −1 可见）+ auto-kanban manifest 挂载
  （主检出组合 baseline 日志在案；os-008 组内无 auto-kanban 检出=
  solo 语义，与 P009-3 既有注记同律）。行级目视并入 AC1 用户腿。

**Findings（全 info 级非阻塞，无 reopen 项）**：
- **F1** vue.rs extra-root skip 文案 "no entry .at" 对 render 过滤跳过
  同型复用（既有措辞混用）——顺手修候选，不属本计划 scope。
- **F2** `ffi_dual_019_dep_layout_invariants` 在 1959 并行满载下偶发
  失败、隔离复跑即绿（master 同绿）——既有并行干扰 flake，与本计划零
  交集；框架侧观察项。
- **F3** KNOWN-DEBT 候选：desktop-host ext 依赖 app 物化（Plan 465 v1
  限制扩展；ui-gallery 首例消费方）——④ 裁定随批登记。

**Spec delta 复核**：`### 规范增量` P008-1（画廊根契约：解析序锚/开关/
双轨 parity/opt-out 可见性）与 P008-2（验证矩阵 + 受控基线语义）按验证
后行为成文，无与既有 canonical 知识冲突（P009-3 的 38/22 与 36/20 组内
solo 语义注记正交互容）；`new_spec_components: [P008-1, P008-2]`，
`supersedes_spec_components: []`，`touched_goals: []`（auto-os 无
canonical goals 文档，frontmatter 附空 Impact 说明）。ledger 落账归
merge 轮。

**状态**：`executing → reviewed`。下一步 `/auto-plan:merge`（worktree
组保留至 merge 清理；merge 前置含 ledger P008-1/P008-2 落账）。

### work 交接（2026-09-11，/auto-plan:work 轮）

`stage: work | plan_id: PLAN-008 | plan_revision: 执行重锚定案（2026-09-11）
| outcome: blocked（单点）——T1-T5/T7 全绿交付，T6 双轨冒烟完成，验收 3
后半受挫待裁定 | code_commit: auto-os 2c012a3 / auto-lang 45ae96897
（各 os-008-dev；base a8d7450 / 622edfdd9）
| task_ids: T1,T2,T3,T4,T5,T6,T7 | evidence:
docs/plans/evidence/578/（iced 受控对照 36(20)→38(21)=+2/+1；vue 生成
`✓ extra root: ui-gallery`；nextest app_registry 19/19 + scan_examples_ui
2/2 + desktop_extra 2/2 + three_track 1/1 + extra_roots 5/5；check 零新
警告）| blockers: 验收 3 后半"注册表含 ui-gallery 且可 launch"被 Plan 465
v1 限制（needs ext files）拦住，非本计划回归——待澄清④用户裁定
| next: ④裁定后（amend 验收或另立 ext-materialize 专项）→ execution_done
→ /auto-plan:review；交互腿用户实机复核可并入 review 轮`

### merge 归档回执（2026-09-11，/auto-plan:merge 轮）

`PLAN-008:r2 | completion_kind: delivered`

| 检查点 | 证据 |
|---|---|
| `prepared` | 复审基线 auto-os `2c012a3` / auto-lang `45ae96897`（r2 pass）；canonical diff=specs.json P008-1/P008-2（worktree `90e65a3`）；delivery 候选=两 os-008-dev 合流头 |
| `landed` | auto-os main **`f3e4570`**（merge os-008-dev --no-ff：pac ×2+scripts ×2+specs 沉积）；auto-lang master **`c4481ca58`**（merge 前先 reconcile master 591-610 → `db63beeb5` 零冲突，融合树 cargo tf 3509/3510 唯一红=ffi_dual_019 在案 flake 隔离绿，scoped 19+2+1 全绿，落地后 master 复跑 app_registry 19/19+scan_examples_ui 2/2） |
| `ledger_refreshed` | `.autoos/specs.json`：P008-1（architecture）/P008-2（tests）main 落地后读回在位（总条目 19）；file→本件 archive 终态路径 |
| `archived` | 本件 `git mv` → `docs/plans/archive/008-desktop-gallery-apps.md` + status archived + 台账行翻 📦 |
| `cleaned` | wt-guard：auto-os ✓ / auto-down ✓ 首跑 clean；auto-lang 首跑 **BLOCKED**（T6 vue 轨 pnpm 在 desktop-host gen 产物留 309 个 junction——guard 拦截生效）→ 按 guard 程序逐个 `cmd /c rmdir` 摘链接（309/309，零穿透）→ 复跑 clean → 三 worktree remove + 分支 `os-008-dev` ×3 删除（auto-os `90e65a3` / auto-lang `db63beeb5` / auto-down `1557a39` 零提交点）+ 残余 vite node 进程（PID 35040，锁 gen 目录）清杀 + 空组目录 `.wt/os-008` 移除；三仓 `worktree list` 复验零残留 |

并发注记：auto-lang 主检出存在他人并发 WIP（renderer.rs/snapshot.rs/
v05-release-promo.md 等，非本计划触及文件，会话间出现）——未触碰未提交，
PLAN-011 auto-lang 折叠协调事项另案在途。

## 待澄清事项

1. VM 轨 ui-gallery 解释器路径的实际形态（可用/占位/半可用）以 T6 实机
   结果为准记录——若完全占位且体验不可接受，可选后续给 pac 增按轨可见
   区分（本计划不做，552 语义 desktop_visible 是全局单值）。
2. storage 开关注入测试形态（T2 测试设计 2 的两种落地）实现期定，
   语义不变。
3. **画廊与示例的迁移归宿（auto-ui 拆仓议题，不在本计划裁定）**：两画廊
   与 examples/ui 属 auto-ui 项目资产，现居 auto-lang（cargo 管理约束）；
   未来 auto-ui 拆仓（用户举例 submodule 形态）时"这些示例也都要挪"
   ——用户原话"那时候这些示例也都要挪到 auto-lang 去"存在两读
   （① 随 auto-ui 项目迁出 auto-lang；② 钉回/留在 auto-lang 本仓、仅
   UI 栈代码迁出），以 auto-ui 拆仓立项时用户裁定为准。对 578 无机制
   差：两读下画廊与 examples/ui 均同树共置，兄弟锚定均成立；仅当拆仓
   后二者分置两树才需 storage `shell.apps.extra_dirs` 补注册（机制已备）。
   **时序裁定**（用户 2026-09-07，已同步登记 PLAN-579 待澄清⑥）：
   auto-ui 迁移讨论的前置 = ①虚拟桌面建仓结束（579 Stage B）+ ②新虚拟
   桌面跑起来 + ③能够展示两个 gallery——其中③由本计划在新桌面交付
   （执行链：579 → Stage B → **578** → auto-ui 独立讨论），满足后独立
   立项讨论，届时零返工跟随迁置。
4. **（2026-09-11 work 轮提出，同日 review 轮用户裁定销号）验收 3 后半
   处置**：裁定 **Amend 验收**——验收 3 已按裁定修正（见该条注记）；
   ext-materialize 立 KNOWN-DEBT 候选（候选登记：vue desktop-host 生成器
   对 `@/ext/` 导入 app 的物化能力，Plan 465 registered-limitation 扩展；
   ui-gallery 为首例消费方）。小注两则随复审记录归档：⚠ "no entry .at"
   文案对 render 过滤跳过同型复用（既有措辞混用，顺手修候选）；
   022-kanban 在 vue 轨系 v1 "needs API client" 既有跳过，验收 6 vue 腿
   自然成立。
