use handbook_engine::{
    ApproverAdminRequestV1, RepositoryInvocationIdentityServiceV1, RepositoryInvocationOperationV1,
    RepositoryInvocationPreflightResultV1, RepositoryInvocationPreparationFailureV1,
    RepositoryInvocationRecoveryRefusalV1,
};
use serde_json::{json, Value};

const REPOSITORY_IDENTITY: &str =
    "sha256:ee7147baa7ae6a6d858d9e253e503bc8c90c2ba23a5f1096b5717d1c3231e1db";

#[test]
fn typed_operations_serialize_to_the_closed_product_domain() {
    let cases = [
        (
            RepositoryInvocationOperationV1::ApproverBootstrap,
            "bootstrap",
        ),
        (
            RepositoryInvocationOperationV1::ApproverAddCredential,
            "add_credential",
        ),
        (
            RepositoryInvocationOperationV1::ApproverRevokeCredential,
            "revoke_credential",
        ),
        (
            RepositoryInvocationOperationV1::ApproverUpdateMapping,
            "update_mapping",
        ),
        (
            RepositoryInvocationOperationV1::CharterApproval,
            "charter_approval",
        ),
    ];

    for (operation, expected) in cases {
        assert_eq!(serde_json::to_value(operation).unwrap(), json!(expected));
    }
}

#[test]
fn production_allocations_are_engine_owned_and_have_exact_shapes() {
    let service = RepositoryInvocationIdentityServiceV1::new();
    let identity = service.allocate_repository_identity().unwrap();
    assert_eq!(identity.repository_identity_fingerprint().len(), 71);
    assert!(is_lowercase_sha256(
        identity.repository_identity_fingerprint()
    ));

    let allocation = service
        .allocate_operation_id(
            RepositoryInvocationOperationV1::ApproverUpdateMapping,
            identity.repository_identity_fingerprint(),
        )
        .unwrap();
    assert_eq!(
        allocation.repository_identity_fingerprint(),
        identity.repository_identity_fingerprint()
    );
    assert!(allocation.operation_id().starts_with("update-mapping-"));
    assert_eq!(allocation.operation_id().len(), 79);
}

#[test]
fn preflight_branches_serialize_exactly_and_reject_substitution() {
    let missing = RepositoryInvocationPreflightResultV1::repository_identity_unavailable(
        RepositoryInvocationOperationV1::ApproverBootstrap,
    );
    assert_eq!(
        serde_json::to_value(&missing).unwrap(),
        json!({
            "schema_id": "handbook.repository-invocation-preflight-result",
            "schema_version": "1.0",
            "operation": "bootstrap",
            "stage": "repository_identity",
            "status": "refused",
            "repository_identity_fingerprint": null,
            "operation_id": null,
            "changed_paths": [],
            "refusal": {
                "code": "repository_identity_unavailable",
                "message": "repository invocation identity is unavailable",
                "retryable": false
            },
            "next_actions": ["run or repair handbook setup, then retry the complete operation"]
        })
    );

    let entropy = RepositoryInvocationPreflightResultV1::operation_id_entropy_unavailable(
        RepositoryInvocationOperationV1::CharterApproval,
        REPOSITORY_IDENTITY,
    )
    .unwrap();
    let entropy_json = serde_json::to_value(&entropy).unwrap();
    assert_eq!(
        entropy_json,
        json!({
            "schema_id": "handbook.repository-invocation-preflight-result",
            "schema_version": "1.0",
            "operation": "charter_approval",
            "stage": "operation_id",
            "status": "refused",
            "repository_identity_fingerprint": REPOSITORY_IDENTITY,
            "operation_id": null,
            "changed_paths": [],
            "refusal": {
                "code": "operation_id_entropy_unavailable",
                "message": "operating-system randomness was unavailable for operation ID allocation",
                "retryable": true
            },
            "next_actions": ["retry the complete operation with fresh operating-system randomness"]
        })
    );

    let mut crossed = entropy_json.clone();
    crossed["repository_identity_fingerprint"] = Value::Null;
    assert!(serde_json::from_value::<RepositoryInvocationPreflightResultV1>(crossed).is_err());

    let mut substitution = entropy_json.clone();
    substitution["schema_id"] = json!("handbook.approver-admin-result");
    assert!(serde_json::from_value::<RepositoryInvocationPreflightResultV1>(substitution).is_err());

    let mut unknown = entropy_json;
    unknown["detail"] = json!("adapter invented detail");
    assert!(serde_json::from_value::<RepositoryInvocationPreflightResultV1>(unknown).is_err());
}

#[test]
fn recovery_refusal_is_distinct_closed_and_null_before_identity() {
    let refusal = RepositoryInvocationRecoveryRefusalV1::authority_recovery_blocked(
        RepositoryInvocationOperationV1::ApproverBootstrap,
    );
    let document = serde_json::to_value(&refusal).unwrap();
    assert_eq!(
        document,
        json!({
            "schema_id": "handbook.repository-invocation-recovery-refusal",
            "schema_version": "1.0",
            "operation": "bootstrap",
            "status": "refused",
            "repository_identity_fingerprint": null,
            "operation_id": null,
            "changed_paths": [],
            "refusal": {
                "code": "authority_recovery_blocked",
                "message": "repository authority recovery could not complete safely",
                "retryable": false
            },
            "next_actions": ["repair retained registry/approval recovery evidence, then retry the complete operation"]
        })
    );

    assert!(
        serde_json::from_value::<RepositoryInvocationPreflightResultV1>(document.clone()).is_err()
    );
    let mut non_null = document.clone();
    non_null["repository_identity_fingerprint"] = json!(REPOSITORY_IDENTITY);
    assert!(serde_json::from_value::<RepositoryInvocationRecoveryRefusalV1>(non_null).is_err());
    let mut unknown = document;
    unknown["detail"] = json!("unsafe evidence");
    assert!(serde_json::from_value::<RepositoryInvocationRecoveryRefusalV1>(unknown).is_err());
}

#[test]
fn typed_failure_accessors_preserve_each_closed_envelope_without_json_parsing() {
    let preflight = RepositoryInvocationPreparationFailureV1::Preflight(
        RepositoryInvocationPreflightResultV1::repository_identity_unavailable(
            RepositoryInvocationOperationV1::ApproverBootstrap,
        ),
    );
    assert_eq!(
        preflight.schema_id(),
        "handbook.repository-invocation-preflight-result"
    );
    assert_eq!(preflight.schema_version(), "1.0");
    assert_eq!(preflight.stage(), Some("repository_identity"));
    assert_eq!(preflight.status(), "refused");
    assert!(preflight.changed_paths().is_empty());
    assert_eq!(
        preflight.refusal().code(),
        "repository_identity_unavailable"
    );
    assert_eq!(preflight.next_actions().len(), 1);

    let recovery = RepositoryInvocationPreparationFailureV1::Recovery(
        RepositoryInvocationRecoveryRefusalV1::authority_recovery_blocked(
            RepositoryInvocationOperationV1::CharterApproval,
        ),
    );
    assert_eq!(
        recovery.schema_id(),
        "handbook.repository-invocation-recovery-refusal"
    );
    assert_eq!(recovery.schema_version(), "1.0");
    assert_eq!(recovery.stage(), None);
    assert_eq!(recovery.status(), "refused");
    assert!(recovery.changed_paths().is_empty());
    assert_eq!(recovery.refusal().code(), "authority_recovery_blocked");
    assert_eq!(recovery.next_actions().len(), 1);
}

#[test]
fn direct_engine_request_keeps_bounded_opaque_operation_id_compatibility() {
    let vectors: Value = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../docs/specs/handbook-contract-membrane/slices/HCM-2.2/contracts/approver-admin-api-vectors-v1.0.json"
    )))
    .unwrap();
    let mut request = vectors["vectors"]
        .as_array()
        .unwrap()
        .iter()
        .find(|vector| vector["vector_id"] == "bootstrap-request")
        .unwrap()["document"]
        .clone();
    request["operation_id"] = json!("opaque-direct-engine-correlation-1");

    let parsed = ApproverAdminRequestV1::from_json_value(request).unwrap();
    assert_eq!(parsed.operation_id(), "opaque-direct-engine-correlation-1");
}

fn is_lowercase_sha256(value: &str) -> bool {
    value.len() == 71
        && value.starts_with("sha256:")
        && value.as_bytes()[7..]
            .iter()
            .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(byte))
}
