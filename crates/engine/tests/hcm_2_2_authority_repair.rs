use handbook_engine::{
    parse_definition_yaml, resolve_shipped_profile_decisions, CharterAcquisitionMode,
    CharterApprovalRefusalCodeV1, CharterApprovalRequestV1, CharterApprovalResultV1,
    CharterApprovalServiceV1, CharterAuthorPersistenceErrorKindV1,
    CharterAuthorPersistenceResultV1, CharterAuthorPersistenceServiceV1, CharterCoverageSubmission,
    CharterIntakeConsumer, CharterIntakeEnvelope, CharterIntakeSourceKind,
    CharterPromotionIntentV1, CharterPromotionWorkflowErrorKindV1,
    CharterPromotionWorkflowServiceV1, DefinitionFingerprint, NativeAuthenticatorPortErrorV1,
    NativeAuthenticatorPortV1,
};
use serde_json::Value;
use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

const CANONICAL_CHARTER: &[u8] = include_bytes!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../docs/specs/handbook-contract-membrane/slices/HCM-2.2/contracts/canonical-charter-boundary-v1.1.yaml"
));
const AUTHORITY_REPAIR_VECTORS: &[u8] = include_bytes!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../docs/specs/handbook-contract-membrane/slices/HCM-2.2/contracts/authority-repair-runtime-vectors-v1.0.json"
));
const LIFECYCLE_VALIDATION_SCHEMA: &[u8] = include_bytes!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../docs/specs/handbook-contract-membrane/slices/HCM-2.2/contracts/lifecycle-validation-result-1.0.0.schema.json"
));
const EMBEDDED_LIFECYCLE_VALIDATION_SCHEMA: &[u8] = include_bytes!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/src/contracts/lifecycle-validation-result-1.0.0.schema.json"
));

#[test]
fn authority_repair_vectors_freeze_complete_provenance_and_exact_result_documents() {
    let vectors: Value = serde_json::from_slice(AUTHORITY_REPAIR_VECTORS).unwrap();
    for chain in vectors["chains"].as_array().unwrap() {
        let subject = &chain["candidate_subject_preimage"];
        assert_eq!(subject["field_sources"].as_array().unwrap().len(), 113);
        assert_eq!(
            DefinitionFingerprint::from_json_value(subject)
                .unwrap()
                .to_string(),
            chain["expected_candidate_subject_fingerprint"]
        );
        let result_fingerprint = DefinitionFingerprint::from_json_value(
            &chain["validation_result_fingerprint_preimage"],
        )
        .unwrap()
        .to_string();
        assert_eq!(
            Value::String(result_fingerprint),
            chain["expected_validation_result_fingerprint"]
        );
        let mut result = chain["validation_result_fingerprint_preimage"].clone();
        result.as_object_mut().unwrap().extend(
            chain["validation_result_record_additions"]
                .as_object()
                .unwrap()
                .clone(),
        );
        let exact_result = result_bytes(&result);
        assert_eq!(
            exact_result.len(),
            chain["expected_validation_result_document_byte_length"]
                .as_u64()
                .unwrap() as usize
        );
        assert_eq!(
            DefinitionFingerprint::from_bytes(&exact_result).to_string(),
            chain["expected_validation_result_document_sha256"]
        );
        assert_eq!(
            DefinitionFingerprint::from_json_value(&chain["final_candidate_fingerprint_preimage"])
                .unwrap()
                .to_string(),
            chain["expected_final_candidate_fingerprint"]
        );
        assert_eq!(
            chain["final_candidate_record_additions"]["candidate_fingerprint"],
            chain["expected_final_candidate_fingerprint"]
        );
        assert_eq!(
            chain["final_candidate_record_additions"]["candidate_id"],
            chain["expected_final_candidate_id"]
        );
    }
}

#[test]
fn author_finalization_builds_and_replays_the_exact_acyclic_candidate_1_3_chain() {
    let repo = tempfile::tempdir().unwrap();
    let (service, decisions, first) = author(repo.path());
    let candidate_path = candidate_path(repo.path(), &first.candidate_ref);
    let candidate_bytes = std::fs::read(&candidate_path).unwrap();
    let candidate: Value = serde_json::from_slice(&candidate_bytes).unwrap();

    assert_eq!(candidate["schema_id"], "handbook.artifact-candidate");
    assert_eq!(candidate["schema_version"], "1.3");
    assert_eq!(
        candidate["candidate_fingerprint"],
        first.candidate_fingerprint
    );
    assert_exact_keys(
        &candidate,
        &[
            "schema_id",
            "schema_version",
            "candidate_id",
            "intake_record_ref",
            "target_kind_ref",
            "target_instance_id",
            "target_schema_ref",
            "profile_ref",
            "resolved_profile_fingerprint",
            "normalized_content_ref",
            "field_sources",
            "unresolved_coverage_ids",
            "promotion_eligibility",
            "required_approval_policy_ref",
            "basis_artifact_fingerprint",
            "candidate_subject_fingerprint",
            "validation_result_binding",
            "candidate_fingerprint",
        ],
    );

    let subject_fingerprint = recompute_subject(&candidate);
    assert_eq!(
        candidate["candidate_subject_fingerprint"],
        subject_fingerprint
    );
    let binding = &candidate["validation_result_binding"];
    assert_exact_keys(
        binding,
        &[
            "validation_result_ref",
            "validation_result_fingerprint",
            "result_document_sha256",
            "result_byte_length",
        ],
    );
    let validation_ref = binding["validation_result_ref"].as_str().unwrap();
    assert_eq!(validation_ref, first.lifecycle_validation_result_ref);
    let validation_path = repo.path().join(".handbook/state").join(validation_ref);
    let validation_bytes = std::fs::read(&validation_path).unwrap();
    assert_eq!(
        binding["result_document_sha256"],
        DefinitionFingerprint::from_bytes(&validation_bytes).to_string()
    );
    assert_eq!(
        binding["result_byte_length"],
        Value::from(validation_bytes.len())
    );

    let validation: Value = serde_json::from_slice(&validation_bytes).unwrap();
    let validation_schema: Value = serde_json::from_slice(LIFECYCLE_VALIDATION_SCHEMA).unwrap();
    let embedded_validation_schema: Value =
        serde_json::from_slice(EMBEDDED_LIFECYCLE_VALIDATION_SCHEMA).unwrap();
    assert_eq!(embedded_validation_schema, validation_schema);
    jsonschema::draft202012::options()
        .build(&validation_schema)
        .unwrap()
        .validate(&validation)
        .unwrap();
    assert_eq!(validation["schema_version"], "1.0");
    assert_eq!(
        validation["candidate_subject_fingerprint"],
        subject_fingerprint
    );
    assert_eq!(
        validation["validation_result_fingerprint"],
        binding["validation_result_fingerprint"]
    );
    assert!(validation.get("candidate_ref").is_none());
    assert!(validation.get("candidate_fingerprint").is_none());

    let vectors: Value = serde_json::from_slice(AUTHORITY_REPAIR_VECTORS).unwrap();
    assert_eq!(
        validation["resolved_definitions"],
        vectors["resolved_definitions"]
    );
    let mut final_preimage = candidate.clone();
    final_preimage
        .as_object_mut()
        .unwrap()
        .remove("candidate_id");
    final_preimage
        .as_object_mut()
        .unwrap()
        .remove("candidate_fingerprint");
    assert_eq!(
        DefinitionFingerprint::from_json_value(&final_preimage)
            .unwrap()
            .to_string(),
        first.candidate_fingerprint
    );
    assert!(!repo
        .path()
        .join(".handbook/state/lifecycle-validation-result-witnesses")
        .exists());
    assert!(!repo
        .path()
        .join(".handbook/state/lifecycle-validation-result-bindings")
        .exists());

    let second = service
        .persist(&decisions, author_envelope(), None)
        .unwrap();
    assert_eq!(second, first);
    assert_eq!(std::fs::read(candidate_path).unwrap(), candidate_bytes);
    assert_eq!(std::fs::read(validation_path).unwrap(), validation_bytes);
}

#[test]
fn timestamp_only_result_rewrite_refuses_replay_without_mutation() {
    let repo = tempfile::tempdir().unwrap();
    let (service, decisions, authored) = author(repo.path());
    let candidate = read_candidate(repo.path(), &authored.candidate_ref);
    let result_path = result_path(repo.path(), &candidate);
    let mut result: Value = serde_json::from_slice(&std::fs::read(&result_path).unwrap()).unwrap();
    result["validated_at_utc"] = Value::String("2030-01-02T03:04:05Z".to_owned());
    let forged = result_bytes(&result);
    std::fs::write(&result_path, &forged).unwrap();

    let refusal = service
        .persist(&decisions, author_envelope(), None)
        .expect_err("timestamp-only result rewrite must refuse");
    assert_eq!(
        refusal.kind(),
        CharterAuthorPersistenceErrorKindV1::PersistenceRefused
    );
    assert_eq!(std::fs::read(result_path).unwrap(), forged);
}

#[test]
fn schema_invalid_unrelated_result_timestamp_refuses_author_replay() {
    schema_invalid_unrelated_result_refuses(|result| {
        result["validated_at_utc"] = Value::String("abcd-ef-ghTij:kl:mnZ".to_owned());
    });
}

#[test]
fn schema_invalid_unrelated_result_reference_refuses_author_replay() {
    schema_invalid_unrelated_result_refuses(|result| {
        result["intake_record_ref"] = Value::String("intake-records/not-owned.json".to_owned());
    });
}

#[test]
fn schema_invalid_unrelated_result_observation_bound_refuses_author_replay() {
    schema_invalid_unrelated_result_refuses(|result| {
        install_nonabsent_lifecycle_authority(result, false);
        result["active_observations"] = Value::Array(
            (0..257)
                .map(|index| {
                    serde_json::json!({
                        "observation_ref": format!(
                            "lifecycle-observations/lifecycle-observation_{index:064x}.json"
                        ),
                        "observation_fingerprint": format!("sha256:{index:064x}"),
                    })
                })
                .collect(),
        );
    });
}

#[test]
fn schema_invalid_unrelated_result_coverage_bound_refuses_author_replay() {
    schema_invalid_unrelated_result_refuses(|result| {
        install_nonabsent_lifecycle_authority(result, false);
        result["reopened_coverage_ids"] = Value::Array(
            [
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
                "unknown.coverage",
            ]
            .into_iter()
            .map(|coverage_id| Value::String(coverage_id.to_owned()))
            .collect(),
        );
    });
}

#[test]
fn coherent_result_witness_and_binding_rewrite_refuses_without_mutation() {
    let repo = tempfile::tempdir().unwrap();
    let (service, decisions, authored) = author(repo.path());
    let candidate = read_candidate(repo.path(), &authored.candidate_ref);
    let result_path = result_path(repo.path(), &candidate);
    let mut result: Value = serde_json::from_slice(&std::fs::read(&result_path).unwrap()).unwrap();
    result["validated_at_utc"] = Value::String("2033-04-05T06:07:08Z".to_owned());
    let forged_result = result_bytes(&result);
    let basename = result_path.file_name().unwrap();
    let witness_path = repo
        .path()
        .join(".handbook/state/lifecycle-validation-result-witnesses")
        .join(basename);
    let binding_path = repo
        .path()
        .join(".handbook/state/lifecycle-validation-result-bindings")
        .join(basename);
    std::fs::create_dir_all(witness_path.parent().unwrap()).unwrap();
    std::fs::create_dir_all(binding_path.parent().unwrap()).unwrap();
    std::fs::write(&result_path, &forged_result).unwrap();
    std::fs::write(&witness_path, &forged_result).unwrap();
    let forged_companion = serde_json::to_vec(&serde_json::json!({
        "result_document_sha256": DefinitionFingerprint::from_bytes(&forged_result).to_string(),
        "result_byte_length": forged_result.len()
    }))
    .unwrap();
    std::fs::write(&binding_path, &forged_companion).unwrap();

    let replay = service.persist(&decisions, author_envelope(), None);
    let promotion =
        CharterPromotionWorkflowServiceV1::new(repo.path()).promote(CharterPromotionIntentV1 {
            candidate_ref: authored.candidate_ref,
            approval_ref: format!("approvals/approval_{}.json", "a".repeat(64)),
            expected_current_fingerprint: None,
        });
    assert_eq!(
        replay
            .expect_err("coherent companion rewrite must refuse")
            .kind(),
        CharterAuthorPersistenceErrorKindV1::PersistenceRefused
    );
    assert_eq!(
        promotion
            .expect_err("promotion must refuse coherent companion rewrite")
            .kind(),
        CharterPromotionWorkflowErrorKindV1::CandidateRefused
    );
    assert_eq!(std::fs::read(result_path).unwrap(), forged_result);
    assert_eq!(std::fs::read(witness_path).unwrap(), forged_result);
    assert_eq!(std::fs::read(binding_path).unwrap(), forged_companion);
}

#[test]
fn wrong_exact_document_digest_refuses_and_preserves_both_records() {
    candidate_binding_mutation_refuses(|binding| {
        binding["result_document_sha256"] = Value::String(format!("sha256:{}", "0".repeat(64)));
    });
}

#[test]
fn wrong_lf_inclusive_byte_length_refuses_and_preserves_both_records() {
    candidate_binding_mutation_refuses(|binding| {
        let current = binding["result_byte_length"].as_u64().unwrap();
        binding["result_byte_length"] = Value::from(current + 1);
    });
}

#[test]
fn semantic_result_crossing_refuses_without_adopting_forged_candidate() {
    let repo = tempfile::tempdir().unwrap();
    let (service, decisions, authored) = author(repo.path());
    let mut candidate = read_candidate(repo.path(), &authored.candidate_ref);
    let old_path = candidate_path(repo.path(), &authored.candidate_ref);
    let binding = candidate["validation_result_binding"]
        .as_object_mut()
        .unwrap();
    let zero = "0".repeat(64);
    binding.insert(
        "validation_result_ref".to_owned(),
        Value::String(format!(
            "lifecycle-validation-results/lifecycle-validation-result_{zero}.json"
        )),
    );
    binding.insert(
        "validation_result_fingerprint".to_owned(),
        Value::String(format!("sha256:{zero}")),
    );
    let (forged_ref, forged_path, forged_bytes) =
        install_forged_candidate(repo.path(), &mut candidate);
    std::fs::remove_file(&old_path).unwrap();

    let refusal = service
        .persist(&decisions, author_envelope(), None)
        .expect_err("crossed semantic result must refuse");
    assert_eq!(
        refusal.kind(),
        CharterAuthorPersistenceErrorKindV1::PersistenceRefused
    );
    assert_eq!(std::fs::read(&forged_path).unwrap(), forged_bytes);
    assert!(forged_ref.starts_with("candidates/candidate_"));
}

#[test]
fn forged_second_matching_candidate_refuses_and_preserves_every_candidate() {
    let repo = tempfile::tempdir().unwrap();
    let (service, decisions, authored) = author(repo.path());
    let original_path = candidate_path(repo.path(), &authored.candidate_ref);
    let original_bytes = std::fs::read(&original_path).unwrap();
    let mut candidate: Value = serde_json::from_slice(&original_bytes).unwrap();
    candidate["validation_result_binding"]["result_document_sha256"] =
        Value::String(format!("sha256:{}", "0".repeat(64)));
    let (_, forged_path, forged_bytes) = install_forged_candidate(repo.path(), &mut candidate);

    let refusal = service
        .persist(&decisions, author_envelope(), None)
        .expect_err("two selector-matching candidates must refuse");
    assert_eq!(
        refusal.kind(),
        CharterAuthorPersistenceErrorKindV1::PersistenceRefused
    );
    assert_eq!(std::fs::read(original_path).unwrap(), original_bytes);
    assert_eq!(std::fs::read(forged_path).unwrap(), forged_bytes);
}

type CandidateFieldSourceMutation = Box<dyn Fn(&mut Value)>;

#[derive(Clone, Copy)]
enum CandidateFieldSourceGate {
    AuthorInventory,
    Approval,
    Promotion,
}

#[derive(Default)]
struct UnreachableApprovalPort;

impl NativeAuthenticatorPortV1 for UnreachableApprovalPort {
    fn make_credential(
        &mut self,
        _request_cbor: &[u8],
    ) -> Result<Vec<u8>, NativeAuthenticatorPortErrorV1> {
        panic!("candidate provenance refusal must precede credential creation")
    }

    fn get_assertion(
        &mut self,
        _request_cbor: &[u8],
    ) -> Result<Vec<u8>, NativeAuthenticatorPortErrorV1> {
        panic!("candidate provenance refusal must precede assertion request")
    }
}

#[test]
fn coherently_resigned_candidate_field_source_forgery_refuses_author_inventory() {
    assert_candidate_field_source_forgery_matrix(CandidateFieldSourceGate::AuthorInventory);
}

#[test]
fn coherently_resigned_candidate_field_source_forgery_refuses_approval() {
    assert_candidate_field_source_forgery_matrix(CandidateFieldSourceGate::Approval);
}

#[test]
fn coherently_resigned_candidate_field_source_forgery_refuses_promotion() {
    assert_candidate_field_source_forgery_matrix(CandidateFieldSourceGate::Promotion);
}

fn assert_candidate_field_source_forgery_matrix(gate: CandidateFieldSourceGate) {
    let cases: Vec<(&str, CandidateFieldSourceMutation)> = vec![
        (
            "empty",
            Box::new(|candidate| candidate["field_sources"] = Value::Array(Vec::new())),
        ),
        (
            "missing",
            Box::new(|candidate| {
                candidate["field_sources"].as_array_mut().unwrap().remove(0);
            }),
        ),
        (
            "reordered",
            Box::new(|candidate| {
                candidate["field_sources"]
                    .as_array_mut()
                    .unwrap()
                    .swap(0, 1);
            }),
        ),
        (
            "duplicate",
            Box::new(|candidate| {
                let duplicate = candidate["field_sources"][0].clone();
                candidate["field_sources"]
                    .as_array_mut()
                    .unwrap()
                    .insert(1, duplicate);
            }),
        ),
        (
            "unsafe-path",
            Box::new(|candidate| {
                candidate["field_sources"][0]["target_path"] =
                    Value::String("../policy/revision".to_owned());
            }),
        ),
        (
            "wrong-coverage",
            Box::new(|candidate| {
                candidate["field_sources"][0]["coverage_id"] =
                    Value::String("decisions.records".to_owned());
            }),
        ),
        (
            "unsupported-source-kind",
            Box::new(|candidate| {
                candidate["field_sources"][0]["source_kind"] = Value::String("approval".to_owned());
            }),
        ),
        (
            "source-free-leaf",
            Box::new(|candidate| {
                candidate["field_sources"].as_array_mut().unwrap().pop();
            }),
        ),
    ];

    for (case, mutator) in cases {
        let repo = tempfile::tempdir().unwrap();
        let (service, decisions, authored) = author(repo.path());
        let (candidate_ref, candidate_path, candidate_bytes, result_path, result_bytes) =
            install_coherent_field_source_forgery(repo.path(), &authored.candidate_ref, mutator);

        match gate {
            CandidateFieldSourceGate::AuthorInventory => {
                let refusal = match service.persist(&decisions, author_envelope(), None) {
                    Err(refusal) => refusal,
                    Ok(_) => panic!("{case} provenance forgery must refuse replay"),
                };
                assert_eq!(
                    refusal.kind(),
                    CharterAuthorPersistenceErrorKindV1::PersistenceRefused,
                    "{case}"
                );
            }
            CandidateFieldSourceGate::Approval => {
                let mut approvals =
                    CharterApprovalServiceV1::new(repo.path(), UnreachableApprovalPort);
                let result = approvals.approve_candidate(
                    &decisions,
                    CharterApprovalRequestV1 {
                        operation_id: format!("field-source-{case}"),
                        candidate_ref,
                        approval_class: "Project owner approval".to_owned(),
                        authority_ref: "Project owner".to_owned(),
                        accepted_waiver_refs: Vec::new(),
                    },
                );
                let CharterApprovalResultV1::Refused(refusal) = result else {
                    panic!("{case} provenance forgery must refuse approval")
                };
                assert_eq!(
                    refusal.code,
                    CharterApprovalRefusalCodeV1::LineageViolation,
                    "{case}"
                );
            }
            CandidateFieldSourceGate::Promotion => {
                let refusal = CharterPromotionWorkflowServiceV1::new(repo.path())
                    .promote(CharterPromotionIntentV1 {
                        candidate_ref,
                        approval_ref: format!("approvals/approval_{}.json", "a".repeat(64)),
                        expected_current_fingerprint: None,
                    })
                    .unwrap_err();
                assert_eq!(
                    refusal.kind(),
                    CharterPromotionWorkflowErrorKindV1::CandidateRefused,
                    "{case}"
                );
            }
        }
        assert_eq!(
            std::fs::read(candidate_path).unwrap(),
            candidate_bytes,
            "{case}"
        );
        assert_eq!(std::fs::read(result_path).unwrap(), result_bytes, "{case}");
        assert!(!repo.path().join(".handbook/project/charter.yaml").exists());
    }
}

#[test]
fn orphan_result_refuses_without_recreating_candidate() {
    let repo = tempfile::tempdir().unwrap();
    let (service, decisions, authored) = author(repo.path());
    let candidate = read_candidate(repo.path(), &authored.candidate_ref);
    let result_path = result_path(repo.path(), &candidate);
    let result_bytes = std::fs::read(&result_path).unwrap();
    let candidate_path = candidate_path(repo.path(), &authored.candidate_ref);
    std::fs::remove_file(&candidate_path).unwrap();

    let refusal = service
        .persist(&decisions, author_envelope(), None)
        .expect_err("orphan result must never be adopted");
    assert_eq!(
        refusal.kind(),
        CharterAuthorPersistenceErrorKindV1::PersistenceRefused
    );
    assert!(!candidate_path.exists());
    assert_eq!(std::fs::read(result_path).unwrap(), result_bytes);
}

#[test]
fn missing_result_refuses_without_rewriting_candidate() {
    let repo = tempfile::tempdir().unwrap();
    let (service, decisions, authored) = author(repo.path());
    let candidate_path = candidate_path(repo.path(), &authored.candidate_ref);
    let candidate_bytes = std::fs::read(&candidate_path).unwrap();
    let candidate: Value = serde_json::from_slice(&candidate_bytes).unwrap();
    let result_path = result_path(repo.path(), &candidate);
    std::fs::remove_file(&result_path).unwrap();

    let refusal = service
        .persist(&decisions, author_envelope(), None)
        .expect_err("candidate with missing result must refuse");
    assert_eq!(
        refusal.kind(),
        CharterAuthorPersistenceErrorKindV1::PersistenceRefused
    );
    assert_eq!(std::fs::read(candidate_path).unwrap(), candidate_bytes);
    assert!(!result_path.exists());
}

#[test]
fn unsafe_inventory_names_and_nonregular_entries_refuse_without_cleanup() {
    let repo = tempfile::tempdir().unwrap();
    let (service, decisions, _) = author(repo.path());
    let candidate_root = repo.path().join(".handbook/evidence/charter/candidates");
    let unsafe_file = candidate_root.join("candidate-not-owned.json");
    std::fs::write(&unsafe_file, b"retained unsafe evidence").unwrap();
    let refusal = service
        .persist(&decisions, author_envelope(), None)
        .expect_err("unowned candidate name must refuse");
    assert_eq!(
        refusal.kind(),
        CharterAuthorPersistenceErrorKindV1::PersistenceRefused
    );
    assert_eq!(
        std::fs::read(&unsafe_file).unwrap(),
        b"retained unsafe evidence"
    );
    std::fs::remove_file(&unsafe_file).unwrap();
    let unsafe_directory = candidate_root.join(format!("candidate_{}.json", "0".repeat(64)));
    std::fs::create_dir(&unsafe_directory).unwrap();
    let refusal = service
        .persist(&decisions, author_envelope(), None)
        .expect_err("owned name with nonregular type must refuse");
    assert_eq!(
        refusal.kind(),
        CharterAuthorPersistenceErrorKindV1::PersistenceRefused
    );
    assert!(unsafe_directory.is_dir());

    std::fs::remove_dir(&unsafe_directory).unwrap();
    let result_root = repo
        .path()
        .join(".handbook/state/lifecycle-validation-results");
    let unsafe_result = result_root.join("result-not-owned.json");
    std::fs::write(&unsafe_result, b"retained unsafe result evidence").unwrap();
    let refusal = service
        .persist(&decisions, author_envelope(), None)
        .expect_err("unowned result name must refuse");
    assert_eq!(
        refusal.kind(),
        CharterAuthorPersistenceErrorKindV1::PersistenceRefused
    );
    assert_eq!(
        std::fs::read(&unsafe_result).unwrap(),
        b"retained unsafe result evidence"
    );
    std::fs::remove_file(&unsafe_result).unwrap();
    let unsafe_result_directory = result_root.join(format!(
        "lifecycle-validation-result_{}.json",
        "0".repeat(64)
    ));
    std::fs::create_dir(&unsafe_result_directory).unwrap();
    let refusal = service
        .persist(&decisions, author_envelope(), None)
        .expect_err("owned result name with nonregular type must refuse");
    assert_eq!(
        refusal.kind(),
        CharterAuthorPersistenceErrorKindV1::PersistenceRefused
    );
    assert!(unsafe_result_directory.is_dir());
}

#[cfg(unix)]
#[test]
fn linked_candidate_and_result_entries_refuse_without_following_or_cleanup() {
    use std::os::unix::fs::symlink;

    for result_store in [false, true] {
        let repo = tempfile::tempdir().unwrap();
        let (service, decisions, _) = author(repo.path());
        let outside = repo.path().join("outside-authority.json");
        std::fs::write(&outside, b"outside retained evidence").unwrap();
        let linked = if result_store {
            repo.path()
                .join(".handbook/state/lifecycle-validation-results")
                .join(format!(
                    "lifecycle-validation-result_{}.json",
                    "0".repeat(64)
                ))
        } else {
            repo.path()
                .join(".handbook/evidence/charter/candidates")
                .join(format!("candidate_{}.json", "0".repeat(64)))
        };
        symlink(&outside, &linked).unwrap();

        let refusal = service
            .persist(&decisions, author_envelope(), None)
            .expect_err("linked authority entry must refuse");
        assert_eq!(
            refusal.kind(),
            CharterAuthorPersistenceErrorKindV1::PersistenceRefused
        );
        assert!(linked.symlink_metadata().unwrap().file_type().is_symlink());
        assert_eq!(
            std::fs::read(&outside).unwrap(),
            b"outside retained evidence"
        );
    }
}

#[test]
fn promotion_refuses_timestamp_rewrite_before_approval_or_canonical_mutation() {
    let repo = tempfile::tempdir().unwrap();
    let (_, _, authored) = author(repo.path());
    let candidate = read_candidate(repo.path(), &authored.candidate_ref);
    let result_path = result_path(repo.path(), &candidate);
    let mut result: Value = serde_json::from_slice(&std::fs::read(&result_path).unwrap()).unwrap();
    result["validated_at_utc"] = Value::String("2030-01-02T03:04:05Z".to_owned());
    let forged = result_bytes(&result);
    std::fs::write(&result_path, &forged).unwrap();

    let error = CharterPromotionWorkflowServiceV1::new(repo.path())
        .promote(CharterPromotionIntentV1 {
            candidate_ref: authored.candidate_ref,
            approval_ref: format!("approvals/approval_{}.json", "a".repeat(64)),
            expected_current_fingerprint: None,
        })
        .expect_err("exact result refusal must precede approval resolution");
    assert_eq!(
        error.kind(),
        CharterPromotionWorkflowErrorKindV1::CandidateRefused
    );
    assert_eq!(std::fs::read(result_path).unwrap(), forged);
    assert!(!repo.path().join(".handbook/project/charter.yaml").exists());
}

#[test]
fn historical_candidate_versions_are_promotion_ineligible_and_preserved() {
    for version in ["1.0", "1.1", "1.2"] {
        let repo = tempfile::tempdir().unwrap();
        let (_, _, authored) = author(repo.path());
        let current = read_candidate(repo.path(), &authored.candidate_ref);
        let (historical_ref, historical_path, historical_bytes) =
            install_historical_candidate(repo.path(), &current, version);

        let error = CharterPromotionWorkflowServiceV1::new(repo.path())
            .promote(CharterPromotionIntentV1 {
                candidate_ref: historical_ref,
                approval_ref: format!("approvals/approval_{}.json", "a".repeat(64)),
                expected_current_fingerprint: None,
            })
            .expect_err("historical candidates require reauthoring and a new approval");

        assert_eq!(
            error.kind(),
            CharterPromotionWorkflowErrorKindV1::CandidateRefused
        );
        assert_eq!(std::fs::read(historical_path).unwrap(), historical_bytes);
        assert!(!repo.path().join(".handbook/project/charter.yaml").exists());
    }
}

#[test]
fn exact_historical_candidate_versions_are_safe_author_replay_nonmatches() {
    for version in ["1.0", "1.1", "1.2"] {
        let repo = tempfile::tempdir().unwrap();
        let (service, decisions, authored) = author(repo.path());
        let current = read_candidate(repo.path(), &authored.candidate_ref);
        let (_, historical_path, historical_bytes) =
            install_historical_candidate(repo.path(), &current, version);

        let replayed = service
            .persist(&decisions, author_envelope(), None)
            .expect("exact historical candidate must remain a safe nonmatch");

        assert_eq!(replayed, authored);
        assert_eq!(std::fs::read(historical_path).unwrap(), historical_bytes);
    }
}

#[test]
fn historical_candidate_wrong_internal_id_refuses_author_replay_without_mutation() {
    let repo = tempfile::tempdir().unwrap();
    let (service, decisions, authored) = author(repo.path());
    let current = read_candidate(repo.path(), &authored.candidate_ref);
    let (_, historical_path, _) = install_historical_candidate(repo.path(), &current, "1.1");
    let mut historical: Value =
        serde_json::from_slice(&std::fs::read(&historical_path).unwrap()).unwrap();
    historical["candidate_id"] = Value::String(format!("candidate_{}", "0".repeat(64)));
    let forged = serde_json_canonicalizer::to_vec(&historical).unwrap();
    std::fs::write(&historical_path, &forged).unwrap();

    let refusal = service
        .persist(&decisions, author_envelope(), None)
        .expect_err("historical candidate internal ID mismatch must make inventory unsafe");

    assert_eq!(
        refusal.kind(),
        CharterAuthorPersistenceErrorKindV1::PersistenceRefused
    );
    assert_eq!(std::fs::read(historical_path).unwrap(), forged);
}

#[test]
fn candidate_v10_with_additive_v11_basis_refuses_author_replay_without_mutation() {
    let repo = tempfile::tempdir().unwrap();
    let (service, decisions, authored) = author(repo.path());
    let current = read_candidate(repo.path(), &authored.candidate_ref);
    let (_, historical_path, historical_bytes) =
        install_historical_candidate(repo.path(), &current, "1.0");
    let mut malformed: Value = serde_json::from_slice(&historical_bytes).unwrap();
    malformed["basis_artifact_fingerprint"] = Value::Null;
    std::fs::remove_file(historical_path).unwrap();
    let (_, malformed_path, malformed_bytes) =
        install_forged_candidate(repo.path(), &mut malformed);

    let refusal = service
        .persist(&decisions, author_envelope(), None)
        .expect_err("candidate 1.0 with the additive 1.1 basis field must make inventory unsafe");

    assert_eq!(
        refusal.kind(),
        CharterAuthorPersistenceErrorKindV1::PersistenceRefused
    );
    assert_eq!(std::fs::read(malformed_path).unwrap(), malformed_bytes);
}

type HistoricalCandidateMutation = Box<dyn Fn(&mut Value)>;

#[test]
fn malformed_historical_candidate_semantics_refuse_author_replay_without_mutation() {
    let cases: Vec<(&str, HistoricalCandidateMutation)> = vec![
        (
            "intake-ref",
            Box::new(|candidate| {
                candidate["intake_record_ref"] = Value::String("../intake.json".to_owned());
            }),
        ),
        (
            "kind-ref",
            Box::new(|candidate| {
                candidate["target_kind_ref"] = Value::String("not-an-exact-ref".to_owned());
            }),
        ),
        (
            "instance-id",
            Box::new(|candidate| {
                candidate["target_instance_id"] = Value::String("Not_Symbolic".to_owned());
            }),
        ),
        (
            "profile-fingerprint",
            Box::new(|candidate| {
                candidate["resolved_profile_fingerprint"] =
                    Value::String(format!("sha256:{}", "A".repeat(64)));
            }),
        ),
        (
            "content-ref",
            Box::new(|candidate| {
                candidate["normalized_content_ref"] =
                    Value::String("candidates/../candidate.yaml".to_owned());
            }),
        ),
        (
            "basis-fingerprint",
            Box::new(|candidate| {
                candidate["basis_artifact_fingerprint"] =
                    Value::String("sha256:not-a-digest".to_owned());
            }),
        ),
        (
            "empty-source-map",
            Box::new(|candidate| {
                candidate["field_sources"] = Value::Array(Vec::new());
            }),
        ),
        (
            "open-source-map-row",
            Box::new(|candidate| {
                candidate["field_sources"][0]["extra"] = Value::String("forged".to_owned());
            }),
        ),
        (
            "unsupported-source-kind",
            Box::new(|candidate| {
                candidate["field_sources"][0]["source_kind"] = Value::String("approval".to_owned());
            }),
        ),
        (
            "duplicate-target-path",
            Box::new(|candidate| {
                let duplicate = candidate["field_sources"][0].clone();
                candidate["field_sources"]
                    .as_array_mut()
                    .unwrap()
                    .push(duplicate);
            }),
        ),
        (
            "invalid-unresolved-coverage",
            Box::new(|candidate| {
                candidate["unresolved_coverage_ids"] = serde_json::json!(["not/a/coverage-id"]);
            }),
        ),
        (
            "duplicate-result-ref",
            Box::new(|candidate| {
                candidate["validation_result_refs"] =
                    serde_json::json!(["validation_001", "validation_001"]);
            }),
        ),
        (
            "promotion-eligibility",
            Box::new(|candidate| {
                candidate["promotion_eligibility"] = Value::String("approved".to_owned());
            }),
        ),
        (
            "approval-policy-ref",
            Box::new(|candidate| {
                candidate["required_approval_policy_ref"] =
                    Value::String("not-an-exact-ref".to_owned());
            }),
        ),
    ];
    for (case, mutator) in cases {
        assert_malformed_historical_candidate_refuses("1.1", case, mutator);
    }
    assert_malformed_historical_candidate_refuses(
        "1.2",
        "missing-result-ref",
        Box::new(|candidate| candidate["validation_result_refs"] = Value::Array(Vec::new())),
    );
    assert_malformed_historical_candidate_refuses(
        "1.2",
        "wrong-subject-fingerprint",
        Box::new(|candidate| {
            candidate["candidate_subject_fingerprint"] =
                Value::String(format!("sha256:{}", "0".repeat(64)));
        }),
    );
}

#[test]
fn coherently_rewritten_stale_policy_authority_refuses_and_is_preserved() {
    coherent_semantic_authority_rewrite_refuses(|result| {
        result["lifecycle_policy_fingerprint"] =
            Value::String(format!("sha256:{}", "0".repeat(64)));
    });
}

#[test]
fn coherently_rewritten_stale_canonical_and_lifecycle_authority_refuses() {
    coherent_semantic_authority_rewrite_refuses(|result| {
        install_nonabsent_lifecycle_authority(result, false);
    });
}

#[test]
fn coherently_rewritten_stale_observation_and_coverage_authority_refuses() {
    coherent_semantic_authority_rewrite_refuses(|result| {
        install_nonabsent_lifecycle_authority(result, true);
    });
}

fn candidate_binding_mutation_refuses(mutator: impl FnOnce(&mut Value)) {
    let repo = tempfile::tempdir().unwrap();
    let (service, decisions, authored) = author(repo.path());
    let old_path = candidate_path(repo.path(), &authored.candidate_ref);
    let mut candidate = read_candidate(repo.path(), &authored.candidate_ref);
    mutator(&mut candidate["validation_result_binding"]);
    let (_, forged_path, forged_bytes) = install_forged_candidate(repo.path(), &mut candidate);
    std::fs::remove_file(old_path).unwrap();
    let result_path = result_path(repo.path(), &candidate);
    let result_bytes = std::fs::read(&result_path).unwrap();

    let refusal = service
        .persist(&decisions, author_envelope(), None)
        .expect_err("forged exact-result binding must refuse");
    assert_eq!(
        refusal.kind(),
        CharterAuthorPersistenceErrorKindV1::PersistenceRefused
    );
    assert_eq!(std::fs::read(forged_path).unwrap(), forged_bytes);
    assert_eq!(std::fs::read(result_path).unwrap(), result_bytes);
}

fn coherent_semantic_authority_rewrite_refuses(mutator: impl FnOnce(&mut Value)) {
    let repo = tempfile::tempdir().unwrap();
    let (service, decisions, authored) = author(repo.path());
    let old_candidate_path = candidate_path(repo.path(), &authored.candidate_ref);
    let mut candidate = read_candidate(repo.path(), &authored.candidate_ref);
    let old_result_path = result_path(repo.path(), &candidate);
    let mut result: Value =
        serde_json::from_slice(&std::fs::read(&old_result_path).unwrap()).unwrap();
    mutator(&mut result);
    let mut semantic_preimage = result.clone();
    for field in [
        "validation_result_id",
        "validation_result_fingerprint",
        "validated_at_utc",
    ] {
        semantic_preimage.as_object_mut().unwrap().remove(field);
    }
    let semantic_fingerprint = DefinitionFingerprint::from_json_value(&semantic_preimage)
        .unwrap()
        .to_string();
    let validation_id = format!(
        "lifecycle-validation-result_{}",
        semantic_fingerprint.strip_prefix("sha256:").unwrap()
    );
    result["validation_result_id"] = Value::String(validation_id.clone());
    result["validation_result_fingerprint"] = Value::String(semantic_fingerprint.clone());
    let result_ref = format!("lifecycle-validation-results/{validation_id}.json");
    let result_bytes = result_bytes(&result);
    candidate["validation_result_binding"] = serde_json::json!({
        "validation_result_ref": result_ref,
        "validation_result_fingerprint": semantic_fingerprint,
        "result_document_sha256": DefinitionFingerprint::from_bytes(&result_bytes).to_string(),
        "result_byte_length": result_bytes.len(),
    });
    let (candidate_ref, candidate_path, candidate_bytes) =
        install_forged_candidate(repo.path(), &mut candidate);
    let result_path = result_path(repo.path(), &candidate);
    std::fs::write(&result_path, &result_bytes).unwrap();
    std::fs::remove_file(old_candidate_path).unwrap();
    std::fs::remove_file(old_result_path).unwrap();

    let replay = service
        .persist(&decisions, author_envelope(), None)
        .expect_err("coherently stale semantic authority must refuse");
    assert_eq!(
        replay.kind(),
        CharterAuthorPersistenceErrorKindV1::PersistenceRefused
    );
    let promotion = CharterPromotionWorkflowServiceV1::new(repo.path())
        .promote(CharterPromotionIntentV1 {
            candidate_ref,
            approval_ref: format!("approvals/approval_{}.json", "a".repeat(64)),
            expected_current_fingerprint: None,
        })
        .expect_err("promotion must independently refuse stale semantic authority");
    assert_eq!(
        promotion.kind(),
        CharterPromotionWorkflowErrorKindV1::CandidateRefused
    );
    assert_eq!(std::fs::read(candidate_path).unwrap(), candidate_bytes);
    assert_eq!(std::fs::read(result_path).unwrap(), result_bytes);
    assert!(!repo.path().join(".handbook/project/charter.yaml").exists());
}

fn schema_invalid_unrelated_result_refuses(mutator: impl FnOnce(&mut Value)) {
    let repo = tempfile::tempdir().unwrap();
    let (service, decisions, authored) = author(repo.path());
    let candidate = read_candidate(repo.path(), &authored.candidate_ref);
    let source_path = result_path(repo.path(), &candidate);
    let mut result: Value = serde_json::from_slice(&std::fs::read(&source_path).unwrap()).unwrap();
    result["candidate_subject_fingerprint"] = Value::String(format!("sha256:{}", "0".repeat(64)));
    mutator(&mut result);
    let mut semantic_preimage = result.clone();
    for field in [
        "validation_result_id",
        "validation_result_fingerprint",
        "validated_at_utc",
    ] {
        semantic_preimage.as_object_mut().unwrap().remove(field);
    }
    let semantic_fingerprint = DefinitionFingerprint::from_json_value(&semantic_preimage)
        .unwrap()
        .to_string();
    let validation_id = format!(
        "lifecycle-validation-result_{}",
        semantic_fingerprint.strip_prefix("sha256:").unwrap()
    );
    result["validation_result_id"] = Value::String(validation_id.clone());
    result["validation_result_fingerprint"] = Value::String(semantic_fingerprint);
    let unrelated_path = repo
        .path()
        .join(".handbook/state/lifecycle-validation-results")
        .join(format!("{validation_id}.json"));
    let bytes = result_bytes(&result);
    std::fs::write(&unrelated_path, &bytes).unwrap();

    let refusal = service
        .persist(&decisions, author_envelope(), None)
        .expect_err("schema-invalid unrelated result must make inventory unsafe");
    assert_eq!(
        refusal.kind(),
        CharterAuthorPersistenceErrorKindV1::PersistenceRefused
    );
    assert_eq!(std::fs::read(unrelated_path).unwrap(), bytes);
}

fn install_nonabsent_lifecycle_authority(result: &mut Value, with_observation: bool) {
    let zero = "0".repeat(64);
    let fingerprint = Value::String(format!("sha256:{zero}"));
    result["basis_artifact_fingerprint"] = fingerprint.clone();
    result["observed_current_artifact_fingerprint"] = fingerprint.clone();
    result["lifecycle_head_ref"] = Value::String(format!(
        "lifecycle-transitions/lifecycle-transition_{zero}.json"
    ));
    result["lifecycle_head_fingerprint"] = fingerprint.clone();
    result["lifecycle_state"] = Value::String("review_required".to_owned());
    result["lifecycle_state_fingerprint"] = fingerprint.clone();
    if with_observation {
        result["active_observations"] = serde_json::json!([{
            "observation_ref": format!("lifecycle-observations/lifecycle-observation_{zero}.json"),
            "observation_fingerprint": fingerprint,
        }]);
        result["reopened_coverage_ids"] = serde_json::json!(["policy.authority_and_revision"]);
    }
}

fn author(
    repo_root: &Path,
) -> (
    CharterAuthorPersistenceServiceV1,
    handbook_engine::ResolvedProfileDecisions,
    CharterAuthorPersistenceResultV1,
) {
    let decisions =
        resolve_shipped_profile_decisions(Path::new(env!("CARGO_MANIFEST_DIR"))).unwrap();
    let service = CharterAuthorPersistenceServiceV1::new(repo_root);
    let authored = service
        .persist(&decisions, author_envelope(), None)
        .expect("candidate 1.3 first authoring");
    (service, decisions, authored)
}

fn candidate_path(repo_root: &Path, candidate_ref: &str) -> PathBuf {
    repo_root
        .join(".handbook/evidence/charter")
        .join(candidate_ref)
}

fn read_candidate(repo_root: &Path, candidate_ref: &str) -> Value {
    serde_json::from_slice(&std::fs::read(candidate_path(repo_root, candidate_ref)).unwrap())
        .unwrap()
}

fn result_path(repo_root: &Path, candidate: &Value) -> PathBuf {
    repo_root.join(".handbook/state").join(
        candidate["validation_result_binding"]["validation_result_ref"]
            .as_str()
            .unwrap(),
    )
}

fn recompute_subject(candidate: &Value) -> String {
    let mut preimage = candidate.clone();
    let object = preimage.as_object_mut().unwrap();
    for field in [
        "candidate_id",
        "candidate_fingerprint",
        "candidate_subject_fingerprint",
        "validation_result_binding",
    ] {
        object.remove(field);
    }
    DefinitionFingerprint::from_json_value(&preimage)
        .unwrap()
        .to_string()
}

fn install_forged_candidate(repo_root: &Path, candidate: &mut Value) -> (String, PathBuf, Vec<u8>) {
    candidate.as_object_mut().unwrap().remove("candidate_id");
    candidate
        .as_object_mut()
        .unwrap()
        .remove("candidate_fingerprint");
    let fingerprint = DefinitionFingerprint::from_json_value(candidate)
        .unwrap()
        .to_string();
    let id = format!("candidate_{}", fingerprint.strip_prefix("sha256:").unwrap());
    candidate["candidate_id"] = Value::String(id.clone());
    candidate["candidate_fingerprint"] = Value::String(fingerprint);
    let bytes = serde_json_canonicalizer::to_vec(candidate).unwrap();
    let candidate_ref = format!("candidates/{id}.json");
    let path = candidate_path(repo_root, &candidate_ref);
    std::fs::write(&path, &bytes).unwrap();
    (candidate_ref, path, bytes)
}

fn install_coherent_field_source_forgery(
    repo_root: &Path,
    original_candidate_ref: &str,
    mutator: CandidateFieldSourceMutation,
) -> (String, PathBuf, Vec<u8>, PathBuf, Vec<u8>) {
    let original_candidate_path = candidate_path(repo_root, original_candidate_ref);
    let mut candidate = read_candidate(repo_root, original_candidate_ref);
    let original_result_path = result_path(repo_root, &candidate);
    let mut result: Value =
        serde_json::from_slice(&std::fs::read(&original_result_path).unwrap()).unwrap();

    mutator(&mut candidate);
    let subject_fingerprint = recompute_subject(&candidate);
    candidate["candidate_subject_fingerprint"] = Value::String(subject_fingerprint.clone());
    result["candidate_subject_fingerprint"] = Value::String(subject_fingerprint);

    let mut semantic_preimage = result.clone();
    for field in [
        "validation_result_id",
        "validation_result_fingerprint",
        "validated_at_utc",
    ] {
        semantic_preimage.as_object_mut().unwrap().remove(field);
    }
    let semantic_fingerprint = DefinitionFingerprint::from_json_value(&semantic_preimage)
        .unwrap()
        .to_string();
    let validation_id = format!(
        "lifecycle-validation-result_{}",
        semantic_fingerprint.strip_prefix("sha256:").unwrap()
    );
    result["validation_result_id"] = Value::String(validation_id.clone());
    result["validation_result_fingerprint"] = Value::String(semantic_fingerprint.clone());
    let validation_result_ref = format!("lifecycle-validation-results/{validation_id}.json");
    let validation_result_bytes = result_bytes(&result);
    candidate["validation_result_binding"] = serde_json::json!({
        "validation_result_ref": validation_result_ref,
        "validation_result_fingerprint": semantic_fingerprint,
        "result_document_sha256": DefinitionFingerprint::from_bytes(&validation_result_bytes).to_string(),
        "result_byte_length": validation_result_bytes.len(),
    });
    let (candidate_ref, forged_candidate_path, candidate_bytes) =
        install_forged_candidate(repo_root, &mut candidate);
    let forged_result_path = result_path(repo_root, &candidate);
    std::fs::write(&forged_result_path, &validation_result_bytes).unwrap();
    std::fs::remove_file(original_candidate_path).unwrap();
    std::fs::remove_file(original_result_path).unwrap();
    (
        candidate_ref,
        forged_candidate_path,
        candidate_bytes,
        forged_result_path,
        validation_result_bytes,
    )
}

fn install_historical_candidate(
    repo_root: &Path,
    current: &Value,
    version: &str,
) -> (String, PathBuf, Vec<u8>) {
    let result_ref = current["validation_result_binding"]["validation_result_ref"].clone();
    let mut candidate = current.clone();
    candidate["schema_version"] = Value::String(version.to_owned());
    candidate
        .as_object_mut()
        .unwrap()
        .remove("validation_result_binding");
    candidate["validation_result_refs"] = if version == "1.2" {
        Value::Array(vec![result_ref])
    } else {
        Value::Array(vec![Value::String("validation_001".to_owned())])
    };
    if version == "1.2" {
        let mut subject = candidate.clone();
        for field in [
            "candidate_id",
            "candidate_fingerprint",
            "candidate_subject_fingerprint",
            "validation_result_refs",
        ] {
            subject.as_object_mut().unwrap().remove(field);
        }
        candidate["candidate_subject_fingerprint"] = Value::String(
            DefinitionFingerprint::from_json_value(&subject)
                .unwrap()
                .to_string(),
        );
    } else {
        candidate["normalized_content_ref"] = Value::String("candidates/candidate.yaml".to_owned());
        candidate
            .as_object_mut()
            .unwrap()
            .remove("candidate_subject_fingerprint");
        if version == "1.0" {
            candidate
                .as_object_mut()
                .unwrap()
                .remove("basis_artifact_fingerprint");
        }
    }
    candidate.as_object_mut().unwrap().remove("candidate_id");
    candidate
        .as_object_mut()
        .unwrap()
        .remove("candidate_fingerprint");
    let fingerprint = DefinitionFingerprint::from_json_value(&candidate)
        .unwrap()
        .to_string();
    let id = format!("candidate_{}", fingerprint.strip_prefix("sha256:").unwrap());
    candidate["candidate_id"] = Value::String(id.clone());
    candidate["candidate_fingerprint"] = Value::String(fingerprint);
    let bytes = serde_json_canonicalizer::to_vec(&candidate).unwrap();
    let relative_ref = format!("candidates/{id}.json");
    let path = candidate_path(repo_root, &relative_ref);
    std::fs::write(&path, &bytes).unwrap();
    (relative_ref, path, bytes)
}

fn assert_malformed_historical_candidate_refuses(
    version: &str,
    case: &str,
    mutator: Box<dyn Fn(&mut Value)>,
) {
    let repo = tempfile::tempdir().unwrap();
    let (service, decisions, authored) = author(repo.path());
    let current = read_candidate(repo.path(), &authored.candidate_ref);
    let (_, historical_path, historical_bytes) =
        install_historical_candidate(repo.path(), &current, version);
    let mut malformed: Value = serde_json::from_slice(&historical_bytes).unwrap();
    mutator(&mut malformed);
    std::fs::remove_file(historical_path).unwrap();
    let (_, malformed_path, malformed_bytes) =
        install_forged_candidate(repo.path(), &mut malformed);

    let refusal = match service.persist(&decisions, author_envelope(), None) {
        Err(refusal) => refusal,
        Ok(_) => panic!("{case} historical candidate must make inventory unsafe"),
    };
    assert_eq!(
        refusal.kind(),
        CharterAuthorPersistenceErrorKindV1::PersistenceRefused,
        "{case}"
    );
    assert_eq!(
        std::fs::read(malformed_path).unwrap(),
        malformed_bytes,
        "{case}"
    );
}

fn result_bytes(result: &Value) -> Vec<u8> {
    let mut bytes = serde_json_canonicalizer::to_vec(result).unwrap();
    bytes.push(b'\n');
    bytes
}

fn assert_exact_keys(value: &Value, expected: &[&str]) {
    let observed = value
        .as_object()
        .unwrap()
        .keys()
        .map(String::as_str)
        .collect::<BTreeSet<_>>();
    let expected = expected.iter().copied().collect::<BTreeSet<_>>();
    assert_eq!(observed, expected);
}

fn author_envelope() -> CharterIntakeEnvelope {
    let observational = [
        "project_shape.definition",
        "delivery.constraints",
        "operational_reality.production_state",
        "risk.domains",
        "debt.register",
    ];
    let normative = [
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
    let mut coverage = Vec::new();
    for coverage_id in observational {
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
    for coverage_id in normative {
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
        content: parse_definition_yaml(CANONICAL_CHARTER).unwrap(),
        coverage,
        consumer: CharterIntakeConsumer {
            kind: "handbook_skill".to_owned(),
            id: "handbook".to_owned(),
            version: "1.1".to_owned(),
        },
        prompt_event_refs: vec![],
        finalized_at_utc: "2026-07-20T16:30:00Z".to_owned(),
        expected_current_fingerprint: None,
    }
}
