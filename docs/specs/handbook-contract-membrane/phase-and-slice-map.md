# Handbook Contract Membrane Phase and Slice Map

This document owns program navigation for the Handbook contract membrane: the
program status vocabulary, phase and slice order, hard predecessors, concise
outcome intent, phase-exit state, and primary unresolved decisions.

Its current authority is the user-selected reduced Phase-3 boundary recorded
below. That decision accepts the named implementation checkpoints as the
selected Phase-3 outcome boundary; its fresh exit evidence is now complete.

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

**Phase exited under the selected reduced boundary.** Fresh closeout evidence
comprises strict Clippy across the engine, Flow, and pipeline owner boundary;
the focused engine, Flow, and pipeline `HCM-3.5` suites; the focused engine
`project_posture_tests` and `charter_posture_transaction_tests` suites; and one
focused causal review of the complete product-and-documentation subject.

This exit does not upgrade or reclaim the rehomed work above. The later HCM-4.1
and Packet-0 checkpoints remain chronology, not evidence for this exit.

## Phase 4 — SDK and machine transports

| Slice | Program status | Concise outcome and boundary | Hard predecessor |
| --- | --- | --- | --- |
| `HCM-4.1` — SDK owner and use-case implementation | Implemented checkpoint | Checkpoint `28fa4ab` provides transport-free SDK composition and the typed posture ingress under a past dependency-order exception. Its chronology was not Phase-3 exit evidence. | Past explicit exception; no retroactive inference |
| [`HCM-4.2` — Shared DTO and JSON Schema membrane](slices/HCM-4.2/README.md) | Implemented checkpoint; slice closeout complete | Packets 0–5 established the bounded typed kernel, 53-root schema/manifest tree, exact 62-row operation inventory and owner-admission boundary, immutable production bootstrap/catalog witness, one admitted Rust-SDK posture operation, and integrated proof through `c1f5d33`. One aggregate verification and causal review completed the slice. | Phase-3 exit, satisfied |
| [`HCM-4.3` — Complete CLI JSON parity](slices/HCM-4.3/README.md) | Implemented checkpoint; slice closeout complete | Packets 1–8 and one aggregate review closed all 36 supported nontrivial leaves/modes through 18 admitted `cli_json` ordinary operations or six fixed SDK use cases, with one schema-valid machine document, shared human and bounded legacy projections, stable diagnostics/exits, 115 schema roots, exact 62-row discovery continuity, and route-resolve correctly omitted from ordinary transport. | **Phase-3 exit and HCM-4.2 slice closeout are satisfied** |
| [`HCM-4.4` — Tauri-ready command facade](slices/HCM-4.4/README.md) | Implemented checkpoint; slice closeout complete | Packets 1–6 and one aggregate review close three versioned non-UI Tauri commands over five new SDK contract roots and a host-managed repository context. The 120-root schema tree, exact 19 capabilities, and six fixed non-discoverable use cases close without a normal-operation CLI subprocess, command-name-derived identity, or frontend-owned semantics. | HCM-4.3 slice closeout, satisfied |
| [`HCM-4.5` — Capability-driven Handbook skill](slices/HCM-4.5/README.md) | Implemented checkpoint; slice closeout complete | The installed root skill and four-action generic CLI membrane discover admitted ordinary capabilities, retrieve exact contracts and schemas, gather and validate structured bodies, invoke only through current transport evidence, and preserve typed owner outcome/mutation/replay truth without prompt-owned product logic. Six fixed use cases remain outside discovery. One aggregate verification and causal review completed the slice. | HCM-4.4 slice closeout, satisfied |

Packet 0's chronology is recorded, not retroactively authorized by this map.
Phase 4 may exit only when supported SDK, CLI, and Tauri-facing use cases share
typed owner outcomes; machine schemas cover supported responses; transports own
no domain truth; and capability discovery does not create command-name coupling.

### Phase-4 exit state

**Phase exited.** HCM-4.2 through HCM-4.5 are closed. Current SDK, CLI, Tauri,
and installed-skill consumers share typed owner outcomes and the deterministic
120-root machine-schema tree; transports remain adapter-only; capability
identity comes from admitted operation bindings and catalog evidence rather
than CLI or Tauri command names. The six fixed SDK use cases remain deliberately
outside ordinary discovery.

This exit does not itself authorize Phase 5 or supply contract, evidence,
verdict, gate, dock, or validator semantics. The separate HCM-5.1 authority and
closeout establish the contract-owner predecessor; the separate HCM-5.2
authority selects the next slice boundary without changing this exit.

## Phase 5 — contract membrane and docks

Owner and executor remain separate: contract, evidence, verdict, and gate owners
decide semantic truth; a dock executes a bounded process and emits operational
records plus untrusted candidates.

| Slice | Program status | Concise outcome and boundary | Hard predecessor |
| --- | --- | --- | --- |
| [`HCM-5.1` — Contract lifecycle and claim model](slices/HCM-5.1/README.md) | Implemented checkpoint; slice closeout complete | The purpose-named `handbook-contracts` owner, definition/claim/applicability kernel, sealed Engine authority admission, lifecycle chain, durable repository use cases, active-after-independent-lock basis, exact four-operation outward SDK membrane, generic SDK/CLI/Tauri consumer proof, and final `4 / 62 / 23 / 135` integrated proof are current. All six packets, 16 acceptance conditions, aggregate verification/review, and the single slice closeout are complete. | Phase-4 exit, satisfied |
| [`HCM-5.2` — Evidence, verdict, and gate engine](slices/HCM-5.2/README.md) | Implemented checkpoint; slice closeout complete | `handbook-contracts` owns protocol-neutral untrusted-candidate admission, immutable-all canonical evidence, closed verdict/accounting/currentness/freshness/Resolution semantics, exact gate/score composition, and a sibling local-closeout result while parent promotion remains later. Packets 1–5 establish the canonical identity/policy/run kernel, sealed exact read-only candidate-source normalization, retained-root durable canonical-evidence repository, pure closed claim-verdict evaluation, and gate/local-closeout composition. Packet 6 maps the five exact owner use cases through the SDK and closes the current `62 / 28 / 157` inventory, capability, and schema truth without an adapter-specific route. Packet 7 proves all five through the Rust SDK and unchanged generic CLI/Tauri consumers plus the integrated failure, secrecy, source-gate, and deterministic-count boundary. All 17 acceptance conditions, aggregate verification/review, and the single slice closeout are complete. | HCM-5.1 slice closeout, satisfied |
| [`HCM-5.3` — Process dock protocol](slices/HCM-5.3/README.md) | Packets 1–5 implemented; aggregate closeout pending | The language-neutral one-shot bounded JSON process contract, purpose-named first-party Rust dock owner/executor, exact native bundle/launch identity, direct no-shell execution, evidence-matched portable controls, completed-only candidate handoff, three reserved operations, 11 new schema roots, and unchanged generic Rust SDK/CLI/Tauri consumers are current at exact `62 / 31 / 168` truth. Production manifest inventory remains exactly empty. Aggregate verification/review and the single HCM-5.3 closeout remain separate; HCM-5.4 is unprepared and unimplemented. | HCM-5.2 slice closeout, satisfied |
| `HCM-5.4` — First real validator dock proof | Planned; authority unprepared | Install and exercise the already-selected `handbook.dock.json-schema@1.0.0` native-Rust validator over the existing `jsonschema` crate and exact offline Draft 2020-12 local-closure practices, then carry completed candidate observations through Contracts-owned evidence, verdict, gate, and local closeout without granting the validator or executor semantic authority. No HCM-5.4 authority or implementation is current. | HCM-5.3 slice closeout |

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
| CLI operation-inventory expansion beyond the completed 62-row predecessor | HCM-4.3 does not expand the inventory for transport parity: its authority fixes the exact ordinary mappings and a closed six-entry SDK use-case registry. Any new semantic operation or use case requires later explicit authority, complete schemas, and independent owner admission where applicable. |
| Gate-to-posture input and policy boundary | HCM-5.2 fixes gate and local-closeout truth without posture/promotion fields; future posture/promotion authority must separately decide whether and how to consume those results |
| Third-party dock admission and stronger host confinement | Later explicit authority after the HCM-5.3/HCM-5.4 repository-owned first-party path; neither prepared slice authorizes arbitrary-code execution or claims a general hostile-code boundary |
| CLI/Tauri membrane routes and framing | HCM-4.3 owns current CLI parity and framing; HCM-4.4 separately owns the Tauri facade after HCM-4.3 closeout. |
| Publication mechanism, exact versions, bridge, and external adopter | Future publication and consumer authorities with current external proof |

Use this map to route a future slice author to the proper predecessor and
decision boundary. Then create separate, explicit slice authority before
execution. Update this map when a current decision changes ordering, status, or
exit state; update the linked owner document instead when only product truth,
architecture rationale, or seam routing changes.
