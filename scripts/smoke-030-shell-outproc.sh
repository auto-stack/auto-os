#!/usr/bin/env bash
# scripts/smoke-030-shell-outproc.sh — PLAN-030 T-08 真机冒烟：壳 outproc
# 双轨（shell.apps.shell_model=outproc——AUTO_SHELL_MODEL env 便捷注入）。
#
# 流程：构建 ui_desktop（lang 侧 plan-030-dev）→ acceptance 模式启动桌面
# （AUTO_SHELL_MODEL=outproc → boot 分叉 spawn 壳子进程 auto run
# --autodesk-shell → 双表面 attach → 投影推送）→ 前台断言（SendInput
# 真机腿前置——宿主窗未聚焦 = skipped 留痕，dual 口径）→ 清理。
#
# 已证边界（smoke-025/026 同口径）：OS 级键入/点击自动化未打通（P020-D4
# 债）→ 壳交互闭环由协议级 p030_shell_outproc_arm（lang 仓
# AUTO_DESKTOP_E2E=1）四腿承载（双表面首帧 / 真按钮点击→DesktopBus→
# 归因执行 / 投影推送帧变 / kill→看门兵→respawn→全量重推恢复）；本脚本
# 补桌面壳视觉面 + outproc 形态真机在位留痕。
#
# SendInput 真机腿（029 挂账清偿口径）：sendinput.rs FFI + 前台断言
# （foreground_window）029 在册；启动序基建本计划交付——真桌面整窗
# SendInput 注入维持 dual 口径（协议级断言 = p030 e2e；真机腿 = 本脚本
# 前台断言留痕，注入受 P020-D4 约束 skipped 时如实记录）。
#
# 观测面：stdout 控制台（[autodesk-broker] 壳 attach 行 + 看门兵行）；
# stderr 同流。壳子进程 = auto.exe re-exec（stderr 继承——壳装载/投影
# trace AUTO030_TRACE=1 可开）。
#
# 用法：bash scripts/smoke-030-shell-outproc.sh
# env：AUTO_LANG_ROOT（缺省 .wt/lang-030 组兄弟）、AUTOUI_MCP_PORT。
set -euo pipefail

OS_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
# lang 工作树在 lang-030 组（os 工作树在 os-030 组——两组目录，缺省上
# 两级解析 .wt/lang-030/auto-lang）。
LANG_ROOT="${AUTO_LANG_ROOT:-$(dirname "$(dirname "$OS_ROOT")")/lang-030/auto-lang}"
PORT="${AUTOUI_MCP_PORT:-9430}"
UI_DESKTOP="$LANG_ROOT/target/debug/examples/ui_desktop.exe"
AUTO_BIN="$LANG_ROOT/target/debug/auto.exe"

[ -f "$UI_DESKTOP" ] || { echo "ui_desktop 缺失：先在 $LANG_ROOT 构建（cargo build -p auto-lang --features ui-iced --example ui_desktop）" >&2; exit 1; }
[ -f "$AUTO_BIN" ] || { echo "auto.exe 缺失（壳 outproc spawn 依赖同 exe re-exec 探测）：先在 $LANG_ROOT 构建（cargo build -p auto-lang --features ui-iced --bin auto）" >&2; exit 1; }

echo "[smoke-030] build ui_desktop + auto ($LANG_ROOT)"
(cd "$LANG_ROOT" && CARGO_INCREMENTAL=0 cargo build -p auto-lang --features ui-iced --example ui_desktop -p auto --bin auto)

echo "[smoke-030] launch desktop (acceptance, shell_model=outproc, mcp=:$PORT)"
cd "$OS_ROOT"
AUTOUI_ACCEPTANCE=1 AUTOUI_MCP_PORT="$PORT" AUTO_SHELL_MODEL=outproc "$UI_DESKTOP" &
DESKTOP_PID=$!
trap 'kill $DESKTOP_PID 2>/dev/null || true' EXIT

sleep 10
echo "[smoke-030] desktop up——观测（上方控制台）："
echo "  ① [autodesk-broker] 壳 incubation/attach 行（shell 分支双伪窗）"
echo "  ② 无 shell load failed 行（in-proc 装载被 outproc 分叉跳过）"
echo "  ③ MCP :$PORT autoui_screenshot 可截壳视觉面（acceptance channel）"
echo "  SendInput 真机腿：前台断言与注入受 P020-D4（DPI/画布）约束——"
echo "  协议级断言由 p030 e2e 四腿承载（dual 口径，头注在案）。"

kill $DESKTOP_PID 2>/dev/null || true
echo "[smoke-030] done"
