//! Additive local definitions. V1 content remains an inert semantic component;
//! permitted use, complete dependencies and local lifecycle have their own identity.
use crate::canonical::{canonical_bytes, canonical_fingerprint};
use crate::project_admission_governance::*;
use crate::*;
use handbook_engine::local_authority::local_check::{
    LocalCheckCurrentBasisV1, NativeLocalCheckAuthorityV1,
};
use handbook_engine::{
    ContextResolutionStackDefinition, RepositoryReadV1, RetainedRepositoryRootV1,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::BTreeSet;
use std::path::Path;

pub const LOCAL_DEFINITION_SCHEMA_REF_V2: &str = "handbook.contract-definition.local@2.0.0";
pub const LOCAL_LIFECYCLE_POLICY_REF_V2: &str =
    "handbook.contract-lifecycle-policy.local-review@2.0.0";
pub const LOCAL_EVIDENCE_ADMISSION_POLICY_REF_V2: &str =
    "handbook.contract-evidence-admission-policy.local-advisory@2.0.0";

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum LocalPermittedUseV2 {
    DiagnosticOnly,
    DecisionSeekingEligible,
}
#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum LocalContractUseIntentV2 {
    Diagnostic,
    DecisionSeeking,
}
impl LocalPermittedUseV2 {
    pub fn permits(self, intent: LocalContractUseIntentV2) -> bool {
        self == Self::DecisionSeekingEligible || intent == LocalContractUseIntentV2::Diagnostic
    }
}
#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum LocalReviewDecisionV2 {
    Approve,
    Reject,
}

pub fn local_lifecycle_policy_binding_v2() -> Result<ExactBindingV1, ContractError> {
    fixed_local_binding(
        LOCAL_LIFECYCLE_POLICY_REF_V2,
        serde_json::json!({
            "schema_ref": LOCAL_LIFECYCLE_POLICY_REF_V2,
            "states": ["draft", "review_ready", "locked", "active", "deprecated", "closed"],
            "review": "actual_exact_closure_review_by_selected_distinct_participant",
            "authority": "sealed_local_advisory_exact_action",
            "currentness": "exact_expected_head_and_live_native_authority_through_commit"
        }),
    )
}
pub fn local_evidence_admission_policy_binding_v2() -> Result<ExactBindingV1, ContractError> {
    fixed_local_binding(
        LOCAL_EVIDENCE_ADMISSION_POLICY_REF_V2,
        serde_json::json!({
            "schema_ref": LOCAL_EVIDENCE_ADMISSION_POLICY_REF_V2,
            "candidate_trust": "untrusted_completed_native_dock_candidate",
            "authority": "sealed_local_advisory_exact_source",
            "source_currentness": "fresh_native_authority_applicability_and_captured_sources_each_callback",
            "credential_downgrade": false,
            "evidence_retention": "immutable_all"
        }),
    )
}
fn fixed_local_binding(reference: &str, content: Value) -> Result<ExactBindingV1, ContractError> {
    Ok(ExactBindingV1::new(
        ExactRefV1::parse(reference)?,
        canonical_fingerprint(&content)?,
    ))
}
pub fn local_extensions_policy_binding_v2() -> Result<ExactBindingV1, ContractError> {
    fixed_local_binding(
        "handbook.contract-extensions.closed@2.0.0",
        serde_json::json!({"schema_ref":"handbook.contract-extensions.closed@2.0.0","extensions":"refuse","metrics":"none_selected"}),
    )
}

/// Immutable JSON content with its exact canonical source-byte identity. A
/// declared final text newline remains part of that source identity.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct LocalExactContentV2 {
    pub binding: ExactBindingV1,
    pub content: Value,
    pub source_bytes: Vec<u8>,
}
impl LocalExactContentV2 {
    pub fn new(reference: ExactRefV1, content: Value) -> Result<Self, ContractError> {
        let source_bytes = canonical_bytes(&content)?;
        Ok(Self {
            binding: ExactBindingV1::new(reference, FingerprintV1::from_bytes(&source_bytes)),
            content,
            source_bytes,
        })
    }
    pub fn from_source_bytes(reference: ExactRefV1, bytes: &[u8]) -> Result<Self, ContractError> {
        let content: Value =
            serde_json::from_slice(bytes).map_err(|_| ContractError::InvalidInvariant)?;
        let value = Self {
            binding: ExactBindingV1::new(reference, FingerprintV1::from_bytes(bytes)),
            content,
            source_bytes: bytes.to_vec(),
        };
        value.validate()?;
        Ok(value)
    }
    fn validate(&self) -> Result<(), ContractError> {
        let canonical = canonical_bytes(&self.content)?;
        let mut with_lf = canonical.clone();
        with_lf.push(b'\n');
        if FingerprintV1::from_bytes(&self.source_bytes) != *self.binding.fingerprint()
            || (self.source_bytes != canonical && self.source_bytes != with_lf)
            || self.source_bytes.len() > 256 * 1024
        {
            return Err(ContractError::InvalidInvariant);
        }
        Ok(())
    }
}
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct LocalSchemaDocumentV2 {
    pub path: String,
    pub schema: Value,
    pub document_fingerprint: FingerprintV1,
}
impl LocalSchemaDocumentV2 {
    pub fn new(path: String, schema: Value) -> Result<Self, ContractError> {
        let document_fingerprint = canonical_fingerprint(&schema)?;
        let value = Self {
            path,
            schema,
            document_fingerprint,
        };
        value.validate()?;
        Ok(value)
    }
    fn validate(&self) -> Result<(), ContractError> {
        // Match the selected validator's existing logical-path representation.
        // Schema keyword support is decided by its native execution owner.
        let components = self.path.split('/').collect::<Vec<_>>();
        if self.path.len() > 1024
            || components.len() > 64
            || !self.path.starts_with("definitions/schemas/")
            || self.path.contains(['\\', ':', '\0'])
            || components.iter().any(|part| {
                part.is_empty()
                    || *part == "."
                    || *part == ".."
                    || part.trim() != *part
                    || part
                        .chars()
                        .any(|c| c.is_control() || matches!(c, '<' | '>' | '"' | '|' | '?' | '*'))
            })
            || !self.schema.is_object()
            || canonical_fingerprint(&self.schema)? != self.document_fingerprint
        {
            return Err(ContractError::InvalidInvariant);
        }
        Ok(())
    }
}
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct LocalSchemaClosureV2 {
    pub root_schema: ExactBindingV1,
    pub root_document: String,
    pub documents: Vec<LocalSchemaDocumentV2>,
    pub closure_fingerprint: FingerprintV1,
}
impl LocalSchemaClosureV2 {
    pub fn new(
        root_schema: ExactBindingV1,
        root_document: String,
        documents: Vec<LocalSchemaDocumentV2>,
    ) -> Result<Self, ContractError> {
        let closure_fingerprint = schema_closure_fingerprint(&documents)?;
        let value = Self {
            root_schema,
            root_document,
            documents,
            closure_fingerprint,
        };
        value.validate()?;
        Ok(value)
    }
    fn validate(&self) -> Result<(), ContractError> {
        if self.documents.is_empty()
            || self.documents.len() > 128
            || self.documents.windows(2).any(|p| p[0].path >= p[1].path)
        {
            return Err(ContractError::InvalidInvariant);
        }
        for d in &self.documents {
            d.validate()?;
        }
        let root = self
            .documents
            .iter()
            .find(|d| d.path == self.root_document)
            .ok_or(ContractError::InvalidInvariant)?;
        if self.root_schema.fingerprint() != &root.document_fingerprint
            || self.closure_fingerprint != schema_closure_fingerprint(&self.documents)?
            || canonical_bytes(self)?.len() > 1024 * 1024
        {
            return Err(ContractError::InvalidInvariant);
        }
        // The supported offline closure admits local fragment/document refs only.
        for d in &self.documents {
            validate_schema_refs(&d.schema, &d.path, &self.documents)?;
        }
        Ok(())
    }
}
fn schema_closure_fingerprint(
    documents: &[LocalSchemaDocumentV2],
) -> Result<FingerprintV1, ContractError> {
    #[derive(Serialize)]
    struct Member<'a> {
        path: &'a str,
        document_fingerprint: &'a FingerprintV1,
    }
    canonical_fingerprint(
        &documents
            .iter()
            .map(|d| Member {
                path: &d.path,
                document_fingerprint: &d.document_fingerprint,
            })
            .collect::<Vec<_>>(),
    )
}
fn validate_schema_refs(
    value: &Value,
    current: &str,
    documents: &[LocalSchemaDocumentV2],
) -> Result<(), ContractError> {
    match value {
        Value::Object(o) => {
            if let Some(reference) = o.get("$ref") {
                let reference = reference.as_str().ok_or(ContractError::InvalidInvariant)?;
                if reference.contains(['?', '\\', '%'])
                    || reference.starts_with('/')
                    || reference.matches('#').count() > 1
                {
                    return Err(ContractError::InvalidInvariant);
                }
                let (name, pointer) = reference.split_once('#').unwrap_or((reference, ""));
                if name.contains(':')
                    || (!name.is_empty()
                        && name
                            .split('/')
                            .any(|p| p.is_empty() || p == "." || p == ".."))
                    || pointer.len() > 1024
                    || (!pointer.is_empty() && !pointer.starts_with('/'))
                {
                    return Err(ContractError::InvalidInvariant);
                }
                let mut pointer_bytes = pointer.bytes();
                while let Some(byte) = pointer_bytes.next() {
                    if byte == b'~' && !matches!(pointer_bytes.next(), Some(b'0' | b'1')) {
                        return Err(ContractError::InvalidInvariant);
                    }
                }
                let target = if name.is_empty() {
                    current.to_owned()
                } else {
                    let parent = current.rsplit_once('/').map(|p| p.0).unwrap_or("");
                    format!("{parent}/{name}")
                };
                let document = documents
                    .iter()
                    .find(|d| d.path == target)
                    .ok_or(ContractError::InvalidInvariant)?;
                if document.schema.pointer(pointer).is_none() {
                    return Err(ContractError::InvalidInvariant);
                }
            }
            for child in o.values() {
                validate_schema_refs(child, current, documents)?;
            }
        }
        Value::Array(a) => {
            for child in a {
                validate_schema_refs(child, current, documents)?;
            }
        }
        _ => {}
    }
    Ok(())
}

/// Exact operational definition content; native Dock resolution checks it again
/// before source use. This content registration makes no readiness claim.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct LocalValidatorDependencyV2 {
    pub manifest: ExactBindingV1,
    pub canonical_manifest: Value,
}
impl LocalValidatorDependencyV2 {
    pub fn from_manifest_json(bytes: &[u8]) -> Result<Self, ContractError> {
        if bytes.len() > 1024 * 1024 {
            return Err(ContractError::BoundExceeded);
        }
        let canonical_manifest: Value =
            serde_json::from_slice(bytes).map_err(|_| ContractError::InvalidInvariant)?;
        if canonical_bytes(&canonical_manifest)? != bytes {
            return Err(ContractError::DerivedValueMismatch);
        }
        let manifest = ExactBindingV1::new(
            ExactRefV1::parse(
                canonical_manifest
                    .get("dock_ref")
                    .and_then(Value::as_str)
                    .ok_or(ContractError::InvalidInvariant)?,
            )?,
            FingerprintV1::parse(
                canonical_manifest
                    .get("manifest_fingerprint")
                    .and_then(Value::as_str)
                    .ok_or(ContractError::InvalidInvariant)?,
            )?,
        );
        let value = Self {
            manifest,
            canonical_manifest,
        };
        value.validate()?;
        Ok(value)
    }
    fn validate(&self) -> Result<(), ContractError> {
        // Reuse the same immutable, checked-in native descriptor selected by
        // Dock's production registry. No runtime Dock permission is inferred.
        let shipped_manifest: Value = serde_json::from_slice(include_bytes!(
            "../../dock/production-manifests/x86_64-pc-windows-msvc/manifest.json"
        ))
        .map_err(|_| ContractError::InvalidInvariant)?;
        if self.manifest.reference().as_str() != "handbook.dock.json-schema@1.0.0"
            || self.canonical_manifest != shipped_manifest
        {
            return Err(ContractError::InvalidInvariant);
        }
        let mut preimage = self.canonical_manifest.clone();
        let object = preimage
            .as_object_mut()
            .ok_or(ContractError::InvalidInvariant)?;
        if object
            .remove("manifest_fingerprint")
            .and_then(|v| v.as_str().map(str::to_owned))
            .as_deref()
            != Some(self.manifest.fingerprint().as_str())
            || object.get("dock_ref").and_then(Value::as_str)
                != Some(self.manifest.reference().as_str())
            || ["offline", "read_only", "deterministic"]
                .iter()
                .any(|key| object.get(*key).and_then(Value::as_bool) != Some(true))
            || canonical_fingerprint(&preimage)? != *self.manifest.fingerprint()
        {
            return Err(ContractError::DerivedValueMismatch);
        }
        Ok(())
    }
}
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct LocalDefinitionDependenciesV2 {
    pub definition_schema: LocalExactContentV2,
    pub extensions_policy: ExactBindingV1,
    pub freshness_policies: Vec<EvidenceFreshnessPolicyV1>,
    pub gate_policy: GatePolicyV1,
    pub stability_policy: ExactBindingV1,
    pub evidence_admission_policy: ExactBindingV1,
    pub lifecycle_policy: ExactBindingV1,
    pub fact_schema: LocalExactContentV2,
    pub validator: LocalValidatorDependencyV2,
    pub schema_closure: LocalSchemaClosureV2,
    pub required_governance_scopes: Vec<String>,
}
impl LocalDefinitionDependenciesV2 {
    fn validate(&self, definition: &ContractDefinitionV1) -> Result<(), ContractError> {
        self.definition_schema.validate()?;
        if self.definition_schema.binding != *definition.definition_schema_binding()
            || self.definition_schema.binding.reference().as_str()
                != "handbook.contract-definition@1.0.0"
            || self.definition_schema.binding.fingerprint().as_str()
                != "sha256:506659a20adb4c6171dd6e44bf1c4d50bf82a3b8118feeff9c81136316c185cc"
            || self.extensions_policy != local_extensions_policy_binding_v2()?
            || definition.extensions_policy_binding() != &self.extensions_policy
            || definition.lifecycle_policy() != &self.lifecycle_policy
        {
            return Err(ContractError::InvalidInvariant);
        }
        if self.freshness_policies.is_empty()
            || self.freshness_policies.len() > 16
            || self
                .freshness_policies
                .windows(2)
                .any(|p| p[0].policy_ref() >= p[1].policy_ref())
            || self.stability_policy != evidence_stability_policy_binding_v1()?
            || self.evidence_admission_policy != local_evidence_admission_policy_binding_v2()?
            || self.lifecycle_policy != local_lifecycle_policy_binding_v2()?
            || self.gate_policy.binding() != definition.gate_policy()
            || self.gate_policy.local_closeout_policy() != &local_closeout_policy_binding_v1()?
        {
            return Err(ContractError::InvalidInvariant);
        }
        self.fact_schema.validate()?;
        self.validator.validate()?;
        self.schema_closure.validate()?;
        if self.fact_schema.binding.reference().as_str() != "handbook.fact.schema-conformance@1.0.0"
            || self.fact_schema.binding.fingerprint().as_str()
                != "sha256:c8a0f54ae4e504dd6a0921ec57b6736ea9ec918448969bc931782d5c1969a6f6"
        {
            return Err(ContractError::InvalidInvariant);
        }
        let closure = definition.evaluation_closure();
        if self
            .freshness_policies
            .iter()
            .map(EvidenceFreshnessPolicyV1::exact_binding)
            .collect::<Vec<_>>()
            != closure.freshness_policies
            || closure.stability_policies != vec![self.stability_policy.clone()]
            || closure.matcher_bindings != vec![MatcherBindingV1::exact_fields_v1()]
            || definition.claims().len() != 1
            || definition
                .claims()
                .iter()
                .any(|c| c.score_weight().is_some() || c.evidence_requirements().len() != 1)
        {
            return Err(ContractError::InvalidInvariant);
        }
        let claim = &definition.claims()[0];
        if claim.evidence_requirements()[0].evidence_kind().as_str()
            != "handbook.evidence.schema-conformance"
            || claim.evidence_requirements()[0].cardinality() != EvidenceCardinalityV1::ExactlyOne
        {
            return Err(ContractError::InvalidInvariant);
        }
        if self.required_governance_scopes.len() > 16
            || self
                .required_governance_scopes
                .windows(2)
                .any(|p| p[0] >= p[1])
            || self
                .required_governance_scopes
                .iter()
                .any(|s| s.is_empty() || s.len() > 256)
        {
            return Err(ContractError::InvalidInvariant);
        }
        Ok(())
    }
    pub fn policy_registry(&self) -> Result<EvaluationPolicyRegistryV1, ContractError> {
        let mut r = EvaluationPolicyRegistryV1::new()?;
        for p in &self.freshness_policies {
            r.register_freshness(p.clone())?;
        }
        r.register_gate(self.gate_policy.clone())?;
        Ok(r)
    }
}
#[derive(Clone, Debug)]
pub struct LocalContractDefinitionInputV2 {
    pub semantic_definition: ContractDefinitionV1,
    pub permitted_use: LocalPermittedUseV2,
    pub dependencies: LocalDefinitionDependenciesV2,
}
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct LocalContractDefinitionV2 {
    schema_ref: String,
    semantic_definition: ContractDefinitionV1,
    permitted_use: LocalPermittedUseV2,
    dependencies: LocalDefinitionDependenciesV2,
    closure_fingerprint: FingerprintV1,
    definition_fingerprint: FingerprintV1,
    resolution_stack: ExactBindingV1,
}
impl LocalContractDefinitionV2 {
    pub fn new(
        input: LocalContractDefinitionInputV2,
        stack: &ContextResolutionStackDefinition,
    ) -> Result<Self, ContractError> {
        input.dependencies.validate(&input.semantic_definition)?;
        let resolution_stack = ExactBindingV1::new(
            ExactRefV1::parse(stack.exact_ref().as_str())?,
            FingerprintV1::parse(stack.definition_fingerprint().as_str())?,
        );
        let closure_fingerprint = canonical_fingerprint(&(
            "handbook.local-definition-closure@2.0.0",
            &input.semantic_definition,
            &input.dependencies,
            &resolution_stack,
        ))?;
        let definition_fingerprint = canonical_fingerprint(&(
            LOCAL_DEFINITION_SCHEMA_REF_V2,
            &input.semantic_definition,
            input.permitted_use,
            &input.dependencies,
            &resolution_stack,
            &closure_fingerprint,
        ))?;
        let value = Self {
            schema_ref: LOCAL_DEFINITION_SCHEMA_REF_V2.into(),
            semantic_definition: input.semantic_definition,
            permitted_use: input.permitted_use,
            dependencies: input.dependencies,
            closure_fingerprint,
            definition_fingerprint,
            resolution_stack,
        };
        value.validate_resolution(stack)?;
        Ok(value)
    }
    pub fn validate_resolution(
        &self,
        stack: &ContextResolutionStackDefinition,
    ) -> Result<(), ContractError> {
        if self.resolution_stack.reference().as_str() != stack.exact_ref().as_str()
            || self.resolution_stack.fingerprint().as_str()
                != stack.definition_fingerprint().as_str()
        {
            return Err(ContractError::InvalidInvariant);
        }
        for claim in self.semantic_definition.claims() {
            let min = claim.minimum_resolution();
            if min.resolution_stack() != &self.resolution_stack
                || stack
                    .dimension_ranks(
                        crate::evidence_admission::engine_resolution_values(min.dimension_values())
                            .map_err(|_| ContractError::InvalidInvariant)?
                            .each_ref()
                            .map(String::as_str),
                    )
                    .is_none()
            {
                return Err(ContractError::InvalidInvariant);
            }
        }
        Ok(())
    }
    pub(crate) fn validate(&self) -> Result<(), ContractError> {
        self.dependencies.validate(&self.semantic_definition)?;
        if self.schema_ref != LOCAL_DEFINITION_SCHEMA_REF_V2
            || self.closure_fingerprint
                != canonical_fingerprint(&(
                    "handbook.local-definition-closure@2.0.0",
                    &self.semantic_definition,
                    &self.dependencies,
                    &self.resolution_stack,
                ))?
            || self.definition_fingerprint
                != canonical_fingerprint(&(
                    LOCAL_DEFINITION_SCHEMA_REF_V2,
                    &self.semantic_definition,
                    self.permitted_use,
                    &self.dependencies,
                    &self.resolution_stack,
                    &self.closure_fingerprint,
                ))?
        {
            return Err(ContractError::DerivedValueMismatch);
        }
        Ok(())
    }
    pub fn binding(&self) -> ExactBindingV1 {
        ExactBindingV1::new(
            self.semantic_definition.contract_ref().clone(),
            self.definition_fingerprint.clone(),
        )
    }
    pub fn semantic_definition(&self) -> &ContractDefinitionV1 {
        &self.semantic_definition
    }
    pub fn permitted_use(&self) -> LocalPermittedUseV2 {
        self.permitted_use
    }
    pub fn dependencies(&self) -> &LocalDefinitionDependenciesV2 {
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
        let value: Self =
            serde_json::from_slice(bytes).map_err(|_| ContractError::InvalidInvariant)?;
        if value.canonical_json_bytes()? != bytes {
            return Err(ContractError::DerivedValueMismatch);
        }
        Ok(value)
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct LocalReviewInputV2 {
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
impl LocalReviewInputV2 {
    pub(crate) fn validate(
        &self,
        definition: &LocalContractDefinitionV2,
        author_participant: &str,
    ) -> Result<(), ContractError> {
        if self.definition != definition.binding()
            || self.closure_fingerprint != definition.closure_fingerprint
            || self.permitted_use != definition.permitted_use
            || self.author_participant_ref != author_participant
            || self.reviewer_participant_ref == author_participant
            || [
                self.reviewer_participant_ref.as_str(),
                self.reviewer_actor_ref.as_str(),
                self.source_ref.as_str(),
                self.outcome_text.as_str(),
            ]
            .iter()
            .any(|s| s.trim().is_empty() || s.len() > 4096 || s.contains('\0'))
        {
            return Err(ContractError::InvalidInvariant);
        }
        Ok(())
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum LocalDependencyQualificationV2 {
    DeferredIntrinsic,
    QualifiedFreshness,
    NotApplicableToFreshnessScope,
}
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct LocalDependencyApplicabilityV2 {
    pub kind: String,
    pub binding: ExactBindingV1,
    pub qualification: LocalDependencyQualificationV2,
}
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct LocalContractApplicabilityV2 {
    pub governance_basis: ProjectAdmissionScopeBasisV3,
    pub owner_rule: ExactBindingV1,
    pub dependencies: Vec<LocalDependencyApplicabilityV2>,
    pub policy_admission_closure_fingerprint: FingerprintV1,
}
pub(crate) fn resolve_local_applicability_v2(
    repo: &Path,
    project: &ProjectAdmissionBindingV3,
    definition: &LocalContractDefinitionV2,
) -> Result<(LocalContractApplicabilityV2, LocalCheckCurrentBasisV1), ContractError> {
    definition.validate()?;
    if !matches!(project, ProjectAdmissionBindingV3::ScopedChoiceV2(_))
        || definition
            .dependencies
            .required_governance_scopes
            .iter()
            .any(|s| s != ADMISSION_GOVERNANCE_SCOPE_V1)
    {
        return Err(ContractError::InvalidInvariant);
    }
    let basis =
        resolve_project_admission_scope_v3(repo, project, ADMISSION_GOVERNANCE_SCOPE_V1, None)
            .map_err(|_| ContractError::InvalidInvariant)?;
    let current = observe_basis(repo)?;
    if basis
        != resolve_project_admission_scope_v3(repo, project, ADMISSION_GOVERNANCE_SCOPE_V1, None)
            .map_err(|_| ContractError::InvalidInvariant)?
        || current != observe_basis(repo)?
    {
        return Err(ContractError::InvalidInvariant);
    }
    let owner_rule = freshness_admission_rule_descriptor_v1()
        .map_err(|_| ContractError::InvalidInvariant)?
        .binding;
    let governed = match &basis {
        ProjectAdmissionScopeBasisV3::DeferredV2(_) => None,
        ProjectAdmissionScopeBasisV3::EstablishedFromDeferredV3(g) => {
            if g.owner_rule.binding != owner_rule || g.configuration.bindings.len() != 1 {
                return Err(ContractError::InvalidInvariant);
            }
            let r = &g.configuration.bindings[0];
            if r.owner != ADMISSION_GOVERNANCE_OWNER_V1
                || r.rule_name != FRESHNESS_ADMISSION_RULE_NAME_V1
                || r.rule_version != GOVERNANCE_RULE_VERSION_V1
                || r.candidate_kind != FRESHNESS_CANDIDATE_KIND_V1
                || !(1..=JSON_SAFE_INTEGER_MAX).contains(&r.max_observation_age_seconds)
                || !r.require_exact_source_revision
                || r.future_skew_seconds != 0
                || !r.required_authored_sources.is_empty()
                || !r.approval_sources.is_empty()
                || !r.exception_sources.is_empty()
            {
                return Err(ContractError::InvalidInvariant);
            }
            Some(r)
        }
        _ => return Err(ContractError::InvalidInvariant),
    };
    let mut dependencies = vec![];
    for policy in &definition.dependencies.freshness_policies {
        if let Some(r) = governed {
            if !(1..=JSON_SAFE_INTEGER_MAX).contains(&policy.max_observation_age_seconds())
                || policy.max_observation_age_seconds() > r.max_observation_age_seconds
                || !policy.require_exact_source_revision()
                || policy.future_skew_seconds() != 0
            {
                return Err(ContractError::InvalidInvariant);
            }
        }
        dependencies.push(LocalDependencyApplicabilityV2 {
            kind: "evidence_freshness_policy".into(),
            binding: policy.exact_binding(),
            qualification: if governed.is_some() {
                LocalDependencyQualificationV2::QualifiedFreshness
            } else {
                LocalDependencyQualificationV2::DeferredIntrinsic
            },
        });
    }
    let d = &definition.dependencies;
    let mut other = vec![
        ("contract_definition", definition.binding()),
        ("definition_schema", d.definition_schema.binding.clone()),
        ("extensions_policy", d.extensions_policy.clone()),
        ("lifecycle_policy", d.lifecycle_policy.clone()),
        (
            "evidence_admission_policy",
            d.evidence_admission_policy.clone(),
        ),
        ("evidence_stability_policy", d.stability_policy.clone()),
        ("gate_policy", d.gate_policy.exact_binding()),
        (
            "local_closeout_policy",
            d.gate_policy.local_closeout_policy().clone(),
        ),
        ("fact_schema", d.fact_schema.binding.clone()),
        ("validator_manifest", d.validator.manifest.clone()),
        ("root_schema", d.schema_closure.root_schema.clone()),
        (
            "matcher",
            MatcherBindingV1::exact_fields_v1().matcher().clone(),
        ),
        (
            "matcher_input_schema",
            MatcherBindingV1::exact_fields_v1().input_schema().clone(),
        ),
        ("resolution_stack", definition.resolution_stack.clone()),
    ];
    for doc in &d.schema_closure.documents {
        other.push((
            "schema_document",
            ExactBindingV1::new(
                ExactRefV1::parse(&format!(
                    "handbook.local-schema.document.sha256-{}@1.0.0",
                    FingerprintV1::from_bytes(doc.path.as_bytes())
                        .as_str()
                        .trim_start_matches("sha256:")
                ))?,
                doc.document_fingerprint.clone(),
            ),
        ));
    }
    let mut exact = BTreeSet::new();
    for (kind, binding) in other {
        if !exact.insert((kind, binding.clone())) {
            return Err(ContractError::InvalidInvariant);
        }
        dependencies.push(LocalDependencyApplicabilityV2 {
            kind: kind.into(),
            binding,
            qualification: if governed.is_some() {
                LocalDependencyQualificationV2::NotApplicableToFreshnessScope
            } else {
                LocalDependencyQualificationV2::DeferredIntrinsic
            },
        });
    }
    let policy_admission_closure_fingerprint = canonical_fingerprint(&(
        "handbook.local-policy-admission-closure@2.0.0",
        &basis,
        &owner_rule,
        &dependencies,
        &current,
    ))?;
    Ok((
        LocalContractApplicabilityV2 {
            governance_basis: basis,
            owner_rule,
            dependencies,
            policy_admission_closure_fingerprint,
        },
        current,
    ))
}
fn observe_basis(repo: &Path) -> Result<LocalCheckCurrentBasisV1, ContractError> {
    let root =
        RetainedRepositoryRootV1::open_exact(repo).map_err(|_| ContractError::InvalidInvariant)?;
    let bytes = match root
        .read_regular_bounded(GOVERNANCE_CURRENT_PATH_V2, MAX_GOVERNANCE_RECORD_BYTES_V1)
        .map_err(|_| ContractError::InvalidInvariant)?
    {
        RepositoryReadV1::Found(bytes) => bytes,
        RepositoryReadV1::Absent => return Err(ContractError::InvalidInvariant),
    };
    NativeLocalCheckAuthorityV1::observe_basis(repo, FingerprintV1::from_bytes(&bytes).as_str())
        .map_err(|_| ContractError::InvalidInvariant)
}
/// Private construction is owner-only; serialization exposes provenance, never
/// a replacement for fresh applicability or a live Engine admission.
#[derive(Clone, Debug)]
pub struct LocalContractUseV2 {
    pub(crate) definition: LocalContractDefinitionV2,
    pub(crate) applicability: LocalContractApplicabilityV2,
    pub(crate) current_basis: LocalCheckCurrentBasisV1,
    pub(crate) lifecycle_head_fingerprint: FingerprintV1,
    pub(crate) intent: LocalContractUseIntentV2,
    pub(crate) required_state_paths: Vec<String>,
}
impl LocalContractUseV2 {
    pub fn definition(&self) -> &LocalContractDefinitionV2 {
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
