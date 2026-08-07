# HCM-3.5 P4 descriptive transition references — source and impact record

**Phase / slice / packet:** HCM-3 / HCM-3.5 / P4

**Status:** BLOCKED before record materialization

**source_handoff_ids:**
`["20260807T144015Z--HCM-3-5--orchestration--p1-grounding-authority-boundary"]`

**supersedes:**
`["20260807T144015Z--HCM-3-5--orchestration--p1-grounding-authority-boundary"]`

**Authority-admission slot:** direct user-authorized P1-P4 recovery grant.

## Read-only facts

- `handoff-record.v1.4.schema.json` already declares the seven required,
  nullable `snapshot_refs` members. No schema, template, or validator change
  is necessary or authorized.
- The direct same-parent source handoff keeps all six refs null with
  `capture_status: "not_available"`; it is immutable historical evidence and
  was not changed.
- The P1 positive compatible-delta route is a temporary `TempDir` source test,
  not a repository-owned session transition capture.
- P2 and P3 do not create snapshot, delta, or grounding-projection records;
  they only carry typed opaque engine values.

## Finding

`HCM35-P4-DISC-001` is a P2-priority truthfulness blocker: the recovery grant
requires exact descriptive values, while the current repository lacks the
persisted current transition records that would supply them. A null record is
honest but does not achieve P4's selected reference materialization. This is a
missing technical input, not a request for product-policy selection.

No existing symbol, record, schema, ledger, protected checkout, integration
ref, or remote was touched. The generated `AGENTS.md` and `CLAUDE.md` count
refreshes remain unstaged, uncommitted, undiscarded, and excluded.
