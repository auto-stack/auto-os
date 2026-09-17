# PLAN-014 rev2 — repair cycle 1 收据（work 阶段，2026-09-17）

review needs_fix（F-01..F-04 blocking / F-05 non-blocking）→ 本轮修复实证。

## F-03 根因（review 疑似「求值深度限制」实为注释语句合同）

- 现象：`desktop_surface_at_loads_interactions_and_dispatch` hover 变体类
  0/2；实证追加：格 col 的 style 解析为**空串**（非取错分支）→
  `Style::parse("")` 失败 → 格 col style 整体 None（aura 树 prop 完好
  = `AuraPropValue::Expr(If)`，降链完好，丢失在求值）。
- 根因：desktop.at 图标格样式链（drag→drop→launching→selected→hover）
  的**嵌套 else 分支体内含 `//` 注释**；解析器把注释收进语句流
  （`Stmt::Comment` 是 `ast::Stmt` 正式成员）→ 第二层 If 的 else 体
  `stmts.len() == 2` → Plan 339「分支体 = 单表达式」合同（string 侧与
  value 侧 If 臂均 `len() == 1` 严格判定）不识别 → 整链落 `String::new()`
  → 深层分支（launching/selected/hover）从未被求值。
- 与分支深度的相关是**伴随现象**：W-04 深化链时把注释写进了嵌套 else
  体（022 三分支版注释在外层，未触发）。
- 修复（builder 侧，非测试让步）：`aura_view_builder.rs` 新增
  `first_meaningful_stmt()`（跳过 `Stmt::Comment`/`Stmt::EmptyLine` 取
  首条有效语句），string/value 两个 If 求值臂共用。注释从此在分支体
  任意深度合法。
- 回归：`conditional_style_with_comments_in_branches_resolves_hover`
  （端到端：注释进嵌套链 → hover 变体存活）+
  `first_meaningful_stmt_skips_comments_and_blank_lines`（单元）。

## F-03 附带发现（同测试后段，W-02 配套）

hover 修复后测试推进到 wallpaper 断言：W-02 追加语义下 `__desktop_cmd`
含前步 `activate` 记录拼接，`== "wallpaper_pick"` 必红（review 轮因
hover 断言在前未暴露）。修 fixture：activate 断言后补
`drain_app_desktop_commands`（排空兼断 `ActivateApp` 到达宿主）。

## F-02 / F-04（fixture 合同更新）

- F-02：`desktop_shell_at_builds_with_dock_defaults` 删 `WorkspaceClose`
  调用段（W-01 已批准退役，pager × 按钮无发送者），注释同步。
- F-04：`switcher_summon_advance_confirm_roundtrip` 断言更新为新合同：
  summon 后显式断 `sel == 1`（W-10 预选）；Advance 断回绕 `1 → 0`；
  confirm 落 `rows[0] = Beta` 与既有 drain 断言连贯。

## F-01（SD-01 spec）

`docs/specs/shell/showdesk-ux-polish.md` 落盘本 worktree：六节行为合同
（SD-01 桌面选中/启动反馈、SD-02 hidden 去重与恢复、SD-03 时钟/日期注
入面、SD-04 badge 形态、SD-05 通知来源跳转、SD-06 switcher 预选/hover）
+ 验收锚映射 AC-02..04。

## F-05（vue 轨运行时门，非阻塞项）

worktree `auto-lang/examples/desktop-host/gen/front/vue`：生成 src 实际
import `reka-ui`(×65)/`@vueuse/core`(×18)/`class-variance-authority`
(×12)/`vue-sonner`(×1) 而 package.json 四者皆缺 → vite 启动失败。处置
（review 授权的「gen 等价补齐，禁 junction」）：生成产物 package.json 补
四依赖（版本对齐伞内 ui-gallery/widgets-gallery）+ `pnpm install` 装齐
（Done in 5.2s，四包在位）。**codegen 侧债务**（auto-man vue.rs 442
usage 检测漏 desktop-host 场景依赖 → package.json 漏记）不在本计划授权
面，挂账待另行处置。

## 验证实录（worktree `.wt/os-014/auto-lang`，分支 auto-os-dev）

- 修复测试三件 PASS：`desktop_shell_at_builds_with_dock_defaults` /
  `switcher_summon_advance_confirm_roundtrip` /
  `desktop_surface_at_loads_interactions_and_dispatch`（后者连带
  `conditional_style_with_comments_in_branches_resolves_hover` 与
  `first_meaningful_stmt_skips_comments_and_blank_lines`）。
- 全 ui 套件门与新增红归因：见计划 §9 repair work 记录（desktop 进程
  并发污染类沿用 review 已证口径——`auto`(29724)/`ui_desktop`(13140)
  实测在跑；本修 diff 零新增红以 stash 对照实证）。
