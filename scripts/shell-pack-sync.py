#!/usr/bin/env python3
"""shell-pack-sync.py — Stage B P-7 hash-lock 同步契约（Design 01 §4-P7 步 3）。

本仓 `shell/` 四件为唯一真相源；auto-lang `crates/auto-lang/assets/` 内嵌
副本是 pin 快照（include_str! 编译期回退）。契约：

- 单向同步：auto-os → auto-lang（**永不反向**——反向会把 pin 快照的漂移
  洗成权威）；
- 缺省 = 校验模式：四件逐对 sha256 比对，全等 exit 0；任何漂移 exit 1
  红灯并列出差异件（双源静默漂移守卫）；
- `--sync` = 同步模式：把本仓四件覆盖 auto-lang 快照，打印 pin 版本注记
  （同步提交 message 用）；同步后仍需在 auto-lang 仓提交快照变更。

auto-lang 根解析序（沿两仓 AGENTS 解析序家族）：
`AUTO_LANG_ROOT` env → 兄弟 `../auto-lang` → 主检出 `D:/autostack/auto-lang`。

用法：
  python scripts/shell-pack-sync.py          # 校验（红灯模式）
  python scripts/shell-pack-sync.py --sync   # 同步 + pin 注记
"""

import hashlib
import os
import shutil
import sys
from pathlib import Path

PACK = ["shell.at", "desktop.at", "switcher.at", "notification_center.at"]
HERE = Path(__file__).resolve().parent
OS_SHELL = HERE.parent / "shell"


def resolve_lang_root() -> Path:
    candidates = []
    if env := os.environ.get("AUTO_LANG_ROOT"):
        candidates.append(Path(env))
    candidates.append(HERE.parent.parent / "auto-lang")
    candidates.append(Path("D:/autostack/auto-lang"))
    for c in candidates:
        if (c / "crates" / "auto-lang" / "assets" / "shell.at").is_file():
            return c
    print("error: auto-lang root 未解析（AUTO_LANG_ROOT/兄弟/主检出均未命中）", file=sys.stderr)
    sys.exit(2)


def sha256(p: Path) -> str:
    return hashlib.sha256(p.read_bytes()).hexdigest()


def main() -> int:
    do_sync = "--sync" in sys.argv[1:]
    lang_assets = resolve_lang_root() / "crates" / "auto-lang" / "assets"
    drifted = []
    for name in PACK:
        src = OS_SHELL / name
        dst = lang_assets / name
        if not src.is_file():
            print(f"error: 权威源缺件 {src}", file=sys.stderr)
            return 2
        if not dst.is_file() or sha256(src) != sha256(dst):
            drifted.append((name, src, dst))
    if not drifted:
        pin = ",".join(f"{n}={sha256(OS_SHELL / n)[:10]}" for n in PACK)
        print(f"OK: 四件全等（pin {pin}）")
        return 0
    if not do_sync:
        print("DRIFT: 下列件与 pin 快照不一致（跑 --sync 对齐，或在 auto-os 侧回改权威源）:")
        for name, src, dst in drifted:
            have = sha256(dst)[:10] if dst.is_file() else "<缺件>"
            print(f"  {name}: 权威 {sha256(src)[:10]} vs 快照 {have}")
        return 1
    # 同步：单向 auto-os → auto-lang
    stamp = __import__("datetime").date.today().isoformat()
    for name, src, dst in drifted:
        shutil.copyfile(src, dst)
        print(f"synced: {name} -> {dst} ({sha256(src)[:10]})")
    pin = ",".join(f"{n}={sha256(OS_SHELL / n)[:10]}" for n in PACK)
    print(f"\npin 版本（{stamp}）: {pin}")
    print(f"提交注记建议: chore(shell): pin 快照同步 auto-os → auto-lang（{stamp}，{len(drifted)} 件）")
    return 0


if __name__ == "__main__":
    sys.exit(main())
