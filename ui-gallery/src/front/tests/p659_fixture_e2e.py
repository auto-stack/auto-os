"""P659 T-05/T-06 E2E：fixture trigger 双形态派发 + canonical 哨兵语义。

腿目（对应 AC-04/AC-05）：
  A. handler 形态正例——state{addr:D:/, addr_editing:true}+trigger{handler:AddrGo}
     → addr_editing 翻 false、current_path 落 canonical（NavTo 真导航）。
  B. canonical 哨兵——state{addr:非法路径}+trigger{handler:AddrGo}
     → toast.error "无法打开"（can == "" 判空分支，AC-05 核心）。
  C. handler 未命中响亮报错——trigger{handler:NoSuch} → ack error
     handler_not_found（此前静默无操作，AC-04）。
  D. widget 形态正例——trigger{widget:Demo027FileManager, event:AddrEdit}
     → addr_editing true（既有形态不回归）。
  E. widget 未命中响亮报错——event:Bogus → ack error handler_not_found。

启动：AUTOUI_TEST_FIXTURES=1（tool_fixture 门控）。
"""
import re
import subprocess
import sys
import time
from pathlib import Path

HERE = Path(__file__).resolve().parent
GROUP = HERE.parents[4]                      # .wt/lang-659
AUTO_LANG_WT = GROUP / "auto-lang"
AUTO_BIN = AUTO_LANG_WT / "target" / "debug" / "auto.exe"
GALLERY_DIR = HERE.parents[2]

for _cand in (AUTO_LANG_WT, Path("D:/autostack/auto-lang")):
    _scripts = _cand / ".agents" / "skills" / "autoui-verifier" / "scripts"
    if (_scripts / "test_vm_mcp.py").exists():
        sys.path.insert(0, str(_scripts))
        break
from test_vm_mcp import AutoUiMcpClient  # noqa: E402


def pick_free_port() -> int:
    import socket
    with socket.socket(socket.AF_INET, socket.SOCK_STREAM) as s:
        s.bind(("127.0.0.1", 0))
        return s.getsockname()[1]


def find_id(snap, needle):
    for line in snap.splitlines():
        if needle in line:
            m = re.search(r"(aura_\d+|vnode_\d+)", line)
            if m:
                return m.group(0)
    return None


def state_text(c, fields=None):
    try:
        return c.state(fields)
    except Exception as e:
        return f"<state error: {e}>"


def main():
    import os
    port = pick_free_port()
    env = dict(
        os.environ,
        AUTOUI_MCP_PORT=str(port),
        AUTOUI_TEST_FIXTURES="1",
        AUTO_GALLERY_APPS=str(AUTO_LANG_WT / "examples" / "ui"),
    )
    proc = subprocess.Popen(
        [str(AUTO_BIN), "run", "-r", "vm"],
        cwd=str(GALLERY_DIR), env=env,
        stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL,
        stdin=subprocess.DEVNULL,
        creationflags=subprocess.CREATE_NO_WINDOW)
    print(f"[*] gallery pid={proc.pid} port={port}")
    c = AutoUiMcpClient(port)
    t0 = time.time()
    ready = False
    while time.time() - t0 < 300:
        if proc.poll() is not None:
            print("[!] gallery exited during boot")
            return 2
        try:
            if "045-style-import" in c.snapshot():
                ready = True
                break
        except Exception:
            pass
        time.sleep(2.0)
    if not ready:
        proc.kill()
        print("[!] mcp ready timeout")
        return 2

    id027 = find_id(c.snapshot(), "027-file-manager")
    c.press(id027)
    time.sleep(2.5)
    failures = []

    def check(name, cond, detail=""):
        print(f"[{'PASS' if cond else 'FAIL'}] {name} {detail}")
        if not cond:
            failures.append(name)

    # A. handler 形态正例：AddrGo 真导航。
    res = c.fixture(
        state={"addr": "D:/", "addr_editing": True},
        trigger={"handler": "AddrGo"})
    print(f"[*] A receipt: {res}")
    time.sleep(0.8)
    st = state_text(c)
    a_ok = ("addr_editing: false" in st or "addr_editing=false" in st
            or '"addr_editing": false' in st or "addr_editing False" in st)
    check("A handler-form AddrGo applied", '"status": "applied"' in str(res)
          or "applied" in str(res), f"state addr_editing hit={a_ok}")
    check("A addr_editing flipped false", a_ok,
          st[:160].replace("\n", " "))

    # B. canonical 哨兵：非法路径 → 无法打开 toast。
    res = c.fixture(
        state={"addr": "Z:/::p659-no-such-path"},
        trigger={"handler": "AddrGo"})
    time.sleep(0.8)
    snap = c.snapshot()
    check("B invalid path toast 无法打开", "无法打开" in snap)
    check("B receipt applied (handler ran)", "applied" in str(res))

    # C. handler 未命中 → 响亮报错。
    res = c.fixture(
        state={"addr": "D:/"},
        trigger={"handler": "NoSuchHandlerP659"})
    print(f"[*] C receipt: {res}")
    check("C unknown handler loud error",
          "handler_not_found" in str(res), str(res)[:120])

    # D. widget 形态正例：AddrEdit。
    res = c.fixture(
        state={"addr": "D:/"},
        trigger={"widget": "Demo027FileManager", "event": "AddrEdit"})
    time.sleep(0.8)
    st = state_text(c)
    d_ok = ("addr_editing: true" in st or "addr_editing=true" in st
            or '"addr_editing": true' in st or "addr_editing True" in st)
    check("D widget-form AddrEdit applied", "applied" in str(res))
    check("D addr_editing flipped true", d_ok)

    # E. widget 未命中 → 响亮报错。
    res = c.fixture(
        state={"addr": "D:/"},
        trigger={"widget": "Demo027FileManager", "event": "Bogus"})
    print(f"[*] E receipt: {res}")
    check("E unknown widget-event loud error",
          "handler_not_found" in str(res), str(res)[:120])

    c.screenshot("p659_fixture_e2e_final", baseline=False)
    subprocess.run(["taskkill", "/F", "/T", "/PID", str(proc.pid)],
                   capture_output=True)
    print(f"[*] done; failures={failures}")
    return 0 if not failures else 1


if __name__ == "__main__":
    sys.exit(main())
