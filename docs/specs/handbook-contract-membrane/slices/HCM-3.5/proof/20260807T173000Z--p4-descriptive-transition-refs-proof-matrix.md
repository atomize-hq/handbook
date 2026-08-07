# HCM-3.5 P4 descriptive transition references — blocked proof matrix

**Phase / slice / packet:** HCM-3 / HCM-3.5 / P4

**Status:** BLOCKED before implementation

**source_handoff_ids:**
`["20260807T144015Z--HCM-3-5--orchestration--p1-grounding-authority-boundary"]`

**supersedes:**
`["20260807T144015Z--HCM-3-5--orchestration--p1-grounding-authority-boundary"]`

**Authority-admission slot:** direct user-authorized P1-P4 recovery grant.

| Required proof | Status | Evidence |
| --- | --- | --- |
| Use only existing nullable `snapshot_refs` capacity | PASS (read-only) | v1.4 schema declares all fields; P4 makes no schema/template change. |
| Prior end, start, compatible delta, and grounding projection refs are exact/current/descriptive | BLOCKED | No repository-owned capture/projection records exist for this task. |
| End snapshot and final delta are exact/current/descriptive | BLOCKED | No session-end capture/final compatible delta exists. |
| Missing/partial capture remains truthful and non-promoting | PASS | Existing `not_available`/null representation is retained; no record is fabricated. |
| No copied snapshot, promotion claim, gate runtime, or ledger/history rewrite | PASS | No handoff record, schema, validator, gate, or remote action occurred. |

The P1 temporary test source proves engine compatibility only; it cannot be
promoted into a real handoff transition reference. P4 therefore has no valid
consolidated implementation remediation within its stated ceiling. P1-P3 stay
CLEAN, P4 is not CLEAN, and no integration-target CAS or P5/P6 work is
permitted.
