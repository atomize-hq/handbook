# Spec: Handbook Published-Import Decoupling — Set 3: Published Consumer Proof + Substrate Proof + Guard Rails

## Assumptions I'm Making

1. Set 2 is complete only at the **packaged-boundary** level: the retained `handbook-pipeline` public boundary exists in live source and the packaged external proof passed locally on 2026-06-23, but that does **not** yet count as released-crate proof or downstream Substrate proof.
2. The currently published crates.io `handbook-pipeline = 0.1.1` does **not** contain the full Set 2 public boundary. The Set 2 packaged proof exercises `PipelineDeclarativeRootsContract`, `PipelineStorageLayoutContract`, the retained metadata/definition `*_with_roots` entrypoints, and the retained route-state/capture/handoff `*_with_storage_layout` entrypoints, but that proof landed only in the current unpublished `handbook-pipeline` source/package. Set 3 must therefore route through a newly published version before released-consumer proof can count as complete.
3. Set 3 may touch both `/Users/spensermcconnell/__Active_Code/system` and a dedicated Substrate worktree under `/Users/spensermcconnell/.codex/worktrees/`, but the active execution authority for the workstream remains the `system` repo docs.
4. Set 3 must **not** widen the public surface beyond the Set 2 retained/dropped matrix unless the active authority is explicitly reopened first.
5. Packet 4.2 remains only an `engine + flow` proof; Set 3 must not reclassify that seam as `handbook-pipeline` proof.
6. Unless live release constraints prove otherwise, Set 3 should prefer the narrowest publish necessary to make the Set 2 boundary real on crates.io, and should avoid dragging unrelated `engine`, `flow`, CLI, or compiler redesign into this proof set.

## Active Authority Routing For Set 3

Use the authority stack in this order:

1. `/Users/spensermcconnell/__Active_Code/system/docs/specs/MAP.md`
   - exact objective, exact intent, success criteria, set sequencing, and downstream worktree rule
2. the active Set 1 triplet
   - authority reconciliation, Set 2 acceptance wall, and Set 3 proof/guard-rail handoff wall
3. the active Set 2 triplet
   - the retained/dropped public boundary matrix and the packaged-boundary closeout line that Set 3 must now turn into released proof
4. this active Set 3 triplet
   - concrete implementation authority for released-consumer proof, downstream Substrate proof, and guard rails
5. `/Users/spensermcconnell/__Active_Code/system/HANDBOOK_PUBLISHED_IMPORT_DECOUPLING_AUDIT_2026-06-23.md`
   - freshness evidence and starting-state audit input only
6. `/Users/spensermcconnell/__Active_Code/system/docs/specs/archive/`
   - provenance only, including the older archived Set 3 parameterization docs
7. `/Users/spensermcconnell/.codex/worktrees/substrate-packet-4-2-20260622-133054/` plus related downstream notes
   - downstream context and current proof baseline only; not stronger authority than the active `system` docs

If any archived Set 3 note conflicts with the MAP, the active Set 1 triplet, the active Set 2 triplet, or this Set 3 triplet, treat the archived note as superseded for active planning.

## Objective

Close the overall handbook published-import decoupling effort by proving that the reviewed Set 2 `handbook-pipeline` boundary works as a **real published boundary** and as a **real downstream Substrate seam**, while preserving the MAP ownership split and the MAP rule of **maximum functional capability with minimum unnecessary public surface**.

Set 3 is successful only if it makes all of the following true at the same time:

- a released crates.io consumer can use the Set 2 boundary through public APIs only
- Substrate can use the capability it actually needs through that same published boundary in a dedicated downstream worktree
- final wording, runtime behavior, and downstream user experience remain Substrate-owned where the MAP says they should
- release/update guard rails prevent the workstream from drifting back into false-complete claims

## Exact Intent For This Set

Set 3 is not a second API-design set.

Set 3 is the proof-and-honesty set that must:

1. turn the Set 2 packaged proof into a **real released-crate proof**,
2. prove **real downstream Substrate usage** against the published surface,
3. keep Packet 4.2 classified only as `engine + flow` proof,
4. keep the Set 2 public boundary minimal instead of widening it to make proof easier,
5. add regression protection so “docs say public but crates.io says private” cannot silently recur.

If Set 3 discovers that the retained Set 2 boundary is insufficient for Substrate's real needs, the correct action is to reopen authority explicitly rather than silently widening public surface inside Set 3.

## Tech Stack

- Rust 2021 workspace in `/Users/spensermcconnell/__Active_Code/system`
- Primary published crates under proof:
  - `handbook-pipeline`
  - `handbook-engine`
  - `handbook-flow`
- Primary proof assets already landed from Set 2:
  - `/Users/spensermcconnell/__Active_Code/system/tools/proof/handbook_pipeline_minimal_boundary.sh`
  - `/Users/spensermcconnell/__Active_Code/system/tests/fixtures/external_consumers/handbook_pipeline_minimal_boundary/`
- Primary downstream baseline worktree currently proving only `engine + flow`:
  - `/Users/spensermcconnell/.codex/worktrees/substrate-packet-4-2-20260622-133054`
- Primary downstream baseline file currently proving only `engine + flow`:
  - `/Users/spensermcconnell/.codex/worktrees/substrate-packet-4-2-20260622-133054/crates/shell/src/execution/prompt_fulfillment.rs`
- Expected new Set 3 proof assets:
  - `/Users/spensermcconnell/__Active_Code/system/tools/proof/handbook_pipeline_released_boundary.sh`
  - `/Users/spensermcconnell/__Active_Code/system/tests/fixtures/external_consumers/handbook_pipeline_released_boundary/`

## Commands

```bash
# Re-read the active authority stack before implementation
sed -n '1,340p' docs/specs/MAP.md
sed -n '170,190p' docs/specs/handbook-published-import-decoupling-set-1-pipeline-boundary-authority-reconciliation-tasks.md
sed -n '368,378p' docs/specs/handbook-published-import-decoupling-set-2-minimal-public-capability-boundary-spec.md
sed -n '223,228p' docs/specs/handbook-published-import-decoupling-set-2-minimal-public-capability-boundary-plan.md
sed -n '117,122p' docs/specs/handbook-published-import-decoupling-set-2-minimal-public-capability-boundary-tasks.md

# Re-run the Set 2 boundary regression wall before any release work
cargo test -p handbook-pipeline --test pipeline_catalog
cargo test -p handbook-pipeline --test pipeline_loader
cargo test -p handbook-pipeline --test pipeline_state_store
cargo test -p handbook-pipeline --test pipeline_route_resolution
cargo test -p handbook-pipeline --test pipeline_capture
cargo test -p handbook-pipeline --test pipeline_handoff
cargo check --workspace

# Package + publish verification in system
cargo package -p handbook-pipeline --allow-dirty
cargo publish --dry-run -p handbook-pipeline
cargo publish -p handbook-pipeline

# Packaged-boundary proof inherited from Set 2
bash tools/proof/handbook_pipeline_minimal_boundary.sh

# Released-boundary proof to add in Set 3
bash tools/proof/handbook_pipeline_released_boundary.sh --version <published_version>

# Dedicated downstream Substrate worktree verification
cargo tree -p handbook-engine
cargo tree -p handbook-flow
cargo tree -p handbook-pipeline
cargo check --workspace

# Repo-required commit gate in system
npx gitnexus detect-changes --repo system
```

## Project Structure

```text
/Users/spensermcconnell/__Active_Code/system/
  docs/specs/MAP.md
    -> root authority for exact objective, exact intent, and set sequencing
  docs/specs/handbook-published-import-decoupling-set-1-*.md
    -> Set 1 authority reconciliation and Set 3 proof/guard-rail handoff wall
  docs/specs/handbook-published-import-decoupling-set-2-*.md
    -> Set 2 retained/dropped boundary matrix and packaged-proof closeout line
  docs/specs/handbook-published-import-decoupling-set-3-published-consumer-proof-substrate-proof-guard-rails-*.md
    -> active Set 3 execution authority

  tools/proof/handbook_pipeline_minimal_boundary.sh
    -> existing Set 2 packaged-boundary proof harness
  tools/proof/handbook_pipeline_released_boundary.sh
    -> new Set 3 released-crate proof harness to add

  tests/fixtures/external_consumers/handbook_pipeline_minimal_boundary/
    -> existing packaged-boundary external consumer fixture
  tests/fixtures/external_consumers/handbook_pipeline_released_boundary/
    -> new exact-published-version external consumer fixture to add

/Users/spensermcconnell/.codex/worktrees/substrate-packet-4-2-20260622-133054/
  crates/shell/src/execution/prompt_fulfillment.rs
    -> current `engine + flow` baseline seam; provenance only, not sufficient Set 3 proof

/Users/spensermcconnell/.codex/worktrees/
  <future dedicated Substrate Set 3 worktree>
    -> required write location for downstream source-touching proof; never use the main Substrate checkout
```

## Code Style

Use only the reviewed public boundary in proof code, and keep the ownership split explicit:

```rust
use handbook_pipeline::{
    pipeline::{
        load_pipeline_catalog_metadata_with_roots,
        load_selected_pipeline_definition_with_roots,
    },
    route_state::load_trusted_pipeline_session_with_storage_layout,
    PipelineDeclarativeRootsContract,
    PipelineStorageLayoutContract,
};

// handbook owns typed reusable mechanics
let roots = PipelineDeclarativeRootsContract::try_from_paths(
    "custom/core/pipelines",
    "custom/core/profiles",
    "custom/core/runners",
    "custom/core/stages",
)?;

// Substrate still owns final wording and runtime behavior
let pipeline = load_selected_pipeline_definition_with_roots(repo_root, &roots, selector)?;
let session = load_trusted_pipeline_session_with_storage_layout(repo_root, &pipeline, layout)?;
let substrate_owned_message = format!("Substrate selected {}", session.pipeline_id);
```

Conventions for Set 3 docs/proof code:

- prefer exact published-version pins over loose dependency ranges in proof fixtures
- keep comments explicit about whether something is:
  - packaged proof only,
  - released-crate proof,
  - downstream proof,
  - or provenance only
- use explicit wording such as `engine + flow only`, `pipeline released proof`, and `downstream Substrate proof` instead of broad “published consumption complete” shorthand
- do not import private-module paths such as `handbook_pipeline::layout::*` or `handbook_pipeline::declarative_roots::*`

## Testing Strategy

### 1. Upstream Set 2 regression wall

Before any release or downstream proof work counts, rerun the targeted `handbook-pipeline` tests plus `cargo check --workspace` in `/Users/spensermcconnell/__Active_Code/system`.

### 2. Released external consumer proof

Set 3 must add a released-crate proof harness that:

- depends on the **exact published version** of `handbook-pipeline`
- exercises the retained Set 2 public families through public APIs only
- runs outside the source tree
- fails if it falls back to sibling-path or source-tree dependency accidents

### 3. Downstream Substrate proof

Set 3 must prove one narrow Substrate-owned production seam in a dedicated worktree that:

- resolves the exact published crate version
- uses the reviewed public boundary only
- preserves Substrate-owned wording and runtime behavior
- does not touch the main Substrate checkout

### 4. Guard-rail verification

Set 3 guard rails must verify all of the following:

- active docs do not classify Packet 4.2 as `handbook-pipeline` proof
- released proof is distinguishable from packaged proof
- future release/update work re-runs the released-boundary external consumer proof
- downstream proof claims remain tied to an exact published version and an exact proof seam

## Boundaries

- Always:
  - re-read `docs/specs/MAP.md` plus the active Set 1 and Set 2 handoff lines before Set 3 execution
  - keep Packet 4.2 classified only as `engine + flow` proof
  - use a dedicated Substrate worktree for any downstream source-touching proof
  - prove exact published crates.io versions, not local path dependencies or source-tree checkouts
  - preserve the Set 2 retained/dropped matrix unless authority is explicitly reopened
  - run `npx gitnexus detect-changes --repo system` before every real commit in `system`
- Ask first:
  - publishing a new crates.io version
  - widening the public API beyond the Set 2 retained/dropped matrix
  - changing CI or release automation beyond the narrow guard rails required for Set 3
  - using a different downstream proof seam if the first narrow seam proves insufficient
- Never:
  - claim packaged proof is the same as released-crate proof
  - claim `engine + flow` proof is the same as `handbook-pipeline` proof
  - perform downstream source-touching proof in the main Substrate checkout
  - use sibling-path or direct source-tree dependency accidents as proof
  - silently widen public surface or expose private helper modules to “make proof easier”

## Success Criteria

Set 3 is complete only when all of the following are true:

1. A newly published crates.io version of `handbook-pipeline` contains the Set 2 public boundary.
2. A released external consumer depending on that exact published version can construct non-default declarative-roots and storage-layout contracts and exercise the retained public capability families through public APIs only.
3. A dedicated downstream Substrate worktree proves one narrow production seam can use the published `handbook-pipeline` boundary while keeping wording and runtime behavior Substrate-owned.
4. The Set 3 proof artifacts explicitly distinguish:
   - packaged proof,
   - released external proof,
   - and downstream Substrate proof.
5. Packet 4.2 remains explicitly classified only as `handbook-engine` + `handbook-flow` proof.
6. No Set 3 closeout note claims capability beyond what the released external proof and downstream Substrate proof actually demonstrate.
7. Release/update guard rails exist so future work cannot honestly pass if:
   - crates.io no longer exposes the claimed public boundary,
   - docs overclaim the boundary relative to the released crate,
   - or `engine + flow` proof is mislabeled as pipeline proof.

## Packet 3.1 release-preparation note (2026-06-23)

- The released-boundary harness now lives at `/Users/spensermcconnell/__Active_Code/system/tools/proof/handbook_pipeline_released_boundary.sh` and the exact-version external consumer fixture now lives at `/Users/spensermcconnell/__Active_Code/system/tests/fixtures/external_consumers/handbook_pipeline_released_boundary/`.
- That harness intentionally keeps the released tier distinct from the Set 2 packaged tier: it requires `--version <published_version>`, pins `handbook-pipeline` with an exact crates.io version in the fixture, copies the proof corpus into an isolated temp workspace, and rejects path-dependency or source-tree fallback before any proof run counts.
- `handbook-pipeline 0.1.1` is insufficient for released proof because crates.io already contains that version while the retained Set 2 boundary exercised by the packaged proof exists only in the current unpublished `handbook-pipeline` source/package.
- The smallest honest publish target for Packet 3.2 is `handbook-pipeline 0.1.2` only. Packet 3.1 found no evidence that `handbook-engine` or `handbook-flow` require a coordinated version train: `cargo publish --dry-run -p handbook-pipeline` still verifies against the existing `handbook-engine 0.1.1` dependency, the released-proof fixture imports `handbook-pipeline` only, and Packet 4.2 remains classified only as `engine + flow` proof.

## Open Questions

1. Should the downstream Set 3 proof reuse the current dedicated worktree at `/Users/spensermcconnell/.codex/worktrees/substrate-packet-4-2-20260622-133054` if it stays isolated and clean, or should it start from a fresh dedicated worktree?
2. Which exact narrow downstream production seam is the least-disruptive place to prove `handbook-pipeline` capability while preserving Substrate-owned behavior?
