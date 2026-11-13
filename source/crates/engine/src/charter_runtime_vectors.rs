use crate::DefinitionFingerprint;
use serde::Deserialize;
use serde_json::Value;
use std::collections::BTreeMap;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RuntimeRecordVectorReport {
    pub vector_count: usize,
    pub cross_record_binding_count: usize,
    pub external_reference_binding_count: usize,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RuntimeRecordVectorError {
    detail: String,
}

impl RuntimeRecordVectorError {
    fn new(detail: impl Into<String>) -> Self {
        Self {
            detail: detail.into(),
        }
    }

    pub fn detail(&self) -> &str {
        &self.detail
    }
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Fixture {
    schema_id: String,
    schema_version: String,
    normalization: String,
    id_rule: String,
    vectors: Vec<RecordVector>,
    cross_record_contract: Value,
    cross_record_bindings: Vec<CrossRecordBinding>,
    external_reference_contract: Value,
    external_reference_bindings: Vec<ExternalReferenceBinding>,
    candidate_1_3_exact_result_contract: Value,
    historical_candidate_1_2_contract: Value,
    exact_result_negative_vectors: Value,
    candidate_1_3_amendment_chain_contract: Value,
    author_inventory_contract: Value,
    author_inventory_boundary_vectors: Value,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct RecordVector {
    record_class: String,
    schema_contract: String,
    audit_only_excluded_fields: Vec<String>,
    fingerprint_field: String,
    id_field: Option<String>,
    id_location: String,
    expected_fingerprint: String,
    expected_id: String,
    fingerprint_preimage: Value,
    record: Value,
    candidate_subject_fingerprint_preimage: Option<Value>,
    expected_candidate_subject_fingerprint: Option<String>,
    expected_document_byte_length: Option<usize>,
    expected_document_sha256: Option<String>,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct CrossRecordBinding {
    source_record_class: String,
    source_field: String,
    target_record_class: String,
    expected_ref: String,
    declared_fingerprint_field: Option<String>,
    expected_target_id: String,
    expected_target_fingerprint: String,
    declared_document_sha256_field: Option<String>,
    expected_target_document_sha256: Option<String>,
    declared_byte_length_field: Option<String>,
    expected_target_byte_length: Option<usize>,
    transitive_exact_document_authority: Option<String>,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct ExternalReferenceBinding {
    source_record_class: String,
    source_field: String,
    expected_ref: String,
    producer_kind: String,
    expected_producer_fingerprint: String,
    declared_fingerprint_field: Option<String>,
}

pub fn validate_runtime_record_fingerprint_vectors(
    bytes: &[u8],
) -> Result<RuntimeRecordVectorReport, RuntimeRecordVectorError> {
    let fixture: Fixture = serde_json::from_slice(bytes).map_err(|error| {
        RuntimeRecordVectorError::new(format!(
            "runtime vector fixture is not closed JSON: {error}"
        ))
    })?;
    if fixture.schema_id != "hcm-2.2-runtime-record-fingerprint-vectors"
        || fixture.schema_version != "1.0"
        || fixture.normalization != "RFC8785-JCS-UTF8"
        || fixture.id_rule.trim().is_empty()
        || !fixture.cross_record_contract.is_object()
        || !fixture.external_reference_contract.is_object()
        || !fixture.candidate_1_3_exact_result_contract.is_object()
        || !fixture.historical_candidate_1_2_contract.is_object()
        || !fixture.exact_result_negative_vectors.is_array()
        || !fixture.candidate_1_3_amendment_chain_contract.is_object()
        || !fixture.author_inventory_contract.is_object()
        || !fixture.author_inventory_boundary_vectors.is_array()
    {
        return Err(RuntimeRecordVectorError::new(
            "runtime vector fixture identity or contract metadata is invalid",
        ));
    }

    let mut vectors = BTreeMap::new();
    for vector in &fixture.vectors {
        validate_vector(vector)?;
        if vector.schema_contract.trim().is_empty()
            || vectors
                .insert(vector.record_class.as_str(), vector)
                .is_some()
        {
            return Err(RuntimeRecordVectorError::new(
                "runtime vector classes must be unique and schema-bound",
            ));
        }
    }
    for binding in &fixture.cross_record_bindings {
        validate_cross_binding(binding, &vectors)?;
    }
    for binding in &fixture.external_reference_bindings {
        validate_external_binding(binding, &vectors)?;
    }

    Ok(RuntimeRecordVectorReport {
        vector_count: fixture.vectors.len(),
        cross_record_binding_count: fixture.cross_record_bindings.len(),
        external_reference_binding_count: fixture.external_reference_bindings.len(),
    })
}

fn validate_vector(vector: &RecordVector) -> Result<(), RuntimeRecordVectorError> {
    let fingerprint = DefinitionFingerprint::from_json_value(&vector.fingerprint_preimage)
        .map_err(|_| RuntimeRecordVectorError::new("runtime preimage cannot be canonicalized"))?
        .to_string();
    if fingerprint != vector.expected_fingerprint {
        return Err(RuntimeRecordVectorError::new(format!(
            "{} fingerprint does not recompute",
            vector.record_class
        )));
    }
    let expected_hex = vector
        .expected_fingerprint
        .strip_prefix("sha256:")
        .ok_or_else(|| {
            RuntimeRecordVectorError::new("expected fingerprint is not lowercase sha256")
        })?;
    if !vector.expected_id.ends_with(expected_hex) {
        return Err(RuntimeRecordVectorError::new(format!(
            "{} ID does not bind its fingerprint",
            vector.record_class
        )));
    }
    if field(&vector.record, &vector.fingerprint_field).and_then(Value::as_str)
        != Some(vector.expected_fingerprint.as_str())
    {
        return Err(RuntimeRecordVectorError::new(format!(
            "{} record fingerprint field disagrees",
            vector.record_class
        )));
    }
    match (&vector.id_field, vector.id_location.as_str()) {
        (Some(id_field), "record")
            if field(&vector.record, id_field).and_then(Value::as_str)
                == Some(vector.expected_id.as_str()) => {}
        (None, "derived_content_addressed_path") => {}
        _ => {
            return Err(RuntimeRecordVectorError::new(format!(
                "{} record ID field disagrees",
                vector.record_class
            )))
        }
    }

    let mut reconstructed = vector.record.clone();
    let object = reconstructed
        .as_object_mut()
        .ok_or_else(|| RuntimeRecordVectorError::new("runtime record must have an object root"))?;
    object.remove(&vector.fingerprint_field);
    if let Some(id_field) = &vector.id_field {
        object.remove(id_field);
    }
    for field in &vector.audit_only_excluded_fields {
        object.remove(field);
    }
    if reconstructed != vector.fingerprint_preimage {
        return Err(RuntimeRecordVectorError::new(format!(
            "{} record does not reproduce its exact preimage after exclusions",
            vector.record_class
        )));
    }
    match (
        &vector.candidate_subject_fingerprint_preimage,
        &vector.expected_candidate_subject_fingerprint,
    ) {
        (Some(preimage), Some(expected))
            if matches!(
                vector.record_class.as_str(),
                "candidate" | "candidate-amendment"
            ) =>
        {
            let observed = DefinitionFingerprint::from_json_value(preimage)
                .map_err(|_| {
                    RuntimeRecordVectorError::new(
                        "candidate subject preimage cannot be canonicalized",
                    )
                })?
                .to_string();
            if observed != *expected
                || field(&vector.record, "candidate_subject_fingerprint").and_then(Value::as_str)
                    != Some(expected.as_str())
            {
                return Err(RuntimeRecordVectorError::new(
                    "candidate subject fingerprint witness does not recompute",
                ));
            }
        }
        (None, None)
            if !matches!(
                vector.record_class.as_str(),
                "candidate" | "candidate-amendment"
            ) => {}
        _ => {
            return Err(RuntimeRecordVectorError::new(
                "candidate subject witness presence is not class-exact",
            ))
        }
    }
    match (
        vector.expected_document_byte_length,
        &vector.expected_document_sha256,
    ) {
        (Some(expected_length), Some(expected_sha256))
            if matches!(
                vector.record_class.as_str(),
                "lifecycle-validation-result" | "lifecycle-validation-result-amendment"
            ) =>
        {
            let mut document = serde_json_canonicalizer::to_vec(&vector.record).map_err(|_| {
                RuntimeRecordVectorError::new("runtime result document cannot be canonicalized")
            })?;
            document.push(b'\n');
            if document.len() != expected_length
                || DefinitionFingerprint::from_bytes(&document).as_str() != expected_sha256
            {
                return Err(RuntimeRecordVectorError::new(
                    "runtime result exact JCS-plus-LF witness does not recompute",
                ));
            }
        }
        (None, None)
            if !matches!(
                vector.record_class.as_str(),
                "lifecycle-validation-result" | "lifecycle-validation-result-amendment"
            ) => {}
        _ => {
            return Err(RuntimeRecordVectorError::new(
                "exact result document witness presence is not class-exact",
            ))
        }
    }
    Ok(())
}

fn validate_cross_binding(
    binding: &CrossRecordBinding,
    vectors: &BTreeMap<&str, &RecordVector>,
) -> Result<(), RuntimeRecordVectorError> {
    let source = vectors
        .get(binding.source_record_class.as_str())
        .ok_or_else(|| RuntimeRecordVectorError::new("cross-record source class is missing"))?;
    let target = vectors
        .get(binding.target_record_class.as_str())
        .ok_or_else(|| RuntimeRecordVectorError::new("cross-record target class is missing"))?;
    if target.expected_id != binding.expected_target_id
        || target.expected_fingerprint != binding.expected_target_fingerprint
        || field(&source.record, &binding.source_field).and_then(Value::as_str)
            != Some(binding.expected_ref.as_str())
    {
        return Err(RuntimeRecordVectorError::new(
            "cross-record binding target identity disagrees",
        ));
    }
    let basename = binding
        .expected_ref
        .rsplit('/')
        .next()
        .and_then(|name| name.strip_suffix(".json"))
        .ok_or_else(|| RuntimeRecordVectorError::new("cross-record ref is not a JSON path"))?;
    if basename != binding.expected_target_id {
        return Err(RuntimeRecordVectorError::new(
            "cross-record ref basename does not bind the target ID",
        ));
    }
    if let Some(declared) = &binding.declared_fingerprint_field {
        if field(&source.record, declared).and_then(Value::as_str)
            != Some(binding.expected_target_fingerprint.as_str())
        {
            return Err(RuntimeRecordVectorError::new(
                "cross-record declared fingerprint disagrees",
            ));
        }
    }
    match (
        &binding.declared_document_sha256_field,
        &binding.expected_target_document_sha256,
        &binding.declared_byte_length_field,
        binding.expected_target_byte_length,
    ) {
        (Some(digest_field), Some(expected_digest), Some(length_field), Some(expected_length)) => {
            if field(&source.record, digest_field).and_then(Value::as_str)
                != Some(expected_digest.as_str())
                || field(&source.record, length_field).and_then(Value::as_u64)
                    != Some(expected_length as u64)
                || target.expected_document_sha256.as_deref() != Some(expected_digest.as_str())
                || target.expected_document_byte_length != Some(expected_length)
            {
                return Err(RuntimeRecordVectorError::new(
                    "cross-record exact result document binding disagrees",
                ));
            }
        }
        (None, None, None, None) => {}
        _ => {
            return Err(RuntimeRecordVectorError::new(
                "cross-record exact result document witness is incomplete",
            ))
        }
    }
    if binding
        .transitive_exact_document_authority
        .as_ref()
        .is_some_and(|description| description.trim().is_empty())
    {
        return Err(RuntimeRecordVectorError::new(
            "transitive exact-document authority description is empty",
        ));
    }
    Ok(())
}

fn validate_external_binding(
    binding: &ExternalReferenceBinding,
    vectors: &BTreeMap<&str, &RecordVector>,
) -> Result<(), RuntimeRecordVectorError> {
    let source = vectors
        .get(binding.source_record_class.as_str())
        .ok_or_else(|| RuntimeRecordVectorError::new("external binding source class is missing"))?;
    if binding.producer_kind.trim().is_empty()
        || field(&source.record, &binding.source_field).and_then(Value::as_str)
            != Some(binding.expected_ref.as_str())
        || DefinitionFingerprint::parse(&binding.expected_producer_fingerprint).is_err()
    {
        return Err(RuntimeRecordVectorError::new(
            "external producer binding identity disagrees",
        ));
    }
    if let Some(declared) = &binding.declared_fingerprint_field {
        if field(&source.record, declared).and_then(Value::as_str)
            != Some(binding.expected_producer_fingerprint.as_str())
        {
            return Err(RuntimeRecordVectorError::new(
                "external producer declared fingerprint disagrees",
            ));
        }
    }
    Ok(())
}

fn field<'a>(value: &'a Value, accessor: &str) -> Option<&'a Value> {
    let mut current = value;
    for segment in accessor.split('.') {
        let (key, index) = if let Some((key, suffix)) = segment.split_once('[') {
            let index = suffix.strip_suffix(']')?.parse::<usize>().ok()?;
            (key, Some(index))
        } else {
            (segment, None)
        };
        current = current.get(key)?;
        if let Some(index) = index {
            current = current.get(index)?;
        }
    }
    Some(current)
}
