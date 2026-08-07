# HCM-3.5 P1 — Engine grounding and bounded delta summary selector

**Created:** 2026-08-07T14:15:00Z
**Phase / slice / packet:** HCM-3 / HCM-3.5 / P1
**Continuation parent:** `handbook-hcm-3-5-continuation-implementation-20260807`
**Bound task / host:** `019fdc90-717f-7470-aa93-946027954127` / `local`
**Dispatch nonce:** `e7242bf06d18a6327d4b223866f0bed5871dbcd1ff4f4be05e81ad4193d1c1f5`

## Authority and lineage

This is the first selector under the distinct continuation parent
`handbook-hcm-3-5-continuation-implementation-20260807`. It is not a same-parent
resumption or supersession of the immutable P0 parent
`handbook-hcm-3-5-implementation-20260807`.

The sole predecessor authority is the immutable P0 handoff:

- `20260807T124510Z--HCM-3-5--orchestration--implementation-admission-and-contract-freeze-completed`
- `docs/specs/handbook-contract-membrane/handoffs/records/20260807T124510Z--HCM-3-5--orchestration--implementation-admission-and-contract-freeze-completed.json`

Its documented historical validator exception remains visible evidence only; it
is neither rewritten nor treated as a successful same-parent continuation.

The admitted local baseline is commit
`7e0836a1c60f3992a80719b597f3fc18fabe807e`, tree
`ddeaf41a367d069c38b378398117673d5c8e8218`, with required ancestor
`1a10bfd59e8433fa344dd2baa4880b4b87c7f8f6`. The target ref is
`refs/heads/orchestration/handbook-hcm-3-5-planning-20260806`; no remote is
consulted or changed.

## Selected implementation boundary

P1 creates the engine-owned grounding operation and the only consumer-view
values needed by later, separately selected purpose-named Flow and pipeline
paths. The public surface is closed to:

- `ground_resolution`;
- `GroundingRequest::new`;
- `GroundingSnapshotRef::parse_exact`, `GroundingDeltaRef::parse_exact`,
  `GroundingDefinitionRef::parse_exact`, and
  `GroundingDisclosureRef::parse_exact`;
- the P0-listed public types and only the P0-listed named read-only accessors.

`ground_resolution` accepts a typed request, resolves the exact private
snapshot/delta source pair inside `handbook-engine`, and returns only
`GroundingOutcome::{Grounded, Refused}` or a payload-free
`GroundingOperationError`. A refusal never becomes a grounded result.

The engine remains the only owner of source-pair binding, all five currentness
families, redaction-before-read, Resolution filtering, summary construction,
provenance, and typed omissions. The definition-bounded delta-signal summary
has stable ordering and bounded cardinality, accounts for every eligible source
signal as included or typed omitted/refused, and never exposes raw changes,
snapshot payload, the raw signal collection, unrestricted prose, or a generic
JSON/Serde bridge. `GroundingEvidence` remains non-promoting with
`authority_effect: none`; it exposes only unavailable or false local-closeout
and parent-promotion availability.

`FlowPacketGrounding` and `SharedResolutionInclusion` are opaque consumer-view
values. P1 creates no Flow, pipeline, compiler, CLI, SDK, Substrate, or gate
runtime path and does not re-export private HCM-3.4 record, delta, request,
result, or evaluation types.

## Exact path ceiling

Only these source and test paths may change for the primary P1 implementation:

- `crates/engine/src/lib.rs` — the declaration `pub mod grounding;` only.
- `crates/engine/src/grounding.rs` — new module only.
- `crates/engine/src/projection.rs` — narrow `pub(crate)` grounding adapters
  only.
- `crates/engine/src/snapshot_memory/mod.rs` — narrow `pub(crate)` grounding
  adapters only.
- `crates/engine/src/snapshot_memory/record.rs` — narrow `pub(crate)`
  grounding adapters only.
- `crates/engine/src/snapshot_memory/delta.rs` — narrow `pub(crate)` grounding
  adapters only.
- `crates/engine/tests/hcm_3_5_grounding.rs` — new test only.

The P1 selector, source/impact record, proof matrix, immutable dispatches,
parent-owned handoff record, and rebuilt ledger are the only allowed
documentation/control artifacts. No Cargo, dependency, version, schema,
configuration, fixture, gate-runtime, public JSON/Serde, publication, remote,
or protected-checkout path is admitted.

## Mandatory pre-edit revalidation

Before changing each existing function, class, method, or existing module
surface, record an upstream GitNexus impact result and the direct callers,
affected processes, and risk. A HIGH or CRITICAL result pauses that edit for
explicit review. Revalidate:

1. the HCM-3.4 source-pair, relation-only delta, private visibility, and all
   five currentness families;
2. the public `ContextResolutionEnvelope` seam and its current consumers;
3. retained `resolve`, `resolve_with_contract`, `compile_pipeline_stage*`, and
   their CLI/compiler callers as unchanged regression baselines;
4. nullable `snapshot_refs`, the lack of an SDK crate, Substrate checkout and
   gate runtime, and the lack of Flow/pipeline raw-signal consumers; and
5. the exact source/test ceiling before every write.

If GitNexus is locally unavailable after the required local index attempt, the
record must say unavailable rather than GREEN; no code symbol is edited until a
replayable equivalent upstream blast-radius report is available for that exact
symbol.

## Proof matrix

The P1 wall must prove all of the following with the new engine integration
test and focused crate regression:

- exact current source pair yields a bounded grounded result with typed
  provenance and no promotion authority;
- duplicate, substituted, incompatible, reversed, stale, malformed, and
  missing source/ref inputs refuse without a grounded result;
- every currentness family and slot is enforced;
- required redaction happens before source reading, leaves a typed omission,
  and cannot leak an underlying signal/change;
- insufficient Resolution and summary overflow refuse or omit exactly as the
  fixed definition requires, while byte pressure never widens the envelope;
- the bounded summary is stably ordered, accountably redacted, and cannot
  reveal raw full-delta data; and
- no private HCM-3.4 type is publicly exported, no raw-signal consumer is
  added, and no Flow/pipeline/CLI/compiler surface changes.

## Review and closeout cadence

After a complete P1 subject and proof wall converge, a fresh read-only
discovery reviewer independently assesses the whole subject. All valid P1/P2
findings receive one consolidated parent remediation. A different fresh,
read-only closure reviewer then examines only that remediation and its direct
consequences. P1 is CLEAN only after that closure; no review follows CLEAN.

The implementer is never a reviewer. All P1 dispatches and records bind this
distinct continuation parent and the bound nonce. P2 may not begin until P1 is
CLEAN.

## Explicit non-goals and stop conditions

Stop and return control instead of widening if P1 needs any consumer adoption,
raw delta/signal access, new public serialization, a dependency/configuration
change, a fixture change, a gate/promotion decision, or a path outside this
ceiling. Do not start HCM-3.6, HCM-4+, HCM-5, SDK/CLI/Substrate, P5 gate
runtime, P6 publication/adoption, or any remote operation.
