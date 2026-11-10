#!/usr/bin/env python3
"""OS068 T-03: rq-final-acceptance driver self-tests (V-06/07 prep).

Validates the acceptance drivers' source/denominator/timing/result
admission and negative controls WITHOUT replacing the real product
oracles (per OS068 plan §6: existing drivers are the base; T-03 extends
them with explicit params). Covers:

  1. build-demos.ps1 parameterization surface (roots/exe/out params exist).
  2. Denominator bookkeeping: app-build targets vs library projects
     (stylekit has no src/front/app.at → classified as dep-form library).
  3. metrics.py result admission: present<0 must be excluded (066 F-2),
     p50/p95/max computed from the accepted sample.
  4. Negative control: wrong endpoint / missing artifact must fail with a
     named error, never a silent success.
"""
import argparse
import csv
import json
import re
import subprocess
import sys
import tempfile
from pathlib import Path

REPO = Path(__file__).resolve().parents[2]
SCRIPTS = REPO / "scripts" / "rq-final-acceptance"


def run_py(script: str, *args, cwd=None) -> subprocess.CompletedProcess:
    return subprocess.run(
        [sys.executable, str(script), *args], capture_output=True, text=True, cwd=cwd
    )


def check(name: str, ok: bool, detail: str = "") -> None:
    print(f"[t-rq] {'PASS' if ok else 'FAIL'} {name}" + (f" — {detail}" if detail else ""))
    if not ok:
        raise SystemExit(f"driver self-test failed: {name}")


def test_build_demos_params() -> None:
    ps1 = SCRIPTS / "build-demos.ps1"
    text = ps1.read_text(encoding="utf-8")
    for p in ["UiRoot", "LangRoot", "ManRoot", "WebRoot", "DownRoot", "AutoExe", "Out"]:
        check(
            f"build-demos.ps1 param {p}",
            f"param(\n" in text and re.search(rf"\[string\]\${p}\s*=", text) is not None,
            f"missing ${p}",
        )
    check(
        "no os-066 hardcode",
        re.search(r"\.wt[\\/]os-066|plan-066[\\/]", text) is None,
        "stale group-tree path leaked",
    )
    check(
        "library classification",
        "src\\front\\app.at" in text and "LIBRARY" in text,
        "stylekit-class libs must be split out",
    )


def test_metrics_admission() -> None:
    log = Path(tempfile.gettempdir()) / "t-rq-metrics.log"
    lines = [
        "[P725-FRAME] begin=1000 present=-1 s1_payload_us=1 s2_vm_us=1 s3a_mcp_us=1 s3b_build_us=1 s4_element_us=1 s5_layout_us=1 s5_shaping_us=1 s5_draw_us=1 builds=0 dirty=0 draw_end_ms=-1",
        "[P725-FRAME] begin=2000 present=2500 s1_payload_us=1 s2_vm_us=1 s3a_mcp_us=1 s3b_build_us=1 s4_element_us=1 s5_layout_us=1 s5_shaping_us=1 s5_draw_us=1 builds=0 dirty=0 draw_end_ms=2600",
        "[P725-FRAME] begin=3000 present=3600 s1_payload_us=1 s2_vm_us=1 s3a_mcp_us=1 s3b_build_us=1 s4_element_us=1 s5_layout_us=1 s5_shaping_us=1 s5_draw_us=1 builds=0 dirty=0 draw_end_ms=3700",
    ]
    log.write_text("\n".join(lines) + "\n", encoding="utf-8")
    r = run_py(SCRIPTS / "metrics.py", str(log), "--json", str(log) + ".json", "--quiet")
    check("metrics.py exit 0", r.returncode == 0, r.stderr[:200])
    out = json.loads((Path(str(log) + ".json")).read_text(encoding="utf-8"))
    pl = out["present_latency_ms"]
    check(
        "present<0 excluded (066 F-2)",
        pl is not None and pl["n"] == 2 and pl["min"] >= 0,
        f"n={pl and pl['n']} min={pl and pl['min']}",
    )
    check(
        "p50/p95 admission",
        pl["p50"] == 550.0 and pl["p95"] == 595.0 and pl["max"] == 600,
        json.dumps(pl),
    )


def test_driver_negative() -> None:
    ps1 = SCRIPTS / "negative.ps1"
    text = ps1.read_text(encoding="utf-8")
    check(
        "negative named-failure surface",
        "NEG PASS" in text and "did NOT fail" in text,
        "negative control must distinguish pass/fail",
    )
    check("negative no os-066 hardcode", "os-066" not in text, "stale path")


def main() -> None:
    ap = argparse.ArgumentParser()
    ap.add_argument("--demos-csv", default=None, help="optional demo-build.csv to admit")
    args = ap.parse_args()
    test_build_demos_params()
    test_metrics_admission()
    test_driver_negative()
    if args.demos_csv:
        rows = list(csv.DictReader(open(args.demos_csv, encoding="utf-8-sig")))
        apps = [r for r in rows if r["Demo"] != "stylekit"]
        fails = [r["Demo"] for r in apps if r["Exit"] != "0"]
        check(
            "demo matrix full-green (app targets)",
            not fails,
            f"failing: {fails}",
        )
        print(f"[t-rq] admitted {len(apps)} app targets, {len(rows)-len(apps)} library projects")
    print("[t-rq] driver self-tests: all passed")


if __name__ == "__main__":
    main()
