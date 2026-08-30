//! Non-UI Tauri command adapter over the transport-neutral Handbook SDK.

#![deny(missing_docs)]

use handbook_sdk::contract_membrane::{
    canonical_json_bytes, BootstrapFacadeResponseV1, BootstrapOutcome, BootstrapRequestV1,
    CatalogKindV1, CatalogReadRequestV1, CatalogReadResponseV1, ContractError, ContractLimits,
    InvocationRequestV1, InvocationResponseV1, ProductionCatalogRead, ProductionWitness,
    UtcTimestamp,
};
use handbook_sdk::HandbookSdkV1;
use serde::Serialize;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};

/// Supplies trusted UTC time from the host rather than from an IPC payload.
pub trait TrustedClock: Send + Sync + 'static {
    /// Returns the host's current UTC time, or `None` when trusted time is unavailable.
    fn now_utc(&self) -> Option<UtcTimestamp>;
}

/// Bounded host/framework failures for which no truthful SDK response exists.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case", tag = "code")]
pub enum HostError {
    /// The host-selected repository directory could not be normalized.
    RepositoryUnavailable,
    /// The immutable production witness could not be constructed.
    WitnessUnavailable,
    /// The host's trusted clock could not provide a value.
    ClockUnavailable,
    /// A response could not satisfy the transport-neutral contract.
    ContractUnavailable,
    /// Tauri's blocking executor did not complete the task.
    RuntimeUnavailable,
}

/// Host-managed state shared by all Handbook Tauri commands.
#[derive(Clone)]
pub struct FacadeState {
    inner: Arc<TrustedFacadeState>,
}

struct TrustedFacadeState {
    _repository_root: PathBuf,
    _retention_until_utc: UtcTimestamp,
    _sdk: HandbookSdkV1,
    witness: ProductionWitness,
    limits: ContractLimits,
    clock: Arc<dyn TrustedClock>,
    snapshot_source: Mutex<Option<Box<dyn handbook_engine::SnapshotSourcePortV1 + Send>>>,
}

impl FacadeState {
    /// Constructs the trusted facade state for one existing host-selected repository.
    ///
    /// The repository root is normalized once and is never accepted from or exposed to IPC.
    pub fn open(
        repository_root: impl AsRef<Path>,
        retention_until_utc: UtcTimestamp,
        clock: impl TrustedClock,
    ) -> Result<Self, HostError> {
        Self::open_inner(repository_root, retention_until_utc, clock, None)
    }

    /// Constructs trusted facade state with the fixed session-grounding source adapter enabled.
    ///
    /// The source adapter remains host-owned and is never accepted from or exposed to IPC.
    pub fn open_with_snapshot_source<P>(
        repository_root: impl AsRef<Path>,
        retention_until_utc: UtcTimestamp,
        clock: impl TrustedClock,
        snapshot_source: P,
    ) -> Result<Self, HostError>
    where
        P: handbook_engine::SnapshotSourcePortV1 + Send + 'static,
    {
        Self::open_inner(
            repository_root,
            retention_until_utc,
            clock,
            Some(Box::new(snapshot_source)),
        )
    }

    fn open_inner(
        repository_root: impl AsRef<Path>,
        retention_until_utc: UtcTimestamp,
        clock: impl TrustedClock,
        snapshot_source: Option<Box<dyn handbook_engine::SnapshotSourcePortV1 + Send>>,
    ) -> Result<Self, HostError> {
        let repository_root =
            std::fs::canonicalize(repository_root).map_err(|_| HostError::RepositoryUnavailable)?;
        if !repository_root.is_dir() {
            return Err(HostError::RepositoryUnavailable);
        }
        let witness = ProductionWitness::from_current(retention_until_utc.clone())
            .map_err(|_| HostError::WitnessUnavailable)?;
        let sdk = HandbookSdkV1::open(&repository_root);
        Ok(Self {
            inner: Arc::new(TrustedFacadeState {
                _repository_root: repository_root,
                _retention_until_utc: retention_until_utc,
                _sdk: sdk,
                witness,
                limits: ContractLimits::packet0(),
                clock: Arc::new(clock),
                snapshot_source: Mutex::new(snapshot_source),
            }),
        })
    }
}

macro_rules! command_registry {
    ($($command:ident),+ $(,)?) => {
        /// Exact transport-visible command IDs registered by this adapter.
        pub const COMMAND_IDS: [&str; 3] = [$(stringify!($command)),+];

        /// Builds the single Tauri invoke handler containing the exact command registry.
        pub fn invoke_handler<R: tauri::Runtime>() -> Box<tauri::ipc::InvokeHandler<R>> {
            Box::new(tauri::generate_handler![$($command),+])
        }
    };
}

command_registry!(
    handbook_bootstrap_v1,
    handbook_catalog_read_v1,
    handbook_invoke_v1,
);

#[tauri::command]
async fn handbook_bootstrap_v1(
    request: BootstrapRequestV1,
    state: tauri::State<'_, FacadeState>,
) -> Result<BootstrapFacadeResponseV1, HostError> {
    let inner = Arc::clone(&state.inner);
    run_off_main_thread(move || execute_bootstrap(&inner, request)).await
}

#[tauri::command]
async fn handbook_catalog_read_v1(
    request: CatalogReadRequestV1,
    state: tauri::State<'_, FacadeState>,
) -> Result<CatalogReadResponseV1, HostError> {
    let inner = Arc::clone(&state.inner);
    run_off_main_thread(move || execute_catalog(&inner, request)).await
}

#[tauri::command]
async fn handbook_invoke_v1(
    request: InvocationRequestV1,
    state: tauri::State<'_, FacadeState>,
) -> Result<InvocationResponseV1, HostError> {
    let inner = Arc::clone(&state.inner);
    run_off_main_thread(move || execute_invocation(&inner, request)).await
}

async fn run_off_main_thread<T, F>(task: F) -> Result<T, HostError>
where
    T: Send + 'static,
    F: FnOnce() -> Result<T, HostError> + Send + 'static,
{
    tauri::async_runtime::spawn_blocking(task)
        .await
        .map_err(|_| HostError::RuntimeUnavailable)?
}

fn execute_bootstrap(
    state: &TrustedFacadeState,
    request: BootstrapRequestV1,
) -> Result<BootstrapFacadeResponseV1, HostError> {
    let now_utc = state.clock.now_utc().ok_or(HostError::ClockUnavailable)?;
    let response = match state
        .witness
        .bootstrap(&request, &now_utc, &state.limits)
        .map_err(contract_error)?
    {
        BootstrapOutcome::Ok(response) => BootstrapFacadeResponseV1::ok(*response, &state.limits),
        BootstrapOutcome::Refused(response) => {
            BootstrapFacadeResponseV1::refused(*response, &state.limits)
        }
    }
    .map_err(contract_error)?;
    readmit_bootstrap(response, &state.limits)
}

fn execute_catalog(
    state: &TrustedFacadeState,
    request: CatalogReadRequestV1,
) -> Result<CatalogReadResponseV1, HostError> {
    let now_utc = state.clock.now_utc().ok_or(HostError::ClockUnavailable)?;
    let kind = request.catalog_kind();
    let response = match kind {
        CatalogKindV1::Capabilities => close_catalog_read(
            kind,
            state
                .witness
                .read_capabilities(
                    request.page_size(),
                    request.cursor(),
                    &now_utc,
                    &state.limits,
                )
                .map_err(contract_error)?,
            CatalogReadResponseV1::capability_page,
            &state.limits,
        ),
        CatalogKindV1::Schemas => close_catalog_read(
            kind,
            state
                .witness
                .read_schemas(
                    request.page_size(),
                    request.cursor(),
                    &now_utc,
                    &state.limits,
                )
                .map_err(contract_error)?,
            CatalogReadResponseV1::schema_page,
            &state.limits,
        ),
        CatalogKindV1::Profiles => close_catalog_read(
            kind,
            state
                .witness
                .read_profiles(
                    request.page_size(),
                    request.cursor(),
                    &now_utc,
                    &state.limits,
                )
                .map_err(contract_error)?,
            CatalogReadResponseV1::profile_page,
            &state.limits,
        ),
    }
    .map_err(contract_error)?;
    readmit_catalog(response, &state.limits)
}

fn close_catalog_read<T>(
    kind: CatalogKindV1,
    read: ProductionCatalogRead<T>,
    page: fn(T, &ContractLimits) -> Result<CatalogReadResponseV1, ContractError>,
    limits: &ContractLimits,
) -> Result<CatalogReadResponseV1, ContractError> {
    match read {
        ProductionCatalogRead::Page(value) => page(value, limits),
        ProductionCatalogRead::CursorMismatch => {
            CatalogReadResponseV1::cursor_mismatch(kind, limits)
        }
        ProductionCatalogRead::Blocked(value) => {
            CatalogReadResponseV1::blocked(kind, *value, limits)
        }
    }
}

fn execute_invocation(
    state: &TrustedFacadeState,
    request: InvocationRequestV1,
) -> Result<InvocationResponseV1, HostError> {
    execute_invocation_with_fault(state, request, InvocationFault::None)
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum InvocationFault {
    None,
    #[cfg(test)]
    PreCommit,
    #[cfg(test)]
    PostCommit,
}

fn execute_invocation_with_fault(
    state: &TrustedFacadeState,
    request: InvocationRequestV1,
    fault: InvocationFault,
) -> Result<InvocationResponseV1, HostError> {
    #[cfg(not(test))]
    let _ = fault;
    #[cfg(test)]
    if fault == InvocationFault::PreCommit {
        return pre_commit_transport_failure(&request, &state.limits);
    }
    let response = {
        let mut snapshot_source = state
            .snapshot_source
            .lock()
            .map_err(|_| HostError::RuntimeUnavailable)?;
        match snapshot_source.as_deref_mut() {
            Some(source) => {
                state
                    ._sdk
                    .invoke_tauri_with_snapshot_source(&request, &state.limits, source)
            }
            None => state._sdk.invoke_tauri(&request, &state.limits),
        }
        .map_err(contract_error)?
    };
    let response = readmit_invocation(state, &request, response)?;
    #[cfg(test)]
    if fault == InvocationFault::PostCommit {
        return Err(HostError::ContractUnavailable);
    }
    Ok(response)
}

#[cfg(test)]
fn pre_commit_transport_failure(
    request: &InvocationRequestV1,
    limits: &ContractLimits,
) -> Result<InvocationResponseV1, HostError> {
    use handbook_sdk::contract_membrane::{
        operation_inventory, InvocationRequestBodyV1, OperationTransport, TransportFailureKind,
        TransportFailureV1,
    };

    let InvocationRequestBodyV1::Operation(operation) = request.request() else {
        return Err(HostError::ContractUnavailable);
    };
    let selected = operation_inventory().iter().any(|row| {
        row.operation_ref() == operation.operation().reference()
            && row
                .current_transports()
                .contains(&OperationTransport::Tauri)
    });
    if !selected {
        return Err(HostError::ContractUnavailable);
    }
    let response = InvocationResponseV1::transport_failure(
        TransportFailureV1::adapter_error(
            TransportFailureKind::Adapter,
            "transport.invocation_construction_failed",
            limits,
        )
        .map_err(contract_error)?,
        limits,
    )
    .map_err(contract_error)?;
    let bytes = canonical_json_bytes(&response, limits).map_err(contract_error)?;
    let admitted =
        InvocationResponseV1::parse_and_admit(&bytes, None, limits).map_err(contract_error)?;
    let exact = admitted == response
        && admitted.transport_failure_kind() == Some(TransportFailureKind::Adapter)
        && admitted
            .transport_failure_reason_code(limits)
            .map_err(contract_error)?
            .as_deref()
            == Some("transport.invocation_construction_failed")
        && admitted
            .transport_presentation(limits)
            .map_err(contract_error)?
            .is_none();
    if exact {
        Ok(admitted)
    } else {
        Err(HostError::ContractUnavailable)
    }
}

fn readmit_bootstrap(
    response: BootstrapFacadeResponseV1,
    limits: &ContractLimits,
) -> Result<BootstrapFacadeResponseV1, HostError> {
    let bytes = canonical_json_bytes(&response, limits).map_err(contract_error)?;
    BootstrapFacadeResponseV1::parse_and_admit(&bytes, limits).map_err(contract_error)
}

fn readmit_catalog(
    response: CatalogReadResponseV1,
    limits: &ContractLimits,
) -> Result<CatalogReadResponseV1, HostError> {
    let bytes = canonical_json_bytes(&response, limits).map_err(contract_error)?;
    CatalogReadResponseV1::parse_and_admit(&bytes, limits).map_err(contract_error)
}

fn readmit_invocation(
    state: &TrustedFacadeState,
    request: &InvocationRequestV1,
    response: InvocationResponseV1,
) -> Result<InvocationResponseV1, HostError> {
    state
        ._sdk
        .readmit_invocation_response(request, &response, &state.limits)
        .map_err(contract_error)
}

fn contract_error(_: ContractError) -> HostError {
    HostError::ContractUnavailable
}

#[cfg(test)]
mod tests {
    use super::*;
    use handbook_engine::{
        DefinitionFingerprint, SnapshotGroundingOwnerV1, SnapshotSlotObservationV1,
        SnapshotSourceFamilyV1, SnapshotSourcePortErrorV1, SnapshotSourcePortV1,
    };
    use handbook_sdk::artifact::{
        ArtifactAcquisitionMode, ArtifactInputDocument, ArtifactSdk, ArtifactSelector,
        ArtifactValidateRequest,
    };
    use handbook_sdk::contract_membrane::{
        artifact_operation_request, flow_resolve_operation_request,
        pipeline_capture_direct_operation_request, pipeline_capture_plan_operation_request,
        pipeline_catalog_list_operation_request, pipeline_catalog_read_operation_request,
        pipeline_compile_operation_request, pipeline_handoff_emit_operation_request,
        pipeline_state_apply_operation_request, posture_transition_apply_request,
        ApproverAdministrationRequestV1, ArtifactOperationInputV1, AuthorCharterRequestV1,
        AuthorProjectContextRequestV1, BootstrapFacadeOutcomeKindV1, BootstrapRequestV1,
        CatalogKindV1, CatalogPageSize, CatalogReadOutcomeKindV1, CatalogReadRequestV1,
        CatalogReadResponseBodyV1, ContractLimits, ExactBinding, ExactRef, Fingerprint,
        InvocationRequestV1, InvocationResponseKind, PipelineRouteResolveAndPersistRequestV1,
        PostureCanonicalBindingDtoV1, PostureDimensionChangeDtoV1, PostureReferenceDtoV1,
        PostureTransitionApplyRequestDataV1, ProjectContextInputV1, RepositorySetupModeV1,
        RepositorySetupRequestV1, SemVer, SessionGroundingBoundaryKindV1,
        SessionGroundingBoundaryRequestV1, SessionGroundingBoundaryUseCaseOutcomeV1,
        SessionGroundingTerminalV1, TransportFailureKind, TypedCatalogCursor, UseCaseRequestV1,
        UtcTimestamp,
    };
    use handbook_sdk::pipeline_route_api::PipelineStateMutation;
    use handbook_sdk::{SetupMode, SetupRequest};
    use serde_json::{json, Value};
    use std::collections::BTreeMap;
    use std::path::{Path, PathBuf};
    use std::thread;
    use tempfile::tempdir;

    #[derive(Clone)]
    struct FixedClock(UtcTimestamp);

    impl TrustedClock for FixedClock {
        fn now_utc(&self) -> Option<UtcTimestamp> {
            Some(self.0.clone())
        }
    }

    struct CompleteSnapshotSource {
        observations: BTreeMap<(SnapshotSourceFamilyV1, &'static str), SnapshotSlotObservationV1>,
        values: BTreeMap<(SnapshotSourceFamilyV1, &'static str), Value>,
    }

    impl CompleteSnapshotSource {
        fn new() -> Self {
            let owner = SnapshotGroundingOwnerV1::new();
            let mut observations = BTreeMap::new();
            let mut values = BTreeMap::new();
            for contract in owner.source_contracts() {
                for &slot in contract.source_slots() {
                    let reference = format!(
                        "handbook.snapshot-observation.{}.{}@1.0.0",
                        contract.family().as_str(),
                        slot.replace('_', "-")
                    );
                    let value = json!({"ref": reference});
                    observations.insert(
                        (contract.family(), slot),
                        SnapshotSlotObservationV1::retained(
                            &reference,
                            &DefinitionFingerprint::from_json_value(&value)
                                .unwrap()
                                .to_string(),
                        )
                        .unwrap(),
                    );
                    values.insert((contract.family(), slot), value);
                }
            }
            observations.insert(
                (SnapshotSourceFamilyV1::Git, "worktree"),
                SnapshotSlotObservationV1::redacted(
                    "handbook.snapshot-observation.git.worktree@1.0.0",
                    &DefinitionFingerprint::from_json_value(&json!({"redacted": true}))
                        .unwrap()
                        .to_string(),
                )
                .unwrap(),
            );
            observations.insert(
                (SnapshotSourceFamilyV1::Evidence, "dock_execution_candidate"),
                SnapshotSlotObservationV1::omitted("absent_or_inapplicable"),
            );
            observations.insert(
                (SnapshotSourceFamilyV1::Work, "handoff"),
                SnapshotSlotObservationV1::inapplicable("policy_inapplicable"),
            );
            let decision = values
                .get(&(SnapshotSourceFamilyV1::Evidence, "contracts_decision"))
                .unwrap()
                .clone();
            observations.insert(
                (SnapshotSourceFamilyV1::Evidence, "contracts_decision"),
                SnapshotSlotObservationV1::derived(
                    "handbook.snapshot-observation.evidence.contracts-decision@1.0.0",
                    &DefinitionFingerprint::from_json_value(&decision)
                        .unwrap()
                        .to_string(),
                )
                .unwrap(),
            );
            let resolution = json!({
                "resolution": {
                    "stack_fingerprint": "sha256:4444444444444444444444444444444444444444444444444444444444444444",
                    "stack_ref": "handbook.context-resolution-stack.packet-4-tauri@1.0.0",
                    "values": ["task", "repository", "session", "bounded", "execution", "verified"]
                }
            });
            observations.insert(
                (
                    SnapshotSourceFamilyV1::Handbook,
                    "schema_artifact_resolution",
                ),
                SnapshotSlotObservationV1::retained(
                    "handbook.snapshot-observation.handbook.schema-artifact-resolution@1.0.0",
                    &DefinitionFingerprint::from_json_value(&resolution)
                        .unwrap()
                        .to_string(),
                )
                .unwrap(),
            );
            values.insert(
                (
                    SnapshotSourceFamilyV1::Handbook,
                    "schema_artifact_resolution",
                ),
                resolution,
            );
            Self {
                observations,
                values,
            }
        }
    }

    impl SnapshotSourcePortV1 for CompleteSnapshotSource {
        fn observe_slot(
            &mut self,
            family: SnapshotSourceFamilyV1,
            slot: &'static str,
        ) -> Result<SnapshotSlotObservationV1, SnapshotSourcePortErrorV1> {
            self.observations
                .get(&(family, slot))
                .cloned()
                .ok_or_else(SnapshotSourcePortErrorV1::missing)
        }

        fn read_bounded_slot(
            &mut self,
            family: SnapshotSourceFamilyV1,
            slot: &'static str,
        ) -> Result<Value, SnapshotSourcePortErrorV1> {
            self.values
                .get(&(family, slot))
                .cloned()
                .ok_or_else(SnapshotSourcePortErrorV1::missing)
        }
    }

    fn session_workspace() -> ExactBinding {
        ExactBinding::new(
            ExactRef::parse("handbook.workspace.packet-4-tauri@1.0.0").unwrap(),
            Fingerprint::parse(
                "sha256:4444444444444444444444444444444444444444444444444444444444444444",
            )
            .unwrap(),
        )
    }

    fn state(repo_root: &Path, now: &str, retention: &str) -> FacadeState {
        FacadeState::open(
            repo_root,
            UtcTimestamp::parse(retention).unwrap(),
            FixedClock(UtcTimestamp::parse(now).unwrap()),
        )
        .unwrap()
    }

    fn bootstrap_request(state: &FacadeState, api_version: &str) -> BootstrapRequestV1 {
        BootstrapRequestV1::new(
            state
                .inner
                .witness
                .descriptor()
                .schemas()
                .bootstrap_request()
                .clone(),
            None,
            state.inner.witness.descriptor().binding(),
            SemVer::parse(api_version).unwrap(),
            CatalogPageSize::new(1).unwrap(),
            &state.inner.limits,
        )
        .unwrap()
    }

    fn catalog_request(
        kind: CatalogKindV1,
        page_size: u64,
        cursor: Option<TypedCatalogCursor>,
    ) -> CatalogReadRequestV1 {
        CatalogReadRequestV1::new(
            kind,
            CatalogPageSize::new(page_size).unwrap(),
            cursor,
            &ContractLimits::packet0(),
        )
        .unwrap()
    }

    fn invocation_request() -> InvocationRequestV1 {
        InvocationRequestV1::operation(
            pipeline_catalog_list_operation_request(Some("tauri-packet-2")).unwrap(),
            &ContractLimits::packet0(),
        )
        .unwrap()
    }

    fn read_only_invocation_requests() -> Vec<InvocationRequestV1> {
        let limits = ContractLimits::packet0();
        let coverage = ArtifactInputDocument::new(
            serde_json::to_vec(&json!({
                "coverage_submissions": [
                    {
                        "coverage_id": "registry_brief.title",
                        "state": "supplied",
                        "source_kind": "user_declaration",
                        "value": "Packet 3 parity",
                        "specificity": "exact",
                        "confidence": "high",
                        "contradiction_refs": []
                    },
                    {
                        "coverage_id": "registry_brief.summary",
                        "state": "supplied",
                        "source_kind": "user_declaration",
                        "value": "Read-only invocation dispatcher",
                        "specificity": "concrete",
                        "confidence": "high",
                        "contradiction_refs": []
                    }
                ]
            }))
            .unwrap(),
        )
        .unwrap();
        let kind_ref = "example.artifact-kind.registry-brief@1.0.0";
        let instance_id = "registry_brief";
        let operation_requests = vec![
            artifact_operation_request(None, ArtifactOperationInputV1::KindList).unwrap(),
            artifact_operation_request(None, ArtifactOperationInputV1::InstanceList).unwrap(),
            artifact_operation_request(
                None,
                ArtifactOperationInputV1::Read {
                    kind_ref: kind_ref.to_owned(),
                    instance_id: instance_id.to_owned(),
                },
            )
            .unwrap(),
            artifact_operation_request(
                None,
                ArtifactOperationInputV1::Validate {
                    kind_ref: kind_ref.to_owned(),
                    instance_id: instance_id.to_owned(),
                },
            )
            .unwrap(),
            artifact_operation_request(
                None,
                ArtifactOperationInputV1::IntakeDefinitionRead {
                    kind_ref: kind_ref.to_owned(),
                    instance_id: instance_id.to_owned(),
                },
            )
            .unwrap(),
            artifact_operation_request(
                None,
                ArtifactOperationInputV1::IntakeCoverageEvaluate {
                    kind_ref: kind_ref.to_owned(),
                    instance_id: instance_id.to_owned(),
                    mode: ArtifactAcquisitionMode::Express,
                    expected_current_artifact_fingerprint: None,
                    input: coverage,
                },
            )
            .unwrap(),
            artifact_operation_request(
                None,
                ArtifactOperationInputV1::CandidateValidate {
                    kind_ref: kind_ref.to_owned(),
                    instance_id: instance_id.to_owned(),
                    intake_record_ref: ".handbook/state/artifacts/registry_brief/intake-records/intake_0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef.json".to_owned(),
                    intake_record_fingerprint: "sha256:0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef".to_owned(),
                    expected_current_artifact_fingerprint: None,
                },
            )
            .unwrap(),
            flow_resolve_operation_request(None, "planning.packet").unwrap(),
            pipeline_catalog_list_operation_request(None).unwrap(),
            pipeline_catalog_read_operation_request(None, "pipeline.foundation_inputs").unwrap(),
            pipeline_compile_operation_request(
                None,
                "pipeline.foundation_inputs",
                "stage.10_feature_spec",
            )
            .unwrap(),
            pipeline_capture_plan_operation_request(
                None,
                "pipeline.foundation_inputs",
                "stage.10_feature_spec",
                "packet-3 preview\n",
            )
            .unwrap(),
        ];
        operation_requests
            .into_iter()
            .map(|request| InvocationRequestV1::operation(request, &limits).unwrap())
            .collect()
    }

    fn mutation_invocation_requests() -> Vec<InvocationRequestV1> {
        const HASH: &str =
            "sha256:0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef";
        let limits = ContractLimits::packet0();
        let document =
            |value| ArtifactInputDocument::new(serde_json::to_vec(&value).unwrap()).unwrap();
        let kind_ref = "example.artifact-kind.registry-brief@1.0.0";
        let instance_id = "registry_brief";
        vec![
            artifact_operation_request(
                Some("tauri-intake"),
                ArtifactOperationInputV1::IntakeRecordAppend {
                    kind_ref: kind_ref.to_owned(),
                    instance_id: instance_id.to_owned(),
                    input: document(json!({
                        "idempotency_key": "tauri_intake_key_0001",
                        "acquisition_mode": "express",
                        "expected_current_artifact_fingerprint": HASH,
                        "coverage_submissions": []
                    })),
                },
            )
            .unwrap(),
            artifact_operation_request(
                Some("tauri-candidate"),
                ArtifactOperationInputV1::CandidateAppend {
                    kind_ref: kind_ref.to_owned(),
                    instance_id: instance_id.to_owned(),
                    input: document(json!({
                        "idempotency_key": "tauri_candidate_key_0001",
                        "intake_record_ref": ".handbook/state/artifacts/registry_brief/intake-records/intake_0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef.json",
                        "intake_record_fingerprint": HASH,
                        "expected_candidate_fingerprint": HASH
                    })),
                },
            )
            .unwrap(),
            artifact_operation_request(
                Some("tauri-promote"),
                ArtifactOperationInputV1::CandidatePromote {
                    kind_ref: kind_ref.to_owned(),
                    instance_id: instance_id.to_owned(),
                    input: document(json!({
                        "idempotency_key": "tauri_promotion_key_0001",
                        "candidate_ref": ".handbook/state/artifacts/registry_brief/candidates/candidate_0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef.json",
                        "candidate_fingerprint": HASH,
                        "expected_current_artifact_fingerprint": HASH
                    })),
                },
            )
            .unwrap(),
            pipeline_capture_direct_operation_request(
                Some("tauri-capture"),
                "pipeline.foundation_inputs",
                "stage.10_feature_spec",
                "tauri packet 5 capture",
                "tauri_capture_key_0001",
            )
            .unwrap(),
            pipeline_handoff_emit_operation_request(
                Some("tauri-handoff"),
                "pipeline.foundation_inputs",
                "consumer.example",
                "handbook",
                "1.0.0",
                "tauri_handoff_key_0001",
            )
            .unwrap(),
            pipeline_state_apply_operation_request(
                Some("tauri-state"),
                "pipeline.foundation_inputs",
                &PipelineStateMutation::RoutingVariable {
                    variable: "has_charter".to_owned(),
                    value: true,
                },
                0,
                "tauri_state_key_0001",
            )
            .unwrap(),
            posture_transition_apply_request(
                Some("tauri-posture"),
                "tauri_posture_key_0001",
                PostureTransitionApplyRequestDataV1::new(
                    HASH.to_owned(),
                    PostureCanonicalBindingDtoV1::new(
                        ".handbook/project/charter.yaml".to_owned(),
                        HASH.to_owned(),
                        HASH.to_owned(),
                        1,
                    )
                    .unwrap(),
                    PostureReferenceDtoV1::new(
                        "recommendation.tauri-packet5".to_owned(),
                        HASH.to_owned(),
                    )
                    .unwrap(),
                    vec!["coverage.tauri-packet5".to_owned()],
                    PostureDimensionChangeDtoV1::new(
                        "dimension.tauri-packet5".to_owned(),
                        0,
                        "/posture/dimensions/0".to_owned(),
                        None,
                        0,
                        1,
                    )
                    .unwrap(),
                    "2030-01-01T00:00:00Z".to_owned(),
                )
                .unwrap(),
            )
            .unwrap(),
        ]
        .into_iter()
        .map(|request| InvocationRequestV1::operation(request, &limits).unwrap())
        .collect()
    }

    fn copy_tree(source: &Path, target: &Path) {
        std::fs::create_dir_all(target).unwrap();
        for entry in std::fs::read_dir(source).unwrap() {
            let entry = entry.unwrap();
            let target_path = target.join(entry.file_name());
            if entry.path().is_dir() {
                copy_tree(&entry.path(), &target_path);
            } else {
                std::fs::copy(entry.path(), target_path).unwrap();
            }
        }
    }

    fn repository_snapshot(root: &Path) -> BTreeMap<PathBuf, Vec<u8>> {
        fn collect(root: &Path, current: &Path, snapshot: &mut BTreeMap<PathBuf, Vec<u8>>) {
            for entry in std::fs::read_dir(current).unwrap() {
                let entry = entry.unwrap();
                let path = entry.path();
                if path.is_dir() {
                    collect(root, &path, snapshot);
                } else {
                    snapshot.insert(
                        path.strip_prefix(root).unwrap().to_path_buf(),
                        std::fs::read(path).unwrap(),
                    );
                }
            }
        }
        let mut snapshot = BTreeMap::new();
        collect(root, root, &mut snapshot);
        snapshot
    }

    fn use_case_invocation_requests() -> Vec<InvocationRequestV1> {
        let limits = ContractLimits::packet0();
        let context = ProjectContextInputV1::new(
            "handbook.artifact.project-context",
            "1.0",
            "handbook.project-context",
            "Packet 4 Tauri request",
            vec!["SDK facade".to_owned()],
            vec!["Project owner".to_owned()],
            vec!["docs/START_HERE.md".to_owned()],
            vec!["None".to_owned()],
        )
        .unwrap();
        [
            UseCaseRequestV1::repository_setup(
                RepositorySetupRequestV1::new(RepositorySetupModeV1::Refresh, false, false),
                &limits,
            )
            .unwrap(),
            UseCaseRequestV1::repository_doctor(&limits).unwrap(),
            UseCaseRequestV1::author_charter(AuthorCharterRequestV1::Validate, &limits).unwrap(),
            UseCaseRequestV1::author_project_context(
                AuthorProjectContextRequestV1::Validate(context),
                &limits,
            )
            .unwrap(),
            UseCaseRequestV1::approver_administration(
                ApproverAdministrationRequestV1::revoke_credential("sha256:packet-four").unwrap(),
                &limits,
            )
            .unwrap(),
            UseCaseRequestV1::pipeline_route_resolve_and_persist(
                PipelineRouteResolveAndPersistRequestV1::new("packet-four-missing").unwrap(),
                &limits,
            )
            .unwrap(),
        ]
        .into_iter()
        .map(|request| InvocationRequestV1::use_case(request, &limits).unwrap())
        .collect()
    }

    fn inject_repository_root<T>(request: &T) -> Value
    where
        T: serde::Serialize,
    {
        let mut value = serde_json::to_value(request).unwrap();
        value
            .as_object_mut()
            .unwrap()
            .insert("repository_root".to_owned(), json!("C:/untrusted"));
        value
    }

    fn contains_path(value: &Value, path: &str) -> bool {
        match value {
            Value::String(candidate) => candidate.contains(path),
            Value::Array(values) => values.iter().any(|value| contains_path(value, path)),
            Value::Object(values) => values.values().any(|value| contains_path(value, path)),
            _ => false,
        }
    }

    #[test]
    fn command_registry_is_exact_and_generated_from_the_same_list() {
        assert_eq!(
            COMMAND_IDS,
            [
                "handbook_bootstrap_v1",
                "handbook_catalog_read_v1",
                "handbook_invoke_v1",
            ]
        );
        assert_eq!(COMMAND_IDS.len(), 3);
    }

    #[test]
    fn host_state_normalizes_the_root_and_errors_are_bounded() {
        let repo = tempdir().unwrap();
        let selected = repo.path().join(".");
        let state = state(&selected, "2029-01-01T00:00:00Z", "2030-01-01T00:00:00Z");
        assert_eq!(
            state.inner._repository_root,
            repo.path().canonicalize().unwrap()
        );

        let missing = repo.path().join("private-missing-root");
        let error = FacadeState::open(
            &missing,
            UtcTimestamp::parse("2030-01-01T00:00:00Z").unwrap(),
            FixedClock(UtcTimestamp::parse("2029-01-01T00:00:00Z").unwrap()),
        )
        .err()
        .unwrap();
        assert_eq!(
            serde_json::to_value(error).unwrap(),
            json!({"code": "repository_unavailable"})
        );
        assert!(!serde_json::to_string(&error)
            .unwrap()
            .contains("private-missing-root"));
    }

    #[test]
    fn ipc_request_shapes_cannot_supply_a_repository_root() {
        let repo = tempdir().unwrap();
        let state = state(repo.path(), "2029-01-01T00:00:00Z", "2030-01-01T00:00:00Z");
        assert!(
            serde_json::from_value::<BootstrapRequestV1>(inject_repository_root(
                &bootstrap_request(&state, "1.0.0")
            ))
            .is_err()
        );
        assert!(
            serde_json::from_value::<CatalogReadRequestV1>(inject_repository_root(
                &catalog_request(CatalogKindV1::Schemas, 1, None)
            ))
            .is_err()
        );
        assert!(
            serde_json::from_value::<InvocationRequestV1>(inject_repository_root(
                &invocation_request()
            ))
            .is_err()
        );
    }

    #[test]
    fn bootstrap_resolves_success_and_negotiation_refusal_as_typed_values() {
        let repo = tempdir().unwrap();
        let state = state(repo.path(), "2029-01-01T00:00:00Z", "2030-01-01T00:00:00Z");
        let accepted = execute_bootstrap(&state.inner, bootstrap_request(&state, "1.0.0")).unwrap();
        assert_eq!(accepted.outcome_kind(), BootstrapFacadeOutcomeKindV1::Ok);

        let refused = execute_bootstrap(&state.inner, bootstrap_request(&state, "2.0.0")).unwrap();
        assert_eq!(
            refused.outcome_kind(),
            BootstrapFacadeOutcomeKindV1::Refused
        );

        let normalized = state.inner._repository_root.to_string_lossy();
        assert!(!contains_path(
            &serde_json::to_value(accepted).unwrap(),
            normalized.as_ref()
        ));
        assert!(!contains_path(
            &serde_json::to_value(refused).unwrap(),
            normalized.as_ref()
        ));
    }

    #[test]
    fn catalog_preserves_all_pages_cursor_mismatch_and_expiry_blocking() {
        let repo = tempdir().unwrap();
        let current = state(repo.path(), "2029-01-01T00:00:00Z", "2030-01-01T00:00:00Z");
        for kind in [
            CatalogKindV1::Capabilities,
            CatalogKindV1::Schemas,
            CatalogKindV1::Profiles,
        ] {
            let response = execute_catalog(&current.inner, catalog_request(kind, 1, None)).unwrap();
            assert_eq!(response.catalog_kind(), kind);
            assert_eq!(response.outcome_kind(), CatalogReadOutcomeKindV1::Page);
        }

        let first = execute_catalog(
            &current.inner,
            catalog_request(CatalogKindV1::Schemas, 1, None),
        )
        .unwrap();
        let cursor = match first.response() {
            CatalogReadResponseBodyV1::SchemaPage(page) => page.next_cursor().unwrap().clone(),
            other => panic!("expected schema page, got {other:?}"),
        };
        let mismatch = execute_catalog(
            &current.inner,
            catalog_request(CatalogKindV1::Schemas, 2, Some(cursor)),
        )
        .unwrap();
        assert_eq!(
            mismatch.outcome_kind(),
            CatalogReadOutcomeKindV1::CursorMismatch
        );

        let expired = state(repo.path(), "2031-01-01T00:00:00Z", "2030-01-01T00:00:00Z");
        let blocked = execute_catalog(
            &expired.inner,
            catalog_request(CatalogKindV1::Profiles, 1, None),
        )
        .unwrap();
        assert_eq!(blocked.outcome_kind(), CatalogReadOutcomeKindV1::Blocked);
    }

    #[test]
    fn invocation_runs_all_twelve_read_only_routes_off_main_thread_without_reshaping() {
        let repo = tempdir().unwrap();
        let state = state(repo.path(), "2029-01-01T00:00:00Z", "2030-01-01T00:00:00Z");
        let requests = read_only_invocation_requests();
        assert_eq!(requests.len(), 12);

        for request in requests {
            let expected = state
                .inner
                ._sdk
                .invoke(&request, &state.inner.limits)
                .unwrap();
            let caller = thread::current().id();
            let inner = Arc::clone(&state.inner);
            let (worker, actual) = tauri::async_runtime::block_on(run_off_main_thread(move || {
                let worker = thread::current().id();
                execute_invocation(&inner, request).map(|response| (worker, response))
            }))
            .unwrap();

            assert_ne!(caller, worker);
            assert_eq!(actual.response_kind(), InvocationResponseKind::Operation);
            assert_eq!(actual, expected);
        }
    }

    #[test]
    fn invocation_runs_all_six_use_cases_off_main_thread_with_fixed_tauri_presentation() {
        let requests = use_case_invocation_requests();
        assert_eq!(requests.len(), 6);

        for (index, request) in requests.into_iter().enumerate() {
            let direct_repo = tempdir().unwrap();
            let tauri_repo = tempdir().unwrap();
            if index == 4 {
                for root in [direct_repo.path(), tauri_repo.path()] {
                    std::fs::create_dir(root.join(".git")).unwrap();
                    let _ = HandbookSdkV1::open(root).run_setup(&SetupRequest {
                        mode: SetupMode::Init,
                        rewrite: false,
                        reset_state: false,
                    });
                }
            }
            let expected = HandbookSdkV1::open(direct_repo.path())
                .invoke_tauri(&request, &ContractLimits::packet0())
                .unwrap();
            let state = state(
                tauri_repo.path(),
                "2029-01-01T00:00:00Z",
                "2030-01-01T00:00:00Z",
            );
            let caller = thread::current().id();
            let inner = Arc::clone(&state.inner);
            let (worker, actual) = tauri::async_runtime::block_on(run_off_main_thread(move || {
                let worker = thread::current().id();
                execute_invocation(&inner, request).map(|response| (worker, response))
            }))
            .unwrap_or_else(|error| panic!("use-case {index}: {error:?}"));

            assert_ne!(caller, worker);
            assert_eq!(actual.response_kind(), InvocationResponseKind::UseCase);
            assert_eq!(actual, expected);
            if index == 3 {
                let document = serde_json::to_value(actual).unwrap();
                let presentation = document["response"]["data"]
                    .as_object()
                    .map(|_| &document["response"]["data"])
                    .unwrap_or(&document["response"]["refusals"][0]["details"]);
                assert_eq!(presentation["input_mode"], "structured_inputs_tauri");
                assert_eq!(presentation["input_source"], "tauri");
            }
        }
    }

    #[test]
    fn fixed_session_grounding_start_and_end_use_the_unchanged_generic_tauri_command() {
        let repo = tempdir().unwrap();
        let limits = ContractLimits::packet0();
        let state = FacadeState::open_with_snapshot_source(
            repo.path(),
            UtcTimestamp::parse("2030-01-01T00:00:00Z").unwrap(),
            FixedClock(UtcTimestamp::parse("2029-01-01T00:00:00Z").unwrap()),
            CompleteSnapshotSource::new(),
        )
        .unwrap();

        for (sequence, boundary, key, expected_terminal) in [
            (
                1,
                SessionGroundingBoundaryKindV1::Start,
                "tauri-session-start-key",
                SessionGroundingTerminalV1::Started,
            ),
            (
                2,
                SessionGroundingBoundaryKindV1::End,
                "tauri-session-end-key",
                SessionGroundingTerminalV1::Finalized,
            ),
        ] {
            let request = UseCaseRequestV1::session_grounding_boundary(
                SessionGroundingBoundaryRequestV1::new(
                    session_workspace(),
                    "tauri-session-packet-4",
                    boundary,
                    key,
                )
                .unwrap(),
                &limits,
            )
            .unwrap();
            let request = InvocationRequestV1::use_case(request, &limits).unwrap();
            let response = execute_invocation(&state.inner, request).unwrap();
            assert_eq!(response.response_kind(), InvocationResponseKind::UseCase);
            let Some(SessionGroundingBoundaryUseCaseOutcomeV1::Composed(result)) = response
                .session_grounding_boundary_outcome(&limits)
                .unwrap()
            else {
                panic!("generic Tauri invocation must compose session grounding")
            };
            assert_eq!(result.sequence(), sequence);
            assert_eq!(result.terminal(), expected_terminal);
            assert_eq!(result.flow_status(), expected_terminal);
            assert_eq!(result.pipeline_status(), expected_terminal);
        }

        assert_eq!(COMMAND_IDS.len(), 3);
    }

    #[test]
    fn invocation_default_denies_mutation_and_non_target_without_repository_effects() {
        let repo = tempdir().unwrap();
        let sentinel = repo.path().join("sentinel.txt");
        std::fs::write(&sentinel, b"unchanged").unwrap();
        let state = state(repo.path(), "2029-01-01T00:00:00Z", "2030-01-01T00:00:00Z");
        let limits = ContractLimits::packet0();
        let mutation = InvocationRequestV1::operation(
            pipeline_capture_direct_operation_request(
                None,
                "pipeline.foundation_inputs",
                "stage.10_feature_spec",
                "must not execute",
                "packet-3-denied-mutation",
            )
            .unwrap(),
            &limits,
        )
        .unwrap();
        let response = execute_invocation(&state.inner, mutation).unwrap();
        assert_eq!(response.response_kind(), InvocationResponseKind::Operation);

        let mut non_target = serde_json::to_value(invocation_request()).unwrap();
        non_target["request"]["operation"]["ref"] = json!("pipeline.route.resolve@1.0.0");
        assert!(serde_json::from_value::<InvocationRequestV1>(non_target).is_err());

        assert_eq!(std::fs::read(&sentinel).unwrap(), b"unchanged");
    }

    #[test]
    fn invocation_runs_all_seven_mutation_routes_off_main_thread_with_owner_parity() {
        for request in mutation_invocation_requests() {
            let direct_repo = tempdir().unwrap();
            let tauri_repo = tempdir().unwrap();
            let expected = HandbookSdkV1::open(direct_repo.path())
                .invoke_tauri(&request, &ContractLimits::packet0())
                .unwrap();
            let state = state(
                tauri_repo.path(),
                "2029-01-01T00:00:00Z",
                "2030-01-01T00:00:00Z",
            );
            let caller = thread::current().id();
            let inner = Arc::clone(&state.inner);
            let (worker, actual) = tauri::async_runtime::block_on(run_off_main_thread(move || {
                let worker = thread::current().id();
                execute_invocation(&inner, request).map(|response| (worker, response))
            }))
            .unwrap();

            assert_ne!(caller, worker);
            assert_eq!(actual.response_kind(), InvocationResponseKind::Operation);
            assert_eq!(actual, expected);
        }
    }

    #[test]
    fn pre_commit_adapter_failure_is_re_admitted_and_performs_no_owner_write() {
        let repo = tempdir().unwrap();
        std::fs::write(repo.path().join("sentinel.txt"), b"unchanged").unwrap();
        let before = repository_snapshot(repo.path());
        let state = state(repo.path(), "2029-01-01T00:00:00Z", "2030-01-01T00:00:00Z");
        let request = mutation_invocation_requests().remove(0);
        let inner = Arc::clone(&state.inner);
        let response = tauri::async_runtime::block_on(run_off_main_thread(move || {
            execute_invocation_with_fault(&inner, request, InvocationFault::PreCommit)
        }))
        .unwrap();

        assert_eq!(
            response.response_kind(),
            InvocationResponseKind::TransportFailure
        );
        assert_eq!(
            response.transport_failure_kind(),
            Some(TransportFailureKind::Adapter)
        );
        assert_eq!(
            response
                .transport_failure_reason_code(&ContractLimits::packet0())
                .unwrap()
                .as_deref(),
            Some("transport.invocation_construction_failed")
        );
        assert_eq!(repository_snapshot(repo.path()), before);
    }

    #[test]
    fn post_commit_delivery_failure_returns_only_host_error_then_same_key_replays_without_writes() {
        let repo = tempdir().unwrap();
        copy_tree(
            &Path::new(env!("CARGO_MANIFEST_DIR"))
                .join("../engine/tests/fixtures/hcm_2_3_generic_custom_kind"),
            repo.path(),
        );
        std::fs::write(
            repo.path().join(".handbook/repository-identity.v1"),
            "sha256:abcdef0123456789abcdef0123456789abcdef0123456789abcdef0123456789",
        )
        .unwrap();
        let current = ArtifactSdk::open(repo.path())
            .validate(ArtifactValidateRequest {
                selector: ArtifactSelector::new(
                    "example.artifact-kind.registry-brief@1.0.0",
                    "registry_brief",
                ),
            })
            .unwrap()
            .artifact_fingerprint;
        let request = || {
            let input = ArtifactInputDocument::new(
                serde_json::to_vec(&json!({
                    "idempotency_key": "tauri_delivery_uncertainty_key_0001",
                    "acquisition_mode": "express",
                    "expected_current_artifact_fingerprint": current,
                    "coverage_submissions": [
                        {
                            "coverage_id": "registry_brief.title",
                            "state": "supplied",
                            "source_kind": "user_declaration",
                            "value": "Delivery uncertainty",
                            "specificity": "exact",
                            "confidence": "high",
                            "contradiction_refs": []
                        },
                        {
                            "coverage_id": "registry_brief.summary",
                            "state": "supplied",
                            "source_kind": "user_declaration",
                            "value": "Same-key recovery",
                            "specificity": "concrete",
                            "confidence": "high",
                            "contradiction_refs": []
                        }
                    ]
                }))
                .unwrap(),
            )
            .unwrap();
            InvocationRequestV1::operation(
                artifact_operation_request(
                    Some("tauri-delivery-uncertainty"),
                    ArtifactOperationInputV1::IntakeRecordAppend {
                        kind_ref: "example.artifact-kind.registry-brief@1.0.0".to_owned(),
                        instance_id: "registry_brief".to_owned(),
                        input,
                    },
                )
                .unwrap(),
                &ContractLimits::packet0(),
            )
            .unwrap()
        };
        let state = state(repo.path(), "2029-01-01T00:00:00Z", "2030-01-01T00:00:00Z");
        let before = repository_snapshot(repo.path());
        let inner = Arc::clone(&state.inner);
        let fault_request = request();
        let failure = tauri::async_runtime::block_on(run_off_main_thread(move || {
            execute_invocation_with_fault(&inner, fault_request, InvocationFault::PostCommit)
        }))
        .unwrap_err();
        assert_eq!(failure, HostError::ContractUnavailable);
        let committed = repository_snapshot(repo.path());
        assert_ne!(
            committed, before,
            "the owner commit must survive delivery loss"
        );

        let recovered = execute_invocation(&state.inner, request()).unwrap();
        let recovered_value = serde_json::to_value(&recovered).unwrap();
        assert_eq!(
            recovered_value["response"]["idempotency"]["state"],
            "established"
        );
        assert_eq!(recovered_value["response"]["idempotency"]["replayed"], true);
        assert_eq!(
            recovered_value["response"]["idempotency"]["original_result_fingerprint"],
            recovered_value["response"]["result_fingerprint"]
        );
        assert_eq!(
            recovered_value["response"]["receipts"]
                .as_array()
                .unwrap()
                .len(),
            1
        );
        assert_eq!(repository_snapshot(repo.path()), committed);

        let replayed_again = execute_invocation(&state.inner, request()).unwrap();
        assert_eq!(replayed_again, recovered);
        assert_eq!(repository_snapshot(repo.path()), committed);
        let wire = canonical_json_bytes(&recovered, &ContractLimits::packet0()).unwrap();
        assert!(!String::from_utf8(wire)
            .unwrap()
            .contains("tauri_delivery_uncertainty_key_0001"));
    }

    #[test]
    fn blocking_work_runs_on_the_dedicated_executor() {
        let caller = thread::current().id();
        let worker = tauri::async_runtime::block_on(run_off_main_thread(move || {
            Ok::<_, HostError>(thread::current().id())
        }))
        .unwrap();
        assert_ne!(caller, worker);
    }
}
