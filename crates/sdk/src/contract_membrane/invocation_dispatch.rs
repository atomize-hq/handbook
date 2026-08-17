use super::artifact_operation::{ArtifactSchemaValidator, ARTIFACT_OPERATION_REFS};
use super::flow_operation::FLOW_OPERATION_REF;
use super::pipeline_operation::{
    PIPELINE_CAPTURE_APPLY_REF, PIPELINE_CAPTURE_PLAN_REF, PIPELINE_COMPILE_REF,
    PIPELINE_HANDOFF_EMIT_REF, PIPELINE_LIST_REF, PIPELINE_READ_REF, PIPELINE_STATE_APPLY_REF,
};
use super::{
    artifact_operation_definition, canonical_json_bytes, execute_artifact_operation,
    execute_flow_operation, execute_pipeline_operation, flow_resolve_operation_definition,
    pipeline_operation_definitions, ContractError, ContractLimits, InvocationRequestBodyV1,
    InvocationRequestV1, InvocationResponseV1, OperationClass, OperationDefinition,
    OperationRequestV1, OperationSchemaValidator, PipelineOperationSchemaValidator,
    TransportFailureKind, TransportFailureV1,
};
use super::{FlowResolveSchemaValidator, OperationTransport};
use std::path::Path;

const ROUTE_NOT_READY: &str = "transport.invocation_route_not_ready";

#[derive(Clone, Copy)]
enum OperationRoute {
    Artifact,
    Flow,
    Pipeline,
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
        }
    }

    fn validator(self) -> &'static dyn OperationSchemaValidator {
        match self {
            Self::Artifact => &ArtifactSchemaValidator,
            Self::Flow => &FlowResolveSchemaValidator,
            Self::Pipeline => &PipelineOperationSchemaValidator,
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
        }
    }
}

pub(crate) fn dispatch_invocation(
    repo_root: &Path,
    request: &InvocationRequestV1,
    limits: &ContractLimits,
) -> Result<InvocationResponseV1, ContractError> {
    require_packet_limits(limits)?;
    readmit_request(request, limits)?;
    let Some((route, operation)) = read_only_route(request)? else {
        return default_denial(limits);
    };
    let response = route.execute(repo_root, operation)?;
    InvocationResponseV1::operation_response(response, limits)
}

pub(crate) fn readmit_invocation_response(
    request: &InvocationRequestV1,
    response: &InvocationResponseV1,
    limits: &ContractLimits,
) -> Result<InvocationResponseV1, ContractError> {
    require_packet_limits(limits)?;
    let bytes = canonical_json_bytes(response, limits)?;
    if let Some((route, operation)) = read_only_route(request)? {
        let definition = route.definition(operation.operation().reference().as_str())?;
        return InvocationResponseV1::parse_and_admit(
            &bytes,
            Some((&definition, route.validator())),
            limits,
        );
    }
    InvocationResponseV1::parse_and_admit(&bytes, None, limits)
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

fn read_only_route(
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
    } else {
        return Ok(None);
    };
    let definition = route.definition(operation_ref)?;
    if definition.class() != OperationClass::ReadOnly
        || !definition
            .declared_transports()
            .contains(&OperationTransport::CliJson)
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
