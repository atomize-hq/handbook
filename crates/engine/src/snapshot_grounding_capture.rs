use crate::snapshot_grounding_owner::{
    EvidenceSnapshotSourceV1, GitSnapshotSourceV1, HandbookSnapshotSourceV1,
    SessionBoundarySourceV1, WorkSnapshotSourceV1,
};
use crate::snapshot_grounding_repository::{
    SnapshotBoundaryCommitIntentV1, SnapshotPredecessorV1, SnapshotRecordRefV1,
    SnapshotRepositoryCapturePlanV1, SnapshotRepositoryCommitV1, SnapshotRepositoryErrorKindV1,
    SnapshotRepositoryErrorV1, SnapshotRepositoryV1,
};
use crate::{
    DefinitionFingerprint, ExactDefinitionRef, ResolutionObservationV1, SnapshotBoundaryKindV1,
    SnapshotGroundingErrorKindV1, SnapshotGroundingOwnerV1, SnapshotRepositoryBindingV1,
    SnapshotSourceContractV1, SnapshotSourceFamilyV1, SnapshotStreamIdentityV1,
};
use serde_json::{json, Value};
use std::collections::BTreeSet;
use std::fmt;

const MAX_IDEMPOTENCY_KEY_BYTES: usize = 256;
const MAX_SESSION_REF_BYTES: usize = 256;
const MAX_BOUNDED_SLOT_BYTES: usize = 16 * 1024;
const MAX_BOUNDED_VALUE_DEPTH: usize = 64;
const MAX_BOUNDED_VALUE_NODES: usize = 4096;
const MAX_HANDOFF_REFS: usize = 32;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum SnapshotSlotDispositionV1 {
    RetainedObservation,
    DerivedBoundedFact,
    Redacted,
    Omitted,
    Inapplicable,
    Unavailable,
    Refused,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SnapshotSlotObservationV1 {
    disposition: SnapshotSlotDispositionV1,
    exact_ref: Option<String>,
    fingerprint: Option<DefinitionFingerprint>,
    rule_condition: Option<&'static str>,
    currentness_nonce: String,
}

impl SnapshotSlotObservationV1 {
    pub fn retained(exact_ref: &str, fingerprint: &str) -> Result<Self, SnapshotCaptureErrorV1> {
        Self::bounded(
            SnapshotSlotDispositionV1::RetainedObservation,
            exact_ref,
            fingerprint,
        )
    }

    pub fn derived(exact_ref: &str, fingerprint: &str) -> Result<Self, SnapshotCaptureErrorV1> {
        Self::bounded(
            SnapshotSlotDispositionV1::DerivedBoundedFact,
            exact_ref,
            fingerprint,
        )
    }

    pub fn redacted(exact_ref: &str, fingerprint: &str) -> Result<Self, SnapshotCaptureErrorV1> {
        Self::bounded(SnapshotSlotDispositionV1::Redacted, exact_ref, fingerprint)
    }

    pub fn omitted(rule_condition: &'static str) -> Self {
        Self::without_payload(SnapshotSlotDispositionV1::Omitted, rule_condition)
    }

    pub fn inapplicable(rule_condition: &'static str) -> Self {
        Self::without_payload(SnapshotSlotDispositionV1::Inapplicable, rule_condition)
    }

    pub fn unavailable(rule_condition: &'static str) -> Self {
        Self::without_payload(SnapshotSlotDispositionV1::Unavailable, rule_condition)
    }

    pub fn refused(rule_condition: &'static str) -> Self {
        Self::without_payload(SnapshotSlotDispositionV1::Refused, rule_condition)
    }

    #[cfg(any(test, feature = "test-support"))]
    pub fn with_currentness_nonce_for_testing(mut self, nonce: &str) -> Self {
        self.currentness_nonce = nonce.to_owned();
        self
    }

    fn bounded(
        disposition: SnapshotSlotDispositionV1,
        exact_ref: &str,
        fingerprint: &str,
    ) -> Result<Self, SnapshotCaptureErrorV1> {
        ExactDefinitionRef::parse(exact_ref).map_err(|_| {
            SnapshotCaptureErrorV1::new(SnapshotGroundingErrorKindV1::UnknownReference)
        })?;
        let fingerprint = DefinitionFingerprint::parse(fingerprint).map_err(|_| {
            SnapshotCaptureErrorV1::new(SnapshotGroundingErrorKindV1::UnknownReference)
        })?;
        Ok(Self {
            disposition,
            exact_ref: Some(exact_ref.to_owned()),
            fingerprint: Some(fingerprint),
            rule_condition: None,
            currentness_nonce: String::new(),
        })
    }

    fn without_payload(
        disposition: SnapshotSlotDispositionV1,
        rule_condition: &'static str,
    ) -> Self {
        Self {
            disposition,
            exact_ref: None,
            fingerprint: None,
            rule_condition: Some(rule_condition),
            currentness_nonce: String::new(),
        }
    }

    fn is_readable(&self) -> bool {
        matches!(
            self.disposition,
            SnapshotSlotDispositionV1::RetainedObservation
                | SnapshotSlotDispositionV1::DerivedBoundedFact
        )
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SnapshotSourcePortErrorV1 {
    Missing,
    Unavailable,
}

impl SnapshotSourcePortErrorV1 {
    pub fn missing() -> Self {
        Self::Missing
    }

    pub fn unavailable() -> Self {
        Self::Unavailable
    }
}

pub trait SnapshotSourcePortV1 {
    fn observe_slot(
        &mut self,
        family: SnapshotSourceFamilyV1,
        slot: &'static str,
    ) -> Result<SnapshotSlotObservationV1, SnapshotSourcePortErrorV1>;

    fn read_bounded_slot(
        &mut self,
        family: SnapshotSourceFamilyV1,
        slot: &'static str,
    ) -> Result<Value, SnapshotSourcePortErrorV1>;
}

pub struct SnapshotCaptureRequestV1 {
    idempotency_key: Vec<u8>,
    host_session_ref: String,
    kind: SnapshotBoundaryKindV1,
}

impl SnapshotCaptureRequestV1 {
    pub fn new(
        idempotency_key: impl AsRef<[u8]>,
        host_session_ref: &str,
        kind: SnapshotBoundaryKindV1,
    ) -> Result<Self, SnapshotCaptureErrorV1> {
        let idempotency_key = idempotency_key.as_ref();
        if idempotency_key.is_empty()
            || idempotency_key.len() > MAX_IDEMPOTENCY_KEY_BYTES
            || host_session_ref.is_empty()
            || host_session_ref.len() > MAX_SESSION_REF_BYTES
            || host_session_ref.chars().any(char::is_control)
        {
            return Err(SnapshotCaptureErrorV1::new(
                SnapshotGroundingErrorKindV1::MalformedInput,
            ));
        }
        Ok(Self {
            idempotency_key: idempotency_key.to_vec(),
            host_session_ref: host_session_ref.to_owned(),
            kind,
        })
    }
}

impl fmt::Debug for SnapshotCaptureRequestV1 {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("SnapshotCaptureRequestV1")
            .field("idempotency_key", &"<redacted>")
            .field("host_session_ref", &self.host_session_ref)
            .field("kind", &self.kind)
            .finish_non_exhaustive()
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SnapshotSlotAccountingV1 {
    family: SnapshotSourceFamilyV1,
    slot: &'static str,
    disposition: SnapshotSlotDispositionV1,
}

impl SnapshotSlotAccountingV1 {
    pub fn family(&self) -> SnapshotSourceFamilyV1 {
        self.family
    }

    pub fn slot(&self) -> &'static str {
        self.slot
    }

    pub fn disposition(&self) -> &'static str {
        disposition_name(self.disposition)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SnapshotCaptureErrorV1 {
    kind: SnapshotGroundingErrorKindV1,
    accounting: Vec<SnapshotSlotAccountingV1>,
}

impl SnapshotCaptureErrorV1 {
    fn new(kind: SnapshotGroundingErrorKindV1) -> Self {
        Self {
            kind,
            accounting: Vec::new(),
        }
    }

    fn with_accounting(kind: SnapshotGroundingErrorKindV1, slots: &[CapturedSlotV1]) -> Self {
        Self {
            kind,
            accounting: slots
                .iter()
                .map(|slot| SnapshotSlotAccountingV1 {
                    family: slot.family,
                    slot: slot.slot,
                    disposition: slot.observation.disposition,
                })
                .collect(),
        }
    }

    pub fn kind(&self) -> SnapshotGroundingErrorKindV1 {
        self.kind
    }

    pub fn accounting(&self) -> &[SnapshotSlotAccountingV1] {
        &self.accounting
    }
}

impl fmt::Display for SnapshotCaptureErrorV1 {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("snapshot capture failed closed")
    }
}

impl std::error::Error for SnapshotCaptureErrorV1 {}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SnapshotCommittedRecordV1 {
    exact_ref: String,
    fingerprint: String,
}

impl SnapshotCommittedRecordV1 {
    pub fn exact_ref(&self) -> &str {
        &self.exact_ref
    }

    pub fn fingerprint(&self) -> &str {
        &self.fingerprint
    }
}

#[derive(Clone, Debug)]
struct SnapshotCommittedGroundingV1 {
    sequence: u64,
    predecessor_boundary_ref: Option<String>,
    boundary_record: SnapshotCommittedRecordV1,
    accounting_record: SnapshotCommittedRecordV1,
    delta_record: Option<SnapshotCommittedRecordV1>,
    grounding_record: SnapshotCommittedRecordV1,
    receipt_record: SnapshotCommittedRecordV1,
    replayed: bool,
    repository_commit: SnapshotRepositoryCommitV1,
}

#[derive(Clone, Debug)]
pub struct SnapshotStartedGroundingV1 {
    committed: SnapshotCommittedGroundingV1,
    first_start: bool,
}

#[derive(Clone, Debug)]
pub struct SnapshotFinalGroundingV1 {
    committed: SnapshotCommittedGroundingV1,
    handoff_refs: Vec<String>,
}

#[derive(Clone, Debug)]
pub enum SnapshotCaptureOutcomeV1 {
    Started(SnapshotStartedGroundingV1),
    Finalized(SnapshotFinalGroundingV1),
}

impl SnapshotCaptureOutcomeV1 {
    pub fn boundary_record(&self) -> &SnapshotCommittedRecordV1 {
        match self {
            Self::Started(value) => value.boundary_record(),
            Self::Finalized(value) => value.boundary_record(),
        }
    }
}

macro_rules! committed_getters {
    ($type:ty) => {
        impl $type {
            pub fn sequence(&self) -> u64 {
                self.committed.sequence
            }

            pub fn replayed(&self) -> bool {
                self.committed.replayed
            }

            pub fn predecessor_boundary_ref(&self) -> Option<&str> {
                self.committed.predecessor_boundary_ref.as_deref()
            }

            pub fn boundary_record(&self) -> &SnapshotCommittedRecordV1 {
                &self.committed.boundary_record
            }

            pub fn accounting_record(&self) -> &SnapshotCommittedRecordV1 {
                &self.committed.accounting_record
            }

            pub fn delta_record(&self) -> Option<&SnapshotCommittedRecordV1> {
                self.committed.delta_record.as_ref()
            }

            pub fn grounding_record(&self) -> &SnapshotCommittedRecordV1 {
                &self.committed.grounding_record
            }

            pub fn receipt_record(&self) -> &SnapshotCommittedRecordV1 {
                &self.committed.receipt_record
            }

            #[cfg(any(test, feature = "test-support"))]
            pub fn repository_commit_for_testing(&self) -> &SnapshotRepositoryCommitV1 {
                &self.committed.repository_commit
            }
        }
    };
}

committed_getters!(SnapshotStartedGroundingV1);
committed_getters!(SnapshotFinalGroundingV1);

impl SnapshotStartedGroundingV1 {
    pub fn is_first_start(&self) -> bool {
        self.first_start
    }
}

impl SnapshotFinalGroundingV1 {
    pub fn handoff_refs(&self) -> &[String] {
        &self.handoff_refs
    }
}

impl SnapshotGroundingOwnerV1 {
    pub fn capture_boundary<P: SnapshotSourcePortV1>(
        &self,
        binding: &SnapshotRepositoryBindingV1,
        stream: &SnapshotStreamIdentityV1,
        request: &SnapshotCaptureRequestV1,
        port: &mut P,
    ) -> Result<SnapshotCaptureOutcomeV1, SnapshotCaptureErrorV1> {
        let repository = self
            .open_repository(binding, stream)
            .map_err(map_repository_error)?;
        match repository
            .plan_capture(
                &request.idempotency_key,
                &request.host_session_ref,
                request.kind,
            )
            .map_err(map_repository_error)?
        {
            SnapshotRepositoryCapturePlanV1::Replay(commit) => {
                outcome_from_commit(&repository, *commit)
            }
            SnapshotRepositoryCapturePlanV1::Fresh {
                sequence,
                predecessor,
                predecessor_grounding,
            } => {
                let previous = predecessor_grounding
                    .as_ref()
                    .map(|record| repository.read_record(record))
                    .transpose()
                    .map_err(map_repository_error)?
                    .map(|bytes| parse_grounding_capture(&bytes))
                    .transpose()?;
                let mut slots =
                    capture_sources(self, request, stream, sequence, &predecessor, port)?;
                final_currentness_recheck(&mut slots, port);
                let first_start = matches!(predecessor, SnapshotPredecessorV1::NoApplicablePrior);
                let terminal_failure = validate_complete_capture(
                    self,
                    &mut slots,
                    first_start,
                    request.kind,
                    previous.as_ref(),
                );
                if let Some(kind) = terminal_failure {
                    return Err(SnapshotCaptureErrorV1::with_accounting(kind, &slots));
                }
                let accounting = accounting_value(self, &slots, first_start)?;
                let resolution = derive_resolution(&slots).map_err(|failure| {
                    SnapshotCaptureErrorV1::with_accounting(failure.kind(), &slots)
                })?;
                let delta = previous
                    .as_ref()
                    .map(|prior| delta_value(prior, &slots, sequence, request.kind))
                    .transpose()
                    .map_err(|failure| {
                        SnapshotCaptureErrorV1::with_accounting(failure.kind(), &slots)
                    })?;
                let grounding = grounding_value(
                    self,
                    request,
                    &slots,
                    sequence,
                    &predecessor,
                    delta.as_ref(),
                    &resolution,
                )
                .map_err(|failure| {
                    SnapshotCaptureErrorV1::with_accounting(failure.kind(), &slots)
                })?;
                let intent = SnapshotBoundaryCommitIntentV1::new(
                    &request.idempotency_key,
                    &request.host_session_ref,
                    request.kind,
                    canonical_json(&accounting).map_err(|failure| {
                        SnapshotCaptureErrorV1::with_accounting(failure.kind(), &slots)
                    })?,
                    delta
                        .as_ref()
                        .map(canonical_json)
                        .transpose()
                        .map_err(|failure| {
                            SnapshotCaptureErrorV1::with_accounting(failure.kind(), &slots)
                        })?,
                    canonical_json(&grounding).map_err(|failure| {
                        SnapshotCaptureErrorV1::with_accounting(failure.kind(), &slots)
                    })?,
                )
                .with_owner_plan(sequence, predecessor);
                let commit = repository
                    .commit_boundary(intent)
                    .map_err(map_repository_error)
                    .map_err(|failure| {
                        SnapshotCaptureErrorV1::with_accounting(failure.kind(), &slots)
                    })?;
                outcome_from_commit(&repository, commit).map_err(|failure| {
                    SnapshotCaptureErrorV1::with_accounting(failure.kind(), &slots)
                })
            }
        }
    }
}

#[derive(Clone, Debug)]
struct CapturedSlotV1 {
    family: SnapshotSourceFamilyV1,
    slot: &'static str,
    observation: SnapshotSlotObservationV1,
    bounded_value: Option<Value>,
    failure: Option<SnapshotGroundingErrorKindV1>,
}

trait ProductionCaptureAdapterV1 {
    const FAMILY: SnapshotSourceFamilyV1;

    fn capture<P: SnapshotSourcePortV1>(
        owner: &SnapshotGroundingOwnerV1,
        port: &mut P,
    ) -> Vec<CapturedSlotV1> {
        let contract = owner
            .source_contracts()
            .iter()
            .find(|contract| contract.family() == Self::FAMILY)
            .expect("admitted production family has one exact adapter contract");
        capture_contract(contract, port)
    }
}

impl ProductionCaptureAdapterV1 for EvidenceSnapshotSourceV1 {
    const FAMILY: SnapshotSourceFamilyV1 = SnapshotSourceFamilyV1::Evidence;
}
impl ProductionCaptureAdapterV1 for GitSnapshotSourceV1 {
    const FAMILY: SnapshotSourceFamilyV1 = SnapshotSourceFamilyV1::Git;
}
impl ProductionCaptureAdapterV1 for HandbookSnapshotSourceV1 {
    const FAMILY: SnapshotSourceFamilyV1 = SnapshotSourceFamilyV1::Handbook;
}
impl ProductionCaptureAdapterV1 for WorkSnapshotSourceV1 {
    const FAMILY: SnapshotSourceFamilyV1 = SnapshotSourceFamilyV1::Work;
}

fn capture_sources<P: SnapshotSourcePortV1>(
    owner: &SnapshotGroundingOwnerV1,
    request: &SnapshotCaptureRequestV1,
    stream: &SnapshotStreamIdentityV1,
    sequence: u64,
    predecessor: &SnapshotPredecessorV1,
    port: &mut P,
) -> Result<Vec<CapturedSlotV1>, SnapshotCaptureErrorV1> {
    let mut slots = EvidenceSnapshotSourceV1::capture(owner, port);
    slots.extend(GitSnapshotSourceV1::capture(owner, port));
    slots.extend(HandbookSnapshotSourceV1::capture(owner, port));
    slots.push(SessionBoundarySourceV1::capture_owner_derived(
        request,
        stream,
        sequence,
        predecessor,
    )?);
    slots.extend(WorkSnapshotSourceV1::capture(owner, port));
    Ok(slots)
}

impl SessionBoundarySourceV1 {
    fn capture_owner_derived(
        request: &SnapshotCaptureRequestV1,
        stream: &SnapshotStreamIdentityV1,
        sequence: u64,
        predecessor: &SnapshotPredecessorV1,
    ) -> Result<CapturedSlotV1, SnapshotCaptureErrorV1> {
        let bounded_value = json!({
            "boundary_kind": if request.kind == SnapshotBoundaryKindV1::Start { "start" } else { "end" },
            "host_session_ref": request.host_session_ref,
            "predecessor": predecessor,
            "sequence": sequence,
            "stream_fingerprint": stream.fingerprint(),
        });
        let fingerprint = DefinitionFingerprint::from_json_value(&bounded_value).map_err(|_| {
            SnapshotCaptureErrorV1::new(SnapshotGroundingErrorKindV1::MalformedInput)
        })?;
        Ok(CapturedSlotV1 {
            family: SnapshotSourceFamilyV1::Session,
            slot: "boundary_context",
            observation: SnapshotSlotObservationV1::derived(
                "handbook.snapshot-observation.session.boundary-context@1.0.0",
                fingerprint.as_str(),
            )?,
            bounded_value: Some(bounded_value),
            failure: None,
        })
    }
}

fn capture_contract<P: SnapshotSourcePortV1>(
    contract: &SnapshotSourceContractV1,
    port: &mut P,
) -> Vec<CapturedSlotV1> {
    contract
        .source_slots()
        .iter()
        .map(|&slot| capture_slot(contract, slot, port))
        .collect()
}

fn capture_slot<P: SnapshotSourcePortV1>(
    contract: &SnapshotSourceContractV1,
    slot: &'static str,
    port: &mut P,
) -> CapturedSlotV1 {
    let family = contract.family();
    let before = match port.observe_slot(family, slot) {
        Ok(observation) => observation,
        Err(failure) => return port_failure_slot(family, slot, failure),
    };
    let bounded_value = if before.is_readable() {
        match port.read_bounded_slot(family, slot) {
            Ok(value) => Some(value),
            Err(failure) => return port_failure_slot(family, slot, failure),
        }
    } else {
        None
    };
    let after = match port.observe_slot(family, slot) {
        Ok(observation) => observation,
        Err(failure) => return port_failure_slot(family, slot, failure),
    };
    let failure = if before.exact_ref != after.exact_ref {
        Some(SnapshotGroundingErrorKindV1::SubstitutedReference)
    } else if before != after {
        Some(SnapshotGroundingErrorKindV1::SourceDrift)
    } else {
        None
    };
    CapturedSlotV1 {
        family,
        slot,
        observation: before,
        bounded_value,
        failure,
    }
}

fn final_currentness_recheck<P: SnapshotSourcePortV1>(slots: &mut [CapturedSlotV1], port: &mut P) {
    for slot in slots {
        if slot.failure.is_some() {
            continue;
        }
        if slot.family == SnapshotSourceFamilyV1::Session {
            continue;
        }
        match port.observe_slot(slot.family, slot.slot) {
            Ok(observation) if observation == slot.observation => {}
            Ok(observation) if observation.exact_ref != slot.observation.exact_ref => {
                slot.failure = Some(SnapshotGroundingErrorKindV1::SubstitutedReference);
            }
            Ok(_) => slot.failure = Some(SnapshotGroundingErrorKindV1::SourceDrift),
            Err(SnapshotSourcePortErrorV1::Missing) => {
                slot.failure = Some(SnapshotGroundingErrorKindV1::IncompleteAccounting);
            }
            Err(SnapshotSourcePortErrorV1::Unavailable) => {
                slot.failure = Some(SnapshotGroundingErrorKindV1::RequiredSourceUnavailable);
            }
        }
    }
}

fn port_failure_slot(
    family: SnapshotSourceFamilyV1,
    slot: &'static str,
    failure: SnapshotSourcePortErrorV1,
) -> CapturedSlotV1 {
    let kind = match failure {
        SnapshotSourcePortErrorV1::Missing => SnapshotGroundingErrorKindV1::IncompleteAccounting,
        SnapshotSourcePortErrorV1::Unavailable => {
            SnapshotGroundingErrorKindV1::RequiredSourceUnavailable
        }
    };
    CapturedSlotV1 {
        family,
        slot,
        observation: SnapshotSlotObservationV1::without_payload(
            SnapshotSlotDispositionV1::Unavailable,
            "port_observation_failed",
        ),
        bounded_value: None,
        failure: Some(kind),
    }
}

fn validate_complete_capture(
    owner: &SnapshotGroundingOwnerV1,
    slots: &mut [CapturedSlotV1],
    first_start: bool,
    kind: SnapshotBoundaryKindV1,
    previous: Option<&PersistedGroundingCaptureV1>,
) -> Option<SnapshotGroundingErrorKindV1> {
    let expected = owner
        .source_contracts()
        .iter()
        .flat_map(|contract| {
            contract
                .source_slots()
                .iter()
                .map(move |slot| (contract.family(), *slot))
        })
        .collect::<BTreeSet<_>>();
    let actual = slots
        .iter()
        .map(|slot| (slot.family, slot.slot))
        .collect::<BTreeSet<_>>();
    if slots.len() != expected.len() || actual != expected {
        return Some(SnapshotGroundingErrorKindV1::IncompleteAccounting);
    }
    for slot in slots.iter_mut() {
        if slot.failure.is_none() {
            slot.failure = validate_slot(owner, slot);
        }
    }
    if kind == SnapshotBoundaryKindV1::End && previous.is_none() {
        return Some(SnapshotGroundingErrorKindV1::IncompatiblePredecessor);
    }
    if first_start && kind != SnapshotBoundaryKindV1::Start {
        return Some(SnapshotGroundingErrorKindV1::SequenceConflict);
    }
    slots.iter().find_map(|slot| slot.failure)
}

fn validate_slot(
    owner: &SnapshotGroundingOwnerV1,
    slot: &CapturedSlotV1,
) -> Option<SnapshotGroundingErrorKindV1> {
    let contract = owner
        .source_contracts()
        .iter()
        .find(|contract| contract.family() == slot.family)?;
    match slot.observation.disposition {
        SnapshotSlotDispositionV1::RetainedObservation
        | SnapshotSlotDispositionV1::DerivedBoundedFact => {
            let Some(value) = slot.bounded_value.as_ref() else {
                return Some(SnapshotGroundingErrorKindV1::IncompleteAccounting);
            };
            if !bounded_value_is_safe(value) || !slot_payload_shape_is_admitted(slot, value) {
                return Some(SnapshotGroundingErrorKindV1::RedactionUncertain);
            }
            let Ok(bytes) = canonical_json(value) else {
                return Some(SnapshotGroundingErrorKindV1::MalformedInput);
            };
            if bytes.len() > MAX_BOUNDED_SLOT_BYTES {
                return Some(SnapshotGroundingErrorKindV1::RedactionUncertain);
            }
            let Ok(actual) = DefinitionFingerprint::from_json_value(value) else {
                return Some(SnapshotGroundingErrorKindV1::MalformedInput);
            };
            if slot.observation.fingerprint.as_ref() != Some(&actual) {
                return Some(SnapshotGroundingErrorKindV1::SubstitutedReference);
            }
            None
        }
        SnapshotSlotDispositionV1::Redacted => None,
        disposition => {
            let condition = slot.observation.rule_condition?;
            let rule = contract
                .failure_rules()
                .iter()
                .find(|rule| rule.condition() == condition);
            let Some(rule) = rule else {
                return Some(SnapshotGroundingErrorKindV1::UnknownRule);
            };
            match disposition {
                SnapshotSlotDispositionV1::Omitted | SnapshotSlotDispositionV1::Inapplicable
                    if matches!(
                        rule.outcome(),
                        crate::SnapshotAdapterFailureOutcomeV1::AccountedOmission
                    ) =>
                {
                    None
                }
                SnapshotSlotDispositionV1::Unavailable
                    if matches!(
                        rule.outcome(),
                        crate::SnapshotAdapterFailureOutcomeV1::Unavailable
                    ) =>
                {
                    Some(SnapshotGroundingErrorKindV1::RequiredSourceUnavailable)
                }
                SnapshotSlotDispositionV1::Refused
                    if matches!(
                        rule.outcome(),
                        crate::SnapshotAdapterFailureOutcomeV1::Refused
                    ) =>
                {
                    Some(SnapshotGroundingErrorKindV1::SubstitutedReference)
                }
                _ => Some(SnapshotGroundingErrorKindV1::AdapterMismatch),
            }
        }
    }
}

fn slot_payload_shape_is_admitted(slot: &CapturedSlotV1, value: &Value) -> bool {
    if slot.family == SnapshotSourceFamilyV1::Session && slot.slot == "boundary_context" {
        return exact_object_keys(
            value,
            &[
                "boundary_kind",
                "host_session_ref",
                "predecessor",
                "sequence",
                "stream_fingerprint",
            ],
        );
    }
    if slot.family == SnapshotSourceFamilyV1::Handbook && slot.slot == "schema_artifact_resolution"
    {
        let Some(resolution) = value.get("resolution") else {
            return false;
        };
        return exact_object_keys(value, &["resolution"])
            && exact_object_keys(resolution, &["stack_fingerprint", "stack_ref", "values"]);
    }
    if slot.family == SnapshotSourceFamilyV1::Work && slot.slot == "handoff" {
        return exact_object_keys(value, &["refs"])
            && value
                .get("refs")
                .and_then(Value::as_array)
                .is_some_and(|refs| refs.len() <= MAX_HANDOFF_REFS);
    }
    exact_object_keys(value, &["ref"])
        && value
            .get("ref")
            .and_then(Value::as_str)
            .is_some_and(|reference| ExactDefinitionRef::parse(reference).is_ok())
}

fn exact_object_keys(value: &Value, expected: &[&str]) -> bool {
    value.as_object().is_some_and(|object| {
        object.len() == expected.len() && expected.iter().all(|key| object.contains_key(*key))
    })
}

fn bounded_value_is_safe(value: &Value) -> bool {
    let mut pending = vec![(value, 0usize)];
    let mut visited = 0usize;
    while let Some((value, depth)) = pending.pop() {
        visited += 1;
        if visited > MAX_BOUNDED_VALUE_NODES || depth > MAX_BOUNDED_VALUE_DEPTH {
            return false;
        }
        match value {
            Value::Object(object) => {
                for (key, value) in object {
                    let key = key.to_ascii_lowercase();
                    if [
                        "secret",
                        "credential",
                        "environment",
                        "raw_command",
                        "unrestricted_diff",
                        "transcript",
                        "candidate_body",
                        "evidence_body",
                    ]
                    .iter()
                    .any(|forbidden| key.contains(forbidden))
                    {
                        return false;
                    }
                    pending.push((value, depth + 1));
                }
            }
            Value::Array(values) => {
                pending.extend(values.iter().map(|value| (value, depth + 1)));
            }
            _ => {}
        }
    }
    true
}

fn accounting_value(
    owner: &SnapshotGroundingOwnerV1,
    slots: &[CapturedSlotV1],
    first_start: bool,
) -> Result<Value, SnapshotCaptureErrorV1> {
    let slot_values = slots
        .iter()
        .map(|slot| {
            json!({
                "disposition": disposition_name(slot.observation.disposition),
                "family": slot.family.as_str(),
                "slot": slot.slot,
            })
        })
        .collect::<Vec<_>>();
    let rule_values = owner
        .source_contracts()
        .iter()
        .flat_map(|contract| {
            contract.failure_rules().iter().map(move |rule| {
                let applied = (contract.family() == SnapshotSourceFamilyV1::Session
                    && rule.condition() == "first_start"
                    && first_start)
                    || slots.iter().any(|slot| {
                        slot.family == contract.family()
                            && slot.observation.rule_condition == Some(rule.condition())
                    });
                json!({
                    "condition": rule.condition(),
                    "family": contract.family().as_str(),
                    "result": if applied { "applied" } else { "not_triggered" },
                })
            })
        })
        .collect::<Vec<_>>();
    let currentness = owner
        .source_contracts()
        .iter()
        .map(|contract| {
            json!({
                "basis": currentness_name(contract.currentness()),
                "family": contract.family().as_str(),
                "recheck": "stable_before_and_after",
                "slot_count": contract.source_slots().len(),
            })
        })
        .collect::<Vec<_>>();
    if slot_values.len() != 15 || rule_values.len() != 11 {
        return Err(SnapshotCaptureErrorV1::with_accounting(
            SnapshotGroundingErrorKindV1::IncompleteAccounting,
            slots,
        ));
    }
    Ok(json!({
        "complete_exactly_once": true,
        "currentness": currentness,
        "rules": rule_values,
        "schema": "handbook.snapshot-source-accounting@1.0.0",
        "slots": slot_values,
    }))
}

#[derive(Clone, Debug)]
struct PersistedGroundingCaptureV1 {
    sequence: u64,
    exact_ref: String,
    fingerprint: String,
    slots: Vec<PersistedSlotV1>,
}

#[derive(Clone, Debug)]
struct PersistedSlotV1 {
    family: String,
    slot: String,
    disposition: String,
    fingerprint: Option<String>,
}

fn parse_grounding_capture(
    bytes: &[u8],
) -> Result<PersistedGroundingCaptureV1, SnapshotCaptureErrorV1> {
    let value: Value = serde_json::from_slice(bytes)
        .map_err(|_| SnapshotCaptureErrorV1::new(SnapshotGroundingErrorKindV1::CorruptStore))?;
    let capture = value
        .get("capture")
        .and_then(Value::as_object)
        .ok_or_else(|| SnapshotCaptureErrorV1::new(SnapshotGroundingErrorKindV1::CorruptStore))?;
    let exact_ref = capture
        .get("ref")
        .and_then(Value::as_str)
        .ok_or_else(|| SnapshotCaptureErrorV1::new(SnapshotGroundingErrorKindV1::CorruptStore))?
        .to_owned();
    let fingerprint = capture
        .get("fingerprint")
        .and_then(Value::as_str)
        .ok_or_else(|| SnapshotCaptureErrorV1::new(SnapshotGroundingErrorKindV1::CorruptStore))?
        .to_owned();
    let sequence = capture
        .get("sequence")
        .and_then(Value::as_u64)
        .ok_or_else(|| SnapshotCaptureErrorV1::new(SnapshotGroundingErrorKindV1::CorruptStore))?;
    let authored = capture
        .get("slots")
        .and_then(Value::as_array)
        .ok_or_else(|| SnapshotCaptureErrorV1::new(SnapshotGroundingErrorKindV1::CorruptStore))?;
    let mut slots = Vec::with_capacity(authored.len());
    for slot in authored {
        slots.push(PersistedSlotV1 {
            family: required_string(slot, "family")?.to_owned(),
            slot: required_string(slot, "slot")?.to_owned(),
            disposition: required_string(slot, "disposition")?.to_owned(),
            fingerprint: slot
                .get("fingerprint")
                .and_then(Value::as_str)
                .map(ToOwned::to_owned),
        });
    }
    if slots.len() != 15 {
        return Err(SnapshotCaptureErrorV1::new(
            SnapshotGroundingErrorKindV1::IncompleteAccounting,
        ));
    }
    let mut body = Value::Object(capture.clone());
    let body = body
        .as_object_mut()
        .expect("cloned capture object remains an object");
    body.remove("ref");
    body.remove("fingerprint");
    let derived = DefinitionFingerprint::from_json_value(&Value::Object(body.clone()))
        .map_err(|_| SnapshotCaptureErrorV1::new(SnapshotGroundingErrorKindV1::CorruptStore))?;
    if fingerprint != derived.as_str()
        || exact_ref != format!("handbook.snapshot-capture@{}", derived.as_str())
    {
        return Err(SnapshotCaptureErrorV1::new(
            SnapshotGroundingErrorKindV1::CorruptStore,
        ));
    }
    Ok(PersistedGroundingCaptureV1 {
        sequence,
        exact_ref,
        fingerprint,
        slots,
    })
}

fn delta_value(
    prior: &PersistedGroundingCaptureV1,
    current: &[CapturedSlotV1],
    sequence: u64,
    kind: SnapshotBoundaryKindV1,
) -> Result<Value, SnapshotCaptureErrorV1> {
    if prior.slots.len() != current.len() || prior.sequence >= sequence {
        return Err(SnapshotCaptureErrorV1::new(
            SnapshotGroundingErrorKindV1::IncompatiblePredecessor,
        ));
    }
    let evaluations = prior
        .slots
        .iter()
        .zip(current)
        .map(|(before, after)| {
            if before.family != after.family.as_str() || before.slot != after.slot {
                return Err(SnapshotCaptureErrorV1::new(
                    SnapshotGroundingErrorKindV1::IncompatiblePredecessor,
                ));
            }
            let after_fingerprint = after
                .observation
                .fingerprint
                .as_ref()
                .map(ToString::to_string);
            let after_disposition = disposition_name(after.observation.disposition);
            Ok(json!({
                "after_disposition": after_disposition,
                "after_fingerprint": after_fingerprint,
                "before_disposition": before.disposition,
                "before_fingerprint": before.fingerprint,
                "classification": if before.fingerprint == after_fingerprint && before.disposition == after_disposition { "unchanged" } else { "changed" },
                "family": before.family,
                "slot": before.slot,
            }))
        })
        .collect::<Result<Vec<_>, _>>()?;
    let current_capture = capture_value(current, sequence)?;
    Ok(json!({
        "after": {"ref":current_capture["ref"],"fingerprint":current_capture["fingerprint"]},
        "after_sequence": sequence,
        "before": {"ref":prior.exact_ref,"fingerprint":prior.fingerprint},
        "before_sequence": prior.sequence,
        "evaluations": evaluations,
        "pair_kind": if kind == SnapshotBoundaryKindV1::End { "open_start_to_end" } else { "prior_end_to_start" },
        "schema": "handbook.snapshot-compatible-delta@1.0.0",
    }))
}

fn grounding_value(
    owner: &SnapshotGroundingOwnerV1,
    request: &SnapshotCaptureRequestV1,
    slots: &[CapturedSlotV1],
    sequence: u64,
    predecessor: &SnapshotPredecessorV1,
    delta: Option<&Value>,
    resolution: &ResolutionObservationV1,
) -> Result<Value, SnapshotCaptureErrorV1> {
    let sources = owner
        .production_sources()
        .map_err(|_| SnapshotCaptureErrorV1::new(SnapshotGroundingErrorKindV1::PolicyMismatch))?;
    let omitted = slots
        .iter()
        .filter(|slot| {
            matches!(
                slot.observation.disposition,
                SnapshotSlotDispositionV1::Redacted
                    | SnapshotSlotDispositionV1::Omitted
                    | SnapshotSlotDispositionV1::Inapplicable
            )
        })
        .map(|slot| json!({"family":slot.family.as_str(),"slot":slot.slot,"disposition":disposition_name(slot.observation.disposition)}))
        .collect::<Vec<_>>();
    let handoff_refs = extract_handoff_refs(slots)?;
    let capture = capture_value(slots, sequence)?;
    Ok(json!({
        "boundary_kind": if request.kind == SnapshotBoundaryKindV1::Start { "start" } else { "end" },
        "capture": capture,
        "delta_fingerprint": delta.map(DefinitionFingerprint::from_json_value).transpose().map_err(|_| SnapshotCaptureErrorV1::new(SnapshotGroundingErrorKindV1::MalformedInput))?.map(|value| value.to_string()),
        "grounding_type": if request.kind == SnapshotBoundaryKindV1::Start { "start_grounding" } else { "final_grounding" },
        "handoff_refs": handoff_refs,
        "predecessor": predecessor,
        "projection": {
            "definition": {"ref":sources.grounding_definition().exact_ref(),"fingerprint":sources.grounding_definition().fingerprint()},
            "disclosure": {"ref":sources.disclosure_policy().exact_ref(),"fingerprint":sources.disclosure_policy().fingerprint()},
            "lossiness": if omitted.is_empty() { "bounded_lossless" } else { "bounded_lossy" },
            "omissions": omitted,
            "refusals": [],
            "required_rechecks": owner.source_contracts().iter().map(|contract| json!({"basis":currentness_name(contract.currentness()),"family":contract.family().as_str()})).collect::<Vec<_>>(),
            "resolution": {"stack_ref":resolution.stack_ref(),"stack_fingerprint":resolution.stack_fingerprint(),"values":resolution.values()},
        },
        "schema": "handbook.snapshot-production-grounding@1.0.0",
    }))
}

fn capture_value(slots: &[CapturedSlotV1], sequence: u64) -> Result<Value, SnapshotCaptureErrorV1> {
    let captured_slots = slots
        .iter()
        .map(|slot| {
            json!({
                "bounded_value": slot.bounded_value,
                "disposition": disposition_name(slot.observation.disposition),
                "family": slot.family.as_str(),
                "fingerprint": slot.observation.fingerprint.as_ref().map(ToString::to_string),
                "ref": slot.observation.exact_ref,
                "slot": slot.slot,
            })
        })
        .collect::<Vec<_>>();
    let body = json!({"sequence":sequence,"slots":captured_slots});
    let fingerprint = DefinitionFingerprint::from_json_value(&body)
        .map_err(|_| SnapshotCaptureErrorV1::new(SnapshotGroundingErrorKindV1::MalformedInput))?;
    Ok(json!({
        "fingerprint": fingerprint.as_str(),
        "ref": format!("handbook.snapshot-capture@{}", fingerprint.as_str()),
        "sequence": sequence,
        "slots": body["slots"],
    }))
}

fn derive_resolution(
    slots: &[CapturedSlotV1],
) -> Result<ResolutionObservationV1, SnapshotCaptureErrorV1> {
    let value = slots
        .iter()
        .find(|slot| {
            slot.family == SnapshotSourceFamilyV1::Handbook
                && slot.slot == "schema_artifact_resolution"
        })
        .and_then(|slot| slot.bounded_value.as_ref())
        .and_then(|value| value.get("resolution"))
        .ok_or_else(|| {
            SnapshotCaptureErrorV1::new(SnapshotGroundingErrorKindV1::InsufficientResolution)
        })?;
    let stack_ref = value
        .get("stack_ref")
        .and_then(Value::as_str)
        .ok_or_else(|| {
            SnapshotCaptureErrorV1::new(SnapshotGroundingErrorKindV1::InsufficientResolution)
        })?;
    let stack_fingerprint = value
        .get("stack_fingerprint")
        .and_then(Value::as_str)
        .ok_or_else(|| {
            SnapshotCaptureErrorV1::new(SnapshotGroundingErrorKindV1::InsufficientResolution)
        })?;
    let authored_values = value
        .get("values")
        .and_then(Value::as_array)
        .filter(|values| values.len() == 6)
        .ok_or_else(|| {
            SnapshotCaptureErrorV1::new(SnapshotGroundingErrorKindV1::InsufficientResolution)
        })?;
    let mut values = [""; 6];
    for (index, authored) in authored_values.iter().enumerate() {
        values[index] = authored.as_str().ok_or_else(|| {
            SnapshotCaptureErrorV1::new(SnapshotGroundingErrorKindV1::InsufficientResolution)
        })?;
    }
    ResolutionObservationV1::new(stack_ref, stack_fingerprint, values).map_err(|_| {
        SnapshotCaptureErrorV1::new(SnapshotGroundingErrorKindV1::InsufficientResolution)
    })
}

fn extract_handoff_refs(slots: &[CapturedSlotV1]) -> Result<Vec<String>, SnapshotCaptureErrorV1> {
    let Some(handoff) = slots
        .iter()
        .find(|slot| slot.family == SnapshotSourceFamilyV1::Work && slot.slot == "handoff")
    else {
        return Err(SnapshotCaptureErrorV1::new(
            SnapshotGroundingErrorKindV1::IncompleteAccounting,
        ));
    };
    let Some(refs) = handoff
        .bounded_value
        .as_ref()
        .and_then(|value| value.get("refs"))
        .and_then(Value::as_array)
    else {
        return Ok(Vec::new());
    };
    if refs.len() > MAX_HANDOFF_REFS {
        return Err(SnapshotCaptureErrorV1::new(
            SnapshotGroundingErrorKindV1::InputLimitExceeded,
        ));
    }
    let mut result = Vec::with_capacity(refs.len());
    for reference in refs {
        let reference = reference.as_str().ok_or_else(|| {
            SnapshotCaptureErrorV1::new(SnapshotGroundingErrorKindV1::MalformedInput)
        })?;
        if !is_bounded_handoff_ref(reference) {
            return Err(SnapshotCaptureErrorV1::new(
                SnapshotGroundingErrorKindV1::MalformedInput,
            ));
        }
        result.push(reference.to_owned());
    }
    if !result.windows(2).all(|pair| pair[0] < pair[1]) {
        return Err(SnapshotCaptureErrorV1::new(
            SnapshotGroundingErrorKindV1::MalformedInput,
        ));
    }
    Ok(result)
}

fn is_bounded_handoff_ref(reference: &str) -> bool {
    !reference.is_empty()
        && reference.len() <= 256
        && reference.bytes().all(|byte| {
            byte.is_ascii_alphanumeric() || matches!(byte, b'-' | b'.' | b'_' | b'/' | b'@' | b':')
        })
}

fn outcome_from_commit(
    repository: &SnapshotRepositoryV1<'_>,
    commit: SnapshotRepositoryCommitV1,
) -> Result<SnapshotCaptureOutcomeV1, SnapshotCaptureErrorV1> {
    let grounding_bytes = repository
        .read_record(commit.grounding_record())
        .map_err(map_repository_error)?;
    let grounding: Value = serde_json::from_slice(&grounding_bytes)
        .map_err(|_| SnapshotCaptureErrorV1::new(SnapshotGroundingErrorKindV1::CorruptStore))?;
    let boundary_kind = required_string(&grounding, "boundary_kind")?;
    let handoff_refs = grounding
        .get("handoff_refs")
        .and_then(Value::as_array)
        .ok_or_else(|| SnapshotCaptureErrorV1::new(SnapshotGroundingErrorKindV1::CorruptStore))?
        .iter()
        .map(|value| {
            value.as_str().map(ToOwned::to_owned).ok_or_else(|| {
                SnapshotCaptureErrorV1::new(SnapshotGroundingErrorKindV1::CorruptStore)
            })
        })
        .collect::<Result<Vec<_>, _>>()?;
    let first_start = matches!(
        commit.predecessor(),
        SnapshotPredecessorV1::NoApplicablePrior
    );
    let committed = committed_grounding(commit);
    match boundary_kind {
        "start" => Ok(SnapshotCaptureOutcomeV1::Started(
            SnapshotStartedGroundingV1 {
                committed,
                first_start,
            },
        )),
        "end" => Ok(SnapshotCaptureOutcomeV1::Finalized(
            SnapshotFinalGroundingV1 {
                committed,
                handoff_refs,
            },
        )),
        _ => Err(SnapshotCaptureErrorV1::new(
            SnapshotGroundingErrorKindV1::CorruptStore,
        )),
    }
}

fn committed_grounding(commit: SnapshotRepositoryCommitV1) -> SnapshotCommittedGroundingV1 {
    let predecessor_boundary_ref = match commit.predecessor() {
        SnapshotPredecessorV1::NoApplicablePrior => None,
        SnapshotPredecessorV1::ApplicablePrior { boundary_ref, .. } => Some(boundary_ref.clone()),
    };
    SnapshotCommittedGroundingV1 {
        sequence: commit.sequence(),
        predecessor_boundary_ref,
        boundary_record: committed_record(commit.boundary_record()),
        accounting_record: committed_record(commit.accounting_record()),
        delta_record: commit.delta_record().map(committed_record),
        grounding_record: committed_record(commit.grounding_record()),
        receipt_record: committed_record(commit.receipt_record()),
        replayed: commit.replayed(),
        repository_commit: commit,
    }
}

fn committed_record(record: &SnapshotRecordRefV1) -> SnapshotCommittedRecordV1 {
    SnapshotCommittedRecordV1 {
        exact_ref: record.exact_ref().to_owned(),
        fingerprint: record.fingerprint().to_owned(),
    }
}

fn disposition_name(disposition: SnapshotSlotDispositionV1) -> &'static str {
    match disposition {
        SnapshotSlotDispositionV1::RetainedObservation => "retained_observation",
        SnapshotSlotDispositionV1::DerivedBoundedFact => "derived_bounded_fact",
        SnapshotSlotDispositionV1::Redacted => "redacted",
        SnapshotSlotDispositionV1::Omitted => "omitted",
        SnapshotSlotDispositionV1::Inapplicable => "inapplicable",
        SnapshotSlotDispositionV1::Unavailable => "unavailable",
        SnapshotSlotDispositionV1::Refused => "refused",
    }
}

fn currentness_name(currentness: crate::SnapshotSourceCurrentnessV1) -> &'static str {
    use crate::SnapshotSourceCurrentnessV1 as Currentness;
    match currentness {
        Currentness::ContractsAndDockOwnerHeads => "contracts_and_dock_owner_heads",
        Currentness::RepositoryTripletStable => "repository_triplet_stable",
        Currentness::HandbookAuthorityHeadStable => "handbook_authority_head_stable",
        Currentness::RequestAndStreamHeadLocked => "request_and_stream_head_locked",
        Currentness::FlowPipelineHeadsStable => "flow_pipeline_heads_stable",
    }
}

fn required_string<'a>(value: &'a Value, field: &str) -> Result<&'a str, SnapshotCaptureErrorV1> {
    value
        .get(field)
        .and_then(Value::as_str)
        .ok_or_else(|| SnapshotCaptureErrorV1::new(SnapshotGroundingErrorKindV1::CorruptStore))
}

fn canonical_json(value: &Value) -> Result<Vec<u8>, SnapshotCaptureErrorV1> {
    serde_json_canonicalizer::to_vec(value)
        .map_err(|_| SnapshotCaptureErrorV1::new(SnapshotGroundingErrorKindV1::MalformedInput))
}

fn map_repository_error(error: SnapshotRepositoryErrorV1) -> SnapshotCaptureErrorV1 {
    let kind = match error.kind() {
        SnapshotRepositoryErrorKindV1::MalformedInput => {
            SnapshotGroundingErrorKindV1::MalformedInput
        }
        SnapshotRepositoryErrorKindV1::RepositoryMismatch => {
            SnapshotGroundingErrorKindV1::RepositoryMismatch
        }
        SnapshotRepositoryErrorKindV1::SequenceConflict => {
            SnapshotGroundingErrorKindV1::SequenceConflict
        }
        SnapshotRepositoryErrorKindV1::IdempotencyConflict => {
            SnapshotGroundingErrorKindV1::IdempotencyConflict
        }
        SnapshotRepositoryErrorKindV1::RecordNotFound
        | SnapshotRepositoryErrorKindV1::CorruptStore => SnapshotGroundingErrorKindV1::CorruptStore,
        SnapshotRepositoryErrorKindV1::LockUnavailable => {
            SnapshotGroundingErrorKindV1::LockUnavailable
        }
        SnapshotRepositoryErrorKindV1::RecoveryFailed => {
            SnapshotGroundingErrorKindV1::RecoveryFailed
        }
        SnapshotRepositoryErrorKindV1::DurabilityUncertain => {
            SnapshotGroundingErrorKindV1::DurabilityUncertain
        }
        SnapshotRepositoryErrorKindV1::MaintenanceDeferred => {
            SnapshotGroundingErrorKindV1::RetentionConflict
        }
    };
    SnapshotCaptureErrorV1::new(kind)
}
