# HCM-3.1 implementation discovery remediation

Timestamp: `2026-08-01T13:38:53Z`

Parent orchestration: `20260801T122747Z--HCM-3-1--vocabulary-resolution`

Predecessor dispatch: `20260801T132625Z--HCM-3-1--vocabulary-resolution-implementation-review`

Predecessor subject: `sha256:7e03ef62442187febff0a8c28fc2f77fedd2141b6e1aa4e73b7d3ff8e42af708`

## Consolidated disposition

The fresh implementation reviewer returned three P2 proof gaps and no P1,
P3, or P4 finding. The parent remediated all three in the existing
`crates/engine/tests/vocabulary_registry.rs` ceiling without changing a
production symbol, fixture, shipped definition, dependency, or public surface.

| Finding | Disposition | Added proof |
|---|---|---|
| `HCM31-IMP-DISC-001` | remediated | one contract-derived normalization matrix covers Unicode whitespace collapse, Unicode scalar lowercase expansion (`İ` -> `i` plus combining dot), and explicit refusal to fold punctuation, accents, or compatibility characters |
| `HCM31-IMP-DISC-002` | remediated | an absorption whose `delivery_unit` owns itself is fingerprint-valid and refuses with `DependencyCycle` |
| `HCM31-IMP-DISC-003` | remediated | the shipped empty vocabulary asserts exact fingerprint `sha256:69113b1a9271ce207d45bdb91ebae8d6516249e16b59891b292546078364a22b` |

GitNexus could not index any of the three edited Rust test functions. Upstream
impact queries returned `UNKNOWN`, zero discovered callers, and zero discovered
processes; FTS remained unavailable. The CLEAN selector bounded the edit to the
three existing test bodies, and no production symbol was changed.

## Post-remediation convergence

- `cargo test -p handbook-engine --test vocabulary_registry`: 7/7 PASS.
- `cargo test -p handbook-engine --test profile_selection`: 20/20 PASS.
- `cargo test -p handbook-pipeline --test pipeline_capture`: 48/48 PASS.
- `cargo clippy -p handbook-engine --test vocabulary_registry -- -D warnings`: PASS.
- `cargo fmt --all -- --check`: PASS after mechanical formatting.
- `git diff --check`: PASS.

The test-file SHA-256 after remediation is
`04832d5c80e73a7810f8eabb38df793f31d649bbeb0f9b36c41269bdcd708ee5`.
The shipped vocabulary bytes remain unchanged.

## Remaining gate

A different fresh read-only implementation closure reviewer must replay the
complete remediated aggregate, close all three predecessor finding IDs, and
return CLEAN before the separately typed proof stage may begin.
