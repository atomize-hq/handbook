use handbook_contracts::{
    lifecycle_policy_binding_v1, prepare_contract_definition_append, ApplicabilityRuleV1,
    ContractApprovalClassV1, ContractClaimInputV1, ContractClaimV1, ContractDefinitionInputV1,
    ContractDefinitionV1, ContractIdV1, ContractKindV1, ContractLifecycleTransitionInputV1,
    ContractOwnerRepositoryErrorKindV1, ContractOwnerRepositoryV1, DefinitionListRequestV1,
    DefinitionReadOutcomeV1, EvaluationCaseV1, EvaluationPolicyRegistryV1, EvaluationRunNonceV1,
    EvaluationSubjectV1, EvidenceCardinalityV1, EvidenceCaseSelectionV1,
    EvidenceFreshnessPolicyInputV1, EvidenceFreshnessPolicyV1, EvidenceRequirementV1,
    ExactBindingV1, ExactRefV1, FingerprintV1, GateEffectV1, GatePolicyAlgorithmV1,
    GatePolicyInputV1, GatePolicyV1, LifecycleEvidenceKindV1, LifecycleReasonTextV1,
    LifecycleReasonV1, LifecycleStateV1, LifecycleSupportingEvidenceV1, MatcherBindingV1,
    MatcherInputValueV1, OwnerMutationDispositionV1, PrepareEvaluationRunInputV1,
    ResolutionEnvelopeV1, ResolutionMinimumV1, SelectorCriterionV1, SelectorV1, StableIdV1,
    VersionV1, CONTRACT_OWNER_REPOSITORY_RELATIVE_ROOT_V1,
};
use handbook_engine::{
    observe_committed_approver_registry, ApproverAdminRequestV1, ApproverRegistryServiceV1,
    CredentialAuthorityAdmissionRequestV1, CredentialAuthorityAdmissionServiceV1,
    NativeAuthenticatorPortErrorV1, NativeAuthenticatorPortV1, AUTHENTICATOR_RP_ID,
};
use p256::ecdsa::{signature::Signer, Signature, SigningKey};
use serde_json::json;
use sha2::{Digest, Sha256};
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::{Arc, Barrier};

#[cfg(feature = "test-support")]
use handbook_contracts::OwnerRepositoryFaultBoundaryV1;

const REPOSITORY_IDENTITY: &str =
    "sha256:1111111111111111111111111111111111111111111111111111111111111111";
const AUTHOR_AUTHORITY_REF: &str = "handbook.authority.contract-definition-author@1.0.0";

#[test]
fn owner_repository_uses_the_fixed_repository_relative_layout_and_empty_root() {
    let repo = tempfile::tempdir().unwrap();
    let owner = ContractOwnerRepositoryV1::open(repo.path()).unwrap();

    assert_eq!(
        CONTRACT_OWNER_REPOSITORY_RELATIVE_ROOT_V1,
        ".handbook/state/contracts/v1"
    );
    let page = owner
        .list_definitions(DefinitionListRequestV1::first(10).unwrap())
        .unwrap();
    assert!(page.entries().is_empty());
    assert!(page.next_cursor().is_none());
    assert!(page.is_current());
}

#[test]
fn credential_admitted_append_exact_read_replay_and_conflict_are_owner_truth() {
    let mut harness = Harness::new();
    let initial_definition = definition(&harness.registry_state_fingerprint, "1.0.0");
    let exact_ref = initial_definition.contract_ref().clone();
    let exact_fingerprint = initial_definition.definition_fingerprint().clone();
    let key = "owner-append-key-0001";

    let first = harness.append(initial_definition.clone(), key).unwrap();
    assert_eq!(first.disposition(), OwnerMutationDispositionV1::Committed);
    assert_eq!(first.contract_ref(), &exact_ref);
    assert_eq!(first.definition_fingerprint(), &exact_fingerprint);

    let read = harness
        .owner
        .read_definition(&exact_ref, &exact_fingerprint)
        .unwrap();
    let DefinitionReadOutcomeV1::Found(record) = read else {
        panic!("definition must be present");
    };
    assert_eq!(record.definition().definition(), &initial_definition);
    assert_eq!(record.record_ref(), first.record_ref());
    assert_eq!(record.record_fingerprint(), first.record_fingerprint());
    assert!(!record.record_bytes().is_empty());

    let replay = harness.append(initial_definition, key).unwrap();
    assert_eq!(replay.disposition(), OwnerMutationDispositionV1::Replayed);
    assert_eq!(replay.request_fingerprint(), first.request_fingerprint());

    let conflict = harness.append(
        definition(&harness.registry_state_fingerprint, "1.0.1"),
        key,
    );
    assert_eq!(
        conflict.unwrap_err().kind(),
        ContractOwnerRepositoryErrorKindV1::IdempotencyConflict
    );

    let persisted = all_persisted_bytes(harness.repo.path());
    assert!(!persisted
        .windows(key.len())
        .any(|window| window == key.as_bytes()));
}

#[test]
fn definition_pages_are_stable_bounded_current_and_cursor_bound() {
    let mut harness = Harness::new();
    for (version, key) in [
        ("1.0.2", "owner-page-key-0002"),
        ("1.0.0", "owner-page-key-0000"),
        ("1.0.1", "owner-page-key-0001"),
    ] {
        harness
            .append(
                definition(&harness.registry_state_fingerprint, version),
                key,
            )
            .unwrap();
    }

    let first = harness
        .owner
        .list_definitions(DefinitionListRequestV1::first(2).unwrap())
        .unwrap();
    assert_eq!(first.entries().len(), 2);
    assert!(first.entries()[0].contract_ref() < first.entries()[1].contract_ref());
    let cursor = first.next_cursor().unwrap().to_owned();
    let second = harness
        .owner
        .list_definitions(
            DefinitionListRequestV1::continue_from(2, first.root().clone(), cursor.clone())
                .unwrap(),
        )
        .unwrap();
    assert_eq!(second.root(), first.root());
    assert_eq!(second.entries().len(), 1);
    assert!(second.next_cursor().is_none());

    let mut substituted = cursor;
    let last = substituted.pop().unwrap();
    substituted.push(if last == '0' { '1' } else { '0' });
    let refused = harness.owner.list_definitions(
        DefinitionListRequestV1::continue_from(2, first.root().clone(), substituted).unwrap(),
    );
    assert_eq!(
        refused.unwrap_err().kind(),
        ContractOwnerRepositoryErrorKindV1::RecordSubstituted
    );

    harness
        .append(
            definition(&harness.registry_state_fingerprint, "1.0.3"),
            "owner-page-key-0003",
        )
        .unwrap();
    let stale = harness.owner.list_definitions(
        DefinitionListRequestV1::continue_from(
            2,
            first.root().clone(),
            first.next_cursor().unwrap(),
        )
        .unwrap(),
    );
    assert_eq!(
        stale.unwrap_err().kind(),
        ContractOwnerRepositoryErrorKindV1::RecordSubstituted
    );
    assert_eq!(
        DefinitionListRequestV1::first(101).unwrap_err().kind(),
        ContractOwnerRepositoryErrorKindV1::BoundExceeded
    );
}

#[test]
fn lifecycle_compare_and_write_rechecks_head_and_replays_exactly() {
    let mut harness = Harness::new();
    let definition = definition(&harness.registry_state_fingerprint, "1.0.0");
    let contract_ref = definition.contract_ref().clone();
    let definition_fingerprint = definition.definition_fingerprint().clone();
    let append = harness
        .append(definition, "owner-transition-append-0001")
        .unwrap();
    let draft = append.lifecycle_head().clone();
    let ready_input = ContractLifecycleTransitionInputV1 {
        expected_lifecycle_fingerprint: draft.lifecycle_fingerprint().clone(),
        from_state: LifecycleStateV1::Draft,
        to_state: LifecycleStateV1::ReviewReady,
        approval_class: ContractApprovalClassV1::DefinitionAuthor,
        supporting_evidence: vec![LifecycleSupportingEvidenceV1::new(
            LifecycleEvidenceKindV1::DefinitionValidation,
            binding("handbook.contract-lifecycle-evidence.owner@1.0.0", '1'),
        )],
        reason: LifecycleReasonV1::ReadyForReview,
    };
    let ready = harness
        .owner
        .prepare_lifecycle_transition(&contract_ref, &definition_fingerprint, ready_input)
        .unwrap();
    let retry = ready.clone();
    let wrong_action_retry = ready.clone();
    let wrong_role_retry = ready.clone();
    let stale_alternative = harness
        .owner
        .prepare_lifecycle_transition(
            &contract_ref,
            &definition_fingerprint,
            ContractLifecycleTransitionInputV1 {
                expected_lifecycle_fingerprint: draft.lifecycle_fingerprint().clone(),
                from_state: LifecycleStateV1::Draft,
                to_state: LifecycleStateV1::Closed,
                approval_class: ContractApprovalClassV1::DefinitionAuthor,
                supporting_evidence: vec![],
                reason: LifecycleReasonV1::Withdrawn,
            },
        )
        .unwrap();

    let first = harness
        .service
        .admit(ready.authority_request(), |admission| {
            harness
                .owner
                .transition_lifecycle(ready, admission, "owner-transition-key-0001")
        })
        .unwrap()
        .unwrap();
    assert_eq!(first.disposition(), OwnerMutationDispositionV1::Committed);
    assert_eq!(
        first.lifecycle_head().state(),
        LifecycleStateV1::ReviewReady
    );
    assert_eq!(
        harness
            .owner
            .current_lifecycle_head(&contract_ref, &definition_fingerprint)
            .unwrap(),
        *first.lifecycle_head()
    );

    let replay = harness
        .service
        .admit(retry.authority_request(), |admission| {
            harness
                .owner
                .transition_lifecycle(retry, admission, "owner-transition-key-0001")
        })
        .unwrap()
        .unwrap();
    assert_eq!(replay.disposition(), OwnerMutationDispositionV1::Replayed);
    assert_eq!(replay.transition_ref(), first.transition_ref());

    let exact_request = wrong_action_retry.authority_request();
    let wrong_action_request = CredentialAuthorityAdmissionRequestV1::new(
        "handbook.contract-definition.append@1.0.0",
        exact_request.subject_ref(),
        exact_request.approval_class(),
        exact_request.semantic_scope_fingerprint(),
    )
    .unwrap();
    let wrong_action_replay = harness
        .service
        .admit(wrong_action_request, |admission| {
            harness.owner.transition_lifecycle(
                wrong_action_retry,
                admission,
                "owner-transition-key-0001",
            )
        })
        .unwrap()
        .unwrap_err();
    assert_eq!(
        wrong_action_replay.kind(),
        ContractOwnerRepositoryErrorKindV1::LifecycleRefused
    );

    let wrong_role_request = CredentialAuthorityAdmissionRequestV1::new(
        exact_request.action_ref(),
        exact_request.subject_ref(),
        ContractApprovalClassV1::Lifecycle.as_str(),
        exact_request.semantic_scope_fingerprint(),
    )
    .unwrap();
    let wrong_role_replay = harness
        .service
        .admit(wrong_role_request, |admission| {
            harness.owner.transition_lifecycle(
                wrong_role_retry,
                admission,
                "owner-transition-key-0001",
            )
        })
        .unwrap()
        .unwrap_err();
    assert_eq!(
        wrong_role_replay.kind(),
        ContractOwnerRepositoryErrorKindV1::LifecycleRefused
    );

    let conflict = harness
        .service
        .admit(stale_alternative.authority_request(), |admission| {
            harness.owner.transition_lifecycle(
                stale_alternative.clone(),
                admission,
                "owner-transition-key-0001",
            )
        })
        .unwrap()
        .unwrap_err();
    assert_eq!(
        conflict.kind(),
        ContractOwnerRepositoryErrorKindV1::IdempotencyConflict
    );
    let stale = harness
        .service
        .admit(stale_alternative.authority_request(), |admission| {
            harness.owner.transition_lifecycle(
                stale_alternative,
                admission,
                "owner-transition-key-0002",
            )
        })
        .unwrap()
        .unwrap_err();
    assert_eq!(
        stale.kind(),
        ContractOwnerRepositoryErrorKindV1::LifecycleRefused
    );

    let reopened = ContractOwnerRepositoryV1::open(harness.repo.path()).unwrap();
    assert_eq!(
        reopened
            .current_lifecycle_head(&contract_ref, &definition_fingerprint)
            .unwrap(),
        *first.lifecycle_head()
    );
    let heads = harness
        .repo
        .path()
        .join(".handbook/state/contracts/v1/heads");
    let head_files = fs::read_dir(heads)
        .unwrap()
        .map(|entry| entry.unwrap().file_name().into_string().unwrap())
        .collect::<Vec<_>>();
    assert_eq!(
        head_files,
        vec![format!(
            "{}.json",
            definition_fingerprint
                .as_str()
                .strip_prefix("sha256:")
                .unwrap()
        )]
    );
}

#[test]
fn append_recovery_completes_or_rolls_back_every_established_stage() {
    for prefix in 0..=3 {
        let mut harness = Harness::new();
        harness
            .append(
                definition(&harness.registry_state_fingerprint, "1.0.0"),
                &format!("owner-recovery-marker-{prefix:04}"),
            )
            .unwrap();
        let pending = reopen_as_pending(harness.repo.path());
        remove_installed_suffix(harness.repo.path(), &pending, prefix);
        let reopened = ContractOwnerRepositoryV1::open(harness.repo.path()).unwrap();
        assert_eq!(
            reopened
                .list_definitions(DefinitionListRequestV1::first(10).unwrap())
                .unwrap()
                .entries()
                .len(),
            1,
            "marker recovery failed at installed prefix {prefix}"
        );
    }

    for retained_staged in 0..=3 {
        let mut harness = Harness::new();
        harness
            .append(
                definition(&harness.registry_state_fingerprint, "1.0.0"),
                &format!("owner-recovery-unestablished-{retained_staged:04}"),
            )
            .unwrap();
        let pending = reopen_as_pending(harness.repo.path());
        remove_installed_suffix(harness.repo.path(), &pending, 0);
        fs::remove_file(pending.join("intent.json")).unwrap();
        fs::remove_file(pending.join("commit-marker.json")).unwrap();
        for staged in staged_paths(&pending).into_iter().skip(retained_staged) {
            fs::remove_file(staged).unwrap();
        }
        let reopened = ContractOwnerRepositoryV1::open(harness.repo.path()).unwrap();
        assert!(reopened
            .list_definitions(DefinitionListRequestV1::first(10).unwrap())
            .unwrap()
            .entries()
            .is_empty());
    }

    let mut harness = Harness::new();
    harness
        .append(
            definition(&harness.registry_state_fingerprint, "1.0.0"),
            "owner-recovery-markerless-0001",
        )
        .unwrap();
    let pending = reopen_as_pending(harness.repo.path());
    remove_installed_suffix(harness.repo.path(), &pending, 1);
    fs::remove_file(pending.join("commit-marker.json")).unwrap();
    let reopened = ContractOwnerRepositoryV1::open(harness.repo.path()).unwrap();
    assert_eq!(
        reopened
            .list_definitions(DefinitionListRequestV1::first(10).unwrap())
            .unwrap()
            .entries()
            .len(),
        1
    );
}

#[cfg(feature = "test-support")]
#[test]
fn injected_append_faults_reopen_to_exact_rollback_or_commit_at_every_boundary() {
    use OwnerRepositoryFaultBoundaryV1 as Fault;

    for (fault, commits) in [
        (Fault::PendingDirectoryDurable, false),
        (Fault::StagedPrimaryDurable, false),
        (Fault::StagedHeadDurable, false),
        (Fault::StagedLedgerDurable, false),
        (Fault::IntentDurable, false),
        (Fault::MarkerDurable, true),
        (Fault::PrimaryInstalledDurable, true),
        (Fault::HeadInstalledDurable, true),
        (Fault::LedgerInstalledDurable, true),
        (Fault::CommittedRenameDurable, true),
    ] {
        let mut harness = Harness::new();
        harness.owner = ContractOwnerRepositoryV1::open(harness.repo.path())
            .unwrap()
            .with_fault_for_testing(fault);
        let result = harness.append(
            definition(&harness.registry_state_fingerprint, "1.0.0"),
            "owner-injected-fault-append-0001",
        );
        assert_eq!(
            result.unwrap_err().kind(),
            ContractOwnerRepositoryErrorKindV1::DurabilityUncertain,
            "fault {fault:?} returned the wrong refusal"
        );
        let reopened = ContractOwnerRepositoryV1::open(harness.repo.path()).unwrap();
        assert_eq!(
            reopened
                .list_definitions(DefinitionListRequestV1::first(10).unwrap())
                .unwrap()
                .entries()
                .len(),
            usize::from(commits),
            "fault {fault:?} recovered to the wrong terminal state"
        );
    }
}

#[cfg(feature = "test-support")]
#[test]
fn injected_transition_faults_preserve_one_exact_old_or_new_head() {
    use OwnerRepositoryFaultBoundaryV1 as Fault;

    for (fault, commits) in [
        (Fault::PendingDirectoryDurable, false),
        (Fault::StagedPrimaryDurable, false),
        (Fault::StagedHeadDurable, false),
        (Fault::StagedLedgerDurable, false),
        (Fault::IntentDurable, false),
        (Fault::MarkerDurable, true),
        (Fault::PrimaryInstalledDurable, true),
        (Fault::HeadInstalledDurable, true),
        (Fault::LedgerInstalledDurable, true),
        (Fault::CommittedRenameDurable, true),
    ] {
        let mut harness = Harness::new();
        let definition = definition(&harness.registry_state_fingerprint, "1.0.0");
        let contract_ref = definition.contract_ref().clone();
        let definition_fingerprint = definition.definition_fingerprint().clone();
        let draft = harness
            .append(definition, "owner-injected-fault-base-0001")
            .unwrap()
            .lifecycle_head()
            .clone();
        harness.owner = ContractOwnerRepositoryV1::open(harness.repo.path())
            .unwrap()
            .with_fault_for_testing(fault);
        let prepared = harness
            .owner
            .prepare_lifecycle_transition(
                &contract_ref,
                &definition_fingerprint,
                ContractLifecycleTransitionInputV1 {
                    expected_lifecycle_fingerprint: draft.lifecycle_fingerprint().clone(),
                    from_state: LifecycleStateV1::Draft,
                    to_state: LifecycleStateV1::ReviewReady,
                    approval_class: ContractApprovalClassV1::DefinitionAuthor,
                    supporting_evidence: vec![LifecycleSupportingEvidenceV1::new(
                        LifecycleEvidenceKindV1::DefinitionValidation,
                        binding("handbook.contract-lifecycle-evidence.owner@1.0.0", '1'),
                    )],
                    reason: LifecycleReasonV1::ReadyForReview,
                },
            )
            .unwrap();
        let result = harness
            .service
            .admit(prepared.authority_request(), |admission| {
                harness.owner.transition_lifecycle(
                    prepared,
                    admission,
                    "owner-injected-fault-transition-0001",
                )
            })
            .unwrap();
        assert_eq!(
            result.unwrap_err().kind(),
            ContractOwnerRepositoryErrorKindV1::DurabilityUncertain,
            "fault {fault:?} returned the wrong refusal"
        );
        let reopened = ContractOwnerRepositoryV1::open(harness.repo.path()).unwrap();
        assert_eq!(
            reopened
                .current_lifecycle_head(&contract_ref, &definition_fingerprint)
                .unwrap()
                .state(),
            if commits {
                LifecycleStateV1::ReviewReady
            } else {
                LifecycleStateV1::Draft
            },
            "fault {fault:?} recovered the wrong head"
        );
        assert_eq!(
            fs::read_dir(
                harness
                    .repo
                    .path()
                    .join(".handbook/state/contracts/v1/heads"),
            )
            .unwrap()
            .count(),
            1
        );
    }
}

#[test]
fn recovery_fails_closed_on_non_prefix_and_tampered_state() {
    let mut harness = Harness::new();
    harness
        .append(
            definition(&harness.registry_state_fingerprint, "1.0.0"),
            "owner-recovery-extra-inventory-0001",
        )
        .unwrap();
    let committed = only_transaction(harness.repo.path());
    fs::write(committed.join("evidence.json"), b"forbidden").unwrap();
    assert_eq!(
        ContractOwnerRepositoryV1::open(harness.repo.path())
            .unwrap_err()
            .kind(),
        ContractOwnerRepositoryErrorKindV1::RecoveryAmbiguous
    );

    let mut harness = Harness::new();
    harness
        .append(
            definition(&harness.registry_state_fingerprint, "1.0.0"),
            "owner-recovery-duplicate-suffix-0001",
        )
        .unwrap();
    let committed = only_transaction(harness.repo.path());
    let pending = committed.with_file_name(
        committed
            .file_name()
            .unwrap()
            .to_str()
            .unwrap()
            .replace(".committed", ".pending"),
    );
    copy_tree(&committed, &pending);
    assert_eq!(
        ContractOwnerRepositoryV1::open(harness.repo.path())
            .unwrap_err()
            .kind(),
        ContractOwnerRepositoryErrorKindV1::RecoveryAmbiguous
    );

    let mut harness = Harness::new();
    harness
        .append(
            definition(&harness.registry_state_fingerprint, "1.0.0"),
            "owner-recovery-ambiguous-0001",
        )
        .unwrap();
    let pending = reopen_as_pending(harness.repo.path());
    let outputs = transaction_outputs(harness.repo.path(), &pending);
    fs::remove_file(&outputs[0]).unwrap();
    fs::remove_file(pending.join("commit-marker.json")).unwrap();
    let refusal = ContractOwnerRepositoryV1::open(harness.repo.path()).unwrap_err();
    assert_eq!(
        refusal.kind(),
        ContractOwnerRepositoryErrorKindV1::RecoveryAmbiguous
    );

    let mut harness = Harness::new();
    harness
        .append(
            definition(&harness.registry_state_fingerprint, "1.0.0"),
            "owner-recovery-tamper-0001",
        )
        .unwrap();
    let committed = only_transaction(harness.repo.path());
    let output = transaction_outputs(harness.repo.path(), &committed)[0].clone();
    let mut bytes = fs::read(&output).unwrap();
    bytes[0] ^= 1;
    fs::write(&output, bytes).unwrap();
    let refusal = ContractOwnerRepositoryV1::open(harness.repo.path()).unwrap_err();
    assert!(matches!(
        refusal.kind(),
        ContractOwnerRepositoryErrorKindV1::RecoveryAmbiguous
            | ContractOwnerRepositoryErrorKindV1::RecordSubstituted
    ));

    let mut harness = Harness::new();
    let definition = definition(&harness.registry_state_fingerprint, "1.0.0");
    let contract_ref = definition.contract_ref().clone();
    let definition_fingerprint = definition.definition_fingerprint().clone();
    let draft = harness
        .append(definition, "owner-recovery-orphan-append-0001")
        .unwrap()
        .lifecycle_head()
        .clone();
    harness.transition(
        &contract_ref,
        &definition_fingerprint,
        ContractLifecycleTransitionInputV1 {
            expected_lifecycle_fingerprint: draft.lifecycle_fingerprint().clone(),
            from_state: LifecycleStateV1::Draft,
            to_state: LifecycleStateV1::ReviewReady,
            approval_class: ContractApprovalClassV1::DefinitionAuthor,
            supporting_evidence: vec![LifecycleSupportingEvidenceV1::new(
                LifecycleEvidenceKindV1::DefinitionValidation,
                binding("handbook.contract-lifecycle-evidence.owner@1.0.0", '1'),
            )],
            reason: LifecycleReasonV1::ReadyForReview,
        },
        "owner-recovery-orphan-ready-0001",
    );
    let transitions = harness
        .repo
        .path()
        .join(".handbook/state/contracts/v1/transitions");
    let original = fs::read_dir(&transitions)
        .unwrap()
        .next()
        .unwrap()
        .unwrap()
        .path();
    fs::copy(
        original,
        transitions.join(format!("{}.json", "f".repeat(64))),
    )
    .unwrap();
    assert_eq!(
        ContractOwnerRepositoryV1::open(harness.repo.path())
            .unwrap_err()
            .kind(),
        ContractOwnerRepositoryErrorKindV1::RecoveryAmbiguous
    );
}

#[test]
fn head_replacement_intermediate_recovers_exactly_and_third_state_refuses() {
    for corrupt_displaced in [false, true] {
        let mut harness = Harness::new();
        let definition = definition(&harness.registry_state_fingerprint, "1.0.0");
        let contract_ref = definition.contract_ref().clone();
        let definition_fingerprint = definition.definition_fingerprint().clone();
        let draft = harness
            .append(definition, "owner-head-intermediate-append-0001")
            .unwrap()
            .lifecycle_head()
            .clone();
        let append_transaction =
            transaction_by_operation(harness.repo.path(), "contract.definition.append");
        let old_head = fs::read(append_transaction.join("01-head.json")).unwrap();
        let ready = harness.transition(
            &contract_ref,
            &definition_fingerprint,
            ContractLifecycleTransitionInputV1 {
                expected_lifecycle_fingerprint: draft.lifecycle_fingerprint().clone(),
                from_state: LifecycleStateV1::Draft,
                to_state: LifecycleStateV1::ReviewReady,
                approval_class: ContractApprovalClassV1::DefinitionAuthor,
                supporting_evidence: vec![LifecycleSupportingEvidenceV1::new(
                    LifecycleEvidenceKindV1::DefinitionValidation,
                    binding("handbook.contract-lifecycle-evidence.owner@1.0.0", '1'),
                )],
                reason: LifecycleReasonV1::ReadyForReview,
            },
            "owner-head-intermediate-transition-0001",
        );
        let committed =
            transaction_by_operation(harness.repo.path(), "contract.lifecycle.transition");
        let transaction_id = committed
            .file_name()
            .unwrap()
            .to_str()
            .unwrap()
            .strip_suffix(".committed")
            .unwrap()
            .to_owned();
        let pending = committed.with_file_name(format!("{transaction_id}.pending"));
        fs::rename(&committed, &pending).unwrap();
        let intent: serde_json::Value =
            serde_json::from_slice(&fs::read(pending.join("intent.json")).unwrap()).unwrap();
        let head_ref = intent["outputs"][1]["relative_ref"].as_str().unwrap();
        let head = harness.repo.path().join(Path::new(head_ref));
        let new_head = fs::read(pending.join("01-head.json")).unwrap();
        fs::remove_file(&head).unwrap();
        let name = head.file_name().unwrap().to_str().unwrap();
        let prepared = head.with_file_name(format!(".{name}.{transaction_id}.owner-prepared"));
        let displaced = head.with_file_name(format!(".{name}.{transaction_id}.owner-displaced"));
        fs::write(&prepared, new_head).unwrap();
        fs::write(
            &displaced,
            if corrupt_displaced {
                b"substituted".as_slice()
            } else {
                old_head.as_slice()
            },
        )
        .unwrap();
        let reopened = ContractOwnerRepositoryV1::open(harness.repo.path());
        if corrupt_displaced {
            assert_eq!(
                reopened.unwrap_err().kind(),
                ContractOwnerRepositoryErrorKindV1::RecoveryAmbiguous
            );
        } else {
            assert_eq!(
                reopened
                    .unwrap()
                    .current_lifecycle_head(&contract_ref, &definition_fingerprint)
                    .unwrap(),
                *ready.lifecycle_head()
            );
            assert!(!prepared.exists());
            assert!(!displaced.exists());
        }
    }
}

#[test]
fn exact_absence_substitution_paths_and_concurrent_writers_refuse_or_serialize() {
    assert_eq!(
        ContractOwnerRepositoryV1::open(Path::new("relative-repository"))
            .unwrap_err()
            .kind(),
        ContractOwnerRepositoryErrorKindV1::InvalidRequest
    );

    let mut harness = Harness::new();
    let first_definition = definition(&harness.registry_state_fingerprint, "1.0.0");
    let first_ref = first_definition.contract_ref().clone();
    harness
        .append(first_definition, "owner-concurrent-initial-0001")
        .unwrap();
    let absent_ref = ExactRefV1::parse("handbook.contract.absent@1.0.0").unwrap();
    let absent_fingerprint = FingerprintV1::parse(&format!("sha256:{}", "a".repeat(64))).unwrap();
    assert_eq!(
        harness
            .owner
            .read_definition(&absent_ref, &absent_fingerprint)
            .unwrap(),
        DefinitionReadOutcomeV1::Absent
    );
    assert_eq!(
        harness
            .owner
            .read_definition(&first_ref, &absent_fingerprint)
            .unwrap_err()
            .kind(),
        ContractOwnerRepositoryErrorKindV1::RecordSubstituted
    );

    let owner = Arc::new(ContractOwnerRepositoryV1::open(harness.repo.path()).unwrap());
    let barrier = Arc::new(Barrier::new(3));
    let mut joins = Vec::new();
    for (version, key) in [
        ("1.0.1", "owner-concurrent-key-0001"),
        ("1.0.2", "owner-concurrent-key-0002"),
    ] {
        let definition = definition(&harness.registry_state_fingerprint, version);
        let prepared = prepare_contract_definition_append(definition).unwrap();
        let request = prepared.authority_request();
        let owner = Arc::clone(&owner);
        let barrier = Arc::clone(&barrier);
        let repo = harness.repo.path().to_path_buf();
        joins.push(std::thread::spawn(move || {
            let mut service =
                CredentialAuthorityAdmissionServiceV1::new(repo, TestAuthenticatorPort::new());
            barrier.wait();
            service
                .admit(request, |admission| {
                    owner.append_definition(prepared, admission, key)
                })
                .unwrap()
                .unwrap()
        }));
    }
    barrier.wait();
    for join in joins {
        assert_eq!(
            join.join().unwrap().disposition(),
            OwnerMutationDispositionV1::Committed
        );
    }
    assert_eq!(
        owner
            .list_definitions(DefinitionListRequestV1::first(10).unwrap())
            .unwrap()
            .entries()
            .len(),
        3
    );
}

#[test]
fn cross_process_writers_serialize_without_duplicate_owner_state() {
    let harness = Harness::new();
    let executable = std::env::current_exe().unwrap();
    let mut children = Vec::new();
    for key in ["owner-process-writer-0001", "owner-process-writer-0002"] {
        children.push(
            Command::new(&executable)
                .args([
                    "--ignored",
                    "--exact",
                    "owner_cross_process_helper",
                    "--nocapture",
                ])
                .env("HANDBOOK_OWNER_PROCESS_REPO", harness.repo.path())
                .env("HANDBOOK_OWNER_PROCESS_KEY", key)
                .stdout(std::process::Stdio::piped())
                .stderr(std::process::Stdio::piped())
                .spawn()
                .unwrap(),
        );
    }
    let outcomes = children
        .into_iter()
        .map(|child| {
            let output = child.wait_with_output().unwrap();
            assert!(
                output.status.success(),
                "{}",
                String::from_utf8_lossy(&output.stderr)
            );
            String::from_utf8_lossy(&output.stdout).into_owned()
        })
        .collect::<Vec<_>>();
    assert_eq!(
        outcomes
            .iter()
            .filter(|output| output.contains("OWNER_PROCESS_COMMITTED"))
            .count(),
        1
    );
    assert_eq!(
        outcomes
            .iter()
            .filter(|output| output.contains("OWNER_PROCESS_REFUSED"))
            .count(),
        1
    );
    let reopened = ContractOwnerRepositoryV1::open(harness.repo.path()).unwrap();
    assert_eq!(
        reopened
            .list_definitions(DefinitionListRequestV1::first(10).unwrap())
            .unwrap()
            .entries()
            .len(),
        1
    );
}

#[test]
#[ignore = "spawned by cross_process_writers_serialize_without_duplicate_owner_state"]
fn owner_cross_process_helper() {
    let Some(repo) = std::env::var_os("HANDBOOK_OWNER_PROCESS_REPO") else {
        return;
    };
    let key = std::env::var("HANDBOOK_OWNER_PROCESS_KEY").unwrap();
    let observation = observe_committed_approver_registry(Path::new(&repo)).unwrap();
    let registry_state_fingerprint = observation.state_fingerprint.clone();
    drop(observation);
    let owner = ContractOwnerRepositoryV1::open(Path::new(&repo)).unwrap();
    let mut service =
        CredentialAuthorityAdmissionServiceV1::new(Path::new(&repo), TestAuthenticatorPort::new());
    let prepared =
        prepare_contract_definition_append(definition(&registry_state_fingerprint, "9.9.9"))
            .unwrap();
    let result = service
        .admit(prepared.authority_request(), |admission| {
            owner.append_definition(prepared, admission, &key)
        })
        .unwrap();
    match result {
        Ok(_) => println!("OWNER_PROCESS_COMMITTED"),
        Err(error) => {
            assert_eq!(
                error.kind(),
                ContractOwnerRepositoryErrorKindV1::RecordSubstituted
            );
            println!("OWNER_PROCESS_REFUSED");
        }
    }
}

#[test]
fn owner_layout_refuses_symlinked_ambient_state_where_supported() {
    let repo = tempfile::tempdir().unwrap();
    let target = tempfile::tempdir().unwrap();
    let link = repo.path().join(".handbook");
    #[cfg(windows)]
    if std::os::windows::fs::symlink_dir(target.path(), &link).is_err() {
        return;
    }
    #[cfg(unix)]
    std::os::unix::fs::symlink(target.path(), &link).unwrap();
    assert_eq!(
        ContractOwnerRepositoryV1::open(repo.path())
            .unwrap_err()
            .kind(),
        ContractOwnerRepositoryErrorKindV1::UnsafeFilesystem
    );
}

#[test]
fn durable_owner_records_refuse_hard_link_aliases_where_supported() {
    let mut harness = Harness::new();
    harness
        .append(
            definition(&harness.registry_state_fingerprint, "1.0.0"),
            "owner-hard-link-refusal-0001",
        )
        .unwrap();
    let definitions = harness
        .repo
        .path()
        .join(".handbook/state/contracts/v1/definitions");
    let original = fs::read_dir(&definitions)
        .unwrap()
        .next()
        .unwrap()
        .unwrap()
        .path();
    if fs::hard_link(
        &original,
        definitions.join(format!("{}.json", "f".repeat(64))),
    )
    .is_err()
    {
        return;
    }
    assert!(matches!(
        ContractOwnerRepositoryV1::open(harness.repo.path())
            .unwrap_err()
            .kind(),
        ContractOwnerRepositoryErrorKindV1::UnsafeFilesystem
            | ContractOwnerRepositoryErrorKindV1::RecoveryAmbiguous
    ));
}

#[test]
fn cloned_logical_authority_cannot_mutate_another_physical_repository() {
    let mut owner_harness = Harness::new();
    let cloned_repo = tempfile::tempdir().unwrap();
    copy_tree(
        &owner_harness.repo.path().join(".handbook"),
        &cloned_repo.path().join(".handbook"),
    );
    let mut cloned_authority = CredentialAuthorityAdmissionServiceV1::new(
        cloned_repo.path(),
        TestAuthenticatorPort::new(),
    );
    let definition = definition(&owner_harness.registry_state_fingerprint, "1.0.0");
    let prepared = prepare_contract_definition_append(definition.clone()).unwrap();
    let refusal = cloned_authority
        .admit(prepared.authority_request(), |admission| {
            owner_harness.owner.append_definition(
                prepared,
                admission,
                "owner-cloned-root-refusal-0001",
            )
        })
        .unwrap()
        .unwrap_err();
    assert_eq!(
        refusal.kind(),
        ContractOwnerRepositoryErrorKindV1::UnsafeFilesystem
    );
    assert!(owner_harness
        .owner
        .list_definitions(DefinitionListRequestV1::first(10).unwrap())
        .unwrap()
        .entries()
        .is_empty());
    let appended = owner_harness
        .append(definition, "owner-same-root-success-0001")
        .unwrap();
    let prepared = owner_harness
        .owner
        .prepare_lifecycle_transition(
            appended.contract_ref(),
            appended.definition_fingerprint(),
            ContractLifecycleTransitionInputV1 {
                expected_lifecycle_fingerprint: appended
                    .lifecycle_head()
                    .lifecycle_fingerprint()
                    .clone(),
                from_state: LifecycleStateV1::Draft,
                to_state: LifecycleStateV1::ReviewReady,
                approval_class: ContractApprovalClassV1::DefinitionAuthor,
                supporting_evidence: vec![LifecycleSupportingEvidenceV1::new(
                    LifecycleEvidenceKindV1::DefinitionValidation,
                    binding("handbook.contract-lifecycle-evidence.owner@1.0.0", '1'),
                )],
                reason: LifecycleReasonV1::ReadyForReview,
            },
        )
        .unwrap();
    let refusal = cloned_authority
        .admit(prepared.authority_request(), |admission| {
            owner_harness.owner.transition_lifecycle(
                prepared,
                admission,
                "owner-cloned-root-transition-0001",
            )
        })
        .unwrap()
        .unwrap_err();
    assert_eq!(
        refusal.kind(),
        ContractOwnerRepositoryErrorKindV1::UnsafeFilesystem
    );
    assert_eq!(
        owner_harness
            .owner
            .current_lifecycle_head(appended.contract_ref(), appended.definition_fingerprint())
            .unwrap(),
        *appended.lifecycle_head()
    );
}

#[test]
fn owner_compare_and_write_preserves_head_when_author_attempts_self_lock() {
    let mut harness = Harness::new();
    let definition = definition(&harness.registry_state_fingerprint, "1.0.0");
    let contract_ref = definition.contract_ref().clone();
    let definition_fingerprint = definition.definition_fingerprint().clone();
    let draft = harness
        .append(definition, "owner-self-lock-append-0001")
        .unwrap()
        .lifecycle_head()
        .clone();
    let ready = harness
        .owner
        .prepare_lifecycle_transition(
            &contract_ref,
            &definition_fingerprint,
            ContractLifecycleTransitionInputV1 {
                expected_lifecycle_fingerprint: draft.lifecycle_fingerprint().clone(),
                from_state: LifecycleStateV1::Draft,
                to_state: LifecycleStateV1::ReviewReady,
                approval_class: ContractApprovalClassV1::DefinitionAuthor,
                supporting_evidence: vec![LifecycleSupportingEvidenceV1::new(
                    LifecycleEvidenceKindV1::DefinitionValidation,
                    binding("handbook.contract-lifecycle-evidence.owner@1.0.0", '1'),
                )],
                reason: LifecycleReasonV1::ReadyForReview,
            },
        )
        .unwrap();
    let ready_execution = harness
        .service
        .admit(ready.authority_request(), |admission| {
            harness
                .owner
                .transition_lifecycle(ready, admission, "owner-self-lock-ready-0001")
        })
        .unwrap()
        .unwrap();
    let ready_head = ready_execution.lifecycle_head().clone();
    let lock = harness
        .owner
        .prepare_lifecycle_transition(
            &contract_ref,
            &definition_fingerprint,
            ContractLifecycleTransitionInputV1 {
                expected_lifecycle_fingerprint: ready_head.lifecycle_fingerprint().clone(),
                from_state: LifecycleStateV1::ReviewReady,
                to_state: LifecycleStateV1::Locked,
                approval_class: ContractApprovalClassV1::IndependentLock,
                supporting_evidence: vec![LifecycleSupportingEvidenceV1::new(
                    LifecycleEvidenceKindV1::IndependentReview,
                    binding("handbook.contract-lifecycle-evidence.owner@1.0.0", '2'),
                )],
                reason: LifecycleReasonV1::IndependentlyLocked,
            },
        )
        .unwrap();
    let refusal = harness
        .service
        .admit(lock.authority_request(), |admission| {
            harness
                .owner
                .transition_lifecycle(lock, admission, "owner-self-lock-key-0001")
        })
        .unwrap()
        .unwrap_err();
    assert_eq!(
        refusal.kind(),
        ContractOwnerRepositoryErrorKindV1::LifecycleRefused
    );
    assert_eq!(
        harness
            .owner
            .current_lifecycle_head(&contract_ref, &definition_fingerprint)
            .unwrap(),
        ready_head
    );
}

#[test]
fn evaluation_basis_is_derived_only_from_the_current_reopened_active_chain() {
    let mut harness = Harness::new_with_independent_authority();
    let definition = definition(&harness.registry_state_fingerprint, "1.0.0");
    let contract_ref = definition.contract_ref().clone();
    let definition_fingerprint = definition.definition_fingerprint().clone();
    let draft = harness
        .append(definition, "owner-active-append-0001")
        .unwrap()
        .lifecycle_head()
        .clone();
    let ready = harness.transition(
        &contract_ref,
        &definition_fingerprint,
        ContractLifecycleTransitionInputV1 {
            expected_lifecycle_fingerprint: draft.lifecycle_fingerprint().clone(),
            from_state: LifecycleStateV1::Draft,
            to_state: LifecycleStateV1::ReviewReady,
            approval_class: ContractApprovalClassV1::DefinitionAuthor,
            supporting_evidence: vec![LifecycleSupportingEvidenceV1::new(
                LifecycleEvidenceKindV1::DefinitionValidation,
                binding("handbook.contract-lifecycle-evidence.owner@1.0.0", '1'),
            )],
            reason: LifecycleReasonV1::ReadyForReview,
        },
        "owner-active-ready-0001",
    );
    let locked = harness.transition(
        &contract_ref,
        &definition_fingerprint,
        ContractLifecycleTransitionInputV1 {
            expected_lifecycle_fingerprint: ready.lifecycle_head().lifecycle_fingerprint().clone(),
            from_state: LifecycleStateV1::ReviewReady,
            to_state: LifecycleStateV1::Locked,
            approval_class: ContractApprovalClassV1::IndependentLock,
            supporting_evidence: vec![LifecycleSupportingEvidenceV1::new(
                LifecycleEvidenceKindV1::IndependentReview,
                binding("handbook.contract-lifecycle-evidence.owner@1.0.0", '2'),
            )],
            reason: LifecycleReasonV1::IndependentlyLocked,
        },
        "owner-active-lock-0001",
    );
    let active = harness.transition(
        &contract_ref,
        &definition_fingerprint,
        ContractLifecycleTransitionInputV1 {
            expected_lifecycle_fingerprint: locked.lifecycle_head().lifecycle_fingerprint().clone(),
            from_state: LifecycleStateV1::Locked,
            to_state: LifecycleStateV1::Active,
            approval_class: ContractApprovalClassV1::Activation,
            supporting_evidence: vec![
                LifecycleSupportingEvidenceV1::new(
                    LifecycleEvidenceKindV1::ApplicabilityClosure,
                    binding("handbook.contract-lifecycle-evidence.owner@1.0.0", '3'),
                ),
                LifecycleSupportingEvidenceV1::new(
                    LifecycleEvidenceKindV1::MatcherClosure,
                    binding("handbook.contract-lifecycle-evidence.owner@1.0.0", '4'),
                ),
                LifecycleSupportingEvidenceV1::new(
                    LifecycleEvidenceKindV1::PolicyClosure,
                    binding("handbook.contract-lifecycle-evidence.owner@1.0.0", '5'),
                ),
                LifecycleSupportingEvidenceV1::new(
                    LifecycleEvidenceKindV1::CurrentnessClosure,
                    binding("handbook.contract-lifecycle-evidence.owner@1.0.0", '6'),
                ),
            ],
            reason: LifecycleReasonV1::Activated,
        },
        "owner-active-activate-0001",
    );
    let current = active.lifecycle_head().lifecycle_fingerprint().clone();
    let basis = harness
        .owner
        .evaluation_lifecycle_basis(&contract_ref, &definition_fingerprint, &current)
        .unwrap();
    assert_eq!(basis.active_resulting_lifecycle_fingerprint(), &current);
    assert_eq!(
        harness
            .owner
            .evaluation_lifecycle_basis(
                &contract_ref,
                &definition_fingerprint,
                locked.lifecycle_head().lifecycle_fingerprint(),
            )
            .unwrap_err()
            .kind(),
        ContractOwnerRepositoryErrorKindV1::LifecycleRefused
    );
    let reopened = ContractOwnerRepositoryV1::open(harness.repo.path()).unwrap();
    assert_eq!(
        reopened
            .evaluation_lifecycle_basis(&contract_ref, &definition_fingerprint, &current)
            .unwrap()
            .basis_fingerprint(),
        basis.basis_fingerprint()
    );
}

#[test]
fn evaluation_run_is_atomically_prepared_from_exact_current_owner_truth() {
    let mut harness = Harness::new_with_independent_authority();
    let freshness = EvidenceFreshnessPolicyV1::new(
        EvidenceFreshnessPolicyInputV1::new(
            ContractIdV1::parse("handbook.contract-evidence-freshness-policy.fixture").unwrap(),
            VersionV1::parse("1.0.0+exact").unwrap(),
            300,
            true,
            0,
        )
        .unwrap(),
    )
    .unwrap();
    let gate = GatePolicyV1::new(
        GatePolicyInputV1::new(
            ContractIdV1::parse("handbook.contract-gate-policy.fixture").unwrap(),
            VersionV1::parse("1.0.0").unwrap(),
            GatePolicyAlgorithmV1::ClosedAccounting,
            None,
            handbook_contracts::local_closeout_policy_binding_v1().unwrap(),
        )
        .unwrap(),
    )
    .unwrap();
    let mut policies = EvaluationPolicyRegistryV1::new().unwrap();
    policies.register_freshness(freshness.clone()).unwrap();
    policies.register_gate(gate.clone()).unwrap();
    let substituted_freshness = EvidenceFreshnessPolicyV1::new(
        EvidenceFreshnessPolicyInputV1::new(
            ContractIdV1::parse("handbook.contract-evidence-freshness-policy.substitute").unwrap(),
            VersionV1::parse("1.0.0").unwrap(),
            600,
            true,
            0,
        )
        .unwrap(),
    )
    .unwrap();
    policies
        .register_freshness(substituted_freshness.clone())
        .unwrap();
    let substituted_gate = GatePolicyV1::new(
        GatePolicyInputV1::new(
            ContractIdV1::parse("handbook.contract-gate-policy.substitute").unwrap(),
            VersionV1::parse("1.0.0").unwrap(),
            GatePolicyAlgorithmV1::ClosedAccounting,
            None,
            handbook_contracts::local_closeout_policy_binding_v1().unwrap(),
        )
        .unwrap(),
    )
    .unwrap();
    policies.register_gate(substituted_gate.clone()).unwrap();

    let definition = evaluation_definition(
        &harness.registry_state_fingerprint,
        freshness.binding().clone(),
        gate.binding().clone(),
    );
    let contract_ref = definition.contract_ref().clone();
    let definition_fingerprint = definition.definition_fingerprint().clone();
    let draft = harness
        .append(definition, "owner-run-append-0001")
        .unwrap()
        .lifecycle_head()
        .clone();
    let ready = harness.transition(
        &contract_ref,
        &definition_fingerprint,
        ContractLifecycleTransitionInputV1 {
            expected_lifecycle_fingerprint: draft.lifecycle_fingerprint().clone(),
            from_state: LifecycleStateV1::Draft,
            to_state: LifecycleStateV1::ReviewReady,
            approval_class: ContractApprovalClassV1::DefinitionAuthor,
            supporting_evidence: vec![LifecycleSupportingEvidenceV1::new(
                LifecycleEvidenceKindV1::DefinitionValidation,
                binding("handbook.contract-lifecycle-evidence.owner@1.0.0", '1'),
            )],
            reason: LifecycleReasonV1::ReadyForReview,
        },
        "owner-run-ready-0001",
    );
    let locked = harness.transition(
        &contract_ref,
        &definition_fingerprint,
        ContractLifecycleTransitionInputV1 {
            expected_lifecycle_fingerprint: ready.lifecycle_head().lifecycle_fingerprint().clone(),
            from_state: LifecycleStateV1::ReviewReady,
            to_state: LifecycleStateV1::Locked,
            approval_class: ContractApprovalClassV1::IndependentLock,
            supporting_evidence: vec![LifecycleSupportingEvidenceV1::new(
                LifecycleEvidenceKindV1::IndependentReview,
                binding("handbook.contract-lifecycle-evidence.owner@1.0.0", '2'),
            )],
            reason: LifecycleReasonV1::IndependentlyLocked,
        },
        "owner-run-lock-0001",
    );
    let active = harness.transition(
        &contract_ref,
        &definition_fingerprint,
        ContractLifecycleTransitionInputV1 {
            expected_lifecycle_fingerprint: locked.lifecycle_head().lifecycle_fingerprint().clone(),
            from_state: LifecycleStateV1::Locked,
            to_state: LifecycleStateV1::Active,
            approval_class: ContractApprovalClassV1::Activation,
            supporting_evidence: vec![
                LifecycleSupportingEvidenceV1::new(
                    LifecycleEvidenceKindV1::ApplicabilityClosure,
                    binding("handbook.contract-lifecycle-evidence.owner@1.0.0", '3'),
                ),
                LifecycleSupportingEvidenceV1::new(
                    LifecycleEvidenceKindV1::MatcherClosure,
                    binding("handbook.contract-lifecycle-evidence.owner@1.0.0", '4'),
                ),
                LifecycleSupportingEvidenceV1::new(
                    LifecycleEvidenceKindV1::PolicyClosure,
                    binding("handbook.contract-lifecycle-evidence.owner@1.0.0", '5'),
                ),
                LifecycleSupportingEvidenceV1::new(
                    LifecycleEvidenceKindV1::CurrentnessClosure,
                    binding("handbook.contract-lifecycle-evidence.owner@1.0.0", '6'),
                ),
            ],
            reason: LifecycleReasonV1::Activated,
        },
        "owner-run-active-0001",
    );
    let active_head = active.lifecycle_head().lifecycle_fingerprint().clone();
    let make_input = |freshness_policy: ExactBindingV1, gate_policy: ExactBindingV1| {
        PrepareEvaluationRunInputV1::new(
            ExactBindingV1::new(contract_ref.clone(), definition_fingerprint.clone()),
            active_head.clone(),
            evaluation_subject('7'),
            vec![evaluation_case('8')],
            evaluation_resolution(),
            MatcherBindingV1::exact_fields_v1(),
            vec![freshness_policy],
            vec![handbook_contracts::evidence_stability_policy_binding_v1().unwrap()],
            gate_policy,
            handbook_contracts::local_closeout_policy_binding_v1().unwrap(),
            EvaluationRunNonceV1::parse("owner-observation-0001").unwrap(),
        )
        .unwrap()
    };

    let run = harness
        .owner
        .prepare_evaluation_run_v1(
            make_input(freshness.binding().clone(), gate.binding().clone()),
            &policies,
        )
        .unwrap();
    assert_eq!(run.claims().len(), 1);
    assert_eq!(run.cases()[0].case_id().as_str(), "case.positive");
    assert!(run
        .evaluation_run_ref()
        .starts_with(&format!("{contract_ref}#evaluation/")));
    assert_eq!(
        run.owner_currentness().lifecycle_head_fingerprint(),
        &active_head
    );
    assert_eq!(
        run.owner_currentness().semantic_time_utc().as_str().len(),
        20
    );
    let bytes = run.canonical_json_bytes().unwrap();
    assert_eq!(
        serde_json::from_slice::<handbook_contracts::ContractEvaluationRunV1>(&bytes).unwrap(),
        run
    );
    let mut unknown: serde_json::Value = serde_json::from_slice(&bytes).unwrap();
    unknown["parent_promotion_eligible"] = serde_json::Value::Bool(true);
    assert!(
        serde_json::from_value::<handbook_contracts::ContractEvaluationRunV1>(unknown).is_err()
    );
    assert_eq!(
        harness
            .owner
            .prepare_evaluation_run_v1(
                make_input(
                    substituted_freshness.binding().clone(),
                    gate.binding().clone(),
                ),
                &policies,
            )
            .unwrap_err()
            .kind(),
        ContractOwnerRepositoryErrorKindV1::InvalidRequest
    );
    assert_eq!(
        harness
            .owner
            .prepare_evaluation_run_v1(
                make_input(
                    freshness.binding().clone(),
                    substituted_gate.binding().clone(),
                ),
                &policies,
            )
            .unwrap_err()
            .kind(),
        ContractOwnerRepositoryErrorKindV1::InvalidRequest
    );

    harness.transition(
        &contract_ref,
        &definition_fingerprint,
        ContractLifecycleTransitionInputV1 {
            expected_lifecycle_fingerprint: active_head.clone(),
            from_state: LifecycleStateV1::Active,
            to_state: LifecycleStateV1::Deprecated,
            approval_class: ContractApprovalClassV1::Lifecycle,
            supporting_evidence: vec![LifecycleSupportingEvidenceV1::new(
                LifecycleEvidenceKindV1::NoReplacementRationale,
                binding("handbook.contract-lifecycle-evidence.owner@1.0.0", '9'),
            )],
            reason: LifecycleReasonV1::DeprecatedWithoutReplacement(
                LifecycleReasonTextV1::parse("No exact replacement is currently authorized.")
                    .unwrap(),
            ),
        },
        "owner-run-deprecate-0001",
    );
    assert_eq!(
        harness
            .owner
            .prepare_evaluation_run_v1(
                make_input(freshness.binding().clone(), gate.binding().clone()),
                &policies,
            )
            .unwrap_err()
            .kind(),
        ContractOwnerRepositoryErrorKindV1::LifecycleRefused
    );
}

fn only_transaction(repo: &Path) -> PathBuf {
    let root = repo.join(".handbook/state/contracts/v1/transactions");
    let mut entries = fs::read_dir(root)
        .unwrap()
        .map(|entry| entry.unwrap().path())
        .collect::<Vec<_>>();
    entries.sort();
    assert_eq!(entries.len(), 1);
    entries.pop().unwrap()
}

fn transaction_by_operation(repo: &Path, operation: &str) -> PathBuf {
    fs::read_dir(repo.join(".handbook/state/contracts/v1/transactions"))
        .unwrap()
        .map(|entry| entry.unwrap().path())
        .find(|path| {
            fs::read(path.join("intent.json"))
                .ok()
                .and_then(|bytes| serde_json::from_slice::<serde_json::Value>(&bytes).ok())
                .and_then(|intent| intent["operation"].as_str().map(str::to_owned))
                .as_deref()
                == Some(operation)
        })
        .unwrap()
}

fn reopen_as_pending(repo: &Path) -> PathBuf {
    let committed = only_transaction(repo);
    let name = committed.file_name().unwrap().to_str().unwrap();
    let pending = committed.with_file_name(name.replace(".committed", ".pending"));
    fs::rename(committed, &pending).unwrap();
    pending
}

fn staged_paths(transaction: &Path) -> Vec<PathBuf> {
    let mut paths = fs::read_dir(transaction)
        .unwrap()
        .map(|entry| entry.unwrap().path())
        .filter(|path| {
            path.file_name()
                .and_then(|name| name.to_str())
                .is_some_and(|name| {
                    name.starts_with("00-") || name.starts_with("01-") || name.starts_with("02-")
                })
        })
        .collect::<Vec<_>>();
    paths.sort();
    paths
}

fn transaction_outputs(repo: &Path, transaction: &Path) -> Vec<PathBuf> {
    let intent: serde_json::Value =
        serde_json::from_slice(&fs::read(transaction.join("intent.json")).unwrap()).unwrap();
    intent["outputs"]
        .as_array()
        .unwrap()
        .iter()
        .map(|output| repo.join(output["relative_ref"].as_str().unwrap()))
        .collect()
}

fn remove_installed_suffix(repo: &Path, transaction: &Path, prefix: usize) {
    for output in transaction_outputs(repo, transaction)
        .into_iter()
        .skip(prefix)
    {
        fs::remove_file(output).unwrap();
    }
}

struct TestAuthenticatorPort {
    author_key: SigningKey,
    authority_key: SigningKey,
    author_credential_id: Vec<u8>,
    authority_credential_id: Vec<u8>,
    make_calls: usize,
    author_assertions: u32,
    authority_assertions: u32,
}

impl TestAuthenticatorPort {
    fn new() -> Self {
        Self {
            author_key: SigningKey::from_slice(&[7_u8; 32]).unwrap(),
            authority_key: SigningKey::from_slice(&[6_u8; 32]).unwrap(),
            author_credential_id: vec![0x32; 32],
            authority_credential_id: vec![0x31; 32],
            make_calls: 0,
            author_assertions: 0,
            authority_assertions: 0,
        }
    }
}

impl NativeAuthenticatorPortV1 for TestAuthenticatorPort {
    fn make_credential(
        &mut self,
        _request_cbor: &[u8],
    ) -> Result<Vec<u8>, NativeAuthenticatorPortErrorV1> {
        self.make_calls += 1;
        if self.make_calls == 1 {
            Ok(make_credential_response(
                &self.author_key,
                &self.author_credential_id,
            ))
        } else {
            Ok(make_credential_response(
                &self.authority_key,
                &self.authority_credential_id,
            ))
        }
    }

    fn get_assertion(
        &mut self,
        request_cbor: &[u8],
    ) -> Result<Vec<u8>, NativeAuthenticatorPortErrorV1> {
        let marker = request_cbor
            .windows(3)
            .position(|window| window == [0x02, 0x58, 0x20])
            .unwrap();
        let client_data_hash = request_cbor[marker + 3..marker + 35].try_into().unwrap();
        if request_cbor
            .windows(self.authority_credential_id.len())
            .any(|window| window == self.authority_credential_id)
        {
            self.authority_assertions += 1;
            Ok(assertion_response(
                &self.authority_key,
                &self.authority_credential_id,
                client_data_hash,
                self.authority_assertions,
            ))
        } else {
            self.author_assertions += 1;
            Ok(assertion_response(
                &self.author_key,
                &self.author_credential_id,
                client_data_hash,
                self.author_assertions,
            ))
        }
    }
}

struct Harness {
    repo: tempfile::TempDir,
    owner: ContractOwnerRepositoryV1,
    service: CredentialAuthorityAdmissionServiceV1<TestAuthenticatorPort>,
    registry_state_fingerprint: String,
}

impl Harness {
    fn new() -> Self {
        let repo = tempfile::tempdir().unwrap();
        let mut registry =
            ApproverRegistryServiceV1::for_repository(repo.path(), TestAuthenticatorPort::new());
        let request = ApproverAdminRequestV1::from_json_value(json!({
            "schema_id":"handbook.approver-admin-request",
            "schema_version":"1.0",
            "operation":"bootstrap",
            "operation_id":"hcm-5-1-owner-bootstrap",
            "repository_identity_fingerprint":REPOSITORY_IDENTITY,
            "expected_registry_state_fingerprint":null,
            "expected_transition_fingerprint":null,
            "initial_charter_quorum":[
                {
                    "approval_class":"contract_definition_author",
                    "authority_ref":AUTHOR_AUTHORITY_REF
                },
                {
                    "approval_class":"contract_independent_lock",
                    "authority_ref":"handbook.authority.contract-independent-lock@1.0.0"
                },
                {
                    "approval_class":"contract_activation",
                    "authority_ref":"handbook.authority.contract-activation@1.0.0"
                },
                {
                    "approval_class":"contract_lifecycle",
                    "authority_ref":"handbook.authority.contract-lifecycle@1.0.0"
                }
            ]
        }))
        .unwrap();
        let result = registry
            .bootstrap_approver_registry(request)
            .to_json_value()
            .unwrap();
        assert_eq!(result["status"], "succeeded", "{result:#}");
        let observation = observe_committed_approver_registry(repo.path()).unwrap();
        let registry_state_fingerprint = observation.state_fingerprint.clone();
        drop(observation);
        let service = CredentialAuthorityAdmissionServiceV1::new(repo.path(), registry.into_port());
        let owner = ContractOwnerRepositoryV1::open(repo.path()).unwrap();
        Self {
            repo,
            owner,
            service,
            registry_state_fingerprint,
        }
    }

    fn new_with_independent_authority() -> Self {
        let repo = tempfile::tempdir().unwrap();
        let mut registry =
            ApproverRegistryServiceV1::for_repository(repo.path(), TestAuthenticatorPort::new());
        let bootstrap = ApproverAdminRequestV1::from_json_value(json!({
            "schema_id":"handbook.approver-admin-request",
            "schema_version":"1.0",
            "operation":"bootstrap",
            "operation_id":"hcm-5-1-owner-independent-bootstrap",
            "repository_identity_fingerprint":REPOSITORY_IDENTITY,
            "expected_registry_state_fingerprint":null,
            "expected_transition_fingerprint":null,
            "initial_charter_quorum":[{
                "approval_class":"contract_definition_author",
                "authority_ref":AUTHOR_AUTHORITY_REF
            }]
        }))
        .unwrap();
        let bootstrapped = registry
            .bootstrap_approver_registry(bootstrap)
            .to_json_value()
            .unwrap();
        assert_eq!(bootstrapped["status"], "succeeded", "{bootstrapped:#}");
        let add = ApproverAdminRequestV1::from_json_value(json!({
            "schema_id":"handbook.approver-admin-request",
            "schema_version":"1.0",
            "operation":"add_credential",
            "operation_id":"hcm-5-1-owner-independent-add",
            "repository_identity_fingerprint":REPOSITORY_IDENTITY,
            "expected_registry_state_fingerprint":bootstrapped["result_registry_state_fingerprint"],
            "expected_transition_fingerprint":bootstrapped["transition_fingerprint"],
            "approval_mappings":[
                {
                    "approval_class":"contract_activation",
                    "authority_ref":"handbook.authority.contract-activation@1.0.0"
                },
                {
                    "approval_class":"contract_independent_lock",
                    "authority_ref":"handbook.authority.contract-independent-lock@1.0.0"
                },
                {
                    "approval_class":"contract_lifecycle",
                    "authority_ref":"handbook.authority.contract-lifecycle@1.0.0"
                }
            ]
        }))
        .unwrap();
        let added = registry
            .add_approver_credential(add)
            .to_json_value()
            .unwrap();
        assert_eq!(added["status"], "succeeded", "{added:#}");
        let observation = observe_committed_approver_registry(repo.path()).unwrap();
        let registry_state_fingerprint = observation.state_fingerprint.clone();
        drop(observation);
        let service = CredentialAuthorityAdmissionServiceV1::new(repo.path(), registry.into_port());
        let owner = ContractOwnerRepositoryV1::open(repo.path()).unwrap();
        Self {
            repo,
            owner,
            service,
            registry_state_fingerprint,
        }
    }

    fn append(
        &mut self,
        definition: ContractDefinitionV1,
        key: &str,
    ) -> Result<
        handbook_contracts::DefinitionAppendExecutionV1,
        handbook_contracts::ContractOwnerRepositoryErrorV1,
    > {
        let prepared = prepare_contract_definition_append(definition).unwrap();
        self.service
            .admit(prepared.authority_request(), |admission| {
                self.owner.append_definition(prepared, admission, key)
            })
            .unwrap()
    }

    fn transition(
        &mut self,
        contract_ref: &ExactRefV1,
        definition_fingerprint: &FingerprintV1,
        input: ContractLifecycleTransitionInputV1,
        key: &str,
    ) -> handbook_contracts::LifecycleTransitionExecutionV1 {
        let prepared = self
            .owner
            .prepare_lifecycle_transition(contract_ref, definition_fingerprint, input)
            .unwrap();
        self.service
            .admit(prepared.authority_request(), |admission| {
                self.owner.transition_lifecycle(prepared, admission, key)
            })
            .unwrap()
            .unwrap()
    }
}

fn definition(registry_state_fingerprint: &str, version: &str) -> ContractDefinitionV1 {
    ContractDefinitionV1::new(ContractDefinitionInputV1 {
        definition_schema: binding("handbook.contract-definition@1.0.0", 'b'),
        contract_id: ContractIdV1::parse("handbook.contract.example").unwrap(),
        contract_version: VersionV1::parse(version).unwrap(),
        contract_kind: ContractKindV1::ClaimSet,
        claims: vec![claim()],
        definition_author_ref: StableIdV1::parse(&format!(
            "credential.sha256-{:x}",
            Sha256::digest([0x32; 32])
        ))
        .unwrap(),
        definition_authority: ExactBindingV1::new(
            ExactRefV1::parse(AUTHOR_AUTHORITY_REF).unwrap(),
            FingerprintV1::parse(registry_state_fingerprint).unwrap(),
        ),
        lifecycle_policy: lifecycle_policy_binding_v1().unwrap(),
        matcher_bindings: vec![MatcherBindingV1::exact_fields_v1()],
        gate_policy: binding("handbook.contract-gate-policy.closed-v1@1.0.0", 'c'),
        extensions_policy: binding("handbook.contract-extensions.closed-v1@1.0.0", 'd'),
    })
    .unwrap()
}

fn evaluation_definition(
    registry_state_fingerprint: &str,
    freshness_policy: ExactBindingV1,
    gate_policy: ExactBindingV1,
) -> ContractDefinitionV1 {
    ContractDefinitionV1::new(ContractDefinitionInputV1 {
        definition_schema: binding("handbook.contract-definition@1.0.0", 'b'),
        contract_id: ContractIdV1::parse("handbook.contract.evaluation-example").unwrap(),
        contract_version: VersionV1::parse("1.0.0").unwrap(),
        contract_kind: ContractKindV1::ClaimSet,
        claims: vec![ContractClaimV1::new(ContractClaimInputV1 {
            claim_id: id("schema.payload.valid"),
            statement: "Payload conforms to its exact schema.".to_owned(),
            subject_selector: selector("subject.kind", "artifact"),
            case_selector: selector("case.kind", "positive"),
            applicability_rule: ApplicabilityRuleV1::SubjectAndCaseMatch,
            gate_effect: GateEffectV1::Required,
            evidence_requirements: vec![EvidenceRequirementV1::new(
                id("handbook.evidence.schema-conformance"),
                EvidenceCardinalityV1::ExactlyOne,
                EvidenceCaseSelectionV1::ClaimCaseSelector,
                handbook_contracts::evidence_stability_policy_binding_v1().unwrap(),
            )
            .unwrap()],
            freshness_policy,
            minimum_resolution: ResolutionMinimumV1::new(
                binding("handbook.context-resolution.shipped-root@1.0.0", '1'),
                id("resolution.scope.local-observation"),
                id("resolution.detail.identifier-only"),
                id("resolution.time.current-operation"),
                id("resolution.authority.read-only"),
                id("resolution.memory.operation"),
                id("resolution.validation.observation-only"),
            ),
            score_weight: None,
        })
        .unwrap()],
        definition_author_ref: StableIdV1::parse(&format!(
            "credential.sha256-{:x}",
            Sha256::digest([0x32; 32])
        ))
        .unwrap(),
        definition_authority: ExactBindingV1::new(
            ExactRefV1::parse(AUTHOR_AUTHORITY_REF).unwrap(),
            FingerprintV1::parse(registry_state_fingerprint).unwrap(),
        ),
        lifecycle_policy: lifecycle_policy_binding_v1().unwrap(),
        matcher_bindings: vec![MatcherBindingV1::exact_fields_v1()],
        gate_policy,
        extensions_policy: binding("handbook.contract-extensions.closed-v1@1.0.0", 'd'),
    })
    .unwrap()
}

fn evaluation_subject(hex: char) -> EvaluationSubjectV1 {
    let fingerprint =
        FingerprintV1::parse(&format!("sha256:{}", hex.to_string().repeat(64))).unwrap();
    EvaluationSubjectV1::new(
        handbook_contracts::ContentAddressedBindingV1::new(
            &format!("subject.example/{}", &fingerprint.as_str()[7..]),
            fingerprint,
        )
        .unwrap(),
        vec![MatcherInputValueV1::new(id("subject.kind"), "artifact").unwrap()],
    )
    .unwrap()
}

fn evaluation_case(hex: char) -> EvaluationCaseV1 {
    let fingerprint =
        FingerprintV1::parse(&format!("sha256:{}", hex.to_string().repeat(64))).unwrap();
    EvaluationCaseV1::new(
        id("case.positive"),
        handbook_contracts::ContentAddressedBindingV1::new(
            &format!("case.example/{}", &fingerprint.as_str()[7..]),
            fingerprint,
        )
        .unwrap(),
        vec![MatcherInputValueV1::new(id("case.kind"), "positive").unwrap()],
    )
    .unwrap()
}

fn evaluation_resolution() -> ResolutionEnvelopeV1 {
    ResolutionEnvelopeV1::new(
        binding("handbook.context-resolution.shipped-root@1.0.0", '1'),
        id("resolution.scope.local-observation"),
        id("resolution.detail.identifier-only"),
        id("resolution.time.current-operation"),
        id("resolution.authority.read-only"),
        id("resolution.memory.operation"),
        id("resolution.validation.observation-only"),
    )
}

fn claim() -> ContractClaimV1 {
    ContractClaimV1::new(ContractClaimInputV1 {
        claim_id: id("schema.payload.valid"),
        statement: "Payload conforms to its exact schema.".to_owned(),
        subject_selector: selector("subject.kind", "artifact"),
        case_selector: selector("case.kind", "positive"),
        applicability_rule: ApplicabilityRuleV1::SubjectAndCaseMatch,
        gate_effect: GateEffectV1::Required,
        evidence_requirements: vec![EvidenceRequirementV1::new(
            id("handbook.evidence.schema-conformance"),
            EvidenceCardinalityV1::ExactlyOne,
            EvidenceCaseSelectionV1::ClaimCaseSelector,
            binding("handbook.stability.exact@1.0.0", 'e'),
        )
        .unwrap()],
        freshness_policy: binding("handbook.freshness.exact-revision@1.0.0", 'f'),
        minimum_resolution: ResolutionMinimumV1::new(
            binding("handbook.context-resolution.shipped-root@1.0.0", '1'),
            id("resolution.scope.local-observation"),
            id("resolution.detail.identifier-only"),
            id("resolution.time.current-operation"),
            id("resolution.authority.read-only"),
            id("resolution.memory.operation"),
            id("resolution.validation.observation-only"),
        ),
        score_weight: None,
    })
    .unwrap()
}

fn selector(field: &str, expected: &str) -> SelectorV1 {
    SelectorV1::new(
        MatcherBindingV1::exact_fields_v1(),
        vec![SelectorCriterionV1::new(id(field), expected).unwrap()],
    )
    .unwrap()
}

fn id(value: &str) -> StableIdV1 {
    StableIdV1::parse(value).unwrap()
}

fn binding(value: &str, hex: char) -> ExactBindingV1 {
    ExactBindingV1::new(
        ExactRefV1::parse(value).unwrap(),
        FingerprintV1::parse(&format!("sha256:{}", hex.to_string().repeat(64))).unwrap(),
    )
}

fn all_persisted_bytes(root: &std::path::Path) -> Vec<u8> {
    let mut output = Vec::new();
    let mut pending = vec![root.to_path_buf()];
    while let Some(path) = pending.pop() {
        if path.is_dir() {
            pending.extend(
                std::fs::read_dir(path)
                    .unwrap()
                    .map(|entry| entry.unwrap().path()),
            );
        } else {
            output.extend(std::fs::read(path).unwrap());
        }
    }
    output
}

fn copy_tree(source: &Path, target: &Path) {
    fs::create_dir(target).unwrap();
    for entry in fs::read_dir(source).unwrap() {
        let entry = entry.unwrap();
        let destination = target.join(entry.file_name());
        if entry.file_type().unwrap().is_dir() {
            copy_tree(&entry.path(), &destination);
        } else {
            fs::copy(entry.path(), destination).unwrap();
        }
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
    let der = signature.to_der();
    let mut response = vec![0, 0xa3, 0x01, 0xa2];
    cbor_text(&mut response, "id");
    cbor_bytes(&mut response, credential_id);
    cbor_text(&mut response, "type");
    cbor_text(&mut response, "public-key");
    response.push(0x02);
    cbor_bytes(&mut response, &authenticator_data);
    response.push(0x03);
    cbor_bytes(&mut response, der.as_bytes());
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
