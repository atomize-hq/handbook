# HCM-3.5 P3 namespaced shared-grounding inclusion — proof matrix

**Phase / slice / packet:** HCM-3 / HCM-3.5 / P3

**Status:** CLEAN

**source_handoff_ids:**
`["20260807T144015Z--HCM-3-5--orchestration--p1-grounding-authority-boundary"]`

**supersedes:**
`["20260807T144015Z--HCM-3-5--orchestration--p1-grounding-authority-boundary"]`

**Authority-admission slot:** direct user-authorized P1-P4 recovery grant.

| Requirement | Evidence | Result |
| --- | --- | --- |
| Purpose-named path accepts only typed engine inclusion | `cargo test -p handbook-pipeline --test hcm_3_5_shared_grounding_inclusion` type assertion for `fn(SharedResolutionInclusion) -> GroundedSharedResolutionInclusion` | PASS |
| Namespace is deterministic and namespaced | Same focused test asserts `handbook.hcm-3-5.shared-resolution` | PASS |
| No raw `work_level` or L0-L3 input/fallback | P3 module/test source inspection for `work_level`, compiler functions, repository/filesystem, snapshot/delta/signal, and serde terms | PASS: no matches |
| Existing compile behavior is unchanged | `cargo test -p handbook-pipeline --test pipeline_compile` | PASS (20 tests) |
| New pipeline crate path compiles | `cargo check -p handbook-pipeline` | PASS |
| Formatting and whitespace are clean | `cargo fmt --all -- --check`; `git diff --check` | PASS |
| HIGH runtime compile seam is not changed or used | exact GitNexus impact plus changed-subject inspection | PASS |
| Staged changed-scope scan is safe | local `gitnexus detect-changes --scope staged`: 6 files, 1 container symbol, 0 processes, LOW; exact `pipeline_contract_version` impact: LOW | PASS |
| No Cargo/dependency/engine/Flow/CLI/compiler/SDK/Substrate/schema/config/handoff-schema/gate change | changed-path allowlist | PASS |

The engine's `SharedResolutionInclusion` is deliberately opaque and
engine-created. P3 proves the consumer membrane and fixed namespace without
adding a raw/test-only constructor or a second selection algorithm. The engine
continues to own currentness, compatibility, redaction, and refusal.

## Closure disposition

The P3 discovery review, one implementation remediation, and distinct delta
closure found no remaining P3-scope defect. P3 is CLEAN. P4 has not begun.
