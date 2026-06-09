# Tasks: Handbook Engine Extraction Phase 1 Slice 1 - Layout Contract And Inventory

Plan reference: [handbook-engine-extraction-phase-1-slice-1-layout-contract-and-inventory-plan.md](./handbook-engine-extraction-phase-1-slice-1-layout-contract-and-inventory-plan.md)

## Packet 1.1.1: Layout Type Family And Ownership Boundary

Packet 1.1.1 is the Ownership domain freeze for Slice 1.1: keep separate layout types rather than one global layout object, allow no caller migration in Slice 1.1, and hand Canonical root layout to Slice 1.2, Runtime state layout / Capture provenance layout / Handoff bundle layout to Slice 1.3, and Authoring layout to Slice 1.4.

- [ ] Task: Freeze the layout ownership domains in the slice authority docs
  - Acceptance: The spec defines separate ownership domains for canonical root, runtime state, capture provenance, handoff bundle, and authoring paths/locks, and it explicitly rejects a single global layout object.
  - Verify: `rg -n "separate layout types|Canonical root layout|Runtime state layout|Capture provenance layout|Handoff bundle layout|Authoring layout" docs/specs/handbook-engine-extraction-phase-1-slice-1-layout-contract-and-inventory-spec.md`
  - Files: `docs/specs/handbook-engine-extraction-phase-1-slice-1-layout-contract-and-inventory-spec.md`

- [ ] Task: Freeze the Slice 1.1 no-migration boundary across the authority set
  - Acceptance: The spec, plan, and tasks docs all say Slice 1.1 does contract-and-inventory work only, with caller adoption deferred to Slices 1.2, 1.3, and 1.4.
  - Verify: `rg -n "no caller migration|Slice 1\.2|Slice 1\.3|Slice 1\.4|Out of scope" docs/specs/handbook-engine-extraction-phase-1-slice-1-layout-contract-and-inventory-spec.md docs/specs/handbook-engine-extraction-phase-1-slice-1-layout-contract-and-inventory-plan.md docs/specs/handbook-engine-extraction-phase-1-slice-1-layout-contract-and-inventory-tasks.md`
  - Files: `docs/specs/handbook-engine-extraction-phase-1-slice-1-layout-contract-and-inventory-spec.md`, `docs/specs/handbook-engine-extraction-phase-1-slice-1-layout-contract-and-inventory-plan.md`, `docs/specs/handbook-engine-extraction-phase-1-slice-1-layout-contract-and-inventory-tasks.md`

## Packet 1.1.2: Storage Inventory And Temporary-Exception Freeze

- [ ] Task: Build the reusable-internal inventory table for the targeted compiler corpus
  - Acceptance: The spec contains one row per targeted compiler file, classifies the current `.handbook/**` or `.handbook/state/**` ownership shape, and maps each row to its follow-on adoption slice.
  - Verify: `rg -n "\.handbook|\.handbook/state" crates/compiler/src/canonical_artifacts.rs crates/compiler/src/route_state.rs crates/compiler/src/pipeline_capture.rs crates/compiler/src/pipeline_handoff.rs crates/compiler/src/stage_10_feature_spec_provenance.rs crates/compiler/src/setup.rs crates/compiler/src/author/charter.rs crates/compiler/src/author/project_context.rs crates/compiler/src/author/environment_inventory.rs`
  - Files: `docs/specs/handbook-engine-extraction-phase-1-slice-1-layout-contract-and-inventory-spec.md`

- [ ] Task: Freeze explicit exclusions and temporary exceptions
  - Acceptance: The authority docs explicitly mark CLI/product-shell references as excluded, `pipeline_handoff.rs` as an indirect/no-direct-hit dependency for this slice, and non-`.handbook` future owners as acknowledged but outside the primary verifier corpus.
  - Verify: `rg -n "product-shell exclusion|indirect dependency|temporary exceptions|non-.handbook|no direct \\.handbook" docs/specs/handbook-engine-extraction-phase-1-slice-1-layout-contract-and-inventory-spec.md docs/specs/handbook-engine-extraction-phase-1-slice-1-layout-contract-and-inventory-plan.md docs/specs/handbook-engine-extraction-phase-1-slice-1-layout-contract-and-inventory-tasks.md`
  - Files: `docs/specs/handbook-engine-extraction-phase-1-slice-1-layout-contract-and-inventory-spec.md`, `docs/specs/handbook-engine-extraction-phase-1-slice-1-layout-contract-and-inventory-plan.md`, `docs/specs/handbook-engine-extraction-phase-1-slice-1-layout-contract-and-inventory-tasks.md`

## Human Review Gate

Do not start Slice 1.2 implementation work until the human has reviewed and approved this Slice 1.1 spec/plan/tasks set.
