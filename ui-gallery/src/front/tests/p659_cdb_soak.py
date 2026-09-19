"""P659 F1 崩溃族 cdb 定向 soak 编排（T-00/T-01 主器械）。

形态：cdb 从进程起点托管画廊 VM 实例（零 attach 竞态），sxe 事件命令在
first-chance 捕获四死法（0xC00000FD 栈溢出 / 0xC0000409 fastfail /
0xC0000005 AV / 0xC0000374 堆损坏），现场（寄存器/故障线程深栈/全线程栈/
模块表）经 -logo 落档；soak 驱动复用 P642 画像（2 轮全遍历+逐条截图+
029 双开+空闲段）。

判读流程（T-00 固化）：
  1. cdb 日志含 P659_CRASH_CAUGHT → 死法=异常码映射（FD=栈溢出/409=fastfail/
     005=AV/374=堆损坏），故障帧=kv 首帧模块!符号+源码行；
  2. cdb 退出但无 CAUGHT → 正常退出或非四死法，查原生退出码；
  3. 审计日志差分：死亡窗口新增 code=101 行=Rust panic（非 F1 型）；
     零新增=绕过 panic 钩子（吻合栈溢出/fastfail）。
三方交叉定罪后停轮（--all 不停则继续记账）。

用法：
  python p659_cdb_soak.py [--rounds 4] [--port-base 23900] [--all]
"""
import argparse
import json
import os
import re
import socket
import subprocess
import sys
import time
import urllib.request
from pathlib import Path

HERE = Path(__file__).resolve().parent
GROUP = HERE.parents[4]                      # .wt/lang-659
AUTO_OS_WT = HERE.parents[3]                 # .wt/lang-659/auto-os
GALLERY_DIR = HERE.parents[2]                # .../ui-gallery
AUTO_LANG_MAIN = Path("D:/autostack/auto-lang")
AUTO_LANG_WT = GROUP / "auto-lang"
AUTO_BIN_WT = AUTO_LANG_WT / "target" / "debug" / "auto.exe"
CDB = Path(os.environ.get(
    "CDB_PATH",
    r"C:/Program Files (x86)/Windows Kits/10/Debuggers/x64/cdb.exe"))
AUDIT_LOG = Path(os.environ["LOCALAPPDATA"]) / "auto-desktop" / "exit-audit.log"
LOG_DIR = HERE / "cdb_logs"

CATCH_CHAIN = (
    ".echo ===P659_CRASH_CAUGHT===; .lastevent; .exr -1; r; kv 80; "
    "~#k 250; ~*k 40; lm; .echo ===P659_CAPTURE_DONE===; q")
# PLAN-659 T-02：Rust panic（unwind，非 SEH——sxe 链不可达）的断点臂：
# panic_fmt/bounds_check 处打全栈后 `g` 续跑（不杀进程，tokio 捕获语义
# 保持），腐坏索引族（virt_memory.rs:409/:519 审计在案）由此拿现场。
PANIC_BP_CHAIN = (
    ".echo ===P659_RUST_PANIC===; r; kv 60; ~#k 60; g")
PANIC_BPS = [
    "auto!core::panicking::panic_fmt",
    "auto!core::panicking::panic_bounds_check",
]
DEATH_CODES = ["0xC00000FD", "0xC0000409", "0xC0000005", "0xC0000374"]
DEATH_NAMES = {
    "c00000fd": "stack_overflow",
    "c0000409": "fastfail",
    "c0000005": "access_violation",
    "c0000374": "heap_corruption",
}

# AutoUiMcpClient 导入路径：组内 auto-lang 优先，回退主检出
for _cand in (AUTO_LANG_WT, AUTO_LANG_MAIN):
    _scripts = _cand / ".agents" / "skills" / "autoui-verifier" / "scripts"
    if (_scripts / "test_vm_mcp.py").exists():
        sys.path.insert(0, str(_scripts))
        break
from test_vm_mcp import AutoUiMcpClient  # noqa: E402


def pick_free_port(base: int) -> int:
    with socket.socket(socket.AF_INET, socket.SOCK_STREAM) as s:
        s.bind(("127.0.0.1", 0))
        return s.getsockname()[1]


def sidebar_ids(snap: str):
    ids = {}
    for line in snap.splitlines():
        m = re.search(r"(aura_\d+|vnode_\d+)", line)
        label = re.search(r"\b(\d{3}-[a-z][a-z0-9-]*)\b", line)
        if m and label and label.group(1) not in ids:
            ids[label.group(1)] = m.group(0)
    return ids


def audit_len() -> int:
    try:
        with open(AUDIT_LOG, "rb") as f:
            return sum(1 for _ in f)
    except FileNotFoundError:
        return 0


def audit_new_lines(before: int) -> list:
    try:
        with open(AUDIT_LOG, "r", encoding="utf-8", errors="replace") as f:
            lines = f.readlines()
    except FileNotFoundError:
        return []
    return [ln.rstrip() for ln in lines[before:] if ln.strip()]


def launch_cdb(port: int, log_file: Path):
    LOG_DIR.mkdir(exist_ok=True)
    # -c2 = second-chance 命令（fastfail 0xC0000409 按设计只以 second-chance
    # 送达，first-chance 链对它不触发）；-hd = 禁用调试堆（调试器创建的
    # 进程默认启用 debug heap，实测画廊生成期即死——详见计划 §5）。
    init = "; ".join(
        f'sxe -c "{CATCH_CHAIN}" -c2 "{CATCH_CHAIN}" {code}'
        for code in DEATH_CODES)
    for sym in PANIC_BPS:
        # bu = 延迟断点：初始断点期 exe 符号未载，bp 会拿 UMPDC.dll
        # 解析失败（实测）；bu 在模块载入时解析。
        init += f'; bu {sym} "{PANIC_BP_CHAIN}"'
    init += "; g"
    env = dict(
        os.environ,
        AUTOUI_MCP_PORT=str(port),
        AUTO_GALLERY_APPS=str(AUTO_LANG_WT / "examples" / "ui"),
    )
    return subprocess.Popen(
        [str(CDB), "-G", "-hd", "-lines", "-logo", str(log_file), "-c", init,
         str(AUTO_BIN_WT), "run", "-r", "vm"],
        cwd=str(GALLERY_DIR), env=env,
        stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL,
        stdin=subprocess.DEVNULL,
        creationflags=subprocess.CREATE_NO_WINDOW)


def wait_mcp_ready(port: int, cdb_proc, timeout_s: int = 300):
    c = AutoUiMcpClient(port)
    t0 = time.time()
    while time.time() - t0 < timeout_s:
        if cdb_proc.poll() is not None:
            return None, "cdb_exited_during_boot"
        try:
            if "045-style-import" in c.snapshot():
                return c, None
        except Exception:
            pass
        time.sleep(2.0)
    return None, "mcp_ready_timeout"


def soak_drive(client, rounds: int = 2):
    """P642 画像：N 轮全遍历+逐条截图+029 双开+空闲 20s。"""
    snap = client.snapshot()
    ids = sidebar_ids(snap)
    order = sorted(ids)
    for rnd in range(1, rounds + 1):
        for label in order:
            client.press(ids[label])
            time.sleep(0.6)
            client.screenshot(f"soak_r{rnd}_{label.replace('-', '_')}",
                              baseline=False)
        if "029-photo-gallery" in ids:
            client.press(ids["029-photo-gallery"]); time.sleep(1.0)
            nxt = ids.get("008-pricing-table", ids[order[0]])
            client.press(nxt); time.sleep(0.5)
            client.press(ids["029-photo-gallery"]); time.sleep(1.0)
    time.sleep(20)
    return len(client.snapshot())


def parse_crash(log_file: Path):
    """从 cdb 日志提取死法+故障帧。返回 None（未捕获）或 dict。

    标记只认独立输出行（.echo 产物）；初始断点处的 sxe 命令回显行含同
    文本，不作为捕获证据。异常码/故障帧仅解析标记之后的内容。
    """
    try:
        text = log_file.read_text(encoding="utf-8", errors="replace")
    except FileNotFoundError:
        return None
    m = re.search(r"^===P659_CRASH_CAUGHT===\s*$", text, re.M)
    rust_panics = len(re.findall(r"^===P659_RUST_PANIC===", text, re.M))
    if not m and rust_panics == 0:
        return None
    if not m:
        # 仅 Rust panic 断点命中（进程未死，tokio 捕获语义）——报腐坏族
        # 计数 + 首个 panic 帧样本。
        after = text.split("===P659_RUST_PANIC===", 1)[1]
        frames = re.findall(r"^\S+!(\S+\+0x[0-9a-f]+)", after, re.M)
        return {
            "death": "rust_panic_captured",
            "code": None,
            "last_event": f"{rust_panics} rust panic(s)",
            "fault_frames": frames[:12],
            "rust_panics": rust_panics,
            "log": str(log_file),
        }
    after = text[m.end():]
    code = None
    for cm in re.finditer(r"code (c0000[0-9a-f]+)", after):
        code = cm.group(1).lower()
    frames = re.findall(
        r"^\S+\s+\S+\s+\S+\s+\S+\s+:\s+.*?:\s+(\S+!\S+)", after, re.M)
    event_frames = re.findall(r"^\S+!(\S+\+0x[0-9a-f]+)", after, re.M)
    lastevent = ""
    lm = re.search(r"Last event: (.*)", after)
    if lm:
        lastevent = lm.group(1).strip()
    return {
        "death": DEATH_NAMES.get(code, f"unknown_{code}"),
        "code": code,
        "last_event": lastevent,
        "fault_frames": frames[:12] or event_frames[:12],
        "log": str(log_file),
    }


def kill_tree(pid: int):
    subprocess.run(["taskkill", "/F", "/T", "/PID", str(pid)],
                   capture_output=True)
    # 兜底清扫：cdb 树杀偶发漏掉调试体（taskkill 与调试器分离竞态，实测
    # round-05 泄漏僵尸——半活实例窗口尺寸零、占端口）。按可执行路径
    # 匹配本 worktree 二进制逐个补杀。
    try:
        out = subprocess.run(
            ["powershell", "-NoProfile", "-Command",
             "Get-CimInstance Win32_Process -Filter \"Name='auto.exe'\" | "
             "Where-Object { $_.ExecutablePath -match 'lang-659' } | "
             "Select-Object -ExpandProperty ProcessId"],
            capture_output=True, text=True, timeout=20).stdout.split()
        for stray in out:
            subprocess.run(["taskkill", "/F", "/PID", stray],
                           capture_output=True)
    except Exception:
        pass


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--rounds", type=int, default=4)
    ap.add_argument("--all", action="store_true",
                    help="捕获到崩溃后继续跑完剩余轮次")
    args = ap.parse_args()

    if not CDB.exists():
        print(f"[!] cdb not found: {CDB}")
        return 2
    if not AUTO_BIN_WT.exists():
        print(f"[!] auto.exe not found: {AUTO_BIN_WT}")
        return 2
    LOG_DIR.mkdir(exist_ok=True)
    print(f"[*] gallery={GALLERY_DIR}")
    print(f"[*] auto bin={AUTO_BIN_WT}")
    print(f"[*] cdb={CDB}")
    print(f"[*] audit log={AUDIT_LOG}")

    results = []
    for rnd in range(1, args.rounds + 1):
        port = pick_free_port(0)
        log_file = LOG_DIR / f"round_{rnd:02d}.log"
        audit_before = audit_len()
        print(f"\n=== round {rnd}/{args.rounds} port={port} "
              f"log={log_file.name} audit_lines={audit_before} ===")
        t0 = time.time()
        proc = launch_cdb(port, log_file)
        client, err = wait_mcp_ready(port, proc)
        if client is None:
            alive = proc.poll() is None
            print(f"[!] boot failed: {err} (cdb alive={alive})")
            crash = parse_crash(log_file)
            if alive:
                kill_tree(proc.pid)
            results.append({
                "round": rnd, "outcome": f"boot_fail:{err}",
                "crash": crash,
                "audit_new": audit_new_lines(audit_before),
                "minutes": round((time.time() - t0) / 60, 1),
            })
            if crash:
                print(f"[!!!] CRASH DURING BOOT: {crash['death']} "
                      f"{crash['fault_frames'][:3]}")
                if not args.all:
                    break
            continue

        print(f"[*] mcp ready in {time.time()-t0:.0f}s; driving soak ...")
        outcome = "survived"
        err_detail = ""
        try:
            final_len = soak_drive(client)
            print(f"[*] soak done; final snapshot {final_len} chars")
        except Exception as e:
            err_detail = str(e)[:200]
            if proc.poll() is None:
                outcome = "driver_error"
            else:
                outcome = "crash_captured"
        if outcome == "survived":
            kill_tree(proc.pid)
            proc.wait(timeout=30)
        elif outcome == "driver_error":
            kill_tree(proc.pid)
            try:
                proc.wait(timeout=30)
            except subprocess.TimeoutExpired:
                pass
        else:
            proc.wait(timeout=60)

        crash = parse_crash(log_file)
        audit_new = audit_new_lines(audit_before)
        rec = {
            "round": rnd, "outcome": outcome, "crash": crash,
            # cdb 退出码 = debuggee 退出码（101=panic-unwind / 1=main Err /
            # 127 族=fastfail·栈溢出·截断；判读流程输入之一）。
            "cdb_rc": proc.returncode,
            "err": err_detail, "audit_new": audit_new,
            "minutes": round((time.time() - t0) / 60, 1),
        }
        results.append(rec)
        print(f"[*] outcome={outcome} audit_new={len(audit_new)}")
        if crash:
            print(f"[!!!] ROUND {rnd} CRASH: {crash['death']} "
                  f"code={crash['code']}")
            for fr in crash["fault_frames"][:6]:
                print(f"      {fr}")
        for ln in audit_new:
            print(f"    audit+: {ln}")
        if crash and not args.all:
            break

    print("\n=== SUMMARY ===")
    print(json.dumps(results, indent=1, ensure_ascii=False))
    out = LOG_DIR / "summary.json"
    with open(out, "w", encoding="utf-8") as f:
        json.dump(results, f, indent=1, ensure_ascii=False)
    print(f"[*] summary -> {out}")
    crashed = [r for r in results if r["crash"]]
    return 0 if not crashed else 1


if __name__ == "__main__":
    sys.exit(main())
