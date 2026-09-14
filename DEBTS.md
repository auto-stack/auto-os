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
