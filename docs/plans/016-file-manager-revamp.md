---
plan_id: PLAN-016
status: executing              # drafting → executing → execution_done → reviewed → archived（r2 Phase 2 执行中）
feature_name: file-manager-revamp
plan_revision: 2               # r1 初版契约；r2 增 Phase 2 UX 反馈批（9 项）
author: [agent]
created_at: 2026-09-14
updated_at: 2026-09-14

# /auto-plan:review 结束时填写：
supersedes_spec_components: []
new_spec_components:
  - auto-lang/docs/specs/auto-man/project.md §「pac.at opens 文件关联键（PLAN-016）」
  - auto-lang/schema/projection-protocol-v1.md §6 v1.7「open_with 动词」
touched_goals: []              # 引用 docs/specs/goals.md 的 GOAL-NNN

affects:
  - auto-lang/examples/ui/027-file-manager/**            # 主战场：app 重构
  - auto-lang/examples/ui/041-auto-edit/pac.at           # opens 声明 + 接收臂
  - auto-lang/examples/ui/041-auto-edit/src/front/editor_store.at
  - auto-lang/examples/ui/031-image-viewer/pac.at        # opens 声明 + 接收臂
  - auto-lang/examples/ui/031-image-viewer/src/**        # 路径接收臂（形态待 T-08 决策）
  - auto-lang/crates/auto-lang/src/ui/app_registry.rs    # opens 字段解析
  - auto-lang/crates/auto-lang/src/ui/session.rs         # OpenWith 动词执行臂
  - auto-lang/crates/auto-lang/src/ui/iced/renderer.rs   # 启动态注入 / 投递臂
  - auto-lang/schema/projection-protocol-v1.md           # 协议 v1.7
  - auto-os/docs/plans/016-file-manager-revamp.md

current_step: 10
total_steps: 18
---

# [PLAN-016] file-manager-revamp

> 跨仓计划：**主导仓 = auto-os**（桌面程序域，沿 PLAN-013/015 先例）。
> auto-lang 侧改动（examples/ui + crates + stdlib）按 AGENTS §1 经 auto-lang
> 仓 plan/工作区分账互链，或由 auto-plan:work 组内多仓模式执行。
> 执行 worktree 布局：`.wt/os-016/auto-os`（Plan 529 组目录）。

> **执行环境（2026-09-14 work 进入时记录）**：worktree
> `D:/autostack/.wt/os-016/auto-os`（branch `os-016-dev`，base = auto-os main
> `08f81b8`）＋ 依赖组 worktree `D:/autostack/.wt/os-016/auto-lang`
> （branch `os-016-dev`，base = auto-lang master `8c3b4db57`，沿 os-015 组
> 命名先例）。auto-lang 主检出存在他人未提交改动（i18n_lookup.rs 等）不纳入。
> PLAN-015 尚未 merge（worktree os-015 在途）——本计划 pac.at 触面
> （027/041/031 加 theme/opens 键）与其 title_zh 批量为不同行，接受合并期
> trivial 解冲突；PLAN-014 无 worktree，renderer.rs 无在途竞争。

## 0. 变更摘要

027-file-manager 是桌面注册表里完成度最低的 app：**硬编码深色 zinc 色板**（不
跟随桌面主题）、**emoji/unicode 充当图标**（未用框架 lucide 体系）、**弹层全部
堆叠在 view 树尾部**（三个固定坐标 popover + 手写 toast，未用 alert-dialog 等
正规组件）、**数据是硬编码 mock 平行数组**（假文件系统）、**与桌面零互操作**
（无真实"桌面"目录、无 open-with 文件关联）。本计划一次收口五项，分两个交付面：

| 面 | 内容 | 改动域 |
|----|------|--------|
| A. app 现代化（W1–W3） | 语义 token 双主题、lucide 图标、alert-dialog/toast 正规弹层 | 仅 027 源码（零框架改动，纯消费既有组件） |
| B. 真实化 + 互操作（W4–W5） | VM stdlib 真实目录浏览/文件操作；pac `opens` 关联键 + 桌面 `open_with` 动词 + auto-edit/image-viewer 接收臂 | 027 + 框架（app_registry/session/renderer）+ 协议 v1.7 + 两个目标 app |

## 1. 目标

1. **G1 主题跟随**：027 全量改用语义 token（`bg-background/text-foreground/
   bg-card/border-border/…`）并声明 `dark_mode` 状态变量——桌面设置切换深/浅
   （含 stella 浅色，PLAN-014 mock 已定稿浅色映射）时 027 即时跟随，无需重启。
2. **G2 图标正规化**：027 主视图全部 emoji/unicode 符号（`←→↑ 📁 📄 📂 ≡ ⊞
   ✓ ✕ ··· 及右键菜单`）替换为框架 `icon (name: "<lucide-kebab>")` 元素；
   与左侧 TreeView 组件族（已 lucide，`tree_icon.at`）对齐，VM/vue 双端同形。
3. **G3 弹层正规化**：新建/重命名/删除确认改 `alert-dialog`（025-sys-monitor
   形态）；操作反馈改框架 `toast()`（VM 窗口级堆叠悬浮层）；右键菜单不再使用
   固定假坐标，锚定到触发点。
4. **G4 真实目录**：浏览真实文件系统——home 派生快捷访问（含**桌面**）、
   `fs.read_dir` + `fs.metadata` 列目录、前进/后退/上级真实导航栈、地址栏、
   新建/重命名/删除/搜索（当前目录）、隐藏项开关、空目录/无权限错误态。
   mock 数据退役为 vue 轨回退（见 5.4 决策点 D-3）。
5. **G5 桌面互操作**：
   - 快捷访问含真实 **桌面（Desktop）** 目录（用户第 5 条"没有展示桌面"）。
   - 桌面级 `open_with` 机制：027 双击/右键可按扩展名关联用 **auto-edit 打开
     文本、image-viewer 打开图片**；目标 app 未运行则带路径启动、已运行则
     聚焦并送达路径。

**非目标**：
- 回收站真实现（删除进 OS 回收站；v1 直接删除 + 确认框，Trash 虚拟目录退役）。
- 磁盘容量统计（侧栏"存储空间 42.5/128 GB"假数据直接移除，不做 disk 总量 API）。
- 全文内容搜索 / 索引（v1 仅当前目录文件名过滤 + 有限深度 walk）。
- 虚拟桌面图标（storage `shell.desktop.icons`）与文件系统合一展示（桌面图标
  是策展清单非磁盘目录，合一属桌面程序后续设计，见待澄清③）。
- 文件关联的系统级注册（只做桌面注册表内关联，不写 Windows file association）。
- 027 收编为独立仓 / apps/ 容器臂（保持 examples/ui 原位，收编另行计划）。

## 2. 架构方案

三层递进，A 面不碰框架、B 面才动 crates：

```
[A] app 层（027 自身，纯消费既有框架能力）
    语义 token + dark_mode 声明   ← 桌面 SetTheme 全 App 回写链已有
                                   (renderer.rs:9707-9714 execute_set_theme)
    icon (name:) 字面量发射       ← lucide_generated.rs 1401 图标，双端同名同形
    alert-dialog / toast()        ← 025 形态 + vm_bridge __toast 重写

[B] 框架层（auto-lang crates，最小增量）
    app_registry.rs: parse_pac_fields 平铺键加 opens（"ext 列表"）
    session.rs:    DesktopCommand::OpenWith(String app_id, String path)
    renderer.rs:   启动态注入臂（boot state seed，dark_mode 同款注入点）

    open-with 数据流：
    027 双击/右键"打开方式"
      → __desktop_cmd 记录 "open_with\t<app-id>\t<path>"   （shell 同款上行通道）
      → session 执行臂（每周期 drain，session.rs:1201-1313 同族）
          ├─ 未运行：launch_app + boot seed state auto_open_path=<path>
          │          （注入点 = dark_mode boot seed 同位，renderer.rs:12169-12191）
          └─ 已运行：FocusWindow + 送达臂（决策点 D-1：write_state +
                     handler 直调（acceptance 频道 "same handler pipeline as
                     onclick" 泛化）｜运行中 app 聚焦 + toast 提示手动打开）
      → 目标 app（041/031）：Init 臂读 auto_open_path state → 走既有打开流
          （041 ActOpen 已有 AUTO_OPEN_PATH env 旁路同构逻辑，editor_store.at:233-256）

[C] 协议层
    projection-protocol-v1.md v1.6 → v1.7：新增 `open_with` 动词（§6）
```

真实目录数据层（027 内部，零框架改动）：

```
use auto.file / auto.fs / auto.env （VM stdlib，FFI 已实证：stdlib.rs shim 337/1093/8187）
  Env.get("USERPROFILE") → home
  fs.read_dir(dir)       → 文件名 JSON 数组（一次调用）
  fs.metadata(path)      → {size,is_dir,is_file,modified}（逐条补列，cap 500 条防宿主卡顿）
  file.at: create_dir/write_text/delete/copy/rename(fs)/exists/is_dir
  错误态：read_dir 空/异常 → 空态视图 + toast("无法读取目录")
```

## 3. 技术栈

- **app DSL**：auto-lang .at（VM 轨为桌面事实轨；pac `render: "vue"` 仅为 vue
  调试轨声明，桌面 launch 不读该字段——`renderer.rs:12447-12451` boot 不过滤 render）。
- **框架组件**：`icon`/`alert-dialog`/`popover`/`toast`（vue 侧 shadcn-vue 生成
  `ui_gen/vue.rs`，VM 侧 `aura_view_builder.rs` + renderer 原生模态/悬浮层）。
- **VM stdlib**：`stdlib/auto/file.at`、`stdlib/auto/fs.at`（Plan 250）、
  `stdlib/auto/env`（`Env.get`，041 在用）、`stdlib/auto/image.at`（Plan 547，
  031 接收臂候选）。
- **框架改动**：Rust（app_registry.rs 平铺键解析 / session.rs DesktopCommand /
  renderer.rs boot seed）。本计划改 auto-lang crates → 允许且需要在 auto-lang
  跑 `cargo t`（AGENTS §3 门档只禁"不改 crates 的工作"）。
- **测试**：`027/tests/desktop_mcp.py`（autoui MCP，VM 模式）扩展 + acceptance
  频道（`AUTOUI_ACCEPTANCE=1`，mcp_server.rs:962）做 open-with 端到端。

## 4. 需求分析与背景调查

### 4.1 需求来源与授权记录

- **需求**：用户 2026-09-14 实机截图反馈五项（原文归纳）：①浅色主题下显示与
  深色相同；②图标非 lucide、系 unicode 直显；③弹框消息堆叠页面底部（"写得
  比较早，没用 alert-dialog 等正规组件"）；④数据虚假、未接真实目录；⑤未与
  虚拟桌面结合（无"桌面"、不能用 auto-edit/image-viewer 打开关联文件）。
- **授权范围**：起草本改进计划（auto-plan:new）；执行另行 handoff（next: work）。
- **允许仓库/动作**：auto-os（计划/文档）、auto-lang（只读调查；执行期 crates/
  examples/stdlib 修改在本计划范围内）。用户未给预算/自动继续限制——未发明。

### 4.2 决定性架构事实（调查实证）

1. **桌面 app 全走 VM 轨**：pac `render:` 只是前端目标声明，桌面 launch =
   `build_dynamic_component` 进程内装载（`session.rs:2396-2440`；
   `renderer.rs:12447-12451`）。front_port 在桌面里不使用。
2. **主题机制**：宿主 `SetTheme(bool)`/`SetThemeName` 动词 → `execute_set_theme`
   （`renderer.rs:9662-9715`）→ 对声明了 `dark_mode` 状态变量的 app 逐个
   `write_state("dark_mode", …)` + view_dirty；boot 期从 `AUTO_UI_THEME` 播种
   （`renderer.rs:12169-12191`）。语义 token 单一事实源 =
   `crates/auto-lang/src/design_tokens/registry.rs`（五内置主题，stella 为宿主
   主题；浅色映射见 PLAN-014 mock 定稿：背景 #F5F1E8 / card #FBF8F2 /
   foreground #2A2723 / primary #6466F1）。
   **027 现状**：无 `dark_mode` 声明，`app.at:225` 起约 20 处 `zinc-xxx` 字面色
   （:225/230/231/240/263/282/301/326/336/344/363/389/405/409/470/516/546/578/
   610/632），故桌面切浅色时 027 纹丝不动（问题①根因）。
   **参照**：015-notes（`app.at:25` `var dark_mode bool = true`、:96-99 Toggle
   handler、全篇 `bg-background/text-foreground/text-muted-foreground/border-
   border`）；auto-os `apps/025-sys-monitor/src/front/app.at:21,69`；shell 自身
   `shell.at:155,166`。
3. **图标体系**：框架 icon = `icon (name: "<lucide-kebab>")` 元素；VM 轨
   `aura_view_builder.rs:6296-6324` → `View::Image { src: "lucide:{name}" }`，
   vue 轨 `ui_gen/vue.rs:6325-6380` → lucide-vue-next 组件；数据表
   `lucide_generated.rs`（1401 图标，双端同名同形）。
   **约束**：icon name 须字面量（动态名 vue 轨落 Circle 占位）——按类型分派
   用字面量分支发射，`tree_icon.at:7-54` 已是现成范式。
   **027 现状**：主视图 `←→↑`（:238-252）、`📁📄`（:395-397,473-477）、`📂`
   （:454,503）、`≡⊞`（:288-292）、`✓✕`（:407-411）、`···`（:442）、右键菜单
   emoji（:547-573）；仅 TreeView 组件族已 lucide（问题②根因）。
4. **弹层组件**：`alert-dialog` 全家双轨齐备（vue `ui_gen/vue.rs:12450-12500`；
   VM 模态 `ui_gen/rust.rs:3655-3768`，ESC/外点语义 :3613）——**VM 轨可用性的
   实证 = 025-sys-monitor**（`processes.at:156-164`，桌面 V2 验收 13/0 基线）。
   `toast()` 在 VM 轨被 `vm_bridge.rs:489-495` 重写为 `__toast` 状态赋值，由
   renderer 渲染窗口级堆叠悬浮层（`renderer.rs:6865-7153`）。
   **027 现状**：三个 popover 全部挂在 view 树尾部 + 固定假坐标（右键菜单
   :545-574 写死 450/200；新建 :577-606 固定 380/220；删除 :609-627 固定
   380/230）；toast 是手写条件渲染 row（:630-639）（问题③根因）。
5. **FS 能力**：stdlib `file.at`（read_text/write_text/exists/delete/copy/size/
   create_dir/is_dir）+ `fs.at`（Plan 250：read_dir 返回文件名 JSON 数组 /
   metadata 返回 {size,is_dir,is_file,modified} JSON / rename/ext/stem/filename/
   parent/join/walk_files/canonical）；VM FFI 实证 `stdlib.rs:337-338,1093-1140,
   8187-8188,8909-8917`。宿主对 app 无 fs 门控（app 在宿主进程 VM 内，天然持
   stdlib 能力）。真实目录扫描先例 = kanban `lang_plans.at:33-47`（fs.read_dir
   扫 docs/plans）；`dialog_open`（原生文件对话框，native_catalog.rs:52）041 在用。
6. **互操作现状**：`__desktop_cmd` 是 app→宿主唯一上行通道（`"verb\targ"` 记录，
   宿主每周期 drain 执行，`session.rs:1201-1313`）；动词全集无 open-with。
   注册表 `parse_pac_fields` 只认平铺键（`app_registry.rs:168-189`），
   `AppRegistryEntry` 无 mime/extension 槽——**文件关联字段不存在**。
   宿主→app 已有两种投递原语：`write_state`（dark_mode 回写同款）与
   acceptance 频道 handler 直调（`mcp_server.rs:962`："call a privileged app
   handler directly … same handler pipeline as onclick"，现限 shell|settings|
   notification|launcher 四特权 app + AUTOUI_ACCEPTANCE=1 门）。
   **041-auto-edit** 已有路径注入旁路：`ActOpen` 读 `Env.get("AUTO_OPEN_PATH")`
   （`editor_store.at:233-256`，env 缺省走 dialog_open）——接收臂有同构先例。
   **031-image-viewer**：front `use back.api: open_file/…`（`app.at:4`），back
   为 `api: "rust"` HTTP 形态（back_port 8031），pac 无 `back: { project }` →
   桌面 in-process launch 时 back_root 缺席（`session.rs:2417-2426` 只认该
   字段），`back.api` 落桩——**031 在桌面 VM 轨的图片打开链路当前不完整**，
   T-08 需先补（决策点 D-2）。
7. **"桌面"目录**：桌面图标是 storage 策展（`shell.desktop.icons` +
   `renderer.rs:11428-11492` 注入），非磁盘目录；用户要的"展示桌面"按真实
   Windows Desktop 目录落（快捷访问项），虚拟图标合一列入非目标。
8. **在途计划关系**：
   - **PLAN-015**（reviewed，next=merge）：pac.at 四名称契约（name/exe_name/
     title/title_zh），批量触所有 pac.at。本计划新增 `opens` 键与四名称契约
     正交（同为平铺键），**排 015 merge 之后开工**避免同文件并发。
   - **PLAN-014**（drafting）：触 renderer.rs/desktop_config.rs（主题/UX 面，
     与本计划触点相邻不同域）。建议 worktree 串行（014 先或错峰），冲突面
     仅 renderer.rs 注入点附近。

### 4.3 027 现状结构（观察面）

| 文件 | 行数 | 内容 |
|---|---|---|
| `pac.at` | 17 | name/render:vue/front_port:4027/icon:folder/title/desktop:true；**无 theme/accent/opens** |
| `src/front/app.at` | 1251 | mock 平行数组（:94-185 item_* 20 条 + :187-220 fmNodes）、view（:223-641）、handlers（:643-1250） |
| `src/front/components/filetree.at` | 81 | 快捷栏（已 lucide） |
| `src/front/components/treeview.at` | 70 | TreeView（已 lucide） |
| `src/front/components/tree_icon.at` | 70 | 有界 lucide 调色板（字面量分支范式，PLAN-614） |
| `src/front/components/tree_util.at` | 216 | toggle_id 等纯函数 |
| `SPEC.md` / `tests/desktop_mcp.py` | 119 / 491 | app 规格 / VM 模式 MCP 测试 |

注册表 id = 目录名 `027-file-manager`（`app_registry.rs:28`）；`desktop: "true"`
使其进 launcher/桌面/dock 展示清单。

## 5. 详细设计

### 5.1 W1 主题（→ G1，T-01）

- `app.at` 增加 `var dark_mode bool = true`（真值随桌面，boot 播种覆盖）。
- zinc → 语义 token 映射（全视图逐处替换，含 filetree/treeview 复核）：

| 现值（例） | 替换 |
|---|---|
| `bg-zinc-950 text-zinc-100`（:225 根） | `bg-background text-foreground` |
| `bg-zinc-900` 面板/行卡 | `bg-card`（半透明处 `bg-card/50`，025 形态） |
| `border-zinc-800` | `border-border` |
| `text-zinc-400/500` 次级文字 | `text-muted-foreground` |
| `hover:bg-zinc-800` | `hover:bg-accent` |
| 选中高亮 `bg-zinc-800` | `bg-accent`（或 `bg-primary/10`，随 mock 校准） |
| 蓝/紫主按钮 | `bg-primary text-primary-foreground` |
| 危险操作红 | `bg-destructive text-destructive-foreground` |

- `pac.at` 补 `theme: "dark"`（vue 轨缺省面；沿 015-notes :7）。
- 验证锚点：桌面 SetTheme(false) → 027 窗口随写（VM 回写链），双主题截图。

### 5.2 W2 图标（→ G2，T-02）

替换表（均为字面量 name；按扩展名的文件图标走 tree_icon.at 字面量分支范式扩翼）：

| 位置 | 现值 | lucide name |
|---|---|---|
| 导航 后退/前进/上级 | `←` `→` `↑` | `arrow-left` `arrow-right` `arrow-up` |
| 面包屑/目录行 | `📁`/`📂` | `folder` / `folder-open` |
| 文件行 | `📄` | 按扩展名分支：`file-text`(.md/.txt) `file-code`(.toml/.json/.rs/.at) `file-image`(.png/…) `file`（兜底） |
| 列表/网格切换 | `≡` `⊞` | `align-justify` `layout-grid`（shell.at:397,405 同款） |
| 隐藏项开关 | 文字按钮 | `eye-off` / `eye` |
| 确认/取消 | `✓` `✕` | `check` `x` |
| 行操作 `···` | `···` | `ellipsis`（或 `more-horizontal`，取 registry 存在者） |
| 右键菜单 | `📂✏️📄✂️📋🗑️` | `folder-open` `pencil` `file-plus` `scissors` `clipboard` `trash-2` |
| 新建文件夹/文件 | `+ 文件夹`/`+ 文件` | `folder-plus` `file-plus`（041 actions 同款） |

### 5.3 W3 弹层（→ G3，T-03/T-04）

- **alert-dialog 化**（025 `processes.at:156-164` 形态，state 驱动 open）：
  删除确认（`confirm_delete_open`）、新建文件夹、新建文件、重命名 四处；
  `alert-dialog-cancel/action` 接既有 handler（`ConfirmDelete` 等，:643-1250
  的业务逻辑保留，仅换壳）。固定坐标 popover（:545-627 三个）整体退役。
- **toast()**：删除/新建/重命名成功、目录读取失败、无效路径等反馈改框架
  `toast("…")`（VM `__toast` 链）；手写 toast row（:630-639）+ `DismissToast`
  退役。
- **右键菜单**：保留 popover 原语（Plan 422 支持坐标锚），**决策点 D-1**：
  - 首选：onclick/oncontext 事件若携带指针坐标 → 真实锚定（调查 `.at` 事件
    是否暴露坐标；tree/桌面拖拽链有坐标通道可考）。
  - 回退：锚定不可得则右键菜单降为**选中行上下文操作条**（行内 `ellipsis`
    按钮锚定 popover）或居中 modal——执行期定，不改计划契约。
- 决策工件：D-1 结论写入 `docs/plans/evidence/016/d1-context-menu.md`。

### 5.4 W4 真实目录（→ G4，T-05/T-06）

**store 数据层**（替换 :94-220 mock）：

```
var cwd str = ""            # 当前目录（canonical 绝对路径）
var home str = ""           # Env.get("USERPROFILE")，boot 解析
var entries_name []str      # read_dir JSON 解出（cap 500）
var entries_meta []str      # 平行数组：JSON {size,is_dir,modified} 逐条 metadata
var hist_back []str / hist_fwd []str   # 真实导航栈
var show_hidden bool = false
var sel_path str / ctx_path str        # 选中/右键目标（真实路径）
```

- **加载流**：`Navigate(dir)` → `fs.canonical` → `fs.read_dir` → JSON 解析
  （`auto.json`，kanban 同款 use）→ 逐条 `fs.metadata`（cap 500，超出 toast
  提示"仅显示前 500 项"）→ 排序（目录先、名称次）→ 过滤 hidden（`.` 前缀）
  → 状态写入。VM FFI 同步调用，cap 防宿主进程卡顿。
- **快捷访问**（filetree.at 改真实）：主目录/桌面/文档/下载/图片/音乐 =
  home join 标准名（Windows：Desktop/Documents/Downloads/Pictures/Music），
  `fs.exists` 门控显隐；**回收站项移除**（非目标）。
- **地址栏**：显示 cwd，可编辑回车跳转（canonical 校验失败 toast）。
- **操作映射**：新建文件夹 `file.create_dir`；新建文件 `file.write_text(p,"")`；
  重命名 `fs.rename`；删除 `file.delete`（**决策点 D-4**：目录删除 stdlib 无
  rmdir 递归——空目录 delete 可用，非空目录 v1 拒绝 + toast"仅支持删除空目录
  与文件"，或 fs.walk 逆序删除；执行期按 stdlib 实测定）。
- **搜索**：当前目录 `fs.read_dir` 结果文件名 contains 过滤（现有搜索框语义
  不变）；可选加 `fs.walk_files` 一层深度开关。
- **底栏**：`N 个项目 | 选中 M 项 X KB`（真实聚合）；侧栏假容量计（42.5/128GB）
  移除。
- **错误态**：read_dir 失败/无权限 → 空态图标 + `toast("无法读取目录: <path>")`，
  cwd 回退上一有效目录。
- **决策点 D-3（vue 轨口径）**：VM stdlib 在 vue 轨需 ts_adapter 桥，存在性
  未证。执行期调查：桥齐 → vue 轨同启真实 FS；桥缺 → vue 轨保留 mock 数据集
  并显式标注"演示数据"徽标（桌面事实轨 = VM，不因 vue 回退阻塞 G4 验收）。
  工件：`docs/plans/evidence/016/d3-vue-fs.md`。

### 5.5 W5 桌面互操作（→ G5，T-07/T-08/T-09）

**pac `opens` 关联键**（平铺键，`parse_pac_fields` 零成本扩展）：

```
# 041-auto-edit/pac.at
opens: ".txt,.md,.at,.au,.rs,.json,.toml,.yaml,.yml,.log"
# 031-image-viewer/pac.at
opens: ".jpg,.jpeg,.png,.webp,.gif,.bmp"
```

- `AppRegistryEntry` 增 `opens: Vec<String>`（小写规范化）；027 侧注册表投影
  暴露 `{ext → app-id}` 派生（宿主注入平行串，沿 `__desktop_cells` B12 规避
  先例：**派生面由宿主算**，shell/app 不跨列表反查）。
- **关联解析（027 内）**：双击文件 → 按扩展名查关联 → 命中即 `open_with`；
  未命中 → "打开方式"选择 popover（列注册表 desktop app，记住选择写
  `shell.open_with.overrides` storage——可选增强，v1 可不做记忆）。
- **DesktopCommand::OpenWith(String, String)** + 文本协议
  `open_with\t<app-id>\t<path>`（path 单行约束，同 Notify msg 先例；协议 v1.7）。
- **session 执行臂**：
  1. 注册表校验 app-id 存在（未知 → 桌面 toast）。
  2. **未运行**：`launch_app` 扩展带 `init_state: Vec<(String, String)>`（或
     `launch_app_with(name, open_path)` 变体），boot 注入点写 state
     `auto_open_path=<path>`（与 dark_mode boot seed 同位同法）。
  3. **已运行**（**决策点 D-2**，二选一，执行期定）：
     - a. 聚焦 + `write_state("auto_open_path", …)` + 泛化 acceptance 频道
       handler 直调（`mcp_server.rs:962` "handler" 动作放宽一档：允许宿主对
       任意 desktop app 调 `OpenRequest` 空参 handler——"same handler pipeline
       as onclick" 既有管线，放宽的是 allowlist 不是机制）；
     - b. v1 收缩：聚焦 + 桌面 toast"已在 <app> 中打开：<file>"但不断流，
       仅未运行路径送达（先交付 G5 主链，D-2a 作为 T-07 内可选项）。
  - 工件：`docs/plans/evidence/016/d2-delivery-arm.md`。
- **目标 app 接收臂**：
  - **041-auto-edit**：`editor_store.at` 增 `var auto_open_path str = ""`；
    Init 臂非空 → 走既有 ActOpen 主体（读文本入 tab）；与 AUTO_OPEN_PATH env
    旁路并存（env 优先级更高，测试语义不变）。声明 `opens` 键。
  - **031-image-viewer**（**前置**）：pac 无 `back: { project }` → 桌面 VM 轨
    back.api 落桩，打开链路不完整。两个候选（执行期调查定，写入 D-2 工件）：
    - i. front 直调 `auto.image` stdlib（`open_session/request_view` 等
      Plan 547 面——若 `#[vm]` 可用则删 back 依赖最小）；
    - ii. pac 补 `back: { project }`/daemon 声明走既有 cdylib/ensure 链。
    接收臂：`var auto_open_path str` + Init/`OpenRequest` → open_session(path)。
- **文件类型→图标/关联兜底**：无关联扩展名双击 → toast + 打开方式菜单。

### 5.6 规范增量

| delta_id | 增/改/退 | 目标 | before/after 规则 | 依据 | 验收 |
|---|---|---|---|---|---|
| SD-01 | add | auto-lang/schema/projection-protocol-v1.md §6（v1.7 节） | before：`__desktop_cmd` 动词全集无 open-with / after：新增 `open_with\t<app-id>\t<path>`（app-id 必须在注册表；path 单行），宿主执行臂 = 校验→未运行 seed 启动/已运行送达（D-2 定案回填） | G5 主链，Notify 单行先例 | AC-08/09 |
| SD-02 | add | auto-lang/docs/specs/auto-man/project.md §「pac.at opens 文件关联键（PLAN-016）」 | before：pac 平铺键无文件关联语义 / after：`opens: "<逗号扩展名列表>"`（小写、点前缀、registry 解析为 Vec、AppRegistryEntry.opens），未声明 = 不参与关联解析；与 PLAN-015 四名称契约正交并存 | G5 关联解析唯一事实源 | AC-08/10 |
| SD-03 | modify | 027 `SPEC.md`（app 本地规格） | before：mock 数据/固定坐标弹层/无主题声明 / after：真实 FS 数据层（cap 500/错误态）、alert-dialog+toast、dark_mode+语义 token、opens 关联消费、D-3 vue 轨口径 | G1–G4 落地事实 | AC-01–07 |

（无 retire 项；auto-os 侧 `.autoos/specs.json` 台账条目随 merge 沉淀，drafting
期不写。）

## 6. 测试设计

1. **027 桌面 MCP 套件**（`tests/desktop_mcp.py` 扩展，VM 模式）：
   - testdata：`tests/testdata/` 预置 `notes.txt / photo.png / config.toml /
     子目录/`（合成语料，沿 apps 测试约定）——**只对 testdata 根断言**，测试
     不触碰用户真实目录（导航初值指向 testdata，破坏性操作只对临时副本）。
   - 断言组：目录列出（名称/大小/类型列）→ 进入子目录 → 后退/前进/上级 →
     新建文件夹/文件 → 重命名 → 删除（alert-dialog 出现→确认）→ 搜索过滤 →
     隐藏项开关 → 空目录错误态。
2. **主题**：acceptance 频道 `bus` 动词 `set_theme\t0/1`（既有消费臂）→
   截图对比 027 双主题（浅色断言背景非深色 token，PLAN-014 浅色映射口径）。
3. **open-with 端到端**：`AUTOUI_ACCEPTANCE=1` 宿主 → MCP 驱动 027 对
   testdata/notes.txt 触发打开 → 断言 041 窗口出现且 tab 标题/内容含语料；
   对 photo.png → 031 会话打开（D-2 定案后回填断言细节）。
4. **vue 轨冒烟**：`auto run`（027）截图——mock 回退或真实 FS（D-3 定案）
   均可接受，断言不回归（可渲染、无 console 异常）。
5. **框架单测**（auto-lang crates）：`opens` 解析（正常/空/缺省）、
   `OpenWith` 文本协议解析（含未知 app 拒绝）、boot seed 注入。`cargo t -p
   auto-lang` 目标用例，不跑全站 docs_gen。

## 7. 验收标准

| ID | 可观察行为 | 验证方法 |
|---|---|---|
| AC-01 | 桌面切浅色（os-config/`set_theme`）后 027 全窗随变为浅色 token；切回深色复原；重启后跟随会话主题 | MCP `set_theme` 双向 + 双主题截图对照 |
| AC-02 | 027 主视图无 emoji/unicode 图标；导航/视图切换/右键菜单/行图标均为 lucide 同名形（与 vue 轨一致） | MCP snapshot 扫描 027 视图树无 emoji 码点；双端截图 |
| AC-03 | 新建/重命名/删除操作出现居中模态（alert-dialog），ESC/取消/确认语义正确；页面底部无堆叠弹层 | MCP 快照 + 交互断言 |
| AC-04 | 操作反馈以窗口级 toast 呈现（自动消退）；无手写 toast div | 交互断言 + 截图 |
| AC-05 | 027 启动后快捷访问为真实 home 派生目录（含桌面/文档/下载/图片/音乐，存在性门控）；点击进入真实目录且列出的条目与 `fs.read_dir` 一致 | 对 testdata 根比对 MCP 列表 vs 磁盘 |
| AC-06 | 前进/后退/上级与地址栏跳转在真实路径上正确；无权限/不存在目录显示空态 + toast，不崩溃 | testdata 序列断言 |
| AC-07 | 新建/重命名/删除真实落盘（testdata 副本上验证）；隐藏项开关生效；底栏计数/选中体积真实 | 磁盘状态断言（测试前后清理） |
| AC-08 | `open_with\t<app>\t<path>` 动词：未知 app 拒绝（toast）；合法 app 未运行→带路径启动；协议 v1.7 成文 | crates 单测 + 协议文档 diff |
| AC-09 | 027 双击 testdata/notes.txt → auto-edit 启动（或聚焦）且打开该文件内容 | acceptance 频道端到端断言 |
| AC-10 | 027 双击 testdata/photo.png → image-viewer 打开（D-2 定案形态）；无关联扩展名出现"打开方式" | 端到端 + 交互断言 |
| AC-11 | `desktop_mcp.py` 全套通过（0 退出），含新增断言组 | 测试报告 |
| AC-12 | vue 轨 027 可渲染不回归（mock 回退或真实 FS，D-3 口径），双端截图归档 evidence | 截图对照 |
| AC-13 | SPEC.md 与协议 v1.7/opens 契约文档三处一致；evidence 含 D-1..D-4 决策工件 | 文档走查 |

## 8. 执行步骤

> 原子上子任务；每步完成后在任务行下追加 `[✅ 已完成] <证据路径/命令输出摘要>`。
> 执行域标注：〔os〕= auto-os 仓；〔lang〕= auto-lang 仓（worktree
> `plan-016-dev`，或组内多仓模式）。T-01–T-06 为 A/B 面 app 层（零 crates），
> 可先行；T-07 起 B 面框架层。

- **T-01 〔lang〕主题 token 化**（G1/AC-01）
  文件：`examples/ui/027-file-manager/src/front/app.at`（5.1 映射表逐处）、
  `pac.at`（+theme）、`components/filetree.at|treeview.at` 复核。
  依赖：无（建议 PLAN-015 merge 后开工， pac 文件并发面）。
  验证：iced 轨宿主实机 SetTheme 双向 + 截图 → evidence/016/theme-{dark,light}.png。
- **T-02 〔lang〕lucide 图标替换**（G2/AC-02）
  文件：`app.at`（5.2 表全部位置）、`components/tree_icon.at`（扩文件类型分支）。
  依赖：T-01（同文件串行）。
  验证：MCP snapshot 无 emoji 码点；`auto run` vue 轨 icon 渲染冒烟。
- **T-03 〔lang〕alert-dialog 化**（G3/AC-03）
  文件：`app.at` :545-627 三个 popover → alert-dialog（025 形态）；handler
  保留。依赖：T-01。
  验证：MCP 交互断言（ESC/取消/确认三路径）。
- **T-04 〔lang〕toast + 右键菜单锚定**（G3/AC-04，含 **D-1** 决策工件）
  文件：`app.at` :630-639 → `toast()`；右键菜单坐标臂按 D-1 结论落地。
  依赖：T-03。验证：toast 断言 + evidence/016/d1-context-menu.md。
- **T-05 〔lang〕真实 FS 数据层**（G4/AC-05/06，含 **D-3** 决策工件）
  文件：`app.at`（store :94-220 替换 + Navigate/加载流/地址栏/快捷访问真实
  化/filetree.at）；`use auto.file/auto.fs/auto.env/auto.json`。
  依赖：T-04。验证：desktop_mcp testdata 断言组；d3-vue-fs.md。
- **T-06 〔lang〕文件操作 + 搜索 + 隐藏项**（G4/AC-07，含 **D-4** 目录删除
  口径）依赖：T-05。
  验证：testdata 副本落盘断言（前后清理钩子）。
- **T-07 〔lang·crates〕opens 字段 + OpenWith 动词 + 投递臂**（G5/AC-08，
  含 **D-2** 决策工件）
  文件：`crates/auto-lang/src/ui/app_registry.rs`（opens 解析 + entry 槽 +
  027 投影派生面）、`session.rs`（DesktopCommand::OpenWith + 文本协议 +
  执行臂 + launch_app init_state 变体）、`iced/renderer.rs`（boot seed 注入
  臂 + 已运行送达臂按 D-2）、`schema/projection-protocol-v1.md`（v1.7）。
  依赖：T-05（027 侧消费面就绪）；与 PLAN-014 错峰。
  验证：`cargo t -p auto-lang`（opens/协议/seed 用例）；evidence/016/
  d2-delivery-arm.md。
- **T-08 〔lang〕目标 app 接收臂**（G5/AC-09/10 前半）
  文件：`041-auto-edit/pac.at`（opens）+ `src/front/editor_store.at`（Init 臂，
  env 旁路并存）；`031-image-viewer/pac.at` + `src/**`（先补 D-2 前置：
  image stdlib front 直调 or back 声明；再落 auto_open_path 接收臂）。
  依赖：T-07。验证：acceptance 端到端（notes.txt → 041）。
- **T-09 〔lang〕027 关联消费 UX**（G5/AC-10）
  文件：`app.at`（双击关联解析 + 打开方式 popover + `open_with` 记录）。
  依赖：T-07/T-08。验证：端到端两例 + 无关联分支。
- **T-10 〔lang〕测试收口**（AC-11/12）
  文件：`027/tests/desktop_mcp.py`（testdata 组 + open-with 组 + 主题组）、
  `tests/testdata/**`。依赖：T-09。
  验证：`python tests/desktop_mcp.py` 0 退出；双端截图 → evidence/016/。
- **T-11 〔os+lang〕文档与台账收口**（AC-13）
  文件：`027/SPEC.md`（SD-03）、auto-lang `docs/plans/INDEX.md` 互链行、
  本仓计划回填执行证据；merge 阶段沉淀 `.autoos/specs.json`（SD-01/02）。
  依赖：T-10。

## 9. 复审记录

### 2026-09-14 tf/tv 回归补跑 ✅（绿）

- `cargo tf`：**3555 tests run: 3555 passed**（44.7s）；`cargo tv`：
  **3701 tests run: 3701 passed**（23.5s）——worktree os-016-dev 全量
  （含 PLAN-016 全部改动）。
- 前置：master 并入 os-016-dev（merge a39f2c6b9，带 PLAN-018 assets
  pin/金样同步——旧 assets 曾致 desktop 金样对拍失败）；ffi_dual_019
  以 `-E` 排除——**master 基线同失败**（auto-cache 依赖指纹 wide/narrow
  变体未触发重建，i32 截断 705032704≠5000000000，nightly 门控测试），
  属基线债务非本计划回归。
- 日志：auto-os tmp/rev-tf-g4.log、rev-tv-g4.log。

### 2026-09-14 桌面内实机验收补充（R7 构建宿主实测）

- **AC-01 ✅**：桌面切浅色（set_theme 0）后 027 完整跟随浅色（stella
  light 全 token：工具栏/侧栏/行/状态栏），真实目录 67 项同屏——
  evidence/016/ac01-light-desktop.png。
- **AC-10 桌面腿 ✅**：bus 注入 open_with 031-image-viewer + photo.png →
  031 启动聚焦、缩略图/大图渲染（testdata 1x1 红图按缩放呈现）——
  **031 back.api 桌面轨可用性疑虑解除**（D-2 残留闭合）。031 为浅色
  主题（pac theme 既有）。
- 证据：ac01-light-desktop.png、ac10-image-open-desktop.png。

### 2026-09-14 修复轮 7（R7）：open_with 桌面可达性分流（用户实机反馈）

- 现象：独立窗口（auto run -r vm）双击 .jpg 无反应——命中 image-viewer
  关联走 open_with，但 `__desktop_cmd` 需虚拟桌面宿主排空，独立窗口无人
  排空 → 静默无效。
- 修复：ui_desktop 宿主启动设 `AUTO_UI_IN_DESKTOP=1`（App 以 Env 探测
  桌面在场）；027 OpenItem 分流——`关联命中 && in_desktop` → open_with；
  否则（无关联 / 独立窗口）→ `process.spawn(cmd /c start path)` 走
  Windows 关联默认程序。
- 语义：open_with 的能力边界 = 虚拟桌面内；桌面外文件管理器一律系统
  默认程序（与用户预期一致）。
- **判定序（用户 2026-09-14 确认）**：打开文件时优先在当前"环境"里找
  关联程序（虚拟桌面在场 = 环境提供 image-viewer/auto-edit 等，以
  `AUTO_UI_IN_DESKTOP` 标记 + pac `opens` 关联为准）；环境里没有 → 系统
  打开方式（cmd start 关联 verb）。四象限：桌面内+有关联 → open_with；
  桌面内+无关联 → 系统；独立窗口+有关联 → 系统（环境未提供，通道不可
  达）；独立窗口+无关联 → 系统。
- 残留（债务）：宿主执行臂 launch 失败（如目标 app 装载异常）目前仅
  toast 反馈，无回执通道让 027 降级到系统打开——需 open_with 回执面，
  记框架债 F-6。

### 2026-09-14 修复轮 6（性能）：选中/悬停卡顿

- 现象：点击选中 ~0.3s 才高亮，不流畅。根因 = 两次叠加：①每次状态变更
  全量重建视图树（67 行 × 行内 popover 菜单 ≈ 千级控件，VM 动态视图无
  diff），debug 构建放大 10-50x；②R5 的 hover 高亮为事件驱动
  （mouseenter/leave 每次跨行切换也触发全量重建）。
- 处置：a) 行内菜单瘦身（粘贴/divider 移除——状态栏已有粘贴入口）；
  b) 换 `CARGO_PROFILE_DEV_OPT_LEVEL=2 DEBUG=0` 优化 dev 构建（运行时
  预期降一个数量级）；c) MouseArea 容器不支持 hover: 变体对（renderer
  build_container 单样式），事件驱动 hover 为现框架下唯一解 → 记 F-5
  框架债（MouseArea hover: 变体支持后可零重建 hover）。
- 残留：若优化构建后仍有可感卡顿 → 视图 diffing 属框架级工作，另立。

### 2026-09-14 修复轮 5 补充：选中行纵向居中

- 内行 `h-full`（框架 Fill 语义）未使内容居中——改显式 `h-11` 与
  mouse-area 同高，items-center 生效。用户运行实例已含此修复（重选可见）。

### 2026-09-14 修复轮 5 work 记录（行交互/菜单实机反馈）

- `code_commit`（auto-lang os-016-dev）：`35712be35`。
- **popover 根因闭合（F-2 细化）**：first-child 被当锚件就地渲染不进菜单
  ——"打开"消失 + 菜单锚错位的共同根因；trigger/content 子标签强制拆分
  修复（shadcn 规范形态），菜单恢复 Open 为首项、锚 = ··· 钮。
- 行 hover 高亮（mouseenter/leave 显式驱动）+ 内行 h-full 居中修复。
- 文件打开系统兜底：无关联 → `process.spawn(cmd /c start path)`。
- F-4 新增：多选 ctrl/shift 需 click 事件修饰键面（框架债）。
- 待用户实机验收：右键菜单位置与形态、双击/Enter 打开、hover 手感、
  点空白编辑路径。

### 2026-09-14 修复轮 4 work 记录（第二批反馈 8 项）

- `code_commit`（auto-lang os-016-dev）：`ba2f79360`（T-19/20/21 单提交）。
- 迭代单根元素纪律入档：VM `for` 迭代多根 → 纵向堆叠（chevron 掉行根因）；
  popover 不可入 mouse-area 子树（F-2 泄漏面）；两者均已写提交注记。
- 实证：面包屑单行内联 chevron（p2-breadcrumb-inline.png）；列表行无泄漏、
  无操作列、名称纯文本（p2 列表态截图同版式）。行交互（单击选中/双击打开/
  整行右键/Enter）结构就位，交互手感留用户实机确认。

### 2026-09-14 修复轮 3（面包屑 Win11 风格 + 编辑入口实钮）

- crumbs 加 chevron-right 分隔（末层无）；非 hover 态纯文本、hover 显按钮
  底（用户要求）；左侧 folder-open 图标升级为「编辑路径」显式入口
  （onclick AddrEdit，值=完整路径）。
- 发现：mouse-area 空白区点击不可靠（MCP/实机均未触发 AddrEdit）——入口
  以显式按钮为准，mouse-area 保留为增强。
- commit f5dc845d9；证据 p2-breadcrumb-win11.png（chevron 分隔 + 单行
  对齐实机截图）。

### 2026-09-14 Phase 2（r2）work 记录：UX 反馈批 9 项全实施

- `code_commit`（auto-lang os-016-dev）：`beb1ad124`（T-12..T-18 单提交）。
- AC-15..22 全部实机验证：列表/网格双态截图（p2-list-final.png、
  p2-grid-final.png）；双击打开结构就位（ondblclick，交互留用户实机）。
- F-1（中文字体家族）/F-2（popover closed 泄漏）记框架债；F-2 的用户可见
  面已随卡内 popover 移除消除。
- tf/tv：按用户指示暂缓（首次失败 = D 盘瞬时空满 + ffi oracle 二进制
  缺失，oracle ×5 已补构建；重跑待用户示下）。
- 附加交付：盘符切换（侧栏「此电脑」组，C..Z exists 探测，用户修订需求）。

### 2026-09-14 修复轮 2（用户实机反馈：重复项 + 侧栏形态）

- **重复项根因**：T-05 排序块的选择排序交换漏回写——`out[filled] = out[best]`
  未先把旧 `out[filled]` 存走，造成条目复制 + 丢失（实机 AppData×4/
  Documents×2）。重写为直接对象比较 + 真三行交换，平行键数组退役。
- **侧栏换装**（用户裁定）：手搓 button 列 → Sidebar 组件族（015-notes
  NavTree 同款：sidebar_provider/header/content/group/menu/menu_button
  active 高亮）。
- Tick 错峰 20→8（引导加载态 5s→2s）。
- 实证：实机截图无重复（AppData/Documents 各一次，字母序）+ Sidebar
  渲染 active 高亮 → evidence/016/repair-sidebar-dedup.png；
  commit b70185cd8。
- 注：列表中 `Application Data`/`Cookies`/`Local Settings` 等为 Windows
  用户目录真实 junction 条目（非重复 bug）；Explorer 默认隐藏，本应用
  stdlib 无属性面暂不区分，已记 SPEC。

### 2026-09-14 work 执行 handoff（T-01..T-11 全量执行；10/11 步达成）

- `stage: work` | `plan_id: PLAN-016` | `plan_revision: 1`
- `outcome: blocked`（保持 executing）——**唯一残留 = T-10 全绿整跑**被
  框架级 MCP 服务器线程偶发静默失联阻塞（进程存活、socket 消失；mock 时代
  同机制，非本计划 app 改动引入；三轮修复尝试后按上限停）。套件本体已
  重写对齐并提交，各流程已单点交互实证；解除动作见 §10 #0。
- `code_commit`（auto-lang `os-016-dev`，base 8c3b4db57）：
  `522857d33`(T-01..04) → `1073db0f6`(T-05/06+crates 修复) →
  `4a5f5e27d`(T-07/08/09 协议 v1.7) → `8b1631058`(T-10 套件) →
  `cea828063`(T-11 SPEC)。worktree `.wt/os-016/auto-os`（无实现改动）与
  `.wt/os-016/auto-lang` + 依赖 auto-down detached 检出。
- `task_ids`: T-01..T-09、T-11 ✅；T-10 部分完成（套件就位，绿跑被阻塞）。
- `evidence`: docs/plans/evidence/016/（d1 右键锚定定案、d3 JsonValue None
  级联 + 轨口径 + fs native id 撞号迁移、d4 目录删除口径、t01 深色列表、
  t03 重命名模态、t05 真实主目录快照、t07 open_with 端到端截图）。
- AC 状态：AC-01–07 结构与交互实证（AC-01 浅色视觉留复审桌面实测；
  AC-02 快照零 emoji + 截图；AC-03/04 模态/锚定菜单/创建/删除/重命名
  实证）；AC-08/09 端到端实证（bus 注入 → 041 启动消费，截图）；AC-10
  发送面完成、031 消费臂就位（back.api 桌面轨形态留 D-2 残留实测）；
  AC-11 部分被阻（见上）；AC-12 vue 轨留复审；AC-13 文档三处一致 ✅。
- 设计调整（授权范围内，证据已记录）：D-1 定案锚定 popover（shell 先例，
  免坐标）；D-2 定案 write_state `auto_open_path` + 目标 Tick 消费（双臂
  统一，未用 acceptance 放宽/纯 toast 收缩）；D-4 定案非空目录拒绝；
  新增 `fs.mtime` native（2986）与 `fs.rename/canonical/ext` id 迁移
  2983-2985（撞号修复）——crates 面超出原 T-05 预期，属等价实现范围内的
  必要修复，全部三表一致 + 提交注记。
- `next`: review（复审时裁定 T-10 残留是否随 AC-11 一并延展，或按
  blocked 项单独追踪）。

### 2026-09-14 起草 handoff（draft）

- `stage: new`，`plan_id: PLAN-016`，`plan_revision: 1`。
- `outcome: pass`——十一任务覆盖 AC-01–13 与 SD-01–03；路径/命令均经本仓与
  auto-lang 主检出实证（4.2 证据行号）；三个执行期决策点（D-1 右键锚定 /
  D-2 已运行送达臂 + 031 back 前置 / D-3 vue FS 桥 / D-4 目录删除口径）均配
  bounded 调查 + evidence 工件，不需用户先裁决即可开工。
- `next: work`（建议 PLAN-015 merge 后、与 PLAN-014 worktree 错峰开工；
  T-01–T-06 不碰 crates 可最先并行）。
- 变更任务/验收 ID：无（初稿）。

## 10. 待澄清事项

| # | 事项 | 状态/去向 |
|---|---|---|
| 0 | **T-10 残留（work handoff）**：desktop_mcp 全绿整跑被 MCP 服务器线程静默失联阻塞（进程存活 socket 消失；~50% 复现；与 app 改动无关）。解除动作：复审期在框架侧定位 MCP HTTP 线程死因（hyper/tokio task abort 无日志），或套件改注入式驱动 | **blocked**（唯一残留；其余 T-01..T-09/T-11 完成） |
| 1 | D-1：✅ 已定案——锚定 popover + placement（shell dock 菜单范式，免坐标）；`.at` 事件无坐标面也不再需要 | 已闭合（evidence/016/d1-context-menu.md） |
| 2 | D-2：已运行 app 的 open_with 送达臂 + 031 桌面轨 back 前置 | ✅ 主链定案——双臂统一 `write_state(auto_open_path)` + 目标 Tick 消费（免 handler 直调放宽）；031 消费臂已挂 SettleTick（open_file 同 OpenFile 流程），**桌面轨 back.api 实际可用性留实测**（opens 声明与启动路径不受阻） | 部分闭合（d3/d2 注记） |
| 3 | 虚拟桌面图标（storage 策展）与文件系统"桌面"合一展示 | 非目标（本计划）；桌面程序后续设计议题，建议届时在桌面程序台账另立条目 |
| 4 | 027 是否收编独立仓/apps 容器臂（PLAN-013 混合形态） | 非目标；待 app 成熟后另行计划 |
| 5 | 全盘驱动器枚举（"此电脑"） | v1 不做（stdlib 无 drives API）；地址栏手输绝对路径已可上探（**T-10 已实现地址栏**，可编辑回车跳转 + canonical 剥 `\?\` 前缀）；后续可提 stdlib `fs.drives` 提案 |
| 6 | 与 PLAN-014/015 的开工顺序 | 依赖既有授权范围内排程：015 merge 后开工；014 错峰——不需用户新授权，若用户指定并行则接受 renderer.rs 冲突面人工协调 |

---

# Phase 2：UX 反馈批（plan_revision 2 · 2026-09-14 用户实机反馈 9 项）

> 触发：用户实机观察反馈（截图 4 张），用户直接下达。设计/验证约束：VM 轨
> flex-wrap 不支持（P614 纪律）→ 图标模式改 grid 类（025 performance 先例）；
> `ondblclick` 支持（桌面图标同款）；button `title:` prop = 悬停 tooltip
> （PLAN-053）；`.at` 无 blur 事件 → 地址编辑退出用显式确认/取消钮；字体
> 家族框架面仅 serif/sans/mono（指定中文黑体需 renderer default_font 工作
> → 记 finding F-1，本轮做字号提升）。

| # | 用户反馈 | 分析 | 任务 |
|---|---|---|---|
| 1 | 右上 path input 与最右挤扁图标无用；取路径应在地址栏（点击变 input） | 独立地址栏与面包屑功能重复；挤扁体 = 布局压缩牺牲品 | T-12 |
| 2 | 面包屑层级 `/` 换行、三层阶梯错位；模拟地址栏过高 | row 内混排 button/text 高度不一致；分隔符独立节点被挤下行 | T-12 |
| 3 | 隐藏/+文件夹/+文件改纯图标（tooltip 文字）；小窗不压扁、地址栏可伸缩、右侧控件定宽 | 右侧控件全部 shrink-0 + icon-only + title tooltip；面包屑区 flex-1 独占伸缩 | T-13 |
| 4 | 大小与类型贴死；类型列应居中 | 两列间无间距（pr 缺失）；类型内容列左对齐与表头居中不一致 | T-14 |
| 5 | 中文默认字体太小、字型不对（期望黑体/系统默认） | 027 正文 text-xs(12px) 偏小 → 名列/侧栏升 text-sm；字体家族 = F-1 框架项 | T-15 |
| 6 | 快捷访问 icon 应差异化 | home/monitor/file-text/download/image/music 字面量分支（TreeIcon 范式） | T-16 |
| 7 | 图标模式未成 grid（VM flex-wrap 降级单行），溢出隐藏 | `row flex-wrap` → `grid grid-cols-4 md:grid-cols-6 xl:grid-cols-8`（025 先例） | T-17 |
| 8 | 图标卡只有名称可点 | 卡整体 mouse-area（onclick 选中 + ondblclick 打开）；grid popover 内容泄漏 bug 一并消除（卡内 popover 移除） | T-17 |
| 9 | 单击打开 → 应双击打开 | ondblclick 已支持；列表名列 onclick=选中 / ondblclick=打开；··· 菜单不变 | T-18 |

## 7.P2 验收标准（Phase 2 增量）

| ID | 可观察行为 | 验证方法 |
|---|---|---|
| AC-15 | 无独立地址栏 input；点击面包屑区变输入态，回车跳转、✕ 取消 | 实机操作 + 截图 |
| AC-16 | 面包屑各层同一行水平对齐、无换行分隔符；胶囊高与 input 一致（h-9） | 截图对照 |
| AC-17 | 隐藏/+文件夹/+文件为纯图标按钮（title 悬停出文字），定宽不压扁；搜索框/面包屑伸缩正常 | 窄窗 + 最大化截图 |
| AC-18 | 大小列与类型列有间距；类型内容列居中 | 截图对照 |
| AC-19 | 名称列与侧栏字号 text-sm | 截图对照 |
| AC-20 | 快捷访问五项图标各异（monitor/file-text/download/image/music） | 截图 |
| AC-21 | 图标模式为多列 grid；整卡可点（单击选中/双击打开）；卡内无泄漏菜单 | 实机操作 + 截图 |
| AC-22 | 列表行单击=选中、双击=打开；··· 菜单行为不变 | 实机操作 |

## 8.P2 Phase 2 执行步骤

- **T-12 〔lang〕面包屑点击编辑 + 去独立地址栏**（AC-15/16）：msg 增
  AddrEdit/AddrCancel；view 面包屑区 mouse-area 包裹 → addr_editing 切换
  input（值同步 current_path）；crumbs 全按钮化（去独立分隔符、统一 h-7）。
- **T-13 〔lang〕工具栏图标化 + 定宽**（AC-17）：隐藏/＋文件夹/＋文件 →
  icon-only + title tooltip + w-8 shrink-0；右区全部 shrink-0。
- **T-14 〔lang〕列表列距 + 类型居中**（AC-18）。
- **T-15 〔lang〕字号提升**（AC-19）：名列/侧栏 text-sm；字体家族记 F-1。
- **T-16 〔lang〕快捷图标差异化**（AC-20）：monitor/file-text/download/
  image/music 字面量分支。
- **T-17 〔lang〕grid 图标模式 + 整卡点击**（AC-21）：grid 类容器、mouse-area
  整卡 onclick/ondblclick、卡内 popover 移除（泄漏 bug 消除，记录框架
  popover 网格子树泄漏现象）。
- **T-18 〔lang〕双击打开**（AC-22）：名列 onclick=选中/ondblclick=打开。

## 9.P2 修复轮 4（用户实机反馈第二批 8 项，2026-09-14）

| # | 反馈 | 根因 | 修复 |
|---|---|---|---|
| 1 | "编辑路径"图标钮多余；点地址栏空白应切 input | mouse-area 多子件结构命中不可靠 | 删图标钮；mouse-area 包单行子件重构（空白点击→AddrEdit） |
| 2 | chevron 掉到按钮下方 | 循环体内条件节点（if !last icon）被 VM 纵向布局 | sep 进 crumb 对象字段，恒渲染 `text c.sep`（末层空串零宽） |
| 3 | 路径常态显示成按钮状 | 无 bg 类时按钮默认底色透出 | crumbs 改 variant="ghost"（常态透明、hover 显底） |
| 4 | 双击进不去目录；"选定：xxx"错位 | ondblclick 挂在 button 上（aura dblclick 通路在 mouse-area）；行 id 排序前编号致 files_view[id] 错位 | 整行 mouse-area（onclick 选中/ondblclick 打开/右键菜单）；排序后重编 id |
| 5 | 右键仅名称区可出菜单、不跟随鼠标 | oncontextmenu 挂名称钮；事件不携带坐标 | oncontextmenu 提升到整行（mouse-area）；菜单行内锚定；鼠标精确跟随 = F-3 框架债 |
| 6 | 名称列单独高亮多余 | 名称钮 hover 底 | 名称去按钮化（纯文本），整行高亮 |
| 7 | 右键菜单首项应为"打开"（=双击） | 已是 CtxOpen→OpenItem，行级重构后保持 | 复核 |
| 8 | 选中 + Enter = 双击 | 无键盘面 | actions DSL `shortcut: "Enter"`（弹层打开时守卫跳过） |

（任务并入 T-19 行级交互重构 / T-20 面包屑 r3 / T-21 Enter 打开。）

## 10.P2 Phase 2 新增 finding

| ID | 事项 | 去向 |
|---|---|---|
| F-1 | 中文字体家族指定（黑体/微软雅黑）：iced_adapter font_family 仅 serif/sans/mono 抽象，指定具体中文字体需 renderer default_font / cosmic fallback 面——框架工作，另立 | 框架债（复审裁定归属） |
| F-2 | 【定性修正】popover first-child = 锚件就地渲染（PLAN-528 设计语义，非泄漏）——菜单必须用 popover-trigger/popover-content 子标签拆分，否则首项（"打开"）会变锚件消失、菜单锚在首项位置 | 已按规范形态修复（R5） |
| F-6 | 【框架·高优】VM 动态视图无细粒度更新：任何被视图引用的状态写（选中/hover/ctx）→ view_dirty → 整棵 .at→iced 视图树重转换（无行级依赖追踪、无 diffing）。千级节点 × debug 构建 = 每次交互 0.1-0.3s 卡顿。框架方向：视图 diffing 或依赖追踪细粒度失效；短期缓解 = 优化构建 + 控制单视图节点规模 | **已立项 auto-lang PLAN-631**（autoui-interaction-primitives：F-5 hover 样式对 / F-7 popover pointer 定位 / F-6 剖析+缓存；含 F-3 事件坐标与 F-4 多选修饰键的演进面） |
| F-7 | 【框架】右键菜单正确终态 = 全视图单实例菜单 + 指针位置定位（Win11 式）：需要 a) 事件坐标面（=F-3）或 b) popover 原生指针定位原语（坐标不走状态回写，避免每次移动全量重建）。现 Plan 422 popover 仅锚件/坐标态两种定位 | 框架债（与 F-3 合并推进） |
| F-3 | 右键菜单无法精确跟随鼠标：`.at` 事件不携带指针坐标（D-1 同源），popover 仅锚件/坐标态定位 | 框架债（需事件坐标面） |
