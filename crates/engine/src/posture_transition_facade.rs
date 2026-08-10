//! Direct typed Rust ingress for a single Charter posture transition.
//!
//! This module deliberately exposes only bounded semantic inputs and closed
//! outcomes. It does not expose private transition records, durable journal
//! bytes, a transport envelope, or an operation catalog entry.

use crate::charter_approval_workflow::observe_committed_candidate_approval_refs_locked;
use crate::charter_authority_transaction::{
    CommittedPostureTransitionReplayV1, PostureTransitionApplyDispositionV1,
};
use crate::charter_lifecycle_transition_v11::{
    construct_lifecycle_transition_v11, LifecycleTransitionDraftV11,
};
use crate::charter_lifecycle_validation::{
    validate_candidate_exact_result_authority, validate_result_bytes,
};
use crate::charter_lineage_store::{LineageRecordClassV1, TrustedLineageStoreV1};
use crate::charter_posture_transaction_intent_v1::{
    construct_posture_transaction_intent_with_id_v1, construct_posture_transition_v1,
    KernelReplayDimensionV1, KernelReplayV1, PairV1, PostureChangeV1, PostureReassessmentV1,
    PostureTransitionDraftV1,
};
use crate::project_posture::{
    bind_exact_canonical_bytes, derive_project_posture_kernel, fixed_dimension_binding,
    prepare_posture_change, PostureChangeRequest,
};
use crate::{
    parse_canonical_charter, resolve_shipped_profile_decisions, serialize_canonical_charter,
    CharterAuthorityTransactionServiceV1, CharterPromotionErrorKindV1, DefinitionFingerprint,
    REPOSITORY_IDENTITY_REPO_PATH,
};
use sha2::{Digest, Sha256};
use std::fs;
use std::path::{Path, PathBuf};

const CANONICAL_CHARTER_REF: &str = ".handbook/project/charter.yaml";
const SOURCE_KERNEL_REF: &str = "project-posture-kernels/source";
const RESULTING_KERNEL_REF: &str = "project-posture-kernels/result";
const MAX_REASSESSMENT_COVERAGE_IDS: usize = 64;
const MAX_REASSESSMENT_COVERAGE_ID_BYTES: usize = 256;

/// An exact semantic reference/fingerprint pair. This is Rust-only data, not a
/// Serde or transport DTO.
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct PostureReferenceV1 {
    pub reference: String,
    pub fingerprint: String,
}

/// The fixed-leaf compare-and-swap transition requested for a Charter posture.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PostureDimensionChangeRequestV1 {
    pub dimension_id: String,
    pub dimension_index: u8,
    pub authority_path: String,
    pub expected_stored_value: Option<u8>,
    pub expected_effective_level: u8,
    pub proposed_effective_level: u8,
}

/// All caller-supplied semantic inputs for the direct compile-time posture
/// ingress. The engine independently resolves the current authority head,
/// source kernel, approvals, policy, profile, and lifecycle-validation pair.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PostureTransitionApplyRequestV1 {
    /// An opaque bounded caller key. The engine hashes it before creating the
    /// private journal identity; the key itself is never persisted.
    pub idempotency_key: String,
    /// Exact identity of the repository whose current authority is being
    /// changed. The facade re-reads this persisted identity before composing
    /// private records, so an SDK handle cannot be redirected across repos.
    pub repository_identity_fingerprint: String,
    /// Exact canonical Charter bytes the caller observed when it formed this
    /// request. The facade rejects a request whose canonical binding is no
    /// longer current before it constructs private records.
    pub expected_canonical: PostureCanonicalBindingV1,
    /// Exact candidate recommendation retained through normal Charter author
    /// and approval workflows. Its content must be the requested result.
    pub recommendation: PostureReferenceV1,
    /// The explicit, ordered coverage IDs reassessed by this transition. The
    /// remaining reassessment authority is derived from the candidate's exact
    /// lifecycle-validation result.
    pub reassessment_coverage_ids: Vec<String>,
    pub change: PostureDimensionChangeRequestV1,
    pub effective_at_utc: String,
}

/// A non-durable public summary of a successfully applied or replayed call.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PostureTransitionReceiptV1 {
    pub idempotency_key_fingerprint: String,
    pub repository_identity_fingerprint: String,
    pub prior_canonical: PostureCanonicalBindingV1,
    pub resulting_canonical: PostureCanonicalBindingV1,
    pub posture_transition: PostureReferenceV1,
    pub lifecycle_transition: PostureReferenceV1,
    pub resulting_kernel: PostureReferenceV1,
}

/// Exact identity of a canonical Charter retained in a public receipt only.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PostureCanonicalBindingV1 {
    pub reference: String,
    pub fingerprint: String,
    pub document_sha256: String,
    pub byte_length: u64,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PostureTransitionBlockedCodeV1 {
    UnsupportedPlatform,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PostureTransitionRefusalCodeV1 {
    InvalidIdempotencyKey,
    RepositoryIdentityMismatch,
    InvalidFixedDimension,
    StaleCanonical,
    MissingLifecycleAuthority,
    LifecycleNotCurrent,
    InvalidSemanticInput,
    AuthorityConflict,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PostureTransitionErrorCodeV1 {
    UnsafeFilesystem,
    IntegrityOrDurability,
    Io,
}

/// Every public call is closed: a caller receives one typed disposition and
/// never private records, journal paths, raw validation text, or bytes.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum PostureTransitionApplyResultV1 {
    Applied(PostureTransitionReceiptV1),
    Replayed(PostureTransitionReceiptV1),
    Blocked(PostureTransitionBlockedCodeV1),
    Refused(PostureTransitionRefusalCodeV1),
    Error(PostureTransitionErrorCodeV1),
}

/// Purpose-named public engine facade for direct typed Rust linkage.
#[derive(Clone, Debug)]
pub struct PostureTransitionEngineFacadeV1 {
    repo_root: PathBuf,
}

impl PostureTransitionEngineFacadeV1 {
    /// Opens the facade against one repository. Opening is side-effect free;
    /// every mutation still performs the private owner checks under authority
    /// locks.
    pub fn open(repo_root: impl AsRef<Path>) -> Self {
        Self {
            repo_root: repo_root.as_ref().to_path_buf(),
        }
    }

    /// Applies one bounded posture transition through the private transaction
    /// owner. A repeat of the exact request/key returns `Replayed`; a changed
    /// request for that key is refused without a new write.
    pub fn apply_posture_transition(
        &self,
        request: PostureTransitionApplyRequestV1,
    ) -> PostureTransitionApplyResultV1 {
        let (transaction_id, idempotency_key_fingerprint) =
            match transaction_identity(&request.idempotency_key) {
                Some(identity) => identity,
                None => {
                    return PostureTransitionApplyResultV1::Refused(
                        PostureTransitionRefusalCodeV1::InvalidIdempotencyKey,
                    )
                }
            };

        let repository_identity_fingerprint = match read_repository_identity(&self.repo_root) {
            Some(fingerprint) if fingerprint == request.repository_identity_fingerprint => {
                fingerprint
            }
            _ => {
                return PostureTransitionApplyResultV1::Refused(
                    PostureTransitionRefusalCodeV1::RepositoryIdentityMismatch,
                )
            }
        };

        let transactions = CharterAuthorityTransactionServiceV1::new(&self.repo_root);
        match transactions.recover_and_load_committed_posture_transition(&transaction_id) {
            Ok(Some(replay)) => {
                return if request_matches_replay(&request, &replay) {
                    PostureTransitionApplyResultV1::Replayed(receipt_from_replay(
                        &replay,
                        idempotency_key_fingerprint,
                        repository_identity_fingerprint,
                    ))
                } else {
                    PostureTransitionApplyResultV1::Refused(
                        PostureTransitionRefusalCodeV1::AuthorityConflict,
                    )
                }
            }
            Ok(None) => {}
            Err(error) => return result_from_engine_error(error.kind()),
        }

        let authority_head = match transactions.observe_posture_authority_head() {
            Ok(Some(head)) => head,
            Ok(None) => {
                return PostureTransitionApplyResultV1::Refused(
                    PostureTransitionRefusalCodeV1::MissingLifecycleAuthority,
                )
            }
            Err(error) => return result_from_engine_error(error.kind()),
        };
        let old_canonical = match fs::read(self.repo_root.join(CANONICAL_CHARTER_REF)) {
            Ok(bytes) => bytes,
            Err(_) => {
                return PostureTransitionApplyResultV1::Error(PostureTransitionErrorCodeV1::Io)
            }
        };
        let observed_canonical = canonical_binding_from_bytes(&old_canonical);
        if canonical_from_public(&request.expected_canonical)
            != canonical_from_public(&observed_canonical)
        {
            return PostureTransitionApplyResultV1::Refused(
                PostureTransitionRefusalCodeV1::StaleCanonical,
            );
        }
        if authority_head.canonical != canonical_from_public(&observed_canonical) {
            return PostureTransitionApplyResultV1::Refused(
                PostureTransitionRefusalCodeV1::StaleCanonical,
            );
        }
        let old_exact = match bind_exact_canonical_bytes(&old_canonical) {
            Ok(value) => value,
            Err(_) => {
                return PostureTransitionApplyResultV1::Refused(
                    PostureTransitionRefusalCodeV1::InvalidSemanticInput,
                )
            }
        };

        let Some(binding) = fixed_dimension_binding(&request.change.dimension_id) else {
            return PostureTransitionApplyResultV1::Refused(
                PostureTransitionRefusalCodeV1::InvalidFixedDimension,
            );
        };
        if request.change.dimension_index != binding.index as u8
            || request.change.authority_path != binding.authority_path
            || !valid_reassessment_coverage_ids(&request.reassessment_coverage_ids)
        {
            return PostureTransitionApplyResultV1::Refused(
                PostureTransitionRefusalCodeV1::InvalidSemanticInput,
            );
        }

        let decisions = match resolve_shipped_profile_decisions(&self.repo_root) {
            Ok(decisions) => decisions,
            Err(_) => {
                return PostureTransitionApplyResultV1::Error(
                    PostureTransitionErrorCodeV1::IntegrityOrDurability,
                )
            }
        };
        let source = match parse_canonical_charter(&decisions, &old_canonical) {
            Ok(charter) => charter,
            Err(_) => {
                return PostureTransitionApplyResultV1::Error(
                    PostureTransitionErrorCodeV1::IntegrityOrDurability,
                )
            }
        };
        let change = match prepare_posture_change(
            &source,
            old_exact,
            PostureChangeRequest {
                dimension_id: request.change.dimension_id.clone(),
                dimension_index: request.change.dimension_index,
                authority_path: request.change.authority_path.clone(),
                expected_stored_value: request.change.expected_stored_value,
                expected_effective_level: request.change.expected_effective_level,
                proposed_effective_level: request.change.proposed_effective_level,
            },
        ) {
            Ok(change) => change,
            Err(_) => {
                return PostureTransitionApplyResultV1::Refused(
                    PostureTransitionRefusalCodeV1::InvalidSemanticInput,
                )
            }
        };
        let resulting_canonical =
            match serialize_canonical_charter(&decisions, &change.resulting_charter) {
                Ok(bytes) => bytes,
                Err(_) => {
                    return PostureTransitionApplyResultV1::Error(
                        PostureTransitionErrorCodeV1::IntegrityOrDurability,
                    )
                }
            };
        let source_kernel = match derive_project_posture_kernel(&source, &change.current_canonical)
        {
            Ok(kernel) => kernel,
            Err(_) => {
                return PostureTransitionApplyResultV1::Error(
                    PostureTransitionErrorCodeV1::IntegrityOrDurability,
                )
            }
        };
        let resulting_exact = match bind_exact_canonical_bytes(&resulting_canonical) {
            Ok(value) => value,
            Err(_) => {
                return PostureTransitionApplyResultV1::Error(
                    PostureTransitionErrorCodeV1::IntegrityOrDurability,
                )
            }
        };
        let resulting_kernel =
            match derive_project_posture_kernel(&change.resulting_charter, &resulting_exact) {
                Ok(kernel) => kernel,
                Err(_) => {
                    return PostureTransitionApplyResultV1::Error(
                        PostureTransitionErrorCodeV1::IntegrityOrDurability,
                    )
                }
            };

        let lineage = TrustedLineageStoreV1::new(&self.repo_root);
        let candidate = match lineage.read_record_by_ref(
            LineageRecordClassV1::Candidate,
            &request.recommendation.reference,
            Some(&request.recommendation.fingerprint),
        ) {
            Ok(candidate) => candidate,
            Err(_) => {
                return PostureTransitionApplyResultV1::Refused(
                    PostureTransitionRefusalCodeV1::AuthorityConflict,
                )
            }
        };
        let retained_validation =
            match validate_candidate_exact_result_authority(&self.repo_root, &candidate) {
                Ok(validation) => validation,
                Err(_) => {
                    return PostureTransitionApplyResultV1::Refused(
                        PostureTransitionRefusalCodeV1::AuthorityConflict,
                    )
                }
            };
        let validation = match validate_result_bytes(
            &retained_validation.bytes,
            Some(&retained_validation.relative_ref),
        ) {
            Ok(validation) => validation,
            Err(_) => {
                return PostureTransitionApplyResultV1::Refused(
                    PostureTransitionRefusalCodeV1::AuthorityConflict,
                )
            }
        };
        let Some(normalized_content_ref) = candidate
            .get("normalized_content_ref")
            .and_then(serde_json::Value::as_str)
        else {
            return PostureTransitionApplyResultV1::Refused(
                PostureTransitionRefusalCodeV1::AuthorityConflict,
            );
        };
        let candidate_result = match lineage.read_candidate_content(normalized_content_ref) {
            Ok(bytes) => bytes,
            Err(_) => {
                return PostureTransitionApplyResultV1::Refused(
                    PostureTransitionRefusalCodeV1::AuthorityConflict,
                )
            }
        };
        if candidate_result != resulting_canonical {
            return PostureTransitionApplyResultV1::Refused(
                PostureTransitionRefusalCodeV1::InvalidSemanticInput,
            );
        }

        let approval_refs = match observe_committed_candidate_approval_refs_locked(
            &self.repo_root,
            &request.recommendation.reference,
            &request.recommendation.fingerprint,
            None,
        ) {
            Ok(refs) => refs,
            Err(_) => {
                return PostureTransitionApplyResultV1::Refused(
                    PostureTransitionRefusalCodeV1::AuthorityConflict,
                )
            }
        };
        let approval_inputs = match approval_pairs_and_authorized_actor(&lineage, &approval_refs) {
            Some(value) => value,
            None => {
                return PostureTransitionApplyResultV1::Refused(
                    PostureTransitionRefusalCodeV1::AuthorityConflict,
                )
            }
        };
        let (approval_inputs, authorized_by_ref) = approval_inputs;

        let posture = match construct_posture_transition_v1(PostureTransitionDraftV1 {
            recommendation: pair_from_public(&request.recommendation),
            repository_identity: PairV1 {
                reference: REPOSITORY_IDENTITY_REPO_PATH.to_owned(),
                fingerprint: repository_identity_fingerprint.clone(),
            },
            source_kernel: PairV1 {
                reference: SOURCE_KERNEL_REF.to_owned(),
                fingerprint: source_kernel.kernel_fingerprint.clone(),
            },
            evaluation_policy: PairV1 {
                reference: validation.lifecycle_policy_ref,
                fingerprint: validation.lifecycle_policy_fingerprint,
            },
            prior_authority_head: authority_head,
            expected_canonical: canonical_from_public(&observed_canonical),
            change: PostureChangeV1 {
                dimension_id: request.change.dimension_id,
                dimension_index: request.change.dimension_index,
                authority_path: request.change.authority_path,
                operation: "replace".to_owned(),
                baseline_level: change.change.baseline_level,
                expected_stored_value: change.change.expected_stored_value,
                expected_effective_level: change.change.expected_effective_level,
                proposed_stored_value: change.change.proposed_stored_value,
                proposed_effective_level: change.change.proposed_effective_level,
            },
            approval_inputs,
            authorized_by_ref,
            reassessment: PostureReassessmentV1 {
                intake_definition: PairV1 {
                    reference: validation.intake_record_ref.clone(),
                    fingerprint: validation.intake_record_fingerprint.clone(),
                },
                affected_coverage_ids: request.reassessment_coverage_ids,
                validation_result_inputs: vec![PairV1 {
                    reference: retained_validation.relative_ref,
                    fingerprint: validation.validation_result_fingerprint.clone(),
                }],
            },
            kernel_replay: KernelReplayV1 {
                constitutional_artifact_ref: CANONICAL_CHARTER_REF.to_owned(),
                source_authority_fingerprint: observed_canonical.fingerprint.clone(),
                resulting_authority_fingerprint: resulting_exact.canonical_fingerprint.clone(),
                source_input_fingerprint: source_kernel.input_fingerprint.clone(),
                resulting_input_fingerprint: resulting_kernel.input_fingerprint.clone(),
                profile_input: PairV1 {
                    reference: validation.profile_ref,
                    fingerprint: validation.resolved_profile_fingerprint,
                },
                override_inputs: Vec::new(),
                condition_inputs: Vec::new(),
                contract_inputs: Vec::new(),
                evidence_inputs: Vec::new(),
                snapshot_inputs: Vec::new(),
                freshness_basis: None,
                dimensions: source_kernel
                    .dimensions
                    .iter()
                    .zip(&resulting_kernel.dimensions)
                    .map(|(source, resulting)| KernelReplayDimensionV1 {
                        dimension_id: source.dimension_id.clone(),
                        source_effective_level: source.effective_level,
                        resulting_effective_level: resulting.effective_level,
                        floor: 1,
                        red_line_refs: source.red_line_refs.clone(),
                        trigger_refs: source.trigger_refs.clone(),
                        allowed_shortcut_refs: source.allowed_shortcut_refs.clone(),
                        proof_obligation_refs: Vec::new(),
                    })
                    .collect(),
                applicable_scope_refs: Vec::new(),
                omitted_condition_refs: Vec::new(),
                unresolved_condition_refs: Vec::new(),
            },
            resulting_canonical: canonical_binding_from_bytes(&resulting_canonical).into_private(),
            resulting_kernel: PairV1 {
                reference: RESULTING_KERNEL_REF.to_owned(),
                fingerprint: resulting_kernel.kernel_fingerprint.clone(),
            },
            effective_at_utc: request.effective_at_utc,
        }) {
            Ok(posture) => posture,
            Err(_) => {
                return PostureTransitionApplyResultV1::Refused(
                    PostureTransitionRefusalCodeV1::InvalidSemanticInput,
                )
            }
        };
        let lifecycle_authority =
            match crate::CharterLifecycleStoreV1::new(&self.repo_root).observe() {
                Ok(Some(authority)) => authority,
                Ok(None) => {
                    return PostureTransitionApplyResultV1::Refused(
                        PostureTransitionRefusalCodeV1::MissingLifecycleAuthority,
                    )
                }
                Err(_) => {
                    return PostureTransitionApplyResultV1::Error(
                        PostureTransitionErrorCodeV1::IntegrityOrDurability,
                    )
                }
            };
        let lifecycle = match construct_lifecycle_transition_v11(
            &posture,
            LifecycleTransitionDraftV11 {
                prior_transition: posture
                    .record
                    .prior_authority_head
                    .lifecycle_transition
                    .clone(),
                prior_state_fingerprint: lifecycle_authority.state_fingerprint,
            },
        ) {
            Ok(lifecycle) => lifecycle,
            Err(_) => {
                return PostureTransitionApplyResultV1::Refused(
                    PostureTransitionRefusalCodeV1::InvalidSemanticInput,
                )
            }
        };
        let intent = match construct_posture_transaction_intent_with_id_v1(
            &posture,
            &lifecycle,
            &old_canonical,
            &resulting_canonical,
            transaction_id,
        ) {
            Ok(intent) => intent,
            Err(_) => {
                return PostureTransitionApplyResultV1::Refused(
                    PostureTransitionRefusalCodeV1::InvalidSemanticInput,
                )
            }
        };
        let receipt = receipt_from_records(
            &posture,
            &lifecycle,
            idempotency_key_fingerprint,
            repository_identity_fingerprint,
        );
        match transactions.apply_posture_transition_idempotently(&posture, &lifecycle, &intent) {
            Ok(PostureTransitionApplyDispositionV1::Applied) => {
                PostureTransitionApplyResultV1::Applied(receipt)
            }
            Ok(PostureTransitionApplyDispositionV1::Replayed) => {
                PostureTransitionApplyResultV1::Replayed(receipt)
            }
            Err(error) => result_from_engine_error(error.kind()),
        }
    }
}

impl PostureCanonicalBindingV1 {
    fn into_private(self) -> crate::charter_posture_transaction_intent_v1::CanonicalDocumentV1 {
        crate::charter_posture_transaction_intent_v1::CanonicalDocumentV1 {
            reference: self.reference,
            fingerprint: self.fingerprint,
            document_sha256: self.document_sha256,
            byte_length: self.byte_length,
        }
    }
}

fn transaction_identity(key: &str) -> Option<(String, String)> {
    if key.is_empty() || key.len() > 256 || !key.bytes().all(|byte| byte.is_ascii_graphic()) {
        return None;
    }
    let digest = format!("{:x}", Sha256::digest(key.as_bytes()));
    Some((
        format!("posture-transaction_{}", &digest[..32]),
        format!("sha256:{digest}"),
    ))
}

fn read_repository_identity(repo_root: &Path) -> Option<String> {
    let raw = fs::read_to_string(repo_root.join(REPOSITORY_IDENTITY_REPO_PATH)).ok()?;
    if raw.is_empty() || raw.contains('\n') || DefinitionFingerprint::parse(&raw).is_err() {
        return None;
    }
    Some(raw)
}

fn valid_reassessment_coverage_ids(ids: &[String]) -> bool {
    !ids.is_empty()
        && ids.len() <= MAX_REASSESSMENT_COVERAGE_IDS
        && ids.iter().all(|id| {
            !id.is_empty()
                && id.len() <= MAX_REASSESSMENT_COVERAGE_ID_BYTES
                && id.bytes().all(|byte| byte.is_ascii_graphic())
        })
        && ids.windows(2).all(|pair| pair[0] < pair[1])
}

fn approval_pairs_and_authorized_actor(
    lineage: &TrustedLineageStoreV1,
    approval_refs: &[String],
) -> Option<(Vec<PairV1>, String)> {
    if approval_refs.is_empty() {
        return None;
    }
    let mut pairs = Vec::with_capacity(approval_refs.len());
    let mut actors = Vec::with_capacity(approval_refs.len());
    for reference in approval_refs {
        let fingerprint = fingerprint_from_content_ref(reference, "approvals/approval_", ".json")?;
        let approval = lineage
            .read_record_by_ref(
                LineageRecordClassV1::Approval,
                reference,
                Some(&fingerprint),
            )
            .ok()?;
        let actor = approval
            .get("authority_ref")
            .and_then(serde_json::Value::as_str)?;
        if actor.is_empty() || actor.len() > MAX_REASSESSMENT_COVERAGE_ID_BYTES {
            return None;
        }
        pairs.push(PairV1 {
            reference: reference.clone(),
            fingerprint,
        });
        actors.push(actor.to_owned());
    }
    actors.sort();
    Some((pairs, actors.remove(0)))
}

fn fingerprint_from_content_ref(reference: &str, prefix: &str, suffix: &str) -> Option<String> {
    let hex = reference.strip_prefix(prefix)?.strip_suffix(suffix)?;
    if hex.len() != 64
        || !hex
            .bytes()
            .all(|byte| byte.is_ascii_digit() || matches!(byte, b'a'..=b'f'))
    {
        return None;
    }
    Some(format!("sha256:{hex}"))
}

fn request_matches_replay(
    request: &PostureTransitionApplyRequestV1,
    replay: &CommittedPostureTransitionReplayV1,
) -> bool {
    let posture = &replay.posture.record;
    let change = &posture.change;
    posture.recommendation == pair_from_public(&request.recommendation)
        && posture.repository_identity
            == PairV1 {
                reference: REPOSITORY_IDENTITY_REPO_PATH.to_owned(),
                fingerprint: request.repository_identity_fingerprint.clone(),
            }
        && posture.expected_canonical == canonical_from_public(&request.expected_canonical)
        && posture.reassessment.affected_coverage_ids == request.reassessment_coverage_ids
        && change.dimension_id == request.change.dimension_id
        && change.dimension_index == request.change.dimension_index
        && change.authority_path == request.change.authority_path
        && change.expected_stored_value == request.change.expected_stored_value
        && change.expected_effective_level == request.change.expected_effective_level
        && change.proposed_effective_level == request.change.proposed_effective_level
        && posture.effective_at_utc == request.effective_at_utc
        && replay
            .intent
            .record
            .transaction_id
            .starts_with("posture-transaction_")
}

fn receipt_from_replay(
    replay: &CommittedPostureTransitionReplayV1,
    idempotency_key_fingerprint: String,
    repository_identity_fingerprint: String,
) -> PostureTransitionReceiptV1 {
    receipt_from_records(
        &replay.posture,
        &replay.lifecycle,
        idempotency_key_fingerprint,
        repository_identity_fingerprint,
    )
}

fn receipt_from_records(
    posture: &crate::charter_posture_transaction_intent_v1::ValidatedPostureTransitionV1,
    lifecycle: &crate::charter_lifecycle_transition_v11::ValidatedLifecycleTransitionV11,
    idempotency_key_fingerprint: String,
    repository_identity_fingerprint: String,
) -> PostureTransitionReceiptV1 {
    PostureTransitionReceiptV1 {
        idempotency_key_fingerprint,
        repository_identity_fingerprint,
        prior_canonical: canonical_binding_from_private(&posture.record.expected_canonical),
        resulting_canonical: canonical_binding_from_private(&posture.record.resulting_canonical),
        posture_transition: PostureReferenceV1 {
            reference: format!("posture-transitions/{}.json", posture.record.transition_id),
            fingerprint: posture.record.transition_fingerprint.clone(),
        },
        lifecycle_transition: PostureReferenceV1 {
            reference: format!(
                "lifecycle-transitions/{}.json",
                lifecycle.record.transition_id
            ),
            fingerprint: lifecycle.record.transition_fingerprint.clone(),
        },
        resulting_kernel: PostureReferenceV1 {
            reference: posture.record.resulting_kernel.reference.clone(),
            fingerprint: posture.record.resulting_kernel.fingerprint.clone(),
        },
    }
}

fn canonical_binding_from_bytes(bytes: &[u8]) -> PostureCanonicalBindingV1 {
    let fingerprint = DefinitionFingerprint::from_bytes(bytes).to_string();
    PostureCanonicalBindingV1 {
        reference: CANONICAL_CHARTER_REF.to_owned(),
        fingerprint: fingerprint.clone(),
        document_sha256: fingerprint,
        byte_length: bytes.len() as u64,
    }
}

fn canonical_binding_from_private(
    value: &crate::charter_posture_transaction_intent_v1::CanonicalDocumentV1,
) -> PostureCanonicalBindingV1 {
    PostureCanonicalBindingV1 {
        reference: value.reference.clone(),
        fingerprint: value.fingerprint.clone(),
        document_sha256: value.document_sha256.clone(),
        byte_length: value.byte_length,
    }
}

fn pair_from_public(value: &PostureReferenceV1) -> PairV1 {
    PairV1 {
        reference: value.reference.clone(),
        fingerprint: value.fingerprint.clone(),
    }
}

fn canonical_from_public(
    value: &PostureCanonicalBindingV1,
) -> crate::charter_posture_transaction_intent_v1::CanonicalDocumentV1 {
    value.clone().into_private()
}

fn result_from_engine_error(kind: CharterPromotionErrorKindV1) -> PostureTransitionApplyResultV1 {
    match kind {
        CharterPromotionErrorKindV1::UnsupportedPlatform => {
            PostureTransitionApplyResultV1::Blocked(
                PostureTransitionBlockedCodeV1::UnsupportedPlatform,
            )
        }
        CharterPromotionErrorKindV1::InvalidRequest
        | CharterPromotionErrorKindV1::BasisMismatch
        | CharterPromotionErrorKindV1::LineageViolation
        | CharterPromotionErrorKindV1::Conflict => PostureTransitionApplyResultV1::Refused(
            PostureTransitionRefusalCodeV1::AuthorityConflict,
        ),
        CharterPromotionErrorKindV1::UnsafeFilesystem => {
            PostureTransitionApplyResultV1::Error(PostureTransitionErrorCodeV1::UnsafeFilesystem)
        }
        CharterPromotionErrorKindV1::DurabilityViolation => PostureTransitionApplyResultV1::Error(
            PostureTransitionErrorCodeV1::IntegrityOrDurability,
        ),
        CharterPromotionErrorKindV1::IoFailure => {
            PostureTransitionApplyResultV1::Error(PostureTransitionErrorCodeV1::Io)
        }
        #[cfg(test)]
        CharterPromotionErrorKindV1::InjectedFault => PostureTransitionApplyResultV1::Error(
            PostureTransitionErrorCodeV1::IntegrityOrDurability,
        ),
    }
}
