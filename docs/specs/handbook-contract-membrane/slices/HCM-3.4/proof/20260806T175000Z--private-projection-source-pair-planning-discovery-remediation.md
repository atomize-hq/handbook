# HCM-3.4 private Projection source-pair planning discovery remediation

## Trigger and disposition

Fresh built-in discovery run `/root/source_pair_discovery_review` returned
three valid P2 findings for the frozen source-pair planning subject. They share
one narrow cause: the first plan described the desired generic pair closure but
did not specify all private raw/request/result data-shape changes needed for a
later implementation to prove it. This is one consolidated documentation-only
remediation under the unchanged
`hcm-3.4-private-projection-source-pair-planning` outcome and causal budget.

| Finding | Disposition | Documentary repair |
|---|---|---|
| `HCM34-SPP-DISC-001` | remediated | `RawSourceDocument` now decodes exactly one optional legacy singleton or canonical multi-family array, then normalizes to one closure; missing, both-present, empty, unsorted, and duplicate forms refuse before payload access. |
| `HCM34-SPP-DISC-002` | remediated | private current state identity is bound in source document, request selection, delta `to_snapshot` dependency, and copied result provenance; legacy selections remain exact-pair-only. |
| `HCM34-SPP-DISC-003` | remediated | currentness now specifies exact adapter pairs, allowed empty declared slot sets, and exactly one result check per family with work slot maps nested in its one check. |

## Scope and proof preservation

The remediation changes only HCM-3.4 planning documents and adds this evidence
record. It does not touch code, tests, fixtures, generic Projection runtime
configuration, schemas, public APIs, dependencies, consumers, or HCM-3.5.
`PG-SNAP-04` remains open. The fixed future test matrix now includes all
requested parser, state-substitution, adapter-fingerprint, slot-map, and
five-check cardinality negatives alongside the earlier pair/redaction/Resolution
wall.

The next review is a different-fresh closure focused on these three repairs,
their effect on the generic one-source baseline, and their stated proof matrix.
It must not reopen discovery or add a new source-pair authority surface.
