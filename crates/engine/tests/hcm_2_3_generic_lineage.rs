#![allow(dead_code)]

#[path = "../src/artifact_lineage_store.rs"]
mod artifact_lineage_store;
#[path = "../src/canonical_repo_support.rs"]
mod canonical_repo_support;

use artifact_lineage_store::{
    set_after_retained_read_hook_for_testing, set_before_publication_rename_hook_for_testing,
    EstablishedRefusalCodeV1, EstablishedRefusalV1, GenericArtifactLineageStoreV1,
    GenericArtifactOperationV1, GenericAuthorityClassV1, GenericCommitPlanV1,
    GenericExecutionDispositionV1, GenericInstallModeV1, GenericLineageFaultPointV1,
    GenericLineageStoreErrorKindV1, GenericMutationRequestV1, GenericPlannedOutcomeV1,
    GenericPlannedOutputV1, GenericReceiptClassV1, GenericRefusalLayerV1, GenericRefusalPlanV1,
};
use handbook_engine::{DefinitionFingerprint, RepositoryInvocationIdentityServiceV1};
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::collections::BTreeMap;
use std::fs::{self, File, FileTimes};
use std::path::{Path, PathBuf};
#[cfg(windows)]
use std::sync::atomic::AtomicBool;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Barrier};
use std::thread;

const REPOSITORY_ID: &str =
    "sha256:1111111111111111111111111111111111111111111111111111111111111111";
const OWNER_SUBJECT: &str =
    "sha256:6666666666666666666666666666666666666666666666666666666666666666";
const CONTEXT: &str = "sha256:6247735430f2296162010902e7f6e1e20403f67ab6de7a4e4cc7fcdf9bd466c8";
const KIND_REF: &str = "example.artifact-kind.registry-brief@1.0.0";
const INSTANCE_ID: &str = "registry_brief";
const FIXED_NOW: &str = "2026-07-21T00:00:00Z";

fn runtime_vectors() -> Value {
    serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../docs/specs/handbook-contract-membrane/slices/HCM-2.3/contracts/",
        "generic-artifact-runtime-vectors-v1.0.json"
    )))
    .expect("runtime vectors")
}

fn runtime_schema() -> Value {
    serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../docs/specs/handbook-contract-membrane/slices/HCM-2.3/contracts/",
        "generic-artifact-runtime-records-1.0.0.schema.json"
    )))
    .expect("runtime schema")
}

fn control_vectors() -> Value {
    serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../docs/specs/handbook-contract-membrane/slices/HCM-2.3/contracts/",
        "generic-artifact-control-vectors-v1.0.json"
    )))
    .expect("control vectors")
}

fn control_schema() -> Value {
    serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../docs/specs/handbook-contract-membrane/slices/HCM-2.3/contracts/",
        "generic-artifact-control-records-1.0.0.schema.json"
    )))
    .expect("control schema")
}

fn jcs_lf(value: &Value) -> Vec<u8> {
    let mut bytes = serde_json_canonicalizer::to_vec(value).expect("JCS");
    bytes.push(b'\n');
    bytes
}

fn sha256(bytes: &[u8]) -> String {
    format!("sha256:{:x}", Sha256::digest(bytes))
}

fn record(class: &str) -> (Value, Vec<u8>) {
    let vectors = runtime_vectors();
    let value = vectors["records"]
        .as_array()
        .expect("record array")
        .iter()
        .find(|entry| entry["record_class"] == class)
        .expect("record class")["record"]
        .clone();
    let bytes = jcs_lf(&value);
    (value, bytes)
}

fn request(operation: GenericArtifactOperationV1, key: &str) -> GenericMutationRequestV1 {
    let request_subject = match operation {
        GenericArtifactOperationV1::IntakeRecordAppend => json!({
            "operation_id": "intake.record.append",
            "kind_ref": KIND_REF,
            "instance_id": INSTANCE_ID,
            "acquisition_mode": "express",
            "expected_current_artifact_fingerprint": null,
            "coverage_input_fingerprint": "sha256:8888888888888888888888888888888888888888888888888888888888888888",
            "operation_context_fingerprint": CONTEXT
        }),
        GenericArtifactOperationV1::ArtifactCandidateAppend => json!({
            "operation_id": "artifact.candidate.append",
            "kind_ref": KIND_REF,
            "instance_id": INSTANCE_ID,
            "intake_record_ref": ".handbook/state/artifacts/registry_brief/intake-records/intake_b88c7d71baafeef02f379be8e317df15d9ad77d6b8cc7599ce3b25a6b69d684c.json",
            "intake_record_fingerprint": "sha256:b88c7d71baafeef02f379be8e317df15d9ad77d6b8cc7599ce3b25a6b69d684c",
            "expected_candidate_fingerprint": "sha256:dad8bb3c473f18ea28d6fd006f98a43d8574b5c031c05d0c6f2b14fcdf21d57c",
            "operation_context_fingerprint": CONTEXT
        }),
        GenericArtifactOperationV1::ArtifactCandidatePromote => json!({
            "operation_id": "artifact.candidate.promote",
            "kind_ref": KIND_REF,
            "instance_id": INSTANCE_ID,
            "candidate_ref": ".handbook/evidence/artifacts/registry_brief/candidates/candidate_dad8bb3c473f18ea28d6fd006f98a43d8574b5c031c05d0c6f2b14fcdf21d57c.json",
            "candidate_fingerprint": "sha256:dad8bb3c473f18ea28d6fd006f98a43d8574b5c031c05d0c6f2b14fcdf21d57c",
            "expected_current_artifact_fingerprint": null,
            "operation_context_fingerprint": CONTEXT,
            "canonical_artifact_ref": ".handbook/project/registry-brief.yaml"
        }),
    };
    GenericMutationRequestV1 {
        repository_identity_fingerprint: REPOSITORY_ID.to_owned(),
        owner_contract_subject_fingerprint: OWNER_SUBJECT.to_owned(),
        operation,
        idempotency_key: key.to_owned(),
        request_subject,
    }
}

fn promotion_request(
    key: &str,
    expected_current_artifact_fingerprint: Option<&str>,
) -> GenericMutationRequestV1 {
    let mut request = request(GenericArtifactOperationV1::ArtifactCandidatePromote, key);
    request.request_subject["expected_current_artifact_fingerprint"] =
        expected_current_artifact_fingerprint.map_or(Value::Null, |value| json!(value));
    request
}

fn output(
    token: &str,
    authority_class: GenericAuthorityClassV1,
    final_ref: &str,
    bytes: Vec<u8>,
    output_fingerprint: &str,
    install_mode: GenericInstallModeV1,
    receipt_class: Option<GenericReceiptClassV1>,
) -> GenericPlannedOutputV1 {
    GenericPlannedOutputV1 {
        token: token.to_owned(),
        authority_class,
        final_ref: final_ref.to_owned(),
        bytes,
        output_fingerprint: output_fingerprint.to_owned(),
        install_mode,
        receipt_class,
    }
}

fn intake_plan(sampled_time: &str) -> GenericPlannedOutcomeV1 {
    assert_eq!(sampled_time, FIXED_NOW);
    let vectors = runtime_vectors();
    let (_, intake_bytes) = record("intake-1.2");
    let values = vectors["value_objects"].as_array().expect("values");
    GenericPlannedOutcomeV1::Commit(GenericCommitPlanV1 {
        operation_context_fingerprint: CONTEXT.to_owned(),
        expected_basis_fingerprint: None,
        sampled_finalized_at_utc: Some(sampled_time.to_owned()),
        outputs: vec![
            output(
                "title-value",
                GenericAuthorityClassV1::SubordinateClosure,
                values[0]["ref"].as_str().unwrap(),
                jcs_lf(&values[0]["value"]),
                values[0]["fingerprint"].as_str().unwrap(),
                GenericInstallModeV1::CreateNew,
                None,
            ),
            output(
                "summary-value",
                GenericAuthorityClassV1::SubordinateClosure,
                values[1]["ref"].as_str().unwrap(),
                jcs_lf(&values[1]["value"]),
                values[1]["fingerprint"].as_str().unwrap(),
                GenericInstallModeV1::CreateNew,
                None,
            ),
            output(
                "intake-record",
                GenericAuthorityClassV1::SemanticRecord,
                ".handbook/state/artifacts/registry_brief/intake-records/intake_b88c7d71baafeef02f379be8e317df15d9ad77d6b8cc7599ce3b25a6b69d684c.json",
                intake_bytes,
                "sha256:b88c7d71baafeef02f379be8e317df15d9ad77d6b8cc7599ce3b25a6b69d684c",
                GenericInstallModeV1::CreateNew,
                Some(GenericReceiptClassV1::SemanticRecord),
            ),
        ],
    })
}

fn variable_intake_plan(value_count: usize, duplicate_last: bool) -> GenericPlannedOutcomeV1 {
    assert!((1..=3).contains(&value_count));
    let vectors = runtime_vectors();
    let (mut intake, _) = record("intake-1.2");
    let template_rows = intake["coverage_results"].as_array().unwrap().clone();
    let mut rows = Vec::new();
    let mut outputs = Vec::new();
    for ordinal in 0..value_count {
        let value = if ordinal < 2 {
            vectors["value_objects"][ordinal]["value"].clone()
        } else if duplicate_last {
            vectors["value_objects"][0]["value"].clone()
        } else {
            json!("Third distinct value")
        };
        let bytes = jcs_lf(&value);
        let fingerprint = sha256(&serde_json_canonicalizer::to_vec(&value).unwrap());
        let reference = format!(
            ".handbook/evidence/artifacts/registry_brief/intake-values/value_{}.json",
            fingerprint.strip_prefix("sha256:").unwrap()
        );
        let mut row = template_rows[ordinal.min(1)].clone();
        row["coverage_id"] = json!(format!("registry_brief.field_{ordinal}"));
        row["value_ref"] = json!(reference);
        row["value_fingerprint"] = json!(fingerprint);
        rows.push(row);
        if !outputs
            .iter()
            .any(|output: &GenericPlannedOutputV1| output.output_fingerprint == fingerprint)
        {
            outputs.push(output(
                &format!("field-{ordinal}-value"),
                GenericAuthorityClassV1::SubordinateClosure,
                &reference,
                bytes,
                &fingerprint,
                GenericInstallModeV1::CreateNew,
                None,
            ));
        }
    }
    intake["coverage_results"] = Value::Array(rows);
    let object = intake.as_object_mut().unwrap();
    object.remove("intake_record_id");
    object.remove("record_fingerprint");
    let fingerprint = sha256(&serde_json_canonicalizer::to_vec(&intake).unwrap());
    let id = format!("intake_{}", fingerprint.strip_prefix("sha256:").unwrap());
    intake["intake_record_id"] = json!(id);
    intake["record_fingerprint"] = json!(fingerprint);
    outputs.push(output(
        "intake-record",
        GenericAuthorityClassV1::SemanticRecord,
        &format!(".handbook/state/artifacts/registry_brief/intake-records/{id}.json"),
        jcs_lf(&intake),
        &fingerprint,
        GenericInstallModeV1::CreateNew,
        Some(GenericReceiptClassV1::SemanticRecord),
    ));
    GenericPlannedOutcomeV1::Commit(GenericCommitPlanV1 {
        operation_context_fingerprint: CONTEXT.to_owned(),
        expected_basis_fingerprint: None,
        sampled_finalized_at_utc: Some(FIXED_NOW.to_owned()),
        outputs,
    })
}

fn candidate_plan() -> GenericPlannedOutcomeV1 {
    let vectors = runtime_vectors();
    let (_, validation_bytes) = record("validation-result-1.0");
    let (_, candidate_bytes) = record("candidate-1.4");
    GenericPlannedOutcomeV1::Commit(GenericCommitPlanV1 {
        operation_context_fingerprint: CONTEXT.to_owned(),
        expected_basis_fingerprint: None,
        sampled_finalized_at_utc: None,
        outputs: vec![
            output(
                "normalized-content",
                GenericAuthorityClassV1::SubordinateClosure,
                vectors["normalized_content"]["ref"].as_str().unwrap(),
                jcs_lf(&vectors["normalized_content"]["value"]),
                vectors["normalized_content"]["fingerprint"]
                    .as_str()
                    .unwrap(),
                GenericInstallModeV1::CreateNew,
                None,
            ),
            output(
                "validation-result",
                GenericAuthorityClassV1::SubordinateClosure,
                ".handbook/evidence/artifacts/registry_brief/validation-results/validation_f6395dd6b06c2016c37e5393c09fb6ba63bf8fdc60a223d1a1727bccd4357e6e.json",
                validation_bytes,
                "sha256:f6395dd6b06c2016c37e5393c09fb6ba63bf8fdc60a223d1a1727bccd4357e6e",
                GenericInstallModeV1::CreateNew,
                None,
            ),
            output(
                "candidate-record",
                GenericAuthorityClassV1::SemanticRecord,
                ".handbook/evidence/artifacts/registry_brief/candidates/candidate_dad8bb3c473f18ea28d6fd006f98a43d8574b5c031c05d0c6f2b14fcdf21d57c.json",
                candidate_bytes,
                "sha256:dad8bb3c473f18ea28d6fd006f98a43d8574b5c031c05d0c6f2b14fcdf21d57c",
                GenericInstallModeV1::CreateNew,
                Some(GenericReceiptClassV1::SemanticRecord),
            ),
        ],
    })
}

fn promotion_plan(expected_basis: Option<&str>) -> GenericPlannedOutcomeV1 {
    let vectors = runtime_vectors();
    let (mut promotion, mut promotion_bytes) = record("promotion-1.2");
    let mut promotion_fingerprint = promotion["promotion_fingerprint"]
        .as_str()
        .unwrap()
        .to_owned();
    if let Some(expected_basis) = expected_basis {
        promotion["expected_current_artifact_fingerprint"] = json!(expected_basis);
        promotion.as_object_mut().unwrap().remove("promotion_id");
        promotion
            .as_object_mut()
            .unwrap()
            .remove("promotion_fingerprint");
        promotion_fingerprint = sha256(&serde_json_canonicalizer::to_vec(&promotion).unwrap());
        let promotion_hex = promotion_fingerprint.strip_prefix("sha256:").unwrap();
        promotion["promotion_id"] = json!(format!("promotion_{promotion_hex}"));
        promotion["promotion_fingerprint"] = json!(promotion_fingerprint);
        promotion_bytes = jcs_lf(&promotion);
    }
    let promotion_hex = promotion_fingerprint.strip_prefix("sha256:").unwrap();
    GenericPlannedOutcomeV1::Commit(GenericCommitPlanV1 {
        operation_context_fingerprint: CONTEXT.to_owned(),
        expected_basis_fingerprint: expected_basis.map(str::to_owned),
        sampled_finalized_at_utc: None,
        outputs: vec![
            output(
                "canonical-artifact",
                GenericAuthorityClassV1::CanonicalTruth,
                ".handbook/project/registry-brief.yaml",
                vectors["canonical_yaml"]["bytes_utf8_lf"]
                    .as_str()
                    .unwrap()
                    .as_bytes()
                    .to_vec(),
                vectors["canonical_yaml"]["sha256"].as_str().unwrap(),
                GenericInstallModeV1::ReplaceIfCurrent,
                Some(GenericReceiptClassV1::CanonicalTruth),
            ),
            output(
                "promotion-record",
                GenericAuthorityClassV1::SemanticRecord,
                &format!(
                    ".handbook/state/artifacts/registry_brief/promotion-records/promotion_{promotion_hex}.json"
                ),
                promotion_bytes,
                &promotion_fingerprint,
                GenericInstallModeV1::CreateNew,
                Some(GenericReceiptClassV1::SemanticRecord),
            ),
        ],
    })
}

fn refusal_plan() -> GenericPlannedOutcomeV1 {
    GenericPlannedOutcomeV1::Refuse(GenericRefusalPlanV1 {
        operation_context_fingerprint: CONTEXT.to_owned(),
        expected_basis_fingerprint: None,
        refusal: artifact_lineage_store::EstablishedRefusalV1 {
            code: EstablishedRefusalCodeV1::StaleCurrentArtifact,
            layer: GenericRefusalLayerV1::Currentness,
            expected_fingerprint: None,
            observed_fingerprint: Some(
                "sha256:698d2414185110592a993896e8239f4a324a41e6293a3f77b1ec7538436ab63d"
                    .to_owned(),
            ),
        },
    })
}

fn execute(
    store: &GenericArtifactLineageStoreV1,
    request: GenericMutationRequestV1,
    outcome: GenericPlannedOutcomeV1,
) -> artifact_lineage_store::GenericMutationExecutionV1 {
    store
        .execute_at_for_testing(request, FIXED_NOW, move |_| Ok(outcome))
        .expect("mutation")
}

fn domain_paths(root: &Path) -> Vec<String> {
    let mut paths = Vec::new();
    fn visit(root: &Path, at: &Path, out: &mut Vec<String>) {
        let Ok(entries) = fs::read_dir(at) else {
            return;
        };
        let mut entries = entries.map(Result::unwrap).collect::<Vec<_>>();
        entries.sort_by_key(|entry| entry.file_name());
        for entry in entries {
            let path = entry.path();
            let relative = path
                .strip_prefix(root)
                .unwrap()
                .to_string_lossy()
                .replace('\\', "/");
            if relative != ".handbook/state/locks"
                && relative != ".handbook/state/locks/generic-artifact-operations.lock"
            {
                out.push(relative);
            }
            if path.is_dir() {
                visit(root, &path, out);
            }
        }
    }
    visit(root, root, &mut paths);
    paths
}

fn tree_bytes(root: &Path) -> BTreeMap<String, Vec<u8>> {
    let mut map = BTreeMap::new();
    for relative in domain_paths(root) {
        let path = root.join(&relative);
        if path.is_file() {
            map.insert(relative, fs::read(path).unwrap());
        }
    }
    map
}

fn rewrite_control(path: &Path, fingerprint_field: &str, mutate: impl FnOnce(&mut Value)) {
    let mut value: Value = serde_json::from_slice(&fs::read(path).unwrap()).unwrap();
    mutate(&mut value);
    value.as_object_mut().unwrap().remove(fingerprint_field);
    let fingerprint = sha256(&serde_json_canonicalizer::to_vec(&value).unwrap());
    value
        .as_object_mut()
        .unwrap()
        .insert(fingerprint_field.into(), json!(fingerprint));
    fs::write(path, jcs_lf(&value)).unwrap();
}

fn refingerprint_control(mut value: Value, fingerprint_field: &str) -> Value {
    value.as_object_mut().unwrap().remove(fingerprint_field);
    let fingerprint = sha256(&serde_json_canonicalizer::to_vec(&value).unwrap());
    value
        .as_object_mut()
        .unwrap()
        .insert(fingerprint_field.to_owned(), json!(fingerprint));
    value
}

fn remove_tree_if_present(path: &Path) {
    if path.exists() {
        fs::remove_dir_all(path).unwrap();
    }
}

fn assert_no_promotion_authority(root: &Path) {
    let paths = domain_paths(root);
    assert!(!paths
        .iter()
        .any(|path| path.ends_with("commit-marker.json")));
    assert!(!paths.iter().any(|path| {
        path.contains("idempotency/generic-artifact-operations/results/")
            || path.contains("idempotency/generic-artifact-operations/ledger/")
            || path.contains("state/artifacts/registry_brief/promotion-records/")
    }));
}

fn refingerprint_forged_intent(value: &mut Value) {
    let request = json!({
        "schema_id": "handbook.generic-domain-mutation-request",
        "schema_version": "1.0",
        "repository_identity_fingerprint": value["repository_identity_fingerprint"],
        "owner_contract_ref": artifact_lineage_store::GENERIC_ARTIFACT_OWNER_CONTRACT_REF,
        "owner_contract_subject_fingerprint": value["owner_contract_subject_fingerprint"],
        "domain_mutation_key_fingerprint": value["domain_mutation_key_fingerprint"],
        "request_subject": value["request_subject"],
    });
    let request_fingerprint =
        sha256(&serde_json_canonicalizer::to_vec(&request).expect("request JCS"));
    value["request_fingerprint"] = json!(request_fingerprint);
    let transaction = json!({
        "repository_identity_fingerprint": value["repository_identity_fingerprint"],
        "owner_contract_ref": artifact_lineage_store::GENERIC_ARTIFACT_OWNER_CONTRACT_REF,
        "owner_contract_subject_fingerprint": value["owner_contract_subject_fingerprint"],
        "operation_id": value["operation_id"],
        "domain_mutation_key_fingerprint": value["domain_mutation_key_fingerprint"],
        "request_fingerprint": value["request_fingerprint"],
    });
    let transaction_fingerprint =
        sha256(&serde_json_canonicalizer::to_vec(&transaction).expect("transaction JCS"));
    let token = match value["operation_id"].as_str().unwrap() {
        "intake.record.append" => "intake_record_append",
        "artifact.candidate.append" => "artifact_candidate_append",
        "artifact.candidate.promote" => "artifact_candidate_promote",
        _ => unreachable!(),
    };
    value["transaction_id"] = json!(format!(
        "{token}_{}",
        transaction_fingerprint.strip_prefix("sha256:").unwrap()
    ));
}

fn install_subset(root: &Path, plan: &GenericPlannedOutcomeV1, subset: usize) {
    let GenericPlannedOutcomeV1::Commit(plan) = plan else {
        panic!("commit plan")
    };
    for (ordinal, output) in plan.outputs.iter().enumerate() {
        if subset & (1 << ordinal) != 0 {
            let target = root.join(&output.final_ref);
            fs::create_dir_all(target.parent().unwrap()).unwrap();
            if output.install_mode == GenericInstallModeV1::ReplaceIfCurrent {
                let transaction =
                    pending_dir(root, GenericArtifactOperationV1::ArtifactCandidatePromote);
                let publication: Value = serde_json::from_slice(
                    &fs::read(transaction.join("native-publication.json")).unwrap(),
                )
                .unwrap();
                assert_eq!(publication["canonical_ref"], output.final_ref);
                let candidate = root.join(publication["candidate_ref"].as_str().unwrap());
                fs::rename(candidate, target).unwrap();
            } else {
                fs::write(target, &output.bytes).unwrap();
            }
        }
    }
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

fn journal_family(operation: GenericArtifactOperationV1) -> &'static str {
    match operation {
        GenericArtifactOperationV1::IntakeRecordAppend => "intake-records",
        GenericArtifactOperationV1::ArtifactCandidateAppend => "artifact-candidates",
        GenericArtifactOperationV1::ArtifactCandidatePromote => "artifact-promotions",
    }
}

fn pending_dir(root: &Path, operation: GenericArtifactOperationV1) -> PathBuf {
    let family = root
        .join(".handbook/state/transactions")
        .join(journal_family(operation));
    fs::read_dir(family)
        .unwrap()
        .map(Result::unwrap)
        .map(|entry| entry.path())
        .find(|path| path.extension().is_some_and(|value| value == "pending"))
        .expect("pending transaction")
}

fn establishing_intent(root: &Path) -> PathBuf {
    fs::read_dir(root.join(".handbook/state/idempotency/generic-artifact-operations/establishing"))
        .expect("establishing directory")
        .map(|entry| entry.expect("establishing entry").path())
        .find(|path| path.extension().is_some_and(|value| value == "intent"))
        .expect("established intent")
}

#[test]
fn pre_establishment_invalid_key_has_zero_domain_delta() {
    let repo = tempfile::tempdir().unwrap();
    let store = GenericArtifactLineageStoreV1::new(repo.path());
    let mut invalid = request(GenericArtifactOperationV1::IntakeRecordAppend, "short");
    invalid.idempotency_key = "contains spaces and is short".to_owned();

    let failure = store
        .execute_at_for_testing(invalid, FIXED_NOW, |_| {
            panic!("invalid request must not reach evaluation")
        })
        .unwrap_err();

    assert_eq!(
        failure.kind(),
        GenericLineageStoreErrorKindV1::InvalidRequest
    );
    assert!(domain_paths(repo.path()).is_empty());
}

#[test]
fn intake_output_contract_accepts_one_three_and_deduplicated_equal_values() {
    for (suffix, plan, expected_outputs) in [
        ("one", variable_intake_plan(1, false), 2usize),
        ("three", variable_intake_plan(3, false), 4usize),
        ("dedup", variable_intake_plan(3, true), 3usize),
    ] {
        let repo = tempfile::tempdir().unwrap();
        let store = GenericArtifactLineageStoreV1::new(repo.path());
        let key = format!("intake_dynamic_{suffix}_0001");
        assert_eq!(
            match &plan {
                GenericPlannedOutcomeV1::Commit(plan) => plan.outputs.len(),
                GenericPlannedOutcomeV1::Refuse(_) => 0,
            },
            expected_outputs
        );
        let execution = execute(
            &store,
            request(GenericArtifactOperationV1::IntakeRecordAppend, &key),
            plan,
        );
        assert_eq!(
            execution.disposition,
            GenericExecutionDispositionV1::Committed
        );
    }
}

#[test]
fn verified_descriptors_must_equal_the_exact_intent_projection() {
    let repo = tempfile::tempdir().unwrap();
    let store = GenericArtifactLineageStoreV1::new(repo.path());
    let failure = store
        .execute_at_with_fault_for_testing(
            request(
                GenericArtifactOperationV1::IntakeRecordAppend,
                "verified_projection_0001",
            ),
            FIXED_NOW,
            GenericLineageFaultPointV1::AfterVerified,
            |context| {
                let mut plan = intake_plan(FIXED_NOW);
                let GenericPlannedOutcomeV1::Commit(plan) = &mut plan else {
                    unreachable!()
                };
                plan.sampled_finalized_at_utc = context.sampled_finalized_at_utc;
                Ok(GenericPlannedOutcomeV1::Commit(plan.clone()))
            },
        )
        .unwrap_err();
    assert_eq!(
        failure.kind(),
        GenericLineageStoreErrorKindV1::InjectedFault,
        "{failure:?}"
    );
    let verified = pending_dir(repo.path(), GenericArtifactOperationV1::IntakeRecordAppend)
        .join("verified.json");
    rewrite_control(&verified, "verified_fingerprint", |value| {
        let length = value["staged_outputs"][0]["byte_length"].as_u64().unwrap();
        value["staged_outputs"][0]["byte_length"] = json!(length + 1);
    });
    assert_eq!(
        store.recover().unwrap_err().kind(),
        GenericLineageStoreErrorKindV1::ConflictingTransactionState
    );
}

#[test]
fn committed_ledger_rejects_unknown_and_nonclosed_tombstone_states() {
    for mutation in ["unknown", "tombstone"] {
        let repo = tempfile::tempdir().unwrap();
        let store = GenericArtifactLineageStoreV1::new(repo.path());
        execute(
            &store,
            request(
                GenericArtifactOperationV1::IntakeRecordAppend,
                &format!("ledger_{mutation}_00000001"),
            ),
            intake_plan(FIXED_NOW),
        );
        let ledger_root = repo
            .path()
            .join(".handbook/state/idempotency/generic-artifact-operations/ledger");
        let ledger = fs::read_dir(ledger_root)
            .unwrap()
            .next()
            .unwrap()
            .unwrap()
            .path();
        rewrite_control(&ledger, "entry_fingerprint", |value| match mutation {
            "unknown" => value["state"] = json!("unrecognized"),
            "tombstone" => {
                value["state"] = json!("tombstone");
                value["retained_until_utc"] = Value::Null;
            }
            _ => unreachable!(),
        });
        assert_eq!(
            store.recover().unwrap_err().kind(),
            GenericLineageStoreErrorKindV1::RetainedResultMismatch,
            "mutation {mutation}"
        );
    }
}

#[test]
fn artifact_inventory_rejects_unknown_names_and_per_instance_byte_overflow() {
    let repo = tempfile::tempdir().unwrap();
    let store = GenericArtifactLineageStoreV1::new(repo.path());
    let content = repo
        .path()
        .join(".handbook/evidence/artifacts/registry_brief/content");
    fs::create_dir_all(&content).unwrap();
    fs::write(content.join("unexpected.json"), b"x").unwrap();
    assert_eq!(
        store.recover().unwrap_err().kind(),
        GenericLineageStoreErrorKindV1::ConflictingTransactionState
    );

    fs::remove_file(content.join("unexpected.json")).unwrap();
    for ordinal in 0..257u64 {
        let filename = format!("content_{ordinal:064x}.json");
        let file = fs::File::create(content.join(filename)).unwrap();
        file.set_len(1024 * 1024).unwrap();
    }
    assert_eq!(
        store.recover().unwrap_err().kind(),
        GenericLineageStoreErrorKindV1::BoundExceeded
    );
}

#[test]
fn authority_revalidation_precedes_replay_lookup_and_intent_establishment() {
    let repo = tempfile::tempdir().unwrap();
    let store = GenericArtifactLineageStoreV1::new(repo.path());
    let retained = request(
        GenericArtifactOperationV1::IntakeRecordAppend,
        "authority_replay_00000001",
    );
    execute(&store, retained.clone(), intake_plan(FIXED_NOW));
    let failure = store
        .execute_authorized(
            retained,
            || {
                Err(artifact_lineage_store::GenericLineageStoreErrorV1::new(
                    GenericLineageStoreErrorKindV1::InvalidRequest,
                    "injected authority mismatch",
                ))
            },
            |_| Ok(()),
            |_, _, _, _| Ok(()),
            |_| panic!("authority mismatch must precede retained lookup"),
            |_, _| panic!("authority mismatch must precede plan revalidation"),
        )
        .unwrap_err();
    assert_eq!(
        failure.kind(),
        GenericLineageStoreErrorKindV1::InvalidRequest
    );

    let before = domain_paths(repo.path());
    let mut authorizations = 0usize;
    let failure = store
        .execute_authorized(
            request(
                GenericArtifactOperationV1::IntakeRecordAppend,
                "authority_establish_00000001",
            ),
            || {
                authorizations += 1;
                if authorizations == 2 {
                    Err(artifact_lineage_store::GenericLineageStoreErrorV1::new(
                        GenericLineageStoreErrorKindV1::InvalidRequest,
                        "injected authority mutation before establish",
                    ))
                } else {
                    Ok(())
                }
            },
            |_| Ok(()),
            |_, _, _, _| Ok(()),
            |context| {
                let GenericPlannedOutcomeV1::Commit(mut plan) = intake_plan(FIXED_NOW) else {
                    unreachable!()
                };
                plan.sampled_finalized_at_utc = context.sampled_finalized_at_utc;
                Ok(GenericPlannedOutcomeV1::Commit(plan))
            },
            |_, _| panic!("identity revalidation must precede plan revalidation"),
        )
        .unwrap_err();
    assert_eq!(
        failure.kind(),
        GenericLineageStoreErrorKindV1::InvalidRequest
    );
    assert_eq!(authorizations, 2);
    assert_eq!(before, domain_paths(repo.path()));
}

#[test]
fn exact_plan_revalidation_precedes_intent_establishment() {
    let repo = tempfile::tempdir().unwrap();
    let store = GenericArtifactLineageStoreV1::new(repo.path());
    store
        .recover()
        .expect("initialize generic store lock domain");
    let before = domain_paths(repo.path());
    let failure = store
        .execute_authorized(
            request(
                GenericArtifactOperationV1::IntakeRecordAppend,
                "plan_revalidation_000001",
            ),
            || Ok(()),
            |_| Ok(()),
            |_, _, _, _| Ok(()),
            |context| {
                let GenericPlannedOutcomeV1::Commit(mut plan) = intake_plan(FIXED_NOW) else {
                    unreachable!()
                };
                plan.sampled_finalized_at_utc = context.sampled_finalized_at_utc;
                Ok(GenericPlannedOutcomeV1::Commit(plan))
            },
            |_, _| {
                Err(artifact_lineage_store::GenericLineageStoreErrorV1::new(
                    GenericLineageStoreErrorKindV1::InvalidRequest,
                    "injected selection, profile, or descriptor authority mutation",
                ))
            },
        )
        .unwrap_err();
    assert_eq!(
        failure.kind(),
        GenericLineageStoreErrorKindV1::InvalidRequest
    );
    assert_eq!(before, domain_paths(repo.path()));
}

fn markerless_repository() -> (
    tempfile::TempDir,
    handbook_engine::artifact_repository::ArtifactRepositoryV1,
    handbook_engine::artifact_repository::ArtifactTargetV1,
    PathBuf,
) {
    let fixture =
        Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/hcm_2_3_generic_custom_kind");
    let repo = tempfile::tempdir().unwrap();
    copy_fixture_tree(&fixture, repo.path());
    RepositoryInvocationIdentityServiceV1::new()
        .initialize_for_setup(repo.path())
        .unwrap();
    let repository =
        handbook_engine::artifact_repository::ArtifactRepositoryV1::open(repo.path()).unwrap();
    let target = handbook_engine::artifact_repository::ArtifactTargetV1::parse(
        "example.artifact-kind.registry-brief@1.0.0",
        "registry_brief",
    )
    .unwrap();
    let current = repository
        .current_artifact_fingerprint(&target)
        .unwrap()
        .unwrap();
    let intake_request = serde_json::to_vec(&json!({
        "idempotency_key": "markerless_intake_000001",
        "acquisition_mode": "express",
        "expected_current_artifact_fingerprint": current.as_str(),
        "coverage_submissions": [
            {"coverage_id":"registry_brief.title","state":"supplied","source_kind":"user_declaration","value":"title","specificity":"exact","confidence":"high","contradiction_refs":[]},
            {"coverage_id":"registry_brief.summary","state":"supplied","source_kind":"user_declaration","value":"summary","specificity":"concrete","confidence":"high","contradiction_refs":[]}
        ]
    }))
    .unwrap();
    let intake = handbook_engine::artifact_mutation::ArtifactMutationServiceV1::intake_append(
        repo.path(),
        KIND_REF,
        INSTANCE_ID,
        &intake_request,
    )
    .unwrap();
    let intake_output = &intake.result.authoritative_outputs[0];
    let preview =
        handbook_engine::artifact_mutation::ArtifactMutationServiceV1::candidate_validate(
            repo.path(),
            KIND_REF,
            INSTANCE_ID,
            &intake_output.relative_ref,
            &intake_output.fingerprint,
            Some(current.as_str()),
        )
        .unwrap();
    let candidate_request = serde_json::to_vec(&json!({
        "idempotency_key": "markerless_candidate_0001",
        "intake_record_ref": intake_output.relative_ref,
        "intake_record_fingerprint": intake_output.fingerprint,
        "expected_candidate_fingerprint": preview.candidate_fingerprint,
    }))
    .unwrap();
    let candidate =
        handbook_engine::artifact_mutation::ArtifactMutationServiceV1::candidate_append(
            repo.path(),
            KIND_REF,
            INSTANCE_ID,
            &candidate_request,
        )
        .unwrap();
    let candidate_output = &candidate.result.authoritative_outputs[0];
    let promotion_request = serde_json::to_vec(&json!({
        "idempotency_key": "markerless_promotion_0001",
        "candidate_ref": candidate_output.relative_ref,
        "candidate_fingerprint": candidate_output.fingerprint,
        "expected_current_artifact_fingerprint": current.as_str(),
    }))
    .unwrap();
    handbook_engine::artifact_mutation::ArtifactMutationServiceV1::promote(
        repo.path(),
        KIND_REF,
        INSTANCE_ID,
        &promotion_request,
    )
    .unwrap();
    let family = repo
        .path()
        .join(".handbook/state/transactions/artifact-promotions");
    let committed = fs::read_dir(&family)
        .unwrap()
        .map(|entry| entry.unwrap().path())
        .find(|path| path.extension().is_some_and(|value| value == "committed"))
        .expect("committed promotion");
    let intent: Value =
        serde_json::from_slice(&fs::read(committed.join("intent.json")).unwrap()).unwrap();
    let transaction_id = intent["transaction_id"].as_str().unwrap();
    let key = intent["domain_mutation_key_fingerprint"]
        .as_str()
        .unwrap()
        .strip_prefix("sha256:")
        .unwrap();
    fs::remove_file(committed.join("commit-marker.json")).unwrap();
    fs::remove_file(committed.join("evidence.json")).unwrap();
    fs::remove_file(
        repo.path()
            .join(".handbook/state/idempotency/generic-artifact-operations/results")
            .join(format!("{transaction_id}.json")),
    )
    .unwrap();
    fs::remove_file(
        repo.path()
            .join(".handbook/state/idempotency/generic-artifact-operations/ledger")
            .join(format!("{key}.json")),
    )
    .unwrap();
    let pending = committed.with_extension("pending");
    fs::rename(committed, &pending).unwrap();
    let pending = pending_dir(
        repo.path(),
        GenericArtifactOperationV1::ArtifactCandidatePromote,
    );
    (repo, repository, target, pending)
}

#[test]
fn ordinary_repository_read_recovers_markerless_installed_promotion_first() {
    let (repo, repository, target, pending) = markerless_repository();

    let read = repository
        .read(target.kind_ref(), target.instance_id())
        .expect("ordinary read performs recovery while both authority locks remain held");
    assert_eq!(
        DefinitionFingerprint::from_json_value(&read.content)
            .unwrap()
            .as_str()
            .len(),
        71
    );
    assert!(!pending.exists());
    assert!(fs::read_dir(
        repo.path()
            .join(".handbook/state/transactions/artifact-promotions")
    )
    .unwrap()
    .any(|entry| entry
        .unwrap()
        .path()
        .extension()
        .is_some_and(|ext| ext == "committed")));
}

#[test]
fn every_public_repository_metadata_read_recovers_before_returning_owned_data() {
    for operation in [
        "selection",
        "kinds",
        "instances",
        "context",
        "intake",
        "canonical",
    ] {
        let (_repo, repository, target, pending) = markerless_repository();
        match operation {
            "selection" => {
                repository.selection_fingerprint().expect("selection read");
            }
            "kinds" => {
                repository.list_kinds().expect("kind list");
            }
            "instances" => {
                repository.list_instances().expect("instance list");
            }
            "context" => {
                repository
                    .operation_context(target.kind_ref(), target.instance_id())
                    .expect("operation context");
            }
            "intake" => {
                repository
                    .intake_definition(&target)
                    .expect("intake definition");
            }
            "canonical" => {
                repository.canonical_path(&target).expect("canonical path");
            }
            _ => unreachable!(),
        }
        assert!(!pending.exists(), "{operation} bypassed recovery");
    }
}

#[test]
fn every_public_repository_read_refuses_changed_selection_authority_without_state_delta() {
    let fixture =
        Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/hcm_2_3_generic_custom_kind");
    for operation in [
        "selection",
        "kinds",
        "instances",
        "context",
        "read",
        "validate",
        "evaluate",
        "evaluate-document",
        "intake",
        "current",
        "canonical",
    ] {
        let repo = tempfile::tempdir().unwrap();
        copy_fixture_tree(&fixture, repo.path());
        RepositoryInvocationIdentityServiceV1::new()
            .initialize_for_setup(repo.path())
            .unwrap();
        let repository =
            handbook_engine::artifact_repository::ArtifactRepositoryV1::open(repo.path()).unwrap();
        let target =
            handbook_engine::artifact_repository::ArtifactTargetV1::parse(KIND_REF, INSTANCE_ID)
                .unwrap();
        fs::write(
            repo.path().join(".handbook/profile-selection.json"),
            b"{}\n",
        )
        .unwrap();
        let before = tree_bytes(repo.path());
        let result = match operation {
            "selection" => repository.selection_fingerprint().map(|_| ()),
            "kinds" => repository.list_kinds().map(|_| ()),
            "instances" => repository.list_instances().map(|_| ()),
            "context" => repository
                .operation_context(target.kind_ref(), target.instance_id())
                .map(|_| ()),
            "read" => repository
                .read(target.kind_ref(), target.instance_id())
                .map(|_| ()),
            "validate" => repository
                .validate(target.kind_ref(), target.instance_id())
                .map(|_| ()),
            "evaluate" => repository
                .evaluate_intake(
                    target.kind_ref(),
                    target.instance_id(),
                    handbook_engine::artifact_intake_registry::AcquisitionModeV1::Express,
                    None,
                    &[],
                )
                .map(|_| ()),
            "evaluate-document" => repository
                .evaluate_intake_document(
                    &target,
                    handbook_engine::artifact_intake_registry::AcquisitionModeV1::Express,
                    None,
                    b"{}\n",
                )
                .map(|_| ()),
            "intake" => repository.intake_definition(&target).map(|_| ()),
            "current" => repository.current_artifact_fingerprint(&target).map(|_| ()),
            "canonical" => repository.canonical_path(&target).map(|_| ()),
            _ => unreachable!(),
        };
        let error = result.expect_err("changed selection authority must fail closed");
        assert_eq!(
            error.kind(),
            handbook_engine::artifact_repository::ArtifactRepositoryErrorKindV1::SelectionAdmission,
            "{operation}"
        );
        assert_eq!(before, tree_bytes(repo.path()), "{operation} changed state");
    }
}

#[test]
fn public_reads_refuse_refingerprinted_journal_and_runtime_authority_forgery() {
    let (base, _repository, _target, _pending) = markerless_repository();
    for mutation in ["identity", "context", "canonical", "runtime-definition"] {
        let repo = tempfile::tempdir().unwrap();
        copy_fixture_tree(base.path(), repo.path());
        let pending = pending_dir(
            repo.path(),
            GenericArtifactOperationV1::ArtifactCandidatePromote,
        );
        let intent_path = pending.join("intent.json");
        let mut runtime_replacement = None;
        if mutation == "runtime-definition" {
            let intent: Value = serde_json::from_slice(&fs::read(&intent_path).unwrap()).unwrap();
            let old_ref = intent["outputs"][1]["final_ref"]
                .as_str()
                .unwrap()
                .to_owned();
            let old_path = repo.path().join(&old_ref);
            let mut promotion: Value =
                serde_json::from_slice(&fs::read(&old_path).unwrap()).unwrap();
            promotion["resolved_definitions"][0]["definition_fingerprint"] =
                json!("sha256:abababababababababababababababababababababababababababababababab");
            promotion.as_object_mut().unwrap().remove("promotion_id");
            promotion
                .as_object_mut()
                .unwrap()
                .remove("promotion_fingerprint");
            let promotion_fingerprint =
                sha256(&serde_json_canonicalizer::to_vec(&promotion).unwrap());
            let promotion_hex = promotion_fingerprint.strip_prefix("sha256:").unwrap();
            promotion["promotion_id"] = json!(format!("promotion_{promotion_hex}"));
            promotion["promotion_fingerprint"] = json!(promotion_fingerprint);
            let bytes = jcs_lf(&promotion);
            let new_ref = format!(
                ".handbook/state/artifacts/{INSTANCE_ID}/promotion-records/promotion_{promotion_hex}.json"
            );
            let new_path = repo.path().join(&new_ref);
            fs::write(&new_path, &bytes).unwrap();
            fs::remove_file(old_path).unwrap();
            let staged = pending.join("staged/1-promotion-record.bin");
            if staged.exists() {
                fs::write(staged, &bytes).unwrap();
            }
            runtime_replacement = Some((new_ref, bytes));
        }
        rewrite_control(&intent_path, "intent_fingerprint", |value| {
            match mutation {
                "identity" => {
                    value["repository_identity_fingerprint"] = json!(
                        "sha256:aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"
                    );
                }
                "context" => {
                    let forged = json!(
                        "sha256:bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb"
                    );
                    value["operation_context_fingerprint"] = forged.clone();
                    value["request_subject"]["operation_context_fingerprint"] = forged;
                }
                "canonical" => {
                    value["request_subject"]["canonical_artifact_ref"] =
                        json!(".handbook/project/charter.yaml");
                    value["outputs"][0]["final_ref"] = json!(".handbook/project/charter.yaml");
                }
                "runtime-definition" => {
                    let (new_ref, bytes) = runtime_replacement.as_ref().unwrap();
                    value["outputs"][1]["final_ref"] = json!(new_ref);
                    value["outputs"][1]["bytes_sha256"] = json!(sha256(bytes));
                    value["outputs"][1]["byte_length"] = json!(bytes.len());
                }
                _ => unreachable!(),
            }
            refingerprint_forged_intent(value);
        });
        if let Some((new_ref, bytes)) = &runtime_replacement {
            let intent: Value = serde_json::from_slice(&fs::read(&intent_path).unwrap()).unwrap();
            rewrite_control(
                &pending.join("verified.json"),
                "verified_fingerprint",
                |value| {
                    value["intent_fingerprint"] = intent["intent_fingerprint"].clone();
                    value["staged_outputs"][1]["bytes_sha256"] = json!(sha256(bytes));
                    value["staged_outputs"][1]["byte_length"] = json!(bytes.len());
                    if !value["staged_outputs"][1]["staged_ref"]
                        .as_str()
                        .is_some_and(|reference| reference.starts_with("staged/"))
                    {
                        value["staged_outputs"][1]["staged_ref"] = json!(new_ref);
                    }
                },
            );
        }
        let repository =
            handbook_engine::artifact_repository::ArtifactRepositoryV1::open(repo.path()).unwrap();
        let before = tree_bytes(repo.path());
        let error = repository
            .list_kinds()
            .expect_err("forged persisted authority must fail before recovery");
        assert_eq!(
            error.kind(),
            handbook_engine::artifact_repository::ArtifactRepositoryErrorKindV1::ArtifactRead,
            "{mutation}"
        );
        assert_eq!(before, tree_bytes(repo.path()), "{mutation} changed state");
    }
}

#[test]
fn intake_commit_replays_exact_result_and_persists_no_raw_key() {
    let repo = tempfile::tempdir().unwrap();
    let store = GenericArtifactLineageStoreV1::new(repo.path());
    let request = request(
        GenericArtifactOperationV1::IntakeRecordAppend,
        "intake_key_00000001",
    );
    let first = execute(&store, request.clone(), intake_plan(FIXED_NOW));
    assert_eq!(first.disposition, GenericExecutionDispositionV1::Committed);
    assert_eq!(first.result.authoritative_outputs.len(), 1);
    assert!(first.result.internal_transaction_evidence_ref.is_some());

    let replay = store
        .execute_at_for_testing(request.clone(), "2027-01-01T00:00:00Z", |_| {
            panic!("retained replay must precede fresh evaluation")
        })
        .unwrap();
    assert_eq!(replay.disposition, GenericExecutionDispositionV1::Replayed);
    assert_eq!(replay.result, first.result);

    let all_bytes = tree_bytes(repo.path())
        .into_values()
        .flatten()
        .collect::<Vec<_>>();
    assert!(!String::from_utf8_lossy(&all_bytes).contains("intake_key_00000001"));
}

#[test]
fn every_installed_subset_recovers_the_exact_missing_complement() {
    for (operation, outputs, make_plan) in [
        (
            GenericArtifactOperationV1::IntakeRecordAppend,
            3usize,
            intake_plan as fn(&str) -> GenericPlannedOutcomeV1,
        ),
        (
            GenericArtifactOperationV1::ArtifactCandidateAppend,
            3usize,
            |_| candidate_plan(),
        ),
        (
            GenericArtifactOperationV1::ArtifactCandidatePromote,
            2usize,
            |_| promotion_plan(None),
        ),
    ] {
        for subset in 0..(1usize << outputs) {
            let repo = tempfile::tempdir().unwrap();
            let store = GenericArtifactLineageStoreV1::new(repo.path());
            let key = format!("subset_key_{outputs}_{subset:08}");
            let request = request(operation, &key);
            let plan = make_plan(FIXED_NOW);
            let failure = store
                .execute_at_with_fault_for_testing(
                    request.clone(),
                    FIXED_NOW,
                    GenericLineageFaultPointV1::AfterVerified,
                    {
                        let plan = plan.clone();
                        move |_| Ok(plan)
                    },
                )
                .unwrap_err();
            assert_eq!(
                failure.kind(),
                GenericLineageStoreErrorKindV1::InjectedFault
            );
            install_subset(repo.path(), &plan, subset);

            let fresh = GenericArtifactLineageStoreV1::new(repo.path());
            if operation == GenericArtifactOperationV1::ArtifactCandidatePromote && subset & 1 != 0
            {
                let before_recovery = tree_bytes(repo.path());
                let error = fresh
                    .execute_at_for_testing(request, FIXED_NOW, |_| {
                        panic!("conflicting recovery must precede fresh evaluation")
                    })
                    .unwrap_err();
                assert_eq!(
                    error.kind(),
                    GenericLineageStoreErrorKindV1::ConflictingTransactionState
                );
                assert_eq!(before_recovery, tree_bytes(repo.path()));
                continue;
            }
            let result = execute(&fresh, request, plan.clone());
            assert_eq!(result.disposition, GenericExecutionDispositionV1::Replayed);
            let GenericPlannedOutcomeV1::Commit(plan) = plan else {
                unreachable!()
            };
            for output in plan.outputs {
                assert_eq!(
                    fs::read(repo.path().join(output.final_ref)).unwrap(),
                    output.bytes
                );
            }
        }
    }
}

#[test]
fn promotion_recovery_replaces_the_exact_persisted_basis_after_staging() {
    let repo = tempfile::tempdir().unwrap();
    let canonical = repo.path().join(".handbook/project/registry-brief.yaml");
    fs::create_dir_all(canonical.parent().unwrap()).unwrap();
    let old_bytes = b"artifact_kind: previous\n";
    fs::write(&canonical, old_bytes).unwrap();
    let expected_basis = sha256(old_bytes);
    let request = promotion_request("promotion_recovery_basis_01", Some(&expected_basis));
    let plan = promotion_plan(Some(&expected_basis));
    let store = GenericArtifactLineageStoreV1::new(repo.path());
    let failure = store
        .execute_at_with_fault_for_testing(
            request.clone(),
            FIXED_NOW,
            GenericLineageFaultPointV1::AfterVerified,
            {
                let plan = plan.clone();
                move |_| Ok(plan)
            },
        )
        .unwrap_err();
    assert_eq!(
        failure.kind(),
        GenericLineageStoreErrorKindV1::InjectedFault,
        "{failure:?}"
    );

    let fresh = GenericArtifactLineageStoreV1::new(repo.path());
    let replay = fresh
        .execute_at_for_testing(request, FIXED_NOW, |_| {
            panic!("established promotion must not be re-evaluated")
        })
        .unwrap();
    assert_eq!(replay.disposition, GenericExecutionDispositionV1::Replayed);
    let GenericPlannedOutcomeV1::Commit(plan) = plan else {
        unreachable!()
    };
    assert_eq!(fs::read(canonical).unwrap(), plan.outputs[0].bytes);
}

#[test]
fn every_commit_staging_install_and_suffix_fault_recovers_to_exact_replay() {
    for (index, point) in [
        GenericLineageFaultPointV1::AfterVerified,
        GenericLineageFaultPointV1::AfterInstall(0),
        GenericLineageFaultPointV1::AfterInstall(1),
        GenericLineageFaultPointV1::AfterInstall(2),
        GenericLineageFaultPointV1::AfterInstalledVerification,
        GenericLineageFaultPointV1::AfterMarker,
        GenericLineageFaultPointV1::AfterEvidence,
        GenericLineageFaultPointV1::AfterResult,
        GenericLineageFaultPointV1::AfterLedger,
        GenericLineageFaultPointV1::AfterCommittedRename,
    ]
    .into_iter()
    .enumerate()
    {
        let repo = tempfile::tempdir().unwrap();
        let store = GenericArtifactLineageStoreV1::new(repo.path());
        let request = request(
            GenericArtifactOperationV1::ArtifactCandidateAppend,
            &format!("commit_prefix_key_{index:08}"),
        );
        let failure = store
            .execute_at_with_fault_for_testing(request.clone(), FIXED_NOW, point, |_| {
                Ok(candidate_plan())
            })
            .unwrap_err();
        assert_eq!(
            failure.kind(),
            GenericLineageStoreErrorKindV1::InjectedFault
        );

        let fresh = GenericArtifactLineageStoreV1::new(repo.path());
        let replay = fresh
            .execute_at_for_testing(request, FIXED_NOW, |_| {
                panic!("established commit must not be re-evaluated")
            })
            .unwrap();
        assert_eq!(replay.disposition, GenericExecutionDispositionV1::Replayed);
        assert_eq!(replay.result.outcome, "committed");
    }
}

#[test]
fn commit_establishment_only_prefixes_move_the_intent_then_refuse_incomplete_staging() {
    for (index, point) in [
        GenericLineageFaultPointV1::AfterEstablishedIntent,
        GenericLineageFaultPointV1::AfterPendingIntent,
    ]
    .into_iter()
    .enumerate()
    {
        let repo = tempfile::tempdir().unwrap();
        let store = GenericArtifactLineageStoreV1::new(repo.path());
        let failure = store
            .execute_at_with_fault_for_testing(
                request(
                    GenericArtifactOperationV1::ArtifactCandidateAppend,
                    &format!("commit_establishment_prefix_{index:08}"),
                ),
                FIXED_NOW,
                point,
                |_| Ok(candidate_plan()),
            )
            .unwrap_err();
        assert_eq!(
            failure.kind(),
            GenericLineageStoreErrorKindV1::InjectedFault
        );
        assert!(GenericArtifactLineageStoreV1::new(repo.path())
            .recover()
            .is_err());
        let pending = pending_dir(
            repo.path(),
            GenericArtifactOperationV1::ArtifactCandidateAppend,
        );
        assert!(pending.join("intent.json").is_file());
        assert!(!repo
            .path()
            .join(".handbook/evidence/artifacts/registry_brief")
            .exists());
        assert!(!repo
            .path()
            .join(".handbook/state/idempotency/generic-artifact-operations/results")
            .exists());
        assert!(!repo
            .path()
            .join(".handbook/state/idempotency/generic-artifact-operations/ledger")
            .exists());
    }
}

#[test]
fn commit_suffix_non_prefixes_refuse_without_mutation() {
    for missing in ["evidence", "result"] {
        let repo = tempfile::tempdir().unwrap();
        let store = GenericArtifactLineageStoreV1::new(repo.path());
        let request = request(
            GenericArtifactOperationV1::ArtifactCandidateAppend,
            &format!("commit_non_prefix_{missing:0<16}"),
        );
        let failure = store
            .execute_at_with_fault_for_testing(
                request,
                FIXED_NOW,
                GenericLineageFaultPointV1::AfterLedger,
                |_| Ok(candidate_plan()),
            )
            .unwrap_err();
        assert_eq!(
            failure.kind(),
            GenericLineageStoreErrorKindV1::InjectedFault
        );
        let pending = pending_dir(
            repo.path(),
            GenericArtifactOperationV1::ArtifactCandidateAppend,
        );
        let intent: Value =
            serde_json::from_slice(&fs::read(pending.join("intent.json")).unwrap()).unwrap();
        if missing == "evidence" {
            fs::remove_file(pending.join("evidence.json")).unwrap();
        } else {
            fs::remove_file(
                repo.path()
                    .join(".handbook/state/idempotency/generic-artifact-operations/results")
                    .join(format!(
                        "{}.json",
                        intent["transaction_id"].as_str().unwrap()
                    )),
            )
            .unwrap();
        }
        let before = tree_bytes(repo.path());

        assert!(GenericArtifactLineageStoreV1::new(repo.path())
            .recover()
            .is_err());
        assert_eq!(tree_bytes(repo.path()), before);
    }
}

#[test]
fn unknown_pending_inventory_refuses_without_mutation() {
    let repo = tempfile::tempdir().unwrap();
    let store = GenericArtifactLineageStoreV1::new(repo.path());
    let request = request(
        GenericArtifactOperationV1::ArtifactCandidateAppend,
        "inventory_key_0001",
    );
    let failure = store
        .execute_at_with_fault_for_testing(
            request,
            FIXED_NOW,
            GenericLineageFaultPointV1::AfterVerified,
            |_| Ok(candidate_plan()),
        )
        .unwrap_err();
    assert_eq!(
        failure.kind(),
        GenericLineageStoreErrorKindV1::InjectedFault
    );
    fs::write(
        pending_dir(
            repo.path(),
            GenericArtifactOperationV1::ArtifactCandidateAppend,
        )
        .join("unknown.bin"),
        b"attacker",
    )
    .unwrap();
    let before = tree_bytes(repo.path());

    let recovery = GenericArtifactLineageStoreV1::new(repo.path()).recover();
    assert_eq!(
        recovery.unwrap_err().kind(),
        GenericLineageStoreErrorKindV1::ConflictingTransactionState
    );
    assert_eq!(tree_bytes(repo.path()), before);
}

#[test]
fn every_established_refusal_prefix_resumes_outputless_with_exact_evidence() {
    let points = [
        GenericLineageFaultPointV1::AfterEstablishedIntent,
        GenericLineageFaultPointV1::AfterPendingIntent,
        GenericLineageFaultPointV1::AfterVerified,
        GenericLineageFaultPointV1::AfterMarker,
        GenericLineageFaultPointV1::AfterEvidence,
        GenericLineageFaultPointV1::AfterResult,
        GenericLineageFaultPointV1::AfterLedger,
    ];
    for (index, point) in points.into_iter().enumerate() {
        let repo = tempfile::tempdir().unwrap();
        let store = GenericArtifactLineageStoreV1::new(repo.path());
        let request = request(
            GenericArtifactOperationV1::ArtifactCandidatePromote,
            &format!("refusal_key_{index:08}"),
        );
        let failure = store
            .execute_at_with_fault_for_testing(request.clone(), FIXED_NOW, point, |_| {
                Ok(refusal_plan())
            })
            .unwrap_err();
        assert_eq!(
            failure.kind(),
            GenericLineageStoreErrorKindV1::InjectedFault
        );

        let fresh = GenericArtifactLineageStoreV1::new(repo.path());
        let replay = fresh
            .execute_at_for_testing(request, FIXED_NOW, |_| {
                panic!("established refusal must not be re-evaluated")
            })
            .unwrap();
        assert_eq!(replay.disposition, GenericExecutionDispositionV1::Replayed);
        assert_eq!(replay.result.outcome, "refused");
        assert!(replay.result.authoritative_outputs.is_empty());
        assert!(replay.result.internal_transaction_evidence_ref.is_some());
        assert!(!domain_paths(repo.path())
            .iter()
            .any(|path| path.contains("/artifacts/")));
    }
}

#[test]
fn retained_result_expires_to_non_expiring_tombstone() {
    let repo = tempfile::tempdir().unwrap();
    let store = GenericArtifactLineageStoreV1::new(repo.path());
    let exact = request(
        GenericArtifactOperationV1::IntakeRecordAppend,
        "tombstone_key_001",
    );
    execute(&store, exact.clone(), intake_plan(FIXED_NOW));
    store
        .expire_retained_result_at_for_testing(&exact, "2026-08-21T00:00:00Z")
        .unwrap();

    let exact_failure = store
        .execute_at_for_testing(exact.clone(), "2030-01-01T00:00:00Z", |_| {
            panic!("tombstone must precede evaluation")
        })
        .unwrap_err();
    assert_eq!(
        exact_failure.kind(),
        GenericLineageStoreErrorKindV1::IdempotencyExpired
    );

    let mut different = exact;
    different.request_subject["coverage_input_fingerprint"] = Value::String(
        "sha256:9999999999999999999999999999999999999999999999999999999999999999".to_owned(),
    );
    let conflict = store
        .execute_at_for_testing(different, "2030-01-01T00:00:00Z", |_| {
            panic!("different request under tombstone must not evaluate")
        })
        .unwrap_err();
    assert_eq!(
        conflict.kind(),
        GenericLineageStoreErrorKindV1::IdempotencyConflict
    );
}

#[test]
fn promotion_compare_and_write_refuses_a_stale_absent_basis_before_establishment() {
    let repo = tempfile::tempdir().unwrap();
    let store = GenericArtifactLineageStoreV1::new(repo.path());
    let first = request(
        GenericArtifactOperationV1::ArtifactCandidatePromote,
        "promotion_key_001",
    );
    execute(&store, first, promotion_plan(None));

    let stale = request(
        GenericArtifactOperationV1::ArtifactCandidatePromote,
        "promotion_key_002",
    );
    let before = tree_bytes(repo.path());
    let failure = store
        .execute_at_for_testing(stale, FIXED_NOW, |_| Ok(promotion_plan(None)))
        .unwrap_err();
    assert_eq!(
        failure.kind(),
        GenericLineageStoreErrorKindV1::BasisMismatch
    );
    assert_eq!(tree_bytes(repo.path()), before);
}

#[test]
fn output_bearing_refusal_is_rejected_before_establishment() {
    let repo = tempfile::tempdir().unwrap();
    let store = GenericArtifactLineageStoreV1::new(repo.path());
    let request = request(
        GenericArtifactOperationV1::ArtifactCandidatePromote,
        "bad_refusal_key_01",
    );
    let mut invalid = refusal_plan();
    if let GenericPlannedOutcomeV1::Refuse(plan) = &mut invalid {
        plan.refusal.expected_fingerprint = Some(
            "sha256:aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa".to_owned(),
        );
        plan.refusal.observed_fingerprint = Some(
            "sha256:aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa".to_owned(),
        );
    }
    let failure = store
        .execute_at_for_testing(request, FIXED_NOW, |_| Ok(invalid))
        .unwrap_err();
    assert_eq!(
        failure.kind(),
        GenericLineageStoreErrorKindV1::InvalidEstablishedRefusal
    );
    assert!(domain_paths(repo.path())
        .iter()
        .all(|path| !path.contains("idempotency/generic-artifact-operations")));
}

#[test]
fn same_key_same_request_race_commits_once_and_replays_once() {
    let repo = tempfile::tempdir().unwrap();
    let root = repo.path().to_path_buf();
    let barrier = Arc::new(Barrier::new(3));
    let mut handles = Vec::new();
    for _ in 0..2 {
        let root = root.clone();
        let barrier = barrier.clone();
        handles.push(thread::spawn(move || {
            let store = GenericArtifactLineageStoreV1::new(root);
            let request = request(
                GenericArtifactOperationV1::IntakeRecordAppend,
                "concurrent_key_01",
            );
            barrier.wait();
            execute(&store, request, intake_plan(FIXED_NOW)).disposition
        }));
    }
    barrier.wait();
    let mut outcomes = handles
        .into_iter()
        .map(|handle| handle.join().unwrap())
        .collect::<Vec<_>>();
    outcomes.sort();
    assert_eq!(
        outcomes,
        vec![
            GenericExecutionDispositionV1::Committed,
            GenericExecutionDispositionV1::Replayed
        ]
    );
}

#[test]
fn same_key_different_request_race_commits_once_and_conflicts_once() {
    let repo = tempfile::tempdir().unwrap();
    let root = repo.path().to_path_buf();
    let barrier = Arc::new(Barrier::new(3));
    let evaluations = Arc::new(AtomicUsize::new(0));
    let mut handles = Vec::new();
    for index in 0..2 {
        let root = root.clone();
        let barrier = barrier.clone();
        let evaluations = evaluations.clone();
        handles.push(thread::spawn(move || {
            let store = GenericArtifactLineageStoreV1::new(root);
            let mut request = request(
                GenericArtifactOperationV1::IntakeRecordAppend,
                "concurrent_different_key_01",
            );
            request.request_subject["coverage_input_fingerprint"] = Value::String(format!(
                "sha256:{}",
                if index == 0 {
                    "8".repeat(64)
                } else {
                    "9".repeat(64)
                }
            ));
            barrier.wait();
            store.execute_at_for_testing(request, FIXED_NOW, |_| {
                evaluations.fetch_add(1, Ordering::SeqCst);
                Ok(intake_plan(FIXED_NOW))
            })
        }));
    }
    barrier.wait();
    let outcomes = handles
        .into_iter()
        .map(|handle| handle.join().unwrap())
        .collect::<Vec<_>>();
    assert_eq!(outcomes.iter().filter(|outcome| outcome.is_ok()).count(), 1);
    assert_eq!(
        outcomes
            .iter()
            .filter(|outcome| outcome.as_ref().is_err_and(|error| {
                error.kind() == GenericLineageStoreErrorKindV1::IdempotencyConflict
            }))
            .count(),
        1
    );
    assert_eq!(evaluations.load(Ordering::SeqCst), 1);
}

#[test]
fn persisted_records_are_jcs_lf_and_vector_fingerprints_are_preserved() {
    let repo = tempfile::tempdir().unwrap();
    let store = GenericArtifactLineageStoreV1::new(repo.path());
    execute(
        &store,
        request(
            GenericArtifactOperationV1::ArtifactCandidateAppend,
            "canonical_key_001",
        ),
        candidate_plan(),
    );
    for (relative, bytes) in tree_bytes(repo.path()) {
        if relative.ends_with(".json") {
            assert_eq!(bytes.last(), Some(&b'\n'), "{relative}");
            let value: Value = serde_json::from_slice(&bytes).expect("JSON record");
            assert_eq!(jcs_lf(&value), bytes, "{relative}");
        }
    }
    let candidate = fs::read(
        repo.path().join(
            ".handbook/evidence/artifacts/registry_brief/candidates/\
             candidate_dad8bb3c473f18ea28d6fd006f98a43d8574b5c031c05d0c6f2b14fcdf21d57c.json"
                .replace(" ", ""),
        ),
    )
    .unwrap();
    assert_eq!(
        sha256(&candidate),
        "sha256:448683872456a2a472b50f442abf9abf3c68b90f6c243b3cfa048e61c037f0f9"
    );
}

#[test]
fn frozen_control_vectors_validate_and_reproduce_the_complete_identity_chain() {
    let schema = control_schema();
    jsonschema::draft202012::meta::validate(&schema).expect("control schema meta-validation");
    let validator = jsonschema::draft202012::options()
        .with_pattern_options(jsonschema::PatternOptions::fancy_regex())
        .build(&schema)
        .expect("control schema validator");
    let vectors = control_vectors();
    for vector in vectors["positive_vectors"].as_array().unwrap() {
        let id = vector["vector_id"].as_str().unwrap();
        let record = &vector["record"];
        let schema_errors = validator
            .iter_errors(record)
            .map(|error| error.to_string())
            .collect::<Vec<_>>();
        assert!(
            schema_errors.is_empty(),
            "schema-invalid vector: {id}: {schema_errors:?}"
        );
        let terminal = vector["identity_excludes"][0].as_str().unwrap();
        let declared = record[terminal].as_str().unwrap();
        let mut preimage = record.clone();
        preimage.as_object_mut().unwrap().remove(terminal);
        assert_eq!(
            sha256(&serde_json_canonicalizer::to_vec(&preimage).unwrap()),
            declared,
            "terminal fingerprint: {id}"
        );
        if record["schema_id"] == "handbook.generic-transaction-intent" {
            let operation = match record["operation_id"].as_str().unwrap() {
                "intake.record.append" => GenericArtifactOperationV1::IntakeRecordAppend,
                "artifact.candidate.append" => GenericArtifactOperationV1::ArtifactCandidateAppend,
                "artifact.candidate.promote" => {
                    GenericArtifactOperationV1::ArtifactCandidatePromote
                }
                value => panic!("unknown operation: {value}"),
            };
            let transaction_preimage = json!({
                "repository_identity_fingerprint": record["repository_identity_fingerprint"],
                "owner_contract_ref": record["owner_contract_ref"],
                "owner_contract_subject_fingerprint": record["owner_contract_subject_fingerprint"],
                "operation_id": record["operation_id"],
                "domain_mutation_key_fingerprint": record["domain_mutation_key_fingerprint"],
                "request_fingerprint": record["request_fingerprint"],
            });
            let digest = sha256(&serde_json_canonicalizer::to_vec(&transaction_preimage).unwrap());
            assert_eq!(
                record["transaction_id"],
                format!(
                    "{}_{}",
                    operation.transaction_token(),
                    digest.strip_prefix("sha256:").unwrap()
                ),
                "transaction identity: {id}"
            );
        }
    }
}

#[test]
fn frozen_runtime_vectors_validate_and_reproduce_record_identities() {
    let schema = runtime_schema();
    jsonschema::draft202012::meta::validate(&schema).expect("runtime schema meta-validation");
    let validator = jsonschema::draft202012::options()
        .with_pattern_options(jsonschema::PatternOptions::fancy_regex())
        .build(&schema)
        .expect("runtime schema validator");
    for vector in runtime_vectors()["records"].as_array().unwrap() {
        let class = vector["record_class"].as_str().unwrap();
        let record = &vector["record"];
        assert!(validator.is_valid(record), "schema-invalid record: {class}");
        let (id_field, fingerprint_field, prefix) = match class {
            "intake-1.2" => ("intake_record_id", "record_fingerprint", "intake"),
            "validation-result-1.0" => (
                "validation_result_id",
                "validation_result_fingerprint",
                "validation",
            ),
            "candidate-1.4" => ("candidate_id", "candidate_fingerprint", "candidate"),
            "promotion-1.2" => ("promotion_id", "promotion_fingerprint", "promotion"),
            value => panic!("unknown runtime record: {value}"),
        };
        let declared = record[fingerprint_field].as_str().unwrap();
        let mut preimage = record.clone();
        preimage.as_object_mut().unwrap().remove(id_field);
        preimage.as_object_mut().unwrap().remove(fingerprint_field);
        assert_eq!(
            sha256(&serde_json_canonicalizer::to_vec(&preimage).unwrap()),
            declared,
            "runtime fingerprint: {class}"
        );
        assert_eq!(
            record[id_field],
            format!("{prefix}_{}", declared.strip_prefix("sha256:").unwrap()),
            "runtime ID: {class}"
        );
    }
}

#[test]
fn recovery_rejects_a_refingerprinted_intent_with_a_forged_transaction_id() {
    let repo = tempfile::tempdir().unwrap();
    let store = GenericArtifactLineageStoreV1::new(repo.path());
    let failure = store
        .execute_at_with_fault_for_testing(
            request(
                GenericArtifactOperationV1::ArtifactCandidatePromote,
                "forged_transaction_0001",
            ),
            FIXED_NOW,
            GenericLineageFaultPointV1::AfterEstablishedIntent,
            |_| Ok(refusal_plan()),
        )
        .unwrap_err();
    assert_eq!(
        failure.kind(),
        GenericLineageStoreErrorKindV1::InjectedFault
    );

    let intent = establishing_intent(repo.path());
    rewrite_control(&intent, "intent_fingerprint", |value| {
        value["transaction_id"] = Value::String(format!(
            "{}_{}",
            GenericArtifactOperationV1::ArtifactCandidatePromote.transaction_token(),
            "f".repeat(64)
        ));
    });
    let error = store.recover().unwrap_err();
    assert_eq!(
        error.kind(),
        GenericLineageStoreErrorKindV1::InvalidTransactionIntent,
        "{}",
        error.detail()
    );
}

#[test]
fn recovery_rejects_establishing_filename_and_output_authority_forgery() {
    for mutation in ["filename", "output"] {
        let repo = tempfile::tempdir().unwrap();
        let store = GenericArtifactLineageStoreV1::new(repo.path());
        let operation = if mutation == "filename" {
            GenericArtifactOperationV1::ArtifactCandidatePromote
        } else {
            GenericArtifactOperationV1::ArtifactCandidateAppend
        };
        store
            .execute_at_with_fault_for_testing(
                request(operation, &format!("forged_{mutation}_00000001")),
                FIXED_NOW,
                GenericLineageFaultPointV1::AfterEstablishedIntent,
                |_| {
                    if mutation == "filename" {
                        Ok(refusal_plan())
                    } else {
                        Ok(candidate_plan())
                    }
                },
            )
            .unwrap_err();
        let intent = establishing_intent(repo.path());
        if mutation == "filename" {
            fs::rename(
                &intent,
                intent
                    .parent()
                    .unwrap()
                    .join(format!("{}.intent", "a".repeat(64))),
            )
            .unwrap();
        } else {
            rewrite_control(&intent, "intent_fingerprint", |value| {
                value["outputs"][2]["final_ref"] = json!(".handbook/project/charter.yaml");
            });
        }
        assert_eq!(
            store.recover().unwrap_err().kind(),
            GenericLineageStoreErrorKindV1::InvalidTransactionIntent,
            "{mutation}"
        );
        assert!(!repo.path().join(".handbook/project/charter.yaml").exists());
    }
}

#[test]
fn recovery_rejects_a_refingerprinted_schema_invalid_nested_runtime_record() {
    let mutation = "nested-schema";
    let repo = tempfile::tempdir().unwrap();
    let store = GenericArtifactLineageStoreV1::new(repo.path());
    store
        .execute_at_with_fault_for_testing(
            request(
                GenericArtifactOperationV1::ArtifactCandidateAppend,
                &format!("forged_runtime_{mutation}_0001"),
            ),
            FIXED_NOW,
            GenericLineageFaultPointV1::AfterVerified,
            |_| Ok(candidate_plan()),
        )
        .unwrap_err();
    let pending = pending_dir(
        repo.path(),
        GenericArtifactOperationV1::ArtifactCandidateAppend,
    );
    let candidate_stage = fs::read_dir(pending.join("staged"))
        .unwrap()
        .map(|entry| entry.unwrap().path())
        .find(|path| {
            path.file_name()
                .unwrap()
                .to_string_lossy()
                .contains("candidate-record")
        })
        .expect("candidate stage");
    let mut candidate: Value =
        serde_json::from_slice(&fs::read(&candidate_stage).unwrap()).unwrap();
    candidate["field_sources"][0]["source_kind"] = json!("evidence");
    candidate.as_object_mut().unwrap().remove("candidate_id");
    candidate
        .as_object_mut()
        .unwrap()
        .remove("candidate_fingerprint");
    let candidate_fingerprint =
        sha256(&serde_json_canonicalizer::to_vec(&candidate).expect("candidate JCS"));
    let candidate_hex = candidate_fingerprint.strip_prefix("sha256:").unwrap();
    candidate["candidate_id"] = json!(format!("candidate_{candidate_hex}"));
    candidate["candidate_fingerprint"] = json!(candidate_fingerprint);
    let candidate_bytes = jcs_lf(&candidate);
    fs::write(&candidate_stage, &candidate_bytes).unwrap();

    let intent_path = pending.join("intent.json");
    rewrite_control(&intent_path, "intent_fingerprint", |value| {
        value["request_subject"]["expected_candidate_fingerprint"] = json!(candidate_fingerprint);
        value["outputs"][2]["final_ref"] = json!(format!(
            ".handbook/evidence/artifacts/{INSTANCE_ID}/candidates/candidate_{candidate_hex}.json"
        ));
        value["outputs"][2]["bytes_sha256"] = json!(sha256(&candidate_bytes));
        value["outputs"][2]["byte_length"] = json!(candidate_bytes.len());
        refingerprint_forged_intent(value);
    });
    let intent: Value = serde_json::from_slice(&fs::read(&intent_path).unwrap()).unwrap();
    let transaction_id = intent["transaction_id"].as_str().unwrap().to_owned();
    let intent_fingerprint = intent["intent_fingerprint"].as_str().unwrap().to_owned();
    rewrite_control(
        &pending.join("verified.json"),
        "verified_fingerprint",
        |value| {
            value["transaction_id"] = json!(transaction_id);
            value["intent_fingerprint"] = json!(intent_fingerprint);
            value["staged_outputs"][2]["bytes_sha256"] = json!(sha256(&candidate_bytes));
            value["staged_outputs"][2]["byte_length"] = json!(candidate_bytes.len());
        },
    );
    let renamed = pending
        .parent()
        .unwrap()
        .join(format!("{transaction_id}.pending"));
    fs::rename(&pending, &renamed).unwrap();

    let error = store.recover().unwrap_err();
    assert_eq!(
        error.kind(),
        GenericLineageStoreErrorKindV1::InvalidTransactionIntent,
        "{mutation}: {}",
        error.detail()
    );
}

#[test]
fn recovery_admits_only_the_exact_empty_pending_shell_for_an_established_intent() {
    let repo = tempfile::tempdir().unwrap();
    let store = GenericArtifactLineageStoreV1::new(repo.path());
    let request = request(
        GenericArtifactOperationV1::ArtifactCandidatePromote,
        "empty_pending_shell_0001",
    );
    store
        .execute_at_with_fault_for_testing(
            request.clone(),
            FIXED_NOW,
            GenericLineageFaultPointV1::AfterEstablishedIntent,
            |_| Ok(refusal_plan()),
        )
        .unwrap_err();
    let intent: Value =
        serde_json::from_slice(&fs::read(establishing_intent(repo.path())).unwrap())
            .expect("intent JSON");
    let pending = repo
        .path()
        .join(".handbook/state/transactions/artifact-promotions")
        .join(format!(
            "{}.pending",
            intent["transaction_id"].as_str().expect("transaction ID")
        ));
    fs::create_dir_all(&pending).unwrap();

    store.recover().expect("empty pending shell resumes");
    let replay = store
        .execute_at_for_testing(request, FIXED_NOW, |_| {
            panic!("recovered refusal must replay without evaluation")
        })
        .unwrap();
    assert_eq!(replay.disposition, GenericExecutionDispositionV1::Replayed);
    assert_eq!(replay.result.outcome, "refused");
}

#[test]
fn recovery_refuses_an_incomplete_deterministic_canonical_scratch_without_mutation() {
    let repo = tempfile::tempdir().unwrap();
    let canonical = repo.path().join(".handbook/project/registry-brief.yaml");
    fs::create_dir_all(canonical.parent().unwrap()).unwrap();
    let old_bytes = b"artifact_kind: previous\n";
    fs::write(&canonical, old_bytes).unwrap();
    let expected_basis = sha256(old_bytes);
    let request = promotion_request("recover_replace_scratch_0001", Some(&expected_basis));
    let plan = promotion_plan(Some(&expected_basis));
    let store = GenericArtifactLineageStoreV1::new(repo.path());
    store
        .execute_at_with_fault_for_testing(
            request.clone(),
            FIXED_NOW,
            GenericLineageFaultPointV1::AfterVerified,
            {
                let plan = plan.clone();
                move |_| Ok(plan)
            },
        )
        .unwrap_err();
    let transaction = pending_dir(
        repo.path(),
        GenericArtifactOperationV1::ArtifactCandidatePromote,
    );
    let publication: Value =
        serde_json::from_slice(&fs::read(transaction.join("native-publication.json")).unwrap())
            .unwrap();
    let candidate = repo
        .path()
        .join(publication["candidate_ref"].as_str().unwrap());
    fs::write(candidate, b"partial").unwrap();

    let before = tree_bytes(repo.path());
    let error = store.recover().unwrap_err();
    assert_eq!(
        error.kind(),
        GenericLineageStoreErrorKindV1::ConflictingTransactionState
    );
    assert_eq!(fs::read(canonical).unwrap(), old_bytes);
    assert_eq!(tree_bytes(repo.path()), before);
}

#[test]
fn canonical_scratch_resume_rechecks_the_basis_immediately_before_publication() {
    for mutate_basis in [false, true] {
        let repo = tempfile::tempdir().unwrap();
        let canonical = repo.path().join(".handbook/project/registry-brief.yaml");
        fs::create_dir_all(canonical.parent().unwrap()).unwrap();
        let old_bytes = b"artifact_kind: previous\n";
        fs::write(&canonical, old_bytes).unwrap();
        let expected_basis = sha256(old_bytes);
        let request = promotion_request(
            if mutate_basis {
                "scratch_basis_recheck_mutated"
            } else {
                "scratch_basis_recheck_exact"
            },
            Some(&expected_basis),
        );
        let plan = promotion_plan(Some(&expected_basis));
        let store = GenericArtifactLineageStoreV1::new(repo.path());
        let error = store
            .execute_at_with_fault_for_testing(
                request,
                FIXED_NOW,
                GenericLineageFaultPointV1::AfterCanonicalScratch(0),
                {
                    let plan = plan.clone();
                    move |_| Ok(plan)
                },
            )
            .unwrap_err();
        assert_eq!(error.kind(), GenericLineageStoreErrorKindV1::InjectedFault);
        if mutate_basis {
            fs::write(&canonical, b"artifact_kind: externally-mutated\n").unwrap();
            let error = store.recover().unwrap_err();
            assert_eq!(error.kind(), GenericLineageStoreErrorKindV1::BasisMismatch);
            assert_eq!(
                fs::read(&canonical).unwrap(),
                b"artifact_kind: externally-mutated\n"
            );
        } else {
            store.recover().expect("exact durable scratch resumes");
            let GenericPlannedOutcomeV1::Commit(plan) = plan else {
                unreachable!()
            };
            assert_eq!(fs::read(&canonical).unwrap(), plan.outputs[0].bytes);
        }
    }
}

#[test]
fn canonical_basis_same_bytes_aba_with_restored_mtime_refuses_without_store_mutation() {
    let repo = tempfile::tempdir().unwrap();
    let canonical = repo.path().join(".handbook/project/registry-brief.yaml");
    fs::create_dir_all(canonical.parent().unwrap()).unwrap();
    let old_bytes = b"artifact_kind: previous\n";
    fs::write(&canonical, old_bytes).unwrap();
    let original_modified = fs::metadata(&canonical).unwrap().modified().unwrap();
    let expected_basis = sha256(old_bytes);
    let request = promotion_request("same_bytes_aba_basis_0001", Some(&expected_basis));
    let plan = promotion_plan(Some(&expected_basis));
    let store = GenericArtifactLineageStoreV1::new(repo.path());
    let error = store
        .execute_at_with_fault_for_testing(
            request,
            FIXED_NOW,
            GenericLineageFaultPointV1::AfterCanonicalScratch(0),
            {
                let plan = plan.clone();
                move |_| Ok(plan)
            },
        )
        .unwrap_err();
    assert_eq!(error.kind(), GenericLineageStoreErrorKindV1::InjectedFault);

    fs::write(&canonical, b"artifact_kind: attacker\n").unwrap();
    fs::write(&canonical, old_bytes).unwrap();
    File::options()
        .write(true)
        .open(&canonical)
        .unwrap()
        .set_times(FileTimes::new().set_modified(original_modified))
        .unwrap();
    assert_eq!(
        fs::metadata(&canonical).unwrap().len(),
        old_bytes.len() as u64
    );
    assert_eq!(fs::read(&canonical).unwrap(), old_bytes);
    let before = tree_bytes(repo.path());

    let error = GenericArtifactLineageStoreV1::new(repo.path())
        .recover()
        .unwrap_err();
    assert_eq!(error.kind(), GenericLineageStoreErrorKindV1::BasisMismatch);
    assert_eq!(tree_bytes(repo.path()), before);
}

#[derive(Clone, Copy, Debug)]
enum SameBytesSubstitution {
    Regular,
    Symlink,
    HardLink,
}

fn same_bytes_substitutions() -> Vec<SameBytesSubstitution> {
    #[cfg(unix)]
    {
        return vec![
            SameBytesSubstitution::Regular,
            SameBytesSubstitution::Symlink,
            SameBytesSubstitution::HardLink,
        ];
    }
    #[cfg(windows)]
    {
        return vec![
            SameBytesSubstitution::Regular,
            SameBytesSubstitution::HardLink,
        ];
    }
    #[allow(unreachable_code)]
    Vec::new()
}

fn substitute_with_same_bytes(path: &Path, bytes: &[u8], kind: SameBytesSubstitution) {
    fs::remove_file(path).unwrap();
    match kind {
        SameBytesSubstitution::Regular => fs::write(path, bytes).unwrap(),
        SameBytesSubstitution::Symlink | SameBytesSubstitution::HardLink => {
            let source = path.with_extension(format!(
                "{}.same-bytes-source",
                path.extension()
                    .and_then(|value| value.to_str())
                    .unwrap_or("bin")
            ));
            fs::write(&source, bytes).unwrap();
            if matches!(kind, SameBytesSubstitution::HardLink) {
                fs::hard_link(source, path).unwrap();
            } else {
                #[cfg(unix)]
                std::os::unix::fs::symlink(source, path).unwrap();
                #[cfg(windows)]
                std::os::windows::fs::symlink_file(source, path).unwrap();
            }
        }
    }
}

#[test]
fn canonical_scratch_identity_substitution_after_basis_verification_refuses() {
    for kind in same_bytes_substitutions() {
        let repo = tempfile::tempdir().unwrap();
        let canonical = repo.path().join(".handbook/project/registry-brief.yaml");
        fs::create_dir_all(canonical.parent().unwrap()).unwrap();
        let old_bytes = b"artifact_kind: previous\n";
        fs::write(&canonical, old_bytes).unwrap();
        let expected_basis = sha256(old_bytes);
        let plan = promotion_plan(Some(&expected_basis));
        let GenericPlannedOutcomeV1::Commit(commit) = &plan else {
            unreachable!()
        };
        let intended = commit.outputs[0].bytes.clone();
        let request = promotion_request(
            &format!("scratch_identity_substitution_{kind:?}"),
            Some(&expected_basis),
        );
        let store = GenericArtifactLineageStoreV1::new(repo.path());
        let error = store
            .execute_at_with_fault_for_testing(
                request,
                FIXED_NOW,
                GenericLineageFaultPointV1::AfterCanonicalScratch(0),
                move |_| Ok(plan),
            )
            .unwrap_err();
        assert_eq!(error.kind(), GenericLineageStoreErrorKindV1::InjectedFault);
        let transaction = pending_dir(
            repo.path(),
            GenericArtifactOperationV1::ArtifactCandidatePromote,
        );
        let publication: Value =
            serde_json::from_slice(&fs::read(transaction.join("native-publication.json")).unwrap())
                .unwrap();
        let scratch = repo
            .path()
            .join(publication["candidate_ref"].as_str().unwrap());
        substitute_with_same_bytes(&scratch, &intended, kind);
        let before = tree_bytes(repo.path());

        assert!(GenericArtifactLineageStoreV1::new(repo.path())
            .recover()
            .is_err());
        assert_eq!(fs::read(&canonical).unwrap(), old_bytes);
        assert_eq!(tree_bytes(repo.path()), before);
    }
}

#[test]
fn fresh_promotion_refuses_preexisting_replacement_scratch_without_mutation() {
    for equal in [true, false] {
        let repo = tempfile::tempdir().unwrap();
        let canonical = repo.path().join(".handbook/project/registry-brief.yaml");
        fs::create_dir_all(canonical.parent().unwrap()).unwrap();
        let old_bytes = b"artifact_kind: previous\n";
        fs::write(&canonical, old_bytes).unwrap();
        let expected_basis = sha256(old_bytes);
        let plan = promotion_plan(Some(&expected_basis));
        let GenericPlannedOutcomeV1::Commit(commit) = &plan else {
            unreachable!()
        };
        let scratch = canonical
            .parent()
            .unwrap()
            .join(".registry-brief.yaml.generic-replace");
        let store = GenericArtifactLineageStoreV1::new(repo.path());
        store.recover().expect("prime the generic lock");
        fs::write(
            &scratch,
            if equal {
                commit.outputs[0].bytes.as_slice()
            } else {
                b"unequal residue"
            },
        )
        .unwrap();
        let before = tree_bytes(repo.path());

        let error = store
            .execute_at_for_testing(
                promotion_request(
                    if equal {
                        "fresh_equal_scratch_residue"
                    } else {
                        "fresh_unequal_scratch_residue"
                    },
                    Some(&expected_basis),
                ),
                FIXED_NOW,
                move |_| Ok(plan),
            )
            .expect_err("fresh promotion must never adopt or rewrite scratch residue");
        assert_eq!(
            error.kind(),
            GenericLineageStoreErrorKindV1::ConflictingTransactionState
        );
        assert_eq!(fs::read(&canonical).unwrap(), old_bytes);
        assert_eq!(tree_bytes(repo.path()), before);
    }
}

#[test]
#[cfg(unix)]
fn same_byte_basis_substitution_between_retained_read_and_identity_refuses() {
    let repo = tempfile::tempdir().unwrap();
    let canonical = repo.path().join(".handbook/project/registry-brief.yaml");
    fs::create_dir_all(canonical.parent().unwrap()).unwrap();
    let old_bytes = b"artifact_kind: previous\n";
    fs::write(&canonical, old_bytes).unwrap();
    let expected_basis = sha256(old_bytes);
    let plan = promotion_plan(Some(&expected_basis));
    let before = tree_bytes(repo.path());
    let hook_path = canonical.clone();
    set_after_retained_read_hook_for_testing(canonical.clone(), move |_| {
        substitute_with_same_bytes(&hook_path, old_bytes, SameBytesSubstitution::Regular);
    });

    let error = GenericArtifactLineageStoreV1::new(repo.path())
        .execute_at_for_testing(
            promotion_request("basis_read_identity_substitution", Some(&expected_basis)),
            FIXED_NOW,
            move |_| Ok(plan),
        )
        .expect_err("retained-read/path-identity substitution must refuse");
    assert!(matches!(
        error.kind(),
        GenericLineageStoreErrorKindV1::BasisMismatch
            | GenericLineageStoreErrorKindV1::ConflictingTransactionState
            | GenericLineageStoreErrorKindV1::UnsafeFilesystem
            | GenericLineageStoreErrorKindV1::IoFailure
    ));
    assert_eq!(fs::read(&canonical).unwrap(), old_bytes);
    assert_eq!(tree_bytes(repo.path()), before);
}

#[test]
#[cfg(windows)]
fn retained_basis_handle_denies_same_byte_path_substitution_on_windows() {
    let repo = tempfile::tempdir().unwrap();
    let canonical = repo.path().join(".handbook/project/registry-brief.yaml");
    fs::create_dir_all(canonical.parent().unwrap()).unwrap();
    let old_bytes = b"artifact_kind: previous\n";
    fs::write(&canonical, old_bytes).unwrap();
    let expected_basis = sha256(old_bytes);
    let plan = promotion_plan(Some(&expected_basis));
    let deletion_denied = Arc::new(AtomicBool::new(false));
    let hook_denied = Arc::clone(&deletion_denied);
    let hook_path = canonical.clone();
    set_after_retained_read_hook_for_testing(canonical.clone(), move |_| {
        if fs::remove_file(&hook_path).is_err() {
            hook_denied.store(true, Ordering::SeqCst);
        } else {
            fs::write(&hook_path, old_bytes).expect("unexpected substitution recreation");
        }
    });

    let execution = GenericArtifactLineageStoreV1::new(repo.path())
        .execute_at_for_testing(
            promotion_request("basis_retained_handle_denial", Some(&expected_basis)),
            FIXED_NOW,
            move |_| Ok(plan),
        )
        .expect("denied substitution leaves the promotion eligible");
    assert_eq!(
        execution.disposition,
        GenericExecutionDispositionV1::Committed
    );
    assert!(deletion_denied.load(Ordering::SeqCst));
}

#[test]
fn same_byte_scratch_substitution_immediately_before_publication_refuses_without_canonical_delta() {
    let repo = tempfile::tempdir().unwrap();
    let canonical = repo.path().join(".handbook/project/registry-brief.yaml");
    fs::create_dir_all(canonical.parent().unwrap()).unwrap();
    let old_bytes = b"artifact_kind: previous\n";
    fs::write(&canonical, old_bytes).unwrap();
    let expected_basis = sha256(old_bytes);
    let plan = promotion_plan(Some(&expected_basis));
    let GenericPlannedOutcomeV1::Commit(commit) = &plan else {
        unreachable!()
    };
    let intended = commit.outputs[0].bytes.clone();
    let mutation_request =
        promotion_request("scratch_prepublication_substitution", Some(&expected_basis));
    let probe = tempfile::tempdir().unwrap();
    let probe_canonical = probe.path().join(".handbook/project/registry-brief.yaml");
    fs::create_dir_all(probe_canonical.parent().unwrap()).unwrap();
    fs::write(&probe_canonical, old_bytes).unwrap();
    let probe_store = GenericArtifactLineageStoreV1::new(probe.path());
    let error = probe_store
        .execute_at_with_fault_for_testing(
            mutation_request.clone(),
            FIXED_NOW,
            GenericLineageFaultPointV1::AfterCanonicalScratch(0),
            {
                let plan = plan.clone();
                move |_| Ok(plan)
            },
        )
        .unwrap_err();
    assert_eq!(error.kind(), GenericLineageStoreErrorKindV1::InjectedFault);
    let probe_transaction = pending_dir(
        probe.path(),
        GenericArtifactOperationV1::ArtifactCandidatePromote,
    );
    let publication: Value = serde_json::from_slice(
        &fs::read(probe_transaction.join("native-publication.json")).unwrap(),
    )
    .unwrap();
    let scratch = repo
        .path()
        .join(publication["candidate_ref"].as_str().unwrap());
    drop(probe);
    let hook_path = scratch.clone();
    set_before_publication_rename_hook_for_testing(scratch, move |_| {
        substitute_with_same_bytes(&hook_path, &intended, SameBytesSubstitution::Regular);
    });

    let error = GenericArtifactLineageStoreV1::new(repo.path())
        .execute_at_for_testing(mutation_request, FIXED_NOW, move |_| Ok(plan))
        .expect_err("prepublication scratch substitution must refuse");
    assert!(matches!(
        error.kind(),
        GenericLineageStoreErrorKindV1::ConflictingTransactionState
            | GenericLineageStoreErrorKindV1::UnsafeFilesystem
            | GenericLineageStoreErrorKindV1::IoFailure
    ));
    assert_eq!(fs::read(&canonical).unwrap(), old_bytes);
}

#[test]
fn canonical_identity_substitution_after_installed_verification_refuses() {
    for kind in same_bytes_substitutions() {
        let repo = tempfile::tempdir().unwrap();
        let canonical = repo.path().join(".handbook/project/registry-brief.yaml");
        fs::create_dir_all(canonical.parent().unwrap()).unwrap();
        let old_bytes = b"artifact_kind: previous\n";
        fs::write(&canonical, old_bytes).unwrap();
        let expected_basis = sha256(old_bytes);
        let plan = promotion_plan(Some(&expected_basis));
        let GenericPlannedOutcomeV1::Commit(commit) = &plan else {
            unreachable!()
        };
        let intended = commit.outputs[0].bytes.clone();
        let request = promotion_request(
            &format!("installed_identity_substitution_{kind:?}"),
            Some(&expected_basis),
        );
        let store = GenericArtifactLineageStoreV1::new(repo.path());
        let error = store
            .execute_at_with_fault_for_testing(
                request,
                FIXED_NOW,
                GenericLineageFaultPointV1::AfterInstalledVerification,
                move |_| Ok(plan),
            )
            .unwrap_err();
        assert_eq!(error.kind(), GenericLineageStoreErrorKindV1::InjectedFault);
        substitute_with_same_bytes(&canonical, &intended, kind);
        let before = tree_bytes(repo.path());

        assert!(
            GenericArtifactLineageStoreV1::new(repo.path())
                .recover()
                .is_err(),
            "substitution kind: {kind:?}"
        );
        assert_eq!(tree_bytes(repo.path()), before);
    }
}

#[derive(Clone, Copy, Debug)]
enum FinalBasisMutation {
    Changed,
    Deleted,
    SameBytesAba,
}

fn final_basis_mutations() -> [FinalBasisMutation; 3] {
    [
        FinalBasisMutation::Changed,
        FinalBasisMutation::Deleted,
        FinalBasisMutation::SameBytesAba,
    ]
}

fn mutate_final_basis(
    path: &Path,
    old_bytes: &[u8],
    old_modified: std::time::SystemTime,
    mutation: FinalBasisMutation,
) {
    match mutation {
        FinalBasisMutation::Changed => {
            fs::write(path, b"artifact_kind: external-final-writer\n").unwrap();
        }
        FinalBasisMutation::Deleted => fs::remove_file(path).unwrap(),
        FinalBasisMutation::SameBytesAba => {
            substitute_with_same_bytes(path, old_bytes, SameBytesSubstitution::Regular);
            File::options()
                .write(true)
                .open(path)
                .unwrap()
                .set_times(FileTimes::new().set_modified(old_modified))
                .unwrap();
        }
    }
}

#[test]
fn final_publication_hook_rechecks_canonical_basis_for_fresh_and_recovery_paths() {
    for recovery in [false, true] {
        for mutation in final_basis_mutations() {
            let repo = tempfile::tempdir().unwrap();
            let canonical = repo.path().join(".handbook/project/registry-brief.yaml");
            fs::create_dir_all(canonical.parent().unwrap()).unwrap();
            let old_bytes = b"artifact_kind: previous\n";
            fs::write(&canonical, old_bytes).unwrap();
            let old_modified = fs::metadata(&canonical).unwrap().modified().unwrap();
            let expected_basis = sha256(old_bytes);
            let plan = promotion_plan(Some(&expected_basis));
            let store = GenericArtifactLineageStoreV1::new(repo.path());
            let mutation_request = promotion_request(
                &format!("final_basis_{recovery}_{mutation:?}"),
                Some(&expected_basis),
            );
            if recovery {
                let error = store
                    .execute_at_with_fault_for_testing(
                        mutation_request.clone(),
                        FIXED_NOW,
                        GenericLineageFaultPointV1::AfterCanonicalScratch(0),
                        {
                            let plan = plan.clone();
                            move |_| Ok(plan)
                        },
                    )
                    .unwrap_err();
                assert_eq!(error.kind(), GenericLineageStoreErrorKindV1::InjectedFault);
            }
            let scratch = if recovery {
                let transaction = pending_dir(
                    repo.path(),
                    GenericArtifactOperationV1::ArtifactCandidatePromote,
                );
                let publication: Value = serde_json::from_slice(
                    &fs::read(transaction.join("native-publication.json")).unwrap(),
                )
                .unwrap();
                repo.path()
                    .join(publication["candidate_ref"].as_str().unwrap())
            } else {
                let probe = tempfile::tempdir().unwrap();
                let probe_canonical = probe.path().join(".handbook/project/registry-brief.yaml");
                fs::create_dir_all(probe_canonical.parent().unwrap()).unwrap();
                fs::write(&probe_canonical, old_bytes).unwrap();
                let probe_store = GenericArtifactLineageStoreV1::new(probe.path());
                let error = probe_store
                    .execute_at_with_fault_for_testing(
                        mutation_request.clone(),
                        FIXED_NOW,
                        GenericLineageFaultPointV1::AfterCanonicalScratch(0),
                        {
                            let plan = plan.clone();
                            move |_| Ok(plan)
                        },
                    )
                    .unwrap_err();
                assert_eq!(error.kind(), GenericLineageStoreErrorKindV1::InjectedFault);
                let probe_transaction = pending_dir(
                    probe.path(),
                    GenericArtifactOperationV1::ArtifactCandidatePromote,
                );
                let publication: Value = serde_json::from_slice(
                    &fs::read(probe_transaction.join("native-publication.json")).unwrap(),
                )
                .unwrap();
                let scratch = repo
                    .path()
                    .join(publication["candidate_ref"].as_str().unwrap());
                drop(probe);
                scratch
            };
            let hook_canonical = canonical.clone();
            set_before_publication_rename_hook_for_testing(scratch, move |_| {
                mutate_final_basis(&hook_canonical, old_bytes, old_modified, mutation);
            });

            let error = if recovery {
                store.recover().unwrap_err()
            } else {
                store
                    .execute_at_for_testing(mutation_request, FIXED_NOW, move |_| Ok(plan))
                    .unwrap_err()
            };
            assert_eq!(
                error.kind(),
                GenericLineageStoreErrorKindV1::BasisMismatch,
                "recovery={recovery}, mutation={mutation:?}"
            );
            match mutation {
                FinalBasisMutation::Changed => assert_eq!(
                    fs::read(&canonical).unwrap(),
                    b"artifact_kind: external-final-writer\n"
                ),
                FinalBasisMutation::Deleted => assert!(!canonical.exists()),
                FinalBasisMutation::SameBytesAba => {
                    assert_eq!(fs::read(&canonical).unwrap(), old_bytes)
                }
            }
            assert_no_promotion_authority(repo.path());
        }
    }
}

#[test]
fn fresh_create_new_refuses_equal_and_unequal_closure_or_semantic_residue_before_intent() {
    for ordinal in [0usize, 2usize] {
        for equal in [true, false] {
            let repo = tempfile::tempdir().unwrap();
            let store = GenericArtifactLineageStoreV1::new(repo.path());
            store.recover().unwrap();
            let plan = intake_plan(FIXED_NOW);
            let GenericPlannedOutcomeV1::Commit(commit) = &plan else {
                unreachable!()
            };
            let target = repo.path().join(&commit.outputs[ordinal].final_ref);
            let residue = if equal {
                commit.outputs[ordinal].bytes.clone()
            } else {
                b"unequal immutable residue".to_vec()
            };
            let planted = residue.clone();
            let planted_target = target.clone();
            let error = store
                .execute_at_for_testing(
                    request(
                        GenericArtifactOperationV1::IntakeRecordAppend,
                        &format!("fresh_output_{ordinal}_{equal}_0001"),
                    ),
                    FIXED_NOW,
                    move |_| {
                        fs::create_dir_all(planted_target.parent().unwrap()).unwrap();
                        fs::write(&planted_target, &planted).unwrap();
                        Ok(plan)
                    },
                )
                .unwrap_err();
            assert_eq!(
                error.kind(),
                GenericLineageStoreErrorKindV1::ConflictingTransactionState
            );
            assert_eq!(fs::read(&target).unwrap(), residue);
            assert!(!repo.path().join(".handbook/state/transactions").exists());
            assert!(!repo
                .path()
                .join(".handbook/state/idempotency/generic-artifact-operations/ledger")
                .exists());
        }
    }
}

#[test]
fn reverse_reachability_refuses_manufactured_replay_expiry_and_orphan_controls() {
    for case in [
        "retained_without_journal",
        "tombstone_without_journal",
        "result_only",
        "ledger_only",
        "malformed_ledger",
        "ambiguous_ledger",
    ] {
        let repo = tempfile::tempdir().unwrap();
        let store = GenericArtifactLineageStoreV1::new(repo.path());
        let mutation_request = request(
            GenericArtifactOperationV1::IntakeRecordAppend,
            &format!("orphan_control_{case}_0001"),
        );
        execute(&store, mutation_request.clone(), intake_plan(FIXED_NOW));
        let idempotency = repo
            .path()
            .join(".handbook/state/idempotency/generic-artifact-operations");
        let ledger_root = idempotency.join("ledger");
        let result_root = idempotency.join("results");
        let ledger = fs::read_dir(&ledger_root)
            .unwrap()
            .next()
            .unwrap()
            .unwrap()
            .path();
        if case == "tombstone_without_journal" {
            store
                .expire_retained_result_at_for_testing(&mutation_request, "2026-08-21T00:00:00Z")
                .unwrap();
        }
        if case == "malformed_ledger" {
            rewrite_control(&ledger, "entry_fingerprint", |value| {
                value["unexpected"] = json!(true);
            });
        }
        if case == "ambiguous_ledger" {
            let mut value: Value = serde_json::from_slice(&fs::read(&ledger).unwrap()).unwrap();
            let other_key = format!("sha256:{}", "a".repeat(64));
            value["domain_mutation_key_fingerprint"] = json!(other_key);
            let value = refingerprint_control(value, "entry_fingerprint");
            fs::write(
                ledger_root.join(format!("{}.json", "a".repeat(64))),
                jcs_lf(&value),
            )
            .unwrap();
        }
        if !matches!(case, "malformed_ledger" | "ambiguous_ledger") {
            remove_tree_if_present(&repo.path().join(".handbook/state/transactions"));
            remove_tree_if_present(&repo.path().join(".handbook/state/artifacts"));
            remove_tree_if_present(&repo.path().join(".handbook/evidence/artifacts"));
        }
        match case {
            "result_only" => remove_tree_if_present(&ledger_root),
            "ledger_only" => remove_tree_if_present(&result_root),
            "ambiguous_ledger" => {}
            _ => {}
        }
        let before = tree_bytes(repo.path());
        let error = store
            .execute_at_for_testing(mutation_request, FIXED_NOW, |_| {
                panic!("orphan controls must not manufacture replay or expiry")
            })
            .unwrap_err();
        assert_eq!(
            error.kind(),
            GenericLineageStoreErrorKindV1::RetainedResultMismatch,
            "case={case}"
        );
        assert_eq!(tree_bytes(repo.path()), before, "case={case}");
    }
}

#[test]
fn validly_named_orphan_artifact_and_self_fingerprinted_malformed_ledger_refuse_without_delta() {
    let repo = tempfile::tempdir().unwrap();
    let store = GenericArtifactLineageStoreV1::new(repo.path());
    store.recover().unwrap();
    let orphan = repo.path().join(format!(
        ".handbook/evidence/artifacts/registry_brief/content/content_{}.json",
        "b".repeat(64)
    ));
    fs::create_dir_all(orphan.parent().unwrap()).unwrap();
    fs::write(&orphan, b"{}\n").unwrap();
    let before = tree_bytes(repo.path());
    assert_eq!(
        store.recover().unwrap_err().kind(),
        GenericLineageStoreErrorKindV1::RetainedResultMismatch
    );
    assert_eq!(tree_bytes(repo.path()), before);

    fs::remove_file(&orphan).unwrap();
    let ledger_root = repo
        .path()
        .join(".handbook/state/idempotency/generic-artifact-operations/ledger");
    fs::create_dir_all(&ledger_root).unwrap();
    let malformed = refingerprint_control(
        json!({
            "schema_id": "handbook.generic-domain-ledger-entry",
            "schema_version": "1.0",
            "unexpected": true
        }),
        "entry_fingerprint",
    );
    fs::write(
        ledger_root.join(format!("{}.json", "c".repeat(64))),
        jcs_lf(&malformed),
    )
    .unwrap();
    let before = tree_bytes(repo.path());
    assert_eq!(
        store.recover().unwrap_err().kind(),
        GenericLineageStoreErrorKindV1::RetainedResultMismatch
    );
    assert_eq!(tree_bytes(repo.path()), before);
}

#[test]
fn publication_refusal_enums_have_only_the_contract_wire_values() {
    assert_eq!(
        serde_json::to_value(EstablishedRefusalCodeV1::PublicationBasisConflict).unwrap(),
        json!("publication_basis_conflict")
    );
    assert_eq!(
        serde_json::to_value(GenericRefusalLayerV1::Publication).unwrap(),
        json!("publication")
    );
    assert_eq!(
        serde_json::from_value::<EstablishedRefusalCodeV1>(json!("publication_basis_conflict"))
            .unwrap(),
        EstablishedRefusalCodeV1::PublicationBasisConflict
    );
    assert_eq!(
        serde_json::from_value::<GenericRefusalLayerV1>(json!("publication")).unwrap(),
        GenericRefusalLayerV1::Publication
    );
    assert!(
        serde_json::from_value::<EstablishedRefusalCodeV1>(json!("publication_conflict")).is_err()
    );
    assert!(serde_json::from_value::<GenericRefusalLayerV1>(json!("native_publication")).is_err());
}

#[test]
fn promotion_intent_persists_only_the_five_exact_basis_fields() {
    for present in [false, true] {
        let repo = tempfile::tempdir().unwrap();
        let canonical = repo.path().join(".handbook/project/registry-brief.yaml");
        let old_bytes = b"title: old\n";
        let expected_basis = present.then(|| {
            fs::create_dir_all(canonical.parent().unwrap()).unwrap();
            fs::write(&canonical, old_bytes).unwrap();
            sha256(old_bytes)
        });
        let store = GenericArtifactLineageStoreV1::new(repo.path());
        let error = store
            .execute_at_with_fault_for_testing(
                promotion_request(
                    if present {
                        "five_field_present_basis"
                    } else {
                        "five_field_absent_basis_"
                    },
                    expected_basis.as_deref(),
                ),
                FIXED_NOW,
                GenericLineageFaultPointV1::AfterPendingIntent,
                |_| Ok(promotion_plan(expected_basis.as_deref())),
            )
            .expect_err("fault must retain the exact promotion intent");
        assert_eq!(error.kind(), GenericLineageStoreErrorKindV1::InjectedFault);

        let intent: Value = serde_json::from_slice(
            &fs::read(
                pending_dir(
                    repo.path(),
                    GenericArtifactOperationV1::ArtifactCandidatePromote,
                )
                .join("intent.json"),
            )
            .unwrap(),
        )
        .unwrap();
        assert_eq!(
            intent["expected_basis_presence"],
            json!(if present { "present" } else { "absent" })
        );
        assert_eq!(
            intent["expected_basis_fingerprint"],
            expected_basis
                .as_ref()
                .map_or(Value::Null, |value| json!(value))
        );
        if present {
            assert_eq!(intent["expected_basis_byte_length"], json!(old_bytes.len()));
            assert!(intent["expected_basis_identity_token"]
                .as_str()
                .is_some_and(|value| value.starts_with("native-id-v1:")));
            assert!(intent["expected_basis_version_token"]
                .as_str()
                .is_some_and(|value| value.starts_with("native-version-v1:")));
        } else {
            assert!(intent["expected_basis_byte_length"].is_null());
            assert!(intent["expected_basis_identity_token"].is_null());
            assert!(intent["expected_basis_version_token"].is_null());
        }
        assert!(intent.get("expected_basis_platform_observation").is_none());

        let schema = control_schema();
        let validator = jsonschema::draft202012::options()
            .with_pattern_options(jsonschema::PatternOptions::fancy_regex())
            .build(&schema)
            .unwrap();
        assert!(
            validator.is_valid(&intent),
            "persisted promotion intent must satisfy the frozen schema"
        );
    }
}

#[test]
fn publication_conflict_cannot_be_an_established_refusal() {
    let repo = tempfile::tempdir().unwrap();
    let store = GenericArtifactLineageStoreV1::new(repo.path());
    let error = store
        .execute_at_for_testing(
            request(
                GenericArtifactOperationV1::ArtifactCandidatePromote,
                "preplanned_publication_conflict",
            ),
            FIXED_NOW,
            |_| {
                Ok(GenericPlannedOutcomeV1::Refuse(GenericRefusalPlanV1 {
                    operation_context_fingerprint: CONTEXT.to_owned(),
                    expected_basis_fingerprint: None,
                    refusal: EstablishedRefusalV1 {
                        code: EstablishedRefusalCodeV1::PublicationBasisConflict,
                        layer: GenericRefusalLayerV1::Publication,
                        expected_fingerprint: Some(
                            "sha256:aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"
                                .to_owned(),
                        ),
                        observed_fingerprint: None,
                    },
                }))
            },
        )
        .expect_err("publication conflict is terminal-only");
    assert_eq!(error.kind(), GenericLineageStoreErrorKindV1::InvalidPlan);
    assert!(!repo.path().join(".handbook/state/transactions").exists());
    assert!(!repo
        .path()
        .join(".handbook/state/idempotency/generic-artifact-operations")
        .exists());
}

#[test]
fn promotion_persists_the_schema_exact_native_result_chain() {
    let repo = tempfile::tempdir().unwrap();
    let store = GenericArtifactLineageStoreV1::new(repo.path());
    let execution = store
        .execute_at_for_testing(
            request(
                GenericArtifactOperationV1::ArtifactCandidatePromote,
                "native_result_chain_0001",
            ),
            FIXED_NOW,
            |_| Ok(promotion_plan(None)),
        )
        .expect("promotion must complete through its native result");
    assert_eq!(
        execution.disposition,
        GenericExecutionDispositionV1::Committed
    );

    let committed = fs::read_dir(
        repo.path()
            .join(".handbook/state/transactions/artifact-promotions"),
    )
    .unwrap()
    .map(|entry| entry.unwrap().path())
    .find(|path| path.extension().is_some_and(|value| value == "committed"))
    .unwrap();
    let observation: Value =
        serde_json::from_slice(&fs::read(committed.join("native-publication.json")).unwrap())
            .unwrap();
    let native_result: Value = serde_json::from_slice(
        &fs::read(committed.join("native-publication-result.json")).unwrap(),
    )
    .unwrap();
    let marker: Value =
        serde_json::from_slice(&fs::read(committed.join("commit-marker.json")).unwrap()).unwrap();
    let evidence: Value =
        serde_json::from_slice(&fs::read(committed.join("evidence.json")).unwrap()).unwrap();
    let schema = control_schema();
    let validator = jsonschema::draft202012::options()
        .with_pattern_options(jsonschema::PatternOptions::fancy_regex())
        .build(&schema)
        .unwrap();
    for (name, value) in [
        ("native-publication.json", &observation),
        ("native-publication-result.json", &native_result),
        ("commit-marker.json", &marker),
        ("evidence.json", &evidence),
    ] {
        let errors = validator
            .iter_errors(value)
            .map(|error| error.to_string())
            .collect::<Vec<_>>();
        assert!(errors.is_empty(), "{name}: {errors:?}");
    }
    assert_eq!(native_result["publication_disposition"], "authorized");
    assert_eq!(
        marker["publication_result_fingerprint"],
        native_result["result_fingerprint"]
    );
    assert_eq!(
        evidence["publication_result_fingerprint"],
        native_result["result_fingerprint"]
    );
}

#[test]
fn final_boundary_conflict_commits_only_an_outputless_refused_suffix() {
    let repo = tempfile::tempdir().unwrap();
    let store = GenericArtifactLineageStoreV1::new(repo.path());
    let request = request(
        GenericArtifactOperationV1::ArtifactCandidatePromote,
        "terminal_publication_conflict",
    );
    let error = store
        .execute_at_with_fault_for_testing(
            request.clone(),
            FIXED_NOW,
            GenericLineageFaultPointV1::AfterVerified,
            |_| Ok(promotion_plan(None)),
        )
        .expect_err("fault must retain the pre-publication prefix");
    assert_eq!(error.kind(), GenericLineageStoreErrorKindV1::InjectedFault);
    let pending = pending_dir(
        repo.path(),
        GenericArtifactOperationV1::ArtifactCandidatePromote,
    );
    let intent: Value =
        serde_json::from_slice(&fs::read(pending.join("intent.json")).unwrap()).unwrap();
    let transaction_id = intent["transaction_id"].as_str().unwrap();
    let candidate = repo.path().join(format!(
        ".handbook/project/.registry-brief.yaml.generic-publish-{transaction_id}.candidate"
    ));
    let canonical = repo.path().join(".handbook/project/registry-brief.yaml");
    let hook_canonical = canonical.clone();
    set_before_publication_rename_hook_for_testing(candidate.clone(), move |_| {
        fs::write(&hook_canonical, b"competing publication\n").unwrap();
    });
    store
        .recover()
        .expect("stable pre-call conflict must terminalize");

    let committed = pending.with_extension("committed");
    let marker: Value =
        serde_json::from_slice(&fs::read(committed.join("commit-marker.json")).unwrap()).unwrap();
    let native_result: Value = serde_json::from_slice(
        &fs::read(committed.join("native-publication-result.json")).unwrap(),
    )
    .unwrap();
    let evidence: Value =
        serde_json::from_slice(&fs::read(committed.join("evidence.json")).unwrap()).unwrap();
    assert_eq!(marker["outcome"], "refused");
    assert_eq!(marker["refusal"]["code"], "publication_basis_conflict");
    assert_eq!(marker["refusal"]["layer"], "publication");
    assert_eq!(marker["authoritative_outputs"], json!([]));
    assert_eq!(marker["subordinate_outputs"], json!([]));
    assert_eq!(
        marker["publication_result_fingerprint"],
        native_result["result_fingerprint"]
    );
    assert_eq!(
        evidence["publication_result_fingerprint"],
        native_result["result_fingerprint"]
    );
    assert_eq!(fs::read(&canonical).unwrap(), b"competing publication\n");
    assert!(candidate.exists(), "replacement evidence must be retained");
    assert!(!repo
        .path()
        .join(
            ".handbook/state/artifacts/registry_brief/promotion-records/\
             promotion_d3dfb97fa1e24ac8a16651fa733cf60d83bdae4ecdcaddce1c73c263146ff8ae.json"
                .replace(' ', "")
        )
        .exists());
    store
        .recover()
        .expect("outputless conflict must satisfy committed reachability");
    let replay = store
        .execute_at_for_testing(request, FIXED_NOW, |_| Ok(promotion_plan(None)))
        .expect("terminal conflict must replay");
    assert_eq!(replay.disposition, GenericExecutionDispositionV1::Replayed);
    assert_eq!(replay.result.outcome, "refused");
    assert!(replay.result.authoritative_outputs.is_empty());
}
