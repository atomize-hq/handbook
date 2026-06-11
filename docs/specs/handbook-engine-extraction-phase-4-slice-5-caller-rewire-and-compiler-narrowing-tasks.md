# Tasks: Handbook Engine Extraction Phase 4 Slice 5 (Slice 4.5) - Caller Rewires And Compiler Narrowing

Plan reference: [handbook-engine-extraction-phase-4-slice-5-caller-rewire-and-compiler-narrowing-plan.md](./handbook-engine-extraction-phase-4-slice-5-caller-rewire-and-compiler-narrowing-plan.md)

## Prerequisite: direct callers must move first so the compiler end state is evidence-driven

Slice 4.5 must make the real owner crates visible at call sites before deciding whether `crates/compiler` can be narrowed sharply or retired entirely.

- Slice 4.5 must not widen into Phase 5 CLI-thinning or new runtime feature work.

## Packet 4.5.1: Direct Caller Rewires To New Crates

- [ ] Task: Inventory and rewire production CLI imports to the real owner crates
  - Acceptance: `crates/cli/src/main.rs` imports engine-owned, pipeline-owned, and flow-owned surfaces directly where earlier slices already established those owners, and any remaining compiler imports are only for explicitly unresolved product-shell surfaces pending Packet 4.5.2.
  - Verify: `rg -n 'handbook_compiler::|use handbook_compiler' crates/cli/src/main.rs && cargo tree -p handbook-cli -e normal && cargo test -p handbook-cli --test cli_surface`
  - Files: `crates/cli/Cargo.toml`, `crates/cli/src/main.rs`, optionally `crates/engine/Cargo.toml`, `crates/pipeline/Cargo.toml`, `crates/flow/Cargo.toml`

- [ ] Task: Rewire CLI tests and support callers to the real owner crates
  - Acceptance: `crates/cli/tests/author_cli.rs` and `crates/cli/tests/cli_surface.rs` stop defaulting to `handbook_compiler::*` for extracted logic, while preserving current public CLI behavior and fixture expectations.
  - Verify: `rg -n 'handbook_compiler::|use handbook_compiler' crates/cli/tests && cargo test -p handbook-cli --test author_cli && cargo test -p handbook-cli --test cli_surface`
  - Files: `crates/cli/tests/author_cli.rs`, `crates/cli/tests/cli_surface.rs`, optionally `crates/cli/Cargo.toml`

- [ ] Task: Remove remaining scaffold-level compiler forwarding from `handbook-flow`
  - Acceptance: `crates/flow/src/lib.rs` stops routing its visible surface through `handbook_compiler::*`, and the flow crate exposes its own approved public contract directly.
  - Verify: `rg -n 'handbook_compiler::|use handbook_compiler' crates/flow && cargo test -p handbook-flow`
  - Files: `crates/flow/src/lib.rs`, optionally `crates/flow/Cargo.toml`, `crates/flow/tests/**`

- [ ] Task: Align crate manifests with the rewired ownership graph
  - Acceptance: `crates/cli` and any other direct consumers depend on `handbook-engine`, `handbook-pipeline`, and `handbook-flow` as needed, and unnecessary dependency edges to `handbook-compiler` are removed or justified by the still-pending compiler posture decision.
  - Verify: `cargo tree -p handbook-cli -e normal && cargo test -p handbook-engine && cargo test -p handbook-pipeline && cargo test -p handbook-flow`
  - Files: `crates/cli/Cargo.toml`, optionally `Cargo.toml`, `crates/compiler/Cargo.toml`, `crates/flow/Cargo.toml`

## Packet 4.5.2: Compiler Narrowing Or Retirement Decision Landing

- [ ] Task: Land the chosen compiler end state as a deliberate workspace posture
  - Acceptance: `crates/compiler` is either reduced to a thin compatibility/support crate with reviewable remaining exports and no real implementation ownership for extracted logic, or it is retired cleanly from the workspace after remaining required surfaces are moved or remapped.
  - Verify: `cargo tree -p handbook-compiler -e normal && cargo test --workspace`
  - Files: `Cargo.toml`, `crates/compiler/Cargo.toml`, `crates/compiler/src/lib.rs`, optionally remaining compiler source files or removal edits if retirement is chosen

- [ ] Task: Update repo-facing ownership docs and guard rails to match the chosen compiler posture
  - Acceptance: the minimum repo-facing docs and command-surface contract that make ownership claims remain truthful after Packet 4.5.2, and help/doc guard coverage reflects the resulting boundary without stale compiler-facade assumptions.
  - Verify: `cargo test -p handbook-cli --test help_drift_guard`
  - Files: `README.md`, `PLAN.md`, `docs/README.md`, `docs/contracts/C-02-rust-workspace-and-cli-command-surface.md`, optionally `crates/cli/tests/help_drift_guard.rs`

## Final Slice Verification

- [ ] Task: Run the full Slice 4.5 verification wall after both packets land
  - Acceptance: direct callers use the real owner crates, the compiler end state is intentional and truthful, docs and help guards are aligned, and the workspace is format-clean, clippy-clean, and test-green.
  - Verify: `cargo fmt --all -- --check && cargo clippy --workspace --all-targets -- -D warnings && cargo test --workspace`
  - Files: verification only

## Human Review Gate

Stop after the Slice 4.5 spec, plan, and tasks are reviewed and approved. Do not start implementation packets until the human approves this authority set.
