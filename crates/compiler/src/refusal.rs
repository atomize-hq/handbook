use serde::Serialize;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum RefusalCategory {
    NonCanonicalInputAttempt,
    SystemRootMissing,
    SystemRootNotDir,
    SystemRootSymlinkNotAllowed,
    RequiredArtifactMissing,
    RequiredArtifactEmpty,
    RequiredArtifactStarterTemplate,
    RequiredArtifactInvalid,
    ArtifactReadError,
    FreshnessInvalid,
    BudgetRefused,
    UnsupportedRequest,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(untagged)]
pub enum SubjectRef {
    CanonicalArtifact {
        instance_id: String,
        kind_ref: String,
        label: String,
        canonical_repo_relative_path: String,
    },
    InheritedDependency {
        dependency_id: String,
        version: Option<String>,
    },
    Policy {
        policy_id: &'static str,
    },
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum NextSafeAction {
    RunSetup,
    RunSetupInit,
    RunSetupRefresh,
    RunAuthorCharter,
    RunAuthorProjectContext,
    CreateSystemRoot {
        canonical_repo_relative_path: String,
    },
    EnsureSystemRootIsDirectory {
        canonical_repo_relative_path: String,
    },
    RemoveSystemRootSymlink {
        canonical_repo_relative_path: String,
    },
    CreateCanonicalArtifact {
        canonical_repo_relative_path: String,
    },
    FillCanonicalArtifact {
        canonical_repo_relative_path: String,
    },
    ReduceCanonicalArtifactSize {
        canonical_repo_relative_path: String,
    },
    RunGenerate {
        packet_id: &'static str,
    },
    RunDoctor,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Refusal {
    pub category: RefusalCategory,
    pub summary: String,
    pub broken_subject: SubjectRef,
    pub next_safe_action: NextSafeAction,
}
