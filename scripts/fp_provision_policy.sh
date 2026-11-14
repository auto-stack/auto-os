#!/usr/bin/env bash
# PLAN-067 T-02 A-02：FeaturePack 受管 policy 供给与书籍范围授予——
# **私有凭据面**。契约（auto-ui SD-01 §5 + auto-notes repository.at
# fp_policy_path）：凭据不出现在公开 manifest、诊断、日志、回执中。
#
# 本脚本默认输出**不含凭据**：只回显 policy 路径、principal、book_scopes 与
# cred 指纹（SHA256 前 12 hex，用于跨会话/跨宿主关联核对，不可逆推凭据）。
# 显式 `--reveal-cred` 才完整回显（私有传递窗口使用，stdout 原样不得归档）。
#
# policy 位置与 notes 服务解析同律：NOTES_FP_POLICY 覆盖 > NOTES_DATA_DIR
# （即数据目录本体）> <APPDATA>/auto-notes/data/fp-policy.json。Windows
# 缺省落点在用户级 %APPDATA%（按用户隔离，非系统全局可写）。
#
# 用法：
#   scripts/fp_provision_policy.sh [--reveal-cred]
#                                  [add-book BOOK_ID | revoke-book BOOK_ID]
#
# 消费面：manifest daemon.host.credential（env/path/principal/field）由
# auto-ui 运行时在宿主后端子进程 env 注入——本脚本 stdout 不参与生产传递。
set -euo pipefail

fail() { printf 'fp_provision_policy: %s\n' "$1" >&2; exit "${2:-1}"; }

REVEAL=0
CMD="ensure"
BOOK_ID=""
while [[ $# -gt 0 ]]; do
  case "$1" in
    --reveal-cred) REVEAL=1; shift ;;
    add-book|revoke-book)
      [[ $# -ge 2 && -n "$2" ]] || fail "missing BOOK_ID for $1 (usage: $1 BOOK_ID)" 2
      CMD="$1"; BOOK_ID="$2"; shift 2 ;;
    *) fail "unknown argument: $1 (--reveal-cred | add-book BOOK_ID | revoke-book BOOK_ID)" 2 ;;
  esac
done

# 数据目录/策略路径（与 notes repository.at fp_policy_path/data_dir 同律）。
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

python -I - "$POLICY" "$CMD" "$BOOK_ID" "$REVEAL" <<'PY'
import hashlib, json, secrets, sys

policy_path, cmd, book_id, reveal = sys.argv[1], sys.argv[2], sys.argv[3], sys.argv[4] == "1"
principal = "fp-reader"
operations = ["capture", "read", "update", "receipt", "list_by_book"]
had_bom = False
try:
    # utf-8-sig：容忍历史 BOM 写入（ps1 旧版缺陷），读出即归一；重写落盘
    # 恒为无 BOM UTF-8（服务侧 fs_read 与运行时 serde_json 均不容忍 BOM）。
    raw = open(policy_path, "rb").read()
    had_bom = raw.startswith(b"\xef\xbb\xbf")
    doc = json.loads(raw.decode("utf-8-sig"))
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
if entry_created or changed or had_bom:
    with open(policy_path, "w", encoding="utf-8") as f:
        json.dump(doc, f, ensure_ascii=False, indent=2)

# 私有凭据面（A-02）：默认只出指纹；--reveal-cred 显式完整回显。
fp = hashlib.sha256(entry["cred"].encode()).hexdigest()[:12]
print(f"policy={policy_path}")
print(f"principal={principal}")
print(f"cred_fingerprint=sha256:{fp} (first 12 hex; credential itself is NOT printed)")
print(f"book_scopes={entry['book_scopes']}")
if reveal:
    print(f"cred={entry['cred']}  # private delivery window only; do not archive stdout")
PY
