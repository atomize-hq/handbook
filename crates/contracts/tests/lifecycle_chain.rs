use handbook_contracts::{
    current_lifecycle_head, evaluation_lifecycle_admission_basis_v1, lifecycle_policy_binding_v1,
    lifecycle_policy_canonical_json_v1, prepare_contract_definition_append,
    prepare_lifecycle_transition, ApplicabilityRuleV1, ContractApprovalClassV1,
    ContractClaimInputV1, ContractClaimV1, ContractDefinitionInputV1, ContractDefinitionV1,
    ContractIdV1, ContractKindV1, ContractLifecycleTransitionInputV1,
    ContractLifecycleTransitionV1, EvidenceCardinalityV1, EvidenceCaseSelectionV1,
    EvidenceRequirementV1, ExactBindingV1, ExactRefV1, FingerprintV1, GateEffectV1,
    LifecycleEvidenceKindV1, LifecycleReasonTextV1, LifecycleReasonV1, LifecycleStateV1,
    LifecycleSupportingEvidenceV1, MatcherBindingV1, ResolutionMinimumV1, SelectorCriterionV1,
    SelectorV1, StableIdV1, VersionV1,
};
use handbook_engine::{
    observe_committed_approver_registry, ApproverAdminRequestV1, ApproverRegistryServiceV1,
    CredentialAuthorityAdmissionServiceV1, NativeAuthenticatorPortErrorV1,
    NativeAuthenticatorPortV1, AUTHENTICATOR_RP_ID,
};
use p256::ecdsa::{signature::Signer, Signature, SigningKey};
use serde_json::{json, Value};
use sha2::{Digest, Sha256};

const REPOSITORY_IDENTITY: &str =
    "sha256:1111111111111111111111111111111111111111111111111111111111111111";
const AUTHOR_AUTHORITY_REF: &str = "handbook.authority.contract-definition-author@1.0.0";

struct DualAuthorityPort {
    author_key: SigningKey,
    authority_key: SigningKey,
    author_credential_id: Vec<u8>,
    authority_credential_id: Vec<u8>,
    make_calls: usize,
    author_assertions: u32,
    authority_assertions: u32,
}

impl DualAuthorityPort {
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

impl NativeAuthenticatorPortV1 for DualAuthorityPort {
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
            .expect("GetAssertion client-data hash marker");
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
    _repo: tempfile::TempDir,
    service: CredentialAuthorityAdmissionServiceV1<DualAuthorityPort>,
    registry_state_fingerprint: String,
}

impl Harness {
    fn new() -> Self {
        let repo = tempfile::tempdir().unwrap();
        let mut registry =
            ApproverRegistryServiceV1::for_repository(repo.path(), DualAuthorityPort::new());
        let bootstrap = ApproverAdminRequestV1::from_json_value(json!({
            "schema_id":"handbook.approver-admin-request",
            "schema_version":"1.0",
            "operation":"bootstrap",
            "operation_id":"hcm-5-1-authority-bootstrap",
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
            "operation_id":"hcm-5-1-authority-add-independent",
            "repository_identity_fingerprint":REPOSITORY_IDENTITY,
            "expected_registry_state_fingerprint":bootstrapped["result_registry_state_fingerprint"],
            "expected_transition_fingerprint":bootstrapped["transition_fingerprint"],
            "approval_mappings":[
                {"approval_class":"contract_activation","authority_ref":"handbook.authority.contract-activation@1.0.0"},
                {"approval_class":"contract_independent_lock","authority_ref":"handbook.authority.contract-independent-lock@1.0.0"},
                {"approval_class":"contract_lifecycle","authority_ref":"handbook.authority.contract-lifecycle@1.0.0"}
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
        Self {
            _repo: repo,
            service,
            registry_state_fingerprint,
        }
    }

    fn admitted_definition(&mut self) -> handbook_contracts::AdmittedContractDefinitionV1 {
        let definition = definition(&self.registry_state_fingerprint);
        let prepared = prepare_contract_definition_append(definition).unwrap();
        let request = prepared.authority_request();
        self.service
            .admit(request, |admission| prepared.finalize(admission))
            .unwrap()
            .unwrap()
    }

    fn transition(
        &mut self,
        definition: &handbook_contracts::AdmittedContractDefinitionV1,
        chain: &[ContractLifecycleTransitionV1],
        to_state: LifecycleStateV1,
    ) -> ContractLifecycleTransitionV1 {
        let head = current_lifecycle_head(definition, chain).unwrap();
        let (approval_class, reason, evidence) = exact_edge_inputs(head.state(), to_state);
        let prepared = prepare_lifecycle_transition(
            definition,
            chain,
            ContractLifecycleTransitionInputV1 {
                expected_lifecycle_fingerprint: head.lifecycle_fingerprint().clone(),
                from_state: head.state(),
                to_state,
                approval_class,
                supporting_evidence: evidence,
                reason,
            },
        )
        .unwrap();
        let request = prepared.authority_request();
        self.service
            .admit(request, |admission| prepared.finalize(admission))
            .unwrap()
            .unwrap()
    }

    fn chain_to(
        &mut self,
        definition: &handbook_contracts::AdmittedContractDefinitionV1,
        state: LifecycleStateV1,
    ) -> Vec<ContractLifecycleTransitionV1> {
        let route: &[LifecycleStateV1] = match state {
            LifecycleStateV1::Draft => &[],
            LifecycleStateV1::ReviewReady => &[LifecycleStateV1::ReviewReady],
            LifecycleStateV1::Locked => &[LifecycleStateV1::ReviewReady, LifecycleStateV1::Locked],
            LifecycleStateV1::Active => &[
                LifecycleStateV1::ReviewReady,
                LifecycleStateV1::Locked,
                LifecycleStateV1::Active,
            ],
            LifecycleStateV1::Deprecated => &[
                LifecycleStateV1::ReviewReady,
                LifecycleStateV1::Locked,
                LifecycleStateV1::Active,
                LifecycleStateV1::Deprecated,
            ],
            LifecycleStateV1::Closed => &[
                LifecycleStateV1::ReviewReady,
                LifecycleStateV1::Locked,
                LifecycleStateV1::Active,
                LifecycleStateV1::Deprecated,
                LifecycleStateV1::Closed,
            ],
        };
        let mut chain = Vec::new();
        for next in route {
            let transition = self.transition(definition, &chain, *next);
            chain.push(transition);
        }
        chain
    }
}

#[test]
fn checked_policy_and_draft_genesis_are_exact_and_rederivable() {
    let policy: Value = serde_json::from_str(lifecycle_policy_canonical_json_v1()).unwrap();
    assert_eq!(
        serde_json_canonicalizer::to_string(&policy).unwrap(),
        lifecycle_policy_canonical_json_v1()
    );
    assert_eq!(policy["approval_classes"].as_array().unwrap().len(), 4);
    assert_eq!(policy["transitions"].as_array().unwrap().len(), 8);
    let binding = lifecycle_policy_binding_v1().unwrap();
    assert_eq!(
        binding.reference().as_str(),
        "handbook.contract-lifecycle-policy.independent-lock@1.0.0"
    );
    assert_eq!(
        binding.fingerprint().as_str(),
        "sha256:0c248472dcd9577b4e317fb77776cdc3a019ae85f3b64dd68275cc3c28138b7e"
    );

    let mut harness = Harness::new();
    let admitted = harness.admitted_definition();
    let first = current_lifecycle_head(&admitted, &[]).unwrap();
    let second = current_lifecycle_head(&admitted, &[]).unwrap();
    assert_eq!(first, second);
    assert_eq!(first.state(), LifecycleStateV1::Draft);
    assert!(first.transition_ref().is_none());
}

#[test]
fn all_eight_legal_edges_accept_only_the_exact_role_reason_and_evidence() {
    let mut harness = Harness::new();
    let admitted = harness.admitted_definition();

    let mut active_path = Vec::new();
    for state in [
        LifecycleStateV1::ReviewReady,
        LifecycleStateV1::Locked,
        LifecycleStateV1::Active,
        LifecycleStateV1::Deprecated,
        LifecycleStateV1::Closed,
    ] {
        let transition = harness.transition(&admitted, &active_path, state);
        active_path.push(transition);
    }
    assert_eq!(
        current_lifecycle_head(&admitted, &active_path)
            .unwrap()
            .state(),
        LifecycleStateV1::Closed
    );

    let draft_closed = vec![harness.transition(&admitted, &[], LifecycleStateV1::Closed)];
    assert_eq!(
        current_lifecycle_head(&admitted, &draft_closed)
            .unwrap()
            .state(),
        LifecycleStateV1::Closed
    );

    let mut rejected = vec![harness.transition(&admitted, &[], LifecycleStateV1::ReviewReady)];
    rejected.push(harness.transition(&admitted, &rejected, LifecycleStateV1::Closed));

    let mut locked_deprecated =
        vec![harness.transition(&admitted, &[], LifecycleStateV1::ReviewReady)];
    locked_deprecated.push(harness.transition(
        &admitted,
        &locked_deprecated,
        LifecycleStateV1::Locked,
    ));
    locked_deprecated.push(harness.transition(
        &admitted,
        &locked_deprecated,
        LifecycleStateV1::Deprecated,
    ));
}

#[test]
fn unlisted_edges_stale_heads_and_wrong_conditions_refuse_before_authentication() {
    let mut harness = Harness::new();
    let admitted = harness.admitted_definition();
    let states = [
        LifecycleStateV1::Draft,
        LifecycleStateV1::ReviewReady,
        LifecycleStateV1::Locked,
        LifecycleStateV1::Active,
        LifecycleStateV1::Deprecated,
        LifecycleStateV1::Closed,
    ];
    for from_state in states {
        let chain = harness.chain_to(&admitted, from_state);
        let head = current_lifecycle_head(&admitted, &chain).unwrap();
        for to_state in states {
            let (approval_class, reason, supporting_evidence) = if legal_edge(from_state, to_state)
            {
                exact_edge_inputs(from_state, to_state)
            } else {
                (
                    ContractApprovalClassV1::DefinitionAuthor,
                    LifecycleReasonV1::ReadyForReview,
                    vec![],
                )
            };
            let accepted = prepare_lifecycle_transition(
                &admitted,
                &chain,
                ContractLifecycleTransitionInputV1 {
                    expected_lifecycle_fingerprint: head.lifecycle_fingerprint().clone(),
                    from_state,
                    to_state,
                    approval_class,
                    supporting_evidence,
                    reason,
                },
            )
            .is_ok();
            assert_eq!(
                accepted,
                legal_edge(from_state, to_state),
                "unexpected graph decision for {from_state:?} -> {to_state:?}"
            );
        }
    }
    let head = current_lifecycle_head(&admitted, &[]).unwrap();
    let stale = prepare_lifecycle_transition(
        &admitted,
        &[],
        ContractLifecycleTransitionInputV1 {
            expected_lifecycle_fingerprint: fingerprint('f'),
            from_state: LifecycleStateV1::Draft,
            to_state: LifecycleStateV1::ReviewReady,
            approval_class: ContractApprovalClassV1::DefinitionAuthor,
            supporting_evidence: vec![evidence(LifecycleEvidenceKindV1::DefinitionValidation, '1')],
            reason: LifecycleReasonV1::ReadyForReview,
        },
    );
    assert!(stale.is_err());

    let wrong_role = prepare_lifecycle_transition(
        &admitted,
        &[],
        ContractLifecycleTransitionInputV1 {
            expected_lifecycle_fingerprint: head.lifecycle_fingerprint().clone(),
            from_state: LifecycleStateV1::Draft,
            to_state: LifecycleStateV1::ReviewReady,
            approval_class: ContractApprovalClassV1::Lifecycle,
            supporting_evidence: vec![evidence(LifecycleEvidenceKindV1::DefinitionValidation, '1')],
            reason: LifecycleReasonV1::ReadyForReview,
        },
    );
    assert!(wrong_role.is_err());
}

#[test]
fn every_legal_edge_refuses_a_wrong_role_reason_or_evidence_shape() {
    let mut harness = Harness::new();
    let admitted = harness.admitted_definition();
    for (from_state, to_state) in legal_edges() {
        let chain = harness.chain_to(&admitted, from_state);
        let head = current_lifecycle_head(&admitted, &chain).unwrap();
        let (approval_class, reason, supporting_evidence) = exact_edge_inputs(from_state, to_state);
        let base = ContractLifecycleTransitionInputV1 {
            expected_lifecycle_fingerprint: head.lifecycle_fingerprint().clone(),
            from_state,
            to_state,
            approval_class,
            supporting_evidence,
            reason,
        };

        let mut wrong_role = base.clone();
        wrong_role.approval_class = match base.approval_class {
            ContractApprovalClassV1::DefinitionAuthor => ContractApprovalClassV1::Activation,
            _ => ContractApprovalClassV1::DefinitionAuthor,
        };
        assert!(prepare_lifecycle_transition(&admitted, &chain, wrong_role).is_err());

        let mut wrong_reason = base.clone();
        wrong_reason.reason = LifecycleReasonV1::Activated;
        if matches!(base.reason, LifecycleReasonV1::Activated) {
            wrong_reason.reason = LifecycleReasonV1::Withdrawn;
        }
        assert!(prepare_lifecycle_transition(&admitted, &chain, wrong_reason).is_err());

        let mut wrong_evidence = base;
        wrong_evidence.supporting_evidence = if wrong_evidence.supporting_evidence.is_empty() {
            vec![evidence(LifecycleEvidenceKindV1::DefinitionValidation, 'f')]
        } else {
            vec![]
        };
        assert!(prepare_lifecycle_transition(&admitted, &chain, wrong_evidence).is_err());
    }
}

#[test]
fn lifecycle_authority_may_withdraw_draft_but_scope_substitution_refuses() {
    let mut harness = Harness::new();
    let admitted = harness.admitted_definition();
    let head = current_lifecycle_head(&admitted, &[]).unwrap();
    let lifecycle_withdrawal = prepare_lifecycle_transition(
        &admitted,
        &[],
        ContractLifecycleTransitionInputV1 {
            expected_lifecycle_fingerprint: head.lifecycle_fingerprint().clone(),
            from_state: LifecycleStateV1::Draft,
            to_state: LifecycleStateV1::Closed,
            approval_class: ContractApprovalClassV1::Lifecycle,
            supporting_evidence: vec![],
            reason: LifecycleReasonV1::Withdrawn,
        },
    )
    .unwrap();
    let request = lifecycle_withdrawal.authority_request();
    let transition = harness
        .service
        .admit(request, |admission| {
            lifecycle_withdrawal.finalize(admission)
        })
        .unwrap()
        .unwrap();
    assert_eq!(transition.to_state(), LifecycleStateV1::Closed);

    let first = prepare_lifecycle_transition(
        &admitted,
        &[],
        ContractLifecycleTransitionInputV1 {
            expected_lifecycle_fingerprint: head.lifecycle_fingerprint().clone(),
            from_state: LifecycleStateV1::Draft,
            to_state: LifecycleStateV1::ReviewReady,
            approval_class: ContractApprovalClassV1::DefinitionAuthor,
            supporting_evidence: vec![evidence(LifecycleEvidenceKindV1::DefinitionValidation, '1')],
            reason: LifecycleReasonV1::ReadyForReview,
        },
    )
    .unwrap();
    let substituted = prepare_lifecycle_transition(
        &admitted,
        &[],
        ContractLifecycleTransitionInputV1 {
            expected_lifecycle_fingerprint: head.lifecycle_fingerprint().clone(),
            from_state: LifecycleStateV1::Draft,
            to_state: LifecycleStateV1::ReviewReady,
            approval_class: ContractApprovalClassV1::DefinitionAuthor,
            supporting_evidence: vec![evidence(LifecycleEvidenceKindV1::DefinitionValidation, '2')],
            reason: LifecycleReasonV1::ReadyForReview,
        },
    )
    .unwrap();
    let refusal = harness
        .service
        .admit(first.authority_request(), |admission| {
            substituted.finalize(admission)
        })
        .unwrap()
        .unwrap_err();
    assert_eq!(
        refusal,
        handbook_contracts::ContractLifecycleErrorV1::InvalidAdmission
    );
}

#[test]
fn definition_authority_substitution_and_self_lock_refuse() {
    let mappings = vec![
        json!({
            "approval_class":"contract_definition_author",
            "authority_ref":AUTHOR_AUTHORITY_REF
        }),
        json!({
            "approval_class":"contract_independent_lock",
            "authority_ref":"handbook.authority.contract-independent-lock@1.0.0"
        }),
    ];
    let (_repo, mut service, state_fingerprint) = single_credential_service(mappings);

    let substituted_definition = definition_with(
        &state_fingerprint,
        "handbook.authority.substituted@1.0.0",
        "1.0.0",
        lifecycle_policy_binding_v1().unwrap(),
    );
    let prepared = prepare_contract_definition_append(substituted_definition).unwrap();
    let refusal = service
        .admit(prepared.authority_request(), |admission| {
            prepared.finalize(admission)
        })
        .unwrap()
        .unwrap_err();
    assert_eq!(
        refusal,
        handbook_contracts::ContractLifecycleErrorV1::InvalidAdmission
    );

    let substituted_actor = definition_with_actor(
        &state_fingerprint,
        AUTHOR_AUTHORITY_REF,
        "1.0.0",
        lifecycle_policy_binding_v1().unwrap(),
        0x31,
    );
    let prepared = prepare_contract_definition_append(substituted_actor).unwrap();
    let refusal = service
        .admit(prepared.authority_request(), |admission| {
            prepared.finalize(admission)
        })
        .unwrap()
        .unwrap_err();
    assert_eq!(
        refusal,
        handbook_contracts::ContractLifecycleErrorV1::InvalidAdmission
    );

    let prepared = prepare_contract_definition_append(definition(&state_fingerprint)).unwrap();
    let admitted = service
        .admit(prepared.authority_request(), |admission| {
            prepared.finalize(admission)
        })
        .unwrap()
        .unwrap();
    let review_head = current_lifecycle_head(&admitted, &[]).unwrap();
    let review = prepare_lifecycle_transition(
        &admitted,
        &[],
        ContractLifecycleTransitionInputV1 {
            expected_lifecycle_fingerprint: review_head.lifecycle_fingerprint().clone(),
            from_state: LifecycleStateV1::Draft,
            to_state: LifecycleStateV1::ReviewReady,
            approval_class: ContractApprovalClassV1::DefinitionAuthor,
            supporting_evidence: vec![evidence(LifecycleEvidenceKindV1::DefinitionValidation, '1')],
            reason: LifecycleReasonV1::ReadyForReview,
        },
    )
    .unwrap();
    let review = service
        .admit(review.authority_request(), |admission| {
            review.finalize(admission)
        })
        .unwrap()
        .unwrap();
    let lock_head = current_lifecycle_head(&admitted, std::slice::from_ref(&review)).unwrap();
    let lock = prepare_lifecycle_transition(
        &admitted,
        std::slice::from_ref(&review),
        ContractLifecycleTransitionInputV1 {
            expected_lifecycle_fingerprint: lock_head.lifecycle_fingerprint().clone(),
            from_state: LifecycleStateV1::ReviewReady,
            to_state: LifecycleStateV1::Locked,
            approval_class: ContractApprovalClassV1::IndependentLock,
            supporting_evidence: vec![evidence(LifecycleEvidenceKindV1::IndependentReview, '2')],
            reason: LifecycleReasonV1::IndependentlyLocked,
        },
    )
    .unwrap();
    let refusal = service
        .admit(lock.authority_request(), |admission| {
            lock.finalize(admission)
        })
        .unwrap()
        .unwrap_err();
    assert_eq!(
        refusal,
        handbook_contracts::ContractLifecycleErrorV1::SelfLock
    );
}

#[test]
fn replacement_deprecation_and_duplicate_evidence_require_exact_bindings() {
    let mut harness = Harness::new();
    let admitted = harness.admitted_definition();
    let mut chain = harness.chain_to(&admitted, LifecycleStateV1::Locked);
    let head = current_lifecycle_head(&admitted, &chain).unwrap();
    let replacement = binding("handbook.contract.replacement@2.0.0", 'e');
    let prepared = prepare_lifecycle_transition(
        &admitted,
        &chain,
        ContractLifecycleTransitionInputV1 {
            expected_lifecycle_fingerprint: head.lifecycle_fingerprint().clone(),
            from_state: LifecycleStateV1::Locked,
            to_state: LifecycleStateV1::Deprecated,
            approval_class: ContractApprovalClassV1::Lifecycle,
            supporting_evidence: vec![LifecycleSupportingEvidenceV1::new(
                LifecycleEvidenceKindV1::Replacement,
                replacement.clone(),
            )],
            reason: LifecycleReasonV1::DeprecatedWithReplacement(replacement.clone()),
        },
    )
    .unwrap();
    let transition = harness
        .service
        .admit(prepared.authority_request(), |admission| {
            prepared.finalize(admission)
        })
        .unwrap()
        .unwrap();
    chain.push(transition);
    assert_eq!(
        current_lifecycle_head(&admitted, &chain).unwrap().state(),
        LifecycleStateV1::Deprecated
    );

    let mismatched = prepare_lifecycle_transition(
        &admitted,
        &chain[..2],
        ContractLifecycleTransitionInputV1 {
            expected_lifecycle_fingerprint: head.lifecycle_fingerprint().clone(),
            from_state: LifecycleStateV1::Locked,
            to_state: LifecycleStateV1::Deprecated,
            approval_class: ContractApprovalClassV1::Lifecycle,
            supporting_evidence: vec![LifecycleSupportingEvidenceV1::new(
                LifecycleEvidenceKindV1::Replacement,
                binding("handbook.contract.replacement@2.0.0", 'f'),
            )],
            reason: LifecycleReasonV1::DeprecatedWithReplacement(replacement),
        },
    );
    assert!(mismatched.is_err());

    let draft = current_lifecycle_head(&admitted, &[]).unwrap();
    let duplicate = evidence(LifecycleEvidenceKindV1::DefinitionValidation, '1');
    let duplicate_evidence = prepare_lifecycle_transition(
        &admitted,
        &[],
        ContractLifecycleTransitionInputV1 {
            expected_lifecycle_fingerprint: draft.lifecycle_fingerprint().clone(),
            from_state: LifecycleStateV1::Draft,
            to_state: LifecycleStateV1::ReviewReady,
            approval_class: ContractApprovalClassV1::DefinitionAuthor,
            supporting_evidence: vec![duplicate.clone(), duplicate],
            reason: LifecycleReasonV1::ReadyForReview,
        },
    );
    assert!(duplicate_evidence.is_err());
}

#[test]
fn same_definition_author_remains_valid_after_unrelated_registry_head_advance() {
    let mappings = vec![json!({
        "approval_class":"contract_definition_author",
        "authority_ref":AUTHOR_AUTHORITY_REF
    })];
    let (repo, mut service, original_state_fingerprint) = single_credential_service(mappings);
    let prepared =
        prepare_contract_definition_append(definition(&original_state_fingerprint)).unwrap();
    let admitted = service
        .admit(prepared.authority_request(), |admission| {
            prepared.finalize(admission)
        })
        .unwrap()
        .unwrap();

    let observation = observe_committed_approver_registry(repo.path()).unwrap();
    let mut registry = ApproverRegistryServiceV1::for_repository(repo.path(), service.into_port());
    let add = ApproverAdminRequestV1::from_json_value(json!({
        "schema_id":"handbook.approver-admin-request",
        "schema_version":"1.0",
        "operation":"add_credential",
        "operation_id":"hcm-5-1-unrelated-head-advance",
        "repository_identity_fingerprint":REPOSITORY_IDENTITY,
        "expected_registry_state_fingerprint":observation.state_fingerprint,
        "expected_transition_fingerprint":observation.head_transition_fingerprint,
        "approval_mappings":[{
            "approval_class":"unrelated_approval_class",
            "authority_ref":"handbook.authority.unrelated@1.0.0"
        }]
    }))
    .unwrap();
    drop(observation);
    let result = registry
        .add_approver_credential(add)
        .to_json_value()
        .unwrap();
    assert_eq!(result["status"], "succeeded", "{result:#}");
    assert_ne!(
        result["result_registry_state_fingerprint"],
        original_state_fingerprint
    );
    let mut service = CredentialAuthorityAdmissionServiceV1::new(repo.path(), registry.into_port());

    let head = current_lifecycle_head(&admitted, &[]).unwrap();
    let review = prepare_lifecycle_transition(
        &admitted,
        &[],
        ContractLifecycleTransitionInputV1 {
            expected_lifecycle_fingerprint: head.lifecycle_fingerprint().clone(),
            from_state: LifecycleStateV1::Draft,
            to_state: LifecycleStateV1::ReviewReady,
            approval_class: ContractApprovalClassV1::DefinitionAuthor,
            supporting_evidence: vec![evidence(LifecycleEvidenceKindV1::DefinitionValidation, '1')],
            reason: LifecycleReasonV1::ReadyForReview,
        },
    )
    .unwrap();
    let transition = service
        .admit(review.authority_request(), |admission| {
            review.finalize(admission)
        })
        .unwrap()
        .unwrap();
    assert_eq!(
        transition.actor_ref(),
        admitted.definition().definition_author_ref()
    );
}

#[test]
fn transition_and_result_fingerprints_bind_the_immediate_ordered_chain() {
    let mut harness = Harness::new();
    let admitted = harness.admitted_definition();
    let review = harness.transition(&admitted, &[], LifecycleStateV1::ReviewReady);
    let lock = harness.transition(
        &admitted,
        std::slice::from_ref(&review),
        LifecycleStateV1::Locked,
    );
    assert_eq!(lock.prior_transition_ref(), Some(review.transition_ref()));
    assert_eq!(
        lock.prior_lifecycle_fingerprint(),
        review.resulting_lifecycle_fingerprint()
    );
    assert_ne!(
        lock.transition_fingerprint(),
        review.transition_fingerprint()
    );

    let bytes = lock.canonical_json_bytes().unwrap();
    assert_eq!(
        serde_json::from_slice::<Value>(&bytes).unwrap()["definition_fingerprint"],
        admitted.definition().definition_fingerprint().as_str()
    );
    assert!(serde_json::from_slice::<Value>(&bytes)
        .unwrap()
        .get("verdict")
        .is_none());
    assert!(serde_json::from_slice::<Value>(&bytes)
        .unwrap()
        .get("gate_result")
        .is_none());
}

#[test]
fn evaluation_basis_requires_adjacent_independent_lock_and_current_active_head() {
    let mut harness = Harness::new();
    let admitted = harness.admitted_definition();
    assert!(evaluation_lifecycle_admission_basis_v1(&admitted, &[]).is_err());
    let review = harness.transition(&admitted, &[], LifecycleStateV1::ReviewReady);
    let lock = harness.transition(
        &admitted,
        std::slice::from_ref(&review),
        LifecycleStateV1::Locked,
    );
    assert!(
        evaluation_lifecycle_admission_basis_v1(&admitted, &[review.clone(), lock.clone()])
            .is_err()
    );
    let active = harness.transition(
        &admitted,
        &[review.clone(), lock.clone()],
        LifecycleStateV1::Active,
    );
    let active_chain = vec![review, lock.clone(), active.clone()];
    let basis = evaluation_lifecycle_admission_basis_v1(&admitted, &active_chain).unwrap();
    assert_eq!(
        basis.independent_lock_transition_ref(),
        lock.transition_ref()
    );
    assert_eq!(basis.active_transition_ref(), active.transition_ref());
    assert_eq!(
        basis.active_resulting_lifecycle_fingerprint(),
        active.resulting_lifecycle_fingerprint()
    );
    assert!(basis.basis_ref().contains("#evaluation-lifecycle-basis/"));

    let deprecated = harness.transition(&admitted, &active_chain, LifecycleStateV1::Deprecated);
    let mut stale = active_chain;
    stale.push(deprecated);
    assert!(evaluation_lifecycle_admission_basis_v1(&admitted, &stale).is_err());

    let substituted_definition = definition_with(
        &harness.registry_state_fingerprint,
        AUTHOR_AUTHORITY_REF,
        "2.0.0",
        lifecycle_policy_binding_v1().unwrap(),
    );
    let prepared = prepare_contract_definition_append(substituted_definition).unwrap();
    let substituted = harness
        .service
        .admit(prepared.authority_request(), |admission| {
            prepared.finalize(admission)
        })
        .unwrap()
        .unwrap();
    assert!(evaluation_lifecycle_admission_basis_v1(&substituted, &stale[..3]).is_err());
}

fn legal_edges() -> [(LifecycleStateV1, LifecycleStateV1); 8] {
    [
        (LifecycleStateV1::Draft, LifecycleStateV1::ReviewReady),
        (LifecycleStateV1::Draft, LifecycleStateV1::Closed),
        (LifecycleStateV1::ReviewReady, LifecycleStateV1::Locked),
        (LifecycleStateV1::ReviewReady, LifecycleStateV1::Closed),
        (LifecycleStateV1::Locked, LifecycleStateV1::Active),
        (LifecycleStateV1::Locked, LifecycleStateV1::Deprecated),
        (LifecycleStateV1::Active, LifecycleStateV1::Deprecated),
        (LifecycleStateV1::Deprecated, LifecycleStateV1::Closed),
    ]
}

fn legal_edge(from: LifecycleStateV1, to: LifecycleStateV1) -> bool {
    legal_edges().contains(&(from, to))
}

fn exact_edge_inputs(
    from: LifecycleStateV1,
    to: LifecycleStateV1,
) -> (
    ContractApprovalClassV1,
    LifecycleReasonV1,
    Vec<LifecycleSupportingEvidenceV1>,
) {
    match (from, to) {
        (LifecycleStateV1::Draft, LifecycleStateV1::ReviewReady) => (
            ContractApprovalClassV1::DefinitionAuthor,
            LifecycleReasonV1::ReadyForReview,
            vec![evidence(LifecycleEvidenceKindV1::DefinitionValidation, '1')],
        ),
        (LifecycleStateV1::Draft, LifecycleStateV1::Closed) => (
            ContractApprovalClassV1::DefinitionAuthor,
            LifecycleReasonV1::Withdrawn,
            vec![],
        ),
        (LifecycleStateV1::ReviewReady, LifecycleStateV1::Locked) => (
            ContractApprovalClassV1::IndependentLock,
            LifecycleReasonV1::IndependentlyLocked,
            vec![evidence(LifecycleEvidenceKindV1::IndependentReview, '2')],
        ),
        (LifecycleStateV1::ReviewReady, LifecycleStateV1::Closed) => (
            ContractApprovalClassV1::IndependentLock,
            LifecycleReasonV1::Rejected,
            vec![evidence(LifecycleEvidenceKindV1::Rejection, '3')],
        ),
        (LifecycleStateV1::Locked, LifecycleStateV1::Active) => (
            ContractApprovalClassV1::Activation,
            LifecycleReasonV1::Activated,
            vec![
                evidence(LifecycleEvidenceKindV1::ApplicabilityClosure, '4'),
                evidence(LifecycleEvidenceKindV1::MatcherClosure, '5'),
                evidence(LifecycleEvidenceKindV1::PolicyClosure, '6'),
                evidence(LifecycleEvidenceKindV1::CurrentnessClosure, '7'),
            ],
        ),
        (LifecycleStateV1::Locked | LifecycleStateV1::Active, LifecycleStateV1::Deprecated) => (
            ContractApprovalClassV1::Lifecycle,
            LifecycleReasonV1::DeprecatedWithoutReplacement(
                LifecycleReasonTextV1::parse("No exact replacement is currently authorized.")
                    .unwrap(),
            ),
            vec![evidence(
                LifecycleEvidenceKindV1::NoReplacementRationale,
                '8',
            )],
        ),
        (LifecycleStateV1::Deprecated, LifecycleStateV1::Closed) => (
            ContractApprovalClassV1::Lifecycle,
            LifecycleReasonV1::ClosedAfterRetention,
            vec![
                evidence(LifecycleEvidenceKindV1::RetentionSafety, '9'),
                evidence(LifecycleEvidenceKindV1::ReferenceSafety, 'a'),
            ],
        ),
        _ => panic!("not a legal edge"),
    }
}

fn evidence(kind: LifecycleEvidenceKindV1, hex: char) -> LifecycleSupportingEvidenceV1 {
    LifecycleSupportingEvidenceV1::new(
        kind,
        binding("handbook.contract-lifecycle-evidence.example@1.0.0", hex),
    )
}

fn definition(registry_state_fingerprint: &str) -> ContractDefinitionV1 {
    definition_with(
        registry_state_fingerprint,
        AUTHOR_AUTHORITY_REF,
        "1.0.0",
        lifecycle_policy_binding_v1().unwrap(),
    )
}

fn definition_with(
    registry_state_fingerprint: &str,
    authority_ref: &str,
    version: &str,
    lifecycle_policy: ExactBindingV1,
) -> ContractDefinitionV1 {
    definition_with_actor(
        registry_state_fingerprint,
        authority_ref,
        version,
        lifecycle_policy,
        0x32,
    )
}

fn definition_with_actor(
    registry_state_fingerprint: &str,
    authority_ref: &str,
    version: &str,
    lifecycle_policy: ExactBindingV1,
    actor_credential_byte: u8,
) -> ContractDefinitionV1 {
    ContractDefinitionV1::new(ContractDefinitionInputV1 {
        definition_schema: binding("handbook.contract-definition@1.0.0", 'b'),
        contract_id: ContractIdV1::parse("handbook.contract.example").unwrap(),
        contract_version: VersionV1::parse(version).unwrap(),
        contract_kind: ContractKindV1::ClaimSet,
        claims: vec![claim()],
        definition_author_ref: StableIdV1::parse(&format!(
            "credential.sha256-{}",
            DefinitionFingerprintString::from_bytes(&[actor_credential_byte; 32])
        ))
        .unwrap(),
        definition_authority: ExactBindingV1::new(
            ExactRefV1::parse(authority_ref).unwrap(),
            FingerprintV1::parse(registry_state_fingerprint).unwrap(),
        ),
        lifecycle_policy,
        matcher_bindings: vec![MatcherBindingV1::exact_fields_v1()],
        gate_policy: binding("handbook.contract-gate-policy.closed-v1@1.0.0", 'c'),
        extensions_policy: binding("handbook.contract-extensions.closed-v1@1.0.0", 'd'),
    })
    .unwrap()
}

fn single_credential_service(
    mappings: Vec<Value>,
) -> (
    tempfile::TempDir,
    CredentialAuthorityAdmissionServiceV1<DualAuthorityPort>,
    String,
) {
    let repo = tempfile::tempdir().unwrap();
    let mut registry =
        ApproverRegistryServiceV1::for_repository(repo.path(), DualAuthorityPort::new());
    let request = ApproverAdminRequestV1::from_json_value(json!({
        "schema_id":"handbook.approver-admin-request",
        "schema_version":"1.0",
        "operation":"bootstrap",
        "operation_id":"hcm-5-1-single-credential-bootstrap",
        "repository_identity_fingerprint":REPOSITORY_IDENTITY,
        "expected_registry_state_fingerprint":null,
        "expected_transition_fingerprint":null,
        "initial_charter_quorum":mappings
    }))
    .unwrap();
    let result = registry
        .bootstrap_approver_registry(request)
        .to_json_value()
        .unwrap();
    assert_eq!(result["status"], "succeeded", "{result:#}");
    let observation = observe_committed_approver_registry(repo.path()).unwrap();
    let state_fingerprint = observation.state_fingerprint.clone();
    drop(observation);
    let service = CredentialAuthorityAdmissionServiceV1::new(repo.path(), registry.into_port());
    (repo, service, state_fingerprint)
}

struct DefinitionFingerprintString;

impl DefinitionFingerprintString {
    fn from_bytes(bytes: &[u8]) -> String {
        format!("{:x}", Sha256::digest(bytes))
    }
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
    ExactBindingV1::new(ExactRefV1::parse(value).unwrap(), fingerprint(hex))
}

fn fingerprint(hex: char) -> FingerprintV1 {
    FingerprintV1::parse(&format!("sha256:{}", hex.to_string().repeat(64))).unwrap()
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
