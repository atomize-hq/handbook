use crate::VersionV1;
use serde::{Deserialize, Serialize};

/// Closed semantic mutation categories for definition versioning.
///
/// This matrix classifies the minimum authoring version change. It never
/// authorizes compatible-version substitution for an exact consumer pin.
#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum DefinitionChangeV1 {
    /// A non-authoritative annotation with identical semantic closure.
    PresentationOnlyAnnotation,
    /// An advisory claim with no score weight and no hard/required membership.
    AdvisoryUnweightedClaimAddition,
    /// An existing claim was added, removed, or changed otherwise.
    ExistingClaimChange,
    /// A hard-fail or required claim was added.
    HardOrRequiredClaimAddition,
    /// A subject, case, evidence, freshness, resolution, or stability rule changed.
    SelectorOrEvidenceChange,
    /// A matcher or applicability rule changed.
    MatcherOrApplicabilityChange,
    /// A lifecycle or gate-policy binding changed.
    LifecycleOrGatePolicyChange,
    /// Schema, contract kind, authority, or extension semantics changed.
    SchemaKindAuthorityOrExtensionsChange,
    /// Compatibility could not be proved from the closed matrix.
    UnprovedCompatibility,
}

impl DefinitionChangeV1 {
    /// Returns the minimum version-component change required by this mutation.
    pub fn minimum_version_change(self) -> MinimumVersionChangeV1 {
        match self {
            Self::PresentationOnlyAnnotation => MinimumVersionChangeV1::Patch,
            Self::AdvisoryUnweightedClaimAddition => MinimumVersionChangeV1::Minor,
            Self::ExistingClaimChange
            | Self::HardOrRequiredClaimAddition
            | Self::SelectorOrEvidenceChange
            | Self::MatcherOrApplicabilityChange
            | Self::LifecycleOrGatePolicyChange
            | Self::SchemaKindAuthorityOrExtensionsChange
            | Self::UnprovedCompatibility => MinimumVersionChangeV1::Major,
        }
    }
}

/// The minimum SemVer component that must advance for a semantic mutation.
#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum MinimumVersionChangeV1 {
    /// At least the patch component advances.
    Patch,
    /// At least the minor component advances.
    Minor,
    /// The major component advances.
    Major,
}

impl MinimumVersionChangeV1 {
    /// Checks only whether an authored candidate has the required version
    /// component change. It does not make the candidate substitutable.
    pub fn is_satisfied_by(self, previous: &VersionV1, candidate: &VersionV1) -> bool {
        match self {
            Self::Patch => {
                candidate.major() > previous.major()
                    || (candidate.major() == previous.major()
                        && (candidate.minor() > previous.minor()
                            || (candidate.minor() == previous.minor()
                                && candidate.patch() > previous.patch())))
            }
            Self::Minor => {
                candidate.major() > previous.major()
                    || (candidate.major() == previous.major()
                        && candidate.minor() > previous.minor())
            }
            Self::Major => candidate.major() > previous.major(),
        }
    }
}
