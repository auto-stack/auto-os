#!/usr/bin/env bash
# scripts/check-app-submodules.sh — apps/ 子模块健康闸门（PLAN-876 phase：OS876-SUBMODULE 根因守卫）
#
# 背景（2026-10-28 实测）：`apps/` 下 24 个子模块工作树曾被清空——只剩 `.git`
# 指针、索引里是成批 `D `（暂存删除）。表征：桌面注册表从 30 条缩到 4 条、
# 小组件面 `faces=0`，而宿主只表现为"图标不对/没有小组件"。
#
# 本闸门把这些签名变成一条确定性检查，可手工/CI 在启动桌面或合入前跑：
#   ①任何子模块未初始化（`git submodule status` 行首 `-`）或指针不匹配（`+`/`U`）；
#   ②任何已初始化子模块出现"批量暂存删除"（`^D ` 行 > 阈值）= 工作区被清空；
#   ③`apps/` 下有 `pac.at` 的目录数低于 `apps.manifest` 的 active repo 条目数。
#
# 用法：
#   bash scripts/check-app-submodules.sh            # 体检本仓（APP_OS_ROOT 可指向另一检出）
#   bash scripts/check-app-submodules.sh --self-test # 用临时仓验证检测器本身
# 退出码：0 = healthy；1 = 发现缺口（输出修复建议）；2 = 环境/用法错误。
set -uo pipefail

THRESHOLD="${APP_SUBMODULE_WIPE_THRESHOLD:-5}"

fail() { printf '[app-submodules] FAIL: %s\n' "$*" >&2; }

# 单个工作树里"批量暂存删除"的行数（wipe 签名）。
wiped_deletions() {
  local dir="$1"
  git -C "$dir" status --porcelain 2>/dev/null | grep -c '^D ' || true
}

scan() {
  local os_root os_status rc=0
  os_root="${APP_OS_ROOT:-$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)}"
  cd "$os_root" || { fail "cannot cd $os_root"; return 2; }

  os_status="$(git submodule status --recursive 2>/dev/null)"
  if [ -z "$os_status" ]; then
    fail "no submodules registered（apps/ 为空壳？）"
    return 1
  fi

  local line
  while IFS= read -r line; do
    case "$line" in
      -*) printf '[app-submodules] UNINITIALIZED: %s\n' "$line"; rc=1 ;;
      +*|U*) printf '[app-submodules] MISMATCH: %s\n' "$line"; rc=1 ;;
    esac
  done <<< "$os_status"

  # ② 批量暂存删除签名（已初始化模块）
  local path n
  while read -r _sha path _desc; do
    [ -d "$path" ] || continue
    n="$(wiped_deletions "$path")"
    if [ "${n:-0}" -gt "$THRESHOLD" ]; then
      printf '[app-submodules] WIPED? %s staged_deletions=%s\n' "$path" "$n"
      rc=1
    fi
  done <<< "$os_status"

  # ③ pac.at 覆盖 vs manifest active repo 条目
  if [ -f apps.manifest ]; then
    local manifest_repos pac_dirs
    manifest_repos="$(python - <<'PY' 2>/dev/null || echo 0
import json
d = json.load(open('apps.manifest', encoding='utf-8'))
print(sum(1 for a in d.get('apps', []) if a.get('status') == 'active' and a.get('kind') == 'repo'))
PY
)"
    pac_dirs="$(find apps -maxdepth 2 -name pac.at -type f 2>/dev/null | wc -l | tr -d ' ')"
    printf '[app-submodules] manifest_repo_entries=%s pac_at_dirs=%s\n' "${manifest_repos:-0}" "${pac_dirs:-0}"
    if [ "${manifest_repos:-0}" -gt 0 ] && [ "${pac_dirs:-0}" -lt "${manifest_repos:-0}" ]; then
      printf '[app-submodules] COVERAGE: %s manifest repo entries lack a pac.at on disk\n' \
        "$(( manifest_repos - pac_dirs ))"
      rc=1
    fi
  fi

  if [ "$rc" -eq 0 ]; then
    printf '[app-submodules] healthy\n'
  else
    printf '%s\n' '[app-submodules] 修复建议：git submodule update --init --recursive' \
      '  （若某模块索引已成批删除而文件不在：先在模块内 git reset --hard HEAD，再 recheck）' >&2
  fi
  return "$rc"
}

self_test() {
  local tmp rc=0
  tmp="$(mktemp -d 2>/dev/null || mktemp -d -t p876)"
  git -C "$tmp" init -q || { fail "git init failed"; return 2; }
  git -C "$tmp" config user.email t@example.com
  git -C "$tmp" config user.name t
  local i
  for i in 1 2 3 4 5 6 7 8 9 10; do printf 'x' > "$tmp/f$i.txt"; done
  git -C "$tmp" add -A && git -C "$tmp" commit -qm fixture
  local clean_n wiped_n
  clean_n="$(wiped_deletions "$tmp")"
  git -C "$tmp" rm -rq . >/dev/null 2>&1
  wiped_n="$(wiped_deletions "$tmp")"
  printf '[self-test] clean_staged_deletions=%s wiped_staged_deletions=%s threshold=%s\n' \
    "$clean_n" "$wiped_n" "$THRESHOLD"
  if [ "${clean_n:-1}" -ne 0 ] || [ "${wiped_n:-0}" -le "$THRESHOLD" ]; then
    fail "detector does not separate clean vs wiped states"
    rc=1
  fi
  rm -rf "$tmp"
  [ "$rc" -eq 0 ] && printf '[self-test] detector OK\n'
  return "$rc"
}

case "${1:-}" in
  --self-test) self_test ;;
  "" ) scan ;;
  * ) printf 'usage: %s [--self-test]\n' "$0" >&2; exit 2 ;;
esac
