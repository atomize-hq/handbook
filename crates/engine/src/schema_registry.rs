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
const MAX_SCHEMA_PATH_BYTES: usize = 1024;
const MAX_SCHEMA_PATH_COMPONENTS: usize = 64;
const MAX_BINDING_ARRAY_WITNESS_ITEMS: usize = 64;
const MAX_BINDING_ARRAY_WITNESS_STATES: usize = 4096;
const MAX_BINDING_ITEM_CANDIDATES: usize = 32;
const HCM_2_2_DATE_TIME_SCHEMA_DOCUMENTS: &[&str] = &[
    "definitions/schemas/handbook.schemas.lifecycle.trigger-evidence/1.0.0.schema.json",
    "definitions/schemas/handbook.schemas.security.approver-registry-transition/1.0.0.schema.json",
    "definitions/schemas/handbook.schemas.security.authenticator-registration/1.0.0.schema.json",
    "definitions/schemas/handbook.schemas.security.authenticator-assertion/1.0.0.schema.json",
];

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct StructuralValidationError {
    instance_location: String,
    schema_location: String,
}

#[doc(hidden)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ResolvedBindingJsonType {
    Object,
    Array,
    String,
}

#[allow(dead_code)] // Consumed by the Task 7 capability-binding increment.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum ResolvedBindingCardinality {
    Singular,
    Plural,
}

#[allow(dead_code)] // Consumed by the Task 7 capability-binding increment.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum ResolvedBindingEmptyPolicy {
    Forbidden,
    Allowed,
}

#[allow(dead_code)] // Consumed by the Task 7 capability-binding increment.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct ResolvedBindingShape {
    json_type: ResolvedBindingJsonType,
    cardinality: ResolvedBindingCardinality,
    empty_policy: ResolvedBindingEmptyPolicy,
}

#[allow(dead_code)] // Consumed by the Task 7 capability-binding increment.
impl ResolvedBindingShape {
    pub(crate) fn json_type(self) -> ResolvedBindingJsonType {
        self.json_type
    }

    pub(crate) fn cardinality(self) -> ResolvedBindingCardinality {
        self.cardinality
    }

    pub(crate) fn empty_policy(self) -> ResolvedBindingEmptyPolicy {
        self.empty_policy
    }
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
    #[allow(dead_code)] // Consumed by the Task 7 capability-binding increment.
    documents: BTreeMap<String, LoadedSchemaDocument>,
    validator: Validator,
    deferred_fingerprint_error: Option<RegistryLoadError>,
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

    pub(crate) fn coverage_leaf_shapes(
        &self,
    ) -> Result<BTreeMap<String, ResolvedBindingJsonType>, RegistryLoadError> {
        let mut leaves = BTreeMap::new();
        let mut active = BTreeSet::new();
        self.collect_coverage_leaf_shapes(
            &self.entry.document_ref,
            "",
            "",
            &mut active,
            &mut leaves,
        )?;
        if leaves.is_empty() {
            return Err(indeterminate_binding_shape(
                "candidate schema has no finitely enumerable coverage leaves",
            ));
        }
        Ok(leaves)
    }

    fn collect_coverage_leaf_shapes(
        &self,
        document_path: &str,
        schema_pointer: &str,
        instance_pointer: &str,
        active: &mut BTreeSet<SchemaLocation>,
        leaves: &mut BTreeMap<String, ResolvedBindingJsonType>,
    ) -> Result<(), RegistryLoadError> {
        let mut references = BindingReferenceTraversal::default();
        let (document_path, schema_pointer) =
            self.resolve_schema_reference(document_path, schema_pointer, &mut references)?;
        let location = SchemaLocation {
            document_path: document_path.clone(),
            pointer: schema_pointer.clone(),
        };
        if !active.insert(location.clone()) {
            return Err(indeterminate_binding_shape(
                "candidate schema coverage contains a reference cycle",
            ));
        }
        let result = (|| {
            let document = self.documents.get(&document_path).ok_or_else(|| {
                indeterminate_binding_shape("candidate schema document is absent")
            })?;
            let node = document
                .value
                .pointer(&schema_pointer)
                .ok_or_else(|| indeterminate_binding_shape("candidate schema pointer is absent"))?;
            require_unambiguous_schema_node(node)?;
            match node.get("type").and_then(Value::as_str) {
                Some("object") => {
                    if node.get("additionalProperties").and_then(Value::as_bool) != Some(false)
                        || node
                            .get("patternProperties")
                            .and_then(Value::as_object)
                            .is_some_and(|patterns| !patterns.is_empty())
                    {
                        return Err(indeterminate_binding_shape(
                            "candidate coverage requires closed object schemas",
                        ));
                    }
                    let properties = node
                        .get("properties")
                        .and_then(Value::as_object)
                        .ok_or_else(|| {
                            indeterminate_binding_shape(
                                "candidate coverage object must declare properties",
                            )
                        })?;
                    if properties.is_empty() {
                        if instance_pointer.is_empty() {
                            return Err(indeterminate_binding_shape(
                                "candidate root cannot be an empty object terminal",
                            ));
                        }
                        leaves.insert(instance_pointer.to_owned(), ResolvedBindingJsonType::Object);
                        return Ok(());
                    }
                    for property in properties.keys() {
                        let child_schema_pointer = format!(
                            "{schema_pointer}/properties/{}",
                            escape_json_pointer_token(property)
                        );
                        let child_instance_pointer =
                            format!("{instance_pointer}/{}", escape_json_pointer_token(property));
                        self.collect_coverage_leaf_shapes(
                            &document_path,
                            &child_schema_pointer,
                            &child_instance_pointer,
                            active,
                            leaves,
                        )?;
                    }
                    Ok(())
                }
                Some("array") => {
                    if instance_pointer.is_empty() {
                        return Err(indeterminate_binding_shape(
                            "candidate root must be an object",
                        ));
                    }
                    leaves.insert(instance_pointer.to_owned(), ResolvedBindingJsonType::Array);
                    Ok(())
                }
                Some("string") => {
                    if instance_pointer.is_empty() {
                        return Err(indeterminate_binding_shape(
                            "candidate root must be an object",
                        ));
                    }
                    leaves.insert(instance_pointer.to_owned(), ResolvedBindingJsonType::String);
                    Ok(())
                }
                _ => Err(indeterminate_binding_shape(
                    "candidate coverage leaf has an unsupported or indeterminate JSON type",
                )),
            }
        })();
        active.remove(&location);
        result
    }

    #[allow(dead_code)] // Consumed by the Task 7 capability-binding increment.
    pub(crate) fn binding_shape(
        &self,
        instance_pointer: &str,
    ) -> Result<ResolvedBindingShape, RegistryLoadError> {
        let is_frozen_charter_1_1 =
            self.entry.exact_ref.as_str() == "handbook.schemas.artifacts.project-authority@1.1.0";
        let tokens = parse_instance_pointer(instance_pointer)?;
        let mut document_path = self.entry.document_ref.clone();
        let mut schema_pointer = String::new();
        let mut traversal = BindingReferenceTraversal::default();

        for token in tokens {
            let (resolved_document, resolved_pointer) =
                self.resolve_schema_reference(&document_path, &schema_pointer, &mut traversal)?;
            let document = self
                .documents
                .get(&resolved_document)
                .ok_or_else(|| indeterminate_binding_shape("binding schema document is absent"))?;
            let node = document
                .value
                .pointer(&resolved_pointer)
                .ok_or_else(|| indeterminate_binding_shape("binding schema pointer is absent"))?;
            let is_frozen_charter_1_1_root = is_frozen_charter_1_1
                && resolved_document == self.entry.document_ref
                && resolved_pointer.is_empty();
            if !is_frozen_charter_1_1_root {
                require_unambiguous_schema_node(node)?;
            }
            if node.get("type").and_then(Value::as_str) != Some("object") {
                return Err(indeterminate_binding_shape(
                    "binding path parent must have exact object type",
                ));
            }
            if node.get("additionalProperties").and_then(Value::as_bool) != Some(false) {
                return Err(indeterminate_binding_shape(
                    "binding path parent must be a closed object",
                ));
            }
            if node
                .get("patternProperties")
                .and_then(Value::as_object)
                .is_some_and(|patterns| !patterns.is_empty())
            {
                return Err(indeterminate_binding_shape(
                    "binding path parent cannot admit patterned properties",
                ));
            }
            if !is_frozen_charter_1_1 {
                require_satisfiable_binding_parent(node)?;
                self.require_satisfiable_schema_location(&resolved_document, &resolved_pointer)?;
            }
            let properties = node
                .get("properties")
                .and_then(Value::as_object)
                .ok_or_else(|| {
                    indeterminate_binding_shape("binding path parent must declare properties")
                })?;
            if !properties.contains_key(&token) {
                return Err(indeterminate_binding_shape(
                    "binding path property is absent",
                ));
            }
            let required = node
                .get("required")
                .and_then(Value::as_array)
                .ok_or_else(|| {
                    indeterminate_binding_shape("binding path parent must declare required")
                })?;
            if !required.iter().any(|value| value.as_str() == Some(&token)) {
                return Err(indeterminate_binding_shape(
                    "binding path property must be required",
                ));
            }
            document_path = resolved_document;
            schema_pointer = format!(
                "{resolved_pointer}/properties/{}",
                escape_json_pointer_token(&token)
            );
        }

        let (document_path, schema_pointer) =
            self.resolve_schema_reference(&document_path, &schema_pointer, &mut traversal)?;
        let document = self
            .documents
            .get(&document_path)
            .ok_or_else(|| indeterminate_binding_shape("binding schema document is absent"))?;
        let node = document
            .value
            .pointer(&schema_pointer)
            .ok_or_else(|| indeterminate_binding_shape("binding schema pointer is absent"))?;
        require_unambiguous_schema_node(node)?;
        let json_type = match node.get("type").and_then(Value::as_str) {
            Some("object") => ResolvedBindingJsonType::Object,
            Some("array") => ResolvedBindingJsonType::Array,
            Some("string") => ResolvedBindingJsonType::String,
            _ => {
                return Err(indeterminate_binding_shape(
                    "binding terminal must have one explicit supported JSON type",
                ))
            }
        };
        if json_type == ResolvedBindingJsonType::Object
            && node.get("additionalProperties").and_then(Value::as_bool) != Some(false)
        {
            return Err(indeterminate_binding_shape(
                "binding object terminal must be closed",
            ));
        }
        if json_type == ResolvedBindingJsonType::Object
            && node
                .get("patternProperties")
                .and_then(Value::as_object)
                .is_some_and(|patterns| !patterns.is_empty())
        {
            return Err(indeterminate_binding_shape(
                "binding object terminal cannot admit patterned properties",
            ));
        }
        if !is_frozen_charter_1_1 {
            require_satisfiable_binding_terminal(node, json_type)?;
            self.require_satisfiable_schema_location(&document_path, &schema_pointer)?;
        }
        let cardinality = if json_type == ResolvedBindingJsonType::Array {
            ResolvedBindingCardinality::Plural
        } else {
            ResolvedBindingCardinality::Singular
        };
        let non_empty = match json_type {
            ResolvedBindingJsonType::Object => {
                node.get("required")
                    .and_then(Value::as_array)
                    .is_some_and(|required| !required.is_empty())
                    || node
                        .get("minProperties")
                        .and_then(Value::as_u64)
                        .is_some_and(|value| value >= 1)
            }
            ResolvedBindingJsonType::Array => node
                .get("minItems")
                .and_then(Value::as_u64)
                .is_some_and(|value| value >= 1),
            ResolvedBindingJsonType::String => node
                .get("minLength")
                .and_then(Value::as_u64)
                .is_some_and(|value| value >= 1),
        };
        Ok(ResolvedBindingShape {
            json_type,
            cardinality,
            empty_policy: if non_empty {
                ResolvedBindingEmptyPolicy::Forbidden
            } else {
                ResolvedBindingEmptyPolicy::Allowed
            },
        })
    }

    fn require_satisfiable_schema_location(
        &self,
        document_path: &str,
        schema_pointer: &str,
    ) -> Result<(), RegistryLoadError> {
        let mut traversal = BindingSatisfiabilityTraversal::default();
        if self.schema_location_is_conservatively_satisfiable(
            document_path,
            schema_pointer,
            0,
            &mut traversal,
        )? {
            Ok(())
        } else {
            Err(indeterminate_binding_shape(
                "binding schema location is not provably satisfiable",
            ))
        }
    }

    fn schema_location_is_conservatively_satisfiable(
        &self,
        document_path: &str,
        schema_pointer: &str,
        reference_depth: usize,
        traversal: &mut BindingSatisfiabilityTraversal,
    ) -> Result<bool, RegistryLoadError> {
        let mut references = BindingReferenceTraversal::default();
        let (document_path, schema_pointer) =
            self.resolve_schema_reference(document_path, schema_pointer, &mut references)?;
        let reference_depth = reference_depth
            .checked_add(references.edges)
            .filter(|depth| *depth <= MAX_REFERENCE_DEPTH)
            .ok_or_else(|| {
                indeterminate_binding_shape(
                    "binding satisfiability reference depth exceeds 32 edges",
                )
            })?;
        let location = SchemaLocation {
            document_path: document_path.clone(),
            pointer: schema_pointer.clone(),
        };
        if traversal.proven.contains(&location) {
            return Ok(true);
        }
        if !traversal.active.insert(location.clone()) {
            return Ok(false);
        }
        let document = self
            .documents
            .get(&document_path)
            .ok_or_else(|| indeterminate_binding_shape("binding schema document is absent"))?;
        let node = document
            .value
            .pointer(&schema_pointer)
            .ok_or_else(|| indeterminate_binding_shape("binding schema pointer is absent"))?;
        let mut satisfiable = schema_fragment_is_conservatively_satisfiable(node)
            || schema_fragment_is_conservatively_satisfiable_with_deferred_property_references(
                node,
            );
        if satisfiable {
            if let Some(object) = node.as_object() {
                let candidate_types = match object.get("type") {
                    Some(Value::String(json_type)) => vec![json_type.as_str()],
                    Some(Value::Array(types)) => {
                        types.iter().filter_map(Value::as_str).collect::<Vec<_>>()
                    }
                    _ => Vec::new(),
                };
                if !candidate_types.is_empty() {
                    satisfiable = false;
                    let mut branch_error = None;
                    for json_type in candidate_types {
                        let mut normalized = object.clone();
                        normalized.insert("type".into(), Value::String(json_type.to_owned()));
                        let normalized = Value::Object(normalized);
                        if !schema_fragment_is_conservatively_satisfiable(&normalized)
                            && !schema_fragment_is_conservatively_satisfiable_with_deferred_property_references(
                                &normalized,
                            )
                        {
                            continue;
                        }
                        let active_before = traversal.active.clone();
                        let proven_before = traversal.proven.clone();
                        match self.schema_type_branch_is_conservatively_satisfiable(
                            object,
                            json_type,
                            &document_path,
                            &schema_pointer,
                            reference_depth,
                            traversal,
                        ) {
                            Ok(true) => {
                                satisfiable = true;
                                break;
                            }
                            Ok(false) => {
                                traversal.active = active_before;
                                traversal.proven = proven_before;
                            }
                            Err(error) => {
                                traversal.active = active_before;
                                traversal.proven = proven_before;
                                branch_error.get_or_insert(error);
                            }
                        }
                    }
                    if !satisfiable {
                        if let Some(error) = branch_error {
                            traversal.active.remove(&location);
                            return Err(error);
                        }
                    }
                }
            }
        }
        traversal.active.remove(&location);
        if satisfiable {
            traversal.proven.insert(location);
        }
        Ok(satisfiable)
    }

    fn schema_type_branch_is_conservatively_satisfiable(
        &self,
        object: &serde_json::Map<String, Value>,
        json_type: &str,
        document_path: &str,
        schema_pointer: &str,
        reference_depth: usize,
        traversal: &mut BindingSatisfiabilityTraversal,
    ) -> Result<bool, RegistryLoadError> {
        match json_type {
            "object" => {
                let required_names = effective_required_names(object)?;
                let properties = object
                    .get("properties")
                    .and_then(Value::as_object)
                    .cloned()
                    .unwrap_or_default();
                let Some(witness_names) = dependency_safe_minimum_property_names(
                    object,
                    &required_names,
                    &properties,
                    true,
                ) else {
                    return Ok(false);
                };
                for name in witness_names {
                    if !properties.contains_key(&name)
                        || !self.schema_location_is_conservatively_satisfiable(
                            document_path,
                            &format!(
                                "{schema_pointer}/properties/{}",
                                escape_json_pointer_token(&name)
                            ),
                            reference_depth,
                            traversal,
                        )?
                    {
                        return Ok(false);
                    }
                }
                Ok(true)
            }
            "array" => {
                let minimum_items = object.get("minItems").and_then(Value::as_u64).unwrap_or(0);
                let prefix_items = object
                    .get("prefixItems")
                    .and_then(Value::as_array)
                    .map(Vec::as_slice)
                    .unwrap_or_default();
                for index in 0..prefix_items
                    .len()
                    .min(usize::try_from(minimum_items).unwrap_or(usize::MAX))
                {
                    if !self.schema_location_is_conservatively_satisfiable(
                        document_path,
                        &format!("{schema_pointer}/prefixItems/{index}"),
                        reference_depth,
                        traversal,
                    )? {
                        return Ok(false);
                    }
                }
                if array_requires_post_prefix_item(object, prefix_items)
                    && object.contains_key("items")
                    && !self.schema_location_is_conservatively_satisfiable(
                        document_path,
                        &format!("{schema_pointer}/items"),
                        reference_depth,
                        traversal,
                    )?
                {
                    return Ok(false);
                }
                if object.contains_key("contains")
                    && object
                        .get("minContains")
                        .and_then(Value::as_u64)
                        .unwrap_or(1)
                        > 0
                    && !self.schema_location_is_conservatively_satisfiable(
                        document_path,
                        &format!("{schema_pointer}/contains"),
                        reference_depth,
                        traversal,
                    )?
                {
                    return Ok(false);
                }
                Ok(true)
            }
            _ => Ok(true),
        }
    }

    #[allow(dead_code)] // Consumed by the Task 7 capability-binding increment.
    fn resolve_schema_reference(
        &self,
        document_path: &str,
        schema_pointer: &str,
        traversal: &mut BindingReferenceTraversal,
    ) -> Result<(String, String), RegistryLoadError> {
        let mut document_path = document_path.to_owned();
        let mut schema_pointer = schema_pointer.to_owned();
        loop {
            let location = SchemaLocation {
                document_path: document_path.clone(),
                pointer: schema_pointer.clone(),
            };
            let document = self
                .documents
                .get(&document_path)
                .ok_or_else(|| indeterminate_binding_shape("binding schema document is absent"))?;
            let Some(reference) = document
                .references
                .iter()
                .find(|reference| reference.source_pointer == schema_pointer)
            else {
                return Ok((document_path, schema_pointer));
            };
            let node = document.value.pointer(&schema_pointer).ok_or_else(|| {
                indeterminate_binding_shape("binding schema reference source is absent")
            })?;
            require_reference_only_node(node)?;
            if !traversal.visited.insert(location) {
                return Err(indeterminate_binding_shape(
                    "binding schema reference cycle is refused",
                ));
            }
            if traversal.edges == MAX_REFERENCE_DEPTH {
                return Err(indeterminate_binding_shape(
                    "binding schema reference depth exceeds 32 edges",
                ));
            }
            traversal.edges += 1;
            document_path = reference
                .target_path
                .clone()
                .unwrap_or_else(|| document_path.clone());
            schema_pointer = reference.fragment.clone();
        }
    }
}

fn require_satisfiable_binding_terminal(
    node: &Value,
    json_type: ResolvedBindingJsonType,
) -> Result<(), RegistryLoadError> {
    let object = node
        .as_object()
        .ok_or_else(|| indeterminate_binding_shape("binding terminal must be an object schema"))?;
    if ["not", "const", "enum"]
        .iter()
        .any(|key| object.contains_key(*key))
    {
        return Err(indeterminate_binding_shape(
            "binding terminal has a semantic constraint whose satisfiability is not proven",
        ));
    }
    match json_type {
        ResolvedBindingJsonType::String => {
            require_ordered_bounds(object, "minLength", "maxLength")?;
        }
        ResolvedBindingJsonType::Array => {
            require_ordered_bounds(object, "minItems", "maxItems")?;
            if object.contains_key("contains") {
                let minimum = object
                    .get("minContains")
                    .and_then(Value::as_u64)
                    .unwrap_or(1);
                if object
                    .get("maxContains")
                    .and_then(Value::as_u64)
                    .is_some_and(|maximum| minimum > maximum)
                {
                    return Err(indeterminate_binding_shape(
                        "binding array terminal minimum contains exceeds its maximum",
                    ));
                }
            }
            let prefix_items = object
                .get("prefixItems")
                .and_then(Value::as_array)
                .map(Vec::as_slice)
                .unwrap_or_default();
            if array_requires_post_prefix_item(object, prefix_items)
                && object
                    .get("items")
                    .is_some_and(|items| !schema_fragment_is_conservatively_satisfiable(items))
            {
                return Err(indeterminate_binding_shape(
                    "binding array terminal requires an impossible item",
                ));
            }
            if object
                .get("contains")
                .is_some_and(|contains| !schema_fragment_is_conservatively_satisfiable(contains))
                && object
                    .get("minContains")
                    .and_then(Value::as_u64)
                    .unwrap_or(1)
                    > 0
            {
                return Err(indeterminate_binding_shape(
                    "binding array terminal requires an impossible contained item",
                ));
            }
        }
        ResolvedBindingJsonType::Object => {
            require_ordered_bounds(object, "minProperties", "maxProperties")?;
            let properties = object
                .get("properties")
                .and_then(Value::as_object)
                .cloned()
                .unwrap_or_default();
            let required_names = effective_required_names(object)?;
            if required_names
                .iter()
                .any(|name| !properties.contains_key(name))
                || object
                    .get("maxProperties")
                    .and_then(Value::as_u64)
                    .is_some_and(|maximum| required_names.len() as u64 > maximum)
                || object
                    .get("minProperties")
                    .and_then(Value::as_u64)
                    .is_some_and(|minimum| minimum > properties.len() as u64)
                || required_names.iter().any(|name| {
                    properties.get(name).is_some_and(|schema| {
                        !schema_fragment_is_conservatively_satisfiable(schema)
                    })
                })
                || !required_names_satisfy_property_names(object, &required_names)
            {
                return Err(indeterminate_binding_shape(
                    "binding object terminal has an impossible closed required set",
                ));
            }
        }
    }
    Ok(())
}

fn require_satisfiable_binding_parent(node: &Value) -> Result<(), RegistryLoadError> {
    let object = node
        .as_object()
        .ok_or_else(|| indeterminate_binding_shape("binding parent must be an object schema"))?;
    require_ordered_bounds(object, "minProperties", "maxProperties")?;
    let properties = object
        .get("properties")
        .and_then(Value::as_object)
        .cloned()
        .unwrap_or_default();
    let required_names = effective_required_names(object)?;
    if required_names
        .iter()
        .any(|name| !properties.contains_key(name))
        || object
            .get("maxProperties")
            .and_then(Value::as_u64)
            .is_some_and(|maximum| required_names.len() as u64 > maximum)
        || object
            .get("minProperties")
            .and_then(Value::as_u64)
            .is_some_and(|minimum| minimum > properties.len() as u64)
        || !required_names_satisfy_property_names(object, &required_names)
    {
        return Err(indeterminate_binding_shape(
            "binding parent has an impossible closed required set",
        ));
    }
    Ok(())
}

fn effective_required_names(
    object: &serde_json::Map<String, Value>,
) -> Result<BTreeSet<String>, RegistryLoadError> {
    let required = object
        .get("required")
        .and_then(Value::as_array)
        .cloned()
        .unwrap_or_default();
    let names = required
        .iter()
        .map(|value| value.as_str().map(str::to_owned))
        .collect::<Option<BTreeSet<_>>>()
        .ok_or_else(|| indeterminate_binding_shape("binding object required set is invalid"))?;
    dependent_required_closure(object, names)
}

fn dependent_required_closure(
    object: &serde_json::Map<String, Value>,
    mut names: BTreeSet<String>,
) -> Result<BTreeSet<String>, RegistryLoadError> {
    let dependencies = object
        .get("dependentRequired")
        .and_then(Value::as_object)
        .cloned()
        .unwrap_or_default();
    loop {
        let before = names.len();
        for trigger in names.clone() {
            if let Some(dependent_names) = dependencies.get(&trigger).and_then(Value::as_array) {
                for dependent in dependent_names {
                    names.insert(
                        dependent
                            .as_str()
                            .ok_or_else(|| {
                                indeterminate_binding_shape(
                                    "binding object dependent required set is invalid",
                                )
                            })?
                            .to_owned(),
                    );
                }
            }
        }
        if names.len() == before {
            return Ok(names);
        }
    }
}

fn required_names_satisfy_property_names(
    object: &serde_json::Map<String, Value>,
    required_names: &BTreeSet<String>,
) -> bool {
    required_names
        .iter()
        .all(|name| property_name_satisfies(object, name))
}

fn property_name_satisfies(object: &serde_json::Map<String, Value>, name: &str) -> bool {
    let Some(schema) = object.get("propertyNames") else {
        return true;
    };
    if schema.as_bool() == Some(true) {
        return true;
    }
    if schema.as_bool() == Some(false) {
        return false;
    }
    if schema.get("$ref").is_some() {
        return false;
    }
    jsonschema::draft202012::options()
        .with_pattern_options(PatternOptions::regex())
        .build(schema)
        .is_ok_and(|validator| validator.is_valid(&Value::String(name.to_owned())))
}

fn require_ordered_bounds(
    object: &serde_json::Map<String, Value>,
    minimum_key: &str,
    maximum_key: &str,
) -> Result<(), RegistryLoadError> {
    if let (Some(minimum), Some(maximum)) = (
        object.get(minimum_key).and_then(Value::as_u64),
        object.get(maximum_key).and_then(Value::as_u64),
    ) {
        if minimum > maximum {
            return Err(indeterminate_binding_shape(
                "binding terminal minimum exceeds its maximum",
            ));
        }
    }
    Ok(())
}

fn array_requires_post_prefix_item(
    object: &serde_json::Map<String, Value>,
    prefix_items: &[Value],
) -> bool {
    let minimum_items = object.get("minItems").and_then(Value::as_u64).unwrap_or(0);
    let contains_requires_post_prefix = object.get("contains").is_some_and(|contains| {
        let minimum_contains = object
            .get("minContains")
            .and_then(Value::as_u64)
            .unwrap_or(1);
        let prefix_matches = prefix_items
            .iter()
            .filter(|prefix| schemas_share_witness(prefix, contains))
            .count() as u64;
        minimum_contains > prefix_matches
    });
    minimum_items > prefix_items.len() as u64 || contains_requires_post_prefix
}

fn schema_fragment_is_conservatively_satisfiable(node: &Value) -> bool {
    let Some(object) = node.as_object() else {
        return node.as_bool().unwrap_or(false);
    };
    if ["not", "allOf", "anyOf", "oneOf", "if", "then", "else"]
        .iter()
        .any(|key| object.contains_key(*key))
    {
        return false;
    }
    for (minimum, maximum) in [
        ("minLength", "maxLength"),
        ("minItems", "maxItems"),
        ("minProperties", "maxProperties"),
    ] {
        if matches!(
            (
                object.get(minimum).and_then(Value::as_u64),
                object.get(maximum).and_then(Value::as_u64),
            ),
            (Some(left), Some(right)) if left > right
        ) {
            return false;
        }
    }
    if let Some(constant) = object.get("const") {
        return standalone_schema_accepts(node, constant);
    }
    if let Some(values) = object.get("enum").and_then(Value::as_array) {
        return values
            .iter()
            .any(|candidate| standalone_schema_accepts(node, candidate));
    }
    if let Some(types) = object.get("type").and_then(Value::as_array) {
        let mut unique = BTreeSet::new();
        if types.is_empty()
            || types.iter().any(|value| {
                value.as_str().is_none_or(|value| {
                    !matches!(
                        value,
                        "array" | "object" | "string" | "integer" | "number" | "boolean" | "null"
                    ) || !unique.insert(value)
                })
            })
        {
            return false;
        }
        return unique.into_iter().any(|json_type| {
            let mut normalized = object.clone();
            normalized.insert("type".into(), Value::String(json_type.to_owned()));
            schema_fragment_is_conservatively_satisfiable(&Value::Object(normalized))
        });
    }
    match object.get("type").and_then(Value::as_str) {
        Some("array") => {
            let minimum_items = object.get("minItems").and_then(Value::as_u64).unwrap_or(0);
            let prefix_items = object
                .get("prefixItems")
                .and_then(Value::as_array)
                .map(Vec::as_slice)
                .unwrap_or_default();
            if prefix_items
                .iter()
                .take(usize::try_from(minimum_items).unwrap_or(usize::MAX))
                .any(|schema| !schema_fragment_is_conservatively_satisfiable(schema))
            {
                return false;
            }
            let unique_items = object.get("uniqueItems").and_then(Value::as_bool) == Some(true);
            let needs_joint_witness = (unique_items
                && (!prefix_items.is_empty() || object.contains_key("contains")))
                || (object.contains_key("contains")
                    && (!prefix_items.is_empty()
                        || object.contains_key("maxContains")
                        || object.contains_key("maxItems")))
                || (minimum_items > 0 && object.contains_key("unevaluatedItems"));
            if needs_joint_witness {
                if !array_schema_has_joint_candidate(node, object) {
                    return false;
                }
                return true;
            }
            if unique_items
                && minimum_items > 1
                && !shared_item_domain_proves_unique_minimum(object, prefix_items, minimum_items)
            {
                return false;
            }
            let minimum_contains = object.contains_key("contains").then(|| {
                object
                    .get("minContains")
                    .and_then(Value::as_u64)
                    .unwrap_or(1)
            });
            if minimum_items.max(minimum_contains.unwrap_or(0)) > 0
                && object
                    .get("items")
                    .is_some_and(|items| !schema_fragment_is_conservatively_satisfiable(items))
            {
                return false;
            }
            if object.contains_key("contains") {
                let minimum_contains = minimum_contains.expect("contains established the default");
                let contains = object
                    .get("contains")
                    .expect("contains established the schema");
                let guaranteed_matches = required_items_provably_matching_contains(
                    object,
                    prefix_items,
                    minimum_items,
                    contains,
                );
                if object
                    .get("maxContains")
                    .and_then(Value::as_u64)
                    .is_some_and(|maximum| minimum_contains > maximum)
                    || object
                        .get("maxContains")
                        .and_then(Value::as_u64)
                        .is_some_and(|maximum| guaranteed_matches > maximum)
                    || object
                        .get("maxItems")
                        .and_then(Value::as_u64)
                        .is_some_and(|maximum| minimum_contains > maximum)
                    || (minimum_contains > 0
                        && object.get("contains").is_some_and(|contains| {
                            !schema_fragment_is_conservatively_satisfiable(contains)
                        }))
                    || (minimum_contains > 0
                        && object.get("items").is_some_and(|items| {
                            object
                                .get("contains")
                                .is_some_and(|contains| !schemas_share_witness(items, contains))
                        }))
                {
                    return false;
                }
            }
        }
        Some("object") => {
            let Ok(required_names) = effective_required_names(object) else {
                return false;
            };
            let properties = object
                .get("properties")
                .and_then(Value::as_object)
                .cloned()
                .unwrap_or_default();
            if object
                .get("patternProperties")
                .and_then(Value::as_object)
                .is_some_and(|patterns| !patterns.is_empty())
                && (!required_names.is_empty()
                    || object
                        .get("minProperties")
                        .and_then(Value::as_u64)
                        .is_some_and(|minimum| minimum > 0))
            {
                return false;
            }
            if object.get("additionalProperties").and_then(Value::as_bool) == Some(false)
                && required_names
                    .iter()
                    .any(|name| !properties.contains_key(name))
            {
                return false;
            }
            if !closed_object_has_dependency_safe_minimum(object, &required_names, &properties)
                || object
                    .get("maxProperties")
                    .and_then(Value::as_u64)
                    .is_some_and(|maximum| required_names.len() as u64 > maximum)
                || required_names.iter().any(|name| {
                    properties.get(name).is_some_and(|schema| {
                        !schema_fragment_is_conservatively_satisfiable(schema)
                    })
                })
                || !required_dependent_schemas_are_trivially_satisfiable(object, &required_names)
                || !required_names_satisfy_property_names(object, &required_names)
            {
                return false;
            }
        }
        Some("string") => return string_schema_has_candidate(node, object),
        Some("integer") => return numeric_schema_has_candidate(node, object, true),
        Some("number") => return numeric_schema_has_candidate(node, object, false),
        Some("boolean") => {
            return standalone_schema_accepts(node, &Value::Bool(false))
                || standalone_schema_accepts(node, &Value::Bool(true));
        }
        Some("null") => return standalone_schema_accepts(node, &Value::Null),
        _ => {}
    }
    true
}

fn schema_fragment_is_conservatively_satisfiable_with_deferred_property_references(
    node: &Value,
) -> bool {
    let mut deferred_reference = false;
    let deferred = defer_property_references(node, &mut deferred_reference);
    deferred_reference && schema_fragment_is_conservatively_satisfiable(&deferred)
}

fn defer_property_references(node: &Value, deferred_reference: &mut bool) -> Value {
    let Some(object) = node.as_object() else {
        return node.clone();
    };
    let mut deferred = object.clone();
    if let Some(properties) = object.get("properties").and_then(Value::as_object) {
        deferred.insert(
            "properties".into(),
            Value::Object(
                properties
                    .iter()
                    .map(|(name, schema)| {
                        (
                            name.clone(),
                            if schema.get("$ref").is_some() {
                                *deferred_reference = true;
                                Value::Bool(true)
                            } else {
                                defer_property_references(schema, deferred_reference)
                            },
                        )
                    })
                    .collect(),
            ),
        );
    }
    Value::Object(deferred)
}

fn array_schema_has_joint_candidate(
    schema: &Value,
    object: &serde_json::Map<String, Value>,
) -> bool {
    let minimum_items = object.get("minItems").and_then(Value::as_u64).unwrap_or(0);
    let maximum_items = object
        .get("maxItems")
        .and_then(Value::as_u64)
        .unwrap_or(u64::MAX);
    let minimum_contains = if object.contains_key("contains") {
        object
            .get("minContains")
            .and_then(Value::as_u64)
            .unwrap_or(1)
    } else {
        0
    };
    let prefix_items = object
        .get("prefixItems")
        .and_then(Value::as_array)
        .map(Vec::as_slice)
        .unwrap_or_default();
    let search_maximum = maximum_items.min(
        minimum_items
            .saturating_add(minimum_contains)
            .max((prefix_items.len() as u64).saturating_add(minimum_contains)),
    );
    let Ok(minimum_items) = usize::try_from(minimum_items) else {
        return false;
    };
    let Ok(search_maximum) = usize::try_from(search_maximum) else {
        return false;
    };
    if minimum_items > search_maximum || search_maximum > MAX_BINDING_ARRAY_WITNESS_ITEMS {
        return false;
    }
    let unique = object.get("uniqueItems").and_then(Value::as_bool) == Some(true);
    let mut states = 0;
    for length in minimum_items..=search_maximum {
        let mut domains = Vec::with_capacity(length);
        for index in 0..length {
            let item_schema = prefix_items.get(index).or_else(|| object.get("items"));
            let domain = item_schema
                .map(bounded_schema_candidates)
                .unwrap_or_else(unconstrained_schema_candidates);
            if domain.is_empty() {
                domains.clear();
                break;
            }
            domains.push(domain);
        }
        if domains.len() == length
            && array_candidate_search(
                schema,
                &domains,
                unique,
                &mut Vec::with_capacity(length),
                &mut states,
            )
        {
            return true;
        }
        if states >= MAX_BINDING_ARRAY_WITNESS_STATES {
            return false;
        }
    }
    false
}

fn array_candidate_search(
    schema: &Value,
    domains: &[Vec<Value>],
    unique: bool,
    candidate: &mut Vec<Value>,
    states: &mut usize,
) -> bool {
    if *states >= MAX_BINDING_ARRAY_WITNESS_STATES {
        return false;
    }
    if candidate.len() == domains.len() {
        *states += 1;
        return standalone_schema_accepts(schema, &Value::Array(candidate.clone()));
    }
    for value in &domains[candidate.len()] {
        if unique && candidate.contains(value) {
            continue;
        }
        *states += 1;
        candidate.push(value.clone());
        if array_candidate_search(schema, domains, unique, candidate, states) {
            return true;
        }
        candidate.pop();
        if *states >= MAX_BINDING_ARRAY_WITNESS_STATES {
            return false;
        }
    }
    false
}

fn bounded_schema_candidates(schema: &Value) -> Vec<Value> {
    let mut candidates = schema_candidate_values(schema);
    candidates.extend(unconstrained_schema_candidates());
    if let Some(object) = schema.as_object() {
        for key in [
            "minimum",
            "maximum",
            "exclusiveMinimum",
            "exclusiveMaximum",
            "multipleOf",
        ] {
            if let Some(value) = object.get(key).and_then(Value::as_f64) {
                for candidate in [value - 1.0, value, value + 1.0] {
                    if let Some(number) = serde_json::Number::from_f64(candidate) {
                        candidates.push(Value::Number(number));
                    }
                }
            }
        }
        if let Some(minimum) = object.get("minLength").and_then(Value::as_u64) {
            if minimum <= 8_192 {
                candidates.push(Value::String("a".repeat(minimum as usize)));
            }
        }
    }
    let mut proven = Vec::new();
    for candidate in candidates {
        if proven.len() == MAX_BINDING_ITEM_CANDIDATES {
            break;
        }
        if standalone_schema_accepts(schema, &candidate) && !proven.contains(&candidate) {
            proven.push(candidate);
        }
    }
    proven
}

fn unconstrained_schema_candidates() -> Vec<Value> {
    vec![
        Value::Null,
        Value::Bool(false),
        Value::Bool(true),
        Value::from(-1),
        Value::from(0),
        Value::from(1),
        Value::String(String::new()),
        Value::String("a".into()),
        Value::String("b".into()),
        Value::String("c".into()),
        Value::String("a.a".into()),
        Value::Array(Vec::new()),
        Value::Array(vec![Value::Null]),
        Value::Object(serde_json::Map::new()),
    ]
}

fn closed_object_has_dependency_safe_minimum(
    object: &serde_json::Map<String, Value>,
    required_names: &BTreeSet<String>,
    properties: &serde_json::Map<String, Value>,
) -> bool {
    dependency_safe_minimum_property_names(object, required_names, properties, false).is_some()
}

fn dependency_safe_minimum_property_names(
    object: &serde_json::Map<String, Value>,
    required_names: &BTreeSet<String>,
    properties: &serde_json::Map<String, Value>,
    defer_local_references: bool,
) -> Option<BTreeSet<String>> {
    let minimum = object
        .get("minProperties")
        .and_then(Value::as_u64)
        .unwrap_or(0);
    if required_names.len() as u64 >= minimum {
        return Some(required_names.clone());
    }
    let maximum = object
        .get("maxProperties")
        .and_then(Value::as_u64)
        .unwrap_or(u64::MAX);
    let mut selected = required_names.clone();
    for name in properties.keys() {
        if selected.contains(name) {
            continue;
        }
        let mut candidate = selected.clone();
        candidate.insert(name.clone());
        let Ok(closure) = dependent_required_closure(object, candidate) else {
            continue;
        };
        if closure.len() as u64 > maximum
            || closure.iter().any(|name| {
                !property_name_satisfies(object, name)
                    || properties.get(name).is_none_or(|schema| {
                        if schema.get("$ref").is_some() {
                            !defer_local_references
                        } else {
                            !schema_fragment_is_conservatively_satisfiable(schema)
                        }
                    })
            })
            || !required_dependent_schemas_are_trivially_satisfiable(object, &closure)
        {
            continue;
        }
        selected = closure;
        if selected.len() as u64 >= minimum {
            return Some(selected);
        }
    }
    None
}

fn shared_item_domain_proves_unique_minimum(
    object: &serde_json::Map<String, Value>,
    prefix_items: &[Value],
    minimum_items: u64,
) -> bool {
    if !prefix_items.is_empty() {
        return false;
    }
    let Some(items) = object.get("items") else {
        return false;
    };
    finite_schema_domain(items)
        .is_some_and(|domain| u64::try_from(domain.len()).is_ok_and(|size| size >= minimum_items))
}

fn finite_schema_domain(schema: &Value) -> Option<Vec<Value>> {
    if schema.as_bool() == Some(false) {
        return Some(Vec::new());
    }
    if schema.as_bool() == Some(true) {
        return None;
    }
    let object = schema.as_object()?;
    let candidates = if let Some(constant) = object.get("const") {
        vec![constant.clone()]
    } else if let Some(values) = object.get("enum").and_then(Value::as_array) {
        values.clone()
    } else {
        match object.get("type").and_then(Value::as_str) {
            Some("boolean") => vec![Value::Bool(false), Value::Bool(true)],
            Some("null") => vec![Value::Null],
            _ => return None,
        }
    };
    let mut domain = Vec::new();
    for candidate in candidates {
        if standalone_schema_accepts(schema, &candidate) && !domain.contains(&candidate) {
            domain.push(candidate);
        }
    }
    Some(domain)
}

fn required_items_provably_matching_contains(
    object: &serde_json::Map<String, Value>,
    prefix_items: &[Value],
    minimum_items: u64,
    contains: &Value,
) -> u64 {
    if contains.as_bool() == Some(true) {
        return minimum_items;
    }
    let prefix_count = minimum_items.min(prefix_items.len() as u64);
    let guaranteed_prefix = prefix_items
        .iter()
        .take(usize::try_from(prefix_count).unwrap_or(usize::MAX))
        .filter(|schema| schema_is_provably_subset_of(schema, contains))
        .count() as u64;
    let remaining = minimum_items.saturating_sub(prefix_count);
    guaranteed_prefix
        + object
            .get("items")
            .filter(|items| schema_is_provably_subset_of(items, contains))
            .map(|_| remaining)
            .unwrap_or(0)
}

fn schema_is_provably_subset_of(schema: &Value, containing_schema: &Value) -> bool {
    if containing_schema.as_bool() == Some(true)
        || schema.as_bool() == Some(false)
        || schema == containing_schema
    {
        return true;
    }
    if let Some(constant) = schema.get("const") {
        return standalone_schema_accepts(containing_schema, constant);
    }
    schema
        .get("enum")
        .and_then(Value::as_array)
        .is_some_and(|values| {
            !values.is_empty()
                && values
                    .iter()
                    .all(|candidate| standalone_schema_accepts(containing_schema, candidate))
        })
}

fn required_dependent_schemas_are_trivially_satisfiable(
    object: &serde_json::Map<String, Value>,
    required_names: &BTreeSet<String>,
) -> bool {
    let Some(dependent_schemas) = object.get("dependentSchemas").and_then(Value::as_object) else {
        return true;
    };
    required_names.iter().all(|name| {
        dependent_schemas.get(name).is_none_or(|schema| {
            schema.as_bool() == Some(true)
                || schema.as_object().is_some_and(serde_json::Map::is_empty)
        })
    })
}

fn schemas_share_witness(left: &Value, right: &Value) -> bool {
    let mut candidates = schema_candidate_values(left);
    candidates.extend(schema_candidate_values(right));
    candidates.extend([
        Value::Null,
        Value::Bool(false),
        Value::Bool(true),
        Value::from(-1),
        Value::from(0),
        Value::from(1),
        Value::String(String::new()),
        Value::String("a".into()),
        Value::String("a.a".into()),
        Value::Array(Vec::new()),
        Value::Array(vec![Value::Null]),
        Value::Object(serde_json::Map::new()),
    ]);
    candidates.iter().any(|candidate| {
        standalone_schema_accepts(left, candidate) && standalone_schema_accepts(right, candidate)
    })
}

fn schema_candidate_values(schema: &Value) -> Vec<Value> {
    let mut candidates = Vec::new();
    if let Some(constant) = schema.get("const") {
        candidates.push(constant.clone());
    }
    if let Some(values) = schema.get("enum").and_then(Value::as_array) {
        candidates.extend(values.iter().cloned());
    }
    candidates
}

fn string_schema_has_candidate(schema: &Value, object: &serde_json::Map<String, Value>) -> bool {
    let mut candidates = ["", "a", "a.a", "0", "A", "a-b", "example"]
        .into_iter()
        .map(|value| Value::String(value.to_owned()))
        .collect::<Vec<_>>();
    let minimum = object.get("minLength").and_then(Value::as_u64).unwrap_or(0);
    if minimum <= 8_192 {
        candidates.push(Value::String("a".repeat(minimum as usize)));
        if minimum >= 3 {
            let mut value = "a.a".to_owned();
            value.extend(std::iter::repeat_n('a', minimum as usize - 3));
            candidates.push(Value::String(value));
        }
    }
    candidates
        .iter()
        .any(|candidate| standalone_schema_accepts(schema, candidate))
}

fn numeric_schema_has_candidate(
    schema: &Value,
    object: &serde_json::Map<String, Value>,
    integer: bool,
) -> bool {
    let mut candidates = vec![
        -1024.0, -100.0, -10.0, -2.0, -1.0, 0.0, 1.0, 2.0, 10.0, 100.0, 1024.0,
    ];
    for key in [
        "minimum",
        "maximum",
        "exclusiveMinimum",
        "exclusiveMaximum",
        "multipleOf",
    ] {
        if let Some(value) = object.get(key).and_then(Value::as_f64) {
            candidates.extend([value, value - 1.0, value + 1.0]);
        }
    }
    if let (Some(minimum), Some(multiple)) = (
        object
            .get("minimum")
            .or_else(|| object.get("exclusiveMinimum"))
            .and_then(Value::as_f64),
        object.get("multipleOf").and_then(Value::as_f64),
    ) {
        if multiple > 0.0 {
            let candidate = (minimum / multiple).ceil() * multiple;
            candidates.extend([candidate, candidate + multiple]);
        }
    }
    candidates.into_iter().any(|candidate| {
        candidate.is_finite()
            && (!integer || candidate.fract() == 0.0)
            && serde_json::Number::from_f64(candidate)
                .map(Value::Number)
                .is_some_and(|candidate| standalone_schema_accepts(schema, &candidate))
    })
}

fn standalone_schema_accepts(schema: &Value, candidate: &Value) -> bool {
    if schema.get("$ref").is_some() {
        return false;
    }
    jsonschema::draft202012::options()
        .with_pattern_options(PatternOptions::regex())
        .build(schema)
        .is_ok_and(|validator| validator.is_valid(candidate))
}

#[derive(Default)]
struct BindingReferenceTraversal {
    visited: BTreeSet<SchemaLocation>,
    edges: usize,
}

#[derive(Default)]
struct BindingSatisfiabilityTraversal {
    active: BTreeSet<SchemaLocation>,
    proven: BTreeSet<SchemaLocation>,
}

fn require_reference_only_node(node: &Value) -> Result<(), RegistryLoadError> {
    let object = node.as_object().ok_or_else(|| {
        indeterminate_binding_shape("binding schema reference node must be an object")
    })?;
    let allowed_annotation = |key: &str| {
        matches!(
            key,
            "$ref"
                | "$comment"
                | "title"
                | "description"
                | "default"
                | "examples"
                | "deprecated"
                | "readOnly"
                | "writeOnly"
        )
    };
    if object.keys().any(|key| !allowed_annotation(key)) {
        return Err(indeterminate_binding_shape(
            "binding schema reference node has a semantic sibling",
        ));
    }
    Ok(())
}

#[allow(dead_code)] // Consumed by the Task 7 capability-binding increment.
fn parse_instance_pointer(pointer: &str) -> Result<Vec<String>, RegistryLoadError> {
    if pointer.is_empty() {
        return Ok(Vec::new());
    }
    if !pointer.starts_with('/') {
        return Err(indeterminate_binding_shape(
            "binding pointer must be an RFC 6901 JSON Pointer",
        ));
    }
    pointer[1..]
        .split('/')
        .map(|token| {
            let mut decoded = String::with_capacity(token.len());
            let bytes = token.as_bytes();
            let mut index = 0;
            while index < bytes.len() {
                if bytes[index] == b'~' {
                    let Some(next) = bytes.get(index + 1) else {
                        return Err(indeterminate_binding_shape(
                            "binding pointer contains an invalid escape",
                        ));
                    };
                    match next {
                        b'0' => decoded.push('~'),
                        b'1' => decoded.push('/'),
                        _ => {
                            return Err(indeterminate_binding_shape(
                                "binding pointer contains an invalid escape",
                            ))
                        }
                    }
                    index += 2;
                } else {
                    let remainder = &token[index..];
                    let character = remainder.chars().next().expect("non-empty remainder");
                    decoded.push(character);
                    index += character.len_utf8();
                }
            }
            Ok(decoded)
        })
        .collect()
}

#[allow(dead_code)] // Consumed by the Task 7 capability-binding increment.
fn require_unambiguous_schema_node(node: &Value) -> Result<(), RegistryLoadError> {
    let Some(object) = node.as_object() else {
        return Err(indeterminate_binding_shape(
            "binding schema node must be a closed object schema",
        ));
    };
    if ["allOf", "anyOf", "oneOf", "if", "then", "else"]
        .iter()
        .any(|keyword| object.contains_key(*keyword))
        || object.get("type").is_some_and(Value::is_array)
    {
        return Err(indeterminate_binding_shape(
            "binding schema node has an indeterminate union or conditional shape",
        ));
    }
    Ok(())
}

#[allow(dead_code)] // Consumed by the Task 7 capability-binding increment.
fn indeterminate_binding_shape(detail: &'static str) -> RegistryLoadError {
    RegistryLoadError::at(
        RegistryLoadErrorKind::UnsupportedDependency,
        "binding_shape",
        detail,
    )
}

#[derive(Clone, Debug)]
pub struct SchemaRegistry {
    entries: BTreeMap<ExactDefinitionRef, ResolvedSchema>,
    fingerprint: DefinitionFingerprint,
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub(crate) enum SchemaSourceKind {
    BuiltIn,
    Repository,
}

pub(crate) struct AdmittedSchemaEntrySource<'a> {
    pub(crate) declared_ref: &'a ExactDefinitionRef,
    pub(crate) source_kind: SchemaSourceKind,
    pub(crate) bytes: &'a [u8],
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
        let mut request_schema_state = RequestSchemaState::default();
        Self::load_with_request_budget(
            repo_root,
            entry_source_paths,
            allowed_schema_roots,
            budget,
            &mut request_schema_state,
        )
    }

    fn load_with_request_budget(
        repo_root: &Path,
        entry_source_paths: &[String],
        allowed_schema_roots: &[String],
        budget: &mut SourceByteBudget,
        request_schema_state: &mut RequestSchemaState,
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
            let resolved = authored.resolve(
                repo_root,
                &allowed_roots,
                budget,
                request_schema_state,
                SchemaSourceKind::Repository,
            )?;
            insert_resolved_schema(&mut entries, exact_ref, resolved)?;
        }

        let registry = finish_schema_registry(entries)?;
        registry.validate_fingerprints()?;
        Ok(registry)
    }

    #[allow(dead_code)] // Eager internal path retained for direct registry callers and tests.
    pub(crate) fn load_admitted(
        repo_root: &Path,
        entry_sources: &[AdmittedSchemaEntrySource<'_>],
        allowed_schema_roots: &[String],
        budget: &mut SourceByteBudget,
    ) -> Result<Self, RegistryLoadError> {
        let registry = Self::load_admitted_deferred_fingerprints(
            repo_root,
            entry_sources,
            allowed_schema_roots,
            budget,
        )?;
        registry.validate_fingerprints()?;
        Ok(registry)
    }

    pub(crate) fn load_admitted_deferred_fingerprints(
        repo_root: &Path,
        entry_sources: &[AdmittedSchemaEntrySource<'_>],
        allowed_schema_roots: &[String],
        budget: &mut SourceByteBudget,
    ) -> Result<Self, RegistryLoadError> {
        if entry_sources.is_empty() {
            return Err(RegistryLoadError::new(
                RegistryLoadErrorKind::MissingSchema,
                "at least one schema-registry entry source is required",
            ));
        }
        let allowed_roots = normalize_allowed_roots(repo_root, allowed_schema_roots)?;
        let mut request_schema_state = RequestSchemaState::default();
        let mut entries = BTreeMap::new();
        for source in entry_sources {
            let authored = AuthoredSchemaRegistryEntry::parse(source.bytes)?;
            let exact_ref = authored.exact_ref()?;
            if &exact_ref != source.declared_ref {
                return Err(RegistryLoadError::at(
                    RegistryLoadErrorKind::ConflictingIdentity,
                    "schema_entry_source",
                    "schema-entry derived exact ref does not match its typed source binding",
                ));
            }
            let resolved = authored.resolve(
                repo_root,
                &allowed_roots,
                budget,
                &mut request_schema_state,
                source.source_kind,
            )?;
            insert_resolved_schema(&mut entries, exact_ref, resolved)?;
        }
        finish_schema_registry(entries)
    }

    pub(crate) fn validate_fingerprints(&self) -> Result<(), RegistryLoadError> {
        for resolved in self.entries.values() {
            if let Some(error) = &resolved.deferred_fingerprint_error {
                return Err(error.clone());
            }
        }
        Ok(())
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

    pub(crate) fn select_entries(
        &self,
        exact_refs: &[ExactDefinitionRef],
    ) -> Result<Self, RegistryLoadError> {
        let mut entries = BTreeMap::new();
        for exact_ref in exact_refs {
            let resolved = self.entries.get(exact_ref).ok_or_else(|| {
                RegistryLoadError::at(
                    RegistryLoadErrorKind::MissingSchema,
                    "schema_registry_sources",
                    "effective profile schema entry is absent from the admitted closure",
                )
            })?;
            entries.insert(exact_ref.clone(), resolved.clone());
        }
        finish_schema_registry(entries)
    }
}

fn insert_resolved_schema(
    entries: &mut BTreeMap<ExactDefinitionRef, ResolvedSchema>,
    exact_ref: ExactDefinitionRef,
    resolved: ResolvedSchema,
) -> Result<(), RegistryLoadError> {
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
    Ok(())
}

fn finish_schema_registry(
    entries: BTreeMap<ExactDefinitionRef, ResolvedSchema>,
) -> Result<SchemaRegistry, RegistryLoadError> {
    let fingerprint_members = entries
        .values()
        .map(|resolved| SchemaRegistryFingerprintMember {
            entry_ref: resolved.entry.exact_ref.as_str(),
            entry_fingerprint: resolved.entry.entry_fingerprint.as_str(),
            closure_fingerprint: resolved.entry.closure_fingerprint.as_str(),
        })
        .collect::<Vec<_>>();
    let fingerprint = fingerprint_serializable(&fingerprint_members)?;
    Ok(SchemaRegistry {
        entries,
        fingerprint,
    })
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
        request_schema_state: &mut RequestSchemaState,
        source_kind: SchemaSourceKind,
    ) -> Result<ResolvedSchema, RegistryLoadError> {
        self.validate_static_contract()?;
        let exact_ref = self.exact_ref()?;
        let normalized_document_ref = normalize_schema_path(repo_root, &self.document_ref)?;
        if normalized_document_ref != self.document_ref {
            return Err(RegistryLoadError::at(
                RegistryLoadErrorKind::InvalidSourcePath,
                "document_ref",
                "schema document_ref must already be normalized",
            ));
        }

        let mut loader = ClosureLoader::new(
            repo_root,
            allowed_roots,
            budget,
            request_schema_state,
            source_kind,
        );
        loader.load_document(&self.document_ref)?;
        loader.validate_reference_graph()?;
        let root = loader.documents.get(&self.document_ref).ok_or_else(|| {
            RegistryLoadError::at(
                RegistryLoadErrorKind::ValidatorTargetMismatch,
                "document_ref",
                "prewalk did not retain the selected root document",
            )
        })?;
        let computed_document_fingerprint = root.fingerprint.clone();
        let closure_members = loader
            .documents
            .values()
            .map(|document| ClosureFingerprintMember {
                document_ref: document.path.as_str(),
                document_fingerprint: document.fingerprint.as_str(),
            })
            .collect::<Vec<_>>();
        let computed_closure_fingerprint = fingerprint_serializable(&closure_members)?;
        let computed_entry_fingerprint = fingerprint_serializable(&self)?;

        let deferred_fingerprint_error = match DefinitionFingerprint::parse(
            &self.document_fingerprint,
        ) {
            Err(error) => Some(error),
            Ok(supplied) if computed_document_fingerprint != supplied => {
                Some(RegistryLoadError::at(
                    RegistryLoadErrorKind::FingerprintMismatch,
                    "document_fingerprint",
                    "schema document fingerprint does not match exact source bytes",
                ))
            }
            Ok(_) => match DefinitionFingerprint::parse(&self.closure_fingerprint) {
                Err(error) => Some(error),
                Ok(supplied) if computed_closure_fingerprint != supplied => {
                    Some(RegistryLoadError::at(
                        RegistryLoadErrorKind::FingerprintMismatch,
                        "closure_fingerprint",
                        "schema closure fingerprint does not match the prewalked closure",
                    ))
                }
                Ok(_) => match DefinitionFingerprint::parse(&self.entry_fingerprint) {
                    Err(error) => Some(error),
                    Ok(supplied) if computed_entry_fingerprint != supplied => {
                        Some(RegistryLoadError::at(
                            RegistryLoadErrorKind::FingerprintMismatch,
                            "entry_fingerprint",
                            "schema-registry entry fingerprint does not match normalized source",
                        ))
                    }
                    Ok(_) => None,
                },
            },
        };

        let validator = build_validator(&self.document_ref, &loader.documents)?;
        let closure_document_refs = loader.documents.keys().cloned().collect();
        let documents = loader.documents;
        Ok(ResolvedSchema {
            entry: SchemaRegistryEntry {
                exact_ref,
                document_ref: self.document_ref,
                document_fingerprint: computed_document_fingerprint,
                closure_fingerprint: computed_closure_fingerprint,
                entry_fingerprint: computed_entry_fingerprint,
            },
            closure_document_refs,
            documents,
            validator,
            deferred_fingerprint_error,
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

pub(crate) fn admitted_schema_entry_exact_ref(
    bytes: &[u8],
) -> Result<ExactDefinitionRef, RegistryLoadError> {
    let authored = AuthoredSchemaRegistryEntry::parse(bytes)?;
    if authored.schema_id != "handbook.schema-registry-entry" || authored.schema_version != "1.0" {
        return Err(RegistryLoadError::new(
            RegistryLoadErrorKind::UnsupportedRecord,
            "schema-registry entry must use handbook.schema-registry-entry / 1.0",
        ));
    }
    authored.exact_ref()
}

#[derive(Clone, Debug)]
struct LoadedSchemaDocument {
    path: String,
    value: Value,
    fingerprint: DefinitionFingerprint,
    schema_pointers: BTreeSet<String>,
    schema_children: BTreeMap<String, Vec<String>>,
    references: Vec<LocalSchemaReference>,
}

#[derive(Default)]
struct RequestSchemaState {
    documents: BTreeMap<SchemaDocumentIdentity, LoadedSchemaDocument>,
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
struct SchemaDocumentIdentity {
    source_kind: SchemaSourceKind,
    path: String,
}

struct ClosureLoader<'a> {
    repo_root: &'a Path,
    allowed_roots: &'a [String],
    budget: &'a mut SourceByteBudget,
    request_schema_state: &'a mut RequestSchemaState,
    source_kind: SchemaSourceKind,
    documents: BTreeMap<String, LoadedSchemaDocument>,
}

const BUILTIN_SCHEMA_ROOT: &str = "definitions/schemas";

impl<'a> ClosureLoader<'a> {
    fn new(
        repo_root: &'a Path,
        allowed_roots: &'a [String],
        budget: &'a mut SourceByteBudget,
        request_schema_state: &'a mut RequestSchemaState,
        source_kind: SchemaSourceKind,
    ) -> Self {
        Self {
            repo_root,
            allowed_roots,
            budget,
            request_schema_state,
            source_kind,
            documents: BTreeMap::new(),
        }
    }

    fn load_document(&mut self, requested_path: &str) -> Result<(), RegistryLoadError> {
        let normalized = normalize_schema_path(self.repo_root, requested_path)?;
        self.require_allowed_root(&normalized)?;
        if self.documents.contains_key(&normalized) {
            return Ok(());
        }
        let source_identity = SchemaDocumentIdentity {
            source_kind: self.source_kind,
            path: normalized.clone(),
        };
        if let Some(cached) = self
            .request_schema_state
            .documents
            .get(&source_identity)
            .cloned()
        {
            self.documents.insert(normalized.clone(), cached);
            return self.load_transitive_and_validate(&normalized);
        }
        if self.request_schema_state.documents.len() >= MAX_CLOSURE_DOCUMENTS {
            return Err(RegistryLoadError::new(
                RegistryLoadErrorKind::DocumentLimitExceeded,
                "request-wide schema closure exceeds 128 distinct documents",
            ));
        }

        let bytes = match self.source_kind {
            SchemaSourceKind::BuiltIn => {
                let bytes =
                    crate::profile_builtins::schema_document(&normalized).ok_or_else(|| {
                        RegistryLoadError::at(
                            RegistryLoadErrorKind::LocalReferenceMissing,
                            "schema_document",
                            "referenced built-in schema document is not compile-time allowlisted",
                        )
                    })?;
                self.budget.admit(bytes.len())?;
                bytes.to_vec()
            }
            SchemaSourceKind::Repository => {
                let (_, bytes) = read_trusted_repo_source(self.repo_root, &normalized, self.budget)
                    .map_err(|error| {
                        if error.kind() == RegistryLoadErrorKind::MissingSource {
                            RegistryLoadError::at(
                                RegistryLoadErrorKind::LocalReferenceMissing,
                                "schema_document",
                                "referenced local schema document does not exist",
                            )
                        } else {
                            error
                        }
                    })?;
                bytes
            }
        };
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

        let document = LoadedSchemaDocument {
            path: normalized.clone(),
            value,
            fingerprint: DefinitionFingerprint::from_bytes(&bytes),
            schema_pointers: profile.schema_pointers,
            schema_children: profile.schema_children,
            references: profile.references,
        };
        self.request_schema_state
            .documents
            .insert(source_identity, document.clone());
        self.documents.insert(normalized.clone(), document);

        self.load_transitive_and_validate(&normalized)
    }

    fn load_transitive_and_validate(&mut self, normalized: &str) -> Result<(), RegistryLoadError> {
        let target_paths = self
            .documents
            .get(normalized)
            .map(|document| {
                document
                    .references
                    .iter()
                    .filter_map(|reference| reference.target_path.clone())
                    .collect::<BTreeSet<_>>()
            })
            .unwrap_or_default();

        for target_path in target_paths {
            self.load_document(&target_path)?;
        }
        let references = self
            .documents
            .get(normalized)
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
        let allowed = match self.source_kind {
            SchemaSourceKind::BuiltIn => {
                path_is_within_schema_root(normalized_path, BUILTIN_SCHEMA_ROOT)
            }
            SchemaSourceKind::Repository => self
                .allowed_roots
                .iter()
                .any(|root| path_is_within_schema_root(normalized_path, root)),
        };
        if allowed {
            return Ok(());
        }
        Err(RegistryLoadError::at(
            RegistryLoadErrorKind::LocalReferenceOutsideRoot,
            normalized_path,
            "schema document is outside every explicitly allowed schema root",
        ))
    }
}

fn path_is_within_schema_root(path: &str, root: &str) -> bool {
    path == root
        || path
            .strip_prefix(root)
            .is_some_and(|suffix| suffix.starts_with('/'))
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
        if !is_allowed_schema_keyword(keyword)
            && !is_hcm_2_2_date_time_format(document_path, keyword, keyword_value)
        {
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
    if !document_part.is_empty() {
        validate_schema_path_limits(document_part, current_document)?;
    }
    validate_json_pointer_fragment(current_document, fragment)?;
    if document_part.is_empty() {
        return Ok(LocalSchemaReference {
            source_pointer: source_pointer.to_owned(),
            target_path: None,
            fragment: fragment.to_owned(),
        });
    }

    let parent = current_document
        .rsplit_once('/')
        .map(|(parent, _)| parent)
        .unwrap_or("");
    let joined = if parent.is_empty() {
        document_part.to_owned()
    } else {
        format!("{parent}/{document_part}")
    };
    let target_path = normalize_schema_path(Path::new("."), &joined)?;
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
    let validator = if root_document_ref
        == "definitions/schemas/handbook.schemas.artifacts.project-authority/1.1.0.schema.json"
    {
        jsonschema::draft202012::options()
            .with_registry(&registry)
            .with_base_uri(internal_resource_uri(root_document_ref))
            .with_pattern_options(PatternOptions::fancy_regex())
            .build(&root_validator_value)
    } else if HCM_2_2_DATE_TIME_SCHEMA_DOCUMENTS.contains(&root_document_ref) {
        jsonschema::draft202012::options()
            .with_registry(&registry)
            .with_base_uri(internal_resource_uri(root_document_ref))
            .with_pattern_options(PatternOptions::regex())
            .should_validate_formats(true)
            .build(&root_validator_value)
    } else {
        jsonschema::draft202012::options()
            .with_registry(&registry)
            .with_base_uri(internal_resource_uri(root_document_ref))
            .with_pattern_options(PatternOptions::regex())
            .build(&root_validator_value)
    };
    validator.map_err(|_| {
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
        validate_schema_path_limits(root, "allowed_schema_root")?;
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
    validate_schema_path_limits(path, "source_path")?;
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

fn validate_schema_path_limits(path: &str, location: &str) -> Result<(), RegistryLoadError> {
    let components = path.split('/').collect::<Vec<_>>();
    let invalid = path.trim() != path
        || !(1..=MAX_SCHEMA_PATH_BYTES).contains(&path.len())
        || !(1..=MAX_SCHEMA_PATH_COMPONENTS).contains(&components.len())
        || path.starts_with('/')
        || path.ends_with('/')
        || path.contains('\\')
        || path.contains('\0')
        || crate::instance_profile::has_uri_scheme_or_drive_prefix(path)
        || components
            .iter()
            .any(|component| component.is_empty() || *component == "." || *component == "..");
    if invalid {
        return Err(RegistryLoadError::at(
            RegistryLoadErrorKind::InvalidSourcePath,
            location,
            "schema path must be normalized and contain 1-1024 bytes and 1-64 components",
        ));
    }
    Ok(())
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

fn is_hcm_2_2_date_time_format(document_path: &str, keyword: &str, value: &Value) -> bool {
    keyword == "format"
        && value.as_str() == Some("date-time")
        && HCM_2_2_DATE_TIME_SCHEMA_DOCUMENTS.contains(&document_path)
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

#[cfg(test)]
mod binding_shape_tests {
    use super::{
        RegistryLoadError, ResolvedBindingCardinality, ResolvedBindingEmptyPolicy,
        ResolvedBindingJsonType, ResolvedSchema, SchemaRegistry,
    };
    use crate::{DefinitionFingerprint, ExactDefinitionRef};
    use serde_json::{json, Value};

    fn load_binding_schema(schema: Value) -> ResolvedSchema {
        try_load_binding_schema(schema).unwrap()
    }

    fn try_load_binding_schema(schema: Value) -> Result<ResolvedSchema, RegistryLoadError> {
        let repo = tempfile::tempdir().unwrap();
        std::fs::create_dir_all(repo.path().join("schemas")).unwrap();
        std::fs::create_dir_all(repo.path().join("definitions")).unwrap();
        let document_ref = "schemas/root.schema.json";
        let bytes = serde_json::to_vec(&schema).unwrap();
        std::fs::write(repo.path().join(document_ref), &bytes).unwrap();
        let document_fingerprint = DefinitionFingerprint::from_bytes(&bytes).to_string();
        let closure = DefinitionFingerprint::from_json_value(&json!([{
            "document_ref": document_ref,
            "document_fingerprint": document_fingerprint,
        }]))
        .unwrap()
        .to_string();
        let preimage = json!({
            "schema_id": "handbook.schema-registry-entry",
            "schema_version": "1.0",
            "content_schema_id": "example.schemas.binding-test",
            "content_schema_version": "1.0.0",
            "document_ref": document_ref,
            "document_fingerprint": document_fingerprint,
            "closure_fingerprint": closure,
            "meta_schema_ref": super::DRAFT_2020_12,
            "media_type": super::SCHEMA_MEDIA_TYPE,
            "compatibility": "exact",
            "extensions": {},
        });
        let mut authored = preimage.as_object().unwrap().clone();
        authored.insert(
            "entry_fingerprint".into(),
            DefinitionFingerprint::from_json_value(&preimage)
                .unwrap()
                .to_string()
                .into(),
        );
        std::fs::write(
            repo.path().join("definitions/binding.entry.yaml"),
            serde_yaml_bw::to_string(&authored).unwrap(),
        )
        .unwrap();
        let mut registry = SchemaRegistry::load(
            repo.path(),
            &["definitions/binding.entry.yaml".into()],
            &["schemas".into()],
        )?;
        Ok(registry
            .entries
            .remove(&ExactDefinitionRef::parse("example.schemas.binding-test@1.0.0").unwrap())
            .unwrap())
    }

    #[test]
    fn project_authority_1_1_binding_shapes_are_exactly_determinate() {
        let registry = SchemaRegistry::load(
            env!("CARGO_MANIFEST_DIR"),
            &[
                "definitions/schemas/handbook.schemas.artifacts.project-authority/1.1.0.entry.yaml"
                    .to_owned(),
            ],
            &["definitions/schemas".to_owned()],
        )
        .unwrap();
        let schema = registry
            .resolved(
                &ExactDefinitionRef::parse("handbook.schemas.artifacts.project-authority@1.1.0")
                    .unwrap(),
            )
            .unwrap();
        for pointer in [
            "/policy",
            "/policy/revision",
            "/governance/decision_authority",
            "/governance/required_approvals",
            "/governance/exception_policy",
            "/engineering_posture/dimensions",
            "/engineering_posture/red_lines",
            "/governance/review_triggers",
            "/governance/reassessment_triggers",
        ] {
            schema
                .binding_shape(pointer)
                .unwrap_or_else(|error| panic!("{pointer}: {error}"));
        }
    }

    #[test]
    fn binding_shape_requires_determinate_required_closed_paths() {
        let repo = tempfile::tempdir().unwrap();
        std::fs::create_dir_all(repo.path().join("schemas")).unwrap();
        std::fs::create_dir_all(repo.path().join("definitions")).unwrap();
        let document_ref = "schemas/root.schema.json";
        let schema = json!({
            "$schema": super::DRAFT_2020_12,
            "type": "object",
            "properties": {
                "policy": {"$ref": "#/$defs/policy"},
                "approvals": {"type": "array", "minItems": 1, "items": {"type": "string"}},
                "ambiguous": {"oneOf": [{"type": "string"}, {"type": "array"}]}
            },
            "$defs": {
                "policy": {
                    "type": "object",
                    "properties": {"revision": {"type": "string", "minLength": 1}},
                    "required": ["revision"],
                    "additionalProperties": false
                }
            },
            "required": ["policy", "approvals"],
            "additionalProperties": false
        });
        let bytes = serde_json::to_vec(&schema).unwrap();
        std::fs::write(repo.path().join(document_ref), &bytes).unwrap();
        let document_fingerprint = DefinitionFingerprint::from_bytes(&bytes).to_string();
        let closure = DefinitionFingerprint::from_json_value(&json!([{
            "document_ref": document_ref,
            "document_fingerprint": document_fingerprint,
        }]))
        .unwrap()
        .to_string();
        let preimage = json!({
            "schema_id": "handbook.schema-registry-entry",
            "schema_version": "1.0",
            "content_schema_id": "example.schemas.binding-shape",
            "content_schema_version": "1.0.0",
            "document_ref": document_ref,
            "document_fingerprint": document_fingerprint,
            "closure_fingerprint": closure,
            "meta_schema_ref": super::DRAFT_2020_12,
            "media_type": super::SCHEMA_MEDIA_TYPE,
            "compatibility": "exact",
            "extensions": {},
        });
        let mut authored = preimage.as_object().unwrap().clone();
        authored.insert(
            "entry_fingerprint".into(),
            DefinitionFingerprint::from_json_value(&Value::Object(
                preimage.as_object().unwrap().clone(),
            ))
            .unwrap()
            .to_string()
            .into(),
        );
        std::fs::write(
            repo.path().join("definitions/binding.entry.yaml"),
            serde_yaml_bw::to_string(&authored).unwrap(),
        )
        .unwrap();
        let registry = SchemaRegistry::load(
            repo.path(),
            &["definitions/binding.entry.yaml".into()],
            &["schemas".into()],
        )
        .unwrap();
        let schema_ref = ExactDefinitionRef::parse("example.schemas.binding-shape@1.0.0").unwrap();
        let resolved = registry.resolved(&schema_ref).unwrap();

        let policy = resolved.binding_shape("/policy").unwrap();
        assert_eq!(policy.json_type(), ResolvedBindingJsonType::Object);
        assert_eq!(policy.cardinality(), ResolvedBindingCardinality::Singular);
        assert_eq!(policy.empty_policy(), ResolvedBindingEmptyPolicy::Forbidden);

        let revision = resolved.binding_shape("/policy/revision").unwrap();
        assert_eq!(revision.json_type(), ResolvedBindingJsonType::String);
        assert_eq!(
            revision.empty_policy(),
            ResolvedBindingEmptyPolicy::Forbidden
        );

        let approvals = resolved.binding_shape("/approvals").unwrap();
        assert_eq!(approvals.json_type(), ResolvedBindingJsonType::Array);
        assert_eq!(approvals.cardinality(), ResolvedBindingCardinality::Plural);
        assert_eq!(
            approvals.empty_policy(),
            ResolvedBindingEmptyPolicy::Forbidden
        );

        for pointer in ["policy", "/missing", "/ambiguous"] {
            assert!(resolved.binding_shape(pointer).is_err(), "{pointer}");
        }
    }

    #[test]
    fn binding_shape_refuses_open_parents_and_semantic_ref_siblings() {
        let open = load_binding_schema(json!({
            "$schema": super::DRAFT_2020_12,
            "type": "object",
            "properties": {"policy": {"type": "string", "minLength": 1}},
            "required": ["policy"]
        }));
        assert!(open.binding_shape("/policy").is_err());

        let semantic_sibling = load_binding_schema(json!({
            "$schema": super::DRAFT_2020_12,
            "type": "object",
            "properties": {
                "policy": {"$ref": "#/$defs/policy", "minLength": 1}
            },
            "$defs": {"policy": {"type": "string", "minLength": 1}},
            "required": ["policy"],
            "additionalProperties": false
        }));
        assert!(semantic_sibling.binding_shape("/policy").is_err());

        let annotation_sibling = load_binding_schema(json!({
            "$schema": super::DRAFT_2020_12,
            "type": "object",
            "properties": {
                "policy": {"$ref": "#/$defs/policy", "description": "annotation only"}
            },
            "$defs": {"policy": {"type": "string", "minLength": 1}},
            "required": ["policy"],
            "additionalProperties": false
        }));
        assert!(annotation_sibling.binding_shape("/policy").is_ok());
    }

    #[test]
    fn binding_shape_refuses_pattern_opened_parents_and_object_terminals() {
        let opened_parent = load_binding_schema(json!({
            "$schema": super::DRAFT_2020_12,
            "type": "object",
            "properties": {
                "policy": {"type": "string", "minLength": 1}
            },
            "patternProperties": {"^x-": {"type": "string"}},
            "required": ["policy"],
            "additionalProperties": false
        }));
        assert!(opened_parent.binding_shape("/policy").is_err());

        let opened_terminal = load_binding_schema(json!({
            "$schema": super::DRAFT_2020_12,
            "type": "object",
            "properties": {
                "policy": {
                    "type": "object",
                    "properties": {"revision": {"type": "string"}},
                    "patternProperties": {".*": {"type": "string"}},
                    "required": ["revision"],
                    "additionalProperties": false
                }
            },
            "required": ["policy"],
            "additionalProperties": false
        }));
        assert!(opened_terminal.binding_shape("/policy").is_err());
    }

    #[test]
    fn binding_shape_refuses_contradictory_object_array_and_string_terminals() {
        for terminal in [
            json!({"type": "string", "minLength": 1, "maxLength": 0}),
            json!({"type": "string", "minLength": 1, "not": {}}),
            json!({
                "type": "array",
                "minItems": 1,
                "maxItems": 0,
                "items": {"type": "string"}
            }),
            json!({
                "type": "array",
                "minItems": 1,
                "items": false
            }),
            json!({
                "type": "object",
                "properties": {},
                "required": ["missing"],
                "additionalProperties": false
            }),
            json!({
                "type": "object",
                "properties": {"required_value": {"not": {}}},
                "required": ["required_value"],
                "additionalProperties": false
            }),
        ] {
            let resolved = load_binding_schema(json!({
                "$schema": super::DRAFT_2020_12,
                "type": "object",
                "properties": {"policy": terminal},
                "required": ["policy"],
                "additionalProperties": false
            }));
            assert!(resolved.binding_shape("/policy").is_err());
        }

        for terminal in [
            json!({
                "type": "array",
                "minItems": 2,
                "items": {"enum": ["a", "b"]},
                "uniqueItems": true,
                "contains": {"enum": ["a", "b"]},
                "minContains": 2,
                "maxContains": 2
            }),
            json!({
                "type": "array",
                "minItems": 2,
                "maxItems": 3,
                "prefixItems": [{"const": "a"}, {"const": "b"}],
                "items": {"const": "c"},
                "contains": {"const": "c"},
                "minContains": 1
            }),
            json!({
                "type": "object",
                "properties": {
                    "a": {"type": "string"},
                    "b": {"type": "string"}
                },
                "required": [],
                "minProperties": 1,
                "maxProperties": 1,
                "dependentSchemas": {"a": false},
                "additionalProperties": false
            }),
            json!({
                "type": "object",
                "properties": {
                    "a": {"type": "string"},
                    "b": {"type": "string"}
                },
                "required": [],
                "minProperties": 1,
                "maxProperties": 2,
                "dependentRequired": {"a": ["b"]},
                "additionalProperties": false
            }),
        ] {
            let resolved = load_binding_schema(json!({
                "$schema": super::DRAFT_2020_12,
                "type": "object",
                "properties": {"policy": terminal},
                "required": ["policy"],
                "additionalProperties": false
            }));
            assert!(resolved.binding_shape("/policy").is_ok());
        }
    }

    #[test]
    fn binding_shape_refuses_unsatisfiable_root_intermediate_and_referenced_parents() {
        for schema in [
            json!({
                "$schema": super::DRAFT_2020_12,
                "type": "object",
                "properties": {
                    "policy": {"type": "string", "minLength": 1}
                },
                "required": ["policy"],
                "maxProperties": 0,
                "additionalProperties": false
            }),
            json!({
                "$schema": super::DRAFT_2020_12,
                "type": "object",
                "properties": {
                    "policy": {
                        "type": "object",
                        "properties": {
                            "revision": {"type": "string", "minLength": 1}
                        },
                        "required": ["revision"],
                        "maxProperties": 0,
                        "additionalProperties": false
                    }
                },
                "required": ["policy"],
                "additionalProperties": false
            }),
            json!({
                "$schema": super::DRAFT_2020_12,
                "type": "object",
                "properties": {
                    "policy": {"$ref": "#/$defs/impossibleParent"}
                },
                "required": ["policy"],
                "additionalProperties": false,
                "$defs": {
                    "impossibleParent": {
                        "type": "object",
                        "properties": {
                            "revision": {"type": "string", "minLength": 1}
                        },
                        "required": ["revision"],
                        "maxProperties": 0,
                        "additionalProperties": false
                    }
                }
            }),
        ] {
            let resolved = load_binding_schema(schema);
            assert!(resolved.binding_shape("/policy/revision").is_err());
        }
    }

    #[test]
    fn binding_shape_refuses_unsatisfiable_required_children_and_object_dependencies() {
        for (schema, pointer) in [
            (
                json!({
                    "$schema": super::DRAFT_2020_12,
                    "type": "object",
                    "properties": {
                        "policy": {"type": "string", "minLength": 1},
                        "blocker": false
                    },
                    "required": ["policy", "blocker"],
                    "additionalProperties": false
                }),
                "/policy",
            ),
            (
                json!({
                    "$schema": super::DRAFT_2020_12,
                    "$ref": "#/$defs/root",
                    "$defs": {
                        "root": {
                            "type": "object",
                            "properties": {
                                "policy": {"type": "string", "minLength": 1},
                                "blocker": {"type": "string", "minLength": 2, "maxLength": 1}
                            },
                            "required": ["policy", "blocker"],
                            "additionalProperties": false
                        }
                    }
                }),
                "/policy",
            ),
            (
                json!({
                    "$schema": super::DRAFT_2020_12,
                    "type": "object",
                    "properties": {
                        "policy": {"type": "string", "minLength": 1},
                        "blocker": {"$ref": "#/$defs/impossible"}
                    },
                    "required": ["policy", "blocker"],
                    "additionalProperties": false,
                    "$defs": {"impossible": false}
                }),
                "/policy",
            ),
            (
                json!({
                    "$schema": super::DRAFT_2020_12,
                    "type": "object",
                    "properties": {
                        "outer": {
                            "type": "object",
                            "properties": {
                                "policy": {"type": "string", "minLength": 1},
                                "blocker": false
                            },
                            "required": ["policy", "blocker"],
                            "additionalProperties": false
                        }
                    },
                    "required": ["outer"],
                    "additionalProperties": false
                }),
                "/outer/policy",
            ),
            (
                json!({
                    "$schema": super::DRAFT_2020_12,
                    "type": "object",
                    "properties": {"policy": {"type": "string", "minLength": 1}},
                    "required": ["policy"],
                    "dependentRequired": {"policy": ["missing"]},
                    "additionalProperties": false
                }),
                "/policy",
            ),
            (
                json!({
                    "$schema": super::DRAFT_2020_12,
                    "type": "object",
                    "properties": {"policy": {"type": "string", "minLength": 1}},
                    "required": ["policy"],
                    "propertyNames": false,
                    "additionalProperties": false
                }),
                "/policy",
            ),
        ] {
            let resolved = load_binding_schema(schema);
            assert!(resolved.binding_shape(pointer).is_err(), "{pointer}");
        }
    }

    #[test]
    fn binding_shape_applies_default_min_contains_before_certifying_arrays() {
        let resolved = load_binding_schema(json!({
            "$schema": super::DRAFT_2020_12,
            "type": "object",
            "properties": {
                "policy": {
                    "type": "array",
                    "minItems": 1,
                    "contains": {},
                    "maxContains": 0
                }
            },
            "required": ["policy"],
            "additionalProperties": false
        }));
        assert!(resolved.binding_shape("/policy").is_err());
    }

    #[test]
    fn binding_shape_refuses_cross_keyword_array_and_object_contradictions() {
        for terminal in [
            json!({
                "type": "array",
                "maxItems": 1,
                "contains": {},
                "minContains": 2
            }),
            json!({
                "type": "array",
                "items": false,
                "contains": {}
            }),
            json!({
                "type": "object",
                "properties": {},
                "required": [],
                "minProperties": 1,
                "additionalProperties": false
            }),
            json!({
                "type": "object",
                "properties": {"candidate": {"type": "string"}},
                "required": [],
                "propertyNames": false,
                "minProperties": 1,
                "additionalProperties": false
            }),
        ] {
            let resolved = load_binding_schema(json!({
                "$schema": super::DRAFT_2020_12,
                "type": "object",
                "properties": {"policy": terminal},
                "required": ["policy"],
                "additionalProperties": false
            }));
            assert!(resolved.binding_shape("/policy").is_err());
        }

        for terminal in [
            json!({
                "type": "array",
                "minItems": 2,
                "items": {"enum": ["first", "second"]},
                "uniqueItems": true
            }),
            json!({
                "type": "array",
                "minItems": 2,
                "contains": true,
                "maxContains": 2
            }),
            json!({
                "type": "array",
                "minItems": 1,
                "prefixItems": [true]
            }),
            json!({
                "type": "object",
                "properties": {"a": {"type": "string"}},
                "required": ["a"],
                "dependentSchemas": {"a": true},
                "additionalProperties": false
            }),
        ] {
            let resolved = load_binding_schema(json!({
                "$schema": super::DRAFT_2020_12,
                "type": "object",
                "properties": {"policy": terminal},
                "required": ["policy"],
                "additionalProperties": false
            }));
            assert!(resolved.binding_shape("/policy").is_ok());
        }
    }

    #[test]
    fn binding_shape_refuses_cross_keyword_contradictions_in_required_references() {
        for impossible in [
            json!({
                "type": "array",
                "maxItems": 1,
                "contains": {},
                "minContains": 2
            }),
            json!({
                "type": "object",
                "properties": {},
                "required": [],
                "minProperties": 1,
                "additionalProperties": false
            }),
        ] {
            let resolved = load_binding_schema(json!({
                "$schema": super::DRAFT_2020_12,
                "type": "object",
                "properties": {
                    "policy": {"type": "string", "minLength": 1},
                    "blocker": {"$ref": "#/$defs/impossible"}
                },
                "required": ["policy", "blocker"],
                "additionalProperties": false,
                "$defs": {"impossible": impossible}
            }));
            assert!(resolved.binding_shape("/policy").is_err());
        }
    }

    #[test]
    fn binding_shape_refuses_unproven_joint_constraint_witnesses() {
        for terminal in [
            json!({
                "type": "array",
                "items": {"type": "string"},
                "contains": {"type": "object"}
            }),
            json!({
                "type": "object",
                "properties": {"only": false},
                "required": [],
                "minProperties": 1,
                "additionalProperties": false
            }),
        ] {
            let resolved = load_binding_schema(json!({
                "$schema": super::DRAFT_2020_12,
                "type": "object",
                "properties": {"policy": terminal},
                "required": ["policy"],
                "additionalProperties": false
            }));
            assert!(resolved.binding_shape("/policy").is_err());
        }

        for impossible in [
            json!({"type": "integer", "minimum": 1, "maximum": 0}),
            json!({
                "type": "array",
                "items": {"type": "string"},
                "contains": {"type": "object"}
            }),
            json!({
                "type": "object",
                "properties": {"only": false},
                "required": [],
                "minProperties": 1,
                "additionalProperties": false
            }),
        ] {
            let resolved = load_binding_schema(json!({
                "$schema": super::DRAFT_2020_12,
                "type": "object",
                "properties": {
                    "outer": {
                        "type": "object",
                        "properties": {
                            "policy": {"type": "string", "minLength": 1},
                            "blocker": {"$ref": "#/$defs/impossible"}
                        },
                        "required": ["policy", "blocker"],
                        "additionalProperties": false
                    }
                },
                "required": ["outer"],
                "additionalProperties": false,
                "$defs": {"impossible": impossible}
            }));
            assert!(resolved.binding_shape("/outer/policy").is_err());
        }
    }

    #[test]
    fn binding_shape_refuses_advanced_array_and_dependency_contradictions() {
        for terminal in [
            json!({
                "type": "array",
                "minItems": 2,
                "items": {"const": "only"},
                "uniqueItems": true
            }),
            json!({
                "type": "array",
                "minItems": 2,
                "contains": true,
                "minContains": 0,
                "maxContains": 1
            }),
            json!({
                "type": "array",
                "minItems": 1,
                "prefixItems": [false]
            }),
            json!({
                "type": "object",
                "properties": {"a": {"type": "string"}},
                "required": ["a"],
                "dependentSchemas": {"a": false},
                "additionalProperties": false
            }),
        ] {
            let resolved = load_binding_schema(json!({
                "$schema": super::DRAFT_2020_12,
                "type": "object",
                "properties": {"policy": terminal},
                "required": ["policy"],
                "additionalProperties": false
            }));
            assert!(resolved.binding_shape("/policy").is_err());
        }
    }

    #[test]
    fn binding_shape_refuses_advanced_contradictions_in_required_references() {
        for impossible in [
            json!({
                "type": "array",
                "minItems": 2,
                "items": {"const": "only"},
                "uniqueItems": true
            }),
            json!({
                "type": "array",
                "minItems": 2,
                "contains": true,
                "minContains": 0,
                "maxContains": 1
            }),
            json!({
                "type": "array",
                "minItems": 1,
                "prefixItems": [false]
            }),
            json!({
                "type": "object",
                "properties": {"a": {"type": "string"}},
                "required": ["a"],
                "dependentSchemas": {"a": false},
                "additionalProperties": false
            }),
        ] {
            let resolved = load_binding_schema(json!({
                "$schema": super::DRAFT_2020_12,
                "type": "object",
                "properties": {
                    "outer": {
                        "type": "object",
                        "properties": {
                            "policy": {"type": "string", "minLength": 1},
                            "blocker": {"$ref": "#/$defs/impossible"}
                        },
                        "required": ["policy", "blocker"],
                        "additionalProperties": false
                    }
                },
                "required": ["outer"],
                "additionalProperties": false,
                "$defs": {"impossible": impossible}
            }));
            assert!(resolved.binding_shape("/outer/policy").is_err());
        }
    }

    #[test]
    fn binding_shape_refuses_joint_array_and_optional_dependency_contradictions() {
        for terminal in [
            json!({
                "type": "array",
                "minItems": 2,
                "items": {"enum": ["a", "b"]},
                "uniqueItems": true,
                "contains": {"const": "a"},
                "minContains": 2
            }),
            json!({
                "type": "array",
                "minItems": 2,
                "maxItems": 2,
                "prefixItems": [{"const": "a"}, {"const": "b"}],
                "contains": {"const": "c"},
                "minContains": 1
            }),
            json!({
                "type": "object",
                "properties": {"a": {"type": "string"}},
                "required": [],
                "minProperties": 1,
                "dependentSchemas": {"a": false},
                "additionalProperties": false
            }),
            json!({
                "type": "object",
                "properties": {
                    "a": {"type": "string"},
                    "b": {"type": "string"}
                },
                "required": [],
                "minProperties": 1,
                "maxProperties": 1,
                "dependentRequired": {"a": ["b"], "b": ["a"]},
                "additionalProperties": false
            }),
        ] {
            let resolved = load_binding_schema(json!({
                "$schema": super::DRAFT_2020_12,
                "type": "object",
                "properties": {"policy": terminal},
                "required": ["policy"],
                "additionalProperties": false
            }));
            assert!(resolved.binding_shape("/policy").is_err());
        }
    }

    #[test]
    fn binding_shape_refuses_joint_contradictions_in_required_references() {
        for impossible in [
            json!({
                "type": "array",
                "minItems": 2,
                "items": {"enum": ["a", "b"]},
                "uniqueItems": true,
                "contains": {"const": "a"},
                "minContains": 2
            }),
            json!({
                "type": "array",
                "minItems": 2,
                "maxItems": 2,
                "prefixItems": [{"const": "a"}, {"const": "b"}],
                "contains": {"const": "c"},
                "minContains": 1
            }),
            json!({
                "type": "object",
                "properties": {"a": {"type": "string"}},
                "required": [],
                "minProperties": 1,
                "dependentSchemas": {"a": false},
                "additionalProperties": false
            }),
            json!({
                "type": "object",
                "properties": {
                    "a": {"type": "string"},
                    "b": {"type": "string"}
                },
                "required": [],
                "minProperties": 1,
                "maxProperties": 1,
                "dependentRequired": {"a": ["b"], "b": ["a"]},
                "additionalProperties": false
            }),
        ] {
            let resolved = load_binding_schema(json!({
                "$schema": super::DRAFT_2020_12,
                "type": "object",
                "properties": {
                    "outer": {
                        "type": "object",
                        "properties": {
                            "policy": {"type": "string", "minLength": 1},
                            "blocker": {"$ref": "#/$defs/impossible"}
                        },
                        "required": ["policy", "blocker"],
                        "additionalProperties": false
                    }
                },
                "required": ["outer"],
                "additionalProperties": false,
                "$defs": {"impossible": impossible}
            }));
            assert!(resolved.binding_shape("/outer/policy").is_err());
        }
    }

    #[test]
    fn binding_shape_refuses_required_unevaluated_items() {
        let impossible = load_binding_schema(json!({
            "$schema": super::DRAFT_2020_12,
            "type": "object",
            "properties": {
                "policy": {
                    "type": "array",
                    "minItems": 1,
                    "unevaluatedItems": false
                }
            },
            "required": ["policy"],
            "additionalProperties": false
        }));
        assert!(impossible.binding_shape("/policy").is_err());

        let empty = load_binding_schema(json!({
            "$schema": super::DRAFT_2020_12,
            "type": "object",
            "properties": {
                "policy": {
                    "type": "array",
                    "minItems": 0,
                    "maxItems": 0,
                    "unevaluatedItems": false
                }
            },
            "required": ["policy"],
            "additionalProperties": false
        }));
        assert!(empty.binding_shape("/policy").is_ok());
    }

    #[test]
    fn binding_shape_refuses_required_unevaluated_items_in_references() {
        let resolved = load_binding_schema(json!({
            "$schema": super::DRAFT_2020_12,
            "type": "object",
            "properties": {
                "outer": {
                    "type": "object",
                    "properties": {
                        "policy": {"type": "string", "minLength": 1},
                        "blocker": {"$ref": "#/$defs/impossible"}
                    },
                    "required": ["policy", "blocker"],
                    "additionalProperties": false
                }
            },
            "required": ["outer"],
            "additionalProperties": false,
            "$defs": {
                "impossible": {
                    "type": "array",
                    "minItems": 1,
                    "unevaluatedItems": false
                }
            }
        }));
        assert!(resolved.binding_shape("/outer/policy").is_err());
    }

    #[test]
    fn binding_shape_refuses_required_unevaluated_properties() {
        let impossible = load_binding_schema(json!({
            "$schema": super::DRAFT_2020_12,
            "type": "object",
            "properties": {
                "policy": {"type": "string", "minLength": 1},
                "blocker": {
                    "type": "object",
                    "minProperties": 1,
                    "unevaluatedProperties": false
                }
            },
            "required": ["policy", "blocker"],
            "additionalProperties": false
        }));
        assert!(impossible.binding_shape("/policy").is_err());

        let declared = load_binding_schema(json!({
            "$schema": super::DRAFT_2020_12,
            "type": "object",
            "properties": {
                "policy": {"type": "string", "minLength": 1},
                "context": {
                    "type": "object",
                    "properties": {"a": {"type": "string"}},
                    "minProperties": 1,
                    "unevaluatedProperties": false
                }
            },
            "required": ["policy", "context"],
            "additionalProperties": false
        }));
        assert!(declared.binding_shape("/policy").is_ok());
    }

    #[test]
    fn binding_shape_refuses_required_unevaluated_properties_in_references() {
        let resolved = load_binding_schema(json!({
            "$schema": super::DRAFT_2020_12,
            "type": "object",
            "properties": {
                "outer": {
                    "type": "object",
                    "properties": {
                        "policy": {"type": "string", "minLength": 1},
                        "blocker": {"$ref": "#/$defs/impossible"}
                    },
                    "required": ["policy", "blocker"],
                    "additionalProperties": false
                }
            },
            "required": ["outer"],
            "additionalProperties": false,
            "$defs": {
                "impossible": {
                    "type": "object",
                    "minProperties": 1,
                    "unevaluatedProperties": false
                }
            }
        }));
        assert!(resolved.binding_shape("/outer/policy").is_err());
    }

    #[test]
    fn binding_shape_proves_singleton_array_valued_object_types() {
        let impossible = load_binding_schema(json!({
            "$schema": super::DRAFT_2020_12,
            "type": "object",
            "properties": {
                "policy": {"type": "string", "minLength": 1},
                "blocker": {
                    "type": ["object"],
                    "minProperties": 1,
                    "unevaluatedProperties": false
                }
            },
            "required": ["policy", "blocker"],
            "additionalProperties": false
        }));
        assert!(impossible.binding_shape("/policy").is_err());

        let declared = load_binding_schema(json!({
            "$schema": super::DRAFT_2020_12,
            "type": "object",
            "properties": {
                "policy": {"type": "string", "minLength": 1},
                "context": {
                    "type": ["object"],
                    "properties": {"a": {"type": "string"}},
                    "minProperties": 1,
                    "unevaluatedProperties": false
                }
            },
            "required": ["policy", "context"],
            "additionalProperties": false
        }));
        assert!(declared.binding_shape("/policy").is_ok());
    }

    #[test]
    fn binding_shape_refuses_array_valued_object_types_in_references() {
        let resolved = load_binding_schema(json!({
            "$schema": super::DRAFT_2020_12,
            "type": "object",
            "properties": {
                "outer": {
                    "type": "object",
                    "properties": {
                        "policy": {"type": "string", "minLength": 1},
                        "blocker": {"$ref": "#/$defs/impossible"}
                    },
                    "required": ["policy", "blocker"],
                    "additionalProperties": false
                }
            },
            "required": ["outer"],
            "additionalProperties": false,
            "$defs": {
                "impossible": {
                    "type": ["object"],
                    "minProperties": 1,
                    "unevaluatedProperties": false
                }
            }
        }));
        assert!(resolved.binding_shape("/outer/policy").is_err());
    }

    #[test]
    fn binding_shape_refuses_unwitnessed_open_object_minimums() {
        for blocker in [
            json!({
                "type": "object",
                "minProperties": 1,
                "propertyNames": false
            }),
            json!({
                "type": "object",
                "minProperties": 1,
                "additionalProperties": {
                    "type": "string",
                    "minLength": 2,
                    "maxLength": 1
                }
            }),
        ] {
            let impossible = load_binding_schema(json!({
                "$schema": super::DRAFT_2020_12,
                "type": "object",
                "properties": {
                    "policy": {"type": "string", "minLength": 1},
                    "blocker": blocker
                },
                "required": ["policy", "blocker"],
                "additionalProperties": false
            }));
            assert!(impossible.binding_shape("/policy").is_err());
        }

        let declared = load_binding_schema(json!({
            "$schema": super::DRAFT_2020_12,
            "type": "object",
            "properties": {
                "policy": {"type": "string", "minLength": 1},
                "context": {
                    "type": "object",
                    "properties": {"a": {"type": "string"}},
                    "propertyNames": {"pattern": "^[a-z]+$"},
                    "minProperties": 1
                }
            },
            "required": ["policy", "context"],
            "additionalProperties": false
        }));
        assert!(declared.binding_shape("/policy").is_ok());
    }

    #[test]
    fn binding_shape_refuses_unwitnessed_open_object_minimums_in_references() {
        for blocker in [
            json!({
                "type": "object",
                "minProperties": 1,
                "propertyNames": false
            }),
            json!({
                "type": "object",
                "minProperties": 1,
                "additionalProperties": {
                    "type": "string",
                    "minLength": 2,
                    "maxLength": 1
                }
            }),
        ] {
            let resolved = load_binding_schema(json!({
                "$schema": super::DRAFT_2020_12,
                "type": "object",
                "properties": {
                    "outer": {
                        "type": "object",
                        "properties": {
                            "policy": {"type": "string", "minLength": 1},
                            "blocker": {"$ref": "#/$defs/impossible"}
                        },
                        "required": ["policy", "blocker"],
                        "additionalProperties": false
                    }
                },
                "required": ["outer"],
                "additionalProperties": false,
                "$defs": {"impossible": blocker}
            }));
            assert!(resolved.binding_shape("/outer/policy").is_err());
        }
    }

    #[test]
    fn binding_shape_resolves_references_in_optional_minimum_witnesses() {
        let impossible = load_binding_schema(json!({
            "$schema": super::DRAFT_2020_12,
            "type": "object",
            "properties": {
                "policy": {"type": "string", "minLength": 1},
                "blocker": {
                    "type": "object",
                    "properties": {
                        "candidate": {
                            "type": "object",
                            "properties": {"nested": {"$ref": "#/$defs/impossible"}},
                            "required": ["nested"],
                            "additionalProperties": false
                        }
                    },
                    "minProperties": 1
                }
            },
            "required": ["policy", "blocker"],
            "additionalProperties": false,
            "$defs": {"impossible": false}
        }));
        assert!(impossible.binding_shape("/policy").is_err());

        let possible = load_binding_schema(json!({
            "$schema": super::DRAFT_2020_12,
            "type": "object",
            "properties": {
                "policy": {"type": "string", "minLength": 1},
                "context": {
                    "type": "object",
                    "properties": {
                        "candidate": {
                            "type": "object",
                            "properties": {"nested": {"$ref": "#/$defs/possible"}},
                            "required": ["nested"],
                            "additionalProperties": false
                        }
                    },
                    "minProperties": 1
                }
            },
            "required": ["policy", "context"],
            "additionalProperties": false,
            "$defs": {"possible": {"type": "string", "minLength": 1}}
        }));
        assert!(possible.binding_shape("/policy").is_ok());
    }

    #[test]
    fn binding_shape_resolves_optional_minimum_witnesses_in_references() {
        let resolved = load_binding_schema(json!({
            "$schema": super::DRAFT_2020_12,
            "type": "object",
            "properties": {
                "outer": {
                    "type": "object",
                    "properties": {
                        "policy": {"type": "string", "minLength": 1},
                        "blocker": {"$ref": "#/$defs/blocker"}
                    },
                    "required": ["policy", "blocker"],
                    "additionalProperties": false
                }
            },
            "required": ["outer"],
            "additionalProperties": false,
            "$defs": {
                "blocker": {
                    "type": "object",
                    "properties": {
                        "candidate": {
                            "type": "object",
                            "properties": {"nested": {"$ref": "#/$defs/impossible"}},
                            "required": ["nested"],
                            "additionalProperties": false
                        }
                    },
                    "minProperties": 1
                },
                "impossible": false
            }
        }));
        assert!(resolved.binding_shape("/outer/policy").is_err());
    }

    #[test]
    fn binding_shape_resolves_children_in_array_valued_type_branches() {
        for blocker in [
            json!({
                "type": ["object"],
                "properties": {"nested": {"$ref": "#/$defs/impossible"}},
                "required": ["nested"],
                "additionalProperties": false
            }),
            json!({
                "type": ["array"],
                "minItems": 1,
                "items": {"$ref": "#/$defs/impossible"}
            }),
        ] {
            let impossible = load_binding_schema(json!({
                "$schema": super::DRAFT_2020_12,
                "type": "object",
                "properties": {
                    "policy": {"type": "string", "minLength": 1},
                    "blocker": blocker
                },
                "required": ["policy", "blocker"],
                "additionalProperties": false,
                "$defs": {"impossible": false}
            }));
            assert!(impossible.binding_shape("/policy").is_err());
        }

        for context in [
            json!({
                "type": ["object"],
                "properties": {"nested": {"$ref": "#/$defs/possible"}},
                "required": ["nested"],
                "additionalProperties": false
            }),
            json!({
                "type": ["array"],
                "minItems": 1,
                "items": {"$ref": "#/$defs/possible"}
            }),
        ] {
            let possible = load_binding_schema(json!({
                "$schema": super::DRAFT_2020_12,
                "type": "object",
                "properties": {
                    "policy": {"type": "string", "minLength": 1},
                    "context": context
                },
                "required": ["policy", "context"],
                "additionalProperties": false,
                "$defs": {"possible": {"type": "string", "minLength": 1}}
            }));
            assert!(possible.binding_shape("/policy").is_ok());
        }
    }

    #[test]
    fn binding_shape_resolves_array_valued_type_branches_in_references() {
        for blocker in [
            json!({
                "type": ["object"],
                "properties": {"nested": {"$ref": "#/$defs/impossible"}},
                "required": ["nested"],
                "additionalProperties": false
            }),
            json!({
                "type": ["array"],
                "minItems": 1,
                "items": {"$ref": "#/$defs/impossible"}
            }),
        ] {
            let resolved = load_binding_schema(json!({
                "$schema": super::DRAFT_2020_12,
                "type": "object",
                "properties": {
                    "outer": {
                        "type": "object",
                        "properties": {
                            "policy": {"type": "string", "minLength": 1},
                            "blocker": {"$ref": "#/$defs/blocker"}
                        },
                        "required": ["policy", "blocker"],
                        "additionalProperties": false
                    }
                },
                "required": ["outer"],
                "additionalProperties": false,
                "$defs": {
                    "blocker": blocker,
                    "impossible": false
                }
            }));
            assert!(resolved.binding_shape("/outer/policy").is_err());
        }
    }

    #[test]
    fn binding_shape_accepts_fully_prefixed_joint_array_witnesses() {
        let resolved = load_binding_schema(json!({
            "$schema": super::DRAFT_2020_12,
            "type": "object",
            "properties": {
                "policy": {"type": "string", "minLength": 1},
                "context": {
                    "type": ["array"],
                    "minItems": 1,
                    "maxItems": 1,
                    "prefixItems": [{"const": "a"}],
                    "items": false,
                    "contains": {"const": "a"},
                    "minContains": 1,
                    "maxContains": 1,
                    "uniqueItems": true,
                    "unevaluatedItems": false
                }
            },
            "required": ["policy", "context"],
            "additionalProperties": false
        }));
        assert!(resolved.binding_shape("/policy").is_ok());
    }

    #[test]
    fn binding_shape_accepts_fully_prefixed_joint_array_witnesses_in_references() {
        let resolved = load_binding_schema(json!({
            "$schema": super::DRAFT_2020_12,
            "type": "object",
            "properties": {
                "outer": {
                    "type": "object",
                    "properties": {
                        "policy": {"type": "string", "minLength": 1},
                        "context": {"$ref": "#/$defs/prefixed"}
                    },
                    "required": ["policy", "context"],
                    "additionalProperties": false
                }
            },
            "required": ["outer"],
            "additionalProperties": false,
            "$defs": {
                "prefixed": {
                    "type": ["array"],
                    "minItems": 1,
                    "maxItems": 1,
                    "prefixItems": [{"const": "a"}],
                    "items": false,
                    "contains": {"const": "a"},
                    "minContains": 1,
                    "maxContains": 1,
                    "uniqueItems": true,
                    "unevaluatedItems": false
                }
            }
        }));
        assert!(resolved.binding_shape("/outer/policy").is_ok());
    }

    #[test]
    fn binding_shape_proves_direct_ref_optional_minimum_witnesses() {
        for (index, context) in [
            json!({
                "type": "object",
                "properties": {"candidate": {"$ref": "#/$defs/possible"}},
                "minProperties": 1,
                "additionalProperties": false
            }),
            json!({"$ref": "#/$defs/context"}),
        ]
        .into_iter()
        .enumerate()
        {
            let resolved = load_binding_schema(json!({
                "$schema": super::DRAFT_2020_12,
                "type": "object",
                "properties": {
                    "policy": {"type": "string", "minLength": 1},
                    "context": context
                },
                "required": ["policy", "context"],
                "additionalProperties": false,
                "$defs": {
                    "possible": {"type": "string", "minLength": 1},
                    "context": {
                        "type": "object",
                        "properties": {"candidate": {"$ref": "#/$defs/possible"}},
                        "minProperties": 1,
                        "additionalProperties": false
                    }
                }
            }));
            let shape = resolved.binding_shape("/policy");
            assert!(
                shape.is_ok(),
                "direct reference witness case {index}: {shape:?}"
            );
        }

        let impossible = load_binding_schema(json!({
            "$schema": super::DRAFT_2020_12,
            "type": "object",
            "properties": {
                "policy": {"type": "string", "minLength": 1},
                "context": {
                    "type": "object",
                    "properties": {"candidate": {"$ref": "#/$defs/impossible"}},
                    "minProperties": 1,
                    "additionalProperties": false
                }
            },
            "required": ["policy", "context"],
            "additionalProperties": false,
            "$defs": {"impossible": false}
        }));
        assert!(impossible.binding_shape("/policy").is_err());

        assert!(try_load_binding_schema(json!({
            "$schema": super::DRAFT_2020_12,
            "type": "object",
            "properties": {
                "policy": {"type": "string", "minLength": 1},
                "context": {
                    "type": "object",
                    "properties": {"candidate": {"$ref": "#/$defs/impossible"}},
                    "minProperties": 1,
                    "additionalProperties": false
                }
            },
            "required": ["policy", "context"],
            "additionalProperties": false,
            "$defs": {
                "impossible": {"$ref": "#/$defs/cycle"},
                "cycle": {"$ref": "#/$defs/impossible"}
            }
        }))
        .is_err());
    }

    #[test]
    fn binding_shape_accepts_fully_prefix_covered_array_terminals() {
        let prefixed = json!({
            "type": "array",
            "minItems": 1,
            "maxItems": 1,
            "prefixItems": [{"const": "a"}],
            "items": false,
            "contains": {"const": "a"},
            "minContains": 1,
            "maxContains": 1,
            "uniqueItems": true,
            "unevaluatedItems": false
        });
        for terminal in [prefixed.clone(), json!({"$ref": "#/$defs/prefixed"})] {
            let resolved = load_binding_schema(json!({
                "$schema": super::DRAFT_2020_12,
                "type": "object",
                "properties": {"values": terminal},
                "required": ["values"],
                "additionalProperties": false,
                "$defs": {"prefixed": prefixed}
            }));
            assert!(resolved.binding_shape("/values").is_ok());
        }

        let post_prefix_required = load_binding_schema(json!({
            "$schema": super::DRAFT_2020_12,
            "type": "object",
            "properties": {
                "values": {
                    "type": "array",
                    "minItems": 2,
                    "maxItems": 2,
                    "prefixItems": [{"const": "a"}],
                    "items": false
                }
            },
            "required": ["values"],
            "additionalProperties": false
        }));
        assert!(post_prefix_required.binding_shape("/values").is_err());
    }
}

#[cfg(test)]
mod shared_profile_budget_tests {
    use super::{AdmittedSchemaEntrySource, SchemaRegistry, SchemaSourceKind};
    use crate::{
        ExactDefinitionRef, RegistryLoadErrorKind, SourceByteBudget, MAX_SOURCE_DOCUMENT_BYTES,
        MAX_TOTAL_SOURCE_BYTES,
    };

    fn fill_budget(budget: &mut SourceByteBudget, mut bytes: usize) {
        while bytes > 0 {
            let chunk = bytes.min(MAX_SOURCE_DOCUMENT_BYTES);
            budget.admit(chunk).unwrap();
            bytes -= chunk;
        }
    }

    #[test]
    fn admitted_definition_and_schema_document_bytes_share_the_exact_8_mib_boundary() {
        let reference =
            ExactDefinitionRef::parse("handbook.schemas.artifacts.project-authority@1.0.0")
                .unwrap();
        let entry = crate::profile_builtins::definition(&reference).unwrap();
        let document_path =
            "definitions/schemas/handbook.schemas.artifacts.project-authority/1.0.0.schema.json";
        let document_bytes = crate::profile_builtins::schema_document(document_path).unwrap();
        let source = AdmittedSchemaEntrySource {
            declared_ref: &reference,
            source_kind: SchemaSourceKind::BuiltIn,
            bytes: entry.bytes,
        };
        let repo = tempfile::tempdir().unwrap();

        let mut exact = SourceByteBudget::default();
        fill_budget(&mut exact, MAX_TOTAL_SOURCE_BYTES - document_bytes.len());
        SchemaRegistry::load_admitted(
            repo.path(),
            std::slice::from_ref(&source),
            &["definitions/schemas".into()],
            &mut exact,
        )
        .unwrap();
        assert_eq!(exact.total_bytes(), MAX_TOTAL_SOURCE_BYTES);

        let mut over = SourceByteBudget::default();
        fill_budget(&mut over, MAX_TOTAL_SOURCE_BYTES - document_bytes.len() + 1);
        assert_eq!(
            SchemaRegistry::load_admitted(
                repo.path(),
                std::slice::from_ref(&source),
                &["definitions/schemas".into()],
                &mut over,
            )
            .unwrap_err()
            .kind(),
            RegistryLoadErrorKind::AggregateLimitExceeded
        );
    }
}

#[cfg(test)]
mod schema_source_domain_tests {
    use super::{ClosureLoader, RequestSchemaState, SchemaSourceKind, DRAFT_2020_12};
    use crate::{RegistryLoadError, RegistryLoadErrorKind, SourceByteBudget};
    use std::path::Path;

    const BUILTIN_DOCUMENT: &str =
        "definitions/schemas/handbook.schemas.artifacts.decision-record/1.0.0.schema.json";
    const REPOSITORY_ROOT: &str = ".handbook/definitions/schemas";

    fn schema_bytes() -> &'static [u8] {
        br#"{"$schema":"https://json-schema.org/draft/2020-12/schema","type":"object"}"#
    }

    fn write_schema(repo: &Path, path: &str) {
        let absolute = repo.join(path);
        std::fs::create_dir_all(absolute.parent().expect("schema parent")).unwrap();
        std::fs::write(absolute, schema_bytes()).unwrap();
    }

    fn load_one(
        repo: &Path,
        source_kind: SchemaSourceKind,
        allowed_roots: &[String],
        path: &str,
    ) -> Result<(), RegistryLoadError> {
        let mut budget = SourceByteBudget::default();
        let mut state = RequestSchemaState::default();
        ClosureLoader::new(repo, allowed_roots, &mut budget, &mut state, source_kind)
            .load_document(path)
    }

    #[test]
    fn built_in_containment_uses_only_the_fixed_package_schema_root() {
        let repo = tempfile::tempdir().unwrap();
        let request_roots = [REPOSITORY_ROOT.to_string()];
        let shadow = format!("{REPOSITORY_ROOT}/shadow.schema.json");
        write_schema(repo.path(), &shadow);

        load_one(
            repo.path(),
            SchemaSourceKind::BuiltIn,
            &request_roots,
            BUILTIN_DOCUMENT,
        )
        .expect("allowlisted built-in descendant");

        let root = load_one(
            repo.path(),
            SchemaSourceKind::BuiltIn,
            &request_roots,
            "definitions/schemas",
        )
        .expect_err("the built-in root is contained but is not an allowlisted document");
        assert_eq!(root.kind(), RegistryLoadErrorKind::LocalReferenceMissing);

        for path in [
            REPOSITORY_ROOT,
            shadow.as_str(),
            "definitions/schemas-collision/shadow.schema.json",
            "other/schemas/shadow.schema.json",
        ] {
            let error = load_one(repo.path(), SchemaSourceKind::BuiltIn, &request_roots, path)
                .expect_err(path);
            assert_eq!(
                error.kind(),
                RegistryLoadErrorKind::LocalReferenceOutsideRoot,
                "{path}"
            );
        }

        let error = load_one(
            repo.path(),
            SchemaSourceKind::BuiltIn,
            &request_roots,
            "definitions/schemas/not-allowlisted.schema.json",
        )
        .expect_err("absent built-in allowlist key");
        assert_eq!(error.kind(), RegistryLoadErrorKind::LocalReferenceMissing);
    }

    #[test]
    fn repository_containment_uses_only_request_authorized_roots() {
        let request_roots = [REPOSITORY_ROOT.to_string()];

        let root_repo = tempfile::tempdir().unwrap();
        write_schema(root_repo.path(), REPOSITORY_ROOT);
        load_one(
            root_repo.path(),
            SchemaSourceKind::Repository,
            &request_roots,
            REPOSITORY_ROOT,
        )
        .expect("repository root document");

        let repo = tempfile::tempdir().unwrap();
        let descendant = format!("{REPOSITORY_ROOT}/nested/value.schema.json");
        write_schema(repo.path(), &descendant);
        write_schema(repo.path(), BUILTIN_DOCUMENT);
        load_one(
            repo.path(),
            SchemaSourceKind::Repository,
            &request_roots,
            &descendant,
        )
        .expect("repository descendant document");

        for path in [
            BUILTIN_DOCUMENT,
            ".handbook/definitions/schemas-collision/value.schema.json",
            "outside/value.schema.json",
        ] {
            let error = load_one(
                repo.path(),
                SchemaSourceKind::Repository,
                &request_roots,
                path,
            )
            .expect_err(path);
            assert_eq!(
                error.kind(),
                RegistryLoadErrorKind::LocalReferenceOutsideRoot,
                "{path}"
            );
        }
    }

    #[test]
    fn containment_precedes_cache_reuse_and_repository_filesystem_access() {
        let repo = tempfile::tempdir().unwrap();
        write_schema(repo.path(), BUILTIN_DOCUMENT);
        let broad_roots = ["definitions/schemas".to_string()];
        let narrow_roots = [REPOSITORY_ROOT.to_string()];
        let mut budget = SourceByteBudget::default();
        let mut state = RequestSchemaState::default();

        ClosureLoader::new(
            repo.path(),
            &broad_roots,
            &mut budget,
            &mut state,
            SchemaSourceKind::Repository,
        )
        .load_document(BUILTIN_DOCUMENT)
        .expect("prime repository cache");

        let error = ClosureLoader::new(
            repo.path(),
            &narrow_roots,
            &mut budget,
            &mut state,
            SchemaSourceKind::Repository,
        )
        .load_document(BUILTIN_DOCUMENT)
        .expect_err("cached document outside the current root");
        assert_eq!(
            error.kind(),
            RegistryLoadErrorKind::LocalReferenceOutsideRoot
        );

        let missing = format!("{REPOSITORY_ROOT}/missing.schema.json");
        let error = load_one(
            repo.path(),
            SchemaSourceKind::Repository,
            &narrow_roots,
            &missing,
        )
        .expect_err("contained missing repository document");
        assert_eq!(error.kind(), RegistryLoadErrorKind::LocalReferenceMissing);
    }

    #[test]
    fn repository_unsafe_traversal_and_non_regular_sources_still_refuse() {
        let repo = tempfile::tempdir().unwrap();
        let request_roots = [REPOSITORY_ROOT.to_string()];
        std::fs::create_dir_all(repo.path().join(REPOSITORY_ROOT).join("directory")).unwrap();

        let traversal = load_one(
            repo.path(),
            SchemaSourceKind::Repository,
            &request_roots,
            ".handbook/definitions/schemas/../escape.schema.json",
        )
        .expect_err("traversal");
        assert_eq!(traversal.kind(), RegistryLoadErrorKind::InvalidSourcePath);

        let non_regular = load_one(
            repo.path(),
            SchemaSourceKind::Repository,
            &request_roots,
            ".handbook/definitions/schemas/directory",
        )
        .expect_err("non-regular source");
        assert_eq!(non_regular.kind(), RegistryLoadErrorKind::NonRegularSource);
    }

    #[test]
    fn schema_test_fixture_uses_the_expected_dialect_constant() {
        let value: serde_json::Value = serde_json::from_slice(schema_bytes()).unwrap();
        assert_eq!(value["$schema"], DRAFT_2020_12);
    }
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
