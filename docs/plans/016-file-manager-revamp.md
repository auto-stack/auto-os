---
plan_id: PLAN-016
status: executing              # drafting → executing → execution_done → reviewed → archived
feature_name: file-manager-revamp
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
total_steps: 11
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
