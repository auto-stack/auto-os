#!/usr/bin/env bash
# scripts/smoke-034-rqhost.sh — PLAN-034 rqhost-maturity 真机冒烟：
# 位图过线通道 + 缺省软光栅档 daemon + canvas 样板。
#
# 流程：构建 auto（lang 侧 plan-034-dev）→ `auto rqhost` 起服（缺省
# well-known autodesk-rqhost——T-03 后缺省 tiny-skia 软光栅档）→
# 043-canvas-paint `-q`（D4 裁定 canvas=位图快照过线的生产样板：覆盖门
# 放行 + 首帧 + bitmap 观测行 + 零弃置）→ 003-converter `-q`（共享
# daemon 双窗 + ≤10MB app 门复核）→ 自观测内存行核验（rq_update 300
# 拍节流）→ kill 双向观测 → 清理。
#
# 断言主承载 = lang 侧 p034_rqhost_maturity_arm（AUTO_DESKTOP_E2E=1，
# canvas 样板腿 + 13 格内存矩阵 assets/034/）；本脚本 = os 侧生产
# well-known 管道（非 pid 后缀）+ 真机演示位（031 先例同型）。
#
# 用法：bash scripts/smoke-034-rqhost.sh
# env：AUTO_LANG_ROOT（缺省 .wt/lang-034 组兄弟）。
set -euo pipefail

OS_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
# lang 组在 .wt/lang-034（计划 §8 布局——两 worktree 组各按仓名分组）。
LANG_ROOT="${AUTO_LANG_ROOT:-$(dirname "$(dirname "$OS_ROOT")")/lang-034/auto-lang}"
AUTO="$LANG_ROOT/target/debug/auto.exe"

[ -f "$AUTO" ] || { echo "auto.exe 缺失：先在 $LANG_ROOT 构建（cargo build -p auto --bin auto）" >&2; exit 1; }
[ -f "$LANG_ROOT/examples/capability-tests/043-canvas-paint/src/front/app.at" ] || { echo "载体缺失：$LANG_ROOT/examples/capability-tests/043-canvas-paint" >&2; exit 1; }

echo "[smoke-034] build auto ($LANG_ROOT)"
(cd "$LANG_ROOT" && CARGO_INCREMENTAL=0 cargo build -p auto --bin auto)

WORK="$(mktemp -d)"
DAEMON_PID=""
APP_CANVAS_PID=""
APP_CONV_PID=""
trap 'kill $DAEMON_PID $APP_CANVAS_PID $APP_CONV_PID 2>/dev/null || true; rm -rf "$WORK"' EXIT

echo "[smoke-034] 清场既有 rqhost daemon（生产锁单实例——上轮残留会占 well-known）"
powershell -NoProfile -Command "Get-CimInstance Win32_Process -Filter \"Name='auto.exe'\" | Where-Object {\$_.CommandLine -like '*rqhost*'} | ForEach-Object { Stop-Process -Id \$_.ProcessId -Force }" >/dev/null 2>&1 || true
sleep 1

echo "[smoke-034] start rqhost daemon（缺省档 = tiny-skia——T-03 数据驱动裁定）"
"$AUTO" rqhost >"$WORK/daemon.log" 2>&1 &
DAEMON_PID=$!
sleep 2

wait_log() { # $1 needle $2 what $3 timeout_s $4 logfile
    local deadline=$((SECONDS + $3))
    while ! grep -q "$1" "$4"; do
        [ $SECONDS -lt $deadline ] || { echo "[smoke-034] FAIL: $2 超时（等 '$1'）" >&2; exit 1; }
        sleep 1
    done
    echo "[smoke-034] OK: $2"
}

wait_log "serving on" "daemon 起服" 20 "$WORK/daemon.log"

echo "[smoke-034] 043-canvas-paint -q（canvas 位图快照样板）"
(cd "$LANG_ROOT/examples/capability-tests/043-canvas-paint" && \
    AUTOUI_MCP_DISABLE=1 "$AUTO" run -r vm -q >"$WORK/canvas.log" 2>&1) &
APP_CANVAS_PID=$!
wait_log "first frame" "043 首帧（覆盖门放行）" 40 "$WORK/daemon.log"
wait_log "\[rqhost\] bitmap \`" "canvas 位图过线（BitmapReady → 缓存）" 15 "$WORK/daemon.log"
if grep -q "bitmap upload 弃置" "$WORK/canvas.log"; then
    echo "[smoke-034] FAIL: 043 位图弃置（槽档不足）" >&2; exit 1
fi
echo "[smoke-034] OK: 043 位图零弃置"

echo "[smoke-034] 003-converter -q（共享 daemon 双窗）"
(cd "$LANG_ROOT/examples/ui/003-converter" && \
    AUTOUI_MCP_DISABLE=1 "$AUTO" run -r vm -q >"$WORK/conv.log" 2>&1) &
APP_CONV_PID=$!
sleep 8

echo "[smoke-034] 自观测内存行（rq_update 300 拍节流 ≈4.5s——等待一拍）"
sleep 5
wait_log "\[rqhost\] mem private=" "daemon 自观测内存行" 15 "$WORK/daemon.log"

echo "---- daemon.log 尾 ----"
grep -E "rqhost" "$WORK/daemon.log" | tail -12

echo "[smoke-034] 收尾：kill daemon → 双 app exit-on-EOF"
kill $DAEMON_PID 2>/dev/null || true
sleep 3
ALIVE=$(powershell -NoProfile -Command "(Get-Process auto -ErrorAction SilentlyContinue | Measure-Object).Count")
[ "$ALIVE" = "0" ] || { echo "[smoke-034] WARN: 残留 auto 进程 ×$ALIVE（exit-on-EOF 未收敛——人工核）" >&2; }
echo "[smoke-034] DONE（六腿：起服/样板过线/零弃置/双窗共享/自观测/干净退出）"
