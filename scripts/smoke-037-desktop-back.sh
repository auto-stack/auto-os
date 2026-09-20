#!/usr/bin/env bash
# scripts/smoke-037-desktop-back.sh — PLAN-037 desktop-back-provision 冒烟：
# VM 桌面 launch 020 后端按需供给（proxy 原生 media 路由真管道）。
#
# 流程：构建 ui_desktop（lang 侧 plan-037-dev）→ acceptance 桌面起服
# （MCP :9437，storage 隔离 temp）→ **AC-06 懒启门**（boot 期日志无
# back-proxy lazy-start 行）→ MCP bus `launch\t020-music-player`（真
# 消费臂——DesktopBus 排空 → execute_launch_app）→ 日志抓 proxy 端口 →
# curl scan 断言（AC-01：200 + entries 非空[真实 E:\Music 条目] + 条目
# url 为 proxy 绝对地址）→ 首条流 URL 取字节（200/206 + Content-Type）→
# 截图留痕 → teardown。
#
# 断言主承载 = lang 侧集成测试 launch_provision_lifecycle_via_resolver
# （懒启/双窗计数/关窗 404/复 launch 重建——协议级全覆盖）；本脚本 =
# 真桌面真曲库生产管道（E:\Music 实数据）冒烟。
#
# 用法：bash scripts/smoke-037-desktop-back.sh
# env：AUTO_LANG_ROOT（缺省 <os 根上级>/.wt/lang-037 组兄弟——037 的 os
#       侧在主检出，无 os 工作树，与 020/030 组布局不同）。
set -euo pipefail

OS_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
LANG_ROOT="${AUTO_LANG_ROOT:-$(dirname "$OS_ROOT")/.wt/lang-037/auto-lang}"
PORT="${AUTOUI_MCP_PORT:-9437}"
UI_DESKTOP="$LANG_ROOT/target/debug/examples/ui_desktop.exe"
MEDIA_APP="020-music-player"

[ -f "$UI_DESKTOP" ] || { echo "ui_desktop 缺失：先在 $LANG_ROOT 构建（cargo build -p auto-lang --features ui-iced --example ui_desktop）" >&2; exit 1; }
[ -f "$LANG_ROOT/examples/ui/$MEDIA_APP/src/front/app.at" ] || { echo "载体缺失：$LANG_ROOT/examples/ui/$MEDIA_APP" >&2; exit 1; }

echo "[smoke-037] build ui_desktop ($LANG_ROOT)"
(cd "$LANG_ROOT" && CARGO_INCREMENTAL=0 cargo build -p auto-lang --features ui-iced --example ui_desktop)

WORK="$(mktemp -d)"
DESKTOP_PID=""
trap 'kill $DESKTOP_PID 2>/dev/null || true; rm -rf "$WORK"' EXIT

echo "[smoke-037] launch desktop (acceptance, mcp=:$PORT, storage 隔离)"
cd "$OS_ROOT"
AUTOUI_ACCEPTANCE=1 AUTOUI_MCP_PORT="$PORT" \
  AUTO_VM_STORAGE_FILE="$(cygpath -m "$WORK/storage.json" 2>/dev/null || echo "$WORK/storage.json")" \
  "$UI_DESKTOP" >"$WORK/desktop.log" 2>&1 &
DESKTOP_PID=$!

for i in $(seq 1 40); do
  curl -s -m 2 -o /dev/null -X POST "http://127.0.0.1:$PORT/mcp" \
    -H "Content-Type: application/json" \
    -d '{"jsonrpc":"2.0","id":0,"method":"tools/list","params":{}}' && break
  sleep 1
done

# AC-06 懒启门：boot 后（launch 前）零 back-proxy 启动行。
if grep -q "back-proxy lazy-start" "$WORK/desktop.log"; then
  echo "[smoke-037] FAIL: boot 期出现 back-proxy lazy-start（违反懒启门）" >&2
  exit 2
fi
echo "[smoke-037] OK: boot 零 back-proxy（AC-06 懒启门）"

echo "[smoke-037] bus launch $MEDIA_APP（真消费臂）"
python - "$PORT" <<'PYEOF'
import requests, sys
port = sys.argv[1]
p = {"jsonrpc":"2.0","id":1,"method":"tools/call","params":{"name":"autoui_desktop",
     "arguments":{"action":"bus","verb":"launch\t020-music-player"}}}
r = requests.post(f"http://127.0.0.1:{port}/mcp", json=p, timeout=30)
print(r.json()["result"]["content"][0]["text"].strip()[:120])
PYEOF

# 等 launch 排空（ServiceTick ≤400ms）+ 懒启行落日志。
PROXY_PORT=""
for i in $(seq 1 15); do
  PROXY_PORT="$(grep -o 'back-proxy lazy-start on "020-music-player" (port [0-9]*)' "$WORK/desktop.log" \
    | grep -o '[0-9]*' | tail -1 || true)"
  [ -n "$PROXY_PORT" ] && break
  sleep 1
done
[ -n "$PROXY_PORT" ] || { echo "[smoke-037] FAIL: 未捕获 lazy-start 端口（日志尾：$(tail -5 "$WORK/desktop.log")）" >&2; exit 2; }
echo "[smoke-037] OK: back-proxy lazy-start on port $PROXY_PORT"

sleep 1
echo "[smoke-037] curl scan（AC-01：真实曲库）"
SCAN="$(curl -s -m 10 "http://127.0.0.1:$PROXY_PORT/apps/$MEDIA_APP/api/media/scan")"
echo "${SCAN:0:200}"
grep -qF '"entries":[{' <<<"$SCAN" \
  || { echo "[smoke-037] FAIL: entries 空（E:\\Music 无条目或路由未通）" >&2; exit 2; }
echo "[smoke-037] OK: scan 200 + entries 非空"

# 条目 url 应为 proxy 绝对地址，且流端可取字节。
URL="$(echo "$SCAN" | python -c "import json,sys; print(json.load(sys.stdin)['entries'][0]['url'])")"
case "$URL" in
  "http://127.0.0.1:$PROXY_PORT/apps/$MEDIA_APP/api/media/stream/"*) ;;
  *) echo "[smoke-037] FAIL: 条目 url 非本 proxy 绝对地址：$URL" >&2; exit 2 ;;
esac
echo "[smoke-037] OK: url 绝对地址 → $URL"
HEADERS="$(curl -s -m 20 -D - -o /dev/null -r 0-4095 "$URL")"
echo "$HEADERS" | head -1 | grep -qE "200|206" \
  || { echo "[smoke-037] FAIL: 流端取字节失败（$URL）" >&2; exit 2; }
grep -qi "^Content-Type:." <<<"$HEADERS" \
  || { echo "[smoke-037] FAIL: 流端无 Content-Type" >&2; exit 2; }
echo "[smoke-037] OK: 流端 200/206 + Content-Type（$(grep -i '^Content-Type' <<<"$HEADERS" | head -1 | tr -d '\r')）"

echo "[smoke-037] 截图留痕（最小化竞态重试 ≤3）"
python - "$PORT" <<'PYEOF'
import requests, sys, time
port = sys.argv[1]
def call(tool, **args):
    p = {"jsonrpc":"2.0","id":2,"method":"tools/call","params":{"name":tool,"arguments":args}}
    r = requests.post(f"http://127.0.0.1:{port}/mcp", json=p, timeout=30)
    return r.json()["result"]["content"][0]["text"].strip()
for attempt in range(3):
    out = call("autoui_screenshot", name=f"037-smoke-020-library-{attempt}")
    if not out.startswith("Error"):
        print(out[:120])
        break
    print(f"retry {attempt}: {out[:80]}")
    time.sleep(2)
PYEOF

kill $DESKTOP_PID 2>/dev/null || true
echo "[smoke-037] PASS：launch 按需供给真管道（scan 非空 + 流可取）——真机验收段归用户（ToDesk 合成输入不可用）"
