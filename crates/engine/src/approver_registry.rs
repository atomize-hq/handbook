use crate::charter_authenticator::{
    decode_make_credential_response, derive_authenticator_user_handle,
    encode_make_credential_request, NativeAuthenticatorPortErrorV1, NativeAuthenticatorPortV1,
};
use base64::{engine::general_purpose::STANDARD as BASE64_STANDARD, Engine as _};
use jsonschema::PatternOptions;
use serde::{de::Error as _, Deserialize, Deserializer, Serialize, Serializer};
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{
    collections::BTreeSet,
    fmt,
    path::{Path, PathBuf},
};

const ADMIN_SCHEMA: &str = include_str!(
    "../definitions/schemas/handbook.schemas.security.approver-admin-api/1.0.0.schema.json"
);

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ApproverAdminJsonErrorV1(String);

impl fmt::Display for ApproverAdminJsonErrorV1 {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

impl std::error::Error for ApproverAdminJsonErrorV1 {}

#[derive(Clone, Debug, PartialEq)]
pub struct ApproverAdminRequestV1 {
    document: Value,
}

impl ApproverAdminRequestV1 {
    pub fn from_json_value(document: Value) -> Result<Self, ApproverAdminJsonErrorV1> {
        validate_admin_document(&document, "handbook.approver-admin-request")?;
        validate_mapping_uniqueness(&document)?;
        Ok(Self { document })
    }

    pub fn to_json_value(&self) -> Result<Value, ApproverAdminJsonErrorV1> {
        validate_admin_document(&self.document, "handbook.approver-admin-request")?;
        Ok(self.document.clone())
    }

    pub fn operation(&self) -> &str {
        self.string("operation")
    }

    pub fn operation_id(&self) -> &str {
        self.string("operation_id")
    }

    pub fn repository_identity_fingerprint(&self) -> &str {
        self.string("repository_identity_fingerprint")
    }

    pub(crate) fn document(&self) -> &Value {
        &self.document
    }

    fn string(&self, name: &str) -> &str {
        self.document[name]
            .as_str()
            .expect("validated approver request string")
    }
}

impl Serialize for ApproverAdminRequestV1 {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.document.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for ApproverAdminRequestV1 {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        Self::from_json_value(Value::deserialize(deserializer)?).map_err(D::Error::custom)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct ApproverAdminResultV1 {
    document: Value,
}

impl ApproverAdminResultV1 {
    pub fn from_json_value(document: Value) -> Result<Self, ApproverAdminJsonErrorV1> {
        validate_admin_document(&document, "handbook.approver-admin-result")?;
        Ok(Self { document })
    }

    pub fn to_json_value(&self) -> Result<Value, ApproverAdminJsonErrorV1> {
        validate_admin_document(&self.document, "handbook.approver-admin-result")?;
        Ok(self.document.clone())
    }

    fn checked(document: Value) -> Self {
        debug_assert!(Self::from_json_value(document.clone()).is_ok());
        Self { document }
    }
}

impl Serialize for ApproverAdminResultV1 {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.document.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for ApproverAdminResultV1 {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        Self::from_json_value(Value::deserialize(deserializer)?).map_err(D::Error::custom)
    }
}

pub struct ApproverRegistryServiceV1<P> {
    port: P,
    repo_root: Option<PathBuf>,
}

impl<P: NativeAuthenticatorPortV1> ApproverRegistryServiceV1<P> {
    pub fn new(port: P) -> Self {
        Self {
            port,
            repo_root: None,
        }
    }

    pub fn for_repository(repo_root: impl AsRef<Path>, port: P) -> Self {
        Self {
            port,
            repo_root: Some(repo_root.as_ref().to_path_buf()),
        }
    }

    pub fn into_port(self) -> P {
        self.port
    }

    pub fn bootstrap_approver_registry(
        &mut self,
        request: ApproverAdminRequestV1,
    ) -> ApproverAdminResultV1 {
        if let Some(repo_root) = &self.repo_root {
            return crate::charter_authority_workflow::bootstrap_registry(
                repo_root,
                &mut self.port,
                &request,
            );
        }
        if request.operation() != "bootstrap" {
            return refused(
                &request,
                "invalid_request",
                "bootstrap endpoint requires bootstrap operation",
                false,
            );
        }
        if request.document["initial_charter_quorum"]
            .as_array()
            .expect("validated bootstrap quorum")
            .iter()
            .any(|mapping| {
                mapping["approval_class"] == "registry_admin"
                    && mapping["authority_ref"] == "repository_registry_admin"
            })
        {
            return refused(
                &request,
                "invalid_request",
                "bootstrap quorum duplicates the engine-owned registry administrator mapping",
                false,
            );
        }

        let user_handle =
            match derive_authenticator_user_handle(request.repository_identity_fingerprint()) {
                Ok(value) => value,
                Err(_) => {
                    return refused(
                        &request,
                        "invalid_request",
                        "repository identity is not admissible",
                        false,
                    )
                }
            };
        let client_data_hash = match bootstrap_client_data_hash(&request, &user_handle.hash) {
            Ok(value) => value,
            Err(_) => {
                return refused(
                    &request,
                    "transaction_conflict",
                    "operating-system randomness was unavailable",
                    true,
                )
            }
        };
        let cbor = encode_make_credential_request(client_data_hash, user_handle.bytes)
            .expect("validated fixed-size make-credential request");
        let response = match self.port.make_credential(&cbor) {
            Ok(value) => value,
            Err(error) => return refused_for_port_error(&request, error),
        };
        match decode_make_credential_response(&response) {
            Ok(_) => {}
            Err(_) => {
                return refused(
                    &request,
                    "registration_invalid",
                    "authenticator response was invalid",
                    false,
                )
            }
        }
        refused(
            &request,
            "transaction_conflict",
            "registry transaction integration is not available in the in-memory security core",
            true,
        )
    }

    pub fn add_approver_credential(
        &mut self,
        request: ApproverAdminRequestV1,
    ) -> ApproverAdminResultV1 {
        if let Some(repo_root) = &self.repo_root {
            return crate::approver_registry_mutation::add_credential(
                repo_root,
                &mut self.port,
                &request,
            );
        }
        self.closed_nonbootstrap(request, "add_credential")
    }

    pub fn revoke_approver_credential(
        &mut self,
        request: ApproverAdminRequestV1,
    ) -> ApproverAdminResultV1 {
        if let Some(repo_root) = &self.repo_root {
            return crate::approver_registry_mutation::revoke_credential(
                repo_root,
                &mut self.port,
                &request,
            );
        }
        self.closed_nonbootstrap(request, "revoke_credential")
    }

    pub fn update_approver_mapping(
        &mut self,
        request: ApproverAdminRequestV1,
    ) -> ApproverAdminResultV1 {
        if let Some(repo_root) = &self.repo_root {
            return crate::approver_registry_mutation::update_mapping(
                repo_root,
                &mut self.port,
                &request,
            );
        }
        self.closed_nonbootstrap(request, "update_mapping")
    }

    fn closed_nonbootstrap(
        &mut self,
        request: ApproverAdminRequestV1,
        expected_operation: &str,
    ) -> ApproverAdminResultV1 {
        if request.operation() != expected_operation {
            return refused(
                &request,
                "invalid_request",
                "operation does not match endpoint",
                false,
            );
        }
        refused(
            &request,
            "authorization_refused",
            "no committed registry state was supplied to authorize this operation",
            false,
        )
    }
}

fn validate_admin_document(
    document: &Value,
    expected_schema_id: &str,
) -> Result<(), ApproverAdminJsonErrorV1> {
    if document.get("schema_id").and_then(Value::as_str) != Some(expected_schema_id) {
        return Err(ApproverAdminJsonErrorV1(
            "unexpected approver admin schema_id".to_owned(),
        ));
    }
    let schema: Value = serde_json::from_str(ADMIN_SCHEMA)
        .map_err(|_| ApproverAdminJsonErrorV1("invalid embedded admin schema".to_owned()))?;
    let validator = jsonschema::draft202012::options()
        .with_pattern_options(PatternOptions::regex())
        .build(&schema)
        .map_err(|_| ApproverAdminJsonErrorV1("invalid embedded admin validator".to_owned()))?;
    if validator.is_valid(document) {
        Ok(())
    } else {
        Err(ApproverAdminJsonErrorV1(
            "approver admin document violates its closed schema".to_owned(),
        ))
    }
}

fn validate_mapping_uniqueness(document: &Value) -> Result<(), ApproverAdminJsonErrorV1> {
    let field = if document["operation"] == "bootstrap" {
        "initial_charter_quorum"
    } else {
        "approval_mappings"
    };
    let Some(mappings) = document.get(field).and_then(Value::as_array) else {
        return Ok(());
    };
    let mut pairs = BTreeSet::new();
    for mapping in mappings {
        let pair = (
            mapping["approval_class"].as_str().unwrap_or_default(),
            mapping["authority_ref"].as_str().unwrap_or_default(),
        );
        if !pairs.insert(pair) {
            return Err(ApproverAdminJsonErrorV1(
                "duplicate approval mapping".to_owned(),
            ));
        }
    }
    Ok(())
}

fn bootstrap_client_data_hash(
    request: &ApproverAdminRequestV1,
    user_handle_hash: &str,
) -> Result<[u8; 32], getrandom::Error> {
    let mut nonce = [0u8; 32];
    getrandom::fill(&mut nonce)?;
    let mut initial_charter_quorum = request.document["initial_charter_quorum"]
        .as_array()
        .expect("validated bootstrap quorum")
        .clone();
    sort_mappings(&mut initial_charter_quorum);
    let mut approval_mappings = initial_charter_quorum.clone();
    approval_mappings.push(json!({
        "approval_class": "registry_admin",
        "authority_ref": "repository_registry_admin"
    }));
    sort_mappings(&mut approval_mappings);
    let registration_request = json!({
        "$schema": "https://json-schema.org/draft/2020-12/schema",
        "allowed_algorithms": ["ES256"],
        "approval_mappings": approval_mappings,
        "attestation_policy": "none",
        "extensions": {},
        "initial_charter_quorum": initial_charter_quorum,
        "nonce_base64": BASE64_STANDARD.encode(nonce),
        "operation": "bootstrap",
        "operation_id": request.operation_id(),
        "protocol_id": "fido2-ctap2.1-native",
        "repository_identity_fingerprint": request.repository_identity_fingerprint(),
        "rp": {"id": "handbook.local", "name": "Handbook repository authority"},
        "schema_id": "handbook.authenticator-registration-request",
        "schema_version": "1.0",
        "user_handle_hash": user_handle_hash,
        "user_verification": "required"
    });
    let canonical = serde_json_canonicalizer::to_vec(&registration_request)
        .expect("JSON-only registration request is canonicalizable");
    Ok(Sha256::digest(canonical).into())
}

fn sort_mappings(mappings: &mut [Value]) {
    mappings.sort_by(|left, right| {
        left["approval_class"]
            .as_str()
            .expect("validated approval class")
            .cmp(
                right["approval_class"]
                    .as_str()
                    .expect("validated approval class"),
            )
            .then_with(|| {
                left["authority_ref"]
                    .as_str()
                    .expect("validated authority ref")
                    .cmp(
                        right["authority_ref"]
                            .as_str()
                            .expect("validated authority ref"),
                    )
            })
    });
}

fn refused_for_port_error(
    request: &ApproverAdminRequestV1,
    error: NativeAuthenticatorPortErrorV1,
) -> ApproverAdminResultV1 {
    match error {
        NativeAuthenticatorPortErrorV1::Unavailable | NativeAuthenticatorPortErrorV1::Transport => {
            refused(
                request,
                "AUTHENTICATOR_UNAVAILABLE",
                "native CTAP2.1 authenticator API is unavailable",
                true,
            )
        }
    }
}

fn refused(
    request: &ApproverAdminRequestV1,
    code: &str,
    message: &str,
    retryable: bool,
) -> ApproverAdminResultV1 {
    ApproverAdminResultV1::checked(json!({
        "assertion_fingerprint": null,
        "assertion_ref": null,
        "changed_paths": [],
        "next_actions": [if retryable { "retry through a new explicit operation" } else { "correct the request or committed authority before retry" }],
        "operation": request.operation(),
        "operation_id": request.operation_id(),
        "prior_registry_state_fingerprint": null,
        "prior_registry_state_ref": null,
        "refusal": {"code": code, "message": message, "retryable": retryable},
        "registration_fingerprint": null,
        "registration_ref": null,
        "repository_identity_fingerprint": request.repository_identity_fingerprint(),
        "result_registry_state_fingerprint": null,
        "result_registry_state_ref": null,
        "schema_id": "handbook.approver-admin-result",
        "schema_version": "1.0",
        "status": "refused",
        "transition_fingerprint": null,
        "transition_ref": null
    }))
}
