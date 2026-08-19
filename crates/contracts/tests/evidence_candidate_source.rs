#![cfg(feature = "test-support")]

use handbook_contracts::{
    seed_committed_candidate_observation_for_testing_v1, CandidateObservationFixtureV1,
    ContentAddressedBindingV1, EvidenceCandidateSourceErrorKindV1, EvidenceCandidateSourceV1,
    ExactBindingV1, ExactRefV1, FingerprintV1, RepositoryEvidenceCandidateSourceV1,
};
use handbook_engine::{NativeAuthenticatorPortErrorV1, NativeAuthenticatorPortV1};

const FP_A: &str = "sha256:aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa";
const FP_B: &str = "sha256:bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb";

struct UnavailablePort;

impl NativeAuthenticatorPortV1 for UnavailablePort {
    fn make_credential(
        &mut self,
        _request_cbor: &[u8],
    ) -> Result<Vec<u8>, NativeAuthenticatorPortErrorV1> {
        Err(NativeAuthenticatorPortErrorV1::Unavailable)
    }

    fn get_assertion(
        &mut self,
        _request_cbor: &[u8],
    ) -> Result<Vec<u8>, NativeAuthenticatorPortErrorV1> {
        Err(NativeAuthenticatorPortErrorV1::Unavailable)
    }
}

#[test]
fn repository_source_resolves_only_exact_committed_observations_before_sealed_admission() {
    let repo = tempfile::tempdir().unwrap();
    let candidate = content(
        "producer/candidate/aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
        FP_A,
    );
    let fixture = CandidateObservationFixtureV1::new(
        candidate.clone(),
        br#"{"body":{},"candidate_fingerprint":"sha256:aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa","candidate_ref":"producer/candidate/aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa","schema_ref":"handbook.contract-evidence-candidate@1.0.0"}"#.to_vec(),
        br#"{"evaluation_run_fingerprint":"sha256:bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb","evaluation_run_ref":"handbook.contract.fixture@1.0.0#evaluation/bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb","schema_ref":"handbook.contract-evaluation-run@1.0.0"}"#.to_vec(),
        content("producer/ref", FP_B),
        content("request/ref", FP_A),
        content("execution/ref", FP_B),
        None,
    )
    .unwrap();
    seed_committed_candidate_observation_for_testing_v1(repo.path(), &fixture).unwrap();
    let mut source =
        RepositoryEvidenceCandidateSourceV1::open(repo.path(), UnavailablePort).unwrap();

    let refusal = source
        .resolve_exact(&candidate, |_| panic!("authority absence must not admit"))
        .unwrap_err();
    assert_eq!(
        refusal.kind(),
        EvidenceCandidateSourceErrorKindV1::AuthorityAbsent
    );

    let substituted = content(candidate.reference(), FP_B);
    let refusal = source
        .resolve_exact(&substituted, |_| panic!("substitution must not admit"))
        .unwrap_err();
    assert_eq!(
        refusal.kind(),
        EvidenceCandidateSourceErrorKindV1::CandidateSubstituted
    );
}

#[test]
fn repository_source_has_no_production_writer_or_executor_surface() {
    let source = include_str!("../src/evidence_candidate_source.rs");
    let production = source
        .split("#[cfg(feature = \"test-support\")]")
        .next()
        .unwrap();
    assert!(!production.contains("write_new_durable"));
    assert!(!production.contains("replace_file_durable"));
    assert!(!production.contains("execute"));
    assert!(!production.contains("dock"));
}

#[test]
fn repository_source_refuses_missing_malformed_and_over_limit_observations() {
    let repo = tempfile::tempdir().unwrap();
    let candidate = content(
        "producer/candidate/aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
        FP_A,
    );
    let observation = repo.path().join(
        ".handbook/state/contracts/v1/candidate-observations/aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa.json",
    );
    std::fs::create_dir_all(observation.parent().unwrap()).unwrap();
    let mut source =
        RepositoryEvidenceCandidateSourceV1::open(repo.path(), UnavailablePort).unwrap();
    assert_eq!(
        source.resolve_exact(&candidate, |_| ()).unwrap_err().kind(),
        EvidenceCandidateSourceErrorKindV1::Missing
    );

    std::fs::write(&observation, b"not-json").unwrap();
    assert_eq!(
        source.resolve_exact(&candidate, |_| ()).unwrap_err().kind(),
        EvidenceCandidateSourceErrorKindV1::Malformed
    );

    std::fs::write(&observation, vec![b' '; 1024 * 1024 + 1]).unwrap();
    assert_eq!(
        source.resolve_exact(&candidate, |_| ()).unwrap_err().kind(),
        EvidenceCandidateSourceErrorKindV1::OverLimit
    );
}

fn content(reference: &str, fingerprint: &str) -> ContentAddressedBindingV1 {
    ContentAddressedBindingV1::new(reference, FingerprintV1::parse(fingerprint).unwrap()).unwrap()
}

#[allow(dead_code)]
fn exact(reference: &str, fingerprint: &str) -> ExactBindingV1 {
    ExactBindingV1::new(
        ExactRefV1::parse(reference).unwrap(),
        FingerprintV1::parse(fingerprint).unwrap(),
    )
}
