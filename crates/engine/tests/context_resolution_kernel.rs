#![allow(dead_code)]

#[path = "../src/artifact_lineage_store.rs"]
mod artifact_lineage_store;
#[path = "../src/canonical_repo_support.rs"]
mod canonical_repo_support;

use base64::{engine::general_purpose::STANDARD as BASE64_STANDARD, Engine as _};
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
const AUTHORITY_KIND_REF: &str =
    "handbook.artifact-kind.context-resolution-authority-binding@1.0.0";
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

fn tagged_fingerprint(tag: &[u8], bytes: &[u8]) -> String {
    let mut digest = Sha256::new();
    digest.update(tag);
    digest.update((bytes.len() as u64).to_be_bytes());
    digest.update(bytes);
    format!("sha256:{:x}", digest.finalize())
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
                "handbook.schemas.context-resolution-authority-capsule@1.0.0",
            )
            .unwrap(),
        )
        .unwrap();
    assert_eq!(leaves.len(), 1, "unexpected capsule leaves: {leaves:?}");
    let mappings = authority_mappings();
    let mut quorum = mappings
        .as_object()
        .unwrap()
        .values()
        .cloned()
        .collect::<Vec<_>>();
    quorum.push(json!({"approval_class":"context_resolution_authority_publisher","authority_ref":"repository_context_resolution_authority_publisher"}));
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
    assert_eq!(result["status"], "succeeded", "{result:#}");
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
    let semantic_jcs = serde_json_canonicalizer::to_vec(&binding).unwrap();
    let binding_fingerprint =
        tagged_fingerprint(b"handbook.hcm-3.2.semantic-binding.v1\0", &semantic_jcs);
    binding["binding_fingerprint"] = json!(binding_fingerprint.clone());
    let payload_jcs = serde_json_canonicalizer::to_vec(&binding).unwrap();
    let payload_byte_fingerprint =
        tagged_fingerprint(b"handbook.hcm-3.2.payload-jcs.v1\0", &payload_jcs);
    let declaration = json!({
        "schema_id":"handbook.context-resolution-authority-declaration",
        "schema_version":"1.0",
        "codec_id":"handbook.hcm-3.2.context-resolution-authority-binding-jcs",
        "codec_version":"1.0",
        "payload_jcs":String::from_utf8(payload_jcs).unwrap(),
        "payload_byte_fingerprint":payload_byte_fingerprint,
        "semantic_binding_fingerprint":binding_fingerprint,
        "predecessor":{
            "outer_artifact_ref":null,
            "outer_artifact_fingerprint":null,
            "payload_byte_fingerprint":null,
            "semantic_binding_fingerprint":null
        },
        "prior_registry":registry_binding
    });
    let declaration_jcs = serde_json_canonicalizer::to_vec(&declaration).unwrap();
    let declaration_jcs = String::from_utf8(declaration_jcs).unwrap();
    let wrapper = json!({"declaration_jcs":declaration_jcs});
    let intake_request = serde_json::to_vec(&json!({
        "idempotency_key":"hcm32_capsule_intake_0001",
        "acquisition_mode":"express",
        "expected_current_artifact_fingerprint":null,
        "coverage_submissions":[{
            "coverage_id":"context_resolution_authority.declaration_jcs",
            "state":"supplied",
            "source_kind":"user_declaration",
            "value":wrapper["declaration_jcs"],
            "specificity":"exact",
            "confidence":"high",
            "contradiction_refs":[]
        }]
    }))
    .unwrap();
    let intake = handbook_engine::artifact_mutation::ArtifactMutationServiceV1::intake_append(
        repo.path(),
        AUTHORITY_KIND_REF,
        AUTHORITY_INSTANCE_ID,
        &intake_request,
    )
    .unwrap();
    let intake_output = &intake.result.authoritative_outputs[0];
    let preview =
        handbook_engine::artifact_mutation::ArtifactMutationServiceV1::candidate_validate(
            repo.path(),
            AUTHORITY_KIND_REF,
            AUTHORITY_INSTANCE_ID,
            &intake_output.relative_ref,
            &intake_output.fingerprint,
            Some("absent"),
        )
        .unwrap();
    assert_eq!(preview.normalized_content, wrapper);
    let candidate_request = serde_json::to_vec(&json!({
        "idempotency_key":"hcm32_capsule_candidate_0001",
        "intake_record_ref":intake_output.relative_ref,
        "intake_record_fingerprint":intake_output.fingerprint,
        "expected_candidate_fingerprint":preview.candidate_fingerprint
    }))
    .unwrap();
    let candidate =
        handbook_engine::artifact_mutation::ArtifactMutationServiceV1::candidate_append(
            repo.path(),
            AUTHORITY_KIND_REF,
            AUTHORITY_INSTANCE_ID,
            &candidate_request,
        )
        .unwrap();
    let candidate_output = &candidate.result.authoritative_outputs[0];
    let canonical_bytes = handbook_engine::canonical_yaml::canonical_yaml_bytes(&wrapper).unwrap();
    let promoted = DefinitionFingerprint::from_bytes(&canonical_bytes).to_string();
    let outer_hex = promoted.strip_prefix("sha256:").unwrap();
    quorum.push(json!({
        "approval_class":"registry_admin",
        "authority_ref":"repository_registry_admin"
    }));
    quorum.push(json!({
        "approval_class":"context_resolution_authority_publication",
        "authority_ref":format!("context-resolution-authority/{outer_hex}")
    }));
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
    let publisher_credential_id_hash = DefinitionFingerprint::from_bytes(&[0x32; 32]).to_string();
    let update = ApproverAdminRequestV1::from_json_value(json!({
        "schema_id":"handbook.approver-admin-request",
        "schema_version":"1.0",
        "operation":"update_mapping",
        "operation_id":format!("hcm32-authorize-{outer_hex}"),
        "repository_identity_fingerprint":identity.repository_identity_fingerprint(),
        "expected_registry_state_fingerprint":final_result["result_registry_state_fingerprint"],
        "expected_transition_fingerprint":final_result["transition_fingerprint"],
        "credential_id_hash":publisher_credential_id_hash,
        "approval_mappings":quorum
    }))
    .unwrap();
    let publication = registry
        .update_approver_mapping(update)
        .to_json_value()
        .unwrap();
    assert_eq!(publication["status"], "succeeded", "{publication:#}");
    let assertion: serde_json::Value = serde_json::from_slice(
        &fs::read(
            repo.path()
                .join(".handbook/state")
                .join(publication["assertion_ref"].as_str().unwrap()),
        )
        .unwrap(),
    )
    .unwrap();
    let response: serde_json::Value = serde_json::from_slice(
        &fs::read(
            repo.path()
                .join(".handbook/state")
                .join(assertion["decoded_response_ref"].as_str().unwrap()),
        )
        .unwrap(),
    )
    .unwrap();
    let publisher_authority = json!({
        "schema_id":"handbook.context-resolution-publisher-authority",
        "schema_version":"1.0",
        "result_registry_state_ref":publication["result_registry_state_ref"],
        "result_registry_state_fingerprint":publication["result_registry_state_fingerprint"],
        "result_transition_ref":publication["transition_ref"],
        "result_transition_fingerprint":publication["transition_fingerprint"],
        "publisher_credential_id_hash":publisher_credential_id_hash,
        "challenge_jcs_base64":assertion["challenge_jcs_base64"],
        "raw_assertion_response_base64":response["raw_response_base64"]
    });
    let port = registry.into_port();
    assert_eq!(
        BASE64_STANDARD
            .decode(
                publisher_authority["raw_assertion_response_base64"]
                    .as_str()
                    .unwrap()
            )
            .unwrap(),
        port.last_response.as_ref().unwrap().as_slice()
    );
    let promotion_request = serde_json::to_vec(&json!({
        "idempotency_key":format!("hcm32crpub_{outer_hex}"),
        "candidate_ref":candidate_output.relative_ref,
        "candidate_fingerprint":candidate_output.fingerprint,
        "expected_current_artifact_fingerprint":null,
        "publisher_authority":publisher_authority
    }))
    .unwrap();
    handbook_engine::artifact_mutation::ArtifactMutationServiceV1::promote(
        repo.path(),
        AUTHORITY_KIND_REF,
        AUTHORITY_INSTANCE_ID,
        &promotion_request,
    )
    .unwrap();
    (repo, port, promoted, binding_fingerprint)
}

fn seed_real_authority_repository() -> (tempfile::TempDir, AuthorityPort, String, String) {
    seed_real_authority_repository_with_credentials(false)
}

fn prepare_authority_replacement(
    repo: &Path,
    port: AuthorityPort,
    sequence: u32,
    predecessor_outer_fingerprint: &str,
    predecessor_semantic_fingerprint: &str,
) -> (AuthorityPort, String, String, Vec<u8>) {
    let current_bytes =
        fs::read(repo.join(".handbook/project/context-resolution-authority.yaml")).unwrap();
    let current_wrapper =
        handbook_engine::canonical_yaml::parse_canonical_yaml(&current_bytes).unwrap();
    let current_declaration: serde_json::Value =
        serde_json::from_str(current_wrapper["declaration_jcs"].as_str().unwrap()).unwrap();
    let predecessor_payload_fingerprint = current_declaration["payload_byte_fingerprint"]
        .as_str()
        .unwrap()
        .to_owned();

    let (repository_identity, registry_binding, mut publisher_mappings) = {
        let publisher = fs::read_dir(repo.join(".handbook/state/transactions/artifact-promotions"))
            .unwrap()
            .map(|entry| entry.unwrap().path())
            .filter(|path| {
                path.extension()
                    .is_some_and(|extension| extension == "committed")
            })
            .find_map(|path| {
                let intent: serde_json::Value =
                    serde_json::from_slice(&fs::read(path.join("intent.json")).ok()?).ok()?;
                intent["outputs"]
                    .as_array()?
                    .iter()
                    .any(|output| {
                        output["bytes_sha256"].as_str() == Some(predecessor_outer_fingerprint)
                    })
                    .then(|| intent["request_subject"]["publisher_authority"].clone())
            })
            .expect("committed predecessor publisher authority");
        let state_ref = publisher["result_registry_state_ref"].as_str().unwrap();
        let state: serde_json::Value = serde_json::from_slice(
            &fs::read(repo.join(".handbook/state").join(state_ref)).unwrap(),
        )
        .unwrap();
        let repository_identity = state["repository_identity_fingerprint"]
            .as_str()
            .unwrap()
            .to_owned();
        let credential = state["credentials"]
            .as_array()
            .unwrap()
            .iter()
            .find(|row| {
                row["credential_id_hash"] == DefinitionFingerprint::from_bytes(&[0x32; 32]).as_str()
            })
            .unwrap();
        (
            repository_identity,
            json!({
                "state_ref":publisher["result_registry_state_ref"],
                "state_fingerprint":publisher["result_registry_state_fingerprint"],
                "head_transition_ref":publisher["result_transition_ref"],
                "head_transition_fingerprint":publisher["result_transition_fingerprint"]
            }),
            credential["approval_mappings"].as_array().unwrap().clone(),
        )
    };
    let selection =
        handbook_engine::artifact_intake_registry::RepositoryProfileSelectionV1::from_json_bytes(
            &fs::read(repo.join(".handbook/profile-selection.json")).unwrap(),
        )
        .unwrap();
    let profile = resolve_profile_selection(repo, selection.profile_request()).unwrap();
    let mut binding = json!({
        "schema_id":"handbook.context-resolution-authority-binding",
        "schema_version":"1.0",
        "binding_id":"handbook.context-resolution-authority.fixture",
        "binding_version":format!("1.0.{sequence}"),
        "repository_identity_fingerprint":repository_identity,
        "resolved_profile":{"ref":profile.exact_ref().as_str(),"fingerprint":profile.resolved_profile_fingerprint().as_str()},
        "resolution_stack":{"ref":profile.context_resolution().exact_ref().as_str(),"fingerprint":profile.context_resolution().definition_fingerprint().as_str()},
        "approver_registry":registry_binding,
        "authority_mappings":authority_mappings(),
        "candidate_mappings":candidate_mappings(),
        "extensions":{}
    });
    let semantic_jcs = serde_json_canonicalizer::to_vec(&binding).unwrap();
    let semantic_fingerprint =
        tagged_fingerprint(b"handbook.hcm-3.2.semantic-binding.v1\0", &semantic_jcs);
    binding["binding_fingerprint"] = json!(semantic_fingerprint.clone());
    let payload_jcs = serde_json_canonicalizer::to_vec(&binding).unwrap();
    let payload_fingerprint =
        tagged_fingerprint(b"handbook.hcm-3.2.payload-jcs.v1\0", &payload_jcs);
    let declaration = json!({
        "schema_id":"handbook.context-resolution-authority-declaration",
        "schema_version":"1.0",
        "codec_id":"handbook.hcm-3.2.context-resolution-authority-binding-jcs",
        "codec_version":"1.0",
        "payload_jcs":String::from_utf8(payload_jcs).unwrap(),
        "payload_byte_fingerprint":payload_fingerprint,
        "semantic_binding_fingerprint":semantic_fingerprint,
        "predecessor":{
            "outer_artifact_ref":".handbook/project/context-resolution-authority.yaml",
            "outer_artifact_fingerprint":predecessor_outer_fingerprint,
            "payload_byte_fingerprint":predecessor_payload_fingerprint,
            "semantic_binding_fingerprint":predecessor_semantic_fingerprint
        },
        "prior_registry":registry_binding
    });
    let wrapper = json!({
        "declaration_jcs":String::from_utf8(
            serde_json_canonicalizer::to_vec(&declaration).unwrap()
        ).unwrap()
    });
    let intake_request = serde_json::to_vec(&json!({
        "idempotency_key":format!("hcm32_capsule_intake_{sequence:04}"),
        "acquisition_mode":"express",
        "expected_current_artifact_fingerprint":predecessor_outer_fingerprint,
        "coverage_submissions":[{
            "coverage_id":"context_resolution_authority.declaration_jcs",
            "state":"supplied",
            "source_kind":"user_declaration",
            "value":wrapper["declaration_jcs"],
            "specificity":"exact",
            "confidence":"high",
            "contradiction_refs":[]
        }]
    }))
    .unwrap();
    let intake = handbook_engine::artifact_mutation::ArtifactMutationServiceV1::intake_append(
        repo,
        AUTHORITY_KIND_REF,
        AUTHORITY_INSTANCE_ID,
        &intake_request,
    )
    .unwrap();
    let intake_output = &intake.result.authoritative_outputs[0];
    let preview =
        handbook_engine::artifact_mutation::ArtifactMutationServiceV1::candidate_validate(
            repo,
            AUTHORITY_KIND_REF,
            AUTHORITY_INSTANCE_ID,
            &intake_output.relative_ref,
            &intake_output.fingerprint,
            Some(predecessor_outer_fingerprint),
        )
        .unwrap();
    let candidate_request = serde_json::to_vec(&json!({
        "idempotency_key":format!("hcm32_capsule_candidate_{sequence:04}"),
        "intake_record_ref":intake_output.relative_ref,
        "intake_record_fingerprint":intake_output.fingerprint,
        "expected_candidate_fingerprint":preview.candidate_fingerprint
    }))
    .unwrap();
    let candidate =
        handbook_engine::artifact_mutation::ArtifactMutationServiceV1::candidate_append(
            repo,
            AUTHORITY_KIND_REF,
            AUTHORITY_INSTANCE_ID,
            &candidate_request,
        )
        .unwrap();
    let candidate_output = &candidate.result.authoritative_outputs[0];
    let outer = DefinitionFingerprint::from_bytes(
        &handbook_engine::canonical_yaml::canonical_yaml_bytes(&wrapper).unwrap(),
    )
    .to_string();
    let outer_hex = outer.strip_prefix("sha256:").unwrap();
    publisher_mappings.push(json!({
        "approval_class":"context_resolution_authority_publication",
        "authority_ref":format!("context-resolution-authority/{outer_hex}")
    }));
    publisher_mappings.sort_by(|left, right| {
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
    let publisher_credential_id_hash = DefinitionFingerprint::from_bytes(&[0x32; 32]).to_string();
    let update = ApproverAdminRequestV1::from_json_value(json!({
        "schema_id":"handbook.approver-admin-request",
        "schema_version":"1.0",
        "operation":"update_mapping",
        "operation_id":format!("hcm32-authorize-{outer_hex}"),
        "repository_identity_fingerprint":repository_identity,
        "expected_registry_state_fingerprint":registry_binding["state_fingerprint"],
        "expected_transition_fingerprint":registry_binding["head_transition_fingerprint"],
        "credential_id_hash":publisher_credential_id_hash,
        "approval_mappings":publisher_mappings
    }))
    .unwrap();
    let mut registry = ApproverRegistryServiceV1::for_repository(repo, port);
    let publication = registry
        .update_approver_mapping(update)
        .to_json_value()
        .unwrap();
    assert_eq!(publication["status"], "succeeded", "{publication:#}");
    let assertion: serde_json::Value = serde_json::from_slice(
        &fs::read(
            repo.join(".handbook/state")
                .join(publication["assertion_ref"].as_str().unwrap()),
        )
        .unwrap(),
    )
    .unwrap();
    let response: serde_json::Value = serde_json::from_slice(
        &fs::read(
            repo.join(".handbook/state")
                .join(assertion["decoded_response_ref"].as_str().unwrap()),
        )
        .unwrap(),
    )
    .unwrap();
    let publisher_authority = json!({
        "schema_id":"handbook.context-resolution-publisher-authority",
        "schema_version":"1.0",
        "result_registry_state_ref":publication["result_registry_state_ref"],
        "result_registry_state_fingerprint":publication["result_registry_state_fingerprint"],
        "result_transition_ref":publication["transition_ref"],
        "result_transition_fingerprint":publication["transition_fingerprint"],
        "publisher_credential_id_hash":publisher_credential_id_hash,
        "challenge_jcs_base64":assertion["challenge_jcs_base64"],
        "raw_assertion_response_base64":response["raw_response_base64"]
    });
    let port = registry.into_port();
    let promotion_request = serde_json::to_vec(&json!({
        "idempotency_key":format!("hcm32crpub_{outer_hex}"),
        "candidate_ref":candidate_output.relative_ref,
        "candidate_fingerprint":candidate_output.fingerprint,
        "expected_current_artifact_fingerprint":predecessor_outer_fingerprint,
        "publisher_authority":publisher_authority
    }))
    .unwrap();
    (port, outer, semantic_fingerprint, promotion_request)
}

fn snapshot_files(root: &Path) -> Vec<(String, Vec<u8>)> {
    fn visit(base: &Path, path: &Path, rows: &mut Vec<(String, Vec<u8>)>) {
        let mut entries = fs::read_dir(path)
            .unwrap()
            .map(|entry| entry.unwrap().path())
            .collect::<Vec<_>>();
        entries.sort();
        for entry in entries {
            if entry.is_dir() {
                visit(base, &entry, rows);
            } else {
                rows.push((
                    entry
                        .strip_prefix(base)
                        .unwrap()
                        .to_string_lossy()
                        .replace('\\', "/"),
                    fs::read(entry).unwrap(),
                ));
            }
        }
    }

    let mut rows = Vec::new();
    visit(root, root, &mut rows);
    rows
}

fn promote_authority_replacement(
    repo: &Path,
    port: AuthorityPort,
    sequence: u32,
    predecessor_outer_fingerprint: &str,
    predecessor_semantic_fingerprint: &str,
) -> (AuthorityPort, String, String) {
    let (port, outer, semantic_fingerprint, promotion_request) = prepare_authority_replacement(
        repo,
        port,
        sequence,
        predecessor_outer_fingerprint,
        predecessor_semantic_fingerprint,
    );
    handbook_engine::artifact_mutation::ArtifactMutationServiceV1::promote(
        repo,
        AUTHORITY_KIND_REF,
        AUTHORITY_INSTANCE_ID,
        &promotion_request,
    )
    .unwrap();
    (port, outer, semantic_fingerprint)
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
    assert_eq!(port.get_calls.len(), 12);
    let subject = ContextResolutionExactBinding::new("subject.root_creation", FINGERPRINT).unwrap();
    let replay = resolver
        .admit(
            &mut port,
            ContextResolutionAuthorityUse::RootCreation,
            &subject,
        )
        .unwrap();
    assert_eq!(replay.subject(), &subject);
    assert_eq!(port.get_calls.len(), 12);
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
fn replacement_chain_retains_unique_historical_witnesses_and_only_o3_is_operational() {
    let (repo, mut port, o1, s1) = seed_real_authority_repository();
    let (profile, stack) = load_kernel_profile_stack(repo.path());
    let old_input = envelope_input(
        "o1-before-replacement",
        &profile,
        &stack,
        "coordination",
        [
            "execution",
            "work",
            "reversible",
            "repository",
            "brief",
            "direct",
        ],
        None,
        Vec::new(),
    );
    let mut o1_resolver = ContextResolutionAuthorityAdmissionResolver::new(
        repo.path(),
        AUTHORITY_KIND_REF,
        AUTHORITY_INSTANCE_ID,
        &o1,
    )
    .unwrap();
    let o1_admission = o1_resolver
        .admit(
            &mut port,
            ContextResolutionAuthorityUse::RootCreation,
            old_input.candidate_binding(),
        )
        .unwrap();
    let (port, o2, s2) = promote_authority_replacement(repo.path(), port, 2, &o1, &s1);
    let (mut port, o3, s3) = promote_authority_replacement(repo.path(), port, 3, &o2, &s2);

    let mut current = ContextResolutionAuthorityAdmissionResolver::new(
        repo.path(),
        AUTHORITY_KIND_REF,
        AUTHORITY_INSTANCE_ID,
        &o3,
    )
    .unwrap();
    let subject = ContextResolutionExactBinding::new("subject.o3", FINGERPRINT).unwrap();
    let admission = current
        .admit(
            &mut port,
            ContextResolutionAuthorityUse::RootCreation,
            &subject,
        )
        .unwrap();
    assert_eq!(admission.binding_fingerprint(), s3);
    assert_eq!(
        ContextResolutionEnvelope::resolve_root(&profile, &stack, old_input, &o1_admission, &[],)
            .unwrap_err()
            .kind(),
        ContextResolutionKernelErrorKind::StaleAuthority
    );

    let mut displaced_fingerprints = fs::read_dir(repo.path().join(".handbook/project"))
        .unwrap()
        .map(|entry| entry.unwrap().path())
        .filter(|path| {
            path.extension()
                .is_some_and(|extension| extension == "displaced")
        })
        .map(|path| DefinitionFingerprint::from_bytes(&fs::read(path).unwrap()).to_string())
        .collect::<Vec<_>>();
    displaced_fingerprints.sort();
    let mut expected = vec![o1, o2];
    expected.sort();
    assert_eq!(displaced_fingerprints, expected);
}

#[test]
fn authorized_without_journal_is_quarantined_and_exact_retry_closes_it() {
    let (repo, port, o1, s1) = seed_real_authority_repository();
    let (mut port, o2, _, promotion_request) =
        prepare_authority_replacement(repo.path(), port, 2, &o1, &s1);
    let promotion: serde_json::Value = serde_json::from_slice(&promotion_request).unwrap();
    let transition_fingerprint = promotion["publisher_authority"]["result_transition_fingerprint"]
        .as_str()
        .unwrap();
    let failure = match ContextResolutionAuthorityAdmissionResolver::new(
        repo.path(),
        AUTHORITY_KIND_REF,
        AUTHORITY_INSTANCE_ID,
        &o1,
    ) {
        Ok(_) => panic!("cold O1 input must discover and quarantine the live H2-for-O2 gap"),
        Err(error) => error,
    };
    assert_eq!(
        failure.kind(),
        ContextResolutionKernelErrorKind::AuthorityRefused
    );
    let quarantine_path = repo
        .path()
        .join(".handbook/state/context-resolution-authority/quarantine")
        .join(format!(
            "quarantine_{}.json",
            transition_fingerprint.strip_prefix("sha256:").unwrap()
        ));
    let open: serde_json::Value =
        serde_json::from_slice(&fs::read(&quarantine_path).unwrap()).unwrap();
    assert_eq!(open.as_object().unwrap().len(), 19);
    assert_eq!(open["phase"], "authorized_without_generic_intent");
    assert_eq!(open["reason"], "retry_available");
    assert_eq!(open["resolution"], "open");
    assert_eq!(open["outer_artifact_fingerprint"], o2);
    assert!(open["candidate_ref"].is_string());
    let open_bytes = fs::read(&quarantine_path).unwrap();
    assert!(ContextResolutionAuthorityAdmissionResolver::new(
        repo.path(),
        AUTHORITY_KIND_REF,
        AUTHORITY_INSTANCE_ID,
        &o1,
    )
    .is_err());
    assert_eq!(fs::read(&quarantine_path).unwrap(), open_bytes);
    let unknown = quarantine_path.parent().unwrap().join("unexpected.json");
    fs::write(&unknown, b"{}\n").unwrap();
    assert!(ContextResolutionAuthorityAdmissionResolver::new(
        repo.path(),
        AUTHORITY_KIND_REF,
        AUTHORITY_INSTANCE_ID,
        &o1,
    )
    .is_err());
    fs::remove_file(unknown).unwrap();

    let execution = handbook_engine::artifact_mutation::ArtifactMutationServiceV1::promote(
        repo.path(),
        AUTHORITY_KIND_REF,
        AUTHORITY_INSTANCE_ID,
        &promotion_request,
    )
    .unwrap();
    let completed_path = quarantine_path.with_file_name(format!(
        "{}.committed.json",
        quarantine_path.file_stem().unwrap().to_string_lossy()
    ));
    let completed: serde_json::Value =
        serde_json::from_slice(&fs::read(&completed_path).unwrap()).unwrap();
    assert_eq!(completed["resolution"], "committed");
    assert_eq!(
        completed["journal_transaction_id"],
        execution.result.transaction_id
    );
    assert_eq!(
        completed["result_transaction_fingerprint"],
        execution
            .result
            .internal_transaction_evidence_fingerprint
            .as_deref()
            .unwrap()
    );
    let completed_bytes = fs::read(&completed_path).unwrap();
    let replay = handbook_engine::artifact_mutation::ArtifactMutationServiceV1::promote(
        repo.path(),
        AUTHORITY_KIND_REF,
        AUTHORITY_INSTANCE_ID,
        &promotion_request,
    )
    .expect("exact completed retry is idempotent");
    assert_eq!(
        replay.result.transaction_id,
        execution.result.transaction_id
    );
    assert_eq!(fs::read(&completed_path).unwrap(), completed_bytes);
    assert_eq!(fs::read(&quarantine_path).unwrap(), open_bytes);

    let mut resolver = ContextResolutionAuthorityAdmissionResolver::new(
        repo.path(),
        AUTHORITY_KIND_REF,
        AUTHORITY_INSTANCE_ID,
        &o2,
    )
    .unwrap();
    let subject = ContextResolutionExactBinding::new("subject.recovered-o2", FINGERPRINT).unwrap();
    resolver
        .admit(
            &mut port,
            ContextResolutionAuthorityUse::RootCreation,
            &subject,
        )
        .unwrap();
}

#[test]
fn matching_invalid_recovery_candidate_refuses_in_both_lexical_orders() {
    let (repo, port, o1, s1) = seed_real_authority_repository();
    let (_, authorized_outer, _, promotion_request) =
        prepare_authority_replacement(repo.path(), port, 2, &o1, &s1);
    let promotion: serde_json::Value = serde_json::from_slice(&promotion_request).unwrap();
    let valid_candidate_path = repo
        .path()
        .join(promotion["candidate_ref"].as_str().unwrap());
    let candidate_root = valid_candidate_path.parent().unwrap();
    let valid_candidate: serde_json::Value =
        serde_json::from_slice(&fs::read(&valid_candidate_path).unwrap()).unwrap();
    let store = artifact_lineage_store::GenericArtifactLineageStoreV1::new(repo.path());
    store
        .read_committed_authoritative_for_currentness(
            promotion["candidate_ref"].as_str().unwrap(),
            promotion["candidate_fingerprint"].as_str().unwrap(),
        )
        .expect("the matching recovery candidate begins committed and valid");

    for (invalid_sorts_before, hex) in [(true, "0".repeat(64)), (false, "f".repeat(64))] {
        let invalid_candidate_path = candidate_root.join(format!("candidate_{hex}.json"));
        assert_eq!(
            invalid_candidate_path < valid_candidate_path,
            invalid_sorts_before
        );
        let mut invalid_candidate = valid_candidate.clone();
        invalid_candidate["candidate_id"] = json!(format!("candidate_{hex}"));
        invalid_candidate["candidate_fingerprint"] = json!(format!("sha256:{hex}"));
        let mut invalid_candidate_bytes =
            serde_json_canonicalizer::to_vec(&invalid_candidate).unwrap();
        invalid_candidate_bytes.push(b'\n');
        fs::write(&invalid_candidate_path, invalid_candidate_bytes).unwrap();

        let invalid_content: serde_json::Value = serde_json::from_slice(
            &fs::read(
                repo.path().join(
                    invalid_candidate["normalized_content_ref"]
                        .as_str()
                        .unwrap(),
                ),
            )
            .unwrap(),
        )
        .unwrap();
        assert_eq!(
            DefinitionFingerprint::from_bytes(
                &handbook_engine::canonical_yaml::canonical_yaml_bytes(&invalid_content).unwrap()
            )
            .as_str(),
            authorized_outer
        );

        let error = store
            .read_committed_authoritative_for_currentness(
                promotion["candidate_ref"].as_str().unwrap(),
                promotion["candidate_fingerprint"].as_str().unwrap(),
            )
            .expect_err("uncited matching evidence must fail closed before classification");
        assert_eq!(
            error.detail(),
            "artifact inventory contains a record uncited by any journal chain"
        );
        fs::remove_file(&invalid_candidate_path).unwrap();
        store
            .read_committed_authoritative_for_currentness(
                promotion["candidate_ref"].as_str().unwrap(),
                promotion["candidate_fingerprint"].as_str().unwrap(),
            )
            .expect("removing uncited evidence restores exact committed authority");
    }
}

#[test]
fn different_valid_quarantine_candidate_refuses_before_publication_mutation() {
    let (repo, port, o1, s1) = seed_real_authority_repository();
    let (_, _, _, promotion_request) =
        prepare_authority_replacement(repo.path(), port, 2, &o1, &s1);
    assert!(ContextResolutionAuthorityAdmissionResolver::new(
        repo.path(),
        AUTHORITY_KIND_REF,
        AUTHORITY_INSTANCE_ID,
        &o1,
    )
    .is_err());
    let promotion: serde_json::Value = serde_json::from_slice(&promotion_request).unwrap();
    let canonical_path = repo
        .path()
        .join(".handbook/project/context-resolution-authority.yaml");
    let quarantine_root = repo
        .path()
        .join(".handbook/state/context-resolution-authority/quarantine");
    let anchor_root = quarantine_root.parent().unwrap().join("quarantine-anchors");
    let transaction_root = repo.path().join(".handbook/state/transactions");
    let promotion_root = repo
        .path()
        .join(".handbook/state/transactions/artifact-promotions");
    let quarantine_path = fs::read_dir(&quarantine_root)
        .unwrap()
        .next()
        .unwrap()
        .unwrap()
        .path();
    let quarantine_bytes_before = fs::read(&quarantine_path).unwrap();
    let quarantine: serde_json::Value = serde_json::from_slice(&quarantine_bytes_before).unwrap();
    assert_eq!(quarantine["reason"], "retry_available");
    assert_eq!(quarantine["candidate_ref"], promotion["candidate_ref"]);
    assert_eq!(
        quarantine["candidate_fingerprint"],
        promotion["candidate_fingerprint"]
    );
    assert_eq!(quarantine["idempotency_key"], promotion["idempotency_key"]);
    let anchor_path = anchor_root.join(quarantine_path.file_name().unwrap());
    let anchor_bytes_before = fs::read(&anchor_path).unwrap();
    let recorded_ref = promotion["candidate_ref"].as_str().unwrap();
    let different_candidate_path = fs::read_dir(
        repo.path()
            .join(".handbook/evidence/artifacts/context_resolution_authority/candidates"),
    )
    .unwrap()
    .map(|entry| entry.unwrap().path())
    .find(|path| {
        path.strip_prefix(repo.path())
            .unwrap()
            .to_string_lossy()
            .replace('\\', "/")
            != recorded_ref
    })
    .unwrap();
    let different_candidate: serde_json::Value =
        serde_json::from_slice(&fs::read(&different_candidate_path).unwrap()).unwrap();
    let different_candidate_ref = different_candidate_path
        .strip_prefix(repo.path())
        .unwrap()
        .to_string_lossy()
        .replace('\\', "/");
    let different_candidate_fingerprint = different_candidate["candidate_fingerprint"]
        .as_str()
        .unwrap()
        .to_owned();
    let different_normalized_content: serde_json::Value = serde_json::from_slice(
        &fs::read(
            repo.path().join(
                different_candidate["normalized_content_ref"]
                    .as_str()
                    .unwrap(),
            ),
        )
        .unwrap(),
    )
    .unwrap();
    let different_outer_fingerprint = DefinitionFingerprint::from_bytes(
        &handbook_engine::canonical_yaml::canonical_yaml_bytes(&different_normalized_content)
            .unwrap(),
    )
    .to_string();
    let different_idempotency_key = format!(
        "hcm32crpub_{}",
        different_outer_fingerprint.strip_prefix("sha256:").unwrap()
    );
    assert_ne!(different_candidate_ref, recorded_ref);
    assert_ne!(
        different_candidate_fingerprint,
        promotion["candidate_fingerprint"].as_str().unwrap()
    );
    assert_ne!(
        different_idempotency_key,
        promotion["idempotency_key"].as_str().unwrap()
    );
    let scratch_path = quarantine_path.with_file_name(format!(
        "{}.committed.writing",
        quarantine_path.file_stem().unwrap().to_string_lossy()
    ));
    fs::write(&scratch_path, b"{partial-closeout").unwrap();
    let scratch_bytes_before = fs::read(&scratch_path).unwrap();
    let completion_path = quarantine_path.with_file_name(format!(
        "{}.committed.json",
        quarantine_path.file_stem().unwrap().to_string_lossy()
    ));
    let completion_bytes_before = fs::read(&completion_path).ok();
    let canonical_bytes_before = fs::read(&canonical_path).unwrap();
    let quarantine_inventory_before = snapshot_files(&quarantine_root);
    let anchor_inventory_before = snapshot_files(&anchor_root);
    let transaction_inventory_before = snapshot_files(&transaction_root);
    let promotion_inventory_before = snapshot_files(&promotion_root);

    let refusal = artifact_lineage_store::GenericArtifactLineageStoreV1::new(repo.path())
        .require_hcm_publication_quarantine_candidate_during_evaluation(
            promotion["publisher_authority"]["result_transition_fingerprint"]
                .as_str()
                .unwrap(),
            &different_candidate_ref,
            &different_candidate_fingerprint,
            &different_idempotency_key,
        );

    assert!(refusal.is_err());
    assert_eq!(
        fs::read(&scratch_path).ok(),
        Some(scratch_bytes_before.clone())
    );
    assert_eq!(fs::read(&quarantine_path).unwrap(), quarantine_bytes_before);
    assert_eq!(fs::read(&anchor_path).unwrap(), anchor_bytes_before);
    assert_eq!(fs::read(&completion_path).ok(), completion_bytes_before);
    assert_eq!(fs::read(&canonical_path).unwrap(), canonical_bytes_before);
    assert_eq!(
        snapshot_files(&quarantine_root),
        quarantine_inventory_before
    );
    assert_eq!(snapshot_files(&anchor_root), anchor_inventory_before);
    assert_eq!(
        snapshot_files(&transaction_root),
        transaction_inventory_before
    );
    assert_eq!(snapshot_files(&promotion_root), promotion_inventory_before);
}

#[test]
fn quarantine_anchor_only_and_committed_only_states_fail_closed() {
    let (repo, port, o1, s1) = seed_real_authority_repository();
    let (_, _, _, promotion_request) =
        prepare_authority_replacement(repo.path(), port, 2, &o1, &s1);
    assert!(ContextResolutionAuthorityAdmissionResolver::new(
        repo.path(),
        AUTHORITY_KIND_REF,
        AUTHORITY_INSTANCE_ID,
        &o1,
    )
    .is_err());
    let promotion: serde_json::Value = serde_json::from_slice(&promotion_request).unwrap();
    let transition_fingerprint = promotion["publisher_authority"]["result_transition_fingerprint"]
        .as_str()
        .unwrap();
    let quarantine_root = repo
        .path()
        .join(".handbook/state/context-resolution-authority/quarantine");
    let open_path = fs::read_dir(&quarantine_root)
        .unwrap()
        .next()
        .unwrap()
        .unwrap()
        .path();
    let open_record: serde_json::Value =
        serde_json::from_slice(&fs::read(&open_path).unwrap()).unwrap();
    let anchor_path = quarantine_root
        .parent()
        .unwrap()
        .join("quarantine-anchors")
        .join(open_path.file_name().unwrap());
    assert!(anchor_path.exists());
    fs::remove_file(&open_path).unwrap();
    let anchor_only_before = snapshot_files(quarantine_root.parent().unwrap());
    let store = artifact_lineage_store::GenericArtifactLineageStoreV1::new(repo.path());
    assert!(store
        .require_hcm_publication_quarantine_candidate_during_evaluation(
            transition_fingerprint,
            promotion["candidate_ref"].as_str().unwrap(),
            promotion["candidate_fingerprint"].as_str().unwrap(),
            promotion["idempotency_key"].as_str().unwrap(),
        )
        .is_err());
    assert!(store
        .complete_hcm_publication_quarantine(
            transition_fingerprint,
            promotion["candidate_ref"].as_str().unwrap(),
            promotion["candidate_fingerprint"].as_str().unwrap(),
            promotion["idempotency_key"].as_str().unwrap(),
        )
        .is_err());
    assert!(store
        .reconcile_hcm_publication_quarantine(transition_fingerprint)
        .is_err());
    assert_eq!(
        snapshot_files(quarantine_root.parent().unwrap()),
        anchor_only_before
    );

    store
        .record_hcm_publication_quarantine(transition_fingerprint, &open_record)
        .expect("an exact retry reconstructs the anchor-bound open record");
    assert_eq!(
        serde_json::from_slice::<serde_json::Value>(&fs::read(&open_path).unwrap()).unwrap(),
        open_record
    );
    handbook_engine::artifact_mutation::ArtifactMutationServiceV1::promote(
        repo.path(),
        AUTHORITY_KIND_REF,
        AUTHORITY_INSTANCE_ID,
        &promotion_request,
    )
    .unwrap();
    let committed_path = open_path.with_file_name(format!(
        "{}.committed.json",
        open_path.file_stem().unwrap().to_string_lossy()
    ));
    assert!(committed_path.exists());
    fs::remove_file(&open_path).unwrap();
    let committed_only_before = snapshot_files(quarantine_root.parent().unwrap());
    assert!(store
        .require_hcm_publication_quarantine_candidate_during_evaluation(
            transition_fingerprint,
            promotion["candidate_ref"].as_str().unwrap(),
            promotion["candidate_fingerprint"].as_str().unwrap(),
            promotion["idempotency_key"].as_str().unwrap(),
        )
        .is_err());
    assert!(store
        .reconcile_hcm_publication_quarantine(transition_fingerprint)
        .is_err());
    assert_eq!(
        snapshot_files(quarantine_root.parent().unwrap()),
        committed_only_before
    );
}

#[test]
fn refused_hcm_promotion_preserves_open_quarantine_and_previous_authority() {
    let (repo, port, o1, s1) = seed_real_authority_repository();
    let (_, o2, _, promotion_request) =
        prepare_authority_replacement(repo.path(), port, 2, &o1, &s1);
    assert!(ContextResolutionAuthorityAdmissionResolver::new(
        repo.path(),
        AUTHORITY_KIND_REF,
        AUTHORITY_INSTANCE_ID,
        &o1,
    )
    .is_err());
    let mut promotion: serde_json::Value = serde_json::from_slice(&promotion_request).unwrap();
    promotion["expected_current_artifact_fingerprint"] = json!(FINGERPRINT);
    let refused_request = serde_json::to_vec(&promotion).unwrap();
    let canonical_path = repo
        .path()
        .join(".handbook/project/context-resolution-authority.yaml");
    let quarantine_root = repo
        .path()
        .join(".handbook/state/context-resolution-authority/quarantine");
    let canonical_before = fs::read(&canonical_path).unwrap();
    let quarantine_before = snapshot_files(&quarantine_root);

    let execution = handbook_engine::artifact_mutation::ArtifactMutationServiceV1::promote(
        repo.path(),
        AUTHORITY_KIND_REF,
        AUTHORITY_INSTANCE_ID,
        &refused_request,
    )
    .unwrap();
    assert_eq!(execution.result.outcome, "refused");
    assert_eq!(fs::read(&canonical_path).unwrap(), canonical_before);
    assert_eq!(snapshot_files(&quarantine_root), quarantine_before);
    assert!(ContextResolutionAuthorityAdmissionResolver::new(
        repo.path(),
        AUTHORITY_KIND_REF,
        AUTHORITY_INSTANCE_ID,
        &o2,
    )
    .is_err());
}

#[test]
fn cold_load_reconciles_post_commit_quarantine_and_partial_closeout_scratch() {
    let (repo, port, o1, s1) = seed_real_authority_repository();
    let (_, o2, _, promotion_request) =
        prepare_authority_replacement(repo.path(), port, 2, &o1, &s1);
    assert!(ContextResolutionAuthorityAdmissionResolver::new(
        repo.path(),
        AUTHORITY_KIND_REF,
        AUTHORITY_INSTANCE_ID,
        &o1,
    )
    .is_err());
    handbook_engine::artifact_mutation::ArtifactMutationServiceV1::promote(
        repo.path(),
        AUTHORITY_KIND_REF,
        AUTHORITY_INSTANCE_ID,
        &promotion_request,
    )
    .unwrap();
    let quarantine_root = repo
        .path()
        .join(".handbook/state/context-resolution-authority/quarantine");
    let open_path = fs::read_dir(&quarantine_root)
        .unwrap()
        .map(|entry| entry.unwrap().path())
        .find(|path| {
            path.file_name()
                .unwrap()
                .to_string_lossy()
                .ends_with(".json")
                && !path
                    .file_name()
                    .unwrap()
                    .to_string_lossy()
                    .ends_with(".committed.json")
        })
        .unwrap();
    let stem = open_path.file_stem().unwrap().to_string_lossy();
    let completed_path = quarantine_root.join(format!("{stem}.committed.json"));
    let writing_path = quarantine_root.join(format!("{stem}.committed.writing"));
    let open_bytes = fs::read(&open_path).unwrap();
    fs::remove_file(&completed_path).unwrap();
    fs::write(&writing_path, b"{partial-closeout").unwrap();

    ContextResolutionAuthorityAdmissionResolver::new(
        repo.path(),
        AUTHORITY_KIND_REF,
        AUTHORITY_INSTANCE_ID,
        &o2,
    )
    .expect("cold load reconciles the exact committed T2 chain");
    let completed: serde_json::Value =
        serde_json::from_slice(&fs::read(&completed_path).unwrap()).unwrap();
    assert_eq!(completed["resolution"], "committed");
    assert!(!writing_path.exists());
    assert_eq!(fs::read(open_path).unwrap(), open_bytes);
}

#[test]
fn publication_gate_cryptographically_refuses_forged_signature_rp_flags_counter_and_challenge() {
    let (repo, port, o1, s1) = seed_real_authority_repository();
    let (_port, _o2, _, promotion_request) =
        prepare_authority_replacement(repo.path(), port, 2, &o1, &s1);
    let original: serde_json::Value = serde_json::from_slice(&promotion_request).unwrap();
    let raw = BASE64_STANDARD
        .decode(
            original["publisher_authority"]["raw_assertion_response_base64"]
                .as_str()
                .unwrap(),
        )
        .unwrap();
    let rp = Sha256::digest(AUTHENTICATOR_RP_ID.as_bytes());
    let rp_offset = raw
        .windows(rp.len())
        .position(|window| window == rp.as_slice())
        .expect("authenticator data RP hash");
    let mut forged = Vec::new();
    for (label, index) in [
        ("signature", raw.len() - 1),
        ("rp", rp_offset),
        ("flags", rp_offset + 32),
        ("counter", rp_offset + 36),
    ] {
        let mut changed = raw.clone();
        changed[index] ^= 1;
        forged.push((label, BASE64_STANDARD.encode(changed), None));
    }
    let challenge = BASE64_STANDARD
        .decode(
            original["publisher_authority"]["challenge_jcs_base64"]
                .as_str()
                .unwrap(),
        )
        .unwrap();
    let mut challenge_value: serde_json::Value = serde_json::from_slice(&challenge).unwrap();
    challenge_value["operation_id"] = json!("hcm32-authorize-forged-client-data");
    forged.push((
        "challenge",
        original["publisher_authority"]["raw_assertion_response_base64"]
            .as_str()
            .unwrap()
            .to_owned(),
        Some(BASE64_STANDARD.encode(serde_json_canonicalizer::to_vec(&challenge_value).unwrap())),
    ));

    for (label, raw_response, challenge) in forged {
        let mut request = original.clone();
        request["publisher_authority"]["raw_assertion_response_base64"] = json!(raw_response);
        if let Some(challenge) = challenge {
            request["publisher_authority"]["challenge_jcs_base64"] = json!(challenge);
        }
        let error = handbook_engine::artifact_mutation::ArtifactMutationServiceV1::promote(
            repo.path(),
            AUTHORITY_KIND_REF,
            AUTHORITY_INSTANCE_ID,
            &serde_json::to_vec(&request).unwrap(),
        );
        let error = error.expect_err("forged publisher evidence reached persistence");
        assert!(
            error.detail().contains("cryptographic verification"),
            "forged {label} stopped before the cryptographic verifier: {}",
            error.detail()
        );
    }
    assert_eq!(
        DefinitionFingerprint::from_bytes(
            &fs::read(
                repo.path()
                    .join(".handbook/project/context-resolution-authority.yaml"),
            )
            .unwrap(),
        )
        .as_str(),
        o1
    );
}

#[test]
fn cached_and_cloned_admissions_refuse_marker_only_or_incomplete_committed_chains() {
    let (repo, mut port, artifact_fingerprint, _) = seed_real_authority_repository();
    let (profile, stack) = load_kernel_profile_stack(repo.path());
    let input = envelope_input(
        "incomplete-chain",
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
        Vec::new(),
    );
    let mut resolver = ContextResolutionAuthorityAdmissionResolver::new(
        repo.path(),
        AUTHORITY_KIND_REF,
        AUTHORITY_INSTANCE_ID,
        &artifact_fingerprint,
    )
    .unwrap();
    let admission = resolver
        .admit(
            &mut port,
            ContextResolutionAuthorityUse::RootCreation,
            input.candidate_binding(),
        )
        .unwrap();
    let envelope =
        ContextResolutionEnvelope::resolve_root(&profile, &stack, input.clone(), &admission, &[])
            .unwrap();
    let cloned_envelope = envelope.clone();
    let committed = fs::read_dir(
        repo.path()
            .join(".handbook/state/transactions/artifact-promotions"),
    )
    .unwrap()
    .map(|entry| entry.unwrap().path())
    .find(|path| {
        path.extension()
            .is_some_and(|extension| extension == "committed")
            && serde_json::from_slice::<serde_json::Value>(
                &fs::read(path.join("intent.json")).unwrap(),
            )
            .unwrap()["outputs"]
                .as_array()
                .unwrap()
                .iter()
                .any(|output| output["bytes_sha256"] == artifact_fingerprint)
    })
    .expect("committed HCM publication transaction");
    fs::remove_file(committed.join("verified.json")).unwrap();
    assert_eq!(
        resolver
            .admit(
                &mut port,
                ContextResolutionAuthorityUse::RootCreation,
                input.candidate_binding(),
            )
            .unwrap_err()
            .kind(),
        ContextResolutionKernelErrorKind::StaleAuthority
    );
    assert_eq!(
        ContextResolutionEnvelope::resolve_root(&profile, &stack, input, &admission.clone(), &[])
            .unwrap_err()
            .kind(),
        ContextResolutionKernelErrorKind::StaleAuthority
    );
    assert_eq!(
        cloned_envelope.evaluate_mutation("repository_path", "docs/README.md"),
        ContextResolutionMutationDecision::Indeterminate
    );
    assert_eq!(
        cloned_envelope
            .authorize_memory("strategic")
            .unwrap_err()
            .kind(),
        ContextResolutionKernelErrorKind::StaleAuthority
    );
    assert_eq!(
        cloned_envelope
            .authorize_validation("program_gate")
            .unwrap_err()
            .kind(),
        ContextResolutionKernelErrorKind::StaleAuthority
    );
}

#[test]
fn recovery_quarantine_classifies_invalid_predecessor_journal_and_installed_states() {
    fn observed_record(repo: &Path, stale_outer: &str) -> serde_json::Value {
        assert!(ContextResolutionAuthorityAdmissionResolver::new(
            repo,
            AUTHORITY_KIND_REF,
            AUTHORITY_INSTANCE_ID,
            stale_outer,
        )
        .is_err());
        let path =
            fs::read_dir(repo.join(".handbook/state/context-resolution-authority/quarantine"))
                .unwrap()
                .next()
                .unwrap()
                .unwrap()
                .path();
        serde_json::from_slice(&fs::read(path).unwrap()).unwrap()
    }
    fn assert_record(
        repo: &Path,
        stale_outer: &str,
        phase: &str,
        reason: &str,
        journal: Option<&str>,
    ) {
        let record = observed_record(repo, stale_outer);
        assert_eq!(record["phase"], phase);
        assert_eq!(record["reason"], reason);
        assert_eq!(record["journal_transaction_id"].as_str(), journal);
    }
    fn candidate_and_content(
        repo: &Path,
        request: &[u8],
    ) -> (std::path::PathBuf, std::path::PathBuf) {
        let request: serde_json::Value = serde_json::from_slice(request).unwrap();
        let candidate = repo.join(request["candidate_ref"].as_str().unwrap());
        let record: serde_json::Value =
            serde_json::from_slice(&fs::read(&candidate).unwrap()).unwrap();
        let content = repo.join(record["normalized_content_ref"].as_str().unwrap());
        (candidate, content)
    }

    let (repo, port, o1, s1) = seed_real_authority_repository();
    let (_, _, _, _) = prepare_authority_replacement(repo.path(), port, 2, &o1, &s1);
    assert_record(
        repo.path(),
        &o1,
        "authorized_without_generic_intent",
        "retry_available",
        None,
    );

    let (repo, port, o1, s1) = seed_real_authority_repository();
    let (_, _, _, _) = prepare_authority_replacement(repo.path(), port, 2, &o1, &s1);
    fs::remove_dir_all(
        repo.path()
            .join(".handbook/evidence/artifacts/context_resolution_authority/candidates"),
    )
    .unwrap();
    assert_record(
        repo.path(),
        &o1,
        "authorized_without_generic_intent",
        "candidate_missing",
        None,
    );

    let (repo, port, o1, s1) = seed_real_authority_repository();
    let (_, _, _, request) = prepare_authority_replacement(repo.path(), port, 2, &o1, &s1);
    let (_, content) = candidate_and_content(repo.path(), &request);
    fs::write(content, b"{}\n").unwrap();
    assert_record(
        repo.path(),
        &o1,
        "authorized_without_generic_intent",
        "candidate_invalid",
        None,
    );

    let (repo, port, o1, s1) = seed_real_authority_repository();
    let (_, _, _, _) = prepare_authority_replacement(repo.path(), port, 2, &o1, &s1);
    let canonical = repo
        .path()
        .join(".handbook/project/context-resolution-authority.yaml");
    let mut changed = fs::read(&canonical).unwrap();
    changed.push(b'\n');
    fs::write(canonical, changed).unwrap();
    assert_record(
        repo.path(),
        &o1,
        "authorized_without_generic_intent",
        "predecessor_mismatch",
        None,
    );

    let (repo, port, o1, s1) = seed_real_authority_repository();
    let (_, o2, _, _) = prepare_authority_replacement(repo.path(), port, 2, &o1, &s1);
    let conflict = repo
        .path()
        .join(".handbook/state/transactions/artifact-promotions/forged.pending");
    fs::create_dir(&conflict).unwrap();
    fs::write(
        conflict.join("intent.json"),
        serde_json_canonicalizer::to_vec(&json!({"outputs":[{"bytes_sha256":o2}]})).unwrap(),
    )
    .unwrap();
    assert_record(
        repo.path(),
        &o1,
        "generic_pending",
        "journal_conflict",
        Some("forged"),
    );

    let (repo, port, o1, s1) = seed_real_authority_repository();
    let (_, _, _, request) = prepare_authority_replacement(repo.path(), port, 2, &o1, &s1);
    let (_, content) = candidate_and_content(repo.path(), &request);
    let wrapper: serde_json::Value = serde_json::from_slice(&fs::read(content).unwrap()).unwrap();
    fs::write(
        repo.path()
            .join(".handbook/project/context-resolution-authority.yaml"),
        handbook_engine::canonical_yaml::canonical_yaml_bytes(&wrapper).unwrap(),
    )
    .unwrap();
    assert_record(
        repo.path(),
        &o1,
        "installed_without_commit",
        "installed_result_ambiguous",
        None,
    );
}

#[test]
fn quarantine_records_enforce_optional_types_idempotency_and_all_selected_phases() {
    fn finalize(record: &mut serde_json::Value) {
        record.as_object_mut().unwrap().remove("record_fingerprint");
        let fingerprint = DefinitionFingerprint::from_json_value(record).unwrap();
        record["record_fingerprint"] = json!(fingerprint.as_str());
    }

    fn record_in_fresh_store(
        record: &serde_json::Value,
        transition_fingerprint: &str,
    ) -> Result<(), artifact_lineage_store::GenericLineageStoreErrorV1> {
        let repo = tempfile::tempdir().unwrap();
        artifact_lineage_store::GenericArtifactLineageStoreV1::new(repo.path())
            .record_hcm_publication_quarantine(transition_fingerprint, record)
    }

    let (repo, port, o1, s1) = seed_real_authority_repository();
    let (_, _, _, _) = prepare_authority_replacement(repo.path(), port, 2, &o1, &s1);
    assert!(ContextResolutionAuthorityAdmissionResolver::new(
        repo.path(),
        AUTHORITY_KIND_REF,
        AUTHORITY_INSTANCE_ID,
        &o1,
    )
    .is_err());
    let quarantine_path = fs::read_dir(
        repo.path()
            .join(".handbook/state/context-resolution-authority/quarantine"),
    )
    .unwrap()
    .next()
    .unwrap()
    .unwrap()
    .path();
    let base: serde_json::Value =
        serde_json::from_slice(&fs::read(&quarantine_path).unwrap()).unwrap();
    let transition_fingerprint = base["result_transition_fingerprint"]
        .as_str()
        .unwrap()
        .to_owned();
    let before_wrong_candidate = fs::read(&quarantine_path).unwrap();
    let wrong_candidate = artifact_lineage_store::GenericArtifactLineageStoreV1::new(repo.path())
        .complete_hcm_publication_quarantine(
            &transition_fingerprint,
            ".handbook/state/artifact-candidates/wrong.json",
            FINGERPRINT,
            base["idempotency_key"].as_str().unwrap(),
        )
        .unwrap_err();
    assert_eq!(
        wrong_candidate.kind(),
        artifact_lineage_store::GenericLineageStoreErrorKindV1::ConflictingTransactionState
    );
    assert_eq!(before_wrong_candidate, fs::read(&quarantine_path).unwrap());

    for phase in ["generic_pending", "installed_without_commit"] {
        let mut record = base.clone();
        record["phase"] = json!(phase);
        record["reason"] = json!(if phase == "generic_pending" {
            "journal_conflict"
        } else {
            "installed_result_ambiguous"
        });
        record["candidate_ref"] = serde_json::Value::Null;
        record["candidate_fingerprint"] = serde_json::Value::Null;
        record["journal_transaction_id"] = json!("hcm32_pending_fixture");
        finalize(&mut record);
        record_in_fresh_store(&record, &transition_fingerprint)
            .unwrap_or_else(|error| panic!("selected phase {phase} was refused: {error:?}"));
    }

    for (field, value) in [
        ("observed_canonical_artifact_fingerprint", json!(7)),
        ("candidate_ref", json!(7)),
        ("candidate_fingerprint", json!(7)),
        ("journal_transaction_id", json!(7)),
        ("result_transaction_ref", json!(7)),
        ("result_transaction_fingerprint", json!(7)),
    ] {
        let mut record = base.clone();
        record[field] = value;
        if matches!(field, "candidate_ref" | "candidate_fingerprint") {
            record[if field == "candidate_ref" {
                "candidate_fingerprint"
            } else {
                "candidate_ref"
            }] = json!(7);
        }
        finalize(&mut record);
        assert!(
            record_in_fresh_store(&record, &transition_fingerprint).is_err(),
            "wrong-typed optional {field} was admitted"
        );
    }

    let mut wrong_key = base;
    wrong_key["idempotency_key"] = json!("hcm32crpub_wrong");
    finalize(&mut wrong_key);
    assert!(
        record_in_fresh_store(&wrong_key, &transition_fingerprint).is_err(),
        "transition record admitted a non-deterministic idempotency key"
    );
}

#[test]
fn authority_resolver_passes_the_complete_eligible_allow_list_and_refuses_unknown_selection() {
    let (repo, mut port, artifact_fingerprint, _) =
        seed_real_authority_repository_with_credentials(true);
    assert_eq!(port.make_calls, 2);
    port.get_calls.clear();
    port.fixed_sign_count = Some(3);
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
    let path = repo.join(".handbook/project/context-resolution-authority.yaml");
    let mut binding =
        handbook_engine::canonical_yaml::parse_canonical_yaml(&fs::read(&path).unwrap()).unwrap();
    mutate(&mut binding);
    let _ = recompute_semantic_fingerprint;
    let bytes = handbook_engine::canonical_yaml::canonical_yaml_bytes(&binding).unwrap();
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
        .join(".handbook/project/context-resolution-authority.yaml");
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
            binding["declaration_jcs"] = json!("{\"schema_id\":\"uncovered\"}");
        },
        true,
    );
    assert!(ContextResolutionAuthorityAdmissionResolver::new(
        repo.path(),
        AUTHORITY_KIND_REF,
        AUTHORITY_INSTANCE_ID,
        &uncovered,
    )
    .is_err());
    let _ = &mut port;
}

#[test]
fn capsule_gate_refuses_noncanonical_duplicate_substituted_unknown_and_oversized_declarations() {
    let (repo, _port, current, _) = seed_real_authority_repository();
    let wrapper = handbook_engine::canonical_yaml::parse_canonical_yaml(
        &fs::read(
            repo.path()
                .join(".handbook/project/context-resolution-authority.yaml"),
        )
        .unwrap(),
    )
    .unwrap();
    let valid = wrapper["declaration_jcs"].as_str().unwrap();
    let value: serde_json::Value = serde_json::from_str(valid).unwrap();
    let noncanonical = serde_json::to_string_pretty(&value).unwrap();
    let duplicate = format!("{{\"codec_id\":\"duplicate\",{}", &valid[1..]);
    let mut substituted = value.clone();
    substituted["payload_byte_fingerprint"] = json!(FINGERPRINT);
    let substituted =
        String::from_utf8(serde_json_canonicalizer::to_vec(&substituted).unwrap()).unwrap();
    let mut unknown = value;
    unknown["codec_version"] = json!("2.0");
    let unknown = String::from_utf8(serde_json_canonicalizer::to_vec(&unknown).unwrap()).unwrap();
    let oversized = "x".repeat(98_305);

    for (index, declaration) in [noncanonical, duplicate, substituted, unknown, oversized]
        .into_iter()
        .enumerate()
    {
        let intake_request = serde_json::to_vec(&json!({
            "idempotency_key":format!("hcm32_invalid_capsule_{index:04}"),
            "acquisition_mode":"express",
            "expected_current_artifact_fingerprint":current,
            "coverage_submissions":[{
                "coverage_id":"context_resolution_authority.declaration_jcs",
                "state":"supplied",
                "source_kind":"user_declaration",
                "value":declaration,
                "specificity":"exact",
                "confidence":"high",
                "contradiction_refs":[]
            }]
        }))
        .unwrap();
        let intake = handbook_engine::artifact_mutation::ArtifactMutationServiceV1::intake_append(
            repo.path(),
            AUTHORITY_KIND_REF,
            AUTHORITY_INSTANCE_ID,
            &intake_request,
        )
        .unwrap();
        if intake.result.authoritative_outputs.is_empty() {
            assert_eq!(intake.result.outcome, "refused");
            continue;
        }
        let intake_output = &intake.result.authoritative_outputs[0];
        assert!(
            handbook_engine::artifact_mutation::ArtifactMutationServiceV1::candidate_validate(
                repo.path(),
                AUTHORITY_KIND_REF,
                AUTHORITY_INSTANCE_ID,
                &intake_output.relative_ref,
                &intake_output.fingerprint,
                Some(&current),
            )
            .is_err(),
            "invalid capsule case {index} reached candidate eligibility"
        );
    }
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
    port.fixed_sign_count = Some(2);
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
    first_port.fixed_sign_count = Some(2);
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
