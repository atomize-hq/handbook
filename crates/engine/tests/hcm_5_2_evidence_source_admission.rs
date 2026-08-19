use handbook_engine::{
    effective_resolution_minimum_v1, resolution_meets_minimum_v1, ContextResolutionPolicyRegistry,
    ContextResolutionStackDefinition, EvidenceSourceAdmissionRequestV1,
    EvidenceSourceAdmissionServiceV1, NativeAuthenticatorPortErrorV1, NativeAuthenticatorPortV1,
    ResolutionObservationV1, SourceCurrentnessObservationV1, SourceCurrentnessRequestV1,
    CONTRACT_EVIDENCE_SOURCE_APPROVAL_CLASS_V1,
};
use std::path::Path;

const FP_A: &str = "sha256:aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa";
const FP_B: &str = "sha256:bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb";
const STACK_PATH: &str =
    "definitions/context-resolution/handbook.context-resolution.shipped-root/1.0.0.yaml";
const POLICY_PATHS: [&str; 3] = [
    "definitions/context-resolution-policies/handbook.mutation-matcher.core/1.0.0.yaml",
    "definitions/context-resolution-policies/handbook.resolution-escalation.core/1.0.0.yaml",
    "definitions/context-resolution-policies/handbook.memory-promotion.core/1.0.0.yaml",
];

#[test]
fn evidence_source_request_is_exact_bounded_and_class_fixed() {
    let request = EvidenceSourceAdmissionRequestV1::new(
        "producer/candidate/aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
        FP_A,
        "producer/ref",
        FP_B,
        "request/ref",
        FP_A,
        "execution/ref",
        FP_B,
    )
    .unwrap();

    assert_eq!(
        CONTRACT_EVIDENCE_SOURCE_APPROVAL_CLASS_V1,
        "contract_evidence_source"
    );
    assert_eq!(request.candidate_fingerprint(), FP_A);
    assert!(EvidenceSourceAdmissionRequestV1::new(
        "../candidate",
        FP_A,
        "producer/ref",
        FP_B,
        "request/ref",
        FP_A,
        "execution/ref",
        FP_B,
    )
    .is_err());
    assert!(EvidenceSourceAdmissionRequestV1::new(
        "candidate/ref",
        FP_A,
        "producer/ref",
        &format!("sha256:{}", "A".repeat(64)),
        "request/ref",
        FP_A,
        "execution/ref",
        FP_B,
    )
    .is_err());
}

#[test]
fn effective_resolution_is_exact_dimensionwise_minimum_under_registered_stack() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let policies =
        ContextResolutionPolicyRegistry::load(root, &POLICY_PATHS.map(str::to_string)).unwrap();
    let stack = ContextResolutionStackDefinition::load(root, STACK_PATH, &policies).unwrap();
    let stack_ref = stack.exact_ref().as_str();
    let stack_fingerprint = stack.definition_fingerprint().as_str();
    let wide = ResolutionObservationV1::new(
        stack_ref,
        stack_fingerprint,
        [
            "program",
            "full",
            "long_range",
            "program_policy",
            "strategic",
            "program_gate",
        ],
    )
    .unwrap();
    let narrow = ResolutionObservationV1::new(
        stack_ref,
        stack_fingerprint,
        [
            "assigned_unit",
            "normal",
            "immediate",
            "local_write",
            "execution",
            "unit_closeout",
        ],
    )
    .unwrap();

    let effective = effective_resolution_minimum_v1(&stack, &[wide, narrow]).unwrap();
    assert_eq!(
        effective.values(),
        [
            "assigned_unit",
            "normal",
            "immediate",
            "local_write",
            "execution",
            "unit_closeout",
        ]
    );

    let unknown = ResolutionObservationV1::new(
        stack_ref,
        stack_fingerprint,
        [
            "unknown",
            "normal",
            "immediate",
            "local_write",
            "execution",
            "unit_closeout",
        ],
    )
    .unwrap();
    assert!(effective_resolution_minimum_v1(&stack, &[unknown]).is_err());
    let substituted = ResolutionObservationV1::new(
        "handbook.context-resolution.other@1.0.0",
        stack_fingerprint,
        [
            "assigned_unit",
            "normal",
            "immediate",
            "local_write",
            "execution",
            "unit_closeout",
        ],
    )
    .unwrap();
    assert!(effective_resolution_minimum_v1(&stack, &[substituted]).is_err());

    let required = ResolutionObservationV1::new(
        stack_ref,
        stack_fingerprint,
        [
            "slice",
            "full",
            "current_slice",
            "slice_write",
            "coordination",
            "slice_closeout",
        ],
    )
    .unwrap();
    assert!(!resolution_meets_minimum_v1(&stack, &effective, &required).unwrap());
}

#[test]
fn generic_source_currentness_observation_cannot_change_the_requested_binding() {
    let request = SourceCurrentnessRequestV1::new("source/ref", FP_A, "revision-1").unwrap();
    let observation =
        SourceCurrentnessObservationV1::new(&request, "source/head", FP_B, "2026-08-19T12:34:56Z")
            .unwrap();
    assert_eq!(observation.source_ref(), "source/ref");
    assert_eq!(observation.source_fingerprint(), FP_A);
    assert_eq!(observation.revision(), "revision-1");
    assert!(
        SourceCurrentnessObservationV1::new(&request, "../head", FP_B, "2026-08-19T12:34:56Z",)
            .is_err()
    );
    assert!(SourceCurrentnessObservationV1::new(
        &request,
        "source/head",
        FP_B,
        "2026-08-19T12:34:56+00:00",
    )
    .is_err());
}

// The full authenticator-backed happy path is exercised through the contracts
// repository adapter. This local port keeps the Engine test target type-checked
// without introducing a credential bypass or test-only admission constructor.
struct UnusedPort;

impl NativeAuthenticatorPortV1 for UnusedPort {
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
fn evidence_source_service_has_no_unsealed_admission_constructor() {
    let service = EvidenceSourceAdmissionServiceV1::new(".", UnusedPort);
    let _port = service.into_port();
    let source = include_str!("../src/evidence_source_admission.rs");
    let declaration = source
        .split("pub struct EvidenceSourceAdmissionV1")
        .nth(1)
        .unwrap()
        .split("impl EvidenceSourceAdmissionV1")
        .next()
        .unwrap();
    assert!(!declaration.contains("Serialize"));
    assert!(!source.contains("impl Serialize for EvidenceSourceAdmissionV1"));
}
