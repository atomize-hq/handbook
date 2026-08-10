use handbook_sdk::{
    AuthorOperationStatus, CharterAcquisitionMode, CharterCommandRequest,
    CharterCoverageSubmission, CharterInputValue, CharterIntakeConsumer, CharterIntakeEnvelope,
    CharterIntakeSourceKind, HandbookSdkV1, ProjectContextInput,
};

const CHARTER_YAML: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../docs/specs/handbook-contract-membrane/slices/HCM-2.2/contracts/canonical-charter-boundary-v1.1.yaml"
));

const COVERAGE_IDS: [&str; 16] = [
    "project_shape.definition",
    "delivery.constraints",
    "delivery.default_implications",
    "operational_reality.production_state",
    "risk.domains",
    "engineering_posture.baseline",
    "policy.authority_and_revision",
    "governance.decision_authority",
    "governance.required_approvals",
    "governance.exception_policy",
    "engineering_posture.dimensions",
    "engineering_posture.red_lines",
    "governance.review_triggers",
    "governance.reassessment_triggers",
    "debt.register",
    "decisions.records",
];

fn charter_value(value: serde_json::Value) -> CharterInputValue {
    match value {
        serde_json::Value::Null => CharterInputValue::Null,
        serde_json::Value::Bool(value) => CharterInputValue::Boolean(value),
        serde_json::Value::Number(value) => CharterInputValue::Number(value.to_string()),
        serde_json::Value::String(value) => CharterInputValue::String(value),
        serde_json::Value::Array(values) => {
            CharterInputValue::Sequence(values.into_iter().map(charter_value).collect())
        }
        serde_json::Value::Object(values) => CharterInputValue::Mapping(
            values
                .into_iter()
                .map(|(key, value)| (key, charter_value(value)))
                .collect(),
        ),
    }
}

fn valid_intake() -> CharterIntakeEnvelope {
    let content = serde_yaml_bw::from_str(CHARTER_YAML).expect("canonical Charter YAML");
    CharterIntakeEnvelope {
        mode: CharterAcquisitionMode::GuidedAdaptive,
        content: charter_value(content),
        coverage: COVERAGE_IDS
            .into_iter()
            .map(|coverage_id| CharterCoverageSubmission {
                coverage_id: coverage_id.to_owned(),
                source_kind: CharterIntakeSourceKind::UserDeclaration,
                value_ref: format!("input://{coverage_id}"),
                evidence_refs: Vec::new(),
                confidence: "high".to_owned(),
                freshness: None,
                sensitivity: "public".to_owned(),
                contradiction_refs: Vec::new(),
                waiver_ref: None,
            })
            .collect(),
        consumer: CharterIntakeConsumer {
            kind: "agent".to_owned(),
            id: "handbook-charter-intake".to_owned(),
            version: "1.0".to_owned(),
        },
        prompt_event_refs: Vec::new(),
        finalized_at_utc: "2026-07-20T00:00:00Z".to_owned(),
        expected_current_fingerprint: None,
    }
}

#[test]
fn closed_charter_author_request_persists_the_immutable_candidate_bundle() {
    let repo = tempfile::tempdir().expect("repository");
    let result = HandbookSdkV1::open(repo.path()).author_charter(CharterCommandRequest::Author {
        mode: CharterAcquisitionMode::GuidedAdaptive,
        intake: valid_intake(),
    });

    assert_eq!(result.status, AuthorOperationStatus::Succeeded);
    assert!(result
        .intake_ref
        .as_deref()
        .is_some_and(|value| value.starts_with("intake-records/")));
    assert!(result
        .candidate_ref
        .as_deref()
        .is_some_and(|value| value.starts_with("candidates/")));
    assert_eq!(result.changed_paths.len(), 4);
    assert!(result
        .changed_paths
        .iter()
        .all(|path| repo.path().join(path).is_file()));
    assert!(!repo.path().join(".handbook/project/charter.yaml").exists());
}

#[test]
fn closed_author_request_rejects_a_mode_mismatch_without_mutation() {
    let repo = tempfile::tempdir().expect("repository");
    let result = HandbookSdkV1::open(repo.path()).author_charter(CharterCommandRequest::Author {
        mode: CharterAcquisitionMode::Express,
        intake: valid_intake(),
    });

    assert_eq!(result.status, AuthorOperationStatus::Refused);
    assert_eq!(
        result.refusal.as_ref().map(|refusal| refusal.code.as_str()),
        Some("acquisition_mode_mismatch")
    );
    assert!(result.changed_paths.is_empty());
    assert!(!repo.path().join(".handbook").exists());
}

#[test]
fn closed_approval_request_preserves_repository_preflight_projection() {
    let repo = tempfile::tempdir().expect("repository");
    let result = HandbookSdkV1::open(repo.path()).author_charter(CharterCommandRequest::Approve {
        candidate_ref: "candidates/candidate_aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa.json".to_owned(),
        approval_class: "Project owner approval".to_owned(),
        authority_ref: "Project owner".to_owned(),
        accepted_waiver_refs: Vec::new(),
    });

    assert_eq!(result.status, AuthorOperationStatus::Refused);
    let failure = result
        .invocation_failure
        .expect("repository preflight projection");
    assert_eq!(
        failure.schema_id,
        "handbook.repository-invocation-preflight-result"
    );
    assert_eq!(failure.operation, "charter_approval");
    assert_eq!(failure.stage.as_deref(), Some("repository_identity"));
    assert_eq!(failure.refusal.code, "repository_identity_unavailable");
    assert!(failure.changed_paths.is_empty());
}

#[test]
fn typed_project_context_validation_refuses_invalid_selected_contract_values() {
    let repo = tempfile::tempdir().expect("repository");
    let input = ProjectContextInput {
        schema_id: "handbook.artifact.project-context".to_owned(),
        schema_version: "1.0".to_owned(),
        record_id: "handbook.project-context".to_owned(),
        summary: String::new(),
        system_boundaries: Vec::new(),
        ownership: Vec::new(),
        authoritative_references: Vec::new(),
        known_unknowns: Vec::new(),
    };

    let refusal = HandbookSdkV1::open(repo.path())
        .validate_project_context(&input)
        .expect_err("empty summary must not satisfy the selected Project Context contract");
    assert_eq!(
        refusal.kind,
        handbook_sdk::AuthorProjectContextRefusalKind::IncompleteStructuredInput
    );
}
