use super::*;
use crate::local_authority::local_check::{LocalCheckCurrentBasisV1, LocalCheckLocatorV1};

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct WorkDescriptorV1 {
    pub reference: String,
    pub compatibility_version: String,
    pub initial_state: String,
    pub role_schema: Vec<String>,
    pub relation_support: Vec<String>,
    pub completion_support: String,
}
impl WorkDescriptorV1 {
    pub fn ordinary_leaf() -> Self {
        Self {
            reference: ORDINARY_LEAF_COMPOSITION_V1.into(),
            compatibility_version: "1".into(),
            initial_state: "open".into(),
            role_schema: vec![],
            relation_support: vec![],
            completion_support: "unsupported_not_assessed".into(),
        }
    }
    pub fn planning_output() -> Self {
        Self {
            reference: PLANNING_OUTPUT_COMPOSITION_V1.into(),
            role_schema: vec!["required_contract_output_question@1.0.0".into()],
            ..Self::ordinary_leaf()
        }
    }
    pub fn requirements_input() -> Self {
        Self {
            reference: REQUIREMENTS_INPUT_COMPOSITION_V1.into(),
            role_schema: vec!["exact_requirements_definition@1.0.0".into()],
            ..Self::ordinary_leaf()
        }
    }
    pub fn canonical_json_bytes(&self) -> Result<Vec<u8>, LocalAuthorityRefusalV1> {
        self.validate()?;
        encode(self)
    }
    pub(super) fn validate(&self) -> Result<(), LocalAuthorityRefusalV1> {
        if self != &Self::ordinary_leaf()
            && self != &Self::planning_output()
            && self != &Self::requirements_input()
        {
            Err(refuse(
                "unsupported_work_composition",
                "exact fixed ordinary/planning-output/requirements-input open leaf descriptor required",
            ))
        } else {
            Ok(())
        }
    }
}
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct WorkCompositionV1 {
    pub descriptor: WorkDescriptorV1,
    pub kind: Option<crate::local_authority::LocalBindingV1>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub role: Option<WorkRoleValueV1>,
}
impl WorkCompositionV1 {
    pub fn ordinary_leaf() -> Self {
        Self {
            descriptor: WorkDescriptorV1::ordinary_leaf(),
            kind: None,
            role: None,
        }
    }
    pub fn planning_output(question: impl Into<String>) -> Self {
        Self {
            descriptor: WorkDescriptorV1::planning_output(),
            kind: None,
            role: Some(WorkRoleValueV1::PlanningOutput {
                question: question.into(),
            }),
        }
    }
    pub fn requirements_input(definition: WorkRequirementsDefinitionV1) -> Self {
        Self {
            descriptor: WorkDescriptorV1::requirements_input(),
            kind: None,
            role: Some(WorkRoleValueV1::RequirementsInput { definition }),
        }
    }
    pub(super) fn validate(&self) -> Result<(), LocalAuthorityRefusalV1> {
        self.descriptor.validate()?;
        if self.kind.is_some() {
            return Err(refuse(
                "unsupported_work_kind",
                "kind bindings require separately supported definition authority",
            ));
        }
        match (&self.descriptor, &self.role) {
            (d, None) if d == &WorkDescriptorV1::ordinary_leaf() => {}
            (d, Some(WorkRoleValueV1::PlanningOutput { question }))
                if d == &WorkDescriptorV1::planning_output() =>
            {
                text(question)?
            }
            (d, Some(WorkRoleValueV1::RequirementsInput { definition }))
                if d == &WorkDescriptorV1::requirements_input() =>
            {
                definition.validate()?
            }
            _ => {
                return Err(refuse(
                    "work_role_values",
                    "exact descriptor and complete matching role value required",
                ))
            }
        }
        Ok(())
    }
}

/// Exact explicit approval under the fixed establishment rule, not a grant that
/// can approve itself. Old LocalCheck reach is never expanded or reinterpreted.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct WorkRegistrationTermsV1 {
    pub request_key: String,
    pub rule_ref: String,
    pub mode: String,
    pub principal: String,
    pub agent: String,
    pub instruction: String,
    pub delegation: String,
    pub approval_source_ref: String,
    pub human_approval_asserted: bool,
    pub approved_content_fingerprint: String,
    pub effect: String,
    /// Schema dispatch retains the original singleton wire field exactly.
    #[serde(rename = "composition")]
    pub scope: WorkRegistrationScopeV1,
    pub owner_write_reach: String,
    pub owner_read_reach: String,
    pub reserved_decisions: Vec<String>,
    pub revocation_principal: String,
    pub retention: String,
}
impl WorkRegistrationTermsV1 {
    pub fn fingerprint_approved_content(&self) -> Result<String, LocalAuthorityRefusalV1> {
        let mut v = serde_json::to_value(self).map_err(codec)?;
        let o = v.as_object_mut().ok_or_else(|| codec("object"))?;
        o.remove("approved_content_fingerprint");
        o.remove("human_approval_asserted");
        fp("handbook.work-registration-approved@1.0.0", &v)
    }
    pub fn canonical_json_bytes(&self) -> Result<Vec<u8>, LocalAuthorityRefusalV1> {
        self.validate()?;
        encode(self)
    }
    pub fn from_json_bytes(b: &[u8]) -> Result<Self, LocalAuthorityRefusalV1> {
        let v: Self = decode(b)?;
        v.validate()?;
        Ok(v)
    }
    pub(crate) fn validate(&self) -> Result<(), LocalAuthorityRefusalV1> {
        token(&self.request_key)?;
        for v in [
            &self.principal,
            &self.agent,
            &self.instruction,
            &self.delegation,
            &self.approval_source_ref,
            &self.revocation_principal,
        ] {
            text(v)?;
        }
        self.scope.validate()?;
        if self.rule_ref != WORK_REGISTRATION_RULE_V1
            || self.mode != "local_advisory"
            || self.effect != "register_open_leaf"
            || self.owner_write_reach != WORK_STATE_FULL_OWNER_REACH_V1
            || self.owner_read_reach != WORK_STATE_FULL_OWNER_REACH_V1
            || self.retention != "retain_original_history"
            || self.reserved_decisions != ["composition_change", "terms_change", "close"]
            || self.revocation_principal != self.principal
            || !self.human_approval_asserted
            || self.approved_content_fingerprint != self.fingerprint_approved_content()?
        {
            return Err(refuse("work_terms", "exact explicit human-approved registration effect, full owner reach, reservations and retention required"));
        }
        encode(self)?;
        Ok(())
    }
}
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct WorkGrantLocatorV1 {
    pub check: LocalCheckLocatorV1,
    pub basis: LocalCheckCurrentBasisV1,
    /// Exactly one expected predecessor for scope change or unchanged-scope D→G.
    pub predecessor_grant_fingerprint: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub change: Option<WorkAuthorityChangeV1>,
    pub terms: WorkRegistrationTermsV1,
    pub fingerprint: String,
}
impl WorkGrantLocatorV1 {
    pub fn canonical_json_bytes(&self) -> Result<Vec<u8>, LocalAuthorityRefusalV1> {
        self.validate()?;
        encode(self)
    }
    pub fn from_json_bytes(b: &[u8]) -> Result<Self, LocalAuthorityRefusalV1> {
        let v: Self = decode(b)?;
        v.validate()?;
        Ok(v)
    }
    pub(crate) fn validate(&self) -> Result<(), LocalAuthorityRefusalV1> {
        self.check.canonical_json_bytes()?;
        self.terms.validate()?;
        if self.change == Some(WorkAuthorityChangeV1::Initial)
            && self.predecessor_grant_fingerprint.is_some()
            || matches!(
                self.change,
                Some(
                    WorkAuthorityChangeV1::ScopeChange
                        | WorkAuthorityChangeV1::EstablishFromDeferred
                )
            ) && self.predecessor_grant_fingerprint.is_none()
        {
            return Err(refuse(
                "work_change_binding",
                "explicit change and expected predecessor differ",
            ));
        }
        if self.terms.principal != self.check.predecessor.terms.principal
            || self.terms.agent != self.check.terms.agent
            || self.fingerprint != self.identity()?
        {
            return Err(refuse(
                "work_grant_binding",
                "original claimed human/check agent and exact approved lineage required",
            ));
        }
        Ok(())
    }
    pub(crate) fn identity(&self) -> Result<String, LocalAuthorityRefusalV1> {
        if let Some(change) = self.change {
            return fp(
                "handbook.work-registration-grant@2.0.0",
                &(
                    &self.check,
                    &self.basis,
                    &self.predecessor_grant_fingerprint,
                    &self.terms,
                    change,
                ),
            );
        }
        fp(
            WORK_REGISTRATION_RULE_V1,
            &(
                &self.check,
                &self.basis,
                &self.predecessor_grant_fingerprint,
                &self.terms,
            ),
        )
    }
    pub(crate) fn project(&self) -> &str {
        self.check
            .predecessor
            .charter
            .binding
            .initialization_claim
            .proposed_repository_identity()
    }
}
#[derive(Debug)]
pub struct PreparedWorkGrantV1 {
    pub(crate) locator: WorkGrantLocatorV1,
}
impl PreparedWorkGrantV1 {
    pub fn locator(&self) -> &WorkGrantLocatorV1 {
        &self.locator
    }
}
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct WorkGrantV1 {
    pub locator: WorkGrantLocatorV1,
    pub assurance: String,
    pub establishment_rule: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct WorkRegistrationInputV1 {
    pub request_key: String,
    pub item_key: String,
    pub label: String,
    pub question: String,
    pub actor: String,
    pub expected_state: String,
    pub composition: WorkCompositionInputV1,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub display_level: Option<String>,
}
impl WorkRegistrationInputV1 {
    pub fn canonical_json_bytes(&self) -> Result<Vec<u8>, LocalAuthorityRefusalV1> {
        self.validate()?;
        encode(self)
    }
    pub fn from_json_bytes(b: &[u8]) -> Result<Self, LocalAuthorityRefusalV1> {
        let v: Self = decode(b)?;
        v.validate()?;
        Ok(v)
    }
    pub(crate) fn validate(&self) -> Result<(), LocalAuthorityRefusalV1> {
        token(&self.request_key)?;
        token(&self.item_key)?;
        for v in [&self.label, &self.question, &self.actor] {
            text(v)?;
        }
        self.composition.validate()?;
        if let Some(level) = &self.display_level {
            token(level)?;
        }
        if self.expected_state != "absent" {
            return Err(refuse(
                "unsupported_work_effect",
                "only initial absent-to-open registration is supported",
            ));
        }
        encode(self)?;
        Ok(())
    }
}
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct WorkRequestLocatorV1 {
    pub grant: WorkGrantLocatorV1,
    pub input: WorkRegistrationInputV1,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kind_provenance: Option<WorkKindProvenanceV1>,
    pub item_identity: String,
    pub fingerprint: String,
}
impl WorkRequestLocatorV1 {
    pub fn canonical_json_bytes(&self) -> Result<Vec<u8>, LocalAuthorityRefusalV1> {
        self.validate()?;
        encode(self)
    }
    pub fn from_json_bytes(b: &[u8]) -> Result<Self, LocalAuthorityRefusalV1> {
        let v: Self = decode(b)?;
        v.validate()?;
        Ok(v)
    }
    pub(crate) fn validate(&self) -> Result<(), LocalAuthorityRefusalV1> {
        self.grant.validate()?;
        self.input.validate()?;
        self.grant.terms.scope.permits(&self.effective_input()?)?;
        if self.input.actor != self.grant.terms.agent
            || self.item_identity
                != work_item_identity_v1(self.grant.project(), &self.input.item_key)?
            || self.fingerprint != self.identity()?
        {
            return Err(refuse(
                "work_request_binding",
                "exact actor/composition/project/item/request binding required",
            ));
        }
        Ok(())
    }
    pub(crate) fn identity(&self) -> Result<String, LocalAuthorityRefusalV1> {
        if !self.input.composition.retained() || self.kind_provenance.is_some() {
            return fp(
                "handbook.work-registration-request@2.0.0",
                &(
                    &self.grant,
                    &self.input,
                    &self.item_identity,
                    &self.kind_provenance,
                ),
            );
        }
        fp(
            "handbook.work-registration-request@1.0.0",
            &(&self.grant, &self.input, &self.item_identity),
        )
    }
    pub(crate) fn effective_input(
        &self,
    ) -> Result<WorkRegistrationInputV1, LocalAuthorityRefusalV1> {
        let mut input = self.input.clone();
        match (&input.composition, &self.kind_provenance) {
            (WorkCompositionInputV1::Kind { binding, role }, Some(provenance)) => {
                provenance.establishment.canonical_json_bytes()?;
                let definition = &provenance.establishment.definition;
                let expanded = WorkCompositionV1 {
                    descriptor: definition.template.descriptor.clone(),
                    kind: None,
                    role: role.clone(),
                };
                expanded.validate()?;
                if binding != &provenance.establishment.binding
                    || binding.project_identity != self.grant.project()
                    || provenance.expanded != expanded
                {
                    return Err(refuse(
                        "work_kind_provenance",
                        "exact same-project committed kind and expansion required",
                    ));
                }
                input.composition = expanded.into();
                if !definition.template.permits(&input) {
                    return Err(refuse(
                        "work_kind_bounds",
                        "expanded values exceed exact retained kind bounds",
                    ));
                }
            }
            (
                WorkCompositionInputV1::Explicit(_) | WorkCompositionInputV1::RetainedExplicit(_),
                None,
            ) => {}
            _ => {
                return Err(refuse(
                    "work_kind_provenance",
                    "submitted form and complete exact kind dependency differ",
                ))
            }
        }
        Ok(input)
    }
}
/// Identity has no codec, release, descriptor or delivery input.
pub fn work_item_identity_v1(project: &str, key: &str) -> Result<String, LocalAuthorityRefusalV1> {
    token(key)?;
    text(project)?;
    fp(WORK_IDENTITY_SCHEME_V1, &(project, key))
}
#[derive(Debug)]
pub struct PreparedWorkRegistrationV1 {
    pub(crate) locator: WorkRequestLocatorV1,
}
impl PreparedWorkRegistrationV1 {
    pub fn locator(&self) -> &WorkRequestLocatorV1 {
        &self.locator
    }
}
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct WorkSnapshotV1 {
    pub version: String,
    pub project_identity: String,
    pub item_identity: String,
    pub item_key: String,
    pub label: String,
    pub question: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub display_level: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub role: Option<WorkRoleValueV1>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub admitted_kind: Option<WorkKindBindingV1>,
    pub state: String,
    pub descriptor: WorkDescriptorV1,
    pub descriptor_bytes: Vec<u8>,
    pub descriptor_fingerprint: String,
    pub registration_request_fingerprint: String,
}
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct WorkRegistrationReceiptV1 {
    pub locator: WorkRequestLocatorV1,
    pub snapshot: WorkSnapshotV1,
    pub effect: String,
    pub assurance: String,
    pub receipt_fingerprint: String,
}
#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum WorkDispositionV1 {
    Committed,
    Replayed,
    Refused,
    NonCommitted,
    Uncertain,
    Pending,
    Unaccepted,
}
#[derive(Clone, Debug, Serialize)]
pub struct WorkExecutionV1<T> {
    pub disposition: WorkDispositionV1,
    pub outcome: Option<T>,
    pub refusal: Option<LocalAuthorityRefusalV1>,
}
impl<T> WorkExecutionV1<T> {
    pub(crate) fn success(d: WorkDispositionV1, v: Option<T>) -> Self {
        Self {
            disposition: d,
            outcome: v,
            refusal: None,
        }
    }
    pub(crate) fn error(d: WorkDispositionV1, e: LocalAuthorityRefusalV1) -> Self {
        Self {
            disposition: d,
            outcome: None,
            refusal: Some(e),
        }
    }
}
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct WorkRevokeLocatorV1 {
    pub grant: WorkGrantLocatorV1,
    pub request_key: String,
    pub principal: String,
    pub source_ref: String,
    pub instruction: String,
    pub fingerprint: String,
}
impl WorkRevokeLocatorV1 {
    pub fn canonical_json_bytes(&self) -> Result<Vec<u8>, LocalAuthorityRefusalV1> {
        self.validate()?;
        encode(self)
    }
    pub fn from_json_bytes(b: &[u8]) -> Result<Self, LocalAuthorityRefusalV1> {
        let v: Self = decode(b)?;
        v.validate()?;
        Ok(v)
    }
    pub(crate) fn identity(&self) -> Result<String, LocalAuthorityRefusalV1> {
        fp(
            "handbook.work-revocation@1.0.0",
            &(
                &self.grant,
                &self.request_key,
                &self.principal,
                &self.source_ref,
                &self.instruction,
            ),
        )
    }
    pub(crate) fn validate(&self) -> Result<(), LocalAuthorityRefusalV1> {
        self.grant.validate()?;
        token(&self.request_key)?;
        text(&self.source_ref)?;
        text(&self.instruction)?;
        if self.principal != self.grant.terms.revocation_principal
            || self.fingerprint != self.identity()?
        {
            return Err(refuse(
                "work_revoke_principal",
                "exact declared revocation principal and original content required",
            ));
        }
        Ok(())
    }
}
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct WorkRevocationReceiptV1 {
    pub locator: WorkRevokeLocatorV1,
    pub effect: String,
}
#[derive(Clone, Debug, PartialEq, Serialize)]
pub enum WorkCurrentItemV1 {
    Absent,
    Pending {
        original_request: WorkRequestLocatorV1,
    },
    Open(WorkSnapshotV1),
}
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct WorkCompletionSupportV1 {
    pub item: WorkSnapshotV1,
    pub completion_support: String,
    pub assessment: String,
}
