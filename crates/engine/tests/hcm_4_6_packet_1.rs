use handbook_engine::{
    parse_schema_json, DefinitionFingerprint, SnapshotAdapterFailureOutcomeV1,
    SnapshotGroundingErrorClassV1, SnapshotGroundingErrorKindV1, SnapshotGroundingMutationEffectV1,
    SnapshotGroundingOwnerV1, SnapshotSourceAccountingDispositionV1, SnapshotSourceCurrentnessV1,
    SnapshotSourceFamilyV1, SnapshotSourceReadFloorV1,
};
use tempfile::TempDir;

const WORKSPACE_REF: &str = "handbook.workspace.test@1.0.0";
const WORKSPACE_FINGERPRINT: &str =
    "sha256:1111111111111111111111111111111111111111111111111111111111111111";

#[test]
fn production_sources_have_exact_independent_identities_and_derived_fingerprints() {
    let owner = SnapshotGroundingOwnerV1::new();
    let sources = owner
        .production_sources()
        .expect("admitted production sources");

    assert_eq!(
        sources.policy().exact_ref(),
        "handbook.snapshot-policy.production-session-boundary@1.0.0"
    );
    assert_eq!(
        sources.grounding_definition().exact_ref(),
        "handbook.grounding.summary.production-session-boundary@1.0.0"
    );
    assert_eq!(
        sources.disclosure_policy().exact_ref(),
        "handbook.grounding.disclosure.production-session-boundary@1.0.0"
    );

    for source in [
        sources.policy(),
        sources.grounding_definition(),
        sources.disclosure_policy(),
    ] {
        let value = parse_schema_json(source.canonical_bytes()).expect("canonical production JSON");
        let derived = DefinitionFingerprint::from_json_value(&value)
            .expect("production source fingerprint derives from its own canonical bytes");
        assert_eq!(source.fingerprint(), derived.as_str());
    }

    assert_ne!(
        sources.policy().fingerprint(),
        sources.grounding_definition().fingerprint()
    );
    assert_ne!(
        sources.policy().fingerprint(),
        sources.disclosure_policy().fingerprint()
    );
    assert_ne!(
        sources.grounding_definition().fingerprint(),
        sources.disclosure_policy().fingerprint()
    );
}

#[test]
fn owner_closes_the_five_precursor_families_exact_slots_and_private_adapter_contracts() {
    let contracts = SnapshotGroundingOwnerV1::new().source_contracts();
    assert_eq!(contracts.len(), 5);
    assert_eq!(
        contracts
            .iter()
            .map(|contract| contract.family())
            .collect::<Vec<_>>(),
        SnapshotSourceFamilyV1::ALL
    );

    let expected = [
        (
            SnapshotSourceFamilyV1::Evidence,
            "EvidenceSnapshotSourceV1",
            "handbook.snapshot-source.evidence@1.0.0",
            &[
                "contracts_definition_lifecycle",
                "contracts_evidence_root",
                "contracts_decision",
                "dock_manifest_bundle",
                "dock_execution_candidate",
            ][..],
            SnapshotSourceCurrentnessV1::ContractsAndDockOwnerHeads,
            &[
                SnapshotAdapterFailureOutcomeV1::AccountedOmission,
                SnapshotAdapterFailureOutcomeV1::Refused,
                SnapshotAdapterFailureOutcomeV1::Unavailable,
            ][..],
        ),
        (
            SnapshotSourceFamilyV1::Git,
            "GitSnapshotSourceV1",
            "handbook.snapshot-source.git@1.0.0",
            &["head", "index", "worktree"][..],
            SnapshotSourceCurrentnessV1::RepositoryTripletStable,
            &[
                SnapshotAdapterFailureOutcomeV1::AccountedOmission,
                SnapshotAdapterFailureOutcomeV1::Unavailable,
            ][..],
        ),
        (
            SnapshotSourceFamilyV1::Handbook,
            "HandbookSnapshotSourceV1",
            "handbook.snapshot-source.handbook@1.0.0",
            &["engine_authority", "profile", "schema_artifact_resolution"][..],
            SnapshotSourceCurrentnessV1::HandbookAuthorityHeadStable,
            &[
                SnapshotAdapterFailureOutcomeV1::AccountedOmission,
                SnapshotAdapterFailureOutcomeV1::Refused,
            ][..],
        ),
        (
            SnapshotSourceFamilyV1::Session,
            "SessionBoundarySourceV1",
            "handbook.snapshot-source.session@1.0.0",
            &["boundary_context"][..],
            SnapshotSourceCurrentnessV1::RequestAndStreamHeadLocked,
            &[
                SnapshotAdapterFailureOutcomeV1::Refused,
                SnapshotAdapterFailureOutcomeV1::ValidNoApplicablePrior,
            ][..],
        ),
        (
            SnapshotSourceFamilyV1::Work,
            "WorkSnapshotSourceV1",
            "handbook.snapshot-source.work@1.0.0",
            &["flow_state", "pipeline_state", "handoff"][..],
            SnapshotSourceCurrentnessV1::FlowPipelineHeadsStable,
            &[
                SnapshotAdapterFailureOutcomeV1::AccountedOmission,
                SnapshotAdapterFailureOutcomeV1::Unavailable,
            ][..],
        ),
    ];

    for (contract, (family, adapter, adapter_ref, slots, currentness, failures)) in
        contracts.iter().zip(expected)
    {
        assert_eq!(contract.family(), family);
        assert_eq!(contract.adapter_type_name(), adapter);
        assert_eq!(contract.adapter_ref(), adapter_ref);
        assert_eq!(contract.source_slots(), slots);
        assert_eq!(contract.currentness(), currentness);
        assert_eq!(contract.failure_outcomes(), failures);
        assert_eq!(
            contract.read_floor(),
            SnapshotSourceReadFloorV1::RedactOrOmitBeforePayloadRead
        );
    }

    let expected_failure_conditions = [
        &[
            "absent_or_inapplicable",
            "stale_incomplete_noncompleted_or_untrusted",
            "unavailable_or_durability_uncertain",
        ][..],
        &[
            "policy_excluded",
            "required_observation_changed_or_unavailable",
        ][..],
        &[
            "optional_absent",
            "stale_substituted_or_unadmitted_authority",
        ][..],
        &[
            "wrong_open_closed_or_replayed_conflict_sequence",
            "first_start",
        ][..],
        &["policy_inapplicable", "required_stale_or_unavailable"][..],
    ];
    for (contract, expected_conditions) in contracts.iter().zip(expected_failure_conditions) {
        assert_eq!(
            contract
                .failure_rules()
                .iter()
                .map(|rule| rule.condition())
                .collect::<Vec<_>>(),
            expected_conditions
        );
    }
}

#[test]
fn repository_stream_and_boundary_bindings_are_exact_without_creating_state() {
    let repo = TempDir::new().expect("temporary retained repository");
    let owner = SnapshotGroundingOwnerV1::new();
    let binding = owner
        .bind_repository(repo.path(), WORKSPACE_REF, WORKSPACE_FINGERPRINT)
        .expect("exact repository binding");

    assert_eq!(binding.state_root(), ".handbook/state/snapshot-memory/v1");
    assert_eq!(binding.format_ref(), "handbook.snapshot-memory@1.0.0");
    assert_eq!(binding.workspace_ref(), WORKSPACE_REF);
    assert_eq!(
        binding.engine_authority_ref(),
        "handbook.engine.snapshot-grounding-owner@1.0.0"
    );
    assert_eq!(
        binding.policy_ref(),
        "handbook.snapshot-policy.production-session-boundary@1.0.0"
    );
    assert!(!repo.path().join(".handbook").exists());

    let stream = owner.bind_session_stream(&binding);
    assert_eq!(stream.stream_class(), "session");
    assert_eq!(stream.workspace_ref(), WORKSPACE_REF);
    assert_eq!(
        stream.repository_binding_fingerprint(),
        binding.fingerprint()
    );

    assert!(!repo.path().join(".handbook").exists());
}

#[test]
fn accounting_and_failure_vocabulary_is_closed_and_precommit_failures_write_nothing() {
    assert_eq!(
        SnapshotSourceAccountingDispositionV1::ALL,
        [
            SnapshotSourceAccountingDispositionV1::RetainedObservation,
            SnapshotSourceAccountingDispositionV1::DerivedBoundedFact,
            SnapshotSourceAccountingDispositionV1::Redacted,
            SnapshotSourceAccountingDispositionV1::Omitted,
            SnapshotSourceAccountingDispositionV1::Inapplicable,
            SnapshotSourceAccountingDispositionV1::Unavailable,
            SnapshotSourceAccountingDispositionV1::Refused,
        ]
    );

    for kind in SnapshotGroundingErrorKindV1::ALL {
        match kind.class() {
            SnapshotGroundingErrorClassV1::PostCommit => assert_eq!(
                kind.mutation_effect(),
                SnapshotGroundingMutationEffectV1::CommittedReceiptPreserved
            ),
            SnapshotGroundingErrorClassV1::Refusal
            | SnapshotGroundingErrorClassV1::Conflict
            | SnapshotGroundingErrorClassV1::Unavailable
            | SnapshotGroundingErrorClassV1::Error => assert_eq!(
                kind.mutation_effect(),
                SnapshotGroundingMutationEffectV1::WriteNothing
            ),
        }
    }
}

#[test]
fn production_sources_do_not_embed_the_demonstrative_hcm_3_5_tree_or_placeholders() {
    let sources = SnapshotGroundingOwnerV1::new()
        .production_sources()
        .expect("admitted production sources");
    for source in [
        sources.policy(),
        sources.grounding_definition(),
        sources.disclosure_policy(),
    ] {
        let text = std::str::from_utf8(source.canonical_bytes()).expect("UTF-8 canonical source");
        assert!(!text.contains("hcm-3.5"));
        assert!(!text.contains("hcm-3-5"));
        for repeated in ['a', 'b', 'c', 'd', 'e', 'f', '1', '2', '3', '4', '5'] {
            assert!(!text.contains(&format!("sha256:{}", repeated.to_string().repeat(64))));
        }
    }
}
