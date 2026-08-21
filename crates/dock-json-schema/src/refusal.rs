use serde::{Deserialize, Serialize};
use std::fmt;

/// The unchanged child-process refusal vocabulary supplied by HCM-5.3.
#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ValidatorRefusalReasonV1 {
    InputRefused,
    UnsupportedInput,
    LimitRefused,
    ObservationUnavailable,
    InternalRefusal,
}

/// Stable bounded validator diagnostic codes selected by HCM-5.4.
#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum ValidatorDiagnosticCodeV1 {
    #[serde(rename = "handbook.validator.input-malformed")]
    InputMalformed,
    #[serde(rename = "handbook.validator.input-substituted")]
    InputSubstituted,
    #[serde(rename = "handbook.validator.schema-unsupported")]
    SchemaUnsupported,
    #[serde(rename = "handbook.validator.ref-remote")]
    RefRemote,
    #[serde(rename = "handbook.validator.ref-missing")]
    RefMissing,
    #[serde(rename = "handbook.validator.limit-exceeded")]
    LimitExceeded,
    #[serde(rename = "handbook.validator.observation-unavailable")]
    ObservationUnavailable,
    #[serde(rename = "handbook.validator.internal-refusal")]
    InternalRefusal,
}

impl ValidatorDiagnosticCodeV1 {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::InputMalformed => "handbook.validator.input-malformed",
            Self::InputSubstituted => "handbook.validator.input-substituted",
            Self::SchemaUnsupported => "handbook.validator.schema-unsupported",
            Self::RefRemote => "handbook.validator.ref-remote",
            Self::RefMissing => "handbook.validator.ref-missing",
            Self::LimitExceeded => "handbook.validator.limit-exceeded",
            Self::ObservationUnavailable => "handbook.validator.observation-unavailable",
            Self::InternalRefusal => "handbook.validator.internal-refusal",
        }
    }
}

/// Closed validator-local classification used before wire projection.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ValidatorRefusalV1 {
    InputMalformed,
    InputSubstituted,
    SchemaUnsupported,
    RefRemote,
    RefMissing,
    LimitExceeded,
    ObservationUnavailable,
    InternalRefusal,
}

impl ValidatorRefusalV1 {
    pub const fn reason(self) -> ValidatorRefusalReasonV1 {
        match self {
            Self::InputMalformed | Self::InputSubstituted => ValidatorRefusalReasonV1::InputRefused,
            Self::SchemaUnsupported | Self::RefRemote | Self::RefMissing => {
                ValidatorRefusalReasonV1::UnsupportedInput
            }
            Self::LimitExceeded => ValidatorRefusalReasonV1::LimitRefused,
            Self::ObservationUnavailable => ValidatorRefusalReasonV1::ObservationUnavailable,
            Self::InternalRefusal => ValidatorRefusalReasonV1::InternalRefusal,
        }
    }

    pub const fn diagnostic_code(self) -> ValidatorDiagnosticCodeV1 {
        match self {
            Self::InputMalformed => ValidatorDiagnosticCodeV1::InputMalformed,
            Self::InputSubstituted => ValidatorDiagnosticCodeV1::InputSubstituted,
            Self::SchemaUnsupported => ValidatorDiagnosticCodeV1::SchemaUnsupported,
            Self::RefRemote => ValidatorDiagnosticCodeV1::RefRemote,
            Self::RefMissing => ValidatorDiagnosticCodeV1::RefMissing,
            Self::LimitExceeded => ValidatorDiagnosticCodeV1::LimitExceeded,
            Self::ObservationUnavailable => ValidatorDiagnosticCodeV1::ObservationUnavailable,
            Self::InternalRefusal => ValidatorDiagnosticCodeV1::InternalRefusal,
        }
    }
}

impl fmt::Display for ValidatorRefusalV1 {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.diagnostic_code().as_str())
    }
}

impl std::error::Error for ValidatorRefusalV1 {}
