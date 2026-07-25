# Changelog

All notable changes to this project are documented in this file.

## Unreleased

### Changed
- Advanced `handbook-engine` to `0.2.0` and updated workspace dependency constraints after HCM-2.3 added the public `EstablishedRefusalCodeV1::PublicationBasisConflict` and `GenericRefusalLayerV1::Publication` variants.
- Documented the exhaustive-match and wire-value migration requirements in `docs/migrations/handbook-engine-0.2.0.md`. No tag, package publication, or remote push is part of this compatibility boundary.

## [0.7.1.0] - 2026-04-29

### Added
- Added `handbook doctor --json` as the machine-readable readiness surface for the installed charter-intake runtime, with serialized blocker, checklist, and next-safe-action contracts locked by compiler and CLI tests.
- Added the Codex packaging/install surface under `tools/codex/`, including generated `.agents/skills/` assets, install and dev-setup flows, and install/live-smoke coverage for the shipped charter-intake path.
- Added `handbook author charter --validate --from-inputs <path|->` as a mutation-free preflight that validates structured inputs and repo write preconditions without touching canonical truth.

### Changed
- Changed `handbook author charter --from-inputs <path|->` to a compiler-owned deterministic write path, while keeping guided `handbook author charter` as the Codex-backed interview surface.
- Updated docs, contracts, help snapshots, and smoke rails so the shipped runtime story matches the installed `handbook` and `handbook-charter-intake` skill surfaces and the deterministic charter-authoring contract.

### Fixed
- Fixed charter authoring so malformed structured-input refusals win before existing-truth checks on the deterministic path, preserving the real input error instead of masking it behind repo state.
- Fixed packaging smoke coverage with a valid runtime charter fixture and end-to-end checks for reinstall safety, stale-runtime refusal, repo-local runtime override, and outside-repo refusal behavior.

## [0.7.0.0] - 2026-04-22

### Added
- Added the shipped M8 baseline-authoring surfaces `handbook author project-context` and `handbook author environment-inventory`, including guided and structured-input project-context flows, environment-inventory synthesis, canonical-write safety, and validation/refusal coverage for invalid or already-authored truth.
- Added a compiler-owned baseline validation and doctor model that classifies `SCAFFOLDED`, `PARTIAL_BASELINE`, `INVALID_BASELINE`, and `BASELINE_COMPLETE`, with ordered checklist output and concrete next-safe-action commands for each baseline artifact.

### Changed
- Cut the setup/bootstrap contract over to the M8 baseline tier so setup now scaffolds only the real baseline set, hands operators to `handbook doctor`, and no longer treats `FEATURE_SPEC` as part of baseline readiness.
- Split compiler authoring into explicit `charter`, `project_context`, and `environment_inventory` modules, and made the canonical artifact registry the single source of truth for baseline participation, setup scaffolding, and downstream rendering semantics.
- Realigned README, supported-command docs, contracts, glossary, tone/output guides, fixtures, and help snapshots around the new baseline truth story where `.handbook/environment_inventory/ENVIRONMENT_INVENTORY.md` is the sole canonical environment-inventory path.

### Fixed
- Fixed M8 canonical-truth regressions so empty baseline files classify as `INVALID_BASELINE`, doctor repair guidance points at the concrete authoring commands, and invalid baseline truth no longer slips through as a scaffolded or partial state.
- Fixed environment-inventory authority drift by rejecting legacy root-canonical claims in templates, directives, synthesis output, proof corpus fixtures, and product-surface docs.

## [0.6.0.0] - 2026-04-20

### Added
- Added the shipped M7 top-level `author` family with `handbook author charter` and `handbook author charter --from-inputs <path|->` as the public charter-authoring surfaces for scaffolded repos.
- Added a change-scoped live Codex authoring smoke in CI so transport or prompt-contract drift on the shipped charter-authoring path gets caught before merge.

### Changed
- Cut the setup/readiness story over to authored charter truth so scaffolded setup routes operators to `handbook author charter`, and successful charter authoring is what clears `doctor` and planning `generate`.
- Wired both M7 charter-authoring paths through one shared final synthesis engine in the compiler seam using direct `codex exec`, with the repo-owned authoring method, synthesis directive, and charter template now driving runtime behavior.
- Realigned CLI help, docs, contracts, fixtures, and local smoke commands around the new authoring surface and the runtime model-selection contract used by shipped charter synthesis.

### Fixed
- Fixed the shipped M7 authoring runtime so it no longer bypasses the final synthesis pass with deterministic direct rendering, and now refuses invalid synthesis output before mutating canonical charter truth.
- Fixed the local and CI authoring smoke rails so they watch all repo-owned prompt inputs that affect charter synthesis and pass the selected model through the runtime variable the compiler actually reads.

## [0.5.2.0] - 2026-04-18

### Added
- Added the shipped M6 setup family with `handbook setup`, `handbook setup init`, and `handbook setup refresh`, including safe canonical `.handbook/` scaffold creation, preserve-by-default refresh behavior, explicit `--rewrite` and `--reset-state` flags, and routed success/refusal output for new and initialized repos.
- Added compiler and CLI regression coverage for setup-family help, first-run bootstrap, refresh preservation and rewrite behavior, runtime-state reset behavior, nested-repo routing, and the new starter-template readiness boundary.

### Changed
- Realigned README, START_HERE, supported-command docs, CLI hierarchy/output/vocabulary docs, contracts, and the active M6 plan so the shipped product story now has one honest front door instead of the old placeholder-versus-guided-setup split.
- Updated canonical-artifact and blocker handling so shipped starter templates are explicit setup-owned scaffolds, required-starter-template blockers have their own category, and recovery guidance consistently routes missing or invalid canonical truth back through the setup family.

### Fixed
- Fixed setup recovery for invalid `.handbook` roots and unsafe `.handbook/state` reset targets so file-backed roots, symlinked roots, and reset failures repair or refuse without partial mutation.
- Fixed planning readiness so `doctor` and `generate` no longer treat shipped starter templates as completed canonical truth, and removed the repo-root setup scaffolds that previously masked that gap.

## [0.5.1.0] - 2026-04-17

### Changed
- Refreshed the top-level README so it now points straight at the shipped Rust CLI surface, the real docs entrypoints, and the current reduced-v1 command boundaries instead of the old scaffold-first story.
- Updated the docs index to highlight the supported command surface, release notes, backlog, and shipped downstream handoff-emission wedge.
- Re-scoped the glossary as legacy-harness terminology and cleaned the broad vision doc so the long-range docs stop mixing product guidance with chat residue.

## [0.5.0.1] - 2026-04-16

### Fixed
- Fixed the cross-platform install smoke script so fixture-output checks no longer fail on macOS when `grep -q` exits early under `pipefail`, keeping the post-install execution-demo proof stable on `main`.

## [0.5.0.0] - 2026-04-16

### Added
- Added a realistic `foundation_flow_demo` proof corpus with committed happy-path and skip-path evidence transcripts, expected artifacts, stage model outputs, and demo repo inputs for the M4 journey proof.
- Added CLI journey coverage for the full `pipeline.foundation_inputs` happy and skip paths, including deterministic rerun evidence and a structural `FEATURE_SPEC.md` contract checker tied to the shipped directive and template.

### Changed
- Solidified `PLAN.md` around the actual M4 stopping point: one believable operator journey, one truthful stage-10 handoff contract, deterministic evidence, and no premature downstream-adoption claims.
- Updated README, START_HERE, supported-command docs, operator-journey guidance, and pipeline contracts so stage `10` is consistently documented as `compile -> external model output -> capture`.

### Fixed
- Hardened `pipeline capture` for `stage.10_feature_spec` so raw compile payload is refused as `invalid_capture_input` and only a completed `FEATURE_SPEC.md` body can be materialized.

## [0.4.1.0] - 2026-04-15

### Added
- Added the remaining shipped M3.5 `pipeline capture` targets for `pipeline.foundation_inputs`: `stage.04_charter_inputs`, `stage.06_project_context_interview`, and `stage.10_feature_spec`.
- Added shared proof-corpus goldens plus compiler and CLI regression coverage for preview/apply success, wrapper refusal, empty-body refusal, route progression, and the real stage-10 compile-to-capture handoff.

### Changed
- Tightened the capture operator contract so help text, README/start-here guidance, supported-command docs, and pipeline contracts all describe one exact `foundation_inputs` path with the manual `needs_project_context` handoff and the narrow single-writer safety claim.
- Updated `pipeline capture` wording and refusal messaging to describe the shipped supported stage set instead of the earlier milestone-limited boundary.

## [0.4.0.0] - 2026-04-15

### Added
- Added the shipped M3 `pipeline capture` wedge for `pipeline.foundation_inputs`, including direct apply from stdin, `--preview` with deterministic `capture_id`, and cached `pipeline capture apply --capture-id <capture-id>` for the charter-synthesize and foundation-pack stages.
- Added compiler-owned capture planning, typed preview/apply models, runtime preview-cache entries, repo-mirror materialization for `CHARTER.md` and `ENVIRONMENT_INVENTORY.md`, and deterministic post-capture route-state updates plus next-safe-action guidance.
- Added proof-corpus, compiler, and CLI regression coverage for capture preview/apply success paths and refusal paths, including stale route basis, inactive stages, missing capture ids, malformed FILE blocks, tampered cached plans, revision conflicts, symlinked write targets, and rollback behavior.

### Changed
- Hardened the capture apply path so cached previews are revalidated under lock before writes, writes stay transactional with rollback on later failure, and cache-path plus write-target handling refuses symlink races or tampered plans instead of trusting the filesystem.
- Realigned README, START_HERE, supported-command docs, CLI vocabulary/output anatomy, DESIGN guidance, and the new `C-12` contract around the shipped capture surface so docs, help text, snapshots, and compiler behavior all describe the same operator workflow.

## [0.3.0.0] - 2026-04-14

### Added
- Added the shipped M2 `pipeline compile` wedge for `pipeline.foundation_inputs` + `stage.10_feature_spec`, including payload-only success output, `--explain` proof mode, and compiler-owned route-basis persistence through `pipeline resolve`.
- Added bounded compile-time document loading and refusal coverage for missing route basis, stale or forged route snapshots, inactive stages, missing required artifacts or variables, and shared golden outputs reused by both compiler and CLI test suites.
- Added proof-corpus and CLI regression coverage for the new compile surface, including refusal goldens, help snapshots, and route-basis state seeds that lock the operator contract in place.

### Changed
- Tightened compile safety by rejecting incomplete profile packs, refusing symlinked compile inputs, honoring the selected stage work level when filtering scoped blocks, and restricting compile selection to the declared M2 stage boundary.
- Normalized compile-facing `repo_root` handling to the stable `${repo_root}` symbolic root and updated route-state contracts so persisted `route_basis` snapshots stay deterministic and auditable across resolve and compile runs.
- Realigned README, DESIGN, support docs, and pipeline contracts around the shipped M2 compile surface so docs, help text, snapshots, and proof-corpus expectations all describe the same supported workflow.

## [0.2.0.0] - 2026-04-13

### Added
- Added the supported reduced-v1 `pipeline` command family with `pipeline list`, `pipeline show`, `pipeline resolve`, and `pipeline state set` so operators can inspect pipeline inventory, compute route truth, and persist narrow routing state without manual repo archaeology.
- Added compiler-owned pipeline catalog loading, canonical ID discovery, deterministic route evaluation, typed route-state persistence, advisory locking, revision-conflict refusals, and shared foundation-family proof corpus goldens for `pipeline resolve` and `pipeline state set`.
- Added dedicated CLI and compiler coverage for pipeline inventory inspection, shorthand ambiguity, activation drift, malformed-state refusals, lock-aware mutation behavior, help snapshots, and proof-corpus rendering.

### Changed
- Tightened the M1 route/state contract so stage `sets`, activation clauses, persisted refs, and runner/profile mutations all validate against explicit reduced-v1 grammar and allowlists instead of drifting into loose stringly-typed behavior.
- Improved the operator proof surface with route-basis summaries, declared route metadata in `pipeline show`, clearer invalid-canonical-id and inventory-refusal behavior, and metadata-only inventory commands that ignore unrelated broken pipeline files during inspection.
- Realigned product docs, contracts, seam/governance artifacts, README guidance, and CLI support docs around the shipped M1 pipeline/routing spine and the shared proof corpus.

## [0.1.0.0] - 2026-04-11

### Added
- Added a compiler-owned Rust pipeline loader that reads the current two-document pipeline YAML shape and exposes typed pipeline definitions for the reduced-v1 foundation-family wedge.
- Added strict parser-gate coverage for duplicate stage ids, stage file validity, activation operators, extra YAML documents, and other refusal paths so unsupported shapes fail loudly instead of becoming compatibility debt.

### Changed
- Tightened reduced-v1 activation handling to boolean equality only, matching the narrow contract the current Rust proof corpus actually uses.
- Clarified glossary and legacy handbook-model docs so the supported Rust loader contract no longer implies broader legacy activation semantics than the compiler ships today.

## [0.0.1.0] - 2026-04-07

### Added
- Introduced the Rust workspace, `handbook` CLI, and `handbook-compiler` library as the reduced-v1 product path for planning packet generation, inspect proof surfaces, doctor recovery output, and explicit live-execution refusal.
- Added typed canonical artifact ingestion, manifest/freshness computation, resolver contracts, packet rendering surfaces, and fixture-backed execution demo support with broad Rust test coverage.
- Added CI rails for `rustfmt`, `clippy`, workspace tests, archive-boundary enforcement, and cross-platform install smoke checks on Linux and macOS.
- Added reduced-v1 contracts and Rust-first docs entrypoints covering approved repo surface, command surface, manifest/freshness truth, resolver/blocker behavior, renderer surfaces, fixture-demo boundaries, and conformance rails.

### Changed
- Cut the repo over to a Rust-first supported surface, including updated README guidance, supported-command docs, and archived snapshots of legacy generated artifacts for reference-only use.
- Hardened repo discovery, nested git boundary handling, packet budgeting, omission notes, and help-text drift guards to match shipped CLI behavior.

### Removed
- Removed committed legacy `dist/` generated outputs from the supported repo surface now that the archived snapshot preserves the old scaffold artifacts.
