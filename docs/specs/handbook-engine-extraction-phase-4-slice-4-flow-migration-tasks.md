# Tasks: Handbook Engine Extraction Phase 4 Slice 4 (Slice 4.4) - Flow Crate Migration

Plan reference: [handbook-engine-extraction-phase-4-slice-4-flow-migration-plan.md](./handbook-engine-extraction-phase-4-slice-4-flow-migration-plan.md)

## Prerequisite: flow ownership must become real without forcing premature renderer or compiler-end-state decisions

Slice 4.4 must make `handbook-flow` the implementation home for typed packet-selection and readiness/result logic while keeping `rendering`, `refusal`, `error`, direct caller rewires, and compiler-retirement decisions outside this slice unless a tiny compatibility bridge is strictly required.

- Slice 4.4 must not widen into Phase 4.5 caller rewires or compiler narrowing.

## Packet 4.4.1: Resolver Packet-Result And Budget Migration

- [ ] Task: Flip the flow/compiler dependency posture to support real flow ownership
  - Acceptance: `handbook-flow` is no longer only a workspace-contract forwarder, the resulting package graph is acyclic, and the narrow supporting-type rule is implemented well enough for the flow crate to own its migrated modules directly.
  - Verify: `cargo tree -p handbook-flow -e normal && cargo tree -p handbook-compiler -e normal && cargo check --workspace`
  - Files: `crates/flow/Cargo.toml`, `crates/compiler/Cargo.toml`, optionally `Cargo.toml`, `crates/flow/src/lib.rs`, `crates/compiler/src/lib.rs`, and any narrow supporting files needed for the ownership flip

- [ ] Task: Move budget policy, outcomes, and evaluation behavior into `handbook-flow`
  - Acceptance: the `budget.rs` implementation family has one real owner under `crates/flow/src/**`, and any remaining compiler surfaces are explicit compatibility re-exports or thin adapters rather than duplicate implementation bodies.
  - Verify: `rg -n 'struct BudgetOutcome|enum BudgetDisposition|enum BudgetReason|fn evaluate_budget' crates/flow crates/compiler/src && cargo test -p handbook-flow && cargo test -p handbook-compiler --test resolver_core && cargo test -p handbook-compiler --test refusal_mapping`
  - Files: `crates/flow/src/lib.rs`, `crates/flow/src/budget.rs`, `crates/compiler/src/lib.rs`, `crates/compiler/src/budget.rs`, optionally `crates/flow/tests/**`, `crates/compiler/tests/resolver_core.rs`, `crates/compiler/tests/refusal_mapping.rs`

- [ ] Task: Move packet-result models into `handbook-flow` and update typed consumers to use the flow-owned result surface
  - Acceptance: `packet_result.rs` lives under `crates/flow/src/**`, renderer and refusal-mapping consumers read the flow-owned typed models, and any remaining compiler-local surface is a thin compatibility export rather than a second implementation owner.
  - Verify: `rg -n 'struct PacketResult|struct PacketDecisionSummary|enum PacketVariant|enum PacketBodyNoteKind' crates/flow crates/compiler/src && cargo test -p handbook-flow && cargo test -p handbook-compiler --test rendering_surface`
  - Files: `crates/flow/src/packet_result.rs`, `crates/compiler/src/packet_result.rs`, `crates/compiler/src/lib.rs`, `crates/compiler/src/rendering/shared.rs`, `crates/compiler/src/rendering/model.rs`, `crates/compiler/src/rendering/inspect.rs`, `crates/compiler/src/rendering/markdown.rs`, `crates/compiler/src/rendering/json.rs`, optionally `crates/flow/tests/**`, `crates/compiler/tests/rendering_surface.rs`

- [ ] Task: Move typed packet resolution into `handbook-flow` while keeping compiler on a thin compatibility path
  - Acceptance: `resolve`, `ResolveRequest`, `ResolverResult`, `PacketSelection`, and `PacketSelectionStatus` have one real implementation owner under `crates/flow/src/**`, and `handbook-compiler` retains only compatibility re-exports, thin adapters, or local glue strictly needed to preserve current callers.
  - Verify: `rg -n 'pub fn resolve|struct ResolveRequest|struct ResolverResult|enum PacketSelectionStatus|struct PacketSelection' crates/flow crates/compiler/src && cargo test -p handbook-flow && cargo test -p handbook-compiler --test resolver_core && cargo test -p handbook-compiler --test refusal_mapping && cargo test -p handbook-cli --test cli_surface`
  - Files: `crates/flow/src/resolver.rs`, `crates/compiler/src/resolver.rs`, `crates/compiler/src/lib.rs`, optionally `crates/flow/tests/**`, `crates/compiler/tests/resolver_core.rs`, `crates/compiler/tests/refusal_mapping.rs`, `crates/cli/tests/cli_surface.rs`

- [ ] Task: Move or recreate regression coverage for the migrated flow seam inside the flow crate
  - Acceptance: `handbook-flow` has package-local regression coverage for budget, packet-result, and resolver behavior, and the flow package can prove those surfaces without depending on compiler-owned tests as the primary contract.
  - Verify: `cargo test -p handbook-flow`
  - Files: `crates/flow/tests/**`, optionally the corresponding `crates/compiler/tests/*.rs` transition guards

## Final Slice Verification

- [ ] Task: Run the full Slice 4.4 verification wall after the packet lands
  - Acceptance: `handbook-flow` is the real implementation owner for the approved Slice 4.4 surfaces, the dependency graph is acyclic, flow-owned tests pass, CLI behavior stays stable, and no adjacent-slice leakage appears.
  - Verify: `cargo check --workspace && cargo test -p handbook-flow && cargo test -p handbook-cli --test cli_surface`
  - Files: verification only

## Human Review Gate

Stop after the Slice 4.4 spec, plan, and tasks are reviewed and approved. Do not start implementation packets until the human approves this authority set.
