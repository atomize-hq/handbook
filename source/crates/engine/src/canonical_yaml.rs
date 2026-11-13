use crate::{parse_definition_yaml, MAX_SOURCE_DOCUMENT_BYTES};
use serde_json::Value;
use std::fmt;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CanonicalYamlErrorKindV1 {
    UnsupportedValue,
    OutputLimitExceeded,
    ParseFailure,
    RoundTripMismatch,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CanonicalYamlErrorV1 {
    kind: CanonicalYamlErrorKindV1,
    detail: &'static str,
}

impl CanonicalYamlErrorV1 {
    pub fn kind(&self) -> CanonicalYamlErrorKindV1 {
        self.kind
    }

    pub fn detail(&self) -> &'static str {
        self.detail
    }
}

impl fmt::Display for CanonicalYamlErrorV1 {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.detail)
    }
}

impl std::error::Error for CanonicalYamlErrorV1 {}

pub fn canonical_yaml_bytes(value: &Value) -> Result<Vec<u8>, CanonicalYamlErrorV1> {
    let mut output = String::new();
    emit_value(value, 0, &mut output)?;
    if !output.ends_with('\n') {
        output.push('\n');
    }
    if output.len() > MAX_SOURCE_DOCUMENT_BYTES {
        return Err(yaml_error(
            CanonicalYamlErrorKindV1::OutputLimitExceeded,
            "canonical YAML exceeds the 1 MiB document limit",
        ));
    }
    let bytes = output.into_bytes();
    let reparsed = parse_canonical_yaml(&bytes)?;
    if &reparsed != value {
        return Err(yaml_error(
            CanonicalYamlErrorKindV1::RoundTripMismatch,
            "canonical YAML parse/emit/parse equality failed",
        ));
    }
    Ok(bytes)
}

pub fn parse_canonical_yaml(bytes: &[u8]) -> Result<Value, CanonicalYamlErrorV1> {
    parse_definition_yaml(bytes).map_err(|_| {
        yaml_error(
            CanonicalYamlErrorKindV1::ParseFailure,
            "YAML is not one duplicate-free JSON-data-model document",
        )
    })
}

fn emit_value(
    value: &Value,
    indentation: usize,
    output: &mut String,
) -> Result<(), CanonicalYamlErrorV1> {
    match value {
        Value::Object(object) if object.is_empty() => {
            indent(indentation, output);
            output.push_str("{}\n");
        }
        Value::Array(values) if values.is_empty() => {
            indent(indentation, output);
            output.push_str("[]\n");
        }
        Value::Object(object) => {
            let mut members = object.iter().collect::<Vec<_>>();
            members.sort_by(|(left, _), (right, _)| left.as_bytes().cmp(right.as_bytes()));
            for (key, child) in members {
                indent(indentation, output);
                output.push_str(&emit_key(key)?);
                output.push(':');
                if is_inline(child) {
                    output.push(' ');
                    output.push_str(&emit_inline(child)?);
                    output.push('\n');
                } else {
                    output.push('\n');
                    emit_value(child, indentation + 2, output)?;
                }
            }
        }
        Value::Array(values) => {
            for child in values {
                indent(indentation, output);
                output.push('-');
                if is_inline(child) {
                    output.push(' ');
                    output.push_str(&emit_inline(child)?);
                    output.push('\n');
                } else {
                    output.push('\n');
                    emit_value(child, indentation + 2, output)?;
                }
            }
        }
        scalar => {
            indent(indentation, output);
            output.push_str(&emit_inline(scalar)?);
            output.push('\n');
        }
    }
    if output.len() > MAX_SOURCE_DOCUMENT_BYTES {
        return Err(yaml_error(
            CanonicalYamlErrorKindV1::OutputLimitExceeded,
            "canonical YAML exceeds the 1 MiB document limit",
        ));
    }
    Ok(())
}

fn is_inline(value: &Value) -> bool {
    !matches!(value, Value::Object(object) if !object.is_empty())
        && !matches!(value, Value::Array(values) if !values.is_empty())
}

fn emit_inline(value: &Value) -> Result<String, CanonicalYamlErrorV1> {
    match value {
        Value::Null => Ok("null".to_string()),
        Value::Bool(value) => Ok(value.to_string()),
        Value::Number(value) => Ok(value.to_string()),
        Value::String(value) => serde_json::to_string(value).map_err(|_| {
            yaml_error(
                CanonicalYamlErrorKindV1::UnsupportedValue,
                "string is not representable in the JSON data model",
            )
        }),
        Value::Object(object) if object.is_empty() => Ok("{}".to_string()),
        Value::Array(values) if values.is_empty() => Ok("[]".to_string()),
        Value::Object(_) | Value::Array(_) => Err(yaml_error(
            CanonicalYamlErrorKindV1::UnsupportedValue,
            "nonempty containers require block serialization",
        )),
    }
}

fn emit_key(value: &str) -> Result<String, CanonicalYamlErrorV1> {
    let safe_plain = !value.is_empty()
        && value.is_ascii()
        && value
            .bytes()
            .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'_' | b'-'))
        && value
            .as_bytes()
            .first()
            .is_some_and(|byte| byte.is_ascii_alphabetic() || *byte == b'_')
        && !matches!(
            value.to_ascii_lowercase().as_str(),
            "null" | "true" | "false" | "yes" | "no" | "on" | "off"
        );
    if safe_plain {
        Ok(value.to_string())
    } else {
        serde_json::to_string(value).map_err(|_| {
            yaml_error(
                CanonicalYamlErrorKindV1::UnsupportedValue,
                "mapping key is not representable in the JSON data model",
            )
        })
    }
}

fn indent(count: usize, output: &mut String) {
    output.extend(std::iter::repeat_n(' ', count));
}

fn yaml_error(kind: CanonicalYamlErrorKindV1, detail: &'static str) -> CanonicalYamlErrorV1 {
    CanonicalYamlErrorV1 { kind, detail }
}
