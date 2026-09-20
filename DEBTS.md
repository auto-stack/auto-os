# AutoOS DEBTS — deferred delivery follow-ups

This ledger holds follow-up work explicitly deferred from delivered app Plans.
Entries remain tied to the app Spec and are not treated as failures of the
delivered feature scope.

## DEBT-005-01 — native input lifecycle and visual fixtures

- **Source:** PLAN-005, revision 3; `docs/specs/apps/tetris.md` input/UI sections.
- **Status:** open; deferred by user decision on 2026-09-14.
- **Scope:** complete long-press/repeat, keyup, blur/focus and physical-input
  coverage across VM/Rust; add stable narrow-window, theme and pixel fixtures.
- **Acceptance:** a reproducible M1–M7 input/visual matrix with durable process,
  window and screenshot evidence.

## DEBT-005-02 — desktop and game-gallery acceptance

- **Source:** PLAN-005, revision 3; `docs/specs/apps/tetris.md` integration section.
- **Status:** open; deferred by user decision on 2026-09-14.
- **Scope:** verify desktop discovery, opening, starting a game, and the
  `05-games` gallery entry on the real desktop host.
- **Acceptance:** discoverable `tetris` registration, successful launch and one
  playable round through the desktop/gallery path.

## DEBT-005-03 — merged persistence fault matrix

- **Source:** PLAN-005, revision 3; `docs/specs/apps/tetris.md` persistence section.
- **Status:** open; deferred by user decision on 2026-09-14.
- **Scope:** exercise merged native restart, corrupted/read-only records,
  concurrent writes, maximum-value races and cross-mode data roots.
- **Acceptance:** durable evidence for the failure semantics and maximum-value
  invariant in the merged and no-merge deployments.

## DEBT-035-01 — 关闭态坐标锚 popover 合成层内容泄漏（rq/canvas 族）

- **Source:** PLAN-035 T-00 实机归因（`tmp/p035/attribution.md` #2；
  证据 t00_b0_boot12s 截图：boot 未交互即有 blank 菜单三文本 50% 透明
  幽灵于桌面左下 + 任务栏区白色卡块；vtree 无对应节点）。
- **Status:** open；本计划以 .at 内容守卫（desktop.at 四处坐标锚
  popover 内容包 open 态 if）止血——内容不物化即无可泄漏面，用户可见
  症状消除；深层根因移交本债。
- **Scope:** 定位关闭态 popover 面板内容进入合成/绘制管道路径（疑似
  rq 表面 canvas 绘制族丢 fill op/半透明层合成——PLAN-032..034
  RenderQueue 线）；补绘制层回归测试（关闭态面板零 draw op）。
- **Acceptance:** 关闭态坐标锚 popover 在任意组件零绘制零命中；headless
  或实机证据各一。

