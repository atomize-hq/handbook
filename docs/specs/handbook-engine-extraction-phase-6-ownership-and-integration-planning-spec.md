# Spec: Handbook Engine Extraction Phase 6 - Ownership And Integration Planning

## Status

- Packets 1, 2, and 3 are now landed in this triplet.
- Packet 1 froze the authority chain, root decision rule, verification-time baseline, and hard planning-only scope guard.
- Packet 2 lands the handbook-owned imported-core boundary decisions for `handbook-engine` and `handbook-pipeline` only.
- Packet 3 lands the handbook-side deferred-boundary and non-target decisions for `handbook-flow`, `handbook-cli`, and retained `handbook-compiler`.
- This family remains intentionally **docs-only** and **planning-only**.
- Packet 4 is **not started** here and remains out of scope.
- Packet 1 verification-time repo-truth freeze (pre-landing baseline, not the later landed HEAD):
  - branch: `feat/seam-extraction`
  - pre-landing baseline HEAD: `01b50868599bc55e7680784a9b5b2dace5ab6042`
  - working tree posture at verification time: dirty only in unrelated local files `AGENTS.md` and `CLAUDE.md`
- `aa882af42792a250cc02a6740bd1e2123178caff..01b50868599bc55e7680784a9b5b2dace5ab6042` is the Packet 1 verification-time **docs-only** baseline range. Later docs-only commits may advance HEAD without changing the prerequisite authority truth frozen there.

## Packet 1 Objective

Freeze the authority chain, root decision rule, verification-time branch/pre-landing-baseline truth, and hard scope guard for the new Phase 6 ownership family so later planning packets cannot silently drift back to stale pre-READY assumptions.

Packet 1 does **not** make per-crate ownership decisions. It only establishes the authoritative starting point for that later work.

## Packet 2 Objective

Decide the handbook-owned imported-core posture for `handbook-engine` and `handbook-pipeline` separately.

Packet 2 must:

- keep the engine and pipeline calls distinct
- state the architectural owner for each crate
- state Substrate's posture for each crate
- state the intended import boundary for each crate
- name any residual cleanup seam explicitly instead of hiding it inside the ownership call
- keep `handbook-flow`, `handbook-cli`, and retained `handbook-compiler` final posture work out of scope except for tiny consistency cross-references

## Packet 3 Objective

Decide the handbook-side deferred-boundary and non-target posture for `handbook-flow`, `handbook-cli`, and retained `handbook-compiler` separately.

Packet 3 must:

- keep `handbook-flow`, `handbook-cli`, and retained `handbook-compiler` distinct instead of bundling them into one generic “later” bucket
- state whether `handbook-flow` remains handbook-owned longer-term and what a future narrower import boundary would have to prove before later planning could bless it
- state why `handbook-cli` is handbook-owned product shell rather than any import target
- state why retained `handbook-compiler` is transition glue rather than a future ownership target
- name the support surfaces that remain deferred to later seams instead of pretending Packet 3 resolves them
- keep Packet 2’s `handbook-engine` / `handbook-pipeline` decisions intact

## Authority Chain Frozen By Packet 1

1. `HANDBOOK_ENGINE_EXTRACTION_PLAN.md` is the root authority.
2. `docs/specs/handbook-engine-extraction-slice-map.md` confirms Phases 1 through 5 are fully landed through Slice 5.3 and that Phase 6 is the next authoritative step.
3. `docs/specs/handbook-engine-extraction-closeout-four-set-map.md` records the honest Phase 1 through 5 closeout posture that Phase 6 inherits.
4. `docs/specs/handbook-engine-extraction-phase-6-slice-1-migration-readiness-reassessment-{spec,plan,tasks}.md` are the prerequisite READY gate for this family.
5. `docs/specs/handbook-engine-extraction-phase-6-slice-1-packet-6-1-2-ownership-matrix.md` remains useful background input, but it is **not** the final verdict because it predates the later boundary-fix and READY reassessment closeout.

## Root Ownership Decision Rule

Per `HANDBOOK_ENGINE_EXTRACTION_PLAN.md` Phase 6:

- if a crate is still fundamentally handbook-domain, **handbook should own it and Substrate should import it**
- only move a crate into Substrate if its real center of gravity becomes **substrate-specific**

Packets 2 and 3 apply that rule crate-by-crate for the imported-core and handbook-side deferred-boundary candidates in scope here.

## Packet 2 Scope Guard

In scope for Packet 2:

- `handbook-engine` ownership/import verdict
- `handbook-pipeline` ownership/import verdict
- exact repo-level boundary text for both crates
- the post-`aa882af` generic default layout contract truth for the engine call
- the bounded compiler-backed fixture/support coupling truth for the pipeline call
- explicit naming of any later bounded cleanup seam required by the pipeline evidence

Out of scope for Packet 2:

- `handbook-flow`, `handbook-cli`, and retained `handbook-compiler` final posture work beyond tiny cross-references strictly required for consistency
- any production code edits
- any crate publication or crates.io planning
- any Substrate integration implementation
- any claim that `handbook-pipeline` is fully decoupled while the live evidence still shows bounded compiler-backed fixture/support coupling
- packet-prompt authoring or approval

If any production symbol edit somehow becomes unavoidable, run GitNexus impact analysis first, report the blast radius, and stop unless the human explicitly approves widening.

## Packet 2 Repo-Truth Evidence

Packet 2 is grounded in the required command wall plus the authority set:

- `HANDBOOK_ENGINE_EXTRACTION_PLAN.md` Phase 6 still says the default rule is handbook-owned/imported unless a crate becomes truly substrate-specific.
- `crates/engine/src/lib.rs` exports a narrow crate root around canonical artifacts, authored-truth parsing/rendering/validation, baseline validation, freshness, `CanonicalLayoutContract`, and `default_canonical_layout_contract`.
- `crates/engine/src/canonical_paths.rs` now exposes the generic `default_canonical_layout_contract()` over `CanonicalLayoutContract`; the prior handbook-product-specific default-layout naming blocker is gone.
- `crates/pipeline/src/lib.rs` exports the catalog/runtime wedge directly: declarative loading, compile, capture, handoff, route resolution, route-state, and setup surfaces.
- `cargo tree -p handbook-pipeline` shows a runtime dependency on `handbook-engine`, while the remaining `handbook-compiler` edge is a **dev-dependency** rather than the runtime center of gravity.
- `crates/pipeline/tests/pipeline_catalog.rs` still imports `handbook_compiler::author::template_library::{resolve_shipped_template_library, TemplateLibraryRequest, TemplateLibrarySelection}`, which proves bounded compiler-backed fixture/support coupling still exists.
- `cargo test -p handbook-engine --test canonical_artifacts_ingest` passes, including the non-default layout-contract coverage.
- `cargo test -p handbook-pipeline --test pipeline_catalog` passes, including the catalog/supported-target wedge checks and the template-library-backed declarative-source assertions.

## Packet 2 Decisions

### `handbook-engine`

- **Architectural owner:** handbook remains the architectural owner.
- **Substrate posture:** Substrate should import `handbook-engine` through the current public crate surface; Packet 2 does **not** require a thinner adapter before making the ownership call.
- **Exact repo-level boundary text to record:** `handbook-engine` remains a handbook-owned imported core. Substrate may depend on the existing public `handbook-engine` surface as the reusable canonical-artifact, validation, freshness, and layout-contract boundary. The generic layout seam is the exported `CanonicalLayoutContract` / `default_canonical_layout_contract` pair plus the shared workspace contract version, not a handbook-product-specific default path contract.
- **Residual cleanup deferred out of this packet:** if Substrate later wants a narrower consumer-specific façade for ergonomics, that is a later downstream integration-seam choice, not an ownership blocker and not Packet 2 work.

### `handbook-pipeline`

- **Architectural owner:** handbook remains the architectural owner.
- **Substrate posture:** Substrate should treat `handbook-pipeline` as a handbook-owned external core and should import only through a thinner reviewed integration boundary, not by blessing the entire current crate re-export surface as the durable importer contract.
- **Exact repo-level boundary text to record:** `handbook-pipeline` remains handbook-owned. Substrate may integrate through the catalog-backed supported-target wedge only: the reviewed loading/selection, compile, capture, handoff, and route-state surfaces needed for supported pipeline execution. Do **not** record the full public re-export surface as fully decoupled or move ownership into Substrate while bounded compiler-backed fixture/support coupling still exists.
- **Residual cleanup deferred out of this packet:** the pipeline-specific deferred seam is compiler-backed fixture/support decoupling for the catalog/runtime wedge, starting with `crates/pipeline/tests/pipeline_catalog.rs` importing `handbook_compiler::author::template_library`. That cleanup should become its own later bounded seam instead of being hidden inside the ownership verdict.

## Contradictions Vs. Prerequisite Authority Set

- **No contradiction found** in the prerequisite authority set used for Packet 2.
- Packet 2 resolves the per-crate imported-core call for `handbook-engine` and `handbook-pipeline` without reopening the Packet 1 READY gate or the root ownership rule.
- Packet 2 also preserves the root-plan caution that `handbook-pipeline` must not be overstated as fully generic while bounded compiler-backed fixture/support coupling remains.

## Packet 3 Scope Guard

In scope for Packet 3:

- `handbook-flow` longer-term handbook-ownership posture
- the exact condition Packet 3 requires before any later planning could bless a narrower `handbook-flow` import slice
- the explicit `handbook-cli` product-shell boundary and non-target posture
- the explicit retained `handbook-compiler` transition-glue / non-target posture
- naming which support surfaces remain deferred to later seams rather than being resolved here

Out of scope for Packet 3:

- any change to Packet 2’s `handbook-engine` or `handbook-pipeline` decisions beyond tiny wording corrections required for consistency
- any production code edits
- any CLI redesign or product split
- any retained-compiler narrowing or retirement implementation
- any crate publication or crates.io planning
- any Substrate integration implementation or downstream execution-seam definition
- any claim that a future `handbook-flow` import slice is already proven

If Packet 3 ever requires code changes or repairing an earlier seam first, stop and route that blocker explicitly instead of widening.

## Packet 3 Repo-Truth Evidence

Packet 3 is grounded in the required command wall plus the already-frozen authority set:

- `crates/flow/src/lib.rs` exports only the `budget`, `packet_result`, and `resolver` modules plus their reviewed re-exports, and `flow_contract_version()` delegates to `handbook_engine::workspace_contract_version()`.
- `cargo tree -p handbook-flow` shows `handbook-flow` sitting directly on `handbook-engine`, without `handbook-cli` or retained `handbook-compiler` as runtime owners underneath it.
- `cargo test -p handbook-flow --test resolver_core` passes, so the resolver/budget/packet-result middle layer is live and stable as a handbook-owned seam today.
- `crates/cli/src/main.rs` remains the clap product entrypoint that owns command wiring, dynamic pipeline help, prompting, rendering, command dispatch, doctor/setup/operator wording, and exit-code handling.
- `cargo test -p handbook-cli --test help_drift_guard` passes, so the current CLI help/product-shell posture remains intentional repo truth rather than stale prose.
- `crates/compiler/src/lib.rs` still re-exports author/template-library, doctor, refusal, rendering, resolver, and setup adapters while depending on `handbook-engine`, `handbook-flow`, and `handbook-pipeline`.
- `cargo tree -p handbook-compiler` confirms retained `handbook-compiler` sits above the extracted owner crates instead of underneath them as the implementation center of gravity.
- `cargo check --workspace` passes, so Packet 3’s handbook-side boundary calls do not depend on any hidden code regression.

## Packet 3 Decisions

### `handbook-flow`

- **Architectural owner:** handbook remains the owner longer-term for the current live seam.
- **Current posture:** treat `handbook-flow` as a handbook-owned middle layer, not as a current move target and not as an already-proven importer boundary.
- **Exact repo-level boundary text to record:** `handbook-flow` remains handbook-owned longer-term as the resolver / packet-result / budget composition seam. Later planning may consider a narrower reviewed import slice only if it first proves that downstream callers can consume a stable subset of that seam without dragging CLI product-shell concerns or retained compiler support glue into the boundary.
- **What a future narrower import boundary would have to prove:** it would need to prove a reviewed importer contract around the actual `resolver`, `packet_result`, and `budget` family named in the root plan; prove that the contract is stable enough to freeze separately from later `rendering`, `refusal`, and error-surface decisions; and prove that importer ergonomics are improved without implicitly widening ownership into CLI or retained compiler glue. Packet 3 does **not** claim that proof exists yet.

### `handbook-cli`

- **Architectural owner:** handbook remains the owner.
- **Current posture:** `handbook-cli` is the handbook product shell, not any import target.
- **Exact repo-level boundary text to record:** `handbook-cli` stays handbook-owned because the live shell owns the command tree, dynamic help text, prompting flows, rendering/output formatting, doctor/setup/operator wording, and exit-code policy. Substrate should integrate around that shell or around lower crates, not treat the CLI crate itself as a reusable owner layer to import.
- **Explicit non-target rule:** Packet 3 does not authorize CLI redesign, product-shell splitting, or exporting CLI-owned shell helpers as a future ownership target.

### retained `handbook-compiler`

- **Architectural owner/posture:** retained `handbook-compiler` stays handbook-side as transition glue.
- **Current posture:** this crate is not a future ownership target and not the implementation center of gravity.
- **Exact repo-level boundary text to record:** retained `handbook-compiler` is transition glue that still exposes CLI-facing support adapters — author/template-library, setup, doctor, rendering, refusal, and resolver compatibility surfaces — on top of the extracted `handbook-engine`, `handbook-flow`, and `handbook-pipeline` owners. Later work may narrow or retire that glue, but Packet 3 does not treat the crate as a future durable owner boundary.
- **Explicit non-target rule:** do not frame retained `handbook-compiler` as the crate Substrate should import through, and do not treat compiler narrowing/retirement timing as resolved here.

## Support Surfaces Deferred To Later Seams

Packet 3 leaves all of the following explicit and deferred rather than pretending to resolve them:

- the exact ownership split for `rendering`, `refusal`, and `error` surfaces across CLI, `handbook-flow`, and retained `handbook-compiler`
- the later reviewed proof, if any, for a narrower `handbook-flow` importer contract
- the remaining CLI-facing support adapters that still span CLI and retained compiler glue, including `doctor`, `setup`, and template-library-backed authoring support
- the timing and scope of retained `handbook-compiler` narrowing or retirement
- the downstream Substrate execution seams and review gate that Packet 4 must define separately

## Contradictions Vs. Prerequisite Authority Set

- **No contradiction found** in the prerequisite authority set used for Packet 3.
- Packet 3 keeps Packet 2’s `handbook-engine` and `handbook-pipeline` decisions intact.
- Packet 3 also preserves the root-plan rule that callers should move directly to `handbook-engine`, `handbook-pipeline`, and `handbook-flow` instead of relying on a compiler facade, while still treating retained `handbook-compiler` as temporary handbook-side transition glue.

## Deferred Later Packets (Not Started Here)

- **Packet 4:** define downstream execution seams and the review gate

That packet is framed here only so the family boundary stays honest; it remains out of scope for this landing.

## Required Verification For Packet 2

```bash
rg -n "pub use|pub mod|mod " crates/engine/src/lib.rs crates/pipeline/src/lib.rs
rg -n "default_canonical_layout_contract|workspace_contract_version" crates/engine/src/lib.rs crates/engine/src/canonical_paths.rs
rg -n "PipelineCapture|PipelineHandoff|RouteState|template_library|stage_10_feature_spec" crates/pipeline/src crates/pipeline/tests
cargo tree -p handbook-engine
cargo tree -p handbook-pipeline
cargo test -p handbook-engine --test canonical_artifacts_ingest
cargo test -p handbook-pipeline --test pipeline_catalog
```

## Required Verification For Packet 3

```bash
rg -n "pub mod|pub use|resolve|PacketResult|BudgetOutcome" crates/flow/src/lib.rs crates/flow/src
cargo tree -p handbook-flow
cargo test -p handbook-flow --test resolver_core
rg -n "CommandFactory|ExitCode|pipeline_help|doctor|rendering|prompt" crates/cli/src/main.rs crates/cli/src
cargo test -p handbook-cli --test help_drift_guard
rg -n "rendering|refusal|doctor|setup|template_library|pub use" crates/compiler/src/lib.rs crates/compiler/src
cargo tree -p handbook-compiler
cargo check --workspace
```

## Success Criteria

- the triplet keeps Packet 1's authority freeze intact
- `handbook-engine` and `handbook-pipeline` receive separate ownership/import calls
- the engine boundary explicitly reflects the generic `default_canonical_layout_contract` truth
- the pipeline boundary explicitly records the bounded compiler-backed fixture/support coupling instead of ignoring it or overstating it as a blocker
- the repo-level boundary text for each imported-core crate is explicit enough to quote in later planning work
- the pipeline-specific deferred cleanup seam is named explicitly rather than hidden inside the ownership verdict
- `handbook-flow`, `handbook-cli`, and retained `handbook-compiler` each receive separate Packet 3 boundary text
- Packet 3 is explicit about which support surfaces remain deferred instead of hand-waving them into a generic “later”
- Packet 4 remains pending and out of scope
- the result is ready for orchestration review, but not execution-approved
