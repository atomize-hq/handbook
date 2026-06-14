# Plan: Handbook Engine Extraction Phase 6 Slice 1 (Slice 6.1) - Migration Readiness Reassessment

## Status

- Planned reassessment slice.
- This plan defines how to decide whether the repo is ready for a separate ownership/integration planning family.
- The packet order below is sequential by default and should stay narrow unless live regression evidence forces a different seam.
- Packet 6.1.2's ownership matrix is now captured in `docs/specs/handbook-engine-extraction-phase-6-slice-1-packet-6-1-2-ownership-matrix.md`.
- Packet 6.1.3's final verdict remains READY because the prior `handbook-engine` boundary blocker stays cleared in current live repo truth.
- Packet 6.1.4 now names the exact follow-on family as `Handbook Engine Extraction Phase 6 - Ownership And Integration Planning` while keeping that family itself out of scope for this slice.

## Objective

Revalidate the migration gate, reassess crate ownership posture, and finish with an explicit Phase 6 readiness verdict plus an explicit deferral set.

Spec reference: [handbook-engine-extraction-phase-6-slice-1-migration-readiness-reassessment-spec.md](./handbook-engine-extraction-phase-6-slice-1-migration-readiness-reassessment-spec.md)

## Major Artifacts

1. Live migration-gate baseline
   - freezes the current working-tree truth, authority-doc truth, and verification-wall status
   - prevents Phase 6 from drifting into stale-document interpretation

2. Crate-by-crate ownership assessment matrix
   - evaluates `handbook-engine`, `handbook-pipeline`, `handbook-flow`, `handbook-cli`, and retained `handbook-compiler`
   - uses the root ownership decision rule instead of optimistic future assumptions
   - lives in `docs/specs/handbook-engine-extraction-phase-6-slice-1-packet-6-1-2-ownership-matrix.md`

3. Final reassessment verdict
   - decides whether the repo is ready for a separate ownership/integration planning family
   - distinguishes true blockers from intentional deferrals

4. Explicit deferral boundary
   - records what later ownership/integration planning work is justified if Phase 6 finishes READY
   - keeps naming or authoring the follow-on family out of scope for Packet 6.1.3

## Planned Packet Order

### Packet 6.1.1: Freeze live repo truth and revalidate the migration gate

Why first:

- Phase 6 only means anything if it is grounded in the actual current checkout
- a dirty tree, stale authority wording, or failing verification wall would invalidate later ownership conclusions
- this packet establishes whether reassessment is being performed against committed truth or local-only truth

Output:

- recorded branch / HEAD / working-tree posture
- current authority-doc snapshot for root plan, slice map, and closeout map
- rerun verification wall and any targeted evidence rails needed to anchor the earlier closeout claims

### Packet 6.1.2: Reassess extracted crate boundaries against the ownership rule

Why second:

- the root plan already defines the decision rule; Phase 6 must now apply it to real crate boundaries
- engine, pipeline, flow, CLI, and retained compiler seam each need their own evidence-backed posture
- the right outcome may differ per crate, so this packet should not collapse everything into one “move or don’t move” answer

Output:

- one crate-by-crate matrix covering handbook-domain versus substrate-domain center of gravity
- explicit notes on where handbook-product assumptions still matter
- explicit notes on where Substrate can likely consume through a clean boundary
- artifact: `docs/specs/handbook-engine-extraction-phase-6-slice-1-packet-6-1-2-ownership-matrix.md`

### Packet 6.1.3: Resolve the readiness verdict and explicit deferral set

Why third:

- a green verification wall is necessary but not sufficient
- Phase 6 must separate “ready for follow-on planning” from “ownership still too ambiguous” and from “real regression blocker”
- explicit deferrals keep later agents from widening this slice into early ownership/import implementation

Output:

- one final verdict: ready for a separate ownership/integration planning family, or not ready because of a named blocker
- explicit unresolved questions that remain legitimate follow-on planning inputs rather than hidden blockers
- explicit statement of whether `handbook-compiler` is merely retained temporary glue or still a readiness ambiguity

Landed Packet 6.1.3 result:

- Final verdict: **READY** for a separate ownership/integration planning family.
- Prior blocker cleared: `handbook-engine` no longer exposes the handbook-product-named layout boundary; live repo truth now uses `default_canonical_layout_contract` instead, and `handbook-flow` fallback sites consume that generic default owner.
- Packet 6.1.1's verification wall remains the authoritative regression evidence; this packet did not rerun unchanged expensive commands.
- Cheap live-truth revalidation at committed HEAD `c8d9e7222b3b2e436a9484fc59f6ec923f2a01b6` still shows no `handbook_product_canonical_layout_contract` usage in `crates/engine` or `crates/flow`, while `default_canonical_layout_contract` remains the live engine export and flow fallback path.
- Handbook-owned/imported now: `handbook-engine` and `handbook-pipeline`.
- Handbook-side deferrals that remain explicit but non-blocking: `handbook-flow`, `handbook-cli`, and retained `handbook-compiler`.
- Retained `handbook-compiler` is classified as temporary transition glue rather than a remaining readiness ambiguity.
- Still deferred: naming or authoring the ownership/integration planning family itself, any code repair, and any actual crate-move or runtime-widening work.

### Packet 6.1.4: Name the next planning artifact boundary without starting it

Why last:

- the user asked for the final planning boundary, not for Phase 6 to silently continue into the next family
- if READY, the next artifact family must be named clearly enough that a future session can start cleanly
- if NOT READY, the next artifact must be the narrow repair seam rather than a vague “more Phase 6”

Output:

- exact next planning family name and scope if READY
- exact blocker seam name and scope if NOT READY
- no ownership/import plan authored yet

Landed Packet 6.1.4 result:

- Current verdict path: **READY**, so Packet 6.1.4 names the next family as `Handbook Engine Extraction Phase 6 - Ownership And Integration Planning`.
- Exact artifact boundary for that future family: `docs/specs/handbook-engine-extraction-phase-6-ownership-and-integration-planning-{spec,plan,tasks}.md`.
- Exact next-family scope: decide per-crate architectural ownership, define handbook-import versus future substrate-move criteria, and sequence later execution seams without starting code implementation or packet prompts.
- Existing local draft docs at that artifact stem remain outside Packet 6.1.4 authority; this packet only names the boundary and does not approve, revise, or land those drafts.
- The NOT READY branch stays preserved as a rule only: if later evidence flips the verdict, route the blocker back to a named earlier seam instead of reopening vague “more Phase 6”.

## Risks And Mitigations

### Risk: Phase 6 overclaims readiness because the test wall is green

Mitigation:

- require crate-by-crate ownership calls, not only green tests
- require explicit handling of `handbook-flow`, `handbook-cli`, and retained `handbook-compiler`, which are the easiest places to overstate certainty

### Risk: intentional bounded runtime posture is mistaken for a regression

Mitigation:

- evaluate boundedness against the root plan’s explicit deferred posture
- only call it blocking if live code/docs/tests contradict the migration gate or the claimed closeout state

### Risk: stale docs or local dirt distort the reassessment

Mitigation:

- start with `git status --short --branch` and `git log --oneline --decorate -20`
- re-read root authority files before making any ownership call

### Risk: Phase 6 widens into ownership/import planning implementation

Mitigation:

- end the slice at the readiness verdict and explicit deferrals
- do not name or author the follow-on family inside this packet

### Risk: a real blocker is found but gets buried as “open questions”

Mitigation:

- distinguish blockers from open questions explicitly
- if a blocker belongs to an earlier seam, route it back to that seam rather than leaving it as vague Phase 6 uncertainty

## Verification Checkpoints

### Checkpoint 1: Truth surface is frozen before conclusions

Confirm:

- branch / HEAD / working-tree posture is recorded
- root authority files still name Phase 6 as the next step

Suggested verification:

```bash
git status --short --branch
git log --oneline --decorate -20
rg -n "Phase 6|Migration Gate|next authoritative step" HANDBOOK_ENGINE_EXTRACTION_PLAN.md docs/specs/handbook-engine-extraction-slice-map.md docs/specs/handbook-engine-extraction-closeout-four-set-map.md
```

### Checkpoint 2: Earlier closeout evidence still holds

Confirm targeted representative rails still pass:

```bash
cargo test -p handbook-engine --test canonical_artifacts_ingest
cargo test -p handbook-engine --test freshness_computation
cargo test -p handbook-engine --test baseline_validation
cargo test -p handbook-flow --test resolver_core
cargo test -p handbook-cli --test help_drift_guard
cargo check --workspace
```

### Checkpoint 3: Ownership posture is explicit for every crate

Confirm:

- every extracted crate has an evidence-backed call
- handbook-domain versus substrate-domain reasoning is written down
- retained compiler posture is handled explicitly
- Packet 6.1.2 matrix artifact is populated and referenced by the Phase 6 authority docs

Suggested verification:

```bash
rg -n "pub use|pub mod|mod " crates/engine/src/lib.rs crates/pipeline/src/lib.rs crates/flow/src/lib.rs crates/compiler/src/lib.rs crates/cli/src/main.rs
cargo tree -p handbook-engine
cargo tree -p handbook-pipeline
cargo tree -p handbook-flow
cargo tree -p handbook-compiler
```

### Checkpoint 4: Final readiness verdict is justified by the full wall

Confirm:

```bash
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
```

## Planned Exit Conditions

Slice 6.1 should be considered complete only when all of the following are true:

- the Phase 1 through Phase 5 migration gate has been revalidated against live repo truth
- the verification wall is green, or any failure is explicitly named as a blocker
- `handbook-engine`, `handbook-pipeline`, `handbook-flow`, `handbook-cli`, and retained `handbook-compiler` each have an explicit ownership/readiness posture
- the slice ends with an explicit readiness verdict and explicit deferral set
- no follow-on ownership/import planning has started yet inside this slice
