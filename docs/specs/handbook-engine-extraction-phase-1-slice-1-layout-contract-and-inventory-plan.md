# Plan: Handbook Engine Extraction Phase 1 Slice 1 - Layout Contract And Inventory

## Objective

Freeze the layout type-family contract and complete the reusable-internal storage inventory that later Phase 1 slices will adopt.

Spec reference: [handbook-engine-extraction-phase-1-slice-1-layout-contract-and-inventory-spec.md](./handbook-engine-extraction-phase-1-slice-1-layout-contract-and-inventory-spec.md)

## Major Artifacts

1. Slice-local layout contract
   - lives in the Slice 1.1 spec/plan/tasks authority set
   - defines ownership domains without migrating callers yet

2. Reusable-internal compiler inventory
   - records the current `.handbook/**` and `.handbook/state/**` assumptions for the targeted compiler files
   - maps each assumption to a future adoption slice

3. Exclusion and temporary-exception ledger
   - records CLI/product-shell exclusions
   - records no-hit but still relevant files such as `pipeline_handoff.rs`
   - records any future-layout owners that are outside the primary verifier corpus

4. Slice handoff contract
   - gives Slice 1.2 the canonical/setup adoption inputs
   - gives Slice 1.3 the runtime-state/capture/handoff adoption inputs
   - gives Slice 1.4 the authoring adoption inputs

## Dependencies And Order

### Packet 1.1.1 first: Freeze the layout type-family contract

Why first:

- the inventory needs a stable ownership vocabulary
- later slices should not invent their own local path-owner names
- the root plan already locked in “separate layout types rather than one global layout object,” so Slice 1.1 must make that operational before inventory freeze

Output:

- one approved Ownership domain list covering Canonical root layout, Runtime state layout, Capture provenance layout, Handoff bundle layout, and Authoring layout
- one explicit “no caller migration in Slice 1.1” rule
- one mapping from owner domains to future Phase 1 slices: Slice 1.2 for canonical/setup adoption, Slice 1.3 for runtime-state/capture/handoff adoption, and Slice 1.4 for authoring adoption

### Packet 1.1.2 second: Freeze the reusable-internal storage inventory

Why second:

- the inventory is only useful after the ownership domains are stable
- exclusions and no-hit files are easier to classify once the contract exists
- later slices need a durable corpus, not a fresh `rg` pass every time

Output:

- one inventory table covering the targeted compiler corpus
- one explicit exclusion record for CLI/product-shell references
- one explicit temporary-exception record for indirect or non-primary verifier items

## Risks And Mitigations

### Risk: the contract drifts back into one global layout object

Mitigation:

- freeze the “separate layout types” decision in Slice 1.1 itself
- reject any packet language that collapses all storage ownership into one monolithic layout owner

### Risk: inventory misses references

Mitigation:

- verify the targeted compiler corpus with the slice `rg` command
- require one row per target file, even if the row is “no direct hit / indirect dependency”

### Risk: CLI/product-shell references get mixed into reusable-internal ownership

Mitigation:

- keep `crates/cli/src/main.rs` in the auxiliary context corpus only
- require explicit exclusion language in the slice authority docs

### Risk: Slice 1.1 widens into migration work

Mitigation:

- treat all adoption work as deferred to Slices 1.2, 1.3, and 1.4
- allow only docs and, if later needed, behavior-neutral contract scaffolding with no caller adoption

### Risk: future slices lose the handoff bundle owner because it is not in the primary `.handbook` corpus

Mitigation:

- keep `pipeline_handoff.rs` in the inventory with an explicit indirect-dependency classification
- preserve the handoff-bundle owner in the layout-domain table even when Packet 1.1.2 focuses on `.handbook/**`

## Parallel Vs Sequential

Sequential:

- Packet 1.1.1 before Packet 1.1.2
- ownership-domain freeze before inventory freeze
- exclusions after the main inventory corpus has been classified

Not parallel:

- do not split contract naming and inventory ownership mapping across simultaneous packets
- do not begin Slice 1.2, 1.3, or 1.4 adoption planning from stale or partial Slice 1.1 output

## Verification Checkpoints

### Checkpoint 1: Contract freeze complete

Confirm the slice authority docs contain:

- the separate-layout-types rule
- the ownership-domain table
- the explicit “no caller migration in Slice 1.1” rule

Suggested verification:

```bash
rg -n "separate layout types|no caller migration|Ownership domain" \
  docs/specs/handbook-engine-extraction-phase-1-slice-1-layout-contract-and-inventory-spec.md \
  docs/specs/handbook-engine-extraction-phase-1-slice-1-layout-contract-and-inventory-plan.md \
  docs/specs/handbook-engine-extraction-phase-1-slice-1-layout-contract-and-inventory-tasks.md
```

### Checkpoint 2: Inventory freeze complete

Confirm every targeted compiler file is represented and the corpus still matches live repo truth.

Suggested verification:

```bash
rg -n "\.handbook|\.handbook/state" \
  crates/compiler/src/canonical_artifacts.rs \
  crates/compiler/src/route_state.rs \
  crates/compiler/src/pipeline_capture.rs \
  crates/compiler/src/pipeline_handoff.rs \
  crates/compiler/src/stage_10_feature_spec_provenance.rs \
  crates/compiler/src/setup.rs \
  crates/compiler/src/author/charter.rs \
  crates/compiler/src/author/project_context.rs \
  crates/compiler/src/author/environment_inventory.rs
```

### Checkpoint 3: Exclusions and temporary exceptions explicit

Confirm the docs explicitly record:

- CLI/product-shell exclusion
- `pipeline_handoff.rs` as an indirect dependency / no-direct-hit file
- non-`.handbook` future owners as acknowledged but not primary verifier items for this packet

Suggested verification:

```bash
rg -n "product-shell exclusion|indirect dependency|temporary exceptions|non-.handbook" \
  docs/specs/handbook-engine-extraction-phase-1-slice-1-layout-contract-and-inventory-spec.md \
  docs/specs/handbook-engine-extraction-phase-1-slice-1-layout-contract-and-inventory-plan.md \
  docs/specs/handbook-engine-extraction-phase-1-slice-1-layout-contract-and-inventory-tasks.md
```

## Exit Conditions

Slice 1.1 is ready for human review when:

- the layout ownership domains are frozen clearly enough that later slices can consume them without renaming the contract
- the reusable-internal compiler corpus has a durable inventory row for every targeted file
- exclusions and temporary exceptions are explicit
- the docs make clear that no caller adoption belongs to Slice 1.1
- the output is ready to hand off to Slice 1.2, 1.3, and 1.4 without rediscovery work

Slice 1.1 is ready for implementation only after the human reviews and accepts the spec/plan/tasks set.
