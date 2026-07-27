# HCM-2.4 P3B M5 canonical-consumer selector repair

Status: **proposed exact test/evidence amendment; implementation paused for
fresh review**

Date: 2026-07-27

## Trigger

After the reviewed foundation feature-identity repair, six focused P3B
behaviors passed. The M5 journey advanced through capture and handoff emission,
then the test-only bundle consumer failed:

```text
missing handoff input for `artifacts/feature_spec/FEATURE_SPEC.md`
```

The live P3 handoff correctly includes
`artifacts/work-specification/work-specification.yaml` as the authoritative
external-manual input and excludes its deterministic Markdown view. The M5
consumer harness and evidence still parse the old view as authority. This is a
test/evidence consumer gap, not a production defect.

## Exact selector amendment

Inside the already selected `crates/cli/tests/cli_surface.rs`, edits are limited
to:

1. `collect_repo_reread_planning_inputs`, which must reread canonical Work
   Specification YAML instead of the Markdown view;
2. `run_bundle_only_feature_slice_consumer_harness`, which must read the
   allowlisted canonical Work Specification YAML input from the handoff bundle;
3. `build_planning_inputs`, which must use the existing public
   `handbook_engine::canonical_yaml::parse_canonical_yaml` and obtain only
   `objective`, `scope`, and `acceptance_criteria` from the admitted YAML
   object; and
4. local test-variable names and exact assertions required by those changes.

No Cargo or support-module edit is required because `handbook-cli` already
depends on `handbook-engine`.

Add these exact evidence paths:

- `tests/fixtures/foundation_flow_demo/expected/happy_path/final_feature_spec.md`
- `tests/fixtures/foundation_flow_demo/expected/skip_path/final_feature_spec.md`
- `tests/fixtures/foundation_flow_demo/expected/happy_path/SLICE_PLAN.md`
- `tests/fixtures/foundation_flow_demo/evidence/m5_handoff_scorecard.md`

The two already selected foundation-flow Work Specification inputs may now
replace their generic content with the same exact M4 proof record:

```yaml
acceptance_criteria:
  - "AC-001: A CLI happy-path test resolves, captures stages 04/05/06/07, compiles stage 10, captures one completed Work Specification YAML document, and writes the canonical YAML plus deterministic Markdown view."
  - "AC-002: The happy-path canonical Work Specification YAML exactly matches the admitted external model output and its Markdown view exactly matches the committed renderer golden."
  - "AC-003: A CLI skip-path test proves stage 06 is skipped because needs_project_context=false and charter_gaps_detected=false."
  - "AC-004: No stage-10 success-path test captures raw compile payload."
  - "AC-005: Docs and proof drift checks fail if stage 10 is described as direct compile-to-capture."
non_goals: []
objective: "Build the M4 proof wedge for one pipeline.foundation_inputs journey using staged external model output, canonical Work Specification YAML capture, and a deterministic Markdown view. A credible alternative is to retain Markdown as Stage 10 authority; that is simpler for manual editing but forfeits schema-selected validation, stable canonical identity, and byte-exact handoff provenance."
record_id: "fs.m4.foundation.journey-2026-04"
schema_id: "handbook.artifact.work-specification"
schema_version: "1.0"
scope:
  - "G1: Prove a believable happy path that reaches stage 10 only after stage 06 and stage 07 complete."
  - "G2: Prove a believable skip path that leaves stage 06 skipped because both activation predicates are false."
  - "G3: Lock docs and proof to the same canonical Work Specification handoff contract."
status: "approved"
```

The two generated Markdown goldens must be mechanically regenerated from this
record. The expected slice plan may change only where its objective, G1-G3,
AC-001 through AC-005, and canonical/view terms derive from that Work
Specification. The M5 scorecard may change only its canonical source and bundle
paths. The already selected happy/skip transcripts may be mechanically
regenerated after the focused journeys pass.

## Reviewability and non-goals

This repair changes three private test helpers, two input fixtures, four
expected/evidence files, and the already selected transcripts. It adds no
production symbol, runtime behavior, handoff input, renderer behavior, schema,
released definition, public API, dependency, Cargo/version, unsafe policy, or
sibling slice. The shared pipeline proof corpus remains unchanged.

The foundation-flow stage 04-07 inputs, base/Charter/Project Context/Foundation
documents, main core assets, and non-M5 CLI tests are not selected. Historical
upstream wording in those fixture inputs is context rather than authority for
the P3 canonical handoff.

Entry requires fresh built-in review. Exit remains the eight focused tests, all
98 `cli_surface` tests, P3 packet wall, workspace wall, formatting, clippy,
archive boundary, diff checks, and different-fresh implementation review.
