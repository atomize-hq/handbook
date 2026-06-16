# Plan: Handbook Engine Extraction Phase 6 - `handbook-pipeline` Boundary Cleanup

Spec reference: [handbook-engine-extraction-phase-6-handbook-pipeline-boundary-cleanup-spec.md](./handbook-engine-extraction-phase-6-handbook-pipeline-boundary-cleanup-spec.md)

## Objective

Turn the landed Phase 6 Packet 4 `handbook-pipeline` seam map entry into one bounded execution plan, without starting implementation.

Success for this plan means:

- the reviewed supported-target importer boundary is concrete enough that later packets can implement against it without rediscovering scope
- the remaining compiler-backed fixture/support coupling is split cleanly into a live evidence ledger, retained compiler context, bounded cleanup target, and explicit non-goals
- the plan preserves the Phase 6 human-review posture: this triplet is preparation for later implementation approval, not implementation approval itself

## Major Artifacts

1. **Boundary freeze**
   - the quote-ready reviewed importer boundary for `handbook-pipeline`
   - the explicit statement that the full current public re-export surface is not the durable importer contract
   - the explicit statement that `setup` stays outside this seam’s reviewed importer boundary

2. **Live evidence ledger**
   - the concrete `pipeline_catalog` import path into compiler template-library support
   - the explicit statement that `cargo tree -p handbook-pipeline` still shows `handbook-compiler` as a dev-dependency rather than a runtime owner
   - any additional bounded pipeline test/support surfaces that must be considered before implementation starts

3. **Retained compiler context**
   - the retained compiler re-export path that makes the `pipeline_catalog` coupling possible today
   - the statement that `crates/compiler/src/template_library.rs` remains the live implementation home for the shipped-template support currently reached by the pipeline test

4. **Bounded cleanup target**
   - the quote-ready target: later implementation must remove or relocate the pipeline-side compiler-backed fixture/support dependency needed by pipeline catalog/runtime proof
   - the minimum acceptable future implementation shape for removing or relocating that bounded coupling

5. **Explicit non-goals**
   - the explicit non-goals that stop the seam from expanding into retained `handbook-compiler` retirement, broader authoring-stack relocation, or CLI shell/support reassignment

6. **Verification and review gate**
   - the future implementation verification wall for pipeline and compiler authoring proof
   - the explicit human review gate that still blocks implementation, packet-prompt authoring for later execution, production edits, publication, crates.io work, Substrate consumption, and integration implementation

## Order

### Packet 1: Freeze seam authority and exact boundary question

Why first:

- Packet 2 and Packet 4 already define the residual `handbook-pipeline` problem; later planning should not re-litigate why the seam exists
- the reviewed importer boundary must be fixed before any decoupling tactic can be judged in or out of scope
- this packet keeps the plan anchored to the already-landed ownership family instead of drifting into a new architecture story

Output:

- one quote-ready statement that `handbook-pipeline` remains handbook-owned
- one quote-ready statement that only the reviewed supported-target importer boundary is durable and the full current public `handbook-pipeline` surface is not the durable importer contract
- one quote-ready statement that this seam owns the pipeline-side `template_library` / compiler-backed fixture-support decoupling question
- one explicit inside/outside boundary freeze:
  - loading/selection surfaces
  - compile surfaces
  - capture surfaces
  - handoff surfaces
  - route-state surfaces needed for supported pipeline execution
  - `setup` stays outside this reviewed supported-target importer boundary
  - CLI/product-shell helpers stay outside this boundary
  - compiler-routed compatibility helpers stay outside this boundary

### Packet 2: Freeze the coupling evidence ledger and cleanup target

Why second:

- later implementation packets need one agreed statement of the exact compiler-backed coupling they are allowed to remove
- the seam must stay focused on pipeline catalog/runtime proof rather than full compiler support ownership
- the plan needs to distinguish the pipeline-side `template_library` fixture/support question from broader compiler authoring and CLI shell questions

Output:

- one explicit **live evidence ledger** centered on `crates/pipeline/tests/pipeline_catalog.rs`, including the exact import of `handbook_compiler::author::template_library::{resolve_shipped_template_library, TemplateLibraryRequest, TemplateLibrarySelection}`
- one explicit **retained compiler context** statement covering `crates/compiler/src/lib.rs`, `crates/compiler/src/author/mod.rs`, and `crates/compiler/src/template_library.rs`
- one explicit **bounded cleanup target**: later implementation must remove or relocate the pipeline-side compiler-backed fixture/support dependency needed by pipeline catalog/runtime proof
- one explicit **non-goals** list that keeps retained `handbook-compiler` retirement, broader authoring-stack relocation, and CLI shell/support reassignment out of this seam

### Packet 3: Choose the allowed cleanup shape, verification wall, and review stop

Why third:

- once the seam boundary and coupling evidence are frozen, the plan can describe what future implementation is allowed to do without over-prescribing the code change
- the cleanup must be narrow enough to preserve compiler authoring support while removing the pipeline test/support dependency
- the verification wall must prove both pipeline behavior and non-regression of the still-retained compiler authoring support
- the landed Phase 6 family already established that planning does not equal execution approval, so Packet 3 must end at review instead of leaving the stop condition implicit

Output:

- one allowed cleanup envelope: pipeline-owned or compiler-neutral support for the specific shipped template defaults needed by pipeline catalog/runtime proof
- one explicit prohibition on turning this seam into full retained-compiler narrowing
- one verification wall spanning `pipeline_catalog`, pipeline compile/capture/handoff proof, compiler authoring proof, and workspace compile health
- one explicit justification that `pipeline_loader`, `pipeline_route_resolution`, and `pipeline_state_store` stay adjacent evidence for loading/selection and route-state coverage rather than new mandatory wall items because they do not import compiler template-library support
- one explicit stop point for the new triplet
- one statement that implementation, packet-prompt authoring for later execution, production edits, publication, crates.io work, Substrate consumption, and integration implementation remain blocked until a human separately reviews this triplet and explicitly approves a later execution packet

## Risks And Mitigations

### Risk: the plan accidentally blesses the full current `handbook-pipeline` surface

Mitigation:

- repeat the Packet 2 boundary text at the top of the plan
- define the reviewed importer boundary by included surface families and explicit exclusions
- keep `setup` explicitly outside this seam’s boundary

### Risk: the plan widens into CLI shell/support clarification

Mitigation:

- treat `setup`, `doctor`, prompting, operator wording, rendering, refusal, and exit-code policy as explicit non-goals for this seam
- keep CLI shell/support clarification named as a different downstream seam

### Risk: the plan silently turns into retained `handbook-compiler` narrowing

Mitigation:

- limit the cleanup target to removing or relocating the pipeline-side compiler-backed fixture/support coupling
- require compiler authoring proof to stay green rather than planning broad compiler retirement
- keep broader retained-compiler reassignment/retirement work in its own later seam

### Risk: the plan over-specifies one implementation tactic before repo truth forces it

Mitigation:

- define the required outcome instead of prematurely hard-coding one code move
- allow either pipeline-owned or compiler-neutral support for the specific shipped template defaults, as long as the result removes the pipeline-side compiler dependency without widening scope

### Risk: later work mistakes this triplet for execution approval

Mitigation:

- preserve the human review gate in the spec, plan, and tasks docs
- state explicitly that no implementation, publication, crates.io, Substrate consumption, or integration work starts from this triplet alone

## Verification Checkpoints

### Checkpoint 1: Current seam evidence is frozen correctly

Confirm:

- `crates/pipeline/src/lib.rs` still exposes the broader current public surface
- `pipeline_catalog.rs` still imports `handbook_compiler::author::template_library::{resolve_shipped_template_library, TemplateLibraryRequest, TemplateLibrarySelection}`
- `handbook-pipeline` still carries the bounded compiler edge only as a dev-dependency rather than a runtime owner

Suggested verification:

```bash
rg -n "pub use|pub mod|mod " crates/pipeline/src/lib.rs
rg -n "template_library|resolve_shipped_template_library|TemplateLibraryRequest|TemplateLibrarySelection" crates/pipeline/tests/pipeline_catalog.rs
cargo tree -p handbook-pipeline
cargo test -p handbook-pipeline --test pipeline_catalog
```

### Checkpoint 2: Planned importer boundary is explicit enough

Confirm the triplet states all of the following:

- `handbook-pipeline` remains handbook-owned
- only the reviewed supported-target importer boundary is durable
- this seam owns the pipeline-side `template_library` / compiler-backed fixture-support decoupling question
- reviewed supported-target importer boundary
- loading/selection surfaces
- compile surfaces
- capture surfaces
- handoff surfaces
- route-state surfaces
- `setup` stays outside this seam’s reviewed importer boundary
- CLI/product-shell helpers stay outside this boundary
- compiler-routed compatibility helpers stay outside this boundary
- the full current public surface is not approved as the durable importer contract

Suggested verification:

```bash
rg -n "handbook-owned|reviewed supported-target importer boundary|template_library|compiler-backed fixture/support coupling" docs/specs/handbook-engine-extraction-phase-6-handbook-pipeline-boundary-cleanup-{spec,plan,tasks}.md
rg -n "loading/selection|compile surfaces|capture surfaces|handoff surfaces|route-state surfaces|setup stays outside|CLI/product-shell helpers|compiler-routed compatibility helpers" docs/specs/handbook-engine-extraction-phase-6-handbook-pipeline-boundary-cleanup-{spec,plan,tasks}.md
```

### Checkpoint 3: Planned cleanup remains bounded

Confirm the triplet states all of the following:

- `pipeline_catalog` coupling is the concrete cleanup target
- pipeline-side `template_library` / compiler-backed fixture-support decoupling belongs to this seam
- later implementation must remove or relocate the pipeline-side compiler-backed fixture/support dependency needed by pipeline catalog/runtime proof
- retained `handbook-compiler` retirement stays out
- broader authoring-stack relocation stays out
- CLI shell/support reassignment stays out

Suggested verification:

```bash
rg -n "remove or relocate|pipeline catalog/runtime proof|retained handbook-compiler retirement|authoring-stack relocation|CLI shell/support" docs/specs/handbook-engine-extraction-phase-6-handbook-pipeline-boundary-cleanup-{spec,plan,tasks}.md
```

### Checkpoint 4: Future implementation verification wall and review stop are ready

Confirm the triplet records the future implementation proof wall:

```bash
cargo test -p handbook-pipeline --test pipeline_catalog
cargo test -p handbook-pipeline --test pipeline_compile
cargo test -p handbook-pipeline --test pipeline_capture
cargo test -p handbook-pipeline --test pipeline_handoff
cargo test -p handbook-compiler --test author
cargo check --workspace
```

Confirm the triplet also explains why `pipeline_loader`, `pipeline_route_resolution`, and `pipeline_state_store` remain adjacent evidence instead of mandatory seam-specific wall entries: they cover loading/selection and route-state behavior, but live repo truth does not show them importing compiler template-library support, so `pipeline_catalog` remains the only known compiler-backed coupling proof inside this seam-specific wall.

Confirm the triplet also states plainly that it is planning-only and that implementation, packet-prompt authoring for later execution, production edits, publication, crates.io work, Substrate consumption, and integration implementation remain blocked until a human separately reviews this triplet and explicitly approves a later execution packet.

## Exit Conditions

This triplet is ready for human review when:

- the rationale for planning `handbook-pipeline` next is explicit and grounded in landed Phase 6 docs
- the reviewed importer boundary is specific enough that later packets can tell what is in and out
- the Packet 2 freeze distinguishes the live evidence ledger, retained compiler context, bounded cleanup target, and explicit non-goals clearly enough that later packets can remove the coupling without reopening adjacent seams
- the verification wall covers both pipeline proof and retained compiler authoring non-regression
- the final stop condition is explicit: this triplet is planning-only and awaits separate human approval before implementation

## Human Review Gate

This triplet is planning-only and stops at review. Implementation, packet-prompt authoring for later execution, production edits, publication, crates.io work, Substrate consumption, and integration implementation remain blocked until a human separately reviews this triplet and explicitly approves a later execution packet.
