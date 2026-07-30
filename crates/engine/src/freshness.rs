use crate::artifact_instance::RequirednessMode;
use crate::canonical_artifacts::{
    ArtifactPresence, CanonicalArtifactIdentity, CanonicalArtifactKind,
};
use crate::profile_decision::ArtifactApplicability;
use sha2::{Digest, Sha256};

pub const C03_SCHEMA_VERSION: &str = "reduced-v1-m8";
pub const MANIFEST_GENERATION_VERSION: u32 = 1;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InheritedDependency {
    pub id: String,
    pub version: Option<String>,
    pub content_sha256: Option<String>,
}

impl Ord for InheritedDependency {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        (&self.id, &self.version, &self.content_sha256).cmp(&(
            &other.id,
            &other.version,
            &other.content_sha256,
        ))
    }
}

impl PartialOrd for InheritedDependency {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OverrideTarget {
    CanonicalArtifact(CanonicalArtifactKind),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OverrideWithRationale {
    pub target: OverrideTarget,
    pub rationale: String,
}

impl Ord for OverrideWithRationale {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        (&self.rationale, format!("{:?}", self.target))
            .cmp(&(&other.rationale, format!("{:?}", other.target)))
    }
}

impl PartialOrd for OverrideWithRationale {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FreshnessStatus {
    Ok,
    Invalid,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FreshnessIssueKind {
    RequiredArtifactMissing,
    RequiredArtifactEmpty,
    RequiredArtifactStarterTemplate,
    ForbiddenOverride,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FreshnessIssue {
    pub kind: FreshnessIssueKind,
    pub detail: String,
}

impl Ord for FreshnessIssue {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        (freshness_issue_kind_sort_key(self.kind), &self.detail)
            .cmp(&(freshness_issue_kind_sort_key(other.kind), &other.detail))
    }
}

impl PartialOrd for FreshnessIssue {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FreshnessTruth {
    pub schema_version: String,
    pub manifest_generation_version: u32,
    pub fingerprint_sha256: String,
    pub inherited_dependencies: Vec<InheritedDependency>,
    pub override_records: Vec<OverrideWithRationale>,
    pub status: FreshnessStatus,
    pub issues: Vec<FreshnessIssue>,
}

pub fn compute_freshness(
    artifacts: &[CanonicalArtifactIdentity],
    inherited_dependencies: &[InheritedDependency],
    overrides: &[OverrideWithRationale],
) -> FreshnessTruth {
    let mut sorted_artifacts: Vec<&CanonicalArtifactIdentity> = artifacts.iter().collect();
    sorted_artifacts.sort_by(|a, b| {
        (
            match a.applicability {
                ArtifactApplicability::Required => 0,
                ArtifactApplicability::Optional => 1,
                ArtifactApplicability::Indeterminate => 2,
            },
            a.instance_id.as_str(),
        )
            .cmp(&(
                match b.applicability {
                    ArtifactApplicability::Required => 0,
                    ArtifactApplicability::Optional => 1,
                    ArtifactApplicability::Indeterminate => 2,
                },
                b.instance_id.as_str(),
            ))
    });

    let mut sorted_deps = inherited_dependencies.to_vec();
    sorted_deps.sort();

    let mut sorted_overrides = overrides.to_vec();
    let override_target_instance_id = |target: OverrideTarget| match target {
        OverrideTarget::CanonicalArtifact(kind) => sorted_artifacts
            .iter()
            .find(|artifact| artifact.kind == kind)
            .map_or("unadmitted_canonical_artifact", |artifact| {
                artifact.instance_id.as_str()
            }),
    };
    sorted_overrides.sort_by(|left, right| {
        (
            override_target_instance_id(left.target),
            left.rationale.as_str(),
        )
            .cmp(&(
                override_target_instance_id(right.target),
                right.rationale.as_str(),
            ))
    });
    let sorted_override_target_instance_ids = sorted_overrides
        .iter()
        .map(|record| override_target_instance_id(record.target).to_owned())
        .collect::<Vec<_>>();

    let mut issues = Vec::new();
    for override_record in &sorted_overrides {
        match override_record.target {
            OverrideTarget::CanonicalArtifact(kind) => {
                issues.push(FreshnessIssue {
                    kind: FreshnessIssueKind::ForbiddenOverride,
                    detail: format!(
                        "override forbidden for reduced-v1 canonical artifact {kind:?}: {}",
                        override_record.rationale
                    ),
                });
            }
        }
    }

    for artifact in &sorted_artifacts {
        if !artifact.packet_required {
            continue;
        }

        match artifact.presence {
            ArtifactPresence::Missing => {
                issues.push(FreshnessIssue {
                    kind: FreshnessIssueKind::RequiredArtifactMissing,
                    detail: format!(
                        "required canonical artifact missing: {:?} at {}",
                        artifact.kind, artifact.relative_path
                    ),
                });
            }
            ArtifactPresence::PresentEmpty => {
                issues.push(FreshnessIssue {
                    kind: FreshnessIssueKind::RequiredArtifactEmpty,
                    detail: format!(
                        "required canonical artifact empty: {:?} at {}",
                        artifact.kind, artifact.relative_path
                    ),
                });
            }
            ArtifactPresence::PresentNonEmpty => {
                if artifact.matches_setup_starter_template {
                    issues.push(FreshnessIssue {
                        kind: FreshnessIssueKind::RequiredArtifactStarterTemplate,
                        detail: format!(
                            "required canonical artifact still contains the shipped starter template: {:?} at {}",
                            artifact.kind, artifact.relative_path
                        ),
                    });
                }
            }
        }
    }
    issues.sort();

    let status = if issues.is_empty() {
        FreshnessStatus::Ok
    } else {
        FreshnessStatus::Invalid
    };

    let fingerprint_sha256 = sha256_hex(&fingerprint_bytes(
        &sorted_artifacts,
        &sorted_deps,
        &sorted_overrides,
        &sorted_override_target_instance_ids,
    ));

    FreshnessTruth {
        schema_version: C03_SCHEMA_VERSION.to_string(),
        manifest_generation_version: MANIFEST_GENERATION_VERSION,
        fingerprint_sha256,
        inherited_dependencies: sorted_deps,
        override_records: sorted_overrides,
        status,
        issues,
    }
}

fn fingerprint_bytes(
    artifacts: &[&CanonicalArtifactIdentity],
    inherited_dependencies: &[InheritedDependency],
    overrides: &[OverrideWithRationale],
    override_target_instance_ids: &[String],
) -> Vec<u8> {
    let mut enc = Encoder::new();

    enc.str(C03_SCHEMA_VERSION);
    enc.u32(MANIFEST_GENERATION_VERSION);

    enc.u32(artifacts.len() as u32);
    for artifact in artifacts {
        enc.str(&artifact.instance_id);
        enc.str(&artifact.kind_ref);
        enc.str(&artifact.label);
        enc.str(&artifact.relative_path);
        enc.u8(match artifact.requiredness_mode {
            RequirednessMode::Always => 0,
            RequirednessMode::Optional => 1,
            RequirednessMode::Conditional => 2,
        });
        enc.u8(match artifact.applicability {
            ArtifactApplicability::Required => 0,
            ArtifactApplicability::Optional => 1,
            ArtifactApplicability::Indeterminate => 2,
        });
        enc.u32(artifact.renderer_definition_refs.len() as u32);
        for renderer in &artifact.renderer_definition_refs {
            enc.str(renderer);
        }
        enc.bool(artifact.packet_required);
        enc.u8(artifact_presence_sort_key(artifact.presence));
        enc.opt_str(artifact.content_sha256.as_deref());
        enc.bool(artifact.matches_setup_starter_template);
    }

    enc.u32(inherited_dependencies.len() as u32);
    for dep in inherited_dependencies {
        enc.str(&dep.id);
        enc.opt_str(dep.version.as_deref());
        enc.opt_str(dep.content_sha256.as_deref());
    }

    enc.u32(overrides.len() as u32);
    for (ov, target_instance_id) in overrides.iter().zip(override_target_instance_ids) {
        enc.str(target_instance_id);
        enc.str(&ov.rationale);
    }

    enc.finish()
}

fn sha256_hex(bytes: &[u8]) -> String {
    let digest = Sha256::digest(bytes);
    bytes_to_lower_hex(&digest)
}

fn bytes_to_lower_hex(bytes: &[u8]) -> String {
    let mut out = String::with_capacity(bytes.len() * 2);
    for b in bytes {
        use std::fmt::Write;
        let _ = write!(out, "{:02x}", b);
    }
    out
}

fn artifact_presence_sort_key(presence: ArtifactPresence) -> u8 {
    match presence {
        ArtifactPresence::Missing => 0,
        ArtifactPresence::PresentEmpty => 1,
        ArtifactPresence::PresentNonEmpty => 2,
    }
}

fn freshness_issue_kind_sort_key(kind: FreshnessIssueKind) -> u8 {
    match kind {
        FreshnessIssueKind::RequiredArtifactMissing => 0,
        FreshnessIssueKind::RequiredArtifactEmpty => 1,
        FreshnessIssueKind::RequiredArtifactStarterTemplate => 2,
        FreshnessIssueKind::ForbiddenOverride => 3,
    }
}

struct Encoder {
    out: Vec<u8>,
}

impl Encoder {
    fn new() -> Self {
        Self { out: Vec::new() }
    }

    fn finish(self) -> Vec<u8> {
        self.out
    }

    fn u8(&mut self, value: u8) {
        self.out.push(value);
    }

    fn u32(&mut self, value: u32) {
        self.out.extend_from_slice(&value.to_le_bytes());
    }

    fn u64(&mut self, value: u64) {
        self.out.extend_from_slice(&value.to_le_bytes());
    }

    fn bool(&mut self, value: bool) {
        self.u8(if value { 1 } else { 0 });
    }

    fn bytes(&mut self, bytes: &[u8]) {
        self.u64(bytes.len() as u64);
        self.out.extend_from_slice(bytes);
    }

    fn str(&mut self, value: &str) {
        self.bytes(value.as_bytes());
    }

    fn opt_str(&mut self, value: Option<&str>) {
        match value {
            Some(s) => {
                self.u8(1);
                self.str(s);
            }
            None => self.u8(0),
        }
    }
}
