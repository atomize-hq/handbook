# Plan: Handbook Engine Extraction Phase 3 Slice 3 (Slice 3.3) - Environment-Inventory Deterministic Core Split

## Objective

Split reusable deterministic environment-inventory contract behavior away from synthesis runtime orchestration and product-shell authoring logic inside the current compiler crate, while preserving the existing public `handbook author environment-inventory` contract.

Spec reference: [handbook-engine-extraction-phase-3-slice-3-environment-inventory-core-split-spec.md](./handbook-engine-extraction-phase-3-slice-3-environment-inventory-core-split-spec.md)

## Major Modules

1. Deterministic environment-inventory core
   - proposed as `crates/compiler/src/author/environment_inventory_core.rs`
   - owns markdown contract validation, required heading ordering, canonical-file assertions, exact project-context reference validation, and any pure contract helpers

2. Environment-inventory shell / orchestration layer
   - `crates/compiler/src/author/environment_inventory.rs`
   - stays as the outward facade and thin orchestration layer for preflight, public function wiring, and compiler-facing refusal mapping

3. Runtime synthesis helper layer
   - proposed as `crates/compiler/src/author/environment_inventory_shell.rs`
   - owns upstream charter/project-context truth loading, template-library prompt assembly, `codex exec` runtime, env-var override handling, temp output/process-summary handling, lock acquisition, canonical-write validation, repo mutation, and CLI-oriented refusal wording

4. Public export and caller layer
   - `crates/compiler/src/author/mod.rs`
   - `crates/compiler/src/lib.rs`
   - `crates/cli/src/main.rs`
   - should preserve current exported functions, refusal types, and command behavior even if ownership moves behind new internal modules

5. Regression layer
   - `crates/compiler/tests/author.rs`
   - `crates/cli/tests/author_cli.rs`
   - proves deterministic contract checks and authoring flows both survive the boundary cleanup

## Dependencies And Order

### Prerequisite: Freeze the Slice 3.2 split pattern, the Slice 2.3 template-library boundary, and the current public environment-inventory contract

Why first:

- Slice 3.3 is internal boundary cleanup only
- Slice 3.2 already established the in-place authoring split pattern for another Phase 3 authoring surface
- Slice 2.3 already moved shipped directive/template ownership behind the template-library resolver
- the public environment-inventory CLI and compiler contract must stay stable while internals are reorganized

Output:

- one frozen statement that Slice 3.3 is about deterministic-core separation only
- one stable public contract for `handbook author environment-inventory`, canonical writes, required charter truth, optional project-context posture, and refusal behavior

### Packet 3.3.1: Environment-inventory deterministic model split

Why first:

- the reusable deterministic core must exist before prompt/runtime cleanup can safely target it
- later Phase 4 engine migration needs a stable engine-shaped environment-inventory contract to move
- authoring and refusal flows should depend on deterministic contract helpers, not the other way around

Output:

- one deterministic environment-inventory owner for markdown contract validation and pure contract checks
- a thinner `author/environment_inventory.rs` facade that delegates deterministic behavior instead of owning it inline
- preserved public validation entrypoints and regression coverage for contract enforcement

### Packet 3.3.2: Environment-inventory prompt and product cleanup

Why second:

- shell/runtime cleanup is only safe once the deterministic contract is already explicit
- prompt construction, synthesis runtime, lock/write flow, and refusal wording should wrap a stable core, not migrate simultaneously with it

Output:

- one shell/runtime owner for upstream truth loading, template-library prompt assembly, `codex exec` runtime, env-var override handling, temp output/process-summary handling, and canonical mutation flow
- thin `preflight_author_environment_inventory` and `author_environment_inventory` adapters over core + shell helpers
- preserved CLI behavior after the internal split

## Risks And Mitigations

### Risk: the split changes the public compiler API or CLI behavior

Mitigation:

- keep `author/mod.rs` and `lib.rs` exports stable
- use compiler and CLI regression tests as required rails for both packets
- treat public behavior changes as out of scope unless the spec changes first

### Risk: deterministic code still retains env/process/filesystem dependencies

Mitigation:

- define the core boundary first and keep it free of env reads, `codex exec` spawning, temp output handling, lock helpers, and canonical writes
- use ownership scans after each packet to confirm runtime constructs stayed outside the core

### Risk: Packet 3.3.2 reopens Slice 2.3 template-library ownership

Mitigation:

- keep template-library resolution inside shell/runtime helpers
- treat shipped directive/template selection as already solved authority from Slice 2.3
- allow only local call-site cleanup, not new selection rules or asset redesign

### Risk: prompt/runtime cleanup widens into shared authoring infrastructure

Mitigation:

- keep the work scoped to environment-inventory files and tests
- defer any shared helper extraction unless it is environment-inventory-local or explicitly approved later

### Risk: exact project-context reference or required-heading behavior drifts unexpectedly

Mitigation:

- keep the existing canonical-file and `Project Context Ref` contract stable
- prove heading-order and reference-line behavior through existing compiler and CLI regression rails
- isolate deterministic contract checks so the shell consumes them instead of re-deriving them inline

## Parallel Vs Sequential

Sequential:

- deterministic core extraction before any prompt/runtime cleanup
- public export stabilization before CLI-surface verification
- shell/runtime cleanup before final workspace wall

Parallel opportunities after Packet 3.3.1 starts:

- compiler regression updates for deterministic validation helpers and CLI regression updates for authoring outcomes can proceed in parallel once the core boundary is explicit
- prompt/runtime helper extraction and refusal-surface assertions can proceed together as long as they do not reopen deterministic-core ownership or Slice 2.3 asset ownership

## Verification Checkpoints

### Checkpoint 1: Deterministic environment-inventory core exists and public contract validation still works

```bash
cargo test -p handbook-compiler --test author
```

### Checkpoint 2: Authoring preflight, synthesis runtime, canonical writes, and CLI behavior still preserve the public contract

```bash
cargo test -p handbook-compiler --test author
cargo test -p handbook-cli --test author_cli
```

### Ownership spot-check: runtime behavior stays outside the deterministic core

```bash
rg -n "HANDBOOK_AUTHOR_ENVIRONMENT_INVENTORY_CODEX_|Command::new|codex exec|next_safe_action|acquire_environment_inventory_authoring_lock|write_repo_relative_bytes|prepare_environment_inventory_authoring_inputs|build_environment_inventory_synthesis_prompt|summarize_process_output" crates/compiler/src/author/environment_inventory*.rs crates/compiler/src/author/mod.rs
```

### Ownership spot-check: deterministic environment-inventory contract API remains explicit

```bash
rg -n "validate_environment_inventory_markdown|validate_required_heading_order_result|REQUIRED_ENVIRONMENT_INVENTORY_HEADINGS|Project Context Ref|legacy non-canonical path claims" crates/compiler/src/author/environment_inventory*.rs crates/compiler/src/author/mod.rs
```

### Final checkpoint

```bash
cargo check --workspace
cargo test --workspace
```

## Exit Conditions

The slice is ready for human review when:

- deterministic environment-inventory contract logic is isolated behind one engine-shaped boundary
- upstream truth loading, prompt/runtime behavior, and authoring mutation flow are isolated behind shell/runtime helpers
- `author/environment_inventory.rs` is a thin facade instead of the monolithic owner of every environment-inventory concern
- public compiler and CLI environment-inventory behavior remain stable under regression coverage
- no template-library redesign, shared-authoring cleanup, or adjacent-slice work leaked in
