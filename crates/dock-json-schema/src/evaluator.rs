use crate::canonical::canonical_bytes;
use crate::{
    ConformanceV1, JsonSchemaValidationInputV1, LogicalSchemaPathV1, NormalizedViolationV1,
    SchemaConformanceFactV1, ValidatorLimitsV1, ValidatorRefusalV1, ViolationCodeV1,
};
use jsonschema::error::ValidationErrorKind;
use jsonschema::{Draft, PatternOptions, ValidationError, Validator};
use serde_json::Value;
use std::collections::{BTreeMap, BTreeSet};

const DRAFT_2020_12: &str = "https://json-schema.org/draft/2020-12/schema";
const INTERNAL_URI_PREFIX: &str = "handbook+repo:///";
const MAX_CANDIDATE_PAYLOAD_BYTES: usize = 256 * 1_024;
const FANCY_REGEX_ROOT: &str =
    "definitions/schemas/handbook.schemas.artifacts.project-authority/1.1.0.schema.json";
const DATE_TIME_ROOTS: [&str; 4] = [
    "definitions/schemas/handbook.schemas.lifecycle.trigger-evidence/1.0.0.schema.json",
    "definitions/schemas/handbook.schemas.security.approver-registry-transition/1.0.0.schema.json",
    "definitions/schemas/handbook.schemas.security.authenticator-registration/1.0.0.schema.json",
    "definitions/schemas/handbook.schemas.security.authenticator-assertion/1.0.0.schema.json",
];

/// Evaluates exactly one JSON-compatible instance against one admitted offline closure.
pub fn evaluate_json_schema_v1(
    input: &JsonSchemaValidationInputV1,
) -> Result<SchemaConformanceFactV1, ValidatorRefusalV1> {
    input.verify_fingerprints()?;
    let closure = AdmittedClosure::prewalk(input)?;
    let validator = closure.build_validator(input.root_document().as_str())?;

    let mut violations = BTreeSet::new();
    let mut limit_exceeded = false;
    let mut observation_unavailable = false;
    for error in validator.iter_errors(input.instance().value()) {
        match normalize_error(&error) {
            Ok(violation) => {
                if violations.len() <= ValidatorLimitsV1::v1().max_violations() {
                    violations.insert(violation);
                }
                if violations.len() > ValidatorLimitsV1::v1().max_violations() {
                    limit_exceeded = true;
                }
            }
            Err(ValidatorRefusalV1::LimitExceeded) => limit_exceeded = true,
            Err(_) => observation_unavailable = true,
        }
    }
    if observation_unavailable {
        return Err(ValidatorRefusalV1::ObservationUnavailable);
    }
    if limit_exceeded {
        return Err(ValidatorRefusalV1::LimitExceeded);
    }

    let violations = violations.into_iter().collect::<Vec<_>>();
    let conformance = if violations.is_empty() {
        ConformanceV1::Conformant
    } else {
        ConformanceV1::Nonconformant
    };
    let fact = SchemaConformanceFactV1::from_validation_input(input, conformance, violations)
        .map_err(classify_post_admission_fact_error)?;
    if canonical_bytes(&fact)?.len() > MAX_CANDIDATE_PAYLOAD_BYTES {
        return Err(ValidatorRefusalV1::LimitExceeded);
    }
    Ok(fact)
}

fn classify_post_admission_fact_error(error: ValidatorRefusalV1) -> ValidatorRefusalV1 {
    match error {
        ValidatorRefusalV1::InputSubstituted => ValidatorRefusalV1::InputSubstituted,
        ValidatorRefusalV1::LimitExceeded => ValidatorRefusalV1::LimitExceeded,
        ValidatorRefusalV1::InternalRefusal => ValidatorRefusalV1::InternalRefusal,
        ValidatorRefusalV1::InputMalformed
        | ValidatorRefusalV1::SchemaUnsupported
        | ValidatorRefusalV1::RefRemote
        | ValidatorRefusalV1::RefMissing
        | ValidatorRefusalV1::ObservationUnavailable => ValidatorRefusalV1::InternalRefusal,
    }
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
struct SchemaLocation {
    document: String,
    pointer: String,
}

#[derive(Clone)]
struct LocalReference {
    source_pointer: String,
    target_document: String,
    target_pointer: String,
}

struct AdmittedDocument {
    value: Value,
    references: Vec<LocalReference>,
    schema_pointers: BTreeSet<String>,
    schema_children: BTreeMap<String, Vec<String>>,
}

struct AdmittedClosure {
    documents: BTreeMap<String, AdmittedDocument>,
}

impl AdmittedClosure {
    fn prewalk(input: &JsonSchemaValidationInputV1) -> Result<Self, ValidatorRefusalV1> {
        let mut documents = BTreeMap::new();
        for member in input.documents() {
            validate_document_root(member.schema())?;
            jsonschema::draft202012::meta::validate(member.schema())
                .map_err(|_| ValidatorRefusalV1::SchemaUnsupported)?;
            let mut profile = SchemaProfile::default();
            validate_schema_node(
                member.path().as_str(),
                member.schema(),
                "",
                true,
                &mut profile,
            )?;
            documents.insert(
                member.path().as_str().to_owned(),
                AdmittedDocument {
                    value: member.schema().clone(),
                    references: profile.references,
                    schema_pointers: profile.schema_pointers,
                    schema_children: profile.schema_children,
                },
            );
        }
        let mut closure = Self { documents };
        closure.resolve_references()?;
        closure.validate_reference_graph()?;
        Ok(closure)
    }

    fn resolve_references(&mut self) -> Result<(), ValidatorRefusalV1> {
        let paths = self.documents.keys().cloned().collect::<BTreeSet<_>>();
        for (document_path, document) in &mut self.documents {
            for reference in &mut document.references {
                let (target_document, target_pointer) =
                    parse_local_reference(document_path, &reference.target_document)?;
                if !paths.contains(&target_document) {
                    return Err(ValidatorRefusalV1::RefMissing);
                }
                reference.target_document = target_document;
                reference.target_pointer = target_pointer;
            }
        }
        for document in self.documents.values() {
            for reference in &document.references {
                let target = self
                    .documents
                    .get(&reference.target_document)
                    .ok_or(ValidatorRefusalV1::RefMissing)?;
                if target.value.pointer(&reference.target_pointer).is_none()
                    || !target.schema_pointers.contains(&reference.target_pointer)
                {
                    return Err(ValidatorRefusalV1::RefMissing);
                }
            }
        }
        Ok(())
    }

    fn validate_reference_graph(&self) -> Result<(), ValidatorRefusalV1> {
        let mut visiting = BTreeSet::new();
        let mut longest_paths = BTreeMap::new();
        for (document, admitted) in &self.documents {
            for pointer in &admitted.schema_pointers {
                self.longest_reference_path(
                    SchemaLocation {
                        document: document.clone(),
                        pointer: pointer.clone(),
                    },
                    &mut visiting,
                    &mut longest_paths,
                )?;
            }
        }
        Ok(())
    }

    fn longest_reference_path(
        &self,
        location: SchemaLocation,
        visiting: &mut BTreeSet<SchemaLocation>,
        longest_paths: &mut BTreeMap<SchemaLocation, usize>,
    ) -> Result<usize, ValidatorRefusalV1> {
        if let Some(longest) = longest_paths.get(&location) {
            return Ok(*longest);
        }
        if !visiting.insert(location.clone()) {
            return Err(ValidatorRefusalV1::RefMissing);
        }
        let document = self
            .documents
            .get(&location.document)
            .ok_or(ValidatorRefusalV1::RefMissing)?;
        let mut longest = 0usize;
        for child in document
            .schema_children
            .get(&location.pointer)
            .into_iter()
            .flatten()
        {
            longest = longest.max(self.longest_reference_path(
                SchemaLocation {
                    document: location.document.clone(),
                    pointer: child.clone(),
                },
                visiting,
                longest_paths,
            )?);
        }
        for reference in document
            .references
            .iter()
            .filter(|reference| reference.source_pointer == location.pointer)
        {
            let target_longest = self.longest_reference_path(
                SchemaLocation {
                    document: reference.target_document.clone(),
                    pointer: reference.target_pointer.clone(),
                },
                visiting,
                longest_paths,
            )?;
            longest = longest.max(
                target_longest
                    .checked_add(1)
                    .ok_or(ValidatorRefusalV1::LimitExceeded)?,
            );
            if longest > ValidatorLimitsV1::v1().max_ref_depth() {
                return Err(ValidatorRefusalV1::LimitExceeded);
            }
        }
        visiting.remove(&location);
        longest_paths.insert(location, longest);
        Ok(longest)
    }

    fn build_validator(&self, root_path: &str) -> Result<Validator, ValidatorRefusalV1> {
        let mut registry = jsonschema::Registry::new().draft(Draft::Draft202012);
        let mut transformed = BTreeMap::new();
        for (path, document) in &self.documents {
            let value = transformed_document(document)?;
            registry = registry
                .add(internal_resource_uri(path)?, value.clone())
                .map_err(|_| ValidatorRefusalV1::InternalRefusal)?;
            transformed.insert(path.clone(), value);
        }
        let registry = registry
            .prepare()
            .map_err(|_| ValidatorRefusalV1::InternalRefusal)?;
        let root = transformed
            .get(root_path)
            .ok_or(ValidatorRefusalV1::InternalRefusal)?;
        let options = jsonschema::draft202012::options()
            .with_registry(&registry)
            .with_base_uri(internal_resource_uri(root_path)?);
        let result = if root_path == FANCY_REGEX_ROOT {
            options
                .with_pattern_options(PatternOptions::fancy_regex())
                .build(root)
        } else if DATE_TIME_ROOTS.contains(&root_path) {
            options
                .with_pattern_options(PatternOptions::regex())
                .should_validate_formats(true)
                .build(root)
        } else {
            options
                .with_pattern_options(PatternOptions::regex())
                .build(root)
        };
        result.map_err(|_| ValidatorRefusalV1::ObservationUnavailable)
    }
}

#[derive(Default)]
struct SchemaProfile {
    references: Vec<LocalReference>,
    schema_pointers: BTreeSet<String>,
    schema_children: BTreeMap<String, Vec<String>>,
}

fn validate_document_root(value: &Value) -> Result<(), ValidatorRefusalV1> {
    let object = value
        .as_object()
        .ok_or(ValidatorRefusalV1::SchemaUnsupported)?;
    if object.get("$schema").and_then(Value::as_str) != Some(DRAFT_2020_12) {
        return Err(ValidatorRefusalV1::SchemaUnsupported);
    }
    Ok(())
}

fn validate_schema_node(
    document_path: &str,
    node: &Value,
    pointer: &str,
    document_root: bool,
    profile: &mut SchemaProfile,
) -> Result<(), ValidatorRefusalV1> {
    profile.schema_pointers.insert(pointer.to_owned());
    profile
        .schema_children
        .entry(pointer.to_owned())
        .or_default();
    if node.is_boolean() {
        return if document_root {
            Err(ValidatorRefusalV1::SchemaUnsupported)
        } else {
            Ok(())
        };
    }
    let object = node
        .as_object()
        .ok_or(ValidatorRefusalV1::SchemaUnsupported)?;
    for (keyword, value) in object {
        if is_unsupported_identifier(keyword)
            || (keyword == "$schema" && !document_root)
            || (!is_allowed_keyword(keyword)
                && !is_allowed_date_time_format(document_path, keyword, value))
        {
            return Err(ValidatorRefusalV1::SchemaUnsupported);
        }
        if keyword == "$ref" {
            let reference = value.as_str().ok_or(ValidatorRefusalV1::RefRemote)?;
            profile.references.push(LocalReference {
                source_pointer: pointer.to_owned(),
                target_document: reference.to_owned(),
                target_pointer: String::new(),
            });
        }
    }

    for keyword in [
        "not",
        "if",
        "then",
        "else",
        "contains",
        "items",
        "additionalProperties",
        "propertyNames",
        "unevaluatedItems",
        "unevaluatedProperties",
    ] {
        if let Some(child) = object.get(keyword) {
            let child_pointer = pointer_join(pointer, keyword);
            profile
                .schema_children
                .entry(pointer.to_owned())
                .or_default()
                .push(child_pointer.clone());
            validate_schema_node(document_path, child, &child_pointer, false, profile)?;
        }
    }
    for keyword in ["allOf", "anyOf", "oneOf", "prefixItems"] {
        if let Some(children) = object.get(keyword).and_then(Value::as_array) {
            for (index, child) in children.iter().enumerate() {
                let child_pointer = format!("{pointer}/{keyword}/{index}");
                profile
                    .schema_children
                    .entry(pointer.to_owned())
                    .or_default()
                    .push(child_pointer.clone());
                validate_schema_node(document_path, child, &child_pointer, false, profile)?;
            }
        }
    }
    for keyword in [
        "$defs",
        "properties",
        "patternProperties",
        "dependentSchemas",
    ] {
        if let Some(children) = object.get(keyword).and_then(Value::as_object) {
            for (name, child) in children {
                let child_pointer =
                    format!("{pointer}/{keyword}/{}", escape_json_pointer_token(name));
                profile
                    .schema_children
                    .entry(pointer.to_owned())
                    .or_default()
                    .push(child_pointer.clone());
                validate_schema_node(document_path, child, &child_pointer, false, profile)?;
            }
        }
    }
    Ok(())
}

fn parse_local_reference(
    current_document: &str,
    reference: &str,
) -> Result<(String, String), ValidatorRefusalV1> {
    if reference.contains('?')
        || reference.contains('\\')
        || reference.contains('%')
        || reference.starts_with('/')
        || reference.starts_with("//")
        || reference.matches('#').count() > 1
    {
        return Err(ValidatorRefusalV1::RefRemote);
    }
    let (document_part, pointer) = reference.split_once('#').unwrap_or((reference, ""));
    if document_part.contains(':')
        || (!document_part.is_empty()
            && document_part
                .split('/')
                .any(|segment| segment.is_empty() || segment == "." || segment == ".."))
    {
        return Err(ValidatorRefusalV1::RefRemote);
    }
    validate_reference_pointer(pointer)?;
    let target = if document_part.is_empty() {
        current_document.to_owned()
    } else {
        let parent = current_document
            .rsplit_once('/')
            .map_or("", |(parent, _)| parent);
        let joined = checked_join(parent, document_part)?;
        LogicalSchemaPathV1::parse(&joined)
            .map_err(|error| match error {
                ValidatorRefusalV1::LimitExceeded => ValidatorRefusalV1::LimitExceeded,
                _ => ValidatorRefusalV1::RefRemote,
            })?
            .as_str()
            .to_owned()
    };
    Ok((target, pointer.to_owned()))
}

fn validate_reference_pointer(pointer: &str) -> Result<(), ValidatorRefusalV1> {
    if pointer.len() > ValidatorLimitsV1::v1().max_pointer_bytes() {
        return Err(ValidatorRefusalV1::LimitExceeded);
    }
    if !pointer.is_empty() && !pointer.starts_with('/') {
        return Err(ValidatorRefusalV1::RefRemote);
    }
    let mut bytes = pointer.bytes();
    while let Some(byte) = bytes.next() {
        if byte == b'~' && !matches!(bytes.next(), Some(b'0' | b'1')) {
            return Err(ValidatorRefusalV1::RefRemote);
        }
    }
    Ok(())
}

fn transformed_document(document: &AdmittedDocument) -> Result<Value, ValidatorRefusalV1> {
    let mut value = document.value.clone();
    for reference in &document.references {
        let mut target = internal_resource_uri(&reference.target_document)?;
        if !reference.target_pointer.is_empty() {
            target.push('#');
            target.push_str(&percent_encode_uri_path_like(&reference.target_pointer)?);
        }
        let source = pointer_join(&reference.source_pointer, "$ref");
        let slot = value
            .pointer_mut(&source)
            .ok_or(ValidatorRefusalV1::InternalRefusal)?;
        *slot = Value::String(target);
    }
    Ok(value)
}

fn normalize_error(
    error: &ValidationError<'_>,
) -> Result<NormalizedViolationV1, ValidatorRefusalV1> {
    let code = match error.kind() {
        ValidationErrorKind::AdditionalItems { .. } => ViolationCodeV1::AdditionalItems,
        ValidationErrorKind::AdditionalProperties { .. } => ViolationCodeV1::AdditionalProperties,
        ValidationErrorKind::AnyOf { .. } => ViolationCodeV1::AnyOf,
        ValidationErrorKind::Constant { .. } => ViolationCodeV1::Const,
        ValidationErrorKind::Contains => ViolationCodeV1::Contains,
        ValidationErrorKind::Enum { .. } => ViolationCodeV1::Enum,
        ValidationErrorKind::ExclusiveMaximum { .. } => ViolationCodeV1::ExclusiveMaximum,
        ValidationErrorKind::ExclusiveMinimum { .. } => ViolationCodeV1::ExclusiveMinimum,
        ValidationErrorKind::FalseSchema => ViolationCodeV1::FalseSchema,
        ValidationErrorKind::Format { .. } => ViolationCodeV1::Format,
        ValidationErrorKind::MaxItems { .. } => ViolationCodeV1::MaxItems,
        ValidationErrorKind::Maximum { .. } => ViolationCodeV1::Maximum,
        ValidationErrorKind::MaxLength { .. } => ViolationCodeV1::MaxLength,
        ValidationErrorKind::MaxProperties { .. } => ViolationCodeV1::MaxProperties,
        ValidationErrorKind::MinItems { .. } => ViolationCodeV1::MinItems,
        ValidationErrorKind::Minimum { .. } => ViolationCodeV1::Minimum,
        ValidationErrorKind::MinLength { .. } => ViolationCodeV1::MinLength,
        ValidationErrorKind::MinProperties { .. } => ViolationCodeV1::MinProperties,
        ValidationErrorKind::MultipleOf { .. } => ViolationCodeV1::MultipleOf,
        ValidationErrorKind::Not { .. } => ViolationCodeV1::Not,
        ValidationErrorKind::OneOfMultipleValid { .. } => ViolationCodeV1::OneOfMultipleValid,
        ValidationErrorKind::OneOfNotValid { .. } => ViolationCodeV1::OneOfNotValid,
        ValidationErrorKind::Pattern { .. } => ViolationCodeV1::Pattern,
        ValidationErrorKind::PropertyNames { .. } => ViolationCodeV1::PropertyNames,
        ValidationErrorKind::Required { .. } => ViolationCodeV1::Required,
        ValidationErrorKind::Type { .. } => ViolationCodeV1::Type,
        ValidationErrorKind::UnevaluatedItems { .. } => ViolationCodeV1::UnevaluatedItems,
        ValidationErrorKind::UnevaluatedProperties { .. } => ViolationCodeV1::UnevaluatedProperties,
        ValidationErrorKind::UniqueItems => ViolationCodeV1::UniqueItems,
        ValidationErrorKind::BacktrackLimitExceeded { .. }
        | ValidationErrorKind::RegexEngineFailure { .. }
        | ValidationErrorKind::ContentEncoding { .. }
        | ValidationErrorKind::ContentMediaType { .. }
        | ValidationErrorKind::Custom { .. }
        | ValidationErrorKind::FromUtf8 { .. }
        | ValidationErrorKind::Referencing(_) => {
            return Err(ValidatorRefusalV1::ObservationUnavailable)
        }
    };
    let absolute = error
        .absolute_keyword_location()
        .ok_or(ValidatorRefusalV1::ObservationUnavailable)?;
    let (document, schema_pointer) = parse_internal_location(absolute.as_str())?;
    NormalizedViolationV1::new(
        code,
        error.instance_path().as_str(),
        LogicalSchemaPathV1::parse(&document)
            .map_err(|_| ValidatorRefusalV1::ObservationUnavailable)?,
        &schema_pointer,
    )
}

fn parse_internal_location(uri: &str) -> Result<(String, String), ValidatorRefusalV1> {
    let encoded = uri
        .strip_prefix(INTERNAL_URI_PREFIX)
        .ok_or(ValidatorRefusalV1::ObservationUnavailable)?;
    let (path, pointer) = encoded.split_once('#').unwrap_or((encoded, ""));
    let path = percent_decode_utf8(path)?;
    let pointer = percent_decode_utf8(pointer)?;
    validate_reference_pointer(&pointer).map_err(|error| match error {
        ValidatorRefusalV1::LimitExceeded => ValidatorRefusalV1::LimitExceeded,
        _ => ValidatorRefusalV1::ObservationUnavailable,
    })?;
    Ok((path, pointer))
}

fn internal_resource_uri(path: &str) -> Result<String, ValidatorRefusalV1> {
    let encoded = percent_encode_uri_path_like(path)?;
    let capacity = checked_total_lengths(&[INTERNAL_URI_PREFIX.len(), encoded.len()])?;
    let mut uri = String::with_capacity(capacity);
    uri.push_str(INTERNAL_URI_PREFIX);
    uri.push_str(&encoded);
    Ok(uri)
}

fn percent_encode_uri_path_like(value: &str) -> Result<String, ValidatorRefusalV1> {
    const HEX: &[u8; 16] = b"0123456789ABCDEF";
    let capacity = value
        .len()
        .checked_mul(3)
        .ok_or(ValidatorRefusalV1::LimitExceeded)?;
    let mut encoded = String::with_capacity(capacity);
    for byte in value.bytes() {
        if byte.is_ascii_alphanumeric() || matches!(byte, b'-' | b'.' | b'_' | b'~' | b'/') {
            encoded.push(char::from(byte));
        } else {
            encoded.push('%');
            encoded.push(char::from(HEX[usize::from(byte >> 4)]));
            encoded.push(char::from(HEX[usize::from(byte & 0x0f)]));
        }
    }
    Ok(encoded)
}

fn percent_decode_utf8(value: &str) -> Result<String, ValidatorRefusalV1> {
    let bytes = value.as_bytes();
    let mut decoded = Vec::with_capacity(bytes.len());
    let mut index = 0;
    while index < bytes.len() {
        if bytes[index] == b'%' {
            if index + 2 >= bytes.len() {
                return Err(ValidatorRefusalV1::ObservationUnavailable);
            }
            let high = hex_value(bytes[index + 1])?;
            let low = hex_value(bytes[index + 2])?;
            decoded.push((high << 4) | low);
            index += 3;
        } else {
            decoded.push(bytes[index]);
            index += 1;
        }
    }
    String::from_utf8(decoded).map_err(|_| ValidatorRefusalV1::ObservationUnavailable)
}

fn hex_value(byte: u8) -> Result<u8, ValidatorRefusalV1> {
    match byte {
        b'0'..=b'9' => Ok(byte - b'0'),
        b'a'..=b'f' => Ok(byte - b'a' + 10),
        b'A'..=b'F' => Ok(byte - b'A' + 10),
        _ => Err(ValidatorRefusalV1::ObservationUnavailable),
    }
}

fn checked_join(parent: &str, child: &str) -> Result<String, ValidatorRefusalV1> {
    let capacity = checked_total_lengths(&[parent.len(), 1, child.len()])?;
    let mut joined = String::with_capacity(capacity);
    if !parent.is_empty() {
        joined.push_str(parent);
        joined.push('/');
    }
    joined.push_str(child);
    Ok(joined)
}

fn checked_total_lengths(lengths: &[usize]) -> Result<usize, ValidatorRefusalV1> {
    lengths.iter().try_fold(0usize, |total, length| {
        total
            .checked_add(*length)
            .ok_or(ValidatorRefusalV1::LimitExceeded)
    })
}

fn pointer_join(pointer: &str, token: &str) -> String {
    format!("{pointer}/{}", escape_json_pointer_token(token))
}

fn escape_json_pointer_token(token: &str) -> String {
    token.replace('~', "~0").replace('/', "~1")
}

fn is_unsupported_identifier(keyword: &str) -> bool {
    matches!(
        keyword,
        "$id" | "$anchor" | "$dynamicAnchor" | "$recursiveAnchor" | "$recursiveRef" | "$dynamicRef"
    )
}

fn is_allowed_keyword(keyword: &str) -> bool {
    matches!(
        keyword,
        "$schema"
            | "$ref"
            | "$defs"
            | "$comment"
            | "allOf"
            | "anyOf"
            | "oneOf"
            | "not"
            | "if"
            | "then"
            | "else"
            | "dependentSchemas"
            | "prefixItems"
            | "items"
            | "contains"
            | "properties"
            | "patternProperties"
            | "additionalProperties"
            | "propertyNames"
            | "unevaluatedItems"
            | "unevaluatedProperties"
            | "type"
            | "enum"
            | "const"
            | "multipleOf"
            | "maximum"
            | "exclusiveMaximum"
            | "minimum"
            | "exclusiveMinimum"
            | "maxLength"
            | "minLength"
            | "pattern"
            | "maxItems"
            | "minItems"
            | "uniqueItems"
            | "maxContains"
            | "minContains"
            | "maxProperties"
            | "minProperties"
            | "required"
            | "dependentRequired"
            | "title"
            | "description"
            | "default"
            | "deprecated"
            | "readOnly"
            | "writeOnly"
            | "examples"
    )
}

fn is_allowed_date_time_format(document: &str, keyword: &str, value: &Value) -> bool {
    keyword == "format"
        && value.as_str() == Some("date-time")
        && DATE_TIME_ROOTS.contains(&document)
}

#[cfg(test)]
mod tests {
    use super::{checked_total_lengths, classify_post_admission_fact_error};
    use crate::ValidatorRefusalV1;

    #[test]
    fn arithmetic_overflow_maps_to_the_closed_limit_refusal() {
        assert_eq!(
            checked_total_lengths(&[usize::MAX, 1]),
            Err(ValidatorRefusalV1::LimitExceeded)
        );
    }

    #[test]
    fn post_admission_shape_invariants_map_to_internal_refusal() {
        assert_eq!(
            classify_post_admission_fact_error(ValidatorRefusalV1::InputMalformed),
            ValidatorRefusalV1::InternalRefusal
        );
        assert_eq!(
            classify_post_admission_fact_error(ValidatorRefusalV1::InputSubstituted),
            ValidatorRefusalV1::InputSubstituted
        );
        assert_eq!(
            classify_post_admission_fact_error(ValidatorRefusalV1::LimitExceeded),
            ValidatorRefusalV1::LimitExceeded
        );
    }
}
