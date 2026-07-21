use crate::canonical_repo_support::{CanonicalWorkspace, RepoRelativeFileAccessError};
use crate::charter_authority_transaction::CharterAuthorityTransactionServiceV1;
use crate::charter_lifecycle::{
    apply_charter_lifecycle_events, CharterLifecycleEvent, CharterLifecycleEventKind,
    CharterLifecycleObservation, CharterLifecycleState,
};
use crate::charter_lineage_store::{
    create_new_file, create_safe_directories, reject_reparse_or_symlink, string_field,
    sync_directory, validate_record, LineageRecordClassV1, LineageStoreErrorV1,
    TrustedLineageStoreV1,
};
use crate::definition_identity::{canonical_json_bytes, parse_definition_yaml};
use crate::{
    load_shipped_charter_definition_registry, parse_schema_json, DefinitionFingerprint,
    ExactDefinitionRef,
};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::collections::{BTreeMap, BTreeSet};
use std::fmt;
use std::fs::{self, File, OpenOptions};
use std::io::{Read, Write};
use std::path::{Path, PathBuf};

pub const CHARTER_LIFECYCLE_POLICY_REF: &str =
    "handbook.lifecycle.constitutional-review-lock@1.0.0";
pub const CHARTER_REVIEW_TRIGGER_REF: &str =
    "handbook.lifecycle-trigger.charter-amendment-proposed@1.0.0";
pub const PRODUCTION_POSTURE_TRIGGER_REF: &str =
    "handbook.intake-trigger.production-posture-changed@1.0.0";
pub const TRUST_BOUNDARY_TRIGGER_REF: &str = "handbook.intake-trigger.trust-boundary-changed@1.0.0";

const CANONICAL_REF: &str = ".handbook/project/charter.yaml";
const MAX_CANONICAL_BYTES: usize = 8 * 1024 * 1024;
const MAX_JOURNAL_BYTES: usize = 1024 * 1024;
const MAX_LIFECYCLE_TRANSITIONS: usize = 4096;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CharterLifecycleAuthorityV1 {
    pub canonical_fingerprint: String,
    pub lifecycle_transition_ref: String,
    pub lifecycle_transition_fingerprint: String,
    pub state: CharterLifecycleState,
    pub state_fingerprint: String,
    pub active_observation_refs: Vec<String>,
    pub active_observation_fingerprints: Vec<String>,
    pub active_observations: Vec<CharterLifecycleObservation>,
    pub reopened_coverage_ids: Vec<String>,
}

pub(crate) struct RetainedLifecycleRecordV1 {
    pub(crate) relative_ref: String,
    pub(crate) bytes: Vec<u8>,
}

pub(crate) struct RetainedLifecycleAuthorityV1 {
    pub(crate) canonical_bytes: Option<Vec<u8>>,
    pub(crate) authority: Option<CharterLifecycleAuthorityV1>,
    pub(crate) records: Vec<RetainedLifecycleRecordV1>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum CharterLifecycleEventIntentV1 {
    CandidateAmendment {
        candidate_ref: String,
        candidate_fingerprint: String,
        observed_at_utc: String,
    },
    TriggerEvidence {
        evidence_ref: String,
        evidence_fingerprint: String,
    },
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CharterLifecycleEventDispositionV1 {
    Committed,
    Replayed,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CharterLifecycleEventCommitV1 {
    pub disposition: CharterLifecycleEventDispositionV1,
    pub observation_ref: Option<String>,
    pub transition_ref: String,
    pub state: CharterLifecycleState,
    pub state_fingerprint: String,
    pub reopened_coverage_ids: Vec<String>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CharterLifecycleStoreErrorKindV1 {
    AuthorityAbsent,
    InvalidEvent,
    UnknownTrigger,
    StaleBasis,
    LineageViolation,
    UnsafeFilesystem,
    DurabilityViolation,
    Conflict,
    UnsupportedPlatform,
    IoFailure,
    #[cfg(test)]
    InjectedFault,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CharterLifecycleStoreErrorV1 {
    kind: CharterLifecycleStoreErrorKindV1,
    detail: String,
}

impl CharterLifecycleStoreErrorV1 {
    fn new(kind: CharterLifecycleStoreErrorKindV1, detail: impl Into<String>) -> Self {
        Self {
            kind,
            detail: detail.into(),
        }
    }

    pub fn kind(&self) -> CharterLifecycleStoreErrorKindV1 {
        self.kind
    }

    pub fn detail(&self) -> &str {
        &self.detail
    }
}

impl fmt::Display for CharterLifecycleStoreErrorV1 {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.detail)
    }
}

impl std::error::Error for CharterLifecycleStoreErrorV1 {}

#[cfg(test)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum CharterLifecycleFaultPointV1 {
    Intent,
    Prepared,
    ObservationInstalled,
    RecordsInstalled,
}

#[cfg(test)]
std::thread_local! {
    static SELECTED_LIFECYCLE_FAULT_V1: std::cell::Cell<Option<CharterLifecycleFaultPointV1>> =
        const { std::cell::Cell::new(None) };
}

#[cfg(test)]
struct ScopedLifecycleFaultV1(Option<CharterLifecycleFaultPointV1>);

#[cfg(test)]
impl ScopedLifecycleFaultV1 {
    fn select(fault: CharterLifecycleFaultPointV1) -> Self {
        let previous = SELECTED_LIFECYCLE_FAULT_V1.with(|selected| selected.replace(Some(fault)));
        Self(previous)
    }
}

#[cfg(test)]
impl Drop for ScopedLifecycleFaultV1 {
    fn drop(&mut self) {
        SELECTED_LIFECYCLE_FAULT_V1.with(|selected| selected.set(self.0));
    }
}

#[cfg(test)]
fn inject_lifecycle_fault(
    boundary: CharterLifecycleFaultPointV1,
    detail: &'static str,
) -> Result<(), CharterLifecycleStoreErrorV1> {
    let selected = SELECTED_LIFECYCLE_FAULT_V1.with(std::cell::Cell::get);
    if selected == Some(boundary) {
        return Err(injected(detail));
    }
    Ok(())
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CharterLifecycleRecoveryInputsV1 {
    pub observation_final: bool,
    pub transition_final: bool,
    pub observation_staged: bool,
    pub transition_staged: bool,
    pub matching_commit_marker: bool,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CharterLifecycleRecoveryActionV1 {
    RollbackOwnedState,
    InstallBothAndCommit,
    InstallMissingAndCommit,
    RollForwardCommit,
    Finalize,
    PreserveAndRefuse,
}

pub fn classify_lifecycle_recovery(
    inputs: CharterLifecycleRecoveryInputsV1,
) -> CharterLifecycleRecoveryActionV1 {
    if inputs.matching_commit_marker {
        return if inputs.observation_final && inputs.transition_final {
            CharterLifecycleRecoveryActionV1::Finalize
        } else {
            CharterLifecycleRecoveryActionV1::PreserveAndRefuse
        };
    }
    match (inputs.observation_final, inputs.transition_final) {
        (true, true) => CharterLifecycleRecoveryActionV1::RollForwardCommit,
        (false, false) if inputs.observation_staged && inputs.transition_staged => {
            CharterLifecycleRecoveryActionV1::InstallBothAndCommit
        }
        (true, false) if inputs.transition_staged => {
            CharterLifecycleRecoveryActionV1::InstallMissingAndCommit
        }
        (false, true) if inputs.observation_staged => {
            CharterLifecycleRecoveryActionV1::InstallMissingAndCommit
        }
        _ => CharterLifecycleRecoveryActionV1::RollbackOwnedState,
    }
}

pub fn charter_lifecycle_event_commit_marker(raw_intent_bytes: &[u8]) -> Vec<u8> {
    let digest = Sha256::digest(raw_intent_bytes);
    format!("sha256:{digest:x}\n").into_bytes()
}

pub fn charter_lifecycle_state_fingerprint(
    lifecycle_policy_ref: &str,
    lifecycle_policy_fingerprint: &str,
    target_instance_id: &str,
    current_canonical_fingerprint: &str,
    result_state: CharterLifecycleState,
    active_observation_fingerprints: &[String],
) -> Result<String, CharterLifecycleStoreErrorV1> {
    let policy_ref = ExactDefinitionRef::parse(lifecycle_policy_ref)
        .map_err(|_| invalid_event("lifecycle state policy ref is not an exact definition ref"))?;
    DefinitionFingerprint::parse(lifecycle_policy_fingerprint)
        .map_err(|_| invalid_event("lifecycle state policy fingerprint is invalid"))?;
    DefinitionFingerprint::parse(current_canonical_fingerprint)
        .map_err(|_| invalid_event("lifecycle canonical fingerprint is invalid"))?;
    if target_instance_id != "project_authority" {
        return Err(invalid_event(
            "lifecycle state target instance must be project_authority",
        ));
    }
    let mut unique = BTreeSet::new();
    for fingerprint in active_observation_fingerprints {
        DefinitionFingerprint::parse(fingerprint)
            .map_err(|_| invalid_event("active observation fingerprint is invalid"))?;
        if !unique.insert(fingerprint) {
            return Err(invalid_event(
                "active lifecycle observation fingerprints must be unique",
            ));
        }
    }
    let value = json!({
        "lifecycle_policy_ref": policy_ref.as_str(),
        "lifecycle_policy_fingerprint": lifecycle_policy_fingerprint,
        "target_instance_id": target_instance_id,
        "current_canonical_fingerprint": current_canonical_fingerprint,
        "result_state": result_state,
        "active_observation_fingerprints": active_observation_fingerprints,
    });
    let bytes = canonical_json_bytes(&value).map_err(|_| {
        CharterLifecycleStoreErrorV1::new(
            CharterLifecycleStoreErrorKindV1::InvalidEvent,
            "lifecycle state fingerprint preimage could not be canonicalized",
        )
    })?;
    Ok(DefinitionFingerprint::from_bytes(&bytes).to_string())
}

#[derive(Clone, Debug)]
pub struct CharterLifecycleStoreV1 {
    repo_root: PathBuf,
    lineage: TrustedLineageStoreV1,
}

impl CharterLifecycleStoreV1 {
    pub fn new(repo_root: impl AsRef<Path>) -> Self {
        let repo_root = repo_root.as_ref().to_path_buf();
        Self {
            lineage: TrustedLineageStoreV1::new(&repo_root),
            repo_root,
        }
    }

    pub fn observe(
        &self,
    ) -> Result<Option<CharterLifecycleAuthorityV1>, CharterLifecycleStoreErrorV1> {
        ensure_supported_platform()?;
        let _lock = LifecycleLock::acquire(&self.repo_root)?;
        self.recover_pending_locked()?;
        self.load_current_locked()
            .map(|authority| authority.map(CharterLifecycleAuthorityV1::from))
    }

    pub fn record_event(
        &self,
        intent: CharterLifecycleEventIntentV1,
    ) -> Result<CharterLifecycleEventCommitV1, CharterLifecycleStoreErrorV1> {
        self.record_event_inner(intent)
    }

    #[cfg(test)]
    fn record_event_with_fault_for_testing(
        &self,
        intent: CharterLifecycleEventIntentV1,
        fault: CharterLifecycleFaultPointV1,
    ) -> Result<CharterLifecycleEventCommitV1, CharterLifecycleStoreErrorV1> {
        let _fault = ScopedLifecycleFaultV1::select(fault);
        self.record_event_inner(intent)
    }

    pub fn recover(&self) -> Result<(), CharterLifecycleStoreErrorV1> {
        ensure_supported_platform()?;
        let _lock = LifecycleLock::acquire(&self.repo_root)?;
        self.recover_pending_locked()
    }

    pub(crate) fn recover_retained_locked(&self) -> Result<(), CharterLifecycleStoreErrorV1> {
        ensure_supported_platform()?;
        self.recover_pending_locked()
    }

    pub(crate) fn observe_retained_locked(
        &self,
    ) -> Result<RetainedLifecycleAuthorityV1, CharterLifecycleStoreErrorV1> {
        ensure_supported_platform()?;
        let Some(loaded) = self.load_current_locked()? else {
            return Ok(RetainedLifecycleAuthorityV1 {
                canonical_bytes: None,
                authority: None,
                records: Vec::new(),
            });
        };
        self.retain_loaded_authority(loaded)
    }

    pub(crate) fn observe_canonical_bytes_retained_locked(
        &self,
        canonical_bytes: &[u8],
    ) -> Result<RetainedLifecycleAuthorityV1, CharterLifecycleStoreErrorV1> {
        ensure_supported_platform()?;
        let loaded = self.load_canonical_bytes_locked(canonical_bytes.to_vec())?;
        self.retain_loaded_authority(loaded)
    }

    fn retain_loaded_authority(
        &self,
        loaded: LoadedLifecycleAuthority,
    ) -> Result<RetainedLifecycleAuthorityV1, CharterLifecycleStoreErrorV1> {
        let mut records = Vec::with_capacity(1 + loaded.active_observation_refs.len());
        records.push(RetainedLifecycleRecordV1 {
            relative_ref: loaded.lifecycle_transition_ref.clone(),
            bytes: self
                .lineage
                .read_record(
                    LineageRecordClassV1::LifecycleTransition,
                    &loaded.lifecycle_transition_ref,
                    &loaded.lifecycle_transition_fingerprint,
                )
                .map_err(map_lineage)?,
        });
        for (relative_ref, fingerprint) in loaded
            .active_observation_refs
            .iter()
            .zip(&loaded.active_observation_fingerprints)
        {
            records.push(RetainedLifecycleRecordV1 {
                relative_ref: relative_ref.clone(),
                bytes: self
                    .lineage
                    .read_record(
                        LineageRecordClassV1::LifecycleObservation,
                        relative_ref,
                        fingerprint,
                    )
                    .map_err(map_lineage)?,
            });
        }
        let canonical_bytes = loaded.canonical_bytes.clone();
        Ok(RetainedLifecycleAuthorityV1 {
            canonical_bytes: Some(canonical_bytes),
            authority: Some(CharterLifecycleAuthorityV1::from(loaded)),
            records,
        })
    }

    fn record_event_inner(
        &self,
        intent: CharterLifecycleEventIntentV1,
    ) -> Result<CharterLifecycleEventCommitV1, CharterLifecycleStoreErrorV1> {
        ensure_supported_platform()?;
        let _lock = LifecycleLock::acquire(&self.repo_root)?;
        self.recover_pending_locked()?;
        let authority = self.load_current_locked()?.ok_or_else(|| {
            error(
                CharterLifecycleStoreErrorKindV1::AuthorityAbsent,
                "lifecycle events require committed canonical Charter authority",
            )
        })?;
        let resolved = self.resolve_event(&authority, intent)?;
        let transition = apply_charter_lifecycle_events(
            authority.state,
            &authority.active_observations,
            vec![resolved.event.clone()],
        )
        .map_err(|failure| {
            error(
                match failure.kind() {
                    crate::CharterLifecycleErrorKind::StaleBasis => {
                        CharterLifecycleStoreErrorKindV1::StaleBasis
                    }
                    _ => CharterLifecycleStoreErrorKindV1::InvalidEvent,
                },
                failure.detail(),
            )
        })?;
        if transition.new_observations.is_empty() {
            return Ok(CharterLifecycleEventCommitV1 {
                disposition: CharterLifecycleEventDispositionV1::Replayed,
                observation_ref: None,
                transition_ref: authority.lifecycle_transition_ref,
                state: authority.state,
                state_fingerprint: authority.state_fingerprint,
                reopened_coverage_ids: authority.reopened_coverage_ids,
            });
        }

        let definition_registry = load_shipped_charter_definition_registry()
            .map_err(|_| lineage_error("lifecycle definition closure could not be re-resolved"))?;
        let lifecycle_policy = definition_registry
            .record(&ExactDefinitionRef::parse(CHARTER_LIFECYCLE_POLICY_REF).expect("fixed ref"))
            .ok_or_else(|| lineage_error("lifecycle policy is absent from the shipped closure"))?;
        let observation_value = build_observation_record(
            &resolved,
            lifecycle_policy.definition_fingerprint().as_str(),
            &authority.canonical_fingerprint,
        )?;
        let observation_bytes = canonical_json_bytes(&observation_value)
            .map_err(|_| io_error("lifecycle observation canonicalization failed"))?;
        let observation_identity = validate_record(
            LineageRecordClassV1::LifecycleObservation,
            &observation_bytes,
        )
        .map_err(map_lineage)?;

        let mut active_records = authority
            .active_observation_refs
            .iter()
            .cloned()
            .zip(authority.active_observation_fingerprints.iter().cloned())
            .zip(authority.active_observations.iter().cloned())
            .map(
                |((reference, fingerprint), observation)| ActiveObservationRecord {
                    reference,
                    fingerprint,
                    observation,
                },
            )
            .collect::<Vec<_>>();
        active_records.push(ActiveObservationRecord {
            reference: observation_identity.relative_ref.clone(),
            fingerprint: observation_identity.fingerprint.clone(),
            observation: transition.new_observations[0].clone(),
        });
        active_records.sort_by(|left, right| {
            lifecycle_event_precedence(left.observation.kind)
                .cmp(&lifecycle_event_precedence(right.observation.kind))
                .then_with(|| {
                    left.observation
                        .event_fingerprint
                        .cmp(&right.observation.event_fingerprint)
                })
        });
        let active_fingerprints = active_records
            .iter()
            .map(|record| record.fingerprint.clone())
            .collect::<Vec<_>>();
        let result_state_fingerprint = charter_lifecycle_state_fingerprint(
            CHARTER_LIFECYCLE_POLICY_REF,
            lifecycle_policy.definition_fingerprint().as_str(),
            "project_authority",
            &authority.canonical_fingerprint,
            transition.result_state,
            &active_fingerprints,
        )?;
        let transition_value = build_transition_record(
            lifecycle_policy.definition_fingerprint().as_str(),
            authority.state,
            &authority.state_fingerprint,
            transition.result_state,
            &result_state_fingerprint,
            vec![observation_identity.relative_ref.clone()],
            active_records
                .iter()
                .map(|record| record.reference.clone())
                .collect(),
            None,
            &resolved.observed_at_utc,
        )?;
        let transition_bytes = canonical_json_bytes(&transition_value)
            .map_err(|_| io_error("lifecycle transition canonicalization failed"))?;
        let transition_identity =
            validate_record(LineageRecordClassV1::LifecycleTransition, &transition_bytes)
                .map_err(map_lineage)?;
        let event_id = format!(
            "lifecycle-event-{}",
            resolved
                .event
                .event_fingerprint
                .strip_prefix("sha256:")
                .expect("validated fingerprint")
        );
        let pending = self.pending_path(&event_id);
        let committed = self.committed_path(&event_id);
        if pending.exists() || committed.exists() {
            return Err(error(
                CharterLifecycleStoreErrorKindV1::Conflict,
                "lifecycle event transaction identity already exists",
            ));
        }
        create_safe_directories(&self.repo_root, &self.transaction_root()).map_err(map_lineage)?;
        fs::create_dir(&pending)
            .map_err(|_| io_error("lifecycle pending directory create failed"))?;
        sync_directory(&self.transaction_root()).map_err(map_lineage)?;
        let intent_record = LifecycleEventIntentRecordV1 {
            schema_id: "handbook.lifecycle-event-transaction-intent".to_owned(),
            schema_version: "1.0".to_owned(),
            transaction_id: event_id.clone(),
            event_id,
            canonical_fingerprint: authority.canonical_fingerprint.clone(),
            prior_transition_ref: authority.lifecycle_transition_ref.clone(),
            prior_transition_fingerprint: authority.lifecycle_transition_fingerprint.clone(),
            prior_state_fingerprint: authority.state_fingerprint.clone(),
            evidence_ref: resolved.event.evidence_ref.clone(),
            evidence_fingerprint: resolved.event.evidence_fingerprint.clone(),
            observation_ref: observation_identity.relative_ref.clone(),
            observation_fingerprint: observation_identity.fingerprint.clone(),
            observation_staged_fingerprint: raw_fingerprint(&observation_bytes),
            transition_ref: transition_identity.relative_ref.clone(),
            transition_fingerprint: transition_identity.fingerprint.clone(),
            transition_staged_fingerprint: raw_fingerprint(&transition_bytes),
        };
        let intent_bytes = serde_json_canonicalizer::to_vec(&intent_record)
            .map_err(|_| io_error("lifecycle intent canonicalization failed"))?;
        write_new_durable(&pending.join("intent.tmp"), &intent_bytes)?;
        rename_durable(
            &pending.join("intent.tmp"),
            &pending.join("intent.json"),
            &pending,
        )?;
        #[cfg(test)]
        inject_lifecycle_fault(
            CharterLifecycleFaultPointV1::Intent,
            "injected fault after lifecycle intent",
        )?;
        write_new_durable(&pending.join("observation.new"), &observation_bytes)?;
        write_new_durable(&pending.join("transition.new"), &transition_bytes)?;
        #[cfg(test)]
        inject_lifecycle_fault(
            CharterLifecycleFaultPointV1::Prepared,
            "injected fault after lifecycle staging",
        )?;
        self.lineage
            .append_record(
                LineageRecordClassV1::LifecycleObservation,
                &observation_bytes,
            )
            .map_err(map_lineage)?;
        #[cfg(test)]
        inject_lifecycle_fault(
            CharterLifecycleFaultPointV1::ObservationInstalled,
            "injected fault after lifecycle observation install",
        )?;
        self.lineage
            .append_record(LineageRecordClassV1::LifecycleTransition, &transition_bytes)
            .map_err(map_lineage)?;
        #[cfg(test)]
        inject_lifecycle_fault(
            CharterLifecycleFaultPointV1::RecordsInstalled,
            "injected fault after lifecycle record install",
        )?;
        self.commit_and_finalize(&pending, &committed, &intent_bytes)?;
        Ok(CharterLifecycleEventCommitV1 {
            disposition: CharterLifecycleEventDispositionV1::Committed,
            observation_ref: Some(observation_identity.relative_ref),
            transition_ref: transition_identity.relative_ref,
            state: transition.result_state,
            state_fingerprint: result_state_fingerprint,
            reopened_coverage_ids: transition.reopened_coverage_ids,
        })
    }

    fn resolve_event(
        &self,
        authority: &LoadedLifecycleAuthority,
        intent: CharterLifecycleEventIntentV1,
    ) -> Result<ResolvedLifecycleEvent, CharterLifecycleStoreErrorV1> {
        let definitions = load_shipped_charter_definition_registry()
            .map_err(|_| lineage_error("lifecycle trigger definitions could not be re-resolved"))?;
        match intent {
            CharterLifecycleEventIntentV1::TriggerEvidence {
                evidence_ref,
                evidence_fingerprint,
            } => {
                let bytes = self
                    .lineage
                    .read_record(
                        LineageRecordClassV1::TriggerEvidence,
                        &evidence_ref,
                        &evidence_fingerprint,
                    )
                    .map_err(map_lineage)?;
                let evidence = parse_schema_json(&bytes)
                    .map_err(|_| invalid_event("trigger evidence is not closed JSON"))?;
                validate_trigger_evidence(&evidence, &authority.canonical_fingerprint)?;
                let event_kind = string_field(&evidence, "event_kind").map_err(map_lineage)?;
                let (kind, trigger_ref) = match event_kind {
                    "production_posture_changed" => (
                        CharterLifecycleEventKind::ProductionPostureChanged,
                        PRODUCTION_POSTURE_TRIGGER_REF,
                    ),
                    "trust_boundary_changed" => (
                        CharterLifecycleEventKind::TrustBoundaryChanged,
                        TRUST_BOUNDARY_TRIGGER_REF,
                    ),
                    _ => {
                        return Err(error(
                            CharterLifecycleStoreErrorKindV1::UnknownTrigger,
                            "standalone trigger evidence does not map to an admitted reassessment trigger",
                        ))
                    }
                };
                let trigger = definitions
                    .record(&ExactDefinitionRef::parse(trigger_ref).expect("fixed trigger ref"))
                    .ok_or_else(|| lineage_error("mapped trigger definition is absent"))?;
                Ok(ResolvedLifecycleEvent {
                    event: CharterLifecycleEvent {
                        event_fingerprint: evidence_fingerprint,
                        basis_artifact_fingerprint: authority.canonical_fingerprint.clone(),
                        kind,
                        evidence_ref,
                        evidence_fingerprint: string_field(&evidence, "evidence_fingerprint")
                            .map_err(map_lineage)?
                            .to_owned(),
                    },
                    trigger_ref: trigger_ref.to_owned(),
                    trigger_fingerprint: trigger.definition_fingerprint().to_string(),
                    event_kind: event_kind.to_owned(),
                    observed_at_utc: string_field(&evidence, "observed_at_utc")
                        .map_err(map_lineage)?
                        .to_owned(),
                })
            }
            CharterLifecycleEventIntentV1::CandidateAmendment {
                candidate_ref,
                candidate_fingerprint,
                observed_at_utc,
            } => {
                validate_utc(&observed_at_utc)?;
                let candidate = self
                    .lineage
                    .read_record_by_ref(
                        LineageRecordClassV1::Candidate,
                        &candidate_ref,
                        Some(&candidate_fingerprint),
                    )
                    .map_err(map_lineage)?;
                if candidate
                    .get("basis_artifact_fingerprint")
                    .and_then(Value::as_str)
                    != Some(authority.canonical_fingerprint.as_str())
                {
                    return Err(error(
                        CharterLifecycleStoreErrorKindV1::StaleBasis,
                        "amendment candidate basis differs from current canonical authority",
                    ));
                }
                let content_ref =
                    string_field(&candidate, "normalized_content_ref").map_err(map_lineage)?;
                let candidate_bytes = self
                    .lineage
                    .read_candidate_content(content_ref)
                    .map_err(map_lineage)?;
                validate_amendment_change(&authority.canonical_bytes, &candidate_bytes)?;
                let trigger = definitions
                    .record(
                        &ExactDefinitionRef::parse(CHARTER_REVIEW_TRIGGER_REF)
                            .expect("fixed trigger ref"),
                    )
                    .ok_or_else(|| lineage_error("review trigger definition is absent"))?;
                Ok(ResolvedLifecycleEvent {
                    event: CharterLifecycleEvent {
                        event_fingerprint: candidate_fingerprint.clone(),
                        basis_artifact_fingerprint: authority.canonical_fingerprint.clone(),
                        kind: CharterLifecycleEventKind::Review,
                        evidence_ref: candidate_ref,
                        evidence_fingerprint: candidate_fingerprint,
                    },
                    trigger_ref: CHARTER_REVIEW_TRIGGER_REF.to_owned(),
                    trigger_fingerprint: trigger.definition_fingerprint().to_string(),
                    event_kind: "charter_amendment_proposed".to_owned(),
                    observed_at_utc,
                })
            }
        }
    }

    fn recover_pending_locked(&self) -> Result<(), CharterLifecycleStoreErrorV1> {
        let root = self.transaction_root();
        if !root.exists() {
            return Ok(());
        }
        reject_reparse_or_symlink(&root, true).map_err(map_lineage)?;
        for pending in transaction_directories(&root, ".pending")? {
            self.recover_one_pending(&pending)?;
        }
        Ok(())
    }

    fn recover_one_pending(&self, pending: &Path) -> Result<(), CharterLifecycleStoreErrorV1> {
        reject_reparse_or_symlink(pending, true).map_err(map_lineage)?;
        if !pending.join("intent.json").exists() {
            return cleanup_owned_pending(pending);
        }
        let read_intent = read_lifecycle_intent(pending)?;
        let current = self
            .load_current_locked()?
            .ok_or_else(|| durability("pending lifecycle event has no canonical authority"))?;
        if current.canonical_fingerprint != read_intent.record.canonical_fingerprint {
            return Err(durability(
                "pending lifecycle event basis is not current canonical authority",
            ));
        }
        let marker_path = pending.join("committed");
        let matching_marker = if marker_path.exists() {
            let marker = read_bounded_regular(&marker_path, 72)?;
            if marker != charter_lifecycle_event_commit_marker(&read_intent.raw_bytes) {
                return Err(durability(
                    "lifecycle event marker does not bind exact raw intent bytes",
                ));
            }
            true
        } else {
            false
        };
        let observation_staged = staged_record_exact(
            pending,
            "observation.new",
            LineageRecordClassV1::LifecycleObservation,
            &read_intent.record.observation_ref,
            &read_intent.record.observation_fingerprint,
            &read_intent.record.observation_staged_fingerprint,
        )?;
        let transition_staged = staged_record_exact(
            pending,
            "transition.new",
            LineageRecordClassV1::LifecycleTransition,
            &read_intent.record.transition_ref,
            &read_intent.record.transition_fingerprint,
            &read_intent.record.transition_staged_fingerprint,
        )?;
        let observation_final = final_record_exact(
            &self.lineage,
            LineageRecordClassV1::LifecycleObservation,
            &read_intent.record.observation_ref,
            &read_intent.record.observation_fingerprint,
        )?;
        let transition_final = final_record_exact(
            &self.lineage,
            LineageRecordClassV1::LifecycleTransition,
            &read_intent.record.transition_ref,
            &read_intent.record.transition_fingerprint,
        )?;
        let action = classify_lifecycle_recovery(CharterLifecycleRecoveryInputsV1 {
            observation_final,
            transition_final,
            observation_staged,
            transition_staged,
            matching_commit_marker: matching_marker,
        });
        if !matches!(action, CharterLifecycleRecoveryActionV1::Finalize)
            && (current.lifecycle_transition_ref != read_intent.record.prior_transition_ref
                || current.lifecycle_transition_fingerprint
                    != read_intent.record.prior_transition_fingerprint
                || current.state_fingerprint != read_intent.record.prior_state_fingerprint)
        {
            return Err(durability(
                "pending lifecycle event prior head/state is not current",
            ));
        }
        match action {
            CharterLifecycleRecoveryActionV1::RollbackOwnedState => cleanup_owned_pending(pending),
            CharterLifecycleRecoveryActionV1::InstallBothAndCommit => {
                self.install_staged_record(
                    pending,
                    "observation.new",
                    LineageRecordClassV1::LifecycleObservation,
                )?;
                self.install_staged_record(
                    pending,
                    "transition.new",
                    LineageRecordClassV1::LifecycleTransition,
                )?;
                self.commit_and_finalize(
                    pending,
                    &self.committed_path(&read_intent.record.transaction_id),
                    &read_intent.raw_bytes,
                )
            }
            CharterLifecycleRecoveryActionV1::InstallMissingAndCommit => {
                if !observation_final {
                    self.install_staged_record(
                        pending,
                        "observation.new",
                        LineageRecordClassV1::LifecycleObservation,
                    )?;
                }
                if !transition_final {
                    self.install_staged_record(
                        pending,
                        "transition.new",
                        LineageRecordClassV1::LifecycleTransition,
                    )?;
                }
                self.commit_and_finalize(
                    pending,
                    &self.committed_path(&read_intent.record.transaction_id),
                    &read_intent.raw_bytes,
                )
            }
            CharterLifecycleRecoveryActionV1::RollForwardCommit => self.commit_and_finalize(
                pending,
                &self.committed_path(&read_intent.record.transaction_id),
                &read_intent.raw_bytes,
            ),
            CharterLifecycleRecoveryActionV1::Finalize => {
                let committed = self.committed_path(&read_intent.record.transaction_id);
                if committed.exists() {
                    return Err(durability(
                        "lifecycle committed journal destination already exists",
                    ));
                }
                fs::rename(pending, &committed)
                    .map_err(|_| io_error("lifecycle recovery finalization failed"))?;
                sync_directory(&self.transaction_root()).map_err(map_lineage)
            }
            CharterLifecycleRecoveryActionV1::PreserveAndRefuse => Err(durability(
                "committed lifecycle marker is missing a required final record",
            )),
        }
    }

    fn install_staged_record(
        &self,
        pending: &Path,
        name: &str,
        class: LineageRecordClassV1,
    ) -> Result<(), CharterLifecycleStoreErrorV1> {
        let bytes = read_bounded_regular(&pending.join(name), MAX_JOURNAL_BYTES)?;
        self.lineage
            .append_record(class, &bytes)
            .map_err(map_lineage)?;
        Ok(())
    }

    fn load_current_locked(
        &self,
    ) -> Result<Option<LoadedLifecycleAuthority>, CharterLifecycleStoreErrorV1> {
        let Some(canonical_bytes) = read_current_canonical(&self.repo_root)? else {
            if self.transaction_root().exists()
                && !transaction_directories(&self.transaction_root(), ".committed")?.is_empty()
            {
                return Err(durability(
                    "lifecycle journals exist without canonical Charter authority",
                ));
            }
            return Ok(None);
        };
        self.load_canonical_bytes_locked(canonical_bytes).map(Some)
    }

    fn load_canonical_bytes_locked(
        &self,
        canonical_bytes: Vec<u8>,
    ) -> Result<LoadedLifecycleAuthority, CharterLifecycleStoreErrorV1> {
        let canonical_fingerprint = DefinitionFingerprint::from_bytes(&canonical_bytes).to_string();
        let promotion = self.current_promotion_anchor(&canonical_fingerprint)?;
        let mut transition_ref = promotion.lifecycle_transition_ref;
        let mut transition_fingerprint = promotion.lifecycle_transition_fingerprint;
        let mut committed_by_prior: BTreeMap<String, Vec<LifecycleEventIntentRecordV1>> =
            BTreeMap::new();
        let root = self.transaction_root();
        if root.exists() {
            for committed in transaction_directories(&root, ".committed")? {
                let intent = read_lifecycle_intent(&committed)?;
                if intent.record.canonical_fingerprint != canonical_fingerprint {
                    continue;
                }
                let marker = read_bounded_regular(&committed.join("committed"), 72)?;
                if marker != charter_lifecycle_event_commit_marker(&intent.raw_bytes) {
                    return Err(durability("committed lifecycle journal marker is invalid"));
                }
                self.lineage
                    .read_record(
                        LineageRecordClassV1::LifecycleObservation,
                        &intent.record.observation_ref,
                        &intent.record.observation_fingerprint,
                    )
                    .map_err(map_lineage)?;
                self.lineage
                    .read_record(
                        LineageRecordClassV1::LifecycleTransition,
                        &intent.record.transition_ref,
                        &intent.record.transition_fingerprint,
                    )
                    .map_err(map_lineage)?;
                committed_by_prior
                    .entry(intent.record.prior_transition_ref.clone())
                    .or_default()
                    .push(intent.record);
            }
        }
        for _ in 0..MAX_LIFECYCLE_TRANSITIONS {
            let Some(successors) = committed_by_prior.remove(&transition_ref) else {
                break;
            };
            if successors.len() != 1 {
                return Err(durability(
                    "committed lifecycle history contains a forked prior head",
                ));
            }
            let successor = &successors[0];
            if successor.prior_transition_fingerprint != transition_fingerprint {
                return Err(durability(
                    "committed lifecycle history prior fingerprint is discontinuous",
                ));
            }
            transition_ref = successor.transition_ref.clone();
            transition_fingerprint = successor.transition_fingerprint.clone();
        }
        if !committed_by_prior.is_empty() {
            return Err(durability(
                "committed lifecycle history contains unreachable transitions or exceeds its bound",
            ));
        }
        let transition_bytes = self
            .lineage
            .read_record(
                LineageRecordClassV1::LifecycleTransition,
                &transition_ref,
                &transition_fingerprint,
            )
            .map_err(map_lineage)?;
        let transition = parse_schema_json(&transition_bytes)
            .map_err(|_| lineage_error("lifecycle transition is not closed JSON"))?;
        validate_transition_shape(&transition)?;
        let state = parse_state(string_field(&transition, "result_state").map_err(map_lineage)?)?;
        let state_fingerprint = string_field(&transition, "result_state_fingerprint")
            .map_err(map_lineage)?
            .to_owned();
        let active_refs = string_array(&transition, "active_observation_refs")?;
        let mut active_observations = Vec::new();
        let mut active_fingerprints = Vec::new();
        for reference in &active_refs {
            let fingerprint = fingerprint_from_ref(reference).ok_or_else(|| {
                lineage_error("active lifecycle observation ref does not carry its fingerprint")
            })?;
            let bytes = self
                .lineage
                .read_record(
                    LineageRecordClassV1::LifecycleObservation,
                    reference,
                    &fingerprint,
                )
                .map_err(map_lineage)?;
            let value = parse_schema_json(&bytes)
                .map_err(|_| lineage_error("lifecycle observation is not closed JSON"))?;
            validate_observation_shape(&value, &canonical_fingerprint)?;
            active_observations.push(observation_from_record(&value)?);
            active_fingerprints.push(fingerprint);
        }
        let policy_fingerprint =
            string_field(&transition, "lifecycle_policy_fingerprint").map_err(map_lineage)?;
        let expected_state = charter_lifecycle_state_fingerprint(
            CHARTER_LIFECYCLE_POLICY_REF,
            policy_fingerprint,
            "project_authority",
            &canonical_fingerprint,
            state,
            &active_fingerprints,
        )?;
        if expected_state != state_fingerprint {
            return Err(durability(
                "lifecycle result-state fingerprint does not match exact current authority",
            ));
        }
        let reopened_coverage_ids = active_observations
            .iter()
            .flat_map(|observation| observation.reopened_coverage_ids.iter().cloned())
            .collect::<BTreeSet<_>>()
            .into_iter()
            .collect();
        Ok(LoadedLifecycleAuthority {
            canonical_bytes,
            canonical_fingerprint,
            lifecycle_transition_ref: transition_ref,
            lifecycle_transition_fingerprint: transition_fingerprint,
            state,
            state_fingerprint,
            active_observation_refs: active_refs,
            active_observation_fingerprints: active_fingerprints,
            active_observations,
            reopened_coverage_ids,
        })
    }

    fn current_promotion_anchor(
        &self,
        canonical_fingerprint: &str,
    ) -> Result<PromotionLifecycleAnchor, CharterLifecycleStoreErrorV1> {
        let intent = CharterAuthorityTransactionServiceV1::new(&self.repo_root)
            .current_committed_intent_locked(canonical_fingerprint)
            .map_err(|failure| {
                durability(format!(
                    "promotion terminal history refused: {}",
                    failure.detail()
                ))
            })?;
        let output = intent.record.outputs.lifecycle_transition;
        Ok(PromotionLifecycleAnchor {
            lifecycle_transition_ref: output.record_ref,
            lifecycle_transition_fingerprint: output.record_fingerprint,
        })
    }

    fn commit_and_finalize(
        &self,
        pending: &Path,
        committed: &Path,
        intent_bytes: &[u8],
    ) -> Result<(), CharterLifecycleStoreErrorV1> {
        remove_owned_progress_marker(pending)?;
        write_new_durable(
            &pending.join("committed.tmp"),
            &charter_lifecycle_event_commit_marker(intent_bytes),
        )?;
        rename_durable(
            &pending.join("committed.tmp"),
            &pending.join("committed"),
            pending,
        )?;
        fs::rename(pending, committed)
            .map_err(|_| io_error("lifecycle journal finalization failed"))?;
        sync_directory(&self.transaction_root()).map_err(map_lineage)
    }

    fn transaction_root(&self) -> PathBuf {
        self.repo_root
            .join(".handbook/state/transactions/lifecycle-events")
    }

    fn pending_path(&self, transaction_id: &str) -> PathBuf {
        self.transaction_root()
            .join(format!("{transaction_id}.pending"))
    }

    fn committed_path(&self, transaction_id: &str) -> PathBuf {
        self.transaction_root()
            .join(format!("{transaction_id}.committed"))
    }
}

impl From<LoadedLifecycleAuthority> for CharterLifecycleAuthorityV1 {
    fn from(value: LoadedLifecycleAuthority) -> Self {
        Self {
            canonical_fingerprint: value.canonical_fingerprint,
            lifecycle_transition_ref: value.lifecycle_transition_ref,
            lifecycle_transition_fingerprint: value.lifecycle_transition_fingerprint,
            state: value.state,
            state_fingerprint: value.state_fingerprint,
            active_observation_refs: value.active_observation_refs,
            active_observation_fingerprints: value.active_observation_fingerprints,
            active_observations: value.active_observations,
            reopened_coverage_ids: value.reopened_coverage_ids,
        }
    }
}

#[derive(Clone, Debug)]
struct LoadedLifecycleAuthority {
    canonical_bytes: Vec<u8>,
    canonical_fingerprint: String,
    lifecycle_transition_ref: String,
    lifecycle_transition_fingerprint: String,
    state: CharterLifecycleState,
    state_fingerprint: String,
    active_observation_refs: Vec<String>,
    active_observation_fingerprints: Vec<String>,
    active_observations: Vec<CharterLifecycleObservation>,
    reopened_coverage_ids: Vec<String>,
}

#[derive(Clone, Debug)]
struct PromotionLifecycleAnchor {
    lifecycle_transition_ref: String,
    lifecycle_transition_fingerprint: String,
}

#[derive(Clone, Debug)]
struct ResolvedLifecycleEvent {
    event: CharterLifecycleEvent,
    trigger_ref: String,
    trigger_fingerprint: String,
    event_kind: String,
    observed_at_utc: String,
}

#[derive(Clone, Debug)]
struct ActiveObservationRecord {
    reference: String,
    fingerprint: String,
    observation: CharterLifecycleObservation,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
struct LifecycleEventIntentRecordV1 {
    schema_id: String,
    schema_version: String,
    transaction_id: String,
    event_id: String,
    canonical_fingerprint: String,
    prior_transition_ref: String,
    prior_transition_fingerprint: String,
    prior_state_fingerprint: String,
    evidence_ref: String,
    evidence_fingerprint: String,
    observation_ref: String,
    observation_fingerprint: String,
    observation_staged_fingerprint: String,
    transition_ref: String,
    transition_fingerprint: String,
    transition_staged_fingerprint: String,
}

struct ReadLifecycleIntent {
    record: LifecycleEventIntentRecordV1,
    raw_bytes: Vec<u8>,
}

struct LifecycleLock {
    file: File,
}

impl LifecycleLock {
    fn acquire(repo_root: &Path) -> Result<Self, CharterLifecycleStoreErrorV1> {
        let root = repo_root.join(".handbook/state/locks");
        create_safe_directories(repo_root, &root).map_err(map_lineage)?;
        let file = open_lock_file(&root.join("lifecycle.lock"))?;
        file.lock()
            .map_err(|_| io_error("lifecycle lock acquisition failed"))?;
        Ok(Self { file })
    }
}

impl Drop for LifecycleLock {
    fn drop(&mut self) {
        let _ = self.file.unlock();
    }
}

fn build_observation_record(
    resolved: &ResolvedLifecycleEvent,
    policy_fingerprint: &str,
    canonical_fingerprint: &str,
) -> Result<Value, CharterLifecycleStoreErrorV1> {
    finalize_content_addressed_record(
        json!({
            "schema_id": "handbook.lifecycle-observation",
            "schema_version": "1.0",
            "lifecycle_policy_ref": CHARTER_LIFECYCLE_POLICY_REF,
            "lifecycle_policy_fingerprint": policy_fingerprint,
            "target_instance_id": "project_authority",
            "basis_canonical_fingerprint": canonical_fingerprint,
            "trigger_ref": resolved.trigger_ref,
            "trigger_fingerprint": resolved.trigger_fingerprint,
            "evidence_ref": resolved.event.evidence_ref,
            "evidence_fingerprint": resolved.event.evidence_fingerprint,
            "event_kind": resolved.event_kind,
            "observed_at_utc": resolved.observed_at_utc,
        }),
        "observation_id",
        "observation_fingerprint",
        "lifecycle-observation",
        &[],
    )
}

#[allow(clippy::too_many_arguments)]
pub(crate) fn build_transition_record(
    policy_fingerprint: &str,
    prior_state: CharterLifecycleState,
    prior_state_fingerprint: &str,
    result_state: CharterLifecycleState,
    result_state_fingerprint: &str,
    new_observation_refs: Vec<String>,
    active_observation_refs: Vec<String>,
    clearance_promotion_ref: Option<String>,
    transitioned_at_utc: &str,
) -> Result<Value, CharterLifecycleStoreErrorV1> {
    validate_utc(transitioned_at_utc)?;
    finalize_content_addressed_record(
        json!({
            "schema_id": "handbook.lifecycle-transition",
            "schema_version": "1.0",
            "lifecycle_policy_ref": CHARTER_LIFECYCLE_POLICY_REF,
            "lifecycle_policy_fingerprint": policy_fingerprint,
            "target_instance_id": "project_authority",
            "prior_state": prior_state,
            "prior_state_fingerprint": prior_state_fingerprint,
            "new_observation_refs": new_observation_refs,
            "active_observation_refs": active_observation_refs,
            "result_state": result_state,
            "result_state_fingerprint": result_state_fingerprint,
            "clearance_promotion_ref": clearance_promotion_ref,
            "transitioned_at_utc": transitioned_at_utc,
        }),
        "transition_id",
        "transition_fingerprint",
        "lifecycle-transition",
        &["transitioned_at_utc"],
    )
}

pub(crate) fn finalize_content_addressed_record(
    mut value: Value,
    id_field: &str,
    fingerprint_field: &str,
    id_prefix: &str,
    audit_only_fields: &[&str],
) -> Result<Value, CharterLifecycleStoreErrorV1> {
    let mut preimage = value.clone();
    let object = preimage
        .as_object_mut()
        .ok_or_else(|| invalid_event("runtime record must have an object root"))?;
    object.remove(id_field);
    object.remove(fingerprint_field);
    for field in audit_only_fields {
        object.remove(*field);
    }
    let fingerprint = DefinitionFingerprint::from_json_value(&preimage)
        .map_err(|_| invalid_event("runtime record fingerprint preimage is invalid"))?
        .to_string();
    let record_id = format!(
        "{id_prefix}_{}",
        fingerprint
            .strip_prefix("sha256:")
            .expect("fingerprint is normalized")
    );
    let target = value
        .as_object_mut()
        .ok_or_else(|| invalid_event("runtime record must have an object root"))?;
    target.insert(id_field.to_owned(), Value::String(record_id));
    target.insert(fingerprint_field.to_owned(), Value::String(fingerprint));
    Ok(value)
}

fn validate_trigger_evidence(
    evidence: &Value,
    expected_basis: &str,
) -> Result<(), CharterLifecycleStoreErrorV1> {
    require_exact_keys(
        evidence,
        &[
            "schema_id",
            "schema_version",
            "event_id",
            "target_instance_id",
            "basis_canonical_fingerprint",
            "event_kind",
            "prior_evidence_ref",
            "prior_evidence_fingerprint",
            "current_evidence_ref",
            "current_evidence_fingerprint",
            "observed_at_utc",
            "extensions",
            "evidence_fingerprint",
        ],
        "trigger evidence",
    )?;
    if string_field(evidence, "schema_id").map_err(map_lineage)?
        != "handbook.lifecycle-trigger-evidence"
        || string_field(evidence, "schema_version").map_err(map_lineage)? != "1.0"
        || string_field(evidence, "target_instance_id").map_err(map_lineage)? != "project_authority"
    {
        return Err(invalid_event("trigger evidence identity is invalid"));
    }
    if string_field(evidence, "basis_canonical_fingerprint").map_err(map_lineage)? != expected_basis
    {
        return Err(error(
            CharterLifecycleStoreErrorKindV1::StaleBasis,
            "trigger evidence basis differs from current canonical authority",
        ));
    }
    let prior = string_field(evidence, "prior_evidence_fingerprint").map_err(map_lineage)?;
    let current = string_field(evidence, "current_evidence_fingerprint").map_err(map_lineage)?;
    DefinitionFingerprint::parse(prior)
        .map_err(|_| invalid_event("prior evidence fingerprint is invalid"))?;
    DefinitionFingerprint::parse(current)
        .map_err(|_| invalid_event("current evidence fingerprint is invalid"))?;
    if prior == current {
        return Err(invalid_event(
            "trigger evidence prior and current fingerprints must differ",
        ));
    }
    let extensions = evidence
        .get("extensions")
        .and_then(Value::as_object)
        .ok_or_else(|| invalid_event("trigger evidence extensions must be an object"))?;
    if !extensions.is_empty() {
        return Err(invalid_event("trigger evidence extensions must be empty"));
    }
    validate_utc(string_field(evidence, "observed_at_utc").map_err(map_lineage)?)
}

fn validate_amendment_change(
    canonical_bytes: &[u8],
    candidate_bytes: &[u8],
) -> Result<(), CharterLifecycleStoreErrorV1> {
    let canonical = parse_definition_yaml(canonical_bytes)
        .map_err(|_| invalid_event("current canonical Charter is invalid YAML"))?;
    let candidate = parse_definition_yaml(candidate_bytes)
        .map_err(|_| invalid_event("candidate Charter is invalid YAML"))?;
    let changed = ["policy", "governance", "engineering_posture"]
        .iter()
        .any(|field| canonical.get(*field) != candidate.get(*field));
    if !changed {
        return Err(error(
            CharterLifecycleStoreErrorKindV1::UnknownTrigger,
            "candidate does not change a review-triggered constitutional subtree",
        ));
    }
    Ok(())
}

fn validate_observation_shape(
    value: &Value,
    canonical_fingerprint: &str,
) -> Result<(), CharterLifecycleStoreErrorV1> {
    require_exact_keys(
        value,
        &[
            "schema_id",
            "schema_version",
            "observation_id",
            "lifecycle_policy_ref",
            "lifecycle_policy_fingerprint",
            "target_instance_id",
            "basis_canonical_fingerprint",
            "trigger_ref",
            "trigger_fingerprint",
            "evidence_ref",
            "evidence_fingerprint",
            "event_kind",
            "observed_at_utc",
            "observation_fingerprint",
        ],
        "lifecycle observation",
    )?;
    if string_field(value, "basis_canonical_fingerprint").map_err(map_lineage)?
        != canonical_fingerprint
        || string_field(value, "lifecycle_policy_ref").map_err(map_lineage)?
            != CHARTER_LIFECYCLE_POLICY_REF
        || string_field(value, "target_instance_id").map_err(map_lineage)? != "project_authority"
        || string_field(value, "schema_id").map_err(map_lineage)?
            != "handbook.lifecycle-observation"
        || string_field(value, "schema_version").map_err(map_lineage)? != "1.0"
    {
        return Err(lineage_error(
            "lifecycle observation does not bind current policy, instance, and basis",
        ));
    }
    validate_utc(string_field(value, "observed_at_utc").map_err(map_lineage)?)?;
    let definitions = load_shipped_charter_definition_registry()
        .map_err(|_| lineage_error("stored lifecycle definitions could not be re-resolved"))?;
    let policy = definitions
        .record(&ExactDefinitionRef::parse(CHARTER_LIFECYCLE_POLICY_REF).expect("fixed ref"))
        .ok_or_else(|| lineage_error("stored lifecycle policy definition is absent"))?;
    if string_field(value, "lifecycle_policy_fingerprint").map_err(map_lineage)?
        != policy.definition_fingerprint().as_str()
    {
        return Err(lineage_error(
            "stored lifecycle observation policy fingerprint is stale",
        ));
    }
    let expected_trigger_ref = match string_field(value, "event_kind").map_err(map_lineage)? {
        "charter_amendment_proposed" => CHARTER_REVIEW_TRIGGER_REF,
        "production_posture_changed" => PRODUCTION_POSTURE_TRIGGER_REF,
        "trust_boundary_changed" => TRUST_BOUNDARY_TRIGGER_REF,
        _ => {
            return Err(error(
                CharterLifecycleStoreErrorKindV1::UnknownTrigger,
                "stored lifecycle observation has an unknown event kind",
            ))
        }
    };
    let trigger = definitions
        .record(&ExactDefinitionRef::parse(expected_trigger_ref).expect("fixed ref"))
        .ok_or_else(|| lineage_error("stored lifecycle trigger definition is absent"))?;
    if string_field(value, "trigger_ref").map_err(map_lineage)? != expected_trigger_ref
        || string_field(value, "trigger_fingerprint").map_err(map_lineage)?
            != trigger.definition_fingerprint().as_str()
    {
        return Err(lineage_error(
            "stored lifecycle observation trigger definition is stale or substituted",
        ));
    }
    Ok(())
}

fn validate_transition_shape(value: &Value) -> Result<(), CharterLifecycleStoreErrorV1> {
    require_exact_keys(
        value,
        &[
            "schema_id",
            "schema_version",
            "transition_id",
            "lifecycle_policy_ref",
            "lifecycle_policy_fingerprint",
            "target_instance_id",
            "prior_state",
            "prior_state_fingerprint",
            "new_observation_refs",
            "active_observation_refs",
            "result_state",
            "result_state_fingerprint",
            "clearance_promotion_ref",
            "transitioned_at_utc",
            "transition_fingerprint",
        ],
        "lifecycle transition",
    )?;
    if string_field(value, "schema_id").map_err(map_lineage)? != "handbook.lifecycle-transition"
        || string_field(value, "schema_version").map_err(map_lineage)? != "1.0"
        || string_field(value, "lifecycle_policy_ref").map_err(map_lineage)?
            != CHARTER_LIFECYCLE_POLICY_REF
        || string_field(value, "target_instance_id").map_err(map_lineage)? != "project_authority"
    {
        return Err(lineage_error(
            "lifecycle transition identity, policy, or target is invalid",
        ));
    }
    validate_utc(string_field(value, "transitioned_at_utc").map_err(map_lineage)?)?;
    let definitions = load_shipped_charter_definition_registry()
        .map_err(|_| lineage_error("stored lifecycle definitions could not be re-resolved"))?;
    let policy = definitions
        .record(&ExactDefinitionRef::parse(CHARTER_LIFECYCLE_POLICY_REF).expect("fixed ref"))
        .ok_or_else(|| lineage_error("stored lifecycle policy definition is absent"))?;
    if string_field(value, "lifecycle_policy_fingerprint").map_err(map_lineage)?
        != policy.definition_fingerprint().as_str()
    {
        return Err(lineage_error(
            "stored lifecycle transition policy fingerprint is stale",
        ));
    }
    parse_state(string_field(value, "prior_state").map_err(map_lineage)?)?;
    parse_state(string_field(value, "result_state").map_err(map_lineage)?)?;
    string_array(value, "new_observation_refs")?;
    string_array(value, "active_observation_refs")?;
    Ok(())
}

fn observation_from_record(
    value: &Value,
) -> Result<CharterLifecycleObservation, CharterLifecycleStoreErrorV1> {
    let event_kind = string_field(value, "event_kind").map_err(map_lineage)?;
    let kind = match event_kind {
        "charter_amendment_proposed" => CharterLifecycleEventKind::Review,
        "production_posture_changed" => CharterLifecycleEventKind::ProductionPostureChanged,
        "trust_boundary_changed" => CharterLifecycleEventKind::TrustBoundaryChanged,
        _ => {
            return Err(error(
                CharterLifecycleStoreErrorKindV1::UnknownTrigger,
                "stored lifecycle observation has an unknown event kind",
            ))
        }
    };
    Ok(CharterLifecycleObservation {
        event_fingerprint: string_field(value, "evidence_fingerprint")
            .map_err(map_lineage)?
            .to_owned(),
        basis_artifact_fingerprint: string_field(value, "basis_canonical_fingerprint")
            .map_err(map_lineage)?
            .to_owned(),
        kind,
        evidence_ref: string_field(value, "evidence_ref")
            .map_err(map_lineage)?
            .to_owned(),
        evidence_fingerprint: string_field(value, "evidence_fingerprint")
            .map_err(map_lineage)?
            .to_owned(),
        reopened_coverage_ids: reopened_coverage(kind)
            .iter()
            .map(|value| (*value).to_owned())
            .collect(),
    })
}

fn reopened_coverage(kind: CharterLifecycleEventKind) -> &'static [&'static str] {
    match kind {
        CharterLifecycleEventKind::Review => &[],
        CharterLifecycleEventKind::ProductionPostureChanged => {
            &["operational_reality.production_state"]
        }
        CharterLifecycleEventKind::TrustBoundaryChanged => &["governance.exception_policy"],
    }
}

fn lifecycle_event_precedence(kind: CharterLifecycleEventKind) -> u8 {
    match kind {
        CharterLifecycleEventKind::ProductionPostureChanged
        | CharterLifecycleEventKind::TrustBoundaryChanged => 0,
        CharterLifecycleEventKind::Review => 1,
    }
}

fn parse_state(value: &str) -> Result<CharterLifecycleState, CharterLifecycleStoreErrorV1> {
    match value {
        "current" => Ok(CharterLifecycleState::Current),
        "review_required" => Ok(CharterLifecycleState::ReviewRequired),
        "reassessment_required" => Ok(CharterLifecycleState::ReassessmentRequired),
        _ => Err(lineage_error("lifecycle state is not admitted")),
    }
}

fn require_exact_keys(
    value: &Value,
    expected: &[&str],
    label: &str,
) -> Result<(), CharterLifecycleStoreErrorV1> {
    let object = value
        .as_object()
        .ok_or_else(|| invalid_event(format!("{label} must be an object")))?;
    let observed = object.keys().map(String::as_str).collect::<BTreeSet<_>>();
    let expected = expected.iter().copied().collect::<BTreeSet<_>>();
    if observed != expected {
        return Err(invalid_event(format!(
            "{label} field set is not the exact closed contract"
        )));
    }
    Ok(())
}

fn string_array(value: &Value, field: &str) -> Result<Vec<String>, CharterLifecycleStoreErrorV1> {
    value
        .get(field)
        .and_then(Value::as_array)
        .ok_or_else(|| lineage_error(format!("lifecycle `{field}` must be an array")))?
        .iter()
        .map(|item| {
            item.as_str()
                .map(str::to_owned)
                .ok_or_else(|| lineage_error(format!("lifecycle `{field}` item is not a string")))
        })
        .collect()
}

fn read_lifecycle_intent(
    directory: &Path,
) -> Result<ReadLifecycleIntent, CharterLifecycleStoreErrorV1> {
    let raw_bytes = read_bounded_regular(&directory.join("intent.json"), 64 * 1024)?;
    let record: LifecycleEventIntentRecordV1 = serde_json::from_slice(&raw_bytes)
        .map_err(|_| durability("lifecycle intent is not closed JSON"))?;
    let canonical = serde_json_canonicalizer::to_vec(&record)
        .map_err(|_| durability("lifecycle intent cannot be canonicalized"))?;
    if canonical != raw_bytes
        || record.schema_id != "handbook.lifecycle-event-transaction-intent"
        || record.schema_version != "1.0"
        || record.transaction_id != record.event_id
        || !valid_transaction_id(&record.transaction_id)
    {
        return Err(durability(
            "lifecycle intent identity or canonical bytes are invalid",
        ));
    }
    for fingerprint in [
        &record.canonical_fingerprint,
        &record.prior_transition_fingerprint,
        &record.prior_state_fingerprint,
        &record.evidence_fingerprint,
        &record.observation_fingerprint,
        &record.observation_staged_fingerprint,
        &record.transition_fingerprint,
        &record.transition_staged_fingerprint,
    ] {
        DefinitionFingerprint::parse(fingerprint)
            .map_err(|_| durability("lifecycle intent contains an invalid fingerprint"))?;
    }
    Ok(ReadLifecycleIntent { record, raw_bytes })
}

fn staged_record_exact(
    pending: &Path,
    name: &str,
    class: LineageRecordClassV1,
    expected_ref: &str,
    expected_fingerprint: &str,
    expected_raw_fingerprint: &str,
) -> Result<bool, CharterLifecycleStoreErrorV1> {
    let path = pending.join(name);
    match fs::symlink_metadata(&path) {
        Ok(metadata) => {
            if metadata.file_type().is_symlink() || !metadata.is_file() {
                return Err(durability("lifecycle staged path is not a regular file"));
            }
            let bytes = read_bounded_regular(&path, MAX_JOURNAL_BYTES)?;
            let identity = validate_record(class, &bytes).map_err(map_lineage)?;
            if identity.relative_ref != expected_ref
                || identity.fingerprint != expected_fingerprint
                || raw_fingerprint(&bytes) != expected_raw_fingerprint
            {
                return Err(durability(
                    "lifecycle staged bytes disagree with durable intent",
                ));
            }
            Ok(true)
        }
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => Ok(false),
        Err(_) => Err(io_error("lifecycle staged metadata read failed")),
    }
}

fn final_record_exact(
    lineage: &TrustedLineageStoreV1,
    class: LineageRecordClassV1,
    reference: &str,
    fingerprint: &str,
) -> Result<bool, CharterLifecycleStoreErrorV1> {
    let path = lineage.state_path(reference).map_err(map_lineage)?;
    match fs::symlink_metadata(&path) {
        Ok(_) => lineage
            .read_record(class, reference, fingerprint)
            .map(|_| true)
            .map_err(|_| durability("lifecycle final record bytes disagree with intent")),
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => Ok(false),
        Err(_) => Err(io_error("lifecycle final record metadata read failed")),
    }
}

fn read_current_canonical(
    repo_root: &Path,
) -> Result<Option<Vec<u8>>, CharterLifecycleStoreErrorV1> {
    let workspace = CanonicalWorkspace::new(repo_root);
    let relative = workspace
        .normalize_repo_relative(CANONICAL_REF)
        .map_err(|_| unsafe_error("canonical Charter path is invalid"))?;
    match workspace.trusted_read_strict(&relative) {
        Ok(file) => {
            let (bytes, exceeded) = file
                .read_bytes_bounded(MAX_CANONICAL_BYTES)
                .map_err(|_| io_error("canonical Charter read failed"))?;
            if exceeded {
                return Err(durability("canonical Charter exceeds 8 MiB"));
            }
            Ok(Some(bytes))
        }
        Err(RepoRelativeFileAccessError::Missing(_)) => Ok(None),
        Err(_) => Err(unsafe_error(
            "canonical Charter is not a safe retained regular file",
        )),
    }
}

fn transaction_directories(
    root: &Path,
    suffix: &str,
) -> Result<Vec<PathBuf>, CharterLifecycleStoreErrorV1> {
    let mut paths = Vec::new();
    for entry in
        fs::read_dir(root).map_err(|_| io_error("lifecycle transaction directory read failed"))?
    {
        let entry = entry.map_err(|_| io_error("lifecycle transaction entry read failed"))?;
        if !entry.file_name().to_string_lossy().ends_with(suffix) {
            continue;
        }
        let path = entry.path();
        reject_reparse_or_symlink(&path, true).map_err(map_lineage)?;
        paths.push(path);
    }
    paths.sort();
    Ok(paths)
}

fn write_new_durable(path: &Path, bytes: &[u8]) -> Result<(), CharterLifecycleStoreErrorV1> {
    let mut file =
        create_new_file(path).map_err(|_| io_error("lifecycle staged file create-new failed"))?;
    file.write_all(bytes)
        .map_err(|_| io_error("lifecycle staged file write failed"))?;
    file.sync_all()
        .map_err(|_| io_error("lifecycle staged file fsync failed"))?;
    sync_directory(
        path.parent()
            .ok_or_else(|| unsafe_error("lifecycle staged file has no parent"))?,
    )
    .map_err(map_lineage)
}

fn rename_durable(
    source: &Path,
    target: &Path,
    parent: &Path,
) -> Result<(), CharterLifecycleStoreErrorV1> {
    fs::rename(source, target).map_err(|_| io_error("lifecycle durable rename failed"))?;
    sync_directory(parent).map_err(map_lineage)
}

fn open_lock_file(path: &Path) -> Result<File, CharterLifecycleStoreErrorV1> {
    match create_new_file(path) {
        Ok(file) => return Ok(file),
        Err(error) if error.kind() == std::io::ErrorKind::AlreadyExists => {}
        Err(_) => return Err(io_error("lifecycle lock file create failed")),
    }
    reject_reparse_or_symlink(path, false).map_err(map_lineage)?;
    let mut options = OpenOptions::new();
    options.read(true).write(true);
    #[cfg(windows)]
    {
        use std::os::windows::fs::OpenOptionsExt;
        const FILE_FLAG_OPEN_REPARSE_POINT: u32 = 0x0020_0000;
        options.custom_flags(FILE_FLAG_OPEN_REPARSE_POINT);
    }
    #[cfg(unix)]
    {
        use std::os::unix::fs::OpenOptionsExt;
        options.custom_flags(rustix::fs::OFlags::NOFOLLOW.bits() as i32);
    }
    let file = options
        .open(path)
        .map_err(|_| io_error("lifecycle lock file open failed"))?;
    if !file
        .metadata()
        .map_err(|_| io_error("lifecycle lock metadata failed"))?
        .is_file()
    {
        return Err(unsafe_error("lifecycle lock is not a regular file"));
    }
    Ok(file)
}

fn read_bounded_regular(
    path: &Path,
    limit: usize,
) -> Result<Vec<u8>, CharterLifecycleStoreErrorV1> {
    reject_reparse_or_symlink(path, false).map_err(map_lineage)?;
    let mut options = OpenOptions::new();
    options.read(true);
    #[cfg(windows)]
    {
        use std::os::windows::fs::OpenOptionsExt;
        const FILE_FLAG_OPEN_REPARSE_POINT: u32 = 0x0020_0000;
        options.custom_flags(FILE_FLAG_OPEN_REPARSE_POINT);
    }
    #[cfg(unix)]
    {
        use std::os::unix::fs::OpenOptionsExt;
        options.custom_flags(rustix::fs::OFlags::NOFOLLOW.bits() as i32);
    }
    let file = options
        .open(path)
        .map_err(|_| io_error("lifecycle file no-follow open failed"))?;
    let metadata = file
        .metadata()
        .map_err(|_| io_error("lifecycle file metadata failed"))?;
    #[cfg(windows)]
    {
        use std::os::windows::fs::MetadataExt;
        const FILE_ATTRIBUTE_REPARSE_POINT: u32 = 0x0000_0400;
        if metadata.file_attributes() & FILE_ATTRIBUTE_REPARSE_POINT != 0 {
            return Err(unsafe_error("lifecycle file is a reparse point"));
        }
    }
    if !metadata.is_file() || metadata.len() > limit as u64 {
        return Err(durability("lifecycle file is not a bounded regular file"));
    }
    let mut bytes = Vec::new();
    file.take(limit as u64 + 1)
        .read_to_end(&mut bytes)
        .map_err(|_| io_error("lifecycle file read failed"))?;
    if bytes.len() > limit {
        return Err(durability("lifecycle file exceeds its admission bound"));
    }
    Ok(bytes)
}

fn cleanup_owned_pending(pending: &Path) -> Result<(), CharterLifecycleStoreErrorV1> {
    let allowed = [
        "intent.tmp",
        "intent.json",
        "observation.new",
        "transition.new",
        "records-installed",
        "committed.tmp",
    ];
    for entry in
        fs::read_dir(pending).map_err(|_| io_error("lifecycle cleanup directory read failed"))?
    {
        let entry = entry.map_err(|_| io_error("lifecycle cleanup entry read failed"))?;
        let name = entry.file_name();
        let Some(name) = name.to_str() else {
            return Err(durability(
                "lifecycle pending journal contains non-UTF-8 state",
            ));
        };
        if !allowed.contains(&name) {
            return Err(durability(
                "lifecycle pending journal contains unowned state",
            ));
        }
        let metadata = fs::symlink_metadata(entry.path())
            .map_err(|_| io_error("lifecycle cleanup metadata failed"))?;
        if metadata.file_type().is_symlink() || !metadata.is_file() {
            return Err(unsafe_error(
                "lifecycle cleanup target is not an owned regular file",
            ));
        }
        fs::remove_file(entry.path())
            .map_err(|_| io_error("lifecycle owned-file cleanup failed"))?;
    }
    fs::remove_dir(pending).map_err(|_| io_error("lifecycle pending cleanup failed"))?;
    sync_directory(
        pending
            .parent()
            .ok_or_else(|| unsafe_error("lifecycle pending journal has no parent"))?,
    )
    .map_err(map_lineage)
}

fn remove_owned_progress_marker(pending: &Path) -> Result<(), CharterLifecycleStoreErrorV1> {
    let path = pending.join("committed.tmp");
    match fs::symlink_metadata(&path) {
        Ok(metadata) => {
            if metadata.file_type().is_symlink() || !metadata.is_file() {
                return Err(unsafe_error(
                    "lifecycle progress marker is not an owned regular file",
                ));
            }
            fs::remove_file(&path)
                .map_err(|_| io_error("lifecycle progress marker cleanup failed"))?;
            sync_directory(pending).map_err(map_lineage)
        }
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => Ok(()),
        Err(_) => Err(io_error("lifecycle progress marker metadata failed")),
    }
}

fn valid_transaction_id(value: &str) -> bool {
    !value.is_empty()
        && value.len() <= 128
        && value
            .bytes()
            .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'-' | b'_'))
}

fn validate_utc(value: &str) -> Result<(), CharterLifecycleStoreErrorV1> {
    let bytes = value.as_bytes();
    let shape = bytes.len() == 20
        && bytes[4] == b'-'
        && bytes[7] == b'-'
        && bytes[10] == b'T'
        && bytes[13] == b':'
        && bytes[16] == b':'
        && bytes[19] == b'Z'
        && bytes.iter().enumerate().all(|(index, byte)| {
            matches!(index, 4 | 7 | 10 | 13 | 16 | 19) || byte.is_ascii_digit()
        });
    if !shape {
        return Err(invalid_event(
            "lifecycle audit time must be second-precision UTC",
        ));
    }
    let number = |start: usize, end: usize| -> u32 {
        bytes[start..end]
            .iter()
            .fold(0, |value, byte| value * 10 + u32::from(byte - b'0'))
    };
    let year = number(0, 4);
    let month = number(5, 7);
    let day = number(8, 10);
    let hour = number(11, 13);
    let minute = number(14, 16);
    let second = number(17, 19);
    let leap_year = year % 4 == 0 && (year % 100 != 0 || year % 400 == 0);
    let days_in_month = match month {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        4 | 6 | 9 | 11 => 30,
        2 if leap_year => 29,
        2 => 28,
        _ => 0,
    };
    if year == 0 || day == 0 || day > days_in_month || hour > 23 || minute > 59 || second > 59 {
        return Err(invalid_event(
            "lifecycle audit time is not a valid second-precision UTC instant",
        ));
    }
    Ok(())
}

fn raw_fingerprint(bytes: &[u8]) -> String {
    DefinitionFingerprint::from_bytes(bytes).to_string()
}

fn fingerprint_from_ref(reference: &str) -> Option<String> {
    let basename = reference.rsplit('/').next()?.strip_suffix(".json")?;
    let (_, hex) = basename.rsplit_once('_')?;
    if hex.len() == 64
        && hex
            .bytes()
            .all(|byte| byte.is_ascii_hexdigit() && !byte.is_ascii_uppercase())
    {
        Some(format!("sha256:{hex}"))
    } else {
        None
    }
}

fn ensure_supported_platform() -> Result<(), CharterLifecycleStoreErrorV1> {
    if cfg!(any(unix, windows)) {
        Ok(())
    } else {
        Err(error(
            CharterLifecycleStoreErrorKindV1::UnsupportedPlatform,
            "lifecycle persistence requires native strict reads, locks, rename, and durable directory flush",
        ))
    }
}

fn map_lineage(source: LineageStoreErrorV1) -> CharterLifecycleStoreErrorV1 {
    error(
        match source.kind() {
            crate::LineageStoreErrorKindV1::UnsupportedPlatform => {
                CharterLifecycleStoreErrorKindV1::UnsupportedPlatform
            }
            crate::LineageStoreErrorKindV1::UnsafeFilesystem
            | crate::LineageStoreErrorKindV1::UnsafeReference => {
                CharterLifecycleStoreErrorKindV1::UnsafeFilesystem
            }
            _ => CharterLifecycleStoreErrorKindV1::LineageViolation,
        },
        source.detail(),
    )
}

#[cfg(test)]
#[allow(
    clippy::items_after_test_module,
    reason = "the focused promotion-anchor fixtures stay adjacent to the parser they exercise"
)]
mod promotion_anchor_v12_tests {
    use super::*;
    use crate::charter_promotion_intent_v12::promotion_intent_marker_v12;

    const RUNTIME_VECTORS: &[u8] = include_bytes!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../docs/specs/handbook-contract-membrane/slices/HCM-2.2/contracts/runtime-record-fingerprint-vectors-v1.0.json"
    ));
    const INTENT_VECTORS: &[u8] = include_bytes!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../docs/specs/handbook-contract-membrane/slices/HCM-2.2/contracts/promotion-transaction-intent-vectors-v1.0.json"
    ));

    fn runtime_record(class: &str) -> Value {
        let vectors: Value = serde_json::from_slice(RUNTIME_VECTORS).unwrap();
        vectors["vectors"]
            .as_array()
            .unwrap()
            .iter()
            .find(|row| row["record_class"] == class)
            .unwrap()["record"]
            .clone()
    }

    fn resign_record(
        value: &mut Value,
        id_field: &str,
        fingerprint_field: &str,
        prefix: &str,
        audit_only: &[&str],
    ) {
        let mut preimage = value.clone();
        let object = preimage.as_object_mut().unwrap();
        object.remove(id_field);
        object.remove(fingerprint_field);
        for field in audit_only {
            object.remove(*field);
        }
        let fingerprint = DefinitionFingerprint::from_json_value(&preimage)
            .unwrap()
            .to_string();
        value[id_field] = Value::String(format!(
            "{prefix}_{}",
            fingerprint.strip_prefix("sha256:").unwrap()
        ));
        value[fingerprint_field] = Value::String(fingerprint);
    }

    fn valid_intent(repo: &Path, mode: &str, transaction_id: &str) -> (Value, Vec<u8>) {
        let vectors: Value = serde_json::from_slice(INTENT_VECTORS).unwrap();
        let mut intent = vectors["amendment_positive"]["intent_fingerprint_preimage"].clone();
        intent["transaction_id"] = Value::String(transaction_id.to_owned());
        if mode == "create" {
            intent["mutation_mode"] = Value::String("create".to_owned());
            intent["target"]["basis_artifact_fingerprint"] = Value::Null;
            intent["target"]["observed_current_artifact_fingerprint"] = Value::Null;
            intent["selected_contract"]["prior_lifecycle_head_ref"] = Value::Null;
            intent["selected_contract"]["prior_lifecycle_head_fingerprint"] = Value::Null;
            intent["selected_contract"]["prior_lifecycle_state"] =
                Value::String("absent".to_owned());
            intent["selected_contract"]["prior_lifecycle_state_fingerprint"] = Value::Null;
            intent["selected_contract"]["active_observations"] = Value::Array(Vec::new());
            intent["selected_contract"]["reopened_coverage_ids"] = Value::Array(Vec::new());
            intent["recovery"]["old_canonical_status"] = Value::String("absent".to_owned());
            intent["recovery"]["old_canonical_fingerprint"] = Value::Null;
            intent["recovery"]["old_canonical_document_sha256"] = Value::Null;
            intent["recovery"]["old_canonical_byte_length"] = Value::Null;
        }

        let mut promotion = runtime_record("promotion");
        promotion["candidate_ref"] = intent["candidate_lineage"]["candidate_ref"].clone();
        promotion["candidate_fingerprint"] =
            intent["candidate_lineage"]["candidate_fingerprint"].clone();
        promotion["canonical_artifact_ref"] = intent["target"]["canonical_artifact_ref"].clone();
        promotion["canonical_artifact_fingerprint"] =
            intent["outputs"]["new_canonical_fingerprint"].clone();
        promotion["target_instance_id"] = intent["target"]["target_instance_id"].clone();
        promotion["basis_artifact_fingerprint"] =
            intent["target"]["basis_artifact_fingerprint"].clone();
        promotion["expected_current_artifact_fingerprint"] =
            intent["target"]["observed_current_artifact_fingerprint"].clone();
        promotion["profile_ref"] = intent["selected_contract"]["profile_ref"].clone();
        promotion["resolved_profile_fingerprint"] =
            intent["selected_contract"]["resolved_profile_fingerprint"].clone();
        promotion["resolved_definitions"] = Value::Array(
            intent["selected_contract"]["resolved_definitions"]
                .as_array()
                .unwrap()
                .iter()
                .map(|binding| {
                    json!({
                        "definition_ref": binding["definition_ref"],
                        "definition_fingerprint": binding["definition_fingerprint"],
                    })
                })
                .collect(),
        );
        promotion["approval_refs"] = Value::Array(
            intent["human_authority"]["approval_bindings"]
                .as_array()
                .unwrap()
                .iter()
                .map(|binding| binding["approval_ref"].clone())
                .collect(),
        );
        promotion["validation_result_refs"] = Value::Array(vec![intent["candidate_lineage"]
            ["validation_result_ref"]
            .clone()]);
        for field in [
            "approver_registry_state_ref",
            "approver_registry_state_fingerprint",
            "registry_head_transition_ref",
            "registry_head_transition_fingerprint",
        ] {
            promotion[field] = intent["human_authority"][field].clone();
        }
        promotion["decision"] = Value::String("approved".to_owned());
        resign_record(
            &mut promotion,
            "promotion_id",
            "promotion_fingerprint",
            "promotion",
            &[],
        );
        let promotion_bytes = serde_json_canonicalizer::to_vec(&promotion).unwrap();
        let promotion = TrustedLineageStoreV1::new(repo)
            .append_record(LineageRecordClassV1::Promotion, &promotion_bytes)
            .unwrap();
        intent["promotion_id"] = Value::String(
            promotion
                .relative_ref
                .strip_prefix("promotions/")
                .unwrap()
                .strip_suffix(".json")
                .unwrap()
                .to_owned(),
        );
        intent["outputs"]["promotion_record"] = json!({
            "record_ref": promotion.relative_ref,
            "record_fingerprint": promotion.fingerprint,
            "document_sha256": DefinitionFingerprint::from_bytes(&promotion_bytes).to_string(),
            "byte_length": promotion_bytes.len() as u64,
        });

        let mut lifecycle = runtime_record("lifecycle-transition");
        lifecycle["lifecycle_policy_ref"] =
            intent["selected_contract"]["lifecycle_policy_ref"].clone();
        lifecycle["lifecycle_policy_fingerprint"] =
            intent["selected_contract"]["lifecycle_policy_fingerprint"].clone();
        lifecycle["target_instance_id"] = intent["target"]["target_instance_id"].clone();
        lifecycle["new_observation_refs"] = Value::Array(Vec::new());
        lifecycle["active_observation_refs"] = Value::Array(Vec::new());
        lifecycle["result_state"] = Value::String("current".to_owned());
        let result_state_fingerprint = charter_lifecycle_state_fingerprint(
            intent["selected_contract"]["lifecycle_policy_ref"]
                .as_str()
                .unwrap(),
            intent["selected_contract"]["lifecycle_policy_fingerprint"]
                .as_str()
                .unwrap(),
            "project_authority",
            intent["outputs"]["new_canonical_fingerprint"]
                .as_str()
                .unwrap(),
            CharterLifecycleState::Current,
            &[],
        )
        .unwrap();
        lifecycle["prior_state"] = if mode == "create" {
            Value::String("current".to_owned())
        } else {
            intent["selected_contract"]["prior_lifecycle_state"].clone()
        };
        lifecycle["prior_state_fingerprint"] = if mode == "create" {
            Value::String(result_state_fingerprint.clone())
        } else {
            intent["selected_contract"]["prior_lifecycle_state_fingerprint"].clone()
        };
        lifecycle["result_state_fingerprint"] = Value::String(result_state_fingerprint);
        lifecycle["clearance_promotion_ref"] =
            intent["outputs"]["promotion_record"]["record_ref"].clone();
        resign_record(
            &mut lifecycle,
            "transition_id",
            "transition_fingerprint",
            "lifecycle-transition",
            &["transitioned_at_utc"],
        );
        let lifecycle_bytes = serde_json_canonicalizer::to_vec(&lifecycle).unwrap();
        let lifecycle = TrustedLineageStoreV1::new(repo)
            .append_record(LineageRecordClassV1::LifecycleTransition, &lifecycle_bytes)
            .unwrap();
        intent["outputs"]["lifecycle_transition"] = json!({
            "record_ref": lifecycle.relative_ref,
            "record_fingerprint": lifecycle.fingerprint,
            "document_sha256": DefinitionFingerprint::from_bytes(&lifecycle_bytes).to_string(),
            "byte_length": lifecycle_bytes.len() as u64,
        });
        let mut preimage = intent.clone();
        preimage
            .as_object_mut()
            .unwrap()
            .remove("intent_fingerprint");
        intent["intent_fingerprint"] = Value::String(
            DefinitionFingerprint::from_json_value(&preimage)
                .unwrap()
                .to_string(),
        );
        let mut bytes = serde_json_canonicalizer::to_vec(&intent).unwrap();
        bytes.push(b'\n');
        (intent, bytes)
    }

    fn publish_committed(repo: &Path, transaction_id: &str, bytes: &[u8]) {
        let directory = repo
            .join(".handbook/state/transactions/promotions")
            .join(format!("{transaction_id}.committed"));
        fs::create_dir_all(&directory).unwrap();
        fs::write(directory.join("intent.json"), bytes).unwrap();
        let marker = promotion_intent_marker_v12(bytes);
        for name in [
            "prepared",
            "canonical-installed",
            "records-installed",
            "committed",
        ] {
            fs::write(directory.join(name), &marker).unwrap();
        }
    }

    fn resign_intent(intent: &mut Value) -> Vec<u8> {
        let mut preimage = intent.clone();
        preimage
            .as_object_mut()
            .unwrap()
            .remove("intent_fingerprint");
        intent["intent_fingerprint"] = Value::String(
            DefinitionFingerprint::from_json_value(&preimage)
                .unwrap()
                .to_string(),
        );
        let mut bytes = serde_json_canonicalizer::to_vec(intent).unwrap();
        bytes.push(b'\n');
        bytes
    }

    #[test]
    fn valid_create_anchor_reads_only_exact_terminal_intent_1_2_outputs() {
        let mode = "create";
        let repo = tempfile::tempdir().unwrap();
        let transaction_id = format!("promotion-transaction_{mode}");
        let (intent, bytes) = valid_intent(repo.path(), mode, &transaction_id);
        publish_committed(repo.path(), &transaction_id, &bytes);
        let expected_fingerprint = intent["outputs"]["new_canonical_fingerprint"]
            .as_str()
            .unwrap();
        let anchor = CharterLifecycleStoreV1::new(repo.path())
            .current_promotion_anchor(expected_fingerprint)
            .expect("valid nested intent 1.2 anchor");
        assert_eq!(
            anchor.lifecycle_transition_ref,
            intent["outputs"]["lifecycle_transition"]["record_ref"]
        );
        assert_eq!(
            anchor.lifecycle_transition_fingerprint,
            intent["outputs"]["lifecycle_transition"]["record_fingerprint"]
        );
    }

    #[test]
    fn legacy_shape_wrong_schema_fingerprint_marker_and_nested_record_refuse() {
        for case in [
            "legacy",
            "schema",
            "version",
            "fingerprint",
            "marker",
            "nested",
        ] {
            let repo = tempfile::tempdir().unwrap();
            let transaction_id = format!("promotion-transaction_{case}");
            let (mut intent, mut bytes) = valid_intent(repo.path(), "create", &transaction_id);
            match case {
                "legacy" => {
                    intent["canonical_fingerprint"] =
                        intent["outputs"]["new_canonical_fingerprint"].clone();
                    bytes = resign_intent(&mut intent);
                }
                "schema" => {
                    intent["schema_id"] =
                        Value::String("handbook.promotion-transaction-intent".to_owned());
                    bytes = resign_intent(&mut intent);
                }
                "version" => {
                    intent["schema_version"] = Value::String("1.0".to_owned());
                    bytes = resign_intent(&mut intent);
                }
                "fingerprint" => {
                    intent["intent_fingerprint"] =
                        Value::String(format!("sha256:{}", "0".repeat(64)));
                    bytes = {
                        let mut value = serde_json_canonicalizer::to_vec(&intent).unwrap();
                        value.push(b'\n');
                        value
                    };
                }
                "nested" => {
                    intent["outputs"]["lifecycle_transition"]["document_sha256"] =
                        Value::String(format!("sha256:{}", "0".repeat(64)));
                    bytes = resign_intent(&mut intent);
                }
                "marker" => {}
                _ => unreachable!(),
            }
            publish_committed(repo.path(), &transaction_id, &bytes);
            if case == "marker" {
                fs::write(
                    repo.path()
                        .join(".handbook/state/transactions/promotions")
                        .join(format!("{transaction_id}.committed/committed")),
                    format!("sha256:{}\n", "0".repeat(64)),
                )
                .unwrap();
            }
            let fingerprint = intent["outputs"]["new_canonical_fingerprint"]
                .as_str()
                .unwrap();
            assert!(
                CharterLifecycleStoreV1::new(repo.path())
                    .current_promotion_anchor(fingerprint)
                    .is_err(),
                "{case} must refuse"
            );
        }
    }

    #[test]
    fn duplicate_matching_intent_1_2_transactions_refuse_exactly_one_anchor() {
        let repo = tempfile::tempdir().unwrap();
        let mut fingerprint = String::new();
        for suffix in ["one", "two"] {
            let transaction_id = format!("promotion-transaction_duplicate-{suffix}");
            let (intent, bytes) = valid_intent(repo.path(), "create", &transaction_id);
            fingerprint = intent["outputs"]["new_canonical_fingerprint"]
                .as_str()
                .unwrap()
                .to_owned();
            publish_committed(repo.path(), &transaction_id, &bytes);
        }
        assert!(CharterLifecycleStoreV1::new(repo.path())
            .current_promotion_anchor(&fingerprint)
            .is_err());
    }
}

#[cfg(test)]
mod lifecycle_fault_control_tests {
    use super::*;

    #[test]
    fn every_lifecycle_fault_boundary_is_module_local_and_exactly_selected() {
        let _private_hook = CharterLifecycleStoreV1::record_event_with_fault_for_testing;
        for selected in [
            CharterLifecycleFaultPointV1::Intent,
            CharterLifecycleFaultPointV1::Prepared,
            CharterLifecycleFaultPointV1::ObservationInstalled,
            CharterLifecycleFaultPointV1::RecordsInstalled,
        ] {
            let _guard = ScopedLifecycleFaultV1::select(selected);
            for observed in [
                CharterLifecycleFaultPointV1::Intent,
                CharterLifecycleFaultPointV1::Prepared,
                CharterLifecycleFaultPointV1::ObservationInstalled,
                CharterLifecycleFaultPointV1::RecordsInstalled,
            ] {
                let result = inject_lifecycle_fault(observed, "test-only lifecycle fault");
                assert_eq!(result.is_err(), observed == selected);
            }
        }
    }
}

fn error(
    kind: CharterLifecycleStoreErrorKindV1,
    detail: impl Into<String>,
) -> CharterLifecycleStoreErrorV1 {
    CharterLifecycleStoreErrorV1::new(kind, detail)
}

fn invalid_event(detail: impl Into<String>) -> CharterLifecycleStoreErrorV1 {
    error(CharterLifecycleStoreErrorKindV1::InvalidEvent, detail)
}

fn lineage_error(detail: impl Into<String>) -> CharterLifecycleStoreErrorV1 {
    error(CharterLifecycleStoreErrorKindV1::LineageViolation, detail)
}

fn unsafe_error(detail: impl Into<String>) -> CharterLifecycleStoreErrorV1 {
    error(CharterLifecycleStoreErrorKindV1::UnsafeFilesystem, detail)
}

fn durability(detail: impl Into<String>) -> CharterLifecycleStoreErrorV1 {
    error(
        CharterLifecycleStoreErrorKindV1::DurabilityViolation,
        detail,
    )
}

fn io_error(detail: impl Into<String>) -> CharterLifecycleStoreErrorV1 {
    error(CharterLifecycleStoreErrorKindV1::IoFailure, detail)
}

#[cfg(test)]
fn injected(detail: impl Into<String>) -> CharterLifecycleStoreErrorV1 {
    error(CharterLifecycleStoreErrorKindV1::InjectedFault, detail)
}
