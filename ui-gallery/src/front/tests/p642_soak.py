"""P642 F1 崩溃族定向 soak：2 轮全遍历 + 逐条截图 + 029 双开 + 空闲段。

复现画像对齐 R642-F1：长会话（2+ 轮全遍历 + 截图），复现率 ≈1/4 实例。
退出码/审计日志判读（127 枚举小步成果）：
  bash 139 = access violation；bash 127 = fastfail(0xC0000409)/栈溢出(0xC00000FD)/高位截断；
  死亡时刻审计日志有 code=101 行 = Rust panic（非 F1 型）；零新增行 = 绕过 panic 钩子（栈溢出/原生 fastfail）。
"""
import re
import sys
import time
from pathlib import Path

# P659：组内 auto-lang 优先（.wt/lang-<NNN> 组），回退主检出
for _cand in (Path(__file__).resolve().parents[4] / "auto-lang",
              Path("D:/autostack/auto-lang")):
    _scripts = _cand / ".agents" / "skills" / "autoui-verifier" / "scripts"
    if (_scripts / "test_vm_mcp.py").exists():
        sys.path.insert(0, str(_scripts))
        break
from test_vm_mcp import AutoUiMcpClient  # noqa: E402


def sidebar_ids(snap: str):
    ids = {}
    for line in snap.splitlines():
        m = re.search(r"(aura_\d+|vnode_\d+)", line)
        label = re.search(r"\b(\d{3}-[a-z][a-z0-9-]*)\b", line)
        if m and label and label.group(1) not in ids:
            ids[label.group(1)] = m.group(0)
    return ids


def main():
    port = int(sys.argv[1])
    c = AutoUiMcpClient(port)
    snap = c.snapshot()
    if "045-style-import" not in snap:
        print("[!] gallery not ready")
        return 2
    ids = sidebar_ids(snap)
    print(f"[*] {len(ids)} sidebar entries")
    order = sorted(ids)
    for rnd in (1, 2):
        for label in order:
            t0 = time.strftime("%H:%M:%S")
            try:
                c.press(ids[label])
                time.sleep(0.6)
                c.screenshot(f"soak_r{rnd}_{label.replace('-', '_')}", baseline=False)
            except Exception as e:
                print(f"[!] r{rnd} {label}: {e}")
                return 3
            print(f"  r{rnd} {t0} {label}")
        # 029 双开（R642-F1 原触发序列）
        if "029-photo-gallery" in ids:
            c.press(ids["029-photo-gallery"]); time.sleep(1.0)
            c.press(ids["008-pricing-table"] if "008-pricing-table" in ids else ids[order[0]]); time.sleep(0.5)
            c.press(ids["029-photo-gallery"]); time.sleep(1.0)
            print(f"  r{rnd} 029 double-open done")
    print("[*] idle 20s ...")
    time.sleep(20)
    snap2 = c.snapshot()
    print(f"[*] survived; final snapshot {len(snap2)} chars")
    return 0


if __name__ == "__main__":
    sys.exit(main())
