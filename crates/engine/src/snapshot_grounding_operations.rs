use crate::snapshot_grounding_repository::{
    SnapshotRepositoryErrorKindV1, SnapshotRepositoryErrorV1,
};
use crate::{
    DefinitionFingerprint, SnapshotBoundaryKindV1, SnapshotGroundingErrorKindV1,
    SnapshotGroundingOwnerV1, SnapshotRepositoryBindingV1, SnapshotStreamIdentityV1,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SnapshotRecordBindingV1 {
    pub class: String,
    pub exact_ref: String,
    pub fingerprint: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case", tag = "outcome", deny_unknown_fields)]
pub enum SnapshotPredecessorOutcomeV1 {
    NoApplicablePrior,
    ApplicablePrior {
        boundary_ref: String,
        boundary_fingerprint: String,
    },
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SnapshotAccountingEntryV1 {
    pub family: String,
    pub slot: String,
    pub disposition: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SnapshotDescriptorV1 {
    pub boundary: SnapshotRecordBindingV1,
    pub boundary_kind: String,
    pub sequence: u64,
    pub predecessor: SnapshotPredecessorOutcomeV1,
    pub accounting: SnapshotRecordBindingV1,
    pub delta: Option<SnapshotRecordBindingV1>,
    pub grounding: SnapshotRecordBindingV1,
    pub source_accounting: Vec<SnapshotAccountingEntryV1>,
    pub current: bool,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SnapshotDeltaProjectionV1 {
    pub delta: SnapshotRecordBindingV1,
    pub before_ref: String,
    pub before_fingerprint: String,
    pub before_sequence: u64,
    pub after_ref: String,
    pub after_fingerprint: String,
    pub after_sequence: u64,
    pub pair_kind: String,
    pub changed: u64,
    pub unchanged: u64,
    pub current: bool,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SnapshotRequiredRecheckV1 {
    pub family: String,
    pub basis: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SnapshotProjectionV1 {
    pub grounding: SnapshotRecordBindingV1,
    pub capture_ref: String,
    pub capture_fingerprint: String,
    pub boundary_kind: String,
    pub grounding_type: String,
    pub definition_ref: String,
    pub definition_fingerprint: String,
    pub disclosure_ref: String,
    pub disclosure_fingerprint: String,
    pub lossiness: String,
    pub omissions: Vec<SnapshotAccountingEntryV1>,
    pub refusals: Vec<SnapshotAccountingEntryV1>,
    pub required_rechecks: Vec<SnapshotRequiredRecheckV1>,
    pub resolution_stack_ref: String,
    pub resolution_stack_fingerprint: String,
    pub resolution_values: [String; 6],
    pub handoff_refs: Vec<String>,
    pub current: bool,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SnapshotOperationErrorV1 {
    kind: SnapshotGroundingErrorKindV1,
    detail: &'static str,
}

impl SnapshotOperationErrorV1 {
    pub fn kind(&self) -> SnapshotGroundingErrorKindV1 {
        self.kind
    }
    pub fn detail(&self) -> &'static str {
        self.detail
    }
}

impl SnapshotGroundingOwnerV1 {
    pub fn read_current_snapshot(
        &self,
        binding: &SnapshotRepositoryBindingV1,
        stream: &SnapshotStreamIdentityV1,
        expected_boundary_ref: &str,
        expected_boundary_fingerprint: &str,
    ) -> Result<Option<SnapshotDescriptorV1>, SnapshotOperationErrorV1> {
        let repository = self
            .open_repository(binding, stream)
            .map_err(repository_error)?;
        let Some(head) = repository.head().map_err(repository_error)? else {
            return Ok(None);
        };
        if head.boundary_record().exact_ref() != expected_boundary_ref
            || head.boundary_record().fingerprint() != expected_boundary_fingerprint
        {
            return Err(operation_error(
                SnapshotGroundingErrorKindV1::SubstitutedReference,
                "expected boundary is not the owner-derived current head",
            ));
        }
        let boundary_bytes = repository
            .read_record(head.boundary_record())
            .map_err(repository_error)?;
        let boundary: BoundaryRecord =
            serde_json::from_slice(&boundary_bytes).map_err(|_| corrupt())?;
        if boundary.sequence != head.sequence()
            || boundary.boundary_kind != boundary_kind(head.kind())
            || DefinitionFingerprint::parse(&boundary.boundary_fingerprint).is_err()
            || boundary.stream_fingerprint != stream.fingerprint()
            || boundary.schema != "handbook.snapshot-memory.boundary@1.0.0"
        {
            return Err(corrupt());
        }
        let accounting_bytes = repository
            .read_record_binding(
                &boundary.accounting.class,
                &boundary.accounting.exact_ref,
                &boundary.accounting.fingerprint,
            )
            .map_err(repository_error)?;
        let accounting: AccountingRecord =
            serde_json::from_slice(&accounting_bytes).map_err(|_| corrupt())?;
        if !accounting.complete_exactly_once
            || accounting.schema != "handbook.snapshot-source-accounting@1.0.0"
            || accounting.slots.len() != 15
        {
            return Err(operation_error(
                SnapshotGroundingErrorKindV1::IncompleteAccounting,
                "snapshot source accounting is incomplete",
            ));
        }
        let current = repository
            .head()
            .map_err(repository_error)?
            .is_some_and(|after| after.boundary_record() == head.boundary_record());
        if !current {
            return Err(operation_error(
                SnapshotGroundingErrorKindV1::SourceDrift,
                "snapshot head changed during read",
            ));
        }
        Ok(Some(SnapshotDescriptorV1 {
            boundary: SnapshotRecordBindingV1 {
                class: head.boundary_record().class().to_owned(),
                exact_ref: expected_boundary_ref.to_owned(),
                fingerprint: expected_boundary_fingerprint.to_owned(),
            },
            boundary_kind: boundary.boundary_kind,
            sequence: boundary.sequence,
            predecessor: boundary.predecessor,
            accounting: boundary.accounting,
            delta: boundary.delta,
            grounding: boundary.grounding,
            source_accounting: accounting.slots,
            current,
        }))
    }

    pub fn read_current_delta(
        &self,
        binding: &SnapshotRepositoryBindingV1,
        stream: &SnapshotStreamIdentityV1,
        expected_boundary_ref: &str,
        expected_boundary_fingerprint: &str,
    ) -> Result<Option<SnapshotDeltaProjectionV1>, SnapshotOperationErrorV1> {
        let Some(descriptor) = self.read_current_snapshot(
            binding,
            stream,
            expected_boundary_ref,
            expected_boundary_fingerprint,
        )?
        else {
            return Ok(None);
        };
        let Some(delta) = descriptor.delta else {
            return Ok(None);
        };
        let repository = self
            .open_repository(binding, stream)
            .map_err(repository_error)?;
        let bytes = repository
            .read_record_binding(&delta.class, &delta.exact_ref, &delta.fingerprint)
            .map_err(repository_error)?;
        let authored: DeltaRecord = serde_json::from_slice(&bytes).map_err(|_| corrupt())?;
        if authored.schema != "handbook.snapshot-compatible-delta@1.0.0"
            || authored.after_sequence != descriptor.sequence
        {
            return Err(corrupt());
        }
        let changed = authored
            .evaluations
            .iter()
            .filter(|row| row.classification == "changed")
            .count() as u64;
        let unchanged = authored
            .evaluations
            .iter()
            .filter(|row| row.classification == "unchanged")
            .count() as u64;
        if changed + unchanged != 15 {
            return Err(corrupt());
        }
        Ok(Some(SnapshotDeltaProjectionV1 {
            delta,
            before_ref: authored.before.exact_ref,
            before_fingerprint: authored.before.fingerprint,
            before_sequence: authored.before_sequence,
            after_ref: authored.after.exact_ref,
            after_fingerprint: authored.after.fingerprint,
            after_sequence: authored.after_sequence,
            pair_kind: authored.pair_kind,
            changed,
            unchanged,
            current: true,
        }))
    }

    pub fn read_current_projection(
        &self,
        binding: &SnapshotRepositoryBindingV1,
        stream: &SnapshotStreamIdentityV1,
        expected_boundary_ref: &str,
        expected_boundary_fingerprint: &str,
    ) -> Result<Option<SnapshotProjectionV1>, SnapshotOperationErrorV1> {
        let Some(descriptor) = self.read_current_snapshot(
            binding,
            stream,
            expected_boundary_ref,
            expected_boundary_fingerprint,
        )?
        else {
            return Ok(None);
        };
        let repository = self
            .open_repository(binding, stream)
            .map_err(repository_error)?;
        let grounding = descriptor.grounding;
        let bytes = repository
            .read_record_binding(
                &grounding.class,
                &grounding.exact_ref,
                &grounding.fingerprint,
            )
            .map_err(repository_error)?;
        let authored: GroundingRecord = serde_json::from_slice(&bytes).map_err(|_| corrupt())?;
        if authored.schema != "handbook.snapshot-production-grounding@1.0.0" {
            return Err(corrupt());
        }
        Ok(Some(SnapshotProjectionV1 {
            grounding,
            capture_ref: authored.capture.exact_ref,
            capture_fingerprint: authored.capture.fingerprint,
            boundary_kind: authored.boundary_kind,
            grounding_type: authored.grounding_type,
            definition_ref: authored.projection.definition.exact_ref,
            definition_fingerprint: authored.projection.definition.fingerprint,
            disclosure_ref: authored.projection.disclosure.exact_ref,
            disclosure_fingerprint: authored.projection.disclosure.fingerprint,
            lossiness: authored.projection.lossiness,
            omissions: authored.projection.omissions,
            refusals: authored.projection.refusals,
            required_rechecks: authored.projection.required_rechecks,
            resolution_stack_ref: authored.projection.resolution.stack_ref,
            resolution_stack_fingerprint: authored.projection.resolution.stack_fingerprint,
            resolution_values: authored.projection.resolution.values,
            handoff_refs: authored.handoff_refs,
            current: true,
        }))
    }
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct BoundaryRecord {
    accounting: SnapshotRecordBindingV1,
    boundary_fingerprint: String,
    boundary_kind: String,
    delta: Option<SnapshotRecordBindingV1>,
    grounding: SnapshotRecordBindingV1,
    #[allow(dead_code)]
    host_session_ref: String,
    predecessor: SnapshotPredecessorOutcomeV1,
    schema: String,
    sequence: u64,
    stream_fingerprint: String,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct AccountingRecord {
    complete_exactly_once: bool,
    #[allow(dead_code)]
    currentness: Vec<Value>,
    #[allow(dead_code)]
    rules: Vec<Value>,
    schema: String,
    slots: Vec<SnapshotAccountingEntryV1>,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct ExactPairBinding {
    #[serde(rename = "ref")]
    exact_ref: String,
    fingerprint: String,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct DeltaEvaluation {
    #[allow(dead_code)]
    after_disposition: String,
    #[allow(dead_code)]
    after_fingerprint: Option<String>,
    #[allow(dead_code)]
    before_disposition: String,
    #[allow(dead_code)]
    before_fingerprint: Option<String>,
    classification: String,
    #[allow(dead_code)]
    family: String,
    #[allow(dead_code)]
    slot: String,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct DeltaRecord {
    after: ExactPairBinding,
    after_sequence: u64,
    before: ExactPairBinding,
    before_sequence: u64,
    evaluations: Vec<DeltaEvaluation>,
    pair_kind: String,
    schema: String,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct GroundingRecord {
    boundary_kind: String,
    capture: GroundingCapture,
    #[allow(dead_code)]
    delta_fingerprint: Option<String>,
    grounding_type: String,
    handoff_refs: Vec<String>,
    #[allow(dead_code)]
    predecessor: Value,
    projection: GroundingProjection,
    schema: String,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct GroundingCapture {
    #[serde(rename = "ref")]
    exact_ref: String,
    fingerprint: String,
    #[allow(dead_code)]
    sequence: u64,
    #[allow(dead_code)]
    slots: Vec<Value>,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct GroundingProjection {
    definition: ExactPairBinding,
    disclosure: ExactPairBinding,
    lossiness: String,
    omissions: Vec<SnapshotAccountingEntryV1>,
    refusals: Vec<SnapshotAccountingEntryV1>,
    required_rechecks: Vec<SnapshotRequiredRecheckV1>,
    resolution: GroundingResolution,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct GroundingResolution {
    stack_ref: String,
    stack_fingerprint: String,
    values: [String; 6],
}

fn boundary_kind(kind: SnapshotBoundaryKindV1) -> &'static str {
    match kind {
        SnapshotBoundaryKindV1::Start => "start",
        SnapshotBoundaryKindV1::End => "end",
    }
}

fn repository_error(error: SnapshotRepositoryErrorV1) -> SnapshotOperationErrorV1 {
    use SnapshotRepositoryErrorKindV1 as K;
    let kind = match error.kind() {
        K::MalformedInput => SnapshotGroundingErrorKindV1::MalformedInput,
        K::RepositoryMismatch => SnapshotGroundingErrorKindV1::RepositoryMismatch,
        K::SequenceConflict => SnapshotGroundingErrorKindV1::SequenceConflict,
        K::IdempotencyConflict => SnapshotGroundingErrorKindV1::IdempotencyConflict,
        K::LockUnavailable => SnapshotGroundingErrorKindV1::LockUnavailable,
        K::RecoveryFailed => SnapshotGroundingErrorKindV1::RecoveryFailed,
        K::CorruptStore => SnapshotGroundingErrorKindV1::CorruptStore,
        K::RecordNotFound => SnapshotGroundingErrorKindV1::UnknownReference,
        K::MaintenanceDeferred => SnapshotGroundingErrorKindV1::RequiredSourceUnavailable,
        K::DurabilityUncertain => SnapshotGroundingErrorKindV1::DurabilityUncertain,
    };
    operation_error(kind, error.detail())
}

fn corrupt() -> SnapshotOperationErrorV1 {
    operation_error(
        SnapshotGroundingErrorKindV1::CorruptStore,
        "snapshot repository record is corrupt",
    )
}
fn operation_error(
    kind: SnapshotGroundingErrorKindV1,
    detail: &'static str,
) -> SnapshotOperationErrorV1 {
    SnapshotOperationErrorV1 { kind, detail }
}
