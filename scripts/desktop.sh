#!/usr/bin/env bash
# scripts/desktop.sh — Stage B §3-a 桌面薄包装（PLAN-009 T1，bash 版，语义同 desktop.ps1）
#
# 用法：./scripts/desktop.sh [vue|iced] [--fullscreen] [--dry-run]
#   vue（缺省）  CWD=<lang>/examples/desktop-host；auto run --desktop
#                （注入 AUTO_OS_ROOT + AUTO_DESKTOP_APPS_EXTRA=<os>/apps）
#   iced         CWD=<本仓根>；cargo run ui_desktop（../auto-os/apps 兄弟探测自命中）
#
# 解析序（本仓 AGENTS §2）：$AUTO_LANG_ROOT → 兄弟 ../auto-lang → D:/autostack/auto-lang
set -euo pipefail

# 本仓根（脚本位于 <os>/scripts/；DESKTOP_OS_ROOT env 可显式覆盖——指向
# 另一伞形检出/主检出用，缺省=脚本所在仓根）。
OS_ROOT="${DESKTOP_OS_ROOT:-$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)}"
OS_PARENT="$(dirname "$OS_ROOT")"

TRACK="vue"
FULLSCREEN=0
DRYRUN=0
for a in "$@"; do
  case "$a" in
    vue|iced) TRACK="$a" ;;
    --fullscreen) FULLSCREEN=1 ;;
    --dry-run) DRYRUN=1 ;;
    *) echo "unknown arg: $a（支持 vue|iced --fullscreen --dry-run）" >&2; exit 2 ;;
  esac
done

lang_probe() { [ -d "$1/crates/auto-lang" ] && [ -d "$1/examples/desktop-host" ]; }

LANG_ROOT=""
for c in "${AUTO_LANG_ROOT:-}" "$OS_PARENT/auto-lang" "D:/autostack/auto-lang"; do
  if [ -n "$c" ] && lang_probe "$c"; then LANG_ROOT="$(cd "$c" && pwd)"; break; fi
done
if [ -z "$LANG_ROOT" ]; then
  echo "auto-lang 未解析（解析序：AUTO_LANG_ROOT env → 兄弟 auto-lang → D:/autostack/auto-lang）" >&2
  exit 1
fi

export AUTO_OS_ROOT="$OS_ROOT"   # manifest 聚合 env 臂（P-3：设置即权威）

if [ "$TRACK" = "vue" ]; then
  AUTO_CLI=""
  command -v auto >/dev/null 2>&1 && AUTO_CLI="$(command -v auto)"
  for c in "$LANG_ROOT/target/release/auto" "$LANG_ROOT/target/release/auto.exe" \
           "$LANG_ROOT/target/debug/auto" "$LANG_ROOT/target/debug/auto.exe"; do
    [ -z "$AUTO_CLI" ] && [ -f "$c" ] && AUTO_CLI="$c"
  done
  if [ -z "$AUTO_CLI" ]; then
    echo "auto CLI 未找到：请先在 auto-lang 构建（cargo build -p auto）或加入 PATH" >&2
    exit 1
  fi
  export AUTO_DESKTOP_APPS_EXTRA="$OS_ROOT/apps"
  echo "[desktop.sh] track=vue  lang=$LANG_ROOT  os=$OS_ROOT  auto=$AUTO_CLI"
  echo "[desktop.sh] AUTO_OS_ROOT=$AUTO_OS_ROOT  AUTO_DESKTOP_APPS_EXTRA=$AUTO_DESKTOP_APPS_EXTRA"
  if [ "$DRYRUN" = 1 ]; then echo "[dry-run] cd $LANG_ROOT/examples/desktop-host; auto run --desktop"; exit 0; fi
  cd "$LANG_ROOT/examples/desktop-host"
  exec "$AUTO_CLI" run --desktop
else
  ARGS=""
  [ "$FULLSCREEN" = 1 ] && ARGS="--fullscreen"
  echo "[desktop.sh] track=iced  lang=$LANG_ROOT  os=$OS_ROOT  args=$ARGS"
  echo "[desktop.sh] AUTO_OS_ROOT=$AUTO_OS_ROOT（apps 容器经 CWD=本仓根兄弟探测自命中）"
  # T1 实证教训：cargo 按**调用方 CWD** 发现 .cargo/config.toml（/STACK:32MB
  # link 旗标在 lang 仓 config 内）——从本仓 CWD cargo run 会丢旗标致主栈
  # 1MB 起动即溢出。故：lang 侧 build（config 生效）+ 本仓 CWD 直接 exec exe
  # （../auto-os/apps 兄弟探测自命中保持）。
  if [ "$DRYRUN" = 1 ]; then
    echo "[dry-run] (cd $LANG_ROOT && cargo build -p auto-lang --features ui-iced --example ui_desktop)"
    echo "[dry-run] cd $OS_ROOT; exec $LANG_ROOT/target/debug/examples/ui_desktop $ARGS"
    exit 0
  fi
  (cd "$LANG_ROOT" && cargo build -p auto-lang --features ui-iced --example ui_desktop)
  cd "$OS_ROOT"
  exec "$LANG_ROOT/target/debug/examples/ui_desktop" $ARGS
fi
