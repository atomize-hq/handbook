# HCM-4.2 corrective owner and admission matrix

**Authority:**
`../decision/20260811T025558Z--hcm-4-2-causal-remediation-selector.md`
**Scope:** planning evidence only; no DTO, schema, operation definition,
catalog entry, SDK operation, CLI adapter, or Tauri adapter is implemented by
this document
**Inventory:** exactly 62 ordinary operation IDs, in the canonical order from
`../../../05-contracts-schemas-and-gates.md`

## Reading rule

`live_precursor` means only that the named Rust `path::symbol` exists and is
useful to a future owner adapter. It is not an HCM-4.2 operation implementation
and does not establish the planned request, result, outcome schemas, operation
definition, `CapabilityEntry`, or transport admission. `absent` means that no
operation-shaped public/runtime Rust precursor was found for that exact
ordinary operation; lower-level domain types and test-only helpers do not
change that state. `phase5_deferred` means the owner itself is intentionally
deferred to Phase 5.

Every row therefore has current operation transports `none` and discovery
state `omit`. The declared future targets are `rust_sdk`, `cli_json`, and
`tauri`, but a target is not a current transport. Discovery may include a row
only after its exact definition, request/result/blocker/refusal/error schemas,
owner mapping, receipts, and that transport's admission proof all close.

The outcome/receipt codes are exact per-row bindings:

- `R0`: planned `read_only` / `safe`; the operation result and three outcome
  schemas are absent; successful realized write receipts must be `[]`.
- `A1`: planned `append_only` / `idempotency_key_required`; the definition-
  pinned key path, operation result/outcome schemas, and declared realized
  receipt mapping are absent. A future success must match the append write set
  in 05, and a refusal before commit has no realized receipts.
- `C1`: planned `compare_and_write` / `compare_and_write_required`; the
  definition-pinned key path and expected-basis fields, operation result/
  outcome schemas, and declared realized receipt mapping are absent. A future
  success must match the atomic write set in 05, and a refusal before commit
  has no realized receipts.

## Authoritative 62-row matrix

| Proof | Operation ID | Live owner/SDK state and exact symbol | Schema and owner source | Mutability / idempotency | Current Rust result/refusal and HCM outcome/receipt binding | Declared / current transports | Discovery |
|---|---|---|---|---|---|---|---|
| M62-001 | `capabilities.describe` | `absent` | SDK registry planned by 05; shared closure absent | `R0` | no live typed operation result; `R0` | all three / none | omit |
| M62-002 | `profile.list` | `absent` | engine owner per 05; shared closure absent | `R0` | no live typed operation result; `R0` | all three / none | omit |
| M62-003 | `profile.resolve` | `absent` | engine owner per 05; shared closure absent | `R0` | lower-level profile resolution is not an ordinary operation; `R0` | all three / none | omit |
| M62-004 | `schema.list` | `absent` | engine owner per 05; shared closure absent | `R0` | no live typed operation result; `R0` | all three / none | omit |
| M62-005 | `schema.read` | `absent` | engine owner per 05; shared closure absent | `R0` | no live typed operation result; `R0` | all three / none | omit |
| M62-006 | `vocabulary.read` | `absent` | engine owner per 05; shared closure absent | `R0` | resolved-profile accessors are not an ordinary operation; `R0` | all three / none | omit |
| M62-007 | `resolution.stack.read` | `absent` | engine owner per 05; shared closure absent | `R0` | no live typed operation result; `R0` | all three / none | omit |
| M62-008 | `projection.definition.read` | `absent` | flow over engine per 05; shared closure absent | `R0` | no live typed operation result; `R0` | all three / none | omit |
| M62-009 | `artifact.kind.list` | `live_precursor`: `crates/sdk/src/artifact.rs::ArtifactSdk::list_kinds` | engine through SDK precursor; shared closure absent | `R0` | `ArtifactKindList` / `ArtifactSdkError`; HCM mapping absent; `R0` | all three / none (Rust precursor only) | omit |
| M62-010 | `artifact.instance.list` | `live_precursor`: `crates/sdk/src/artifact.rs::ArtifactSdk::list_instances` | engine through SDK precursor; shared closure absent | `R0` | `ArtifactInstanceList` / `ArtifactSdkError`; HCM mapping absent; `R0` | all three / none (Rust precursor only) | omit |
| M62-011 | `artifact.read` | `live_precursor`: `crates/sdk/src/artifact.rs::ArtifactSdk::read` | engine through SDK precursor; shared closure absent | `R0` | `ArtifactReadResult` / `ArtifactSdkError`; HCM mapping absent; `R0` | all three / none (Rust precursor only) | omit |
| M62-012 | `artifact.validate` | `live_precursor`: `crates/sdk/src/artifact.rs::ArtifactSdk::validate` | engine through SDK precursor; shared closure absent | `R0` | `ArtifactValidationResult` / `ArtifactSdkError`; HCM mapping absent; `R0` | all three / none (Rust precursor only) | omit |
| M62-013 | `artifact.render` | `absent` | engine owner planned by 05; shared closure absent | `R0` | fixed render helpers are not an ordinary operation; `R0` | all three / none | omit |
| M62-014 | `intake.definition.read` | `live_precursor`: `crates/sdk/src/artifact.rs::ArtifactSdk::intake_definition` | engine through SDK precursor; shared closure absent | `R0` | `ArtifactIntakeDefinition` / `ArtifactSdkError`; HCM mapping absent; `R0` | all three / none (Rust precursor only) | omit |
| M62-015 | `intake.coverage.evaluate` | `live_precursor`: `crates/sdk/src/artifact.rs::ArtifactSdk::evaluate_intake` | engine through SDK precursor; shared closure absent | `R0` | `ArtifactIntakeEvaluation` / `ArtifactSdkError`; HCM mapping absent; `R0` | all three / none (Rust precursor only) | omit |
| M62-016 | `intake.record.append` | `live_precursor`: `crates/sdk/src/artifact.rs::ArtifactSdk::intake_append` | engine transaction through SDK precursor; shared closure absent | `A1` | `ArtifactMutationExecution` / `ArtifactSdkError`; HCM outcome/key/receipt mapping absent; `A1` | all three / none (Rust precursor only) | omit |
| M62-017 | `record.list` | `absent` | engine owner per 05; shared closure absent | `R0` | no generic typed ordinary record catalog exists; `R0` | all three / none | omit |
| M62-018 | `record.read` | `absent` | engine owner per 05; shared closure absent | `R0` | no generic typed ordinary record read exists; `R0` | all three / none | omit |
| M62-019 | `artifact.candidate.validate` | `live_precursor`: `crates/sdk/src/artifact.rs::ArtifactSdk::candidate_validate` | engine through SDK precursor; shared closure absent | `R0` | `ArtifactCandidateValidation` / `ArtifactSdkError`; HCM mapping absent; `R0` | all three / none (Rust precursor only) | omit |
| M62-020 | `artifact.candidate.append` | `live_precursor`: `crates/sdk/src/artifact.rs::ArtifactSdk::candidate_append` | engine transaction through SDK precursor; shared closure absent | `A1` | `ArtifactMutationExecution` / `ArtifactSdkError`; HCM outcome/key/receipt mapping absent; `A1` | all three / none (Rust precursor only) | omit |
| M62-021 | `artifact.approval.append` | `absent` | engine transaction owner planned by 05; shared closure absent | `A1` | lower-level approval persistence is not an SDK ordinary operation; `A1` | all three / none | omit |
| M62-022 | `artifact.candidate.promote` | `live_precursor`: `crates/sdk/src/artifact.rs::ArtifactSdk::promote` | engine transaction through SDK precursor; shared closure absent | `C1` | `ArtifactMutationExecution` / `ArtifactSdkError`; HCM outcome/key/two-receipt mapping absent; `C1` | all three / none (Rust precursor only) | omit |
| M62-023 | `posture.resolve` | `absent` | engine owner per 05; shared closure absent | `R0` | kernel construction helpers are not an ordinary operation; `R0` | all three / none | omit |
| M62-024 | `posture.recommendation.evaluate` | `absent` | engine owner per 05; shared closure absent | `R0` | no operation-shaped public evaluator exists; `R0` | all three / none | omit |
| M62-025 | `posture.recommendation.append` | `absent` | engine transaction owner planned by 05; shared closure absent | `A1` | private repository transaction pieces are not an SDK ordinary operation; `A1` | all three / none | omit |
| M62-026 | `posture.recommendation.acknowledge` | `absent` | engine transaction owner planned by 05; shared closure absent | `A1` | no operation-shaped public/runtime seam exists; `A1` | all three / none | omit |
| M62-027 | `posture.transition.apply` | `live_precursor`: `crates/sdk/src/lib.rs::HandbookSdkV1::apply_posture_transition`; owner `crates/engine/src/posture_transition_facade.rs::PostureTransitionEngineFacadeV1::apply_posture_transition` | engine facade through SDK precursor; shared closure absent | `C1`; future key path `/body/idempotency_key` | `PostureTransitionApplyResultV1`; exhaustive corrective mapping in SPEC/05; operation schemas and receipts not implemented | all three / none (Rust precursor only) | omit |
| M62-028 | `projection.create` | `absent` | flow over engine planned by 05; shared closure absent | `R0` | projection helpers are not an ordinary operation; `R0` | all three / none | omit |
| M62-029 | `resolution.escalation.request.append` | `live_precursor`: `crates/engine/src/context_resolution.rs::ContextResolutionTransitionRegistry::admit_escalation_request` | engine in-memory owner precursor; SDK/repository/shared closure absent | `A1` | `Result<bool, ContextResolutionKernelError>`; no SDK result/outcome/receipt mapping; `A1` | all three / none (owner precursor only) | omit |
| M62-030 | `resolution.escalation.disposition.append` | `live_precursor`: `crates/engine/src/context_resolution.rs::ContextResolutionTransitionRegistry::admit_escalation_disposition` | engine in-memory owner precursor; SDK/repository/shared closure absent | `A1` | `Result<bool, ContextResolutionKernelError>`; no SDK result/outcome/receipt mapping; `A1` | all three / none (owner precursor only) | omit |
| M62-031 | `memory.promotion.request.append` | `live_precursor`: `crates/engine/src/context_resolution.rs::ContextResolutionTransitionRegistry::admit_promotion_request` | engine in-memory owner precursor; SDK/repository/shared closure absent | `A1` | `Result<bool, ContextResolutionKernelError>`; no SDK result/outcome/receipt mapping; `A1` | all three / none (owner precursor only) | omit |
| M62-032 | `memory.promotion.disposition.append` | `live_precursor`: `crates/engine/src/context_resolution.rs::ContextResolutionTransitionRegistry::admit_promotion_disposition` | engine in-memory owner precursor; SDK/repository/shared closure absent | `C1` | `Result<bool, ContextResolutionKernelError>`; no SDK result/outcome/conditional receipt mapping; `C1` | all three / none (owner precursor only) | omit |
| M62-033 | `snapshot.capture` | `absent` | SDK orchestration planned by 05; shared closure absent | `A1` | private snapshot helpers are not an ordinary operation; `A1` | all three / none | omit |
| M62-034 | `snapshot.read` | `absent` | engine storage through SDK planned by 05; shared closure absent | `R0` | no operation-shaped public/runtime seam exists; `R0` | all three / none | omit |
| M62-035 | `snapshot.delta` | `absent` | engine owner planned by 05; shared closure absent | `R0` | private delta semantics are not an ordinary operation; `R0` | all three / none | omit |
| M62-036 | `snapshot.project` | `absent` | flow over engine planned by 05; shared closure absent | `R0` | private projection semantics are not an ordinary operation; `R0` | all three / none | omit |
| M62-037 | `snapshot.verify_current` | `absent` | SDK orchestration planned by 05; shared closure absent | `R0` | no operation-shaped public/runtime seam exists; `R0` | all three / none | omit |
| M62-038 | `snapshot.resolve_applicable` | `absent` | SDK orchestration planned by 05; shared closure absent | `R0` | no operation-shaped public/runtime seam exists; `R0` | all three / none | omit |
| M62-039 | `repository.setup.plan` | `absent` | SDK composition planned by 05; shared closure absent | `R0` | `crates/sdk/src/setup.rs::plan_setup` is `#[cfg(test)]` and is excluded; `R0` | all three / none | omit |
| M62-040 | `repository.setup.apply` | `live_precursor`: `crates/sdk/src/lib.rs::HandbookSdkV1::run_setup` | SDK repository composition precursor; shared closure absent | `C1` | `Result<SetupOutcome, SetupError>`; HCM outcome/key/receipt mapping absent; `C1` | all three / none (Rust precursor only) | omit |
| M62-041 | `repository.doctor` | `live_precursor`: `crates/sdk/src/lib.rs::HandbookSdkV1::doctor` | SDK owner composition precursor; shared closure absent | `R0` | `Result<DoctorReport, DoctorError>`; HCM mapping absent; `R0` | all three / none (Rust precursor only) | omit |
| M62-042 | `flow.resolve` | `live_precursor`: `crates/sdk/src/lib.rs::HandbookSdkV1::generate_packet`; owner `crates/flow/src/resolver.rs::resolve` | flow through SDK precursor; shared closure absent | `R0` | `Result<GeneratePacketResult, FlowRouteError>`; HCM mapping absent; `R0` | all three / none (Rust precursor only) | omit |
| M62-043 | `pipeline.catalog.list` | `live_precursor`: `crates/sdk/src/pipeline_route_api.rs::list_pipeline_catalog` | pipeline through SDK precursor; shared closure absent | `R0` | `Result<PipelineCatalogOutcome, String>`; HCM mapping absent; `R0` | all three / none (Rust precursor only) | omit |
| M62-044 | `pipeline.catalog.read` | `live_precursor`: `crates/sdk/src/pipeline_route_api.rs::show_pipeline` | pipeline through SDK precursor; shared closure absent | `R0` | `Result<PipelineShowOutcome, PipelineShowFailure>`; HCM mapping absent; `R0` | all three / none (Rust precursor only) | omit |
| M62-045 | `pipeline.route.resolve` | `live_precursor`: `crates/sdk/src/pipeline_route_api.rs::resolve_pipeline` | pipeline through SDK precursor; shared closure absent | planned `R0`; live precursor persists route basis, so it cannot be admitted as this read-only operation | `Result<PipelineResolveOutcome, PipelineResolveFailure>`; semantic mismatch and HCM mapping absent | all three / none (Rust precursor only) | omit |
| M62-046 | `pipeline.compile` | `live_precursor`: `crates/sdk/src/pipeline_route_api.rs::compile_pipeline` | pipeline through SDK precursor; shared closure absent | `R0` | `Result<PipelineCompileOutcome, PipelineCompileRefusalView>`; HCM mapping absent; `R0` | all three / none (Rust precursor only) | omit |
| M62-047 | `pipeline.capture.plan` | `live_precursor`: `crates/sdk/src/pipeline_route_api.rs::preview_pipeline_capture` | pipeline through SDK precursor; shared closure absent | `R0` | `Result<PipelineCapturePreviewView, PipelineCaptureRefusalView>`; HCM mapping absent; `R0` | all three / none (Rust precursor only) | omit |
| M62-048 | `pipeline.capture.apply` | `live_precursor`: `crates/sdk/src/pipeline_route_api.rs::apply_pipeline_capture` | pipeline through SDK precursor; shared closure absent | `C1` | `Result<PipelineCaptureApplyView, PipelineCaptureRefusalView>`; HCM key/outcome/receipt mapping absent; `C1` | all three / none (Rust precursor only) | omit |
| M62-049 | `pipeline.handoff.emit` | `live_precursor`: `crates/sdk/src/pipeline_route_api.rs::emit_pipeline_handoff` | pipeline through SDK precursor; shared closure absent | `A1` | `Result<PipelineHandoffOutcome, PipelineHandoffRefusalView>`; HCM key/outcome/receipt mapping absent; `A1` | all three / none (Rust precursor only) | omit |
| M62-050 | `pipeline.state.apply` | `live_precursor`: `crates/sdk/src/pipeline_route_api.rs::update_pipeline_state` | pipeline through SDK precursor; shared closure absent | `C1` | `Result<PipelineStateUpdateOutcome, PipelineStateUpdateFailure>`; HCM key/outcome/receipt mapping absent; `C1` | all three / none (Rust precursor only) | omit |
| M62-051 | `contract.definition.list` | `phase5_deferred` | future `handbook-contracts`; no owner/shared closure | `R0` | Phase-5 result/outcome mapping absent; `R0` | all three / none | omit |
| M62-052 | `contract.definition.read` | `phase5_deferred` | future `handbook-contracts`; no owner/shared closure | `R0` | Phase-5 result/outcome mapping absent; `R0` | all three / none | omit |
| M62-053 | `contract.definition.append` | `phase5_deferred` | future `handbook-contracts`; no owner/shared closure | `A1` | Phase-5 result/key/outcome/receipt mapping absent; `A1` | all three / none | omit |
| M62-054 | `contract.lifecycle.transition` | `phase5_deferred` | future `handbook-contracts`; no owner/shared closure | `C1` | Phase-5 result/key/outcome/receipt mapping absent; `C1` | all three / none | omit |
| M62-055 | `contract.evidence.list` | `phase5_deferred` | future `handbook-contracts`; no owner/shared closure | `R0` | Phase-5 result/outcome mapping absent; `R0` | all three / none | omit |
| M62-056 | `contract.evidence.read` | `phase5_deferred` | future `handbook-contracts`; no owner/shared closure | `R0` | Phase-5 result/outcome mapping absent; `R0` | all three / none | omit |
| M62-057 | `contract.evidence.append` | `phase5_deferred` | future `handbook-contracts`; no owner/shared closure | `A1` | Phase-5 result/key/outcome/receipt mapping absent; `A1` | all three / none | omit |
| M62-058 | `contract.verdict.evaluate` | `phase5_deferred` | future `handbook-contracts`; no owner/shared closure | `R0` | Phase-5 result/outcome mapping absent; `R0` | all three / none | omit |
| M62-059 | `contract.gate.evaluate` | `phase5_deferred` | future `handbook-contracts`; no owner/shared closure | `R0` | Phase-5 result/outcome mapping absent; `R0` | all three / none | omit |
| M62-060 | `dock.manifest.list` | `phase5_deferred` | future `handbook-contracts`; no owner/shared closure | `R0` | Phase-5 result/outcome mapping absent; `R0` | all three / none | omit |
| M62-061 | `dock.manifest.read` | `phase5_deferred` | future `handbook-contracts`; no owner/shared closure | `R0` | Phase-5 result/outcome mapping absent; `R0` | all three / none | omit |
| M62-062 | `dock.run` | `phase5_deferred` | future contracts semantic owner plus separable executor; no owner/shared closure | `A1` | Phase-5 result/key/outcome/receipt mapping absent; `A1` | all three / none | omit |

## Mechanical consequences

- Exact matrix row count is 62; `M62-001` through `M62-062` are contiguous
  and unique.
- The operation-ID set is equal to the canonical 05 inventory, HCM-4.1
  `tasks/plan.md`, and the predecessor HCM-4.2 source inventory; it has 50
  HCM-4-owned rows and 12 Phase-5-deferred rows.
- `live_precursor` rows are source-location facts only. All 62 rows have
  incomplete shared closure, zero current admitted operation transports, and
  an `omit` discovery result.
- The `pipeline.route.resolve` mismatch is explicit: its live precursor writes
  a route basis while the planned operation is read-only. Future implementation
  must select or introduce an owner adapter whose behavior matches the frozen
  definition; this document does not resolve that implementation choice.
