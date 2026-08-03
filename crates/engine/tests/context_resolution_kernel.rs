#![allow(dead_code)]

#[path = "../src/artifact_lineage_store.rs"]
mod artifact_lineage_store;
#[path = "../src/canonical_repo_support.rs"]
mod canonical_repo_support;

use handbook_engine::{
    parse_definition_yaml, resolve_profile_selection, ApproverAdminRequestV1,
    ApproverRegistryServiceV1, ContextResolutionAuthorityAdmission,
    ContextResolutionAuthorityAdmissionResolver, ContextResolutionAuthorityUse,
    ContextResolutionDimensions, ContextResolutionEnvelope, ContextResolutionEnvelopeInput,
    ContextResolutionEscalationDisposition, ContextResolutionEscalationOutcome,
    ContextResolutionEscalationRequest, ContextResolutionExactBinding,
    ContextResolutionKernelErrorKind, ContextResolutionMemoryDecision,
    ContextResolutionMutationDecision, ContextResolutionMutationEffect,
    ContextResolutionMutationRule, ContextResolutionPolicyRegistry,
    ContextResolutionPromotionDisposition, ContextResolutionPromotionOutcome,
    ContextResolutionPromotionRequest, ContextResolutionSemanticMemoryTarget,
    ContextResolutionStackDefinition, ContextResolutionTransitionRegistry,
    ContextResolutionValidationDecision, DefinitionFingerprint, NativeAuthenticatorPortErrorV1,
    NativeAuthenticatorPortV1, RepositoryInvocationIdentityServiceV1, ResolvedInstanceProfile,
    AUTHENTICATOR_RP_ID,
};
use p256::ecdsa::{signature::Signer, Signature, SigningKey};
use serde_json::json;
use sha2::{Digest, Sha256};
use std::fs;
use std::path::Path;

const FINGERPRINT: &str = "sha256:aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa";
const AUTHORITY_KIND_REF: &str = "example.artifact-kind.context-resolution-authority@1.0.0";
const AUTHORITY_INSTANCE_ID: &str = "context_resolution_authority";
const STACK_PATH: &str =
    "definitions/context-resolution/handbook.context-resolution.shipped-root/1.0.0.yaml";
const POLICY_PATHS: [&str; 3] = [
    "definitions/context-resolution-policies/handbook.mutation-matcher.core/1.0.0.yaml",
    "definitions/context-resolution-policies/handbook.resolution-escalation.core/1.0.0.yaml",
    "definitions/context-resolution-policies/handbook.memory-promotion.core/1.0.0.yaml",
];

fn authority_fixture_root() -> std::path::PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/hcm_3_2_context_resolution")
}

fn copy_fixture_tree(source: &Path, target: &Path) {
    fs::create_dir_all(target).unwrap();
    for entry in fs::read_dir(source).unwrap() {
        let entry = entry.unwrap();
        let source_path = entry.path();
        let target_path = target.join(entry.file_name());
        if source_path.is_dir() {
            copy_fixture_tree(&source_path, &target_path);
        } else {
            fs::copy(source_path, target_path).unwrap();
        }
    }
}

struct AuthorityPort {
    key: SigningKey,
    added_key: SigningKey,
    credential_id: Vec<u8>,
    added_credential_id: Vec<u8>,
    make_calls: usize,
    get_calls: Vec<Vec<u8>>,
    refused: bool,
    fixed_sign_count: Option<u32>,
    replay_response: Option<Vec<u8>>,
    last_response: Option<Vec<u8>>,
}

impl AuthorityPort {
    fn new() -> Self {
        Self {
            key: SigningKey::from_slice(&[7_u8; 32]).unwrap(),
            added_key: SigningKey::from_slice(&[6_u8; 32]).unwrap(),
            credential_id: vec![0x32; 32],
            added_credential_id: vec![0x31; 32],
            make_calls: 0,
            get_calls: Vec::new(),
            refused: false,
            fixed_sign_count: None,
            replay_response: None,
            last_response: None,
        }
    }
}

impl NativeAuthenticatorPortV1 for AuthorityPort {
    fn make_credential(
        &mut self,
        _request_cbor: &[u8],
    ) -> Result<Vec<u8>, NativeAuthenticatorPortErrorV1> {
        self.make_calls += 1;
        Ok(if self.make_calls == 1 {
            make_credential_response(&self.key, &self.credential_id)
        } else {
            make_credential_response(&self.added_key, &self.added_credential_id)
        })
    }

    fn get_assertion(
        &mut self,
        request_cbor: &[u8],
    ) -> Result<Vec<u8>, NativeAuthenticatorPortErrorV1> {
        self.get_calls.push(request_cbor.to_vec());
        if self.refused {
            return Err(NativeAuthenticatorPortErrorV1::Unavailable);
        }
        if let Some(response) = &self.replay_response {
            return Ok(response.clone());
        }
        let marker = request_cbor
            .windows(3)
            .position(|window| window == [0x02, 0x58, 0x20])
            .expect("GetAssertion client-data hash marker");
        let client_data_hash = request_cbor[marker + 3..marker + 35].try_into().unwrap();
        let sign_count = self.fixed_sign_count.unwrap_or(self.get_calls.len() as u32);
        let response =
            assertion_response(&self.key, &self.credential_id, client_data_hash, sign_count);
        self.last_response = Some(response.clone());
        Ok(response)
    }
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
    let mut response = vec![0, 0xa3, 0x01, 0xa2];
    cbor_text(&mut response, "id");
    cbor_bytes(&mut response, credential_id);
    cbor_text(&mut response, "type");
    cbor_text(&mut response, "public-key");
    response.push(0x02);
    cbor_bytes(&mut response, &authenticator_data);
    response.push(0x03);
    cbor_bytes(&mut response, signature.to_der().as_bytes());
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

fn authority_mappings() -> serde_json::Value {
    json!({
        "root_creation": {"approval_class":"context_resolution_root","authority_ref":"work.root.owner"},
        "parent": {"approval_class":"context_resolution_parent","authority_ref":"work.parent.owner"},
        "requested": {"approval_class":"context_resolution_requested","authority_ref":"work.request.owner"},
        "approving": {"approval_class":"context_resolution_approving","authority_ref":"work.approver"},
        "decision": {"approval_class":"context_resolution_decision","authority_ref":"decision.owner"},
        "evidence": {"approval_class":"context_resolution_evidence","authority_ref":"evidence.owner"},
        "trigger_condition": {"approval_class":"context_resolution_trigger","authority_ref":"trigger.owner"},
        "constraint": {"approval_class":"context_resolution_constraint","authority_ref":"constraint.owner"},
        "source": {"approval_class":"context_resolution_source","authority_ref":"source.owner"},
        "target": {"approval_class":"context_resolution_target","authority_ref":"target.owner"},
        "target_memory": {"approval_class":"context_resolution_target_memory","authority_ref":"memory.owner"}
    })
}

fn candidate_mappings() -> serde_json::Value {
    json!({
        "dimension_rank_increase": {"trigger":{"ref":"trigger.dimension-rank-increase@1.0.0","fingerprint":"sha256:1111111111111111111111111111111111111111111111111111111111111111"},"missing_condition":"one_or_more_dimension_ranks_exceed_parent","evidence_requirement":"current_and_proposed_envelopes"},
        "mutation_allow_expansion": {"trigger":{"ref":"trigger.mutation-allow-expansion@1.0.0","fingerprint":"sha256:2222222222222222222222222222222222222222222222222222222222222222"},"missing_condition":"child_allow_not_provably_contained","evidence_requirement":"parent_and_child_mutation_rules"},
        "missing_context": {"trigger":{"ref":"trigger.missing-context@1.0.0","fingerprint":"sha256:3333333333333333333333333333333333333333333333333333333333333333"},"missing_condition":"required_context_record_absent","evidence_requirement":"available_context_and_missing_ref"},
        "missing_authority": {"trigger":{"ref":"trigger.missing-authority@1.0.0","fingerprint":"sha256:4444444444444444444444444444444444444444444444444444444444444444"},"missing_condition":"required_authority_mapping_absent","evidence_requirement":"current_authority_and_requested_pair"}
    })
}

fn authority_coverage_submissions(binding: &serde_json::Value) -> Vec<serde_json::Value> {
    fn visit(
        value: &serde_json::Value,
        fields: &mut Vec<String>,
        submissions: &mut Vec<serde_json::Value>,
    ) {
        if let Some(object) = value.as_object().filter(|object| !object.is_empty()) {
            for (field, child) in object {
                fields.push(field.clone());
                visit(child, fields, submissions);
                fields.pop();
            }
        } else {
            submissions.push(json!({
                "coverage_id":format!("context_resolution_authority.{}", fields.join(".")),
                "state":"supplied",
                "source_kind":"user_declaration",
                "value":value,
                "specificity":"exact",
                "confidence":"high",
                "contradiction_refs":[]
            }));
        }
    }
    let mut submissions = Vec::new();
    visit(binding, &mut Vec::new(), &mut submissions);
    submissions
}

#[test]
fn authority_fixture_has_a_repository_profile_selection() {
    let root = authority_fixture_root();
    let selection =
        handbook_engine::artifact_intake_registry::RepositoryProfileSelectionV1::from_json_bytes(
            &fs::read(root.join(".handbook/profile-selection.json"))
                .expect("the dedicated HCM-3.2 authority fixture closure must exist"),
        )
        .unwrap();
    let profile = resolve_profile_selection(&root, selection.profile_request()).unwrap();
    assert_eq!(
        profile.exact_ref().as_str(),
        "example.profile.hcm-3-2-context-resolution-authority@1.0.0"
    );
}

#[test]
fn authority_fixture_definition_fingerprints_are_exact() {
    let definitions = authority_fixture_root().join(".handbook/definitions");
    let schema_path = definitions.join("schemas/context-resolution-authority-1.0.0.schema.json");
    let document_fingerprint = DefinitionFingerprint::from_bytes(&fs::read(&schema_path).unwrap());
    let closure_fingerprint = DefinitionFingerprint::from_json_value(&json!([{
        "document_ref": ".handbook/definitions/schemas/context-resolution-authority-1.0.0.schema.json",
        "document_fingerprint": document_fingerprint.as_str(),
    }]))
    .unwrap();

    let mut entry = parse_definition_yaml(
        &fs::read(definitions.join("schemas/context-resolution-authority-1.0.0.entry.yaml"))
            .unwrap(),
    )
    .unwrap();
    let supplied_document = entry["document_fingerprint"].as_str().unwrap().to_owned();
    let supplied_closure = entry["closure_fingerprint"].as_str().unwrap().to_owned();
    let supplied_entry = entry
        .as_object_mut()
        .unwrap()
        .remove("entry_fingerprint")
        .unwrap()
        .as_str()
        .unwrap()
        .to_owned();
    entry["document_fingerprint"] = json!(document_fingerprint.as_str());
    entry["closure_fingerprint"] = json!(closure_fingerprint.as_str());
    let entry_fingerprint = DefinitionFingerprint::from_json_value(&entry).unwrap();

    let mut kind = parse_definition_yaml(
        &fs::read(definitions.join("artifact-kinds/context-resolution-authority-1.0.0.yaml"))
            .unwrap(),
    )
    .unwrap();
    let supplied_kind = kind
        .as_object_mut()
        .unwrap()
        .remove("definition_fingerprint")
        .unwrap()
        .as_str()
        .unwrap()
        .to_owned();
    let kind_fingerprint = DefinitionFingerprint::from_json_value(&json!({
        "definition": kind,
        "stable_role_registry_fingerprint": "sha256:0c85b1b53786e7980c4fd0d7975cd9cde1a3eae2bc8daceb23be1a1731263029",
        "schema_entry_fingerprint": entry_fingerprint.as_str(),
        "schema_closure_fingerprint": closure_fingerprint.as_str(),
    }))
    .unwrap();

    let mut intake = parse_definition_yaml(
        &fs::read(definitions.join("intakes/context-resolution-authority-1.0.0.yaml")).unwrap(),
    )
    .unwrap();
    let supplied_intake = intake
        .as_object_mut()
        .unwrap()
        .remove("intake_definition_fingerprint")
        .unwrap()
        .as_str()
        .unwrap()
        .to_owned();
    let intake_fingerprint = DefinitionFingerprint::from_json_value(&intake).unwrap();

    assert_eq!(
        [
            supplied_document,
            supplied_closure,
            supplied_entry,
            supplied_kind,
            supplied_intake
        ],
        [
            document_fingerprint.to_string(),
            closure_fingerprint.to_string(),
            entry_fingerprint.to_string(),
            kind_fingerprint.to_string(),
            intake_fingerprint.to_string(),
        ]
    );
}

fn seed_real_authority_repository_with_credentials(
    add_second_credential: bool,
) -> (tempfile::TempDir, AuthorityPort, String, String) {
    let repo = tempfile::tempdir().unwrap();
    copy_fixture_tree(&authority_fixture_root(), repo.path());
    let identity = RepositoryInvocationIdentityServiceV1::new()
        .initialize_for_setup(repo.path())
        .unwrap();
    let selection =
        handbook_engine::artifact_intake_registry::RepositoryProfileSelectionV1::from_json_bytes(
            &fs::read(repo.path().join(".handbook/profile-selection.json")).unwrap(),
        )
        .unwrap();
    let profile = resolve_profile_selection(repo.path(), selection.profile_request()).unwrap();
    let resolved_registry =
        handbook_engine::artifact_registry::ResolvedArtifactRegistry::from_profile(&profile)
            .unwrap();
    let leaves = resolved_registry
        .intake_schema_leaf_shapes(
            &handbook_engine::ExactDefinitionRef::parse(
                "example.schemas.context-resolution-authority@1.0.0",
            )
            .unwrap(),
        )
        .unwrap();
    assert_eq!(
        leaves.len(),
        53,
        "unexpected authority schema leaves: {leaves:?}"
    );
    let mappings = authority_mappings();
    let mut quorum = mappings
        .as_object()
        .unwrap()
        .values()
        .cloned()
        .collect::<Vec<_>>();
    quorum.sort_by(|left, right| {
        left["approval_class"]
            .as_str()
            .unwrap()
            .cmp(right["approval_class"].as_str().unwrap())
            .then_with(|| {
                left["authority_ref"]
                    .as_str()
                    .unwrap()
                    .cmp(right["authority_ref"].as_str().unwrap())
            })
    });
    let request = ApproverAdminRequestV1::from_json_value(json!({
        "schema_id":"handbook.approver-admin-request",
        "schema_version":"1.0",
        "operation":"bootstrap",
        "operation_id":"hcm-3-2-authority-bootstrap",
        "repository_identity_fingerprint":identity.repository_identity_fingerprint(),
        "expected_registry_state_fingerprint":null,
        "expected_transition_fingerprint":null,
        "initial_charter_quorum":quorum
    }))
    .unwrap();
    let mut registry = ApproverRegistryServiceV1::for_repository(repo.path(), AuthorityPort::new());
    let result = registry
        .bootstrap_approver_registry(request)
        .to_json_value()
        .unwrap();
    assert_eq!(result["status"], "succeeded");
    let final_result = if add_second_credential {
        let add = ApproverAdminRequestV1::from_json_value(json!({
            "schema_id":"handbook.approver-admin-request",
            "schema_version":"1.0",
            "operation":"add_credential",
            "operation_id":"hcm-3-2-authority-add-second",
            "repository_identity_fingerprint":identity.repository_identity_fingerprint(),
            "expected_registry_state_fingerprint":result["result_registry_state_fingerprint"],
            "expected_transition_fingerprint":result["transition_fingerprint"],
            "approval_mappings":quorum,
        }))
        .unwrap();
        let add_result = registry
            .add_approver_credential(add)
            .to_json_value()
            .unwrap();
        assert_eq!(add_result["status"], "succeeded", "{add_result:#}");
        add_result
    } else {
        result
    };
    let port = registry.into_port();
    let registry_binding = json!({
        "state_ref":final_result["result_registry_state_ref"],
        "state_fingerprint":final_result["result_registry_state_fingerprint"],
        "head_transition_ref":final_result["transition_ref"],
        "head_transition_fingerprint":final_result["transition_fingerprint"]
    });

    let mut binding = json!({
        "schema_id":"handbook.context-resolution-authority-binding",
        "schema_version":"1.0",
        "binding_id":"handbook.context-resolution-authority.fixture",
        "binding_version":"1.0.0",
        "repository_identity_fingerprint":identity.repository_identity_fingerprint(),
        "resolved_profile":{"ref":profile.exact_ref().as_str(),"fingerprint":profile.resolved_profile_fingerprint().as_str()},
        "resolution_stack":{"ref":profile.context_resolution().exact_ref().as_str(),"fingerprint":profile.context_resolution().definition_fingerprint().as_str()},
        "approver_registry":registry_binding,
        "authority_mappings":mappings,
        "candidate_mappings":candidate_mappings(),
        "extensions":{}
    });
    let binding_fingerprint = DefinitionFingerprint::from_json_value(&binding)
        .unwrap()
        .to_string();
    binding["binding_fingerprint"] = json!(binding_fingerprint.clone());
    let canonical_bytes = serde_json::to_vec_pretty(&binding).unwrap();
    fs::write(
        repo.path()
            .join(".handbook/project/context-resolution-authority.json"),
        &canonical_bytes,
    )
    .unwrap();
    let promoted = DefinitionFingerprint::from_bytes(&canonical_bytes).to_string();
    (repo, port, promoted, binding_fingerprint)
}

fn seed_real_authority_repository() -> (tempfile::TempDir, AuthorityPort, String, String) {
    seed_real_authority_repository_with_credentials(false)
}

fn load_kernel_profile_stack(
    repo: &Path,
) -> (ResolvedInstanceProfile, ContextResolutionStackDefinition) {
    let selection =
        handbook_engine::artifact_intake_registry::RepositoryProfileSelectionV1::from_json_bytes(
            &fs::read(repo.join(".handbook/profile-selection.json")).unwrap(),
        )
        .unwrap();
    let profile = resolve_profile_selection(repo, selection.profile_request()).unwrap();
    let crate_root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let policies =
        ContextResolutionPolicyRegistry::load(crate_root, &POLICY_PATHS.map(str::to_string))
            .unwrap();
    let stack = ContextResolutionStackDefinition::load(crate_root, STACK_PATH, &policies).unwrap();
    (profile, stack)
}

fn envelope_input(
    id: &str,
    profile: &ResolvedInstanceProfile,
    stack: &ContextResolutionStackDefinition,
    level: &str,
    dimensions: [&str; 6],
    parent: Option<ContextResolutionExactBinding>,
    rules: Vec<ContextResolutionMutationRule>,
) -> ContextResolutionEnvelopeInput {
    ContextResolutionEnvelopeInput::new(
        id,
        "objective.fixture",
        ContextResolutionExactBinding::new(
            profile.exact_ref().as_str(),
            profile.resolved_profile_fingerprint().as_str(),
        )
        .unwrap(),
        ContextResolutionExactBinding::new(
            stack.exact_ref().as_str(),
            stack.definition_fingerprint().as_str(),
        )
        .unwrap(),
        level,
        ContextResolutionDimensions::new(dimensions).unwrap(),
        parent,
        Vec::new(),
        rules,
        Vec::new(),
    )
    .unwrap()
}

fn admit_subject(
    resolver: &mut ContextResolutionAuthorityAdmissionResolver,
    port: &mut AuthorityPort,
    authority_use: ContextResolutionAuthorityUse,
    subject: &ContextResolutionExactBinding,
) -> ContextResolutionAuthorityAdmission {
    resolver.admit(port, authority_use, subject).unwrap()
}

#[test]
fn real_authority_path_admits_all_eleven_uses_and_replays_exactly() {
    let (repo, mut port, artifact_fingerprint, binding_fingerprint) =
        seed_real_authority_repository();
    assert_eq!(port.make_calls, 1);
    let mut resolver = ContextResolutionAuthorityAdmissionResolver::new(
        repo.path(),
        AUTHORITY_KIND_REF,
        AUTHORITY_INSTANCE_ID,
        &artifact_fingerprint,
    )
    .unwrap();
    let uses = [
        ContextResolutionAuthorityUse::RootCreation,
        ContextResolutionAuthorityUse::Parent,
        ContextResolutionAuthorityUse::Requested,
        ContextResolutionAuthorityUse::Approving,
        ContextResolutionAuthorityUse::Decision,
        ContextResolutionAuthorityUse::Evidence,
        ContextResolutionAuthorityUse::TriggerCondition,
        ContextResolutionAuthorityUse::Constraint,
        ContextResolutionAuthorityUse::Source,
        ContextResolutionAuthorityUse::Target,
        ContextResolutionAuthorityUse::TargetMemory,
    ];
    for authority_use in uses {
        let subject = ContextResolutionExactBinding::new(
            &format!("subject.{}", authority_use.as_str()),
            FINGERPRINT,
        )
        .unwrap();
        let admission = resolver.admit(&mut port, authority_use, &subject).unwrap();
        assert_eq!(admission.authority_use(), authority_use);
        assert_eq!(admission.subject(), &subject);
        assert_eq!(admission.binding_fingerprint(), binding_fingerprint);
    }
    assert_eq!(port.get_calls.len(), 11);
    let subject = ContextResolutionExactBinding::new("subject.root_creation", FINGERPRINT).unwrap();
    let replay = resolver
        .admit(
            &mut port,
            ContextResolutionAuthorityUse::RootCreation,
            &subject,
        )
        .unwrap();
    assert_eq!(replay.subject(), &subject);
    assert_eq!(port.get_calls.len(), 11);
    let changed = ContextResolutionExactBinding::new(
        "subject.root_creation",
        "sha256:bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb",
    )
    .unwrap();
    assert_eq!(
        resolver
            .admit(
                &mut port,
                ContextResolutionAuthorityUse::RootCreation,
                &changed,
            )
            .unwrap_err()
            .kind(),
        ContextResolutionKernelErrorKind::ReplayMismatch
    );
}

#[test]
fn authority_resolver_passes_the_complete_eligible_allow_list_and_refuses_unknown_selection() {
    let (repo, mut port, artifact_fingerprint, _) =
        seed_real_authority_repository_with_credentials(true);
    assert_eq!(port.make_calls, 2);
    port.get_calls.clear();
    port.fixed_sign_count = Some(2);
    let mut resolver = ContextResolutionAuthorityAdmissionResolver::new(
        repo.path(),
        AUTHORITY_KIND_REF,
        AUTHORITY_INSTANCE_ID,
        &artifact_fingerprint,
    )
    .unwrap();
    let subject = ContextResolutionExactBinding::new("subject.multi", FINGERPRINT).unwrap();
    resolver
        .admit(
            &mut port,
            ContextResolutionAuthorityUse::RootCreation,
            &subject,
        )
        .unwrap();
    let request = &port.get_calls[0];
    assert!(request
        .windows(port.credential_id.len())
        .any(|window| window == port.credential_id));
    assert!(request
        .windows(port.added_credential_id.len())
        .any(|window| window == port.added_credential_id));

    let unknown_subject =
        ContextResolutionExactBinding::new("subject.unknown", FINGERPRINT).unwrap();
    port.credential_id = vec![0x77; 32];
    assert_eq!(
        resolver
            .admit(
                &mut port,
                ContextResolutionAuthorityUse::Parent,
                &unknown_subject,
            )
            .unwrap_err()
            .kind(),
        ContextResolutionKernelErrorKind::AuthorityRefused
    );
}

fn rewrite_authority_binding(
    repo: &Path,
    mutate: impl FnOnce(&mut serde_json::Value),
    recompute_semantic_fingerprint: bool,
) -> String {
    let path = repo.join(".handbook/project/context-resolution-authority.json");
    let mut binding: serde_json::Value = serde_json::from_slice(&fs::read(&path).unwrap()).unwrap();
    mutate(&mut binding);
    if recompute_semantic_fingerprint {
        binding
            .as_object_mut()
            .unwrap()
            .remove("binding_fingerprint");
        let fingerprint = DefinitionFingerprint::from_json_value(&binding).unwrap();
        binding["binding_fingerprint"] = json!(fingerprint.as_str());
    }
    let bytes = serde_json::to_vec_pretty(&binding).unwrap();
    fs::write(path, &bytes).unwrap();
    DefinitionFingerprint::from_bytes(&bytes).to_string()
}

#[test]
fn real_authority_path_refuses_stale_malformed_and_uncovered_bindings() {
    let (repo, _port, artifact_fingerprint, _) = seed_real_authority_repository();
    let mut resolver = ContextResolutionAuthorityAdmissionResolver::new(
        repo.path(),
        AUTHORITY_KIND_REF,
        AUTHORITY_INSTANCE_ID,
        &artifact_fingerprint,
    )
    .unwrap();
    let canonical = repo
        .path()
        .join(".handbook/project/context-resolution-authority.json");
    let mut changed_bytes = fs::read(&canonical).unwrap();
    changed_bytes.push(b'\n');
    fs::write(&canonical, &changed_bytes).unwrap();
    let changed_raw = DefinitionFingerprint::from_bytes(&changed_bytes).to_string();
    assert_ne!(changed_raw, artifact_fingerprint);
    let subject = ContextResolutionExactBinding::new("subject.stale", FINGERPRINT).unwrap();
    let mut fresh_port = AuthorityPort::new();
    assert_eq!(
        resolver
            .admit(
                &mut fresh_port,
                ContextResolutionAuthorityUse::RootCreation,
                &subject,
            )
            .unwrap_err()
            .kind(),
        ContextResolutionKernelErrorKind::StaleAuthority
    );

    let (repo, _port, _, _) = seed_real_authority_repository();
    let malformed = rewrite_authority_binding(
        repo.path(),
        |binding| binding["unexpected"] = json!(true),
        true,
    );
    assert!(ContextResolutionAuthorityAdmissionResolver::new(
        repo.path(),
        AUTHORITY_KIND_REF,
        AUTHORITY_INSTANCE_ID,
        &malformed,
    )
    .is_err());

    let (repo, mut port, _, _) = seed_real_authority_repository();
    let uncovered = rewrite_authority_binding(
        repo.path(),
        |binding| {
            binding["authority_mappings"]["root_creation"]["approval_class"] =
                json!("context_resolution_uncovered");
        },
        true,
    );
    let mut resolver = ContextResolutionAuthorityAdmissionResolver::new(
        repo.path(),
        AUTHORITY_KIND_REF,
        AUTHORITY_INSTANCE_ID,
        &uncovered,
    )
    .unwrap();
    let subject = ContextResolutionExactBinding::new("subject.uncovered", FINGERPRINT).unwrap();
    assert_eq!(
        resolver
            .admit(
                &mut port,
                ContextResolutionAuthorityUse::RootCreation,
                &subject,
            )
            .unwrap_err()
            .kind(),
        ContextResolutionKernelErrorKind::AuthorityRefused
    );
}

#[test]
fn real_authority_path_refuses_authenticator_counter_and_cross_resolver_replay() {
    let (repo, mut port, artifact_fingerprint, _) = seed_real_authority_repository();
    let subject = ContextResolutionExactBinding::new("subject.authenticator", FINGERPRINT).unwrap();
    let mut resolver = ContextResolutionAuthorityAdmissionResolver::new(
        repo.path(),
        AUTHORITY_KIND_REF,
        AUTHORITY_INSTANCE_ID,
        &artifact_fingerprint,
    )
    .unwrap();
    port.refused = true;
    assert_eq!(
        resolver
            .admit(
                &mut port,
                ContextResolutionAuthorityUse::RootCreation,
                &subject,
            )
            .unwrap_err()
            .kind(),
        ContextResolutionKernelErrorKind::AuthorityRefused
    );

    port.refused = false;
    port.key = SigningKey::from_slice(&[8_u8; 32]).unwrap();
    assert_eq!(
        resolver
            .admit(
                &mut port,
                ContextResolutionAuthorityUse::RootCreation,
                &subject,
            )
            .unwrap_err()
            .kind(),
        ContextResolutionKernelErrorKind::AuthorityRefused
    );

    port.key = SigningKey::from_slice(&[7_u8; 32]).unwrap();
    port.fixed_sign_count = Some(1);
    resolver
        .admit(
            &mut port,
            ContextResolutionAuthorityUse::RootCreation,
            &subject,
        )
        .unwrap();
    let second = ContextResolutionExactBinding::new("subject.counter", FINGERPRINT).unwrap();
    assert_eq!(
        resolver
            .admit(&mut port, ContextResolutionAuthorityUse::Parent, &second,)
            .unwrap_err()
            .kind(),
        ContextResolutionKernelErrorKind::AuthorityRefused
    );

    let (repo, mut first_port, artifact_fingerprint, _) = seed_real_authority_repository();
    first_port.fixed_sign_count = Some(0);
    let replay_subject =
        ContextResolutionExactBinding::new("subject.cross-resolver", FINGERPRINT).unwrap();
    let mut first = ContextResolutionAuthorityAdmissionResolver::new(
        repo.path(),
        AUTHORITY_KIND_REF,
        AUTHORITY_INSTANCE_ID,
        &artifact_fingerprint,
    )
    .unwrap();
    first
        .admit(
            &mut first_port,
            ContextResolutionAuthorityUse::RootCreation,
            &replay_subject,
        )
        .unwrap();
    let mut replay_port = AuthorityPort::new();
    replay_port.replay_response = first_port.last_response.clone();
    let mut second_resolver = ContextResolutionAuthorityAdmissionResolver::new(
        repo.path(),
        AUTHORITY_KIND_REF,
        AUTHORITY_INSTANCE_ID,
        &artifact_fingerprint,
    )
    .unwrap();
    assert_eq!(
        second_resolver
            .admit(
                &mut replay_port,
                ContextResolutionAuthorityUse::RootCreation,
                &replay_subject,
            )
            .unwrap_err()
            .kind(),
        ContextResolutionKernelErrorKind::AuthorityRefused
    );
}

#[test]
fn envelopes_prove_six_dimension_narrowing_mutation_decisions_and_typed_horizons() {
    let (repo, mut port, artifact_fingerprint, _) = seed_real_authority_repository();
    let (profile, stack) = load_kernel_profile_stack(repo.path());
    let mut resolver = ContextResolutionAuthorityAdmissionResolver::new(
        repo.path(),
        AUTHORITY_KIND_REF,
        AUTHORITY_INSTANCE_ID,
        &artifact_fingerprint,
    )
    .unwrap();
    let root_input = envelope_input(
        "root-behavior",
        &profile,
        &stack,
        "strategic",
        [
            "program",
            "full",
            "long_range",
            "program_policy",
            "strategic",
            "program_gate",
        ],
        None,
        vec![
            ContextResolutionMutationRule::new(
                "root-allow",
                ContextResolutionMutationEffect::Allow,
                "repository_path",
                "crates/**",
            )
            .unwrap(),
            ContextResolutionMutationRule::new(
                "root-deny",
                ContextResolutionMutationEffect::Deny,
                "repository_path",
                "crates/secret/**",
            )
            .unwrap(),
        ],
    );
    let creator = admit_subject(
        &mut resolver,
        &mut port,
        ContextResolutionAuthorityUse::RootCreation,
        root_input.candidate_binding(),
    );
    let root = ContextResolutionEnvelope::resolve_root(&profile, &stack, root_input, &creator, &[])
        .unwrap();

    let broad = [
        "program",
        "full",
        "long_range",
        "program_policy",
        "strategic",
        "program_gate",
    ];
    let narrow = [
        "slice",
        "normal",
        "current_slice",
        "slice_write",
        "coordination",
        "slice_closeout",
    ];
    for dimension in 0..6 {
        let mut values = broad;
        values[dimension] = narrow[dimension];
        let input = envelope_input(
            &format!("child-dimension-{dimension}"),
            &profile,
            &stack,
            "strategic",
            values,
            Some(root.exact_binding().clone()),
            vec![ContextResolutionMutationRule::new(
                &format!("child-allow-{dimension}"),
                ContextResolutionMutationEffect::Allow,
                "repository_path",
                "crates/engine/**",
            )
            .unwrap()],
        );
        let parent = admit_subject(
            &mut resolver,
            &mut port,
            ContextResolutionAuthorityUse::Parent,
            input.candidate_binding(),
        );
        let child =
            ContextResolutionEnvelope::resolve_child(&profile, &stack, &root, input, &parent, &[])
                .unwrap();
        assert_eq!(child.dimensions().values()[dimension], narrow[dimension]);
    }

    let child_input = envelope_input(
        "child-behavior",
        &profile,
        &stack,
        "execution",
        [
            "assigned_unit",
            "normal",
            "immediate",
            "local_write",
            "execution",
            "unit_closeout",
        ],
        Some(root.exact_binding().clone()),
        vec![ContextResolutionMutationRule::new(
            "child-engine",
            ContextResolutionMutationEffect::Allow,
            "repository_path",
            "crates/engine/**",
        )
        .unwrap()],
    );
    let parent = admit_subject(
        &mut resolver,
        &mut port,
        ContextResolutionAuthorityUse::Parent,
        child_input.candidate_binding(),
    );
    let child = ContextResolutionEnvelope::resolve_child(
        &profile,
        &stack,
        &root,
        child_input,
        &parent,
        &[],
    )
    .unwrap();
    assert_eq!(
        child.evaluate_mutation("repository_path", "crates/engine/src/lib.rs"),
        ContextResolutionMutationDecision::Allowed
    );
    assert_eq!(
        child.evaluate_mutation("repository_path", "crates/other/src/lib.rs"),
        ContextResolutionMutationDecision::Denied
    );
    assert_eq!(
        child.evaluate_mutation("repository_path", "crates/secret/key"),
        ContextResolutionMutationDecision::Denied
    );
    assert_eq!(
        child.evaluate_mutation("artifact", "crates/engine/src/lib.rs"),
        ContextResolutionMutationDecision::Indeterminate
    );
    assert_eq!(
        child.authorize_memory("operation").unwrap(),
        ContextResolutionMemoryDecision::Authorized
    );
    assert_eq!(
        child.authorize_memory("coordination").unwrap(),
        ContextResolutionMemoryDecision::PromotionRequired
    );
    assert_eq!(
        child.authorize_validation("observation_only").unwrap(),
        ContextResolutionValidationDecision::Authorized
    );
    assert_eq!(
        child.authorize_validation("slice_closeout").unwrap(),
        ContextResolutionValidationDecision::NotAuthorized
    );
}

#[test]
fn escalation_registry_requires_a_distinct_current_superseding_request() {
    let (repo, mut port, artifact_fingerprint, _) = seed_real_authority_repository();
    let (profile, stack) = load_kernel_profile_stack(repo.path());
    let mut resolver = ContextResolutionAuthorityAdmissionResolver::new(
        repo.path(),
        AUTHORITY_KIND_REF,
        AUTHORITY_INSTANCE_ID,
        &artifact_fingerprint,
    )
    .unwrap();
    let make_root = |id: &str| {
        envelope_input(
            id,
            &profile,
            &stack,
            "execution",
            [
                "assigned_unit",
                "normal",
                "immediate",
                "local_write",
                "execution",
                "unit_closeout",
            ],
            None,
            vec![ContextResolutionMutationRule::new(
                &format!("allow-{id}"),
                ContextResolutionMutationEffect::Allow,
                "repository_path",
                "crates/engine/**",
            )
            .unwrap()],
        )
    };
    let current_input = make_root("escalation-current");
    let creator = admit_subject(
        &mut resolver,
        &mut port,
        ContextResolutionAuthorityUse::RootCreation,
        current_input.candidate_binding(),
    );
    let current =
        ContextResolutionEnvelope::resolve_root(&profile, &stack, current_input, &creator, &[])
            .unwrap();
    let proposed_input = make_root("escalation-proposed");
    let creator = admit_subject(
        &mut resolver,
        &mut port,
        ContextResolutionAuthorityUse::RootCreation,
        proposed_input.candidate_binding(),
    );
    let proposed =
        ContextResolutionEnvelope::resolve_root(&profile, &stack, proposed_input, &creator, &[])
            .unwrap();

    let evidence = ContextResolutionExactBinding::new("evidence.missing", FINGERPRINT).unwrap();
    let first_candidate = current
        .missing_context_candidate(&proposed, vec![evidence.clone()])
        .unwrap();
    let first_trigger = first_candidate.trigger().clone();
    let first = ContextResolutionEscalationRequest::new("request-first", &current, first_candidate)
        .unwrap();
    let first_binding = first.exact_binding().clone();
    let first_requested = admit_subject(
        &mut resolver,
        &mut port,
        ContextResolutionAuthorityUse::Requested,
        &first_binding,
    );
    let first_trigger_admission = admit_subject(
        &mut resolver,
        &mut port,
        ContextResolutionAuthorityUse::TriggerCondition,
        &first_trigger,
    );
    let evidence_admission = admit_subject(
        &mut resolver,
        &mut port,
        ContextResolutionAuthorityUse::Evidence,
        &evidence,
    );

    let second_evidence =
        ContextResolutionExactBinding::new("evidence.authority", FINGERPRINT).unwrap();
    let second_candidate = current
        .missing_authority_candidate(&proposed, vec![second_evidence.clone()])
        .unwrap();
    let second_trigger = second_candidate.trigger().clone();
    let second =
        ContextResolutionEscalationRequest::new("request-second", &current, second_candidate)
            .unwrap();
    let second_binding = second.exact_binding().clone();
    let second_requested = admit_subject(
        &mut resolver,
        &mut port,
        ContextResolutionAuthorityUse::Requested,
        &second_binding,
    );
    let second_trigger_admission = admit_subject(
        &mut resolver,
        &mut port,
        ContextResolutionAuthorityUse::TriggerCondition,
        &second_trigger,
    );
    let second_evidence_admission = admit_subject(
        &mut resolver,
        &mut port,
        ContextResolutionAuthorityUse::Evidence,
        &second_evidence,
    );
    let mut registry = ContextResolutionTransitionRegistry::new();
    assert!(registry
        .admit_escalation_request(
            first,
            &first_requested,
            &first_trigger_admission,
            &[evidence_admission]
        )
        .unwrap());
    assert!(registry
        .admit_escalation_request(
            second,
            &second_requested,
            &second_trigger_admission,
            &[second_evidence_admission]
        )
        .unwrap());

    let decision = ContextResolutionExactBinding::new("decision.supersede", FINGERPRINT).unwrap();
    for (id, superseding) in [
        ("self", first_binding.clone()),
        (
            "unknown",
            ContextResolutionExactBinding::new(
                "context-resolution-escalation-request/unknown",
                FINGERPRINT,
            )
            .unwrap(),
        ),
        (
            "stale",
            ContextResolutionExactBinding::new(
                second_binding.reference(),
                "sha256:bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb",
            )
            .unwrap(),
        ),
    ] {
        let disposition = ContextResolutionEscalationDisposition::new(
            id,
            first_binding.clone(),
            ContextResolutionEscalationOutcome::Superseded,
            decision.clone(),
            None,
            Some(superseding),
        )
        .unwrap();
        let approving = admit_subject(
            &mut resolver,
            &mut port,
            ContextResolutionAuthorityUse::Approving,
            disposition.exact_binding(),
        );
        let decision_admission = admit_subject(
            &mut resolver,
            &mut port,
            ContextResolutionAuthorityUse::Decision,
            &decision,
        );
        assert_eq!(
            registry
                .admit_escalation_disposition(disposition, &approving, &decision_admission)
                .unwrap_err()
                .kind(),
            ContextResolutionKernelErrorKind::StaleAuthority
        );
    }
    let disposition = ContextResolutionEscalationDisposition::new(
        "valid",
        first_binding,
        ContextResolutionEscalationOutcome::Superseded,
        decision.clone(),
        None,
        Some(second_binding),
    )
    .unwrap();
    let approving = admit_subject(
        &mut resolver,
        &mut port,
        ContextResolutionAuthorityUse::Approving,
        disposition.exact_binding(),
    );
    let decision_admission = admit_subject(
        &mut resolver,
        &mut port,
        ContextResolutionAuthorityUse::Decision,
        &decision,
    );
    assert!(registry
        .admit_escalation_disposition(disposition, &approving, &decision_admission)
        .unwrap());
}

#[test]
fn promotion_registry_binds_the_recorded_approving_authority_to_authenticated_authority() {
    let (repo, mut port, artifact_fingerprint, _) = seed_real_authority_repository();
    let (profile, stack) = load_kernel_profile_stack(repo.path());
    let mut resolver = ContextResolutionAuthorityAdmissionResolver::new(
        repo.path(),
        AUTHORITY_KIND_REF,
        AUTHORITY_INSTANCE_ID,
        &artifact_fingerprint,
    )
    .unwrap();
    let source_input = envelope_input(
        "promotion-source",
        &profile,
        &stack,
        "execution",
        [
            "assigned_unit",
            "normal",
            "immediate",
            "local_write",
            "execution",
            "unit_closeout",
        ],
        None,
        vec![ContextResolutionMutationRule::new(
            "promotion-allow",
            ContextResolutionMutationEffect::Allow,
            "repository_path",
            "crates/engine/**",
        )
        .unwrap()],
    );
    let creator = admit_subject(
        &mut resolver,
        &mut port,
        ContextResolutionAuthorityUse::RootCreation,
        source_input.candidate_binding(),
    );
    let source =
        ContextResolutionEnvelope::resolve_root(&profile, &stack, source_input, &creator, &[])
            .unwrap();
    let source_binding = ContextResolutionExactBinding::new("source.input", FINGERPRINT).unwrap();
    let target =
        ContextResolutionSemanticMemoryTarget::new("semantic-memory.example", None).unwrap();
    let target_admission = admit_subject(
        &mut resolver,
        &mut port,
        ContextResolutionAuthorityUse::Target,
        target.candidate_binding(),
    );
    let request = ContextResolutionPromotionRequest::new(
        "promotion-request",
        vec![source_binding.clone()],
        &source,
        &stack,
        "coordination",
        target,
        &target_admission,
        "work.request.owner",
    )
    .unwrap();
    let request_binding = request.exact_binding().clone();
    let decision = ContextResolutionExactBinding::new("decision.promotion", FINGERPRINT).unwrap();
    let dispositions = [
        ("promotion-mismatch", "other.approver", false),
        ("promotion-empty", "", false),
        ("promotion-match", "work.approver", true),
    ]
    .into_iter()
    .map(|(id, authority_ref, succeeds)| {
        (
            ContextResolutionPromotionDisposition::new(
                id,
                &request,
                ContextResolutionPromotionOutcome::Refused,
                decision.clone(),
                Vec::new(),
                authority_ref,
                None,
                None,
            )
            .unwrap(),
            succeeds,
        )
    })
    .collect::<Vec<_>>();
    let requested = admit_subject(
        &mut resolver,
        &mut port,
        ContextResolutionAuthorityUse::Requested,
        &request_binding,
    );
    let source_admission = admit_subject(
        &mut resolver,
        &mut port,
        ContextResolutionAuthorityUse::Source,
        &source_binding,
    );
    let envelope_admission = admit_subject(
        &mut resolver,
        &mut port,
        ContextResolutionAuthorityUse::Source,
        source.exact_binding(),
    );
    let target_memory = admit_subject(
        &mut resolver,
        &mut port,
        ContextResolutionAuthorityUse::TargetMemory,
        &request_binding,
    );
    let mut registry = ContextResolutionTransitionRegistry::new();
    assert!(registry
        .admit_promotion_request(
            request,
            &requested,
            &[source_admission, envelope_admission],
            &target_admission,
            &target_memory,
        )
        .unwrap());
    for (disposition, succeeds) in dispositions {
        let approving = admit_subject(
            &mut resolver,
            &mut port,
            ContextResolutionAuthorityUse::Approving,
            disposition.exact_binding(),
        );
        let decision_admission = admit_subject(
            &mut resolver,
            &mut port,
            ContextResolutionAuthorityUse::Decision,
            &decision,
        );
        let result = registry.admit_promotion_disposition(
            disposition,
            &approving,
            &decision_admission,
            &[],
            &target_memory,
        );
        if succeeds {
            assert!(result.unwrap());
        } else {
            assert_eq!(
                result.unwrap_err().kind(),
                ContextResolutionKernelErrorKind::AuthorityRefused
            );
        }
    }
}

#[test]
fn exact_bindings_dimensions_and_authority_uses_are_closed() {
    let binding = ContextResolutionExactBinding::new("subject.example", FINGERPRINT).unwrap();
    assert_eq!(binding.reference(), "subject.example");
    assert_eq!(binding.fingerprint(), FINGERPRINT);
    assert!(ContextResolutionExactBinding::new("", FINGERPRINT).is_err());
    assert_eq!(
        ContextResolutionAuthorityUse::TargetMemory.as_str(),
        "target_memory"
    );

    let dimensions = ContextResolutionDimensions::new([
        "scope",
        "detail",
        "time",
        "authority",
        "memory",
        "validation",
    ])
    .unwrap();
    assert_eq!(dimensions.values()[4], "memory");
}

#[test]
fn mutation_and_typed_decision_contracts_are_namespaced() {
    let _rule = ContextResolutionMutationRule::new(
        "allow-source",
        ContextResolutionMutationEffect::Allow,
        "repository_path",
        "crates/*/src/**",
    )
    .unwrap();
    assert_eq!(
        ContextResolutionMutationDecision::Denied,
        ContextResolutionMutationDecision::Denied
    );
    assert_eq!(
        ContextResolutionMemoryDecision::PromotionRequired,
        ContextResolutionMemoryDecision::PromotionRequired
    );
    assert_eq!(
        ContextResolutionValidationDecision::NotAuthorized,
        ContextResolutionValidationDecision::NotAuthorized
    );
    assert_eq!(
        ContextResolutionKernelErrorKind::ExpansionRefused,
        ContextResolutionKernelErrorKind::ExpansionRefused
    );
}

#[test]
fn closed_input_and_transition_outcome_matrix_refuses() {
    assert!(ContextResolutionDimensions::new(["", "d", "t", "a", "m", "v"]).is_err());
    for selector in ["/absolute", "a//b", "a/../b", "a/**/b", "https://host/x"] {
        assert!(ContextResolutionMutationRule::new(
            "rule",
            ContextResolutionMutationEffect::Allow,
            "repository_path",
            selector,
        )
        .is_err());
    }
    assert!(ContextResolutionMutationRule::new(
        "rule",
        ContextResolutionMutationEffect::Allow,
        "artifact",
        "a/b",
    )
    .is_err());
    for forbidden in [
        "artifact.example@1.0.0",
        "contract.example@1.0.0",
        "Snapshot/example",
        "Projection/example",
        "gate/example",
        ".handbook/state/durable-memory/example",
    ] {
        assert!(ContextResolutionSemanticMemoryTarget::new(forbidden, None).is_err());
    }
    let target =
        ContextResolutionSemanticMemoryTarget::new("semantic-memory.example", None).unwrap();
    assert_eq!(target.candidate_binding(), target.candidate_binding());

    let request = ContextResolutionExactBinding::new("request/1", FINGERPRINT).unwrap();
    let decision = ContextResolutionExactBinding::new("decision/1", FINGERPRINT).unwrap();
    let replacement = ContextResolutionExactBinding::new("envelope/2", FINGERPRINT).unwrap();
    let superseding = ContextResolutionExactBinding::new("request/2", FINGERPRINT).unwrap();
    assert!(ContextResolutionEscalationDisposition::new(
        "d1",
        request.clone(),
        ContextResolutionEscalationOutcome::Approved,
        decision.clone(),
        None,
        None,
    )
    .is_err());
    assert!(ContextResolutionEscalationDisposition::new(
        "d2",
        request.clone(),
        ContextResolutionEscalationOutcome::Refused,
        decision.clone(),
        Some(replacement.clone()),
        None,
    )
    .is_err());
    assert!(ContextResolutionEscalationDisposition::new(
        "d3",
        request,
        ContextResolutionEscalationOutcome::Superseded,
        decision,
        Some(replacement),
        Some(superseding),
    )
    .is_err());
}

#[test]
fn registry_transaction_family_coexists_with_generic_inventory_without_generic_parsing() {
    let repo = tempfile::tempdir().unwrap();
    let pending = repo
        .path()
        .join(".handbook/state/transactions/registry/opaque.pending");
    fs::create_dir_all(&pending).unwrap();
    fs::write(pending.join("intent.json"), b"not generic json").unwrap();
    artifact_lineage_store::GenericArtifactLineageStoreV1::new(repo.path())
        .recover()
        .unwrap();
}

#[test]
fn generic_journal_authority_excludes_registry_intents() {
    registry_transaction_family_coexists_with_generic_inventory_without_generic_parsing();
}

#[test]
fn unknown_transaction_family_remains_refused() {
    let repo = tempfile::tempdir().unwrap();
    fs::create_dir_all(repo.path().join(".handbook/state/transactions/unknown")).unwrap();
    assert!(
        artifact_lineage_store::GenericArtifactLineageStoreV1::new(repo.path())
            .recover()
            .is_err()
    );
}
