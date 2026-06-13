# Tasks: Handbook Engine Extraction Phase 5 Slice 3 (Slice 5.3) - CLI Shell Closeout

Plan reference: [handbook-engine-extraction-phase-5-slice-3-cli-shell-closeout-plan.md](./handbook-engine-extraction-phase-5-slice-3-cli-shell-closeout-plan.md)

## Prerequisite: keep the closeout bounded to CLI product-shell ownership

Before implementation, re-read:

- `HANDBOOK_ENGINE_EXTRACTION_PLAN.md`
- `docs/specs/handbook-engine-extraction-slice-map.md`
- `docs/specs/handbook-engine-extraction-closeout-four-set-map.md`
- `docs/specs/handbook-engine-extraction-phase-5-slice-1-cli-skeleton-and-author-setup-*.md`
- `docs/specs/handbook-engine-extraction-phase-5-slice-2-cli-runtime-command-shell-*.md`

This slice is the final Phase 5 closeout seam. It should finish CLI product-shell ownership without reopening layout/storage parameterization, orchestration-target parameterization, or compiler narrowing.

## Packet 5.3.1: Prompting, Rendering, And Help Helper Extraction

- [ ] Task: Isolate guided-prompt helper ownership from the oversized author shell module
  - Acceptance: guided prompt context, prompt primitives, and closely related author-shell refusal/prompt helpers no longer live inline as one undifferentiated block inside `crates/cli/src/author.rs`; the resulting placement makes CLI prompt ownership explicit without changing guided interview behavior.
  - Verify: `cargo test -p handbook-cli --test author_cli && cargo test -p handbook-cli --test cli_surface`
  - Files: `crates/cli/src/author.rs`, any new CLI prompt/helper modules under `crates/cli/src/`, and only narrowly-related shared shell files if live reuse requires them

- [ ] Task: Move CLI help summaries/examples out of reusable crates where they currently act as clap shell copy owners
  - Acceptance: clap-facing help summaries/examples and similar command-shell copy now live in `handbook-cli` rather than in reusable crates such as `crates/pipeline/src/pipeline.rs`, while public help output stays behaviorally unchanged.
  - Verify: `cargo test -p handbook-cli --test help_drift_guard && cargo test -p handbook-cli --test pipeline_handoff_refusals`
  - Files: `crates/cli/src/main.rs`, `crates/cli/src/pipeline.rs`, any new CLI help-text module(s), and the specific reusable help-export surfaces that stop owning final shell copy

- [ ] Task: Establish CLI-local last-mile presentation adapters for generate/inspect and adjacent shell output
  - Acceptance: final generate/inspect shell presentation is clearly owned in CLI-local rendering adapters rather than delegated wholesale to reusable-crate final copy renderers; fixture-demo injection and operator-visible output remain stable.
  - Verify: `cargo test -p handbook-cli --test cli_surface && cargo test -p handbook-cli --test manual_qa_fixture_checkout && cargo test -p handbook-cli --test feature_spec_contract`
  - Files: `crates/cli/src/rendering.rs`, `crates/cli/src/generate.rs`, `crates/cli/src/inspect.rs`, and only the specific reusable rendering surfaces whose ownership is being narrowed

## Packet 5.3.2: Exit-Code And Final Shell Closeout

- [ ] Task: Centralize or clearly justify the remaining CLI exit-code policy boundaries
  - Acceptance: success/failure decisions for the remaining shell flows are clearly owned by CLI modules or shared CLI presentation helpers; the implementation no longer leaves exit-code policy implicitly split across reusable crates and ad hoc command-family code paths.
  - Verify: `cargo test -p handbook-cli --test cli_surface && cargo test -p handbook-cli --test manual_qa_fixture_checkout && cargo test -p handbook-cli --test feature_spec_contract`
  - Files: `crates/cli/src/generate.rs`, `crates/cli/src/inspect.rs`, `crates/cli/src/doctor.rs`, `crates/cli/src/setup.rs`, `crates/cli/src/rendering.rs`, `crates/cli/src/doctor_rendering.rs`, and any narrowly-scoped CLI support modules needed to make exit ownership explicit

- [ ] Task: Remove or explicitly justify remaining reusable-crate final shell copy exports
  - Acceptance: any reusable-crate exports whose only purpose was final operator-facing help/render/command copy are either removed, replaced by typed support surfaces, or explicitly documented as temporary compatibility glue with clear ownership rationale.
  - Verify: `rg -n "SUPPORTED_.*HELP|render_(markdown|inspect|json)|render_supported_handoff_emit_command" crates/cli/src crates/pipeline/src crates/compiler/src && cargo test -p handbook-cli --test help_drift_guard`
  - Files: the specific CLI and reusable render/help surfaces still participating in final shell-copy ownership

- [ ] Task: Re-run the full closeout wall and confirm Phase 5 is honestly at the CLI-shell steady state
  - Acceptance: fmt/clippy/workspace tests pass, targeted CLI tests pass, `main.rs` remains thin and honest, and the resulting implementation leaves Set 4 clearly closed without widening into the neighboring closeout sets.
  - Verify: `cargo fmt --all -- --check && cargo clippy --workspace --all-targets -- -D warnings && cargo test --workspace`
  - Files: verification only

## Human Review Gate

Stop after the Slice 5.3 authority set is reviewed. Do not start implementation packets until the human approves this spec/plan/tasks set.
