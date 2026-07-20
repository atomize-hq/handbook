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

fn seed_lineage(repo: &Path) {
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
        (LineageRecordClassV1::Candidate, "candidate"),
        (LineageRecordClassV1::Approval, "approval"),
    ] {
        store
            .append_record(class, &canonical(&vector(name)))
            .expect("seed lineage");
    }
}

pub fn promote_committed_charter(repo: &Path, canonical_bytes: &[u8], transaction_id: &str) {
    seed_lineage(repo);
    let retained_candidate_path = repo.join(".handbook/state/candidates/candidate.yaml");
    std::fs::create_dir_all(
        retained_candidate_path
            .parent()
            .expect("candidate content parent"),
    )
    .expect("candidate content directory");
    std::fs::write(&retained_candidate_path, canonical_bytes).expect("retained candidate content");
    let canonical_fingerprint = DefinitionFingerprint::from_bytes(canonical_bytes).to_string();

    let mut promotion = vector("promotion");
    promotion["canonical_artifact_fingerprint"] = Value::String(canonical_fingerprint.clone());
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
    lifecycle["result_state_fingerprint"] = Value::String(
        charter_lifecycle_state_fingerprint(
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
        .expect("current lifecycle state fingerprint"),
    );
    lifecycle["clearance_promotion_ref"] = Value::String(format!("promotions/{promotion_id}.json"));
    resign(
        &mut lifecycle,
        "transition_id",
        "transition_fingerprint",
        "lifecycle-transition",
        &["transitioned_at_utc"],
    );

    let store = TrustedLineageStoreV1::new(repo);
    let promotion = store
        .append_record(LineageRecordClassV1::Promotion, &canonical(&promotion))
        .expect("promotion fixture record");
    let lifecycle = store
        .append_record(
            LineageRecordClassV1::LifecycleTransition,
            &canonical(&lifecycle),
        )
        .expect("lifecycle fixture record");
    let intent = json!({
        "schema_id": "handbook.promotion-transaction-intent",
        "schema_version": "1.0",
        "transaction_id": transaction_id,
        "canonical_artifact_ref": ".handbook/project/charter.yaml",
        "canonical_fingerprint": canonical_fingerprint,
        "expected_current_artifact_fingerprint": null,
        "had_old_canonical": false,
        "promotion_ref": promotion.relative_ref,
        "promotion_fingerprint": promotion.fingerprint,
        "lifecycle_transition_ref": lifecycle.relative_ref,
        "lifecycle_transition_fingerprint": lifecycle.fingerprint,
    });
    let intent_bytes = canonical(&intent);
    let marker = format!("{}\n", DefinitionFingerprint::from_bytes(&intent_bytes));
    let committed = repo
        .join(".handbook/state/transactions/promotions")
        .join(format!("{transaction_id}.committed"));
    std::fs::create_dir_all(&committed).expect("committed promotion fixture journal");
    std::fs::write(committed.join("intent.json"), intent_bytes)
        .expect("committed promotion fixture intent");
    std::fs::write(committed.join("committed"), marker.as_bytes())
        .expect("committed promotion fixture marker");
    let canonical_path = repo.join(".handbook/project/charter.yaml");
    std::fs::create_dir_all(canonical_path.parent().expect("canonical parent"))
        .expect("canonical fixture parent");
    std::fs::write(canonical_path, canonical_bytes).expect("canonical Charter fixture");
}
