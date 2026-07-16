# HCM-1.1 Review 3 Remediation Proof Wall

**Captured:** 2026-07-16T20:45:35Z  
**Phase / slice:** `HCM-1` / `HCM-1.1`  
**Entry handoff:** `20260716T181353Z--HCM-0-7--orchestration--implementation-program-approved`  
**Baseline HEAD:** `87bc3ab3e175aa1d6c058ac0ed57495989fe5451`  
**Implementation lineage HEAD before remediation:** `459f422`  
**Active packet:** `docs/specs/handbook-contract-membrane/slices/HCM-1.1`

## Review 3 result and disposition

Fresh Final Registry Review 3 used the immutable 33-entry dispatch at
`docs/specs/handbook-contract-membrane/handoffs/dispatches/20260716T202257Z--HCM-1-1--fresh-final-registry-review-3.json`
(dispatch-byte SHA-256
`be97a36c4d9056b52d9f29290eb4a8d79dae788b23f6d06c8e1eacfc89445860`).
Reviewer `/root/hcm_1_1_final_review_3` replayed every entry and reproduced
`sha256:498ff99d0844330e6b5216ee870595e88d14b04041a4a75e008803456f157f3e`.
The reviewer made no repository edits and returned `CHANGES_REQUIRED` with
four Required findings:

1. parser and closed-type decoder display strings could disclose large
   duplicate keys, unknown fields, or unknown variants;
2. a final FIFO opened read-only in blocking mode before the retained-handle
   regular-file check;
3. macOS reports an intermediate `NOFOLLOW|DIRECTORY` symlink as `ENOTDIR`,
   which was incorrectly classified as an ordinary non-directory; and
4. the non-Unix strict-reader fallback was shared with the existing canonical-
   artifact path, turning a fail-closed registry choice into a product-path
   panic.

The two previously reported Optional findings remain unchanged and
non-blocking: unknown-kind validation still uses an empty error vector and the
bounded reference graph still performs a quadratic lookup.

The parent accepted all four Required findings. Earlier proof walls remain
immutable lineage; this file supersedes their error-boundary and portability
claims.

## RED and impact evidence

Review 3 independently reproduced all four defects with read-only external
probes, including a 50,000-byte duplicate-key disclosure, a 500-byte
unknown-field disclosure, a FIFO blocked beyond one second, the macOS
intermediate-symlink discriminant, and a compiling Windows target reaching the
regressed shared path.

The parent added focused tests before behavior repair. The initial 141-line
stream at `/tmp/hcm-1.1-review3-red.log`, SHA-256
`638f26ac168da5856f0fe24cbccba122b77ddad1e3069c80ac748ad299a99b0b`,
records the stable-role/schema/kind secret disclosures. It also exposed two
test-harness corrections before the filesystem test could execute: a 50,000-
byte YAML simple key exceeded YAML's key syntax rather than reaching duplicate
classification, and `rustix::fs::mkfifoat` is not exported on Apple targets.
The YAML sentinel was reduced to 500 bytes while the JSON sentinel remains
50,000 bytes; the FIFO fixture now uses the platform `mkfifo` utility. The
missing Windows standard-library target was installed before the bounded
cross-target compile. These are recorded as test-harness corrections, not
product fixes or fabricated RED results.

GitNexus impact analysis was LOW for every edited symbol. The widest effects
were `CanonicalWorkspace::trusted_read` with 12 impacted symbols and one schema-
load process family, and `fingerprint_serializable` with eight test-facing
dependents. Each decoder classifier and descriptor helper was otherwise
isolated or had at most two direct callers.

## Bounded remediation

### Stable bounded errors

Duplicate-key detection still uses the typed `DuplicateKey` discriminant, but
the visitor and parser boundary no longer copy authored keys into outward
errors. YAML/JSON syntax errors, stable-role/schema-entry/kind closed-type
errors, canonicalization errors, and typed serialization errors now use short
static categories. Invalid identities already used static categories.

Focused sentinel cases cover:

- 500-byte duplicate YAML and 50,000-byte duplicate JSON keys;
- 500-byte stable-role, schema-entry, and artifact-kind unknown fields;
- a 500-byte unknown stable-role category; and
- a 1,000-byte malformed exact-definition identity.

For every case, the typed discriminant remains exact while `detail`, optional
`location`, and `Display` contain no sentinel and remain under the asserted
bound. Invalid/duplicate role locations and supported-role locations use stable
field categories rather than authored values. Structural validation locations
also fall back to their safe document/root location if an instance or schema
pointer exceeds 512 bytes.

### Nonblocking retained handles and exact typing

The final Unix descriptor-relative open now adds `NONBLOCK` to
`RDONLY|CLOEXEC|NOFOLLOW`. Retained-handle metadata rejects FIFO and other
non-regular sources before any read. The focused FIFO test uses a 250 ms
deadline and would release a blocked pre-fix reader before failing, so it does
not hang the suite.

When an intermediate open returns `ENOTDIR`, the retained parent descriptor is
used for `statat(..., SYMLINK_NOFOLLOW)`. A symlink becomes typed
`SymlinkSource`; an ordinary intermediate file remains typed
`NonRegularSource`. The existing final-symlink and component-substitution tests
remain green.

### Strict registry versus legacy product path

The strict registry reader is now an explicit sibling of the legacy canonical-
product reader. On Unix both use the strengthened retained descriptor path. On
non-Unix, registry loads fail closed because equivalent descriptor-relative
no-follow traversal is unavailable, while the canonical-product reader retains
its pre-HCM-1.1 normalized component-check/open behavior rather than reaching
the existing `unreachable!` branch.

A `cfg(not(unix))` test encodes both expectations. This run did not execute a
Windows binary; it compiled the engine and all engine tests for
`x86_64-pc-windows-gnu`, proving the non-Unix branches and test contract type-
check. Native Unix engine/workspace tests prove the strengthened registry path.

## Complete post-remediation replay

All commands below exited `0` in one fail-fast run. The raw 1,965-line stream is
`/tmp/hcm-1.1-review3-remediation-proof.log`, with SHA-256
`3ac0dc3932bd7f2971bc4c569571c590632a0327f5aa48d80f7670db2c604e38`.

| Command | Result |
|---|---|
| `cargo fmt --all -- --check` | PASS |
| `cargo clippy -p handbook-engine --all-targets -- -D warnings` | PASS |
| `cargo test -p handbook-engine` | PASS; 71 tests plus doc tests, 0 failed |
| `cargo test --workspace` | PASS; every workspace suite and doc test, 0 failed |
| `cargo check -p handbook-engine --tests --target x86_64-pc-windows-gnu` | PASS; non-Unix implementation and test branches compile |
| `cargo tree -p handbook-engine -e features` | PASS; `rustix 1.1.4` with `fs`; no HTTP/TLS/async/file resolver feature path |
| `cargo package -p handbook-engine --allow-dirty --no-verify` | PASS; 39 files, 445.0 KiB uncompressed, 82.7 KiB compressed |
| exact package member checks | PASS; both stable-role assets and the complete custom-kind fixture corpus present |
| `python3 tools/check_archive_boundary.py --self-test` | PASS; injected runtime archive reference rejected |
| `python3 tools/check_archive_boundary.py` | PASS; current supported roots clean |
| `python3 docs/specs/handbook-contract-membrane/handoffs/validate_handoffs.py` | PASS; 34 records, 104 current dispatches, 34 ledger entries |
| `python3 docs/specs/handbook-contract-membrane/handoffs/validate_handoffs.py --self-test-v1-admission` | PASS |
| `python3 docs/specs/handbook-contract-membrane/handoffs/validate_handoffs.py --self-test-orchestration-contract` | PASS |
| `git diff --check` | PASS |

The package archive still contains exactly the required stable-role assets:

- `handbook-engine-0.1.1/definitions/stable-roles/handbook.roles.core/1.0.0.yaml`
- `handbook-engine-0.1.1/definitions/stable-roles/handbook.roles.core/1.1.0.yaml`

## Scope and next gate

The remediation is confined to the HCM-1.1 error and source-access boundaries,
focused tests, the packet's exact portability clarification, and this proof.
No canonical product callsite or product-contract module was edited; the shared
repo-support module preserves its legacy non-Unix product path. The fixed `CanonicalArtifactKind`,
canonical layout, setup, doctor, flow, compiler, CLI, SDK, Tauri, Substrate,
profiles, instances, intake, rendering, lifecycle, Projection, publication,
sibling gates, and HCM-1.2+ bytes remain unchanged.

`PG-KIND-01` and `PG-KIND-02` remain open. The classification ceiling remains
`BoundaryLanded` for the owner-library `Artifact kind/schema registry` seam.

This proof is not approval. A fourth immutable complete-subject dispatch must
bind every current byte, all three prior dispatches, and all four proof walls.
A fourth different fresh isolated read-only built-in `default` reviewer must
return `CLEAN`. No subject byte may change after that result, and no HCM-1.2
work is authorized.
