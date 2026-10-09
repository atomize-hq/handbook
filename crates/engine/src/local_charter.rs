//! Native initial local-advisory Charter authority. Portable locators are
//! observations and recovery inputs; only retained owner records establish truth.

use crate::charter_authority_transaction::CommittedCharterAuthorityV1;
use crate::charter_intake::evaluate_charter_intake;
use crate::charter_lifecycle_store::{
    CharterLifecycleAuthorityV1, RetainedLifecycleAuthorityV1, RetainedLifecycleRecordV1,
};
use crate::definition_identity::canonical_json_bytes;
use crate::project_posture::{bind_exact_canonical_bytes, derive_project_posture_kernel};
use crate::repository_durability::{
    RepositoryExclusiveLockV1, RepositoryReadV1, RetainedRepositoryRootV1,
};
use crate::repository_initialization::{
    RepositoryInitializationClaimV1, RepositoryInitializationDispositionV1,
    RepositoryInitializationGuardV1, REPOSITORY_INITIALIZATION_CLAIM_PATH_V1,
    REPOSITORY_INITIALIZATION_LOCK_PATH_V1, REPOSITORY_INITIALIZATION_RECEIPT_PATH_V1,
};
use crate::{
    parse_definition_yaml, parse_schema_json, CharterIntakeEnvelope, CharterLifecycleState,
    DefinitionFingerprint, REPOSITORY_IDENTITY_REPO_PATH,
};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::fmt;
use std::path::Path;
use std::sync::OnceLock;

#[cfg(test)]
#[path = "local_charter/fixed_profile_tests.rs"]
mod fixed_profile_tests;

pub const LOCAL_CHARTER_ROOT_V1: &str = ".handbook/local-charter/v1";
const INTENT: &str = ".handbook/local-charter/v1/initial-intent.json";
const HEAD: &str = ".handbook/local-charter/v1/current.json";
const COMMITTED: &str = ".handbook/local-charter/v1/committed.json";
const PENDING: &str = ".handbook/local-charter/v1/pending.json";
const CANONICAL: &str = ".handbook/project/charter.yaml";
const DEFERRAL_OWNER: &str = "handbook-contracts.admission-deferral@2.0.0";
const MAX_RECORD: usize = 1_048_576;
const PROFILE_REF: &str = "handbook.profile.native-local-charter@1.0.0";
const APPROVAL_REF: &str = "handbook.approval.native-local-initial-charter@1.0.0";
const INTAKE_REF: &str = "handbook.intake.native-local-initial-charter@1.0.0";
const LIFECYCLE_REF: &str = "handbook.lifecycle.native-local-initial-charter@1.0.0";
const LOCKS: [&str; 4] = [
    "author-project_authority.lock",
    "promotion.lock",
    "registry.lock",
    "lifecycle.lock",
];

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct LocalProjectBindingV1 {
    pub initialization_claim: RepositoryInitializationClaimV1,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct LocalCharterHumanApprovalV1 {
    pub accepted: bool,
    pub principal_claim: String,
    pub agent_claim: String,
    pub delegation_claim: String,
    pub source_ref: String,
    pub approved_content_fingerprint: String,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct NativeLocalCharterRequestV1 {
    pub request_key: String,
    pub intake: CharterIntakeEnvelope,
    pub human_approval: LocalCharterHumanApprovalV1,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct LocalCharterProfileBindingV1 {
    pub profile_ref: String,
    pub profile_fingerprint: String,
    pub closure_fingerprint: String,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct LocalCharterLocatorV1 {
    pub binding: LocalProjectBindingV1,
    pub request: NativeLocalCharterRequestV1,
    pub canonical_fingerprint: String,
    pub request_fingerprint: String,
    pub profile_binding: LocalCharterProfileBindingV1,
}

impl LocalCharterLocatorV1 {
    pub fn canonical_json_bytes(&self) -> Result<Vec<u8>, LocalAuthorityRefusalV1> {
        encode(self)
    }

    pub fn from_json_bytes(bytes: &[u8]) -> Result<Self, LocalAuthorityRefusalV1> {
        decode(bytes)
    }
}

#[derive(Debug)]
pub struct PreparedLocalCharterV1 {
    locator: LocalCharterLocatorV1,
}

impl PreparedLocalCharterV1 {
    pub fn locator(&self) -> &LocalCharterLocatorV1 {
        &self.locator
    }
    pub fn canonical_fingerprint(&self) -> &str {
        &self.locator.canonical_fingerprint
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum LocalCharterDispositionV1 {
    Committed,
    Replayed,
    Refused,
    NonCommitted,
    Uncertain,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct LocalCharterOutcomeV1 {
    pub locator: LocalCharterLocatorV1,
    pub canonical_fingerprint: String,
    pub profile_binding: LocalCharterProfileBindingV1,
    pub promotion_ref: String,
    pub lifecycle_transition_ref: String,
    pub human_approval_accepted: bool,
    pub advisory_permission_accepted: bool,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct LocalAuthorityRefusalV1 {
    pub code: String,
    pub detail: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unresolved_request: Option<Box<crate::local_authority::LocalUnresolvedRequestV1>>,
}

impl fmt::Display for LocalAuthorityRefusalV1 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.code, self.detail)
    }
}
impl std::error::Error for LocalAuthorityRefusalV1 {}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct LocalCharterExecutionV1 {
    pub disposition: LocalCharterDispositionV1,
    pub outcome: Option<LocalCharterOutcomeV1>,
    pub refusal: Option<LocalAuthorityRefusalV1>,
}

pub struct NativeLocalCharterServiceV1;

impl NativeLocalCharterServiceV1 {
    /// Pure prospective computation. Registration is revalidated at execution.
    pub fn prepare(
        repo: impl AsRef<Path>,
        binding: &LocalProjectBindingV1,
        request: NativeLocalCharterRequestV1,
    ) -> Result<PreparedLocalCharterV1, LocalAuthorityRefusalV1> {
        let repo = repo.as_ref();
        let root = RetainedRepositoryRootV1::open_exact(repo).map_err(filesystem)?;
        validate_prospective_binding(&root, binding)?;
        let (canonical, profile, _) = evaluate(repo, &request)?;
        let request_fingerprint = fingerprint(
            &json!({"schema_id":"handbook.native-local-charter-request","schema_version":"1.0","binding":binding,"request":request,"profile_binding":profile}),
        )?;
        let locator = LocalCharterLocatorV1 {
            binding: binding.clone(),
            request,
            canonical_fingerprint: DefinitionFingerprint::from_bytes(&canonical).to_string(),
            request_fingerprint,
            profile_binding: profile,
        };
        encode(&locator)?;
        Ok(PreparedLocalCharterV1 { locator })
    }

    pub fn establish(
        repo: impl AsRef<Path>,
        prepared: PreparedLocalCharterV1,
    ) -> LocalCharterExecutionV1 {
        execute(repo.as_ref(), &prepared.locator, true)
    }

    /// Reads retained history. This never writes or reactivates current state.
    pub fn resolve_request(
        repo: impl AsRef<Path>,
        locator: &LocalCharterLocatorV1,
    ) -> LocalCharterExecutionV1 {
        let mut pending = false;
        let result = (|| {
            let root = RetainedRepositoryRootV1::open_exact(repo).map_err(filesystem)?;
            validate_binding_retained(&root, &locator.binding)?;
            let expected = material(&root, locator)?;
            let intent = encode(&LocalIntentV1 {
                schema_id: "handbook.native-local-charter-intent".into(),
                schema_version: "1.0".into(),
                locator: locator.clone(),
            })?;
            require_bytes(&root, INTENT, &intent)?;
            if read(&root, COMMITTED)?.is_none() {
                require_bytes(&root, PENDING, &expected.marker)?;
                pending = true;
                return Err(refuse("original_local_charter_request_unresolved", "the exact native original request remains pending; only its retained recovery may resolve it"));
            }
            resolve_history(&root, locator)
        })();
        let mut observed = execution(result, LocalCharterDispositionV1::Replayed);
        if pending {
            observed.disposition = LocalCharterDispositionV1::Uncertain;
        }
        observed
    }

    /// A portable locator alone cannot supply the retained original intent.
    pub fn resume_request(
        repo: impl AsRef<Path>,
        locator: &LocalCharterLocatorV1,
    ) -> LocalCharterExecutionV1 {
        execute(repo.as_ref(), locator, false)
    }

    pub fn resolve_current(
        repo: impl AsRef<Path>,
        locator: &LocalCharterLocatorV1,
    ) -> Result<LocalCharterOutcomeV1, LocalAuthorityRefusalV1> {
        let repo = repo.as_ref();
        let root = RetainedRepositoryRootV1::open_exact(repo).map_err(filesystem)?;
        require_existing_locks(&root)?;
        let guard = RepositoryInitializationGuardV1::acquire(repo).map_err(filesystem)?;
        validate_binding_guard(&guard, &locator.binding)?;
        let locks = acquire_charter_locks(guard.root(), false)?;
        let result = resolve_current_retained(guard.root(), locator);
        validate_locks(guard.root(), &locks)?;
        guard.validate().map_err(filesystem)?;
        result
    }
}

#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
struct LocalIntentV1 {
    schema_id: String,
    schema_version: String,
    locator: LocalCharterLocatorV1,
}

struct LocalMaterial {
    canonical: Vec<u8>,
    records: Vec<(String, Vec<u8>)>,
    head: Vec<u8>,
    marker: Vec<u8>,
    outcome: LocalCharterOutcomeV1,
}

fn evaluate(
    _repo: &Path,
    request: &NativeLocalCharterRequestV1,
) -> Result<(Vec<u8>, LocalCharterProfileBindingV1, Value), LocalAuthorityRefusalV1> {
    let approval = &request.human_approval;
    if !approval.accepted
        || !bounded_text(&request.request_key, 128)
        || [
            &approval.principal_claim,
            &approval.agent_claim,
            &approval.delegation_claim,
            &approval.source_ref,
        ]
        .into_iter()
        .any(|value| !bounded_text(value, 4096))
    {
        return Err(refuse("explicit_initial_approval_missing", "exact initial human approval, principal, agent, delegation and source claims are required"));
    }
    if request.intake.expected_current_fingerprint.is_some()
        || request
            .intake
            .coverage
            .iter()
            .any(|coverage| coverage.waiver_ref.is_some())
    {
        return Err(refuse("unsupported_local_charter_request", "this local rule supports fresh initial Charter creation without waivers; amendment is unimplemented"));
    }
    let decisions = fixed_charter_decisions()?;
    let profile = local_profile(decisions)?;
    // Reuse shipped coverage/schema/posture computation, but publish no old
    // candidate or approval identity: local records select their own rule.
    let evaluated = evaluate_charter_intake(decisions, request.intake.clone(), None)
        .map_err(|e| refuse("charter_intake_refused", e.detail()))?;
    let canonical = evaluated.normalized_content;
    let fp = DefinitionFingerprint::from_bytes(&canonical).to_string();
    if approval.approved_content_fingerprint != fp {
        return Err(refuse(
            "approved_content_mismatch",
            "human approval does not name the exact canonical Charter bytes",
        ));
    }
    let bound = bind_exact_canonical_bytes(&canonical)
        .map_err(|_| refuse("invalid_posture", "canonical byte binding refused"))?;
    let posture = derive_project_posture_kernel(&evaluated.charter, &bound)
        .map_err(|_| refuse("invalid_posture", "the exact native posture rubric refused"))?;
    let validation = json!({"coverage_results":evaluated.intake.coverage_results,"field_sources":evaluated.candidate_subject.field_sources,"posture_kernel_fingerprint":posture.kernel_fingerprint,"posture_replay_plan":posture.replay_plan});
    Ok((canonical, profile, validation))
}

fn fixed_charter_decisions(
) -> Result<&'static crate::ResolvedProfileDecisions, LocalAuthorityRefusalV1> {
    static DECISIONS: OnceLock<Result<crate::ResolvedProfileDecisions, LocalAuthorityRefusalV1>> =
        OnceLock::new();
    DECISIONS
        .get_or_init(resolve_fixed_charter_decisions)
        .as_ref()
        .map_err(Clone::clone)
}

fn resolve_fixed_charter_decisions(
) -> Result<crate::ResolvedProfileDecisions, LocalAuthorityRefusalV1> {
    let result = (|| {
        let request = crate::profile_decision::local_operating_profile_request();
        crate::local_authority::require_fixed_builtin_profile_request(&request).map_err(|_| ())?;
        let profile =
            crate::resolve_profile_selection(Path::new(env!("CARGO_MANIFEST_DIR")), request)
                .map_err(|_| ())?;
        crate::local_authority::require_fixed_builtin_profile(&profile).map_err(|_| ())?;
        let decisions = crate::ResolvedProfileDecisions::from_profile(&profile).map_err(|_| ())?;
        crate::load_shipped_charter_definition_registry()
            .and_then(|registry| registry.validate_selected_decisions(&decisions))
            .map_err(|_| ())?;
        Ok::<_, ()>(decisions)
    })();
    result.map_err(|_| {
        refuse(
            "incompatible_definition_closure",
            "the exact compatible shipped base cannot be resolved",
        )
    })
}

fn local_profile(
    decisions: &crate::ResolvedProfileDecisions,
) -> Result<LocalCharterProfileBindingV1, LocalAuthorityRefusalV1> {
    if decisions.profile_ref().as_str() != "handbook.profile.shipped-root@1.2.0"
        || decisions.profile_definition_fingerprint().as_str()
            != "sha256:40c5fdb8a6ea42cf0f5f2c5cac8306ec7ad3a238c341653947f85abc93d72c40"
    {
        return Err(refuse(
            "incompatible_definition_closure",
            "local Charter requires the exact shipped 1.2 base",
        ));
    }
    let registry = crate::load_shipped_charter_definition_registry().map_err(|_| {
        refuse(
            "incompatible_definition_closure",
            "base Charter definition registry refused",
        )
    })?;
    registry
        .validate_selected_decisions(decisions)
        .map_err(|_| {
            refuse(
                "incompatible_definition_closure",
                "base Charter definition closure refused",
            )
        })?;
    let definitions = [
        (
            PROFILE_REF,
            include_bytes!("../definitions/local-charter/profile-v1.yaml").as_slice(),
        ),
        (
            APPROVAL_REF,
            include_bytes!("../definitions/local-charter/approval-v1.yaml").as_slice(),
        ),
        (
            INTAKE_REF,
            include_bytes!("../definitions/local-charter/intake-v1.yaml").as_slice(),
        ),
        (
            LIFECYCLE_REF,
            include_bytes!("../definitions/local-charter/lifecycle-v1.yaml").as_slice(),
        ),
    ];
    let expected = [
        json!({"schema_id":"handbook.native-local-charter-profile-definition","schema_version":"1.0","definition_ref":PROFILE_REF,"base_profile_ref":"handbook.profile.shipped-root@1.2.0","base_profile_definition_fingerprint":decisions.profile_definition_fingerprint().as_str(),"approval_policy_ref":APPROVAL_REF,"intake_definition_ref":INTAKE_REF,"lifecycle_policy_ref":LIFECYCLE_REF,"default_replacement":"forbidden"}),
        json!({"schema_id":"handbook.native-local-charter-approval-definition","schema_version":"1.0","definition_ref":APPROVAL_REF,"assurance":"local_advisory","creation":"initial_only","approval":"explicit_exact_canonical_bytes","claims":"retained_unverified_principal_agent_delegation_source","authenticator_fallback":"forbidden","amendment":"unsupported"}),
        json!({"schema_id":"handbook.native-local-charter-intake-definition","schema_version":"1.0","definition_ref":INTAKE_REF,"coverage_validation_ref":"handbook.intake.charter@1.0.0","approval_policy_ref":APPROVAL_REF,"target_kind_ref":"handbook.artifact-kind.project-authority@1.1.0","target_schema_ref":"handbook.schemas.artifacts.project-authority@1.1.0","waivers":"unsupported"}),
        json!({"schema_id":"handbook.native-local-charter-lifecycle-definition","schema_version":"1.0","definition_ref":LIFECYCLE_REF,"approval_policy_ref":APPROVAL_REF,"initial_state":"current","canonical_change":"invalidate_operating_authority","amendment":"unsupported","state_loss":"refuse"}),
    ];
    let mut closure = Vec::new();
    for ((reference, bytes), expected) in definitions.into_iter().zip(expected) {
        let value = parse_definition_yaml(bytes).map_err(|_| {
            refuse(
                "incompatible_definition_closure",
                "local definition parse refused",
            )
        })?;
        if value != expected {
            return Err(refuse(
                "incompatible_definition_closure",
                "local definition differs from its supported closed semantics",
            ));
        }
        closure.push(
            json!({"definition_ref":reference,"definition_fingerprint":fingerprint(&value)?}),
        );
    }
    for reference in registry.refs() {
        let record = registry.record(reference).expect("registry reference");
        closure.push(json!({"definition_ref":reference.as_str(),"definition_fingerprint":record.definition_fingerprint().as_str()}));
    }
    // The actual native shipped resolver validates these unchanged producers.
    for (reference, field) in [
        (
            "handbook.context-resolution.shipped-root@1.0.0",
            "definition_fingerprint",
        ),
        (
            "handbook.mutation-matcher.core@1.0.0",
            "definition_fingerprint",
        ),
        (
            "handbook.resolution-escalation.core@1.0.0",
            "policy_fingerprint",
        ),
        ("handbook.memory-promotion.core@1.0.0", "policy_fingerprint"),
    ] {
        let exact = crate::ExactDefinitionRef::parse(reference)
            .map_err(|_| refuse("incompatible_definition_closure", "base reference refused"))?;
        let source = crate::profile_builtins::definition(&exact).ok_or_else(|| {
            refuse(
                "incompatible_definition_closure",
                "base producer is missing",
            )
        })?;
        let value = parse_definition_yaml(source.bytes).map_err(|_| {
            refuse(
                "incompatible_definition_closure",
                "base producer parse refused",
            )
        })?;
        closure.push(json!({"definition_ref":reference,"definition_fingerprint":value.get(field).and_then(Value::as_str).ok_or_else(|| refuse("incompatible_definition_closure", "base producer fingerprint is missing"))?}));
    }
    Ok(LocalCharterProfileBindingV1 {
        profile_ref: PROFILE_REF.into(),
        profile_fingerprint: closure[0]["definition_fingerprint"]
            .as_str()
            .expect("built fingerprint")
            .into(),
        closure_fingerprint: fingerprint(
            &json!({"base_profile_ref":decisions.profile_ref().as_str(),"base_profile_definition_fingerprint":decisions.profile_definition_fingerprint().as_str(),"base_resolved_profile_fingerprint":decisions.profile_fingerprint().as_str(),"resolved_definitions":closure}),
        )?,
    })
}

fn material(
    root: &RetainedRepositoryRootV1,
    locator: &LocalCharterLocatorV1,
) -> Result<LocalMaterial, LocalAuthorityRefusalV1> {
    validate_prospective_binding(root, &locator.binding)?;
    let prepared = NativeLocalCharterServiceV1::prepare(
        root.initialization_root_path(),
        &locator.binding,
        locator.request.clone(),
    )?;
    if &prepared.locator != locator {
        return Err(refuse(
            "local_charter_locator_mismatch",
            "locator differs from its exact original request, root or definition closure",
        ));
    }
    let (canonical, _, validation) = evaluate(root.initialization_root_path(), &locator.request)?;
    let common = json!({"request_fingerprint":locator.request_fingerprint,"canonical_fingerprint":locator.canonical_fingerprint,"profile_binding":locator.profile_binding});
    let values = [
        (
            "intake",
            json!({"schema_id":"handbook.native-local-charter-intake","schema_version":"1.0","binding":common,"intake_definition_ref":INTAKE_REF,"envelope":locator.request.intake}),
        ),
        (
            "candidate",
            json!({"schema_id":"handbook.native-local-charter-candidate","schema_version":"1.0","binding":common,"target_instance_id":"project_authority","basis":null,"approval_policy_ref":APPROVAL_REF}),
        ),
        (
            "validation",
            json!({"schema_id":"handbook.native-local-charter-validation","schema_version":"1.0","binding":common,"native_validation":validation}),
        ),
        (
            "approval",
            json!({"schema_id":"handbook.native-local-charter-approval","schema_version":"1.0","binding":common,"rule_ref":APPROVAL_REF,"assurance":"local_advisory","approval":locator.request.human_approval}),
        ),
    ];
    let mut records = Vec::new();
    for (kind, value) in values {
        records.push(record(kind, &value)?);
    }
    let lineage = records.iter().map(|(reference, bytes)| json!({"reference":reference,"document_fingerprint":DefinitionFingerprint::from_bytes(bytes).to_string()})).collect::<Vec<_>>();
    let promotion = record(
        "promotion",
        &json!({"schema_id":"handbook.native-local-charter-promotion","schema_version":"1.0","binding":common,"predecessor":null,"canonical_path":CANONICAL,"lineage":lineage,"decision":"approved_advisory"}),
    )?;
    let promotion_ref = promotion.0.clone();
    records.push(promotion);
    let state_fingerprint = fingerprint(
        &json!({"lifecycle_policy_ref":LIFECYCLE_REF,"canonical_fingerprint":locator.canonical_fingerprint,"state":"current","active_observations":[]}),
    )?;
    let lifecycle = record(
        "lifecycle",
        &json!({"schema_id":"handbook.native-local-charter-lifecycle","schema_version":"1.0","binding":common,"lifecycle_policy_ref":LIFECYCLE_REF,"promotion_ref":promotion_ref,"state":"current","state_fingerprint":state_fingerprint,"active_observations":[]}),
    )?;
    let lifecycle_transition_ref = lifecycle.0.clone();
    records.push(lifecycle);
    let outcome = LocalCharterOutcomeV1 {
        locator: locator.clone(),
        canonical_fingerprint: locator.canonical_fingerprint.clone(),
        profile_binding: locator.profile_binding.clone(),
        promotion_ref,
        lifecycle_transition_ref,
        human_approval_accepted: true,
        advisory_permission_accepted: true,
    };
    let head = encode(
        &json!({"schema_id":"handbook.native-local-charter-head","schema_version":"1.0","request_fingerprint":locator.request_fingerprint,"canonical_fingerprint":locator.canonical_fingerprint,"promotion_ref":outcome.promotion_ref,"lifecycle_transition_ref":outcome.lifecycle_transition_ref,"profile_binding":locator.profile_binding}),
    )?;
    let marker = encode(
        &json!({"schema_id":"handbook.native-local-charter-commit","schema_version":"1.0","request_fingerprint":locator.request_fingerprint,"head_document_fingerprint":DefinitionFingerprint::from_bytes(&head).to_string()}),
    )?;
    Ok(LocalMaterial {
        canonical,
        records,
        head,
        marker,
        outcome,
    })
}

fn record(kind: &str, value: &Value) -> Result<(String, Vec<u8>), LocalAuthorityRefusalV1> {
    let bytes = encode(value)?;
    let fp = DefinitionFingerprint::from_bytes(&bytes).to_string();
    Ok((
        format!("{LOCAL_CHARTER_ROOT_V1}/records/{kind}_{}.json", &fp[7..]),
        bytes,
    ))
}

fn execute(
    repo: &Path,
    locator: &LocalCharterLocatorV1,
    may_start: bool,
) -> LocalCharterExecutionV1 {
    let mut began = false;
    let mut replayed = false;
    let result = (|| {
        let observed_root = RetainedRepositoryRootV1::open_exact(repo).map_err(filesystem)?;
        validate_prospective_binding(&observed_root, &locator.binding)?;
        if !may_start && read(&observed_root, INTENT)?.is_none() {
            return Err(refuse(
                "original_local_charter_request_missing",
                "restored locators cannot create or resume a missing original native intent",
            ));
        }
        let guard = RepositoryInitializationGuardV1::acquire(repo).map_err(filesystem)?;
        validate_binding_guard(&guard, &locator.binding)?;
        let local_exists = has_local_footprint(guard.root())?;
        let locks = acquire_charter_locks(guard.root(), !local_exists)?;
        let root = guard.root();
        let intent = encode(&LocalIntentV1 {
            schema_id: "handbook.native-local-charter-intent".into(),
            schema_version: "1.0".into(),
            locator: locator.clone(),
        })?;
        let expected = material(root, locator)?;
        if let Some(actual) = read(root, INTENT)? {
            if actual != intent {
                return Err(refuse(
                    "local_charter_request_conflict",
                    "retained initial request differs; no second bootstrap or reset is supported",
                ));
            }
            if read(root, COMMITTED)?.is_some() {
                replayed = true;
                return resolve_history(root, locator);
            }
            require_bytes(root, PENDING, &expected.marker)?;
        } else {
            if !may_start {
                return Err(refuse(
                    "original_local_charter_request_missing",
                    "resume requires the exact retained original intent",
                ));
            }
            if has_local_footprint(root)?
                || has_dependent_local_footprint(root)?
                || read(root, CANONICAL)?.is_some()
                || has_verified_charter_footprint(root)?
            {
                return Err(refuse("incompatible_selected_charter_authority", "an existing ordinary or incompatible Charter state requires a separately authorized boundary"));
            }
            began = true;
            root.create_directories(&format!("{LOCAL_CHARTER_ROOT_V1}/records"))
                .map_err(filesystem)?;
            root.write_new_durable(INTENT, &intent)
                .map_err(filesystem)?;
            root.write_new_durable(PENDING, &expected.marker)
                .map_err(filesystem)?;
        }
        began = true;
        for (reference, bytes) in &expected.records {
            ensure_exact_or_create(root, reference, bytes)?;
        }
        root.create_directories(".handbook/project")
            .map_err(filesystem)?;
        ensure_exact_or_create(root, CANONICAL, &expected.canonical)?;
        ensure_exact_or_create(root, HEAD, &expected.head)?;
        validate_binding_guard(&guard, &locator.binding)?;
        validate_locks(root, &locks)?;
        root.rename_noreplace_durable(PENDING, COMMITTED)
            .map_err(filesystem)?;
        resolve_current_retained(root, locator)
    })();
    match result {
        Ok(outcome) => LocalCharterExecutionV1 {
            disposition: if replayed {
                LocalCharterDispositionV1::Replayed
            } else {
                LocalCharterDispositionV1::Committed
            },
            outcome: Some(outcome),
            refusal: None,
        },
        Err(refusal) => LocalCharterExecutionV1 {
            disposition: if began {
                LocalCharterDispositionV1::Uncertain
            } else {
                LocalCharterDispositionV1::Refused
            },
            outcome: None,
            refusal: Some(refusal),
        },
    }
}

fn resolve_history(
    root: &RetainedRepositoryRootV1,
    locator: &LocalCharterLocatorV1,
) -> Result<LocalCharterOutcomeV1, LocalAuthorityRefusalV1> {
    validate_binding_retained(root, &locator.binding)?;
    let expected = material(root, locator)?;
    let intent = encode(&LocalIntentV1 {
        schema_id: "handbook.native-local-charter-intent".into(),
        schema_version: "1.0".into(),
        locator: locator.clone(),
    })?;
    require_bytes(root, INTENT, &intent)?;
    for (reference, bytes) in &expected.records {
        require_bytes(root, reference, bytes)?;
    }
    require_bytes(root, COMMITTED, &expected.marker)?;
    if read(root, PENDING)?.is_some() {
        return Err(refuse(
            "local_charter_terminal_conflict",
            "both pending and committed native terminal states are present",
        ));
    }
    Ok(expected.outcome)
}

pub(crate) fn resolve_current_retained(
    root: &RetainedRepositoryRootV1,
    locator: &LocalCharterLocatorV1,
) -> Result<LocalCharterOutcomeV1, LocalAuthorityRefusalV1> {
    require_existing_locks(root)?;
    let outcome = resolve_history(root, locator)?;
    let expected = material(root, locator)?;
    require_bytes(root, CANONICAL, &expected.canonical)?;
    require_bytes(root, HEAD, &expected.head)?;
    if has_verified_charter_footprint(root)? {
        return Err(refuse(
            "conflicting_charter_authority",
            "local and verified Charter histories cannot both be selected",
        ));
    }
    validate_binding_retained(root, &locator.binding)?;
    Ok(outcome)
}

/// The additive evaluation owner resolves original Charter history without
/// reinterpreting later selection/currentness or reacquiring native owner locks.
pub(crate) fn local_evaluation_historical_dependencies(
    root: &RetainedRepositoryRootV1,
    locator: &LocalCharterLocatorV1,
) -> Result<Vec<String>, LocalAuthorityRefusalV1> {
    resolve_history(root, locator)?;
    let expected = material(root, locator)?;
    let mut paths = expected
        .records
        .into_iter()
        .map(|(path, _)| path)
        .collect::<Vec<_>>();
    paths.extend([
        INTENT.to_owned(),
        COMMITTED.to_owned(),
        REPOSITORY_IDENTITY_REPO_PATH.to_owned(),
        REPOSITORY_INITIALIZATION_CLAIM_PATH_V1.to_owned(),
        REPOSITORY_INITIALIZATION_RECEIPT_PATH_V1.to_owned(),
        native_terminal(root, &locator.binding)?.0,
    ]);
    paths.sort();
    paths.dedup();
    Ok(paths)
}

/// Exact self-contained original Charter closure for the new work owner. This
/// validates its original selected local rule from retained bytes, without
/// consulting a live Charter selection or requiring runtime operating locks.
pub(crate) fn work_registration_historical_bytes(
    locator: &LocalCharterLocatorV1,
) -> Result<Vec<(String, Vec<u8>)>, LocalAuthorityRefusalV1> {
    let (canonical, profile, validation) = evaluate(Path::new(""), &locator.request)?;
    if profile != locator.profile_binding
        || DefinitionFingerprint::from_bytes(&canonical).as_str() != locator.canonical_fingerprint
        || fingerprint(
            &json!({"schema_id":"handbook.native-local-charter-request","schema_version":"1.0","binding":locator.binding,"request":locator.request,"profile_binding":profile}),
        )? != locator.request_fingerprint
    {
        return Err(refuse(
            "work_charter_history",
            "original exact Charter request, profile or canonical binding changed",
        ));
    }
    let common = json!({"request_fingerprint":locator.request_fingerprint,"canonical_fingerprint":locator.canonical_fingerprint,"profile_binding":locator.profile_binding});
    let mut records = vec![
        record(
            "intake",
            &json!({"schema_id":"handbook.native-local-charter-intake","schema_version":"1.0","binding":common,"intake_definition_ref":INTAKE_REF,"envelope":locator.request.intake}),
        )?,
        record(
            "candidate",
            &json!({"schema_id":"handbook.native-local-charter-candidate","schema_version":"1.0","binding":common,"target_instance_id":"project_authority","basis":null,"approval_policy_ref":APPROVAL_REF}),
        )?,
        record(
            "validation",
            &json!({"schema_id":"handbook.native-local-charter-validation","schema_version":"1.0","binding":common,"native_validation":validation}),
        )?,
        record(
            "approval",
            &json!({"schema_id":"handbook.native-local-charter-approval","schema_version":"1.0","binding":common,"rule_ref":APPROVAL_REF,"assurance":"local_advisory","approval":locator.request.human_approval}),
        )?,
    ];
    let lineage = records.iter().map(|(reference, bytes)| json!({"reference":reference,"document_fingerprint":DefinitionFingerprint::from_bytes(bytes).to_string()})).collect::<Vec<_>>();
    let promotion = record(
        "promotion",
        &json!({"schema_id":"handbook.native-local-charter-promotion","schema_version":"1.0","binding":common,"predecessor":null,"canonical_path":CANONICAL,"lineage":lineage,"decision":"approved_advisory"}),
    )?;
    let promotion_ref = promotion.0.clone();
    records.push(promotion);
    let state_fingerprint = fingerprint(
        &json!({"lifecycle_policy_ref":LIFECYCLE_REF,"canonical_fingerprint":locator.canonical_fingerprint,"state":"current","active_observations":[]}),
    )?;
    let lifecycle = record(
        "lifecycle",
        &json!({"schema_id":"handbook.native-local-charter-lifecycle","schema_version":"1.0","binding":common,"lifecycle_policy_ref":LIFECYCLE_REF,"promotion_ref":promotion_ref,"state":"current","state_fingerprint":state_fingerprint,"active_observations":[]}),
    )?;
    let lifecycle_transition_ref = lifecycle.0.clone();
    records.push(lifecycle);
    let head = encode(
        &json!({"schema_id":"handbook.native-local-charter-head","schema_version":"1.0","request_fingerprint":locator.request_fingerprint,"canonical_fingerprint":locator.canonical_fingerprint,"promotion_ref":promotion_ref,"lifecycle_transition_ref":lifecycle_transition_ref,"profile_binding":locator.profile_binding}),
    )?;
    records.extend([
        (INTENT.into(), encode(&LocalIntentV1 { schema_id: "handbook.native-local-charter-intent".into(), schema_version: "1.0".into(), locator: locator.clone() })?),
        (COMMITTED.into(), encode(&json!({"schema_id":"handbook.native-local-charter-commit","schema_version":"1.0","request_fingerprint":locator.request_fingerprint,"head_document_fingerprint":DefinitionFingerprint::from_bytes(&head).to_string()}))?),
        (HEAD.into(), head),
        (CANONICAL.into(), canonical),
    ]);
    records.sort_by(|a, b| a.0.cmp(&b.0));
    Ok(records)
}

/// Common readers hold their native locks already. No nested lock is taken.
pub(crate) fn read_local_committed_retained(
    root: &RetainedRepositoryRootV1,
) -> Result<Option<CommittedCharterAuthorityV1>, LocalAuthorityRefusalV1> {
    let Some(bytes) = read(root, INTENT)? else {
        if has_local_footprint(root)? {
            return Err(refuse(
                "required_local_charter_state_missing",
                "local Charter footprint lost its retained original intent",
            ));
        }
        return Ok(None);
    };
    let intent: LocalIntentV1 = decode(&bytes)?;
    if intent.schema_id != "handbook.native-local-charter-intent" || intent.schema_version != "1.0"
    {
        return Err(refuse(
            "local_charter_intent_invalid",
            "unsupported local Charter intent",
        ));
    }
    let outcome = resolve_current_retained(root, &intent.locator)?;
    let canonical = material(root, &intent.locator)?.canonical;
    Ok(Some(CommittedCharterAuthorityV1 {
        canonical_bytes: canonical,
        canonical_fingerprint: outcome.canonical_fingerprint,
        promotion_ref: outcome.promotion_ref,
        lifecycle_transition_ref: outcome.lifecycle_transition_ref,
    }))
}

pub(crate) fn read_local_lifecycle_retained(
    root: &RetainedRepositoryRootV1,
) -> Result<Option<RetainedLifecycleAuthorityV1>, LocalAuthorityRefusalV1> {
    let Some(committed) = read_local_committed_retained(root)? else {
        return Ok(None);
    };
    let intent: LocalIntentV1 = decode(&require_read(root, INTENT)?)?;
    let expected = material(root, &intent.locator)?;
    let bytes = require_read(root, &committed.lifecycle_transition_ref)?;
    let value = parse_schema_json(&bytes).map_err(|_| {
        refuse(
            "local_lifecycle_invalid",
            "local lifecycle record is not exact JSON",
        )
    })?;
    let state_fingerprint = value["state_fingerprint"]
        .as_str()
        .ok_or_else(|| {
            refuse(
                "local_lifecycle_invalid",
                "local lifecycle state fingerprint missing",
            )
        })?
        .to_owned();
    let transition_fingerprint = DefinitionFingerprint::from_bytes(&bytes).to_string();
    Ok(Some(RetainedLifecycleAuthorityV1 {
        canonical_bytes: Some(expected.canonical),
        authority: Some(CharterLifecycleAuthorityV1 {
            canonical_fingerprint: committed.canonical_fingerprint,
            lifecycle_transition_ref: committed.lifecycle_transition_ref.clone(),
            lifecycle_transition_fingerprint: transition_fingerprint,
            state: CharterLifecycleState::Current,
            state_fingerprint,
            active_observation_refs: vec![],
            active_observation_fingerprints: vec![],
            active_observations: vec![],
            reopened_coverage_ids: vec![],
        }),
        records: vec![RetainedLifecycleRecordV1 {
            relative_ref: committed.lifecycle_transition_ref,
            bytes,
        }],
    }))
}

pub(crate) fn has_local_footprint(
    root: &RetainedRepositoryRootV1,
) -> Result<bool, LocalAuthorityRefusalV1> {
    if !root.initialization_handbook_exists().map_err(filesystem)? {
        return Ok(false);
    }
    Ok(root
        .list_directory_bounded(".handbook", 4096)
        .map_err(filesystem)?
        .iter()
        .any(|entry| matches!(entry.name(), "local-charter" | "local-operating")))
}

fn has_dependent_local_footprint(
    root: &RetainedRepositoryRootV1,
) -> Result<bool, LocalAuthorityRefusalV1> {
    if !root.initialization_handbook_exists().map_err(filesystem)? {
        return Ok(false);
    }
    Ok(root
        .list_directory_bounded(".handbook", 4096)
        .map_err(filesystem)?
        .iter()
        .any(|entry| entry.name() == "local-operating"))
}

/// Selection probe only. Authority is always re-read under the retained root.
/// Keeping this narrow avoids changing the accepted roots of legacy routes.
pub(crate) fn local_partition_present(repo: &Path) -> Result<bool, LocalAuthorityRefusalV1> {
    for partition in [".handbook/local-charter", ".handbook/local-operating"] {
        match std::fs::symlink_metadata(repo.join(partition)) {
            Ok(_) => return Ok(true),
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => {}
            Err(_) => {
                return Err(refuse(
                    "local_charter_filesystem_refused",
                    "local Charter selection cannot be inspected",
                ))
            }
        }
    }
    Ok(false)
}

fn has_verified_charter_footprint(
    root: &RetainedRepositoryRootV1,
) -> Result<bool, LocalAuthorityRefusalV1> {
    for directory in [
        ".handbook/state/transactions/promotions",
        ".handbook/state/transactions/posture-transitions",
    ] {
        if directory_present(root, directory)?
            && !root
                .list_directory_bounded(directory, 4096)
                .map_err(filesystem)?
                .is_empty()
        {
            return Ok(true);
        }
    }
    Ok(false)
}

fn directory_present(
    root: &RetainedRepositoryRootV1,
    relative: &str,
) -> Result<bool, LocalAuthorityRefusalV1> {
    if !root.initialization_handbook_exists().map_err(filesystem)? {
        return Ok(false);
    }
    let mut parent = ".handbook".to_owned();
    for component in relative.split('/').skip(1) {
        let entries = root
            .list_directory_bounded(&parent, 4096)
            .map_err(filesystem)?;
        match entries.iter().find(|entry| entry.name() == component) {
            None => return Ok(false),
            Some(entry) if entry.kind() == crate::RepositoryDirectoryEntryKindV1::Directory => {}
            Some(_) => {
                return Err(refuse(
                    "local_charter_filesystem_refused",
                    "owner directory source is not a native directory",
                ))
            }
        }
        parent.push('/');
        parent.push_str(component);
    }
    Ok(true)
}

fn validate_prospective_binding(
    root: &RetainedRepositoryRootV1,
    binding: &LocalProjectBindingV1,
) -> Result<(), LocalAuthorityRefusalV1> {
    let claim = &binding.initialization_claim;
    if claim.native_owner() != DEFERRAL_OWNER {
        return Err(refuse(
            "defer_first_registration_required",
            "only exact native admission-deferral 2.0 initialization is supported",
        ));
    }
    let computed = RepositoryInitializationClaimV1::prepare(
        root.initialization_root_path(),
        root,
        claim.native_owner(),
        claim.request_digest(),
        claim.proposed_repository_identity(),
    )
    .map_err(filesystem)?;
    if &computed != claim {
        return Err(refuse(
            "project_root_mismatch",
            "project claim differs from the exact physical root/path/identity",
        ));
    }
    Ok(())
}

#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
struct BindingClaimRecord {
    schema_id: String,
    schema_version: String,
    claim: RepositoryInitializationClaimV1,
    retained_lock_fingerprint: String,
}
#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
struct BindingReceipt {
    schema_id: String,
    schema_version: String,
    claim_fingerprint: String,
    disposition: RepositoryInitializationDispositionV1,
    terminal_path: String,
    terminal_digest: String,
}

fn native_terminal(
    root: &RetainedRepositoryRootV1,
    binding: &LocalProjectBindingV1,
) -> Result<(String, Vec<u8>), LocalAuthorityRefusalV1> {
    validate_prospective_binding(root, binding)?;
    let claim: BindingClaimRecord = decode(&require_read(
        root,
        REPOSITORY_INITIALIZATION_CLAIM_PATH_V1,
    )?)?;
    if claim.schema_id != "handbook.repository-initialization-claim-record"
        || claim.schema_version != "1.0"
        || claim.claim != binding.initialization_claim
        || claim.retained_lock_fingerprint
            != root
                .initialization_lock_physical_fingerprint_v1()
                .map_err(filesystem)?
    {
        return Err(refuse(
            "registration_claim_mismatch",
            "retained native claim or physical lock differs from the supplied project binding",
        ));
    }
    let receipt: BindingReceipt = decode(&require_read(
        root,
        REPOSITORY_INITIALIZATION_RECEIPT_PATH_V1,
    )?)?;
    if receipt.schema_id != "handbook.repository-initialization-receipt"
        || receipt.schema_version != "1.0"
        || receipt.claim_fingerprint != binding.initialization_claim.claim_fingerprint()
        || receipt.disposition != RepositoryInitializationDispositionV1::Committed
        || receipt.terminal_path.starts_with(".handbook/state/")
        || [
            REPOSITORY_INITIALIZATION_CLAIM_PATH_V1,
            REPOSITORY_INITIALIZATION_RECEIPT_PATH_V1,
            REPOSITORY_INITIALIZATION_LOCK_PATH_V1,
            REPOSITORY_IDENTITY_REPO_PATH,
        ]
        .contains(&receipt.terminal_path.as_str())
    {
        return Err(refuse(
            "committed_registration_required",
            "exact native committed deferral receipt is required",
        ));
    }
    let terminal = require_read(root, &receipt.terminal_path)?;
    if terminal.is_empty()
        || receipt.terminal_digest
            != domain_fingerprint(
                "handbook.repository-initialization-terminal-bytes@1.0",
                &terminal,
            )
    {
        return Err(refuse(
            "registration_terminal_mismatch",
            "native terminal bytes differ from their exact receipt",
        ));
    }
    require_bytes(
        root,
        REPOSITORY_IDENTITY_REPO_PATH,
        binding
            .initialization_claim
            .proposed_repository_identity()
            .as_bytes(),
    )?;
    Ok((receipt.terminal_path, terminal))
}

pub(crate) fn validate_binding_retained(
    root: &RetainedRepositoryRootV1,
    binding: &LocalProjectBindingV1,
) -> Result<(), LocalAuthorityRefusalV1> {
    native_terminal(root, binding).map(|_| ())
}

pub(crate) fn validate_binding_guarded(
    guard: &RepositoryInitializationGuardV1,
    binding: &LocalProjectBindingV1,
) -> Result<(), LocalAuthorityRefusalV1> {
    let (path, terminal) = native_terminal(guard.root(), binding)?;
    guard
        .require_native_terminal(
            &binding.initialization_claim,
            RepositoryInitializationDispositionV1::Committed,
            &path,
            &terminal,
        )
        .map_err(filesystem)
}

fn validate_binding_guard(
    guard: &RepositoryInitializationGuardV1,
    binding: &LocalProjectBindingV1,
) -> Result<(), LocalAuthorityRefusalV1> {
    validate_binding_guarded(guard, binding)
}

struct LocalLocks(Vec<RepositoryExclusiveLockV1>);
impl Drop for LocalLocks {
    fn drop(&mut self) {
        while let Some(lock) = self.0.pop() {
            drop(lock);
        }
    }
}
fn acquire_charter_locks(
    root: &RetainedRepositoryRootV1,
    create: bool,
) -> Result<LocalLocks, LocalAuthorityRefusalV1> {
    if create {
        root.create_directories(".handbook/state/locks")
            .map_err(filesystem)?;
    } else {
        require_existing_locks(root)?;
    }
    let mut locks = LocalLocks(Vec::new());
    for name in LOCKS {
        locks.0.push(
            root.acquire_exclusive_lock(&format!(".handbook/state/locks/{name}"))
                .map_err(filesystem)?,
        );
    }
    Ok(locks)
}
fn require_existing_locks(root: &RetainedRepositoryRootV1) -> Result<(), LocalAuthorityRefusalV1> {
    if !directory_present(root, ".handbook/state/locks")? {
        return Err(refuse(
            "required_local_charter_state_missing",
            "native Charter lock directory is absent",
        ));
    }
    for reference in std::iter::once(REPOSITORY_INITIALIZATION_LOCK_PATH_V1.to_owned()).chain(
        LOCKS
            .into_iter()
            .map(|name| format!(".handbook/state/locks/{name}")),
    ) {
        if !root
            .initialization_regular_file_exists(&reference, 0)
            .map_err(filesystem)?
        {
            return Err(refuse(
                "required_local_charter_state_missing",
                format!("native lock source is absent: {reference}"),
            ));
        }
    }
    Ok(())
}

pub(crate) fn require_local_read_locks(
    root: &RetainedRepositoryRootV1,
) -> Result<(), LocalAuthorityRefusalV1> {
    if has_local_footprint(root)? {
        require_existing_locks(root)?;
    }
    Ok(())
}
fn validate_locks(
    root: &RetainedRepositoryRootV1,
    locks: &LocalLocks,
) -> Result<(), LocalAuthorityRefusalV1> {
    for (name, guard) in LOCKS.iter().zip(&locks.0) {
        guard
            .validate_at(root, &format!(".handbook/state/locks/{name}"))
            .map_err(filesystem)?;
    }
    Ok(())
}

fn ensure_exact_or_create(
    root: &RetainedRepositoryRootV1,
    reference: &str,
    bytes: &[u8],
) -> Result<(), LocalAuthorityRefusalV1> {
    match read(root, reference)? {
        Some(actual) if actual == bytes => Ok(()),
        Some(_) => Err(refuse(
            "local_charter_state_conflict",
            "same native owner reference contains different bytes",
        )),
        None => root.write_new_durable(reference, bytes).map_err(filesystem),
    }
}
fn require_bytes(
    root: &RetainedRepositoryRootV1,
    reference: &str,
    expected: &[u8],
) -> Result<(), LocalAuthorityRefusalV1> {
    if read(root, reference)?.as_deref() != Some(expected) {
        return Err(refuse(
            "required_local_charter_state_missing",
            format!("required exact owner source is absent or changed: {reference}"),
        ));
    }
    Ok(())
}
fn require_read(
    root: &RetainedRepositoryRootV1,
    reference: &str,
) -> Result<Vec<u8>, LocalAuthorityRefusalV1> {
    read(root, reference)?.ok_or_else(|| {
        refuse(
            "required_local_charter_state_missing",
            format!("required owner source is absent: {reference}"),
        )
    })
}
fn read(
    root: &RetainedRepositoryRootV1,
    reference: &str,
) -> Result<Option<Vec<u8>>, LocalAuthorityRefusalV1> {
    match root.read_local_owner_optional_bounded(reference, MAX_RECORD) {
        Ok(RepositoryReadV1::Absent) => Ok(None),
        Ok(RepositoryReadV1::Found(bytes)) => Ok(Some(bytes)),
        Err(error) if error.kind() == crate::RepositoryDurabilityErrorKindV1::NotFound => Ok(None),
        Err(error) => Err(refuse(
            "local_charter_filesystem_refused",
            format!("{reference}: {}", error.detail()),
        )),
    }
}
fn encode<T: Serialize>(value: &T) -> Result<Vec<u8>, LocalAuthorityRefusalV1> {
    let value = serde_json::to_value(value).map_err(|_| {
        refuse(
            "local_record_invalid",
            "local owner closed encoding refused",
        )
    })?;
    let bytes = canonical_json_bytes(&value).map_err(|_| {
        refuse(
            "local_record_invalid",
            "local owner canonical JSON encoding refused",
        )
    })?;
    if bytes.len() > MAX_RECORD {
        return Err(refuse(
            "local_record_bound_exceeded",
            "local owner JSON exceeds 1 MiB",
        ));
    }
    Ok(bytes)
}
fn decode<T: for<'de> Deserialize<'de> + Serialize>(
    bytes: &[u8],
) -> Result<T, LocalAuthorityRefusalV1> {
    if bytes.len() > MAX_RECORD {
        return Err(refuse(
            "local_record_bound_exceeded",
            "local owner JSON exceeds 1 MiB",
        ));
    }
    let value = parse_schema_json(bytes).map_err(|_| {
        refuse(
            "local_record_invalid",
            "local owner JSON is malformed or has duplicate fields",
        )
    })?;
    let decoded: T = serde_json::from_value(value).map_err(|_| {
        refuse(
            "local_record_invalid",
            "local owner record is not its closed supported shape",
        )
    })?;
    if encode(&decoded)? != bytes {
        return Err(refuse(
            "local_record_invalid",
            "local owner stored bytes are not exact canonical JSON",
        ));
    }
    Ok(decoded)
}
fn fingerprint(value: &Value) -> Result<String, LocalAuthorityRefusalV1> {
    DefinitionFingerprint::from_json_value(value)
        .map(|fp| fp.to_string())
        .map_err(|_| refuse("local_record_invalid", "local semantic fingerprint refused"))
}
fn domain_fingerprint(domain: &str, bytes: &[u8]) -> String {
    let mut hash = Sha256::new();
    hash.update(domain.as_bytes());
    hash.update([0]);
    hash.update(bytes);
    format!("sha256:{:x}", hash.finalize())
}
fn bounded_text(value: &str, maximum: usize) -> bool {
    !value.trim().is_empty() && value.len() <= maximum && !value.chars().any(char::is_control)
}
fn refuse(code: &str, detail: impl Into<String>) -> LocalAuthorityRefusalV1 {
    LocalAuthorityRefusalV1 {
        code: code.into(),
        detail: detail.into(),
        unresolved_request: None,
    }
}
fn filesystem(error: crate::RepositoryDurabilityErrorV1) -> LocalAuthorityRefusalV1 {
    refuse("local_charter_filesystem_refused", error.detail())
}
fn execution(
    result: Result<LocalCharterOutcomeV1, LocalAuthorityRefusalV1>,
    disposition: LocalCharterDispositionV1,
) -> LocalCharterExecutionV1 {
    match result {
        Ok(outcome) => LocalCharterExecutionV1 {
            disposition,
            outcome: Some(outcome),
            refusal: None,
        },
        Err(refusal) => LocalCharterExecutionV1 {
            disposition: LocalCharterDispositionV1::Refused,
            outcome: None,
            refusal: Some(refusal),
        },
    }
}
