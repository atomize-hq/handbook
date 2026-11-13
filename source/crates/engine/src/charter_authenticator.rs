use p256::ecdsa::{
    signature::Verifier, Signature as Es256Signature, VerifyingKey as Es256VerifyingKey,
};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::{collections::BTreeSet, fmt};

pub const CTAP_MAKE_CREDENTIAL_COMMAND: u8 = 0x01;
pub const CTAP_GET_ASSERTION_COMMAND: u8 = 0x02;
pub const AUTHENTICATOR_RP_ID: &str = "handbook.local";
pub const MAX_CREDENTIAL_ID_BYTES: usize = 1024;
pub const MAX_ALLOW_LIST_ITEMS: usize = 64;
const MAX_AUTHENTICATOR_RESPONSE_BYTES: usize = 16_384;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AuthenticatorUserHandleV1 {
    pub bytes: [u8; 32],
    pub hash: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AuthenticatorCredentialV1 {
    pub credential_id: Vec<u8>,
    pub cose_public_key: Vec<u8>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MakeCredentialResponseV1 {
    pub rp_id_hash: [u8; 32],
    pub flags_byte: u8,
    pub sign_count: u32,
    pub aaguid: [u8; 16],
    pub credential_id: Vec<u8>,
    pub credential_id_hash: String,
    pub cose_public_key: Vec<u8>,
    pub authenticator_data: Vec<u8>,
    pub attestation_object: Vec<u8>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct GetAssertionResponseV1 {
    pub credential_id: Vec<u8>,
    pub authenticator_data: [u8; 37],
    pub signature_der: Vec<u8>,
    pub flags_byte: u8,
    pub sign_count: u32,
    pub attested_credential_data_included: bool,
    pub extensions_included: bool,
    pub raw_response: Vec<u8>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CredentialSelectionCandidateV1 {
    pub credential_id: Vec<u8>,
    pub active: bool,
    pub sequence: u16,
    pub algorithm_es256: bool,
    pub covers_required_pair: bool,
}

impl CredentialSelectionCandidateV1 {
    pub fn eligible(credential_id: Vec<u8>) -> Self {
        Self {
            credential_id,
            active: true,
            sequence: 0,
            algorithm_es256: true,
            covers_required_pair: true,
        }
    }

    pub fn inactive(credential_id: Vec<u8>) -> Self {
        Self {
            active: false,
            ..Self::eligible(credential_id)
        }
    }

    pub fn exhausted(credential_id: Vec<u8>) -> Self {
        Self {
            sequence: 4096,
            ..Self::eligible(credential_id)
        }
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CtapRefusalCodeV1 {
    InvalidRequest,
    TransactionConflict,
    AuthenticatorUserTimeout,
    AuthenticatorResourceExhausted,
    AuthorizationRefused,
    AuthenticatorSecurityBlocked,
    AuthenticatorUserCancelled,
    AuthenticatorUserVerificationRetry,
    AuthenticatorUnknownError,
}

impl CtapRefusalCodeV1 {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::InvalidRequest => "invalid_request",
            Self::TransactionConflict => "transaction_conflict",
            Self::AuthenticatorUserTimeout => "authenticator_user_timeout",
            Self::AuthenticatorResourceExhausted => "authenticator_resource_exhausted",
            Self::AuthorizationRefused => "authorization_refused",
            Self::AuthenticatorSecurityBlocked => "authenticator_security_blocked",
            Self::AuthenticatorUserCancelled => "authenticator_user_cancelled",
            Self::AuthenticatorUserVerificationRetry => "authenticator_user_verification_retry",
            Self::AuthenticatorUnknownError => "authenticator_unknown_error",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct CtapRefusalV1 {
    pub status_byte: u8,
    pub code: CtapRefusalCodeV1,
    pub retryable: bool,
    pub next_action: &'static str,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum CtapStatusOutcomeV1 {
    Success,
    Refused(CtapRefusalV1),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum NativeAuthenticatorPortErrorV1 {
    Unavailable,
    Transport,
}

pub trait NativeAuthenticatorPortV1 {
    fn make_credential(
        &mut self,
        request_cbor: &[u8],
    ) -> Result<Vec<u8>, NativeAuthenticatorPortErrorV1>;

    fn get_assertion(
        &mut self,
        request_cbor: &[u8],
    ) -> Result<Vec<u8>, NativeAuthenticatorPortErrorV1>;
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum AuthenticatorErrorV1 {
    InvalidInput(&'static str),
    InvalidCbor(&'static str),
    InvalidAuthenticatorData(&'static str),
    InvalidCredential(&'static str),
    InvalidSignature,
    Refused(CtapRefusalV1),
}

impl fmt::Display for AuthenticatorErrorV1 {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidInput(message)
            | Self::InvalidCbor(message)
            | Self::InvalidAuthenticatorData(message)
            | Self::InvalidCredential(message) => formatter.write_str(message),
            Self::InvalidSignature => formatter.write_str("ES256 signature verification failed"),
            Self::Refused(refusal) => write!(
                formatter,
                "authenticator status 0x{:02x}: {}",
                refusal.status_byte,
                refusal.code.as_str()
            ),
        }
    }
}

impl std::error::Error for AuthenticatorErrorV1 {}

pub fn derive_authenticator_user_handle(
    repository_identity_fingerprint: &str,
) -> Result<AuthenticatorUserHandleV1, AuthenticatorErrorV1> {
    if !is_sha256_fingerprint(repository_identity_fingerprint) {
        return Err(AuthenticatorErrorV1::InvalidInput(
            "repository identity must be an exact lowercase SHA-256 fingerprint",
        ));
    }
    let bytes: [u8; 32] = Sha256::digest(repository_identity_fingerprint.as_bytes()).into();
    Ok(AuthenticatorUserHandleV1 {
        hash: fingerprint_from_digest(&bytes),
        bytes,
    })
}

pub fn encode_make_credential_request(
    client_data_hash: [u8; 32],
    user_handle: [u8; 32],
) -> Result<Vec<u8>, AuthenticatorErrorV1> {
    let mut output = vec![CTAP_MAKE_CREDENTIAL_COMMAND];
    cbor_map(&mut output, 5);
    cbor_uint(&mut output, 1);
    cbor_bytes(&mut output, &client_data_hash);
    cbor_uint(&mut output, 2);
    cbor_map(&mut output, 2);
    cbor_text(&mut output, "id");
    cbor_text(&mut output, AUTHENTICATOR_RP_ID);
    cbor_text(&mut output, "name");
    cbor_text(&mut output, "Handbook repository authority");
    cbor_uint(&mut output, 3);
    cbor_map(&mut output, 3);
    cbor_text(&mut output, "id");
    cbor_bytes(&mut output, &user_handle);
    cbor_text(&mut output, "name");
    cbor_text(&mut output, "handbook-repository-authority");
    cbor_text(&mut output, "displayName");
    cbor_text(&mut output, "Handbook repository authority");
    cbor_uint(&mut output, 4);
    cbor_array(&mut output, 1);
    cbor_map(&mut output, 2);
    cbor_text(&mut output, "alg");
    cbor_negative(&mut output, -7);
    cbor_text(&mut output, "type");
    cbor_text(&mut output, "public-key");
    cbor_uint(&mut output, 7);
    cbor_map(&mut output, 2);
    cbor_text(&mut output, "rk");
    output.push(0xf4);
    cbor_text(&mut output, "uv");
    output.push(0xf5);
    Ok(output)
}

pub fn encode_get_assertion_request(
    client_data_hash: [u8; 32],
    credential_ids: &[Vec<u8>],
) -> Result<Vec<u8>, AuthenticatorErrorV1> {
    let mut credential_ids = credential_ids.to_vec();
    validate_and_sort_credential_ids(&mut credential_ids)?;

    let mut output = vec![CTAP_GET_ASSERTION_COMMAND];
    cbor_map(&mut output, 4);
    cbor_uint(&mut output, 1);
    cbor_text(&mut output, AUTHENTICATOR_RP_ID);
    cbor_uint(&mut output, 2);
    cbor_bytes(&mut output, &client_data_hash);
    cbor_uint(&mut output, 3);
    cbor_array(&mut output, credential_ids.len());
    for credential_id in credential_ids {
        cbor_map(&mut output, 2);
        cbor_text(&mut output, "id");
        cbor_bytes(&mut output, &credential_id);
        cbor_text(&mut output, "type");
        cbor_text(&mut output, "public-key");
    }
    cbor_uint(&mut output, 5);
    cbor_map(&mut output, 2);
    cbor_text(&mut output, "up");
    output.push(0xf5);
    cbor_text(&mut output, "uv");
    output.push(0xf5);
    Ok(output)
}

pub fn select_eligible_credentials(
    candidates: &[CredentialSelectionCandidateV1],
) -> Result<Vec<Vec<u8>>, AuthenticatorErrorV1> {
    let mut all_ids = BTreeSet::new();
    let mut selected = Vec::new();
    for candidate in candidates {
        validate_credential_id(&candidate.credential_id)?;
        if !all_ids.insert(candidate.credential_id.as_slice()) {
            return Err(AuthenticatorErrorV1::InvalidCredential(
                "duplicate credential ID in registry state",
            ));
        }
        if candidate.active
            && candidate.sequence < 4096
            && candidate.algorithm_es256
            && candidate.covers_required_pair
        {
            selected.push(candidate.credential_id.clone());
        }
    }
    validate_and_sort_credential_ids(&mut selected)?;
    Ok(selected)
}

pub fn decode_make_credential_response(
    raw_response: &[u8],
) -> Result<MakeCredentialResponseV1, AuthenticatorErrorV1> {
    let payload = success_payload(raw_response)?;
    let mut cursor = CborCursor::new(payload);
    cursor.exact_map(3)?;
    cursor.exact_text("fmt")?;
    cursor.exact_text("none")?;
    cursor.exact_text("attStmt")?;
    cursor.exact_map(0)?;
    cursor.exact_text("authData")?;
    let authenticator_data = cursor.bytes(MAX_AUTHENTICATOR_RESPONSE_BYTES)?.to_vec();
    cursor.finish()?;

    if authenticator_data.len() < 55 {
        return Err(AuthenticatorErrorV1::InvalidAuthenticatorData(
            "make-credential authenticator data is truncated",
        ));
    }
    let rp_id_hash: [u8; 32] = authenticator_data[..32].try_into().unwrap();
    if rp_id_hash != Sha256::digest(AUTHENTICATOR_RP_ID.as_bytes()).as_slice() {
        return Err(AuthenticatorErrorV1::InvalidAuthenticatorData(
            "make-credential RP ID hash mismatch",
        ));
    }
    let flags_byte = authenticator_data[32];
    if flags_byte != 0x45 {
        return Err(AuthenticatorErrorV1::InvalidAuthenticatorData(
            "make-credential flags must equal 0x45",
        ));
    }
    let sign_count = u32::from_be_bytes(authenticator_data[33..37].try_into().unwrap());
    let aaguid: [u8; 16] = authenticator_data[37..53].try_into().unwrap();
    let credential_length = u16::from_be_bytes(authenticator_data[53..55].try_into().unwrap());
    let credential_length = usize::from(credential_length);
    if credential_length == 0 || credential_length > MAX_CREDENTIAL_ID_BYTES {
        return Err(AuthenticatorErrorV1::InvalidCredential(
            "credential ID length is outside the admitted range",
        ));
    }
    let credential_end =
        55usize
            .checked_add(credential_length)
            .ok_or(AuthenticatorErrorV1::InvalidCredential(
                "credential ID length overflow",
            ))?;
    if credential_end >= authenticator_data.len() {
        return Err(AuthenticatorErrorV1::InvalidAuthenticatorData(
            "credential ID or COSE key is truncated",
        ));
    }
    let credential_id = authenticator_data[55..credential_end].to_vec();
    let cose_public_key = authenticator_data[credential_end..].to_vec();
    parse_cose_es256_key(&cose_public_key)?;

    Ok(MakeCredentialResponseV1 {
        rp_id_hash,
        flags_byte,
        sign_count,
        aaguid,
        credential_id_hash: sha256_fingerprint(&credential_id),
        credential_id,
        cose_public_key,
        authenticator_data,
        attestation_object: payload.to_vec(),
    })
}

pub fn decode_and_verify_get_assertion_response(
    raw_response: &[u8],
    requested_credentials: &[AuthenticatorCredentialV1],
    client_data_hash: [u8; 32],
) -> Result<GetAssertionResponseV1, AuthenticatorErrorV1> {
    if requested_credentials.is_empty() || requested_credentials.len() > MAX_ALLOW_LIST_ITEMS {
        return Err(AuthenticatorErrorV1::InvalidCredential(
            "requested credential list must contain one through 64 entries",
        ));
    }
    let mut requested_ids = BTreeSet::new();
    for credential in requested_credentials {
        validate_credential_id(&credential.credential_id)?;
        if !requested_ids.insert(credential.credential_id.as_slice()) {
            return Err(AuthenticatorErrorV1::InvalidCredential(
                "duplicate requested credential ID",
            ));
        }
    }

    let payload = success_payload(raw_response)?;
    let mut cursor = CborCursor::new(payload);
    cursor.exact_map(3)?;
    cursor.exact_uint(1)?;
    cursor.exact_map(2)?;
    cursor.exact_text("id")?;
    let credential_id = cursor.bytes(MAX_CREDENTIAL_ID_BYTES)?.to_vec();
    validate_credential_id(&credential_id)?;
    cursor.exact_text("type")?;
    cursor.exact_text("public-key")?;
    cursor.exact_uint(2)?;
    let authenticator_data: [u8; 37] = cursor.bytes(37)?.try_into().map_err(|_| {
        AuthenticatorErrorV1::InvalidAuthenticatorData(
            "authenticator data must be exactly 37 bytes",
        )
    })?;
    cursor.exact_uint(3)?;
    let signature_der = cursor.bytes(4096)?.to_vec();
    cursor.finish()?;

    let matches: Vec<_> = requested_credentials
        .iter()
        .filter(|credential| credential.credential_id == credential_id)
        .collect();
    if matches.len() != 1 {
        return Err(AuthenticatorErrorV1::InvalidCredential(
            "response descriptor must select exactly one requested credential",
        ));
    }
    let expected_rp_hash: [u8; 32] = Sha256::digest(AUTHENTICATOR_RP_ID.as_bytes()).into();
    if authenticator_data[..32] != expected_rp_hash {
        return Err(AuthenticatorErrorV1::InvalidAuthenticatorData(
            "assertion RP ID hash mismatch",
        ));
    }
    let flags_byte = authenticator_data[32];
    if flags_byte != 0x05 {
        return Err(AuthenticatorErrorV1::InvalidAuthenticatorData(
            "assertion flags must equal literal 0x05",
        ));
    }
    let sign_count = u32::from_be_bytes(authenticator_data[33..37].try_into().unwrap());
    verify_es256_signature(
        &matches[0].cose_public_key,
        &authenticator_data,
        client_data_hash,
        &signature_der,
    )?;

    Ok(GetAssertionResponseV1 {
        credential_id,
        authenticator_data,
        signature_der,
        flags_byte,
        sign_count,
        attested_credential_data_included: false,
        extensions_included: false,
        raw_response: raw_response.to_vec(),
    })
}

pub fn verify_es256_signature(
    cose_public_key: &[u8],
    authenticator_data: &[u8],
    client_data_hash: [u8; 32],
    signature_der: &[u8],
) -> Result<(), AuthenticatorErrorV1> {
    let (x, y) = parse_cose_es256_key(cose_public_key)?;
    let mut sec1 = [0u8; 65];
    sec1[0] = 0x04;
    sec1[1..33].copy_from_slice(&x);
    sec1[33..].copy_from_slice(&y);
    let verifying_key = Es256VerifyingKey::from_sec1_bytes(&sec1)
        .map_err(|_| AuthenticatorErrorV1::InvalidCredential("invalid P-256 public key"))?;
    let signature = Es256Signature::from_der(signature_der)
        .map_err(|_| AuthenticatorErrorV1::InvalidSignature)?;
    if signature.to_der().as_bytes() != signature_der {
        return Err(AuthenticatorErrorV1::InvalidSignature);
    }
    let mut signed_preimage = Vec::with_capacity(authenticator_data.len() + 32);
    signed_preimage.extend_from_slice(authenticator_data);
    signed_preimage.extend_from_slice(&client_data_hash);
    verifying_key
        .verify(&signed_preimage, &signature)
        .map_err(|_| AuthenticatorErrorV1::InvalidSignature)
}

pub fn map_ctap_status(status_byte: u8) -> CtapStatusOutcomeV1 {
    use CtapRefusalCodeV1::*;
    if status_byte == 0 {
        return CtapStatusOutcomeV1::Success;
    }
    let (code, retryable, next_action) = match status_byte {
        0x01 | 0x02 | 0x03 | 0x04 | 0x0b | 0x11 | 0x12 | 0x14 | 0x26 | 0x2b | 0x2c | 0x37
        | 0x39 | 0x3e => (
            InvalidRequest,
            false,
            "correct the engine-built request or reject the incompatible authenticator response",
        ),
        0x06 | 0x0a | 0x21 | 0x23 | 0x24 | 0x25 => (
            TransactionConflict,
            true,
            "retry the complete operation with a fresh observed head and fresh challenge",
        ),
        0x05 | 0x2f | 0x3a => (
            AuthenticatorUserTimeout,
            true,
            "repeat only through a new explicit user-directed ceremony",
        ),
        0x15 | 0x17 | 0x18 | 0x28 => (
            AuthenticatorResourceExhausted,
            false,
            "free or replace authenticator capacity before retry",
        ),
        0x19 | 0x22 | 0x27 | 0x2e | 0x30 | 0x35 | 0x40 => (
            AuthorizationRefused,
            false,
            "use an eligible authorized credential or change committed authority",
        ),
        0x32 | 0x34 | 0x3c | 0x3d => (
            AuthenticatorSecurityBlocked,
            false,
            "unblock or replace the authenticator through its trusted operator flow",
        ),
        0x2d => (
            AuthenticatorUserCancelled,
            false,
            "begin a new explicit user-directed ceremony if still desired",
        ),
        0x31 | 0x33 | 0x36 | 0x3b | 0x3f => (
            AuthenticatorUserVerificationRetry,
            true,
            "repeat with a fresh challenge and successful user verification",
        ),
        _ => (
            AuthenticatorUnknownError,
            false,
            "preserve the status as evidence and require operator diagnosis",
        ),
    };
    CtapStatusOutcomeV1::Refused(CtapRefusalV1 {
        status_byte,
        code,
        retryable,
        next_action,
    })
}

fn success_payload(raw_response: &[u8]) -> Result<&[u8], AuthenticatorErrorV1> {
    if raw_response.is_empty() || raw_response.len() > MAX_AUTHENTICATOR_RESPONSE_BYTES {
        return Err(AuthenticatorErrorV1::InvalidInput(
            "authenticator response length is outside the admitted range",
        ));
    }
    match map_ctap_status(raw_response[0]) {
        CtapStatusOutcomeV1::Success if raw_response.len() > 1 => Ok(&raw_response[1..]),
        CtapStatusOutcomeV1::Success => Err(AuthenticatorErrorV1::InvalidCbor(
            "successful response is missing CBOR",
        )),
        CtapStatusOutcomeV1::Refused(refusal) => Err(AuthenticatorErrorV1::Refused(refusal)),
    }
}

fn validate_and_sort_credential_ids(
    credential_ids: &mut [Vec<u8>],
) -> Result<(), AuthenticatorErrorV1> {
    if credential_ids.is_empty() || credential_ids.len() > MAX_ALLOW_LIST_ITEMS {
        return Err(AuthenticatorErrorV1::InvalidCredential(
            "eligible credential list must contain one through 64 entries",
        ));
    }
    for credential_id in credential_ids.iter() {
        validate_credential_id(credential_id)?;
    }
    credential_ids.sort();
    if credential_ids.windows(2).any(|pair| pair[0] == pair[1]) {
        return Err(AuthenticatorErrorV1::InvalidCredential(
            "duplicate credential ID",
        ));
    }
    Ok(())
}

fn validate_credential_id(credential_id: &[u8]) -> Result<(), AuthenticatorErrorV1> {
    if credential_id.is_empty() || credential_id.len() > MAX_CREDENTIAL_ID_BYTES {
        return Err(AuthenticatorErrorV1::InvalidCredential(
            "credential ID length is outside the admitted range",
        ));
    }
    Ok(())
}

fn parse_cose_es256_key(bytes: &[u8]) -> Result<([u8; 32], [u8; 32]), AuthenticatorErrorV1> {
    let mut cursor = CborCursor::new(bytes);
    cursor.exact_map(5)?;
    cursor.exact_uint(1)?;
    cursor.exact_uint(2)?;
    cursor.exact_uint(3)?;
    cursor.exact_integer(-7)?;
    cursor.exact_integer(-1)?;
    cursor.exact_uint(1)?;
    cursor.exact_integer(-2)?;
    let x: [u8; 32] = cursor
        .bytes(32)?
        .try_into()
        .map_err(|_| AuthenticatorErrorV1::InvalidCredential("COSE x must be 32 bytes"))?;
    cursor.exact_integer(-3)?;
    let y: [u8; 32] = cursor
        .bytes(32)?
        .try_into()
        .map_err(|_| AuthenticatorErrorV1::InvalidCredential("COSE y must be 32 bytes"))?;
    cursor.finish()?;
    let mut sec1 = [0u8; 65];
    sec1[0] = 0x04;
    sec1[1..33].copy_from_slice(&x);
    sec1[33..].copy_from_slice(&y);
    Es256VerifyingKey::from_sec1_bytes(&sec1)
        .map_err(|_| AuthenticatorErrorV1::InvalidCredential("COSE key is not on P-256"))?;
    Ok((x, y))
}

fn is_sha256_fingerprint(value: &str) -> bool {
    value.len() == 71
        && value.starts_with("sha256:")
        && value.as_bytes()[7..]
            .iter()
            .all(|byte| byte.is_ascii_digit() || matches!(byte, b'a'..=b'f'))
}

fn sha256_fingerprint(bytes: &[u8]) -> String {
    fingerprint_from_digest(&Sha256::digest(bytes))
}

fn fingerprint_from_digest(bytes: &[u8]) -> String {
    let mut value = String::with_capacity(71);
    value.push_str("sha256:");
    for byte in bytes {
        use std::fmt::Write as _;
        write!(&mut value, "{byte:02x}").expect("writing to a String cannot fail");
    }
    value
}

fn cbor_head(output: &mut Vec<u8>, major: u8, value: usize) {
    match value {
        0..=23 => output.push(major << 5 | value as u8),
        24..=255 => output.extend_from_slice(&[major << 5 | 24, value as u8]),
        256..=65_535 => {
            output.push(major << 5 | 25);
            output.extend_from_slice(&(value as u16).to_be_bytes());
        }
        _ => {
            output.push(major << 5 | 26);
            output.extend_from_slice(&(value as u32).to_be_bytes());
        }
    }
}

fn cbor_uint(output: &mut Vec<u8>, value: usize) {
    cbor_head(output, 0, value);
}

fn cbor_negative(output: &mut Vec<u8>, value: i64) {
    debug_assert!(value < 0);
    cbor_head(output, 1, (-1 - value) as usize);
}

fn cbor_bytes(output: &mut Vec<u8>, value: &[u8]) {
    cbor_head(output, 2, value.len());
    output.extend_from_slice(value);
}

fn cbor_text(output: &mut Vec<u8>, value: &str) {
    cbor_head(output, 3, value.len());
    output.extend_from_slice(value.as_bytes());
}

fn cbor_array(output: &mut Vec<u8>, length: usize) {
    cbor_head(output, 4, length);
}

fn cbor_map(output: &mut Vec<u8>, length: usize) {
    cbor_head(output, 5, length);
}

struct CborCursor<'a> {
    bytes: &'a [u8],
    offset: usize,
}

impl<'a> CborCursor<'a> {
    fn new(bytes: &'a [u8]) -> Self {
        Self { bytes, offset: 0 }
    }

    fn argument(&mut self, expected_major: u8) -> Result<u64, AuthenticatorErrorV1> {
        let initial = *self
            .bytes
            .get(self.offset)
            .ok_or(AuthenticatorErrorV1::InvalidCbor("truncated CBOR item"))?;
        self.offset += 1;
        if initial >> 5 != expected_major {
            return Err(AuthenticatorErrorV1::InvalidCbor(
                "unexpected CBOR major type",
            ));
        }
        let additional = initial & 0x1f;
        let (value, minimum) = match additional {
            value @ 0..=23 => (u64::from(value), 0),
            24 => (u64::from(self.take_array::<1>()?[0]), 24),
            25 => (u64::from(u16::from_be_bytes(self.take_array::<2>()?)), 256),
            26 => (
                u64::from(u32::from_be_bytes(self.take_array::<4>()?)),
                65_536,
            ),
            27 => (u64::from_be_bytes(self.take_array::<8>()?), 4_294_967_296),
            _ => {
                return Err(AuthenticatorErrorV1::InvalidCbor(
                    "indefinite or reserved CBOR argument",
                ))
            }
        };
        if value < minimum {
            return Err(AuthenticatorErrorV1::InvalidCbor(
                "non-shortest CBOR integer or length",
            ));
        }
        Ok(value)
    }

    fn exact_uint(&mut self, expected: u64) -> Result<(), AuthenticatorErrorV1> {
        if self.argument(0)? == expected {
            Ok(())
        } else {
            Err(AuthenticatorErrorV1::InvalidCbor(
                "unexpected CBOR integer key",
            ))
        }
    }

    fn exact_integer(&mut self, expected: i64) -> Result<(), AuthenticatorErrorV1> {
        let initial = *self
            .bytes
            .get(self.offset)
            .ok_or(AuthenticatorErrorV1::InvalidCbor("truncated CBOR integer"))?;
        let value = match initial >> 5 {
            0 => self.argument(0)? as i64,
            1 => -1 - self.argument(1)? as i64,
            _ => return Err(AuthenticatorErrorV1::InvalidCbor("expected CBOR integer")),
        };
        if value == expected {
            Ok(())
        } else {
            Err(AuthenticatorErrorV1::InvalidCbor(
                "unexpected CBOR integer value",
            ))
        }
    }

    fn exact_map(&mut self, expected: u64) -> Result<(), AuthenticatorErrorV1> {
        if self.argument(5)? == expected {
            Ok(())
        } else {
            Err(AuthenticatorErrorV1::InvalidCbor(
                "unexpected CBOR map length",
            ))
        }
    }

    fn exact_text(&mut self, expected: &str) -> Result<(), AuthenticatorErrorV1> {
        let length = usize::try_from(self.argument(3)?)
            .map_err(|_| AuthenticatorErrorV1::InvalidCbor("CBOR text length overflow"))?;
        if self.take(length)? == expected.as_bytes() {
            Ok(())
        } else {
            Err(AuthenticatorErrorV1::InvalidCbor("unexpected CBOR text"))
        }
    }

    fn bytes(&mut self, maximum: usize) -> Result<&'a [u8], AuthenticatorErrorV1> {
        let length = usize::try_from(self.argument(2)?)
            .map_err(|_| AuthenticatorErrorV1::InvalidCbor("CBOR byte length overflow"))?;
        if length > maximum {
            return Err(AuthenticatorErrorV1::InvalidCbor(
                "CBOR byte string exceeds admitted bound",
            ));
        }
        self.take(length)
    }

    fn take(&mut self, length: usize) -> Result<&'a [u8], AuthenticatorErrorV1> {
        let end = self
            .offset
            .checked_add(length)
            .ok_or(AuthenticatorErrorV1::InvalidCbor("CBOR length overflow"))?;
        let value = self
            .bytes
            .get(self.offset..end)
            .ok_or(AuthenticatorErrorV1::InvalidCbor("truncated CBOR value"))?;
        self.offset = end;
        Ok(value)
    }

    fn take_array<const N: usize>(&mut self) -> Result<[u8; N], AuthenticatorErrorV1> {
        self.take(N)?
            .try_into()
            .map_err(|_| AuthenticatorErrorV1::InvalidCbor("truncated CBOR argument"))
    }

    fn finish(self) -> Result<(), AuthenticatorErrorV1> {
        if self.offset == self.bytes.len() {
            Ok(())
        } else {
            Err(AuthenticatorErrorV1::InvalidCbor("trailing CBOR data"))
        }
    }
}
