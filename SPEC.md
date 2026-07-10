# Spec: Agent-Driven Deterministic Baseline Authoring

## Status

- Phase: Specify
- Scope: authoring-surface cleanup only
- Implementation: not started
- Approval gate: this specification must be confirmed before planning or code changes

## Assumptions

1. Charter, project context, and environment inventory remain supported, required baseline artifacts.
2. All three artifacts use the same agent-facing model: an agent gathers facts, writes normalized structured inputs, and invokes the Handbook CLI to validate and author the canonical Markdown.
3. The Handbook CLI must not launch Codex or any other model runtime.
4. The Handbook CLI must not conduct TTY interviews or collect answers one question at a time.
5. Existing Handbook skill packaging remains the agent orchestration/discovery layer. This cleanup must not add one skill leaf per artifact.
6. `tools/codex/**` and `install/handbook-home/**` remain in scope as skill packaging and discovery infrastructure despite their names; they are not the unwanted embedded model runtime.
7. This cleanup targets the CLI-owned human-guided authoring path and the nested model transport that path introduced. It does not redesign declarative pipelines, the supported-target registry, or the frozen pipeline importer boundary.
8. Generic pipeline runner vocabulary such as `core/runners/codex-cli.md` is outside this cleanup unless implementation proves that a file exists only to support the removed CLI authoring wrapper and removing it does not alter the pipeline contract.
9. Phase 5 CLI ownership and Phase 6 crate/import boundaries are preservation constraints, not historical behavior to reimplement elsewhere.
10. Historical files under `archived/**` remain historical provenance and are not rewritten to describe the new behavior.

## Objective

Remove the human-guided and CLI-inside-agent authoring paths before the repository grows additional features. Replace them with one consistent, deterministic authoring contract for all supported baseline artifacts.

The target user is an agent operating inside a managed repository. The agent—not a human interview loop and not a nested Codex process—derives project facts and supplies them to the CLI. The CLI owns schema validation, deterministic rendering, guarded canonical writes, refusal behavior, and final verification surfaces.

The cleanup succeeds when:

- no supported `handbook author` command prompts a human for answers;
- no Handbook authoring code starts `codex exec` or requires model credentials;
- charter, project context, and environment inventory share the same input, validation, and mutation shape;
- the existing agent skill can drive the retained CLI contract without nested agent execution; and
- obsolete transport, prompt, test, CI, dependency, and configuration leaves are removed rather than left dormant.

## User Contract

### Supported authoring commands

All baseline authoring subcommands require normalized structured input from a file or stdin:

```bash
handbook author charter --from-inputs ./charter-inputs.yaml
handbook author project-context --from-inputs ./project-context-inputs.yaml
handbook author environment-inventory --from-inputs ./environment-inventory-inputs.yaml
```

All three support validation without mutation:

```bash
handbook author charter --validate --from-inputs ./charter-inputs.yaml
handbook author project-context --validate --from-inputs ./project-context-inputs.yaml
handbook author environment-inventory --validate --from-inputs ./environment-inventory-inputs.yaml
```

All three accept stdin through `-`:

```bash
cat ./environment-inventory-inputs.yaml \
  | handbook author environment-inventory --validate --from-inputs -
```

### Bare-command behavior

These commands are no longer interviews:

```bash
handbook author charter
handbook author project-context
handbook author environment-inventory
```

Each must fail immediately and consistently because `--from-inputs <path|->` is required. The command must not inspect terminal capabilities, render a question, read stdin as an interview, or launch a model. Help and refusal text must point the caller to the structured-input template and the corresponding deterministic command.

### Artifact outputs

Successful mutation writes only the canonical target:

| Authoring command | Canonical output |
|---|---|
| `author charter` | `.handbook/charter/CHARTER.md` |
| `author project-context` | `.handbook/project_context/PROJECT_CONTEXT.md` |
| `author environment-inventory` | `.handbook/environment_inventory/ENVIRONMENT_INVENTORY.md` |

Validation mode must not mutate canonical files, lock files, prompt captures, run evidence, or repository state.

### Structured-input contract

- Charter retains `CHARTER_INPUTS.yaml.tmpl` and its existing validated structured-input model.
- Project context retains `PROJECT_CONTEXT_INPUTS.yaml.tmpl` and its existing validated structured-input model.
- Environment inventory gains `ENVIRONMENT_INVENTORY_INPUTS.yaml.tmpl` and a first-class structured-input model equivalent in rigor to the other two artifacts.
- Input schemas must describe facts, not model instructions or conversational turns.
- Unknown facts must be represented explicitly as unknowns rather than invented by the CLI renderer.
- The environment-inventory schema must cover its canonical sections: metadata, environment variables, secret handling, services/infrastructure, runtime assumptions, local development, CI, production/deployment, dependency/tooling inventory, update contract, and known unknowns.
- Inputs must never contain real secrets. Secret fields describe names, storage locations, rotation expectations, and usage only.

## Functional Requirements

### FR-1: Remove human-guided interviews

- Delete the CLI prompting module and prompt helper functions when no retained command uses them.
- Remove charter and project-context guided collectors, TTY detection, PTY-only behavior, guided defaults, follow-up prompts, and interview-specific refusals.
- Remove `guided_interview` as a successful authoring mode.
- Remove compiler entry points that exist solely to support guided charter synthesis.
- Update help, command descriptions, authoring documentation, and doctor recovery text so they never direct users into a TTY interview.

### FR-2: Remove embedded model execution

- Delete authoring-owned `std::process::Command` paths that start `codex exec`.
- Remove authoring-only Codex binary/model environment variables.
- Remove temporary output-message files, prompt capture files, process-output summarization, auth-error mapping, and model-transport refusals when they have no non-authoring consumers.
- The installed `handbook` binary must not require Codex, Node.js, an API key, or network access to author baseline artifacts from valid inputs.
- Skill instructions may direct the already-running agent to inspect a repository and assemble inputs, but must never tell the CLI to start another agent.

### FR-3: Preserve deterministic charter authoring

- Keep charter YAML parsing, semantic validation, deterministic Markdown rendering, canonical heading validation, guarded write paths, authoring locks, overwrite refusals, and `--validate` behavior.
- Remove the alternative guided/Codex-backed charter renderer.
- A given normalized input and deterministic timestamp policy must produce stable output.

### FR-4: Preserve deterministic project-context authoring

- Keep project-context YAML parsing, semantic validation, deterministic Markdown rendering, canonical heading validation, guarded writes, authoring locks, and overwrite refusals.
- Remove the alternative TTY input collector.
- Add validation-only parity with charter without writing the canonical file.

### FR-5: Add deterministic environment-inventory authoring

- Introduce an engine-owned `EnvironmentInventoryStructuredInput` model, YAML parser, semantic validator, and deterministic Markdown renderer.
- Move environment-inventory authoring from Codex synthesis to the same compiler pattern used by charter and project context: parse, validate, preflight, lock, render, validate rendered Markdown, and guarded write.
- Preserve required-charter validation and optional-project-context validation.
- Preserve existing canonical heading/order rules, existing-truth refusal, invalid-path refusal, and safe repair behavior.
- Add `--from-inputs <path|->` and `--validate` to the environment-inventory CLI command.
- Do not preserve the old no-argument behavior as a compatibility alias.

### FR-6: Keep the agent skill thin

- Retain the existing installed runtime and Codex skill-discovery packaging.
- Update skill instructions and installed resources to describe agent-gathered structured inputs and deterministic CLI authoring only.
- Do not create separate charter, project-context, and environment-inventory skill leaves as part of this cleanup.
- Do not embed schema copies in generated skill projections; installed resources remain the source used by the skill.
- Generated `.agents/skills/**` projections remain ignored, reproducible outputs rather than hand-edited source.

### FR-7: Remove obsolete proof and packaging leaves

- Delete the live Codex authoring smoke job and its script because the retained authoring path is offline and deterministic.
- Remove Codex API-key and model configuration used only by that smoke.
- Remove PTY test support and the `portable-pty` development dependency when no retained tests use it.
- Remove stub-Codex fixtures, transport tests, model-flag tests, auth-failure tests, and prompt-capture assertions that exclusively prove deleted behavior.
- Replace them with deterministic input, validation, rendering, write-safety, and CLI-parity tests.
- Audit authoring prompt/directive assets and delete those with no remaining supported runtime consumer. Do not delete generic pipeline assets merely because they mention Codex.

### FR-8: Preserve downstream baseline behavior

- `handbook setup init` continues to scaffold the three canonical baseline artifact locations.
- `handbook doctor --json` remains the machine-readable evidence surface for baseline completeness and artifact validity.
- Doctor actions for missing or invalid artifacts must point to the new deterministic command shape.
- Planning generation and inspection must accept artifacts produced through the new deterministic environment-inventory path exactly as they accept currently valid canonical truth.
- Existing valid canonical artifacts remain valid; this cleanup must not force repositories to rewrite already-authored Markdown solely because the authoring transport changed.

## Non-Goals

- Removing charter, project context, or environment inventory from the baseline.
- Letting agents bypass CLI validation and mutation safeguards as the documented default.
- Adding a general-purpose agent runtime, provider abstraction, model SDK, or prompt execution framework.
- Adding a new skill for each artifact.
- Redesigning canonical Markdown headings or the baseline-complete verdict.
- Reworking unrelated planning, sprint, release, feature-spec, pipeline, or contract-membrane behavior.
- Removing the generic pipeline engine or generic runner support solely because retired authoring stages used them.
- Removing or renaming declarative pipeline IDs, stage IDs, supported capture pairings, consumer IDs, or `SupportedTargetRegistry` ownership.
- Implementing the future `handbook contract ...` surface, contract lifecycle, evidence model, dock protocol, verdict model, or gate semantics.
- Rewriting archived provenance.
- Renaming every path that contains the word `codex`; skill discovery paths remain valid where they describe Codex integration rather than nested execution.

## Tech Stack

- Rust 2021 workspace
- `clap` 4 for CLI parsing
- `serde`, `serde_yaml_bw`, and `serde_json` for structured input and evidence
- Existing `handbook-engine`, `handbook-compiler`, and `handbook-cli` crate boundaries
- Bash and Python only for existing repository proof/install scripts
- No new runtime or development dependency is expected

## Landed Phase 5 And Phase 6 Preservation Contract

This cleanup intentionally removes a Phase 5 product behavior—the human-guided authoring flow—but must preserve the ownership and coupling work that Phase 5 and Phase 6 established.

### Phase 5 CLI ownership that must remain

- `crates/cli/src/main.rs` remains the thin clap registration and dispatch entrypoint.
- `crates/cli/src/author.rs` remains the owner of author command parsing, input-source handling, final operator/agent-facing output, refusal presentation, and exit-code policy.
- Removing `author_prompting.rs` means deleting an obsolete CLI-owned behavior, not relocating prompting into `handbook-engine`, `handbook-compiler`, or another crate.
- `handbook-engine` remains the owner of pure structured authoring types, parsing, normalization, validation, and deterministic rendering.
- Reusable crates must not begin owning final CLI help, final shell wording, or exit-code decisions.

### Phase 6 dependency direction that must remain

The following intra-workspace dependency posture is frozen for this cleanup:

| Crate | Allowed Handbook dependencies | Forbidden new edges |
|---|---|---|
| `handbook-engine` | none | `handbook-pipeline`, `handbook-flow`, `handbook-compiler`, `handbook-cli` |
| `handbook-pipeline` | `handbook-engine` | `handbook-flow`, `handbook-compiler`, `handbook-cli` |
| `handbook-flow` | `handbook-engine` | `handbook-pipeline`, `handbook-compiler`, `handbook-cli` |
| `handbook-compiler` | `handbook-engine`, `handbook-pipeline`, `handbook-flow` as retained transition/support glue | any lower-crate back-edge into compiler; new durable ownership claims |
| `handbook-cli` | the owner crates plus retained compiler adapters | exporting CLI-owned shell helpers as reusable lower-crate APIs |

Additional invariants:

- `handbook-pipeline` must retain zero runtime and dev dependency on `handbook-compiler`.
- The supported pipeline loading/selection, compile, capture, handoff, route, and route-state boundary remains unchanged.
- `handbook-flow` remains a clean resolver/result/budget layer over `handbook-engine` only.
- Deterministic environment-inventory core logic belongs with the existing engine-owned authoring core and must not create a new pipeline/compiler/flow dependency cycle.
- Retained compiler code may adapt engine-owned authoring behavior to guarded repository writes, but must not become the new implementation center or gain unrelated responsibilities.
- If implementation requires a new public API that is intended for downstream Substrate consumption, stop and apply the downstream crates.io plus real Substrate worktree proof gate. An additive symbol used only to complete Handbook-owned authoring is not to be advertised as a new downstream membrane API.

## Contract-Membrane Runway

This cleanup prepares for, but does not implement, the direction in:

- `docs/ideas/handbook-contract-membrane-architecture-memo.md`
- `docs/ideas/substrate_executable_contracts_architecture.md`

Where the two idea documents differ on authority ownership, the later Handbook membrane memo controls the direction: Handbook owns canonical contract authority and semantics; Substrate code-intelligence consumes those semantics for Substrate-side orchestration and gating. The older executable-contracts document remains valuable for vocabulary, contract/evidence/verdict shapes, dock taxonomy, validator ecosystem research, and roadmap ideas, but its original “Substrate owns” wording is not the ownership rule for future Handbook implementation.

The cleanup must leave these architectural conditions intact:

- Handbook remains positioned to own canonical contract truth, claims/invariants, evidence semantics, dock protocol, verdict semantics, and canonical artifact representation.
- Substrate code-intelligence remains a consumer/orchestrator, not a peer source of contract truth.
- The future membrane remains a first-class `handbook contract ...` surface rather than being buried under `author`, `pipeline`, `inspect`, or `doctor` during this cleanup.
- `doctor` remains baseline health/recovery and `inspect` remains packet/proof inspection; neither silently absorbs future contract evaluation semantics.
- This cleanup does not introduce a universal validator, dock framework, evidence store, lifecycle state machine, verdict model, or gate engine.
- No existing test, document, code path, or generated artifact is promoted into canonical contract truth by this cleanup.
- Any later downstream-intended public API work must meet the memo's publication and real Substrate-consumer proof gate before it is called complete.

## Commands

### Build and check

```bash
cargo check --workspace
cargo build --workspace
```

### Format and lint

```bash
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --all-features -- -D warnings
```

### Tests

```bash
cargo test --workspace
cargo test -p handbook-engine --test author_core
cargo test -p handbook-compiler --test author
cargo test -p handbook-cli --test author_cli
bash tools/ci/install-smoke.sh
python3 tools/check_archive_boundary.py --self-test
python3 tools/check_archive_boundary.py
```

### Final static checks for removed behavior

The implementation plan may refine exact path exclusions, but final verification must include bounded searches equivalent to:

```bash
! rg -n 'author_charter_guided|guided_interview|interactive_authoring_is_allowed|author_prompting' \
  crates/cli crates/compiler crates/engine

! rg -n 'HANDBOOK_AUTHOR_(CHARTER|ENVIRONMENT_INVENTORY)_CODEX|HANDBOOK_RUN_LIVE_AUTHOR_CHARTER_SMOKE' \
  crates tools .github justfile

! rg -n 'Command::new\([^)]*codex|failed to start `codex exec`|Codex-backed guided synthesis' \
  crates/cli crates/compiler crates/engine

! cargo tree -p handbook-pipeline -e normal,dev | rg 'handbook-compiler|handbook-cli'
! cargo tree -p handbook-flow -e normal,dev | rg 'handbook-pipeline|handbook-compiler|handbook-cli'
! cargo tree -p handbook-engine -e normal,dev | rg 'handbook-pipeline|handbook-flow|handbook-compiler|handbook-cli'

! rg -n 'handbook_compiler|handbook_cli' crates/pipeline/src crates/pipeline/tests
! rg -n 'handbook_pipeline|handbook_compiler|handbook_cli' crates/flow/src crates/flow/tests

git diff --check
```

The search must not ban legitimate Codex skill-packaging references or unrelated generic pipeline runner documentation.

## Project Structure

```text
crates/cli/src/main.rs
  CLI command and argument contract; all author subcommands require --from-inputs.

crates/cli/src/author.rs
  Shared deterministic CLI flow: read source, parse, validate, preflight, render/write, report.

crates/cli/src/author_prompting.rs
  Removed; no retained TTY prompting behavior.

crates/engine/src/author/*_core.rs
  Pure structured-input types, parsing, normalization, validation, and deterministic render logic.

crates/compiler/src/author/*.rs
  Public compiler authoring interfaces and refusal mapping.

crates/compiler/src/author/*_shell.rs
  Repo-aware preflight, canonical path checks, locks, and guarded writes; no model processes.

core/library/{charter,project_context,environment_inventory}/
  Canonical Markdown templates, normalized input templates, and agent-facing input guidance.

core/pipelines/ and core/stages/
  Existing declarative pipeline/stage truth remains unchanged by this CLI authoring cleanup.

install/handbook-home/
  Source templates for installed Handbook skill/runtime resources.

tools/codex/
  Retained skill generation/install/discovery tooling; not an authoring model wrapper.

crates/{engine,compiler,cli}/tests/
  Pure-core, guarded-write, and binary CLI contract coverage.

tools/ci/
  Deterministic install/proof scripts; no live model smoke.
```

## Code Style

Follow existing Rust conventions: typed inputs, narrow public functions, explicit refusal mapping, compiler shell/core separation, and no panics for user-controlled data.

Representative target pattern:

```rust
pub fn author_environment_inventory_from_input(
    repo_root: impl AsRef<Path>,
    input: &EnvironmentInventoryStructuredInput,
) -> Result<AuthorEnvironmentInventoryResult, AuthorEnvironmentInventoryRefusal> {
    let repo_root = repo_root.as_ref();
    validate_environment_inventory_structured_input(input)?;
    preflight_author_environment_inventory(repo_root)?;
    with_environment_inventory_authoring_lock(repo_root, || {
        let markdown = render_environment_inventory_markdown(input)?;
        validate_environment_inventory_markdown(&markdown)?;
        write_canonical_environment_inventory_markdown(repo_root, &markdown)
    })
}
```

Conventions:

- Use artifact-specific structured types rather than untyped maps.
- Keep parsing/rendering pure in `handbook-engine`.
- Keep filesystem checks and writes in `handbook-compiler` shell modules.
- Keep CLI modules responsible for argument/source handling and rendered outcomes only.
- Reuse shared input-source and output-rendering helpers where behavior is genuinely identical.
- Prefer deleting guided-only branches over retaining feature flags or dormant compatibility code.
- Preserve existing canonical path constants and refusal taxonomy where still semantically correct.

## Testing Strategy

### Engine tests

For each artifact:

- valid YAML parses into the typed input;
- malformed YAML is rejected;
- missing or vague required facts are rejected;
- normalization is deterministic;
- rendered Markdown contains required headings exactly once and in canonical order;
- rendered Markdown does not leak template placeholders;
- environment-inventory unknowns render explicitly rather than being invented;
- real secret values are not required or emitted by fixtures.

### Compiler tests

For each artifact:

- validation succeeds without filesystem mutation;
- authoring replaces starter-owned or semantically invalid baseline content where current policy permits;
- valid non-starter canonical truth is not overwritten;
- missing/invalid `.handbook` roots refuse safely;
- symlink and invalid write targets refuse safely;
- locks and guarded writes remain intact;
- failures do not partially write canonical output;
- environment inventory still requires a valid charter and validates optional project context.

### CLI tests

For each artifact and both file/stdin input sources:

- `--from-inputs` authors successfully;
- `--validate --from-inputs` validates without mutation;
- missing `--from-inputs` fails immediately and does not prompt;
- malformed inputs produce artifact-specific refusals;
- success output identifies deterministic input mode and canonical output;
- help snapshots show the same command grammar across all three artifacts.

No CLI test may require a PTY, Codex stub, API key, network, model name, or prompt-capture file.

### Integration and packaging tests

- `handbook setup init` followed by deterministic authoring of all three artifacts reaches `baseline_complete` in `handbook doctor --json`.
- The installed skill resolves the installed input templates and binary.
- The install smoke proves packaged authoring without Codex installed.
- CI runs entirely offline with respect to model inference.
- Existing valid artifact fixtures continue to pass canonical validation.
- Existing pipeline catalog/compile/capture/handoff tests remain green and prove the cleanup did not alter the frozen supported-target wedge.
- `cargo tree` and source-coupling checks prove no Phase 6 dependency edge regressed.

### Coverage posture

No numeric coverage target is introduced. Every removed transport/interview behavior must be replaced by tests for the retained deterministic contract, with at least one end-to-end all-three-artifact baseline proof.

## Boundaries

### Always do

- Run GitNexus impact analysis immediately before modifying each function or method in implementation.
- Warn before editing any symbol whose refreshed impact is HIGH or CRITICAL.
- Preserve all three canonical artifacts and their validation rules.
- Require explicit structured inputs for mutation.
- Keep validation mode side-effect free.
- Keep CLI output deterministic and actionable for agents.
- Run `gitnexus detect-changes` before any implementation commit.
- Run the full Rust workspace checks and install smoke before declaring completion.
- Preserve the landed Phase 5/6 crate dependency direction and rerun `cargo tree` proof for the owner crates.

### Ask first

- Changing canonical Markdown headings or validity rules.
- Renaming or removing the installed `handbook` or `handbook-charter-intake` skill identities.
- Changing declarative pipeline definitions, supported targets, capture pairings, generic runner support, or active pipeline stages.
- Introducing a new dependency or schema format.
- Changing overwrite/repair policy for existing canonical truth.
- Expanding this cleanup into planning, sprint, release, feature-spec, or contract authoring.

### Never do

- Remove environment inventory or make it a direct-file-only exception.
- Keep a hidden or undocumented interactive fallback.
- Launch Codex or another model from the Handbook authoring runtime.
- Require model credentials for deterministic authoring or CI.
- Generate project facts inside the deterministic renderer.
- Store real secrets in templates, fixtures, logs, or canonical inventory examples.
- Hand-edit ignored generated `.agents/skills/**` projections.
- Rewrite archived history to pretend the removed behavior never existed.
- Preserve dead transport code solely for hypothetical compatibility.

## Risks and Mitigations

### CLI contract blast radius

GitNexus currently rates `execute_author_charter_command` as CRITICAL because it participates in the dispatcher and multiple injected command tests. It rates `author_environment_inventory` as HIGH because it feeds the CLI and many authoring tests.

Mitigation:

- land the shared command grammar and tests together;
- keep deterministic charter behavior unchanged while deleting only its guided branch;
- introduce environment-inventory core rendering before removing its synthesis shell;
- verify command help, doctor actions, and install smoke at each cutover.

The local GitNexus index was stale and `npx gitnexus analyze` crashed with a native N-API error during specification work. Implementation must refresh or otherwise repair the index before relying on its final risk classification.

### Environment-inventory schema growth

A complete inventory has more repeated structures than charter or project context. An overly permissive schema would move invention into the renderer; an overly elaborate schema would recreate bloat.

Mitigation:

- model only fields needed by the existing canonical template;
- use typed repeated entries for environment variables and dependencies;
- represent unknowns explicitly;
- avoid provider-specific or deployment-platform-specific fields.

### Skill/runtime naming confusion

The repository uses `tools/codex/**` for skill packaging while the removed feature also invokes `codex exec`.

Mitigation:

- define the boundary by behavior, not by path substring;
- retain discovery/install tooling;
- remove only nested execution, credentials, model flags, and live inference proof.

### Cross-crate ownership regression

Deleting a CLI feature can accidentally push structured-input handling, shell rendering, or repository mutation into the wrong crate, or tempt implementation to reuse pipeline internals from authoring.

Mitigation:

- treat the Phase 5/6 dependency table in this spec as a hard implementation contract;
- keep pure environment-inventory input/render logic in the engine-owned authoring core;
- keep final shell presentation in CLI;
- keep repository-aware transition adapters narrow and do not add pipeline/flow dependencies for authoring convenience;
- run dependency and source-coupling proof before and after implementation.

## Success Criteria

- [ ] Charter, project context, and environment inventory all require `--from-inputs <path|->`.
- [ ] All three support `--validate` without mutation.
- [ ] Environment inventory has a committed normalized input template and typed deterministic renderer.
- [ ] No supported authoring command reads a TTY interview answer.
- [ ] No authoring runtime code launches `codex exec` or another model.
- [ ] No authoring test or CI job requires Codex, an API key, model selection, a PTY, or network inference.
- [ ] Guided-only modules, functions, dependencies, fixtures, scripts, environment variables, and prompt assets have been removed when unreferenced.
- [ ] Existing skill installation and discovery still work.
- [ ] An agent can run setup, author all three artifacts through the CLI, and receive `baseline_complete` from `handbook doctor --json`.
- [ ] Existing valid canonical artifacts remain valid without migration.
- [ ] `handbook-engine`, `handbook-pipeline`, `handbook-flow`, retained `handbook-compiler`, and `handbook-cli` preserve the Phase 5/6 dependency direction recorded in this spec.
- [ ] `handbook-pipeline` still has no `handbook-compiler` dependency and its catalog/compile/capture/handoff/route-state verification remains green.
- [ ] No contract-membrane model or CLI behavior was prematurely implemented or buried in an existing command family.
- [ ] `cargo fmt --all -- --check`, clippy, workspace tests, workspace check, archive checks, install smoke, and `git diff --check` pass.
- [ ] Refreshed GitNexus impact analysis was run before symbol edits and `gitnexus detect-changes` confirms only expected authoring/test/CI flows changed before commit.

## Open Questions

None currently. The clarified direction is that environment inventory must remain a first-class CLI-authored artifact and match the deterministic structured-input contract of the other authoring surfaces.
