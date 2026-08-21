//! Closed input and fact kernel for the native offline JSON Schema validator.
//!
//! This crate deliberately has no Handbook product-crate dependency. It owns
//! only structural admission, canonical RFC-8785/SHA-256 bindings, and the
//! normalized observation vocabulary. Evaluation and process framing are
//! separate packets.

mod bounds;
mod canonical;
mod fact;
mod identity;
mod input;
mod refusal;

pub use bounds::ValidatorLimitsV1;
pub use fact::{ConformanceV1, NormalizedViolationV1, SchemaConformanceFactV1, ViolationCodeV1};
pub use identity::{CanonicalFingerprintV1, ExactSchemaBindingV1, LogicalSchemaPathV1};
pub use input::{JsonInstanceV1, JsonSchemaValidationInputV1, SchemaClosureMemberV1};
pub use refusal::{ValidatorDiagnosticCodeV1, ValidatorRefusalReasonV1, ValidatorRefusalV1};

/// The sole validation-input schema identity selected by HCM-5.4.
pub const VALIDATION_INPUT_SCHEMA_REF_V1: &str = "handbook.dock-json-schema-validation-input@1.0.0";

/// The sole schema-conformance fact identity selected by HCM-5.4.
pub const SCHEMA_CONFORMANCE_FACT_SCHEMA_REF_V1: &str = "handbook.fact.schema-conformance@1.0.0";
