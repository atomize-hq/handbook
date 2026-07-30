# HCM-2.4 P6 pipeline-handoff provenance test selector amendment

Status: expanded additive amendment pending different-fresh v1.4 closure

Recorded: 2026-07-29

Parent selector: `20260729-p6-aggregate-flow-cleanup-selector.md`

Active packet: `HCM-2.4 P6 aggregate-flow cleanup`

## Trigger and additive authority

The proportional workspace wall exposed two mirrored proof gaps outside the
original P6 19-test-path manifest. The compiler and pipeline pipeline-handoff
stale-provenance tests still mutate the retired legacy Charter Markdown decoy.
The descriptor-selected P6 flow correctly ignores that decoy, so neither test
exercises canonical provenance drift.

The operator authorizes this record as an additive P6 selector amendment. It
does not replace or rewrite the immutable P6 selector or any prior dispatch.
All original P6 production, test, ancillary, risk, stop, and non-goal ceilings
remain in force except for the exact test-only allowance below.

## Discovery findings and expanded authority

The fresh review dispatched as
`20260730T021416Z--HCM-2-4--p6-pipeline-handoff-selector-amendment-review`
returned two P2 findings. The parent consolidates them under these exact stable
IDs for closure lineage:

- `HCM-2.4-P6-PROVENANCE-DUPLICATE-TEST-001`: the mirrored pipeline-crate test
  has the same obsolete mutation target and independently fails the workspace
  wall;
- `HCM-2.4-P6-PROVENANCE-DRIFT-TRUTH-001`: the selected Charter YAML is absent
  at bundle emission, so the authorized test proves post-emission
  missing-to-present canonical provenance drift, not existing-byte tampering.

The operator expands the P6 test ceiling only enough to remediate those two
findings and explicitly accepts the corrected missing-to-present proof truth.

## Exact ancillary allowance

The frozen allowance is exactly two additional test paths and two mutation
expressions:

- path: `crates/compiler/tests/pipeline_handoff.rs`
- test: `handoff_validation_refuses_stale_canonical_provenance`
- before: `repo_root.join(".handbook/charter/CHARTER.md")`
- after: `repo_root.join(".handbook/project/charter.yaml")`
- path: `crates/pipeline/tests/pipeline_handoff.rs`
- test: `handoff_validation_refuses_stale_canonical_provenance`
- before: `repo_root.join(".handbook/charter/CHARTER.md")`
- after: `repo_root.join(".handbook/project/charter.yaml")`
- classification: test assertion/proof mutation only
- risk ceiling: LOW
- expression ceiling: one replaced mutation expression per path

The committed legacy Charter Markdown installed by `install_canonical_inputs`
remains an intentional non-authoritative decoy. No setup helper, fixture,
production code, public API, schema, definition, profile, renderer, other
legacy-path assertion, or additional path may change under this amendment.

The selected `.handbook/project/charter.yaml` path is absent when each bundle
is emitted. Writing it afterward is the canonical missing-to-present state
transition that must invalidate the emitted provenance snapshot. It is not an
existing-byte tamper proof. Existing-byte mutation remains covered by
`manifest_from_snapshot_keeps_pre_mutation_identity` in
`crates/engine/tests/artifact_manifest_interface.rs`.

## Pre-edit gates

Before either mutation expression changes:

1. GitNexus upstream impact for both exact test functions must report LOW risk
   with no unexplained production caller, process, or module.
2. A different-fresh bounded read-only v1.4 closure review must name both exact
   P2 IDs and return CLEAN over this remediated subject.
3. Any need to change `install_canonical_inputs`, a fixture, production code,
   another assertion, a third path, or more than the two exact expressions
   stops the amendment.

Impact replay after expanded authority: GitNexus reported exact LOW risk for
both test functions. Each has zero direct callers, zero affected processes,
and zero affected modules. The allowance is therefore frozen at two test
paths, two replaced mutation expressions, LOW risk, and zero
production/API/helper/fixture changes. No test edit is authorized until the
different-fresh closure returns CLEAN.

## Required proof and integration

After the reviewed edit:

- each focused test must return
  `PipelineHandoffValidationFailureClassification::StaleCanonicalProvenance`;
- the complete compiler and pipeline `pipeline_handoff` targets must pass;
- affected canonical-provenance consumers must pass;
- the existing P6 convergence and proportional workspace wall must resume;
- both paths must be included in the final P6 implementation subject manifest
  and independent implementation review;
- the result must be described as post-emission missing-to-present canonical
  provenance drift, while existing-byte mutation coverage remains attributed
  to `manifest_from_snapshot_keeps_pre_mutation_identity`.

This is a proof-gap and manifest-scope correction inside the active P6 loop.
It does not select P7 or authorize Phase 2, HCM-3.x, task-gate runtime,
automatic continuation, push, or release.
