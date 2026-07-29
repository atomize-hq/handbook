use handbook_engine::{
    evaluate_charter_intake, load_shipped_charter_definition_registry, parse_definition_yaml,
    resolve_profile_selection, resolve_shipped_profile_decisions, ApproverAdminRequestV1,
    ApproverRegistryServiceV1, CharterAcquisitionMode, CharterApprovalRequestV1,
    CharterApprovalResultV1, CharterApprovalServiceV1, CharterAuthorPersistenceServiceV1,
    CharterAuthorityTransactionServiceV1, CharterCoverageSubmission, CharterIntakeConsumer,
    CharterIntakeEnvelope, CharterIntakeErrorKind, CharterIntakeSourceKind,
    CharterPromotionIntentV1, CharterPromotionWorkflowServiceV1, DefinitionSource,
    DefinitionSourceBinding, ExactDefinitionRef, NativeAuthenticatorPortErrorV1,
    NativeAuthenticatorPortV1, ProfileSelectionRequest, ResolvedProfileDecisions,
    AUTHENTICATOR_RP_ID,
};
use p256::ecdsa::{signature::Signer, Signature, SigningKey};
use serde_json::Value;
use sha2::{Digest, Sha256};
use std::fs;
use std::path::Path;

const RELEASED_PROFILE_REF: &str = "handbook.profile.shipped-root@1.1.0";
const RELEASED_PROFILE_FINGERPRINT: &str =
    "sha256:6a7b41befa77b999b9ee20f513636051726a8401a81bf2f369501e8f3dd4fa74";
const SUCCESSOR_PROFILE_REF: &str = "handbook.profile.shipped-root@1.2.0";
const SUCCESSOR_PROFILE_FINGERPRINT: &str =
    "sha256:40c5fdb8a6ea42cf0f5f2c5cac8306ec7ad3a238c341653947f85abc93d72c40";
const BOUNDARY_YAML: &[u8] = include_bytes!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../docs/specs/handbook-contract-membrane/slices/HCM-2.2/contracts/canonical-charter-boundary-v1.1.yaml"
));
const REGISTRY_SOURCE: &str = include_str!("../src/charter_definition_registry.rs");
const PROMOTION_SOURCE: &str = include_str!("../src/charter_promotion_workflow.rs");
const TRANSACTION_SOURCE: &str = include_str!("../src/charter_authority_transaction.rs");
const INCOMPATIBLE_PROFILE_YAML: &[u8] =
    include_bytes!("fixtures/hcm_1_2_repository_profile/root.yaml");
const ADMIN_VECTORS: &[u8] = include_bytes!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../docs/specs/handbook-contract-membrane/slices/HCM-2.2/contracts/approver-admin-api-vectors-v1.0.json"
));

const OBSERVATIONAL: [&str; 5] = [
    "project_shape.definition",
    "delivery.constraints",
    "operational_reality.production_state",
    "risk.domains",
    "debt.register",
];

const NORMATIVE: [&str; 11] = [
    "delivery.default_implications",
    "engineering_posture.baseline",
    "policy.authority_and_revision",
    "governance.decision_authority",
    "governance.required_approvals",
    "governance.exception_policy",
    "engineering_posture.dimensions",
    "engineering_posture.red_lines",
    "governance.review_triggers",
    "governance.reassessment_triggers",
    "decisions.records",
];

struct UnavailableApprovalPort;

impl NativeAuthenticatorPortV1 for UnavailableApprovalPort {
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

struct DynamicApprovalPort {
    key: SigningKey,
    credential_id: Vec<u8>,
    get_calls: usize,
}

impl DynamicApprovalPort {
    fn new() -> Self {
        Self {
            key: SigningKey::from_slice(&[1_u8; 32]).unwrap(),
            credential_id: vec![0x11; 32],
            get_calls: 0,
        }
    }
}

impl NativeAuthenticatorPortV1 for DynamicApprovalPort {
    fn make_credential(
        &mut self,
        _request_cbor: &[u8],
    ) -> Result<Vec<u8>, NativeAuthenticatorPortErrorV1> {
        Ok(make_credential_response(&self.key, &self.credential_id))
    }

    fn get_assertion(
        &mut self,
        request_cbor: &[u8],
    ) -> Result<Vec<u8>, NativeAuthenticatorPortErrorV1> {
        let marker = request_cbor
            .windows(3)
            .position(|window| window == [0x02, 0x58, 0x20])
            .expect("GetAssertion client-data hash marker");
        let client_data_hash: [u8; 32] = request_cbor[marker + 3..marker + 35].try_into().unwrap();
        self.get_calls += 1;
        Ok(assertion_response(
            &self.key,
            &self.credential_id,
            client_data_hash,
            self.get_calls as u32,
        ))
    }
}

fn bootstrap_request() -> ApproverAdminRequestV1 {
    let fixture: Value = serde_json::from_slice(ADMIN_VECTORS).unwrap();
    let document = fixture["vectors"]
        .as_array()
        .unwrap()
        .iter()
        .find(|row| row["vector_id"] == "bootstrap-request")
        .unwrap()["document"]
        .clone();
    ApproverAdminRequestV1::from_json_value(document).unwrap()
}

fn make_credential_response(key: &SigningKey, credential_id: &[u8]) -> Vec<u8> {
    let point = key.verifying_key().to_encoded_point(false);
    let mut cose = vec![0xa5, 0x01, 0x02, 0x03, 0x26, 0x20, 0x01, 0x21, 0x58, 0x20];
    cose.extend_from_slice(point.x().unwrap());
    cose.extend_from_slice(&[0x22, 0x58, 0x20]);
    cose.extend_from_slice(point.y().unwrap());
    let mut auth_data = Sha256::digest(AUTHENTICATOR_RP_ID.as_bytes()).to_vec();
    auth_data.push(0x45);
    auth_data.extend_from_slice(&0_u32.to_be_bytes());
    auth_data.extend_from_slice(&[0_u8; 16]);
    auth_data.extend_from_slice(&(credential_id.len() as u16).to_be_bytes());
    auth_data.extend_from_slice(credential_id);
    auth_data.extend_from_slice(&cose);
    let mut response = vec![0, 0xa3];
    cbor_text(&mut response, "fmt");
    cbor_text(&mut response, "none");
    cbor_text(&mut response, "attStmt");
    response.push(0xa0);
    cbor_text(&mut response, "authData");
    cbor_bytes(&mut response, &auth_data);
    response
}

fn assertion_response(
    key: &SigningKey,
    credential_id: &[u8],
    client_data_hash: [u8; 32],
    sign_count: u32,
) -> Vec<u8> {
    let mut authenticator_data = Sha256::digest(AUTHENTICATOR_RP_ID.as_bytes()).to_vec();
    authenticator_data.push(0x05);
    authenticator_data.extend_from_slice(&sign_count.to_be_bytes());
    let mut preimage = authenticator_data.clone();
    preimage.extend_from_slice(&client_data_hash);
    let signature: Signature = key.sign(&preimage);
    let der = signature.to_der();
    let mut response = vec![0, 0xa3, 0x01, 0xa2];
    cbor_text(&mut response, "id");
    cbor_bytes(&mut response, credential_id);
    cbor_text(&mut response, "type");
    cbor_text(&mut response, "public-key");
    response.push(0x02);
    cbor_bytes(&mut response, &authenticator_data);
    response.push(0x03);
    cbor_bytes(&mut response, der.as_bytes());
    response
}

fn cbor_text(output: &mut Vec<u8>, value: &str) {
    cbor_length(output, 3, value.len());
    output.extend_from_slice(value.as_bytes());
}

fn cbor_bytes(output: &mut Vec<u8>, value: &[u8]) {
    cbor_length(output, 2, value.len());
    output.extend_from_slice(value);
}

fn cbor_length(output: &mut Vec<u8>, major: u8, length: usize) {
    if length < 24 {
        output.push((major << 5) | length as u8);
    } else if length <= u8::MAX as usize {
        output.extend_from_slice(&[(major << 5) | 24, length as u8]);
    } else {
        output.push((major << 5) | 25);
        output.extend_from_slice(&(length as u16).to_be_bytes());
    }
}

fn exact(value: &str) -> ExactDefinitionRef {
    ExactDefinitionRef::parse(value).expect("exact definition ref")
}

fn builtin(value: &str) -> DefinitionSourceBinding {
    let definition_ref = exact(value);
    DefinitionSourceBinding {
        definition_ref: definition_ref.clone(),
        source: DefinitionSource::BuiltIn(definition_ref),
    }
}

fn successor_request() -> ProfileSelectionRequest {
    let schema_refs = [
        "handbook.schemas.artifacts.project-authority@1.0.0",
        "handbook.schemas.artifacts.project-authority@1.1.0",
        "handbook.schemas.artifacts.project-context@1.0.0",
        "handbook.schemas.artifacts.environment-context@1.0.0",
        "handbook.schemas.artifacts.environment-context@1.1.0",
        "handbook.schemas.artifacts.work-specification@1.0.0",
        "handbook.schemas.artifacts.decision-record@1.0.0",
        "handbook.schemas.artifacts.risk-record@1.0.0",
        "handbook.schemas.lifecycle.trigger-evidence@1.0.0",
        "handbook.schemas.security.approver-registry@1.0.0",
        "handbook.schemas.security.approver-registry-transition@1.0.0",
        "handbook.schemas.security.authenticator-registration-request@1.0.0",
        "handbook.schemas.security.authenticator-challenge@1.0.0",
        "handbook.schemas.security.authenticator-make-credential-response@1.0.0",
        "handbook.schemas.security.authenticator-registration@1.0.0",
        "handbook.schemas.security.authenticator-get-assertion-response@1.0.0",
        "handbook.schemas.security.authenticator-assertion@1.0.0",
        "handbook.schemas.security.approver-admin-api@1.0.0",
    ];
    let kind_refs = [
        "handbook.artifact-kind.project-authority@1.0.0",
        "handbook.artifact-kind.project-authority@1.1.0",
        "handbook.artifact-kind.project-context@1.0.0",
        "handbook.artifact-kind.project-context@1.1.0",
        "handbook.artifact-kind.environment-context@1.0.0",
        "handbook.artifact-kind.environment-context@1.1.0",
        "handbook.artifact-kind.work-specification@1.0.0",
        "handbook.artifact-kind.work-specification@1.1.0",
        "handbook.artifact-kind.decision-record@1.0.0",
        "handbook.artifact-kind.decision-record@1.1.0",
        "handbook.artifact-kind.risk-record@1.0.0",
        "handbook.artifact-kind.risk-record@1.1.0",
    ];
    ProfileSelectionRequest {
        selected_profile_ref: exact(SUCCESSOR_PROFILE_REF),
        profile_sources: vec![builtin(SUCCESSOR_PROFILE_REF)],
        stable_role_registry_sources: vec![builtin("handbook.roles.core@1.1.0")],
        schema_entry_sources: schema_refs
            .iter()
            .map(|reference| builtin(reference))
            .collect(),
        artifact_kind_sources: kind_refs
            .iter()
            .map(|reference| builtin(reference))
            .collect(),
        semantic_capability_sources: vec![builtin(
            "handbook.capabilities.constitutional-root@1.0.0",
        )],
        semantic_validator_sources: vec![
            builtin("handbook.semantic-validation.constitutional-root@1.0.0"),
            builtin("handbook.semantic-validation.constitutional-root@1.1.0"),
        ],
        project_condition_sources: vec![],
        vocabulary_sources: vec![builtin("handbook.vocabulary.shipped-root@1.0.0")],
        context_resolution_sources: vec![builtin("handbook.context-resolution.shipped-root@1.0.0")],
        context_resolution_policy_sources: vec![
            builtin("handbook.mutation-matcher.core@1.0.0"),
            builtin("handbook.resolution-escalation.core@1.0.0"),
            builtin("handbook.memory-promotion.core@1.0.0"),
        ],
        intake_definition_sources: vec![],
        allowed_schema_roots: vec!["definitions/schemas".to_owned()],
    }
}

fn released_request() -> ProfileSelectionRequest {
    let mut request = successor_request();
    request.selected_profile_ref = exact(RELEASED_PROFILE_REF);
    request.profile_sources = vec![builtin(RELEASED_PROFILE_REF)];
    request.artifact_kind_sources.retain(|binding| {
        (!binding.definition_ref.as_str().ends_with("@1.1.0")
            || binding.definition_ref.as_str() == "handbook.artifact-kind.project-authority@1.1.0")
            && !binding.definition_ref.as_str().ends_with("@1.2.0")
    });
    request.schema_entry_sources.retain(|binding| {
        binding.definition_ref.as_str() != "handbook.schemas.artifacts.environment-context@1.1.0"
    });
    request.project_condition_sources = vec![builtin(
        "handbook.condition.project.managed-operational-surface@1.0.0",
    )];
    request
}

fn repository_profile_request(profile_ref: &str, profile_path: &str) -> ProfileSelectionRequest {
    let names = [
        "project-authority",
        "project-context",
        "environment-context",
        "work-specification",
        "decision-record",
        "risk-record",
    ];
    ProfileSelectionRequest {
        selected_profile_ref: exact(profile_ref),
        profile_sources: vec![DefinitionSourceBinding {
            definition_ref: exact(profile_ref),
            source: DefinitionSource::RepositoryPath(profile_path.to_owned()),
        }],
        stable_role_registry_sources: vec![builtin("handbook.roles.core@1.1.0")],
        schema_entry_sources: names
            .iter()
            .map(|name| builtin(&format!("handbook.schemas.artifacts.{name}@1.0.0")))
            .collect(),
        artifact_kind_sources: names
            .iter()
            .map(|name| builtin(&format!("handbook.artifact-kind.{name}@1.0.0")))
            .collect(),
        semantic_capability_sources: vec![builtin(
            "handbook.capabilities.constitutional-root@1.0.0",
        )],
        semantic_validator_sources: vec![builtin(
            "handbook.semantic-validation.constitutional-root@1.0.0",
        )],
        project_condition_sources: vec![builtin(
            "handbook.condition.project.managed-operational-surface@1.0.0",
        )],
        vocabulary_sources: vec![builtin("handbook.vocabulary.shipped-root@1.0.0")],
        context_resolution_sources: vec![builtin("handbook.context-resolution.shipped-root@1.0.0")],
        context_resolution_policy_sources: vec![
            builtin("handbook.mutation-matcher.core@1.0.0"),
            builtin("handbook.resolution-escalation.core@1.0.0"),
            builtin("handbook.memory-promotion.core@1.0.0"),
        ],
        intake_definition_sources: vec![],
        allowed_schema_roots: vec!["definitions/schemas".to_owned()],
    }
}

fn incompatible_request() -> ProfileSelectionRequest {
    repository_profile_request(
        "example.profile.root@1.0.0",
        "tests/fixtures/hcm_1_2_repository_profile/root.yaml",
    )
}

fn decisions(request: ProfileSelectionRequest) -> ResolvedProfileDecisions {
    let profile = resolve_profile_selection(Path::new(env!("CARGO_MANIFEST_DIR")), request)
        .expect("resolve profile selection");
    ResolvedProfileDecisions::from_profile(&profile).expect("resolve profile decisions")
}

fn envelope() -> CharterIntakeEnvelope {
    let mut coverage = Vec::new();
    for coverage_id in OBSERVATIONAL {
        coverage.push(CharterCoverageSubmission {
            coverage_id: coverage_id.to_owned(),
            source_kind: CharterIntakeSourceKind::EvidencedInference,
            value_ref: format!("intake-values/{coverage_id}.json"),
            evidence_refs: vec![format!("evidence.{coverage_id}")],
            confidence: "high".to_owned(),
            freshness: Some("session".to_owned()),
            sensitivity: "internal".to_owned(),
            contradiction_refs: vec![],
            waiver_ref: None,
        });
    }
    for coverage_id in NORMATIVE {
        coverage.push(CharterCoverageSubmission {
            coverage_id: coverage_id.to_owned(),
            source_kind: CharterIntakeSourceKind::UserDeclaration,
            value_ref: format!("intake-values/{coverage_id}.json"),
            evidence_refs: vec![],
            confidence: "high".to_owned(),
            freshness: None,
            sensitivity: "internal".to_owned(),
            contradiction_refs: vec![],
            waiver_ref: None,
        });
    }
    CharterIntakeEnvelope {
        mode: CharterAcquisitionMode::Express,
        content: parse_definition_yaml(BOUNDARY_YAML).expect("canonical Charter"),
        coverage,
        consumer: CharterIntakeConsumer {
            kind: "handbook_skill".to_owned(),
            id: "handbook".to_owned(),
            version: "1.1".to_owned(),
        },
        prompt_event_refs: vec![],
        finalized_at_utc: "2026-07-26T21:00:00Z".to_owned(),
        expected_current_fingerprint: None,
    }
}

#[test]
fn shipped_decisions_select_the_successor_three_row_closure() {
    let shipped = resolve_shipped_profile_decisions(Path::new(env!("CARGO_MANIFEST_DIR"))).unwrap();
    assert_eq!(shipped.profile_ref().as_str(), SUCCESSOR_PROFILE_REF);
    assert_eq!(
        shipped.profile_definition_fingerprint().as_str(),
        SUCCESSOR_PROFILE_FINGERPRINT
    );

    let registry = shipped.registry();
    assert_eq!(
        registry
            .instance_ids()
            .iter()
            .map(|id| id.as_str())
            .collect::<Vec<_>>(),
        [
            "environment_context",
            "project_authority",
            "project_context"
        ]
    );
    for (id, intake_ref, renderer_ref) in [
        (
            "project_context",
            "handbook.intake.project-context@1.0.0",
            "handbook.renderer.project-context-review-markdown@1.0.0",
        ),
        (
            "environment_context",
            "handbook.intake.environment-context@1.0.0",
            "handbook.renderer.environment-context-review-markdown@1.0.0",
        ),
    ] {
        let instance_id = registry
            .instance_ids()
            .into_iter()
            .find(|candidate| candidate.as_str() == id)
            .unwrap();
        let instance = registry.instance(instance_id).unwrap();
        assert_eq!(
            instance.intake_definition_ref().unwrap().as_str(),
            intake_ref
        );
        assert_eq!(instance.renderer_definition_refs().len(), 1);
        assert_eq!(
            instance.renderer_definition_refs()[0].as_str(),
            renderer_ref
        );
    }

    let charter =
        evaluate_charter_intake(&shipped, envelope(), None).expect("successor shipped intake");
    assert_eq!(charter.intake.profile_ref, RELEASED_PROFILE_REF);
    assert_eq!(
        charter.intake.resolved_profile_fingerprint,
        RELEASED_PROFILE_FINGERPRINT
    );
}

#[test]
fn exact_successor_decisions_pass_the_charter_membrane_but_emit_released_identity() {
    let released = decisions(released_request());
    let successor = decisions(successor_request());
    assert_eq!(released.profile_ref().as_str(), RELEASED_PROFILE_REF);
    assert_eq!(
        released.profile_definition_fingerprint().as_str(),
        RELEASED_PROFILE_FINGERPRINT
    );
    assert_eq!(successor.profile_ref().as_str(), SUCCESSOR_PROFILE_REF);
    assert_eq!(
        successor.profile_definition_fingerprint().as_str(),
        SUCCESSOR_PROFILE_FINGERPRINT
    );

    let registry = load_shipped_charter_definition_registry().unwrap();
    registry.validate_selected_decisions(&released).unwrap();
    registry.validate_selected_decisions(&successor).unwrap();

    let released_bundle =
        evaluate_charter_intake(&released, envelope(), None).expect("released intake");
    let successor_bundle =
        evaluate_charter_intake(&successor, envelope(), None).expect("successor intake");
    assert_eq!(
        successor_bundle.normalized_content,
        released_bundle.normalized_content
    );
    assert_eq!(successor_bundle.intake, released_bundle.intake);
    assert_eq!(
        successor_bundle.candidate_subject,
        released_bundle.candidate_subject
    );
    assert_eq!(successor_bundle.intake.profile_ref, RELEASED_PROFILE_REF);
    assert_eq!(
        successor_bundle.intake.resolved_profile_fingerprint,
        RELEASED_PROFILE_FINGERPRINT
    );
}

#[test]
fn invalid_compatible_profile_decisions_cannot_produce_charter_intake() {
    let incompatible = decisions(incompatible_request());
    let error = evaluate_charter_intake(&incompatible, envelope(), None)
        .expect_err("an unreviewed profile tuple must refuse before producing intake records");
    assert_eq!(
        error.kind(),
        CharterIntakeErrorKind::SourceAuthorityMismatch
    );
}

#[test]
fn non_exact_crossed_rebound_and_repeated_profile_sources_refuse_selection() {
    for non_exact in [
        "handbook.profile.shipped-root@1.2",
        "handbook.profile.shipped-root@1",
        "handbook.profile.shipped-root@>=1.1.0",
    ] {
        assert!(
            ExactDefinitionRef::parse(non_exact).is_err(),
            "{non_exact} must not be admitted as an exact profile ref"
        );
    }

    let mut cases = Vec::new();

    let mut crossed_successor = successor_request();
    crossed_successor.profile_sources[0].source =
        DefinitionSource::BuiltIn(exact(RELEASED_PROFILE_REF));
    cases.push(("successor ref with released source", crossed_successor));

    let mut crossed_released = successor_request();
    crossed_released.selected_profile_ref = exact(RELEASED_PROFILE_REF);
    crossed_released.profile_sources = vec![DefinitionSourceBinding {
        definition_ref: exact(RELEASED_PROFILE_REF),
        source: DefinitionSource::BuiltIn(exact(SUCCESSOR_PROFILE_REF)),
    }];
    cases.push(("released ref with successor source", crossed_released));

    let mut package_rebound = successor_request();
    package_rebound.profile_sources[0].source = DefinitionSource::RepositoryPath(
        "definitions/profiles/handbook.profile.shipped-root/1.2.0.yaml".to_owned(),
    );
    cases.push((
        "package profile rebound to repository bytes",
        package_rebound,
    ));

    let mut repeated = successor_request();
    repeated.profile_sources.push(DefinitionSourceBinding {
        definition_ref: exact(SUCCESSOR_PROFILE_REF),
        source: DefinitionSource::RepositoryPath(
            "definitions/profiles/handbook.profile.shipped-root/1.2.0.yaml".to_owned(),
        ),
    });
    cases.push(("second source for the selected exact ref", repeated));

    let near_ref = "handbook.profile.shipped-root@1.2.1";
    let mut near_version = successor_request();
    near_version.selected_profile_ref = exact(near_ref);
    near_version.profile_sources = vec![DefinitionSourceBinding {
        definition_ref: exact(near_ref),
        source: DefinitionSource::RepositoryPath(
            "definitions/profiles/handbook.profile.shipped-root/1.2.0.yaml".to_owned(),
        ),
    }];
    cases.push(("near version with successor bytes", near_version));

    for (case, request) in cases {
        assert!(
            resolve_profile_selection(Path::new(env!("CARGO_MANIFEST_DIR")), request).is_err(),
            "{case} must refuse before producing compatible decisions"
        );
    }
}

#[test]
fn every_project_authority_descriptor_mutation_refuses_on_a_fresh_second_read() {
    let mutations = [
        (
            "/artifact_instances/0/schema_id",
            serde_json::json!("wrong"),
        ),
        (
            "/artifact_instances/0/schema_version",
            serde_json::json!("2.0"),
        ),
        (
            "/artifact_instances/0/id",
            serde_json::json!("other_authority"),
        ),
        (
            "/artifact_instances/0/kind_ref",
            serde_json::json!("handbook.artifact-kind.project-context@1.0.0"),
        ),
        (
            "/artifact_instances/0/role_ref",
            serde_json::json!("project_context"),
        ),
        (
            "/artifact_instances/0/capability_refs",
            serde_json::json!([]),
        ),
        ("/artifact_instances/0/label", serde_json::json!("Other")),
        (
            "/artifact_instances/0/canonical_path",
            serde_json::json!(".handbook/project/other.yaml"),
        ),
        (
            "/artifact_instances/0/requiredness/mode",
            serde_json::json!("optional"),
        ),
        (
            "/artifact_instances/0/requiredness/condition_ref",
            serde_json::json!("handbook.condition.project.managed-operational-surface@1.0.0"),
        ),
        (
            "/artifact_instances/0/depends_on",
            serde_json::json!([{
                "target_kind": "instance",
                "target_ref": "project_context",
                "target_contract_ref": null,
                "cardinality": "one"
            }]),
        ),
        (
            "/artifact_instances/0/lifecycle_policy_ref",
            serde_json::json!("handbook.lifecycle.constitutional-review-lock@1.0.0"),
        ),
        (
            "/artifact_instances/0/intake_definition_ref",
            serde_json::json!("handbook.intake.charter@1.0.0"),
        ),
        (
            "/artifact_instances/0/renderer_definition_refs",
            serde_json::json!([
                "handbook.renderer.charter-review-markdown@1.0.0",
                "handbook.renderer.charter-review-markdown@1.0.0"
            ]),
        ),
        (
            "/artifact_instances/0/projection_definition_refs",
            serde_json::json!(["example.projection.other@1.0.0"]),
        ),
        (
            "/artifact_instances/0/validation_overlay_refs",
            serde_json::json!(["example.overlay.other@1.0.0"]),
        ),
        (
            "/artifact_instances/0/extensions",
            serde_json::json!({"unexpected": true}),
        ),
        (
            "/profile_fingerprint",
            serde_json::json!(SUCCESSOR_PROFILE_FINGERPRINT),
        ),
    ];

    for (pointer, replacement) in mutations {
        let repo = tempfile::tempdir().unwrap();
        let profile_path = repo.path().join("root.yaml");
        fs::write(&profile_path, INCOMPATIBLE_PROFILE_YAML).unwrap();
        let request = repository_profile_request("example.profile.root@1.0.0", "root.yaml");
        resolve_profile_selection(repo.path(), request.clone())
            .expect("the unchanged custom fixture must pass its first read");

        let mut document = parse_definition_yaml(INCOMPATIBLE_PROFILE_YAML).unwrap();
        *document
            .pointer_mut(pointer)
            .unwrap_or_else(|| panic!("fixture pointer {pointer}")) = replacement;
        fs::write(&profile_path, serde_yaml_bw::to_string(&document).unwrap()).unwrap();
        assert!(
            resolve_profile_selection(repo.path(), request).is_err(),
            "mutating {pointer} must refuse on the fresh second read"
        );
    }

    for predicate in [
        "charter.id().as_str() == \"project_authority\"",
        "charter.kind_ref().as_str() == \"handbook.artifact-kind.project-authority@1.1.0\"",
        "role.role_id() == \"constitutional_authority\"",
        "charter.capabilities().len() == 1",
        "capability_id().as_str() == \"constitutional_root\"",
        "contract_ref().as_str()\n                == \"handbook.capabilities.constitutional-root@1.0.0\"",
        "charter.label() == \"Charter\"",
        "charter.canonical_path() == \".handbook/project/charter.yaml\"",
        "charter.requiredness_mode() == crate::artifact_instance::RequirednessMode::Always",
        "charter.condition_ref().is_none()",
        "charter.dependencies().is_empty()",
        "charter.renderer_definition_refs().len() == 1",
        "charter.projection_definition_refs().is_empty()",
        "charter.validation_overlay_refs().is_empty()",
        "charter.extensions().is_empty()",
    ] {
        assert!(
            REGISTRY_SOURCE.contains(predicate),
            "exact descriptor predicate is absent: {predicate}"
        );
    }
}

#[test]
fn successor_authoring_persists_the_released_charter_identity_and_exact_bytes() {
    let released = decisions(released_request());
    let successor = decisions(successor_request());
    let repo = tempfile::tempdir().unwrap();
    let service = CharterAuthorPersistenceServiceV1::new(repo.path());
    let released_result = service
        .persist(&released, envelope(), None)
        .expect("released authoring");
    let intake_path = repo
        .path()
        .join(".handbook/state")
        .join(&released_result.intake_ref);
    let content_path = repo
        .path()
        .join(".handbook/state")
        .join(&released_result.normalized_content_ref);
    let candidate_path = repo
        .path()
        .join(".handbook/evidence/charter")
        .join(&released_result.candidate_ref);
    let validation_path = repo
        .path()
        .join(".handbook/state")
        .join(&released_result.lifecycle_validation_result_ref);
    let released_bytes = [
        fs::read(&intake_path).unwrap(),
        fs::read(&content_path).unwrap(),
        fs::read(&candidate_path).unwrap(),
        fs::read(&validation_path).unwrap(),
    ];
    let successor_result = service
        .persist(&successor, envelope(), None)
        .expect("successor-compatible authoring");

    assert_eq!(successor_result.intake_ref, released_result.intake_ref);
    assert_eq!(
        successor_result.intake_fingerprint,
        released_result.intake_fingerprint
    );
    assert_eq!(
        successor_result.candidate_ref,
        released_result.candidate_ref
    );
    assert_eq!(
        successor_result.candidate_fingerprint,
        released_result.candidate_fingerprint
    );
    assert_eq!(
        successor_result.normalized_content_ref,
        released_result.normalized_content_ref
    );
    assert_eq!(
        successor_result.lifecycle_validation_result_ref,
        released_result.lifecycle_validation_result_ref
    );

    assert_eq!(
        [
            fs::read(&intake_path).unwrap(),
            fs::read(&content_path).unwrap(),
            fs::read(&candidate_path).unwrap(),
            fs::read(&validation_path).unwrap(),
        ],
        released_bytes
    );

    let successor_validation: Value = serde_json::from_slice(&fs::read(validation_path).unwrap())
        .expect("lifecycle validation result");
    assert_eq!(
        successor_validation["profile_ref"],
        Value::String(RELEASED_PROFILE_REF.to_owned())
    );
    assert_eq!(
        successor_validation["resolved_profile_fingerprint"],
        Value::String(RELEASED_PROFILE_FINGERPRINT.to_owned())
    );
}

#[test]
fn successor_approval_currentness_accepts_the_released_candidate_identity() {
    let successor = decisions(successor_request());
    let repo = tempfile::tempdir().unwrap();
    let authored = CharterAuthorPersistenceServiceV1::new(repo.path())
        .persist(&successor, envelope(), None)
        .expect("successor-compatible authoring");
    let result = CharterApprovalServiceV1::new(repo.path(), UnavailableApprovalPort)
        .approve_candidate(
            &successor,
            CharterApprovalRequestV1 {
                operation_id: "hcm-2-4-successor-currentness".to_owned(),
                candidate_ref: authored.candidate_ref,
                approval_class: "Project owner approval".to_owned(),
                authority_ref: "Project owner".to_owned(),
                accepted_waiver_refs: vec![],
            },
        );
    let CharterApprovalResultV1::Refused(refusal) = result else {
        panic!("an absent approver registry cannot authorize approval")
    };
    assert_ne!(
        refusal.message,
        "candidate does not bind the current exact Charter profile, kind, schema, and policy"
    );
}

#[test]
fn successor_authored_candidate_completes_the_released_approval_and_promotion_path() {
    let successor = decisions(successor_request());
    let repo = tempfile::tempdir().unwrap();
    let authored = CharterAuthorPersistenceServiceV1::new(repo.path())
        .persist(&successor, envelope(), None)
        .expect("successor-compatible authoring");

    let mut registry =
        ApproverRegistryServiceV1::for_repository(repo.path(), DynamicApprovalPort::new());
    assert_eq!(
        registry
            .bootstrap_approver_registry(bootstrap_request())
            .to_json_value()
            .unwrap()["status"],
        "succeeded"
    );
    let mut approvals = CharterApprovalServiceV1::new(repo.path(), registry.into_port());
    let approval = approvals.approve_candidate(
        &successor,
        CharterApprovalRequestV1 {
            operation_id: "hcm-2-4-successor-approval".to_owned(),
            candidate_ref: authored.candidate_ref.clone(),
            approval_class: "Project owner approval".to_owned(),
            authority_ref: "Project owner".to_owned(),
            accepted_waiver_refs: vec![],
        },
    );
    let CharterApprovalResultV1::Succeeded(approval) = approval else {
        panic!("successor-authored candidate approval must succeed: {approval:?}")
    };
    let committed = CharterPromotionWorkflowServiceV1::new(repo.path())
        .promote(CharterPromotionIntentV1 {
            candidate_ref: authored.candidate_ref,
            approval_ref: approval.approval_ref,
            expected_current_fingerprint: None,
        })
        .expect("successor-authored candidate promotion");
    let observed = CharterAuthorityTransactionServiceV1::new(repo.path())
        .read_committed_charter()
        .unwrap()
        .expect("committed Charter authority");
    assert_eq!(
        observed.canonical_fingerprint,
        committed.canonical_fingerprint
    );
    assert_eq!(approvals.into_port().get_calls, 1);
}

#[test]
fn promotion_and_transaction_preflight_retain_the_released_profile_pair() {
    assert!(PROMOTION_SOURCE.contains(
        "string_field(candidate, \"resolved_profile_fingerprint\").map_err(map_lineage_candidate)?\n        != SELECTED_PROFILE_FINGERPRINT"
    ));
    assert!(
        !PROMOTION_SOURCE.contains("if decisions.profile_ref().as_str() != SELECTED_PROFILE_REF")
    );
    assert!(PROMOTION_SOURCE.contains("\"profile_ref\": SELECTED_PROFILE_REF,"));
    assert!(PROMOTION_SOURCE
        .contains("\"resolved_profile_fingerprint\": SELECTED_PROFILE_FINGERPRINT,"));
    assert!(!PROMOTION_SOURCE.contains("\"profile_ref\": decisions.profile_ref().as_str(),"));
    assert!(!PROMOTION_SOURCE.contains(
        "\"resolved_profile_fingerprint\": decisions.profile_definition_fingerprint().as_str(),"
    ));
    assert!(TRANSACTION_SOURCE.contains(
        "require_equal_string(&promotion.value, \"profile_ref\", SELECTED_PROFILE_REF)?;"
    ));
    assert!(TRANSACTION_SOURCE.contains(
        "require_equal_string(\n            &promotion.value,\n            \"resolved_profile_fingerprint\",\n            SELECTED_PROFILE_FINGERPRINT,\n        )?;"
    ));
    assert!(!TRANSACTION_SOURCE.contains(
        "require_equal_string(\n            &promotion.value,\n            \"profile_ref\",\n            decisions.profile_ref().as_str(),\n        )?;"
    ));
}
