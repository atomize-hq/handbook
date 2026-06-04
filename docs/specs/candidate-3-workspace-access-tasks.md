# Tasks: Candidate 3 Workspace Access Deepening

Plan reference: [candidate-3-workspace-access-plan.md](./candidate-3-workspace-access-plan.md)

## Packet 1: Workspace Access Core

- [ ] Task: Add the deep workspace access seam in the compiler
  - Acceptance: One compiler-owned module or module section exposes typed repo-relative normalization, validation, trusted no-follow reads, and trusted write entrypoints without forcing callers to reconstruct those rules inline.
  - Verify: `cargo test -p handbook-compiler repo_file_access`
  - Files: `crates/compiler/src/repo_file_access.rs`, `crates/compiler/src/lib.rs`

- [ ] Task: Add packet-local regression coverage for workspace invariants
  - Acceptance: Unit or integration tests explicitly prove normalized repo-relative paths, symlink refusal, and no-follow read behavior through the new seam.
  - Verify: `cargo test -p handbook-compiler repo_file_access`
  - Files: `crates/compiler/src/repo_file_access.rs`, optionally `crates/compiler/tests/*`

## Packet 2: Canonical Artifact Migration

- [ ] Task: Migrate canonical artifact ingest onto the workspace seam
  - Acceptance: `canonical_artifacts.rs` consumes the shared workspace access seam for canonical `.handbook/` discovery and no-follow file reads instead of keeping a second read helper.
  - Verify: `cargo test -p handbook-compiler canonical_artifacts && cargo test -p handbook-compiler --test resolver_core`
  - Files: `crates/compiler/src/canonical_artifacts.rs`, `crates/compiler/src/repo_file_access.rs`, `crates/compiler/tests/resolver_core.rs`

- [ ] Task: Preserve canonical artifact ingest posture after migration
  - Acceptance: Existing missing, empty, starter-template, and ingest-error behavior remains stable after the migration unless an explicit spec update says otherwise.
  - Verify: `cargo test -p handbook-compiler --test resolver_core`
  - Files: `crates/compiler/src/canonical_artifacts.rs`, `crates/compiler/tests/resolver_core.rs`

## Packet 3: Pipeline Migration

- [ ] Task: Migrate pipeline path validation and stage-file access onto the workspace seam
  - Acceptance: `pipeline.rs` stops owning duplicate repo-relative stage-file validation where the workspace seam can own it, and stage/front-matter reads consume shared trusted file access.
  - Verify: `cargo test -p handbook-compiler --test pipeline_loader && cargo test -p handbook-compiler --test pipeline_catalog`
  - Files: `crates/compiler/src/pipeline.rs`, `crates/compiler/src/repo_file_access.rs`, `crates/compiler/tests/pipeline_loader.rs`, `crates/compiler/tests/pipeline_catalog.rs`

- [ ] Task: Keep pipeline-facing behavior stable after migration
  - Acceptance: Pipeline and stage load failures still classify invalid paths and invalid files correctly, and public CLI fallout remains unchanged.
  - Verify: `cargo test -p handbook-cli --test cli_surface`
  - Files: `crates/compiler/src/pipeline.rs`, `crates/cli/tests/cli_surface.rs`

## Packet 4: Route-State Workspace Cleanup

- [ ] Task: Migrate overlapping route-state filesystem access onto the workspace seam
  - Acceptance: The repo-relative read and traversal rules in `route_state.rs` that belong to workspace ownership move under the shared seam, while route-state-specific domain behavior stays local.
  - Verify: `cargo test -p handbook-compiler --test pipeline_state_store && cargo test -p handbook-compiler --test pipeline_route_resolution`
  - Files: `crates/compiler/src/route_state.rs`, `crates/compiler/src/repo_file_access.rs`, `crates/compiler/tests/pipeline_state_store.rs`, `crates/compiler/tests/pipeline_route_resolution.rs`

- [ ] Task: Preserve runtime-state reset and inventory behavior after migration
  - Acceptance: Runtime-state reset planning and inventory enumeration keep their current safety posture and error behavior after consuming the shared workspace seam.
  - Verify: `cargo test -p handbook-compiler --test pipeline_state_store`
  - Files: `crates/compiler/src/route_state.rs`, `crates/compiler/tests/pipeline_state_store.rs`

## Packet 5: Export Posture And Closeout

- [ ] Task: Decide and document the library export posture for the workspace seam
  - Acceptance: The implementation either keeps the workspace seam internal with clear future-export notes or exposes a small reviewed library surface suitable for downstream consumers, and the choice is reflected in the spec/plan docs.
  - Verify: `cargo check --workspace`
  - Files: `crates/compiler/src/lib.rs`, `docs/specs/candidate-3-workspace-access-spec.md`, `docs/specs/candidate-3-workspace-access-plan.md`

- [ ] Task: Run the final regression wall for the deepening
  - Acceptance: Formatting, lint, targeted compiler tests, CLI regressions, and full workspace verification pass with the new workspace seam in place.
  - Verify: `cargo fmt --all -- --check && cargo clippy --workspace --all-targets --all-features -- -D warnings && cargo test --workspace && cargo check --workspace`
  - Files: no new source files; verification only
