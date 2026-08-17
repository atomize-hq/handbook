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
use std::sync::Arc;

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
    let response = state
        ._sdk
        .invoke(&request, &state.limits)
        .map_err(contract_error)?;
    readmit_invocation(state, &request, response)
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
    use handbook_sdk::artifact::{ArtifactAcquisitionMode, ArtifactInputDocument};
    use handbook_sdk::contract_membrane::{
        artifact_operation_request, flow_resolve_operation_request,
        pipeline_capture_direct_operation_request, pipeline_capture_plan_operation_request,
        pipeline_catalog_list_operation_request, pipeline_catalog_read_operation_request,
        pipeline_compile_operation_request, ArtifactOperationInputV1, BootstrapFacadeOutcomeKindV1,
        BootstrapRequestV1, CatalogKindV1, CatalogPageSize, CatalogReadOutcomeKindV1,
        CatalogReadRequestV1, CatalogReadResponseBodyV1, ContractLimits, InvocationRequestV1,
        InvocationResponseKind, SemVer, TransportFailureKind, TypedCatalogCursor, UseCaseRequestV1,
        UtcTimestamp,
    };
    use serde_json::{json, Value};
    use std::path::Path;
    use std::thread;
    use tempfile::tempdir;

    #[derive(Clone)]
    struct FixedClock(UtcTimestamp);

    impl TrustedClock for FixedClock {
        fn now_utc(&self) -> Option<UtcTimestamp> {
            Some(self.0.clone())
        }
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
    fn invocation_default_denies_mutation_use_case_and_non_target_without_repository_effects() {
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
        let use_case = InvocationRequestV1::use_case(
            UseCaseRequestV1::repository_doctor(&limits).unwrap(),
            &limits,
        )
        .unwrap();

        for request in [mutation, use_case] {
            let response = execute_invocation(&state.inner, request).unwrap();
            assert_eq!(
                response.response_kind(),
                InvocationResponseKind::TransportFailure
            );
            assert_eq!(
                response.transport_failure_kind(),
                Some(TransportFailureKind::Adapter)
            );
            assert_eq!(
                response.transport_failure_reason_code(&limits).unwrap(),
                Some("transport.invocation_route_not_ready".to_owned())
            );
        }

        let mut non_target = serde_json::to_value(invocation_request()).unwrap();
        non_target["request"]["operation"]["ref"] = json!("pipeline.route.resolve@1.0.0");
        assert!(serde_json::from_value::<InvocationRequestV1>(non_target).is_err());

        assert_eq!(std::fs::read(&sentinel).unwrap(), b"unchanged");
        assert!(!repo.path().join(".handbook").exists());
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
