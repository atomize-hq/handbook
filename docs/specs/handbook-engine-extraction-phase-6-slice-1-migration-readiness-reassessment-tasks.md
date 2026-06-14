# Tasks: Handbook Engine Extraction Phase 6 Slice 1 (Slice 6.1) - Migration Readiness Reassessment

Plan reference: [handbook-engine-extraction-phase-6-slice-1-migration-readiness-reassessment-plan.md](./handbook-engine-extraction-phase-6-slice-1-migration-readiness-reassessment-plan.md)

## Status

- Planned reassessment task ledger.
- This checklist exists to execute Phase 6 validation and produce the final readiness verdict plus explicit deferrals.
- No code implementation or ownership/import planning should begin from this file without a later explicit approval step.
- Packet 6.1.1 is now revalidated against unstaged local truth at HEAD `a883d16`; unrelated unstaged `AGENTS.md` and `CLAUDE.md` edits plus four untracked follow-on planning docs were preserved outside packet scope.
- Packet 6.1.2 is now recorded against committed Packet 6.1.1 truth at `30b22d5`; at Packet 6.1.2 capture time, unrelated local edits in `AGENTS.md` and `CLAUDE.md` were preserved outside packet scope.
- Packet 6.1.3 is now revalidated at committed HEAD `c8d9e7222b3b2e436a9484fc59f6ec923f2a01b6` with a READY verdict because the prior `handbook-engine` boundary blocker remains cleared in live repo truth.
- Packet 6.1.4 now names the exact next family as `Handbook Engine Extraction Phase 6 - Ownership And Integration Planning` while keeping that family itself out of scope for this packet.

## Implementation Authority Used

Before execution, this slice is grounded in:

- `HANDBOOK_ENGINE_EXTRACTION_PLAN.md`
- `docs/specs/handbook-engine-extraction-slice-map.md`
- `docs/specs/handbook-engine-extraction-closeout-four-set-map.md`
- the landed Phase 1 Slice 1.5 closeout triplet
- the landed Phase 2 Slice 2.4 closeout triplet
- the landed Phase 4 Slice 4.5 closeout triplet
- the landed Phase 5 Slice 5.3 closeout triplet

This slice is a reassessment seam. It should validate readiness and make later planning explicit without silently widening into implementation or into the follow-on ownership/import planning family.

## Packet 6.1.1: Freeze Live Repo Truth And Revalidate The Migration Gate

- [x] Task: Record working-tree truth and authority-doc truth before making any readiness call
  - Acceptance: Branch, HEAD, and working-tree posture are recorded; the root plan, slice map, and closeout map still name Phase 6 as the next authoritative step; the execution clearly states whether it is validating committed truth or local-only truth.
  - Verify: `git status --short --branch && git log --oneline --decorate -20 && rg -n "Phase 6|Migration Gate|Exit criteria|Open Questions|next authoritative step" HANDBOOK_ENGINE_EXTRACTION_PLAN.md && rg -n "Phase 6|next step|fully landed through Slice 5.3|Authority And Assumptions" docs/specs/handbook-engine-extraction-slice-map.md && rg -n "Phase 6|verification wall|four-set closeout|next authoritative step" docs/specs/handbook-engine-extraction-closeout-four-set-map.md`
  - Files: `HANDBOOK_ENGINE_EXTRACTION_PLAN.md`, `docs/specs/handbook-engine-extraction-slice-map.md`, `docs/specs/handbook-engine-extraction-closeout-four-set-map.md`, `docs/specs/handbook-engine-extraction-phase-6-slice-1-migration-readiness-reassessment-*.md`, `docs/specs/handbook-engine-extraction-phase-6-slice-1-packet-6-1-1-live-truth-freeze.md`
  - Completion note: Recorded the current dirty `feat/seam-extraction` checkout at local HEAD `a883d16`, explicitly preserved the unrelated unstaged `AGENTS.md` / `CLAUDE.md` edits plus the untracked follow-on planning docs outside packet scope, and refroze the root-plan / slice-map / closeout-map authority agreement in `docs/specs/handbook-engine-extraction-phase-6-slice-1-packet-6-1-1-live-truth-freeze.md`.

- [x] Task: Re-run the representative proof rails and full workspace wall before any final verdict
  - Acceptance: The representative closeout rails and the full verification wall are green, or any failure is captured as a concrete blocker rather than omitted from the Phase 6 call.
  - Verify: `cargo test -p handbook-engine --test canonical_artifacts_ingest && cargo test -p handbook-pipeline --test pipeline_catalog && cargo test -p handbook-cli --test help_drift_guard && cargo fmt --all -- --check && cargo clippy --workspace --all-targets -- -D warnings && cargo test --workspace`
  - Files: `crates/engine/tests/canonical_artifacts_ingest.rs`, `crates/pipeline/tests/pipeline_catalog.rs`, `crates/cli/tests/help_drift_guard.rs`, `docs/specs/handbook-engine-extraction-phase-6-slice-1-packet-6-1-1-live-truth-freeze.md`
  - Completion note: This Packet 6.1.1 revalidation pass reran `cargo test -p handbook-engine --test canonical_artifacts_ingest`, `cargo test -p handbook-pipeline --test pipeline_catalog`, `cargo test -p handbook-cli --test help_drift_guard`, `cargo fmt --all -- --check`, `cargo clippy --workspace --all-targets -- -D warnings`, and `cargo test --workspace`; all passed cleanly against unstaged local truth at `a883d16` while preserving the unrelated `AGENTS.md` / `CLAUDE.md` edits and four untracked follow-on planning docs outside packet scope, so no true blocker or production regression was found.

## Packet 6.1.2: Reassess Extracted Crate Boundaries Against The Ownership Rule

- [x] Task: Reassess `handbook-engine` and `handbook-pipeline` against handbook-domain versus substrate-domain center of gravity
  - Acceptance: Each crate has an explicit evidence-backed posture that answers whether Substrate should import it through a clean boundary or whether moving ownership would be cleaner later; intentional boundedness is not misreported as a blocker by itself.
  - Verify: `rg -n "pub use|pub mod|mod " crates/engine/src/lib.rs crates/pipeline/src/lib.rs && cargo tree -p handbook-engine && cargo tree -p handbook-pipeline`
  - Files: `crates/engine/src/lib.rs`, `crates/pipeline/src/lib.rs`, `crates/engine/tests/canonical_artifacts_ingest.rs`, `crates/pipeline/tests/pipeline_catalog.rs`, `docs/specs/handbook-engine-extraction-phase-6-slice-1-migration-readiness-reassessment-*.md`
  - Completion note: `docs/specs/handbook-engine-extraction-phase-6-slice-1-packet-6-1-2-ownership-matrix.md` now records both crates as handbook-domain centers of gravity that Substrate should likely consume through a clean boundary rather than absorb.

- [x] Task: Reassess `handbook-flow`, `handbook-cli`, and retained `handbook-compiler` posture explicitly
  - Acceptance: `handbook-flow` is not left as an implied afterthought, `handbook-cli` is treated honestly as a product shell rather than a move target, and retained `handbook-compiler` posture is classified as either temporary glue to retire later or a still-blocking ambiguity.
  - Verify: `rg -n "pub use|pub mod|mod " crates/flow/src/lib.rs crates/compiler/src/lib.rs crates/cli/src/main.rs && cargo tree -p handbook-flow && cargo tree -p handbook-compiler`
  - Files: `crates/flow/src/lib.rs`, `crates/compiler/src/lib.rs`, `crates/cli/src/main.rs`, `docs/specs/handbook-engine-extraction-phase-6-slice-1-migration-readiness-reassessment-*.md`
  - Completion note: The Packet 6.1.2 matrix now records `handbook-flow` as a handbook-side middle layer, `handbook-cli` as the product shell rather than any move target, and retained `handbook-compiler` as compatibility/support glue rather than the implementation center.

## Packet 6.1.3: Resolve The Readiness Verdict And Explicit Deferrals

- [x] Task: Carry the Packet 6.1.2 ownership/readiness matrix into explicit blockers versus open questions
  - Acceptance: The already-authored Packet 6.1.2 matrix is used to separate blockers from non-blocking open questions before the final verdict; no crate is left implicit.
  - Verify: Manual review against `HANDBOOK_ENGINE_EXTRACTION_PLAN.md` Phase 6 checklist, the live crate surfaces named in the spec, and `docs/specs/handbook-engine-extraction-phase-6-slice-1-packet-6-1-2-ownership-matrix.md`.
  - Files: `HANDBOOK_ENGINE_EXTRACTION_PLAN.md`, `docs/specs/handbook-engine-extraction-phase-6-slice-1-packet-6-1-2-ownership-matrix.md`, `docs/specs/handbook-engine-extraction-phase-6-slice-1-migration-readiness-reassessment-spec.md`, `docs/specs/handbook-engine-extraction-phase-6-slice-1-migration-readiness-reassessment-plan.md`, `docs/specs/handbook-engine-extraction-phase-6-slice-1-migration-readiness-reassessment-tasks.md`
  - Completion note: Packet 6.1.3 now makes every crate explicit without leaving any blocker implicit: `handbook-engine` stays handbook-owned/imported and is no longer blocking because the handbook-product-named layout boundary is gone in live code; `handbook-pipeline`, `handbook-flow`, `handbook-cli`, and retained `handbook-compiler` remain explicit non-blocking handbook-side postures or deferrals.

- [x] Task: Write the final Phase 6 verdict without beginning the follow-on planning family
  - Acceptance: The slice ends with an explicit readiness call for separate ownership/integration planning, or with a named narrow blocker seam if the repo is not ready; the output does not silently spill into authoring the next family.
  - Verify: Manual review against the Phase 6 success criteria and planned exit conditions in the spec/plan.
  - Files: `docs/specs/handbook-engine-extraction-phase-6-slice-1-migration-readiness-reassessment-*.md`
  - Completion note: Packet 6.1.3 now lands a READY verdict because the old `handbook-engine` blocker stays cleared in live repo truth at `c8d9e7222b3b2e436a9484fc59f6ec923f2a01b6`; the remaining open questions stay separate from readiness, retained `handbook-compiler` remains temporary transition glue rather than a readiness ambiguity, and naming any exact follow-on planning family remains explicitly deferred.

## Packet 6.1.4: Name The Next Planning Boundary

- [x] Task: If READY, name the exact next planning family without starting it
  - Acceptance: The output names the follow-on ownership/integration planning family clearly enough for a future session to start it cleanly, but does not generate that family inside Slice 6.1.
  - Verify: Manual review that the named next family is distinct from this reassessment slice and does not reopen a closed earlier seam.
  - Files: `docs/specs/handbook-engine-extraction-phase-6-slice-1-migration-readiness-reassessment-*.md`
  - Completion note: Because Packet 6.1.3 remains explicitly **READY**, Packet 6.1.4 now names the next family as `Handbook Engine Extraction Phase 6 - Ownership And Integration Planning` with artifact stem `docs/specs/handbook-engine-extraction-phase-6-ownership-and-integration-planning-{spec,plan,tasks}.md`; this is the next honest step because the remaining work is ownership/integration decision framing, not more reassessment and not implementation.

- [x] Task: If NOT READY, route the blocker back to a narrow earlier seam rather than vague “more Phase 6”
  - Acceptance: Any blocking regression is attached to a concrete earlier seam owner (for example layout parameterization, target parameterization, caller/compiler narrowing, or CLI shell closeout) instead of being left as generic unfinished reassessment work.
  - Verify: Manual review against `docs/specs/handbook-engine-extraction-closeout-four-set-map.md` and the relevant landed closeout triplet.
  - Files: `docs/specs/handbook-engine-extraction-closeout-four-set-map.md`, `docs/specs/handbook-engine-extraction-phase-6-slice-1-migration-readiness-reassessment-*.md`
  - Completion note: Not exercised in the current READY path, but the boundary remains explicit: if future evidence ever flips Phase 6 back to **NOT READY**, the blocker must route to a concrete earlier seam owner from the closeout map rather than vague “more Phase 6”.

## Human Review Gate

Do not start or author any follow-on ownership/integration planning family from this task ledger until the human reviews the Phase 6 verdict and explicitly approves that next step.
