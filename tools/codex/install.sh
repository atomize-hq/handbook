#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ROOT_DIR="$(cd "$SCRIPT_DIR/../.." && pwd)"
GENERATE_SCRIPT="$SCRIPT_DIR/generate.sh"
RUNTIME_TEMPLATE_ROOT="$SCRIPT_DIR/runtime"
INSTALL_SOURCE_ROOT="$ROOT_DIR/install/handbook-home"
HANDBOOK_HOME="$HOME/handbook"
HANDBOOK_DISCOVERY_ROOT="$HANDBOOK_HOME/.agents/skills"
CODEX_DISCOVERY_ROOT="$HOME/.codex/skills"
LEGACY_AGENTS_DISCOVERY_ROOT="$HOME/.agents/skills"
DISCOVERY_NAME="handbook-charter-intake"
ROOT_SKILL_NAME="handbook"
MANIFEST_VERSION="1"
INSTALL_REPO_LOCAL_PROJECTIONS=0

require_file() {
  local path="$1"

  if [[ ! -f "$path" ]]; then
    printf 'missing required file: %s\n' "$path" >&2
    exit 1
  fi
}

require_directory() {
  local path="$1"

  if [[ ! -d "$path" ]]; then
    printf 'missing required directory: %s\n' "$path" >&2
    exit 1
  fi
}

require_command() {
  local command_name="$1"

  command -v "$command_name" >/dev/null 2>&1 || {
    printf 'required command not found on PATH: %s\n' "$command_name" >&2
    exit 1
  }
}

while [[ $# -gt 0 ]]; do
  case "$1" in
    --repo-local)
      INSTALL_REPO_LOCAL_PROJECTIONS=1
      shift
      ;;
    -h|--help)
      cat <<'EOF'
Usage: install.sh [--repo-local]

Install the global Handbook runtime under ~/handbook and refresh ~/.codex/skills.

Options:
  --repo-local  Also refresh repo-local .agents/skills projections for development.
EOF
      exit 0
      ;;
    *)
      echo "unknown argument: $1" >&2
      exit 1
      ;;
  esac
done

render_template() {
  local template_path="$1"
  local destination_path="$2"

  python3 - "$template_path" "$destination_path" <<'PY'
import os
import pathlib
import sys

template_path = pathlib.Path(sys.argv[1])
destination_path = pathlib.Path(sys.argv[2])
text = template_path.read_text(encoding="utf-8")
replacements = {
    "__SKILL_NAME__": os.environ["SKILL_NAME"],
    "__HANDBOOK_RELEASE_VERSION__": os.environ["HANDBOOK_RELEASE_VERSION"],
    "__MANIFEST_VERSION__": os.environ["MANIFEST_VERSION"],
    "__GENERATED_AT_UTC__": os.environ["GENERATED_AT_UTC"],
}
for token, value in replacements.items():
    text = text.replace(token, value)
destination_path.parent.mkdir(parents=True, exist_ok=True)
destination_path.write_text(text, encoding="utf-8")
PY
}

install_copy() {
  local source_path="$1"
  local dest_path="$2"

  mkdir -p "$(dirname "$dest_path")"
  if [[ -d "$source_path" ]]; then
    cp -R "$source_path" "$dest_path"
  else
    cp "$source_path" "$dest_path"
  fi
}

install_discovery_entry() {
  local source_path="$1"
  local dest_path="$2"

  rm -rf "$dest_path"
  mkdir -p "$(dirname "$dest_path")"
  if ln -s "$source_path" "$dest_path" 2>/dev/null; then
    return 0
  fi
  cp -R "$source_path" "$dest_path"
}

if [[ -n "${SOURCE_DATE_EPOCH:-}" ]]; then
  GENERATED_AT_UTC="$(python3 - <<'PY'
import datetime
import os
print(datetime.datetime.fromtimestamp(int(os.environ["SOURCE_DATE_EPOCH"]), datetime.timezone.utc).strftime("%Y-%m-%dT%H:%M:%SZ"))
PY
)"
elif git -C "$ROOT_DIR" rev-parse --verify HEAD >/dev/null 2>&1; then
  GENERATED_AT_UTC="$(python3 - "$ROOT_DIR" <<'PY'
import datetime
import subprocess
import sys

root = sys.argv[1]
epoch = subprocess.check_output(
    ["git", "-C", root, "show", "-s", "--format=%ct", "HEAD"],
    text=True,
).strip()
print(datetime.datetime.fromtimestamp(int(epoch), datetime.timezone.utc).strftime("%Y-%m-%dT%H:%M:%SZ"))
PY
)"
else
  GENERATED_AT_UTC="$(date -u +%Y-%m-%dT%H:%M:%SZ)"
fi

HANDBOOK_RELEASE_VERSION="$(tr -d '[:space:]' <"$ROOT_DIR/VERSION")"
HANDBOOK_BIN_ON_PATH="$(command -v handbook 2>/dev/null || true)"

require_file "$GENERATE_SCRIPT"
require_file "$RUNTIME_TEMPLATE_ROOT/runtime-manifest.json.tmpl"
require_file "$INSTALL_SOURCE_ROOT/SKILL.md.tmpl"
require_file "$INSTALL_SOURCE_ROOT/agents/openai.yaml"
require_file "$INSTALL_SOURCE_ROOT/charter-intake/SKILL.md.tmpl"
require_file "$INSTALL_SOURCE_ROOT/charter-intake/openai.yaml"
require_command handbook

generated_root="$(mktemp -d "${TMPDIR:-/tmp}/handbook-install-generated.XXXXXX")"
generated_cleanup() {
  rm -rf "$generated_root"
}
trap generated_cleanup RETURN
bash "$GENERATE_SCRIPT" --output-root "$generated_root"

ROOT_SKILL_SOURCE="$generated_root/$ROOT_SKILL_NAME"
DISCOVERY_SOURCE="$generated_root/$DISCOVERY_NAME"

require_directory "$ROOT_SKILL_SOURCE"
require_directory "$DISCOVERY_SOURCE"

binary_version="$(handbook --version | awk '{print $NF}')"
[[ "$binary_version" == "$HANDBOOK_RELEASE_VERSION" ]] || {
  printf 'handbook binary version mismatch: repo=%s found=%s\n' "$HANDBOOK_RELEASE_VERSION" "$binary_version" >&2
  exit 1
}
[[ -n "$HANDBOOK_BIN_ON_PATH" && -x "$HANDBOOK_BIN_ON_PATH" ]] || {
  printf 'resolved handbook binary is not executable: %s\n' "$HANDBOOK_BIN_ON_PATH" >&2
  exit 1
}

stage_root="$(mktemp -d "$HOME/handbook.install.XXXXXX")"
cleanup() {
  rm -rf "$stage_root"
}
trap cleanup EXIT

mkdir -p \
  "$stage_root/bin" \
  "$stage_root/agents" \
  "$stage_root/charter-intake" \
  "$stage_root/resources/authoring" \
  "$stage_root/resources/charter" \
  "$stage_root/resources/project_context" \
  "$stage_root/resources/environment_inventory" \
  "$stage_root/.agents/skills"

install_copy "$INSTALL_SOURCE_ROOT/SKILL.md.tmpl" "$stage_root/SKILL.md.tmpl"
install_copy "$INSTALL_SOURCE_ROOT/agents/openai.yaml" "$stage_root/agents/openai.yaml"
install_copy "$INSTALL_SOURCE_ROOT/charter-intake/SKILL.md.tmpl" "$stage_root/charter-intake/SKILL.md.tmpl"
install_copy "$INSTALL_SOURCE_ROOT/charter-intake/openai.yaml" "$stage_root/charter-intake/openai.yaml"
install_copy "$HANDBOOK_BIN_ON_PATH" "$stage_root/bin/handbook"
install_copy "$ROOT_SKILL_SOURCE" "$stage_root/.agents/skills/$ROOT_SKILL_NAME"
install_copy "$DISCOVERY_SOURCE" "$stage_root/.agents/skills/$DISCOVERY_NAME"
install_copy "$ROOT_DIR/core/library/authoring/charter_authoring_method.md" "$stage_root/resources/authoring/charter_authoring_method.md"
install_copy "$ROOT_DIR/core/library/charter/CHARTER_INPUTS.yaml.tmpl" "$stage_root/resources/charter/CHARTER_INPUTS.yaml.tmpl"
install_copy "$ROOT_DIR/core/library/charter/charter_inputs_directive.md" "$stage_root/resources/charter/charter_inputs_directive.md"
install_copy "$ROOT_DIR/core/library/project_context/PROJECT_CONTEXT_INPUTS.yaml.tmpl" "$stage_root/resources/project_context/PROJECT_CONTEXT_INPUTS.yaml.tmpl"
install_copy "$ROOT_DIR/core/library/environment_inventory/ENVIRONMENT_INVENTORY_INPUTS.yaml.tmpl" "$stage_root/resources/environment_inventory/ENVIRONMENT_INVENTORY_INPUTS.yaml.tmpl"

export GENERATED_AT_UTC MANIFEST_VERSION HANDBOOK_RELEASE_VERSION
export SKILL_NAME="$ROOT_SKILL_NAME"
render_template "$INSTALL_SOURCE_ROOT/SKILL.md.tmpl" "$stage_root/SKILL.md"
export SKILL_NAME="$DISCOVERY_NAME"
render_template "$INSTALL_SOURCE_ROOT/charter-intake/SKILL.md.tmpl" "$stage_root/charter-intake/SKILL.md"
render_template "$RUNTIME_TEMPLATE_ROOT/runtime-manifest.json.tmpl" "$stage_root/runtime-manifest.json"
chmod 0755 "$stage_root/bin/handbook"

rm -rf "$HANDBOOK_HOME"
mv "$stage_root" "$HANDBOOK_HOME"
trap - EXIT

mkdir -p "$CODEX_DISCOVERY_ROOT"
install_discovery_entry "$HANDBOOK_DISCOVERY_ROOT/$ROOT_SKILL_NAME" "$CODEX_DISCOVERY_ROOT/$ROOT_SKILL_NAME"
install_discovery_entry "$HANDBOOK_DISCOVERY_ROOT/$DISCOVERY_NAME" "$CODEX_DISCOVERY_ROOT/$DISCOVERY_NAME"
rm -rf \
  "$LEGACY_AGENTS_DISCOVERY_ROOT/$ROOT_SKILL_NAME" \
  "$LEGACY_AGENTS_DISCOVERY_ROOT/$DISCOVERY_NAME"
rm -rf \
  "$ROOT_DIR/.agents/skills/$ROOT_SKILL_NAME" \
  "$ROOT_DIR/.agents/skills/$DISCOVERY_NAME"

if [[ "$INSTALL_REPO_LOCAL_PROJECTIONS" -eq 1 ]]; then
  bash "$GENERATE_SCRIPT" --repo-local
fi
