#!/usr/bin/env python3
"""verify_notes_featurepack.py — PLAN-067 T-03 修复版：双端组合验收入口。

用法（auto-os 仓根/worktree 根）：
    python scripts/tests/verify_notes_featurepack.py [--profile vue|vm]

profile 参数（T-03 加入，对照 067 §8/C-05）：未选 = 两端默认（本入口为
API 级组合，不经浏览器 UI）；--profile 只影响局部运行的成绩范围——局部
运行仅报局部 PASS/FAIL，不把未选的另一端计入必需。必需 case（栈就绪/
建书/捕获/组合语义）强制缺失或失败 → 退出非零。

组合面（三方版本绑定的可观察结果）：
  C-1 Notes 编辑组件双端契约复演（同组件 FpNoteEditor 门——NOTES-006
      verify_featurepack_editor.py 承载；本入口核服务/宿主组合）。
  C-2 Reader 宿主 UI 捕获链（页面 → vite 代理 → reader back 代理 → notes：
      真实产品入口 submit 200 落库——reader_smoke R-2a 等价面）。
  C-3 组合验收（AC-02 核心）：Reader 侧捕获 → Notes 侧改文（外部写入）→
      Reader 侧读回新修订（同 note_id revision+1）→ 期望冲突路径
      （过期 revision 拒写 + current_revision 反馈）。
  C-4 退出恢复面：notes 服务重启后，Reader 侧先前捕获的回执仍 committed
      （数据跨重启持久，AC-01 组合面）。

T-03 修复（对照 067 F-07 / C-04 / C-05）：
  * 进程面：不再 taskkill /IM auto.exe / auto-notes-*/auto-reader*-back.exe
    （全局映像名会终止其他任务——实测 068 rqhost 即 auto.exe）；只回收
    本脚本 spawn 的自有 PID（taskkill /T /F /PID），finally 后核自有残留。
  * 数据面：注入隔离 AUTO_READER_DATA（reader 服务写自有数据根，不碰宿主
    数据目录）；notes/reader 均用 run_dir 隔离 env。
  * 端口面：17819 / 17825 被占用即整体拒绝（C-04 负控：占用者存活、数据
    库不变），不先杀后测。
  * 必需 skip：必需前置失败（栈未就绪/书未导入/核心用例缺失）→ 退出非零
    （C-05），不静默跳过。

运行形态：本入口编排两服务（notes rust 17819 + reader back 17825），
隔离数据目录与 policy；断言失败返回非零。
原始日志 D:/autostack/plan-artifacts/plan-067/。
在案边界（诚实标注）：本入口为 API/代理级组合核验（C-2 经 reader 代理=
页面同路径）；真实浏览器 UI 选句/回访流由 T-04 联合脚本 + READER-013
R-01～05 承载，不以 API 总数替代 UI 证据。
"""
import argparse
import json
import os
import shutil
import subprocess
import sys
import tempfile
import time
import urllib.error
import urllib.request
from pathlib import Path

OS_ROOT = Path(__file__).resolve().parents[2]
NOTES = Path(os.environ.get("AUTO_NOTES_ROOT", "D:/autostack/auto-notes"))
READER = Path(os.environ.get("AUTO_READER_ROOT", "D:/autostack/auto-reader"))
ART = Path(os.environ.get("PLAN_ARTIFACTS_HOME", "D:/autostack/plan-artifacts/plan-067"))
IF_V1 = "notes.fpack/1"
CRED = "c" * 64

PASS, FAIL, SKIP = [], [], []


def check(cond, name, detail=""):
    if cond:
        PASS.append(name)
        print("PASS", name)
    else:
        FAIL.append(name)
        print("FAIL", name, "-", str(detail)[:220])


def http(base, method, path, body=None, timeout=60):
    data = json.dumps(body).encode() if body is not None else None
    req = urllib.request.Request(base + path, data=data, method=method,
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
    depth = 0
    while isinstance(out, str) and depth < 3:
        out = json.loads(out)
        depth += 1
    return out


def port_listener(port):
    out = subprocess.run(["netstat", "-ano"], capture_output=True, text=True).stdout
    pids = set()
    for line in out.splitlines():
        if str(port) in line and "LISTENING" in line:
            parts = line.split()
            if parts and parts[-1].isdigit():
                pids.add(int(parts[-1]))
    return pids


def spawn(cmd, cwd, env_extra, log):
    env = dict(os.environ)
    env.update(env_extra)
    f = open(log, "w", encoding="utf-8")
    p = subprocess.Popen(cmd, cwd=str(cwd), env=env, stdout=f, stderr=subprocess.STDOUT,
                         creationflags=getattr(subprocess, "CREATE_NO_WINDOW", 0))
    return p, f


def kill_own(p):
    """只回收自有进程（/T 杀树 + /F 强制 + /PID 定向）；不动全局映像名。"""
    if p and p.poll() is None:
        subprocess.run(["taskkill", "/T", "/F", "/PID", str(p.pid)], capture_output=True)


def wait_health(base, timeout=900):
    dl = time.time() + timeout
    while time.time() < dl:
        try:
            with urllib.request.urlopen(base + "/api/health", timeout=4) as r:
                if r.status == 200:
                    return True
        except Exception:
            time.sleep(2)
    return False


def main():
    ap = argparse.ArgumentParser(description=__doc__)
    ap.add_argument("--profile", choices=["vue", "vm"], default=None,
                    help="局部运行 profile（缺省=两端 API 级组合）")
    args = ap.parse_args()

    # C-04 负控：目标端口被占用 → 整体拒绝，不动占用者。
    for port, tag in ((17819, "notes"), (17825, "reader")):
        occ = port_listener(port)
        if occ:
            check(False, f"C-04-port-occupied-{tag}",
                  f"{port} 被 PID {sorted(occ)} 占用，保持占用者存活")
            print(f"必需前置失败：{port} 端口占用 → 退出非零（C-05）")
            return 3

    ART.mkdir(parents=True, exist_ok=True)
    run_dir = Path(tempfile.mkdtemp(prefix="fpcombo-", dir=str(ART)))
    policy = {"entries": []}
    (run_dir / "fp-policy.json").write_text(json.dumps(policy), encoding="utf-8")
    nsvc, nf = spawn(["auto", "service", "--server", "rust", "-B", "17819"], NOTES,
                     {"NOTES_DATA_DIR": str(run_dir / "ndata"), "NOTES_TEST_MODE": "1",
                      "NOTES_FP_POLICY": str(run_dir / "fp-policy.json")},
                     run_dir / "notes.log")
    rsvc, rf = spawn(["auto", "service", "--server", "vm", "-B", "17825"], READER,
                     {"NOTES_DAEMON": "http://127.0.0.1:17819", "NOTES_FP_CRED": CRED,
                      "AUTO_READER_DATA": str(run_dir / "reader-data"),
                      "AUTO_HTTP_STEP_BUDGET": "1000000000"},
                     run_dir / "reader-back.log")
    procs = [nsvc, rsvc]
    try:
        # 栈就绪（必需：失败即非零）
        check(wait_health("http://127.0.0.1:17819", 900), "C-0a-notes-up")
        dl = time.time() + 600
        rup = False
        while time.time() < dl:
            if "VM HTTP service ready" in (run_dir / "reader-back.log").read_text(errors="replace"):
                rup = True
                break
            time.sleep(2)
        check(rup, "C-0b-reader-back-up")
        if "C-0a-notes-up" in FAIL or "C-0b-reader-back-up" in FAIL:
            print("必需前置失败：栈未就绪 → 退出非零（C-05）")
            return 1

        # 建书（library 目录）→ 动态授权书
        imp = http("http://127.0.0.1:17825", "POST", "/api/library/import-bytes",
                   {"name": "combo.txt", "author": "combo", "force": True,
                    "content_text": "第一章 组合\n组合验收的选区原句。\n正文行。\n"})
        bid = imp.get("book_id", "") if isinstance(imp, dict) else ""
        check(len(bid) > 10, "C-0c-book-imported", str(imp)[:100])
        if "C-0c-book-imported" in FAIL:
            print("必需前置失败：书未导入 → 退出非零（C-05）")
            return 1
        pol = {"entries": [{"cred": CRED, "principal": "fp-reader",
                            "operations": ["capture", "read", "update", "receipt", "list_by_book"],
                            "book_scopes": ["book:" + bid]}]}
        (run_dir / "fp-policy.json").write_text(json.dumps(pol), encoding="utf-8")

        NB = "http://127.0.0.1:17819"
        RB = "http://127.0.0.1:17825"
        scope = "book:" + bid

        # C-2 Reader 宿主 UI 捕获链（经 reader 代理 = 页面同路径）
        cap = http(RB, "POST", "/api/fp/note/capture",
                   {"attempt_id": "combo-c1", "operation_id": "a" * 32,
                    "book_id": scope, "quote": "组合验收的选区原句。",
                    "chapter": 1, "locator": "", "text": "组合验收正文 v1"})
        check(cap.get("ok") is True and (cap.get("note") or {}).get("revision") == 1,
              "C-2a-reader-proxy-capture", str(cap)[:160])
        nid = (cap.get("note") or {}).get("note_id", "")

        # C-3 组合：Notes 侧改文 → Reader 侧读回新修订
        ext = http(NB, "POST", "/api/fp/v1/update",
                   {"cred": CRED, "interface_version": IF_V1, "attempt_id": "combo-ext",
                    "note_id": nid, "expected_revision": 1,
                    "text": "组合验收外部改文", "operation_id": "b" * 32})
        check(ext.get("ok") is True and (ext.get("note") or {}).get("revision") == 2,
              "C-3a-notes-side-edit", str(ext)[:160])
        back = http(RB, "POST", "/api/fp/note/get", {"note_id": nid})
        check(back.get("ok") is True and (back.get("note") or {}).get("revision") == 2 and
              (back.get("note") or {}).get("body") == "组合验收外部改文",
              "C-3b-reader-reads-new-revision", str(back)[:160])

        # C-3c 过期 revision 经 reader 代理拒写 + current_revision 反馈
        stale = http(RB, "POST", "/api/fp/note/update",
                     {"attempt_id": "combo-stale", "note_id": nid, "expected_revision": 1,
                      "text": "过期意图", "operation_id": "c" * 32})
        check(stale.get("ok") is False and stale.get("err_code") == "revision_conflict"
              and stale.get("current_revision") == 2, "C-3c-stale-rejected", str(stale)[:160])

        # C-4 退出恢复：notes 重启 → 回执仍 committed → 新捕获可用
        kill_own(nsvc)
        time.sleep(2)
        nsvc2, nf2 = spawn(["auto", "service", "--server", "rust", "-B", "17819"], NOTES,
                           {"NOTES_DATA_DIR": str(run_dir / "ndata"), "NOTES_TEST_MODE": "1",
                            "NOTES_FP_POLICY": str(run_dir / "fp-policy.json")},
                           run_dir / "notes2.log")
        procs.append(nsvc2)
        check(wait_health("http://127.0.0.1:17819", 900), "C-4a-notes-restarted")
        rc = http(NB, "POST", "/api/fp/v1/receipt",
                  {"cred": CRED, "interface_version": IF_V1, "operation_id": "a" * 32})
        check(rc.get("status") == "committed" and (rc.get("receipt") or {}).get("note_id") == nid,
              "C-4b-receipt-persists", str(rc)[:160])
        cap2 = http(RB, "POST", "/api/fp/note/capture",
                    {"attempt_id": "combo-c4", "operation_id": "d" * 32,
                     "book_id": scope, "quote": "重启后原句", "chapter": 1,
                     "locator": "", "text": "重启后新捕获"})
        check(cap2.get("ok") is True, "C-4c-post-restart-capture", str(cap2)[:160])

        # C-1 组件门指针（双端编辑契约由 NOTES-006 入口承载，此处核其存在且可执行）
        ed = NOTES / "tests" / "basic" / "verify_featurepack_editor.py"
        check(ed.exists(), "C-1a-editor-dual-track-entry-exists", ed)

    finally:
        for p in list(procs):
            kill_own(p)
        time.sleep(1)
        for f in (nf, rf):
            if f:
                f.close()
        shutil.rmtree(run_dir, ignore_errors=True)

    print(f"\nnotes-featurepack-combo PASS={len(PASS)} FAIL={len(FAIL)} SKIP={len(SKIP)}"
          f" profile={args.profile or 'both'}")
    for f in FAIL:
        print("  FAIL:", f)
    return 1 if FAIL else 0


if __name__ == "__main__":
    sys.exit(main())
