"""P642 T-13①b 主题污染 A/B 追踪驱动。

序列：008(深) → 016 → 008，每步截图；配合 AUTO_DEBUG_THEME_TRACE=dark_mode
探针日志（进程 stderr）定位根态 dark_mode 的真实写入者。

用法：python t13_ab.py --port <mcp_port> [--shotdir <dir>]
"""
import argparse
import re
import sys
import time
import urllib.request

sys.path.insert(0, r"D:/autostack/.wt/lang-642/auto-lang/.agents/skills/autoui-verifier/scripts")
from test_vm_mcp import AutoUiMcpClient  # noqa: E402


def find_sidebar_id(snapshot: str, label: str) -> str | None:
    """在渲染树快照里找含 label 的行，取其元素 id（aura_N/vnode_N）。"""
    for line in snapshot.splitlines():
        if label in line:
            m = re.search(r"(?:aura_\d+|vnode_\d+)", line)
            if m:
                return m.group(0)
    return None


def main() -> int:
    ap = argparse.ArgumentParser()
    ap.add_argument("--port", type=int, required=True)
    ap.add_argument("--shotdir", default=r"D:/autostack/.wt/lang-642/auto-os/ui-gallery/src/front/tests/screenshots")
    args = ap.parse_args()

    c = AutoUiMcpClient(args.port)

    # 就绪判定：snapshot 含 045-style-import（§8.3 约定）
    snap = c.snapshot()
    if "045-style-import" not in snap:
        print("[!] 未就绪：快照缺 045-style-import")
        return 2
    with open(args.shotdir + "/t13_ab_snap0.txt", "w", encoding="utf-8") as f:
        f.write(snap)

    id008 = find_sidebar_id(snap, "008-pricing-table")
    id016 = find_sidebar_id(snap, "016-calendar")
    print(f"[*] sidebar ids: 008={id008} 016={id016}")
    if not id008 or not id016:
        print("[!] 找不到侧栏条目 id")
        return 2

    def step(tag: str, eid: str, settle: float = 1.2):
        print(f"[*] {time.strftime('%H:%M:%S')} press {tag} ({eid})")
        c.press(eid)
        time.sleep(settle)
        res = c.screenshot(f"p642_{tag}", baseline=False, save_path=args.shotdir)
        print(f"    shot: {res[:120]}")

    step("t13a_008_dark_before", id008, 2.0)
    step("t13b_016_open", id016, 2.5)
    step("t13c_008_after", id008, 2.0)
    print("[*] done — 对照探针日志 [P642T13] 行")
    return 0


if __name__ == "__main__":
    sys.exit(main())
