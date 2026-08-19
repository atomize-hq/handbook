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
- `HCM-4.4` identifies the completed Tauri-facade migration without reopening
  or authorizing it.
- `HCM-4.5` identifies the completed generic installed-skill consumer seam
  without reopening or authorizing it.
- `HCM-5.1` identifies the completed contract-owner and outward SDK migration.
- `HCM-5.2` identifies prepared evidence/verdict/gate/local-closeout authority;
  Packets 1–2 implement its owner identity/run and sealed candidate-source
  normalization seams, while persistence/evaluation/composition remain future.
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
| Direct typed posture | Engine posture facade owns authority, policy, persistence, replay, and transition decisions | Implemented HCM-4.1 direct typed ingress plus the definition-bound `posture.transition.apply@1.0.0` operation through exactly `rust_sdk` and `tauri`; broader posture policy is absent/future | Preserve the admitted SDK/owner route without accepting caller-made authority evidence; any future CLI posture command requires separate authority | Decision required in phase map for any new CLI command; HCM-4.4 for Tauri | Structurally valid public input manufactures the evidence for its own mutation |
| Schema and manifest | HCM-4.2 canonical values, operation/bootstrap/posture types, and deterministic SDK generator | Implemented 135-root Draft 2020-12 tree and exact-byte manifest, including the 15 HCM-5.1 Packet-4 domain/request/result/problem roots and all predecessor roots | Future schema additions must extend the same generator while checked-in schema bytes remain distinct from instance fingerprints and transport framing | Future explicitly authorized schema owner | Generated DTOs, runtime values, and checked-in schemas drift or share the wrong identity domain |
| Operation and admission | Semantic owner crates plus HCM-4.2 operation/envelope primitives | Implemented exact 62-row inventory and independent owner-admission boundary; the four Contracts operations close their reserved rows with exact owner evidence and `[cli_json, rust_sdk, tauri]`, the 18 predecessor operations expose `[cli_json, tauri]`, posture exposes `[rust_sdk, tauri]`, and the six fixed use cases remain outside ordinary discovery | Future transports consume admitted definitions without adding command-derived operations or converting use cases into ordinary capabilities | HCM-5.1 Packet 4 for Contracts closure | Adapters invent operations, bypass owner admission, or disagree about terminal outcomes |
| Bootstrap and catalog witness | HCM-4.2 production witness over checked-in schemas and owner evidence | Implemented immutable descriptor plus capability/schema/profile roots, typed pages/cursors, and exactly 23 capabilities with their current SDK/CLI/Tauri transport vectors; the SDK consumer, generic CLI actions, and installed skill consume this witness without another inventory | Later transport evidence changes current transports only after complete definition/schema/owner/transport closure is current | HCM-5.1 Packet 4 for Contracts witness closure | Discovery claims unsupported operations or treats structural closure as owner trust |
| Contract-owner placement and dock execution | HCM-5.1 Packets 1–6 establish and integrate `handbook-contracts` definition/claim/applicability/lifecycle ownership inward of SDK, one sealed Engine authority-admission dependency, repository-bound durable owner use cases, the bounded outward four-operation SDK membrane, and generic SDK/CLI/Tauri consumer proof; evidence/gate and dock behavior are absent | The owner has no SDK/transport dependency; lifecycle state and the single persisted current head are chain-derived, admitted mutations require the same retained physical repository root, exact-inventory bounded recovery fails closed, SDK DTO/schema identity never substitutes for owner truth, and generic transports cannot manufacture authority. Final current witness truth is four HCM-5.1 operations within 62 inventory rows, 23 capabilities, and 135 schema roots. | Preserve the proven generic consumer boundary while HCM-5.2 extends the same owner and a separate HCM-5.3 dock later executes rather than owns semantics | HCM-5.1 closeout complete; HCM-5.2 authority prepared; HCM-5.3 for execution | Circular dependencies make presentation concerns authoritative, callers manufacture lifecycle authority, durability reports false receipts, or an executor becomes the semantic owner |
| Evidence, verdict, gate, local closeout, and posture boundary | HCM-5.2 Packets 1–2 implement owner identities/run preparation and sealed exact candidate-source normalization; canonical-evidence persistence, evaluation, gate/local-closeout composition, SDK operations, and dock execution remain absent; Engine retains only generic authority/currentness/Resolution facts and posture authority | Candidate bytes and immutable provenance are read from already committed bounded observations, authenticated under the exact `contract_evidence_source` class, and independently rechecked by `handbook-contracts` before its private witness can construct canonical evidence; no verdict or gate authority follows | Preserve `handbook-contracts` as canonical evidence/verdict/gate/local-closeout owner; future executors provide untrusted candidates only, and future posture/promotion owners independently consume results without a gate-owned mutation | HCM-5.2 Packets 1–2 implemented; Packets 3–7 and later posture/promotion authority remain | A candidate producer becomes semantic authority, local closeout implies parent promotion, or the gate silently becomes posture policy/mutation authority |
| CLI transport | SDK owns transport-free composition; CLI owns parsing, repository discovery, rendering, streams, and exit mapping | Implemented exact 36-leaf coverage plus exactly four generic capability actions over one typed owner/SDK source per route; human, new-JSON, established legacy, and generic membrane renderers preserve the same typed result and exit truth | Preserve the adapter-only boundary as commands evolve under separate authority | HCM-4.5 for the generic membrane; future command-specific authority otherwise | CLI wording or local branches become a second domain implementation |
| CLI JSON | SDK owns the invocation/use-case/transport types and schemas; CLI owns serialization selection and physical emission | Implemented global `--json`: one schema-valid `handbook.invocation-response@1.0.0` document plus LF on writable stdout, semantic diagnostics in the result, operational stderr separation, and bounded explicit legacy migration | Tauri consumes the same typed outcomes while keeping CLI LF framing distinct from IPC framing and discovery truth | HCM-4.4 for Tauri | Scripts parse prose, receive mixed/multiple stdout values, or observe status/exit disagreement |
| Installed skill consumer | SDK owns the consumer contracts and owner dispatch; CLI owns the four generic machine actions | Implemented generated root/leaf projections and source/release/public installation; the root skill pages live discovery, retrieves exact schemas, invokes once, and preserves typed owner and replay truth through CLI only | Later admitted ordinary capabilities require complete product-owned definition/schema/owner/transport closure, then become available without skill prose or another generic action | HCM-4.5 | Prompt text becomes a product inventory, bypasses mutation authority, retries uncertain writes, or reclassifies typed terminals |
| Tauri transport | SDK owns transport-free composition; `handbook-tauri` owns host state, scheduling, bounded host failures, serialization handoff, and IPC framing | Implemented exact three-command non-UI facade over 23 ordinary routes and six fixed use cases, with no normal-operation CLI subprocess or frontend semantic authority | Preserve thin Serde/Tauri commands, host-managed trust, canonical re-admission, and owner-controlled mutation/replay | HCM-4.4; HCM-5.1 Packet 5 proves the four Contracts routes without changing the facade | Tauri framing acquires semantic authority or shells out as the permanent boundary |
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
