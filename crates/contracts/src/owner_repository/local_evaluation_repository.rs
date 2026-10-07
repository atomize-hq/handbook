//! Finite full-evaluation state uses the original Contracts lock and retained
//! root. Every transition is an immutable record, exact head and replay ledger.
use super::*;
use crate::local_evaluation::*;
use crate::*;
use handbook_engine::local_authority::local_evaluation::*;
use serde_json::json;
use std::collections::BTreeMap;

pub(super) const EVALUATION_OPERATION: &str = "contract.local-evaluation.transition";
const DOMAIN: &str = "handbook.contract-local-evaluation-state@3.0.0";
const ORIGINAL_EVALUATION_LOCK_PATHS: [&str; 7] = [
    ".handbook-initialization.lock",
    LOCK_RELATIVE_PATH,
    ".handbook/state/locks/dock-operational-store.lock",
    ".handbook/state/locks/local-operating.lock",
    ".handbook/state/locks/promotion.lock",
    ".handbook/state/locks/registry.lock",
    ".handbook/state/locks/lifecycle.lock",
];

fn retained_evaluation_dependency_fingerprint(
    root: &RetainedRepositoryRootV1,
    path: &str,
    lock_generations: &[handbook_engine::local_authority::LocalBindingV1],
) -> Result<Option<FingerprintV1>, ContractOwnerRepositoryErrorV1> {
    if ORIGINAL_EVALUATION_LOCK_PATHS.contains(&path) {
        let original = lock_generations
            .iter()
            .find(|lock| lock.reference == path)
            .ok_or_else(|| invalid("original native lock generation is not retained"))?;
        // The native observer independently requires a zero-length, single-link
        // regular file and the exact stable physical generation. Reading bytes
        // through another handle would violate a held Windows exclusive lock.
        let current = root
            .existing_lock_physical_fingerprint_v1(path)
            .map_err(map_durability)?;
        if current != original.fingerprint {
            return Err(substituted(
                "original native lock physical generation changed",
            ));
        }
        return Ok(Some(FingerprintV1::from_bytes(&[])));
    }
    match root
        .read_regular_bounded(path, 16 * 1024 * 1024)
        .map_err(map_durability)?
    {
        RepositoryReadV1::Found(bytes) => Ok(Some(FingerprintV1::from_bytes(&bytes))),
        RepositoryReadV1::Absent => Ok(None),
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
struct EvaluationLedger {
    operation_key: String,
    key_fingerprint: FingerprintV1,
    request_fingerprint: FingerprintV1,
    locator: LocalEvaluationLocatorV3,
    record: ContentAddressedBindingV1,
}

impl ContractOwnerRepositoryV1 {
    /// Historical result admission is based on its own committed occurrence
    /// chain. Unavailable definition dependencies must not become current reads.
    pub fn open_existing_for_local_evaluation_history_v3(
        root: impl AsRef<Path>,
    ) -> Result<Self, ContractOwnerRepositoryErrorV1> {
        let root = root.as_ref();
        let owner = Self {
            repo_root: root.to_path_buf(),
            repository_root: RetainedRepositoryRootV1::open_exact(root).map_err(map_durability)?,
            #[cfg(feature = "test-support")]
            fault: None,
        };
        owner.with_existing_full_definition_lock(|_| Ok(()))?;
        Ok(owner)
    }
    pub fn prepare_local_evaluation_v3(
        &self,
        project: &crate::project_admission_governance::ProjectAdmissionBindingV3,
        input: LocalEvaluationPrepareInputV3,
    ) -> Result<PreparedLocalEvaluationV3, ContractOwnerRepositoryErrorV1> {
        validate_key(&input.request_key)?;
        if !declared_read_subtree(
            &input.formation.reach.input_paths,
            ".handbook/state/contracts",
        ) || !declared_read(&input.formation.reach.input_paths, LOCK_RELATIVE_PATH)
        {
            return Err(invalid(
                "formation omits complete definition or original lock proof reach",
            ));
        }
        let intent = match input.formation.use_intent {
            LocalEvaluationUseIntentV1::DecisionSeeking => {
                LocalContractUseIntentV2::DecisionSeeking
            }
            LocalEvaluationUseIntentV1::Diagnostic => LocalContractUseIntentV2::Diagnostic,
        };
        let local_use = self.prepare_local_contract_use_v3(project, &input.contract, intent)?;
        let definition = local_use.definition();
        if input.expected_lifecycle_fingerprint != *local_use.lifecycle_head_fingerprint()
            || input.formation.subject_ref != input.subject.subject().reference()
            || input.formation.subject_fingerprint != input.subject.subject().fingerprint().as_str()
            || input.case.case_binding() != input.subject.subject()
            || input
                .requested_resolution
                .resolution_stack()
                .reference()
                .as_str()
                != input.formation.requested_reach.stack.reference
            || input
                .requested_resolution
                .resolution_stack()
                .fingerprint()
                .as_str()
                != input.formation.requested_reach.stack.fingerprint
            || crate::evidence_admission::engine_resolution_values(
                input.requested_resolution.dimension_values(),
            )
            .map_err(|_| invalid("requested Resolution dimensions malformed"))?
                != input.formation.requested_reach.dimensions
        {
            return Err(invalid(
                "exact captured subject/case/lifecycle/Resolution must close formation",
            ));
        }
        let mut requirements = LocalEvaluationRequirementsV3 {
            schema_ref: "handbook.contract-local-requirements@3.0.0".into(),
            source: "direct_contract".into(),
            contract: definition.binding(),
            permitted_use: definition.permitted_use(),
            intent,
            claims: definition.semantic_definition().claims().to_vec(),
            subject: input.subject.clone(),
            case: input.case.clone(),
            observation_roles: vec!["schema_conformance".into()],
            metrics: vec![],
            dependencies: definition.full_dependencies().clone(),
            applicability: local_use.applicability().clone(),
            selected: vec![definition.binding()],
            excluded: vec![],
            missing: vec![],
            conflicting: vec![],
            fingerprint: FingerprintV1::from_bytes(b""),
        };
        requirements.fingerprint = requirements
            .expected_fingerprint()
            .map_err(map_contract_input)?;
        let deps = definition.dependencies();
        let plan = LocalEvaluationPlanV1 {
            schema_ref: LOCAL_EVALUATION_PLAN_SCHEMA_V1.into(),
            requirements_fingerprint: requirements.fingerprint.as_str().into(),
            selections: vec![LocalEvaluationSelectionV1 {
                selection_id: "native-json-schema".into(),
                role_ref: "schema_conformance".into(),
                producer_ref: deps.validator.manifest.reference().as_str().into(),
                configuration_fingerprint: canonical_fingerprint(&(
                    &deps.validator,
                    &deps.schema_closure,
                ))
                .map_err(map_contract_input)?
                .as_str()
                .into(),
                capability_fingerprint: native_local_evaluation_capability_binding_v3()
                    .map_err(map_contract_input)?
                    .fingerprint()
                    .as_str()
                    .into(),
                operating_ceiling_fingerprint: canonical_fingerprint(
                    &deps.validator.canonical_manifest["resolution_ceiling"],
                )
                .map_err(map_contract_input)?
                .as_str()
                .into(),
                host_prerequisites_fingerprint: canonical_fingerprint(&json!([]))
                    .map_err(map_contract_input)?
                    .as_str()
                    .into(),
                requested_reach: input.formation.requested_reach.clone(),
            }],
            composition: "single".into(),
            max_attempts: 1,
        };
        plan.validate()
            .map_err(|_| invalid("owner Plan cannot close supported selection"))?;
        let request_fingerprint = canonical_fingerprint(&(
            "handbook.local-evaluation-formation-request@3.0.0",
            &input,
            &requirements,
            &plan,
        ))
        .map_err(map_contract_input)?;
        Ok(PreparedLocalEvaluationV3 {
            input,
            local_use,
            requirements,
            plan,
            request_fingerprint,
        })
    }

    pub fn form_local_evaluation_v3(
        &self,
        prepared: PreparedLocalEvaluationV3,
        admission: &LocalEvaluationTermsAdmissionV1<'_>,
    ) -> Result<LocalEvaluationSnapshotV3, ContractOwnerRepositoryErrorV1> {
        admission
            .require_repository_root(&self.repository_root)
            .map_err(|_| invalid("formation authority belongs to another root"))?;
        if !declared_read_subtree(
            &admission.request().reach.input_paths,
            ".handbook/state/contracts",
        ) {
            return Err(invalid(
                "formation omits complete owner inventory read subtree",
            ));
        }
        for required in [
            LOCAL_EVALUATION_OWNER_ROOT_V3,
            ".handbook/state/contracts/v1/transactions",
        ] {
            admission
                .require_owner_write_subtree(required)
                .map_err(|_| invalid("formation omits required complete owner write subtree"))?;
        }
        if admission.request() != &prepared.input.formation
            || admission.current_basis() != prepared.current_basis()
        {
            return Err(invalid(
                "formation permission or owner D/G basis is not exact",
            ));
        }
        for path in prepared.local_use.required_state_paths() {
            admission
                .require_input_path(path)
                .map_err(|_| invalid("formation proof omits exact definition state path"))?;
        }
        self.with_existing_full_definition_lock(|owner| {
            owner.recover_locked()?;
            owner.validate_inventory()?;
            let formation_key = prepared.input.request_key.clone();
            let formation_request_fingerprint = prepared.request_fingerprint.clone();
            let key = eval_key(&formation_key)?;
            if let Some(ledger) = owner.eval_ledger(&key)? {
                if ledger.request_fingerprint != prepared.request_fingerprint {
                    return Err(repository_error(
                        ContractOwnerRepositoryErrorKindV1::IdempotencyConflict,
                        "formation key conflicts with original exact request",
                    ));
                }
                return owner.eval_current(&ledger.locator);
            }
            let (definition, head) = owner.full_local_bundle(&prepared.input.contract)?;
            if definition.definition() != prepared.local_use.definition()
                || head.lifecycle_fingerprint() != prepared.local_use.lifecycle_head_fingerprint()
                || head.state() != LifecycleStateV1::Active
                || head.approving_review().is_none()
            {
                return Err(substituted(
                    "full definition changed before Purpose allocation",
                ));
            }
            let ordinal = owner
                .eval_max_ordinal()?
                .checked_add(1)
                .filter(|n| *n <= 9_007_199_254_740_991)
                .ok_or_else(|| bound("owner occurrence ordinal exhausted"))?;
            let owner_identity = FingerprintV1::parse(
                &owner
                    .repository_root
                    .governance_physical_root_fingerprint_v1()
                    .map_err(map_durability)?,
            )
            .map_err(map_contract_input)?;
            let lock_generation = owner
                .repository_root
                .existing_lock_physical_fingerprint_v1(LOCK_RELATIVE_PATH)
                .map_err(map_durability)?;
            let occurrence_fp = canonical_fingerprint(&(
                "handbook.contract-local-occurrence-allocation@3.0.0",
                &owner_identity,
                ordinal,
            ))
            .map_err(map_contract_input)?;
            let occurrence = ContentAddressedBindingV1::new(
                &format!(
                    "handbook.contract-local-occurrence@3.0.0#{}:{ordinal}",
                    fingerprint_hex(&owner_identity)
                ),
                occurrence_fp.clone(),
            )
            .map_err(map_contract_input)?;
            let now = SemanticTimeV1::owner_now().map_err(map_contract_input)?;
            let deps = prepared.local_use.definition().full_dependencies();
            let mut purpose = LocalEvaluationPurposeV1 {
                schema_ref: LOCAL_EVALUATION_PURPOSE_SCHEMA_V1.into(),
                owner_identity: owner_identity.as_str().into(),
                project_identity: admission.project_identity().to_owned(),
                occurrence_identity: occurrence_fp.as_str().into(),
                requirements_fingerprint: prepared.requirements.fingerprint.as_str().into(),
                requirements_source: "direct_contract".into(),
                contract: handbook_engine::local_authority::LocalBindingV1 {
                    reference: prepared.input.contract.reference().as_str().into(),
                    fingerprint: prepared.input.contract.fingerprint().as_str().into(),
                },
                subject_ref: prepared.input.formation.subject_ref.clone(),
                subject_fingerprint: prepared.input.formation.subject_fingerprint.clone(),
                scope_fingerprint: prepared.input.formation.scope_fingerprint.clone(),
                objective: prepared.input.formation.objective.clone(),
                permitted_use: match prepared.local_use.definition().permitted_use() {
                    LocalPermittedUseV2::DiagnosticOnly => "diagnostic_only",
                    LocalPermittedUseV2::DecisionSeekingEligible => "decision_seeking_eligible",
                }
                .into(),
                use_intent: prepared.input.formation.use_intent,
                plan_fingerprint: prepared
                    .plan
                    .fingerprint()
                    .map_err(|_| invalid("plan fingerprint unavailable"))?,
                formation_policy_fingerprint: deps.formation_policy.fingerprint().as_str().into(),
                evaluation_rule_fingerprint: deps.evaluation_rule.fingerprint().as_str().into(),
                time_rules_fingerprint: deps.time_rules.fingerprint().as_str().into(),
                retention_until_utc: prepared.input.formation.retention_until_utc.clone(),
                content_fingerprint: String::new(),
                formed_at_utc: now.as_str().into(),
            };
            purpose.content_fingerprint = purpose
                .fingerprint_semantic_content()
                .map_err(|_| invalid("Purpose content fingerprint unavailable"))?;
            purpose
                .validate()
                .map_err(|_| invalid("Purpose owner time or exact content is invalid"))?;
            let run_input = PrepareEvaluationRunInputV1::new(
                prepared.input.contract.clone(),
                prepared.input.expected_lifecycle_fingerprint.clone(),
                prepared.input.subject.clone(),
                vec![prepared.input.case.clone()],
                prepared.input.requested_resolution.clone(),
                MatcherBindingV1::exact_fields_v1(),
                prepared
                    .local_use
                    .definition()
                    .dependencies()
                    .freshness_policies
                    .iter()
                    .map(EvidenceFreshnessPolicyV1::exact_binding)
                    .collect(),
                vec![prepared
                    .local_use
                    .definition()
                    .dependencies()
                    .stability_policy
                    .clone()],
                deps.evaluation_gate.clone(),
                deps.evaluation_rule.clone(),
                EvaluationRunNonceV1::parse(occurrence_fp.as_str()).map_err(map_contract_input)?,
            )
            .map_err(map_contract_input)?;
            let lifecycle = identified(
                "handbook.local-reviewed-lifecycle@3.0.0",
                &(
                    prepared.input.contract.clone(),
                    prepared.input.expected_lifecycle_fingerprint.clone(),
                ),
            )
            .map_err(map_contract_input)?;
            let run = ContractEvaluationRunV1::from_local_evaluation_v3(
                run_input,
                prepared.requirements.claims.clone(),
                lifecycle,
                OwnerCurrentnessObservationV1::new(
                    now,
                    prepared.input.expected_lifecycle_fingerprint.clone(),
                ),
            )
            .map_err(map_contract_input)?;
            let locator = LocalEvaluationLocatorV3 {
                request_key: prepared.input.request_key.clone(),
                request_fingerprint: prepared.request_fingerprint.clone(),
                occurrence,
                owner_identity,
                lock_generation,
            };
            let mut state = LocalEvaluationSnapshotV3 {
                locator,
                ordinal,
                sequence: 0,
                stage: LocalEvaluationStageV3::Purpose,
                requirements: prepared.requirements,
                purpose,
                plan: prepared.plan,
                run,
                current_basis: prepared.local_use.current_basis().clone(),
                original_input: prepared.input,
                definition: prepared.local_use.definition().clone(),
                formation_authority_fingerprint: admission.authority_fingerprint().into(),
                authorization: None,
                issuance_locator: None,
                retained_lock_generations: vec![],
                readiness_bases: vec![],
                committed_basis: None,
                collection_checks: vec![],
                attempt: None,
                origin_intent: None,
                origin_terminal: None,
                origin_paths: vec![],
                evidence: vec![],
                admission_terminals: vec![],
                frontier: None,
                evidence_set: None,
                result: None,
                reason: None,
                identity: identified(DOMAIN, &()).map_err(map_contract_input)?,
            };
            state.identity = state_identity(&state)?;
            admission
                .validate_current()
                .map_err(|_| invalid("formation authority changed before allocation intent"))?;
            owner.commit_eval_state(
                None,
                state,
                &formation_request_fingerprint,
                &key,
                &formation_key,
                || {
                    admission
                        .validate_current()
                        .map_err(|_| invalid("formation authority changed before commit"))
                },
            )
        })
    }

    pub fn assess_local_evaluation_v3(
        &self,
        locator: &LocalEvaluationLocatorV3,
        admission: &LocalEvaluationAdmissionV1<'_>,
        capability: &ResolvedLocalEvaluationCapabilityV3<'_>,
    ) -> Result<LocalEvaluationSnapshotV3, ContractOwnerRepositoryErrorV1> {
        capability
            .require_repository_root(&self.repository_root)
            .map_err(map_contract_input)?;
        self.eval_mutation(
            locator,
            admission,
            LocalEvaluationCheckpointV1::Preparation,
            None,
            "readiness",
            None,
            |state| {
                if state.committed_basis.is_some()
                    || !matches!(
                        state.stage,
                        LocalEvaluationStageV3::Purpose
                            | LocalEvaluationStageV3::Held
                            | LocalEvaluationStageV3::Attemptable
                    )
                {
                    return Err(invalid(
                        "readiness cannot replace committed collection basis",
                    ));
                }
                let record = capability.record();
                if record.manifest != state.definition.dependencies().validator.manifest
                    || record.semantic_capability
                        != native_local_evaluation_capability_binding_v3()
                            .map_err(map_contract_input)?
                    || record.retention_until.as_str() < state.purpose.retention_until_utc.as_str()
                    || record.observed_at.as_str()
                        > admission.observation().observed_at_utc.as_str()
                    || record.controls["operating_ceiling"]
                        != state.definition.dependencies().validator.canonical_manifest
                            ["resolution_ceiling"]
                {
                    return Err(invalid(
                        "required native capability/ceiling/retention is missing or incompatible",
                    ));
                }
                let ceiling: ResolutionEnvelopeV1 =
                    serde_json::from_value(record.controls["operating_ceiling"].clone())
                        .map_err(|_| invalid("native operating ceiling is malformed"))?;
                let requested = resolution_ranks(
                    admission,
                    state.original_input.requested_resolution.dimension_values(),
                )?;
                let maximum = resolution_ranks(admission, ceiling.dimension_values())?;
                let minimum = resolution_ranks(
                    admission,
                    state.requirements.claims[0]
                        .minimum_resolution()
                        .dimension_values(),
                )?;
                let adequate =
                    requested.iter().zip(maximum).zip(minimum).all(
                        |((&request, maximum), minimum)| request <= maximum && request >= minimum,
                    );
                let readiness = match record.availability.as_str() {
                    "available" if adequate => LocalEvaluationReadinessV3::Ready,
                    "available" | "known_limited" => LocalEvaluationReadinessV3::KnownLimited,
                    "unknown" => LocalEvaluationReadinessV3::Unknown,
                    _ => return Err(invalid("native capability observation is malformed")),
                };
                let attemptable = readiness == LocalEvaluationReadinessV3::Ready
                    || state.purpose.use_intent == LocalEvaluationUseIntentV1::Diagnostic;
                let mut basis = LocalEvaluationReadinessBasisV3 {
                    capability: record.clone(),
                    verification: admission.observation().clone(),
                    readiness,
                    disposition: if attemptable { "attemptable" } else { "held" }.into(),
                    reason: if !adequate {
                        "requested_resolution_or_claim_minimum_exceeds_effective_reach"
                    } else {
                        record.controls["availability_reason"]
                            .as_str()
                            .unwrap_or("native_readiness")
                    }
                    .into(),
                    observed_at: SemanticTimeV1::owner_now().map_err(map_contract_input)?,
                    fingerprint: FingerprintV1::from_bytes(b""),
                };
                let mut value = serde_json::to_value(&basis)
                    .map_err(|_| invalid("readiness encoding failed"))?;
                value.as_object_mut().unwrap().remove("fingerprint");
                basis.fingerprint = canonical_fingerprint(&value).map_err(map_contract_input)?;
                if state.readiness_bases.last().is_some_and(|previous| {
                    previous.capability == basis.capability
                        && previous.verification == basis.verification
                }) {
                    return Ok(());
                }
                state.readiness_bases.push(basis);
                state.authorization = Some(admission.binding().clone());
                state.stage = if attemptable {
                    LocalEvaluationStageV3::Attemptable
                } else {
                    LocalEvaluationStageV3::Held
                };
                Ok(())
            },
        )
    }
    pub fn commit_local_evaluation_v3(
        &self,
        locator: &LocalEvaluationLocatorV3,
        admission: &LocalEvaluationAdmissionV1<'_>,
    ) -> Result<LocalEvaluationSnapshotV3, ContractOwnerRepositoryErrorV1> {
        self.eval_mutation(
            locator,
            admission,
            LocalEvaluationCheckpointV1::CollectionCommit,
            None,
            "commit",
            None,
            |state| {
                let expected = prospective_basis(state)?;
                if admission.checkpoint_target() != Some(expected.fingerprint().as_str()) {
                    return Err(invalid(
                        "collection verification must name exact prospective basis",
                    ));
                }
                if let Some(basis) = &state.committed_basis {
                    if basis == &expected {
                        return Ok(());
                    }
                    return Err(invalid("occurrence already committed to another basis"));
                }
                if state.stage != LocalEvaluationStageV3::Attemptable {
                    return Err(invalid("held or refused readiness cannot commit or launch"));
                }
                state.committed_basis = Some(expected);
                state
                    .collection_checks
                    .push(admission.observation().clone());
                state.stage = LocalEvaluationStageV3::Committed;
                Ok(())
            },
        )
    }
    pub fn prospective_local_evaluation_basis_v3(
        &self,
        locator: &LocalEvaluationLocatorV3,
    ) -> Result<ContentAddressedBindingV1, ContractOwnerRepositoryErrorV1> {
        self.with_existing_full_definition_lock(|owner| {
            owner.validate_inventory()?;
            prospective_basis(&owner.eval_current(locator)?)
        })
    }
    pub fn local_evaluation_final_set_source_v3(
        &self,
        locator: &LocalEvaluationLocatorV3,
    ) -> Result<LocalEvaluationFinalSetSourceV1, ContractOwnerRepositoryErrorV1> {
        self.with_existing_full_definition_lock(|owner| {
            owner.validate_inventory()?;
            let state = owner.eval_current(locator)?;
            if state.stage != LocalEvaluationStageV3::SetClosed {
                return Err(invalid(
                    "exact set must be fixed before final verification source",
                ));
            }
            let set = state
                .evidence_set
                .as_ref()
                .ok_or_else(|| invalid("fixed set identity is missing"))?;
            let relative_path = eval_ref("records", state.identity.fingerprint());
            let canonical_record_bytes =
                owner.read_regular(&owner.repo_root.join(&relative_path))?;
            Ok(LocalEvaluationFinalSetSourceV1 {
                relative_path,
                record_fingerprint: FingerprintV1::from_bytes(&canonical_record_bytes)
                    .as_str()
                    .into(),
                set_fingerprint: set.identity.fingerprint().as_str().into(),
                canonical_record_bytes,
            })
        })
    }
    pub fn bind_local_evaluation_attempt_v3(
        &self,
        locator: &LocalEvaluationLocatorV3,
        admission: &LocalEvaluationAdmissionV1<'_>,
        attempt: ContentAddressedBindingV1,
    ) -> Result<LocalEvaluationSnapshotV3, ContractOwnerRepositoryErrorV1> {
        self.eval_mutation(
            locator,
            admission,
            LocalEvaluationCheckpointV1::Launch,
            None,
            "attempt",
            None,
            |state| {
                require_basis_target(state, admission)?;
                if let Some(existing) = &state.attempt {
                    if existing == &attempt {
                        return Ok(());
                    }
                    return Err(invalid("one real native attempt is the maximum"));
                }
                if state.stage != LocalEvaluationStageV3::Committed {
                    return Err(invalid(
                        "attempt requires one committed basis and open collection",
                    ));
                }
                state.attempt = Some(attempt);
                state
                    .collection_checks
                    .push(admission.observation().clone());
                Ok(())
            },
        )
    }
    pub fn admit_local_evaluation_origin_v3(
        &self,
        locator: &LocalEvaluationLocatorV3,
        admission: &LocalEvaluationAdmissionV1<'_>,
        origin: &ResolvedLocalEvaluationOriginV3<'_>,
        operation_key: &str,
    ) -> Result<LocalEvaluationSnapshotV3, ContractOwnerRepositoryErrorV1> {
        validate_key(operation_key)?;
        origin
            .require_repository_root(&self.repository_root)
            .map_err(map_contract_input)?;
        for path in origin.native_paths() {
            admission
                .require_input_path(path)
                .map_err(|_| invalid("origin admission omitted an actual proof path"))?;
        }
        self.eval_mutation(
            locator,
            admission,
            LocalEvaluationCheckpointV1::EvidenceAppend,
            None,
            operation_key,
            Some(origin),
            |state| {
                require_basis_target(state, admission)?;
                if state.frontier.is_some() {
                    if state.origin_terminal.as_ref() == Some(origin.terminal())
                        && origin.candidates().iter().all(|candidate| {
                            state.admission_terminals.iter().any(|t| {
                                t.operation_id == operation_key
                                    && t.candidate.fingerprint()
                                        == candidate.candidate_fingerprint()
                                    && t.candidate.reference() == candidate.candidate_ref()
                            })
                        })
                    {
                        return Ok(());
                    }
                    return Err(invalid("post-frontier origin cannot reopen collection"));
                }
                let expected_attempt = state.attempt.as_ref().ok_or_else(|| {
                    invalid("origin has no precommitted original attempt locator")
                })?;
                let observed_attempt: ContentAddressedBindingV1 =
                    serde_json::from_value(origin.terminal()["attempt"].clone())
                        .map_err(|_| invalid("origin attempt identity malformed"))?;
                if &observed_attempt != expected_attempt
                    || origin.intent()["basis"]
                        != serde_json::to_value(state.committed_basis.as_ref().unwrap())
                            .map_err(|_| invalid("basis encoding failed"))?
                    || origin.intent()["occurrence"]
                        != serde_json::to_value(&state.locator.occurrence)
                            .map_err(|_| invalid("occurrence encoding failed"))?
                    || origin.intent()["retention_until"]
                        .as_str()
                        .is_none_or(|h| h < state.purpose.retention_until_utc.as_str())
                    || origin.intent()["capability_observation"]
                        != serde_json::to_value(
                            state
                                .readiness_bases
                                .last()
                                .ok_or_else(|| invalid("origin readiness basis missing"))?
                                .capability
                                .binding()
                                .map_err(map_contract_input)?,
                        )
                        .map_err(|_| invalid("capability binding encoding failed"))?
                {
                    return Err(invalid(
                        "origin is mixed or retention obligation inadequate",
                    ));
                }
                if let Some(existing) = &state.origin_terminal {
                    if existing != origin.terminal() {
                        return Err(substituted("native origin terminal is immutable"));
                    }
                    if origin.candidates().iter().all(|candidate| {
                        state.admission_terminals.iter().any(|terminal| {
                            terminal.operation_id == operation_key
                                && terminal.candidate.fingerprint()
                                    == candidate.candidate_fingerprint()
                                && terminal.candidate.reference() == candidate.candidate_ref()
                        })
                    }) {
                        return Ok(());
                    }
                }
                state.origin_intent = Some(origin.intent().clone());
                state.origin_terminal = Some(origin.terminal().clone());
                state.origin_paths = origin.native_paths().to_vec();
                for candidate in origin.candidates() {
                    let binding = ContentAddressedBindingV1::new(
                        candidate.candidate_ref(),
                        candidate.candidate_fingerprint().clone(),
                    )
                    .map_err(map_contract_input)?;
                    if state
                        .admission_terminals
                        .iter()
                        .any(|t| t.operation_id == operation_key && t.candidate == binding)
                    {
                        continue;
                    }
                    let now = SemanticTimeV1::owner_now().map_err(map_contract_input)?;
                    let candidate_body = candidate.admission_body();
                    let refusal = if candidate_body.evaluation_run().fingerprint()
                        != state.run.evaluation_run_fingerprint()
                        || candidate_body.contract() != &state.definition.binding()
                        || candidate_body.observed_claim_ids()
                            != [state.requirements.claims[0].claim_id().clone()]
                    {
                        Some("candidate_is_outside_exact_occurrence_requirements")
                    } else if candidate_body.observed_at() > &now {
                        Some("candidate_observation_is_future")
                    } else {
                        None
                    };
                    let existing = state
                        .evidence
                        .iter()
                        .find(|e| e.candidate.candidate_ref() == candidate.candidate_ref());
                    let (disposition, reason, evidence) = if let Some(reason) = refusal {
                        ("refused", reason, None)
                    } else if let Some(existing) = existing {
                        if existing.candidate != *candidate {
                            ("refused", "conflicting_candidate_identity_bytes", None)
                        } else {
                            (
                                "duplicate",
                                "exact_duplicate_delivery",
                                Some(existing.identity.clone()),
                            )
                        }
                    } else {
                        let actual = resolution_ranks(
                            admission,
                            candidate_body.actual_resolution().dimension_values(),
                        )?;
                        let minimum = resolution_ranks(
                            admission,
                            state.requirements.claims[0]
                                .minimum_resolution()
                                .dimension_values(),
                        )?;
                        let requested = resolution_ranks(
                            admission,
                            candidate_body.request_resolution().dimension_values(),
                        )?;
                        let coverage_sufficient = actual.iter().zip(minimum).zip(requested).all(
                            |((&actual, minimum), requested)| {
                                actual >= minimum && actual <= requested
                            },
                        );
                        let mut record = LocalCanonicalEvidenceV3 {
                            schema_ref: "handbook.contract-local-canonical-evidence@3.0.0".into(),
                            occurrence: state.locator.occurrence.clone(),
                            basis: state.committed_basis.clone().unwrap(),
                            selection: state.selection().clone(),
                            attempt: expected_attempt.clone(),
                            candidate: candidate.clone(),
                            admission_policy: local_evidence_admission_policy_binding_v3()
                                .map_err(map_contract_input)?,
                            verification: admission.observation().clone(),
                            admitted_at: now.clone(),
                            coverage_sufficient,
                            identity: identified("handbook.local-canonical-evidence@3.0.0", &())
                                .map_err(map_contract_input)?,
                        };
                        record.identity = identified(
                            "handbook.local-canonical-evidence@3.0.0",
                            &without_identity(&record).map_err(map_contract_input)?,
                        )
                        .map_err(map_contract_input)?;
                        let identity = record.identity.clone();
                        state.evidence.push(record);
                        (
                            "admitted",
                            "exact_native_candidate_admitted",
                            Some(identity),
                        )
                    };
                    state
                        .admission_terminals
                        .push(LocalEvaluationAdmissionTerminalV3 {
                            operation_id: operation_key.into(),
                            candidate: binding,
                            disposition: disposition.into(),
                            reason: reason.into(),
                            evidence,
                            semantic_time: now,
                        });
                }
                state
                    .collection_checks
                    .push(admission.observation().clone());
                state.stage = LocalEvaluationStageV3::OriginClosed;
                Ok(())
            },
        )
    }

    pub fn seal_local_evaluation_frontier_v3(
        &self,
        locator: &LocalEvaluationLocatorV3,
        admission: &LocalEvaluationAdmissionV1<'_>,
    ) -> Result<LocalEvaluationSnapshotV3, ContractOwnerRepositoryErrorV1> {
        self.eval_mutation(
            locator,
            admission,
            LocalEvaluationCheckpointV1::SourceAdmission,
            None,
            "frontier",
            None,
            |state| {
                require_basis_target(state, admission)?;
                if state.frontier.is_some() {
                    return Ok(());
                }
                if state.stage != LocalEvaluationStageV3::OriginClosed {
                    return Err(invalid(
                        "uncertain or incomplete native origin cannot seal frontier",
                    ));
                }
                let mut frontier = LocalEvaluationFrontierV3 {
                    occurrence: state.locator.occurrence.clone(),
                    basis: state.committed_basis.clone().unwrap(),
                    plan_fingerprint: FingerprintV1::parse(&state.purpose.plan_fingerprint)
                        .map_err(map_contract_input)?,
                    attempt: state.attempt.clone().unwrap(),
                    origin_terminal: state.origin_terminal.clone().unwrap(),
                    collection_checks: state.collection_checks.clone(),
                    sealed_at: SemanticTimeV1::owner_now().map_err(map_contract_input)?,
                    identity: identified("handbook.local-frontier@3.0.0", &())
                        .map_err(map_contract_input)?,
                };
                frontier.identity = identified(
                    "handbook.local-frontier@3.0.0",
                    &without_identity(&frontier).map_err(map_contract_input)?,
                )
                .map_err(map_contract_input)?;
                state.frontier = Some(frontier);
                state.stage = LocalEvaluationStageV3::FrontierSealed;
                Ok(())
            },
        )
    }
    pub fn close_local_evaluation_set_v3(
        &self,
        locator: &LocalEvaluationLocatorV3,
        admission: &LocalEvaluationAdmissionV1<'_>,
    ) -> Result<LocalEvaluationSnapshotV3, ContractOwnerRepositoryErrorV1> {
        self.eval_mutation(
            locator,
            admission,
            LocalEvaluationCheckpointV1::SourceAdmission,
            None,
            "set",
            None,
            |state| {
                require_basis_target(state, admission)?;
                if state.evidence_set.is_some() {
                    return Ok(());
                }
                let frontier = state
                    .frontier
                    .as_ref()
                    .ok_or_else(|| invalid("one sealed frontier is required before set closure"))?;
                let candidates = validate_closed_population(
                    &frontier.origin_terminal,
                    &state.admission_terminals,
                    &population_evidence(&state.evidence)?,
                )?;
                let mut members = state
                    .evidence
                    .iter()
                    .map(|e| e.identity.clone())
                    .collect::<Vec<_>>();
                members.sort_by(|a, b| a.reference().cmp(b.reference()));
                if members.windows(2).any(|w| w[0] == w[1])
                    || state.evidence.iter().any(|e| {
                        e.occurrence != state.locator.occurrence
                            || Some(&e.basis) != state.committed_basis.as_ref()
                            || !candidates.iter().any(|c| {
                                c.reference() == e.candidate.candidate_ref()
                                    && c.fingerprint() == e.candidate.candidate_fingerprint()
                            })
                    })
                {
                    return Err(invalid(
                        "mixed duplicated or omitted matching evidence refuses set closure",
                    ));
                }
                let mut set = LocalEvaluationEvidenceSetV3 {
                    occurrence: state.locator.occurrence.clone(),
                    basis: state.committed_basis.clone().unwrap(),
                    frontier: frontier.identity.clone(),
                    formation_policy: state
                        .definition
                        .full_dependencies()
                        .formation_policy
                        .clone(),
                    eligible_owner: state.locator.owner_identity.clone(),
                    eligible_namespace: LOCAL_EVALUATION_OWNER_ROOT_V3.into(),
                    members,
                    admission_terminals: state.admission_terminals.clone(),
                    nonmembers: state
                        .admission_terminals
                        .iter()
                        .filter(|t| t.disposition == "refused")
                        .cloned()
                        .collect(),
                    closed_at: SemanticTimeV1::owner_now().map_err(map_contract_input)?,
                    identity: identified("handbook.local-evidence-set@3.0.0", &())
                        .map_err(map_contract_input)?,
                };
                set.identity = identified(
                    "handbook.local-evidence-set@3.0.0",
                    &without_identity(&set).map_err(map_contract_input)?,
                )
                .map_err(map_contract_input)?;
                state.evidence_set = Some(set);
                state.stage = LocalEvaluationStageV3::SetClosed;
                Ok(())
            },
        )
    }
    pub fn close_local_evaluation_result_v3(
        &self,
        locator: &LocalEvaluationLocatorV3,
        admission: &LocalEvaluationAdmissionV1<'_>,
        origin: &ResolvedLocalEvaluationOriginV3<'_>,
    ) -> Result<LocalEvaluationSnapshotV3, ContractOwnerRepositoryErrorV1> {
        origin
            .require_repository_root(&self.repository_root)
            .map_err(map_contract_input)?;
        for path in origin.native_paths() {
            admission
                .require_input_path(path)
                .map_err(|_| invalid("final closure omitted an actual native proof path"))?;
        }
        self.eval_mutation(
            locator,
            admission,
            LocalEvaluationCheckpointV1::FinalContext,
            None,
            "result",
            Some(origin),
            |state| {
                if state.result.is_some() {
                    return Ok(());
                }
                if state.origin_intent.as_ref() != Some(origin.intent())
                    || state.origin_terminal.as_ref() != Some(origin.terminal())
                    || state.origin_paths != origin.native_paths()
                {
                    return Err(substituted(
                        "final native chain differs from the original admitted origin",
                    ));
                }
                let set = state
                    .evidence_set
                    .as_ref()
                    .ok_or_else(|| invalid("final verification follows exact set closure"))?;
                let source = admission.final_set_source().ok_or_else(|| {
                    invalid("final Engine verification lacks pinned owner set source")
                })?;
                if source.relative_path != eval_ref("records", state.identity.fingerprint())
                    || source.canonical_record_bytes
                        != canonical_bytes(state).map_err(map_contract_input)?
                    || source.set_fingerprint != set.identity.fingerprint().as_str()
                {
                    return Err(invalid(
                        "final Engine verification observed another retained set state",
                    ));
                }
                if admission.checkpoint_target() != Some(set.identity.fingerprint().as_str()) {
                    return Err(invalid(
                        "final Engine verification must name already fixed exact evidence set",
                    ));
                }
                let now = SemanticTimeV1::owner_now().map_err(map_contract_input)?;
                let context = identified(
                    "handbook.local-final-context@3.0.0",
                    &(
                        &state.locator.occurrence,
                        &state.purpose,
                        &state.requirements,
                        &state.plan,
                        &state.authorization,
                        &state.readiness_bases,
                        &state.committed_basis,
                        &state.frontier,
                        set,
                        admission.observation(),
                        &now,
                    ),
                )
                .map_err(map_contract_input)?;
                let (claim, gate) = evaluate_closed_local_context_v3(state, set, &now)
                    .map_err(map_contract_input)?;
                let retained_until = SemanticTimeV1::parse(&state.purpose.retention_until_utc)
                    .map_err(map_contract_input)?;
                let explanation =
                    result_explanation(state, &claim, gate, set, admission.observation());
                let mut result = LocalEvaluationResultV3 {
                    schema_ref: "handbook.contract-local-evaluation-result@3.0.0".into(),
                    occurrence: state.locator.occurrence.clone(),
                    purpose: state.purpose.clone(),
                    context,
                    evidence_set: set.clone(),
                    final_verification: admission.observation().clone(),
                    claim,
                    metrics: vec![],
                    gate_policy: state.definition.full_dependencies().evaluation_gate.clone(),
                    gate,
                    explanation,
                    dependencies: vec![],
                    retained_until,
                    evaluated_at: now,
                    identity: identified("handbook.local-evaluation-result@3.0.0", &())
                        .map_err(map_contract_input)?,
                };
                result.identity = identified(
                    "handbook.local-evaluation-result@3.0.0",
                    &without_identity(&result).map_err(map_contract_input)?,
                )
                .map_err(map_contract_input)?;
                state.result = Some(result);
                state.stage = LocalEvaluationStageV3::ResultClosed;
                Ok(())
            },
        )
    }

    pub fn recover_local_evaluation_v3(
        &self,
        locator: &LocalEvaluationLocatorV3,
    ) -> Result<LocalEvaluationSnapshotV3, ContractOwnerRepositoryErrorV1> {
        self.with_existing_full_definition_lock(|owner| {
            owner.require_eval_locator_root(locator)?;
            owner.recover_locked()?;
            owner.validate_inventory()?;
            owner.eval_current(locator)
        })
    }
    pub fn recover_local_evaluation_request_v3(
        &self,
        original: &LocalEvaluationRequestLocatorV3,
    ) -> Result<Option<LocalEvaluationSnapshotV3>, ContractOwnerRepositoryErrorV1> {
        validate_key(&original.request_key)?;
        self.with_existing_full_definition_lock(|owner| {
            owner.recover_locked()?;
            owner.validate_inventory()?;
            let Some(ledger) = owner.eval_ledger(&eval_key(&original.request_key)?)? else {
                return Ok(None);
            };
            if ledger.request_fingerprint != original.request_fingerprint
                || ledger.locator.request_key != original.request_key
            {
                return Err(repository_error(
                    ContractOwnerRepositoryErrorKindV1::IdempotencyConflict,
                    "original request locator conflicts with exact owner formation",
                ));
            }
            owner.eval_current(&ledger.locator).map(Some)
        })
    }
    pub fn read_local_evaluation_history_v3(
        &self,
        locator: &LocalEvaluationLocatorV3,
    ) -> Result<LocalEvaluationHistoryV3, ContractOwnerRepositoryErrorV1> {
        self.with_existing_full_definition_lock(|owner| {
            owner.require_eval_locator_root(locator)?;
            let state = owner.eval_current(locator)?;
            let result = state
                .result
                .ok_or_else(|| invalid("original occurrence has no closed historical result"))?;
            let mut observations = vec![];
            let mut available = true;
            for dependency in &result.dependencies {
                let (status, reason) = match retained_evaluation_dependency_fingerprint(
                    &owner.repository_root,
                    &dependency.path,
                    &state.retained_lock_generations,
                ) {
                    Ok(Some(fingerprint)) if fingerprint == dependency.bytes_fingerprint => {
                        ("resolved", "exact_retained_owner_dependency")
                    }
                    Ok(Some(_)) => {
                        available = false;
                        ("unverifiable", "retained_dependency_substituted")
                    }
                    Ok(None) => {
                        available = false;
                        ("unavailable", "retained_dependency_missing")
                    }
                    Err(_) => {
                        available = false;
                        ("unverifiable", "retained_dependency_unobservable")
                    }
                };
                observations.push(LocalEvaluationHistoricalDependencyObservationV3 {
                    owner: dependency.owner.clone(),
                    path: dependency.path.clone(),
                    status: status.into(),
                    reason: reason.into(),
                });
            }
            for lock in &state.retained_lock_generations {
                let exact = owner
                    .repository_root
                    .existing_lock_physical_fingerprint_v1(&lock.reference)
                    .ok()
                    .as_deref()
                    == Some(lock.fingerprint.as_str());
                if !exact {
                    available = false;
                    observations.push(LocalEvaluationHistoricalDependencyObservationV3 {
                        owner: "native_owner_lock".into(),
                        path: lock.reference.clone(),
                        status: "unverifiable".into(),
                        reason: "original_physical_lock_generation_lost_or_substituted".into(),
                    });
                }
            }
            Ok(LocalEvaluationHistoryV3 {
                result,
                dependencies: observations,
                exact_replay_available: available,
            })
        })
    }
    pub fn read_local_evaluation_result_history_v3(
        &self,
        original: &LocalEvaluationResultLocatorV3,
    ) -> Result<LocalEvaluationHistoryV3, ContractOwnerRepositoryErrorV1> {
        let history = self.read_local_evaluation_history_v3(&original.evaluation)?;
        if history.result.identity != original.result {
            return Err(substituted(
                "historical request does not name original exact result",
            ));
        }
        Ok(history)
    }

    fn eval_mutation(
        &self,
        locator: &LocalEvaluationLocatorV3,
        admission: &LocalEvaluationAdmissionV1<'_>,
        checkpoint: LocalEvaluationCheckpointV1,
        target: Option<&str>,
        operation_key: &str,
        origin: Option<&ResolvedLocalEvaluationOriginV3<'_>>,
        mutate: impl FnOnce(
            &mut LocalEvaluationSnapshotV3,
        ) -> Result<(), ContractOwnerRepositoryErrorV1>,
    ) -> Result<LocalEvaluationSnapshotV3, ContractOwnerRepositoryErrorV1> {
        admission
            .require_repository_root(&self.repository_root)
            .map_err(|_| invalid("evaluation permission belongs to another retained root"))?;
        if !declared_read_subtree(
            &admission.request().reach.input_paths,
            ".handbook/state/contracts",
        ) {
            return Err(invalid(
                "evaluation omits complete owner inventory read subtree",
            ));
        }
        for path in [
            LOCAL_EVALUATION_OWNER_ROOT_V3,
            ".handbook/state/contracts/v1/transactions",
        ] {
            admission
                .require_owner_write_subtree(path)
                .map_err(|_| invalid("evaluation omits mandatory complete write subtree"))?;
        }
        if admission.checkpoint() != checkpoint
            || target.is_some_and(|t| admission.checkpoint_target() != Some(t))
        {
            return Err(invalid("evaluation consuming checkpoint is not exact"));
        }
        self.with_existing_full_definition_lock(|owner| {
            owner.recover_locked()?;
            owner.validate_inventory()?;
            let previous = owner.eval_current(locator)?;
            if admission.current_basis() != &previous.current_basis
                || admission.request().purpose != previous.purpose
                || admission.request().plan != previous.plan
                || admission.binding().purpose_fingerprint
                    != previous
                        .purpose
                        .fingerprint()
                        .map_err(|_| invalid("Purpose identity malformed"))?
            {
                return Err(invalid(
                    "evaluation issuance does not authorize exact owner Purpose/Plan/D/G basis",
                ));
            }
            let (definition, head) = owner.full_local_bundle(&previous.definition.binding())?;
            if definition.definition() != &previous.definition
                || head.lifecycle_fingerprint()
                    != &previous.original_input.expected_lifecycle_fingerprint
                || head.state() != LifecycleStateV1::Active
                || head.approving_review().is_none()
            {
                return Err(invalid(
                    "reviewed exact definition is no longer current for evaluation",
                ));
            }
            for path in owner.full_local_state_paths(&previous.definition.binding())? {
                admission
                    .require_input_path(&path)
                    .map_err(|_| invalid("evaluation omitted current definition proof path"))?;
            }
            let mut next = previous.clone();
            mutate(&mut next)?;
            if next == previous {
                return Ok(previous);
            }
            if next
                .authorization
                .as_ref()
                .is_some_and(|binding| binding != admission.binding())
            {
                return Err(invalid("authorization cannot change within one Purpose"));
            }
            next.authorization = Some(admission.binding().clone());
            if next
                .issuance_locator
                .as_ref()
                .is_some_and(|locator| locator != admission.issuance_locator())
            {
                return Err(invalid(
                    "original Engine issuance locator cannot be replaced",
                ));
            }
            next.issuance_locator = Some(admission.issuance_locator().clone());
            if next.stage == LocalEvaluationStageV3::ResultClosed {
                let mut dependencies = vec![];
                let mut paths = owner.full_local_state_paths(&next.definition.binding())?;
                paths.retain(|path| {
                    !path.starts_with(&format!("{LOCAL_CONTRACT_OWNER_ROOT_V3}/heads/"))
                });
                paths.extend(next.origin_paths.clone());
                paths.extend(
                    admission.historical_dependency_paths().map_err(|_| {
                        invalid("exact Engine retention dependencies are unavailable")
                    })?,
                );
                paths.push(LOCK_RELATIVE_PATH.into());
                paths.extend(owner.eval_immutable_history_paths(&next.locator)?);
                for path in ORIGINAL_EVALUATION_LOCK_PATHS {
                    admission.require_input_path(path).map_err(|_| {
                        invalid("final closure omitted original native lock proof reach")
                    })?;
                    paths.push(path.into());
                    next.retained_lock_generations.push(
                        handbook_engine::local_authority::LocalBindingV1 {
                            reference: path.into(),
                            fingerprint: owner
                                .repository_root
                                .existing_lock_physical_fingerprint_v1(path)
                                .map_err(map_durability)?,
                        },
                    );
                }
                paths.push(eval_ref("records", previous.identity.fingerprint()));
                paths.sort();
                paths.dedup();
                let horizon = SemanticTimeV1::parse(&next.purpose.retention_until_utc)
                    .map_err(map_contract_input)?;
                for path in paths {
                    admission.require_input_path(&path).map_err(|_| {
                        invalid("final closure omitted actual retained dependency proof reach")
                    })?;
                    let Some(bytes_fingerprint) = retained_evaluation_dependency_fingerprint(
                        &owner.repository_root,
                        &path,
                        &next.retained_lock_generations,
                    )?
                    else {
                        return Err(invalid(
                            "required historical dependency is already unavailable",
                        ));
                    };
                    dependencies.push(LocalEvaluationRetentionDependencyV3 {
                        owner: if path.contains("/dock/")
                            || path == ".handbook/state/locks/dock-operational-store.lock"
                        {
                            "handbook-dock"
                        } else if path.contains("/contracts/") || path == LOCK_RELATIVE_PATH {
                            "handbook-contracts"
                        } else {
                            "handbook-engine"
                        }
                        .into(),
                        path,
                        bytes_fingerprint,
                        retained_until: horizon.clone(),
                    });
                }
                let result = next.result.as_mut().unwrap();
                result.dependencies = dependencies;
                result.identity = identified(
                    "handbook.local-evaluation-result@3.0.0",
                    &without_identity(result).map_err(map_contract_input)?,
                )
                .map_err(map_contract_input)?;
            }
            next.sequence = previous
                .sequence
                .checked_add(1)
                .ok_or_else(|| bound("evaluation sequence exhausted"))?;
            next.identity = state_identity(&next)?;
            let request = canonical_fingerprint(&(
                EVALUATION_OPERATION,
                &previous.identity,
                operation_key,
                &next.identity,
            ))
            .map_err(map_contract_input)?;
            let key = eval_key(&format!(
                "{}:{}:{operation_key}",
                locator.request_key, previous.sequence
            ))?;
            owner.commit_eval_state(Some(&previous), next, &request, &key, operation_key, || {
                admission.validate_current().map_err(|_| {
                    invalid("evaluation authorization changed before consuming commit")
                })?;
                if let Some(origin) = origin {
                    origin.validate_current().map_err(map_contract_input)?;
                }
                Ok(())
            })
        })
    }
    fn commit_eval_state(
        &self,
        previous: Option<&LocalEvaluationSnapshotV3>,
        state: LocalEvaluationSnapshotV3,
        request: &FingerprintV1,
        key: &FingerprintV1,
        operation_key: &str,
        check: impl FnOnce() -> Result<(), ContractOwnerRepositoryErrorV1>,
    ) -> Result<LocalEvaluationSnapshotV3, ContractOwnerRepositoryErrorV1> {
        self.ensure_evaluation_layout()?;
        validate_eval_state(&state)?;
        let ledger = EvaluationLedger {
            operation_key: operation_key.into(),
            key_fingerprint: key.clone(),
            request_fingerprint: request.clone(),
            locator: state.locator.clone(),
            record: state.identity.clone(),
        };
        let bytes = [
            canonical_bytes(&state).map_err(map_contract_input)?,
            canonical_bytes(&state.identity).map_err(map_contract_input)?,
            canonical_bytes(&ledger).map_err(map_contract_input)?,
        ];
        let outputs = vec![
            StagedOutput::new(
                "00-transition.json",
                &eval_ref("records", state.identity.fingerprint()),
                &bytes[0],
            )?,
            StagedOutput::new(
                "01-head.json",
                &eval_ref("heads", state.locator.occurrence.fingerprint()),
                &bytes[1],
            )?,
            StagedOutput::new("02-ledger.json", &eval_ref("idempotency", key), &bytes[2])?,
        ];
        let id = canonical_fingerprint(&TransactionIdPreimage {
            schema_ref: "handbook.contract-owner-transaction-identity@1.0.0",
            operation: EVALUATION_OPERATION,
            key_fingerprint: key,
            request_fingerprint: request,
        })
        .map_err(map_contract_input)?;
        let pending = self
            .transactions_root()
            .join(format!("{}.pending", fingerprint_hex(&id)));
        let committed = self
            .transactions_root()
            .join(format!("{}.committed", fingerprint_hex(&id)));
        if self.exists(&pending)? || self.exists(&committed)? {
            return Err(recovery(
                "evaluation transition transaction identity occupied",
            ));
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
        let prior = previous
            .map(|p| canonical_bytes(&p.identity).map(|bytes| FingerprintV1::from_bytes(&bytes)))
            .transpose()
            .map_err(map_contract_input)?;
        let intent = TransactionIntent::new(
            fingerprint_hex(&id),
            EVALUATION_OPERATION,
            key,
            request,
            prior,
            outputs,
        )?;
        check()?;
        self.write_new_durable(
            &pending.join("intent.json"),
            &canonical_bytes(&intent).map_err(map_contract_input)?,
        )?;
        self.maybe_fault(OwnerFaultBoundary::Intent)?;
        let marker = CommitMarker::new(&intent)?;
        self.write_new_durable(
            &pending.join("commit-marker.json"),
            &canonical_bytes(&marker).map_err(map_contract_input)?,
        )?;
        self.maybe_fault(OwnerFaultBoundary::Marker)?;
        self.finish_transaction(&pending, &intent, &marker)?;
        self.rename_dir_durable(&pending, &committed)?;
        self.maybe_fault(OwnerFaultBoundary::CommittedRename)?;
        self.validate_inventory()?;
        Ok(state)
    }
    fn require_eval_locator_root(
        &self,
        locator: &LocalEvaluationLocatorV3,
    ) -> Result<(), ContractOwnerRepositoryErrorV1> {
        if locator.owner_identity.as_str()
            != self
                .repository_root
                .governance_physical_root_fingerprint_v1()
                .map_err(map_durability)?
            || locator.lock_generation
                != self
                    .repository_root
                    .existing_lock_physical_fingerprint_v1(LOCK_RELATIVE_PATH)
                    .map_err(map_durability)?
        {
            return Err(recovery(
                "original evaluation owner root or lock is lost or substituted",
            ));
        }
        Ok(())
    }
    fn eval_current(
        &self,
        locator: &LocalEvaluationLocatorV3,
    ) -> Result<LocalEvaluationSnapshotV3, ContractOwnerRepositoryErrorV1> {
        self.require_eval_locator_root(locator)?;
        let head: ContentAddressedBindingV1 = parse_canonical(
            &self.read_regular(
                &self
                    .repo_root
                    .join(eval_ref("heads", locator.occurrence.fingerprint())),
            )?,
        )?;
        let mut states = self.eval_retained_history(locator)?;
        let state = states
            .pop()
            .ok_or_else(|| recovery("original occurrence has no committed allocation history"))?;
        validate_eval_state(&state)?;
        if state.locator != *locator || state.identity != head {
            return Err(substituted("exact evaluation locator/head is substituted"));
        }
        Ok(state)
    }
    fn eval_retained_history(
        &self,
        locator: &LocalEvaluationLocatorV3,
    ) -> Result<Vec<LocalEvaluationSnapshotV3>, ContractOwnerRepositoryErrorV1> {
        let mut records = BTreeMap::new();
        for path in self.read_dir_bounded(&self.transactions_root(), MAX_RECORDS)? {
            if !file_name(&path)?.ends_with(".committed") {
                continue;
            }
            let intent: TransactionIntent =
                parse_canonical(&self.read_regular(&path.join("intent.json"))?)?;
            if intent.operation != EVALUATION_OPERATION
                || intent.outputs[1].relative_ref
                    != eval_ref("heads", locator.occurrence.fingerprint())
            {
                continue;
            }
            self.validate_transaction(&path)?;
            let state: LocalEvaluationSnapshotV3 =
                parse_canonical(&self.read_regular(&path.join("00-transition.json"))?)?;
            if state.locator != *locator
                || records.insert(state.sequence, (state, intent)).is_some()
            {
                return Err(recovery(
                    "occurrence has conflicting committed sequence or locator",
                ));
            }
        }
        let mut output = vec![];
        for (expected, (sequence, (state, intent))) in records.into_iter().enumerate() {
            if sequence != expected as u64 {
                return Err(recovery(
                    "occurrence immutable transition history has a gap",
                ));
            }
            if let Some(previous) = output.last() {
                validate_eval_transition(previous, &state)?;
                if intent.expected_head_bytes_fingerprint
                    != Some(FingerprintV1::from_bytes(
                        &canonical_bytes(&previous.identity).map_err(map_contract_input)?,
                    ))
                {
                    return Err(recovery(
                        "transition expected head does not close exact predecessor",
                    ));
                }
                let ledger: EvaluationLedger = parse_canonical(
                    &self.read_regular(&self.repo_root.join(&intent.outputs[2].relative_ref))?,
                )?;
                if intent.request_fingerprint
                    != canonical_fingerprint(&(
                        EVALUATION_OPERATION,
                        &previous.identity,
                        &ledger.operation_key,
                        &state.identity,
                    ))
                    .map_err(map_contract_input)?
                    || intent.key_fingerprint
                        != eval_key(&format!(
                            "{}:{}:{}",
                            locator.request_key, previous.sequence, ledger.operation_key
                        ))?
                {
                    return Err(recovery(
                        "transition request does not close exact operation and predecessor",
                    ));
                }
            } else if state.stage != LocalEvaluationStageV3::Purpose
                || intent.expected_head_bytes_fingerprint.is_some()
                || intent.request_fingerprint != locator.request_fingerprint
                || intent.key_fingerprint != eval_key(&locator.request_key)?
            {
                return Err(recovery(
                    "allocation does not close original request ledger",
                ));
            }
            output.push(state);
        }
        Ok(output)
    }
    fn eval_immutable_history_paths(
        &self,
        locator: &LocalEvaluationLocatorV3,
    ) -> Result<Vec<String>, ContractOwnerRepositoryErrorV1> {
        let mut output = vec![];
        for state in self.eval_retained_history(locator)? {
            output.push(eval_ref("records", state.identity.fingerprint()));
        }
        for path in self.read_dir_bounded(&self.transactions_root(), MAX_RECORDS)? {
            if !file_name(&path)?.ends_with(".committed") {
                continue;
            }
            let intent: TransactionIntent =
                parse_canonical(&self.read_regular(&path.join("intent.json"))?)?;
            if intent.operation != EVALUATION_OPERATION
                || intent.outputs[1].relative_ref
                    != eval_ref("heads", locator.occurrence.fingerprint())
            {
                continue;
            }
            for entry in self.read_dir_bounded(&path, 8)? {
                output.push(self.relative_path(&entry)?);
            }
            output.push(intent.outputs[2].relative_ref.clone());
        }
        Ok(output)
    }
    fn eval_ledger(
        &self,
        key: &FingerprintV1,
    ) -> Result<Option<EvaluationLedger>, ContractOwnerRepositoryErrorV1> {
        if !self.exists(&self.repo_root.join(LOCAL_EVALUATION_OWNER_ROOT_V3))? {
            return Ok(None);
        }
        let path = self.repo_root.join(eval_ref("idempotency", key));
        if !self.exists(&path)? {
            return Ok(None);
        }
        parse_canonical(&self.read_regular(&path)?).map(Some)
    }
    fn eval_max_ordinal(&self) -> Result<u64, ContractOwnerRepositoryErrorV1> {
        if !self.exists(&self.repo_root.join(LOCAL_EVALUATION_OWNER_ROOT_V3))? {
            return Ok(0);
        }
        let mut maximum = 0;
        let mut allocations = BTreeSet::new();
        for path in self.read_dir_bounded(
            &self
                .repo_root
                .join(format!("{LOCAL_EVALUATION_OWNER_ROOT_V3}/records")),
            MAX_RECORDS,
        )? {
            let state: LocalEvaluationSnapshotV3 = parse_canonical(&self.read_regular(&path)?)?;
            validate_eval_state(&state)?;
            if state.sequence == 0 && !allocations.insert(state.ordinal) {
                return Err(recovery("owner occurrence allocation ordinal reused"));
            }
            maximum = maximum.max(state.ordinal);
        }
        Ok(maximum)
    }
    pub(super) fn ensure_evaluation_layout(&self) -> Result<(), ContractOwnerRepositoryErrorV1> {
        for suffix in ["", "/records", "/heads", "/idempotency"] {
            self.repository_root
                .create_directories(&format!("{LOCAL_EVALUATION_OWNER_ROOT_V3}{suffix}"))
                .map_err(map_durability)?;
        }
        Ok(())
    }
    pub(super) fn validate_evaluation_inventory(
        &self,
    ) -> Result<(), ContractOwnerRepositoryErrorV1> {
        let mut classes = [BTreeSet::new(), BTreeSet::new(), BTreeSet::new()];
        for path in self.read_dir_bounded(&self.transactions_root(), MAX_RECORDS)? {
            let intent: TransactionIntent =
                parse_canonical(&self.read_regular(&path.join("intent.json"))?)?;
            if intent.operation != EVALUATION_OPERATION {
                continue;
            }
            for (i, out) in intent.outputs.iter().enumerate() {
                if !classes[i].insert(out.relative_ref.clone()) && i != 1 {
                    return Err(recovery(
                        "immutable evaluation output has multiple transactions",
                    ));
                }
            }
        }
        if !self.exists(&self.repo_root.join(LOCAL_EVALUATION_OWNER_ROOT_V3))? {
            if classes.iter().any(|s| !s.is_empty()) {
                return Err(recovery("evaluation namespace missing"));
            }
            return Ok(());
        }
        for (class, expected) in ["records", "heads", "idempotency"].iter().zip(classes) {
            self.require_inventory(
                &self
                    .repo_root
                    .join(format!("{LOCAL_EVALUATION_OWNER_ROOT_V3}/{class}")),
                &expected,
            )?;
        }
        self.eval_max_ordinal()?;
        for path in self.read_dir_bounded(
            &self
                .repo_root
                .join(format!("{LOCAL_EVALUATION_OWNER_ROOT_V3}/heads")),
            MAX_RECORDS,
        )? {
            let head: ContentAddressedBindingV1 = parse_canonical(&self.read_regular(&path)?)?;
            let state: LocalEvaluationSnapshotV3 = parse_canonical(
                &self
                    .read_regular(&self.repo_root.join(eval_ref("records", head.fingerprint())))?,
            )?;
            self.eval_current(&state.locator)?;
        }
        Ok(())
    }
    pub(super) fn validate_evaluation_transaction(
        &self,
        path: &Path,
        intent: &TransactionIntent,
        primary: &[u8],
        head: &[u8],
        ledger: &[u8],
    ) -> Result<(), ContractOwnerRepositoryErrorV1> {
        let state: LocalEvaluationSnapshotV3 = parse_canonical(primary)?;
        validate_eval_state(&state)?;
        let head: ContentAddressedBindingV1 = parse_canonical(head)?;
        let ledger: EvaluationLedger = parse_canonical(ledger)?;
        if state.identity != head
            || ledger.record != head
            || ledger.locator != state.locator
            || ledger.key_fingerprint != intent.key_fingerprint
            || ledger.request_fingerprint != intent.request_fingerprint
            || intent.outputs[0].relative_ref != eval_ref("records", head.fingerprint())
            || intent.outputs[1].relative_ref
                != eval_ref("heads", state.locator.occurrence.fingerprint())
            || intent.outputs[2].relative_ref != eval_ref("idempotency", &ledger.key_fingerprint)
            || (state.sequence == 0) != intent.expected_head_bytes_fingerprint.is_none()
        {
            return Err(recovery(
                "evaluation transaction does not close exact state/head/ledger",
            ));
        }
        for output in &intent.outputs {
            let staged = self.read_regular(&path.join(&output.staged_name))?;
            if FingerprintV1::from_bytes(&staged) != output.bytes_fingerprint
                || (output.staged_name != "01-head.json"
                    && staged != self.read_regular(&self.repo_root.join(&output.relative_ref))?)
            {
                return Err(recovery(
                    "evaluation immutable output substituted or unreachable",
                ));
            }
        }
        Ok(())
    }
}
fn eval_key(key: &str) -> Result<FingerprintV1, ContractOwnerRepositoryErrorV1> {
    canonical_fingerprint(&("handbook.local-evaluation-owner-request-key@3.0.0", key))
        .map_err(map_contract_input)
}
fn eval_ref(class: &str, fp: &FingerprintV1) -> String {
    format!(
        "{LOCAL_EVALUATION_OWNER_ROOT_V3}/{class}/{}.json",
        fingerprint_hex(fp)
    )
}
fn state_identity(
    state: &LocalEvaluationSnapshotV3,
) -> Result<ContentAddressedBindingV1, ContractOwnerRepositoryErrorV1> {
    identified(
        DOMAIN,
        &without_identity(state).map_err(map_contract_input)?,
    )
    .map_err(map_contract_input)
}
fn prospective_basis(
    state: &LocalEvaluationSnapshotV3,
) -> Result<ContentAddressedBindingV1, ContractOwnerRepositoryErrorV1> {
    let readiness = state
        .readiness_bases
        .last()
        .ok_or_else(|| invalid("complete readiness is required"))?;
    if readiness.disposition != "attemptable" {
        return Err(invalid(
            "held readiness forms no prospective evaluation basis",
        ));
    }
    identified(
        "handbook.local-prospective-evaluation-basis@3.0.0",
        &(
            &state.purpose,
            &state.plan,
            &state.requirements,
            &state.authorization,
            readiness,
        ),
    )
    .map_err(map_contract_input)
}
fn require_basis_target(
    state: &LocalEvaluationSnapshotV3,
    admission: &LocalEvaluationAdmissionV1<'_>,
) -> Result<(), ContractOwnerRepositoryErrorV1> {
    if state
        .committed_basis
        .as_ref()
        .map(|b| b.fingerprint().as_str())
        != admission.checkpoint_target()
    {
        return Err(invalid("checkpoint must name exact committed basis"));
    }
    Ok(())
}
fn declared_read(paths: &[String], path: &str) -> bool {
    paths.iter().any(|s| {
        s == path
            || s.strip_suffix("/**")
                .is_some_and(|root| path == root || path.starts_with(&format!("{root}/")))
    })
}
fn declared_read_subtree(paths: &[String], path: &str) -> bool {
    paths.iter().any(|s| {
        s.strip_suffix("/**")
            .is_some_and(|root| path == root || path.starts_with(&format!("{root}/")))
    })
}
fn resolution_ranks(
    admission: &LocalEvaluationAdmissionV1<'_>,
    values: [&StableIdV1; 6],
) -> Result<[u8; 6], ContractOwnerRepositoryErrorV1> {
    let coordinates = crate::evidence_admission::engine_resolution_values(values)
        .map_err(|_| invalid("Resolution coordinates are malformed"))?;
    admission
        .resolution_dimension_ranks(coordinates.each_ref().map(String::as_str))
        .map_err(|_| invalid("Resolution coordinates cannot resolve in exact admitted stack"))
}
fn validate_eval_state(
    state: &LocalEvaluationSnapshotV3,
) -> Result<(), ContractOwnerRepositoryErrorV1> {
    state.definition.validate().map_err(map_contract_input)?;
    state
        .purpose
        .validate()
        .map_err(|_| recovery("retained Purpose malformed"))?;
    let project_identity = match &state.requirements.applicability.governance_basis {
        crate::project_admission_governance::ProjectAdmissionScopeBasisV3::DeferredV2(basis) =>
            &basis.project.repository_identity_fingerprint,
        crate::project_admission_governance::ProjectAdmissionScopeBasisV3::EstablishedFromDeferredV3(basis) =>
            &basis.project.repository_identity_fingerprint,
        _ => return Err(recovery("retained Purpose has an unsupported project basis")),
    };
    if state.identity != state_identity(state)?
        || state.requirements.fingerprint
            != state
                .requirements
                .expected_fingerprint()
                .map_err(map_contract_input)?
        || state.requirements.contract != state.definition.binding()
        || state.requirements.source != "direct_contract"
        || !state.requirements.metrics.is_empty()
        || state.requirements.claims != state.definition.semantic_definition().claims()
        || state.requirements.dependencies != *state.definition.full_dependencies()
        || state.purpose.occurrence_identity != state.locator.occurrence.fingerprint().as_str()
        || (state.authorization.is_some() && &state.purpose.project_identity != project_identity)
        || state.purpose.requirements_fingerprint != state.requirements.fingerprint.as_str()
        || state.purpose.plan_fingerprint
            != state
                .plan
                .fingerprint()
                .map_err(|_| recovery("Plan identity unavailable"))?
        || !state.run.is_local_evaluation_v3()
        || state.run.contract() != state.definition.binding()
        || state.run.nonce().as_str() != state.locator.occurrence.fingerprint().as_str()
        || state.evidence.len() > 1
        || state.ordinal == 0
        || state.sequence > 1024
    {
        return Err(recovery("retained evaluation exact closure is substituted"));
    }
    let occurrence = canonical_fingerprint(&(
        "handbook.contract-local-occurrence-allocation@3.0.0",
        &state.locator.owner_identity,
        state.ordinal,
    ))
    .map_err(map_contract_input)?;
    if state.locator.occurrence.fingerprint() != &occurrence
        || state.purpose.owner_identity != state.locator.owner_identity.as_str()
        || state.purpose.contract.reference != state.definition.binding().reference().as_str()
        || state.purpose.contract.fingerprint != state.definition.binding().fingerprint().as_str()
        || state.requirements.selected != vec![state.definition.binding()]
        || !state.requirements.excluded.is_empty()
        || !state.requirements.missing.is_empty()
        || !state.requirements.conflicting.is_empty()
        || state.plan.requirements_fingerprint != state.requirements.fingerprint.as_str()
        || state.plan.selections[0].requested_reach
            != state.original_input.formation.requested_reach
        || state.run.subject() != &state.requirements.subject
        || state.run.cases() != [state.requirements.case.clone()]
    {
        return Err(recovery(
            "retained owner allocation or nomination closure is substituted",
        ));
    }
    for basis in &state.readiness_bases {
        basis.capability.validate().map_err(map_contract_input)?;
        let mut value =
            serde_json::to_value(basis).map_err(|_| recovery("readiness object malformed"))?;
        value.as_object_mut().unwrap().remove("fingerprint");
        if basis.fingerprint != canonical_fingerprint(&value).map_err(map_contract_input)?
            || basis.verification.checkpoint != LocalEvaluationCheckpointV1::Preparation
            || basis.verification.purpose_fingerprint
                != state
                    .purpose
                    .fingerprint()
                    .map_err(|_| recovery("Purpose malformed"))?
        {
            return Err(recovery("retained readiness identity substituted"));
        }
    }
    if state
        .committed_basis
        .as_ref()
        .is_some_and(|basis| prospective_basis(state).ok().as_ref() != Some(basis))
    {
        return Err(recovery(
            "committed basis differs from complete retained readiness",
        ));
    }
    for evidence in &state.evidence {
        if evidence.identity
            != identified(
                "handbook.local-canonical-evidence@3.0.0",
                &without_identity(evidence).map_err(map_contract_input)?,
            )
            .map_err(map_contract_input)?
            || evidence.occurrence != state.locator.occurrence
            || Some(&evidence.basis) != state.committed_basis.as_ref()
            || Some(&evidence.attempt) != state.attempt.as_ref()
            || evidence.selection != *state.selection()
            || evidence.admission_policy
                != state
                    .definition
                    .full_dependencies()
                    .evidence_admission_policy
            || evidence
                .candidate
                .admission_body()
                .evaluation_run()
                .fingerprint()
                != state.run.evaluation_run_fingerprint()
            || evidence.verification.checkpoint != LocalEvaluationCheckpointV1::EvidenceAppend
        {
            return Err(recovery(
                "retained canonical evidence identity or basis is substituted",
            ));
        }
    }
    if let Some(frontier) = &state.frontier {
        if frontier.identity
            != identified(
                "handbook.local-frontier@3.0.0",
                &without_identity(frontier).map_err(map_contract_input)?,
            )
            .map_err(map_contract_input)?
            || frontier.occurrence != state.locator.occurrence
            || Some(&frontier.basis) != state.committed_basis.as_ref()
            || Some(&frontier.attempt) != state.attempt.as_ref()
            || Some(&frontier.origin_terminal) != state.origin_terminal.as_ref()
        {
            return Err(recovery(
                "retained frontier does not close exact collection",
            ));
        }
    }
    if let Some(set) = &state.evidence_set {
        validate_closed_population(
            &state
                .frontier
                .as_ref()
                .ok_or_else(|| recovery("closed set has no original frontier"))?
                .origin_terminal,
            &state.admission_terminals,
            &population_evidence(&state.evidence)?,
        )?;
        if set.identity
            != identified(
                "handbook.local-evidence-set@3.0.0",
                &without_identity(set).map_err(map_contract_input)?,
            )
            .map_err(map_contract_input)?
            || set.occurrence != state.locator.occurrence
            || Some(&set.basis) != state.committed_basis.as_ref()
            || state.frontier.as_ref().map(|f| &f.identity) != Some(&set.frontier)
            || set.members
                != state
                    .evidence
                    .iter()
                    .map(|e| e.identity.clone())
                    .collect::<Vec<_>>()
            || set.admission_terminals != state.admission_terminals
            || set.eligible_owner != state.locator.owner_identity
            || set.eligible_namespace != LOCAL_EVALUATION_OWNER_ROOT_V3
            || set.nonmembers
                != state
                    .admission_terminals
                    .iter()
                    .filter(|t| t.disposition == "refused")
                    .cloned()
                    .collect::<Vec<_>>()
        {
            return Err(recovery(
                "retained set membership or terminal accounting substituted",
            ));
        }
    }
    if let Some(result) = &state.result {
        let (claim, gate) =
            evaluate_closed_local_context_v3(state, &result.evidence_set, &result.evaluated_at)
                .map_err(map_contract_input)?;
        let context = identified(
            "handbook.local-final-context@3.0.0",
            &(
                &state.locator.occurrence,
                &state.purpose,
                &state.requirements,
                &state.plan,
                &state.authorization,
                &state.readiness_bases,
                &state.committed_basis,
                &state.frontier,
                &result.evidence_set,
                &result.final_verification,
                &result.evaluated_at,
            ),
        )
        .map_err(map_contract_input)?;
        if result.identity
            != identified(
                "handbook.local-evaluation-result@3.0.0",
                &without_identity(result).map_err(map_contract_input)?,
            )
            .map_err(map_contract_input)?
            || result.occurrence != state.locator.occurrence
            || result.purpose != state.purpose
            || Some(&result.evidence_set) != state.evidence_set.as_ref()
            || result.claim != claim
            || result.gate != gate
            || result.context != context
            || result.explanation
                != result_explanation(
                    state,
                    &claim,
                    gate,
                    &result.evidence_set,
                    &result.final_verification,
                )
            || !result.metrics.is_empty()
            || result.gate_policy != state.definition.full_dependencies().evaluation_gate
            || result.final_verification.checkpoint != LocalEvaluationCheckpointV1::FinalContext
            || result.final_verification.checkpoint_target.as_deref()
                != Some(result.evidence_set.identity.fingerprint().as_str())
            || result
                .dependencies
                .iter()
                .any(|dependency| dependency.retained_until < result.retained_until)
        {
            return Err(recovery("retained truth or final context is substituted"));
        }
    }
    let valid_stage = match state.stage {
        LocalEvaluationStageV3::Purpose => {
            state.authorization.is_none()
                && state.readiness_bases.is_empty()
                && state.committed_basis.is_none()
        }
        LocalEvaluationStageV3::Held => {
            state
                .readiness_bases
                .last()
                .is_some_and(|b| b.disposition == "held")
                && state.committed_basis.is_none()
        }
        LocalEvaluationStageV3::Attemptable => {
            state
                .readiness_bases
                .last()
                .is_some_and(|b| b.disposition == "attemptable")
                && state.committed_basis.is_none()
        }
        LocalEvaluationStageV3::Committed => {
            state.committed_basis.is_some() && state.origin_terminal.is_none()
        }
        LocalEvaluationStageV3::OriginClosed => {
            state.committed_basis.is_some()
                && state.attempt.is_some()
                && state.origin_terminal.is_some()
                && state.frontier.is_none()
        }
        LocalEvaluationStageV3::FrontierSealed => {
            state.frontier.is_some() && state.evidence_set.is_none()
        }
        LocalEvaluationStageV3::SetClosed => state.evidence_set.is_some() && state.result.is_none(),
        LocalEvaluationStageV3::ResultClosed => state.result.is_some(),
        LocalEvaluationStageV3::Refused => state.reason.is_some(),
    };
    if !valid_stage {
        return Err(recovery(
            "retained evaluation stage is inconsistent with its closure",
        ));
    }
    Ok(())
}
fn result_explanation(
    state: &LocalEvaluationSnapshotV3,
    claim: &LocalEvaluationClaimResultV3,
    gate: LocalEvaluationGateV3,
    set: &LocalEvaluationEvidenceSetV3,
    verification: &LocalEvaluationVerificationObservationV1,
) -> serde_json::Value {
    json!({
        "source":"direct_contract", "intent":state.purpose.use_intent,
        "permitted_use":state.requirements.permitted_use,
        "applicability":state.requirements.applicability,
        "gate_policy":state.definition.full_dependencies().evaluation_gate,
        "claim":claim, "metrics":[], "gate":gate,
        "readiness":state.readiness_bases,
        "committed_basis":state.committed_basis, "attempt":state.attempt,
        "origin_terminal":state.origin_terminal, "frontier":state.frontier,
        "included":set.members, "excluded":set.nonmembers,
        "admission_terminals":set.admission_terminals, "final_verification":verification,
        "shipment_or_completion_obligation":"not_assessed",
        "authority_mode":"local_advisory", "network_enforcement":"not_provided"
    })
}
fn population_evidence(
    evidence: &[LocalCanonicalEvidenceV3],
) -> Result<
    Vec<(ContentAddressedBindingV1, ContentAddressedBindingV1)>,
    ContractOwnerRepositoryErrorV1,
> {
    evidence
        .iter()
        .map(|record| {
            Ok((
                ContentAddressedBindingV1::new(
                    record.candidate.candidate_ref(),
                    record.candidate.candidate_fingerprint().clone(),
                )
                .map_err(map_contract_input)?,
                record.identity.clone(),
            ))
        })
        .collect()
}
/// Live closure and historical admission share this exact finite accounting.
/// These identities describe retained populations; they grant no source facts.
fn validate_closed_population(
    origin: &serde_json::Value,
    terminals: &[LocalEvaluationAdmissionTerminalV3],
    evidence: &[(ContentAddressedBindingV1, ContentAddressedBindingV1)],
) -> Result<Vec<ContentAddressedBindingV1>, ContractOwnerRepositoryErrorV1> {
    let candidates: Vec<ContentAddressedBindingV1> =
        serde_json::from_value(origin["candidates"].clone())
            .map_err(|_| invalid("frontier candidate population malformed"))?;
    let kind = origin["kind"].as_str().unwrap_or("");
    if !matches!(
        kind,
        "completed"
            | "timed_out"
            | "cancelled"
            | "failed"
            | "refused"
            | "protocol_error"
            | "cancelled_before_launch"
            | "refused_before_launch"
    ) || (kind == "completed" && candidates.len() != 1)
        || (kind != "completed" && !candidates.is_empty())
        || evidence.len() > 1
        || candidates.iter().any(|candidate| {
            !terminals
                .iter()
                .any(|terminal| &terminal.candidate == candidate)
        })
    {
        return Err(invalid(
            "incomplete or uncertain origin population cannot close",
        ));
    }
    for (index, terminal) in terminals.iter().enumerate() {
        if !candidates.contains(&terminal.candidate)
            || terminal.operation_id.is_empty()
            || terminal.reason.is_empty()
            || terminals[..index].iter().any(|old| {
                old.operation_id == terminal.operation_id && old.candidate == terminal.candidate
            })
        {
            return Err(invalid("extra conflicting or repeated admission operation"));
        }
        match terminal.disposition.as_str() {
            "admitted" | "duplicate" => {
                if !evidence.iter().any(|(candidate, identity)| {
                    candidate == &terminal.candidate && Some(identity) == terminal.evidence.as_ref()
                }) {
                    return Err(invalid(
                        "admitted unfavorable or favorable evidence was omitted",
                    ));
                }
            }
            "refused" if terminal.evidence.is_none() => (),
            _ => {
                return Err(invalid(
                    "admission operation lacks an exact terminal disposition",
                ))
            }
        }
    }
    for (candidate, identity) in evidence {
        if !candidates.contains(candidate)
            || terminals
                .iter()
                .filter(|terminal| {
                    &terminal.candidate == candidate
                        && terminal.disposition == "admitted"
                        && terminal.evidence.as_ref() == Some(identity)
                })
                .count()
                != 1
        {
            return Err(invalid(
                "canonical member lacks exactly one original admission",
            ));
        }
    }
    Ok(candidates)
}
fn validate_eval_transition(
    previous: &LocalEvaluationSnapshotV3,
    next: &LocalEvaluationSnapshotV3,
) -> Result<(), ContractOwnerRepositoryErrorV1> {
    use LocalEvaluationStageV3::*;
    let legal = matches!(
        (previous.stage, next.stage),
        (Purpose, Held | Attemptable)
            | (Held | Attemptable, Held | Attemptable)
            | (Attemptable, Committed)
            | (Committed, Committed | OriginClosed)
            | (OriginClosed, OriginClosed | FrontierSealed)
            | (FrontierSealed, SetClosed)
            | (SetClosed, ResultClosed)
    );
    if !legal {
        return Err(recovery(
            "occurrence history contains an unsupported transition",
        ));
    }
    if let Some(result) = &next.result {
        let expected =
            FingerprintV1::from_bytes(&canonical_bytes(previous).map_err(map_contract_input)?);
        if result
            .final_verification
            .checkpoint_source_fingerprint
            .as_deref()
            != Some(expected.as_str())
        {
            return Err(recovery(
                "final verification did not observe exact committed predecessor set bytes",
            ));
        }
    }
    let extends = |old: &[LocalEvaluationAdmissionTerminalV3],
                   new: &[LocalEvaluationAdmissionTerminalV3]| {
        new.starts_with(old)
    };
    if next.sequence != previous.sequence + 1
        || previous.locator != next.locator
        || previous.ordinal != next.ordinal
        || previous.requirements != next.requirements
        || previous.purpose != next.purpose
        || previous.plan != next.plan
        || previous.run != next.run
        || previous.current_basis != next.current_basis
        || previous.definition != next.definition
        || previous.original_input != next.original_input
        || previous.formation_authority_fingerprint != next.formation_authority_fingerprint
        || previous.stage == LocalEvaluationStageV3::ResultClosed
        || previous
            .authorization
            .as_ref()
            .is_some_and(|value| Some(value) != next.authorization.as_ref())
        || previous
            .issuance_locator
            .as_ref()
            .is_some_and(|value| Some(value) != next.issuance_locator.as_ref())
        || previous.committed_basis.as_ref().is_some_and(|value| {
            Some(value) != next.committed_basis.as_ref()
                || previous.readiness_bases != next.readiness_bases
        })
        || previous
            .attempt
            .as_ref()
            .is_some_and(|value| Some(value) != next.attempt.as_ref())
        || previous
            .origin_terminal
            .as_ref()
            .is_some_and(|value| Some(value) != next.origin_terminal.as_ref())
        || previous
            .frontier
            .as_ref()
            .is_some_and(|value| Some(value) != next.frontier.as_ref())
        || previous
            .evidence_set
            .as_ref()
            .is_some_and(|value| Some(value) != next.evidence_set.as_ref())
        || !next.readiness_bases.starts_with(&previous.readiness_bases)
        || !next
            .collection_checks
            .starts_with(&previous.collection_checks)
        || !next.evidence.starts_with(&previous.evidence)
        || !extends(&previous.admission_terminals, &next.admission_terminals)
    {
        return Err(recovery(
            "immutable occurrence history forks, rolls back or replaces committed content",
        ));
    }
    Ok(())
}
pub(super) fn validate_evaluation_intent(
    intent: &TransactionIntent,
) -> Result<(), ContractOwnerRepositoryErrorV1> {
    if intent.schema_ref != "handbook.contract-owner-transaction-intent@1.0.0"
        || intent.operation != EVALUATION_OPERATION
        || intent.outputs.len() != 3
    {
        return Err(recovery("evaluation intent closed shape invalid"));
    }
    for (out, (name, class)) in intent.outputs.iter().zip([
        ("00-transition.json", "records"),
        ("01-head.json", "heads"),
        ("02-ledger.json", "idempotency"),
    ]) {
        validate_relative(&out.relative_ref)?;
        validate_staged(&out.staged_name)?;
        let prefix = format!("{LOCAL_EVALUATION_OWNER_ROOT_V3}/{class}/");
        let fp = out
            .relative_ref
            .strip_prefix(&prefix)
            .and_then(|s| s.strip_suffix(".json"))
            .ok_or_else(|| recovery("evaluation output escapes owner namespace"))?;
        if out.staged_name != name || FingerprintV1::parse(&format!("sha256:{fp}")).is_err() {
            return Err(recovery("evaluation output identity invalid"));
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
        return Err(recovery("evaluation intent identity substituted"));
    }
    Ok(())
}

#[cfg(test)]
mod population_tests {
    use super::*;

    #[test]
    #[ignore = "requires an original immutable native owner evaluation record"]
    fn malformed_retained_plans_refuse_without_writing_original_native_state() {
        let path = std::env::var("PACKET3_ORIGINAL_EVALUATION_RECORD").unwrap();
        let bytes = std::fs::read(&path).unwrap();
        let original: LocalEvaluationSnapshotV3 = serde_json::from_slice(&bytes).unwrap();
        assert_eq!(canonical_bytes(&original).unwrap(), bytes);
        validate_eval_state(&original).unwrap();
        let original_plan = serde_json::to_value(&original.plan).unwrap();
        assert_eq!(
            canonical_fingerprint(&json!({
                "domain":"handbook.local-evaluation-plan-identity@1.0.0",
                "content":original_plan,
            }))
            .unwrap()
            .as_str(),
            original.purpose.plan_fingerprint,
        );
        let mut cases = vec![
            ("/schema_ref", json!("handbook.unsupported-plan@1.0.0")),
            ("/requirements_fingerprint", json!("not-a-fingerprint")),
            ("/composition", json!("parallel")),
            ("/max_attempts", json!(0)),
            ("/max_attempts", json!(2)),
            ("/selections", json!([])),
            (
                "/selections",
                json!([
                    original_plan["selections"][0].clone(),
                    original_plan["selections"][0].clone()
                ]),
            ),
            ("/selections/0/selection_id", json!("invalid selection")),
            ("/selections/0/role_ref", json!("unsupported_role")),
            (
                "/selections/0/producer_ref",
                json!("handbook.unsupported-producer@1.0.0"),
            ),
            (
                "/selections/0/requested_reach/profile/reference",
                json!("handbook.unsupported-profile@1.0.0"),
            ),
            (
                "/selections/0/requested_reach/stack/reference",
                json!("handbook.unsupported-stack@1.0.0"),
            ),
            (
                "/selections/0/requested_reach/active_level_id",
                json!("unsupported.level"),
            ),
            (
                "/selections/0/requested_reach/dimensions/0",
                json!("unsupported.dimension"),
            ),
        ]
        .into_iter()
        .map(|(pointer, value)| (pointer.to_owned(), value))
        .collect::<Vec<_>>();
        for field in [
            "configuration_fingerprint",
            "capability_fingerprint",
            "operating_ceiling_fingerprint",
            "host_prerequisites_fingerprint",
        ] {
            cases.push((format!("/selections/0/{field}"), json!("not-a-fingerprint")));
        }
        for (pointer, value) in cases {
            let mut plan = original_plan.clone();
            *plan.pointer_mut(&pointer).unwrap() = value;
            let mut state = original.clone();
            state.plan = serde_json::from_value(plan.clone()).unwrap();
            // Even a recomputed wrapper and raw Plan hash cannot make an
            // unsupported Plan valid. No original owner bytes are rewritten.
            state.purpose.plan_fingerprint = canonical_fingerprint(&json!({
                "domain":"handbook.local-evaluation-plan-identity@1.0.0",
                "content":plan,
            }))
            .unwrap()
            .as_str()
            .into();
            state.purpose.content_fingerprint =
                state.purpose.fingerprint_semantic_content().unwrap();
            state.identity = state_identity(&state).unwrap();
            let error = validate_eval_state(&state).unwrap_err();
            assert_eq!(
                error.kind(),
                ContractOwnerRepositoryErrorKindV1::RecoveryAmbiguous,
                "{pointer}"
            );
        }
        let mut inconsistent = original.clone();
        inconsistent.purpose.plan_fingerprint = FingerprintV1::from_bytes(b"another complete Plan")
            .as_str()
            .into();
        inconsistent.purpose.content_fingerprint =
            inconsistent.purpose.fingerprint_semantic_content().unwrap();
        inconsistent.identity = state_identity(&inconsistent).unwrap();
        assert_eq!(
            validate_eval_state(&inconsistent).unwrap_err().kind(),
            ContractOwnerRepositoryErrorKindV1::RecoveryAmbiguous
        );
        assert_eq!(std::fs::read(path).unwrap(), bytes);
    }

    #[test]
    fn locked_empty_native_generation_supplies_exact_retention_fingerprint() {
        let directory = tempfile::tempdir().unwrap();
        let root = RetainedRepositoryRootV1::open_exact(directory.path()).unwrap();
        root.create_directories(".handbook/state/locks").unwrap();
        root.write_new_durable(LOCK_RELATIVE_PATH, &[]).unwrap();
        let lock = root
            .acquire_existing_exclusive_lock_v1(LOCK_RELATIVE_PATH)
            .unwrap();
        let generation = handbook_engine::local_authority::LocalBindingV1 {
            reference: LOCK_RELATIVE_PATH.into(),
            fingerprint: lock.physical_fingerprint_v1().unwrap(),
        };
        #[cfg(windows)]
        assert!(
            root.read_regular_bounded(LOCK_RELATIVE_PATH, 16 * 1024 * 1024)
                .is_err(),
            "another handle cannot read through the native exclusive lock"
        );
        assert_eq!(
            root.existing_lock_physical_fingerprint_v1(LOCK_RELATIVE_PATH)
                .unwrap(),
            generation.fingerprint
        );
        assert_eq!(
            retained_evaluation_dependency_fingerprint(
                &root,
                LOCK_RELATIVE_PATH,
                std::slice::from_ref(&generation)
            )
            .unwrap(),
            Some(FingerprintV1::from_bytes(&[]))
        );
        assert!(
            retained_evaluation_dependency_fingerprint(&root, LOCK_RELATIVE_PATH, &[]).is_err(),
            "empty bytes alone do not grant an original native generation"
        );
        lock.release().unwrap();
        std::fs::write(
            directory.path().join(LOCK_RELATIVE_PATH),
            b"not an empty native lock",
        )
        .unwrap();
        assert!(retained_evaluation_dependency_fingerprint(
            &root,
            LOCK_RELATIVE_PATH,
            std::slice::from_ref(&generation)
        )
        .is_err());
        std::fs::remove_file(directory.path().join(LOCK_RELATIVE_PATH)).unwrap();
        root.write_new_durable(LOCK_RELATIVE_PATH, &[]).unwrap();
        assert!(
            retained_evaluation_dependency_fingerprint(
                &root,
                LOCK_RELATIVE_PATH,
                std::slice::from_ref(&generation)
            )
            .is_err(),
            "a replacement empty lock cannot become the original generation"
        );
        std::fs::remove_file(directory.path().join(LOCK_RELATIVE_PATH)).unwrap();
        assert!(retained_evaluation_dependency_fingerprint(
            &root,
            LOCK_RELATIVE_PATH,
            &[generation]
        )
        .is_err());
        assert!(!directory.path().join(LOCK_RELATIVE_PATH).exists());
        root.write_new_durable("record.json", b"original retained record")
            .unwrap();
        assert_eq!(
            retained_evaluation_dependency_fingerprint(&root, "record.json", &[]).unwrap(),
            Some(FingerprintV1::from_bytes(b"original retained record"))
        );
        assert_eq!(
            retained_evaluation_dependency_fingerprint(&root, "absent.json", &[]).unwrap(),
            None
        );
    }

    fn binding(label: &str) -> ContentAddressedBindingV1 {
        identified("handbook.test-population@1.0.0", &label).unwrap()
    }
    fn terminal(
        operation: &str,
        candidate: &ContentAddressedBindingV1,
        disposition: &str,
        evidence: Option<&ContentAddressedBindingV1>,
    ) -> LocalEvaluationAdmissionTerminalV3 {
        LocalEvaluationAdmissionTerminalV3 {
            operation_id: operation.into(),
            candidate: candidate.clone(),
            disposition: disposition.into(),
            reason: "exact owner terminal reason".into(),
            evidence: evidence.cloned(),
            semantic_time: SemanticTimeV1::parse("2026-10-06T00:00:00Z").unwrap(),
        }
    }
    #[test]
    fn unfavorable_member_cannot_be_omitted_from_complete_population() {
        let candidate = binding("unfavorable native candidate");
        let evidence = binding("unfavorable canonical evidence");
        let origin = json!({"kind":"completed","candidates":[candidate]});
        let terminals = [terminal(
            "append-original",
            &candidate,
            "admitted",
            Some(&evidence),
        )];
        assert!(validate_closed_population(&origin, &terminals, &[]).is_err());
        assert!(validate_closed_population(&origin, &terminals, &[(candidate, evidence)]).is_ok());
    }
    #[test]
    fn missing_extra_and_open_admission_operations_refuse_closure() {
        let candidate = binding("native candidate");
        let origin = json!({"kind":"completed","candidates":[candidate]});
        assert!(validate_closed_population(&origin, &[], &[]).is_err());
        let extra = terminal(
            "extra-operation",
            &binding("outside frontier"),
            "refused",
            None,
        );
        assert!(validate_closed_population(&origin, &[extra], &[]).is_err());
        let open = terminal("open-operation", &candidate, "pending", None);
        assert!(validate_closed_population(&origin, &[open], &[]).is_err());
    }
    #[test]
    fn exact_duplicate_delivery_is_one_member_and_refusals_are_terminal_nonmembers() {
        let candidate = binding("native candidate");
        let evidence = binding("canonical evidence");
        let origin = json!({"kind":"completed","candidates":[candidate]});
        let original = terminal(
            "original-operation",
            &candidate,
            "admitted",
            Some(&evidence),
        );
        let duplicate = terminal(
            "duplicate-operation",
            &candidate,
            "duplicate",
            Some(&evidence),
        );
        assert!(validate_closed_population(
            &origin,
            &[original.clone(), duplicate.clone()],
            &[(candidate.clone(), evidence.clone())]
        )
        .is_ok());
        assert!(validate_closed_population(
            &origin,
            &[duplicate],
            &[(candidate.clone(), evidence)]
        )
        .is_err());
        let refused = terminal("refused-operation", &candidate, "refused", None);
        assert!(validate_closed_population(&origin, &[refused], &[]).is_ok());
        assert!(validate_closed_population(&origin, &[original.clone(), original], &[]).is_err());
    }
    #[test]
    fn uncertain_open_and_second_native_population_never_yield_empty_truth() {
        for kind in ["pending", "cleanup_uncertain", "completed"] {
            assert!(
                validate_closed_population(&json!({"kind":kind,"candidates":[]}), &[], &[])
                    .is_err()
            );
        }
        for kind in [
            "timed_out",
            "cancelled_before_launch",
            "refused_before_launch",
            "failed",
        ] {
            assert!(
                validate_closed_population(&json!({"kind":kind,"candidates":[]}), &[], &[]).is_ok()
            );
        }
        assert!(validate_closed_population(
            &json!({"kind":"completed","candidates":[binding("first"),binding("second")]}),
            &[],
            &[]
        )
        .is_err());
    }
    #[test]
    fn same_reference_with_conflicting_bytes_cannot_replace_original_member() {
        let candidate = binding("native candidate");
        let conflicting = ContentAddressedBindingV1::new(
            candidate.reference(),
            FingerprintV1::from_bytes(b"changed candidate bytes"),
        )
        .unwrap();
        let evidence = binding("canonical evidence");
        let origin = json!({"kind":"completed","candidates":[candidate]});
        let terminal = terminal(
            "original-operation",
            &candidate,
            "admitted",
            Some(&evidence),
        );
        assert!(
            validate_closed_population(&origin, &[terminal], &[(conflicting, evidence)]).is_err()
        );
    }
}
