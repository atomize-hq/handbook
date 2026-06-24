#!/usr/bin/env bash
set -euo pipefail

usage() {
  cat >&2 <<'USAGE'
usage: handbook_pipeline_released_boundary.sh --version <published_version>

Runs the Set 3 released-boundary proof against an exact crates.io version of
handbook-pipeline. This harness intentionally rejects path-based fallback and
must not be treated as satisfied until the requested version exists on crates.io.
USAGE
}

ROOT="$(git rev-parse --show-toplevel)"
FIXTURE_DIR="$ROOT/tests/fixtures/external_consumers/handbook_pipeline_released_boundary"
PROOF_CORPUS_SOURCE="$ROOT/tests/fixtures/pipeline_proof_corpus/foundation_inputs"
PIPELINE_MANIFEST="$ROOT/crates/pipeline/Cargo.toml"
CURRENT_WORKTREE_VERSION="$(sed -n 's/^version = "\([^"]*\)"$/\1/p' "$PIPELINE_MANIFEST" | head -n1)"
PUBLISHED_VERSION=""

while [[ $# -gt 0 ]]; do
  case "$1" in
    --version)
      [[ $# -ge 2 ]] || {
        echo "error: --version requires a value" >&2
        usage
        exit 1
      }
      PUBLISHED_VERSION="$2"
      shift 2
      ;;
    -h|--help)
      usage
      exit 0
      ;;
    *)
      echo "error: unknown argument: $1" >&2
      usage
      exit 1
      ;;
  esac
done

if [[ -z "$PUBLISHED_VERSION" ]]; then
  echo "error: missing required --version <published_version>" >&2
  usage
  exit 1
fi

if [[ ! -d "$FIXTURE_DIR" ]]; then
  echo "error: missing fixture directory $FIXTURE_DIR" >&2
  exit 1
fi

if [[ ! -d "$PROOF_CORPUS_SOURCE" ]]; then
  echo "error: missing proof corpus directory $PROOF_CORPUS_SOURCE" >&2
  exit 1
fi

TMPDIR_ROOT="${TMPDIR:-/tmp}"
WORKDIR="$(mktemp -d "$TMPDIR_ROOT/handbook-pipeline-released-proof.XXXXXX")"
cleanup() {
  rm -rf "$WORKDIR"
}
trap cleanup EXIT

CONSUMER_DIR="$WORKDIR/consumer"
PROOF_CORPUS_DIR="$WORKDIR/proof_corpus"
CARGO_HOME="$WORKDIR/cargo-home"
CARGO_TARGET_DIR="$WORKDIR/target"
METADATA_JSON="$WORKDIR/metadata.json"
export CARGO_HOME
export CARGO_TARGET_DIR
export CARGO_REGISTRIES_CRATES_IO_PROTOCOL="sparse"

mkdir -p "$CONSUMER_DIR" "$PROOF_CORPUS_DIR" "$CARGO_HOME"
cp -R "$FIXTURE_DIR/." "$CONSUMER_DIR/"
cp -R "$PROOF_CORPUS_SOURCE/." "$PROOF_CORPUS_DIR/"

python3 - "$CONSUMER_DIR/Cargo.toml" "$PUBLISHED_VERSION" <<'PY'
from pathlib import Path
import sys
manifest_path = Path(sys.argv[1])
version = sys.argv[2]
contents = manifest_path.read_text()
contents = contents.replace('__HANDBOOK_PIPELINE_VERSION__', version)
manifest_path.write_text(contents)
PY

if grep -Eq '(^|[[:space:]])path[[:space:]]*=' "$CONSUMER_DIR/Cargo.toml"; then
  echo "error: released proof manifest must not contain path dependencies" >&2
  exit 1
fi

if ! cargo metadata --format-version 1 --manifest-path "$CONSUMER_DIR/Cargo.toml" > "$METADATA_JSON"; then
  echo "error: failed to resolve handbook-pipeline =$PUBLISHED_VERSION from crates.io" >&2
  echo "publish the qualifying released version before running this harness" >&2
  exit 1
fi

python3 - "$METADATA_JSON" "$PUBLISHED_VERSION" "$ROOT" <<'PY'
from pathlib import Path
import json
import sys
metadata_path = Path(sys.argv[1])
expected_version = sys.argv[2]
repo_root = Path(sys.argv[3]).resolve()
metadata = json.loads(metadata_path.read_text())
pipeline_packages = [pkg for pkg in metadata['packages'] if pkg['name'] == 'handbook-pipeline']
if len(pipeline_packages) != 1:
    raise SystemExit(f"error: expected exactly one handbook-pipeline package, found {len(pipeline_packages)}")
pkg = pipeline_packages[0]
if pkg['version'] != expected_version:
    raise SystemExit(
        f"error: resolved handbook-pipeline {pkg['version']} instead of expected {expected_version}"
    )
source = pkg.get('source')
if not source or not source.startswith('registry+https://github.com/rust-lang/crates.io-index'):
    raise SystemExit(f"error: handbook-pipeline did not resolve from crates.io registry: {source!r}")
manifest_path = Path(pkg['manifest_path']).resolve()
if repo_root in manifest_path.parents:
    raise SystemExit(
        f"error: handbook-pipeline resolved from source tree path {manifest_path}, expected registry source"
    )
PY

echo "[proof] current worktree version: ${CURRENT_WORKTREE_VERSION}"
echo "[proof] resolved released version: ${PUBLISHED_VERSION}"
echo "[proof] copied external consumer fixture: $CONSUMER_DIR"
echo "[proof] copied proof corpus: $PROOF_CORPUS_DIR"

cargo run --quiet --manifest-path "$CONSUMER_DIR/Cargo.toml" -- "$PROOF_CORPUS_DIR"

echo "[proof] handbook_pipeline_released_boundary: PASS"
