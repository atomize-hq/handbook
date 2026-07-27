# HCM-2.4 exact implementation packet path manifest v1.0

Status: proposed P0 inventory lock

This manifest converts the implementation selector wildcards in `SPEC.md` into
exact repository-relative paths. It is a maximum surface, not a requirement to
edit every path. A path not named here requires a same-scope correction, fresh
GitNexus impact when it contains a code symbol, and parent disposition before
use.

Read/proof-only paths remain non-editable unless their packet explicitly says
otherwise. New fixture roots may contain only the files named below. Generated
review dispatches and the final parent-owned handoff/ledger closeout are
protocol artifacts governed by `08`, not implementation selector expansion.

## P0 — Inventory lock

Read-only discovery inputs:

- `docs/specs/handbook-contract-membrane/slices/HCM-2.4/SPEC.md`
- `docs/specs/handbook-contract-membrane/slices/HCM-2.4/tasks/plan.md`
- `docs/specs/handbook-contract-membrane/slices/HCM-2.4/tasks/todo.md`

P0 control-record outputs authorized only after read-only discovery establishes
their exact evidence:

- `docs/specs/handbook-contract-membrane/slices/HCM-2.4/decision/20260726-p2-selector-correction.md`
- `docs/specs/handbook-contract-membrane/slices/HCM-2.4/contracts/implementation-packet-path-manifest-v1.0.md`
- `docs/specs/handbook-contract-membrane/slices/HCM-2.4/proof/implementation/P0-inventory-lock.md`

The SPEC/plan/todo paths become editable in P0 only for the smallest coupled
repair of a demonstrated contradiction. Every P0 documentation change requires
fresh independent review before the gate passes.

All runtime, definition, template, fixture, and test paths are read-only in P0.

## P1A — Definition support and admission

New immutable definitions:

- `crates/engine/definitions/artifact-kinds/handbook.artifact-kind.project-context/1.1.0.yaml`
- `crates/engine/definitions/artifact-kinds/handbook.artifact-kind.environment-context/1.1.0.yaml`
- `crates/engine/definitions/artifact-kinds/handbook.artifact-kind.work-specification/1.1.0.yaml`
- `crates/engine/definitions/artifact-kinds/handbook.artifact-kind.decision-record/1.1.0.yaml`
- `crates/engine/definitions/artifact-kinds/handbook.artifact-kind.risk-record/1.1.0.yaml`
- `crates/engine/definitions/intakes/handbook.intake.project-context/1.0.0.yaml`
- `crates/engine/definitions/intakes/handbook.intake.environment-context/1.0.0.yaml`
- `crates/engine/definitions/intakes/handbook.intake.work-specification/1.0.0.yaml`
- `crates/engine/definitions/intakes/handbook.intake.decision-record/1.0.0.yaml`
- `crates/engine/definitions/intakes/handbook.intake.risk-record/1.0.0.yaml`
- `crates/engine/definitions/renderers/handbook.renderer.project-context-review-markdown/1.0.0.yaml`
- `crates/engine/definitions/renderers/handbook.renderer.environment-context-review-markdown/1.0.0.yaml`
- `crates/engine/definitions/renderers/handbook.renderer.work-specification-review-markdown/1.0.0.yaml`
- `crates/engine/definitions/renderers/handbook.renderer.decision-record-review-markdown/1.0.0.yaml`
- `crates/engine/definitions/renderers/handbook.renderer.risk-record-review-markdown/1.0.0.yaml`
- `crates/engine/definitions/profiles/handbook.profile.shipped-root/1.2.0.yaml`

Editable runtime:

- `crates/engine/src/profile_builtins.rs`
- `crates/engine/src/artifact_kind_registry.rs`
- `crates/engine/src/artifact_instance.rs`
- `crates/engine/src/artifact_intake_registry.rs`
- `crates/engine/src/artifact_repository.rs`
- `crates/engine/src/profile_selection.rs`
- `crates/engine/src/schema_registry.rs`

The added runtime ownership is exact:

- `artifact_intake_registry.rs`: only the exact P1A typed-closure branch in
  `ArtifactIntakeDefinitionV1::parse`; do not edit
  `ArtifactIntakeRegistry::load_with_builtin_compatibility`;
- `artifact_kind_registry.rs`: only
  `AuthoredArtifactKindDefinition::validate` plus the already selected
  later-owned dependency guard;
- `artifact_repository.rs`: only `load_repository_intakes` built-in
  compatibility selection so the released Charter intake stays
  compatibility-only and the five new intakes do not;
- `profile_selection.rs`: only the uniform shipped-root `1.2` dependency
  closure and its branch in `validate_authored_profile_fingerprints`; do not
  edit `resolve_profile_selection`;
- `schema_registry.rs`: only
  `ResolvedSchema::collect_coverage_leaf_shapes`, and only to classify a
  type-absent leaf with a string-valued `const` as String after the existing
  reference and ambiguity checks. Test-only assertions may be added in its
  existing test module. Every other unsupported or indeterminate shape remains
  refused.

No public signature or other generic repository-defined intake behavior may
change beyond the exact approved type-absent/string-valued-`const` predicate.
This is the fifth and final P1A production symbol; a sixth production symbol,
broader schema inference, or released-schema edit stops the packet.

Editable tests and immutable vectors:

- `crates/engine/tests/artifact_instances.rs`
- `crates/engine/tests/artifact_kind_registry.rs`
- `crates/engine/tests/profile_artifact_schemas.rs`
- `crates/engine/tests/profile_context_schemas.rs`
- `crates/engine/tests/profile_selection.rs`
- `crates/engine/tests/profile_work_decision_schemas.rs`
- `crates/engine/tests/profile_risk_schema.rs`
- `crates/engine/tests/hcm_1_2_selected_kinds.rs`
- `crates/engine/tests/hcm_1_2_unselected_kinds.rs`
- `crates/engine/tests/hcm_1_4_profile_decisions.rs`
- `crates/engine/tests/hcm_1_4_profile_inspection.rs`
- `crates/engine/tests/hcm_2_2_definition_profile.rs`
- `crates/engine/tests/hcm_2_4_definition_support.rs`
- `crates/engine/tests/hcm_2_4_definition_runtime.rs`
- `docs/specs/handbook-contract-membrane/slices/HCM-2.4/contracts/definition-support-vectors-v1.0.json`
- `docs/specs/handbook-contract-membrane/slices/HCM-2.4/contracts/renderer-goldens-v1.0.json`
- `docs/specs/handbook-contract-membrane/slices/HCM-2.4/proof/implementation/P1A-definition-support.md`

Read/proof-only regression:

- `crates/engine/tests/hcm_2_3_registration_kernel.rs`, immutable at
  `sha256:86e6e98fe8fad63578be3d157d3a836c4bfe3a4dc7aa49f1ff41d87e33b61629`
  with a required zero-byte delta.

## P1B — Charter compatibility membrane

Editable runtime:

- `crates/engine/src/charter_definition_registry.rs`
- `crates/engine/src/charter_lifecycle_validation.rs`
- `crates/engine/src/charter_intake.rs`
- `crates/engine/src/charter_approval_workflow.rs`
- `crates/engine/src/charter_promotion_workflow.rs`
- `crates/engine/src/charter_authority_transaction.rs`

Read/proof-only anchors:

- `crates/compiler/src/doctor.rs`
- `crates/engine/src/charter_lineage_store.rs`
- `crates/engine/src/charter_promotion_intent_v12.rs`

Editable tests:

- `crates/engine/tests/hcm_2_4_charter_profile_compatibility.rs`

Read/proof-only regression tests:

- `crates/engine/tests/hcm_2_2_approval_use.rs`
- `crates/engine/tests/hcm_2_2_authenticator_security.rs`
- `crates/engine/tests/hcm_2_2_authority_repair.rs`
- `crates/engine/tests/hcm_2_2_authority_workflow.rs`
- `crates/engine/tests/hcm_2_2_charter.rs`
- `crates/engine/tests/hcm_2_2_charter_observation.rs`
- `crates/engine/tests/hcm_2_2_definition_profile.rs`
- `crates/engine/tests/hcm_2_2_definition_registry.rs`
- `crates/engine/tests/hcm_2_2_intake.rs`
- `crates/engine/tests/hcm_2_2_lifecycle.rs`
- `crates/engine/tests/hcm_2_2_lifecycle_store.rs`
- `crates/engine/tests/hcm_2_2_lineage_store.rs`
- `crates/engine/tests/hcm_2_2_promotion_workflow.rs`
- `crates/engine/tests/hcm_2_2_runtime_vectors.rs`
- `crates/engine/tests/hcm_2_2_test_surface.rs`
- `crates/engine/tests/hcm_2_2_transaction_promotion.rs`
- `crates/compiler/tests/doctor.rs`
- `crates/compiler/tests/hcm_2_2_c04_version.rs`
- `crates/compiler/tests/hcm_2_2_product_cutover.rs`
- `crates/cli/tests/hcm_2_2_product_cutover.rs`
- `crates/cli/tests/hcm_2_2_skill_assets.rs`
- `crates/cli/tests/cli_surface.rs`
- `docs/specs/handbook-contract-membrane/slices/HCM-2.4/proof/implementation/P1B-charter-compatibility.md`

No HCM-2.2 schema, vector, candidate/result, promotion-intent, approval,
lineage, lifecycle, recovery, replay, or committed-authority format is editable.

## P1C — Shipped-root adoption

Editable runtime:

- `crates/engine/src/profile_decision.rs`
- `crates/engine/src/project_context_artifact.rs` — only the existing private
  `selected_contract_matches` predicate and its existing unit test, for the
  exact Project Context kind `1.0`/`1.1` compatibility pair over the unchanged
  schema `1.0` and canonical path; no new production symbol or fallback

Editable tests:

- `crates/engine/tests/hcm_1_2_selected_kinds.rs`
- `crates/engine/tests/hcm_1_2_unselected_kinds.rs`
- `crates/engine/tests/hcm_1_4_profile_decisions.rs`
- `crates/engine/tests/hcm_1_4_profile_inspection.rs`
- `crates/engine/tests/hcm_2_2_definition_profile.rs`
- `crates/engine/tests/hcm_2_4_charter_profile_compatibility.rs`
- `crates/compiler/tests/doctor.rs` — only the report schema-version
  expectation from `1.1.0` to `1.2.0` and the Project Context kind-ref
  expectation from `1.0.0` to selected `1.1.0` in Unix test
  `doctor_api_projects_the_exact_stable_project_context_row`
- `crates/cli/tests/cli_surface.rs` — only replace the obsolete inline Charter
  schema `1.0` setup with the existing `write_valid_selected_charter` helper
  and change the Project Context kind-ref expectation from `1.0.0` to selected
  `1.1.0` in Unix test
  `doctor_reports_ready_when_required_artifacts_present`
- `docs/specs/handbook-contract-membrane/slices/HCM-2.4/proof/implementation/P1C-shipped-root-adoption.md`

`resolve_shipped_profile_decisions` and
`crates/engine/tests/hcm_2_1_project_context.rs` are read/proof-only. The
HCM-2.1 integration target must retain a zero-byte delta and pass 12/12. The
two Unix test allowances do not authorize helper implementation, fixture
assets, any other setup or assertion, production, or sibling behavior change.

## P2 — Environment Context vertical

Editable runtime and library assets:

- `crates/engine/src/author/environment_inventory_core.rs`
- `crates/engine/src/baseline_validation.rs`
- `crates/engine/src/canonical_artifacts.rs`
- `crates/engine/src/canonical_paths.rs`
- `crates/engine/src/lib.rs`
- `crates/compiler/src/author/environment_inventory.rs`
- `crates/compiler/src/author/environment_inventory_shell.rs`
- `crates/compiler/src/author/mod.rs`
- `crates/compiler/src/layout.rs`
- `crates/compiler/src/baseline_validation.rs`
- `crates/compiler/src/template_library.rs`
- `crates/compiler/src/lib.rs`
- `crates/flow/src/resolver.rs`
- `crates/cli/src/author.rs`
- `crates/cli/src/main.rs`
- `crates/cli/src/rendering.rs`
- `core/library/environment_inventory/ENVIRONMENT_INVENTORY_INPUTS.yaml.tmpl`
- `core/library/environment_inventory/environment_inventory_directive.md`
- `core/library/environment_inventory/ENVIRONMENT_INVENTORY.md.tmpl`

Editable tests:

- `crates/engine/tests/author_core.rs`
- `crates/engine/tests/artifact_manifest_interface.rs`
- `crates/engine/tests/baseline_validation.rs`
- `crates/engine/tests/canonical_artifacts_ingest.rs`
- `crates/engine/tests/freshness_computation.rs`
- `crates/compiler/tests/author.rs`
- `crates/compiler/tests/artifact_manifest_interface.rs`
- `crates/compiler/tests/canonical_artifacts_ingest.rs`
- `crates/compiler/tests/freshness_computation.rs`
- `crates/compiler/tests/rendering_surface.rs`
- `crates/compiler/tests/resolver_core.rs`
- `crates/compiler/tests/setup.rs`
- `crates/compiler/tests/doctor.rs`
- `crates/flow/tests/resolver_core.rs`
- `crates/cli/tests/author_cli.rs`
- `crates/cli/tests/cli_surface.rs`
- `crates/cli/tests/snapshots/handbook-author-environment-inventory-help.txt`
- `docs/specs/handbook-contract-membrane/slices/HCM-2.4/proof/implementation/P2-environment-context.md`

Only Environment Inventory/Environment Context branches are editable inside
shared runtime and test files. P6 owns aggregate bridge/fixed-selector deletion.
`crates/engine/src/profile_decision.rs`, any new condition evaluator, and any
condition evidence-contract surface are read/proof-only and outside the P2
selector. The frozen shipped resolver currently returns
`unresolved` / `EvidenceContractUnavailable` and `indeterminate` for the
conditional Environment Context descriptor. P2 must not infer, coerce, or
self-authorize condition `true`/`false`; runtime edits remain blocked until the
exact condition evidence/evaluator authority and selector are separately
approved or proven complete.

## P3 — Work Specification / Stage 10 vertical

Editable runtime and library assets:

- `core/stages/10_feature_spec.md`
- `core/library/feature_spec/feature_spec_architect_directive.md`
- `core/library/feature_spec/FEATURE_SPEC.md.tmpl`
- `core/schemas/feature_spec.yaml`
- `core/pipelines/default.yaml`
- `core/pipelines/foundation_inputs.yaml`
- `crates/pipeline/src/pipeline_capture.rs`
- `crates/pipeline/src/stage_10_feature_spec_provenance.rs`
- `crates/pipeline/src/pipeline_handoff.rs`
- `crates/pipeline/src/layout.rs`
- `crates/compiler/src/layout.rs`

Editable tests:

- `crates/pipeline/tests/pipeline_capture.rs`
- `crates/pipeline/tests/pipeline_handoff.rs`
- `crates/compiler/tests/pipeline_capture.rs`
- `crates/compiler/tests/pipeline_handoff.rs`
- `crates/cli/tests/pipeline_handoff_refusals.rs`
- `crates/cli/tests/feature_spec_contract.rs`

New exact engine fixture:

- `crates/engine/tests/fixtures/hcm_2_4_work_specification/.handbook/profile-selection.json`
- `crates/engine/tests/fixtures/hcm_2_4_work_specification/.handbook/definitions/profiles/work-specification-root-1.0.0.yaml`
- `crates/engine/tests/fixtures/hcm_2_4_work_specification/artifacts/work-specification/work-specification.yaml`
- `crates/engine/tests/fixtures/hcm_2_4_work_specification/artifacts/feature_spec/FEATURE_SPEC.md`

Exact pipeline proof-corpus mirrors:

- `tests/fixtures/pipeline_proof_corpus/foundation_inputs/repo/.handbook/profile-selection.json`
- `tests/fixtures/pipeline_proof_corpus/foundation_inputs/repo/.handbook/definitions/profiles/work-specification-root-1.0.0.yaml`
- `tests/fixtures/pipeline_proof_corpus/foundation_inputs/repo/artifacts/work-specification/work-specification.yaml`
- `tests/fixtures/pipeline_proof_corpus/foundation_inputs/repo/artifacts/feature_spec/FEATURE_SPEC.md`
- `tests/fixtures/pipeline_proof_corpus/foundation_inputs/model_outputs/stage_10_feature_spec.md`
- `tests/fixtures/pipeline_proof_corpus/foundation_inputs/goldens/capture.preview.stage_10_feature_spec.txt`
- `tests/fixtures/pipeline_proof_corpus/foundation_inputs/goldens/capture.apply.stage_10_feature_spec.txt`
- `tests/fixtures/pipeline_proof_corpus/foundation_inputs/goldens/capture.refused.stage_10_raw_compile_payload.txt`
- `tests/fixtures/pipeline_proof_corpus/foundation_inputs/goldens/compile.stage_10_feature_spec.explain.full_context.txt`
- `tests/fixtures/pipeline_proof_corpus/foundation_inputs/goldens/compile.stage_10_feature_spec.payload.full_context.txt`

Exact foundation-flow mirrors:

- `tests/fixtures/foundation_flow_demo/repo/.handbook/profile-selection.json`
- `tests/fixtures/foundation_flow_demo/repo/.handbook/definitions/profiles/work-specification-root-1.0.0.yaml`
- `tests/fixtures/foundation_flow_demo/repo/artifacts/work-specification/work-specification.yaml`
- `tests/fixtures/foundation_flow_demo/repo/artifacts/feature_spec/FEATURE_SPEC.md`
- `tests/fixtures/foundation_flow_demo/model_outputs/happy_path/stage_10_feature_spec.md`
- `tests/fixtures/foundation_flow_demo/model_outputs/skip_path/stage_10_feature_spec.md`
- `tests/fixtures/foundation_flow_demo/expected/happy_path/final_feature_spec.md`
- `tests/fixtures/foundation_flow_demo/expected/skip_path/final_feature_spec.md`
- `docs/specs/handbook-contract-membrane/slices/HCM-2.4/proof/implementation/P3-work-specification.md`

No public handoff bundle schema or API is editable.

### P3B — CLI surface proof integration

- `crates/cli/tests/cli_surface.rs`
- `tests/fixtures/foundation_flow_demo/evidence/happy_path.transcript.txt`
- `tests/fixtures/foundation_flow_demo/evidence/skip_path.transcript.txt`
- `docs/specs/handbook-contract-membrane/slices/HCM-2.4/proof/implementation/P3B-cli-surface-proof.md`
- `docs/specs/handbook-contract-membrane/slices/HCM-2.4/decision/20260727-p3b-cli-surface-proof-selector-repair.md`
- `docs/specs/handbook-contract-membrane/slices/HCM-2.4/decision/20260727-p3b-fixture-contract-selector-repair.md`
- `docs/specs/handbook-contract-membrane/slices/HCM-2.4/decision/20260727-p3b-repository-identity-prerequisite-selector-repair.md`
- `docs/specs/handbook-contract-membrane/slices/HCM-2.4/decision/20260727-p3b-foundation-feature-identity-selector-repair.md`
- `docs/specs/handbook-contract-membrane/slices/HCM-2.4/decision/20260727-p3b-m5-canonical-consumer-selector-repair.md`
- `docs/specs/handbook-contract-membrane/slices/HCM-2.4/decision/20260727-p3b-shared-compile-golden-selector-repair.md`
- `docs/specs/handbook-contract-membrane/slices/HCM-2.4/decision/20260727-p3b-negative-fixture-independence-selector-repair.md`
- `crates/cli/tests/pipeline_handoff_refusals.rs`
- `docs/specs/handbook-contract-membrane/slices/HCM-2.4/decision/20260727-p3b-feature-spec-contract-selector-repair.md`
- `crates/cli/tests/feature_spec_contract.rs`
- `tests/fixtures/pipeline_proof_corpus/foundation_inputs/repo/core/stages/10_feature_spec.md`
- `tests/fixtures/pipeline_proof_corpus/foundation_inputs/repo/core/library/feature_spec/feature_spec_architect_directive.md`
- `tests/fixtures/pipeline_proof_corpus/foundation_inputs/repo/core/library/feature_spec/FEATURE_SPEC.md.tmpl`
- `tests/fixtures/pipeline_proof_corpus/foundation_inputs/repo/.handbook/profile-selection.json`
- `tests/fixtures/pipeline_proof_corpus/foundation_inputs/repo/.handbook/definitions/profiles/work-specification-root-1.0.0.yaml`
- `tests/fixtures/foundation_flow_demo/repo/core/stages/10_feature_spec.md`
- `tests/fixtures/foundation_flow_demo/repo/core/library/feature_spec/feature_spec_architect_directive.md`
- `tests/fixtures/foundation_flow_demo/repo/core/library/feature_spec/FEATURE_SPEC.md.tmpl`
- `tests/fixtures/foundation_flow_demo/repo/.handbook/profile-selection.json`
- `tests/fixtures/foundation_flow_demo/repo/.handbook/definitions/profiles/work-specification-root-1.0.0.yaml`
- `tests/fixtures/foundation_flow_demo/model_outputs/happy_path/stage_10_feature_spec.md`
- `tests/fixtures/foundation_flow_demo/model_outputs/skip_path/stage_10_feature_spec.md`
- `tests/fixtures/foundation_flow_demo/expected/happy_path/final_feature_spec.md`
- `tests/fixtures/foundation_flow_demo/expected/skip_path/final_feature_spec.md`
- `tests/fixtures/foundation_flow_demo/expected/happy_path/SLICE_PLAN.md`
- `tests/fixtures/foundation_flow_demo/evidence/m5_handoff_scorecard.md`
- `tests/fixtures/pipeline_proof_corpus/foundation_inputs/goldens/compile.stage_10_feature_spec.payload.full_context.txt`
- `tests/fixtures/pipeline_proof_corpus/foundation_inputs/goldens/compile.stage_10_feature_spec.explain.full_context.txt`

Only the Stage 10 setup/assertion, private fresh-identity prerequisite helper,
three private canonical-consumer helpers, deterministic transcript, exact
fixture-authority surfaces, two foundation-flow Work Specification inputs, one
negative-test fixture-absence assertion, and four exact expected/evidence paths
plus the paired shared compile goldens and one cross-case contract assertion
frozen by the eight decisions are editable. Generated fixture-repo outputs remain
transient; no fixture repository identity may be committed. No
production/library path is selected.

Read/proof-only shared-golden consumers:

- `crates/compiler/tests/pipeline_compile.rs`
- `crates/pipeline/tests/pipeline_compile.rs`

## P4 — Decision Record support proof

New exact fixture and test:

- `crates/engine/tests/fixtures/hcm_2_4_decision_record/.handbook/profile-selection.json`
- `crates/engine/tests/fixtures/hcm_2_4_decision_record/.handbook/definitions/profiles/decision-record-root-1.0.0.yaml`
- `crates/engine/tests/fixtures/hcm_2_4_decision_record/.handbook/records/decision.yaml`
- `crates/engine/tests/hcm_2_4_decision_record.rs`
- `docs/specs/handbook-contract-membrane/slices/HCM-2.4/proof/implementation/P4-decision-record.md`
- `docs/specs/handbook-contract-membrane/slices/HCM-2.4/decision/20260727-p4-p5-generic-mutation-token-authority-stop.md`

Generic HCM-2.3 runtime is read/proof-only.

## P5 — Risk Record support proof

New exact fixture and test:

- `crates/engine/tests/fixtures/hcm_2_4_risk_record/.handbook/profile-selection.json`
- `crates/engine/tests/fixtures/hcm_2_4_risk_record/.handbook/definitions/profiles/risk-record-root-1.0.0.yaml`
- `crates/engine/tests/fixtures/hcm_2_4_risk_record/.handbook/records/risk.yaml`
- `crates/engine/tests/hcm_2_4_risk_record.rs`
- `docs/specs/handbook-contract-membrane/slices/HCM-2.4/proof/implementation/P5-risk-record.md`
- `docs/specs/handbook-contract-membrane/slices/HCM-2.4/decision/20260727-p4-p5-generic-mutation-token-authority-stop.md`

Generic HCM-2.3 runtime is read/proof-only.

## P6 — Aggregate flow and fixed-selector deletion

Editable runtime:

- `crates/engine/src/artifact_manifest.rs`
- `crates/engine/src/baseline_validation.rs`
- `crates/engine/src/canonical_artifacts.rs`
- `crates/engine/src/canonical_paths.rs`
- `crates/engine/src/freshness.rs`
- `crates/engine/src/lib.rs`
- `crates/compiler/src/author/charter_shell.rs`
- `crates/compiler/src/author/environment_inventory_shell.rs`
- `crates/compiler/src/author/mod.rs`
- `crates/compiler/src/layout.rs`
- `crates/compiler/src/baseline_validation.rs`
- `crates/compiler/src/blocker.rs`
- `crates/compiler/src/lib.rs`
- `crates/compiler/src/refusal.rs`
- `crates/compiler/src/rendering/markdown.rs`
- `crates/compiler/src/rendering/shared.rs`
- `crates/flow/src/resolver.rs`
- `crates/flow/src/budget.rs`
- `crates/flow/src/packet_result.rs`
- `crates/cli/src/rendering.rs`

Editable tests:

- `crates/engine/tests/artifact_manifest_interface.rs`
- `crates/engine/tests/baseline_validation.rs`
- `crates/engine/tests/canonical_artifacts_ingest.rs`
- `crates/engine/tests/freshness_computation.rs`
- `crates/engine/tests/hcm_1_1_custom_kind.rs`
- `crates/engine/tests/hcm_2_1_project_context.rs`
- `crates/compiler/tests/artifact_manifest_interface.rs`
- `crates/compiler/tests/author.rs`
- `crates/compiler/tests/canonical_artifacts_ingest.rs`
- `crates/compiler/tests/freshness_computation.rs`
- `crates/compiler/tests/refusal_mapping.rs`
- `crates/compiler/tests/rendering_surface.rs`
- `crates/compiler/tests/resolver_core.rs`
- `crates/flow/tests/resolver_core.rs`
- `crates/flow/tests/budget_domains.rs`
- `crates/cli/tests/author_cli.rs`
- `crates/cli/tests/cli_surface.rs`
- `crates/cli/tests/feature_spec_contract.rs`
- `crates/cli/tests/pipeline_handoff_refusals.rs`
- `docs/specs/handbook-contract-membrane/slices/HCM-2.4/proof/implementation/P6-aggregate-cleanup.md`

Only bridge/fixed-family behavior is editable. Durable selected-artifact
behavior tests are rewritten, not deleted.

## P7 — Exit proof and control-pack closeout

Editable control-pack and proof paths:

- `docs/specs/handbook-contract-membrane/03-seam-crosswalk.md`
- `docs/specs/handbook-contract-membrane/04-phase-slice-map.md`
- `docs/specs/handbook-contract-membrane/06-proof-and-regression-ledger.md`
- `docs/specs/handbook-contract-membrane/09-review-finding-inventory.md`
- `docs/specs/handbook-contract-membrane/slices/HCM-2.4/SPEC.md`
- `docs/specs/handbook-contract-membrane/slices/HCM-2.4/tasks/plan.md`
- `docs/specs/handbook-contract-membrane/slices/HCM-2.4/tasks/todo.md`
- `docs/specs/handbook-contract-membrane/slices/HCM-2.4/proof/implementation/P7-final-proof-wall.md`

Read/proof-only closeout inputs:

- `docs/specs/handbook-contract-membrane/slices/HCM-2.4/decision/20260726-p2-selector-correction.md`
- `docs/specs/handbook-contract-membrane/slices/HCM-2.4/contracts/implementation-packet-path-manifest-v1.0.md`
- `docs/specs/handbook-contract-membrane/slices/HCM-2.4/proof/implementation/P0-inventory-lock.md`
- `docs/specs/handbook-contract-membrane/slices/HCM-2.4/proof/implementation/P1A-definition-support.md`
- `docs/specs/handbook-contract-membrane/slices/HCM-2.4/proof/implementation/P1B-charter-compatibility.md`
- `docs/specs/handbook-contract-membrane/slices/HCM-2.4/proof/implementation/P1C-shipped-root-adoption.md`
- `docs/specs/handbook-contract-membrane/slices/HCM-2.4/proof/implementation/P2-environment-context.md`
- `docs/specs/handbook-contract-membrane/slices/HCM-2.4/proof/implementation/P3-work-specification.md`
- `docs/specs/handbook-contract-membrane/slices/HCM-2.4/proof/implementation/P4-decision-record.md`
- `docs/specs/handbook-contract-membrane/slices/HCM-2.4/proof/implementation/P5-risk-record.md`
- `docs/specs/handbook-contract-membrane/slices/HCM-2.4/proof/implementation/P6-aggregate-cleanup.md`

P7 may also create only the validator-approved complete-subject review
dispatches and the single parent-owned true-stop handoff/ledger closeout
required by `08`. It may not open HCM-3.x.
