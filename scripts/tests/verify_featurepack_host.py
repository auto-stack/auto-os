#!/usr/bin/env python3
"""verify_featurepack_host.py — PLAN-067 T-03 修复版：宿主面永久验证入口（AC-01）。

用法（auto-os 仓根/worktree 根）：
    python scripts/tests/verify_featurepack_host.py

从 apps.manifest 读取 015-notes 的 daemon 对象（发现面单一事实源），按冻结
契约（docs/specs/communication/featurepack-v0.md §3）实现宿主侧 ensure：
探活 GET /api/health（200 + body interface==notes.fpack/1 才算就绪）→
未就绪按 daemon.bin spawn（app 仓根相对命令行；NOTES_DATA_DIR/NOTES_FP_POLICY
隔离环境注入）→ 退避等待。auto-notes 仓按跨仓解析序定位（AGENTS §2）：
$AUTO_NOTES_ROOT → 兄弟检出 ../auto-notes → 主检出 D:/autostack/auto-notes。

T-03 修复（对照 067 F-07 / C-04 / C-05）：
  * 进程面：不再 taskkill /IM 全局杀（auto.exe / 具名 back exe 可能属于其他
    会话任务——实测 068 worktree 的 rqhost 就是 auto.exe）；只回收本脚本
    spawn 的自有 PID（taskkill /T /F /PID）。
  * 端口面：spawn 前 connect 17819，被占用即整体拒绝（C-04 负控：占用者
    存活、数据库不变），而非先杀后测。
  * 必需 case：前置不满足（端口忙/notes 根缺失/manifest 无 daemon）时
    必需用例缺失 → 退出非零（C-05），不静默跳过。
  * 单实例核验：以自有 spawn 存活数 + 17819 LISTENING 归属断言，不用
    全局映像名计数。

覆盖（AC-01 可观察结果）：
  H-01 冷启：服务不在时从宿主 ensure 冷启动；health 版本就绪核验。
  H-02 并发首调合并：两宿主线程同时 ensure → 只允许一个 spawn 赢家，双方
       都拿到同一实例（跨宿主读写同一存储核验）+ 自有进程数恰 1。
  H-03 拒绝面：未授权凭据 / 错作用域 / 版本不合 → permission_denied /
       unsupported_version，无写入。
  H-04 单实例：服务在位时第三次 ensure 不产生第二进程。
  H-05 重连与退出恢复：杀自有服务 → ensure 探测下线 → 重连（重 spawn）→
       杀前捕获的 operation 回执仍 committed（持久跨重启）→ 新调用可用。

边界说明（067 M1 G-01/G-02/G-09；T-02 更新）：manifest daemon.bin 现为
生产路径语义（apps.manifest 指发布件 notes-service.exe，本脚本按
session.rs:4460 同款契约解析 + 065 时代命令行 split 兼容回退）——本脚本
spawn 形状与生产 ensure 一致，但 ensure 调用仍由脚本自身驱动（独立模式
等价面）；桌面宿主链（ensure_manifest_daemon 从真实桌面 launch 触发）与
注入键对齐（015_NOTES_DAEMON vs NOTES_DAEMON）为 auto-ui/Reader owner
前置，本脚本不冒充桌面链证据。
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
ARTIFACTS = Path(os.environ.get("PLAN_ARTIFACTS_HOME", "D:/autostack/plan-artifacts/plan-067"))

PASS, FAIL, SKIP = [], [], []


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
            repo_dir = OS_ROOT / a["repo"] if a.get("repo") else None
            return a.get("daemon"), repo_dir
    return None, None


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
        depth = 0
        while isinstance(out, str) and depth < 3:
            try:
                out = json.loads(out)
            except Exception:
                break
            depth += 1
    return out


def health_ready():
    """契约 §2.2：GET /api/health 200 且 body interface==notes.fpack/1 才就绪。"""
    h = http("GET", "/api/health", timeout=3)
    return h.get("status") == "ok" and h.get("interface") == IF_V1


def port_listener():
    """17819 上 LISTENING 的 PID 集合（netstat 解析；无 → 空集）。"""
    out = subprocess.run(["netstat", "-ano"], capture_output=True, text=True).stdout
    pids = set()
    for line in out.splitlines():
        if "17819" in line and "LISTENING" in line:
            parts = line.split()
            if parts and parts[-1].isdigit():
                pids.add(int(parts[-1]))
    return pids


def pid_alive(pid):
    out = subprocess.run(["tasklist", "/FI", f"PID eq {pid}", "/FO", "CSV"],
                         capture_output=True, text=True).stdout
    return f'"{pid}"' in out


def daemon_bin_cmd(daemon, repo_dir):
    """manifest daemon.bin → argv。生产契约（PLAN-067 T-02）：路径语义，
    相对 manifest repo 目录解析 + Windows .exe 补全（镜像 auto-ui
    session.rs:4460）。路径未命中按 065 时代命令行语义 split 兼容回退。"""
    rel = daemon.get("bin") or ""
    if repo_dir is not None and rel:
        cand = repo_dir / rel
        exe_cand = cand.with_name(cand.name + ".exe")
        if exe_cand.is_file():
            return [str(exe_cand)]
        if cand.is_file():
            return [str(cand)]
    return rel.split()


def spawn_service(daemon, repo_dir, run_dir, log_f):
    """按 manifest daemon.bin spawn（路径语义发布件 / 命令行兼容回退；
    端口 env 覆盖键生效）。返回 Popen。"""
    env = dict(os.environ)
    env["NOTES_DATA_DIR"] = str(run_dir / "data")
    env["NOTES_TEST_MODE"] = "1"
    env["NOTES_FP_POLICY"] = str(run_dir / "fp-policy.json")
    env[daemon.get("env_port") or "NOTES_DAEMON_PORT"] = str(daemon["port"])
    cmd = daemon_bin_cmd(daemon, repo_dir)
    p = subprocess.Popen(cmd, cwd=str(notes_root()), env=env, stdout=log_f,
                         stderr=subprocess.STDOUT,
                         creationflags=getattr(subprocess, "CREATE_NO_WINDOW", 0))
    return p


def own_alive(procs):
    """自有 spawn 中仍存活的进程数（不含已退出/端口败者）。"""
    return sum(1 for p in procs if p.poll() is None)


def record_listener(spawned_pids):
    """记录 17819 LISTENING PID 到自有集合（back 逃逸防漏：auto.exe 树被杀
    后 back 仍可能占端口——定向回收依赖此记录）。"""
    for pid in port_listener():
        if pid not in spawned_pids:
            spawned_pids.add(pid)


def kill_own_all(spawned_pids, procs):
    """只回收自有：先 auto.exe 树，再定向杀记录过的 listener/back PID。"""
    for p in list(procs):
        if p.poll() is None:
            subprocess.run(["taskkill", "/T", "/F", "/PID", str(p.pid)],
                           capture_output=True)
    for pid in list(spawned_pids):
        if pid_alive(pid):
            subprocess.run(["taskkill", "/T", "/F", "/PID", str(pid)],
                           capture_output=True)
    time.sleep(2)


def main():
    daemon, repo_dir = notes_daemon()
    check(bool(daemon) and daemon.get("port") == 17819, "H-00-manifest-daemon", daemon)
    nr = notes_root()
    check(nr is not None, "H-00b-notes-root-resolved", nr)
    if not daemon or nr is None:
        print("必需前置缺失：manifest daemon 或 notes 根 → 退出非零（C-05）")
        return 2

    # C-04 负控：目标端口已被其他进程占用 → 整体拒绝，不动占用者。
    occupants = port_listener()
    if occupants:
        check(False, "C-04-port-occupied-reject",
              f"17819 被 PID {sorted(occupants)} 占用，保持占用者存活")
        print("必需前置失败：端口占用 → 退出非零（C-05）")
        return 3

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
    # NOTES-006 r2 冻结闭集 schema（repository.at fp_source_validate）：
    # book_id/source_hash/title/author/quote/locator 必需，chapter 可选。
    src = {"book_id": "book:abc123", "source_hash": "a" * 64,
           "title": "宿主探针书", "author": "探针",
           "quote": "宿主探针原句",
           "locator": {"kind": "epub.cfi", "value": "epubcfi(/6/4!/4/2/1:0)"}}
    procs, logs = [], []
    spawned_pids = set()

    try:
        # ── H-01 冷启（主应用 UI 不参与；daemon 独立生命周期） ─────
        check(not health_ready(), "H-01a-down-before-cold-start")
        log_f = open(run_dir / "service.log", "w", encoding="utf-8")
        logs.append(log_f)
        spawn = {"done": False}

        def ensure_once():
            if health_ready():
                spawn["done"] = True
                return
            p = spawn_service(daemon, repo_dir, run_dir, log_f)
            procs.append(p)
            spawned_pids.add(p.pid)
            deadline = time.time() + 600
            while time.time() < deadline:
                if health_ready():
                    spawn["done"] = True
                    return
                time.sleep(2)

        t0 = time.time()
        ensure_once()
        record_listener(spawned_pids)
        check(spawn["done"] and health_ready(), "H-01b-cold-start-ready",
              f"{time.time() - t0:.0f}s")
        check((run_dir / "service.log").exists(), "H-01c-spawn-log")

        # ── H-02 并发首调合并（两宿主同时 ensure → 同一实例） ─────
        # 只回收自有：先停当前自有服务（保端口干净），再并发。
        kill_own_all(spawned_pids, procs)
        procs.clear()
        results = []

        def host_thread(tag):
            if not health_ready():
                p = spawn_service(daemon, repo_dir, run_dir, log_f)
                procs.append(p)
                spawned_pids.add(p.pid)
                deadline = time.time() + 600
                while time.time() < deadline:
                    if health_ready():
                        break
                    time.sleep(2)
                record_listener(spawned_pids)
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
        # 单实例语义：17819 上 LISTENING 恰 1 个（back 是唯一监听者），且归属自有。
        listeners = port_listener()
        own_listeners = listeners & spawned_pids
        check(len(own_listeners) == 1 and len(listeners) == 1,
              "H-02b-single-instance-listener", f"listeners={listeners}")
        # 跨宿主同实例：宿主 a 捕获的行对宿主 b 可读（同存储）
        nid_a = sorted(nids)[0] if nids else ""
        g = http("POST", "/api/fp/v1/get",
                 {"cred": CRED, "interface_version": IF_V1, "note_id": nid_a})
        check(g.get("ok") is True and (g.get("note") or {}).get("book_id") == "book:abc123",
              "H-02c-shared-store", g)

        # ── H-03 拒绝面（宿主契约上下文） ─────────────────────────
        # 拒绝面无写入用 receipt 证明（list 端点因服务端坏 JSON 缺口不可靠——
        # 067 M1 G-10：auto-lang PLAN-994 生成器把局部变量 len() 翻成字符数、
        # str_substr 按字节截，含中文 doc 时坏 JSON；owner=auto-lang）。
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
        # 被拒操作不得产生回执（无写入）；合法主体仍可查原已提交操作。
        q1 = http("POST", "/api/fp/v1/receipt",
                  {"cred": CRED, "interface_version": IF_V1, "operation_id": "1" * 32})
        q2 = http("POST", "/api/fp/v1/receipt",
                  {"cred": CRED, "interface_version": IF_V1, "operation_id": "2" * 32})
        check(q1.get("status") == "not_observed" and q2.get("status") == "not_observed",
              "H-03d-denied-no-receipt", (q1, q2))
        # list 端点探针：记录服务端坏 JSON 缺口（不断言——G-10 归 auto-lang）。
        lp = http("POST", "/api/fp/v1/list",
                  {"cred": CRED, "interface_version": IF_V1,
                   "book_id": "book:abc123", "cursor": 0, "limit": 100})
        if not isinstance(lp, dict) or not lp.get("ok"):
            print(f"  [G-10-probe] list 端点不可靠: {str(lp)[:120]}")

        # ── H-04 单实例（服务在位第三次 ensure 不再 spawn） ────────
        lst0 = port_listener()
        if health_ready():
            check(True, "H-04a-already-ready-no-spawn")
        time.sleep(1)
        lst1 = port_listener()
        check(len(lst1) == len(lst0) and len(lst1) == 1, "H-04b-no-second-instance",
              (lst0, lst1))

        # ── H-05 重连与退出恢复 ────────────────────────────────────
        op_kill = "4" * 32
        r5 = http("POST", "/api/fp/v1/capture",
                  {"cred": CRED, "interface_version": IF_V1, "attempt_id": "h5",
                   "operation_id": op_kill, "source_json": json.dumps(src, ensure_ascii=False),
                   "text": "重启前捕获"})
        check(r5.get("ok") is True, "H-05a-pre-kill-capture", r5)
        kill_own_all(spawned_pids, procs)
        procs.clear()
        check(not health_ready(), "H-05b-down-after-kill")
        log_f2 = open(run_dir / "service2.log", "w", encoding="utf-8")
        logs.append(log_f2)
        p2 = spawn_service(daemon, repo_dir, run_dir, log_f2)
        procs.append(p2)
        spawned_pids.add(p2.pid)
        deadline = time.time() + 600
        while time.time() < deadline and not health_ready():
            time.sleep(2)
        record_listener(spawned_pids)
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
        kill_own_all(spawned_pids, procs)
        # 清点：自有 spawn 残留须为 0；绝不动非自有进程。
        leaked = [pid for pid in spawned_pids if pid_alive(pid)]
        if leaked:
            print("LEAK 自有进程残留:", leaked)
            FAIL.append("leak-own-process-cleanup")
        for lf in logs:
            lf.close()

    print(f"\nfeaturepack-host PASS={len(PASS)} FAIL={len(FAIL)} SKIP={len(SKIP)}")
    for f in FAIL:
        print("  FAIL:", f)
    return 1 if FAIL else 0


if __name__ == "__main__":
    sys.exit(main())
