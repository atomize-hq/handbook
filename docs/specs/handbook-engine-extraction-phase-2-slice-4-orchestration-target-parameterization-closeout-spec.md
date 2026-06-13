# Spec: Handbook Engine Extraction Phase 2 Slice 4 (Slice 2.4) - Orchestration Target Parameterization Closeout

## Assumptions

1. Phase 2 Slices 2.1, 2.2, and 2.3 are materially landed, so this slice is a closeout/remediation seam for the remaining root-plan target-parameterization gap rather than a restart of Phase 2.
2. Live runtime ownership now sits primarily in `handbook-pipeline`, but the approved wedge is still materially centered on singleton Rust constants and product-surface literals for `pipeline.foundation_inputs`, `stage.10_feature_spec`, and `feature-slice-decomposer`.
3. Pipeline ids and stage ids must continue to come from declarative catalog truth under `core/pipelines/**` and `core/stages/**`; this slice should remove duplicated runtime ownership of those ids rather than introduce a second catalog model.
4. Consumer truth stays code-owned and validated for now. This slice may de-hardcode the bounded default consumer owner, but it must **not** introduce `core/consumers/**`, free-form consumer config, or a generic multi-consumer platform.
5. The currently supported runtime wedge remains bounded during this closeout unless the user explicitly expands scope; success here is de-hardcoding and honest ownership, not adding new supported targets.
6. Phase 1 layout/storage parameterization, Phase 4 caller rewires/compiler narrowing, and Phase 5 CLI shell closeout remain separate seams unless a tiny supporting adjustment is strictly required to eliminate target-literal ownership.

## Objective

Finish the remaining Phase 2 closeout gap so compile, capture, provenance, handoff, and adjacent CLI/help surfaces are driven by declared orchestration targets instead of a singleton hardcoded runtime posture.

The maintainer needs this slice because the repo has already adopted `SupportedTargetRegistry`, but the current supported wedge still leaks through direct owner constants and operator-facing literals. Success means:

- pipeline and stage truth come from declarative catalog inputs rather than duplicated Rust constants
- the bounded current consumer is owned in one code-owned validated default owner rather than scattered runtime literals
- compile/capture/provenance/handoff and their adjacent CLI/help surfaces derive the current supported wedge from shared target ownership
- the supported wedge stays the same while generalized multi-consumer/customizable-consumer work remains explicitly deferred

## Tech Stack

- Rust 2021 workspace
- `handbook-pipeline` library crate
- `handbook-cli` binary crate
- compatibility-observer surfaces in `handbook-compiler` only if live code still routes target ownership there
- declarative pipeline truth under `core/pipelines/**`
- declarative stage truth under `core/stages/**`
- existing Phase 2 authority docs:
  - `HANDBOOK_ENGINE_EXTRACTION_PLAN.md`
  - `docs/specs/handbook-engine-extraction-slice-map.md`
  - `docs/specs/handbook-engine-extraction-closeout-four-set-map.md`
  - `docs/specs/handbook-engine-extraction-phase-2-slice-1-supported-target-contract-*.md`
  - `docs/specs/handbook-engine-extraction-phase-2-slice-2-runtime-target-adoption-*.md`
  - `docs/specs/handbook-engine-extraction-phase-2-slice-3-template-and-library-resolver-*.md`

## Commands

Remaining-literal evidence sweep:

```bash
rg -n "SUPPORTED_PIPELINE_TARGET_ID|SUPPORTED_COMPILE_STAGE_TARGET_ID|SUPPORTED_CAPTURE_STAGE_TARGET_IDS|SUPPORTED_CONSUMER_TARGET_ID|SUPPORTED_CONSUMER_ID|SUPPORTED_STAGE_ID|pipeline\\.foundation_inputs|stage\\.10_feature_spec|feature-slice-decomposer" crates/pipeline/src crates/cli/src
```

Catalog-truth guard:

```bash
cargo test -p handbook-pipeline --test pipeline_catalog
```

Targeted closeout verification:

```bash
cargo test -p handbook-pipeline --test pipeline_compile
cargo test -p handbook-pipeline --test pipeline_capture
cargo test -p handbook-pipeline --test pipeline_handoff
cargo test -p handbook-cli --test cli_surface
```

Final verification wall:

```bash
cargo test --workspace
```

## Project Structure

```text
core/pipelines/**                                 -> Declarative pipeline catalog truth that should remain the source of pipeline identity and declared stage membership
core/stages/**                                    -> Declarative stage catalog truth that should remain the source of supported stage identity
crates/pipeline/src/pipeline.rs                   -> Current supported-target registry owner and the primary closeout seam for singleton hardcoding
crates/pipeline/src/pipeline_compile.rs           -> Compile-target lookup, refusal posture, and recovery wording
crates/pipeline/src/pipeline_capture.rs           -> Capture-target lookup, supported-stage rendering, and capture refusal posture
crates/pipeline/src/stage_10_feature_spec_provenance.rs -> Provenance target validation and stage-specific capture provenance rules
crates/pipeline/src/pipeline_handoff.rs           -> Handoff target validation, consumer ownership, and bounded bundle emission
crates/pipeline/tests/pipeline_catalog.rs         -> Declarative catalog regression coverage
crates/pipeline/tests/pipeline_compile.rs         -> Compile target regression coverage
crates/pipeline/tests/pipeline_capture.rs         -> Capture and provenance regression coverage
crates/pipeline/tests/pipeline_handoff.rs         -> Handoff target and consumer regression coverage
crates/cli/src/main.rs                            -> Public CLI help text and subcommand surface that still exposes target literals
crates/cli/src/pipeline.rs                        -> Public pipeline command adapters and hardcoded producer-command rendering
crates/cli/tests/cli_surface.rs                   -> CLI regression coverage for compile/capture/handoff operator surfaces
docs/specs/                                       -> Slice 2.4 authority documents
```

## Code Style

Prefer one catalog-backed target owner plus one bounded default-consumer owner over duplicated runtime constants or module-local command strings.

```rust
let registry = SupportedTargetRegistry::load(repo_root)?;
let handoff_target = registry.default_handoff_target()?;

let compile_target = registry.resolve_compile_target(
    &handoff_target.pipeline_id,
    &handoff_target.stage_id,
)?;

let producer_command = render_handoff_emit_command(&handoff_target);
```

Conventions:

- derive supported pipeline/stage identities from declarative catalog truth
- keep the bounded current consumer code-owned and validated
- let compile/capture/provenance/handoff consume shared typed targets instead of re-owning literals
- let CLI/help/recovery surfaces render from shared target data or thin product-shell adapters backed by that data
- keep refusal classifications and subsystem-specific operator wording local even when target ownership is centralized

## Testing Strategy

- Framework: existing Rust integration tests in `crates/pipeline/tests/` and CLI regression tests in `crates/cli/tests/`
- Primary test levels:
  - pipeline catalog regression tests for declarative target truth
  - compile/capture/provenance/handoff regression tests for supported-target behavior
  - CLI regression tests for public operator/help fallout
- Coverage focus:
  - the currently supported wedge remains unchanged after de-hardcoding
  - unsupported target refusals still render the right classifications and recoveries
  - handoff consumer validation continues to refuse anything outside the bounded default owner
  - CLI help and producer-command rendering stop being independent owners of target literals
- Coverage expectation:
  - each implementation packet updates targeted tests for the surface it de-hardcodes
  - final closeout proof includes both targeted tests and the workspace wall

## Slice Scope

In scope:

- remove singleton runtime ownership of the current supported pipeline/stage ids where declarative catalog truth should own them
- keep `SupportedTargetRegistry` (or a narrowly related target-owner module) as the shared runtime owner for catalog-backed pipeline/stage target resolution
- move the bounded current consumer into one code-owned validated default owner
- remove scattered `feature-slice-decomposer` ownership from handoff and CLI product-surface code
- derive supported-target help, recovery examples, and producer-command rendering from shared target ownership where those surfaces currently re-own literals
- add or refresh regression coverage proving current supported behavior remains unchanged

Out of scope:

- adding a new consumer, pipeline, or stage to the supported wedge
- introducing `core/consumers/**`, user-editable consumer config, or a broad multi-consumer platform
- reopening Slice 2.3 template/library resolver work
- reopening Phase 1 layout/storage parameterization
- broad caller/dependency rewires that belong to Set 3 / Slice 4.5 refresh
- broad CLI wording/product-shell redesign that belongs to Set 4 / Slice 5.3

## Authority Inputs

- `HANDBOOK_ENGINE_EXTRACTION_PLAN.md`
- `docs/specs/handbook-engine-extraction-slice-map.md`
- `docs/specs/handbook-engine-extraction-closeout-four-set-map.md`
- existing Phase 2 authority sets:
  - `docs/specs/handbook-engine-extraction-phase-2-slice-1-supported-target-contract-spec.md`
  - `docs/specs/handbook-engine-extraction-phase-2-slice-1-supported-target-contract-plan.md`
  - `docs/specs/handbook-engine-extraction-phase-2-slice-1-supported-target-contract-tasks.md`
  - `docs/specs/handbook-engine-extraction-phase-2-slice-2-runtime-target-adoption-spec.md`
  - `docs/specs/handbook-engine-extraction-phase-2-slice-2-runtime-target-adoption-plan.md`
  - `docs/specs/handbook-engine-extraction-phase-2-slice-2-runtime-target-adoption-tasks.md`
  - `docs/specs/handbook-engine-extraction-phase-2-slice-3-template-and-library-resolver-spec.md`
  - `docs/specs/handbook-engine-extraction-phase-2-slice-3-template-and-library-resolver-plan.md`
  - `docs/specs/handbook-engine-extraction-phase-2-slice-3-template-and-library-resolver-tasks.md`
- live declarative truth:
  - `core/pipelines/foundation_inputs.yaml`
  - `core/pipelines/default.yaml`
  - `core/stages/**`
- live runtime/CLI surfaces:
  - `crates/pipeline/src/pipeline.rs`
  - `crates/pipeline/src/pipeline_compile.rs`
  - `crates/pipeline/src/pipeline_capture.rs`
  - `crates/pipeline/src/stage_10_feature_spec_provenance.rs`
  - `crates/pipeline/src/pipeline_handoff.rs`
  - `crates/cli/src/main.rs`
  - `crates/cli/src/pipeline.rs`

## Current Repo-Truth Gaps This Closeout Must Finish

| Surface | Current live hardcoded posture | Slice 2.4 closeout requirement |
| --- | --- | --- |
| `crates/pipeline/src/pipeline.rs` | builds the supported registry around singleton constants for the supported pipeline, compile stage, capture stage set, and bounded consumer | keep one shared runtime owner, but derive pipeline/stage truth from declarative catalog inputs and keep the bounded consumer in one explicit validated default owner |
| `crates/pipeline/src/pipeline_compile.rs` | still exposes canonical compile id helpers and recovery posture tied to the singleton supported compile target | continue central runtime lookup, but stop treating singleton compile ids as independent owner constants |
| `crates/pipeline/src/pipeline_capture.rs` | still renders supported capture posture around direct stage-id expectations | derive supported capture posture from shared target ownership while keeping capture-specific wording local |
| `crates/pipeline/src/stage_10_feature_spec_provenance.rs` | still keeps a direct supported provenance pipeline/stage expectation | derive provenance target validation from shared target ownership while keeping provenance-specific validation local |
| `crates/pipeline/src/pipeline_handoff.rs` | still carries direct supported consumer/stage constants even though the registry is already live | move the bounded consumer and handoff target posture behind one shared validated owner |
| `crates/cli/src/main.rs` | public help text still spells out supported pipeline/stage/consumer ids directly | render or source bounded supported-target examples from shared target ownership without widening into Phase 5 shell redesign |
| `crates/cli/src/pipeline.rs` | hardcodes the producer command for handoff emit | derive producer-command rendering from the shared bounded target owner |

## Boundaries

- Always:
  - keep pipeline/stage truth declarative and catalog-backed
  - keep consumer truth code-owned and validated for now
  - de-hardcode the bounded current consumer
  - keep runtime behavior bounded to the currently supported wedge unless the user explicitly expands scope
  - distinguish central target ownership from CLI/product-shell rendering ownership
  - prove closeout with regression tests, not docs-only assertions
- Ask first:
  - any new consumer schema or `core/consumers/**` tree
  - any new supported consumer, pipeline, or stage
  - any broad CLI help/command redesign beyond sourcing existing examples from shared target ownership
  - any attempt to reopen template/library resolver work or caller rewires inside this slice
- Never:
  - widen into a generic multi-consumer framework
  - treat de-hardcoding `feature-slice-decomposer` as permission to add configurable consumer selection
  - reopen Phase 1 layout/storage parameterization as the main job of this slice
  - call Phase 2 complete while singleton hardcoded target ownership still remains in runtime or adjacent CLI surfaces

## Success Criteria

- The live target owner no longer depends on scattered singleton ownership of `pipeline.foundation_inputs`, `stage.10_feature_spec`, and `feature-slice-decomposer`.
- Pipeline and stage truth are resolved from declarative catalog inputs rather than duplicated Rust owner constants.
- The bounded current consumer is owned in one code-owned validated default owner.
- Compile, capture, provenance, handoff, and adjacent CLI/help surfaces consume shared target ownership for supported-target posture.
- The currently supported wedge remains unchanged under targeted regression coverage.
- Generic consumer-platform work, new consumer catalogs, and broad CLI shell cleanup remain explicitly deferred.

## Open Questions

- Should the bounded default-consumer owner live as a narrow sibling to `SupportedTargetRegistry` in `crates/pipeline/src/pipeline.rs`, or does live code truth justify a dedicated handoff-target owner module inside `handbook-pipeline`?
- Is the smallest durable CLI-side change to render command/help examples directly from shared target data, or should the CLI keep thin product-shell adapters that only format already-owned target values?
