#!/usr/bin/env bash
# PLAN-067 T-02 P-05（OS 面）：FeaturePack 受管 policy 供给与书籍范围授予。
#
# 契约（docs/specs/communication/featurepack-v0.md + auto-notes repository.at
# fp_policy_path）：服务私有 policy——NOTES_FP_POLICY env 覆盖，缺省
# <data>/fp-policy.json；文件缺席 = 全拒绝（FeaturePack 关闭态）。数据目录
# 解析与 notes repository.at data_dir() 同律：NOTES_DATA_DIR → APPDATA →
# XDG_DATA_HOME → HOME。生产 ensure spawn 只带端口 env（auto-ui
# osconfig_daemon.rs:358），因此生产 policy 固定落缺省 <data>/fp-policy.json。
#
# 用法：
#   scripts/fp_provision_policy.sh                 # 确保 fp-reader 条目存在（缺席则
#                                                  # 生成 256-bit hex cred），打印凭据回执
#   scripts/fp_provision_policy.sh add-book BID    # 授予 book:<BID>（幂等）
#   scripts/fp_provision_policy.sh revoke-book BID # 撤销 book:<BID>（幂等）
#
# 每 app 会话取自身凭据（NOTES_FP_CRED）的注入点属 auto-ui 运行时 owner
# 交付面（PLAN-067 §5 T-02 前置登记）；本脚本是 v0 服务侧供给与授权变更的
# OS 管理入口。绝不改动 policy 中非本脚本文面的既有条目。
set -euo pipefail

fail() { printf 'fp_provision_policy: %s\n' "$1" >&2; exit "${2:-1}"; }

CMD="ensure"
BOOK_ID=""
if [[ $# -ge 1 ]]; then
  case "$1" in
    add-book|revoke-book)
      [[ $# -eq 2 && -n "$2" ]] || fail "missing BOOK_ID for $1 (usage: $1 BOOK_ID)" 2
      CMD="$1"; BOOK_ID="$2" ;;
    *) fail "unknown command: $1 (ensure | add-book BOOK_ID | revoke-book BOOK_ID)" 2 ;;
  esac
fi

# 数据目录/策略路径（与 notes repository.at fp_policy_path/data_dir 同律）：
# NOTES_FP_POLICY 直接覆盖；NOTES_DATA_DIR 即数据目录本体（不再拼接）；否则
# 缺省派生 <base>/auto-notes/data。
if [[ -n "${NOTES_FP_POLICY:-}" ]]; then
  POLICY="$NOTES_FP_POLICY"
elif [[ -n "${NOTES_DATA_DIR:-}" ]]; then
  POLICY="$NOTES_DATA_DIR/fp-policy.json"
else
  BASE="${APPDATA:-}"
  if [[ -z "$BASE" ]]; then BASE="${XDG_DATA_HOME:-}"; fi
  if [[ -z "$BASE" ]]; then BASE="${HOME:-}/.local/share"; fi
  [[ -n "$BASE" ]] || fail "no data base dir resolved (APPDATA/XDG_DATA_HOME/HOME)"
  POLICY="$BASE/auto-notes/data/fp-policy.json"
fi
mkdir -p "$(dirname "$POLICY")"

python -I - "$POLICY" "$CMD" "$BOOK_ID" <<'PY'
import json, secrets, sys

policy_path, cmd, book_id = sys.argv[1], sys.argv[2], sys.argv[3]
principal = "fp-reader"
operations = ["capture", "read", "update", "receipt", "list_by_book"]
try:
    with open(policy_path, encoding="utf-8") as f:
        doc = json.load(f)
except FileNotFoundError:
    doc = {}
except Exception as e:
    print(f"policy parse failed: {e}", file=sys.stderr)
    sys.exit(4)
entries = doc.setdefault("entries", [])
entry = next((e for e in entries if e.get("cred") and e.get("principal") == principal), None)
entry_created = entry is None
if entry is None:
    entry = {"cred": secrets.token_hex(32), "principal": principal,
             "operations": list(operations), "book_scopes": []}
    entries.append(entry)
changed = False
if cmd == "add-book":
    scope = f"book:{book_id}"
    if scope not in entry["book_scopes"]:
        entry["book_scopes"].append(scope)
        changed = True
elif cmd == "revoke-book":
    scope = f"book:{book_id}"
    if scope in entry["book_scopes"]:
        entry["book_scopes"].remove(scope)
        changed = True
if entry_created or changed:
    with open(policy_path, "w", encoding="utf-8") as f:
        json.dump(doc, f, ensure_ascii=False, indent=2)
print(f"policy={policy_path}")
print(f"principal={principal}")
print(f"cred={entry['cred']}")
print(f"book_scopes={entry['book_scopes']}")
PY
