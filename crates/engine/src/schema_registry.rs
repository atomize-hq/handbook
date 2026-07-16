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
const MAX_STRUCTURAL_LOCATION_BYTES: usize = 512;

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
                let instance_location = error.instance_path().to_string();
                let schema_location = error
                    .absolute_keyword_location()
                    .map(|uri| internal_uri_to_repo_location(uri.as_str()))
                    .unwrap_or_else(|| {
                        format!("{}#{}", self.entry.document_ref, error.schema_path())
                    });
                StructuralValidationError {
                    instance_location: if instance_location.len() <= 512 {
                        instance_location
                    } else {
                        String::new()
                    },
                    schema_location: bounded_structural_location(schema_location),
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
                    "schema_entry_source",
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
                    "schema_registry_entries",
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
                "document_ref",
                "schema document_ref must already be normalized",
            ));
        }

        let mut loader = ClosureLoader::new(repo_root, allowed_roots, budget);
        loader.load_document(&self.document_ref)?;
        loader.validate_reference_graph()?;
        let root = loader.documents.get(&self.document_ref).ok_or_else(|| {
            RegistryLoadError::at(
                RegistryLoadErrorKind::ValidatorTargetMismatch,
                "document_ref",
                "prewalk did not retain the selected root document",
            )
        })?;
        if root.fingerprint != supplied_document_fingerprint {
            return Err(RegistryLoadError::at(
                RegistryLoadErrorKind::FingerprintMismatch,
                "document_fingerprint",
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
                "closure_fingerprint",
                "schema closure fingerprint does not match the prewalked closure",
            ));
        }

        let computed_entry_fingerprint = fingerprint_serializable(&self)?;
        if computed_entry_fingerprint != supplied_entry_fingerprint {
            return Err(RegistryLoadError::at(
                RegistryLoadErrorKind::FingerprintMismatch,
                "entry_fingerprint",
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
    schema_children: BTreeMap<String, Vec<String>>,
    references: Vec<LocalSchemaReference>,
}

struct ClosureLoader<'a> {
    repo_root: &'a Path,
    allowed_roots: &'a [String],
    budget: &'a mut SourceByteBudget,
    documents: BTreeMap<String, LoadedSchemaDocument>,
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
        }
    }

    fn load_document(&mut self, requested_path: &str) -> Result<(), RegistryLoadError> {
        let normalized = normalize_schema_path(self.repo_root, requested_path)?;
        self.require_allowed_root(&normalized)?;
        if self.documents.contains_key(&normalized) {
            return Ok(());
        }
        if self.documents.len() >= MAX_CLOSURE_DOCUMENTS {
            return Err(RegistryLoadError::new(
                RegistryLoadErrorKind::DocumentLimitExceeded,
                "schema closure exceeds 128 documents",
            ));
        }

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
        validate_schema_meta(&normalized, &value)?;

        for reference in &profile.references {
            if reference.target_path.is_none() {
                validate_pointer_target(
                    &normalized,
                    &value,
                    &profile.schema_pointers,
                    &reference.fragment,
                )?;
            }
        }

        let target_paths = profile
            .references
            .iter()
            .filter_map(|reference| reference.target_path.clone())
            .collect::<BTreeSet<_>>();
        self.documents.insert(
            normalized.clone(),
            LoadedSchemaDocument {
                path: normalized.clone(),
                value,
                fingerprint: DefinitionFingerprint::from_bytes(&bytes),
                schema_pointers: profile.schema_pointers,
                schema_children: profile.schema_children,
                references: profile.references,
            },
        );

        for target_path in target_paths {
            self.load_document(&target_path)?;
        }
        let references = self
            .documents
            .get(&normalized)
            .map(|document| document.references.clone())
            .unwrap_or_default();
        for reference in references {
            if let Some(target_path) = reference.target_path {
                let target = self.documents.get(&target_path).ok_or_else(|| {
                    RegistryLoadError::at(
                        RegistryLoadErrorKind::ValidatorTargetMismatch,
                        &target_path,
                        "prewalk target was not retained in the schema closure",
                    )
                })?;
                validate_pointer_target(
                    &target_path,
                    &target.value,
                    &target.schema_pointers,
                    &reference.fragment,
                )?;
            }
        }
        Ok(())
    }

    fn validate_reference_graph(&self) -> Result<(), RegistryLoadError> {
        let mut visiting = BTreeSet::new();
        let mut longest_paths = BTreeMap::new();
        for document in self.documents.values() {
            for pointer in &document.schema_pointers {
                self.longest_reference_path(
                    SchemaLocation {
                        document_path: document.path.clone(),
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
    ) -> Result<usize, RegistryLoadError> {
        if let Some(longest) = longest_paths.get(&location) {
            return Ok(*longest);
        }
        if !visiting.insert(location.clone()) {
            return Err(RegistryLoadError::at(
                RegistryLoadErrorKind::LocalReferenceCycle,
                &location.document_path,
                "local schema reference cycle is not admitted in HCM-1.1",
            ));
        }

        let document = self.documents.get(&location.document_path).ok_or_else(|| {
            RegistryLoadError::at(
                RegistryLoadErrorKind::ValidatorTargetMismatch,
                &location.document_path,
                "reference graph document is absent from the admitted closure",
            )
        })?;
        let children = document
            .schema_children
            .get(&location.pointer)
            .cloned()
            .unwrap_or_default();
        let references = document
            .references
            .iter()
            .filter(|reference| reference.source_pointer == location.pointer)
            .cloned()
            .collect::<Vec<_>>();

        let mut longest = 0;
        for child in children {
            longest = longest.max(self.longest_reference_path(
                SchemaLocation {
                    document_path: location.document_path.clone(),
                    pointer: child,
                },
                visiting,
                longest_paths,
            )?);
        }
        for reference in references {
            let target = SchemaLocation {
                document_path: reference
                    .target_path
                    .unwrap_or_else(|| location.document_path.clone()),
                pointer: reference.fragment,
            };
            let target_longest = self.longest_reference_path(target, visiting, longest_paths)?;
            longest = longest.max(target_longest.saturating_add(1));
            if longest > MAX_REFERENCE_DEPTH {
                return Err(RegistryLoadError::at(
                    RegistryLoadErrorKind::ReferenceDepthExceeded,
                    &location.document_path,
                    "local schema reference depth exceeds 32 edges",
                ));
            }
        }

        visiting.remove(&location);
        longest_paths.insert(location, longest);
        Ok(longest)
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

#[derive(Clone, Debug)]
struct LocalSchemaReference {
    source_pointer: String,
    target_path: Option<String>,
    fragment: String,
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
struct SchemaLocation {
    document_path: String,
    pointer: String,
}

struct SchemaProfile {
    references: Vec<LocalSchemaReference>,
    schema_pointers: BTreeSet<String>,
    schema_children: BTreeMap<String, Vec<String>>,
}

fn collect_schema_profile(
    document_path: &str,
    value: &Value,
) -> Result<SchemaProfile, RegistryLoadError> {
    let mut references = Vec::new();
    let mut schema_pointers = BTreeSet::new();
    let mut schema_children = BTreeMap::new();
    validate_schema_node(
        document_path,
        value,
        "",
        true,
        &mut references,
        &mut schema_pointers,
        &mut schema_children,
    )?;
    Ok(SchemaProfile {
        references,
        schema_pointers,
        schema_children,
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
    jsonschema::draft202012::meta::validate(value).map_err(|_| {
        RegistryLoadError::at(
            RegistryLoadErrorKind::UnsupportedDialect,
            path,
            "schema document fails Draft 2020-12 meta-validation",
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
    schema_children: &mut BTreeMap<String, Vec<String>>,
) -> Result<(), RegistryLoadError> {
    schema_pointers.insert(pointer.to_owned());
    schema_children.entry(pointer.to_owned()).or_default();
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
            bounded_schema_location(document_path, pointer),
            "schema position must contain an object or boolean",
        ));
    };

    for (keyword, keyword_value) in object {
        let keyword_pointer = format!("{pointer}/{}", escape_json_pointer_token(keyword));
        if is_unsupported_identifier_keyword(keyword) {
            return Err(RegistryLoadError::at(
                RegistryLoadErrorKind::UnsupportedSchemaIdentifier,
                bounded_schema_location(document_path, &keyword_pointer),
                "authored identifiers, anchors, and dynamic or recursive references are unsupported",
            ));
        }
        if keyword == "$schema" && !document_root {
            return Err(RegistryLoadError::at(
                RegistryLoadErrorKind::UnsupportedDialect,
                bounded_schema_location(document_path, &keyword_pointer),
                "nested $schema declarations are unsupported",
            ));
        }
        if !is_allowed_schema_keyword(keyword) {
            return Err(RegistryLoadError::at(
                RegistryLoadErrorKind::UnsupportedSchemaKeyword,
                bounded_schema_location(document_path, &keyword_pointer),
                "schema-position keyword is outside the frozen HCM-1.1 allowlist",
            ));
        }
        if keyword == "$ref" {
            let reference = keyword_value.as_str().ok_or_else(|| {
                RegistryLoadError::at(
                    RegistryLoadErrorKind::RemoteReferenceRefused,
                    bounded_schema_location(document_path, &keyword_pointer),
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
            let child_pointer = format!("{pointer}/{}", escape_json_pointer_token(keyword));
            schema_children
                .entry(pointer.to_owned())
                .or_default()
                .push(child_pointer.clone());
            validate_schema_node(
                document_path,
                child,
                &child_pointer,
                false,
                references,
                schema_pointers,
                schema_children,
            )?;
        }
    }
    for keyword in ["allOf", "anyOf", "oneOf", "prefixItems"] {
        if let Some(children) = object.get(keyword).and_then(Value::as_array) {
            for (index, child) in children.iter().enumerate() {
                let child_pointer = format!("{pointer}/{keyword}/{index}");
                schema_children
                    .entry(pointer.to_owned())
                    .or_default()
                    .push(child_pointer.clone());
                validate_schema_node(
                    document_path,
                    child,
                    &child_pointer,
                    false,
                    references,
                    schema_pointers,
                    schema_children,
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
                let child_pointer =
                    format!("{pointer}/{keyword}/{}", escape_json_pointer_token(name));
                schema_children
                    .entry(pointer.to_owned())
                    .or_default()
                    .push(child_pointer.clone());
                validate_schema_node(
                    document_path,
                    child,
                    &child_pointer,
                    false,
                    references,
                    schema_pointers,
                    schema_children,
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
            document_path,
            "local JSON Pointer target does not exist",
        ));
    }
    if !schema_pointers.contains(fragment) {
        return Err(RegistryLoadError::at(
            RegistryLoadErrorKind::ValidatorTargetMismatch,
            document_path,
            "local JSON Pointer target is not an admitted schema position",
        ));
    }
    Ok(())
}

fn build_validator(
    root_document_ref: &str,
    documents: &BTreeMap<String, LoadedSchemaDocument>,
) -> Result<Validator, RegistryLoadError> {
    let mut registry_builder = jsonschema::Registry::new().draft(Draft::Draft202012);
    for document in documents.values() {
        let validator_value = validator_document_value(document)?;
        registry_builder = registry_builder
            .add(internal_resource_uri(&document.path), validator_value)
            .map_err(|_| {
                RegistryLoadError::at(
                    RegistryLoadErrorKind::DuplicateResourceIdentity,
                    &document.path,
                    "internal schema resource registration failed",
                )
            })?;
    }
    let registry = registry_builder.prepare().map_err(|_| {
        RegistryLoadError::new(
            RegistryLoadErrorKind::DuplicateResourceIdentity,
            "in-memory schema resource registry failed",
        )
    })?;
    let root = documents.get(root_document_ref).ok_or_else(|| {
        RegistryLoadError::at(
            RegistryLoadErrorKind::ValidatorTargetMismatch,
            root_document_ref,
            "validator root is missing from the prevalidated closure",
        )
    })?;
    let root_validator_value = validator_document_value(root)?;
    jsonschema::draft202012::options()
        .with_registry(&registry)
        .with_base_uri(internal_resource_uri(root_document_ref))
        .with_pattern_options(PatternOptions::regex())
        .build(&root_validator_value)
        .map_err(|_| {
            RegistryLoadError::at(
                RegistryLoadErrorKind::StructuralValidationSetup,
                root_document_ref,
                "prevalidated in-memory schema failed validator construction",
            )
        })
}

fn validator_document_value(document: &LoadedSchemaDocument) -> Result<Value, RegistryLoadError> {
    let mut value = document.value.clone();
    for reference in &document.references {
        let mut target = match &reference.target_path {
            Some(target_path) => internal_resource_uri(target_path),
            None => internal_resource_uri(&document.path),
        };
        if !reference.fragment.is_empty() {
            target.push('#');
            target.push_str(&percent_encode_uri_path_like(&reference.fragment));
        }
        let pointer = if reference.source_pointer.is_empty() {
            "/$ref".to_string()
        } else {
            format!("{}/$ref", reference.source_pointer)
        };
        let Some(slot) = value.pointer_mut(&pointer) else {
            return Err(RegistryLoadError::at(
                RegistryLoadErrorKind::ValidatorTargetMismatch,
                "schema_reference",
                "prewalk reference source is absent from the validator document",
            ));
        };
        *slot = Value::String(target);
    }
    Ok(value)
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
            RegistryLoadError::at(
                RegistryLoadErrorKind::InvalidSourcePath,
                "allowed_schema_root",
                detail,
            )
        })?;
        if path.as_str() != root {
            return Err(RegistryLoadError::at(
                RegistryLoadErrorKind::InvalidSourcePath,
                "allowed_schema_root",
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
            RegistryLoadError::at(
                RegistryLoadErrorKind::InvalidSourcePath,
                "source_path",
                detail,
            )
        })
}

fn classify_entry_decode_error(error: serde_json::Error) -> RegistryLoadError {
    let rendered = error.to_string();
    let (kind, detail) = if rendered.contains("unknown field") {
        (
            RegistryLoadErrorKind::UnknownField,
            "schema-registry entry contains an unknown field",
        )
    } else {
        (
            RegistryLoadErrorKind::SyntaxError,
            "schema-registry entry does not match its closed typed record",
        )
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

fn remote_reference_error(document_path: &str, _reference: &str) -> RegistryLoadError {
    RegistryLoadError::at(
        RegistryLoadErrorKind::RemoteReferenceRefused,
        document_path,
        "remote, ambient, encoded, query, backslash, or traversal ref refused",
    )
}

fn bounded_schema_location(document_path: &str, pointer: &str) -> String {
    if pointer.len() <= 256 {
        format!("{document_path}#{pointer}")
    } else {
        document_path.to_owned()
    }
}

fn internal_resource_uri(path: &str) -> String {
    let mut uri = String::with_capacity("handbook+repo:///".len() + path.len());
    uri.push_str("handbook+repo:///");
    uri.push_str(&percent_encode_uri_path_like(path));
    uri
}

fn percent_encode_uri_path_like(value: &str) -> String {
    const HEX: &[u8; 16] = b"0123456789ABCDEF";

    let mut encoded = String::with_capacity(value.len());
    for byte in value.bytes() {
        if byte.is_ascii_alphanumeric() || matches!(byte, b'-' | b'.' | b'_' | b'~' | b'/') {
            encoded.push(char::from(byte));
        } else {
            encoded.push('%');
            encoded.push(char::from(HEX[usize::from(byte >> 4)]));
            encoded.push(char::from(HEX[usize::from(byte & 0x0f)]));
        }
    }
    encoded
}

fn internal_uri_to_repo_location(uri: &str) -> String {
    let Some(encoded) = uri.strip_prefix("handbook+repo:///") else {
        return "schema_root".to_string();
    };
    let (encoded_path, fragment) = encoded.split_once('#').unwrap_or((encoded, ""));
    let Some(path) = percent_decode_utf8(encoded_path) else {
        return "schema_root".to_string();
    };
    if fragment.is_empty() {
        return path;
    }
    let Some(fragment) = percent_decode_utf8(fragment) else {
        return "schema_root".to_string();
    };
    format!("{path}#{fragment}")
}

fn percent_decode_utf8(encoded: &str) -> Option<String> {
    let bytes = encoded.as_bytes();
    let mut decoded = Vec::with_capacity(bytes.len());
    let mut index = 0;
    while index < bytes.len() {
        if bytes[index] == b'%' {
            let high = *bytes.get(index + 1)?;
            let low = *bytes.get(index + 2)?;
            decoded.push((hex_value(high)? << 4) | hex_value(low)?);
            index += 3;
        } else {
            decoded.push(bytes[index]);
            index += 1;
        }
    }
    String::from_utf8(decoded).ok()
}

fn hex_value(byte: u8) -> Option<u8> {
    match byte {
        b'0'..=b'9' => Some(byte - b'0'),
        b'a'..=b'f' => Some(byte - b'a' + 10),
        b'A'..=b'F' => Some(byte - b'A' + 10),
        _ => None,
    }
}

fn bounded_structural_location(location: String) -> String {
    if location.len() <= MAX_STRUCTURAL_LOCATION_BYTES && !location.chars().any(char::is_control) {
        location
    } else {
        "schema_root".to_string()
    }
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

#[cfg(test)]
mod internal_resource_uri_tests {
    use super::{internal_resource_uri, internal_uri_to_repo_location};

    #[test]
    fn internal_resource_uris_are_injective_and_round_trip_utf8_paths() {
        for (path, expected) in [
            ("schemas/a b.json", "handbook+repo:///schemas/a%20b.json"),
            (
                "schemas/a%20b.json",
                "handbook+repo:///schemas/a%2520b.json",
            ),
            (
                "schemas/café.json",
                "handbook+repo:///schemas/caf%C3%A9.json",
            ),
        ] {
            let uri = internal_resource_uri(path);
            assert_eq!(uri, expected);
            assert_eq!(internal_uri_to_repo_location(&uri), path);
        }
        assert_ne!(
            internal_resource_uri("schemas/a b.json"),
            internal_resource_uri("schemas/a%20b.json")
        );
    }

    #[test]
    fn encoded_private_fragments_decode_to_exact_json_pointers() {
        assert_eq!(
            internal_uri_to_repo_location(
                "handbook+repo:///schemas/child.json#/$defs/caf%C3%A9%20field/type"
            ),
            "schemas/child.json#/$defs/café field/type"
        );
    }
}
