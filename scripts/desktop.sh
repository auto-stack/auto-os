#!/usr/bin/env bash
# Source group: lang/ui/man -> one target. Candidate mode: explicit binaries only.
desktop_fail() { printf '%s\n' "desktop: $1" >&2; exit "${2:-1}"; }
desktop_win_path() {
  if command -v cygpath >/dev/null 2>&1; then cygpath -am "$1";
  elif [[ "$1" = /* ]]; then printf '%s\n' "$1";
  else printf '%s/%s\n' "$PWD" "$1"; fi
}
desktop_source_root() {
  local explicit="$1" env_name="$2" repo="$3" parent="$4" probe="$5" override choice
  override="${explicit:-${!env_name:-}}"
  if [[ -n "$override" ]]; then
    [[ -f "$override/$probe" ]] || desktop_fail "$repo root invalid (parameter/$env_name): $override"
    (cd "$override" && pwd); return
  fi
  for choice in "$parent/$repo" "D:/autostack/$repo"; do
    if [[ -f "$choice/$probe" ]]; then (cd "$choice" && pwd); return; fi
  done
  desktop_fail "$repo root not found (OS sibling -> main checkout)"
}
desktop_cargo() {
  local root="$1" target="$2"; shift 2
  (cd "$root" && export CARGO_TARGET_DIR="$(desktop_win_path "$target")" && cargo "$@")
}
desktop_program() { "$@"; }

desktop_main() (
  set -euo pipefail
  local track=vue fullscreen=0 dry_run=0 lang_root='' ui_root='' man_root='' build_root='' candidate_dir=''
  while [[ $# -gt 0 ]]; do
    case "$1" in
      vue|iced) track="$1"; shift ;;
      --fullscreen) fullscreen=1; shift ;;
      --dry-run) dry_run=1; shift ;;
      --lang-root|--ui-root|--man-root|--build-root|--candidate-dir)
        [[ $# -ge 2 && -n "$2" ]] || desktop_fail "missing value for $1" 2
        case "$1" in
          --lang-root) lang_root="$2";; --ui-root) ui_root="$2";; --man-root) man_root="$2";;
          --build-root) build_root="$2";; --candidate-dir) candidate_dir="$2";;
        esac
        shift 2 ;;
      *) desktop_fail "unknown argument: $1 (vue|iced --fullscreen --dry-run --lang-root/--ui-root/--man-root/--build-root/--candidate-dir PATH)" 2 ;;
    esac
  done
  local script_dir os parent mode target='' bin desktop suffix=''
  script_dir="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
  os="${DESKTOP_OS_ROOT:-$script_dir/..}"
  os="$(cd "$os" && pwd)" || desktop_fail "invalid DESKTOP_OS_ROOT: $os"
  parent="$(dirname "$os")"
  [[ -z "$candidate_dir" || -z "$build_root" ]] || desktop_fail 'CandidateDir and BuildRoot are mutually exclusive'
  case "${OSTYPE:-}" in msys*|cygwin*|win32*) suffix='.exe';; esac
  mode=source
  if [[ -n "$candidate_dir" ]]; then
    mode=candidate; bin="$(desktop_win_path "$candidate_dir")"; desktop="$bin/ui-desktop$suffix"
    if [[ "$track" = vue ]]; then
      lang_root="$(desktop_source_root "$lang_root" AUTO_LANG_ROOT auto-lang "$parent" crates/auto-lang/Cargo.toml)" || exit $?
    fi
  else
    lang_root="$(desktop_source_root "$lang_root" AUTO_LANG_ROOT auto-lang "$parent" crates/auto-lang/Cargo.toml)" || exit $?
    ui_root="$(desktop_source_root "$ui_root" AUTO_UI_ROOT auto-ui "$parent" crates/auto-ui-tools/Cargo.toml)" || exit $?
    man_root="$(desktop_source_root "$man_root" AUTO_MAN_ROOT auto-man "$parent" crates/auto/Cargo.toml)" || exit $?
    [[ "$(basename "$lang_root")" = auto-lang && "$(basename "$ui_root")" = auto-ui && "$(basename "$man_root")" = auto-man ]] || desktop_fail 'source roots must keep auto-lang/auto-ui/auto-man sibling directory names'
    [[ "$(dirname "$lang_root")" = "$(dirname "$ui_root")" && "$(dirname "$lang_root")" = "$(dirname "$man_root")" ]] || desktop_fail 'source roots must be a sibling lang/ui/man group (Cargo path dependencies)'
    build_root="${build_root:-${AUTO_DESKTOP_BUILD_ROOT:-$os/.auto/desktop-runtime}}"
    target="$(desktop_win_path "$build_root")/target"; bin="$target/debug"; desktop="$bin/ui-desktop$suffix"
    export AUTO_LANG_ROOT="$(desktop_win_path "$lang_root")" AUTO_UI_ROOT="$(desktop_win_path "$ui_root")" AUTO_MAN_ROOT="$(desktop_win_path "$man_root")"
  fi
  if [[ "$track" = vue && ! -d "$lang_root/examples/desktop-host" ]]; then desktop_fail 'Vue desktop requires the language examples/desktop-host project'; fi
  local cli="$bin/auto$suffix" tool="$bin/auto-ui-tool$suffix" file
  export AUTO_UI_TOOL_BIN="$tool" AUTO_OS_ROOT="$(desktop_win_path "$os")"
  export AUTO_DESKTOP_RQHOST="${AUTO_DESKTOP_RQHOST:-1}"
  if [[ -z "${AUTO_MPV_LIB:-}" && -f "$parent/tools/mpv/libmpv-2.dll" ]]; then export AUTO_MPV_LIB="$(desktop_win_path "$parent/tools/mpv/libmpv-2.dll")"; fi
  # Keep the prior shell desktop storage location; never overwrite explicit isolation.
  export AUTO_VM_STORAGE_FILE="${AUTO_VM_STORAGE_FILE:-$(desktop_win_path "$HOME/.config/autoos/desktop-storage.json")}"
  if [[ "$track" = vue ]]; then unset AUTO_DESKTOP_APPS_EXTRA; export AUTO_DESKTOP_APPS="$(desktop_win_path "$os/apps")"; fi
  printf '[desktop] mode=%s track=%s os=%s\n' "$mode" "$track" "$os"
  printf '[desktop] lang=%s ui=%s man=%s\n' "$lang_root" "$ui_root" "$man_root"
  printf '[desktop] auto=%s tool=%s desktop=%s\n' "$cli" "$tool" "$desktop"
  desktop_build() {
    local root="$1"; shift
    printf '[desktop] cwd=%s target=%s cargo' "$root" "$target"; printf ' %q' "$@"; printf '\n'
    if [[ "$dry_run" != 1 ]]; then
      if desktop_cargo "$root" "$target" "$@"; then :;
      else local code=$?; desktop_fail "cargo failed in $root (exit $code); desktop will not launch" "$code"; fi
    fi
  }
  if [[ "$mode" = source ]]; then
    local -a cli_args=(build --locked -p auto --bin auto) tool_args=(build --locked -p auto-ui-tools --bin auto-ui-tool)
    if [[ "$track" = iced ]]; then cli_args+=(--features mpv-widget); tool_args+=(--features auto-ui-runtime/mpv-widget); fi
    desktop_build "$man_root" "${cli_args[@]}"
    desktop_build "$ui_root" "${tool_args[@]}"
    if [[ "$track" = iced ]]; then desktop_build "$os" build --locked -p os-desktop --bin ui-desktop; fi
  fi
  if [[ "$dry_run" != 1 ]]; then
    for file in "$cli" "$tool"; do [[ -f "$file" ]] || desktop_fail "required desktop artifact missing: $file"; done
    if [[ "$track" = iced ]]; then [[ -f "$desktop" ]] || desktop_fail "required desktop artifact missing: $desktop"; fi
  fi
  local cwd executable
  local -a arguments
  if [[ "$track" = iced ]]; then
    cwd="$os"; executable="$desktop"; arguments=(--apps-dir "$(desktop_win_path "$os/apps")")
    [[ "$fullscreen" = 0 ]] || arguments+=(--fullscreen)
  else cwd="$lang_root/examples/desktop-host"; executable="$cli"; arguments=(run --desktop); fi
  printf '[desktop] cwd=%s run=%s' "$cwd" "$executable"; printf ' %q' "${arguments[@]}"; printf '\n'
  [[ "$dry_run" != 1 ]] || exit 0
  cd "$cwd"
  desktop_program "$executable" "${arguments[@]}"
)

if [[ "${BASH_SOURCE[0]}" = "$0" ]]; then desktop_main "$@"; fi
