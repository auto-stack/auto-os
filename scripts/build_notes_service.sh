#!/usr/bin/env bash
# PLAN-067 T-02 P-01：Notes FeaturePack 服务可执行件发布入口。
#
# 背景（auto-os PLAN-067 §5 r2 P-01）：apps.manifest daemon.bin 走生产路径
# 语义（session.rs:4460 repo_dir.join + .exe 补全），而 notes 条目此前填的是
# 命令行 "auto service --server rust -B 17819"（生成器不产独立 exe，直启必
# 失败）。本脚本兑现「与现有路径契约兼容的已构建服务可执行件及发布路径」：
# 经 auto service 生成链构建生成后端，复制为**稳定命名**发布件
# <notes>/target/release/notes-service.exe（生成件名含路径哈希 id8，随检出
# 路径变，不能进 manifest；稳定名对齐 musk/jade-garden 先例）。
#
# 用法（auto-os 仓根/worktree 根）：
#   scripts/build_notes_service.sh [--notes-root PATH] [--port PORT]
#
# 解析序（AGENTS §2 红线：永不建链接）：notes 根 AUTO_NOTES_ROOT →
# ../auto-notes → D:/autostack/auto-notes；auto CLI（auto-man debug 构建）
# AUTO_MAN_ROOT → ../auto-man → D:/autostack/auto-man，缺席 = 具名失败
# （本脚本不代建工具链）。
#
# 进程纪律：只回收本脚本 spawn 的自有 PID；不使用 taskkill /IM。
set -euo pipefail

fail() { printf 'build_notes_service: %s\n' "$1" >&2; exit "${2:-1}"; }
win_path() {
  if command -v cygpath >/dev/null 2>&1; then cygpath -am "$1";
  elif [[ "$1" = /* ]]; then printf '%s\n' "$1";
  else printf '%s/%s\n' "$PWD" "$1"; fi
}

NOTES_ROOT="" ; BUILD_PORT="17919" ; WAIT_SECS="600" ; TARGET_ROOT=""
while [[ $# -gt 0 ]]; do
  case "$1" in
    --notes-root) [[ $# -ge 2 && -n "$2" ]] || fail "missing value for --notes-root" 2
                  NOTES_ROOT="$2"; shift 2 ;;
    --port)       [[ $# -ge 2 && -n "$2" ]] || fail "missing value for --port" 2
                  BUILD_PORT="$2"; shift 2 ;;
    --wait-secs)  [[ $# -ge 2 && -n "$2" ]] || fail "missing value for --wait-secs" 2
                  WAIT_SECS="$2"; shift 2 ;;
    --target-root) [[ $# -ge 2 && -n "$2" ]] || fail "missing value for --target-root" 2
                  TARGET_ROOT="$2"; shift 2 ;;
    *) fail "unknown argument: $1 (--notes-root PATH --port PORT --wait-secs N --target-root PATH)" 2 ;;
  esac
done

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
OS_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"

notes_root() {
  local c
  if [[ -n "$NOTES_ROOT" ]]; then
    [[ -f "$NOTES_ROOT/pac.at" ]] || fail "notes root invalid: $NOTES_ROOT"
    win_path "$NOTES_ROOT"; return
  fi
  for c in "${AUTO_NOTES_ROOT:-}" "$OS_ROOT/../auto-notes" "D:/autostack/auto-notes"; do
    [[ -n "$c" && -f "$c/pac.at" ]] && { win_path "$c"; return; }
  done
  fail "notes root not found (AUTO_NOTES_ROOT -> ../auto-notes -> main checkout)"
}
NOTES="$(notes_root)"

auto_cli() {
  local c
  for c in "${AUTO_MAN_ROOT:-}/target/debug/auto.exe" \
           "$OS_ROOT/../auto-man/target/debug/auto.exe" \
           "D:/autostack/auto-man/target/debug/auto.exe"; do
    [[ -n "$c" && -f "$c" ]] && { win_path "$c"; return; }
  done
  fail "auto CLI not found (AUTO_MAN_ROOT -> ../auto-man -> main checkout, target/debug/auto.exe)"
}
AUTO_BIN="$(auto_cli)"

lang_root() {
  local c
  for c in "${AUTO_LANG_ROOT:-}" "$OS_ROOT/../auto-lang" "D:/autostack/auto-lang"; do
    [[ -n "$c" && -f "$c/crates/auto-lang/Cargo.toml" ]] && { win_path "$c"; return; }
  done
  fail "auto-lang root not found (AUTO_LANG_ROOT -> ../auto-lang -> main checkout)"
}
LANG_ROOT="$(lang_root)"

ART_HOME="${PLAN_ARTIFACTS_HOME:-D:/autostack/plan-artifacts/plan-067}"
mkdir -p "$ART_HOME"
LOG="$ART_HOME/build-notes-service-$(date +%Y%m%d-%H%M%S).log"

NOTES_SHA="$(git -C "$NOTES" rev-parse HEAD)" || fail "notes HEAD unavailable"
printf '[publish] notes=%s\n[publish] notes HEAD=%s\n[publish] auto=%s\n[publish] build port=%s\n' \
  "$NOTES" "$NOTES_SHA" "$AUTO_BIN" "$BUILD_PORT"

# 生成 + 构建（auto service 生成链 = 唯一经生产验证的产物来源）。默认端口
# 17919：避开 17819，构建期不与在跑服务/宿主冲突；等待窗 --wait-secs
# （缺省 600s 开发 profile，冷构建可调大）。
# --target-root（可选）：AUTO_GENERATED_TARGET_ROOT 显式生成缓存根——与其他
# 会话共享的 producer 受管缓存根解耦（活动会话持锁/lease 时不再互相阻塞）；
# 根须先存在（生成器校验），孤儿 back exe 也因此可按路径归属本脚本回收。
# AUTO_ALLOW_PARTIAL_DB=1：asset_upload（NOTES-002/PLAN-730 UploadRequest 形）
# 无 db.rs 匹配，734 strict 门拒绝混合态服务——notes 仓自有 battery
# （tests/basic/verify_featurepack_service.py 等）同款 env 接受面，属 Notes
# 迁移期已知混合态；fp v1 端点全部有 db.rs 匹配，不受影响。
if [[ -n "$TARGET_ROOT" ]]; then
  mkdir -p "$TARGET_ROOT"
  TARGET_ROOT="$(win_path "$TARGET_ROOT")"
fi
SVC_ENV=(AUTO_ALLOW_PARTIAL_DB=1)
[[ -n "$TARGET_ROOT" ]] && SVC_ENV+=("AUTO_GENERATED_TARGET_ROOT=$TARGET_ROOT")
( cd "$NOTES" && env "${SVC_ENV[@]}" "$AUTO_BIN" service --server rust -B "$BUILD_PORT" ) >"$LOG" 2>&1 &
SVC_PID=$!
own_kill() {
  # Git Bash $! 是 MSYS pid：taskkill 需 Windows pid（ps -o winpid 映射），
  # 两者都尽力回收；显式 target root 时按 back exe 路径归属清孤儿。
  kill "$SVC_PID" 2>/dev/null || true
  local wpid
  wpid="$( { ps -p "$SVC_PID" -o winpid= 2>/dev/null || true; } | tr -d ' ' || true)"
  if [[ -n "$wpid" ]]; then
    taskkill //T //F //PID "$wpid" >/dev/null 2>&1 || true
  fi
  if [[ -n "$TARGET_ROOT" ]]; then
    powershell -NoProfile -Command "Get-CimInstance Win32_Process -Filter \"Name like 'auto-notes-%-back.exe'\" | Where-Object { \$_.CommandLine -like '$TARGET_ROOT*' } | ForEach-Object { taskkill /T /F /PID \$_.ProcessId }" >/dev/null 2>&1 || true
  fi
}
READY=0
deadline=$(( $(date +%s) + WAIT_SECS ))
while (( $(date +%s) < deadline )); do
  if grep -q "AUTO_SERVICE_READY" "$LOG" 2>/dev/null; then READY=1; break; fi
  if ! kill -0 "$SVC_PID" 2>/dev/null; then
    wait "$SVC_PID" 2>/dev/null || true
    break
  fi
  sleep 2
done
own_kill
sleep 1
[[ READY -eq 1 ]] || fail "auto service 未在 ${WAIT_SECS}s 内就绪（log: $LOG；末尾见下）$(tail -5 "$LOG" | tr '\n' ' ')" 3

# 生成件定位：显式 target root 优先，回退 producer 受管缓存根
# （auto-lang target/generated-consumers/<ctx>/release/auto-notes-<id8>-back.exe），
# 取 mtime 最新。
EXE=""
[[ -n "$TARGET_ROOT" ]] && EXE="$(ls -t "$TARGET_ROOT"/release/auto-notes-*-back.exe 2>/dev/null | head -1)"
if [[ -z "$EXE" ]]; then
  EXE="$(ls -t "$LANG_ROOT"/target/generated-consumers/*/release/auto-notes-*-back.exe 2>/dev/null | head -1)"
fi
[[ -n "$EXE" && -f "$EXE" ]] || fail "生成后端 exe 未定位（target root 或 $LANG_ROOT/target/generated-consumers/*/release）" 4

mkdir -p "$NOTES/target/release"
PUB="$NOTES/target/release/notes-service.exe"
cp -f "$EXE" "$PUB"
EXE_SHA="$(sha256sum "$EXE" | awk '{print $1}')"
PUB_SHA="$(sha256sum "$PUB" | awk '{print $1}')"
printf '[publish] source exe=%s\n[publish] source sha256=%s\n[publish] published=%s\n[publish] published sha256=%s\n' \
  "$EXE" "$EXE_SHA" "$PUB" "$PUB_SHA"
printf '[publish] receipt: notes HEAD=%s exe=%s sha256=%s\n' "$NOTES_SHA" "$PUB" "$PUB_SHA"
