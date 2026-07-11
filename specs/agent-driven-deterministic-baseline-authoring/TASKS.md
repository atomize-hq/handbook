# Todo: Agent-Driven Deterministic Baseline Authoring

Plan reference: [`plan.md`](./plan.md)

## Phase 0 — Preflight

- [x] **P0.1: Refresh GitNexus and record the impact ledger**
  - Depends on: none
  - Acceptance:
    - GitNexus index matches implementation `HEAD`.
    - Fresh upstream impact exists for every existing symbol to be edited.
    - HIGH/CRITICAL results are reported before edits.
  - Verify:
    - `npx gitnexus analyze`
    - `npx gitnexus status`
    - `npx gitnexus impact --repo system --direction upstream --include-tests <symbol>`
  - Files: no product files
  - Result (2026-07-10):
    - Refreshed the local index to `d6de19c`; `npx gitnexus status` reports up to date.
    - Used the checkout path to disambiguate the repository from another registered `system` index.
    - Fresh impact: `execute_author_charter_command` CRITICAL; `author_environment_inventory` HIGH; `synthesize_environment_inventory_markdown` HIGH; `author_charter_guided` MEDIUM; remaining required symbols LOW.
    - HIGH/CRITICAL results were reported before product edits. Symbol-specific impact remains mandatory immediately before each implementation edit.

- [x] **P0.2: Re-run and record the author/pipeline baseline wall**
  - Depends on: P0.1
  - Acceptance: author-core, compiler-author, CLI-author, pipeline catalog/compile/capture/handoff, and coupling checks match the green planning baseline.
  - Verify: commands in `plan.md` Phase 0
  - Files: verification only
  - Result (2026-07-10):
    - Author baselines passed: engine 4, compiler 59, CLI 25 tests.
    - Pipeline baselines passed: catalog 15, compile 21, capture 46, handoff 9 tests.
    - Engine, flow, and pipeline dependency/source-coupling checks passed.

## Phase 1 — Environment Inventory Deterministic Path

- [x] **P1.1: Add typed environment-inventory inputs and deterministic rendering**
  - Depends on: P0.2
  - Acceptance:
    - Typed YAML covers every existing canonical section.
    - Unknowns are explicit; secrets are never required as values.
    - Rendering is deterministic and passes canonical heading validation.
    - No new Handbook crate dependency is added to engine.
  - Verify:
    - `cargo test -p handbook-engine --test author_core`
    - `cargo test -p handbook-engine`
    - `cargo tree -p handbook-engine -e normal,dev`
  - Files:
    - `crates/engine/src/author/environment_inventory_core.rs`
    - `crates/engine/src/author/mod.rs`
    - `crates/engine/tests/author_core.rs`
    - `core/library/environment_inventory/ENVIRONMENT_INVENTORY_INPUTS.yaml.tmpl` (new)
  - Result (2026-07-10):
    - Added typed YAML for metadata, variables, secret handling, services, runtime, local development, CI, production, tooling, update contract, and known unknowns.
    - Added deterministic rendering with an explicit timestamp, canonical heading validation, placeholder/render-safety checks, and secret-value refusal.
    - Added four focused engine tests; the engine test suite and dependency tree pass with no new Handbook dependency edge.

- [x] **P1.2: Add compiler validation/write adapters for environment inputs**
  - Depends on: P1.1
  - Acceptance:
    - Compiler maps engine parse/validation errors into existing artifact refusals.
    - Validation is non-mutating.
    - Deterministic authoring preserves preflight, upstream-artifact checks, lock, safe repair, and canonical write behavior.
    - Old Codex path gains no new callers.
  - Verify:
    - `cargo test -p handbook-compiler --test author`
    - `cargo check -p handbook-compiler`
  - Files:
    - `crates/compiler/src/author/environment_inventory.rs`
    - `crates/compiler/src/author/environment_inventory_core.rs`
    - `crates/compiler/src/author/environment_inventory_shell.rs`
    - `crates/compiler/src/author/mod.rs`
    - `crates/compiler/tests/author.rs`
  - Result (2026-07-10):
    - Added compiler parse/validate/render adapters and structured-input refusal mapping.
    - Added non-mutating input preflight and guarded deterministic authoring while retaining the old synthesis path without new callers.
    - Added malformed-input, non-mutation, deterministic-write, and safe-repair tests; all 63 compiler author tests and compiler check pass.

- [x] **P1.3: Cut environment CLI to `--from-inputs` and `--validate`**
  - Depends on: P1.2
  - Acceptance:
    - File and stdin authoring succeed deterministically.
    - Validation mode performs no mutation.
    - Bare command fails immediately without launching a process.
    - Help grammar matches charter/project-context target shape.
  - Verify:
    - `cargo test -p handbook-cli --test author_cli`
    - `cargo test -p handbook-cli`
  - Files:
    - `crates/cli/src/main.rs`
    - `crates/cli/src/author.rs`
    - `crates/cli/tests/author_cli.rs`
    - `crates/cli/tests/snapshots/handbook-author-environment-inventory-help.txt`
  - Result (2026-07-10):
    - Added required file/stdin `--from-inputs` handling and non-mutating `--validate` behavior.
    - Bare authoring now refuses before preflight or process launch; deterministic file/stdin authoring and safe repair do not create prompt captures.
    - Updated help grammar and refusal mapping; 27 author CLI tests and the full CLI crate suite pass.

## Phase 2 — Remove Guided Existing Paths

- [x] **P2.1: Make project-context authoring deterministic-only with validation parity**
  - Depends on: P0.2
  - Acceptance:
    - `--from-inputs` is required.
    - `--validate` works for file and stdin without mutation.
    - No TTY/default/question path remains active.
    - Existing canonical output and repair/refusal policy remain unchanged.
  - Verify:
    - `cargo test -p handbook-cli --test author_cli project_context`
    - `cargo test -p handbook-compiler --test author project_context`
  - Files:
    - `crates/cli/src/main.rs`
    - `crates/cli/src/author.rs`
    - `crates/cli/tests/author_cli.rs`
    - `crates/cli/tests/snapshots/handbook-author-project-context-help.txt`
  - Result (2026-07-10):
    - Bare project-context authoring now immediately requires `--from-inputs`; no TTY/default/question branch is reachable.
    - Added file/stdin validation-only parity with zero canonical mutation and updated help output.
    - Targeted CLI and compiler project-context tests pass; obsolete PTY success tests are quarantined for deletion in P3.3.

- [x] **P2.2: Make charter authoring deterministic-only**
  - Depends on: P0.2
  - Acceptance:
    - `--from-inputs` is required for mutation.
    - Existing validation, deterministic render, file/stdin, lock, and repair behavior remain green.
    - Guided callbacks, TTY checks, and guided output mode leave the active CLI path.
  - Verify:
    - `cargo test -p handbook-cli --test author_cli`
    - `cargo test -p handbook-cli`
  - Files:
    - `crates/cli/src/main.rs`
    - `crates/cli/src/author.rs`
    - `crates/cli/tests/author_cli.rs`
    - `crates/cli/tests/snapshots/handbook-author-charter-help.txt`
  - Result (2026-07-10):
    - Bare charter authoring now immediately requires `--from-inputs`; the active dispatcher no longer checks TTY state or accepts guided callbacks.
    - Existing deterministic file/stdin, validation, render, lock, and repair behavior remains green.
    - The full CLI suite passes; obsolete PTY success tests are quarantined for deletion with prompting infrastructure in P3.3.

## Phase 3 — Delete Obsolete Leaves

- [x] **P3.1: Delete charter Codex synthesis and its tests**
  - Depends on: P2.2
  - Acceptance: no charter compiler API launches Codex; deterministic authoring and template-library consumers remain intact.
  - Verify:
    - `cargo test -p handbook-compiler --test author`
    - `cargo test -p handbook-cli --test author_cli`
  - Files:
    - `crates/compiler/src/author/charter.rs`
    - `crates/compiler/src/author/charter_shell.rs`
    - `crates/compiler/src/author/mod.rs`
    - `crates/compiler/tests/author.rs`
  - Result (2026-07-10):
    - Removed the public guided charter API, Codex process launch, prompt construction, output transport, model/bin configuration, and synthesis-only validation.
    - Removed twelve guided/model/auth/transport tests while retaining deterministic authoring and template-library coverage.
    - Compiler author tests (51) and CLI author tests pass; no charter synthesis entry point remains.

- [x] **P3.2: Delete environment Codex synthesis and its tests**
  - Depends on: P1.3
  - Acceptance: no environment compiler API launches Codex; deterministic preflight/write and upstream-artifact rules remain intact.
  - Verify:
    - `cargo test -p handbook-engine --test author_core`
    - `cargo test -p handbook-compiler --test author`
    - `cargo test -p handbook-cli --test author_cli`
  - Files:
    - `crates/compiler/src/author/environment_inventory.rs`
    - `crates/compiler/src/author/environment_inventory_shell.rs`
    - `crates/compiler/src/author/mod.rs`
    - `crates/compiler/tests/author.rs`
  - Result (2026-07-10):
    - Removed the no-input environment author API, Codex process/model configuration, prompt/template transport, output-file handling, and shared process-summary helpers.
    - Retained deterministic input rendering, preflight, lock/write behavior, charter requirements, optional project-context validation, existing-truth refusal, and safe repair tests.
    - Engine author-core, compiler author (45), and CLI author tests pass; compiler check is warning-free.

- [x] **P3.3: Delete CLI prompting, PTY, and guided test leaves**
  - Depends on: P2.1, P2.2, P3.1
  - Acceptance:
    - `author_prompting.rs` is removed.
    - No guided collector/default/parser remains solely for interviews.
    - PTY dependency/harnesses and guided tests are removed.
    - Declarative pipeline/stage truth is untouched.
  - Verify:
    - `cargo test -p handbook-cli`
    - `! rg -n 'author_prompting|interactive_authoring_is_allowed|guided_interview' crates/cli crates/compiler crates/engine`
  - Files:
    - `crates/cli/src/author_prompting.rs` (delete)
    - `crates/cli/src/author.rs`
    - `crates/cli/src/main.rs`
    - `crates/cli/tests/author_cli.rs`
    - `crates/cli/Cargo.toml`
  - Result (2026-07-10):
    - Removed the CLI prompting module, guided collectors/defaults/parsers, TTY detection, PTY harnesses, guided fixtures, and obsolete live-Codex claim.
    - Removed `portable-pty` and the remaining CLI stub-Codex transport fixtures while preserving deterministic file/stdin authoring coverage.
    - The full CLI suite passes (2 unit, 22 author, 119 surface, and supporting integration tests), and the bounded guided-surface search is empty.

- [x] **P3.4: Remove live author inference CI/configuration**
  - Depends on: P3.1, P3.2, P3.3
  - Acceptance: CI and local checks require no model/API key for authoring; deterministic skill/install smoke remains.
  - Verify:
    - bounded `rg` command in `plan.md` Phase 3
  - Files:
    - `tools/ci/author-live-smoke.sh` (delete)
    - `.github/workflows/ci.yml`
    - `justfile`
  - Result (2026-07-10):
    - Removed the live author inference workflow job and smoke script, including API-key and model configuration.
    - Converted the local smoke check to the deterministic workspace test wall and retained the cross-platform install-smoke job.
    - Removed the final obsolete compiler test transport overrides; the bounded author-inference configuration search is empty and compiler author tests pass (45).

## Phase 4 — Shell, Recovery, And Skill Contract

- [x] **P4.1: Align help, output, doctor, and blocker recovery guidance**
  - Depends on: P1.3, P2.1, P2.2
  - Acceptance:
    - Author help says deterministic/agent-facing, not human-guided.
    - Every recovery action points to the skill or an executable `--from-inputs` form.
    - Baseline verdicts and canonical validity do not change.
    - Final shell presentation remains CLI-owned.
  - Verify:
    - `cargo test -p handbook-cli --test cli_surface`
    - `cargo test -p handbook-cli --test author_cli`
    - `cargo test -p handbook-compiler --test doctor`
  - Files:
    - `crates/cli/src/main.rs`
    - `crates/cli/src/doctor_rendering.rs` if presentation changes require it
    - `crates/compiler/src/doctor_shell.rs`
    - `crates/compiler/src/blocker.rs` only if typed next-action construction requires it
    - `crates/cli/tests/cli_surface.rs`
    - `crates/cli/tests/snapshots/handbook-author-help.txt`
  - Result (2026-07-10):
    - Author help now describes deterministic agent-facing authoring, and all three subcommand snapshots enforce the shared file/stdin plus validation grammar.
    - Doctor checklist, blocker rendering, and author refusal recovery now point to executable `--from-inputs <path|->` commands; obsolete guided-answer wording is gone.
    - CLI surface (123), CLI author (22), and compiler doctor (10) tests pass without changing baseline verdict or canonical-validity behavior.

- [x] **P4.2: Package all deterministic input resources through existing skills**
  - Depends on: P1.1, P1.3, P2.1, P2.2
  - Acceptance:
    - No new skill identity is added.
    - Root `handbook` skill explains all-three baseline authoring.
    - Charter leaf remains deterministic and charter-specific.
    - Installed home includes all required input templates.
    - Human-guided charter method wording is removed.
  - Verify:
    - `bash tools/codex/generate.sh --output-root "$(mktemp -d)"`
    - `bash tools/ci/install-smoke.sh`
  - Files:
    - `install/handbook-home/SKILL.md.tmpl`
    - `install/handbook-home/charter-intake/SKILL.md.tmpl`
    - `core/library/authoring/charter_authoring_method.md`
    - `tools/codex/install.sh`
    - `tools/ci/install-smoke.sh`
    - deterministic input templates/resources required by the installed home
  - Result (2026-07-10):
    - Expanded the existing root `handbook` skill into the all-three deterministic baseline workflow while keeping `handbook-charter-intake` charter-specific.
    - Replaced human-interview/synthesis method guidance with normalized-input validation and deterministic rendering rules.
    - Installed and exact-file-set tested charter, project-context, and environment-inventory templates; generation and the full install smoke pass without adding a skill identity.

- [x] **P4.3: Prove installed all-three baseline authoring without Codex**
  - Depends on: P4.2, P3.4
  - Acceptance:
    - Clean installed runtime authors all three artifacts from file/stdin inputs.
    - `handbook doctor --json` reports `baseline_complete`.
    - No Codex binary, API key, model, PTY, or network inference is required.
  - Verify:
    - `bash tools/ci/install-smoke.sh`
    - `bash tools/ci/codex-skill-live-smoke.sh`
  - Files:
    - `tools/ci/codex-skill-live-smoke.sh`
    - deterministic fixture inputs under `tools/fixtures/**`
    - `tools/ci/install-smoke.sh` only if the exact installed file-set assertion changes
  - Result (2026-07-10):
    - Added valid project-context and environment-inventory runtime-smoke fixtures alongside the existing charter fixture.
    - Extended the installed-skill smoke to setup, validate, and author all three artifacts using file and stdin inputs with credentials unset and an empty executable search path.
    - Installed `handbook doctor --json` reaches `baseline_complete` with no blockers; install and skill smoke scripts both pass without Codex, PTY, or network inference.

## Phase 5 — Closeout

- [x] **P5.1: Run the full functional verification wall**
  - Depends on: P3.4, P4.1, P4.3
  - Acceptance: fmt, clippy, workspace tests/check, archive checks, install/skill smoke, and diff checks pass.
  - Verify: `plan.md` Phase 5 functional wall
  - Files: verification only; fixes remain scoped to the failing packet
  - Result (2026-07-10):
    - Formatting, warning-denied clippy, all workspace tests, workspace check, archive boundary checks, install smoke, offline all-three skill smoke, and diff checks all pass.
    - No verification-only fix was required.

- [x] **P5.2: Run Phase 6 coupling and pipeline no-regression proof**
  - Depends on: P5.1
  - Acceptance:
    - Frozen dependency direction is unchanged.
    - Pipeline retains zero compiler/CLI dependency.
    - Pipeline catalog/compile/capture/handoff remain green.
    - No supported target or declarative stage changed.
  - Verify: `plan.md` Phase 5 coupling/pipeline wall
  - Files: verification only
  - Result (2026-07-10):
    - Dependency-tree and source searches confirm no compiler/CLI back-edge in pipeline, flow, or engine.
    - Pipeline catalog (15), compile (21), capture (46), and handoff (9) suites pass.
    - No supported target, declarative stage, pairing, or public importer boundary changed.

- [x] **P5.3: Run GitNexus detect-changes and stop at human review**
  - Depends on: P5.2
  - Acceptance:
    - Affected flows are limited to authoring, CLI presentation, skills/install, tests, and obsolete inference CI.
    - No pipeline execution, flow, or contract-membrane implementation drift appears.
    - Human review gate is reached with no implementation beyond this plan.
  - Verify:
    - `npx gitnexus detect-changes --repo system`
    - `git diff --check`
    - `git status --short`
  - Files: verification only
  - Result (2026-07-10):
    - Full branch comparison reports the expected critical authoring/presentation blast radius (41 files, 194 symbols, 30 flows); named flows are authoring and shared recovery presentation only.
    - Changed-path review confirms no pipeline/flow crate, declarative pipeline/stage, supported target, or contract-membrane surface changed.
    - Branch diff checks pass, the worktree is clean after closeout, and the implementation stops at the human review gate.
