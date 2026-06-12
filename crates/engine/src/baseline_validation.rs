use crate::canonical_artifacts::{
    canonical_artifact_descriptors, ArtifactIngestIssueKind, ArtifactPresence, CanonicalArtifact,
    CanonicalArtifactDescriptor, CanonicalArtifactKind, CanonicalArtifacts,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BaselineArtifactVerdict {
    Missing,
    Empty,
    StarterOwned,
    IngestInvalid,
    SemanticallyInvalid { summary: String },
    ValidCanonicalTruth { markdown: String },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BaselineArtifactValidation {
    pub kind: CanonicalArtifactKind,
    pub canonical_repo_relative_path: &'static str,
    pub packet_required: bool,
    pub verdict: BaselineArtifactVerdict,
}

pub fn baseline_artifact_validations<F>(
    artifacts: &CanonicalArtifacts,
    validate_artifact_markdown: F,
) -> Vec<BaselineArtifactValidation>
where
    F: Fn(CanonicalArtifactKind, &str) -> Result<(), String> + Copy,
{
    canonical_artifact_descriptors()
        .iter()
        .filter(|descriptor| descriptor.baseline_required)
        .map(|descriptor| {
            validation_for_descriptor(artifacts, descriptor, validate_artifact_markdown)
        })
        .collect()
}

pub fn baseline_artifact_validation<F>(
    artifacts: &CanonicalArtifacts,
    kind: CanonicalArtifactKind,
    validate_artifact_markdown: F,
) -> Option<BaselineArtifactValidation>
where
    F: Fn(CanonicalArtifactKind, &str) -> Result<(), String> + Copy,
{
    canonical_artifact_descriptors()
        .iter()
        .find(|descriptor| descriptor.baseline_required && descriptor.kind == kind)
        .map(|descriptor| {
            validation_for_descriptor(artifacts, descriptor, validate_artifact_markdown)
        })
}

pub fn baseline_artifact_validation_for_path<'a>(
    validations: &'a [BaselineArtifactValidation],
    canonical_repo_relative_path: &str,
) -> Option<&'a BaselineArtifactValidation> {
    validations
        .iter()
        .find(|validation| validation.canonical_repo_relative_path == canonical_repo_relative_path)
}

fn validation_for_descriptor<F>(
    artifacts: &CanonicalArtifacts,
    descriptor: &CanonicalArtifactDescriptor,
    validate_artifact_markdown: F,
) -> BaselineArtifactValidation
where
    F: Fn(CanonicalArtifactKind, &str) -> Result<(), String> + Copy,
{
    let artifact = canonical_artifact(artifacts, descriptor.kind);
    BaselineArtifactValidation {
        kind: descriptor.kind,
        canonical_repo_relative_path: artifact.identity.relative_path,
        packet_required: artifact.identity.packet_required,
        verdict: verdict_for_descriptor(artifacts, descriptor, validate_artifact_markdown),
    }
}

fn verdict_for_descriptor<F>(
    artifacts: &CanonicalArtifacts,
    descriptor: &CanonicalArtifactDescriptor,
    validate_artifact_markdown: F,
) -> BaselineArtifactVerdict
where
    F: Fn(CanonicalArtifactKind, &str) -> Result<(), String> + Copy,
{
    let artifact = canonical_artifact(artifacts, descriptor.kind);

    if has_ingest_issue_for_artifact(artifacts, artifact) {
        return BaselineArtifactVerdict::IngestInvalid;
    }

    match artifact.identity.presence {
        ArtifactPresence::Missing => BaselineArtifactVerdict::Missing,
        ArtifactPresence::PresentEmpty => BaselineArtifactVerdict::Empty,
        ArtifactPresence::PresentNonEmpty if artifact.identity.matches_setup_starter_template => {
            BaselineArtifactVerdict::StarterOwned
        }
        ArtifactPresence::PresentNonEmpty => {
            let markdown = match artifact.bytes.as_ref() {
                Some(bytes) => match String::from_utf8(bytes.clone()) {
                    Ok(markdown) => markdown,
                    Err(_) => {
                        return BaselineArtifactVerdict::SemanticallyInvalid {
                            summary: "canonical artifact must be valid UTF-8 markdown".to_string(),
                        };
                    }
                },
                None => {
                    return BaselineArtifactVerdict::SemanticallyInvalid {
                        summary: "canonical artifact bytes could not be loaded".to_string(),
                    };
                }
            };

            match validate_artifact_markdown(artifact.identity.kind, &markdown) {
                Ok(()) => BaselineArtifactVerdict::ValidCanonicalTruth { markdown },
                Err(summary) => BaselineArtifactVerdict::SemanticallyInvalid { summary },
            }
        }
    }
}

fn has_ingest_issue_for_artifact(
    artifacts: &CanonicalArtifacts,
    artifact: &CanonicalArtifact,
) -> bool {
    artifacts.ingest_issues.iter().any(|issue| {
        matches!(
            issue.kind,
            ArtifactIngestIssueKind::CanonicalArtifactReadError
                | ArtifactIngestIssueKind::CanonicalArtifactSymlinkNotAllowed
        ) && issue.artifact_kind == artifact.identity.kind
            && issue.canonical_repo_relative_path == artifact.identity.relative_path
    })
}

fn canonical_artifact(
    artifacts: &CanonicalArtifacts,
    kind: CanonicalArtifactKind,
) -> &CanonicalArtifact {
    match kind {
        CanonicalArtifactKind::Charter => &artifacts.charter,
        CanonicalArtifactKind::ProjectContext => &artifacts.project_context,
        CanonicalArtifactKind::EnvironmentInventory => &artifacts.environment_inventory,
        CanonicalArtifactKind::FeatureSpec => &artifacts.feature_spec,
    }
}
