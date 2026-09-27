#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""PLAN-045 T-07 A/B runner：sidebar 切页计时（mouse_event 通道版）。

用法：python tmp/p045_run_ab.py <tag>
  tag=A → memo ON 语料；tag=B → memo OFF 语料（由外层先改语料）。
每次点击后等宿主 [VM-VIEW] widget=App 行（毫秒 wall 前缀），取其后首个
build_ms 即本次导航的主线程阻塞（VM 整树重解释时延，0927 口径）。
逐页截图 tmp/p045_<tag>_<page>.png 供 A/B 产物对拍。
"""
import ctypes, io, json, os, sys, time
from ctypes import wintypes
from PIL import ImageGrab

user32 = ctypes.windll.user32
user32.SetProcessDPIAware()

hwnd = int(open(os.path.join(os.path.dirname(__file__), "p045_hwnd.txt")).read())
_pid = wintypes.DWORD()
user32.GetWindowThreadProcessId(hwnd, ctypes.byref(_pid))
MY_PID = _pid.value
LOG = os.path.join(os.path.dirname(__file__), "p045_host.log")

POINTS = [("Row", 478, 873), ("Column", 500, 946), ("Center", 495, 1018),
          ("Flex", 474, 1090), ("Alignment", 518, 1162), ("Absolute", 508, 1234),
          ("Home", 585, 713)]

def me_move(x, y):
    user32.mouse_event(0x0001 | 0x8000, int(x * 65535 / 3839), int(y * 65535 / 2559), 0, 0)

def me_click(x, y):
    me_move(x, y)
    time.sleep(0.08)
    user32.mouse_event(0x0002, 0, 0, 0, 0)
    time.sleep(0.05)
    user32.mouse_event(0x0004, 0, 0, 0, 0)

def window_at(x, y):
    pt = wintypes.POINT(x, y)
    hw = user32.WindowFromPoint(pt)
    pid = wintypes.DWORD()
    user32.GetWindowThreadProcessId(hw, ctypes.byref(pid))
    return pid.value

def wait_app_build(t0, timeout=30.0):
    end = time.time() + timeout
    while time.time() < end:
        with io.open(LOG, encoding="utf-8", errors="replace") as f:
            for ln in f:
                if ln.startswith("[") and "VM-VIEW] widget=App" in ln:
                    ts = int(ln[1:ln.index("]")])
                    if ts >= t0 - 50:
                        ms = ln.split("build_ms=")[1].split(" ")[0]
                        return ts, int(ms)
        time.sleep(0.1)
    return None, None

def main(tag):
    results = []
    for name, x, y in POINTS:
        ok = False
        for attempt in range(5):
            user32.SetWindowPos(hwnd, -1, 0, 0, 3840, 2560, 0x0040)
            user32.SetForegroundWindow(hwnd)
            time.sleep(0.3)
            if window_at(x, y) != MY_PID:
                print(f"{name}: window not mine, retry", flush=True)
                continue
            t0 = int(time.time() * 1000)
            me_click(x, y)
            ts, ms = wait_app_build(t0)
            if ts is None:
                print(f"{name}: no build, attempt {attempt+1}", flush=True)
                time.sleep(1.5)
                continue
            time.sleep(1.2)
            ImageGrab.grab(all_screens=True).crop((350, 380, 1950, 1330)).save(
                os.path.join(os.path.dirname(__file__), f"p045_{tag}_{name}.png"))
            print(f"{name}: build_wait={ts-t0}ms block_ms={ms}", flush=True)
            results.append({"page": name, "click_to_build": ts - t0, "block_ms": ms})
            ok = True
            break
        if not ok:
            results.append({"page": name, "click_to_build": None, "block_ms": None})
    # 收尾回 Home（若最后点的是 Home 已在）
    out = os.path.join(os.path.dirname(__file__), f"p045_{tag}_result.json")
    json.dump(results, io.open(out, "w", encoding="utf-8"))
    print("SAVED", out, flush=True)

if __name__ == "__main__":
    main(sys.argv[1] if len(sys.argv) > 1 else "A")
