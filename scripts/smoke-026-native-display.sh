#!/usr/bin/env bash
# scripts/smoke-026-native-display.sh — PLAN-026 T-07 真机冒烟：native
# display 族（image/icon/badge/divider/spacer/avatar/scroll/grid/center）
# queue 档真 exe 入桌面。
#
# 流程：构建 ui_desktop（lang 侧 plan-026-dev）→ acceptance 模式启动桌面
# （apps-dir = scratch026 载体注册表根）→ DesktopBus `launch` 孵化
# profile-card.exe / display026.exe / converter.exe → MCP 截图留痕
# （display 族视觉面）→ 清理。
#
# 已证边界（与 smoke-025 同口径）：OS 级键入/点击自动化未打通（P020-D4
# 债）→ IME/display 交互闭环由协议级 p026_native_display_arm（lang 仓，
# AUTO_DESKTOP_E2E=1 + scratch026 载体）承载——真 exe + 真管道 + 宿主
# broker 生产路径（broker_ime_commit 注入→联动帧）全链已证；本脚本补
# 桌面壳视觉面留痕。
#
# 载体生成（不入库）：lang 仓 scratch026/ 三 app——
#   cd scratch026/<app> && auto build --gen-only -r rust
#   （pac 附 desktop_exe/desktop_render 声明；member 编译入
#   examples/rust-workspace 共享 workspace，exe 萄 lang 仓 target/debug/）
#
# 用法：bash scripts/smoke-026-native-display.sh
# env：AUTO_LANG_ROOT（缺省 .wt/lang-026 组兄弟）、SMOKE_APPS_DIR
#      （缺省 lang-026 组 scratch026）、AUTOUI_MCP_PORT。
set -euo pipefail

OS_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
LANG_ROOT="${AUTO_LANG_ROOT:-$(dirname "$OS_ROOT")/lang-026/auto-lang}"
APPS_DIR="${SMOKE_APPS_DIR:-$(dirname "$OS_ROOT")/lang-026/scratch026}"
PORT="${AUTOUI_MCP_PORT:-9439}"
UI_DESKTOP="$LANG_ROOT/target/debug/examples/ui_desktop.exe"

[ -f "$UI_DESKTOP" ] || { echo "ui_desktop 缺失：先在 $LANG_ROOT 构建（cargo build -p auto-lang --features ui-iced --example ui_desktop）" >&2; exit 1; }
[ -f "$APPS_DIR/004-profile-card/pac.at" ] || { echo "载体注册表缺失：$APPS_DIR（004-profile-card + desktop_exe 声明）" >&2; exit 1; }
[ -f "$APPS_DIR/026-display/pac.at" ] || { echo "载体注册表缺失：$APPS_DIR（026-display + desktop_exe 声明）" >&2; exit 1; }
[ -f "$APPS_DIR/003-converter/pac.at" ] || { echo "载体注册表缺失：$APPS_DIR（003-converter + desktop_exe 声明）" >&2; exit 1; }

echo "[smoke-026] build ui_desktop ($LANG_ROOT)"
(cd "$LANG_ROOT" && CARGO_INCREMENTAL=0 cargo build -p auto-lang --features ui-iced --example ui_desktop)

echo "[smoke-026] launch desktop (acceptance, mcp=:$PORT, apps-dir=$APPS_DIR)"
cd "$OS_ROOT"
AUTOUI_ACCEPTANCE=1 AUTOUI_MCP_PORT="$PORT" "$UI_DESKTOP" --apps-dir "$(cygpath -m "$APPS_DIR" 2>/dev/null || echo "$APPS_DIR")" &
DESKTOP_PID=$!
trap 'kill $DESKTOP_PID 2>/dev/null || true' EXIT

sleep 8
echo "[smoke-026] desktop up（MCP :$PORT）；用 acceptance channel launch"
echo "  004-profile-card / 026-display / 003-converter 后 MCP 截图（视觉"
echo "  留痕 → lang 仓 docs/plans/reports/assets/026/，帧 dump 见"
echo "  p026_native_display_arm 三腿）。交互闭环由协议级 p026 承载（见头注）。"

kill $DESKTOP_PID 2>/dev/null || true
echo "[smoke-026] done"
