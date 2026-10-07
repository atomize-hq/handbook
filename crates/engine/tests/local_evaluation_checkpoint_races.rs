//! Separate-process checkpoint worker for actual owner-created SDK fixtures.
//! This target creates no Charter, governance, terms, issuance, or set records.
//! The ordinary consumer proof remains a separate normal-dependency program.
use handbook_engine::local_authority::local_check::LocalCheckCurrentBasisV1;
use handbook_engine::local_authority::local_evaluation::*;
use handbook_engine::local_authority::LocalAuthorityRefusalV1;
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use std::time::{Duration, Instant};

#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
struct NativeCheckpointFixture {
    repository_path: PathBuf,
    authority: LocalEvaluationAuthorityLocatorV1,
    current_basis: LocalCheckCurrentBasisV1,
    operation: CheckpointOperation,
}

#[derive(Deserialize, Serialize)]
#[serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)]
enum CheckpointOperation {
    Formation {
        request: LocalEvaluationFormationRequestV1,
    },
    Verification {
        original: LocalEvaluationBindingLocatorV1,
        request: LocalEvaluationAuthorizationRequestV1,
        checkpoint: LocalEvaluationCheckpointV1,
        checkpoint_target: Option<String>,
    },
    FinalContext {
        original: LocalEvaluationBindingLocatorV1,
        request: LocalEvaluationAuthorizationRequestV1,
        source: LocalEvaluationFinalSetSourceV1,
    },
}

#[derive(Serialize)]
struct NativeCheckpointObservation {
    admitted: bool,
    refusal: Option<LocalAuthorityRefusalV1>,
    authority_fingerprint: Option<String>,
    purpose_fingerprint: Option<String>,
    issuance_fingerprint: Option<String>,
    verification: Option<LocalEvaluationVerificationObservationV1>,
}

fn admitted_terms(
    admission: &LocalEvaluationTermsAdmissionV1<'_>,
) -> Result<NativeCheckpointObservation, LocalAuthorityRefusalV1> {
    admission.validate_current()?;
    Ok(NativeCheckpointObservation {
        admitted: true,
        refusal: None,
        authority_fingerprint: Some(admission.authority_fingerprint().into()),
        purpose_fingerprint: None,
        issuance_fingerprint: None,
        verification: None,
    })
}

fn admitted_evaluation(
    admission: &LocalEvaluationAdmissionV1<'_>,
) -> Result<NativeCheckpointObservation, LocalAuthorityRefusalV1> {
    admission.validate_current()?;
    Ok(NativeCheckpointObservation {
        admitted: true,
        refusal: None,
        authority_fingerprint: None,
        purpose_fingerprint: Some(admission.binding().purpose_fingerprint.clone()),
        issuance_fingerprint: Some(admission.binding().issuance_fingerprint.clone()),
        verification: Some(admission.observation().clone()),
    })
}

fn run_fixture(
    fixture: &NativeCheckpointFixture,
) -> Result<NativeCheckpointObservation, LocalAuthorityRefusalV1> {
    match &fixture.operation {
        CheckpointOperation::Formation { request } => {
            NativeLocalEvaluationAuthorityV1::with_terms_admission(
                &fixture.repository_path,
                &fixture.authority,
                &fixture.current_basis,
                request,
                admitted_terms,
            )
        }
        CheckpointOperation::Verification {
            original,
            request,
            checkpoint,
            checkpoint_target,
        } => NativeLocalEvaluationAuthorityV1::with_verification(
            &fixture.repository_path,
            &fixture.authority,
            original,
            request,
            *checkpoint,
            checkpoint_target.as_deref(),
            &fixture.current_basis,
            &request.reach,
            admitted_evaluation,
        ),
        CheckpointOperation::FinalContext {
            original,
            request,
            source,
        } => NativeLocalEvaluationAuthorityV1::with_final_verification(
            &fixture.repository_path,
            &fixture.authority,
            original,
            request,
            source,
            &fixture.current_basis,
            &request.reach,
            admitted_evaluation,
        ),
    }
}

fn write_canonical(path: &Path, value: &impl Serialize) {
    let bytes = serde_json_canonicalizer::to_vec(value).unwrap();
    std::fs::write(path, bytes).unwrap();
}

/// The SDK native integration launches this exact test in another process with
/// `--ignored --exact native_checkpoint_process_worker`. Its handoff contains
/// portable untrusted inputs only; every permission comes from native rereads.
#[test]
#[ignore = "requires an actual owner-created native fixture and process barrier"]
fn native_checkpoint_process_worker() {
    let fixture_path = PathBuf::from(
        std::env::var_os("HANDBOOK_ENGINE_CHECKPOINT_FIXTURE")
            .expect("supply exact native checkpoint fixture path"),
    );
    let barrier = PathBuf::from(
        std::env::var_os("HANDBOOK_ENGINE_CHECKPOINT_BARRIER")
            .expect("supply checkpoint process barrier path"),
    );
    assert!(fixture_path.is_absolute() && barrier.is_absolute());
    assert!(barrier.is_dir());
    let bytes = std::fs::read(fixture_path).unwrap();
    let value = handbook_engine::parse_schema_json(&bytes).unwrap();
    assert_eq!(serde_json_canonicalizer::to_vec(&value).unwrap(), bytes);
    let fixture: NativeCheckpointFixture = serde_json::from_value(value).unwrap();
    assert!(fixture.repository_path.is_absolute());
    assert!(!barrier.join("ready.json").exists());
    assert!(!barrier.join("outcome.json").exists());
    write_canonical(
        &barrier.join("ready.json"),
        &serde_json::json!({
            "phase": "before_native_admission", "pid": std::process::id()
        }),
    );
    let deadline = Instant::now() + Duration::from_secs(1200);
    while !barrier.join("release").exists() {
        assert!(
            Instant::now() < deadline,
            "native process barrier timed out"
        );
        std::thread::sleep(Duration::from_millis(20));
    }
    // Native authority resolution may need more than a test-harness stack.
    let worker = std::thread::Builder::new()
        .stack_size(8 * 1024 * 1024)
        .spawn(move || run_fixture(&fixture))
        .unwrap();
    let observation = match worker.join().unwrap() {
        Ok(value) => value,
        Err(refusal) => NativeCheckpointObservation {
            admitted: false,
            refusal: Some(refusal),
            authority_fingerprint: None,
            purpose_fingerprint: None,
            issuance_fingerprint: None,
            verification: None,
        },
    };
    write_canonical(&barrier.join("outcome.json"), &observation);
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct NativeCheckpointMatrix {
    formation_fixture: PathBuf,
    launch_fixture: PathBuf,
    evidence_append_fixture: PathBuf,
    final_fixture: PathBuf,
    observations_directory: PathBuf,
}

fn read_fixture(path: &Path) -> NativeCheckpointFixture {
    let bytes = std::fs::read(path).unwrap();
    let value = handbook_engine::parse_schema_json(&bytes).unwrap();
    assert_eq!(serde_json_canonicalizer::to_vec(&value).unwrap(), bytes);
    serde_json::from_value(value).unwrap()
}

fn process_observation(
    fixture: &NativeCheckpointFixture,
    observations: &Path,
    name: &str,
    change: impl FnOnce(),
) -> serde_json::Value {
    use std::process::{Command, Stdio};
    eprintln!("native Engine checkpoint matrix: {name}");
    let barrier = observations.join(name);
    std::fs::create_dir(&barrier).unwrap();
    let input = barrier.join("fixture.json");
    write_canonical(&input, fixture);
    let mut process = Command::new(std::env::current_exe().unwrap())
        .args([
            "--ignored",
            "--exact",
            "native_checkpoint_process_worker",
            "--nocapture",
        ])
        .env("HANDBOOK_ENGINE_CHECKPOINT_FIXTURE", input)
        .env("HANDBOOK_ENGINE_CHECKPOINT_BARRIER", &barrier)
        .stdout(Stdio::null())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();
    let deadline = Instant::now() + Duration::from_secs(60);
    while !barrier.join("ready.json").exists() {
        assert!(
            Instant::now() < deadline,
            "child did not reach checkpoint: {name}"
        );
        assert!(
            process.try_wait().unwrap().is_none(),
            "child exited before checkpoint: {name}"
        );
        std::thread::sleep(Duration::from_millis(20));
    }
    change();
    std::fs::write(barrier.join("release"), b"release original exact request").unwrap();
    let output = process.wait_with_output().unwrap();
    assert!(
        output.status.success(),
        "{name}: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let bytes = std::fs::read(barrier.join("outcome.json")).unwrap();
    let value = handbook_engine::parse_schema_json(&bytes).unwrap();
    assert_eq!(serde_json_canonicalizer::to_vec(&value).unwrap(), bytes);
    value
}

fn assert_refused(name: &str, observation: &serde_json::Value) {
    assert_eq!(observation["admitted"], false, "{name}: {observation}");
    assert!(
        observation["refusal"]["code"].as_str().is_some(),
        "{name}: {observation}"
    );
    assert!(observation["verification"].is_null());
}

fn changed_file_case(
    fixture: &NativeCheckpointFixture,
    observations: &Path,
    name: &str,
    relative_path: &str,
) -> serde_json::Value {
    let path = fixture.repository_path.join(relative_path);
    let before = std::fs::read(&path).unwrap();
    let outcome = process_observation(fixture, observations, name, || {
        std::fs::write(&path, b"substituted native owner bytes").unwrap();
    });
    std::fs::write(&path, before).unwrap();
    assert_refused(name, &outcome);
    outcome
}

/// Actual SDK-created D fixtures supply four stages. The parent pauses a child
/// before each check, changes one real retained owner source, and restores that
/// source only after the child has returned its serialized native refusal.
#[test]
#[ignore = "requires actual current D fixtures exported by the SDK native target"]
fn native_checkpoint_substitution_matrix() {
    let matrix_path = PathBuf::from(
        std::env::var_os("HANDBOOK_ENGINE_CHECKPOINT_MATRIX")
            .expect("supply actual native checkpoint matrix manifest"),
    );
    assert!(matrix_path.is_absolute());
    let bytes = std::fs::read(matrix_path).unwrap();
    let value = handbook_engine::parse_schema_json(&bytes).unwrap();
    assert_eq!(serde_json_canonicalizer::to_vec(&value).unwrap(), bytes);
    let matrix: NativeCheckpointMatrix = serde_json::from_value(value).unwrap();
    assert!(matrix.observations_directory.is_absolute());
    assert!(!matrix.observations_directory.exists());
    std::fs::create_dir(&matrix.observations_directory).unwrap();
    let formation = read_fixture(&matrix.formation_fixture);
    assert!(
        !matrix
            .observations_directory
            .canonicalize()
            .unwrap()
            .starts_with(formation.repository_path.canonicalize().unwrap()),
        "process barriers must be outside the repository being substituted"
    );
    let launch = read_fixture(&matrix.launch_fixture);
    let append = read_fixture(&matrix.evidence_append_fixture);
    let final_context = read_fixture(&matrix.final_fixture);
    assert!(matches!(
        formation.operation,
        CheckpointOperation::Formation { .. }
    ));
    assert!(matches!(
        launch.operation,
        CheckpointOperation::Verification {
            checkpoint: LocalEvaluationCheckpointV1::Launch,
            ..
        }
    ));
    assert!(matches!(
        append.operation,
        CheckpointOperation::Verification {
            checkpoint: LocalEvaluationCheckpointV1::EvidenceAppend,
            ..
        }
    ));
    assert!(matches!(
        final_context.operation,
        CheckpointOperation::FinalContext { .. }
    ));
    for fixture in [&launch, &append, &final_context] {
        assert_eq!(fixture.repository_path, formation.repository_path);
        assert_eq!(fixture.authority, formation.authority);
        assert_eq!(fixture.current_basis, formation.current_basis);
    }
    let mut outcomes = Vec::new();
    for (name, fixture) in [
        ("baseline-formation", &formation),
        ("baseline-launch", &launch),
        ("baseline-evidence-append", &append),
        ("baseline-final-after-set", &final_context),
    ] {
        let outcome = process_observation(fixture, &matrix.observations_directory, name, || {});
        assert_eq!(outcome["admitted"], true, "{name}: {outcome}");
        assert!(outcome["refusal"].is_null());
        outcomes.push(serde_json::json!({"case":name,"outcome":outcome}));
    }
    for (name, fixture, path, expected_code) in [
        (
            "charter-before-formation",
            &formation,
            ".handbook/project/charter.yaml".to_owned(),
            "required_local_charter_state_missing",
        ),
        (
            "evaluation-head-before-launch",
            &launch,
            format!("{LOCAL_EVALUATION_ROOT_V1}/current.json"),
            "closed_json",
        ),
        (
            "predecessor-head-before-append",
            &append,
            ".handbook/local-operating/local-check-v1/current.json".to_owned(),
            "closed_json",
        ),
        (
            "immutable-terms-before-append",
            &append,
            format!(
                "{LOCAL_EVALUATION_ROOT_V1}/authority/{}/request.json",
                append.authority.terms.request_key
            ),
            "owner_history",
        ),
    ] {
        let outcome = changed_file_case(fixture, &matrix.observations_directory, name, &path);
        assert_eq!(
            outcome["refusal"]["code"], expected_code,
            "{name}: {outcome}"
        );
        outcomes.push(serde_json::json!({"case":name,"outcome":outcome}));
    }
    let source = match &final_context.operation {
        CheckpointOperation::FinalContext { source, .. } => source,
        _ => unreachable!(),
    };
    let outcome = changed_file_case(
        &final_context,
        &matrix.observations_directory,
        "source-substitution-after-set",
        &source.relative_path,
    );
    assert_eq!(outcome["refusal"]["code"], "retained_state");
    outcomes.push(serde_json::json!({"case":"source-substitution-after-set","outcome":outcome}));

    let original = match &launch.operation {
        CheckpointOperation::Verification { original, .. } => original,
        _ => unreachable!(),
    };
    let supersession = launch.repository_path.join(format!(
        "{LOCAL_EVALUATION_ROOT_V1}/issuances/{}/supersession.json",
        original.request_key
    ));
    assert!(!supersession.exists());
    let outcome = process_observation(
        &launch,
        &matrix.observations_directory,
        "unrecognized-supersession-before-launch",
        || {
            std::fs::write(&supersession, b"caller written supersession claim").unwrap();
        },
    );
    std::fs::remove_file(supersession).unwrap();
    assert_refused("unrecognized supersession", &outcome);
    assert_eq!(outcome["refusal"]["code"], "supersession_unverifiable");
    outcomes.push(
        serde_json::json!({"case":"unrecognized-supersession-before-launch","outcome":outcome}),
    );

    let lock = launch
        .repository_path
        .join(".handbook/state/locks/local-operating.lock");
    let saved = launch
        .repository_path
        .join(".handbook/state/locks/local-operating.engine-matrix-original");
    assert!(!saved.exists());
    let lock_bytes = std::fs::read(&lock).unwrap();
    let outcome = process_observation(
        &launch,
        &matrix.observations_directory,
        "missing-lock-before-launch",
        || std::fs::rename(&lock, &saved).unwrap(),
    );
    assert!(
        !lock.exists(),
        "current check must not create its missing original lock"
    );
    std::fs::rename(&saved, &lock).unwrap();
    assert_refused("missing lock", &outcome);
    assert_eq!(outcome["refusal"]["code"], "required_lock_state_missing");
    outcomes.push(serde_json::json!({"case":"missing-lock-before-launch","outcome":outcome}));
    let outcome = process_observation(
        &append,
        &matrix.observations_directory,
        "byte-equal-replaced-lock-before-append",
        || {
            std::fs::rename(&lock, &saved).unwrap();
            std::fs::write(&lock, &lock_bytes).unwrap();
        },
    );
    std::fs::remove_file(&lock).unwrap();
    std::fs::rename(&saved, &lock).unwrap();
    assert_refused("replaced lock", &outcome);
    assert_eq!(outcome["refusal"]["code"], "operating_lock_substituted");
    outcomes.push(
        serde_json::json!({"case":"byte-equal-replaced-lock-before-append","outcome":outcome}),
    );

    // Keep the path identical while replacing the actual repository directory.
    // Both targets are exact temporary SDK-fixture directories in one parent.
    let original_root = launch.repository_path.canonicalize().unwrap();
    let displaced_root = original_root.with_extension("engine-matrix-original");
    assert_eq!(original_root.parent(), displaced_root.parent());
    assert!(!displaced_root.exists());
    let outcome = process_observation(
        &launch,
        &matrix.observations_directory,
        "actual-root-substitution-before-launch",
        || {
            std::fs::rename(&original_root, &displaced_root).unwrap();
            std::fs::create_dir(&original_root).unwrap();
        },
    );
    assert!(!original_root.join(".handbook-initialization.lock").exists());
    std::fs::remove_dir(&original_root).unwrap();
    std::fs::rename(&displaced_root, &original_root).unwrap();
    assert_refused("root substitution", &outcome);
    assert_eq!(outcome["refusal"]["code"], "retained_state");
    outcomes.push(
        serde_json::json!({"case":"actual-root-substitution-before-launch","outcome":outcome}),
    );
    write_canonical(
        &matrix.observations_directory.join("matrix.json"),
        &outcomes,
    );
    assert_eq!(outcomes.len(), 13);
}
