#!/usr/bin/env bash
# PLAN-067 T-02 A-01/A-03：Notes FeaturePack 服务可执行件发布入口。
#
# 背景（auto-os PLAN-067 §5 r3 A-01）：发布件必须与本次构建严格绑定——成员名
# 取自本构建日志（"✓ Generated Rust server: <member>/"），exe 取自该成员在
# 显式缓存根的产物且 mtime 晚于本次构建起点；**无任何跨根/全局最新回退**——
# 构建失败即发布失败（不拿另一根旧产物冒充新成功）。回执绑定
# source/工具/构建开关/支持端点与 SHA（A-03）。
#
# 产物来源（唯一经生产验证的通道）：`auto service --server rust` 生成链构建
# 生成后端，复制为**稳定命名**发布件 <notes>/target/release/notes-service.exe
# （生成件名含路径哈希 id8 随检出路径变，不能进 manifest；稳定名对齐
# musk/jade-garden 先例）。manifest daemon.bin 走生产路径语义消费
# （auto-ui session.rs/generic_daemon_from_manifest join + .exe 补全）。
#
# 用法（auto-os 仓根/worktree 根）：
#   scripts/build_notes_service.sh [--notes-root PATH] [--port PORT]
#                                  [--wait-secs N] [--target-root PATH]
#
# 解析序（AGENTS §2 红线：永不建链接）：notes 根 AUTO_NOTES_ROOT →
# ../auto-notes → D:/autostack/auto-notes；auto CLI（auto-man debug 构建）
# AUTO_MAN_ROOT → ../auto-man → D:/autostack/auto-man，缺席 = 具名失败
# （本脚本不代建工具链）。
#
# 进程纪律：只回收本脚本 spawn 的自有 PID（按 back exe 路径归属清孤儿）；
# 不使用 taskkill /IM。
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

EXPLICIT_TARGET_ROOT=""
if [[ -z "$TARGET_ROOT" ]]; then
  # 缺省缓存根 = producer 受管 generated-consumers（与 auto service 缺省一致）。
  TARGET_ROOT="$LANG_ROOT/target/generated-consumers"
else
  EXPLICIT_TARGET_ROOT="1"
fi
mkdir -p "$TARGET_ROOT"
TARGET_ROOT="$(win_path "$TARGET_ROOT")"

NOTES_SHA="$(git -C "$NOTES" rev-parse HEAD)" || fail "notes HEAD unavailable"
TOOL_SHA="$(git -C "$(dirname "$(dirname "$AUTO_BIN")")" rev-parse HEAD 2>/dev/null || echo unknown)"
BUILD_START="$(date +%s)"
printf '[publish] notes=%s\n[publish] notes HEAD=%s\n[publish] auto=%s\n[publish] tool HEAD=%s\n[publish] target-root=%s\n[publish] build port=%s\n' \
  "$NOTES" "$NOTES_SHA" "$AUTO_BIN" "$TOOL_SHA" "$TARGET_ROOT" "$BUILD_PORT"

# 生成 + 构建（等待窗 --wait-secs，缺省 600s 开发 profile 预算）。
# 显式 --target-root 经 AUTO_GENERATED_TARGET_ROOT 钉住构建落点（生成器校验
# 根须存在——上方已 mkdir）；缺省不设 env（CLI 自行解析 generated-consumers，
# 产物定位 glob 同根）。
# AUTO_ALLOW_PARTIAL_DB=1：asset_upload（NOTES-002/PLAN-730 UploadRequest 形）
# 无 db.rs 匹配，734 strict 门拒绝混合态服务——notes 仓自有 battery 同款 env
# 接受面（A-03：回执如实列明该开关与实际支持端点，不推导完整 API 可用）。
SVC_ENV=(AUTO_ALLOW_PARTIAL_DB=1)
[[ -n "$EXPLICIT_TARGET_ROOT" ]] && SVC_ENV+=("AUTO_GENERATED_TARGET_ROOT=$TARGET_ROOT")
( cd "$NOTES" && env "${SVC_ENV[@]}" "$AUTO_BIN" service --server rust -B "$BUILD_PORT" ) >"$LOG" 2>&1 &
SVC_PID=$!
own_kill() {
  # Git Bash $! 是 MSYS pid：taskkill 需 Windows pid（ps -o winpid 映射）。
  # 按路径清孤儿**仅限显式 --target-root**（自有私有根）——缺省共享
  # generated-consumers 根可能承载其他会话的 back 进程，绝不按路径清扫
  # （只回收自有：auto.exe 进程树 /T 覆盖 cargo→backend 全链）。
  kill "$SVC_PID" 2>/dev/null || true
  local wpid
  wpid="$( { ps -p "$SVC_PID" -o winpid= 2>/dev/null || true; } | tr -d ' ' || true)"
  if [[ -n "$wpid" ]]; then
    taskkill //T //F //PID "$wpid" >/dev/null 2>&1 || true
  fi
  if [[ -n "$EXPLICIT_TARGET_ROOT" ]]; then
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
if [[ $READY -ne 1 ]]; then
  # A-01 反例守门：构建未就绪 = 发布失败。此处绝不回退定位任何既有产物
  # （跨根/全局最新均不允许）——fail 前不触碰发布件。
  fail "auto service 未在 ${WAIT_SECS}s 内就绪（log: $LOG；末尾见下）$(tail -5 "$LOG" | tr '\n' ' ')" 3
fi

# A-01 严格绑定：成员名取自**本构建日志**的生成行（非 glob 最新）。
MEMBER="$(grep -oE 'Generated Rust server: [A-Za-z0-9._-]+/' "$LOG" | tail -1 | sed 's#Generated Rust server: ##; s#/##')"
[[ -n "$MEMBER" ]] || fail "构建日志缺 'Generated Rust server: <member>/' 行（log: $LOG）" 4
# 产物定位限本缓存根：显式根 = <root>/release/<member>.exe；producer 受管根
# = <root>/<ctx>/release/<member>.exe。要求唯一命中，无跨根/全局回退。
CANDS="$( { ls "$TARGET_ROOT/release/$MEMBER.exe" 2>/dev/null || true; ls "$TARGET_ROOT"/*/release/"$MEMBER.exe" 2>/dev/null || true; } | sort -u )"
N_CANDS="$(printf '%s' "$CANDS" | grep -c . || true)"
if [[ -z "$CANDS" || "$N_CANDS" != "1" ]]; then
  fail "本构建成员产物定位非唯一或缺席（member=$MEMBER, root=$TARGET_ROOT, n=${N_CANDS:-0}；log: $LOG）" 4
fi
EXE="$CANDS"
[[ -f "$EXE" ]] || fail "本构建成员产物缺席：$EXE（log: $LOG）" 4
# 绑定门 = READY（本构建的服务真从该根跑通）+ 成员名出自本日志 + 根内唯一定位。
# mtime 作关联记录：晚于构建起点 = fresh-build；早于 = 无变更增量重建（不重链），
# 须有 cargo Finished 行佐证构建确实在本根完整跑过，否则拒绝。
EXE_MTIME="$(stat -c %Y "$EXE" 2>/dev/null || date -r "$EXE" +%s 2>/dev/null || echo 0)"
if (( EXE_MTIME < BUILD_START )); then
  grep -qE "Finished .*(release)?" "$LOG" 2>/dev/null \
    || fail "产物 mtime 早于构建起点且日志无 cargo Finished——绑定证据不足，拒绝发布（A-01；log: $LOG）" 5
  BUILD_CORRELATION="no-change-incremental-rebuild (mtime ${EXE_MTIME} < start ${BUILD_START}; cargo Finished in log)"
else
  BUILD_CORRELATION="fresh-build (mtime ${EXE_MTIME} >= start ${BUILD_START})"
fi

mkdir -p "$NOTES/target/release"
PUB="$NOTES/target/release/notes-service.exe"
cp -f "$EXE" "$PUB"
EXE_SHA="$(sha256sum "$EXE" | awk '{print $1}')"
PUB_SHA="$(sha256sum "$PUB" | awk '{print $1}')"
printf '[publish] member=%s\n[publish] build correlation=%s\n[publish] source exe=%s\n[publish] source sha256=%s\n[publish] published=%s\n[publish] published sha256=%s\n' \
  "$MEMBER" "$BUILD_CORRELATION" "$EXE" "$EXE_SHA" "$PUB" "$PUB_SHA"
# A-03 支持面回执：构建开关 + 实际支持端点（fp v1 全部 db.rs 匹配；
# asset_upload 为 NOTES-002 已知混合态接受面，不代表完整 API 可用）。
cat <<RECEIPT
[publish] build switches: AUTO_ALLOW_PARTIAL_DB=1 (asset_upload=NOTES-002/PLAN-730 mixed-state accepted per notes battery; fp v1 endpoints all db.rs-matched)
[publish] supported fp surface: GET /api/health (notes.fpack/1); POST /api/fp/v1/{capture,get,list,update,receipt}; POST /api/fp/draft/{put,get} (main-app scratch, no policy gate)
[publish] service profile: media_scan/photo_scan off (auto service default); NOT covered by this receipt: asset_upload mixed-state writes
[publish] receipt: notes HEAD=$NOTES_SHA member=$MEMBER tool HEAD=$TOOL_SHA exe=$PUB sha256=$PUB_SHA
RECEIPT
