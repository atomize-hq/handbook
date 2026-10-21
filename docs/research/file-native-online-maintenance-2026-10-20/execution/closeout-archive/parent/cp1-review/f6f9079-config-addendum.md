# f6f9079 activation authority addendum

This addendum concerns committed `f6f907943c54936e49123a136a5f0f87993758ab` / candidate8 only. It preserves the separate focused review and does not accept a later pin. Meta forwarded an independent decoder finding; the source path was independently checked by this reviewer. No reproduction or engine call was run here.

## P2 Required: mutable target can change frozen configuration before selection

`packing.rs:331-378` authenticates the target catalog/index/Work and revalidates exact current STORE bytes against journal.expected_config. It does not compare target mode, consumer, fallback or navigation_version to the current STORE before selecting the target. Work2 authenticates store/generation/catalog, not those nonphysical fields. A valid installed journal's target.mode can therefore change A to B while the physical candidate remains valid and expected-parent CAS still matches. The altered browsing/publication contract is selected, violating physical-only maintenance's frozen semantic configuration.

Before selection require equality of store identity and every frozen nonphysical field, permit only the intended physical version/catalog/binding changes, and test A-to-B, consumer, fallback and navigation mismatches with exact pre-call STORE bytes unchanged. Already-selected recovery remains controlled by actual STORE; stale journal target text must not roll back or corrupt it.

The adjacent candidate identity also needs explicit binding: target.physical.id must equal j.id; target ordinal must be a checked successor of the current physical ordinal; target physical version must be the intended one. At this pin stage paths use j.id, but there is no explicit target-id equality before replacement. The selected-id mismatch is checked only at `packing.rs:572`, after STORE has already changed. A valid other-owner Work target should refuse before any selection rather than become a changed STORE followed by a conflict response. These checks do not replace exact expected-parent CAS.

State: OPEN at f6f9079, routed to meta/source owner. New-pin source and deterministic negative evidence required.
