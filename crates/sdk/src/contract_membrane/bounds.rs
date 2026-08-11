use super::ContractError;
use serde::de::{Error as _, MapAccess, SeqAccess, Visitor};
use serde::ser::{SerializeMap, SerializeSeq};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::cmp::Ordering;
use std::fmt;
use std::marker::PhantomData;
use time::{Date, Month};

const MAX_STRING_BYTES: usize = 65_536;
const MAX_ITEMS: usize = 128;
const MAX_DOCUMENT_BYTES: usize = 1_048_576;
const MAX_NESTING_DEPTH: usize = 32;
const MAX_SAFE_INTEGER: u64 = 9_007_199_254_740_991;
const MIN_SAFE_INTEGER: i64 = -9_007_199_254_740_991;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BoundedString<const N: usize>(String);

impl<const N: usize> BoundedString<N> {
    pub fn new(value: String) -> Result<Self, ContractError> {
        if N > MAX_STRING_BYTES {
            return Err(ContractError::LimitOutOfRange);
        }
        if value.is_empty() {
            return Err(ContractError::Empty);
        }
        if value.len() > N {
            return Err(ContractError::BoundExceeded);
        }
        Ok(Self(value))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }

    pub fn into_string(self) -> String {
        self.0
    }
}

impl<const N: usize> Serialize for BoundedString<N> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.0)
    }
}

impl<'de, const N: usize> Deserialize<'de> for BoundedString<N> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        if N > MAX_STRING_BYTES {
            return Err(D::Error::custom(ContractError::LimitOutOfRange));
        }

        struct StringVisitor<const N: usize>;

        impl<const N: usize> Visitor<'_> for StringVisitor<N> {
            type Value = BoundedString<N>;

            fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                formatter.write_str("a bounded non-empty string")
            }

            fn visit_borrowed_str<E>(self, value: &str) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                BoundedString::new(value.to_owned()).map_err(E::custom)
            }

            fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                self.visit_borrowed_str(value)
            }

            fn visit_string<E>(self, value: String) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                BoundedString::new(value).map_err(E::custom)
            }
        }

        deserializer
            .deserialize_string(StringVisitor::<N>)
            .map_err(sanitize_deserialize_error)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BoundedVec<T, const N: usize>(Vec<T>);

impl<T, const N: usize> BoundedVec<T, N> {
    pub fn new(value: Vec<T>) -> Result<Self, ContractError> {
        if N > MAX_ITEMS {
            return Err(ContractError::LimitOutOfRange);
        }
        if value.len() > N {
            return Err(ContractError::BoundExceeded);
        }
        Ok(Self(value))
    }

    pub fn as_slice(&self) -> &[T] {
        &self.0
    }

    pub fn into_vec(self) -> Vec<T> {
        self.0
    }
}

impl<T: Serialize, const N: usize> Serialize for BoundedVec<T, N> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut sequence = serializer.serialize_seq(Some(self.0.len()))?;
        for item in &self.0 {
            sequence.serialize_element(item)?;
        }
        sequence.end()
    }
}

impl<'de, T: Deserialize<'de>, const N: usize> Deserialize<'de> for BoundedVec<T, N> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        if N > MAX_ITEMS {
            return Err(D::Error::custom(ContractError::LimitOutOfRange));
        }

        struct VecVisitor<T, const N: usize>(PhantomData<T>);

        impl<'de, T: Deserialize<'de>, const N: usize> Visitor<'de> for VecVisitor<T, N> {
            type Value = BoundedVec<T, N>;

            fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                formatter.write_str("a bounded sequence")
            }

            fn visit_seq<A>(self, mut sequence: A) -> Result<Self::Value, A::Error>
            where
                A: SeqAccess<'de>,
            {
                if sequence.size_hint().is_some_and(|size| size > N) {
                    return Err(A::Error::custom(ContractError::BoundExceeded));
                }
                let capacity = sequence.size_hint().unwrap_or(0).min(N);
                let mut values = Vec::new();
                values
                    .try_reserve_exact(capacity)
                    .map_err(|_| A::Error::custom(ContractError::BoundExceeded))?;
                while let Some(item) = sequence.next_element()? {
                    if values.len() == N {
                        return Err(A::Error::custom(ContractError::BoundExceeded));
                    }
                    values.push(item);
                }
                Ok(BoundedVec(values))
            }
        }

        deserializer
            .deserialize_seq(VecVisitor::<T, N>(PhantomData))
            .map_err(sanitize_deserialize_error)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SafeU64(u64);

impl SafeU64 {
    pub fn new(value: u64) -> Result<Self, ContractError> {
        if value > MAX_SAFE_INTEGER {
            return Err(ContractError::InvalidNumber);
        }
        Ok(Self(value))
    }

    pub fn get(self) -> u64 {
        self.0
    }
}

impl Serialize for SafeU64 {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_u64(self.0)
    }
}

impl<'de> Deserialize<'de> for SafeU64 {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SafeU64Visitor;

        impl Visitor<'_> for SafeU64Visitor {
            type Value = SafeU64;

            fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                formatter.write_str("a non-negative safe JSON integer")
            }

            fn visit_u64<E>(self, value: u64) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                SafeU64::new(value).map_err(E::custom)
            }

            fn visit_i64<E>(self, value: i64) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                u64::try_from(value)
                    .ok()
                    .and_then(|value| SafeU64::new(value).ok())
                    .ok_or_else(|| E::custom(ContractError::InvalidNumber))
            }

            fn visit_f64<E>(self, _value: f64) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Err(E::custom(ContractError::InvalidNumber))
            }
        }

        deserializer
            .deserialize_any(SafeU64Visitor)
            .map_err(sanitize_deserialize_error)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SafeI64(i64);

impl SafeI64 {
    pub fn new(value: i64) -> Result<Self, ContractError> {
        if !(MIN_SAFE_INTEGER..=MAX_SAFE_INTEGER as i64).contains(&value) {
            return Err(ContractError::InvalidNumber);
        }
        Ok(Self(value))
    }

    pub fn get(self) -> i64 {
        self.0
    }
}

impl Serialize for SafeI64 {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_i64(self.0)
    }
}

impl<'de> Deserialize<'de> for SafeI64 {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SafeI64Visitor;

        impl Visitor<'_> for SafeI64Visitor {
            type Value = SafeI64;

            fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                formatter.write_str("a safe JSON integer")
            }

            fn visit_i64<E>(self, value: i64) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                SafeI64::new(value).map_err(E::custom)
            }

            fn visit_u64<E>(self, value: u64) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                i64::try_from(value)
                    .ok()
                    .and_then(|value| SafeI64::new(value).ok())
                    .ok_or_else(|| E::custom(ContractError::InvalidNumber))
            }

            fn visit_f64<E>(self, _value: f64) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Err(E::custom(ContractError::InvalidNumber))
            }
        }

        deserializer
            .deserialize_any(SafeI64Visitor)
            .map_err(sanitize_deserialize_error)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UtcTimestamp(String);

impl UtcTimestamp {
    pub fn parse(value: &str) -> Result<Self, ContractError> {
        let bytes = value.as_bytes();
        if bytes.len() != 20
            || bytes[4] != b'-'
            || bytes[7] != b'-'
            || bytes[10] != b'T'
            || bytes[13] != b':'
            || bytes[16] != b':'
            || bytes[19] != b'Z'
            || bytes.iter().enumerate().any(|(index, byte)| {
                !matches!(index, 4 | 7 | 10 | 13 | 16 | 19) && !byte.is_ascii_digit()
            })
        {
            return Err(ContractError::InvalidTimestamp);
        }

        let number = |start: usize, end: usize| -> Result<u16, ContractError> {
            value[start..end]
                .parse()
                .map_err(|_| ContractError::InvalidTimestamp)
        };
        let year = number(0, 4)?;
        let month = number(5, 7)?;
        let day = number(8, 10)?;
        let hour = number(11, 13)?;
        let minute = number(14, 16)?;
        let second = number(17, 19)?;
        if year == 0 || hour > 23 || minute > 59 || second > 59 {
            return Err(ContractError::InvalidTimestamp);
        }
        let month = Month::try_from(month as u8).map_err(|_| ContractError::InvalidTimestamp)?;
        Date::from_calendar_date(year as i32, month, day as u8)
            .map_err(|_| ContractError::InvalidTimestamp)?;
        Ok(Self(value.to_owned()))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl Serialize for UtcTimestamp {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.0)
    }
}

impl<'de> Deserialize<'de> for UtcTimestamp {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = String::deserialize(deserializer).map_err(sanitize_deserialize_error)?;
        Self::parse(&value).map_err(D::Error::custom)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ContractLimits {
    document_bytes: usize,
    string_bytes: usize,
    array_items: usize,
    object_members: usize,
    nesting_depth: usize,
}

impl ContractLimits {
    pub fn new(
        document_bytes: usize,
        string_bytes: usize,
        array_items: usize,
        object_members: usize,
        nesting_depth: usize,
    ) -> Result<Self, ContractError> {
        if document_bytes == 0
            || document_bytes > MAX_DOCUMENT_BYTES
            || string_bytes == 0
            || string_bytes > MAX_STRING_BYTES
            || array_items == 0
            || array_items > MAX_ITEMS
            || object_members == 0
            || object_members > MAX_ITEMS
            || nesting_depth == 0
            || nesting_depth > MAX_NESTING_DEPTH
        {
            return Err(ContractError::LimitOutOfRange);
        }
        Ok(Self {
            document_bytes,
            string_bytes,
            array_items,
            object_members,
            nesting_depth,
        })
    }

    pub fn packet0() -> Self {
        Self {
            document_bytes: MAX_DOCUMENT_BYTES,
            string_bytes: MAX_STRING_BYTES,
            array_items: MAX_ITEMS,
            object_members: MAX_ITEMS,
            nesting_depth: MAX_NESTING_DEPTH,
        }
    }

    pub fn document_bytes(&self) -> usize {
        self.document_bytes
    }

    pub fn string_bytes(&self) -> usize {
        self.string_bytes
    }

    pub fn array_items(&self) -> usize {
        self.array_items
    }

    pub fn object_members(&self) -> usize {
        self.object_members
    }

    pub fn nesting_depth(&self) -> usize {
        self.nesting_depth
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ContractMember {
    key: BoundedString<255>,
    value: ContractValue,
}

impl ContractMember {
    pub fn new(key: BoundedString<255>, value: ContractValue) -> Self {
        Self { key, value }
    }

    pub fn key(&self) -> &BoundedString<255> {
        &self.key
    }

    pub fn value(&self) -> &ContractValue {
        &self.value
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ContractValue(ContractValueKind);

#[derive(Clone, Debug, Eq, PartialEq)]
enum ContractValueKind {
    Null,
    Bool(bool),
    String(BoundedString<65_536>),
    Integer(SafeI64),
    Array(BoundedVec<ContractValue, 128>),
    Object(BoundedVec<ContractMember, 128>),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ContractValueRef<'a> {
    Null,
    Bool(bool),
    String(&'a str),
    Integer(i64),
    Array(&'a [ContractValue]),
    Object(&'a [ContractMember]),
}

impl ContractValue {
    pub fn null() -> Self {
        Self(ContractValueKind::Null)
    }

    pub fn boolean(value: bool) -> Self {
        Self(ContractValueKind::Bool(value))
    }

    pub fn string(value: BoundedString<65_536>) -> Self {
        Self(ContractValueKind::String(value))
    }

    pub fn integer(value: SafeI64) -> Self {
        Self(ContractValueKind::Integer(value))
    }

    pub fn array(values: Vec<ContractValue>) -> Result<Self, ContractError> {
        let value = Self(ContractValueKind::Array(BoundedVec::new(values)?));
        value.validate_hard_depth()?;
        Ok(value)
    }

    pub fn object(members: Vec<ContractMember>) -> Result<Self, ContractError> {
        let members = BoundedVec::new(members)?;
        for (index, member) in members.as_slice().iter().enumerate() {
            if members.as_slice()[index + 1..]
                .iter()
                .any(|other| member.key == other.key)
            {
                return Err(ContractError::DuplicateMember);
            }
        }
        if members
            .as_slice()
            .windows(2)
            .any(|pair| utf16_cmp(pair[0].key.as_str(), pair[1].key.as_str()) != Ordering::Less)
        {
            return Err(ContractError::NonCanonicalOrder);
        }
        let value = Self(ContractValueKind::Object(members));
        value.validate_hard_depth()?;
        Ok(value)
    }

    pub fn as_ref(&self) -> ContractValueRef<'_> {
        match &self.0 {
            ContractValueKind::Null => ContractValueRef::Null,
            ContractValueKind::Bool(value) => ContractValueRef::Bool(*value),
            ContractValueKind::String(value) => ContractValueRef::String(value.as_str()),
            ContractValueKind::Integer(value) => ContractValueRef::Integer(value.get()),
            ContractValueKind::Array(value) => ContractValueRef::Array(value.as_slice()),
            ContractValueKind::Object(value) => ContractValueRef::Object(value.as_slice()),
        }
    }

    fn validate_hard_depth(&self) -> Result<(), ContractError> {
        if self.nesting_depth() > MAX_NESTING_DEPTH {
            return Err(ContractError::BoundExceeded);
        }
        Ok(())
    }

    fn nesting_depth(&self) -> usize {
        match &self.0 {
            ContractValueKind::Array(values) => {
                1 + values
                    .as_slice()
                    .iter()
                    .map(Self::nesting_depth)
                    .max()
                    .unwrap_or(0)
            }
            ContractValueKind::Object(members) => {
                1 + members
                    .as_slice()
                    .iter()
                    .map(|member| member.value.nesting_depth())
                    .max()
                    .unwrap_or(0)
            }
            _ => 0,
        }
    }
}

impl Serialize for ContractValue {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match &self.0 {
            ContractValueKind::Null => serializer.serialize_unit(),
            ContractValueKind::Bool(value) => serializer.serialize_bool(*value),
            ContractValueKind::String(value) => serializer.serialize_str(value.as_str()),
            ContractValueKind::Integer(value) => serializer.serialize_i64(value.get()),
            ContractValueKind::Array(values) => {
                let mut sequence = serializer.serialize_seq(Some(values.as_slice().len()))?;
                for value in values.as_slice() {
                    sequence.serialize_element(value)?;
                }
                sequence.end()
            }
            ContractValueKind::Object(members) => {
                let mut map = serializer.serialize_map(Some(members.as_slice().len()))?;
                for member in members.as_slice() {
                    map.serialize_entry(member.key.as_str(), &member.value)?;
                }
                map.end()
            }
        }
    }
}

impl<'de> Deserialize<'de> for ContractValue {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ContractValueVisitor;

        impl<'de> Visitor<'de> for ContractValueVisitor {
            type Value = ContractValue;

            fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                formatter.write_str("a bounded contract JSON value")
            }

            fn visit_unit<E>(self) -> Result<Self::Value, E> {
                Ok(ContractValue::null())
            }

            fn visit_none<E>(self) -> Result<Self::Value, E> {
                Ok(ContractValue::null())
            }

            fn visit_bool<E>(self, value: bool) -> Result<Self::Value, E> {
                Ok(ContractValue::boolean(value))
            }

            fn visit_i64<E>(self, value: i64) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                SafeI64::new(value)
                    .map(ContractValue::integer)
                    .map_err(E::custom)
            }

            fn visit_u64<E>(self, value: u64) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                i64::try_from(value)
                    .ok()
                    .and_then(|value| SafeI64::new(value).ok())
                    .map(ContractValue::integer)
                    .ok_or_else(|| E::custom(ContractError::InvalidNumber))
            }

            fn visit_f64<E>(self, _value: f64) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Err(E::custom(ContractError::InvalidNumber))
            }

            fn visit_borrowed_str<E>(self, value: &'de str) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                BoundedString::new(value.to_owned())
                    .map(ContractValue::string)
                    .map_err(E::custom)
            }

            fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                BoundedString::new(value.to_owned())
                    .map(ContractValue::string)
                    .map_err(E::custom)
            }

            fn visit_string<E>(self, value: String) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                BoundedString::new(value)
                    .map(ContractValue::string)
                    .map_err(E::custom)
            }

            fn visit_seq<A>(self, mut sequence: A) -> Result<Self::Value, A::Error>
            where
                A: SeqAccess<'de>,
            {
                if sequence.size_hint().is_some_and(|size| size > MAX_ITEMS) {
                    return Err(A::Error::custom(ContractError::BoundExceeded));
                }
                let capacity = sequence.size_hint().unwrap_or(0).min(MAX_ITEMS);
                let mut values = Vec::new();
                values
                    .try_reserve_exact(capacity)
                    .map_err(|_| A::Error::custom(ContractError::BoundExceeded))?;
                while let Some(value) = sequence.next_element()? {
                    if values.len() == MAX_ITEMS {
                        return Err(A::Error::custom(ContractError::BoundExceeded));
                    }
                    values.push(value);
                }
                ContractValue::array(values).map_err(A::Error::custom)
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
            where
                A: MapAccess<'de>,
            {
                if map.size_hint().is_some_and(|size| size > MAX_ITEMS) {
                    return Err(A::Error::custom(ContractError::BoundExceeded));
                }
                let capacity = map.size_hint().unwrap_or(0).min(MAX_ITEMS);
                let mut members = Vec::new();
                members
                    .try_reserve_exact(capacity)
                    .map_err(|_| A::Error::custom(ContractError::BoundExceeded))?;
                while let Some(key) = map.next_key::<BoundedString<255>>()? {
                    if members.len() == MAX_ITEMS {
                        return Err(A::Error::custom(ContractError::BoundExceeded));
                    }
                    let value = map.next_value()?;
                    members.push(ContractMember::new(key, value));
                }
                ContractValue::object(members).map_err(A::Error::custom)
            }
        }

        deserializer
            .deserialize_any(ContractValueVisitor)
            .map_err(sanitize_deserialize_error)
    }
}

fn utf16_cmp(left: &str, right: &str) -> Ordering {
    left.encode_utf16().cmp(right.encode_utf16())
}

fn sanitize_deserialize_error<E: serde::de::Error>(error: E) -> E {
    E::custom(super::serialization::classify_contract_error_message(
        &error.to_string(),
    ))
}
