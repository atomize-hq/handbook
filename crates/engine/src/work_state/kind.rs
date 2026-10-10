//! Exact immutable whole-composition definitions. Approval establishes retained
//! meaning under a fixed human rule; it supplies no work-registration permission.
use super::*;
use crate::local_authority::local_check::{LocalCheckCurrentBasisV1, LocalCheckLocatorV1};

pub const WORK_KIND_DEFINITION_SCHEMA_V1: &str = "handbook.work-kind-definition@1.0.0";
pub const WORK_KIND_BINDING_SCHEMA_V1: &str = "handbook.work-kind-binding@1.0.0";
pub const WORK_KIND_ESTABLISHMENT_RULE_V1: &str =
    "handbook.operating.work-kind-establishment@1.0.0";
const OWNER: &str = "handbook-engine.work-state";

fn exact_reference(value: &str) -> Result<(), LocalAuthorityRefusalV1> {
    let (name, version) = value
        .split_once('@')
        .ok_or_else(|| codec("exact kind reference"))?;
    if name.len() > 255
        || !name.contains('.')
        || !name.split('.').all(|segment| {
            !segment.is_empty()
                && segment.as_bytes()[0].is_ascii_lowercase()
                && segment
                    .as_bytes()
                    .last()
                    .is_some_and(|b| b.is_ascii_lowercase() || b.is_ascii_digit())
                && segment
                    .bytes()
                    .all(|b| b.is_ascii_lowercase() || b.is_ascii_digit() || b == b'-')
        })
        || version.len() > 255
        || !semver::Version::parse(version).is_ok_and(|v| v.to_string() == version)
    {
        return Err(refuse(
            "work_kind_reference",
            "bounded exact dotted lowercase name and canonical semver required",
        ));
    }
    Ok(())
}
fn fingerprint(value: &str) -> Result<(), LocalAuthorityRefusalV1> {
    if value.len() != 71
        || !value.starts_with("sha256:")
        || !value.as_bytes()[7..]
            .iter()
            .all(|b| b.is_ascii_digit() || matches!(b, b'a'..=b'f'))
    {
        return Err(codec("full kind fingerprint"));
    }
    Ok(())
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct WorkKindDefinitionV1 {
    pub schema_ref: String,
    pub reference: String,
    pub template: WorkCompositionClauseV1,
}
impl WorkKindDefinitionV1 {
    pub fn canonical_json_bytes(&self) -> Result<Vec<u8>, LocalAuthorityRefusalV1> {
        self.validate()?;
        encode(self)
    }
    pub fn from_json_bytes(bytes: &[u8]) -> Result<Self, LocalAuthorityRefusalV1> {
        let value: Self = decode(bytes)?;
        value.validate()?;
        Ok(value)
    }
    pub fn fingerprint(&self) -> Result<String, LocalAuthorityRefusalV1> {
        self.validate()?;
        fp(WORK_KIND_DEFINITION_SCHEMA_V1, self)
    }
    pub(crate) fn validate(&self) -> Result<(), LocalAuthorityRefusalV1> {
        exact_reference(&self.reference)?;
        if self.schema_ref != WORK_KIND_DEFINITION_SCHEMA_V1 {
            return Err(codec("closed kind definition schema"));
        }
        self.template.validate()?;
        encode(self)?;
        Ok(())
    }
    pub(crate) fn binding(
        &self,
        project: &str,
    ) -> Result<WorkKindBindingV1, LocalAuthorityRefusalV1> {
        let binding = WorkKindBindingV1 {
            owner: OWNER.into(),
            schema_ref: WORK_KIND_BINDING_SCHEMA_V1.into(),
            project_identity: project.into(),
            reference: self.reference.clone(),
            fingerprint: self.fingerprint()?,
        };
        binding.validate()?;
        Ok(binding)
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct WorkKindBindingV1 {
    pub owner: String,
    pub schema_ref: String,
    pub project_identity: String,
    pub reference: String,
    pub fingerprint: String,
}
impl WorkKindBindingV1 {
    pub fn canonical_json_bytes(&self) -> Result<Vec<u8>, LocalAuthorityRefusalV1> {
        self.validate()?;
        encode(self)
    }
    pub fn from_json_bytes(bytes: &[u8]) -> Result<Self, LocalAuthorityRefusalV1> {
        let value: Self = decode(bytes)?;
        value.validate()?;
        Ok(value)
    }
    pub(crate) fn validate(&self) -> Result<(), LocalAuthorityRefusalV1> {
        exact_reference(&self.reference)?;
        text(&self.project_identity)?;
        fingerprint(&self.fingerprint)?;
        if self.owner != OWNER || self.schema_ref != WORK_KIND_BINDING_SCHEMA_V1 {
            return Err(refuse(
                "work_kind_binding",
                "exact purpose-specific Engine kind binding required",
            ));
        }
        Ok(())
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct WorkKindApprovalV1 {
    pub request_key: String,
    pub rule_ref: String,
    pub mode: String,
    pub principal: String,
    pub agent: String,
    pub check_request_fingerprint: String,
    pub basis: LocalCheckCurrentBasisV1,
    pub instruction: String,
    pub delegation: String,
    pub approval_source_ref: String,
    pub human_approval_asserted: bool,
    pub approved_content_fingerprint: String,
    pub effect: String,
    pub owner_write_reach: String,
    pub owner_read_reach: String,
    pub retention: String,
}
impl WorkKindApprovalV1 {
    pub fn fingerprint_approved_content(
        &self,
        definition: &WorkKindDefinitionV1,
    ) -> Result<String, LocalAuthorityRefusalV1> {
        definition.validate()?;
        let mut claims = serde_json::to_value(self).map_err(codec)?;
        let object = claims
            .as_object_mut()
            .ok_or_else(|| codec("approval object"))?;
        object.remove("human_approval_asserted");
        object.remove("approved_content_fingerprint");
        fp("handbook.work-kind-approved@1.0.0", &(claims, definition))
    }
    /// Portable claims are shape-checked here; prepare additionally checks the
    /// approved full content and the exact held native lineage and basis.
    pub fn canonical_json_bytes(&self) -> Result<Vec<u8>, LocalAuthorityRefusalV1> {
        self.validate_claims()?;
        encode(self)
    }
    pub fn from_json_bytes(bytes: &[u8]) -> Result<Self, LocalAuthorityRefusalV1> {
        let value: Self = decode(bytes)?;
        value.validate_claims()?;
        Ok(value)
    }
    fn validate_claims(&self) -> Result<(), LocalAuthorityRefusalV1> {
        token(&self.request_key)?;
        for value in [
            &self.principal,
            &self.agent,
            &self.instruction,
            &self.delegation,
            &self.approval_source_ref,
        ] {
            text(value)?;
        }
        fingerprint(&self.check_request_fingerprint)?;
        fingerprint(&self.basis.relation_fingerprint)?;
        fingerprint(&self.basis.governance_tree_fingerprint)?;
        fingerprint(&self.approved_content_fingerprint)?;
        if self.rule_ref != WORK_KIND_ESTABLISHMENT_RULE_V1
            || self.mode != "local_advisory"
            || self.effect != "establish_immutable_work_kind"
            || !self.human_approval_asserted
            || self.owner_write_reach != WORK_STATE_FULL_OWNER_REACH_V1
            || self.owner_read_reach != WORK_STATE_FULL_OWNER_REACH_V1
            || self.retention != "retain_original_history"
        {
            return Err(refuse("work_kind_approval", "separate exact original-human version approval and fixed definition effect required"));
        }
        Ok(())
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct WorkKindLocatorV1 {
    pub check: LocalCheckLocatorV1,
    pub basis: LocalCheckCurrentBasisV1,
    pub definition: WorkKindDefinitionV1,
    pub approval: WorkKindApprovalV1,
    pub binding: WorkKindBindingV1,
    pub fingerprint: String,
}
impl WorkKindLocatorV1 {
    pub fn canonical_json_bytes(&self) -> Result<Vec<u8>, LocalAuthorityRefusalV1> {
        self.validate()?;
        encode(self)
    }
    pub fn from_json_bytes(bytes: &[u8]) -> Result<Self, LocalAuthorityRefusalV1> {
        let value: Self = decode(bytes)?;
        value.validate()?;
        Ok(value)
    }
    pub(crate) fn project(&self) -> &str {
        self.check
            .predecessor
            .charter
            .binding
            .initialization_claim
            .proposed_repository_identity()
    }
    pub(crate) fn identity(&self) -> Result<String, LocalAuthorityRefusalV1> {
        fp(
            "handbook.work-kind-establishment-request@1.0.0",
            &(
                &self.check,
                &self.basis,
                &self.definition,
                &self.approval,
                &self.binding,
            ),
        )
    }
    pub(crate) fn validate(&self) -> Result<(), LocalAuthorityRefusalV1> {
        self.check.canonical_json_bytes()?;
        self.definition.validate()?;
        self.approval.validate_claims()?;
        if self.approval.principal != self.check.predecessor.terms.principal
            || self.approval.agent != self.check.terms.agent
            || self.approval.check_request_fingerprint != self.check.request_fingerprint
            || self.approval.basis != self.basis
            || self.approval.approved_content_fingerprint
                != self
                    .approval
                    .fingerprint_approved_content(&self.definition)?
            || self.binding != self.definition.binding(self.project())?
            || self.fingerprint != self.identity()?
        {
            return Err(refuse("work_kind_approval_binding", "exact original claimed human/check agent, approved version/content and native lineage/basis required"));
        }
        encode(self)?;
        Ok(())
    }
}
#[derive(Debug)]
pub struct PreparedWorkKindV1 {
    pub(crate) locator: WorkKindLocatorV1,
}
impl PreparedWorkKindV1 {
    pub fn locator(&self) -> &WorkKindLocatorV1 {
        &self.locator
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct WorkKindReceiptV1 {
    pub locator: WorkKindLocatorV1,
    pub binding: WorkKindBindingV1,
    pub definition: WorkKindDefinitionV1,
    pub effect: String,
    pub assurance: String,
    pub receipt_fingerprint: String,
}
impl WorkKindReceiptV1 {
    pub(crate) fn from_locator(
        locator: &WorkKindLocatorV1,
    ) -> Result<Self, LocalAuthorityRefusalV1> {
        locator.validate()?;
        let binding = locator.binding.clone();
        let definition = locator.definition.clone();
        let effect = "establish_immutable_work_kind";
        let assurance = "local_advisory_claims";
        let receipt_fingerprint = fp(
            "handbook.work-kind-establishment-receipt@1.0.0",
            &(locator, &binding, &definition, effect, assurance),
        )?;
        let value = Self {
            locator: locator.clone(),
            binding,
            definition,
            effect: effect.into(),
            assurance: assurance.into(),
            receipt_fingerprint,
        };
        encode(&value)?;
        Ok(value)
    }
    pub fn canonical_json_bytes(&self) -> Result<Vec<u8>, LocalAuthorityRefusalV1> {
        if self != &Self::from_locator(&self.locator)? {
            return Err(codec("exact kind receipt"));
        }
        encode(self)
    }
    pub fn from_json_bytes(bytes: &[u8]) -> Result<Self, LocalAuthorityRefusalV1> {
        let value: Self = decode(bytes)?;
        value.canonical_json_bytes()?;
        Ok(value)
    }
}
#[derive(Clone, Debug, PartialEq, Serialize)]
pub enum WorkCurrentKindV1 {
    Absent,
    Pending { original_request: WorkKindLocatorV1 },
    Committed(WorkKindReceiptV1),
}

/// Sealed into the request identity at preparation, then checked against the
/// actual retained terminal/definition bytes through commit and historical read.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct WorkKindProvenanceV1 {
    pub establishment: WorkKindReceiptV1,
    pub expanded: WorkCompositionV1,
}
