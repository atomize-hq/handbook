use handbook_contracts::{
    lifecycle_policy_binding_v1, prepare_contract_definition_append, ApplicabilityRuleV1,
    ContentAddressedBindingV1, ContractApprovalClassV1, ContractClaimInputV1, ContractClaimV1,
    ContractDefinitionInputV1, ContractDefinitionV1, ContractIdV1, ContractKindV1,
    ContractLifecycleTransitionInputV1, ContractOwnerRepositoryV1, EvaluationCaseV1,
    EvaluationPolicyRegistryV1, EvaluationSubjectV1, EvidenceCardinalityV1,
    EvidenceCaseSelectionV1, EvidenceFreshnessPolicyInputV1, EvidenceFreshnessPolicyV1,
    EvidenceRequirementV1, ExactBindingV1 as OwnerBinding, ExactRefV1 as OwnerRef,
    FingerprintV1 as OwnerFingerprint, GateEffectV1, GatePolicyAlgorithmV1, GatePolicyInputV1,
    GatePolicyV1, LifecycleEvidenceKindV1, LifecycleReasonTextV1, LifecycleReasonV1,
    LifecycleStateV1, LifecycleSupportingEvidenceV1, MatcherBindingV1, MatcherInputValueV1,
    ResolutionEnvelopeV1, ResolutionMinimumV1, SelectorCriterionV1, SelectorV1, StableIdV1,
    VersionV1,
};
use handbook_dock::{
    DockBundleFileV1, DockCancellationV1, DockImplementationBundleInputV1,
    DockImplementationBundleV1, DockImplementationKindV1, DockInputRoleV1, DockManifestInputV1,
    DockManifestV1, DockOperationalStoreV1, DockProtocolLimitInputV1, DockProtocolLimitsV1,
    DockResolvedFirstPartyBundleV1, DockTargetTupleV1, SafeRelativePathV1,
    DOCK_MANIFEST_SCHEMA_REF_V1, DOCK_PROCESS_PROTOCOL_REF_V1, DOCK_PROCESS_REQUEST_SCHEMA_REF_V1,
    DOCK_PROCESS_RESULT_SCHEMA_REF_V1,
};
use handbook_engine::{
    observe_committed_approver_registry, ApproverAdminRequestV1, ApproverRegistryServiceV1,
    ContextResolutionPolicyRegistry, ContextResolutionStackDefinition,
    CredentialAuthorityAdmissionServiceV1, NativeAuthenticatorPortErrorV1,
    NativeAuthenticatorPortV1, AUTHENTICATOR_RP_ID,
};
use handbook_sdk::contract_membrane::{
    dock_run_operation_request, parse_canonical_json, BoundedString, BoundedVec, ContractLimits,
    ContractRecordBindingDtoV1, ContractValue, DockEvaluationPreparationDtoV1,
    DockInlineJsonInputDtoV1, DockProtocolLimitsDtoV1, DockProtocolLimitsInputV1,
    DockRunExecutionContextV1, DockRunRequestV1, Fingerprint, OperationRequestV1,
    OperationResponseV1,
};
use p256::ecdsa::{signature::Signer, Signature, SigningKey};
use serde::Serialize;
use serde_json::json;
use sha2::{Digest, Sha256};
use std::fs;
use std::path::Path;

const REPOSITORY_IDENTITY: &str =
    "sha256:1111111111111111111111111111111111111111111111111111111111111111";
const AUTHOR_AUTHORITY_REF: &str = "handbook.authority.contract-definition-author@1.0.0";
const EVIDENCE_KIND: &str = "handbook.evidence.schema-conformance";

pub struct IntegratedDockFixture {
    owner: OwnerHarness,
    _bundle: tempfile::TempDir,
    manifest: DockManifestV1,
    resolved_bundle: DockResolvedFirstPartyBundleV1,
    store: DockOperationalStoreV1,
    policies: EvaluationPolicyRegistryV1,
    evaluation: DockEvaluationPreparationDtoV1,
    contract: OwnerRef,
    definition_fingerprint: OwnerFingerprint,
    active_head: OwnerFingerprint,
}

impl IntegratedDockFixture {
    pub fn new() -> Self {
        Self::with_helper_mode("completed-one")
    }

    pub fn process_failure() -> Self {
        Self::with_helper_mode("nonzero")
    }

    fn with_helper_mode(helper_mode: &str) -> Self {
        let mut owner = OwnerHarness::new();
        let (stack, stack_binding) = evidence_resolution_stack();
        let resolution = evidence_resolution(&stack);
        let freshness = EvidenceFreshnessPolicyV1::new(
            EvidenceFreshnessPolicyInputV1::new(
                ContractIdV1::parse("handbook.contract-evidence-freshness-policy.packet-five")
                    .unwrap(),
                VersionV1::parse("1.0.0+packet5").unwrap(),
                300,
                true,
                0,
            )
            .unwrap(),
        )
        .unwrap();
        let gate = GatePolicyV1::new(
            GatePolicyInputV1::new(
                ContractIdV1::parse("handbook.contract-gate-policy.packet-five").unwrap(),
                VersionV1::parse("1.0.0+packet5").unwrap(),
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
        let definition = evidence_definition(
            &owner.registry_state_fingerprint,
            freshness.binding().clone(),
            gate.binding().clone(),
            stack_binding,
        );
        let (contract, definition_fingerprint, active_head) =
            activate_definition(&mut owner, definition);

        let bundle = tempfile::tempdir().unwrap();
        let bin = bundle.path().join("bin");
        fs::create_dir(&bin).unwrap();
        let helper = Path::new(match option_env!("CARGO_BIN_EXE_dock-sdk-test-helper") {
            Some(helper) => helper,
            None => panic!("the explicit test-runtime fixture requires its helper executable"),
        });
        let entry_name = helper.file_name().unwrap();
        let entry = bin.join(entry_name);
        fs::copy(helper, &entry).unwrap();
        let bytes = fs::read(&entry).unwrap();
        let relative = format!("bin/{}", entry_name.to_string_lossy());
        let limits = owner_limits();
        let bundle_definition = DockImplementationBundleV1::new(DockImplementationBundleInputV1 {
            bundle_ref: OwnerRef::parse("handbook.dock.bundle.packet-five@1.0.0").unwrap(),
            files: vec![DockBundleFileV1::new(
                SafeRelativePathV1::parse(&relative).unwrap(),
                bytes.len() as u64,
                digest(&bytes),
            )
            .unwrap()],
            entrypoint: SafeRelativePathV1::parse(&relative).unwrap(),
            entrypoint_fingerprint: digest(&bytes),
            fixed_arguments: vec![helper_mode.to_owned()],
            target: DockTargetTupleV1::parse(handbook_dock::host_target_tuple_v1()).unwrap(),
        })
        .unwrap();
        let input_schema = owner_binding("handbook.schema.packet-five-input@1.0.0", '5');
        let manifest = DockManifestV1::new(DockManifestInputV1 {
            manifest_schema: owner_binding(DOCK_MANIFEST_SCHEMA_REF_V1, '1'),
            dock_id: ContractIdV1::parse("handbook.dock.packet-five-fixture").unwrap(),
            dock_version: VersionV1::parse("1.0.0").unwrap(),
            protocol: owner_binding(DOCK_PROCESS_PROTOCOL_REF_V1, '2'),
            request_schema: owner_binding(DOCK_PROCESS_REQUEST_SCHEMA_REF_V1, '3'),
            result_schema: owner_binding(DOCK_PROCESS_RESULT_SCHEMA_REF_V1, '4'),
            implementation_kind: DockImplementationKindV1::NativeProcess,
            implementation_bundle: bundle_definition,
            supported_evidence_kinds: vec![owner_id(EVIDENCE_KIND)],
            supported_inputs: vec![DockInputRoleV1::new(
                owner_id("input.instance"),
                "application/json",
                input_schema,
            )
            .unwrap()],
            resolution_ceiling: resolution.clone(),
            limits: limits.clone(),
        })
        .unwrap();
        let resolved_bundle =
            DockResolvedFirstPartyBundleV1::for_testing(manifest.clone(), bundle.path()).unwrap();
        let store = DockOperationalStoreV1::open(owner.repo.path()).unwrap();
        let evaluation = DockEvaluationPreparationDtoV1::new(
            outward_binding(&OwnerBinding::new(
                contract.clone(),
                definition_fingerprint.clone(),
            )),
            outward_fingerprint(&active_head),
            contract_value(&evaluation_subject('7')),
            BoundedVec::new(vec![contract_value(&evaluation_case('8'))]).unwrap(),
            contract_value(&resolution),
            contract_value(&MatcherBindingV1::exact_fields_v1()),
            BoundedVec::new(vec![outward_binding(freshness.binding())]).unwrap(),
            BoundedVec::new(vec![outward_binding(
                &handbook_contracts::evidence_stability_policy_binding_v1().unwrap(),
            )])
            .unwrap(),
            outward_binding(gate.binding()),
            outward_binding(&handbook_contracts::local_closeout_policy_binding_v1().unwrap()),
            BoundedString::new("packet-five-integrated-run".to_owned()).unwrap(),
        );

        Self {
            owner,
            _bundle: bundle,
            manifest,
            resolved_bundle,
            store,
            policies,
            evaluation,
            contract,
            definition_fingerprint,
            active_head,
        }
    }

    pub fn request(&self, raw_key: &str, content: serde_json::Value) -> OperationRequestV1 {
        let body = DockRunRequestV1::new(
            outward_binding(&self.manifest.binding()),
            self.evaluation.clone(),
            BoundedVec::new(vec![BoundedString::new(EVIDENCE_KIND.to_owned()).unwrap()]).unwrap(),
            BoundedVec::new(vec![DockInlineJsonInputDtoV1::new(
                BoundedString::new("input.instance".to_owned()).unwrap(),
                BoundedString::new("application/json".to_owned()).unwrap(),
                outward_binding(&owner_binding(
                    "handbook.schema.packet-five-input@1.0.0",
                    '5',
                )),
                contract_value(&content),
            )])
            .unwrap(),
            outward_limits(self.manifest.limits()),
        );
        dock_run_operation_request(Some("packet-five-integrated"), raw_key, body).unwrap()
    }

    pub fn repository_path(&self) -> &Path {
        self.owner.repo.path()
    }

    pub fn deprecate_contract(&mut self) {
        self.owner.transition(
            &self.contract,
            &self.definition_fingerprint,
            ContractLifecycleTransitionInputV1 {
                expected_lifecycle_fingerprint: self.active_head.clone(),
                from_state: LifecycleStateV1::Active,
                to_state: LifecycleStateV1::Deprecated,
                approval_class: ContractApprovalClassV1::Lifecycle,
                supporting_evidence: vec![LifecycleSupportingEvidenceV1::new(
                    LifecycleEvidenceKindV1::NoReplacementRationale,
                    owner_binding(
                        "handbook.contract-lifecycle-evidence.packet-five@1.0.0",
                        '9',
                    ),
                )],
                reason: LifecycleReasonV1::DeprecatedWithoutReplacement(
                    LifecycleReasonTextV1::parse("No exact replacement is currently authorized.")
                        .unwrap(),
                ),
            },
            "packet-five-definition-deprecate",
        );
    }

    pub fn execute(&self, request: &OperationRequestV1) -> OperationResponseV1 {
        handbook_sdk::HandbookSdkV1::open(self.owner.repo.path())
            .execute_dock_operation_with_runtime(
                request,
                DockRunExecutionContextV1 {
                    policies: &self.policies,
                    manifest: &self.manifest,
                    resolved_bundle: &self.resolved_bundle,
                    store: &self.store,
                    cancellation: &DockCancellationV1::new(),
                },
            )
            .unwrap()
    }
}

#[allow(dead_code)]
pub struct ProductionDockFixture {
    owner: OwnerHarness,
    policies: EvaluationPolicyRegistryV1,
    evaluation: DockEvaluationPreparationDtoV1,
}

#[allow(dead_code)]
impl ProductionDockFixture {
    pub fn new() -> Self {
        let mut owner = OwnerHarness::new();
        let (stack, stack_binding) = evidence_resolution_stack();
        let resolution = evidence_resolution(&stack);
        let freshness = EvidenceFreshnessPolicyV1::new(
            EvidenceFreshnessPolicyInputV1::new(
                ContractIdV1::parse("handbook.contract-evidence-freshness-policy.packet-four")
                    .unwrap(),
                VersionV1::parse("1.0.0+packet4").unwrap(),
                300,
                true,
                0,
            )
            .unwrap(),
        )
        .unwrap();
        let gate = GatePolicyV1::new(
            GatePolicyInputV1::new(
                ContractIdV1::parse("handbook.contract-gate-policy.packet-four").unwrap(),
                VersionV1::parse("1.0.0+packet4").unwrap(),
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
        let definition = production_evidence_definition(
            &owner.registry_state_fingerprint,
            freshness.binding().clone(),
            gate.binding().clone(),
            stack_binding,
        );
        let (contract, definition_fingerprint, active_head) =
            activate_definition(&mut owner, definition);
        let evaluation = DockEvaluationPreparationDtoV1::new(
            outward_binding(&OwnerBinding::new(contract, definition_fingerprint)),
            outward_fingerprint(&active_head),
            contract_value(&evaluation_subject('7')),
            BoundedVec::new(vec![contract_value(&production_evaluation_case('8'))]).unwrap(),
            contract_value(&resolution),
            contract_value(&MatcherBindingV1::exact_fields_v1()),
            BoundedVec::new(vec![outward_binding(freshness.binding())]).unwrap(),
            BoundedVec::new(vec![outward_binding(
                &handbook_contracts::evidence_stability_policy_binding_v1().unwrap(),
            )])
            .unwrap(),
            outward_binding(gate.binding()),
            outward_binding(&handbook_contracts::local_closeout_policy_binding_v1().unwrap()),
            BoundedString::new("packet-four-production-run".to_owned()).unwrap(),
        );
        Self {
            owner,
            policies,
            evaluation,
        }
    }

    pub fn request(
        &self,
        manifest: &DockManifestV1,
        raw_key: &str,
        content: serde_json::Value,
    ) -> OperationRequestV1 {
        let body = DockRunRequestV1::new(
            outward_binding(&manifest.binding()),
            self.evaluation.clone(),
            BoundedVec::new(vec![BoundedString::new(EVIDENCE_KIND.to_owned()).unwrap()]).unwrap(),
            BoundedVec::new(vec![DockInlineJsonInputDtoV1::new(
                BoundedString::new("handbook.input.json-schema-validation".to_owned()).unwrap(),
                BoundedString::new("application/json".to_owned()).unwrap(),
                outward_binding(manifest.supported_inputs()[0].schema()),
                contract_value(&content),
            )])
            .unwrap(),
            outward_limits(manifest.limits()),
        );
        dock_run_operation_request(Some("packet-four-production"), raw_key, body).unwrap()
    }

    pub fn repository_path(&self) -> &Path {
        self.owner.repo.path()
    }

    pub fn policies(&self) -> &EvaluationPolicyRegistryV1 {
        &self.policies
    }
}

struct OwnerHarness {
    repo: tempfile::TempDir,
    owner: ContractOwnerRepositoryV1,
    service: CredentialAuthorityAdmissionServiceV1<TestAuthenticatorPort>,
    registry_state_fingerprint: String,
}

impl OwnerHarness {
    fn new() -> Self {
        let repo = tempfile::tempdir().unwrap();
        let mut registry =
            ApproverRegistryServiceV1::for_repository(repo.path(), TestAuthenticatorPort::new());
        let request = ApproverAdminRequestV1::from_json_value(json!({
            "schema_id":"handbook.approver-admin-request",
            "schema_version":"1.0",
            "operation":"bootstrap",
            "operation_id":"hcm-5-3-packet-five-bootstrap",
            "repository_identity_fingerprint":REPOSITORY_IDENTITY,
            "expected_registry_state_fingerprint":null,
            "expected_transition_fingerprint":null,
            "initial_charter_quorum":[
                {"approval_class":"contract_definition_author","authority_ref":AUTHOR_AUTHORITY_REF},
                {"approval_class":"contract_evidence_source","authority_ref":"handbook.authority.contract-evidence-source@1.0.0"}
            ]
        }))
        .unwrap();
        let result = registry
            .bootstrap_approver_registry(request)
            .to_json_value()
            .unwrap();
        assert_eq!(result["status"], "succeeded", "{result:#}");
        let add = ApproverAdminRequestV1::from_json_value(json!({
            "schema_id":"handbook.approver-admin-request",
            "schema_version":"1.0",
            "operation":"add_credential",
            "operation_id":"hcm-5-3-packet-five-lifecycle-add",
            "repository_identity_fingerprint":REPOSITORY_IDENTITY,
            "expected_registry_state_fingerprint":result["result_registry_state_fingerprint"],
            "expected_transition_fingerprint":result["transition_fingerprint"],
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
        let owner = ContractOwnerRepositoryV1::open(repo.path()).unwrap();
        Self {
            repo,
            owner,
            service,
            registry_state_fingerprint,
        }
    }

    fn append_definition(
        &mut self,
        definition: ContractDefinitionV1,
        key: &str,
    ) -> handbook_contracts::DefinitionAppendExecutionV1 {
        let prepared = prepare_contract_definition_append(definition).unwrap();
        self.service
            .admit(prepared.authority_request(), |admission| {
                self.owner.append_definition(prepared, admission, key)
            })
            .unwrap()
            .unwrap()
    }

    fn transition(
        &mut self,
        contract: &OwnerRef,
        definition_fingerprint: &OwnerFingerprint,
        input: ContractLifecycleTransitionInputV1,
        key: &str,
    ) -> handbook_contracts::LifecycleTransitionExecutionV1 {
        let prepared = self
            .owner
            .prepare_lifecycle_transition(contract, definition_fingerprint, input)
            .unwrap();
        self.service
            .admit(prepared.authority_request(), |admission| {
                self.owner.transition_lifecycle(prepared, admission, key)
            })
            .unwrap()
            .unwrap()
    }
}

fn activate_definition(
    harness: &mut OwnerHarness,
    definition: ContractDefinitionV1,
) -> (OwnerRef, OwnerFingerprint, OwnerFingerprint) {
    let contract = definition.contract_ref().clone();
    let definition_fingerprint = definition.definition_fingerprint().clone();
    let draft = harness
        .append_definition(definition, "packet-five-definition-append")
        .lifecycle_head()
        .clone();
    let ready = harness.transition(
        &contract,
        &definition_fingerprint,
        ContractLifecycleTransitionInputV1 {
            expected_lifecycle_fingerprint: draft.lifecycle_fingerprint().clone(),
            from_state: LifecycleStateV1::Draft,
            to_state: LifecycleStateV1::ReviewReady,
            approval_class: ContractApprovalClassV1::DefinitionAuthor,
            supporting_evidence: vec![LifecycleSupportingEvidenceV1::new(
                LifecycleEvidenceKindV1::DefinitionValidation,
                owner_binding(
                    "handbook.contract-lifecycle-evidence.packet-five@1.0.0",
                    '1',
                ),
            )],
            reason: LifecycleReasonV1::ReadyForReview,
        },
        "packet-five-definition-ready",
    );
    let locked = harness.transition(
        &contract,
        &definition_fingerprint,
        ContractLifecycleTransitionInputV1 {
            expected_lifecycle_fingerprint: ready.lifecycle_head().lifecycle_fingerprint().clone(),
            from_state: LifecycleStateV1::ReviewReady,
            to_state: LifecycleStateV1::Locked,
            approval_class: ContractApprovalClassV1::IndependentLock,
            supporting_evidence: vec![LifecycleSupportingEvidenceV1::new(
                LifecycleEvidenceKindV1::IndependentReview,
                owner_binding(
                    "handbook.contract-lifecycle-evidence.packet-five@1.0.0",
                    '2',
                ),
            )],
            reason: LifecycleReasonV1::IndependentlyLocked,
        },
        "packet-five-definition-lock",
    );
    let active = harness.transition(
        &contract,
        &definition_fingerprint,
        ContractLifecycleTransitionInputV1 {
            expected_lifecycle_fingerprint: locked.lifecycle_head().lifecycle_fingerprint().clone(),
            from_state: LifecycleStateV1::Locked,
            to_state: LifecycleStateV1::Active,
            approval_class: ContractApprovalClassV1::Activation,
            supporting_evidence: vec![
                LifecycleSupportingEvidenceV1::new(
                    LifecycleEvidenceKindV1::ApplicabilityClosure,
                    owner_binding(
                        "handbook.contract-lifecycle-evidence.packet-five@1.0.0",
                        '3',
                    ),
                ),
                LifecycleSupportingEvidenceV1::new(
                    LifecycleEvidenceKindV1::MatcherClosure,
                    owner_binding(
                        "handbook.contract-lifecycle-evidence.packet-five@1.0.0",
                        '4',
                    ),
                ),
                LifecycleSupportingEvidenceV1::new(
                    LifecycleEvidenceKindV1::PolicyClosure,
                    owner_binding(
                        "handbook.contract-lifecycle-evidence.packet-five@1.0.0",
                        '5',
                    ),
                ),
                LifecycleSupportingEvidenceV1::new(
                    LifecycleEvidenceKindV1::CurrentnessClosure,
                    owner_binding(
                        "handbook.contract-lifecycle-evidence.packet-five@1.0.0",
                        '6',
                    ),
                ),
            ],
            reason: LifecycleReasonV1::Activated,
        },
        "packet-five-definition-active",
    );
    (
        contract,
        definition_fingerprint,
        active.lifecycle_head().lifecycle_fingerprint().clone(),
    )
}

fn evidence_definition(
    registry_state_fingerprint: &str,
    freshness_policy: OwnerBinding,
    gate_policy: OwnerBinding,
    stack_binding: OwnerBinding,
) -> ContractDefinitionV1 {
    ContractDefinitionV1::new(ContractDefinitionInputV1 {
        definition_schema: owner_binding("handbook.contract-definition@1.0.0", 'b'),
        contract_id: ContractIdV1::parse("handbook.contract.packet-five-integrated").unwrap(),
        contract_version: VersionV1::parse("1.0.0").unwrap(),
        contract_kind: ContractKindV1::ClaimSet,
        claims: vec![ContractClaimV1::new(ContractClaimInputV1 {
            claim_id: owner_id("schema.payload.valid"),
            statement: "Payload conforms to its exact schema.".to_owned(),
            subject_selector: owner_selector("subject.kind", "artifact"),
            case_selector: owner_selector("case.kind", "positive"),
            applicability_rule: ApplicabilityRuleV1::SubjectAndCaseMatch,
            gate_effect: GateEffectV1::Required,
            evidence_requirements: vec![EvidenceRequirementV1::new(
                owner_id(EVIDENCE_KIND),
                EvidenceCardinalityV1::ExactlyOne,
                EvidenceCaseSelectionV1::ClaimCaseSelector,
                handbook_contracts::evidence_stability_policy_binding_v1().unwrap(),
            )
            .unwrap()],
            freshness_policy,
            minimum_resolution: ResolutionMinimumV1::new(
                stack_binding,
                owner_id("resolution.scope.local-observation"),
                owner_id("resolution.detail.identifier-only"),
                owner_id("resolution.time.current-operation"),
                owner_id("resolution.authority.read-only"),
                owner_id("resolution.memory.operation"),
                owner_id("resolution.validation.observation-only"),
            ),
            score_weight: None,
        })
        .unwrap()],
        definition_author_ref: StableIdV1::parse(&format!(
            "credential.sha256-{:x}",
            Sha256::digest([0x32; 32])
        ))
        .unwrap(),
        definition_authority: OwnerBinding::new(
            OwnerRef::parse(AUTHOR_AUTHORITY_REF).unwrap(),
            OwnerFingerprint::parse(registry_state_fingerprint).unwrap(),
        ),
        lifecycle_policy: lifecycle_policy_binding_v1().unwrap(),
        matcher_bindings: vec![MatcherBindingV1::exact_fields_v1()],
        gate_policy,
        extensions_policy: owner_binding("handbook.contract-extensions.closed-v1@1.0.0", 'd'),
    })
    .unwrap()
}

fn production_evidence_definition(
    registry_state_fingerprint: &str,
    freshness_policy: OwnerBinding,
    gate_policy: OwnerBinding,
    stack_binding: OwnerBinding,
) -> ContractDefinitionV1 {
    ContractDefinitionV1::new(ContractDefinitionInputV1 {
        definition_schema: owner_binding("handbook.contract-definition@1.0.0", 'b'),
        contract_id: ContractIdV1::parse("handbook.contract.packet-four-production").unwrap(),
        contract_version: VersionV1::parse("1.0.0").unwrap(),
        contract_kind: ContractKindV1::ClaimSet,
        claims: vec![ContractClaimV1::new(ContractClaimInputV1 {
            claim_id: owner_id("payload.schema-conformant"),
            statement: "Payload conforms to its exact schema.".to_owned(),
            subject_selector: owner_selector("subject.kind", "artifact"),
            case_selector: owner_selector("case.kind", "positive"),
            applicability_rule: ApplicabilityRuleV1::SubjectAndCaseMatch,
            gate_effect: GateEffectV1::Required,
            evidence_requirements: vec![EvidenceRequirementV1::new(
                owner_id(EVIDENCE_KIND),
                EvidenceCardinalityV1::ExactlyOne,
                EvidenceCaseSelectionV1::ClaimCaseSelector,
                handbook_contracts::evidence_stability_policy_binding_v1().unwrap(),
            )
            .unwrap()],
            freshness_policy,
            minimum_resolution: ResolutionMinimumV1::new(
                stack_binding,
                owner_id("resolution.scope.local-observation"),
                owner_id("resolution.detail.identifier-only"),
                owner_id("resolution.time.current-operation"),
                owner_id("resolution.authority.read-only"),
                owner_id("resolution.memory.operation"),
                owner_id("resolution.validation.observation-only"),
            ),
            score_weight: None,
        })
        .unwrap()],
        definition_author_ref: StableIdV1::parse(&format!(
            "credential.sha256-{:x}",
            Sha256::digest([0x32; 32])
        ))
        .unwrap(),
        definition_authority: OwnerBinding::new(
            OwnerRef::parse(AUTHOR_AUTHORITY_REF).unwrap(),
            OwnerFingerprint::parse(registry_state_fingerprint).unwrap(),
        ),
        lifecycle_policy: lifecycle_policy_binding_v1().unwrap(),
        matcher_bindings: vec![MatcherBindingV1::exact_fields_v1()],
        gate_policy,
        extensions_policy: owner_binding("handbook.contract-extensions.closed-v1@1.0.0", 'd'),
    })
    .unwrap()
}

fn evaluation_subject(hex: char) -> EvaluationSubjectV1 {
    let fingerprint = owner_fp(hex);
    EvaluationSubjectV1::new(
        ContentAddressedBindingV1::new(
            &format!("subject.example/{}", &fingerprint.as_str()[7..]),
            fingerprint,
        )
        .unwrap(),
        vec![MatcherInputValueV1::new(owner_id("subject.kind"), "artifact").unwrap()],
    )
    .unwrap()
}

fn evaluation_case(hex: char) -> EvaluationCaseV1 {
    let fingerprint = owner_fp(hex);
    EvaluationCaseV1::new(
        owner_id("case.positive"),
        ContentAddressedBindingV1::new(
            &format!("case.example/{}", &fingerprint.as_str()[7..]),
            fingerprint,
        )
        .unwrap(),
        vec![MatcherInputValueV1::new(owner_id("case.kind"), "positive").unwrap()],
    )
    .unwrap()
}

fn production_evaluation_case(hex: char) -> EvaluationCaseV1 {
    let fingerprint = owner_fp(hex);
    EvaluationCaseV1::new(
        owner_id("case.schema-conformance"),
        ContentAddressedBindingV1::new(
            &format!("case.example/{}", &fingerprint.as_str()[7..]),
            fingerprint,
        )
        .unwrap(),
        vec![MatcherInputValueV1::new(owner_id("case.kind"), "positive").unwrap()],
    )
    .unwrap()
}

fn evidence_resolution_stack() -> (ContextResolutionStackDefinition, OwnerBinding) {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../engine")
        .canonicalize()
        .unwrap();
    let policies = ContextResolutionPolicyRegistry::load(
        &root,
        &[
            "definitions/context-resolution-policies/handbook.mutation-matcher.core/1.0.0.yaml"
                .to_owned(),
            "definitions/context-resolution-policies/handbook.resolution-escalation.core/1.0.0.yaml"
                .to_owned(),
            "definitions/context-resolution-policies/handbook.memory-promotion.core/1.0.0.yaml"
                .to_owned(),
        ],
    )
    .unwrap();
    let stack = ContextResolutionStackDefinition::load(
        &root,
        "definitions/context-resolution/handbook.context-resolution.shipped-root/1.0.0.yaml",
        &policies,
    )
    .unwrap();
    let binding = OwnerBinding::new(
        OwnerRef::parse(stack.exact_ref().as_str()).unwrap(),
        OwnerFingerprint::parse(stack.definition_fingerprint().as_str()).unwrap(),
    );
    (stack, binding)
}

fn evidence_resolution(stack: &ContextResolutionStackDefinition) -> ResolutionEnvelopeV1 {
    ResolutionEnvelopeV1::new(
        OwnerBinding::new(
            OwnerRef::parse(stack.exact_ref().as_str()).unwrap(),
            OwnerFingerprint::parse(stack.definition_fingerprint().as_str()).unwrap(),
        ),
        owner_id("resolution.scope.local-observation"),
        owner_id("resolution.detail.identifier-only"),
        owner_id("resolution.time.current-operation"),
        owner_id("resolution.authority.read-only"),
        owner_id("resolution.memory.operation"),
        owner_id("resolution.validation.observation-only"),
    )
}

fn owner_limits() -> DockProtocolLimitsV1 {
    DockProtocolLimitsV1::new(DockProtocolLimitInputV1 {
        request_bytes: 1024 * 1024,
        result_bytes: 64 * 1024,
        stderr_bytes: 4096,
        diagnostic_bytes: 1024,
        candidate_bytes: 64 * 1024,
        input_count: 8,
        candidate_count: 8,
        diagnostic_count: 8,
        wall_timeout_ms: 2_000,
        cancellation_grace_ms: 100,
    })
    .unwrap()
}

fn outward_limits(limits: &DockProtocolLimitsV1) -> DockProtocolLimitsDtoV1 {
    DockProtocolLimitsDtoV1::new(DockProtocolLimitsInputV1 {
        request_bytes: limits.request_bytes() as u64,
        result_bytes: limits.result_bytes() as u64,
        stderr_bytes: limits.stderr_bytes() as u64,
        diagnostic_count: limits.diagnostic_count() as u64,
        diagnostic_bytes: limits.diagnostic_bytes() as u64,
        candidate_count: limits.candidate_count() as u64,
        candidate_bytes: limits.candidate_bytes() as u64,
        input_count: limits.input_count() as u64,
        wall_timeout_ms: limits.wall_timeout_ms(),
        cancellation_grace_ms: limits.cancellation_grace_ms(),
    })
    .unwrap()
}

fn outward_binding(binding: &OwnerBinding) -> ContractRecordBindingDtoV1 {
    ContractRecordBindingDtoV1::new(
        binding.reference().as_str(),
        outward_fingerprint(binding.fingerprint()),
    )
    .unwrap()
}

fn outward_fingerprint(fingerprint: &OwnerFingerprint) -> Fingerprint {
    Fingerprint::parse(fingerprint.as_str()).unwrap()
}

fn contract_value(value: &impl Serialize) -> ContractValue {
    parse_canonical_json(
        &serde_json_canonicalizer::to_vec(value).unwrap(),
        &ContractLimits::packet0(),
    )
    .unwrap()
}

fn digest(bytes: &[u8]) -> OwnerFingerprint {
    OwnerFingerprint::parse(&format!("sha256:{:x}", Sha256::digest(bytes))).unwrap()
}

fn owner_fp(hex: char) -> OwnerFingerprint {
    OwnerFingerprint::parse(&format!("sha256:{}", hex.to_string().repeat(64))).unwrap()
}

fn owner_binding(reference: &str, hex: char) -> OwnerBinding {
    OwnerBinding::new(OwnerRef::parse(reference).unwrap(), owner_fp(hex))
}

fn owner_selector(field: &str, expected: &str) -> SelectorV1 {
    SelectorV1::new(
        MatcherBindingV1::exact_fields_v1(),
        vec![SelectorCriterionV1::new(owner_id(field), expected).unwrap()],
    )
    .unwrap()
}

fn owner_id(value: &str) -> StableIdV1 {
    StableIdV1::parse(value).unwrap()
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
