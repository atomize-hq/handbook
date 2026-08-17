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
- `HCM-4.2` identifies a completed boundary without reopening it.
- `HCM-4.3` identifies the completed CLI-parity migration without reopening or
  authorizing it.
- `HCM-4.4` identifies the current Tauri-facade authority while its product
  implementation remains absent; the label does not authorize a packet by
  itself.
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
| Direct typed posture | Engine posture facade owns authority, policy, persistence, replay, and transition decisions | Implemented HCM-4.1 direct typed ingress plus the HCM-4.2 definition-bound `posture.transition.apply@1.0.0` Rust-SDK operation; broader posture policy is absent/future | A future explicitly authorized CLI command or Tauri adapter may consume the admitted SDK operation without accepting caller-made authority evidence | Decision required in phase map for any new CLI command; HCM-4.4 for Tauri | Structurally valid public input manufactures the evidence for its own mutation |
| Schema and manifest | HCM-4.2 canonical values, operation/bootstrap/posture types, and deterministic SDK generator | Implemented 115-root Draft 2020-12 tree and exact-byte manifest, including all invocation, use-case, transport, and admitted ordinary CLI roots | Future schema additions must extend the same generator while checked-in schema bytes remain distinct from instance fingerprints and CLI framing | Future explicitly authorized schema owner | Generated DTOs, runtime values, and checked-in schemas drift or share the wrong identity domain |
| Operation and admission | Semantic owner crates plus HCM-4.2 operation/envelope primitives | Implemented exact 62-row inventory and independent owner-admission boundary; posture is current through Rust SDK and the frozen 18 artifact, Flow, and pipeline operations are current through `cli_json`; the six SDK use cases remain outside ordinary discovery | Future transports consume admitted definitions without adding command-derived operations or converting use cases into ordinary capabilities | HCM-4.4 for Tauri | Adapters invent operations, bypass owner admission, or disagree about terminal outcomes |
| Bootstrap and catalog witness | HCM-4.2 production witness over checked-in schemas and Engine/Flow/pipeline evidence | Implemented immutable descriptor plus capability/schema/profile roots, typed pages/cursors, one Rust-SDK posture capability, and exactly 18 `cli_json` capabilities | Later transport evidence changes current transports only after complete definition/schema/owner/transport closure is current | HCM-4.4 when applicable | Discovery claims unsupported operations or treats structural closure as owner trust |
| Contract-owner placement and dock execution | Packet 0 kernel currently resides in SDK; future contract, evidence, gate, and dock owners are absent | SDK is the present consumer-facing composition layer; no lower-level contract owner or dock exists | Authorized outward mapping or neutral extraction avoids a contracts-to-SDK back-edge, and a separate dock executes rather than owns semantics | Decision required in phase map | Circular dependencies make presentation concerns authoritative, or an executor becomes the semantic owner |
| Gate result to posture | Future evidence or gate owner is absent; engine retains posture authority | No current gate-to-posture integration; broader posture policy remains absent/future | Gate results are bounded posture inputs while aggregation, policy, recommendation, acknowledgement, hysteresis, and transitions remain with posture owners | Decision required in phase map | An evidence producer silently becomes the posture policy and mutation engine |
| CLI transport | SDK owns transport-free composition; CLI owns parsing, repository discovery, rendering, streams, and exit mapping | Implemented exact 36-leaf coverage over one typed owner/SDK source per leaf; human, new-JSON, and established legacy renderers consume the same typed result and shared exit table | Preserve the adapter-only boundary as commands evolve under separate authority | Future command-specific authority | CLI wording or local branches become a second domain implementation |
| CLI JSON | SDK owns the invocation/use-case/transport types and schemas; CLI owns serialization selection and physical emission | Implemented global `--json`: one schema-valid `handbook.invocation-response@1.0.0` document plus LF on writable stdout, semantic diagnostics in the result, operational stderr separation, and bounded explicit legacy migration | Future transports consume the same typed outcomes without weakening singular framing or discovery truth | HCM-4.4 for Tauri | Scripts parse prose, receive mixed/multiple stdout values, or observe status/exit disagreement |
| Tauri transport | SDK owns transport-free composition; no Tauri adapter exists | Absent/future | Thin Serde/Tauri commands preserve closed SDK and owner outcomes without a normal-operation CLI subprocess | HCM-4.4 | Tauri framing acquires semantic authority or shells out as the permanent boundary |
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
