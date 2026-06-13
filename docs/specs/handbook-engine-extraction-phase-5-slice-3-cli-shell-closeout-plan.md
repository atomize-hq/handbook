# Plan: Handbook Engine Extraction Phase 5 Slice 3 (Slice 5.3) - CLI Shell Closeout

## Objective

Finish Phase 5 by closing the remaining CLI product-shell ownership gap: prompting, help wording, final rendering/presentation, and exit-code policy should sit clearly in `handbook-cli` while reusable crates narrow back to typed runtime and support surfaces.

Spec reference: [handbook-engine-extraction-phase-5-slice-3-cli-shell-closeout-spec.md](./handbook-engine-extraction-phase-5-slice-3-cli-shell-closeout-spec.md)

## Major Modules

1. Prompting helper boundary
   - `crates/cli/src/author.rs`
   - any new CLI prompt/helper modules that isolate guided prompt context, prompt primitives, and author-shell refusal presentation

2. Help and shell-copy boundary
   - `crates/cli/src/main.rs`
   - `crates/cli/src/pipeline.rs`
   - any new CLI help-text module(s)
   - reusable help-copy exporters currently living in `crates/pipeline/src/pipeline.rs` and `crates/pipeline/src/pipeline_handoff.rs`

3. Final presentation boundary
   - `crates/cli/src/rendering.rs`
   - `crates/cli/src/doctor_rendering.rs`
   - `crates/cli/src/setup.rs`
   - `crates/cli/src/generate.rs`
   - `crates/cli/src/inspect.rs`
   - reusable compiler rendering exporters currently living in `crates/compiler/src/rendering/**`

4. Exit-code policy boundary
   - CLI command-family modules and shared presentation helpers that currently decide success vs failure for setup/generate/inspect/doctor/pipeline flows

5. Verification wall
   - `author_cli`
   - `cli_surface`
   - `help_drift_guard`
   - `pipeline_handoff_refusals`
   - `manual_qa_fixture_checkout`
   - `feature_spec_contract`
   - full workspace fmt/clippy/test wall

## Dependencies And Order

### Prerequisite: map every remaining shell-owned surface before moving ownership

Why first:

- Slice 5.3 is a closeout seam, so the main failure mode is widening into broad refactors without proving which surfaces still leak shell ownership
- the live repo already has a mostly-thin shell; the work now depends on precise evidence, not generic “clean up CLI” instincts
- agreeing on which surfaces are prompt/help/render/exit ownership prevents accidental reopen of Sets 1 through 3

Output:

- one explicit list of remaining prompt/help/render/exit ownership leaks
- one chosen placement rule for new helpers: family-local first, shared only where there is live multi-family reuse now
- one confirmed fence that target/runtime semantics remain in reusable crates while final shell copy moves to CLI adapters

### Packet 5.3.1: Prompting, Rendering, And Help Helper Extraction

Why first:

- the largest remaining shell debts are helper-ownership debts, not broad runtime-family extractions
- moving prompt/help/render helpers first narrows the surface area before finalizing exit policy
- this packet can preserve existing behavior while making the future Packet 5.3.2 exit-policy closeout smaller and easier to review

Output:

- prompting helpers extracted from `author.rs` into explicit CLI-owned shell surfaces
- CLI-owned help text and example ownership for command-family shell copy now leaking from reusable crates
- CLI-local rendering adapters that own the last-mile generate/inspect/doctor/setup shell presentation, even if reusable crates still temporarily supply typed support helpers underneath
- targeted tests and help snapshots proving behavior stability after ownership moves

### Packet 5.3.2: Exit-Code And Final Shell Closeout

Why second:

- once prompt/help/render ownership is localized to CLI code, exit-code policy can be closed out without ambiguity about who owns the final shell
- exit behavior is the most semantically sensitive remaining slice of the product shell, so it should land after presentation ownership is already clear
- this packet is the final proof that Phase 5 reached the root-plan steady state instead of just spreading shell code across more files

Output:

- explicit CLI-owned exit-code policy for the remaining command families and shared flow presentations
- `main.rs` and CLI modules remain thin and honest after final cleanup
- any remaining reusable-crate shell-copy exports are either removed or explicitly justified as temporary compatibility glue
- full workspace proof that the final shell closeout did not change public behavior

## Risks And Mitigations

### Risk: Slice 5.3 turns into a generic CLI rewrite

Mitigation:

- keep the work bounded to prompt/help/render/exit ownership
- do not add new verbs, new targets, or shell redesign features
- use the closeout-four-set map and the Slice 5.2 tasks doc as explicit fences

### Risk: reusable crates still remain the hidden owners of final shell copy after the refactor

Mitigation:

- audit clap `about` / `after_help`, pipeline help exporters, compiler render exporters, and producer-command renderers explicitly
- reject landings that only move call sites without clarifying ownership
- require evidence-sweep re-runs before calling the slice complete

### Risk: prompt helper extraction breaks guided interview or author refusal behavior

Mitigation:

- use `author_cli` throughout Packet 5.3.1
- keep guided prompt context and refusal rendering behavior-preserving
- split helpers by real ownership instead of extracting a vague shared “utils” bucket

### Risk: help snapshots drift while moving help ownership into CLI

Mitigation:

- keep `help_drift_guard` in the packet verifier, not only the final wall
- move help text ownership without changing public wording unless explicitly approved
- treat snapshot drift as a design decision, not a silent byproduct

### Risk: exit-code cleanup accidentally changes command semantics

Mitigation:

- close prompt/help/render ownership first so exit policy becomes a smaller final decision
- keep `cli_surface`, `manual_qa_fixture_checkout`, `pipeline_handoff_refusals`, and `feature_spec_contract` in the final wall
- centralize exit decisions only where that clarifies ownership; do not flatten semantically distinct command paths

## Parallel Vs Sequential

Sequential:

- perform the ownership inventory before editing
- land Packet 5.3.1 before Packet 5.3.2
- hold the full workspace wall until final closeout

Parallel opportunities after Packet 5.3.1 lands:

- targeted CLI tests can run in parallel while final exit-policy cleanup is underway
- prompt-helper and help-text follow-up nits can be reviewed separately from exit-policy cleanup if the shared ownership rules are already frozen

## Verification Checkpoints

### Checkpoint 1: remaining shell-ownership leaks are fully mapped

```bash
rg -n "prompt_|print_help|after_help|SUPPORTED_.*HELP|render_(markdown|inspect|json)|render_supported_handoff_emit_command|ExitCode|OUTCOME:|NEXT SAFE ACTION:" crates/cli/src crates/pipeline/src crates/compiler/src
wc -l crates/cli/src/main.rs crates/cli/src/author.rs crates/cli/src/pipeline.rs crates/cli/src/rendering.rs crates/cli/src/setup.rs crates/cli/src/doctor_rendering.rs
```

### Checkpoint 2: Packet 5.3.1 preserves prompt/help/render behavior

```bash
cargo test -p handbook-cli --test author_cli
cargo test -p handbook-cli --test help_drift_guard
cargo test -p handbook-cli --test cli_surface
cargo test -p handbook-cli --test pipeline_handoff_refusals
```

### Checkpoint 3: final exit-policy closeout preserves shell semantics

```bash
cargo test -p handbook-cli --test cli_surface
cargo test -p handbook-cli --test manual_qa_fixture_checkout
cargo test -p handbook-cli --test feature_spec_contract
cargo test -p handbook-cli
```

### Final checkpoint

```bash
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
```

## Exit Conditions

The slice is ready for human review when:

- prompt/help/render/exit ownership is clearly CLI-local
- reusable crates no longer act as the de facto owners of final shell copy except for explicitly justified temporary glue
- `main.rs` remains thin and the shell-module boundaries are still honest
- public CLI behavior, help snapshots, guided prompting, and packet proof output remain stable
- the targeted CLI tests and the full workspace wall pass
- the resulting docs/notes still keep Sets 1 through 3 explicitly out of scope
