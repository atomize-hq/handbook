# HCM-3.5 P4 transition materialization — proof matrix

**Phase / slice / packet:** HCM-3 / HCM-3.5 / P4

**Status:** IMPLEMENTED PENDING DIFFERENT-FRESH CLOSURE

**source_handoff_ids:**
`["20260807T144015Z--HCM-3-5--orchestration--p1-grounding-authority-boundary"]`

**supersedes:**
`["20260807T144015Z--HCM-3-5--orchestration--p1-grounding-authority-boundary"]`

**Authority-admission slot:** direct user-authorized P1-P4 recovery grant.

| Required proof | Status | Evidence |
| --- | --- | --- |
| E0 -> S1 predecessor is exact and validated | PASS | `derive_grounding_transition` supplies E0 history to S1; S1 carries E0's actual record fingerprint. |
| Dg and Ds are compatible and exact | PASS | The unchanged private HCM-3.4 delta derivation accepts both pairs; each persisted route verifies its endpoint/catalog fingerprints. |
| Grounding projection is bounded and descriptive | PASS | The tracked route replays through `resolve_with_authority` with 2 public entries, 3 typed omissions, and unavailable non-promoting evidence. |
| All six handoff refs are exact and source-bound | PASS | `materialize_grounding_transition_refs` validates source plus descriptor and pins every exact token in its focused test. |
| No raw snapshot/data copy or promotion claim enters the handoff | PASS | The descriptor contains refs and bounded counts only; final handoff capacity remains existing nullable strings. |
| No schema/template/validator/gate/runtime/consumer expansion | PASS | No such file or runtime is changed or invoked. |
| Different-fresh closure review | PENDING | Must review this complete P4 delta before any durable successor handoff, ledger closeout, or target CAS. |
