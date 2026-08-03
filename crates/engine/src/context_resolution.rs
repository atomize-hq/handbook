use crate::approver_registry_observation::{
    observe_committed_approver_registry_locked, CommittedApproverCredentialV1,
};
use crate::artifact_intake_registry::{
    RepositoryProfileSelectionV1, REPOSITORY_PROFILE_SELECTION_PATH,
};
use crate::artifact_lineage_store::GenericArtifactLineageStoreV1;
use crate::artifact_repository::{ArtifactRepositoryAuthorityGuardV1, ArtifactRepositoryV1};
use crate::charter_authenticator::{
    decode_and_verify_get_assertion_response, encode_get_assertion_request,
    select_eligible_credentials, AuthenticatorCredentialV1, CredentialSelectionCandidateV1,
    NativeAuthenticatorPortV1,
};
use crate::context_resolution_registry::ContextResolutionStackDefinition;
use crate::definition_identity::{
    fingerprint_serializable, DefinitionFingerprint, ExactDefinitionRef, SourceByteBudget,
};
use crate::stable_role_registry::read_trusted_repo_source;
use crate::{resolve_profile_selection, ResolvedInstanceProfile, SymbolicId};
use serde::Serialize;
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::collections::{BTreeMap, BTreeSet};
use std::path::{Path, PathBuf};
use std::sync::Arc;

#[rustfmt::skip]
macro_rules! kernel_error { ($kind:expr, $detail:expr $(,)?) => { ContextResolutionKernelError { kind: $kind, detail: ($detail).into(), candidate: None } }; ($kind:expr, $detail:expr, $candidate:expr $(,)?) => { ContextResolutionKernelError { kind: $kind, detail: ($detail).into(), candidate: Some(Box::new($candidate)) } } }

#[rustfmt::skip]
macro_rules! invalid_fingerprint { ($record:expr) => { kernel_error!(ContextResolutionKernelErrorKind::InvalidInput, format!("{} could not be fingerprinted", $record)) } }
#[rustfmt::skip]
macro_rules! validate_record_id { ($id:expr, $label:expr) => {{ let id = $id; if id.is_empty() || id.len() > 128 || !id.bytes().all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'.' | b'_' | b'-')) { Err(kernel_error!(ContextResolutionKernelErrorKind::InvalidInput, format!("{} is invalid", $label))) } else { Ok::<(), ContextResolutionKernelError>(()) } }} }
#[rustfmt::skip]
macro_rules! require_sorted_unique { ($bindings:expr, $label:expr) => {{ let bindings = $bindings; if bindings.len() > 256 || bindings.windows(2).any(|pair| pair[0].reference >= pair[1].reference) { Err(kernel_error!(ContextResolutionKernelErrorKind::InvalidInput, format!("{} must be strictly ordered and unique", $label))) } else { Ok::<(), ContextResolutionKernelError>(()) } }} }
#[rustfmt::skip]
macro_rules! validate_profile_stack { ($profile:expr, $stack:expr, $input:expr) => {{ let profile=$profile; let stack=$stack; let input=$input; if profile.context_resolution().exact_ref() != stack.exact_ref() || profile.context_resolution().definition_fingerprint() != stack.definition_fingerprint() || input.resolved_profile.reference != profile.exact_ref().as_str() || input.resolved_profile.fingerprint != profile.resolved_profile_fingerprint().as_str() || input.resolution_stack.reference != stack.exact_ref().as_str() || input.resolution_stack.fingerprint != stack.definition_fingerprint().as_str() { Err(kernel_error!(ContextResolutionKernelErrorKind::StaleAuthority, "envelope profile or stack binding is stale")) } else { Ok::<(), ContextResolutionKernelError>(()) } }} }
#[rustfmt::skip]
macro_rules! validate_envelope_values { ($stack:expr, $input:expr) => {{ let stack=$stack; let input=$input; if stack.ranks(&input.active_level_id, input.dimensions.values()).is_none() { Err(kernel_error!(ContextResolutionKernelErrorKind::InvalidInput, "envelope level or dimension value is unknown")) } else { Ok::<(), ContextResolutionKernelError>(()) } }} }
#[rustfmt::skip]
macro_rules! require_admission { ($admission:expr, $use:expr, $subject:expr, $fingerprint:expr $(,)?) => {{ let admission=$admission; let subject=$subject; let expected: Option<&str>=$fingerprint; if admission.authority_use != $use || admission.subject != *subject || expected.is_some_and(|value| admission.binding_fingerprint != value) { Err(kernel_error!(ContextResolutionKernelErrorKind::AuthorityRefused, "authority admission use, subject, or binding is incoherent")) } else { Ok::<(), ContextResolutionKernelError>(()) } }} }
#[rustfmt::skip]
macro_rules! require_admission_list { ($admissions:expr, $use:expr, $subjects:expr, $fingerprint:expr $(,)?) => {{ let admissions=$admissions; let subjects=$subjects; if admissions.len() != subjects.len() { Err(kernel_error!(ContextResolutionKernelErrorKind::Cardinality, "authority admission cardinality is not exact")) } else { admissions.iter().zip(subjects).try_for_each(|(admission, subject)| require_admission!(admission, $use, subject, $fingerprint)) } }} }
#[rustfmt::skip]
macro_rules! rule_binding { ($rule:expr) => {{ let rule=$rule; let fingerprint=fingerprint_serializable(rule).map_err(|_| invalid_fingerprint!("mutation rule evidence"))?; ContextResolutionExactBinding::new(&format!("context-resolution-mutation-rule/{}", rule.rule_id), fingerprint.as_str()) }} }
#[rustfmt::skip]
macro_rules! validate_selector { ($selector:expr) => {{ let selector=$selector; let malformed=selector.is_empty() || selector.len()>1024 || !selector.is_ascii() || selector.starts_with('/') || selector.ends_with('/') || selector.contains("//") || selector.contains('\\') || selector.contains('\0') || selector.contains("://"); let segments=selector.split('/').collect::<Vec<_>>(); let invalid=segments.is_empty() || segments.len()>64 || segments.iter().enumerate().any(|(index, segment)| segment.is_empty() || matches!(*segment, "." | "..") || (*segment=="**" && index+1 != segments.len()) || (segment.contains("**") && *segment!="**") || !segment.bytes().all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'.'|b'_'|b'*'|b'-'))); if malformed || invalid { Err(kernel_error!(ContextResolutionKernelErrorKind::Indeterminate, "mutation selector is malformed")) } else { Ok::<(), ContextResolutionKernelError>(()) } }} }
#[rustfmt::skip]
macro_rules! validate_target { ($target:expr) => {{ match validate_selector!($target) { Err(error) => Err(error), Ok(()) if $target.contains('*') => Err(kernel_error!(ContextResolutionKernelErrorKind::Indeterminate, "mutation target must be a literal repository path")), Ok(()) => Ok::<(), ContextResolutionKernelError>(()) } }} }
#[rustfmt::skip]
macro_rules! segment_matches { ($pattern:expr, $target:expr) => {{ let pattern=$pattern.as_bytes(); let target=$target.as_bytes(); let (mut p,mut t,mut star,mut mark)=(0usize,0usize,None,0usize); let mut matched=true; while t<target.len() { if p<pattern.len() && pattern[p]==target[t] {p+=1;t+=1;} else if p<pattern.len() && pattern[p]==b'*' {star=Some(p);p+=1;mark=t;} else if let Some(index)=star {p=index+1;mark+=1;t=mark;} else {matched=false;break;} } while p<pattern.len() && pattern[p]==b'*' {p+=1;} matched && p==pattern.len() }} }
#[rustfmt::skip]
macro_rules! selector_matches { ($selector:expr, $target:expr) => {{ let pattern=$selector.split('/').collect::<Vec<_>>(); let target=$target.split('/').collect::<Vec<_>>(); if pattern.last()==Some(&"**") { target.len()>=pattern.len()-1 && pattern[..pattern.len()-1].iter().zip(&target).all(|(p,t)| segment_matches!(p,t)) } else { pattern.len()==target.len() && pattern.iter().zip(target).all(|(p,t)| segment_matches!(p,t)) } }} }
#[rustfmt::skip]
macro_rules! selector_contains { ($parent:expr, $child:expr) => {{ let parent=$parent; let child=$child; if parent==child {true} else { let ps=parent.split('/').collect::<Vec<_>>(); let cs=child.split('/').collect::<Vec<_>>(); let pr=ps.last()==Some(&"**"); let cr=cs.last()==Some(&"**"); let pp=if pr {&ps[..ps.len()-1]} else {&ps[..]}; let cp=if cr {&cs[..cs.len()-1]} else {&cs[..]}; !((!pr && pp.len()!=cp.len()) || (pr && pp.len()>cp.len()) || (cr && !pr)) && pp.iter().zip(cp).all(|(p,c)| p==c || (!c.contains('*') && segment_matches!(p,c))) } }} }
#[rustfmt::skip]
macro_rules! semantic_memory_ref { ($reference:expr) => {{ let reference=$reference; (reference.starts_with("semantic-memory.") || reference.starts_with(".handbook/state/semantic-memory/")) && !reference.contains("..") && !reference.bytes().any(|byte| byte.is_ascii_control()) }} }
#[rustfmt::skip]
macro_rules! admit_terminal { ($records:expr, $by_request:expr, $id:expr, $binding:expr, $request:expr $(,)?) => {{ let records=$records; let by_request=$by_request; let id=$id; let binding=$binding; let request=$request; if let Some(existing_id)=by_request.get(request.reference()) { if existing_id==&id && records.get(&id)==Some(&binding) {Ok(false)} else {Err(kernel_error!(ContextResolutionKernelErrorKind::Cardinality, "request already has a terminal disposition"))} } else if records.contains_key(&id) {Err(kernel_error!(ContextResolutionKernelErrorKind::ReplayMismatch, "disposition ID was reused"))} else {by_request.insert(request.reference.clone(),id.clone());records.insert(id,binding);Ok(true)} }} }
#[rustfmt::skip]
macro_rules! stale_request { ($kind:expr) => { kernel_error!(ContextResolutionKernelErrorKind::StaleAuthority, format!("{} disposition cites an unknown or stale request", $kind)) } }

#[rustfmt::skip]
macro_rules! closed_object { ($value:expr, $keys:expr, $detail:expr) => {{
    let object = match $value.as_object() { Some(object) => object, None => return Err(kernel_error!(ContextResolutionKernelErrorKind::InvalidInput, $detail)) };
    let expected = $keys.into_iter().collect::<BTreeSet<_>>();
    if object.keys().map(String::as_str).collect::<BTreeSet<_>>() != expected { return Err(kernel_error!(ContextResolutionKernelErrorKind::InvalidInput, $detail)); }
    object
}} }

#[rustfmt::skip]
macro_rules! string_at { ($value:expr, $pointer:expr, $detail:expr) => {{ match $value.pointer($pointer).and_then(Value::as_str) { Some(value) if !value.is_empty() => value, _ => return Err(kernel_error!(ContextResolutionKernelErrorKind::InvalidInput, $detail)) } }} }

#[rustfmt::skip]
macro_rules! candidate_from_mapping { ($class:expr, $current:expr, $proposed:expr, $evidence:expr, $authority:expr, $profile:expr, $stack:expr $(,)?) => {{
    let class = $class; let authority = $authority;
    let pointer = format!("/candidate_mappings/{class}");
    let trigger = ContextResolutionExactBinding::new(string_at!(authority, &format!("{pointer}/trigger/ref"), "candidate class is not mapped by current authority"), string_at!(authority, &format!("{pointer}/trigger/fingerprint"), "candidate class is not mapped by current authority"))?;
    Ok::<_, ContextResolutionKernelError>(ContextResolutionEscalationCandidate { class: class.to_owned(), trigger, missing_condition: string_at!(authority, &format!("{pointer}/missing_condition"), "candidate class is not mapped by current authority").to_owned(), requested_authority_ref: string_at!(authority, "/authority_mappings/requested/authority_ref", "requested authority mapping is unavailable").to_owned(), evidence: $evidence, current: $current.clone(), proposed: $proposed.clone(), resolved_profile: $profile.clone(), resolution_stack: $stack.clone(), authority: authority.clone() })
}} }

#[rustfmt::skip]
macro_rules! materialize_envelope { ($input:expr, $authority:expr, $stack:expr, $layers:expr $(,)?) => {{
    let input = $input; let mut inherited_allow_layers = $layers;
    let local_allows = input.mutation_rules.iter().filter(|rule| rule.effect == ContextResolutionMutationEffect::Allow).cloned().collect::<Vec<_>>();
    let mutation_denies = input.mutation_rules.iter().filter(|rule| rule.effect == ContextResolutionMutationEffect::Deny).cloned().collect::<Vec<_>>();
    inherited_allow_layers.push(local_allows);
    let fingerprint = DefinitionFingerprint::from_json_value(&json!({"envelope_id": &input.envelope_id, "objective_ref": &input.objective_ref, "resolved_profile": &input.resolved_profile, "resolution_stack": &input.resolution_stack, "active_level_id": &input.active_level_id, "dimensions": &input.dimensions, "parent": &input.parent, "constraint_inputs": &input.constraint_inputs, "mutation_rules": &input.mutation_rules, "escalation_triggers": &input.escalation_triggers})).map_err(|_| kernel_error!(ContextResolutionKernelErrorKind::InvalidInput, "resolved envelope could not be fingerprinted"))?;
    let exact_binding = ContextResolutionExactBinding::new(&format!("context-resolution-envelope/{}", input.envelope_id), fingerprint.as_str())?;
    Ok::<_, ContextResolutionKernelError>(ContextResolutionEnvelope { exact_binding, _objective_ref: input.objective_ref, resolved_profile: input.resolved_profile, resolution_stack: input.resolution_stack, active_level_id: input.active_level_id, dimensions: input.dimensions, _constraint_inputs: input.constraint_inputs, mutation_allow_layers: inherited_allow_layers, mutation_denies, _escalation_triggers: input.escalation_triggers, authority: $authority, stack: $stack })
}} }

#[rustfmt::skip]
macro_rules! typed_candidate { ($current:expr, $class:expr, $proposed:expr, $evidence:expr) => {{
    let current = $current; let proposed = $proposed; let evidence = $evidence;
    if current.resolved_profile != proposed.resolved_profile || current.resolution_stack != proposed.resolution_stack || current.exact_binding == proposed.exact_binding || evidence.is_empty() || evidence.len() > 256 || evidence.windows(2).any(|pair| pair[0].reference >= pair[1].reference) { return Err(kernel_error!(ContextResolutionKernelErrorKind::InvalidInput, "typed escalation proposal relation or evidence is invalid")); }
    candidate_from_mapping!($class, &current.exact_binding, &proposed.exact_binding, evidence, &current.authority, &current.resolved_profile, &current.resolution_stack)
}} }

#[rustfmt::skip]
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd, Serialize)]
pub struct ContextResolutionExactBinding { reference: String, fingerprint: String }

impl ContextResolutionExactBinding {
    pub fn new(reference: &str, fingerprint: &str) -> Result<Self, ContextResolutionKernelError> {
        if reference.is_empty()
            || reference.len() > 1024
            || reference
                .bytes()
                .any(|byte| byte.is_ascii_control() || byte.is_ascii_whitespace())
        {
            return Err(kernel_error!(
                ContextResolutionKernelErrorKind::InvalidInput,
                "exact binding reference is invalid",
            ));
        }
        DefinitionFingerprint::parse(fingerprint).map_err(|_| {
            kernel_error!(
                ContextResolutionKernelErrorKind::InvalidInput,
                "exact binding fingerprint is invalid",
            )
        })?;
        Ok(Self {
            reference: reference.to_owned(),
            fingerprint: fingerprint.to_owned(),
        })
    }

    #[rustfmt::skip]
    pub fn reference(&self) -> &str { &self.reference }
    #[rustfmt::skip]
    pub fn fingerprint(&self) -> &str { &self.fingerprint }
}

#[rustfmt::skip]
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ContextResolutionAuthorityUse { RootCreation, Parent, Requested, Approving, Decision, Evidence, TriggerCondition, Constraint, Source, Target, TargetMemory }

impl ContextResolutionAuthorityUse {
    #[rustfmt::skip]
    pub const fn as_str(self) -> &'static str { match self { Self::RootCreation => "root_creation", Self::Parent => "parent", Self::Requested => "requested", Self::Approving => "approving", Self::Decision => "decision", Self::Evidence => "evidence", Self::TriggerCondition => "trigger_condition", Self::Constraint => "constraint", Self::Source => "source", Self::Target => "target", Self::TargetMemory => "target_memory" } }
}

#[rustfmt::skip]
#[derive(Clone, Debug)]
pub struct ContextResolutionAuthorityAdmission { authority_use: ContextResolutionAuthorityUse, subject: ContextResolutionExactBinding, binding_fingerprint: String, authority: Arc<Value> }

impl ContextResolutionAuthorityAdmission {
    #[rustfmt::skip]
    pub fn authority_use(&self) -> ContextResolutionAuthorityUse { self.authority_use }
    #[rustfmt::skip]
    pub fn subject(&self) -> &ContextResolutionExactBinding { &self.subject }
    #[rustfmt::skip]
    pub fn binding_fingerprint(&self) -> &str { &self.binding_fingerprint }
}

#[rustfmt::skip]
pub struct ContextResolutionAuthorityAdmissionResolver { repo_root: PathBuf, binding_kind_ref: String, binding_instance_id: String, expected_artifact_fingerprint: String, snapshot_binding: Arc<Value>, snapshot_artifact_fingerprint: String, cache: BTreeMap<(ContextResolutionAuthorityUse, String), ContextResolutionAuthorityAdmission>, counters: BTreeMap<Vec<u8>, u32> }

#[rustfmt::skip]
impl ContextResolutionAuthorityAdmissionResolver {
    pub fn new(
        repo_root: impl AsRef<Path>,
        binding_kind_ref: &str,
        binding_instance_id: &str,
        expected_artifact_fingerprint: &str,
    ) -> Result<Self, ContextResolutionKernelError> {
        ExactDefinitionRef::parse(binding_kind_ref).map_err(|_| {
            kernel_error!(
                ContextResolutionKernelErrorKind::InvalidInput,
                "authority binding kind ref is invalid",
            )
        })?;
        SymbolicId::parse(binding_instance_id).map_err(|_| {
            kernel_error!(
                ContextResolutionKernelErrorKind::InvalidInput,
                "authority binding instance ID is invalid",
            )
        })?;
        DefinitionFingerprint::parse(expected_artifact_fingerprint).map_err(|_| {
            kernel_error!(
                ContextResolutionKernelErrorKind::InvalidInput,
                "expected authority artifact fingerprint is invalid",
            )
        })?;
        let repo_root = repo_root.as_ref().to_path_buf();
        let (snapshot_binding, snapshot_artifact_fingerprint, _snapshot_credentials) = load_authority_snapshot(
            &repo_root,
            binding_kind_ref,
            binding_instance_id,
            expected_artifact_fingerprint,
        )?;
        Ok(Self {
            repo_root,
            binding_kind_ref: binding_kind_ref.to_owned(),
            binding_instance_id: binding_instance_id.to_owned(),
            expected_artifact_fingerprint: expected_artifact_fingerprint.to_owned(),
            snapshot_binding,
            snapshot_artifact_fingerprint,
            cache: BTreeMap::new(),
            counters: BTreeMap::new(),
        })
    }

    pub fn admit<P: NativeAuthenticatorPortV1>(
        &mut self,
        authenticator: &mut P,
        authority_use: ContextResolutionAuthorityUse,
        subject: &ContextResolutionExactBinding,
    ) -> Result<ContextResolutionAuthorityAdmission, ContextResolutionKernelError> {
        let key = (authority_use, subject.reference.clone());
        if let Some(cached) = self.cache.get(&key) {
            if cached.subject.fingerprint != subject.fingerprint {
                return Err(kernel_error!(
                    ContextResolutionKernelErrorKind::ReplayMismatch,
                    "authority subject reference was reused with changed bytes",
                ));
            }
            return Ok(cached.clone());
        }
        let (current_binding, current_artifact_fingerprint, current_credentials) = load_authority_snapshot(
            &self.repo_root,
            &self.binding_kind_ref,
            &self.binding_instance_id,
            &self.expected_artifact_fingerprint,
        )?;
        if current_artifact_fingerprint != self.snapshot_artifact_fingerprint
            || string_at!(&current_binding, "/binding_fingerprint", "authority binding fingerprint is unavailable")
                != string_at!(&self.snapshot_binding, "/binding_fingerprint", "authority binding fingerprint is unavailable")
        {
            return Err(kernel_error!(
                ContextResolutionKernelErrorKind::StaleAuthority,
                "context resolution authority binding changed",
            ));
        }
        let required_class = string_at!(&current_binding, &format!("/authority_mappings/{}/approval_class", authority_use.as_str()), "authority mapping is unavailable");
        let required_ref = string_at!(&current_binding, &format!("/authority_mappings/{}/authority_ref", authority_use.as_str()), "authority mapping is unavailable");
        let candidates = current_credentials
            .iter()
            .map(|credential| CredentialSelectionCandidateV1 {
                credential_id: credential.credential_id.clone(),
                active: credential.active,
                sequence: credential.use_sequence,
                algorithm_es256: true,
                covers_required_pair: credential.approval_mappings.iter().any(|pair| {
                    pair.approval_class == required_class && pair.authority_ref == required_ref
                }),
            })
            .collect::<Vec<_>>();
        let selected = select_eligible_credentials(&candidates).map_err(|_| {
            kernel_error!(
                ContextResolutionKernelErrorKind::AuthorityRefused,
                "eligible authority credentials could not be selected",
            )
        })?;
        if selected.is_empty() {
            return Err(kernel_error!(
                ContextResolutionKernelErrorKind::AuthorityRefused,
                "at least one credential must cover the authority mapping",
            ));
        }
        let requested = selected
            .iter()
            .map(|id| {
                let credential = current_credentials
                    .iter()
                    .find(|credential| credential.credential_id == *id)
                    .expect("selected credential came from current observation");
                AuthenticatorCredentialV1 {
                    credential_id: id.clone(),
                    cose_public_key: credential.cose_public_key.clone(),
                }
            })
            .collect::<Vec<_>>();
        let mut nonce = [0u8; 32];
        getrandom::fill(&mut nonce).map_err(|_| {
            kernel_error!(
                ContextResolutionKernelErrorKind::AuthorityRefused,
                "authority challenge entropy is unavailable",
            )
        })?;
        let mut challenge_preimage = Vec::new();
        for bytes in [b"handbook.context-resolution-authority.challenge.v1".as_slice(), nonce.as_slice(), string_at!(&current_binding, "/repository_identity_fingerprint", "authority binding is malformed").as_bytes(), string_at!(&current_binding, "/approver_registry/state_ref", "authority binding is malformed").as_bytes(), string_at!(&current_binding, "/approver_registry/state_fingerprint", "authority binding is malformed").as_bytes(), string_at!(&current_binding, "/approver_registry/head_transition_ref", "authority binding is malformed").as_bytes(), string_at!(&current_binding, "/approver_registry/head_transition_fingerprint", "authority binding is malformed").as_bytes(), current_artifact_fingerprint.as_bytes(), string_at!(&current_binding, "/binding_fingerprint", "authority binding is malformed").as_bytes(), authority_use.as_str().as_bytes(), subject.reference.as_bytes(), subject.fingerprint.as_bytes()] {
            challenge_preimage.extend_from_slice(&(bytes.len() as u64).to_be_bytes());
            challenge_preimage.extend_from_slice(bytes);
        }
        let challenge = <[u8; 32]>::from(Sha256::digest(challenge_preimage));
        let request = encode_get_assertion_request(challenge, &selected).map_err(|_| {
            kernel_error!(
                ContextResolutionKernelErrorKind::AuthorityRefused,
                "authority assertion request could not be encoded",
            )
        })?;
        let response = authenticator.get_assertion(&request).map_err(|_| {
            kernel_error!(
                ContextResolutionKernelErrorKind::AuthorityRefused,
                "native authenticator refused or was unavailable",
            )
        })?;
        let assertion =
            decode_and_verify_get_assertion_response(&response, &requested, challenge).map_err(
                |_| {
                    kernel_error!(
                        ContextResolutionKernelErrorKind::AuthorityRefused,
                        "native authority assertion verification failed",
                    )
                },
            )?;
        let retained = current_credentials
            .iter()
            .find(|credential| credential.credential_id == assertion.credential_id)
            .expect("verified assertion selected one requested credential")
            .sign_count;
        let previous = self
            .counters
            .get(&assertion.credential_id)
            .copied()
            .unwrap_or(retained);
        if (retained != 0 || assertion.sign_count != 0)
            && (assertion.sign_count <= retained || assertion.sign_count <= previous)
        {
            return Err(kernel_error!(
                ContextResolutionKernelErrorKind::AuthorityRefused,
                "authenticator sign counter did not increase",
            ));
        }
        self.counters
            .insert(assertion.credential_id, assertion.sign_count);
        let admission = ContextResolutionAuthorityAdmission {
            authority_use,
            subject: subject.clone(),
            binding_fingerprint: string_at!(&current_binding, "/binding_fingerprint", "authority binding fingerprint is unavailable").to_owned(),
            authority: current_binding,
        };
        self.cache.insert(key, admission.clone());
        Ok(admission)
    }
}

#[rustfmt::skip]
fn load_authority_snapshot(repo_root: &Path, binding_kind_ref: &str, binding_instance_id: &str, expected_artifact_fingerprint: &str) -> Result<(Arc<Value>, String, Vec<CommittedApproverCredentialV1>), ContextResolutionKernelError> {
    let authority = ArtifactRepositoryAuthorityGuardV1::acquire(repo_root).map_err(|_| {
        kernel_error!(
            ContextResolutionKernelErrorKind::AuthorityRefused,
            "repository authority guard refused",
        )
    })?;
    let store = GenericArtifactLineageStoreV1::new(repo_root);
    store.evaluate_committed_read_with(
        authority.repository_identity_fingerprint().as_str(),
        crate::artifact_mutation::HCM_2_3_OWNER_SUBJECT_FINGERPRINT,
        |_| {
            kernel_error!(
                ContextResolutionKernelErrorKind::StaleAuthority,
                "generic artifact recovery refused authority binding read",
            )
        },
        |intent| {
            crate::artifact_mutation::validate_persisted_intent_authority(
                repo_root,
                &authority,
                intent,
            )
        },
        |intent, ordinal, final_ordinal, bytes| {
            crate::artifact_mutation::validate_persisted_output_authority(
                repo_root,
                &authority,
                intent,
                ordinal,
                final_ordinal,
                bytes,
            )
        },
        || {
            authority.require_current_identity().map_err(|_| {
                kernel_error!(
                    ContextResolutionKernelErrorKind::StaleAuthority,
                    "repository authority changed during binding read",
                )
            })?;
            let repository = ArtifactRepositoryV1::open_under_authority(repo_root, &authority)
                .map_err(|_| {
                    kernel_error!(
                        ContextResolutionKernelErrorKind::AuthorityRefused,
                        "artifact repository could not resolve authority binding",
                    )
                })?;
            let kind = ExactDefinitionRef::parse(binding_kind_ref).map_err(|_| {
                kernel_error!(
                    ContextResolutionKernelErrorKind::InvalidInput,
                    "authority binding kind ref is invalid",
                )
            })?;
            let instance = SymbolicId::parse(binding_instance_id).map_err(|_| {
                kernel_error!(
                    ContextResolutionKernelErrorKind::InvalidInput,
                    "authority binding instance ID is invalid",
                )
            })?;
            let operation = repository
                .operation_context_under_lock(&kind, &instance)
                .map_err(|_| {
                    kernel_error!(
                        ContextResolutionKernelErrorKind::AuthorityRefused,
                        "authority binding operation context is unavailable",
                    )
                })?;
            let artifact = repository.read_under_lock(&kind, &instance).map_err(|_| {
                kernel_error!(
                    ContextResolutionKernelErrorKind::AuthorityRefused,
                    "authority binding artifact read was refused",
                )
            })?;
            if artifact.artifact_fingerprint.as_str() != expected_artifact_fingerprint {
                return Err(kernel_error!(
                    ContextResolutionKernelErrorKind::StaleAuthority,
                    "authority binding raw artifact fingerprint mismatch",
                ));
            }
            let binding = artifact.content;
            closed_object!(&binding, ["schema_id", "schema_version", "binding_id", "binding_version", "repository_identity_fingerprint", "resolved_profile", "resolution_stack", "approver_registry", "authority_mappings", "candidate_mappings", "extensions", "binding_fingerprint"], "authority binding is not the exact closed record");
            for pointer in ["/resolved_profile", "/resolution_stack"] {
                closed_object!(binding.pointer(pointer).unwrap_or(&Value::Null), ["ref", "fingerprint"], "authority binding exact reference is malformed");
                ContextResolutionExactBinding::new(string_at!(&binding, &format!("{pointer}/ref"), "authority binding exact reference is malformed"), string_at!(&binding, &format!("{pointer}/fingerprint"), "authority binding exact reference is malformed"))?;
            }
            closed_object!(binding.pointer("/approver_registry").unwrap_or(&Value::Null), ["state_ref", "state_fingerprint", "head_transition_ref", "head_transition_fingerprint"], "authority registry binding is malformed");
            for pointer in ["/approver_registry/state_fingerprint", "/approver_registry/head_transition_fingerprint"] {
                DefinitionFingerprint::parse(string_at!(&binding, pointer, "authority registry fingerprint is malformed")).map_err(|_| kernel_error!(ContextResolutionKernelErrorKind::InvalidInput, "authority registry fingerprint is malformed"))?;
            }
            let uses = [ContextResolutionAuthorityUse::RootCreation, ContextResolutionAuthorityUse::Parent, ContextResolutionAuthorityUse::Requested, ContextResolutionAuthorityUse::Approving, ContextResolutionAuthorityUse::Decision, ContextResolutionAuthorityUse::Evidence, ContextResolutionAuthorityUse::TriggerCondition, ContextResolutionAuthorityUse::Constraint, ContextResolutionAuthorityUse::Source, ContextResolutionAuthorityUse::Target, ContextResolutionAuthorityUse::TargetMemory];
            closed_object!(binding.pointer("/authority_mappings").unwrap_or(&Value::Null), uses.map(ContextResolutionAuthorityUse::as_str), "authority mappings are not exact");
            for authority_use in uses {
                let pointer = format!("/authority_mappings/{}", authority_use.as_str());
                closed_object!(binding.pointer(&pointer).unwrap_or(&Value::Null), ["approval_class", "authority_ref"], "authority mapping is malformed");
                string_at!(&binding, &format!("{pointer}/approval_class"), "authority mapping is incomplete");
                string_at!(&binding, &format!("{pointer}/authority_ref"), "authority mapping is incomplete");
            }
            let candidate_contracts = [("dimension_rank_increase", "one_or_more_dimension_ranks_exceed_parent", "current_and_proposed_envelopes"), ("mutation_allow_expansion", "child_allow_not_provably_contained", "parent_and_child_mutation_rules"), ("missing_context", "required_context_record_absent", "available_context_and_missing_ref"), ("missing_authority", "required_authority_mapping_absent", "current_authority_and_requested_pair")];
            closed_object!(binding.pointer("/candidate_mappings").unwrap_or(&Value::Null), candidate_contracts.map(|entry| entry.0), "candidate mappings are not exact");
            for (class, condition, evidence) in candidate_contracts {
                let pointer = format!("/candidate_mappings/{class}");
                closed_object!(binding.pointer(&pointer).unwrap_or(&Value::Null), ["trigger", "missing_condition", "evidence_requirement"], "candidate mapping is malformed");
                closed_object!(binding.pointer(&format!("{pointer}/trigger")).unwrap_or(&Value::Null), ["ref", "fingerprint"], "candidate trigger is malformed");
                ContextResolutionExactBinding::new(string_at!(&binding, &format!("{pointer}/trigger/ref"), "candidate trigger is malformed"), string_at!(&binding, &format!("{pointer}/trigger/fingerprint"), "candidate trigger is malformed"))?;
                if string_at!(&binding, &format!("{pointer}/missing_condition"), "candidate mapping is malformed") != condition || string_at!(&binding, &format!("{pointer}/evidence_requirement"), "candidate mapping is malformed") != evidence { return Err(kernel_error!(ContextResolutionKernelErrorKind::InvalidInput, "candidate mapping differs from the selected closed contract")); }
            }
            if string_at!(&binding, "/schema_id", "authority schema is malformed") != "handbook.context-resolution-authority-binding" || string_at!(&binding, "/schema_version", "authority schema is malformed") != "1.0" || string_at!(&binding, "/binding_id", "authority binding identity is malformed").len() > 128 || string_at!(&binding, "/binding_version", "authority binding identity is malformed").len() > 128 || !binding.pointer("/extensions").and_then(Value::as_object).is_some_and(serde_json::Map::is_empty) { return Err(kernel_error!(ContextResolutionKernelErrorKind::InvalidInput, "unsupported context resolution authority binding")); }
            DefinitionFingerprint::parse(string_at!(&binding, "/repository_identity_fingerprint", "binding repository identity fingerprint is invalid")).map_err(|_| kernel_error!(ContextResolutionKernelErrorKind::InvalidInput, "binding repository identity fingerprint is invalid"))?;
            let supplied_text = string_at!(&binding, "/binding_fingerprint", "authority semantic fingerprint is invalid");
            let supplied = DefinitionFingerprint::parse(supplied_text).map_err(|_| {
                kernel_error!(
                    ContextResolutionKernelErrorKind::InvalidInput,
                    "authority semantic fingerprint is invalid",
                )
            })?;
            let mut semantic = binding.clone();
            semantic.as_object_mut().expect("closed object").remove("binding_fingerprint");
            let computed = fingerprint_serializable(&semantic).map_err(|_| {
                kernel_error!(
                    ContextResolutionKernelErrorKind::InvalidInput,
                    "authority binding could not be fingerprinted",
                )
            })?;
            if supplied != computed {
                return Err(kernel_error!(
                    ContextResolutionKernelErrorKind::StaleAuthority,
                    "authority semantic fingerprint mismatch",
                ));
            }
            let mut budget = SourceByteBudget::default();
            let (_, profile_bytes) = read_trusted_repo_source(repo_root, REPOSITORY_PROFILE_SELECTION_PATH, &mut budget).map_err(|_| kernel_error!(ContextResolutionKernelErrorKind::StaleAuthority, "repository profile selection is unavailable"))?;
            let selection = RepositoryProfileSelectionV1::from_json_bytes(&profile_bytes).map_err(|_| kernel_error!(ContextResolutionKernelErrorKind::InvalidInput, "repository profile selection is invalid"))?;
            let profile = resolve_profile_selection(repo_root, selection.profile_request()).map_err(|_| kernel_error!(ContextResolutionKernelErrorKind::StaleAuthority, "selected repository profile did not resolve"))?;
            if string_at!(&binding, "/repository_identity_fingerprint", "authority binding is malformed") != authority.repository_identity_fingerprint().as_str()
                || string_at!(&binding, "/resolved_profile/ref", "authority binding is malformed") != operation.profile_ref().as_str()
                || string_at!(&binding, "/resolved_profile/fingerprint", "authority binding is malformed") != operation.resolved_profile_fingerprint().as_str()
                || string_at!(&binding, "/resolved_profile/ref", "authority binding is malformed") != profile.exact_ref().as_str()
                || string_at!(&binding, "/resolved_profile/fingerprint", "authority binding is malformed") != profile.resolved_profile_fingerprint().as_str()
                || string_at!(&binding, "/resolution_stack/ref", "authority binding is malformed") != profile.context_resolution().exact_ref().as_str()
                || string_at!(&binding, "/resolution_stack/fingerprint", "authority binding is malformed") != profile.context_resolution().definition_fingerprint().as_str()
            {
                return Err(kernel_error!(
                    ContextResolutionKernelErrorKind::StaleAuthority,
                    "authority binding repository, profile, or stack is stale",
                ));
            }
            let registry = observe_committed_approver_registry_locked(repo_root).map_err(|_| {
                kernel_error!(
                    ContextResolutionKernelErrorKind::StaleAuthority,
                    "committed approver registry could not be observed",
                )
            })?;
            if string_at!(&binding, "/approver_registry/state_ref", "authority registry binding is malformed") != registry.state_ref
                || string_at!(&binding, "/approver_registry/state_fingerprint", "authority registry binding is malformed") != registry.state_fingerprint
                || string_at!(&binding, "/approver_registry/head_transition_ref", "authority registry binding is malformed") != registry.head_transition_ref
                || string_at!(&binding, "/approver_registry/head_transition_fingerprint", "authority registry binding is malformed") != registry.head_transition_fingerprint
            {
                return Err(kernel_error!(
                    ContextResolutionKernelErrorKind::StaleAuthority,
                    "authority binding approver registry head is stale",
                ));
            }
            Ok((Arc::new(binding), artifact.artifact_fingerprint.to_string(), registry.credentials))
        },
    )
}

#[rustfmt::skip]
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct ContextResolutionDimensions { values: [String; 6] }

impl ContextResolutionDimensions {
    pub fn new(values: [&str; 6]) -> Result<Self, ContextResolutionKernelError> {
        if values.iter().any(|value| {
            value.is_empty()
                || value.len() > 128
                || value
                    .bytes()
                    .any(|byte| byte.is_ascii_control() || byte.is_ascii_whitespace())
        }) {
            return Err(kernel_error!(
                ContextResolutionKernelErrorKind::InvalidInput,
                "context resolution dimension value is invalid",
            ));
        }
        Ok(Self {
            values: values.map(str::to_owned),
        })
    }

    #[rustfmt::skip]
    pub fn values(&self) -> [&str; 6] { self.values.each_ref().map(String::as_str) }
}

#[rustfmt::skip]
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ContextResolutionMutationEffect { Allow, Deny }

#[rustfmt::skip]
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct ContextResolutionMutationRule { rule_id: String, effect: ContextResolutionMutationEffect, target_kind: String, selector: String }

impl ContextResolutionMutationRule {
    pub fn new(
        rule_id: &str,
        effect: ContextResolutionMutationEffect,
        target_kind: &str,
        selector: &str,
    ) -> Result<Self, ContextResolutionKernelError> {
        if rule_id.is_empty()
            || rule_id.len() > 128
            || !rule_id
                .bytes()
                .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'.' | b'_' | b'-'))
        {
            return Err(kernel_error!(
                ContextResolutionKernelErrorKind::InvalidInput,
                "mutation rule ID is invalid",
            ));
        }
        if target_kind != "repository_path" {
            return Err(kernel_error!(
                ContextResolutionKernelErrorKind::Indeterminate,
                "mutation target kind is not admitted",
            ));
        }
        validate_selector!(selector)?;
        Ok(Self {
            rule_id: rule_id.to_owned(),
            effect,
            target_kind: target_kind.to_owned(),
            selector: selector.to_owned(),
        })
    }
}

#[rustfmt::skip]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ContextResolutionMutationDecision { Allowed, Denied, Indeterminate }

#[rustfmt::skip]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ContextResolutionMemoryDecision { Authorized, PromotionRequired }

#[rustfmt::skip]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ContextResolutionValidationDecision { Authorized, NotAuthorized }

#[rustfmt::skip]
#[derive(Clone, Debug, Serialize)]
pub struct ContextResolutionEnvelopeInput { envelope_id: String, objective_ref: String, resolved_profile: ContextResolutionExactBinding, resolution_stack: ContextResolutionExactBinding, active_level_id: String, dimensions: ContextResolutionDimensions, parent: Option<ContextResolutionExactBinding>, constraint_inputs: Vec<ContextResolutionExactBinding>, mutation_rules: Vec<ContextResolutionMutationRule>, escalation_triggers: Vec<ContextResolutionExactBinding>, #[serde(skip)] candidate_binding: ContextResolutionExactBinding }

impl ContextResolutionEnvelopeInput {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        envelope_id: &str,
        objective_ref: &str,
        resolved_profile: ContextResolutionExactBinding,
        resolution_stack: ContextResolutionExactBinding,
        active_level_id: &str,
        dimensions: ContextResolutionDimensions,
        parent: Option<ContextResolutionExactBinding>,
        constraint_inputs: Vec<ContextResolutionExactBinding>,
        mutation_rules: Vec<ContextResolutionMutationRule>,
        escalation_triggers: Vec<ContextResolutionExactBinding>,
    ) -> Result<Self, ContextResolutionKernelError> {
        for (value, label) in [
            (envelope_id, "envelope ID"),
            (objective_ref, "objective ref"),
            (active_level_id, "active level ID"),
        ] {
            if value.is_empty()
                || value.len() > 256
                || value
                    .bytes()
                    .any(|byte| byte.is_ascii_control() || byte.is_ascii_whitespace())
            {
                return Err(kernel_error!(
                    ContextResolutionKernelErrorKind::InvalidInput,
                    format!("{label} is invalid"),
                ));
            }
        }
        require_sorted_unique!(&constraint_inputs, "constraint inputs")?;
        if mutation_rules.len() > 256 || escalation_triggers.len() > 64 {
            return Err(kernel_error!(
                ContextResolutionKernelErrorKind::InvalidInput,
                "envelope collection exceeds its exact bound",
            ));
        }
        let mut rule_ids = BTreeSet::new();
        if mutation_rules
            .iter()
            .any(|rule| !rule_ids.insert(rule.rule_id.as_str()))
        {
            return Err(kernel_error!(
                ContextResolutionKernelErrorKind::InvalidInput,
                "mutation rule ID is duplicated",
            ));
        }
        require_sorted_unique!(&escalation_triggers, "escalation triggers")?;
        let candidate_value = json!({
            "envelope_id": envelope_id,
            "objective_ref": objective_ref,
            "resolved_profile": &resolved_profile,
            "resolution_stack": &resolution_stack,
            "active_level_id": active_level_id,
            "dimensions": &dimensions,
            "parent": &parent,
            "constraint_inputs": &constraint_inputs,
            "mutation_rules": &mutation_rules,
            "escalation_triggers": &escalation_triggers,
        });
        let candidate_fingerprint = DefinitionFingerprint::from_json_value(&candidate_value)
            .map_err(|_| {
                kernel_error!(
                    ContextResolutionKernelErrorKind::InvalidInput,
                    "envelope candidate could not be fingerprinted",
                )
            })?;
        let candidate_binding = ContextResolutionExactBinding::new(
            &format!("context-resolution-envelope-candidate/{envelope_id}"),
            candidate_fingerprint.as_str(),
        )?;
        Ok(Self {
            envelope_id: envelope_id.to_owned(),
            objective_ref: objective_ref.to_owned(),
            resolved_profile,
            resolution_stack,
            active_level_id: active_level_id.to_owned(),
            dimensions,
            parent,
            constraint_inputs,
            mutation_rules,
            escalation_triggers,
            candidate_binding,
        })
    }

    #[rustfmt::skip]
    pub fn candidate_binding(&self) -> &ContextResolutionExactBinding { &self.candidate_binding }
}

#[rustfmt::skip]
#[derive(Clone, Debug)]
pub struct ContextResolutionEnvelope { exact_binding: ContextResolutionExactBinding, _objective_ref: String, resolved_profile: ContextResolutionExactBinding, resolution_stack: ContextResolutionExactBinding, active_level_id: String, dimensions: ContextResolutionDimensions, _constraint_inputs: Vec<ContextResolutionExactBinding>, mutation_allow_layers: Vec<Vec<ContextResolutionMutationRule>>, mutation_denies: Vec<ContextResolutionMutationRule>, _escalation_triggers: Vec<ContextResolutionExactBinding>, authority: Arc<Value>, stack: ContextResolutionStackDefinition }

#[rustfmt::skip]
impl ContextResolutionEnvelope {
    pub fn resolve_root(
        profile: &ResolvedInstanceProfile,
        stack: &ContextResolutionStackDefinition,
        input: ContextResolutionEnvelopeInput,
        creator: &ContextResolutionAuthorityAdmission,
        constraints: &[ContextResolutionAuthorityAdmission],
    ) -> Result<Self, ContextResolutionKernelError> {
        validate_profile_stack!(profile, stack, &input)?;
        if input.parent.is_some() {
            return Err(kernel_error!(
                ContextResolutionKernelErrorKind::InvalidInput,
                "root envelope cannot cite a parent",
            ));
        }
        require_admission!(
            creator,
            ContextResolutionAuthorityUse::RootCreation,
            &input.candidate_binding,
            None,
        )?;
        require_admission_list!(
            constraints,
            ContextResolutionAuthorityUse::Constraint,
            &input.constraint_inputs,
            Some(creator.binding_fingerprint()),
        )?;
        validate_envelope_values!(stack, &input)?;
        materialize_envelope!(input, creator.authority.clone(), stack.clone(), Vec::new())
    }

    pub fn resolve_child(
        profile: &ResolvedInstanceProfile,
        stack: &ContextResolutionStackDefinition,
        parent: &Self,
        input: ContextResolutionEnvelopeInput,
        parent_authority: &ContextResolutionAuthorityAdmission,
        constraints: &[ContextResolutionAuthorityAdmission],
    ) -> Result<Self, ContextResolutionKernelError> {
        validate_profile_stack!(profile, stack, &input)?;
        if input.parent.as_ref() != Some(&parent.exact_binding)
            || parent.resolved_profile != input.resolved_profile
            || parent.resolution_stack != input.resolution_stack
        {
            return Err(kernel_error!(
                ContextResolutionKernelErrorKind::StaleAuthority,
                "child envelope parent, profile, or stack binding is stale",
            ));
        }
        require_admission!(
            parent_authority,
            ContextResolutionAuthorityUse::Parent,
            &input.candidate_binding,
            Some(string_at!(&parent.authority, "/binding_fingerprint", "authority binding is malformed")),
        )?;
        require_admission_list!(
            constraints,
            ContextResolutionAuthorityUse::Constraint,
            &input.constraint_inputs,
            Some(parent_authority.binding_fingerprint()),
        )?;
        validate_envelope_values!(stack, &input)?;
        let input_ranks = stack.ranks(&input.active_level_id, input.dimensions.values()).expect("input dimensions were validated");
        let parent_ranks = stack.ranks(&parent.active_level_id, parent.dimensions.values()).expect("parent dimensions were validated");
        let increases = (0..6)
            .filter(|index| input_ranks[*index] > parent_ranks[*index])
            .collect::<Vec<_>>();
        if !increases.is_empty() {
            let candidate = candidate_from_mapping!(
                "dimension_rank_increase",
                &parent.exact_binding,
                &input.candidate_binding,
                Vec::new(),
                &parent.authority,
                &parent.resolved_profile,
                &parent.resolution_stack,
            )?;
            return Err(kernel_error!(
                ContextResolutionKernelErrorKind::ExpansionRefused,
                "child envelope increases one or more dimension ranks",
                candidate,
            ));
        }
        let child_allows = input
            .mutation_rules
            .iter()
            .filter(|rule| rule.effect == ContextResolutionMutationEffect::Allow)
            .cloned()
            .collect::<Vec<_>>();
        if child_allows.iter().any(|child| {
            parent.mutation_allow_layers.iter().any(|layer| {
                !layer
                    .iter()
                    .any(|parent_allow| selector_contains!(&parent_allow.selector, &child.selector))
            })
        }) {
            let evidence = parent
                .mutation_allow_layers
                .iter()
                .flatten()
                .chain(input.mutation_rules.iter())
                .map(|rule| rule_binding!(rule))
                .collect::<Result<Vec<_>, _>>()?;
            let candidate = candidate_from_mapping!(
                "mutation_allow_expansion",
                &parent.exact_binding,
                &input.candidate_binding,
                evidence,
                &parent.authority,
                &parent.resolved_profile,
                &parent.resolution_stack,
            )?;
            return Err(kernel_error!(
                ContextResolutionKernelErrorKind::ExpansionRefused,
                "child mutation allow is not provably contained",
                candidate,
            ));
        }
        materialize_envelope!(
            input,
            parent.authority.clone(),
            stack.clone(),
            parent.mutation_allow_layers.clone(),
        )
        .map(|mut child| {
            child
                .mutation_denies
                .splice(0..0, parent.mutation_denies.iter().cloned());
            child
        })
    }

    #[rustfmt::skip]
    pub fn exact_binding(&self) -> &ContextResolutionExactBinding { &self.exact_binding }
    #[rustfmt::skip]
    pub fn dimensions(&self) -> &ContextResolutionDimensions { &self.dimensions }

    pub fn evaluate_mutation(
        &self,
        target_kind: &str,
        target: &str,
    ) -> ContextResolutionMutationDecision {
        if target_kind != "repository_path" || validate_target!(target).is_err() {
            return ContextResolutionMutationDecision::Indeterminate;
        }
        if self
            .mutation_denies
            .iter()
            .any(|rule| selector_matches!(&rule.selector, target))
        {
            return ContextResolutionMutationDecision::Denied;
        }
        if self.mutation_allow_layers.iter().all(|layer| {
            !layer.is_empty()
                && layer
                    .iter()
                    .any(|rule| selector_matches!(&rule.selector, target))
        }) {
            ContextResolutionMutationDecision::Allowed
        } else {
            ContextResolutionMutationDecision::Denied
        }
    }

    pub fn authorize_memory(
        &self,
        requested_value: &str,
    ) -> Result<ContextResolutionMemoryDecision, ContextResolutionKernelError> {
        let mut values = self.dimensions.values();
        values[4] = requested_value;
        let requested = self.stack.ranks(&self.active_level_id, values).map(|ranks| ranks[4]).ok_or_else(|| {
            kernel_error!(
                ContextResolutionKernelErrorKind::InvalidInput,
                "requested memory horizon is unknown",
            )
        })?;
        let available = self.stack.ranks(&self.active_level_id, self.dimensions.values()).expect("resolved envelope memory value remains in its stack")[4];
        Ok(if requested <= available {
            ContextResolutionMemoryDecision::Authorized
        } else {
            ContextResolutionMemoryDecision::PromotionRequired
        })
    }

    pub fn authorize_validation(
        &self,
        requested_value: &str,
    ) -> Result<ContextResolutionValidationDecision, ContextResolutionKernelError> {
        let mut values = self.dimensions.values();
        values[5] = requested_value;
        let requested = self.stack.ranks(&self.active_level_id, values).map(|ranks| ranks[5]).ok_or_else(|| {
            kernel_error!(
                ContextResolutionKernelErrorKind::InvalidInput,
                "requested validation horizon is unknown",
            )
        })?;
        let available = self.stack.ranks(&self.active_level_id, self.dimensions.values()).expect("resolved envelope validation value remains in its stack")[5];
        Ok(if requested <= available {
            ContextResolutionValidationDecision::Authorized
        } else {
            ContextResolutionValidationDecision::NotAuthorized
        })
    }

    pub fn missing_context_candidate(
        &self,
        proposed: &ContextResolutionEnvelope,
        evidence: Vec<ContextResolutionExactBinding>,
    ) -> Result<ContextResolutionEscalationCandidate, ContextResolutionKernelError> {
        typed_candidate!(self, "missing_context", proposed, evidence)
    }

    pub fn missing_authority_candidate(
        &self,
        proposed: &ContextResolutionEnvelope,
        evidence: Vec<ContextResolutionExactBinding>,
    ) -> Result<ContextResolutionEscalationCandidate, ContextResolutionKernelError> {
        typed_candidate!(self, "missing_authority", proposed, evidence)
    }
}

#[rustfmt::skip]
#[derive(Clone, Debug)]
pub struct ContextResolutionEscalationCandidate { class: String, trigger: ContextResolutionExactBinding, missing_condition: String, requested_authority_ref: String, evidence: Vec<ContextResolutionExactBinding>, current: ContextResolutionExactBinding, proposed: ContextResolutionExactBinding, resolved_profile: ContextResolutionExactBinding, resolution_stack: ContextResolutionExactBinding, authority: Arc<Value> }

impl ContextResolutionEscalationCandidate {
    #[rustfmt::skip]
    pub fn class(&self) -> &str { &self.class }
    #[rustfmt::skip]
    pub fn trigger(&self) -> &ContextResolutionExactBinding { &self.trigger }
    #[rustfmt::skip]
    pub fn missing_condition(&self) -> &str { &self.missing_condition }
    #[rustfmt::skip]
    pub fn requested_authority_ref(&self) -> &str { &self.requested_authority_ref }
    #[rustfmt::skip]
    pub fn evidence(&self) -> &[ContextResolutionExactBinding] { &self.evidence }
}

#[rustfmt::skip]
#[derive(Debug)]
pub struct ContextResolutionEscalationRequest { request_id: String, exact_binding: ContextResolutionExactBinding, candidate: ContextResolutionEscalationCandidate }

#[rustfmt::skip]
impl ContextResolutionEscalationRequest {
    pub fn new(
        request_id: &str,
        current_envelope: &ContextResolutionEnvelope,
        candidate: ContextResolutionEscalationCandidate,
    ) -> Result<Self, ContextResolutionKernelError> {
        validate_record_id!(request_id, "escalation request ID")?;
        if candidate.current != current_envelope.exact_binding
            || candidate.resolved_profile != current_envelope.resolved_profile
            || candidate.resolution_stack != current_envelope.resolution_stack
            || candidate.current == candidate.proposed
            || string_at!(&candidate.authority, "/binding_fingerprint", "authority binding is malformed")
                != string_at!(&current_envelope.authority, "/binding_fingerprint", "authority binding is malformed")
        {
            return Err(kernel_error!(
                ContextResolutionKernelErrorKind::ReplayMismatch,
                "escalation candidate does not bind the supplied current envelope",
            ));
        }
        let fingerprint = DefinitionFingerprint::from_json_value(&json!({
            "request_id": request_id,
            "current": &candidate.current,
            "proposed": &candidate.proposed,
            "class": &candidate.class,
            "trigger": &candidate.trigger,
            "missing_condition": &candidate.missing_condition,
            "requested_authority_ref": &candidate.requested_authority_ref,
            "evidence": &candidate.evidence,
            "resolved_profile": &candidate.resolved_profile,
            "resolution_stack": &candidate.resolution_stack,
        }))
        .map_err(|_| invalid_fingerprint!("escalation request"))?;
        let exact_binding = ContextResolutionExactBinding::new(
            &format!("context-resolution-escalation-request/{request_id}"),
            fingerprint.as_str(),
        )?;
        Ok(Self {
            request_id: request_id.to_owned(),
            exact_binding,
            candidate,
        })
    }

    #[rustfmt::skip]
    pub fn exact_binding(&self) -> &ContextResolutionExactBinding { &self.exact_binding }
}

#[rustfmt::skip]
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ContextResolutionEscalationOutcome { Approved, Refused, Superseded }

#[rustfmt::skip]
#[derive(Debug)]
pub struct ContextResolutionEscalationDisposition { disposition_id: String, exact_binding: ContextResolutionExactBinding, request: ContextResolutionExactBinding, outcome: ContextResolutionEscalationOutcome, decision: ContextResolutionExactBinding, authorized_envelope: Option<ContextResolutionExactBinding>, superseding_request: Option<ContextResolutionExactBinding> }

impl ContextResolutionEscalationDisposition {
    pub fn new(
        disposition_id: &str,
        request: ContextResolutionExactBinding,
        outcome: ContextResolutionEscalationOutcome,
        decision: ContextResolutionExactBinding,
        authorized_envelope: Option<ContextResolutionExactBinding>,
        superseding_request: Option<ContextResolutionExactBinding>,
    ) -> Result<Self, ContextResolutionKernelError> {
        validate_record_id!(disposition_id, "escalation disposition ID")?;
        let valid = match outcome {
            ContextResolutionEscalationOutcome::Approved => {
                authorized_envelope.is_some() && superseding_request.is_none()
            }
            ContextResolutionEscalationOutcome::Refused => {
                authorized_envelope.is_none() && superseding_request.is_none()
            }
            ContextResolutionEscalationOutcome::Superseded => {
                authorized_envelope.is_none() && superseding_request.is_some()
            }
        };
        if !valid {
            return Err(kernel_error!(
                ContextResolutionKernelErrorKind::InvalidInput,
                "escalation disposition outcome fields are inconsistent",
            ));
        }
        let fingerprint = DefinitionFingerprint::from_json_value(&json!({
            "disposition_id": disposition_id,
            "request": &request,
            "outcome": outcome,
            "decision": &decision,
            "authorized_envelope": &authorized_envelope,
            "superseding_request": &superseding_request,
        }))
        .map_err(|_| invalid_fingerprint!("escalation disposition"))?;
        let exact_binding = ContextResolutionExactBinding::new(
            &format!("context-resolution-escalation-disposition/{disposition_id}"),
            fingerprint.as_str(),
        )?;
        Ok(Self {
            disposition_id: disposition_id.to_owned(),
            exact_binding,
            request,
            outcome,
            decision,
            authorized_envelope,
            superseding_request,
        })
    }

    #[rustfmt::skip]
    pub fn exact_binding(&self) -> &ContextResolutionExactBinding { &self.exact_binding }
}

#[rustfmt::skip]
#[derive(Clone, Debug)]
pub struct ContextResolutionSemanticMemoryTarget { target_record_ref: String, expected_target_fingerprint: Option<String>, candidate_binding: ContextResolutionExactBinding }

impl ContextResolutionSemanticMemoryTarget {
    pub fn new(
        target_record_ref: &str,
        expected_target_fingerprint: Option<&str>,
    ) -> Result<Self, ContextResolutionKernelError> {
        if !semantic_memory_ref!(target_record_ref) {
            return Err(kernel_error!(
                ContextResolutionKernelErrorKind::ForbiddenTarget,
                "promotion target is not a typed semantic-memory record",
            ));
        }
        if let Some(fingerprint) = expected_target_fingerprint {
            DefinitionFingerprint::parse(fingerprint).map_err(|_| {
                kernel_error!(
                    ContextResolutionKernelErrorKind::InvalidInput,
                    "expected semantic-memory target fingerprint is invalid",
                )
            })?;
        }
        let fingerprint = DefinitionFingerprint::from_json_value(&json!({
            "target_record_ref": target_record_ref,
            "expected_target_fingerprint": expected_target_fingerprint,
        }))
        .map_err(|_| invalid_fingerprint!("semantic-memory target"))?;
        let candidate_binding = ContextResolutionExactBinding::new(
            &format!("context-resolution-semantic-memory-target/{target_record_ref}"),
            fingerprint.as_str(),
        )?;
        Ok(Self {
            target_record_ref: target_record_ref.to_owned(),
            expected_target_fingerprint: expected_target_fingerprint.map(str::to_owned),
            candidate_binding,
        })
    }

    #[rustfmt::skip]
    pub fn candidate_binding(&self) -> &ContextResolutionExactBinding { &self.candidate_binding }
}

#[rustfmt::skip]
#[derive(Debug)]
pub struct ContextResolutionSemanticMemoryRecord { record: ContextResolutionExactBinding }

impl ContextResolutionSemanticMemoryRecord {
    pub fn new(
        record: ContextResolutionExactBinding,
        target_memory_authority: &ContextResolutionAuthorityAdmission,
    ) -> Result<Self, ContextResolutionKernelError> {
        if !semantic_memory_ref!(record.reference()) {
            return Err(kernel_error!(
                ContextResolutionKernelErrorKind::ForbiddenTarget,
                "result record is not typed semantic memory",
            ));
        }
        require_admission!(
            target_memory_authority,
            ContextResolutionAuthorityUse::TargetMemory,
            &record,
            None,
        )?;
        Ok(Self { record })
    }

    #[rustfmt::skip]
    pub fn exact_binding(&self) -> &ContextResolutionExactBinding { &self.record }
}

#[rustfmt::skip]
#[derive(Debug)]
pub struct ContextResolutionPromotionRequest { request_id: String, exact_binding: ContextResolutionExactBinding, source_inputs: Vec<ContextResolutionExactBinding>, source_envelope: ContextResolutionExactBinding, _target_memory_horizon: String, target: ContextResolutionSemanticMemoryTarget, _requested_authority_ref: String, binding_fingerprint: String }

#[rustfmt::skip]
impl ContextResolutionPromotionRequest {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        request_id: &str,
        source_inputs: Vec<ContextResolutionExactBinding>,
        source_envelope: &ContextResolutionEnvelope,
        stack: &ContextResolutionStackDefinition,
        target_memory_horizon: &str,
        target: ContextResolutionSemanticMemoryTarget,
        target_authority: &ContextResolutionAuthorityAdmission,
        requested_authority_ref: &str,
    ) -> Result<Self, ContextResolutionKernelError> {
        validate_record_id!(request_id, "promotion request ID")?;
        if source_inputs.is_empty() {
            return Err(kernel_error!(
                ContextResolutionKernelErrorKind::InvalidInput,
                "promotion requires at least one exact source input",
            ));
        }
        require_sorted_unique!(&source_inputs, "promotion source inputs")?;
        if source_envelope.resolution_stack.reference != stack.exact_ref().as_str()
            || source_envelope.resolution_stack.fingerprint
                != stack.definition_fingerprint().as_str()
        {
            return Err(kernel_error!(
                ContextResolutionKernelErrorKind::StaleAuthority,
                "promotion source stack is stale",
            ));
        }
        let source_rank = stack.ranks(&source_envelope.active_level_id, source_envelope.dimensions.values()).map(|ranks| ranks[4]).ok_or_else(|| kernel_error!(ContextResolutionKernelErrorKind::StaleAuthority, "source memory horizon is stale"))?;
        let mut target_values = source_envelope.dimensions.values();
        target_values[4] = target_memory_horizon;
        let target_rank = stack.ranks(&source_envelope.active_level_id, target_values).map(|ranks| ranks[4]).ok_or_else(|| {
            kernel_error!(
                ContextResolutionKernelErrorKind::InvalidInput,
                "promotion target memory horizon is unknown",
            )
        })?;
        if target_rank <= source_rank {
            return Err(kernel_error!(
                ContextResolutionKernelErrorKind::ExpansionRefused,
                "promotion target memory horizon is not strictly higher",
            ));
        }
        require_admission!(
            target_authority,
            ContextResolutionAuthorityUse::Target,
            &target.candidate_binding,
            Some(string_at!(&source_envelope.authority, "/binding_fingerprint", "authority binding is malformed")),
        )?;
        if requested_authority_ref
            != string_at!(&source_envelope.authority, "/authority_mappings/requested/authority_ref", "requested authority mapping is unavailable")
        {
            return Err(kernel_error!(
                ContextResolutionKernelErrorKind::AuthorityRefused,
                "promotion requested authority does not match current binding",
            ));
        }
        let fingerprint = DefinitionFingerprint::from_json_value(&json!({
            "request_id": request_id,
            "source_inputs": &source_inputs,
            "source_envelope": &source_envelope.exact_binding,
            "target_memory_horizon": target_memory_horizon,
            "target_record_ref": &target.target_record_ref,
            "expected_target_fingerprint": &target.expected_target_fingerprint,
            "target_candidate": &target.candidate_binding,
            "requested_authority_ref": requested_authority_ref,
        }))
        .map_err(|_| invalid_fingerprint!("promotion request"))?;
        let exact_binding = ContextResolutionExactBinding::new(
            &format!("context-resolution-promotion-request/{request_id}"),
            fingerprint.as_str(),
        )?;
        Ok(Self {
            request_id: request_id.to_owned(),
            exact_binding,
            source_inputs,
            source_envelope: source_envelope.exact_binding.clone(),
            _target_memory_horizon: target_memory_horizon.to_owned(),
            target,
            _requested_authority_ref: requested_authority_ref.to_owned(),
            binding_fingerprint: string_at!(&source_envelope.authority, "/binding_fingerprint", "authority binding is malformed").to_owned(),
        })
    }

    #[rustfmt::skip]
    pub fn exact_binding(&self) -> &ContextResolutionExactBinding { &self.exact_binding }
}

#[rustfmt::skip]
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ContextResolutionPromotionOutcome { Applied, Refused, Stale }

#[rustfmt::skip]
#[derive(Debug)]
pub struct ContextResolutionPromotionDisposition { disposition_id: String, exact_binding: ContextResolutionExactBinding, request: ContextResolutionExactBinding, _outcome: ContextResolutionPromotionOutcome, decision: ContextResolutionExactBinding, validation_evidence: Vec<ContextResolutionExactBinding>, approving_authority_ref: String, result_record: Option<ContextResolutionSemanticMemoryRecord>, binding_fingerprint: String }

impl ContextResolutionPromotionDisposition {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        disposition_id: &str,
        request: &ContextResolutionPromotionRequest,
        outcome: ContextResolutionPromotionOutcome,
        decision: ContextResolutionExactBinding,
        validation_evidence: Vec<ContextResolutionExactBinding>,
        approving_authority_ref: &str,
        observed_target_fingerprint: Option<&str>,
        result_record: Option<ContextResolutionSemanticMemoryRecord>,
    ) -> Result<Self, ContextResolutionKernelError> {
        validate_record_id!(disposition_id, "promotion disposition ID")?;
        require_sorted_unique!(&validation_evidence, "promotion validation evidence")?;
        let current = request.target.expected_target_fingerprint.as_deref();
        let cas_matches = current == observed_target_fingerprint;
        let valid = match outcome {
            ContextResolutionPromotionOutcome::Applied => cas_matches && result_record.is_some(),
            ContextResolutionPromotionOutcome::Refused => result_record.is_none(),
            ContextResolutionPromotionOutcome::Stale => !cas_matches && result_record.is_none(),
        };
        if !valid {
            return Err(kernel_error!(
                ContextResolutionKernelErrorKind::InvalidInput,
                "promotion disposition violates outcome or compare-and-write semantics",
            ));
        }
        let fingerprint = DefinitionFingerprint::from_json_value(&json!({
            "disposition_id": disposition_id,
            "request": &request.exact_binding,
            "outcome": outcome,
            "decision": &decision,
            "validation_evidence": &validation_evidence,
            "approving_authority_ref": approving_authority_ref,
            "observed_target_fingerprint": observed_target_fingerprint,
            "result_record": result_record.as_ref().map(|record| record.exact_binding()),
        }))
        .map_err(|_| invalid_fingerprint!("promotion disposition"))?;
        let exact_binding = ContextResolutionExactBinding::new(
            &format!("context-resolution-promotion-disposition/{disposition_id}"),
            fingerprint.as_str(),
        )?;
        Ok(Self {
            disposition_id: disposition_id.to_owned(),
            exact_binding,
            request: request.exact_binding.clone(),
            _outcome: outcome,
            decision,
            validation_evidence,
            approving_authority_ref: approving_authority_ref.to_owned(),
            result_record,
            binding_fingerprint: request.binding_fingerprint.clone(),
        })
    }

    #[rustfmt::skip]
    pub fn exact_binding(&self) -> &ContextResolutionExactBinding { &self.exact_binding }
}

#[rustfmt::skip]
#[derive(Default)]
pub struct ContextResolutionTransitionRegistry { escalation_requests: BTreeMap<String, ContextResolutionEscalationRequest>, escalation_dispositions: BTreeMap<String, ContextResolutionExactBinding>, escalation_terminal_by_request: BTreeMap<String, String>, promotion_requests: BTreeMap<String, ContextResolutionPromotionRequest>, promotion_dispositions: BTreeMap<String, ContextResolutionExactBinding>, promotion_terminal_by_request: BTreeMap<String, String> }

#[rustfmt::skip]
impl ContextResolutionTransitionRegistry {
    #[rustfmt::skip]
    pub fn new() -> Self { Self::default() }

    pub fn admit_escalation_request(
        &mut self,
        request: ContextResolutionEscalationRequest,
        requested: &ContextResolutionAuthorityAdmission,
        trigger: &ContextResolutionAuthorityAdmission,
        evidence: &[ContextResolutionAuthorityAdmission],
    ) -> Result<bool, ContextResolutionKernelError> {
        require_admission!(
            requested,
            ContextResolutionAuthorityUse::Requested,
            &request.exact_binding,
            Some(string_at!(&request.candidate.authority, "/binding_fingerprint", "authority binding is malformed")),
        )?;
        require_admission!(
            trigger,
            ContextResolutionAuthorityUse::TriggerCondition,
            &request.candidate.trigger,
            Some(requested.binding_fingerprint()),
        )?;
        require_admission_list!(
            evidence,
            ContextResolutionAuthorityUse::Evidence,
            &request.candidate.evidence,
            Some(requested.binding_fingerprint()),
        )?;
        if let Some(existing) = self.escalation_requests.get(&request.request_id) {
            return if existing.exact_binding == request.exact_binding { Ok(false) } else { Err(kernel_error!(ContextResolutionKernelErrorKind::ReplayMismatch, "record ID was reused with changed bytes")) };
        }
        self.escalation_requests.insert(request.request_id.clone(), request);
        Ok(true)
    }

    pub fn admit_escalation_disposition(
        &mut self,
        disposition: ContextResolutionEscalationDisposition,
        approving: &ContextResolutionAuthorityAdmission,
        decision: &ContextResolutionAuthorityAdmission,
    ) -> Result<bool, ContextResolutionKernelError> {
        let request = self
            .escalation_requests
            .get(disposition.request.reference())
            .or_else(|| {
                self.escalation_requests
                    .values()
                    .find(|request| request.exact_binding == disposition.request)
            })
            .ok_or_else(|| stale_request!("escalation"))?;
        if request.exact_binding != disposition.request
            || (disposition.outcome == ContextResolutionEscalationOutcome::Approved
                && disposition.authorized_envelope.as_ref() != Some(&request.candidate.proposed))
        {
            return Err(stale_request!("escalation"));
        }
        if disposition.outcome == ContextResolutionEscalationOutcome::Superseded {
            let superseding = disposition
                .superseding_request
                .as_ref()
                .ok_or_else(|| stale_request!("escalation"))?;
            if superseding == &disposition.request
                || self
                    .escalation_requests
                    .values()
                    .all(|request| request.exact_binding != *superseding)
            {
                return Err(stale_request!("escalation"));
            }
        }
        require_admission!(
            approving,
            ContextResolutionAuthorityUse::Approving,
            &disposition.exact_binding,
            None,
        )?;
        require_admission!(
            decision,
            ContextResolutionAuthorityUse::Decision,
            &disposition.decision,
            Some(approving.binding_fingerprint()),
        )?;
        admit_terminal!(
            &mut self.escalation_dispositions,
            &mut self.escalation_terminal_by_request,
            disposition.disposition_id,
            disposition.exact_binding,
            disposition.request,
        )
    }

    pub fn admit_promotion_request(
        &mut self,
        request: ContextResolutionPromotionRequest,
        requested: &ContextResolutionAuthorityAdmission,
        sources: &[ContextResolutionAuthorityAdmission],
        target: &ContextResolutionAuthorityAdmission,
        target_memory: &ContextResolutionAuthorityAdmission,
    ) -> Result<bool, ContextResolutionKernelError> {
        require_admission!(
            requested,
            ContextResolutionAuthorityUse::Requested,
            &request.exact_binding,
            Some(&request.binding_fingerprint),
        )?;
        let mut source_subjects = request.source_inputs.clone();
        source_subjects.push(request.source_envelope.clone());
        require_admission_list!(
            sources,
            ContextResolutionAuthorityUse::Source,
            &source_subjects,
            Some(requested.binding_fingerprint()),
        )?;
        require_admission!(
            target,
            ContextResolutionAuthorityUse::Target,
            &request.target.candidate_binding,
            Some(requested.binding_fingerprint()),
        )?;
        require_admission!(
            target_memory,
            ContextResolutionAuthorityUse::TargetMemory,
            &request.exact_binding,
            Some(requested.binding_fingerprint()),
        )?;
        if let Some(existing) = self.promotion_requests.get(&request.request_id) {
            return if existing.exact_binding == request.exact_binding { Ok(false) } else { Err(kernel_error!(ContextResolutionKernelErrorKind::ReplayMismatch, "record ID was reused with changed bytes")) };
        }
        self.promotion_requests.insert(request.request_id.clone(), request);
        Ok(true)
    }

    pub fn admit_promotion_disposition(
        &mut self,
        disposition: ContextResolutionPromotionDisposition,
        approving: &ContextResolutionAuthorityAdmission,
        decision: &ContextResolutionAuthorityAdmission,
        evidence: &[ContextResolutionAuthorityAdmission],
        target_memory: &ContextResolutionAuthorityAdmission,
    ) -> Result<bool, ContextResolutionKernelError> {
        if self
            .promotion_requests
            .values()
            .all(|request| request.exact_binding != disposition.request)
        {
            return Err(stale_request!("promotion"));
        }
        require_admission!(
            approving,
            ContextResolutionAuthorityUse::Approving,
            &disposition.exact_binding,
            Some(&disposition.binding_fingerprint),
        )?;
        if disposition.approving_authority_ref
            != string_at!(
                &approving.authority,
                "/authority_mappings/approving/authority_ref",
                "approving authority mapping is unavailable"
            )
        {
            return Err(kernel_error!(
                ContextResolutionKernelErrorKind::AuthorityRefused,
                "promotion disposition approving authority does not match current binding",
            ));
        }
        require_admission!(
            decision,
            ContextResolutionAuthorityUse::Decision,
            &disposition.decision,
            Some(approving.binding_fingerprint()),
        )?;
        require_admission_list!(
            evidence,
            ContextResolutionAuthorityUse::Evidence,
            &disposition.validation_evidence,
            Some(approving.binding_fingerprint()),
        )?;
        let target_subject = disposition
            .result_record
            .as_ref()
            .map(ContextResolutionSemanticMemoryRecord::exact_binding)
            .unwrap_or(&disposition.request);
        require_admission!(
            target_memory,
            ContextResolutionAuthorityUse::TargetMemory,
            target_subject,
            Some(approving.binding_fingerprint()),
        )?;
        admit_terminal!(
            &mut self.promotion_dispositions,
            &mut self.promotion_terminal_by_request,
            disposition.disposition_id,
            disposition.exact_binding,
            disposition.request,
        )
    }
}

#[rustfmt::skip]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ContextResolutionKernelErrorKind { InvalidInput, AuthorityRefused, StaleAuthority, Indeterminate, ExpansionRefused, Cardinality, ReplayMismatch, ForbiddenTarget }

#[rustfmt::skip]
#[derive(Debug)]
pub struct ContextResolutionKernelError { kind: ContextResolutionKernelErrorKind, detail: String, candidate: Option<Box<ContextResolutionEscalationCandidate>> }

impl ContextResolutionKernelError {
    #[rustfmt::skip]
    pub fn kind(&self) -> ContextResolutionKernelErrorKind { self.kind }
    #[rustfmt::skip]
    pub fn detail(&self) -> &str { &self.detail }
    #[rustfmt::skip]
    pub fn candidate(self) -> Option<ContextResolutionEscalationCandidate> { self.candidate.map(|candidate| *candidate) }
}
