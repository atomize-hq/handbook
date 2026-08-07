# HCM-3.5 P4 transition materialization — source and impact

**Phase / slice / packet:** HCM-3 / HCM-3.5 / P4

**Status:** IMPLEMENTED PENDING DIFFERENT-FRESH CLOSURE

**source_handoff_ids:**
`["20260807T144015Z--HCM-3-5--orchestration--p1-grounding-authority-boundary"]`

**supersedes:**
`["20260807T144015Z--HCM-3-5--orchestration--p1-grounding-authority-boundary"]`

**Authority-admission slot:** direct user-authorized P1-P4 recovery grant.

## Remediation

- Added P4-owned repository source data under
  `.handbook/grounding/hcm-3.5/v1`, including E0/S1/E2 captures and records,
  Dg/Ds routes, currentness, immutable definition/disclosure, and a bounded
  projection descriptor.
- Added a private engine transition adapter that validates E0 as S1's actual
  predecessor, derives both compatible deltas unchanged through HCM-3.4, and
  returns only exact opaque strings for the six handoff fields.
- Retained the existing engine resolver and P2/P3 boundaries. A focused replay
  proves that the tracked P4 source itself grounds to the selected two-entry,
  three-omission bounded public result.

## GitNexus evidence

| Existing symbol | Upstream impact | Disposition |
| --- | --- | --- |
| `derive_grounding_source_pair` | LOW; 1 direct, 4 total, 0 processes | Narrow predecessor-aware fallback; existing legacy route remains first. |
| `ground_resolution` | LOW; 0 callers/processes | Read-only focused replay; no implementation edit. |
| `build_snapshot` | CRITICAL; 2 direct, 233 total, 55 processes | Called unchanged through the private adapter only. |
| `derive_snapshot_delta` | CRITICAL; 2 direct, 233 total, 55 processes | Called unchanged through the private adapter only. |
| `execute_projection_with_live_observer` and compile seams | retained HIGH boundaries | Not called, edited, or used as shortcuts. |

The direct source root is normally ignored, so the P4 source data is explicitly
force-added without changing `.gitignore`. Generated `AGENTS.md` and
`CLAUDE.md` refreshes remain unstaged, uncommitted, undiscarded, and excluded.
