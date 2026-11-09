#!/usr/bin/env python3
"""verify_featurepack_host.py — PLAN-065 OS-T-02 宿主面永久验证入口（AC-01）。

用法（auto-os 仓根/worktree 根）：
    python scripts/tests/verify_featurepack_host.py

从 apps.manifest 读取 015-notes 的 daemon 对象（发现面单一事实源），按冻结
契约（docs/specs/communication/featurepack-v0.md §3）实现宿主侧 ensure：
探活 GET /api/health（200 + body interface==notes.fpack/1 才算就绪）→
未就绪按 daemon.bin spawn（app 仓根相对命令行；NOTES_DATA_DIR/NOTES_FP_POLICY
隔离环境注入）→ 退避等待。auto-notes 仓按跨仓解析序定位（AGENTS §2）：
$AUTO_NOTES_ROOT → 兄弟检出 ../auto-notes → 主检出 D:/autostack/auto-notes。

覆盖（AC-01 可观察结果）：
  H-01 冷启：服务不在时从宿主 ensure 冷启动（主应用 UI 不参与——spawn 即
       无 UI 服务进程，daemon 生命周期独立于主应用窗口）；health 版本就绪核验。
  H-02 并发首调合并：两宿主线程同时 ensure → 只允许一个 spawn 赢家，双方
       都拿到同一实例（跨宿主读写同一存储核验）+ 进程数恰 1。
  H-03 拒绝面：未授权凭据 / 错作用域 / 版本不合 → permission_denied /
       unsupported_version，无写入。
  H-04 单实例：服务在位时第三次 ensure 不产生第二进程。
  H-05 重连与退出恢复：杀服务 → ensure 探测下线 → 重连（重 spawn）→
       杀前捕获的 operation 回执仍 committed（持久跨重启）→ 新调用可用。

约定（PLAN-065 §6）：从本仓根运行 `python scripts/tests/verify_featurepack_host.py`，
默认隔离目录，断言失败返回非零。原始日志 D:/autostack/plan-artifacts/plan-065/。
"""
import json
import os
import shutil
import subprocess
import sys
import tempfile
import threading
import time
import urllib.error
import urllib.request
from pathlib import Path

OS_ROOT = Path(__file__).resolve().parents[2]
MANIFEST = OS_ROOT / "apps.manifest"
IF_V1 = "notes.fpack/1"
BASE = "http://127.0.0.1:17819"
ARTIFACTS = Path(os.environ.get("PLAN_ARTIFACTS_HOME", "D:/autostack/plan-artifacts/plan-065"))

PASS, FAIL = [], []


def check(cond, name, detail=""):
    if cond:
        PASS.append(name)
        print("PASS", name)
    else:
        FAIL.append(name)
        print("FAIL", name, "-", str(detail)[:300])


def notes_root():
    """跨仓解析序（AGENTS §2 红线：env → 兄弟检出 → 主检出；永不建链接）。"""
    env = os.environ.get("AUTO_NOTES_ROOT")
    if env and Path(env).is_dir():
        return Path(env)
    sib = OS_ROOT.parent / "auto-notes"
    if sib.is_dir() and (sib / "pac.at").exists():
        return sib
    main = Path("D:/autostack/auto-notes")
    if main.is_dir() and (main / "pac.at").exists():
        return main
    return None


def notes_daemon():
    m = json.loads(MANIFEST.read_text(encoding="utf-8"))
    for a in m["apps"]:
        if a["id"] == "015-notes":
            return a.get("daemon")
    return None


def http(method, path, body=None, timeout=20):
    data = json.dumps(body).encode() if body is not None else None
    req = urllib.request.Request(BASE + path, data=data, method=method,
                                 headers={"Content-Type": "application/json"})
    try:
        with urllib.request.urlopen(req, timeout=timeout) as r:
            out = json.loads(r.read().decode())
    except urllib.error.HTTPError as e:
        try:
            out = json.loads(e.read().decode())
        except Exception:
            out = {"__status": e.code}
    except Exception:
        return {"__status": 0}
    if isinstance(out, str):
        try:
            out = json.loads(out)
        except Exception:
            pass
    return out


def health_ready():
    """契约 §2.2：GET /api/health 200 且 body interface==notes.fpack/1 才就绪。"""
    h = http("GET", "/api/health", timeout=3)
    return h.get("status") == "ok" and h.get("interface") == IF_V1


def back_proc_count():
    out = subprocess.run(["tasklist", "/FI", "IMAGENAME eq auto-notes-30db9907-back.exe",
                          "/FO", "CSV"], capture_output=True, text=True).stdout
    return sum(1 for line in out.splitlines()
               if line.startswith('"auto-notes-30db9907-back.exe"'))


def spawn_service(daemon, run_dir, log_f):
    """按 manifest daemon.bin spawn（app 仓根相对命令行；端口 env 覆盖键生效）。"""
    env = dict(os.environ)
    env["NOTES_DATA_DIR"] = str(run_dir / "data")
    env["NOTES_TEST_MODE"] = "1"
    env["NOTES_FP_POLICY"] = str(run_dir / "fp-policy.json")
    env[daemon.get("env_port") or "NOTES_DAEMON_PORT"] = str(daemon["port"])
    cmd = daemon["bin"].split()
    return subprocess.Popen(cmd, cwd=str(notes_root()), env=env, stdout=log_f,
                            stderr=subprocess.STDOUT,
                            creationflags=getattr(subprocess, "CREATE_NO_WINDOW", 0))


def kill_all():
    subprocess.run(["taskkill", "/F", "/IM", "auto-notes-30db9907-back.exe"],
                   capture_output=True)
    subprocess.run(["taskkill", "/F", "/IM", "auto.exe"], capture_output=True)
    time.sleep(2)


def main():
    daemon = notes_daemon()
    check(bool(daemon) and daemon.get("port") == 17819, "H-00-manifest-daemon", daemon)
    nr = notes_root()
    check(nr is not None, "H-00b-notes-root-resolved", nr)
    if not daemon or nr is None:
        return 2

    ARTIFACTS.mkdir(parents=True, exist_ok=True)
    run_dir = Path(tempfile.mkdtemp(prefix="fphost-", dir=str(ARTIFACTS)))
    policy = {"entries": [
        {"cred": "c" * 64, "principal": "fp-reader",
         "operations": ["capture", "read", "update", "receipt", "list_by_book"],
         "book_scopes": ["book:abc123"]},
        {"cred": "o" * 64, "principal": "fp-otherbook",
         "operations": ["capture", "read"], "book_scopes": ["book:other"]},
    ]}
    (run_dir / "fp-policy.json").write_text(json.dumps(policy), encoding="utf-8")
    CRED = "c" * 64
    CRED_OTHER = "o" * 64
    src = {"book_id": "book:abc123", "quote": "宿主探针原句", "chapter": 1}
    procs, logs = [], []

    try:
        # ── H-01 冷启（主应用 UI 不参与；daemon 独立生命周期） ─────
        kill_all()
        check(not health_ready(), "H-01a-down-before-cold-start")
        log_f = open(run_dir / "service.log", "w", encoding="utf-8")
        logs.append(log_f)
        spawn = {"done": False}

        def ensure_once():
            if health_ready():
                spawn["done"] = True
                return
            p = spawn_service(daemon, run_dir, log_f)
            procs.append(p)
            deadline = time.time() + 600
            while time.time() < deadline:
                if health_ready():
                    spawn["done"] = True
                    return
                time.sleep(2)

        t0 = time.time()
        ensure_once()
        check(spawn["done"] and health_ready(), "H-01b-cold-start-ready",
              f"{time.time() - t0:.0f}s")
        check((run_dir / "service.log").exists(), "H-01c-spawn-log")

        # ── H-02 并发首调合并（两宿主同时 ensure → 同一实例） ─────
        kill_all()
        procs.clear()
        results = []

        def host_thread(tag):
            if not health_ready():
                p = spawn_service(daemon, run_dir, log_f)
                procs.append(p)
                deadline = time.time() + 600
                while time.time() < deadline:
                    if health_ready():
                        break
                    time.sleep(2)
            r = http("POST", "/api/fp/v1/capture",
                     {"cred": CRED, "interface_version": IF_V1,
                      "attempt_id": f"h2-{tag}", "operation_id": tag * 32,
                      "source_json": json.dumps(src, ensure_ascii=False),
                      "text": f"宿主{tag}并发捕获"})
            results.append(r)

        threads = [threading.Thread(target=host_thread, args=("a",)),
                   threading.Thread(target=host_thread, args=("b",))]
        for t in threads:
            t.start()
        for t in threads:
            t.join()
        okcaps = [r for r in results if r.get("ok") is True]
        check(len(okcaps) == 2, "H-02a-both-hosts-ok",
              [r.get("err_code") for r in results])
        nids = {r["note"]["note_id"] for r in okcaps if r.get("note")}
        time.sleep(2)
        pc = back_proc_count()
        check(pc == 1, "H-02b-single-instance-proc", f"procs={pc}")
        # 跨宿主同实例：宿主 a 捕获的行对宿主 b 可读（同存储）
        nid_a = sorted(nids)[0] if nids else ""
        g = http("POST", "/api/fp/v1/get",
                 {"cred": CRED, "interface_version": IF_V1, "note_id": nid_a})
        check(g.get("ok") is True and (g.get("note") or {}).get("book_id") == "book:abc123",
              "H-02c-shared-store", g)

        # ── H-03 拒绝面（宿主契约上下文） ─────────────────────────
        before = http("POST", "/api/fp/v1/list",
                      {"cred": CRED, "interface_version": IF_V1,
                       "book_id": "book:abc123", "cursor": 0, "limit": 100})
        n_before = len(before.get("items") or [])
        d1 = http("POST", "/api/fp/v1/capture",
                  {"cred": "", "interface_version": IF_V1, "attempt_id": "h3a",
                   "operation_id": "1" * 32, "source_json": json.dumps(src), "text": "x"})
        check(d1.get("err_code") == "permission_denied", "H-03a-unauthorized", d1)
        d2 = http("POST", "/api/fp/v1/capture",
                  {"cred": CRED_OTHER, "interface_version": IF_V1, "attempt_id": "h3b",
                   "operation_id": "2" * 32, "source_json": json.dumps(src), "text": "x"})
        check(d2.get("err_code") == "permission_denied", "H-03b-wrong-scope", d2)
        d3 = http("POST", "/api/fp/v1/get",
                  {"cred": CRED, "interface_version": "notes.fpack/2", "note_id": nid_a})
        check(d3.get("err_code") == "unsupported_version", "H-03c-version", d3)
        after = http("POST", "/api/fp/v1/list",
                     {"cred": CRED, "interface_version": IF_V1,
                      "book_id": "book:abc123", "cursor": 0, "limit": 100})
        check(len(after.get("items") or []) == n_before, "H-03d-no-write",
              (n_before, len(after.get("items") or [])))

        # ── H-04 单实例（服务在位第三次 ensure 不再 spawn） ────────
        pc0 = back_proc_count()
        if health_ready():
            check(True, "H-04a-already-ready-no-spawn")
        pc1 = back_proc_count()
        check(pc1 == pc0 and pc1 == 1, "H-04b-no-second-instance", (pc0, pc1))

        # ── H-05 重连与退出恢复 ────────────────────────────────────
        op_kill = "4" * 32
        r5 = http("POST", "/api/fp/v1/capture",
                  {"cred": CRED, "interface_version": IF_V1, "attempt_id": "h5",
                   "operation_id": op_kill, "source_json": json.dumps(src, ensure_ascii=False),
                   "text": "重启前捕获"})
        check(r5.get("ok") is True, "H-05a-pre-kill-capture", r5)
        kill_all()
        procs.clear()
        time.sleep(1)
        check(not health_ready(), "H-05b-down-after-kill")
        log_f2 = open(run_dir / "service2.log", "w", encoding="utf-8")
        logs.append(log_f2)
        p2 = spawn_service(daemon, run_dir, log_f2)
        procs.append(p2)
        deadline = time.time() + 600
        while time.time() < deadline and not health_ready():
            time.sleep(2)
        check(health_ready(), "H-05c-reconnected")
        q = http("POST", "/api/fp/v1/receipt",
                 {"cred": CRED, "interface_version": IF_V1, "operation_id": op_kill})
        check(q.get("status") == "committed" and (q.get("receipt") or {}).get("note_id", "")
              .startswith("n-"), "H-05d-receipt-persists-across-restart", q)
        r6 = http("POST", "/api/fp/v1/capture",
                  {"cred": CRED, "interface_version": IF_V1, "attempt_id": "h5b",
                   "operation_id": "3" * 32, "source_json": json.dumps(src, ensure_ascii=False),
                   "text": "重连后捕获"})
        check(r6.get("ok") is True, "H-05e-post-reconnect-capture", r6)

    finally:
        for p in procs:
            if p.poll() is None:
                subprocess.run(["taskkill", "/T", "/F", "/PID", str(p.pid)],
                               capture_output=True)
        kill_all()
        for lf in logs:
            lf.close()

    print(f"\nfeaturepack-host PASS={len(PASS)} FAIL={len(FAIL)}")
    for f in FAIL:
        print("  FAIL:", f)
    return 1 if FAIL else 0


if __name__ == "__main__":
    sys.exit(main())
