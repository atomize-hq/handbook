//! The bounded complete local check. Coordination, admission, set formation and
//! pure truth remain distinct owner responsibilities in one finite route.
use crate::canonical::{canonical_bytes, canonical_fingerprint};
use crate::*;
use handbook_engine::local_authority::local_check::LocalCheckCurrentBasisV1;
use handbook_engine::local_authority::local_evaluation::*;
use serde::{Deserialize, Serialize};
use serde_json::Value;

pub const LOCAL_EVALUATION_OWNER_ROOT_V3: &str = ".handbook/state/contracts/local-evaluation-v3";

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct LocalEvaluationPrepareInputV3 {
    pub request_key: String,
    pub contract: ExactBindingV1,
    pub expected_lifecycle_fingerprint: FingerprintV1,
    pub subject: EvaluationSubjectV1,
    pub case: EvaluationCaseV1,
    pub requested_resolution: ResolutionEnvelopeV1,
    pub formation: LocalEvaluationFormationRequestV1,
}
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct LocalEvaluationRequestLocatorV3 {
    pub request_key: String,
    pub request_fingerprint: FingerprintV1,
}
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct LocalEvaluationRequirementsV3 {
    pub schema_ref: String,
    pub source: String,
    pub contract: ExactBindingV1,
    pub permitted_use: LocalPermittedUseV2,
    pub intent: LocalContractUseIntentV2,
    pub claims: Vec<ContractClaimV1>,
    pub subject: EvaluationSubjectV1,
    pub case: EvaluationCaseV1,
    pub observation_roles: Vec<String>,
    pub metrics: Vec<ExactBindingV1>,
    pub dependencies: LocalDefinitionDependenciesV3,
    pub applicability: LocalContractApplicabilityV2,
    pub selected: Vec<ExactBindingV1>,
    pub excluded: Vec<ExactBindingV1>,
    pub missing: Vec<ExactBindingV1>,
    pub conflicting: Vec<ExactBindingV1>,
    pub fingerprint: FingerprintV1,
}
impl LocalEvaluationRequirementsV3 {
    pub(crate) fn expected_fingerprint(&self) -> Result<FingerprintV1, ContractError> {
        let mut value = serde_json::to_value(self).map_err(|_| ContractError::InvalidInvariant)?;
        value.as_object_mut().unwrap().remove("fingerprint");
        canonical_fingerprint(&value)
    }
}
#[derive(Clone, Debug)]
pub struct PreparedLocalEvaluationV3 {
    pub(crate) input: LocalEvaluationPrepareInputV3,
    pub(crate) local_use: LocalContractUseV3,
    pub(crate) requirements: LocalEvaluationRequirementsV3,
    pub(crate) plan: LocalEvaluationPlanV1,
    pub(crate) request_fingerprint: FingerprintV1,
}
impl PreparedLocalEvaluationV3 {
    pub fn input(&self) -> &LocalEvaluationPrepareInputV3 {
        &self.input
    }
    pub fn formation_request(&self) -> &LocalEvaluationFormationRequestV1 {
        &self.input.formation
    }
    pub fn current_basis(&self) -> &LocalCheckCurrentBasisV1 {
        self.local_use.current_basis()
    }
    pub fn requirements(&self) -> &LocalEvaluationRequirementsV3 {
        &self.requirements
    }
    pub fn plan(&self) -> &LocalEvaluationPlanV1 {
        &self.plan
    }
    pub fn request_fingerprint(&self) -> &FingerprintV1 {
        &self.request_fingerprint
    }
    pub fn request_locator(&self) -> LocalEvaluationRequestLocatorV3 {
        LocalEvaluationRequestLocatorV3 {
            request_key: self.input.request_key.clone(),
            request_fingerprint: self.request_fingerprint.clone(),
        }
    }
}
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct LocalEvaluationLocatorV3 {
    pub request_key: String,
    pub request_fingerprint: FingerprintV1,
    pub occurrence: ContentAddressedBindingV1,
    pub owner_identity: FingerprintV1,
    pub lock_generation: String,
}
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct LocalEvaluationResultLocatorV3 {
    pub evaluation: LocalEvaluationLocatorV3,
    pub result: ContentAddressedBindingV1,
}
#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum LocalEvaluationStageV3 {
    Purpose,
    Held,
    Attemptable,
    Committed,
    OriginClosed,
    FrontierSealed,
    SetClosed,
    ResultClosed,
    Refused,
}
#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum LocalEvaluationReadinessV3 {
    Ready,
    KnownLimited,
    Unknown,
}
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct LocalEvaluationReadinessBasisV3 {
    pub capability: NativeLocalEvaluationCapabilityV3,
    pub verification: LocalEvaluationVerificationObservationV1,
    pub readiness: LocalEvaluationReadinessV3,
    pub disposition: String,
    pub reason: String,
    pub observed_at: SemanticTimeV1,
    pub fingerprint: FingerprintV1,
}
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct LocalEvaluationAdmissionTerminalV3 {
    pub operation_id: String,
    pub candidate: ContentAddressedBindingV1,
    pub disposition: String,
    pub reason: String,
    pub evidence: Option<ContentAddressedBindingV1>,
    pub semantic_time: SemanticTimeV1,
}
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct LocalCanonicalEvidenceV3 {
    pub schema_ref: String,
    pub occurrence: ContentAddressedBindingV1,
    pub basis: ContentAddressedBindingV1,
    pub selection: ExactBindingV1,
    pub attempt: ContentAddressedBindingV1,
    pub candidate: EvidenceCandidateV1,
    pub admission_policy: ExactBindingV1,
    pub verification: LocalEvaluationVerificationObservationV1,
    pub admitted_at: SemanticTimeV1,
    pub identity: ContentAddressedBindingV1,
    pub coverage_sufficient: bool,
}
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct LocalEvaluationFrontierV3 {
    pub occurrence: ContentAddressedBindingV1,
    pub basis: ContentAddressedBindingV1,
    pub plan_fingerprint: FingerprintV1,
    pub attempt: ContentAddressedBindingV1,
    pub origin_terminal: Value,
    pub collection_checks: Vec<LocalEvaluationVerificationObservationV1>,
    pub sealed_at: SemanticTimeV1,
    pub identity: ContentAddressedBindingV1,
}
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct LocalEvaluationEvidenceSetV3 {
    pub occurrence: ContentAddressedBindingV1,
    pub basis: ContentAddressedBindingV1,
    pub frontier: ContentAddressedBindingV1,
    pub formation_policy: ExactBindingV1,
    pub eligible_owner: FingerprintV1,
    pub eligible_namespace: String,
    pub members: Vec<ContentAddressedBindingV1>,
    pub admission_terminals: Vec<LocalEvaluationAdmissionTerminalV3>,
    pub nonmembers: Vec<LocalEvaluationAdmissionTerminalV3>,
    pub closed_at: SemanticTimeV1,
    pub identity: ContentAddressedBindingV1,
}
#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum LocalEvaluationGateV3 {
    Pass,
    Fail,
    Indeterminate,
    NotApplicable,
}
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct LocalEvaluationClaimResultV3 {
    pub claim_id: StableIdV1,
    pub applicability: ApplicabilityOutcomeV1,
    pub finding: String,
    pub coverage: String,
    pub currentness: String,
    pub consistency: String,
    pub uncertainty: Vec<String>,
    pub violations: Vec<Value>,
    pub supporting_evidence: Vec<ContentAddressedBindingV1>,
}
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct LocalEvaluationRetentionDependencyV3 {
    pub owner: String,
    pub path: String,
    pub bytes_fingerprint: FingerprintV1,
    pub retained_until: SemanticTimeV1,
}
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct LocalEvaluationResultV3 {
    pub schema_ref: String,
    pub occurrence: ContentAddressedBindingV1,
    pub purpose: LocalEvaluationPurposeV1,
    pub context: ContentAddressedBindingV1,
    pub evidence_set: LocalEvaluationEvidenceSetV3,
    pub final_verification: LocalEvaluationVerificationObservationV1,
    pub claim: LocalEvaluationClaimResultV3,
    pub metrics: Vec<Value>,
    pub gate_policy: ExactBindingV1,
    pub gate: LocalEvaluationGateV3,
    pub explanation: Value,
    pub dependencies: Vec<LocalEvaluationRetentionDependencyV3>,
    pub retained_until: SemanticTimeV1,
    pub evaluated_at: SemanticTimeV1,
    pub identity: ContentAddressedBindingV1,
}
impl LocalEvaluationResultV3 {
    pub fn canonical_json_bytes(&self) -> Result<Vec<u8>, ContractError> {
        canonical_bytes(self)
    }
    pub fn human_summary(&self) -> String {
        let applicability = &self.explanation["applicability"];
        let governance = &applicability["governance_basis"];
        let basis = &governance["basis"];
        let qualification = serde_json::json!({
            "governance_basis": {
                "state": governance["state"],
                "project_identity": basis["project"]["repository_identity_fingerprint"],
                "scope": basis["scope"],
                "relation_fingerprint": basis["relation_fingerprint"],
                "outcome_fingerprint": basis["outcome_fingerprint"],
                "configuration": basis["configuration"],
                "establishment_rule": basis["establishment_rule"],
                "owner_rule": basis["owner_rule"],
            },
            "owner_rule": applicability["owner_rule"],
            "dependencies": applicability["dependencies"],
            "policy_admission_closure_fingerprint": applicability["policy_admission_closure_fingerprint"],
        });
        let readiness = self.explanation["readiness"].as_array().map(|bases| {
            bases.iter().map(|basis| {
                let capability = &basis["capability"];
                let controls = &capability["controls"];
                serde_json::json!({
                    "readiness": basis["readiness"], "disposition": basis["disposition"],
                    "reason": basis["reason"], "fingerprint": basis["fingerprint"],
                    "verification": basis["verification"]["observation_fingerprint"],
                    "capability": {
                        "semantic_capability": capability["semantic_capability"],
                        "manifest": capability["manifest"],
                        "implementation_bundle": capability["implementation_bundle"],
                        "target": capability["target"], "availability": capability["availability"],
                        "fingerprint": capability["fingerprint"], "observed_at": capability["observed_at"],
                        "retention_until": capability["retention_until"],
                        "controls": {
                            "profile": controls["profile"], "environment": controls["environment"],
                            "cwd": controls["cwd"], "time": controls["time"], "output": controls["output"],
                            "cancellation": controls["cancellation"],
                            "network_enforcement": controls["network_enforcement"],
                            "availability_reason": controls["availability_reason"],
                            "operating_ceiling": controls["operating_ceiling"],
                            "effective_limits": controls["effective_limits"],
                        },
                    },
                })
            }).collect::<Vec<_>>()
        });
        format!(
            "{:?}: {}. Applicability: {:?}; coverage: {}; currentness: {}; consistency: {}; uncertainty: {}. \
             Source: direct_contract; intent: {:?}; permitted use: {}. \
             Original governance and dependency qualification: {}. Violations: {}. \
             Readiness: {}. Native operational terminal: {}. \
             Accounting: {} included member(s), {} admission terminal(s), {} admitted, {} duplicate, {} refused, {} nonmember(s); exact set: {}. \
             Final verification: {}. Shipment or completion obligation: not assessed.",
            self.gate,
            self.claim.finding,
            self.claim.applicability,
            self.claim.coverage,
            self.claim.currentness,
            self.claim.consistency,
            serde_json::to_string(&self.claim.uncertainty).unwrap_or_else(|_| "unavailable".into()),
            self.purpose.use_intent,
            self.purpose.permitted_use,
            qualification,
            serde_json::to_string(&self.claim.violations).unwrap_or_else(|_| "unavailable".into()),
            serde_json::to_value(readiness).unwrap_or(Value::Null),
            self.explanation["origin_terminal"],
            self.evidence_set.members.len(),
            self.evidence_set.admission_terminals.len(),
            self.evidence_set.admission_terminals.iter().filter(|t| t.disposition == "admitted").count(),
            self.evidence_set.admission_terminals.iter().filter(|t| t.disposition == "duplicate").count(),
            self.evidence_set.admission_terminals.iter().filter(|t| t.disposition == "refused").count(),
            self.evidence_set.nonmembers.len(),
            self.evidence_set.identity.fingerprint().as_str(),
            self.final_verification.observation_fingerprint,
        )
    }
}
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct LocalEvaluationSnapshotV3 {
    pub locator: LocalEvaluationLocatorV3,
    pub ordinal: u64,
    pub sequence: u64,
    pub stage: LocalEvaluationStageV3,
    pub requirements: LocalEvaluationRequirementsV3,
    pub purpose: LocalEvaluationPurposeV1,
    pub plan: LocalEvaluationPlanV1,
    pub run: ContractEvaluationRunV1,
    pub current_basis: LocalCheckCurrentBasisV1,
    pub original_input: LocalEvaluationPrepareInputV3,
    pub definition: LocalContractDefinitionV3,
    pub formation_authority_fingerprint: String,
    pub authorization: Option<LocalEvaluationAuthorizationBindingV1>,
    pub issuance_locator: Option<LocalEvaluationBindingLocatorV1>,
    pub retained_lock_generations: Vec<handbook_engine::local_authority::LocalBindingV1>,
    pub readiness_bases: Vec<LocalEvaluationReadinessBasisV3>,
    pub committed_basis: Option<ContentAddressedBindingV1>,
    pub collection_checks: Vec<LocalEvaluationVerificationObservationV1>,
    pub attempt: Option<ContentAddressedBindingV1>,
    pub origin_intent: Option<Value>,
    pub origin_terminal: Option<Value>,
    pub origin_paths: Vec<String>,
    pub evidence: Vec<LocalCanonicalEvidenceV3>,
    pub admission_terminals: Vec<LocalEvaluationAdmissionTerminalV3>,
    pub frontier: Option<LocalEvaluationFrontierV3>,
    pub evidence_set: Option<LocalEvaluationEvidenceSetV3>,
    pub result: Option<LocalEvaluationResultV3>,
    pub reason: Option<String>,
    pub identity: ContentAddressedBindingV1,
}
impl LocalEvaluationSnapshotV3 {
    pub fn result_locator(&self) -> Result<LocalEvaluationResultLocatorV3, ContractError> {
        Ok(LocalEvaluationResultLocatorV3 {
            evaluation: self.locator.clone(),
            result: self
                .result
                .as_ref()
                .ok_or(ContractError::InvalidInvariant)?
                .identity
                .clone(),
        })
    }
    pub fn authorization_request(
        &self,
        envelope: LocalEvaluationEnvelopeV1,
        reach: LocalEvaluationPathReachV1,
    ) -> LocalEvaluationAuthorizationRequestV1 {
        LocalEvaluationAuthorizationRequestV1 {
            request_key: format!("{}-authorization", self.locator.request_key),
            purpose: self.purpose.clone(),
            plan: self.plan.clone(),
            envelope,
            current_basis: self.current_basis.clone(),
            reach,
            retention_until_utc: self.purpose.retention_until_utc.clone(),
        }
    }
    pub fn occurrence(&self) -> &ContentAddressedBindingV1 {
        &self.locator.occurrence
    }
    pub fn basis(&self) -> Option<&ContentAddressedBindingV1> {
        self.committed_basis.as_ref()
    }
    pub fn selection(&self) -> &ExactBindingV1 {
        &self.definition.dependencies().validator.manifest
    }
    pub fn authorization_binding(&self) -> Result<ContentAddressedBindingV1, ContractError> {
        let authorization = self
            .authorization
            .as_ref()
            .ok_or(ContractError::InvalidInvariant)?;
        ContentAddressedBindingV1::new(
            &format!(
                "handbook.engine.local-evaluation-issuance@1.0.0#{}",
                authorization
                    .issuance_fingerprint
                    .trim_start_matches("sha256:")
            ),
            FingerprintV1::parse(&authorization.issuance_fingerprint)?,
        )
    }
}
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct LocalEvaluationHistoricalDependencyObservationV3 {
    pub owner: String,
    pub path: String,
    pub status: String,
    pub reason: String,
}
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct LocalEvaluationHistoryV3 {
    pub result: LocalEvaluationResultV3,
    pub dependencies: Vec<LocalEvaluationHistoricalDependencyObservationV3>,
    pub exact_replay_available: bool,
}

pub(crate) fn identified<T: Serialize>(
    domain: &str,
    value: &T,
) -> Result<ContentAddressedBindingV1, ContractError> {
    let fp = canonical_fingerprint(&(domain, value))?;
    ContentAddressedBindingV1::new(
        &format!("{domain}#{}", fp.as_str().trim_start_matches("sha256:")),
        fp,
    )
}
pub(crate) fn without_identity<T: Serialize>(value: &T) -> Result<Value, ContractError> {
    let mut value = serde_json::to_value(value).map_err(|_| ContractError::InvalidInvariant)?;
    value
        .as_object_mut()
        .ok_or(ContractError::InvalidInvariant)?
        .remove("identity");
    Ok(value)
}

/// Pure truth derives only from an already closed context. No repository,
/// current authorization or clock is consulted here.
pub(crate) fn evaluate_closed_local_context_v3(
    state: &LocalEvaluationSnapshotV3,
    set: &LocalEvaluationEvidenceSetV3,
    now: &SemanticTimeV1,
) -> Result<(LocalEvaluationClaimResultV3, LocalEvaluationGateV3), ContractError> {
    if state.requirements.claims.len() != 1
        || !state.requirements.metrics.is_empty()
        || state.requirements.dependencies.evaluation_gate != local_evaluation_gate_binding_v3()?
    {
        return Err(ContractError::InvalidInvariant);
    }
    let claim = &state.requirements.claims[0];
    let subject = MatcherInputCandidateV1::current(
        ApplicabilityInputSourceV1::OwnerRegistry,
        MatcherBindingV1::exact_fields_v1().input_schema().clone(),
        state
            .requirements
            .subject
            .matcher_inputs()
            .iter()
            .map(|v| MatcherValueV1::new(v.field().clone(), v.value()))
            .collect::<Result<Vec<_>, _>>()?,
    )?;
    let case = MatcherInputCandidateV1::current(
        ApplicabilityInputSourceV1::OwnerRegistry,
        MatcherBindingV1::exact_fields_v1().input_schema().clone(),
        state
            .requirements
            .case
            .matcher_inputs()
            .iter()
            .map(|v| MatcherValueV1::new(v.field().clone(), v.value()))
            .collect::<Result<Vec<_>, _>>()?,
    )?;
    let applicability = evaluate_applicability(claim, &subject, &case);
    let mut output = LocalEvaluationClaimResultV3 {
        claim_id: claim.claim_id().clone(),
        applicability,
        finding: "unresolved".into(),
        coverage: "insufficient".into(),
        currentness: "unknown".into(),
        consistency: "consistent".into(),
        uncertainty: vec![],
        violations: vec![],
        supporting_evidence: set.members.clone(),
    };
    if applicability == ApplicabilityOutcomeV1::NotApplicable {
        output.finding = "not_assessed".into();
        return Ok((output, LocalEvaluationGateV3::NotApplicable));
    }
    if applicability == ApplicabilityOutcomeV1::Blocked {
        output.uncertainty.push("applicability_unresolved".into());
        return Ok((output, LocalEvaluationGateV3::Indeterminate));
    }
    if set.members.len() != 1 {
        output
            .uncertainty
            .push("required_observation_absent".into());
        return Ok((output, LocalEvaluationGateV3::Indeterminate));
    }
    let record = state
        .evidence
        .iter()
        .find(|r| r.identity == set.members[0])
        .ok_or(ContractError::InvalidInvariant)?;
    let body = record.candidate.admission_body();
    output.coverage = if record.coverage_sufficient {
        "sufficient"
    } else {
        "insufficient"
    }
    .into();
    let max_age = state
        .definition
        .dependencies()
        .freshness_policies
        .iter()
        .find(|p| p.exact_binding() == *claim.freshness_policy())
        .ok_or(ContractError::InvalidInvariant)?
        .max_observation_age_seconds();
    let age = semantic_seconds(now)? - semantic_seconds(body.observed_at())?;
    output.currentness = if age >= 0 && age <= i128::from(max_age) {
        "current"
    } else {
        "stale"
    }
    .into();
    let serialized =
        serde_json::to_value(&record.candidate).map_err(|_| ContractError::InvalidInvariant)?;
    output.violations = serialized["body"]["fact_payload"]["violations"]
        .as_array()
        .cloned()
        .unwrap_or_default();
    output.finding = match body.observation() {
        EvidenceObservationV1::Satisfied => "satisfied",
        EvidenceObservationV1::Violated => "unsatisfied",
    }
    .into();
    if output.currentness != "current" {
        output
            .uncertainty
            .push("stale_or_unresolved_assessment".into());
    }
    if !record.coverage_sufficient {
        output
            .uncertainty
            .push("insufficient_effective_resolution".into());
    }
    let gate = reduce_claim_gate_v3(&output);
    Ok((output, gate))
}
fn reduce_claim_gate_v3(claim: &LocalEvaluationClaimResultV3) -> LocalEvaluationGateV3 {
    if claim.applicability == ApplicabilityOutcomeV1::NotApplicable {
        return LocalEvaluationGateV3::NotApplicable;
    }
    if claim.applicability != ApplicabilityOutcomeV1::Applicable
        || claim.coverage != "sufficient"
        || claim.currentness != "current"
        || claim.consistency != "consistent"
        || !claim.uncertainty.is_empty()
    {
        return LocalEvaluationGateV3::Indeterminate;
    }
    match claim.finding.as_str() {
        "satisfied" => LocalEvaluationGateV3::Pass,
        "unsatisfied" => LocalEvaluationGateV3::Fail,
        _ => LocalEvaluationGateV3::Indeterminate,
    }
}
fn semantic_seconds(time: &SemanticTimeV1) -> Result<i128, ContractError> {
    // Civil UTC is already validated. Chronological day arithmetic does not
    // assume a fixed month length or parse current host timezone.
    let s = time.as_str();
    let number = |a: usize, b: usize| {
        s[a..b]
            .parse::<i128>()
            .map_err(|_| ContractError::InvalidInvariant)
    };
    let year = number(0, 4)?;
    let month = number(5, 7)?;
    let day = number(8, 10)?;
    let y = year - i128::from(month <= 2);
    let era = y.div_euclid(400);
    let yoe = y - era * 400;
    let mp = month + if month > 2 { -3 } else { 9 };
    let doy = (153 * mp + 2) / 5 + day - 1;
    let days = era * 146097 + yoe * 365 + yoe / 4 - yoe / 100 + doy;
    Ok(days * 86400 + number(11, 13)? * 3600 + number(14, 16)? * 60 + number(17, 19)?)
}

#[cfg(test)]
mod tests {
    use super::*;
    fn claim(finding: &str) -> LocalEvaluationClaimResultV3 {
        LocalEvaluationClaimResultV3 {
            claim_id: StableIdV1::parse("schema.conformant").unwrap(),
            applicability: ApplicabilityOutcomeV1::Applicable,
            finding: finding.into(),
            coverage: "sufficient".into(),
            currentness: "current".into(),
            consistency: "consistent".into(),
            uncertainty: vec![],
            violations: vec![],
            supporting_evidence: vec![],
        }
    }
    #[test]
    fn gate_preserves_independent_dimensions_and_positive_non_applicability() {
        let satisfied = claim("satisfied");
        assert_eq!(
            reduce_claim_gate_v3(&satisfied),
            LocalEvaluationGateV3::Pass
        );
        let violated = claim("unsatisfied");
        assert_eq!(reduce_claim_gate_v3(&violated), LocalEvaluationGateV3::Fail);
        for field in ["coverage", "currentness", "consistency", "uncertainty"] {
            let mut unresolved = satisfied.clone();
            match field {
                "coverage" => unresolved.coverage = "insufficient".into(),
                "currentness" => unresolved.currentness = "stale".into(),
                "consistency" => unresolved.consistency = "conflicting".into(),
                _ => unresolved.uncertainty.push("unknown".into()),
            };
            assert_eq!(
                reduce_claim_gate_v3(&unresolved),
                LocalEvaluationGateV3::Indeterminate,
                "{field}"
            );
            assert_eq!(unresolved.finding, "satisfied");
        }
        let mut non_applicable = claim("unresolved");
        non_applicable.applicability = ApplicabilityOutcomeV1::NotApplicable;
        non_applicable.coverage = "insufficient".into();
        assert_eq!(
            reduce_claim_gate_v3(&non_applicable),
            LocalEvaluationGateV3::NotApplicable
        );
        non_applicable.applicability = ApplicabilityOutcomeV1::Blocked;
        assert_eq!(
            reduce_claim_gate_v3(&non_applicable),
            LocalEvaluationGateV3::Indeterminate
        );
        let mut stale_violation = violated;
        stale_violation.currentness = "stale".into();
        assert_eq!(
            reduce_claim_gate_v3(&stale_violation),
            LocalEvaluationGateV3::Indeterminate
        );
        assert_eq!(stale_violation.finding, "unsatisfied");
    }
    #[test]
    fn freshness_age_uses_civil_utc_across_leap_day_year_and_month_boundaries() {
        for (a, b, seconds) in [
            ("2024-02-28T23:59:59Z", "2024-03-01T00:00:00Z", 86401),
            ("2025-02-28T23:59:59Z", "2025-03-01T00:00:00Z", 1),
            ("2025-12-31T23:59:59Z", "2026-01-01T00:00:00Z", 1),
        ] {
            assert_eq!(
                semantic_seconds(&SemanticTimeV1::parse(b).unwrap()).unwrap()
                    - semantic_seconds(&SemanticTimeV1::parse(a).unwrap()).unwrap(),
                seconds
            );
        }
    }
}
