# HCM-3.5 Resolution-aware snapshot, packet, and pipeline adoption

**Status:** CLEAN-admitted, documentation-only implementation plan. This
packet freezes future ownership, cutover, proof, and refusal decisions; it
does not authorize Rust, runtime behavior, public APIs, schemas,
configuration, dependencies, package publication, SDK/CLI/Substrate adoption,
remote work, or a gate runtime.

## Objective and completed dependency evidence

HCM-3.5 plans the first future consumer path for the completed private HCM-3.4
Snapshot Memory boundary. It turns an exact current snapshot plus compatible
delta into a bounded Resolution Projection for Flow, namespaced shared
inclusion for pipeline, descriptive session transition references, and typed
non-promoting evidence. It does not turn any observation, packet, pipeline
result, or local completion into parent promotion.

The only Snapshot Memory dependency evidence is immutable and read-only:

- `ca765cc7a45206392dc1997adaa924d4a8ecaeb3` supplies the private
  Snapshot Memory model and deterministic delta behavior; and
- `cc6d84926e6435c3960af86e5927a18991dd04fd` supplies the private generic
  Projection source-pair proof.

At the planning base, `crates/engine/src/lib.rs` retains private
`snapshot_memory` and `projection` modules; `SnapshotDelta` is private to the
engine. No Flow, pipeline, handoff, gate, SDK, CLI, transport, or Substrate
consumer can use those private types. That is dependency evidence, not a
compatibility constraint or a reason to expose the existing private model.

## Packet 0 — implementation admission and typed-boundary contract freeze

This Packet 0 record is the only admission for the first engine-grounding code
packet. It freezes an intended Rust-library boundary; it does not add that
boundary to `crates/engine/src/lib.rs`, select a serialized DTO/schema, or
authorize any source edit.

### Exact future operation and visibility posture

Packet 1 may add `pub mod grounding;` to `handbook-engine` and create
`crates/engine/src/grounding.rs`. The cross-crate surface is limited to the
following purpose-named contract:

```text
handbook_engine::grounding::ground_resolution(
    repo_root: &std::path::Path,
    request: GroundingRequest,
) -> Result<GroundingOutcome, GroundingOperationError>
```

`GroundingRequest`, `GroundingOutcome`, `GroundedResolution`,
`GroundingRefusal`, `GroundingRefusalKind`, `GroundingOmission`,
`GroundingEvidence`, `EvidenceAvailability`, `DeltaSignalSummary`,
`DeltaSignalSummaryEntry`, `GroundingProvenance`, `FlowPacketGrounding`,
`SharedResolutionInclusion`, `GroundingSnapshotRef`, `GroundingDeltaRef`,
`GroundingDefinitionRef`, `GroundingDisclosureRef`, `GroundingReferenceError`,
and `GroundingOperationError` are public types in that module. Their fields
are private; construction and source loading stay engine-owned. In particular:

- `GroundingRequest` binds one `GroundingSnapshotRef`, one compatible
  `GroundingDeltaRef`, exact Projection/summary/disclosure definition pairs,
  and the public `ContextResolutionEnvelope`. It admits neither source bytes,
  `serde_json::Value`, an unbounded selector, a raw signal, nor a caller-made
  currentness or disclosure filter.
- `GroundingOutcome::Grounded(GroundedResolution)` is the only successful
  semantic result. `GroundedResolution` exposes narrow engine values for the
  Flow packet and pipeline inclusion paths, plus typed provenance, omissions,
  bounded summary, and non-promoting evidence. It does not expose an HCM-3.4
  snapshot, a `SnapshotDelta`, the raw `/signals` collection, raw changes, or
  Projection internals.
- `GroundingOutcome::Refused(GroundingRefusal)` represents every expected
  fail-closed condition: missing/duplicate/substituted pair, incompatible or
  reversed delta, stale binding/currentness family or slot, insufficient
  Resolution, required redaction, unsupported definition, unaccounted signal,
  or overflow that the exact definition requires to refuse. The outer
  `GroundingOperationError` is reserved for a non-semantic operation-integrity
  failure; it carries no source payload and is never converted into a grounded
  value.
- `snapshot_memory` and `projection` remain private modules. Packet 1 may add
  only `pub(crate)` adapters needed by `grounding`; it must not re-export their
  existing record, delta, request, result, or evaluation types. No top-level
  `pub use` or generic dispatcher is selected.

The public construction and extraction posture is also fixed, so the later
engine integration test needs no private HCM-3.4 value or raw input. Each
`Grounding*Ref::parse_exact(&str) -> Result<Self, GroundingReferenceError>`
accepts only an opaque canonical identity token; it never reads source data or
accepts a path, bytes, JSON, signal, filter, currentness tuple, or payload.
`GroundingRequest::new(snapshot_ref, delta_ref, definition_ref,
disclosure_ref, envelope) -> Self` is the only request factory. The engine's
private `GroundingSourceResolver` takes `repo_root` plus those validated refs,
loads the exact current/delta and definitions, and maps identity/binding/
semantic failures to `GroundingOutcome::Refused`; it is neither public nor a
second operation. `GroundingOperationError` is limited to operation-integrity
failure after a typed request has been made and has no source payload.

`GroundedResolution` has only the named read-only accessors
`flow_packet_grounding`, `shared_resolution_inclusion`, `delta_signal_summary`,
`provenance`, `omissions`, and `evidence`. `GroundingRefusal` has only `kind`,
`omissions`, and `provenance`. `DeltaSignalSummary` exposes bounded entries,
omission accounting, and provenance through named accessors; each
`DeltaSignalSummaryEntry` exposes only the previously allowed identity,
classification, rule, bounded affected-reference/fingerprint, and durable
evidence/justification values. `GroundingEvidence` exposes only
`local_closeout` and `parent_promotion` as
`EvidenceAvailability::{Unavailable, False}`. The consumer-view values are
opaque inputs to later purpose-named Flow/pipeline operations, not maps,
generic serializers, or raw-source extractors.

The dependency direction is fixed:

```text
handbook-flow     -> handbook-engine::grounding
handbook-pipeline -> handbook-engine::grounding
handbook-engine   -/-> handbook-flow, handbook-pipeline, handbook-sdk, CLI, or Substrate
```

`handbook-flow` later consumes `FlowPacketGrounding` through a new
purpose-named packet operation and returns its own ready/refused packet type.
`handbook-pipeline` later consumes only `SharedResolutionInclusion` through a
new purpose-named inclusion operation. Neither call is an overload, option,
or mandatory field on `resolve`, `resolve_with_contract`,
`compile_pipeline_stage*`, `PipelineCompileResult`, or `ResolverResult`.

### Bounded delta summary and evidence algebra

`DeltaSignalSummary` is the engine-produced reconciliation of
`reveal_delta_signals`. Its exact definition fixes maximum cardinality,
eligible kinds, stable order, overflow action, permitted source metadata, and
the included/omitted partition. Each source signal is accounted for once as
included, redacted, out-of-Resolution, unsupported, overflowed, or refused;
the result preserves exact source-delta/envelope/definition/disclosure
provenance. Included entries contain only signal ID/kind, exact rule pair,
bounded affected-reference or fingerprint data, and allowed evidence or
justification references. They never carry raw changes, snapshots, concealed
signals, unrestricted prose, or generic JSON.

`GroundingEvidence` has distinct `local_closeout` and `parent_promotion`
dimensions. Both are `unavailable` or `false` when any input is missing,
omitted, redacted, stale, malformed, or indeterminate. It has no gate policy,
score, `GateResult`, or promote operation. Only the later
`handbook-contracts` gate owner can evaluate those dimensions.

### Greenfield and Packet 1 ceiling

No typed-path refusal may fall back to `handbook_flow::resolve`,
`resolve_with_contract`, `compile_pipeline_stage*`, or raw L0-L3 filtering.
Those retained paths and their CLI/compiler callers are regression baselines
only. Flow/pipeline may surface an engine refusal as their own typed refusal,
but they may not re-read snapshot/delta data, synthesize a summary, or select
a legacy/raw path.

Packet 1 is limited to this initial source/test/proof ceiling:

| Category | Exact ceiling |
|---|---|
| Engine implementation | `crates/engine/src/lib.rs` (`pub mod grounding` only), new `crates/engine/src/grounding.rs`, narrow `pub(crate)` adapters in `crates/engine/src/projection.rs` and `crates/engine/src/snapshot_memory/{mod.rs,record.rs,delta.rs}` only |
| Public symbols | `ground_resolution`; `GroundingRequest::new`; `GroundingSnapshotRef::parse_exact`, `GroundingDeltaRef::parse_exact`, `GroundingDefinitionRef::parse_exact`, and `GroundingDisclosureRef::parse_exact`; named `GroundedResolution`, `GroundingRefusal`, `DeltaSignalSummary`, and `GroundingEvidence` read-only accessors only; `GroundingRequest`, `GroundingOutcome`, `GroundedResolution`, `GroundingRefusal`, `GroundingRefusalKind`, `GroundingOmission`, `GroundingEvidence`, `EvidenceAvailability`, `DeltaSignalSummary`, `DeltaSignalSummaryEntry`, `GroundingProvenance`, `FlowPacketGrounding`, `SharedResolutionInclusion`, `GroundingSnapshotRef`, `GroundingDeltaRef`, `GroundingDefinitionRef`, `GroundingDisclosureRef`, `GroundingReferenceError`, and `GroundingOperationError` only |
| Tests and fixtures | New `crates/engine/tests/hcm_3_5_grounding.rs`; reuse the HCM-3.4 fixed source-pair fixture without changing it unless a fresh selector grants a separately reviewed fixture delta |
| Documentation/proof | One P1 selector, exact source/impact record, proof matrix, immutable dispatches, parent handoff, and rebuilt ledger only |
| Explicitly excluded | Flow/pipeline/CLI/compiler/SDK/Substrate code, Cargo/dependency/version/schema/config changes, handoff schema changes, gate runtime, public JSON/Serde transport, package publication, and remote work |

Packet 1 admission must revalidate the exact base/tree/authority and all five
currentness families; the HCM-3.4 private modules and source-pair semantics;
the public `ContextResolutionEnvelope` seam; every retained Flow/pipeline and
CLI/compiler caller; Handoff `snapshot_refs` nullability; the absence of an
SDK crate, Substrate checkout, gate runtime, and raw `/signals` consumer; and
fresh upstream impact for every existing symbol it would edit. A HIGH or
CRITICAL impact result stops for explicit review before any edit.

## Frozen future owner boundary

| Future owner | Owns | May consume | Must not own or do |
|---|---|---|---|
| `handbook-engine` | Exact snapshot/delta binding and compatibility, family currentness, redaction-before-read, deterministic bounded Resolution Projection, typed omission/provenance, and a bounded/redacted delta-signal summary. | HCM-3.4 private snapshot/delta semantics and the HCM-3.2 Resolution kernel. | It does not promote evidence, select Substrate orchestration, or expose raw snapshot/delta payloads by default. |
| `handbook-flow` | A purpose-named future packet path that consumes one exact engine grounding result and reports ready/refused/omitted packet state. | The typed engine result only. | It does not recreate snapshots/deltas/currentness/redaction, read relation-only delta signals, or make gate/promotion decisions. |
| `handbook-pipeline` | A purpose-named future namespaced shared-inclusion path, compilation/capture/handoff sequencing, and provenance forwarding. | The exact shared engine Resolution inclusion input and selected Flow/engine evidence refs where needed. | It does not retain raw `work_level` or L0–L3 as semantic authority, create a second resolver, or consume full snapshot/delta data. |
| Future gate owner (`handbook-contracts`) | Exact local-closeout and parent-promotion policy evaluation. | Typed evidence refs/fingerprints and omissions from the adoption path. | It remains the only promotion decision owner; HCM-3.5 does not create this runtime. |
| `handbook-sdk` and standalone Handbook CLI | Future ordinary-consumer composition and transport adaptation over the chosen owner operations. | Selected SDK/library capabilities and typed results. | They do not absorb semantic ownership or become a fallback engine/Flow/pipeline path. |
| Substrate | Future orchestration/product wording and consumption or wrapping of the published SDK/library. | Exact published crates.io capability boundary. | It does not become a semantic owner or use a sibling/path/patch substitute as permanent adoption. |

The required dependency direction is `handbook-flow -> handbook-engine` and
`handbook-pipeline -> handbook-engine`; neither direction reverses, and neither
consumer reaches into an engine private module. `handbook-sdk` composes owner
operations; transports, including the standalone Handbook CLI and Substrate,
depend outward on that composition.

## Greenfield cutover and current call-path seams

This is a greenfield semantic cutover. The future adoption paths are
purpose-named and typed; they do not alter the request/result contracts or
observable behavior of the current resolver/compiler paths. Existing callers
are regression evidence only.

| Current read-only seam | Future cutover decision | Regression boundary |
|---|---|---|
| `handbook_flow::resolve` / `resolve_with_contract` -> `resolver::build_packet_result` | Add a separate Flow grounding path that accepts only the selected typed engine grounding result. | Existing resolver inputs/results remain unchanged and do not acquire a required snapshot or Resolution field. |
| `compile_pipeline_stage*` -> `assemble_documents` -> `load_repo_relative_document` -> `filter_scoped_blocks` | Add a separate pipeline inclusion path driven by an exact namespaced shared engine Resolution input. | Existing `work_level` behavior remains replayable while the new path is proved; it is never a fallback for a refusal on the new path. |
| v1.4 parent handoff `snapshot_refs` | Populate descriptive prior-end/start/grounding/end/delta references only after the selected capture/currentness checks pass. | Nullable/missing references remain honest; no record copies snapshot contents or becomes snapshot authority. |
| Current CLI/compiler/SDK/Substrate seams | Future standalone CLI calls SDK composition; Substrate consumes/wraps exact crates.io SDK/library versions. | Tier 2 exact binary/JSON remains an isolated transitional bridge, not API proof or permanent fallback. |

The only permitted cutover bridge is a definition-pinned, one-way mapping from
legacy scoped metadata to the namespaced shared Resolution inclusion input. It
must carry exact source/envelope/provenance identity and has no default. An
unknown, stale, malformed, ambiguous, nested, or broader-than-requested
mapping refuses. Raw `work_level` is never the final inclusion decision after
the new path is selected.

## Engine grounding and bounded delta-signal decision

The HCM-3.4 `snapshot_delta` remains a relation-only, complete immutable
observation. Its `/signals` collection remains owned by the delta and is not a
Flow/pipeline input. The engine produces the only consumer-facing delta view:
a bounded/redacted delta-signal summary derived from the exact selected delta,
the exact Resolution envelope, and the exact disclosure/redaction policy.

The summary is definition-bounded: the future definition fixes its ordered
maximum cardinality, eligibility/order rule, overflow behavior, permitted
signal metadata, and all source/provenance fields. It includes only allowed
signal identity/classification, rule identity, bounded affected-reference or
fingerprint data, and allowed evidence/justification refs. It excludes raw
changes, snapshot payloads, concealed signals, and unrestricted prose. It is
not a second delta, mutable cache, caller filter, or generic JSON bridge.

`reveal_delta_signals` is reconciled by revealing that summary only. The
future result must account exactly once for each eligible included signal and
every omitted, redacted, unsupported, stale, excess, or Resolution-insufficient
signal. If an unfiltered signal would rely on an unchecked currentness family,
the engine refuses or records the typed omission required by the selected
definition; Flow and pipeline cannot compensate by reading the raw delta.

Every future grounding Projection is bounded by its exact Resolution envelope,
definition, source pair, currentness closure, disclosure policy, and output
budget. Byte budget remains a subordinate resource report: it cannot widen an
envelope, turn an omission into inclusion, replace currentness, or mark a
refusal packet ready.

## Session-transition and non-promoting evidence boundary

A future parent transition follows this fixed reference sequence:

```text
prior end snapshot ref
  -> session start snapshot ref
  -> compatible delta ref
  -> engine grounding Projection ref
  -> Flow/pipeline work
  -> session end snapshot ref
  -> final compatible delta ref
  -> parent handoff references
```

All refs are exact ref/fingerprint pairs. Capture/currentness/compatibility
must be rechecked before each use. Missing, partial, unstable, stale,
redacted-only, incompatible, or otherwise refused capture is recorded with
its truthful status and blocks any claim that a grounding projection or
promotion decision exists. The handoff remains the normative transition
record; snapshot and delta records remain descriptive observations.

Flow and pipeline may emit a typed evidence boundary containing exact evidence
refs/fingerprints, refusal/omission status, and separately named local versus
parent dimensions. It is deliberately non-promoting: no local success,
advisory score, packet-ready result, pipeline completion, or handoff presence
implies parent promotion. Both dimensions default false or unavailable when
evidence is missing, omitted, redacted, stale, malformed, or indeterminate.
Only the later selected gate owner can evaluate the exact policies and produce
a parent-promotion result.

## Future implementation packets and proof matrix

| Packet order | Future owner/call path | Acceptance boundary | Required proof/refusal wall |
|---|---|---|---|
| A. Engine grounding | selected engine boundary -> private current snapshot + compatible relation-only delta -> bounded Projection and summary | Exact source/envelope/currentness/redaction/provenance closure; summary reconciles with `reveal_delta_signals`. | Positive bounded projection; missing/duplicate/substituted source; stale family/slot; incompatible/reversed delta; redaction-before-read; hidden raw signal/change; overflow; insufficient Resolution; no result on refusal. |
| B. Flow packet adoption | future Flow path -> engine grounding -> packet-ready/refused result | Flow forwards typed provenance/omissions and does not recreate engine semantics. | Packet include/omit/refuse matrix; byte-pressure non-widening; legacy resolver replay; no raw delta or private module access. |
| C. Pipeline inclusion | future pipeline path -> exact namespaced shared Resolution input -> scoped inclusion -> capture/handoff evidence | Inclusion is deterministic and provenance-bearing; legacy mapping, if present, is one-way and temporary. | Same-input replay; allowed/narrow/deny cases; unknown/stale/malformed/ambiguous/overbroad mapping refusal; raw-level no-fallback; existing scoped-path regression. |
| D. Transition refs | prior end -> start -> delta -> grounding -> end -> final delta -> parent handoff | Only exact validated descriptive refs are recorded. | Valid sequence plus absent/partial/unstable/stale/incompatible/reversed/redacted failure cases; no copied snapshot; no promotion claim. |
| E. Evidence/gate separation | Flow/pipeline evidence -> future gate owner | Local and parent dimensions are distinct and non-promoting. | Omitted/redacted/stale/not-observed/indeterminate/default-false matrix; prove local completion cannot promote a parent. |
| F. External composition | SDK -> standalone CLI; published SDK/library -> Substrate wrapper/consumer | CLI is a thin adapter; Tier 2 is replaceable; permanent Substrate path is registry-only. | Existing `PG-SDK-01`, `PG-JSON-01`, `PG-SUB-CLI-01`, `PG-PUBLISH-01`, and `PG-SUB-RUST-01` walls separately pass. |

Each future code packet needs its own implementation selector, fresh impact
analysis before every symbol edit, and independent review. GitNexus MCP/CLI
is unavailable in this checkout, so required impact/change-detection evidence
must be recorded as unavailable, never GREEN, until that capability exists.

## Invariants and fail-closed rules

1. Private HCM-3.4 semantics remain one engine owner; no consumer duplicates
   Snapshot Memory, delta, currentness, or redaction logic.
2. Exact ref/fingerprint provenance, all five currentness families, typed
   omission accounting, redaction-before-read, and `authority_effect: none`
   survive every permitted crossing.
3. Resolution is six-dimensional and non-widening. A byte budget, raw work
   level, handoff, or local status cannot substitute for it.
4. Full delta signals never route to Flow or pipeline. The bounded summary is
   engine-derived, envelope-bound, redacted, and complete in its own accounting.
5. Handoffs cite observations but do not make them canonical truth, gate
   truth, dispatch authority, or a parent-promotion decision.
6. Missing, stale, incompatible, malformed, duplicate, uncurrent,
   insufficient, redacted-only, unsupported, or indeterminate evidence fails
   closed with a typed refusal, omission, unavailable state, or non-promotion.
7. Tier 2 binary/JSON is isolated and replaceable. Its presence neither proves
   crates.io publication nor permits permanent Substrate fallback.

## Explicit non-goals

- No Rust, test, fixture, runtime, public API, schema, configuration, Cargo,
  dependency, SDK, CLI, Substrate, registry, publication, remote, or consumer
  change.
- No export or use of existing HCM-3.4 private types, no full
  `snapshot_delta` signal routing, and no duplicate snapshot/delta model.
- No modification to HCM-3.4 evidence, historical selectors/preflight,
  handoff records/ledger, or dispatches; Packet 0 may create only its new
  review dispatch and mechanical local-closeout handoff/ledger records. No
  HCM-3.6, HCM-4+, HCM-5, or Phase-3 exit is selected.
- No gate runtime, parent promotion decision, product test result, fabricated
  snapshot reference, or adoption/completion claim.
