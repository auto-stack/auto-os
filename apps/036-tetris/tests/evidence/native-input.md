# Native input evidence

Driver: `python tests/native_physical.py --mcp-url http://127.0.0.1:9271/mcp --blur`

The driver is ready to send real Windows key-down/key-up events, hold the first
key for a configurable interval, move focus to a second window for blur, then
capture AutoUI snapshots before and after the sequence.  The 2026-09-14 run
returned:

`BLOCKED: no visible native window matching '俄罗斯方块'; matches: none`

The driver was then run against the visible VM window on 2026-09-14 with
`--long-press-ms 600 --blur`; it completed with key-down/key-up events for
`ArrowLeft, ArrowRight, ArrowDown, Space, P`, recorded a focus switch to a
second visible window, and left the game paused after the `P` shortcut. The
full before/after snapshot and event record is in
`tests/evidence/native-input-live.json`.

The same live VM run produced `tests/evidence/vm-ready-modal.png`; the ready
dialog is centered in the window with a dimmed game surface behind it.
