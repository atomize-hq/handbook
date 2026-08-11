use super::{ContractError, Fingerprint};
use std::fmt;

const MAX_RAW_KEY_BYTES: usize = 65_536;
const REDACTION: &str = "[REDACTED raw idempotency key]";

pub struct RawIdempotencyKey {
    bytes: Vec<u8>,
}

impl RawIdempotencyKey {
    pub fn new(bytes: Vec<u8>, max_bytes: usize) -> Result<Self, ContractError> {
        if max_bytes == 0 || max_bytes > MAX_RAW_KEY_BYTES {
            return Err(ContractError::LimitOutOfRange);
        }
        if bytes.is_empty() {
            return Err(ContractError::Empty);
        }
        if bytes.len() > max_bytes {
            return Err(ContractError::BoundExceeded);
        }
        let value = std::str::from_utf8(&bytes).map_err(|_| ContractError::InvalidString)?;
        if value.chars().any(char::is_control) {
            return Err(ContractError::InvalidString);
        }
        Ok(Self { bytes })
    }
}

impl fmt::Debug for RawIdempotencyKey {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(REDACTION)
    }
}

impl fmt::Display for RawIdempotencyKey {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(REDACTION)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum DefinitionPinnedKeyPointer {
    BodyIdempotencyKey,
}

impl fmt::Display for DefinitionPinnedKeyPointer {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::BodyIdempotencyKey => formatter.write_str("/body/idempotency_key"),
        }
    }
}

#[doc(hidden)]
pub(crate) trait IdempotencyKeyConsumerSealed {
    fn consume_owner_idempotency_key(&mut self, pointer: &str, bytes: &[u8]) -> Fingerprint;
}

/// A sealed SDK owner capability that can consume a raw idempotency key.
///
/// Downstream crates cannot implement this capability or obtain the borrowed
/// secret bytes:
///
/// ```compile_fail
/// use handbook_sdk::contract_membrane::IdempotencyKeyConsumer;
///
/// struct AlwaysConsume;
/// impl IdempotencyKeyConsumer for AlwaysConsume {}
/// ```
#[allow(private_bounds)]
pub trait IdempotencyKeyConsumer: IdempotencyKeyConsumerSealed {}

pub fn with_owner_idempotency_key(
    key: &RawIdempotencyKey,
    pointer: DefinitionPinnedKeyPointer,
    consumer: &mut dyn IdempotencyKeyConsumer,
) -> Fingerprint {
    let pointer = match pointer {
        DefinitionPinnedKeyPointer::BodyIdempotencyKey => "/body/idempotency_key",
    };
    consumer.consume_owner_idempotency_key(pointer, &key.bytes)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::contract_membrane::sha256_fingerprint;

    struct DigestConsumer {
        expected_bytes: *const u8,
    }

    impl IdempotencyKeyConsumerSealed for DigestConsumer {
        fn consume_owner_idempotency_key(&mut self, pointer: &str, bytes: &[u8]) -> Fingerprint {
            assert_eq!(pointer, "/body/idempotency_key");
            assert_eq!(bytes.as_ptr(), self.expected_bytes);
            sha256_fingerprint(bytes)
        }
    }

    impl IdempotencyKeyConsumer for DigestConsumer {}

    #[test]
    fn owner_bridge_delivers_the_fixed_pointer_and_borrowed_bytes() {
        let key = RawIdempotencyKey::new(b"owner-secret".to_vec(), 64).unwrap();
        let mut consumer = DigestConsumer {
            expected_bytes: key.bytes.as_ptr(),
        };

        let digest = with_owner_idempotency_key(
            &key,
            DefinitionPinnedKeyPointer::BodyIdempotencyKey,
            &mut consumer,
        );
        assert_eq!(digest, sha256_fingerprint(b"owner-secret"));
    }
}
