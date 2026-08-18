use super::artifact_operation::{ArtifactSchemaValidator, ARTIFACT_OPERATION_REFS};
use super::facade::SdkUseCaseRequest;
use super::flow_operation::FLOW_OPERATION_REF;
use super::pipeline_operation::{
    PIPELINE_CAPTURE_APPLY_REF, PIPELINE_CAPTURE_PLAN_REF, PIPELINE_COMPILE_REF,
    PIPELINE_HANDOFF_EMIT_REF, PIPELINE_LIST_REF, PIPELINE_READ_REF, PIPELINE_STATE_APPLY_REF,
};
use super::posture::{PostureSchemaValidator, OPERATION_REF as POSTURE_OPERATION_REF};
use super::{
    artifact_operation_definition, canonical_json_bytes, execute_artifact_operation,
    execute_flow_operation, execute_pipeline_operation, flow_resolve_operation_definition,
    pipeline_operation_definitions, posture_transition_apply_definition, ContractError,
    ContractLimits, InvocationRequestBodyV1, InvocationRequestV1, InvocationResponseKind,
    InvocationResponseV1, OperationDefinition, OperationRequestV1, OperationSchemaValidator,
    PipelineOperationSchemaValidator, TransportFailureKind, TransportFailureV1,
};
use super::{FlowResolveSchemaValidator, OperationTransport};
use crate::HandbookSdkV1;
use std::path::Path;

const ROUTE_NOT_READY: &str = "transport.invocation_route_not_ready";

#[derive(Clone, Copy)]
enum OperationRoute {
    Artifact,
    Flow,
    Pipeline,
    Posture,
}

#[derive(Clone, Copy)]
enum UseCasePresentation {
    RustSdk,
    Tauri,
}

impl OperationRoute {
    fn definition(self, operation_ref: &str) -> Result<OperationDefinition, ContractError> {
        match self {
            Self::Artifact => artifact_operation_definition(operation_ref),
            Self::Flow => flow_resolve_operation_definition(),
            Self::Pipeline => pipeline_operation_definitions()?
                .into_iter()
                .find(|definition| definition.operation().as_str() == operation_ref)
                .ok_or(ContractError::InvalidInvariant),
            Self::Posture => posture_transition_apply_definition(),
        }
    }

    fn validator(self) -> &'static dyn OperationSchemaValidator {
        match self {
            Self::Artifact => &ArtifactSchemaValidator,
            Self::Flow => &FlowResolveSchemaValidator,
            Self::Pipeline => &PipelineOperationSchemaValidator,
            Self::Posture => &PostureSchemaValidator,
        }
    }

    fn execute(
        self,
        repo_root: &Path,
        request: &OperationRequestV1,
    ) -> Result<super::OperationResponseV1, ContractError> {
        match self {
            Self::Artifact => execute_artifact_operation(repo_root, request),
            Self::Flow => execute_flow_operation(repo_root, request),
            Self::Pipeline => execute_pipeline_operation(repo_root, request),
            Self::Posture => {
                HandbookSdkV1::open(repo_root).apply_posture_transition_operation(request)
            }
        }
    }
}

pub(crate) fn dispatch_invocation(
    repo_root: &Path,
    request: &InvocationRequestV1,
    limits: &ContractLimits,
) -> Result<InvocationResponseV1, ContractError> {
    dispatch_invocation_with_presentation(repo_root, request, UseCasePresentation::RustSdk, limits)
}

pub(crate) fn dispatch_tauri_invocation(
    repo_root: &Path,
    request: &InvocationRequestV1,
    limits: &ContractLimits,
) -> Result<InvocationResponseV1, ContractError> {
    dispatch_invocation_with_presentation(repo_root, request, UseCasePresentation::Tauri, limits)
}

fn dispatch_invocation_with_presentation(
    repo_root: &Path,
    request: &InvocationRequestV1,
    presentation: UseCasePresentation,
    limits: &ContractLimits,
) -> Result<InvocationResponseV1, ContractError> {
    require_packet_limits(limits)?;
    readmit_request(request, limits)?;
    match request.request() {
        InvocationRequestBodyV1::Operation(_) => {
            let Some((route, operation)) = operation_route(request)? else {
                return default_denial(limits);
            };
            let response = route.execute(repo_root, operation)?;
            InvocationResponseV1::operation_response(response, limits)
        }
        InvocationRequestBodyV1::UseCase(request) => dispatch_use_case(
            repo_root,
            request.request().sdk_request(),
            presentation,
            limits,
        ),
    }
}

pub(crate) fn readmit_invocation_response(
    request: &InvocationRequestV1,
    response: &InvocationResponseV1,
    limits: &ContractLimits,
) -> Result<InvocationResponseV1, ContractError> {
    require_packet_limits(limits)?;
    let bytes = canonical_json_bytes(response, limits)?;
    if matches!(request.request(), InvocationRequestBodyV1::Operation(_)) {
        let Some((route, operation)) = operation_route(request)? else {
            let admitted = InvocationResponseV1::parse_and_admit(&bytes, None, limits)?;
            let is_exact_default_denial = admitted.response_kind()
                == InvocationResponseKind::TransportFailure
                && admitted.transport_failure_kind() == Some(TransportFailureKind::Adapter)
                && admitted.transport_failure_reason_code(limits)?.as_deref()
                    == Some(ROUTE_NOT_READY)
                && admitted.transport_presentation(limits)?.is_none();
            return if is_exact_default_denial {
                Ok(admitted)
            } else {
                Err(ContractError::InvalidInvariant)
            };
        };
        let definition = route.definition(operation.operation().reference().as_str())?;
        let admitted = InvocationResponseV1::parse_and_admit(
            &bytes,
            Some((&definition, route.validator())),
            limits,
        )?;
        let Some((response_operation, request_fingerprint, request_id)) =
            admitted.operation_request_binding()
        else {
            return Err(ContractError::InvalidInvariant);
        };
        if response_operation != operation.operation()
            || request_fingerprint != operation.request_fingerprint()
            || request_id != operation.request_id()
        {
            return Err(ContractError::InvalidInvariant);
        }
        return Ok(admitted);
    }
    let admitted = InvocationResponseV1::parse_and_admit(&bytes, None, limits)?;
    if let InvocationRequestBodyV1::UseCase(request) = request.request() {
        let matches_request = match admitted.response_kind() {
            super::InvocationResponseKind::UseCase => {
                admitted.use_case_binding() == Some(request.use_case())
            }
            super::InvocationResponseKind::TransportFailure => {
                transport_failure_matches(request.request(), &admitted, limits)?
            }
            super::InvocationResponseKind::Operation => false,
        };
        if !matches_request {
            return Err(ContractError::InvalidInvariant);
        }
    }
    Ok(admitted)
}

fn transport_failure_matches(
    request: &super::UseCaseRequestBodyV1,
    response: &InvocationResponseV1,
    limits: &ContractLimits,
) -> Result<bool, ContractError> {
    let Some(super::TransportPresentationViewV1::RepositoryInvocation { command, .. }) =
        response.transport_presentation(limits)?
    else {
        return Ok(false);
    };
    Ok(matches!(
        (request, command),
        (
            super::UseCaseRequestBodyV1::AuthorCharter(
                super::AuthorCharterRequestV1::Author { .. }
            ),
            super::RepositoryInvocationCommandV1::CharterAuthor
        ) | (
            super::UseCaseRequestBodyV1::AuthorCharter(
                super::AuthorCharterRequestV1::Approve { .. }
            ),
            super::RepositoryInvocationCommandV1::CharterApprove
        ) | (
            super::UseCaseRequestBodyV1::AuthorCharter(
                super::AuthorCharterRequestV1::Promote { .. }
            ),
            super::RepositoryInvocationCommandV1::CharterPromote
        ) | (
            super::UseCaseRequestBodyV1::AuthorCharter(super::AuthorCharterRequestV1::Validate),
            super::RepositoryInvocationCommandV1::CharterValidate
        ) | (
            super::UseCaseRequestBodyV1::ApproverAdministration(
                super::ApproverAdministrationRequestV1::Bootstrap { .. }
            ),
            super::RepositoryInvocationCommandV1::ApproverBootstrap
        ) | (
            super::UseCaseRequestBodyV1::ApproverAdministration(
                super::ApproverAdministrationRequestV1::AddCredential { .. }
            ),
            super::RepositoryInvocationCommandV1::ApproverAddCredential
        ) | (
            super::UseCaseRequestBodyV1::ApproverAdministration(
                super::ApproverAdministrationRequestV1::RevokeCredential { .. }
            ),
            super::RepositoryInvocationCommandV1::ApproverRevokeCredential
        ) | (
            super::UseCaseRequestBodyV1::ApproverAdministration(
                super::ApproverAdministrationRequestV1::UpdateMapping { .. }
            ),
            super::RepositoryInvocationCommandV1::ApproverUpdateMapping
        )
    ))
}

fn dispatch_use_case(
    repo_root: &Path,
    request: SdkUseCaseRequest,
    presentation: UseCasePresentation,
    limits: &ContractLimits,
) -> Result<InvocationResponseV1, ContractError> {
    let sdk = HandbookSdkV1::open(repo_root);
    match request {
        SdkUseCaseRequest::RepositorySetup(request) => sdk
            .run_setup_use_case(&request, limits)
            .and_then(|response| InvocationResponseV1::use_case(response, limits)),
        SdkUseCaseRequest::RepositoryDoctor => {
            let response = match sdk.doctor() {
                Ok(report) => super::UseCaseResponseV1::repository_doctor(&report, limits),
                Err(error) => super::UseCaseResponseV1::repository_doctor_error(error, limits),
            }?;
            InvocationResponseV1::use_case(response, limits)
        }
        SdkUseCaseRequest::AuthorCharter(request) => {
            sdk.run_author_charter_use_case(request, limits)
        }
        SdkUseCaseRequest::AuthorProjectContext(request) => {
            let (input_mode, input_source) = project_context_presentation(presentation);
            sdk.run_author_project_context_use_case(input_mode, input_source, request, limits)
                .and_then(|response| InvocationResponseV1::use_case(response, limits))
        }
        SdkUseCaseRequest::ApproverAdministration(request) => {
            sdk.run_approver_administration_use_case(request, limits)
        }
        SdkUseCaseRequest::PipelineRouteResolveAndPersist(selector) => sdk
            .resolve_pipeline_use_case(&selector, limits)
            .and_then(|response| InvocationResponseV1::use_case(response, limits)),
    }
}

fn project_context_presentation(presentation: UseCasePresentation) -> (&'static str, &'static str) {
    match presentation {
        UseCasePresentation::RustSdk => ("structured_inputs_rust_sdk", "rust_sdk"),
        UseCasePresentation::Tauri => ("structured_inputs_tauri", "tauri"),
    }
}

fn readmit_request(
    request: &InvocationRequestV1,
    limits: &ContractLimits,
) -> Result<(), ContractError> {
    let bytes = canonical_json_bytes(request, limits)?;
    let admitted = InvocationRequestV1::parse_and_admit(&bytes, limits)?;
    if canonical_json_bytes(&admitted, limits)? != bytes {
        return Err(ContractError::InvalidInvariant);
    }
    Ok(())
}

fn operation_route(
    request: &InvocationRequestV1,
) -> Result<Option<(OperationRoute, &OperationRequestV1)>, ContractError> {
    let operation = match request.request() {
        InvocationRequestBodyV1::Operation(operation) => operation,
        InvocationRequestBodyV1::UseCase(_) => return Ok(None),
    };
    let operation_ref = operation.operation().reference().as_str();
    let route = if ARTIFACT_OPERATION_REFS.contains(&operation_ref) {
        OperationRoute::Artifact
    } else if operation_ref == FLOW_OPERATION_REF {
        OperationRoute::Flow
    } else if [
        PIPELINE_LIST_REF,
        PIPELINE_READ_REF,
        PIPELINE_COMPILE_REF,
        PIPELINE_CAPTURE_PLAN_REF,
        PIPELINE_CAPTURE_APPLY_REF,
        PIPELINE_HANDOFF_EMIT_REF,
        PIPELINE_STATE_APPLY_REF,
    ]
    .contains(&operation_ref)
    {
        OperationRoute::Pipeline
    } else if operation_ref == POSTURE_OPERATION_REF {
        OperationRoute::Posture
    } else {
        return Ok(None);
    };
    let definition = route.definition(operation_ref)?;
    let required_predecessor_transport = if matches!(route, OperationRoute::Posture) {
        OperationTransport::RustSdk
    } else {
        OperationTransport::CliJson
    };
    if !definition
        .declared_transports()
        .contains(&required_predecessor_transport)
        || operation.operation() != &definition.binding()
        || operation.body_schema() != definition.request_schema()
    {
        return Ok(None);
    }
    Ok(Some((route, operation)))
}

fn default_denial(limits: &ContractLimits) -> Result<InvocationResponseV1, ContractError> {
    let failure =
        TransportFailureV1::adapter_error(TransportFailureKind::Adapter, ROUTE_NOT_READY, limits)?;
    InvocationResponseV1::transport_failure(failure, limits)
}

fn require_packet_limits(limits: &ContractLimits) -> Result<(), ContractError> {
    if limits == &ContractLimits::packet0() {
        Ok(())
    } else {
        Err(ContractError::LimitOutOfRange)
    }
}
