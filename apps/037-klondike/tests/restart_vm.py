import os, subprocess, time, json, urllib.request, re

# kill leftover VM auto processes
try:
    out = subprocess.run(
        ["powershell", "-Command",
         "Get-CimInstance Win32_Process -Filter \"Name='auto.exe'\" | ForEach-Object { \"$($_.ProcessId) $($_.CommandLine)\" }"],
        capture_output=True, text=True
    ).stdout
    for line in out.splitlines():
        if "run" in line and "-r" in line and "vm" in line:
            pid = line.split()[0]
            print("kill", pid, line[:100])
            subprocess.run(["taskkill", "/F", "/PID", pid], capture_output=True)
except Exception as e:
    print("kill_err", e)

time.sleep(1)
app = r"D:\autostack\auto-os\apps\037-klondike"
auto = r"D:\autostack\auto-lang\target\debug\auto.exe"
logdir = os.path.join(app, "src", "front", "tmp")
os.makedirs(logdir, exist_ok=True)
outf = open(os.path.join(logdir, "vm_run_out.log"), "w")
errf = open(os.path.join(logdir, "vm_run_err.log"), "w")
env = os.environ.copy()
env["AUTOUI_MCP_PORT"] = "17699"
p = subprocess.Popen(
    [auto, "run", "-r", "vm"],
    cwd=app, env=env, stdout=outf, stderr=errf,
    creationflags=0x00000008 | 0x00000200,
)
print("STARTED_PID", p.pid)

url = "http://127.0.0.1:17699/mcp"
ready = False
for i in range(45):
    time.sleep(1)
    try:
        req = urllib.request.Request(
            url,
            json.dumps({"jsonrpc": "2.0", "id": 1, "method": "tools/list", "params": {}}).encode(),
            {"Content-Type": "application/json"},
        )
        urllib.request.urlopen(req, timeout=2)
        print("MCP_READY", i + 1)
        ready = True
        break
    except Exception:
        pass
if not ready:
    print("MCP_TIMEOUT")
    raise SystemExit(1)

time.sleep(3)

def call(tool, args):
    req = urllib.request.Request(
        url,
        json.dumps({"jsonrpc": "2.0", "id": 1, "method": "tools/call", "params": {"name": tool, "arguments": args}}).encode(),
        {"Content-Type": "application/json"},
    )
    with urllib.request.urlopen(req, timeout=15) as r:
        return json.loads(r.read())

path = os.path.join(app, "tests", "screenshots", "vm_visual_hit_v5.png")
res = call("autoui_screenshot", {"name": "klondike_vm_visual_hit_v5", "save_path": path})
print("SHOT", json.dumps(res, ensure_ascii=False)[:220])
st = call("autoui_state", {"fields": ["sel_source", "sel_col", "sel_idx", "game_state"]})
print("STATE", json.dumps(st, ensure_ascii=False)[:300])
