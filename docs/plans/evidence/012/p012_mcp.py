#!/usr/bin/env python3
"""PLAN-012 探针 CLI：对运行中的 ui_desktop MCP 端口发单个工具调用。

用法：python tmp/p012_mcp.py <tool> [k=v ...] [--timeout N]
示例：python tmp/p012_mcp.py autoui_state fields=__wm_wins fields=__dock_pinned_csv
"""
import json
import sys

import requests

PORT = 9348
URL = f"http://localhost:{PORT}/mcp"


def call(tool, timeout=20, **args):
    resp = requests.post(URL, json={
        "jsonrpc": "2.0", "method": "tools/call",
        "params": {"name": tool, "arguments": args}, "id": 1,
    }, timeout=timeout)
    data = resp.json()
    if "error" in data:
        return f"MCP-ERROR: {data['error']}"
    content = data.get("result", {}).get("content", [])
    return content[0]["text"] if content else ""


def main():
    argv = sys.argv[1:]
    if not argv:
        print(__doc__)
        return 1
    tool = argv[0]
    timeout = 20
    args = {}
    for a in argv[1:]:
        if a.startswith("--timeout"):
            timeout = float(a.split("=", 1)[1])
            continue
        if "=" in a:
            k, v = a.split("=", 1)
            args.setdefault(k, []).append(v)
    flat = {k: (v[0] if len(v) == 1 else v) for k, v in args.items()}
    print(call(tool, timeout=timeout, **flat))
    return 0


if __name__ == "__main__":
    sys.exit(main())
