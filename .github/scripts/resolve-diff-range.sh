#!/usr/bin/env bash

set -euo pipefail

usage() {
  printf '%s\n' \
    "usage: $(basename "$0") --repo <path> --event-name <name>" \
    "       [--pull-request-base-sha <sha> --pull-request-head-sha <sha>]" >&2
}

fail() {
  printf 'resolve-diff-range: %s\n' "$1" >&2
  exit 1
}

repo=""
event_name=""
pull_request_base_sha=""
pull_request_head_sha=""

while [[ "$#" -gt 0 ]]; do
  case "$1" in
    --repo)
      [[ "$#" -ge 2 ]] || fail "--repo requires a value"
      repo="$2"
      shift 2
      ;;
    --event-name)
      [[ "$#" -ge 2 ]] || fail "--event-name requires a value"
      event_name="$2"
      shift 2
      ;;
    --pull-request-base-sha)
      [[ "$#" -ge 2 ]] || fail "--pull-request-base-sha requires a value"
      pull_request_base_sha="$2"
      shift 2
      ;;
    --pull-request-head-sha)
      [[ "$#" -ge 2 ]] || fail "--pull-request-head-sha requires a value"
      pull_request_head_sha="$2"
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
[[ -n "$event_name" ]] || fail "--event-name is required"

case "$event_name" in
  pull_request)
    [[ "$pull_request_base_sha" =~ ^[0-9a-fA-F]{40}$ ]] \
      || fail "invalid pull-request base SHA"
    [[ "$pull_request_head_sha" =~ ^[0-9a-fA-F]{40}$ ]] \
      || fail "invalid pull-request head SHA"
    printf '%s %s\n' "$pull_request_base_sha" "$pull_request_head_sha"
    ;;
  workflow_dispatch)
    git -C "$repo" rev-parse --verify "HEAD^" >/dev/null 2>&1 \
      || fail "workflow_dispatch requires a parent commit for HEAD"
    printf '%s %s\n' \
      "$(git -C "$repo" rev-parse "HEAD^")" \
      "$(git -C "$repo" rev-parse "HEAD")"
    ;;
  *)
    fail "unsupported event: $event_name"
    ;;
esac
