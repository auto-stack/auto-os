#!/usr/bin/env python3
"""PLAN-012 F2 走查探针：隔离档案启动 ui_desktop（MCP 验收通道开）。

env 沿 handoff 记录：AUTOOS_DESKTOP_CONFIG / AUTO_VM_STORAGE_FILE /
AUTO_OS_ROOT 指 os-012 worktree；CWD 必须 = os worktree 根。
MCP 端口 9348（PLAN-008 先例，避开缺省 9247）。
"""
import os
import subprocess
import sys
import time

import requests

PORT = 9348
URL = f"http://localhost:{PORT}/mcp"
EXE = r"D:\autostack\.wt\os-012\auto-lang\target\debug\examples\ui_desktop_probe.exe"
CWD = r"D:\autostack\.wt\os-012\auto-os"
PROFILE = os.path.expandvars(r"%TEMP%\os012-f2")


def main():
    env = {**os.environ,
           "AUTOOS_DESKTOP_CONFIG": os.path.join(PROFILE, "config.at"),
           "AUTO_VM_STORAGE_FILE": os.path.join(PROFILE, "storage.json"),
           "AUTO_OS_ROOT": CWD,
           "AUTOUI_MCP_PORT": str(PORT),
           "AUTOUI_ACCEPTANCE": "1"}
    DETACH = 0x00000008
    proc = subprocess.Popen([EXE], cwd=CWD, env=env,
                            creationflags=DETACH,
                            stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL)
    print(f"launched pid={proc.pid}")
    for _ in range(60):
        try:
            requests.post(URL, json={"jsonrpc": "2.0", "method": "tools/call",
                                     "params": {"name": "autoui_state", "arguments": {}},
                                     "id": 0}, timeout=2)
            print("MCP ready")
            return 0
        except Exception:
            time.sleep(0.5)
    print("MCP not ready in 30s")
    return 1


if __name__ == "__main__":
    sys.exit(main())
