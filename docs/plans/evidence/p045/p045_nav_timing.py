#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""PLAN-045 T-07：widgets-gallery sidebar 切页计时（0927 计时法固化）。

子命令：
  stamp            — stdin 行打点（单调毫秒前缀）→ stdout（宿主 stderr 计时面）
  bus <verb>       — MCP bus 调用（launch/focus/close/layout）
  shot <path>      — PIL 全屏截屏（autoui_screenshot 最大化态误报，PIL 兜底）
  click <x> <y>    — SendInput 物理像素点击（SetProcessDPIAware）
  analyze <clicks.json> <host.log> — 逐点击：stderr 静默窗 = [click, click 后首行]
                                     输出每次导航主线程阻塞毫秒表

物理分辨率 3840x2560（2x DPI）——click 传物理像素。
"""
import ctypes, io, json, struct, sys, time, urllib.request

user32 = ctypes.windll.user32
kernel32 = ctypes.windll.kernel32

def now_ms():
    # wall clock——stamper 与 click 是两个进程，perf_counter 纪元不同；
    # time.time 同源（毫秒精度足够）。
    return int(time.time() * 1000)

def dpi_aware():
    try:
        user32.SetProcessDPIAware()
    except Exception:
        pass

# ---------------------------------------------------------------- stamp
def cmd_stamp():
    for line in sys.stdin:
        sys.stdout.write(f"[{now_ms()}] {line}")
        sys.stdout.flush()

# ---------------------------------------------------------------- bus
def mcp_call(tool: str, args: dict):
    payload = json.dumps({
        "jsonrpc": "2.0", "id": 1, "method": "tools/call",
        "params": {"name": tool, "arguments": args},
    }).encode()
    req = urllib.request.Request(
        f"http://127.0.0.1:{PORT}/mcp", data=payload,
        headers={"Content-Type": "application/json"}, method="POST")
    with urllib.request.urlopen(req, timeout=30) as r:
        return r.read().decode()

def cmd_bus(verb: str):
    out = mcp_call("autoui_desktop", {"action": "bus", "verb": verb})
    print(out[:2000])

# ---------------------------------------------------------------- shot
def cmd_shot(path: str):
    from PIL import ImageGrab
    dpi_aware()
    img = ImageGrab.grab(all_screens=True)
    img.save(path)
    print(f"saved {path} {img.size}")

# ---------------------------------------------------------------- click
def send_click(x: int, y: int):
    dpi_aware()
    INPUT_MOUSE, MOUSEEVENTF_MOVE, MOUSEEVENTF_ABSOLUTE = 0, 0x0001, 0x8000
    MOUSEEVENTF_LEFTDOWN, MOUSEEVENTF_LEFTUP = 0x0002, 0x0004
    sw, sh = user32.GetSystemMetrics(0), user32.GetSystemMetrics(1)
    ax = int(x * 65535 / (sw - 1))
    ay = int(y * 65535 / (sh - 1))

    class MOUSEINPUT(ctypes.Structure):
        _fields_ = [("dx", ctypes.c_long),
                    ("dy", ctypes.c_long), ("mouseData", ctypes.c_ulong),
                    ("dwFlags", ctypes.c_ulong), ("time", ctypes.c_ulong),
                    ("dwExtraInfo", ctypes.POINTER(ctypes.c_ulong))]

    class _INPUTUNION(ctypes.Union):
        _fields_ = [("mi", MOUSEINPUT), ("pad", ctypes.c_ulonglong * 32)]

    class INPUT(ctypes.Structure):
        _fields_ = [("type", ctypes.c_ulong), ("union", _INPUTUNION)]

    def mk(flags):
        i = INPUT()
        i.type = INPUT_MOUSE
        i.union.mi.dx, i.union.mi.dy = ax, ay
        i.union.mi.dwFlags = flags | MOUSEEVENTF_ABSOLUTE
        return i

    arr = (INPUT * 3)(mk(MOUSEEVENTF_MOVE), mk(MOUSEEVENTF_LEFTDOWN),
                      mk(MOUSEEVENTF_LEFTUP))
    user32.SendInput(3, arr, ctypes.sizeof(INPUT))
    print(f"clicked {x},{y} @ {now_ms()}")

def cmd_click(x: int, y: int, record: str | None):
    send_click(x, y)
    if record:
        with io.open(record, "a", encoding="utf-8") as f:
            f.write(json.dumps({"t": now_ms(), "x": x, "y": y}) + "\n")

# ---------------------------------------------------------------- analyze
def cmd_analyze(clicks_path: str, log_path: str):
    clicks = [json.loads(l) for l in io.open(clicks_path, encoding="utf-8") if l.strip()]
    lines = []
    for l in io.open(log_path, encoding="utf-8", errors="replace"):
        if l.startswith("["):
            try:
                ts = int(l[1:l.index("]")])
                lines.append((ts, l[l.index("]") + 1:].strip()[:120]))
            except ValueError:
                pass
    print(f"clicks={len(clicks)} log_lines={len(lines)}")
    rows = []
    for i, c in enumerate(clicks):
        end = clicks[i + 1]["t"] if i + 1 < len(clicks) else c["t"] + 60_000
        after = [ts for ts, _ in lines if ts >= c["t"]]
        block_end = after[0] if after else end
        # 静默窗结束于点击后首条 stderr；阻塞时长 = 首条 - 点击（含队列延迟）。
        rows.append((c, block_end - c["t"]))
    print(f"{'click':>16} {'block_ms':>10}")
    for c, ms in rows:
        print(f"{c['x']},{c['y']:>10} {ms:>10}")
    bl = [ms for _, ms in rows]
    if bl:
        print(f"max={max(bl)}ms median={sorted(bl)[len(bl)//2]}ms")

PORT = 9471

def main():
    global PORT
    args = sys.argv[1:]
    if args and args[0] == "stamp":
        cmd_stamp()
        return
    if not args:
        print(__doc__)
        return
    cmd = args[0]
    if cmd == "bus":
        cmd_bus(args[1])
    elif cmd == "shot":
        cmd_shot(args[1])
    elif cmd == "click":
        rec = None
        if "--record" in args:
            rec = args[args.index("--record") + 1]
        cmd_click(int(args[1]), int(args[2]), rec)
    elif cmd == "analyze":
        cmd_analyze(args[1], args[2])
    else:
        print(__doc__)

if __name__ == "__main__":
    main()
