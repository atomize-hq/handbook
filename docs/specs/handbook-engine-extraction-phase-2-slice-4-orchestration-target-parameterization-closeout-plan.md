# Plan: Handbook Engine Extraction Phase 2 Slice 4 (Slice 2.4) - Orchestration Target Parameterization Closeout

## Objective

Close the remaining Phase 2 orchestration-target parameterization gap by removing singleton hardcoded ownership of the currently supported pipeline/stage/consumer wedge while preserving current behavior and explicitly deferring generalized consumer-platform work.

Spec reference: [handbook-engine-extraction-phase-2-slice-4-orchestration-target-parameterization-closeout-spec.md](./handbook-engine-extraction-phase-2-slice-4-orchestration-target-parameterization-closeout-spec.md)

## Major Artifacts

1. Catalog-backed pipeline/stage target owner closeout
   - keeps declarative `core/pipelines/**` and `core/stages/**` truth authoritative
   - removes singleton pipeline/stage owner constants as the de facto runtime source of truth

2. Bounded default-consumer owner
   - keeps the current consumer code-owned and validated
   - removes scattered `feature-slice-decomposer` ownership from handoff and CLI surfaces

3. Runtime and CLI target-posture alignment
   - keeps compile/capture/provenance/handoff using shared target ownership
   - keeps CLI/help/producer-command surfaces sourced from the same bounded target posture without widening into Phase 5 shell redesign

4. Closeout verification wall
   - proves the currently supported wedge remains unchanged
   - explicitly records that multi-consumer platforms, new consumer catalogs, and CLI shell finish work remain deferred

## Order

### Packet 2.4.1: Close the pipeline/stage target owner around declarative catalog truth

Why first:

- the root closeout gap starts with pipeline/stage target ownership still encoded as singleton Rust constants
- handoff and CLI cleanup should consume one shared target owner rather than invent their own interpretations
- keeping pipeline/stage truth authoritative in catalog data is the narrowest way to de-hardcode the current wedge without expanding supported scope

Output:

- one shared runtime target owner whose supported pipeline/stage posture is driven by declarative catalog truth
- compile/capture/provenance adopters still bounded to the current wedge, but no longer treated as owners of duplicated singleton pipeline/stage ids
- updated catalog and adopter regression coverage proving behavior stability

### Packet 2.4.2: Consolidate bounded default-consumer ownership

Why second:

- `feature-slice-decomposer` remains the most obvious remaining root-plan literal and is still duplicated across handoff and CLI surfaces
- consumer ownership should be settled once, after pipeline/stage ownership is already centralized
- this packet can stay narrow by preserving the current consumer while eliminating scattered literals

Output:

- one code-owned validated default-consumer owner for the bounded current consumer
- handoff runtime logic that consumes that owner rather than re-owning direct consumer/stage literals
- targeted handoff regression coverage proving the bounded consumer posture remains unchanged

### Packet 2.4.3: Align CLI/help and producer-command surfaces to shared target ownership

Why third:

- once runtime ownership is settled, CLI/product surfaces can safely stop hardcoding examples and producer commands
- this keeps the closeout honest without widening into broad Phase 5 shell redesign
- user-facing help/examples are the remaining places where singleton ownership can still hide after runtime cleanup

Output:

- CLI help/examples that source supported-target posture from shared owner data or thin product-shell adapters backed by that data
- producer-command rendering that no longer hardcodes the handoff emit command around product ids
- CLI regression coverage proving public behavior remains stable

### Packet 2.4.4: Final closeout proof and explicit deferral ledger

Why last:

- the slice should not be called complete until targeted tests and the workspace wall prove the wedge stayed stable
- explicit deferrals are required to prevent future agents from reinterpreting this closeout as permission to add multi-consumer or CLI-shell redesign work

Output:

- one final verification wall for catalog, compile, capture, handoff, CLI, and workspace behavior
- one explicit deferred-scope ledger covering multi-consumer platforms, consumer catalogs, new supported targets, and Set 4 CLI shell cleanup

## Risks And Mitigations

### Risk: the closeout over-generalizes into a multi-consumer platform

Mitigation:

- keep the supported wedge unchanged
- keep consumer truth code-owned and validated
- require ask-first approval for any `core/consumers/**` or user-editable consumer config

### Risk: pipeline/stage truth becomes split between catalog data and new Rust ownership

Mitigation:

- treat `core/pipelines/**` and `core/stages/**` as the only source of pipeline/stage identity truth
- reject implementations that merely move the same constants to a second Rust module without reducing ownership duplication

### Risk: CLI/help cleanup leaks into Phase 5 shell redesign

Mitigation:

- limit CLI work to sourcing existing examples/commands from shared target ownership
- avoid command-taxonomy, copywriting, or shell-flow redesign in this slice

### Risk: the supported wedge changes while de-hardcoding

Mitigation:

- guard every packet with targeted regression tests
- keep verification centered on the current compile/capture/handoff wedge and current refusal posture

### Risk: runtime and CLI surfaces both remain owners after the packet series

Mitigation:

- require a shared bounded target owner before CLI/help cleanup
- verify remaining literals explicitly with the evidence sweep before calling the slice complete

## Verification Checkpoints

### Checkpoint 1: Remaining-literal evidence is fully mapped before edits

Confirm live repo evidence for:

- singleton supported pipeline/stage owner constants
- bounded consumer literals
- CLI help and producer-command literals

Suggested verification:

```bash
rg -n "SUPPORTED_PIPELINE_TARGET_ID|SUPPORTED_COMPILE_STAGE_TARGET_ID|SUPPORTED_CAPTURE_STAGE_TARGET_IDS|SUPPORTED_CONSUMER_TARGET_ID|SUPPORTED_CONSUMER_ID|SUPPORTED_STAGE_ID|pipeline\\.foundation_inputs|stage\\.10_feature_spec|feature-slice-decomposer" crates/pipeline/src crates/cli/src
```

### Checkpoint 2: Catalog-backed pipeline/stage ownership is live

Confirm targeted catalog and adopter behavior still passes:

```bash
cargo test -p handbook-pipeline --test pipeline_catalog
cargo test -p handbook-pipeline --test pipeline_compile
cargo test -p handbook-pipeline --test pipeline_capture
```

### Checkpoint 3: Bounded consumer ownership is centralized

Confirm handoff and CLI surfaces still pass after consumer de-hardcoding:

```bash
cargo test -p handbook-pipeline --test pipeline_handoff
cargo test -p handbook-cli --test cli_surface
```

### Checkpoint 4: Final closeout wall is green

Confirm the full workspace still passes:

```bash
cargo test --workspace
```

## Exit Conditions

Slice 2.4 is ready for human review when:

- declarative catalog truth is the authoritative owner of supported pipeline/stage ids
- the bounded current consumer is owned in one code-owned validated default owner
- compile/capture/provenance/handoff and adjacent CLI/help surfaces all consume shared target ownership for their supported-target posture
- the currently supported wedge is unchanged under targeted and workspace verification
- deferred scope is explicit enough that future agents do not widen this closeout into multi-consumer or CLI-shell redesign work
