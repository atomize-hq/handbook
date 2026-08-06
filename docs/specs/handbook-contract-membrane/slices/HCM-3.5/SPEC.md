# HCM-3.5 Resolution-aware snapshot, packet, and pipeline adoption

**Status:** planning-only, authority-boundary candidate. This packet may add
documentation, planning, selector, proof, review-dispatch, handoff, ledger,
and the two coupled control-pack truth corrections named by its selector. It
does not authorize implementation, runtime checks, public API selection,
schema/dependency changes, configuration, consumer adoption, or remote work.

## Objective

HCM-3.5 is the first proposed adoption of the completed private Phase-3
primitives. A later implementation must make Flow consume an exact
`ContextResolutionEnvelope`, produce a bounded Resolution-aware grounding
Projection from an exact current snapshot and compatible delta, replace
pipeline's raw `work_level` filtering with namespaced shared semantics, record
session-start/session-end snapshot and delta references in parent handoffs, and
keep local closeout distinct from parent-promotion readiness.

This planning run must not infer how a published `handbook-engine` API exposes
the currently private Snapshot Memory and Projection modules. It records that
missing compatibility decision as a true authority boundary. Therefore this
packet can establish the implementation seams, invariants, proof matrix, and
non-goals, but cannot claim an implementation-ready or completed HCM-3.5 plan
until the named decision exists.

## Verified predecessor and repository truth

The selected predecessor is the completed HCM-3.4 source-pair handoff
`20260806T191500Z--HCM-3-4--orchestration--private-projection-source-pair-implementation-completed`.
It is valid v1.4 ledger evidence at the assigned base and was independently
validated with the complete handoff corpus. Its primary commit
`cc6d84926e6435c3960af86e5927a18991dd04fd` adds one exact private
`snapshot_current` / `snapshot_delta` source relation, five-family currentness,
typed refusal, provenance, redaction handling, and `authority_effect: none`.

The earlier HCM-3.4 primary `ca765cc7a45206392dc1997adaa924d4a8ecaeb3`
contains the private Snapshot Memory implementation:

- `crates/engine/src/snapshot_memory/policy.rs`, `record.rs`, `delta.rs`, and
  `redaction.rs` model capture, immutable snapshots, compatible deltas, drift,
  retention, and redaction;
- `crates/engine/src/projection.rs` consumes the private source pair; and
- `crates/engine/src/lib.rs` declares both `projection` and `snapshot_memory`
  with private `mod` visibility. No non-engine runtime consumer references
  `ContextMemorySnapshot`, `SnapshotDelta`, or the capture API.

That boundary is adequate predecessor evidence for private semantics only. It
does not select an exported type, service, serialization, SemVer posture, or
Flow-facing operation. The previous HCM-3.4 handoff's `snapshot_refs` are
honestly `not_available`; the current v1.4 handoff schema already has nullable
start/end/projection/delta reference fields but does not itself create a
snapshot producer.

## Frozen future seams and ownership

| Future seam | Current call path | Later owner and required boundary | Must remain true |
|---|---|---|---|
| Flow Resolution consumption | `handbook_flow::resolve` / `resolve_with_contract` accept public `ResolveRequest { budget_policy, packet_id }`, then `resolver::build_packet_result` emits rendered artifact sections. | `handbook-flow` consumes an exact engine-defined, current snapshot/delta grounding input through a reviewed engine-to-flow contract. It must not recreate Snapshot Memory, Projection, currentness, or redaction logic. | Resolution is six-dimensional, not a byte-budget alias; existing byte budgeting remains a resource constraint; stale, missing, incompatible, uncurrent, or insufficient sources refuse before a packet is ready. |
| Grounding Projection | HCM-3.4's private `projection::execute_projection` binds `snapshot_current` and `snapshot_delta`; Flow cannot name that private module. | `handbook-engine` remains the deterministic Projection owner. The compatibility decision must select the minimal exported typed operation/value boundary consumed by Flow. | Exact source/profile/vocabulary/definition/envelope fingerprints, complete disclosure/omission accounting, per-family currentness, redaction-before-read, and `authority_effect: none` survive the crossing. Comprehensive capture is never comprehensive disclosure. |
| Pipeline scoped inclusion | `pipeline_compile::compile_pipeline_stage*` obtains a raw `work_level`, passes it to `assemble_documents`, then `load_repo_relative_document` calls `filter_scoped_blocks`. | `handbook-pipeline` consumes an exact namespaced shared Resolution input from `handbook-engine`; it does not retain L0-L3 as semantic authority or add a competing resolver. | Unknown, stale, malformed, or indeterminate Resolution refuses; deny/least-disclosure behavior wins; legacy work-level behavior is preserved only through an explicit compatibility mapping approved with the new interface. |
| Parent handoff transition | v1.4 `snapshot_refs` already stores nullable prior/start/grounding/end/delta refs. | The orchestration handoff producer records exact refs from the selected capture/projection operation after source currentness is rechecked. | Handoffs cite observations; they never copy snapshots, make a snapshot canonical authority, or make a snapshot alone authorize dispatch. Missing/failed capture is explicit, not `captured`. |
| Local versus promotion gate | `GateResult` semantics specify separate local-closeout and parent-promotion eligibility; no `handbook-contracts` runtime exists. | HCM-3.5 may only consume an exact future gate policy/result boundary if its owner and compatibility posture are explicitly selected. | Local success never implies parent promotion; omitted/unobserved claims cannot pass; no Flow or pipeline default becomes a gate policy. |

## Authority boundary: public compatibility and gate-operation decision

The private HCM-3.4 types cannot be used from `handbook-flow` or
`handbook-pipeline`: Rust module privacy forbids access across crates. The
current Flow entry points and result structs are public and are called by the
CLI and compatibility compiler. Reusing them by changing their required input
would therefore select observable compatibility behavior. The existing crate
dependencies already point inward to `handbook-engine`; no new dependency is
needed, but the engine export and Flow/pipeline operation shape are a public
API decision.

The user or a separately authorized product decision must choose one bounded
posture before implementation planning can be completed:

1. Add a purpose-named, typed public engine snapshot-grounding operation and a
   new Flow Resolution-aware entry point, leaving existing resolver/compile
   operations unchanged.
2. Extend the existing public Flow and pipeline request/result operations with
   a versioned compatibility/migration contract.
3. Select another exact public owner/interface that preserves the acyclic
   `flow -> engine` and `pipeline -> engine` dependency direction.

The recommended posture is option 1: a narrow typed engine operation plus new
purpose-named Flow and pipeline operations. It isolates HCM-3.5 semantics,
does not silently change existing `ResolveRequest` or `PipelineCompileResult`
behavior, and leaves SDK/CLI/transport adoption for their selected later
slices. This is a recommendation, not an authorized decision.

The same decision must state whether HCM-3.5 may consume a concrete
`GateResult` operation now or must retain gate-policy integration as an
unimplemented typed boundary until HCM-5 supplies the runtime owner. The
frozen semantic distinction in `05` is not permission to invent a runtime
gate evaluator.

## Future fail-closed behavior and proof matrix

| Area | Positive proof | Required refusal / non-promotion proof |
|---|---|---|
| Engine-to-Flow grounding | One exact current snapshot plus compatible delta produces one fingerprinted Projection under an exact envelope; packet fields are included or typed-omitted once. | Private-module bypass, wrong profile/definition/envelope, missing/duplicate/substituted source, stale family/slot, incompatible/reversed delta, uncurrent capture, or unchecked delta family produces no ready packet. |
| Flow budget compatibility | Existing source/render byte budget remains independently observable after a permitted Resolution-aware projection. | Budget status never widens disclosure, substitutes for Resolution, turns an omission into inclusion, or declares readiness after a projection refusal. |
| Pipeline shared semantics | A named Resolution mapping deterministically includes only allowed scoped content while carrying exact provenance. | Unknown/malformed/stale mapping, raw L0-L3 fallback without approved mapping, nested/ambiguous scope syntax, or a broader envelope cannot silently include content. |
| Handoff capture | A parent records exact prior-end/start/grounding/end/delta refs after capture/currentness validation. | Absent/unstable/partial/incompatible capture, stale handoff, or snapshot-only evidence leaves capture status honest and blocks any promotion claim. |
| Gate distinction | A selected local result and parent-promotion result remain separately fingerprinted and reported. | Local closure, advisory score, omission, `not_observed`, stale policy, or unavailable gate runtime cannot become parent promotion. |
| Cross-crate compatibility | New selected APIs preserve existing resolver, compiler, and pipeline behavior under their old operations. | No engine-to-flow cycle, public untyped `Value` bridge, duplicate snapshot model, hidden snapshot payload, SDK/CLI/transport adoption, or Phase-3 exit claim. |

## Explicit non-goals

- No Rust, tests, fixtures, schemas, dependency/Cargo changes, runtime
  configuration, migration, public API selection, SDK, CLI, transport, or
  consumer adoption.
- No changes to HCM-3.4's immutable handoffs, source-pair proof, private
  engine semantics, HCM-3.6, HCM-4+, HCM-5, or Phase-3 exit claims.
- No snapshot capture from this planning task, no fabricated snapshot ref, and
  no product behavior test result.
- No protected-checkout mutation, remote query/fetch/push/publication, merge,
  rebase, reset, clean, force update, or pull request.

## Planning exit and stop condition

This packet may close only if the user supplies the public compatibility and
gate-operation decision above, after which a reviewed selector can freeze the
exact implementation API/seams. Until then, the correct result is a reviewed
v1.4 `authority_boundary` handoff with status `escalation_required`; it is not
a completed HCM-3.5 plan and does not consume authority for implementation.
