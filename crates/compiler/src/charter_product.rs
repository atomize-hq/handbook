use handbook_engine::{
    load_selected_charter, resolve_shipped_profile_decisions, ApproverAdminRequestV1,
    ApproverAdminResultV1, ApproverRegistryObservationErrorKindV1, ApproverRegistryServiceV1,
    ArtifactInspectionReason, ArtifactInspectionStatus, CharterAcquisitionMode,
    CharterApprovalRequestV1, CharterApprovalResultV1, CharterApprovalServiceV1,
    CharterIntakeEnvelope, CharterPromotionIntentV1, CharterPromotionWorkflowCommitV1,
    CharterPromotionWorkflowErrorV1, CharterPromotionWorkflowServiceV1,
    NativeAuthenticatorPortErrorV1, NativeAuthenticatorPortV1,
    RepositoryInvocationIdentityServiceV1, RepositoryInvocationOperationV1,
    RepositoryInvocationPreparationFailureV1,
};
use serde::{Serialize, Serializer};
use serde_json::{json, Value};
use std::path::Path;

pub const CHARTER_OPERATION_RESULT_SCHEMA_ID: &str = "handbook.charter-operation-result";
pub const CHARTER_OPERATION_RESULT_SCHEMA_VERSION: &str = "1.0";
pub const APPROVER_ADAPTER_RESULT_SCHEMA_ID: &str = "handbook.approver-admin-adapter-result";
pub const APPROVER_ADAPTER_RESULT_SCHEMA_VERSION: &str = "1.0";

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CharterOperation {
    Author,
    Approve,
    Promote,
    Validate,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum AdapterOperationStatus {
    Succeeded,
    Refused,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct AdapterRefusal {
    pub code: String,
    pub message: String,
    pub retryable: bool,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CharterOperationResult {
    pub schema_id: String,
    pub schema_version: String,
    pub operation: CharterOperation,
    pub status: AdapterOperationStatus,
    pub canonical_path: Option<String>,
    pub source_fingerprint: Option<String>,
    pub rendered_output_fingerprint: Option<String>,
    pub intake_ref: Option<String>,
    pub intake_fingerprint: Option<String>,
    pub candidate_ref: Option<String>,
    pub candidate_fingerprint: Option<String>,
    pub approval_ref: Option<String>,
    pub approval_fingerprint: Option<String>,
    pub promotion_ref: Option<String>,
    pub promotion_fingerprint: Option<String>,
    pub changed_paths: Vec<String>,
    pub refusal: Option<AdapterRefusal>,
    pub next_actions: Vec<String>,
    projection: Option<RepositoryInvocationPreparationFailureV1>,
}

impl Serialize for CharterOperationResult {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        if let Some(projection) = &self.projection {
            return projection.serialize(serializer);
        }
        #[derive(Serialize)]
        struct Wire<'a> {
            schema_id: &'a str,
            schema_version: &'a str,
            operation: CharterOperation,
            status: AdapterOperationStatus,
            canonical_path: &'a Option<String>,
            source_fingerprint: &'a Option<String>,
            rendered_output_fingerprint: &'a Option<String>,
            intake_ref: &'a Option<String>,
            intake_fingerprint: &'a Option<String>,
            candidate_ref: &'a Option<String>,
            candidate_fingerprint: &'a Option<String>,
            approval_ref: &'a Option<String>,
            approval_fingerprint: &'a Option<String>,
            promotion_ref: &'a Option<String>,
            promotion_fingerprint: &'a Option<String>,
            changed_paths: &'a [String],
            refusal: &'a Option<AdapterRefusal>,
            next_actions: &'a [String],
        }
        Wire {
            schema_id: &self.schema_id,
            schema_version: &self.schema_version,
            operation: self.operation,
            status: self.status,
            canonical_path: &self.canonical_path,
            source_fingerprint: &self.source_fingerprint,
            rendered_output_fingerprint: &self.rendered_output_fingerprint,
            intake_ref: &self.intake_ref,
            intake_fingerprint: &self.intake_fingerprint,
            candidate_ref: &self.candidate_ref,
            candidate_fingerprint: &self.candidate_fingerprint,
            approval_ref: &self.approval_ref,
            approval_fingerprint: &self.approval_fingerprint,
            promotion_ref: &self.promotion_ref,
            promotion_fingerprint: &self.promotion_fingerprint,
            changed_paths: &self.changed_paths,
            refusal: &self.refusal,
            next_actions: &self.next_actions,
        }
        .serialize(serializer)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum CharterCommandIntent {
    Author {
        mode: CharterAcquisitionMode,
        envelope: CharterIntakeEnvelope,
    },
    Approve {
        candidate_ref: String,
        approval_class: String,
        authority_ref: String,
        accepted_waiver_refs: Vec<String>,
    },
    Promote {
        candidate_ref: String,
        approval_ref: String,
        expected_current_fingerprint: Option<String>,
    },
    Validate,
}

#[allow(clippy::result_large_err)]
pub fn parse_charter_intake_envelope(
    yaml: &str,
) -> Result<CharterIntakeEnvelope, CharterOperationResult> {
    serde_yaml_bw::from_str(yaml).map_err(|_| {
        refused_charter(
            CharterOperation::Author,
            "invalid_intake_envelope",
            "Charter intake input does not match the closed engine envelope",
            false,
            "repair the typed Charter intake envelope and retry with an explicit --mode",
        )
    })
}

pub fn execute_charter_command(
    repo_root: impl AsRef<Path>,
    intent: CharterCommandIntent,
) -> CharterOperationResult {
    execute_charter_command_with_port(repo_root, intent, UnavailableNativeAuthenticatorPortV1)
}

fn execute_charter_command_with_port<P: NativeAuthenticatorPortV1>(
    repo_root: impl AsRef<Path>,
    intent: CharterCommandIntent,
    port: P,
) -> CharterOperationResult {
    let repo_root = repo_root.as_ref();
    match intent {
        CharterCommandIntent::Validate => validate_selected_charter(repo_root),
        CharterCommandIntent::Author { mode, envelope } => {
            if mode != envelope.mode {
                return refused_charter(
                    CharterOperation::Author,
                    "acquisition_mode_mismatch",
                    "--mode must equal the acquisition mode in the typed intake envelope",
                    false,
                    "make the CLI mode and intake-envelope mode identical, then retry",
                );
            }
            let decisions = match resolve_shipped_profile_decisions(repo_root) {
                Ok(decisions) => decisions,
                Err(_) => return profile_refused(CharterOperation::Author),
            };
            let current = match load_selected_charter(repo_root, &decisions) {
                Ok(projection) => Some(projection.source_fingerprint().clone()),
                Err(error)
                    if error.status() == ArtifactInspectionStatus::Missing
                        && error.reason() == ArtifactInspectionReason::RequiredPathMissing =>
                {
                    None
                }
                Err(error) => return selected_charter_refused(CharterOperation::Author, &error),
            };
            let persistence = match handbook_engine::CharterAuthorPersistenceServiceV1::new(
                repo_root,
            )
            .persist(&decisions, envelope, current.as_ref())
            {
                Ok(persistence) => persistence,
                Err(error) => return refused_charter(
                    CharterOperation::Author,
                    match error.kind() {
                        handbook_engine::CharterAuthorPersistenceErrorKindV1::IntakeRefused => {
                            "charter_intake_refused"
                        }
                        handbook_engine::CharterAuthorPersistenceErrorKindV1::PersistenceRefused => {
                            "lineage_persistence_refused"
                        }
                    },
                    error.detail(),
                    error.kind()
                        == handbook_engine::CharterAuthorPersistenceErrorKindV1::PersistenceRefused,
                    "repair the refused Charter authority input or retained state, then retry",
                ),
            };
            CharterOperationResult {
                schema_id: CHARTER_OPERATION_RESULT_SCHEMA_ID.to_owned(),
                schema_version: CHARTER_OPERATION_RESULT_SCHEMA_VERSION.to_owned(),
                operation: CharterOperation::Author,
                status: AdapterOperationStatus::Succeeded,
                canonical_path: None,
                source_fingerprint: None,
                rendered_output_fingerprint: None,
                intake_ref: Some(persistence.intake_ref.clone()),
                intake_fingerprint: Some(persistence.intake_fingerprint.clone()),
                candidate_ref: Some(persistence.candidate_ref.clone()),
                candidate_fingerprint: Some(persistence.candidate_fingerprint.clone()),
                approval_ref: None,
                approval_fingerprint: None,
                promotion_ref: None,
                promotion_fingerprint: None,
                changed_paths: vec![
                    format!(".handbook/state/{}", persistence.normalized_content_ref),
                    format!(".handbook/state/{}", persistence.intake_ref),
                    format!(
                        ".handbook/state/{}",
                        persistence.lifecycle_validation_result_ref
                    ),
                    format!(".handbook/evidence/charter/{}", persistence.candidate_ref),
                ],
                refusal: None,
                next_actions: vec![
                    "review the immutable candidate and record every required human approval"
                        .to_owned(),
                ],
                projection: None,
            }
        }
        CharterCommandIntent::Approve {
            candidate_ref,
            approval_class,
            authority_ref,
            accepted_waiver_refs,
        } => {
            let identity = match RepositoryInvocationIdentityServiceV1::new()
                .prepare_operation(repo_root, RepositoryInvocationOperationV1::CharterApproval)
            {
                Ok(identity) => identity,
                Err(failure) => {
                    return repository_invocation_failure(CharterOperation::Approve, failure)
                }
            };
            let decisions = match resolve_shipped_profile_decisions(repo_root) {
                Ok(decisions) => decisions,
                Err(_) => return profile_refused(CharterOperation::Approve),
            };
            let mut service = CharterApprovalServiceV1::new(repo_root, port);
            match service.approve_candidate(
                &decisions,
                CharterApprovalRequestV1 {
                    operation_id: identity.operation_id().to_owned(),
                    candidate_ref,
                    approval_class,
                    authority_ref,
                    accepted_waiver_refs,
                },
            ) {
                CharterApprovalResultV1::Succeeded(success) => CharterOperationResult {
                    schema_id: CHARTER_OPERATION_RESULT_SCHEMA_ID.to_owned(),
                    schema_version: CHARTER_OPERATION_RESULT_SCHEMA_VERSION.to_owned(),
                    operation: CharterOperation::Approve,
                    status: AdapterOperationStatus::Succeeded,
                    canonical_path: None,
                    source_fingerprint: None,
                    rendered_output_fingerprint: None,
                    intake_ref: None,
                    intake_fingerprint: None,
                    candidate_ref: None,
                    candidate_fingerprint: None,
                    approval_ref: Some(success.approval_ref),
                    approval_fingerprint: Some(success.approval_fingerprint),
                    promotion_ref: None,
                    promotion_fingerprint: None,
                    changed_paths: success.changed_paths,
                    refusal: None,
                    next_actions: Vec::new(),
                    projection: None,
                },
                CharterApprovalResultV1::Refused(refusal) => CharterOperationResult {
                    schema_id: CHARTER_OPERATION_RESULT_SCHEMA_ID.to_owned(),
                    schema_version: CHARTER_OPERATION_RESULT_SCHEMA_VERSION.to_owned(),
                    operation: CharterOperation::Approve,
                    status: AdapterOperationStatus::Refused,
                    canonical_path: None,
                    source_fingerprint: None,
                    rendered_output_fingerprint: None,
                    intake_ref: None,
                    intake_fingerprint: None,
                    candidate_ref: None,
                    candidate_fingerprint: None,
                    approval_ref: None,
                    approval_fingerprint: None,
                    promotion_ref: None,
                    promotion_fingerprint: None,
                    changed_paths: refusal.changed_paths,
                    refusal: Some(AdapterRefusal {
                        code: refusal.code.as_str().to_owned(),
                        message: refusal.message,
                        retryable: refusal.retryable,
                    }),
                    next_actions: refusal.next_actions,
                    projection: None,
                },
            }
        }
        CharterCommandIntent::Promote {
            candidate_ref,
            approval_ref,
            expected_current_fingerprint,
        } => promotion_result(CharterPromotionWorkflowServiceV1::new(repo_root).promote(
            CharterPromotionIntentV1 {
                candidate_ref,
                approval_ref,
                expected_current_fingerprint,
            },
        )),
    }
}

fn promotion_result(
    result: Result<CharterPromotionWorkflowCommitV1, CharterPromotionWorkflowErrorV1>,
) -> CharterOperationResult {
    match result {
        Ok(commit) => match commit.product_projection() {
            Ok(projection) => CharterOperationResult {
                schema_id: CHARTER_OPERATION_RESULT_SCHEMA_ID.to_owned(),
                schema_version: CHARTER_OPERATION_RESULT_SCHEMA_VERSION.to_owned(),
                operation: CharterOperation::Promote,
                status: AdapterOperationStatus::Succeeded,
                canonical_path: Some(projection.canonical_path),
                source_fingerprint: Some(projection.canonical_fingerprint),
                rendered_output_fingerprint: None,
                intake_ref: None,
                intake_fingerprint: None,
                candidate_ref: None,
                candidate_fingerprint: None,
                approval_ref: None,
                approval_fingerprint: None,
                promotion_ref: Some(projection.promotion_ref),
                promotion_fingerprint: Some(projection.promotion_fingerprint),
                changed_paths: projection.changed_paths,
                refusal: None,
                next_actions: projection.next_actions,
                projection: None,
            },
            Err(error) => {
                let refusal = error.product_refusal();
                promotion_refused(
                    refusal.code,
                    refusal.message,
                    refusal.retryable,
                    refusal.next_actions,
                )
            }
        },
        Err(error) => {
            let refusal = error.product_refusal();
            promotion_refused(
                refusal.code,
                refusal.message,
                refusal.retryable,
                refusal.next_actions,
            )
        }
    }
}

fn promotion_refused(
    code: String,
    message: String,
    retryable: bool,
    next_actions: Vec<String>,
) -> CharterOperationResult {
    CharterOperationResult {
        schema_id: CHARTER_OPERATION_RESULT_SCHEMA_ID.to_owned(),
        schema_version: CHARTER_OPERATION_RESULT_SCHEMA_VERSION.to_owned(),
        operation: CharterOperation::Promote,
        status: AdapterOperationStatus::Refused,
        canonical_path: None,
        source_fingerprint: None,
        rendered_output_fingerprint: None,
        intake_ref: None,
        intake_fingerprint: None,
        candidate_ref: None,
        candidate_fingerprint: None,
        approval_ref: None,
        approval_fingerprint: None,
        promotion_ref: None,
        promotion_fingerprint: None,
        changed_paths: Vec::new(),
        refusal: Some(AdapterRefusal {
            code,
            message,
            retryable,
        }),
        next_actions,
        projection: None,
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct UnavailableNativeAuthenticatorPortV1;

impl NativeAuthenticatorPortV1 for UnavailableNativeAuthenticatorPortV1 {
    fn make_credential(
        &mut self,
        _request_cbor: &[u8],
    ) -> Result<Vec<u8>, NativeAuthenticatorPortErrorV1> {
        Err(NativeAuthenticatorPortErrorV1::Unavailable)
    }

    fn get_assertion(
        &mut self,
        _request_cbor: &[u8],
    ) -> Result<Vec<u8>, NativeAuthenticatorPortErrorV1> {
        Err(NativeAuthenticatorPortErrorV1::Unavailable)
    }
}

fn repository_invocation_failure(
    operation: CharterOperation,
    failure: RepositoryInvocationPreparationFailureV1,
) -> CharterOperationResult {
    let refusal = failure.refusal();
    CharterOperationResult {
        schema_id: failure.schema_id().to_owned(),
        schema_version: failure.schema_version().to_owned(),
        operation,
        status: AdapterOperationStatus::Refused,
        canonical_path: None,
        source_fingerprint: None,
        rendered_output_fingerprint: None,
        intake_ref: None,
        intake_fingerprint: None,
        candidate_ref: None,
        candidate_fingerprint: None,
        approval_ref: None,
        approval_fingerprint: None,
        promotion_ref: None,
        promotion_fingerprint: None,
        changed_paths: failure.changed_paths().to_vec(),
        refusal: Some(AdapterRefusal {
            code: refusal.code().to_owned(),
            message: refusal.message().to_owned(),
            retryable: refusal.retryable(),
        }),
        next_actions: failure.next_actions().to_vec(),
        projection: Some(failure),
    }
}

fn validate_selected_charter(repo_root: &Path) -> CharterOperationResult {
    let decisions = match resolve_shipped_profile_decisions(repo_root) {
        Ok(decisions) => decisions,
        Err(_) => return profile_refused(CharterOperation::Validate),
    };
    match load_selected_charter(repo_root, &decisions) {
        Ok(projection) => CharterOperationResult {
            schema_id: CHARTER_OPERATION_RESULT_SCHEMA_ID.to_owned(),
            schema_version: CHARTER_OPERATION_RESULT_SCHEMA_VERSION.to_owned(),
            operation: CharterOperation::Validate,
            status: AdapterOperationStatus::Succeeded,
            canonical_path: Some(projection.canonical_path().to_owned()),
            source_fingerprint: Some(projection.source_fingerprint().to_string()),
            rendered_output_fingerprint: Some(projection.rendered_output_fingerprint().to_string()),
            intake_ref: None,
            intake_fingerprint: None,
            candidate_ref: None,
            candidate_fingerprint: None,
            approval_ref: None,
            approval_fingerprint: None,
            promotion_ref: None,
            promotion_fingerprint: None,
            changed_paths: Vec::new(),
            refusal: None,
            next_actions: vec!["review the validated rendered Charter view".to_owned()],
            projection: None,
        },
        Err(error) => selected_charter_refused(CharterOperation::Validate, &error),
    }
}

fn selected_charter_refused(
    operation: CharterOperation,
    error: &handbook_engine::SelectedCharterLoadError,
) -> CharterOperationResult {
    let (code, action) = match error.status() {
        ArtifactInspectionStatus::Missing => (
            "canonical_charter_missing",
            "create a typed Charter candidate with an explicit acquisition mode",
        ),
        ArtifactInspectionStatus::StructurallyInvalid => (
            "canonical_charter_invalid",
            "repair selected canonical Charter truth before retrying",
        ),
        ArtifactInspectionStatus::UnsafePath => (
            "canonical_charter_unsafe",
            "repair the unsafe selected Charter path before retrying",
        ),
        ArtifactInspectionStatus::Unreadable => (
            "canonical_charter_unreadable",
            "repair selected Charter readability before retrying",
        ),
        ArtifactInspectionStatus::NotInspected | ArtifactInspectionStatus::StructurallyValid => (
            "canonical_charter_observation_refused",
            "rerun after obtaining one stable selected Charter observation",
        ),
    };
    refused_charter(
        operation,
        code,
        &format!(
            "selected Charter observation refused at {} ({:?})",
            error.canonical_path(),
            error.reason()
        ),
        false,
        action,
    )
}

fn profile_refused(operation: CharterOperation) -> CharterOperationResult {
    refused_charter(
        operation,
        "selected_profile_unavailable",
        "selected shipped profile decisions are unavailable",
        false,
        "repair selected profile definitions and retry",
    )
}

pub fn legacy_charter_input_refusal() -> CharterOperationResult {
    refused_charter(
        CharterOperation::Author,
        "legacy_input_refused",
        "--from-inputs without an explicit --mode is not a selected Charter operation",
        false,
        "retry with --mode guided-adaptive, express, or agent-assisted",
    )
}

pub fn invalid_charter_command_refusal(operation: CharterOperation) -> CharterOperationResult {
    refused_charter(
        operation,
        "invalid_request",
        "arguments do not select exactly one frozen Charter operation",
        false,
        "choose exactly one author, approve, promote, or validate command form",
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use handbook_engine::{CharterPromotionCommitV1, RepositoryInvocationIdentityServiceV1};
    use std::cell::RefCell;
    use std::path::PathBuf;
    use std::rc::Rc;

    #[derive(Clone, Default)]
    struct CapturingUnavailablePort {
        make_calls: Rc<RefCell<Vec<Vec<u8>>>>,
        get_calls: Rc<RefCell<Vec<Vec<u8>>>>,
    }

    impl NativeAuthenticatorPortV1 for CapturingUnavailablePort {
        fn make_credential(
            &mut self,
            request_cbor: &[u8],
        ) -> Result<Vec<u8>, NativeAuthenticatorPortErrorV1> {
            self.make_calls.borrow_mut().push(request_cbor.to_vec());
            Err(NativeAuthenticatorPortErrorV1::Unavailable)
        }

        fn get_assertion(
            &mut self,
            request_cbor: &[u8],
        ) -> Result<Vec<u8>, NativeAuthenticatorPortErrorV1> {
            self.get_calls.borrow_mut().push(request_cbor.to_vec());
            Err(NativeAuthenticatorPortErrorV1::Unavailable)
        }
    }

    fn promotion_commit(
        promotion_ref: &str,
        lifecycle_transition_ref: &str,
    ) -> CharterPromotionWorkflowCommitV1 {
        CharterPromotionWorkflowCommitV1 {
            canonical_fingerprint: format!("sha256:{}", "a".repeat(64)),
            promotion_ref: promotion_ref.to_owned(),
            lifecycle_transition_ref: lifecycle_transition_ref.to_owned(),
            approval_refs: Vec::new(),
            required_approval_pairs: Vec::new(),
            transaction: CharterPromotionCommitV1 {
                promotion_ref: promotion_ref.to_owned(),
                lifecycle_transition_ref: lifecycle_transition_ref.to_owned(),
                committed_marker_path: PathBuf::from(
                    ".handbook/state/transactions/promotions/private.committed/committed",
                ),
            },
        }
    }

    #[test]
    fn injected_native_port_reaches_engine_with_engine_allocated_admin_identity() {
        let repo = tempfile::tempdir().expect("repository");
        std::fs::create_dir_all(repo.path().join(".handbook")).expect("managed root");
        let initialized = RepositoryInvocationIdentityServiceV1::new()
            .initialize_for_setup(repo.path())
            .expect("repository identity");
        let port = CapturingUnavailablePort::default();
        let calls = port.make_calls.clone();

        let result = execute_approver_admin_intent_with_port(
            repo.path(),
            ApproverAdminIntent::Bootstrap {
                initial_charter_quorum: vec![
                    "charter_approval=repository_charter_approver".to_owned()
                ],
            },
            port,
        );
        let value = serde_json::to_value(result).expect("closed engine result");

        assert_eq!(calls.borrow().len(), 1);
        assert!(!calls.borrow()[0].is_empty());
        assert_eq!(value["schema_id"], "handbook.approver-admin-result");
        assert_eq!(value["operation"], "bootstrap");
        assert_eq!(
            value["repository_identity_fingerprint"],
            initialized.repository_identity_fingerprint()
        );
        assert!(value["operation_id"]
            .as_str()
            .expect("engine operation ID")
            .starts_with("bootstrap-"));
        assert_eq!(value["refusal"]["code"], "AUTHENTICATOR_UNAVAILABLE");
        assert!(value["changed_paths"]
            .as_array()
            .expect("changed paths")
            .is_empty());
    }

    #[test]
    fn committed_promotion_fixture_maps_only_exact_engine_product_projection() {
        let promotion_ref = format!("promotions/promotion_{}.json", "b".repeat(64));
        let lifecycle_ref = format!(
            "lifecycle-transitions/lifecycle-transition_{}.json",
            "c".repeat(64)
        );
        let result = promotion_result(Ok(promotion_commit(&promotion_ref, &lifecycle_ref)));

        assert_eq!(result.status, AdapterOperationStatus::Succeeded);
        assert_eq!(
            result.canonical_path.as_deref(),
            Some(".handbook/project/charter.yaml")
        );
        assert_eq!(
            result.source_fingerprint.as_deref(),
            Some(format!("sha256:{}", "a".repeat(64)).as_str())
        );
        assert_eq!(
            result.promotion_ref.as_deref(),
            Some(promotion_ref.as_str())
        );
        assert_eq!(
            result.promotion_fingerprint.as_deref(),
            Some(format!("sha256:{}", "b".repeat(64)).as_str())
        );
        assert_eq!(
            result.changed_paths,
            vec![
                ".handbook/project/charter.yaml".to_owned(),
                format!(".handbook/state/{promotion_ref}"),
                format!(".handbook/state/{lifecycle_ref}"),
            ]
        );
        assert!(result.refusal.is_none());
        assert!(!format!("{result:?}").contains("private.committed"));
    }

    #[test]
    fn malformed_commit_uses_engine_owned_projection_refusal() {
        let lifecycle_ref = format!(
            "lifecycle-transitions/lifecycle-transition_{}.json",
            "c".repeat(64)
        );
        let result = promotion_result(Ok(promotion_commit(
            "promotions/not-content-addressed.json",
            &lifecycle_ref,
        )));

        assert_eq!(result.status, AdapterOperationStatus::Refused);
        assert_eq!(
            result.refusal.as_ref().map(|refusal| refusal.code.as_str()),
            Some("promotion_projection_invalid_promotion_ref")
        );
        assert_eq!(
            result.next_actions,
            vec![
                "repair the retained content-addressed promotion ref before retrying product projection"
            ]
        );
    }
}

fn refused_charter(
    operation: CharterOperation,
    code: &str,
    message: &str,
    retryable: bool,
    next_action: &str,
) -> CharterOperationResult {
    CharterOperationResult {
        schema_id: CHARTER_OPERATION_RESULT_SCHEMA_ID.to_owned(),
        schema_version: CHARTER_OPERATION_RESULT_SCHEMA_VERSION.to_owned(),
        operation,
        status: AdapterOperationStatus::Refused,
        canonical_path: None,
        source_fingerprint: None,
        rendered_output_fingerprint: None,
        intake_ref: None,
        intake_fingerprint: None,
        candidate_ref: None,
        candidate_fingerprint: None,
        approval_ref: None,
        approval_fingerprint: None,
        promotion_ref: None,
        promotion_fingerprint: None,
        changed_paths: Vec::new(),
        refusal: Some(AdapterRefusal {
            code: code.to_owned(),
            message: message.to_owned(),
            retryable,
        }),
        next_actions: vec![next_action.to_owned()],
        projection: None,
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ApproverAdminOperation {
    Bootstrap,
    AddCredential,
    RevokeCredential,
    UpdateMapping,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ApproverAdminIntent {
    Bootstrap {
        initial_charter_quorum: Vec<String>,
    },
    AddCredential {
        approval_mappings: Vec<String>,
    },
    RevokeCredential {
        credential_id_hash: String,
    },
    UpdateMapping {
        credential_id_hash: String,
        approval_mappings: Vec<String>,
    },
}

#[allow(clippy::large_enum_variant)]
#[derive(Clone, Debug, PartialEq)]
enum ApproverAdminProjection {
    Engine(ApproverAdminResultV1),
    InvocationFailure(RepositoryInvocationPreparationFailureV1),
}

#[derive(Clone, Debug, PartialEq)]
pub struct ApproverAdapterResult {
    pub schema_id: String,
    pub schema_version: String,
    pub operation: ApproverAdminOperation,
    pub status: AdapterOperationStatus,
    pub changed_paths: Vec<String>,
    pub refusal: Option<AdapterRefusal>,
    pub next_actions: Vec<String>,
    projection: Option<ApproverAdminProjection>,
}

impl Serialize for ApproverAdapterResult {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match &self.projection {
            Some(ApproverAdminProjection::Engine(result)) => result.serialize(serializer),
            Some(ApproverAdminProjection::InvocationFailure(failure)) => {
                failure.serialize(serializer)
            }
            None => {
                #[derive(Serialize)]
                struct Wire<'a> {
                    schema_id: &'a str,
                    schema_version: &'a str,
                    operation: ApproverAdminOperation,
                    status: AdapterOperationStatus,
                    changed_paths: &'a [String],
                    refusal: &'a Option<AdapterRefusal>,
                    next_actions: &'a [String],
                }
                Wire {
                    schema_id: &self.schema_id,
                    schema_version: &self.schema_version,
                    operation: self.operation,
                    status: self.status,
                    changed_paths: &self.changed_paths,
                    refusal: &self.refusal,
                    next_actions: &self.next_actions,
                }
                .serialize(serializer)
            }
        }
    }
}

pub fn execute_approver_admin_intent(
    repo_root: impl AsRef<Path>,
    intent: ApproverAdminIntent,
) -> ApproverAdapterResult {
    execute_approver_admin_intent_with_port(
        repo_root.as_ref(),
        intent,
        UnavailableNativeAuthenticatorPortV1,
    )
}

fn execute_approver_admin_intent_with_port<P: NativeAuthenticatorPortV1>(
    repo_root: &Path,
    intent: ApproverAdminIntent,
    port: P,
) -> ApproverAdapterResult {
    let (operation, identity_operation, request_operation, mut document) = match intent {
        ApproverAdminIntent::Bootstrap {
            initial_charter_quorum,
        } => match parse_mapping_strings(initial_charter_quorum) {
            Ok(initial_charter_quorum) => (
                ApproverAdminOperation::Bootstrap,
                RepositoryInvocationOperationV1::ApproverBootstrap,
                "bootstrap",
                json!({
                    "expected_registry_state_fingerprint": null,
                    "expected_transition_fingerprint": null,
                    "initial_charter_quorum": initial_charter_quorum,
                }),
            ),
            Err(message) => {
                return approver_adapter_refused(
                    ApproverAdminOperation::Bootstrap,
                    "invalid_request",
                    &message,
                    false,
                    "use one or more exact non-empty class=authority quorum pairs",
                )
            }
        },
        ApproverAdminIntent::AddCredential { approval_mappings } => {
            match parse_mapping_strings(approval_mappings) {
                Ok(approval_mappings) => (
                    ApproverAdminOperation::AddCredential,
                    RepositoryInvocationOperationV1::ApproverAddCredential,
                    "add_credential",
                    json!({"approval_mappings": approval_mappings}),
                ),
                Err(message) => {
                    return approver_adapter_refused(
                        ApproverAdminOperation::AddCredential,
                        "invalid_request",
                        &message,
                        false,
                        "use one or more exact non-empty class=authority approval mappings",
                    )
                }
            }
        }
        ApproverAdminIntent::RevokeCredential { credential_id_hash } => (
            ApproverAdminOperation::RevokeCredential,
            RepositoryInvocationOperationV1::ApproverRevokeCredential,
            "revoke_credential",
            json!({"credential_id_hash": credential_id_hash}),
        ),
        ApproverAdminIntent::UpdateMapping {
            credential_id_hash,
            approval_mappings,
        } => match parse_mapping_strings(approval_mappings) {
            Ok(approval_mappings) => (
                ApproverAdminOperation::UpdateMapping,
                RepositoryInvocationOperationV1::ApproverUpdateMapping,
                "update_mapping",
                json!({
                    "credential_id_hash": credential_id_hash,
                    "approval_mappings": approval_mappings,
                }),
            ),
            Err(message) => {
                return approver_adapter_refused(
                    ApproverAdminOperation::UpdateMapping,
                    "invalid_request",
                    &message,
                    false,
                    "use one or more exact non-empty class=authority approval mappings",
                )
            }
        },
    };

    let identity = match RepositoryInvocationIdentityServiceV1::new()
        .prepare_operation(repo_root, identity_operation)
    {
        Ok(identity) => identity,
        Err(failure) => return approver_invocation_failure(operation, failure),
    };
    let object = document
        .as_object_mut()
        .expect("compiler-built approver request is an object");
    object.insert(
        "schema_id".to_owned(),
        Value::String("handbook.approver-admin-request".to_owned()),
    );
    object.insert("schema_version".to_owned(), Value::String("1.0".to_owned()));
    object.insert(
        "operation".to_owned(),
        Value::String(request_operation.to_owned()),
    );
    object.insert(
        "operation_id".to_owned(),
        Value::String(identity.operation_id().to_owned()),
    );
    object.insert(
        "repository_identity_fingerprint".to_owned(),
        Value::String(identity.repository_identity_fingerprint().to_owned()),
    );
    if operation != ApproverAdminOperation::Bootstrap {
        let observation = match handbook_engine::observe_committed_approver_registry(repo_root) {
            Ok(observation) => observation,
            Err(error) => {
                let (code, action) = match error.kind() {
                    ApproverRegistryObservationErrorKindV1::AuthorityAbsent => (
                        "authority_absent",
                        "bootstrap the repository approver registry before retrying",
                    ),
                    ApproverRegistryObservationErrorKindV1::DurabilityViolation
                    | ApproverRegistryObservationErrorKindV1::UnsafeFilesystem
                    | ApproverRegistryObservationErrorKindV1::LineageViolation => (
                        "authority_recovery_blocked",
                        "repair retained registry authority evidence before retrying",
                    ),
                };
                return approver_adapter_refused(operation, code, error.detail(), false, action);
            }
        };
        object.insert(
            "expected_registry_state_fingerprint".to_owned(),
            Value::String(observation.state_fingerprint.clone()),
        );
        object.insert(
            "expected_transition_fingerprint".to_owned(),
            Value::String(observation.head_transition_fingerprint.clone()),
        );
    }
    let request = match ApproverAdminRequestV1::from_json_value(document) {
        Ok(request) => request,
        Err(error) => {
            return approver_adapter_refused(
                operation,
                "invalid_request",
                &error.to_string(),
                false,
                "repair the closed approver request intent before retrying",
            )
        }
    };
    let mut service = ApproverRegistryServiceV1::for_repository(repo_root, port);
    let result = match operation {
        ApproverAdminOperation::Bootstrap => service.bootstrap_approver_registry(request),
        ApproverAdminOperation::AddCredential => service.add_approver_credential(request),
        ApproverAdminOperation::RevokeCredential => service.revoke_approver_credential(request),
        ApproverAdminOperation::UpdateMapping => service.update_approver_mapping(request),
    };
    approver_engine_result(operation, result)
}

fn parse_mapping_strings(values: Vec<String>) -> Result<Vec<Value>, String> {
    let mut mappings = values
        .into_iter()
        .map(|value| {
            let (approval_class, authority_ref) = value.split_once('=').ok_or_else(|| {
                "approver mapping must use exact class=authority syntax".to_owned()
            })?;
            if approval_class.is_empty()
                || authority_ref.is_empty()
                || approval_class.contains('\0')
                || authority_ref.contains('\0')
            {
                return Err(
                    "approver mapping class and authority must be non-empty NUL-free strings"
                        .to_owned(),
                );
            }
            Ok(json!({
                "approval_class": approval_class,
                "authority_ref": authority_ref,
            }))
        })
        .collect::<Result<Vec<_>, String>>()?;
    mappings.sort_by(|left, right| {
        left["approval_class"]
            .as_str()
            .cmp(&right["approval_class"].as_str())
            .then_with(|| {
                left["authority_ref"]
                    .as_str()
                    .cmp(&right["authority_ref"].as_str())
            })
    });
    if mappings.windows(2).any(|pair| pair[0] == pair[1]) {
        return Err("approver mappings must be unique".to_owned());
    }
    Ok(mappings)
}

fn approver_engine_result(
    operation: ApproverAdminOperation,
    result: ApproverAdminResultV1,
) -> ApproverAdapterResult {
    let value = result
        .to_json_value()
        .expect("engine-built approver result must satisfy its closed schema");
    let status = match value["status"].as_str() {
        Some("succeeded") => AdapterOperationStatus::Succeeded,
        Some("refused") => AdapterOperationStatus::Refused,
        _ => unreachable!("closed engine approver status"),
    };
    ApproverAdapterResult {
        schema_id: value["schema_id"].as_str().unwrap().to_owned(),
        schema_version: value["schema_version"].as_str().unwrap().to_owned(),
        operation,
        status,
        changed_paths: string_array_from_value(&value["changed_paths"]),
        refusal: value["refusal"].as_object().map(|refusal| AdapterRefusal {
            code: refusal["code"].as_str().unwrap().to_owned(),
            message: refusal["message"].as_str().unwrap().to_owned(),
            retryable: refusal["retryable"].as_bool().unwrap(),
        }),
        next_actions: string_array_from_value(&value["next_actions"]),
        projection: Some(ApproverAdminProjection::Engine(result)),
    }
}

fn approver_invocation_failure(
    operation: ApproverAdminOperation,
    failure: RepositoryInvocationPreparationFailureV1,
) -> ApproverAdapterResult {
    let refusal = failure.refusal();
    ApproverAdapterResult {
        schema_id: failure.schema_id().to_owned(),
        schema_version: failure.schema_version().to_owned(),
        operation,
        status: AdapterOperationStatus::Refused,
        changed_paths: failure.changed_paths().to_vec(),
        refusal: Some(AdapterRefusal {
            code: refusal.code().to_owned(),
            message: refusal.message().to_owned(),
            retryable: refusal.retryable(),
        }),
        next_actions: failure.next_actions().to_vec(),
        projection: Some(ApproverAdminProjection::InvocationFailure(failure)),
    }
}

fn string_array_from_value(value: &Value) -> Vec<String> {
    value
        .as_array()
        .expect("engine string array")
        .iter()
        .map(|value| value.as_str().expect("engine string").to_owned())
        .collect()
}

fn approver_adapter_refused(
    operation: ApproverAdminOperation,
    code: &str,
    message: &str,
    retryable: bool,
    action: &str,
) -> ApproverAdapterResult {
    ApproverAdapterResult {
        schema_id: APPROVER_ADAPTER_RESULT_SCHEMA_ID.to_owned(),
        schema_version: APPROVER_ADAPTER_RESULT_SCHEMA_VERSION.to_owned(),
        operation,
        status: AdapterOperationStatus::Refused,
        changed_paths: Vec::new(),
        refusal: Some(AdapterRefusal {
            code: code.to_owned(),
            message: message.to_owned(),
            retryable,
        }),
        next_actions: vec![action.to_owned()],
        projection: None,
    }
}
