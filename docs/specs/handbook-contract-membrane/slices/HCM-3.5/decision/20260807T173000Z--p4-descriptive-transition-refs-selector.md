# HCM-3.5 P4 descriptive transition-reference selector

**Phase / slice / packet:** HCM-3 / HCM-3.5 / P4

**Status:** BLOCKED — no truthful repository-owned capture sequence exists

**Continuation parent:** `handbook-hcm-3-5-continuation-implementation-20260807`

**source_handoff_ids:**
`["20260807T144015Z--HCM-3-5--orchestration--p1-grounding-authority-boundary"]`

**supersedes:**
`["20260807T144015Z--HCM-3-5--orchestration--p1-grounding-authority-boundary"]`

**Authority-admission slot:** the direct user-authorized P1-P4 recovery grant,
preserving the same continuation parent and P1 causal identity.

## Existing capacity and selected truthfulness rule

The existing v1.4 `snapshot_refs` object already provides seven required
members without a schema change:

```text
capture_status
prior_end_snapshot_ref
session_start_snapshot_ref
grounding_delta_ref
grounding_projection_ref
session_end_snapshot_ref
session_delta_ref
```

All six reference fields are nullable. P4 may populate a field only with an
exact, repository-owned, current, descriptive ref/fingerprint pair from the
ordered transition sequence. It must never copy a snapshot, assert a
projection, or imply local/parent promotion.

## Stop result

P1 proves a compatible source pair only inside a temporary engine test
repository. Its deterministic policy/captures/snapshots/catalog/route are
constructed in `grounding.rs` test code and are not current session records or
persisted handoff-owned observations. P2 and P3 consume opaque engine values
without creating session capture records. No repository-owned
`prior_end_snapshot_ref`, `session_start_snapshot_ref`, exact
`grounding_delta_ref`, or `grounding_projection_ref` exists for this task.

Therefore P4 cannot truthfully select `capture_status: "captured"` or
`"partial"` and cannot populate any reference member. The existing
`not_available` plus null capacity remains the correct representation until a
separately persisted, current, exact transition source exists. Inventing a
`test-fixture://`, `git:`, definition, or documentation reference as a snapshot
or grounding-projection ref would violate the selected boundary.

No handoff schema, template, validator, gate runtime, ledger history, or
existing handoff record is changed. Because P4 is not CLEAN, the integration
target is not CASed and P5/P6 remain out of scope.
