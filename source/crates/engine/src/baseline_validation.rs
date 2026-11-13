use crate::canonical_artifacts::{
    ArtifactIngestIssueKind, ArtifactPresence, CanonicalArtifact, CanonicalArtifactIdentity,
    CanonicalArtifacts,
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
    pub instance_id: String,
    pub kind_ref: String,
    pub label: String,
    pub canonical_repo_relative_path: String,
    pub packet_required: bool,
    pub verdict: BaselineArtifactVerdict,
}

pub fn baseline_artifact_validations<F>(
    artifacts: &CanonicalArtifacts,
    validate_artifact_markdown: F,
) -> Vec<BaselineArtifactValidation>
where
    F: Fn(&CanonicalArtifactIdentity, &str) -> Result<(), String> + Copy,
{
    artifacts
        .artifacts
        .iter()
        .filter(|artifact| artifact.identity.baseline_required)
        .map(|artifact| validation_for_descriptor(artifacts, artifact, validate_artifact_markdown))
        .collect()
}

pub fn baseline_artifact_validation<F>(
    artifacts: &CanonicalArtifacts,
    instance_id: &str,
    validate_artifact_markdown: F,
) -> Option<BaselineArtifactValidation>
where
    F: Fn(&CanonicalArtifactIdentity, &str) -> Result<(), String> + Copy,
{
    canonical_artifact(artifacts, instance_id)
        .filter(|artifact| artifact.identity.baseline_required)
        .map(|artifact| validation_for_descriptor(artifacts, artifact, validate_artifact_markdown))
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
    artifact: &CanonicalArtifact,
    validate_artifact_markdown: F,
) -> BaselineArtifactValidation
where
    F: Fn(&CanonicalArtifactIdentity, &str) -> Result<(), String> + Copy,
{
    BaselineArtifactValidation {
        instance_id: artifact.identity.instance_id.clone(),
        kind_ref: artifact.identity.kind_ref.clone(),
        label: artifact.identity.label.clone(),
        canonical_repo_relative_path: artifact.identity.relative_path.clone(),
        packet_required: artifact.identity.packet_required,
        verdict: verdict_for_descriptor(artifacts, artifact, validate_artifact_markdown),
    }
}

fn verdict_for_descriptor<F>(
    artifacts: &CanonicalArtifacts,
    artifact: &CanonicalArtifact,
    validate_artifact_markdown: F,
) -> BaselineArtifactVerdict
where
    F: Fn(&CanonicalArtifactIdentity, &str) -> Result<(), String> + Copy,
{
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

            match validate_artifact_markdown(&artifact.identity, &markdown) {
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
        ) && issue.instance_id == artifact.identity.instance_id
            && issue.kind_ref == artifact.identity.kind_ref
            && issue.canonical_repo_relative_path == artifact.identity.relative_path
    })
}

fn canonical_artifact<'a>(
    artifacts: &'a CanonicalArtifacts,
    instance_id: &str,
) -> Option<&'a CanonicalArtifact> {
    artifacts
        .artifacts
        .iter()
        .find(|artifact| artifact.identity.instance_id == instance_id)
}
