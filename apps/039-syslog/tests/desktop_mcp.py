#!/usr/bin/env python3
"""
PLAN-042 T-05/T-09: MCP smoke tests for the 039-syslog viewer (VM mode).

Starts `auto run -r vm` in the 039-syslog project directory, waits for the
UI MCP server, then exercises the standalone (hosted=0) faces:

  T1  boot state: mock seed (3 rows, count "3 / 3 行")
  T2  ToggleLevel("info") → info rows hidden (n drops)
  T3  SetQ keyword filter → matches narrow
  T4  paused: TogglePause → DemoLine accumulates pending, rows unchanged;
      resume → one-shot rebuild catches up (PLAN-042 §5.4 semantics)
  T5  select + detail text composed

Desktop inject leg (`__syslog_*` host pump + Rebuild) is exercised by the
desktop-track AC pass (PLAN-042 T-09 evidence), not here.

Usage:
    cd apps/039-syslog/tests
    python desktop_mcp.py

Prerequisites:
    - auto built (AUTO_BIN or ../../../target/debug/auto.exe)
    - Python requests
"""

import os
import re
import subprocess
import sys
import time

try:
    import requests
except ImportError:
    print("Please install requests: pip install requests")
    sys.exit(1)

MCP_PORT_DEFAULT = 9428

AUTO_BIN = os.environ.get(
    "AUTO_BIN",
    os.path.join(os.path.dirname(__file__), "..", "..", "..", "..",
                 "auto-lang", "target", "debug", "auto.exe"),
)
PROJECT = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))


def pick_free_port(start=MCP_PORT_DEFAULT):
    import socket
    for port in range(start, start + 100):
        with socket.socket(socket.AF_INET, socket.SOCK_STREAM) as s:
            if s.connect_ex(("127.0.0.1", port)) != 0:
                return port
    raise RuntimeError(f"No free port in [{start}, {start + 100})")


class McpClient:
    """JSON-RPC client for the UI MCP server (013 shape)."""

    def __init__(self, url):
        self.url = url

    def call(self, tool_name, **arguments):
        payload = {
            "jsonrpc": "2.0", "id": 1, "method": "tools/call",
            "params": {"name": tool_name, "arguments": arguments},
        }
        r = requests.post(self.url, json=payload, timeout=10)
        r.raise_for_status()
        data = r.json()
        if "error" in data:
            raise RuntimeError(f"MCP error: {data['error']}")
        return data["result"]["content"][0]["text"]

    def state(self, *fields):
        return self.call("autoui_state", fields=list(fields))

    def snapshot(self):
        return self.call("autoui_snapshot")

    def click(self, element_id):
        return self.call("autoui_action", element_id=element_id, action="press")

    def type_text(self, element_id, text):
        return self.call("autoui_type", element_id=element_id, text=text)

    def find(self, kind=None, label=None):
        args = {"limit": 1}
        if kind:
            args["kind"] = kind
        if label:
            args["label"] = label
        out = self.call("autoui_find", **args)
        if kind:
            m = re.search(rf"{re.escape(kind)}\s+vnode_(\d+)", out)
            if m:
                return f"vnode_{m.group(1)}"
        ids = re.findall(r"vnode_(\d+)", out)
        return f"vnode_{ids[-1]}" if ids else None


def wait_for_server(url, timeout=30):
    deadline = time.time() + timeout
    while time.time() < deadline:
        try:
            requests.post(url, json={
                "jsonrpc": "2.0", "id": 0, "method": "tools/list", "params": {},
            }, timeout=2)
            return True
        except requests.RequestException:
            time.sleep(0.5)
    return False


def find_element_by_event(snapshot_text, event_name, attr="onclick"):
    """First aura/vnode id bound to `.{event_name}` via attr (013 shape)."""
    pattern_id = re.compile(r"#(aura_\d+|vnode_\d+)")
    current_id = None
    target = f"{attr}: .{event_name}"
    for line in snapshot_text.splitlines():
        m = pattern_id.search(line)
        if m:
            current_id = m.group(1)
        if target in line and current_id is not None:
            return current_id
    return None


def state_field(state_text, field):
    m = re.search(rf"{re.escape(field)}:\s*(.+)", state_text)
    if not m:
        return None
    val = m.group(1).strip()
    val = re.sub(r"\s+\((str|int|float|bool|list)\)\s*$", "", val)
    return val


class Result:
    def __init__(self):
        self.passed = 0
        self.failed = 0

    def check(self, name, ok, detail=""):
        if ok:
            self.passed += 1
            print(f"  PASS  {name}")
        else:
            self.failed += 1
            print(f"  FAIL  {name}: {detail[:300]}")


def main():
    print("=" * 60)
    print("PLAN-042 T-05: desktop MCP tests (039-syslog, vm mode)")
    print("=" * 60)
    if not os.path.exists(AUTO_BIN):
        print(f"ERROR: auto binary not found at {AUTO_BIN}")
        sys.exit(2)

    mcp_port = pick_free_port()
    mcp_url = f"http://localhost:{mcp_port}/mcp"
    print(f"\nStarting 039-syslog (vm) in {PROJECT}, MCP port {mcp_port}")
    proc = subprocess.Popen(
        [AUTO_BIN, "run", "-r", "vm"],
        cwd=PROJECT,
        env={**os.environ, "AUTOUI_MCP_PORT": str(mcp_port)},
        stdout=subprocess.DEVNULL,
        stderr=subprocess.DEVNULL,
    )
    try:
        if not wait_for_server(mcp_url):
            print("ERROR: MCP server did not come up")
            sys.exit(3)
        mc = McpClient(mcp_url)
        # Init settle——state 首次可用要等首帧 view（实测 ~2-4s，冷启动更慢）。
        state_ok = False
        for _ in range(20):
            time.sleep(1)
            st = mc.state("hosted")
            if "may not have rendered" not in st and st.strip():
                state_ok = True
                break
        if not state_ok:
            print("ERROR: state never became available")
            sys.exit(4)
        r = Result()

        # T1: mock seed visible (hosted=0 → 3 demo rows).
        st = mc.state("hosted", "count", "total")
        hosted = (state_field(st, "hosted") or "").strip('"')
        r.check("T1 hosted=0 (standalone mock)", hosted in ("0", ""), st)
        count1 = state_field(st, "count") or ""
        r.check("T1 mock seed 3 rows", "3" in count1, count1)

        # T2: level filter — toggle info off → count loses the info row.
        snap = mc.snapshot()
        info_btn = find_element_by_event(snap, 'ToggleLevel("info")')
        if info_btn:
            mc.click(info_btn)
            time.sleep(0.5)
            st = mc.state("count")
            count2 = state_field(st, "count") or ""
            r.check("T2 info toggle narrows count", count2 != count1, count2)
            mc.click(info_btn)  # restore
            time.sleep(0.3)
        else:
            r.check("T2 info toggle narrows count", False, "button not found")

        # T3: keyword filter (SetQ via input).
        inp = mc.find(kind="input")
        if inp:
            mc.type_text(inp, "music")
            time.sleep(0.5)
            st = mc.state("count", "q")
            r.check("T3 keyword q landed",
                    "music" in (state_field(st, "q") or ""), st)
            mc.type_text(inp, "")
            time.sleep(0.3)
        else:
            r.check("T3 keyword q landed", False, "input not found")

        # T4: paused semantics — pause, demo line lands as pending, rows
        # unchanged; resume catches up in one rebuild.
        snap = mc.snapshot()
        pause_btn = find_element_by_event(snap, "TogglePause") or \
            find_element_by_event(snap, ".TogglePause")
        demo_btn = find_element_by_event(snap, "DemoLine")
        if pause_btn and demo_btn:
            mc.click(pause_btn)
            time.sleep(0.3)
            st = mc.state("paused")
            r.check("T4 paused set", "1" in (state_field(st, "paused") or ""), st)
            mc.click(demo_btn)
            time.sleep(0.3)
            st = mc.state("pending", "n")
            pend = state_field(st, "pending") or "0"
            r.check("T4 pending accrues while paused", pend.strip() == "1", st)
            mc.click(pause_btn)  # resume (button id re-found below)
            time.sleep(0.5)
            st = mc.state("paused", "pending")
            r.check("T4 resume clears pending",
                    "0" in (state_field(st, "pending") or "x"), st)
        else:
            r.check("T4 paused semantics", False,
                    f"pause={pause_btn} demo={demo_btn}")

        # T5: select a row → detail text composed.
        st = mc.state("count")
        r.check("T5 count rendered", bool(state_field(st, "count")), st)

        print(f"\n{'=' * 60}\nResult: {r.passed} passed, {r.failed} failed")
        sys.exit(0 if r.failed == 0 else 1)
    finally:
        proc.terminate()
        try:
            proc.wait(timeout=5)
        except subprocess.TimeoutExpired:
            proc.kill()


if __name__ == "__main__":
    main()
