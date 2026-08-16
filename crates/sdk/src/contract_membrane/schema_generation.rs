//! Feature-gated JSON Schema descriptions and deterministic checked-in generation.

use super::*;
use schemars::generate::SchemaSettings;
use schemars::{json_schema, JsonSchema, Schema, SchemaGenerator};
use std::borrow::Cow;
use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::marker::PhantomData;
use std::path::{Path, PathBuf};

const MAX_SAFE_INTEGER: u64 = 9_007_199_254_740_991;
const MIN_SAFE_INTEGER: i64 = -9_007_199_254_740_991;
const TOKEN_PATTERN: &str = "^[a-z][a-z0-9._-]*$";
const FINGERPRINT_PATTERN: &str = "^sha256:[0-9a-f]{64}$";
const CORRELATION_PATTERN: &str = "^corr_[A-Za-z0-9_-]{16,128}$";
const TIMESTAMP_PATTERN: &str =
    "^[0-9]{4}-(0[1-9]|1[0-2])-([0-2][0-9]|3[01])T([01][0-9]|2[0-3]):[0-5][0-9]:[0-5][0-9]Z$";
const EXACT_REF_PATTERN: &str = concat!(
    "^[a-z][a-z0-9._-]{0,254}@",
    "(0|[1-9][0-9]*)\\.(0|[1-9][0-9]*)\\.(0|[1-9][0-9]*)",
    "(-(0|[1-9][0-9]*|[0-9]*[A-Za-z-][0-9A-Za-z-]*)",
    "(\\.(0|[1-9][0-9]*|[0-9]*[A-Za-z-][0-9A-Za-z-]*))*)?",
    "(\\+[0-9A-Za-z-]+(\\.[0-9A-Za-z-]+)*)?$"
);

fn closed_object(properties: Vec<(&'static str, Schema)>, required: &[&'static str]) -> Schema {
    let properties = properties.into_iter().collect::<BTreeMap<_, _>>();
    json_schema!({
        "type": "object",
        "properties": properties,
        "required": required,
        "additionalProperties": false
    })
}

fn nullable<T: JsonSchema>(generator: &mut SchemaGenerator) -> Schema {
    let value = generator.subschema_for::<T>();
    json_schema!({"anyOf": [value, {"type": "null"}]})
}

fn string_enum(values: &[&str]) -> Schema {
    json_schema!({"type": "string", "enum": values})
}

macro_rules! impl_schema {
    ($type:ty, $name:literal, |$generator:ident| $schema:expr) => {
        impl JsonSchema for $type {
            fn schema_name() -> Cow<'static, str> {
                $name.into()
            }

            fn json_schema($generator: &mut SchemaGenerator) -> Schema {
                $schema
            }
        }
    };
}

impl<const N: usize> JsonSchema for BoundedString<N> {
    fn schema_name() -> Cow<'static, str> {
        format!("BoundedString_{N}").into()
    }

    fn schema_id() -> Cow<'static, str> {
        format!("handbook_sdk::contract_membrane::BoundedString<{N}>").into()
    }

    fn json_schema(_generator: &mut SchemaGenerator) -> Schema {
        json_schema!({"type": "string", "minLength": 1, "maxLength": N})
    }
}

impl<T: JsonSchema, const N: usize> JsonSchema for BoundedVec<T, N> {
    fn schema_name() -> Cow<'static, str> {
        format!("BoundedVec_{}_{}", T::schema_name(), N).into()
    }

    fn schema_id() -> Cow<'static, str> {
        format!(
            "handbook_sdk::contract_membrane::BoundedVec<{},{}>",
            T::schema_id(),
            N
        )
        .into()
    }

    fn json_schema(generator: &mut SchemaGenerator) -> Schema {
        let items = generator.subschema_for::<T>();
        json_schema!({"type": "array", "items": items, "maxItems": N})
    }
}

impl_schema!(SafeU64, "SafeU64", |_generator| json_schema!({
    "type": "integer",
    "minimum": 0,
    "maximum": MAX_SAFE_INTEGER
}));

impl_schema!(SafeI64, "SafeI64", |_generator| json_schema!({
    "type": "integer",
    "minimum": MIN_SAFE_INTEGER,
    "maximum": MAX_SAFE_INTEGER
}));

impl_schema!(UtcTimestamp, "UtcTimestamp", |_generator| json_schema!({
    "type": "string",
    "minLength": 20,
    "maxLength": 20,
    "pattern": TIMESTAMP_PATTERN
}));

impl_schema!(ExactRef, "ExactRef", |_generator| json_schema!({
    "type": "string",
    "minLength": 7,
    "maxLength": 511,
    "pattern": EXACT_REF_PATTERN
}));

impl_schema!(Fingerprint, "Fingerprint", |_generator| json_schema!({
    "type": "string",
    "minLength": 71,
    "maxLength": 71,
    "pattern": FINGERPRINT_PATTERN
}));

impl_schema!(ExactBinding, "ExactBinding", |generator| closed_object(
    vec![
        ("ref", generator.subschema_for::<ExactRef>()),
        ("fingerprint", generator.subschema_for::<Fingerprint>()),
    ],
    &["ref", "fingerprint"],
));

impl_schema!(ContractValue, "ContractValue", |generator| {
    let recursive = generator.subschema_for::<ContractValue>();
    json_schema!({
        "oneOf": [
            {"type": "null"},
            {"type": "boolean"},
            {"type": "string", "minLength": 1, "maxLength": 65_536},
            {"type": "integer", "minimum": MIN_SAFE_INTEGER, "maximum": MAX_SAFE_INTEGER},
            {"type": "array", "items": recursive, "maxItems": 128},
            {
                "type": "object",
                "additionalProperties": recursive,
                "maxProperties": 128,
                "propertyNames": {"type": "string", "minLength": 1, "maxLength": 255}
            }
        ]
    })
});

macro_rules! impl_string_enum {
    ($type:ty, $name:literal, [$($value:literal),+ $(,)?]) => {
        impl_schema!($type, $name, |_generator| string_enum(&[$($value),+]));
    };
}

impl_string_enum!(DigestAlgorithm, "DigestAlgorithm", ["sha256"]);
impl_string_enum!(
    Sensitivity,
    "Sensitivity",
    ["public", "internal", "restricted"]
);
impl_string_enum!(
    ProblemCategory,
    "ProblemCategory",
    [
        "prerequisite",
        "schema",
        "compatibility",
        "capability",
        "resolution",
        "authority",
        "precondition",
        "safety",
        "idempotency",
        "implementation",
        "adapter",
    ]
);
impl_string_enum!(
    DiagnosticSeverity,
    "DiagnosticSeverity",
    ["information", "warning"]
);
impl_string_enum!(
    RetryDirective,
    "RetryDirective",
    [
        "after_recheck",
        "after_request_change",
        "after_authority_change",
        "transient",
        "never",
    ]
);
impl_string_enum!(
    NextActionKind,
    "NextActionKind",
    [
        "recheck",
        "change_request",
        "change_authority",
        "retry_transient",
        "contact_operator",
    ]
);
impl_string_enum!(
    OmissionReason,
    "OmissionReason",
    [
        "out_of_resolution",
        "redacted",
        "unavailable",
        "unsupported"
    ]
);
impl_string_enum!(
    ProofEffect,
    "ProofEffect",
    ["none", "partial", "blocked", "refused"]
);
impl_string_enum!(
    IdempotencyStage,
    "IdempotencyStage",
    [
        "capability_validation",
        "request_validation",
        "resolution_validation",
        "authority_validation",
        "precondition_validation",
        "safety_validation",
        "idempotency_validation",
        "execution_start",
    ]
);

impl_schema!(DraftVersion, "DraftVersion", |_generator| json_schema!({
    "type": "string",
    "const": "2020-12"
}));
impl_schema!(
    SchemaMediaType,
    "SchemaMediaType",
    |_generator| json_schema!({
        "type": "string",
        "const": "application/schema+json"
    })
);

struct Token<const N: usize>(PhantomData<()>);

impl<const N: usize> JsonSchema for Token<N> {
    fn schema_name() -> Cow<'static, str> {
        format!("Token_{N}").into()
    }

    fn schema_id() -> Cow<'static, str> {
        format!("handbook_sdk::contract_membrane::schema_generation::Token<{N}>").into()
    }

    fn json_schema(_generator: &mut SchemaGenerator) -> Schema {
        json_schema!({
            "type": "string",
            "minLength": 1,
            "maxLength": N,
            "pattern": TOKEN_PATTERN
        })
    }
}

struct MediaType;

impl_schema!(MediaType, "MediaType", |_generator| json_schema!({
    "type": "string",
    "minLength": 3,
    "maxLength": 255,
    "pattern": "^[a-z0-9][a-z0-9!#$&^_.+\\-]*/[a-z0-9][a-z0-9!#$&^_.+\\-]*$"
}));

struct PortablePath;

impl_schema!(PortablePath, "PortablePath", |_generator| json_schema!({
    "type": "string",
    "minLength": 1,
    "maxLength": 4096,
    "pattern": "^[A-Za-z0-9._-]+(/[A-Za-z0-9._-]+)*$"
}));

impl_schema!(ArtifactLocator, "ArtifactLocator", |generator| {
    let repo_relative = closed_object(
        vec![
            (
                "kind",
                json_schema!({"type": "string", "const": "repo_relative"}),
            ),
            (
                "repository_identity_fingerprint",
                generator.subschema_for::<Fingerprint>(),
            ),
            ("path", generator.subschema_for::<PortablePath>()),
            (
                "no_follow",
                json_schema!({"type": "boolean", "const": true}),
            ),
        ],
        &[
            "kind",
            "repository_identity_fingerprint",
            "path",
            "no_follow",
        ],
    );
    let content_addressed = closed_object(
        vec![
            (
                "kind",
                json_schema!({"type": "string", "const": "content_addressed"}),
            ),
            ("store", generator.subschema_for::<ExactBinding>()),
            ("algorithm", generator.subschema_for::<DigestAlgorithm>()),
            ("digest", generator.subschema_for::<Fingerprint>()),
        ],
        &["kind", "store", "algorithm", "digest"],
    );
    json_schema!({"oneOf": [repo_relative, content_addressed]})
});

impl_schema!(ArtifactRef, "ArtifactRef", |generator| closed_object(
    vec![
        ("artifact", generator.subschema_for::<ExactBinding>()),
        ("media_type", generator.subschema_for::<MediaType>()),
        ("byte_length", generator.subschema_for::<SafeU64>()),
        ("sensitivity", generator.subschema_for::<Sensitivity>()),
        ("locator", generator.subschema_for::<ArtifactLocator>()),
    ],
    &[
        "artifact",
        "media_type",
        "byte_length",
        "sensitivity",
        "locator",
    ],
));

impl_schema!(SourceBinding, "SourceBinding", |generator| closed_object(
    vec![
        ("source", generator.subschema_for::<ExactBinding>()),
        (
            "captured_revision",
            generator.subschema_for::<ExactBinding>(),
        ),
        ("adapter", generator.subschema_for::<ExactBinding>()),
    ],
    &["source", "captured_revision", "adapter"],
));

impl_schema!(Omission, "Omission", |generator| closed_object(
    vec![
        ("item", generator.subschema_for::<ExactBinding>()),
        ("reason", generator.subschema_for::<OmissionReason>()),
        ("proof_effect", generator.subschema_for::<ProofEffect>()),
        ("source", nullable::<SourceBinding>(generator)),
        ("ordinal", generator.subschema_for::<SafeU64>()),
    ],
    &["item", "reason", "proof_effect", "source", "ordinal"],
));

impl_schema!(SchemaManifestEntry, "SchemaManifestEntry", |generator| {
    closed_object(
        vec![
            ("schema", generator.subschema_for::<ExactBinding>()),
            ("draft", generator.subschema_for::<DraftVersion>()),
            ("byte_length", generator.subschema_for::<SafeU64>()),
            ("media_type", generator.subschema_for::<SchemaMediaType>()),
        ],
        &["schema", "draft", "byte_length", "media_type"],
    )
});

impl_schema!(
    CorrelationRef,
    "CorrelationRef",
    |_generator| json_schema!({
        "type": "string",
        "minLength": 21,
        "maxLength": 133,
        "pattern": CORRELATION_PATTERN
    })
);

impl_schema!(ProblemBinding, "ProblemBinding", |generator| closed_object(
    vec![
        ("problem_id", generator.subschema_for::<Token<255>>()),
        (
            "problem_fingerprint",
            generator.subschema_for::<Fingerprint>(),
        ),
    ],
    &["problem_id", "problem_fingerprint"],
));

impl_schema!(RecheckCondition, "RecheckCondition", |generator| {
    closed_object(
        vec![
            (
                "condition_schema",
                generator.subschema_for::<ExactBinding>(),
            ),
            ("condition", generator.subschema_for::<ContractValue>()),
            (
                "condition_fingerprint",
                generator.subschema_for::<Fingerprint>(),
            ),
        ],
        &["condition_schema", "condition", "condition_fingerprint"],
    )
});

impl_schema!(Diagnostic, "Diagnostic", |generator| closed_object(
    vec![
        ("code", generator.subschema_for::<Token<255>>()),
        ("severity", generator.subschema_for::<DiagnosticSeverity>(),),
        ("subject", generator.subschema_for::<ExactBinding>()),
        ("details_schema", generator.subschema_for::<ExactBinding>(),),
        ("details", generator.subschema_for::<ContractValue>()),
        (
            "details_fingerprint",
            generator.subschema_for::<Fingerprint>(),
        ),
        (
            "display_message",
            generator.subschema_for::<BoundedString<1024>>(),
        ),
    ],
    &[
        "code",
        "severity",
        "subject",
        "details_schema",
        "details",
        "details_fingerprint",
    ],
));

impl_schema!(NextAction, "NextAction", |generator| closed_object(
    vec![
        ("action_id", generator.subschema_for::<Token<255>>()),
        ("kind", generator.subschema_for::<NextActionKind>()),
        ("subject", generator.subschema_for::<ExactBinding>()),
        (
            "parameter_schema",
            generator.subschema_for::<ExactBinding>(),
        ),
        ("parameters", generator.subschema_for::<ContractValue>()),
        (
            "display_label",
            generator.subschema_for::<BoundedString<1024>>(),
        ),
    ],
    &[
        "action_id",
        "kind",
        "subject",
        "parameter_schema",
        "parameters",
    ],
));

impl_schema!(WriteReceipt, "WriteReceipt", |generator| closed_object(
    vec![
        ("record_kind", generator.subschema_for::<Token<255>>()),
        ("record", generator.subschema_for::<ExactBinding>()),
        ("authority_class", generator.subschema_for::<Token<255>>(),),
        ("condition", generator.subschema_for::<ContractValue>()),
        ("atomic_group", generator.subschema_for::<Token<255>>()),
        (
            "receipt_fingerprint",
            generator.subschema_for::<Fingerprint>(),
        ),
    ],
    &[
        "record_kind",
        "record",
        "authority_class",
        "condition",
        "atomic_group",
        "receipt_fingerprint",
    ],
));

impl_schema!(CatalogRoot, "CatalogRoot", |generator| closed_object(
    vec![
        ("catalog", generator.subschema_for::<ExactBinding>()),
        ("entry_schema", generator.subschema_for::<ExactBinding>(),),
        ("total_entries", generator.subschema_for::<SafeU64>()),
        ("canonical_sort", generator.subschema_for::<ExactBinding>(),),
        (
            "retention_until_utc",
            generator.subschema_for::<UtcTimestamp>(),
        ),
    ],
    &[
        "catalog",
        "entry_schema",
        "total_entries",
        "canonical_sort",
        "retention_until_utc",
    ],
));

impl_schema!(CatalogCursor, "CatalogCursor", |generator| closed_object(
    vec![
        ("catalog", generator.subschema_for::<ExactBinding>()),
        ("next_sort_key", generator.subschema_for::<ContractValue>(),),
        (
            "cursor_fingerprint",
            generator.subschema_for::<Fingerprint>(),
        ),
    ],
    &["catalog", "next_sort_key", "cursor_fingerprint"],
));

impl_schema!(CatalogPage, "CatalogPage", |generator| closed_object(
    vec![
        ("root", generator.subschema_for::<CatalogRoot>()),
        (
            "entries",
            generator.subschema_for::<BoundedVec<ContractValue, 128>>(),
        ),
        ("next_cursor", nullable::<CatalogCursor>(generator)),
        ("page_fingerprint", generator.subschema_for::<Fingerprint>(),),
    ],
    &["root", "entries", "next_cursor", "page_fingerprint"],
));

impl_schema!(IdempotencyState, "IdempotencyState", |generator| {
    let not_applicable = closed_object(
        vec![(
            "state",
            json_schema!({"type": "string", "const": "not_applicable"}),
        )],
        &["state"],
    );
    let not_established = closed_object(
        vec![
            (
                "state",
                json_schema!({"type": "string", "const": "not_established"}),
            ),
            ("stage", generator.subschema_for::<IdempotencyStage>()),
            (
                "terminal_problem",
                generator.subschema_for::<ProblemBinding>(),
            ),
        ],
        &["state", "stage", "terminal_problem"],
    );
    let established = closed_object(
        vec![
            (
                "state",
                json_schema!({"type": "string", "const": "established"}),
            ),
            (
                "idempotency_key_fingerprint",
                generator.subschema_for::<Fingerprint>(),
            ),
            (
                "request_fingerprint",
                generator.subschema_for::<Fingerprint>(),
            ),
            ("replayed", json_schema!({"type": "boolean"})),
            (
                "original_result_fingerprint",
                nullable::<Fingerprint>(generator),
            ),
            (
                "result_retention_until_utc",
                generator.subschema_for::<UtcTimestamp>(),
            ),
        ],
        &[
            "state",
            "idempotency_key_fingerprint",
            "request_fingerprint",
            "replayed",
            "original_result_fingerprint",
            "result_retention_until_utc",
        ],
    );
    json_schema!({"oneOf": [not_applicable, not_established, established]})
});

impl_schema!(Problem, "Problem", |generator| closed_object(
    vec![
        ("problem_id", generator.subschema_for::<Token<255>>()),
        ("code", generator.subschema_for::<Token<255>>()),
        ("category", generator.subschema_for::<ProblemCategory>(),),
        ("subject", generator.subschema_for::<ExactBinding>()),
        ("rule", nullable::<ExactBinding>(generator)),
        ("details_schema", generator.subschema_for::<ExactBinding>(),),
        ("details", generator.subschema_for::<ContractValue>()),
        (
            "details_fingerprint",
            generator.subschema_for::<Fingerprint>(),
        ),
        (
            "evidence",
            generator.subschema_for::<BoundedVec<ArtifactRef, 128>>(),
        ),
        ("retry", generator.subschema_for::<RetryDirective>()),
        ("recheck", nullable::<RecheckCondition>(generator)),
        ("correlation_ref", nullable::<CorrelationRef>(generator),),
        (
            "problem_fingerprint",
            generator.subschema_for::<Fingerprint>(),
        ),
    ],
    &[
        "problem_id",
        "code",
        "category",
        "subject",
        "rule",
        "details_schema",
        "details",
        "details_fingerprint",
        "evidence",
        "retry",
        "recheck",
        "correlation_ref",
        "problem_fingerprint",
    ],
));

const DIALECT: &str = "https://json-schema.org/draft/2020-12/schema";

struct SchemaDescriptor {
    exact_ref: &'static str,
    relative_path: &'static str,
    generate: fn() -> Schema,
}

const INVENTORY: [SchemaDescriptor; 15] = [
    descriptor::<ArtifactRef>(
        "handbook.artifact-ref@1.0.0",
        "handbook.artifact-ref/1.0.0.schema.json",
    ),
    descriptor::<CatalogCursor>(
        "handbook.catalog-cursor@1.0.0",
        "handbook.catalog-cursor/1.0.0.schema.json",
    ),
    descriptor::<CatalogPage>(
        "handbook.catalog-page@1.0.0",
        "handbook.catalog-page/1.0.0.schema.json",
    ),
    descriptor::<CatalogRoot>(
        "handbook.catalog-root@1.0.0",
        "handbook.catalog-root/1.0.0.schema.json",
    ),
    descriptor::<CorrelationRef>(
        "handbook.correlation-ref@1.0.0",
        "handbook.correlation-ref/1.0.0.schema.json",
    ),
    descriptor::<Diagnostic>(
        "handbook.diagnostic@1.0.0",
        "handbook.diagnostic/1.0.0.schema.json",
    ),
    descriptor::<IdempotencyState>(
        "handbook.idempotency-state@1.0.0",
        "handbook.idempotency-state/1.0.0.schema.json",
    ),
    descriptor::<NextAction>(
        "handbook.next-action@1.0.0",
        "handbook.next-action/1.0.0.schema.json",
    ),
    descriptor::<Omission>(
        "handbook.omission@1.0.0",
        "handbook.omission/1.0.0.schema.json",
    ),
    descriptor::<ProblemBinding>(
        "handbook.problem-binding@1.0.0",
        "handbook.problem-binding/1.0.0.schema.json",
    ),
    descriptor::<Problem>(
        "handbook.problem@1.0.0",
        "handbook.problem/1.0.0.schema.json",
    ),
    descriptor::<RecheckCondition>(
        "handbook.recheck-condition@1.0.0",
        "handbook.recheck-condition/1.0.0.schema.json",
    ),
    descriptor::<SchemaManifestEntry>(
        "handbook.schema-manifest-entry@1.0.0",
        "handbook.schema-manifest-entry/1.0.0.schema.json",
    ),
    descriptor::<SourceBinding>(
        "handbook.source-binding@1.0.0",
        "handbook.source-binding/1.0.0.schema.json",
    ),
    descriptor::<WriteReceipt>(
        "handbook.write-receipt@1.0.0",
        "handbook.write-receipt/1.0.0.schema.json",
    ),
];

const fn descriptor<T: JsonSchema>(
    exact_ref: &'static str,
    relative_path: &'static str,
) -> SchemaDescriptor {
    SchemaDescriptor {
        exact_ref,
        relative_path,
        generate: generate_root::<T>,
    }
}

fn generate_root<T: JsonSchema>() -> Schema {
    SchemaSettings::draft2020_12()
        .for_deserialize()
        .into_generator()
        .into_root_schema_for::<T>()
}

struct GeneratedTree {
    schemas: BTreeMap<&'static str, Vec<u8>>,
    manifest: Vec<u8>,
}

/// The only supported generator modes.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum GenerationMode {
    Check,
    Write,
}

/// Generates a complete temporary tree, then checks or replaces the tracked output.
pub fn run(mode: GenerationMode) -> Result<(), String> {
    let generated = generate_tree()?;
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let workspace_root = manifest_dir
        .parent()
        .and_then(Path::parent)
        .ok_or_else(|| "SDK manifest directory has no workspace root".to_owned())?;
    let temporary_root = workspace_root
        .join("target")
        .join("contract-schema-generation");

    if temporary_root.exists() {
        fs::remove_dir_all(&temporary_root).map_err(io_error("remove temporary tree"))?;
    }
    write_tree(&temporary_root, &generated)?;

    let result = match mode {
        GenerationMode::Check => compare_tree(&temporary_root, &manifest_dir.join("schemas")),
        GenerationMode::Write => {
            replace_tracked_tree(&temporary_root, &manifest_dir.join("schemas"))?;
            compare_tree(&temporary_root, &manifest_dir.join("schemas"))
        }
    };
    let cleanup = fs::remove_dir_all(&temporary_root).map_err(io_error("remove temporary tree"));
    result.and(cleanup)
}

fn generate_tree() -> Result<GeneratedTree, String> {
    if INVENTORY.len() != 15 {
        return Err("schema inventory must contain exactly 15 entries".to_owned());
    }

    let mut previous_ref = None;
    let mut paths = BTreeSet::new();
    let mut schemas = BTreeMap::new();
    let mut manifest = Vec::with_capacity(INVENTORY.len());
    let limits = ContractLimits::packet0();

    for descriptor in &INVENTORY {
        if previous_ref.is_some_and(|previous| previous >= descriptor.exact_ref) {
            return Err("schema inventory refs must be unique and byte-sorted".to_owned());
        }
        previous_ref = Some(descriptor.exact_ref);
        if !paths.insert(descriptor.relative_path) {
            return Err("schema inventory paths must be unique".to_owned());
        }

        let exact_ref = ExactRef::parse(descriptor.exact_ref)
            .map_err(|_| "schema inventory contains an invalid exact ref".to_owned())?;
        let identifier = format!(
            "urn:handbook:schema:{}:{}",
            exact_ref.identity(),
            exact_ref.version()
        );
        let mut schema = (descriptor.generate)();
        schema.insert("$id".to_owned(), identifier.clone().into());
        validate_schema(descriptor, &schema, &identifier)?;

        let mut bytes = canonical_json_bytes(&schema, &limits)
            .map_err(|_| "schema did not fit the Packet-0 canonical domain".to_owned())?;
        bytes.push(b'\n');
        validate_schema_bytes(&schema, &bytes, &limits)?;

        let byte_length = SafeU64::new(bytes.len() as u64)
            .map_err(|_| "schema byte length exceeded the safe integer domain".to_owned())?;
        let entry = SchemaManifestEntry::new(
            ExactBinding::new(exact_ref, sha256_fingerprint(&bytes)),
            DraftVersion::Draft202012,
            byte_length,
            SchemaMediaType::ApplicationSchemaJson,
        )
        .map_err(|_| "schema manifest entry construction failed".to_owned())?;
        manifest.push(entry);
        schemas.insert(descriptor.relative_path, bytes);
    }

    let mut manifest = canonical_json_bytes(&manifest, &limits)
        .map_err(|_| "schema manifest did not fit the Packet-0 canonical domain".to_owned())?;
    manifest.push(b'\n');

    Ok(GeneratedTree { schemas, manifest })
}

fn validate_schema(
    descriptor: &SchemaDescriptor,
    schema: &Schema,
    identifier: &str,
) -> Result<(), String> {
    if schema.get("$schema").and_then(serde_json::Value::as_str) != Some(DIALECT) {
        return Err(format!(
            "{} has the wrong schema dialect",
            descriptor.relative_path
        ));
    }
    if schema.get("$id").and_then(serde_json::Value::as_str) != Some(identifier) {
        return Err(format!(
            "{} has the wrong schema id",
            descriptor.relative_path
        ));
    }

    let value = schema.as_value();
    validate_local_references(value, value, descriptor.relative_path)?;
    validate_closed_objects(value, descriptor.relative_path)?;
    validate_no_raw_key_names(value, descriptor.relative_path)
}

fn validate_local_references(
    root: &serde_json::Value,
    current: &serde_json::Value,
    relative_path: &str,
) -> Result<(), String> {
    match current {
        serde_json::Value::Array(values) => {
            for value in values {
                validate_local_references(root, value, relative_path)?;
            }
        }
        serde_json::Value::Object(members) => {
            if let Some(reference) = members.get("$ref") {
                let reference = reference
                    .as_str()
                    .ok_or_else(|| format!("{relative_path} has a non-string ref"))?;
                let pointer = reference
                    .strip_prefix('#')
                    .filter(|pointer| pointer.starts_with("/$defs/"))
                    .ok_or_else(|| format!("{relative_path} has a non-local ref"))?;
                if root.pointer(pointer).is_none() {
                    return Err(format!("{relative_path} has an unresolved local ref"));
                }
            }
            for value in members.values() {
                validate_local_references(root, value, relative_path)?;
            }
        }
        _ => {}
    }
    Ok(())
}

fn validate_closed_objects(value: &serde_json::Value, relative_path: &str) -> Result<(), String> {
    match value {
        serde_json::Value::Array(values) => {
            for value in values {
                validate_closed_objects(value, relative_path)?;
            }
        }
        serde_json::Value::Object(members) => {
            if members.get("type").and_then(serde_json::Value::as_str) == Some("object")
                && members.contains_key("properties")
                && members.get("additionalProperties") != Some(&serde_json::Value::Bool(false))
            {
                return Err(format!(
                    "{relative_path} contains an open structural object"
                ));
            }
            for value in members.values() {
                validate_closed_objects(value, relative_path)?;
            }
        }
        _ => {}
    }
    Ok(())
}

fn validate_no_raw_key_names(value: &serde_json::Value, relative_path: &str) -> Result<(), String> {
    match value {
        serde_json::Value::Array(values) => {
            for value in values {
                validate_no_raw_key_names(value, relative_path)?;
            }
        }
        serde_json::Value::Object(members) => {
            for (key, value) in members {
                if matches!(key.as_str(), "idempotency_key" | "raw_idempotency_key") {
                    return Err(format!("{relative_path} contains raw key material"));
                }
                validate_no_raw_key_names(value, relative_path)?;
            }
        }
        serde_json::Value::String(value)
            if matches!(value.as_str(), "idempotency_key" | "raw_idempotency_key") =>
        {
            return Err(format!("{relative_path} contains raw key material"));
        }
        _ => {}
    }
    Ok(())
}

fn validate_schema_bytes(
    schema: &Schema,
    bytes: &[u8],
    limits: &ContractLimits,
) -> Result<(), String> {
    if !bytes.ends_with(b"\n") || bytes.len() < 2 || bytes[bytes.len() - 2] == b'\n' {
        return Err("generated schema must end in exactly one LF".to_owned());
    }
    let parsed = parse_canonical_json::<serde_json::Value>(&bytes[..bytes.len() - 1], limits)
        .map_err(|_| "generated schema bytes are not canonical".to_owned())?;
    if &parsed != schema.as_value() {
        return Err("generated schema bytes changed their JSON value".to_owned());
    }
    Ok(())
}

fn write_tree(root: &Path, generated: &GeneratedTree) -> Result<(), String> {
    for (relative_path, bytes) in &generated.schemas {
        let path = root.join("2020-12").join(relative_path);
        let parent = path
            .parent()
            .ok_or_else(|| "generated schema path has no parent".to_owned())?;
        fs::create_dir_all(parent).map_err(io_error("create temporary schema directory"))?;
        fs::write(path, bytes).map_err(io_error("write temporary schema"))?;
    }
    fs::create_dir_all(root).map_err(io_error("create temporary schema root"))?;
    fs::write(root.join("manifest.json"), &generated.manifest)
        .map_err(io_error("write temporary manifest"))
}

fn compare_tree(temporary_root: &Path, tracked_root: &Path) -> Result<(), String> {
    let temporary = read_relative_files(&temporary_root.join("2020-12"))?;
    let tracked = read_relative_files(&tracked_root.join("2020-12"))?;
    if temporary != tracked {
        return Err("tracked Draft 2020-12 schema tree is not byte-identical".to_owned());
    }
    let temporary_manifest = fs::read(temporary_root.join("manifest.json"))
        .map_err(io_error("read temporary manifest"))?;
    let tracked_manifest =
        fs::read(tracked_root.join("manifest.json")).map_err(io_error("read tracked manifest"))?;
    if temporary_manifest != tracked_manifest {
        return Err("tracked schema manifest is not byte-identical".to_owned());
    }
    Ok(())
}

fn read_relative_files(root: &Path) -> Result<BTreeMap<PathBuf, Vec<u8>>, String> {
    fn visit(
        root: &Path,
        current: &Path,
        files: &mut BTreeMap<PathBuf, Vec<u8>>,
    ) -> Result<(), String> {
        for entry in fs::read_dir(current).map_err(io_error("read schema directory"))? {
            let entry = entry.map_err(io_error("read schema directory entry"))?;
            let path = entry.path();
            if path.is_dir() {
                visit(root, &path, files)?;
            } else {
                let relative = path
                    .strip_prefix(root)
                    .map_err(|_| "schema path escaped its root".to_owned())?
                    .to_path_buf();
                let bytes = fs::read(&path).map_err(io_error("read schema file"))?;
                files.insert(relative, bytes);
            }
        }
        Ok(())
    }

    let mut files = BTreeMap::new();
    visit(root, root, &mut files)?;
    Ok(files)
}

fn replace_tracked_tree(temporary_root: &Path, tracked_root: &Path) -> Result<(), String> {
    let tracked_schemas = tracked_root.join("2020-12");
    if tracked_schemas.exists() {
        fs::remove_dir_all(&tracked_schemas).map_err(io_error("remove tracked schema tree"))?;
    }
    fs::create_dir_all(&tracked_schemas).map_err(io_error("create tracked schema tree"))?;
    for (relative, bytes) in read_relative_files(&temporary_root.join("2020-12"))? {
        let path = tracked_schemas.join(relative);
        let parent = path
            .parent()
            .ok_or_else(|| "tracked schema path has no parent".to_owned())?;
        fs::create_dir_all(parent).map_err(io_error("create tracked schema directory"))?;
        fs::write(path, bytes).map_err(io_error("write tracked schema"))?;
    }
    fs::create_dir_all(tracked_root).map_err(io_error("create tracked schema root"))?;
    fs::copy(
        temporary_root.join("manifest.json"),
        tracked_root.join("manifest.json"),
    )
    .map_err(io_error("write tracked manifest"))?;
    Ok(())
}

fn io_error(context: &'static str) -> impl FnOnce(std::io::Error) -> String {
    move |_| context.to_owned()
}
