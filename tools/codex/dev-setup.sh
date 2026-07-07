#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ROOT_DIR="$(cd "$SCRIPT_DIR/../.." && pwd)"
GENERATE_SCRIPT="$SCRIPT_DIR/generate.sh"
GENERATED_ROOT="$ROOT_DIR/.agents/skills"
CODEX_INSTALL_ROOT="$HOME/.codex/skills"
LEGACY_AGENTS_INSTALL_ROOT="$HOME/.agents/skills"

DISCOVERY_NAME="handbook-charter-intake"
RUNTIME_NAME="handbook"

DISCOVERY_SOURCE="$GENERATED_ROOT/$DISCOVERY_NAME"
RUNTIME_SOURCE="$GENERATED_ROOT/$RUNTIME_NAME"
CODEX_DISCOVERY_DEST="$CODEX_INSTALL_ROOT/$DISCOVERY_NAME"
CODEX_RUNTIME_DEST="$CODEX_INSTALL_ROOT/$RUNTIME_NAME"

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

install_symlink() {
  local source_path="$1"
  local dest_path="$2"

  rm -rf "$dest_path"
  ln -s "$source_path" "$dest_path"
}

require_file "$GENERATE_SCRIPT"
bash "$GENERATE_SCRIPT" --repo-local

require_directory "$DISCOVERY_SOURCE"
require_directory "$RUNTIME_SOURCE"

mkdir -p "$CODEX_INSTALL_ROOT"

install_symlink "$DISCOVERY_SOURCE" "$CODEX_DISCOVERY_DEST"
install_symlink "$RUNTIME_SOURCE" "$CODEX_RUNTIME_DEST"
rm -rf \
  "$LEGACY_AGENTS_INSTALL_ROOT/$DISCOVERY_NAME" \
  "$LEGACY_AGENTS_INSTALL_ROOT/$RUNTIME_NAME"
