//! Feature-gated JSON Schema descriptions and deterministic checked-in generation.

use super::posture::REQUEST_SCHEMA_REF;
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
const SEMVER_PATTERN: &str = concat!(
    "^(0|[1-9][0-9]*)\\.(0|[1-9][0-9]*)\\.(0|[1-9][0-9]*)",
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

impl_schema!(SemVer, "SemVer", |_generator| json_schema!({
    "type": "string",
    "minLength": 5,
    "maxLength": 255,
    "pattern": SEMVER_PATTERN
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

impl_schema!(RequestId, "RequestId", |_generator| json_schema!({
    "type": "string",
    "minLength": 1,
    "maxLength": 128,
    "pattern": "^[A-Za-z0-9][A-Za-z0-9._:-]{0,127}$"
}));

impl_schema!(JsonPointer, "JsonPointer", |_generator| json_schema!({
    "type": "string",
    "minLength": 2,
    "maxLength": 255,
    "pattern": "^/([^~/]|~[01])+(/([^~/]|~[01])+)*$"
}));

impl_schema!(
    OperationOwnerBinding,
    "OperationOwnerBinding",
    |generator| closed_object(
        vec![
            (
                "owner_crate",
                generator.subschema_for::<BoundedString<255>>()
            ),
            ("owner_version", generator.subschema_for::<SemVer>()),
            ("source", generator.subschema_for::<ExactBinding>()),
        ],
        &["owner_crate", "owner_version", "source"],
    )
);

impl_schema!(
    OperationOutcomeSchemas,
    "OperationOutcomeSchemas",
    |generator| closed_object(
        vec![
            ("blocker", generator.subschema_for::<ExactBinding>()),
            ("refusal", generator.subschema_for::<ExactBinding>()),
            ("error", generator.subschema_for::<ExactBinding>()),
        ],
        &["blocker", "refusal", "error"],
    )
);

impl_schema!(DeclaredWrite, "DeclaredWrite", |generator| closed_object(
    vec![
        (
            "record_kind",
            generator.subschema_for::<BoundedString<255>>()
        ),
        (
            "authority_class",
            generator.subschema_for::<BoundedString<255>>()
        ),
        (
            "atomic_group",
            generator.subschema_for::<BoundedString<255>>()
        ),
        (
            "condition_schema",
            generator.subschema_for::<ExactBinding>()
        ),
    ],
    &[
        "record_kind",
        "authority_class",
        "atomic_group",
        "condition_schema"
    ],
));

impl_schema!(OperationIdempotency, "OperationIdempotency", |generator| {
    let pointer = generator.subschema_for::<JsonPointer>();
    let basis = generator.subschema_for::<BoundedVec<JsonPointer, 128>>();
    json_schema!({
        "oneOf": [
            closed_object(vec![("kind", json_schema!({"const": "safe"}))], &["kind"]),
            closed_object(
                vec![
                    ("kind", json_schema!({"const": "idempotency_key_required"})),
                    ("key_pointer", pointer.clone()),
                ],
                &["kind", "key_pointer"],
            ),
            closed_object(
                vec![
                    ("kind", json_schema!({"const": "compare_and_write_required"})),
                    ("key_pointer", pointer),
                    ("expected_basis", basis),
                ],
                &["kind", "key_pointer", "expected_basis"],
            ),
        ]
    })
});

impl_schema!(OperationDefinition, "OperationDefinition", |generator| {
    closed_object(
        vec![
            (
                "definition_schema",
                generator.subschema_for::<ExactBinding>(),
            ),
            ("operation", generator.subschema_for::<ExactRef>()),
            ("owner", generator.subschema_for::<OperationOwnerBinding>()),
            ("request_schema", generator.subschema_for::<ExactBinding>()),
            ("result_schema", generator.subschema_for::<ExactBinding>()),
            (
                "outcome_schemas",
                generator.subschema_for::<OperationOutcomeSchemas>(),
            ),
            (
                "required_capabilities",
                generator.subschema_for::<BoundedVec<ExactBinding, 128>>(),
            ),
            ("class", generator.subschema_for::<OperationClass>()),
            (
                "idempotency",
                generator.subschema_for::<OperationIdempotency>(),
            ),
            (
                "write_set",
                generator.subschema_for::<BoundedVec<DeclaredWrite, 128>>(),
            ),
            (
                "declared_transports",
                generator.subschema_for::<BoundedVec<OperationTransport, 3>>(),
            ),
            ("deprecation", nullable::<ExactBinding>(generator)),
            (
                "definition_fingerprint",
                generator.subschema_for::<Fingerprint>(),
            ),
        ],
        &[
            "definition_schema",
            "operation",
            "owner",
            "request_schema",
            "result_schema",
            "outcome_schemas",
            "required_capabilities",
            "class",
            "idempotency",
            "write_set",
            "declared_transports",
            "deprecation",
            "definition_fingerprint",
        ],
    )
});

impl_schema!(OperationRequestV1, "OperationRequestV1", |generator| {
    closed_object(
        vec![
            ("schema", generator.subschema_for::<ExactBinding>()),
            ("request_id", nullable::<RequestId>(generator)),
            ("operation", generator.subschema_for::<ExactBinding>()),
            ("body_schema", generator.subschema_for::<ExactBinding>()),
            ("body", generator.subschema_for::<ContractValue>()),
            (
                "request_fingerprint",
                generator.subschema_for::<Fingerprint>(),
            ),
        ],
        &[
            "schema",
            "request_id",
            "operation",
            "body_schema",
            "body",
            "request_fingerprint",
        ],
    )
});

impl_schema!(OperationResponseV1, "OperationResponseV1", |generator| {
    closed_object(
        vec![
            ("schema", generator.subschema_for::<ExactBinding>()),
            ("request_id", nullable::<RequestId>(generator)),
            (
                "request_fingerprint",
                generator.subschema_for::<Fingerprint>(),
            ),
            ("operation", generator.subschema_for::<ExactBinding>()),
            ("result_schema", generator.subschema_for::<ExactBinding>()),
            ("status", generator.subschema_for::<TerminalStatus>()),
            ("data", nullable::<ContractValue>(generator)),
            (
                "blockers",
                generator.subschema_for::<BoundedVec<Problem, 128>>(),
            ),
            (
                "refusals",
                generator.subschema_for::<BoundedVec<Problem, 128>>(),
            ),
            (
                "errors",
                generator.subschema_for::<BoundedVec<Problem, 128>>(),
            ),
            ("idempotency", generator.subschema_for::<IdempotencyState>()),
            (
                "receipts",
                generator.subschema_for::<BoundedVec<WriteReceipt, 128>>(),
            ),
            (
                "diagnostics",
                generator.subschema_for::<BoundedVec<Diagnostic, 128>>(),
            ),
            (
                "next_actions",
                generator.subschema_for::<BoundedVec<NextAction, 128>>(),
            ),
            (
                "result_fingerprint",
                generator.subschema_for::<Fingerprint>(),
            ),
            (
                "response_fingerprint",
                generator.subschema_for::<Fingerprint>(),
            ),
        ],
        &[
            "schema",
            "request_id",
            "request_fingerprint",
            "operation",
            "result_schema",
            "status",
            "data",
            "blockers",
            "refusals",
            "errors",
            "idempotency",
            "receipts",
            "diagnostics",
            "next_actions",
            "result_fingerprint",
            "response_fingerprint",
        ],
    )
});

impl_schema!(
    OperationAdmissionRefusalV1,
    "OperationAdmissionRefusalV1",
    |generator| closed_object(
        vec![
            ("schema", generator.subschema_for::<ExactBinding>()),
            ("request_id", nullable::<RequestId>(generator)),
            ("attempted_operation", nullable::<ExactRef>(generator)),
            ("attempted_definition", nullable::<ExactBinding>(generator)),
            ("request_fingerprint", nullable::<Fingerprint>(generator)),
            ("status", json_schema!({"const": "refused"})),
            ("data", json_schema!({"type": "null"})),
            ("blockers", json_schema!({"type": "array", "maxItems": 0})),
            (
                "refusals",
                json_schema!({"type": "array", "items": generator.subschema_for::<Problem>(), "minItems": 1, "maxItems": 1})
            ),
            ("errors", json_schema!({"type": "array", "maxItems": 0})),
            (
                "response_fingerprint",
                generator.subschema_for::<Fingerprint>()
            ),
        ],
        &[
            "schema",
            "request_id",
            "attempted_operation",
            "attempted_definition",
            "request_fingerprint",
            "status",
            "data",
            "blockers",
            "refusals",
            "errors",
            "response_fingerprint",
        ],
    )
);

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

impl_string_enum!(
    OperationClass,
    "OperationClass",
    ["read_only", "append_only", "compare_and_write"]
);
impl_string_enum!(
    OperationTransport,
    "OperationTransport",
    ["cli_json", "rust_sdk", "tauri"]
);
impl_string_enum!(
    TerminalStatus,
    "TerminalStatus",
    ["ok", "blocked", "refused", "error"]
);
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

impl_schema!(
    CatalogPageSize,
    "CatalogPageSize",
    |_generator| json_schema!({
        "type": "integer", "minimum": 1, "maximum": 128
    })
);

impl_string_enum!(
    BootstrapRefusalReason,
    "BootstrapRefusalReason",
    [
        "stale_descriptor",
        "tampered_descriptor",
        "unsupported_api_version",
        "unsupported_api_major",
        "cross_major_descriptor",
    ]
);

impl_schema!(BootstrapCeilings, "BootstrapCeilings", |_generator| {
    closed_object(
        vec![
            (
                "bootstrap_request_max_bytes",
                json_schema!({"const": 1_048_576}),
            ),
            (
                "bootstrap_response_max_bytes",
                json_schema!({"const": 1_048_576}),
            ),
            (
                "ordinary_request_max_bytes",
                json_schema!({"const": 1_048_576}),
            ),
            (
                "ordinary_response_max_bytes",
                json_schema!({"const": 1_048_576}),
            ),
            ("catalog_page_size_max", json_schema!({"const": 128})),
            ("catalog_entry_max_bytes", json_schema!({"const": 65_536})),
            ("catalog_total_entries_max", json_schema!({"const": 128})),
            ("owner_versions_max_items", json_schema!({"const": 128})),
            ("idempotency_key_max_bytes", json_schema!({"const": 65_536})),
            (
                "bridge_recovery_record_max_bytes",
                json_schema!({"const": 1_048_576}),
            ),
        ],
        &[
            "bootstrap_request_max_bytes",
            "bootstrap_response_max_bytes",
            "ordinary_request_max_bytes",
            "ordinary_response_max_bytes",
            "catalog_page_size_max",
            "catalog_entry_max_bytes",
            "catalog_total_entries_max",
            "owner_versions_max_items",
            "idempotency_key_max_bytes",
            "bridge_recovery_record_max_bytes",
        ],
    )
});

impl_schema!(
    BootstrapSchemaBindings,
    "BootstrapSchemaBindings",
    |generator| {
        let mut binding = || generator.subschema_for::<ExactBinding>();
        closed_object(
            vec![
                ("descriptor_schema", binding()),
                ("bootstrap_request", binding()),
                ("bootstrap_response", binding()),
                ("bootstrap_refusal", binding()),
                ("bootstrap_refusal_details", binding()),
                ("capability_entry", binding()),
                ("owner_version_entry", binding()),
                ("profile_catalog_entry", binding()),
                ("catalog_roots", binding()),
                ("catalog_root", binding()),
                ("typed_catalog_cursor", binding()),
                ("capability_catalog_page", binding()),
                ("schema_catalog_page", binding()),
                ("profile_catalog_page", binding()),
                ("catalog_read_blocked", binding()),
                ("operation_definition", binding()),
                ("operation_request", binding()),
                ("operation_response", binding()),
                ("operation_admission_refusal", binding()),
            ],
            &[
                "descriptor_schema",
                "bootstrap_request",
                "bootstrap_response",
                "bootstrap_refusal",
                "bootstrap_refusal_details",
                "capability_entry",
                "owner_version_entry",
                "profile_catalog_entry",
                "catalog_roots",
                "catalog_root",
                "typed_catalog_cursor",
                "capability_catalog_page",
                "schema_catalog_page",
                "profile_catalog_page",
                "catalog_read_blocked",
                "operation_definition",
                "operation_request",
                "operation_response",
                "operation_admission_refusal",
            ],
        )
    }
);

impl_schema!(
    BootstrapDescriptorV1,
    "BootstrapDescriptorV1",
    |generator| closed_object(
        vec![
            ("schema", generator.subschema_for::<ExactBinding>()),
            ("descriptor", generator.subschema_for::<ExactRef>()),
            ("api_version", generator.subschema_for::<SemVer>()),
            ("api_major", json_schema!({"const": 1})),
            (
                "supported_api_versions",
                generator.subschema_for::<BoundedVec<SemVer, 16>>()
            ),
            (
                "schemas",
                generator.subschema_for::<BootstrapSchemaBindings>()
            ),
            ("ceilings", generator.subschema_for::<BootstrapCeilings>()),
            (
                "declared_transports",
                generator.subschema_for::<BoundedVec<OperationTransport, 3>>()
            ),
            (
                "descriptor_content_fingerprint",
                generator.subschema_for::<Fingerprint>()
            ),
        ],
        &[
            "schema",
            "descriptor",
            "api_version",
            "api_major",
            "supported_api_versions",
            "schemas",
            "ceilings",
            "declared_transports",
            "descriptor_content_fingerprint"
        ],
    )
);

impl_schema!(BootstrapRequestV1, "BootstrapRequestV1", |generator| {
    closed_object(
        vec![
            ("schema", generator.subschema_for::<ExactBinding>()),
            ("request_id", nullable::<RequestId>(generator)),
            (
                "bootstrap_descriptor",
                generator.subschema_for::<ExactBinding>(),
            ),
            ("api_version", generator.subschema_for::<SemVer>()),
            ("page_size", generator.subschema_for::<CatalogPageSize>()),
            (
                "request_fingerprint",
                generator.subschema_for::<Fingerprint>(),
            ),
        ],
        &[
            "schema",
            "request_id",
            "bootstrap_descriptor",
            "api_version",
            "page_size",
            "request_fingerprint",
        ],
    )
});

impl_schema!(OwnerVersionEntry, "OwnerVersionEntry", |generator| {
    closed_object(
        vec![
            (
                "owner_crate",
                generator.subschema_for::<BoundedString<255>>(),
            ),
            ("owner_version", generator.subschema_for::<SemVer>()),
            ("source", generator.subschema_for::<ExactBinding>()),
            (
                "entry_fingerprint",
                generator.subschema_for::<Fingerprint>(),
            ),
        ],
        &[
            "owner_crate",
            "owner_version",
            "source",
            "entry_fingerprint",
        ],
    )
});

impl_schema!(ProfileCatalogEntry, "ProfileCatalogEntry", |generator| {
    closed_object(
        vec![
            ("profile", generator.subschema_for::<ExactBinding>()),
            ("source", generator.subschema_for::<ExactBinding>()),
            (
                "entry_fingerprint",
                generator.subschema_for::<Fingerprint>(),
            ),
        ],
        &["profile", "source", "entry_fingerprint"],
    )
});

impl_schema!(CapabilityEntry, "CapabilityEntry", |generator| {
    closed_object(
        vec![
            ("operation", generator.subschema_for::<ExactBinding>()),
            ("owner", generator.subschema_for::<OperationOwnerBinding>()),
            ("request_schema", generator.subschema_for::<ExactBinding>()),
            ("result_schema", generator.subschema_for::<ExactBinding>()),
            (
                "outcome_schemas",
                generator.subschema_for::<OperationOutcomeSchemas>(),
            ),
            (
                "required_capabilities",
                generator.subschema_for::<BoundedVec<ExactBinding, 128>>(),
            ),
            ("class", generator.subschema_for::<OperationClass>()),
            (
                "idempotency",
                generator.subschema_for::<OperationIdempotency>(),
            ),
            (
                "declared_transports",
                generator.subschema_for::<BoundedVec<OperationTransport, 3>>(),
            ),
            (
                "current_transports",
                generator.subschema_for::<BoundedVec<OperationTransport, 3>>(),
            ),
            ("deprecation", nullable::<ExactBinding>(generator)),
            (
                "entry_fingerprint",
                generator.subschema_for::<Fingerprint>(),
            ),
        ],
        &[
            "operation",
            "owner",
            "request_schema",
            "result_schema",
            "outcome_schemas",
            "required_capabilities",
            "class",
            "idempotency",
            "declared_transports",
            "current_transports",
            "deprecation",
            "entry_fingerprint",
        ],
    )
});

impl_schema!(CatalogRoots, "CatalogRoots", |generator| closed_object(
    vec![
        ("schema", generator.subschema_for::<ExactBinding>()),
        ("operations", generator.subschema_for::<CatalogRoot>()),
        ("schemas", generator.subschema_for::<CatalogRoot>()),
        ("profiles", generator.subschema_for::<CatalogRoot>()),
        (
            "roots_fingerprint",
            generator.subschema_for::<Fingerprint>()
        ),
    ],
    &[
        "schema",
        "operations",
        "schemas",
        "profiles",
        "roots_fingerprint"
    ],
));

impl_schema!(TypedCatalogCursor, "TypedCatalogCursor", |generator| {
    closed_object(
        vec![
            ("schema", generator.subschema_for::<ExactBinding>()),
            ("api_major", json_schema!({"const": 1})),
            ("catalog", generator.subschema_for::<ExactBinding>()),
            ("page_size", generator.subschema_for::<CatalogPageSize>()),
            ("next_index", generator.subschema_for::<SafeU64>()),
            ("next_sort_key", generator.subschema_for::<ExactRef>()),
            ("expires_at_utc", generator.subschema_for::<UtcTimestamp>()),
            (
                "cursor_fingerprint",
                generator.subschema_for::<Fingerprint>(),
            ),
        ],
        &[
            "schema",
            "api_major",
            "catalog",
            "page_size",
            "next_index",
            "next_sort_key",
            "expires_at_utc",
            "cursor_fingerprint",
        ],
    )
});

impl<T> JsonSchema for TypedCatalogPage<T>
where
    T: JsonSchema + TypedCatalogEntry,
{
    fn schema_name() -> Cow<'static, str> {
        format!("TypedCatalogPage_{}", T::schema_name()).into()
    }

    fn schema_id() -> Cow<'static, str> {
        format!(
            "handbook_sdk::contract_membrane::TypedCatalogPage<{}>",
            T::schema_id()
        )
        .into()
    }

    fn json_schema(generator: &mut SchemaGenerator) -> Schema {
        closed_object(
            vec![
                ("schema", generator.subschema_for::<ExactBinding>()),
                ("api_major", json_schema!({"const": 1})),
                ("root", generator.subschema_for::<CatalogRoot>()),
                ("page_size", generator.subschema_for::<CatalogPageSize>()),
                ("start_index", generator.subschema_for::<SafeU64>()),
                ("entries", generator.subschema_for::<BoundedVec<T, 128>>()),
                ("next_cursor", nullable::<TypedCatalogCursor>(generator)),
                ("page_fingerprint", generator.subschema_for::<Fingerprint>()),
            ],
            &[
                "schema",
                "api_major",
                "root",
                "page_size",
                "start_index",
                "entries",
                "next_cursor",
                "page_fingerprint",
            ],
        )
    }
}

impl_schema!(BootstrapResponseV1, "BootstrapResponseV1", |generator| {
    closed_object(
        vec![
            ("schema", generator.subschema_for::<ExactBinding>()),
            ("request_id", nullable::<RequestId>(generator)),
            (
                "request_fingerprint",
                generator.subschema_for::<Fingerprint>(),
            ),
            ("status", json_schema!({"const": "ok"})),
            (
                "descriptor",
                generator.subschema_for::<BootstrapDescriptorV1>(),
            ),
            ("catalog_roots", generator.subschema_for::<CatalogRoots>()),
            (
                "first_capability_page",
                generator.subschema_for::<CapabilityCatalogPage>(),
            ),
            (
                "owner_versions",
                generator.subschema_for::<BoundedVec<OwnerVersionEntry, 128>>(),
            ),
            (
                "schema_manifest",
                generator.subschema_for::<BoundedVec<SchemaManifestEntry, 128>>(),
            ),
            (
                "response_fingerprint",
                generator.subschema_for::<Fingerprint>(),
            ),
        ],
        &[
            "schema",
            "request_id",
            "request_fingerprint",
            "status",
            "descriptor",
            "catalog_roots",
            "first_capability_page",
            "owner_versions",
            "schema_manifest",
            "response_fingerprint",
        ],
    )
});

impl_schema!(
    BootstrapRefusalDetails,
    "BootstrapRefusalDetails",
    |generator| closed_object(
        vec![
            (
                "reason",
                generator.subschema_for::<BootstrapRefusalReason>()
            ),
            (
                "supported_api_versions",
                generator.subschema_for::<BoundedVec<SemVer, 16>>()
            ),
        ],
        &["reason", "supported_api_versions"],
    )
);

impl_schema!(BootstrapRefusalV1, "BootstrapRefusalV1", |generator| {
    closed_object(
        vec![
            ("schema", generator.subschema_for::<ExactBinding>()),
            ("request_id", nullable::<RequestId>(generator)),
            (
                "request_fingerprint",
                generator.subschema_for::<Fingerprint>(),
            ),
            ("bootstrap_descriptor", nullable::<ExactBinding>(generator)),
            ("status", json_schema!({"const": "refused"})),
            ("data", json_schema!({"type": "null"})),
            ("blockers", json_schema!({"type": "array", "maxItems": 0})),
            (
                "refusals",
                json_schema!({"type": "array", "items": generator.subschema_for::<Problem>(), "minItems": 1, "maxItems": 1}),
            ),
            ("errors", json_schema!({"type": "array", "maxItems": 0})),
            (
                "schema_manifest",
                generator.subschema_for::<BoundedVec<SchemaManifestEntry, 128>>(),
            ),
            (
                "response_fingerprint",
                generator.subschema_for::<Fingerprint>(),
            ),
        ],
        &[
            "schema",
            "request_id",
            "request_fingerprint",
            "bootstrap_descriptor",
            "status",
            "data",
            "blockers",
            "refusals",
            "errors",
            "schema_manifest",
            "response_fingerprint",
        ],
    )
});

impl_schema!(CatalogReadBlockedV1, "CatalogReadBlockedV1", |generator| {
    closed_object(
        vec![
            ("schema", generator.subschema_for::<ExactBinding>()),
            ("api_major", json_schema!({"const": 1})),
            ("catalog", generator.subschema_for::<ExactBinding>()),
            ("status", json_schema!({"const": "blocked"})),
            ("data", json_schema!({"type": "null"})),
            (
                "blockers",
                json_schema!({"type": "array", "items": generator.subschema_for::<Problem>(), "minItems": 1, "maxItems": 1}),
            ),
            ("refusals", json_schema!({"type": "array", "maxItems": 0})),
            ("errors", json_schema!({"type": "array", "maxItems": 0})),
            ("restart_from_first_page", json_schema!({"const": true})),
            (
                "response_fingerprint",
                generator.subschema_for::<Fingerprint>(),
            ),
        ],
        &[
            "schema",
            "api_major",
            "catalog",
            "status",
            "data",
            "blockers",
            "refusals",
            "errors",
            "restart_from_first_page",
            "response_fingerprint",
        ],
    )
});

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

impl_schema!(
    PostureReferenceDtoV1,
    "PostureReferenceDtoV1",
    |generator| {
        closed_object(
            vec![
                ("reference", generator.subschema_for::<BoundedString<255>>()),
                ("fingerprint", generator.subschema_for::<Fingerprint>()),
            ],
            &["reference", "fingerprint"],
        )
    }
);

impl_schema!(
    PostureCanonicalBindingDtoV1,
    "PostureCanonicalBindingDtoV1",
    |generator| {
        closed_object(
            vec![
                (
                    "reference",
                    json_schema!({"const": ".handbook/project/charter.yaml"}),
                ),
                ("fingerprint", generator.subschema_for::<Fingerprint>()),
                ("document_sha256", generator.subschema_for::<Fingerprint>()),
                (
                    "byte_length",
                    json_schema!({"type": "integer", "minimum": 1, "maximum": MAX_SAFE_INTEGER}),
                ),
            ],
            &["reference", "fingerprint", "document_sha256", "byte_length"],
        )
    }
);

impl_schema!(
    PostureDimensionChangeDtoV1,
    "PostureDimensionChangeDtoV1",
    |generator| {
        let level = json_schema!({"type": "integer", "minimum": 0, "maximum": 255});
        closed_object(
            vec![
                (
                    "dimension_id",
                    generator.subschema_for::<BoundedString<255>>(),
                ),
                ("dimension_index", level.clone()),
                ("authority_path", generator.subschema_for::<JsonPointer>()),
                (
                    "expected_stored_value",
                    json_schema!({"anyOf": [level.clone(), {"type": "null"}]}),
                ),
                ("expected_effective_level", level.clone()),
                ("proposed_effective_level", level),
            ],
            &[
                "dimension_id",
                "dimension_index",
                "authority_path",
                "expected_stored_value",
                "expected_effective_level",
                "proposed_effective_level",
            ],
        )
    }
);

struct PostureTransitionApplyRequestSchemaV1;

impl_schema!(
    PostureTransitionApplyRequestSchemaV1,
    "PostureTransitionApplyRequestSchemaV1",
    |generator| {
        closed_object(
            vec![
                (
                    "change",
                    generator.subschema_for::<PostureDimensionChangeDtoV1>(),
                ),
                (
                    "effective_at_utc",
                    generator.subschema_for::<UtcTimestamp>(),
                ),
                (
                    "expected_canonical",
                    generator.subschema_for::<PostureCanonicalBindingDtoV1>(),
                ),
                (
                    "idempotency_key",
                    json_schema!({"type": "string", "minLength": 1, "maxLength": 256, "pattern": "^[!-~]{1,256}$"}),
                ),
                (
                    "reassessment_coverage_ids",
                    json_schema!({
                        "type": "array",
                        "items": {"type": "string", "minLength": 1, "maxLength": 256, "pattern": "^[!-~]{1,256}$"},
                        "minItems": 1,
                        "maxItems": 64,
                        "uniqueItems": true
                    }),
                ),
                (
                    "recommendation",
                    generator.subschema_for::<PostureReferenceDtoV1>(),
                ),
                (
                    "repository_identity_fingerprint",
                    generator.subschema_for::<Fingerprint>(),
                ),
            ],
            &[
                "change",
                "effective_at_utc",
                "expected_canonical",
                "idempotency_key",
                "reassessment_coverage_ids",
                "recommendation",
                "repository_identity_fingerprint",
            ],
        )
    }
);

impl_schema!(
    PostureTransitionApplyResultDataV1,
    "PostureTransitionApplyResultDataV1",
    |generator| {
        closed_object(
            vec![
                (
                    "idempotency_key_fingerprint",
                    generator.subschema_for::<Fingerprint>(),
                ),
                (
                    "repository_identity_fingerprint",
                    generator.subschema_for::<Fingerprint>(),
                ),
                (
                    "prior_canonical",
                    generator.subschema_for::<PostureCanonicalBindingDtoV1>(),
                ),
                (
                    "resulting_canonical",
                    generator.subschema_for::<PostureCanonicalBindingDtoV1>(),
                ),
                (
                    "posture_transition",
                    generator.subschema_for::<PostureReferenceDtoV1>(),
                ),
                (
                    "lifecycle_transition",
                    generator.subschema_for::<PostureReferenceDtoV1>(),
                ),
                (
                    "resulting_kernel",
                    generator.subschema_for::<PostureReferenceDtoV1>(),
                ),
            ],
            &[
                "idempotency_key_fingerprint",
                "repository_identity_fingerprint",
                "prior_canonical",
                "resulting_canonical",
                "posture_transition",
                "lifecycle_transition",
                "resulting_kernel",
            ],
        )
    }
);

impl_schema!(
    PostureCanonicalWriteConditionV1,
    "PostureCanonicalWriteConditionV1",
    |generator| closed_object(
        vec![
            (
                "prior_canonical",
                generator.subschema_for::<PostureCanonicalBindingDtoV1>(),
            ),
            (
                "resulting_canonical",
                generator.subschema_for::<PostureCanonicalBindingDtoV1>(),
            ),
        ],
        &["prior_canonical", "resulting_canonical"],
    )
);

impl_schema!(
    PostureSemanticWriteConditionV1,
    "PostureSemanticWriteConditionV1",
    |generator| closed_object(
        vec![
            (
                "lifecycle_transition",
                generator.subschema_for::<PostureReferenceDtoV1>(),
            ),
            (
                "posture_transition",
                generator.subschema_for::<PostureReferenceDtoV1>(),
            ),
            (
                "resulting_kernel",
                generator.subschema_for::<PostureReferenceDtoV1>(),
            ),
        ],
        &[
            "lifecycle_transition",
            "posture_transition",
            "resulting_kernel",
        ],
    )
);

macro_rules! posture_variant_schema {
    ($type:ident, $schema_name:literal, $variant:literal) => {
        struct $type;
        impl_schema!($type, $schema_name, |_generator| closed_object(
            vec![("owner_variant", json_schema!({"const": $variant}))],
            &["owner_variant"],
        ));
    };
}

posture_variant_schema!(
    PostureAuthorityConflictDetailsSchema,
    "PostureAuthorityConflictDetailsSchema",
    "AuthorityConflict"
);
posture_variant_schema!(
    PostureIntegrityOrDurabilityDetailsSchema,
    "PostureIntegrityOrDurabilityDetailsSchema",
    "IntegrityOrDurability"
);
posture_variant_schema!(
    PostureInvalidFixedDimensionDetailsSchema,
    "PostureInvalidFixedDimensionDetailsSchema",
    "InvalidFixedDimension"
);
posture_variant_schema!(
    PostureInvalidIdempotencyKeyDetailsSchema,
    "PostureInvalidIdempotencyKeyDetailsSchema",
    "InvalidIdempotencyKey"
);
posture_variant_schema!(
    PostureInvalidSemanticInputDetailsSchema,
    "PostureInvalidSemanticInputDetailsSchema",
    "InvalidSemanticInput"
);
posture_variant_schema!(PostureIoDetailsSchema, "PostureIoDetailsSchema", "Io");
posture_variant_schema!(
    PostureLifecycleNotCurrentDetailsSchema,
    "PostureLifecycleNotCurrentDetailsSchema",
    "LifecycleNotCurrent"
);
posture_variant_schema!(
    PostureMissingLifecycleAuthorityDetailsSchema,
    "PostureMissingLifecycleAuthorityDetailsSchema",
    "MissingLifecycleAuthority"
);
posture_variant_schema!(
    PostureRepositoryIdentityMismatchDetailsSchema,
    "PostureRepositoryIdentityMismatchDetailsSchema",
    "RepositoryIdentityMismatch"
);
posture_variant_schema!(
    PostureStaleCanonicalDetailsSchema,
    "PostureStaleCanonicalDetailsSchema",
    "StaleCanonical"
);
posture_variant_schema!(
    PostureUnsafeFilesystemDetailsSchema,
    "PostureUnsafeFilesystemDetailsSchema",
    "UnsafeFilesystem"
);
posture_variant_schema!(
    PostureUnsupportedPlatformDetailsSchema,
    "PostureUnsupportedPlatformDetailsSchema",
    "UnsupportedPlatform"
);
posture_variant_schema!(
    PostureUnsupportedPlatformRecheckSchema,
    "PostureUnsupportedPlatformRecheckSchema",
    "UnsupportedPlatform"
);

const DIALECT: &str = "https://json-schema.org/draft/2020-12/schema";

struct SchemaDescriptor {
    exact_ref: &'static str,
    relative_path: &'static str,
    generate: fn() -> Schema,
}

const INVENTORY: [SchemaDescriptor; 53] = [
    descriptor::<ArtifactRef>(
        "handbook.artifact-ref@1.0.0",
        "handbook.artifact-ref/1.0.0.schema.json",
    ),
    descriptor::<BootstrapDescriptorV1>(
        "handbook.bootstrap-descriptor-schema@1.0.0",
        "handbook.bootstrap-descriptor-schema/1.0.0.schema.json",
    ),
    descriptor::<BootstrapRefusalDetails>(
        "handbook.bootstrap-refusal-details@1.0.0",
        "handbook.bootstrap-refusal-details/1.0.0.schema.json",
    ),
    descriptor::<BootstrapRefusalV1>(
        "handbook.bootstrap-refusal@1.0.0",
        "handbook.bootstrap-refusal/1.0.0.schema.json",
    ),
    descriptor::<BootstrapRequestV1>(
        "handbook.bootstrap-request@1.0.0",
        "handbook.bootstrap-request/1.0.0.schema.json",
    ),
    descriptor::<BootstrapResponseV1>(
        "handbook.bootstrap-response@1.0.0",
        "handbook.bootstrap-response/1.0.0.schema.json",
    ),
    descriptor::<CapabilityCatalogPage>(
        "handbook.capability-catalog-page@1.0.0",
        "handbook.capability-catalog-page/1.0.0.schema.json",
    ),
    descriptor::<CapabilityEntry>(
        "handbook.capability-entry@1.0.0",
        "handbook.capability-entry/1.0.0.schema.json",
    ),
    descriptor::<CatalogCursor>(
        "handbook.catalog-cursor@1.0.0",
        "handbook.catalog-cursor/1.0.0.schema.json",
    ),
    descriptor::<CatalogPage>(
        "handbook.catalog-page@1.0.0",
        "handbook.catalog-page/1.0.0.schema.json",
    ),
    descriptor::<CatalogReadBlockedV1>(
        "handbook.catalog-read-blocked@1.0.0",
        "handbook.catalog-read-blocked/1.0.0.schema.json",
    ),
    descriptor::<CatalogRoot>(
        "handbook.catalog-root@1.0.0",
        "handbook.catalog-root/1.0.0.schema.json",
    ),
    descriptor::<CatalogRoots>(
        "handbook.catalog-roots@1.0.0",
        "handbook.catalog-roots/1.0.0.schema.json",
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
    descriptor::<OperationAdmissionRefusalV1>(
        "handbook.operation-admission-refusal@1.0.0",
        "handbook.operation-admission-refusal/1.0.0.schema.json",
    ),
    descriptor::<Problem>(
        "handbook.operation-blocker@1.0.0",
        "handbook.operation-blocker/1.0.0.schema.json",
    ),
    descriptor::<OperationDefinition>(
        "handbook.operation-definition@1.0.0",
        "handbook.operation-definition/1.0.0.schema.json",
    ),
    descriptor::<Problem>(
        "handbook.operation-error@1.0.0",
        "handbook.operation-error/1.0.0.schema.json",
    ),
    descriptor::<Problem>(
        "handbook.operation-refusal@1.0.0",
        "handbook.operation-refusal/1.0.0.schema.json",
    ),
    descriptor::<OperationRequestV1>(
        "handbook.operation-request@1.0.0",
        "handbook.operation-request/1.0.0.schema.json",
    ),
    descriptor::<OperationResponseV1>(
        "handbook.operation-response@1.0.0",
        "handbook.operation-response/1.0.0.schema.json",
    ),
    descriptor::<PostureTransitionApplyRequestSchemaV1>(
        "handbook.operation.posture-transition-apply-request@1.0.0",
        "handbook.operation.posture-transition-apply-request/1.0.0.schema.json",
    ),
    descriptor::<PostureTransitionApplyResultDataV1>(
        "handbook.operation.posture-transition-apply-result@1.0.0",
        "handbook.operation.posture-transition-apply-result/1.0.0.schema.json",
    ),
    descriptor::<OwnerVersionEntry>(
        "handbook.owner-version-entry@1.0.0",
        "handbook.owner-version-entry/1.0.0.schema.json",
    ),
    descriptor::<PostureAuthorityConflictDetailsSchema>(
        "handbook.posture-details.authority-conflict@1.0.0",
        "handbook.posture-details.authority-conflict/1.0.0.schema.json",
    ),
    descriptor::<PostureIntegrityOrDurabilityDetailsSchema>(
        "handbook.posture-details.integrity-or-durability@1.0.0",
        "handbook.posture-details.integrity-or-durability/1.0.0.schema.json",
    ),
    descriptor::<PostureInvalidFixedDimensionDetailsSchema>(
        "handbook.posture-details.invalid-fixed-dimension@1.0.0",
        "handbook.posture-details.invalid-fixed-dimension/1.0.0.schema.json",
    ),
    descriptor::<PostureInvalidIdempotencyKeyDetailsSchema>(
        "handbook.posture-details.invalid-idempotency-key@1.0.0",
        "handbook.posture-details.invalid-idempotency-key/1.0.0.schema.json",
    ),
    descriptor::<PostureInvalidSemanticInputDetailsSchema>(
        "handbook.posture-details.invalid-semantic-input@1.0.0",
        "handbook.posture-details.invalid-semantic-input/1.0.0.schema.json",
    ),
    descriptor::<PostureIoDetailsSchema>(
        "handbook.posture-details.io@1.0.0",
        "handbook.posture-details.io/1.0.0.schema.json",
    ),
    descriptor::<PostureLifecycleNotCurrentDetailsSchema>(
        "handbook.posture-details.lifecycle-not-current@1.0.0",
        "handbook.posture-details.lifecycle-not-current/1.0.0.schema.json",
    ),
    descriptor::<PostureMissingLifecycleAuthorityDetailsSchema>(
        "handbook.posture-details.missing-lifecycle-authority@1.0.0",
        "handbook.posture-details.missing-lifecycle-authority/1.0.0.schema.json",
    ),
    descriptor::<PostureRepositoryIdentityMismatchDetailsSchema>(
        "handbook.posture-details.repository-identity-mismatch@1.0.0",
        "handbook.posture-details.repository-identity-mismatch/1.0.0.schema.json",
    ),
    descriptor::<PostureStaleCanonicalDetailsSchema>(
        "handbook.posture-details.stale-canonical@1.0.0",
        "handbook.posture-details.stale-canonical/1.0.0.schema.json",
    ),
    descriptor::<PostureUnsafeFilesystemDetailsSchema>(
        "handbook.posture-details.unsafe-filesystem@1.0.0",
        "handbook.posture-details.unsafe-filesystem/1.0.0.schema.json",
    ),
    descriptor::<PostureUnsupportedPlatformDetailsSchema>(
        "handbook.posture-details.unsupported-platform@1.0.0",
        "handbook.posture-details.unsupported-platform/1.0.0.schema.json",
    ),
    descriptor::<PostureUnsupportedPlatformRecheckSchema>(
        "handbook.posture-recheck.unsupported-platform@1.0.0",
        "handbook.posture-recheck.unsupported-platform/1.0.0.schema.json",
    ),
    descriptor::<PostureCanonicalWriteConditionV1>(
        "handbook.posture-write-condition.canonical-truth@1.0.0",
        "handbook.posture-write-condition.canonical-truth/1.0.0.schema.json",
    ),
    descriptor::<PostureSemanticWriteConditionV1>(
        "handbook.posture-write-condition.semantic-record@1.0.0",
        "handbook.posture-write-condition.semantic-record/1.0.0.schema.json",
    ),
    descriptor::<ProblemBinding>(
        "handbook.problem-binding@1.0.0",
        "handbook.problem-binding/1.0.0.schema.json",
    ),
    descriptor::<Problem>(
        "handbook.problem@1.0.0",
        "handbook.problem/1.0.0.schema.json",
    ),
    descriptor::<ProfileCatalogEntry>(
        "handbook.profile-catalog-entry@1.0.0",
        "handbook.profile-catalog-entry/1.0.0.schema.json",
    ),
    descriptor::<ProfileCatalogPage>(
        "handbook.profile-catalog-page@1.0.0",
        "handbook.profile-catalog-page/1.0.0.schema.json",
    ),
    descriptor::<RecheckCondition>(
        "handbook.recheck-condition@1.0.0",
        "handbook.recheck-condition/1.0.0.schema.json",
    ),
    descriptor::<SchemaCatalogPage>(
        "handbook.schema-catalog-page@1.0.0",
        "handbook.schema-catalog-page/1.0.0.schema.json",
    ),
    descriptor::<SchemaManifestEntry>(
        "handbook.schema-manifest-entry@1.0.0",
        "handbook.schema-manifest-entry/1.0.0.schema.json",
    ),
    descriptor::<SourceBinding>(
        "handbook.source-binding@1.0.0",
        "handbook.source-binding/1.0.0.schema.json",
    ),
    descriptor::<TypedCatalogCursor>(
        "handbook.typed-catalog-cursor@1.0.0",
        "handbook.typed-catalog-cursor/1.0.0.schema.json",
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
    if INVENTORY.len() != 53 {
        return Err("schema inventory must contain exactly 53 entries".to_owned());
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
    if descriptor.exact_ref == REQUEST_SCHEMA_REF {
        validate_posture_request_raw_key_schema(value, descriptor.relative_path)
    } else {
        validate_no_raw_key_names(value, descriptor.relative_path)
    }
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

fn validate_posture_request_raw_key_schema(
    value: &serde_json::Value,
    relative_path: &str,
) -> Result<(), String> {
    let properties = value
        .get("properties")
        .and_then(serde_json::Value::as_object)
        .ok_or_else(|| format!("{relative_path} has no root properties"))?;
    let required = value
        .get("required")
        .and_then(serde_json::Value::as_array)
        .ok_or_else(|| format!("{relative_path} has no root required set"))?;
    if !properties.contains_key("idempotency_key")
        || required
            .iter()
            .filter(|member| member.as_str() == Some("idempotency_key"))
            .count()
            != 1
        || raw_key_name_occurrences(value) != 2
    {
        return Err(format!(
            "{relative_path} must name the raw key only as one required root wire member"
        ));
    }
    Ok(())
}

fn raw_key_name_occurrences(value: &serde_json::Value) -> usize {
    match value {
        serde_json::Value::Array(values) => values.iter().map(raw_key_name_occurrences).sum(),
        serde_json::Value::Object(members) => members
            .iter()
            .map(|(key, value)| {
                usize::from(matches!(
                    key.as_str(),
                    "idempotency_key" | "raw_idempotency_key"
                )) + raw_key_name_occurrences(value)
            })
            .sum(),
        serde_json::Value::String(value)
            if matches!(value.as_str(), "idempotency_key" | "raw_idempotency_key") =>
        {
            1
        }
        _ => 0,
    }
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
