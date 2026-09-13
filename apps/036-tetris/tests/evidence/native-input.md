# Native input evidence

Driver: `python tests/native_physical.py --mcp-url http://127.0.0.1:9247/mcp --blur`

The driver is ready to send real Windows key-down/key-up events, hold the first
key for a configurable interval, move focus to a second window for blur, then
capture AutoUI snapshots before and after the sequence.  The 2026-09-14 run
returned:

`BLOCKED: no visible native window matching '俄罗斯方块'; matches: none`

The VM process exposed an MCP endpoint but no visible top-level window to the
current desktop session.  This is an environment/desktop-host limitation, not
a logical-input pass.
