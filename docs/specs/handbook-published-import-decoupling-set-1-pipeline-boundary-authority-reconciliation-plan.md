# Plan: Handbook Published-Import Decoupling — Set 1: Pipeline Boundary Authority Reconciliation

Spec reference: [handbook-published-import-decoupling-set-1-pipeline-boundary-authority-reconciliation-spec.md](./handbook-published-import-decoupling-set-1-pipeline-boundary-authority-reconciliation-spec.md)

## Overview

Set 1 is the first active set after the 2026-06-23 audit invalidated the stronger claim that the whole `engine + flow + pipeline` decoupling story is already honest.

This set is intentionally **docs-only**. Its job is to leave the repo with one active answer to three questions:

1. What is actually proven today?
2. Which prior claims are now superseded?
3. What exactly must Set 2 and Set 3 do, given the MAP requirement that Substrate must receive the full reusable handbook capability it needs with minimum unnecessary public surface?

This set should not widen a published crate boundary itself. Per `/Users/spensermcconnell/__Active_Code/system/docs/specs/MAP.md`, Set 1 is not deciding whether `handbook-pipeline` capability work is needed. That need is already locked. Set 1 is deciding how to route that need into the smallest reviewed public boundary and how that boundary must later be proven.

This set is one triplet with four sequential packets:

```text
Packet 1.1 (evidence matrix)
  -> Packet 1.2 (boundary-shape decision)
  -> Packet 1.3 (authority reconciliation)
  -> Packet 1.4 (final proof + Set 2/3 handoff)
```

No packet is parallel-safe by default because each later packet depends on the outputs from the earlier one.

## Current State (live repo truth)

- `handbook-engine` and `handbook-flow` are published at `0.1.1` and are externally consumable through a real typed seam.
- The dedicated Packet 4.2 Substrate worktree proves published consumption of `engine + flow` through a narrow production seam in `prompt_fulfillment.rs`.
- `crates/pipeline/src/lib.rs` still keeps `declarative_roots` and `layout` private.
- `PipelineDeclarativeRootsContract` and `PipelineStorageLayoutContract` remain non-public in live source and therefore remain unavailable through the current published crates.io boundary.
- Archived Set 1 / Set 3 parameterization docs claim a stronger pipeline import-facing story than the current published crate surface actually provides.
- Archived published-consumption docs intentionally froze the first-wave public `handbook-pipeline` boundary to a narrower subset that excludes those private seams.
- `/Users/spensermcconnell/__Active_Code/system/docs/specs/MAP.md` now locks the true objective: Substrate needs the full reusable `handbook-pipeline` capability set, but implementation must expose the minimum reviewed public surface needed to provide it.
- The `9b83` Substrate notes remain useful for long-term provider-boundary thinking, but they are non-canonical and should only be cited as stale design context.

## Components

### 1. Evidence reconciliation matrix

Build a side-by-side map of:

- current live code truth
- current crates.io behavior
- the root MAP objective/intent
- the root audit's claims
- the Packet 4.2 proof memo's claims
- the archived parameterization claims
- the archived published-boundary claims
- the stale `9b83` provider-boundary context

This component exists so Set 1 stops being a vague “something is inconsistent” feeling and becomes a precise authority conflict map.

### 2. Pipeline boundary-shape decision

Make the set-level decision explicit:

- Substrate requires the full reusable pipeline capability set.
- Set 2 must implement that capability through a reviewed published boundary.
- Set 1 must determine the narrowest stable public shape that can provide it.

The set must define the intended downstream shape precisely enough that Set 2 can be scoped without reopening the objective itself.

### 3. Active authority reconciliation

Create or update active docs under `/Users/spensermcconnell/__Active_Code/system/docs/specs/` so a fresh session no longer has to infer truth by comparing multiple contradictory archived docs.

This component should:

- preserve archive files as provenance
- avoid rewriting history inside ignored archival areas unless explicitly necessary
- clearly mark which claims are still valid, which are superseded, and why
- explicitly point back to `/Users/spensermcconnell/__Active_Code/system/docs/specs/MAP.md` as the root authority

### 4. Set 2 handoff contract

Once the boundary-shape decision is made, define the exact next implementation wall:

- required code surfaces
- required external-consumer proof inputs
- required downstream Substrate revalidation inputs
- required review questions from the MAP
- required guard rails to prevent “internally parameterized but externally private” regressions from being called complete again

### 5. Set 3 proof contract

Define the exact proof wall that must exist after implementation:

- external published-consumer proof
- real downstream Substrate proof
- anti-regression guard rails
- explicit checks that capability was exposed without overexposing handbook-only internals

## Packet Plan

## Packet 1.1 — Current-State Evidence Matrix

### Goal

Turn the current disagreement into a bounded matrix of exact claims versus live truth versus MAP intent.

### Work

- re-read `/Users/spensermcconnell/__Active_Code/system/docs/specs/MAP.md`
- re-read the root audit and Packet 4.2 proof memo
- inspect live crate-boundary code for `engine`, `flow`, and `pipeline`
- reproduce the positive `engine + flow` external-consumer proof
- reproduce the negative `pipeline` external-consumer proof
- record which archived docs make which stronger/weaker boundary claims

### Evidence baseline captured on 2026-06-23

| Area | Current truth | Packet 1.1 implication |
|---|---|---|
| MAP objective | Full reusable `handbook-pipeline` capability is required; minimum unnecessary public surface remains the intent. | Packet 1.1 must reject any “`engine + flow` is enough” reading. |
| Positive published seam | A scratch crate depending on crates.io `handbook-engine = "=0.1.1"` + `handbook-flow = "=0.1.1"` compiled successfully using `CanonicalLayoutContract` plus `resolve_with_contract(...)`. | Keep Packet 4.2 classified as honest narrow published-consumption proof. |
| Negative published seam | A scratch crate depending on crates.io `handbook-pipeline = "=0.1.1"` failed to import `handbook_pipeline::layout::PipelineStorageLayoutContract` with `error[E0603]: module 'layout' is private`. | Treat the pipeline seam as still private in current published truth. |
| Live pipeline source | `crates/pipeline/src/lib.rs` keeps `declarative_roots` and `layout` private; the corresponding contract types remain `pub(crate)`. | Supersede archive claims that describe those seams as already import-facing. |
| Archived published-boundary doc | The first-wave published boundary intentionally froze a narrower subset that excludes `declarative_roots` and `layout`. | Keep as provenance for why crates.io `0.1.1` is narrow, not as evidence that the MAP objective is complete. |

### Verification checkpoint

```bash
sed -n '1,120p' crates/pipeline/src/lib.rs
sed -n '1,220p' crates/pipeline/src/declarative_roots.rs
sed -n '1,260p' crates/pipeline/src/layout.rs
sed -n '452,520p' crates/flow/src/resolver.rs
cargo check --workspace
```

### Exit condition

The repo has an explicit “claim -> MAP alignment -> live truth -> status” matrix instead of only prose disagreement.

Packet 1.1 is complete only when the active Set 1 docs also clearly distinguish:

- the **proven** published `engine + flow` seam, and
- the **still-private** `handbook-pipeline` seam that Set 2 must expose minimally.

## Packet 1.2 — Boundary-Shape Decision And Set 2 Target

### Goal

Decide what Set 2 is actually supposed to build without reopening the MAP objective.

### Work

- treat full reusable `handbook-pipeline` capability for Substrate as a fixed requirement
- decide whether Set 2 should satisfy that requirement by direct module/type promotion or by a narrower public façade
- define what counts as successful future proof for that implementation shape
- explicitly carry forward the MAP principle: expose capabilities, not guts
- capture open questions that still require human review rather than burying them in implementation

### Decision captured on 2026-06-23

- **Requirement lock:** full reusable `handbook-pipeline` capability for Substrate remains mandatory active authority work; Packet 4.2 stays classified as published `engine + flow` proof only.
- **Chosen boundary shape:** Set 2 should expose a narrower public façade, not wholesale direct promotion of `declarative_roots` or `layout`.
- **Why:** the live private modules mix required downstream contracts with handbook-product defaults, nested helper structs, and repo-layout plumbing; a façade can expose capability without freezing those internals publicly.
- **Bounded Set 2 candidate proof surface:**
  - public declarative-roots contract surface, with candidate first-wave public proof bounded to `handbook_pipeline::pipeline::SupportedTargetRegistry::load`, `handbook_pipeline::pipeline::load_pipeline_catalog`, `handbook_pipeline::pipeline::load_pipeline_catalog_metadata`, `handbook_pipeline::pipeline::load_pipeline_selection_metadata`, `handbook_pipeline::pipeline::load_pipeline_definition`, and `handbook_pipeline::pipeline::load_selected_pipeline_definition` for declarative-root control plus stage-root-aware catalog/loading behavior
  - public storage-layout contract surface, with candidate first-wave public proof bounded to `handbook_pipeline::route_state::{load_route_state, set_route_state, load_trusted_pipeline_session, persist_route_basis}`, `handbook_pipeline::pipeline_capture::{preview_pipeline_capture, capture_pipeline_output, apply_pipeline_capture, load_pipeline_capture_cache_entry}`, and `handbook_pipeline::pipeline_handoff::{emit_pipeline_handoff_bundle, validate_pipeline_handoff_bundle}` for route-state, capture, and handoff storage-layout control
  - contract-aware entrypoints on that candidate existing public pipeline surface
  - only the typed results/errors that downstream consumers must actually handle
- **Keep private:** `RepoLayoutRoot`, nested storage-layout helper structs, handbook-product default helpers/constants unless later external-consumer proof names a concrete need, repo/file/path plumbing, and product-shell/CLI-only behavior.
- **Acceptance wall:** Set 2 is only honest when a published `handbook-pipeline` consumer can construct non-default contracts and then either justify each retained candidate `handbook_pipeline::pipeline::*`, `handbook_pipeline::route_state::*`, `handbook_pipeline::pipeline_capture::*`, and `handbook_pipeline::pipeline_handoff::*` path against a named MAP-required capability plus intended consumer shape, or shrink that candidate proof surface explicitly; Set 2 may not widen beyond that bounded candidate surface without reopening authority, and downstream Substrate adoption remains a later dedicated-worktree proof.

### Verification checkpoint

```bash
rg -n "frozen subset|declarative_roots|layout|public/import-facing|published" \
  docs/specs/archive/handbook-published-crates-and-substrate-consumption-* \
  docs/specs/archive/handbook-substrate-import-adoption-plan.md \
  docs/specs/archive/handbook-substrate-parameterization-set-1-pipeline-import-layout-* \
  docs/specs/archive/handbook-substrate-parameterization-set-3-import-surface-honesty-cleanup-*
```

### Exit condition

Set 2 has a stable problem statement and acceptance wall that do not rely on contradictory archive-era assumptions and do not reopen the MAP objective.

## Packet 1.3 — Active Authority Reconciliation

### Goal

Replace the current implicit contradiction with explicit active authority.

### Work

- update the active Set 1 triplet with the settled decision
- ensure the active Set 1 triplet quotes or references the MAP exact objective and exact intent explicitly
- add any necessary supersession wording to the root audit or new active completion notes
- cite the stale `9b83` Substrate notes only as non-canonical design context
- keep archive materials as provenance, not execution authority

### Verification checkpoint

```bash
rg -n "docs/specs/MAP.md|supersede|superseded|non-canonical|Packet 4.2|pipeline" \
  HANDBOOK_PUBLISHED_IMPORT_DECOUPLING_AUDIT_2026-06-23.md \
  docs/specs/handbook-published-import-decoupling-set-1-pipeline-boundary-authority-reconciliation-*.md
```

### Exit condition

A fresh session can read `docs/specs/MAP.md` plus active docs only and understand the real state without diffing archive history manually.

## Packet 1.4 — Final Proof And Set 2/3 Handoff

### Goal

Close Set 1 honestly and hand off clean Set 2 and Set 3 start points.

### Work

- re-run the docs-only proof wall
- confirm no Rust source changed during Set 1
- record the chosen Set 2 boundary shape and exact stop boundary
- record the Set 3 proof obligations required by the MAP
- leave explicit completion notes in the tasks doc

### Verification checkpoint

```bash
git diff -- docs/specs/MAP.md docs/specs HANDBOOK_PUBLISHED_IMPORT_DECOUPLING_AUDIT_2026-06-23.md
```

### Exit condition

Set 1 closes as a reviewed docs-only authority set and Sets 2 and 3 can start without reopening the same objective debate.

## Implementation Order

1. **Packet 1.1 first** because Set 1 must start from an evidence wall, not intuition.
2. **Packet 1.2 second** because there is no honest docs reconciliation until the boundary shape is explicit.
3. **Packet 1.3 third** because active authority should reflect a settled decision, not an in-progress debate.
4. **Packet 1.4 last** because the handoff wall depends on the earlier packets already landing.

## Risks And Mitigations

### Risk 1: treating published `engine + flow` proof as if it already proved `pipeline`

- **Why it matters:** this is the exact confusion Set 1 exists to eliminate.
- **Mitigation:** require an explicit side-by-side proof matrix and keep Packet 4.2 classified as an `engine + flow` seam only.

### Risk 2: rewriting archive history instead of creating new active authority

- **Why it matters:** archive files are provenance and may be ignored by Git policy; editing them may not create a durable new planning surface.
- **Mitigation:** keep Set 1 active authority under `/Users/spensermcconnell/__Active_Code/system/docs/specs/` and use archive references only as historical inputs.

### Risk 3: accidentally starting Set 2 implementation inside Set 1

- **Why it matters:** once code edits begin, the authority conflict can get papered over instead of being resolved.
- **Mitigation:** keep Set 1 docs-only and require `git diff` at Packet 1.4 to confirm no Rust files changed.

### Risk 4: overweighting stale Substrate context

- **Why it matters:** the `9b83` notes contain useful provider-boundary ideas but are explicitly non-canonical.
- **Mitigation:** cite them only as secondary design context, never as stronger authority than live `system` repo truth.

### Risk 5: solving the capability problem by overexposing internals

- **Why it matters:** the MAP intent is maximum capability with minimum unnecessary public surface.
- **Mitigation:** require Set 1 to identify which capabilities are required and prefer a narrower façade whenever it can provide the same capability honestly.

## Verification Wall

Use this as the set-level proof wall for Packet 1.4:

```bash
# live code truth
sed -n '1,120p' crates/pipeline/src/lib.rs
sed -n '1,220p' crates/pipeline/src/declarative_roots.rs
sed -n '1,260p' crates/pipeline/src/layout.rs
sed -n '452,520p' crates/flow/src/resolver.rs

# workspace sanity
cargo check --workspace

# active-docs-only handoff review
rg -n "engine \\+ flow|pipeline|supersede|non-canonical|Set 2|Set 3|Expose capabilities, not guts|minimum unnecessary public surface" \
  docs/specs/MAP.md docs/specs/handbook-published-import-decoupling-set-1-pipeline-boundary-authority-reconciliation-*.md

# docs-only guardrail
git diff -- docs/specs/MAP.md docs/specs HANDBOOK_PUBLISHED_IMPORT_DECOUPLING_AUDIT_2026-06-23.md
```

## Stop Boundary

Stop after Set 1 leaves behind:

- one active reconciled authority set,
- one explicit Set 2 problem statement,
- one exact Set 2 proof wall,
- and one explicit Set 3 proof/guard-rail wall.

Do not:

- edit `crates/engine/**`, `crates/flow/**`, or `crates/pipeline/**`
- widen the published boundary during this set
- rewrite archived packet history as if it never happened
- treat full reusable pipeline capability for Substrate as optional
- widen into CLI/compiler/product-shell redesign
- start downstream Substrate implementation work
