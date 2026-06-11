#![forbid(unsafe_code)]

pub mod artifact_manifest;
pub mod baseline_validation;
pub mod canonical_artifacts;
pub mod freshness;
mod canonical_paths;
mod canonical_repo_support;

pub use artifact_manifest::{
    ArtifactManifest, ManifestError, ManifestInputs, ManifestVersion, SchemaVersion,
};
pub use baseline_validation::{
    baseline_artifact_validation, baseline_artifact_validation_for_path,
    baseline_artifact_validations, BaselineArtifactValidation, BaselineArtifactVerdict,
};
pub use canonical_artifacts::{
    canonical_artifact_descriptors, matches_setup_starter_template, setup_starter_template,
    setup_starter_template_bytes, ArtifactIngestError, ArtifactIngestIssue,
    ArtifactIngestIssueKind, ArtifactPresence, CanonicalArtifact, CanonicalArtifactDescriptor,
    CanonicalArtifactIdentity, CanonicalArtifactKind, CanonicalArtifacts, SystemRootStatus,
};
pub use freshness::{
    compute_freshness, FreshnessIssue, FreshnessIssueKind, FreshnessStatus, FreshnessTruth,
    InheritedDependency, OverrideTarget, OverrideWithRationale, C03_SCHEMA_VERSION,
    MANIFEST_GENERATION_VERSION,
};

pub fn workspace_contract_version() -> &'static str {
    "C-02"
}

pub fn engine_contract_version() -> &'static str {
    workspace_contract_version()
}
