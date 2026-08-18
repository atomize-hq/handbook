use crate::canonical::{canonical_bytes, canonical_fingerprint};
use crate::claim::validate_claims;
use crate::{
    ContractClaimV1, ContractError, ContractIdV1, ExactBindingV1, ExactRefV1, FingerprintV1,
    GateEffectV1, MatcherBindingV1, MinimumVersionChangeV1, StableIdV1, VersionV1,
};
use serde::de::Error as _;
use serde::{Deserialize, Deserializer, Serialize};

const MAX_MATCHER_BINDINGS: usize = 16;
const DEFINITION_SCHEMA_REF_V1: &str = "handbook.contract-definition@1.0.0";

/// Exact source identity for this semantic owner crate.
pub const CONTRACT_OWNER_SOURCE_REF_V1: &str = "handbook.owner.contracts@0.1.0";

/// The sole generic V1 contract kind. It declares a typed claim set and grants
/// no lifecycle, evaluation, verdict, or gate authority.
#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ContractKindV1 {
    ClaimSet,
}

/// Authored inputs for immutable definition construction.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ContractDefinitionInputV1 {
    pub definition_schema: ExactBindingV1,
    pub contract_id: ContractIdV1,
    pub contract_version: VersionV1,
    pub contract_kind: ContractKindV1,
    pub claims: Vec<ContractClaimV1>,
    pub definition_author_ref: StableIdV1,
    pub definition_authority: ExactBindingV1,
    pub lifecycle_policy: ExactBindingV1,
    pub matcher_bindings: Vec<MatcherBindingV1>,
    pub gate_policy: ExactBindingV1,
    pub extensions_policy: ExactBindingV1,
}

/// One exact immutable owner-native contract definition.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct ContractDefinitionV1 {
    definition_schema: ExactBindingV1,
    contract_id: ContractIdV1,
    contract_version: VersionV1,
    contract_ref: ExactRefV1,
    contract_kind: ContractKindV1,
    claims: Vec<ContractClaimV1>,
    definition_author_ref: StableIdV1,
    definition_authority: ExactBindingV1,
    lifecycle_policy: ExactBindingV1,
    matcher_bindings: Vec<MatcherBindingV1>,
    gate_policy: ExactBindingV1,
    extensions_policy: ExactBindingV1,
    definition_fingerprint: FingerprintV1,
}

impl ContractDefinitionV1 {
    /// Validates all closed definition fields and derives the exact ref and
    /// RFC-8785/SHA-256 semantic fingerprint.
    pub fn new(mut input: ContractDefinitionInputV1) -> Result<Self, ContractError> {
        if input.definition_schema.reference().as_str() != DEFINITION_SCHEMA_REF_V1 {
            return Err(ContractError::InvalidInvariant);
        }
        validate_claims(&input.claims)?;
        normalize_matcher_bindings(&mut input.matcher_bindings)?;
        validate_claim_matcher_closure(&input.claims, &input.matcher_bindings)?;

        let contract_ref = ExactRefV1::derive(&input.contract_id, &input.contract_version);
        let preimage = DefinitionFingerprintPreimage {
            definition_schema: &input.definition_schema,
            contract_id: &input.contract_id,
            contract_version: &input.contract_version,
            contract_ref: &contract_ref,
            contract_kind: input.contract_kind,
            claims: &input.claims,
            definition_author_ref: &input.definition_author_ref,
            definition_authority: &input.definition_authority,
            lifecycle_policy: &input.lifecycle_policy,
            matcher_bindings: &input.matcher_bindings,
            gate_policy: &input.gate_policy,
            extensions_policy: &input.extensions_policy,
        };
        let definition_fingerprint = canonical_fingerprint(&preimage)?;
        Ok(Self {
            definition_schema: input.definition_schema,
            contract_id: input.contract_id,
            contract_version: input.contract_version,
            contract_ref,
            contract_kind: input.contract_kind,
            claims: input.claims,
            definition_author_ref: input.definition_author_ref,
            definition_authority: input.definition_authority,
            lifecycle_policy: input.lifecycle_policy,
            matcher_bindings: input.matcher_bindings,
            gate_policy: input.gate_policy,
            extensions_policy: input.extensions_policy,
            definition_fingerprint,
        })
    }

    /// Returns the mechanically derived exact definition ref.
    pub fn contract_ref(&self) -> &ExactRefV1 {
        &self.contract_ref
    }

    /// Returns the stable contract identity.
    pub fn contract_id(&self) -> &ContractIdV1 {
        &self.contract_id
    }

    /// Returns the exact full SemVer.
    pub fn contract_version(&self) -> &VersionV1 {
        &self.contract_version
    }

    /// Returns the closed V1 contract kind.
    pub fn contract_kind(&self) -> ContractKindV1 {
        self.contract_kind
    }

    /// Returns the ordered, unique claim list.
    pub fn claims(&self) -> &[ContractClaimV1] {
        &self.claims
    }

    /// Returns the derived semantic fingerprint.
    pub fn definition_fingerprint(&self) -> &FingerprintV1 {
        &self.definition_fingerprint
    }

    /// Returns true only for this definition's exact ref and fingerprint.
    pub fn matches_exact(&self, reference: &ExactRefV1, fingerprint: &FingerprintV1) -> bool {
        &self.contract_ref == reference && &self.definition_fingerprint == fingerprint
    }

    /// Serializes the complete immutable definition as RFC-8785 canonical JSON.
    pub fn canonical_json_bytes(&self) -> Result<Vec<u8>, ContractError> {
        canonical_bytes(self)
    }

    /// Checks whether the candidate version advances enough for the classified
    /// mutation. It never authorizes using the candidate for an exact pin.
    pub fn candidate_version_satisfies(
        &self,
        candidate: &Self,
        classification: DefinitionMutationClassificationV1,
    ) -> bool {
        classification
            .minimum_version_change()
            .is_none_or(|minimum| {
                minimum.is_satisfied_by(&self.contract_version, &candidate.contract_version)
            })
    }
}

impl<'de> Deserialize<'de> for ContractDefinitionV1 {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        #[serde(deny_unknown_fields)]
        struct Raw {
            definition_schema: ExactBindingV1,
            contract_id: ContractIdV1,
            contract_version: VersionV1,
            contract_ref: ExactRefV1,
            contract_kind: ContractKindV1,
            claims: Vec<ContractClaimV1>,
            definition_author_ref: StableIdV1,
            definition_authority: ExactBindingV1,
            lifecycle_policy: ExactBindingV1,
            matcher_bindings: Vec<MatcherBindingV1>,
            gate_policy: ExactBindingV1,
            extensions_policy: ExactBindingV1,
            definition_fingerprint: FingerprintV1,
        }
        let raw = Raw::deserialize(deserializer)?;
        let supplied_ref = raw.contract_ref;
        let supplied_fingerprint = raw.definition_fingerprint;
        let value = Self::new(ContractDefinitionInputV1 {
            definition_schema: raw.definition_schema,
            contract_id: raw.contract_id,
            contract_version: raw.contract_version,
            contract_kind: raw.contract_kind,
            claims: raw.claims,
            definition_author_ref: raw.definition_author_ref,
            definition_authority: raw.definition_authority,
            lifecycle_policy: raw.lifecycle_policy,
            matcher_bindings: raw.matcher_bindings,
            gate_policy: raw.gate_policy,
            extensions_policy: raw.extensions_policy,
        })
        .map_err(D::Error::custom)?;
        if supplied_ref != value.contract_ref
            || supplied_fingerprint != value.definition_fingerprint
        {
            return Err(D::Error::custom(ContractError::DerivedValueMismatch));
        }
        Ok(value)
    }
}

#[derive(Serialize)]
struct DefinitionFingerprintPreimage<'a> {
    definition_schema: &'a ExactBindingV1,
    contract_id: &'a ContractIdV1,
    contract_version: &'a VersionV1,
    contract_ref: &'a ExactRefV1,
    contract_kind: ContractKindV1,
    claims: &'a [ContractClaimV1],
    definition_author_ref: &'a StableIdV1,
    definition_authority: &'a ExactBindingV1,
    lifecycle_policy: &'a ExactBindingV1,
    matcher_bindings: &'a [MatcherBindingV1],
    gate_policy: &'a ExactBindingV1,
    extensions_policy: &'a ExactBindingV1,
}

/// Descriptive result of comparing two immutable definition versions.
#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum DefinitionMutationClassificationV1 {
    /// Both exact definition values are identical.
    ExactSame,
    /// Semantic closure is identical; at least a patch component must advance.
    PatchMinimum,
    /// Only unweighted advisory claims were added in stable relative order.
    MinorMinimum,
    /// A breaking or unproved semantic change requires a major component.
    MajorMinimum,
}

impl DefinitionMutationClassificationV1 {
    /// Returns the minimum authoring version change, or none for exact identity.
    pub fn minimum_version_change(self) -> Option<MinimumVersionChangeV1> {
        match self {
            Self::ExactSame => None,
            Self::PatchMinimum => Some(MinimumVersionChangeV1::Patch),
            Self::MinorMinimum => Some(MinimumVersionChangeV1::Minor),
            Self::MajorMinimum => Some(MinimumVersionChangeV1::Major),
        }
    }
}

/// Classifies mutation between two exact definitions without authorizing
/// compatible substitution.
pub fn classify_definition_mutation(
    previous: &ContractDefinitionV1,
    candidate: &ContractDefinitionV1,
) -> DefinitionMutationClassificationV1 {
    if previous == candidate {
        return DefinitionMutationClassificationV1::ExactSame;
    }
    if !same_non_claim_closure(previous, candidate) {
        return DefinitionMutationClassificationV1::MajorMinimum;
    }
    if previous.claims == candidate.claims {
        return DefinitionMutationClassificationV1::PatchMinimum;
    }
    if is_unweighted_advisory_addition(&previous.claims, &candidate.claims) {
        DefinitionMutationClassificationV1::MinorMinimum
    } else {
        DefinitionMutationClassificationV1::MajorMinimum
    }
}

fn same_non_claim_closure(
    previous: &ContractDefinitionV1,
    candidate: &ContractDefinitionV1,
) -> bool {
    previous.definition_schema == candidate.definition_schema
        && previous.contract_id == candidate.contract_id
        && previous.contract_kind == candidate.contract_kind
        && previous.definition_author_ref == candidate.definition_author_ref
        && previous.definition_authority == candidate.definition_authority
        && previous.lifecycle_policy == candidate.lifecycle_policy
        && previous.matcher_bindings == candidate.matcher_bindings
        && previous.gate_policy == candidate.gate_policy
        && previous.extensions_policy == candidate.extensions_policy
}

fn is_unweighted_advisory_addition(
    previous: &[ContractClaimV1],
    candidate: &[ContractClaimV1],
) -> bool {
    if candidate.len() <= previous.len() {
        return false;
    }
    let mut prior_index = 0;
    for claim in candidate {
        if prior_index < previous.len() && claim == &previous[prior_index] {
            prior_index += 1;
        } else if claim.gate_effect() != GateEffectV1::Advisory || claim.score_weight().is_some() {
            return false;
        }
    }
    prior_index == previous.len()
}

fn normalize_matcher_bindings(
    matcher_bindings: &mut [MatcherBindingV1],
) -> Result<(), ContractError> {
    if matcher_bindings.is_empty() {
        return Err(ContractError::Empty);
    }
    if matcher_bindings.len() > MAX_MATCHER_BINDINGS {
        return Err(ContractError::BoundExceeded);
    }
    if matcher_bindings
        .iter()
        .any(|binding| !binding.is_registered())
    {
        return Err(ContractError::UnknownMatcher);
    }
    matcher_bindings
        .sort_by(|left, right| left.matcher().reference().cmp(right.matcher().reference()));
    if matcher_bindings
        .windows(2)
        .any(|window| window[0] == window[1])
    {
        return Err(ContractError::InvalidInvariant);
    }
    Ok(())
}

fn validate_claim_matcher_closure(
    claims: &[ContractClaimV1],
    matcher_bindings: &[MatcherBindingV1],
) -> Result<(), ContractError> {
    let used = claims.iter().flat_map(|claim| {
        [
            claim.subject_selector().matcher(),
            claim.case_selector().matcher(),
        ]
    });
    if used
        .clone()
        .any(|binding| !matcher_bindings.contains(binding))
        || matcher_bindings
            .iter()
            .any(|binding| !used.clone().any(|used_binding| used_binding == binding))
    {
        return Err(ContractError::InvalidInvariant);
    }
    Ok(())
}
