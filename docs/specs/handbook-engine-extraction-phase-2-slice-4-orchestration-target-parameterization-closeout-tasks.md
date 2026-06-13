# Tasks: Handbook Engine Extraction Phase 2 Slice 4 (Slice 2.4) - Orchestration Target Parameterization Closeout

Plan reference: [handbook-engine-extraction-phase-2-slice-4-orchestration-target-parameterization-closeout-plan.md](./handbook-engine-extraction-phase-2-slice-4-orchestration-target-parameterization-closeout-plan.md)

## Prerequisite Authority

Before implementation, re-read:

- `HANDBOOK_ENGINE_EXTRACTION_PLAN.md`
- `docs/specs/handbook-engine-extraction-slice-map.md`
- `docs/specs/handbook-engine-extraction-closeout-four-set-map.md`
- the landed Phase 2 Slice 2.1, 2.2, and 2.3 triplets

This slice is a closeout/remediation seam. It should finish the remaining target-parameterization gap without reopening template/library work, caller rewires, or CLI shell redesign.

## Packet 2.4.1: Catalog-Backed Pipeline And Stage Target Closeout

- [ ] Task: Remove singleton pipeline/stage ownership from the runtime target owner
  - Acceptance: The shared runtime target owner no longer depends on singleton Rust owner constants as the de facto source of supported pipeline/stage truth; instead it consumes declarative catalog truth while preserving the current supported wedge.
  - Verify: `cargo test -p handbook-pipeline --test pipeline_catalog && cargo test -p handbook-pipeline --test pipeline_compile && cargo test -p handbook-pipeline --test pipeline_capture`
  - Files: `crates/pipeline/src/pipeline.rs`, `crates/pipeline/src/pipeline_compile.rs`, `crates/pipeline/src/pipeline_capture.rs`, `crates/pipeline/src/stage_10_feature_spec_provenance.rs`, `crates/pipeline/tests/pipeline_catalog.rs`

- [ ] Task: Keep compile/capture/provenance adopter behavior stable while consuming the shared target owner
  - Acceptance: Compile, capture, and provenance still enforce the same bounded wedge and refusal posture, but they no longer act as parallel owners of supported pipeline/stage ids.
  - Verify: `cargo test -p handbook-pipeline --test pipeline_compile && cargo test -p handbook-pipeline --test pipeline_capture`
  - Files: `crates/pipeline/src/pipeline_compile.rs`, `crates/pipeline/src/pipeline_capture.rs`, `crates/pipeline/src/stage_10_feature_spec_provenance.rs`, `crates/pipeline/tests/pipeline_compile.rs`, `crates/pipeline/tests/pipeline_capture.rs`

## Packet 2.4.2: Bounded Default-Consumer Ownership

- [ ] Task: Centralize the bounded current consumer in one code-owned validated default owner
  - Acceptance: `feature-slice-decomposer` is owned in one shared validated location; `pipeline_handoff.rs` no longer carries separate consumer ownership literals for the current supported posture.
  - Verify: `cargo test -p handbook-pipeline --test pipeline_handoff`
  - Files: `crates/pipeline/src/pipeline.rs`, `crates/pipeline/src/pipeline_handoff.rs`, `crates/pipeline/tests/pipeline_handoff.rs`

- [ ] Task: Keep handoff validation and bundle emission behavior stable after consumer de-hardcoding
  - Acceptance: Handoff emit/validate behavior stays bounded to the current supported wedge, preserves provenance/trust/refusal behavior, and does not add a generic consumer selection platform.
  - Verify: `cargo test -p handbook-pipeline --test pipeline_handoff && cargo test -p handbook-cli --test cli_surface`
  - Files: `crates/pipeline/src/pipeline_handoff.rs`, `crates/cli/src/pipeline.rs`, `crates/cli/tests/cli_surface.rs`

## Packet 2.4.3: CLI Help, Recovery, And Producer-Command Alignment

- [ ] Task: Source supported-target examples from shared bounded target ownership
  - Acceptance: CLI help, recovery examples, and producer-command rendering stop acting as independent owners of `pipeline.foundation_inputs`, `stage.10_feature_spec`, and `feature-slice-decomposer`; any remaining product-shell wording is backed by shared target ownership rather than fresh literals.
  - Verify: `cargo test -p handbook-cli --test cli_surface && cargo test -p handbook-pipeline --test pipeline_capture && cargo test -p handbook-pipeline --test pipeline_handoff`
  - Files: `crates/cli/src/main.rs`, `crates/cli/src/pipeline.rs`, `crates/pipeline/src/pipeline_capture.rs`, `crates/pipeline/src/pipeline_handoff.rs`, `crates/cli/tests/cli_surface.rs`

## Packet 2.4.4: Final Closeout Proof

- [ ] Task: Re-run the full closeout verification wall and preserve explicit deferrals
  - Acceptance: The targeted catalog/compile/capture/handoff/CLI tests and full workspace wall pass, and the resulting implementation notes or follow-up handoff make explicit that multi-consumer platforms, new consumer catalogs, and Phase 5 CLI shell cleanup remain deferred.
  - Verify: `cargo test -p handbook-pipeline --test pipeline_catalog && cargo test -p handbook-pipeline --test pipeline_compile && cargo test -p handbook-pipeline --test pipeline_capture && cargo test -p handbook-pipeline --test pipeline_handoff && cargo test -p handbook-cli --test cli_surface && cargo test --workspace`
  - Files: `crates/pipeline/tests/pipeline_catalog.rs`, `crates/pipeline/tests/pipeline_compile.rs`, `crates/pipeline/tests/pipeline_capture.rs`, `crates/pipeline/tests/pipeline_handoff.rs`, `crates/cli/tests/cli_surface.rs`

## Human Review Gate

Stop after the Slice 2.4 authority set is reviewed. Do not widen into Set 3 / Slice 4.5 refresh or Set 4 / Slice 5.3 CLI shell closeout until the human explicitly approves the next seam.
