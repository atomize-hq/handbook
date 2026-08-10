use handbook_sdk::{
    AuthorOperationStatus, CharterAcquisitionMode, CharterCommandRequest,
    CharterCoverageSubmission, CharterInputValue, CharterIntakeConsumer, CharterIntakeEnvelope,
    CharterIntakeSourceKind, DoctorCharterDefinitionClosureStatus, DoctorCharterLifecycleState,
    DoctorCharterNextAction, HandbookSdkV1, SetupMode, SetupRequest,
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
fn author_persists_the_exact_immutable_bundle_without_selecting_canonical_truth() {
    let repo = tempfile::tempdir().expect("repository");
    let result = HandbookSdkV1::open(repo.path()).author_charter(CharterCommandRequest::Author {
        mode: CharterAcquisitionMode::GuidedAdaptive,
        intake: valid_intake(),
    });

    assert_eq!(result.status, AuthorOperationStatus::Succeeded);
    let candidate_ref = result.candidate_ref.as_deref().expect("candidate ref");
    let candidate: serde_json::Value = serde_json::from_slice(
        &std::fs::read(
            repo.path()
                .join(".handbook/evidence/charter")
                .join(candidate_ref),
        )
        .expect("candidate bundle"),
    )
    .expect("candidate JSON");
    assert_eq!(candidate["schema_version"], "1.3");
    assert_eq!(
        candidate["candidate_fingerprint"].as_str(),
        result.candidate_fingerprint.as_deref()
    );
    assert_eq!(result.changed_paths.len(), 4);
    assert!(result
        .changed_paths
        .iter()
        .all(|path| repo.path().join(path).is_file()));
    assert!(!repo.path().join(".handbook/project/charter.yaml").exists());
}

#[test]
fn approval_projects_a_closed_repository_identity_preflight() {
    let repo = tempfile::tempdir().expect("repository");
    let result = HandbookSdkV1::open(repo.path()).author_charter(CharterCommandRequest::Approve {
        candidate_ref: "candidates/candidate_aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa.json".to_owned(),
        approval_class: "Project owner approval".to_owned(),
        authority_ref: "Project owner".to_owned(),
        accepted_waiver_refs: Vec::new(),
    });

    assert_eq!(result.status, AuthorOperationStatus::Refused);
    let failure = result.invocation_failure.expect("preflight projection");
    assert_eq!(
        failure.schema_id,
        "handbook.repository-invocation-preflight-result"
    );
    assert_eq!(failure.schema_version, "1.0");
    assert_eq!(failure.operation, "charter_approval");
    assert_eq!(failure.stage.as_deref(), Some("repository_identity"));
    assert_eq!(failure.status, "refused");
    assert_eq!(failure.repository_identity_fingerprint, None);
    assert_eq!(failure.operation_id, None);
    assert!(failure.changed_paths.is_empty());
    assert_eq!(failure.refusal.code, "repository_identity_unavailable");
    assert_eq!(
        failure.next_actions,
        vec!["run or repair handbook setup, then retry the complete operation"]
    );
}

#[test]
fn promotion_projects_the_engine_owned_invalid_intent_refusal() {
    let repo = tempfile::tempdir().expect("repository");
    let result = HandbookSdkV1::open(repo.path()).author_charter(CharterCommandRequest::Promote {
        candidate_ref: String::new(),
        approval_ref: String::new(),
        expected_current_fingerprint: None,
    });

    assert_eq!(result.status, AuthorOperationStatus::Refused);
    assert!(result.changed_paths.is_empty());
    assert_eq!(
        result.refusal.as_ref().map(|refusal| (
            refusal.code.as_str(),
            refusal.message.as_str(),
            refusal.retryable,
        )),
        Some((
            "invalid_intent",
            "promotion candidate or approval anchor ref is invalid",
            false,
        ))
    );
    assert_eq!(
        result.next_actions,
        vec!["correct the bounded promotion intent and retry"]
    );
    assert!(!repo.path().join(".handbook").exists());
}

#[test]
fn setup_creates_only_the_root_and_never_authors_selected_charter_truth() {
    let repo = tempfile::tempdir().expect("repository");
    let outcome = HandbookSdkV1::open(repo.path())
        .run_setup(&SetupRequest {
            mode: SetupMode::Init,
            rewrite: false,
            reset_state: false,
        })
        .expect("setup outcome");

    assert!(!repo.path().join(".handbook/project/charter.yaml").exists());
    assert!(!repo.path().join(".handbook/charter/CHARTER.md").exists());
    let charter = outcome
        .plan
        .artifacts
        .iter()
        .find(|row| row.artifact.instance_id == "project_authority")
        .expect("selected Charter action");
    assert_eq!(
        charter.action,
        handbook_sdk::SetupArtifactActionKind::AuthorRequired
    );
}

#[test]
fn doctor_adds_a_typed_missing_charter_observation_without_changing_readiness_rows() {
    let repo = tempfile::tempdir().expect("repository");
    let report = HandbookSdkV1::open(repo.path())
        .doctor()
        .expect("doctor report");
    let charter = report.charter.expect("additive Charter row");

    assert!(
        !repo.path().join(".handbook").exists(),
        "doctor must not create the managed root"
    );
    assert_eq!(charter.instance_id, "project_authority");
    assert_eq!(
        charter.definition_closure_status,
        DoctorCharterDefinitionClosureStatus::Resolved
    );
    assert_eq!(
        charter.canonical_status,
        handbook_sdk::ArtifactInspectionStatus::Missing
    );
    assert_eq!(
        charter.canonical_reason,
        handbook_sdk::ArtifactInspectionReason::RequiredPathMissing
    );
    assert_eq!(
        charter.lifecycle_state,
        DoctorCharterLifecycleState::Unobserved
    );
    assert_eq!(
        charter.next_actions,
        vec![DoctorCharterNextAction::RunCharterAuthor]
    );
}
