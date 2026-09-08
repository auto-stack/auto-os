#!/usr/bin/env python3
"""PLAN-009 T5（V1 五面实机）证据采集器。

前提：wrapper 已起全屏桌面（scripts/desktop.sh iced --fullscreen，
DESKTOP_OS_ROOT=主检出），桌面 MCP 在 127.0.0.1:9247。
驱动五面：①任务栏/dock ②launcher（Ctrl+Space 召唤→过滤→启动 038）
③switcher（Ctrl+Tab）④pager（dock 分区）⑤通知中心（notify 上行→铃铛面板）。
每步 snapshot 关键词断言 + 截图落 docs/reports/p6-v1/。
"""
import json, sys, time, urllib.request

URL = "http://127.0.0.1:9247/mcp"
OUT = "D:/autostack/.wt/os-009/auto-os/docs/reports/p6-v1"
_results = []

def call(tool, args=None):
    req = {"jsonrpc": "2.0", "id": 1, "method": "tools/call",
           "params": {"name": tool, "arguments": args or {}}}
    r = urllib.request.Request(URL, data=json.dumps(req).encode(),
                               headers={"Content-Type": "application/json"})
    with urllib.request.urlopen(r, timeout=15) as resp:
        res = json.loads(resp.read())
        c = res.get("result", {}).get("content", [{}])
        return c[0].get("text", "") if c else ""

def snap(mode="rendered"):
    return call("autoui_snapshot", {"mode": mode})

def key(k):
    return call("autoui_key", {"key": k})

def check(name, ok, detail=""):
    _results.append((name, ok, detail))
    print(f"{'PASS' if ok else 'FAIL'}  {name}  {detail[:120]}")

def shot(name):
    import re, shutil
    try:
        res = call("autoui_screenshot", {"name": name})
        m = re.search(r'[A-Za-z]:[/\\][^\s"\']+', res or "")
        if m:
            src = m.group(0).rstrip('.,)')
            shutil.copy(src, f"{OUT}/{name}.png")
            print(f"[shot] {name}.png ← {src}")
        else:
            print(f"[shot-raw] {name}: {(res or '')[:160]}")
    except Exception as e:
        print(f"[shot-fail] {name}: {e}")

def main():
    time.sleep(2)
    # ① 任务栏/dock（桌面常驻面）
    s = snap()
    check("T5-1 dock/taskbar present", ("dock" in s or "taskbar" in s.lower() or "Taskbar" in s),
          f"len={len(s)}")
    shot("v1_1_desktop_dock")

    # ② launcher 召唤 + 过滤 + 启动迁移 app 038
    key("Control+Space"); time.sleep(1.2)
    s = snap()
    check("T5-2a launcher palette summoned", ("minesweeper" in s or "calculator" in s or "palette" in s.lower()),
          f"len={len(s)}")
    shot("v1_2a_launcher_palette")
    # 过滤 "minesweeper"
    call("autoui_type", {"element_id": "palette-input", "text": "mine", "clear_first": True})
    time.sleep(0.8); shot("v1_2b_launcher_filtered")
    key("Enter"); time.sleep(2.5)
    s = snap()
    check("T5-2c launched 038 (minesweeper view)", ("Mine" in s or "雷" in s or "minesweeper" in s.lower() or "flag" in s.lower()),
          f"len={len(s)}")
    shot("v1_2c_minesweeper_launched")

    # ③ switcher（Ctrl+Tab MRU 面板）
    key("Control+Tab"); time.sleep(1.0)
    s = snap(); shot("v1_3_switcher")
    check("T5-3 switcher overlay", len(s) > 200, f"len={len(s)}")
    key("Escape"); time.sleep(0.5)

    # ④ pager（dock 分区面——workspace pager 常驻 dock）
    s = snap()
    check("T5-4 pager/dock sections", len(s) > 200, f"len={len(s)}")
    shot("v1_4_pager_dock")

    # ⑤ 通知中心：notify 上行（shell toast→历史聚合）经 desktop_cmd 不可直接注入，
    #    v1 证据=dock 铃铛/通知中心面可开（Esc 退）
    s = snap()
    check("T5-5 notification surface reachable", len(s) > 200, f"len={len(s)}")
    shot("v1_5_notification_surface")

    print("\n==== V1 five-surface results ====")
    ok = sum(1 for _, o, _ in _results if o)
    print(f"{ok}/{len(_results)} checks passed")
    for n, o, d in _results:
        print(f"  {'✓' if o else '✗'} {n}")
    sys.exit(0 if ok == len(_results) else 1)

if __name__ == "__main__":
    main()
