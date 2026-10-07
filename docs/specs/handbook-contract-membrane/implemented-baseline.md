# Handbook Contract Membrane Implemented Baseline

This document classifies the current Handbook contract-membrane product state
at the tracked product and documentation HEAD. It is a compact implementation
checkpoint, not a plan.

## Scope and authority

This baseline owns the program-level answer to one question: what is implemented, precursor-only, absent/future, or external-unknown now?
The tracked source, product assets, and tests remain the detailed product truth; if this prose conflicts with them, follow the drift rule below.

This document does not:

- authorize a slice, packet, or implementation task;
- define a remaining schedule or choose a future owner;
- declare a phase or slice complete;
- select a Phase-3 boundary or establish Phase-3 exit evidence; or
- determine lifecycle status or authorize follow-on work.

The [`README`](README.md) remains the authority and navigation entry point.
Execution-unit identity comes from the [`work-unit lifecycle`](work-unit-lifecycle.md), and verification and closeout rules come from the [`review and closeout policy`](review-and-closeout-policy.md).

## Status vocabulary

- **Implemented:** current tracked product source provides the capability, with current tests or direct executable anchors supporting that classification.
- **Precursor-only:** useful product components exist, but a required ordinary consumer, owner, integration, or completion boundary is not present.
- **Absent/future:** the named capability is not implemented in the current tracked product. This label records absence; it does not authorize the work.
- **External-unknown:** repository state cannot establish whether an external publication, deployment, bridge, or adoption exists today.

These are product-state labels only. None is a lifecycle state, authorization, phase-exit verdict, or promise of future scope.

## HR-007 semantic approval is not product delivery

The [governance amendment](semantic-model/amendments/governance-defaults-and-delegated-operation.md)
was approved on 2026-10-04. This documentation-only landing changes no product
classification. The completed [fresh-project establishment slice](slices/admission-governance-establishment/README.md)
remains narrow; the amendment's full independent-use path, later establishment,
replaceable selections and retrospective qualification are not established as
delivered by that slice. Adaptive skill-guided intake must remain distinct from
native setup and deterministic owner operations when preparing subsequent work.
The [program reconciliation](coverage-and-landing-map.md#current-source-constraints-on-the-next-outcome)
records the current source constraints. Ordinary setup does not record positive
deferral; Contracts has standalone credential-backed authority foundations without
requiring a Charter. The retained V1 path lacks the new complete-use closure;
the additive local definition/permitted-use/applicability/source boundary delivered
by Packet 2 is described below. The bounded full evaluation closure is implemented under the human-approved
Packet-3 plan at `2b09faf`, with selected acceptance verified, final causal delta
**APPROVE** and completed human/meta checkpoint. The human requested the bounded
[Packet-4 plan](slices/independent-local-check/packet-4.md); its missing workflow
additions remain unimplemented and require human implementation-plan approval.
The proposed [scoped-deferral shape](coverage-and-landing-map.md#proposed-next-slice-shape-scoped-governance-deferral)
subsequently landed as the bounded [deferral slice](slices/scoped-governance-deferral/README.md)
at `8b13505`: native initial scoped deferral, exact current resolution/replay,
retained history and typed SDK consumption are implemented. Ordinary setup still
does not record deferral; the full independent-use workflow remains unfinished.
[deferred-governance-establishment / Packet 1](slices/deferred-governance-establishment/packet-1.md)
at `9f28f99` implements first establishment on that same positive deferred identity
and authoritative v3 current resolution, retaining history and truthful recovery.
Ordinary-root enrollment, established replacement and broader qualification remain
unsupported. Main confirms its aggregate closeout/recap complete as supplied meta
evidence; no suite was rerun here. The deferral slice's broader Windows owner-suite
limit remains recorded rather than cleared by focused passes.

The [current delivery sequencing](phase-and-slice-map.md#current-delivery-sequencing)
distinguishes current native/SDK work from later CLI/installed-skill/agent workflows
for the new local-check capabilities. Packets 1–2 expose typed SDK routes; the
ordinary [SDK example](../../../crates/sdk/examples/local_contract_check.rs) is
consumer proof, not a new Handbook CLI command or installed adaptive-intake workflow.
Packet 3 implements complete native SDK checking with usage/response/recovery
docs and verified selected acceptance, with final causal delta **APPROVE** and no
unresolved P1/P2.
Delivered generic CLI, root skill and non-UI Tauri capability
proof remains valid without establishing this new local-check route.

## Credential verification and the selected local route

Focused source inspection at `6a15ef68ab751383187d91e296bcf187dfda2dc1` on
2026-10-05 distinguishes the implemented mechanism from device integration and
the selected workflow. independent-local-check / Packet 1 subsequently adds the bounded native local route
described below. This section supersedes any inference that all
credential code is unused, hardware integration is complete, or local advisory
use is already supported across these paths. Historical checkpoints retain
their original scope.

| Boundary | Current implementation and evidence limit |
| --- | --- |
| Credential-verification mechanism — implemented, test-covered | Engine [authenticator code](../../../crates/engine/src/charter_authenticator.rs) encodes CTAP make-credential/get-assertion requests, strictly decodes responses and verifies ES256 assertions against the registered key, exact descriptor and RP hash. [Credential admission](../../../crates/engine/src/authority_admission.rs) binds action/subject/class/scope, repository identity and current registry/head, checks counters and consumes lifetime-sealed authority while its guards hold. [Transcript/security tests](../../../crates/engine/tests/hcm_2_2_authenticator_security.rs) and [admission tests](../../../crates/engine/tests/hcm_5_1_authority_admission.rs) cover this actual scope; their bodies were inspected, not rerun here. |
| Shipped selected assurance — credential-backed, not proof of hardware provenance | The [constitutional approval policy](../../../crates/engine/definitions/approval-policies/handbook.approval.constitutional-candidate/1.0.0.yaml) selects `fido2-ctap2.1-native`, required user presence/verification, ES256, `attestation_policy: none` and forbidden fallback. Verification requires literal assertion flags `0x05`. These checks do not establish device/manufacturer provenance; “hardware-verified” overstates the policy's proof. |
| End-to-end authenticator integration — incomplete/unproven | `NativeAuthenticatorPortV1` is caller-supplied. [Charter SDK default dispatch](../../../crates/sdk/src/charter_product.rs) supplies `UnavailableNativeAuthenticatorPortV1`; [Contract SDK default dispatch](../../../crates/sdk/src/contract_membrane/contract_operation.rs) supplies `UnavailableAuthenticator`, with a public supplied-port seam. Inspected trait implementations provide test/fixture signing, unavailable ports and a borrowing adapter; no production device transport was found. Success [Dock fixture ports](../../../crates/dock-json-schema/tests/packet5_support.rs) synthesize responses with test keys. This source/test coverage does not prove real device acquisition or external host integration. |
| Current credential-dependent callers — live requirements | The [approver registry](../../../crates/engine/src/approver_registry_mutation.rs), [Charter approval](../../../crates/engine/src/charter_approval_workflow.rs) and [promotion](../../../crates/engine/src/charter_promotion_workflow.rs) retain their selected registry/approval lineage. Contract SDK append/transition bridges use credential admission; [Contracts lifecycle](../../../crates/contracts/src/lifecycle.rs) retains its selected distinct-author lock and active-basis checks. [Context Resolution](../../../crates/engine/src/context_resolution.rs) uses mapped assertions and current native authority. [Dock source resolution](../../../crates/dock/src/store.rs) uses [Engine evidence-source admission](../../../crates/engine/src/evidence_source_admission.rs), which invokes credential admission; [evidence append](../../../crates/contracts/src/owner_repository/evidence_repository.rs) and [verdict revalidation](../../../crates/contracts/src/claim_verdict.rs) can resolve sources repeatedly. This code is not all unused, and new checks can require multiple assertions. Contract list/read and the standalone validator do not thereby become credential-dependent operations. |
| Selected local-advisory workflow — complete native check delivered; repeat/correction/resolution workflow unfinished | [Packet 1](slices/independent-local-check/packet-1.md) at `fccaf11` supplies attributable initial local Charter/posture/terms and current-D Engine envelope checks, restart/recovery and wider/stale/revoked refusal. Its current-D boundary is not a permanent G shutdown. [Packet 2](slices/independent-local-check/packet-2.md) at `5bf0f3b` supplies immutable definitions/permitted use, actual distinct review/lifecycle, responsible fresh D/G qualification and sealed native Dock source admission/revalidation; its source-seam result is not complete-check truth. [Packet 3](slices/independent-local-check/packet-3.md) at `2b09faf` delivers complete Coordination, owner Purpose/occurrence/plan, independent Engine issuance/checkpoints, native one-attempt execution, closed evidence/context and immutable explained result. Original-request/authorization/attempt recovery, equal-content fresh occurrence identity, bounded precommit readiness reassessment and exact result history after later G/revocation already exist. Selected acceptance, final causal delta APPROVE and human/meta checkpoint are complete. The [Packet-4 plan](slices/independent-local-check/packet-4.md) uses those primitives for deliberate-repeat/corrected-capture usability and adds the absent bounded caller-carried InputCorrection decision consumer with an exact prior-approved distinct-decider binding. Current unresolved context is not decision consumption; existing one-agent terms cannot infer a separate decision agent. These additions are planned, not delivered. Existing credential rules retain their meaning without automatic downgrade; local advisory claims no authenticated identity or independent review by itself. |

The earlier documentation reconciliation ran no product, device/provider or
external integration tests. Packet 2 records its focused product verification
and ordinary consumer in its authority file. Exact append replay can deliver
an old immutable effect without new admission; it is not fresh evaluation or
proof of full historical-result delivery. Local mode needs actual native owner
support, rather than reusing fixture authority or relabeling verified admissions.

## Verified domain baseline

| Domain | State | Current boundary |
| --- | --- | --- |
| Artifact, profile, schema, kind, and instance foundations | Implemented | The engine exposes registries, profiles, schema validation, and instance behavior through current source and focused tests under [`crates/engine`](../../../crates/engine/). |
| Context Resolution | Implemented | The current engine includes the resolution kernel, registry, typed policy loading, stack behavior, and focused tests such as [`context_resolution_kernel.rs`](../../../crates/engine/tests/context_resolution_kernel.rs). |
| Deterministic Projection and Snapshot components | Implemented | The Engine production snapshot-grounding owner provides bounded redacted capture, immutable durable records, exact predecessor/sequence selection, compatible delta, Resolution-bounded projection, currentness, retention, recovery, and fail-closed refusal semantics. The earlier HCM-3.3–3.5 fixture remains test-only. |
| Grounding and Flow adapters | Implemented | Flow and pipeline preserve the typed Engine grounding outcome without recomputation, and one fixed non-discoverable SDK session-boundary use case composes it. Automatic session triggering and an external Phase-6 adopter remain absent/unselected. Current HCM-5 evidence/gate truth remains separately owned. |
| Production snapshot-grounding owner and ordinary seam | Implemented | HCM-4.6 Packets 1–6 provide the production Engine owner, durable repository lifecycle, five private currentness adapters over 15 slots, exact four-operation and ten-root closure, one fixed SDK composition, and unchanged generic SDK/CLI/Tauri proof at `62 operations / 35 capabilities / 180 roots / 1 production manifest / 7 fixed use cases`. The other four reserved rows remain absent, and the external Phase-6 adopter is not selected. |
| Atomic posture authority | Implemented | One-leaf compare-and-swap transition, private records, idempotent replay, persistence/recovery, engine facade, and direct typed SDK ingress are present. Broader posture-policy behavior is not. |
| SDK and CLI composition | Implemented | The transport-free SDK and CLI compose current owner capabilities, including generic pipeline routes. Every supported nontrivial CLI leaf now has one typed ordinary-operation or fixed use-case source for human and machine rendering. |
| HCM-4.2 shared DTO and JSON Schema membrane | Implemented | Packets 0–5 provide bounded contract types, canonical serialization and fingerprints, a deterministic 53-root Draft 2020-12 schema tree and manifest, the exact 62-row operation inventory and independent owner-admission boundary, immutable production bootstrap/catalog witnessing, one admitted Rust-SDK posture operation, and integrated proof. |
| Complete CLI JSON parity | Implemented | HCM-4.3 Packets 1–8 provide one global `--json` invocation envelope, 36-leaf exact coverage, six fixed use cases, 18 `cli_json` ordinary operations, a deterministic 115-root schema/manifest tree, singular writable-stdout framing, shared terminal/exit mapping, physical write-failure handling, and bounded `--legacy-json` compatibility. Aggregate slice closeout remains a lifecycle decision, not a product-state label. |
| Tauri-ready command facade | Implemented | HCM-4.4 Packets 1–6 provide the non-UI `handbook-tauri` library, exact three-command registry, five SDK facade roots, and host-managed repository context. Subsequent Contracts, Dock, and HCM-4.6 packets extend the unchanged generic facade to all 35 current capability routes and seven fixed non-discoverable use cases over the deterministic 180-root schema tree. No snapshot-specific command or semantic branch is added. |
| Capability-driven installed Handbook skill | Implemented | HCM-4.5 Packets 1–6 provide one SDK consumer seam, exactly four generic CLI actions (`bootstrap`, `catalog-read`, `schema-read`, and `invoke`), and generated/source/release/public installed skill projections. The skill discovers the current 35-capability witness, retrieves exact manifest-bound schemas, invokes read-only or explicitly authorized mutations, and preserves typed terminal, idempotency, receipt, and replay truth without a capability-specific route. The deterministic tree now contains 180 schema roots over the unchanged 62 operation rows and seven fixed-use-case omissions. HCM-4.6 adds no skill or operation-specific consumer branch. |
| Contract definition, claim, applicability, lifecycle, and durable owner use cases | Implemented | HCM-5.1 Packets 1–3 provide the inward `handbook-contracts` owner with exact definition identity/fingerprints, closed claims and evidence declarations, applicability tri-state, descriptive compatibility/mutation matrices, the exact four-class lifecycle policy, immutable prior-bound transitions, one exact replaceable current head per definition, and active-after-independent-lock bases. Its fixed repository-relative owner store adds bounded current-only paging, exact immutable reads, credential-admitted idempotent append and compare-and-write transition, replay, concurrent-writer serialization, exact transaction inventories, and bounded fail-closed recovery. A narrow Engine seam supplies lifetime-sealed credential-backed authority admissions bound to the same retained physical repository root and exact current committed approver authority. |
| Outward contract SDK mapping | Implemented | HCM-5.1 Packet 4 projects the four owner use cases through bounded SDK DTOs and exact request/result/problem schemas, independently re-admits owner evidence, closes the four reserved inventory rows and generic dispatcher, and extends the deterministic manifest and production witness to 135 roots and 23 capabilities. The four operations carry current `rust_sdk`, `cli_json`, and `tauri` evidence without a command-specific adapter route. |
| Generic contract consumer proof | Implemented | HCM-5.1 Packet 5 proves the four exact operations through the existing Rust SDK, four-action generic CLI membrane, and three-command generic Tauri facade. Discovery/schema/invocation parity, deterministic list/read, authorized append receipts and replay/conflict, compare-and-write stale-head and lifecycle-basis refusal, substituted identities/roots, unavailable authority, singular CLI framing, generic Tauri envelopes, non-leakage, and transport non-authority are regression-protected without a contract-specific consumer branch. |
| Integrated contract product-state proof | Implemented | HCM-5.1 Packet 6 traverses owner-native definition/claim/applicability construction, sealed independent authority, durable append/replay/conflict/list/read/transition/current-head and active-basis truth, outward SDK mapping, the production witness, and generic transport dispatch. It fixes final deterministic truth at exactly four HCM-5.1 operations, 62 inventory rows, 23 capabilities, 135 schema roots, and `[cli_json, rust_sdk, tauri]` on each contract operation. Aggregate verification/review and the single HCM-5.1 closeout are recorded by current lifecycle/status sources; that does not alter this product-state classification. |
| Evidence/gate owner | Implemented | HCM-5.2 Packets 1–7 implement the owner-native candidate/evaluation/evidence/verdict/gate/local-closeout identity and policy kernel, atomic read-only run preparation, an exact repository-backed candidate observation source, sealed `contract_evidence_source` authority admission, contracts-owned lifecycle/freshness/Resolution/currentness rechecks, private candidate-to-canonical-evidence normalization, retained-root immutable-all canonical-evidence list/read/idempotent append with bounded atomic recovery, pure closed claim-verdict evaluation, gate/local-closeout composition, and an exact five-operation outward SDK membrane. The Rust SDK and unchanged capability-driven CLI/Tauri consumers prove all five operations, fail-closed/write-nothing behavior, response re-admission, secrecy, and exact deterministic truth at 62 inventory rows, 28 capabilities, and 157 schema roots. There is no public candidate writer or verdict persistence; all 17 acceptance conditions, aggregate verification/review, and the single slice closeout are complete. |
| Process dock protocol and executor | Implemented | HCM-5.3 Packets 1–5 provide `handbook-dock`, the exact one-shot canonical JSON process protocol, owner-admitted immutable manifests, first-party native bundle resolution, direct no-shell portable execution, bounded cancellation/cleanup accounting, atomic operational persistence/replay, completed-only candidate materialization, Contracts-owned source re-admission, the exact three-operation outward SDK membrane, 11 new roots, and unchanged generic SDK/CLI/Tauri consumption. Its closed checkpoint is exactly 62 operation rows, 31 capabilities, 168 roots, and an empty production manifest registry; HCM-5.4 extends that implemented base in the next row without changing the HCM-5.3 closeout record. |
| First production validator Dock proof | Implemented | HCM-5.4 Packets 1–6 add the purpose-named native Rust validator, exact offline Draft 2020-12 local-closure evaluator, reproducible current-target package, one production manifest, and the real Contracts-owned evidence/verdict/gate/local-closeout proof. The two checked-in validator input/fact roots extend current truth to exactly 62 operation rows, 31 capabilities, 170 schema roots, one production manifest, and six fixed use cases. The existing Rust SDK, four-action generic CLI membrane, and three-command generic Tauri facade discover, read, and invoke the path without a new route or semantic branch. All 19 acceptance conditions, aggregate verification/review, and the single HCM-5.4 closeout are complete; the current phase map records Phase-5 exit. That lifecycle fact does not authorize a stronger product claim. |
| Current publication and Substrate adoption | External-unknown | Current repository evidence does not prove a present external publication, bridge, or Substrate seam; absence from the repository is not proof of external absence. |

## Phase-3 implementation and residual facts

HCM-3.5 Packets 1 through 4 are present in the current product ancestry:

- `8ea607a` added bounded grounding resolution;
- `5be1766` added grounded Flow-packet adoption;
- `e32897a` added shared pipeline grounding inclusion; and
- `935c939` materialized transition references and snapshots; `50b16e3` later hardened source ingestion to fail closed.

The current grounding implementation is anchored in
[`grounding.rs`](../../../crates/engine/src/grounding.rs),
[`grounding_transition.rs`](../../../crates/engine/src/grounding_transition.rs),
and [`hcm_3_5_grounding.rs`](../../../crates/engine/tests/hcm_3_5_grounding.rs).
Packet 5 remains fail-closed precursor-only. The legacy-named
`GroundingEvidence::local_closeout` and `parent_promotion` accessors are inert
placeholders: both always report `Unavailable`, no current producer or consumer
supplies meaning through them, and their presence does not establish a current
parent-promotion implementation requirement. No future parent domain, binding,
policy, owner, or consumer is inferred here.

HCM-4.6 supersedes the precursor-only product classification with one production
Engine owner, four ordinary operations, and one fixed SDK session-boundary
composition through typed Flow/pipeline preservation. The repository still does
not establish an automatic trigger, grounding publication, bridge, or external
Substrate adopter. Those external conditions remain absent or external-unknown
as applicable.

HCM-3.6's private atomic transition is present through commit `1d4844f` and the
current engine source. The implementation provides:

- a single fixed-leaf compare-and-swap transition;
- private lifecycle, intent, result, and journal records;
- exact authority and approval checks;
- idempotent replay and refusal of conflicting reuse;
- persistence, recovery, and canonical-result verification; and
- the engine facade and SDK ingress described below.

Multi-source posture evaluation, `PostureEvaluationPolicy`, recommendation
generation, notification and acknowledgement, and hysteresis are absent/future.
The selected profile accepts only a null `posture_evaluation_policy` today.

The historical dead-code condition was superseded by the later facade and SDK
ingress. The phase-and-slice map now records Phase 3 exited under the
user-selected reduced boundary after fresh strict-Clippy, focused-regression,
and focused-review evidence. This baseline does not own or reproduce that exit
evidence; it continues to classify product state only.

## HCM-4.1 SDK boundary

Commit `6f7c78f` added the direct typed posture facade and SDK ingress.
Commit `28fa4ab` then retired the compiler composition boundary and consolidated
ordinary composition in the SDK and CLI.

[`HandbookSdkV1`](../../../crates/sdk/src/lib.rs) is transport-free and
repository-bound. Its posture method accepts bounded typed Rust input and returns
closed outcomes through the engine facade. The facade independently resolves
current authority, policy, profile, approvals, lifecycle validation, persistence,
and replay rather than trusting caller-supplied private records.

The focused anchor is
[`direct_typed_posture_api.rs`](../../../crates/sdk/tests/direct_typed_posture_api.rs).
This boundary does not provide a generic wire envelope or complete SDK/JSON/CLI
transport parity, and it does not establish the later membrane routes or owners.

## HCM-4.2 Packet 0: typed contract-membrane kernel

### Responsibility and provenance

Packet 0 supplies shared, owner-neutral Rust contract primitives for later
membrane surfaces. It bounds data before owners consume it, gives exact values a
canonical byte and fingerprint identity, and separates structural admission from
owner-trusted verification without implementing an ordinary operation surface.

It was implemented after the archived HCM-4.2 planning-only package.
Commit `8605d6539355f350c55b921d95fb450bca30ee18`,
`feat(sdk): add typed contract membrane kernel`, added the original product and
test boundary. Packet 1 later extended the module for schema descriptions and
generation while its focused regressions continued to protect Packet-0
behavior. Current source under
[`contract_membrane`](../../../crates/sdk/src/contract_membrane/)
and [`contract_membrane.rs`](../../../crates/sdk/tests/contract_membrane.rs)
is product truth.

The archive's statement that no runtime implementation existed is therefore
stale. The archive remains historical provenance and non-authority; this current
baseline records the implemented result instead of restoring stale planning.

### Public categories

The module's public surface groups:

- bounded strings, vectors, integers, timestamps, contract values, and limits;
- exact SemVer references, bindings, and SHA-256 fingerprints;
- portable artifact locators, digest bindings, and artifact references;
- shared terminal DTO fields, problems, diagnostics, omissions, actions,
  recheck conditions, correlations, receipts, and idempotency state;
- catalog roots, cursors, and pages with exact local closure;
- bounded raw idempotency keys and a sealed owner-consumer bridge;
- canonical JSON parsing, serialization, and fingerprinting; and
- sealed witnesses plus structural-to-trusted verification.

### Safety and invariant boundary

Construction and parsing are bounded and fail closed on invalid lexical forms,
non-canonical JSON or ordering, duplicate members, invalid invariants, stale or
substituted bindings, and fingerprint mismatch. Canonical serialization is the
shared fingerprint preimage rather than a display representation.

Raw idempotency keys are bounded, redacted in diagnostics, excluded from terminal
wire state, and exposed to owners only through the sealed borrowed bridge.
Verification first establishes structural validity, then uses sealed owner
witnesses to produce `Verified<T>`; structural admission alone is not trust.
Catalog values enforce bounded page/root/cursor relationships before witness use.

The module root
[`mod.rs`](../../../crates/sdk/src/contract_membrane/mod.rs)
is the concise export inventory. The focused test anchor exercises limits,
canonical form, exact identities, redacted errors, DTO fingerprints, catalog
closure, terminal truth tables, raw-key secrecy, and frozen public accessors.

### Explicit exclusions

Packet 0 does not implement:

- a schema tree, generator, or schema manifest producer;
- operation definitions or an operation registry;
- generic ordinary request, response, or admission envelopes;
- a bootstrap descriptor or producer, owner-admission catalog, or production witness;
- posture membrane mapping or a CLI membrane route;
- Tauri integration;
- a contract/gate/dock owner, publication, bridge, or Substrate adoption.

## HCM-4.2 Packets 1–5: completed shared membrane

Commit `f54dcbd3ec9a50ad586d7952cbec8ca6d508eefe`,
`feat(sdk): generate contract membrane schemas`, added the exactly pinned,
feature-gated Rust generator and the original 15 self-contained Draft 2020-12
roots. Later packets extended the same deterministic exact-byte tree to 53
roots without changing its generation or identity rules. Normal SDK builds do
not generate files, and the validator uses no resolver features.

The Packet-1 executable anchors are
[`schema_generation.rs`](../../../crates/sdk/src/contract_membrane/schema_generation.rs),
[`schemas/`](../../../crates/sdk/schemas/), and
[`contract_schema_generation.rs`](../../../crates/sdk/tests/contract_schema_generation.rs).
Commit `4d70701` added the exact 62-row operation inventory, generic ordinary
request/response/admission-refusal envelopes, structural schema admission, and
independent Engine/Flow/pipeline owner-token boundary. All rows default deny
until their complete definition, schemas, owner admission, write closure, and
current transport are proven.

Commit `241b998` added the immutable production witness, descriptor negotiation,
owner/schema/profile evidence, and typed capability/schema/profile roots,
pages, and cursors. It rejects stale, tampered, gapped, substituted, or
self-attested inputs before discovery.

Commit `caf0ef4` closed `posture.transition.apply@1.0.0` through exact request
and result schemas, Engine-native definition admission, sealed raw-key use,
exhaustive owner-result projection, compare-and-write receipts, and the current
Rust-SDK transport. It is the sole discoverable current capability; the other
61 inventory rows remain honestly omitted.

Commit `c1f5d33` added the three-group integrated proof covering deterministic
regeneration, identity-domain separation, owner/version parity, replay,
raw-key absence, catalog closure, and adapter non-authority. The HCM-4.2 slice
authority records the one aggregate closeout. No CLI or Tauri membrane route
was added.

## Current anchors and drift rule

Use these anchors without treating the list as an exhaustive file inventory:

- engine exports and owner composition: [`crates/engine/src/lib.rs`](../../../crates/engine/src/lib.rs);
- SDK composition and exports: [`crates/sdk/src/lib.rs`](../../../crates/sdk/src/lib.rs);
- generic pipeline SDK surface: [`pipeline_route_api.rs`](../../../crates/sdk/src/pipeline_route_api.rs);
- posture facade: [`posture_transition_facade.rs`](../../../crates/engine/src/posture_transition_facade.rs);
- Packet 0 exports and focused regression: the two anchors in its section above;
- complete HCM-4.2 exports and production witness:
  [`contract_membrane/mod.rs`](../../../crates/sdk/src/contract_membrane/mod.rs),
  [`production_witness.rs`](../../../crates/sdk/src/contract_membrane/production_witness.rs),
  and [`contract_packet5_integrated_proof.rs`](../../../crates/sdk/tests/contract_packet5_integrated_proof.rs);
- checked-in assets and schema-generation tests: the three Packet-1 anchors
  above;
- the non-UI Tauri adapter and integrated Packet-6 proof:
  [`crates/tauri/src/lib.rs`](../../../crates/tauri/src/lib.rs) and
  [`hcm_4_4_packet6.rs`](../../../crates/tauri/tests/hcm_4_4_packet6.rs); and
- retained executable product fixtures: [`slices/`](slices/), as classified by the README.

When source, tests, or product-consumed assets change a classification, update
this baseline in the same bounded change or record the discrepancy as drift.
Never use this prose to override newer executable truth. Never upgrade
external-unknown from repository inference alone. Historical material remains
non-authority unless a current authority explicitly adopts a bounded part.
