# HCM-3.5-P0 implementation-admission and typed-boundary contract freeze

**Status:** documentation subject pending one fresh complete-subject discovery
review. This selector is bound to increment task
`019fda6a-f05f-7a23-ad68-89b30f372578`, host `local`, meta workflow
`handbook-hcm-3-5-implementation-20260807`, and dispatch nonce
`d9432121a0209ee059502e74ed1480cb5ccd7c74d5c639a070d98b82b4d2571c`.

## Selected outcome and authority

| Field | Value |
|---|---|
| Integrated outcome | `hcm-3-5-implementation-admission-and-contract-freeze` |
| Packet | `implementation-admission-and-contract-freeze` |
| Parent orchestration | `handbook-hcm-3-5-implementation-20260807` |
| Base commit/tree | `0aa11d2851caee63f373c19c394925ec70473a8a` / `ebbddc19ad869f108e021ac31674fd5b5efde4ba` |
| Required ancestor | `1a10bfd59e8433fa344dd2baa4880b4b87c7f8f6` |
| Local integration ref | `refs/heads/orchestration/handbook-hcm-3-5-planning-20260806` |
| Publication | local-only expected-old compare-and-swap; no remote operation |

The human-approved planning tip is a transparent local planning landing after
a known validator-semantics contradiction. It is the exact base for this
Packet 0, not a normal previous v1.4 handoff or ledger closeout. This selector
does not resume, amend, supersede, or normalize that historical exception.

## Scope and selected documentation paths

The reviewed primary documentation subject is limited to the HCM-3.5 Packet 0
contract in `00-README.md` through `06-proof-and-regression-ledger.md`,
`slices/HCM-3.5/SPEC.md`, `slices/HCM-3.5/tasks/plan.md`,
`slices/HCM-3.5/tasks/todo.md`, this decision, its preflight proof, and one
new v1.4 review dispatch. Mechanical closeout may add exactly one parent-owned
v1.4 handoff and the rebuilt ledger entry. No historical dispatch, handoff,
ledger row, HCM-3.4 record, selector, or authority artifact is editable.

The purpose is to admit a later engine-only code packet without reopening the
owner architecture. The operation and values selected for that packet are:

```text
handbook_engine::grounding::ground_resolution(
    repo_root: &std::path::Path,
    request: GroundingRequest,
) -> Result<GroundingOutcome, GroundingOperationError>
```

`GroundingRequest` has one exact snapshot ref, one exact compatible delta ref,
the exact definition/disclosure closure, and `ContextResolutionEnvelope`.
`GroundingOutcome::Grounded` carries a private-field `GroundedResolution` with
only engine-created `FlowPacketGrounding`, `SharedResolutionInclusion`,
`DeltaSignalSummary`, typed provenance/omissions, and `GroundingEvidence`.
`GroundingOutcome::Refused` is the expected semantic failure path; the outer
error is only for operation integrity. Snapshot/delta bodies, raw `/signals`,
raw changes, generic JSON, caller filters, and caller currentness tuples are
not inputs or outputs.

The exact request/reference/extraction protocol is part of this freeze.
`GroundingSnapshotRef`, `GroundingDeltaRef`, `GroundingDefinitionRef`, and
`GroundingDisclosureRef` are opaque public value types with private fields and
only `parse_exact(&str) -> Result<Self, GroundingReferenceError>` factories.
`GroundingRequest::new(snapshot_ref, delta_ref, definition_ref,
disclosure_ref, envelope) -> Self` is the sole public request factory. A
private `GroundingSourceResolver` uses `repo_root` and those refs to load and
verify the exact sources; it has no public operation or type. Semantic source
or binding failure becomes `GroundingOutcome::Refused`; only non-semantic
operation-integrity failure becomes `GroundingOperationError`.

`GroundedResolution` exposes only named read-only Flow-packet, shared-
inclusion, summary, provenance, omission, and evidence accessors. A refusal
exposes only kind, omissions, and provenance; `DeltaSignalSummary` exposes
only bounded entries, omission accounting, and provenance; each summary entry
exposes only its already-admitted redacted fields. `GroundingEvidence` exposes
only `local_closeout` and `parent_promotion` as
`EvidenceAvailability::{Unavailable, False}`. The two consumer-view types
remain opaque named inputs to their later Flow/pipeline operations.

## Owner, cutover, and refusal rules

`handbook-engine` alone validates HCM-3.4 source-pair identity, all five
currentness families, exact envelope/definition bindings, redaction before
read, Projection accounting, and the bounded delta summary. It may use
private `snapshot_memory` and `projection` helpers through narrow `pub(crate)`
adapters, but those modules remain private and are not re-exported.

`handbook-flow` later consumes only `FlowPacketGrounding` in a separate
purpose-named operation. `handbook-pipeline` later consumes only
`SharedResolutionInclusion` in a separate purpose-named inclusion operation.
The direction is Flow -> engine and pipeline -> engine; engine never imports
either consumer, `handbook-sdk`, CLI, or Substrate. The retained
`resolve`/`resolve_with_contract` and `compile_pipeline_stage*` operations are
unchanged regression baselines. A typed-path refusal is terminal for that
typed path: no raw-level, legacy resolver/compiler, raw `work_level`, or raw
delta fallback is allowed.

`DeltaSignalSummary` replaces only the consumer-facing meaning of
`reveal_delta_signals`; it does not alter the relation-only HCM-3.4 delta.
The definition fixes its ordered maximum cardinality, eligibility, overflow,
source/provenance, and complete included/omitted/refused partition. Its items
are limited to allowed identity/classification, exact rule identity, bounded
affected refs/fingerprints, and allowed durable evidence/justification refs.

`GroundingEvidence` contains distinct local-closeout and parent-promotion
dimensions. Neither is a gate result, score, or policy evaluation. Both are
false or unavailable for missing, omitted, redacted, stale, malformed, or
indeterminate evidence, and only a future `handbook-contracts` owner may
evaluate their exact policies.

## Packet 1 admission ceiling and revalidation

P1 may modify only `crates/engine/src/lib.rs`, new
`crates/engine/src/grounding.rs`, narrow `pub(crate)` seams in
`projection.rs` and `snapshot_memory/{mod.rs,record.rs,delta.rs}`, and one new
`crates/engine/tests/hcm_3_5_grounding.rs`, plus its own selector/proof/
dispatch/closeout documentation. It may not change any Flow, pipeline, CLI,
compiler, SDK, Substrate, Cargo, dependency, schema/configuration, handoff
schema, gate-runtime, publication, or remote surface.

Before P1 edits, a fresh selector must revalidate:

1. this exact Packet 0 contract and the assigned base/tree/ref/ancestor;
2. the private HCM-3.4 source-pair identifiers and source semantics;
3. `ContextResolutionEnvelope`, retained resolver/compiler signatures, all
   Flow/pipeline/CLI/compiler callers, and nullable handoff `snapshot_refs`;
4. the lack of an SDK crate, Substrate checkout, gate runtime, or raw
   `/signals` consumer; and
5. an upstream impact result for every existing code symbol. HIGH or CRITICAL
   impact must be reported before editing; unavailable GitNexus is unavailable,
   never GREEN.

## Review, stop, and non-goals

One fresh independent complete-subject discovery review is required. A valid
P1/P2 requires one consolidated remediation and a different-fresh,
delta-focused closure review. At most two immediately causal supplemental
cycles are available; no review follows CLEAN. P3/P4 disposition follows
`09-review-finding-inventory.md` and does not expand the subject.

No Rust/code/runtime behavior, test/fixture, public API implementation,
schema, configuration, Cargo/dependency, package, registry, SDK, CLI,
Substrate, consumer implementation, gate runtime, remote action, protected
checkout mutation, HCM-3.6/HCM-4+/HCM-5 work, merge, rebase, reset, clean, or
force-push is authorized. Stop if the selected type boundary would require any
such expansion or a product-facing decision not frozen here.
