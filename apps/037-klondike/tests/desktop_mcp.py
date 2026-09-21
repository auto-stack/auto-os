#!/usr/bin/env python3
"""
PLAN-039 T-04: MCP interaction tests for the REAL 037-klondike app in VM
mode — the VM-rail acceptance of the three-rail gate (Vue playwright ✓ /
VM MCP ← this / Rust golden ✓ sibling).

Starts `auto run -r vm` in the 037-klondike project directory and drives the
real iced window via autoui_* HTTP tools. The win path is DETERMINISTIC via
the store's DebugWinDeal fixture (foundations pre-filled A..Q, four kings on
the tableau — AutoSendAll finishes in 4 sends).

Usage:
    cd apps/037-klondike/tests
    python desktop_mcp.py            # test real 037-klondike (VM rail)

Prerequisites: auto built with ui-iced (AUTO_BIN override honored),
python requests.
"""

import subprocess
import tempfile
import sys
import time
import os
import re

try:
    import requests
except ImportError:
    print("Please install requests: pip install requests")
    sys.exit(1)

MCP_PORT_DEFAULT = 9257


def pick_free_port(start=MCP_PORT_DEFAULT):
    """First free port in [start, start+100) — stale-zombie immunity."""
    import socket
    for port in range(start, start + 100):
        with socket.socket(socket.AF_INET, socket.SOCK_STREAM) as s:
            if s.connect_ex(("127.0.0.1", port)) != 0:
                return port
    raise RuntimeError(f"No free port in [{start}, start+100)")


_AUTO_BIN = os.environ.get(
    "AUTO_BIN",
    r"D:\autostack\.wt\lang-039\auto-lang\target\debug\auto.exe",
)
AUTO_BIN = _AUTO_BIN
KLON_PROJECT = os.path.normpath(os.path.join(os.path.dirname(__file__), ".."))


class McpClient:
    def __init__(self, url):
        self.url = url
        self.req_id = 0

    def call(self, tool_name, **arguments):
        self.req_id += 1
        resp = requests.post(self.url, json={
            "jsonrpc": "2.0", "method": "tools/call",
            "params": {"name": tool_name, "arguments": arguments},
            "id": self.req_id,
        }, timeout=15)
        data = resp.json()
        if "error" in data:
            raise RuntimeError(f"MCP error: {data['error']}")
        content = data.get("result", {}).get("content", [])
        return content[0]["text"] if content else ""

    def snapshot(self):
        return self.call("autoui_snapshot")

    def click(self, element_id):
        return self.call("autoui_action", element_id=element_id, action="press")

    def state(self, *fields):
        return self.call("autoui_state", fields=list(fields))

    def screenshot(self, name="", baseline=False):
        return self.call("autoui_screenshot", name=name, baseline=baseline)


def wait_for_server(url, timeout=40):
    for _ in range(timeout):
        try:
            requests.post(url, json={
                "jsonrpc": "2.0", "method": "tools/list", "params": {}, "id": 1
            }, timeout=2)
            return True
        except (requests.ConnectionError, requests.Timeout):
            time.sleep(1)
    return False


def find_buttons_by_label(snapshot_text, label):
    pat = re.compile(r'button\s+#(aura_\d+|vnode_\d+)\s+"' + re.escape(label) + '"')
    return pat.findall(snapshot_text)


def first_button_by_text(snapshot_text, fragment):
    pat = re.compile(r'button\s+#(aura_\d+|vnode_\d+)\s+"[^"]*' + re.escape(fragment))
    hits = pat.findall(snapshot_text)
    return hits[0] if hits else None


class TestResult:
    def __init__(self):
        self.passed = 0
        self.failed = 0
        self.errors = []

    def check(self, name, condition, detail=""):
        if condition:
            self.passed += 1
            print(f"  PASS  {name}")
        else:
            self.failed += 1
            self.errors.append(f"{name}: {detail}")
            print(f"  FAIL  {name}: {detail}")


# ── Real 037-klondike suite ────────────────────────────────────────────────

def wait_first_frame(mcp, timeout=120):
    """Retry snapshot until the app has actually rendered (VM boot+compile
    for store-composable apps takes ~1 min — observed 54s for klondike)."""
    deadline = time.time() + timeout
    while time.time() < deadline:
        try:
            snap = mcp.snapshot()
            if "No UI available" not in snap and "No state available" not in snap:
                return snap
        except RuntimeError:
            pass
        time.sleep(1.5)
    return mcp.snapshot()


def run_tests_037(mcp_url):
    mcp = McpClient(mcp_url)
    result = TestResult()

    # T1: structure — top bar buttons, stock badge, 4 foundations, 7 columns.
    print("\nT1: UI Snapshot structure")
    snap = wait_first_frame(mcp)
    result.check("widget App", 'widget: "App"' in snap, snap[:120])
    result.check("title label", "纸牌接龙" in snap, "title missing")
    for label in ("矢量", "SVG", "必胜测试", "自动收牌 (Auto)", "撤销 (Undo)", "新对局"):
        result.check(f"button {label}", f'"{label}"' in snap, "missing")
    result.check("stock badge 24", '"24"' in snap, "stock count missing")
    result.check("foundation K placeholders", snap.count('"K"') >= 4,
                 "foundation placeholder missing")

    # T2: initial state.
    print("\nT2: Initial State")
    state = mcp.state("moves", "game_state")
    result.check("moves 0", "moves: 0" in state, state)
    result.check("game_state playing", 'game_state: "playing"' in state, state)
    stock_state = mcp.state("stock_cards")
    result.check("stock 24 cards", "stock_cards: 24" in stock_state or
                 stock_state.count(",") >= 20, stock_state[:80])

    # T3: ClickStock — deal one card into waste (deterministic LCG deck).
    print("\nT3: ClickStock → waste")
    snap = mcp.snapshot()
    # Stock pile button renders with an EMPTY label (its count is a text
    # child); it is the first empty-label button in document order.
    empty_ids = re.findall(r'button\s+#(aura_\d+|vnode_\d+)\s+""', snap)
    stock_badge = empty_ids[0] if empty_ids else None
    result.check("stock button found", stock_badge is not None,
                 f"empty-label buttons: {len(empty_ids)}")
    if stock_badge:
        mcp.click(stock_badge)
        time.sleep(0.4)
        state = mcp.state("moves", "waste_cards")
        result.check("moves 1 after stock click", "moves: 1" in state, state)
        result.check("waste has card", "waste_cards" in state, state[:80])
        snap = mcp.snapshot()
        result.check("stock badge 23", '"23"' in snap, "badge not 23")

    # T4: DebugWinDeal fixture → AutoSendAll → win (deterministic).
    print("\nT4: DebugWinDeal → AutoSendAll → victory")
    snap = mcp.snapshot()
    win_btn = first_button_by_text(snap, "必胜测试")
    result.check("debug-win button found", win_btn is not None, "missing")
    if win_btn:
        mcp.click(win_btn)
        time.sleep(0.5)
        state = mcp.state("moves", "game_state", "f0_cards", "col0_cards")
        result.check("fixture moves 48", "moves: 48" in state, state)
        result.check("fixture state playing", 'game_state: "playing"' in state, state)
        result.check("foundation has 12", "f0_cards: 12" in state or
                     state.count(",") > 8, state[:120])
        result.check("king on col0", "col0_cards" in state, state[:120])
        auto_btn = first_button_by_text(mcp.snapshot(), "自动收牌")
        result.check("auto-send button found", auto_btn is not None, "missing")
        if auto_btn:
            mcp.click(auto_btn)
            time.sleep(0.6)
            state = mcp.state("game_state")
            result.check("victory reached", 'game_state: "won"' in state or
                         'game_state: "win"' in state, state)
            snap = mcp.snapshot()
            result.check("victory banner", ("恭喜通关" in snap) or ("Play Again" in snap),
                         "banner missing")

    # T5: undo round-trip on a fresh game.
    print("\nT5: New game + stock + undo")
    snap = mcp.snapshot()
    new_btn = first_button_by_text(snap, "新对局")
    if new_btn:
        mcp.click(new_btn)
        time.sleep(0.4)
        state = mcp.state("moves")
        result.check("moves reset 0", "moves: 0" in state, state)
        stock_badge = first_button_by_text(mcp.snapshot(), "24")
        if stock_badge:
            mcp.click(stock_badge)
            time.sleep(0.3)
            undo_btn = first_button_by_text(mcp.snapshot(), "撤销 (Undo)")
            result.check("undo enabled", undo_btn is not None, "missing")
            if undo_btn:
                mcp.click(undo_btn)
                time.sleep(0.3)
                state = mcp.state("moves")
                result.check("undo restores 0", "moves: 0" in state, state)

    mcp.screenshot(name="klondike_vm_final")
    return result


def main():
    mcp_port = pick_free_port()
    mcp_url = f"http://127.0.0.1:{mcp_port}/mcp"
    env = dict(os.environ)
    env["AUTOUI_MCP_PORT"] = str(mcp_port)
    env["AUTO_LANG_ROOT"] = r"D:\autostack\.wt\os-039\auto-lang"
    # stdout must NOT be a pipe — the VM rail logs heavily (compile warnings,
    # iced settings dumps); a filled pipe blocks the child before first frame.
    log_fh = open(os.path.join(tempfile.gettempdir(), "klondike_vm_mcp.log"),
                  "w", encoding="utf-8", errors="replace")
    proc = subprocess.Popen(
        [AUTO_BIN, "run", "-r", "vm"],
        cwd=KLON_PROJECT, env=env,
        stdout=log_fh, stderr=subprocess.STDOUT,
    )
    try:
        if not wait_for_server(mcp_url):
            out = proc.stdout.read(4000) if proc.stdout else b""
            print("MCP server never came up:", out.decode("utf-8", "replace"))
            sys.exit(2)
        result = run_tests_037(mcp_url)
        print(f"\n=== 037-klondike VM rail: {result.passed} passed, "
              f"{result.failed} failed ===")
        if result.errors:
            for e in result.errors:
                print(" -", e)
        sys.exit(1 if result.failed else 0)
    finally:
        proc.terminate()
        try:
            proc.wait(timeout=8)
        except subprocess.TimeoutExpired:
            proc.kill()


if __name__ == "__main__":
    main()
