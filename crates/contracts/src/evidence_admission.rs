use crate::evaluation_identity::{
    AdmittedEvidenceWitnessV1, EvidenceSourceAuthorityObservationV1,
    SourceCurrentnessObservationV1 as ContractSourceCurrentnessObservationV1,
};
use crate::{
    evidence_admission_policy_binding_v1, ContentAddressedBindingV1, ContractEvaluationRunV1,
    ContractOwnerRepositoryV1, EvaluationPolicyKindV1, EvaluationPolicyRegistryV1,
    EvidenceCandidateSourceErrorKindV1, EvidenceCandidateSourceV1, EvidenceCaseSelectionV1,
    EvidenceFreshnessPolicyV1, EvidenceRecordV1, ExactBindingV1, ExactRefV1, FingerprintV1,
    ResolutionEnvelopeV1, SemanticTimeV1, StableIdV1,
};
use handbook_engine::{
    effective_resolution_minimum_v1, resolution_meets_minimum_v1, ContextResolutionStackDefinition,
    EvidenceSourceAdmissionErrorKindV1, EvidenceSourceAdmissionV1, ResolutionObservationV1,
    SourceCurrentnessObservationV1, SourceCurrentnessPortV1, SourceCurrentnessRequestV1,
};
use std::error::Error;
use std::fmt;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum EvidenceCandidateAdmissionErrorKindV1 {
    InvalidCandidate,
    CandidateMissing,
    CandidateMalformed,
    CandidateOverLimit,
    CandidateSubstituted,
    SourceAuthorityRefused,
    SourceUnavailable,
    StaleLifecycle,
    StaleSource,
    PolicySubstituted,
    InsufficientResolution,
    ClockIndeterminate,
    InternalInvariant,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EvidenceCandidateAdmissionErrorV1 {
    kind: EvidenceCandidateAdmissionErrorKindV1,
    detail: &'static str,
}

impl EvidenceCandidateAdmissionErrorV1 {
    pub fn kind(&self) -> EvidenceCandidateAdmissionErrorKindV1 {
        self.kind
    }

    pub fn detail(&self) -> &str {
        self.detail
    }
}

impl fmt::Display for EvidenceCandidateAdmissionErrorV1 {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.detail)
    }
}

impl Error for EvidenceCandidateAdmissionErrorV1 {}

/// Resolves and independently admits one exact committed candidate into an
/// owner-native canonical evidence record. The caller contributes only the
/// candidate binding; no caller-supplied body or credential is trusted.
pub fn admit_evidence_candidate_v1<S, C>(
    owner: &ContractOwnerRepositoryV1,
    source: &mut S,
    source_currentness: &mut C,
    resolution_stack: &ContextResolutionStackDefinition,
    policy_registry: &EvaluationPolicyRegistryV1,
    candidate_binding: &ContentAddressedBindingV1,
) -> Result<EvidenceRecordV1, EvidenceCandidateAdmissionErrorV1>
where
    S: EvidenceCandidateSourceV1,
    C: SourceCurrentnessPortV1,
{
    Ok(prepare_admitted_evidence_candidate_v1(
        owner,
        source,
        source_currentness,
        resolution_stack,
        policy_registry,
        candidate_binding,
    )?
    .record)
}

pub(crate) struct PreparedEvidenceAppendV1 {
    record: EvidenceRecordV1,
    run: ContractEvaluationRunV1,
    candidate_jcs: Vec<u8>,
    evaluation_run_jcs: Vec<u8>,
    producer: ContentAddressedBindingV1,
    request_record: ContentAddressedBindingV1,
    execution_record: ContentAddressedBindingV1,
    producer_resolution_ceiling: Option<ResolutionEnvelopeV1>,
    observation: crate::evaluation_identity::EvidenceCandidateBodyV1,
    initial_sources: Vec<AdmittedSourceCurrentness>,
    admission_time: SemanticTimeV1,
    authority: EvidenceAuthorityCheckpointV1,
}

impl PreparedEvidenceAppendV1 {
    pub(crate) fn record(&self) -> &EvidenceRecordV1 {
        &self.record
    }

    pub(crate) fn run(&self) -> &ContractEvaluationRunV1 {
        &self.run
    }

    pub(crate) fn into_record(self) -> EvidenceRecordV1 {
        self.record
    }

    pub(crate) fn recheck_source_currentness<C: SourceCurrentnessPortV1>(
        &self,
        port: &mut C,
    ) -> Result<(), EvidenceCandidateAdmissionErrorV1> {
        let final_sources = observe_sources(&self.observation, port, &self.admission_time)?;
        if !same_source_heads(&self.initial_sources, &final_sources) {
            return Err(refusal(
                EvidenceCandidateAdmissionErrorKindV1::StaleSource,
                "source registry head changed before evidence commit",
            ));
        }
        Ok(())
    }

    pub(crate) fn validate_final_resolved(
        &self,
        resolved: &crate::ResolvedEvidenceCandidateObservationV1<'_>,
    ) -> Result<(), EvidenceCandidateAdmissionErrorV1> {
        if resolved.candidate_jcs() != self.candidate_jcs
            || resolved.evaluation_run_jcs() != self.evaluation_run_jcs
            || resolved.producer() != &self.producer
            || resolved.request_record() != &self.request_record
            || resolved.execution_record() != &self.execution_record
            || resolved.producer_resolution_ceiling() != self.producer_resolution_ceiling.as_ref()
            || !self.authority.matches(resolved.admission())
        {
            return Err(substituted(
                "final candidate provenance or evidence-source authority changed",
            ));
        }
        Ok(())
    }
}

#[derive(Clone)]
struct EvidenceAuthorityCheckpointV1 {
    actor_ref: String,
    authority_ref: String,
    authority_fingerprint: String,
    registry_state_ref: String,
    registry_state_fingerprint: String,
    registry_head_ref: String,
    registry_head_fingerprint: String,
    repository_identity_fingerprint: String,
}

impl EvidenceAuthorityCheckpointV1 {
    fn from_admission(admission: &EvidenceSourceAdmissionV1<'_>) -> Self {
        Self {
            actor_ref: admission.actor_ref().to_owned(),
            authority_ref: admission.authority_ref().to_owned(),
            authority_fingerprint: admission.authority_fingerprint().to_owned(),
            registry_state_ref: admission.registry_state_ref().to_owned(),
            registry_state_fingerprint: admission.registry_state_fingerprint().to_owned(),
            registry_head_ref: admission.registry_head_ref().to_owned(),
            registry_head_fingerprint: admission.registry_head_fingerprint().to_owned(),
            repository_identity_fingerprint: admission.repository_identity_fingerprint().to_owned(),
        }
    }

    fn matches(&self, admission: &EvidenceSourceAdmissionV1<'_>) -> bool {
        self.actor_ref == admission.actor_ref()
            && self.authority_ref == admission.authority_ref()
            && self.authority_fingerprint == admission.authority_fingerprint()
            && self.registry_state_ref == admission.registry_state_ref()
            && self.registry_state_fingerprint == admission.registry_state_fingerprint()
            && self.registry_head_ref == admission.registry_head_ref()
            && self.registry_head_fingerprint == admission.registry_head_fingerprint()
            && self.repository_identity_fingerprint == admission.repository_identity_fingerprint()
    }
}

pub(crate) fn prepare_admitted_evidence_candidate_v1<S, C>(
    owner: &ContractOwnerRepositoryV1,
    source: &mut S,
    source_currentness: &mut C,
    resolution_stack: &ContextResolutionStackDefinition,
    policy_registry: &EvaluationPolicyRegistryV1,
    candidate_binding: &ContentAddressedBindingV1,
) -> Result<PreparedEvidenceAppendV1, EvidenceCandidateAdmissionErrorV1>
where
    S: EvidenceCandidateSourceV1,
    C: SourceCurrentnessPortV1,
{
    source
        .resolve_exact(candidate_binding, |resolved| {
            resolved
                .admission()
                .require_repository_root(owner.retained_repository_root())
                .map_err(map_engine_error)?;
            let candidate: crate::EvidenceCandidateV1 =
                parse_exact(resolved.candidate_jcs(), "candidate identity is malformed")?;
            if candidate.candidate_ref() != candidate_binding.reference()
                || candidate.candidate_fingerprint() != candidate_binding.fingerprint()
            {
                return Err(substituted("resolved candidate identity was substituted"));
            }
            let run: ContractEvaluationRunV1 = parse_exact(
                resolved.evaluation_run_jcs(),
                "evaluation-run identity is malformed",
            )?;
            let body = candidate.admission_body();
            validate_candidate_closure(&candidate, body, &run, &resolved)?;

            let initial_lifecycle = owner
                .readmit_evaluation_run_v1(&run, policy_registry)
                .map_err(|_| {
                    refusal(
                        EvidenceCandidateAdmissionErrorKindV1::StaleLifecycle,
                        "evaluation lifecycle is not exactly current",
                    )
                })?;

            let admission_time = SemanticTimeV1::parse(resolved.admission().semantic_time_utc())
                .map_err(|_| clock("owner semantic time is indeterminate"))?;
            let initial_sources = observe_sources(body, source_currentness, &admission_time)?;
            validate_freshness(body, &run, policy_registry, &admission_time)?;
            let effective_resolution = validate_resolution(
                body,
                &run,
                resolved.producer_resolution_ceiling(),
                resolution_stack,
            )?;

            let final_lifecycle = owner
                .readmit_evaluation_run_v1(&run, policy_registry)
                .map_err(|_| {
                    refusal(
                        EvidenceCandidateAdmissionErrorKindV1::StaleLifecycle,
                        "evaluation lifecycle changed before evidence admission",
                    )
                })?;
            if final_lifecycle.lifecycle_fingerprint() != initial_lifecycle.lifecycle_fingerprint()
            {
                return Err(refusal(
                    EvidenceCandidateAdmissionErrorKindV1::StaleLifecycle,
                    "evaluation lifecycle changed before evidence admission",
                ));
            }
            let final_sources = observe_sources(body, source_currentness, &admission_time)?;
            if !same_source_heads(&initial_sources, &final_sources) {
                return Err(refusal(
                    EvidenceCandidateAdmissionErrorKindV1::StaleSource,
                    "source registry head changed before evidence admission",
                ));
            }

            let authority = authority_observation(resolved.admission(), admission_time.clone())?;
            let admitted_candidate = ContentAddressedBindingV1::new(
                candidate.candidate_ref(),
                candidate.candidate_fingerprint().clone(),
            )
            .map_err(|_| internal("candidate binding could not be retained"))?;
            let owner_currentness = crate::OwnerCurrentnessObservationV1::new(
                admission_time.clone(),
                final_lifecycle.lifecycle_fingerprint().clone(),
            );
            let record = AdmittedEvidenceWitnessV1::new(
                body.clone(),
                effective_resolution,
                owner_currentness,
                initial_sources
                    .iter()
                    .map(|observed| observed.contract.clone())
                    .collect(),
                authority,
                evidence_admission_policy_binding_v1()
                    .map_err(|_| internal("admission policy is unavailable"))?,
                run.freshness_policies().to_vec(),
                run.stability_policies().to_vec(),
                admitted_candidate,
            )
            .into_record()
            .map_err(|_| internal("admitted evidence witness was invalid"))?;
            Ok(PreparedEvidenceAppendV1 {
                record,
                run,
                candidate_jcs: resolved.candidate_jcs().to_vec(),
                evaluation_run_jcs: resolved.evaluation_run_jcs().to_vec(),
                producer: resolved.producer().clone(),
                request_record: resolved.request_record().clone(),
                execution_record: resolved.execution_record().clone(),
                producer_resolution_ceiling: resolved.producer_resolution_ceiling().cloned(),
                observation: body.clone(),
                initial_sources,
                admission_time,
                authority: EvidenceAuthorityCheckpointV1::from_admission(resolved.admission()),
            })
        })
        .map_err(map_source_error)?
}

fn parse_exact<T>(
    bytes: &[u8],
    detail: &'static str,
) -> Result<T, EvidenceCandidateAdmissionErrorV1>
where
    T: serde::de::DeserializeOwned + serde::Serialize,
{
    let value: serde_json::Value = serde_json::from_slice(bytes).map_err(|_| {
        refusal(
            EvidenceCandidateAdmissionErrorKindV1::CandidateMalformed,
            detail,
        )
    })?;
    if serde_json_canonicalizer::to_vec(&value).map_err(|_| {
        refusal(
            EvidenceCandidateAdmissionErrorKindV1::CandidateMalformed,
            detail,
        )
    })? != bytes
    {
        return Err(refusal(
            EvidenceCandidateAdmissionErrorKindV1::CandidateMalformed,
            detail,
        ));
    }
    serde_json::from_value(value).map_err(|_| {
        refusal(
            EvidenceCandidateAdmissionErrorKindV1::CandidateMalformed,
            detail,
        )
    })
}

fn validate_candidate_closure(
    candidate: &crate::EvidenceCandidateV1,
    body: &crate::evaluation_identity::EvidenceCandidateBodyV1,
    run: &ContractEvaluationRunV1,
    resolved: &crate::ResolvedEvidenceCandidateObservationV1<'_>,
) -> Result<(), EvidenceCandidateAdmissionErrorV1> {
    let run_binding = ContentAddressedBindingV1::new(
        run.evaluation_run_ref(),
        run.evaluation_run_fingerprint().clone(),
    )
    .map_err(|_| internal("evaluation-run binding could not be rederived"))?;
    if body.evaluation_run() != &run_binding
        || body.evaluation_lifecycle_basis() != run.evaluation_lifecycle_basis()
        || body.contract() != &run.contract()
        || body.subject() != run.subject()
        || !run.cases().contains(body.case())
        || body.request_resolution() != run.requested_resolution()
        || body.producer() != resolved.producer()
        || body.request_record() != resolved.request_record()
        || body.execution_record() != resolved.execution_record()
        || resolved.admission().candidate_ref() != candidate.candidate_ref()
        || resolved.admission().candidate_fingerprint()
            != candidate.candidate_fingerprint().as_str()
        || resolved.admission().producer_ref() != body.producer().reference()
        || resolved.admission().producer_fingerprint() != body.producer().fingerprint().as_str()
        || resolved.admission().request_ref() != body.request_record().reference()
        || resolved.admission().request_fingerprint()
            != body.request_record().fingerprint().as_str()
        || resolved.admission().execution_ref() != body.execution_record().reference()
        || resolved.admission().execution_fingerprint()
            != body.execution_record().fingerprint().as_str()
    {
        return Err(substituted(
            "candidate closure or provenance was substituted",
        ));
    }

    let mut run_claims = run
        .claims()
        .iter()
        .map(|claim| claim.claim_id().clone())
        .collect::<Vec<_>>();
    let mut partition = body.claim_partition().observed().to_vec();
    partition.extend_from_slice(body.claim_partition().unobserved());
    partition.extend(body.claim_partition().excluded_ids().cloned());
    run_claims.sort();
    partition.sort();
    if body.observed_claim_ids() != body.claim_partition().observed()
        || partition != run_claims
        || partition.windows(2).any(|window| window[0] == window[1])
    {
        return Err(substituted(
            "candidate claim partition is not the exact run closure",
        ));
    }
    for claim_id in body.observed_claim_ids() {
        let claim = run
            .claims()
            .iter()
            .find(|claim| claim.claim_id() == claim_id)
            .ok_or_else(|| substituted("candidate observed an unknown claim"))?;
        let matching = claim.evidence_requirements().iter().any(|requirement| {
            requirement.evidence_kind() == body.evidence_kind()
                && match requirement.cases() {
                    EvidenceCaseSelectionV1::ClaimCaseSelector => true,
                    EvidenceCaseSelectionV1::ExactCases { case_ids } => {
                        case_ids.contains(body.case().case_id())
                    }
                }
        });
        if !matching {
            return Err(refusal(
                EvidenceCandidateAdmissionErrorKindV1::InvalidCandidate,
                "candidate evidence kind or case is outside the claim closure",
            ));
        }
    }
    Ok(())
}

#[derive(Clone)]
struct AdmittedSourceCurrentness {
    engine: SourceCurrentnessObservationV1,
    contract: ContractSourceCurrentnessObservationV1,
}

fn observe_sources<C: SourceCurrentnessPortV1>(
    body: &crate::evaluation_identity::EvidenceCandidateBodyV1,
    port: &mut C,
    admission_time: &SemanticTimeV1,
) -> Result<Vec<AdmittedSourceCurrentness>, EvidenceCandidateAdmissionErrorV1> {
    body.sources()
        .iter()
        .map(|source| {
            let request = SourceCurrentnessRequestV1::new(
                source.source().reference(),
                source.source().fingerprint().as_str(),
                source.revision(),
            )
            .map_err(map_engine_error)?;
            let observed = port.observe_current(&request).map_err(map_engine_error)?;
            let observed_at = SemanticTimeV1::parse(observed.observed_at_utc())
                .map_err(|_| clock("source currentness time is indeterminate"))?;
            require_not_future(&observed_at, admission_time)?;
            let current_head = ContentAddressedBindingV1::new(
                observed.current_head_ref(),
                FingerprintV1::parse(observed.current_head_fingerprint())
                    .map_err(|_| substituted("source current head fingerprint is malformed"))?,
            )
            .map_err(|_| substituted("source current head binding is malformed"))?;
            let contract = ContractSourceCurrentnessObservationV1::new(
                source.source().clone(),
                source.revision().to_owned(),
                source.collected_at().clone(),
                observed_at,
                current_head,
            );
            Ok(AdmittedSourceCurrentness {
                engine: observed,
                contract,
            })
        })
        .collect()
}

fn same_source_heads(
    initial: &[AdmittedSourceCurrentness],
    final_observations: &[AdmittedSourceCurrentness],
) -> bool {
    initial.len() == final_observations.len()
        && initial.iter().zip(final_observations).all(|(left, right)| {
            left.engine.source_ref() == right.engine.source_ref()
                && left.engine.source_fingerprint() == right.engine.source_fingerprint()
                && left.engine.revision() == right.engine.revision()
                && left.engine.current_head_ref() == right.engine.current_head_ref()
                && left.engine.current_head_fingerprint() == right.engine.current_head_fingerprint()
        })
}

fn validate_freshness(
    body: &crate::evaluation_identity::EvidenceCandidateBodyV1,
    run: &ContractEvaluationRunV1,
    registry: &EvaluationPolicyRegistryV1,
    admission_time: &SemanticTimeV1,
) -> Result<(), EvidenceCandidateAdmissionErrorV1> {
    require_not_future(body.observed_at(), admission_time)?;
    for source in body.sources() {
        require_not_future(source.collected_at(), admission_time)?;
        if source.collected_at() > body.observed_at() {
            return Err(clock(
                "source collection time follows candidate observation",
            ));
        }
    }
    let age = semantic_seconds(admission_time)?
        .checked_sub(semantic_seconds(body.observed_at())?)
        .ok_or_else(|| clock("freshness interval overflowed"))?;
    for claim_id in body.observed_claim_ids() {
        let claim = run
            .claims()
            .iter()
            .find(|claim| claim.claim_id() == claim_id)
            .ok_or_else(|| substituted("freshness claim is absent"))?;
        let registered = registry
            .resolve_exact(claim.freshness_policy())
            .map_err(|_| policy("freshness policy is not exactly registered"))?;
        if registered.kind() != EvaluationPolicyKindV1::Freshness {
            return Err(policy("freshness policy kind was substituted"));
        }
        let policy: EvidenceFreshnessPolicyV1 =
            serde_json::from_slice(&registered.canonical_json())
                .map_err(|_| policy("freshness policy bytes are malformed"))?;
        if !policy.require_exact_source_revision()
            || policy.future_skew_seconds() != 0
            || policy.max_observation_age_seconds() == 0
            || age > i128::from(policy.max_observation_age_seconds())
        {
            return Err(refusal(
                EvidenceCandidateAdmissionErrorKindV1::StaleSource,
                "candidate is not fresh under the exact registered policy",
            ));
        }
    }
    Ok(())
}

fn validate_resolution(
    body: &crate::evaluation_identity::EvidenceCandidateBodyV1,
    run: &ContractEvaluationRunV1,
    producer_ceiling: Option<&ResolutionEnvelopeV1>,
    stack: &ContextResolutionStackDefinition,
) -> Result<ResolutionEnvelopeV1, EvidenceCandidateAdmissionErrorV1> {
    let mut observations = vec![
        resolution_observation(body.request_resolution())?,
        resolution_observation(body.actual_resolution())?,
    ];
    if let Some(ceiling) = producer_ceiling {
        observations.push(resolution_observation(ceiling)?);
    }
    let effective =
        effective_resolution_minimum_v1(stack, &observations).map_err(map_engine_error)?;
    for claim_id in body.observed_claim_ids() {
        let minimum = run
            .claims()
            .iter()
            .find(|claim| claim.claim_id() == claim_id)
            .ok_or_else(|| substituted("Resolution claim is absent"))?
            .minimum_resolution();
        if minimum.resolution_stack() != body.request_resolution().resolution_stack() {
            return Err(substituted("minimum Resolution stack was substituted"));
        }
        let minimum_observation = ResolutionObservationV1::new(
            minimum.resolution_stack().reference().as_str(),
            minimum.resolution_stack().fingerprint().as_str(),
            engine_resolution_values(minimum.dimension_values())?
                .each_ref()
                .map(String::as_str),
        )
        .map_err(map_engine_error)?;
        if !resolution_meets_minimum_v1(stack, &effective, &minimum_observation)
            .map_err(map_engine_error)?
        {
            return Err(refusal(
                EvidenceCandidateAdmissionErrorKindV1::InsufficientResolution,
                "effective Resolution is below a claim minimum",
            ));
        }
    }
    let values = effective.values();
    let contract_values = contract_resolution_values(values)?;
    Ok(ResolutionEnvelopeV1::new(
        body.request_resolution().resolution_stack().clone(),
        contract_values[0].clone(),
        contract_values[1].clone(),
        contract_values[2].clone(),
        contract_values[3].clone(),
        contract_values[4].clone(),
        contract_values[5].clone(),
    ))
}

fn resolution_observation(
    envelope: &ResolutionEnvelopeV1,
) -> Result<ResolutionObservationV1, EvidenceCandidateAdmissionErrorV1> {
    ResolutionObservationV1::new(
        envelope.resolution_stack().reference().as_str(),
        envelope.resolution_stack().fingerprint().as_str(),
        engine_resolution_values(envelope.dimension_values())?
            .each_ref()
            .map(String::as_str),
    )
    .map_err(map_engine_error)
}

fn engine_resolution_values(
    values: [&StableIdV1; 6],
) -> Result<[String; 6], EvidenceCandidateAdmissionErrorV1> {
    const PREFIXES: [&str; 6] = [
        "resolution.scope.",
        "resolution.detail.",
        "resolution.time.",
        "resolution.authority.",
        "resolution.memory.",
        "resolution.validation.",
    ];
    let mut normalized = std::array::from_fn(|_| String::new());
    for index in 0..6 {
        let suffix = values[index]
            .as_str()
            .strip_prefix(PREFIXES[index])
            .ok_or_else(|| substituted("Resolution dimension namespace was substituted"))?;
        normalized[index] = suffix.replace('-', "_");
    }
    Ok(normalized)
}

fn contract_resolution_values(
    values: [&str; 6],
) -> Result<[StableIdV1; 6], EvidenceCandidateAdmissionErrorV1> {
    const PREFIXES: [&str; 6] = [
        "resolution.scope.",
        "resolution.detail.",
        "resolution.time.",
        "resolution.authority.",
        "resolution.memory.",
        "resolution.validation.",
    ];
    let mut normalized = Vec::with_capacity(6);
    for index in 0..6 {
        normalized.push(
            StableIdV1::parse(&format!(
                "{}{}",
                PREFIXES[index],
                values[index].replace('_', "-")
            ))
            .map_err(|_| internal("effective Resolution is invalid"))?,
        );
    }
    normalized
        .try_into()
        .map_err(|_| internal("effective Resolution is incomplete"))
}

fn authority_observation(
    admission: &EvidenceSourceAdmissionV1<'_>,
    observed_at: SemanticTimeV1,
) -> Result<EvidenceSourceAuthorityObservationV1, EvidenceCandidateAdmissionErrorV1> {
    Ok(EvidenceSourceAuthorityObservationV1::new(
        StableIdV1::parse(admission.actor_ref())
            .map_err(|_| substituted("authenticated actor reference is malformed"))?,
        ExactBindingV1::new(
            ExactRefV1::parse(admission.authority_ref())
                .map_err(|_| substituted("evidence-source authority reference is malformed"))?,
            FingerprintV1::parse(admission.authority_fingerprint())
                .map_err(|_| substituted("evidence-source authority fingerprint is malformed"))?,
        ),
        ContentAddressedBindingV1::new(
            admission.registry_state_ref(),
            FingerprintV1::parse(admission.registry_state_fingerprint())
                .map_err(|_| substituted("authority registry state is malformed"))?,
        )
        .map_err(|_| substituted("authority registry state is malformed"))?,
        ContentAddressedBindingV1::new(
            admission.registry_head_ref(),
            FingerprintV1::parse(admission.registry_head_fingerprint())
                .map_err(|_| substituted("authority registry head is malformed"))?,
        )
        .map_err(|_| substituted("authority registry head is malformed"))?,
        observed_at,
    ))
}

fn require_not_future(
    observed: &SemanticTimeV1,
    owner_time: &SemanticTimeV1,
) -> Result<(), EvidenceCandidateAdmissionErrorV1> {
    if semantic_seconds(observed)? > semantic_seconds(owner_time)? {
        return Err(clock("future observations are refused"));
    }
    Ok(())
}

fn semantic_seconds(time: &SemanticTimeV1) -> Result<i128, EvidenceCandidateAdmissionErrorV1> {
    let value = time.as_str().as_bytes();
    let number = |start: usize, end: usize| -> Result<i128, EvidenceCandidateAdmissionErrorV1> {
        std::str::from_utf8(&value[start..end])
            .ok()
            .and_then(|value| value.parse::<i128>().ok())
            .ok_or_else(|| clock("semantic time arithmetic is indeterminate"))
    };
    let mut year = number(0, 4)?;
    let month = number(5, 7)?;
    let day = number(8, 10)?;
    let hour = number(11, 13)?;
    let minute = number(14, 16)?;
    let second = number(17, 19)?;
    year -= i128::from(month <= 2);
    let era = if year >= 0 { year } else { year - 399 } / 400;
    let year_of_era = year - era * 400;
    let shifted_month = month + if month > 2 { -3 } else { 9 };
    let day_of_year = (153 * shifted_month + 2) / 5 + day - 1;
    let day_of_era = year_of_era * 365 + year_of_era / 4 - year_of_era / 100 + day_of_year;
    let days = era
        .checked_mul(146_097)
        .and_then(|value| value.checked_add(day_of_era))
        .and_then(|value| value.checked_sub(719_468))
        .ok_or_else(|| clock("semantic time arithmetic overflowed"))?;
    days.checked_mul(86_400)
        .and_then(|value| value.checked_add(hour * 3_600 + minute * 60 + second))
        .ok_or_else(|| clock("semantic time arithmetic overflowed"))
}

fn map_source_error(
    error: crate::EvidenceCandidateSourceErrorV1,
) -> EvidenceCandidateAdmissionErrorV1 {
    let kind = match error.kind() {
        EvidenceCandidateSourceErrorKindV1::Missing => {
            EvidenceCandidateAdmissionErrorKindV1::CandidateMissing
        }
        EvidenceCandidateSourceErrorKindV1::Malformed => {
            EvidenceCandidateAdmissionErrorKindV1::CandidateMalformed
        }
        EvidenceCandidateSourceErrorKindV1::OverLimit => {
            EvidenceCandidateAdmissionErrorKindV1::CandidateOverLimit
        }
        EvidenceCandidateSourceErrorKindV1::CandidateSubstituted
        | EvidenceCandidateSourceErrorKindV1::InvalidRequest => {
            EvidenceCandidateAdmissionErrorKindV1::CandidateSubstituted
        }
        EvidenceCandidateSourceErrorKindV1::AuthorityAbsent
        | EvidenceCandidateSourceErrorKindV1::AuthorityAmbiguous
        | EvidenceCandidateSourceErrorKindV1::StaleAuthority
        | EvidenceCandidateSourceErrorKindV1::AuthenticationRefused => {
            EvidenceCandidateAdmissionErrorKindV1::SourceAuthorityRefused
        }
        EvidenceCandidateSourceErrorKindV1::Unavailable => {
            EvidenceCandidateAdmissionErrorKindV1::SourceUnavailable
        }
    };
    refusal(kind, "evidence candidate source refused admission")
}

fn map_engine_error(
    error: handbook_engine::EvidenceSourceAdmissionErrorV1,
) -> EvidenceCandidateAdmissionErrorV1 {
    let kind = match error.kind() {
        EvidenceSourceAdmissionErrorKindV1::ResolutionRefused => {
            EvidenceCandidateAdmissionErrorKindV1::InsufficientResolution
        }
        EvidenceSourceAdmissionErrorKindV1::AuthorityAbsent
        | EvidenceSourceAdmissionErrorKindV1::AuthorityAmbiguous
        | EvidenceSourceAdmissionErrorKindV1::StaleAuthority
        | EvidenceSourceAdmissionErrorKindV1::AuthenticationRefused => {
            EvidenceCandidateAdmissionErrorKindV1::SourceAuthorityRefused
        }
        EvidenceSourceAdmissionErrorKindV1::InvalidRequest => {
            EvidenceCandidateAdmissionErrorKindV1::CandidateSubstituted
        }
        EvidenceSourceAdmissionErrorKindV1::AuthenticatorUnavailable
        | EvidenceSourceAdmissionErrorKindV1::DurabilityViolation => {
            EvidenceCandidateAdmissionErrorKindV1::SourceUnavailable
        }
    };
    refusal(kind, "generic Engine observation refused admission")
}

const fn refusal(
    kind: EvidenceCandidateAdmissionErrorKindV1,
    detail: &'static str,
) -> EvidenceCandidateAdmissionErrorV1 {
    EvidenceCandidateAdmissionErrorV1 { kind, detail }
}

const fn substituted(detail: &'static str) -> EvidenceCandidateAdmissionErrorV1 {
    refusal(
        EvidenceCandidateAdmissionErrorKindV1::CandidateSubstituted,
        detail,
    )
}

const fn policy(detail: &'static str) -> EvidenceCandidateAdmissionErrorV1 {
    refusal(
        EvidenceCandidateAdmissionErrorKindV1::PolicySubstituted,
        detail,
    )
}

const fn clock(detail: &'static str) -> EvidenceCandidateAdmissionErrorV1 {
    refusal(
        EvidenceCandidateAdmissionErrorKindV1::ClockIndeterminate,
        detail,
    )
}

const fn internal(detail: &'static str) -> EvidenceCandidateAdmissionErrorV1 {
    refusal(
        EvidenceCandidateAdmissionErrorKindV1::InternalInvariant,
        detail,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn future_time_and_resolution_namespace_substitution_refuse_closed() {
        let owner_time = SemanticTimeV1::parse("2026-08-19T12:00:00Z").unwrap();
        let future = SemanticTimeV1::parse("2026-08-19T12:00:01Z").unwrap();
        assert_eq!(
            require_not_future(&future, &owner_time).unwrap_err().kind(),
            EvidenceCandidateAdmissionErrorKindV1::ClockIndeterminate
        );
        assert_eq!(
            semantic_seconds(&SemanticTimeV1::parse("2026-08-20T12:00:00Z").unwrap()).unwrap()
                - semantic_seconds(&owner_time).unwrap(),
            86_400
        );

        let wrong = StableIdV1::parse("resolution.wrong.local-observation").unwrap();
        let valid = StableIdV1::parse("resolution.detail.identifier-only").unwrap();
        let time = StableIdV1::parse("resolution.time.current-operation").unwrap();
        let authority = StableIdV1::parse("resolution.authority.read-only").unwrap();
        let memory = StableIdV1::parse("resolution.memory.operation").unwrap();
        let validation = StableIdV1::parse("resolution.validation.observation-only").unwrap();
        assert_eq!(
            engine_resolution_values([&wrong, &valid, &time, &authority, &memory, &validation,])
                .unwrap_err()
                .kind(),
            EvidenceCandidateAdmissionErrorKindV1::CandidateSubstituted
        );
    }
}
