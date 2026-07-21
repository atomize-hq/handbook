# HCM-2.2 exact-result implementation Review 2 remediation

**Proof ID:** `HCM-2.2-ERIR-R2-REMEDIATION`  
**Recorded:** `2026-07-21T18:57:25Z`  
**Authority class:** additive implementation remediation proof; no clean or
landing claim  
**Reviewed subject:**
[`20260721T181400Z--HCM-2-2--fresh-exact-result-implementation-review-2.json`](../../../handoffs/dispatches/20260721T181400Z--HCM-2-2--fresh-exact-result-implementation-review-2.json)

## Disposition

Fresh isolated implementation Review 2 admitted exact 46-path subject
`sha256:d32031c5cb3e68298293e0da03c5103da8b4439c67755d27c05a191b725db13d`
and returned `CHANGES_REQUIRED` with one consolidated Required finding,
`HCM-2.2-R2-001`. It is accepted without waiver. Both earlier review
dispatches and proof files remain byte-immutable evidence.

The reviewed historical candidate validator applied additive candidate `1.1`
field `basis_artifact_fingerprint` to base candidate `1.0`, so a genuine
`1.0` record falsely refused. It also derived an ID from the final fingerprint
but compared only the filename, so a record with a forged internal
`candidate_id` falsely admitted. The test fixture cloned `1.3` into all
historical versions and therefore reproduced the same shape error.

## Test-first repair

Three public author-replay tests were observed RED before production repair:

- `exact_historical_candidate_versions_are_safe_author_replay_nonmatches`
  rejected genuine candidate `1.0` with the exact base shape;
- `historical_candidate_wrong_internal_id_refuses_author_replay_without_mutation`
  returned successful replay for a record whose stored ID disagreed with its
  fingerprint-derived ID; and
- `candidate_v10_with_additive_v11_basis_refuses_author_replay_without_mutation`
  returned successful replay for a `1.0` record carrying the additive `1.1`
  basis field.

All three are GREEN after repair. The fixture now emits:

- candidate `1.0`: exact sixteen-field base shape with no basis field;
- candidate `1.1`: exact seventeen-field additive-basis shape; and
- candidate `1.2`: exact eighteen-field subject/result-ref shape.

The validator dispatches to those distinct closed shapes and validates exact
schema version, definition refs, instance ID, fingerprints, versioned basis,
safe intake/content refs, nonempty exact-closed source-map rows, supported
source kinds, unique target paths, coverage IDs, unique version-appropriate
result refs, final fingerprint, exact stored candidate ID, and candidate `1.2`
subject fingerprint. A 16-case malformed-semantic matrix covers unsafe refs,
bad fingerprints, empty/open/duplicate source maps, unsupported source kinds,
invalid/duplicate coverage and result entries, eligibility/policy drift,
candidate-`1.2` result cardinality, and subject mismatch. Every refusal
preserves bytes and occurs before author mutation.

## Complete proof replay

| Proof | Result |
|---|---|
| Review 2 manifest final replay | PASS, 46/46 hashes, ordinal order, aggregate `sha256:d32031c5cb3e68298293e0da03c5103da8b4439c67755d27c05a191b725db13d` |
| `cargo test -p handbook-engine --test hcm_2_2_authority_repair` | PASS, 23 tests |
| `cargo test -p handbook-engine --test hcm_2_2_approval_use` | PASS, 14 tests |
| promotion/transaction/authority/definition/vector/lineage/test-surface suites | PASS, 21 tests |
| `cargo test -p handbook-engine --lib` | PASS, 138 tests, including every W0-W15, R0-R9, and S0-S11 matrix |
| `cargo test --workspace --all-targets --all-features` | PASS, 265.3 seconds |
| `cargo check --workspace --all-targets --all-features` | PASS |
| `cargo clippy --workspace --all-targets --all-features -- -D warnings` | PASS |
| `cargo test --workspace --doc` | PASS, 2 compiler and 7 engine doctests |
| `RUSTDOCFLAGS=-D warnings cargo doc --workspace --no-deps` | PASS |
| `cargo fmt --all -- --check` and `git diff --check` | PASS |
| WSL installed-runtime and live-skill smokes | PASS, terminal `OK` |
| `cargo package -p handbook-engine --allow-dirty --no-verify` | PASS, 173 regular members; 170 source-identical including `Cargo.toml.orig`; all 62 definitions exact |
| packaged engine archive | 448,139 bytes; SHA-256 `64f042deedae82a508901f4af7b7715df2effe6ec339671e41925a8b8e37a2bd` |
| lifecycle-result `1.0` schema | 15,430 bytes; SHA-256 `1d7d8733b19599804fcda32fe119766b1b910e2223aa981fcd9eb92cc4d5c03f`; unchanged |
| promotion-intent `1.2` schema | 16,019 bytes; SHA-256 `c2f5cf51b833585bc10cfcd5b99ad2a2e2c0e952a79b3f2910ae6afbff746caa`; unchanged |

The package-only generated `Cargo.toml`, `.cargo_vcs_info.json`, and
`Cargo.lock` account for the non-source archive members. No released
definition, result schema, intent schema, candidate `1.3` identity, approval,
promotion, lifecycle, product, or recovery contract changed.

## GitNexus and next-review boundary

GitNexus has no indexed symbol for the newly added
`validate_historical_candidate` or its new test helpers, so each upstream
query returns `UNKNOWN`, zero impacts, and no process/module expansion. The
caller remains the previously reviewed bounded author-inventory path. No new or
expanded HIGH/CRITICAL process or module entered the subject. Final
`detect-changes` remains a post-`CLEAN`, pre-commit gate.

This proof does not declare the subject clean. The next exact subject must go
to another different fresh isolated read-only reviewer. Any finding requires a
new additive repair, full proof replay, new exact dispatch, and another
different fresh reviewer. HCM-2.3 remains unauthorized.
