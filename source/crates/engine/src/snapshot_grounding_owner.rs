use crate::snapshot_grounding_repository::{SnapshotRepositoryErrorV1, SnapshotRepositoryV1};
use crate::{
    parse_schema_json, DefinitionFingerprint, ExactDefinitionRef, RetainedRepositoryRootV1,
};
use serde::Deserialize;
use serde_json::{json, Value};
use std::fmt;
use std::path::Path;

const REPOSITORY_STATE_ROOT: &str = ".handbook/state/snapshot-memory/v1";
const REPOSITORY_FORMAT_REF: &str = "handbook.snapshot-memory@1.0.0";
const POLICY_REF: &str = "handbook.snapshot-policy.production-session-boundary@1.0.0";
const GROUNDING_DEFINITION_REF: &str =
    "handbook.grounding.summary.production-session-boundary@1.0.0";
const DISCLOSURE_POLICY_REF: &str =
    "handbook.grounding.disclosure.production-session-boundary@1.0.0";
const ENGINE_AUTHORITY_REF: &str = "handbook.engine.snapshot-grounding-owner@1.0.0";
const OWNER_TYPE: &str = "SnapshotGroundingOwnerV1";
const STREAM_CLASS: &str = "session";
#[allow(
    dead_code,
    reason = "Packet 1 fixes the boundary contract before Packet 2 supplies repository-owned sequences"
)]
const MAX_SESSION_REF_BYTES: usize = 256;

const POLICY_SOURCE: &[u8] = include_bytes!("snapshot_memory/production/production-policy.json");
const GROUNDING_DEFINITION_SOURCE: &[u8] =
    include_bytes!("snapshot_memory/production/grounding-definition.json");
const DISCLOSURE_POLICY_SOURCE: &[u8] =
    include_bytes!("snapshot_memory/production/disclosure-policy.json");

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum SnapshotSourceFamilyV1 {
    Evidence,
    Git,
    Handbook,
    Session,
    Work,
}

impl SnapshotSourceFamilyV1 {
    pub const ALL: [Self; 5] = [
        Self::Evidence,
        Self::Git,
        Self::Handbook,
        Self::Session,
        Self::Work,
    ];

    pub fn as_str(self) -> &'static str {
        match self {
            Self::Evidence => "evidence",
            Self::Git => "git",
            Self::Handbook => "handbook",
            Self::Session => "session",
            Self::Work => "work",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SnapshotSourceCurrentnessV1 {
    ContractsAndDockOwnerHeads,
    RepositoryTripletStable,
    HandbookAuthorityHeadStable,
    RequestAndStreamHeadLocked,
    FlowPipelineHeadsStable,
}

impl SnapshotSourceCurrentnessV1 {
    fn as_str(self) -> &'static str {
        match self {
            Self::ContractsAndDockOwnerHeads => "contracts_and_dock_owner_heads",
            Self::RepositoryTripletStable => "repository_triplet_stable",
            Self::HandbookAuthorityHeadStable => "handbook_authority_head_stable",
            Self::RequestAndStreamHeadLocked => "request_and_stream_head_locked",
            Self::FlowPipelineHeadsStable => "flow_pipeline_heads_stable",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SnapshotAdapterFailureOutcomeV1 {
    AccountedOmission,
    Refused,
    Unavailable,
    ValidNoApplicablePrior,
}

impl SnapshotAdapterFailureOutcomeV1 {
    fn as_str(self) -> &'static str {
        match self {
            Self::AccountedOmission => "accounted_omission",
            Self::Refused => "refused",
            Self::Unavailable => "unavailable",
            Self::ValidNoApplicablePrior => "valid_no_applicable_prior",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SnapshotSourceReadFloorV1 {
    RedactOrOmitBeforePayloadRead,
}

impl SnapshotSourceReadFloorV1 {
    fn as_str(self) -> &'static str {
        match self {
            Self::RedactOrOmitBeforePayloadRead => "redact_or_omit_before_payload_read",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SnapshotAdapterFailureRuleV1 {
    condition: &'static str,
    outcome: SnapshotAdapterFailureOutcomeV1,
}

impl SnapshotAdapterFailureRuleV1 {
    pub fn condition(&self) -> &'static str {
        self.condition
    }

    pub fn outcome(&self) -> SnapshotAdapterFailureOutcomeV1 {
        self.outcome
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SnapshotSourceContractV1 {
    family: SnapshotSourceFamilyV1,
    adapter_type_name: &'static str,
    adapter_ref: &'static str,
    source_slots: &'static [&'static str],
    currentness: SnapshotSourceCurrentnessV1,
    failure_rules: &'static [SnapshotAdapterFailureRuleV1],
    read_floor: SnapshotSourceReadFloorV1,
}

impl SnapshotSourceContractV1 {
    pub fn family(&self) -> SnapshotSourceFamilyV1 {
        self.family
    }

    pub fn adapter_type_name(&self) -> &'static str {
        self.adapter_type_name
    }

    pub fn adapter_ref(&self) -> &'static str {
        self.adapter_ref
    }

    pub fn source_slots(&self) -> &'static [&'static str] {
        self.source_slots
    }

    pub fn currentness(&self) -> SnapshotSourceCurrentnessV1 {
        self.currentness
    }

    pub fn failure_rules(&self) -> &'static [SnapshotAdapterFailureRuleV1] {
        self.failure_rules
    }

    pub fn failure_outcomes(&self) -> Vec<SnapshotAdapterFailureOutcomeV1> {
        self.failure_rules.iter().map(|rule| rule.outcome).collect()
    }

    pub fn read_floor(&self) -> SnapshotSourceReadFloorV1 {
        self.read_floor
    }
}

trait SnapshotSourceAdapterV1 {
    const CONTRACT: SnapshotSourceContractV1;
}

pub(crate) struct EvidenceSnapshotSourceV1;
pub(crate) struct GitSnapshotSourceV1;
pub(crate) struct HandbookSnapshotSourceV1;
pub(crate) struct SessionBoundarySourceV1;
pub(crate) struct WorkSnapshotSourceV1;

const READ_FLOOR: SnapshotSourceReadFloorV1 =
    SnapshotSourceReadFloorV1::RedactOrOmitBeforePayloadRead;

impl SnapshotSourceAdapterV1 for EvidenceSnapshotSourceV1 {
    const CONTRACT: SnapshotSourceContractV1 = SnapshotSourceContractV1 {
        family: SnapshotSourceFamilyV1::Evidence,
        adapter_type_name: "EvidenceSnapshotSourceV1",
        adapter_ref: "handbook.snapshot-source.evidence@1.0.0",
        source_slots: &[
            "contracts_definition_lifecycle",
            "contracts_evidence_root",
            "contracts_decision",
            "dock_manifest_bundle",
            "dock_execution_candidate",
        ],
        currentness: SnapshotSourceCurrentnessV1::ContractsAndDockOwnerHeads,
        failure_rules: &[
            SnapshotAdapterFailureRuleV1 {
                condition: "absent_or_inapplicable",
                outcome: SnapshotAdapterFailureOutcomeV1::AccountedOmission,
            },
            SnapshotAdapterFailureRuleV1 {
                condition: "stale_incomplete_noncompleted_or_untrusted",
                outcome: SnapshotAdapterFailureOutcomeV1::Refused,
            },
            SnapshotAdapterFailureRuleV1 {
                condition: "unavailable_or_durability_uncertain",
                outcome: SnapshotAdapterFailureOutcomeV1::Unavailable,
            },
        ],
        read_floor: READ_FLOOR,
    };
}

impl SnapshotSourceAdapterV1 for GitSnapshotSourceV1 {
    const CONTRACT: SnapshotSourceContractV1 = SnapshotSourceContractV1 {
        family: SnapshotSourceFamilyV1::Git,
        adapter_type_name: "GitSnapshotSourceV1",
        adapter_ref: "handbook.snapshot-source.git@1.0.0",
        source_slots: &["head", "index", "worktree"],
        currentness: SnapshotSourceCurrentnessV1::RepositoryTripletStable,
        failure_rules: &[
            SnapshotAdapterFailureRuleV1 {
                condition: "policy_excluded",
                outcome: SnapshotAdapterFailureOutcomeV1::AccountedOmission,
            },
            SnapshotAdapterFailureRuleV1 {
                condition: "required_observation_changed_or_unavailable",
                outcome: SnapshotAdapterFailureOutcomeV1::Unavailable,
            },
        ],
        read_floor: READ_FLOOR,
    };
}

impl SnapshotSourceAdapterV1 for HandbookSnapshotSourceV1 {
    const CONTRACT: SnapshotSourceContractV1 = SnapshotSourceContractV1 {
        family: SnapshotSourceFamilyV1::Handbook,
        adapter_type_name: "HandbookSnapshotSourceV1",
        adapter_ref: "handbook.snapshot-source.handbook@1.0.0",
        source_slots: &["engine_authority", "profile", "schema_artifact_resolution"],
        currentness: SnapshotSourceCurrentnessV1::HandbookAuthorityHeadStable,
        failure_rules: &[
            SnapshotAdapterFailureRuleV1 {
                condition: "optional_absent",
                outcome: SnapshotAdapterFailureOutcomeV1::AccountedOmission,
            },
            SnapshotAdapterFailureRuleV1 {
                condition: "stale_substituted_or_unadmitted_authority",
                outcome: SnapshotAdapterFailureOutcomeV1::Refused,
            },
        ],
        read_floor: READ_FLOOR,
    };
}

impl SnapshotSourceAdapterV1 for SessionBoundarySourceV1 {
    const CONTRACT: SnapshotSourceContractV1 = SnapshotSourceContractV1 {
        family: SnapshotSourceFamilyV1::Session,
        adapter_type_name: "SessionBoundarySourceV1",
        adapter_ref: "handbook.snapshot-source.session@1.0.0",
        source_slots: &["boundary_context"],
        currentness: SnapshotSourceCurrentnessV1::RequestAndStreamHeadLocked,
        failure_rules: &[
            SnapshotAdapterFailureRuleV1 {
                condition: "wrong_open_closed_or_replayed_conflict_sequence",
                outcome: SnapshotAdapterFailureOutcomeV1::Refused,
            },
            SnapshotAdapterFailureRuleV1 {
                condition: "first_start",
                outcome: SnapshotAdapterFailureOutcomeV1::ValidNoApplicablePrior,
            },
        ],
        read_floor: READ_FLOOR,
    };
}

impl SnapshotSourceAdapterV1 for WorkSnapshotSourceV1 {
    const CONTRACT: SnapshotSourceContractV1 = SnapshotSourceContractV1 {
        family: SnapshotSourceFamilyV1::Work,
        adapter_type_name: "WorkSnapshotSourceV1",
        adapter_ref: "handbook.snapshot-source.work@1.0.0",
        source_slots: &["flow_state", "pipeline_state", "handoff"],
        currentness: SnapshotSourceCurrentnessV1::FlowPipelineHeadsStable,
        failure_rules: &[
            SnapshotAdapterFailureRuleV1 {
                condition: "policy_inapplicable",
                outcome: SnapshotAdapterFailureOutcomeV1::AccountedOmission,
            },
            SnapshotAdapterFailureRuleV1 {
                condition: "required_stale_or_unavailable",
                outcome: SnapshotAdapterFailureOutcomeV1::Unavailable,
            },
        ],
        read_floor: READ_FLOOR,
    };
}

const SOURCE_CONTRACTS: [SnapshotSourceContractV1; 5] = [
    EvidenceSnapshotSourceV1::CONTRACT,
    GitSnapshotSourceV1::CONTRACT,
    HandbookSnapshotSourceV1::CONTRACT,
    SessionBoundarySourceV1::CONTRACT,
    WorkSnapshotSourceV1::CONTRACT,
];

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SnapshotSourceAccountingDispositionV1 {
    RetainedObservation,
    DerivedBoundedFact,
    Redacted,
    Omitted,
    Inapplicable,
    Unavailable,
    Refused,
}

impl SnapshotSourceAccountingDispositionV1 {
    pub const ALL: [Self; 7] = [
        Self::RetainedObservation,
        Self::DerivedBoundedFact,
        Self::Redacted,
        Self::Omitted,
        Self::Inapplicable,
        Self::Unavailable,
        Self::Refused,
    ];

    fn as_str(self) -> &'static str {
        match self {
            Self::RetainedObservation => "retained_observation",
            Self::DerivedBoundedFact => "derived_bounded_fact",
            Self::Redacted => "redacted",
            Self::Omitted => "omitted",
            Self::Inapplicable => "inapplicable",
            Self::Unavailable => "unavailable",
            Self::Refused => "refused",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SnapshotGroundingErrorClassV1 {
    Refusal,
    Conflict,
    Unavailable,
    Error,
    PostCommit,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SnapshotGroundingMutationEffectV1 {
    WriteNothing,
    CommittedReceiptPreserved,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SnapshotGroundingErrorKindV1 {
    MalformedInput,
    InputLimitExceeded,
    UnknownReference,
    SubstitutedReference,
    RepositoryMismatch,
    PolicyMismatch,
    UnknownFamily,
    UnknownRule,
    AdapterMismatch,
    MatcherFailure,
    SourceDrift,
    IncompatiblePredecessor,
    SequenceConflict,
    IncompleteAccounting,
    InsufficientResolution,
    RedactionUncertain,
    RetentionConflict,
    CorruptStore,
    IdempotencyConflict,
    RequiredSourceUnavailable,
    LockUnavailable,
    RecoveryFailed,
    DurabilityUncertain,
    CompositionFailedAfterCommit,
    DeliveryUncertainAfterCommit,
}

impl SnapshotGroundingErrorKindV1 {
    pub const ALL: [Self; 25] = [
        Self::MalformedInput,
        Self::InputLimitExceeded,
        Self::UnknownReference,
        Self::SubstitutedReference,
        Self::RepositoryMismatch,
        Self::PolicyMismatch,
        Self::UnknownFamily,
        Self::UnknownRule,
        Self::AdapterMismatch,
        Self::MatcherFailure,
        Self::SourceDrift,
        Self::IncompatiblePredecessor,
        Self::SequenceConflict,
        Self::IncompleteAccounting,
        Self::InsufficientResolution,
        Self::RedactionUncertain,
        Self::RetentionConflict,
        Self::CorruptStore,
        Self::IdempotencyConflict,
        Self::RequiredSourceUnavailable,
        Self::LockUnavailable,
        Self::RecoveryFailed,
        Self::DurabilityUncertain,
        Self::CompositionFailedAfterCommit,
        Self::DeliveryUncertainAfterCommit,
    ];

    pub fn class(self) -> SnapshotGroundingErrorClassV1 {
        match self {
            Self::SequenceConflict | Self::IdempotencyConflict => {
                SnapshotGroundingErrorClassV1::Conflict
            }
            Self::RequiredSourceUnavailable | Self::LockUnavailable => {
                SnapshotGroundingErrorClassV1::Unavailable
            }
            Self::RecoveryFailed | Self::DurabilityUncertain => {
                SnapshotGroundingErrorClassV1::Error
            }
            Self::CompositionFailedAfterCommit | Self::DeliveryUncertainAfterCommit => {
                SnapshotGroundingErrorClassV1::PostCommit
            }
            _ => SnapshotGroundingErrorClassV1::Refusal,
        }
    }

    pub fn mutation_effect(self) -> SnapshotGroundingMutationEffectV1 {
        match self.class() {
            SnapshotGroundingErrorClassV1::PostCommit => {
                SnapshotGroundingMutationEffectV1::CommittedReceiptPreserved
            }
            _ => SnapshotGroundingMutationEffectV1::WriteNothing,
        }
    }

    fn as_str(self) -> &'static str {
        match self {
            Self::MalformedInput => "malformed_input",
            Self::InputLimitExceeded => "input_limit_exceeded",
            Self::UnknownReference => "unknown_reference",
            Self::SubstitutedReference => "substituted_reference",
            Self::RepositoryMismatch => "repository_mismatch",
            Self::PolicyMismatch => "policy_mismatch",
            Self::UnknownFamily => "unknown_family",
            Self::UnknownRule => "unknown_rule",
            Self::AdapterMismatch => "adapter_mismatch",
            Self::MatcherFailure => "matcher_failure",
            Self::SourceDrift => "source_drift",
            Self::IncompatiblePredecessor => "incompatible_predecessor",
            Self::SequenceConflict => "sequence_conflict",
            Self::IncompleteAccounting => "incomplete_accounting",
            Self::InsufficientResolution => "insufficient_resolution",
            Self::RedactionUncertain => "redaction_uncertain",
            Self::RetentionConflict => "retention_conflict",
            Self::CorruptStore => "corrupt_store",
            Self::IdempotencyConflict => "idempotency_conflict",
            Self::RequiredSourceUnavailable => "required_source_unavailable",
            Self::LockUnavailable => "lock_unavailable",
            Self::RecoveryFailed => "recovery_failed",
            Self::DurabilityUncertain => "durability_uncertain",
            Self::CompositionFailedAfterCommit => "composition_failed_after_commit",
            Self::DeliveryUncertainAfterCommit => "delivery_uncertain_after_commit",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SnapshotProductionSourceV1 {
    exact_ref: String,
    fingerprint: DefinitionFingerprint,
    canonical_bytes: Vec<u8>,
}

impl SnapshotProductionSourceV1 {
    pub fn exact_ref(&self) -> &str {
        &self.exact_ref
    }

    pub fn fingerprint(&self) -> &str {
        self.fingerprint.as_str()
    }

    pub fn canonical_bytes(&self) -> &[u8] {
        &self.canonical_bytes
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SnapshotProductionSourcesV1 {
    policy: SnapshotProductionSourceV1,
    grounding_definition: SnapshotProductionSourceV1,
    disclosure_policy: SnapshotProductionSourceV1,
}

impl SnapshotProductionSourcesV1 {
    pub fn policy(&self) -> &SnapshotProductionSourceV1 {
        &self.policy
    }

    pub fn grounding_definition(&self) -> &SnapshotProductionSourceV1 {
        &self.grounding_definition
    }

    pub fn disclosure_policy(&self) -> &SnapshotProductionSourceV1 {
        &self.disclosure_policy
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SnapshotBoundaryKindV1 {
    Start,
    End,
}

impl SnapshotBoundaryKindV1 {
    #[allow(
        dead_code,
        reason = "Packet 1 fixes the boundary contract before Packet 2 supplies repository-owned sequences"
    )]
    fn as_str(self) -> &'static str {
        match self {
            Self::Start => "start",
            Self::End => "end",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SnapshotGroundingKernelErrorKindV1 {
    InvalidProductionSource,
    InvalidExactBinding,
    InvalidRepositoryRoot,
    InvalidSessionRef,
    InvalidSequence,
    RepositoryMismatch,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SnapshotGroundingKernelErrorV1 {
    kind: SnapshotGroundingKernelErrorKindV1,
    detail: &'static str,
}

impl SnapshotGroundingKernelErrorV1 {
    pub fn kind(&self) -> SnapshotGroundingKernelErrorKindV1 {
        self.kind
    }

    pub fn detail(&self) -> &'static str {
        self.detail
    }
}

impl fmt::Display for SnapshotGroundingKernelErrorV1 {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.detail)
    }
}

impl std::error::Error for SnapshotGroundingKernelErrorV1 {}

pub struct SnapshotRepositoryBindingV1 {
    _retained_root: RetainedRepositoryRootV1,
    workspace_ref: String,
    workspace_fingerprint: DefinitionFingerprint,
    engine_authority_ref: String,
    engine_authority_fingerprint: DefinitionFingerprint,
    policy_ref: String,
    policy_fingerprint: DefinitionFingerprint,
    fingerprint: DefinitionFingerprint,
}

impl SnapshotRepositoryBindingV1 {
    #[allow(
        dead_code,
        reason = "Packet 2 installs the private repository seam before Packet 3 invokes it"
    )]
    pub(crate) fn retained_root(&self) -> &RetainedRepositoryRootV1 {
        &self._retained_root
    }

    pub fn state_root(&self) -> &'static str {
        REPOSITORY_STATE_ROOT
    }

    pub fn format_ref(&self) -> &'static str {
        REPOSITORY_FORMAT_REF
    }

    pub fn workspace_ref(&self) -> &str {
        &self.workspace_ref
    }

    pub fn workspace_fingerprint(&self) -> &str {
        self.workspace_fingerprint.as_str()
    }

    pub fn engine_authority_ref(&self) -> &str {
        &self.engine_authority_ref
    }

    pub fn engine_authority_fingerprint(&self) -> &str {
        self.engine_authority_fingerprint.as_str()
    }

    pub fn policy_ref(&self) -> &str {
        &self.policy_ref
    }

    pub fn policy_fingerprint(&self) -> &str {
        self.policy_fingerprint.as_str()
    }

    pub fn fingerprint(&self) -> &str {
        self.fingerprint.as_str()
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SnapshotStreamIdentityV1 {
    repository_binding_fingerprint: String,
    workspace_ref: String,
    policy_fingerprint: String,
    fingerprint: DefinitionFingerprint,
}

impl SnapshotStreamIdentityV1 {
    pub fn stream_class(&self) -> &'static str {
        STREAM_CLASS
    }

    pub fn repository_binding_fingerprint(&self) -> &str {
        &self.repository_binding_fingerprint
    }

    pub fn workspace_ref(&self) -> &str {
        &self.workspace_ref
    }

    pub fn policy_fingerprint(&self) -> &str {
        &self.policy_fingerprint
    }

    pub fn fingerprint(&self) -> &str {
        self.fingerprint.as_str()
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SnapshotBoundaryIdentityV1 {
    stream_fingerprint: String,
    kind: SnapshotBoundaryKindV1,
    sequence: u64,
    fingerprint: DefinitionFingerprint,
}

impl SnapshotBoundaryIdentityV1 {
    pub fn stream_fingerprint(&self) -> &str {
        &self.stream_fingerprint
    }

    pub fn kind(&self) -> SnapshotBoundaryKindV1 {
        self.kind
    }

    pub fn sequence(&self) -> u64 {
        self.sequence
    }

    pub fn fingerprint(&self) -> &str {
        self.fingerprint.as_str()
    }
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SnapshotGroundingOwnerV1;

impl SnapshotGroundingOwnerV1 {
    pub fn new() -> Self {
        Self
    }

    pub fn production_sources(
        &self,
    ) -> Result<SnapshotProductionSourcesV1, SnapshotGroundingKernelErrorV1> {
        let policy = admit_production_source(POLICY_SOURCE, "policy_ref", POLICY_REF)?;
        let grounding_definition = admit_production_source(
            GROUNDING_DEFINITION_SOURCE,
            "definition_ref",
            GROUNDING_DEFINITION_REF,
        )?;
        let disclosure_policy = admit_production_source(
            DISCLOSURE_POLICY_SOURCE,
            "disclosure_ref",
            DISCLOSURE_POLICY_REF,
        )?;
        validate_production_policy(&policy)?;
        validate_grounding_definition(&grounding_definition)?;
        validate_disclosure_policy(&disclosure_policy)?;
        Ok(SnapshotProductionSourcesV1 {
            policy,
            grounding_definition,
            disclosure_policy,
        })
    }

    pub fn source_contracts(&self) -> &'static [SnapshotSourceContractV1] {
        &SOURCE_CONTRACTS
    }

    pub fn bind_repository(
        &self,
        repo_root: impl AsRef<Path>,
        workspace_ref: &str,
        workspace_fingerprint: &str,
    ) -> Result<SnapshotRepositoryBindingV1, SnapshotGroundingKernelErrorV1> {
        let workspace_fingerprint = exact_binding(workspace_ref, workspace_fingerprint)?;
        let sources = self.production_sources()?;
        let policy = sources.policy();
        let engine_authority_fingerprint = fingerprint_value(json!({
            "engine_authority_ref": ENGINE_AUTHORITY_REF,
            "owner_type": OWNER_TYPE,
            "policy_fingerprint": policy.fingerprint(),
        }))?;
        let retained_root = RetainedRepositoryRootV1::open_exact(repo_root).map_err(|_| {
            kernel_error(
                SnapshotGroundingKernelErrorKindV1::InvalidRepositoryRoot,
                "snapshot repository root is unavailable or not an exact retained root",
            )
        })?;
        let physical_root_fingerprint = retained_root.opaque_physical_root_fingerprint();
        let fingerprint = fingerprint_value(json!({
            "engine_authority": {
                "fingerprint": engine_authority_fingerprint.as_str(),
                "ref": ENGINE_AUTHORITY_REF,
            },
            "policy": {
                "fingerprint": policy.fingerprint(),
                "ref": policy.exact_ref(),
            },
            "repository_format_ref": REPOSITORY_FORMAT_REF,
            "repository_physical_root_fingerprint": physical_root_fingerprint,
            "repository_state_root": REPOSITORY_STATE_ROOT,
            "workspace": {
                "fingerprint": workspace_fingerprint.as_str(),
                "ref": workspace_ref,
            }
        }))?;
        Ok(SnapshotRepositoryBindingV1 {
            _retained_root: retained_root,
            workspace_ref: workspace_ref.to_owned(),
            workspace_fingerprint,
            engine_authority_ref: ENGINE_AUTHORITY_REF.to_owned(),
            engine_authority_fingerprint,
            policy_ref: policy.exact_ref().to_owned(),
            policy_fingerprint: DefinitionFingerprint::parse(policy.fingerprint())
                .map_err(|_| invalid_production_source())?,
            fingerprint,
        })
    }

    pub fn bind_session_stream(
        &self,
        binding: &SnapshotRepositoryBindingV1,
    ) -> SnapshotStreamIdentityV1 {
        let fingerprint = fingerprint_value(json!({
            "policy_fingerprint": binding.policy_fingerprint(),
            "repository_binding_fingerprint": binding.fingerprint(),
            "stream_class": STREAM_CLASS,
            "workspace_ref": binding.workspace_ref(),
        }))
        .expect("validated repository binding always fingerprints");
        SnapshotStreamIdentityV1 {
            repository_binding_fingerprint: binding.fingerprint().to_owned(),
            workspace_ref: binding.workspace_ref().to_owned(),
            policy_fingerprint: binding.policy_fingerprint().to_owned(),
            fingerprint,
        }
    }

    #[cfg(any(test, feature = "test-support"))]
    pub fn open_repository<'binding>(
        &self,
        binding: &'binding SnapshotRepositoryBindingV1,
        stream: &SnapshotStreamIdentityV1,
    ) -> Result<SnapshotRepositoryV1<'binding>, SnapshotRepositoryErrorV1> {
        SnapshotRepositoryV1::open(*self, binding, stream)
    }

    #[cfg(not(any(test, feature = "test-support")))]
    #[allow(
        dead_code,
        reason = "Packet 2 installs owner-open lifecycle before Packet 3 invokes capture"
    )]
    pub(crate) fn open_repository<'binding>(
        &self,
        binding: &'binding SnapshotRepositoryBindingV1,
        stream: &SnapshotStreamIdentityV1,
    ) -> Result<SnapshotRepositoryV1<'binding>, SnapshotRepositoryErrorV1> {
        SnapshotRepositoryV1::open(*self, binding, stream)
    }

    #[allow(
        dead_code,
        reason = "Packet 1 fixes the boundary contract before Packet 2 supplies repository-owned sequences"
    )]
    pub(crate) fn bind_boundary(
        &self,
        binding: &SnapshotRepositoryBindingV1,
        stream: &SnapshotStreamIdentityV1,
        host_session_ref: &str,
        kind: SnapshotBoundaryKindV1,
        sequence: u64,
    ) -> Result<SnapshotBoundaryIdentityV1, SnapshotGroundingKernelErrorV1> {
        if stream.repository_binding_fingerprint != binding.fingerprint()
            || stream.workspace_ref != binding.workspace_ref()
            || stream.policy_fingerprint != binding.policy_fingerprint()
        {
            return Err(kernel_error(
                SnapshotGroundingKernelErrorKindV1::RepositoryMismatch,
                "snapshot stream does not match the retained repository binding",
            ));
        }
        if host_session_ref.is_empty()
            || host_session_ref.len() > MAX_SESSION_REF_BYTES
            || host_session_ref.chars().any(char::is_control)
        {
            return Err(kernel_error(
                SnapshotGroundingKernelErrorKindV1::InvalidSessionRef,
                "snapshot session reference is not a bounded opaque value",
            ));
        }
        if sequence == 0 {
            return Err(kernel_error(
                SnapshotGroundingKernelErrorKindV1::InvalidSequence,
                "snapshot boundary sequence must be positive",
            ));
        }
        let fingerprint = fingerprint_value(json!({
            "boundary_kind": kind.as_str(),
            "host_session_ref": host_session_ref,
            "sequence": sequence,
            "stream_fingerprint": stream.fingerprint(),
        }))?;
        Ok(SnapshotBoundaryIdentityV1 {
            stream_fingerprint: stream.fingerprint().to_owned(),
            kind,
            sequence,
            fingerprint,
        })
    }
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct AuthoredProductionPolicy {
    accounting_dispositions: Vec<String>,
    engine_authority_ref: String,
    error_kinds: Vec<String>,
    families: Vec<AuthoredSourceContract>,
    owner_type: String,
    policy_ref: String,
    redaction_stage: String,
    repository_format_ref: String,
    schema_id: String,
    schema_version: String,
    stream_class: String,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct AuthoredSourceContract {
    adapter_ref: String,
    adapter_type: String,
    currentness: String,
    failure_rules: Vec<AuthoredFailureRule>,
    family: String,
    source_slots: Vec<String>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct AuthoredFailureRule {
    condition: String,
    outcome: String,
}

fn admit_production_source(
    source: &[u8],
    ref_field: &str,
    expected_ref: &str,
) -> Result<SnapshotProductionSourceV1, SnapshotGroundingKernelErrorV1> {
    ExactDefinitionRef::parse(expected_ref).map_err(|_| invalid_production_source())?;
    let value = parse_schema_json(source).map_err(|_| invalid_production_source())?;
    if value.get(ref_field).and_then(Value::as_str) != Some(expected_ref) {
        return Err(invalid_production_source());
    }
    let canonical_bytes =
        serde_json_canonicalizer::to_vec(&value).map_err(|_| invalid_production_source())?;
    let fingerprint =
        DefinitionFingerprint::from_json_value(&value).map_err(|_| invalid_production_source())?;
    Ok(SnapshotProductionSourceV1 {
        exact_ref: expected_ref.to_owned(),
        fingerprint,
        canonical_bytes,
    })
}

fn validate_production_policy(
    source: &SnapshotProductionSourceV1,
) -> Result<(), SnapshotGroundingKernelErrorV1> {
    let value =
        parse_schema_json(source.canonical_bytes()).map_err(|_| invalid_production_source())?;
    let authored: AuthoredProductionPolicy =
        serde_json::from_value(value).map_err(|_| invalid_production_source())?;
    if authored.policy_ref != POLICY_REF
        || authored.engine_authority_ref != ENGINE_AUTHORITY_REF
        || authored.owner_type != OWNER_TYPE
        || authored.redaction_stage != READ_FLOOR.as_str()
        || authored.repository_format_ref != REPOSITORY_FORMAT_REF
        || authored.schema_id != "handbook.snapshot-policy.production-session-boundary"
        || authored.schema_version != "1.0"
        || authored.stream_class != STREAM_CLASS
        || authored.accounting_dispositions
            != SnapshotSourceAccountingDispositionV1::ALL.map(|value| value.as_str().to_owned())
        || authored.error_kinds
            != SnapshotGroundingErrorKindV1::ALL.map(|value| value.as_str().to_owned())
        || authored.families.len() != SOURCE_CONTRACTS.len()
    {
        return Err(invalid_production_source());
    }
    for (authored, expected) in authored.families.iter().zip(SOURCE_CONTRACTS) {
        if authored.family != expected.family.as_str()
            || authored.adapter_type != expected.adapter_type_name
            || authored.adapter_ref != expected.adapter_ref
            || authored.source_slots
                != expected
                    .source_slots
                    .iter()
                    .map(|slot| (*slot).to_owned())
                    .collect::<Vec<_>>()
            || authored.currentness != expected.currentness.as_str()
            || authored.failure_rules.len() != expected.failure_rules.len()
            || authored
                .failure_rules
                .iter()
                .zip(expected.failure_rules)
                .any(|(actual, expected)| {
                    actual.condition != expected.condition
                        || actual.outcome != expected.outcome.as_str()
                })
        {
            return Err(invalid_production_source());
        }
    }
    Ok(())
}

fn validate_grounding_definition(
    source: &SnapshotProductionSourceV1,
) -> Result<(), SnapshotGroundingKernelErrorV1> {
    let value =
        parse_schema_json(source.canonical_bytes()).map_err(|_| invalid_production_source())?;
    if value.get("definition_ref").and_then(Value::as_str) != Some(GROUNDING_DEFINITION_REF)
        || value.get("schema_version").and_then(Value::as_str) != Some("1.0")
        || value
            .get("source_accounting_required")
            .and_then(Value::as_str)
            != Some("exactly_once")
        || value.get("unchecked_disposition").and_then(Value::as_str) != Some("refuse")
        || value
            .get("required_currentness_families")
            .and_then(Value::as_array)
            .is_none_or(|families| {
                families.as_slice()
                    != SnapshotSourceFamilyV1::ALL
                        .map(|family| Value::String(family.as_str().to_owned()))
                        .as_slice()
            })
    {
        return Err(invalid_production_source());
    }
    Ok(())
}

fn validate_disclosure_policy(
    source: &SnapshotProductionSourceV1,
) -> Result<(), SnapshotGroundingKernelErrorV1> {
    let value =
        parse_schema_json(source.canonical_bytes()).map_err(|_| invalid_production_source())?;
    if value.get("disclosure_ref").and_then(Value::as_str) != Some(DISCLOSURE_POLICY_REF)
        || value.get("schema_version").and_then(Value::as_str) != Some("1.0")
        || value.get("default_raw_snapshot").and_then(Value::as_str) != Some("absent")
        || value.get("redaction_stage").and_then(Value::as_str) != Some(READ_FLOOR.as_str())
    {
        return Err(invalid_production_source());
    }
    Ok(())
}

fn exact_binding(
    exact_ref: &str,
    fingerprint: &str,
) -> Result<DefinitionFingerprint, SnapshotGroundingKernelErrorV1> {
    ExactDefinitionRef::parse(exact_ref).map_err(|_| {
        kernel_error(
            SnapshotGroundingKernelErrorKindV1::InvalidExactBinding,
            "snapshot binding requires an exact definition reference",
        )
    })?;
    DefinitionFingerprint::parse(fingerprint).map_err(|_| {
        kernel_error(
            SnapshotGroundingKernelErrorKindV1::InvalidExactBinding,
            "snapshot binding requires an exact canonical fingerprint",
        )
    })
}

fn fingerprint_value(
    value: Value,
) -> Result<DefinitionFingerprint, SnapshotGroundingKernelErrorV1> {
    DefinitionFingerprint::from_json_value(&value).map_err(|_| invalid_production_source())
}

fn invalid_production_source() -> SnapshotGroundingKernelErrorV1 {
    kernel_error(
        SnapshotGroundingKernelErrorKindV1::InvalidProductionSource,
        "snapshot production source is not the exact admitted canonical contract",
    )
}

fn kernel_error(
    kind: SnapshotGroundingKernelErrorKindV1,
    detail: &'static str,
) -> SnapshotGroundingKernelErrorV1 {
    SnapshotGroundingKernelErrorV1 { kind, detail }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    const WORKSPACE_REF: &str = "handbook.workspace.test@1.0.0";
    const WORKSPACE_FINGERPRINT: &str =
        "sha256:1111111111111111111111111111111111111111111111111111111111111111";

    #[test]
    fn owner_derives_deterministic_boundary_identity_from_its_sequence() {
        let repo = TempDir::new().expect("temporary retained repository");
        let owner = SnapshotGroundingOwnerV1::new();
        let binding = binding(&owner, &repo);
        let stream = owner.bind_session_stream(&binding);

        let start = owner
            .bind_boundary(
                &binding,
                &stream,
                "opaque-session-1",
                SnapshotBoundaryKindV1::Start,
                1,
            )
            .expect("owner-derived start identity");
        let repeated = owner
            .bind_boundary(
                &binding,
                &stream,
                "opaque-session-1",
                SnapshotBoundaryKindV1::Start,
                1,
            )
            .expect("deterministic owner-derived start identity");
        let end = owner
            .bind_boundary(
                &binding,
                &stream,
                "opaque-session-1",
                SnapshotBoundaryKindV1::End,
                2,
            )
            .expect("owner-derived end identity");

        assert_eq!(start.fingerprint(), repeated.fingerprint());
        assert_ne!(start.fingerprint(), end.fingerprint());
        assert_eq!(start.sequence(), 1);
        assert_eq!(end.sequence(), 2);
    }

    #[test]
    fn owner_refuses_cross_repository_streams_and_invalid_sequences() {
        let first_repo = TempDir::new().expect("first retained repository");
        let second_repo = TempDir::new().expect("second retained repository");
        let owner = SnapshotGroundingOwnerV1::new();
        let first_binding = binding(&owner, &first_repo);
        let second_binding = binding(&owner, &second_repo);
        let wrong_stream = owner.bind_session_stream(&second_binding);

        let mismatch = owner
            .bind_boundary(
                &first_binding,
                &wrong_stream,
                "opaque-session-1",
                SnapshotBoundaryKindV1::Start,
                1,
            )
            .expect_err("cross-repository stream must fail closed");
        assert_eq!(
            mismatch.kind(),
            SnapshotGroundingKernelErrorKindV1::RepositoryMismatch
        );

        let first_stream = owner.bind_session_stream(&first_binding);
        let invalid_sequence = owner
            .bind_boundary(
                &first_binding,
                &first_stream,
                "opaque-session-1",
                SnapshotBoundaryKindV1::Start,
                0,
            )
            .expect_err("caller cannot name sequence zero");
        assert_eq!(
            invalid_sequence.kind(),
            SnapshotGroundingKernelErrorKindV1::InvalidSequence
        );
    }

    fn binding(owner: &SnapshotGroundingOwnerV1, repo: &TempDir) -> SnapshotRepositoryBindingV1 {
        owner
            .bind_repository(repo.path(), WORKSPACE_REF, WORKSPACE_FINGERPRINT)
            .expect("exact repository binding")
    }
}
