# Plan: Handbook Engine Extraction Phase 4 Slice 5 (Slice 4.5) - Caller Rewires And Compiler Narrowing

## Objective

Make the Phase 4 extraction operationally real by rewiring remaining callers directly to `handbook-engine`, `handbook-pipeline`, and `handbook-flow`, then landing one intentional end state for `crates/compiler`: a thin compatibility/support crate or clean retirement.

Spec reference: [handbook-engine-extraction-phase-4-slice-5-caller-rewire-and-compiler-narrowing-spec.md](./handbook-engine-extraction-phase-4-slice-5-caller-rewire-and-compiler-narrowing-spec.md)

## Major Modules

1. Caller inventory and dependency map
   - `crates/cli/Cargo.toml`
   - `crates/cli/src/main.rs`
   - `crates/cli/tests/{author_cli,cli_surface,help_drift_guard}.rs`
   - `crates/flow/src/lib.rs`
   - identifies every direct `handbook_compiler::*` caller that should move to a real owner crate

2. Direct CLI rewires
   - `crates/cli/src/main.rs`
   - rewires production command handlers to engine/pipeline/flow-owned surfaces without changing the CLI product behavior

3. Test and support rewires
   - `crates/cli/tests/{author_cli,cli_surface}.rs`
   - `crates/flow/src/lib.rs`
   - removes remaining compiler-as-default-import patterns from integration callers and crate scaffolds

4. Compiler posture decision
   - `Cargo.toml`
   - `crates/compiler/Cargo.toml`
   - `crates/compiler/src/lib.rs`
   - any remaining compiler-local source files required by the chosen posture
   - chooses and lands either a narrow compatibility/support crate or retirement

5. Doc and guard truth alignment
   - `README.md`
   - `PLAN.md`
   - `docs/README.md`
   - `docs/contracts/C-02-rust-workspace-and-cli-command-surface.md`
   - `crates/cli/tests/help_drift_guard.rs`
   - proves the final ownership model is explained and enforced correctly

## Dependencies And Order

### Prerequisite: freeze the direct-caller inventory and target-owner map

Why first:

- Slice 4.5 cannot succeed without one explicit inventory of which `handbook_compiler::*` usages represent genuine unresolved product-shell ownership and which are stale facade imports
- the compiler posture decision depends on seeing what remains after direct rewires, not on guessing beforehand
- keeping Phase 5 shell-thinning out of scope requires a bounded caller map so import rewires do not turn into open-ended `main.rs` cleanup

Output:

- one agreed caller inventory covering production CLI, CLI tests, and the flow scaffold
- one agreed owner map from each migrated surface to `handbook-engine`, `handbook-pipeline`, or `handbook-flow`
- one agreed rule for compiler end-state evaluation: decide after direct rewires land, not before

### Packet 4.5.1: Direct Caller Rewires To New Crates

Why first:

- the repo cannot choose the final compiler posture intelligently until the real direct-caller surface is visible without the facade
- direct imports make the remaining compiler responsibilities explicit and testable
- rewiring callers first keeps the later compiler decision focused on genuine leftovers rather than convenience imports

Output:

- `handbook-cli` depends directly on the owning crates for approved engine, pipeline, and flow surfaces
- CLI tests and the flow scaffold stop defaulting to `handbook_compiler::*` for extracted logic
- package manifests reflect the real dependency graph after the rewire

### Packet 4.5.2: Compiler Narrowing Or Retirement Decision Landing

Why second:

- the right compiler end state is only clear after direct callers no longer mask the remaining surface area
- docs, contracts, and help guards must describe the chosen end state, not a hypothetical one
- postponing this packet until after rewires keeps the decision reviewable: what remains in compiler is what the repo truly still needs there

Output:

- either `handbook-compiler` becomes a sharply narrowed compatibility/support crate with reviewable remaining exports, or the workspace retires it cleanly
- repo-facing docs and help guards describe the final ownership boundary truthfully
- the full workspace verification wall passes under the chosen end state

## Risks And Mitigations

### Risk: caller rewires expand into a Phase 5 CLI refactor

Mitigation:

- keep the work framed as import-path and manifest rewiring, not module decomposition
- reject opportunistic `main.rs` extraction unless the rewire is impossible without a tiny local helper move
- use the caller inventory as the scope fence for Packet 4.5.1

### Risk: compiler retirement is attempted before the remaining product-shell surfaces are understood

Mitigation:

- decide the compiler end state only after direct rewires land
- allow a narrowed compiler crate if retirement would force premature renderer/refusal/setup relocation
- require explicit documentation of what remains and why if compiler is retained

### Risk: docs and help tests drift from the new ownership model

Mitigation:

- treat doc and guard alignment as part of Packet 4.5.2, not optional cleanup
- keep `help_drift_guard` in the focused verification set throughout the landing
- update only the docs that make ownership or command-surface claims affected by the chosen posture

### Risk: direct caller rewires leave hidden compiler dependencies in tests or scaffolds

Mitigation:

- scan `crates/cli` and `crates/flow` for `handbook_compiler::*` before and after Packet 4.5.1
- use `cargo tree -p handbook-cli -e normal` to confirm the real direct dependency posture
- treat unexpected remaining facade imports as packet leakage unless explicitly justified

## Parallel Vs Sequential

Sequential:

- freeze the caller inventory before changing imports
- land production/test caller rewires before deciding compiler narrowing or retirement
- update docs and help guards only after the chosen compiler posture is known
- run the full workspace wall last

Parallel opportunities after Packet 4.5.1 lands:

- doc updates and help-guard alignment can be prepared in parallel with compiler-surface trimming once the end-state decision is made
- engine/pipeline/flow package tests can be run in parallel while compiler-posture cleanup is being finalized

## Verification Checkpoints

### Checkpoint 1: direct caller inventory is materially reduced

```bash
rg -n 'handbook_compiler::|use handbook_compiler|extern crate handbook_compiler' crates/cli crates/flow
cargo tree -p handbook-cli -e normal
```

### Checkpoint 2: caller rewires preserve CLI behavior

```bash
cargo test -p handbook-cli --test author_cli
cargo test -p handbook-cli --test cli_surface
cargo test -p handbook-engine
cargo test -p handbook-pipeline
cargo test -p handbook-flow
```

### Checkpoint 3: chosen compiler posture is explicit and workspace-clean

```bash
cargo tree -p handbook-compiler -e normal
cargo test --workspace
```

### Final checkpoint

```bash
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
```

## Exit Conditions

The slice is ready for human review when:

- direct callers use the real owner crates for extracted logic
- the CLI dependency graph reflects those direct owners
- `crates/compiler` has a deliberate end state rather than remaining a default umbrella
- docs and help guards are truthful about the resulting ownership model
- the full workspace wall passes
- no Phase 5 CLI-thinning work or new runtime behavior leaked into the landing
