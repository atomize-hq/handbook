#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
SOURCE_ROOT="$ROOT_DIR/install/handbook-home"
OUTPUT_ROOT=""

usage() {
  cat <<'EOF'
Usage: generate.sh [--repo-local | --output-root <dir>]

Generate the thin Handbook skill projections.

Options:
  --repo-local         Write projections into the repo at .agents/skills/
  --output-root <dir>  Write projections into the provided output root
EOF
}

while [[ $# -gt 0 ]]; do
  case "$1" in
    --repo-local)
      OUTPUT_ROOT="$ROOT_DIR/.agents/skills"
      shift
      ;;
    --output-root)
      OUTPUT_ROOT="${2:-}"
      [[ -n "$OUTPUT_ROOT" ]] || {
        echo "missing value for --output-root" >&2
        usage >&2
        exit 1
      }
      shift 2
      ;;
    -h|--help)
      usage
      exit 0
      ;;
    *)
      echo "unknown argument: $1" >&2
      usage >&2
      exit 1
      ;;
  esac
done

[[ -n "$OUTPUT_ROOT" ]] || {
  echo "refusing to generate projections without an explicit target; use --repo-local or --output-root" >&2
  usage >&2
  exit 1
}

tmp_dir="$(mktemp -d "${TMPDIR:-/tmp}/handbook-codex-generate.XXXXXX")"
cleanup() {
  rm -rf "$tmp_dir"
}
trap cleanup EXIT

render_template() {
  local template_path="$1"
  local destination_path="$2"

  python3 - "$template_path" "$destination_path" <<'PY'
import pathlib
import sys

template_path = pathlib.Path(sys.argv[1])
destination_path = pathlib.Path(sys.argv[2])
text = template_path.read_text(encoding="utf-8")
destination_path.parent.mkdir(parents=True, exist_ok=True)
destination_path.write_text(text, encoding="utf-8")
PY
}

copy_file() {
  local source_path="$1"
  local destination_path="$2"

  mkdir -p "$(dirname "$destination_path")"
  cp "$source_path" "$destination_path"
}

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
    echo "unexpected generated file set under $root" >&2
    echo "actual:" >&2
    printf '%s\n' "$actual" >&2
    echo "expected:" >&2
    printf '%s\n' "$expected" >&2
    exit 1
  }
}

generated_root_skill_tmp="$tmp_dir/handbook-skill.md"
generated_leaf_skill_tmp="$tmp_dir/handbook-charter-intake-skill.md"
generated_root="$tmp_dir/.agents/skills"
root_projection_tmp="$generated_root/handbook"
leaf_projection_tmp="$generated_root/handbook-charter-intake"

render_template "$SOURCE_ROOT/SKILL.md.tmpl" "$generated_root_skill_tmp"
render_template "$SOURCE_ROOT/charter-intake/SKILL.md.tmpl" "$generated_leaf_skill_tmp"

copy_file "$generated_root_skill_tmp" "$root_projection_tmp/SKILL.md"
copy_file "$SOURCE_ROOT/agents/openai.yaml" "$root_projection_tmp/agents/openai.yaml"
copy_file "$generated_leaf_skill_tmp" "$leaf_projection_tmp/SKILL.md"
copy_file "$SOURCE_ROOT/charter-intake/openai.yaml" "$leaf_projection_tmp/agents/openai.yaml"

assert_exact_file_set "$root_projection_tmp" "$(cat <<'EOF'
SKILL.md
agents/openai.yaml
EOF
)"
assert_exact_file_set "$leaf_projection_tmp" "$(cat <<'EOF'
SKILL.md
agents/openai.yaml
EOF
)"

mkdir -p "$OUTPUT_ROOT"
rm -rf "$OUTPUT_ROOT/handbook" "$OUTPUT_ROOT/handbook-charter-intake"
cp -R "$root_projection_tmp" "$OUTPUT_ROOT/handbook"
cp -R "$leaf_projection_tmp" "$OUTPUT_ROOT/handbook-charter-intake"
