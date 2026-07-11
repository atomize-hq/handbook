#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
EVIDENCE_DIR="$ROOT_DIR/.implemented/m10-orchestration"
LOG_PATH="$EVIDENCE_DIR/codex-skill-live-smoke.log"
STATE_ROOT="${XDG_STATE_HOME:-$HOME/.local/state}/handbook/intake/runs"
HANDBOOK_HOME="$HOME/handbook"
HANDBOOK_BINARY="$HANDBOOK_HOME/bin/handbook"
RUNTIME_MANIFEST="$HANDBOOK_HOME/runtime-manifest.json"
CHARTER_FIXTURE_INPUTS="$ROOT_DIR/tools/fixtures/charter_inputs/runtime_smoke_valid.yaml"
PROJECT_CONTEXT_FIXTURE_INPUTS="$ROOT_DIR/tools/fixtures/project_context_inputs/runtime_smoke_valid.yaml"
ENVIRONMENT_INVENTORY_FIXTURE_INPUTS="$ROOT_DIR/tools/fixtures/environment_inventory_inputs/runtime_smoke_valid.yaml"
RELEASE_VERSION="$(tr -d '[:space:]' <"$ROOT_DIR/VERSION")"
export PATH="${CARGO_HOME:-$HOME/.cargo}/bin:$PATH"

mkdir -p "$EVIDENCE_DIR" "$STATE_ROOT"
exec > >(tee "$LOG_PATH") 2>&1

tmp_root=""
binary_backup=""
MANIFEST_VERSION_VALUE=""
LAST_RUN_DIR=""

cleanup() {
  if [[ -n "$binary_backup" && -f "$binary_backup" ]]; then
    mv "$binary_backup" "$HANDBOOK_BINARY"
  fi
  if [[ -n "$tmp_root" && -d "$tmp_root" ]]; then
    rm -rf "$tmp_root"
  fi
}
trap cleanup EXIT

count_run_dirs() {
  find "$STATE_ROOT" -mindepth 1 -maxdepth 1 -type d | wc -l | tr -d '[:space:]'
}

assert_path_absent() {
  local path="$1"

  [[ ! -e "$path" ]] || {
    echo "unexpected path present: $path"
    exit 1
  }
}

assert_session_fields() {
  local session_path="$1"
  local expected_runtime_root="$2"
  python3 - "$session_path" "$expected_runtime_root" <<'PY'
import json
import os
import sys

with open(sys.argv[1], encoding="utf-8") as handle:
    data = json.load(handle)
required = {
    "started_at_utc",
    "repo_root",
    "runtime_root",
    "handbook_release_version",
    "runtime_manifest_version",
    "skill_name",
}
missing = sorted(required.difference(data))
if missing:
    raise SystemExit(f"missing session fields: {missing}")
actual_runtime_root = os.path.realpath(data["runtime_root"])
expected_runtime_root = os.path.realpath(sys.argv[2])
if actual_runtime_root != expected_runtime_root:
    raise SystemExit(
        f"unexpected runtime_root: {data['runtime_root']} != {sys.argv[2]}"
    )
if data["skill_name"] != "handbook-charter-intake":
    raise SystemExit(f"unexpected skill_name: {data['skill_name']}")
PY
}

assert_run_dir_file_set() {
  local run_dir="$1"
  local expected="$2"
  local actual
  actual="$(
    find "$run_dir" -maxdepth 1 -type f -print \
      | sed "s#^$run_dir/##" \
      | sort
  )"
  [[ "$actual" == "$expected" ]] || {
    echo "unexpected run artifact file set under $run_dir"
    echo "actual:"
    printf '%s\n' "$actual"
    echo "expected:"
    printf '%s\n' "$expected"
    exit 1
  }
}

assert_baseline_complete() {
  local doctor_json_path="$1"

  python3 - "$doctor_json_path" <<'PY'
import json
import sys

with open(sys.argv[1], encoding="utf-8") as handle:
    data = json.load(handle)
if data.get("baseline_state") != "baseline_complete":
    raise SystemExit(f"expected baseline_complete, got: {data.get('baseline_state')}")
if data.get("status") != "baseline_complete":
    raise SystemExit(f"expected complete status, got: {data.get('status')}")
if data.get("blockers"):
    raise SystemExit(f"expected no blockers, got: {data['blockers']}")
PY
}

assert_no_misplaced_run_evidence() {
  local root="$1"
  local misplaced
  misplaced="$(
    find "$root" -type f \( \
      -name 'session.json' -o \
      -name 'doctor.before.json' -o \
      -name 'doctor.after_setup.json' -o \
      -name 'doctor.after_write.json' -o \
      -name 'author.stdout.txt' -o \
      -name 'author.stderr.txt' -o \
      -name 'validate.stdout.txt' -o \
      -name 'validate.stderr.txt' \
    \) -print
  )"
  [[ -z "$misplaced" ]] || {
    echo "unexpected run evidence under $root"
    printf '%s\n' "$misplaced"
    exit 1
  }
}

common_success_files() {
  cat <<'EOF'
author.exit
author.stderr.txt
author.stdout.txt
charter_inputs.yaml
doctor.after_setup.json
doctor.after_write.json
doctor.before.json
session.json
validate.exit
validate.stderr.txt
validate.stdout.txt
EOF
}

existing_truth_refusal_files() {
  cat <<'EOF'
author.exit
author.stderr.txt
author.stdout.txt
charter_inputs.yaml
doctor.before.json
session.json
validate.exit
validate.stderr.txt
validate.stdout.txt
EOF
}

read_manifest_metadata() {
  local manifest_path="$1"
  local expected_release="$2"

  python3 - "$manifest_path" "$expected_release" <<'PY'
import json
import sys

with open(sys.argv[1], encoding="utf-8") as handle:
    data = json.load(handle)

required = {
    "skill_name",
    "handbook_release_version",
    "manifest_version",
    "generated_at_utc",
}
missing = sorted(required.difference(data))
if missing:
    raise SystemExit(f"missing manifest fields: {missing}")
if data["skill_name"] != "handbook-charter-intake":
    raise SystemExit(f"unexpected manifest skill_name: {data['skill_name']}")
if data["handbook_release_version"] != sys.argv[2]:
    raise SystemExit(
        "runtime manifest version mismatch: "
        f"manifest={data['handbook_release_version']} repo={sys.argv[2]}"
    )
print(data["manifest_version"])
PY
}

capture_in_repo() {
  local repo_root="$1"
  local stdout_path="$2"
  local stderr_path="$3"
  local exit_path="$4"
  shift 4

  local status
  local restore_errexit=0
  if [[ $- == *e* ]]; then
    restore_errexit=1
    set +e
  fi
  (
    cd "$repo_root"
    "$@" >"$stdout_path" 2>"$stderr_path"
  )
  status=$?
  if [[ $restore_errexit -eq 1 ]]; then
    set -e
  fi
  printf '%s\n' "$status" >"$exit_path"
  printf '%s\n' "$status"
}

capture_doctor_json() {
  local repo_root="$1"
  local output_path="$2"
  local status

  local restore_errexit=0
  if [[ $- == *e* ]]; then
    restore_errexit=1
    set +e
  fi
  (
    cd "$repo_root"
    "$HANDBOOK_BINARY" doctor --json >"$output_path"
  )
  status=$?
  if [[ $restore_errexit -eq 1 ]]; then
    set -e
  fi
  printf '%s\n' "$status"
}

validate_runtime_contract() {
  local binary_version

  [[ -x "$HANDBOOK_BINARY" ]] || {
    echo "REFUSED: missing installed handbook binary: $HANDBOOK_BINARY" >&2
    return 1
  }
  [[ -f "$RUNTIME_MANIFEST" ]] || {
    echo "REFUSED: missing installed runtime manifest: $RUNTIME_MANIFEST" >&2
    return 1
  }
  for required in \
    "$HANDBOOK_HOME/resources/authoring/charter_authoring_method.md" \
    "$HANDBOOK_HOME/resources/charter/CHARTER_INPUTS.yaml.tmpl" \
    "$HANDBOOK_HOME/resources/charter/charter_inputs_directive.md" \
    "$HANDBOOK_HOME/resources/project_context/PROJECT_CONTEXT_INPUTS.yaml.tmpl" \
    "$HANDBOOK_HOME/resources/environment_inventory/ENVIRONMENT_INVENTORY_INPUTS.yaml.tmpl"; do
    [[ -f "$required" ]] || {
      echo "REFUSED: missing installed handbook home prerequisite: $required" >&2
      return 1
    }
  done

  MANIFEST_VERSION_VALUE="$(read_manifest_metadata "$RUNTIME_MANIFEST" "$RELEASE_VERSION")"
  binary_version="$("$HANDBOOK_BINARY" --version | awk '{print $NF}')"
  [[ "$binary_version" == "$RELEASE_VERSION" ]] || {
    echo "REFUSED: installed handbook binary version mismatch: binary=$binary_version repo=$RELEASE_VERSION" >&2
    return 1
  }
}

write_session_json() {
  local destination_path="$1"
  local repo_root="$2"
  local runtime_root="$3"
  local manifest_version="$4"

  python3 - "$destination_path" "$repo_root" "$runtime_root" "$RELEASE_VERSION" "$manifest_version" <<'PY'
import json
import pathlib
import sys
from datetime import datetime, timezone

destination = pathlib.Path(sys.argv[1])
payload = {
    "started_at_utc": datetime.now(timezone.utc).strftime("%Y-%m-%dT%H:%M:%SZ"),
    "repo_root": str(pathlib.Path(sys.argv[2]).resolve()),
    "runtime_root": str(pathlib.Path(sys.argv[3]).resolve()),
    "handbook_release_version": sys.argv[4],
    "runtime_manifest_version": sys.argv[5],
    "skill_name": "handbook-charter-intake",
}
destination.write_text(json.dumps(payload, indent=2, sort_keys=True) + "\n", encoding="utf-8")
PY
}

run_leaf_skill() {
  local repo_dir="$1"
  local repo_root
  local run_dir
  local doctor_before_status
  local author_status

  LAST_RUN_DIR=""
  repo_root="$(git -C "$repo_dir" rev-parse --show-toplevel 2>/dev/null)" || {
    echo "REFUSED: run this skill from inside a real git repository." >&2
    return 1
  }

  validate_runtime_contract || return 1
  export HANDBOOK_HOME

  run_dir="$(mktemp -d "$STATE_ROOT/run.XXXXXX")"
  LAST_RUN_DIR="$run_dir"
  cp "$CHARTER_FIXTURE_INPUTS" "$run_dir/charter_inputs.yaml"
  write_session_json "$run_dir/session.json" "$repo_root" "$HANDBOOK_HOME" "$MANIFEST_VERSION_VALUE"

  doctor_before_status="$(capture_doctor_json "$repo_root" "$run_dir/doctor.before.json")"
  if [[ ! -d "$repo_root/.handbook" ]]; then
    (
      cd "$repo_root"
      "$HANDBOOK_BINARY" setup >/dev/null
    )
    capture_doctor_json "$repo_root" "$run_dir/doctor.after_setup.json" >/dev/null
  fi

  capture_in_repo \
    "$repo_root" \
    "$run_dir/validate.stdout.txt" \
    "$run_dir/validate.stderr.txt" \
    "$run_dir/validate.exit" \
    "$HANDBOOK_BINARY" author charter --validate --from-inputs "$CHARTER_FIXTURE_INPUTS" >/dev/null

  author_status="$(
    capture_in_repo \
    "$repo_root" \
    "$run_dir/author.stdout.txt" \
    "$run_dir/author.stderr.txt" \
    "$run_dir/author.exit" \
    "$HANDBOOK_BINARY" author charter --from-inputs "$CHARTER_FIXTURE_INPUTS"
  )"

  if [[ $author_status -eq 0 ]]; then
    capture_doctor_json "$repo_root" "$run_dir/doctor.after_write.json" >/dev/null
  fi

  return "$author_status"
}

echo "==> install current handbook binary"
cd "$ROOT_DIR"
cargo install --locked --force --path crates/cli

echo "==> install generated codex skill assets"
bash tools/codex/install.sh
test -x "$HANDBOOK_BINARY"
test -f "$RUNTIME_MANIFEST"
assert_path_absent "$HANDBOOK_HOME/bin/handbook-charter-intake"
assert_path_absent "$HANDBOOK_HOME/share"

tmp_root="$(mktemp -d)"

echo "==> happy path smoke against installed runtime"
happy_repo="$tmp_root/happy-repo"
mkdir -p "$happy_repo"
git -C "$happy_repo" init -q
before_happy_count="$(count_run_dirs)"
run_leaf_skill "$happy_repo" >/dev/null
happy_run_dir="$LAST_RUN_DIR"
after_happy_count="$(count_run_dirs)"
[[ -n "$happy_run_dir" && -d "$happy_run_dir" && "$after_happy_count" -gt "$before_happy_count" ]] || {
  echo "failed to capture happy-path run dir"
  exit 1
}
assert_run_dir_file_set "$happy_run_dir" "$(common_success_files)"
assert_session_fields "$happy_run_dir/session.json" "$HANDBOOK_HOME"
test -f "$happy_repo/.handbook/charter/CHARTER.md"

echo "==> all-three deterministic baseline smoke without Codex or credentials"
all_three_repo="$tmp_root/all-three-repo"
offline_path="$tmp_root/no-codex-path"
all_three_doctor="$tmp_root/all-three-doctor.json"
mkdir -p "$all_three_repo" "$offline_path"
git -C "$all_three_repo" init -q
(
  cd "$all_three_repo"
  env -u CODEX_API_KEY -u OPENAI_API_KEY \
    PATH="$offline_path" \
    "$HANDBOOK_BINARY" setup >/dev/null
  env -u CODEX_API_KEY -u OPENAI_API_KEY PATH="$offline_path" \
    "$HANDBOOK_BINARY" author charter --validate --from-inputs "$CHARTER_FIXTURE_INPUTS" >/dev/null
  env -u CODEX_API_KEY -u OPENAI_API_KEY PATH="$offline_path" \
    "$HANDBOOK_BINARY" author charter --from-inputs "$CHARTER_FIXTURE_INPUTS" >/dev/null
  env -u CODEX_API_KEY -u OPENAI_API_KEY PATH="$offline_path" \
    "$HANDBOOK_BINARY" author project-context --validate --from-inputs - \
    <"$PROJECT_CONTEXT_FIXTURE_INPUTS" >/dev/null
  env -u CODEX_API_KEY -u OPENAI_API_KEY PATH="$offline_path" \
    "$HANDBOOK_BINARY" author project-context --from-inputs - \
    <"$PROJECT_CONTEXT_FIXTURE_INPUTS" >/dev/null
  env -u CODEX_API_KEY -u OPENAI_API_KEY PATH="$offline_path" \
    "$HANDBOOK_BINARY" author environment-inventory --validate --from-inputs "$ENVIRONMENT_INVENTORY_FIXTURE_INPUTS" >/dev/null
  env -u CODEX_API_KEY -u OPENAI_API_KEY PATH="$offline_path" \
    "$HANDBOOK_BINARY" author environment-inventory --from-inputs "$ENVIRONMENT_INVENTORY_FIXTURE_INPUTS" >/dev/null
  env -u CODEX_API_KEY -u OPENAI_API_KEY PATH="$offline_path" \
    "$HANDBOOK_BINARY" doctor --json >"$all_three_doctor"
)
test -f "$all_three_repo/.handbook/charter/CHARTER.md"
test -f "$all_three_repo/.handbook/project_context/PROJECT_CONTEXT.md"
test -f "$all_three_repo/.handbook/environment_inventory/ENVIRONMENT_INVENTORY.md"
assert_baseline_complete "$all_three_doctor"

echo "==> existing charter refusal smoke"
existing_repo="$tmp_root/existing-charter-repo"
mkdir -p "$existing_repo"
git -C "$existing_repo" init -q
(
  cd "$existing_repo"
  "$HANDBOOK_BINARY" setup >/dev/null
  "$HANDBOOK_BINARY" author charter --from-inputs "$CHARTER_FIXTURE_INPUTS" >/dev/null
)
before_existing_count="$(count_run_dirs)"
set +e
run_leaf_skill "$existing_repo"
existing_status=$?
set -e
if [[ $existing_status -eq 0 ]]; then
  echo "expected refusal when canonical charter already exists"
  exit 1
fi
existing_run_dir="$LAST_RUN_DIR"
after_existing_count="$(count_run_dirs)"
[[ -n "$existing_run_dir" && -d "$existing_run_dir" && "$after_existing_count" -gt "$before_existing_count" ]] || {
  echo "failed to capture existing-truth refusal run dir"
  exit 1
}
existing_refusal="$(cat "$existing_run_dir/author.stdout.txt")"
[[ "$existing_refusal" == *"canonical charter truth already exists"* ]] || {
  echo "expected existing charter refusal"
  printf '%s\n' "$existing_refusal"
  exit 1
}
assert_run_dir_file_set "$existing_run_dir" "$(existing_truth_refusal_files)"
assert_session_fields "$existing_run_dir/session.json" "$HANDBOOK_HOME"

echo "==> outside-git-repo refusal smoke"
outside_dir="$tmp_root/not-a-repo"
mkdir -p "$outside_dir"
outside_before_count="$(count_run_dirs)"
set +e
outside_output="$(run_leaf_skill "$outside_dir" 2>&1)"
outside_status=$?
set -e
if [[ $outside_status -eq 0 ]]; then
  echo "expected refusal outside a git repo"
  exit 1
fi
[[ "$outside_output" == *"run this skill from inside a real git repository"* ]] || {
  echo "expected outside-repo refusal"
  echo "$outside_output"
  exit 1
}
outside_after_count="$(count_run_dirs)"
[[ "$outside_before_count" == "$outside_after_count" ]] || {
  echo "outside-repo refusal unexpectedly created run evidence"
  exit 1
}

echo "==> missing installed binary refusal smoke"
missing_binary_repo="$tmp_root/missing-binary-repo"
mkdir -p "$missing_binary_repo"
git -C "$missing_binary_repo" init -q
binary_backup="$HANDBOOK_BINARY.bak"
mv "$HANDBOOK_BINARY" "$binary_backup"
missing_binary_before_count="$(count_run_dirs)"
set +e
missing_binary_output="$(run_leaf_skill "$missing_binary_repo" 2>&1)"
missing_binary_status=$?
set -e
mv "$binary_backup" "$HANDBOOK_BINARY"
binary_backup=""
if [[ $missing_binary_status -eq 0 ]]; then
  echo "expected refusal when installed handbook binary is missing"
  exit 1
fi
[[ "$missing_binary_output" == *"missing installed handbook binary"* ]] || {
  echo "expected missing installed handbook binary refusal"
  echo "$missing_binary_output"
  exit 1
}
missing_binary_after_count="$(count_run_dirs)"
[[ "$missing_binary_before_count" == "$missing_binary_after_count" ]] || {
  echo "missing-binary refusal unexpectedly created run evidence"
  exit 1
}

assert_no_misplaced_run_evidence "$HANDBOOK_HOME"
assert_no_misplaced_run_evidence "$HOME/.codex/skills"

echo "OK"
