use handbook_contracts::{
    lifecycle_policy_binding_v1, prepare_contract_definition_append, ApplicabilityRuleV1,
    ClaimVerdictDecisionV1, ContentAddressedBindingV1, ContractApprovalClassV1,
    ContractClaimInputV1, ContractClaimV1, ContractDefinitionInputV1, ContractDefinitionV1,
    ContractEvaluationRunV1, ContractIdV1, ContractKindV1, ContractLifecycleTransitionInputV1,
    ContractOwnerRepositoryV1, EvaluationCaseV1, EvaluationPolicyRegistryV1, EvaluationSubjectV1,
    EvidenceCardinalityV1, EvidenceCaseSelectionV1, EvidenceFreshnessPolicyInputV1,
    EvidenceFreshnessPolicyV1, EvidenceRequirementV1, ExactBindingV1, ExactRefV1, FingerprintV1,
    GateDecisionV1, GateEffectV1, GateEvaluationV1, GatePolicyAlgorithmV1, GatePolicyInputV1,
    GatePolicyV1, LifecycleEvidenceKindV1, LifecycleReasonV1, LifecycleStateV1,
    LifecycleSupportingEvidenceV1, LocalCloseoutDecisionV1, MatcherBindingV1, MatcherInputValueV1,
    ResolutionEnvelopeV1, ResolutionMinimumV1, SelectorCriterionV1, SelectorV1, StableIdV1,
    VersionV1,
};
use handbook_dock::{DockEvidenceCandidateSourceV1, DockManifestV1, DockProtocolLimitsV1};
use handbook_engine::{
    observe_committed_approver_registry, ApproverAdminRequestV1, ApproverRegistryServiceV1,
    ContextResolutionPolicyRegistry, ContextResolutionStackDefinition,
    CredentialAuthorityAdmissionServiceV1, EvidenceSourceAdmissionErrorV1,
    NativeAuthenticatorPortErrorV1, NativeAuthenticatorPortV1, SourceCurrentnessObservationV1,
    SourceCurrentnessPortV1, SourceCurrentnessRequestV1, AUTHENTICATOR_RP_ID,
};
use handbook_sdk::contract_membrane::{
    dock_run_operation_request, parse_canonical_json, BoundedString, BoundedVec, ContractLimits,
    ContractRecordBindingDtoV1, ContractValue, DockEvaluationPreparationDtoV1,
    DockInlineJsonInputDtoV1, DockProtocolLimitsDtoV1, DockProtocolLimitsInputV1, DockRunRequestV1,
    DockRunResultV1, Fingerprint, OperationRequestV1, OperationResponseV1, ProductionWitness,
    UtcTimestamp,
};
use p256::ecdsa::{signature::Signer, Signature, SigningKey};
use serde::Deserialize;
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::collections::BTreeSet;
use std::fs;
use std::path::{Path, PathBuf};

const REPOSITORY_IDENTITY: &str =
    "sha256:5454545454545454545454545454545454545454545454545454545454545454";
const EVIDENCE_KIND: &str = "handbook.evidence.schema-conformance";
const ROOT_SCHEMA_REF: &str = "handbook.schema.fixture.hcm-5-4-payload@1.0.0";
const ROOT_DOCUMENT: &str = "definitions/schemas/hcm-5-4/root.schema.json";
const SHARED_DOCUMENT: &str = "definitions/schemas/hcm-5-4/shared.schema.json";

#[derive(Clone, Deserialize)]
struct CredentialSpec {
    role: String,
    key_seed_byte: u8,
    credential_id_byte: u8,
    approval_class: String,
    authority_ref: String,
}

#[derive(Deserialize)]
struct ContractSpec {
    contract_id: String,
    contract_version: String,
    contract_kind: String,
    claim: ClaimSpec,
    resolution_minimum: ResolutionSpec,
}

#[derive(Deserialize)]
struct ClaimSpec {
    claim_id: String,
    statement: String,
    subject_selector: Value,
    case_selector: Value,
    applicability_rule: String,
    gate_effect: String,
    evidence_kind: String,
    evidence_cardinality: String,
    evidence_case_selection: String,
    stability_policy_ref: String,
    score_weight: Option<Value>,
}

#[derive(Deserialize)]
struct ResolutionSpec {
    scope_horizon: String,
    detail_resolution: String,
    temporal_horizon: String,
    authority_horizon: String,
    memory_horizon: String,
    validation_horizon: String,
}

#[derive(Deserialize)]
struct FreshnessSpec {
    policy_id: String,
    policy_version: String,
    max_observation_age_seconds: u64,
    require_exact_source_revision: bool,
    future_skew_seconds: u64,
}

#[derive(Deserialize)]
struct GateSpec {
    policy_id: String,
    policy_version: String,
    algorithm: String,
    score_threshold: Option<Value>,
    local_closeout_policy_ref: String,
}

pub struct Packet5Fixture {
    repo: tempfile::TempDir,
    owner: ContractOwnerRepositoryV1,
    service: CredentialAuthorityAdmissionServiceV1<FixtureAuthenticator>,
    policies: EvaluationPolicyRegistryV1,
    stack: ContextResolutionStackDefinition,
    resolution: ResolutionEnvelopeV1,
    freshness: ExactBindingV1,
    gate: ExactBindingV1,
    contract: ExactBindingV1,
    active_head: FingerprintV1,
    subject: EvaluationSubjectV1,
    case: EvaluationCaseV1,
    manifest: DockManifestV1,
    actor_refs: Vec<StableIdV1>,
}

impl Packet5Fixture {
    pub fn new(manifest: DockManifestV1) -> Self {
        let credentials: Vec<CredentialSpec> = fixture_json("credentials.json");
        assert_eq!(credentials.len(), 4);
        assert_eq!(
            credentials
                .iter()
                .map(|value| value.role.as_str())
                .collect::<Vec<_>>(),
            [
                "definition_author",
                "independent_lock",
                "activation",
                "evidence_source"
            ]
        );
        let actor_refs = credentials
            .iter()
            .map(|credential| actor_ref(credential.credential_id_byte))
            .collect::<Vec<_>>();
        assert_eq!(
            actor_refs
                .iter()
                .map(StableIdV1::as_str)
                .collect::<BTreeSet<_>>()
                .len(),
            4,
            "the four fixture credentials must derive pairwise-distinct actors"
        );

        let repo = tempfile::tempdir().unwrap();
        let mut registry = ApproverRegistryServiceV1::for_repository(
            repo.path(),
            FixtureAuthenticator::new(&credentials),
        );
        let author = &credentials[0];
        let bootstrap = ApproverAdminRequestV1::from_json_value(json!({
            "schema_id":"handbook.approver-admin-request",
            "schema_version":"1.0",
            "operation":"bootstrap",
            "operation_id":"hcm-5-4-packet5-author-bootstrap",
            "repository_identity_fingerprint":REPOSITORY_IDENTITY,
            "expected_registry_state_fingerprint":null,
            "expected_transition_fingerprint":null,
            "initial_charter_quorum":[{
                "approval_class":author.approval_class,
                "authority_ref":author.authority_ref
            }]
        }))
        .unwrap();
        let mut registry_result = registry
            .bootstrap_approver_registry(bootstrap)
            .to_json_value()
            .unwrap();
        assert_eq!(
            registry_result["status"], "succeeded",
            "{registry_result:#}"
        );

        for credential in &credentials[1..] {
            let mut mappings = vec![json!({
                "approval_class":credential.approval_class,
                "authority_ref":credential.authority_ref,
            })];
            if credential.role == "activation" {
                mappings.push(json!({
                    "approval_class":"contract_lifecycle",
                    "authority_ref":"handbook.authority.contract-lifecycle@1.0.0",
                }));
            }
            let add = ApproverAdminRequestV1::from_json_value(json!({
                "schema_id":"handbook.approver-admin-request",
                "schema_version":"1.0",
                "operation":"add_credential",
                "operation_id":format!("hcm-5-4-packet5-{}-add", credential.role),
                "repository_identity_fingerprint":REPOSITORY_IDENTITY,
                "expected_registry_state_fingerprint":registry_result["result_registry_state_fingerprint"],
                "expected_transition_fingerprint":registry_result["transition_fingerprint"],
                "approval_mappings":mappings,
            }))
            .unwrap();
            registry_result = registry
                .add_approver_credential(add)
                .to_json_value()
                .unwrap();
            assert_eq!(
                registry_result["status"], "succeeded",
                "{registry_result:#}"
            );
        }
        let registry_observation = observe_committed_approver_registry(repo.path()).unwrap();
        let registry_state_fingerprint = registry_observation.state_fingerprint.clone();
        drop(registry_observation);
        let service = CredentialAuthorityAdmissionServiceV1::new(repo.path(), registry.into_port());
        let owner = ContractOwnerRepositoryV1::open(repo.path()).unwrap();

        let (stack, stack_binding) = resolution_stack();
        let resolution =
            resolution_from_spec(&fixture_json("contract.json"), stack_binding.clone());
        assert_eq!(manifest.resolution_ceiling(), &resolution);

        let freshness_spec: FreshnessSpec = fixture_json("freshness-policy.json");
        let freshness = EvidenceFreshnessPolicyV1::new(
            EvidenceFreshnessPolicyInputV1::new(
                ContractIdV1::parse(&freshness_spec.policy_id).unwrap(),
                VersionV1::parse(&freshness_spec.policy_version).unwrap(),
                freshness_spec.max_observation_age_seconds,
                freshness_spec.require_exact_source_revision,
                freshness_spec.future_skew_seconds,
            )
            .unwrap(),
        )
        .unwrap();
        assert_eq!(
            freshness.policy_ref().as_str(),
            "handbook.contract-evidence-freshness-policy.hcm-5-4-proof@1.0.0"
        );

        let gate_spec: GateSpec = fixture_json("gate-policy.json");
        assert_eq!(gate_spec.algorithm, "closed_accounting");
        assert!(gate_spec.score_threshold.is_none());
        assert_eq!(
            gate_spec.local_closeout_policy_ref,
            handbook_contracts::local_closeout_policy_binding_v1()
                .unwrap()
                .reference()
                .as_str()
        );
        let gate = GatePolicyV1::new(
            GatePolicyInputV1::new(
                ContractIdV1::parse(&gate_spec.policy_id).unwrap(),
                VersionV1::parse(&gate_spec.policy_version).unwrap(),
                GatePolicyAlgorithmV1::ClosedAccounting,
                None,
                handbook_contracts::local_closeout_policy_binding_v1().unwrap(),
            )
            .unwrap(),
        )
        .unwrap();
        assert_eq!(
            gate.binding().reference().as_str(),
            "handbook.contract-gate-policy.hcm-5-4-proof@1.0.0"
        );
        let mut policies = EvaluationPolicyRegistryV1::new().unwrap();
        policies.register_freshness(freshness.clone()).unwrap();
        policies.register_gate(gate.clone()).unwrap();

        let contract_spec: ContractSpec = fixture_json("contract.json");
        let definition = definition_from_spec(
            &contract_spec,
            &credentials[0],
            &actor_refs[0],
            &registry_state_fingerprint,
            freshness.binding().clone(),
            gate.binding().clone(),
            stack_binding,
        );
        let definition_bytes = definition.canonical_json_bytes().unwrap();
        assert_eq!(
            serde_json_canonicalizer::to_vec(
                &serde_json::from_slice::<Value>(&definition_bytes).unwrap()
            )
            .unwrap(),
            definition_bytes
        );
        let contract_ref = definition.contract_ref().clone();
        let definition_fingerprint = definition.definition_fingerprint().clone();
        let mut fixture = Self {
            repo,
            owner,
            service,
            policies,
            stack,
            resolution,
            freshness: freshness.binding().clone(),
            gate: gate.binding().clone(),
            contract: ExactBindingV1::new(contract_ref, definition_fingerprint),
            active_head: fp('0'),
            subject: evaluation_subject(),
            case: evaluation_case(),
            manifest,
            actor_refs,
        };
        fixture.activate(definition);
        fixture
    }

    pub fn repository_path(&self) -> &Path {
        self.repo.path()
    }

    pub fn actor_refs(&self) -> &[StableIdV1] {
        &self.actor_refs
    }

    pub fn manifest(&self) -> &DockManifestV1 {
        &self.manifest
    }

    pub fn run_request(
        &self,
        nonce: &str,
        idempotency_key: &str,
        instance_file: &str,
    ) -> OperationRequestV1 {
        self.run_request_with(
            nonce,
            idempotency_key,
            validation_content(instance_file),
            self.resolution.clone(),
        )
    }

    pub fn substituted_instance_request(
        &self,
        nonce: &str,
        idempotency_key: &str,
    ) -> OperationRequestV1 {
        let mut content = validation_content("instances/conformant.json");
        content["instance"]["value"] = json!({"name":"substituted"});
        self.run_request_with(nonce, idempotency_key, content, self.resolution.clone())
    }

    pub fn under_resolved_request(&self, nonce: &str, idempotency_key: &str) -> OperationRequestV1 {
        let under = ResolutionEnvelopeV1::new(
            self.resolution.resolution_stack().clone(),
            id("resolution.scope.none"),
            id("resolution.detail.identifier-only"),
            id("resolution.time.current-operation"),
            id("resolution.authority.read-only"),
            id("resolution.memory.operation"),
            id("resolution.validation.observation-only"),
        );
        self.run_request_with(
            nonce,
            idempotency_key,
            validation_content("instances/conformant.json"),
            under,
        )
    }

    pub fn execute(&self, request: &OperationRequestV1) -> OperationResponseV1 {
        handbook_sdk::contract_membrane::execute_dock_operation_with_context(
            self.repo.path(),
            request,
            &self.policies,
        )
        .unwrap()
    }

    pub fn deprecate(&mut self) {
        let input = ContractLifecycleTransitionInputV1 {
            expected_lifecycle_fingerprint: self.active_head.clone(),
            from_state: LifecycleStateV1::Active,
            to_state: LifecycleStateV1::Deprecated,
            approval_class: ContractApprovalClassV1::Lifecycle,
            supporting_evidence: vec![LifecycleSupportingEvidenceV1::new(
                LifecycleEvidenceKindV1::NoReplacementRationale,
                fixture_binding(
                    "handbook.fixture.hcm-5-4.lifecycle.deprecation@1.0.0",
                    "contract.json",
                ),
            )],
            reason: LifecycleReasonV1::DeprecatedWithoutReplacement(
                handbook_contracts::LifecycleReasonTextV1::parse(
                    "No exact replacement is currently authorized.",
                )
                .unwrap(),
            ),
        };
        self.active_head = self.transition(input, "hcm-5-4-packet5-deprecate");
    }

    pub fn into_evidence(self) -> EvidenceFixture {
        let repo_path = self.repo.path().to_path_buf();
        EvidenceFixture {
            _repo: self.repo,
            owner: self.owner,
            source: DockEvidenceCandidateSourceV1::open(&repo_path, self.service.into_port())
                .unwrap(),
            policies: self.policies,
            stack: self.stack,
        }
    }

    fn run_request_with(
        &self,
        nonce: &str,
        idempotency_key: &str,
        content: Value,
        resolution: ResolutionEnvelopeV1,
    ) -> OperationRequestV1 {
        let evaluation = DockEvaluationPreparationDtoV1::new(
            outward_binding(&self.contract),
            outward_fingerprint(&self.active_head),
            contract_value(&self.subject),
            BoundedVec::new(vec![contract_value(&self.case)]).unwrap(),
            contract_value(&resolution),
            contract_value(&MatcherBindingV1::exact_fields_v1()),
            BoundedVec::new(vec![outward_binding(&self.freshness)]).unwrap(),
            BoundedVec::new(vec![outward_binding(
                &handbook_contracts::evidence_stability_policy_binding_v1().unwrap(),
            )])
            .unwrap(),
            outward_binding(&self.gate),
            outward_binding(&handbook_contracts::local_closeout_policy_binding_v1().unwrap()),
            BoundedString::new(nonce.to_owned()).unwrap(),
        );
        let body = DockRunRequestV1::new(
            outward_binding(&self.manifest.binding()),
            evaluation,
            BoundedVec::new(vec![BoundedString::new(EVIDENCE_KIND.to_owned()).unwrap()]).unwrap(),
            BoundedVec::new(vec![DockInlineJsonInputDtoV1::new(
                BoundedString::new("handbook.input.json-schema-validation".to_owned()).unwrap(),
                BoundedString::new("application/json".to_owned()).unwrap(),
                outward_binding(self.manifest.supported_inputs()[0].schema()),
                contract_value(&content),
            )])
            .unwrap(),
            outward_limits(self.manifest.limits()),
        );
        dock_run_operation_request(Some("hcm-5-4-packet5"), idempotency_key, body).unwrap()
    }

    fn activate(&mut self, definition: ContractDefinitionV1) {
        let prepared = prepare_contract_definition_append(definition).unwrap();
        let expected_actor = self.actor_refs[0].as_str().to_owned();
        let draft = self
            .service
            .admit(prepared.authority_request(), |admission| {
                assert_eq!(admission.actor_ref(), expected_actor);
                self.owner
                    .append_definition(prepared, admission, "hcm-5-4-packet5-definition")
            })
            .unwrap()
            .unwrap();
        let ready = self.transition(
            ContractLifecycleTransitionInputV1 {
                expected_lifecycle_fingerprint: draft
                    .lifecycle_head()
                    .lifecycle_fingerprint()
                    .clone(),
                from_state: LifecycleStateV1::Draft,
                to_state: LifecycleStateV1::ReviewReady,
                approval_class: ContractApprovalClassV1::DefinitionAuthor,
                supporting_evidence: vec![lifecycle_evidence(
                    LifecycleEvidenceKindV1::DefinitionValidation,
                    "contract.json",
                )],
                reason: LifecycleReasonV1::ReadyForReview,
            },
            "hcm-5-4-packet5-review-ready",
        );
        let locked = self.transition(
            ContractLifecycleTransitionInputV1 {
                expected_lifecycle_fingerprint: ready,
                from_state: LifecycleStateV1::ReviewReady,
                to_state: LifecycleStateV1::Locked,
                approval_class: ContractApprovalClassV1::IndependentLock,
                supporting_evidence: vec![lifecycle_evidence(
                    LifecycleEvidenceKindV1::IndependentReview,
                    "credentials.json",
                )],
                reason: LifecycleReasonV1::IndependentlyLocked,
            },
            "hcm-5-4-packet5-independent-lock",
        );
        let active = self.transition(
            ContractLifecycleTransitionInputV1 {
                expected_lifecycle_fingerprint: locked,
                from_state: LifecycleStateV1::Locked,
                to_state: LifecycleStateV1::Active,
                approval_class: ContractApprovalClassV1::Activation,
                supporting_evidence: vec![
                    lifecycle_evidence(
                        LifecycleEvidenceKindV1::ApplicabilityClosure,
                        "subject.json",
                    ),
                    lifecycle_evidence(LifecycleEvidenceKindV1::MatcherClosure, "case.json"),
                    lifecycle_evidence(LifecycleEvidenceKindV1::PolicyClosure, "gate-policy.json"),
                    lifecycle_evidence(
                        LifecycleEvidenceKindV1::CurrentnessClosure,
                        "freshness-policy.json",
                    ),
                ],
                reason: LifecycleReasonV1::Activated,
            },
            "hcm-5-4-packet5-activate",
        );
        self.active_head = active;
    }

    fn transition(
        &mut self,
        input: ContractLifecycleTransitionInputV1,
        key: &str,
    ) -> FingerprintV1 {
        let expected_actor = match input.approval_class {
            ContractApprovalClassV1::DefinitionAuthor => self.actor_refs[0].as_str(),
            ContractApprovalClassV1::IndependentLock => self.actor_refs[1].as_str(),
            ContractApprovalClassV1::Activation | ContractApprovalClassV1::Lifecycle => {
                self.actor_refs[2].as_str()
            }
        }
        .to_owned();
        let prepared = self
            .owner
            .prepare_lifecycle_transition(
                self.contract.reference(),
                self.contract.fingerprint(),
                input,
            )
            .unwrap();
        let execution = self
            .service
            .admit(prepared.authority_request(), |admission| {
                assert_eq!(admission.actor_ref(), expected_actor);
                self.owner.transition_lifecycle(prepared, admission, key)
            })
            .unwrap()
            .unwrap();
        execution.lifecycle_head().lifecycle_fingerprint().clone()
    }
}

pub struct EvidenceFixture {
    _repo: tempfile::TempDir,
    owner: ContractOwnerRepositoryV1,
    source: DockEvidenceCandidateSourceV1<FixtureAuthenticator>,
    policies: EvaluationPolicyRegistryV1,
    stack: ContextResolutionStackDefinition,
}

impl EvidenceFixture {
    pub fn repository_path(&self) -> &Path {
        self._repo.path()
    }

    pub fn append(
        &mut self,
        candidate: &ContentAddressedBindingV1,
        run: &ContractEvaluationRunV1,
        key: &str,
    ) -> handbook_contracts::EvidenceAppendExecutionV1 {
        let mut currentness = SourceCurrentness::stable(run);
        self.owner
            .append_evidence_candidate_v1(
                &mut self.source,
                &mut currentness,
                &self.stack,
                &self.policies,
                candidate,
                key,
            )
            .unwrap()
    }

    pub fn append_with_substituted_currentness(
        &mut self,
        candidate: &ContentAddressedBindingV1,
        run: &ContractEvaluationRunV1,
        key: &str,
    ) -> handbook_contracts::ContractOwnerRepositoryErrorKindV1 {
        let before = self.evidence_count();
        let mut currentness = SourceCurrentness::changing(run, 2);
        let error = self
            .owner
            .append_evidence_candidate_v1(
                &mut self.source,
                &mut currentness,
                &self.stack,
                &self.policies,
                candidate,
                key,
            )
            .unwrap_err();
        assert_eq!(self.evidence_count(), before);
        error.kind()
    }

    pub fn append_refusal(
        &mut self,
        candidate: &ContentAddressedBindingV1,
        run: &ContractEvaluationRunV1,
        key: &str,
    ) -> handbook_contracts::ContractOwnerRepositoryErrorKindV1 {
        let before = self.evidence_count();
        let mut currentness = SourceCurrentness::stable(run);
        let error = self
            .owner
            .append_evidence_candidate_v1(
                &mut self.source,
                &mut currentness,
                &self.stack,
                &self.policies,
                candidate,
                key,
            )
            .unwrap_err();
        assert_eq!(self.evidence_count(), before);
        error.kind()
    }

    pub fn evaluate(&mut self, run: &ContractEvaluationRunV1) -> GateEvaluationV1 {
        let mut currentness = SourceCurrentness::stable(run);
        handbook_contracts::evaluate_gate_and_local_closeout_v1(
            &self.owner,
            &mut self.source,
            &mut currentness,
            &self.stack,
            &self.policies,
            run,
        )
        .unwrap()
    }

    pub fn evidence_count(&self) -> usize {
        self.owner
            .list_evidence(handbook_contracts::EvidenceListRequestV1::first(32).unwrap())
            .unwrap()
            .entries()
            .len()
    }

    pub fn assert_outcome(
        evaluation: &GateEvaluationV1,
        verdict: ClaimVerdictDecisionV1,
        gate: GateDecisionV1,
        closeout: LocalCloseoutDecisionV1,
    ) {
        assert_eq!(evaluation.verdicts().len(), 1);
        assert_eq!(
            evaluation.verdicts()[0].claim_id().as_str(),
            "payload.schema-conformant"
        );
        assert_eq!(evaluation.verdicts()[0].decision(), verdict);
        assert_eq!(evaluation.gate_result().decision(), gate);
        assert_eq!(evaluation.local_closeout_result().decision(), closeout);
    }
}

pub fn owner_run(result: &DockRunResultV1) -> ContractEvaluationRunV1 {
    serde_json::from_value(serde_json::to_value(result.evaluation_run().document()).unwrap())
        .unwrap()
}

pub fn owner_candidate(result: &DockRunResultV1) -> ContentAddressedBindingV1 {
    let candidate = result
        .candidates()
        .first()
        .expect("completed validator run must expose one candidate");
    ContentAddressedBindingV1::new(
        candidate.reference(),
        FingerprintV1::parse(candidate.fingerprint().as_str()).unwrap(),
    )
    .unwrap()
}

pub fn parse_run_result(response: &OperationResponseV1) -> DockRunResultV1 {
    parse_canonical_json(
        &handbook_sdk::contract_membrane::canonical_json_bytes(
            response.terminal().data().unwrap(),
            &ContractLimits::packet0(),
        )
        .unwrap(),
        &ContractLimits::packet0(),
    )
    .unwrap()
}

pub fn candidate_file_count(repo: &Path) -> usize {
    file_count(&repo.join(".handbook/state/dock/v1/candidates"))
}

pub fn evidence_file_count(repo: &Path) -> usize {
    file_count(&repo.join(".handbook/state/contracts/v1/evidence-v1/records"))
}

pub fn derived_state_is_absent(repo: &Path) -> bool {
    let contracts = repo.join(".handbook/state/contracts/v1");
    !["verdict", "gate", "local-closeout"]
        .iter()
        .any(|name| contracts.join(name).exists())
}

fn file_count(path: &Path) -> usize {
    fs::read_dir(path)
        .map(|entries| entries.count())
        .unwrap_or(0)
}

struct SourceCurrentness {
    observed_at: String,
    calls: usize,
    change_after: Option<usize>,
}

impl SourceCurrentness {
    fn stable(run: &ContractEvaluationRunV1) -> Self {
        Self {
            observed_at: run
                .owner_currentness()
                .semantic_time_utc()
                .as_str()
                .to_owned(),
            calls: 0,
            change_after: None,
        }
    }

    fn changing(run: &ContractEvaluationRunV1, change_after: usize) -> Self {
        Self {
            observed_at: run
                .owner_currentness()
                .semantic_time_utc()
                .as_str()
                .to_owned(),
            calls: 0,
            change_after: Some(change_after),
        }
    }
}

impl SourceCurrentnessPortV1 for SourceCurrentness {
    fn observe_current(
        &mut self,
        request: &SourceCurrentnessRequestV1,
    ) -> Result<SourceCurrentnessObservationV1, EvidenceSourceAdmissionErrorV1> {
        self.calls += 1;
        let fingerprint = if self.change_after.is_some_and(|call| self.calls > call) {
            fp('e').as_str().to_owned()
        } else {
            request.revision().to_owned()
        };
        SourceCurrentnessObservationV1::new(
            request,
            &format!("{}#current", request.source_ref()),
            &fingerprint,
            &self.observed_at,
        )
    }
}

fn definition_from_spec(
    spec: &ContractSpec,
    author: &CredentialSpec,
    author_actor: &StableIdV1,
    registry_state_fingerprint: &str,
    freshness: ExactBindingV1,
    gate: ExactBindingV1,
    stack: ExactBindingV1,
) -> ContractDefinitionV1 {
    assert_eq!(spec.contract_kind, "claim_set");
    assert_eq!(spec.claim.applicability_rule, "subject_and_case_match");
    assert_eq!(spec.claim.gate_effect, "hard_fail");
    assert_eq!(spec.claim.evidence_kind, EVIDENCE_KIND);
    assert_eq!(spec.claim.evidence_cardinality, "exactly_one");
    assert_eq!(spec.claim.evidence_case_selection, "claim_case_selector");
    assert_eq!(
        spec.claim.stability_policy_ref,
        handbook_contracts::evidence_stability_policy_binding_v1()
            .unwrap()
            .reference()
            .as_str()
    );
    assert!(spec.claim.score_weight.is_none());
    let subject = spec.claim.subject_selector.as_object().unwrap();
    let case = spec.claim.case_selector.as_object().unwrap();
    assert_eq!(subject.len(), 1);
    assert_eq!(case.len(), 1);
    let subject_selector = selector(
        subject.keys().next().unwrap(),
        subject.values().next().unwrap().as_str().unwrap(),
    );
    let case_selector = selector(
        case.keys().next().unwrap(),
        case.values().next().unwrap().as_str().unwrap(),
    );
    ContractDefinitionV1::new(ContractDefinitionInputV1 {
        definition_schema: production_schema_binding("handbook.contract-definition@1.0.0"),
        contract_id: ContractIdV1::parse(&spec.contract_id).unwrap(),
        contract_version: VersionV1::parse(&spec.contract_version).unwrap(),
        contract_kind: ContractKindV1::ClaimSet,
        claims: vec![ContractClaimV1::new(ContractClaimInputV1 {
            claim_id: id(&spec.claim.claim_id),
            statement: spec.claim.statement.clone(),
            subject_selector,
            case_selector,
            applicability_rule: ApplicabilityRuleV1::SubjectAndCaseMatch,
            gate_effect: GateEffectV1::HardFail,
            evidence_requirements: vec![EvidenceRequirementV1::new(
                id(EVIDENCE_KIND),
                EvidenceCardinalityV1::ExactlyOne,
                EvidenceCaseSelectionV1::ClaimCaseSelector,
                handbook_contracts::evidence_stability_policy_binding_v1().unwrap(),
            )
            .unwrap()],
            freshness_policy: freshness,
            minimum_resolution: ResolutionMinimumV1::new(
                stack,
                id(&spec.resolution_minimum.scope_horizon),
                id(&spec.resolution_minimum.detail_resolution),
                id(&spec.resolution_minimum.temporal_horizon),
                id(&spec.resolution_minimum.authority_horizon),
                id(&spec.resolution_minimum.memory_horizon),
                id(&spec.resolution_minimum.validation_horizon),
            ),
            score_weight: None,
        })
        .unwrap()],
        definition_author_ref: author_actor.clone(),
        definition_authority: ExactBindingV1::new(
            ExactRefV1::parse(&author.authority_ref).unwrap(),
            FingerprintV1::parse(registry_state_fingerprint).unwrap(),
        ),
        lifecycle_policy: lifecycle_policy_binding_v1().unwrap(),
        matcher_bindings: vec![MatcherBindingV1::exact_fields_v1()],
        gate_policy: gate,
        extensions_policy: fixture_binding(
            "handbook.contract-extensions.closed-v1@1.0.0",
            "extensions-policy.json",
        ),
    })
    .unwrap()
}

fn production_schema_binding(reference: &str) -> ExactBindingV1 {
    let witness =
        ProductionWitness::from_current(UtcTimestamp::parse("9999-12-31T23:59:59Z").unwrap())
            .unwrap();
    let schema = witness
        .schema_manifest()
        .iter()
        .find(|entry| entry.schema().reference().as_str() == reference)
        .unwrap()
        .schema();
    ExactBindingV1::new(
        ExactRefV1::parse(schema.reference().as_str()).unwrap(),
        FingerprintV1::parse(schema.fingerprint().as_str()).unwrap(),
    )
}

fn resolution_from_spec(spec: &ContractSpec, stack: ExactBindingV1) -> ResolutionEnvelopeV1 {
    ResolutionEnvelopeV1::new(
        stack,
        id(&spec.resolution_minimum.scope_horizon),
        id(&spec.resolution_minimum.detail_resolution),
        id(&spec.resolution_minimum.temporal_horizon),
        id(&spec.resolution_minimum.authority_horizon),
        id(&spec.resolution_minimum.memory_horizon),
        id(&spec.resolution_minimum.validation_horizon),
    )
}

fn resolution_stack() -> (ContextResolutionStackDefinition, ExactBindingV1) {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../engine")
        .canonicalize()
        .unwrap();
    let policies = ContextResolutionPolicyRegistry::load(
        &root,
        &[
            "definitions/context-resolution-policies/handbook.mutation-matcher.core/1.0.0.yaml".to_owned(),
            "definitions/context-resolution-policies/handbook.resolution-escalation.core/1.0.0.yaml".to_owned(),
            "definitions/context-resolution-policies/handbook.memory-promotion.core/1.0.0.yaml".to_owned(),
        ],
    )
    .unwrap();
    let stack = ContextResolutionStackDefinition::load(
        &root,
        "definitions/context-resolution/handbook.context-resolution.shipped-root/1.0.0.yaml",
        &policies,
    )
    .unwrap();
    let binding = ExactBindingV1::new(
        ExactRefV1::parse(stack.exact_ref().as_str()).unwrap(),
        FingerprintV1::parse(stack.definition_fingerprint().as_str()).unwrap(),
    );
    (stack, binding)
}

fn evaluation_subject() -> EvaluationSubjectV1 {
    let value: Value = fixture_json("subject.json");
    EvaluationSubjectV1::new(
        content_binding("subject", "subject.json"),
        vec![MatcherInputValueV1::new(
            id("artifact.kind"),
            value["artifact.kind"].as_str().unwrap(),
        )
        .unwrap()],
    )
    .unwrap()
}

fn evaluation_case() -> EvaluationCaseV1 {
    let value: Value = fixture_json("case.json");
    EvaluationCaseV1::new(
        id("case.schema-conformance"),
        content_binding("case", "case.json"),
        vec![MatcherInputValueV1::new(
            id("validation.case"),
            value["validation.case"].as_str().unwrap(),
        )
        .unwrap()],
    )
    .unwrap()
}

fn validation_content(instance_file: &str) -> Value {
    let root: Value = fixture_json(ROOT_DOCUMENT);
    let shared: Value = fixture_json(SHARED_DOCUMENT);
    let instance: Value = fixture_json(instance_file);
    let root_fingerprint = validator_fingerprint(&root);
    let shared_fingerprint = validator_fingerprint(&shared);
    let members = json!([
        {"path":ROOT_DOCUMENT,"document_fingerprint":root_fingerprint},
        {"path":SHARED_DOCUMENT,"document_fingerprint":shared_fingerprint}
    ]);
    let closure_fingerprint = validator_fingerprint(&members);
    let instance_fingerprint = validator_fingerprint(&instance);
    json!({
        "root_schema":{"ref":ROOT_SCHEMA_REF,"fingerprint":root_fingerprint},
        "root_document":ROOT_DOCUMENT,
        "root_document_fingerprint":root_fingerprint,
        "closure_fingerprint":closure_fingerprint,
        "documents":[
            {"path":ROOT_DOCUMENT,"schema":root,"document_fingerprint":root_fingerprint},
            {"path":SHARED_DOCUMENT,"schema":shared,"document_fingerprint":shared_fingerprint}
        ],
        "instance":{"value":instance,"instance_fingerprint":instance_fingerprint}
    })
}

fn validator_fingerprint(value: &Value) -> String {
    handbook_dock_json_schema::CanonicalFingerprintV1::for_value(value)
        .unwrap()
        .as_str()
        .to_owned()
}

fn lifecycle_evidence(
    kind: LifecycleEvidenceKindV1,
    fixture: &str,
) -> LifecycleSupportingEvidenceV1 {
    LifecycleSupportingEvidenceV1::new(
        kind,
        fixture_binding("handbook.fixture.hcm-5-4.lifecycle-evidence@1.0.0", fixture),
    )
}

fn fixture_binding(reference: &str, fixture: &str) -> ExactBindingV1 {
    ExactBindingV1::new(
        ExactRefV1::parse(reference).unwrap(),
        fixture_fingerprint(fixture),
    )
}

fn content_binding(kind: &str, fixture: &str) -> ContentAddressedBindingV1 {
    let fingerprint = fixture_fingerprint(fixture);
    ContentAddressedBindingV1::new(
        &format!(
            "handbook.fixture.hcm-5-4-json-schema-dock@1.0.0#{kind}/{}",
            &fingerprint.as_str()[7..]
        ),
        fingerprint,
    )
    .unwrap()
}

fn fixture_fingerprint(relative: &str) -> FingerprintV1 {
    let value: Value = fixture_json(relative);
    FingerprintV1::parse(&format!(
        "sha256:{:x}",
        Sha256::digest(serde_json_canonicalizer::to_vec(&value).unwrap())
    ))
    .unwrap()
}

fn fixture_json<T: serde::de::DeserializeOwned>(relative: &str) -> T {
    serde_json::from_slice(&fs::read(fixture_root().join(relative)).unwrap()).unwrap()
}

fn fixture_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../../tests/fixtures/hcm_5_4_json_schema_dock")
}

fn selector(field: &str, expected: &str) -> SelectorV1 {
    SelectorV1::new(
        MatcherBindingV1::exact_fields_v1(),
        vec![SelectorCriterionV1::new(id(field), expected).unwrap()],
    )
    .unwrap()
}

fn outward_binding(binding: &ExactBindingV1) -> ContractRecordBindingDtoV1 {
    ContractRecordBindingDtoV1::new(
        binding.reference().as_str(),
        outward_fingerprint(binding.fingerprint()),
    )
    .unwrap()
}

fn outward_fingerprint(fingerprint: &FingerprintV1) -> Fingerprint {
    Fingerprint::parse(fingerprint.as_str()).unwrap()
}

fn contract_value(value: &impl serde::Serialize) -> ContractValue {
    parse_canonical_json(
        &serde_json_canonicalizer::to_vec(value).unwrap(),
        &ContractLimits::packet0(),
    )
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

fn id(value: &str) -> StableIdV1 {
    StableIdV1::parse(value).unwrap()
}

fn fp(byte: char) -> FingerprintV1 {
    FingerprintV1::parse(&format!("sha256:{}", byte.to_string().repeat(64))).unwrap()
}

fn actor_ref(credential_id_byte: u8) -> StableIdV1 {
    id(&format!(
        "credential.sha256-{:x}",
        Sha256::digest([credential_id_byte; 32])
    ))
}

struct FixtureCredential {
    key: SigningKey,
    credential_id: Vec<u8>,
    assertions: u32,
}

struct FixtureAuthenticator {
    credentials: Vec<FixtureCredential>,
    make_index: usize,
}

impl FixtureAuthenticator {
    fn new(specs: &[CredentialSpec]) -> Self {
        Self {
            credentials: specs
                .iter()
                .map(|spec| FixtureCredential {
                    key: SigningKey::from_slice(&[spec.key_seed_byte; 32]).unwrap(),
                    credential_id: vec![spec.credential_id_byte; 32],
                    assertions: 0,
                })
                .collect(),
            make_index: 0,
        }
    }
}

impl NativeAuthenticatorPortV1 for FixtureAuthenticator {
    fn make_credential(
        &mut self,
        _request_cbor: &[u8],
    ) -> Result<Vec<u8>, NativeAuthenticatorPortErrorV1> {
        let credential = self.credentials.get(self.make_index).unwrap();
        self.make_index += 1;
        Ok(make_credential_response(
            &credential.key,
            &credential.credential_id,
        ))
    }

    fn get_assertion(
        &mut self,
        request_cbor: &[u8],
    ) -> Result<Vec<u8>, NativeAuthenticatorPortErrorV1> {
        let client_hash_marker = request_cbor
            .windows(3)
            .position(|window| window == [0x02, 0x58, 0x20])
            .unwrap();
        let client_data_hash = request_cbor[client_hash_marker + 3..client_hash_marker + 35]
            .try_into()
            .unwrap();
        let credential = self
            .credentials
            .iter_mut()
            .find(|credential| {
                request_cbor
                    .windows(credential.credential_id.len())
                    .any(|window| window == credential.credential_id)
            })
            .unwrap();
        credential.assertions += 1;
        Ok(assertion_response(
            &credential.key,
            &credential.credential_id,
            client_data_hash,
            credential.assertions,
        ))
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
