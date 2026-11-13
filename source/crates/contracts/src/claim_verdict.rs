use crate::evaluation_identity::{
    claim_verdict_from_accounting_v1, EvidenceSourceAuthorityObservationV1,
};
use crate::owner_repository::{EvidenceEvaluationEntryV1, EvidenceEvaluationSnapshotV1};
use crate::{
    evaluate_applicability, evidence_stability_policy_binding_v1, ApplicabilityInputSourceV1,
    ApplicabilityOutcomeV1, ClaimVerdictDecisionV1, ClaimVerdictV1, ContentAddressedBindingV1,
    ContractClaimV1, ContractEvaluationRunV1, ContractOwnerRepositoryErrorKindV1,
    ContractOwnerRepositoryV1, EvaluationPolicyKindV1, EvaluationPolicyRegistryV1,
    EvidenceCandidateSourceErrorKindV1, EvidenceCandidateSourceV1, EvidenceCardinalityV1,
    EvidenceCaseSelectionV1, EvidenceFreshnessPolicyV1, EvidenceObservationV1, FingerprintV1,
    GateEffectV1, MatcherInputCandidateV1, MatcherValueV1, ResolutionEnvelopeV1,
    ResolutionMinimumV1, SemanticTimeV1, StableIdV1,
};
use handbook_engine::{
    resolution_meets_minimum_v1, ContextResolutionStackDefinition, ResolutionObservationV1,
    SourceCurrentnessPortV1, SourceCurrentnessRequestV1,
};
use std::error::Error;
use std::fmt;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ClaimVerdictEvaluationErrorKindV1 {
    RepositoryUnavailable,
    RepositoryCorrupt,
    InternalInvariant,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ClaimVerdictEvaluationErrorV1 {
    kind: ClaimVerdictEvaluationErrorKindV1,
    detail: &'static str,
}

impl ClaimVerdictEvaluationErrorV1 {
    pub fn kind(&self) -> ClaimVerdictEvaluationErrorKindV1 {
        self.kind
    }

    pub fn detail(&self) -> &str {
        self.detail
    }
}

impl fmt::Display for ClaimVerdictEvaluationErrorV1 {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.detail)
    }
}

impl Error for ClaimVerdictEvaluationErrorV1 {}

#[derive(Clone, Debug, PartialEq)]
pub struct ClaimVerdictEvaluationV1 {
    evidence_repository_root: FingerprintV1,
    verdicts: Vec<ClaimVerdictV1>,
}

impl ClaimVerdictEvaluationV1 {
    pub fn evidence_repository_root(&self) -> &FingerprintV1 {
        &self.evidence_repository_root
    }

    pub fn verdicts(&self) -> &[ClaimVerdictV1] {
        &self.verdicts
    }
}

#[derive(Clone)]
struct CurrentRecordV1 {
    sequence: u64,
    record: crate::EvidenceRecordV1,
    binding: ContentAddressedBindingV1,
    authority: Option<EvidenceSourceAuthorityObservationV1>,
    defect: Option<StableIdV1>,
}

#[derive(Clone)]
struct RevalidationV1 {
    records: Vec<CurrentRecordV1>,
    global_reasons: Vec<(StableIdV1, Vec<ContentAddressedBindingV1>)>,
}

/// Recomputes exactly one in-memory verdict for every definition-order claim
/// from one complete immutable evidence snapshot. It writes no result state.
pub fn evaluate_claim_verdicts_v1<S, C>(
    owner: &ContractOwnerRepositoryV1,
    source: &mut S,
    source_currentness: &mut C,
    resolution_stack: &ContextResolutionStackDefinition,
    policy_registry: &EvaluationPolicyRegistryV1,
    run: &ContractEvaluationRunV1,
) -> Result<ClaimVerdictEvaluationV1, ClaimVerdictEvaluationErrorV1>
where
    S: EvidenceCandidateSourceV1,
    C: SourceCurrentnessPortV1,
{
    let initial_lifecycle_ok = owner
        .readmit_evaluation_run_v1(run, policy_registry)
        .is_ok();
    let snapshot = owner
        .evidence_evaluation_snapshot_v1()
        .map_err(map_repository_error)?;
    let run_binding = ContentAddressedBindingV1::new(
        run.evaluation_run_ref(),
        run.evaluation_run_fingerprint().clone(),
    )
    .map_err(|_| internal("evaluation-run binding cannot be rederived"))?;
    let target_entries = snapshot
        .entries
        .iter()
        .filter(|entry| entry.record.evaluation_run() == &run_binding)
        .cloned()
        .collect::<Vec<_>>();

    let mut initial = revalidate_records(source, source_currentness, run, &target_entries);
    if !initial_lifecycle_ok {
        initial
            .global_reasons
            .insert(0, (reason("verdict.blocked.lifecycle-stale")?, Vec::new()));
    }
    normalize_reasons(&mut initial.global_reasons);
    let _preliminary = derive_verdicts(
        run,
        resolution_stack,
        policy_registry,
        &snapshot,
        &initial,
        &run_binding,
    )?;

    let mut final_validation = revalidate_records(source, source_currentness, run, &target_entries);
    let final_lifecycle_ok = owner
        .readmit_evaluation_run_v1(run, policy_registry)
        .is_ok();
    if validation_fingerprint(&initial) != validation_fingerprint(&final_validation) {
        final_validation.global_reasons.push((
            reason("verdict.blocked.currentness-changed")?,
            target_entries
                .iter()
                .map(entry_binding)
                .collect::<Result<Vec<_>, _>>()?,
        ));
    }
    if !initial_lifecycle_ok {
        final_validation
            .global_reasons
            .insert(0, (reason("verdict.blocked.lifecycle-stale")?, Vec::new()));
    }
    if initial_lifecycle_ok != final_lifecycle_ok {
        final_validation.global_reasons.insert(
            usize::from(!initial_lifecycle_ok),
            (reason("verdict.blocked.lifecycle-changed")?, Vec::new()),
        );
    }
    normalize_reasons(&mut final_validation.global_reasons);
    let verdicts = derive_verdicts(
        run,
        resolution_stack,
        policy_registry,
        &snapshot,
        &final_validation,
        &run_binding,
    )?;
    Ok(ClaimVerdictEvaluationV1 {
        evidence_repository_root: snapshot.root,
        verdicts,
    })
}

fn derive_verdicts(
    run: &ContractEvaluationRunV1,
    resolution_stack: &ContextResolutionStackDefinition,
    policy_registry: &EvaluationPolicyRegistryV1,
    snapshot: &EvidenceEvaluationSnapshotV1,
    validation: &RevalidationV1,
    run_binding: &ContentAddressedBindingV1,
) -> Result<Vec<ClaimVerdictV1>, ClaimVerdictEvaluationErrorV1> {
    let verdicts = run
        .claims()
        .iter()
        .map(|claim| {
            evaluate_claim(
                claim,
                run,
                resolution_stack,
                policy_registry,
                snapshot,
                validation,
                run_binding,
            )
        })
        .collect::<Result<Vec<_>, ClaimVerdictEvaluationErrorV1>>()?;
    if verdicts.len() != run.claims().len()
        || verdicts
            .iter()
            .zip(run.claims())
            .any(|(verdict, claim)| verdict.claim_id() != claim.claim_id())
    {
        return Err(internal("claim verdict partition is incomplete"));
    }
    Ok(verdicts)
}

fn revalidate_records<S, C>(
    source: &mut S,
    source_currentness: &mut C,
    run: &ContractEvaluationRunV1,
    entries: &[EvidenceEvaluationEntryV1],
) -> RevalidationV1
where
    S: EvidenceCandidateSourceV1,
    C: SourceCurrentnessPortV1,
{
    let mut records = Vec::with_capacity(entries.len());
    let mut global_reasons = Vec::new();
    for entry in entries {
        let binding = match entry_binding(entry) {
            Ok(binding) => binding,
            Err(_) => {
                global_reasons.push((
                    reason_unchecked("verdict.blocked.evidence-identity"),
                    Vec::new(),
                ));
                continue;
            }
        };
        let checked = source.resolve_exact(entry.record.admitted_candidate(), |resolved| {
            let candidate = parse_exact_candidate(resolved.candidate_jcs())?;
            let resolved_run = parse_exact_run(resolved.evaluation_run_jcs())?;
            let body = entry.record.observation();
            if candidate.candidate_ref() != entry.record.admitted_candidate().reference()
                || candidate.candidate_fingerprint()
                    != entry.record.admitted_candidate().fingerprint()
                || candidate.admission_body() != body
                || &resolved_run != run
                || resolved.producer() != body.producer()
                || resolved.request_record() != body.request_record()
                || resolved.execution_record() != body.execution_record()
            {
                return Err(reason_unchecked("verdict.blocked.evidence-substituted"));
            }
            // Local sources remain guarded while the actual retained source
            // observer runs. A post-callback cache cannot supply this check.
            if resolved.admission().is_local() {
                if !sources_are_current(&entry.record, source_currentness) {
                    return Err(reason_unchecked("verdict.blocked.source-currentness"));
                }
                resolved
                    .validate_current()
                    .map_err(|_| reason_unchecked("verdict.blocked.source-currentness"))?;
            }
            let observed_at = if resolved.admission().is_local() {
                SemanticTimeV1::owner_observation_now_v1()
                    .map_err(|_| reason_unchecked("verdict.blocked.authority-malformed"))?
            } else {
                run.owner_currentness().semantic_time_utc().clone()
            };
            let authority =
                crate::evidence_admission::authority_observation(resolved.admission(), observed_at)
                    .map_err(|_| reason_unchecked("verdict.blocked.authority-malformed"))?;
            if !authority.same_authority_identity(entry.record.evidence_source_authority()) {
                return Err(reason_unchecked("verdict.blocked.evidence-substituted"));
            }
            Ok(authority)
        });

        let (authority, mut defect) = match checked {
            Ok(Ok(authority)) => (Some(authority), None),
            Ok(Err(code)) => (None, Some(code)),
            Err(error) => (None, Some(source_error_reason(error.kind()))),
        };
        if defect.is_none()
            && !entry.record.evidence_source_authority().is_local()
            && !sources_are_current(&entry.record, source_currentness)
        {
            defect = Some(reason_unchecked("verdict.blocked.source-currentness"));
        }
        if let Some(code) = defect.clone() {
            global_reasons.push((code, vec![binding.clone()]));
        }
        records.push(CurrentRecordV1 {
            sequence: entry.sequence,
            record: entry.record.clone(),
            binding,
            authority,
            defect,
        });
    }
    normalize_reasons(&mut global_reasons);
    RevalidationV1 {
        records,
        global_reasons,
    }
}

fn sources_are_current<C: SourceCurrentnessPortV1>(
    record: &crate::EvidenceRecordV1,
    port: &mut C,
) -> bool {
    let sources = record.observation().sources();
    if sources.len() != record.source_currentness().len() {
        return false;
    }
    sources
        .iter()
        .zip(record.source_currentness())
        .all(|(source, admitted)| {
            if admitted.source() != source.source() || admitted.revision() != source.revision() {
                return false;
            }
            let Ok(request) = SourceCurrentnessRequestV1::new(
                source.source().reference(),
                source.source().fingerprint().as_str(),
                source.revision(),
            ) else {
                return false;
            };
            let Ok(observed) = port.observe_current(&request) else {
                return false;
            };
            observed.current_head_ref() == admitted.current_head().reference()
                && observed.current_head_fingerprint()
                    == admitted.current_head().fingerprint().as_str()
        })
}

fn evaluate_claim(
    claim: &ContractClaimV1,
    run: &ContractEvaluationRunV1,
    resolution_stack: &ContextResolutionStackDefinition,
    policy_registry: &EvaluationPolicyRegistryV1,
    snapshot: &EvidenceEvaluationSnapshotV1,
    validation: &RevalidationV1,
    run_binding: &ContentAddressedBindingV1,
) -> Result<ClaimVerdictV1, ClaimVerdictEvaluationErrorV1> {
    let (derived_applicability, matching_cases) = claim_applicability(claim, run)?;
    let mut supporting = Vec::new();
    let mut fresh = Vec::new();
    let mut stale = Vec::new();
    let mut resolution_eligible = Vec::new();
    let mut resolution_insufficient = Vec::new();
    let mut authorities = Vec::new();
    for authority in validation
        .records
        .iter()
        .filter_map(|record| record.authority.clone())
    {
        if !authorities.contains(&authority) {
            authorities.push(authority);
        }
    }
    let mut reasons = validation.global_reasons.clone();
    let applicability = if reasons.is_empty() {
        derived_applicability
    } else {
        ApplicabilityOutcomeV1::Blocked
    };

    let decision = match applicability {
        ApplicabilityOutcomeV1::Blocked => {
            if reasons.is_empty() {
                reasons.push((reason("verdict.blocked.applicability")?, Vec::new()));
            }
            ClaimVerdictDecisionV1::Blocked
        }
        ApplicabilityOutcomeV1::NotApplicable if reasons.is_empty() => {
            reasons.push((reason("verdict.not-applicable.matcher-false")?, Vec::new()));
            ClaimVerdictDecisionV1::NotApplicable
        }
        ApplicabilityOutcomeV1::NotApplicable => ClaimVerdictDecisionV1::Blocked,
        ApplicabilityOutcomeV1::Applicable => {
            let clause_decisions = claim
                .evidence_requirements()
                .iter()
                .map(|requirement| {
                    evaluate_requirement(
                        claim,
                        requirement,
                        &matching_cases,
                        validation,
                        run,
                        resolution_stack,
                        policy_registry,
                        &mut supporting,
                        &mut fresh,
                        &mut stale,
                        &mut resolution_eligible,
                        &mut resolution_insufficient,
                        &mut reasons,
                    )
                })
                .collect::<Result<Vec<_>, ClaimVerdictEvaluationErrorV1>>()?;
            if validation.global_reasons.is_empty() {
                combine_all_of(&clause_decisions)
            } else {
                ClaimVerdictDecisionV1::Blocked
            }
        }
    };

    sort_bindings_by_sequence(&mut supporting, &validation.records);
    sort_bindings_by_sequence(&mut fresh, &validation.records);
    sort_bindings_by_sequence(&mut stale, &validation.records);
    sort_bindings_by_sequence(&mut resolution_eligible, &validation.records);
    sort_bindings_by_sequence(&mut resolution_insufficient, &validation.records);
    let mut disqualifying = if applicability != ApplicabilityOutcomeV1::NotApplicable {
        validation
            .records
            .iter()
            .filter(|record| !supporting.contains(&record.binding))
            .map(|record| record.binding.clone())
            .collect::<Vec<_>>()
    } else {
        Vec::new()
    };
    sort_bindings_by_sequence(&mut disqualifying, &validation.records);
    normalize_reasons(&mut reasons);
    let verdict = claim_verdict_from_accounting_v1(
        run_binding.clone(),
        run.evaluation_lifecycle_basis().clone(),
        run.contract(),
        claim.claim_id().clone(),
        applicability,
        snapshot.root.clone(),
        supporting,
        disqualifying,
        claim.freshness_policy().clone(),
        fresh,
        stale,
        run.requested_resolution().clone(),
        resolution_eligible,
        resolution_insufficient,
        authorities,
        decision,
        reasons,
    )
    .map_err(|_| internal("claim verdict identity cannot be constructed"))?;
    Ok(verdict)
}

#[allow(clippy::too_many_arguments)]
fn evaluate_requirement(
    claim: &ContractClaimV1,
    requirement: &crate::EvidenceRequirementV1,
    matching_cases: &[StableIdV1],
    validation: &RevalidationV1,
    run: &ContractEvaluationRunV1,
    resolution_stack: &ContextResolutionStackDefinition,
    policy_registry: &EvaluationPolicyRegistryV1,
    supporting: &mut Vec<ContentAddressedBindingV1>,
    fresh: &mut Vec<ContentAddressedBindingV1>,
    stale: &mut Vec<ContentAddressedBindingV1>,
    resolution_eligible: &mut Vec<ContentAddressedBindingV1>,
    resolution_insufficient: &mut Vec<ContentAddressedBindingV1>,
    reasons: &mut Vec<(StableIdV1, Vec<ContentAddressedBindingV1>)>,
) -> Result<ClaimVerdictDecisionV1, ClaimVerdictEvaluationErrorV1> {
    if requirement.stability_policy()
        != &evidence_stability_policy_binding_v1()
            .map_err(|_| internal("stability policy is unavailable"))?
        || policy_registry
            .resolve_exact(requirement.stability_policy())
            .is_err()
    {
        reasons.push((reason("verdict.blocked.stability-policy")?, Vec::new()));
        return Ok(ClaimVerdictDecisionV1::Blocked);
    }
    let required_cases = match requirement.cases() {
        EvidenceCaseSelectionV1::ClaimCaseSelector => matching_cases.to_vec(),
        EvidenceCaseSelectionV1::ExactCases { case_ids } => {
            if case_ids
                .iter()
                .any(|case_id| !run.cases().iter().any(|case| case.case_id() == case_id))
            {
                reasons.push((reason("verdict.blocked.unknown-case")?, Vec::new()));
                return Ok(ClaimVerdictDecisionV1::Blocked);
            }
            case_ids.clone()
        }
    };
    if required_cases.is_empty() {
        reasons.push((reason("verdict.not-observed.missing-case")?, Vec::new()));
        return Ok(ClaimVerdictDecisionV1::NotObserved);
    }

    let mut tuple_decisions = Vec::new();
    let tuple_cases = required_cases
        .iter()
        .cloned()
        .map(|value| vec![value])
        .collect::<Vec<_>>();
    for cases in tuple_cases {
        let relevant = validation
            .records
            .iter()
            .filter(|record| {
                let body = record.record.observation();
                body.observed_claim_ids().contains(claim.claim_id())
                    && body.evidence_kind() == requirement.evidence_kind()
                    && cases.contains(body.case().case_id())
            })
            .collect::<Vec<_>>();
        let mut eligible = Vec::new();
        let mut tuple_blocked = false;
        for record in relevant {
            if record.defect.is_some() {
                tuple_blocked = true;
                continue;
            }
            if !is_fresh(claim, &record.record, run, policy_registry)? {
                push_unique(stale, record.binding.clone());
                reasons.push((
                    reason("verdict.blocked.stale-evidence")?,
                    vec![record.binding.clone()],
                ));
                tuple_blocked = true;
                continue;
            }
            push_unique(fresh, record.binding.clone());
            if !matches!(
                meets_resolution(claim.minimum_resolution(), &record.record, resolution_stack),
                Ok(true)
            ) {
                push_unique(resolution_insufficient, record.binding.clone());
                reasons.push((
                    reason("verdict.blocked.insufficient-resolution")?,
                    vec![record.binding.clone()],
                ));
                tuple_blocked = true;
                continue;
            }
            push_unique(resolution_eligible, record.binding.clone());
            eligible.push(record);
        }
        if tuple_blocked {
            tuple_decisions.push(ClaimVerdictDecisionV1::Blocked);
            continue;
        }
        if eligible.is_empty() {
            reasons.push((reason("verdict.not-observed.missing-evidence")?, Vec::new()));
            tuple_decisions.push(ClaimVerdictDecisionV1::NotObserved);
            continue;
        }
        if requirement.cardinality() == EvidenceCardinalityV1::ExactlyOne && eligible.len() != 1 {
            let related = eligible
                .iter()
                .map(|record| record.binding.clone())
                .collect::<Vec<_>>();
            reasons.push((reason("verdict.blocked.cardinality-surplus")?, related));
            tuple_decisions.push(ClaimVerdictDecisionV1::Blocked);
            continue;
        }
        eligible.sort_by_key(|record| record.sequence);
        for record in &eligible {
            push_unique(supporting, record.binding.clone());
        }
        let first = eligible[0].record.observation().observation();
        if eligible
            .iter()
            .skip(1)
            .any(|record| record.record.observation().observation() != first)
        {
            reasons.push((
                reason("verdict.flaky.outcome-consensus")?,
                eligible
                    .iter()
                    .map(|record| record.binding.clone())
                    .collect(),
            ));
            tuple_decisions.push(ClaimVerdictDecisionV1::Flaky);
            continue;
        }
        let (decision, code) = match (first, claim.gate_effect()) {
            (EvidenceObservationV1::Satisfied, _) => (
                ClaimVerdictDecisionV1::Pass,
                "verdict.pass.observed-satisfaction",
            ),
            (EvidenceObservationV1::Violated, GateEffectV1::Advisory) => (
                ClaimVerdictDecisionV1::Warning,
                "verdict.warning.observed-violation",
            ),
            (EvidenceObservationV1::Violated, _) => (
                ClaimVerdictDecisionV1::Fail,
                "verdict.fail.observed-violation",
            ),
        };
        reasons.push((
            reason(code)?,
            eligible
                .iter()
                .map(|record| record.binding.clone())
                .collect(),
        ));
        tuple_decisions.push(decision);
    }
    Ok(combine_all_of(&tuple_decisions))
}

fn claim_applicability(
    claim: &ContractClaimV1,
    run: &ContractEvaluationRunV1,
) -> Result<(ApplicabilityOutcomeV1, Vec<StableIdV1>), ClaimVerdictEvaluationErrorV1> {
    let subject = matcher_candidate(run.subject().matcher_inputs(), run.matcher())?;
    let mut matches = Vec::new();
    let mut blocked = false;
    for case in run.cases() {
        let case_candidate = matcher_candidate(case.matcher_inputs(), run.matcher())?;
        match evaluate_applicability(claim, &subject, &case_candidate) {
            ApplicabilityOutcomeV1::Applicable => matches.push(case.case_id().clone()),
            ApplicabilityOutcomeV1::NotApplicable => {}
            ApplicabilityOutcomeV1::Blocked => blocked = true,
        }
    }
    if blocked {
        Ok((ApplicabilityOutcomeV1::Blocked, Vec::new()))
    } else if matches.is_empty() {
        Ok((ApplicabilityOutcomeV1::NotApplicable, Vec::new()))
    } else {
        Ok((ApplicabilityOutcomeV1::Applicable, matches))
    }
}

fn matcher_candidate(
    values: &[crate::MatcherInputValueV1],
    matcher: &crate::MatcherBindingV1,
) -> Result<MatcherInputCandidateV1, ClaimVerdictEvaluationErrorV1> {
    MatcherInputCandidateV1::current(
        ApplicabilityInputSourceV1::OwnerRegistry,
        matcher.input_schema().clone(),
        values
            .iter()
            .map(|value| MatcherValueV1::new(value.field().clone(), value.value()))
            .collect::<Result<Vec<_>, _>>()
            .map_err(|_| internal("matcher inputs cannot be rederived"))?,
    )
    .map_err(|_| internal("matcher input closure cannot be rederived"))
}

fn is_fresh(
    claim: &ContractClaimV1,
    record: &crate::EvidenceRecordV1,
    run: &ContractEvaluationRunV1,
    registry: &EvaluationPolicyRegistryV1,
) -> Result<bool, ClaimVerdictEvaluationErrorV1> {
    let registered = registry
        .resolve_exact(claim.freshness_policy())
        .map_err(|_| internal("freshness policy is not exactly registered"))?;
    if registered.kind() != EvaluationPolicyKindV1::Freshness {
        return Ok(false);
    }
    let policy: EvidenceFreshnessPolicyV1 = serde_json::from_slice(&registered.canonical_json())
        .map_err(|_| internal("freshness policy bytes are malformed"))?;
    let observed = semantic_seconds(record.observation().observed_at())?;
    let evaluation = semantic_seconds(run.owner_currentness().semantic_time_utc())?;
    let Some(age) = evaluation.checked_sub(observed) else {
        return Ok(false);
    };
    Ok(age >= 0
        && age <= i128::from(policy.max_observation_age_seconds())
        && policy.require_exact_source_revision()
        && policy.future_skew_seconds() == 0)
}

fn meets_resolution(
    minimum: &ResolutionMinimumV1,
    record: &crate::EvidenceRecordV1,
    stack: &ContextResolutionStackDefinition,
) -> Result<bool, ClaimVerdictEvaluationErrorV1> {
    let actual = resolution_observation(record.effective_resolution())?;
    let minimum = ResolutionObservationV1::new(
        minimum.resolution_stack().reference().as_str(),
        minimum.resolution_stack().fingerprint().as_str(),
        engine_resolution_values(minimum.dimension_values())?
            .each_ref()
            .map(String::as_str),
    )
    .map_err(|_| internal("claim minimum Resolution is malformed"))?;
    resolution_meets_minimum_v1(stack, &actual, &minimum)
        .map_err(|_| internal("Resolution comparison is indeterminate"))
}

fn resolution_observation(
    value: &ResolutionEnvelopeV1,
) -> Result<ResolutionObservationV1, ClaimVerdictEvaluationErrorV1> {
    ResolutionObservationV1::new(
        value.resolution_stack().reference().as_str(),
        value.resolution_stack().fingerprint().as_str(),
        engine_resolution_values(value.dimension_values())?
            .each_ref()
            .map(String::as_str),
    )
    .map_err(|_| internal("effective Resolution is malformed"))
}

fn engine_resolution_values(
    values: [&StableIdV1; 6],
) -> Result<[String; 6], ClaimVerdictEvaluationErrorV1> {
    const PREFIXES: [&str; 6] = [
        "resolution.scope.",
        "resolution.detail.",
        "resolution.time.",
        "resolution.authority.",
        "resolution.memory.",
        "resolution.validation.",
    ];
    let mut result = std::array::from_fn(|_| String::new());
    for index in 0..6 {
        let suffix = values[index]
            .as_str()
            .strip_prefix(PREFIXES[index])
            .ok_or_else(|| internal("Resolution namespace is substituted"))?;
        result[index] = suffix.replace('-', "_");
    }
    Ok(result)
}

fn combine_all_of(values: &[ClaimVerdictDecisionV1]) -> ClaimVerdictDecisionV1 {
    for candidate in [
        ClaimVerdictDecisionV1::Blocked,
        ClaimVerdictDecisionV1::Flaky,
        ClaimVerdictDecisionV1::Fail,
        ClaimVerdictDecisionV1::Warning,
        ClaimVerdictDecisionV1::NotObserved,
        ClaimVerdictDecisionV1::Pass,
    ] {
        if values.contains(&candidate) {
            return candidate;
        }
    }
    ClaimVerdictDecisionV1::Blocked
}

fn entry_binding(
    entry: &EvidenceEvaluationEntryV1,
) -> Result<ContentAddressedBindingV1, ClaimVerdictEvaluationErrorV1> {
    ContentAddressedBindingV1::new(
        entry.record.record_ref(),
        entry.record.record_fingerprint().clone(),
    )
    .map_err(|_| internal("evidence binding cannot be rederived"))
}

fn parse_exact_candidate(bytes: &[u8]) -> Result<crate::EvidenceCandidateV1, StableIdV1> {
    parse_exact(bytes, "verdict.blocked.candidate-malformed")
}

fn parse_exact_run(bytes: &[u8]) -> Result<ContractEvaluationRunV1, StableIdV1> {
    parse_exact(bytes, "verdict.blocked.run-substituted")
}

fn parse_exact<T: serde::de::DeserializeOwned + serde::Serialize>(
    bytes: &[u8],
    code: &'static str,
) -> Result<T, StableIdV1> {
    let value: serde_json::Value =
        serde_json::from_slice(bytes).map_err(|_| reason_unchecked(code))?;
    if serde_json_canonicalizer::to_vec(&value).map_err(|_| reason_unchecked(code))? != bytes {
        return Err(reason_unchecked(code));
    }
    serde_json::from_value(value).map_err(|_| reason_unchecked(code))
}

fn semantic_seconds(value: &SemanticTimeV1) -> Result<i128, ClaimVerdictEvaluationErrorV1> {
    let bytes = value.as_str().as_bytes();
    let number = |start: usize, end: usize| -> Result<i128, ClaimVerdictEvaluationErrorV1> {
        std::str::from_utf8(&bytes[start..end])
            .ok()
            .and_then(|value| value.parse::<i128>().ok())
            .ok_or_else(|| internal("semantic time arithmetic is indeterminate"))
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
        .ok_or_else(|| internal("semantic time arithmetic overflowed"))?;
    days.checked_mul(86_400)
        .and_then(|value| value.checked_add(hour * 3_600 + minute * 60 + second))
        .ok_or_else(|| internal("semantic time arithmetic overflowed"))
}

fn sort_bindings_by_sequence(
    values: &mut Vec<ContentAddressedBindingV1>,
    records: &[CurrentRecordV1],
) {
    values.sort_by_key(|value| {
        records
            .iter()
            .find(|record| &record.binding == value)
            .map_or(u64::MAX, |record| record.sequence)
    });
    values.dedup();
}

fn push_unique(values: &mut Vec<ContentAddressedBindingV1>, value: ContentAddressedBindingV1) {
    if !values.contains(&value) {
        values.push(value);
    }
}

fn normalize_reasons(values: &mut Vec<(StableIdV1, Vec<ContentAddressedBindingV1>)>) {
    values.dedup();
}

fn validation_fingerprint(value: &RevalidationV1) -> Vec<(String, Option<String>, Option<String>)> {
    value
        .records
        .iter()
        .map(|record| {
            (
                record.binding.reference().to_owned(),
                record.authority.as_ref().and_then(|authority| {
                    if authority.is_local() {
                        // Each native callback observes its own time. The
                        // repeated check compares all authority and policy
                        // fields while allowing time to advance normally.
                        let mut identity = serde_json::to_value(authority).ok()?;
                        identity.as_object_mut()?.remove("observed_at");
                        serde_json_canonicalizer::to_string(&identity).ok()
                    } else {
                        serde_json_canonicalizer::to_string(authority).ok()
                    }
                }),
                record.defect.as_ref().map(|code| code.as_str().to_owned()),
            )
        })
        .collect()
}

fn reason(value: &'static str) -> Result<StableIdV1, ClaimVerdictEvaluationErrorV1> {
    StableIdV1::parse(value).map_err(|_| internal("verdict reason code is invalid"))
}

fn reason_unchecked(value: &'static str) -> StableIdV1 {
    StableIdV1::parse(value).expect("checked-in verdict reason is valid")
}

fn source_error_reason(kind: EvidenceCandidateSourceErrorKindV1) -> StableIdV1 {
    reason_unchecked(match kind {
        EvidenceCandidateSourceErrorKindV1::Missing => "verdict.blocked.candidate-missing",
        EvidenceCandidateSourceErrorKindV1::Malformed => "verdict.blocked.candidate-malformed",
        EvidenceCandidateSourceErrorKindV1::OverLimit => "verdict.blocked.candidate-over-limit",
        EvidenceCandidateSourceErrorKindV1::CandidateSubstituted
        | EvidenceCandidateSourceErrorKindV1::InvalidRequest => {
            "verdict.blocked.evidence-substituted"
        }
        EvidenceCandidateSourceErrorKindV1::AuthorityAbsent
        | EvidenceCandidateSourceErrorKindV1::AuthorityAmbiguous
        | EvidenceCandidateSourceErrorKindV1::StaleAuthority
        | EvidenceCandidateSourceErrorKindV1::AuthenticationRefused => {
            "verdict.blocked.source-authority"
        }
        EvidenceCandidateSourceErrorKindV1::Unavailable => "verdict.blocked.source-unavailable",
    })
}

fn map_repository_error(
    error: crate::ContractOwnerRepositoryErrorV1,
) -> ClaimVerdictEvaluationErrorV1 {
    let kind = match error.kind() {
        ContractOwnerRepositoryErrorKindV1::DurabilityUncertain => {
            ClaimVerdictEvaluationErrorKindV1::RepositoryUnavailable
        }
        _ => ClaimVerdictEvaluationErrorKindV1::RepositoryCorrupt,
    };
    ClaimVerdictEvaluationErrorV1 {
        kind,
        detail: "evidence repository cannot supply an admitted exact snapshot root",
    }
}

const fn internal(detail: &'static str) -> ClaimVerdictEvaluationErrorV1 {
    ClaimVerdictEvaluationErrorV1 {
        kind: ClaimVerdictEvaluationErrorKindV1::InternalInvariant,
        detail,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn all_of_precedence_is_exact_and_closed() {
        use ClaimVerdictDecisionV1 as Verdict;
        for (clauses, expected) in [
            (vec![Verdict::Pass], Verdict::Pass),
            (
                vec![Verdict::Pass, Verdict::NotObserved],
                Verdict::NotObserved,
            ),
            (
                vec![Verdict::NotObserved, Verdict::Warning],
                Verdict::Warning,
            ),
            (vec![Verdict::Pass, Verdict::Fail], Verdict::Fail),
            (vec![Verdict::Fail, Verdict::Flaky], Verdict::Flaky),
            (vec![Verdict::Flaky, Verdict::Blocked], Verdict::Blocked),
            (
                vec![
                    Verdict::Pass,
                    Verdict::NotObserved,
                    Verdict::Warning,
                    Verdict::Fail,
                    Verdict::Flaky,
                    Verdict::Blocked,
                ],
                Verdict::Blocked,
            ),
        ] {
            assert_eq!(combine_all_of(&clauses), expected, "clauses: {clauses:?}");
        }
    }

    #[test]
    fn empty_or_illegal_internal_clause_partition_fails_closed() {
        assert_eq!(combine_all_of(&[]), ClaimVerdictDecisionV1::Blocked);
        assert_eq!(
            combine_all_of(&[ClaimVerdictDecisionV1::NotApplicable]),
            ClaimVerdictDecisionV1::Blocked
        );
    }
}
