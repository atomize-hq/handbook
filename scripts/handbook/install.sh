#!/usr/bin/env bash
set -euo pipefail

latest_release_tag() {
  local latest_url
  latest_url="$(curl -fsSL -o /dev/null -w '%{url_effective}' "https://github.com/atomize-hq/handbook/releases/latest" || true)"
  if [[ "$latest_url" =~ /tag/([^/]+)$ ]]; then
    printf '%s\n' "${BASH_REMATCH[1]}"
    return 0
  fi
  return 1
}

usage() {
  cat <<'EOF'
Usage: install.sh [--version <version-or-tag>]

Install the latest tagged handbook release into ~/handbook and refresh agent discovery links.

Options:
  --version <v>  Install a specific release version or tag, for example 0.7.1.0 or v0.7.1.0
EOF
}

log() {
  printf '[handbook-install] %s\n' "$*"
}

fatal() {
  printf '[handbook-install][ERROR] %s\n' "$*" >&2
  exit 1
}

download_file() {
  local url="$1"
  local dest="$2"
  curl -fsSL "$url" -o "$dest"
}

compute_sha256() {
  local path="$1"
  python3 - "$path" <<'PY'
import hashlib
import pathlib
import sys

path = pathlib.Path(sys.argv[1])
digest = hashlib.sha256()
with path.open("rb") as handle:
    for chunk in iter(lambda: handle.read(1024 * 1024), b""):
        digest.update(chunk)
print(digest.hexdigest())
PY
}

verify_checksum() {
  local archive_path="$1"
  local checksums_path="$2"
  local asset_name="$3"
  local expected
  local actual

  expected="$(grep "  ${asset_name}$" "$checksums_path" | awk '{print $1}' || true)"
  [[ -n "$expected" ]] || fatal "checksum entry for ${asset_name} not found in ${checksums_path}"

  actual="$(compute_sha256 "$archive_path")"
  [[ "$expected" == "$actual" ]] || {
    fatal "checksum mismatch for ${asset_name}: expected ${expected}, got ${actual}"
  }
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

target_label() {
  local os arch
  os="$(uname -s)"
  arch="$(uname -m)"

  case "$os:$arch" in
    Darwin:arm64|Darwin:aarch64)
      printf 'macos_arm64\n'
      ;;
    Linux:x86_64|Linux:amd64)
      printf 'linux_x86_64\n'
      ;;
    *)
      fatal "unsupported platform for release install: ${os} ${arch}. Supported targets are macOS arm64 and Linux x86_64."
      ;;
  esac
}

RELEASE_TAG="${HANDBOOK_INSTALL_REF:-}"

while [[ $# -gt 0 ]]; do
  case "$1" in
    --version)
      RELEASE_TAG="${2:-}"
      shift 2
      ;;
    -h|--help)
      usage
      exit 0
      ;;
    *)
      usage
      fatal "unknown argument: $1"
      ;;
  esac
done

if [[ -z "$RELEASE_TAG" ]]; then
  RELEASE_TAG="$(latest_release_tag)" || fatal "unable to resolve latest handbook release tag"
fi

if [[ "$RELEASE_TAG" != v* ]]; then
  RELEASE_TAG="v${RELEASE_TAG}"
fi

VERSION="${RELEASE_TAG#v}"
LABEL="$(target_label)"
ASSET_NAME="handbook-${RELEASE_TAG}-${LABEL}.tar.gz"
BASE_URL="${HANDBOOK_INSTALL_BASE_URL:-https://github.com/atomize-hq/handbook/releases/download}"
ARCHIVE_URL="${BASE_URL}/${RELEASE_TAG}/${ASSET_NAME}"
CHECKSUMS_URL="${BASE_URL}/${RELEASE_TAG}/SHA256SUMS"

tmp_root="$(mktemp -d "${TMPDIR:-/tmp}/handbook-install.XXXXXX")"
cleanup() {
  rm -rf "$tmp_root"
}
trap cleanup EXIT

archive_path="$tmp_root/$ASSET_NAME"
checksums_path="$tmp_root/SHA256SUMS"
extract_root="$tmp_root/extracted"
handbook_home="$HOME/handbook"
codex_root="$HOME/.codex/skills"
legacy_agents_root="$HOME/.agents/skills"

mkdir -p "$HOME"
mkdir -p "$extract_root"

log "Downloading ${ASSET_NAME}"
download_file "$ARCHIVE_URL" "$archive_path"
log "Downloading SHA256SUMS"
download_file "$CHECKSUMS_URL" "$checksums_path"
verify_checksum "$archive_path" "$checksums_path" "$ASSET_NAME"

log "Extracting release bundle"
tar -xzf "$archive_path" -C "$extract_root"
[[ -d "$extract_root/handbook" ]] || fatal "release bundle did not contain a top-level handbook/ directory"

rm -rf "$handbook_home"
mv "$extract_root/handbook" "$handbook_home"

[[ -x "$handbook_home/bin/handbook" ]] || fatal "installed bundle missing ~/handbook/bin/handbook"
[[ -f "$handbook_home/runtime-manifest.json" ]] || fatal "installed bundle missing ~/handbook/runtime-manifest.json"

mkdir -p "$codex_root"
install_discovery_entry "$handbook_home/.agents/skills/handbook" "$codex_root/handbook"
install_discovery_entry "$handbook_home/.agents/skills/handbook-charter-intake" "$codex_root/handbook-charter-intake"
rm -rf \
  "$legacy_agents_root/handbook" \
  "$legacy_agents_root/handbook-charter-intake"

log "Installed handbook ${VERSION} to ${handbook_home}"
log "Codex discovery links refreshed under ${codex_root}"
log "Optional PATH entry: ${handbook_home}/bin"
