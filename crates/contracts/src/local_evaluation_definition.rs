//! Exact full-check revisions. The intrinsic v2 definition is retained as an
//! explicit dependency; the new policies and distinct review bind a new C.
use crate::canonical::{canonical_bytes, canonical_fingerprint};
use crate::local_contract::resolve_local_applicability_v2;
use crate::project_admission_governance::ProjectAdmissionBindingV3;
use crate::*;
use handbook_engine::local_authority::local_check::LocalCheckCurrentBasisV1;
use handbook_engine::ContextResolutionStackDefinition;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::path::Path;

pub const LOCAL_DEFINITION_SCHEMA_REF_V3: &str = "handbook.contract-definition.local@3.0.0";

pub fn local_evaluation_rule_binding_v3() -> Result<ExactBindingV1, ContractError> {
    Ok(ExactBindingV1::new(
        ExactRefV1::parse("handbook.contract-local-evaluation-rule@3.0.0")?,
        canonical_fingerprint(&json!({"source":"direct_contract","claims":1,"cases":1,
            "roles":["schema_conformance"],"metrics":[],"attempts":{"max":1,"retry":false,"partial":false},
            "formation":"all_exact_matching_admissions_once_complete_terminal_accounting",
            "checkpoint":"current_permission_before_commit_launch_append_and_after_set_before_truth",
            "time":"owner_utc_at_each_semantic_boundary_exact_sources_zero_future_skew",
            "history":"retained_exact_dependencies_no_current_substitution_no_producer_execution"}))?,
    ))
}
pub fn local_evaluation_gate_binding_v3() -> Result<ExactBindingV1, ContractError> {
    Ok(ExactBindingV1::new(
        ExactRefV1::parse("handbook.contract-local-evaluation-gate@3.0.0")?,
        canonical_fingerprint(
            &json!({"algorithm":"one_noncompensatory_claim","metrics":[],
            "pass":"applicable_satisfied_current_sufficient_consistent",
            "fail":"established_unsatisfied","indeterminate":"valid_insufficient_stale_unknown_unresolved",
            "not_applicable":"positively_non_applicable","invalid_closure":"refusal_before_truth"}),
        )?,
    ))
}
pub fn local_evidence_admission_policy_binding_v3() -> Result<ExactBindingV1, ContractError> {
    Ok(ExactBindingV1::new(
        ExactRefV1::parse("handbook.contract-local-evidence-admission@3.0.0")?,
        canonical_fingerprint(
            &json!({"origin":"native_retained_complete","basis":"one_committed_exact",
            "source":"complete_pinned_request_input_bytes","authority":"engine_current_sealed",
            "admission_terminals":"admitted_refused_duplicate","membership":"non_discretionary"}),
        )?,
    ))
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct LocalDefinitionDependenciesV3 {
    pub intrinsic: LocalDefinitionDependenciesV2,
    pub evaluation_rule: ExactBindingV1,
    pub evaluation_gate: ExactBindingV1,
    pub formation_policy: ExactBindingV1,
    pub evidence_admission_policy: ExactBindingV1,
    pub time_rules: ExactBindingV1,
    pub metrics: Vec<ExactBindingV1>,
}
impl LocalDefinitionDependenciesV3 {
    pub fn new(intrinsic: LocalDefinitionDependenciesV2) -> Result<Self, ContractError> {
        let rule = local_evaluation_rule_binding_v3()?;
        Ok(Self {
            intrinsic,
            evaluation_rule: rule.clone(),
            evaluation_gate: local_evaluation_gate_binding_v3()?,
            formation_policy: rule.clone(),
            evidence_admission_policy: local_evidence_admission_policy_binding_v3()?,
            time_rules: rule,
            metrics: vec![],
        })
    }
    pub(crate) fn validate(&self) -> Result<(), ContractError> {
        if self.evaluation_rule != local_evaluation_rule_binding_v3()?
            || self.evaluation_gate != local_evaluation_gate_binding_v3()?
            || self.formation_policy != self.evaluation_rule
            || self.time_rules != self.evaluation_rule
            || self.evidence_admission_policy != local_evidence_admission_policy_binding_v3()?
            || !self.metrics.is_empty()
        {
            return Err(ContractError::InvalidInvariant);
        }
        Ok(())
    }
}
#[derive(Clone, Debug)]
pub struct LocalContractDefinitionInputV3 {
    pub intrinsic_definition: LocalContractDefinitionV2,
    pub dependencies: LocalDefinitionDependenciesV3,
}
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct LocalContractDefinitionV3 {
    schema_ref: String,
    intrinsic_definition: LocalContractDefinitionV2,
    dependencies: LocalDefinitionDependenciesV3,
    closure_fingerprint: FingerprintV1,
    definition_fingerprint: FingerprintV1,
}
impl LocalContractDefinitionV3 {
    pub fn new(
        input: LocalContractDefinitionInputV3,
        stack: &ContextResolutionStackDefinition,
    ) -> Result<Self, ContractError> {
        input.intrinsic_definition.validate_resolution(stack)?;
        input.intrinsic_definition.canonical_json_bytes()?;
        input.dependencies.validate()?;
        if input.dependencies.intrinsic != *input.intrinsic_definition.dependencies() {
            return Err(ContractError::InvalidInvariant);
        }
        let closure_fingerprint = canonical_fingerprint(&(
            "handbook.local-definition-closure@3.0.0",
            &input.intrinsic_definition,
            &input.dependencies,
        ))?;
        let definition_fingerprint = canonical_fingerprint(&(
            LOCAL_DEFINITION_SCHEMA_REF_V3,
            &input.intrinsic_definition,
            &input.dependencies,
            &closure_fingerprint,
        ))?;
        let value = Self {
            schema_ref: LOCAL_DEFINITION_SCHEMA_REF_V3.into(),
            intrinsic_definition: input.intrinsic_definition,
            dependencies: input.dependencies,
            closure_fingerprint,
            definition_fingerprint,
        };
        value.validate()?;
        Ok(value)
    }
    pub(crate) fn validate(&self) -> Result<(), ContractError> {
        self.intrinsic_definition.canonical_json_bytes()?;
        self.dependencies.validate()?;
        let claims = self.intrinsic_definition.semantic_definition().claims();
        if claims.len() != 1
            || claims[0].gate_effect() != GateEffectV1::Required
            || claims[0].score_weight().is_some()
            || claims[0].evidence_requirements().len() != 1
            || claims[0].evidence_requirements()[0].cardinality()
                != EvidenceCardinalityV1::ExactlyOne
        {
            return Err(ContractError::InvalidInvariant);
        }
        if self
            .dependencies
            .intrinsic
            .schema_closure
            .documents
            .iter()
            .any(|d| {
                d.schema.get("$schema").and_then(serde_json::Value::as_str)
                    != Some("https://json-schema.org/draft/2020-12/schema")
            })
        {
            return Err(ContractError::InvalidInvariant);
        }
        if self.schema_ref != LOCAL_DEFINITION_SCHEMA_REF_V3
            || self.dependencies.intrinsic != *self.intrinsic_definition.dependencies()
            || self.closure_fingerprint
                != canonical_fingerprint(&(
                    "handbook.local-definition-closure@3.0.0",
                    &self.intrinsic_definition,
                    &self.dependencies,
                ))?
            || self.definition_fingerprint
                != canonical_fingerprint(&(
                    LOCAL_DEFINITION_SCHEMA_REF_V3,
                    &self.intrinsic_definition,
                    &self.dependencies,
                    &self.closure_fingerprint,
                ))?
        {
            return Err(ContractError::DerivedValueMismatch);
        }
        Ok(())
    }
    pub fn validate_resolution(
        &self,
        stack: &ContextResolutionStackDefinition,
    ) -> Result<(), ContractError> {
        self.validate()?;
        self.intrinsic_definition.validate_resolution(stack)
    }
    pub fn binding(&self) -> ExactBindingV1 {
        ExactBindingV1::new(
            self.intrinsic_definition.binding().reference().clone(),
            self.definition_fingerprint.clone(),
        )
    }
    pub fn semantic_definition(&self) -> &ContractDefinitionV1 {
        self.intrinsic_definition.semantic_definition()
    }
    pub fn permitted_use(&self) -> LocalPermittedUseV2 {
        self.intrinsic_definition.permitted_use()
    }
    /// Intrinsic dependencies retain their original identities and semantics.
    pub fn dependencies(&self) -> &LocalDefinitionDependenciesV2 {
        &self.dependencies.intrinsic
    }
    pub fn full_dependencies(&self) -> &LocalDefinitionDependenciesV3 {
        &self.dependencies
    }
    pub fn closure_fingerprint(&self) -> &FingerprintV1 {
        &self.closure_fingerprint
    }
    pub fn canonical_json_bytes(&self) -> Result<Vec<u8>, ContractError> {
        self.validate()?;
        canonical_bytes(self)
    }
    pub fn from_json_bytes(bytes: &[u8]) -> Result<Self, ContractError> {
        if bytes.len() > 1024 * 1024 {
            return Err(ContractError::BoundExceeded);
        }
        let result: Self =
            serde_json::from_slice(bytes).map_err(|_| ContractError::InvalidInvariant)?;
        if result.canonical_json_bytes()? != bytes {
            return Err(ContractError::DerivedValueMismatch);
        }
        Ok(result)
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct LocalReviewInputV3 {
    pub definition: ExactBindingV1,
    pub closure_fingerprint: FingerprintV1,
    pub permitted_use: LocalPermittedUseV2,
    pub author_participant_ref: String,
    pub reviewer_participant_ref: String,
    pub reviewer_actor_ref: String,
    pub decision: LocalReviewDecisionV2,
    pub source_ref: String,
    pub outcome_text: String,
    pub semantic_time: SemanticTimeV1,
}
impl LocalReviewInputV3 {
    pub(crate) fn validate(
        &self,
        definition: &LocalContractDefinitionV3,
        author: &str,
    ) -> Result<(), ContractError> {
        if self.definition != definition.binding()
            || self.closure_fingerprint != *definition.closure_fingerprint()
            || self.permitted_use != definition.permitted_use()
            || self.author_participant_ref != author
            || self.reviewer_participant_ref == author
            || [
                &self.reviewer_participant_ref,
                &self.reviewer_actor_ref,
                &self.source_ref,
                &self.outcome_text,
            ]
            .iter()
            .any(|s| s.trim().is_empty() || s.len() > 4096 || s.contains('\0'))
        {
            return Err(ContractError::InvalidInvariant);
        }
        Ok(())
    }
}
pub(crate) fn resolve_local_applicability_v3(
    repo: &Path,
    project: &ProjectAdmissionBindingV3,
    definition: &LocalContractDefinitionV3,
) -> Result<(LocalContractApplicabilityV2, LocalCheckCurrentBasisV1), ContractError> {
    definition.validate()?;
    let (mut result, current) =
        resolve_local_applicability_v2(repo, project, &definition.intrinsic_definition)?;
    result
        .dependencies
        .iter_mut()
        .filter(|d| d.kind == "contract_definition")
        .for_each(|d| d.binding = definition.binding());
    let qualification = if matches!(
        result.governance_basis,
        crate::project_admission_governance::ProjectAdmissionScopeBasisV3::DeferredV2(_)
    ) {
        LocalDependencyQualificationV2::DeferredIntrinsic
    } else {
        LocalDependencyQualificationV2::NotApplicableToFreshnessScope
    };
    for (kind, binding) in [
        ("evaluation_rule", &definition.dependencies.evaluation_rule),
        ("evaluation_gate", &definition.dependencies.evaluation_gate),
        (
            "formation_policy",
            &definition.dependencies.formation_policy,
        ),
        ("time_rules", &definition.dependencies.time_rules),
        (
            "evaluation_admission_policy",
            &definition.dependencies.evidence_admission_policy,
        ),
    ] {
        result.dependencies.push(LocalDependencyApplicabilityV2 {
            kind: kind.into(),
            binding: binding.clone(),
            qualification: qualification.clone(),
        });
    }
    result.policy_admission_closure_fingerprint = canonical_fingerprint(&(
        "handbook.local-policy-admission-closure@3.0.0",
        &result.governance_basis,
        &result.owner_rule,
        &result.dependencies,
        &current,
    ))?;
    Ok((result, current))
}
#[derive(Clone, Debug)]
pub struct LocalContractUseV3 {
    pub(crate) definition: LocalContractDefinitionV3,
    pub(crate) applicability: LocalContractApplicabilityV2,
    pub(crate) current_basis: LocalCheckCurrentBasisV1,
    pub(crate) lifecycle_head_fingerprint: FingerprintV1,
    pub(crate) intent: LocalContractUseIntentV2,
    pub(crate) required_state_paths: Vec<String>,
}
impl LocalContractUseV3 {
    pub fn definition(&self) -> &LocalContractDefinitionV3 {
        &self.definition
    }
    pub fn closure_fingerprint(&self) -> &FingerprintV1 {
        self.definition.closure_fingerprint()
    }
    pub fn current_basis(&self) -> &LocalCheckCurrentBasisV1 {
        &self.current_basis
    }
    pub fn lifecycle_head_fingerprint(&self) -> &FingerprintV1 {
        &self.lifecycle_head_fingerprint
    }
    pub fn applicability(&self) -> &LocalContractApplicabilityV2 {
        &self.applicability
    }
    pub fn intent(&self) -> LocalContractUseIntentV2 {
        self.intent
    }
    pub fn required_state_paths(&self) -> &[String] {
        &self.required_state_paths
    }
}
