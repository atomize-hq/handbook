# Tasks: Handbook Engine Extraction Phase 3 Slice 3 (Slice 3.3) - Environment-Inventory Deterministic Core Split

Plan reference: [handbook-engine-extraction-phase-3-slice-3-environment-inventory-core-split-plan.md](./handbook-engine-extraction-phase-3-slice-3-environment-inventory-core-split-plan.md)

## Prerequisite: Slice 3.2 split pattern, Slice 2.3 template-library ownership, and the public environment-inventory contract stay frozen

Slice 3.2 already established the approved in-place split pattern for authoring surfaces. Slice 2.3 already moved shipped environment-inventory asset selection behind the template-library resolver. Slice 3.3 must stay boundary-cleanup-only: it cannot redesign the CLI command, change the environment-inventory document contract, or reopen adjacent authoring surfaces.

- Slice 3.3 must split deterministic environment-inventory contract logic from shell/runtime behavior, not redesign environment-inventory authoring.

## Packet 3.3.1: Environment-Inventory Deterministic Model Split

- [ ] Task: Introduce a deterministic environment-inventory-core owner for markdown contract validation
  - Acceptance: One environment-inventory-core boundary owns required heading ordering, canonical-file assertions, legacy-path rejection, and exact `Project Context Ref` contract validation, and that boundary does not read environment variables, spawn `codex exec`, allocate temp output files, acquire authoring locks, or mutate repo state.
  - Verify: `cargo test -p handbook-compiler --test author`
  - Files: `crates/compiler/src/author/environment_inventory.rs`, `crates/compiler/src/author/environment_inventory_core.rs`, `crates/compiler/src/author/mod.rs`, `crates/compiler/tests/author.rs`

- [ ] Task: Preserve the current public environment-inventory validation API while moving pure contract checks behind the core boundary
  - Acceptance: `validate_environment_inventory_markdown` remains available through the current public compiler surface, and shell-owned authoring flows consume explicit core-validated expectations instead of reimplementing inline deterministic checks.
  - Verify: `cargo test -p handbook-compiler --test author`
  - Files: `crates/compiler/src/author/environment_inventory.rs`, `crates/compiler/src/author/environment_inventory_core.rs`, `crates/compiler/src/author/mod.rs`, `crates/compiler/tests/author.rs`

## Packet 3.3.2: Environment-Inventory Prompt And Product Cleanup

- [ ] Task: Extract environment-inventory prompt construction and synthesis runtime into shell-owned helpers
  - Acceptance: Upstream charter/project-context loading, template-library prompt assembly, env-var override resolution, temp output handling, `codex exec` invocation, and process-summary formatting move out of the deterministic core and are owned by shell/runtime helpers instead.
  - Verify: `cargo test -p handbook-compiler --test author && cargo test -p handbook-cli --test author_cli`
  - Files: `crates/compiler/src/author/environment_inventory.rs`, `crates/compiler/src/author/environment_inventory_shell.rs`, `crates/compiler/src/author/mod.rs`, `crates/compiler/tests/author.rs`, `crates/cli/tests/author_cli.rs`

- [ ] Task: Thin environment-inventory preflight and authoring entrypoints around the new core and shell boundaries
  - Acceptance: `preflight_author_environment_inventory` and `author_environment_inventory` become thin orchestrators that preserve required charter truth, optional project-context behavior, canonical write behavior, lock semantics, refusal posture, and current public compiler/CLI behavior without resuming ownership of deterministic environment-inventory contract logic.
  - Verify: `cargo test -p handbook-compiler --test author && cargo test -p handbook-cli --test author_cli`
  - Files: `crates/compiler/src/author/environment_inventory.rs`, `crates/compiler/src/author/environment_inventory_shell.rs`, `crates/compiler/src/author/mod.rs`, `crates/compiler/tests/author.rs`, `crates/cli/tests/author_cli.rs`

## Final Slice Verification

- [ ] Task: Run the full Slice 3.3 verification wall after both packets land
  - Acceptance: Deterministic environment-inventory contract logic is isolated from prompt/runtime/mutation behavior, public compiler and CLI behavior remain stable, and workspace checks pass without adjacent-slice leakage.
  - Verify: `cargo check --workspace && cargo test --workspace`
  - Files: verification only

## Human Review Gate

Stop after the Slice 3.3 spec, plan, and tasks are reviewed and approved. Do not start implementation packets until the human approves this authority set.
