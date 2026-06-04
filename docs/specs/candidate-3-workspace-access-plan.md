# Plan: Candidate 3 Workspace Access Deepening

## Objective

Implement one deep compiler-owned workspace access seam so pipeline loading, canonical artifact ingest, and remaining route-state filesystem operations stop reimplementing repo-relative validation and no-follow access rules inline.

Spec reference: [candidate-3-workspace-access-spec.md](./candidate-3-workspace-access-spec.md)

## Major Modules

1. Workspace access core
   - centered on `crates/compiler/src/repo_file_access.rs` or a renamed successor
   - owns repo-relative normalization, trusted file reads, trusted writes, and canonical path resolution

2. Canonical artifact adapter
   - `crates/compiler/src/canonical_artifacts.rs`
   - consumes workspace primitives for `.handbook/` discovery and artifact ingest

3. Pipeline adapter
   - `crates/compiler/src/pipeline.rs`
   - consumes workspace primitives for pipeline/stage path validation and stage/front-matter loading

4. Route-state adapter
   - `crates/compiler/src/route_state.rs`
   - consumes workspace primitives for the remaining filesystem-owning behavior: inventory reads, reset planning, and safe file access where it overlaps current helpers

5. Export and regression layer
   - `crates/compiler/src/lib.rs`
   - compiler and CLI tests that prove no public behavior drift

## Dependencies And Order

### Phase 1: Define the deep workspace seam

Build the workspace access core first.

Why first:

- every downstream migration depends on stable path, read, and discovery behavior
- without one typed seam, caller migrations will keep duplicating validation rules

Output:

- one typed workspace access entrypoint
- one owner for repo-relative normalization and validation
- one owner for trusted no-follow file reads and trusted writes

### Phase 2: Migrate canonical artifact ingest

Canonical artifact ingest is the cleanest consumer after the core.

Why second:

- it has clear `.handbook/` discovery semantics
- it currently duplicates no-follow read behavior directly

Output:

- `canonical_artifacts.rs` stops owning its own read helper and leans on the workspace seam

### Phase 3: Migrate pipeline file loading

Pipeline loading should move next.

Why third:

- it has duplicated repo-relative validation today
- stage and front-matter reads are a direct test of the deeper read seam

Output:

- `pipeline.rs` stops owning a second repo-relative validator for stage/pipeline file access where the workspace seam can own it

### Phase 4: Migrate remaining route-state filesystem ownership

Route-state should consume the seam after canonical artifacts and pipeline prove it out.

Why fourth:

- route-state has the widest remaining directory traversal surface
- it is easier to separate “true workspace concerns” from route-state-specific behavior after the earlier migrations

Output:

- overlapping repo-relative read/traversal rules move under the workspace seam
- route-state keeps only state-domain behavior that does not belong in the workspace module

### Phase 5: Stabilize exports and docs

After all migrations:

- decide whether the seam should be exported publicly now or remain compiler-internal
- refresh docs/spec references if implementation naming or scope changed

## Risks And Mitigations

### Risk: the new module remains shallow

If the seam only renames helpers without shrinking caller knowledge, the refactor does not deepen the codebase.

Mitigation:

- require migrated callers to consume typed workspace operations
- explicitly remove duplicate validators and no-follow readers from migrated modules

### Risk: path semantics drift

The repo has contract-owned rules around `.handbook/`, `core/**`, and repo-relative safety.

Mitigation:

- keep normalization and validation behavior byte-for-byte compatible where possible
- run targeted pipeline and canonical artifact tests after each migration

### Risk: directory traversal scope balloons

`route_state.rs` contains both workspace-facing traversal and route-state-specific behavior.

Mitigation:

- move only the traversal rules that overlap the workspace seam
- keep state-domain behavior in `route_state.rs`

### Risk: premature public API hardening

The eventual `substrate` consumer matters, but a public export too early could freeze a weak interface.

Mitigation:

- land the deep implementation seam first
- choose public export posture only after all migrated callers read cleanly

## Parallel Vs Sequential

Sequential:

- workspace seam before any caller migration
- canonical artifact and pipeline migrations before route-state cleanup
- export decision after all migrations

Potentially parallel after Phase 1:

- canonical artifact and pipeline migrations can proceed in separate packets if the workspace seam stabilizes early
- test additions can land alongside the corresponding caller migration

## Verification Checkpoints

### Checkpoint 1: Workspace seam core

- `cargo test -p handbook-compiler repo_file_access`

### Checkpoint 2: Canonical artifact migration

- `cargo test -p handbook-compiler canonical_artifacts`
- `cargo test -p handbook-compiler --test resolver_core`

### Checkpoint 3: Pipeline migration

- `cargo test -p handbook-compiler --test pipeline_loader`
- `cargo test -p handbook-compiler --test pipeline_catalog`
- `cargo test -p handbook-cli --test cli_surface`

### Checkpoint 4: Route-state migration

- `cargo test -p handbook-compiler --test pipeline_state_store`
- `cargo test -p handbook-compiler --test pipeline_route_resolution`

### Final checkpoint

- `cargo fmt --all -- --check`
- `cargo clippy --workspace --all-targets --all-features -- -D warnings`
- `cargo test --workspace`
- `cargo check --workspace`

## Exit Conditions

The plan is complete when:

- one deep workspace seam owns repo-relative validation, normalized path handling, trusted reads, trusted writes, and canonical `.handbook/` discovery primitives
- migrated callers read as adapters over that seam instead of secondary workspace owners
- tests prove no public reduced-v1 behavior drift
- the compiler is in a better position for future direct library consumption from `substrate`
