//! Owner interruption tests consume the actual native fixture and live Engine
//! admissions. Fault injection changes delivery boundaries, never authority.
use super::*;
use handbook_contracts::OwnerRepositoryFaultBoundaryV1 as Fault;

#[path = "qualification_acceptance.rs"]
mod qualification_acceptance;

#[test]
fn native_final_closure_revalidates_admitted_dock_dependencies() {
    on_stack(|| {
        let _package = InstalledProductionPackage::install();
        let resume = std::env::var_os("HANDBOOK_NATIVE_FINAL_SOURCE_PUBLIC_HANDOFF");
        let fixture = resume.as_ref().map_or_else(Fixture::new, |path| {
            Fixture::from_public_handoff(std::path::Path::new(path))
        });
        let formed = if resume.is_some() {
            serde_json::from_slice::<Formed>(
                &std::fs::read(
                    fixture
                        .checkpoint_artifacts
                        .path()
                        .join("formed-packet3-final-native-source-0001.json"),
                )
                .unwrap(),
            )
            .unwrap()
        } else {
            fixture.form(
                "packet3-final-native-source-0001",
                supplied_validation_content(),
                production_manifest().resolution_ceiling().clone(),
                LocalEvaluationUseIntentV1::DecisionSeeking,
            )
        };
        let session = fixture.session(&formed);
        let fixed = if resume.is_some() {
            fixture
                .sdk
                .recover_local_evaluation(&formed.snapshot.locator)
                .unwrap()
        } else {
            fixture.assess(&formed);
            fixture
                .sdk
                .commit_local_evaluation_collection(&session)
                .unwrap();
            let attempted = fixture
                .sdk
                .execute_local_evaluation_once(
                    &session,
                    formed.process_request.as_ref().unwrap(),
                    &dock_reach(),
                    &DockCancellationV1::new(),
                )
                .unwrap();
            assert!(matches!(
                attempted,
                DockLocalEvaluationAttemptOutcomeV1::Terminal { ref terminal, .. }
                    if terminal.kind == "completed" && terminal.candidates.len() == 1
            ));
            fixture
                .sdk
                .admit_local_evaluation_origin(
                    &session,
                    &dock_reach(),
                    "packet3-final-source-admit",
                )
                .unwrap();
            fixture
                .sdk
                .seal_local_evaluation_frontier(&session)
                .unwrap();
            fixture.sdk.close_local_evaluation_set(&session).unwrap()
        };
        assert_eq!(fixed.stage, LocalEvaluationStageV3::SetClosed);
        let fixed_bytes = retained_bytes(&fixture, &fixed);
        let original_root = fixture.root.path().canonicalize().unwrap();

        // Change actual original producer records only after their evidence is
        // admitted and fixed. Restoring bytes supplies no new source or grant.
        // A retained run may continue after its already verified package probe;
        // the default run always exercises the complete source population.
        let completed_prefix = usize::from(
            resume.is_some()
                && std::env::var_os("HANDBOOK_NATIVE_FINAL_SOURCE_RESUME_AFTER_PACKAGE").is_some(),
        );
        for (name, relative, expected_kind) in [
            (
                "package member",
                fixed
                    .origin_paths
                    .iter()
                    .find(|path| path.contains("/packages/") && !path.ends_with("/manifest.json"))
                    .unwrap(),
                DockLocalEvaluationErrorKindV1::Substituted,
            ),
            (
                "request",
                fixed
                    .origin_paths
                    .iter()
                    .find(|path| path.contains("/requests/"))
                    .unwrap(),
                DockLocalEvaluationErrorKindV1::Unavailable,
            ),
            (
                "candidate",
                fixed
                    .origin_paths
                    .iter()
                    .find(|path| path.contains("/candidates/"))
                    .unwrap(),
                DockLocalEvaluationErrorKindV1::Unavailable,
            ),
            (
                "terminal",
                fixed
                    .origin_paths
                    .iter()
                    .find(|path| path.ends_with("/terminal.json"))
                    .unwrap(),
                DockLocalEvaluationErrorKindV1::Substituted,
            ),
        ]
        .into_iter()
        .skip(completed_prefix)
        {
            let path = fixture.root.path().join(relative);
            assert!(path.canonicalize().unwrap().starts_with(&original_root));
            let original = std::fs::read(&path).unwrap();
            std::fs::write(&path, b"{}").unwrap();
            let outcome = fixture.sdk.close_local_evaluation_result(&session);
            std::fs::write(&path, &original).unwrap();
            let error = outcome.expect_err(&format!("substituted native {name} published truth"));
            let LocalEvaluationSdkErrorV1::Dock(error) = error else {
                panic!("native {name} did not reach the source integrity boundary: {error:?}");
            };
            assert_eq!(error.kind(), expected_kind, "native {name} source refusal");
            assert_eq!(
                error.detail(),
                match expected_kind {
                    DockLocalEvaluationErrorKindV1::Unavailable => {
                        "exact native Dock dependency is unavailable"
                    }
                    DockLocalEvaluationErrorKindV1::Substituted => {
                        "exact native Dock dependency was substituted"
                    }
                    _ => unreachable!("only exact source-attributed refusals are expected"),
                },
                "native {name} refusal must come from original source resolution",
            );
            let recovered = fixture
                .sdk
                .recover_local_evaluation(&formed.snapshot.locator)
                .unwrap();
            assert_eq!(recovered, fixed);
            assert!(recovered.result.is_none());
            assert_eq!(retained_bytes(&fixture, &fixed), fixed_bytes);
            eprintln!("final native source: {name} substitution refused without truth");
        }

        let closed = fixture.sdk.close_local_evaluation_result(&session).unwrap();
        assert_eq!(closed.stage, LocalEvaluationStageV3::ResultClosed);
        let result = closed.result.unwrap();
        assert_eq!(result.claim.finding, "satisfied");
        assert_eq!(result.claim.coverage, "sufficient");
        assert_eq!(result.claim.consistency, "consistent");
        // Time advances during the genuine process checks; the original
        // criterion stays fixed and determines the honest closed gate.
        match result.claim.currentness.as_str() {
            "current" => assert_eq!(result.gate, LocalEvaluationGateV3::Pass),
            "stale" => assert_eq!(result.gate, LocalEvaluationGateV3::Indeterminate),
            other => panic!("unexpected currentness after exact source closure: {other}"),
        }
        let history = fixture
            .sdk
            .read_local_evaluation_history(&formed.snapshot.locator)
            .unwrap();
        assert_eq!(history.result, result);
        assert!(history.exact_replay_available);
        assert_eq!(
            std::fs::read_dir(
                fixture
                    .root
                    .path()
                    .join(".handbook/state/dock/v1/executions")
            )
            .unwrap()
            .count(),
            1
        );
    });
}

/// Runs against an original physical fixture retained after a failed delivery.
/// The supplied handoff only identifies that fixture; no owner bytes are seeded.
#[test]
#[ignore = "requires the original retained native fixture and anchor snapshot"]
fn native_original_repository_recovers_pending_transaction_via_public_sdk() {
    on_stack(|| {
        let handoff_path = std::env::var("PACKET3_ORIGINAL_NATIVE_HANDOFF").unwrap();
        let formed_path = std::env::var("PACKET3_ORIGINAL_NATIVE_FORMED").unwrap();
        let handoff: Value = serde_json::from_slice(&std::fs::read(handoff_path).unwrap()).unwrap();
        let formed: Value = serde_json::from_slice(&std::fs::read(formed_path).unwrap()).unwrap();
        let root = std::path::PathBuf::from(handoff["repository_path"].as_str().unwrap());
        let original: LocalEvaluationSnapshotV3 =
            serde_json::from_value(formed["snapshot"].clone()).unwrap();
        let record_path = root.join(format!(
            "{LOCAL_EVALUATION_OWNER_ROOT_V3}/records/{}.json",
            original
                .identity
                .fingerprint()
                .as_str()
                .trim_start_matches("sha256:")
        ));
        let original_bytes = std::fs::read(&record_path).unwrap();
        let transactions = root.join(".handbook/state/contracts/v1/transactions");
        let pending: Vec<_> = std::fs::read_dir(&transactions)
            .unwrap()
            .map(|entry| entry.unwrap().path())
            .filter(|path| {
                path.extension()
                    .is_some_and(|extension| extension == "pending")
            })
            .collect();
        assert_eq!(
            pending.len(),
            1,
            "the original interrupted transaction exists"
        );
        assert_eq!(std::fs::read_dir(&pending[0]).unwrap().count(), 0);
        let strict_error = ContractOwnerRepositoryV1::open_existing_for_local_use_v3(&root)
            .err()
            .expect("live use must refuse pending recovery");
        assert_eq!(
            strict_error.kind(),
            ContractOwnerRepositoryErrorKindV1::RecoveryAmbiguous
        );

        let sdk = HandbookSdkV1::open(&root);
        let request = LocalEvaluationRequestLocatorV3 {
            request_key: original.locator.request_key.clone(),
            request_fingerprint: original.locator.request_fingerprint.clone(),
        };
        assert_eq!(
            sdk.recover_local_evaluation_request(&request).unwrap(),
            Some(original.clone())
        );
        assert_eq!(
            sdk.recover_local_evaluation(&original.locator).unwrap(),
            original
        );
        assert_eq!(std::fs::read(&record_path).unwrap(), original_bytes);
        assert!(
            !pending[0].exists(),
            "the retained early interruption was resolved"
        );

        let mut conflicting = request;
        conflicting.request_fingerprint = FingerprintV1::parse(
            "sha256:0000000000000000000000000000000000000000000000000000000000000000",
        )
        .unwrap();
        match sdk
            .recover_local_evaluation_request(&conflicting)
            .unwrap_err()
        {
            LocalEvaluationSdkErrorV1::Contracts(error) => assert_eq!(
                error.kind(),
                ContractOwnerRepositoryErrorKindV1::IdempotencyConflict,
            ),
            error => {
                panic!("original request conflict must be attributable to Contracts: {error:?}")
            }
        }
        assert_eq!(std::fs::read(record_path).unwrap(), original_bytes);
    });
}

/// Continues only the original immutable result retained after the fault matrix's
/// demonstrated stale-Pass assertion failure. No fault or original effect repeats.
#[test]
#[ignore = "requires the original public handoff and immutable stale result record"]
fn native_original_stale_result_continues_history_and_new_cancelled_occurrence_without_replay() {
    on_stack(|| {
        let _package = InstalledProductionPackage::install();
        let handoff =
            std::path::PathBuf::from(std::env::var("PACKET3_ORIGINAL_OWNER_HANDOFF").unwrap());
        let record =
            std::path::PathBuf::from(std::env::var("PACKET3_ORIGINAL_OWNER_RECORD").unwrap());
        let fixture = Fixture::from_public_handoff(&handoff);
        let original_bytes = std::fs::read(&record).unwrap();
        let nominated: LocalEvaluationSnapshotV3 = serde_json::from_slice(&original_bytes).unwrap();
        assert_eq!(
            record.canonicalize().unwrap(),
            fixture
                .root
                .path()
                .join(format!(
                    "{LOCAL_EVALUATION_OWNER_ROOT_V3}/records/{}.json",
                    nominated
                        .identity
                        .fingerprint()
                        .as_str()
                        .trim_start_matches("sha256:")
                ))
                .canonicalize()
                .unwrap(),
            "the locator comes from this original immutable owner record",
        );
        let original = fixture
            .sdk
            .recover_local_evaluation(&nominated.locator)
            .unwrap();
        assert_eq!(original, nominated);
        assert_eq!(
            serde_json_canonicalizer::to_vec(&original).unwrap(),
            original_bytes
        );
        assert_eq!(original.stage, LocalEvaluationStageV3::ResultClosed);
        let result = original.result.as_ref().unwrap();
        assert_eq!(result.gate, LocalEvaluationGateV3::Indeterminate);
        assert_eq!(
            result.claim.applicability,
            ApplicabilityOutcomeV1::Applicable
        );
        assert_eq!(result.claim.finding, "satisfied");
        assert_eq!(result.claim.coverage, "sufficient");
        assert_eq!(result.claim.currentness, "stale");
        assert_eq!(result.claim.consistency, "consistent");
        assert_eq!(result.claim.uncertainty, ["stale_or_unresolved_assessment"]);
        assert!(result.claim.violations.is_empty());
        assert!(result.metrics.is_empty());
        assert_eq!(result.evidence_set.members.len(), 1);
        assert_eq!(result.evidence_set.admission_terminals.len(), 2);

        // Existing time primitives check actual elapsed UTC seconds. This does
        // not retime the captured candidate, result, or selected freshness rule.
        let utc_seconds = |value: &SemanticTimeV1| {
            let value = value.as_str();
            time::Date::from_calendar_date(
                value[..4].parse().unwrap(),
                time::Month::try_from(value[5..7].parse::<u8>().unwrap()).unwrap(),
                value[8..10].parse().unwrap(),
            )
            .unwrap()
            .with_hms(
                value[11..13].parse().unwrap(),
                value[14..16].parse().unwrap(),
                value[17..19].parse().unwrap(),
            )
            .unwrap()
            .assume_utc()
            .unix_timestamp()
        };
        let claim = serde_json::to_value(&original.requirements.claims[0]).unwrap();
        let freshness: ExactBindingV1 =
            serde_json::from_value(claim["freshness_policy"].clone()).unwrap();
        let policy = original
            .definition
            .dependencies()
            .freshness_policies
            .iter()
            .find(|policy| policy.exact_binding() == freshness)
            .unwrap();
        assert_eq!(policy.max_observation_age_seconds(), 3600);
        let candidate = serde_json::to_value(&original.evidence[0].candidate).unwrap();
        let observed =
            SemanticTimeV1::parse(candidate["body"]["observed_at"].as_str().unwrap()).unwrap();
        let age = utc_seconds(&result.evaluated_at) - utc_seconds(&observed);
        assert!(
            age > 3600,
            "the original result must prove actual stale evidence"
        );
        eprintln!(
            "original result preserved: {:?}; evaluated={}, original age={}s, rule=3600s",
            result.gate,
            result.evaluated_at.as_str(),
            age,
        );
        let summary = result.human_summary();
        for required in [
            "Indeterminate: satisfied.",
            "coverage: sufficient",
            "currentness: stale",
            "consistency: consistent",
            "stale_or_unresolved_assessment",
            "permitted use: decision_seeking_eligible",
            "deferred_v2",
            "deferred_intrinsic",
            "Violations: []",
            "1 included member(s)",
            "2 admission terminal(s)",
            "1 admitted, 1 duplicate, 0 refused, 0 nonmember(s)",
            "Shipment or completion obligation: not assessed",
            result.evidence_set.identity.fingerprint().as_str(),
        ] {
            assert!(
                summary.contains(required),
                "missing original meaning: {required}"
            );
        }
        for private in [
            fixture.root.path().to_str().unwrap(),
            "retained_root",
            "manifest_path",
            "package_files",
            "owner_write_paths",
            "input_paths",
            "private_envelope",
        ] {
            assert!(
                !summary.contains(private),
                "private proof in summary: {private}"
            );
        }
        let exact_result = result.canonical_json_bytes().unwrap();
        let exact_locator = original.result_locator().unwrap();
        let history = fixture
            .sdk
            .read_local_evaluation_result_history(&exact_locator)
            .unwrap();
        assert!(history.exact_replay_available);
        assert_eq!(history.result.canonical_json_bytes().unwrap(), exact_result);
        assert_eq!(std::fs::read(&record).unwrap(), original_bytes);

        let subsequent = fixture.form(
            "packet3-owner-continued-cancelled-empty-0001",
            supplied_validation_content(),
            production_manifest().resolution_ceiling().clone(),
            LocalEvaluationUseIntentV1::DecisionSeeking,
        );
        assert_eq!(subsequent.snapshot.ordinal, original.ordinal + 1);
        assert_ne!(
            subsequent.snapshot.locator.occurrence,
            original.locator.occurrence
        );
        assert_eq!(
            subsequent.snapshot.purpose.content_fingerprint,
            original.purpose.content_fingerprint
        );
        fixture.assess(&subsequent);
        let session = fixture.session(&subsequent);
        fixture
            .sdk
            .commit_local_evaluation_collection(&session)
            .unwrap();
        let cancellation = DockCancellationV1::new();
        assert!(cancellation.cancel());
        let terminal = match fixture
            .sdk
            .execute_local_evaluation_once(
                &session,
                subsequent.process_request.as_ref().unwrap(),
                &dock_reach(),
                &cancellation,
            )
            .unwrap()
        {
            DockLocalEvaluationAttemptOutcomeV1::Terminal { terminal, .. } => terminal,
            other => panic!("{other:?}"),
        };
        assert_eq!(terminal.kind, "cancelled_before_launch");
        assert!(terminal.candidates.is_empty());
        let empty_result = finish(&fixture, &subsequent);
        assert_eq!(empty_result.gate, LocalEvaluationGateV3::Indeterminate);
        assert!(empty_result.evidence_set.members.is_empty());
        assert_eq!(
            empty_result.explanation["origin_terminal"]["reason"],
            terminal.reason
        );
        assert_eq!(
            empty_result.explanation["origin_terminal"]["kind"],
            "cancelled_before_launch"
        );
        let summary = empty_result.human_summary();
        for required in [
            terminal.reason.as_str(),
            "cancelled_before_launch",
            "required_observation_absent",
            "0 included member(s)",
        ] {
            assert!(
                summary.contains(required),
                "missing empty population reason: {required}"
            );
        }
        let empty_bytes = empty_result.canonical_json_bytes().unwrap();
        assert_eq!(
            fixture
                .sdk
                .read_local_evaluation_history(&subsequent.snapshot.locator)
                .unwrap()
                .result
                .canonical_json_bytes()
                .unwrap(),
            empty_bytes
        );
        assert_eq!(
            fixture
                .sdk
                .recover_local_evaluation(&original.locator)
                .unwrap(),
            original
        );
        assert_eq!(
            fixture
                .sdk
                .read_local_evaluation_result_history(&exact_locator)
                .unwrap()
                .result
                .canonical_json_bytes()
                .unwrap(),
            exact_result
        );
        assert_eq!(std::fs::read(record).unwrap(), original_bytes);
    });
}

fn owner(fixture: &Fixture, fault: Option<Fault>) -> ContractOwnerRepositoryV1 {
    let owner =
        ContractOwnerRepositoryV1::open_existing_for_local_use_v3(fixture.root.path()).unwrap();
    match fault {
        Some(fault) => owner.with_fault_for_testing(fault),
        None => owner,
    }
}

fn retained_bytes(fixture: &Fixture, snapshot: &LocalEvaluationSnapshotV3) -> Vec<u8> {
    std::fs::read(fixture.root.path().join(format!(
        "{LOCAL_EVALUATION_OWNER_ROOT_V3}/records/{}.json",
        snapshot.identity.fingerprint().as_str().trim_start_matches("sha256:")
    )))
    .unwrap()
}

fn pending_bytes(fixture: &Fixture) -> Vec<u8> {
    let pending = std::fs::read_dir(
        fixture
            .root
            .path()
            .join(".handbook/state/contracts/v1/transactions"),
    )
    .unwrap()
    .map(|entry| entry.unwrap().path())
    .filter(|path| {
        path.extension()
            .is_some_and(|extension| extension == "pending")
    })
    .collect::<Vec<_>>();
    assert_eq!(pending.len(), 1, "one interrupted owner transaction");
    std::fs::read(pending[0].join("00-transition.json")).unwrap()
}

fn issue_recovered(fixture: &Fixture, snapshot: LocalEvaluationSnapshotV3) -> Formed {
    let request = snapshot.authorization_request(
        LocalEvaluationEnvelopeV1::local(
            &snapshot.purpose.subject_ref,
            snapshot.plan.selections[0].requested_reach.clone(),
            &fixture.authority.terms,
        ),
        engine_reach(),
    );
    let issuance = fixture
        .sdk
        .local_evaluation_authorization_locator(&fixture.authority, &request)
        .unwrap();
    let issued = fixture
        .sdk
        .issue_local_evaluation_authorization(&fixture.authority, &request);
    assert!(issued.binding.is_some(), "{issued:?}");
    let manifest = production_manifest();
    let role = &manifest.supported_inputs()[0];
    let capture = DockInlineJsonInputV1::new(
        role.role().clone(),
        role.media_type(),
        role.schema().clone(),
        supplied_validation_content(),
    )
    .unwrap();
    Formed {
        formation: snapshot.original_input.formation.clone(),
        process_request: Some(process_request(&snapshot, capture)),
        snapshot,
        request,
        issuance,
    }
}

#[derive(Clone, Copy)]
enum ClosureOperation {
    Commit,
    Frontier,
    Set,
    Result,
}

fn mixed_native_origin_never_mints_a_consumable_fact(
    fixture: &Fixture,
    formed: &Formed,
    snapshot: &LocalEvaluationSnapshotV3,
    other_occurrence: &ContentAddressedBindingV1,
) {
    let dock = DockLocalEvaluationOwnerV1::open_existing(fixture.root.path()).unwrap();
    let exact = DockLocalEvaluationCoordinatesV1 {
        occurrence: snapshot.occurrence().clone(),
        basis: snapshot.basis().unwrap().clone(),
        selection: snapshot.selection().clone(),
        authorization: snapshot.authorization_binding().unwrap(),
        capability_observation: snapshot
            .readiness_bases
            .last()
            .unwrap()
            .capability
            .binding()
            .unwrap(),
    };
    for coordinate in ["occurrence", "basis", "selection", "capability"] {
        let mut mixed = exact.clone();
        match coordinate {
            "occurrence" => mixed.occurrence = other_occurrence.clone(),
            "basis" => mixed.basis = other_occurrence.clone(),
            "selection" => {
                mixed.selection = ExactBindingV1::new(
                    snapshot.selection().reference().clone(),
                    FingerprintV1::parse(&format!("sha256:{}", "b".repeat(64))).unwrap(),
                )
            }
            _ => mixed.capability_observation = other_occurrence.clone(),
        }
        let consumed = std::cell::Cell::new(false);
        assert!(
            dock.with_origin(
                snapshot.attempt.as_ref().unwrap(),
                &mixed,
                &snapshot.run,
                snapshot.definition.dependencies(),
                &dock_reach(),
                |_| consumed.set(true),
            )
            .is_err(),
            "mixed {coordinate}"
        );
        assert!(
            !consumed.get(),
            "untrusted {coordinate} must not mint native fact"
        );
        assert_eq!(
            fixture
                .sdk
                .recover_local_evaluation(&formed.snapshot.locator)
                .unwrap(),
            *snapshot
        );
    }
}

fn operation(
    fixture: &Fixture,
    formed: &Formed,
    operation: ClosureOperation,
    fault: Option<Fault>,
) -> Result<LocalEvaluationSnapshotV3, ContractOwnerRepositoryErrorV1> {
    let owner = owner(fixture, fault);
    let current = fixture
        .sdk
        .observe_local_check_basis(&fixture.initial)
        .unwrap();
    if matches!(operation, ClosureOperation::Result) {
        let source = owner
            .local_evaluation_final_set_source_v3(&formed.snapshot.locator)
            .unwrap();
        let snapshot: LocalEvaluationSnapshotV3 =
            serde_json::from_slice(&source.canonical_record_bytes).unwrap();
        let coordinates = DockLocalEvaluationCoordinatesV1 {
            occurrence: snapshot.occurrence().clone(),
            basis: snapshot.basis().unwrap().clone(),
            selection: snapshot.selection().clone(),
            authorization: snapshot.authorization_binding().unwrap(),
            capability_observation: snapshot
                .readiness_bases
                .last()
                .unwrap()
                .capability
                .binding()
                .unwrap(),
        };
        return NativeLocalEvaluationAuthorityV1::with_final_verification(
            fixture.root.path(),
            &fixture.authority,
            &formed.issuance,
            &formed.request,
            &source,
            &current,
            &formed.request.reach,
            |admission| {
                let dock = DockLocalEvaluationOwnerV1::open_existing(fixture.root.path()).unwrap();
                Ok(dock
                    .with_origin(
                        snapshot.attempt.as_ref().unwrap(),
                        &coordinates,
                        &snapshot.run,
                        snapshot.definition.dependencies(),
                        &dock_reach(),
                        |origin| {
                            owner.close_local_evaluation_result_v3(
                                &formed.snapshot.locator,
                                admission,
                                origin,
                            )
                        },
                    )
                    .unwrap())
            },
        )
        .unwrap();
    }
    let snapshot = fixture
        .sdk
        .recover_local_evaluation(&formed.snapshot.locator)
        .unwrap();
    let target = match operation {
        ClosureOperation::Commit => owner
            .prospective_local_evaluation_basis_v3(&snapshot.locator)
            .unwrap(),
        _ => snapshot.committed_basis.clone().unwrap(),
    };
    let checkpoint = match operation {
        ClosureOperation::Commit => LocalEvaluationCheckpointV1::CollectionCommit,
        _ => LocalEvaluationCheckpointV1::SourceAdmission,
    };
    NativeLocalEvaluationAuthorityV1::with_verification(
        fixture.root.path(),
        &fixture.authority,
        &formed.issuance,
        &formed.request,
        checkpoint,
        Some(target.fingerprint().as_str()),
        &current,
        &formed.request.reach,
        |admission| {
            Ok(match operation {
                ClosureOperation::Commit => {
                    owner.commit_local_evaluation_v3(&snapshot.locator, admission)
                }
                ClosureOperation::Frontier => {
                    owner.seal_local_evaluation_frontier_v3(&snapshot.locator, admission)
                }
                ClosureOperation::Set => {
                    owner.close_local_evaluation_set_v3(&snapshot.locator, admission)
                }
                ClosureOperation::Result => unreachable!(),
            })
        },
    )
    .unwrap()
}

fn interrupted_transition(
    fixture: &Fixture,
    formed: &Formed,
    operation_kind: ClosureOperation,
    fault: Fault,
    commits: bool,
) -> LocalEvaluationSnapshotV3 {
    let before = fixture
        .sdk
        .recover_local_evaluation(&formed.snapshot.locator)
        .unwrap();
    let before_bytes = retained_bytes(fixture, &before);
    eprintln!(
        "owner fault: {:?} from {:?}, commits={commits}",
        fault, before.stage
    );
    let error = operation(fixture, formed, operation_kind, Some(fault)).unwrap_err();
    assert_eq!(
        error.kind(),
        ContractOwnerRepositoryErrorKindV1::DurabilityUncertain
    );
    let staged_bytes = pending_bytes(fixture);
    let recovered = fixture
        .sdk
        .recover_local_evaluation(&formed.snapshot.locator)
        .unwrap();
    assert_eq!(retained_bytes(fixture, &before), before_bytes);
    if commits {
        assert_eq!(retained_bytes(fixture, &recovered), staged_bytes);
        assert_eq!(
            serde_json_canonicalizer::to_vec(&recovered).unwrap(),
            staged_bytes
        );
        assert_eq!(recovered.sequence, before.sequence + 1);
    } else {
        assert_eq!(recovered, before);
        assert_eq!(retained_bytes(fixture, &recovered), before_bytes);
    }
    eprintln!(
        "owner recovery: exact {:?} sequence {}",
        recovered.stage, recovered.sequence
    );
    recovered
}

#[test]
fn native_owner_allocation_and_each_closure_recover_original_bytes_without_reusing_ordinals() {
    on_stack(|| {
        let _package = InstalledProductionPackage::install();
        let fixture = Fixture::new();
        let anchor = fixture.form(
            "packet3-owner-anchor-0001",
            supplied_validation_content(),
            production_manifest().resolution_ceiling().clone(),
            LocalEvaluationUseIntentV1::DecisionSeeking,
        );
        let mut input = anchor.snapshot.original_input.clone();
        input.request_key = "packet3-owner-interrupted-allocation-0001".into();
        let prepared = fixture
            .sdk
            .prepare_local_evaluation(&fixture.initial, &fixture.authority, input)
            .unwrap();
        let original_request = prepared.request_locator();
        for fault in [
            Fault::PendingDirectoryDurable,
            Fault::StagedPrimaryDurable,
            Fault::StagedHeadDurable,
            Fault::StagedLedgerDurable,
        ] {
            let owner = owner(&fixture, Some(fault));
            let result = NativeLocalEvaluationAuthorityV1::with_terms_admission(
                fixture.root.path(),
                &fixture.authority,
                prepared.current_basis(),
                prepared.formation_request(),
                |admission| Ok(owner.form_local_evaluation_v3(prepared.clone(), admission)),
            )
            .unwrap();
            assert_eq!(
                result.unwrap_err().kind(),
                ContractOwnerRepositoryErrorKindV1::DurabilityUncertain
            );
            assert_eq!(
                fixture
                    .sdk
                    .recover_local_evaluation_request(&original_request)
                    .unwrap(),
                None
            );
            eprintln!("owner allocation: {fault:?} recovered NotCommitted");
        }
        let owner = owner(&fixture, Some(Fault::IntentDurable));
        let interrupted = NativeLocalEvaluationAuthorityV1::with_terms_admission(
            fixture.root.path(),
            &fixture.authority,
            prepared.current_basis(),
            prepared.formation_request(),
            |admission| Ok(owner.form_local_evaluation_v3(prepared.clone(), admission)),
        )
        .unwrap();
        assert_eq!(
            interrupted.unwrap_err().kind(),
            ContractOwnerRepositoryErrorKindV1::DurabilityUncertain
        );
        let original_bytes = pending_bytes(&fixture);
        let allocated = fixture
            .sdk
            .recover_local_evaluation_request(&original_request)
            .unwrap()
            .unwrap();
        assert_eq!(allocated.ordinal, anchor.snapshot.ordinal + 1);
        eprintln!(
            "owner allocation: original intent recovered ordinal {}",
            allocated.ordinal
        );
        assert_eq!(retained_bytes(&fixture, &allocated), original_bytes);
        assert_eq!(
            fixture
                .sdk
                .form_local_evaluation(&fixture.authority, prepared.clone())
                .unwrap(),
            allocated
        );
        let mut conflict = original_request.clone();
        conflict.request_fingerprint =
            FingerprintV1::parse(&format!("sha256:{}", "9".repeat(64))).unwrap();
        assert!(fixture
            .sdk
            .recover_local_evaluation_request(&conflict)
            .is_err());
        let formed = issue_recovered(&fixture, allocated);
        fixture.assess(&formed);
        let committed = interrupted_transition(
            &fixture,
            &formed,
            ClosureOperation::Commit,
            Fault::IntentDurable,
            false,
        );
        assert_eq!(committed.stage, LocalEvaluationStageV3::Attemptable);
        let committed = interrupted_transition(
            &fixture,
            &formed,
            ClosureOperation::Commit,
            Fault::MarkerDurable,
            true,
        );
        assert_eq!(committed.stage, LocalEvaluationStageV3::Committed);
        let session = fixture.session(&formed);
        fixture
            .sdk
            .execute_local_evaluation_once(
                &session,
                formed.process_request.as_ref().unwrap(),
                &dock_reach(),
                &DockCancellationV1::new(),
            )
            .unwrap();
        let original_attempt = fixture
            .sdk
            .recover_local_evaluation(&formed.snapshot.locator)
            .unwrap();
        mixed_native_origin_never_mints_a_consumable_fact(
            &fixture,
            &formed,
            &original_attempt,
            &anchor.snapshot.locator.occurrence,
        );
        let second = ContentAddressedBindingV1::new(
            "handbook.dock.local-evaluation-attempt@1.0.0#second",
            FingerprintV1::parse(&format!("sha256:{}", "a".repeat(64))).unwrap(),
        )
        .unwrap();
        let current = fixture
            .sdk
            .observe_local_check_basis(&fixture.initial)
            .unwrap();
        let one_attempt_owner = self::owner(&fixture, None);
        let refused = NativeLocalEvaluationAuthorityV1::with_verification(
            fixture.root.path(),
            &fixture.authority,
            &formed.issuance,
            &formed.request,
            LocalEvaluationCheckpointV1::Launch,
            Some(original_attempt.basis().unwrap().fingerprint().as_str()),
            &current,
            &formed.request.reach,
            |admission| {
                Ok(one_attempt_owner.bind_local_evaluation_attempt_v3(
                    &formed.snapshot.locator,
                    admission,
                    second,
                ))
            },
        )
        .unwrap();
        assert_eq!(
            refused.unwrap_err().kind(),
            ContractOwnerRepositoryErrorKindV1::InvalidRequest
        );
        assert_eq!(
            fixture
                .sdk
                .recover_local_evaluation(&formed.snapshot.locator)
                .unwrap(),
            original_attempt
        );
        let origin = fixture
            .sdk
            .admit_local_evaluation_origin(&session, &dock_reach(), "packet3-owner-admission-0001")
            .unwrap();
        assert_eq!(origin.evidence.len(), 1);
        let duplicate = fixture
            .sdk
            .admit_local_evaluation_origin(&session, &dock_reach(), "packet3-owner-duplicate-0001")
            .unwrap();
        assert_eq!(duplicate.evidence.len(), 1);
        assert_eq!(duplicate.admission_terminals.len(), 2);
        assert_eq!(duplicate.admission_terminals[1].disposition, "duplicate");
        assert_eq!(
            fixture
                .sdk
                .admit_local_evaluation_origin(
                    &session,
                    &dock_reach(),
                    "packet3-owner-duplicate-0001"
                )
                .unwrap(),
            duplicate
        );
        for (kind, expected_stage) in [
            (
                ClosureOperation::Frontier,
                LocalEvaluationStageV3::FrontierSealed,
            ),
            (ClosureOperation::Set, LocalEvaluationStageV3::SetClosed),
            (
                ClosureOperation::Result,
                LocalEvaluationStageV3::ResultClosed,
            ),
        ] {
            interrupted_transition(&fixture, &formed, kind, Fault::IntentDurable, false);
            let recovered =
                interrupted_transition(&fixture, &formed, kind, Fault::MarkerDurable, true);
            assert_eq!(recovered.stage, expected_stage);
        }
        let closed = fixture
            .sdk
            .recover_local_evaluation(&formed.snapshot.locator)
            .unwrap();
        let result_locator = closed.result_locator().unwrap();
        let result_bytes =
            serde_json_canonicalizer::to_vec(closed.result.as_ref().unwrap()).unwrap();
        assert_eq!(
            closed.result.as_ref().unwrap().evidence_set.members.len(),
            1
        );
        let result = closed.result.as_ref().unwrap();
        assert_eq!(result.claim.finding, "satisfied");
        assert_eq!(result.claim.coverage, "sufficient");
        assert_eq!(result.claim.consistency, "consistent");
        assert!(result.claim.violations.is_empty());
        let utc_seconds = |value: &SemanticTimeV1| {
            let value = value.as_str();
            time::Date::from_calendar_date(
                value[..4].parse().unwrap(),
                time::Month::try_from(value[5..7].parse::<u8>().unwrap()).unwrap(),
                value[8..10].parse().unwrap(),
            )
            .unwrap()
            .with_hms(
                value[11..13].parse().unwrap(),
                value[14..16].parse().unwrap(),
                value[17..19].parse().unwrap(),
            )
            .unwrap()
            .assume_utc()
            .unix_timestamp()
        };
        let claim = serde_json::to_value(&closed.requirements.claims[0]).unwrap();
        let freshness: ExactBindingV1 =
            serde_json::from_value(claim["freshness_policy"].clone()).unwrap();
        let max_age = closed
            .definition
            .dependencies()
            .freshness_policies
            .iter()
            .find(|policy| policy.exact_binding() == freshness)
            .unwrap()
            .max_observation_age_seconds();
        assert_eq!(max_age, 3600);
        let candidate = serde_json::to_value(&closed.evidence[0].candidate).unwrap();
        let observed =
            SemanticTimeV1::parse(candidate["body"]["observed_at"].as_str().unwrap()).unwrap();
        let age = utc_seconds(&result.evaluated_at) - utc_seconds(&observed);
        assert!(age >= 0, "an original observation cannot be in the future");
        let expected_gate = if age <= i64::try_from(max_age).unwrap() {
            assert_eq!(result.claim.currentness, "current");
            assert!(result.claim.uncertainty.is_empty());
            LocalEvaluationGateV3::Pass
        } else {
            assert_eq!(result.claim.currentness, "stale");
            assert_eq!(result.claim.uncertainty, ["stale_or_unresolved_assessment"]);
            LocalEvaluationGateV3::Indeterminate
        };
        assert_eq!(result.gate, expected_gate);
        let summary = result.human_summary();
        let gate_meaning = format!("{expected_gate:?}: satisfied.");
        let currentness_meaning = format!("currentness: {}", result.claim.currentness);
        let uncertainty_meaning = format!(
            "uncertainty: {}",
            serde_json::to_string(&result.claim.uncertainty).unwrap(),
        );
        for required in [
            gate_meaning.as_str(),
            "coverage: sufficient",
            currentness_meaning.as_str(),
            "consistency: consistent",
            uncertainty_meaning.as_str(),
            "permitted use: decision_seeking_eligible",
            "deferred_v2",
            "deferred_intrinsic",
            "Violations: []",
            "1 included member(s)",
            "2 admission terminal(s)",
            "1 admitted, 1 duplicate, 0 refused, 0 nonmember(s)",
            "Shipment or completion obligation: not assessed",
            closed
                .result
                .as_ref()
                .unwrap()
                .evidence_set
                .identity
                .fingerprint()
                .as_str(),
        ] {
            assert!(
                summary.contains(required),
                "missing retained meaning: {required}"
            );
        }
        for private_proof in [
            fixture.root.path().to_str().unwrap(),
            "retained_root",
            "manifest_path",
            "package_files",
            "owner_write_paths",
            "input_paths",
            "private_envelope",
        ] {
            assert!(
                !summary.contains(private_proof),
                "human summary exposes filesystem or private proof: {private_proof}"
            );
        }
        assert_eq!(
            serde_json_canonicalizer::to_vec(closed.result.as_ref().unwrap()).unwrap(),
            result_bytes
        );
        assert!(fixture
            .sdk
            .admit_local_evaluation_origin(&session, &dock_reach(), "packet3-owner-late-new-0001")
            .is_err());
        let history = fixture
            .sdk
            .read_local_evaluation_result_history(&result_locator)
            .unwrap();
        assert_eq!(
            serde_json_canonicalizer::to_vec(&history.result).unwrap(),
            result_bytes
        );
        let subsequent = fixture.form(
            "packet3-owner-subsequent-allocation-0001",
            supplied_validation_content(),
            production_manifest().resolution_ceiling().clone(),
            LocalEvaluationUseIntentV1::DecisionSeeking,
        );
        assert_eq!(subsequent.snapshot.ordinal, formed.snapshot.ordinal + 1);
        assert_ne!(
            subsequent.snapshot.locator.occurrence,
            formed.snapshot.locator.occurrence
        );
        assert_eq!(
            subsequent.snapshot.purpose.content_fingerprint,
            formed.snapshot.purpose.content_fingerprint
        );
        assert_eq!(
            fixture
                .sdk
                .recover_local_evaluation(&formed.snapshot.locator)
                .unwrap(),
            closed
        );
        fixture.assess(&subsequent);
        let empty_session = fixture.session(&subsequent);
        fixture
            .sdk
            .commit_local_evaluation_collection(&empty_session)
            .unwrap();
        let cancelled = DockCancellationV1::new();
        assert!(cancelled.cancel());
        let terminal = match fixture
            .sdk
            .execute_local_evaluation_once(
                &empty_session,
                subsequent.process_request.as_ref().unwrap(),
                &dock_reach(),
                &cancelled,
            )
            .unwrap()
        {
            DockLocalEvaluationAttemptOutcomeV1::Terminal { terminal, .. } => terminal,
            other => panic!("{other:?}"),
        };
        assert_eq!(terminal.kind, "cancelled_before_launch");
        assert!(terminal.candidates.is_empty());
        let empty_result = finish(&fixture, &subsequent);
        assert_eq!(empty_result.gate, LocalEvaluationGateV3::Indeterminate);
        assert!(empty_result.evidence_set.members.is_empty());
        assert_eq!(
            empty_result.explanation["origin_terminal"]["reason"],
            terminal.reason
        );
        assert_eq!(
            empty_result.explanation["origin_terminal"]["kind"],
            "cancelled_before_launch"
        );
        let summary = empty_result.human_summary();
        assert!(summary.contains(&terminal.reason));
        assert!(summary.contains("cancelled_before_launch"));
        assert!(summary.contains("required_observation_absent"));
        assert!(summary.contains("0 included member(s)"));
        let exact = empty_result.canonical_json_bytes().unwrap();
        assert_eq!(
            fixture
                .sdk
                .read_local_evaluation_history(&subsequent.snapshot.locator)
                .unwrap()
                .result
                .canonical_json_bytes()
                .unwrap(),
            exact
        );
    });
}

/// Continues the genuine cancellation already retained after the final-wrapper
/// refusal. It never forms another occurrence or launches a producer.
#[test]
#[ignore = "requires the original cancellation public handoff"]
fn native_original_cancelled_set_closes_and_uncertain_terminal_never_retries() {
    use sha2::{Digest, Sha256};
    on_stack(|| {
        let handoff =
            std::path::PathBuf::from(std::env::var("PACKET3_ORIGINAL_OWNER_HANDOFF").unwrap());
        let fixture = Fixture::from_public_handoff(&handoff);
        let formed: Formed = serde_json::from_slice(
            &std::fs::read(
                fixture
                    .checkpoint_artifacts
                    .path()
                    .join("formed-packet3-owner-continued-cancelled-empty-0001.json"),
            )
            .unwrap(),
        )
        .unwrap();
        let original_path = fixture.root.path().join(format!(
            "{LOCAL_EVALUATION_OWNER_ROOT_V3}/records/36bf486b80067cee97794025a95b81346e7b27ed35880d5750eb3a89cae65832.json",
        ));
        let original_bytes = std::fs::read(&original_path).unwrap();
        let nominated: LocalEvaluationSnapshotV3 = serde_json::from_slice(&original_bytes).unwrap();
        let original = fixture
            .sdk
            .recover_local_evaluation(&nominated.locator)
            .unwrap();
        assert_eq!(original, nominated);
        assert_eq!(original.ordinal, 2);
        let original_result = original.result.as_ref().unwrap();
        assert_eq!(original_result.gate, LocalEvaluationGateV3::Indeterminate);
        assert_eq!(original_result.claim.currentness, "stale");
        let original_result_bytes = original_result.canonical_json_bytes().unwrap();
        let original_result_locator = original.result_locator().unwrap();
        let fixed = fixture
            .sdk
            .recover_local_evaluation(&formed.snapshot.locator)
            .unwrap();
        assert!(matches!(
            fixed.stage,
            LocalEvaluationStageV3::SetClosed | LocalEvaluationStageV3::ResultClosed
        ));
        assert_eq!(fixed.ordinal, original.ordinal + 1);
        assert_eq!(
            fixed.purpose.content_fingerprint,
            original.purpose.content_fingerprint
        );
        assert_ne!(fixed.occurrence(), original.occurrence());
        assert_ne!(
            fixed.run.evaluation_run_fingerprint(),
            original.run.evaluation_run_fingerprint(),
        );
        let attempt = fixed.attempt.as_ref().unwrap().clone();
        let terminal_relative = fixed
            .origin_paths
            .iter()
            .find(|path| path.ends_with("/terminal.json"))
            .unwrap()
            .clone();
        let terminal_path = fixture.root.path().join(&terminal_relative);
        assert!(terminal_path
            .canonicalize()
            .unwrap()
            .starts_with(fixture.root.path().canonicalize().unwrap()));
        let terminal_bytes = std::fs::read(&terminal_path).unwrap();
        let terminal: DockLocalEvaluationAttemptTerminalV1 =
            serde_json::from_slice(&terminal_bytes).unwrap();
        assert_eq!(terminal.attempt, attempt);
        assert_eq!(terminal.kind, "cancelled_before_launch");
        assert!(terminal.execution.is_none() && terminal.store_root.is_none());
        assert!(terminal.candidates.is_empty());
        assert_eq!(
            serde_json::to_value(&terminal).unwrap(),
            fixed.origin_terminal.clone().unwrap()
        );
        let intent_path = terminal_path.parent().unwrap().join("intent.json");
        let intent_bytes = std::fs::read(&intent_path).unwrap();
        let intent: DockLocalEvaluationAttemptIntentV1 =
            serde_json::from_slice(&intent_bytes).unwrap();
        assert_eq!(intent.binding().unwrap(), attempt);
        assert_eq!(
            serde_json::to_value(&intent).unwrap(),
            fixed.origin_intent.clone().unwrap()
        );

        let session = fixture.session(&formed);
        let closed = fixture.sdk.close_local_evaluation_result(&session).unwrap();
        let result = closed.result.as_ref().unwrap();
        assert_eq!(closed.stage, LocalEvaluationStageV3::ResultClosed);
        assert_eq!(result.gate, LocalEvaluationGateV3::Indeterminate);
        assert!(result.evidence_set.members.is_empty() && result.metrics.is_empty());
        assert_eq!(
            result.explanation["origin_terminal"],
            serde_json::to_value(&terminal).unwrap()
        );
        for reason in [
            terminal.reason.as_str(),
            "cancelled_before_launch",
            "required_observation_absent",
            "0 included member(s)",
        ] {
            assert!(
                result.human_summary().contains(reason),
                "missing empty reason: {reason}"
            );
        }
        let result_bytes = result.canonical_json_bytes().unwrap();
        let result_locator = closed.result_locator().unwrap();
        let closed_bytes = retained_bytes(&fixture, &closed);
        let replay = fixture
            .sdk
            .admit_local_evaluation_origin(&session, &dock_reach(), "native-origin-admission")
            .unwrap();
        assert_eq!(
            replay, closed,
            "late empty origin replay cannot reopen the set"
        );
        assert_eq!(retained_bytes(&fixture, &closed), closed_bytes);
        let history = fixture
            .sdk
            .read_local_evaluation_result_history(&result_locator)
            .unwrap();
        assert!(history.exact_replay_available);
        assert_eq!(history.result.canonical_json_bytes().unwrap(), result_bytes);

        // Capture native owner inventory, rather than infer no effect from a DTO.
        let inventory = || {
            let base = fixture.root.path().join(".handbook/state/dock");
            let mut directories = vec![base.clone()];
            let mut files = vec![];
            while let Some(directory) = directories.pop() {
                for entry in std::fs::read_dir(directory).unwrap() {
                    let entry = entry.unwrap();
                    let kind = entry.file_type().unwrap();
                    assert!(!kind.is_symlink());
                    if kind.is_dir() {
                        directories.push(entry.path());
                    } else {
                        assert!(kind.is_file());
                        files.push((
                            entry.path().strip_prefix(&base).unwrap().to_owned(),
                            format!("{:x}", Sha256::digest(std::fs::read(entry.path()).unwrap())),
                        ));
                    }
                }
            }
            files.sort_by(|a, b| a.0.cmp(&b.0));
            files
        };
        let before_inventory = inventory();
        let held = fixture
            .checkpoint_artifacts
            .path()
            .join("original-cancelled-terminal.held");
        assert!(
            !held.exists(),
            "never overwrite a previously retained missing source"
        );
        std::fs::rename(&terminal_path, &held).unwrap();
        let missing = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let missing_inventory = inventory();
            match fixture
                .sdk
                .recover_local_evaluation_attempt(&attempt, &dock_reach())
                .unwrap()
            {
                DockLocalEvaluationAttemptOutcomeV1::EffectUncertain { intent: recovered } => {
                    assert_eq!(recovered, intent)
                }
                other => panic!("missing original terminal must remain uncertain: {other:?}"),
            }
            assert_eq!(inventory(), missing_inventory);
            let unavailable = fixture
                .sdk
                .read_local_evaluation_result_history(&result_locator)
                .unwrap();
            assert!(!unavailable.exact_replay_available);
            assert_eq!(
                unavailable.result.canonical_json_bytes().unwrap(),
                result_bytes
            );
            assert!(unavailable
                .dependencies
                .iter()
                .any(|dependency| dependency.path == terminal_relative
                    && dependency.status == "unavailable"
                    && dependency.reason == "retained_dependency_missing"));
            match fixture
                .sdk
                .execute_local_evaluation_once(
                    &session,
                    formed.process_request.as_ref().unwrap(),
                    &dock_reach(),
                    &DockCancellationV1::new(),
                )
                .unwrap()
            {
                DockLocalEvaluationAttemptOutcomeV1::EffectUncertain { intent: replayed } => {
                    assert_eq!(replayed, intent)
                }
                other => panic!("identical uncertain attempt cannot launch again: {other:?}"),
            }
            assert_eq!(
                inventory(),
                missing_inventory,
                "no native execution or inventory growth"
            );
            assert_eq!(retained_bytes(&fixture, &closed), closed_bytes);
            assert_eq!(std::fs::read(&intent_path).unwrap(), intent_bytes);
        }));
        // Rename back the same original file even when an assertion fails.
        std::fs::rename(&held, &terminal_path).unwrap();
        assert_eq!(std::fs::read(&terminal_path).unwrap(), terminal_bytes);
        assert_eq!(inventory(), before_inventory);
        if let Err(panic) = missing {
            std::panic::resume_unwind(panic);
        }
        let restored = fixture
            .sdk
            .read_local_evaluation_result_history(&result_locator)
            .unwrap();
        assert!(restored.exact_replay_available);
        assert_eq!(
            restored.result.canonical_json_bytes().unwrap(),
            result_bytes
        );
        assert_eq!(
            fixture
                .sdk
                .recover_local_evaluation(&original.locator)
                .unwrap(),
            original
        );
        let original_history = fixture
            .sdk
            .read_local_evaluation_result_history(&original_result_locator)
            .unwrap();
        assert!(original_history.exact_replay_available);
        assert_eq!(
            original_history.result.canonical_json_bytes().unwrap(),
            original_result_bytes
        );
        assert_eq!(std::fs::read(original_path).unwrap(), original_bytes);
    });
}

/// Completes only the qualification assertions beyond the original G formation;
/// neither the positive control nor a refused nomination allocates an occurrence.
#[test]
#[ignore = "requires the original qualification public handoff"]
fn native_original_g_refuses_over_ceiling_without_formation_and_preserves_d_history() {
    use sha2::{Digest, Sha256};
    on_stack(|| {
        use handbook_contracts::project_admission_governance::ProjectAdmissionScopeBasisV3;
        let handoff = std::path::PathBuf::from(
            std::env::var("PACKET3_ORIGINAL_QUALIFICATION_HANDOFF").unwrap(),
        );
        let fixture = Fixture::from_public_handoff(&handoff);
        let load = |name| {
            serde_json::from_slice::<Formed>(
                &std::fs::read(fixture.checkpoint_artifacts.path().join(name)).unwrap(),
            )
            .unwrap()
        };
        let fresh = load("formed-packet3-native-fresh-g-complete-0001.json");
        let prior = load("formed-packet3-native-not-applicable-0001.json");
        let fresh_state = fixture
            .sdk
            .recover_local_evaluation(&fresh.snapshot.locator)
            .unwrap();
        assert_eq!(fresh_state.ordinal, 2);
        assert!(fresh_state.attempt.is_none());
        assert_eq!(fresh_state.purpose, fresh.snapshot.purpose);
        let usable = fixture
            .sdk
            .prepare_local_evaluation_use(
                &fixture.initial,
                &fresh_state.definition.binding(),
                LocalContractUseIntentV2::DecisionSeeking,
            )
            .unwrap();
        let ProjectAdmissionScopeBasisV3::EstablishedFromDeferredV3(g) =
            &usable.applicability().governance_basis
        else {
            panic!("actual original same-project G required")
        };
        assert_eq!(
            g.configuration.bindings[0].max_observation_age_seconds,
            3600
        );
        assert!(usable
            .applicability()
            .dependencies
            .iter()
            .any(|d| d.kind == "evidence_freshness_policy"
                && d.qualification == LocalDependencyQualificationV2::QualifiedFreshness));
        assert!(usable
            .applicability()
            .dependencies
            .iter()
            .filter(|d| d.kind != "evidence_freshness_policy")
            .all(|d| d.qualification
                == LocalDependencyQualificationV2::NotApplicableToFreshnessScope));
        let original_d = fixture
            .sdk
            .recover_local_evaluation(&prior.snapshot.locator)
            .unwrap();
        assert!(matches!(
            original_d.requirements.applicability.governance_basis,
            ProjectAdmissionScopeBasisV3::DeferredV2(_)
        ));
        assert_eq!(
            original_d.result.as_ref().unwrap().gate,
            LocalEvaluationGateV3::NotApplicable
        );
        let original_d_bytes = retained_bytes(&fixture, &original_d);
        let original_result_bytes = original_d
            .result
            .as_ref()
            .unwrap()
            .canonical_json_bytes()
            .unwrap();
        let result_locator = original_d.result_locator().unwrap();
        let fresh_bytes = retained_bytes(&fixture, &fresh_state);
        let allocation_directory = fixture.root.path().join(LOCAL_EVALUATION_OWNER_ROOT_V3);
        let allocation_inventory = || {
            let mut directories = vec![allocation_directory.clone()];
            let mut files = vec![];
            while let Some(directory) = directories.pop() {
                for entry in std::fs::read_dir(directory).unwrap() {
                    let entry = entry.unwrap();
                    let kind = entry.file_type().unwrap();
                    assert!(!kind.is_symlink());
                    if kind.is_dir() {
                        directories.push(entry.path());
                    } else {
                        assert!(kind.is_file());
                        files.push((
                            entry.path(),
                            format!("{:x}", Sha256::digest(std::fs::read(entry.path()).unwrap())),
                        ));
                    }
                }
            }
            files.sort_by(|a, b| a.0.cmp(&b.0));
            files
        };
        let before_files = allocation_inventory();
        let over_binding = ExactBindingV1::new(
            ExactRefV1::parse("handbook.contract.order-capture@4.0.0").unwrap(),
            FingerprintV1::parse(
                "sha256:2d85f02db640b5eab2ae11c2c7d7c441c998b7aadaa47e35c37140b017ec4616",
            )
            .unwrap(),
        );
        let repository =
            ContractOwnerRepositoryV1::open_existing_for_local_use_v3(fixture.root.path()).unwrap();
        let over = repository.read_local_definition_v3(&over_binding).unwrap();
        assert_eq!(
            over.definition().dependencies().freshness_policies[0].max_observation_age_seconds(),
            3601
        );
        let use_error = fixture
            .sdk
            .prepare_local_evaluation_use(
                &fixture.initial,
                &over_binding,
                LocalContractUseIntentV2::DecisionSeeking,
            )
            .unwrap_err();
        let LocalEvaluationSdkErrorV1::Contracts(error) = use_error else {
            panic!("over-ceiling use must refuse at the Contracts owner: {use_error:?}")
        };
        assert_eq!(
            error.kind(),
            ContractOwnerRepositoryErrorKindV1::InvalidRequest
        );
        let mut input = fresh_state.original_input.clone();
        input.request_key = "packet3-over-ceiling-dependent-formation-0001".into();
        input.contract = over_binding.clone();
        input.expected_lifecycle_fingerprint = fixture
            .sdk
            .current_local_evaluation_lifecycle(&over_binding)
            .unwrap()
            .lifecycle_fingerprint()
            .clone();
        let error =
            match fixture
                .sdk
                .prepare_local_evaluation(&fixture.initial, &fixture.authority, input)
            {
                Err(LocalEvaluationSdkErrorV1::Contracts(error)) => error,
                Err(other) => {
                    panic!("over-ceiling Q must refuse at the Contracts owner: {other:?}")
                }
                Ok(_) => panic!("over-ceiling Q was prepared"),
            };
        assert_eq!(
            error.kind(),
            ContractOwnerRepositoryErrorKindV1::InvalidRequest
        );
        assert_eq!(
            allocation_inventory(),
            before_files,
            "no ordinal reservation or formation"
        );
        assert_eq!(retained_bytes(&fixture, &fresh_state), fresh_bytes);
        let history = fixture
            .sdk
            .read_local_evaluation_result_history(&result_locator)
            .unwrap();
        assert!(history.exact_replay_available);
        assert_eq!(history.result.gate, LocalEvaluationGateV3::NotApplicable);
        assert_eq!(
            history.result.canonical_json_bytes().unwrap(),
            original_result_bytes
        );
        assert_eq!(retained_bytes(&fixture, &original_d), original_d_bytes);
    });
}
