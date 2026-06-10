# Spec: Handbook Engine Extraction Phase 3 Slice 3 (Slice 3.3) - Environment-Inventory Deterministic Core Split

## Assumptions

1. Phase 3 Slice 2 is complete, so the authoring-module split pattern established for charter and project-context (`public facade` + `deterministic core` + `shell/runtime helpers`) is now an approved in-repo precedent for the remaining Phase 3 authoring surfaces.
2. Phase 2 Slice 3 is complete, so template-library resolution already owns shipped directive/template selection for environment-inventory authoring; Slice 3.3 must not reopen template ownership or shipped-default asset rules.
3. Live `crates/compiler/src/author/environment_inventory.rs` currently mixes at least four concerns in one module:
   - deterministic environment-inventory markdown contract validation, including required heading order, canonical-file assertions, and legacy-path rejection
   - synthesis-result contract validation, including the exact `Project Context Ref` line for the optional project-context input
   - runtime synthesis orchestration, including template-library prompt assembly, `codex exec` spawning, env-var overrides, temp output handling, and process-summary formatting
   - canonical authoring preflight, upstream charter/project-context truth loading, lock/write flow, and CLI/product-specific refusal wording that references `handbook author environment-inventory`
4. Slice 3.3 must preserve the current public compiler and CLI surface: `handbook author environment-inventory`, canonical write target `.handbook/environment_inventory/ENVIRONMENT_INVENTORY.md`, required charter / optional project-context posture, current refusal categories, and the runtime override contract for `HANDBOOK_AUTHOR_ENVIRONMENT_INVENTORY_CODEX_BIN` and `HANDBOOK_AUTHOR_ENVIRONMENT_INVENTORY_CODEX_MODEL` all stay behavior-stable unless the human approves a separate shell redesign.
5. There is no guided TTY or `--from-inputs` environment-inventory surface today; Slice 3.3 must not invent one as part of this boundary cleanup.
6. `setup`, `doctor`, shared refusal cleanup, generalized authoring-helper extraction, and Phase 4 crate work remain deferred even if live code shows adjacent duplication.
7. The smallest durable seam is expected to keep `crates/compiler/src/author/environment_inventory.rs` as the public facade while extracting new environment-inventory-local subowners such as `environment_inventory_core.rs` and `environment_inventory_shell.rs`; if live implementation proves a different local file split is cleaner, the ownership contract still must match this spec.

## Objective

Separate reusable deterministic environment-inventory contract logic from synthesis runtime orchestration and product-shell behavior inside the current compiler crate, without changing the public environment-inventory authoring contract.

The maintainer needs this slice so the future `handbook-engine` boundary is visible before any crate move begins. Success means:

- environment-inventory markdown contract validation and pure contract/model checks live behind one engine-shaped deterministic boundary
- prompt construction, `codex exec` synthesis runtime, env-var override handling, temp output/process-summary handling, and authoring mutation flow live outside that deterministic boundary
- `preflight_author_environment_inventory` and `author_environment_inventory` become thin orchestrators over the split instead of owning every concern directly
- required charter truth, optional project-context posture, and the current canonical-file contract remain behavior-stable
- no setup, doctor, shared-authoring cleanup, or Phase 4 crate work leaks in

## Tech Stack

- Rust 2021 workspace
- `handbook-compiler` library crate
- `handbook-cli` binary crate
- Existing authority docs:
  - `HANDBOOK_ENGINE_EXTRACTION_PLAN.md`
  - `docs/specs/handbook-engine-extraction-slice-map.md`
  - `docs/specs/handbook-engine-extraction-phase-2-slice-3-template-and-library-resolver-spec.md`
  - `docs/specs/handbook-engine-extraction-phase-2-slice-3-template-and-library-resolver-plan.md`
  - `docs/specs/handbook-engine-extraction-phase-2-slice-3-template-and-library-resolver-tasks.md`
  - `docs/specs/handbook-engine-extraction-phase-3-slice-2-project-context-core-split-spec.md`
  - `docs/specs/handbook-engine-extraction-phase-3-slice-2-project-context-core-split-plan.md`
  - `docs/specs/handbook-engine-extraction-phase-3-slice-2-project-context-core-split-tasks.md`

## Commands

Primary slice verifier:

```bash
cargo test -p handbook-compiler --test author
```

Public-surface fallout guard:

```bash
cargo test -p handbook-cli --test author_cli
```

Shell-vs-core ownership evidence scan:

```bash
rg -n "HANDBOOK_AUTHOR_ENVIRONMENT_INVENTORY_CODEX_|Command::new|codex exec|next_safe_action|acquire_environment_inventory_authoring_lock|write_repo_relative_bytes|prepare_environment_inventory_authoring_inputs|build_environment_inventory_synthesis_prompt|summarize_process_output" crates/compiler/src/author/environment_inventory*.rs crates/compiler/src/author/mod.rs
```

Core contract evidence scan:

```bash
rg -n "validate_environment_inventory_markdown|validate_required_heading_order_result|REQUIRED_ENVIRONMENT_INVENTORY_HEADINGS|Project Context Ref|legacy non-canonical path claims" crates/compiler/src/author/environment_inventory*.rs crates/compiler/src/author/mod.rs
```

Final workspace verification wall:

```bash
cargo check --workspace
cargo test --workspace
```

## Project Structure

```text
crates/compiler/src/author/environment_inventory.rs       -> current public environment-inventory facade; should become a thin orchestration layer after the split
crates/compiler/src/author/environment_inventory_core.rs  -> proposed deterministic owner for markdown contract validation, required heading ordering, canonical-file assertions, and pure synthesis-result contract checks
crates/compiler/src/author/environment_inventory_shell.rs -> proposed shell owner for upstream truth loading, prompt construction, `codex exec` runtime, env-var overrides, temp output/process-summary handling, lock/write flow, and product-shell refusal wording
crates/compiler/src/author/mod.rs                         -> author-surface reexports that should keep the outward compiler API stable
crates/compiler/src/lib.rs                                -> compiler public surface that should remain stable for CLI callers
crates/compiler/tests/author.rs                           -> compiler-level regression coverage for environment-inventory validation and authoring flows
crates/cli/src/main.rs                                    -> CLI command surface that should remain behavior-stable for `handbook author environment-inventory`
crates/cli/tests/author_cli.rs                            -> CLI-level environment-inventory authoring and refusal-surface regression coverage
```

If live implementation proves `author/environment_inventory/{core,shell}.rs` is cleaner than sibling files, that layout is acceptable only if the same ownership split and public API stability hold.

## Code Style

Prefer a thin public adapter over one file that owns deterministic markdown contract checks, prompt/runtime orchestration, repo mutation, and operator-facing refusal copy all at once.

```rust
let inputs = environment_inventory_shell::load_authoring_inputs(repo_root)?;
let markdown = environment_inventory_shell::synthesize_markdown(repo_root, &inputs)?;

environment_inventory_core::validate_markdown(
    &markdown,
    inputs.expected_project_context_ref(),
)?;

environment_inventory_shell::write_canonical_environment_inventory(repo_root, &markdown)
```

Conventions:

- deterministic environment-inventory core must not read environment variables, spawn `codex exec`, allocate temp output files, inspect the repo root, acquire authoring locks, or mutate the repo
- shell adapters may own `.handbook` root inspection, charter/project-context artifact loading, template-library prompt assembly, env-var/runtime override handling, synthesis transport, lock handling, canonical writes, and CLI-oriented refusal wording
- keep the public `handbook_compiler` exports stable through `author/mod.rs`
- keep environment-inventory-specific helpers local to this surface unless the human later approves shared authoring infrastructure
- prefer typed handoffs between core and shell instead of ad hoc inline checks

## Testing Strategy

- Framework: existing Rust integration tests in `crates/compiler/tests/` and CLI regression tests in `crates/cli/tests/`
- Primary test levels:
  - compiler author tests for deterministic markdown contract validation and authoring flows
  - compiler author tests for required charter truth, optional project-context behavior, and synthesis-result validation
  - CLI author tests to prove the outward `handbook author environment-inventory` contract stays stable
- Coverage focus:
  - deterministic environment-inventory contract checks remain free of env/process/filesystem mutation concerns
  - synthesis runtime behavior keeps env-var override support and still fails closed when prompt/runtime/output handling breaks
  - canonical authoring preflight and write flows still refuse safely without partial writes
  - refusal summaries and next-safe-action wording remain stable at the CLI surface unless the spec changes first
  - optional project-context posture remains unchanged: valid canonical truth contributes the exact canonical reference line, while missing/empty/starter truth still behaves as optional
- Coverage expectation:
  - Packet 3.3.1 proves the deterministic environment-inventory core boundary without changing public behavior
  - Packet 3.3.2 proves synthesis/runtime behavior and CLI regressions still pass after the split

## Slice Scope

In scope:

- extract deterministic environment-inventory markdown contract validation into one engine-shaped owner
- reduce `environment_inventory.rs` to a thin orchestrator and public facade
- isolate upstream truth loading, prompt construction, `codex exec` synthesis runtime, env-var overrides, temp output/process-summary handling, canonical authoring preflight, lock/write flow, and refusal wording away from the deterministic core
- preserve the current public compiler and CLI environment-inventory contract
- preserve the Slice 2.3 template-library boundary while making the environment-inventory compiler seam cleaner

Out of scope:

- changing template-library selection rules, shipped directive/template content, or starter asset ownership
- redesigning CLI prompts, output wording, success rendering, help text, or refusal categories
- changing required headings, canonical reference lines, or document semantics intentionally
- adding guided TTY, structured-input, or new runtime configuration surfaces
- moving code into new crates before Phase 4
- widening into project-context, setup, doctor, generalized authoring-helper cleanup, or shared refusal cleanup outside environment-inventory authoring

## Authority Inputs

- `HANDBOOK_ENGINE_EXTRACTION_PLAN.md`
- `docs/specs/handbook-engine-extraction-slice-map.md`
- Slice 2.3 authority set:
  - `docs/specs/handbook-engine-extraction-phase-2-slice-3-template-and-library-resolver-spec.md`
  - `docs/specs/handbook-engine-extraction-phase-2-slice-3-template-and-library-resolver-plan.md`
  - `docs/specs/handbook-engine-extraction-phase-2-slice-3-template-and-library-resolver-tasks.md`
- Slice 3.2 authority set:
  - `docs/specs/handbook-engine-extraction-phase-3-slice-2-project-context-core-split-spec.md`
  - `docs/specs/handbook-engine-extraction-phase-3-slice-2-project-context-core-split-plan.md`
  - `docs/specs/handbook-engine-extraction-phase-3-slice-2-project-context-core-split-tasks.md`
- Live environment-inventory surfaces:
  - `crates/compiler/src/author/environment_inventory.rs`
  - `crates/compiler/src/author/mod.rs`
  - `crates/compiler/src/lib.rs`
  - `crates/cli/src/main.rs`
- Live regression evidence:
  - `crates/compiler/tests/author.rs`
  - `crates/cli/tests/author_cli.rs`

## Current Mixed Responsibilities To Untangle

| Responsibility | Current live owner | Slice 3.3 requirement |
| --- | --- | --- |
| deterministic markdown contract validation | `validate_environment_inventory_markdown` and `validate_required_heading_order_result` in `author/environment_inventory.rs` | move behind one deterministic core boundary with no env/process/filesystem or product-copy ownership |
| exact project-context reference contract | `validate_synthesized_environment_inventory_markdown` plus `EnvironmentInventorySynthesisInputs` in `author/environment_inventory.rs` | keep as deterministic contract logic, but make shell helpers supply explicit expectations instead of owning inline contract checks |
| upstream canonical truth loading and baseline gating | `prepare_environment_inventory_authoring_inputs`, `required_charter_markdown`, `optional_project_context_markdown`, and precondition helpers in `author/environment_inventory.rs` | keep shell-owned because they inspect repo state and canonical artifacts |
| template-library prompt construction | `build_environment_inventory_synthesis_prompt` in `author/environment_inventory.rs` | keep shell-owned and dependent on the Slice 2.3 resolver, not on the deterministic core |
| synthesis runtime and env/process handling | `synthesize_environment_inventory_markdown`, env-var lookups, temp output handling, and process-summary helpers in `author/environment_inventory.rs` | isolate as shell/runtime behavior so deterministic validation depends on explicit inputs instead of ambient runtime state |
| canonical authoring preflight, lock/write flow, and CLI-oriented refusal summaries | `preflight_author_environment_inventory`, `author_environment_inventory`, lock helpers, and refusal constructors in `author/environment_inventory.rs` | leave as thin orchestration around core + shell helpers instead of mixed owner logic |

## Boundaries

- Always:
  - keep deterministic environment-inventory code free of env reads, process spawning, temp output files, authoring locks, and repo mutation
  - preserve the Slice 2.3 template-library ownership boundary
  - preserve the current public `handbook author environment-inventory` behavior first; this slice is internal seam cleanup
  - prove both compiler-level and CLI-level environment-inventory regressions after the split
  - keep required heading order, canonical path references, and optional project-context posture stable unless a spec update says otherwise
- Ask first:
  - renaming public `handbook_compiler` environment-inventory functions, refusal kinds, or result types
  - changing the required headings, canonical reference lines, or template semantics intentionally
  - changing `HANDBOOK_AUTHOR_ENVIRONMENT_INVENTORY_CODEX_BIN` / `HANDBOOK_AUTHOR_ENVIRONMENT_INVENTORY_CODEX_MODEL` or the current `codex exec` invocation contract
  - extracting shared prompt/runtime or authoring-lock helpers that also touch charter, project-context, setup, or doctor
  - changing the canonical environment-inventory write contract or lock-file contract
- Never:
  - move Phase 4 crate-boundary work into this slice
  - pull template-library selection ownership back out of the Slice 2.3 resolver
  - let the deterministic core depend directly on env vars, process spawning, temp files, lock acquisition, or canonical writes
  - invent guided-interview or `--from-inputs` flows for environment-inventory authoring inside this slice
  - widen into project-context, setup, doctor, or generalized shell-copy cleanup

## Success Criteria

- A deterministic environment-inventory core boundary exists for markdown contract validation and pure contract/model checks.
- `preflight_author_environment_inventory` and `author_environment_inventory` become thin orchestrators over deterministic core and shell helpers.
- Upstream truth loading, prompt construction, synthesis runtime, env-var override handling, temp output/process-summary handling, and lock/write flow are isolated outside the deterministic core.
- Required charter truth, optional project-context posture, canonical-file assertions, and current refusal posture remain behavior-stable.
- `cargo test -p handbook-compiler --test author` and `cargo test -p handbook-cli --test author_cli` continue to pass against the split architecture.
- No template-library redesign, setup/doctor work, or Phase 4 crate-move work leaked in.

## Open Questions

- Should the deterministic core own a small typed contract object for the expected project-context reference line, or is passing an explicit expected reference string into core validation enough for Slice 3.3 as long as repo/runtime access stays outside the core?
- Is a dedicated `environment_inventory_shell.rs` the smallest durable shell owner, or does live implementation truth prove that a thinner local runtime module is sufficient as long as the same core-vs-shell contract holds?
