use crate::canonical_artifacts::{
    ArtifactIngestError, ArtifactIngestIssue, CanonicalArtifactIdentity, CanonicalArtifacts,
    SystemRootStatus,
};
use crate::freshness::{
    compute_freshness, FreshnessTruth, InheritedDependency, OverrideWithRationale,
    C03_SCHEMA_VERSION, MANIFEST_GENERATION_VERSION,
};
use std::path::Path;

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct ManifestInputs {
    pub inherited_dependencies: Vec<InheritedDependency>,
    pub overrides: Vec<OverrideWithRationale>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SchemaVersion {
    pub contract_id: &'static str,
    pub version: &'static str,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ManifestVersion {
    pub schema: SchemaVersion,
    pub generation: u32,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ArtifactManifest {
    pub version: ManifestVersion,
    pub system_root_status: SystemRootStatus,
    pub artifacts: Vec<CanonicalArtifactIdentity>,
    pub ingest_issues: Vec<ArtifactIngestIssue>,
    pub freshness: FreshnessTruth,
}

impl ArtifactManifest {
    pub fn from_canonical_artifacts(
        artifacts: &CanonicalArtifacts,
        inputs: ManifestInputs,
    ) -> Self {
        let system_root_status = artifacts.system_root_status;
        let ingest_issues = artifacts.ingest_issues.clone();
        let ordered_identities = artifacts
            .identities()
            .map(|identity| identity.clone())
            .to_vec();

        let freshness = compute_freshness(
            &ordered_identities,
            &inputs.inherited_dependencies,
            &inputs.overrides,
        );

        Self {
            version: ManifestVersion {
                schema: SchemaVersion {
                    contract_id: "C-03",
                    version: C03_SCHEMA_VERSION,
                },
                generation: MANIFEST_GENERATION_VERSION,
            },
            system_root_status,
            artifacts: ordered_identities,
            ingest_issues,
            freshness,
        }
    }

    pub fn generate(
        repo_root: impl AsRef<Path>,
        inputs: ManifestInputs,
    ) -> Result<Self, ManifestError> {
        let artifacts =
            CanonicalArtifacts::load(repo_root.as_ref()).map_err(ManifestError::Ingest)?;

        Ok(Self::from_canonical_artifacts(&artifacts, inputs))
    }
}

#[derive(Debug)]
pub enum ManifestError {
    Ingest(ArtifactIngestError),
}

impl std::fmt::Display for ManifestError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ManifestError::Ingest(err) => write!(f, "{err}"),
        }
    }
}

impl std::error::Error for ManifestError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            ManifestError::Ingest(err) => Some(err),
        }
    }
}
