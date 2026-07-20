use handbook_engine::{
    decode_and_verify_get_assertion_response, decode_make_credential_response,
    derive_authenticator_user_handle, encode_get_assertion_request, encode_make_credential_request,
    map_ctap_status, select_eligible_credentials, ApproverAdminRequestV1, ApproverAdminResultV1,
    ApproverRegistryServiceV1, AuthenticatorCredentialV1, CredentialSelectionCandidateV1,
    CtapStatusOutcomeV1, NativeAuthenticatorPortErrorV1, NativeAuthenticatorPortV1,
};
use serde_json::Value;
use std::{cell::RefCell, fs, path::PathBuf, rc::Rc};

const MAKE_CREDENTIAL_REQUEST_BASE64: &str = "AaUBWCDr1lx50B6SuuZimUIYxcnXfgIx1kprHEfUZXC8UktdOAKiYmlkbmhhbmRib29rLmxvY2FsZG5hbWV4HUhhbmRib29rIHJlcG9zaXRvcnkgYXV0aG9yaXR5A6NiaWRYIF1MyCCzfz0f0MbgTtUKVurY1Jfs/Twlx0mFXtnYUoN9ZG5hbWV4HWhhbmRib29rLXJlcG9zaXRvcnktYXV0aG9yaXR5a2Rpc3BsYXlOYW1leB1IYW5kYm9vayByZXBvc2l0b3J5IGF1dGhvcml0eQSBomNhbGcmZHR5cGVqcHVibGljLWtleQeiYnJr9GJ1dvU=";

fn contract(name: &str) -> Value {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../docs/specs/handbook-contract-membrane/slices/HCM-2.2/contracts")
        .join(name);
    serde_json::from_slice(&fs::read(path).unwrap()).unwrap()
}

fn decode_hex(value: &str) -> Vec<u8> {
    value
        .as_bytes()
        .chunks_exact(2)
        .map(|pair| {
            let digit = |byte: u8| match byte {
                b'0'..=b'9' => byte - b'0',
                b'a'..=b'f' => byte - b'a' + 10,
                _ => panic!("non-lowercase-hex test fixture"),
            };
            digit(pair[0]) << 4 | digit(pair[1])
        })
        .collect()
}

fn decode_base64(value: &str) -> Vec<u8> {
    let sextet = |byte: u8| match byte {
        b'A'..=b'Z' => byte - b'A',
        b'a'..=b'z' => byte - b'a' + 26,
        b'0'..=b'9' => byte - b'0' + 52,
        b'+' => 62,
        b'/' => 63,
        _ => panic!("invalid base64 test fixture"),
    };
    let mut output = Vec::new();
    for chunk in value.as_bytes().chunks_exact(4) {
        let a = sextet(chunk[0]);
        let b = sextet(chunk[1]);
        output.push(a << 2 | b >> 4);
        if chunk[2] != b'=' {
            let c = sextet(chunk[2]);
            output.push(b << 4 | c >> 2);
            if chunk[3] != b'=' {
                output.push(c << 6 | sextet(chunk[3]));
            }
        }
    }
    output
}

fn fingerprint_bytes(value: &str) -> [u8; 32] {
    decode_hex(value.strip_prefix("sha256:").unwrap())
        .try_into()
        .unwrap()
}

fn bootstrap_cose_key(transcripts: &Value) -> Vec<u8> {
    decode_base64(
        transcripts["vectors"][0]["make_credential_response"]["cose_public_key_base64"]
            .as_str()
            .unwrap(),
    )
}

fn assertion_credential(transcripts: &Value) -> AuthenticatorCredentialV1 {
    AuthenticatorCredentialV1 {
        credential_id: decode_base64(
            transcripts["vectors"][0]["make_credential_response"]["credential_id_base64"]
                .as_str()
                .unwrap(),
        ),
        cose_public_key: bootstrap_cose_key(transcripts),
    }
}

#[test]
fn user_handle_derivation_matches_every_frozen_vector() {
    let vectors = contract("user-handle-derivation-vectors-v1.0.json");
    for vector in vectors["vectors"].as_array().unwrap() {
        let derived = derive_authenticator_user_handle(
            vector["repository_identity_fingerprint"].as_str().unwrap(),
        )
        .unwrap();
        assert_eq!(
            derived.bytes.as_slice(),
            decode_hex(vector["user_handle_bytes_hex"].as_str().unwrap())
        );
        assert_eq!(derived.hash, vector["user_handle_hash"]);
    }

    for invalid in [
        "1111111111111111111111111111111111111111111111111111111111111111",
        "SHA256:1111111111111111111111111111111111111111111111111111111111111111",
        "sha256:AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA",
        "sha256:1111",
    ] {
        assert!(derive_authenticator_user_handle(invalid).is_err());
    }
}

#[test]
fn engine_builds_exact_closed_make_credential_request() {
    let client_data_hash = fingerprint_bytes(
        "sha256:ebd65c79d01e92bae662994218c5c9d77e0231d64a6b1c47d46570bc524b5d38",
    );
    let user = derive_authenticator_user_handle(
        "sha256:1111111111111111111111111111111111111111111111111111111111111111",
    )
    .unwrap();
    let encoded = encode_make_credential_request(client_data_hash, user.bytes).unwrap();
    assert_eq!(encoded, decode_base64(MAKE_CREDENTIAL_REQUEST_BASE64));
}

#[test]
fn every_assertion_request_and_success_response_matches_the_frozen_transcript() {
    let transcripts = contract("authenticator-transcript-vectors-v1.0.json");
    let credential = assertion_credential(&transcripts);

    for vector in transcripts["vectors"]
        .as_array()
        .unwrap()
        .iter()
        .filter(|vector| vector["kind"] == "assertion")
    {
        let client_data_hash = fingerprint_bytes(vector["client_data_hash"].as_str().unwrap());
        let request = encode_get_assertion_request(
            client_data_hash,
            std::slice::from_ref(&credential.credential_id),
        )
        .unwrap();
        assert_eq!(
            request,
            decode_base64(
                vector["get_assertion_request_cbor_base64"]
                    .as_str()
                    .unwrap()
            )
        );

        let raw = decode_base64(
            vector["get_assertion_response"]["raw_response_base64"]
                .as_str()
                .unwrap(),
        );
        let decoded = decode_and_verify_get_assertion_response(
            &raw,
            std::slice::from_ref(&credential),
            client_data_hash,
        )
        .unwrap();
        assert_eq!(decoded.credential_id, credential.credential_id);
        assert_eq!(decoded.authenticator_data.len(), 37);
        assert_eq!(decoded.flags_byte, 0x05);
        assert_eq!(decoded.sign_count, 7);
        assert!(!decoded.attested_credential_data_included);
        assert!(!decoded.extensions_included);
    }
}

#[test]
fn every_malformed_or_status_assertion_response_refuses() {
    let transcripts = contract("authenticator-transcript-vectors-v1.0.json");
    let credential = assertion_credential(&transcripts);
    let client_data_hash = fingerprint_bytes(
        transcripts["vectors"][2]["client_data_hash"]
            .as_str()
            .unwrap(),
    );

    for vector in transcripts["assertion_response_negative_vectors"]
        .as_array()
        .unwrap()
    {
        let raw = decode_base64(vector["raw_transport_response_base64"].as_str().unwrap());
        assert!(
            decode_and_verify_get_assertion_response(
                &raw,
                std::slice::from_ref(&credential),
                client_data_hash,
            )
            .is_err(),
            "negative vector {} was admitted",
            vector["vector_id"]
        );
    }
}

#[test]
fn make_credential_success_is_strict_and_byte_closed() {
    let transcripts = contract("authenticator-transcript-vectors-v1.0.json");
    for vector in &transcripts["vectors"].as_array().unwrap()[..2] {
        let attestation = decode_base64(
            vector["make_credential_response"]["attestation_object_base64"]
                .as_str()
                .unwrap(),
        );
        let mut raw = vec![0x00];
        raw.extend_from_slice(&attestation);
        let decoded = decode_make_credential_response(&raw).unwrap();
        assert_eq!(decoded.flags_byte, 0x45);
        assert_eq!(
            decoded.credential_id_hash,
            vector["make_credential_response"]["credential_id_hash"]
        );
        assert_eq!(
            decoded.cose_public_key,
            decode_base64(
                vector["make_credential_response"]["cose_public_key_base64"]
                    .as_str()
                    .unwrap()
            )
        );

        raw.push(0);
        assert!(decode_make_credential_response(&raw).is_err());
    }
}

#[test]
fn received_status_mapping_is_total_disjoint_and_exact() {
    let fixture = contract("ctap-status-mapping-v1.0.json");
    let groups = fixture["refusal_groups"].as_array().unwrap();
    let mut seen = [false; 256];

    assert!(matches!(map_ctap_status(0), CtapStatusOutcomeV1::Success));
    for group in groups {
        for status in group["status_bytes"].as_array().unwrap() {
            let status = status.as_u64().unwrap() as u8;
            assert!(!seen[usize::from(status)]);
            seen[usize::from(status)] = true;
            let CtapStatusOutcomeV1::Refused(mapped) = map_ctap_status(status) else {
                panic!("nonzero status mapped to success");
            };
            assert_eq!(mapped.code.as_str(), group["refusal_code"]);
            assert_eq!(mapped.retryable, group["retryable"]);
            assert_eq!(mapped.next_action, group["next_action"]);
        }
    }
    assert!(seen[1..].iter().all(|mapped| *mapped));
}

#[test]
fn credential_selection_is_deterministic_bounded_and_fail_closed() {
    let low = vec![0; 32];
    let high = vec![0xff; 32];
    let selected = select_eligible_credentials(&[
        CredentialSelectionCandidateV1::eligible(high.clone()),
        CredentialSelectionCandidateV1::eligible(low.clone()),
        CredentialSelectionCandidateV1::inactive(vec![0x11; 32]),
        CredentialSelectionCandidateV1::exhausted(vec![0x22; 32]),
    ])
    .unwrap();
    assert_eq!(selected, vec![low, high]);

    assert!(select_eligible_credentials(&[
        CredentialSelectionCandidateV1::eligible(vec![0x33; 32]),
        CredentialSelectionCandidateV1::eligible(vec![0x33; 32]),
    ])
    .is_err());
    assert!(select_eligible_credentials(&vec![
        CredentialSelectionCandidateV1::eligible(vec![
            0x44;
            32
        ]);
        65
    ])
    .is_err());
}

#[test]
fn approver_admin_json_projection_accepts_all_positives_and_rejects_all_negatives() {
    let fixture = contract("approver-admin-api-vectors-v1.0.json");
    for vector in fixture["vectors"].as_array().unwrap() {
        let document = vector["document"].clone();
        match document["schema_id"].as_str().unwrap() {
            "handbook.approver-admin-request" => {
                let parsed = ApproverAdminRequestV1::from_json_value(document.clone()).unwrap();
                assert_eq!(parsed.to_json_value().unwrap(), document);
            }
            "handbook.approver-admin-result" => {
                let parsed = ApproverAdminResultV1::from_json_value(document.clone()).unwrap();
                assert_eq!(parsed.to_json_value().unwrap(), document);
            }
            other => panic!("unexpected fixture schema {other}"),
        }
    }

    for vector in fixture["negative_vectors"].as_array().unwrap() {
        let document = vector["document"].clone();
        let admitted = if document["schema_id"] == "handbook.approver-admin-request" {
            ApproverAdminRequestV1::from_json_value(document).is_ok()
        } else {
            ApproverAdminResultV1::from_json_value(document).is_ok()
        };
        assert!(
            !admitted,
            "negative vector {} was admitted",
            vector["vector_id"]
        );
    }
}

#[derive(Clone)]
struct UnavailablePort {
    calls: Rc<RefCell<Vec<Vec<u8>>>>,
}

impl NativeAuthenticatorPortV1 for UnavailablePort {
    fn make_credential(
        &mut self,
        request_cbor: &[u8],
    ) -> Result<Vec<u8>, NativeAuthenticatorPortErrorV1> {
        self.calls.borrow_mut().push(request_cbor.to_vec());
        Err(NativeAuthenticatorPortErrorV1::Unavailable)
    }

    fn get_assertion(
        &mut self,
        request_cbor: &[u8],
    ) -> Result<Vec<u8>, NativeAuthenticatorPortErrorV1> {
        self.calls.borrow_mut().push(request_cbor.to_vec());
        Err(NativeAuthenticatorPortErrorV1::Unavailable)
    }
}

#[test]
fn unavailable_port_returns_closed_all_null_refusal() {
    let fixture = contract("approver-admin-api-vectors-v1.0.json");
    let request =
        ApproverAdminRequestV1::from_json_value(fixture["vectors"][0]["document"].clone()).unwrap();
    let calls = Rc::new(RefCell::new(Vec::new()));
    let mut service = ApproverRegistryServiceV1::new(UnavailablePort {
        calls: Rc::clone(&calls),
    });

    let result = service.bootstrap_approver_registry(request);
    let document = result.to_json_value().unwrap();
    assert_eq!(document["status"], "refused");
    assert_eq!(document["refusal"]["code"], "AUTHENTICATOR_UNAVAILABLE");
    assert_eq!(document["changed_paths"], serde_json::json!([]));
    for field in [
        "prior_registry_state_ref",
        "prior_registry_state_fingerprint",
        "result_registry_state_ref",
        "result_registry_state_fingerprint",
        "transition_ref",
        "transition_fingerprint",
        "registration_ref",
        "registration_fingerprint",
        "assertion_ref",
        "assertion_fingerprint",
    ] {
        assert!(document[field].is_null(), "{field} leaked on refusal");
    }
    assert_eq!(calls.borrow().len(), 1);
}
