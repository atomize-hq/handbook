# Spec: Handbook Engine Extraction Phase 5 Slice 3 (Slice 5.3) - CLI Shell Closeout

## Assumptions

1. Phase 5 Slices 5.1 and 5.2 are materially landed in live code: `crates/cli/src/main.rs` is now a 569-line clap/dispatch entrypoint with dedicated `author`, `setup`, `pipeline`, `generate`, `inspect`, and `doctor` modules instead of the old monolith.
2. The remaining Phase 5 gap is no longer “extract more command families”; it is the root-plan steady-state ownership gap: prompting helpers, operator-facing wording/help, output rendering, and exit-code policy are still split across CLI code and reusable crates.
3. `handbook-cli` should become the clear owner of final shell presentation. Reusable crates may still expose typed data, typed classifications, and narrow structured adapters, but they should not remain the long-term home for final clap help text, final operator-facing markdown/inspect copy, or CLI exit-policy decisions.
4. This slice should preserve the already-landed module layout from Slices 5.1 and 5.2. It must not reopen layout/storage parameterization, orchestration-target parameterization, or compiler narrowing as the main story.
5. Public CLI vocabulary and help posture should remain behaviorally stable unless the user explicitly approves wording changes. Passing `help_drift_guard`, `author_cli`, and `cli_surface` remains the primary proof that ownership changed without product drift.
6. Success is ownership honesty, not merely fewer lines in `main.rs`: if `main.rs` stays about the same size but the final shell surfaces move behind clear CLI-owned boundaries, the slice still succeeds.

## Objective

Finish the Phase 5 steady-state target so `handbook-cli` is the honest owner of the product shell: interactive prompting, operator-facing wording/help, final rendering/presentation, and exit-code decisions.

The maintainer needs this slice because the CLI has already been thinned structurally, but some final shell behavior still leaks through reusable crates or oversized CLI family modules. Success means:

- prompting helpers are isolated behind explicit CLI-owned shell surfaces instead of living inline inside large command modules
- final help text and example ownership sits in `handbook-cli` rather than in reusable crates such as `handbook-pipeline`
- final generate/inspect/doctor/setup rendering and exit-code decisions are owned by CLI-local presentation adapters
- reusable crates expose reusable data and narrow typed helpers, not the final operator shell copy
- `main.rs` remains thin and honest after the closeout

## Tech Stack

- Rust 2021 workspace
- crates in play:
  - `handbook-cli`
  - `handbook-pipeline`
  - `handbook-flow`
  - `handbook-engine`
  - `handbook-compiler` only where live code still exposes shell-facing rendering/support glue that this slice needs to narrow
- clap 4 derive-based CLI
- Phase 5 planning authorities:
  - `HANDBOOK_ENGINE_EXTRACTION_PLAN.md`
  - `docs/specs/handbook-engine-extraction-slice-map.md`
  - `docs/specs/handbook-engine-extraction-closeout-four-set-map.md`
  - `docs/specs/handbook-engine-extraction-phase-5-slice-1-cli-skeleton-and-author-setup-*.md`
  - `docs/specs/handbook-engine-extraction-phase-5-slice-2-cli-runtime-command-shell-*.md`

## Commands

Shell-ownership evidence sweep:

```bash
rg -n "prompt_|print_help|after_help|SUPPORTED_.*HELP|render_(markdown|inspect|json)|render_supported_handoff_emit_command|ExitCode|OUTCOME:|NEXT SAFE ACTION:" crates/cli/src crates/pipeline/src crates/compiler/src
```

Current shell-thinning snapshot:

```bash
wc -l crates/cli/src/main.rs crates/cli/src/author.rs crates/cli/src/pipeline.rs crates/cli/src/rendering.rs crates/cli/src/setup.rs crates/cli/src/doctor_rendering.rs
```

Targeted CLI verification:

```bash
cargo test -p handbook-cli --test author_cli
cargo test -p handbook-cli --test cli_surface
cargo test -p handbook-cli --test help_drift_guard
cargo test -p handbook-cli --test pipeline_handoff_refusals
cargo test -p handbook-cli --test manual_qa_fixture_checkout
cargo test -p handbook-cli --test feature_spec_contract
cargo test -p handbook-cli
```

Final verification wall:

```bash
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
```

## Project Structure

```text
crates/cli/src/main.rs                        -> top-level clap metadata and command registration; should stay thin while shell ownership becomes clearer
crates/cli/src/author.rs                      -> current author command shell plus a very large guided-prompt helper surface that should no longer stay inline forever
crates/cli/src/setup.rs                       -> setup command shell and setup-specific success/refusal rendering
crates/cli/src/pipeline.rs                    -> pipeline command shell; should consume CLI-owned help/presentation surfaces instead of reusable-crate shell copy
crates/cli/src/generate.rs                    -> generate command shell that currently relies on shared flow rendering helpers
crates/cli/src/inspect.rs                     -> inspect command shell that currently relies on shared flow rendering helpers
crates/cli/src/doctor.rs                      -> doctor command shell
crates/cli/src/doctor_rendering.rs            -> doctor-specific text/JSON presentation helpers already in the CLI package
crates/cli/src/rendering.rs                   -> current flow presentation adapter, fixture-demo injection, and exit-code policy for generate/inspect
crates/cli/src/request_shared.rs              -> packet/request preparation helpers that should remain CLI-owned support code
crates/cli/src/shell_shared.rs                -> root discovery, stdin, and subcommand-help helpers for shared shell behavior
crates/cli/tests/author_cli.rs                -> author prompting and refusal/output regression wall
crates/cli/tests/cli_surface.rs               -> end-to-end command-surface regression wall
crates/cli/tests/help_drift_guard.rs          -> public help snapshot wall for product-shell vocabulary stability
crates/cli/tests/manual_qa_fixture_checkout.rs -> inspect fixture-backed proof behavior wall
crates/cli/tests/pipeline_handoff_refusals.rs -> pipeline refusal / producer-command behavior wall
crates/cli/tests/feature_spec_contract.rs     -> generated feature-spec output contract regression wall
crates/pipeline/src/pipeline.rs               -> currently still exports CLI-facing capture/handoff help summaries/examples
crates/pipeline/src/pipeline_handoff.rs       -> currently still exports supported handoff emit command rendering
crates/compiler/src/rendering/**              -> currently still owns final markdown/inspect shell copy that the CLI consumes indirectly
```

## Code Style

Prefer CLI-owned presentation adapters that take typed reusable-crate results and turn them into final shell output, instead of letting reusable crates stay the owner of operator-facing copy.

```rust
let presentation = cli_rendering::render_flow_markdown(result)?;
println!("{}", presentation.body);
return presentation.exit_code;
```

Conventions:

- keep reusable crates responsible for typed runtime results, not final clap/help shell copy
- keep prompting helpers isolated behind explicit CLI-owned modules or family-local prompt adapters
- keep help text and examples in the CLI shell when they describe CLI behavior rather than reusable runtime truth
- keep exit-code policy explicit and centralized in CLI-owned presentation code
- prefer family-local shell helpers first; only introduce cross-family helper modules when there is real live reuse now
- avoid trampoline modules that merely re-export old behavior without clarifying ownership

## Testing Strategy

- Framework: existing `cargo test` unit/integration tests plus help snapshots in `crates/cli/tests/**`
- Primary test levels:
  - `author_cli` for guided-prompting, author refusal posture, and authored-output stability
  - `help_drift_guard` for public CLI help/output vocabulary stability
  - `cli_surface` for end-to-end shell behavior across setup/generate/inspect/doctor/pipeline surfaces
  - `pipeline_handoff_refusals` for CLI-visible pipeline handoff messaging and supported command posture
  - `manual_qa_fixture_checkout` for fixture-demo inspection behavior
  - `feature_spec_contract` for generated feature-spec output contract stability
- Coverage focus:
  - guided prompting still behaves the same after prompt helper extraction
  - help summaries/examples remain stable after CLI takes ownership away from reusable crates
  - generate/inspect output and exit codes remain stable after rendering ownership moves to the CLI shell
  - doctor/setup output remains stable when final presentation ownership is clarified
  - reusable crates no longer need to expose final shell copy except for explicitly temporary compatibility glue
- Coverage expectation:
  - Packet 5.3.1 proves prompting/rendering/help helper extraction without shell drift
  - Packet 5.3.2 proves exit-code policy and final shell ownership are fully CLI-local under the full workspace wall

## Slice Scope

In scope:

- isolate CLI prompting helpers now embedded in `author.rs` into explicit CLI-owned shell surfaces
- move CLI help summaries/examples and similar product-shell copy out of reusable crates where they currently leak shell ownership
- move final generate/inspect presentation ownership behind CLI-local rendering adapters
- keep doctor/setup/pipeline/generate/inspect exit-code policy clearly owned by the CLI shell
- reduce or justify remaining reusable-crate exports whose only job is final CLI shell copy
- preserve behavior while making shell ownership explicit

Out of scope:

- Phase 1 layout/storage parameterization repairs
- Phase 2 orchestration-target parameterization work
- Phase 4 caller rewires or `handbook-compiler` retirement as the main story
- new command verbs, new supported targets, or new product-shell features
- broad copywriting or vocabulary redesign beyond behavior-preserving ownership cleanup
- migration planning into Substrate

## Authority Inputs

- `HANDBOOK_ENGINE_EXTRACTION_PLAN.md`
- `docs/specs/handbook-engine-extraction-slice-map.md`
- `docs/specs/handbook-engine-extraction-closeout-four-set-map.md`
- `docs/specs/handbook-engine-extraction-phase-5-slice-1-cli-skeleton-and-author-setup-spec.md`
- `docs/specs/handbook-engine-extraction-phase-5-slice-1-cli-skeleton-and-author-setup-plan.md`
- `docs/specs/handbook-engine-extraction-phase-5-slice-1-cli-skeleton-and-author-setup-tasks.md`
- `docs/specs/handbook-engine-extraction-phase-5-slice-2-cli-runtime-command-shell-spec.md`
- `docs/specs/handbook-engine-extraction-phase-5-slice-2-cli-runtime-command-shell-plan.md`
- `docs/specs/handbook-engine-extraction-phase-5-slice-2-cli-runtime-command-shell-tasks.md`
- live shell files and tests:
  - `crates/cli/src/main.rs`
  - `crates/cli/src/author.rs`
  - `crates/cli/src/setup.rs`
  - `crates/cli/src/pipeline.rs`
  - `crates/cli/src/generate.rs`
  - `crates/cli/src/inspect.rs`
  - `crates/cli/src/doctor.rs`
  - `crates/cli/src/doctor_rendering.rs`
  - `crates/cli/src/rendering.rs`
  - `crates/cli/src/request_shared.rs`
  - `crates/cli/src/shell_shared.rs`
  - `crates/cli/tests/author_cli.rs`
  - `crates/cli/tests/cli_surface.rs`
  - `crates/cli/tests/help_drift_guard.rs`
  - `crates/cli/tests/manual_qa_fixture_checkout.rs`
  - `crates/cli/tests/pipeline_handoff_refusals.rs`
  - `crates/cli/tests/feature_spec_contract.rs`
- specific reusable surfaces still leaking shell ownership:
  - `crates/pipeline/src/pipeline.rs`
  - `crates/pipeline/src/pipeline_handoff.rs`
  - `crates/compiler/src/rendering/mod.rs`
  - `crates/compiler/src/rendering/markdown.rs`
  - `crates/compiler/src/rendering/inspect.rs`
  - `crates/compiler/src/rendering/shared.rs`

## Current Repo-Truth Gaps This Closeout Must Finish

| Surface | Current live posture | Slice 5.3 closeout requirement |
| --- | --- | --- |
| `crates/cli/src/main.rs` | thin compared with earlier phases, but still owns top-level clap shell copy while some subcommand help strings come from reusable crates | keep `main.rs` thin while ensuring CLI help ownership is explicit and local |
| `crates/cli/src/author.rs` | command routing, guided-prompt context, prompt primitives, normalization loops, and refusal rendering still live in one ~2k-line module | isolate prompting helpers into explicit CLI-owned shell surfaces without changing author behavior |
| `crates/cli/src/rendering.rs` | generate/inspect output currently converts flow results into compiler render models, delegates final shell text to compiler renderers, injects fixture-demo sections, and decides exit codes | make final generate/inspect presentation and exit-code ownership clearly CLI-local |
| `crates/cli/src/doctor_rendering.rs` and `crates/cli/src/setup.rs` | doctor/setup already present locally, but shell presentation policy is still family-local and not yet reconciled with the broader final-shell ownership target | keep these CLI-owned and align their presentation/exit policy with the final shell boundary |
| `crates/pipeline/src/pipeline.rs` | still exports `SUPPORTED_*HELP_*` clap-facing summaries/examples for capture and handoff | move CLI help copy ownership into `handbook-cli` while preserving target/runtime truth |
| `crates/pipeline/src/pipeline_handoff.rs` | still exports `render_supported_handoff_emit_command` for CLI-visible producer-command copy | reduce reusable-crate ownership of final operator shell copy |
| `crates/compiler/src/rendering/**` | still exports final markdown/inspect shell rendering consumed by CLI flow presentation | leave reusable crates with structured data or narrow helpers, but move final shell rendering ownership to CLI adapters |

## Boundaries

- Always:
  - keep this slice about CLI product-shell ownership
  - preserve Phase 5.1 and 5.2 behavior while clarifying ownership
  - keep `main.rs` thin without replacing old inline code with trampoline-only indirection
  - prefer CLI-owned adapters over reusable-crate shell copy
  - prove shell ownership changes with tests and help snapshots, not narrative claims
- Ask first:
  - any intentional public wording or vocabulary change visible in help snapshots or operator output
  - any change that would force Set 1, Set 2, or Set 3 work to land first
  - any proposal to move reusable data-model logic into the CLI rather than only final shell presentation
- Never:
  - widen into target-parameterization or compiler narrowing as the main story
  - call Phase 5 complete while reusable crates still clearly own final help/render/exit shell behavior without explicit justification
  - hide shell ownership in generic dumping-ground utility modules

## Success Criteria

- Prompting helpers are isolated behind explicit CLI-owned shell surfaces instead of living inline inside oversized command modules.
- Final CLI help summaries/examples are owned in `handbook-cli` rather than exported from reusable crates as clap shell copy.
- Generate/inspect/doctor/setup exit-code and final presentation policy are clearly owned by CLI-local adapters.
- Reusable crates expose structured runtime data or narrow helper surfaces rather than the final operator-facing shell text.
- `crates/cli/src/main.rs` remains thin and honest after the closeout.
- `cargo fmt --all -- --check`, `cargo clippy --workspace --all-targets -- -D warnings`, `cargo test --workspace`, and the targeted CLI tests all pass.

## Open Questions

- Is the smallest durable prompting split a CLI-wide `prompting.rs`/`prompt_context.rs` pair, or should the guided prompt helpers stay author-local but move behind smaller author-owned modules?
- Should the final flow presentation closeout move compiler rendering wholesale into `handbook-cli`, or should the CLI instead own a last-mile formatting layer over reusable structured models while leaving lower-level text helpers temporarily intact?
- Should pipeline handoff/capture help copy live in a dedicated CLI help-text module, or should each command-family module own its own clap-facing shell text locally?
