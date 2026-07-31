#!/usr/bin/env bash

set -euo pipefail

usage() {
  printf '%s\n' \
    "usage: $(basename "$0") --repo <path> --base <commit> --head <commit>" \
    "       $(basename "$0") --repo <path> --changed-paths-file0 <file>" >&2
}

fail() {
  printf 'detect-modified-targets: %s\n' "$1" >&2
  exit 1
}

json_array() {
  if [[ "$#" -eq 0 ]]; then
    printf '[]\n'
    return
  fi

  printf '%s\n' "$@" \
    | LC_ALL=C sort -u \
    | jq -R -s -c 'split("\n") | map(select(length > 0))'
}

repo=""
base=""
head=""
changed_paths_file0=""

while [[ "$#" -gt 0 ]]; do
  case "$1" in
    --repo)
      [[ "$#" -ge 2 ]] || fail "--repo requires a value"
      repo="$2"
      shift 2
      ;;
    --base)
      [[ "$#" -ge 2 ]] || fail "--base requires a value"
      base="$2"
      shift 2
      ;;
    --head)
      [[ "$#" -ge 2 ]] || fail "--head requires a value"
      head="$2"
      shift 2
      ;;
    --changed-paths-file0)
      [[ "$#" -ge 2 ]] || fail "--changed-paths-file0 requires a value"
      changed_paths_file0="$2"
      shift 2
      ;;
    --help)
      usage
      exit 0
      ;;
    *)
      usage
      fail "unknown argument: $1"
      ;;
  esac
done

[[ -n "$repo" ]] || fail "--repo is required"
[[ -d "$repo" ]] || fail "repository is unavailable: $repo"
git -C "$repo" rev-parse --git-dir >/dev/null 2>&1 \
  || fail "repository is unavailable: $repo"
command -v jq >/dev/null 2>&1 || fail "jq is required"

targets=()
changed_paths_file=""
owns_changed_paths_file=false
targets_file=""
owns_targets_file=false
cleanup() {
  if [[ "$owns_changed_paths_file" == true ]]; then
    rm -f "$changed_paths_file"
  fi
  if [[ "$owns_targets_file" == true ]]; then
    rm -f "$targets_file"
  fi
}
trap cleanup EXIT

if [[ -n "$changed_paths_file0" ]]; then
  [[ -z "$base" && -z "$head" ]] \
    || fail "--changed-paths-file0 cannot be combined with --base or --head"
  [[ -f "$changed_paths_file0" ]] \
    || fail "changed-path fixture is unavailable: $changed_paths_file0"
  changed_paths_file="$changed_paths_file0"

  shopt -s nullglob
  for target_dir in "$repo"/src/*/; do
    targets+=("$(basename "$target_dir")")
  done
  shopt -u nullglob
else
  [[ -n "$base" ]] || fail "--base is required"
  [[ -n "$head" ]] || fail "--head is required"

  git -C "$repo" cat-file -e "${base}^{commit}" 2>/dev/null \
    || fail "base commit is unavailable: $base"
  git -C "$repo" cat-file -e "${head}^{commit}" 2>/dev/null \
    || fail "head commit is unavailable: $head"

  merge_base="$(git -C "$repo" merge-base "$base" "$head")" \
    || fail "base and head do not have a merge base"

  targets_file="$(mktemp "${TMPDIR:-/tmp}/binance-ci-targets.XXXXXX")"
  owns_targets_file=true
  git -C "$repo" ls-tree -d -z --name-only "${head}:src" \
    >"$targets_file" \
    || fail "unable to discover target directories from head: $head"
  while IFS= read -r -d '' target; do
    targets+=("$target")
  done <"$targets_file"

  changed_paths_file="$(mktemp "${TMPDIR:-/tmp}/binance-ci-paths.XXXXXX")"
  owns_changed_paths_file=true
  git -C "$repo" diff --name-only --no-renames -z "$merge_base" "$head" \
    >"$changed_paths_file" \
    || fail "unable to calculate the complete pull-request diff"
fi

[[ "${#targets[@]}" -gt 0 ]] || fail "no target directories found"

has_common=false
for target in "${targets[@]}"; do
  if [[ "$target" == "common" ]]; then
    has_common=true
    break
  fi
done
[[ "$has_common" == true ]] || fail "target directories do not include common"

selected=()
selected_count=0
select_all=false
path_count=0

is_target() {
  local candidate="$1"
  local current

  for current in "${targets[@]}"; do
    if [[ "$candidate" == "$current" ]]; then
      return 0
    fi
  done
  return 1
}

while IFS= read -r -d '' path; do
  path_count=$((path_count + 1))
  case "$path" in
    Cargo.toml | Cargo.lock | build.rs | src/lib.rs | rustfmt.toml | \
      rust-toolchain | rust-toolchain.toml | .cargo/* | \
      .github/workflows/* | .github/scripts/* | tests/*)
      select_all=true
      ;;
    *.md | LICENSE | LICENSE.* | LICENCE | LICENCE.* | CHANGELOG | \
      CHANGELOG.* | CHANGELOG-* | MIGRATION | MIGRATION.* | .gitignore)
      ;;
    src/*/* | examples/*/*)
      target_name="${path#*/}"
      target_name="${target_name%%/*}"
      if is_target "$target_name"; then
        selected+=("$target_name")
        selected_count=$((selected_count + 1))
      else
        select_all=true
      fi
      ;;
    *)
      select_all=true
      ;;
  esac
done <"$changed_paths_file"

[[ "$path_count" -gt 0 ]] || fail "changed-path input contains no paths"

if [[ "$select_all" == true ]]; then
  json_array "${targets[@]}"
elif [[ "$selected_count" -eq 0 ]]; then
  json_array
else
  json_array "${selected[@]}"
fi
