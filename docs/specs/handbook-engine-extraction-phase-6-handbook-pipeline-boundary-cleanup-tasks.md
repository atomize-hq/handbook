# Tasks: Handbook Engine Extraction Phase 6 - `handbook-pipeline` Boundary Cleanup

Plan reference: [handbook-engine-extraction-phase-6-handbook-pipeline-boundary-cleanup-plan.md](./handbook-engine-extraction-phase-6-handbook-pipeline-boundary-cleanup-plan.md)

## Prerequisites Already Landed

- Phase 6 ownership/integration planning Packet 2 already states that `handbook-pipeline` remains handbook-owned and that Substrate should import only through a thinner reviewed supported-target boundary rather than the full current public re-export surface.
- Phase 6 ownership/integration planning Packet 4 already names this seam as: the `handbook-pipeline` boundary cleanup seam, owned by the reviewed importer-boundary question plus the pipeline-side `template_library` / compiler-backed fixture-support decoupling question for the catalog/runtime wedge.
- The Phase 6 human review gate remains in force: this new triplet does not itself approve implementation.

## Packet 1: Freeze The Seam Boundary

- [x] Task: Re-state the landed Packet 2 and Packet 4 boundary language as the authority for this seam
  - Acceptance: The spec, plan, and tasks docs all state that `handbook-pipeline` remains handbook-owned, that only a thinner reviewed supported-target importer boundary is the durable posture, and that this seam owns the pipeline-side `template_library` / compiler-backed fixture-support decoupling question.
  - Verify: `rg -n "handbook-owned|reviewed supported-target importer boundary|template_library|compiler-backed fixture/support coupling" docs/specs/handbook-engine-extraction-phase-6-handbook-pipeline-boundary-cleanup-{spec,plan,tasks}.md`
  - Files: `docs/specs/handbook-engine-extraction-phase-6-handbook-pipeline-boundary-cleanup-spec.md`, `docs/specs/handbook-engine-extraction-phase-6-handbook-pipeline-boundary-cleanup-plan.md`, `docs/specs/handbook-engine-extraction-phase-6-handbook-pipeline-boundary-cleanup-tasks.md`
  - Completion note: The triplet now re-states the landed Packet 2 / Packet 4 authority in quote-ready form: `handbook-pipeline` remains handbook-owned; only the reviewed supported-target importer boundary is durable; and this seam owns the pipeline-side `template_library` / compiler-backed fixture-support decoupling question for the catalog/runtime wedge.

- [x] Task: Freeze what is inside and outside the reviewed importer boundary
  - Acceptance: The triplet explicitly records that loading/selection surfaces, compile surfaces, capture surfaces, handoff surfaces, and route-state surfaces needed for supported pipeline execution are the reviewed importer boundary, while `setup`, CLI/product-shell helpers, and compiler-routed compatibility helpers remain outside this seam.
  - Verify: `rg -n "loading/selection|compile surfaces|capture surfaces|handoff surfaces|route-state surfaces|setup stays outside|CLI/product-shell helpers|compiler-routed compatibility helpers" docs/specs/handbook-engine-extraction-phase-6-handbook-pipeline-boundary-cleanup-{spec,plan,tasks}.md`
  - Files: `docs/specs/handbook-engine-extraction-phase-6-handbook-pipeline-boundary-cleanup-spec.md`, `docs/specs/handbook-engine-extraction-phase-6-handbook-pipeline-boundary-cleanup-plan.md`, `docs/specs/handbook-engine-extraction-phase-6-handbook-pipeline-boundary-cleanup-tasks.md`
  - Completion note: The triplet now freezes the reviewed supported-target importer boundary as loading/selection surfaces, compile surfaces, capture surfaces, handoff surfaces, and route-state surfaces needed for supported pipeline execution. It also states that `setup` stays outside this reviewed supported-target importer boundary, that CLI/product-shell helpers stay outside this boundary, and that compiler-routed compatibility helpers stay outside this boundary.

## Packet 2: Freeze The Coupling Evidence Ledger And Cleanup Target

- [x] Task: Record the live compiler-backed fixture/support coupling evidence
  - Acceptance: The triplet explicitly labels a **live evidence ledger** and a **retained compiler context**. It records that `crates/pipeline/tests/pipeline_catalog.rs` imports `handbook_compiler::author::template_library::{resolve_shipped_template_library, TemplateLibraryRequest, TemplateLibrarySelection}`, and that `cargo tree -p handbook-pipeline` still shows `handbook-compiler` only as a dev-dependency rather than a runtime owner.
  - Verify: `rg -n "resolve_shipped_template_library|TemplateLibraryRequest|TemplateLibrarySelection" crates/pipeline/tests/pipeline_catalog.rs && cargo tree -p handbook-pipeline`
  - Files: `docs/specs/handbook-engine-extraction-phase-6-handbook-pipeline-boundary-cleanup-spec.md`, `docs/specs/handbook-engine-extraction-phase-6-handbook-pipeline-boundary-cleanup-plan.md`, `docs/specs/handbook-engine-extraction-phase-6-handbook-pipeline-boundary-cleanup-tasks.md`
  - Completion note: Packet 2 now freezes the live evidence ledger around the exact `pipeline_catalog` import and the dev-dependency-only `cargo tree -p handbook-pipeline` result, then separates the retained compiler context into the `crates/compiler/src/lib.rs`, `crates/compiler/src/author/mod.rs`, and `crates/compiler/src/template_library.rs` surfaces that currently expose that support.

- [x] Task: Freeze the bounded cleanup target without widening into retained compiler narrowing
  - Acceptance: The triplet explicitly labels a **bounded cleanup target** and **explicit non-goals**. It states that later implementation must remove or relocate the pipeline-side compiler-backed fixture/support dependency needed by pipeline catalog/runtime proof, but does not plan wholesale retained `handbook-compiler` retirement, broader authoring-stack relocation, or CLI shell/support reassignment.
  - Verify: `rg -n "remove or relocate|pipeline catalog/runtime proof|retained handbook-compiler retirement|authoring-stack relocation|CLI shell/support" docs/specs/handbook-engine-extraction-phase-6-handbook-pipeline-boundary-cleanup-{spec,plan,tasks}.md`
  - Files: `docs/specs/handbook-engine-extraction-phase-6-handbook-pipeline-boundary-cleanup-spec.md`, `docs/specs/handbook-engine-extraction-phase-6-handbook-pipeline-boundary-cleanup-plan.md`, `docs/specs/handbook-engine-extraction-phase-6-handbook-pipeline-boundary-cleanup-tasks.md`
  - Completion note: Packet 2 now freezes the cleanup target as “Later implementation must remove or relocate the pipeline-side compiler-backed fixture/support dependency needed by pipeline catalog/runtime proof,” and pairs that sentence with an explicit non-goals list that keeps retained `handbook-compiler` retirement, broader authoring-stack relocation, and CLI shell/support reassignment out of scope.

## Packet 3: Prepare Future Implementation Verification And Stop At Review

- [x] Task: Record the future implementation verification wall for this seam
  - Acceptance: The triplet records `pipeline_catalog`, `pipeline_compile`, `pipeline_capture`, `pipeline_handoff`, compiler `author`, and `cargo check --workspace` as the future implementation verification wall for this seam.
  - Verify: `rg -n "pipeline_catalog|pipeline_compile|pipeline_capture|pipeline_handoff|handbook-compiler --test author|cargo check --workspace" docs/specs/handbook-engine-extraction-phase-6-handbook-pipeline-boundary-cleanup-{spec,plan,tasks}.md`
  - Files: `docs/specs/handbook-engine-extraction-phase-6-handbook-pipeline-boundary-cleanup-spec.md`, `docs/specs/handbook-engine-extraction-phase-6-handbook-pipeline-boundary-cleanup-plan.md`, `docs/specs/handbook-engine-extraction-phase-6-handbook-pipeline-boundary-cleanup-tasks.md`
  - Completion note: Packet 3 now records the full future implementation verification wall in quote-ready form across the triplet: `pipeline_catalog`, `pipeline_compile`, `pipeline_capture`, `pipeline_handoff`, compiler `author`, and `cargo check --workspace` all remain required proof for a later execution packet.
  - Scope note: `pipeline_loader`, `pipeline_route_resolution`, and `pipeline_state_store` remain adjacent evidence for loading/selection and route-state coverage rather than new mandatory wall entries because live repo truth does not show them importing compiler template-library support; `pipeline_catalog` remains the known compiler-backed coupling seam this wall must prove.

- [x] Task: Preserve the human review gate and planning-only stop condition
  - Acceptance: The triplet states plainly that it is planning-only; implementation, packet-prompt authoring for later execution, production edits, publication, crates.io work, Substrate consumption, and integration implementation remain blocked until a human separately reviews this triplet and explicitly approves a later execution packet.
  - Verify: `rg -n "planning-only|packet-prompt authoring|production edits|publication|crates\.io|Substrate consumption|integration implementation|Human Review Gate" docs/specs/handbook-engine-extraction-phase-6-handbook-pipeline-boundary-cleanup-{spec,plan,tasks}.md`
  - Files: `docs/specs/handbook-engine-extraction-phase-6-handbook-pipeline-boundary-cleanup-spec.md`, `docs/specs/handbook-engine-extraction-phase-6-handbook-pipeline-boundary-cleanup-plan.md`, `docs/specs/handbook-engine-extraction-phase-6-handbook-pipeline-boundary-cleanup-tasks.md`
  - Completion note: Packet 3 now ends at an explicit **Human Review Gate** in all three docs. The stop condition is quote-ready and says that implementation, packet-prompt authoring for later execution, production edits, publication, crates.io work, Substrate consumption, and integration implementation remain blocked until a human separately reviews this triplet and explicitly approves a later execution packet.

## Human Review Gate

This triplet is planning-only and stops at review. Implementation, packet-prompt authoring for later execution, production edits, publication, crates.io work, Substrate consumption, and integration implementation remain blocked until a human separately reviews this triplet and explicitly approves a later execution packet.
