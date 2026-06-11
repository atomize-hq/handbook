# Spec: Handbook Engine Extraction Phase 4 Slice 5 (Slice 4.5) - Caller Rewires And Compiler Narrowing

## Assumptions

1. Slices 4.2 through 4.4 are complete enough in live code that `handbook-engine`, `handbook-pipeline`, and `handbook-flow` already own the approved implementation families moved in those slices, even though many callers still import them through `handbook-compiler` today.
2. The current direct-caller gap is large and real: `crates/cli/src/main.rs`, `crates/cli/tests/**`, and the `handbook-flow` scaffold still contain many `handbook_compiler::*` references even after engine and pipeline crates exist as first-class packages.
3. Slice 4.5 is an ownership-and-import landing, not the full CLI-thinning program. Parsing, prompting, wording, rendering, and exit-code behavior should remain in `handbook-cli`; Phase 5 is where larger `main.rs` decomposition happens.
4. The slice map intentionally leaves two valid end states for `crates/compiler`: a narrowed compatibility/support crate or a retired crate. Slice 4.5 must choose and prove one end state instead of drifting in place.
5. Contract, help, README, and guard-doc truth may need targeted updates during this slice because direct caller ownership claims and crate boundaries become user-visible and test-enforced once the compiler facade stops being the default import path.
6. `rendering`, `refusal`, `error`, `doctor`, and product-facing `setup` may remain where they are if the compiler-narrowing decision proves that is the smallest coherent posture, but they must no longer justify keeping `handbook-compiler` as the main import surface for engine/pipeline/flow-owned logic.
7. The final Slice 4.5 verifier is the full workspace wall: formatting, clippy, and tests. A narrower command set is not enough once direct caller rewires and compiler-shape decisions land.

## Objective

Move remaining callers directly to `handbook-engine`, `handbook-pipeline`, and `handbook-flow` for the logic those crates now own, then intentionally narrow or retire `crates/compiler` so it stops being the default integration center for extracted logic.

The maintainer needs this slice so the Phase 4 extraction becomes operationally real, not just internally reorganized. Success means:

- direct production and test callers stop using `handbook-compiler` as the default path to engine-owned, pipeline-owned, and flow-owned logic
- `crates/cli` depends directly on the crates that own the logic it invokes, subject to any explicitly unresolved product-shell exceptions
- `crates/compiler` lands an intentional end state: either a narrow compatibility/support crate with reduced exports, or a clean retirement from the workspace
- docs, contracts, and help/test guards remain truthful about the resulting ownership model
- `cargo fmt --all -- --check`, `cargo clippy --workspace --all-targets -- -D warnings`, and `cargo test --workspace` pass after the decision lands

## Tech Stack

- Rust 2021 workspace
- Current crates:
  - `handbook-compiler`
  - `handbook-cli`
  - `handbook-engine`
  - `handbook-pipeline`
  - `handbook-flow`
- Authority docs:
  - `HANDBOOK_ENGINE_EXTRACTION_PLAN.md`
  - `docs/specs/handbook-engine-extraction-slice-map.md`
  - `docs/specs/handbook-engine-extraction-phase-4-slice-2-engine-migration-spec.md`
  - `docs/specs/handbook-engine-extraction-phase-4-slice-3-pipeline-migration-spec.md`
  - `docs/specs/handbook-engine-extraction-phase-4-slice-4-flow-migration-spec.md`
  - `docs/contracts/C-02-rust-workspace-and-cli-command-surface.md`
  - `docs/contracts/C-04-resolver-result-and-doctor-blockers.md`
  - `docs/contracts/C-05-renderer-and-proof-surfaces.md`

## Commands

Primary slice verifier:

```bash
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
```

Direct-caller inventory scan:

```bash
rg -n 'handbook_compiler::|use handbook_compiler|extern crate handbook_compiler' crates/cli crates/flow
```

Dependency-posture verification:

```bash
cargo tree -p handbook-cli -e normal
cargo tree -p handbook-compiler -e normal
```

Focused caller-rewire guards:

```bash
cargo test -p handbook-cli --test author_cli
cargo test -p handbook-cli --test cli_surface
cargo test -p handbook-cli --test help_drift_guard
```

Focused crate-owner guards:

```bash
cargo test -p handbook-engine
cargo test -p handbook-pipeline
cargo test -p handbook-flow
```

Optional compiler-posture guards while the crate still exists:

```bash
cargo test -p handbook-compiler --test author
cargo test -p handbook-compiler --test setup
cargo test -p handbook-compiler --test doctor
```

## Project Structure

```text
Cargo.toml                                            -> workspace membership and any compiler-retirement changes
crates/cli/Cargo.toml                                 -> direct crate dependencies for engine/pipeline/flow-owned surfaces
crates/cli/src/main.rs                                -> main production caller set that should stop defaulting to handbook_compiler for extracted logic
crates/cli/tests/author_cli.rs                        -> CLI authoring regression callers that should import engine-owned types directly when possible
crates/cli/tests/cli_surface.rs                       -> CLI integration callers that should import engine/pipeline/flow-owned types directly when possible
crates/cli/tests/help_drift_guard.rs                  -> doc/help truth guard that must stay aligned with the final ownership posture
crates/flow/src/lib.rs                                -> current scaffold forwarder that should stop routing through handbook-compiler
crates/engine/Cargo.toml                              -> direct dependency target for engine-owned authoring/canonical-artifact surfaces
crates/engine/src/lib.rs                              -> public engine surface consumed directly after caller rewires
crates/pipeline/Cargo.toml                            -> direct dependency target for pipeline-owned runtime surfaces
crates/pipeline/src/lib.rs                            -> public pipeline surface consumed directly after caller rewires
crates/flow/Cargo.toml                                -> direct dependency target for flow-owned resolver/result/budget surfaces
crates/flow/src/lib.rs                                -> public flow surface consumed directly after caller rewires
crates/compiler/Cargo.toml                            -> narrowed compatibility/support crate or retirement candidate
crates/compiler/src/lib.rs                            -> current wide re-export hub to narrow sharply or delete
README.md, PLAN.md, docs/README.md                    -> repo-facing boundary docs that may need targeted ownership-truth updates if the compiler posture changes
docs/contracts/C-02-rust-workspace-and-cli-command-surface.md -> command-surface and crate-boundary contract that must stay truthful after the rewire decision
```

## Code Style

Prefer direct imports from the crate that owns the logic over umbrella imports through `handbook-compiler`.

```rust
use handbook_engine::{
    author_charter, parse_charter_structured_input_yaml, CanonicalArtifactKind,
};
use handbook_flow::{resolve, PacketResult, ResolveRequest};
use handbook_pipeline::{compile_pipeline_stage_with_runtime, validate_pipeline_handoff_bundle};
```

Conventions:

- production and test callers should import from the real owner crate when that owner is already established by earlier slices
- if `handbook-compiler` remains after Slice 4.5, its public surface must be intentionally small and justified by the chosen end-state decision
- keep import changes explicit; do not hide the transition behind broad wildcard prelude modules
- keep CLI behavior stable while changing import paths and dependency posture
- do not turn Slice 4.5 into a broad Phase 5 `main.rs` refactor or a renderer/refusal redesign

## Testing Strategy

- Framework: Cargo workspace checks and existing CLI/crate regression tests
- Primary test levels:
  - `handbook-cli` tests for public behavior and doc/help truth
  - direct crate tests for `handbook-engine`, `handbook-pipeline`, and `handbook-flow`
  - compiler tests only as needed to validate the chosen narrow-or-retire posture
- Coverage focus:
  - direct callers no longer rely on `handbook-compiler` for extracted engine/pipeline/flow logic
  - `handbook-cli` behavior remains stable after dependency rewires
  - docs and help guards stay aligned with the resulting ownership truth
  - the final compiler posture is explicit, reviewable, and enforced by the workspace graph
- Coverage expectation:
  - Packet 4.5.1 proves direct caller rewires first
  - Packet 4.5.2 proves the final compiler posture and doc/help truth second
  - the final slice wall proves the workspace is consistent, warning-clean, and fully tested under the chosen end state

## Slice Scope

In scope:

- rewire direct production and test callers to `handbook-engine`, `handbook-pipeline`, and `handbook-flow` for surfaces those crates now own
- update crate manifests so dependencies reflect the real ownership graph
- stop using `handbook-compiler` as the default import hub for extracted logic
- choose and land one intentional `crates/compiler` end state: narrowed compatibility/support crate or retirement
- update docs/contracts/help guards required to keep ownership claims truthful after the chosen end state lands

Out of scope:

- broad CLI module decomposition, prompt-flow rewrites, or other Phase 5 shell-thinning work
- major renderer/refusal/error reclassification beyond what is strictly required for the chosen compiler posture
- new runtime features or semantic changes to engine, pipeline, or flow behavior
- speculative extraction of additional surfaces not already approved by earlier slices
- ownership or integration planning for Substrate after extraction

## Authority Inputs

- `HANDBOOK_ENGINE_EXTRACTION_PLAN.md`
- `docs/specs/handbook-engine-extraction-slice-map.md`
- Slice 4.2 authority set:
  - `docs/specs/handbook-engine-extraction-phase-4-slice-2-engine-migration-spec.md`
  - `docs/specs/handbook-engine-extraction-phase-4-slice-2-engine-migration-plan.md`
  - `docs/specs/handbook-engine-extraction-phase-4-slice-2-engine-migration-tasks.md`
- Slice 4.3 authority set:
  - `docs/specs/handbook-engine-extraction-phase-4-slice-3-pipeline-migration-spec.md`
  - `docs/specs/handbook-engine-extraction-phase-4-slice-3-pipeline-migration-plan.md`
  - `docs/specs/handbook-engine-extraction-phase-4-slice-3-pipeline-migration-tasks.md`
- Slice 4.4 authority set:
  - `docs/specs/handbook-engine-extraction-phase-4-slice-4-flow-migration-spec.md`
  - `docs/specs/handbook-engine-extraction-phase-4-slice-4-flow-migration-plan.md`
  - `docs/specs/handbook-engine-extraction-phase-4-slice-4-flow-migration-tasks.md`
- Command and renderer contracts:
  - `docs/contracts/C-02-rust-workspace-and-cli-command-surface.md`
  - `docs/contracts/C-04-resolver-result-and-doctor-blockers.md`
  - `docs/contracts/C-05-renderer-and-proof-surfaces.md`
- Live package and module truth:
  - `Cargo.toml`
  - `crates/cli/Cargo.toml`
  - `crates/cli/src/main.rs`
  - `crates/cli/tests/author_cli.rs`
  - `crates/cli/tests/cli_surface.rs`
  - `crates/cli/tests/help_drift_guard.rs`
  - `crates/flow/Cargo.toml`
  - `crates/flow/src/lib.rs`
  - `crates/engine/Cargo.toml`
  - `crates/engine/src/lib.rs`
  - `crates/pipeline/Cargo.toml`
  - `crates/pipeline/src/lib.rs`
  - `crates/compiler/Cargo.toml`
  - `crates/compiler/src/lib.rs`
  - `README.md`
  - `PLAN.md`
  - `docs/README.md`

## Current Ownership Gap To Close

| Surface | Current live caller posture | Slice 4.5 requirement |
| --- | --- | --- |
| engine-owned authoring and canonical-artifact helpers | `crates/cli/src/main.rs` and `crates/cli/tests/author_cli.rs` still import them through `handbook_compiler::*` | rewire direct callers to `handbook-engine` where the owner is already established |
| pipeline-owned compile/capture/handoff and route-state helpers | `crates/cli/src/main.rs` and `crates/cli/tests/cli_surface.rs` still import them through `handbook_compiler::*` | rewire direct callers to `handbook-pipeline` where the owner is already established |
| flow-owned resolver/result/budget helpers | `crates/cli/src/main.rs`, `crates/cli/tests/cli_surface.rs`, and `crates/flow/src/lib.rs` still route through `handbook_compiler::*` | rewire direct callers to `handbook-flow` where the owner is already established |
| compiler crate public surface | `crates/compiler/src/lib.rs` is still a wide umbrella re-export hub | narrow it to an intentional compatibility/support surface or retire it entirely |
| repo-facing ownership truth | docs and guards still assume compiler is an important command-surface and crate-boundary authority | update the minimum docs and tests needed so the final ownership posture is truthful and enforced |

## Boundaries

- Always:
  - move direct callers to the real owner crate when that owner is already approved by earlier slices
  - keep CLI behavior stable while changing imports and package dependencies
  - land one explicit compiler end state instead of leaving the crate as an accidental umbrella
  - keep docs, contracts, and help guards truthful about the resulting ownership model
- Ask first:
  - broad CLI module decomposition beyond import and dependency rewires
  - major renderer/refusal/error relocation not forced by the chosen compiler posture
  - retiring `handbook-compiler` if live code still proves essential unresolved product-shell ownership there
  - changing user-visible command wording instead of preserving it through the rewire
- Never:
  - keep `handbook-compiler` as the default import hub for logic that already has a real owner crate
  - claim compiler retirement or narrowing without proving the resulting workspace graph and tests
  - use Slice 4.5 to smuggle in new runtime features or a broad CLI redesign
  - leave docs/help truth stale after changing the ownership model

## Success Criteria

- Direct production and test callers use `handbook-engine`, `handbook-pipeline`, and `handbook-flow` for the surfaces those crates own.
- `crates/cli/Cargo.toml` reflects the real ownership graph instead of defaulting to `handbook-compiler` for extracted logic.
- `crates/compiler` lands a deliberate end state: thin compatibility/support crate or retirement.
- Repo-facing docs and help guards remain truthful about the resulting command-surface and crate-boundary posture.
- `cargo fmt --all -- --check`, `cargo clippy --workspace --all-targets -- -D warnings`, and `cargo test --workspace` all pass.
- No Phase 5 shell-thinning work, new runtime features, or speculative extractions leak into the slice.

## Open Questions

- Is the smallest durable end state a retained `handbook-compiler` crate that holds only explicitly unresolved product-shell compatibility/support surfaces, or can the workspace retire compiler cleanly once direct caller rewires land?
- If `handbook-compiler` remains, which exact surfaces are justified enough to stay there after Slice 4.5, and which ones must be treated as accidental leftovers that should block approval?
