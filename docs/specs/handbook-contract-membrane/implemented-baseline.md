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
- determine HCM-4.4 lifecycle status or authorize follow-on work.

The [`README`](README.md) remains the authority and navigation entry point.
Execution-unit identity comes from the [`work-unit lifecycle`](work-unit-lifecycle.md), and verification and closeout rules come from the [`review and closeout policy`](review-and-closeout-policy.md).

## Status vocabulary

- **Implemented:** current tracked product source provides the capability, with current tests or direct executable anchors supporting that classification.
- **Precursor-only:** useful product components exist, but a required ordinary consumer, owner, integration, or completion boundary is not present.
- **Absent/future:** the named capability is not implemented in the current tracked product. This label records absence; it does not authorize the work.
- **External-unknown:** repository state cannot establish whether an external publication, deployment, bridge, or adoption exists today.

These are product-state labels only. None is a lifecycle state, authorization, phase-exit verdict, or promise of future scope.

## Verified domain baseline

| Domain | State | Current boundary |
| --- | --- | --- |
| Artifact, profile, schema, kind, and instance foundations | Implemented | The engine exposes registries, profiles, schema validation, and instance behavior through current source and focused tests under [`crates/engine`](../../../crates/engine/). |
| Context Resolution | Implemented | The current engine includes the resolution kernel, registry, typed policy loading, stack behavior, and focused tests such as [`context_resolution_kernel.rs`](../../../crates/engine/tests/context_resolution_kernel.rs). |
| Deterministic Projection and Snapshot components | Precursor-only | Deterministic projection and snapshot-memory components exist, including bounded capture, consistency, and delta behavior, but the complete ordinary/public consumer boundary is not established. |
| Grounding and Flow adapters | Precursor-only | Typed grounding, delta summaries, transition materialization, and Flow adoption components exist; evaluation evidence remains unavailable and there is no ordinary grounding consumer. |
| Atomic posture authority | Implemented | One-leaf compare-and-swap transition, private records, idempotent replay, persistence/recovery, engine facade, and direct typed SDK ingress are present. Broader posture-policy behavior is not. |
| SDK and CLI composition | Implemented | The transport-free SDK and CLI compose current owner capabilities, including generic pipeline routes. Every supported nontrivial CLI leaf now has one typed ordinary-operation or fixed use-case source for human and machine rendering. |
| HCM-4.2 shared DTO and JSON Schema membrane | Implemented | Packets 0–5 provide bounded contract types, canonical serialization and fingerprints, a deterministic 53-root Draft 2020-12 schema tree and manifest, the exact 62-row operation inventory and independent owner-admission boundary, immutable production bootstrap/catalog witnessing, one admitted Rust-SDK posture operation, and integrated proof. |
| Complete CLI JSON parity | Implemented | HCM-4.3 Packets 1–8 provide one global `--json` invocation envelope, 36-leaf exact coverage, six fixed use cases, 18 `cli_json` ordinary operations, a deterministic 115-root schema/manifest tree, singular writable-stdout framing, shared terminal/exit mapping, physical write-failure handling, and bounded `--legacy-json` compatibility. Aggregate slice closeout remains a lifecycle decision, not a product-state label. |
| Tauri-ready command facade | Implemented | HCM-4.4 Packets 1–6 provide the non-UI `handbook-tauri` library, exact three-command registry, five new SDK facade roots, 120-root deterministic schema/manifest tree, host-managed repository context, all 19 current capability routes, and six fixed non-discoverable use cases. The 18 CLI operations expose exactly `[cli_json, tauri]`, posture exposes `[rust_sdk, tauri]`, and the other 43 inventory rows remain transport-empty and omitted. Aggregate slice closeout is recorded by the current HCM lifecycle/status sources; that verdict does not change this product-state label. |
| Future contract/evidence/gate owner and dock executor | Absent/future | No current product owner or implementation establishes these capabilities. |
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
Packet 5 remains fail-closed precursor-only:
`GroundingEvidence::local_closeout` and `parent_promotion` both report
`Unavailable`; no future evidence/verdict/gate owner is inferred here.

Packet 6 has later generic SDK and CLI foundations, including pipeline
composition, but no ordinary grounding consumer is present.
The repository also does not establish a current grounding publication, bridge,
or Substrate adoption seam. Those external conditions remain external-unknown.

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
