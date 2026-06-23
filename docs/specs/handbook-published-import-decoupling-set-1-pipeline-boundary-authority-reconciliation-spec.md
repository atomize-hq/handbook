# Spec: Handbook Published-Import Decoupling — Set 1: Pipeline Boundary Authority Reconciliation

## Assumptions I'm Making

1. This set is **docs-only** and exists to reconcile authority before any new `handbook-pipeline` public-boundary implementation begins.
2. `/Users/spensermcconnell/__Active_Code/system/docs/specs/MAP.md` is now the root active authority for exact objective, exact intent, and top-level set sequencing, and this Set 1 triplet must conform to it.
3. `/Users/spensermcconnell/__Active_Code/system/HANDBOOK_PUBLISHED_IMPORT_DECOUPLING_AUDIT_2026-06-23.md` and `/Users/spensermcconnell/__Active_Code/system/docs/ideas/handbook-substrate-packet-4-2-proof-findings.md` are the freshest planning inputs for this seam.
4. The older Set 1 / Set 3 parameterization docs and the published-crate consumption docs remain useful provenance, but they currently disagree about whether `handbook-pipeline` layout/declarative-root seams are part of the supported published boundary.
5. The `9b83` Substrate docs under `/Users/spensermcconnell/.codex/worktrees/9b83/substrate/docs/` are **non-canonical** and slightly stale; they may inform boundary thinking, but they must not overrule live `system` repo truth.
6. The top-level decomposition is now three sets, per `/Users/spensermcconnell/__Active_Code/system/docs/specs/MAP.md`:
   - **Set 1:** authority reconciliation + objective lock
   - **Set 2:** minimal public capability boundary for `handbook-pipeline`
   - **Set 3:** published-consumer proof + Substrate proof + guard rails

## Objective

Create the first active planning set required after the 2026-06-23 audit invalidated the claim that the full `handbook-engine` / `handbook-flow` / `handbook-pipeline` decoupling story is already honest.

This set does **not** widen the published Rust API or edit production code. Its purpose is to lock one prerequisite truth before implementation:

> Substrate must receive the full reusable handbook capability it actually requires, including the reusable `handbook-pipeline` capability set, but Set 2 must expose only the minimum reviewed public surface needed to provide that capability.

The set must produce an active authority that:

1. reconciles conflicting prior docs,
2. distinguishes **published-consumption proof** from **pipeline public-contract proof**,
3. records the exact objective and exact intent from `/Users/spensermcconnell/__Active_Code/system/docs/specs/MAP.md`,
4. states that full reusable pipeline capability for Substrate is a requirement rather than an optional future,
5. defines the Set 2 implementation wall, and
6. defines the Set 3 proof and guard-rail wall.

## Tech Stack

- Rust 2021 workspace in `/Users/spensermcconnell/__Active_Code/system`
- Published crates under review:
  - `handbook-engine = 0.1.1`
  - `handbook-pipeline = 0.1.1`
  - `handbook-flow = 0.1.1`
- Primary code surfaces inspected:
  - `/Users/spensermcconnell/__Active_Code/system/crates/engine/src/lib.rs`
  - `/Users/spensermcconnell/__Active_Code/system/crates/engine/src/canonical_paths.rs`
  - `/Users/spensermcconnell/__Active_Code/system/crates/flow/src/lib.rs`
  - `/Users/spensermcconnell/__Active_Code/system/crates/flow/src/resolver.rs`
  - `/Users/spensermcconnell/__Active_Code/system/crates/pipeline/src/lib.rs`
  - `/Users/spensermcconnell/__Active_Code/system/crates/pipeline/src/declarative_roots.rs`
  - `/Users/spensermcconnell/__Active_Code/system/crates/pipeline/src/layout.rs`
- Primary authority/provenance docs:
  - `/Users/spensermcconnell/__Active_Code/system/docs/specs/MAP.md`
  - `/Users/spensermcconnell/__Active_Code/system/HANDBOOK_PUBLISHED_IMPORT_DECOUPLING_AUDIT_2026-06-23.md`
  - `/Users/spensermcconnell/__Active_Code/system/docs/ideas/handbook-substrate-packet-4-2-proof-findings.md`
  - `/Users/spensermcconnell/__Active_Code/system/docs/specs/archive/handbook-published-crates-and-substrate-consumption-spec.md`
  - `/Users/spensermcconnell/__Active_Code/system/docs/specs/archive/handbook-substrate-import-adoption-plan.md`
  - `/Users/spensermcconnell/__Active_Code/system/docs/specs/archive/handbook-substrate-parameterization-set-1-pipeline-import-layout-spec.md`
  - `/Users/spensermcconnell/__Active_Code/system/docs/specs/archive/handbook-substrate-parameterization-set-3-import-surface-honesty-cleanup-spec.md`
- Non-canonical context inputs:
  - `/Users/spensermcconnell/.codex/worktrees/9b83/substrate/docs/code-intelligence-program.md`
  - `/Users/spensermcconnell/.codex/worktrees/9b83/substrate/docs/code-intelligence-research/overlays/program-wide/handbook-context-integration.md`
  - `/Users/spensermcconnell/.codex/worktrees/9b83/substrate/docs/code-intelligence-research/crates/context/implications.md`

## Commands

```bash
# Read the root active authority plus the freshest audit/proof memo
sed -n '1,260p' docs/specs/MAP.md
sed -n '1,260p' HANDBOOK_PUBLISHED_IMPORT_DECOUPLING_AUDIT_2026-06-23.md
sed -n '1,260p' docs/ideas/handbook-substrate-packet-4-2-proof-findings.md

# Inspect live crate-boundary truth
sed -n '1,120p' crates/pipeline/src/lib.rs
sed -n '1,220p' crates/pipeline/src/declarative_roots.rs
sed -n '1,260p' crates/pipeline/src/layout.rs
sed -n '1,120p' crates/flow/src/lib.rs
rg -n "pub fn resolve\(|pub fn resolve_with_contract" crates/flow/src/resolver.rs
sed -n '452,520p' crates/flow/src/resolver.rs

# Inspect archived authority claims side by side
rg -n "declarative_roots|layout|frozen subset|private layout|public/import-facing|published" \
  docs/specs/archive/handbook-published-crates-and-substrate-consumption-* \
  docs/specs/archive/handbook-substrate-import-adoption-plan.md \
  docs/specs/archive/handbook-substrate-parameterization-set-1-pipeline-import-layout-* \
  docs/specs/archive/handbook-substrate-parameterization-set-3-import-surface-honesty-cleanup-*

# Reconfirm published packageability / repo verification truth
cargo package -p handbook-engine --allow-dirty
cargo package -p handbook-pipeline --allow-dirty
cargo package -p handbook-flow --allow-dirty
cargo check --workspace

# Positive external-consumer proof: engine + flow compile from crates.io =0.1.1
TMPDIR=$(mktemp -d)
mkdir -p "$TMPDIR/src"
cat > "$TMPDIR/Cargo.toml" <<'EOC'
[package]
name = "engine_flow_ok"
version = "0.1.0"
edition = "2021"

[dependencies]
handbook-engine = "=0.1.1"
handbook-flow = "=0.1.1"
EOC
cat > "$TMPDIR/src/main.rs" <<'EOC'
use handbook_engine::CanonicalLayoutContract;
use handbook_flow::{resolve_with_contract, ResolveRequest};
use std::path::Path;

fn main() {
    let contract = CanonicalLayoutContract::from_paths(
        ".substrate/handbook",
        "charter",
        "charter/CHARTER.md",
        "project_context",
        "project_context/PROJECT_CONTEXT.md",
        "environment_inventory",
        "environment_inventory/ENVIRONMENT_INVENTORY.md",
        "feature_spec",
        "feature_spec/FEATURE_SPEC.md",
    );
    let _ = resolve_with_contract(Path::new("."), ResolveRequest::default(), contract);
}
EOC
cargo check --manifest-path "$TMPDIR/Cargo.toml"

# Negative external-consumer proof: pipeline layout contract is still private
TMPDIR=$(mktemp -d)
mkdir -p "$TMPDIR/src"
cat > "$TMPDIR/Cargo.toml" <<'EOC'
[package]
name = "pipeline_fail"
version = "0.1.0"
edition = "2021"

[dependencies]
handbook-pipeline = "=0.1.1"
EOC
cat > "$TMPDIR/src/main.rs" <<'EOC'
use handbook_pipeline::layout::PipelineStorageLayoutContract;

fn main() {
    let _ = std::mem::size_of::<PipelineStorageLayoutContract>();
}
EOC
cargo check --manifest-path "$TMPDIR/Cargo.toml"
```

## Project Structure

```text
/Users/spensermcconnell/__Active_Code/system/
  docs/specs/MAP.md
    → root active authority for exact objective, exact intent, and top-level set map
  HANDBOOK_PUBLISHED_IMPORT_DECOUPLING_AUDIT_2026-06-23.md
    → current root audit that invalidates the "full decoupling already proven" claim
  docs/ideas/handbook-substrate-packet-4-2-proof-findings.md
    → Packet 4.2 proof memo showing the actually-proven published seam
  docs/specs/
    handbook-published-import-decoupling-set-1-pipeline-boundary-authority-reconciliation-spec.md
      → this file
    handbook-published-import-decoupling-set-1-pipeline-boundary-authority-reconciliation-plan.md
      → Set 1 plan
    handbook-published-import-decoupling-set-1-pipeline-boundary-authority-reconciliation-tasks.md
      → Set 1 packets/tasks
  docs/specs/archive/
    handbook-published-crates-and-substrate-consumption-*.md
      → prior published-consumption authority; provenance only for this set
    handbook-substrate-import-adoption-plan.md
      → prior import plan; provenance only for this set
    handbook-substrate-parameterization-set-{1,3}-*.md
      → prior parameterization claims; provenance only for this set
  crates/engine/src/
    lib.rs, canonical_paths.rs
      → proven published engine seam
  crates/flow/src/
    lib.rs, resolver.rs
      → proven published flow seam
  crates/pipeline/src/
    lib.rs, declarative_roots.rs, layout.rs
      → disputed pipeline public-boundary seam

/Users/spensermcconnell/.codex/worktrees/substrate-packet-4-2-20260622-133054/
  crates/shell/src/execution/prompt_fulfillment.rs
    → dedicated downstream proof seam; proves engine + flow consumption, not pipeline adoption

/Users/spensermcconnell/.codex/worktrees/9b83/substrate/docs/
  code-intelligence-program.md
  code-intelligence-research/overlays/program-wide/handbook-context-integration.md
  code-intelligence-research/crates/context/implications.md
    → stale but useful non-authority context on provider boundaries and why reusable capability should flow through a reviewed boundary instead of broad direct internal coupling
```

## Packet 1.1 Current-State Evidence Matrix

This section is the active Packet 1.1 evidence wall for Set 1. It records the current-state claims that matter for execution against the MAP objective and intent.

### Side-by-side claim matrix

| Source | Concrete claim | MAP alignment / conflict | Live code / published truth | Verdict |
|---|---|---|---|---|
| `/Users/spensermcconnell/__Active_Code/system/docs/specs/MAP.md` | Success requires the full reusable `handbook-pipeline` capability Substrate needs through a reviewed published boundary; `engine + flow` proof alone does not count as pipeline completion. | Governing active authority for both exact objective and exact intent. | Live source still keeps the key `handbook-pipeline` declarative-root and layout seams private, so the MAP objective is not yet met. | **Keep as root authority.** |
| `/Users/spensermcconnell/__Active_Code/system/HANDBOOK_PUBLISHED_IMPORT_DECOUPLING_AUDIT_2026-06-23.md` | `handbook-engine` and `handbook-flow` are real published seams, but `handbook-pipeline` is not yet a verified public import seam and the docs currently overstate that boundary. | Aligns directly with the MAP proof and boundary checks. | Reconfirmed by live source inspection plus the positive `engine + flow` compile and the negative `pipeline` compile against crates.io `=0.1.1`. | **Keep as current-state audit input.** |
| `/Users/spensermcconnell/__Active_Code/system/docs/ideas/handbook-substrate-packet-4-2-proof-findings.md` | Packet 4.2 proved honest published consumption of `handbook-engine` + `handbook-flow` through a narrow Substrate seam, but did **not** prove `handbook-pipeline` adoption or final architecture. | Aligns with the MAP only when kept narrow; conflicts only if misread as full pipeline-capability proof. | The live proof seam uses `handbook_flow::resolve_with_contract(...)` plus `handbook_engine::default_canonical_layout_contract()` and does not consume `handbook-pipeline`. | **Keep, but classify as `engine + flow` proof only.** |
| `/Users/spensermcconnell/__Active_Code/system/docs/specs/archive/handbook-substrate-parameterization-set-1-pipeline-import-layout-tasks.md` | The declarative-root and storage-layout seams are already supported public/import-facing boundaries and no structural import blockers remain. | Conflicts with the MAP proof check because published-consumer verification for those seams is missing and live crates.io behavior disproves the claim. | `crates/pipeline/src/lib.rs` keeps `declarative_roots` and `layout` private, `PipelineDeclarativeRootsContract` is `pub(crate)`, `PipelineStorageLayoutContract` is `pub(crate)`, and an external consumer hits `error[E0603]: module 'layout' is private`. | **Superseded by live published truth.** |
| `/Users/spensermcconnell/__Active_Code/system/docs/specs/archive/handbook-published-crates-and-substrate-consumption-spec.md` | The first-wave published `handbook-pipeline` boundary is intentionally a frozen subset that excludes `declarative_roots` and `layout`. | Partially aligns: it honors the MAP “minimum unnecessary public surface” rule, but it is narrower than the MAP capability objective for full Substrate pipeline needs. | Reconfirmed by the live crate root (`mod declarative_roots; mod layout;`) and by the negative external-consumer compile against crates.io `handbook-pipeline = "=0.1.1"`. | **Keep as first-wave publication truth, not as completion authority.** |
| Live `handbook-flow` / `handbook-engine` public surface (`crates/flow/src/lib.rs`, `crates/flow/src/resolver.rs`) | `handbook_flow::resolve_with_contract(...)` and `handbook_engine::CanonicalLayoutContract` form a real typed published seam for downstream consumers. | Aligns with the MAP proof check for a narrow seam, but only partially satisfies the overall objective because it does not expose the required reusable pipeline capability. | The external scratch crate depending on crates.io `handbook-engine = "=0.1.1"` and `handbook-flow = "=0.1.1"` compiled successfully. | **Keep as proven narrow seam.** |
| Live `handbook-pipeline` source (`crates/pipeline/src/lib.rs`, `crates/pipeline/src/declarative_roots.rs`, `crates/pipeline/src/layout.rs`) | The declarative-root and storage-layout control seams remain implementation-private in the published crate. | Aligns with the MAP gap statement and conflicts with any claim that the pipeline public boundary is already complete. | `lib.rs` keeps `declarative_roots` and `layout` private; `PipelineDeclarativeRootsContract` and `PipelineStorageLayoutContract` are still `pub(crate)`; crates.io import of `handbook_pipeline::layout::PipelineStorageLayoutContract` fails with `E0603`. | **Treat as the governing Set 2 gap.** |

### Reproduced external-consumer proofs

| Proof | Published dependencies | Outcome | Classification |
|---|---|---|---|
| Positive narrow seam proof | `handbook-engine = "=0.1.1"`, `handbook-flow = "=0.1.1"` | `cargo check --manifest-path "$TMPDIR/Cargo.toml"` passed for a scratch crate importing `handbook_engine::CanonicalLayoutContract` and `handbook_flow::{resolve_with_contract, ResolveRequest}`. | Real published-consumer proof for the narrow `engine + flow` seam. |
| Negative pipeline seam proof | `handbook-pipeline = "=0.1.1"` | `cargo check --manifest-path "$TMPDIR/Cargo.toml"` failed for `use handbook_pipeline::layout::PipelineStorageLayoutContract;` with `error[E0603]: module 'layout' is private`. | Current published-boundary truth, not a transient environment failure. |

### Packet 1.1 conclusion

- The current published boundary **does** prove a real `handbook-engine` + `handbook-flow` consumer seam.
- The current published boundary **does not** prove a reusable `handbook-pipeline` consumer seam for declarative-root or storage-layout control.
- Set 2 therefore remains required to expose the minimum reviewed public capability surface that satisfies the MAP objective without overexposing internals.

## Code Style

Use an explicit contradiction-and-decision style anchored to the MAP objective and intent.

```md
## MAP Alignment

- Exact objective: full reusable handbook capability for Substrate through a reviewed published boundary
- Exact intent: maximum capability, minimum unnecessary public surface

## Authority Reconciliation Matrix

| Source | Claim | Live truth | Status |
|---|---|---|---|
| Packet 4.2 proof memo | published `engine + flow` consumption is proven | true | keep |
| archived Set 1 tasks | pipeline storage layout is public/import-facing | false for crates.io `0.1.1` | supersede |
| published-consumption spec | first-wave published boundary excludes `layout` and `declarative_roots` | true for `0.1.1` | keep, but narrow in scope |

## Set 1 conclusion

Set 2 must implement the full reusable `handbook-pipeline` capability Substrate needs, but it must expose only the smallest reviewed API that provides that capability.

## Packet 1.2 decision — Set 2 boundary shape and target

### Requirement lock

The required capability is now fixed for the active Set 1 authority:

- Substrate must be able to use the full reusable `handbook-pipeline` capability it actually needs through a reviewed published boundary.
- That requirement includes declarative-root control, stage-root-aware catalog/loading behavior, and storage-layout control for route state, capture, and handoff paths.
- Packet 4.2 does **not** satisfy that requirement because it proves only a narrow published `engine + flow` seam built around `handbook_flow::resolve_with_contract(...)` plus `handbook_engine::default_canonical_layout_contract()`.
- Packet 4.2 does **not** import `handbook-pipeline`, does **not** exercise public declarative-root control, and does **not** exercise public storage-layout control.

### Boundary-shape decision

Set 2 should satisfy the requirement through a **narrower public façade**, not through wholesale direct promotion of the private `declarative_roots` and `layout` modules.

Why this is the narrowest honest shape:

1. The current private modules mix the required downstream contracts with handbook-product defaults, nested helper structs, repo-path plumbing, and convenience helpers that describe one implementation shape rather than the true downstream capability contract.
2. Live source already shows a likely façade route: the crate has existing public pipeline-facing entrypoints plus private `*_with_roots` / `*_with_storage_layout` seams behind them.
3. Direct module promotion would expose more implementation detail than the MAP allows, while a façade can expose the required typed control surface without freezing internal helper ownership.

### Bounded first-wave candidate proof surface

Set 2 should use the following candidate public boundary as Packet 1.2's concrete first-wave proof wall. Packet 1.2 does **not** yet claim that every listed public path is the settled minimum reviewed surface; Set 2 must justify each retained path against a named MAP-required downstream capability and intended external consumer shape, or shrink the surface explicitly. Set 2 may **not** widen beyond this candidate surface without reopening Set 1 authority.

1. **Public declarative-roots contract surface**
   - one reviewed public contract type for pipeline/profile/runner/stage repo-relative roots
   - validated constructor and stable read accessors
   - handbook-product default helpers/constants stay private unless a later external-consumer proof names a concrete need for a public baseline helper
2. **Public storage-layout contract surface**
   - one reviewed public contract type for state/capture/handoff repo-relative roots
   - validated constructor and stable read accessors
   - handbook-product default helpers/constants stay private unless a later external-consumer proof names a concrete need for a public baseline helper
3. **Public contract-aware entrypoints on existing public pipeline surfaces**
   - **declarative-root candidate entrypoint set:** `handbook_pipeline::pipeline::SupportedTargetRegistry::load`, `handbook_pipeline::pipeline::load_pipeline_catalog`, `handbook_pipeline::pipeline::load_pipeline_catalog_metadata`, `handbook_pipeline::pipeline::load_pipeline_selection_metadata`, `handbook_pipeline::pipeline::load_pipeline_definition`, and `handbook_pipeline::pipeline::load_selected_pipeline_definition` as the bounded first-wave proof surface for MAP-required declarative-root control plus stage-root-aware catalog/loading behavior
   - **storage-layout candidate entrypoint set:** `handbook_pipeline::route_state::load_route_state`, `handbook_pipeline::route_state::set_route_state`, `handbook_pipeline::route_state::load_trusted_pipeline_session`, `handbook_pipeline::route_state::persist_route_basis`, `handbook_pipeline::pipeline_capture::preview_pipeline_capture`, `handbook_pipeline::pipeline_capture::capture_pipeline_output`, `handbook_pipeline::pipeline_capture::apply_pipeline_capture`, `handbook_pipeline::pipeline_capture::load_pipeline_capture_cache_entry`, `handbook_pipeline::pipeline_handoff::emit_pipeline_handoff_bundle`, and `handbook_pipeline::pipeline_handoff::validate_pipeline_handoff_bundle` as the bounded first-wave proof surface for MAP-required route-state, capture, and handoff storage-layout control
   - Set 2 must mark each retained candidate path as justified-for-capability or dropped-as-unnecessary once it fixes the intended consumer shape
   - no requirement to make the raw `layout` or `declarative_roots` modules themselves public if a smaller re-exported façade can carry the contract
4. **Only the typed results/errors required by those entrypoints**
   - keep capability-facing result types public where downstream consumers must handle them
   - keep implementation-only helper types private

### What must stay private in Set 2

Set 2 should keep the following private unless live proof later shows they are strictly required:

- `RepoLayoutRoot`
- `RuntimeStateLayoutContract`
- `CaptureStorageLayoutContract`
- `HandoffBundleLayoutContract`
- handbook-product default constants that only describe handbook's own repo shape
- repo/file/path plumbing helpers
- stage-source constants and other implementation-only loading details
- product-shell wording and CLI/product-only behavior

### Set 2 acceptance wall

Set 2 is only honest if all of the following are true:

1. **Implementation boundary wall**
   - `crates/pipeline/src/lib.rs` exposes the chosen façade intentionally
   - touched public surfaces stay limited to the contract owners plus this bounded candidate first-wave source-owner surface: `crates/pipeline/src/pipeline.rs::{SupportedTargetRegistry::load, load_pipeline_catalog, load_pipeline_catalog_metadata, load_pipeline_selection_metadata, load_pipeline_definition, load_selected_pipeline_definition}`, `crates/pipeline/src/route_state.rs::{load_route_state, set_route_state, load_trusted_pipeline_session, persist_route_basis}`, `crates/pipeline/src/pipeline_capture.rs::{preview_pipeline_capture, capture_pipeline_output, apply_pipeline_capture, load_pipeline_capture_cache_entry}`, and `crates/pipeline/src/pipeline_handoff.rs::{emit_pipeline_handoff_bundle, validate_pipeline_handoff_bundle}`
   - Set 2 must either justify each retained path against declarative-root control, stage-root-aware catalog/loading, route-state control, capture control, or handoff control for the intended external consumer shape, or shrink the public proof surface and say why
   - Set 2 may not widen beyond that candidate source-owner surface without reopening Set 1 authority
   - the change does not rely on making the entire private modules public just to reach a small number of types/functions
2. **External published-consumer wall**
   - a scratch consumer using published `handbook-pipeline` can construct non-default declarative-root and/or storage-layout contracts through the public boundary
   - that consumer proves enough of the bounded candidate public paths above to cover declarative-root control, stage-root-aware catalog/loading, route-state control, capture control, and handoff control through public APIs only; if fewer paths are sufficient, Set 2 must shrink the candidate surface explicitly rather than freezing unused extras as public contract
   - the proof/handoff must label every candidate path as retained-with-justification or dropped-as-unnecessary so the minimum reviewed surface is explicit
   - no proof step is allowed to import `handbook_pipeline::layout::*`, `handbook_pipeline::declarative_roots::*`, or other private-module paths
3. **Downstream revalidation input wall**
   - the Set 2 handoff must explicitly preserve that Packet 4.2 remains only an `engine + flow` proof
   - any future Substrate adoption of the Set 2 boundary must run in a dedicated Substrate worktree and must prove Substrate still owns wording and downstream runtime behavior
   - Set 2 does not get to claim downstream adoption merely because the new public boundary exists
4. **Guard-rail wall**
   - docs and tests must distinguish `public façade works` from `private internals remain reachable only inside handbook`
   - do not call Set 2 complete based only on internal tests, default-contract behavior, or sibling-path access
   - do not widen into CLI/compiler/product-shell redesign while chasing the boundary

Conventions for this set:

- prefer **side-by-side claim matrices** over broad prose
- separate **proven published seam** from **required future capability seam**
- separate **required capability** from **optional overexposure**
- label stale Substrate notes as **non-canonical** when cited
- do not rewrite archive history; instead create active superseding authority

## Testing Strategy

This set is docs-only, but it still has a proof wall. The verification strategy has five levels:

1. **Live source inspection**
   - confirm what `crates/pipeline/src/lib.rs` actually exports
   - confirm `PipelineDeclarativeRootsContract` and `PipelineStorageLayoutContract` remain non-public in the live/published seam

2. **External consumer proof**
   - positive compile for `handbook-engine + handbook-flow`
   - negative compile for `handbook-pipeline::layout::PipelineStorageLayoutContract`

3. **Downstream proof reclassification**
   - confirm Packet 4.2 proves only the narrow published seam it actually uses
   - explicitly reject any reading that treats Packet 4.2 as pipeline-adoption proof

4. **Authority reconciliation proof**
   - final Set 1 docs must name which prior claims remain valid, which are superseded, and what Sets 2 and 3 must do next

5. **MAP alignment proof**
   - final Set 1 docs must show that:
     - full reusable pipeline capability for Substrate is required
     - minimum unnecessary public surface is the governing intent
     - future reviews will use those exact standards

## Boundaries

- **Always:**
  - keep Set 1 docs-only
  - treat `/Users/spensermcconnell/__Active_Code/system/docs/specs/MAP.md` as the root authority for this workstream
  - ground every decision in live code, published-crate behavior, and exact source/doc claims
  - treat `/Users/spensermcconnell/__Active_Code/system/docs/specs/archive/` as provenance, not as the only active authority location
  - preserve the difference between:
    - published `engine + flow` proof
    - required future `pipeline` capability proof
  - preserve the MAP rule: expose capabilities, not guts
  - call out stale Substrate notes as non-canonical when used

- **Ask first:**
  - if Set 1 evidence suggests the required capability cannot be delivered without exposing broader internals than expected
  - if the cleanest reconciliation would require changing published crate names, ownership, or versioning policy
  - if the likely Set 2 shape would intentionally contradict the MAP intent of minimum unnecessary public surface

- **Never:**
  - edit Rust production code in this set
  - claim Packet 4.2 proved `handbook-pipeline` public-contract usability
  - treat archived Set 1 / Set 3 completion notes as stronger than live crates.io behavior
  - silently rely on stale `9b83` Substrate docs as if they were current authority
  - frame full reusable `handbook-pipeline` capability for Substrate as optional future nice-to-have
  - widen into CLI/compiler/product-shell redesign

## Success Criteria

1. A new active Set 1 authority exists under `/Users/spensermcconnell/__Active_Code/system/docs/specs/`.
2. That active authority explicitly aligns to `/Users/spensermcconnell/__Active_Code/system/docs/specs/MAP.md`.
3. The active authority explicitly states that Substrate requires the full reusable `handbook-pipeline` capability set through a reviewed published boundary.
4. The active authority explicitly states that Set 2 must deliver that capability with minimum unnecessary public surface.
5. The active authority explicitly resolves the contradiction between:
   - archived parameterization docs that imply a public/import-facing pipeline seam, and
   - published-consumption docs plus live crates.io behavior that still keep the key seams private.
6. Packet 4.2 is correctly reclassified as proof of feasible published `engine + flow` consumption, not proof of full pipeline-capability readiness.
7. Set 2 and Set 3 receive concrete acceptance walls:
   - Set 2 = capability boundary implementation
   - Set 3 = external consumer proof + downstream Substrate proof + guard rails
8. No Rust code or archived history is changed as part of Set 1 execution.

## Open Questions

1. Which candidate first-wave public entrypoints are truly required once the intended external consumer shape is fixed, and which should be dropped to shrink Set 2 further?
2. Which currently private typed seams are truly required for full Substrate capability, and which are merely one current implementation route that should stay private behind the façade?
3. Is the intended downstream consumer shape a direct Substrate call site, a reviewed provider/context boundary, or both?
4. Does the team want the archived parameterization docs to be formally superseded by the new active Set 1 triplet, or should Set 1 also produce a short explicit supersession note?
