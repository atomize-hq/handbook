use crate::evaluation_identity::{
    gate_result_from_composition_v1, local_closeout_result_from_composition_v1,
    EvidenceSourceAuthorityObservationV1, SourceCurrentnessObservationV1,
};
use crate::{
    evaluate_claim_verdicts_v1, ApplicabilityOutcomeV1, ClaimVerdictDecisionV1,
    ClaimVerdictEvaluationErrorKindV1, ClaimVerdictEvaluationErrorV1, ClaimVerdictEvaluationV1,
    ClaimVerdictV1, ContentAddressedBindingV1, ContractEvaluationRunV1,
    ContractOwnerRepositoryErrorKindV1, ContractOwnerRepositoryErrorV1, ContractOwnerRepositoryV1,
    EvaluationPolicyKindV1, EvaluationPolicyRegistryV1, EvidenceCandidateSourceV1, ExactBindingV1,
    ExactRationalThresholdV1, GateDecisionV1, GateEffectV1, GatePolicyAlgorithmV1, GatePolicyV1,
    GateResultV1, LocalCloseoutDecisionV1, LocalCloseoutResultV1, ScoreResultV1, ScoreWeightV1,
    StableIdV1,
};
use handbook_engine::{ContextResolutionStackDefinition, SourceCurrentnessPortV1};
use std::error::Error;
use std::fmt;

type GateIssueV1 = (StableIdV1, Vec<ContentAddressedBindingV1>);
type GateIssuesV1 = Vec<GateIssueV1>;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum ClaimGateDispositionV1 {
    Continue,
    Deficit,
    Block,
    Invalid,
}

fn classify_claim(effect: GateEffectV1, verdict: ClaimVerdictDecisionV1) -> ClaimGateDispositionV1 {
    use ClaimGateDispositionV1::{Block, Continue, Deficit, Invalid};
    use ClaimVerdictDecisionV1::{Blocked, Fail, Flaky, NotApplicable, NotObserved, Pass, Warning};
    use GateEffectV1::{Advisory, HardFail, Required};

    match (effect, verdict) {
        (HardFail | Required | Advisory, Pass | NotApplicable) => Continue,
        (HardFail | Required, Fail | Blocked | NotObserved | Flaky) => Block,
        (HardFail | Required, Warning) | (Advisory, Fail) => Invalid,
        (Advisory, Blocked | Warning | NotObserved | Flaky) => Deficit,
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum ScoreFailureV1 {
    ZeroDenominator,
    Overflow,
}

fn checked_add(left: u128, right: u128) -> Result<u128, ScoreFailureV1> {
    left.checked_add(right).ok_or(ScoreFailureV1::Overflow)
}

fn checked_threshold(
    passed_weight: u128,
    threshold_denominator: u128,
    total_weight: u128,
    threshold_numerator: u128,
) -> Result<bool, ScoreFailureV1> {
    let passed = passed_weight
        .checked_mul(threshold_denominator)
        .ok_or(ScoreFailureV1::Overflow)?;
    let required = total_weight
        .checked_mul(threshold_numerator)
        .ok_or(ScoreFailureV1::Overflow)?;
    Ok(passed >= required)
}

fn checked_score(
    entries: &[(
        ApplicabilityOutcomeV1,
        ClaimVerdictDecisionV1,
        Option<ScoreWeightV1>,
    )],
    threshold: Option<&ExactRationalThresholdV1>,
) -> Result<(ScoreResultV1, bool), ScoreFailureV1> {
    let mut passed_weight = 0_u128;
    let mut total_weight = 0_u128;
    for (applicability, verdict, weight) in entries {
        if *applicability == ApplicabilityOutcomeV1::NotApplicable {
            continue;
        }
        let Some(weight) = weight else {
            continue;
        };
        let weight = u128::from(weight.get());
        total_weight = checked_add(total_weight, weight)?;
        if *verdict == ClaimVerdictDecisionV1::Pass {
            passed_weight = checked_add(passed_weight, weight)?;
        }
    }
    if total_weight == 0 {
        return match threshold {
            Some(_) => Err(ScoreFailureV1::ZeroDenominator),
            None => Ok((ScoreResultV1::NotComputed, true)),
        };
    }
    let score = ScoreResultV1::Computed {
        passed_weight: u64::try_from(passed_weight).map_err(|_| ScoreFailureV1::Overflow)?,
        total_weight: u64::try_from(total_weight).map_err(|_| ScoreFailureV1::Overflow)?,
    };
    let passed = match threshold {
        Some(threshold) => checked_threshold(
            passed_weight,
            u128::from(threshold.denominator()),
            total_weight,
            u128::from(threshold.numerator()),
        )?,
        None => true,
    };
    Ok((score, passed))
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum GateEvaluationErrorKindV1 {
    RepositoryUnavailable,
    RepositoryCorrupt,
    InternalInvariant,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GateEvaluationErrorV1 {
    kind: GateEvaluationErrorKindV1,
    detail: String,
}

impl GateEvaluationErrorV1 {
    pub fn kind(&self) -> GateEvaluationErrorKindV1 {
        self.kind
    }

    pub fn detail(&self) -> &str {
        &self.detail
    }
}

impl fmt::Display for GateEvaluationErrorV1 {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.detail)
    }
}

impl Error for GateEvaluationErrorV1 {}

#[derive(Clone, Debug, PartialEq)]
pub struct GateEvaluationV1 {
    verdicts: Vec<ClaimVerdictV1>,
    gate_result: GateResultV1,
    local_closeout_result: LocalCloseoutResultV1,
}

impl GateEvaluationV1 {
    pub fn verdicts(&self) -> &[ClaimVerdictV1] {
        &self.verdicts
    }

    pub fn gate_result(&self) -> &GateResultV1 {
        &self.gate_result
    }

    pub fn local_closeout_result(&self) -> &LocalCloseoutResultV1 {
        &self.local_closeout_result
    }
}

struct GateCompositionV1 {
    score: ScoreResultV1,
    blockers: GateIssuesV1,
    next_actions: GateIssuesV1,
    decision: GateDecisionV1,
}

/// Recomputes the complete owner-derived verdict partition, composes one exact
/// gate, and derives one separate local-closeout observation. It persists and
/// mutates nothing.
pub fn evaluate_gate_and_local_closeout_v1<S, C>(
    owner: &ContractOwnerRepositoryV1,
    source: &mut S,
    source_currentness: &mut C,
    resolution_stack: &ContextResolutionStackDefinition,
    policy_registry: &EvaluationPolicyRegistryV1,
    run: &ContractEvaluationRunV1,
) -> Result<GateEvaluationV1, GateEvaluationErrorV1>
where
    S: EvidenceCandidateSourceV1,
    C: SourceCurrentnessPortV1,
{
    let derived = evaluate_claim_verdicts_v1(
        owner,
        source,
        source_currentness,
        resolution_stack,
        policy_registry,
        run,
    )
    .map_err(map_verdict_error)?;
    let gate_boundary = evaluate_claim_verdicts_v1(
        owner,
        source,
        source_currentness,
        resolution_stack,
        policy_registry,
        run,
    )
    .map_err(map_verdict_error)?;
    let gate_policy = resolve_gate_policy(policy_registry, run.gate_policy());
    let composition = compose_gate(
        run,
        &gate_boundary,
        gate_policy.as_ref(),
        derived != gate_boundary,
    )?;
    let run_binding_value = run_binding(run)?;
    let gate_authorities = verdict_authorities(gate_boundary.verdicts());
    let gate_result = gate_result_from_composition_v1(
        run_binding_value.clone(),
        run.evaluation_lifecycle_basis().clone(),
        gate_boundary.evidence_repository_root().clone(),
        run.gate_policy().clone(),
        gate_authorities,
        gate_boundary.verdicts(),
        composition.score,
        composition.blockers,
        composition.next_actions,
        composition.decision,
    )
    .map_err(|_| internal("gate result identity could not be constructed"))?;

    let local_boundary = evaluate_claim_verdicts_v1(
        owner,
        source,
        source_currentness,
        resolution_stack,
        policy_registry,
        run,
    )
    .map_err(map_verdict_error)?;
    let snapshot = owner
        .evidence_evaluation_snapshot_v1()
        .map_err(map_repository_error)?;
    let current_head = owner
        .current_lifecycle_head(run.contract().reference(), run.contract().fingerprint())
        .map_err(map_repository_error)?;
    let lifecycle_current = owner
        .readmit_evaluation_run_v1(run, policy_registry)
        .is_ok();
    let local_changed = local_boundary != gate_boundary
        || snapshot.root != *local_boundary.evidence_repository_root();
    let local_passed = gate_result.decision() == GateDecisionV1::Passed
        && lifecycle_current
        && !local_changed
        && gate_policy.is_some();
    let gate_binding = ContentAddressedBindingV1::new(
        gate_result.record_ref(),
        gate_result.record_fingerprint().clone(),
    )
    .map_err(|_| internal("gate binding could not be rederived"))?;
    let (local_blockers, local_actions) = local_issues(
        gate_result.decision(),
        lifecycle_current,
        local_changed,
        &gate_binding,
    )?;
    let local_closeout_result = local_closeout_result_from_composition_v1(
        run_binding_value.clone(),
        gate_binding,
        run.local_closeout_policy().clone(),
        current_head.lifecycle_fingerprint().clone(),
        snapshot_source_currentness(&snapshot.entries, &run_binding_value),
        verdict_authorities(local_boundary.verdicts()),
        if local_passed {
            LocalCloseoutDecisionV1::Eligible
        } else {
            LocalCloseoutDecisionV1::Blocked
        },
        local_blockers,
        local_actions,
    )
    .map_err(|_| internal("local-closeout identity could not be constructed"))?;

    Ok(GateEvaluationV1 {
        verdicts: gate_boundary.verdicts().to_vec(),
        gate_result,
        local_closeout_result,
    })
}

fn compose_gate(
    run: &ContractEvaluationRunV1,
    evaluation: &ClaimVerdictEvaluationV1,
    gate_policy: Option<&GatePolicyV1>,
    currentness_changed: bool,
) -> Result<GateCompositionV1, GateEvaluationErrorV1> {
    let mut blockers = Vec::new();
    let mut actions = Vec::new();
    if currentness_changed {
        push_issue(
            &mut blockers,
            &mut actions,
            "gate.blocked.currentness-changed",
            "gate.action.rederive-verdicts",
            Vec::new(),
        )?;
    }
    if !complete_partition(run, evaluation) {
        push_issue(
            &mut blockers,
            &mut actions,
            "gate.blocked.incomplete-partition",
            "gate.action.rederive-verdicts",
            verdict_bindings(evaluation.verdicts())?,
        )?;
    }
    if gate_policy
        .map(|policy| policy.algorithm() != GatePolicyAlgorithmV1::ClosedAccounting)
        .unwrap_or(true)
    {
        push_issue(
            &mut blockers,
            &mut actions,
            "gate.blocked.policy",
            "gate.action.restore-policy",
            Vec::new(),
        )?;
    }
    let mut global_related = Vec::new();
    for verdict in evaluation.verdicts() {
        if verdict
            .reason_codes()
            .any(|reason| global_integrity_reason(reason.as_str()))
        {
            global_related.push(verdict_binding(verdict)?);
        }
    }
    if !global_related.is_empty() {
        push_issue(
            &mut blockers,
            &mut actions,
            "gate.blocked.global-integrity",
            "gate.action.restore-currentness",
            global_related,
        )?;
    }
    for (claim, verdict) in run.claims().iter().zip(evaluation.verdicts()) {
        let related = vec![verdict_binding(verdict)?];
        match classify_claim(claim.gate_effect(), verdict.decision()) {
            ClaimGateDispositionV1::Continue | ClaimGateDispositionV1::Deficit => {}
            ClaimGateDispositionV1::Block => push_issue(
                &mut blockers,
                &mut actions,
                "gate.blocked.required-claim",
                "gate.action.resolve-required-claim",
                related,
            )?,
            ClaimGateDispositionV1::Invalid => push_issue(
                &mut blockers,
                &mut actions,
                "gate.blocked.invalid-verdict-effect",
                "gate.action.correct-verdict",
                related,
            )?,
        }
    }

    let mut score = ScoreResultV1::NotComputed;
    if blockers.is_empty() {
        let entries = run
            .claims()
            .iter()
            .zip(evaluation.verdicts())
            .map(|(claim, verdict)| {
                (
                    verdict.applicability(),
                    verdict.decision(),
                    claim.score_weight(),
                )
            })
            .collect::<Vec<_>>();
        match checked_score(
            &entries,
            gate_policy.and_then(GatePolicyV1::score_threshold),
        ) {
            Ok((computed, true)) => score = computed,
            Ok((computed, false)) => {
                score = computed;
                push_issue(
                    &mut blockers,
                    &mut actions,
                    "gate.blocked.score-threshold",
                    "gate.action.meet-score-threshold",
                    Vec::new(),
                )?;
            }
            Err(ScoreFailureV1::ZeroDenominator) => push_issue(
                &mut blockers,
                &mut actions,
                "gate.blocked.score-not-computed",
                "gate.action.observe-weighted-claim",
                Vec::new(),
            )?,
            Err(ScoreFailureV1::Overflow) => push_issue(
                &mut blockers,
                &mut actions,
                "gate.blocked.score-overflow",
                "gate.action.correct-score-input",
                Vec::new(),
            )?,
        }
    }
    Ok(GateCompositionV1 {
        score,
        decision: if blockers.is_empty() {
            GateDecisionV1::Passed
        } else {
            GateDecisionV1::Blocked
        },
        blockers,
        next_actions: actions,
    })
}

fn complete_partition(
    run: &ContractEvaluationRunV1,
    evaluation: &ClaimVerdictEvaluationV1,
) -> bool {
    let Ok(run_binding) = run_binding(run) else {
        return false;
    };
    let claim_ids = run
        .claims()
        .iter()
        .map(|claim| claim.claim_id().as_str())
        .collect::<Vec<_>>();
    let verdict_ids = evaluation
        .verdicts()
        .iter()
        .map(|verdict| verdict.claim_id().as_str())
        .collect::<Vec<_>>();
    partition_shape_matches(&claim_ids, &verdict_ids)
        && evaluation
            .verdicts()
            .iter()
            .zip(run.claims())
            .all(|(verdict, claim)| {
                verdict.claim_id() == claim.claim_id()
                    && verdict.evaluation_run() == &run_binding
                    && verdict.evaluation_lifecycle_basis() == run.evaluation_lifecycle_basis()
                    && verdict.contract() == &run.contract()
                    && verdict.evidence_repository_root() == evaluation.evidence_repository_root()
            })
}

fn partition_shape_matches<T: Eq>(claims: &[T], verdicts: &[T]) -> bool {
    claims.len() == verdicts.len()
        && claims
            .iter()
            .zip(verdicts)
            .all(|(claim, verdict)| claim == verdict)
        && verdicts
            .iter()
            .enumerate()
            .all(|(index, verdict)| !verdicts[..index].contains(verdict))
}

fn global_integrity_reason(reason: &str) -> bool {
    reason.starts_with("verdict.blocked.") && reason != "verdict.blocked.insufficient-resolution"
}

fn resolve_gate_policy(
    registry: &EvaluationPolicyRegistryV1,
    binding: &ExactBindingV1,
) -> Option<GatePolicyV1> {
    let registered = registry.resolve_exact(binding).ok()?;
    if registered.kind() != EvaluationPolicyKindV1::Gate {
        return None;
    }
    serde_json::from_slice(&registered.canonical_json()).ok()
}

fn run_binding(
    run: &ContractEvaluationRunV1,
) -> Result<ContentAddressedBindingV1, GateEvaluationErrorV1> {
    ContentAddressedBindingV1::new(
        run.evaluation_run_ref(),
        run.evaluation_run_fingerprint().clone(),
    )
    .map_err(|_| internal("evaluation-run binding could not be rederived"))
}

fn verdict_binding(
    verdict: &ClaimVerdictV1,
) -> Result<ContentAddressedBindingV1, GateEvaluationErrorV1> {
    ContentAddressedBindingV1::new(verdict.record_ref(), verdict.record_fingerprint().clone())
        .map_err(|_| internal("verdict binding could not be rederived"))
}

fn verdict_bindings(
    verdicts: &[ClaimVerdictV1],
) -> Result<Vec<ContentAddressedBindingV1>, GateEvaluationErrorV1> {
    verdicts.iter().map(verdict_binding).collect()
}

fn verdict_authorities(verdicts: &[ClaimVerdictV1]) -> Vec<EvidenceSourceAuthorityObservationV1> {
    let mut values = Vec::new();
    for authority in verdicts
        .iter()
        .flat_map(ClaimVerdictV1::evidence_source_authorities)
    {
        if !values.contains(authority) {
            values.push(authority.clone());
        }
    }
    values
}

fn snapshot_source_currentness(
    entries: &[crate::owner_repository::EvidenceEvaluationEntryV1],
    run: &ContentAddressedBindingV1,
) -> Vec<SourceCurrentnessObservationV1> {
    let mut values = Vec::new();
    for observation in entries
        .iter()
        .filter(|entry| entry.record.evaluation_run() == run)
        .flat_map(|entry| entry.record.source_currentness())
    {
        if !values.contains(observation) {
            values.push(observation.clone());
        }
    }
    values
}

fn local_issues(
    gate_decision: GateDecisionV1,
    lifecycle_current: bool,
    currentness_changed: bool,
    gate: &ContentAddressedBindingV1,
) -> Result<(GateIssuesV1, GateIssuesV1), GateEvaluationErrorV1> {
    let mut blockers = Vec::new();
    let mut actions = Vec::new();
    if gate_decision != GateDecisionV1::Passed {
        push_issue(
            &mut blockers,
            &mut actions,
            "local-closeout.blocked.gate",
            "local-closeout.action.pass-gate",
            vec![gate.clone()],
        )?;
    }
    if !lifecycle_current {
        push_issue(
            &mut blockers,
            &mut actions,
            "local-closeout.blocked.lifecycle",
            "local-closeout.action.restore-lifecycle",
            Vec::new(),
        )?;
    }
    if currentness_changed {
        push_issue(
            &mut blockers,
            &mut actions,
            "local-closeout.blocked.currentness-changed",
            "local-closeout.action.reevaluate-gate",
            vec![gate.clone()],
        )?;
    }
    Ok((blockers, actions))
}

fn push_issue(
    blockers: &mut GateIssuesV1,
    actions: &mut GateIssuesV1,
    blocker_code: &'static str,
    action_code: &'static str,
    related: Vec<ContentAddressedBindingV1>,
) -> Result<(), GateEvaluationErrorV1> {
    let blocker_code =
        StableIdV1::parse(blocker_code).map_err(|_| internal("gate blocker code is invalid"))?;
    let action_code =
        StableIdV1::parse(action_code).map_err(|_| internal("gate action code is invalid"))?;
    merge_issue(blockers, blocker_code, related.clone());
    merge_issue(actions, action_code, related);
    Ok(())
}

fn merge_issue(
    issues: &mut GateIssuesV1,
    code: StableIdV1,
    related: Vec<ContentAddressedBindingV1>,
) {
    if let Some((_, existing)) = issues.iter_mut().find(|(existing, _)| *existing == code) {
        for binding in related {
            if !existing.contains(&binding) {
                existing.push(binding);
            }
        }
    } else {
        issues.push((code, related));
    }
}

fn map_verdict_error(error: ClaimVerdictEvaluationErrorV1) -> GateEvaluationErrorV1 {
    let kind = match error.kind() {
        ClaimVerdictEvaluationErrorKindV1::RepositoryUnavailable => {
            GateEvaluationErrorKindV1::RepositoryUnavailable
        }
        ClaimVerdictEvaluationErrorKindV1::RepositoryCorrupt => {
            GateEvaluationErrorKindV1::RepositoryCorrupt
        }
        ClaimVerdictEvaluationErrorKindV1::InternalInvariant => {
            GateEvaluationErrorKindV1::InternalInvariant
        }
    };
    GateEvaluationErrorV1 {
        kind,
        detail: error.detail().to_owned(),
    }
}

fn map_repository_error(error: ContractOwnerRepositoryErrorV1) -> GateEvaluationErrorV1 {
    let kind = match error.kind() {
        ContractOwnerRepositoryErrorKindV1::DurabilityUncertain => {
            GateEvaluationErrorKindV1::RepositoryUnavailable
        }
        ContractOwnerRepositoryErrorKindV1::UnsafeFilesystem
        | ContractOwnerRepositoryErrorKindV1::RecoveryAmbiguous
        | ContractOwnerRepositoryErrorKindV1::RecordSubstituted => {
            GateEvaluationErrorKindV1::RepositoryCorrupt
        }
        _ => GateEvaluationErrorKindV1::InternalInvariant,
    };
    GateEvaluationErrorV1 {
        kind,
        detail: error.detail().to_owned(),
    }
}

fn internal(detail: &'static str) -> GateEvaluationErrorV1 {
    GateEvaluationErrorV1 {
        kind: GateEvaluationErrorKindV1::InternalInvariant,
        detail: detail.to_owned(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        ApplicabilityOutcomeV1 as Applicability, ClaimVerdictDecisionV1 as Verdict,
        ExactRationalThresholdV1, GateEffectV1 as Effect, ScoreResultV1, ScoreWeightV1,
    };
    use ClaimGateDispositionV1 as Expected;

    #[test]
    fn exact_gate_effect_verdict_matrix_is_closed() {
        let cases = [
            (Effect::HardFail, Verdict::Pass, Expected::Continue),
            (Effect::HardFail, Verdict::NotApplicable, Expected::Continue),
            (Effect::HardFail, Verdict::Fail, Expected::Block),
            (Effect::HardFail, Verdict::Blocked, Expected::Block),
            (Effect::HardFail, Verdict::Warning, Expected::Invalid),
            (Effect::HardFail, Verdict::NotObserved, Expected::Block),
            (Effect::HardFail, Verdict::Flaky, Expected::Block),
            (Effect::Required, Verdict::Pass, Expected::Continue),
            (Effect::Required, Verdict::NotApplicable, Expected::Continue),
            (Effect::Required, Verdict::Fail, Expected::Block),
            (Effect::Required, Verdict::Blocked, Expected::Block),
            (Effect::Required, Verdict::Warning, Expected::Invalid),
            (Effect::Required, Verdict::NotObserved, Expected::Block),
            (Effect::Required, Verdict::Flaky, Expected::Block),
            (Effect::Advisory, Verdict::Pass, Expected::Continue),
            (Effect::Advisory, Verdict::NotApplicable, Expected::Continue),
            (Effect::Advisory, Verdict::Fail, Expected::Invalid),
            (Effect::Advisory, Verdict::Blocked, Expected::Deficit),
            (Effect::Advisory, Verdict::Warning, Expected::Deficit),
            (Effect::Advisory, Verdict::NotObserved, Expected::Deficit),
            (Effect::Advisory, Verdict::Flaky, Expected::Deficit),
        ];

        for (effect, verdict, expected) in cases {
            assert_eq!(classify_claim(effect, verdict), expected);
        }
    }

    #[test]
    fn exact_score_excludes_omitted_and_not_applicable_and_uses_rational_boundaries() {
        let weight = |value| Some(ScoreWeightV1::new(value).unwrap());
        let entries = [
            (Applicability::Applicable, Verdict::Pass, weight(2)),
            (Applicability::Applicable, Verdict::Warning, weight(1)),
            (Applicability::Applicable, Verdict::Pass, None),
            (
                Applicability::NotApplicable,
                Verdict::NotApplicable,
                weight(9),
            ),
        ];
        let equal = ExactRationalThresholdV1::new(2, 3).unwrap();
        let above = ExactRationalThresholdV1::new(3, 4).unwrap();

        assert_eq!(
            checked_score(&entries, Some(&equal)).unwrap(),
            (
                ScoreResultV1::Computed {
                    passed_weight: 2,
                    total_weight: 3
                },
                true
            )
        );
        assert_eq!(
            checked_score(&entries, Some(&above)).unwrap(),
            (
                ScoreResultV1::Computed {
                    passed_weight: 2,
                    total_weight: 3
                },
                false
            )
        );
        assert_eq!(
            checked_score(&entries[2..], None).unwrap(),
            (ScoreResultV1::NotComputed, true)
        );
        assert_eq!(
            checked_score(&entries[2..], Some(&equal)),
            Err(ScoreFailureV1::ZeroDenominator)
        );
    }

    #[test]
    fn checked_score_arithmetic_refuses_sum_and_cross_product_overflow() {
        assert_eq!(checked_add(u128::MAX, 1), Err(ScoreFailureV1::Overflow));
        assert_eq!(
            checked_threshold(u128::MAX, 1, 2, u128::MAX),
            Err(ScoreFailureV1::Overflow)
        );
    }

    #[test]
    fn verdict_partition_requires_exact_definition_order_without_duplicates() {
        let claims = ["claim.a", "claim.b"];
        assert!(partition_shape_matches(&claims, &["claim.a", "claim.b"]));
        assert!(!partition_shape_matches(&claims, &["claim.a"]));
        assert!(!partition_shape_matches(&claims, &["claim.a", "claim.a"]));
        assert!(!partition_shape_matches(&claims, &["claim.b", "claim.a"]));
        assert!(!partition_shape_matches(
            &claims,
            &["claim.a", "claim.b", "claim.c"]
        ));
    }

    #[test]
    fn global_integrity_and_local_lifecycle_block_before_scoring_or_eligibility() {
        assert!(global_integrity_reason("verdict.blocked.lifecycle-changed"));
        assert!(global_integrity_reason(
            "verdict.blocked.source-currentness"
        ));
        assert!(global_integrity_reason("verdict.blocked.source-authority"));
        assert!(global_integrity_reason(
            "verdict.blocked.evidence-substituted"
        ));
        assert!(global_integrity_reason(
            "verdict.blocked.cardinality-surplus"
        ));
        assert!(!global_integrity_reason(
            "verdict.blocked.insufficient-resolution"
        ));

        let gate = ContentAddressedBindingV1::new(
            "contract.example@1.0.0#gate/aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
            crate::FingerprintV1::parse(&format!("sha256:{}", "a".repeat(64))).unwrap(),
        )
        .unwrap();
        let (blockers, actions) =
            local_issues(GateDecisionV1::Passed, false, false, &gate).unwrap();
        assert_eq!(blockers[0].0.as_str(), "local-closeout.blocked.lifecycle");
        assert_eq!(
            actions[0].0.as_str(),
            "local-closeout.action.restore-lifecycle"
        );
    }
}
