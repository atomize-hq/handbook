# Spec: Handbook Published-Import Decoupling — Set 2: Minimal Public Capability Boundary for `handbook-pipeline`

## Assumptions I'm Making

1. Set 1 is complete and remains the active prerequisite authority for this set; `/Users/spensermcconnell/__Active_Code/system/docs/specs/MAP.md` plus the Set 1 triplet control any conflict with older archive-era docs.
2. Set 2 is implementation work inside `/Users/spensermcconnell/__Active_Code/system`; it does **not** include downstream Substrate source-touching proof, CLI/product-shell redesign, or broad compiler refactors.
3. The intended external consumer shape for Set 2 is **both**:
   - a narrow, Substrate-owned provider/context boundary that constructs non-default pipeline contracts once and then calls reviewed public handbook entrypoints, and
   - a direct downstream call-site shape small enough to fit into one narrow production seam without requiring handbook product wording to leak into Substrate.
4. The honest first-wave Set 2 target is to expose **contract-aware public façade entrypoints**, not to make `handbook_pipeline::declarative_roots` or `handbook_pipeline::layout` public modules.
5. Existing handbook-product default entrypoints should stay available for handbook's own product behavior; Set 2 adds or promotes reviewed contract-aware variants without widening into handbook-only convenience helpers.
6. Because the new boundary is not yet released, the Set 2 closeout proof should validate the **packaged public boundary** from outside the crate source tree (for example via `cargo package` output or an unpacked release-candidate artifact), while Set 3 remains responsible for final released-crate and downstream Substrate proof.
7. Unless live implementation proves otherwise, no new public result/error/output types are needed beyond the reviewed public contract owners plus already-public pipeline-facing types.
8. Packet 2.2 must be decomposed into additive retained-façade landing, caller/test migration, and dropped-seam privacy clamp sub-packets because the retained loaders are high-blast-radius seams and the dropped seams still have active in-repo callers.

## Active Authority Routing For Set 2

Use the authority stack in this order:

1. `/Users/spensermcconnell/__Active_Code/system/docs/specs/MAP.md`
   - exact objective, exact intent, success criteria, and set sequencing
2. the active Set 1 triplet
   - required boundary principle, bounded candidate proof surface, and Set 2 acceptance wall
3. this Set 2 triplet
   - concrete implementation authority for the minimal public capability boundary
4. `/Users/spensermcconnell/__Active_Code/system/HANDBOOK_PUBLISHED_IMPORT_DECOUPLING_AUDIT_2026-06-23.md`
   - freshness evidence and current-state audit input only
5. `/Users/spensermcconnell/__Active_Code/system/docs/specs/archive/`
   - provenance only
6. `/Users/spensermcconnell/.codex/worktrees/9b83/substrate/docs/`
   - stale, non-canonical provider-boundary context only

If any archive-era document conflicts with the MAP, the Set 1 triplet, or this Set 2 triplet, treat the archive-era document as superseded for active planning.

## Objective

Implement the smallest reviewed public boundary that gives Substrate the full reusable `handbook-pipeline` capability it actually needs through a stable published seam.

This set must make all of the following possible through public crate APIs:

- choose non-default declarative roots for pipelines/profiles/runners/stages
- choose non-default storage roots for route state, capture, and handoff outputs
- load and validate reusable pipeline metadata/definitions against those chosen roots
- use reusable route-state, capture, and handoff mechanics without private-module reach-in
- preserve the intended ownership split:
  - handbook owns reusable typed mechanics and contracts
  - Substrate owns product/runtime wording, integration behavior, and downstream experience

This set must **not** claim overall workstream success by itself. It lands the minimal public capability boundary and a release-candidate external proof wall, while Set 3 remains responsible for final released-consumer proof, downstream Substrate proof, and regression guard rails.

## Tech Stack

- Rust 2021 workspace in `/Users/spensermcconnell/__Active_Code/system`
- Crates under active boundary review:
  - `handbook-pipeline`
  - `handbook-flow`
  - `handbook-engine`
- Current published versions used in current-state validation:
  - `handbook-engine = 0.1.1`
  - `handbook-pipeline = 0.1.1`
  - `handbook-flow = 0.1.1`
- Primary source-owner files for Set 2:
  - `/Users/spensermcconnell/__Active_Code/system/crates/pipeline/src/lib.rs`
  - `/Users/spensermcconnell/__Active_Code/system/crates/pipeline/src/declarative_roots.rs`
  - `/Users/spensermcconnell/__Active_Code/system/crates/pipeline/src/layout.rs`
  - `/Users/spensermcconnell/__Active_Code/system/crates/pipeline/src/pipeline.rs`
  - `/Users/spensermcconnell/__Active_Code/system/crates/pipeline/src/route_state.rs`
  - `/Users/spensermcconnell/__Active_Code/system/crates/pipeline/src/pipeline_capture.rs`
  - `/Users/spensermcconnell/__Active_Code/system/crates/pipeline/src/pipeline_handoff.rs`
- Primary test surfaces:
  - `/Users/spensermcconnell/__Active_Code/system/crates/pipeline/tests/pipeline_catalog.rs`
  - `/Users/spensermcconnell/__Active_Code/system/crates/pipeline/tests/pipeline_loader.rs`
  - `/Users/spensermcconnell/__Active_Code/system/crates/pipeline/tests/pipeline_state_store.rs`
  - `/Users/spensermcconnell/__Active_Code/system/crates/pipeline/tests/pipeline_route_resolution.rs`
  - `/Users/spensermcconnell/__Active_Code/system/crates/pipeline/tests/pipeline_capture.rs`
  - `/Users/spensermcconnell/__Active_Code/system/crates/pipeline/tests/pipeline_handoff.rs`

## Commands

```bash
# Re-read governing authority before implementation
sed -n '1,360p' docs/specs/MAP.md
sed -n '254,340p' docs/specs/handbook-published-import-decoupling-set-1-pipeline-boundary-authority-reconciliation-spec.md
sed -n '53,190p' docs/specs/handbook-published-import-decoupling-set-1-pipeline-boundary-authority-reconciliation-tasks.md

# Inspect live boundary truth
sed -n '1,170p' crates/pipeline/src/declarative_roots.rs
sed -n '1,220p' crates/pipeline/src/layout.rs
sed -n '1380,1548p' crates/pipeline/src/pipeline.rs
sed -n '2590,2635p' crates/pipeline/src/pipeline.rs
sed -n '438,760p' crates/pipeline/src/route_state.rs
sed -n '1140,1268p' crates/pipeline/src/route_state.rs
sed -n '180,310p' crates/pipeline/src/pipeline_capture.rs
sed -n '220,260p' crates/pipeline/src/pipeline_handoff.rs
sed -n '536,560p' crates/pipeline/src/pipeline_handoff.rs

# Targeted proof wall during Set 2 implementation
cargo test -p handbook-pipeline --test pipeline_catalog
cargo test -p handbook-pipeline --test pipeline_loader
cargo test -p handbook-pipeline --test pipeline_state_store
cargo test -p handbook-pipeline --test pipeline_route_resolution
cargo test -p handbook-pipeline --test pipeline_capture
cargo test -p handbook-pipeline --test pipeline_handoff
cargo check --workspace

# Release-candidate external consumer proof for Set 2 closeout
cargo package -p handbook-pipeline --allow-dirty
cargo publish --dry-run -p handbook-pipeline
bash tools/proof/handbook_pipeline_minimal_boundary.sh

# Repo-required commit gate
npx gitnexus detect-changes --repo system
```

## Project Structure

```text
/Users/spensermcconnell/__Active_Code/system/
  docs/specs/MAP.md
    → root authority for exact objective, exact intent, and set sequencing
  docs/specs/handbook-published-import-decoupling-set-1-pipeline-boundary-authority-reconciliation-*.md
    → Set 1 authority lock, bounded candidate proof surface, and Set 2 acceptance wall
  docs/specs/handbook-published-import-decoupling-set-2-minimal-public-capability-boundary-*.md
    → this active Set 2 triplet

  crates/pipeline/src/lib.rs
    → reviewed public façade exports; must not make whole private modules public
  crates/pipeline/src/declarative_roots.rs
    → public declarative-roots contract owner and validation/constructor logic
  crates/pipeline/src/layout.rs
    → public storage-layout contract owner and validation/constructor logic
  crates/pipeline/src/pipeline.rs
    → declarative-root-aware catalog / selection / definition entrypoints
  crates/pipeline/src/route_state.rs
    → storage-layout-aware route-state and trusted-session entrypoints
  crates/pipeline/src/pipeline_capture.rs
    → storage-layout-aware capture entrypoints
  crates/pipeline/src/pipeline_handoff.rs
    → storage-layout-aware handoff entrypoints

  crates/pipeline/tests/
    pipeline_catalog.rs
    pipeline_loader.rs
    pipeline_state_store.rs
    pipeline_route_resolution.rs
    pipeline_capture.rs
    pipeline_handoff.rs
      → package-local boundary and regression proof

  tools/proof/
    handbook_pipeline_minimal_boundary.sh
      → release-candidate external consumer proof driver for Set 2 closeout
  tests/fixtures/external_consumers/handbook_pipeline_minimal_boundary/
    Cargo.toml
    src/main.rs
      → external consumer fixture that uses only the reviewed public boundary
```

## Intended External Consumer Shape

The intended first-wave consumer is a **Substrate-owned pipeline boundary provider** that:

1. constructs a non-default declarative-roots contract and a non-default storage-layout contract,
2. calls public contract-aware `handbook-pipeline` entrypoints only,
3. receives typed pipeline/route-state/capture/handoff outputs,
4. keeps final product/runtime wording in Substrate, and
5. can be inserted into one narrow downstream seam rather than forcing product-shell rewrites.

This consumer shape is intentionally narrower than “any downstream code can call any pipeline helper directly.”

## Packet 2.2 Decomposition Rule

Packet 2.2 is intentionally split into three sequential sub-packets:

1. **Packet 2.2a — Retained Façade Landing**
   - add or promote only the retained declarative-root-aware public entrypoints
   - keep existing default entrypoints behaviorally stable
   - do **not** privatize dropped seams yet
2. **Packet 2.2b — Caller/Test Migration Off Dropped Seams**
   - migrate in-repo callers and package-local tests away from `SupportedTargetRegistry::load(...)` and route-aware `load_pipeline_catalog(...)`
   - keep the change scoped to replacing dropped-seam usage with retained/public alternatives
3. **Packet 2.2c — Dropped-Seam Privacy Clamp**
   - only after known callers are migrated, make `SupportedTargetRegistry::load(...)` and route-aware `load_pipeline_catalog(...)` private
   - finish the declarative-root-family public-API-only proof wall

This decomposition is part of the active Set 2 authority. Do not collapse these three concerns back into one packet unless the retained/dropped matrix is explicitly reopened first.

## Retained / Dropped Justification Matrix

This matrix satisfies the Set 1 mandatory start gate for Set 2. Every retained row names the MAP-required capability it serves and why the intended consumer shape needs that exact public seam. Every dropped row explains why it stays private.

### New public contract owners

| Item | Decision | Capability served | Consumer-shape justification |
|---|---|---|---|
| `PipelineDeclarativeRootsContract` | **Retain** | declarative-root control; stage-root-aware catalog/loading | Downstream must choose non-default pipeline/profile/runner/stage roots through one typed public owner rather than private-module reach-in. |
| `PipelineStorageLayoutContract` | **Retain** | storage-layout control for route-state, capture, and handoff | Downstream must choose non-default state/capture/handoff roots through one typed public owner rather than handbook-product defaults. |
| `RuntimeStateLayoutContract` | **Drop / keep private** | none directly | Nested helper detail inside storage layout; exposing it would widen the surface beyond the single reviewed storage contract owner. |
| `CaptureStorageLayoutContract` | **Drop / keep private** | none directly | Nested helper detail; a narrower parent contract already carries the required capability. |
| `HandoffBundleLayoutContract` | **Drop / keep private** | none directly | Nested helper detail; public exposure would freeze implementation decomposition unnecessarily. |
| `RepoLayoutRoot` | **Drop / keep private** | none directly | Repo/path plumbing helper; downstream needs chosen outputs, not handbook's internal layout traversal owner. |

### Declarative-root candidate entrypoints from Set 1

| Candidate path | Decision | Capability served | Consumer-shape justification |
|---|---|---|---|
| `SupportedTargetRegistry::load` via a public contract-aware variant | **Drop** | none uniquely | The first-wave consumer does not need registry topology as a public contract if metadata selection and definition load already cover the required downstream capability. Keep internal unless live proof later shows a strict need. |
| `load_pipeline_catalog` via a public contract-aware variant | **Drop** | none uniquely | Route-aware catalog loading is broader than the first-wave consumer needs and mixes in behavior not required to provide custom-root capability. Metadata-only browsing plus explicit definition load is the narrower honest seam. |
| `load_pipeline_catalog_metadata` via a public contract-aware variant | **Retain** | declarative-root control | Needed so downstream can browse available pipeline metadata from non-default declarative roots through the public boundary only. |
| `load_pipeline_selection_metadata` via a new public contract-aware variant | **Retain** | declarative-root control | Needed so downstream can resolve selectors against non-default roots without private helper access. |
| `load_pipeline_definition` via a public contract-aware variant | **Retain** | declarative-root control; stage-root-aware loading | Needed so downstream can load a chosen pipeline definition from non-default roots through public APIs only. |
| `load_selected_pipeline_definition` via a new public contract-aware variant | **Retain** | declarative-root control; stage-root-aware loading | Needed for the narrow provider shape that starts from selectors instead of path-plumbing knowledge. |

### Storage-layout candidate entrypoints from Set 1

| Candidate path | Decision | Capability served | Consumer-shape justification |
|---|---|---|---|
| `load_route_state` via a public contract-aware variant | **Retain** | route-state storage-layout control | Needed to read route state from non-default state roots. |
| `set_route_state` via a public contract-aware variant | **Retain** | route-state storage-layout control | Needed to mutate route state under non-default state roots without private helper access. |
| `load_trusted_pipeline_session` via a public contract-aware variant | **Retain** | route-state storage-layout control | Needed to build trusted route-aware pipeline sessions from non-default storage roots through the public boundary. |
| `persist_route_basis` via a public contract-aware variant | **Retain** | route-state storage-layout control | Needed to persist trusted route basis snapshots under non-default storage roots. |
| `preview_pipeline_capture` via a public contract-aware variant | **Retain** | capture storage-layout control | Needed for the narrow downstream provider to preview capture effects before apply under non-default storage roots. |
| `capture_pipeline_output` via a public contract-aware variant | **Drop** | none uniquely | The first-wave consumer can achieve the same capability through retained preview + apply entrypoints without exposing an extra convenience seam. |
| `apply_pipeline_capture` via a public contract-aware variant | **Retain** | capture storage-layout control | Needed to apply a persisted preview under non-default storage roots. |
| `load_pipeline_capture_cache_entry` via a public contract-aware variant | **Drop** | none uniquely | The first-wave consumer does not need raw cache-entry inspection if preview results plus apply are sufficient; keep private unless live proof shows resume/inspection requires it. |
| `emit_pipeline_handoff_bundle` via a public contract-aware variant | **Retain** | handoff storage-layout control | Needed to emit handoff bundles to non-default roots while keeping handbook product defaults private. |
| `validate_pipeline_handoff_bundle` via a public contract-aware variant | **Retain** | handoff storage-layout control | Needed to validate emitted bundles under the chosen non-default storage layout. |

### Public result / error / output type rule

- **Default rule:** retain existing already-public result/error/output types where the retained entrypoints already use them.
- **No new public result/error/output types should be introduced** unless implementation proves that an already-public type cannot carry a retained capability.
- If a new public result/error/output type becomes necessary during implementation, update this matrix first with a retain/dropped decision before code progress counts.

## Expected Public Boundary Shape

Set 2 should land the following reviewed public shape:

1. **One public declarative-roots contract owner**
   - validated constructor
   - stable read accessors for pipeline/profile/runner/stage roots
   - handbook-product default helpers remain private unless later proof shows strict need
2. **One public storage-layout contract owner**
   - validated constructor
   - stable read accessors for state/capture/handoff roots
   - nested helper structs remain private
3. **Public contract-aware façade entrypoints only for retained matrix rows**
   - the façade should promote existing private `*_with_roots` / `*_with_storage_layout` seams where those names are already honest
   - add new contract-aware selector-based entrypoints only where no retained public seam already exists
4. **No raw private-module promotion**
   - `handbook_pipeline::declarative_roots::*` stays private
   - `handbook_pipeline::layout::*` stays private
   - `crates/pipeline/src/lib.rs` should expose only the reviewed façade and contract owners
5. **Packet 2.2 must land in three passes**
   - additive retained-façade landing first
   - caller/test migration second
   - dropped-seam privacy clamp third

## Code Style

Prefer explicit public façade wrappers that expose typed capability owners while keeping internal plumbing private.

```rust
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PipelineDeclarativeRootsContract {
    pipeline_root_relative: &'static str,
    profile_root_relative: &'static str,
    runner_root_relative: &'static str,
    stage_root_relative: &'static str,
}

impl PipelineDeclarativeRootsContract {
    pub fn try_from_paths(
        pipeline_root_relative: &'static str,
        profile_root_relative: &'static str,
        runner_root_relative: &'static str,
        stage_root_relative: &'static str,
    ) -> Result<Self, String> {
        let contract = Self::from_paths(
            pipeline_root_relative,
            profile_root_relative,
            runner_root_relative,
            stage_root_relative,
        );
        validate_pipeline_declarative_roots_contract(contract)?;
        Ok(contract)
    }
}

pub fn load_pipeline_definition_with_roots(
    repo_root: impl AsRef<Path>,
    roots: PipelineDeclarativeRootsContract,
    pipeline_path: impl AsRef<Path>,
) -> Result<PipelineDefinition, PipelineLoadError> {
    load_pipeline_definition_with_mode_and_roots(
        repo_root.as_ref(),
        &roots,
        pipeline_path.as_ref(),
        PipelineLoadMode::RouteAware,
    )
}
```

Key conventions:

- public API names should describe the retained capability, not handbook implementation structure
- keep handbook-product defaults behind explicit default helpers or existing default-only entrypoints
- do not publish nested helper structs merely because a public parent contract needs them internally
- keep comments honest about whether a function is public boundary, handbook default behavior, or private plumbing

## Testing Strategy

1. **Contract-owner tests**
   - validate non-default declarative roots reject invalid repo-relative paths
   - validate non-default storage layouts reject invalid containment relationships
   - keep these tests close to `declarative_roots.rs` and `layout.rs`
2. **Package-local integration tests for retained declarative-root seams**
   - prove custom-root metadata browse, selector resolution, and definition load through retained public entrypoints only
   - update `pipeline_catalog.rs` and `pipeline_loader.rs`
3. **Package-local integration tests for retained storage-layout seams**
   - prove custom-layout route-state, trusted-session, capture preview/apply, and handoff emit/validate through retained public entrypoints only
   - update `pipeline_state_store.rs`, `pipeline_route_resolution.rs`, `pipeline_capture.rs`, and `pipeline_handoff.rs`
4. **Negative boundary tests**
   - prove dropped/private seams remain private or unused
   - avoid tests that import `handbook_pipeline::layout::*` or `handbook_pipeline::declarative_roots::*`
5. **Release-candidate external consumer proof**
   - a packaged external consumer must construct non-default contracts and exercise every retained capability family using only the reviewed public boundary
   - no sibling-path fallback, no workspace-private imports, no unpublished internal module paths
6. **Repo-wide safety wall**
   - `cargo check --workspace`
   - rerun touched `handbook-pipeline` tests before commit
   - run `npx gitnexus detect-changes --repo system` before every real commit in this repo

## Boundaries

- Always:
  - keep `/Users/spensermcconnell/__Active_Code/system/docs/specs/MAP.md` and the Set 1 triplet as stronger authority than older docs
  - keep the retained/dropped matrix honest when implementation decisions change
  - preserve handbook-product default behavior for existing default entrypoints unless the task explicitly changes them
  - keep Packet 2.2 split into 2.2a / 2.2b / 2.2c; do not bundle additive façade landing, caller migration, and privacy clamp into one implementation diff
  - run targeted `handbook-pipeline` tests and `cargo check --workspace` before claiming packet completion
  - run `npx gitnexus detect-changes --repo system` before every commit
- Ask first:
  - widening beyond the retained matrix rows in this spec
  - introducing any new public result/error/output type not listed in this spec
  - changing crate versions, release choreography, or CI/release automation
  - touching downstream Substrate source as part of Set 2 instead of Set 3
- Never:
  - make `handbook_pipeline::declarative_roots` or `handbook_pipeline::layout` public wholesale just to reach a small number of types/functions
  - expose handbook product-shell wording, CLI-only behavior, or repo-plumbing helpers as downstream public API
  - claim Packet 4.2 already proved `handbook-pipeline` adoption
  - call Set 2 complete based only on internal tests or sibling-path access

## Success Criteria

Set 2 is successful only when all of the following are true:

1. The retained/dropped matrix above still matches the landed public boundary.
2. `handbook-pipeline` exposes one reviewed public declarative-roots contract owner and one reviewed public storage-layout contract owner.
3. All retained entrypoints are public and accept the reviewed contract owners without private-module reach-in.
4. All dropped/private seams in this spec remain private or otherwise unexposed.
5. No raw `declarative_roots` or `layout` module promotion is used to satisfy downstream capability.
6. Package-local tests prove custom-root and custom-layout behavior through the retained public façade only.
7. Packet 2.2a, Packet 2.2b, and Packet 2.2c land sequentially and leave no required dropped-seam callers behind before the privacy clamp.
8. A release-candidate external consumer proof exercises every retained capability family from outside the crate source tree using only public APIs.
9. The Set 2 closeout notes explicitly preserve that downstream Substrate source-touching proof is a Set 3 responsibility.

## Open Questions

1. Should the Set 2 closeout proof be satisfied by packaged release-candidate artifacts alone, or should the workstream require an immediately published version before Set 2 can close? This spec assumes packaged release-candidate proof is enough for Set 2 and reserves final released-crate proof for Set 3.
2. If live downstream consumer ergonomics later prove that one-shot capture is materially required, should `capture_pipeline_output_with_storage_layout` be promoted in Set 2, or deferred to a later narrow reopen of the retained/dropped matrix?
3. If Packet 2.2b discovers an additional in-repo dropped-seam caller outside the initially expected file list, can that migration still count as in-scope declarative-root-family work? This spec assumes yes, as long as it remains limited to replacing dropped-seam usage and does not widen into later packet capability.
