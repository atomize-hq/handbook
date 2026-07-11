use crate::canonical_artifacts::CanonicalArtifactKind;

pub(crate) fn artifact_label(kind: CanonicalArtifactKind) -> &'static str {
    match kind {
        CanonicalArtifactKind::Charter => "CHARTER",
        CanonicalArtifactKind::ProjectContext => "PROJECT_CONTEXT",
        CanonicalArtifactKind::EnvironmentInventory => "ENVIRONMENT_INVENTORY",
        CanonicalArtifactKind::FeatureSpec => "FEATURE_SPEC",
    }
}

pub(crate) fn author_command(kind: CanonicalArtifactKind) -> &'static str {
    match kind {
        CanonicalArtifactKind::Charter => "run `handbook author charter --from-inputs <path|->`",
        CanonicalArtifactKind::ProjectContext => {
            "run `handbook author project-context --from-inputs <path|->`"
        }
        CanonicalArtifactKind::EnvironmentInventory => {
            "run `handbook author environment-inventory --from-inputs <path|->`"
        }
        CanonicalArtifactKind::FeatureSpec => {
            "fill canonical artifact at .handbook/feature_spec/FEATURE_SPEC.md"
        }
    }
}
