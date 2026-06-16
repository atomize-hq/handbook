# Spec: Handbook Engine Extraction Phase 6 - `handbook-pipeline` Boundary Cleanup

## Assumptions

1. Packet 4 names five downstream seams, but it does **not** impose a mandatory execution order among the non-optional seams.
2. When ordering is not forced, the next seam should be the one with the most concrete live blocker and the least dependency on unresolved adjacent seams.
3. The optional `handbook-engine` adapter / boundary-freeze seam stays closed unless later human review explicitly says the current public engine surface is still insufficient.
4. Retained `handbook-compiler` narrowing is not the first seam because Packet 4 says that work waits until remaining support adapters have explicit homes and downstream callers no longer need compiler-routed transition helpers.
5. Absent a newer phase taxonomy in repo truth, this triplet should stay under the existing Phase 6 naming family instead of inventing a new Phase 7 label.

## Objective

Plan one downstream seam only: the `handbook-pipeline` boundary cleanup seam named by Packet 4 of the landed Phase 6 ownership/integration planning family.

This triplet must define, in docs only:

- the reviewed supported-target importer boundary that Packet 2 said is the only durable `handbook-pipeline` import posture
- the live evidence ledger and retained compiler context for the specific compiler-backed fixture/support coupling that still prevents the full current `handbook-pipeline` crate surface from being blessed as the durable importer contract
- the bounded cleanup target and explicit non-goals needed to remove or relocate that coupling without widening into CLI shell/support clarification, `handbook-flow` importer-proof work, retained `handbook-compiler` narrowing, or optional `handbook-engine` boundary-freeze work

This triplet is planning-only and stops at human review. Implementation, packet-prompt authoring for later execution, production edits, publication, crates.io work, Substrate consumption, and integration implementation remain blocked until a human separately reviews this triplet and explicitly approves a later execution packet.

## Why This Seam Is The Next Planning Target

Packet 2 already made `handbook-pipeline` the only imported-core crate whose durable importer posture stayed intentionally narrower than its current public re-export surface. Packet 4 then turned that residual gap into an explicit seam with concrete live evidence: define the reviewed supported-target importer boundary and remove or relocate the remaining compiler-backed fixture/support coupling currently evidenced by `crates/pipeline/tests/pipeline_catalog.rs` importing `handbook_compiler::author::template_library`.

Compared with the other Packet 4 seams:

- the `handbook-engine` seam is optional rather than required
- the `handbook-flow` seam is a proof-or-reject exercise, not a concrete already-evidenced coupling cleanup
- retained `handbook-compiler` narrowing is explicitly downstream of assigning support adapters to real homes
- the CLI shell/support seam is product-shell clarification work, not the importer-boundary cleanup that Packet 2 already identified as the concrete residual `handbook-pipeline` blocker

## Authority Inputs

Primary authority for why this seam exists:

- `docs/specs/handbook-engine-extraction-phase-6-ownership-and-integration-planning-spec.md`
- `docs/specs/handbook-engine-extraction-phase-6-ownership-and-integration-planning-plan.md`
- `docs/specs/handbook-engine-extraction-phase-6-ownership-and-integration-planning-tasks.md`

Supporting repo-truth authorities for seam sizing and earlier boundary language:

- `HANDBOOK_ENGINE_EXTRACTION_PLAN.md`
- `docs/specs/handbook-engine-extraction-slice-map.md`
- `docs/specs/handbook-engine-extraction-closeout-four-set-map.md`
- `docs/specs/handbook-engine-extraction-phase-2-slice-1-supported-target-contract-{spec,plan,tasks}.md`

Live code and test surfaces this seam must plan around:

- `crates/pipeline/src/lib.rs`
- `crates/pipeline/src/pipeline.rs`
- `crates/pipeline/tests/pipeline_catalog.rs`
- `crates/compiler/src/lib.rs`
- `crates/compiler/src/author/mod.rs`
- `crates/compiler/src/template_library.rs`

## Commands

Current live-evidence verification:

```bash
rg -n "pub use|pub mod|mod " crates/pipeline/src/lib.rs
rg -n "template_library|resolve_shipped_template_library|TemplateLibraryRequest|TemplateLibrarySelection|load_pipeline_catalog|load_pipeline_catalog_metadata|render_pipeline_list|supported_target" crates/pipeline/src crates/pipeline/tests/pipeline_catalog.rs
rg -n "template_library|pub use" crates/compiler/src/lib.rs crates/compiler/src/author/mod.rs crates/compiler/src/template_library.rs
cargo tree -p handbook-pipeline
cargo test -p handbook-pipeline --test pipeline_catalog
```

Future seam verification wall for implementation packets derived from this triplet:

```bash
cargo test -p handbook-pipeline --test pipeline_catalog
cargo test -p handbook-pipeline --test pipeline_compile
cargo test -p handbook-pipeline --test pipeline_capture
cargo test -p handbook-pipeline --test pipeline_handoff
cargo test -p handbook-compiler --test author
cargo check --workspace
```

Triplet-consistency verification:

```bash
rg -n "reviewed supported-target importer boundary|compiler-backed fixture/support coupling|template_library|setup stays out of the reviewed importer boundary|not execution approval|Substrate consumption|Human Review Gate" docs/specs/handbook-engine-extraction-phase-6-handbook-pipeline-boundary-cleanup-{spec,plan,tasks}.md
```

## Project Structure

```text
docs/specs/                                                         -> planning artifacts for this seam
crates/pipeline/src/lib.rs                                          -> current public `handbook-pipeline` re-export surface
crates/pipeline/src/pipeline.rs                                     -> catalog/loading/selection logic and supported-target topology
crates/pipeline/tests/pipeline_catalog.rs                           -> live evidence of compiler-backed fixture/support coupling
crates/compiler/src/lib.rs                                          -> current retained compiler re-export surface
crates/compiler/src/author/mod.rs                                   -> path-based re-export of template-library support into authoring flows
crates/compiler/src/template_library.rs                             -> current shipped-template and override-resolution support
```

## Planning Style

Keep this triplet quote-ready and seam-pure:

```md
- **Reviewed importer boundary:** loading/selection, compile, capture, handoff, and route-state surfaces needed for supported pipeline execution.
- **Out of scope:** CLI shell/support clarification, `handbook-flow` importer proof, retained `handbook-compiler` narrowing, optional `handbook-engine` boundary freeze, and any Substrate integration approval.
```

Conventions for this seam:

- reuse the exact crate names and Packet 2 / Packet 4 vocabulary already landed
- prefer explicit boundary bullets over broad prose
- name what stays out of the reviewed importer boundary just as clearly as what stays in
- keep one dominant seam: reviewed `handbook-pipeline` importer-boundary cleanup plus the pipeline-side compiler-backed fixture/support decoupling question

## Testing Strategy

This triplet does not change production code, so its immediate verification is evidence-based rather than behavior-changing.

- **Repo-truth evidence now:** confirm the current public `handbook-pipeline` surface, the remaining compiler dev-dependency, and the `pipeline_catalog` test import into compiler template-library support.
- **Implementation-proof wall later:** future packets for this seam must prove the catalog/runtime wedge still works, that compile/capture/handoff behavior stays green, and that compiler authoring support still works after the pipeline test/support dependency is cleaned up.
- **Boundary-proof expectation:** future implementation must prove that `pipeline_catalog` no longer depends on compiler-owned template-library support while preserving the Packet 2 rule that only the reviewed supported-target wedge, not the full crate surface, is the durable importer contract.

## Packet 3 Verification Wall

Future implementation for this seam must clear the full verification wall below before the seam can be considered implementation-complete:

- `cargo test -p handbook-pipeline --test pipeline_catalog`
- `cargo test -p handbook-pipeline --test pipeline_compile`
- `cargo test -p handbook-pipeline --test pipeline_capture`
- `cargo test -p handbook-pipeline --test pipeline_handoff`
- `cargo test -p handbook-compiler --test author`
- `cargo check --workspace`

This verification wall is reserved for a later execution packet. This triplet does not authorize running implementation, packet-prompt authoring for later execution, production edits, publication, crates.io work, Substrate consumption, or integration implementation.

### Adjacent evidence vs. seam-specific wall

- `pipeline_catalog` stays inside the mandatory Packet 3 wall because live repo truth still shows it importing `handbook_compiler::author::template_library::{resolve_shipped_template_library, TemplateLibraryRequest, TemplateLibrarySelection}`.
- `pipeline_loader` remains adjacent evidence for loading/selection surfaces, and `pipeline_route_resolution` plus `pipeline_state_store` remain adjacent evidence for route-state surfaces.
- Those adjacent tests do **not** currently import compiler template-library support, so they are evidence for the reviewed boundary shape rather than additional seam-specific proof that the compiler-backed coupling was removed.
- The mandatory Packet 3 wall therefore stays intentionally narrow: prove the known compiler-backed coupling seam (`pipeline_catalog`), preserve compile/capture/handoff proof for the reviewed supported-target wedge, preserve compiler authoring proof, and keep workspace compile health green.

## Seam Scope

In scope:

- restate the landed Packet 2 `handbook-pipeline` boundary text as the starting point for this seam
- define the reviewed supported-target importer boundary more concretely around the loading/selection, compile, capture, handoff, and route-state surfaces needed for supported pipeline execution
- explicitly record that `setup` remains outside the reviewed importer boundary for this seam because Packet 4 routes `setup` ownership clarification to the CLI shell/support seam
- record the live compiler-backed fixture/support coupling evidenced by `crates/pipeline/tests/pipeline_catalog.rs` importing compiler template-library support
- plan the bounded cleanup needed so pipeline catalog/runtime verification stops depending on compiler-owned template-library support
- preserve the Packet 4 routing that this seam owns the pipeline-side `template_library` / compiler-backed fixture-support decoupling question for the catalog/runtime wedge

Out of scope:

- blessing the entire current `handbook-pipeline` public re-export surface as the durable importer boundary
- changing `handbook-engine` ownership/import posture or opening the optional engine adapter seam
- proving a narrower `handbook-flow` importer contract
- clarifying CLI shell/support ownership for `rendering`, `refusal`, `error`, `doctor`, `setup`, prompting, operator wording, or exit-code policy
- narrowing or retiring retained `handbook-compiler` as a whole
- crate publication, crates.io work, Substrate consumption approval, or integration implementation
- packet-prompt authoring

## Packet 2 Freeze: Evidence Ledger And Cleanup Target

### Live evidence ledger

- `crates/pipeline/src/lib.rs` currently re-exports a broad surface that includes catalog/loading/selection helpers, compile/capture/handoff helpers, route-state helpers, and `setup`.
- Packet 2 already decided that Substrate must **not** consume that full current surface as the durable boundary; only a thinner reviewed supported-target wedge is approved.
- `crates/pipeline/tests/pipeline_catalog.rs` still imports `handbook_compiler::author::template_library::{resolve_shipped_template_library, TemplateLibraryRequest, TemplateLibrarySelection}`.
- `cargo tree -p handbook-pipeline` still shows `handbook-compiler` as a dev-dependency rather than a runtime owner, so the remaining compiler edge is bounded support coupling rather than runtime center-of-gravity inversion.
- `cargo test -p handbook-pipeline --test pipeline_catalog` passes today, which means the current coupling is live repo truth rather than dead code.

### Retained compiler context

- `crates/compiler/src/author/mod.rs` still owns the path-based authoring hook via `#[path = "../template_library.rs"] pub mod template_library;`.
- `crates/compiler/src/lib.rs` still re-exports `author::template_library::{resolve_shipped_template_library, resolve_template_library, ...}`, which is why pipeline tests can currently reach shipped-template support through retained compiler-owned surfaces.
- `crates/compiler/src/template_library.rs` remains the live implementation home for `resolve_shipped_template_library`, `TemplateLibraryRequest`, and `TemplateLibrarySelection`.

### Bounded cleanup target

- Later implementation must remove or relocate the pipeline-side compiler-backed fixture/support dependency needed by pipeline catalog/runtime proof.
- That target stays intentionally narrow: the goal is to decouple pipeline catalog/runtime proof from compiler-owned template-library support without changing the Packet 2 rule that only the reviewed supported-target wedge, not the full crate surface, is the durable importer contract.

### Explicit non-goals

- Do not widen this seam into retained `handbook-compiler` retirement.
- Do not widen this seam into broader authoring-stack relocation.
- Do not widen this seam into CLI shell/support reassignment.
- Do not treat this seam as approval to reopen the optional `handbook-engine` boundary-freeze seam or the separate `handbook-flow` importer-proof seam.

## Boundary Freeze This Seam Must Produce

### Packet 1 authority freeze inherited from Phase 6 Packet 2 and Packet 4

- `handbook-pipeline` remains handbook-owned.
- Only the reviewed supported-target importer boundary is the durable importer posture for this seam; the full current public re-export surface is **not** the durable importer contract.
- This seam owns the pipeline-side `template_library` / compiler-backed fixture-support decoupling question for the catalog/runtime wedge.

### Reviewed supported-target importer boundary

This seam must make the reviewed supported-target importer boundary concrete enough that later implementation packets can test against it without rediscovering scope.

The planned durable boundary for `handbook-pipeline` should stay limited to the Packet 2 supported-target wedge:

- loading/selection surfaces
- compile surfaces
- capture surfaces
- handoff surfaces
- route-state surfaces required for supported pipeline execution

This seam must **not** silently expand that reviewed supported-target importer boundary to include every current public re-export. In particular:

- `setup` stays outside this reviewed supported-target importer boundary for now
- CLI/product-shell helpers stay outside this boundary
- compiler-routed compatibility helpers stay outside this boundary unless they are the minimum temporary bridge required to remove the specific `pipeline_catalog` coupling this seam owns

### Bounded cleanup target

This seam must plan the removal or relocation of the remaining compiler-backed fixture/support coupling that currently exists because `pipeline_catalog` reaches into compiler template-library support.

Later implementation must remove or relocate the pipeline-side compiler-backed fixture/support dependency needed by pipeline catalog/runtime proof.

The cleanup target is intentionally narrower than “move template_library out of compiler everywhere.” The seam succeeds only if later packets can make `handbook-pipeline` catalog/runtime proof independent from compiler-owned template-library support **without** widening into retained `handbook-compiler` retirement, broader authoring-stack relocation, or CLI shell/support reassignment.

Acceptable future implementation postures may include a pipeline-owned or compiler-neutral source for the specific shipped template defaults that `pipeline_catalog` needs, but this triplet does not authorize or require a full authoring-stack relocation.

## Boundaries

- Always:
  - preserve the landed Packet 2 statement that `handbook-pipeline` remains handbook-owned and only the reviewed supported-target wedge is the durable importer posture
  - preserve the landed Packet 4 routing that this seam owns the pipeline-side `template_library` / compiler-backed fixture-support decoupling question
  - keep `setup` outside the reviewed importer boundary for this seam
  - keep this triplet docs-only and planning-only
  - keep one dominant seam and make adjacent seams explicit non-goals
- Ask first:
  - any attempt to widen the reviewed importer boundary beyond loading/selection, compile, capture, handoff, and route-state surfaces
  - any attempt to move CLI shell/support ownership questions into this seam
  - any attempt to treat retained `handbook-compiler` narrowing as part of this seam rather than a later follow-on
  - any attempt to convert this planning triplet into Substrate consumption approval
- Never:
  - bless the full current `handbook-pipeline` crate surface as the durable importer contract
  - reopen the Phase 6 ownership verdicts for `handbook-engine`, `handbook-flow`, `handbook-cli`, or retained `handbook-compiler`
  - start implementation from this triplet alone
  - authorize packet-prompt authoring for later execution, production edits, publication, crates.io work, Substrate consumption, or integration implementation

## Success Criteria

- The triplet explicitly states why `handbook-pipeline` is the next downstream seam to plan.
- The triplet preserves the landed Packet 2 and Packet 4 boundary language instead of replacing it with new generic wording.
- The triplet freezes a concrete reviewed importer boundary for `handbook-pipeline` that is narrower than the full current public re-export surface.
- The triplet explicitly states that `setup` stays out of this reviewed importer boundary and remains routed to the CLI shell/support seam.
- The triplet distinguishes a live evidence ledger, retained compiler context, bounded cleanup target, and explicit non-goals for Packet 2.
- The triplet records the live compiler-backed fixture/support coupling in `crates/pipeline/tests/pipeline_catalog.rs` and the retained compiler context that currently exposes it.
- The triplet plans a bounded cleanup path that removes or relocates that coupling without widening into retained `handbook-compiler` retirement, broader authoring-stack relocation, or CLI shell/support reassignment.
- The triplet includes a future implementation verification wall and an explicit human review gate.

## Human Review Gate

This triplet stops at review. Implementation, packet-prompt authoring for later execution, production edits, publication, crates.io work, Substrate consumption, and integration implementation remain blocked until a human separately reviews this triplet and explicitly approves a later execution packet.

## Open Questions

- Which minimal replacement source should later implementation use for the shipped template defaults needed by `pipeline_catalog`: a pipeline-owned test/support fixture, or a compiler-neutral shared helper that does not drag author-shell ownership into the pipeline boundary?
- Does any current `handbook-pipeline` export outside the reviewed supported-target wedge still need temporary transitional documentation, or can this triplet treat the non-reviewed remainder simply as out of boundary?
- Current live repo truth says `pipeline_loader`, `pipeline_route_resolution`, and `pipeline_state_store` cover adjacent loading/selection and route-state behavior without importing compiler template-library support; if a later repo change introduces another compiler-backed pipeline test beyond `pipeline_catalog`, add it to the bounded evidence ledger before implementation starts.
