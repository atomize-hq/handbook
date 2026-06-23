# Plan: Handbook Published-Import Decoupling — Set 2: Minimal Public Capability Boundary for `handbook-pipeline`

Spec reference: [handbook-published-import-decoupling-set-2-minimal-public-capability-boundary-spec.md](./handbook-published-import-decoupling-set-2-minimal-public-capability-boundary-spec.md)

## Overview

Set 2 is the implementation set that turns the Set 1 authority lock into a real public `handbook-pipeline` capability boundary.

The goal is not “make more things public.” The goal is narrower:

1. expose exactly the typed contract owners and contract-aware entrypoints the intended downstream consumer needs,
2. keep handbook-product defaults and internal helpers private unless they are strictly required, and
3. leave behind a release-candidate external consumer proof that exercises the retained capability families from outside the crate boundary.

This set is complete only when the landed API matches the retained/dropped matrix from the spec and the proof wall is honest.

## Current State (live repo truth)

- `PipelineDeclarativeRootsContract` exists but is still `pub(crate)` in `crates/pipeline/src/declarative_roots.rs`.
- `PipelineStorageLayoutContract` exists but is still `pub(crate)` in `crates/pipeline/src/layout.rs`.
- Declarative-root-aware variants such as `load_pipeline_catalog_with_roots(...)` and `load_pipeline_definition_with_roots(...)` exist only as `pub(crate)` helpers.
- `SupportedTargetRegistry::load(...)` and route-aware `load_pipeline_catalog(...)` are still public in live repo truth and still have direct in-repo callers across CLI, pipeline internals, and tests.
- Storage-layout-aware variants such as `load_route_state_with_storage_layout(...)`, `set_route_state_with_storage_layout(...)`, `preview_pipeline_capture_with_storage_layout(...)`, and `emit_pipeline_handoff_bundle_with_storage_layout(...)` exist only as `pub(crate)` helpers.
- Selector-based contract-aware variants do not yet exist for all retained declarative-root capabilities.
- Current published `handbook-pipeline = 0.1.1` still fails external imports of layout/declarative-root control seams.
- Live GitNexus impact analysis marks `load_pipeline_catalog_metadata`, `load_pipeline_definition`, and `load_selected_pipeline_definition` as CRITICAL upstream blast-radius seams, so declarative-root landing must prefer additive/public-sibling exposure before caller migration or privacy clamp.

## Intended Consumer Shape

The first-wave consumer is a Substrate-owned provider/context boundary that:

- constructs non-default pipeline contracts once,
- calls retained public `handbook-pipeline` contract-aware entrypoints only,
- receives typed semantics back,
- renders product/runtime wording in Substrate,
- and avoids depending on handbook repo-plumbing helpers.

This keeps the boundary broad enough to satisfy real downstream capability, but narrow enough to avoid accidental overexposure.

## Delivery Strategy

The work should proceed in seven sequential packets:

```text
Packet 2.1 (public contract owners)
  -> Packet 2.2a (retained declarative-root façade landing)
  -> Packet 2.2b (caller/test migration off dropped seams)
  -> Packet 2.2c (dropped-seam privacy clamp)
  -> Packet 2.3 (route-state storage-layout façade)
  -> Packet 2.4 (capture + handoff storage-layout façade)
  -> Packet 2.5 (release-candidate external proof + closeout)
```

These packets are intentionally sequential because later packets depend on API names and contract owners stabilized in earlier packets.

## Packet Plan

## Packet 2.1 — Public Contract Owners

### Goal

Promote the two reviewed contract owners to real public boundary types without promoting their private modules wholesale.

### Work

- make `PipelineDeclarativeRootsContract` public with validated constructor(s) and stable read accessors
- add explicit validation for declarative-root paths if current constructors are too permissive
- make `PipelineStorageLayoutContract` public with validated constructor(s) and stable read accessors
- keep `RuntimeStateLayoutContract`, `CaptureStorageLayoutContract`, `HandoffBundleLayoutContract`, and `RepoLayoutRoot` private
- update `crates/pipeline/src/lib.rs` only as needed to expose the reviewed façade, not raw private modules

### Verification checkpoint

Packet 2.1 is done only when:

- downstream can construct both contract owners through public APIs
- nested helper structs remain private
- no new public module-level exposure was introduced beyond the reviewed façade

## Packet 2.2a — Retained Declarative-Root Façade Landing

### Goal

Land the retained declarative-root-aware entrypoints as additive public façade seams without yet tightening dropped seam visibility.

### Work

- promote or add public contract-aware variants for:
  - `load_pipeline_catalog_metadata`
  - `load_pipeline_selection_metadata`
  - `load_pipeline_definition`
  - `load_selected_pipeline_definition`
- keep existing handbook-product default entrypoints behaviorally stable
- do **not** privatize `SupportedTargetRegistry::load` or route-aware `load_pipeline_catalog` in this packet
- write or update tests that prove custom declarative roots work through the retained public façade only

### Verification checkpoint

Packet 2.2a is done only when:

- custom roots can drive metadata browse, selector resolution, and definition load through retained public APIs only
- no test added in this packet depends on private module imports
- the retained/dropped matrix still matches the landed additive API

## Packet 2.2b — Caller/Test Migration Off Dropped Seams

### Goal

Remove in-repo dependence on dropped declarative-root seams before the privacy clamp.

### Work

- migrate in-repo callers off `SupportedTargetRegistry::load(...)`
- migrate in-repo callers off route-aware `load_pipeline_catalog(...)`
- update package-local proof and adjacent in-repo tests so the declarative-root family is exercised through retained/public APIs instead of the dropped seams
- keep handbook-product default entrypoints intact for handbook's own product behavior
- keep the packet focused on declarative-root-family migration only; do not widen into later packet capability

### Verification checkpoint

Packet 2.2b is done only when:

- known in-repo callers no longer require the dropped seams
- package-local proof stays public-API-only for the retained declarative-root family
- the migration does not widen into route-state, capture, handoff, or release-proof work

## Packet 2.2c — Dropped-Seam Privacy Clamp

### Goal

Make the dropped declarative-root seams private only after the migration wall is satisfied.

### Work

- make `SupportedTargetRegistry::load(...)` private/internal
- make route-aware `load_pipeline_catalog(...)` private/internal
- rerun the declarative-root-family proof wall to confirm no required caller still depends on dropped seams

### Verification checkpoint

Packet 2.2c is done only when:

- `SupportedTargetRegistry::load(...)` and route-aware `load_pipeline_catalog(...)` are no longer public
- known in-repo callers compile and test against the retained/public alternatives
- the retained/dropped matrix still matches the landed API

## Packet 2.3 — Route-State Storage-Layout Public Façade

### Goal

Expose the retained storage-layout-aware route-state and trusted-session seams.

### Work

- promote public storage-layout-aware variants for:
  - `load_route_state`
  - `set_route_state`
  - `load_trusted_pipeline_session`
  - `persist_route_basis`
- keep repo-layout plumbing private
- prove non-default state roots work without leaking handbook-product layout assumptions
- keep Packet 4.2 classification honest: this is a reusable pipeline boundary, not downstream Substrate adoption

### Verification checkpoint

Packet 2.3 is done only when:

- custom storage layout drives route-state read/write and trusted-session behavior through public APIs only
- route-state tests cover at least one non-default storage-root arrangement
- no new public helper types were added outside the Set 2 matrix

## Packet 2.4 — Capture + Handoff Storage-Layout Public Façade

### Goal

Expose the retained capture and handoff storage-layout seams without widening into convenience helpers that the first-wave consumer does not need.

### Work

- promote public storage-layout-aware variants for:
  - `preview_pipeline_capture`
  - `apply_pipeline_capture`
  - `emit_pipeline_handoff_bundle`
  - `validate_pipeline_handoff_bundle`
- keep `capture_pipeline_output_with_storage_layout` and `load_pipeline_capture_cache_entry_with_storage_layout` private unless a live proof reopens the matrix
- prove non-default capture and handoff roots work through the retained public façade
- keep handbook render helpers/public wording behavior out of the new boundary

### Verification checkpoint

Packet 2.4 is done only when:

- custom storage layout drives retained capture and handoff behaviors through public APIs only
- dropped convenience seams remain private
- no private layout module imports are required anywhere in the proof wall

## Packet 2.5 — Release-Candidate External Proof + Closeout

### Goal

Prove the retained Set 2 boundary from outside the crate source tree and close the set without overclaiming Set 3 work.

### Work

- add a packaged external consumer proof harness that:
  - uses a packaged or unpacked release-candidate artifact of `handbook-pipeline`
  - constructs non-default declarative-roots and storage-layout contracts
  - exercises every retained capability family through public APIs only
- rerun `cargo package -p handbook-pipeline --allow-dirty` and `cargo publish --dry-run -p handbook-pipeline`
- record closeout notes that explicitly preserve:
  - Set 3 still owns released-crate proof, downstream Substrate proof, and guard rails
  - Packet 4.2 remains only `engine + flow` proof
  - no downstream source-touching proof happened inside Set 2

### Verification checkpoint

Packet 2.5 is done only when:

- the release-candidate external consumer proof passes
- the retained/dropped matrix still matches the landed API
- Set 2 closeout notes do not overclaim Set 3 work

## Sequential vs Parallel Notes

- **Not parallel-safe by default:** Packets 2.1–2.5 share public API names, crate exports, and proof assumptions.
- Packet 2.2a, Packet 2.2b, and Packet 2.2c are explicitly **not** parallel-safe with each other:
  - 2.2a establishes the retained additive seams
  - 2.2b migrates callers/tests to those seams
  - 2.2c clamps visibility on the dropped seams
- The only work that may be parallelized later is low-risk proof-harness scripting after Packet 2.4 stabilizes the exact retained public names.
- Do not split public API naming and proof writing into parallel work before Packet 2.4 is review-clean.

## Risks And Mitigations

### Risk 1: accidental whole-module publication

- **Why it matters:** Making `declarative_roots` or `layout` public would satisfy capability quickly but violate the MAP intent.
- **Mitigation:** expose only the reviewed façade and contract owners; keep module-level paths private.

### Risk 2: convenience-surface creep

- **Why it matters:** one-shot helpers and raw helper structs can quietly become permanent public contract.
- **Mitigation:** enforce the retained/dropped matrix from the spec; every newly proposed public row must be justified before code progress counts.

### Risk 3: confusing packaged proof with final released proof

- **Why it matters:** Set 2 needs an external proof wall, but the overall workstream still needs Set 3 released-consumer and downstream proof.
- **Mitigation:** close Set 2 with a packaged release-candidate proof and keep released-crate plus downstream Substrate proof explicitly in Set 3.

### Risk 4: overreaching into downstream integration

- **Why it matters:** Set 2 is about the provider boundary in `system`, not downstream product-seam adoption.
- **Mitigation:** keep all downstream Substrate source-touching work out of Set 2 and preserve the dedicated worktree rule for Set 3.

### Risk 5: combining additive façade landing, migration, and privacy clamp into one diff

- **Why it matters:** the retained declarative-root loaders are high-blast-radius seams and the dropped seams still have live callers, so a one-packet landing obscures whether failures come from API introduction, caller migration, or visibility tightening.
- **Mitigation:** keep Packet 2.2 split into 2.2a additive landing, 2.2b migration, and 2.2c privacy clamp; require each sub-packet to verify independently before the next starts.

## Review Checklist

Before accepting Set 2 as complete, verify:

- the public boundary still answers the MAP capability check and boundary check
- retained seams provide the full required capability families
- Packet 2.2a, Packet 2.2b, and Packet 2.2c landed in order without bundling additive exposure, caller migration, and privacy clamp into one unverifiable change
- dropped seams stayed private and were not reintroduced under slightly different names
- the release-candidate external consumer uses public APIs only
- no closeout note implies downstream Substrate proof already happened

## Completion Standard

Set 2 should end with:

- one reviewed public declarative-roots contract owner,
- one reviewed public storage-layout contract owner,
- only the retained contract-aware façade entrypoints,
- a completed Packet 2.2a / 2.2b / 2.2c sequence for the declarative-root family,
- passing package-local tests for custom roots/layouts,
- one passing release-candidate external consumer proof,
- and closeout notes that hand off released-crate proof, downstream Substrate proof, and guard rails to Set 3.
