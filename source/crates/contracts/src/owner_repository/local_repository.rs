//! Local records reuse the enclosing owner lock, retained-root operations and
//! three-output decided transaction/recovery. V1 record bytes stay unchanged.
use super::*;
use crate::local_contract::resolve_local_applicability_v2;
use crate::project_admission_governance::ProjectAdmissionBindingV3;
use crate::{
    LocalContractDefinitionV2, LocalContractUseIntentV2, LocalContractUseV2, LocalReviewDecisionV2,
    LocalReviewInputV2,
};
use handbook_engine::local_authority::local_check::{
    LocalCheckActionRequestV1, LocalCheckActionV1, LocalCheckAdmissionV1, LocalCheckCurrentBasisV1,
    LocalCheckLocatorV1,
};
use handbook_engine::ContextResolutionStackDefinition;

pub const LOCAL_CONTRACT_OWNER_ROOT_V2: &str = ".handbook/state/contracts/v2";
const APPEND: &str = "contract.local-definition.append";
const TRANSITION: &str = "contract.local-lifecycle.transition";

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
struct LocalAuthorityRecordV2 {
    request: LocalCheckActionRequestV1,
    locator: LocalCheckLocatorV1,
    current_basis: LocalCheckCurrentBasisV1,
    authority_fingerprint: String,
}
impl LocalAuthorityRecordV2 {
    fn from_admission(a: &LocalCheckAdmissionV1<'_>) -> Self {
        Self {
            request: a.request().clone(),
            locator: a.locator().clone(),
            current_basis: a.current_basis().clone(),
            authority_fingerprint: a.authority_fingerprint().into(),
        }
    }
}
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct LocalLifecycleHeadV2 {
    definition: ExactBindingV1,
    state: LifecycleStateV1,
    sequence: usize,
    transition_ref: Option<String>,
    lifecycle_fingerprint: FingerprintV1,
    author_participant_ref: String,
    approving_review: Option<LocalReviewInputV2>,
}
impl LocalLifecycleHeadV2 {
    pub fn state(&self) -> LifecycleStateV1 {
        self.state
    }
    pub fn lifecycle_fingerprint(&self) -> &FingerprintV1 {
        &self.lifecycle_fingerprint
    }
    pub fn approving_review(&self) -> Option<&LocalReviewInputV2> {
        self.approving_review.as_ref()
    }
    fn basis(&self) -> Result<crate::ContentAddressedBindingV1, ContractOwnerRepositoryErrorV1> {
        crate::ContentAddressedBindingV1::new(
            &format!(
                "{}#local-reviewed-basis/{}",
                self.definition.reference(),
                fingerprint_hex(&self.lifecycle_fingerprint)
            ),
            canonical_fingerprint(&("handbook.local-reviewed-lifecycle-basis@2.0.0", self))
                .map_err(map_contract_input)?,
        )
        .map_err(map_contract_input)
    }
    fn v1_projection(&self) -> LifecycleHeadV1 {
        LifecycleHeadV1::from_local_v2(
            self.state,
            self.transition_ref.clone(),
            self.lifecycle_fingerprint.clone(),
        )
    }
}
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct LocalDefinitionRecordV2 {
    schema_ref: String,
    definition: LocalContractDefinitionV2,
    author_admission: LocalAuthorityRecordV2,
    owner_lock_fingerprint: String,
    genesis: LocalLifecycleHeadV2,
}
impl LocalDefinitionRecordV2 {
    pub fn definition(&self) -> &LocalContractDefinitionV2 {
        &self.definition
    }
    pub fn genesis(&self) -> &LocalLifecycleHeadV2 {
        &self.genesis
    }
    fn validate(&self) -> Result<(), ContractOwnerRepositoryErrorV1> {
        self.definition.validate().map_err(map_contract_input)?;
        if self.schema_ref != "handbook.contract-local-definition-record@2.0.0"
            || self.genesis != genesis(&self.definition, &self.author_admission)?
            || self.author_admission.request.actor_ref
                != self
                    .definition
                    .semantic_definition()
                    .definition_author_ref()
                    .as_str()
            || self.author_admission.request.participant_ref
                != self.author_admission.locator.terms.author_participant_ref
        {
            return Err(recovery(
                "local definition provenance or genesis is substituted",
            ));
        }
        validate_historical_admission(
            &self.author_admission,
            &self.definition,
            LocalCheckActionV1::DefinitionAuthor,
        )?;
        Ok(())
    }
}
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct LocalLifecycleTransitionInputV2 {
    pub expected_lifecycle_fingerprint: FingerprintV1,
    pub to_state: LifecycleStateV1,
    pub review: Option<LocalReviewInputV2>,
    pub reason: String,
}
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
struct LocalTransitionRecordV2 {
    schema_ref: String,
    input: LocalLifecycleTransitionInputV2,
    previous: LocalLifecycleHeadV2,
    resulting: LocalLifecycleHeadV2,
    admission: LocalAuthorityRecordV2,
}
#[derive(Clone, Debug)]
pub struct PreparedLocalDefinitionAppendV2 {
    definition: LocalContractDefinitionV2,
    applicability: crate::LocalContractApplicabilityV2,
    current_basis: LocalCheckCurrentBasisV1,
}
impl PreparedLocalDefinitionAppendV2 {
    pub fn definition(&self) -> &LocalContractDefinitionV2 {
        &self.definition
    }
    pub fn current_basis(&self) -> &LocalCheckCurrentBasisV1 {
        &self.current_basis
    }
    pub fn applicability(&self) -> &crate::LocalContractApplicabilityV2 {
        &self.applicability
    }
    pub fn authority_request(
        &self,
        actor_ref: String,
        participant_ref: String,
        semantic_time: SemanticTimeV1,
    ) -> LocalCheckActionRequestV1 {
        action_request(
            &self.definition,
            LocalCheckActionV1::DefinitionAuthor,
            actor_ref,
            participant_ref,
            semantic_time,
            self.definition.binding().fingerprint().clone(),
        )
    }
}
#[derive(Clone, Debug)]
pub struct PreparedLocalLifecycleTransitionV2 {
    definition: LocalContractDefinitionV2,
    input: LocalLifecycleTransitionInputV2,
    expected: LocalLifecycleHeadV2,
    current_basis: LocalCheckCurrentBasisV1,
}
impl PreparedLocalLifecycleTransitionV2 {
    pub fn current_basis(&self) -> &LocalCheckCurrentBasisV1 {
        &self.current_basis
    }
    pub fn authority_request(
        &self,
        actor_ref: String,
        participant_ref: String,
        semantic_time: SemanticTimeV1,
    ) -> Result<LocalCheckActionRequestV1, ContractOwnerRepositoryErrorV1> {
        let action = if self.input.to_state == LifecycleStateV1::Locked {
            LocalCheckActionV1::Review
        } else {
            LocalCheckActionV1::Lifecycle
        };
        Ok(action_request(
            &self.definition,
            action,
            actor_ref,
            participant_ref,
            semantic_time,
            canonical_fingerprint(&(
                "handbook.local-lifecycle-action@2.0.0",
                &self.definition.binding(),
                &self.input,
            ))
            .map_err(map_contract_input)?,
        ))
    }
}
fn action_request(
    d: &LocalContractDefinitionV2,
    action: LocalCheckActionV1,
    actor_ref: String,
    participant_ref: String,
    time: SemanticTimeV1,
    content: FingerprintV1,
) -> LocalCheckActionRequestV1 {
    LocalCheckActionRequestV1 {
        action,
        action_ref: match action {
            LocalCheckActionV1::DefinitionAuthor => {
                "handbook.contract-local-definition-append@2.0.0"
            }
            LocalCheckActionV1::Review => "handbook.contract-local-review@2.0.0",
            _ => "handbook.contract-local-lifecycle-transition@2.0.0",
        }
        .into(),
        subject_ref: d.binding().reference().as_str().into(),
        content_fingerprint: content.as_str().into(),
        closure_fingerprint: d.closure_fingerprint().as_str().into(),
        actor_ref,
        participant_ref,
        semantic_time_utc: time.as_str().into(),
        owner_write_paths: vec![
            format!("{CONTRACT_OWNER_REPOSITORY_RELATIVE_ROOT_V1}/transactions"),
            LOCAL_CONTRACT_OWNER_ROOT_V2.into(),
        ],
        input_paths: vec![],
    }
}
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct LocalOwnerMutationReceiptV2 {
    pub disposition: OwnerMutationDispositionV1,
    pub definition: ExactBindingV1,
    pub request_fingerprint: FingerprintV1,
    pub record_ref: String,
    pub record_fingerprint: FingerprintV1,
    pub lifecycle_head: LocalLifecycleHeadV2,
}
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
struct LocalLedgerV2 {
    operation: String,
    key_fingerprint: FingerprintV1,
    receipt: LocalOwnerMutationReceiptV2,
}

impl ContractOwnerRepositoryV1 {
    /// Existing source/history reads may not initialize or recover owner state.
    pub fn open_existing_for_local_use_v2(
        root: impl AsRef<Path>,
    ) -> Result<Self, ContractOwnerRepositoryErrorV1> {
        let root = root.as_ref();
        let owner = Self {
            repo_root: root.to_path_buf(),
            repository_root: RetainedRepositoryRootV1::open_exact(root).map_err(map_durability)?,
            #[cfg(feature = "test-support")]
            fault: None,
        };
        owner.with_existing_local_lock(|o| {
            o.validate_inventory()?;
            o.validate_local_inventory()
        })?;
        Ok(owner)
    }
    fn with_existing_local_lock<T>(
        &self,
        f: impl FnOnce(&Self) -> Result<T, ContractOwnerRepositoryErrorV1>,
    ) -> Result<T, ContractOwnerRepositoryErrorV1> {
        let _process = OWNER_PROCESS_LOCK
            .lock()
            .map_err(|_| durability("process owner lock is uncertain"))?;
        let lock = self
            .repository_root
            .acquire_existing_exclusive_lock_v1(LOCK_RELATIVE_PATH)
            .map_err(map_durability)?;
        lock.validate_at(&self.repository_root, LOCK_RELATIVE_PATH)
            .map_err(map_durability)?;
        let result = f(self);
        let current = lock
            .validate_at(&self.repository_root, LOCK_RELATIVE_PATH)
            .map_err(map_durability);
        let release = lock.release().map_err(map_durability);
        match (result, current, release) {
            (Ok(value), Ok(()), Ok(())) => Ok(value),
            (Err(e), _, _) | (Ok(_), Err(e), _) | (Ok(_), Ok(_), Err(e)) => Err(e),
        }
    }
    pub fn prepare_local_definition_append_v2(
        &self,
        project: &ProjectAdmissionBindingV3,
        definition: LocalContractDefinitionV2,
        stack: &ContextResolutionStackDefinition,
    ) -> Result<PreparedLocalDefinitionAppendV2, ContractOwnerRepositoryErrorV1> {
        definition
            .validate_resolution(stack)
            .map_err(map_contract_input)?;
        let (applicability, current_basis) =
            resolve_local_applicability_v2(&self.repo_root, project, &definition)
                .map_err(map_contract_input)?;
        self.with_existing_local_lock(|o| {
            o.validate_inventory()?;
            Ok(())
        })?;
        Ok(PreparedLocalDefinitionAppendV2 {
            definition,
            applicability,
            current_basis,
        })
    }
    pub fn append_local_definition_v2(
        &self,
        prepared: PreparedLocalDefinitionAppendV2,
        admission: &LocalCheckAdmissionV1<'_>,
        key: &str,
    ) -> Result<LocalOwnerMutationReceiptV2, ContractOwnerRepositoryErrorV1> {
        validate_key(key)?;
        let d = &prepared.definition;
        validate_live_admission(
            self,
            admission,
            d,
            LocalCheckActionV1::DefinitionAuthor,
            &prepared.current_basis,
            d.binding().fingerprint(),
        )?;
        if admission.request().actor_ref != d.semantic_definition().definition_author_ref().as_str()
            || admission.request().participant_ref != admission.terms().author_participant_ref
        {
            return Err(invalid("actual author participant is not exact"));
        }
        if d.semantic_definition()
            .definition_authority()
            .reference()
            .as_str()
            != admission.terms().rule_ref
            || d.semantic_definition()
                .definition_authority()
                .fingerprint()
                .as_str()
                != admission.authority_fingerprint()
        {
            return Err(invalid(
                "local immutable author authority is not the actual supplied admission",
            ));
        }
        self.with_existing_local_lock(|o| {
            o.recover_locked()?;
            o.validate_inventory()?;
            let request =
                canonical_fingerprint(&(APPEND, &d.binding())).map_err(map_contract_input)?;
            let key_fp = canonical_fingerprint(&("handbook.local-owner-key@2.0.0", key))
                .map_err(map_contract_input)?;
            if let Some(receipt) = o.local_replay(&key_fp, &request)? {
                admission
                    .validate_current()
                    .map_err(|_| invalid("authority changed before local replay"))?;
                return Ok(receipt);
            }
            if o.exists(&o.local_head_path(&d.binding()))? {
                return Err(substituted("local exact definition is already immutable"));
            }
            let author = LocalAuthorityRecordV2::from_admission(admission);
            let head = genesis(d, &author)?;
            // Bind the already-held original owner lock rather than creating one.
            let lock_binding = o
                .repository_root
                .existing_lock_physical_fingerprint_v1(LOCK_RELATIVE_PATH)
                .map_err(map_durability)?;
            let record = LocalDefinitionRecordV2 {
                schema_ref: "handbook.contract-local-definition-record@2.0.0".into(),
                definition: d.clone(),
                author_admission: author,
                owner_lock_fingerprint: lock_binding,
                genesis: head.clone(),
            };
            record.validate()?;
            o.commit_local_mutation(
                APPEND,
                &d.binding(),
                &head,
                canonical_bytes(&record).map_err(map_contract_input)?,
                request,
                key_fp,
                None,
                admission,
            )
        })
    }
    pub fn read_local_definition_v2(
        &self,
        binding: &ExactBindingV1,
    ) -> Result<LocalDefinitionRecordV2, ContractOwnerRepositoryErrorV1> {
        self.with_existing_local_lock(|o| {
            o.validate_inventory()?;
            o.local_bundle(binding).map(|b| b.0)
        })
    }
    pub fn current_local_lifecycle_head_v2(
        &self,
        binding: &ExactBindingV1,
    ) -> Result<LocalLifecycleHeadV2, ContractOwnerRepositoryErrorV1> {
        self.with_existing_local_lock(|o| {
            o.validate_inventory()?;
            o.local_bundle(binding).map(|b| b.1)
        })
    }
    pub fn prepare_local_lifecycle_transition_v2(
        &self,
        project: &ProjectAdmissionBindingV3,
        binding: &ExactBindingV1,
        input: LocalLifecycleTransitionInputV2,
    ) -> Result<PreparedLocalLifecycleTransitionV2, ContractOwnerRepositoryErrorV1> {
        let (record, head) = self.with_existing_local_lock(|o| {
            o.validate_inventory()?;
            o.local_bundle(binding)
        })?;
        validate_local_edge(&record, &head, &input)?;
        let (_, current_basis) =
            resolve_local_applicability_v2(&self.repo_root, project, &record.definition)
                .map_err(map_contract_input)?;
        Ok(PreparedLocalLifecycleTransitionV2 {
            definition: record.definition,
            input,
            expected: head,
            current_basis,
        })
    }
    pub fn transition_local_lifecycle_v2(
        &self,
        prepared: PreparedLocalLifecycleTransitionV2,
        admission: &LocalCheckAdmissionV1<'_>,
        key: &str,
    ) -> Result<LocalOwnerMutationReceiptV2, ContractOwnerRepositoryErrorV1> {
        validate_key(key)?;
        let action = if prepared.input.to_state == LifecycleStateV1::Locked {
            LocalCheckActionV1::Review
        } else {
            LocalCheckActionV1::Lifecycle
        };
        let content = canonical_fingerprint(&(
            "handbook.local-lifecycle-action@2.0.0",
            &prepared.definition.binding(),
            &prepared.input,
        ))
        .map_err(map_contract_input)?;
        validate_live_admission(
            self,
            admission,
            &prepared.definition,
            action,
            &prepared.current_basis,
            &content,
        )?;
        self.with_existing_local_lock(|o| {
            o.recover_locked()?;
            o.validate_inventory()?;
            let request = canonical_fingerprint(&(
                TRANSITION,
                &prepared.definition.binding(),
                &prepared.input,
            ))
            .map_err(map_contract_input)?;
            let key_fp = canonical_fingerprint(&("handbook.local-owner-key@2.0.0", key))
                .map_err(map_contract_input)?;
            if let Some(receipt) = o.local_replay(&key_fp, &request)? {
                admission
                    .validate_current()
                    .map_err(|_| invalid("authority changed before local replay"))?;
                return Ok(receipt);
            }
            let (definition, current) = o.local_bundle(&prepared.definition.binding())?;
            if current != prepared.expected {
                return Err(substituted("local expected lifecycle head is stale"));
            }
            validate_local_edge(&definition, &current, &prepared.input)?;
            if action == LocalCheckActionV1::Review {
                let review = prepared
                    .input
                    .review
                    .as_ref()
                    .ok_or_else(|| invalid("actual review missing"))?;
                if review.reviewer_participant_ref != admission.request().participant_ref
                    || review.reviewer_actor_ref != admission.request().actor_ref
                    || review.semantic_time.as_str() != admission.request().semantic_time_utc
                    || review.reviewer_actor_ref == definition.author_admission.request.actor_ref
                    || !admission
                        .terms()
                        .review_participant_refs
                        .contains(&review.reviewer_participant_ref)
                {
                    return Err(invalid(
                        "actual reviewer is not selected distinct participant",
                    ));
                }
            } else if admission.request().participant_ref
                != definition.genesis.author_participant_ref
                && !admission
                    .terms()
                    .review_participant_refs
                    .contains(&admission.request().participant_ref)
            {
                return Err(invalid("lifecycle participant is not selected"));
            }
            let authority = LocalAuthorityRecordV2::from_admission(admission);
            let resulting = transition_head(&current, &prepared.input, &authority)?;
            let record = LocalTransitionRecordV2 {
                schema_ref: "handbook.contract-local-lifecycle-record@2.0.0".into(),
                input: prepared.input,
                previous: current,
                resulting: resulting.clone(),
                admission: authority,
            };
            let prior = FingerprintV1::from_bytes(
                &o.read_regular(&o.local_head_path(&resulting.definition))?,
            );
            o.commit_local_mutation(
                TRANSITION,
                &resulting.definition,
                &resulting,
                canonical_bytes(&record).map_err(map_contract_input)?,
                request,
                key_fp,
                Some(prior),
                admission,
            )
        })
    }
    pub fn prepare_local_contract_use_v2(
        &self,
        project: &ProjectAdmissionBindingV3,
        binding: &ExactBindingV1,
        intent: LocalContractUseIntentV2,
    ) -> Result<LocalContractUseV2, ContractOwnerRepositoryErrorV1> {
        let (record, head) = self.with_existing_local_lock(|o| {
            o.validate_inventory()?;
            o.local_bundle(binding)
        })?;
        require_active_local(&record, &head, intent)?;
        let (applicability, current_basis) =
            resolve_local_applicability_v2(&self.repo_root, project, &record.definition)
                .map_err(map_contract_input)?;
        // Governance locks have been released before taking the owner lock again.
        let required_state_paths = self.with_existing_local_lock(|o| {
            o.validate_inventory()?;
            if o.local_bundle(binding)?.1 != head {
                return Err(substituted(
                    "local head changed during applicability preparation",
                ));
            }
            o.local_state_paths(binding)
        })?;
        Ok(LocalContractUseV2 {
            definition: record.definition,
            applicability,
            current_basis,
            lifecycle_head_fingerprint: head.lifecycle_fingerprint,
            intent,
            required_state_paths,
        })
    }
    /// Rechecks the owner half under an already live Engine admission, without
    /// reentering the governance owner's initialization lock.
    pub fn validate_local_contract_use_current_v2(
        &self,
        prepared: &LocalContractUseV2,
    ) -> Result<(), ContractOwnerRepositoryErrorV1> {
        self.with_existing_local_lock(|o| {
            o.validate_inventory()?;
            let (r, h) = o.local_bundle(&prepared.definition.binding())?;
            require_active_local(&r, &h, prepared.intent)?;
            if r.definition != prepared.definition
                || h.lifecycle_fingerprint != prepared.lifecycle_head_fingerprint
                || o.local_state_paths(&r.definition.binding())? != prepared.required_state_paths
            {
                return Err(substituted(
                    "local complete definition/head/retained closure changed before use",
                ));
            }
            Ok(())
        })
    }
    /// Packet-2 source seam only: this forms no Coordination, Purpose or Engine
    /// Evaluation Authorization closure and does not claim Packet-3 completion.
    pub fn prepare_local_source_seam_run_v2(
        &self,
        input: PrepareEvaluationRunInputV1,
    ) -> Result<ContractEvaluationRunV1, ContractOwnerRepositoryErrorV1> {
        self.with_existing_local_lock(|o| {
            o.validate_inventory()?;
            let (record, head) = o.local_bundle(input.contract())?;
            require_active_local(&record, &head, LocalContractUseIntentV2::Diagnostic)?;
            if input.expected_lifecycle_fingerprint() != &head.lifecycle_fingerprint {
                return Err(substituted("local run expected lifecycle is stale"));
            }
            validate_run_input(&record.definition, &input)?;
            ContractEvaluationRunV1::from_local_source_seam_v2(
                input,
                record.definition.semantic_definition().claims().to_vec(),
                head.basis()?,
                OwnerCurrentnessObservationV1::new(
                    SemanticTimeV1::owner_now().map_err(map_contract_input)?,
                    head.lifecycle_fingerprint,
                ),
            )
            .map_err(map_contract_input)
        })
    }
    pub(super) fn readmit_local_source_seam_run_locked_v2(
        &self,
        run: &ContractEvaluationRunV1,
        registry: &EvaluationPolicyRegistryV1,
    ) -> Result<LifecycleHeadV1, ContractOwnerRepositoryErrorV1> {
        let (record, head) = self.local_bundle(&run.contract())?;
        require_active_local(&record, &head, LocalContractUseIntentV2::Diagnostic)?;
        if !run.is_local_source_seam_v2()
            || run.evaluation_lifecycle_basis() != &head.basis()?
            || run.owner_currentness().lifecycle_head_fingerprint() != &head.lifecycle_fingerprint
            || run.claims() != record.definition.semantic_definition().claims()
            || run.freshness_policies()
                != record
                    .definition
                    .semantic_definition()
                    .evaluation_closure()
                    .freshness_policies
            || run.stability_policies()
                != record
                    .definition
                    .semantic_definition()
                    .evaluation_closure()
                    .stability_policies
            || run.gate_policy() != record.definition.dependencies().gate_policy.binding()
            || run.local_closeout_policy()
                != record
                    .definition
                    .dependencies()
                    .gate_policy
                    .local_closeout_policy()
            || run.matcher() != &crate::MatcherBindingV1::exact_fields_v1()
        {
            return Err(substituted(
                "local source-seam run closure or current head is stale",
            ));
        }
        for binding in run
            .freshness_policies()
            .iter()
            .chain(run.stability_policies())
            .chain([run.gate_policy(), run.local_closeout_policy()])
        {
            if registry
                .resolve_exact(binding)
                .map_err(map_contract_input)?
                .canonical_json()
                != record
                    .definition
                    .dependencies()
                    .policy_registry()
                    .map_err(map_contract_input)?
                    .resolve_exact(binding)
                    .map_err(map_contract_input)?
                    .canonical_json()
            {
                return Err(substituted("local run policy content is substituted"));
            }
        }
        Ok(head.v1_projection())
    }
    fn local_bundle(
        &self,
        binding: &ExactBindingV1,
    ) -> Result<(LocalDefinitionRecordV2, LocalLifecycleHeadV2), ContractOwnerRepositoryErrorV1>
    {
        let record: LocalDefinitionRecordV2 = parse_canonical(
            &self.read_regular(
                &self
                    .repo_root
                    .join(local_ref("definitions", binding.fingerprint())),
            )?,
        )?;
        record.validate()?;
        if record.definition.binding() != *binding {
            return Err(substituted("local definition identity is substituted"));
        }
        let lock_binding = self
            .repository_root
            .existing_lock_physical_fingerprint_v1(LOCK_RELATIVE_PATH)
            .map_err(map_durability)?;
        if lock_binding != record.owner_lock_fingerprint {
            return Err(recovery(
                "original physical Contracts owner lock is lost or substituted",
            ));
        }
        let mut head = record.genesis.clone();
        let mut remaining = vec![];
        for path in self.read_dir_bounded(
            &self
                .repo_root
                .join(format!("{LOCAL_CONTRACT_OWNER_ROOT_V2}/transitions")),
            MAX_RECORDS,
        )? {
            let t: LocalTransitionRecordV2 = parse_canonical(&self.read_regular(&path)?)?;
            if t.previous.definition == *binding {
                remaining.push(t);
            }
        }
        while !remaining.is_empty() {
            let at = remaining
                .iter()
                .position(|t| t.previous == head)
                .ok_or_else(|| recovery("local lifecycle chain has gap or fork"))?;
            let t = remaining.remove(at);
            validate_local_edge(&record, &head, &t.input)?;
            let action = if t.input.to_state == LifecycleStateV1::Locked {
                LocalCheckActionV1::Review
            } else {
                LocalCheckActionV1::Lifecycle
            };
            validate_historical_admission(&t.admission, &record.definition, action)?;
            if t.schema_ref != "handbook.contract-local-lifecycle-record@2.0.0"
                || t.resulting != transition_head(&head, &t.input, &t.admission)?
            {
                return Err(recovery("local lifecycle history is substituted"));
            }
            head = t.resulting;
        }
        let persisted: LocalLifecycleHeadV2 =
            parse_canonical(&self.read_regular(&self.local_head_path(binding))?)?;
        if persisted != head {
            return Err(recovery(
                "local current head does not equal complete immutable history",
            ));
        }
        Ok((record, head))
    }
    fn local_head_path(&self, binding: &ExactBindingV1) -> PathBuf {
        self.repo_root
            .join(local_ref("heads", binding.fingerprint()))
    }
    fn local_state_paths(
        &self,
        binding: &ExactBindingV1,
    ) -> Result<Vec<String>, ContractOwnerRepositoryErrorV1> {
        let mut paths = BTreeSet::from([
            LOCK_RELATIVE_PATH.to_owned(),
            local_ref("definitions", binding.fingerprint()),
            local_ref("heads", binding.fingerprint()),
        ]);
        for transaction in self.read_dir_bounded(&self.transactions_root(), MAX_RECORDS)? {
            let intent: TransactionIntent =
                parse_canonical(&self.read_regular(&transaction.join("intent.json"))?)?;
            if !matches!(intent.operation.as_str(), APPEND | TRANSITION) {
                continue;
            }
            let ledger: LocalLedgerV2 =
                parse_canonical(&self.read_regular(&transaction.join("02-ledger.json"))?)?;
            if ledger.receipt.definition != *binding {
                continue;
            }
            for entry in self.read_dir_bounded(&transaction, 8)? {
                paths.insert(self.relative_path(&entry)?);
            }
            for out in intent.outputs {
                paths.insert(out.relative_ref);
            }
        }
        if paths.len() > 256 {
            return Err(bound(
                "local retained source population exceeds native checkpoint bound",
            ));
        }
        Ok(paths.into_iter().collect())
    }
    fn local_replay(
        &self,
        key: &FingerprintV1,
        request: &FingerprintV1,
    ) -> Result<Option<LocalOwnerMutationReceiptV2>, ContractOwnerRepositoryErrorV1> {
        let path = self.repo_root.join(local_ref("idempotency", key));
        if !self.exists(&path)? {
            return Ok(None);
        }
        let mut ledger: LocalLedgerV2 = parse_canonical(&self.read_regular(&path)?)?;
        if ledger.key_fingerprint != *key
            || ledger.receipt.request_fingerprint != *request
            || !matches!(ledger.operation.as_str(), APPEND | TRANSITION)
        {
            return Err(substituted("local idempotency identity conflicts"));
        }
        ledger.receipt.disposition = OwnerMutationDispositionV1::Replayed;
        Ok(Some(ledger.receipt))
    }
    fn commit_local_mutation(
        &self,
        operation: &str,
        binding: &ExactBindingV1,
        head: &LocalLifecycleHeadV2,
        primary: Vec<u8>,
        request: FingerprintV1,
        key: FingerprintV1,
        prior: Option<FingerprintV1>,
        admission: &LocalCheckAdmissionV1<'_>,
    ) -> Result<LocalOwnerMutationReceiptV2, ContractOwnerRepositoryErrorV1> {
        let primary_fp = FingerprintV1::from_bytes(&primary);
        let primary_ref = local_ref(
            if operation == APPEND {
                "definitions"
            } else {
                "transitions"
            },
            if operation == APPEND {
                binding.fingerprint()
            } else {
                &primary_fp
            },
        );
        let receipt = LocalOwnerMutationReceiptV2 {
            disposition: OwnerMutationDispositionV1::Committed,
            definition: binding.clone(),
            request_fingerprint: request.clone(),
            record_ref: primary_ref.clone(),
            record_fingerprint: primary_fp,
            lifecycle_head: head.clone(),
        };
        let ledger = LocalLedgerV2 {
            operation: operation.into(),
            key_fingerprint: key.clone(),
            receipt: receipt.clone(),
        };
        let bytes = [
            primary,
            canonical_bytes(head).map_err(map_contract_input)?,
            canonical_bytes(&ledger).map_err(map_contract_input)?,
        ];
        let outputs = vec![
            StagedOutput::new(
                if operation == APPEND {
                    "00-definition.json"
                } else {
                    "00-transition.json"
                },
                &primary_ref,
                &bytes[0],
            )?,
            StagedOutput::new(
                "01-head.json",
                &local_ref("heads", binding.fingerprint()),
                &bytes[1],
            )?,
            StagedOutput::new("02-ledger.json", &local_ref("idempotency", &key), &bytes[2])?,
        ];
        let id = canonical_fingerprint(&TransactionIdPreimage {
            schema_ref: "handbook.contract-owner-transaction-identity@1.0.0",
            operation,
            key_fingerprint: &key,
            request_fingerprint: &request,
        })
        .map_err(map_contract_input)?;
        let pending = self
            .transactions_root()
            .join(format!("{}.pending", fingerprint_hex(&id)));
        let committed = self
            .transactions_root()
            .join(format!("{}.committed", fingerprint_hex(&id)));
        if self.exists(&pending)? || self.exists(&committed)? {
            return Err(recovery("local transaction identity occupied"));
        }
        self.create_dir_durable(&pending)?;
        self.maybe_fault(OwnerFaultBoundary::PendingDirectory)?;
        for (index, (out, bytes)) in outputs.iter().zip(&bytes).enumerate() {
            self.write_new_durable(&pending.join(&out.staged_name), bytes)?;
            self.maybe_fault(match index {
                0 => OwnerFaultBoundary::StagedPrimary,
                1 => OwnerFaultBoundary::StagedHead,
                _ => OwnerFaultBoundary::StagedLedger,
            })?;
        }
        let intent = TransactionIntent::new(
            fingerprint_hex(&id),
            operation,
            &key,
            &request,
            prior,
            outputs,
        )?;
        self.write_new_durable(
            &pending.join("intent.json"),
            &canonical_bytes(&intent).map_err(map_contract_input)?,
        )?;
        self.maybe_fault(OwnerFaultBoundary::Intent)?;
        admission
            .validate_current()
            .map_err(|_| invalid("local authority changed before consuming commit decision"))?;
        let marker = CommitMarker::new(&intent)?;
        self.write_new_durable(
            &pending.join("commit-marker.json"),
            &canonical_bytes(&marker).map_err(map_contract_input)?,
        )?;
        self.maybe_fault(OwnerFaultBoundary::Marker)?;
        self.finish_transaction(&pending, &intent, &marker)?;
        self.rename_dir_durable(&pending, &committed)?;
        self.maybe_fault(OwnerFaultBoundary::CommittedRename)?;
        admission
            .validate_current()
            .map_err(|_| durability("local authority currentness uncertain after commit"))?;
        self.validate_inventory()?;
        Ok(receipt)
    }
    pub(super) fn ensure_local_layout(&self) -> Result<(), ContractOwnerRepositoryErrorV1> {
        for class in ["", "/definitions", "/transitions", "/heads", "/idempotency"] {
            self.repository_root
                .create_directories(&format!("{LOCAL_CONTRACT_OWNER_ROOT_V2}{class}"))
                .map_err(map_durability)?;
        }
        Ok(())
    }
    pub(super) fn validate_local_inventory(&self) -> Result<(), ContractOwnerRepositoryErrorV1> {
        let mut classes = [
            BTreeSet::new(),
            BTreeSet::new(),
            BTreeSet::new(),
            BTreeSet::new(),
        ];
        for transaction in self.read_dir_bounded(&self.transactions_root(), MAX_RECORDS)? {
            let intent: TransactionIntent =
                parse_canonical(&self.read_regular(&transaction.join("intent.json"))?)?;
            if !matches!(intent.operation.as_str(), APPEND | TRANSITION) {
                continue;
            }
            for out in intent.outputs {
                let index = if out.relative_ref.contains("/definitions/") {
                    0
                } else if out.relative_ref.contains("/transitions/") {
                    1
                } else if out.relative_ref.contains("/heads/") {
                    2
                } else {
                    3
                };
                if !classes[index].insert(out.relative_ref) && index != 2 {
                    return Err(recovery("local immutable output has multiple transactions"));
                }
            }
        }
        for (class, expected) in ["definitions", "transitions", "heads", "idempotency"]
            .iter()
            .zip(&classes)
        {
            self.require_inventory(
                &self
                    .repo_root
                    .join(format!("{LOCAL_CONTRACT_OWNER_ROOT_V2}/{class}")),
                expected,
            )?;
        }
        for path in self.read_dir_bounded(
            &self
                .repo_root
                .join(format!("{LOCAL_CONTRACT_OWNER_ROOT_V2}/definitions")),
            MAX_RECORDS,
        )? {
            let r: LocalDefinitionRecordV2 = parse_canonical(&self.read_regular(&path)?)?;
            self.local_bundle(&r.definition.binding())?;
        }
        Ok(())
    }
    pub(super) fn validate_local_transaction(
        &self,
        path: &Path,
        intent: &TransactionIntent,
        primary: &[u8],
        head: &[u8],
        ledger: &[u8],
    ) -> Result<(), ContractOwnerRepositoryErrorV1> {
        let head: LocalLifecycleHeadV2 = parse_canonical(head)?;
        let ledger: LocalLedgerV2 = parse_canonical(ledger)?;
        if ledger.operation != intent.operation
            || ledger.key_fingerprint != intent.key_fingerprint
            || ledger.receipt.request_fingerprint != intent.request_fingerprint
            || ledger.receipt.lifecycle_head != head
            || ledger.receipt.definition != head.definition
            || ledger.receipt.record_fingerprint != FingerprintV1::from_bytes(primary)
            || ledger.receipt.record_ref != intent.outputs[0].relative_ref
            || intent.outputs[1].relative_ref != local_ref("heads", head.definition.fingerprint())
            || intent.outputs[2].relative_ref != local_ref("idempotency", &intent.key_fingerprint)
        {
            return Err(recovery(
                "local committed transaction ledger/head do not close",
            ));
        }
        if intent.operation == APPEND {
            let r: LocalDefinitionRecordV2 = parse_canonical(primary)?;
            r.validate()?;
            if r.genesis != head
                || intent.outputs[0].relative_ref
                    != local_ref("definitions", r.definition.binding().fingerprint())
                || intent.request_fingerprint
                    != canonical_fingerprint(&(APPEND, &r.definition.binding()))
                        .map_err(map_contract_input)?
            {
                return Err(recovery("local committed genesis does not derive head"));
            }
        } else {
            let t: LocalTransitionRecordV2 = parse_canonical(primary)?;
            if t.schema_ref != "handbook.contract-local-lifecycle-record@2.0.0"
                || t.resulting != head
                || t.resulting != transition_head(&t.previous, &t.input, &t.admission)?
                || intent.outputs[0].relative_ref
                    != local_ref("transitions", &FingerprintV1::from_bytes(primary))
                || intent.expected_head_bytes_fingerprint
                    != Some(FingerprintV1::from_bytes(
                        &canonical_bytes(&t.previous).map_err(map_contract_input)?,
                    ))
                || intent.request_fingerprint
                    != canonical_fingerprint(&(TRANSITION, &t.resulting.definition, &t.input))
                        .map_err(map_contract_input)?
            {
                return Err(recovery("local transition does not derive staged head"));
            }
        }
        for out in &intent.outputs {
            let staged = self.read_regular(&path.join(&out.staged_name))?;
            if FingerprintV1::from_bytes(&staged) != out.bytes_fingerprint
                || (out.staged_name != "01-head.json"
                    && staged != self.read_regular(&self.repo_root.join(&out.relative_ref))?)
            {
                return Err(recovery(
                    "local committed output substituted or unreachable",
                ));
            }
        }
        Ok(())
    }
}
fn local_ref(class: &str, fingerprint: &FingerprintV1) -> String {
    format!(
        "{LOCAL_CONTRACT_OWNER_ROOT_V2}/{class}/{}.json",
        fingerprint_hex(fingerprint)
    )
}
fn genesis(
    d: &LocalContractDefinitionV2,
    a: &LocalAuthorityRecordV2,
) -> Result<LocalLifecycleHeadV2, ContractOwnerRepositoryErrorV1> {
    Ok(LocalLifecycleHeadV2 {
        definition: d.binding(),
        state: LifecycleStateV1::Draft,
        sequence: 0,
        transition_ref: None,
        lifecycle_fingerprint: canonical_fingerprint(&("handbook.local-draft-genesis@2.0.0", d, a))
            .map_err(map_contract_input)?,
        author_participant_ref: a.request.participant_ref.clone(),
        approving_review: None,
    })
}
fn transition_head(
    head: &LocalLifecycleHeadV2,
    input: &LocalLifecycleTransitionInputV2,
    a: &LocalAuthorityRecordV2,
) -> Result<LocalLifecycleHeadV2, ContractOwnerRepositoryErrorV1> {
    let fingerprint =
        canonical_fingerprint(&("handbook.local-lifecycle-transition@2.0.0", head, input, a))
            .map_err(map_contract_input)?;
    Ok(LocalLifecycleHeadV2 {
        definition: head.definition.clone(),
        state: input.to_state,
        sequence: head.sequence + 1,
        transition_ref: Some(format!(
            "{}#local-transition/{}",
            head.definition.reference(),
            fingerprint_hex(&fingerprint)
        )),
        lifecycle_fingerprint: fingerprint,
        author_participant_ref: head.author_participant_ref.clone(),
        approving_review: input
            .review
            .clone()
            .or_else(|| head.approving_review.clone()),
    })
}
fn validate_local_edge(
    d: &LocalDefinitionRecordV2,
    head: &LocalLifecycleHeadV2,
    input: &LocalLifecycleTransitionInputV2,
) -> Result<(), ContractOwnerRepositoryErrorV1> {
    if input.expected_lifecycle_fingerprint != head.lifecycle_fingerprint
        || input.reason.trim().is_empty()
        || input.reason.len() > 4096
        || head.sequence >= MAX_TRANSITIONS_PER_DEFINITION
    {
        return Err(invalid(
            "local lifecycle expected head/reason/bound invalid",
        ));
    }
    if !matches!(
        (head.state, input.to_state),
        (LifecycleStateV1::Draft, LifecycleStateV1::ReviewReady)
            | (LifecycleStateV1::ReviewReady, LifecycleStateV1::Draft)
            | (LifecycleStateV1::ReviewReady, LifecycleStateV1::Locked)
            | (LifecycleStateV1::Locked, LifecycleStateV1::Active)
            | (LifecycleStateV1::Active, LifecycleStateV1::Deprecated)
            | (LifecycleStateV1::Deprecated, LifecycleStateV1::Closed)
    ) {
        return Err(invalid("local lifecycle edge unsupported"));
    }
    if input.to_state == LifecycleStateV1::Locked {
        let r = input
            .review
            .as_ref()
            .ok_or_else(|| invalid("actual approving review required"))?;
        r.validate(&d.definition, &head.author_participant_ref)
            .map_err(map_contract_input)?;
        if r.decision != LocalReviewDecisionV2::Approve
            || r.reviewer_actor_ref == d.author_admission.request.actor_ref
        {
            return Err(invalid(
                "review rejected or authored by same actual participant",
            ));
        }
    } else if input.review.is_some() {
        return Err(invalid("review belongs only to selected lock operation"));
    }
    if input.to_state == LifecycleStateV1::Active && head.approving_review.is_none() {
        return Err(invalid("cannot activate without actual review"));
    }
    Ok(())
}
fn require_active_local(
    d: &LocalDefinitionRecordV2,
    head: &LocalLifecycleHeadV2,
    intent: LocalContractUseIntentV2,
) -> Result<(), ContractOwnerRepositoryErrorV1> {
    if head.state != LifecycleStateV1::Active
        || head.approving_review.is_none()
        || !d.definition.permitted_use().permits(intent)
    {
        return Err(invalid(
            "local definition is not reviewed active for requested immutable permitted use",
        ));
    }
    Ok(())
}
fn validate_live_admission(
    o: &ContractOwnerRepositoryV1,
    a: &LocalCheckAdmissionV1<'_>,
    d: &LocalContractDefinitionV2,
    action: LocalCheckActionV1,
    basis: &LocalCheckCurrentBasisV1,
    content: &FingerprintV1,
) -> Result<(), ContractOwnerRepositoryErrorV1> {
    a.require_repository_root(&o.repository_root)
        .map_err(|_| unsafe_fs("local admission names other physical root"))?;
    for required in [
        format!("{CONTRACT_OWNER_REPOSITORY_RELATIVE_ROOT_V1}/transactions"),
        LOCAL_CONTRACT_OWNER_ROOT_V2.into(),
    ] {
        if !a.request().owner_write_paths.contains(&required) {
            return Err(invalid(
                "local admission omits a mandatory owner mutation root",
            ));
        }
    }
    a.validate_current()
        .map_err(|_| invalid("local admission no longer current"))?;
    let expected_ref = match action {
        LocalCheckActionV1::DefinitionAuthor => "handbook.contract-local-definition-append@2.0.0",
        LocalCheckActionV1::Review => "handbook.contract-local-review@2.0.0",
        _ => "handbook.contract-local-lifecycle-transition@2.0.0",
    };
    if a.request().action != action
        || a.request().action_ref != expected_ref
        || a.request().subject_ref != d.binding().reference().as_str()
        || a.request().content_fingerprint != content.as_str()
        || a.request().closure_fingerprint != d.closure_fingerprint().as_str()
        || a.current_basis() != basis
    {
        return Err(invalid(
            "local action/content/class/closure/applicability is not exact",
        ));
    }
    Ok(())
}
fn validate_historical_admission(
    a: &LocalAuthorityRecordV2,
    d: &LocalContractDefinitionV2,
    action: LocalCheckActionV1,
) -> Result<(), ContractOwnerRepositoryErrorV1> {
    a.locator
        .canonical_json_bytes()
        .map_err(|_| recovery("local historical terms are malformed"))?;
    if a.request.action != action
        || a.request.subject_ref != d.binding().reference().as_str()
        || a.request.closure_fingerprint != d.closure_fingerprint().as_str()
        || FingerprintV1::parse(&a.authority_fingerprint).is_err()
    {
        return Err(recovery(
            "local historical action/closure provenance is substituted",
        ));
    }
    Ok(())
}
fn validate_run_input(
    d: &LocalContractDefinitionV2,
    input: &PrepareEvaluationRunInputV1,
) -> Result<(), ContractOwnerRepositoryErrorV1> {
    let closure = d.semantic_definition().evaluation_closure();
    if input.contract() != &d.binding()
        || input.freshness_policies() != closure.freshness_policies
        || input.stability_policies() != closure.stability_policies
        || input.matcher() != &crate::MatcherBindingV1::exact_fields_v1()
        || input.gate_policy() != d.dependencies().gate_policy.binding()
        || input.local_closeout_policy() != d.dependencies().gate_policy.local_closeout_policy()
    {
        return Err(invalid(
            "local source-seam run exact dependency closure substituted",
        ));
    }
    Ok(())
}
pub(super) fn validate_local_intent_v2(
    intent: &TransactionIntent,
) -> Result<(), ContractOwnerRepositoryErrorV1> {
    if intent.schema_ref != "handbook.contract-owner-transaction-intent@1.0.0"
        || intent.outputs.len() != 3
        || !matches!(intent.operation.as_str(), APPEND | TRANSITION)
        || (intent.operation == APPEND) != intent.expected_head_bytes_fingerprint.is_none()
    {
        return Err(recovery("local transaction closed shape is invalid"));
    }
    let classes = [
        (
            if intent.operation == APPEND {
                "00-definition.json"
            } else {
                "00-transition.json"
            },
            if intent.operation == APPEND {
                "definitions"
            } else {
                "transitions"
            },
        ),
        ("01-head.json", "heads"),
        ("02-ledger.json", "idempotency"),
    ];
    for (output, (name, class)) in intent.outputs.iter().zip(classes) {
        validate_relative(&output.relative_ref)?;
        validate_staged(&output.staged_name)?;
        let prefix = format!("{LOCAL_CONTRACT_OWNER_ROOT_V2}/{class}/");
        let fp = output
            .relative_ref
            .strip_prefix(&prefix)
            .and_then(|s| s.strip_suffix(".json"))
            .ok_or_else(|| recovery("local transaction output path escapes exact owner class"))?;
        if output.staged_name != name || FingerprintV1::parse(&format!("sha256:{fp}")).is_err() {
            return Err(recovery("local transaction output identity invalid"));
        }
    }
    let id = canonical_fingerprint(&TransactionIdPreimage {
        schema_ref: "handbook.contract-owner-transaction-identity@1.0.0",
        operation: &intent.operation,
        key_fingerprint: &intent.key_fingerprint,
        request_fingerprint: &intent.request_fingerprint,
    })
    .map_err(map_contract_input)?;
    if intent.transaction_id != fingerprint_hex(&id)
        || *intent
            != TransactionIntent::new(
                &intent.transaction_id,
                &intent.operation,
                &intent.key_fingerprint,
                &intent.request_fingerprint,
                intent.expected_head_bytes_fingerprint.clone(),
                intent.outputs.clone(),
            )?
    {
        return Err(recovery("local transaction identity substituted"));
    }
    Ok(())
}
