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

## Current delivery sequencing

The human selected this delivery boundary on 2026-10-06: the current stage
focuses on native semantic-owner Rust mechanisms and SDK features, refinements
and usability. **SDK-first** includes the necessary Engine, Contracts and Dock
implementation; semantic authority stays with those owners.

[independent-local-check / Packet 3](slices/independent-local-check/packet-3.md)
plans a **complete native SDK check**: typed preparation, execution/closure,
recovery and historical reads, usable supplied inputs and structured owner
outcomes, SDK usage/response/recovery documentation, and ordinary normal-dependency
consumer examples/tests. It remains unimplemented pending human plan approval.
Supplied attributable human approvals, actual required review, owner validation,
authority and result explanation remain requirements of this stage.

Product CLI routes, installed-skill workflow instructions, adaptive agent-led
intake and a tested end-to-end agent-guided CLI/skill experience for these new
capabilities are **later, separately bounded consumer-workflow work**, once the
relevant native/SDK capability is sufficiently complete. The approved eventual
intake/delegation semantics remain the target. Later human-directed preparation
selects readiness, adopters and implementation; this is neither an undefined
requirement to finish the entire SDK nor a new phase/packet allocation. Packet 4
retains native/SDK repeat, correction and bounded caller-carried authorized
project-agent decision submission; it does not inherit all deferred consumer work.
The [coverage carry-forward](coverage-and-landing-map.md#local-check-planning-and-host-integration-carry-forward)
tracks that work alongside the existing Phase-6 adoption boundaries and hard order.

Delivered CLI, installed root skill, generic discovery and non-UI Tauri consumers,
and historical phase/slice exits remain intact; they do not establish the new
local-check route. Product skills here are distinct from the development and
orchestration skills used to build Handbook; that engineering process continues.

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

- Gate-owned local-closeout evaluation formerly associated with the grounding
  slice moved to the later evidence/verdict/gate owner and is now implemented
  by HCM-5.2. No parent-eligibility result or consumer is implemented there. The
  proposed [operational-reliance and work-progression decision](semantic-model/operational-reliance-and-actions.md#reliance-actions)
  assigns greenfield semantic responsibility for local-closeout and parent
  eligibility to Work Progression, without approving or scheduling physical
  placement, a work hierarchy/state model, exact parent binding, concrete policy,
  consumer, action/mutation path, or implementation slice.
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
| [`HCM-4.6` — Production snapshot grounding owner and ordinary SDK seam](slices/HCM-4.6/README.md) | Implemented checkpoint; slice closeout complete | Packets 1–6 and one aggregate review establish the Engine semantic/persistence owner, durable immutable snapshot repository and session-boundary lifecycle, five bounded current adapters over 15 slots, exactly four reserved ordinary rows, ten schema roots, one fixed non-discoverable SDK composition, and unchanged generic SDK/CLI/Tauri proof at exact `62 / 35 / 180 / 1 / 7` truth. The other four reserved rows remain absent and Phase-6 adopter selection is separate. | Phase-5 exit, satisfied |

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

HCM-4.6 is a completed post-Phase-5 follow-up over residual precursor work. Its
implemented checkpoint does not retroactively change the completed Phase-4 or
Phase-5 exit claims and does not select a Phase-6 adopter.

## Phase 5 — contract membrane and docks

Owner and executor remain separate: contract, evidence, verdict, and gate owners
decide semantic truth; a dock executes a bounded process and emits operational
records plus untrusted candidates.

| Slice | Program status | Concise outcome and boundary | Hard predecessor |
| --- | --- | --- | --- |
| [`HCM-5.1` — Contract lifecycle and claim model](slices/HCM-5.1/README.md) | Implemented checkpoint; slice closeout complete | The purpose-named `handbook-contracts` owner, definition/claim/applicability kernel, sealed Engine authority admission, lifecycle chain, durable repository use cases, active-after-independent-lock basis, exact four-operation outward SDK membrane, generic SDK/CLI/Tauri consumer proof, and final `4 / 62 / 23 / 135` integrated proof are current. All six packets, 16 acceptance conditions, aggregate verification/review, and the single slice closeout are complete. | Phase-4 exit, satisfied |
| [`HCM-5.2` — Evidence, verdict, and gate engine](slices/HCM-5.2/README.md) | Implemented checkpoint; slice closeout complete | `handbook-contracts` owns protocol-neutral untrusted-candidate admission, immutable-all canonical evidence, closed verdict/accounting/currentness/freshness/Resolution semantics, exact gate/score composition, and a sibling local-closeout result while parent promotion remains later. Packets 1–5 establish the canonical identity/policy/run kernel, sealed exact read-only candidate-source normalization, retained-root durable canonical-evidence repository, pure closed claim-verdict evaluation, and gate/local-closeout composition. Packet 6 maps the five exact owner use cases through the SDK and closes the current `62 / 28 / 157` inventory, capability, and schema truth without an adapter-specific route. Packet 7 proves all five through the Rust SDK and unchanged generic CLI/Tauri consumers plus the integrated failure, secrecy, source-gate, and deterministic-count boundary. All 17 acceptance conditions, aggregate verification/review, and the single slice closeout are complete. | HCM-5.1 slice closeout, satisfied |
| [`HCM-5.3` — Process dock protocol](slices/HCM-5.3/README.md) | Implemented checkpoint; slice closeout complete | The language-neutral one-shot bounded JSON process contract, purpose-named first-party Rust dock owner/executor, exact native bundle/launch identity, direct no-shell execution, evidence-matched portable controls, completed-only candidate handoff, three reserved operations, 11 new schema roots, and unchanged generic Rust SDK/CLI/Tauri consumers are current at exact `62 / 31 / 168` truth. Production manifest inventory remains exactly empty at that checkpoint. All five packets, all 16 acceptance conditions, aggregate verification/review, and the single HCM-5.3 closeout are complete. Current HCM-5.4 authority is recorded separately below. | HCM-5.2 slice closeout, satisfied |
| [`HCM-5.4` — First real validator dock proof](slices/HCM-5.4/README.md) | Implemented checkpoint; slice closeout complete | The selected `handbook.dock.json-schema@1.0.0` native-Rust validator uses the pinned offline Draft 2020-12 local closure, one reproducibly re-frozen current-target package, and the existing ordinary Dock/Contracts path. Packet 5 proves positive `pass / passed / eligible` and negative `fail / blocked / blocked` results without moving semantic authority; Packet 6 finalizes the two checked-in roots and unchanged generic SDK/CLI/Tauri consumers at exact `62 / 31 / 170 / 1` truth with six fixed use cases. All 19 acceptance conditions, aggregate verification/review, and the single slice closeout are complete. | HCM-5.3 slice closeout, satisfied |

Phase 5 may exit only when an exact active contract drives a real selected-dock
path through admitted evidence, verdict, and gate; missing, stale, malformed,
substituted, under-resolved, timed-out, cancelled, failed, protocol-invalid,
cleanup-uncertain, or durability-uncertain inputs fail closed; and the ordinary
contract/dock use cases exist behind SDK types.

### Phase-5 exit state

**Phase 5 is exited.** One exact active contract drove the selected production
Dock path; completed candidates crossed immutable Contracts-owned evidence,
verdict, gate, and local closeout without moving semantic authority; all named
failure classes failed closed; and ordinary contract/Dock use remains behind
SDK types and unchanged generic consumers at exact `62 / 31 / 170 / 1` truth
with six fixed use cases. This exit does not authorize publication, third-party
Dock support, external adoption, stronger confinement, or later-phase work.

## First approved-semantics foundation increment

After human review of the [coverage and landing map](coverage-and-landing-map.md),
the human selected one bounded admission-governance establishment preparation.
This descriptive placement is separate from numbered phases and does not reopen
Phase-3/4/5 exits or HCM-4.6, replace the Phase-6 order, or turn the map's five
provisional areas into phases or packets.

| Slice | Program status | Concise outcome and boundary | Hard predecessor |
| --- | --- | --- | --- |
| [`admission-governance-establishment` — Owner-native admission-governance establishment](slices/admission-governance-establishment/README.md) | Implemented locally at `aa793c1`; single-packet slice complete, awaiting human review | Establishes one new local project's exact Contracts freshness-policy scope, native advisory establishment/committed activation, truthful refusal/recovery and exact resolution/replay through a typed Rust SDK consumer. Caller-supplied ceilings retain exact identities through restart. Ordinary policy admission, successor activation and existing-project enrollment remain excluded. | Human authorization, clean reconciliation review and live identity/durability/owner/SDK compatibility were satisfied; positive native registration is delivered inside this increment. |

The [slice authority](slices/admission-governance-establishment/README.md) owns the
proposed concrete owner, rules, registration, retention, consumer and verification
choices. Its source-backed route avoids Context Resolution's generic-artifact
authority lifecycle; that separate correction is not an exercised hard predecessor
and repository-wide lifecycle confinement is not claimed. Detailed successor
planning and implementation remain subject to later human direction.

### HR-007 semantic landing — 2026-10-04

The human approved and integrated the [governance amendment](semantic-model/amendments/governance-defaults-and-delegated-operation.md).
Subsequent foundation preparation must use its intrinsic-establishment versus
project-qualification split, positive deferral/later establishment, delegated
operation, and protocol-qualified prior-result use. The skill-guided agent owns
adaptive intake and recommendations; deterministic owners validate and commit
requests through CLI/SDK. These are approved target semantics, not delivered APIs.

The fresh-project slice above remains its exact completed historical outcome.
The user subsequently authorized program-level planning reconciliation across
the roadmap and adjacent documents. The [coverage map](coverage-and-landing-map.md#impact-and-dependency-coverage)
records its breadth and the source-backed next outcome below. It remains proposed
for human review. The scoped-deferral outcome below subsequently landed with
meta closeout and human recap complete. The human then selected bounded preparation
of later first establishment. Its deferred-governance-establishment / Packet 1 implemented the local
owner/typed-SDK transition at `9f28f99`; the linked packet records evidence and
limits. Main has confirmed D→G aggregate validation/closeout and human recap
complete; this is supplied meta evidence, not new testing by this reconciliation.
Phase numbering and Phase-6 order remain unchanged.

| Foundation slice | Program status | Concise outcome and boundary | Dependency boundary |
| --- | --- | --- | --- |
| [`scoped-governance-deferral`](slices/scoped-governance-deferral/README.md) — Positive scoped deferral and current resolution | Implemented locally at `8b13505`; single-packet aggregate closeout, meta validation and human recap complete | One newly registered local project retains human-approved deferral of the existing Contracts freshness-policy admission scope and resolves/replays its exact basis through typed SDK. No default activation, global exemption or independent evaluation is inferred; missing/corrupt/uncertain state fails closed. | Existing identity/durability, fresh-project owner and typed SDK were foundations; scoped deferral and narrow initialization coordination landed inside the slice. The completed evidence carries an unattributed broader Windows-suite failure; focused targets passed. This is no successor authorization. |
| [`deferred-governance-establishment`](slices/deferred-governance-establishment/README.md) — First governance on the same deferred project | deferred-governance-establishment / Packet 1 implemented locally at `9f28f99`; Main-confirmed aggregate validation/closeout and human recap complete | Establish one exact newly approved configuration/terms against current never-established D on P/S; retain original history/replay, exact current G, truthful concurrency/recovery and typed SDK results. | Positive deferral at `8b13505` was the foundation; the packet source/evidence records delivered D→G/v3 and inherited limits. Actual runtime G still needs approved configuration/terms and exact current D. Ordinary roots, replacement and independent-use closure remain excluded. |
| [`independent-local-check`](slices/independent-local-check/README.md) — Complete independent offline JSON conformance | independent-local-check / Packet 1 delivered at `fccaf11` and human-reviewed; Packet 2 delivered at `5bf0f3b` with causal APPROVE and meta validation/recap complete; Packet 3 prepared for human implementation review; slice unfinished | Native initial setup/local Engine authority is extended with complete independently usable Contracts definitions/dependencies/permitted use, responsible D/G applicability and local author/lifecycle/native Dock source authority. Complete check, repeat/correction and bounded decision submission remain later packets. | [Packet 3](slices/independent-local-check/packet-3.md) builds on delivered Packet-2 definitions/source authority and existing owners/validator. Its bounded implementation plan requires human approval. D→G supplies a fresh-G discriminator; choosing D use does not require G establishment. Full Substrate adoption remains separate. |

The [next-outcome boundary](coverage-and-landing-map.md#proposed-next-slice-shape-scoped-governance-deferral)
owns the high-level example, refusal and observable acceptance conditions.
For the selected defer-first journey, positive deferral supplies the basis consumed
by later first establishment on that identity. Positive applicability resolution
is the general prerequisite for dependent independent use: deferral, established
non-applicability and satisfied applicable qualification are alternative G4 bases.
Independent use still needs permitted use, conformance, complete requirements and
supported operating authority.
The earlier authorization covered bounded later-establishment preparation.
The human has subsequently authorized slice preparation for
[`independent-local-check`](slices/independent-local-check/README.md): one complete offline JSON payload/schema check
through the typed SDK and native Contracts/Dock owners, under initial bounded
delegation, including fresh repetition/permitted correction and meaningful
out-of-bounds refusal. The [reviewed brief and subsequent human clarification](../../research/independent-local-check-and-initial-delegation-brief-2026-10-04.md)
are required planning inputs. The current plan outlines four separately reviewable
packet outcomes. [independent-local-check / Packet 1](slices/independent-local-check/packet-1.md)
has delivered native initial setup/local Engine authority at `fccaf11` and was
human-reviewed. The human then requested the next bounded plan/status correction.
The human approved [independent-local-check / Packet 2](slices/independent-local-check/packet-2.md)'s
bounded implementation, verification, causal review and local commit. Its local
implementation at `5bf0f3b` returned PACKET_COMPLETE with final fresh causal APPROVE;
meta validation/recap are complete. The human reviewed the takeover recap and
requested [Packet 3](slices/independent-local-check/packet-3.md) planning and stale
updates. Its first complete real-check plan is prepared for human implementation
review. No successor execution is authorized; Packet 4 stays a broad outcome.
The human has selected [local-advisory authority](semantic-model/amendments/approval-authority-and-assurance.md)
for the complete workflow, including later Contracts lifecycle and repeated source
admission/revalidation; hardware/authenticator acquisition is not a predecessor.
The [credential baseline](implemented-baseline.md#credential-verification-and-the-selected-local-route)
distinguishes retained verification mechanisms from unproven device integration
and the delivered initial local route versus unfinished complete workflow.
Existing credential-required rules acquire
no automatic fallback.
No runtime project bounds, assurance provider, or general profile system is selected
by this preparation authorization. Other outcomes remain separately bounded.

Planning must carry forward agent-to-agent escalation under prior delegated
authority, additional role-specific context, and conditional human escalation.
Prioritize reuse of verified Substrate facilities while preserving standalone
Handbook use and native owner authority. General agent-profile/routing delivery
and full Substrate adoption are not automatic predecessors of the local check;
identify any actual causal dependency and track deferred work explicitly. The
[coverage entry](coverage-and-landing-map.md#local-check-planning-and-host-integration-carry-forward)
connects this preparation to the later consumer-adoption work.

## Phase 6 — consumer adoption

Repository evidence does not prove present publication, an operating bridge, or
Substrate adoption. Each external claim requires current evidence from the
responsible external owner or system.

For agent-profile/context/routing integration, the human prioritizes Substrate
reuse while retaining standalone Handbook operation. Agent profiles are distinct
from Engine operating profiles and their selected policies; host routing does not
grant native authority. Carry the [local-check clarification](../../research/independent-local-check-and-initial-delegation-brief-2026-10-04.md#subsequent-human-clarification-agent-escalation-and-substrate-integration)
into the relevant adoption/adapter scope. This preference neither proves an
external integration nor changes the hard predecessor order below.

The [current delivery sequencing](#current-delivery-sequencing) keeps new local-check
CLI/skill/agent workflow delivery separately bounded after sufficient native/SDK
capability. No full Substrate integration prerequisite or Phase-6 reorder follows.

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

Selected consumer workflows must preserve HR-007's scoped applicability,
qualification, original result provenance and adopted assessment protocols from
their native owners. The [coverage reconciliation](coverage-and-landing-map.md#impact-and-dependency-coverage)
records onboarding, feedback and adapter implications. It adds no universal
SDK/CLI/Tauri parity requirement or skill-family pack/integration catalog.

## Primary unresolved decisions

| Decision | Where later authority must decide it |
| --- | --- |
| External grounding trigger/adopter placement | HCM-4.6 implements the bounded owner, repository, four-operation seam, fixed SDK composition, and unchanged generic repository consumers. Future Phase-6 authority must separately choose any publication, bridge, automatic trigger host, and real external adopter. |
| Broader posture evaluation, recommendation, notification, acknowledgement, and hysteresis | Future posture authority must place this policy without transferring engine-owned mutation authority to a gate or adapter |
| CLI operation-inventory expansion beyond the completed 62-row predecessor | HCM-4.3 does not expand the inventory for transport parity: its authority fixes the exact ordinary mappings and a closed six-entry SDK use-case registry. Any new semantic operation or use case requires later explicit authority, complete schemas, and independent owner admission where applicable. |
| Parent eligibility, work hierarchy/state, and dependency-composition realization | HCM-5.2 truthfully fixes gate and local-closeout results without a parent field or consumer. The approved [operational-reliance and work-progression semantics](semantic-model/operational-reliance-and-actions.md#reliance-actions) select Work Progression as the greenfield semantic owner of local/parent eligibility and require separate atomic Operational Reliance Determinations, with prospective assessment or adopted HR-007 current qualification of exact prior results. Later human-approved authority must still select the physical component, work hierarchy/state model, exact parent/child/dependency binding, concrete policies/currentness inputs, consumers, action authorization/mutation mechanisms, implementation slice, and rollout. Local closeout cannot imply parent eligibility or mutation. |
| Gate-to-posture observation, recommendation, reliance, and transition realization | HCM-5.2 fixes gate/local-closeout truth without posture fields. The approved [posture and assurance-policy semantics](semantic-model/assurance-and-evaluation-preparation.md#posture-assurance) fix the forward-only semantic order through admitted observation/recommendation, transition-specific Operational Reliance, separate Action Intent/Authorization, and atomic Engine mutation. Initial skill-guided advice is a separate responsibility. Later authority must still select concrete policies, algorithms, approvals, physical placement, schemas/APIs, host integration, implementation, and rollout without transferring gate or mutation authority to Contracts, SDK, or an adapter. |
| Additional Dock integrations with established tooling | Later explicit authority must select each integration's scope and observation-admission requirements beyond the HCM-5.3/HCM-5.4 repository-owned JSON Schema path; such adapters do not imply arbitrary third-party executable admission or a general hostile-code confinement program |
| CLI/Tauri membrane routes and framing | HCM-4.3 owns current CLI parity and framing; HCM-4.4 separately owns the Tauri facade after HCM-4.3 closeout. |
| Publication mechanism, exact versions, bridge, and external adopter | Future publication and consumer authorities with current external proof |
| CLI, installed-skill and agent workflows for new native/SDK capabilities | Later human-directed consumer-workflow preparation under [current delivery sequencing](#current-delivery-sequencing), with readiness/adopter decisions and scope still to be selected. The [carry-forward](coverage-and-landing-map.md#local-check-planning-and-host-integration-carry-forward) tracks local-check intake, response handling and routing; this is no default assignment to Packet 4 or new phase. |

Use this map to route a future slice author to the proper predecessor and
decision boundary. Then create separate, explicit slice authority before
execution. Update this map when a current decision changes ordering, status, or
exit state; update the linked owner document instead when only product truth,
architecture rationale, or seam routing changes.
