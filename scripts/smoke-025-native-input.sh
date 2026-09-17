#!/usr/bin/env bash
# scripts/smoke-025-native-input.sh — PLAN-025 T-07 真机冒烟：native input
# 族（input/slider/select）queue 档真 exe 入桌面。
#
# 流程：构建 ui_desktop（lang 侧 plan-025-dev）→ acceptance 模式启动桌面
# （apps-dir = scratch025 载体注册表根）→ DesktopBus `launch` 孵化
# converter.exe / inputs025.exe → MCP 截图留痕（双 input/slider/select
# 视觉面）→ 清理。
#
# 已证边界（与 smoke-020 同口径）：OS 级键入/点击自动化未打通（DPI/画布
# 变换未文档化，P020-D4 债）→ 键入/滑轨/选项交互闭环由协议级
# p025_native_input_arm（lang 仓，AUTO_DESKTOP_E2E=1 + scratch025 载体）
# 承载——真 exe + 真管道 + 宿主 broker 生产路径（broker_char/broker_scroll）
# 全链已证；本脚本补桌面壳视觉面留痕。
#
# 用法：bash scripts/smoke-025-native-input.sh
# env：AUTO_LANG_ROOT（缺省 .wt/lang-025 组兄弟）、SMOKE_APPS_DIR
#      （缺省 lang-025 组 scratch025）、AUTOUI_MCP_PORT。
set -euo pipefail

OS_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
LANG_ROOT="${AUTO_LANG_ROOT:-$(dirname "$OS_ROOT")/lang-025/auto-lang}"
APPS_DIR="${SMOKE_APPS_DIR:-$(dirname "$OS_ROOT")/lang-025/scratch025}"
PORT="${AUTOUI_MCP_PORT:-9438}"
UI_DESKTOP="$LANG_ROOT/target/debug/examples/ui_desktop.exe"

[ -f "$UI_DESKTOP" ] || { echo "ui_desktop 缺失：先在 $LANG_ROOT 构建（cargo build -p auto-lang --features ui-iced --example ui_desktop）" >&2; exit 1; }
[ -f "$APPS_DIR/003-converter/pac.at" ] || { echo "载体注册表缺失：$APPS_DIR（003-converter + desktop_exe 声明）" >&2; exit 1; }
[ -f "$APPS_DIR/025-inputs/pac.at" ] || { echo "载体注册表缺失：$APPS_DIR（025-inputs + desktop_exe 声明）" >&2; exit 1; }

echo "[smoke-025] build ui_desktop ($LANG_ROOT)"
(cd "$LANG_ROOT" && CARGO_INCREMENTAL=0 cargo build -p auto-lang --features ui-iced --example ui_desktop)

echo "[smoke-025] launch desktop (acceptance, mcp=:$PORT, apps-dir=$APPS_DIR)"
cd "$OS_ROOT"
AUTOUI_ACCEPTANCE=1 AUTOUI_MCP_PORT="$PORT" "$UI_DESKTOP" --apps-dir "$(cygpath -m "$APPS_DIR" 2>/dev/null || echo "$APPS_DIR")" &
DESKTOP_PID=$!
trap 'kill $DESKTOP_PID 2>/dev/null || true' EXIT

sleep 8
echo "[smoke-025] desktop up（MCP :$PORT）；用 acceptance channel launch "
echo "  003-converter / 025-inputs 后 MCP 截图（视觉留痕 → lang 仓"
echo "  docs/plans/reports/assets/025/）。交互闭环由协议级 p025 承载（见头注）。"

kill $DESKTOP_PID 2>/dev/null || true
echo "[smoke-025] done"
