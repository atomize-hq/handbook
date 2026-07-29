# P2 greenfield Environment Context correction proof

Status: post-remediation verification complete; independent delta closure CLEAN

## Frozen active identity

- shipped profile: `handbook.profile.shipped-root@1.2.0`
- complete typed-closure fingerprint:
  `sha256:40c5fdb8a6ea42cf0f5f2c5cac8306ec7ad3a238c341653947f85abc93d72c40`
- active Environment Context schema/kind/intake/renderer:
  `handbook.schemas.artifacts.environment-context@1.1.0`,
  `handbook.artifact-kind.environment-context@1.1.0`,
  `handbook.intake.environment-context@1.0.0`, and
  `handbook.renderer.environment-context-review-markdown@1.0.0`

Environment schema/kind 1.0 remain only because the immutable HCM-2.2 Charter
and shipped-profile 1.1 closure pins those exact identities. No active immutable
consumer pins the prior P1A Environment kind 1.1, intake 1.0, or renderer 1.0
bytes, so those identities and shipped-root 1.2 are corrected in place under
the greenfield posture. `CharterDefinitionRegistry` retains exact admission by
binding the corrected shipped-root 1.2 fingerprint. No compatibility profile
or subordinate release successor is introduced.

## Hold-point requirement audit

Starting HEAD: `7becde555bcc9742cd0ada4cff27542f521e6abb`.

- SPEC, plan, todo, program map, crosswalk, and proof ledger were compared to
  starting HEAD. Every unaffected P0/P1A/P1B/P1C/P3/P3B/P4/P5/P6/P7 and
  Phase 2 section is present in active authority; only P2 conclusions and
  directly dependent gate truth are superseded.
- P4 remains open on its exact negative-surface/path proof. P6 retains the
  aggregate-flow, both-bridge, live fixed-selector (including temporary fixed
  Environment Context packet/layout branches), legacy-exception, and bridge-test
  deletion gate. P7 and Phase 2 exit remain blocked.
- The deletion-heavy P2S evidence subsystem has no live evaluator/schema
  consumer. Legacy Environment Inventory has no positive production
  read/write/packet consumer. Their active-tree deletion is therefore owned
  by the handoff rather than compatibility churn.
- All packet changes are classified below. The eight protected paths and
  `docs/ideas/intent-to-outcome-fidelity.md` are excluded. No path remains as
  unexplained churn.

### Changed-path classification (143 packet paths)

#### Active authority and gate correction

- `M` `docs/specs/handbook-contract-membrane/00-README.md`
- `M` `docs/specs/handbook-contract-membrane/02-semantic-model.md`
- `M` `docs/specs/handbook-contract-membrane/03-seam-crosswalk.md`
- `M` `docs/specs/handbook-contract-membrane/04-phase-slice-map.md`
- `M` `docs/specs/handbook-contract-membrane/05-contracts-schemas-and-gates.md`
- `M` `docs/specs/handbook-contract-membrane/06-proof-and-regression-ledger.md`
- `M` `docs/specs/handbook-contract-membrane/slices/HCM-0.6/SPEC.md`
- `M` `docs/specs/handbook-contract-membrane/slices/HCM-0.6/decision/shipped-default-artifact-set-decision.md`
- `M` `docs/specs/handbook-contract-membrane/slices/HCM-2.4/SPEC.md`
- `M` `docs/specs/handbook-contract-membrane/slices/HCM-2.4/decision/20260726-p2-condition-evaluator-authority-stop.md`
- `M` `docs/specs/handbook-contract-membrane/slices/HCM-2.4/decision/20260727-p2-managed-operational-surface-evidence-evaluator-planning-amendment.md`
- `M` `docs/specs/handbook-contract-membrane/slices/HCM-2.4/decision/20260728-p2-product-decision-reassessment.md`
- `M` `docs/specs/handbook-contract-membrane/slices/HCM-2.4/tasks/plan.md`
- `M` `docs/specs/handbook-contract-membrane/slices/HCM-2.4/tasks/todo.md`
- `??` `docs/specs/handbook-contract-membrane/slices/HCM-2.4/decision/20260728-p2-greenfield-environment-context-and-task-gate.md`

#### Rejected P2S evaluator/evidence cleanup

- `D` `crates/engine/definitions/project-condition-evaluators/handbook.condition-evaluator.managed-operational-surface/1.0.0.yaml`
- `D` `crates/engine/definitions/schemas/handbook.schemas.project-condition-admitted-evidence-source/1.0.0.schema.json`
- `D` `crates/engine/definitions/schemas/handbook.schemas.project-condition-evidence-assertion/1.0.0.schema.json`
- `D` `crates/engine/definitions/schemas/handbook.schemas.project-condition-evidence-challenge/1.0.0.schema.json`
- `D` `crates/engine/definitions/schemas/handbook.schemas.project-condition-evidence-evaluation-closure/1.0.0.schema.json`
- `D` `crates/engine/definitions/schemas/handbook.schemas.project-condition-evidence-head/1.0.0.schema.json`
- `D` `crates/engine/definitions/schemas/handbook.schemas.project-condition-evidence-transaction/1.0.0.schema.json`
- `D` `crates/engine/definitions/schemas/handbook.schemas.project-condition-evidence-use-transition/1.0.0.schema.json`
- `D` `crates/engine/definitions/schemas/handbook.schemas.project-condition-evidence/1.0.0.schema.json`
- `D` `crates/engine/tests/hcm_2_4_project_condition_evidence_schemas.rs`

#### Canonical Environment Context and advisory runtime

- `M` `crates/compiler/src/profile_readiness.rs`
- `M` `crates/engine/definitions/artifact-kinds/handbook.artifact-kind.environment-context/1.1.0.yaml`
- `M` `crates/engine/definitions/intakes/handbook.intake.environment-context/1.0.0.yaml`
- `M` `crates/engine/definitions/profiles/handbook.profile.shipped-root/1.2.0.yaml`
- `M` `crates/engine/definitions/renderers/handbook.renderer.environment-context-review-markdown/1.0.0.yaml`
- `M` `crates/engine/src/artifact_instance.rs`
- `M` `crates/engine/src/artifact_intake_registry.rs`
- `M` `crates/engine/src/charter_definition_registry.rs`
- `M` `crates/engine/src/profile_builtins.rs`
- `M` `crates/engine/src/profile_decision.rs`
- `M` `crates/flow/src/resolver.rs`
- `M` `crates/flow/tests/resolver_core.rs`
- `??` `crates/engine/definitions/schemas/handbook.schemas.artifacts.environment-context/1.1.0.entry.yaml`
- `??` `crates/engine/definitions/schemas/handbook.schemas.artifacts.environment-context/1.1.0.schema.json`
- `??` `crates/engine/src/environment_context_artifact.rs`

#### Legacy Environment Inventory authority removal

- `D` `core/library/environment_inventory/ENVIRONMENT_INVENTORY.md.tmpl`
- `D` `core/library/environment_inventory/ENVIRONMENT_INVENTORY_INPUTS.yaml.tmpl`
- `D` `core/library/environment_inventory/environment_inventory_directive.md`
- `M` `crates/cli/src/author.rs`
- `M` `crates/cli/src/main.rs`
- `M` `crates/cli/src/rendering.rs`
- `M` `crates/cli/tests/author_cli.rs`
- `M` `crates/cli/tests/cli_surface.rs`
- `D` `crates/cli/tests/snapshots/handbook-author-environment-inventory-help.txt`
- `M` `crates/cli/tests/snapshots/handbook-author-help.txt`
- `M` `crates/cli/tests/snapshots/handbook-help.txt`
- `D` `crates/compiler/src/author/environment_inventory.rs`
- `D` `crates/compiler/src/author/environment_inventory_core.rs`
- `D` `crates/compiler/src/author/environment_inventory_shell.rs`
- `M` `crates/compiler/src/author/mod.rs`
- `M` `crates/compiler/src/baseline_validation.rs`
- `M` `crates/compiler/src/blocker.rs`
- `M` `crates/compiler/src/layout.rs`
- `M` `crates/compiler/src/lib.rs`
- `M` `crates/compiler/src/refusal.rs`
- `M` `crates/compiler/src/rendering/markdown.rs`
- `M` `crates/compiler/src/rendering/shared.rs`
- `M` `crates/compiler/src/resolver.rs`
- `M` `crates/compiler/src/template_library.rs`
- `D` `crates/engine/src/author/environment_inventory_core.rs`
- `M` `crates/engine/src/author/mod.rs`
- `M` `crates/engine/src/baseline_validation.rs`
- `M` `crates/engine/src/canonical_artifacts.rs`
- `M` `crates/engine/src/canonical_paths.rs`
- `M` `crates/engine/src/freshness.rs`
- `M` `crates/engine/src/lib.rs`
- `D` `tests/fixtures/foundation_flow_demo/repo/core/library/environment_inventory/ENVIRONMENT_INVENTORY.md.tmpl`
- `D` `tests/fixtures/foundation_flow_demo/repo/core/library/environment_inventory/environment_inventory_directive.md`
- `D` `tests/fixtures/pipeline_proof_corpus/foundation_inputs/repo/artifacts/foundation/ENVIRONMENT_INVENTORY.md`

#### Pipeline/output and mirrored-fixture cleanup

- `M` `core/library/foundation_pack/FOUNDATION_STRATEGY.md.tmpl`
- `M` `core/library/foundation_pack/foundation_pack_directive.md`
- `M` `core/library/patterns.md`
- `M` `core/pipelines/default.yaml`
- `M` `core/rules/p0_absolute.md`
- `M` `core/runners/codex-cli.md`
- `M` `core/stages/07_foundation_pack.md`
- `M` `core/stages/10_feature_spec.md`
- `M` `crates/compiler/tests/pipeline_capture.rs`
- `M` `crates/compiler/tests/pipeline_catalog.rs`
- `M` `crates/pipeline/src/pipeline_capture.rs`
- `M` `crates/pipeline/tests/pipeline_capture.rs`
- `M` `crates/pipeline/tests/pipeline_catalog.rs`
- `M` `tests/fixtures/foundation_flow_demo/evidence/happy_path.transcript.txt`
- `M` `tests/fixtures/foundation_flow_demo/evidence/skip_path.transcript.txt`
- `M` `tests/fixtures/foundation_flow_demo/model_outputs/happy_path/stage_07_foundation_pack.txt`
- `M` `tests/fixtures/foundation_flow_demo/model_outputs/skip_path/stage_07_foundation_pack.txt`
- `M` `tests/fixtures/foundation_flow_demo/repo/.handbook/definitions/profiles/work-specification-root-1.0.0.yaml`
- `M` `tests/fixtures/foundation_flow_demo/repo/.handbook/profile-selection.json`
- `M` `tests/fixtures/foundation_flow_demo/repo/core/library/foundation_pack/FOUNDATION_STRATEGY.md.tmpl`
- `M` `tests/fixtures/foundation_flow_demo/repo/core/library/foundation_pack/foundation_pack_directive.md`
- `M` `tests/fixtures/foundation_flow_demo/repo/core/rules/p0_absolute.md`
- `M` `tests/fixtures/foundation_flow_demo/repo/core/runners/codex-cli.md`
- `M` `tests/fixtures/foundation_flow_demo/repo/core/stages/07_foundation_pack.md`
- `M` `tests/fixtures/foundation_flow_demo/repo/core/stages/10_feature_spec.md`
- `M` `tests/fixtures/pipeline_proof_corpus/foundation_inputs/goldens/capture.apply.stage_07_foundation_pack.txt`
- `M` `tests/fixtures/pipeline_proof_corpus/foundation_inputs/goldens/capture.preview.stage_07_foundation_pack.txt`
- `M` `tests/fixtures/pipeline_proof_corpus/foundation_inputs/goldens/capture.refused.duplicate_declared_block.txt`
- `M` `tests/fixtures/pipeline_proof_corpus/foundation_inputs/goldens/capture.refused.empty_declared_block.txt`
- `M` `tests/fixtures/pipeline_proof_corpus/foundation_inputs/goldens/capture.refused.missing_declared_block.txt`
- `M` `tests/fixtures/pipeline_proof_corpus/foundation_inputs/goldens/compile.stage_10_feature_spec.explain.full_context.txt`
- `M` `tests/fixtures/pipeline_proof_corpus/foundation_inputs/goldens/compile.stage_10_feature_spec.payload.full_context.txt`
- `M` `tests/fixtures/pipeline_proof_corpus/foundation_inputs/repo/.handbook/definitions/profiles/work-specification-root-1.0.0.yaml`
- `M` `tests/fixtures/pipeline_proof_corpus/foundation_inputs/repo/.handbook/profile-selection.json`
- `M` `tests/fixtures/pipeline_proof_corpus/foundation_inputs/repo/core/rules/p0_absolute.md`
- `M` `tests/fixtures/pipeline_proof_corpus/foundation_inputs/repo/core/runners/codex-cli.md`
- `M` `tests/fixtures/pipeline_proof_corpus/foundation_inputs/repo/core/stages/07_foundation_pack.md`
- `M` `tests/fixtures/pipeline_proof_corpus/foundation_inputs/repo/core/stages/10_feature_spec.md`

#### Exact closure, profile, and golden propagation

- `M` `crates/engine/tests/fixtures/hcm_2_4_decision_record/.handbook/definitions/profiles/decision-record-root-1.0.0.yaml`
- `M` `crates/engine/tests/fixtures/hcm_2_4_decision_record/.handbook/profile-selection.json`
- `M` `crates/engine/tests/fixtures/hcm_2_4_risk_record/.handbook/definitions/profiles/risk-record-root-1.0.0.yaml`
- `M` `crates/engine/tests/fixtures/hcm_2_4_risk_record/.handbook/profile-selection.json`
- `M` `crates/engine/tests/fixtures/hcm_2_4_work_specification/.handbook/definitions/profiles/work-specification-root-1.0.0.yaml`
- `M` `crates/engine/tests/fixtures/hcm_2_4_work_specification/.handbook/profile-selection.json`
- `M` `docs/specs/handbook-contract-membrane/slices/HCM-2.4/contracts/definition-support-vectors-v1.0.json`
- `M` `docs/specs/handbook-contract-membrane/slices/HCM-2.4/contracts/implementation-packet-path-manifest-v1.0.md`
- `M` `docs/specs/handbook-contract-membrane/slices/HCM-2.4/contracts/renderer-goldens-v1.0.json`

#### Focused regression and fixed-surface support

- `M` `crates/compiler/tests/artifact_manifest_interface.rs`
- `M` `crates/compiler/tests/author.rs`
- `M` `crates/compiler/tests/canonical_artifacts_ingest.rs`
- `M` `crates/compiler/tests/freshness_computation.rs`
- `M` `crates/compiler/tests/rendering_surface.rs`
- `M` `crates/compiler/tests/resolver_core.rs`
- `M` `crates/compiler/tests/setup.rs`
- `M` `crates/engine/tests/artifact_manifest_interface.rs`
- `M` `crates/engine/tests/author_core.rs`
- `M` `crates/engine/tests/baseline_validation.rs`
- `M` `crates/engine/tests/canonical_artifacts_ingest.rs`
- `M` `crates/engine/tests/freshness_computation.rs`
- `M` `crates/engine/tests/hcm_1_1_custom_kind.rs`
- `M` `crates/engine/tests/hcm_1_4_profile_decisions.rs`
- `M` `crates/engine/tests/hcm_1_4_profile_inspection.rs`
- `M` `crates/engine/tests/hcm_2_4_charter_profile_compatibility.rs`
- `M` `crates/engine/tests/hcm_2_4_decision_record.rs`
- `M` `crates/engine/tests/hcm_2_4_definition_runtime.rs`
- `M` `crates/engine/tests/hcm_2_4_definition_support.rs`
- `M` `crates/engine/tests/hcm_2_4_risk_record.rs`
- `??` `crates/engine/tests/hcm_2_4_environment_context_greenfield.rs`
- `??` `docs/specs/handbook-contract-membrane/slices/HCM-2.4/proof/implementation/P2-greenfield-environment-context-correction.md`

## Verification

### Symbol impact

- `ArtifactInstanceRegistry::resolve`: CRITICAL; 23 direct callers, 169 total
  impacted symbols, and eight affected processes. The user was warned before
  the exact descriptor-closure edit.
- `shipped_profile_request`: CRITICAL; one direct caller, 156 total impacted
  symbols, and seven affected processes. The user was warned before the exact
  shipped-profile source/selection edit.
- `ArtifactIntakeDefinitionV1::parse`: LOW; three direct callers, 14 total
  impacted symbols, and one affected process. `profile_builtins::definition`
  was LOW with no indexed callers.
- `packet_artifact_plans_for`: LOW; direct caller `resolve_with_contract`, no
  indexed process expansion. The edit only prevents rendered projections from
  bypassing non-verbatim budget dispositions.
- `rendered_projection_for_path`: HIGH; direct callers
  `packet_artifact_plans_for` and `present_fixture_sources_for`, six total
  indexed consumers and no indexed process. The symbol was not deleted or
  generalized in this packet.
- `CanonicalLayoutContract::artifact`: HIGH; direct fixed-layout consumers,
  ten total indexed consumers and no indexed process. The retained P6 bridge
  gate remains open; this packet removes only Environment Inventory branches.
- `Cli` and `main`: LOW with no indexed callers; the edit removes obsolete
  Environment Inventory help text.
- `artifact_repository_open_semantically_admits_all_p1a_intakes`: LOW with no
  direct caller, affected process, or affected module; its only remediation was
  the stale test-only Environment Context intake filename.

### RED/GREEN and full wall

- RED: `oversized_environment_context_is_summarized_without_rendered_body`
  failed because the summarized source still carried rendered metadata and a
  full rendered section. The post-review full wall also exposed stale
  compatibility-only exact identities in the Decision fixture profile and the
  definition-runtime Environment intake filename; focused tests failed before
  those test/fixture closures were corrected.
- GREEN: Environment Context engine target 10/10; flow resolver 10/10;
  scoped readiness 3/3; CLI help 11/11; six-target HCM-2.4 engine wall 56/56;
  Decision Record 4/4; definition runtime 7/7; Unix/WSL profile inspection
  23/23.
- `cargo test --workspace --all-features -j1`: exit 0 in 885.5 seconds;
  complete workspace unit, integration, actual-binary, and doc-test wall.
- `cargo clippy --workspace --all-targets --all-features -- -D warnings`:
  exit 0 in 7.5 seconds.
- `cargo fmt --all -- --check` and `git diff --check`: exit 0.
- all eight changed/non-deleted JSON documents parse successfully.

### Forbidden influence and ownership

- exact P2S evaluator/evidence identity scan across production sources,
  definitions, and core assets: no matches.
- legacy Environment Inventory scan: only the explicit negative catalog and
  no-influence regressions remain; no positive reader, writer, packet source,
  template, or generated output remains.
- production advisory consumer: `EnvironmentContextFlowBridge::load` calls
  `load_selected_environment_context`; valid context is included subject to
  packet budget disposition, while invalid/missing context is omitted and
  reported non-blockingly.

### GitNexus change detection

- index status: current at `7becde5` on
  `feat/handbook-contract-membrane`.
- first implementation pre-commit compare to `main`: 1,167 files, 8,894
  symbols, 243 affected processes, CRITICAL. The final active-authority staged
  compare was 1,168 files with the same 8,894 symbols, 243 processes, and
  CRITICAL rating. These are whole long-lived branch deltas rather than packet
  risk; the counts include newly staged packet definitions/authority.
- unstaged indexed view: 124 files, 236 symbols, eight affected processes,
  HIGH. It includes the eight protected documentation changes and stale graph
  mappings for deleted Environment Inventory preflight symbols; the packet
  introduces no new native, dependency, trust, or generic transport surface.
- first staged implementation group: 107 indexed files, 155 symbols, eight
  affected processes, HIGH; the 128 staged paths include assets and records
  without indexed symbols.
- active-authority staged group: 15 files, 74 symbols, no affected processes,
  LOW.

### Protected paths

All eight original dirty-path SHA-256 values match the task-entry baseline.
They remain unstaged. The concurrent user-owned
`docs/ideas/intent-to-outcome-fidelity.md` remains untracked and unstaged.
No commit or push has occurred at this proof boundary.

### Independent review

The final independent reviewer returned CLEAN after a delta closure confirming
the rebaselined P6 selector names only live post-P2 fixed surfaces. No P1/P2
finding remains open.
