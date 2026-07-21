use handbook_engine::{
    charter_lifecycle_state_fingerprint, CharterLifecycleState, DefinitionFingerprint,
    LineageRecordClassV1, TrustedLineageStoreV1,
};
use serde_json::{json, Value};
use std::path::Path;

const VECTORS: &[u8] = include_bytes!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../docs/specs/handbook-contract-membrane/slices/HCM-2.2/contracts/runtime-record-fingerprint-vectors-v1.0.json"
));
const INTENT_VECTORS: &[u8] = include_bytes!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../docs/specs/handbook-contract-membrane/slices/HCM-2.2/contracts/promotion-transaction-intent-vectors-v1.0.json"
));

fn vector(class: &str) -> Value {
    let fixture: Value = serde_json::from_slice(VECTORS).expect("runtime vectors");
    fixture["vectors"]
        .as_array()
        .expect("vector rows")
        .iter()
        .find(|row| row["record_class"] == class)
        .expect("runtime vector class")["record"]
        .clone()
}

fn canonical(value: &Value) -> Vec<u8> {
    serde_json_canonicalizer::to_vec(value).expect("canonical record")
}

fn resolved_definition_bindings() -> Value {
    let fixture: Value = serde_json::from_slice(INTENT_VECTORS).expect("intent vectors");
    fixture["amendment_positive"]["intent_fingerprint_preimage"]["selected_contract"]
        ["resolved_definitions"]
        .clone()
}

fn resign(
    value: &mut Value,
    id_field: &str,
    fingerprint_field: &str,
    prefix: &str,
    audit_only: &[&str],
) {
    let mut preimage = value.clone();
    let object = preimage.as_object_mut().expect("record object");
    object.remove(id_field);
    object.remove(fingerprint_field);
    for field in audit_only {
        object.remove(*field);
    }
    let fingerprint = DefinitionFingerprint::from_json_value(&preimage)
        .expect("record fingerprint")
        .to_string();
    let id = format!(
        "{prefix}_{}",
        fingerprint
            .strip_prefix("sha256:")
            .expect("sha256 fingerprint")
    );
    value[id_field] = Value::String(id);
    value[fingerprint_field] = Value::String(fingerprint);
}

fn seed_lineage(repo: &Path, canonical_bytes: &[u8]) {
    let store = TrustedLineageStoreV1::new(repo);
    for (class, name) in [
        (
            LineageRecordClassV1::AuthenticatorMakeCredentialResponse,
            "authenticator-make-credential-response",
        ),
        (
            LineageRecordClassV1::AuthenticatorRegistration,
            "authenticator-registration",
        ),
        (LineageRecordClassV1::RegistryState, "registry-state"),
        (
            LineageRecordClassV1::RegistryTransition,
            "registry-transition",
        ),
        (
            LineageRecordClassV1::AuthenticatorGetAssertionResponse,
            "authenticator-get-assertion-response",
        ),
        (
            LineageRecordClassV1::AuthenticatorAssertion,
            "authenticator-assertion",
        ),
        (LineageRecordClassV1::Intake, "intake"),
    ] {
        store
            .append_record(class, &canonical(&vector(name)))
            .expect("seed lineage");
    }
    let content_fingerprint = DefinitionFingerprint::from_bytes(canonical_bytes).to_string();
    let content_ref = format!(
        "candidate-content/charter_{}.yaml",
        content_fingerprint.strip_prefix("sha256:").unwrap()
    );
    let content_path = repo.join(".handbook/state").join(&content_ref);
    std::fs::create_dir_all(content_path.parent().unwrap()).expect("candidate content parent");
    std::fs::write(&content_path, canonical_bytes).expect("candidate content");
    let intake = vector("intake");
    let intake_ref = format!(
        "intake-records/{}.json",
        intake["intake_record_id"].as_str().unwrap()
    );
    let mut candidate = vector("candidate");
    candidate["normalized_content_ref"] = Value::String(content_ref.clone());
    let mut subject = candidate.clone();
    for field in [
        "candidate_id",
        "candidate_fingerprint",
        "candidate_subject_fingerprint",
        "validation_result_binding",
    ] {
        subject.as_object_mut().unwrap().remove(field);
    }
    let subject_fingerprint = DefinitionFingerprint::from_json_value(&subject)
        .expect("candidate subject")
        .to_string();
    candidate["candidate_subject_fingerprint"] = Value::String(subject_fingerprint.clone());
    let mut validation = json!({
        "schema_id": "handbook.lifecycle-validation-result",
        "schema_version": "1.0",
        "validation_result_id": "pending",
        "candidate_subject_fingerprint": subject_fingerprint,
        "intake_record_ref": intake_ref,
        "intake_record_fingerprint": intake["record_fingerprint"],
        "normalized_content_ref": content_ref,
        "normalized_content_fingerprint": content_fingerprint,
        "target_instance_id": "project_authority",
        "canonical_artifact_ref": ".handbook/project/charter.yaml",
        "basis_artifact_fingerprint": null,
        "observed_current_artifact_fingerprint": null,
        "profile_ref": "handbook.profile.shipped-root@1.1.0",
        "resolved_profile_fingerprint": "sha256:6a7b41befa77b999b9ee20f513636051726a8401a81bf2f369501e8f3dd4fa74",
        "resolved_definitions": resolved_definition_bindings(),
        "lifecycle_policy_ref": "handbook.lifecycle.constitutional-review-lock@1.0.0",
        "lifecycle_policy_fingerprint": "sha256:88caafb9caaf137647c42a91cd2762ac0871e0a20e2a1844c2c0076d5fb43cc3",
        "lifecycle_head_ref": null,
        "lifecycle_head_fingerprint": null,
        "lifecycle_state": "absent",
        "lifecycle_state_fingerprint": null,
        "active_observations": [],
        "reopened_coverage_ids": [],
        "validation_status": "passed",
        "validated_at_utc": "2026-07-20T00:00:00Z",
        "validation_result_fingerprint": "pending"
    });
    let mut validation_preimage = validation.clone();
    for field in [
        "validation_result_id",
        "validation_result_fingerprint",
        "validated_at_utc",
    ] {
        validation_preimage.as_object_mut().unwrap().remove(field);
    }
    let validation_fingerprint = DefinitionFingerprint::from_json_value(&validation_preimage)
        .expect("validation identity")
        .to_string();
    let validation_id = format!(
        "lifecycle-validation-result_{}",
        validation_fingerprint.strip_prefix("sha256:").unwrap()
    );
    validation["validation_result_id"] = Value::String(validation_id.clone());
    validation["validation_result_fingerprint"] = Value::String(validation_fingerprint);
    let validation_ref = format!("lifecycle-validation-results/{validation_id}.json");
    let validation_path = repo.join(".handbook/state").join(&validation_ref);
    std::fs::create_dir_all(validation_path.parent().unwrap()).expect("validation parent");
    let mut validation_bytes = canonical(&validation);
    validation_bytes.push(b'\n');
    std::fs::write(validation_path, &validation_bytes).expect("validation result");
    candidate["validation_result_binding"] = json!({
        "validation_result_ref": validation_ref,
        "validation_result_fingerprint": validation["validation_result_fingerprint"],
        "result_document_sha256": DefinitionFingerprint::from_bytes(&validation_bytes).to_string(),
        "result_byte_length": validation_bytes.len(),
    });
    resign(
        &mut candidate,
        "candidate_id",
        "candidate_fingerprint",
        "candidate",
        &[],
    );
    let candidate = store
        .append_record(LineageRecordClassV1::Candidate, &canonical(&candidate))
        .expect("candidate fixture");
    let mut approval = vector("approval");
    approval["candidate_ref"] = Value::String(candidate.relative_ref);
    approval["candidate_fingerprint"] = Value::String(candidate.fingerprint);
    resign(
        &mut approval,
        "approval_id",
        "approval_fingerprint",
        "approval",
        &[],
    );
    store
        .append_record(LineageRecordClassV1::Approval, &canonical(&approval))
        .expect("approval fixture");
}

pub fn promote_committed_charter(repo: &Path, canonical_bytes: &[u8], transaction_id: &str) {
    seed_lineage(repo, canonical_bytes);
    let canonical_fingerprint = DefinitionFingerprint::from_bytes(canonical_bytes).to_string();

    let mut promotion = vector("promotion");
    let candidate_path = std::fs::read_dir(repo.join(".handbook/evidence/charter/candidates"))
        .expect("candidate partition")
        .next()
        .expect("candidate entry")
        .expect("candidate directory entry")
        .path();
    let candidate: Value =
        serde_json::from_slice(&std::fs::read(&candidate_path).expect("candidate fixture bytes"))
            .expect("candidate fixture JSON");
    let candidate_ref = format!(
        "candidates/{}",
        candidate_path.file_name().unwrap().to_string_lossy()
    );
    let approval_path = std::fs::read_dir(repo.join(".handbook/state/approvals"))
        .expect("approval partition")
        .next()
        .expect("approval entry")
        .expect("approval directory entry")
        .path();
    let approval: Value =
        serde_json::from_slice(&std::fs::read(&approval_path).expect("approval fixture bytes"))
            .expect("approval fixture JSON");
    let approval_ref = format!(
        "approvals/{}",
        approval_path.file_name().unwrap().to_string_lossy()
    );
    promotion["canonical_artifact_fingerprint"] = Value::String(canonical_fingerprint.clone());
    promotion["candidate_ref"] = Value::String(candidate_ref.clone());
    promotion["candidate_fingerprint"] = candidate["candidate_fingerprint"].clone();
    promotion["approval_refs"] = Value::Array(vec![Value::String(approval_ref.clone())]);
    promotion["validation_result_refs"] = Value::Array(vec![candidate
        ["validation_result_binding"]["validation_result_ref"]
        .clone()]);
    resign(
        &mut promotion,
        "promotion_id",
        "promotion_fingerprint",
        "promotion",
        &[],
    );
    let promotion_id = promotion["promotion_id"]
        .as_str()
        .expect("promotion id")
        .to_owned();

    let mut lifecycle = vector("lifecycle-transition");
    lifecycle["new_observation_refs"] = Value::Array(Vec::new());
    lifecycle["active_observation_refs"] = Value::Array(Vec::new());
    lifecycle["result_state"] = Value::String("current".to_owned());
    let current_state_fingerprint = charter_lifecycle_state_fingerprint(
        lifecycle["lifecycle_policy_ref"]
            .as_str()
            .expect("lifecycle policy ref"),
        lifecycle["lifecycle_policy_fingerprint"]
            .as_str()
            .expect("lifecycle policy fingerprint"),
        "project_authority",
        &canonical_fingerprint,
        CharterLifecycleState::Current,
        &[],
    )
    .expect("current lifecycle state fingerprint");
    lifecycle["prior_state_fingerprint"] = Value::String(current_state_fingerprint.clone());
    lifecycle["result_state_fingerprint"] = Value::String(current_state_fingerprint);
    lifecycle["clearance_promotion_ref"] = Value::String(format!("promotions/{promotion_id}.json"));
    resign(
        &mut lifecycle,
        "transition_id",
        "transition_fingerprint",
        "lifecycle-transition",
        &["transitioned_at_utc"],
    );

    let store = TrustedLineageStoreV1::new(repo);
    let promotion_value = promotion.clone();
    let promotion_bytes = canonical(&promotion);
    let lifecycle_bytes = canonical(&lifecycle);
    let promotion = store
        .append_record(LineageRecordClassV1::Promotion, &promotion_bytes)
        .expect("promotion fixture record");
    let lifecycle = store
        .append_record(LineageRecordClassV1::LifecycleTransition, &lifecycle_bytes)
        .expect("lifecycle fixture record");
    let validation_ref = candidate["validation_result_binding"]["validation_result_ref"]
        .as_str()
        .expect("validation ref");
    let validation: Value = serde_json::from_slice(
        &std::fs::read(repo.join(".handbook/state").join(validation_ref))
            .expect("validation result bytes"),
    )
    .expect("validation result JSON");
    let content_ref = candidate["normalized_content_ref"]
        .as_str()
        .expect("content ref");
    let content_hex = content_ref
        .strip_prefix("candidate-content/charter_")
        .and_then(|value| value.strip_suffix(".yaml"))
        .expect("content-addressed candidate ref");
    let intake_ref = candidate["intake_record_ref"].as_str().expect("intake ref");
    let intake_hex = intake_ref
        .strip_prefix("intake-records/intake_")
        .and_then(|value| value.strip_suffix(".json"))
        .expect("content-addressed intake ref");
    let transaction_id = format!("promotion-transaction_{transaction_id}");
    let mut intent = json!({
        "schema_id": "handbook.charter-promotion-transaction-intent",
        "schema_version": "1.2",
        "transaction_id": transaction_id,
        "promotion_id": promotion.record_id,
        "mutation_mode": "create",
        "target": {
            "target_instance_id": "project_authority",
            "canonical_artifact_ref": ".handbook/project/charter.yaml",
            "basis_artifact_fingerprint": null,
            "observed_current_artifact_fingerprint": null
        },
        "candidate_lineage": {
            "candidate_ref": candidate_ref,
            "candidate_fingerprint": candidate["candidate_fingerprint"],
            "candidate_subject_fingerprint": candidate["candidate_subject_fingerprint"],
            "intake_record_ref": intake_ref,
            "intake_record_fingerprint": format!("sha256:{intake_hex}"),
            "normalized_content_ref": content_ref,
            "normalized_content_fingerprint": format!("sha256:{content_hex}"),
            "validation_result_ref": validation_ref,
            "validation_result_fingerprint": validation["validation_result_fingerprint"]
        },
        "selected_contract": {
            "profile_ref": validation["profile_ref"],
            "resolved_profile_fingerprint": validation["resolved_profile_fingerprint"],
            "resolved_definitions": validation["resolved_definitions"],
            "lifecycle_policy_ref": validation["lifecycle_policy_ref"],
            "lifecycle_policy_fingerprint": validation["lifecycle_policy_fingerprint"],
            "prior_lifecycle_head_ref": null,
            "prior_lifecycle_head_fingerprint": null,
            "prior_lifecycle_state": "absent",
            "prior_lifecycle_state_fingerprint": null,
            "active_observations": [],
            "reopened_coverage_ids": []
        },
        "human_authority": {
            "approval_bindings": [{
                "approval_ref": approval_ref,
                "approval_fingerprint": approval["approval_fingerprint"],
                "approval_class": approval["approval_class"],
                "authority_ref": approval["authority_ref"]
            }],
            "approver_registry_state_ref": promotion_value["approver_registry_state_ref"],
            "approver_registry_state_fingerprint": promotion_value["approver_registry_state_fingerprint"],
            "registry_head_transition_ref": promotion_value["registry_head_transition_ref"],
            "registry_head_transition_fingerprint": promotion_value["registry_head_transition_fingerprint"]
        },
        "outputs": {
            "new_canonical_fingerprint": canonical_fingerprint,
            "new_canonical_document_sha256": DefinitionFingerprint::from_bytes(canonical_bytes).to_string(),
            "new_canonical_byte_length": canonical_bytes.len(),
            "promotion_record": {
                "record_ref": promotion.relative_ref,
                "record_fingerprint": promotion.fingerprint,
                "document_sha256": DefinitionFingerprint::from_bytes(&promotion_bytes).to_string(),
                "byte_length": promotion_bytes.len()
            },
            "lifecycle_transition": {
                "record_ref": lifecycle.relative_ref,
                "record_fingerprint": lifecycle.fingerprint,
                "document_sha256": DefinitionFingerprint::from_bytes(&lifecycle_bytes).to_string(),
                "byte_length": lifecycle_bytes.len()
            }
        },
        "recovery": {
            "old_canonical_status": "absent",
            "old_canonical_fingerprint": null,
            "old_canonical_document_sha256": null,
            "old_canonical_byte_length": null
        },
        "intent_fingerprint": format!("sha256:{}", "0".repeat(64))
    });
    let mut intent_preimage = intent.clone();
    intent_preimage
        .as_object_mut()
        .unwrap()
        .remove("intent_fingerprint");
    intent["intent_fingerprint"] = Value::String(
        DefinitionFingerprint::from_json_value(&intent_preimage)
            .expect("intent fingerprint")
            .to_string(),
    );
    let mut intent_bytes = canonical(&intent);
    intent_bytes.push(b'\n');
    let marker = format!("{}\n", DefinitionFingerprint::from_bytes(&intent_bytes));
    let committed = repo
        .join(".handbook/state/transactions/promotions")
        .join(format!(
            "{}.committed",
            intent["transaction_id"].as_str().unwrap()
        ));
    std::fs::create_dir_all(&committed).expect("committed promotion fixture journal");
    std::fs::write(committed.join("intent.json"), intent_bytes)
        .expect("committed promotion fixture intent");
    std::fs::write(committed.join("committed"), marker.as_bytes())
        .expect("committed promotion fixture marker");
    for marker_name in ["prepared", "canonical-installed", "records-installed"] {
        std::fs::write(committed.join(marker_name), marker.as_bytes())
            .expect("committed promotion fixture marker");
    }
    let canonical_path = repo.join(".handbook/project/charter.yaml");
    std::fs::create_dir_all(canonical_path.parent().expect("canonical parent"))
        .expect("canonical fixture parent");
    std::fs::write(canonical_path, canonical_bytes).expect("canonical Charter fixture");
}
