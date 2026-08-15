# Handbook Contract Membrane Phase and Slice Map

This document owns program navigation for the Handbook contract membrane: the
program status vocabulary, phase and slice order, hard predecessors, concise
outcome intent, phase-exit state, and primary unresolved decisions.

Its current authority is the user-selected reduced Phase-3 boundary recorded
below. That decision accepts the named implementation checkpoints as the
selected Phase-3 outcome boundary while requiring fresh exit evidence.

This map is roadmap authority only. It does not authorize execution, define
packet acceptance, replace slice-specific authority, repeat detailed product or
architecture truth, or create orchestration state. **Listing a slice or packet
never authorizes it.**

Use the [`implemented baseline`](implemented-baseline.md) for detailed product
state, the [`architecture context`](architecture-context.md) for stable
ownership and dependency direction, and the [`seam crosswalk`](seam-crosswalk.md)
for migration routing. Source and tests remain product truth.

## Program status vocabulary

These are program labels, distinct from the product-state classifications in
the implemented baseline and the execution states in the
[`work-unit lifecycle`](work-unit-lifecycle.md).

- **Implemented checkpoint:** code or product state exists, but this alone does
  not prove slice or phase closeout.
- **Authorized:** current explicit slice authority exists. This label applies
  only when a separate current authority grants it.
- **Planned:** roadmap intent exists, but no current implementation authority
  exists.
- **Decision required:** a material owner or boundary decision remains
  unresolved.
- **Phase exited:** the selected outcome and required exit evidence are both
  complete.

`exit verification pending` is an evidence condition, not another status label.
No wording in this map may be used to upgrade a checkpoint into authorization or
completion.

## Ordering and authority rules

- Implementation chronology and legal continuation order are separate. Later
  code may exist without proving that an earlier phase exited.
- A **hard predecessor** blocks dispatch of a dependent slice until the
  predecessor is satisfied or the user grants an explicit exception.
- Within a phase, the preceding listed slice is the default hard predecessor
  unless a row states a narrower dependency or records a past exception.
- Phase exit is the default hard predecessor for new work in the next phase.
  Existing out-of-order checkpoints remain evidence, not retroactive authority.
- Future slice authority owns technical acceptance and packet choices. The
  [`review and closeout policy`](review-and-closeout-policy.md) owns evidence and
  review behavior once work is authorized.

The durable dependency direction remains:

```text
semantic authority -> projections -> SDK -> machine transports
                   -> contract/evidence/gate owners -> dock executor
                   -> publication and external consumers
```

## Prior-work orientation

| Range | Program status | Orientation |
| --- | --- | --- |
| Phases 0–2 | Implemented checkpoint | Current artifact, profile, schema, kind, intake, registry, and canonical-product foundations descend from this work. Retained fixtures under [`slices/`](slices/) remain product evidence. This map does not reconstruct detailed history or recertify historical exits. |
| Early Phase 3 | Implemented checkpoint | Vocabulary, Context Resolution, private deterministic projection, and snapshot/delta components exist in product ancestry. Public or ordinary-consumer adoption must not be inferred from private components. |

These checkpoints orient the remaining map; they supply no continuing work
authority.

## Phase 3 — resolution-aware context and posture

The selected Phase-3 outcome is deliberately smaller than the historical
aspiration. It accepts the implemented grounding chain and the Charter-only
atomic posture transition while rehoming the residual work to later owners.

| Slice | Program status | Concise outcome and boundary | Hard predecessor |
| --- | --- | --- | --- |
| `HCM-3.1` — Vocabulary resolution | Implemented checkpoint | Typed vocabulary changes presentation while stable machine semantics remain intact. | Phase-2 foundations |
| `HCM-3.2` — Context Resolution kernel | Implemented checkpoint | A typed resolution kernel provides ordered, policy-bound context semantics. | `HCM-3.1` |
| `HCM-3.3` — Deterministic Projection engine | Implemented checkpoint | Private deterministic reveal/derive projections account for omissions, lossiness, provenance, and identity; ordinary adoption is not implied. | `HCM-3.2` |
| `HCM-3.4` — Snapshot Memory and delta engine | Implemented checkpoint | Bounded snapshots and deterministic deltas classify consistency and drift without model-owned semantics. | `HCM-3.3` |
| `HCM-3.5` — Resolution-aware grounding adoption | Implemented checkpoint | Packets 1–4 exist through `8ea607a`, `5be1766`, `e32897a`, and `935c939`, with fail-closed hardening at `50b16e3`. Their grounding, Flow, pipeline, and transition-reference outcome is the selected boundary. | `HCM-3.4` |
| `HCM-3.6` — Project posture transition | Implemented checkpoint | The Charter-only, one-leaf compare-and-swap transition exists through `1d4844f`, including private authority checks, journaling, persistence, recovery, and replay. That atomic transition is the selected boundary. | `HCM-3.5` selected boundary |

### Reduced boundary and rehomed work

- Gate-owned local-closeout and parent-promotion evaluation formerly associated
  with the grounding slice moves to the later evidence/verdict/gate owner.
- Ordinary SDK/JSON exposure, publication, bridging, and Substrate adoption for
  grounding move to the appropriate Phase-4 and Phase-6 owners.
- Multi-source posture evaluation, recommendations, notifications,
  acknowledgements, and hysteresis move to later posture authority. Gate results
  may become posture inputs, but the gate owner must not absorb posture policy or
  mutation authority.

### Phase-3 exit state

**Implemented checkpoint — exit verification pending; not Phase exited.** The
later HCM-4.1 and Packet-0 checkpoints do not alter this state.

Exit requires one separate bounded evidence subject containing all of:

1. fresh strict Clippy for the applicable Phase-3 product boundary;
2. focused grounding regressions for the selected `HCM-3.5` Packets 1–4 result;
3. focused posture regressions for the selected `HCM-3.6` atomic result;
4. one focused causal review of that exact evidence subject; and
5. explicit closeout against this reduced boundary.

## Phase 4 — SDK and machine transports

| Slice | Program status | Concise outcome and boundary | Hard predecessor |
| --- | --- | --- | --- |
| `HCM-4.1` — SDK owner and use-case implementation | Implemented checkpoint | Checkpoint `28fa4ab` provides transport-free SDK composition and the typed posture ingress under a past dependency-order exception. It is not proof that Phase 3 exited. | Past explicit exception; no retroactive inference |
| `HCM-4.2` — Shared DTO and JSON Schema membrane | Implemented checkpoint for Packet 0; Packets 1–5 Planned | Packet 0 at `8605d65` supplies owner-neutral bounded types, canonical identities, catalog primitives, secret-safe keys, and sealed verification. Later packets must establish operations, schemas, envelopes, discovery, mappings, and transport exposure under fresh slice/packet authority. | **Phase-3 exit before any further Packet-1 work**, unless the user grants a new explicit exception |
| `HCM-4.3` — Complete CLI JSON parity | Planned | Every supported nontrivial command uses the shared typed result boundary, with machine output, human rendering, diagnostics, and exit behavior kept distinct. | Phase-3 exit and preceding slice closeout |
| `HCM-4.4` — Tauri-ready command facade | Planned | Thin Serde/Tauri commands consume SDK types without a normal-operation CLI subprocess or frontend-owned semantics. | Preceding slice closeout |
| `HCM-4.5` — Capability-driven Handbook skill | Planned | Agent workflows discover capabilities and gather structured inputs through supported machine surfaces without prompt-owned product logic. | Preceding slice closeout |

Packet 0's chronology is recorded, not retroactively authorized by this map.
Phase 4 may exit only when supported SDK, CLI, and Tauri-facing use cases share
typed owner outcomes; machine schemas cover supported responses; transports own
no domain truth; and capability discovery does not create command-name coupling.

## Phase 5 — contract membrane and docks

Owner and executor remain separate: contract, evidence, verdict, and gate owners
decide semantic truth; a dock executes a bounded process and emits operational
records plus untrusted candidates.

| Slice | Program status | Concise outcome and boundary | Hard predecessor |
| --- | --- | --- | --- |
| `HCM-5.1` — Contract lifecycle and claim model | Decision required | Establish exact versioned contract identity, immutable authority transitions, typed claims and applicability, and gate effects without encoding evaluation as lifecycle state. Contract-owner placement remains unresolved. | Phase-4 exit and owner-placement decision |
| `HCM-5.2` — Evidence, verdict, and gate engine | Planned | Admit immutable evidence, compute closed verdicts, enforce complete accounting and freshness, and keep local closeout separate from parent promotion. | Preceding slice closeout |
| `HCM-5.3` — Process dock protocol | Decision required | Define a bounded, default-deny executor protocol with exact bundle/runtime identity, timeouts, cleanup, and no partial evidence. The runtime and implementation choice remains unresolved; no bundled-Python choice is restored. | Contract/evidence interfaces from the preceding slices and an executor decision |
| `HCM-5.4` — First real validator dock proof | Planned | Exercise one real offline validator through the dock into admitted evidence, verdict, and gate behavior without granting the validator or executor semantic authority. | Both evidence/gate and dock protocol closeout |

Phase 5 may exit only when an exact active contract drives a real selected-dock
path through admitted evidence, verdict, and gate; missing, stale, malformed,
substituted, under-resolved, timed-out, or cleanup-uncertain inputs fail closed;
and the ordinary contract/dock use cases exist behind SDK types.

## Phase 6 — consumer adoption

Repository evidence does not prove present publication, an operating bridge, or
Substrate adoption. Each external claim requires current evidence from the
responsible external owner or system.

| Slice | Program status | Concise outcome and boundary | Hard predecessor |
| --- | --- | --- | --- |
| `HCM-6.1` — Replaceable CLI bridge | Decision required | Establish an isolated version-bound JSON bridge at a real Substrate product seam without moving semantic authority into the adapter. The external adopter remains unselected. | Phase-5 exit and adopter decision |
| `HCM-6.2` — Published SDK and owner APIs | Decision required | Publish exact affected versions and obtain current external-consumer proof without treating repository source as external evidence. Publication mechanism and versions remain unresolved. | Preceding slice closeout and publication decision |
| `HCM-6.3` — Direct Substrate registry adoption | Decision required | Prove a real external seam against exact published versions, then make the bridge-replacement decision. Current adoption remains externally unproven. | Published APIs plus current external-owner participation |
| `HCM-6.4` — Tauri product implementation | Planned | Build a GUI over the same SDK capabilities for artifact, snapshot, projection, contract, execution, and evidence workflows, with no frontend semantic authority. | Preceding slice closeout; Phase-4 and Phase-5 exits remain inherited |
| `HCM-6.5` — Workflow adapter foundation | Decision required | Define bounded adapter manifests and semantic mappings for profile, vocabulary, and Resolution translation; no marketplace or broad inventory is implied. | Preceding slice closeout and an adapter-owner decision |

Phase 6 may exit only when each selected local consumer preserves SDK and owner
truth, and each publication or external-adoption claim has current proof at its
actual owner boundary. Unselected external consumers are not an exit failure.

## Primary unresolved decisions

| Decision | Where later authority must decide it |
| --- | --- |
| Ordinary grounding SDK/JSON exposure and consumer placement | Future Phase-4 authority must choose the bounded public seam; future Phase-6 authority must separately choose any publication, bridge, and adopter |
| Broader posture evaluation, recommendation, notification, acknowledgement, and hysteresis | Future posture authority must place this policy without transferring engine-owned mutation authority to a gate or adapter |
| Schema generation, operation inventory, and parser rules | Packet 1 authority; this map selects none |
| Request/response envelopes, discovery witnesses/catalog representation, and posture mapping | Their later packet authorities; no representation is selected here |
| Contract-owner placement or neutral extraction | Future contract-owner authority, preserving the no-back-edge architecture rule |
| Gate-to-posture input and policy boundary | Future gate and posture authorities, preserving posture ownership |
| Dock implementation and runtime | Future dock authority; current Rust schema use does not preselect the executor |
| CLI/Tauri membrane routes and framing | Future transport authorities after shared typed boundaries exist |
| Publication mechanism, exact versions, bridge, and external adopter | Future publication and consumer authorities with current external proof |

Use this map to route a future slice author to the proper predecessor and
decision boundary. Then create separate, explicit slice authority before
execution. Update this map when a current decision changes ordering, status, or
exit state; update the linked owner document instead when only product truth,
architecture rationale, or seam routing changes.
