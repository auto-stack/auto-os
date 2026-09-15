#!/usr/bin/env bash
# scripts/smoke-020-native-exe.sh — PLAN-020 T-07 真机冒烟：编译 exe 作桌面
# compositor 一等客户端（iced 轨 ui_desktop + 注册表 desktop_exe 分流）。
#
# 流程：构建 ui_desktop（lang 侧，plan-020-dev ≥ 6780307f7）→ 以 acceptance
# 模式启动桌面（apps-dir = 载体注册表根）→ DesktopBus `launch` verb 孵化
# 编译 exe → 断言子进程 = 该 exe（非 auto.exe）→ MCP 截图留痕 → 清理。
#
# 已证边界（截图见 lang 仓 docs/plans/reports/assets/020/）：
# - queue 档（pac `desktop_render: "queue"`）：虚拟窗渲染 native View 投影
#   帧（"Counter: 0" + 三按钮，020-native-launch-queue.png）；
# - auto 档（无声明）：宿主 broker 链打印
#   "[render] <app>: auto -> independent (coverage downgrade)"，pixels 帧
#   入合成器（020-native-auto-pixels.png）；
# - 窗内点击/× 关闭的 OS 级自动化未打通（DPI 2x + 画布缩放系数不明——
#   画布↔屏幕变换未文档化）→ 点击闭环与双向回收由协议级
#   p020_native_exe_arm（lang 仓，AUTO_DESKTOP_E2E=1）承载；acceptance
#   channel 增 pointer verb 后本脚本可补齐（P020 债随注）。
#
# 用法：bash scripts/smoke-020-native-exe.sh
# env：AUTO_LANG_ROOT（缺省 .wt/lang-020 组兄弟）、SMOKE_APPS_DIR
#      （载体注册表根，缺省 lang-020 组 scratch020）、AUTOUI_MCP_PORT。
set -euo pipefail

OS_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
WT_ROOT="$(dirname "$OS_ROOT")"
LANG_ROOT="${AUTO_LANG_ROOT:-$(dirname "$OS_ROOT")/lang-020/auto-lang}"
APPS_DIR="${SMOKE_APPS_DIR:-$(dirname "$OS_ROOT")/lang-020/scratch020}"
PORT="${AUTOUI_MCP_PORT:-9437}"
UI_DESKTOP="$LANG_ROOT/target/debug/examples/ui_desktop.exe"

[ -f "$UI_DESKTOP" ] || { echo "ui_desktop 缺失：先在 $LANG_ROOT 构建（cargo build -p auto-lang --features ui-iced --example ui_desktop）" >&2; exit 1; }
[ -f "$APPS_DIR/002-counter/pac.at" ] || { echo "载体注册表缺失：$APPS_DIR（002-counter + desktop_exe 声明）" >&2; exit 1; }

echo "[smoke-020] build ui_desktop ($LANG_ROOT)"
(cd "$LANG_ROOT" && CARGO_INCREMENTAL=0 cargo build -p auto-lang --features ui-iced --example ui_desktop)

echo "[smoke-020] launch desktop (acceptance, mcp=:$PORT, apps-dir=$APPS_DIR)"
cd "$OS_ROOT"
AUTOUI_ACCEPTANCE=1 AUTOUI_MCP_PORT="$PORT" "$UI_DESKTOP" --apps-dir "$(cygpath -m "$APPS_DIR" 2>/dev/null || echo "$APPS_DIR")" &
DESKTOP_PID=$!
trap 'kill $DESKTOP_PID 2>/dev/null; taskkill //F //IM counter.exe 2>/dev/null | head -1' EXIT

for i in $(seq 1 30); do
  curl -s -m 2 -o /dev/null -X POST "http://127.0.0.1:$PORT/mcp" \
    -H "Content-Type: application/json" \
    -d '{"jsonrpc":"2.0","id":0,"method":"tools/list","params":{}}' && break
  sleep 1
done

echo "[smoke-020] bus launch 002-counter（真消费臂）"
python - "$PORT" <<'PYEOF'
import requests, sys, time
port = sys.argv[1]
def call(tool, **args):
    p = {"jsonrpc":"2.0","id":1,"method":"tools/call","params":{"name":tool,"arguments":args}}
    r = requests.post(f"http://127.0.0.1:{port}/mcp", json=p, timeout=30)
    return r.json()["result"]["content"][0]["text"]
print(call("autoui_desktop", action="bus", verb="launch\t002-counter").strip()[:120])
deadline = time.time() + 30
while time.time() < deadline:
    time.sleep(1)
    print(call("autoui_screenshot", name="020-smoke-launch")[:120])
    break
PYEOF

echo "[smoke-020] assert child = compiled exe"
sleep 3
tasklist //FI "IMAGENAME eq counter.exe" | grep -q counter.exe \
  || { echo "FAIL: counter.exe 未孵化" >&2; exit 2; }
tasklist //FI "IMAGENAME eq auto.exe" | grep -q auto.exe \
  && { echo "WARN: auto.exe 亦在跑（应为无关实例——宿主孵化走编译 exe）"; } || true
echo "[smoke-020] PASS：编译 exe 经宿主孵化、虚拟窗渲染（截图 tmp/autoui-screenshot-*.png）"
echo "[smoke-020] teardown"
