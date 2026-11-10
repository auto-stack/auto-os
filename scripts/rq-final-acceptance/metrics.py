#!/usr/bin/env python3
"""OS066 metrics: parse rq-final-acceptance desktop log into frame metrics.

Input channels:
  [P725-FRAME] begin=.. present=.. s1_payload_us=.. .. s5_draw_us=.. builds=.. dirty=.. draw_end_ms=..
  [plan060] frame compose wid=.. frame_id=.. slot=..
Output: JSON summary (p50/p95/max inter-frame gaps, present latency,
wid distribution, frame counts) + human-readable table.
"""
import argparse
import json
import math
import re
import statistics
from pathlib import Path

P725 = re.compile(
    r"\[P725-FRAME\] begin=(?P<begin>-?\d+) present=(?P<present>-?\d+)"
    r" s1_payload_us=(?P<s1>\d+) s2_vm_us=(?P<s2>\d+) s3a_mcp_us=(?P<s3a>\d+)"
    r" s3b_build_us=(?P<s3b>\d+) s4_element_us=(?P<s4>\d+) s5_layout_us=(?P<s5l>\d+)"
    r" s5_shaping_us=(?P<s5s>\d+) s5_draw_us=(?P<s5d>\d+) builds=(?P<builds>\d+)"
    r" dirty=(?P<dirty>-?\d+) draw_end_ms=(?P<draw_end>-?\d+)"
)
COMPOSE = re.compile(r"\[plan060\] frame compose wid=(?P<wid>\d+) frame_id=(?P<fid>\d+) slot=(?P<slot>\d+)")


def pct(values, p):
    if not values:
        return None
    s = sorted(values)
    k = (len(s) - 1) * p
    f = math.floor(k)
    c = math.ceil(k)
    if f == c:
        return s[int(k)]
    return s[f] * (c - k) + s[c] * (k - f)


def summary(name, values):
    if not values:
        return None
    return {
        "n": len(values),
        "p50": round(pct(values, 0.50), 3),
        "p95": round(pct(values, 0.95), 3),
        "p99": round(pct(values, 0.99), 3),
        "max": round(max(values), 3),
        "min": round(min(values), 3),
        "mean": round(statistics.mean(values), 3),
    }


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("log")
    ap.add_argument("--json", default=None, help="output summary json path")
    ap.add_argument("--quiet", action="store_true")
    args = ap.parse_args()

    frames = []
    composes = []
    wid_fids = {}
    for line in Path(args.log).read_text(encoding="utf-8", errors="replace").splitlines():
        m = P725.search(line)
        if m:
            d = m.groupdict()
            frames.append(
                {
                    "begin": int(d["begin"]),
                    "present": int(d["present"]),
                    "draw_end": int(d["draw_end"]),
                    "s1": int(d["s1"]), "s2": int(d["s2"]), "s3a": int(d["s3a"]),
                    "s3b": int(d["s3b"]), "s4": int(d["s4"]),
                    "s5l": int(d["s5l"]), "s5s": int(d["s5s"]), "s5d": int(d["s5d"]),
                    "builds": int(d["builds"]), "dirty": int(d["dirty"]),
                }
            )
            continue
        m = COMPOSE.search(line)
        if m:
            wid = int(m.group("wid"))
            fid = int(m.group("fid"))
            composes.append((wid, fid))
            wid_fids.setdefault(wid, []).append(fid)

    result = {
        "frames": len(frames),
        "composes": len(composes),
        "frame_gap_ms": summary("frame_gap", [b["begin"] - a["begin"] for a, b in zip(frames, frames[1:])])
        if len(frames) > 1 else None,
        "present_latency_ms": summary("present_latency", [f["present"] - f["begin"] for f in frames if f["present"] > 0])
        if frames else None,
        "draw_end_latency_ms": summary("draw_end_latency", [f["draw_end"] - f["begin"] for f in frames if f["draw_end"] > 0])
        if frames else None,
        "build_ms": summary("build", [f["builds"] for f in frames]) if frames else None,
        "wid": {
            str(w): {
                "composes": len(fids),
                "gap_ms": summary("wid_gap", [b - a for a, b in zip(fids, fids[1:])])
                if len(fids) > 1 else None,
            }
            for w, fids in wid_fids.items()
        },
    }

    out = args.json or (Path(args.log).with_suffix(".summary.json"))
    Path(out).write_text(json.dumps(result, ensure_ascii=False, indent=2), encoding="utf-8")
    if not args.quiet:
        print(json.dumps(result, ensure_ascii=False, indent=2))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
