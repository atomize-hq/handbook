# HCM-1.1 Review 5 Remediation Proof Wall

**Captured:** 2026-07-16T21:29:17Z  
**Phase / slice:** `HCM-1` / `HCM-1.1`  
**Entry handoff:** `20260716T181353Z--HCM-0-7--orchestration--implementation-program-approved`  
**Baseline HEAD:** `87bc3ab3e175aa1d6c058ac0ed57495989fe5451`  
**Implementation lineage HEAD before remediation:** `459f422`  
**Active packet:** `docs/specs/handbook-contract-membrane/slices/HCM-1.1`

## Review 5 result and disposition

Fresh Final Registry Review 5 used the immutable 38-entry dispatch at
`docs/specs/handbook-contract-membrane/handoffs/dispatches/20260716T211109Z--HCM-1-1--fresh-final-registry-review-5.json`
(dispatch-byte SHA-256
`44f6aa0fc98bf1eadfcad58d09b8717c54ebd18c61804d8ae95e331d3edea9e4`).
Reviewer `/root/hcm_1_1_final_review_5` replayed every entry and reproduced
`sha256:7e827a677087cecded04335261a1e8a3a51a8d555b154559fbfb22bc8d409dfb`.
The reviewer made no repository edits and returned `CHANGES_REQUIRED` with two
Required findings:

1. structural validation fell back from an overlong composed location to the
   still-unbounded authored `document_ref`; and
2. normalized repo-relative UTF-8 schema paths containing spaces were admitted
   by the prewalk but could not be registered as raw internal URI strings.

The two repeated Optional findings remain unchanged and non-blocking: unknown-
kind validation still uses an empty error vector and the bounded reference
graph still performs a quadratic lookup.

The parent accepted and remediated both Required findings. Earlier proof walls
remain immutable lineage; this file supersedes their structural-location and
internal-resource path claims.

## RED and impact evidence

The parent added three focused probes before behavior repair:

- a valid normalized document path above 583 bytes containing
  `SECRET_STRUCTURAL_PATH_SENTINEL`, followed by failing instance validation;
- a live two-document closure with `schemas/unicode space/café child.json`,
  including exact decoded reported-location assertions; and
- an exact private-URI matrix for a space, percent-like bytes, and UTF-8,
  including injectivity and reverse mapping.

All three failed at the reviewer-reported boundaries. The 137-line RED stream
is `/tmp/hcm-1.1-review5-red.log`, SHA-256
`393390c4ca30d8220fd99d0a9ebb39abbe54bbe8b802de8e00da169e68725e61`.

GitNexus rated `ResolvedSchema::validate_json` LOW with one direct test-facing
dependent and `internal_resource_uri` LOW with one direct caller.
`build_validator` was isolated and LOW. No HIGH or CRITICAL symbol was edited
for this remediation.

## Bounded remediation

### Structural location ceiling

Every outward structural schema location now passes through one 512-byte
fail-safe bound. Overlong or control-bearing values become the stable
`schema_root` location instead of falling back to an authored document path.
The 583+ byte sentinel path is absent from the returned location, which remains
within the exact ceiling. Short instance/schema locations retain their existing
precise deterministic form.

### Injective internal URI mapping

Each normalized repo-relative path is mapped to `handbook+repo:///...` by
preserving ASCII unreserved bytes and `/` separators and upper-case percent-
encoding every other UTF-8 byte. The mapping is injective: for example, a space
maps to `%20` while authored percent-like bytes map through `%25`, and `café`
round-trips through `caf%C3%A9` without collision.

The prewalk remains authoritative. A validator-only clone translates each
already-prewalked cross-document `$ref` to its exact absolute internal URI;
fragment-only refs remain unchanged. Source bytes, document fingerprints,
closure members, entry fingerprints, and kind fingerprints continue to derive
from the authored documents. The in-memory validator never fetches or reads a
path. Structural absolute locations are decoded through the exact inverse map
back to the original repo-relative UTF-8 path before the 512-byte outward
bound.

The live space/UTF-8 closure loads, has the exact two-member closure, accepts a
valid string, rejects a number, and reports the decoded child path without URI
encoding drift. Existing refusal of authored `%`, query, backslash, scheme,
authority, and traversal reference forms remains green.

## Complete post-remediation replay

All commands below exited `0` in one fail-fast run. The raw 1,962-line stream is
`/tmp/hcm-1.1-review5-remediation-proof.log`, with SHA-256
`00f042e3dd51b31e2cde9e65188a9314e787f235126f73bbdf28455772a3b576`.

| Command | Result |
|---|---|
| `cargo fmt --all -- --check` | PASS |
| `cargo clippy -p handbook-engine --all-targets -- -D warnings` | PASS |
| `cargo test -p handbook-engine` | PASS; 77 tests plus doc tests, 0 failed |
| `cargo test --workspace` | PASS; every workspace suite and doc test, 0 failed |
| `cargo check -p handbook-engine --tests --target x86_64-pc-windows-gnu` | PASS; non-Unix implementation and test branches compile |
| `cargo tree -p handbook-engine -e features` | PASS; `rustix 1.1.4` with `fs`; no HTTP/TLS/async/file resolver feature path |
| `cargo package -p handbook-engine --allow-dirty --no-verify` | PASS; 39 files, 456.3 KiB uncompressed, 85.0 KiB compressed |
| exact package member checks | PASS; literal set equality for both stable-role assets and all eight custom-kind fixtures |
| `python3 tools/check_archive_boundary.py --self-test` | PASS; injected runtime archive reference rejected |
| `python3 tools/check_archive_boundary.py` | PASS; current supported roots clean |
| `python3 docs/specs/handbook-contract-membrane/handoffs/validate_handoffs.py` | PASS; 34 records, 106 current dispatches, 34 ledger entries |
| `python3 docs/specs/handbook-contract-membrane/handoffs/validate_handoffs.py --self-test-v1-admission` | PASS |
| `python3 docs/specs/handbook-contract-membrane/handoffs/validate_handoffs.py --self-test-orchestration-contract` | PASS |
| `git diff --check` | PASS |

The package archive contains exactly the required stable-role assets:

- `handbook-engine-0.1.1/definitions/stable-roles/handbook.roles.core/1.0.0.yaml`
- `handbook-engine-0.1.1/definitions/stable-roles/handbook.roles.core/1.1.0.yaml`

It also contains exactly the eight literal members under
`handbook-engine-0.1.1/tests/fixtures/hcm_1_1_custom_kind/`.

## Scope and next gate

The remediation is confined to HCM-1.1 structural locations, private in-memory
resource identity construction, validator-only reference translation, focused
schema tests, and packet/proof clarification. No `CanonicalArtifactKind`
variant, product callsite, layout, setup, doctor, flow, compiler, CLI, SDK,
Tauri, Substrate, profile, instance, intake, renderer, lifecycle, Projection,
publication, sibling gate, or HCM-1.2+ byte changed.

`PG-KIND-01` and `PG-KIND-02` remain open. The classification ceiling remains
`BoundaryLanded` for the owner-library `Artifact kind/schema registry` seam.

This proof is not approval. A sixth immutable complete-subject dispatch must
bind every current byte, all five prior dispatches, and all six proof walls. A
sixth different fresh isolated read-only built-in `default` reviewer must
return `CLEAN`. No subject byte may change after that result, and no HCM-1.2
work is authorized.
