# Plan: Handbook Engine Extraction Phase 4 Slice 4 (Slice 4.4) - Flow Crate Migration

## Objective

Turn `handbook-flow` from a Slice 4.1 scaffold into the real implementation owner for typed packet-selection and readiness/result logic by moving `resolver`, `packet_result`, and `budget` behind the flow crate, while keeping current CLI behavior and renderer semantics stable through a temporary `handbook-compiler` compatibility facade.

Spec reference: [handbook-engine-extraction-phase-4-slice-4-flow-migration-spec.md](./handbook-engine-extraction-phase-4-slice-4-flow-migration-spec.md)

## Major Modules

1. Flow crate package and public surface
   - `crates/flow/Cargo.toml`
   - `crates/flow/src/lib.rs`
   - owns the package dependency posture and public flow-facing export boundary

2. Budget migration
   - `crates/compiler/src/budget.rs`
   - destination equivalent under `crates/flow/src/budget.rs`
   - becomes the first flow-owned typed policy/result family

3. Packet-result migration
   - `crates/compiler/src/packet_result.rs`
   - destination equivalent under `crates/flow/src/packet_result.rs`
   - becomes the flow-owned typed packet composition/result model

4. Resolver migration
   - `crates/compiler/src/resolver.rs`
   - destination equivalent under `crates/flow/src/resolver.rs`
   - moves the live packet-selection and result-assembly owner without moving renderer or help ownership

5. Compiler compatibility and typed consumers
   - `crates/compiler/Cargo.toml`
   - `crates/compiler/src/lib.rs`
   - `crates/compiler/src/rendering/{shared,model,inspect,markdown,json}.rs`
   - proves compiler consumers can read the flow-owned result types without keeping compiler as the implementation owner

6. Regression coverage
   - `crates/flow/tests/**`
   - `crates/compiler/tests/{resolver_core,refusal_mapping,rendering_surface}.rs`
   - `crates/cli/tests/cli_surface.rs`
   - proves the new crate owns what moved and the public product surface stays stable

## Dependencies And Order

### Prerequisite: freeze the supporting-type and compatibility posture

Why first:

- Slice 4.4 cannot succeed if the repo starts moving `resolver`, `packet_result`, and `budget` without one explicit rule for how compiler-owned support types are treated
- renderer, refusal, blocker, decision-log, and error consumers must remain out of scope unless the migration absolutely requires a tiny supporting move
- moving `budget` and `packet_result` first makes the resolver move smaller and prevents the new flow owner from depending on compiler-owned implementations of its own typed result models

Output:

- one agreed dependency rule for `handbook-flow`: become the implementation owner without creating a cycle or a second long-term copy of migrated logic
- one agreed support-type rule: keep blocker/refusal/error/rendering ownership outside the slice unless a tiny compatibility bridge is strictly required
- one agreed compatibility rule: `handbook-compiler` may remain as a thin facade, but migrated flow logic must live in `handbook-flow`

### Packet 4.4.1: Resolver Packet-Result And Budget Migration

Why this packet stays together:

- live code makes `resolver`, `packet_result`, and `budget` tightly coupled, so splitting them across multiple packets would create temporary cross-crate duplicate ownership or unhealthy circular staging
- `packet_result` depends on budget and packet-selection state, and resolver is the current builder for both; moving them together is the smallest coherent ownership landing
- downstream renderer and CLI compatibility can stay stable if the typed trio lands together behind a single flow-owned namespace

Output:

- `handbook-flow` owns budget policy and evaluation behavior
- `handbook-flow` owns packet-result models and packet decision summary structures
- `handbook-flow` owns `resolve`, `ResolveRequest`, `ResolverResult`, and related packet-selection types
- `handbook-compiler` keeps only thin re-exports or adapters for the migrated flow-owned APIs
- flow-owned regression tests exist for the migrated flow seam while compiler and CLI compatibility guards still pass

## Risks And Mitigations

### Risk: support-type coupling causes the slice to explode into renderer/refusal/error relocation

Mitigation:

- keep the support-type rule explicit before moving code: only move or bridge what is required for real flow ownership
- prefer narrow compiler compatibility imports over premature renderer/refusal ownership decisions
- treat a brand-new shared crate as out of scope unless the live dependency graph proves the packet impossible otherwise

### Risk: `handbook-flow` becomes a new wrapper over compiler instead of a real owner

Mitigation:

- move implementation once, then leave only explicit compatibility re-exports or thin adapters behind
- inspect both `cargo tree -p handbook-flow -e normal` and `cargo tree -p handbook-compiler -e normal`
- scan the repo for duplicate ownership after the packet lands

### Risk: renderer or refusal-mapping behavior drifts even if the typed models compile

Mitigation:

- keep `rendering_surface` and `refusal_mapping` transition guards running while the typed models move
- preserve C-04 and C-05 semantics as frozen contract truth for the migrated result shapes
- avoid renderer rewrites; only update typed imports and compatibility bridges that are necessary for the ownership move

### Risk: Slice 4.4 widens into direct caller rewires or compiler retirement

Mitigation:

- keep `handbook-cli` on the compatibility path during Slice 4.4
- reserve direct dependency rewires for Slice 4.5
- treat large `main.rs` import churn, help changes, or compiler-workspace retirement as adjacent-slice leakage

## Parallel Vs Sequential

Sequential:

- freeze the dependency/support-type posture before moving implementation ownership
- move budget and packet-result before or alongside resolver so the new resolver owner consumes flow-owned typed models
- verify flow ownership and compiler compatibility before checking CLI behavior

Parallel opportunities after the new flow namespace lands:

- flow package tests can be refined in parallel with compiler-facade cleanup
- renderer import updates and compiler re-export cleanup can proceed in parallel once the flow-owned typed modules are fixed

## Verification Checkpoints

### Checkpoint 1: dependency posture matches real flow ownership intent

```bash
cargo tree -p handbook-flow -e normal
cargo tree -p handbook-compiler -e normal
cargo check --workspace
```

### Checkpoint 2: budget and packet-result are flow-owned and tested

```bash
rg -n 'struct BudgetOutcome|enum BudgetDisposition|struct PacketResult|struct PacketDecisionSummary' crates/flow crates/compiler/src
cargo test -p handbook-flow
cargo test -p handbook-compiler --test rendering_surface
```

### Checkpoint 3: resolver is flow-owned while compiler compatibility stays intact

```bash
rg -n 'pub fn resolve|struct ResolveRequest|struct ResolverResult|enum PacketSelectionStatus' crates/flow crates/compiler/src
cargo test -p handbook-flow
cargo test -p handbook-compiler --test resolver_core
cargo test -p handbook-compiler --test refusal_mapping
cargo test -p handbook-cli --test cli_surface
```

### Final checkpoint

```bash
cargo check --workspace
cargo test -p handbook-flow
cargo test -p handbook-cli --test cli_surface
```

## Exit Conditions

The slice is ready for human review when:

- `handbook-flow` is the real implementation owner for `resolver`, `packet_result`, and `budget`
- the package dependency graph shows a real flow crate rather than a trivial compiler-forwarding wrapper
- `handbook-compiler` remains only a temporary compatibility facade for the migrated flow-owned surfaces
- flow-owned regression coverage exists and passes
- compiler renderer/refusal-mapping guards still pass without moving those surfaces into flow
- the CLI surface still passes without a broad direct-caller rewire
- no Phase 4.5 caller-rewire or compiler-retirement work leaked into the landing
