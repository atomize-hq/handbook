#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
EVIDENCE_DIR="$ROOT_DIR/.implemented/m10-orchestration"
LOG_PATH="$EVIDENCE_DIR/codex-skill-live-smoke.log"
STATE_ROOT="${XDG_STATE_HOME:-$HOME/.local/state}/handbook/intake/runs"
HANDBOOK_HOME="$HOME/handbook"
HANDBOOK_BINARY="$HANDBOOK_HOME/bin/handbook"
RUNTIME_MANIFEST="$HANDBOOK_HOME/runtime-manifest.json"
CHARTER_CANONICAL_CONTENT="$ROOT_DIR/docs/specs/handbook-contract-membrane/slices/HCM-2.2/contracts/canonical-charter-boundary-v1.1.yaml"
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

assert_credential_free_doctor_contract() {
  local doctor_json_path="$1"

  python3 - "$doctor_json_path" <<'PY'
import json
import sys

with open(sys.argv[1], encoding="utf-8") as handle:
    data = json.load(handle)
if data.get("schema_id") != "handbook.repository-doctor-report":
    raise SystemExit(f"unexpected Doctor schema: {data.get('schema_id')}")
if data.get("schema_version") != "1.2.0":
    raise SystemExit(f"unexpected Doctor schema version: {data.get('schema_version')}")
if data.get("status") != "indeterminate":
    raise SystemExit(f"expected indeterminate status, got: {data.get('status')}")
project_context = data.get("project_context")
if not project_context:
    raise SystemExit("expected retained Project Context fingerprint row")
if project_context.get("instance_id") != "project_context":
    raise SystemExit(f"unexpected Project Context instance: {project_context.get('instance_id')}")
if project_context.get("kind_ref") != "handbook.artifact-kind.project-context@1.0.0":
    raise SystemExit(f"unexpected Project Context kind: {project_context.get('kind_ref')}")
if project_context.get("canonical_path") != ".handbook/project/context.yaml":
    raise SystemExit(f"unexpected Project Context path: {project_context.get('canonical_path')}")
if not project_context.get("source_fingerprint", "").startswith("sha256:"):
    raise SystemExit("expected Project Context source fingerprint")
if not project_context.get("rendered_output_fingerprint", "").startswith("sha256:"):
    raise SystemExit("expected Project Context rendered fingerprint")
if project_context.get("rendered_media_type") != "text/markdown":
    raise SystemExit("expected Project Context rendered media type")
charter = data.get("charter")
if not charter:
    raise SystemExit("expected additive Charter observation row")
if charter.get("instance_id") != "project_authority":
    raise SystemExit(f"unexpected Charter instance: {charter.get('instance_id')}")
if charter.get("canonical_path") != ".handbook/project/charter.yaml":
    raise SystemExit(f"unexpected Charter path: {charter.get('canonical_path')}")
if charter.get("definition_closure_status") != "resolved":
    raise SystemExit("expected resolved Charter definition closure")
if len(charter.get("definition_closure", [])) != 8:
    raise SystemExit("expected complete Charter definition closure")
if charter.get("canonical_status") != "missing":
    raise SystemExit(f"expected missing selected Charter, got: {charter.get('canonical_status')}")
if charter.get("canonical_reason") != "required_path_missing":
    raise SystemExit(f"unexpected selected Charter reason: {charter.get('canonical_reason')}")
if charter.get("source_fingerprint") is not None:
    raise SystemExit("missing selected Charter unexpectedly has a source fingerprint")
if charter.get("rendered_output_fingerprint") is not None:
    raise SystemExit("missing selected Charter unexpectedly has a rendered fingerprint")
if charter.get("next_actions") != ["run_charter_author"]:
    raise SystemExit(f"unexpected Charter next actions: {charter.get('next_actions')}")
PY
}

assert_credential_free_flow_refusal() {
  local inspect_output_path="$1"

  grep -F 'OUTCOME: REFUSED' "$inspect_output_path" >/dev/null
  grep -F '.handbook/project/charter.yaml' "$inspect_output_path" >/dev/null
  ! grep -F '.handbook/charter/CHARTER.md' "$inspect_output_path" >/dev/null
  ! grep -F '.handbook/project_context/PROJECT_CONTEXT.md' "$inspect_output_path" >/dev/null
}

assert_no_misplaced_run_evidence() {
  local root="$1"
  local misplaced
  misplaced="$(
    find "$root" -type f \( \
      -name 'session.json' -o \
      -name 'doctor.before.json' -o \
      -name 'doctor.after_setup.json' -o \
      -name 'doctor.after_candidate.json' -o \
      -name 'author.stdout.txt' -o \
      -name 'author.stderr.txt' -o \
      -name 'replay.stdout.txt' -o \
      -name 'replay.stderr.txt' -o \
      -name 'bootstrap.stdout.txt' -o \
      -name 'bootstrap.stderr.txt' -o \
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

candidate_success_files() {
  cat <<'EOF'
author.exit
author.stderr.txt
author.stdout.txt
bootstrap.exit
bootstrap.stderr.txt
bootstrap.stdout.txt
charter_intake.yaml
doctor.after_candidate.json
doctor.after_setup.json
doctor.before.json
replay.exit
replay.stderr.txt
replay.stdout.txt
session.json
validate.exit
validate.stderr.txt
validate.stdout.txt
EOF
}

write_charter_intake_envelope() {
  local destination_path="$1"
  local mode="$2"

  python3 - "$CHARTER_CANONICAL_CONTENT" "$destination_path" "$mode" <<'PY'
import pathlib
import sys

source = pathlib.Path(sys.argv[1]).read_text(encoding="utf-8")
destination = pathlib.Path(sys.argv[2])
mode = sys.argv[3]
coverage_ids = (
    "project_shape.definition",
    "delivery.constraints",
    "delivery.default_implications",
    "operational_reality.production_state",
    "risk.domains",
    "engineering_posture.baseline",
    "policy.authority_and_revision",
    "governance.decision_authority",
    "governance.required_approvals",
    "governance.exception_policy",
    "engineering_posture.dimensions",
    "engineering_posture.red_lines",
    "governance.review_triggers",
    "governance.reassessment_triggers",
    "debt.register",
    "decisions.records",
)
parts = [f"mode: {mode}\ncontent:\n"]
parts.extend(f"  {line}\n" for line in source.splitlines())
parts.append("coverage:\n")
for coverage_id in coverage_ids:
    parts.append(
        f'  - coverage_id: "{coverage_id}"\n'
        "    source_kind: user_declaration\n"
        f'    value_ref: "input://{coverage_id}"\n'
        "    evidence_refs: []\n"
        "    confidence: high\n"
        "    freshness: null\n"
        "    sensitivity: public\n"
        "    contradiction_refs: []\n"
        "    waiver_ref: null\n"
    )
parts.append(
    "consumer:\n"
    "  kind: agent\n"
    "  id: handbook-charter-intake\n"
    '  version: "1.0"\n'
    "prompt_event_refs: []\n"
    'finalized_at_utc: "2026-07-20T00:00:00Z"\n'
    "expected_current_fingerprint: null\n"
)
destination.write_text("".join(parts), encoding="utf-8")
PY
}

write_selected_environment_inventory_input() {
  local source_path="$1"
  local destination_path="$2"

  python3 - "$source_path" "$destination_path" <<'PY'
import pathlib
import sys

source = pathlib.Path(sys.argv[1]).read_text(encoding="utf-8")
replacements = {
    'charter_ref: ".handbook/charter/CHARTER.md"':
        'charter_ref: ".handbook/project/charter.yaml"',
    'exception_record_location: ".handbook/charter/CHARTER.md#exceptions"':
        'exception_record_location: ".handbook/project/charter.yaml#/governance/exception_process"',
}
destination = source
for legacy, selected in replacements.items():
    if destination.count(legacy) != 1:
        raise SystemExit(f"expected exactly one fixture reference to project: {legacy}")
    destination = destination.replace(legacy, selected)
if ".handbook/charter/CHARTER.md" in destination:
    raise SystemExit("temporary Environment Inventory input retained legacy Charter Markdown")
pathlib.Path(sys.argv[2]).write_text(destination, encoding="utf-8")
PY
}

snapshot_file_tree() {
  local root="$1"

  (
    cd "$root"
    find . -type f -print0 | sort -z | xargs -0 -r sha256sum
  )
}

assert_native_refusal_repository_delta() {
  local before_snapshot="$1"
  local after_snapshot="$2"
  local repo_root="$3"
  local filtered_before
  local filtered_after

  filtered_before="$(
    printf '%s\n' "$before_snapshot" \
      | grep -v -F -e './.handbook/state/locks/promotion.lock' \
        -e './.handbook/state/locks/registry.lock'
  )"
  filtered_after="$(
    printf '%s\n' "$after_snapshot" \
      | grep -v -F -e './.handbook/state/locks/promotion.lock' \
        -e './.handbook/state/locks/registry.lock'
  )"
  [[ "$filtered_after" == "$filtered_before" ]] || {
    echo "native-authority refusal changed bytes outside predecessor-recovery locks" >&2
    return 1
  }
  for lock_path in \
    "$repo_root/.handbook/state/locks/promotion.lock" \
    "$repo_root/.handbook/state/locks/registry.lock"; do
    [[ -f "$lock_path" && ! -L "$lock_path" && ! -s "$lock_path" ]] || {
      echo "unexpected predecessor-recovery lock state: $lock_path" >&2
      return 1
    }
  done
  test ! -e "$repo_root/.handbook/project/charter.yaml"
}

assert_missing_selected_charter_refusal() {
  local output_path="$1"

  python3 - "$output_path" <<'PY'
import json
import sys

with open(sys.argv[1], encoding="utf-8") as handle:
    data = json.load(handle)
if data.get("schema_id") != "handbook.charter-operation-result":
    raise SystemExit(f"unexpected Charter result schema: {data.get('schema_id')}")
if data.get("schema_version") != "1.0":
    raise SystemExit(f"unexpected Charter result version: {data.get('schema_version')}")
if data.get("operation") != "validate" or data.get("status") != "refused":
    raise SystemExit("selected-truth validate did not refuse")
if data.get("canonical_path") is not None:
    raise SystemExit("missing selected truth unexpectedly projected a canonical path")
if data.get("changed_paths") != []:
    raise SystemExit(f"validate-only changed paths: {data.get('changed_paths')}")
refusal = data.get("refusal") or {}
if refusal.get("code") != "canonical_charter_missing":
    raise SystemExit(f"unexpected validate refusal: {refusal}")
PY
}

assert_candidate_contract() {
  local output_path="$1"
  local repo_root="$2"

  python3 - "$output_path" "$repo_root" <<'PY'
import hashlib
import json
import pathlib
import sys

with open(sys.argv[1], encoding="utf-8") as handle:
    data = json.load(handle)
if data.get("schema_id") != "handbook.charter-operation-result":
    raise SystemExit(f"unexpected Charter result schema: {data.get('schema_id')}")
if data.get("schema_version") != "1.0":
    raise SystemExit(f"unexpected Charter result version: {data.get('schema_version')}")
if data.get("operation") != "author" or data.get("status") != "succeeded":
    raise SystemExit("explicit-mode Charter acquisition did not succeed")
for field in ("intake_ref", "intake_fingerprint", "candidate_ref", "candidate_fingerprint"):
    value = data.get(field)
    if not isinstance(value, str) or not value:
        raise SystemExit(f"missing immutable candidate field: {field}")
for field in ("intake_fingerprint", "candidate_fingerprint"):
    if not data[field].startswith("sha256:"):
        raise SystemExit(f"non-SHA-256 candidate field: {field}")
changed_paths = data.get("changed_paths")
if not isinstance(changed_paths, list) or len(changed_paths) != 4:
    raise SystemExit(f"unexpected candidate changed paths: {changed_paths}")
expected_parents = {
    ".handbook/state/candidate-content",
    ".handbook/state/intake-records",
    ".handbook/state/lifecycle-validation-results",
    ".handbook/evidence/charter/candidates",
}
actual_parents = {str(pathlib.PurePosixPath(path).parent) for path in changed_paths}
if actual_parents != expected_parents:
    raise SystemExit(f"unexpected candidate path partitions: {actual_parents}")
repo_root = pathlib.Path(sys.argv[2])
for changed_path in changed_paths:
    if not (
        changed_path.startswith(".handbook/state/")
        or changed_path.startswith(".handbook/evidence/charter/candidates/")
    ):
        raise SystemExit(f"candidate write escaped immutable state: {changed_path}")
    if not (repo_root / changed_path).is_file():
        raise SystemExit(f"candidate path missing: {changed_path}")
candidate_path = next(
    path
    for path in changed_paths
    if path.startswith(".handbook/evidence/charter/candidates/")
)
validation_path = next(
    path
    for path in changed_paths
    if path.startswith(".handbook/state/lifecycle-validation-results/")
)
with open(repo_root / candidate_path, encoding="utf-8") as handle:
    candidate = json.load(handle)
if candidate.get("schema_version") != "1.3":
    raise SystemExit(f"unexpected candidate version: {candidate.get('schema_version')}")
binding = candidate.get("validation_result_binding")
expected_binding_fields = {
    "validation_result_ref",
    "validation_result_fingerprint",
    "result_document_sha256",
    "result_byte_length",
}
if not isinstance(binding, dict) or set(binding) != expected_binding_fields:
    raise SystemExit(f"candidate validation binding is not closed: {binding}")
if binding["validation_result_ref"] != validation_path.removeprefix(".handbook/state/"):
    raise SystemExit(f"candidate semantic result reference mismatch: {binding}")
validation_bytes = (repo_root / validation_path).read_bytes()
if not validation_bytes.endswith(b"\n") or validation_bytes[:-1].endswith(b"\n"):
    raise SystemExit("lifecycle-validation result is not exact JCS-plus-LF")
expected_digest = "sha256:" + hashlib.sha256(validation_bytes).hexdigest()
if binding["result_document_sha256"] != expected_digest:
    raise SystemExit(f"candidate exact result digest mismatch: {binding}")
if binding["result_byte_length"] != len(validation_bytes):
    raise SystemExit(f"candidate exact result byte length mismatch: {binding}")
validation = json.loads(validation_bytes)
if (
    validation.get("schema_id") != "handbook.lifecycle-validation-result"
    or validation.get("schema_version") != "1.0"
):
    raise SystemExit("unexpected lifecycle-validation result identity")
if validation.get("validation_result_fingerprint") != binding["validation_result_fingerprint"]:
    raise SystemExit("candidate and validation semantic result identities differ")
if validation.get("candidate_subject_fingerprint") != candidate.get(
    "candidate_subject_fingerprint"
):
    raise SystemExit("candidate and validation subject identities differ")
if data.get("canonical_path") is not None:
    raise SystemExit("candidate acquisition projected selected canonical truth")
if data.get("refusal") is not None:
    raise SystemExit(f"candidate acquisition unexpectedly refused: {data.get('refusal')}")
PY
}

assert_candidate_replay_contract() {
  local first_output_path="$1"
  local replay_output_path="$2"

  python3 - "$first_output_path" "$replay_output_path" <<'PY'
import json
import sys

with open(sys.argv[1], encoding="utf-8") as handle:
    first = json.load(handle)
with open(sys.argv[2], encoding="utf-8") as handle:
    replay = json.load(handle)
for field in (
    "intake_ref",
    "intake_fingerprint",
    "candidate_ref",
    "candidate_fingerprint",
    "changed_paths",
):
    if replay.get(field) != first.get(field):
        raise SystemExit(f"candidate replay changed {field}")
if replay.get("status") != "succeeded" or replay.get("operation") != "author":
    raise SystemExit("candidate replay did not remain a successful author operation")
PY
}

assert_native_authority_unavailable() {
  local output_path="$1"

  python3 - "$output_path" <<'PY'
import json
import sys

with open(sys.argv[1], encoding="utf-8") as handle:
    data = json.load(handle)
if data.get("schema_id") != "handbook.approver-admin-result":
    raise SystemExit(f"unexpected approver result schema: {data.get('schema_id')}")
if data.get("schema_version") != "1.0":
    raise SystemExit(f"unexpected approver result version: {data.get('schema_version')}")
if data.get("operation") != "bootstrap" or data.get("status") != "refused":
    raise SystemExit("native bootstrap did not fail closed")
refusal = data.get("refusal") or {}
if refusal.get("code") != "AUTHENTICATOR_UNAVAILABLE":
    raise SystemExit(f"unexpected native-authority refusal: {refusal}")
if refusal.get("message") != "native CTAP2.1 authenticator API is unavailable":
    raise SystemExit(f"unexpected native-authority message: {refusal.get('message')}")
if refusal.get("retryable") is not True:
    raise SystemExit("native-authority refusal must remain retryable")
if data.get("changed_paths") != []:
    raise SystemExit(f"native-authority refusal changed paths: {data.get('changed_paths')}")
PY
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
  local installed_charter_skill="$HANDBOOK_HOME/charter-intake/SKILL.md"

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
    "$installed_charter_skill" \
    "$HANDBOOK_HOME/resources/project_context/PROJECT_CONTEXT_INPUTS.yaml.tmpl" \
    "$HANDBOOK_HOME/resources/environment_inventory/ENVIRONMENT_INVENTORY_INPUTS.yaml.tmpl"; do
    [[ -f "$required" ]] || {
      echo "REFUSED: missing installed handbook home prerequisite: $required" >&2
      return 1
    }
  done
  grep -F 'handbook author charter --mode guided-adaptive --from-inputs <path|->' \
    "$installed_charter_skill" >/dev/null
  grep -F 'handbook author charter --validate' "$installed_charter_skill" >/dev/null
  ! grep -F '.handbook/charter/CHARTER.md' "$installed_charter_skill" >/dev/null

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
  local setup_status
  local author_status
  local validate_status
  local replay_status
  local bootstrap_status
  local before_operation
  local after_operation

  LAST_RUN_DIR=""
  repo_root="$(git -C "$repo_dir" rev-parse --show-toplevel 2>/dev/null)" || {
    echo "REFUSED: run this skill from inside a real git repository." >&2
    return 1
  }

  validate_runtime_contract || return 1
  export HANDBOOK_HOME

  run_dir="$(mktemp -d "$STATE_ROOT/run.XXXXXX")"
  LAST_RUN_DIR="$run_dir"
  write_charter_intake_envelope "$run_dir/charter_intake.yaml" guided_adaptive
  write_session_json "$run_dir/session.json" "$repo_root" "$HANDBOOK_HOME" "$MANIFEST_VERSION_VALUE"

  doctor_before_status="$(capture_doctor_json "$repo_root" "$run_dir/doctor.before.json")"
  if [[ ! -d "$repo_root/.handbook" ]]; then
    setup_status="$(
      capture_in_repo \
        "$repo_root" \
        /dev/null \
        /dev/null \
        /dev/null \
        "$HANDBOOK_BINARY" setup
    )"
    [[ "$setup_status" -eq 1 ]] || {
      echo "expected setup indeterminate exit 1, got: $setup_status" >&2
      return 1
    }
    capture_doctor_json "$repo_root" "$run_dir/doctor.after_setup.json" >/dev/null
  fi

  before_operation="$(snapshot_file_tree "$repo_root")"
  validate_status="$(
    capture_in_repo \
    "$repo_root" \
    "$run_dir/validate.stdout.txt" \
    "$run_dir/validate.stderr.txt" \
    "$run_dir/validate.exit" \
    "$HANDBOOK_BINARY" author charter --validate --json
  )"
  [[ "$validate_status" -eq 1 ]] || {
    echo "expected selected-truth validation refusal exit 1, got: $validate_status" >&2
    return 1
  }
  after_operation="$(snapshot_file_tree "$repo_root")"
  [[ "$after_operation" == "$before_operation" ]] || {
    echo "selected-truth validation refusal mutated repository bytes" >&2
    return 1
  }
  assert_missing_selected_charter_refusal "$run_dir/validate.stdout.txt"

  author_status="$(
    capture_in_repo \
    "$repo_root" \
    "$run_dir/author.stdout.txt" \
    "$run_dir/author.stderr.txt" \
    "$run_dir/author.exit" \
    "$HANDBOOK_BINARY" author charter \
      --mode guided-adaptive \
      --from-inputs "$run_dir/charter_intake.yaml" \
      --json
  )"
  [[ "$author_status" -eq 0 ]] || {
    echo "expected explicit-mode candidate creation exit 0, got: $author_status" >&2
    return 1
  }
  assert_candidate_contract "$run_dir/author.stdout.txt" "$repo_root"
  test ! -e "$repo_root/.handbook/project/charter.yaml"
  test ! -e "$repo_root/.handbook/charter/CHARTER.md"

  before_operation="$(snapshot_file_tree "$repo_root")"
  replay_status="$(
    capture_in_repo \
      "$repo_root" \
      "$run_dir/replay.stdout.txt" \
      "$run_dir/replay.stderr.txt" \
      "$run_dir/replay.exit" \
      "$HANDBOOK_BINARY" author charter \
        --mode guided-adaptive \
        --from-inputs "$run_dir/charter_intake.yaml" \
        --json
  )"
  [[ "$replay_status" -eq 0 ]] || {
    echo "expected immutable candidate replay exit 0, got: $replay_status" >&2
    return 1
  }
  after_operation="$(snapshot_file_tree "$repo_root")"
  [[ "$after_operation" == "$before_operation" ]] || {
    echo "candidate replay changed immutable repository bytes" >&2
    return 1
  }
  assert_candidate_replay_contract "$run_dir/author.stdout.txt" "$run_dir/replay.stdout.txt"

  before_operation="$(snapshot_file_tree "$repo_root")"
  bootstrap_status="$(
    capture_in_repo \
      "$repo_root" \
      "$run_dir/bootstrap.stdout.txt" \
      "$run_dir/bootstrap.stderr.txt" \
      "$run_dir/bootstrap.exit" \
      "$HANDBOOK_BINARY" approvers bootstrap \
        --initial-charter-quorum "Project owner approval=Project owner" \
        --json
  )"
  [[ "$bootstrap_status" -eq 1 ]] || {
    echo "expected native-authority refusal exit 1, got: $bootstrap_status" >&2
    return 1
  }
  after_operation="$(snapshot_file_tree "$repo_root")"
  assert_native_refusal_repository_delta \
    "$before_operation" \
    "$after_operation" \
    "$repo_root"
  assert_native_authority_unavailable "$run_dir/bootstrap.stdout.txt"
  capture_doctor_json "$repo_root" "$run_dir/doctor.after_candidate.json" >/dev/null

  return 0
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

echo "==> explicit-mode immutable candidate smoke against installed runtime"
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
assert_run_dir_file_set "$happy_run_dir" "$(candidate_success_files)"
assert_session_fields "$happy_run_dir/session.json" "$HANDBOOK_HOME"
test ! -e "$happy_repo/.handbook/project/charter.yaml"
test ! -e "$happy_repo/.handbook/charter/CHARTER.md"

echo "==> credential-free integration ceiling without Codex or native credentials"
all_three_repo="$tmp_root/all-three-repo"
offline_path="$tmp_root/no-codex-path"
all_three_doctor="$tmp_root/all-three-doctor.json"
all_three_inspect="$tmp_root/all-three-inspect.txt"
all_three_environment_input="$tmp_root/environment-inventory-selected-charter.yaml"
all_three_environment_validate="$tmp_root/environment-inventory-validate.txt"
all_three_environment_author="$tmp_root/environment-inventory-author.txt"
mkdir -p "$all_three_repo" "$offline_path"
git -C "$all_three_repo" init -q
write_selected_environment_inventory_input \
  "$ENVIRONMENT_INVENTORY_FIXTURE_INPUTS" \
  "$all_three_environment_input"
all_three_setup_status="$(
  capture_in_repo \
    "$all_three_repo" \
    /dev/null \
    /dev/null \
    /dev/null \
    env -u CODEX_API_KEY -u OPENAI_API_KEY \
    PATH="$offline_path" \
    "$HANDBOOK_BINARY" setup
)"
[[ "$all_three_setup_status" -eq 1 ]] || {
  echo "expected all-three setup indeterminate exit 1, got: $all_three_setup_status"
  exit 1
}
(
  cd "$all_three_repo"
  env -u CODEX_API_KEY -u OPENAI_API_KEY PATH="$offline_path" \
    "$HANDBOOK_BINARY" author project-context --validate --from-inputs - \
    <"$PROJECT_CONTEXT_FIXTURE_INPUTS" >/dev/null
  env -u CODEX_API_KEY -u OPENAI_API_KEY PATH="$offline_path" \
    "$HANDBOOK_BINARY" author project-context --from-inputs - \
    <"$PROJECT_CONTEXT_FIXTURE_INPUTS" >/dev/null
)
before_environment="$(snapshot_file_tree "$all_three_repo")"
all_three_environment_validate_status="$(
  capture_in_repo \
    "$all_three_repo" \
    "$all_three_environment_validate" \
    /dev/null \
    /dev/null \
    env -u CODEX_API_KEY -u OPENAI_API_KEY PATH="$offline_path" \
    "$HANDBOOK_BINARY" author environment-inventory --validate \
      --from-inputs "$all_three_environment_input"
)"
[[ "$all_three_environment_validate_status" -eq 1 ]] || {
  echo "expected Environment Inventory validation refusal exit 1, got: $all_three_environment_validate_status"
  exit 1
}
after_environment="$(snapshot_file_tree "$all_three_repo")"
[[ "$after_environment" == "$before_environment" ]] || {
  echo "Environment Inventory validation refusal mutated repository bytes"
  exit 1
}
grep -F 'OUTCOME: REFUSED' "$all_three_environment_validate" >/dev/null
grep -F 'CATEGORY: MissingRequiredCharter' "$all_three_environment_validate" >/dev/null

all_three_environment_author_status="$(
  capture_in_repo \
    "$all_three_repo" \
    "$all_three_environment_author" \
    /dev/null \
    /dev/null \
    env -u CODEX_API_KEY -u OPENAI_API_KEY PATH="$offline_path" \
    "$HANDBOOK_BINARY" author environment-inventory \
      --from-inputs "$all_three_environment_input"
)"
[[ "$all_three_environment_author_status" -eq 1 ]] || {
  echo "expected Environment Inventory author refusal exit 1, got: $all_three_environment_author_status"
  exit 1
}
after_environment="$(snapshot_file_tree "$all_three_repo")"
[[ "$after_environment" == "$before_environment" ]] || {
  echo "Environment Inventory author refusal mutated repository bytes"
  exit 1
}
grep -F 'OUTCOME: REFUSED' "$all_three_environment_author" >/dev/null
grep -F 'CATEGORY: MissingRequiredCharter' "$all_three_environment_author" >/dev/null

all_three_doctor_status="$(capture_doctor_json "$all_three_repo" "$all_three_doctor")"
[[ "$all_three_doctor_status" -eq 1 ]] || {
  echo "expected all-three Doctor exit 1, got: $all_three_doctor_status"
  exit 1
}
test ! -e "$all_three_repo/.handbook/project/charter.yaml"
test ! -e "$all_three_repo/.handbook/charter/CHARTER.md"
test -f "$all_three_repo/.handbook/project/context.yaml"
test ! -e "$all_three_repo/.handbook/project_context/PROJECT_CONTEXT.md"
grep -F 'schema_id: "handbook.artifact.project-context"' \
  "$all_three_repo/.handbook/project/context.yaml" >/dev/null
assert_credential_free_doctor_contract "$all_three_doctor"
all_three_inspect_status="$(
  capture_in_repo \
    "$all_three_repo" \
    "$all_three_inspect" \
    /dev/null \
    /dev/null \
    env -u CODEX_API_KEY -u OPENAI_API_KEY PATH="$offline_path" \
    "$HANDBOOK_BINARY" inspect --packet planning.packet
)"
[[ "$all_three_inspect_status" -eq 1 ]] || {
  echo "expected flow refusal without promoted Charter exit 1, got: $all_three_inspect_status"
  exit 1
}
assert_credential_free_flow_refusal "$all_three_inspect"

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
