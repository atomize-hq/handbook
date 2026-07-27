use handbook_engine::{
    resolve_shipped_profile_decisions, ArtifactApplicability, DefinitionFingerprint,
    ExactDefinitionRef, SchemaRegistry, SymbolicId,
};
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};

const VECTOR_PATH: &str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../docs/specs/handbook-contract-membrane/slices/HCM-2.2/contracts/definition-closure-vectors-v1.0.json"
);
const NORMALIZED_JCS_PATH: &str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../docs/specs/handbook-contract-membrane/slices/HCM-2.2/contracts/definition-normalized-jcs-v1.jsonl"
);

fn exact(value: &str) -> ExactDefinitionRef {
    ExactDefinitionRef::parse(value).expect("published exact definition ref")
}

fn vector() -> Value {
    serde_json::from_slice(&fs::read(VECTOR_PATH).expect("definition closure vector"))
        .expect("valid definition closure vector")
}

fn sha256(bytes: &[u8]) -> String {
    format!("sha256:{:x}", Sha256::digest(bytes))
}

fn definition_path(reference: &str) -> PathBuf {
    let relative = match reference {
        "handbook.semantic-validation.constitutional-root@1.1.0" => {
            "semantic-validators/handbook.semantic-validation.constitutional-root/1.1.0.yaml"
        }
        "handbook.approval.constitutional-candidate@1.0.0" => {
            "approval-policies/handbook.approval.constitutional-candidate/1.0.0.yaml"
        }
        "handbook.waiver.constitutional-intake@1.0.0" => {
            "waiver-policies/handbook.waiver.constitutional-intake/1.0.0.yaml"
        }
        "handbook.intake-trigger.production-posture-changed@1.0.0" => {
            "triggers/handbook.intake-trigger.production-posture-changed/1.0.0.yaml"
        }
        "handbook.intake-trigger.trust-boundary-changed@1.0.0" => {
            "triggers/handbook.intake-trigger.trust-boundary-changed/1.0.0.yaml"
        }
        "handbook.lifecycle-trigger.charter-amendment-proposed@1.0.0" => {
            "triggers/handbook.lifecycle-trigger.charter-amendment-proposed/1.0.0.yaml"
        }
        "handbook.renderer.charter-review-markdown@1.0.0" => {
            "renderers/handbook.renderer.charter-review-markdown/1.0.0.yaml"
        }
        "handbook.lifecycle.constitutional-review-lock@1.0.0" => {
            "lifecycle-policies/handbook.lifecycle.constitutional-review-lock/1.0.0.yaml"
        }
        "handbook.artifact-kind.project-authority@1.1.0" => {
            "artifact-kinds/handbook.artifact-kind.project-authority/1.1.0.yaml"
        }
        "handbook.intake.charter@1.0.0" => "intakes/handbook.intake.charter/1.0.0.yaml",
        "handbook.profile.shipped-root@1.1.0" => {
            "profiles/handbook.profile.shipped-root/1.1.0.yaml"
        }
        _ => panic!("unmapped published definition: {reference}"),
    };
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("definitions")
        .join(relative)
}

#[test]
fn eleven_exact_schema_documents_are_registered_without_changing_old_entries() {
    let vector = vector();
    let entries = vector["schema_entries"]
        .as_object()
        .expect("schema-entry vectors");
    assert_eq!(entries.len(), 11);

    let slice_root = Path::new(VECTOR_PATH).parent().unwrap().parent().unwrap();
    let mut entry_paths = Vec::new();
    for (reference, expected) in entries {
        let entry = &expected["entry"];
        let document_ref = entry["document_ref"].as_str().unwrap();
        let entry_ref = document_ref.replace(".schema.json", ".entry.yaml");
        let entry_path = Path::new(env!("CARGO_MANIFEST_DIR")).join(&entry_ref);
        let document_path = Path::new(env!("CARGO_MANIFEST_DIR")).join(document_ref);
        let contract_path = slice_root.join(expected["document_contract_path"].as_str().unwrap());

        let authored_entry = handbook_engine::parse_definition_yaml(
            &fs::read(&entry_path).unwrap_or_else(|error| panic!("{reference}: {error}")),
        )
        .unwrap();
        assert_eq!(&authored_entry, entry, "{reference}");

        let document = fs::read(&document_path).unwrap();
        assert_eq!(document, fs::read(contract_path).unwrap(), "{reference}");
        assert_eq!(
            document.len() as u64,
            expected["document_byte_length"].as_u64().unwrap(),
            "{reference}"
        );
        assert_eq!(
            sha256(&document),
            entry["document_fingerprint"],
            "{reference}"
        );
        entry_paths.push(entry_ref);
    }

    let registry = SchemaRegistry::load(
        env!("CARGO_MANIFEST_DIR"),
        &entry_paths,
        &["definitions/schemas".to_owned()],
    )
    .expect("the exact eleven-schema additive registry loads");
    for reference in entries.keys() {
        assert!(registry.entry(&exact(reference)).is_some(), "{reference}");
    }
    let trigger_evidence = registry
        .resolved(&exact("handbook.schemas.lifecycle.trigger-evidence@1.0.0"))
        .expect("trigger-evidence schema");
    let valid_evidence = json!({
        "schema_id": "handbook.lifecycle-trigger-evidence",
        "schema_version": "1.0",
        "event_id": "event.one",
        "target_instance_id": "project_authority",
        "basis_canonical_fingerprint": format!("sha256:{}", "0".repeat(64)),
        "event_kind": "charter_amendment_proposed",
        "prior_evidence_ref": "evidence/prior.json",
        "prior_evidence_fingerprint": format!("sha256:{}", "1".repeat(64)),
        "current_evidence_ref": "evidence/current.json",
        "current_evidence_fingerprint": format!("sha256:{}", "2".repeat(64)),
        "observed_at_utc": "2026-07-19T23:59:58Z",
        "extensions": {},
        "evidence_fingerprint": format!("sha256:{}", "3".repeat(64)),
    });
    assert!(trigger_evidence.validate_json(&valid_evidence).is_ok());
    let mut invalid_evidence = valid_evidence;
    invalid_evidence["observed_at_utc"] = json!("not-a-date-time");
    assert!(trigger_evidence.validate_json(&invalid_evidence).is_err());
    assert!(registry
        .entry(&exact("handbook.schemas.artifacts.project-authority@1.0.0"))
        .is_none());
}

#[test]
fn uniform_definition_identities_match_every_published_vector() {
    let vector = vector();
    let definitions = vector["definition_vectors"]
        .as_object()
        .expect("definition vectors");
    assert_eq!(definitions.len(), 11);
    let normalized = fs::read_to_string(NORMALIZED_JCS_PATH).unwrap();
    let normalized_lines = normalized.lines().collect::<Vec<_>>();

    for (reference, expected) in definitions {
        let authored = handbook_engine::parse_definition_yaml(
            &fs::read(definition_path(reference))
                .unwrap_or_else(|error| panic!("{reference}: {error}")),
        )
        .unwrap();
        let mut expected_authored = expected["fingerprint_preimage"]["definition"].clone();
        expected_authored.as_object_mut().unwrap().insert(
            expected["own_fingerprint_field"]
                .as_str()
                .unwrap()
                .to_owned(),
            expected["expected_fingerprint"].clone(),
        );
        assert_eq!(authored, expected_authored, "{reference}");

        let preimage = &expected["fingerprint_preimage"];
        let jcs = serde_json_canonicalizer::to_vec(preimage).unwrap();
        assert_eq!(
            jcs.len() as u64,
            expected["normalized_jcs_byte_length"].as_u64().unwrap(),
            "{reference}"
        );
        assert_eq!(
            sha256(&jcs),
            expected["expected_fingerprint"],
            "{reference}"
        );
        let line = expected["normalized_jcs_line"].as_u64().unwrap() as usize - 1;
        assert_eq!(jcs, normalized_lines[line].as_bytes(), "{reference}");
        assert_eq!(
            DefinitionFingerprint::from_json_value(preimage)
                .unwrap()
                .as_str(),
            expected["expected_fingerprint"].as_str().unwrap(),
            "{reference}"
        );
    }
}

#[test]
fn shipped_profile_1_2_preserves_the_critical_charter_closure_additively() {
    let decisions = resolve_shipped_profile_decisions(env!("CARGO_MANIFEST_DIR"))
        .expect("shipped profile 1.2 decisions");
    assert_eq!(
        decisions.profile_ref(),
        &exact("handbook.profile.shipped-root@1.2.0")
    );

    let registry = decisions.registry();
    for (kind_ref, schema_ref) in [
        (
            "handbook.artifact-kind.project-authority@1.0.0",
            "handbook.schemas.artifacts.project-authority@1.0.0",
        ),
        (
            "handbook.artifact-kind.project-authority@1.1.0",
            "handbook.schemas.artifacts.project-authority@1.1.0",
        ),
    ] {
        assert_eq!(
            registry
                .kind(&exact(kind_ref))
                .unwrap()
                .canonical_schema_ref(),
            &exact(schema_ref)
        );
    }

    let charter_id = SymbolicId::parse("project_authority").unwrap();
    let charter = registry.instance(&charter_id).unwrap();
    assert_eq!(
        charter.kind_ref(),
        &exact("handbook.artifact-kind.project-authority@1.1.0")
    );
    assert_eq!(
        charter.intake_definition_ref(),
        Some(&exact("handbook.intake.charter@1.0.0"))
    );
    assert_eq!(
        charter.lifecycle_policy_ref(),
        Some(&exact(
            "handbook.lifecycle.constitutional-review-lock@1.0.0"
        ))
    );
    assert_eq!(
        charter.renderer_definition_refs(),
        &[exact("handbook.renderer.charter-review-markdown@1.0.0")]
    );

    let charter_decision = decisions
        .artifact_decisions()
        .iter()
        .find(|decision| decision.instance_id().as_str() == "project_authority")
        .unwrap();
    assert_eq!(
        charter_decision.applicability(),
        ArtifactApplicability::Required
    );
    let kind = registry.kind(charter.kind_ref()).unwrap();
    let capability = kind
        .capabilities()
        .iter()
        .next()
        .expect("constitutional capability");
    assert_eq!(capability.bindings().len(), 9);

    let expected_other_kinds = BTreeMap::from([
        (
            "project_context",
            "handbook.artifact-kind.project-context@1.1.0",
        ),
        (
            "environment_context",
            "handbook.artifact-kind.environment-context@1.1.0",
        ),
    ]);
    for (id, reference) in expected_other_kinds {
        assert_eq!(
            registry
                .instance(&SymbolicId::parse(id).unwrap())
                .unwrap()
                .kind_ref(),
            &exact(reference)
        );
    }
}
