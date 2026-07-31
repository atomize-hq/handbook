# P7 final-review lineage repair

Date: 2026-07-31

Status: accepted by explicit operator causal-budget extension

## Context

The first P7 final-review parent,
`20260731T022800Z--HCM-2-4--p7-phase-2-exit-orchestration`, cannot produce a
truthful completed v1.4 handoff. Its discovery review owned
`HCM-P7-FR-0001` and `HCM-P7-FR-0002`; the first closure returned FINDINGS by
reusing `HCM-P7-FR-0002` instead of assigning a new source-run-owned finding
ID. The supplemental dispatch consequently names that reused ID as the closure
trigger. Completed-handoff validation requires globally unique finding
ownership, exact review-run linkage, and a next cycle whose finding references
equal the immediately preceding FINDINGS run's owned P1/P2 IDs. Reusing the ID
violates ownership; substituting a new ID after dispatch would falsify immutable
lineage.

The old parent therefore remains immutable historical FINDINGS context only.
Its three dispatches are preserved byte-for-byte:

| Dispatch | SHA-256 |
|---|---|
| `20260731T022800Z--HCM-2-4--p7-phase-2-exit-final-review.json` | `79f68c06fe6d25d09695d7a47a80c9be4403ee6ea9b3fa50b22d456be8f70d22` |
| `20260731T024500Z--HCM-2-4--p7-phase-2-exit-remediation-closure.json` | `f1b4e09dd7ece0c23cfc70d67658c6e42e2f0980cd1cfb726abf699d2216589e` |
| `20260731T025730Z--HCM-2-4--p7-phase-2-exit-gitnexus-supplemental-closure.json` | `f68ccff8a07a844035403fec2382b5779d7af6b09dacc051d4f1544ff5054c84` |

They are not acceptance evidence and are excluded from the new parent's
delegated-run population.

## Decision

Start one fresh P7 final-review parent with these distinct causal identities:

- parent orchestration ID:
  `20260731T033800Z--HCM-2-4--p7-final-review-restart-orchestration`;
- integrated outcome ID: `hcm-2.4-p7-final-review-restart`;
- outcome-registry fingerprint:
  `sha256:15b5dacd6774e6c7fc981939bb0e66fc24c3611a58081fd54d2c8c7429c5665d`;
- parent-derived causal budget:
  `sha256:a5f7789c21162516bb4244ba4ce4b7d03eb1bbadb6db8bc19f8ac65501a0b1bb`.

The new parent performs a fresh complete-subject discovery review. Its completed
handoff reconciles only dispatches whose `parent_orchestration_id` is the new
parent ID. The old parent and `HCM-P7-FR-0001`, `HCM-P7-FR-0002`, and
`HCM-P7-FR-0003` may be referenced only as causal history. Every finding from a
new review run receives a new unique ID owned by that run.

No validator or schema relaxation is authorized. No runtime, test, fixture,
definition, profile, Cargo, dependency, public API, HCM-3.x, release, or push
authority is added.

## Consequences

- P7, HCM-2.4, and Phase 2 remain pending until the new parent's exact subject
  receives CLEAN review.
- The full P1A-P6 proof replay, eight-row exit mapping, and accepted P3
  dispositions remain evidence, but not substitutes for new-parent review.
- GitNexus symbol cardinality remains a disclosed nondeterministic diagnostic;
  exact staged paths, zero affected processes, and risk remain the operative
  change-detection evidence.
- A CLEAN result permits the ordinary reviewed-primary commit followed by one
  new-parent completed handoff and deterministic ledger closeout commit.
- No automatic continuation beyond P7 is authorized.
