use crate::canonical::canonical_fingerprint;
use crate::{
    CanonicalFingerprintV1, ExactSchemaBindingV1, LogicalSchemaPathV1, ValidatorLimitsV1,
    ValidatorRefusalV1,
};
use serde::de::Error as _;
use serde::{Deserialize, Deserializer, Serialize};
use serde_json::Value;

/// One member of the complete logical schema closure.
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct SchemaClosureMemberV1 {
    path: LogicalSchemaPathV1,
    schema: Value,
    document_fingerprint: CanonicalFingerprintV1,
}

impl SchemaClosureMemberV1 {
    pub fn new(path: LogicalSchemaPathV1, schema: Value) -> Result<Self, ValidatorRefusalV1> {
        let document_fingerprint = canonical_fingerprint(&schema)?;
        Ok(Self {
            path,
            schema,
            document_fingerprint,
        })
    }

    pub fn path(&self) -> &LogicalSchemaPathV1 {
        &self.path
    }
    pub fn schema(&self) -> &Value {
        &self.schema
    }
    pub fn document_fingerprint(&self) -> &CanonicalFingerprintV1 {
        &self.document_fingerprint
    }

    pub fn verify_fingerprint(&self) -> Result<(), ValidatorRefusalV1> {
        self.document_fingerprint.verify_value(&self.schema)
    }
}

impl<'de> Deserialize<'de> for SchemaClosureMemberV1 {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        #[serde(deny_unknown_fields)]
        struct Raw {
            path: LogicalSchemaPathV1,
            schema: Value,
            document_fingerprint: CanonicalFingerprintV1,
        }
        let raw = Raw::deserialize(deserializer)?;
        let supplied = raw.document_fingerprint;
        let admitted = Self::new(raw.path, raw.schema).map_err(D::Error::custom)?;
        if admitted.document_fingerprint != supplied {
            return Err(D::Error::custom(ValidatorRefusalV1::InputSubstituted));
        }
        Ok(admitted)
    }
}

/// Exactly one JSON-compatible instance and its canonical value fingerprint.
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct JsonInstanceV1 {
    value: Value,
    instance_fingerprint: CanonicalFingerprintV1,
}

impl JsonInstanceV1 {
    pub fn new(value: Value) -> Result<Self, ValidatorRefusalV1> {
        let instance_fingerprint = canonical_fingerprint(&value)?;
        Ok(Self {
            value,
            instance_fingerprint,
        })
    }

    pub fn value(&self) -> &Value {
        &self.value
    }
    pub fn instance_fingerprint(&self) -> &CanonicalFingerprintV1 {
        &self.instance_fingerprint
    }
    pub fn verify_fingerprint(&self) -> Result<(), ValidatorRefusalV1> {
        self.instance_fingerprint.verify_value(&self.value)
    }
}

impl<'de> Deserialize<'de> for JsonInstanceV1 {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        #[serde(deny_unknown_fields)]
        struct Raw {
            value: Value,
            instance_fingerprint: CanonicalFingerprintV1,
        }
        let raw = Raw::deserialize(deserializer)?;
        let supplied = raw.instance_fingerprint;
        let admitted = Self::new(raw.value).map_err(D::Error::custom)?;
        if admitted.instance_fingerprint != supplied {
            return Err(D::Error::custom(ValidatorRefusalV1::InputSubstituted));
        }
        Ok(admitted)
    }
}

#[derive(Serialize)]
struct ClosureFingerprintMember<'a> {
    path: &'a LogicalSchemaPathV1,
    document_fingerprint: &'a CanonicalFingerprintV1,
}

/// The sole closed `handbook.input.json-schema-validation` payload.
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct JsonSchemaValidationInputV1 {
    root_schema: ExactSchemaBindingV1,
    root_document: LogicalSchemaPathV1,
    root_document_fingerprint: CanonicalFingerprintV1,
    closure_fingerprint: CanonicalFingerprintV1,
    documents: Vec<SchemaClosureMemberV1>,
    instance: JsonInstanceV1,
}

impl JsonSchemaValidationInputV1 {
    pub fn new(
        root_schema: ExactSchemaBindingV1,
        root_document: LogicalSchemaPathV1,
        documents: Vec<SchemaClosureMemberV1>,
        instance: JsonInstanceV1,
    ) -> Result<Self, ValidatorRefusalV1> {
        let limits = ValidatorLimitsV1::v1();
        if documents.is_empty() || documents.len() > limits.max_documents() {
            return Err(ValidatorRefusalV1::LimitExceeded);
        }
        if documents
            .windows(2)
            .any(|pair| pair[0].path >= pair[1].path)
        {
            return Err(ValidatorRefusalV1::InputMalformed);
        }
        for document in &documents {
            document.verify_fingerprint()?;
        }
        instance.verify_fingerprint()?;
        let root = documents
            .iter()
            .find(|document| document.path == root_document)
            .ok_or(ValidatorRefusalV1::InputMalformed)?;
        let root_document_fingerprint = root.document_fingerprint.clone();
        if root_schema.fingerprint() != &root_document_fingerprint {
            return Err(ValidatorRefusalV1::InputSubstituted);
        }
        let closure_fingerprint = closure_fingerprint(&documents)?;
        Ok(Self {
            root_schema,
            root_document,
            root_document_fingerprint,
            closure_fingerprint,
            documents,
            instance,
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
    pub fn documents(&self) -> &[SchemaClosureMemberV1] {
        &self.documents
    }
    pub fn instance(&self) -> &JsonInstanceV1 {
        &self.instance
    }

    pub fn verify_fingerprints(&self) -> Result<(), ValidatorRefusalV1> {
        for document in &self.documents {
            document.verify_fingerprint()?;
        }
        self.instance.verify_fingerprint()?;
        let root = self
            .documents
            .iter()
            .find(|document| document.path == self.root_document)
            .ok_or(ValidatorRefusalV1::InputMalformed)?;
        if root.document_fingerprint != self.root_document_fingerprint
            || self.root_schema.fingerprint() != &self.root_document_fingerprint
            || closure_fingerprint(&self.documents)? != self.closure_fingerprint
        {
            return Err(ValidatorRefusalV1::InputSubstituted);
        }
        Ok(())
    }

    pub(crate) fn contains_document(&self, path: &LogicalSchemaPathV1) -> bool {
        self.documents
            .binary_search_by(|document| document.path.cmp(path))
            .is_ok()
    }
}

impl<'de> Deserialize<'de> for JsonSchemaValidationInputV1 {
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
            documents: Vec<SchemaClosureMemberV1>,
            instance: JsonInstanceV1,
        }
        let raw = Raw::deserialize(deserializer)?;
        let supplied_root = raw.root_document_fingerprint;
        let supplied_closure = raw.closure_fingerprint;
        let admitted = Self::new(
            raw.root_schema,
            raw.root_document,
            raw.documents,
            raw.instance,
        )
        .map_err(D::Error::custom)?;
        if admitted.root_document_fingerprint != supplied_root
            || admitted.closure_fingerprint != supplied_closure
        {
            return Err(D::Error::custom(ValidatorRefusalV1::InputSubstituted));
        }
        Ok(admitted)
    }
}

fn closure_fingerprint(
    documents: &[SchemaClosureMemberV1],
) -> Result<CanonicalFingerprintV1, ValidatorRefusalV1> {
    let members = documents
        .iter()
        .map(|document| ClosureFingerprintMember {
            path: &document.path,
            document_fingerprint: &document.document_fingerprint,
        })
        .collect::<Vec<_>>();
    canonical_fingerprint(&members)
}
