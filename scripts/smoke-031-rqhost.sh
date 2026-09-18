#!/usr/bin/env bash
# scripts/smoke-031-rqhost.sh — PLAN-031 rqhost 第四形态真机冒烟：
# `auto run -q`（vm 轨）× 共享 rqhost daemon 原生窗。
#
# 流程：构建 auto（lang 侧 plan-031-dev）→ `auto rqhost` 起服（缺省
# well-known autodesk-rqhost）→ 双 `-q` app 并发（003-converter +
# 001-helloworld——AC-02 共享单 daemon 双窗）→ 观测行核验（adopt/开窗/
# 首帧）→ 降级演示位（未解析 image 的最小 demo → [drawlist-image]
# unresolved 占位观测行，I3）→ kill 双向观测（app kill→EOF 窗回收；
# daemon kill→app exit-on-EOF）→ 清理。
#
# 断言主承载 = lang 侧 p031_rqhost_arm（AUTO_DESKTOP_E2E=1 六腿全景，
# 含竞态/resize/截图留痕 reports/assets/031/）；本脚本 = os 侧生产
# well-known 管道（非 pid 后缀）+ 真机演示位。
#
# 用法：bash scripts/smoke-031-rqhost.sh
# env：AUTO_LANG_ROOT（缺省 .wt/lang-031 组兄弟）。
set -euo pipefail

OS_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
LANG_ROOT="${AUTO_LANG_ROOT:-$(dirname "$OS_ROOT")/lang-031/auto-lang}"
AUTO="$LANG_ROOT/target/debug/auto.exe"

[ -f "$AUTO" ] || { echo "auto.exe 缺失：先在 $LANG_ROOT 构建（cargo build -p auto --bin auto）" >&2; exit 1; }
[ -f "$LANG_ROOT/examples/ui/003-converter/src/front/app.at" ] || { echo "载体缺失：$LANG_ROOT/examples/ui/003-converter" >&2; exit 1; }

echo "[smoke-031] build auto ($LANG_ROOT)"
(cd "$LANG_ROOT" && CARGO_INCREMENTAL=0 cargo build -p auto --bin auto)

WORK="$(mktemp -d)"
trap 'kill $DAEMON_PID $APP1_PID $APP2_PID $APPD_PID 2>/dev/null || true; rm -rf "$WORK"' EXIT

echo "[smoke-031] 清场既有 rqhost daemon（生产锁单实例——上轮残留会占 well-known）"
powershell -NoProfile -Command "Get-CimInstance Win32_Process -Filter \"Name='auto.exe'\" | Where-Object {\$_.CommandLine -like '*rqhost*'} | ForEach-Object { Stop-Process -Id \$_.ProcessId -Force }" >/dev/null 2>&1 || true
sleep 1

echo "[smoke-031] start rqhost daemon（缺省 well-known autodesk-rqhost）"
"$AUTO" rqhost >"$WORK/daemon.log" 2>&1 &
DAEMON_PID=$!
sleep 2
grep -q "serving on autodesk-rqhost" "$WORK/daemon.log" || { echo "daemon 起服失败："; cat "$WORK/daemon.log"; exit 1; }

echo "[smoke-031] spawn 双 -q app（共享 daemon——AC-02）"
(cd "$LANG_ROOT/examples/ui/003-converter" && exec "$AUTO" run -r vm -q) >"$WORK/app1.log" 2>&1 &
APP1_PID=$!
(cd "$LANG_ROOT/examples/ui/001-helloworld" && exec "$AUTO" run -r vm -q) >"$WORK/app2.log" 2>&1 &
APP2_PID=$!

wait_for() { # <needle> <file> <what>
  local i=0
  while ! grep -q "$1" "$2" 2>/dev/null; do
    i=$((i + 1)); [ $i -gt 100 ] && { echo "[smoke-031] FAIL: $3 超时（$2）"; cat "$2" "$WORK/daemon.log" 2>/dev/null; exit 1; }
    sleep 0.3
  done
}

wait_for "first frame \`App\`" "$WORK/daemon.log" "双 app 首帧（计数≥2 由下方核验）"
FRAMES=$(grep -c "first frame \`App\`" "$WORK/daemon.log")
[ "$FRAMES" -ge 2 ] || { echo "[smoke-031] FAIL: 共享 daemon 双窗未达（first frame x$FRAMES）"; exit 1; }
echo "[smoke-031] OK: 单 daemon 双 app 原生窗（first frame x$FRAMES）"

echo "[smoke-031] 降级演示位（未解析 image → 占位 + 观测行，I3）"
IMG_APP="$WORK/p031-img"
mkdir -p "$IMG_APP/src/front"
cat >"$IMG_APP/src/front/app.at" <<'AT'
widget P031Img {
    view {
        image (src: "Z:/definitely/missing-031.png") {
            style: "w-[120px] h-[80px]"
        }
    }
}
AT
cat >"$IMG_APP/pac.at" <<'AT'
name: "p031-img"
version: "1.0.0"
scene: "ui"
render: "vm"
title: "P031Img"
window: "480x320"
AT
(cd "$IMG_APP" && exec "$AUTO" run -r vm -q) >"$WORK/appd.log" 2>&1 &
APPD_PID=$!
wait_for "unresolved src (placeholder fallback): Z:/definitely/missing-031.png" "$WORK/daemon.log" "降级观测行"
echo "[smoke-031] OK: 超覆盖降级显式（占位 + 观测行可见）"

echo "[smoke-031] kill 双向"
kill $APPD_PID 2>/dev/null || true
wait $APPD_PID 2>/dev/null || true
wait_for "断连（EOF）——窗回收" "$WORK/daemon.log" "app kill → 窗回收"
echo "[smoke-031] OK: app EOF 窗回收"
kill $DAEMON_PID 2>/dev/null || true
wait $DAEMON_PID 2>/dev/null || true
DEAD=0
for _ in $(seq 1 40); do
  kill -0 $APP1_PID 2>/dev/null || { DEAD=1; break; }
  sleep 0.3
done
[ "$DEAD" = 1 ] || { echo "[smoke-031] FAIL: daemon 死后 app 未 exit-on-EOF"; cat "$WORK/app1.log"; exit 1; }
grep -q "host lost" "$WORK/app1.log" && echo "[smoke-031] OK: daemon 死 → app exit-on-EOF（观测行在 app1.log）"

kill $APP1_PID $APP2_PID 2>/dev/null || true
echo "[smoke-031] done（六腿全景断言见 lang p031_rqhost_arm；留痕 reports/assets/031/）"
