use super::{ContractError, ContractLimits, Fingerprint};
use serde::de::DeserializeOwned;
use serde::ser::{
    SerializeMap, SerializeSeq, SerializeStruct, SerializeStructVariant, SerializeTuple,
    SerializeTupleStruct, SerializeTupleVariant,
};
use serde::{Serialize, Serializer};
use sha2::{Digest, Sha256};
use std::cell::Cell;
use std::cmp::Ordering;
use std::fmt::{self, Write as _};

const MAX_SAFE_INTEGER: i64 = 9_007_199_254_740_991;
const MIN_SAFE_INTEGER: i64 = -9_007_199_254_740_991;

#[derive(Clone, Debug, Eq, PartialEq)]
enum CanonicalValue {
    Null,
    Bool(bool),
    String(String),
    Integer(i64),
    Array(Vec<CanonicalValue>),
    Object(Vec<(String, CanonicalValue)>),
}

pub fn canonical_json_bytes<T: Serialize>(
    value: &T,
    limits: &ContractLimits,
) -> Result<Vec<u8>, ContractError> {
    let capture = CaptureContext::new(limits);
    let value = value
        .serialize(CanonicalSerializer::new(&capture, 0))
        .map_err(|error| error.0)?;
    value.validate(limits, 0)?;
    let mut output = String::new();
    value.write_canonical(&mut output)?;
    if output.len() > limits.document_bytes() {
        return Err(ContractError::BoundExceeded);
    }
    Ok(output.into_bytes())
}

pub fn parse_canonical_json<T: DeserializeOwned + Serialize>(
    bytes: &[u8],
    limits: &ContractLimits,
) -> Result<T, ContractError> {
    if bytes.is_empty() {
        return Err(ContractError::Empty);
    }
    if bytes.len() > limits.document_bytes() {
        return Err(ContractError::BoundExceeded);
    }
    std::str::from_utf8(bytes).map_err(|_| ContractError::InvalidJson)?;

    let parsed = JsonParser::new(bytes, limits).parse_document()?;
    let mut canonical = String::new();
    parsed.write_canonical(&mut canonical)?;
    if canonical.as_bytes() != bytes {
        return Err(ContractError::NonCanonicalJson);
    }

    let value: T = serde_json::from_slice(bytes).map_err(|error| classify_serde_error(&error))?;
    if canonical_json_bytes(&value, limits)?.as_slice() != bytes {
        return Err(ContractError::NonCanonicalJson);
    }
    Ok(value)
}

fn classify_serde_error(error: &serde_json::Error) -> ContractError {
    classify_contract_error_message(&error.to_string())
}

pub(super) fn classify_contract_error_message(message: &str) -> ContractError {
    let known = [
        ContractError::Empty,
        ContractError::LimitOutOfRange,
        ContractError::BoundExceeded,
        ContractError::InvalidString,
        ContractError::InvalidNumber,
        ContractError::InvalidTimestamp,
        ContractError::InvalidSemVer,
        ContractError::InvalidExactRef,
        ContractError::InvalidFingerprint,
        ContractError::InvalidJson,
        ContractError::NonCanonicalJson,
        ContractError::DuplicateMember,
        ContractError::NonCanonicalOrder,
        ContractError::InvalidInvariant,
        ContractError::InvalidArtifactLocator,
        ContractError::FingerprintMismatch,
    ];
    known
        .into_iter()
        .find(|candidate| message.starts_with(&candidate.to_string()))
        .unwrap_or(ContractError::InvalidJson)
}

pub fn sha256_fingerprint(bytes: &[u8]) -> Fingerprint {
    let digest: [u8; 32] = Sha256::digest(bytes).into();
    Fingerprint::from_sha256(digest)
}

pub fn canonical_fingerprint<T: Serialize>(
    value: &T,
    limits: &ContractLimits,
) -> Result<Fingerprint, ContractError> {
    canonical_json_bytes(value, limits).map(|bytes| sha256_fingerprint(&bytes))
}

impl CanonicalValue {
    fn validate(&self, limits: &ContractLimits, depth: usize) -> Result<(), ContractError> {
        match self {
            Self::Null | Self::Bool(_) | Self::Integer(_) => Ok(()),
            Self::String(value) => check_string_bound(value, limits),
            Self::Array(values) => {
                let nested_depth = depth + 1;
                if nested_depth > limits.nesting_depth() || values.len() > limits.array_items() {
                    return Err(ContractError::BoundExceeded);
                }
                for value in values {
                    value.validate(limits, nested_depth)?;
                }
                Ok(())
            }
            Self::Object(members) => {
                let nested_depth = depth + 1;
                if nested_depth > limits.nesting_depth() || members.len() > limits.object_members()
                {
                    return Err(ContractError::BoundExceeded);
                }
                for (index, (key, value)) in members.iter().enumerate() {
                    check_string_bound(key, limits)?;
                    if members[index + 1..]
                        .iter()
                        .any(|(other_key, _)| key == other_key)
                    {
                        return Err(ContractError::InvalidJson);
                    }
                    value.validate(limits, nested_depth)?;
                }
                Ok(())
            }
        }
    }

    fn write_canonical(&self, output: &mut String) -> Result<(), ContractError> {
        match self {
            Self::Null => output.push_str("null"),
            Self::Bool(true) => output.push_str("true"),
            Self::Bool(false) => output.push_str("false"),
            Self::String(value) => write_string(value, output),
            Self::Integer(value) => output.push_str(&value.to_string()),
            Self::Array(values) => {
                output.push('[');
                for (index, value) in values.iter().enumerate() {
                    if index != 0 {
                        output.push(',');
                    }
                    value.write_canonical(output)?;
                }
                output.push(']');
            }
            Self::Object(members) => {
                let mut sorted: Vec<_> = members.iter().collect();
                sorted.sort_by(|(left, _), (right, _)| utf16_cmp(left, right));
                if sorted
                    .windows(2)
                    .any(|pair| pair[0].0.as_str() == pair[1].0.as_str())
                {
                    return Err(ContractError::InvalidJson);
                }
                output.push('{');
                for (index, (key, value)) in sorted.into_iter().enumerate() {
                    if index != 0 {
                        output.push(',');
                    }
                    write_string(key, output);
                    output.push(':');
                    value.write_canonical(output)?;
                }
                output.push('}');
            }
        }
        Ok(())
    }
}

#[derive(Debug)]
struct CaptureError(ContractError);

impl fmt::Display for CaptureError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("contract serialization failed")
    }
}

impl std::error::Error for CaptureError {}

impl serde::ser::Error for CaptureError {
    fn custom<T: fmt::Display>(_message: T) -> Self {
        Self(ContractError::InvalidJson)
    }
}

struct CaptureContext<'a> {
    limits: &'a ContractLimits,
    remaining_document_bytes: Cell<usize>,
}

impl<'a> CaptureContext<'a> {
    fn new(limits: &'a ContractLimits) -> Self {
        Self {
            limits,
            remaining_document_bytes: Cell::new(limits.document_bytes()),
        }
    }

    fn consume(&self, bytes: usize) -> Result<(), CaptureError> {
        let remaining = self.remaining_document_bytes.get();
        let Some(remaining) = remaining.checked_sub(bytes) else {
            return Err(CaptureError(ContractError::BoundExceeded));
        };
        self.remaining_document_bytes.set(remaining);
        Ok(())
    }

    fn ensure(&self, bytes: usize) -> Result<(), CaptureError> {
        if bytes > self.remaining_document_bytes.get() {
            return Err(CaptureError(ContractError::BoundExceeded));
        }
        Ok(())
    }

    fn nested_depth(&self, depth: usize) -> Result<usize, CaptureError> {
        let depth = depth
            .checked_add(1)
            .ok_or(CaptureError(ContractError::BoundExceeded))?;
        if depth > self.limits.nesting_depth() {
            return Err(CaptureError(ContractError::BoundExceeded));
        }
        Ok(depth)
    }

    fn capture_string(&self, value: &str) -> Result<CanonicalValue, CaptureError> {
        if value.len() > self.limits.string_bytes() {
            return Err(CaptureError(ContractError::BoundExceeded));
        }
        let canonical_bytes = canonical_string_bytes(value)?;
        self.consume(canonical_bytes)?;
        let mut captured = String::new();
        captured
            .try_reserve_exact(value.len())
            .map_err(|_| CaptureError(ContractError::BoundExceeded))?;
        captured.push_str(value);
        Ok(CanonicalValue::String(captured))
    }

    fn check_key_before_capture(&self, value: &str) -> Result<(), CaptureError> {
        if value.len() > self.limits.string_bytes() {
            return Err(CaptureError(ContractError::BoundExceeded));
        }
        self.ensure(canonical_string_bytes(value)?)
    }

    fn capture_key(&self, value: &str) -> Result<String, CaptureError> {
        self.check_key_before_capture(value)?;
        self.consume(canonical_string_bytes(value)?)?;
        let mut captured = String::new();
        captured
            .try_reserve_exact(value.len())
            .map_err(|_| CaptureError(ContractError::BoundExceeded))?;
        captured.push_str(value);
        Ok(captured)
    }
}

#[derive(Clone, Copy)]
struct CanonicalSerializer<'a> {
    capture: &'a CaptureContext<'a>,
    depth: usize,
}

impl<'a> CanonicalSerializer<'a> {
    fn new(capture: &'a CaptureContext<'a>, depth: usize) -> Self {
        Self { capture, depth }
    }

    fn scalar(
        self,
        value: CanonicalValue,
        canonical_bytes: usize,
    ) -> Result<CanonicalValue, CaptureError> {
        self.capture.consume(canonical_bytes)?;
        Ok(value)
    }
}

impl<'a> Serializer for CanonicalSerializer<'a> {
    type Ok = CanonicalValue;
    type Error = CaptureError;
    type SerializeSeq = SequenceSerializer<'a>;
    type SerializeTuple = SequenceSerializer<'a>;
    type SerializeTupleStruct = SequenceSerializer<'a>;
    type SerializeTupleVariant = VariantSequenceSerializer<'a>;
    type SerializeMap = ObjectSerializer<'a>;
    type SerializeStruct = ObjectSerializer<'a>;
    type SerializeStructVariant = VariantObjectSerializer<'a>;

    fn serialize_bool(self, value: bool) -> Result<Self::Ok, Self::Error> {
        self.scalar(CanonicalValue::Bool(value), if value { 4 } else { 5 })
    }

    fn serialize_i8(self, value: i8) -> Result<Self::Ok, Self::Error> {
        signed_integer(value as i128, self.capture)
    }

    fn serialize_i16(self, value: i16) -> Result<Self::Ok, Self::Error> {
        signed_integer(value as i128, self.capture)
    }

    fn serialize_i32(self, value: i32) -> Result<Self::Ok, Self::Error> {
        signed_integer(value as i128, self.capture)
    }

    fn serialize_i64(self, value: i64) -> Result<Self::Ok, Self::Error> {
        signed_integer(value as i128, self.capture)
    }

    fn serialize_i128(self, value: i128) -> Result<Self::Ok, Self::Error> {
        signed_integer(value, self.capture)
    }

    fn serialize_u8(self, value: u8) -> Result<Self::Ok, Self::Error> {
        unsigned_integer(value as u128, self.capture)
    }

    fn serialize_u16(self, value: u16) -> Result<Self::Ok, Self::Error> {
        unsigned_integer(value as u128, self.capture)
    }

    fn serialize_u32(self, value: u32) -> Result<Self::Ok, Self::Error> {
        unsigned_integer(value as u128, self.capture)
    }

    fn serialize_u64(self, value: u64) -> Result<Self::Ok, Self::Error> {
        unsigned_integer(value as u128, self.capture)
    }

    fn serialize_u128(self, value: u128) -> Result<Self::Ok, Self::Error> {
        unsigned_integer(value, self.capture)
    }

    fn serialize_f32(self, _value: f32) -> Result<Self::Ok, Self::Error> {
        Err(CaptureError(ContractError::InvalidNumber))
    }

    fn serialize_f64(self, _value: f64) -> Result<Self::Ok, Self::Error> {
        Err(CaptureError(ContractError::InvalidNumber))
    }

    fn serialize_char(self, value: char) -> Result<Self::Ok, Self::Error> {
        self.capture.capture_string(value.encode_utf8(&mut [0; 4]))
    }

    fn serialize_str(self, value: &str) -> Result<Self::Ok, Self::Error> {
        self.capture.capture_string(value)
    }

    fn serialize_bytes(self, value: &[u8]) -> Result<Self::Ok, Self::Error> {
        let mut sequence = SequenceSerializer::new(self.capture, self.depth, Some(value.len()))?;
        for byte in value {
            sequence.push(byte)?;
        }
        Ok(sequence.finish())
    }

    fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
        self.scalar(CanonicalValue::Null, 4)
    }

    fn serialize_some<T: ?Sized + Serialize>(self, value: &T) -> Result<Self::Ok, Self::Error> {
        value.serialize(self)
    }

    fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
        self.scalar(CanonicalValue::Null, 4)
    }

    fn serialize_unit_struct(self, _name: &'static str) -> Result<Self::Ok, Self::Error> {
        self.scalar(CanonicalValue::Null, 4)
    }

    fn serialize_unit_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
    ) -> Result<Self::Ok, Self::Error> {
        self.capture.capture_string(variant)
    }

    fn serialize_newtype_struct<T: ?Sized + Serialize>(
        self,
        _name: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error> {
        value.serialize(self)
    }

    fn serialize_newtype_variant<T: ?Sized + Serialize>(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error> {
        let mut object = ObjectSerializer::new(self.capture, self.depth, Some(1))?;
        object.insert_value(variant, value)?;
        object.finish()
    }

    fn serialize_seq(self, length: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
        SequenceSerializer::new(self.capture, self.depth, length)
    }

    fn serialize_tuple(self, length: usize) -> Result<Self::SerializeTuple, Self::Error> {
        SequenceSerializer::new(self.capture, self.depth, Some(length))
    }

    fn serialize_tuple_struct(
        self,
        _name: &'static str,
        length: usize,
    ) -> Result<Self::SerializeTupleStruct, Self::Error> {
        SequenceSerializer::new(self.capture, self.depth, Some(length))
    }

    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        length: usize,
    ) -> Result<Self::SerializeTupleVariant, Self::Error> {
        VariantSequenceSerializer::new(self.capture, self.depth, variant, length)
    }

    fn serialize_map(self, length: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
        ObjectSerializer::new(self.capture, self.depth, length)
    }

    fn serialize_struct(
        self,
        _name: &'static str,
        length: usize,
    ) -> Result<Self::SerializeStruct, Self::Error> {
        ObjectSerializer::new(self.capture, self.depth, Some(length))
    }

    fn serialize_struct_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        length: usize,
    ) -> Result<Self::SerializeStructVariant, Self::Error> {
        VariantObjectSerializer::new(self.capture, self.depth, variant, length)
    }

    fn collect_str<T: ?Sized + fmt::Display>(self, value: &T) -> Result<Self::Ok, Self::Error> {
        let value = collect_bounded_display(value, self.capture, 0)?;
        self.capture.capture_string(&value)
    }

    fn is_human_readable(&self) -> bool {
        true
    }
}

fn signed_integer(
    value: i128,
    capture: &CaptureContext<'_>,
) -> Result<CanonicalValue, CaptureError> {
    if value < MIN_SAFE_INTEGER as i128 || value > MAX_SAFE_INTEGER as i128 {
        return Err(CaptureError(ContractError::InvalidNumber));
    }
    capture.consume(value.to_string().len())?;
    Ok(CanonicalValue::Integer(value as i64))
}

fn unsigned_integer(
    value: u128,
    capture: &CaptureContext<'_>,
) -> Result<CanonicalValue, CaptureError> {
    if value > MAX_SAFE_INTEGER as u128 {
        return Err(CaptureError(ContractError::InvalidNumber));
    }
    capture.consume(value.to_string().len())?;
    Ok(CanonicalValue::Integer(value as i64))
}

struct SequenceSerializer<'a> {
    capture: &'a CaptureContext<'a>,
    depth: usize,
    values: Vec<CanonicalValue>,
}

impl<'a> SequenceSerializer<'a> {
    fn new(
        capture: &'a CaptureContext<'a>,
        depth: usize,
        length: Option<usize>,
    ) -> Result<Self, CaptureError> {
        let depth = capture.nested_depth(depth)?;
        if length.is_some_and(|length| length > capture.limits.array_items()) {
            return Err(CaptureError(ContractError::BoundExceeded));
        }
        capture.consume(2)?;
        let mut values = Vec::new();
        values
            .try_reserve_exact(length.unwrap_or(0))
            .map_err(|_| CaptureError(ContractError::BoundExceeded))?;
        Ok(Self {
            capture,
            depth,
            values,
        })
    }

    fn push<T: ?Sized + Serialize>(&mut self, value: &T) -> Result<(), CaptureError> {
        if self.values.len() == self.capture.limits.array_items() {
            return Err(CaptureError(ContractError::BoundExceeded));
        }
        self.values
            .try_reserve(1)
            .map_err(|_| CaptureError(ContractError::BoundExceeded))?;
        if !self.values.is_empty() {
            self.capture.consume(1)?;
        }
        let value = value.serialize(CanonicalSerializer::new(self.capture, self.depth))?;
        self.values.push(value);
        Ok(())
    }

    fn finish(self) -> CanonicalValue {
        CanonicalValue::Array(self.values)
    }
}

impl SerializeSeq for SequenceSerializer<'_> {
    type Ok = CanonicalValue;
    type Error = CaptureError;

    fn serialize_element<T: ?Sized + Serialize>(&mut self, value: &T) -> Result<(), Self::Error> {
        self.push(value)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(self.finish())
    }
}

impl SerializeTuple for SequenceSerializer<'_> {
    type Ok = CanonicalValue;
    type Error = CaptureError;

    fn serialize_element<T: ?Sized + Serialize>(&mut self, value: &T) -> Result<(), Self::Error> {
        self.push(value)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(self.finish())
    }
}

impl SerializeTupleStruct for SequenceSerializer<'_> {
    type Ok = CanonicalValue;
    type Error = CaptureError;

    fn serialize_field<T: ?Sized + Serialize>(&mut self, value: &T) -> Result<(), Self::Error> {
        self.push(value)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(self.finish())
    }
}

struct VariantSequenceSerializer<'a> {
    variant: String,
    sequence: SequenceSerializer<'a>,
}

impl<'a> VariantSequenceSerializer<'a> {
    fn new(
        capture: &'a CaptureContext<'a>,
        depth: usize,
        variant: &str,
        length: usize,
    ) -> Result<Self, CaptureError> {
        let object_depth = capture.nested_depth(depth)?;
        capture.consume(2)?;
        let variant = capture.capture_key(variant)?;
        capture.consume(1)?;
        let sequence = SequenceSerializer::new(capture, object_depth, Some(length))?;
        Ok(Self { variant, sequence })
    }
}

impl SerializeTupleVariant for VariantSequenceSerializer<'_> {
    type Ok = CanonicalValue;
    type Error = CaptureError;

    fn serialize_field<T: ?Sized + Serialize>(&mut self, value: &T) -> Result<(), Self::Error> {
        self.sequence.push(value)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(CanonicalValue::Object(vec![(
            self.variant,
            self.sequence.finish(),
        )]))
    }
}

struct ObjectSerializer<'a> {
    capture: &'a CaptureContext<'a>,
    depth: usize,
    members: Vec<(String, CanonicalValue)>,
    pending_key: Option<String>,
}

impl<'a> ObjectSerializer<'a> {
    fn new(
        capture: &'a CaptureContext<'a>,
        depth: usize,
        length: Option<usize>,
    ) -> Result<Self, CaptureError> {
        let depth = capture.nested_depth(depth)?;
        if length.is_some_and(|length| length > capture.limits.object_members()) {
            return Err(CaptureError(ContractError::BoundExceeded));
        }
        capture.consume(2)?;
        let mut members = Vec::new();
        members
            .try_reserve_exact(length.unwrap_or(0))
            .map_err(|_| CaptureError(ContractError::BoundExceeded))?;
        Ok(Self {
            capture,
            depth,
            members,
            pending_key: None,
        })
    }

    fn check_member_available(&self) -> Result<(), CaptureError> {
        if self.members.len() == self.capture.limits.object_members() {
            return Err(CaptureError(ContractError::BoundExceeded));
        }
        Ok(())
    }

    fn insert_captured(&mut self, key: String, value: CanonicalValue) -> Result<(), CaptureError> {
        if self
            .members
            .iter()
            .any(|(existing_key, _)| existing_key == &key)
        {
            return Err(CaptureError(ContractError::InvalidJson));
        }
        self.members
            .try_reserve(1)
            .map_err(|_| CaptureError(ContractError::BoundExceeded))?;
        self.members.push((key, value));
        Ok(())
    }

    fn insert_value<T: ?Sized + Serialize>(
        &mut self,
        key: &str,
        value: &T,
    ) -> Result<(), CaptureError> {
        self.check_member_available()?;
        if self
            .members
            .iter()
            .any(|(existing_key, _)| existing_key == key)
        {
            return Err(CaptureError(ContractError::InvalidJson));
        }
        let overhead = usize::from(!self.members.is_empty()) + 1;
        let key_bytes = canonical_string_bytes(key)?;
        self.capture.ensure(
            overhead
                .checked_add(key_bytes)
                .ok_or(CaptureError(ContractError::BoundExceeded))?,
        )?;
        let key = self.capture.capture_key(key)?;
        self.capture.consume(overhead)?;
        let value = value.serialize(CanonicalSerializer::new(self.capture, self.depth))?;
        self.insert_captured(key, value)
    }

    fn finish(self) -> Result<CanonicalValue, CaptureError> {
        if self.pending_key.is_some() {
            return Err(CaptureError(ContractError::InvalidJson));
        }
        Ok(CanonicalValue::Object(self.members))
    }
}

impl SerializeMap for ObjectSerializer<'_> {
    type Ok = CanonicalValue;
    type Error = CaptureError;

    fn serialize_key<T: ?Sized + Serialize>(&mut self, key: &T) -> Result<(), Self::Error> {
        if self.pending_key.is_some() {
            return Err(CaptureError(ContractError::InvalidJson));
        }
        self.check_member_available()?;
        let overhead = usize::from(!self.members.is_empty()) + 1;
        let key = key.serialize(KeySerializer {
            capture: self.capture,
            reserved_overhead: overhead,
        })?;
        if self
            .members
            .iter()
            .any(|(existing_key, _)| existing_key == &key)
        {
            return Err(CaptureError(ContractError::InvalidJson));
        }
        self.pending_key = Some(key);
        Ok(())
    }

    fn serialize_value<T: ?Sized + Serialize>(&mut self, value: &T) -> Result<(), Self::Error> {
        let key = self
            .pending_key
            .take()
            .ok_or(CaptureError(ContractError::InvalidJson))?;
        let overhead = usize::from(!self.members.is_empty()) + 1;
        self.capture
            .consume(overhead + canonical_string_bytes(&key)?)?;
        let value = value.serialize(CanonicalSerializer::new(self.capture, self.depth))?;
        self.insert_captured(key, value)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        self.finish()
    }
}

impl SerializeStruct for ObjectSerializer<'_> {
    type Ok = CanonicalValue;
    type Error = CaptureError;

    fn serialize_field<T: ?Sized + Serialize>(
        &mut self,
        key: &'static str,
        value: &T,
    ) -> Result<(), Self::Error> {
        self.insert_value(key, value)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        self.finish()
    }
}

struct VariantObjectSerializer<'a> {
    variant: String,
    object: ObjectSerializer<'a>,
}

impl<'a> VariantObjectSerializer<'a> {
    fn new(
        capture: &'a CaptureContext<'a>,
        depth: usize,
        variant: &str,
        length: usize,
    ) -> Result<Self, CaptureError> {
        let outer_depth = capture.nested_depth(depth)?;
        capture.consume(2)?;
        let variant = capture.capture_key(variant)?;
        capture.consume(1)?;
        let object = ObjectSerializer::new(capture, outer_depth, Some(length))?;
        Ok(Self { variant, object })
    }
}

impl SerializeStructVariant for VariantObjectSerializer<'_> {
    type Ok = CanonicalValue;
    type Error = CaptureError;

    fn serialize_field<T: ?Sized + Serialize>(
        &mut self,
        key: &'static str,
        value: &T,
    ) -> Result<(), Self::Error> {
        self.object.insert_value(key, value)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(CanonicalValue::Object(vec![(
            self.variant,
            self.object.finish()?,
        )]))
    }
}

#[derive(Clone, Copy)]
struct KeySerializer<'a> {
    capture: &'a CaptureContext<'a>,
    reserved_overhead: usize,
}

impl KeySerializer<'_> {
    fn check(&self, value: &str) -> Result<(), CaptureError> {
        self.capture.check_key_before_capture(value)?;
        let required = canonical_string_bytes(value)?
            .checked_add(self.reserved_overhead)
            .ok_or(CaptureError(ContractError::BoundExceeded))?;
        self.capture.ensure(required)
    }

    fn capture_str(self, value: &str) -> Result<String, CaptureError> {
        self.check(value)?;
        let mut captured = String::new();
        captured
            .try_reserve_exact(value.len())
            .map_err(|_| CaptureError(ContractError::BoundExceeded))?;
        captured.push_str(value);
        Ok(captured)
    }

    fn capture_owned(self, value: String) -> Result<String, CaptureError> {
        self.check(&value)?;
        Ok(value)
    }
}

impl Serializer for KeySerializer<'_> {
    type Ok = String;
    type Error = CaptureError;
    type SerializeSeq = serde::ser::Impossible<String, CaptureError>;
    type SerializeTuple = serde::ser::Impossible<String, CaptureError>;
    type SerializeTupleStruct = serde::ser::Impossible<String, CaptureError>;
    type SerializeTupleVariant = serde::ser::Impossible<String, CaptureError>;
    type SerializeMap = serde::ser::Impossible<String, CaptureError>;
    type SerializeStruct = serde::ser::Impossible<String, CaptureError>;
    type SerializeStructVariant = serde::ser::Impossible<String, CaptureError>;

    fn serialize_bool(self, value: bool) -> Result<Self::Ok, Self::Error> {
        self.capture_owned(value.to_string())
    }

    fn serialize_i8(self, value: i8) -> Result<Self::Ok, Self::Error> {
        self.capture_owned(value.to_string())
    }

    fn serialize_i16(self, value: i16) -> Result<Self::Ok, Self::Error> {
        self.capture_owned(value.to_string())
    }

    fn serialize_i32(self, value: i32) -> Result<Self::Ok, Self::Error> {
        self.capture_owned(value.to_string())
    }

    fn serialize_i64(self, value: i64) -> Result<Self::Ok, Self::Error> {
        self.capture_owned(value.to_string())
    }

    fn serialize_i128(self, value: i128) -> Result<Self::Ok, Self::Error> {
        self.capture_owned(value.to_string())
    }

    fn serialize_u8(self, value: u8) -> Result<Self::Ok, Self::Error> {
        self.capture_owned(value.to_string())
    }

    fn serialize_u16(self, value: u16) -> Result<Self::Ok, Self::Error> {
        self.capture_owned(value.to_string())
    }

    fn serialize_u32(self, value: u32) -> Result<Self::Ok, Self::Error> {
        self.capture_owned(value.to_string())
    }

    fn serialize_u64(self, value: u64) -> Result<Self::Ok, Self::Error> {
        self.capture_owned(value.to_string())
    }

    fn serialize_u128(self, value: u128) -> Result<Self::Ok, Self::Error> {
        self.capture_owned(value.to_string())
    }

    fn serialize_f32(self, _value: f32) -> Result<Self::Ok, Self::Error> {
        Err(CaptureError(ContractError::InvalidJson))
    }

    fn serialize_f64(self, _value: f64) -> Result<Self::Ok, Self::Error> {
        Err(CaptureError(ContractError::InvalidJson))
    }

    fn serialize_char(self, value: char) -> Result<Self::Ok, Self::Error> {
        self.capture_owned(value.to_string())
    }

    fn serialize_str(self, value: &str) -> Result<Self::Ok, Self::Error> {
        self.capture_str(value)
    }

    fn serialize_bytes(self, _value: &[u8]) -> Result<Self::Ok, Self::Error> {
        Err(CaptureError(ContractError::InvalidJson))
    }

    fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
        Err(CaptureError(ContractError::InvalidJson))
    }

    fn serialize_some<T: ?Sized + Serialize>(self, value: &T) -> Result<Self::Ok, Self::Error> {
        value.serialize(self)
    }

    fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
        Err(CaptureError(ContractError::InvalidJson))
    }

    fn serialize_unit_struct(self, _name: &'static str) -> Result<Self::Ok, Self::Error> {
        Err(CaptureError(ContractError::InvalidJson))
    }

    fn serialize_unit_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
    ) -> Result<Self::Ok, Self::Error> {
        self.capture_str(variant)
    }

    fn serialize_newtype_struct<T: ?Sized + Serialize>(
        self,
        _name: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error> {
        value.serialize(self)
    }

    fn serialize_newtype_variant<T: ?Sized + Serialize>(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _value: &T,
    ) -> Result<Self::Ok, Self::Error> {
        Err(CaptureError(ContractError::InvalidJson))
    }

    fn serialize_seq(self, _length: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
        Err(CaptureError(ContractError::InvalidJson))
    }

    fn serialize_tuple(self, _length: usize) -> Result<Self::SerializeTuple, Self::Error> {
        Err(CaptureError(ContractError::InvalidJson))
    }

    fn serialize_tuple_struct(
        self,
        _name: &'static str,
        _length: usize,
    ) -> Result<Self::SerializeTupleStruct, Self::Error> {
        Err(CaptureError(ContractError::InvalidJson))
    }

    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _length: usize,
    ) -> Result<Self::SerializeTupleVariant, Self::Error> {
        Err(CaptureError(ContractError::InvalidJson))
    }

    fn serialize_map(self, _length: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
        Err(CaptureError(ContractError::InvalidJson))
    }

    fn serialize_struct(
        self,
        _name: &'static str,
        _length: usize,
    ) -> Result<Self::SerializeStruct, Self::Error> {
        Err(CaptureError(ContractError::InvalidJson))
    }

    fn serialize_struct_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _length: usize,
    ) -> Result<Self::SerializeStructVariant, Self::Error> {
        Err(CaptureError(ContractError::InvalidJson))
    }

    fn collect_str<T: ?Sized + fmt::Display>(self, value: &T) -> Result<Self::Ok, Self::Error> {
        let value = collect_bounded_display(value, self.capture, self.reserved_overhead)?;
        self.capture_owned(value)
    }

    fn is_human_readable(&self) -> bool {
        true
    }
}

fn canonical_string_bytes(value: &str) -> Result<usize, CaptureError> {
    value.chars().try_fold(2_usize, |length, character| {
        let bytes = match character {
            '"' | '\\' | '\u{0008}' | '\u{0009}' | '\u{000a}' | '\u{000c}' | '\u{000d}' => 2,
            '\u{0000}'..='\u{001f}' => 6,
            _ => character.len_utf8(),
        };
        length
            .checked_add(bytes)
            .ok_or(CaptureError(ContractError::BoundExceeded))
    })
}

fn collect_bounded_display<T: ?Sized + fmt::Display>(
    value: &T,
    capture: &CaptureContext<'_>,
    reserved_overhead: usize,
) -> Result<String, CaptureError> {
    struct BoundedWriter {
        value: String,
        limit: usize,
        exceeded: bool,
    }

    impl fmt::Write for BoundedWriter {
        fn write_str(&mut self, value: &str) -> fmt::Result {
            let Some(length) = self.value.len().checked_add(value.len()) else {
                self.exceeded = true;
                return Err(fmt::Error);
            };
            if length > self.limit || self.value.try_reserve(value.len()).is_err() {
                self.exceeded = true;
                return Err(fmt::Error);
            }
            self.value.push_str(value);
            Ok(())
        }
    }

    let document_limit = capture
        .remaining_document_bytes
        .get()
        .checked_sub(reserved_overhead)
        .ok_or(CaptureError(ContractError::BoundExceeded))?;
    let mut writer = BoundedWriter {
        value: String::new(),
        limit: capture.limits.string_bytes().min(document_limit),
        exceeded: false,
    };
    if write!(&mut writer, "{value}").is_err() {
        return Err(CaptureError(if writer.exceeded {
            ContractError::BoundExceeded
        } else {
            ContractError::InvalidJson
        }));
    }
    Ok(writer.value)
}

fn check_string_bound(value: &str, limits: &ContractLimits) -> Result<(), ContractError> {
    if value.len() > limits.string_bytes() {
        return Err(ContractError::BoundExceeded);
    }
    Ok(())
}

fn utf16_cmp(left: &str, right: &str) -> Ordering {
    left.encode_utf16().cmp(right.encode_utf16())
}

fn write_string(value: &str, output: &mut String) {
    const HEX: &[u8; 16] = b"0123456789abcdef";
    output.push('"');
    for character in value.chars() {
        match character {
            '"' => output.push_str("\\\""),
            '\\' => output.push_str("\\\\"),
            '\u{0008}' => output.push_str("\\b"),
            '\u{0009}' => output.push_str("\\t"),
            '\u{000a}' => output.push_str("\\n"),
            '\u{000c}' => output.push_str("\\f"),
            '\u{000d}' => output.push_str("\\r"),
            '\u{0000}'..='\u{001f}' => {
                let byte = character as u8;
                output.push_str("\\u00");
                output.push(HEX[(byte >> 4) as usize] as char);
                output.push(HEX[(byte & 0x0f) as usize] as char);
            }
            _ => output.push(character),
        }
    }
    output.push('"');
}

struct JsonParser<'a> {
    bytes: &'a [u8],
    position: usize,
    limits: &'a ContractLimits,
}

impl<'a> JsonParser<'a> {
    fn new(bytes: &'a [u8], limits: &'a ContractLimits) -> Self {
        Self {
            bytes,
            position: 0,
            limits,
        }
    }

    fn parse_document(mut self) -> Result<CanonicalValue, ContractError> {
        self.skip_whitespace();
        if self.position == self.bytes.len() {
            return Err(ContractError::InvalidJson);
        }
        let value = self.parse_value(0)?;
        self.skip_whitespace();
        if self.position != self.bytes.len() {
            return Err(ContractError::InvalidJson);
        }
        value.validate(self.limits, 0)?;
        Ok(value)
    }

    fn parse_value(&mut self, depth: usize) -> Result<CanonicalValue, ContractError> {
        self.skip_whitespace();
        match self.peek() {
            Some(b'n') => {
                self.consume_literal(b"null")?;
                Ok(CanonicalValue::Null)
            }
            Some(b't') => {
                self.consume_literal(b"true")?;
                Ok(CanonicalValue::Bool(true))
            }
            Some(b'f') => {
                self.consume_literal(b"false")?;
                Ok(CanonicalValue::Bool(false))
            }
            Some(b'"') => self.parse_string().map(CanonicalValue::String),
            Some(b'[') => self.parse_array(depth),
            Some(b'{') => self.parse_object(depth),
            Some(b'-' | b'0'..=b'9') => self.parse_number().map(CanonicalValue::Integer),
            _ => Err(ContractError::InvalidJson),
        }
    }

    fn parse_array(&mut self, depth: usize) -> Result<CanonicalValue, ContractError> {
        let nested_depth = depth + 1;
        if nested_depth > self.limits.nesting_depth() {
            return Err(ContractError::BoundExceeded);
        }
        self.position += 1;
        self.skip_whitespace();
        let mut values = Vec::new();
        if self.take(b']') {
            return Ok(CanonicalValue::Array(values));
        }
        loop {
            if values.len() == self.limits.array_items() {
                return Err(ContractError::BoundExceeded);
            }
            values.push(self.parse_value(nested_depth)?);
            self.skip_whitespace();
            if self.take(b']') {
                break;
            }
            if !self.take(b',') {
                return Err(ContractError::InvalidJson);
            }
        }
        Ok(CanonicalValue::Array(values))
    }

    fn parse_object(&mut self, depth: usize) -> Result<CanonicalValue, ContractError> {
        let nested_depth = depth + 1;
        if nested_depth > self.limits.nesting_depth() {
            return Err(ContractError::BoundExceeded);
        }
        self.position += 1;
        self.skip_whitespace();
        let mut members: Vec<(String, CanonicalValue)> = Vec::new();
        if self.take(b'}') {
            return Ok(CanonicalValue::Object(members));
        }
        loop {
            if members.len() == self.limits.object_members() {
                return Err(ContractError::BoundExceeded);
            }
            self.skip_whitespace();
            if self.peek() != Some(b'"') {
                return Err(ContractError::InvalidJson);
            }
            let key = self.parse_string()?;
            if members.iter().any(|(existing, _)| existing == &key) {
                return Err(ContractError::InvalidJson);
            }
            self.skip_whitespace();
            if !self.take(b':') {
                return Err(ContractError::InvalidJson);
            }
            let value = self.parse_value(nested_depth)?;
            members.push((key, value));
            self.skip_whitespace();
            if self.take(b'}') {
                break;
            }
            if !self.take(b',') {
                return Err(ContractError::InvalidJson);
            }
        }
        Ok(CanonicalValue::Object(members))
    }

    fn parse_string(&mut self) -> Result<String, ContractError> {
        let start = self.position;
        self.position += 1;
        while let Some(byte) = self.peek() {
            match byte {
                b'"' => {
                    self.position += 1;
                    let value: String = serde_json::from_slice(&self.bytes[start..self.position])
                        .map_err(|_| ContractError::InvalidJson)?;
                    check_string_bound(&value, self.limits)?;
                    return Ok(value);
                }
                b'\\' => {
                    self.position += 1;
                    if self.peek().is_none() {
                        return Err(ContractError::InvalidJson);
                    }
                    self.position += 1;
                }
                _ => self.position += 1,
            }
        }
        Err(ContractError::InvalidJson)
    }

    fn parse_number(&mut self) -> Result<i64, ContractError> {
        let start = self.position;
        let negative = self.take(b'-');
        match self.peek() {
            Some(b'0') => {
                self.position += 1;
                if self.peek().is_some_and(|byte| byte.is_ascii_digit()) {
                    return Err(ContractError::InvalidJson);
                }
            }
            Some(b'1'..=b'9') => {
                self.position += 1;
                while self.peek().is_some_and(|byte| byte.is_ascii_digit()) {
                    self.position += 1;
                }
            }
            _ => return Err(ContractError::InvalidJson),
        }

        let mut unsupported = false;
        if self.take(b'.') {
            unsupported = true;
            if !self.peek().is_some_and(|byte| byte.is_ascii_digit()) {
                return Err(ContractError::InvalidJson);
            }
            while self.peek().is_some_and(|byte| byte.is_ascii_digit()) {
                self.position += 1;
            }
        }
        if self.peek().is_some_and(|byte| matches!(byte, b'e' | b'E')) {
            unsupported = true;
            self.position += 1;
            if self.peek().is_some_and(|byte| matches!(byte, b'+' | b'-')) {
                self.position += 1;
            }
            if !self.peek().is_some_and(|byte| byte.is_ascii_digit()) {
                return Err(ContractError::InvalidJson);
            }
            while self.peek().is_some_and(|byte| byte.is_ascii_digit()) {
                self.position += 1;
            }
        }
        let token = &self.bytes[start..self.position];
        if unsupported || (negative && token == b"-0") {
            return Err(ContractError::InvalidNumber);
        }
        let token = std::str::from_utf8(token).map_err(|_| ContractError::InvalidJson)?;
        let value: i128 = token.parse().map_err(|_| ContractError::InvalidNumber)?;
        if value < MIN_SAFE_INTEGER as i128 || value > MAX_SAFE_INTEGER as i128 {
            return Err(ContractError::InvalidNumber);
        }
        Ok(value as i64)
    }

    fn consume_literal(&mut self, literal: &[u8]) -> Result<(), ContractError> {
        if self.bytes[self.position..].starts_with(literal) {
            self.position += literal.len();
            Ok(())
        } else {
            Err(ContractError::InvalidJson)
        }
    }

    fn skip_whitespace(&mut self) {
        while self
            .peek()
            .is_some_and(|byte| matches!(byte, b' ' | b'\n' | b'\r' | b'\t'))
        {
            self.position += 1;
        }
    }

    fn peek(&self) -> Option<u8> {
        self.bytes.get(self.position).copied()
    }

    fn take(&mut self, expected: u8) -> bool {
        if self.peek() == Some(expected) {
            self.position += 1;
            true
        } else {
            false
        }
    }
}
