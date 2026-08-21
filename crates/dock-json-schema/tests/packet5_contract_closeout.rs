mod packet5_support;
#[path = "../package-support/production.rs"]
mod production;
#[path = "../package-support/staging.rs"]
#[allow(dead_code)]
mod staging;

use handbook_contracts::{
    ClaimVerdictDecisionV1, ContentAddressedBindingV1, ContractOwnerRepositoryErrorKindV1,
    FingerprintV1, GateDecisionV1, LocalCloseoutDecisionV1, OwnerMutationDispositionV1,
};
use handbook_dock::{
    host_target_tuple_v1, DockManifestListRequestV1, DockManifestReadOutcomeV1,
    DockManifestRegistryV1, DockManifestV1,
};
use handbook_sdk::contract_membrane::{
    operation_inventory, IdempotencyStateRef, ProductionWitness, TerminalStatus, UtcTimestamp,
};
use packet5_support::{
    candidate_file_count, derived_state_is_absent, evidence_file_count, owner_candidate, owner_run,
    parse_run_result, EvidenceFixture, Packet5Fixture,
};
use serde_json::Value;
use sha2::{Digest, Sha256};
use std::fs;
use std::path::Path;
use std::sync::OnceLock;

fn production_manifest() -> DockManifestV1 {
    let registry = DockManifestRegistryV1::production();
    let page = registry
        .list(DockManifestListRequestV1::first(10).unwrap())
        .unwrap();
    assert_eq!(page.entries().len(), 1);
    match registry
        .read(
            page.entries()[0].manifest_ref(),
            page.entries()[0].manifest_fingerprint(),
        )
        .unwrap()
    {
        DockManifestReadOutcomeV1::Found(manifest) => *manifest,
        DockManifestReadOutcomeV1::Absent => panic!("production validator manifest is absent"),
    }
}

fn production_executable_bytes() -> &'static [u8] {
    static EXECUTABLE: OnceLock<Vec<u8>> = OnceLock::new();
    EXECUTABLE
        .get_or_init(|| {
            let target_directory = tempfile::tempdir().unwrap();
            let workspace = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
            let package =
                production::produce_current_package_v1(&workspace, target_directory.path())
                    .unwrap();
            let bytes = fs::read(&package.executable_path).unwrap();
            assert_eq!(bytes.len(), 9_548_800);
            assert_eq!(
                format!("sha256:{:x}", Sha256::digest(&bytes)),
                "sha256:067d9d4386382ed1fc97f312336fc8a148bb135269679d4bb46c5058fe18d411"
            );
            bytes
        })
        .as_slice()
}

struct InstalledProductionPackage {
    root: std::path::PathBuf,
}

impl InstalledProductionPackage {
    fn install() -> Self {
        let host = std::env::current_exe().unwrap();
        let host = host.parent().unwrap();
        let package_root = host
            .join("handbook-docks/handbook.dock.json-schema/1.0.0")
            .join(host_target_tuple_v1());
        assert!(
            !package_root.exists(),
            "production test package must start absent"
        );
        let source = tempfile::tempdir().unwrap();
        let executable = source.path().join(format!(
            "handbook-dock-json-schema{}",
            std::env::consts::EXE_SUFFIX
        ));
        fs::write(&executable, production_executable_bytes()).unwrap();
        let staged = staging::stage_native_package_v1(
            &executable,
            host,
            env!("HANDBOOK_DOCK_JSON_SCHEMA_TARGET"),
            std::env::consts::EXE_SUFFIX,
        )
        .unwrap();
        assert_eq!(staged.package_root, package_root);
        Self { root: package_root }
    }
}

impl Drop for InstalledProductionPackage {
    fn drop(&mut self) {
        if self.root.exists() {
            fs::remove_dir_all(&self.root).unwrap();
            let version = self.root.parent().unwrap();
            let dock = version.parent().unwrap();
            let docks = dock.parent().unwrap();
            fs::remove_dir(version).unwrap();
            fs::remove_dir(dock).unwrap();
            fs::remove_dir(docks).unwrap();
        }
    }
}

#[test]
fn exact_fixture_reaches_contracts_owned_positive_and_negative_local_closeout() {
    let _installed = InstalledProductionPackage::install();
    let manifest = production_manifest();
    let fixture = Packet5Fixture::new(manifest.clone());
    assert_eq!(fixture.actor_refs().len(), 4);
    assert_eq!(fixture.manifest().binding(), manifest.binding());
    assert!(derived_state_is_absent(fixture.repository_path()));

    let positive_request = fixture.run_request(
        "hcm-5-4-positive-evaluation-0001",
        "hcm-5-4-positive-dock-key-0001",
        "instances/conformant.json",
    );
    let positive = fixture.execute(&positive_request);
    assert_eq!(positive.terminal().status(), TerminalStatus::Ok);
    assert_eq!(positive.receipts().len(), 1);
    assert!(matches!(
        positive.idempotency().as_ref(),
        IdempotencyStateRef::Established {
            replayed: false,
            ..
        }
    ));
    let positive_result = parse_run_result(&positive);
    assert_eq!(positive_result.candidates().len(), 1);
    let positive_run = owner_run(&positive_result);
    let positive_candidate = owner_candidate(&positive_result);

    let replay = fixture.execute(&fixture.run_request(
        "hcm-5-4-positive-evaluation-0001",
        "hcm-5-4-positive-dock-key-0001",
        "instances/conformant.json",
    ));
    let replay_result = parse_run_result(&replay);
    assert_eq!(replay.receipts(), positive.receipts());
    assert_eq!(
        replay_result.execution_record(),
        positive_result.execution_record()
    );
    assert_eq!(replay_result.candidates(), positive_result.candidates());
    assert!(matches!(
        replay.idempotency().as_ref(),
        IdempotencyStateRef::Established { replayed: true, .. }
    ));

    let conflict = fixture.execute(&fixture.run_request(
        "hcm-5-4-positive-evaluation-0001",
        "hcm-5-4-positive-dock-key-0001",
        "instances/nonconformant.json",
    ));
    assert_eq!(conflict.terminal().status(), TerminalStatus::Refused);
    assert!(conflict.receipts().is_empty());

    let negative = fixture.execute(&fixture.run_request(
        "hcm-5-4-negative-evaluation-0001",
        "hcm-5-4-negative-dock-key-0001",
        "instances/nonconformant.json",
    ));
    assert_eq!(negative.terminal().status(), TerminalStatus::Ok);
    let negative_result = parse_run_result(&negative);
    assert_eq!(negative_result.candidates().len(), 1);
    let negative_run = owner_run(&negative_result);
    let negative_candidate = owner_candidate(&negative_result);

    let currentness_candidate_response = fixture.execute(&fixture.run_request(
        "hcm-5-4-currentness-evaluation-0001",
        "hcm-5-4-currentness-dock-key-0001",
        "instances/conformant.json",
    ));
    let currentness_candidate_result = parse_run_result(&currentness_candidate_response);
    let currentness_run = owner_run(&currentness_candidate_result);
    let currentness_candidate = owner_candidate(&currentness_candidate_result);

    let candidates_before_refusal = candidate_file_count(fixture.repository_path());
    let evidence_before_refusal = evidence_file_count(fixture.repository_path());
    let substituted = fixture.execute(&fixture.substituted_instance_request(
        "hcm-5-4-substituted-evaluation-0001",
        "hcm-5-4-substituted-dock-key-0001",
    ));
    assert_eq!(substituted.terminal().status(), TerminalStatus::Ok);
    let substituted_result = parse_run_result(&substituted);
    assert!(substituted_result.candidates().is_empty());
    let substituted_record =
        serde_json::to_value(substituted_result.execution_record().document()).unwrap();
    assert_eq!(substituted_record["host_outcome"], "refused");
    assert_eq!(
        candidate_file_count(fixture.repository_path()),
        candidates_before_refusal
    );
    assert_eq!(
        evidence_file_count(fixture.repository_path()),
        evidence_before_refusal
    );
    assert!(derived_state_is_absent(fixture.repository_path()));

    let under_resolved = fixture.execute(&fixture.under_resolved_request(
        "hcm-5-4-under-resolution-evaluation-0001",
        "hcm-5-4-under-resolution-dock-key-0001",
    ));
    assert_eq!(under_resolved.terminal().status(), TerminalStatus::Refused);
    assert!(under_resolved.receipts().is_empty());
    assert_eq!(
        candidate_file_count(fixture.repository_path()),
        candidates_before_refusal
    );
    assert_eq!(
        evidence_file_count(fixture.repository_path()),
        evidence_before_refusal
    );
    assert!(derived_state_is_absent(fixture.repository_path()));

    assert_candidate_fact(
        fixture.repository_path(),
        &positive_candidate,
        "satisfied",
        "conformant",
    );
    assert_candidate_fact(
        fixture.repository_path(),
        &negative_candidate,
        "violated",
        "nonconformant",
    );

    let evidence_actor = fixture.actor_refs()[3].as_str().to_owned();
    let mut evidence = fixture.into_evidence();
    let positive_append = evidence.append(
        &positive_candidate,
        &positive_run,
        "hcm-5-4-positive-evidence-key-0001",
    );
    assert_eq!(
        positive_append.disposition(),
        OwnerMutationDispositionV1::Committed
    );
    assert!(
        String::from_utf8(positive_append.record().canonical_json_bytes().unwrap())
            .unwrap()
            .contains(&evidence_actor)
    );
    let positive_replay = evidence.append(
        &positive_candidate,
        &positive_run,
        "hcm-5-4-positive-evidence-key-0001",
    );
    assert_eq!(
        positive_replay.disposition(),
        OwnerMutationDispositionV1::Replayed
    );
    assert_eq!(positive_replay.record(), positive_append.record());
    assert_eq!(positive_replay.receipt(), positive_append.receipt());

    let negative_append = evidence.append(
        &negative_candidate,
        &negative_run,
        "hcm-5-4-negative-evidence-key-0001",
    );
    assert_eq!(
        negative_append.disposition(),
        OwnerMutationDispositionV1::Committed
    );
    assert_eq!(evidence.evidence_count(), 2);

    assert_eq!(
        evidence.append_refusal(
            &negative_candidate,
            &negative_run,
            "hcm-5-4-positive-evidence-key-0001",
        ),
        ContractOwnerRepositoryErrorKindV1::IdempotencyConflict
    );
    let missing_after_timeout = ContentAddressedBindingV1::new(
        "handbook.dock.missing-after-timeout@1.0.0#candidate/timeout",
        FingerprintV1::parse(&format!("sha256:{}", "9".repeat(64))).unwrap(),
    )
    .unwrap();
    assert_eq!(
        evidence.append_refusal(
            &missing_after_timeout,
            &positive_run,
            "hcm-5-4-timeout-evidence-key-0001",
        ),
        ContractOwnerRepositoryErrorKindV1::EvidenceCandidateMissing
    );
    assert_eq!(evidence.evidence_count(), 2);

    let candidates_before_currentness_refusal = candidate_file_count(evidence.repository_path());
    let currentness_refusal = evidence.append_with_substituted_currentness(
        &currentness_candidate,
        &currentness_run,
        "hcm-5-4-currentness-evidence-key-0001",
    );
    assert_eq!(
        currentness_refusal,
        ContractOwnerRepositoryErrorKindV1::EvidenceSourceRefused
    );
    assert_eq!(
        candidate_file_count(evidence.repository_path()),
        candidates_before_currentness_refusal
    );
    assert_eq!(evidence.evidence_count(), 2);

    let positive_evaluation = evidence.evaluate(&positive_run);
    EvidenceFixture::assert_outcome(
        &positive_evaluation,
        ClaimVerdictDecisionV1::Pass,
        GateDecisionV1::Passed,
        LocalCloseoutDecisionV1::Eligible,
    );
    let negative_evaluation = evidence.evaluate(&negative_run);
    EvidenceFixture::assert_outcome(
        &negative_evaluation,
        ClaimVerdictDecisionV1::Fail,
        GateDecisionV1::Blocked,
        LocalCloseoutDecisionV1::Blocked,
    );
    assert_eq!(evidence.evidence_count(), 2);

    let mut stale_fixture = Packet5Fixture::new(manifest);
    let stale_request = stale_fixture.run_request(
        "hcm-5-4-stale-evaluation-0001",
        "hcm-5-4-stale-dock-key-0001",
        "instances/conformant.json",
    );
    stale_fixture.deprecate();
    let stale_candidates = candidate_file_count(stale_fixture.repository_path());
    let stale_evidence = evidence_file_count(stale_fixture.repository_path());
    let stale = stale_fixture.execute(&stale_request);
    assert_eq!(stale.terminal().status(), TerminalStatus::Refused);
    assert!(stale.receipts().is_empty());
    assert_eq!(
        candidate_file_count(stale_fixture.repository_path()),
        stale_candidates
    );
    assert_eq!(
        evidence_file_count(stale_fixture.repository_path()),
        stale_evidence
    );
    assert!(derived_state_is_absent(stale_fixture.repository_path()));

    let persisted = repository_bytes(evidence.repository_path());
    for forbidden in ["claim_verdict", "gate_result", "local_closeout_result"] {
        assert!(!String::from_utf8_lossy(&persisted).contains(forbidden));
    }
}

#[test]
fn timeout_control_remains_only_in_the_existing_test_native_host_fixture() {
    let production_main = include_str!("../src/main.rs");
    let native_host_fixture = include_str!("../../dock/test-support/native_helper.rs");
    assert!(!production_main.contains("sleep("));
    assert!(!production_main.contains("delay"));
    assert!(native_host_fixture.contains("\"sleep\""));
    assert!(native_host_fixture.contains("Duration::from_millis"));
}

#[test]
fn packet5_preserves_the_exact_interim_public_inventory() {
    let witness =
        ProductionWitness::from_current(UtcTimestamp::parse("9999-12-31T23:59:59Z").unwrap())
            .unwrap();
    assert_eq!(operation_inventory().len(), 62);
    assert_eq!(witness.capabilities().len(), 31);
    assert_eq!(witness.schema_manifest().len(), 168);
    assert_eq!(DockManifestRegistryV1::production().production_len(), 1);

    let fixed_use_cases = [
        "handbook.use-case.repository-setup@1.0.0",
        "handbook.use-case.repository-doctor@1.0.0",
        "handbook.use-case.author-charter@1.0.0",
        "handbook.use-case.author-project-context@1.0.0",
        "handbook.use-case.approver-administration@1.0.0",
        "handbook.use-case.pipeline-route-resolve-and-persist@1.0.0",
    ];
    let source = include_str!("../../sdk/src/contract_membrane/facade/request.rs");
    for use_case in fixed_use_cases {
        assert!(source.contains(use_case));
        assert!(!operation_inventory()
            .iter()
            .any(|row| row.operation_ref().as_str() == use_case));
        assert!(!witness
            .capabilities()
            .iter()
            .any(|entry| entry.operation().reference().as_str() == use_case));
    }
}

fn assert_candidate_fact(
    repo: &Path,
    candidate: &handbook_contracts::ContentAddressedBindingV1,
    observation: &str,
    conformance: &str,
) {
    let file = repo
        .join(".handbook/state/dock/v1/candidates")
        .join(format!("{}.json", &candidate.fingerprint().as_str()[7..]));
    let stored: Value = serde_json::from_slice(&fs::read(file).unwrap()).unwrap();
    let candidate: Value = serde_json::from_str(stored["candidate_jcs"].as_str().unwrap()).unwrap();
    assert_eq!(
        candidate["body"]["evidence_kind"],
        "handbook.evidence.schema-conformance"
    );
    assert_eq!(
        candidate["body"]["fact_schema"]["ref"],
        "handbook.fact.schema-conformance@1.0.0"
    );
    assert_eq!(candidate["body"]["observation"], observation);
    assert_eq!(
        candidate["body"]["fact_payload"]["conformance"],
        conformance
    );
    assert_eq!(
        candidate["body"]["observed_claim_ids"],
        serde_json::json!(["payload.schema-conformant"])
    );
    assert_eq!(
        candidate["body"]["claim_partition"]["observed"],
        serde_json::json!(["payload.schema-conformant"])
    );
    assert_eq!(
        candidate["body"]["claim_partition"]["unobserved"],
        serde_json::json!([])
    );
    assert_eq!(
        candidate["body"]["claim_partition"]["excluded"],
        serde_json::json!([])
    );
    for forbidden in ["claim_verdict", "gate", "local_closeout"] {
        assert!(candidate["body"].get(forbidden).is_none());
    }
}

fn repository_bytes(root: &Path) -> Vec<u8> {
    fn visit(path: &Path, output: &mut Vec<u8>) {
        if path.is_dir() {
            let mut entries = fs::read_dir(path)
                .unwrap()
                .map(|entry| entry.unwrap().path())
                .collect::<Vec<_>>();
            entries.sort();
            for entry in entries {
                visit(&entry, output);
            }
        } else {
            output.extend(fs::read(path).unwrap());
        }
    }
    let mut output = Vec::new();
    visit(root, &mut output);
    output
}
