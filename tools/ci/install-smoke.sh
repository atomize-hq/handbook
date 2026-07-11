#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
EVIDENCE_DIR="$ROOT_DIR/.implemented/m10-orchestration"
LOG_PATH="$EVIDENCE_DIR/install-smoke.log"
export PATH="${CARGO_HOME:-$HOME/.cargo}/bin:$PATH"

mkdir -p "$EVIDENCE_DIR"
exec > >(tee "$LOG_PATH") 2>&1

HANDBOOK_HOME="$HOME/handbook"
INSTALLED_ROOT_SKILL="$HANDBOOK_HOME/.agents/skills/handbook"
INSTALLED_DISCOVERY_SKILL="$HANDBOOK_HOME/.agents/skills/handbook-charter-intake"
CODEX_ROOT_SKILL="$HOME/.codex/skills/handbook"
CODEX_DISCOVERY_SKILL="$HOME/.codex/skills/handbook-charter-intake"
LEGACY_AGENTS_ROOT_SKILL="$HOME/.agents/skills/handbook"
LEGACY_AGENTS_DISCOVERY_SKILL="$HOME/.agents/skills/handbook-charter-intake"

assert_exact_file_set() {
  local root="$1"
  local expected="$2"
  local actual

  actual="$(
    find "$root" -type f -print \
      | sed "s#^$root/##" \
      | sort
  )"
  [[ "$actual" == "$expected" ]] || {
    echo "unexpected file set under $root"
    echo "actual:"
    printf '%s\n' "$actual"
    echo "expected:"
    printf '%s\n' "$expected"
    exit 1
  }
}

assert_path_absent() {
  local path="$1"

  [[ ! -e "$path" ]] || {
    echo "unexpected path present: $path"
    exit 1
  }
}

assert_manifest_fields() {
  local manifest_path="$1"
  python3 - "$manifest_path" <<'PY'
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
PY
}

assert_repo_projection_thin() {
  assert_exact_file_set "$ROOT_DIR/.agents/skills/handbook" "$(cat <<'EOF'
SKILL.md
agents/openai.yaml
EOF
)"
  assert_exact_file_set "$ROOT_DIR/.agents/skills/handbook-charter-intake" "$(cat <<'EOF'
SKILL.md
agents/openai.yaml
EOF
  )"
}

assert_repo_projection_absent() {
  assert_path_absent "$ROOT_DIR/.agents/skills/handbook"
  assert_path_absent "$ROOT_DIR/.agents/skills/handbook-charter-intake"
}

assert_repo_root_install_sources_absent() {
  assert_path_absent "$ROOT_DIR/SKILL.md"
  assert_path_absent "$ROOT_DIR/SKILL.md.tmpl"
  assert_path_absent "$ROOT_DIR/agents"
  assert_path_absent "$ROOT_DIR/charter-intake"
}

assert_installed_home_file_set() {
  assert_exact_file_set "$HANDBOOK_HOME" "$(cat <<'EOF'
.agents/skills/handbook-charter-intake/SKILL.md
.agents/skills/handbook-charter-intake/agents/openai.yaml
.agents/skills/handbook/SKILL.md
.agents/skills/handbook/agents/openai.yaml
SKILL.md
SKILL.md.tmpl
agents/openai.yaml
bin/handbook
charter-intake/SKILL.md
charter-intake/SKILL.md.tmpl
charter-intake/openai.yaml
resources/authoring/charter_authoring_method.md
resources/charter/CHARTER_INPUTS.yaml.tmpl
resources/charter/charter_inputs_directive.md
resources/environment_inventory/ENVIRONMENT_INVENTORY_INPUTS.yaml.tmpl
resources/project_context/PROJECT_CONTEXT_INPUTS.yaml.tmpl
runtime-manifest.json
EOF
)"
}

assert_installed_runtime_contract() {
  test -x "$HANDBOOK_HOME/bin/handbook"
  test -d "$HANDBOOK_HOME/resources/authoring"
  test -d "$HANDBOOK_HOME/resources/charter"
  test -d "$HANDBOOK_HOME/resources/project_context"
  test -d "$HANDBOOK_HOME/resources/environment_inventory"
  assert_path_absent "$HANDBOOK_HOME/bin/handbook-charter-intake"
  assert_path_absent "$HANDBOOK_HOME/share"
}

assert_discovery_links_to_handbook_home() {
  [[ "$(readlink "$CODEX_ROOT_SKILL")" == "$INSTALLED_ROOT_SKILL" ]] || {
    echo "unexpected root discovery link target"
    readlink "$CODEX_ROOT_SKILL" || true
    exit 1
  }
  [[ "$(readlink "$CODEX_DISCOVERY_SKILL")" == "$INSTALLED_DISCOVERY_SKILL" ]] || {
    echo "unexpected leaf discovery link target"
    readlink "$CODEX_DISCOVERY_SKILL" || true
    exit 1
  }
  assert_path_absent "$LEGACY_AGENTS_ROOT_SKILL"
  assert_path_absent "$LEGACY_AGENTS_DISCOVERY_SKILL"
}

assert_dev_setup_links_to_repo() {
  [[ "$(readlink "$CODEX_ROOT_SKILL")" == "$ROOT_DIR/.agents/skills/handbook" ]] || {
    echo "unexpected dev root discovery link target"
    readlink "$CODEX_ROOT_SKILL" || true
    exit 1
  }
  [[ "$(readlink "$CODEX_DISCOVERY_SKILL")" == "$ROOT_DIR/.agents/skills/handbook-charter-intake" ]] || {
    echo "unexpected dev leaf discovery link target"
    readlink "$CODEX_DISCOVERY_SKILL" || true
    exit 1
  }
  assert_path_absent "$LEGACY_AGENTS_ROOT_SKILL"
  assert_path_absent "$LEGACY_AGENTS_DISCOVERY_SKILL"
}

target_label() {
  case "$(uname -s):$(uname -m)" in
    Darwin:arm64|Darwin:aarch64)
      printf 'macos_arm64\n'
      ;;
    Linux:x86_64|Linux:amd64)
      printf 'linux_x86_64\n'
      ;;
    *)
      echo "unsupported platform for release install smoke: $(uname -s) $(uname -m)"
      exit 1
      ;;
  esac
}

echo "==> cargo install (crates/cli)"
cd "$ROOT_DIR"
cargo install --locked --force --path crates/cli

echo "==> help smoke"
handbook --help >/dev/null
handbook doctor --help >/dev/null
handbook author charter --help >/dev/null

echo "==> generator/install smoke"
bash tools/codex/generate.sh --repo-local
assert_repo_projection_thin
assert_repo_root_install_sources_absent
rm -rf "$ROOT_DIR/.agents/skills/handbook" "$ROOT_DIR/.agents/skills/handbook-charter-intake"
bash tools/codex/install.sh
assert_repo_projection_absent
assert_installed_home_file_set
assert_installed_runtime_contract
assert_discovery_links_to_handbook_home
assert_manifest_fields "$HANDBOOK_HOME/runtime-manifest.json"

echo "==> reinstall smoke"
before_listing="$(find "$HANDBOOK_HOME" -type f -print | sort)"
bash tools/codex/install.sh
after_listing="$(find "$HANDBOOK_HOME" -type f -print | sort)"
[[ "$before_listing" == "$after_listing" ]] || {
  echo "reinstall changed installed file set"
  exit 1
}
assert_repo_projection_absent
assert_installed_home_file_set
assert_installed_runtime_contract
assert_discovery_links_to_handbook_home

echo "==> dev-setup symlink smoke"
bash tools/codex/dev-setup.sh
test -L "$CODEX_ROOT_SKILL"
test -L "$CODEX_DISCOVERY_SKILL"
assert_dev_setup_links_to_repo

echo "==> install-mode crossover smoke"
bash tools/codex/install.sh
test -L "$CODEX_ROOT_SKILL"
test -L "$CODEX_DISCOVERY_SKILL"
assert_repo_projection_absent
assert_discovery_links_to_handbook_home
assert_installed_home_file_set
assert_installed_runtime_contract

echo "==> public install wrapper smoke"
release_version="$(tr -d '[:space:]' < "$ROOT_DIR/VERSION")"
release_home="$(mktemp -d "${TMPDIR:-/tmp}/handbook-release-home.XXXXXX")"
release_bundle_root="$(mktemp -d "${TMPDIR:-/tmp}/handbook-release-bundles.XXXXXX")"
mkdir -p "$release_bundle_root/v$release_version"
bash tools/release/package-handbook-home.sh \
  --binary "$(command -v handbook)" \
  --label "$(target_label)" \
  --version "$release_version" \
  --output-dir "$release_bundle_root/v$release_version" >/dev/null
(
  cd "$release_bundle_root/v$release_version"
  shasum -a 256 "handbook-v${release_version}-$(target_label).tar.gz" > SHA256SUMS
)
HOME="$release_home" HANDBOOK_INSTALL_BASE_URL="file://$release_bundle_root" \
  bash scripts/handbook/install.sh --version "$release_version"
[[ "$(readlink "$release_home/.codex/skills/handbook")" == "$release_home/handbook/.agents/skills/handbook" ]] || {
  echo "unexpected public install codex root link target"
  readlink "$release_home/.codex/skills/handbook" || true
  exit 1
}
[[ "$(readlink "$release_home/.codex/skills/handbook-charter-intake")" == "$release_home/handbook/.agents/skills/handbook-charter-intake" ]] || {
  echo "unexpected public install codex leaf link target"
  readlink "$release_home/.codex/skills/handbook-charter-intake" || true
  exit 1
}
assert_path_absent "$release_home/.agents/skills/handbook"
assert_path_absent "$release_home/.agents/skills/handbook-charter-intake"

echo "OK"
