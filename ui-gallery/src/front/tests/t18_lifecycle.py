"""P642 T-18 toast 生命周期 E2E（fixture 版，v2）：挂载 027 → fixture 置
.addr=非法路径 + 直派 AddrGo → toast.error → 立即切 008 → +1s 可见（瞬态
设计内）→ +6s 过期消失。验证 Plan 412 修正 5 订阅到期机制跨 demo 正常。"""
import re
import sys
import time
from pathlib import Path

for _cand in (Path(__file__).resolve().parents[4] / "auto-lang",
              Path("D:/autostack/auto-lang")):
    _scripts = _cand / ".agents" / "skills" / "autoui-verifier" / "scripts"
    if (_scripts / "test_vm_mcp.py").exists():
        sys.path.insert(0, str(_scripts))
        break
from test_vm_mcp import AutoUiMcpClient  # noqa: E402

TOAST_TEXTS = ("无法打开", "不是目录", "无法定位")


def find_id(snap, needle):
    for line in snap.splitlines():
        if needle in line:
            m = re.search(r"(aura_\d+|vnode_\d+)", line)
            if m:
                return m.group(0)
    return None


def toast_hit(snap):
    return [t for t in TOAST_TEXTS if t in snap]


def main():
    c = AutoUiMcpClient(int(sys.argv[1]))
    snap = c.snapshot()
    if "045-style-import" not in snap:
        print("[!] not ready"); return 2
    id027 = find_id(snap, "027-file-manager")
    id008 = find_id(snap, "008-pricing-table")

    c.press(id027); time.sleep(2.5)

    res = c.fixture(
        state={"addr": "Z:/::p642-no-such-path"},
        trigger={"widget": "Demo027FileManager", "event": "AddrGo", "input": None},
    )
    print(f"[*] fixture receipt: {res}")
    time.sleep(0.8)
    fired = toast_hit(c.snapshot())
    print(f"[*] toast fired on 027: {fired}")
    if not fired:
        print("[!] 触发失败"); return 3

    c.press(id008)
    time.sleep(1.0)
    hit1 = toast_hit(c.snapshot())
    c.screenshot("t18_L1_on008_toast_visible", baseline=False)
    print(f"[*] +1s on 008: {hit1} (预期非空)")
    time.sleep(6.0)
    hit6 = toast_hit(c.snapshot())
    c.screenshot("t18_L6_on008_toast_expired", baseline=False)
    print(f"[*] +7s on 008: {hit6} (预期空)")
    ok = bool(hit1) and not hit6
    print("[*] LIFECYCLE OK" if ok else "[!] 生命周期异常")
    return 0 if ok else 4


if __name__ == "__main__":
    sys.exit(main())
