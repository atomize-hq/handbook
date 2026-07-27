use crate::approver_registry_observation::{
    observe_committed_approver_registry_locked, ApproverAuthorityPairV1,
};
use crate::charter_approval_workflow::observe_committed_candidate_approval_refs_locked;
use crate::charter_authority_transaction::{
    CharterAuthorityTransactionServiceV1, CharterPromotionCommitV1, CharterPromotionRequestV1,
    RetainedAuthorityRecordV1, RetainedPromotionAuthorityV1,
};
use crate::charter_lifecycle::{address_charter_lifecycle, CharterLifecycleState};
use crate::charter_lifecycle_store::{
    build_transition_record, charter_lifecycle_state_fingerprint,
    finalize_content_addressed_record, CharterLifecycleAuthorityV1, CharterLifecycleStoreErrorV1,
    CharterLifecycleStoreV1, CHARTER_LIFECYCLE_POLICY_REF,
};
use crate::charter_lifecycle_validation::{
    definition_bindings, CharterLifecycleValidationServiceV1, SELECTED_PROFILE_FINGERPRINT,
    SELECTED_PROFILE_REF,
};
use crate::charter_lineage_store::{
    string_field, LineageRecordClassV1, LineageStoreErrorV1, TrustedLineageStoreV1,
};
use crate::{
    load_shipped_charter_definition_registry, parse_canonical_charter,
    resolve_shipped_profile_decisions, serialize_canonical_charter, DefinitionFingerprint,
    ExactDefinitionRef,
};
use serde_json::{json, Value};
use std::collections::{BTreeMap, BTreeSet};
use std::fmt;
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

const CANONICAL_REF: &str = ".handbook/project/charter.yaml";
const TARGET_INSTANCE_ID: &str = "project_authority";
const SELECTED_KIND_REF: &str = "handbook.artifact-kind.project-authority@1.1.0";
const SELECTED_SCHEMA_REF: &str = "handbook.schemas.artifacts.project-authority@1.1.0";
const APPROVAL_POLICY_REF: &str = "handbook.approval.constitutional-candidate@1.0.0";
const MAX_APPROVAL_REFS: usize = 64;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CharterPromotionIntentV1 {
    pub candidate_ref: String,
    pub approval_ref: String,
    pub expected_current_fingerprint: Option<String>,
}

pub fn normalize_required_approval_pairs(
    mut pairs: Vec<ApproverAuthorityPairV1>,
) -> Result<Vec<ApproverAuthorityPairV1>, CharterPromotionWorkflowErrorV1> {
    if pairs.is_empty() || pairs.len() > MAX_APPROVAL_REFS {
        return Err(approval_refused(
            "required approval pair set must contain 1-64 entries",
        ));
    }
    for pair in &pairs {
        if pair.approval_class.trim().is_empty()
            || pair.authority_ref.trim().is_empty()
            || pair.approval_class.len() > 255
            || pair.authority_ref.len() > 255
        {
            return Err(approval_refused(
                "approval class and authority ref must be bounded non-empty strings",
            ));
        }
    }
    pairs.sort_by(|left, right| {
        (&left.approval_class, &left.authority_ref)
            .cmp(&(&right.approval_class, &right.authority_ref))
    });
    if pairs.windows(2).any(|window| window[0] == window[1]) {
        return Err(approval_refused(
            "required approval authority pairs must be unique",
        ));
    }
    Ok(pairs)
}

fn initial_quorum_pairs(
    registry_state: &Value,
) -> Result<Vec<ApproverAuthorityPairV1>, CharterPromotionWorkflowErrorV1> {
    let quorum = registry_state
        .get("initial_charter_quorum")
        .and_then(Value::as_array)
        .ok_or_else(|| registry_refused("registry initial Charter quorum is absent"))?;
    normalize_required_approval_pairs(
        quorum
            .iter()
            .map(|pair| {
                Ok(ApproverAuthorityPairV1 {
                    approval_class: string_field(pair, "approval_class")
                        .map_err(map_lineage_registry)?
                        .to_owned(),
                    authority_ref: string_field(pair, "authority_ref")
                        .map_err(map_lineage_registry)?
                        .to_owned(),
                })
            })
            .collect::<Result<Vec<_>, CharterPromotionWorkflowErrorV1>>()?,
    )
}

fn amendment_pairs(
    current: &crate::CanonicalCharter,
) -> Result<Vec<ApproverAuthorityPairV1>, CharterPromotionWorkflowErrorV1> {
    let pairs = current
        .governance
        .required_approvals
        .iter()
        .flat_map(|approval_class| {
            current
                .governance
                .decision_authority
                .iter()
                .map(move |authority_ref| ApproverAuthorityPairV1 {
                    approval_class: approval_class.clone(),
                    authority_ref: authority_ref.clone(),
                })
        })
        .collect();
    normalize_required_approval_pairs(pairs)
}

pub(crate) fn required_approval_pairs_for_authority(
    registry_state: &Value,
    current: Option<&crate::CanonicalCharter>,
) -> Result<Vec<ApproverAuthorityPairV1>, CharterPromotionWorkflowErrorV1> {
    match current {
        Some(current) => amendment_pairs(current),
        None => initial_quorum_pairs(registry_state),
    }
}

fn ensure_registry_pair_coverage(
    credentials: &[crate::CommittedApproverCredentialV1],
    required_pairs: &[ApproverAuthorityPairV1],
) -> Result<(), CharterPromotionWorkflowErrorV1> {
    for required in required_pairs {
        let covered = credentials.iter().any(|credential| {
            credential.active
                && credential.use_sequence < 4096
                && credential.approval_mappings.iter().any(|mapping| {
                    mapping.approval_class == required.approval_class
                        && mapping.authority_ref == required.authority_ref
                })
        });
        if !covered {
            return Err(registry_refused(
                "current approver registry does not cover every required authority pair",
            ));
        }
    }
    Ok(())
}

fn charter_for_current(
    current: &Option<crate::CommittedCharterAuthorityV1>,
    decisions: &crate::ResolvedProfileDecisions,
) -> Result<crate::CanonicalCharter, CharterPromotionWorkflowErrorV1> {
    let bytes = &current
        .as_ref()
        .ok_or_else(|| candidate_refused("current canonical Charter is absent"))?
        .canonical_bytes;
    parse_canonical_charter(decisions, bytes)
        .map_err(|_| candidate_refused("current canonical Charter is invalid"))
}

fn validate_candidate_contract(
    candidate: &Value,
    decisions: &crate::ResolvedProfileDecisions,
    definitions: &crate::CharterDefinitionRegistry,
) -> Result<(), CharterPromotionWorkflowErrorV1> {
    for (field, expected) in [
        ("target_instance_id", TARGET_INSTANCE_ID),
        ("target_kind_ref", SELECTED_KIND_REF),
        ("target_schema_ref", SELECTED_SCHEMA_REF),
        ("profile_ref", SELECTED_PROFILE_REF),
        ("required_approval_policy_ref", APPROVAL_POLICY_REF),
        ("promotion_eligibility", "requires_approval"),
    ] {
        if string_field(candidate, field).map_err(map_lineage_candidate)? != expected {
            return Err(candidate_refused(format!(
                "candidate `{field}` is outside the selected Charter contract"
            )));
        }
    }
    if string_field(candidate, "resolved_profile_fingerprint").map_err(map_lineage_candidate)?
        != SELECTED_PROFILE_FINGERPRINT
    {
        return Err(definition_drift(
            "candidate retained profile fingerprint is stale",
        ));
    }
    if !candidate
        .get("unresolved_coverage_ids")
        .and_then(Value::as_array)
        .is_some_and(Vec::is_empty)
    {
        return Err(candidate_refused(
            "candidate has unresolved required intake coverage",
        ));
    }
    if definitions
        .record(&ExactDefinitionRef::parse(APPROVAL_POLICY_REF).expect("fixed ref"))
        .is_none()
    {
        return Err(definition_drift(
            "candidate approval policy is absent from current closure",
        ));
    }
    let kind_ref = ExactDefinitionRef::parse(SELECTED_KIND_REF).expect("fixed ref");
    let kind = decisions
        .registry()
        .kind(&kind_ref)
        .ok_or_else(|| definition_drift("selected Charter kind is absent"))?;
    if kind.canonical_schema_ref().as_str() != SELECTED_SCHEMA_REF
        || kind.capabilities().is_empty()
        || !kind.capabilities().iter().any(|capability| {
            capability.contract_ref().as_str() == "handbook.capabilities.constitutional-root@1.0.0"
        })
    {
        return Err(definition_drift(
            "selected Charter kind/schema/capability contract drifted",
        ));
    }
    Ok(())
}

pub(crate) fn resolved_definition_bindings(
    candidate: &Value,
    decisions: &crate::ResolvedProfileDecisions,
    definitions: &crate::CharterDefinitionRegistry,
) -> Result<Vec<Value>, CharterPromotionWorkflowErrorV1> {
    let kind_ref = ExactDefinitionRef::parse(
        string_field(candidate, "target_kind_ref").map_err(map_lineage_candidate)?,
    )
    .map_err(|_| candidate_refused("candidate kind ref is invalid"))?;
    let kind = decisions
        .registry()
        .kind(&kind_ref)
        .ok_or_else(|| definition_drift("candidate kind is absent from selected registry"))?;
    let expected = definition_bindings();
    if !expected.iter().any(|binding| {
        binding.definition_ref == kind_ref.as_str()
            && binding.definition_fingerprint == kind.definition_fingerprint().as_str()
    }) {
        return Err(definition_drift(
            "selected target kind does not equal the frozen lifecycle-result closure",
        ));
    }
    for reference in definitions.refs() {
        let current = definitions
            .record(reference)
            .expect("registry refs remain resolvable");
        if !expected.iter().any(|binding| {
            binding.definition_ref == reference.as_str()
                && binding.definition_fingerprint == current.definition_fingerprint().as_str()
        }) {
            return Err(definition_drift(
                "selected shipped definition is absent or stale in the frozen lifecycle-result closure",
            ));
        }
    }
    Ok(expected
        .into_iter()
        .map(|binding| {
            json!({
                "definition_ref": binding.definition_ref,
                "definition_fingerprint": binding.definition_fingerprint,
            })
        })
        .collect())
}

fn validate_basis(
    candidate: &Value,
    requested: Option<&str>,
    current: Option<&str>,
) -> Result<(), CharterPromotionWorkflowErrorV1> {
    if let Some(fingerprint) = requested {
        DefinitionFingerprint::parse(fingerprint).map_err(|_| {
            workflow_error(
                CharterPromotionWorkflowErrorKindV1::InvalidIntent,
                "expected-current fingerprint is invalid",
            )
        })?;
    }
    if requested != current {
        return Err(workflow_error(
            CharterPromotionWorkflowErrorKindV1::BasisMismatch,
            "caller expected-current fingerprint does not match canonical authority",
        ));
    }
    let candidate_basis = candidate.get("basis_artifact_fingerprint");
    let expected_value = requested.map_or(Value::Null, |value| Value::String(value.to_owned()));
    if candidate_basis != Some(&expected_value) {
        return Err(workflow_error(
            CharterPromotionWorkflowErrorKindV1::BasisMismatch,
            "candidate basis does not match caller/current canonical authority",
        ));
    }
    Ok(())
}

fn validate_intent(
    intent: &CharterPromotionIntentV1,
) -> Result<(), CharterPromotionWorkflowErrorV1> {
    if intent.candidate_ref.is_empty()
        || intent.candidate_ref.len() > crate::MAX_LINEAGE_REFERENCE_BYTES
        || intent.approval_ref.is_empty()
        || intent.approval_ref.len() > crate::MAX_LINEAGE_REFERENCE_BYTES
    {
        return Err(workflow_error(
            CharterPromotionWorkflowErrorKindV1::InvalidIntent,
            "promotion candidate or approval anchor ref is invalid",
        ));
    }
    Ok(())
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

fn current_audit_utc() -> String {
    let seconds = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|duration| duration.as_secs() as i64)
        .unwrap_or_default();
    let days = seconds.div_euclid(86_400);
    let second_of_day = seconds.rem_euclid(86_400);
    let (year, month, day) = civil_date_from_unix_days(days);
    let hour = second_of_day / 3_600;
    let minute = (second_of_day % 3_600) / 60;
    let second = second_of_day % 60;
    format!("{year:04}-{month:02}-{day:02}T{hour:02}:{minute:02}:{second:02}Z")
}

fn civil_date_from_unix_days(days: i64) -> (i64, i64, i64) {
    let z = days + 719_468;
    let era = if z >= 0 { z } else { z - 146_096 } / 146_097;
    let day_of_era = z - era * 146_097;
    let year_of_era =
        (day_of_era - day_of_era / 1_460 + day_of_era / 36_524 - day_of_era / 146_096) / 365;
    let mut year = year_of_era + era * 400;
    let day_of_year = day_of_era - (365 * year_of_era + year_of_era / 4 - year_of_era / 100);
    let month_prime = (5 * day_of_year + 2) / 153;
    let day = day_of_year - (153 * month_prime + 2) / 5 + 1;
    let month = month_prime + if month_prime < 10 { 3 } else { -9 };
    year += i64::from(month <= 2);
    (year, month, day)
}

fn map_lifecycle(error: CharterLifecycleStoreErrorV1) -> CharterPromotionWorkflowErrorV1 {
    workflow_error(
        CharterPromotionWorkflowErrorKindV1::LifecycleClearanceIncomplete,
        error.detail(),
    )
}

fn map_transaction(error: crate::CharterPromotionErrorV1) -> CharterPromotionWorkflowErrorV1 {
    workflow_error(
        match error.kind() {
            crate::CharterPromotionErrorKindV1::BasisMismatch => {
                CharterPromotionWorkflowErrorKindV1::BasisMismatch
            }
            crate::CharterPromotionErrorKindV1::LineageViolation => {
                CharterPromotionWorkflowErrorKindV1::CandidateRefused
            }
            _ => CharterPromotionWorkflowErrorKindV1::TransactionRefused,
        },
        error.detail(),
    )
}

fn map_lineage_candidate(error: LineageStoreErrorV1) -> CharterPromotionWorkflowErrorV1 {
    candidate_refused(error.detail())
}

fn map_lineage_approval(error: LineageStoreErrorV1) -> CharterPromotionWorkflowErrorV1 {
    approval_refused(error.detail())
}

fn map_lineage_registry(error: LineageStoreErrorV1) -> CharterPromotionWorkflowErrorV1 {
    registry_refused(error.detail())
}

fn workflow_error(
    kind: CharterPromotionWorkflowErrorKindV1,
    detail: impl Into<String>,
) -> CharterPromotionWorkflowErrorV1 {
    CharterPromotionWorkflowErrorV1::new(kind, detail)
}

fn candidate_refused(detail: impl Into<String>) -> CharterPromotionWorkflowErrorV1 {
    workflow_error(
        CharterPromotionWorkflowErrorKindV1::CandidateRefused,
        detail,
    )
}

fn approval_refused(detail: impl Into<String>) -> CharterPromotionWorkflowErrorV1 {
    workflow_error(CharterPromotionWorkflowErrorKindV1::ApprovalRefused, detail)
}

fn registry_refused(detail: impl Into<String>) -> CharterPromotionWorkflowErrorV1 {
    workflow_error(CharterPromotionWorkflowErrorKindV1::RegistryRefused, detail)
}

fn definition_drift(detail: impl Into<String>) -> CharterPromotionWorkflowErrorV1 {
    workflow_error(CharterPromotionWorkflowErrorKindV1::DefinitionDrift, detail)
}

fn clearance_incomplete(detail: impl Into<String>) -> CharterPromotionWorkflowErrorV1 {
    workflow_error(
        CharterPromotionWorkflowErrorKindV1::LifecycleClearanceIncomplete,
        detail,
    )
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CharterPromotionWorkflowCommitV1 {
    pub canonical_fingerprint: String,
    pub promotion_ref: String,
    pub lifecycle_transition_ref: String,
    pub approval_refs: Vec<String>,
    pub required_approval_pairs: Vec<ApproverAuthorityPairV1>,
    pub transaction: CharterPromotionCommitV1,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CharterPromotionProductProjectionV1 {
    pub canonical_path: String,
    pub canonical_fingerprint: String,
    pub promotion_ref: String,
    pub promotion_fingerprint: String,
    pub changed_paths: Vec<String>,
    pub next_actions: Vec<String>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CharterPromotionProductProjectionErrorKindV1 {
    InvalidCanonicalFingerprint,
    InvalidPromotionRef,
    InvalidLifecycleTransitionRef,
    InconsistentCommit,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CharterPromotionProductProjectionErrorV1 {
    kind: CharterPromotionProductProjectionErrorKindV1,
    detail: String,
}

impl CharterPromotionProductProjectionErrorV1 {
    fn new(kind: CharterPromotionProductProjectionErrorKindV1, detail: impl Into<String>) -> Self {
        Self {
            kind,
            detail: detail.into(),
        }
    }

    pub fn kind(&self) -> CharterPromotionProductProjectionErrorKindV1 {
        self.kind
    }

    pub fn detail(&self) -> &str {
        &self.detail
    }

    pub fn product_refusal(&self) -> CharterPromotionProductRefusalV1 {
        let (code, next_action) = match self.kind {
            CharterPromotionProductProjectionErrorKindV1::InvalidCanonicalFingerprint => (
                "promotion_projection_invalid_canonical_fingerprint",
                "repair the retained canonical fingerprint before retrying product projection",
            ),
            CharterPromotionProductProjectionErrorKindV1::InvalidPromotionRef => (
                "promotion_projection_invalid_promotion_ref",
                "repair the retained content-addressed promotion ref before retrying product projection",
            ),
            CharterPromotionProductProjectionErrorKindV1::InvalidLifecycleTransitionRef => (
                "promotion_projection_invalid_lifecycle_transition_ref",
                "repair the retained content-addressed lifecycle transition ref before retrying product projection",
            ),
            CharterPromotionProductProjectionErrorKindV1::InconsistentCommit => (
                "promotion_projection_inconsistent_commit",
                "repair retained workflow and transaction ref agreement before retrying product projection",
            ),
        };
        CharterPromotionProductRefusalV1 {
            code: code.to_owned(),
            message: self.detail.clone(),
            retryable: false,
            next_actions: vec![next_action.to_owned()],
        }
    }
}

impl fmt::Display for CharterPromotionProductProjectionErrorV1 {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.detail)
    }
}

impl std::error::Error for CharterPromotionProductProjectionErrorV1 {}

impl CharterPromotionWorkflowCommitV1 {
    pub fn product_projection(
        &self,
    ) -> Result<CharterPromotionProductProjectionV1, CharterPromotionProductProjectionErrorV1> {
        if DefinitionFingerprint::parse(&self.canonical_fingerprint).is_err() {
            return Err(CharterPromotionProductProjectionErrorV1::new(
                CharterPromotionProductProjectionErrorKindV1::InvalidCanonicalFingerprint,
                "canonical fingerprint is not an exact lowercase SHA-256 fingerprint",
            ));
        }
        if self.promotion_ref != self.transaction.promotion_ref
            || self.lifecycle_transition_ref != self.transaction.lifecycle_transition_ref
        {
            return Err(CharterPromotionProductProjectionErrorV1::new(
                CharterPromotionProductProjectionErrorKindV1::InconsistentCommit,
                "workflow and retained transaction refs disagree",
            ));
        }
        let promotion_fingerprint =
            exact_product_ref_fingerprint(&self.promotion_ref, "promotions/", "promotion_")
                .ok_or_else(|| {
                    CharterPromotionProductProjectionErrorV1::new(
                        CharterPromotionProductProjectionErrorKindV1::InvalidPromotionRef,
                        "promotion ref is not an exact content-addressed promotion ref",
                    )
                })?;
        exact_product_ref_fingerprint(
            &self.lifecycle_transition_ref,
            "lifecycle-transitions/",
            "lifecycle-transition_",
        )
        .ok_or_else(|| {
            CharterPromotionProductProjectionErrorV1::new(
                CharterPromotionProductProjectionErrorKindV1::InvalidLifecycleTransitionRef,
                "lifecycle transition ref is not an exact content-addressed lifecycle transition ref",
            )
        })?;

        Ok(CharterPromotionProductProjectionV1 {
            canonical_path: CANONICAL_REF.to_owned(),
            canonical_fingerprint: self.canonical_fingerprint.clone(),
            promotion_ref: self.promotion_ref.clone(),
            promotion_fingerprint,
            changed_paths: vec![
                CANONICAL_REF.to_owned(),
                format!(".handbook/state/{}", self.promotion_ref),
                format!(".handbook/state/{}", self.lifecycle_transition_ref),
            ],
            next_actions: vec!["retain the committed promotion reference".to_owned()],
        })
    }
}

fn exact_product_ref_fingerprint(
    reference: &str,
    partition: &str,
    id_prefix: &str,
) -> Option<String> {
    let hex = reference
        .strip_prefix(partition)?
        .strip_suffix(".json")?
        .strip_prefix(id_prefix)?;
    if hex.len() == 64
        && hex
            .bytes()
            .all(|byte| byte.is_ascii_digit() || matches!(byte, b'a'..=b'f'))
    {
        Some(format!("sha256:{hex}"))
    } else {
        None
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CharterPromotionWorkflowErrorKindV1 {
    InvalidIntent,
    CandidateRefused,
    ApprovalRefused,
    RegistryRefused,
    DefinitionDrift,
    BasisMismatch,
    LifecycleClearanceIncomplete,
    TransactionRefused,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CharterPromotionProductRefusalV1 {
    pub code: String,
    pub message: String,
    pub retryable: bool,
    pub next_actions: Vec<String>,
}

impl CharterPromotionWorkflowErrorKindV1 {
    pub fn product_refusal(self, message: impl Into<String>) -> CharterPromotionProductRefusalV1 {
        let (code, retryable, next_action) = match self {
            Self::InvalidIntent => (
                "invalid_intent",
                false,
                "correct the bounded promotion intent and retry",
            ),
            Self::CandidateRefused => (
                "candidate_refused",
                false,
                "repair or rebuild the candidate against current selected definitions before retrying",
            ),
            Self::ApprovalRefused => (
                "approval_refused",
                false,
                "record every required current approval, then retry promotion",
            ),
            Self::RegistryRefused => (
                "registry_refused",
                false,
                "repair or bootstrap current approver registry authority before retrying",
            ),
            Self::DefinitionDrift => (
                "definition_drift",
                false,
                "repair the selected profile and Charter definition closure before retrying",
            ),
            Self::BasisMismatch => (
                "basis_mismatch",
                true,
                "re-finalize the candidate against current canonical Charter authority before retrying",
            ),
            Self::LifecycleClearanceIncomplete => (
                "lifecycle_clearance_incomplete",
                false,
                "reevaluate reopened Charter coverage and address every active lifecycle observation before retrying",
            ),
            Self::TransactionRefused => (
                "transaction_refused",
                false,
                "repair retained promotion transaction evidence before retrying",
            ),
        };
        CharterPromotionProductRefusalV1 {
            code: code.to_owned(),
            message: message.into(),
            retryable,
            next_actions: vec![next_action.to_owned()],
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CharterPromotionWorkflowErrorV1 {
    kind: CharterPromotionWorkflowErrorKindV1,
    detail: String,
}

impl CharterPromotionWorkflowErrorV1 {
    fn new(kind: CharterPromotionWorkflowErrorKindV1, detail: impl Into<String>) -> Self {
        Self {
            kind,
            detail: detail.into(),
        }
    }

    pub fn kind(&self) -> CharterPromotionWorkflowErrorKindV1 {
        self.kind
    }

    pub fn detail(&self) -> &str {
        &self.detail
    }

    pub fn product_refusal(&self) -> CharterPromotionProductRefusalV1 {
        self.kind.product_refusal(self.detail.clone())
    }
}

impl fmt::Display for CharterPromotionWorkflowErrorV1 {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.detail)
    }
}

impl std::error::Error for CharterPromotionWorkflowErrorV1 {}

#[derive(Clone, Debug)]
pub struct CharterPromotionWorkflowServiceV1 {
    repo_root: PathBuf,
    lineage: TrustedLineageStoreV1,
    transactions: CharterAuthorityTransactionServiceV1,
    lifecycle: CharterLifecycleStoreV1,
}

impl CharterPromotionWorkflowServiceV1 {
    pub fn new(repo_root: impl AsRef<Path>) -> Self {
        let repo_root = repo_root.as_ref().to_path_buf();
        Self {
            lineage: TrustedLineageStoreV1::new(&repo_root),
            transactions: CharterAuthorityTransactionServiceV1::new(&repo_root),
            lifecycle: CharterLifecycleStoreV1::new(&repo_root),
            repo_root,
        }
    }

    pub fn promote(
        &self,
        intent: CharterPromotionIntentV1,
    ) -> Result<CharterPromotionWorkflowCommitV1, CharterPromotionWorkflowErrorV1> {
        self.promote_at(intent, &current_audit_utc())
    }

    fn promote_at(
        &self,
        intent: CharterPromotionIntentV1,
        transitioned_at_utc: &str,
    ) -> Result<CharterPromotionWorkflowCommitV1, CharterPromotionWorkflowErrorV1> {
        validate_intent(&intent)?;
        let retained_transaction = self
            .transactions
            .begin_retained_authority()
            .map_err(map_transaction)?;
        let decisions = resolve_shipped_profile_decisions(&self.repo_root).map_err(|_| {
            workflow_error(
                CharterPromotionWorkflowErrorKindV1::DefinitionDrift,
                "selected shipped profile could not be re-resolved",
            )
        })?;
        let definitions = load_shipped_charter_definition_registry().map_err(|_| {
            definition_drift("shipped Charter definition closure could not be reloaded")
        })?;
        definitions
            .validate_selected_decisions(&decisions)
            .map_err(|_| {
                definition_drift("selected decisions do not retain the Charter closure")
            })?;

        let candidate_fingerprint = fingerprint_from_ref(&intent.candidate_ref)
            .ok_or_else(|| candidate_refused("candidate ref is not content-addressed"))?;
        let candidate = self
            .lineage
            .read_record_by_ref(
                LineageRecordClassV1::Candidate,
                &intent.candidate_ref,
                Some(&candidate_fingerprint),
            )
            .map_err(map_lineage_candidate)?;
        validate_candidate_contract(&candidate, &decisions, &definitions)?;
        let content_ref =
            string_field(&candidate, "normalized_content_ref").map_err(map_lineage_candidate)?;
        let canonical_bytes = self
            .lineage
            .read_candidate_content(content_ref)
            .map_err(map_lineage_candidate)?;
        let charter = parse_canonical_charter(&decisions, &canonical_bytes)
            .map_err(|_| candidate_refused("candidate content is not a valid canonical Charter"))?;
        if serialize_canonical_charter(&decisions, &charter)
            .map_err(|_| candidate_refused("candidate Charter serialization failed"))?
            != canonical_bytes
        {
            return Err(candidate_refused(
                "candidate content is not exact deterministic canonical YAML",
            ));
        }
        let canonical_fingerprint = DefinitionFingerprint::from_bytes(&canonical_bytes).to_string();
        let current = retained_transaction
            .read_committed_charter()
            .map_err(map_transaction)?;
        validate_basis(
            &candidate,
            intent.expected_current_fingerprint.as_deref(),
            current
                .as_ref()
                .map(|authority| authority.canonical_fingerprint.as_str()),
        )?;
        if current
            .as_ref()
            .is_some_and(|authority| authority.canonical_fingerprint == canonical_fingerprint)
        {
            return Err(candidate_refused(
                "an amendment must change the committed canonical Charter bytes",
            ));
        }

        let retained_lifecycle = self
            .lifecycle
            .observe_retained_locked()
            .map_err(map_lifecycle)?;
        if retained_lifecycle.canonical_bytes.as_deref()
            != current
                .as_ref()
                .map(|authority| authority.canonical_bytes.as_slice())
        {
            return Err(workflow_error(
                CharterPromotionWorkflowErrorKindV1::LifecycleClearanceIncomplete,
                "canonical basis and lifecycle retained bytes disagree",
            ));
        }
        let retained_validation = CharterLifecycleValidationServiceV1::new(&self.repo_root)
            .validate_current_retained(
                &decisions,
                &candidate,
                &canonical_bytes,
                current.as_ref(),
                &retained_lifecycle,
            )
            .map_err(|failure| candidate_refused(failure.detail()))?;

        let committed_approval_refs = observe_committed_candidate_approval_refs_locked(
            &self.repo_root,
            &intent.candidate_ref,
            &candidate_fingerprint,
            Some(&intent.approval_ref),
        )
        .map_err(|failure| approval_refused(failure.message))?;

        let registry =
            observe_committed_approver_registry_locked(&self.repo_root).map_err(|failure| {
                workflow_error(
                    CharterPromotionWorkflowErrorKindV1::RegistryRefused,
                    failure.detail(),
                )
            })?;
        let lifecycle = retained_lifecycle.authority.as_ref();
        let registry_state_ref = registry.state_ref.clone();
        let registry_state_fingerprint = registry.state_fingerprint.clone();
        let registry_head_ref = registry.head_transition_ref.clone();
        let registry_head_fingerprint = registry.head_transition_fingerprint.clone();
        let current_charter = current
            .as_ref()
            .map(|_| charter_for_current(&current, &decisions))
            .transpose()?;
        let required_pairs =
            required_approval_pairs_for_authority(&registry.state, current_charter.as_ref())?;
        ensure_registry_pair_coverage(&registry.credentials, &required_pairs)?;
        let ordered_approval_refs = self.validate_approvals(
            &intent.candidate_ref,
            &committed_approval_refs,
            &candidate,
            &candidate_fingerprint,
            &required_pairs,
            &registry_state_ref,
            &registry_state_fingerprint,
            &registry_head_ref,
            &registry_head_fingerprint,
        )?;
        let lifecycle_policy = definitions
            .record(&ExactDefinitionRef::parse(CHARTER_LIFECYCLE_POLICY_REF).expect("fixed ref"))
            .ok_or_else(|| definition_drift("lifecycle policy is absent"))?;
        let (prior_state, prior_state_fingerprint) = match (&current, lifecycle) {
            (None, None) => {
                let initial = charter_lifecycle_state_fingerprint(
                    CHARTER_LIFECYCLE_POLICY_REF,
                    lifecycle_policy.definition_fingerprint().as_str(),
                    TARGET_INSTANCE_ID,
                    &canonical_fingerprint,
                    CharterLifecycleState::Current,
                    &[],
                )
                .map_err(map_lifecycle)?;
                (CharterLifecycleState::Current, initial)
            }
            (Some(current), Some(lifecycle))
                if lifecycle.canonical_fingerprint == current.canonical_fingerprint =>
            {
                self.validate_lifecycle_clearance(
                    lifecycle,
                    &candidate,
                    &canonical_fingerprint,
                    &current.canonical_fingerprint,
                )?;
                (lifecycle.state, lifecycle.state_fingerprint.clone())
            }
            (None, Some(_)) | (Some(_), None) | (Some(_), Some(_)) => {
                return Err(workflow_error(
                    CharterPromotionWorkflowErrorKindV1::LifecycleClearanceIncomplete,
                    "canonical and lifecycle authority observations are inconsistent",
                ))
            }
        };
        let result_state_fingerprint = charter_lifecycle_state_fingerprint(
            CHARTER_LIFECYCLE_POLICY_REF,
            lifecycle_policy.definition_fingerprint().as_str(),
            TARGET_INSTANCE_ID,
            &canonical_fingerprint,
            CharterLifecycleState::Current,
            &[],
        )
        .map_err(map_lifecycle)?;
        let resolved_definitions =
            resolved_definition_bindings(&candidate, &decisions, &definitions)?;
        let validation_result_refs = vec![retained_validation.relative_ref.clone()];
        let authorized_by_ref = required_pairs
            .first()
            .expect("required pairs are non-empty")
            .authority_ref
            .clone();
        let mut promotion = json!({
            "schema_id": "handbook.artifact-promotion-record",
            "schema_version": "1.1",
            "candidate_ref": intent.candidate_ref,
            "candidate_fingerprint": candidate_fingerprint,
            "target_instance_id": TARGET_INSTANCE_ID,
            "basis_artifact_fingerprint": intent.expected_current_fingerprint,
            "expected_current_artifact_fingerprint": intent.expected_current_fingerprint,
            "profile_ref": SELECTED_PROFILE_REF,
            "resolved_profile_fingerprint": SELECTED_PROFILE_FINGERPRINT,
            "resolved_definitions": resolved_definitions,
            "approval_refs": ordered_approval_refs,
            "validation_result_refs": validation_result_refs,
            "decision": "approved",
            "authorized_by_ref": authorized_by_ref,
            "canonical_artifact_ref": CANONICAL_REF,
            "canonical_artifact_fingerprint": canonical_fingerprint,
            "approver_registry_state_ref": registry_state_ref,
            "approver_registry_state_fingerprint": registry_state_fingerprint,
            "registry_head_transition_ref": registry_head_ref,
            "registry_head_transition_fingerprint": registry_head_fingerprint,
        });
        promotion = finalize_content_addressed_record(
            promotion,
            "promotion_id",
            "promotion_fingerprint",
            "promotion",
            &[],
        )
        .map_err(map_lifecycle)?;
        let promotion_id =
            string_field(&promotion, "promotion_id").map_err(map_lineage_candidate)?;
        let promotion_ref = format!("promotions/{promotion_id}.json");
        let lifecycle_transition = build_transition_record(
            lifecycle_policy.definition_fingerprint().as_str(),
            prior_state,
            &prior_state_fingerprint,
            CharterLifecycleState::Current,
            &result_state_fingerprint,
            vec![],
            vec![],
            Some(promotion_ref.clone()),
            transitioned_at_utc,
        )
        .map_err(map_lifecycle)?;
        let promotion_bytes = serde_json_canonicalizer::to_vec(&promotion)
            .map_err(|_| candidate_refused("promotion record canonicalization failed"))?;
        let lifecycle_bytes = serde_json_canonicalizer::to_vec(&lifecycle_transition)
            .map_err(|_| candidate_refused("lifecycle transition canonicalization failed"))?;
        let mut retained_records = vec![
            RetainedAuthorityRecordV1 {
                label: "lifecycle validation result".to_owned(),
                relative_ref: retained_validation.relative_ref,
                bytes: retained_validation.bytes,
            },
            RetainedAuthorityRecordV1 {
                label: "registry state".to_owned(),
                relative_ref: registry_state_ref.clone(),
                bytes: registry.state_bytes.clone(),
            },
            RetainedAuthorityRecordV1 {
                label: "registry head".to_owned(),
                relative_ref: registry_head_ref.clone(),
                bytes: registry.head_transition_bytes.clone(),
            },
        ];
        for approval_ref in &ordered_approval_refs {
            let approval_fingerprint = fingerprint_from_ref(approval_ref)
                .ok_or_else(|| approval_refused("approval ref is not content-addressed"))?;
            retained_records.push(RetainedAuthorityRecordV1 {
                label: format!("approval {approval_ref}"),
                relative_ref: approval_ref.clone(),
                bytes: self
                    .lineage
                    .read_record(
                        LineageRecordClassV1::Approval,
                        approval_ref,
                        &approval_fingerprint,
                    )
                    .map_err(map_lineage_approval)?,
            });
        }
        retained_records.extend(retained_lifecycle.records.into_iter().map(|record| {
            RetainedAuthorityRecordV1 {
                label: format!("lifecycle authority {}", record.relative_ref),
                relative_ref: record.relative_ref,
                bytes: record.bytes,
            }
        }));
        let retained = RetainedPromotionAuthorityV1 {
            canonical_basis_bytes: current
                .as_ref()
                .map(|authority| authority.canonical_bytes.clone()),
            records: retained_records,
        };
        let transaction = retained_transaction
            .promote_retained(
                CharterPromotionRequestV1 {
                    canonical_bytes,
                    promotion_record_bytes: promotion_bytes,
                    lifecycle_transition_bytes: lifecycle_bytes,
                },
                &retained,
            )
            .map_err(map_transaction)?;
        Ok(CharterPromotionWorkflowCommitV1 {
            canonical_fingerprint,
            promotion_ref: promotion_ref.clone(),
            lifecycle_transition_ref: transaction.lifecycle_transition_ref.clone(),
            approval_refs: ordered_approval_refs,
            required_approval_pairs: required_pairs,
            transaction,
        })
    }

    #[allow(clippy::too_many_arguments)]
    fn validate_approvals(
        &self,
        candidate_ref: &str,
        approval_refs: &[String],
        candidate: &Value,
        candidate_fingerprint: &str,
        required_pairs: &[ApproverAuthorityPairV1],
        registry_state_ref: &str,
        registry_state_fingerprint: &str,
        registry_head_ref: &str,
        registry_head_fingerprint: &str,
    ) -> Result<Vec<String>, CharterPromotionWorkflowErrorV1> {
        if approval_refs.len() != required_pairs.len() {
            return Err(approval_refused(
                "promotion must provide exactly one approval per required authority pair",
            ));
        }
        let mut by_pair = BTreeMap::new();
        for approval_ref in approval_refs {
            let fingerprint = fingerprint_from_ref(approval_ref)
                .ok_or_else(|| approval_refused("approval ref is not content-addressed"))?;
            let approval = self
                .lineage
                .read_record_by_ref(
                    LineageRecordClassV1::Approval,
                    approval_ref,
                    Some(&fingerprint),
                )
                .map_err(map_lineage_approval)?;
            for (field, expected) in [
                ("candidate_ref", candidate_ref),
                ("candidate_fingerprint", candidate_fingerprint),
                ("decision", "approved"),
                ("approval_policy_ref", APPROVAL_POLICY_REF),
                ("approver_registry_state_ref", registry_state_ref),
                (
                    "approver_registry_state_fingerprint",
                    registry_state_fingerprint,
                ),
                ("registry_head_transition_ref", registry_head_ref),
                (
                    "registry_head_transition_fingerprint",
                    registry_head_fingerprint,
                ),
            ] {
                if string_field(&approval, field).map_err(map_lineage_approval)? != expected {
                    return Err(approval_refused(format!(
                        "approval `{field}` does not match retained promotion authority"
                    )));
                }
            }
            if approval.get("basis_artifact_fingerprint")
                != candidate.get("basis_artifact_fingerprint")
            {
                return Err(workflow_error(
                    CharterPromotionWorkflowErrorKindV1::BasisMismatch,
                    "approval basis differs from candidate basis",
                ));
            }
            let pair = (
                string_field(&approval, "approval_class")
                    .map_err(map_lineage_approval)?
                    .to_owned(),
                string_field(&approval, "authority_ref")
                    .map_err(map_lineage_approval)?
                    .to_owned(),
            );
            if by_pair.insert(pair, approval_ref.clone()).is_some() {
                return Err(approval_refused(
                    "multiple approvals cannot satisfy the same authority pair",
                ));
            }
        }
        required_pairs
            .iter()
            .map(|pair| {
                by_pair
                    .remove(&(pair.approval_class.clone(), pair.authority_ref.clone()))
                    .ok_or_else(|| {
                        approval_refused("required approval class/authority pair is not satisfied")
                    })
            })
            .collect()
    }

    fn validate_lifecycle_clearance(
        &self,
        lifecycle: &CharterLifecycleAuthorityV1,
        candidate: &Value,
        new_canonical_fingerprint: &str,
        current_canonical_fingerprint: &str,
    ) -> Result<(), CharterPromotionWorkflowErrorV1> {
        if lifecycle.active_observations.is_empty() {
            return Ok(());
        }
        if new_canonical_fingerprint == current_canonical_fingerprint {
            return Err(clearance_incomplete(
                "active lifecycle observations require changed approved Charter bytes",
            ));
        }
        let intake_ref =
            string_field(candidate, "intake_record_ref").map_err(map_lineage_candidate)?;
        let intake = self
            .lineage
            .read_record_by_ref(LineageRecordClassV1::Intake, intake_ref, None)
            .map_err(map_lineage_candidate)?;
        let coverage = intake
            .get("coverage_results")
            .and_then(Value::as_array)
            .ok_or_else(|| clearance_incomplete("candidate intake coverage is absent"))?;
        let completed = coverage
            .iter()
            .filter_map(|row| {
                let status = row.get("evaluation").and_then(Value::as_str)?;
                matches!(status, "satisfied" | "waived").then(|| {
                    row.get("coverage_id")
                        .and_then(Value::as_str)
                        .map(str::to_owned)
                })?
            })
            .collect::<BTreeSet<_>>();
        if lifecycle
            .reopened_coverage_ids
            .iter()
            .any(|coverage_id| !completed.contains(coverage_id))
        {
            return Err(clearance_incomplete(
                "candidate intake did not reevaluate every reopened lifecycle coverage ID",
            ));
        }
        let addressed = lifecycle
            .active_observations
            .iter()
            .map(|observation| observation.event_fingerprint.clone())
            .collect::<Vec<_>>();
        address_charter_lifecycle(
            &lifecycle.active_observations,
            &addressed,
            &lifecycle.reopened_coverage_ids,
        )
        .map_err(|failure| clearance_incomplete(failure.detail()))?;
        Ok(())
    }
}
