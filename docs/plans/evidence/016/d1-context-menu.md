# D-1 决策工件：右键菜单锚定方案（PLAN-016 T-04）

**日期**：2026-09-14　**任务**：T-04　**状态**：已定案

## 问题

原 027 右键菜单 popover 写死假坐标（`ItemCtx(id, 450.0, 200.0)`），弹层不跟随
触发点。计划首选"事件携带指针坐标真实锚定"，需调查 `.at` 事件是否暴露坐标。

## 调查结论

`.at` 的 `oncontextmenu` 事件**不携带指针坐标**（aura_view_builder 仅提取
handler 名，`oncontextmenu.prevent:` 无坐标参数面）——首选路径不成立，也无需
成立：**shell 早有更优范式**。

## 定案：锚定 popover + placement（shell dock 菜单同款）

证据：`auto-os/shell/shell.at:182,246,299`——dock 条目右键菜单/Hover 预览均为

```
popover (open: <按 id 匹配>, placement: "top", ondismiss: .Close, class: "...") { 菜单项 }
```

- popover 锚定**父容器首个子件**（aura_view_builder.rs:6865 注释"widget 锚
  (first-child)"），不需要任何坐标。
- 触发：`oncontextmenu.prevent: .ItemCtx(item.id)`（阻止浏览器默认菜单，
  shell 同款 `oncontextmenu.prevent: .WinMenu(p.id)`）。
- 打开态按 `ctx_id == item.id` 逐行匹配——shell 的 `win_menu == p.id` 同构；
  for 循环内多 popover 实例是 shell 已验证形态（P320 单态常驻挂载）。

## 027 落地

- 列表行：popover 置于行尾操作格内，锚 = `more-horizontal` 触发钮，
  `placement: "bottom-end"`。
- 网格卡：popover 置于卡尾，锚 = 卡内容，`placement: "bottom-start"`。
- 状态收敛：`ctx_open/ctx_x/ctx_y` 三变量退役 → 单 `ctx_id int = -1`。

## 实证（2026-09-14 VM 轨实机，`auto run -r vm` + autoui MCP）

- `···` 触发 → 快照含全部五项（打开/重命名/复制/剪切/删除）+ 分隔线。
- `CtxRename` 路径端到端：菜单 → 重命名模态 → 改 config.toml → config2.toml
  落列（`t03-rename-dialog.png`）。

## 残留

无。坐标锚 popover（x/y 参数）在 027 已无使用点。
