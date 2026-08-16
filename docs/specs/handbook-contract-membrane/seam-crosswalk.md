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
- `HCM-4.3` identifies the authority-ready CLI-parity migration without
  authorizing a packet by itself.
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
| Schema and manifest | HCM-4.2 canonical values, operation/bootstrap/posture types, and deterministic SDK generator | Implemented 53-root Draft 2020-12 tree and exact-byte manifest | The SDK-owned invocation/use-case/transport roots and ordinary-operation schemas extend the same deterministic tree while checked-in schema bytes remain distinct from instance fingerprints and CLI framing | HCM-4.3 for CLI response additions | Generated DTOs, runtime values, and checked-in schemas drift or share the wrong identity domain |
| Operation and admission | Semantic owner crates plus HCM-4.2 operation/envelope primitives | Implemented exact 62-row inventory and independent owner-admission boundary; only posture transition is admitted/discoverable through Rust SDK and the other 61 rows remain omitted | The exact artifact, Flow, and pipeline mappings fixed by HCM-4.3 must close as ordinary operations; only its closed six-entry SDK use-case registry bypasses operation discovery without inventing owner admission | HCM-4.3 | Adapters invent operations, bypass owner admission, or disagree about terminal outcomes |
| Bootstrap and catalog witness | HCM-4.2 production witness over checked-in schemas and Engine/Flow/pipeline evidence | Implemented immutable descriptor plus capability/schema/profile roots, typed pages/cursors, and one honest Rust-SDK posture capability | Later transport proof updates current transports and discovery only after the complete definition/schema/owner/transport closure is current | HCM-4.3 for CLI evidence when applicable | Discovery claims unsupported operations or treats structural closure as owner trust |
| Contract-owner placement and dock execution | Packet 0 kernel currently resides in SDK; future contract, evidence, gate, and dock owners are absent | SDK is the present consumer-facing composition layer; no lower-level contract owner or dock exists | Authorized outward mapping or neutral extraction avoids a contracts-to-SDK back-edge, and a separate dock executes rather than owns semantics | Decision required in phase map | Circular dependencies make presentation concerns authoritative, or an executor becomes the semantic owner |
| Gate result to posture | Future evidence or gate owner is absent; engine retains posture authority | No current gate-to-posture integration; broader posture policy remains absent/future | Gate results are bounded posture inputs while aggregation, policy, recommendation, acknowledgement, hysteresis, and transitions remain with posture owners | Decision required in phase map | An evidence producer silently becomes the posture policy and mutation engine |
| CLI transport | SDK owns transport-free composition; CLI owns parsing, repository discovery, rendering, streams, and exit mapping | Eight top-level routes consume SDK surfaces, but typed-result use, human rendering, machine output, stderr behavior, and exits are not yet governed by one execution boundary | Every supported nontrivial current leaf maps to one typed owner/SDK result; human and machine renderers consume that result while CLI retains transport-only concerns | HCM-4.3 | CLI wording or local branches become a second domain implementation |
| CLI JSON | HCM-4.2 supplies generic response primitives; current CLI has doctor, Charter, approver, and artifact JSON precedents plus prose-only commands | Precursor-only: JSON is partial and command-specific, several pre-result failures emit prose or empty stdout, and no complete CLI transport is current in discovery | On a writable stdout transport, `--json` emits exactly one `handbook.invocation-response@1.0.0` document plus one LF for every recognized nontrivial invocation; semantic diagnostics remain typed, progress/logging uses stderr, and exit/status agree | HCM-4.3 | Scripts parse prose, receive mixed/multiple stdout values, or observe status/exit disagreement |
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
