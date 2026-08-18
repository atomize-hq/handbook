#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
BINARY_PATH="${1:-}"

if [[ -z "$BINARY_PATH" ]]; then
  for candidate in \
    "$ROOT_DIR/target/debug/handbook" \
    "$ROOT_DIR/target/debug/handbook.exe"; do
    if [[ -x "$candidate" ]]; then
      BINARY_PATH="$candidate"
      break
    fi
  done
fi

[[ -n "$BINARY_PATH" && -x "$BINARY_PATH" ]] || {
  echo "usage: hcm-4-5-packet4-skill-smoke.sh [path-to-handbook-binary]" >&2
  exit 1
}

tmp_root="$(mktemp -d "${TMPDIR:-/tmp}/handbook-packet4-skill-smoke.XXXXXX")"
cleanup() {
  case "$tmp_root" in
    "${TMPDIR:-/tmp}"/handbook-packet4-skill-smoke.*)
      rm -rf "$tmp_root"
      ;;
  esac
}
trap cleanup EXIT

safe_home="$tmp_root/home"
safe_bin="$tmp_root/bin"
release_bundle_root="$tmp_root/releases"
public_home="$tmp_root/public-home"
mkdir -p "$safe_home" "$safe_bin" "$release_bundle_root" "$public_home"
cp "$BINARY_PATH" "$safe_bin/handbook"
chmod 0755 "$safe_bin/handbook"

export HOME="$safe_home"
export PATH="$safe_bin:$PATH"

assert_same() {
  cmp "$1" "$2"
}

assert_projection_bytes() {
  local projection_root="$1"

  assert_same \
    "$ROOT_DIR/install/handbook-home/SKILL.md.tmpl" \
    "$projection_root/handbook/SKILL.md"
  assert_same \
    "$ROOT_DIR/install/handbook-home/agents/openai.yaml" \
    "$projection_root/handbook/agents/openai.yaml"
  assert_same \
    "$ROOT_DIR/install/handbook-home/charter-intake/SKILL.md.tmpl" \
    "$projection_root/handbook-charter-intake/SKILL.md"
  assert_same \
    "$ROOT_DIR/install/handbook-home/charter-intake/openai.yaml" \
    "$projection_root/handbook-charter-intake/agents/openai.yaml"
}

cd "$ROOT_DIR"

echo "==> generated projection"
bash tools/codex/generate.sh --repo-local
assert_projection_bytes "$ROOT_DIR/.agents/skills"

echo "==> source install and discovery links"
bash tools/codex/install.sh
assert_same \
  "$ROOT_DIR/install/handbook-home/SKILL.md.tmpl" \
  "$HOME/handbook/SKILL.md"
assert_projection_bytes "$HOME/handbook/.agents/skills"
test -L "$HOME/.codex/skills/handbook"
test -L "$HOME/.codex/skills/handbook-charter-intake"
assert_projection_bytes "$HOME/.codex/skills"

echo "==> reinstall exact file set"
before_listing="$(find "$HOME/handbook" -type f -print | sed "s#^$HOME/handbook/##" | sort)"
bash tools/codex/install.sh
after_listing="$(find "$HOME/handbook" -type f -print | sed "s#^$HOME/handbook/##" | sort)"
[[ "$before_listing" == "$after_listing" ]]
assert_projection_bytes "$HOME/handbook/.agents/skills"

echo "==> development projection links"
bash tools/codex/dev-setup.sh
test -L "$HOME/.codex/skills/handbook"
test -L "$HOME/.codex/skills/handbook-charter-intake"
assert_projection_bytes "$HOME/.codex/skills"

echo "==> installed discovery/schema/read-only invocation"
python3 - "$HOME/handbook/bin/handbook" "$ROOT_DIR" "$tmp_root" <<'PY'
import json
import subprocess
import sys

binary = sys.argv[1]
repo_root = sys.argv[2]
def run(*args, expect_typed=True, input_bytes=None):
    completed = subprocess.run(
        [binary, *args],
        cwd=repo_root,
        check=False,
        capture_output=True,
        input=input_bytes,
    )
    if expect_typed:
        if completed.stderr:
            raise SystemExit(completed.stderr.decode("utf-8", errors="replace"))
        if completed.stdout.count(b"\n") != 1 or not completed.stdout.endswith(b"\n"):
            raise SystemExit("machine action did not emit one JSON value plus LF")
    return completed


bootstrap_result = run(
    "capability",
    "bootstrap",
    "--client-version",
    "1.0.0",
    "--page-size",
    "2",
)
if bootstrap_result.returncode != 0:
    raise SystemExit("bootstrap did not return its typed ok outcome")
bootstrap = json.loads(bootstrap_result.stdout)
if bootstrap.get("outcome_kind") != "ok":
    raise SystemExit("bootstrap outcome was not ok")
response = bootstrap["response"]
page = response["first_capability_page"]
entries = list(page["entries"])
cursor = page["next_cursor"]
while cursor is not None:
    page_result = run(
        "capability",
        "catalog-read",
        "--catalog-kind",
        "capabilities",
        "--page-size",
        "2",
        "--cursor",
        json.dumps(cursor, separators=(",", ":"), sort_keys=True),
    )
    if page_result.returncode != 0:
        raise SystemExit("capability pagination did not return its typed page")
    page_document = json.loads(page_result.stdout)
    if page_document.get("catalog_kind") != "capabilities":
        raise SystemExit("catalog kind changed during pagination")
    page = page_document["response"]
    entries.extend(page["entries"])
    cursor = page["next_cursor"]

selected = None
for entry in entries:
    if entry.get("class") != "read_only":
        continue
    if "cli_json" not in entry.get("declared_transports", []):
        continue
    if "cli_json" not in entry.get("current_transports", []):
        continue
    request_binding = entry["request_schema"]
    schema_result = run(
        "capability",
        "schema-read",
        "--schema-ref",
        request_binding["ref"],
        "--manifest-fingerprint",
        request_binding["fingerprint"],
        expect_typed=False,
    )
    if schema_result.returncode != 0 or schema_result.stderr:
        continue
    schema = json.loads(schema_result.stdout)
    if schema.get("type") == "object" and not schema.get("required"):
        selected = entry
        break

if selected is None:
    raise SystemExit("no discovered read-only capability admitted an empty closed body")

manifest_by_ref = {
    item["schema"]["ref"]: item["schema"] for item in response["schema_manifest"]
}
shared = manifest_by_ref.get("handbook.invocation-response@1.0.0")
if shared is None:
    raise SystemExit("shared invocation response schema is absent from the manifest")
shared_result = run(
    "capability",
    "schema-read",
    "--schema-ref",
    shared["ref"],
    "--manifest-fingerprint",
    shared["fingerprint"],
    expect_typed=False,
)
if shared_result.returncode != 0 or shared_result.stderr:
    raise SystemExit("shared invocation response schema retrieval failed")
json.loads(shared_result.stdout)

invocation_result = run(
    "capability",
    "invoke",
    "--operation-ref",
    selected["operation"]["ref"],
    "--definition-fingerprint",
    selected["operation"]["fingerprint"],
    "--capability-entry-fingerprint",
    selected["entry_fingerprint"],
    "--from-input",
    "-",
    input_bytes=b"{}",
)
invocation = json.loads(invocation_result.stdout)
if invocation.get("response_kind") != "operation":
    raise SystemExit("generic invocation changed response family")
operation_response = invocation["response"]
if operation_response["operation"]["ref"] != selected["operation"]["ref"]:
    raise SystemExit("generic invocation changed discovered operation identity")
if operation_response.get("status") not in {"ok", "blocked", "refused", "error"}:
    raise SystemExit("generic invocation returned an unknown terminal")
PY

echo "==> release package and public install"
version="$(tr -d '[:space:]' < VERSION)"
bundle_dir="$release_bundle_root/v$version"
mkdir -p "$bundle_dir"
bash tools/release/package-handbook-home.sh \
  --binary "$HOME/handbook/bin/handbook" \
  --label linux_x86_64 \
  --version "$version" \
  --output-dir "$bundle_dir" >/dev/null
(
  cd "$bundle_dir"
  shasum -a 256 "handbook-v${version}-linux_x86_64.tar.gz" > SHA256SUMS
)
HOME="$public_home" HANDBOOK_INSTALL_BASE_URL="file://$release_bundle_root" \
  bash scripts/handbook/install.sh --version "$version"
assert_projection_bytes "$public_home/handbook/.agents/skills"
test -L "$public_home/.codex/skills/handbook"
test -L "$public_home/.codex/skills/handbook-charter-intake"
assert_projection_bytes "$public_home/.codex/skills"

echo "OK"
