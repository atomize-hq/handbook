use crate::canonical::canonical_fingerprint;
use crate::{
    CanonicalFingerprintV1, ExactSchemaBindingV1, JsonSchemaValidationInputV1, LogicalSchemaPathV1,
    ValidatorLimitsV1, ValidatorRefusalV1,
};
use serde::de::Error as _;
use serde::{Deserialize, Deserializer, Serialize};
use std::cmp::Ordering;

/// The closed normalized JSON Schema violation vocabulary.
#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ViolationCodeV1 {
    AdditionalItems,
    AdditionalProperties,
    AnyOf,
    Const,
    Contains,
    Enum,
    ExclusiveMaximum,
    ExclusiveMinimum,
    FalseSchema,
    Format,
    MaxItems,
    Maximum,
    MaxLength,
    MaxProperties,
    MinItems,
    Minimum,
    MinLength,
    MinProperties,
    MultipleOf,
    Not,
    OneOfMultipleValid,
    OneOfNotValid,
    Pattern,
    PropertyNames,
    Required,
    Type,
    UnevaluatedItems,
    UnevaluatedProperties,
    UniqueItems,
}

/// One bounded normalized violation with no library display text or rejected value.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct NormalizedViolationV1 {
    code: ViolationCodeV1,
    instance_pointer: String,
    schema_document: LogicalSchemaPathV1,
    schema_pointer: String,
}

impl NormalizedViolationV1 {
    pub fn new(
        code: ViolationCodeV1,
        instance_pointer: &str,
        schema_document: LogicalSchemaPathV1,
        schema_pointer: &str,
    ) -> Result<Self, ValidatorRefusalV1> {
        validate_pointer(instance_pointer)?;
        validate_pointer(schema_pointer)?;
        Ok(Self {
            code,
            instance_pointer: instance_pointer.to_owned(),
            schema_document,
            schema_pointer: schema_pointer.to_owned(),
        })
    }

    pub fn code(&self) -> ViolationCodeV1 {
        self.code
    }
    pub fn instance_pointer(&self) -> &str {
        &self.instance_pointer
    }
    pub fn schema_document(&self) -> &LogicalSchemaPathV1 {
        &self.schema_document
    }
    pub fn schema_pointer(&self) -> &str {
        &self.schema_pointer
    }
}

impl Ord for NormalizedViolationV1 {
    fn cmp(&self, other: &Self) -> Ordering {
        (
            &self.instance_pointer,
            &self.schema_document,
            &self.schema_pointer,
            self.code,
        )
            .cmp(&(
                &other.instance_pointer,
                &other.schema_document,
                &other.schema_pointer,
                other.code,
            ))
    }
}

impl PartialOrd for NormalizedViolationV1 {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<'de> Deserialize<'de> for NormalizedViolationV1 {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        #[serde(deny_unknown_fields)]
        struct Raw {
            code: ViolationCodeV1,
            instance_pointer: String,
            schema_document: LogicalSchemaPathV1,
            schema_pointer: String,
        }
        let raw = Raw::deserialize(deserializer)?;
        Self::new(
            raw.code,
            &raw.instance_pointer,
            raw.schema_document,
            &raw.schema_pointer,
        )
        .map_err(D::Error::custom)
    }
}

/// Closed observational fact outcome; it carries no verdict or gate semantics.
#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ConformanceV1 {
    Conformant,
    Nonconformant,
}

/// One complete normalized schema-conformance fact.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct SchemaConformanceFactV1 {
    root_schema: ExactSchemaBindingV1,
    root_document: LogicalSchemaPathV1,
    root_document_fingerprint: CanonicalFingerprintV1,
    closure_fingerprint: CanonicalFingerprintV1,
    instance_fingerprint: CanonicalFingerprintV1,
    closure_document_count: u16,
    conformance: ConformanceV1,
    violations: Vec<NormalizedViolationV1>,
}

impl SchemaConformanceFactV1 {
    pub fn from_validation_input(
        input: &JsonSchemaValidationInputV1,
        conformance: ConformanceV1,
        mut violations: Vec<NormalizedViolationV1>,
    ) -> Result<Self, ValidatorRefusalV1> {
        input.verify_fingerprints()?;
        violations.sort();
        violations.dedup();
        if violations
            .iter()
            .any(|violation| !input.contains_document(&violation.schema_document))
        {
            return Err(ValidatorRefusalV1::InputMalformed);
        }
        Self::new(
            input.root_schema().clone(),
            input.root_document().clone(),
            input.root_document_fingerprint().clone(),
            input.closure_fingerprint().clone(),
            input.instance().instance_fingerprint().clone(),
            input.documents().len(),
            conformance,
            violations,
            true,
        )
    }

    #[allow(clippy::too_many_arguments)]
    fn new(
        root_schema: ExactSchemaBindingV1,
        root_document: LogicalSchemaPathV1,
        root_document_fingerprint: CanonicalFingerprintV1,
        closure_fingerprint: CanonicalFingerprintV1,
        instance_fingerprint: CanonicalFingerprintV1,
        closure_document_count: usize,
        conformance: ConformanceV1,
        violations: Vec<NormalizedViolationV1>,
        already_normalized: bool,
    ) -> Result<Self, ValidatorRefusalV1> {
        let limits = ValidatorLimitsV1::v1();
        if closure_document_count == 0 || closure_document_count > limits.max_documents() {
            return Err(ValidatorRefusalV1::LimitExceeded);
        }
        if violations.len() > limits.max_violations() {
            return Err(ValidatorRefusalV1::LimitExceeded);
        }
        if !already_normalized && violations.windows(2).any(|pair| pair[0] >= pair[1]) {
            return Err(ValidatorRefusalV1::InputMalformed);
        }
        let shape_is_valid = match conformance {
            ConformanceV1::Conformant => violations.is_empty(),
            ConformanceV1::Nonconformant => !violations.is_empty(),
        };
        if !shape_is_valid || root_schema.fingerprint() != &root_document_fingerprint {
            return Err(ValidatorRefusalV1::InputMalformed);
        }
        Ok(Self {
            root_schema,
            root_document,
            root_document_fingerprint,
            closure_fingerprint,
            instance_fingerprint,
            closure_document_count: closure_document_count as u16,
            conformance,
            violations,
        })
    }

    pub fn root_schema(&self) -> &ExactSchemaBindingV1 {
        &self.root_schema
    }
    pub fn root_document(&self) -> &LogicalSchemaPathV1 {
        &self.root_document
    }
    pub fn root_document_fingerprint(&self) -> &CanonicalFingerprintV1 {
        &self.root_document_fingerprint
    }
    pub fn closure_fingerprint(&self) -> &CanonicalFingerprintV1 {
        &self.closure_fingerprint
    }
    pub fn instance_fingerprint(&self) -> &CanonicalFingerprintV1 {
        &self.instance_fingerprint
    }
    pub fn closure_document_count(&self) -> usize {
        self.closure_document_count as usize
    }
    pub fn conformance(&self) -> ConformanceV1 {
        self.conformance
    }
    pub fn violations(&self) -> &[NormalizedViolationV1] {
        &self.violations
    }

    pub fn canonical_fingerprint(&self) -> Result<CanonicalFingerprintV1, ValidatorRefusalV1> {
        canonical_fingerprint(self)
    }

    pub fn verify_canonical_fingerprint(
        &self,
        supplied: &CanonicalFingerprintV1,
    ) -> Result<(), ValidatorRefusalV1> {
        supplied.verify_value(self)
    }

    pub fn verify_against_input(
        &self,
        input: &JsonSchemaValidationInputV1,
    ) -> Result<(), ValidatorRefusalV1> {
        input.verify_fingerprints()?;
        if self.root_schema != *input.root_schema()
            || self.root_document != *input.root_document()
            || self.root_document_fingerprint != *input.root_document_fingerprint()
            || self.closure_fingerprint != *input.closure_fingerprint()
            || self.instance_fingerprint != *input.instance().instance_fingerprint()
            || self.closure_document_count() != input.documents().len()
            || self
                .violations
                .iter()
                .any(|violation| !input.contains_document(&violation.schema_document))
        {
            return Err(ValidatorRefusalV1::InputSubstituted);
        }
        Ok(())
    }
}

impl<'de> Deserialize<'de> for SchemaConformanceFactV1 {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        #[serde(deny_unknown_fields)]
        struct Raw {
            root_schema: ExactSchemaBindingV1,
            root_document: LogicalSchemaPathV1,
            root_document_fingerprint: CanonicalFingerprintV1,
            closure_fingerprint: CanonicalFingerprintV1,
            instance_fingerprint: CanonicalFingerprintV1,
            closure_document_count: u16,
            conformance: ConformanceV1,
            violations: Vec<NormalizedViolationV1>,
        }
        let raw = Raw::deserialize(deserializer)?;
        Self::new(
            raw.root_schema,
            raw.root_document,
            raw.root_document_fingerprint,
            raw.closure_fingerprint,
            raw.instance_fingerprint,
            raw.closure_document_count as usize,
            raw.conformance,
            raw.violations,
            false,
        )
        .map_err(D::Error::custom)
    }
}

fn validate_pointer(pointer: &str) -> Result<(), ValidatorRefusalV1> {
    if pointer.len() > ValidatorLimitsV1::v1().max_pointer_bytes() {
        return Err(ValidatorRefusalV1::LimitExceeded);
    }
    if !pointer.is_empty() && !pointer.starts_with('/') {
        return Err(ValidatorRefusalV1::InputMalformed);
    }
    let mut bytes = pointer.bytes();
    while let Some(byte) = bytes.next() {
        if byte == b'~' && !matches!(bytes.next(), Some(b'0' | b'1')) {
            return Err(ValidatorRefusalV1::InputMalformed);
        }
    }
    Ok(())
}
