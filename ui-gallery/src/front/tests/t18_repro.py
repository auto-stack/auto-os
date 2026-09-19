"""P642 T-18 复现驱动：027 打开(等 toast) → 切 008 → 时间序列截图观察 toast 残留。"""
import re
import sys
import time

sys.path.insert(0, r"D:/autostack/.wt/lang-642/auto-lang/.agents/skills/autoui-verifier/scripts")
from test_vm_mcp import AutoUiMcpClient  # noqa: E402


def find_id(snap, label):
    for line in snap.splitlines():
        if label in line:
            m = re.search(r"(aura_\d+|vnode_\d+)", line)
            if m:
                return m.group(0)
    return None


def main():
    c = AutoUiMcpClient(int(sys.argv[1]))
    snap = c.snapshot()
    if "045-style-import" not in snap:
        print("[!] not ready"); return 2
    id027 = find_id(snap, "027-file-manager")
    id008 = find_id(snap, "008-pricing-table")
    print(f"[*] ids 027={id027} 008={id008}")
    c.press(id027); time.sleep(3.0)
    print("    shot A (on 027, +3s)")
    c.screenshot("t18_A_on027", baseline=False)
    c.press(id008)
    for wait, tag in [(1, "B1"), (5, "B5"), (10, "B10")]:
        time.sleep(wait if tag == "B1" else wait - (1 if tag == "B5" else 5))
        snap2 = c.snapshot()
        # toast 文本是否仍在渲染树
        hit = "无法定位主目录" in snap2 or "正在打开" in snap2
        print(f"    shot {tag} (+{wait}s on 008): toast-in-snapshot={hit}")
        c.screenshot(f"t18_{tag}_on008", baseline=False)
    print("[*] done")
    return 0


if __name__ == "__main__":
    sys.exit(main())
