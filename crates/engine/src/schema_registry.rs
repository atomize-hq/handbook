use crate::canonical_repo_support::CanonicalWorkspace;
use crate::definition_identity::{
    fingerprint_serializable, parse_definition_yaml, parse_schema_json, DefinitionFingerprint,
    ExactDefinitionRef, RegistryLoadError, RegistryLoadErrorKind, SourceByteBudget,
};
use crate::stable_role_registry::read_trusted_repo_source;
use jsonschema::{Draft, PatternOptions, Validator};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::{BTreeMap, BTreeSet};
use std::path::{Component, Path};

const DRAFT_2020_12: &str = "https://json-schema.org/draft/2020-12/schema";
const SCHEMA_MEDIA_TYPE: &str = "application/schema+json";
const MAX_CLOSURE_DOCUMENTS: usize = 128;
const MAX_REFERENCE_DEPTH: usize = 32;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct StructuralValidationError {
    instance_location: String,
    schema_location: String,
}

impl StructuralValidationError {
    pub fn instance_location(&self) -> &str {
        &self.instance_location
    }

    pub fn schema_location(&self) -> &str {
        &self.schema_location
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SchemaRegistryEntry {
    exact_ref: ExactDefinitionRef,
    document_ref: String,
    document_fingerprint: DefinitionFingerprint,
    closure_fingerprint: DefinitionFingerprint,
    entry_fingerprint: DefinitionFingerprint,
}

impl SchemaRegistryEntry {
    pub fn exact_ref(&self) -> &ExactDefinitionRef {
        &self.exact_ref
    }

    pub fn document_ref(&self) -> &str {
        &self.document_ref
    }

    pub fn document_fingerprint(&self) -> &DefinitionFingerprint {
        &self.document_fingerprint
    }

    pub fn closure_fingerprint(&self) -> &DefinitionFingerprint {
        &self.closure_fingerprint
    }

    pub fn entry_fingerprint(&self) -> &DefinitionFingerprint {
        &self.entry_fingerprint
    }
}

#[derive(Clone, Debug)]
pub struct ResolvedSchema {
    entry: SchemaRegistryEntry,
    closure_document_refs: Vec<String>,
    validator: Validator,
}

impl ResolvedSchema {
    pub fn entry(&self) -> &SchemaRegistryEntry {
        &self.entry
    }

    pub fn closure_document_refs(&self) -> &[String] {
        &self.closure_document_refs
    }

    pub fn validate_json(&self, instance: &Value) -> Result<(), Vec<StructuralValidationError>> {
        let mut errors = self
            .validator
            .iter_errors(instance)
            .map(|error| {
                let schema_location = error
                    .absolute_keyword_location()
                    .map(|uri| internal_uri_to_repo_location(uri.as_str()))
                    .unwrap_or_else(|| {
                        format!("{}#{}", self.entry.document_ref, error.schema_path())
                    });
                StructuralValidationError {
                    instance_location: error.instance_path().to_string(),
                    schema_location,
                }
            })
            .collect::<Vec<_>>();
        errors.sort_by(|left, right| {
            (&left.instance_location, &left.schema_location)
                .cmp(&(&right.instance_location, &right.schema_location))
        });
        errors.dedup();
        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }
}

#[derive(Clone, Debug)]
pub struct SchemaRegistry {
    entries: BTreeMap<ExactDefinitionRef, ResolvedSchema>,
    fingerprint: DefinitionFingerprint,
}

impl SchemaRegistry {
    pub fn load(
        repo_root: impl AsRef<Path>,
        entry_source_paths: &[String],
        allowed_schema_roots: &[String],
    ) -> Result<Self, RegistryLoadError> {
        let mut budget = SourceByteBudget::default();
        Self::load_with_budget(
            repo_root.as_ref(),
            entry_source_paths,
            allowed_schema_roots,
            &mut budget,
        )
    }

    pub(crate) fn load_with_budget(
        repo_root: &Path,
        entry_source_paths: &[String],
        allowed_schema_roots: &[String],
        budget: &mut SourceByteBudget,
    ) -> Result<Self, RegistryLoadError> {
        if entry_source_paths.is_empty() {
            return Err(RegistryLoadError::new(
                RegistryLoadErrorKind::MissingSchema,
                "at least one schema-registry entry source is required",
            ));
        }
        let allowed_roots = normalize_allowed_roots(repo_root, allowed_schema_roots)?;
        let mut entries: BTreeMap<ExactDefinitionRef, ResolvedSchema> = BTreeMap::new();

        for source_path in entry_source_paths {
            let (normalized_source, bytes) =
                read_trusted_repo_source(repo_root, source_path, budget)?;
            if normalized_source != *source_path {
                return Err(RegistryLoadError::at(
                    RegistryLoadErrorKind::InvalidSourcePath,
                    source_path,
                    "schema-entry source path must already be normalized",
                ));
            }
            let authored = AuthoredSchemaRegistryEntry::parse(&bytes)?;
            let exact_ref = authored.exact_ref()?;
            let resolved = authored.resolve(repo_root, &allowed_roots, budget)?;
            if let Some(existing) = entries.get(&exact_ref) {
                let kind = if existing.entry.entry_fingerprint == resolved.entry.entry_fingerprint
                    && existing.entry.closure_fingerprint == resolved.entry.closure_fingerprint
                {
                    RegistryLoadErrorKind::DuplicateIdentity
                } else {
                    RegistryLoadErrorKind::ConflictingIdentity
                };
                return Err(RegistryLoadError::at(
                    kind,
                    exact_ref.as_str(),
                    "schema-registry exact identity appears more than once",
                ));
            }
            entries.insert(exact_ref, resolved);
        }

        let fingerprint_members = entries
            .values()
            .map(|resolved| SchemaRegistryFingerprintMember {
                entry_ref: resolved.entry.exact_ref.as_str(),
                entry_fingerprint: resolved.entry.entry_fingerprint.as_str(),
                closure_fingerprint: resolved.entry.closure_fingerprint.as_str(),
            })
            .collect::<Vec<_>>();
        let fingerprint = fingerprint_serializable(&fingerprint_members)?;
        Ok(Self {
            entries,
            fingerprint,
        })
    }

    pub fn fingerprint(&self) -> &DefinitionFingerprint {
        &self.fingerprint
    }

    pub fn entry_refs(&self) -> Vec<ExactDefinitionRef> {
        self.entries.keys().cloned().collect()
    }

    pub fn entry(&self, exact_ref: &ExactDefinitionRef) -> Option<&SchemaRegistryEntry> {
        self.entries.get(exact_ref).map(|resolved| &resolved.entry)
    }

    pub fn resolved(&self, exact_ref: &ExactDefinitionRef) -> Option<&ResolvedSchema> {
        self.entries.get(exact_ref)
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
struct AuthoredSchemaRegistryEntry {
    schema_id: String,
    schema_version: String,
    content_schema_id: String,
    content_schema_version: String,
    document_ref: String,
    document_fingerprint: String,
    closure_fingerprint: String,
    meta_schema_ref: String,
    media_type: String,
    compatibility: String,
    extensions: BTreeMap<String, Value>,
    #[serde(skip_serializing)]
    entry_fingerprint: String,
}

impl AuthoredSchemaRegistryEntry {
    fn parse(bytes: &[u8]) -> Result<Self, RegistryLoadError> {
        let value = parse_definition_yaml(bytes)?;
        serde_json::from_value(value).map_err(classify_entry_decode_error)
    }

    fn exact_ref(&self) -> Result<ExactDefinitionRef, RegistryLoadError> {
        ExactDefinitionRef::new(&self.content_schema_id, &self.content_schema_version)
    }

    fn resolve(
        self,
        repo_root: &Path,
        allowed_roots: &[String],
        budget: &mut SourceByteBudget,
    ) -> Result<ResolvedSchema, RegistryLoadError> {
        self.validate_static_contract()?;
        let exact_ref = self.exact_ref()?;
        let supplied_document_fingerprint =
            DefinitionFingerprint::parse(&self.document_fingerprint)?;
        let supplied_closure_fingerprint = DefinitionFingerprint::parse(&self.closure_fingerprint)?;
        let supplied_entry_fingerprint = DefinitionFingerprint::parse(&self.entry_fingerprint)?;

        let normalized_document_ref = normalize_schema_path(repo_root, &self.document_ref)?;
        if normalized_document_ref != self.document_ref {
            return Err(RegistryLoadError::at(
                RegistryLoadErrorKind::InvalidSourcePath,
                &self.document_ref,
                "schema document_ref must already be normalized",
            ));
        }

        let mut loader = ClosureLoader::new(repo_root, allowed_roots, budget);
        loader.load_document(&self.document_ref, 0)?;
        let root = loader.documents.get(&self.document_ref).ok_or_else(|| {
            RegistryLoadError::at(
                RegistryLoadErrorKind::ValidatorTargetMismatch,
                &self.document_ref,
                "prewalk did not retain the selected root document",
            )
        })?;
        if root.fingerprint != supplied_document_fingerprint {
            return Err(RegistryLoadError::at(
                RegistryLoadErrorKind::FingerprintMismatch,
                &self.document_ref,
                "schema document fingerprint does not match exact source bytes",
            ));
        }

        let closure_members = loader
            .documents
            .values()
            .map(|document| ClosureFingerprintMember {
                document_ref: document.path.as_str(),
                document_fingerprint: document.fingerprint.as_str(),
            })
            .collect::<Vec<_>>();
        let computed_closure_fingerprint = fingerprint_serializable(&closure_members)?;
        if computed_closure_fingerprint != supplied_closure_fingerprint {
            return Err(RegistryLoadError::at(
                RegistryLoadErrorKind::FingerprintMismatch,
                &self.document_ref,
                "schema closure fingerprint does not match the prewalked closure",
            ));
        }

        let computed_entry_fingerprint = fingerprint_serializable(&self)?;
        if computed_entry_fingerprint != supplied_entry_fingerprint {
            return Err(RegistryLoadError::at(
                RegistryLoadErrorKind::FingerprintMismatch,
                exact_ref.as_str(),
                "schema-registry entry fingerprint does not match normalized source",
            ));
        }

        let validator = build_validator(&self.document_ref, &loader.documents)?;
        let closure_document_refs = loader.documents.keys().cloned().collect();
        Ok(ResolvedSchema {
            entry: SchemaRegistryEntry {
                exact_ref,
                document_ref: self.document_ref,
                document_fingerprint: supplied_document_fingerprint,
                closure_fingerprint: computed_closure_fingerprint,
                entry_fingerprint: computed_entry_fingerprint,
            },
            closure_document_refs,
            validator,
        })
    }

    fn validate_static_contract(&self) -> Result<(), RegistryLoadError> {
        if self.schema_id != "handbook.schema-registry-entry" || self.schema_version != "1.0" {
            return Err(RegistryLoadError::new(
                RegistryLoadErrorKind::UnsupportedRecord,
                "schema-registry entry must use handbook.schema-registry-entry / 1.0",
            ));
        }
        if self.meta_schema_ref != DRAFT_2020_12 {
            return Err(RegistryLoadError::new(
                RegistryLoadErrorKind::UnsupportedDialect,
                "schema-registry entry must select exact Draft 2020-12",
            ));
        }
        if self.media_type != SCHEMA_MEDIA_TYPE {
            return Err(RegistryLoadError::new(
                RegistryLoadErrorKind::UnsupportedMediaType,
                "schema-registry entry must use application/schema+json",
            ));
        }
        if self.compatibility != "exact" {
            return Err(RegistryLoadError::new(
                RegistryLoadErrorKind::UnsupportedCompatibility,
                "schema-registry entry compatibility must be exact",
            ));
        }
        if !self.extensions.is_empty() {
            return Err(RegistryLoadError::new(
                RegistryLoadErrorKind::UnsupportedDependency,
                "schema-registry extensions must be empty in HCM-1.1",
            ));
        }
        Ok(())
    }
}

#[derive(Debug)]
struct LoadedSchemaDocument {
    path: String,
    value: Value,
    fingerprint: DefinitionFingerprint,
    schema_pointers: BTreeSet<String>,
}

struct ClosureLoader<'a> {
    repo_root: &'a Path,
    allowed_roots: &'a [String],
    budget: &'a mut SourceByteBudget,
    documents: BTreeMap<String, LoadedSchemaDocument>,
    visiting: BTreeSet<String>,
}

impl<'a> ClosureLoader<'a> {
    fn new(
        repo_root: &'a Path,
        allowed_roots: &'a [String],
        budget: &'a mut SourceByteBudget,
    ) -> Self {
        Self {
            repo_root,
            allowed_roots,
            budget,
            documents: BTreeMap::new(),
            visiting: BTreeSet::new(),
        }
    }

    fn load_document(
        &mut self,
        requested_path: &str,
        depth: usize,
    ) -> Result<(), RegistryLoadError> {
        if depth > MAX_REFERENCE_DEPTH {
            return Err(RegistryLoadError::at(
                RegistryLoadErrorKind::ReferenceDepthExceeded,
                requested_path,
                "local schema reference depth exceeds 32 edges",
            ));
        }
        let normalized = normalize_schema_path(self.repo_root, requested_path)?;
        self.require_allowed_root(&normalized)?;
        if self.visiting.contains(&normalized) {
            return Err(RegistryLoadError::at(
                RegistryLoadErrorKind::LocalReferenceCycle,
                &normalized,
                "local schema reference cycle is not admitted in HCM-1.1",
            ));
        }
        if self.documents.contains_key(&normalized) {
            return Ok(());
        }
        if self.documents.len() + self.visiting.len() >= MAX_CLOSURE_DOCUMENTS {
            return Err(RegistryLoadError::new(
                RegistryLoadErrorKind::DocumentLimitExceeded,
                "schema closure exceeds 128 documents",
            ));
        }

        self.visiting.insert(normalized.clone());
        let (_, bytes) = read_trusted_repo_source(self.repo_root, &normalized, self.budget)
            .map_err(|error| {
                if error.kind() == RegistryLoadErrorKind::MissingSource {
                    RegistryLoadError::at(
                        RegistryLoadErrorKind::LocalReferenceMissing,
                        &normalized,
                        "referenced local schema document does not exist",
                    )
                } else {
                    error
                }
            })?;
        let value = parse_schema_json(&bytes)?;
        validate_schema_document(&normalized, &value)?;
        let profile = collect_schema_profile(&normalized, &value)?;
        validate_same_document_reference_cycles(&normalized, &profile.references)?;
        validate_schema_meta(&normalized, &value)?;

        for reference in &profile.references {
            if let Some(target_path) = &reference.target_path {
                self.load_document(target_path, depth + 1)?;
                let target = self.documents.get(target_path).ok_or_else(|| {
                    RegistryLoadError::at(
                        RegistryLoadErrorKind::ValidatorTargetMismatch,
                        target_path,
                        "prewalk target was not retained in the schema closure",
                    )
                })?;
                validate_pointer_target(
                    target_path,
                    &target.value,
                    &target.schema_pointers,
                    &reference.fragment,
                )?;
            } else {
                validate_pointer_target(
                    &normalized,
                    &value,
                    &profile.schema_pointers,
                    &reference.fragment,
                )?;
            }
        }

        self.visiting.remove(&normalized);
        self.documents.insert(
            normalized.clone(),
            LoadedSchemaDocument {
                path: normalized,
                value,
                fingerprint: DefinitionFingerprint::from_bytes(&bytes),
                schema_pointers: profile.schema_pointers,
            },
        );
        Ok(())
    }

    fn require_allowed_root(&self, normalized_path: &str) -> Result<(), RegistryLoadError> {
        if self
            .allowed_roots
            .iter()
            .any(|root| normalized_path == root || normalized_path.starts_with(&format!("{root}/")))
        {
            return Ok(());
        }
        Err(RegistryLoadError::at(
            RegistryLoadErrorKind::LocalReferenceOutsideRoot,
            normalized_path,
            "schema document is outside every explicitly allowed schema root",
        ))
    }
}

#[derive(Debug)]
struct LocalSchemaReference {
    source_pointer: String,
    target_path: Option<String>,
    fragment: String,
}

struct SchemaProfile {
    references: Vec<LocalSchemaReference>,
    schema_pointers: BTreeSet<String>,
}

fn collect_schema_profile(
    document_path: &str,
    value: &Value,
) -> Result<SchemaProfile, RegistryLoadError> {
    let mut references = Vec::new();
    let mut schema_pointers = BTreeSet::new();
    validate_schema_node(
        document_path,
        value,
        "",
        true,
        &mut references,
        &mut schema_pointers,
    )?;
    Ok(SchemaProfile {
        references,
        schema_pointers,
    })
}

fn validate_schema_document(path: &str, value: &Value) -> Result<(), RegistryLoadError> {
    let Some(object) = value.as_object() else {
        return Err(RegistryLoadError::at(
            RegistryLoadErrorKind::UnsupportedDialect,
            path,
            "schema document root must be an object with an exact Draft 2020-12 declaration",
        ));
    };
    match object.get("$schema") {
        Some(Value::String(dialect)) if dialect == DRAFT_2020_12 => {}
        _ => {
            return Err(RegistryLoadError::at(
                RegistryLoadErrorKind::UnsupportedDialect,
                path,
                "schema document root must declare exact Draft 2020-12",
            ))
        }
    }
    Ok(())
}

fn validate_schema_meta(path: &str, value: &Value) -> Result<(), RegistryLoadError> {
    jsonschema::draft202012::meta::validate(value).map_err(|error| {
        RegistryLoadError::at(
            RegistryLoadErrorKind::UnsupportedDialect,
            path,
            format!("schema document fails Draft 2020-12 meta-validation: {error}"),
        )
    })
}

fn validate_schema_node(
    document_path: &str,
    node: &Value,
    pointer: &str,
    document_root: bool,
    references: &mut Vec<LocalSchemaReference>,
    schema_pointers: &mut BTreeSet<String>,
) -> Result<(), RegistryLoadError> {
    schema_pointers.insert(pointer.to_owned());
    if node.is_boolean() {
        if document_root {
            return Err(RegistryLoadError::at(
                RegistryLoadErrorKind::UnsupportedDialect,
                document_path,
                "schema document root cannot be boolean",
            ));
        }
        return Ok(());
    }
    let Some(object) = node.as_object() else {
        return Err(RegistryLoadError::at(
            RegistryLoadErrorKind::UnsupportedDialect,
            format!("{document_path}#{pointer}"),
            "schema position must contain an object or boolean",
        ));
    };

    for (keyword, keyword_value) in object {
        let keyword_pointer = format!("{pointer}/{}", escape_json_pointer_token(keyword));
        if is_unsupported_identifier_keyword(keyword) {
            return Err(RegistryLoadError::at(
                RegistryLoadErrorKind::UnsupportedSchemaIdentifier,
                format!("{document_path}#{keyword_pointer}"),
                "authored identifiers, anchors, and dynamic or recursive references are unsupported",
            ));
        }
        if keyword == "$schema" && !document_root {
            return Err(RegistryLoadError::at(
                RegistryLoadErrorKind::UnsupportedDialect,
                format!("{document_path}#{keyword_pointer}"),
                "nested $schema declarations are unsupported",
            ));
        }
        if !is_allowed_schema_keyword(keyword) {
            return Err(RegistryLoadError::at(
                RegistryLoadErrorKind::UnsupportedSchemaKeyword,
                format!("{document_path}#{keyword_pointer}"),
                "schema-position keyword is outside the frozen HCM-1.1 allowlist",
            ));
        }
        if keyword == "$ref" {
            let reference = keyword_value.as_str().ok_or_else(|| {
                RegistryLoadError::at(
                    RegistryLoadErrorKind::RemoteReferenceRefused,
                    format!("{document_path}#{keyword_pointer}"),
                    "$ref must be a local string reference",
                )
            })?;
            references.push(parse_local_reference(document_path, pointer, reference)?);
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
            validate_schema_node(
                document_path,
                child,
                &format!("{pointer}/{}", escape_json_pointer_token(keyword)),
                false,
                references,
                schema_pointers,
            )?;
        }
    }
    for keyword in ["allOf", "anyOf", "oneOf", "prefixItems"] {
        if let Some(children) = object.get(keyword).and_then(Value::as_array) {
            for (index, child) in children.iter().enumerate() {
                validate_schema_node(
                    document_path,
                    child,
                    &format!("{pointer}/{keyword}/{index}"),
                    false,
                    references,
                    schema_pointers,
                )?;
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
                validate_schema_node(
                    document_path,
                    child,
                    &format!("{pointer}/{keyword}/{}", escape_json_pointer_token(name)),
                    false,
                    references,
                    schema_pointers,
                )?;
            }
        }
    }
    Ok(())
}

fn parse_local_reference(
    current_document: &str,
    source_pointer: &str,
    reference: &str,
) -> Result<LocalSchemaReference, RegistryLoadError> {
    if reference.contains('?')
        || reference.contains('\\')
        || reference.contains('%')
        || reference.starts_with('/')
        || reference.starts_with("//")
    {
        return Err(remote_reference_error(current_document, reference));
    }
    let (document_part, fragment) = reference.split_once('#').unwrap_or((reference, ""));
    if document_part.contains(':')
        || (!document_part.is_empty()
            && document_part
                .split('/')
                .any(|segment| segment.is_empty() || segment == "."))
        || Path::new(document_part).components().any(|component| {
            matches!(
                component,
                Component::ParentDir | Component::RootDir | Component::Prefix(_)
            )
        })
    {
        return Err(remote_reference_error(current_document, reference));
    }
    validate_json_pointer_fragment(current_document, fragment)?;
    if document_part.is_empty() {
        return Ok(LocalSchemaReference {
            source_pointer: source_pointer.to_owned(),
            target_path: None,
            fragment: fragment.to_owned(),
        });
    }

    let parent = Path::new(current_document)
        .parent()
        .unwrap_or_else(|| Path::new(""));
    let joined = parent.join(document_part);
    let joined = joined.to_str().ok_or_else(|| {
        RegistryLoadError::at(
            RegistryLoadErrorKind::InvalidSourcePath,
            current_document,
            "local schema reference path is not UTF-8",
        )
    })?;
    let target_path = normalize_schema_path(Path::new("."), joined)?;
    Ok(LocalSchemaReference {
        source_pointer: source_pointer.to_owned(),
        target_path: Some(target_path),
        fragment: fragment.to_owned(),
    })
}

fn validate_json_pointer_fragment(
    document_path: &str,
    fragment: &str,
) -> Result<(), RegistryLoadError> {
    if !fragment.is_empty() && !fragment.starts_with('/') {
        return Err(RegistryLoadError::at(
            RegistryLoadErrorKind::InvalidJsonPointer,
            document_path,
            "plain-name fragments are unsupported; local fragments must be RFC 6901 JSON Pointers",
        ));
    }
    let bytes = fragment.as_bytes();
    let mut index = 0;
    while index < bytes.len() {
        if bytes[index] == b'~' {
            if index + 1 >= bytes.len() || !matches!(bytes[index + 1], b'0' | b'1') {
                return Err(RegistryLoadError::at(
                    RegistryLoadErrorKind::InvalidJsonPointer,
                    document_path,
                    "JSON Pointer contains an invalid '~' escape",
                ));
            }
            index += 2;
        } else {
            index += 1;
        }
    }
    Ok(())
}

fn validate_pointer_target(
    document_path: &str,
    document: &Value,
    schema_pointers: &BTreeSet<String>,
    fragment: &str,
) -> Result<(), RegistryLoadError> {
    if document.pointer(fragment).is_none() {
        return Err(RegistryLoadError::at(
            RegistryLoadErrorKind::LocalReferenceMissing,
            format!("{document_path}#{fragment}"),
            "local JSON Pointer target does not exist",
        ));
    }
    if !schema_pointers.contains(fragment) {
        return Err(RegistryLoadError::at(
            RegistryLoadErrorKind::ValidatorTargetMismatch,
            format!("{document_path}#{fragment}"),
            "local JSON Pointer target is not an admitted schema position",
        ));
    }
    Ok(())
}

fn validate_same_document_reference_cycles(
    document_path: &str,
    references: &[LocalSchemaReference],
) -> Result<(), RegistryLoadError> {
    let mut edges: BTreeMap<&str, Vec<&str>> = BTreeMap::new();
    for reference in references
        .iter()
        .filter(|reference| reference.target_path.is_none())
    {
        edges
            .entry(reference.source_pointer.as_str())
            .or_default()
            .push(reference.fragment.as_str());
    }
    let mut visiting = BTreeSet::new();
    let mut visited = BTreeSet::new();
    for source in edges.keys().copied().collect::<Vec<_>>() {
        visit_reference_node(document_path, source, &edges, &mut visiting, &mut visited)?;
    }
    Ok(())
}

fn visit_reference_node<'a>(
    document_path: &str,
    node: &'a str,
    edges: &BTreeMap<&'a str, Vec<&'a str>>,
    visiting: &mut BTreeSet<&'a str>,
    visited: &mut BTreeSet<&'a str>,
) -> Result<(), RegistryLoadError> {
    if visited.contains(node) {
        return Ok(());
    }
    if !visiting.insert(node) {
        return Err(RegistryLoadError::at(
            RegistryLoadErrorKind::LocalReferenceCycle,
            format!("{document_path}#{node}"),
            "same-document JSON Pointer reference cycle is not admitted in HCM-1.1",
        ));
    }
    if let Some(targets) = edges.get(node) {
        for target in targets {
            visit_reference_node(document_path, target, edges, visiting, visited)?;
        }
    }
    visiting.remove(node);
    visited.insert(node);
    Ok(())
}

fn build_validator(
    root_document_ref: &str,
    documents: &BTreeMap<String, LoadedSchemaDocument>,
) -> Result<Validator, RegistryLoadError> {
    let mut registry_builder = jsonschema::Registry::new().draft(Draft::Draft202012);
    for document in documents.values() {
        registry_builder = registry_builder
            .add(
                internal_resource_uri(&document.path),
                document.value.clone(),
            )
            .map_err(|error| {
                RegistryLoadError::at(
                    RegistryLoadErrorKind::DuplicateResourceIdentity,
                    &document.path,
                    format!("internal schema resource registration failed: {error}"),
                )
            })?;
    }
    let registry = registry_builder.prepare().map_err(|error| {
        RegistryLoadError::new(
            RegistryLoadErrorKind::DuplicateResourceIdentity,
            format!("in-memory schema resource registry failed: {error}"),
        )
    })?;
    let root = documents.get(root_document_ref).ok_or_else(|| {
        RegistryLoadError::at(
            RegistryLoadErrorKind::ValidatorTargetMismatch,
            root_document_ref,
            "validator root is missing from the prevalidated closure",
        )
    })?;
    jsonschema::draft202012::options()
        .with_registry(&registry)
        .with_base_uri(internal_resource_uri(root_document_ref))
        .with_pattern_options(PatternOptions::regex())
        .build(&root.value)
        .map_err(|error| {
            RegistryLoadError::at(
                RegistryLoadErrorKind::StructuralValidationSetup,
                root_document_ref,
                format!("prevalidated in-memory schema failed validator construction: {error}"),
            )
        })
}

fn normalize_allowed_roots(
    repo_root: &Path,
    allowed_roots: &[String],
) -> Result<Vec<String>, RegistryLoadError> {
    if allowed_roots.is_empty() {
        return Err(RegistryLoadError::new(
            RegistryLoadErrorKind::LocalReferenceOutsideRoot,
            "at least one allowed schema root is required",
        ));
    }
    let workspace = CanonicalWorkspace::new(repo_root);
    let mut normalized = BTreeSet::new();
    for root in allowed_roots {
        let path = workspace.normalize_repo_relative(root).map_err(|detail| {
            RegistryLoadError::at(RegistryLoadErrorKind::InvalidSourcePath, root, detail)
        })?;
        if path.as_str() != root {
            return Err(RegistryLoadError::at(
                RegistryLoadErrorKind::InvalidSourcePath,
                root,
                "allowed schema root must already be normalized",
            ));
        }
        normalized.insert(path.as_str().to_owned());
    }
    Ok(normalized.into_iter().collect())
}

fn normalize_schema_path(repo_root: &Path, path: &str) -> Result<String, RegistryLoadError> {
    let workspace = CanonicalWorkspace::new(repo_root);
    workspace
        .normalize_repo_relative(path)
        .map(|normalized| normalized.as_str().to_owned())
        .map_err(|detail| {
            RegistryLoadError::at(RegistryLoadErrorKind::InvalidSourcePath, path, detail)
        })
}

fn classify_entry_decode_error(error: serde_json::Error) -> RegistryLoadError {
    let detail = error.to_string();
    let kind = if detail.contains("unknown field") {
        RegistryLoadErrorKind::UnknownField
    } else {
        RegistryLoadErrorKind::SyntaxError
    };
    RegistryLoadError::new(kind, detail)
}

fn is_unsupported_identifier_keyword(keyword: &str) -> bool {
    matches!(
        keyword,
        "$id" | "$anchor" | "$dynamicAnchor" | "$recursiveAnchor" | "$recursiveRef" | "$dynamicRef"
    )
}

fn is_allowed_schema_keyword(keyword: &str) -> bool {
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

fn remote_reference_error(document_path: &str, reference: &str) -> RegistryLoadError {
    RegistryLoadError::at(
        RegistryLoadErrorKind::RemoteReferenceRefused,
        document_path,
        format!(
            "remote, ambient, encoded, query, backslash, or traversal ref refused: {reference}"
        ),
    )
}

fn internal_resource_uri(path: &str) -> String {
    format!("handbook+repo:///{path}")
}

fn internal_uri_to_repo_location(uri: &str) -> String {
    uri.strip_prefix("handbook+repo:///")
        .unwrap_or(uri)
        .to_owned()
}

fn escape_json_pointer_token(token: &str) -> String {
    token.replace('~', "~0").replace('/', "~1")
}

#[derive(Serialize)]
struct ClosureFingerprintMember<'a> {
    document_ref: &'a str,
    document_fingerprint: &'a str,
}

#[derive(Serialize)]
struct SchemaRegistryFingerprintMember<'a> {
    entry_ref: &'a str,
    entry_fingerprint: &'a str,
    closure_fingerprint: &'a str,
}
