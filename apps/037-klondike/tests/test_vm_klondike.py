"""
Klondike VM Mode MCP Test Script
Captures initial screenshots of the Klondike solitaire app running in VM/Iced mode.
"""

import json
import os
import socket
import subprocess
import sys
import time
import urllib.request
import urllib.error
import shutil

SCREENSHOTS_DIR = os.path.join(os.path.dirname(__file__), "screenshots")

def pick_free_port() -> int:
    with socket.socket(socket.AF_INET, socket.SOCK_STREAM) as s:
        s.bind(('127.0.0.1', 0))
        return s.getsockname()[1]

class AutoUiMcpClient:
    def __init__(self, port: int):
        self.url = f"http://127.0.0.1:{port}/mcp"
        self._req_id = 1

    def call(self, tool_name: str, args: dict = None) -> dict:
        req = {
            "jsonrpc": "2.0",
            "id": self._req_id,
            "method": "tools/call",
            "params": {"name": tool_name, "arguments": args or {}}
        }
        self._req_id += 1
        data = json.dumps(req).encode('utf-8')
        http_req = urllib.request.Request(
            self.url,
            data=data,
            headers={'Content-Type': 'application/json'}
        )
        with urllib.request.urlopen(http_req, timeout=10) as response:
            res = json.loads(response.read().decode('utf-8'))
            if "error" in res:
                raise RuntimeError(f"MCP Tool '{tool_name}' error: {res['error']}")
            return res.get("result", {})

    def snapshot(self, mode: str = "rendered") -> str:
        res = self.call("autoui_snapshot", {"mode": mode})
        return res.get("content", [{}])[0].get("text", "")

    def press(self, element_id: str) -> str:
        clean_id = element_id.lstrip("#")
        res = self.call("autoui_action", {"element_id": clean_id, "action": "press"})
        return res.get("content", [{}])[0].get("text", "")

    def screenshot(self, name: str, save_path: str = None) -> str:
        args = {"name": name, "baseline": True}
        if save_path:
            args["save_path"] = save_path
        res = self.call("autoui_screenshot", args)
        return res.get("content", [{}])[0].get("text", "")

def main():
    app_dir = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
    os.makedirs(SCREENSHOTS_DIR, exist_ok=True)
    
    port = pick_free_port()
    env = dict(os.environ, AUTOUI_MCP_PORT=str(port))
    
    print(f"[*] Starting VM mode for Klondike on port {port}...")
    print(f"[*] App dir: {app_dir}")
    
    proc = subprocess.Popen(
        ["auto", "run", "-r", "vm"],
        cwd=app_dir,
        env=env,
        stdout=subprocess.PIPE,
        stderr=subprocess.PIPE
    )
    
    try:
        client = AutoUiMcpClient(port)
        ready = False
        start_time = time.time()
        timeout = 30
        
        timeout = 60
        print(f"[*] Waiting for MCP server and UI render (timeout: {timeout}s)...")
        while time.time() - start_time < timeout:
            try:
                snap = client.snapshot()
                # Wait until the actual UI tree is rendered (not the "not available" message)
                if snap and ("tree:" in snap or "Col" in snap or "Row" in snap or "Button" in snap):
                    ready = True
                    break
                elif snap:
                    print(f"    [~] UI not ready yet: {snap[:80]}")
            except Exception as e:
                pass
            time.sleep(1.0)
        
        if not ready:
            print("[-] Timeout waiting for UI render!", file=sys.stderr)
            # Print last known snapshot for debugging
            try:
                snap = client.snapshot()
                print(f"    Last snapshot: {snap[:200]}", file=sys.stderr)
            except:
                pass
            sys.exit(1)
        
        print("[+] VM UI Ready!")
        
        # Get snapshot to understand the UI tree
        snap = client.snapshot()
        print("\n--- AURA Tree (first 2000 chars) ---")
        print(snap[:2000])
        print("---\n")
        
        # Save snapshot to file for analysis
        snap_path = os.path.join(SCREENSHOTS_DIR, "vm_snapshot.txt")
        with open(snap_path, "w", encoding="utf-8") as f:
            f.write(snap)
        print(f"[+] Snapshot saved to: {snap_path}")
        
        # Screenshot 1: Initial state
        save_path_1 = os.path.join(SCREENSHOTS_DIR, "vm_initial.png")
        res = client.screenshot("klondike_vm_initial", save_path=save_path_1)
        print(f"[+] Initial screenshot: {res}")
        
        # Wait a moment then capture again to see if timer updated
        time.sleep(2)
        save_path_2 = os.path.join(SCREENSHOTS_DIR, "vm_after_2s.png")
        res = client.screenshot("klondike_vm_after_2s", save_path=save_path_2)
        print(f"[+] After-2s screenshot: {res}")
        
        # Try clicking the stock pile (first clickable element - likely a button)
        print("[*] Trying to find Stock pile button...")
        snap2 = client.snapshot()
        
        # Look for button IDs in snapshot
        lines = snap2.split('\n')
        button_ids = []
        for line in lines:
            if 'Button' in line and 'id=' in line:
                # Extract id
                for part in line.split():
                    if part.startswith('id='):
                        button_ids.append(part.replace('id=', '').strip('"'))
        
        if button_ids:
            print(f"[*] Found buttons: {button_ids[:5]}")
            # Try clicking first button (Stock pile)
            first_btn = button_ids[0]
            print(f"[*] Pressing first button: {first_btn}")
            press_res = client.press(first_btn)
            print(f"[+] Press result: {press_res}")
            time.sleep(0.5)
        
        # Screenshot after click
        save_path_3 = os.path.join(SCREENSHOTS_DIR, "vm_after_click.png")
        res = client.screenshot("klondike_vm_after_click", save_path=save_path_3)
        print(f"[+] After-click screenshot: {res}")
        
        print(f"\n[+] All VM screenshots saved to: {SCREENSHOTS_DIR}")
        print(f"    - {save_path_1}")
        print(f"    - {save_path_2}")
        print(f"    - {save_path_3}")
        
    finally:
        print("[*] Terminating VM process...")
        proc.terminate()
        try:
            proc.wait(timeout=5)
        except subprocess.TimeoutExpired:
            proc.kill()

if __name__ == "__main__":
    main()
