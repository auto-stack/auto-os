# Rules golden evidence

`cargo test -p tetris --test rules_golden --no-default-features --features ui-iced`

Result on 2026-09-14: **3 passed**.

- `opening_and_lock_golden`: deterministic spawn, move, rotate, soft drop,
  hard drop, lock and next-piece transition.
- `line_clear_score_and_compaction_golden`: one through four line clears,
  score tiers (100/300/500/800), feedback, fixed board length and top-row
  compaction.
- `all_piece_rotations_golden`: all seven pieces and four rotation states,
  including the exact eight coordinate values for every state.

The fixture includes the generated Rust component, so it exercises the same
`TetrisStore::on` implementation used by the Rust renderer.  The VM side still
needs a fixture injection/test hook; AutoUI MCP currently exposes only logical
presses and cannot seed a board.
