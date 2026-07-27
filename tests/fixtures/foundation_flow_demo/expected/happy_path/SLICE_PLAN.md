# Slice Plan
- Feature ID: `fs-m4-foundation-journey-2026-04`
- Handoff Target: `pipeline.foundation_inputs` -> `feature-slice-decomposer`
- Bundle Root: `artifacts/handoff/feature_slice/fs-m4-foundation-journey-2026-04`
- Repo Reread Fallback: disabled

## Planning Intent
Build the M4 proof wedge for one pipeline.foundation_inputs journey using staged external model output, canonical Work Specification YAML capture, and a deterministic Markdown view. A credible alternative is to retain Markdown as Stage 10 authority; that is simpler for manual editing but forfeits schema-selected validation, stable canonical identity, and byte-exact handoff provenance.

## Evidence Pack
- Goals:
  - G1: Prove a believable happy path that reaches stage 10 only after stage 06 and stage 07 complete.
  - G2: Prove a believable skip path that leaves stage 06 skipped because both activation predicates are false.
  - G3: Lock docs and proof to the same canonical Work Specification handoff contract.
- Strategy pillars:
  - Keep the M4 demo corpus separate from the shared proof corpus and from `execution_demo`.
  - Preserve deterministic evidence surfaces for reruns by fixing `now_utc` and normalizing capture ids.
  - Keep operator-facing docs aligned with the shipped handoff contract.
- Journey anchors:
  - Manual state handoff sets `needs_project_context=true`, then resolve activates stage 06.
  - Stage 10 compile produces payload for an external model.
  - Stage 10 capture writes `FEATURE_SPEC.md` only after a completed external response exists.
- Mandatory gates:
  - targeted CLI journey tests for happy and skip paths
  - targeted compiler and CLI stage-10 capture tests using completed feature-spec outputs
  - docs/help drift guard coverage for the stage-10 external-model boundary
  - deterministic rerun validation with fixed clock input

## Proposed Slices
### Slice 1: Route Journey Proof
- Objective: G1: Prove a believable happy path that reaches stage 10 only after stage 06 and stage 07 complete.
- Acceptance:
  - AC-001: A CLI happy-path test resolves, captures stages 04/05/06/07, compiles stage 10, captures one completed Work Specification YAML document, and writes the canonical YAML plus deterministic Markdown view.
  - AC-003: A CLI skip-path test proves stage 06 is skipped because needs_project_context=false and charter_gaps_detected=false.
- Grounding:
  - Manual state handoff sets `needs_project_context=true`, then resolve activates stage 06.
  - targeted CLI journey tests for happy and skip paths
  - targeted compiler and CLI stage-10 capture tests using completed feature-spec outputs
- Deliverable: keep the happy path and skip path evidence truthful through stage 07 before the external-model boundary.

### Slice 2: Stage-10 Handoff Boundary
- Objective: G3: Lock docs and proof to the same canonical Work Specification handoff contract.
- Acceptance:
  - AC-002: The happy-path canonical Work Specification YAML exactly matches the admitted external model output and its Markdown view exactly matches the committed renderer golden.
  - AC-004: No stage-10 success-path test captures raw compile payload.
- Grounding:
  - Stage 10 compile produces payload for an external model.
  - Stage 10 capture writes `FEATURE_SPEC.md` only after a completed external response exists.
  - docs/help drift guard coverage for the stage-10 external-model boundary
- Deliverable: preserve payload-only compile and completed-output capture with no raw compile payload success path.

### Slice 3: Deterministic Downstream Adoption
- Objective: turn the emitted bundle into a bounded planning artifact without repo rereads.
- Acceptance:
  - AC-005: Docs and proof drift checks fail if stage 10 is described as direct compile-to-capture.
- Grounding:
  - Preserve deterministic evidence surfaces for reruns by fixing `now_utc` and normalizing capture ids.
  - Keep operator-facing docs aligned with the shipped handoff contract.
  - deterministic rerun validation with fixed clock input
- Deliverable: validate the emitted bundle, write `artifacts/planning/feature_slice/fs-m4-foundation-journey-2026-04/SLICE_PLAN.md`, and keep bundle-only reads sufficient for the same planning job.

## Sequence
1. Reconfirm the route-journey proof so the happy path and skip path remain believable.
2. Lock the stage-10 handoff boundary so compile stays payload-only and capture consumes completed external output.
3. Emit, validate, and consume the handoff bundle to produce the downstream slice plan without repo rereads.
