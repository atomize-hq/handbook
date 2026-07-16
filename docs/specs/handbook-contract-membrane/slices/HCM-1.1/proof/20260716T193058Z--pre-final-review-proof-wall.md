# HCM-1.1 Pre-Final-Review Proof Wall

**Captured:** 2026-07-16T19:30:58Z  
**Phase / slice:** `HCM-1` / `HCM-1.1`  
**Entry handoff:** `20260716T181353Z--HCM-0-7--orchestration--implementation-program-approved`  
**Baseline HEAD:** `87bc3ab3e175aa1d6c058ac0ed57495989fe5451`  
**Implementation HEAD:** `459f422`  
**Active packet:** `docs/specs/handbook-contract-membrane/slices/HCM-1.1`

## Result

The implementation supplies the additive `handbook-engine` identity,
stable-role, safe local Draft 2020-12 schema-registry, capability-free artifact-
kind registry, and repository-defined custom-kind proof boundary authorized by
HCM-1.1. It does not change the fixed `CanonicalArtifactKind`, canonical layout,
setup, doctor, flow, compiler, CLI, SDK, Tauri, Substrate, profile, instance,
intake, renderer, lifecycle, Projection, publication, or downstream paths.

The intended promotion is exactly `BoundaryLanded` for the owner-library
`Artifact kind/schema registry` seam. `PG-KIND-01` remains open for lifecycle
and Projection coverage. `PG-KIND-02` remains open for supplied-intake coverage.
All sibling gates and product-path classifications remain unchanged.

## Incremental implementation lineage

| Commit | Green increment |
|---|---|
| `f2048a1` | exact definition identity, canonical SemVer, RFC 8785/SHA-256 fingerprinting, duplicate-key and source-budget primitives |
| `d329304` | package-owned stable-role registries, frozen fingerprint replay, and trusted no-follow source access |
| `0de9990` | bounded local Draft 2020-12 schema registry and deterministic structural validation |
| `3034b83` | capability-free artifact-kind meta-validation, exact dependency closure, and fail-closed later-owned fields |
| `459f422` | two-schema/two-kind repository-defined custom-kind fixture and public-API end-to-end proof |

Every increment followed failing focused tests first, then focused/affected green
proof, formatting/lint, scoped inspection, staged GitNexus change detection, and
an atomic commit.

## Focused positive and negative proof

`cargo test -p handbook-engine` passed 65 unit/integration tests plus doc tests.
The focused suites prove:

- exact-ref grammar, full canonical SemVer, valid/invalid fingerprint syntax,
  RFC 8785 normalization, duplicate YAML/JSON keys, and per-document/aggregate
  byte limits;
- exact package-owned `handbook.roles.core@1.0.0` and `@1.1.0` selection and
  frozen fingerprint replay, including unknown field, invalid category,
  duplicate, substitution, missing, directory, symlink, and traversal refusal;
- exact schema-entry/document/closure/registry fingerprints, explicit allowed
  roots, no-follow reads, real local `$ref` closure, exact root Draft 2020-12,
  in-memory validator construction, deterministic locations, boolean
  subschemas, and source-order invariance;
- remote/file/data/unknown-scheme, query, backslash, encoded/raw traversal,
  missing, cycle, over-depth, unsafe pointer, alias, identifier, anchor,
  dynamic/recursive ref, nested dialect, unknown keyword/annotation, format,
  fingerprint drift, duplicate/conflicting identity, and prewalk/validator
  target mismatch refusal;
- exact capability-free kind loading and lookup, known/duplicate/unknown role
  handling, stable-role/schema mismatch, exact structural profile, deterministic
  kind-registry fingerprints, duplicate/conflicting kind identity in both
  orders, safe source reads, and structural validation through the selected
  exact schema; and
- refusal before fingerprinting of every non-empty semantic capability,
  required capability, semantic validator, renderer, lifecycle policy, review
  trigger, Projection ref, extension, opaque producer, forged digest,
  changed-byte, wrong dependency kind, incompatible binding-shaped semantic
  input, wrong-record, instance, setup, and intake field exercised by the
  focused negative matrix.

The custom-kind proof uses two distinct schema entries, two distinct kinds, a
real two-document local closure, one valid YAML instance, and a deterministic
invalid instance. Forward and reverse accepted source permutations have equal
schema/kind registry fingerprints, exact lookup sets, closure membership,
per-definition fingerprints, and validation behavior. The current product enum
remains its exact four variants.

## Required replay commands and raw results

All commands below exited `0` in one fail-fast run captured at the timestamp
above. The command/result stream contained 2,026 lines and had SHA-256
`ac17b7ddbdd239c85ab7fe86cd69ccfbe1c0ebcc782c545fd8101965e5604628` at
capture; this file preserves the replay commands and proof-relevant result
facts rather than machine-specific compiler paths.

| Command | Result |
|---|---|
| `cargo fmt --all -- --check` | PASS |
| `cargo clippy -p handbook-engine --all-targets -- -D warnings` | PASS |
| `cargo test -p handbook-engine` | PASS; 65 tests, 0 failed |
| `cargo test --workspace` | PASS; every workspace suite and doc test, 0 failed |
| `cargo tree -p handbook-engine -e features` | PASS; inspected exact engine feature graph |
| `cargo package -p handbook-engine --allow-dirty --no-verify` | PASS; 39 files, 406.4 KiB uncompressed, 76.9 KiB compressed |
| `python3 tools/check_archive_boundary.py --self-test` | PASS; injected runtime `archived/` reference rejected |
| `python3 tools/check_archive_boundary.py` | PASS; no supported-runtime archive references |
| `python3 docs/specs/handbook-contract-membrane/handoffs/validate_handoffs.py` | PASS; 3 record schemas, 2 dispatch schemas, 2 templates, 34 records, 101 current dispatches, 8 admitted legacy dispatches, 34 ledger entries |
| `python3 docs/specs/handbook-contract-membrane/handoffs/validate_handoffs.py --self-test-v1-admission` | PASS; unknown/modified/deleted historical bytes and dispatches reject; rebuilt ledger is exact |
| `python3 docs/specs/handbook-contract-membrane/handoffs/validate_handoffs.py --self-test-orchestration-contract` | PASS; invalid agent/stop/status/resume/fingerprint/remediation/closeout cases fail closed and valid review/remediation/two-commit lineages admit |
| `git diff --check` | PASS |

The immutable review dispatch increments the current-dispatch count after this
capture. All validator modes are required to rerun after dispatch creation,
after final clean byte replay, and after mechanical closeout.

## Dependency and package assertions

The pre-review engine manifest pinned the four initially authorized direct
dependencies: `jsonschema = 0.47.0` with `default-features = false`,
`semver = 1`, `serde_json = 1`, and `serde_json_canonicalizer = 0.3.2`.
Fresh Final Review 1 later proved that the retained `libc`-based path-wide
metadata/final-open sequence was raceable; this proof is therefore superseded
by the post-remediation proof and does not claim final no-follow correctness.

The feature-tree scan found no `reqwest`, `hyper`, `rustls`, `tokio`,
`resolve-http`, `resolve-file`, `resolve-async`, custom keyword, custom format,
or executable resolver path. The schema registry builds only explicit in-memory
resources and requests `PatternOptions::regex()`.

The package archive contains both exact assets:

- `handbook-engine-0.1.1/definitions/stable-roles/handbook.roles.core/1.0.0.yaml`
- `handbook-engine-0.1.1/definitions/stable-roles/handbook.roles.core/1.1.0.yaml`

It also contains the four new owner modules, focused registry/identity tests,
and the complete custom-kind fixture corpus. No publication claim is made.

## Scope and preserved baseline

The implementation changed only the packet-authorized Cargo/lock, additive
engine owner modules, the smallest public export/shared-budget plumbing, exact
package assets, focused engine tests/fixtures, HCM-1.1 task/proof evidence, the
single seam crosswalk row, and bounded `PG-KIND-01`/`PG-KIND-02` evidence.

No current product-path module, sibling crate, `CanonicalArtifactKind`,
canonical path/layout, setup, doctor, flow, compiler, CLI, SDK, Tauri,
Substrate, intake, renderer, Projection, shipped-profile/default-instance,
canonical YAML, or HCM-1.2+ packet byte changed. `git diff --name-only` from the
entry baseline and the final review manifest are the authoritative literal path
sets.

GitNexus impact checks were run before each existing symbol edit. Those edits
were LOW risk. Staged detection on the registry increments reported only the
expected additive registry flows (maximum staged risk `medium`, with no
HIGH/CRITICAL warning). Staged detection must run again over the final selected
subject before the primary commit and over the mechanical closeout before its
separate commit.

## Final review gate

This proof records no reviewer conclusion. The next immutable dispatch must bind
every implementation, asset, fixture, packet-status, crosswalk, gate-evidence,
and proof byte using `repo-path-null-sha256-newline-v1`. A fresh isolated read-
only built-in `default` reviewer must report findings first. Any validated
Critical or Required finding requires bounded parent remediation, full proof
replay, a new complete manifest, and a different fresh reviewer. No reviewed
subject byte may change after final `CLEAN`.
