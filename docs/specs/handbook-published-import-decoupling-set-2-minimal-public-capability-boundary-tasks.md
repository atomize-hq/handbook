# Tasks: Handbook Published-Import Decoupling — Set 2: Minimal Public Capability Boundary for `handbook-pipeline`

Spec reference: [handbook-published-import-decoupling-set-2-minimal-public-capability-boundary-spec.md](./handbook-published-import-decoupling-set-2-minimal-public-capability-boundary-spec.md)
Plan reference: [handbook-published-import-decoupling-set-2-minimal-public-capability-boundary-plan.md](./handbook-published-import-decoupling-set-2-minimal-public-capability-boundary-plan.md)

## Active authority route for this seam

Use the following order during Set 2 execution:

1. `docs/specs/MAP.md` for exact objective, exact intent, success criteria, and set sequencing
2. the active Set 1 triplet for the required boundary principle, bounded candidate proof surface, and Set 2 acceptance wall
3. this active Set 2 triplet for implementation scope, retained/dropped matrix, packet order, and closeout conditions
4. `HANDBOOK_PUBLISHED_IMPORT_DECOUPLING_AUDIT_2026-06-23.md` for current-state evidence only
5. `docs/specs/archive/` for provenance only
6. the `9b83` Substrate docs for stale, non-canonical context only

If those sources disagree, the MAP plus the Set 1 triplet plus this Set 2 triplet control active planning.

---

## Packet 2.1: Public Contract Owners

- [ ] Task: Promote the reviewed declarative-roots and storage-layout contract owners to public boundary types
  - Acceptance: `PipelineDeclarativeRootsContract` and `PipelineStorageLayoutContract` are public, downstream can construct them through validated public APIs, and their stable read accessors cover the retained capability surface from the Set 2 matrix.
  - Verify: Source inspection of `crates/pipeline/src/declarative_roots.rs`, `crates/pipeline/src/layout.rs`, and `crates/pipeline/src/lib.rs`; targeted test or inline validation coverage for invalid path/layout rejection.
  - Files: `crates/pipeline/src/declarative_roots.rs`, `crates/pipeline/src/layout.rs`, `crates/pipeline/src/lib.rs`

- [ ] Task: Keep nested helper structs and repo-layout plumbing private
  - Acceptance: `RuntimeStateLayoutContract`, `CaptureStorageLayoutContract`, `HandoffBundleLayoutContract`, and `RepoLayoutRoot` remain private, and Set 2 does not make `handbook_pipeline::declarative_roots` or `handbook_pipeline::layout` public modules.
  - Verify: `rg -n "pub struct RuntimeStateLayoutContract|pub struct CaptureStorageLayoutContract|pub struct HandoffBundleLayoutContract|pub struct RepoLayoutRoot|pub mod declarative_roots|pub mod layout" crates/pipeline/src`
  - Files: `crates/pipeline/src/layout.rs`, `crates/pipeline/src/lib.rs`

---

## Packet 2.2a: Retained Declarative-Root Façade Landing

- [ ] Task: Expose only the retained declarative-root-aware entrypoints as additive public façade seams
  - Acceptance: The landed public boundary includes retained contract-aware variants for metadata browse, selector resolution, direct definition load, and selected definition load; existing default entrypoints stay behaviorally stable; this packet does **not** yet privatize `SupportedTargetRegistry::load` or route-aware `load_pipeline_catalog`.
  - Verify: Source inspection of `crates/pipeline/src/pipeline.rs`; `rg -n "load_pipeline_catalog_metadata|load_pipeline_selection_metadata|load_pipeline_definition|load_selected_pipeline_definition|load_with_roots|pub fn" crates/pipeline/src/pipeline.rs`
  - Files: `crates/pipeline/src/pipeline.rs`, `crates/pipeline/src/lib.rs`

- [ ] Task: Prove custom declarative roots through package-local tests only via the retained public façade
  - Acceptance: Tests added or updated in this packet show non-default pipeline/profile/runner/stage roots work for metadata browse, selector resolution, and definition load without importing private module paths.
  - Verify: `cargo test -p handbook-pipeline --test pipeline_catalog`; `cargo test -p handbook-pipeline --test pipeline_loader`
  - Files: `crates/pipeline/tests/pipeline_catalog.rs`, `crates/pipeline/tests/pipeline_loader.rs`, optionally `crates/pipeline/tests/support/*`

---

## Packet 2.2b: Caller/Test Migration Off Dropped Seams

- [ ] Task: Migrate in-repo dropped-seam callers to retained/public declarative-root APIs
  - Acceptance: Known in-repo callers no longer require `SupportedTargetRegistry::load` or route-aware `load_pipeline_catalog` to satisfy the declarative-root capability family, and the migration stays inside declarative-root-family work rather than widening into later packets.
  - Verify: `rg -n "SupportedTargetRegistry::load\\(|load_pipeline_catalog\\(" crates`; source inspection of touched caller files
  - Files: `crates/cli/src/pipeline.rs`, `crates/cli/src/pipeline_help.rs`, `crates/pipeline/src/pipeline_capture.rs`, `crates/pipeline/src/pipeline_compile.rs`, `crates/pipeline/src/pipeline_handoff.rs`, `crates/pipeline/src/stage_10_feature_spec_provenance.rs`, optionally tightly related caller/test files required for honest migration

- [ ] Task: Migrate package-local and adjacent in-repo proof off dropped seams
  - Acceptance: Declarative-root-family tests use retained/public APIs only and no longer rely on the dropped seams for coverage or setup.
  - Verify: `cargo test -p handbook-pipeline --test pipeline_catalog`; `cargo test -p handbook-pipeline --test pipeline_loader`; optionally targeted adjacent test commands for touched non-pipeline test crates
  - Files: `crates/pipeline/tests/pipeline_catalog.rs`, `crates/pipeline/tests/pipeline_loader.rs`, `crates/compiler/tests/pipeline_catalog.rs`, optionally tightly related test support files

---

## Packet 2.2c: Dropped-Seam Privacy Clamp

- [ ] Task: Make dropped declarative-root seams private only after the migration wall is satisfied
  - Acceptance: `SupportedTargetRegistry::load` and route-aware `load_pipeline_catalog` are no longer public, and no required in-repo caller still depends on them.
  - Verify: Source inspection of `crates/pipeline/src/pipeline.rs`; `rg -n "pub fn load_pipeline_catalog\\(|pub fn load\\(" crates/pipeline/src/pipeline.rs`; `rg -n "SupportedTargetRegistry::load\\(|load_pipeline_catalog\\(" crates`
  - Files: `crates/pipeline/src/pipeline.rs`, `crates/pipeline/src/lib.rs`, plus the migrated caller/test files carried forward from Packet 2.2b only if narrowly required to finish the clamp

- [ ] Task: Finish the public-API-only proof wall for the declarative-root family
  - Acceptance: The declarative-root-family proof now runs through retained/public entrypoints only, with dropped seams private and no private module imports required.
  - Verify: `cargo test -p handbook-pipeline --test pipeline_catalog`; `cargo test -p handbook-pipeline --test pipeline_loader`; `cargo check --workspace`
  - Files: `crates/pipeline/tests/pipeline_catalog.rs`, `crates/pipeline/tests/pipeline_loader.rs`, optionally tightly related test support files

---

## Packet 2.3: Route-State Storage-Layout Public Façade

- [ ] Task: Expose only the retained storage-layout-aware route-state entrypoints
  - Acceptance: The landed public boundary includes retained contract-aware variants for `load_route_state`, `set_route_state`, `load_trusted_pipeline_session`, and `persist_route_basis`, and no extra repo-layout or convenience helper surface becomes public.
  - Verify: Source inspection of `crates/pipeline/src/route_state.rs`; `rg -n "load_route_state_with_storage_layout|set_route_state_with_storage_layout|load_trusted_pipeline_session_with_storage_layout|persist_route_basis_with_storage_layout|pub fn" crates/pipeline/src/route_state.rs`
  - Files: `crates/pipeline/src/route_state.rs`, `crates/pipeline/src/lib.rs`

- [ ] Task: Prove custom storage-layout route-state behavior through package-local tests only via public APIs
  - Acceptance: Tests show non-default state roots work for route-state read/write, trusted-session loading, and route-basis persistence without private-module reach-in.
  - Verify: `cargo test -p handbook-pipeline --test pipeline_state_store`; `cargo test -p handbook-pipeline --test pipeline_route_resolution`
  - Files: `crates/pipeline/tests/pipeline_state_store.rs`, `crates/pipeline/tests/pipeline_route_resolution.rs`, optionally `crates/pipeline/tests/support/*`

---

## Packet 2.4: Capture + Handoff Storage-Layout Public Façade

- [ ] Task: Expose only the retained capture and handoff storage-layout-aware entrypoints
  - Acceptance: The landed public boundary includes retained contract-aware variants for `preview_pipeline_capture`, `apply_pipeline_capture`, `emit_pipeline_handoff_bundle`, and `validate_pipeline_handoff_bundle`; `capture_pipeline_output_with_storage_layout` and `load_pipeline_capture_cache_entry_with_storage_layout` stay private unless the Set 2 matrix is reopened first.
  - Verify: Source inspection of `crates/pipeline/src/pipeline_capture.rs` and `crates/pipeline/src/pipeline_handoff.rs`; `rg -n "preview_pipeline_capture_with_storage_layout|apply_pipeline_capture_with_storage_layout|emit_pipeline_handoff_bundle_with_storage_layout|validate_pipeline_handoff_bundle_with_storage_layout|capture_pipeline_output_with_storage_layout|load_pipeline_capture_cache_entry_with_storage_layout|pub fn" crates/pipeline/src/pipeline_capture.rs crates/pipeline/src/pipeline_handoff.rs`
  - Files: `crates/pipeline/src/pipeline_capture.rs`, `crates/pipeline/src/pipeline_handoff.rs`, `crates/pipeline/src/lib.rs`

- [ ] Task: Prove custom storage-layout capture and handoff behavior through package-local tests only via public APIs
  - Acceptance: Tests show non-default capture and handoff roots work for retained preview/apply and emit/validate behavior, while dropped/private convenience seams remain private.
  - Verify: `cargo test -p handbook-pipeline --test pipeline_capture`; `cargo test -p handbook-pipeline --test pipeline_handoff`
  - Files: `crates/pipeline/tests/pipeline_capture.rs`, `crates/pipeline/tests/pipeline_handoff.rs`, optionally `crates/pipeline/tests/support/*`

---

## Packet 2.5: Release-Candidate External Proof + Closeout

- [ ] Task: Add a release-candidate external consumer proof that exercises every retained capability family through public APIs only
  - Acceptance: A packaged external consumer constructs non-default declarative-roots and storage-layout contracts, exercises retained metadata/definition, route-state, capture, and handoff capability families, and does so without private module imports, sibling-path accidents, or direct source-tree reach-in.
  - Verify: `cargo package -p handbook-pipeline --allow-dirty`; `cargo publish --dry-run -p handbook-pipeline`; `bash tools/proof/handbook_pipeline_minimal_boundary.sh`
  - Files: `tools/proof/handbook_pipeline_minimal_boundary.sh`, `tests/fixtures/external_consumers/handbook_pipeline_minimal_boundary/Cargo.toml`, `tests/fixtures/external_consumers/handbook_pipeline_minimal_boundary/src/main.rs`, optionally a small fixture README or helper file under the same proof directory

- [ ] Task: Close Set 2 honestly and preserve the Set 3 handoff boundary
  - Acceptance: Set 2 closeout notes confirm the landed public boundary still matches the retained/dropped matrix, the release-candidate external proof passed, Packet 4.2 remains only `engine + flow` proof, and downstream Substrate source-touching proof plus released-crate proof still belong to Set 3.
  - Verify: `cargo check --workspace`; source inspection of the Set 2 triplet after implementation; `rg -n "Packet 4.2|Set 3|released|downstream|retained/dropped|public boundary" docs/specs/handbook-published-import-decoupling-set-2-minimal-public-capability-boundary-*.md`
  - Files: `docs/specs/handbook-published-import-decoupling-set-2-minimal-public-capability-boundary-spec.md`, `docs/specs/handbook-published-import-decoupling-set-2-minimal-public-capability-boundary-plan.md`, `docs/specs/handbook-published-import-decoupling-set-2-minimal-public-capability-boundary-tasks.md`

---

## Set-Level Guardrail

Stop after Set 2 lands the reviewed minimal public capability boundary and the release-candidate external proof wall. Do not:

- make `declarative_roots` or `layout` public wholesale
- widen beyond the retained/dropped matrix without reopening authority first
- expose handbook product-shell wording, CLI-only behavior, or repo-layout plumbing as public API
- claim Set 3 proof is already complete
- start downstream Substrate source-touching proof inside this set

## Set-Level Completion Standard

Set 2 is complete only when:

- the public API still matches the retained/dropped matrix from the Set 2 spec,
- downstream can construct non-default contract owners through public APIs,
- retained capability families are exercised through public façade entrypoints only,
- the Packet 2.2a / 2.2b / 2.2c sequence completed in order without leaving required dropped-seam callers behind,
- dropped/private seams stayed private,
- the release-candidate external proof passed,
- and the closeout notes hand off released-crate proof, downstream Substrate proof, and guard rails to Set 3 explicitly.
