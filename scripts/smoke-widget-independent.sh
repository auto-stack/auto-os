#!/usr/bin/env bash
# scripts/smoke-widget-independent.sh — 2026-10-10 用户裁定（小组件独立于
# app 窗）真机冒烟：后端 app（013-todo / 020-music-player）随孵化同链供给
# ——面板直显活卡（占位卡退役）、播放不开窗、双击/activate 升格同会话、
# 关窗降格组件存活（会话与后端不随窗释放）。
#
# 流程：构建 ui_desktop → acceptance 起服（MCP :端口，storage 隔离）→
# AC-06 懒启门（boot 期零 back-proxy）→ bus dashboard_toggle → 日志抓
# [dashboard] refresh statuses（断言 013/020 = hatched 非 placeholder）→
# lazy-start 恰一次 → proxy scan 020 非空 → handler PlayPause（孵化会话
# 直呼，断言零 launch 行 + 截图）→ bus activate 升格（断言零 launch 行 +
# promote 日志行）→ bus close 降格 → 面板关/开一轮（断言仍 hatched 且无
# 第二次 lazy-start = 会话与后端存活）→ 截图留痕。
#
# 用法：bash scripts/smoke-widget-independent.sh
set -euo pipefail

OS_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
PORT="${AUTOUI_MCP_PORT:-9438}"

LANG_ROOT=""
for c in "${AUTO_LANG_ROOT:-}" "$(dirname "$OS_ROOT")/auto-lang" "D:/autostack/auto-lang"; do
  [ -n "$c" ] && [ -d "$c/crates/auto-lang" ] && LANG_ROOT="$(cd "$c" && pwd)" && break
done
[ -n "$LANG_ROOT" ] || { echo "auto-lang 未解析" >&2; exit 1; }

echo "[smoke-widget] build ui_desktop ($LANG_ROOT)"
(cd "$LANG_ROOT" && CARGO_INCREMENTAL=0 cargo build -p auto-lang --features ui-iced,mpv-widget --example ui_desktop)
UI_DESKTOP="$LANG_ROOT/target/debug/examples/ui_desktop.exe"

WORK="$(mktemp -d)"
DESKTOP_PID=""
cleanup() {
  kill $DESKTOP_PID 2>/dev/null || true
  [ -f "$WORK/desktop.log" ] && cp "$WORK/desktop.log" "$OS_ROOT/tmp/smoke-widget-desktop.log" 2>/dev/null || true
}
trap 'cleanup; rm -rf "$WORK"' EXIT

echo "[smoke-widget] launch desktop (acceptance mcp=:$PORT)"
cd "$OS_ROOT"
# AUTO_MPV_LIB 显式传入时透传（${var:+...} 内嵌赋值会被 bash 当命令名，分臂）。
if [ -n "${AUTO_MPV_LIB:-}" ]; then
  AUTOUI_ACCEPTANCE=1 AUTOUI_MCP_PORT="$PORT" AUTO_MPV_LIB="$AUTO_MPV_LIB" \
    AUTO_VM_STORAGE_FILE="$(cygpath -m "$WORK/storage.json" 2>/dev/null || echo "$WORK/storage.json")" \
    "$UI_DESKTOP" >"$WORK/desktop.log" 2>&1 &
else
  AUTOUI_ACCEPTANCE=1 AUTOUI_MCP_PORT="$PORT" \
    AUTO_VM_STORAGE_FILE="$(cygpath -m "$WORK/storage.json" 2>/dev/null || echo "$WORK/storage.json")" \
    "$UI_DESKTOP" >"$WORK/desktop.log" 2>&1 &
fi
DESKTOP_PID=$!

for i in $(seq 1 40); do
  curl -s -m 2 -o /dev/null -X POST "http://127.0.0.1:$PORT/mcp" \
    -H "Content-Type: application/json" \
    -d '{"jsonrpc":"2.0","id":0,"method":"tools/list","params":{}}' && break
  sleep 1
done

mcp_bus() {
  python - "$PORT" "$1" <<'PYEOF'
import requests, sys
port, verb = sys.argv[1], sys.argv[2]
p = {"jsonrpc":"2.0","id":1,"method":"tools/call","params":{"name":"autoui_desktop",
     "arguments":{"action":"bus","verb":verb}}}
r = requests.post(f"http://127.0.0.1:{port}/mcp", json=p, timeout=30)
print(r.json()["result"]["content"][0]["text"].strip()[:120])
PYEOF
}

mcp_handler() {
  python - "$PORT" "$1" "$2" <<'PYEOF'
import requests, sys
port, app, handler = sys.argv[1], sys.argv[2], sys.argv[3]
p = {"jsonrpc":"2.0","id":1,"method":"tools/call","params":{"name":"autoui_desktop",
     "arguments":{"action":"handler","app":app,"handler":handler}}}
r = requests.post(f"http://127.0.0.1:{port}/mcp", json=p, timeout=30)
print(r.json()["result"]["content"][0]["text"].strip()[:120])
PYEOF
}

shot() {
  python - "$PORT" "$1" <<'PYEOF'
import requests, sys, time
port, name = sys.argv[1], sys.argv[2]
for attempt in range(3):
    p = {"jsonrpc":"2.0","id":2,"method":"tools/call","params":{"name":"autoui_screenshot",
         "arguments":{"name":name}}}
    r = requests.post(f"http://127.0.0.1:{port}/mcp", json=p, timeout=30)
    out = r.json()["result"]["content"][0]["text"].strip()
    if not out.startswith("Error"):
        print(f"shot {name}: {out[:100]}")
        sys.exit(0)
    print(f"retry {attempt}: {out[:80]}")
    time.sleep(2)
sys.exit(1)
PYEOF
}

# 注：常驻小组件层 boot 即挂载并显示（用户裁定 2026-09-17）——boot 期
# 孵化 + 供给是设计行为；MCP 就绪与 boot 孵化存在竞态，此处不设"boot 零
# 供给"门，统一以首次 refresh 后的"恰一次供给"对账。
# ① 等 boot 面板首帧 refresh → 孵化 + 供给落账。
FACES=""
for i in $(seq 1 20); do
  FACES="$(grep -o '\[dashboard\] refresh: faces=.*' "$WORK/desktop.log" | tail -1 || true)"
  echo "$FACES" | grep -q "020-music-player" && break
  sleep 1
done
[ -n "$FACES" ] || { echo "FAIL: 无 [dashboard] refresh 日志" >&2; exit 2; }
echo "$FACES"
echo "$FACES" | grep -q '("013-todo", "hatched")' \
  || { echo "FAIL: 013-todo 非 hatched（应随孵化供给）" >&2; exit 2; }
echo "$FACES" | grep -q '("020-music-player", "hatched")' \
  || { echo "FAIL: 020-music-player 非 hatched" >&2; exit 2; }
echo "OK: 013-todo / 020-music-player 孵化直显活卡（占位卡退役）"

LAZY_N="$(grep -c 'back-proxy lazy-start' "$WORK/desktop.log" || true)"
[ "$LAZY_N" = "1" ] || { echo "FAIL: lazy-start 次数 $LAZY_N ≠ 1" >&2; exit 2; }
PROXY_PORT="$(grep -o 'back-proxy lazy-start on "[^"]*" (port [0-9]*)' "$WORK/desktop.log" | grep -o '[0-9]*' | tail -1)"
echo "OK: 后端随孵化供给（proxy :$PROXY_PORT，恰一次）"

sleep 1
# /apps/020-music-player is the stable API ID, not a filesystem path.
SCAN="$(curl -s -m 10 "http://127.0.0.1:$PROXY_PORT/apps/020-music-player/api/media/scan")"
echo "${SCAN:0:160}"
grep -qF '"entries":[{' <<<"$SCAN" || { echo "FAIL: 020 scan 空" >&2; exit 2; }
echo "OK: 孵化会话后端可取数（scan 非空）"

# ② 孵化态播放：handler 直呼 PlayPause（mini 按钮同款 handler 面），
#    断言不开窗（零 launch 行）。
LAUNCH_BEFORE="$(grep -c 'launch_app(' "$WORK/desktop.log" || true)"
mcp_handler "020-music-player" "PlayPause"
sleep 2
LAUNCH_NOW="$(grep -c 'launch_app(' "$WORK/desktop.log" || true)"
[ "$LAUNCH_NOW" = "$LAUNCH_BEFORE" ] || { echo "FAIL: 播放触发了 launch（弹 app 窗）" >&2; exit 2; }
grep -q "CALL_HANDLER_ERR\|HandlerNotFound" "$WORK/desktop.log" \
  && echo "WARN: PlayPause dispatch 有错误行（检查 mini 按钮柄）" >&2
echo "OK: 孵化态 PlayPause 零 launch（不弹 app 窗）"
shot "widget-independent-1-play-in-widget"

# ③ activate 升格（双击同语义）：同会话开窗，非重复 launch。
mcp_bus $'activate	020-music-player'
sleep 2
PROMOTE="$(grep -c 'promote `020-music-player`' "$WORK/desktop.log" || true)"
[ "$PROMOTE" = "1" ] || { echo "FAIL: 未走升格路径（promote 行缺失）" >&2; exit 2; }
LAUNCH_AFTER="$(grep -c 'launch_app(' "$WORK/desktop.log" || true)"
[ "$LAUNCH_AFTER" = "$LAUNCH_BEFORE" ] || { echo "FAIL: 升格走了 launch（重复会话）" >&2; exit 2; }
WID="$(grep 'promote `020-music-player`' "$WORK/desktop.log" | tail -1 | awk '{print $NF}')"
echo "OK: activate 升格同会话（wid=$WID，零重复供给）"
shot "widget-independent-2-promoted-window"

# ④ 关窗降格：会话与后端存活——面板关/开一轮后仍 hatched、无二次供给。
mcp_bus "close	$WID"
sleep 1
mcp_bus "dashboard_toggle"   # 关面板
sleep 1
mcp_bus "dashboard_toggle"   # 重开面板
FACES2=""
for i in $(seq 1 20); do
  FACES2="$(grep -o '\[dashboard\] refresh: faces=.*' "$WORK/desktop.log" | tail -1 || true)"
  echo "$FACES2" | grep -q '020-music-player' && break
  sleep 1
done
echo "$FACES2"
echo "$FACES2" | grep -q '("020-music-player", "hatched")' \
  || { echo "FAIL: 关窗后 020 未保持 hatched（降格失败）" >&2; exit 2; }
LAZY_N2="$(grep -c 'back-proxy lazy-start' "$WORK/desktop.log" || true)"
[ "$LAZY_N2" = "1" ] || { echo "FAIL: 关窗后二次供给（lazy-start ×$LAZY_N2）" >&2; exit 2; }
echo "OK: 关窗降格——组件会话与后端存活（无二次孵化/供给）"
shot "widget-independent-3-alive-after-close"

echo "[smoke-widget] PASS：小组件独立于 app 窗（孵化供给 / 播放不开窗 / 升格同会话 / 关窗存活）"
echo "  日志留档：tmp/smoke-widget-desktop.log"
