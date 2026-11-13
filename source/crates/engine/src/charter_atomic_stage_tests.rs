use super::{
    all_output_stage_fault_pairs_for_testing, atomic_rename_no_replace, build_promotion_intent,
    classify_retained_stage_bytes, complete_marker_temp, complete_old_snapshot, output_stage_name,
    publish_intent_from_named_scratch, publish_marker, publish_output_stage_from_named_scratch,
    publish_output_stage_from_scratch, terminal_destination_allows_rename,
    validate_terminal_payload, CharterAuthorityTransactionServiceV1, CharterPromotionErrorKindV1,
    CharterPromotionFaultPointV1, NamedOutputScratchPublicationV12, OutputPurposeV12,
    OutputStageBoundaryV12, PromotionTerminalKindV12, RecoveryBoundaryV12, RetainedStageStateV12,
    ScopedPromotionFaultV12, TerminalDestinationStateV12, WriterBoundaryV12,
};
use std::collections::BTreeSet;

#[derive(Clone, Copy, Debug)]
enum TerminalFixtureKind {
    Committed,
    RolledBack,
}

#[derive(Clone, Copy, Debug)]
enum TerminalFixtureDestination {
    Absent,
    ExactPreExisting,
    Mismatching,
    Unsafe,
}

fn copy_directory_files(source: &std::path::Path, destination: &std::path::Path) {
    std::fs::create_dir(destination).unwrap();
    for entry in std::fs::read_dir(source).unwrap() {
        let entry = entry.unwrap();
        std::fs::copy(entry.path(), destination.join(entry.file_name())).unwrap();
    }
}

fn fixture() -> (
    tempfile::TempDir,
    CharterAuthorityTransactionServiceV1,
    super::CharterPromotionRequestV1,
    super::PreparedPromotion,
) {
    let repo = tempfile::tempdir().unwrap();
    super::charter_authority_transaction_tests::seed_lineage(repo.path());
    let service = CharterAuthorityTransactionServiceV1::new(repo.path());
    let request = super::charter_authority_transaction_tests::request(repo.path());
    let prepared = service.preflight(&request).unwrap();
    (repo, service, request, prepared)
}

fn purpose_bytes(purpose: OutputPurposeV12, request: &super::CharterPromotionRequestV1) -> &[u8] {
    match purpose {
        OutputPurposeV12::Canonical => &request.canonical_bytes,
        OutputPurposeV12::PromotionRecord => &request.promotion_record_bytes,
        OutputPurposeV12::LifecycleTransition => &request.lifecycle_transition_bytes,
    }
}

#[test]
fn output_fault_matrix_is_the_exact_three_by_twelve_cartesian_product() {
    let pairs = all_output_stage_fault_pairs_for_testing();
    assert_eq!(pairs.len(), 36);

    let unique = pairs.iter().copied().collect::<BTreeSet<_>>();
    assert_eq!(unique.len(), 36, "duplicate purpose/boundary pair");

    for purpose in OutputPurposeV12::ALL {
        for boundary in OutputStageBoundaryV12::ALL {
            assert!(
                unique.contains(&(purpose, boundary)),
                "missing {purpose:?} × {boundary:?}"
            );
        }
    }
}

#[test]
fn every_partial_pending_new_output_is_evidence_preserving_mismatch() {
    let expected = b"complete-authoritative-output";
    for prefix_length in 0..expected.len() {
        assert_eq!(
            classify_retained_stage_bytes(Some(&expected[..prefix_length]), expected),
            RetainedStageStateV12::Mismatch,
            "prefix length {prefix_length} was normalized"
        );
    }
    assert_eq!(
        classify_retained_stage_bytes(None, expected),
        RetainedStageStateV12::Absent
    );
    assert_eq!(
        classify_retained_stage_bytes(Some(expected), expected),
        RetainedStageStateV12::Exact
    );
}

#[test]
fn only_an_absent_terminal_destination_allows_rename() {
    assert!(terminal_destination_allows_rename(
        TerminalDestinationStateV12::Absent
    ));
    for state in [
        TerminalDestinationStateV12::ExactPreExisting,
        TerminalDestinationStateV12::Mismatching,
        TerminalDestinationStateV12::Unsafe,
    ] {
        assert!(!terminal_destination_allows_rename(state));
    }
}

#[test]
fn every_purpose_boundary_fault_leaves_the_authoritative_stage_absent_or_exact() {
    let (repo, _service, request, prepared) = fixture();
    let root = repo.path().join(".handbook/state/transactions/promotions");
    std::fs::create_dir_all(&root).unwrap();
    for (index, (purpose, boundary)) in all_output_stage_fault_pairs_for_testing()
        .into_iter()
        .enumerate()
    {
        let transaction_id = format!("promotion-transaction_matrix_{index}");
        let intent = build_promotion_intent(&prepared, None, &transaction_id).unwrap();
        let pending = root.join(format!("{transaction_id}.pending"));
        std::fs::create_dir(&pending).unwrap();
        let expected = purpose_bytes(purpose, &request);
        let _selected =
            ScopedPromotionFaultV12::select(CharterPromotionFaultPointV1::OutputStage {
                purpose,
                boundary,
                write_prefix: (boundary == OutputStageBoundaryV12::S2ScratchWritten)
                    .then_some(expected.len() / 2),
            });
        let failure = publish_output_stage_from_scratch(
            repo.path(),
            &root,
            &pending,
            purpose,
            expected,
            &intent.record,
        )
        .unwrap_err();
        assert_eq!(failure.kind(), CharterPromotionErrorKindV1::InjectedFault);
        let stage = pending.join(output_stage_name(purpose));
        let observed = std::fs::read(&stage).ok();
        let state = classify_retained_stage_bytes(observed.as_deref(), expected);
        let expected_state = if boundary < OutputStageBoundaryV12::S7StageRenamed {
            RetainedStageStateV12::Absent
        } else {
            RetainedStageStateV12::Exact
        };
        assert_eq!(state, expected_state, "{purpose:?} × {boundary:?}");
    }
}

#[test]
fn scratch_write_faults_cover_every_zero_through_complete_prefix_for_each_purpose() {
    let (repo, _service, request, prepared) = fixture();
    let root = repo.path().join(".handbook/state/transactions/promotions");
    std::fs::create_dir_all(&root).unwrap();
    let mut transaction_index = 0_u64;
    for purpose in OutputPurposeV12::ALL {
        let expected = purpose_bytes(purpose, &request);
        for prefix in 0..=expected.len() {
            transaction_index += 1;
            let transaction_id = format!("promotion-transaction_prefix_{transaction_index}");
            let intent = build_promotion_intent(&prepared, None, &transaction_id).unwrap();
            let pending = root.join(format!("{transaction_id}.pending"));
            std::fs::create_dir(&pending).unwrap();
            let _selected =
                ScopedPromotionFaultV12::select(CharterPromotionFaultPointV1::OutputStage {
                    purpose,
                    boundary: OutputStageBoundaryV12::S2ScratchWritten,
                    write_prefix: Some(prefix),
                });
            let failure = publish_output_stage_from_scratch(
                repo.path(),
                &root,
                &pending,
                purpose,
                expected,
                &intent.record,
            )
            .unwrap_err();
            assert_eq!(failure.kind(), CharterPromotionErrorKindV1::InjectedFault);
            assert!(!pending.join(output_stage_name(purpose)).exists());
        }
    }
}

#[test]
fn intent_and_all_output_scratch_collisions_preserve_existing_evidence() {
    let (repo, _service, request, prepared) = fixture();
    let root = repo.path().join(".handbook/state/transactions/promotions");
    std::fs::create_dir_all(&root).unwrap();
    let intent =
        build_promotion_intent(&prepared, None, "promotion-transaction_collision-fixture").unwrap();

    let intent_staging = root.join(super::INTENT_STAGING_NAME);
    std::fs::create_dir(&intent_staging).unwrap();
    let intent_token = "11111111111111111111111111111111";
    let intent_scratch = intent_staging.join(format!("{intent_token}.intent"));
    std::fs::write(&intent_scratch, b"retained intent collision evidence").unwrap();
    let intent_pending = root.join("promotion-transaction_collision-intent.pending");
    let failure = publish_intent_from_named_scratch(
        repo.path(),
        &root,
        &intent_pending,
        &intent,
        intent_token,
    )
    .unwrap_err();
    assert_eq!(failure.kind(), CharterPromotionErrorKindV1::IoFailure);
    assert_eq!(
        std::fs::read(&intent_scratch).unwrap(),
        b"retained intent collision evidence"
    );
    assert!(!intent_pending.exists());

    let output_staging = root.join(super::OUTPUT_STAGING_NAME);
    std::fs::create_dir(&output_staging).unwrap();
    for (index, purpose) in OutputPurposeV12::ALL.into_iter().enumerate() {
        let token = format!("{index:032x}");
        let scratch =
            output_staging.join(format!("{token}.{}", super::output_scratch_suffix(purpose)));
        std::fs::write(&scratch, b"retained output collision evidence").unwrap();
        let pending = root.join(format!("promotion-transaction_collision-{index}.pending"));
        std::fs::create_dir(&pending).unwrap();
        let failure = publish_output_stage_from_named_scratch(
            repo.path(),
            &root,
            &pending,
            NamedOutputScratchPublicationV12 {
                purpose,
                expected: purpose_bytes(purpose, &request),
                intent: &intent.record,
                scratch_token: &token,
            },
        )
        .unwrap_err();
        assert_eq!(failure.kind(), CharterPromotionErrorKindV1::IoFailure);
        assert_eq!(
            std::fs::read(&scratch).unwrap(),
            b"retained output collision evidence"
        );
        assert!(!pending.join(output_stage_name(purpose)).exists());
    }
}

#[test]
fn orphan_scratch_is_ignored_and_retained_across_writer_and_recovery() {
    let (repo, service, request, _prepared) = fixture();
    let root = repo.path().join(".handbook/state/transactions/promotions");
    let intent_staging = root.join(super::INTENT_STAGING_NAME);
    let output_staging = root.join(super::OUTPUT_STAGING_NAME);
    std::fs::create_dir_all(&intent_staging).unwrap();
    std::fs::create_dir(&output_staging).unwrap();
    let intent_orphan = intent_staging.join("ffffffffffffffffffffffffffffffff.intent");
    let output_orphan = output_staging.join("eeeeeeeeeeeeeeeeeeeeeeeeeeeeeeee.canonical");
    std::fs::write(&intent_orphan, b"untrusted orphan intent").unwrap();
    std::fs::write(&output_orphan, b"untrusted orphan output").unwrap();

    service
        .promote_with_fault_for_testing(
            request,
            CharterPromotionFaultPointV1::Writer(WriterBoundaryV12::W3IntentPublished),
        )
        .unwrap_err();
    assert!(service.read_committed_charter().unwrap().is_none());
    assert_eq!(
        std::fs::read(intent_orphan).unwrap(),
        b"untrusted orphan intent"
    );
    assert_eq!(
        std::fs::read(output_orphan).unwrap(),
        b"untrusted orphan output"
    );
}

#[test]
fn every_w0_through_w15_fault_recovers_to_its_declared_authority_side() {
    for boundary in WriterBoundaryV12::ALL {
        let (repo, service, request, _prepared) = fixture();
        let failure = service
            .promote_with_fault_for_testing(
                request.clone(),
                CharterPromotionFaultPointV1::Writer(boundary),
            )
            .unwrap_err();
        assert_eq!(failure.kind(), CharterPromotionErrorKindV1::InjectedFault);
        let observed = service.read_committed_charter().unwrap();
        if boundary < WriterBoundaryV12::W9CanonicalInstalled {
            assert!(observed.is_none(), "{boundary:?} did not roll back");
            assert!(!repo.path().join(".handbook/project/charter.yaml").exists());
        } else {
            assert_eq!(
                observed.expect("W9+ must roll forward").canonical_bytes,
                request.canonical_bytes,
                "{boundary:?}"
            );
        }
    }
}

#[test]
fn every_r0_through_r9_fault_is_replay_safe_from_every_w3_through_w9_old_origin() {
    let origins = [
        ("W3", WriterBoundaryV12::W3IntentPublished),
        ("W4", WriterBoundaryV12::W4OldSnapshot),
        ("W5", WriterBoundaryV12::W5CanonicalStaged),
        ("W6", WriterBoundaryV12::W6PromotionStaged),
        ("W7", WriterBoundaryV12::W7LifecycleStaged),
        ("W8", WriterBoundaryV12::W8Prepared),
        // The old-target observation during W9 is byte-identical to the durable W8
        // state because the canonical rename has not yet become observable.
        ("W9-old", WriterBoundaryV12::W8Prepared),
    ];
    for (origin_name, origin_fault) in origins {
        for recovery_fault in RecoveryBoundaryV12::ALL {
            let (repo, service, request, _prepared) = fixture();
            let writer_failure = service
                .promote_with_fault_for_testing(
                    request,
                    CharterPromotionFaultPointV1::Writer(origin_fault),
                )
                .unwrap_err();
            assert_eq!(
                writer_failure.kind(),
                CharterPromotionErrorKindV1::InjectedFault
            );
            let recovery_failure = service
                .recover_with_fault_for_testing(recovery_fault)
                .unwrap_err();
            assert_eq!(
                recovery_failure.kind(),
                CharterPromotionErrorKindV1::InjectedFault,
                "{origin_name} × {recovery_fault:?}"
            );
            assert!(
                service.read_committed_charter().unwrap().is_none(),
                "{origin_name} × {recovery_fault:?}"
            );
            assert!(!repo.path().join(".handbook/project/charter.yaml").exists());
        }
    }
}

#[test]
fn committed_and_rolled_back_terminal_destinations_cover_the_exact_eight_pair_product() {
    let mut seen = BTreeSet::new();
    for kind in [
        TerminalFixtureKind::Committed,
        TerminalFixtureKind::RolledBack,
    ] {
        for destination_state in [
            TerminalFixtureDestination::Absent,
            TerminalFixtureDestination::ExactPreExisting,
            TerminalFixtureDestination::Mismatching,
            TerminalFixtureDestination::Unsafe,
        ] {
            assert!(seen.insert((format!("{kind:?}"), format!("{destination_state:?}"))));
            let (repo, service, request, _prepared) = fixture();
            match kind {
                TerminalFixtureKind::Committed => {
                    service
                        .promote_with_fault_for_testing(
                            request,
                            CharterPromotionFaultPointV1::Writer(WriterBoundaryV12::W14Committed),
                        )
                        .unwrap_err();
                }
                TerminalFixtureKind::RolledBack => {
                    service
                        .promote_with_fault_for_testing(
                            request,
                            CharterPromotionFaultPointV1::Writer(
                                WriterBoundaryV12::W3IntentPublished,
                            ),
                        )
                        .unwrap_err();
                    service
                        .recover_with_fault_for_testing(
                            RecoveryBoundaryV12::R8RollbackMarkerPublished,
                        )
                        .unwrap_err();
                }
            }
            let root = repo.path().join(".handbook/state/transactions/promotions");
            let pending = std::fs::read_dir(&root)
                .unwrap()
                .map(|entry| entry.unwrap().path())
                .find(|path| {
                    path.file_name()
                        .unwrap()
                        .to_string_lossy()
                        .ends_with(".pending")
                })
                .unwrap();
            let base = pending
                .file_name()
                .unwrap()
                .to_string_lossy()
                .strip_suffix(".pending")
                .unwrap()
                .to_owned();
            let suffix = match kind {
                TerminalFixtureKind::Committed => ".committed",
                TerminalFixtureKind::RolledBack => ".rolled-back",
            };
            let destination = root.join(format!("{base}{suffix}"));
            match destination_state {
                TerminalFixtureDestination::Absent => {}
                TerminalFixtureDestination::ExactPreExisting => {
                    copy_directory_files(&pending, &destination)
                }
                TerminalFixtureDestination::Mismatching => {
                    std::fs::create_dir(&destination).unwrap();
                    std::fs::write(destination.join("mismatch"), b"evidence").unwrap();
                }
                TerminalFixtureDestination::Unsafe => {
                    std::fs::write(&destination, b"unsafe terminal type").unwrap();
                }
            }
            let result = service.read_committed_charter();
            if matches!(destination_state, TerminalFixtureDestination::Absent) {
                match kind {
                    TerminalFixtureKind::Committed => assert!(result.unwrap().is_some()),
                    TerminalFixtureKind::RolledBack => assert!(result.unwrap().is_none()),
                }
                assert!(!pending.exists());
                assert!(destination.exists());
            } else {
                assert_eq!(
                    result.unwrap_err().kind(),
                    CharterPromotionErrorKindV1::Conflict
                );
                assert!(pending.exists(), "pending evidence was removed");
            }
        }
    }
    assert_eq!(seen.len(), 8);
}

#[test]
fn terminal_marker_pending_fsync_replay_covers_both_markers_by_four_boundaries() {
    let mut seen = BTreeSet::new();
    for kind in [
        TerminalFixtureKind::Committed,
        TerminalFixtureKind::RolledBack,
    ] {
        for boundary in 0..4 {
            assert!(seen.insert((format!("{kind:?}"), boundary)));
            let (repo, service, request, _prepared) = fixture();
            match kind {
                TerminalFixtureKind::Committed => {
                    service
                        .promote_with_fault_for_testing(
                            request,
                            CharterPromotionFaultPointV1::Writer(
                                WriterBoundaryV12::W13RecordsMarked,
                            ),
                        )
                        .unwrap_err();
                }
                TerminalFixtureKind::RolledBack => {
                    service
                        .promote_with_fault_for_testing(
                            request,
                            CharterPromotionFaultPointV1::Writer(
                                WriterBoundaryV12::W3IntentPublished,
                            ),
                        )
                        .unwrap_err();
                    service
                        .recover_with_fault_for_testing(RecoveryBoundaryV12::R7RollbackMarkerTemp)
                        .unwrap_err();
                }
            }
            let root = repo.path().join(".handbook/state/transactions/promotions");
            let pending = std::fs::read_dir(&root)
                .unwrap()
                .map(|entry| entry.unwrap().path())
                .find(|path| {
                    path.file_name()
                        .unwrap()
                        .to_string_lossy()
                        .ends_with(".pending")
                })
                .unwrap();
            let intent = super::read_valid_intent(&pending).unwrap();
            let (marker_name, terminal_kind) = match kind {
                TerminalFixtureKind::Committed => {
                    complete_marker_temp(&pending, "committed", &intent.raw_bytes).unwrap();
                    ("committed", PromotionTerminalKindV12::Committed)
                }
                TerminalFixtureKind::RolledBack => {
                    ("rolled-back", PromotionTerminalKindV12::RolledBack)
                }
            };
            atomic_rename_no_replace(
                &pending.join(format!("{marker_name}.tmp")),
                &pending.join(marker_name),
            )
            .unwrap();
            if boundary >= 1 {
                crate::charter_lineage_store::sync_directory(&pending).unwrap();
            }
            if boundary >= 2 {
                crate::charter_lineage_store::sync_directory(&pending).unwrap();
            }
            if boundary >= 3 {
                validate_terminal_payload(&pending, &intent, terminal_kind).unwrap();
            }

            let observed = service.read_committed_charter().unwrap();
            match kind {
                TerminalFixtureKind::Committed => assert!(observed.is_some()),
                TerminalFixtureKind::RolledBack => assert!(observed.is_none()),
            }
            assert!(!pending.exists());
        }
    }
    assert_eq!(seen.len(), 8);
}

#[test]
fn partial_pending_output_mismatch_is_preserved_for_all_three_purposes() {
    for (purpose, origin) in [
        (
            OutputPurposeV12::Canonical,
            WriterBoundaryV12::W5CanonicalStaged,
        ),
        (
            OutputPurposeV12::PromotionRecord,
            WriterBoundaryV12::W6PromotionStaged,
        ),
        (
            OutputPurposeV12::LifecycleTransition,
            WriterBoundaryV12::W7LifecycleStaged,
        ),
    ] {
        let (repo, service, request, _prepared) = fixture();
        service
            .promote_with_fault_for_testing(request, CharterPromotionFaultPointV1::Writer(origin))
            .unwrap_err();
        let root = repo.path().join(".handbook/state/transactions/promotions");
        let pending = std::fs::read_dir(&root)
            .unwrap()
            .map(|entry| entry.unwrap().path())
            .find(|path| {
                path.file_name()
                    .unwrap()
                    .to_string_lossy()
                    .ends_with(".pending")
            })
            .unwrap();
        let stage = pending.join(output_stage_name(purpose));
        let exact = std::fs::read(&stage).unwrap();
        let partial = &exact[..exact.len() / 2];
        std::fs::write(&stage, partial).unwrap();
        assert!(service.read_committed_charter().is_err());
        assert_eq!(std::fs::read(&stage).unwrap(), partial);
        assert!(pending.exists());
    }
}

#[test]
fn all_twenty_seven_pending_stage_states_admit_only_the_writer_prefix_closure() {
    let states = [
        RetainedStageStateV12::Absent,
        RetainedStageStateV12::Exact,
        RetainedStageStateV12::Mismatch,
    ];
    let mut seen = BTreeSet::new();
    for canonical in states {
        for promotion in states {
            for lifecycle in states {
                assert!(seen.insert(format!("{canonical:?}/{promotion:?}/{lifecycle:?}")));
                let (repo, service, request, _prepared) = fixture();
                service
                    .promote_with_fault_for_testing(
                        request.clone(),
                        CharterPromotionFaultPointV1::Writer(WriterBoundaryV12::W3IntentPublished),
                    )
                    .unwrap_err();
                let root = repo.path().join(".handbook/state/transactions/promotions");
                let pending = std::fs::read_dir(&root)
                    .unwrap()
                    .map(|entry| entry.unwrap().path())
                    .find(|path| {
                        path.file_name()
                            .unwrap()
                            .to_string_lossy()
                            .ends_with(".pending")
                    })
                    .unwrap();
                for (purpose, state) in [
                    (OutputPurposeV12::Canonical, canonical),
                    (OutputPurposeV12::PromotionRecord, promotion),
                    (OutputPurposeV12::LifecycleTransition, lifecycle),
                ] {
                    let bytes = purpose_bytes(purpose, &request);
                    match state {
                        RetainedStageStateV12::Absent => {}
                        RetainedStageStateV12::Exact => {
                            std::fs::write(pending.join(output_stage_name(purpose)), bytes)
                                .unwrap();
                        }
                        RetainedStageStateV12::Mismatch => {
                            std::fs::write(
                                pending.join(output_stage_name(purpose)),
                                &bytes[..bytes.len() / 2],
                            )
                            .unwrap();
                        }
                    }
                }
                let admitted = matches!(
                    [canonical, promotion, lifecycle],
                    [
                        RetainedStageStateV12::Absent,
                        RetainedStageStateV12::Absent,
                        RetainedStageStateV12::Absent
                    ] | [
                        RetainedStageStateV12::Exact,
                        RetainedStageStateV12::Absent,
                        RetainedStageStateV12::Absent
                    ] | [
                        RetainedStageStateV12::Exact,
                        RetainedStageStateV12::Exact,
                        RetainedStageStateV12::Absent
                    ] | [
                        RetainedStageStateV12::Exact,
                        RetainedStageStateV12::Exact,
                        RetainedStageStateV12::Exact
                    ]
                );
                if admitted {
                    assert!(service.read_committed_charter().unwrap().is_none());
                    assert!(!pending.exists());
                } else {
                    assert!(service.read_committed_charter().is_err());
                    assert!(pending.exists(), "non-writer evidence was removed");
                }
            }
        }
    }
    assert_eq!(seen.len(), 27);
}

#[test]
fn marker_prefix_completion_covers_zero_through_exactly_72_bytes() {
    let intent_bytes = b"exact intent bytes\n";
    let expected = super::promotion_intent_marker_v12(intent_bytes);
    assert_eq!(expected.len(), 72);
    for prefix in 0..=expected.len() {
        let repo = tempfile::tempdir().unwrap();
        let pending = repo.path().join("pending");
        std::fs::create_dir(&pending).unwrap();
        std::fs::write(pending.join("prepared.tmp"), &expected[..prefix]).unwrap();
        publish_marker(&pending, "prepared", intent_bytes).unwrap();
        assert_eq!(std::fs::read(pending.join("prepared")).unwrap(), expected);
        assert!(!pending.join("prepared.tmp").exists());
    }
}

#[test]
fn amendment_snapshot_completion_covers_every_authentic_old_prefix() {
    let (_repo, _service, _request, mut prepared) = fixture();
    let old = b"old canonical";
    let old_fingerprint = super::DefinitionFingerprint::from_bytes(old).to_string();
    prepared.expected_current_fingerprint = Some(old_fingerprint);
    let prior_fingerprint = format!("sha256:{}", "1".repeat(64));
    prepared.selected_contract.prior_lifecycle_head_ref = Some(format!(
        "lifecycle-transitions/lifecycle-transition_{}.json",
        "1".repeat(64)
    ));
    prepared.selected_contract.prior_lifecycle_head_fingerprint = Some(prior_fingerprint);
    prepared.selected_contract.prior_lifecycle_state = "current".to_owned();
    prepared.selected_contract.prior_lifecycle_state_fingerprint =
        Some(format!("sha256:{}", "2".repeat(64)));
    let intent = build_promotion_intent(
        &prepared,
        Some(old),
        "promotion-transaction_snapshot-prefix",
    )
    .unwrap();
    for prefix in 0..=old.len() {
        let repo = tempfile::tempdir().unwrap();
        let pending = repo.path().join("pending");
        std::fs::create_dir(&pending).unwrap();
        if prefix > 0 {
            std::fs::write(pending.join("canonical.old"), &old[..prefix]).unwrap();
        }
        complete_old_snapshot(&pending, &intent.record, Some(old)).unwrap();
        assert_eq!(std::fs::read(pending.join("canonical.old")).unwrap(), old);
    }
}

fn sole_pending_transaction(repo: &std::path::Path) -> std::path::PathBuf {
    std::fs::read_dir(repo.join(".handbook/state/transactions/promotions"))
        .unwrap()
        .map(|entry| entry.unwrap().path())
        .find(|path| {
            path.file_name()
                .unwrap()
                .to_string_lossy()
                .ends_with(".pending")
        })
        .unwrap()
}

#[test]
fn w9_recovery_revalidates_retained_intake_and_candidate_content_before_commit() {
    for target in ["intake", "content"] {
        let (repo, service, request, _prepared) = fixture();
        service
            .promote_with_fault_for_testing(
                request,
                CharterPromotionFaultPointV1::Writer(WriterBoundaryV12::W9CanonicalInstalled),
            )
            .expect_err("W9 fault");
        let pending = sole_pending_transaction(repo.path());
        let intent = super::read_valid_intent(&pending).unwrap();
        let candidate_path = repo
            .path()
            .join(".handbook/evidence/charter")
            .join(&intent.record.candidate_lineage.candidate_ref);
        let candidate: serde_json::Value =
            serde_json::from_slice(&std::fs::read(candidate_path).unwrap()).unwrap();
        let relative = match target {
            "intake" => candidate["intake_record_ref"].as_str().unwrap(),
            "content" => candidate["normalized_content_ref"].as_str().unwrap(),
            _ => unreachable!(),
        };
        let path = repo.path().join(".handbook/state").join(relative);
        let mut tampered = std::fs::read(&path).unwrap();
        tampered[0] ^= 1;
        std::fs::write(&path, &tampered).unwrap();

        let refusal = service
            .read_committed_charter()
            .expect_err("recovery must revalidate every retained authority byte");
        assert!(matches!(
            refusal.kind(),
            CharterPromotionErrorKindV1::LineageViolation
                | CharterPromotionErrorKindV1::DurabilityViolation
        ));
        assert!(pending.exists(), "pending journal evidence was removed");
        assert!(!pending.join("committed").exists());
        assert_eq!(std::fs::read(path).unwrap(), tampered);
    }
}

fn resign_test_intent(value: &mut serde_json::Value) -> Vec<u8> {
    let mut preimage = value.clone();
    preimage
        .as_object_mut()
        .unwrap()
        .remove("intent_fingerprint");
    value["intent_fingerprint"] = serde_json::Value::String(
        super::DefinitionFingerprint::from_json_value(&preimage)
            .unwrap()
            .to_string(),
    );
    let mut bytes = serde_json_canonicalizer::to_vec(value).unwrap();
    bytes.push(b'\n');
    bytes
}

fn publish_unselected_historical_fixture(
    repo: &std::path::Path,
    source_intent: &[u8],
    case: &str,
) -> std::path::PathBuf {
    let transaction_id = format!("promotion-transaction_historical-{case}");
    let mut value: serde_json::Value = serde_json::from_slice(source_intent).unwrap();
    value["transaction_id"] = serde_json::Value::String(transaction_id.clone());
    let fake = format!("sha256:{}", "f".repeat(64));
    value["outputs"]["new_canonical_fingerprint"] = serde_json::Value::String(fake.clone());
    value["outputs"]["new_canonical_document_sha256"] = serde_json::Value::String(fake);
    let bytes = resign_test_intent(&mut value);
    let directory = repo
        .join(".handbook/state/transactions/promotions")
        .join(format!("{transaction_id}.committed"));
    std::fs::create_dir(&directory).unwrap();
    std::fs::write(directory.join("intent.json"), &bytes).unwrap();
    let marker = super::promotion_intent_marker_v12(&bytes);
    for name in [
        "prepared",
        "canonical-installed",
        "records-installed",
        "committed",
    ] {
        std::fs::write(directory.join(name), &marker).unwrap();
    }
    match case {
        "marker" => std::fs::write(directory.join("committed"), vec![b'0'; 72]).unwrap(),
        "name" => std::fs::write(directory.join("unexpected"), b"retained evidence").unwrap(),
        "final" => {}
        _ => unreachable!(),
    }
    directory
}

#[test]
fn every_historical_committed_terminal_and_rolled_back_terminal_is_validated() {
    for case in ["marker", "name", "final"] {
        let (repo, service, request, _prepared) = fixture();
        let commit = service.promote(request).unwrap();
        let source_intent = std::fs::read(
            commit
                .committed_marker_path
                .parent()
                .unwrap()
                .join("intent.json"),
        )
        .unwrap();
        let historical = publish_unselected_historical_fixture(repo.path(), &source_intent, case);
        assert!(
            service.read_committed_charter().is_err(),
            "historical {case} corruption was skipped"
        );
        assert!(historical.exists());
    }

    let (repo, service, request, _prepared) = fixture();
    let commit = service.promote(request).unwrap();
    let source_intent = std::fs::read(
        commit
            .committed_marker_path
            .parent()
            .unwrap()
            .join("intent.json"),
    )
    .unwrap();
    let transaction_id = "promotion-transaction_malformed-rollback";
    let mut value: serde_json::Value = serde_json::from_slice(&source_intent).unwrap();
    value["transaction_id"] = serde_json::Value::String(transaction_id.to_owned());
    let bytes = resign_test_intent(&mut value);
    let rolled_back = repo
        .path()
        .join(".handbook/state/transactions/promotions")
        .join(format!("{transaction_id}.rolled-back"));
    std::fs::create_dir(&rolled_back).unwrap();
    std::fs::write(rolled_back.join("intent.json"), bytes).unwrap();
    std::fs::write(rolled_back.join("rolled-back"), vec![b'0'; 72]).unwrap();
    assert!(service.read_committed_charter().is_err());
    assert!(rolled_back.exists());
}

#[test]
fn unsupported_transaction_root_suffix_is_preserved_mismatch() {
    let repo = tempfile::tempdir().unwrap();
    let root = repo.path().join(".handbook/state/transactions/promotions");
    std::fs::create_dir_all(&root).unwrap();
    let unsupported = root.join("promotion-transaction_unknown.other");
    std::fs::create_dir(&unsupported).unwrap();
    std::fs::write(unsupported.join("evidence"), b"preserve").unwrap();

    let service = CharterAuthorityTransactionServiceV1::new(repo.path());
    assert_eq!(
        service.read_committed_charter().unwrap_err().kind(),
        CharterPromotionErrorKindV1::DurabilityViolation
    );
    assert_eq!(
        std::fs::read(unsupported.join("evidence")).unwrap(),
        b"preserve"
    );
}

#[test]
fn malformed_history_blocks_pending_recovery_before_mutation() {
    let (repo, service, request, _prepared) = fixture();
    service
        .promote_with_fault_for_testing(
            request,
            CharterPromotionFaultPointV1::Writer(WriterBoundaryV12::W3IntentPublished),
        )
        .unwrap_err();
    let pending = sole_pending_transaction(repo.path());
    let before = std::fs::read_dir(&pending)
        .unwrap()
        .map(|entry| {
            let entry = entry.unwrap();
            (entry.file_name(), std::fs::read(entry.path()).unwrap())
        })
        .collect::<Vec<_>>();
    let source_intent = std::fs::read(pending.join("intent.json")).unwrap();
    let malformed = publish_unselected_historical_fixture(repo.path(), &source_intent, "marker");

    assert!(service.read_committed_charter().is_err());
    assert!(
        malformed.exists(),
        "malformed historical evidence was removed"
    );
    assert!(
        pending.exists(),
        "pending journal mutated before history refusal"
    );
    let after = std::fs::read_dir(&pending)
        .unwrap()
        .map(|entry| {
            let entry = entry.unwrap();
            (entry.file_name(), std::fs::read(entry.path()).unwrap())
        })
        .collect::<Vec<_>>();
    assert_eq!(
        after, before,
        "pending bytes changed before history refusal"
    );
}
