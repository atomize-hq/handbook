# HCM-3.5 P2 grounded Flow packet — proof matrix

**Phase / slice / packet:** HCM-3 / HCM-3.5 / P2

**Status:** CLEAN

**source_handoff_ids:**
`["20260807T144015Z--HCM-3-5--orchestration--p1-grounding-authority-boundary"]`

**supersedes:**
`["20260807T144015Z--HCM-3-5--orchestration--p1-grounding-authority-boundary"]`

**Authority-admission slot:** direct user-authorized P1-P4 recovery grant,
preserving the same P1 causal identity and continuation parent.

| Requirement | Evidence | Result |
| --- | --- | --- |
| Purpose-named Flow operation consumes only the typed engine result | `cargo test -p handbook-flow --test hcm_3_5_grounded_packet` type-surface assertion for `fn(GroundingOutcome) -> GroundedPacketOutcome` | PASS |
| Ready state does not recreate engine semantics | Exhaustive grounded branch preserves only engine-created `FlowPacketGrounding`, `DeltaSignalSummary`, provenance, omissions, and evidence | PASS by complete-subject code review |
| Omitted state is typed and derived only from engine omission accounting | The grounded branch chooses `Omitted` exactly when `omissions()` is nonempty | PASS by exhaustive branch review |
| Refusal has no legacy/raw fallback | The refused branch carries `GroundingRefusal`; no resolver, filesystem, or raw-delta call exists in P2 | PASS by changed-subject review |
| Existing resolver behavior is unchanged | `cargo test -p handbook-flow --test resolver_core` | PASS (16 tests) |
| New Flow code compiles | `cargo check -p handbook-flow` | PASS |
| Formatting and whitespace are clean | `cargo fmt --all -- --check`; `git diff --check` | PASS |
| Staged change scope is safe | local `gitnexus detect-changes --scope staged`: 6 files, 1 container symbol, 0 processes, LOW; exact `resolver` module impact: 0 callers/processes, LOW | PASS |
| No Cargo/dependency/engine/pipeline/CLI/compiler/SDK/Substrate/schema/config/gate change | changed-path allowlist | PASS |

The engine makes its outcome values intentionally private-field and
engine-constructed. P2 therefore proves its public typed membrane at compile
time and its finite outcome mapping by an exhaustive source review; it does not
introduce a raw/test-only engine constructor merely to bypass that ownership
boundary. P1's focused tests independently prove the actual grounded,
omitted, and refused source semantics that P2 carries without reinterpretation.

## Closure disposition

The P2 discovery review, single implementation remediation, and separate delta
closure found no remaining P2-scope defect. P2 is CLEAN. P3 has not begun.
