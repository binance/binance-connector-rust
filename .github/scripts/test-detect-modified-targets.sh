#!/usr/bin/env bash

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/../.." && pwd)"
DETECTOR="$SCRIPT_DIR/detect-modified-targets.sh"
RANGE_RESOLVER="$SCRIPT_DIR/resolve-diff-range.sh"
TEST_ROOT="$(mktemp -d "${TMPDIR:-/tmp}/binance-ci-detector-tests.XXXXXX")"

cleanup() {
  rm -rf "$TEST_ROOT"
}
trap cleanup EXIT

assert_eq() {
  local expected="$1"
  local actual="$2"
  local message="$3"

  if [[ "$actual" != "$expected" ]]; then
    printf 'FAIL: %s\nexpected: %s\nactual:   %s\n' \
      "$message" "$expected" "$actual" >&2
    exit 1
  fi
}

assert_contains() {
  local expected="$1"
  local actual="$2"
  local message="$3"

  if [[ "$actual" != *"$expected"* ]]; then
    printf 'FAIL: %s\nexpected to contain: %s\nactual: %s\n' \
      "$message" "$expected" "$actual" >&2
    exit 1
  fi
}

setup_repo() {
  local name="$1"
  TEST_REPO="$TEST_ROOT/$name"

  git init -q "$TEST_REPO"
  git -C "$TEST_REPO" config user.name "CI Detector Test"
  git -C "$TEST_REPO" config user.email "ci-detector@example.invalid"
  mkdir -p \
    "$TEST_REPO/src/common" \
    "$TEST_REPO/src/spot" \
    "$TEST_REPO/src/wallet"
  printf 'pub fn common() {}\n' >"$TEST_REPO/src/common/mod.rs"
  printf 'pub fn spot() {}\n' >"$TEST_REPO/src/spot/mod.rs"
  printf 'pub fn wallet() {}\n' >"$TEST_REPO/src/wallet/mod.rs"
  git -C "$TEST_REPO" add .
  git -C "$TEST_REPO" commit -qm "initial"
}

test_complete_pull_request_diff_includes_first_commit() {
  setup_repo "multi-commit"
  local base_sha
  base_sha="$(git -C "$TEST_REPO" rev-parse HEAD)"

  printf 'pub fn changed_in_first_commit() {}\n' \
    >>"$TEST_REPO/src/spot/mod.rs"
  git -C "$TEST_REPO" add .
  git -C "$TEST_REPO" commit -qm "change spot"

  printf '# prose only\n' >"$TEST_REPO/README.md"
  git -C "$TEST_REPO" add .
  git -C "$TEST_REPO" commit -qm "update prose"

  local actual
  actual="$(
    bash "$DETECTOR" \
      --repo "$TEST_REPO" \
      --base "$base_sha" \
      --head "$(git -C "$TEST_REPO" rev-parse HEAD)"
  )"

  assert_eq \
    '["spot"]' \
    "$actual" \
    "the complete pull-request diff includes changes from the first commit"
}

test_fixture_global_path_selects_all_targets() {
  setup_repo "fixture-global"
  local changed_paths="$TEST_REPO/changed-paths"
  printf 'tests/rest_api.rs\0' >"$changed_paths"

  local actual
  actual="$(
    bash "$DETECTOR" \
      --repo "$TEST_REPO" \
      --changed-paths-file0 "$changed_paths"
  )"

  assert_eq \
    '["common","spot","wallet"]' \
    "$actual" \
    "a global test path selects every dynamically discovered target"
}

test_fixture_target_example_selects_owned_target() {
  setup_repo "fixture-example"
  local changed_paths="$TEST_REPO/changed-paths"
  printf 'examples/wallet/account.rs\0' >"$changed_paths"

  local actual
  actual="$(
    bash "$DETECTOR" \
      --repo "$TEST_REPO" \
      --changed-paths-file0 "$changed_paths"
  )"

  assert_eq \
    '["wallet"]' \
    "$actual" \
    "a target-owned example selects only its target"
}

test_fixture_validation_neutral_paths_skip_rust_matrix() {
  setup_repo "fixture-neutral"
  local changed_paths="$TEST_REPO/changed-paths"
  printf 'LICENSE\0CHANGELOG\0.gitignore\0docs/guide.md\0' >"$changed_paths"

  local actual
  actual="$(
    bash "$DETECTOR" \
      --repo "$TEST_REPO" \
      --changed-paths-file0 "$changed_paths"
  )"

  assert_eq \
    '[]' \
    "$actual" \
    "explicitly validation-neutral paths skip the Rust matrix"
}

test_empty_changed_path_input_fails_closed() {
  setup_repo "fixture-empty"
  local changed_paths="$TEST_REPO/changed-paths"
  : >"$changed_paths"
  local stderr_file="$TEST_REPO/stderr"

  if bash "$DETECTOR" \
    --repo "$TEST_REPO" \
    --changed-paths-file0 "$changed_paths" \
    >"$TEST_REPO/stdout" \
    2>"$stderr_file"; then
    printf 'FAIL: an empty changed-path input must fail closed\n' >&2
    exit 1
  fi

  assert_contains \
    "contains no paths" \
    "$(cat "$stderr_file")" \
    "an empty changed-path input explains why detection failed"
}

test_detector_change_selects_all_current_targets_in_worktree() {
  local changed_paths="$TEST_ROOT/current-repo-paths"
  printf '.github/scripts/detect-modified-targets.sh\0' >"$changed_paths"

  local actual
  actual="$(
    bash "$DETECTOR" \
      --repo "$PROJECT_ROOT" \
      --changed-paths-file0 "$changed_paths"
  )"

  assert_eq \
    '["algo","alpha","c2c","common","convert","copy_trading","crypto_loan","derivatives_trading_coin_futures","derivatives_trading_options","derivatives_trading_portfolio_margin","derivatives_trading_portfolio_margin_pro","derivatives_trading_usds_futures","dual_investment","fiat","gift_card","margin_trading","mining","pay","rebate","simple_earn","spot","staking","sub_account","vip_loan","w3w_prediction","wallet"]' \
    "$actual" \
    "a detector change selects all 26 current targets from a Git worktree"
}

test_advanced_base_excludes_base_only_changes() {
  setup_repo "advanced-base"
  local base_branch
  base_branch="$(git -C "$TEST_REPO" symbolic-ref --short HEAD)"
  local branch_point
  branch_point="$(git -C "$TEST_REPO" rev-parse HEAD)"

  git -C "$TEST_REPO" checkout -qb feature
  printf 'pub fn feature_change() {}\n' >>"$TEST_REPO/src/spot/mod.rs"
  git -C "$TEST_REPO" add .
  git -C "$TEST_REPO" commit -qm "change feature"
  local head_sha
  head_sha="$(git -C "$TEST_REPO" rev-parse HEAD)"

  git -C "$TEST_REPO" checkout -q "$base_branch"
  printf 'pub fn base_only_change() {}\n' >>"$TEST_REPO/src/wallet/mod.rs"
  git -C "$TEST_REPO" add .
  git -C "$TEST_REPO" commit -qm "advance base"
  local advanced_base
  advanced_base="$(git -C "$TEST_REPO" rev-parse HEAD)"
  git -C "$TEST_REPO" checkout -q feature

  local actual
  actual="$(
    bash "$DETECTOR" \
      --repo "$TEST_REPO" \
      --base "$advanced_base" \
      --head "$head_sha"
  )"

  assert_eq \
    '["spot"]' \
    "$actual" \
    "base-only changes after the branch point are excluded"
  assert_eq \
    "$branch_point" \
    "$(git -C "$TEST_REPO" merge-base "$advanced_base" "$head_sha")" \
    "the fixture has the intended merge base"
}

test_missing_commit_fails_closed() {
  setup_repo "missing-commit"
  local stderr_file="$TEST_REPO/stderr"

  if bash "$DETECTOR" \
    --repo "$TEST_REPO" \
    --base "$(git -C "$TEST_REPO" rev-parse HEAD)" \
    --head "0000000000000000000000000000000000000000" \
    >"$TEST_REPO/stdout" \
    2>"$stderr_file"; then
    printf 'FAIL: an unavailable commit must fail closed\n' >&2
    exit 1
  fi

  assert_contains \
    "head commit is unavailable" \
    "$(cat "$stderr_file")" \
    "an unavailable commit has an actionable diagnostic"
}

test_unrelated_histories_fail_closed() {
  setup_repo "no-merge-base"
  local base_sha
  base_sha="$(git -C "$TEST_REPO" rev-parse HEAD)"

  git -C "$TEST_REPO" checkout --orphan unrelated -q
  git -C "$TEST_REPO" rm -q -r --cached .
  git -C "$TEST_REPO" add .
  git -C "$TEST_REPO" commit -qm "unrelated root"
  local head_sha
  head_sha="$(git -C "$TEST_REPO" rev-parse HEAD)"
  local stderr_file="$TEST_REPO/stderr"

  if bash "$DETECTOR" \
    --repo "$TEST_REPO" \
    --base "$base_sha" \
    --head "$head_sha" \
    >"$TEST_REPO/stdout" \
    2>"$stderr_file"; then
    printf 'FAIL: histories without a merge base must fail closed\n' >&2
    exit 1
  fi

  assert_contains \
    "do not have a merge base" \
    "$(cat "$stderr_file")" \
    "unrelated histories have an actionable diagnostic"
}

test_mixed_paths_are_sorted_and_deduplicated() {
  setup_repo "fixture-mixed"
  local changed_paths="$TEST_REPO/changed-paths"
  printf 'src/wallet/one.rs\0src/spot/two.rs\0src/wallet/three.rs\0src/common/four.rs\0' \
    >"$changed_paths"

  local actual
  actual="$(
    bash "$DETECTOR" \
      --repo "$TEST_REPO" \
      --changed-paths-file0 "$changed_paths"
  )"

  assert_eq \
    '["common","spot","wallet"]' \
    "$actual" \
    "mixed target paths produce stable sorted deduplicated JSON"
}

test_common_source_path_selects_only_common() {
  setup_repo "fixture-common"
  local changed_paths="$TEST_REPO/changed-paths"
  printf 'src/common/http.rs\0' >"$changed_paths"

  local actual
  actual="$(
    bash "$DETECTOR" \
      --repo "$TEST_REPO" \
      --changed-paths-file0 "$changed_paths"
  )"

  assert_eq \
    '["common"]' \
    "$actual" \
    "a common source path selects only the common target"
}

test_rename_classifies_old_source_path() {
  setup_repo "rename-to-prose"
  printf 'pub fn keep_target_directory() {}\n' >"$TEST_REPO/src/spot/keep.rs"
  git -C "$TEST_REPO" add .
  git -C "$TEST_REPO" commit -qm "keep spot target"
  local base_sha
  base_sha="$(git -C "$TEST_REPO" rev-parse HEAD)"

  mkdir -p "$TEST_REPO/docs"
  git -C "$TEST_REPO" mv src/spot/mod.rs docs/spot.md
  git -C "$TEST_REPO" commit -qm "move source to prose"

  local actual
  actual="$(
    bash "$DETECTOR" \
      --repo "$TEST_REPO" \
      --base "$base_sha" \
      --head "$(git -C "$TEST_REPO" rev-parse HEAD)"
  )"

  assert_eq \
    '["spot"]' \
    "$actual" \
    "moving source to prose still classifies the deleted source path"
}

test_special_character_paths_preserve_boundaries() {
  setup_repo "fixture-special-paths"
  local changed_paths="$TEST_REPO/changed-paths"
  printf 'src/spot/file with spaces.rs\0src/wallet/line\nbreak.rs\0' \
    >"$changed_paths"

  local actual
  actual="$(
    bash "$DETECTOR" \
      --repo "$TEST_REPO" \
      --changed-paths-file0 "$changed_paths"
  )"

  assert_eq \
    '["spot","wallet"]' \
    "$actual" \
    "NUL-delimited paths preserve spaces and newlines"
}

test_new_target_is_discovered_from_explicit_head() {
  setup_repo "git-new-target"
  local base_branch
  base_branch="$(git -C "$TEST_REPO" symbolic-ref --short HEAD)"
  local base_sha
  base_sha="$(git -C "$TEST_REPO" rev-parse HEAD)"

  git -C "$TEST_REPO" checkout -qb feature
  mkdir -p "$TEST_REPO/src/new_target"
  printf 'pub fn new_target() {}\n' >"$TEST_REPO/src/new_target/mod.rs"
  printf '[package]\nname = "fixture"\n' >"$TEST_REPO/Cargo.toml"
  git -C "$TEST_REPO" add .
  git -C "$TEST_REPO" commit -qm "add target"
  local head_sha
  head_sha="$(git -C "$TEST_REPO" rev-parse HEAD)"

  git -C "$TEST_REPO" checkout -q "$base_branch"

  local actual
  actual="$(
    bash "$DETECTOR" \
      --repo "$TEST_REPO" \
      --base "$base_sha" \
      --head "$head_sha"
  )"

  assert_eq \
    '["common","new_target","spot","wallet"]' \
    "$actual" \
    "target discovery uses the explicit head rather than the current checkout"
}

test_missing_common_target_fails_closed() {
  setup_repo "fixture-missing-common"
  rm -rf "$TEST_REPO/src/common"
  local changed_paths="$TEST_REPO/changed-paths"
  printf 'Cargo.toml\0' >"$changed_paths"
  local stderr_file="$TEST_REPO/stderr"

  if bash "$DETECTOR" \
    --repo "$TEST_REPO" \
    --changed-paths-file0 "$changed_paths" \
    >"$TEST_REPO/stdout" \
    2>"$stderr_file"; then
    printf 'FAIL: a target set without common must fail closed\n' >&2
    exit 1
  fi

  assert_contains \
    "do not include common" \
    "$(cat "$stderr_file")" \
    "an invalid target set has an actionable diagnostic"
}

test_every_global_or_unknown_path_selects_all_targets() {
  setup_repo "fixture-global-table"
  local changed_paths="$TEST_REPO/changed-paths"
  local path

  for path in \
    "Cargo.toml" \
    "Cargo.lock" \
    "build.rs" \
    "src/lib.rs" \
    "rustfmt.toml" \
    "rust-toolchain" \
    "rust-toolchain.toml" \
    ".cargo/config.toml" \
    ".github/workflows/ci.yaml" \
    ".github/scripts/detect-modified-targets.sh" \
    ".github/scripts/README.md" \
    "tests/rest_api.rs" \
    "tests/fixture.md" \
    "unrecognized/config.json"; do
    printf '%s\0' "$path" >"$changed_paths"

    local actual
    actual="$(
      bash "$DETECTOR" \
        --repo "$TEST_REPO" \
        --changed-paths-file0 "$changed_paths"
    )"

    assert_eq \
      '["common","spot","wallet"]' \
      "$actual" \
      "global or unknown path '$path' selects every target"
  done
}

test_github_script_path_explicitly_selects_all_targets() {
  setup_repo "fixture-github-script"
  local changed_paths="$TEST_REPO/changed-paths"
  printf '.github/scripts/new-detector-helper.sh\0' >"$changed_paths"

  local actual
  actual="$(
    bash "$DETECTOR" \
      --repo "$TEST_REPO" \
      --changed-paths-file0 "$changed_paths"
  )"

  assert_eq \
    '["common","spot","wallet"]' \
    "$actual" \
    "a GitHub detector script explicitly selects every target"
}

test_deleted_source_path_selects_owning_target() {
  setup_repo "delete-source"
  printf 'pub fn delete_me() {}\n' >"$TEST_REPO/src/spot/delete_me.rs"
  git -C "$TEST_REPO" add .
  git -C "$TEST_REPO" commit -qm "add deletable source"
  local base_sha
  base_sha="$(git -C "$TEST_REPO" rev-parse HEAD)"

  git -C "$TEST_REPO" rm -q src/spot/delete_me.rs
  git -C "$TEST_REPO" commit -qm "delete source"

  local actual
  actual="$(
    bash "$DETECTOR" \
      --repo "$TEST_REPO" \
      --base "$base_sha" \
      --head "$(git -C "$TEST_REPO" rev-parse HEAD)"
  )"

  assert_eq \
    '["spot"]' \
    "$actual" \
    "deleting source still selects its owning target"
}

test_deleted_target_selects_every_remaining_target() {
  setup_repo "delete-target"
  local base_sha
  base_sha="$(git -C "$TEST_REPO" rev-parse HEAD)"

  git -C "$TEST_REPO" rm -qr src/spot
  git -C "$TEST_REPO" commit -qm "delete spot target"

  local actual
  actual="$(
    bash "$DETECTOR" \
      --repo "$TEST_REPO" \
      --base "$base_sha" \
      --head "$(git -C "$TEST_REPO" rev-parse HEAD)"
  )"

  assert_eq \
    '["common","wallet"]' \
    "$actual" \
    "deleting a complete target validates every target that remains runnable"
}

test_cross_target_rename_selects_both_targets() {
  setup_repo "rename-cross-target"
  printf 'pub fn keep_spot_target() {}\n' >"$TEST_REPO/src/spot/keep.rs"
  git -C "$TEST_REPO" add .
  git -C "$TEST_REPO" commit -qm "keep spot target"
  local base_sha
  base_sha="$(git -C "$TEST_REPO" rev-parse HEAD)"

  git -C "$TEST_REPO" mv src/spot/mod.rs src/wallet/moved-from-spot.rs
  git -C "$TEST_REPO" commit -qm "move source across targets"

  local actual
  actual="$(
    bash "$DETECTOR" \
      --repo "$TEST_REPO" \
      --base "$base_sha" \
      --head "$(git -C "$TEST_REPO" rev-parse HEAD)"
  )"

  assert_eq \
    '["spot","wallet"]' \
    "$actual" \
    "a cross-target move selects both the source and destination targets"
}

test_manual_dispatch_commit_refs_select_latest_commit() {
  setup_repo "manual-dispatch"
  printf 'pub fn latest_commit() {}\n' >>"$TEST_REPO/src/wallet/mod.rs"
  git -C "$TEST_REPO" add .
  git -C "$TEST_REPO" commit -qm "change wallet"

  local actual
  actual="$(
    bash "$DETECTOR" \
      --repo "$TEST_REPO" \
      --base "HEAD^" \
      --head "HEAD"
  )"

  assert_eq \
    '["wallet"]' \
    "$actual" \
    "manual-dispatch commit refs select the latest commit"
}

test_pull_request_range_resolver_accepts_valid_event_shas() {
  setup_repo "pull-request-range"
  local base_sha
  base_sha="$(git -C "$TEST_REPO" rev-parse HEAD)"
  printf 'pub fn changed() {}\n' >>"$TEST_REPO/src/spot/mod.rs"
  git -C "$TEST_REPO" add .
  git -C "$TEST_REPO" commit -qm "change spot"
  local head_sha
  head_sha="$(git -C "$TEST_REPO" rev-parse HEAD)"

  local actual
  actual="$(
    bash "$RANGE_RESOLVER" \
      --repo "$TEST_REPO" \
      --event-name pull_request \
      --pull-request-base-sha "$base_sha" \
      --pull-request-head-sha "$head_sha"
  )"

  assert_eq \
    "$base_sha $head_sha" \
    "$actual" \
    "pull-request events use their explicit base and head SHAs"
}

test_pull_request_range_resolver_rejects_malformed_sha() {
  setup_repo "pull-request-malformed"
  local stderr_file="$TEST_REPO/stderr"

  if bash "$RANGE_RESOLVER" \
    --repo "$TEST_REPO" \
    --event-name pull_request \
    --pull-request-base-sha invalid \
    --pull-request-head-sha "$(git -C "$TEST_REPO" rev-parse HEAD)" \
    >"$TEST_REPO/stdout" \
    2>"$stderr_file"; then
    printf 'FAIL: malformed pull-request SHAs must fail closed\n' >&2
    exit 1
  fi

  assert_contains \
    "invalid pull-request base SHA" \
    "$(cat "$stderr_file")" \
    "a malformed pull-request SHA has an actionable diagnostic"
}

test_workflow_dispatch_range_resolver_uses_head_parent() {
  setup_repo "workflow-dispatch-range"
  local base_sha
  base_sha="$(git -C "$TEST_REPO" rev-parse HEAD)"
  printf 'pub fn changed() {}\n' >>"$TEST_REPO/src/wallet/mod.rs"
  git -C "$TEST_REPO" add .
  git -C "$TEST_REPO" commit -qm "change wallet"
  local head_sha
  head_sha="$(git -C "$TEST_REPO" rev-parse HEAD)"

  local actual
  actual="$(
    bash "$RANGE_RESOLVER" \
      --repo "$TEST_REPO" \
      --event-name workflow_dispatch
  )"

  assert_eq \
    "$base_sha $head_sha" \
    "$actual" \
    "workflow dispatch compares HEAD with its first parent"
}

test_workflow_dispatch_range_resolver_requires_parent() {
  setup_repo "workflow-dispatch-root"
  local stderr_file="$TEST_REPO/stderr"

  if bash "$RANGE_RESOLVER" \
    --repo "$TEST_REPO" \
    --event-name workflow_dispatch \
    >"$TEST_REPO/stdout" \
    2>"$stderr_file"; then
    printf 'FAIL: workflow dispatch at a root commit must fail closed\n' >&2
    exit 1
  fi

  assert_contains \
    "requires a parent commit" \
    "$(cat "$stderr_file")" \
    "a root-only workflow dispatch has an actionable diagnostic"
}

test_complete_pull_request_diff_includes_first_commit
test_fixture_global_path_selects_all_targets
test_fixture_target_example_selects_owned_target
test_fixture_validation_neutral_paths_skip_rust_matrix
test_empty_changed_path_input_fails_closed
test_detector_change_selects_all_current_targets_in_worktree
test_advanced_base_excludes_base_only_changes
test_missing_commit_fails_closed
test_unrelated_histories_fail_closed
test_mixed_paths_are_sorted_and_deduplicated
test_common_source_path_selects_only_common
test_rename_classifies_old_source_path
test_special_character_paths_preserve_boundaries
test_new_target_is_discovered_from_explicit_head
test_missing_common_target_fails_closed
test_every_global_or_unknown_path_selects_all_targets
test_github_script_path_explicitly_selects_all_targets
test_deleted_source_path_selects_owning_target
test_deleted_target_selects_every_remaining_target
test_cross_target_rename_selects_both_targets
test_manual_dispatch_commit_refs_select_latest_commit
test_pull_request_range_resolver_accepts_valid_event_shas
test_pull_request_range_resolver_rejects_malformed_sha
test_workflow_dispatch_range_resolver_uses_head_parent
test_workflow_dispatch_range_resolver_requires_parent
printf 'PASS: 25 CI detection tests\n'
