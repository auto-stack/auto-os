#!/usr/bin/env bash
# mcpq.sh — desktop AutoUI MCP one-shot JSON-RPC caller (PLAN-041 inspection driver)
# usage: mcpq.sh <port> <tool> [json-args-object] [--raw]
PORT="$1"; TOOL="$2"; ARGS="${3:-{\}}"
BODY=$(printf '{"jsonrpc":"2.0","id":1,"method":"tools/call","params":{"name":"%s","arguments":%s}}' "$TOOL" "$ARGS")
RESP=$(curl -s --max-time 60 -X POST "http://127.0.0.1:${PORT}/mcp" -H "Content-Type: application/json" -d "$BODY")
if [ "${4:-}" = "--raw" ]; then printf '%s' "$RESP"; exit 0; fi
# default: extract result.content[0].text (best effort via python)
python - "$RESP" <<'PY'
import json,sys
try:
    r=json.loads(sys.argv[1])
    res=r.get("result",{})
    if res.get("isError"): print("[tool-error]")
    for c in res.get("content",[]):
        t=c.get("text")
        if t: print(t[:8000])
    if not res: print(json.dumps(r)[:2000])
except Exception as e:
    print("[parse-fail]",e); print(sys.argv[1][:2000])
PY
