use crate::DefinitionFingerprint;
use serde::Deserialize;
use std::collections::{BTreeMap, BTreeSet};

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub(super) enum RedactionAction {
    Omit,
    FingerprintOnly,
    ArtifactRefOnly,
    RedactedSummary,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) enum SurfaceClassification {
    Secret,
    UnrestrictedEnvironment,
    SecretFile,
    RawCommand,
    UnrestrictedDiff,
    Known,
    Unknown,
}

impl SurfaceClassification {
    fn has_omit_floor(self) -> bool {
        matches!(
            self,
            Self::Secret
                | Self::UnrestrictedEnvironment
                | Self::SecretFile
                | Self::RawCommand
                | Self::UnrestrictedDiff
                | Self::Unknown
        )
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(super) enum MatcherOutcome {
    Matched(Vec<RedactionAction>),
    KnownUnmatched,
    MatcherFailed,
    Unknown,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) enum RedactionErrorKind {
    IncomparableNonOmitActions,
    InvalidJsonPointer,
    RetainedPointerMismatch,
    RetainedPointerInsideOriginal,
    InvalidFixture,
    RetentionUncovered,
    RetentionOverlap,
    HeldRecord,
    ReferencedRecord,
    UnexpiredRetentionFloor,
    DuplicateRecordIdentity,
    InvalidRecordIdentity,
    CompactionReviewRequired,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(super) struct RedactionError {
    kind: RedactionErrorKind,
}

impl RedactionError {
    fn new(kind: RedactionErrorKind) -> Self {
        Self { kind }
    }

    pub(super) fn kind(&self) -> RedactionErrorKind {
        self.kind
    }
}

pub(super) fn evaluate_redaction(
    surface: SurfaceClassification,
    matcher: MatcherOutcome,
) -> Result<RedactionAction, RedactionError> {
    if surface.has_omit_floor() {
        return Ok(RedactionAction::Omit);
    }

    let MatcherOutcome::Matched(actions) = matcher else {
        return Ok(RedactionAction::Omit);
    };
    if actions.is_empty() || actions.contains(&RedactionAction::Omit) {
        return Ok(RedactionAction::Omit);
    }

    let action = actions[0];
    if actions.iter().all(|candidate| *candidate == action) {
        Ok(action)
    } else {
        Err(RedactionError::new(
            RedactionErrorKind::IncomparableNonOmitActions,
        ))
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) enum PointerRead {
    Redacted,
    IndependentlyClassified,
}

pub(super) fn validate_disposition(
    original_pointer: &str,
    action: RedactionAction,
    retained_pointer: Option<&str>,
) -> Result<(), RedactionError> {
    if !is_valid_json_pointer(original_pointer) {
        return Err(RedactionError::new(RedactionErrorKind::InvalidJsonPointer));
    }

    match (action, retained_pointer) {
        (RedactionAction::Omit, None) => Ok(()),
        (RedactionAction::Omit, Some(_)) | (_, None) => Err(RedactionError::new(
            RedactionErrorKind::RetainedPointerMismatch,
        )),
        (_, Some(retained_pointer)) => {
            if !is_valid_json_pointer(retained_pointer) {
                return Err(RedactionError::new(RedactionErrorKind::InvalidJsonPointer));
            }
            if pointer_is_same_or_descendant(retained_pointer, original_pointer) {
                return Err(RedactionError::new(
                    RedactionErrorKind::RetainedPointerInsideOriginal,
                ));
            }
            Ok(())
        }
    }
}

pub(super) fn classify_pointer_read(
    original_pointer: &str,
    requested_pointer: &str,
) -> Result<PointerRead, RedactionError> {
    if !is_valid_json_pointer(original_pointer) || !is_valid_json_pointer(requested_pointer) {
        return Err(RedactionError::new(RedactionErrorKind::InvalidJsonPointer));
    }

    Ok(
        if pointer_is_same_or_descendant(requested_pointer, original_pointer) {
            PointerRead::Redacted
        } else {
            PointerRead::IndependentlyClassified
        },
    )
}

fn is_valid_json_pointer(pointer: &str) -> bool {
    if pointer.is_empty() {
        return true;
    }
    if !pointer.starts_with('/') {
        return false;
    }

    pointer
        .split('/')
        .skip(1)
        .all(|segment| valid_pointer_segment(segment))
}

fn valid_pointer_segment(segment: &str) -> bool {
    let bytes = segment.as_bytes();
    let mut index = 0;
    while index < bytes.len() {
        if bytes[index] == b'~' {
            if index + 1 >= bytes.len() || !matches!(bytes[index + 1], b'0' | b'1') {
                return false;
            }
            index += 2;
        } else {
            index += 1;
        }
    }
    true
}

fn pointer_is_same_or_descendant(pointer: &str, original_pointer: &str) -> bool {
    pointer == original_pointer
        || original_pointer.is_empty()
        || pointer
            .strip_prefix(original_pointer)
            .is_some_and(|suffix| suffix.starts_with('/'))
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
#[serde(deny_unknown_fields)]
pub(super) struct RedactionRetentionFixture {
    pub(super) redaction: FixtureRedactionPolicy,
    pub(super) retention: RetentionCatalog,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
#[serde(deny_unknown_fields)]
pub(super) struct FixtureRedactionPolicy {
    pub(super) fail_closed: bool,
    pub(super) unmatched_action: RedactionAction,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
#[serde(deny_unknown_fields)]
pub(super) struct RetentionCatalog {
    pub(super) allowed_tuples: Vec<RetentionTuple>,
    pub(super) rules: Vec<RetentionRule>,
}

#[derive(Clone, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd)]
#[serde(deny_unknown_fields)]
pub(super) struct RetentionTuple {
    pub(super) memory_horizon: String,
    pub(super) trigger: String,
    pub(super) record_class: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
#[serde(deny_unknown_fields)]
pub(super) struct RetentionRule {
    pub(super) rule_id: String,
    pub(super) memory_horizon: String,
    pub(super) trigger: String,
    pub(super) record_class: String,
    pub(super) action: RetentionAction,
}

impl RetentionRule {
    fn matches(&self, tuple: &RetentionTuple) -> bool {
        self.memory_horizon == tuple.memory_horizon
            && self.trigger == tuple.trigger
            && self.record_class == tuple.record_class
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub(super) enum RetentionAction {
    RetainIndefinitely,
    RetentionWindow,
}

pub(super) fn load_redaction_retention_fixture(
    bytes: &[u8],
) -> Result<RedactionRetentionFixture, RedactionError> {
    let fixture: RedactionRetentionFixture = serde_json::from_slice(bytes)
        .map_err(|_| RedactionError::new(RedactionErrorKind::InvalidFixture))?;
    if !fixture.redaction.fail_closed || fixture.redaction.unmatched_action != RedactionAction::Omit
    {
        return Err(RedactionError::new(RedactionErrorKind::InvalidFixture));
    }

    let allowed = fixture
        .retention
        .allowed_tuples
        .iter()
        .cloned()
        .collect::<BTreeSet<_>>();
    if allowed.is_empty() || allowed.len() != fixture.retention.allowed_tuples.len() {
        return Err(RedactionError::new(RedactionErrorKind::InvalidFixture));
    }
    if fixture.retention.rules.iter().any(|rule| {
        rule.rule_id.is_empty()
            || !allowed.contains(&RetentionTuple {
                memory_horizon: rule.memory_horizon.clone(),
                trigger: rule.trigger.clone(),
                record_class: rule.record_class.clone(),
            })
    }) {
        return Err(RedactionError::new(RedactionErrorKind::InvalidFixture));
    }
    for tuple in &fixture.retention.allowed_tuples {
        resolve_retention(&fixture.retention.rules, tuple)?;
    }

    Ok(fixture)
}

pub(super) fn resolve_retention<'a>(
    rules: &'a [RetentionRule],
    tuple: &RetentionTuple,
) -> Result<&'a RetentionRule, RedactionError> {
    let mut matches = rules.iter().filter(|rule| rule.matches(tuple));
    let Some(rule) = matches.next() else {
        return Err(RedactionError::new(RedactionErrorKind::RetentionUncovered));
    };
    if matches.next().is_some() {
        return Err(RedactionError::new(RedactionErrorKind::RetentionOverlap));
    }
    Ok(rule)
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) struct DeletionGuard {
    pub(super) held: bool,
    pub(super) referenced: bool,
    pub(super) unexpired_floor: bool,
}

pub(super) fn approve_deletion(guard: DeletionGuard) -> Result<(), RedactionError> {
    if guard.held {
        Err(RedactionError::new(RedactionErrorKind::HeldRecord))
    } else if guard.referenced {
        Err(RedactionError::new(RedactionErrorKind::ReferencedRecord))
    } else if guard.unexpired_floor {
        Err(RedactionError::new(
            RedactionErrorKind::UnexpiredRetentionFloor,
        ))
    } else {
        Ok(())
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(super) struct ImmutableRecord {
    pub(super) record_id: String,
    pub(super) record_bytes: Vec<u8>,
    pub(super) payload_bytes: Vec<u8>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(super) struct PayloadBinding {
    pub(super) record_id: String,
    pub(super) record_bytes: Vec<u8>,
    pub(super) payload_ref: String,
}

pub(super) fn deduplicate_payloads(
    records: &[ImmutableRecord],
) -> Result<Vec<PayloadBinding>, RedactionError> {
    let mut records_by_id = BTreeMap::new();
    for record in records {
        if record.record_id.is_empty() {
            return Err(RedactionError::new(
                RedactionErrorKind::InvalidRecordIdentity,
            ));
        }
        if records_by_id
            .insert(record.record_id.clone(), record)
            .is_some()
        {
            return Err(RedactionError::new(
                RedactionErrorKind::DuplicateRecordIdentity,
            ));
        }
    }

    Ok(records_by_id
        .into_iter()
        .map(|(record_id, record)| PayloadBinding {
            record_id,
            record_bytes: record.record_bytes.clone(),
            payload_ref: format!(
                "content-addressed:{}",
                DefinitionFingerprint::from_bytes(&record.payload_bytes)
            ),
        })
        .collect())
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(super) struct Compaction {
    pub(super) source_record_ids: Vec<String>,
    pub(super) shared_payload_refs: Vec<String>,
}

pub(super) fn create_compaction(
    reviewed: bool,
    records: &[ImmutableRecord],
) -> Result<Compaction, RedactionError> {
    if !reviewed {
        return Err(RedactionError::new(
            RedactionErrorKind::CompactionReviewRequired,
        ));
    }

    let bindings = deduplicate_payloads(records)?;
    Ok(Compaction {
        source_record_ids: bindings
            .iter()
            .map(|binding| binding.record_id.clone())
            .collect(),
        shared_payload_refs: bindings
            .iter()
            .map(|binding| binding.payload_ref.clone())
            .collect::<BTreeSet<_>>()
            .into_iter()
            .collect(),
    })
}
