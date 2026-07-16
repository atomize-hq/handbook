# HCM-1.1 Review 4 Remediation Proof Wall

**Captured:** 2026-07-16T21:10:02Z  
**Phase / slice:** `HCM-1` / `HCM-1.1`  
**Entry handoff:** `20260716T181353Z--HCM-0-7--orchestration--implementation-program-approved`  
**Baseline HEAD:** `87bc3ab3e175aa1d6c058ac0ed57495989fe5451`  
**Implementation lineage HEAD before remediation:** `459f422`  
**Active packet:** `docs/specs/handbook-contract-membrane/slices/HCM-1.1`

## Review 4 result and disposition

Fresh Final Registry Review 4 used the immutable 35-entry dispatch at
`docs/specs/handbook-contract-membrane/handoffs/dispatches/20260716T204651Z--HCM-1-1--fresh-final-registry-review-4.json`
(dispatch-byte SHA-256
`cc5516f42bc8135c439757ff4cacb074f2f060111c401796dce55673de57b804`).
Reviewer `/root/hcm_1_1_final_review_4` replayed every entry and reproduced
`sha256:d23f2d0a219d34d4a35e8a2a27c5beac8d82812f7b5b755d59db1747bc0db26b`.
The reviewer made no repository edits and returned `CHANGES_REQUIRED` with two
Required findings:

1. dynamic `RegistryLoadError.location` values still echoed unbounded authored
   source paths and exact schema refs; and
2. the Unix descriptor reader applied `O_NOFOLLOW` to the explicitly selected
   repository root, regressing canonical product loads when that root was
   selected through a directory symlink.

The two repeated Optional findings remain unchanged and non-blocking: unknown-
kind validation still uses an empty error vector and the bounded reference
 graph still performs a quadratic lookup.

The parent accepted and remediated both Required findings. Earlier proof walls
remain immutable lineage; this file supersedes their error-location and
selected-root claims.

## RED and impact evidence

The parent added three focused regression probes before behavior repair:

- a source path containing 10,000 redundant `./` components and an authored
  `SECRET_PATH_SENTINEL` location;
- a valid missing exact schema ref containing 100,000 bytes of SemVer build
  metadata and `SECRET-SCHEMA-REF`; and
- a canonical product load through a symlink-selected repository root, paired
  with an intermediate relative artifact-component symlink refusal.

All three failed for the reviewer-reported reasons. The 68-line RED stream is
`/tmp/hcm-1.1-review4-red.log`, SHA-256
`2e130304f0024eeacfbdece371adf096c19654d17df0b8475d97e5fb3512f059`.

GitNexus rated the shared `RegistryLoadError::at` constructor change **HIGH**:
16 direct callers, 27 total impacted symbols, one affected execution process,
and three affected modules. The user was warned before the edit. The
`open_repo_relative_regular_file_with_hook` change was LOW with two direct
dependents; both edited existing test symbols were isolated and LOW. The high-
risk constructor edit is therefore paired with focused disclosure probes and
complete engine, workspace, package, and cross-target replay.

## Bounded remediation

### Stable bounded error locations

`RegistryLoadError::at` now applies a 240-byte maximum, control-character
refusal, and Unix/Windows absolute-location refusal, replacing unsafe values
with the stable `registry_location` category. This is a fail-safe outer bound
for every current and future caller.

The audited HCM-1.1 exact-value sites also now use stable field locations rather
than relying on the outer bound:

- schema and artifact-kind source normalization;
- duplicate schema and kind exact identities;
- schema document, document-fingerprint, closure-fingerprint, and entry-
  fingerprint mismatches; and
- a missing canonical schema selection.

Both long authored sentinels are absent from `location` and `Display`; each
location is below 256 bytes and each rendered error is below 512 bytes. Existing
absolute-path, parser, decoder, meta-schema, regex, reference, and structural-
location disclosure tests remain green.

### Explicit root trust anchor, relative no-follow boundary

On Unix, the caller-selected repository root is now opened with
`RDONLY|CLOEXEC|DIRECTORY` and treated as the explicit trust anchor. Every
repo-relative intermediate component still uses
`RDONLY|CLOEXEC|DIRECTORY|NOFOLLOW`, and the final component still uses
`RDONLY|CLOEXEC|NOFOLLOW|NONBLOCK`, retained-handle regularity verification, and
bounded reads.

The canonical product regression proves loads through the real root and its
symlink-selected alias are byte-identical. A separate fixture in the same test
places a symlink at `.handbook/charter`; that relative component remains typed
and reported as `CanonicalArtifactSymlinkNotAllowed`. Existing final/intermediate
symlink, FIFO, directory, ordinary non-directory, and substitution-race tests
remain green.

## Complete post-remediation replay

All commands below exited `0` in one fail-fast run. The raw 1,956-line stream is
`/tmp/hcm-1.1-review4-remediation-proof.log`, with SHA-256
`6e6dfabefe28328fea742c484e434cccdc9d431b60485ffc54f3ea3ebfbdb00e`.

| Command | Result |
|---|---|
| `cargo fmt --all -- --check` | PASS |
| `cargo clippy -p handbook-engine --all-targets -- -D warnings` | PASS |
| `cargo test -p handbook-engine` | PASS; 74 tests plus doc tests, 0 failed |
| `cargo test --workspace` | PASS; every workspace suite and doc test, 0 failed |
| `cargo check -p handbook-engine --tests --target x86_64-pc-windows-gnu` | PASS; non-Unix implementation and test branches compile |
| `cargo tree -p handbook-engine -e features` | PASS; `rustix 1.1.4` with `fs`; no HTTP/TLS/async/file resolver feature path |
| `cargo package -p handbook-engine --allow-dirty --no-verify` | PASS; 39 files, 449.7 KiB uncompressed, 83.6 KiB compressed |
| exact package member checks | PASS; literal set equality for both stable-role assets and all eight custom-kind fixtures |
| `python3 tools/check_archive_boundary.py --self-test` | PASS; injected runtime archive reference rejected |
| `python3 tools/check_archive_boundary.py` | PASS; current supported roots clean |
| `python3 docs/specs/handbook-contract-membrane/handoffs/validate_handoffs.py` | PASS; 34 records, 105 current dispatches, 34 ledger entries |
| `python3 docs/specs/handbook-contract-membrane/handoffs/validate_handoffs.py --self-test-v1-admission` | PASS |
| `python3 docs/specs/handbook-contract-membrane/handoffs/validate_handoffs.py --self-test-orchestration-contract` | PASS |
| `git diff --check` | PASS |

The package archive contains exactly the required stable-role assets:

- `handbook-engine-0.1.1/definitions/stable-roles/handbook.roles.core/1.0.0.yaml`
- `handbook-engine-0.1.1/definitions/stable-roles/handbook.roles.core/1.1.0.yaml`

It also contains exactly the eight literal members under
`handbook-engine-0.1.1/tests/fixtures/hcm_1_1_custom_kind/`.

## Scope and next gate

The remediation is confined to the HCM-1.1 registry error/source boundaries,
the shared repo-reader's treatment of the explicitly selected root, one
canonical-product regression test, focused registry tests, and packet/proof
clarification. No `CanonicalArtifactKind` variant, product callsite, layout,
setup, doctor, flow, compiler, CLI, SDK, Tauri, Substrate, profile, instance,
intake, renderer, lifecycle, Projection, publication, sibling gate, or HCM-1.2+
byte changed.

`PG-KIND-01` and `PG-KIND-02` remain open. The classification ceiling remains
`BoundaryLanded` for the owner-library `Artifact kind/schema registry` seam.

This proof is not approval. A fifth immutable complete-subject dispatch must
bind every current byte, all four prior dispatches, and all five proof walls. A
fifth different fresh isolated read-only built-in `default` reviewer must
return `CLEAN`. No subject byte may change after that result, and no HCM-1.2
work is authorized.
