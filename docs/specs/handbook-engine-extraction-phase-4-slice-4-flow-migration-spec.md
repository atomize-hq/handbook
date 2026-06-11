# Spec: Handbook Engine Extraction Phase 4 Slice 4 (Slice 4.4) - Flow Crate Migration

## Assumptions

1. Slice 4.3 is complete enough in live code that `handbook-pipeline` already owns the approved reusable pipeline runtime families, while `handbook-flow` is still only a scaffold crate that forwards through `handbook-compiler`.
2. The smallest live Phase 4.4 seam is exactly what the slice map says: `resolver`, `packet_result`, and `budget`. `rendering`, `refusal`, `error`, and `doctor` remain important downstream consumers, but they are not being reassigned as long-term owners in this slice.
3. Current CLI and renderer behavior is already contract-sensitive through the typed readiness/result surfaces frozen in `docs/contracts/C-04-resolver-result-and-doctor-blockers.md` and the renderer semantics frozen in `docs/contracts/C-05-renderer-and-proof-surfaces.md`; Slice 4.4 must preserve those shapes and meanings while changing ownership.
4. Direct caller rewires remain deferred to Slice 4.5. During Slice 4.4, `handbook-cli` may continue to consume these surfaces through `handbook-compiler` if that is the smallest compatibility path.
5. If the smallest coherent Slice 4.4 landing requires `handbook-flow` to depend temporarily on compiler-owned support types such as blocker, refusal, decision-log, or rendering-adjacent glue, that is acceptable only if `resolver`, `packet_result`, and `budget` clearly become flow-owned implementations with no duplicate long-term bodies left in compiler.
6. Flow migration must leave the workspace acyclic and simpler than the current scaffold posture. Creating a brand-new shared-domain crate to solve supporting-type placement is out of scope unless the live dependency graph proves the slice impossible otherwise.
7. Help text, public command wording, and broad compiler retirement decisions remain outside Slice 4.4 except for tiny compatibility adjustments strictly required to keep existing tests and callers stable.

## Objective

Move the typed flow/application seam behind `handbook-flow` by making `resolver`, `packet_result`, and `budget` real flow-owned implementations, while preserving current resolver semantics, renderer inputs, refusal/blocker meaning, and CLI behavior through temporary compatibility layers in `handbook-compiler`.

The maintainer needs this slice so Phase 4 no longer leaves the middle-layer packet selection and readiness/result logic stranded in `handbook-compiler` after engine-safe and pipeline-safe logic have already moved elsewhere. Success means:

- `handbook-flow` becomes the real owner for `resolve`, `ResolveRequest`, `ResolverResult`, `PacketResult`, `PacketSelection*`, `Budget*`, and related flow-typed result surfaces
- `handbook-flow` stops being a trivial workspace-contract forwarder and becomes a tested crate with package-local regression coverage
- `handbook-compiler` keeps only explicit compatibility exports or thin adapters for migrated flow-owned APIs
- renderer and refusal-mapping behavior stay stable even though their typed inputs now come from the flow crate
- `cargo test -p handbook-flow` and `cargo test -p handbook-cli --test cli_surface` pass after the migration lands

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
  - `docs/specs/handbook-engine-extraction-phase-4-slice-1-crate-scaffold-spec.md`
  - `docs/specs/handbook-engine-extraction-phase-4-slice-1-crate-scaffold-plan.md`
  - `docs/specs/handbook-engine-extraction-phase-4-slice-1-crate-scaffold-tasks.md`
  - `docs/specs/handbook-engine-extraction-phase-4-slice-3-pipeline-migration-spec.md`
  - `docs/specs/handbook-engine-extraction-phase-4-slice-3-pipeline-migration-plan.md`
  - `docs/specs/handbook-engine-extraction-phase-4-slice-3-pipeline-migration-tasks.md`
  - `docs/contracts/C-04-resolver-result-and-doctor-blockers.md`
  - `docs/contracts/C-05-renderer-and-proof-surfaces.md`

## Commands

Primary slice verifier:

```bash
cargo test -p handbook-flow
cargo test -p handbook-cli --test cli_surface
```

Dependency-posture verification:

```bash
cargo tree -p handbook-flow -e normal
cargo tree -p handbook-compiler -e normal
```

Focused flow migration guards:

```bash
cargo test -p handbook-compiler --test resolver_core
cargo test -p handbook-compiler --test refusal_mapping
cargo test -p handbook-compiler --test rendering_surface
```

Ownership scan:

```bash
rg -n 'pub fn resolve|struct ResolveRequest|struct ResolverResult|struct PacketResult|enum PacketSelectionStatus|struct BudgetOutcome|enum BudgetDisposition' crates/flow crates/compiler/src
```

Compatibility-consumer scan:

```bash
rg -n 'PacketResult|BudgetOutcome|BudgetDisposition|PacketSelectionStatus|ResolveRequest|ResolverResult' crates/compiler/src/rendering crates/compiler/tests crates/cli/tests
```

Final slice verification wall:

```bash
cargo check --workspace
cargo test -p handbook-flow
cargo test -p handbook-cli --test cli_surface
```

## Project Structure

```text
crates/flow/Cargo.toml                                -> Slice 4.4 turns handbook-flow into a real implementation crate instead of a scaffold wrapper
crates/flow/src/lib.rs                                -> flow public surface and re-export boundary for migrated resolver/result/budget modules
crates/flow/src/resolver.rs                           -> new owner for typed packet resolution, refusal/blocker assembly, and decision-log-backed result construction
crates/flow/src/packet_result.rs                      -> new owner for packet-result, packet sections, notes, and typed packet decision summary models
crates/flow/src/budget.rs                             -> new owner for budget policy, outcome, targets, and evaluation behavior
crates/flow/tests/**                                  -> new flow-owned regression coverage for budget, packet-result, and resolver behavior
crates/compiler/Cargo.toml                            -> temporary compatibility crate that should depend on handbook-flow for Slice 4.4 surfaces
crates/compiler/src/lib.rs                            -> temporary compatibility facade that should re-export or thinly delegate to handbook-flow
crates/compiler/src/resolver.rs                       -> current implementation owner to retire or replace with a thin compatibility layer
crates/compiler/src/packet_result.rs                  -> current implementation owner to retire or replace with a thin compatibility layer
crates/compiler/src/budget.rs                         -> current implementation owner to retire or replace with a thin compatibility layer
crates/compiler/src/rendering/{shared,model,inspect,markdown,json}.rs -> downstream typed consumers that should consume flow-owned result types without becoming flow-owned themselves
crates/compiler/tests/{resolver_core,refusal_mapping,rendering_surface}.rs -> existing compiler transition guards while compatibility surfaces remain
crates/cli/tests/cli_surface.rs                       -> public CLI regression guard for behavior stability during the migration
```

## Code Style

Prefer explicit flow-owned exports plus narrow compatibility re-exports over duplicate implementation bodies or wildcard facade cloning.

```rust
pub use handbook_flow::{
    resolve, BudgetDisposition, BudgetOutcome, BudgetPolicy, PacketResult, ResolveRequest,
    ResolverResult,
};
```

Conventions:

- `handbook-flow` owns typed packet-selection, packet-result, and budget semantics
- `handbook-compiler` may keep temporary compatibility exports, but it should not remain the true implementation home for migrated Slice 4.4 symbols
- keep public re-export lists explicit; do not introduce `pub use handbook_flow::*`
- allow narrow dependency bridging only when required to keep renderer, refusal, and CLI behavior stable without widening the slice
- keep direct caller rewires, CLI help edits, and compiler-retirement decisions out of Slice 4.4
- keep the dependency graph acyclic and avoid a second long-term copy of any migrated resolver/result/budget logic

## Testing Strategy

- Framework: Cargo package tests and existing compiler/CLI regression tests
- Primary test levels:
  - `handbook-flow` package tests for migrated budget, packet-result, and resolver behavior
  - `handbook-compiler` integration tests as transition guards while compatibility facades still exist
  - `handbook-cli` `cli_surface` as a public behavior guard
- Coverage focus:
  - migrated typed flow logic has one real owner in `handbook-flow`
  - packet-result semantics and budget dispositions remain stable after migration
  - refusal, blocker, and rendering consumers still interpret flow-owned typed inputs correctly
  - compiler compatibility facades remain thin and easy to narrow further in Slice 4.5
- Coverage expectation:
  - Packet 4.4.1 proves budget, packet-result, and resolver ownership together because the three families are tightly coupled in live code
  - the final slice wall proves `handbook-flow` is a real tested crate while CLI behavior still matches current expectations

## Slice Scope

In scope:

- move `resolver`, `packet_result`, and `budget` behind `handbook-flow`
- create the minimum `handbook-flow` public module layout needed to own those surfaces cleanly
- update package dependencies so the flow crate becomes the implementation owner for Slice 4.4 families
- keep `handbook-compiler` as a temporary compatibility facade for migrated flow-owned APIs
- update renderer and refusal-mapping consumers to use the flow-owned typed surfaces without reassigning renderer ownership
- move or recreate regression coverage in `crates/flow/tests/**` for migrated surfaces
- preserve current CLI behavior while avoiding direct caller rewires

Out of scope:

- moving `rendering`, `refusal`, `error`, or `doctor` into `handbook-flow`
- broad CLI import rewires, help-text changes, or command-surface redesign
- retiring `handbook-compiler` or deciding its final post-extraction shape beyond what is strictly needed for a temporary compatibility facade
- introducing a brand-new shared crate for blocker/refusal/error glue unless the live dependency graph proves Slice 4.4 impossible otherwise
- changing the approved C-04 blocker semantics or the approved C-05 renderer semantics
- Phase 4.5 caller rewires and compiler narrowing work

## Authority Inputs

- `HANDBOOK_ENGINE_EXTRACTION_PLAN.md`
- `docs/specs/handbook-engine-extraction-slice-map.md`
- Slice 4.1 authority set:
  - `docs/specs/handbook-engine-extraction-phase-4-slice-1-crate-scaffold-spec.md`
  - `docs/specs/handbook-engine-extraction-phase-4-slice-1-crate-scaffold-plan.md`
  - `docs/specs/handbook-engine-extraction-phase-4-slice-1-crate-scaffold-tasks.md`
- Slice 4.3 authority set:
  - `docs/specs/handbook-engine-extraction-phase-4-slice-3-pipeline-migration-spec.md`
  - `docs/specs/handbook-engine-extraction-phase-4-slice-3-pipeline-migration-plan.md`
  - `docs/specs/handbook-engine-extraction-phase-4-slice-3-pipeline-migration-tasks.md`
- Resolver and renderer contracts:
  - `docs/contracts/C-04-resolver-result-and-doctor-blockers.md`
  - `docs/contracts/C-05-renderer-and-proof-surfaces.md`
- Live package and module truth:
  - `Cargo.toml`
  - `crates/flow/Cargo.toml`
  - `crates/flow/src/lib.rs`
  - `crates/compiler/Cargo.toml`
  - `crates/compiler/src/lib.rs`
  - `crates/compiler/src/resolver.rs`
  - `crates/compiler/src/packet_result.rs`
  - `crates/compiler/src/budget.rs`
  - `crates/compiler/src/rendering/shared.rs`
  - `crates/compiler/src/rendering/model.rs`
  - `crates/compiler/src/rendering/inspect.rs`
  - `crates/compiler/src/rendering/markdown.rs`
  - `crates/compiler/src/rendering/json.rs`
  - `crates/compiler/tests/resolver_core.rs`
  - `crates/compiler/tests/refusal_mapping.rs`
  - `crates/compiler/tests/rendering_surface.rs`
  - `crates/cli/tests/cli_surface.rs`

## Current Ownership Gap To Close

| Surface | Current live owner | Slice 4.4 requirement |
| --- | --- | --- |
| budget policy, targeting, and outcome evaluation | `crates/compiler/src/budget.rs` | move to `handbook-flow` as the real flow-owned budget owner |
| packet-result models, sections, notes, and packet decision summaries | `crates/compiler/src/packet_result.rs` | move to `handbook-flow` as the real typed packet-result owner |
| typed packet resolution, packet selection, refusal/blocker assembly, and decision-log-backed result creation | `crates/compiler/src/resolver.rs` | move to `handbook-flow` as the real flow/application owner |
| renderer and refusal-mapping typed consumers | `crates/compiler/src/rendering/**` and compiler tests | keep ownership where it is, but make those consumers read flow-owned types instead of compiler-owned implementations |
| flow crate surface | `crates/flow/src/lib.rs` forwarding only `workspace_contract_version()` through compiler | replace the scaffold posture with real flow-owned APIs and package-local regression coverage |

## Boundaries

- Always:
  - make `handbook-flow` the true implementation owner for the approved Slice 4.4 surfaces
  - preserve typed resolver, packet-result, blocker, refusal, and budget semantics while changing ownership
  - keep `handbook-compiler` compatibility layers explicit and thin
  - preserve current CLI and renderer behavior during the migration
  - add or move regression coverage so migrated logic is tested from the flow crate itself
- Ask first:
  - moving `rendering`, `refusal`, `error`, or `doctor` into `handbook-flow`
  - changing public CLI contract wording, help snapshots, or packet proof text
  - introducing a new shared crate to hold supporting types
  - performing direct caller rewires before Slice 4.5
  - revising C-04 or C-05 semantics instead of preserving them
- Never:
  - leave `resolver`, `packet_result`, or `budget` as long-term compiler-owned implementations after Slice 4.4 lands
  - duplicate migrated implementations in both crates as a compatibility strategy
  - widen into direct caller rewires, compiler retirement, or broad renderer/refusal ownership redesign
  - silently change the meaning of budget dispositions, packet sections, refusal categories, blocker ordering, or decision-log-backed result semantics

## Success Criteria

- `handbook-flow` owns the `resolver`, `packet_result`, and `budget` implementation stack.
- `handbook-flow` has package-local regression coverage and `cargo test -p handbook-flow` passes.
- `handbook-compiler` becomes a thin compatibility facade for the migrated flow-owned surfaces instead of remaining their real implementation home.
- Renderer, refusal-mapping, and CLI regression guards keep passing without moving rendering or help ownership into flow.
- The flow crate stops being a trivial compile-through wrapper and exposes the approved typed flow surfaces directly.
- No direct caller rewires, compiler-retirement work, or broad renderer/refusal reclassification leaks into this slice.

## Open Questions

- Is the smallest durable dependency posture for Slice 4.4 a `handbook-flow` crate that temporarily consumes compiler-owned blocker/refusal/decision-log support types, or does live code truth force a slightly wider supporting-type move to avoid an unhealthy long-term edge?
- Should renderers consume flow-owned `PacketResult` and `BudgetOutcome` directly inside `handbook-compiler`, or is a smaller temporary compatibility alias layer in `crates/compiler/src/lib.rs` the cleaner transition path until Slice 4.5 lands?
