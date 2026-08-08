# HCM-3.6 implementation proof wall

This record covers the final source subject admitted by the HCM-3.6 project-posture implementation selector. It is a mechanical proof record only; it does not grant authority, change the source ceiling, or replace the required independent final review.

## Focused HCM-3.6 evidence

| Command | Result |
| --- | --- |
| `cargo test -p handbook-engine project_posture --lib --no-fail-fast` | PASS — 6 passed, 0 failed. |
| `cargo test -p handbook-engine charter_posture_transaction --lib --no-fail-fast` | PASS — 9 passed, 0 failed. |
| `cargo check -p handbook-engine` | PASS — warnings only. |
| `rustfmt --check crates/engine/src/lib.rs crates/engine/src/project_posture.rs crates/engine/src/project_posture_tests.rs crates/engine/src/charter_posture_transaction_intent_v1.rs crates/engine/src/charter_lifecycle_transition_v11.rs crates/engine/src/charter_posture_transaction_tests.rs crates/engine/src/charter_authority_transaction.rs crates/engine/src/charter_lifecycle_store.rs` | PASS. |
| `git diff --check` | PASS. |

## Consolidated remediation rerun (2026-08-08)

This rerun covers the admitted full-source remediation findings HCM-3.6-P1 through HCM-3.6-P4. It records only commands rerun for the remediation; it does not replace independent closure review.

| Command | Result |
| --- | --- |
| `cargo test -p handbook-engine project_posture --lib --no-fail-fast` | PASS — 6 passed, 0 failed. |
| `cargo test -p handbook-engine charter_posture_transaction --lib --no-fail-fast` | PASS — 9 passed, 0 failed. Includes synthetic authority-closure refusal with no canonical/journal write and preserved/refused synthetic pending result coverage. |
| `cargo test -p handbook-engine retained_v11_posture_rebase_uses_the_exact_private_jcs_lf_reader --lib --no-fail-fast` | PASS — 1 passed, 0 failed. Exercises retained v1.1 selection through the private JCS+LF reader rather than the legacy lineage decoder. |
| `cargo check -p handbook-engine` | PASS — warnings only. |
| `cargo test -p handbook-engine --test hcm_2_2_approval_use --no-fail-fast` | PASS — 14 passed, 0 failed. |
| `cargo test -p handbook-engine --test hcm_2_2_promotion_workflow --no-fail-fast` | PASS — 9 passed, 0 failed. |
| `cargo test -p handbook-engine --test hcm_2_2_lifecycle_store --no-fail-fast` | PASS — 5 passed, 0 failed. |
| `cargo test -p handbook-engine --test hcm_2_2_transaction_promotion --no-fail-fast` | PASS — 1 passed, 0 failed. |
| `rustfmt --check crates/engine/src/lib.rs crates/engine/src/project_posture.rs crates/engine/src/project_posture_tests.rs crates/engine/src/charter_posture_transaction_intent_v1.rs crates/engine/src/charter_lifecycle_transition_v11.rs crates/engine/src/charter_posture_transaction_tests.rs crates/engine/src/charter_authority_transaction.rs crates/engine/src/charter_lifecycle_store.rs` | PASS. |
| `git diff --check` | PASS. |

GitNexus `detect-changes --scope compare --base-ref main` reported `critical` repository-wide impact (1,572 files / 11,834 symbols) because the shared worktree was already dirty outside this remediation. Its FTS extension was unavailable in the local load-only CLI. This output is not a subject-scoped closure result.

## Required compatibility and workspace wall

| Command | Result |
| --- | --- |
| `cargo test -p handbook-engine --test hcm_2_2_transaction_promotion` | PASS — 1 passed, 0 failed. |
| `cargo test -p handbook-engine --test hcm_2_2_promotion_workflow` | PASS — 9 passed, 0 failed. |
| `cargo test -p handbook-engine --test hcm_2_2_lifecycle_store` | PASS — 5 passed, 0 failed. |
| `cargo test -p handbook-engine --test hcm_2_2_approval_use` | PASS — 14 passed, 0 failed. |
| `cargo test -p handbook-engine --test hcm_2_2_authority_repair` | PASS — 27 passed, 0 failed. |
| `cargo test -p handbook-engine --test hcm_2_4_charter_profile_compatibility` | PASS — 9 passed, 0 failed. |
| `cargo test -p handbook-engine --all-features` | PASS — 217 library tests plus all crate integration and doc-test binaries; 1,523.6 seconds. Earlier attempts reached the 124-second and 604-second command allowances without an error; they are recorded as non-passing timeouts and this completed rerun is the governing successful proof. |
| `cargo test -p handbook-flow --all-features` | PASS — 25 tests across unit/integration/doc tests, 0 failed. |
| `cargo test --workspace --all-targets --all-features` | PASS — all workspace targets, 0 failed; 1,869.8 seconds. |

## Final post-remediation complete wall

The following exact commands were rerun after the consolidated authority/lifecycle remediation and are the governing final proof results for the reviewed source state.

| Command | Result |
| --- | --- |
| `cargo test -p handbook-engine --test hcm_2_2_transaction_promotion` | PASS — 1 passed, 0 failed. |
| `cargo test -p handbook-engine --test hcm_2_2_promotion_workflow` | PASS — 9 passed, 0 failed. |
| `cargo test -p handbook-engine --test hcm_2_2_lifecycle_store` | PASS — 5 passed, 0 failed. |
| `cargo test -p handbook-engine --test hcm_2_2_approval_use` | PASS — 14 passed, 0 failed. |
| `cargo test -p handbook-engine --test hcm_2_2_authority_repair` | PASS — 27 passed, 0 failed. |
| `cargo test -p handbook-engine --test hcm_2_4_charter_profile_compatibility` | PASS — 9 passed, 0 failed. |
| `cargo test -p handbook-engine --all-features` | PASS — 218 library tests plus all crate integration and doc-test binaries; 1,402.2 seconds. |
| `cargo test -p handbook-flow --all-features` | PASS — 25 tests across unit/integration/doc tests, 0 failed. |
| `cargo test --workspace --all-targets --all-features` | PASS — all workspace targets, 0 failed; 1,745.4 seconds. |
| `rustfmt --check crates/engine/src/lib.rs crates/engine/src/project_posture.rs crates/engine/src/project_posture_tests.rs crates/engine/src/charter_posture_transaction_intent_v1.rs crates/engine/src/charter_lifecycle_transition_v11.rs crates/engine/src/charter_posture_transaction_tests.rs crates/engine/src/charter_authority_transaction.rs crates/engine/src/charter_lifecycle_store.rs` | PASS. |
| `git diff --check` | PASS. |

## Result and limits

All required HCM-3.6 proof commands have a successful final post-remediation run. The source builds with existing `dead_code` warnings for private HCM-3.6 helpers when compiled outside the crate-local test configuration; no warning was promoted to an error, and independent review must assess reachability and compatibility rather than treating the warnings as proof of a public-surface change.

The inherited HCM-3.5 ordinary-validator failure is not a test failure in this proof wall. It remains preserved separately under the operator's narrowly scoped HCM-3.6 resumption exception and does not waive any HCM-3.6 proof or review obligation.
