# Spec: Handbook Engine Extraction Phase 6 Slice 1 (Slice 6.1) - Migration Readiness Reassessment

## Status

- Planned validation/planning slice.
- Phase 6 is the next authoritative step after the landed Phase 1 through Phase 5 extraction work.
- This slice is a reassessment boundary only. It must not be treated as permission to start ownership/import implementation.
- Packet 6.1.2 ownership reassessment is now recorded in `docs/specs/handbook-engine-extraction-phase-6-slice-1-packet-6-1-2-ownership-matrix.md`.
- Packet 6.1.3 now resolves the readiness verdict and explicit deferrals below.
- Packet 6.1.4 now names the next planning boundary as the `Handbook Engine Extraction Phase 6 - Ownership And Integration Planning` family while keeping that family itself out of scope for Slice 6.1 execution.

## Assumptions

1. The live repo currently satisfies the Phase 1 through Phase 5 migration gate in `HANDBOOK_ENGINE_EXTRACTION_PLAN.md`, and any follow-up to this slice should start from that live truth rather than from stale pre-closeout docs.
2. The current extracted workspace shape is real: `crates/engine`, `crates/pipeline`, `crates/flow`, and `crates/cli` are the primary owner layers, while `crates/compiler` remains a reviewed narrow compatibility/support seam.
3. The bounded current runtime wedge is intentionally retained. Phase 6 must not reinterpret that bounded posture as a hidden regression unless live code or tests prove a real migration-gate failure.
4. The main unresolved problem is no longer extraction mechanics; it is ownership/readiness reassessment: which crates remain handbook-owned and imported by Substrate versus which, if any, should move into Substrate ownership later.
5. This slice should stay narrow and produce a final readiness call plus explicit deferrals. If it uncovers a genuine Phase 1 through Phase 5 regression, that regression should be named explicitly and routed back to a narrow repair seam rather than widened into Phase 6.
6. Success is not “Substrate move planned.” Success is “the repo can now truthfully say whether it is ready for a separate ownership/integration planning family.”

## Objective

Reassess the post-extraction workspace against the root migration rule and decide whether the repo is ready for a separate ownership/integration plan.

The maintainer needs this slice because the extraction work is now landed through the closeout seams, but the root plan intentionally deferred the final ownership call. Success means:

- the Phase 1 through Phase 5 migration gate is revalidated against live repo truth
- each extracted crate is reassessed against the handbook-domain versus substrate-domain ownership rule
- the bounded current wedge is evaluated honestly without confusing intentional deferral with regression
- the repo ends with an explicit READY / NOT READY style migration-readiness reassessment
- the slice names the follow-on ownership/integration planning family clearly enough for a future session to start it cleanly without authoring or starting that family here

## Tech Stack

- Rust 2021 workspace
- crates in scope:
  - `handbook-engine`
  - `handbook-pipeline`
  - `handbook-flow`
  - `handbook-cli`
  - retained `handbook-compiler` compatibility/support seam
- planning authorities:
  - `HANDBOOK_ENGINE_EXTRACTION_PLAN.md`
  - `docs/specs/handbook-engine-extraction-slice-map.md`
  - `docs/specs/handbook-engine-extraction-closeout-four-set-map.md`
- live validation surfaces:
  - `crates/engine/src/**`
  - `crates/pipeline/src/**`
  - `crates/flow/src/**`
  - `crates/cli/src/**`
  - `crates/compiler/src/**`
  - `crates/**/tests/**`

## Commands

Truth-surface freeze:

```bash
git status --short --branch
git log --oneline --decorate -20
```

Root-authority sweep:

```bash
rg -n "Phase 6|Migration Gate|Exit criteria|Open Questions|next authoritative step" HANDBOOK_ENGINE_EXTRACTION_PLAN.md
rg -n "Phase 6|next step|fully landed through Slice 5.3|Authority And Assumptions" docs/specs/handbook-engine-extraction-slice-map.md
rg -n "Phase 6|verification wall|four-set closeout|next authoritative step" docs/specs/handbook-engine-extraction-closeout-four-set-map.md
```

Ownership-surface sweep:

```bash
rg -n "pub use|pub mod|mod " crates/engine/src/lib.rs crates/pipeline/src/lib.rs crates/flow/src/lib.rs crates/compiler/src/lib.rs crates/cli/src/main.rs
cargo tree -p handbook-engine
cargo tree -p handbook-pipeline
cargo tree -p handbook-flow
cargo tree -p handbook-compiler
```

Targeted evidence rails:

```bash
cargo test -p handbook-engine --test canonical_artifacts_ingest
cargo test -p handbook-pipeline --test pipeline_catalog
cargo test -p handbook-cli --test help_drift_guard
```

Final verification wall:

```bash
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
```

## Project Structure

```text
HANDBOOK_ENGINE_EXTRACTION_PLAN.md                                   -> root migration-gate authority and Phase 6 decision rule
docs/specs/handbook-engine-extraction-slice-map.md                   -> Phase -> Slice -> Packet authority through the landed extraction family
docs/specs/handbook-engine-extraction-closeout-four-set-map.md       -> closeout-shape authority for how Phases 1 through 5 were honestly finished
docs/specs/handbook-engine-extraction-phase-1-slice-5-*.md           -> layout/storage parameterization closeout authority
docs/specs/handbook-engine-extraction-phase-2-slice-4-*.md           -> orchestration-target parameterization closeout authority
docs/specs/handbook-engine-extraction-phase-4-slice-5-*.md           -> caller-rewire / compiler-narrowing closeout authority
docs/specs/handbook-engine-extraction-phase-5-slice-3-*.md           -> CLI shell closeout authority
docs/specs/handbook-engine-extraction-phase-6-slice-1-packet-6-1-2-ownership-matrix.md -> Packet 6.1.2 crate-by-crate ownership/readiness matrix
crates/engine/src/lib.rs                                             -> engine-owned outward surface and typed layout contract posture
crates/pipeline/src/lib.rs                                           -> pipeline-owned outward surface and supported-target/runtime posture
crates/flow/src/lib.rs                                               -> flow-owned outward surface and middle-layer composition posture
crates/compiler/src/lib.rs                                           -> retained compatibility/support seam that Phase 6 must reassess honestly
crates/cli/src/main.rs                                               -> thin product-shell entrypoint proving CLI remains a shell, not the import target
crates/**/tests/**                                                   -> live regression evidence for parameterization, target bounding, CLI shell stability, and retained-seam posture
docs/specs/handbook-engine-extraction-phase-6-slice-1-*.md           -> this Phase 6 reassessment authority set
```

## Code Style

Prefer explicit evidence-backed ownership decisions over optimistic prose or premature import plans.

```rust
match ownership_decision {
    OwnershipDecision::HandbookOwnedImported => "clean adapter boundary; keep handbook as owner",
    OwnershipDecision::MoveIntoSubstrate => "center of gravity is substrate-specific",
    OwnershipDecision::Deferred => "not ready to move; keep ownership question open explicitly",
}
```

Conventions:

- name the live evidence behind each ownership call
- distinguish handbook-domain logic from substrate-domain pressure
- distinguish intentional boundedness from real regression
- prefer explicit deferral over wishful certainty
- if a real blocker exists, route it back to the narrow earlier seam that owns it

## Testing Strategy

- Framework: existing Rust integration/unit tests plus live authority/doc consistency checks
- Primary test levels:
  - root-authority sweep for Phase 6 wording and migration-gate truth
  - crate-surface inspection for owner-layer boundaries
  - targeted regression tests that anchor the most important completed seams
  - full workspace verification wall
- Coverage focus:
  - Phases 1 through 5 remain green against the migration gate
  - extracted crates are the real owner layers rather than `handbook-compiler`
  - the supported target wedge remains intentionally bounded and validated
  - CLI remains a thin shell rather than the future ownership target
  - Phase 6 conclusions stay grounded in live code/tests/docs, not only in earlier planning claims
- Coverage expectation:
  - if targeted rails or the full wall fail, the slice cannot call migration readiness complete
  - if the wall is green but ownership questions remain open, the slice must report that as a Phase 6 planning/output issue rather than overclaiming readiness

## Slice Scope

In scope:

- revalidate the Phase 1 through Phase 5 migration gate against current repo truth
- reassess `handbook-engine`, `handbook-pipeline`, `handbook-flow`, `handbook-cli`, and retained `handbook-compiler` posture against the root ownership rule
- identify whether any residual handbook-product assumptions still prevent a clean later import boundary
- decide whether the repo is ready for a separate ownership/integration planning family
- make the follow-on ownership/integration planning work explicit as future work without starting it

Out of scope:

- moving crates into Substrate
- writing the ownership/import plan itself
- adding new supported pipelines, stages, or consumers
- reopening a closed Phase 1 through Phase 5 seam without concrete live regression evidence
- broad CLI redesign, runtime widening, or new product behavior

## Authority Inputs

- `HANDBOOK_ENGINE_EXTRACTION_PLAN.md`
- `docs/specs/handbook-engine-extraction-slice-map.md`
- `docs/specs/handbook-engine-extraction-closeout-four-set-map.md`
- closeout authority sets:
  - `docs/specs/handbook-engine-extraction-phase-1-slice-5-layout-parameterization-closeout-spec.md`
  - `docs/specs/handbook-engine-extraction-phase-1-slice-5-layout-parameterization-closeout-plan.md`
  - `docs/specs/handbook-engine-extraction-phase-1-slice-5-layout-parameterization-closeout-tasks.md`
  - `docs/specs/handbook-engine-extraction-phase-2-slice-4-orchestration-target-parameterization-closeout-spec.md`
  - `docs/specs/handbook-engine-extraction-phase-2-slice-4-orchestration-target-parameterization-closeout-plan.md`
  - `docs/specs/handbook-engine-extraction-phase-2-slice-4-orchestration-target-parameterization-closeout-tasks.md`
  - `docs/specs/handbook-engine-extraction-phase-4-slice-5-caller-rewire-and-compiler-narrowing-spec.md`
  - `docs/specs/handbook-engine-extraction-phase-4-slice-5-caller-rewire-and-compiler-narrowing-plan.md`
  - `docs/specs/handbook-engine-extraction-phase-4-slice-5-caller-rewire-and-compiler-narrowing-tasks.md`
  - `docs/specs/handbook-engine-extraction-phase-5-slice-3-cli-shell-closeout-spec.md`
  - `docs/specs/handbook-engine-extraction-phase-5-slice-3-cli-shell-closeout-plan.md`
  - `docs/specs/handbook-engine-extraction-phase-5-slice-3-cli-shell-closeout-tasks.md`
- live crate surfaces:
  - `crates/engine/src/lib.rs`
  - `crates/pipeline/src/lib.rs`
  - `crates/flow/src/lib.rs`
  - `crates/compiler/src/lib.rs`
  - `crates/cli/src/main.rs`
- live evidence tests:
  - `crates/engine/tests/canonical_artifacts_ingest.rs`
  - `crates/pipeline/tests/pipeline_catalog.rs`
  - `crates/cli/tests/help_drift_guard.rs`
- Packet 6.1.2 matrix artifact:
  - `docs/specs/handbook-engine-extraction-phase-6-slice-1-packet-6-1-2-ownership-matrix.md`

## Current Repo-Truth Questions This Reassessment Must Resolve

| Surface | Current live posture | Phase 6 reassessment question |
| --- | --- | --- |
| `handbook-engine` | typed canonical truth and layout contract surface is exported from the engine crate | is this crate handbook-domain reusable core that Substrate should import rather than own? |
| `handbook-pipeline` | target/runtime posture is catalog-backed and intentionally bounded | is this crate reusable enough to import as-is, or does its center of gravity still remain handbook-owned for longer? |
| `handbook-flow` | middle-layer composition remains the least clearly settled domain seam | is this genuinely reusable workflow composition, or still handbook-specific enough to defer any ownership move? |
| `handbook-cli` | CLI is now the thin product shell | should this remain purely handbook-owned and explicitly excluded from any ownership/import target? |
| retained `handbook-compiler` seam | compiler remains narrowed compatibility/support glue rather than the implementation center | can later planning treat this as temporary retained glue to retire, and not as a future ownership target? |

## Boundaries

- Always:
  - treat live repo truth as authoritative
  - rerun the verification wall before making the final readiness call
  - keep Phase 6 centered on reassessment, ownership criteria, and explicit deferrals
  - separate “tests green” from “ownership decision complete”
- Ask first:
  - any code change
  - any root-plan or slice-map wording change beyond what the human explicitly requested
  - starting the follow-on ownership/import planning family
- Never:
  - treat bounded current runtime posture as automatic proof that pipeline/flow belong in Substrate
  - start ownership/import implementation inside this slice
  - reopen closed Phase 1 through Phase 5 work without concrete live regression evidence
  - call migration readiness complete while crate-by-crate ownership posture remains implicit

## Packet 6.1.2 landed matrix

Packet 6.1.2 now records the explicit crate-by-crate ownership/readiness matrix in:

- `docs/specs/handbook-engine-extraction-phase-6-slice-1-packet-6-1-2-ownership-matrix.md`

That matrix answers, for each crate:

- handbook-domain versus substrate-domain center of gravity
- whether Substrate should likely import it through a clean boundary
- whether ownership should remain handbook-side longer
- which handbook-product assumptions still matter to the call

## Packet 6.1.3 landed verdict

Final verdict: **READY** for a separate ownership/integration planning family.

Verdict basis:

- Packet 6.1.1 already froze the Phase 6 baseline against unstaged local truth at HEAD `a883d16`, explicitly preserving unrelated `AGENTS.md` / `CLAUDE.md` edits and four untracked follow-on planning docs outside packet scope; that frozen Packet 6.1.1 verification wall remains the authoritative regression evidence for Phase 6.
- Packet 6.1.2 already made every in-scope crate explicit instead of leaving ownership/readiness implicit.
- Cheap live-truth revalidation at committed HEAD `c8d9e7222b3b2e436a9484fc59f6ec923f2a01b6` confirms the prior blocker remains cleared: `handbook_product_canonical_layout_contract` is gone from `crates/engine` and `crates/flow`, `handbook-engine` still exports `default_canonical_layout_contract`, and the remaining `handbook-flow` fallback sites still consume that generic default owner rather than a handbook-product-named boundary accessor.
- The remaining charter / project-context / environment-inventory surfaces are handbook-domain engine semantics, not evidence of an active handbook-product layout boundary, so they are follow-on planning inputs rather than a readiness blocker.

### Final crate-by-crate readiness posture

| Crate | Packet 6.1.3 posture | Blocker status | Non-blocking open question or explicit deferral |
| --- | --- | --- | --- |
| `handbook-engine` | Handbook-owned/imported through a generic default layout contract rather than a handbook-product-named boundary. | No blocker. The prior boundary-level readiness blocker remains cleared in current live repo truth at `c8d9e7222b3b2e436a9484fc59f6ec923f2a01b6`. | The later planning family still needs to decide whether this surface stays handbook-owned/imported as-is or whether any thinner adapter should be formalized. |
| `handbook-pipeline` | Handbook-owned/imported through the already-reviewed boundary. | No blocker. The intentionally bounded runtime wedge is still acceptable for readiness. | Keep the compiler-backed test-fixture coupling and longer-term bounded-runtime posture explicit as follow-on planning inputs, not hidden blockers. |
| `handbook-flow` | Explicitly deferred handbook-side middle layer, not a current move target. | No blocker. | The long-term ownership question stays open for the next family, but the current evidence is already clear enough to avoid blocking readiness. |
| `handbook-cli` | Handbook-owned product shell and explicitly outside any import target. | No blocker. | Any future CLI product split or redesign is deferred; it is not part of migration-readiness cleanup. |
| retained `handbook-compiler` seam | Retained handbook-side transition glue, not an import target and not the implementation center. | No blocker. | Retirement/narrowing timing remains explicitly deferred, but the seam is no longer a readiness ambiguity for Phase 6. |

### Readiness blockers

- None from current live repo truth at `c8d9e7222b3b2e436a9484fc59f6ec923f2a01b6`. The prior `handbook-engine` blocker was specifically the handbook-product-named layout boundary, and that boundary is no longer active in code.
- No production-code regression has been reintroduced since the authoritative Packet 6.1.1 verification wall; this packet relies on that frozen wall rather than rerunning unchanged expensive commands.

### Non-blocking open questions

- Should `handbook-pipeline` keep the current compiler-backed test fixture path longer, or should the later planning family narrow that coupling further?
- Does `handbook-flow` ever earn a future ownership move, or does it remain handbook-owned longer-term?
- Which `rendering`, `refusal`, or error-surfaces should the later ownership/integration planning family treat as explicit handbook-owned boundaries?
- When should retained `handbook-compiler` glue be narrowed or retired after the planning family is defined?
- Does `handbook-engine` need any thinner adapter boundary for Substrate consumption, or is the current extracted surface already the correct import seam?

### Explicit deferrals that remain out of scope

- Authoring any separate future ownership/integration planning family itself.
- Naming the exact future ownership/integration planning family or treating any local draft docs for it as landed authority.
- Any production-code repair, crate move, or runtime widening.
- Any decision to move `handbook-flow`, `handbook-cli`, or retained `handbook-compiler` out of handbook ownership.
- Any retirement/narrowing work for retained `handbook-compiler` beyond documenting it as transition glue.

### Retained `handbook-compiler` readiness call

For Phase 6 readiness, retained `handbook-compiler` is temporary transition glue, not a still-blocking ownership ambiguity. The only deferred work is when and how later planning narrows or retires that glue.

## Packet 6.1.4 named next boundary

Packet 6.1.4 lands the planning-boundary statement that Packet 6.1.3 deliberately deferred.

Named next family:

- `Handbook Engine Extraction Phase 6 - Ownership And Integration Planning`
- artifact stem: `docs/specs/handbook-engine-extraction-phase-6-ownership-and-integration-planning-{spec,plan,tasks}.md`

Why this is the next honest step:

- the root Phase 6 exit criterion is now satisfied: the repo is ready for a separate ownership and integration plan
- Packet 6.1.2 already made the crate-by-crate ownership posture explicit, and Packet 6.1.3 already established that none of the remaining questions are readiness blockers
- the remaining work is planning-family work: decide handbook-owned import seams versus any future substrate-owned move targets, define the integration decision framework, and sequence later execution seams without widening into implementation

Still out of scope in Slice 6.1:

- treating any local draft `handbook-engine-extraction-phase-6-ownership-and-integration-planning-*` docs as landed authority
- authoring or revising that follow-on family inside this packet
- any production-code repair, crate move, runtime widening, or ownership/import implementation
- any blocker-routing action, because the live Packet 6.1.3 verdict remains **READY** rather than **NOT READY**

## Success Criteria

- The Phase 1 through Phase 5 migration gate is revalidated against live repo truth.
- The full verification wall is green, or any failure is explicitly named as a blocker.
- Each crate in the extracted workspace has an explicit ownership/readiness assessment.
- The retained `handbook-compiler` seam is described honestly as either temporary glue to retire later or as a still-blocking ambiguity.
- The slice ends with a clear readiness verdict for separate ownership/integration planning.
- The slice names the exact next ownership/integration planning family without silently beginning it.

## Open Questions

- Should `handbook-pipeline` remain handbook-owned longer than `handbook-engine`, even if Substrate can consume it through a clean adapter boundary?
- Does `handbook-flow` ever earn a later ownership move, or is the honest long-term outcome still handbook-owned composition?
- Which parts of `rendering`, `refusal`, and `error` still need ownership clarification before a later import plan can be honest?
- At what point should retained `handbook-compiler` support glue be retired instead of remaining as a transition seam?
