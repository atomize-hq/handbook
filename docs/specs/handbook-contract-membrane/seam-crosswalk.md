# Handbook Contract Membrane Seam Crosswalk

This crosswalk is the compact migration-routing map for contract-membrane
boundaries. It connects each important current producer seam to its present
consumer state, bounded target, and known migration owner.

It does not own architecture rationale, implementation status detail, a program
schedule, phase exits, slice or packet authority, acceptance criteria, or
execution evidence. Use the [`architecture context`](architecture-context.md)
for boundary rationale and the
[`implemented baseline`](implemented-baseline.md) for current product-state
classification. Source and tests remain product truth.

## Reading the map

Current-state wording follows the baseline's classifications:

- **Implemented:** the current product supplies the capability.
- **Precursor-only:** useful components exist without the required ordinary
  consumer or completed owner boundary.
- **Absent/future:** the named product capability is not implemented.
- **External-unknown:** repository state cannot establish the external
  condition.

A target names only the boundary that an authorized migration must preserve; it
does not promise that the target will be built.

Migration labels route ownership only. They neither authorize work nor assert
completion or schedule:

- `None — preserve` means the current boundary should remain intact.
- `HCM-4.2` identifies a boundary already established for that slice, without
  authorizing a packet.
- `Decision required in phase map` leaves the later owner deliberately
  unresolved.

Read each row as a routing constraint, never as a work queue.

## Crosswalk

| Seam | Producer/current owner | Current consumer/state | Target consumer/boundary | Migration slice | Failure prevented |
| --- | --- | --- | --- | --- | --- |
| Semantic owner composition | Engine, Flow, and pipeline own their domain decisions | SDK composes implemented owner surfaces; CLI consumes SDK surfaces | SDK projects bounded owner truth; CLI and future adapters remain non-authoritative consumers | None — preserve | SDK or adapter reimplements semantic branches and produces conflicting product truth |
| Context Resolution | Engine resolution kernel, registry, policy loading, and stack behavior | Implemented engine callers and SDK/CLI composition | Owner results flow outward through bounded SDK projections without moving resolution authority | None — preserve | Public composition becomes a second authority or bypasses repository policy |
| Projection and snapshot | Engine-private deterministic projection and snapshot-memory components | Precursor-only; no authorized ordinary consumer completes the seam | An authorized ordinary consumer receives bounded projections without synthesizing missing authority | Decision required in phase map | Private precursors are mistaken for public adoption or promoted without an owner |
| Grounding adoption | Engine grounding semantics with typed Flow and pipeline adapters | Precursor-only; adapters exist but no ordinary grounding consumer is established | An authorized ordinary consumer composes owner grounding outcomes through a bounded public seam | Decision required in phase map | Adapters infer grounding truth, or precursor evidence is reported as completed adoption |
| Direct typed posture | Engine posture facade owns authority, policy, persistence, replay, and transition decisions | Implemented transport-free SDK ingress; broader posture policy is absent/future | Contract-membrane mapping reaches the existing engine facade without accepting caller-made authority evidence | HCM-4.2 | Structurally valid public input manufactures the evidence for its own mutation |
| Schema and manifest | Packet 0 canonical values, bindings, and fingerprint primitives plus Packet 1's SDK schema generator | Implemented 15-root Draft 2020-12 tree and exact-byte manifest; operation schemas remain future | Checked-in schemas and a deterministic manifest continue to bind exact schema bytes separately from instance fingerprints as later packet roots are added | HCM-4.2 | Generated DTOs, runtime values, and checked-in schemas drift or share the wrong identity domain |
| Operation and admission | Semantic owner crates plus Packet 0 bounded contract primitives | No membrane operation definitions, authoritative registry, or owner-admission surface | One authoritative operation inventory maps bounded envelopes to semantic owners that independently admit them | HCM-4.2 | Adapters invent operations, bypass owner admission, or disagree about terminal outcomes |
| Bootstrap and catalog witness | Packet 0 catalog roots, cursors, pages, sealed witnesses, and verification path | Implemented primitives; no bootstrap producer, owner-admission catalog, or production witness | A bounded bootstrap and catalog seam exposes locally closed discovery only after the responsible owner witnesses it | HCM-4.2 | Discovery claims unsupported operations or treats structural closure as owner trust |
| Contract-owner placement and dock execution | Packet 0 kernel currently resides in SDK; future contract, evidence, gate, and dock owners are absent | SDK is the present consumer-facing composition layer; no lower-level contract owner or dock exists | Authorized outward mapping or neutral extraction avoids a contracts-to-SDK back-edge, and a separate dock executes rather than owns semantics | Decision required in phase map | Circular dependencies make presentation concerns authoritative, or an executor becomes the semantic owner |
| Gate result to posture | Future evidence or gate owner is absent; engine retains posture authority | No current gate-to-posture integration; broader posture policy remains absent/future | Gate results are bounded posture inputs while aggregation, policy, recommendation, acknowledgement, hysteresis, and transitions remain with posture owners | Decision required in phase map | An evidence producer silently becomes the posture policy and mutation engine |
| CLI and Tauri transport | SDK owns transport-free composition; CLI is a current adapter and Tauri is absent/future | Existing CLI consumes SDK but has no membrane route; no Tauri membrane consumer exists | Thin CLI and Tauri adapters parse and frame transport while preserving closed SDK and owner outcomes | Decision required in phase map | Transport framing acquires operation identity, admission, or semantic-result authority |
| Publication, bridge, and Substrate | Repository product sources; no current publication owner is established | Local membrane publication is absent/future; current bridge and Substrate adoption are external-unknown | Explicit publication artifacts and current external-owner evidence establish each bridge or adoption claim independently | Decision required in phase map | Repository absence or stale historical evidence is promoted into a present external-adoption claim |

## Use and drift

Use this map when assigning a migration owner or checking that a proposed slice
crosses a seam without moving semantic authority. The named migration owner must
still receive explicit current slice authority before any work begins.

Structural/schema validation, semantic owner trust, instance fingerprints,
checked-in schema-byte fingerprints, and adapter framing remain distinct even
when a migration touches more than one of them. A row must not be used to select
an unresolved generator, envelope, witness representation, posture mapping,
runtime, crate placement, or publication mechanism.

If current source or tests change a seam, update this crosswalk in the same
bounded change or record the discrepancy as documentation drift. New executable
truth overrides a stale row. External publication, bridge, or Substrate claims
require current external evidence and cannot be upgraded by repository inference.
