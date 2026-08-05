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
    NativeAuthenticatorPortV1, AUTHENTICATOR_RP_ID,
};
use crate::charter_lineage_store::{LineageRecordClassV1, TrustedLineageStoreV1};
use crate::context_resolution_registry::ContextResolutionStackDefinition;
use crate::definition_identity::{
    fingerprint_serializable, DefinitionFingerprint, ExactDefinitionRef, SourceByteBudget,
};
use crate::stable_role_registry::read_trusted_repo_source;
use crate::{resolve_profile_selection, ResolvedInstanceProfile, SymbolicId};
use base64::{engine::general_purpose::STANDARD as BASE64_STANDARD, Engine as _};
use serde::Serialize;
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::Arc;

pub(crate) const CONTEXT_RESOLUTION_AUTHORITY_KIND_REF: &str =
    "handbook.artifact-kind.context-resolution-authority-binding@1.0.0";
pub(crate) const CONTEXT_RESOLUTION_AUTHORITY_INSTANCE_ID: &str = "context_resolution_authority";
const CONTEXT_RESOLUTION_AUTHORITY_SCHEMA_REF: &str =
    "handbook.schemas.context-resolution-authority-capsule@1.0.0";
const CONTEXT_RESOLUTION_AUTHORITY_INTAKE_REF: &str =
    "handbook.intake.context-resolution-authority-capsule@1.0.0";
const CONTEXT_RESOLUTION_AUTHORITY_PATH: &str =
    ".handbook/project/context-resolution-authority.yaml";
const DECLARATION_TAG: &[u8] = b"handbook.hcm-3.2.declaration-jcs.v1\0";
const PAYLOAD_TAG: &[u8] = b"handbook.hcm-3.2.payload-jcs.v1\0";
const SEMANTIC_TAG: &[u8] = b"handbook.hcm-3.2.semantic-binding.v1\0";
const MAX_OUTER_BYTES: usize = 131_072;
const MAX_DECLARATION_BYTES: usize = 98_304;
const MAX_PAYLOAD_BYTES: usize = 81_920;
const MAX_ORDINARY_STRING_BYTES: usize = 8_192;
const MAX_MEMBER_COUNT: usize = 192;
const MAX_JSON_DEPTH: usize = 12;
const MAX_ESCAPE_EXPANSION_BYTES: usize = 16_384;

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
macro_rules! require_admission { ($admission:expr, $use:expr, $subject:expr, $fingerprint:expr $(,)?) => {{ let admission=$admission; let subject=$subject; let expected: Option<&str>=$fingerprint; admission.require_current()?; if admission.authority_use != $use || admission.subject != *subject || expected.is_some_and(|value| admission.binding_fingerprint != value) { Err(kernel_error!(ContextResolutionKernelErrorKind::AuthorityRefused, "authority admission use, subject, or binding is incoherent")) } else { Ok::<(), ContextResolutionKernelError>(()) } }} }
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
macro_rules! string_at { ($value:expr, $pointer:expr, $detail:expr) => {{ match $value.pointer($pointer).and_then(Value::as_str) { Some(value) if !value.is_empty() => value, _ => return Err(kernel_error!(ContextResolutionKernelErrorKind::InvalidInput, $detail)) } }} }

#[rustfmt::skip]
macro_rules! candidate_from_mapping { ($class:expr, $current:expr, $proposed:expr, $evidence:expr, $authority:expr, $admission:expr, $profile:expr, $stack:expr $(,)?) => {{ let class=$class; let authority=$authority; let pointer=format!("/candidate_mappings/{class}"); let trigger=ContextResolutionExactBinding::new(string_at!(authority,&format!("{pointer}/trigger/ref"),"candidate class is not mapped by current authority"),string_at!(authority,&format!("{pointer}/trigger/fingerprint"),"candidate class is not mapped by current authority"))?; Ok::<_,ContextResolutionKernelError>(ContextResolutionEscalationCandidate { class:class.to_owned(), trigger, missing_condition:string_at!(authority,&format!("{pointer}/missing_condition"),"candidate class is not mapped by current authority").to_owned(), requested_authority_ref:string_at!(authority,"/authority_mappings/requested/authority_ref","requested authority mapping is unavailable").to_owned(), evidence:$evidence, current:$current.clone(), proposed:$proposed.clone(), resolved_profile:$profile.clone(), resolution_stack:$stack.clone(), authority:authority.clone(), authority_admission:$admission.clone() }) }} }

#[rustfmt::skip]
macro_rules! materialize_envelope { ($input:expr, $admission:expr, $stack:expr, $layers:expr $(,)?) => {{ let input=$input; let authority_admission=$admission; let mut inherited_allow_layers=$layers; let local_allows=input.mutation_rules.iter().filter(|rule| rule.effect==ContextResolutionMutationEffect::Allow).cloned().collect::<Vec<_>>(); let mutation_denies=input.mutation_rules.iter().filter(|rule| rule.effect==ContextResolutionMutationEffect::Deny).cloned().collect::<Vec<_>>(); inherited_allow_layers.push(local_allows); let fingerprint=DefinitionFingerprint::from_json_value(&json!({"envelope_id":&input.envelope_id,"objective_ref":&input.objective_ref,"resolved_profile":&input.resolved_profile,"resolution_stack":&input.resolution_stack,"active_level_id":&input.active_level_id,"dimensions":&input.dimensions,"parent":&input.parent,"constraint_inputs":&input.constraint_inputs,"mutation_rules":&input.mutation_rules,"escalation_triggers":&input.escalation_triggers})).map_err(|_|kernel_error!(ContextResolutionKernelErrorKind::InvalidInput,"resolved envelope could not be fingerprinted"))?; let exact_binding=ContextResolutionExactBinding::new(&format!("context-resolution-envelope/{}",input.envelope_id),fingerprint.as_str())?; Ok::<_,ContextResolutionKernelError>(ContextResolutionEnvelope { exact_binding, _objective_ref:input.objective_ref, resolved_profile:input.resolved_profile, resolution_stack:input.resolution_stack, active_level_id:input.active_level_id, dimensions:input.dimensions, _constraint_inputs:input.constraint_inputs, mutation_allow_layers:inherited_allow_layers, mutation_denies, _escalation_triggers:input.escalation_triggers, authority:authority_admission.authority.clone(), authority_admission, stack:$stack }) }} }

#[rustfmt::skip]
macro_rules! typed_candidate { ($current:expr, $class:expr, $proposed:expr, $evidence:expr) => {{ let current=$current; let proposed=$proposed; let evidence=$evidence; current.require_current()?; proposed.require_current()?; if current.resolved_profile!=proposed.resolved_profile || current.resolution_stack!=proposed.resolution_stack || current.exact_binding==proposed.exact_binding || evidence.is_empty() || evidence.len()>256 || evidence.windows(2).any(|pair|pair[0].reference>=pair[1].reference) { return Err(kernel_error!(ContextResolutionKernelErrorKind::InvalidInput,"typed escalation proposal relation or evidence is invalid")); } candidate_from_mapping!($class,&current.exact_binding,&proposed.exact_binding,evidence,&current.authority,&current.authority_admission,&current.resolved_profile,&current.resolution_stack) }} }

#[rustfmt::skip]
#[derive(Clone, Debug)]
struct ValidatedAuthorityCapsule { binding: Arc<Value>, outer_artifact_fingerprint: String, declaration_byte_fingerprint: String, payload_byte_fingerprint: String, semantic_binding_fingerprint: String, prior_registry: Value }

#[rustfmt::skip]
#[derive(Clone, Debug)]
struct PublicationAuthorizedPendingCommit { capsule: ValidatedAuthorityCapsule, registry_state_ref: String, registry_state_fingerprint: String, registry_transition_ref: String, registry_transition_fingerprint: String, publisher_credential_id_hash: String, credentials: Vec<CommittedApproverCredentialV1> }

#[rustfmt::skip]
#[derive(Clone, Debug)]
struct CurrentAuthorityWitness { repo_root: PathBuf, expected_outer_fingerprint: String, pending: PublicationAuthorizedPendingCommit }

#[rustfmt::skip]
#[derive(Clone, Debug)]
#[allow(dead_code)]
struct HistoricalCommittedProof { outer_artifact_fingerprint: String, declaration_byte_fingerprint: String, payload_byte_fingerprint: String, semantic_binding_fingerprint: String, committed_successor_transaction_ref: String }

#[rustfmt::skip]
#[derive(Clone, Debug)]
#[allow(dead_code)]
struct CorruptOrAmbiguous { reason: String }

#[rustfmt::skip]
fn tagged_fingerprint(tag: &[u8], bytes: &[u8]) -> String { let mut digest=Sha256::new(); digest.update(tag); digest.update((bytes.len() as u64).to_be_bytes()); digest.update(bytes); format!("sha256:{:x}",digest.finalize()) }

#[rustfmt::skip]
fn exact_object(value: &Value, expected: &[&str]) -> bool { value.as_object().is_some_and(|object|object.len()==expected.len() && expected.iter().all(|key|object.contains_key(*key))) }

#[rustfmt::skip]
fn strict_jcs(bytes: &[u8], maximum: usize, label: &str) -> Result<Value, String> { if bytes.is_empty() || bytes.len()>maximum || bytes.starts_with(&[0xef,0xbb,0xbf]) || std::str::from_utf8(bytes).is_err() { return Err(format!("{label} is not bounded strict UTF-8")); } let value=crate::parse_schema_json(bytes).map_err(|_|format!("{label} is not duplicate-safe I-JSON"))?; let canonical=serde_json_canonicalizer::to_vec(&value).map_err(|_|format!("{label} cannot be emitted as JCS"))?; if canonical!=bytes { return Err(format!("{label} is not byte-identical JCS")); } Ok(value) }

fn escaped_output_bytes(bytes: &[u8]) -> Result<usize, String> {
    let mut total = 0usize;
    let mut index = 0usize;
    while index < bytes.len() {
        if bytes[index] != b'\\' {
            index += 1;
            continue;
        }
        let marker = *bytes
            .get(index + 1)
            .ok_or_else(|| "truncated JSON escape".to_owned())?;
        match marker {
            b'"' | b'\\' | b'/' | b'b' | b'f' | b'n' | b'r' | b't' => {
                total = total
                    .checked_add(1)
                    .ok_or_else(|| "JSON escape accounting overflowed".to_owned())?;
                index += 2;
            }
            b'u' => {
                let first = std::str::from_utf8(
                    bytes
                        .get(index + 2..index + 6)
                        .ok_or_else(|| "truncated unicode escape".to_owned())?,
                )
                .ok()
                .and_then(|value| u16::from_str_radix(value, 16).ok())
                .ok_or_else(|| "invalid unicode escape".to_owned())?;
                let scalar = if (0xd800..=0xdbff).contains(&first) {
                    if bytes.get(index + 6..index + 8) != Some(b"\\u") {
                        return Err("unpaired high surrogate".to_owned());
                    }
                    let second = std::str::from_utf8(
                        bytes
                            .get(index + 8..index + 12)
                            .ok_or_else(|| "truncated surrogate pair".to_owned())?,
                    )
                    .ok()
                    .and_then(|value| u16::from_str_radix(value, 16).ok())
                    .filter(|value| (0xdc00..=0xdfff).contains(value))
                    .ok_or_else(|| "invalid low surrogate".to_owned())?;
                    index += 12;
                    0x1_0000 + (((first as u32 - 0xd800) << 10) | (second as u32 - 0xdc00))
                } else {
                    if (0xdc00..=0xdfff).contains(&first) {
                        return Err("unpaired low surrogate".to_owned());
                    }
                    index += 6;
                    first as u32
                };
                total = total
                    .checked_add(
                        char::from_u32(scalar)
                            .ok_or_else(|| "invalid unicode scalar".to_owned())?
                            .len_utf8(),
                    )
                    .ok_or_else(|| "JSON escape accounting overflowed".to_owned())?;
            }
            _ => return Err("unknown JSON escape".to_owned()),
        }
    }
    Ok(total)
}

fn json_limits(
    value: &Value,
    depth: usize,
    member_count: &mut usize,
    exempt_string_pointer: Option<&str>,
    pointer: &str,
) -> Result<(), String> {
    if depth > MAX_JSON_DEPTH {
        return Err("authority JSON exceeds maximum depth".to_owned());
    }
    match value {
        Value::Object(object) => {
            *member_count = member_count
                .checked_add(object.len())
                .ok_or_else(|| "authority member count overflowed".to_owned())?;
            if *member_count > MAX_MEMBER_COUNT {
                return Err("authority JSON exceeds aggregate member count".to_owned());
            }
            for (key, child) in object {
                if key.len() > MAX_ORDINARY_STRING_BYTES {
                    return Err("authority member name exceeds string bound".to_owned());
                }
                let child_pointer =
                    format!("{pointer}/{}", key.replace('~', "~0").replace('/', "~1"));
                json_limits(
                    child,
                    depth + usize::from(child.is_object() || child.is_array()),
                    member_count,
                    exempt_string_pointer,
                    &child_pointer,
                )?;
            }
        }
        Value::Array(array) => {
            for (index, child) in array.iter().enumerate() {
                json_limits(
                    child,
                    depth + usize::from(child.is_object() || child.is_array()),
                    member_count,
                    exempt_string_pointer,
                    &format!("{pointer}/{index}"),
                )?;
            }
        }
        Value::String(value)
            if exempt_string_pointer != Some(pointer)
                && value.len() > MAX_ORDINARY_STRING_BYTES =>
        {
            return Err("authority ordinary string exceeds bound".to_owned());
        }
        _ => {}
    }
    Ok(())
}

fn semantic_leaf_count(value: &Value) -> usize {
    match value {
        Value::Object(object) if !object.is_empty() => {
            object.values().map(semantic_leaf_count).sum()
        }
        Value::Array(array) if !array.is_empty() => array.iter().map(semantic_leaf_count).sum(),
        _ => 1,
    }
}

#[rustfmt::skip]
fn validate_binding(binding: &Value) -> Result<(), String> { if !exact_object( binding, &[ "schema_id", "schema_version", "binding_id", "binding_version", "repository_identity_fingerprint", "resolved_profile",
"resolution_stack", "approver_registry", "authority_mappings", "candidate_mappings", "extensions", "binding_fingerprint", ], ) || binding.get("schema_id").and_then(Value::as_str)
!= Some("handbook.context-resolution-authority-binding") || binding.get("schema_version").and_then(Value::as_str) != Some("1.0") || !binding .get("extensions") .and_then(Value::as_object)
.is_some_and(serde_json::Map::is_empty) || semantic_leaf_count(binding) != 53 { return Err("authority payload is not the exact 53-leaf binding".to_owned()); } for pointer in ["/resolved_profile", "/resolution_stack"] {
let pair = binding.pointer(pointer).unwrap_or(&Value::Null); if !exact_object(pair, &["ref", "fingerprint"]) || ContextResolutionExactBinding::new( pair.get("ref").and_then(Value::as_str).unwrap_or_default(),
pair.get("fingerprint") .and_then(Value::as_str) .unwrap_or_default(), ) .is_err() { return Err("authority exact binding is malformed".to_owned()); } } let registry = binding .pointer("/approver_registry")
.unwrap_or(&Value::Null); if !exact_object( registry, &[ "state_ref", "state_fingerprint", "head_transition_ref", "head_transition_fingerprint", ], ) {
return Err("authority prior registry quartet is malformed".to_owned()); } for key in ["state_fingerprint", "head_transition_fingerprint"] { DefinitionFingerprint::parse( registry .get(key) .and_then(Value::as_str)
.unwrap_or_default(), ) .map_err(|_| "authority registry fingerprint is malformed".to_owned())?; } let uses = [ ContextResolutionAuthorityUse::RootCreation, ContextResolutionAuthorityUse::Parent,
ContextResolutionAuthorityUse::Requested, ContextResolutionAuthorityUse::Approving, ContextResolutionAuthorityUse::Decision, ContextResolutionAuthorityUse::Evidence, ContextResolutionAuthorityUse::TriggerCondition,
ContextResolutionAuthorityUse::Constraint, ContextResolutionAuthorityUse::Source, ContextResolutionAuthorityUse::Target, ContextResolutionAuthorityUse::TargetMemory, ]; let authority_mappings = binding
.pointer("/authority_mappings") .unwrap_or(&Value::Null); if !exact_object( authority_mappings, &uses.map(ContextResolutionAuthorityUse::as_str), ) { return Err("authority mappings are not exact".to_owned()); }
for authority_use in uses { let pair = authority_mappings .get(authority_use.as_str()) .unwrap_or(&Value::Null); if !exact_object(pair, &["approval_class", "authority_ref"]) || pair .get("approval_class")
.and_then(Value::as_str) .is_none_or(str::is_empty) || pair .get("authority_ref") .and_then(Value::as_str) .is_none_or(str::is_empty) { return Err("authority mapping is malformed".to_owned()); } } let candidates = [ (
"dimension_rank_increase", "one_or_more_dimension_ranks_exceed_parent", "current_and_proposed_envelopes", ), ( "mutation_allow_expansion", "child_allow_not_provably_contained", "parent_and_child_mutation_rules", ), (
"missing_context", "required_context_record_absent", "available_context_and_missing_ref", ), ( "missing_authority", "required_authority_mapping_absent", "current_authority_and_requested_pair", ), ];
let candidate_mappings = binding .pointer("/candidate_mappings") .unwrap_or(&Value::Null); if !exact_object(candidate_mappings, &candidates.map(|entry| entry.0)) {
return Err("candidate mappings are not exact".to_owned()); } for (class, condition, evidence) in candidates { let candidate = candidate_mappings.get(class).unwrap_or(&Value::Null); if !exact_object( candidate,
&["trigger", "missing_condition", "evidence_requirement"], ) || !exact_object( candidate.get("trigger").unwrap_or(&Value::Null), &["ref", "fingerprint"],
) || candidate.get("missing_condition").and_then(Value::as_str) != Some(condition) || candidate .get("evidence_requirement") .and_then(Value::as_str) != Some(evidence) || ContextResolutionExactBinding::new( candidate
.pointer("/trigger/ref") .and_then(Value::as_str) .unwrap_or_default(), candidate .pointer("/trigger/fingerprint") .and_then(Value::as_str) .unwrap_or_default(), ) .is_err() {
return Err("candidate mapping differs from the closed contract".to_owned()); } } for pointer in ["/binding_id", "/binding_version"] { let value = binding .pointer(pointer) .and_then(Value::as_str) .unwrap_or_default();
if value.is_empty() || value.len() > 128 { return Err("authority binding identity is malformed".to_owned()); } } DefinitionFingerprint::parse( binding .pointer("/repository_identity_fingerprint") .and_then(Value::as_str)
.unwrap_or_default(), ) .map_err(|_| "authority repository fingerprint is malformed".to_owned())?; Ok(()) }

#[rustfmt::skip]
fn validate_capsule_value(wrapper: &Value) -> Result<ValidatedAuthorityCapsule, String> { if !exact_object(wrapper, &["declaration_jcs"]) { return Err("Context Resolution capsule wrapper is not exact".to_owned()); }
let declaration_text = wrapper .get("declaration_jcs") .and_then(Value::as_str) .ok_or_else(|| "capsule declaration terminal is not a string".to_owned())?; let declaration_bytes = declaration_text.as_bytes();
let declaration = strict_jcs( declaration_bytes, MAX_DECLARATION_BYTES, "authority declaration", )?; if !exact_object( &declaration, &[ "schema_id", "schema_version", "codec_id", "codec_version", "payload_jcs",
"payload_byte_fingerprint", "semantic_binding_fingerprint", "predecessor", "prior_registry", ], ) || declaration.get("schema_id").and_then(Value::as_str) != Some("handbook.context-resolution-authority-declaration")
|| declaration.get("schema_version").and_then(Value::as_str) != Some("1.0") || declaration.get("codec_id").and_then(Value::as_str) != Some("handbook.hcm-3.2.context-resolution-authority-binding-jcs")
|| declaration.get("codec_version").and_then(Value::as_str) != Some("1.0") { return Err("authority declaration compatibility tuple is unsupported".to_owned()); } let payload_text = declaration .get("payload_jcs")
.and_then(Value::as_str) .ok_or_else(|| "authority payload carrier is not a string".to_owned())?; let payload_bytes = payload_text.as_bytes();
let binding = strict_jcs(payload_bytes, MAX_PAYLOAD_BYTES, "authority payload")?; let mut members = 0usize; json_limits(&declaration, 1, &mut members, Some("/payload_jcs"), "")?;
json_limits(&binding, 1, &mut members, None, "")?; let escape_bytes = escaped_output_bytes(declaration_bytes)? .checked_add(escaped_output_bytes(payload_bytes)?)
.ok_or_else(|| "authority escape accounting overflowed".to_owned())?; if escape_bytes > MAX_ESCAPE_EXPANSION_BYTES { return Err("authority escape expansion exceeds bound".to_owned()); } validate_binding(&binding)?;
let payload_byte_fingerprint = tagged_fingerprint(PAYLOAD_TAG, payload_bytes); if declaration .get("payload_byte_fingerprint") .and_then(Value::as_str) != Some(payload_byte_fingerprint.as_str()) {
return Err("authority payload-byte fingerprint mismatch".to_owned()); } let mut semantic = binding.clone(); semantic .as_object_mut() .expect("validated authority payload object") .remove("binding_fingerprint");
let semantic_bytes = serde_json_canonicalizer::to_vec(&semantic) .map_err(|_| "authority semantic preimage cannot be canonicalized".to_owned())?;
let semantic_binding_fingerprint = tagged_fingerprint(SEMANTIC_TAG, &semantic_bytes); if declaration .get("semantic_binding_fingerprint") .and_then(Value::as_str) != Some(semantic_binding_fingerprint.as_str())
|| binding.get("binding_fingerprint").and_then(Value::as_str) != Some(semantic_binding_fingerprint.as_str()) { return Err("authority semantic fingerprint mismatch".to_owned()); } let predecessor = declaration
.get("predecessor") .cloned() .ok_or_else(|| "authority predecessor is absent".to_owned())?; if !exact_object( &predecessor, &[ "outer_artifact_ref", "outer_artifact_fingerprint", "payload_byte_fingerprint",
"semantic_binding_fingerprint", ], ) { return Err("authority predecessor quartet is not exact".to_owned()); } let predecessor_nulls = predecessor .as_object() .expect("validated predecessor object") .values()
.filter(|value| value.is_null()) .count(); if predecessor_nulls != 0 && predecessor_nulls != 4 { return Err("authority predecessor quartet is mixed-null".to_owned()); }
if predecessor_nulls == 0 { for key in [
"outer_artifact_fingerprint", "payload_byte_fingerprint", "semantic_binding_fingerprint", ] { DefinitionFingerprint::parse( predecessor .get(key) .and_then(Value::as_str) .unwrap_or_default(), )
.map_err(|_| "authority predecessor fingerprint is malformed".to_owned())?; } if predecessor .get("outer_artifact_ref") .and_then(Value::as_str) != Some(CONTEXT_RESOLUTION_AUTHORITY_PATH) {
return Err("authority predecessor ref is not canonical".to_owned()); } } let prior_registry = declaration .get("prior_registry") .cloned() .ok_or_else(|| "authority prior registry is absent".to_owned())?;
if prior_registry != binding["approver_registry"] { return Err("authority prior registry differs from its payload".to_owned()); } let outer_bytes = crate::canonical_yaml::canonical_yaml_bytes(wrapper)
.map_err(|_| "authority wrapper cannot be emitted as canonical YAML".to_owned())?; if outer_bytes.len() > MAX_OUTER_BYTES { return Err("authority canonical artifact exceeds bound".to_owned()); }
Ok(ValidatedAuthorityCapsule { binding: Arc::new(binding), outer_artifact_fingerprint: DefinitionFingerprint::from_bytes(&outer_bytes).to_string(),
declaration_byte_fingerprint: tagged_fingerprint(DECLARATION_TAG, declaration_bytes), payload_byte_fingerprint, semantic_binding_fingerprint, prior_registry, }) }

fn validate_capsule_repository_bindings(
    repo_root: &Path,
    capsule: &ValidatedAuthorityCapsule,
) -> Result<(), String> {
    let mut budget = SourceByteBudget::default();
    let (_, identity_bytes) =
        read_trusted_repo_source(repo_root, ".handbook/repository-identity.v1", &mut budget)
            .map_err(|_| "repository identity is unavailable".to_owned())?;
    let identity = std::str::from_utf8(&identity_bytes)
        .ok()
        .and_then(|value| DefinitionFingerprint::parse(value).ok())
        .ok_or_else(|| "repository identity is malformed".to_owned())?;
    let (_, profile_bytes) =
        read_trusted_repo_source(repo_root, REPOSITORY_PROFILE_SELECTION_PATH, &mut budget)
            .map_err(|_| "repository profile selection is unavailable".to_owned())?;
    let selection = RepositoryProfileSelectionV1::from_json_bytes(&profile_bytes)
        .map_err(|_| "repository profile selection is invalid".to_owned())?;
    let profile = resolve_profile_selection(repo_root, selection.profile_request())
        .map_err(|_| "selected repository profile did not resolve".to_owned())?;
    let binding = &capsule.binding;
    if binding
        .pointer("/repository_identity_fingerprint")
        .and_then(Value::as_str)
        != Some(identity.as_str())
        || binding
            .pointer("/resolved_profile/ref")
            .and_then(Value::as_str)
            != Some(profile.exact_ref().as_str())
        || binding
            .pointer("/resolved_profile/fingerprint")
            .and_then(Value::as_str)
            != Some(profile.resolved_profile_fingerprint().as_str())
        || binding
            .pointer("/resolution_stack/ref")
            .and_then(Value::as_str)
            != Some(profile.context_resolution().exact_ref().as_str())
        || binding
            .pointer("/resolution_stack/fingerprint")
            .and_then(Value::as_str)
            != Some(
                profile
                    .context_resolution()
                    .definition_fingerprint()
                    .as_str(),
            )
    {
        return Err("authority repository, profile, or stack binding is stale".to_owned());
    }
    Ok(())
}

pub(crate) fn validate_context_resolution_capsule_candidate(
    repo_root: &Path,
    wrapper: &Value,
) -> Result<(), String> {
    let capsule = validate_capsule_value(wrapper)?;
    validate_capsule_repository_bindings(repo_root, &capsule)
}

#[rustfmt::skip]
fn capsule_predecessor(wrapper: &Value) -> Result<Value, String> { let declaration=wrapper.get("declaration_jcs").and_then(Value::as_str).ok_or_else(||"capsule declaration terminal is absent".to_owned())?; strict_jcs(declaration.as_bytes(),MAX_DECLARATION_BYTES,"authority declaration")?.get("predecessor").cloned().ok_or_else(||"authority predecessor quartet is absent".to_owned()) }

pub(crate) fn validate_context_resolution_declared_predecessor(
    wrapper: &Value,
    expected_current_artifact_fingerprint: Option<&str>,
) -> Result<(), String> {
    let predecessor = capsule_predecessor(wrapper)?;
    match expected_current_artifact_fingerprint {
        None => {
            if predecessor
                .as_object()
                .is_none_or(|object| object.values().any(|value| !value.is_null()))
            {
                return Err(
                    "root authority declaration must use the literal-null predecessor quartet"
                        .to_owned(),
                );
            }
        }
        Some(expected) => {
            DefinitionFingerprint::parse(expected).map_err(|_| {
                "expected authority predecessor fingerprint is malformed".to_owned()
            })?;
            if predecessor
                .get("outer_artifact_ref")
                .and_then(Value::as_str)
                != Some(CONTEXT_RESOLUTION_AUTHORITY_PATH)
                || predecessor
                    .get("outer_artifact_fingerprint")
                    .and_then(Value::as_str)
                    != Some(expected)
            {
                return Err(
                    "replacement authority declaration names the wrong predecessor".to_owned(),
                );
            }
        }
    }
    Ok(())
}

pub(crate) fn validate_context_resolution_current_predecessor(
    repo_root: &Path,
    wrapper: &Value,
    expected_current_artifact_fingerprint: Option<&str>,
) -> Result<(), String> {
    validate_context_resolution_declared_predecessor(
        wrapper,
        expected_current_artifact_fingerprint,
    )?;
    let Some(expected) = expected_current_artifact_fingerprint else {
        return Ok(());
    };
    let mut budget = SourceByteBudget::default();
    let (_, current_bytes) =
        read_trusted_repo_source(repo_root, CONTEXT_RESOLUTION_AUTHORITY_PATH, &mut budget)
            .map_err(|_| "current authority predecessor bytes are unavailable".to_owned())?;
    if DefinitionFingerprint::from_bytes(&current_bytes).as_str() != expected {
        return Err("current authority predecessor bytes have changed".to_owned());
    }
    let current_wrapper = crate::canonical_yaml::parse_canonical_yaml(&current_bytes)
        .map_err(|_| "current authority predecessor is not canonical YAML".to_owned())?;
    let current_declaration = current_wrapper
        .get("declaration_jcs")
        .and_then(Value::as_str)
        .ok_or_else(|| "current predecessor declaration is absent".to_owned())?;
    let current_declaration = strict_jcs(
        current_declaration.as_bytes(),
        MAX_DECLARATION_BYTES,
        "current predecessor declaration",
    )?;
    let predecessor = capsule_predecessor(wrapper)?;
    if predecessor.get("payload_byte_fingerprint")
        != current_declaration.get("payload_byte_fingerprint")
        || predecessor.get("semantic_binding_fingerprint")
            != current_declaration.get("semantic_binding_fingerprint")
    {
        return Err(
            "replacement authority predecessor quartet is not the exact current artifact"
                .to_owned(),
        );
    }
    Ok(())
}

#[rustfmt::skip]
fn safe_state_ref(value: &str) -> bool { !value.is_empty() && value.len()<=512 && value.is_ascii() && !value.starts_with('/') && !value.contains('\\') && !value.contains("://") && value.split('/').all(|segment|!segment.is_empty() && !matches!(segment,"."|"..")) }

fn strict_base64(value: &str, label: &str) -> Result<Vec<u8>, String> {
    let decoded = BASE64_STANDARD
        .decode(value)
        .map_err(|_| format!("{label} is not standard padded base64"))?;
    if decoded.is_empty() || decoded.len() > 16_384 || BASE64_STANDARD.encode(&decoded) != value {
        return Err(format!(
            "{label} is not byte-identical standard padded base64"
        ));
    }
    Ok(decoded)
}

fn approval_pairs(row: &Value) -> Result<Vec<(String, String)>, String> {
    let values = row
        .get("approval_mappings")
        .and_then(Value::as_array)
        .ok_or_else(|| "publisher credential mappings are absent".to_owned())?;
    let mut pairs = Vec::with_capacity(values.len());
    for value in values {
        if !exact_object(value, &["approval_class", "authority_ref"]) {
            return Err("publisher credential mapping is not exact".to_owned());
        }
        pairs.push((
            value
                .get("approval_class")
                .and_then(Value::as_str)
                .ok_or_else(|| "publisher approval class is malformed".to_owned())?
                .to_owned(),
            value
                .get("authority_ref")
                .and_then(Value::as_str)
                .ok_or_else(|| "publisher authority ref is malformed".to_owned())?
                .to_owned(),
        ));
    }
    if pairs.windows(2).any(|pair| pair[0] >= pair[1]) {
        return Err("publisher credential mappings are not ordered and unique".to_owned());
    }
    Ok(pairs)
}

#[rustfmt::skip]
fn verify_publisher_authority( repo_root: &Path, capsule: ValidatedAuthorityCapsule, outer_artifact_fingerprint: &str, publisher: &Value, require_current: bool,
) -> Result<Option<PublicationAuthorizedPendingCommit>, String> { validate_capsule_repository_bindings(repo_root, &capsule)?; if capsule.outer_artifact_fingerprint != outer_artifact_fingerprint {
return Err("publisher authority names the wrong canonical artifact".to_owned()); } DefinitionFingerprint::parse(outer_artifact_fingerprint) .map_err(|_| "outer authority fingerprint is malformed".to_owned())?;
if !exact_object( publisher, &[ "schema_id", "schema_version", "result_registry_state_ref", "result_registry_state_fingerprint", "result_transition_ref", "result_transition_fingerprint", "publisher_credential_id_hash",
"challenge_jcs_base64", "raw_assertion_response_base64", ], ) || publisher.get("schema_id").and_then(Value::as_str) != Some("handbook.context-resolution-publisher-authority")
|| publisher.get("schema_version").and_then(Value::as_str) != Some("1.0") { return Err("publisher authority object is not the exact closed record".to_owned()); }
if serde_json_canonicalizer::to_vec(publisher)
.map_err(|_| "publisher authority cannot be canonicalized".to_owned())? .len() > 65_536 { return Err("publisher authority object exceeds 64 KiB".to_owned()); } let publisher_string = |field: &str| { publisher .get(field)
.and_then(Value::as_str) .filter(|value| !value.is_empty()) .ok_or_else(|| format!("publisher authority {field} is malformed")) }; let result_state_ref = publisher_string("result_registry_state_ref")?;
let result_state_fingerprint = publisher_string("result_registry_state_fingerprint")?; let result_transition_ref = publisher_string("result_transition_ref")?;
let result_transition_fingerprint = publisher_string("result_transition_fingerprint")?; let publisher_credential_id_hash = publisher_string("publisher_credential_id_hash")?;
if !safe_state_ref(result_state_ref) || !safe_state_ref(result_transition_ref) { return Err("publisher authority registry ref is unsafe".to_owned()); } for fingerprint in [ result_state_fingerprint,
result_transition_fingerprint, publisher_credential_id_hash, ] { DefinitionFingerprint::parse(fingerprint) .map_err(|_| "publisher authority fingerprint is malformed".to_owned())?; } let challenge_bytes = strict_base64(
publisher_string("challenge_jcs_base64")?, "publisher challenge", )?; let raw_response = strict_base64( publisher_string("raw_assertion_response_base64")?, "publisher raw assertion response", )?;
let lineage = TrustedLineageStoreV1::new(repo_root); let (result_state, transition, credentials) = if require_current { let registry = observe_committed_approver_registry_locked(repo_root)
.map_err(|_| "committed publisher registry could not be observed".to_owned())?; if registry.state_ref != result_state_ref || registry.state_fingerprint != result_state_fingerprint
|| registry.head_transition_ref != result_transition_ref || registry.head_transition_fingerprint != result_transition_fingerprint { return Err("publisher authority is not the current committed registry head".to_owned());
} let transition = crate::parse_schema_json(&registry.head_transition_bytes) .map_err(|_| "publisher transition is not duplicate-safe JSON".to_owned())?; ( registry.state, transition, Some(registry.credentials), )
} else { let state_bytes = lineage .read_record( LineageRecordClassV1::RegistryState, result_state_ref, result_state_fingerprint, ) .map_err(|_| "historical publisher registry state is unavailable".to_owned())?;
let transition_bytes = lineage .read_record( LineageRecordClassV1::RegistryTransition, result_transition_ref, result_transition_fingerprint, ) .map_err(|_| "historical publisher transition is unavailable".to_owned())?;
let state = crate::parse_schema_json(&state_bytes) .map_err(|_| "historical publisher state is not duplicate-safe JSON".to_owned())?; let transition = crate::parse_schema_json(&transition_bytes)
.map_err(|_| "historical publisher transition is not duplicate-safe JSON".to_owned())?; if state.get("registry_state_fingerprint").and_then(Value::as_str) != Some(result_state_fingerprint)
|| transition.get("transition_fingerprint").and_then(Value::as_str) != Some(result_transition_fingerprint) || transition.get("result_registry_state_ref").and_then(Value::as_str) != Some(result_state_ref)
|| transition.get("result_registry_state_fingerprint").and_then(Value::as_str) != Some(result_state_fingerprint) { return Err("historical publisher registry bindings are not exact".to_owned()); }
(state, transition, None) }; let prior = &capsule.prior_registry; let equals = |field: &str, expected: Option<&str>| { transition.get(field).and_then(Value::as_str) == expected };
if !equals("operation", Some("update_mapping")) || !equals( "prior_registry_state_ref", prior.get("state_ref").and_then(Value::as_str), ) || !equals( "prior_registry_state_fingerprint",
prior.get("state_fingerprint").and_then(Value::as_str), ) || !equals( "prior_transition_ref", prior.get("head_transition_ref").and_then(Value::as_str), ) || !equals( "prior_transition_fingerprint", prior
.get("head_transition_fingerprint") .and_then(Value::as_str), ) || !equals("result_registry_state_ref", Some(result_state_ref)) || !equals( "result_registry_state_fingerprint", Some(result_state_fingerprint), )
|| !equals("changed_credential_id_hash", Some(publisher_credential_id_hash)) || !equals("admin_credential_id_hash", Some(publisher_credential_id_hash))
|| transition.get("authorization_kind").and_then(Value::as_str) != Some("assertion") { return Err("publisher transition is not the exact direct successor".to_owned()); } let prior_state_ref = prior .get("state_ref")
.and_then(Value::as_str) .ok_or_else(|| "publisher prior state ref is absent".to_owned())?; let prior_state_fingerprint = prior .get("state_fingerprint") .and_then(Value::as_str)
.ok_or_else(|| "publisher prior state fingerprint is absent".to_owned())?; let prior_state = crate::parse_schema_json( &lineage .read_record( LineageRecordClassV1::RegistryState, prior_state_ref, prior_state_fingerprint,
) .map_err(|_| "publisher prior registry state is unavailable".to_owned())?, ) .map_err(|_| "publisher prior registry state is not duplicate-safe JSON".to_owned())?; if prior_state .get("registry_state_fingerprint")
.and_then(Value::as_str) != prior.get("state_fingerprint").and_then(Value::as_str) { return Err("publisher prior registry state fingerprint mismatch".to_owned()); } let credential_row = |state: &Value| { state
.get("credentials") .and_then(Value::as_array) .and_then(|rows| { rows.iter().find(|row| { row.get("credential_id_hash").and_then(Value::as_str) == Some(publisher_credential_id_hash) }) }) .cloned()
.ok_or_else(|| "publisher credential is absent from registry state".to_owned()) }; let prior_credential = credential_row(&prior_state)?; let result_credential = credential_row(&result_state)?;
if prior_credential.get("status").and_then(Value::as_str) != Some("active") || result_credential.get("status").and_then(Value::as_str) != Some("active") { return Err("publisher credential is not active".to_owned()); }
let prior_pairs = approval_pairs(&prior_credential)?; let result_pairs = approval_pairs(&result_credential)?; let outer_hex = outer_artifact_fingerprint .strip_prefix("sha256:")
.ok_or_else(|| "outer fingerprint has no SHA-256 prefix".to_owned())?; let publication_pair = ( "context_resolution_authority_publication".to_owned(), format!("context-resolution-authority/{outer_hex}"), );
for required in [ ( "registry_admin".to_owned(), "repository_registry_admin".to_owned(), ), ( "context_resolution_authority_publisher".to_owned(), "repository_context_resolution_authority_publisher".to_owned(), ), ] {
if prior_pairs.binary_search(&required).is_err() { return Err("publisher credential lacks prerequisite authority".to_owned()); } } let mut expected_pairs = prior_pairs.clone();
expected_pairs.push(publication_pair.clone()); expected_pairs.sort(); if result_pairs != expected_pairs { return Err("publisher registry delta is not exactly one publication mapping".to_owned()); }
let publication_holders = result_state .get("credentials") .and_then(Value::as_array) .map(|rows| { rows.iter() .filter(|row| { row.get("status").and_then(Value::as_str) == Some("active") && row .get("approval_mappings")
.and_then(Value::as_array) .is_some_and(|mappings| { mappings.iter().any(|mapping| { mapping.get("approval_class").and_then(Value::as_str) == Some(publication_pair.0.as_str())
&& mapping.get("authority_ref").and_then(Value::as_str) == Some(publication_pair.1.as_str()) }) }) }) .count() }) .unwrap_or_default(); if publication_holders != 1 {
return Err("publisher mapping holder cardinality is not exact".to_owned()); } let prior_version = prior_state .get("registry_version") .and_then(Value::as_u64)
.ok_or_else(|| "publisher prior registry version is malformed".to_owned())?; if result_state.get("registry_version").and_then(Value::as_u64) != prior_version.checked_add(1) {
return Err("publisher registry version is not a direct successor".to_owned()); } let mut normalized_result = result_state.clone(); normalized_result .as_object_mut()
.ok_or_else(|| "publisher result state is not an object".to_owned())? .remove("registry_state_fingerprint"); normalized_result["registry_version"] = json!(prior_version); let result_rows = normalized_result
.get_mut("credentials") .and_then(Value::as_array_mut) .ok_or_else(|| "publisher result credentials are malformed".to_owned())?; let result_row = result_rows .iter_mut() .find(|row| {
row.get("credential_id_hash").and_then(Value::as_str) == Some(publisher_credential_id_hash) }) .ok_or_else(|| "publisher result credential disappeared".to_owned())?;
result_row["approval_mappings"] = prior_credential["approval_mappings"].clone(); let mut normalized_prior = prior_state.clone(); normalized_prior .as_object_mut()
.ok_or_else(|| "publisher prior state is not an object".to_owned())? .remove("registry_state_fingerprint"); if normalized_result != normalized_prior {
return Err("publisher registry contains an unrelated state delta".to_owned()); } let authorization_ref = transition .get("authorization_ref") .and_then(Value::as_str)
.ok_or_else(|| "publisher assertion ref is absent".to_owned())?; let authorization_fingerprint = transition .get("authorization_fingerprint") .and_then(Value::as_str)
.ok_or_else(|| "publisher assertion fingerprint is absent".to_owned())?; let assertion = crate::parse_schema_json( &lineage .read_record( LineageRecordClassV1::AuthenticatorAssertion, authorization_ref,
authorization_fingerprint, ) .map_err(|_| "publisher assertion record is unavailable".to_owned())?, ) .map_err(|_| "publisher assertion is not duplicate-safe JSON".to_owned())?;
 if assertion.get("assertion_fingerprint").and_then(Value::as_str) != Some(authorization_fingerprint) || assertion.get("credential_id_hash").and_then(Value::as_str) != Some(publisher_credential_id_hash) { return Err("publisher assertion chain is not exact".to_owned()); }
let registration_ref = result_credential.get("registration_ref").and_then(Value::as_str).ok_or_else(|| "publisher registration ref is absent".to_owned())?;
let registration_fingerprint = result_credential.get("registration_fingerprint").and_then(Value::as_str).ok_or_else(|| "publisher registration fingerprint is absent".to_owned())?;
let registration = crate::parse_schema_json(&lineage.read_record(LineageRecordClassV1::AuthenticatorRegistration, registration_ref, registration_fingerprint)
.map_err(|_| "publisher registration record is unavailable".to_owned())?).map_err(|_| "publisher registration record is malformed".to_owned())?;
if registration.get("registration_fingerprint").and_then(Value::as_str) != Some(registration_fingerprint) || registration.get("credential_id_hash").and_then(Value::as_str) != Some(publisher_credential_id_hash) {
return Err("publisher registration binding is not exact".to_owned()); }
let make_ref = registration.get("decoded_response_ref").and_then(Value::as_str).ok_or_else(|| "publisher make-credential response ref is absent".to_owned())?;
let make_fingerprint = registration.get("decoded_response_fingerprint").and_then(Value::as_str).ok_or_else(|| "publisher make-credential response fingerprint is absent".to_owned())?;
let make = crate::parse_schema_json(&lineage.read_record(LineageRecordClassV1::AuthenticatorMakeCredentialResponse, make_ref, make_fingerprint)
.map_err(|_| "publisher make-credential response is unavailable".to_owned())?).map_err(|_| "publisher make-credential response is malformed".to_owned())?;
let credential_id = strict_base64(make.get("credential_id_base64").and_then(Value::as_str).unwrap_or_default(), "publisher credential ID")?;
let cose_public_key = strict_base64(result_credential.get("cose_public_key_base64").and_then(Value::as_str).unwrap_or_default(), "publisher COSE public key")?;
if credential_id.is_empty() || credential_id.len() > 1_024 || cose_public_key.is_empty() || cose_public_key.len() > 4_096
|| DefinitionFingerprint::from_bytes(&credential_id).as_str() != publisher_credential_id_hash || make.get("credential_id_hash").and_then(Value::as_str) != Some(publisher_credential_id_hash)
 || make.get("cose_public_key_base64").and_then(Value::as_str) != result_credential.get("cose_public_key_base64").and_then(Value::as_str) { return Err("publisher retained credential bytes are not exact".to_owned()); }
 let client_data_hash: [u8; 32] = Sha256::digest(&challenge_bytes).into(); let decoded = decode_and_verify_get_assertion_response(&raw_response,
 &[AuthenticatorCredentialV1 { credential_id: credential_id.clone(), cose_public_key: cose_public_key.clone() }], client_data_hash)
 .map_err(|_| "publisher assertion cryptographic verification failed".to_owned())?;
 let challenge = strict_jcs(&challenge_bytes, 16_384, "publisher challenge")?; if !exact_object( &challenge, &[ "$schema", "changed_credential_id_hash", "nonce_base64", "operation", "operation_id",
 "prior_registry_state_fingerprint", "prior_transition_fingerprint", "repository_identity_fingerprint", "result_registry_state_fingerprint", "schema_id", "schema_version", ],
 ) || challenge.get("schema_id").and_then(Value::as_str) != Some("handbook.authenticator-challenge") || challenge.get("schema_version").and_then(Value::as_str) != Some("1.0")
 || challenge.get("operation").and_then(Value::as_str) != Some("update_mapping") || challenge.get("operation_id").and_then(Value::as_str) != Some(format!("hcm32-authorize-{outer_hex}").as_str()) || challenge
 .get("changed_credential_id_hash") .and_then(Value::as_str) != Some(publisher_credential_id_hash) || challenge .get("prior_registry_state_fingerprint") .and_then(Value::as_str)
 != prior.get("state_fingerprint").and_then(Value::as_str) || challenge .get("prior_transition_fingerprint") .and_then(Value::as_str) != prior .get("head_transition_fingerprint") .and_then(Value::as_str) || challenge
 .get("result_registry_state_fingerprint") .and_then(Value::as_str) != Some(result_state_fingerprint) || challenge .get("repository_identity_fingerprint") .and_then(Value::as_str) != capsule .binding
 .get("repository_identity_fingerprint") .and_then(Value::as_str) { return Err("publisher challenge does not bind the exact publication".to_owned()); } strict_base64( challenge .get("nonce_base64")
 .and_then(Value::as_str) .unwrap_or_default(), "publisher challenge nonce", )?; if assertion.get("challenge_jcs_base64").and_then(Value::as_str) != publisher.get("challenge_jcs_base64").and_then(Value::as_str) { return Err("publisher assertion chain is not exact".to_owned()); }
 let response_ref = assertion .get("decoded_response_ref") .and_then(Value::as_str) .ok_or_else(|| "publisher decoded response ref is absent".to_owned())?; let response_fingerprint = assertion .get("decoded_response_fingerprint") .and_then(Value::as_str)
 .ok_or_else(|| "publisher decoded response fingerprint is absent".to_owned())?; let response = crate::parse_schema_json( &lineage .read_record( LineageRecordClassV1::AuthenticatorGetAssertionResponse, response_ref,
 response_fingerprint, ) .map_err(|_| "publisher decoded response record is unavailable".to_owned())?, ) .map_err(|_| "publisher decoded response is not duplicate-safe JSON".to_owned())?;
 if response.get("response_fingerprint").and_then(Value::as_str) != Some(response_fingerprint) { return Err("publisher decoded response fingerprint mismatch".to_owned()); } let retained_raw = strict_base64( response
 .get("raw_response_base64") .and_then(Value::as_str) .unwrap_or_default(), "retained publisher raw response", )?; if retained_raw != raw_response { return Err("publisher raw response differs from retained assertion evidence".to_owned()); }
let authenticator_data = BASE64_STANDARD.encode(decoded.authenticator_data); let signature = BASE64_STANDARD.encode(&decoded.signature_der); let credential_id_base64 = BASE64_STANDARD.encode(&decoded.credential_id);
let mut signed_preimage = Vec::with_capacity(69); signed_preimage.extend_from_slice(&decoded.authenticator_data); signed_preimage.extend_from_slice(&client_data_hash);
let response_descriptor = response.get("credential_descriptor").ok_or_else(|| "publisher response credential descriptor is absent".to_owned())?;
if !exact_object(response_descriptor, &["credential_id_base64", "credential_id_hash", "type"]) || response_descriptor.get("credential_id_base64").and_then(Value::as_str) != Some(credential_id_base64.as_str())
|| response_descriptor.get("credential_id_hash").and_then(Value::as_str) != Some(publisher_credential_id_hash) || response_descriptor.get("type").and_then(Value::as_str) != Some("public-key")
|| response.get("authenticator_data_base64").and_then(Value::as_str) != Some(authenticator_data.as_str()) || response.get("signature_base64").and_then(Value::as_str) != Some(signature.as_str())
|| response.get("flags_byte").and_then(Value::as_u64) != Some(u64::from(decoded.flags_byte)) || response.get("sign_count").and_then(Value::as_u64) != Some(u64::from(decoded.sign_count))
|| response.get("status_byte").and_then(Value::as_u64) != Some(0) || response.get("user_present").and_then(Value::as_bool) != Some(true) || response.get("user_verified").and_then(Value::as_bool) != Some(true)
|| response.get("attested_credential_data_included").and_then(Value::as_bool) != Some(false) || response.get("extensions_included").and_then(Value::as_bool) != Some(false)
|| response.get("rp_id_hash").and_then(Value::as_str) != Some(DefinitionFingerprint::from_bytes(AUTHENTICATOR_RP_ID.as_bytes()).as_str()) { return Err("publisher decoded response projection is not exact".to_owned()); }
if assertion.get("client_data_hash").and_then(Value::as_str) != Some(DefinitionFingerprint::from_bytes(&challenge_bytes).as_str())
|| assertion.get("authenticator_data_base64").and_then(Value::as_str) != Some(authenticator_data.as_str()) || assertion.get("signature_base64").and_then(Value::as_str) != Some(signature.as_str())
|| assertion.get("sign_count").and_then(Value::as_u64) != Some(u64::from(decoded.sign_count)) || assertion.get("flags_byte").and_then(Value::as_u64) != Some(u64::from(decoded.flags_byte))
|| assertion.get("rp_id").and_then(Value::as_str) != Some(AUTHENTICATOR_RP_ID) || assertion.get("user_present").and_then(Value::as_bool) != Some(true) || assertion.get("user_verified").and_then(Value::as_bool) != Some(true)
|| assertion.get("signed_preimage_sha256").and_then(Value::as_str) != Some(DefinitionFingerprint::from_bytes(&signed_preimage).as_str()) { return Err("publisher assertion projection is not exact".to_owned()); }
let Some(credentials) = credentials else { return Ok(None); }; let current_matches = credentials.iter().filter(|credential| credential.active && credential.credential_id_hash == publisher_credential_id_hash).collect::<Vec<_>>();
if current_matches.len() != 1 { return Err("publisher credential currentness is not exact".to_owned()); } let current = current_matches[0];
if current.credential_id != decoded.credential_id || current.cose_public_key != cose_public_key || current.sign_count != decoded.sign_count { return Err("publisher current credential does not bind the verified assertion".to_owned()); }
let head_path = repo_root.join(".handbook/state").join(&current.use_head_ref); let head_bytes = fs::read(&head_path).map_err(|_| "publisher current use head is unavailable".to_owned())?; if head_bytes.len() > 65_536 { return Err("publisher current use head exceeds its bound".to_owned()); }
let head = crate::parse_schema_json(&head_bytes).map_err(|_| "publisher current use head is malformed".to_owned())?; let mut head_preimage = head.clone(); head_preimage.as_object_mut().ok_or_else(|| "publisher current use head is not an object".to_owned())?.remove("head_fingerprint");
if head.get("head_fingerprint").and_then(Value::as_str) != Some(current.use_head_fingerprint.as_str()) || DefinitionFingerprint::from_json_value(&head_preimage).map_err(|_| "publisher current use head cannot be fingerprinted".to_owned())?.as_str() != current.use_head_fingerprint
|| head.get("last_assertion_ref").and_then(Value::as_str) != Some(authorization_ref) || head.get("last_assertion_fingerprint").and_then(Value::as_str) != Some(authorization_fingerprint)
|| head.get("sign_count").and_then(Value::as_u64) != Some(u64::from(decoded.sign_count)) || head.get("sequence").and_then(Value::as_u64) != Some(u64::from(current.use_sequence)) { return Err("publisher current use-head advancement is not exact".to_owned()); }
Ok(Some(PublicationAuthorizedPendingCommit {
capsule, registry_state_ref: result_state_ref.to_owned(), registry_state_fingerprint: result_state_fingerprint.to_owned(), registry_transition_ref: result_transition_ref.to_owned(),
registry_transition_fingerprint: result_transition_fingerprint.to_owned(), publisher_credential_id_hash: publisher_credential_id_hash.to_owned(), credentials, })) }

pub(crate) fn validate_context_resolution_publication(
    repo_root: &Path,
    wrapper: &Value,
    outer_artifact_fingerprint: &str,
    publisher: &Value,
) -> Result<(), String> {
    let capsule = validate_capsule_value(wrapper)?;
    verify_publisher_authority(
        repo_root,
        capsule,
        outer_artifact_fingerprint,
        publisher,
        true,
    )
    .map(|_| ())
}

pub(crate) fn validate_context_resolution_historical_publication(
    repo_root: &Path,
    wrapper: &Value,
    outer_artifact_fingerprint: &str,
    publisher: &Value,
) -> Result<(), String> {
    let capsule = validate_capsule_value(wrapper)?;
    verify_publisher_authority(
        repo_root,
        capsule,
        outer_artifact_fingerprint,
        publisher,
        false,
    )
    .map(|_| ())
}

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
pub struct ContextResolutionAuthorityAdmission { authority_use: ContextResolutionAuthorityUse, subject: ContextResolutionExactBinding, binding_fingerprint: String, authority: Arc<Value>, witness: Arc<CurrentAuthorityWitness> }

impl ContextResolutionAuthorityAdmission {
    #[rustfmt::skip]
    pub fn authority_use(&self) -> ContextResolutionAuthorityUse { self.authority_use }
    #[rustfmt::skip]
    pub fn subject(&self) -> &ContextResolutionExactBinding { &self.subject }
    #[rustfmt::skip]
    pub fn binding_fingerprint(&self) -> &str { &self.binding_fingerprint }

    #[rustfmt::skip]
    fn require_current(&self) -> Result<(), ContextResolutionKernelError> {
        let expected = &self.witness.pending;
        let authority = ArtifactRepositoryAuthorityGuardV1::acquire(&self.witness.repo_root)
            .map_err(|_| {
                kernel_error!(
                    ContextResolutionKernelErrorKind::StaleAuthority,
                    "repository authority is no longer current"
                )
            })?;
        if authority.repository_identity_fingerprint().as_str()
            != expected
                .capsule
                .binding
                .pointer("/repository_identity_fingerprint")
                .and_then(Value::as_str)
                .unwrap_or_default()
        {
            return Err(kernel_error!(
                ContextResolutionKernelErrorKind::StaleAuthority,
                "repository identity changed after admission"
            ));
        }
        let (committed, committed_intent) =
            GenericArtifactLineageStoreV1::new(&self.witness.repo_root)
                .read_committed_authoritative_with_intent_for_currentness(
                    CONTEXT_RESOLUTION_AUTHORITY_PATH,
                    &self.witness.expected_outer_fingerprint,
                )
                .map_err(|_| {
                    kernel_error!(
                        ContextResolutionKernelErrorKind::StaleAuthority,
                        "authority artifact is no longer one exact committed output"
                    )
                })?;
        if DefinitionFingerprint::from_bytes(&committed).as_str()
            != self.witness.expected_outer_fingerprint
        {
            return Err(kernel_error!(
                ContextResolutionKernelErrorKind::StaleAuthority,
                "authority artifact bytes changed after admission"
            ));
        }
        let wrapper = crate::canonical_yaml::parse_canonical_yaml(&committed).map_err(|_| {
            kernel_error!(
                ContextResolutionKernelErrorKind::StaleAuthority,
                "authority capsule is no longer canonical YAML"
            )
        })?;
        let capsule = validate_capsule_value(&wrapper).map_err(|detail| {
            kernel_error!(ContextResolutionKernelErrorKind::StaleAuthority, detail)
        })?;
        validate_capsule_repository_bindings(&self.witness.repo_root, &capsule).map_err(
            |detail| kernel_error!(ContextResolutionKernelErrorKind::StaleAuthority, detail),
        )?;
        let publisher = publisher_authority_from_committed_intent(
            &committed_intent,
            &self.witness.expected_outer_fingerprint,
        )
        .map_err(|detail| {
            kernel_error!(ContextResolutionKernelErrorKind::StaleAuthority, detail)
        })?;
        let registry = observe_committed_approver_registry_locked(&self.witness.repo_root)
            .map_err(|_| {
                kernel_error!(
                    ContextResolutionKernelErrorKind::StaleAuthority,
                    "publisher registry is no longer observable"
                )
            })?;
        let observed = &capsule;
        if observed.outer_artifact_fingerprint != expected.capsule.outer_artifact_fingerprint
            || observed.declaration_byte_fingerprint
                != expected.capsule.declaration_byte_fingerprint
            || observed.payload_byte_fingerprint != expected.capsule.payload_byte_fingerprint
            || observed.semantic_binding_fingerprint
                != expected.capsule.semantic_binding_fingerprint
            || registry.state_ref != expected.registry_state_ref
            || registry.state_fingerprint != expected.registry_state_fingerprint
            || registry.head_transition_ref != expected.registry_transition_ref
            || registry.head_transition_fingerprint != expected.registry_transition_fingerprint
            || registry.credentials != expected.credentials
            || publisher
                .get("result_registry_state_ref")
                .and_then(Value::as_str)
                != Some(expected.registry_state_ref.as_str())
            || publisher
                .get("result_registry_state_fingerprint")
                .and_then(Value::as_str)
                != Some(expected.registry_state_fingerprint.as_str())
            || publisher
                .get("result_transition_ref")
                .and_then(Value::as_str)
                != Some(expected.registry_transition_ref.as_str())
            || publisher
                .get("result_transition_fingerprint")
                .and_then(Value::as_str)
                != Some(expected.registry_transition_fingerprint.as_str())
            || publisher
                .get("publisher_credential_id_hash")
                .and_then(Value::as_str)
                != Some(expected.publisher_credential_id_hash.as_str())
        {
            return Err(kernel_error!(
                ContextResolutionKernelErrorKind::StaleAuthority,
                "context resolution authority admission is no longer current"
            ));
        }
        verify_publisher_authority(&self.witness.repo_root, capsule, &self.witness.expected_outer_fingerprint, &publisher, true)
            .map_err(|detail| kernel_error!(ContextResolutionKernelErrorKind::StaleAuthority, detail))?
            .ok_or_else(|| kernel_error!(ContextResolutionKernelErrorKind::StaleAuthority, "current publisher revalidation did not produce an operational witness"))?;
        Ok(())
    }
}

#[rustfmt::skip]
pub struct ContextResolutionAuthorityAdmissionResolver { snapshot: Arc<CurrentAuthorityWitness>, cache: BTreeMap<(ContextResolutionAuthorityUse, String), ContextResolutionAuthorityAdmission>, counters: BTreeMap<Vec<u8>, u32> }

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
        if binding_kind_ref != CONTEXT_RESOLUTION_AUTHORITY_KIND_REF
            || binding_instance_id != CONTEXT_RESOLUTION_AUTHORITY_INSTANCE_ID
        {
            return Err(kernel_error!(
                ContextResolutionKernelErrorKind::InvalidInput,
                "authority binding target is outside the private HCM-3.2 tuple",
            ));
        }
        let snapshot = load_authority_snapshot(
            &repo_root,
            binding_kind_ref,
            binding_instance_id,
            expected_artifact_fingerprint,
        )?;
        Ok(Self {
            snapshot: Arc::new(snapshot),
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
            cached.require_current()?;
            return Ok(cached.clone());
        }
        let current = self.snapshot.clone();
        ContextResolutionAuthorityAdmission {
            authority_use,
            subject: subject.clone(),
            binding_fingerprint: current.pending.capsule.semantic_binding_fingerprint.clone(),
            authority: current.pending.capsule.binding.clone(),
            witness: current.clone(),
        }
        .require_current()?;
        if current.pending.capsule.outer_artifact_fingerprint
            != self.snapshot.pending.capsule.outer_artifact_fingerprint
            || current.pending.capsule.declaration_byte_fingerprint
                != self.snapshot.pending.capsule.declaration_byte_fingerprint
            || current.pending.capsule.payload_byte_fingerprint
                != self.snapshot.pending.capsule.payload_byte_fingerprint
            || current.pending.capsule.semantic_binding_fingerprint
                != self.snapshot.pending.capsule.semantic_binding_fingerprint
            || current.pending.registry_transition_fingerprint
                != self.snapshot.pending.registry_transition_fingerprint
        {
            return Err(kernel_error!(
                ContextResolutionKernelErrorKind::StaleAuthority,
                "context resolution authority binding changed",
            ));
        }
        let current_binding = current.pending.capsule.binding.clone();
        let current_credentials = &current.pending.credentials;
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
        for bytes in [b"handbook.context-resolution-authority.challenge.v1".as_slice(), nonce.as_slice(), string_at!(&current_binding, "/repository_identity_fingerprint", "authority binding is malformed").as_bytes(), current.pending.registry_state_ref.as_bytes(), current.pending.registry_state_fingerprint.as_bytes(), current.pending.registry_transition_ref.as_bytes(), current.pending.registry_transition_fingerprint.as_bytes(), current.pending.capsule.outer_artifact_fingerprint.as_bytes(), current.pending.capsule.declaration_byte_fingerprint.as_bytes(), current.pending.capsule.payload_byte_fingerprint.as_bytes(), current.pending.capsule.semantic_binding_fingerprint.as_bytes(), authority_use.as_str().as_bytes(), subject.reference.as_bytes(), subject.fingerprint.as_bytes()] {
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
            witness: current,
        };
        self.cache.insert(key, admission.clone());
        Ok(admission)
    }
}

fn publisher_authority_from_committed_intent(
    intent: &Value,
    outer_artifact_fingerprint: &str,
) -> Result<Value, String> {
    let subject = intent
        .get("request_subject")
        .ok_or_else(|| "committed promotion request subject is absent".to_owned())?;
    if subject.get("kind_ref").and_then(Value::as_str)
        != Some(CONTEXT_RESOLUTION_AUTHORITY_KIND_REF)
        || subject.get("instance_id").and_then(Value::as_str)
            != Some(CONTEXT_RESOLUTION_AUTHORITY_INSTANCE_ID)
        || !intent
            .get("outputs")
            .and_then(Value::as_array)
            .is_some_and(|outputs| {
                outputs.iter().any(|output| {
                    output.get("final_ref").and_then(Value::as_str)
                        == Some(CONTEXT_RESOLUTION_AUTHORITY_PATH)
                        && output.get("bytes_sha256").and_then(Value::as_str)
                            == Some(outer_artifact_fingerprint)
                })
            })
    {
        return Err("verified promotion intent does not cite the exact HCM output".to_owned());
    }
    subject
        .get("publisher_authority")
        .cloned()
        .ok_or_else(|| "HCM publication intent omits publisher authority".to_owned())
}

#[rustfmt::skip]
fn recovery_candidate_cardinality(candidates:&[(String,String)]) -> (&'static str,Option<String>,Option<String>) { match candidates { [(candidate_ref,candidate_fingerprint)] => ("retry_available",Some(candidate_ref.clone()),Some(candidate_fingerprint.clone())), [] => ("candidate_missing",None,None), _ => ("candidate_ambiguous",None,None) } }

#[rustfmt::skip]
#[cfg(test)]
mod recovery_candidate_cardinality_tests { use super::recovery_candidate_cardinality; #[test] fn zero_and_multiple_candidates_refuse_exactly() { assert_eq!(recovery_candidate_cardinality(&[]),("candidate_missing",None,None)); let candidates=vec![("a".to_owned(),"sha256:a".to_owned()),("b".to_owned(),"sha256:b".to_owned())]; assert_eq!(recovery_candidate_cardinality(&candidates),("candidate_ambiguous",None,None)); } }

#[rustfmt::skip]
fn detect_authorized_publication_gap(
    repo_root: &Path,
    _expected_outer_fingerprint: &str,
    store: &GenericArtifactLineageStoreV1,
) -> Result<(), ContextResolutionKernelError> {
    let registry = observe_committed_approver_registry_locked(repo_root).map_err(|_| {
        kernel_error!(
            ContextResolutionKernelErrorKind::AuthorityRefused,
            "publisher registry could not be observed for HCM recovery"
        )
    })?;
    let transition = crate::parse_schema_json(&registry.head_transition_bytes).map_err(|_| {
        kernel_error!(
            ContextResolutionKernelErrorKind::AuthorityRefused,
            "publisher transition is malformed during HCM recovery"
        )
    })?;
    if transition.get("operation").and_then(Value::as_str) != Some("update_mapping") {
        return Ok(());
    }
    let Some(assertion_ref) = transition.get("authorization_ref").and_then(Value::as_str) else {
        return Ok(());
    };
    let Some(assertion_fingerprint) = transition
        .get("authorization_fingerprint")
        .and_then(Value::as_str)
    else {
        return Ok(());
    };
    let trusted = TrustedLineageStoreV1::new(repo_root);
    let assertion_bytes = trusted
        .read_record(
            LineageRecordClassV1::AuthenticatorAssertion,
            assertion_ref,
            assertion_fingerprint,
        )
        .map_err(|_| {
            kernel_error!(
                ContextResolutionKernelErrorKind::AuthorityRefused,
                "HCM publication assertion is unavailable"
            )
        })?;
    let assertion = crate::parse_schema_json(&assertion_bytes).map_err(|_| {
        kernel_error!(
            ContextResolutionKernelErrorKind::AuthorityRefused,
            "HCM publication assertion is malformed"
        )
    })?;
    let challenge_bytes = assertion
        .get("challenge_jcs_base64")
        .and_then(Value::as_str)
        .ok_or_else(|| {
            kernel_error!(
                ContextResolutionKernelErrorKind::AuthorityRefused,
                "HCM publication challenge is absent"
            )
        })
        .and_then(|value| {
            strict_base64(value, "HCM publication challenge").map_err(|detail| {
                kernel_error!(ContextResolutionKernelErrorKind::AuthorityRefused, detail)
            })
        })?;
    let challenge =
        strict_jcs(&challenge_bytes, 16_384, "HCM publication challenge").map_err(|detail| {
            kernel_error!(ContextResolutionKernelErrorKind::AuthorityRefused, detail)
        })?;
    let Some(outer_hex) = challenge
        .get("operation_id")
        .and_then(Value::as_str)
        .and_then(|value| value.strip_prefix("hcm32-authorize-"))
        .filter(|value| {
            value.len() == 64
                && value
                    .bytes()
                    .all(|byte| byte.is_ascii_hexdigit() && !byte.is_ascii_uppercase())
        })
    else {
        return Ok(());
    };
    let authorized_outer_fingerprint = format!("sha256:{outer_hex}");
    let publication_pair = format!("context-resolution-authority/{outer_hex}");
    let holders = registry
        .state
        .get("credentials")
        .and_then(Value::as_array)
        .map(|rows| {
            rows.iter()
                .filter(|row| {
                    row.get("status").and_then(Value::as_str) == Some("active")
                        && row
                            .get("approval_mappings")
                            .and_then(Value::as_array)
                            .is_some_and(|mappings| {
                                mappings.iter().any(|mapping| {
                                    mapping.get("approval_class").and_then(Value::as_str)
                                        == Some("context_resolution_authority_publication")
                                        && mapping.get("authority_ref").and_then(Value::as_str)
                                            == Some(publication_pair.as_str())
                                })
                            })
                })
                .count()
        })
        .unwrap_or_default();
    if holders != 1 {
        return record_publication_quarantine(
            repo_root,
            store,
            &registry,
            &authorized_outer_fingerprint,
            None,
            None,
            None,
            "authorized_without_generic_intent",
            "registry_delta_ambiguous",
        );
    }

    let recovery_publisher = assertion.get("decoded_response_ref").and_then(Value::as_str)
        .zip(assertion.get("decoded_response_fingerprint").and_then(Value::as_str))
        .and_then(|(relative_ref, fingerprint)| trusted.read_record(LineageRecordClassV1::AuthenticatorGetAssertionResponse, relative_ref, fingerprint).ok())
        .and_then(|bytes| crate::parse_schema_json(&bytes).ok())
        .map(|response| json!({
            "schema_id":"handbook.context-resolution-publisher-authority", "schema_version":"1.0",
            "result_registry_state_ref":registry.state_ref, "result_registry_state_fingerprint":registry.state_fingerprint,
            "result_transition_ref":registry.head_transition_ref, "result_transition_fingerprint":registry.head_transition_fingerprint,
            "publisher_credential_id_hash":transition["changed_credential_id_hash"], "challenge_jcs_base64":assertion["challenge_jcs_base64"],
            "raw_assertion_response_base64":response["raw_response_base64"]
        }));

    if store
        .read_committed_authoritative_for_currentness(
            CONTEXT_RESOLUTION_AUTHORITY_PATH,
            &authorized_outer_fingerprint,
        )
        .is_ok()
    {
        store
            .reconcile_hcm_publication_quarantine(&registry.head_transition_fingerprint)
            .map_err(|_| {
                kernel_error!(ContextResolutionKernelErrorKind::AuthorityRefused,
                    "committed HCM publication has unresolved quarantine identity")
            })?;
        return Ok(());
    }

    let promotion_root = repo_root.join(".handbook/state/transactions/artifact-promotions");
    let mut journal_conflict = false;
    let mut journal_transaction_id = None;
    if promotion_root.exists() {
        for entry in fs::read_dir(&promotion_root).map_err(|_| {
            kernel_error!(
                ContextResolutionKernelErrorKind::AuthorityRefused,
                "HCM promotion journal is unreadable"
            )
        })? {
            let path = entry
                .map_err(|_| {
                    kernel_error!(
                        ContextResolutionKernelErrorKind::AuthorityRefused,
                        "HCM promotion journal entry is unreadable"
                    )
                })?
                .path();
            if !matches!(
                path.extension().and_then(|value| value.to_str()),
                Some("pending" | "committed")
            ) {
                continue;
            }
            let intent =
                crate::parse_schema_json(&fs::read(path.join("intent.json")).map_err(|_| {
                    kernel_error!(
                        ContextResolutionKernelErrorKind::AuthorityRefused,
                        "HCM promotion intent is unavailable"
                    )
                })?)
                .map_err(|_| {
                    kernel_error!(
                        ContextResolutionKernelErrorKind::AuthorityRefused,
                        "HCM promotion intent is malformed"
                    )
                })?;
            if intent
                .get("outputs")
                .and_then(Value::as_array)
                .is_some_and(|outputs| {
                    outputs.iter().any(|output| {
                        output.get("bytes_sha256").and_then(Value::as_str)
                            == Some(authorized_outer_fingerprint.as_str())
                    })
                })
            {
                journal_conflict = true;
                let transaction_id = path.file_stem().and_then(|value| value.to_str()).unwrap_or_default().to_owned();
                if journal_transaction_id.as_ref().is_none_or(|current: &String| transaction_id < *current) { journal_transaction_id = Some(transaction_id); }
            }
        }
    }

    let candidate_root =
        repo_root.join(".handbook/evidence/artifacts/context_resolution_authority/candidates");
    let mut candidates = Vec::new();
    let mut candidate_invalid = false;
    let mut predecessor_mismatch = false;
    let mut registry_delta_ambiguous = recovery_publisher.is_none();
    if candidate_root.exists() {
        let mut entries = fs::read_dir(&candidate_root)
            .map_err(|_| {
                kernel_error!(
                    ContextResolutionKernelErrorKind::AuthorityRefused,
                    "HCM candidate inventory is unreadable"
                )
            })?
            .map(|entry| entry.map(|value| value.path()))
            .collect::<Result<Vec<_>, _>>()
            .map_err(|_| {
                kernel_error!(
                    ContextResolutionKernelErrorKind::AuthorityRefused,
                    "HCM candidate entry is unreadable"
                )
            })?;
        entries.sort();
        for path in entries {
            let bytes = fs::read(&path).map_err(|_| {
                kernel_error!(
                    ContextResolutionKernelErrorKind::AuthorityRefused,
                    "HCM candidate record is unavailable"
                )
            })?;
            let candidate = match crate::parse_schema_json(&bytes) {
                Ok(value) => value,
                Err(_) => continue,
            };
            if candidate.get("target_kind_ref").and_then(Value::as_str)
                != Some(CONTEXT_RESOLUTION_AUTHORITY_KIND_REF)
                || candidate.get("target_instance_id").and_then(Value::as_str)
                    != Some(CONTEXT_RESOLUTION_AUTHORITY_INSTANCE_ID)
                || candidate
                    .get("promotion_eligibility")
                    .and_then(Value::as_str)
                    != Some("eligible_without_approval")
            {
                continue;
            }
            candidate_invalid = true;
            let predecessor_refused = candidate.get("normalized_content_ref").and_then(Value::as_str)
                .filter(|value| value.starts_with(".handbook/evidence/") && !value.contains("..") && !value.contains('\\'))
                .and_then(|value| fs::read(repo_root.join(value)).ok()).and_then(|bytes| crate::parse_schema_json(&bytes).ok())
                .filter(|wrapper| validate_capsule_value(wrapper).is_ok() && crate::canonical_yaml::canonical_yaml_bytes(wrapper).ok()
                    .is_some_and(|bytes| DefinitionFingerprint::from_bytes(&bytes).as_str() == authorized_outer_fingerprint))
                .is_some_and(|wrapper| { let current = fs::read(repo_root.join(CONTEXT_RESOLUTION_AUTHORITY_PATH)).ok().map(|bytes| DefinitionFingerprint::from_bytes(&bytes).to_string());
                    validate_context_resolution_current_predecessor(repo_root, &wrapper, current.as_deref()).is_err() });
            if predecessor_refused { predecessor_mismatch = true; continue; }
            let Some(candidate_ref) = path
                .strip_prefix(repo_root)
                .ok()
                .and_then(|value| value.to_str())
                .map(|value| value.replace('\\', "/"))
            else {
                continue;
            };
            let Some(candidate_fingerprint) = candidate
                .get("candidate_fingerprint")
                .and_then(Value::as_str)
            else {
                continue;
            };
            if store
                .read_committed_authoritative_for_currentness(&candidate_ref, candidate_fingerprint)
                .is_err()
            {
                continue;
            }
            let Some(content_ref) = candidate
                .get("normalized_content_ref")
                .and_then(Value::as_str)
            else {
                continue;
            };
            let Some(content_fingerprint) = candidate
                .get("normalized_content_fingerprint")
                .and_then(Value::as_str)
            else {
                continue;
            };
            let Ok(content_bytes) = store.read_committed_closure_for_currentness(
                &candidate_ref,
                candidate_fingerprint,
                content_ref,
                content_fingerprint,
            ) else {
                continue;
            };
            let Ok(wrapper) = crate::parse_schema_json(&content_bytes) else {
                continue;
            };
            let Ok(capsule) = validate_capsule_value(&wrapper) else { continue; };
            let Ok(canonical) = crate::canonical_yaml::canonical_yaml_bytes(&wrapper) else {
                continue;
            };
            if DefinitionFingerprint::from_bytes(&canonical).as_str()
                == authorized_outer_fingerprint
            {
                let current = fs::read(repo_root.join(CONTEXT_RESOLUTION_AUTHORITY_PATH))
                    .ok()
                    .map(|bytes| DefinitionFingerprint::from_bytes(&bytes).to_string());
                if validate_context_resolution_current_predecessor(
                    repo_root,
                    &wrapper,
                    current.as_deref(),
                )
                .is_err()
                {
                    predecessor_mismatch = true;
                    continue;
                }
                if recovery_publisher.as_ref().is_none_or(|publisher| verify_publisher_authority(repo_root, capsule, &authorized_outer_fingerprint, publisher, true).is_err()) {
                    registry_delta_ambiguous = true;
                    continue;
                }
                candidate_invalid = false;
                candidates.push((candidate_ref, candidate_fingerprint.to_owned()));
            }
        }
    }
    let installed_without_commit = fs::read(repo_root.join(CONTEXT_RESOLUTION_AUTHORITY_PATH))
        .ok()
        .is_some_and(|bytes| {
            DefinitionFingerprint::from_bytes(&bytes).as_str() == authorized_outer_fingerprint
        });
    let (phase, reason, candidate_ref, candidate_fingerprint) = if installed_without_commit {
        ("installed_without_commit", "installed_result_ambiguous", None, None)
    } else if journal_conflict {
        ("generic_pending", "journal_conflict", None, None)
    } else if registry_delta_ambiguous {
        ("authorized_without_generic_intent", "registry_delta_ambiguous", None, None)
    } else if predecessor_mismatch {
        ("authorized_without_generic_intent", "predecessor_mismatch", None, None)
    } else if candidate_invalid && candidates.is_empty() {
        ("authorized_without_generic_intent", "candidate_invalid", None, None)
    } else {
        let (reason,candidate_ref,candidate_fingerprint)=recovery_candidate_cardinality(&candidates);
        ("authorized_without_generic_intent",reason,candidate_ref,candidate_fingerprint)
    };
    record_publication_quarantine(
        repo_root,
        store,
        &registry,
        &authorized_outer_fingerprint,
        candidate_ref,
        candidate_fingerprint,
        (phase != "authorized_without_generic_intent").then_some(journal_transaction_id).flatten(),
        phase,
        reason,
    )
}

#[allow(clippy::too_many_arguments)]
#[rustfmt::skip]
fn record_publication_quarantine(repo_root: &Path, store: &GenericArtifactLineageStoreV1,
    registry: &crate::approver_registry_observation::RetainedApproverRegistryObservationV1,
    authorized_outer_fingerprint: &str, candidate_ref: Option<String>, candidate_fingerprint: Option<String>, journal_transaction_id: Option<String>, phase: &str, reason: &str,
) -> Result<(), ContextResolutionKernelError> {
    let outer_hex = authorized_outer_fingerprint.strip_prefix("sha256:").ok_or_else(|| kernel_error!(ContextResolutionKernelErrorKind::AuthorityRefused, "authorized HCM fingerprint is malformed"))?;
    let observed_canonical = fs::read(repo_root.join(CONTEXT_RESOLUTION_AUTHORITY_PATH)).ok().map(|bytes| DefinitionFingerprint::from_bytes(&bytes).to_string());
    let mut record = json!({
        "schema_id":"handbook.context-resolution-publication-quarantine",
        "schema_version":"1.0",
        "repository_identity_fingerprint":registry.state["repository_identity_fingerprint"],
        "result_registry_state_ref":registry.state_ref,
        "result_registry_state_fingerprint":registry.state_fingerprint,
        "result_transition_ref":registry.head_transition_ref,
        "result_transition_fingerprint":registry.head_transition_fingerprint,
        "outer_artifact_fingerprint":authorized_outer_fingerprint,
        "observed_canonical_artifact_fingerprint":observed_canonical,
        "candidate_ref":candidate_ref,
        "candidate_fingerprint":candidate_fingerprint,
        "idempotency_key":format!("hcm32crpub_{outer_hex}"),
        "journal_transaction_id":journal_transaction_id,
        "phase":phase,
        "reason":reason,
        "resolution":"open",
        "result_transaction_ref":null,
        "result_transaction_fingerprint":null
    });
    let record_fingerprint = DefinitionFingerprint::from_json_value(&record).map_err(|_| kernel_error!(ContextResolutionKernelErrorKind::AuthorityRefused, "HCM quarantine could not be fingerprinted"))?;
    record["record_fingerprint"] = json!(record_fingerprint.as_str());
    store.record_hcm_publication_quarantine(&registry.head_transition_fingerprint, &record).map_err(|_| kernel_error!(ContextResolutionKernelErrorKind::AuthorityRefused, "HCM quarantine could not be committed"))?;
    Err(kernel_error!(ContextResolutionKernelErrorKind::AuthorityRefused, format!("HCM publication is authorized without a committed generic journal: {reason}")))
}

fn load_authority_snapshot(
    repo_root: &Path,
    binding_kind_ref: &str,
    binding_instance_id: &str,
    expected_artifact_fingerprint: &str,
) -> Result<CurrentAuthorityWitness, ContextResolutionKernelError> {
    if binding_kind_ref != CONTEXT_RESOLUTION_AUTHORITY_KIND_REF
        || binding_instance_id != CONTEXT_RESOLUTION_AUTHORITY_INSTANCE_ID
    {
        return Err(kernel_error!(
            ContextResolutionKernelErrorKind::InvalidInput,
            "authority target is outside the private HCM-3.2 compatibility tuple"
        ));
    }
    let authority = ArtifactRepositoryAuthorityGuardV1::acquire(repo_root).map_err(|_| {
        kernel_error!(
            ContextResolutionKernelErrorKind::AuthorityRefused,
            "repository authority guard refused"
        )
    })?;
    let store = GenericArtifactLineageStoreV1::new(repo_root);
    detect_authorized_publication_gap(repo_root, expected_artifact_fingerprint, &store)?;
    store.evaluate_committed_read_with(
        authority.repository_identity_fingerprint().as_str(),
        crate::artifact_mutation::HCM_2_3_OWNER_SUBJECT_FINGERPRINT,
        |_| {
            kernel_error!(
                ContextResolutionKernelErrorKind::StaleAuthority,
                "generic artifact recovery refused authority binding read"
            )
        },
        |intent| {
            crate::artifact_mutation::validate_persisted_intent_authority(
                repo_root, &authority, intent,
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
                    "repository authority changed during binding read"
                )
            })?;
            let repository = ArtifactRepositoryV1::open_under_authority(repo_root, &authority)
                .map_err(|_| {
                    kernel_error!(
                        ContextResolutionKernelErrorKind::AuthorityRefused,
                        "artifact repository could not resolve authority binding"
                    )
                })?;
            let kind = ExactDefinitionRef::parse(binding_kind_ref).map_err(|_| {
                kernel_error!(
                    ContextResolutionKernelErrorKind::InvalidInput,
                    "authority binding kind ref is invalid"
                )
            })?;
            let instance = SymbolicId::parse(binding_instance_id).map_err(|_| {
                kernel_error!(
                    ContextResolutionKernelErrorKind::InvalidInput,
                    "authority binding instance ID is invalid"
                )
            })?;
            let operation = repository
                .operation_context_under_lock(&kind, &instance)
                .map_err(|_| {
                    kernel_error!(
                        ContextResolutionKernelErrorKind::AuthorityRefused,
                        "authority binding operation context is unavailable"
                    )
                })?;
            let target = crate::artifact_repository::ArtifactTargetV1::parse(
                binding_kind_ref,
                binding_instance_id,
            )
            .map_err(|_| {
                kernel_error!(
                    ContextResolutionKernelErrorKind::InvalidInput,
                    "authority binding target is invalid"
                )
            })?;
            if operation.schema_ref().as_str() != CONTEXT_RESOLUTION_AUTHORITY_SCHEMA_REF
                || operation
                    .intake_definition_ref()
                    .map(ExactDefinitionRef::as_str)
                    != Some(CONTEXT_RESOLUTION_AUTHORITY_INTAKE_REF)
                || repository.canonical_path_under_lock(&target).map_err(|_| {
                    kernel_error!(
                        ContextResolutionKernelErrorKind::AuthorityRefused,
                        "authority canonical path is unavailable"
                    )
                })? != CONTEXT_RESOLUTION_AUTHORITY_PATH
            {
                return Err(kernel_error!(
                    ContextResolutionKernelErrorKind::StaleAuthority,
                    "authority compatibility tuple changed"
                ));
            }
            let artifact = repository.read_under_lock(&kind, &instance).map_err(|_| {
                kernel_error!(
                    ContextResolutionKernelErrorKind::AuthorityRefused,
                    "authority binding artifact read was refused"
                )
            })?;
            if artifact.artifact_fingerprint.as_str() != expected_artifact_fingerprint {
                return Err(kernel_error!(
                    ContextResolutionKernelErrorKind::StaleAuthority,
                    "authority binding raw artifact fingerprint mismatch"
                ));
            }
            let (committed, committed_intent) = store
                .read_committed_authoritative_with_intent_during_evaluation(
                    CONTEXT_RESOLUTION_AUTHORITY_PATH,
                    expected_artifact_fingerprint,
                )
                .map_err(|_| {
                    kernel_error!(
                        ContextResolutionKernelErrorKind::AuthorityRefused,
                        "authority artifact is not an exact committed promotion"
                    )
                })?;
            if committed.len() > MAX_OUTER_BYTES
                || DefinitionFingerprint::from_bytes(&committed).as_str()
                    != expected_artifact_fingerprint
                || crate::canonical_yaml::parse_canonical_yaml(&committed)
                    .ok()
                    .as_ref()
                    != Some(&artifact.content)
            {
                return Err(kernel_error!(
                    ContextResolutionKernelErrorKind::StaleAuthority,
                    "committed authority artifact bytes are not exact"
                ));
            }
            let capsule = validate_capsule_value(&artifact.content).map_err(|detail| {
                kernel_error!(ContextResolutionKernelErrorKind::InvalidInput, detail)
            })?;
            validate_capsule_repository_bindings(repo_root, &capsule).map_err(|detail| {
                kernel_error!(ContextResolutionKernelErrorKind::StaleAuthority, detail)
            })?;
            let publisher = publisher_authority_from_committed_intent(
                &committed_intent,
                expected_artifact_fingerprint,
            )
            .map_err(|detail| {
                kernel_error!(ContextResolutionKernelErrorKind::AuthorityRefused, detail)
            })?;
            let pending = verify_publisher_authority(
                repo_root,
                capsule,
                expected_artifact_fingerprint,
                &publisher,
                true,
            )
            .map_err(|detail| {
                kernel_error!(ContextResolutionKernelErrorKind::StaleAuthority, detail)
            })?
            .ok_or_else(|| {
                kernel_error!(
                    ContextResolutionKernelErrorKind::StaleAuthority,
                    "current publisher validation did not produce an operational witness"
                )
            })?;
            Ok(CurrentAuthorityWitness {
                repo_root: repo_root.to_path_buf(),
                expected_outer_fingerprint: expected_artifact_fingerprint.to_owned(),
                pending,
            })
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
pub struct ContextResolutionEnvelope { exact_binding: ContextResolutionExactBinding, _objective_ref: String, resolved_profile: ContextResolutionExactBinding, resolution_stack: ContextResolutionExactBinding, active_level_id: String, dimensions: ContextResolutionDimensions, _constraint_inputs: Vec<ContextResolutionExactBinding>, mutation_allow_layers: Vec<Vec<ContextResolutionMutationRule>>, mutation_denies: Vec<ContextResolutionMutationRule>, _escalation_triggers: Vec<ContextResolutionExactBinding>, authority: Arc<Value>, authority_admission: ContextResolutionAuthorityAdmission, stack: ContextResolutionStackDefinition }

#[allow(dead_code)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct ProjectionAuthorityView {
    envelope: ContextResolutionExactBinding,
    resolved_profile: ContextResolutionExactBinding,
    resolution_stack: ContextResolutionExactBinding,
    active_level_id: String,
    dimension_values: [String; 6],
    dimension_ranks: [u8; 6],
}

#[allow(dead_code)]
impl ProjectionAuthorityView {
    pub(crate) fn envelope(&self) -> &ContextResolutionExactBinding {
        &self.envelope
    }

    pub(crate) fn resolved_profile(&self) -> &ContextResolutionExactBinding {
        &self.resolved_profile
    }

    pub(crate) fn resolution_stack(&self) -> &ContextResolutionExactBinding {
        &self.resolution_stack
    }

    pub(crate) fn active_level(&self) -> &str {
        &self.active_level_id
    }

    pub(crate) fn dimension_values(&self) -> [&str; 6] {
        self.dimension_values.each_ref().map(String::as_str)
    }

    pub(crate) fn dimension_ranks(&self) -> [u8; 6] {
        self.dimension_ranks
    }
}

#[rustfmt::skip]
impl ContextResolutionEnvelope {
    #[rustfmt::skip]
    fn require_current(&self) -> Result<(), ContextResolutionKernelError> { self.authority_admission.require_current() }

    #[allow(dead_code)]
    pub(crate) fn projection_authority_view(
        &self,
    ) -> Result<ProjectionAuthorityView, ContextResolutionKernelError> {
        self.require_current()?;
        let dimension_values = self.dimensions.values().map(str::to_owned);
        let dimension_ranks = self
            .stack
            .ranks(&self.active_level_id, self.dimensions.values())
            .ok_or_else(|| {
                kernel_error!(
                    ContextResolutionKernelErrorKind::StaleAuthority,
                    "envelope dimensions are no longer in the exact Resolution stack",
                )
            })?;
        Ok(ProjectionAuthorityView {
            envelope: self.exact_binding.clone(),
            resolved_profile: self.resolved_profile.clone(),
            resolution_stack: self.resolution_stack.clone(),
            active_level_id: self.active_level_id.clone(),
            dimension_values,
            dimension_ranks,
        })
    }

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
        materialize_envelope!(input, creator.clone(), stack.clone(), Vec::new())
    }

    pub fn resolve_child(
        profile: &ResolvedInstanceProfile,
        stack: &ContextResolutionStackDefinition,
        parent: &Self,
        input: ContextResolutionEnvelopeInput,
        parent_authority: &ContextResolutionAuthorityAdmission,
        constraints: &[ContextResolutionAuthorityAdmission],
    ) -> Result<Self, ContextResolutionKernelError> {
        parent.require_current()?;
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
                &parent.authority_admission,
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
                &parent.authority_admission,
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
            parent_authority.clone(),
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
        if self.require_current().is_err() { return ContextResolutionMutationDecision::Indeterminate; }
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
        self.require_current()?;
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
        self.require_current()?;
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
pub struct ContextResolutionEscalationCandidate { class: String, trigger: ContextResolutionExactBinding, missing_condition: String, requested_authority_ref: String, evidence: Vec<ContextResolutionExactBinding>, current: ContextResolutionExactBinding, proposed: ContextResolutionExactBinding, resolved_profile: ContextResolutionExactBinding, resolution_stack: ContextResolutionExactBinding, authority: Arc<Value>, authority_admission: ContextResolutionAuthorityAdmission }

impl ContextResolutionEscalationCandidate {
    #[rustfmt::skip]
    fn require_current(&self) -> Result<(), ContextResolutionKernelError> { self.authority_admission.require_current() }

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
        current_envelope.require_current()?;
        candidate.require_current()?;
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
pub struct ContextResolutionPromotionRequest { request_id: String, exact_binding: ContextResolutionExactBinding, source_inputs: Vec<ContextResolutionExactBinding>, source_envelope: ContextResolutionExactBinding, _target_memory_horizon: String, target: ContextResolutionSemanticMemoryTarget, _requested_authority_ref: String, binding_fingerprint: String, authority_admission: ContextResolutionAuthorityAdmission }

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
        source_envelope.require_current()?;
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
            authority_admission: source_envelope.authority_admission.clone(),
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
        request.authority_admission.require_current()?;
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
        request.candidate.require_current()?;
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
        request.candidate.require_current()?;
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
        request.authority_admission.require_current()?;
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
        let request = self
            .promotion_requests
            .values()
            .find(|request| request.exact_binding == disposition.request)
            .ok_or_else(|| stale_request!("promotion"))?;
        request.authority_admission.require_current()?;
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
