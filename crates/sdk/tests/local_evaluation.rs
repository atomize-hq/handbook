//! Packet 3 supplied inputs and owner composition. Fixture approvals are
//! illustrative; the normal-consumer export supplies no review outcome.
#[allow(dead_code)]
mod local_contract_support;
use handbook_contracts::*;
use handbook_sdk::local_check::*;
use handbook_sdk::local_evaluation::*;
use handbook_sdk::local_setup::*;
use handbook_sdk::HandbookSdkV1;
use local_contract_support::*;

fn supplied_full_initial() -> LocalInitialSetupRequestV1 {
    let mut request = supplied_initial_request();
    request
        .terms
        .allowed_paths
        .push(".handbook/local-operating/local-evaluation-v1/**".into());
    request
        .terms
        .allowed_paths
        .push(".handbook/state/locks/**".into());
    request.terms.allowed_paths.sort();
    request.terms.allowed_input_paths = supplied_full_read_paths();
    request.terms.approved_content_fingerprint =
        request.terms.fingerprint_approved_content().unwrap();
    request
}
fn supplied_full_read_paths() -> Vec<String> {
    [
        ".handbook-initialization-receipt.v1.json",
        ".handbook-initialization.lock",
        ".handbook-initialization.v1.json",
        ".handbook/**",
        "handbook-docks/**",
        "inputs/*",
    ]
    .map(str::to_owned)
    .to_vec()
}
fn supplied_full_predecessor(
    sdk: &HandbookSdkV1,
) -> (
    LocalInitialSetupLocatorV1,
    LocalCheckLocatorV1,
    LocalCheckAuthorityOutcomeV1,
) {
    let request = supplied_full_initial();
    let prepared = sdk.prepare_local_initial_setup(&request).unwrap();
    let initial = prepared.locator().clone();
    let output = sdk.establish_local_initial_setup(prepared);
    assert!(output.stopped.is_none(), "{output:?}");
    let original = output.terms.unwrap().outcome.unwrap();
    let mut terms = supplied_successor_terms(&initial, &original.outcome_fingerprint);
    terms.allowed_paths = initial.terms.terms.allowed_paths.clone();
    terms.allowed_input_paths = supplied_full_read_paths();
    terms.approved_content_fingerprint = terms.fingerprint_approved_content().unwrap();
    let prepared = sdk.prepare_local_check_terms(&initial, terms).unwrap();
    let locator = prepared.locator().clone();
    let execution = sdk.adopt_local_check_terms(prepared);
    let authority = execution
        .outcome
        .unwrap_or_else(|| panic!("{:?}", execution.refusal));
    (initial, locator, authority)
}
fn supplied_full_definition(
    authority: &LocalCheckAuthorityOutcomeV1,
) -> (
    LocalContractDefinitionV3,
    handbook_engine::ContextResolutionStackDefinition,
) {
    let (intrinsic, stack) = supplied_definition_with_version(
        LocalPermittedUseV2::DecisionSeekingEligible,
        3600,
        author_authority(authority),
        GateEffectV1::Required,
        "2.0.0",
    );
    let dependencies =
        LocalDefinitionDependenciesV3::new(intrinsic.dependencies().clone()).unwrap();
    (
        LocalContractDefinitionV3::new(
            LocalContractDefinitionInputV3 {
                intrinsic_definition: intrinsic,
                dependencies,
            },
            &stack,
        )
        .unwrap(),
        stack,
    )
}

#[test]
fn full_check_definition_has_new_exact_review_subject_and_preserves_v2_bytes() {
    let root = tempfile::tempdir().unwrap();
    let sdk = HandbookSdkV1::open(root.path());
    let (initial, check, authority) = supplied_full_predecessor(&sdk);
    let (definition, stack) = supplied_full_definition(&authority);
    let (old, _) = supplied_definition(
        LocalPermittedUseV2::DecisionSeekingEligible,
        3600,
        author_authority(&authority),
    );
    let old_bytes = old.canonical_json_bytes().unwrap();
    assert_ne!(old.binding(), definition.binding());
    assert_ne!(old.closure_fingerprint(), definition.closure_fingerprint());
    assert_eq!(
        LocalContractDefinitionV2::from_json_bytes(&old_bytes)
            .unwrap()
            .canonical_json_bytes()
            .unwrap(),
        old_bytes
    );
    let prepared = sdk
        .prepare_local_evaluation_definition(&initial, &check, definition.clone(), &stack)
        .unwrap();
    let authored = sdk
        .append_local_evaluation_definition(
            &check,
            prepared,
            SemanticTimeV1::owner_observation_now_v1().unwrap(),
            "packet3-full-author-01",
        )
        .unwrap();
    assert_eq!(authored.lifecycle_head.state(), LifecycleStateV1::Draft);
    assert!(
        sdk.prepare_local_evaluation_use(
            &initial,
            &definition.binding(),
            LocalContractUseIntentV2::DecisionSeeking
        )
        .is_err(),
        "authored definition still needs actual distinct review and activation"
    );
    assert_eq!(
        sdk.read_local_evaluation_definition(&definition.binding())
            .unwrap()
            .definition(),
        &definition
    );
}

#[test]
#[ignore = "exports supplied approvals and exact candidate for subsequent actual independent review"]
fn export_supplied_full_normal_consumer_inputs() {
    let output = std::path::PathBuf::from(
        std::env::var_os("HANDBOOK_PACKET3_EXPORT_DIR").expect("supply export directory"),
    );
    assert!(output.is_absolute());
    assert!(!output.exists(), "never overwrite earlier proof inputs");
    std::fs::create_dir_all(&output).unwrap();
    let root = output.join("project");
    std::fs::create_dir(&root).unwrap();
    let sdk = HandbookSdkV1::open(&root);
    let (initial, check, authority) = supplied_full_predecessor(&sdk);
    let (definition, _) = supplied_full_definition(&authority);
    let mut evaluation_terms = LocalEvaluationTermsV1 {
        request_key:"packet3-supplied-evaluation-terms".into(),rule_ref:LOCAL_EVALUATION_RULE_V1.into(),
        principal:check.terms.principal.clone(),agent:check.terms.agent.clone(),delegation:"One bounded local advisory evaluation per exact owner occurrence.".into(),
        instruction:"Evaluate supplied captured orders with the reviewed full-check rule and explain local findings.".into(),
        approval_source_ref:"illustrative.supplied.packet3.evaluation-approval".into(),human_approval_asserted:true,approved_content_fingerprint:String::new(),
        expected_predecessor_fingerprint:authority.outcome_fingerprint.clone(),expected_charter_fingerprint:initial.charter.canonical_fingerprint.clone(),
        ceiling:check.terms.ceiling.clone(),constraint_inputs:check.terms.constraint_inputs.clone(),allowed_paths:check.terms.allowed_paths.clone(),allowed_input_paths:check.terms.allowed_input_paths.clone(),
        allowed_use_intents:vec![LocalEvaluationUseIntentV1::DecisionSeeking,LocalEvaluationUseIntentV1::Diagnostic],
        required_checkpoints:handbook_engine::local_authority::local_evaluation::local_evaluation_required_checkpoints_v1(),
        retention_until_utc:"2026-11-06T00:00:00Z".into(),revocation_principal:check.terms.revocation_principal.clone(),
    };
    evaluation_terms.approved_content_fingerprint =
        evaluation_terms.fingerprint_approved_content().unwrap();
    let mut violation = supplied_validation_content();
    violation["instance"]["value"]["amount"] = serde_json::json!(-1);
    violation["instance"]["instance_fingerprint"] =
        serde_json::json!(value_fingerprint(&violation["instance"]["value"]));
    let manifest = production_manifest();
    let resolution = serde_json::to_value(manifest.resolution_ceiling()).unwrap();
    let dimensions = [
        "scope_horizon",
        "detail_resolution",
        "temporal_horizon",
        "authority_horizon",
        "memory_horizon",
        "validation_horizon",
    ]
    .map(|name| {
        resolution[name]
            .as_str()
            .unwrap()
            .splitn(3, '.')
            .nth(2)
            .unwrap()
            .replace('-', "_")
    });
    let requested_reach = LocalEvaluationReachV1 {
        profile: handbook_engine::local_authority::LocalBindingV1 {
            reference: handbook_engine::local_authority::resolved_local_operating_profile()
                .unwrap()
                .exact_ref()
                .to_string(),
            fingerprint: handbook_engine::local_authority::resolved_local_operating_profile()
                .unwrap()
                .resolved_profile_fingerprint()
                .to_string(),
        },
        stack: handbook_engine::local_authority::LocalBindingV1 {
            reference: manifest
                .resolution_ceiling()
                .resolution_stack()
                .reference()
                .as_str()
                .into(),
            fingerprint: manifest
                .resolution_ceiling()
                .resolution_stack()
                .fingerprint()
                .as_str()
                .into(),
        },
        active_level_id: "execution".into(),
        dimensions,
    };
    let reach = LocalEvaluationPathReachV1 {
        owner_write_paths: [
            ".handbook/local-operating/local-evaluation-v1",
            ".handbook/state/contracts",
            ".handbook/state/dock",
            ".handbook/state/locks",
        ]
        .map(str::to_owned)
        .to_vec(),
        input_paths: supplied_full_read_paths(),
    };
    let question = serde_json::json!({"request_key":"packet3-ordinary-pass","objective":"Does this exact captured order conform to its exact supplied offline JSON Schema closure?","scope_fingerprint":value_fingerprint(&serde_json::json!({"contract":definition.binding(),"scope":"one captured order"})),"use_intent":"decision_seeking","requested_reach":requested_reach,"requested_resolution":manifest.resolution_ceiling(),"reach":reach,"retention_until_utc":evaluation_terms.retention_until_utc});
    for (name, bytes) in [
        (
            "initial-request.json",
            serde_json_canonicalizer::to_vec(&supplied_full_initial()).unwrap(),
        ),
        ("initial.json", initial.canonical_json_bytes().unwrap()),
        (
            "check-terms.json",
            check.terms.canonical_json_bytes().unwrap(),
        ),
        ("check.json", check.canonical_json_bytes().unwrap()),
        (
            "definition.json",
            definition.canonical_json_bytes().unwrap(),
        ),
        (
            "binding.json",
            serde_json_canonicalizer::to_vec(&definition.binding()).unwrap(),
        ),
        (
            "validation-pass.json",
            serde_json_canonicalizer::to_vec(&supplied_validation_content()).unwrap(),
        ),
        (
            "validation-fail.json",
            serde_json_canonicalizer::to_vec(&violation).unwrap(),
        ),
        (
            "evaluation-terms.json",
            evaluation_terms.canonical_json_bytes().unwrap(),
        ),
        (
            "adoption-basis.json",
            serde_json_canonicalizer::to_vec(&sdk.observe_local_check_basis(&initial).unwrap())
                .unwrap(),
        ),
        (
            "question-pass.json",
            serde_json_canonicalizer::to_vec(&question).unwrap(),
        ),
        (
            "reach.json",
            serde_json_canonicalizer::to_vec(&reach).unwrap(),
        ),
        (
            "dock-reach.json",
            serde_json_canonicalizer::to_vec(&handbook_dock::DockLocalEvaluationReachV1 {
                read_paths: vec![".handbook/state/**".into(), "handbook-docks/**".into()],
                write_subtrees: vec![
                    ".handbook/state/dock/**".into(),
                    ".handbook/state/locks/dock-operational-store.lock".into(),
                ],
            })
            .unwrap(),
        ),
    ] {
        std::fs::write(output.join(name), bytes).unwrap();
    }
    println!(
        "exact candidate path={} definition={} closure={}",
        output.display(),
        definition.binding().fingerprint().as_str(),
        definition.closure_fingerprint().as_str()
    );
    // Review is obtained after export. No author or lifecycle record is seeded.
}
