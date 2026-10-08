//! Packet 3 owner integration. Supplied approvals and distinct review outcomes
//! below are illustrative test inputs; every permission and origin is resolved
//! by the actual current native owners.
#[allow(dead_code)]
mod local_contract_support;

use handbook_contracts::*;
use handbook_dock::*;
use handbook_engine::local_authority::local_evaluation::{
    local_evaluation_required_checkpoints_v1, LocalEvaluationFinalSetSourceV1,
    NativeLocalEvaluationAuthorityV1,
};
use handbook_sdk::local_evaluation::*;
use handbook_sdk::local_setup::*;
use handbook_sdk::HandbookSdkV1;
use local_contract_support::*;
use serde_json::{json, Value};

const HORIZON: &str = "2098-01-01T00:00:00Z";

fn on_stack(test: impl FnOnce() + Send + 'static) {
    std::thread::Builder::new()
        .stack_size(8 * 1024 * 1024)
        .spawn(test)
        .unwrap()
        .join()
        .unwrap();
}

fn read_paths() -> Vec<String> {
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

fn engine_reach() -> LocalEvaluationPathReachV1 {
    LocalEvaluationPathReachV1 {
        owner_write_paths: [
            ".handbook/local-operating/local-evaluation-v1",
            ".handbook/state/contracts",
            ".handbook/state/dock",
            ".handbook/state/locks",
        ]
        .map(str::to_owned)
        .to_vec(),
        input_paths: read_paths(),
    }
}

fn dock_reach() -> DockLocalEvaluationReachV1 {
    DockLocalEvaluationReachV1 {
        read_paths: vec![".handbook/**".into(), "handbook-docks/**".into()],
        write_subtrees: vec![
            ".handbook/state/dock/**".into(),
            ".handbook/state/locks/**".into(),
        ],
    }
}

enum NativeFixtureDirectory {
    Temporary(tempfile::TempDir),
    Retained(std::path::PathBuf),
}

impl NativeFixtureDirectory {
    fn new() -> Self {
        Self::Temporary(tempfile::tempdir().unwrap())
    }

    fn retained(path: std::path::PathBuf) -> Self {
        assert!(path.is_absolute() && path.is_dir());
        Self::Retained(path)
    }

    fn path(&self) -> &std::path::Path {
        match self {
            Self::Temporary(directory) => directory.path(),
            Self::Retained(path) => path,
        }
    }
}

impl Drop for NativeFixtureDirectory {
    fn drop(&mut self) {
        if std::thread::panicking() {
            if let Self::Temporary(directory) = self {
                directory.disable_cleanup(true);
            }
            eprintln!(
                "native fixture retained at original physical path: {}",
                self.path().display()
            );
        }
    }
}

struct Fixture {
    root: NativeFixtureDirectory,
    sdk: HandbookSdkV1,
    initial: LocalInitialSetupLocatorV1,
    definition: LocalContractDefinitionV3,
    authority: LocalEvaluationAuthorityLocatorV1,
    set_sources:
        std::cell::RefCell<std::collections::BTreeMap<String, LocalEvaluationFinalSetSourceV1>>,
    checkpoint_artifacts: NativeFixtureDirectory,
}

impl Fixture {
    fn from_public_handoff(path: &std::path::Path) -> Self {
        let bytes = std::fs::read(path).unwrap();
        let handoff: Value = serde_json::from_slice(&bytes).unwrap();
        assert_eq!(serde_json_canonicalizer::to_vec(&handoff).unwrap(), bytes);
        let root =
            NativeFixtureDirectory::retained(handoff["repository_path"].as_str().unwrap().into());
        let sdk = HandbookSdkV1::open(root.path());
        Self {
            root,
            sdk,
            initial: serde_json::from_value(handoff["initial"].clone()).unwrap(),
            definition: serde_json::from_value(handoff["definition"].clone()).unwrap(),
            authority: serde_json::from_value(handoff["authority"].clone()).unwrap(),
            set_sources: std::cell::RefCell::new(std::collections::BTreeMap::new()),
            checkpoint_artifacts: NativeFixtureDirectory::retained(path.parent().unwrap().into()),
        }
    }

    fn new() -> Self {
        Self::new_with_content(supplied_validation_content())
    }

    fn new_with_content(content: Value) -> Self {
        Self::new_with_applicability(content, None)
    }

    fn new_with_applicability(content: Value, subject_role: Option<&str>) -> Self {
        eprintln!("native fixture: initial setup");
        let root = NativeFixtureDirectory::new();
        let sdk = HandbookSdkV1::open(root.path());
        let mut input = supplied_initial_request();
        input.terms.allowed_paths.extend([
            ".handbook/local-operating/local-evaluation-v1/**".into(),
            ".handbook/state/locks/**".into(),
        ]);
        input.terms.allowed_paths.sort();
        input.terms.allowed_input_paths = read_paths();
        input.terms.approved_content_fingerprint =
            input.terms.fingerprint_approved_content().unwrap();
        let prepared = sdk.prepare_local_initial_setup(&input).unwrap();
        let initial = prepared.locator().clone();
        let established = sdk.establish_local_initial_setup(prepared);
        assert!(established.stopped.is_none(), "{established:?}");
        let original = established.terms.unwrap().outcome.unwrap();
        let mut terms = supplied_successor_terms(&initial, &original.outcome_fingerprint);
        terms.allowed_paths = initial.terms.terms.allowed_paths.clone();
        terms.allowed_input_paths = read_paths();
        terms.approved_content_fingerprint = terms.fingerprint_approved_content().unwrap();
        let prepared = sdk.prepare_local_check_terms(&initial, terms).unwrap();
        let check = prepared.locator().clone();
        let established = sdk.adopt_local_check_terms(prepared);
        let original = established
            .outcome
            .unwrap_or_else(|| panic!("{:?}", established.refusal));
        eprintln!("native fixture: exact definition and lifecycle");
        let (original_intrinsic, stack) = supplied_definition_with_version(
            LocalPermittedUseV2::DecisionSeekingEligible,
            3600,
            author_authority(&original),
            GateEffectV1::Required,
            "2.0.0",
        );
        let document = LocalSchemaDocumentV2::new(
            content["root_document"].as_str().unwrap().into(),
            content["documents"][0]["schema"].clone(),
        )
        .unwrap();
        let root_schema = ExactBindingV1::new(
            ExactRefV1::parse(content["root_schema"]["ref"].as_str().unwrap()).unwrap(),
            document.document_fingerprint.clone(),
        );
        let mut dependencies = original_intrinsic.dependencies().clone();
        dependencies.schema_closure =
            LocalSchemaClosureV2::new(root_schema, document.path.clone(), vec![document]).unwrap();
        let semantic_definition = if let Some(role) = subject_role {
            let original = original_intrinsic.semantic_definition();
            let mut claims = original.claims().to_vec();
            let mut claim = serde_json::to_value(&claims[0]).unwrap();
            claim["subject_selector"] = serde_json::to_value(
                SelectorV1::new(
                    MatcherBindingV1::exact_fields_v1(),
                    vec![SelectorCriterionV1::new(id("dock.input.role"), role).unwrap()],
                )
                .unwrap(),
            )
            .unwrap();
            claims[0] = serde_json::from_value(claim).unwrap();
            ContractDefinitionV1::new(ContractDefinitionInputV1 {
                definition_schema: original.definition_schema_binding().clone(),
                contract_id: original.contract_id().clone(),
                contract_version: original.contract_version().clone(),
                contract_kind: original.contract_kind(),
                claims,
                definition_author_ref: original.definition_author_ref().clone(),
                definition_authority: original.definition_authority().clone(),
                lifecycle_policy: original.lifecycle_policy().clone(),
                matcher_bindings: vec![MatcherBindingV1::exact_fields_v1()],
                gate_policy: dependencies.gate_policy.exact_binding(),
                extensions_policy: original.extensions_policy_binding().clone(),
            })
            .unwrap()
        } else {
            original_intrinsic.semantic_definition().clone()
        };
        let intrinsic = LocalContractDefinitionV2::new(
            LocalContractDefinitionInputV2 {
                semantic_definition,
                permitted_use: original_intrinsic.permitted_use(),
                dependencies,
            },
            &stack,
        )
        .unwrap();
        let definition = LocalContractDefinitionV3::new(
            LocalContractDefinitionInputV3 {
                dependencies: LocalDefinitionDependenciesV3::new(intrinsic.dependencies().clone())
                    .unwrap(),
                intrinsic_definition: intrinsic,
            },
            &stack,
        )
        .unwrap();
        let prepared = sdk
            .prepare_local_evaluation_definition(&initial, &check, definition.clone(), &stack)
            .unwrap();
        sdk.append_local_evaluation_definition(&check, prepared, now(), "native-full-author")
            .unwrap();
        for (state, review, key) in [
            (LifecycleStateV1::ReviewReady, None, "native-full-review-ready"),
            (
                LifecycleStateV1::Locked,
                Some(LocalReviewInputV3 {
                    definition: definition.binding(),
                    closure_fingerprint: definition.closure_fingerprint().clone(),
                    permitted_use: definition.permitted_use(),
                    author_participant_ref: "packet2.author".into(),
                    reviewer_participant_ref: "packet2.reviewer".into(),
                    reviewer_actor_ref: "packet2.reviewer".into(),
                    decision: LocalReviewDecisionV2::Approve,
                    source_ref: "review.packet3.illustrative-exact-full-closure".into(),
                    outcome_text: "Illustrative distinct review of this exact full definition and native source, one attempt, finite set, final currentness and advisory truth policies.".into(),
                    semantic_time: now(),
                }),
                "native-full-review",
            ),
            (LifecycleStateV1::Active, None, "native-full-active"),
        ] {
            let head = sdk.current_local_evaluation_lifecycle(&definition.binding()).unwrap();
            let prepared = sdk.prepare_local_evaluation_lifecycle(
                &initial,
                &definition.binding(),
                LocalLifecycleTransitionInputV3 {
                    expected_lifecycle_fingerprint: head.lifecycle_fingerprint().clone(),
                    to_state: state,
                    review: review.clone(),
                    reason: "Supplied test participant requests this exact lifecycle edge.".into(),
                },
            ).unwrap();
            let (actor, participant, time) = review.map(|r| (r.reviewer_actor_ref, r.reviewer_participant_ref, r.semantic_time))
                .unwrap_or_else(|| ("packet2.author".into(), "packet2.author".into(), now()));
            sdk.transition_local_evaluation_lifecycle(&check, prepared, actor, participant, time, key).unwrap();
        }
        let mut terms = LocalEvaluationTermsV1 {
            request_key: "native-full-evaluation-terms".into(),
            rule_ref: LOCAL_EVALUATION_RULE_V1.into(),
            principal: initial.terms.terms.principal.clone(),
            agent: initial.terms.terms.agent.clone(),
            delegation: "Perform the complete exact captured advisory evaluation.".into(),
            instruction: "Form, authorize, assess, attempt once, admit, close and retain the exact local evaluation.".into(),
            approval_source_ref: "decision.packet3.illustrative-full-terms".into(),
            human_approval_asserted: true,
            approved_content_fingerprint: String::new(),
            expected_predecessor_fingerprint: original.outcome_fingerprint,
            expected_charter_fingerprint: initial.charter.canonical_fingerprint.clone(),
            ceiling: initial.terms.terms.ceiling.clone(),
            constraint_inputs: initial.terms.terms.constraint_inputs.clone(),
            allowed_paths: initial.terms.terms.allowed_paths.clone(),
            allowed_input_paths: read_paths(),
            allowed_use_intents: vec![LocalEvaluationUseIntentV1::DecisionSeeking, LocalEvaluationUseIntentV1::Diagnostic],
            required_checkpoints: local_evaluation_required_checkpoints_v1(),
            retention_until_utc: "2099-01-01T00:00:00Z".into(),
            revocation_principal: initial.terms.terms.revocation_principal.clone(),
        };
        terms.approved_content_fingerprint = terms.fingerprint_approved_content().unwrap();
        let prepared = sdk
            .prepare_local_evaluation_terms(
                &check,
                terms,
                sdk.observe_local_check_basis(&initial).unwrap(),
            )
            .unwrap();
        let authority = prepared.locator().clone();
        let interrupted = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            NativeLocalEvaluationAuthorityV1::adopt_with_interruption_for_test(
                root.path(),
                prepared,
                || panic!("illustrative terms delivery interruption"),
            )
        }));
        assert!(interrupted.is_err());
        let original_outcome_path = root.path().join(format!(
            ".handbook/local-operating/local-evaluation-v1/authority/{}/outcome.json",
            authority.terms.request_key
        ));
        let original_outcome = std::fs::read(&original_outcome_path).unwrap();
        let adopted = sdk.recover_local_evaluation_terms(&authority);
        assert!(adopted.outcome.is_some(), "{adopted:?}");
        assert_eq!(
            std::fs::read(&original_outcome_path).unwrap(),
            original_outcome
        );
        eprintln!("native fixture: evaluation terms adopted");
        let checkpoint_artifacts = NativeFixtureDirectory::new();
        std::fs::write(
            checkpoint_artifacts.path().join("native-fixture-handoff.json"),
            serde_json_canonicalizer::to_vec(&json!({
                "repository_path":root.path(),"initial":initial,"definition":definition,"authority":authority
            })).unwrap(),
        ).unwrap();
        Self {
            root,
            sdk,
            initial,
            definition,
            authority,
            set_sources: std::cell::RefCell::new(std::collections::BTreeMap::new()),
            checkpoint_artifacts,
        }
    }

    fn form(
        &self,
        key: &str,
        content: Value,
        requested: ResolutionEnvelopeV1,
        intent: LocalEvaluationUseIntentV1,
    ) -> Formed {
        let manifest = production_manifest();
        let role = &manifest.supported_inputs()[0];
        let capture = DockInlineJsonInputV1::new(
            role.role().clone(),
            role.media_type(),
            role.schema().clone(),
            content,
        )
        .unwrap();
        let subject = capture.captured_evaluation_subject_v1().unwrap();
        let requested_reach = reach_for(&self.authority, &requested);
        let formation = LocalEvaluationFormationRequestV1 {
            subject_ref: subject.subject().reference().into(),
            subject_fingerprint: subject.subject().fingerprint().as_str().into(),
            scope_fingerprint: value_fingerprint(&json!({"scope":"the exact one captured order"})),
            objective: "Does the exact captured order conform to its exact offline schema closure?"
                .into(),
            use_intent: intent,
            requested_reach,
            reach: engine_reach(),
            retention_until_utc: HORIZON.into(),
        };
        let prepared = self
            .sdk
            .prepare_local_evaluation(
                &self.initial,
                &self.authority,
                LocalEvaluationPrepareInputV3 {
                    request_key: key.into(),
                    contract: self.definition.binding(),
                    expected_lifecycle_fingerprint: self
                        .sdk
                        .current_local_evaluation_lifecycle(&self.definition.binding())
                        .unwrap()
                        .lifecycle_fingerprint()
                        .clone(),
                    subject,
                    case: capture
                        .captured_evaluation_case_v1(id("case.schema-conformance"))
                        .unwrap(),
                    requested_resolution: requested,
                    formation: formation.clone(),
                },
            )
            .unwrap();
        let snapshot = self
            .sdk
            .form_local_evaluation(&self.authority, prepared)
            .unwrap();
        assert!(snapshot.run.is_local_evaluation_v3());
        let request = snapshot.authorization_request(
            LocalEvaluationEnvelopeV1::local(
                &snapshot.purpose.subject_ref,
                snapshot.plan.selections[0].requested_reach.clone(),
                &self.authority.terms,
            ),
            engine_reach(),
        );
        let issuance = self
            .sdk
            .local_evaluation_authorization_locator(&self.authority, &request)
            .unwrap();
        let issued = if key == "packet3-native-pass-01" {
            let interrupted = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                NativeLocalEvaluationAuthorityV1::issue_with_interruption_for_test(
                    self.root.path(),
                    &self.authority,
                    &request,
                    || panic!("illustrative issuance delivery interruption"),
                )
            }));
            assert!(
                interrupted.is_err(),
                "native issuance did not reach the interrupted delivery boundary: {interrupted:?}"
            );
            let original_path = self.root.path().join(format!(
                ".handbook/local-operating/local-evaluation-v1/issuances/{}/outcome.json",
                issuance.request_key
            ));
            let original_bytes = std::fs::read(&original_path).unwrap();
            let original_directory = original_path.parent().unwrap();
            let request_path = original_directory.join("request.json");
            let pending_path = original_directory.join("pending-terminal.json");
            let terminal_path = original_directory.join("terminal.json");
            let request_bytes = std::fs::read(&request_path).unwrap();
            let pending_bytes = std::fs::read(&pending_path).unwrap();
            assert!(!terminal_path.exists());
            let pending = self.sdk.recover_local_evaluation_authorization(
                &self.authority,
                &issuance,
                &request.reach,
            );
            assert!(pending.binding.is_none());
            assert_eq!(pending.refusal.as_ref().unwrap().code, "issuance_pending");
            assert_eq!(std::fs::read(&original_path).unwrap(), original_bytes);
            assert_eq!(std::fs::read(&request_path).unwrap(), request_bytes);
            assert_eq!(std::fs::read(&pending_path).unwrap(), pending_bytes);
            assert!(!terminal_path.exists());
            let resumed = self.sdk.resume_local_evaluation_authorization(
                &self.authority,
                &issuance,
                &request.reach,
            );
            assert_eq!(std::fs::read(&original_path).unwrap(), original_bytes);
            assert_eq!(std::fs::read(&request_path).unwrap(), request_bytes);
            assert!(!pending_path.exists());
            assert_eq!(std::fs::read(&terminal_path).unwrap(), original_bytes);
            assert_eq!(
                serde_json_canonicalizer::to_vec(resumed.binding.as_ref().unwrap()).unwrap(),
                original_bytes,
                "resume must preserve the original issuance time and private lineage"
            );
            assert_eq!(
                self.sdk
                    .recover_local_evaluation_authorization(
                        &self.authority,
                        &issuance,
                        &request.reach,
                    )
                    .binding,
                resumed.binding,
                "no-effect recovery must read the exact terminal completed by explicit resume"
            );
            assert_eq!(
                self.sdk
                    .read_local_evaluation_authorization_history(
                        &self.authority,
                        &issuance,
                        &request.reach,
                    )
                    .status,
                LocalEvaluationHistoricalStatusV1::HistoricallyValid
            );
            resumed
        } else {
            self.sdk
                .issue_local_evaluation_authorization(&self.authority, &request)
        };
        assert!(issued.binding.is_some(), "{issued:?}");
        let process_request = (snapshot.run.requested_resolution()
            == manifest.resolution_ceiling())
        .then(|| process_request(&snapshot, capture));
        std::fs::write(
            self.checkpoint_artifacts.path().join(format!("formed-{key}.json")),
            serde_json_canonicalizer::to_vec(&json!({
                "formation":formation,"snapshot":snapshot,"request":request,"issuance":issuance,"process_request":process_request
            })).unwrap(),
        ).unwrap();
        Formed {
            formation,
            snapshot,
            request,
            issuance,
            process_request,
        }
    }

    fn session<'a>(&'a self, formed: &'a Formed) -> LocalEvaluationSessionV1<'a> {
        LocalEvaluationSessionV1 {
            initial: &self.initial,
            authority: &self.authority,
            original_issuance: &formed.issuance,
            authorization_request: &formed.request,
            evaluation: &formed.snapshot.locator,
        }
    }

    fn assess(&self, formed: &Formed) -> LocalEvaluationSnapshotV3 {
        let capability = self
            .sdk
            .observe_local_evaluation_capability(
                &self.initial,
                &self.authority,
                &formed.formation,
                &dock_reach(),
            )
            .unwrap();
        self.sdk
            .assess_local_evaluation(&self.session(formed), &capability, &dock_reach())
            .unwrap()
    }
}

#[derive(serde::Deserialize)]
#[serde(deny_unknown_fields)]
struct Formed {
    formation: LocalEvaluationFormationRequestV1,
    snapshot: LocalEvaluationSnapshotV3,
    request: LocalEvaluationAuthorizationRequestV1,
    issuance: LocalEvaluationBindingLocatorV1,
    process_request: Option<DockProcessRequestV1>,
}

fn now() -> SemanticTimeV1 {
    SemanticTimeV1::owner_observation_now_v1().unwrap()
}

fn reach_for(
    authority: &LocalEvaluationAuthorityLocatorV1,
    resolution: &ResolutionEnvelopeV1,
) -> LocalEvaluationReachV1 {
    let mut reach = LocalEvaluationReachV1::selected(&authority.terms.ceiling).unwrap();
    let value = serde_json::to_value(resolution).unwrap();
    for (index, (field, prefix)) in [
        ("scope_horizon", "resolution.scope."),
        ("detail_resolution", "resolution.detail."),
        ("temporal_horizon", "resolution.time."),
        ("authority_horizon", "resolution.authority."),
        ("memory_horizon", "resolution.memory."),
        ("validation_horizon", "resolution.validation."),
    ]
    .into_iter()
    .enumerate()
    {
        reach.dimensions[index] = value[field]
            .as_str()
            .unwrap()
            .strip_prefix(prefix)
            .unwrap()
            .replace('-', "_");
    }
    reach.validate().unwrap();
    reach
}

fn process_request(
    snapshot: &LocalEvaluationSnapshotV3,
    capture: DockInlineJsonInputV1,
) -> DockProcessRequestV1 {
    let manifest = production_manifest();
    let run = &snapshot.run;
    DockProcessRequestV1::new(
        &manifest,
        DockProcessRequestInputV1 {
            run: DockEvaluationRunBindingV1::new(DockEvaluationRunBindingInputV1 {
                evaluation_run: ContentAddressedBindingV1::new(
                    run.evaluation_run_ref(),
                    run.evaluation_run_fingerprint().clone(),
                )
                .unwrap(),
                evaluation_lifecycle_basis: run.evaluation_lifecycle_basis().clone(),
                contract: run.contract(),
                claim_ids: run.claims().iter().map(|c| c.claim_id().clone()).collect(),
                subject: run.subject().clone(),
                cases: run.cases().to_vec(),
                requested_resolution: run.requested_resolution().clone(),
                semantic_time_utc: run.owner_currentness().semantic_time_utc().clone(),
                lifecycle_head_fingerprint: run
                    .owner_currentness()
                    .lifecycle_head_fingerprint()
                    .clone(),
            })
            .unwrap(),
            requested_evidence_kinds: vec![id("handbook.evidence.schema-conformance")],
            inputs: vec![capture],
            effective_limits: manifest.limits().clone(),
        },
    )
    .unwrap()
}

fn finish(fixture: &Fixture, formed: &Formed) -> LocalEvaluationResultV3 {
    let session = fixture.session(formed);
    fixture
        .sdk
        .admit_local_evaluation_origin(&session, &dock_reach(), "native-origin-admission")
        .unwrap();
    fixture
        .sdk
        .seal_local_evaluation_frontier(&session)
        .unwrap();
    fixture.sdk.close_local_evaluation_set(&session).unwrap();
    let source = ContractOwnerRepositoryV1::open_existing_for_local_use_v3(fixture.root.path())
        .unwrap()
        .local_evaluation_final_set_source_v3(&formed.snapshot.locator)
        .unwrap();
    fixture
        .set_sources
        .borrow_mut()
        .insert(formed.snapshot.locator.request_key.clone(), source);
    fixture
        .sdk
        .close_local_evaluation_result(&session)
        .unwrap()
        .result
        .unwrap()
}

#[test]
fn native_full_pass_violation_and_original_recovery_never_relaunch() {
    on_stack(|| {
        let _package = InstalledProductionPackage::install();
        let resume = std::env::var_os("HANDBOOK_NATIVE_FULL_PUBLIC_HANDOFF");
        let fixture = resume.as_ref().map_or_else(Fixture::new, |path| {
            Fixture::from_public_handoff(std::path::Path::new(path))
        });
        if resume.is_some() {
            // The earlier unsupported case has a real immutable refusal.
            // Finish its exact original evidence population without relaunch.
            let original: Formed = serde_json::from_slice(
                &std::fs::read(
                    fixture
                        .checkpoint_artifacts
                        .path()
                        .join("formed-packet3-native-pass-01.json"),
                )
                .unwrap(),
            )
            .unwrap();
            let current = fixture
                .sdk
                .recover_local_evaluation(&original.snapshot.locator)
                .unwrap();
            let attempt = current.attempt.as_ref().unwrap();
            let DockLocalEvaluationAttemptOutcomeV1::Terminal { terminal, .. } = fixture
                .sdk
                .recover_local_evaluation_attempt(attempt, &dock_reach())
                .unwrap()
            else {
                panic!("the original native terminal must already be proven")
            };
            assert_eq!(terminal.kind, "refused");
            assert!(terminal.candidates.is_empty());
            let execution_count = std::fs::read_dir(
                fixture
                    .root
                    .path()
                    .join(".handbook/state/dock/v1/executions"),
            )
            .unwrap()
            .count();
            let refused = if let Some(result) = current.result {
                result
            } else {
                finish(&fixture, &original)
            };
            assert_eq!(refused.gate, LocalEvaluationGateV3::Indeterminate);
            assert!(refused.evidence_set.members.is_empty());
            assert_eq!(
                std::fs::read_dir(
                    fixture
                        .root
                        .path()
                        .join(".handbook/state/dock/v1/executions")
                )
                .unwrap()
                .count(),
                execution_count
            );
            eprintln!("native original unsupported case: exact empty Indeterminate, no relaunch");
        }
        let first = fixture.form(
            if resume.is_some() {
                "packet3-native-pass-correct-native-case-02"
            } else {
                "packet3-native-pass-01"
            },
            supplied_validation_content(),
            production_manifest().resolution_ceiling().clone(),
            LocalEvaluationUseIntentV1::DecisionSeeking,
        );
        eprintln!("native pass: purpose formed and authorization issued");
        assert_eq!(
            fixture.assess(&first).stage,
            LocalEvaluationStageV3::Attemptable
        );
        eprintln!("native pass: ready");
        let session = fixture.session(&first);
        fixture
            .sdk
            .commit_local_evaluation_collection(&session)
            .unwrap();
        eprintln!("native pass: committed");
        let committed = fixture
            .sdk
            .recover_local_evaluation(&first.snapshot.locator)
            .unwrap();
        assert!(fixture
            .sdk
            .seal_local_evaluation_frontier(&session)
            .is_err());
        assert!(fixture.sdk.close_local_evaluation_set(&session).is_err());
        assert!(fixture.sdk.close_local_evaluation_result(&session).is_err());
        assert_eq!(
            fixture
                .sdk
                .recover_local_evaluation(&first.snapshot.locator)
                .unwrap(),
            committed
        );
        let outcome = fixture
            .sdk
            .execute_local_evaluation_once(
                &session,
                first.process_request.as_ref().unwrap(),
                &dock_reach(),
                &DockCancellationV1::new(),
            )
            .unwrap();
        let (intent, terminal) = match outcome {
            DockLocalEvaluationAttemptOutcomeV1::Terminal {
                intent,
                terminal,
                replayed: false,
            } => (intent, terminal),
            other => panic!("{other:?}"),
        };
        assert_eq!(terminal.kind, "completed");
        assert_eq!(terminal.candidates.len(), 1);
        let original_bytes = serde_json_canonicalizer::to_vec(&terminal).unwrap();
        let execution_count = std::fs::read_dir(
            fixture
                .root
                .path()
                .join(".handbook/state/dock/v1/executions"),
        )
        .unwrap()
        .count();
        std::fs::remove_file(fixture.root.path().join(intent.terminal_path())).unwrap();
        let recovered = fixture
            .sdk
            .recover_local_evaluation_attempt(&intent.binding().unwrap(), &dock_reach())
            .unwrap();
        match recovered {
            DockLocalEvaluationAttemptOutcomeV1::Terminal {
                terminal,
                replayed: true,
                ..
            } => assert_eq!(
                serde_json_canonicalizer::to_vec(&terminal).unwrap(),
                original_bytes
            ),
            other => panic!("{other:?}"),
        }
        assert_eq!(
            std::fs::read_dir(
                fixture
                    .root
                    .path()
                    .join(".handbook/state/dock/v1/executions")
            )
            .unwrap()
            .count(),
            execution_count
        );
        match fixture
            .sdk
            .execute_local_evaluation_once(
                &session,
                first.process_request.as_ref().unwrap(),
                &dock_reach(),
                &DockCancellationV1::new(),
            )
            .unwrap()
        {
            DockLocalEvaluationAttemptOutcomeV1::Terminal {
                terminal,
                replayed: true,
                ..
            } => assert_eq!(
                serde_json_canonicalizer::to_vec(&terminal).unwrap(),
                original_bytes
            ),
            other => panic!("{other:?}"),
        }
        let result = finish(&fixture, &first);
        eprintln!("native pass: exact result closed");
        assert_eq!(result.gate, LocalEvaluationGateV3::Pass);
        assert_eq!(result.evidence_set.members.len(), 1);
        assert!(
            fixture
                .sdk
                .read_local_evaluation_history(&first.snapshot.locator)
                .unwrap()
                .exact_replay_available
        );
        let sealed = fixture
            .sdk
            .recover_local_evaluation(&first.snapshot.locator)
            .unwrap();
        assert!(fixture
            .sdk
            .admit_local_evaluation_origin(&session, &dock_reach(), "late-new-nonempty-operation")
            .is_err());
        assert_eq!(
            fixture
                .sdk
                .recover_local_evaluation(&first.snapshot.locator)
                .unwrap(),
            sealed
        );
        let request_path = fixture.root.path().join(format!(
            ".handbook/state/dock/v1/requests/{}.json",
            intent
                .request
                .fingerprint()
                .as_str()
                .trim_start_matches("sha256:")
        ));
        let exact_request = std::fs::read(&request_path).unwrap();
        let mut substituted_request = exact_request.clone();
        substituted_request.push(b' ');
        std::fs::write(&request_path, substituted_request).unwrap();
        assert!(fixture
            .sdk
            .recover_local_evaluation_attempt(&intent.binding().unwrap(), &dock_reach())
            .is_err());
        assert!(fixture
            .sdk
            .admit_local_evaluation_origin(&session, &dock_reach(), "native-origin-admission")
            .is_err());
        let unavailable = fixture
            .sdk
            .read_local_evaluation_history(&first.snapshot.locator)
            .unwrap();
        assert_eq!(unavailable.result, result);
        assert!(!unavailable.exact_replay_available);
        std::fs::write(&request_path, exact_request).unwrap();
        assert!(
            fixture
                .sdk
                .read_local_evaluation_history(&first.snapshot.locator)
                .unwrap()
                .exact_replay_available
        );
        let mut invalid = supplied_validation_content();
        invalid["instance"]["value"]["amount"] = json!(-1);
        invalid["instance"]["instance_fingerprint"] =
            json!(value_fingerprint(&invalid["instance"]["value"]));
        let second = fixture.form(
            "packet3-native-violation-01",
            invalid,
            production_manifest().resolution_ceiling().clone(),
            LocalEvaluationUseIntentV1::DecisionSeeking,
        );
        fixture.assess(&second);
        let session = fixture.session(&second);
        fixture
            .sdk
            .commit_local_evaluation_collection(&session)
            .unwrap();
        fixture
            .sdk
            .execute_local_evaluation_once(
                &session,
                second.process_request.as_ref().unwrap(),
                &dock_reach(),
                &DockCancellationV1::new(),
            )
            .unwrap();
        let unfavorable_unadmitted = fixture
            .sdk
            .recover_local_evaluation(&second.snapshot.locator)
            .unwrap();
        assert!(
            fixture
                .sdk
                .seal_local_evaluation_frontier(&session)
                .is_err(),
            "an actually retained unfavorable native origin cannot be omitted from the frontier"
        );
        assert!(fixture.sdk.close_local_evaluation_set(&session).is_err());
        assert_eq!(
            fixture
                .sdk
                .recover_local_evaluation(&second.snapshot.locator)
                .unwrap(),
            unfavorable_unadmitted
        );
        let result = finish(&fixture, &second);
        assert_eq!(result.gate, LocalEvaluationGateV3::Fail);
        assert!(!result.claim.violations.is_empty());

        let timed_out = fixture.form(
            "packet3-native-timeout-01",
            supplied_validation_content(),
            production_manifest().resolution_ceiling().clone(),
            LocalEvaluationUseIntentV1::DecisionSeeking,
        );
        fixture.assess(&timed_out);
        let session = fixture.session(&timed_out);
        fixture
            .sdk
            .commit_local_evaluation_collection(&session)
            .unwrap();
        let original = timed_out.process_request.as_ref().unwrap();
        let limits = original.effective_limits();
        let bounded = DockProcessRequestV1::new(
            &production_manifest(),
            DockProcessRequestInputV1 {
                run: original.run().clone(),
                requested_evidence_kinds: original.requested_evidence_kinds().to_vec(),
                inputs: original.inputs().to_vec(),
                effective_limits: DockProtocolLimitsV1::new(DockProtocolLimitInputV1 {
                    request_bytes: limits.request_bytes(),
                    result_bytes: limits.result_bytes(),
                    stderr_bytes: limits.stderr_bytes(),
                    diagnostic_bytes: limits.diagnostic_bytes(),
                    candidate_bytes: limits.candidate_bytes(),
                    input_count: limits.input_count(),
                    candidate_count: limits.candidate_count(),
                    diagnostic_count: limits.diagnostic_count(),
                    wall_timeout_ms: 1,
                    cancellation_grace_ms: limits.cancellation_grace_ms(),
                })
                .unwrap(),
            },
        )
        .unwrap();
        match fixture
            .sdk
            .execute_local_evaluation_once(
                &session,
                &bounded,
                &dock_reach(),
                &DockCancellationV1::new(),
            )
            .unwrap()
        {
            DockLocalEvaluationAttemptOutcomeV1::Terminal {
                terminal,
                replayed: false,
                ..
            } => {
                assert_eq!(terminal.kind, "timed_out");
                assert!(terminal.execution.is_some());
                assert!(terminal.candidates.is_empty());
            }
            other => panic!("{other:?}"),
        }
        let timed_out_result = finish(&fixture, &timed_out);
        assert_eq!(timed_out_result.gate, LocalEvaluationGateV3::Indeterminate);
        assert!(timed_out_result.evidence_set.members.is_empty());

        // A later occurrence with identical semantic content remains a distinct
        // owner allocation, then retains a genuine prelaunch cancellation.
        let cancelled = fixture.form(
            "packet3-native-cancelled-01",
            supplied_validation_content(),
            production_manifest().resolution_ceiling().clone(),
            LocalEvaluationUseIntentV1::DecisionSeeking,
        );
        assert_eq!(
            first.snapshot.purpose.content_fingerprint,
            cancelled.snapshot.purpose.content_fingerprint
        );
        assert_ne!(
            first.snapshot.purpose.occurrence_identity,
            cancelled.snapshot.purpose.occurrence_identity
        );
        assert_ne!(
            first.snapshot.run.evaluation_run_fingerprint(),
            cancelled.snapshot.run.evaluation_run_fingerprint()
        );
        fixture.assess(&cancelled);
        let session = fixture.session(&cancelled);
        fixture
            .sdk
            .commit_local_evaluation_collection(&session)
            .unwrap();
        let signal = DockCancellationV1::new();
        assert!(signal.cancel());
        let cancellation = fixture
            .sdk
            .execute_local_evaluation_once(
                &session,
                cancelled.process_request.as_ref().unwrap(),
                &dock_reach(),
                &signal,
            )
            .unwrap();
        let cancellation_intent = match cancellation {
            DockLocalEvaluationAttemptOutcomeV1::Terminal {
                intent,
                terminal,
                replayed: false,
            } => {
                assert_eq!(terminal.kind, "cancelled_before_launch");
                assert!(terminal.execution.is_none());
                assert!(terminal.candidates.is_empty());
                intent
            }
            other => panic!("{other:?}"),
        };
        let cancelled_result = finish(&fixture, &cancelled);
        assert_eq!(cancelled_result.gate, LocalEvaluationGateV3::Indeterminate);
        assert!(cancelled_result.evidence_set.members.is_empty());
        let closed = fixture
            .sdk
            .recover_local_evaluation(&cancelled.snapshot.locator)
            .unwrap();
        // Empty exact delivery adds no population or admission operation.
        fixture
            .sdk
            .admit_local_evaluation_origin(&session, &dock_reach(), "late-empty-replay")
            .unwrap();
        assert_eq!(
            fixture
                .sdk
                .recover_local_evaluation(&cancelled.snapshot.locator)
                .unwrap(),
            closed
        );

        // A live, sealed available producer can be insufficient for this exact
        // request. Decision-seeking is held without any native attempt.
        let mut lower = serde_json::to_value(production_manifest().resolution_ceiling()).unwrap();
        lower["detail_resolution"] = json!("resolution.detail.summary");
        let limited = fixture.form(
            "packet3-native-limited-01",
            supplied_validation_content(),
            serde_json::from_value(lower).unwrap(),
            LocalEvaluationUseIntentV1::DecisionSeeking,
        );
        let held = fixture.assess(&limited);
        assert_eq!(held.stage, LocalEvaluationStageV3::Held);
        assert_eq!(
            held.readiness_bases.last().unwrap().readiness,
            LocalEvaluationReadinessV3::KnownLimited
        );
        assert!(held.attempt.is_none());
        assert!(fixture
            .sdk
            .commit_local_evaluation_collection(&fixture.session(&limited))
            .is_err());

        // Historical immutable truth survives dependency loss; exact replay is
        // unavailable and the unresolved original no-effect attempt cannot retry.
        std::fs::remove_file(
            fixture
                .root
                .path()
                .join(cancellation_intent.terminal_path()),
        )
        .unwrap();
        assert!(matches!(
            fixture
                .sdk
                .recover_local_evaluation_attempt(
                    &cancellation_intent.binding().unwrap(),
                    &dock_reach()
                )
                .unwrap(),
            DockLocalEvaluationAttemptOutcomeV1::EffectUncertain { .. }
        ));
        let history = fixture
            .sdk
            .read_local_evaluation_history(&cancelled.snapshot.locator)
            .unwrap();
        assert_eq!(history.result, cancelled_result);
        assert!(!history.exact_replay_available);
        let handoffs = checkpoint_handoffs(&fixture, &first, "before-g");
        exercise_native_checkpoint_substitutions();
        let before_g = arm_checkpoints(&handoffs, "actual-g");
        // D -> G changes current permission without revising the earlier C or its
        // immutable result. The change uses the public owner establishment route.
        use handbook_contracts::project_admission_governance::{
            ExpectedDeferredBasisV2, ProjectAdmissionScopeBasisV3,
        };
        use handbook_sdk::admission_governance::{
            DeferredEstablishmentExecutionV3, DeferredEstablishmentRequestV3,
        };
        let ProjectAdmissionScopeBasisV3::DeferredV2(d) = first
            .snapshot
            .requirements
            .applicability
            .governance_basis
            .clone()
        else {
            panic!("initial fixture requires D")
        };
        let establishment = fixture.sdk.prepare_deferred_project_admission_governance(&DeferredEstablishmentRequestV3::human_approved(
        d.project, ExpectedDeferredBasisV2 { outcome_fingerprint: d.outcome_fingerprint, relation_fingerprint: d.relation_fingerprint },
        3600, "native-full-establish-g", "Illustrative supplied human approval of this exact same-project G freshness scope.",
    ).unwrap()).unwrap();
        assert!(matches!(
            fixture
                .sdk
                .establish_deferred_project_admission_governance(establishment),
            DeferredEstablishmentExecutionV3::Committed { .. }
        ));
        finish_refused_checkpoints(before_g, "governance_currentness");
        assert!(fixture
            .sdk
            .commit_local_evaluation_collection(&fixture.session(&limited))
            .is_err());
        let original_history = fixture
            .sdk
            .read_local_evaluation_history(&first.snapshot.locator)
            .unwrap();
        assert_eq!(original_history.result, result_for(&fixture, &first));
        assert_eq!(
            fixture
                .sdk
                .close_local_evaluation_result(&fixture.session(&first))
                .unwrap()
                .result,
            Some(original_history.result.clone()),
            "original final-result replay must remain exact after actual G"
        );
        let fresh_g = fixture.form(
            "packet3-native-fresh-g-01",
            supplied_validation_content(),
            production_manifest().resolution_ceiling().clone(),
            LocalEvaluationUseIntentV1::DecisionSeeking,
        );
        assert_ne!(fresh_g.request.current_basis, first.request.current_basis);
        assert_eq!(fresh_g.snapshot.definition, first.snapshot.definition);
        fixture.assess(&fresh_g);
        let session = fixture.session(&fresh_g);
        fixture
            .sdk
            .commit_local_evaluation_collection(&session)
            .unwrap();
        fixture
            .sdk
            .execute_local_evaluation_once(
                &session,
                fresh_g.process_request.as_ref().unwrap(),
                &dock_reach(),
                &DockCancellationV1::new(),
            )
            .unwrap();
        let fresh_g_result = finish(&fixture, &fresh_g);
        assert_eq!(fresh_g_result.gate, LocalEvaluationGateV3::Pass);
        let revoke_handoffs = checkpoint_handoffs(&fixture, &fresh_g, "before-revoke");
        let before_revoke = arm_checkpoints(&revoke_handoffs, "actual-revoke");
        let current = fixture
            .sdk
            .read_local_evaluation_terms_history(&fixture.authority)
            .outcome
            .unwrap();
        NativeLocalEvaluationAuthorityV1::revoke(
            fixture.root.path(),
            &fixture.authority,
            &LocalRevokeRequestV1 {
                request_key: "native-evaluation-revoke".into(),
                expected_current_fingerprint: current.outcome_fingerprint,
                principal: fixture.authority.terms.revocation_principal.clone(),
                source_ref: "decision.packet3.illustrative-revoke".into(),
                instruction:
                    "Revoke the exact retained evaluation permission after these supplied tests."
                        .into(),
            },
            &engine_reach(),
        )
        .unwrap();
        finish_refused_checkpoints(before_revoke, "evaluation_not_current");
        assert_eq!(
            fixture
                .sdk
                .close_local_evaluation_result(&fixture.session(&fresh_g))
                .unwrap()
                .result,
            Some(fresh_g_result.clone()),
            "original final-result replay must remain exact after native revoke"
        );
        assert_eq!(
            fixture
                .sdk
                .read_local_evaluation_history(&fresh_g.snapshot.locator)
                .unwrap()
                .result,
            fresh_g_result
        );
        assert!(fixture
            .sdk
            .execute_local_evaluation_once(
                &fixture.session(&first),
                first.process_request.as_ref().unwrap(),
                &dock_reach(),
                &DockCancellationV1::new()
            )
            .is_err());
        let history = fixture
            .sdk
            .read_local_evaluation_history(&first.snapshot.locator)
            .unwrap();
        assert_eq!(history.result.gate, LocalEvaluationGateV3::Pass);
        assert!(history.exact_replay_available);
        let exact_result = history.result;
        let lock = fixture
            .root
            .path()
            .join(".handbook/state/locks/local-operating.lock");
        let saved = fixture
            .root
            .path()
            .join(".handbook/state/locks/local-operating.packet3-original");
        let lock_bytes = std::fs::read(&lock).unwrap();
        std::fs::rename(&lock, &saved).unwrap();
        let missing = fixture.sdk.recover_local_evaluation_authorization(
            &fixture.authority,
            &first.issuance,
            &engine_reach(),
        );
        assert!(missing.binding.is_none());
        assert!(
            !lock.exists(),
            "historical recovery must not create the lost lock"
        );
        let history = fixture
            .sdk
            .read_local_evaluation_history(&first.snapshot.locator)
            .unwrap();
        assert_eq!(history.result, exact_result);
        assert!(!history.exact_replay_available);
        std::fs::write(&lock, lock_bytes).unwrap();
        assert_eq!(
            fixture
                .sdk
                .read_local_evaluation_authorization_history(
                    &fixture.authority,
                    &first.issuance,
                    &engine_reach()
                )
                .status,
            LocalEvaluationHistoricalStatusV1::Unverifiable
        );
        let original_root = fixture.root.path();
        let relocated = original_root.with_extension("packet3-original-root");
        assert_eq!(relocated.parent(), original_root.parent());
        assert!(!relocated.exists());
        std::fs::rename(original_root, &relocated).unwrap();
        std::fs::create_dir(original_root).unwrap();
        assert!(fixture
            .sdk
            .recover_local_evaluation(&first.snapshot.locator)
            .is_err());
        assert!(fixture
            .sdk
            .execute_local_evaluation_once(
                &fixture.session(&first),
                first.process_request.as_ref().unwrap(),
                &dock_reach(),
                &DockCancellationV1::new()
            )
            .is_err());
        assert_eq!(std::fs::read_dir(original_root).unwrap().count(), 0);
        std::fs::remove_dir(original_root).unwrap();
        std::fs::rename(relocated, original_root).unwrap();
    });
}

fn result_for(fixture: &Fixture, formed: &Formed) -> LocalEvaluationResultV3 {
    fixture
        .sdk
        .recover_local_evaluation(&formed.snapshot.locator)
        .unwrap()
        .result
        .unwrap()
}

fn checkpoint_worker() -> std::path::PathBuf {
    if let Some(path) = std::env::var_os("HANDBOOK_ENGINE_CHECKPOINT_WORKER") {
        return path.into();
    }
    let directory =
        std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../target/debug/deps");
    let mut binaries = std::fs::read_dir(directory)
        .unwrap()
        .map(Result::unwrap)
        .filter(|entry| {
            let name = entry.file_name().to_string_lossy().into_owned();
            name.strip_prefix("local_evaluation_checkpoint_races-")
                .and_then(|suffix| suffix.strip_suffix(std::env::consts::EXE_SUFFIX))
                .is_some_and(|hash| {
                    hash.len() == 16 && hash.bytes().all(|byte| byte.is_ascii_hexdigit())
                })
        })
        .collect::<Vec<_>>();
    binaries.sort_by_key(|entry| entry.metadata().unwrap().modified().unwrap());
    binaries
        .last()
        .expect("build the native Engine checkpoint worker before this target")
        .path()
}

struct ArmedCheckpoint {
    process: std::process::Child,
    barrier: std::path::PathBuf,
}

fn checkpoint_handoffs(fixture: &Fixture, formed: &Formed, class: &str) -> Vec<std::path::PathBuf> {
    let snapshot = fixture
        .sdk
        .recover_local_evaluation(&formed.snapshot.locator)
        .unwrap();
    let source = fixture.set_sources.borrow()[&formed.snapshot.locator.request_key].clone();
    let parent = fixture
        .checkpoint_artifacts
        .path()
        .join(format!("native-checkpoint-{class}"));
    std::fs::create_dir(&parent).unwrap();
    [
        json!({"kind":"formation","request":formed.formation}),
        json!({"kind":"verification","original":formed.issuance,"request":formed.request,"checkpoint":"launch","checkpoint_target":snapshot.basis().unwrap().fingerprint()}),
        json!({"kind":"verification","original":formed.issuance,"request":formed.request,"checkpoint":"evidence_append","checkpoint_target":snapshot.basis().unwrap().fingerprint()}),
        json!({"kind":"final_context","original":formed.issuance,"request":formed.request,"source":source}),
    ].into_iter().enumerate().map(|(index, operation)| {
        let path = parent.join(format!("handoff-{index}.json"));
        let value = json!({"repository_path":fixture.root.path(),"authority":fixture.authority,"current_basis":formed.request.current_basis,"operation":operation});
        std::fs::write(&path, serde_json_canonicalizer::to_vec(&value).unwrap()).unwrap();
        path
    }).collect()
}

fn exercise_native_checkpoint_substitutions() {
    // Renaming and restoring a native lock changes its physical generation.
    // Those probes must not invalidate the separate governance/revoke subject.
    let fixture = Fixture::new();
    let formed = fixture.form(
        "packet3-isolated-checkpoint-matrix-01",
        supplied_validation_content(),
        production_manifest().resolution_ceiling().clone(),
        LocalEvaluationUseIntentV1::DecisionSeeking,
    );
    fixture.assess(&formed);
    let session = fixture.session(&formed);
    fixture
        .sdk
        .commit_local_evaluation_collection(&session)
        .unwrap();
    fixture
        .sdk
        .execute_local_evaluation_once(
            &session,
            formed.process_request.as_ref().unwrap(),
            &dock_reach(),
            &DockCancellationV1::new(),
        )
        .unwrap();
    assert_eq!(finish(&fixture, &formed).gate, LocalEvaluationGateV3::Pass);
    let handoffs = checkpoint_handoffs(&fixture, &formed, "isolated-matrix");
    let path = fixture
        .checkpoint_artifacts
        .path()
        .join("native-checkpoint-matrix.json");
    let value = json!({"formation_fixture":handoffs[0],"launch_fixture":handoffs[1],"evidence_append_fixture":handoffs[2],"final_fixture":handoffs[3],"observations_directory":fixture.checkpoint_artifacts.path().join("native-checkpoint-matrix-observations")});
    std::fs::write(&path, serde_json_canonicalizer::to_vec(&value).unwrap()).unwrap();
    let status = std::process::Command::new(checkpoint_worker())
        .args([
            "--ignored",
            "--exact",
            "native_checkpoint_substitution_matrix",
            "--nocapture",
        ])
        .env("HANDBOOK_ENGINE_CHECKPOINT_MATRIX", &path)
        .status()
        .unwrap();
    assert!(
        status.success(),
        "actual currentness substitution matrix failed"
    );
}

fn arm_checkpoints(handoffs: &[std::path::PathBuf], class: &str) -> Vec<ArmedCheckpoint> {
    let worker = checkpoint_worker();
    handoffs
        .iter()
        .enumerate()
        .map(|(index, path)| {
            let barrier = path
                .parent()
                .unwrap()
                .join(format!("{class}-barrier-{index}"));
            std::fs::create_dir(&barrier).unwrap();
            let mut process = std::process::Command::new(&worker)
                .args([
                    "--ignored",
                    "--exact",
                    "native_checkpoint_process_worker",
                    "--nocapture",
                ])
                .env("HANDBOOK_ENGINE_CHECKPOINT_FIXTURE", path)
                .env("HANDBOOK_ENGINE_CHECKPOINT_BARRIER", &barrier)
                .spawn()
                .unwrap();
            let deadline = std::time::Instant::now() + std::time::Duration::from_secs(60);
            while !barrier.join("ready.json").exists() {
                assert!(
                    process.try_wait().unwrap().is_none(),
                    "worker exited before barrier"
                );
                assert!(
                    std::time::Instant::now() < deadline,
                    "worker did not reach barrier"
                );
                std::thread::sleep(std::time::Duration::from_millis(10));
            }
            ArmedCheckpoint { process, barrier }
        })
        .collect()
}

fn finish_refused_checkpoints(mut checkpoints: Vec<ArmedCheckpoint>, expected_code: &str) {
    for worker in &checkpoints {
        std::fs::write(
            worker.barrier.join("release"),
            b"released after the actual native owner mutation",
        )
        .unwrap();
    }
    for worker in &mut checkpoints {
        assert!(worker.process.wait().unwrap().success());
        let outcome: Value =
            serde_json::from_slice(&std::fs::read(worker.barrier.join("outcome.json")).unwrap())
                .unwrap();
        assert_eq!(outcome["admitted"], false, "{outcome}");
        assert!(!outcome["refusal"].is_null(), "{outcome}");
        assert_eq!(outcome["refusal"]["code"], expected_code, "{outcome}");
        assert!(
            outcome["verification"].is_null(),
            "refused checkpoint cannot yield a positive token: {outcome}"
        );
    }
}

#[cfg(windows)]
#[test]
fn native_unknown_availability_is_observed_from_a_private_host_read_failure() {
    const CHILD: &str = "HANDBOOK_PACKET3_UNKNOWN_PRIVATE_HOST";
    if std::env::var_os(CHILD).is_none() {
        let host = tempfile::tempdir().unwrap();
        let executable = host.path().join("private-native-evaluation-test.exe");
        std::fs::copy(std::env::current_exe().unwrap(), &executable).unwrap();
        let status = std::process::Command::new(&executable)
            .args([
                "--exact",
                "native_unknown_availability_is_observed_from_a_private_host_read_failure",
                "--nocapture",
                "--test-threads=1",
            ])
            .env(CHILD, host.path())
            .status()
            .unwrap();
        assert!(status.success());
        return;
    }
    on_stack(|| {
        use std::os::windows::fs::OpenOptionsExt;
        let _package = InstalledProductionPackage::install();
        let fixture = Fixture::new();
        let formed = fixture.form(
            "packet3-native-unknown-01",
            supplied_validation_content(),
            production_manifest().resolution_ceiling().clone(),
            LocalEvaluationUseIntentV1::DecisionSeeking,
        );
        let executable = std::env::current_exe().unwrap();
        let manifest_path = executable
            .parent()
            .unwrap()
            .join("handbook-docks/handbook.dock.json-schema/1.0.0")
            .join(host_target_tuple_v1())
            .join("manifest.json");
        // The private host's actual manifest stays present and unmodified. A
        // real OS exclusive share denies the native resolver access to bytes.
        let exclusive = std::fs::OpenOptions::new()
            .read(true)
            .share_mode(0)
            .open(&manifest_path)
            .unwrap();
        let record = fixture
            .sdk
            .observe_local_evaluation_capability(
                &fixture.initial,
                &fixture.authority,
                &formed.formation,
                &dock_reach(),
            )
            .unwrap();
        assert_eq!(record.availability, "unknown");
        assert_eq!(
            record.controls["availability_reason"],
            "production_package_availability_unobservable"
        );
        let held = fixture
            .sdk
            .assess_local_evaluation(&fixture.session(&formed), &record, &dock_reach())
            .unwrap();
        assert_eq!(held.stage, LocalEvaluationStageV3::Held);
        assert_eq!(
            held.readiness_bases.last().unwrap().readiness,
            LocalEvaluationReadinessV3::Unknown
        );
        assert!(held.attempt.is_none());
        assert!(fixture
            .sdk
            .commit_local_evaluation_collection(&fixture.session(&formed))
            .is_err());
        let diagnostic = fixture.form(
            "packet3-native-diagnostic-unknown-01",
            supplied_validation_content(),
            production_manifest().resolution_ceiling().clone(),
            LocalEvaluationUseIntentV1::Diagnostic,
        );
        let session = fixture.session(&diagnostic);
        let assessed = fixture
            .sdk
            .assess_local_evaluation(&session, &record, &dock_reach())
            .unwrap();
        assert_eq!(assessed.stage, LocalEvaluationStageV3::Attemptable);
        assert_eq!(
            assessed.readiness_bases.last().unwrap().readiness,
            LocalEvaluationReadinessV3::Unknown
        );
        fixture
            .sdk
            .commit_local_evaluation_collection(&session)
            .unwrap();
        match fixture
            .sdk
            .execute_local_evaluation_once(
                &session,
                diagnostic.process_request.as_ref().unwrap(),
                &dock_reach(),
                &DockCancellationV1::new(),
            )
            .unwrap()
        {
            DockLocalEvaluationAttemptOutcomeV1::Terminal {
                terminal,
                replayed: false,
                ..
            } => {
                assert_eq!(terminal.kind, "refused_before_launch");
                assert!(terminal.execution.is_none());
                assert!(terminal.candidates.is_empty());
            }
            other => panic!("{other:?}"),
        }
        let diagnostic_result = finish(&fixture, &diagnostic);
        assert_eq!(diagnostic_result.gate, LocalEvaluationGateV3::Indeterminate);
        assert!(diagnostic_result.evidence_set.members.is_empty());
        drop(exclusive);
        let absent_manifest = manifest_path.with_extension("packet3-retained-manifest");
        assert_eq!(absent_manifest.parent(), manifest_path.parent());
        assert!(!absent_manifest.exists());
        std::fs::rename(&manifest_path, &absent_manifest).unwrap();
        let limited_record = fixture
            .sdk
            .observe_local_evaluation_capability(
                &fixture.initial,
                &fixture.authority,
                &formed.formation,
                &dock_reach(),
            )
            .unwrap();
        assert_eq!(limited_record.availability, "known_limited");
        assert_eq!(
            limited_record.controls["availability_reason"],
            "production_package_absent"
        );
        let held = fixture
            .sdk
            .assess_local_evaluation(&fixture.session(&formed), &limited_record, &dock_reach())
            .unwrap();
        assert_eq!(held.stage, LocalEvaluationStageV3::Held);
        assert_eq!(
            held.readiness_bases.last().unwrap().readiness,
            LocalEvaluationReadinessV3::KnownLimited
        );
        assert!(held.attempt.is_none());
        let limited_diagnostic = fixture.form(
            "packet3-native-diagnostic-limited-01",
            supplied_validation_content(),
            production_manifest().resolution_ceiling().clone(),
            LocalEvaluationUseIntentV1::Diagnostic,
        );
        let session = fixture.session(&limited_diagnostic);
        let assessed = fixture
            .sdk
            .assess_local_evaluation(&session, &limited_record, &dock_reach())
            .unwrap();
        assert_eq!(assessed.stage, LocalEvaluationStageV3::Attemptable);
        fixture
            .sdk
            .commit_local_evaluation_collection(&session)
            .unwrap();
        match fixture
            .sdk
            .execute_local_evaluation_once(
                &session,
                limited_diagnostic.process_request.as_ref().unwrap(),
                &dock_reach(),
                &DockCancellationV1::new(),
            )
            .unwrap()
        {
            DockLocalEvaluationAttemptOutcomeV1::Terminal {
                terminal,
                replayed: false,
                ..
            } => {
                assert_eq!(terminal.kind, "refused_before_launch");
                assert!(terminal.execution.is_none());
                assert!(terminal.candidates.is_empty());
            }
            other => panic!("{other:?}"),
        }
        let result = finish(&fixture, &limited_diagnostic);
        assert_eq!(result.gate, LocalEvaluationGateV3::Indeterminate);
        assert!(result.evidence_set.members.is_empty());
        std::fs::rename(absent_manifest, &manifest_path).unwrap();
        let available = fixture
            .sdk
            .observe_local_evaluation_capability(
                &fixture.initial,
                &fixture.authority,
                &formed.formation,
                &dock_reach(),
            )
            .unwrap();
        assert_eq!(available.availability, "available");
        assert_ne!(available.binding().unwrap(), record.binding().unwrap());
        assert_eq!(
            fixture
                .sdk
                .assess_local_evaluation(&fixture.session(&formed), &available, &dock_reach())
                .unwrap()
                .stage,
            LocalEvaluationStageV3::Attemptable
        );
        let session = fixture.session(&formed);
        fixture
            .sdk
            .commit_local_evaluation_collection(&session)
            .unwrap();
        let original_manifest = std::fs::read(&manifest_path).unwrap();
        std::fs::write(&manifest_path, b"wrong native package manifest").unwrap();
        let refused = fixture
            .sdk
            .execute_local_evaluation_once(
                &session,
                formed.process_request.as_ref().unwrap(),
                &dock_reach(),
                &DockCancellationV1::new(),
            )
            .unwrap();
        std::fs::write(&manifest_path, original_manifest).unwrap();
        match refused {
            DockLocalEvaluationAttemptOutcomeV1::Terminal {
                terminal,
                replayed: false,
                ..
            } => {
                assert_eq!(terminal.kind, "refused_before_launch");
                assert!(terminal.execution.is_none());
                assert!(terminal.candidates.is_empty());
            }
            other => panic!("{other:?}"),
        }
        let result = finish(&fixture, &formed);
        assert_eq!(result.gate, LocalEvaluationGateV3::Indeterminate);
        assert!(result.evidence_set.members.is_empty());
    });
}

fn content_with_schema(schema: Value) -> Value {
    let mut content = supplied_validation_content();
    let fingerprint = value_fingerprint(&schema);
    content["documents"][0]["schema"] = schema;
    content["documents"][0]["document_fingerprint"] = json!(fingerprint);
    content["root_schema"]["fingerprint"] = json!(fingerprint);
    content["root_document_fingerprint"] = json!(fingerprint);
    content["closure_fingerprint"] = json!(value_fingerprint(&json!([{
        "path":content["root_document"], "document_fingerprint":fingerprint
    }])));
    content
}

#[test]
fn native_not_applicable_is_derived_from_the_exact_authored_selector() {
    on_stack(|| {
        let _package = InstalledProductionPackage::install();
        let content = supplied_validation_content();
        let fixture =
            Fixture::new_with_applicability(content.clone(), Some("handbook.input.unrelated"));
        let formed = fixture.form(
            "packet3-native-not-applicable-01",
            content,
            production_manifest().resolution_ceiling().clone(),
            LocalEvaluationUseIntentV1::DecisionSeeking,
        );
        assert_eq!(
            fixture.assess(&formed).stage,
            LocalEvaluationStageV3::Attemptable
        );
        let session = fixture.session(&formed);
        fixture
            .sdk
            .commit_local_evaluation_collection(&session)
            .unwrap();
        let cancellation = DockCancellationV1::new();
        assert!(cancellation.cancel());
        assert!(matches!(
            fixture
                .sdk
                .execute_local_evaluation_once(
                    &session,
                    formed.process_request.as_ref().unwrap(),
                    &dock_reach(),
                    &cancellation,
                )
                .unwrap(),
            DockLocalEvaluationAttemptOutcomeV1::Terminal { .. }
        ));
        let closed = finish(&fixture, &formed);
        assert_eq!(closed.gate, LocalEvaluationGateV3::NotApplicable);
        assert_eq!(
            closed.claim.applicability,
            ApplicabilityOutcomeV1::NotApplicable
        );
        assert_eq!(closed.claim.finding, "not_assessed");
        assert!(closed.evidence_set.members.is_empty());
    });
}

#[test]
fn native_process_refusals_retain_unsupported_dialect_and_remote_reason() {
    // Retain this selected test symbol while GitNexus cannot resolve a safe
    // rename. Both inputs are refused at their responsible authored-definition
    // boundary, before an evaluation occurrence or producer attempt exists.
    on_stack(|| {
        let _package = InstalledProductionPackage::install();
        {
            let fixture = Fixture::new();
            let content = content_with_schema(json!({
                "$schema":"http://json-schema.org/draft-07/schema#", "type":"object"
            }));
            let document = LocalSchemaDocumentV2::new(
                content["root_document"].as_str().unwrap().into(),
                content["documents"][0]["schema"].clone(),
            )
            .unwrap();
            let mut dependencies = fixture.definition.dependencies().clone();
            dependencies.schema_closure = LocalSchemaClosureV2::new(
                ExactBindingV1::new(
                    ExactRefV1::parse(content["root_schema"]["ref"].as_str().unwrap()).unwrap(),
                    document.document_fingerprint.clone(),
                ),
                document.path.clone(),
                vec![document],
            )
            .unwrap();
            let stack = handbook_engine::local_authority::resolved_local_operating_profile()
                .unwrap()
                .context_resolution()
                .clone();
            let intrinsic = LocalContractDefinitionV2::new(
                LocalContractDefinitionInputV2 {
                    semantic_definition: fixture.definition.semantic_definition().clone(),
                    permitted_use: fixture.definition.permitted_use(),
                    dependencies,
                },
                &stack,
            )
            .unwrap();
            // The full native owner admits only the declared Draft 2020-12
            // profile. An unsupported definition cannot reach occurrence,
            // authorization, capability, or producer effect.
            assert_eq!(
                LocalContractDefinitionV3::new(
                    LocalContractDefinitionInputV3 {
                        dependencies: LocalDefinitionDependenciesV3::new(
                            intrinsic.dependencies().clone(),
                        )
                        .unwrap(),
                        intrinsic_definition: intrinsic,
                    },
                    &stack,
                )
                .unwrap_err(),
                ContractError::InvalidInvariant
            );
            assert!(!fixture
                .root
                .path()
                .join(LOCAL_EVALUATION_OWNER_ROOT_V3)
                .exists());
            assert!(!fixture
                .root
                .path()
                .join(DOCK_LOCAL_EVALUATION_ROOT_V1)
                .exists());
            eprintln!(
                "native unsupported dialect: full definition refused before occurrence/effect"
            );
            let content = content_with_schema(json!({
                "$schema":"https://json-schema.org/draft/2020-12/schema",
                "$ref":"https://example.invalid/schema.json"
            }));
            let document = LocalSchemaDocumentV2::new(
                content["root_document"].as_str().unwrap().into(),
                content["documents"][0]["schema"].clone(),
            )
            .unwrap();
            assert_eq!(
                LocalSchemaClosureV2::new(
                    ExactBindingV1::new(
                        ExactRefV1::parse(content["root_schema"]["ref"].as_str().unwrap()).unwrap(),
                        document.document_fingerprint.clone(),
                    ),
                    document.path.clone(),
                    vec![document],
                )
                .unwrap_err(),
                ContractError::InvalidInvariant
            );
            assert!(!fixture
                .root
                .path()
                .join(LOCAL_EVALUATION_OWNER_ROOT_V3)
                .exists());
            assert!(!fixture
                .root
                .path()
                .join(DOCK_LOCAL_EVALUATION_ROOT_V1)
                .exists());
            eprintln!(
                "native remote retrieval: exact offline closure refused before occurrence/effect"
            );
        }
    });
}

#[test]
fn direct_native_owner_refuses_a_forged_broader_run_before_attempt_reservation() {
    on_stack(|| {
        use handbook_engine::local_authority::local_evaluation::LocalEvaluationCheckpointV1;
        let _package = InstalledProductionPackage::install();
        let fixture = Fixture::new();
        let mut lower = serde_json::to_value(production_manifest().resolution_ceiling()).unwrap();
        lower["detail_resolution"] = json!("resolution.detail.summary");
        let formed = fixture.form(
            "packet3-native-direct-broader-run-01",
            supplied_validation_content(),
            serde_json::from_value(lower).unwrap(),
            LocalEvaluationUseIntentV1::Diagnostic,
        );
        assert_eq!(
            fixture.assess(&formed).stage,
            LocalEvaluationStageV3::Attemptable
        );
        fixture
            .sdk
            .commit_local_evaluation_collection(&fixture.session(&formed))
            .unwrap();
        let committed = fixture
            .sdk
            .recover_local_evaluation(&formed.snapshot.locator)
            .unwrap();
        for (detail, expected_code) in [
            ("not_in_the_shipped_detail_table", "resolution_stack"),
            ("full", "resolution_expansion"),
        ] {
            let mut unsupported = formed.formation.clone();
            unsupported.requested_reach.dimensions[1] = detail.into();
            let consumed = std::cell::Cell::new(false);
            let refusal = NativeLocalEvaluationAuthorityV1::with_terms_admission(
                fixture.root.path(),
                &fixture.authority,
                &formed.request.current_basis,
                &unsupported,
                |_| {
                    consumed.set(true);
                    Ok(())
                },
            )
            .unwrap_err();
            assert_eq!(refusal.code, expected_code, "{refusal:?}");
            assert!(
                !consumed.get(),
                "unsupported Q cannot receive a native formation token"
            );
            assert_eq!(
                fixture
                    .sdk
                    .recover_local_evaluation(&formed.snapshot.locator)
                    .unwrap(),
                committed
            );
        }
        let mut forged = serde_json::to_value(&committed.run).unwrap();
        forged["requested_resolution"] =
            serde_json::to_value(production_manifest().resolution_ceiling()).unwrap();
        let object = forged.as_object_mut().unwrap();
        object.remove("evaluation_run_ref");
        object.remove("evaluation_run_fingerprint");
        let fingerprint = value_fingerprint(&forged);
        forged["evaluation_run_ref"] = json!(format!(
            "{}#evaluation/{}",
            forged["contract_ref"].as_str().unwrap(),
            fingerprint.trim_start_matches("sha256:")
        ));
        forged["evaluation_run_fingerprint"] = json!(fingerprint);
        let mut forged_snapshot = committed.clone();
        forged_snapshot.run = serde_json::from_value(forged).unwrap();
        let capture = DockInlineJsonInputV1::new(
            production_manifest().supported_inputs()[0].role().clone(),
            production_manifest().supported_inputs()[0].media_type(),
            production_manifest().supported_inputs()[0].schema().clone(),
            supplied_validation_content(),
        )
        .unwrap();
        let request = process_request(&forged_snapshot, capture);
        let run = forged_snapshot.run.canonical_json_bytes().unwrap();
        let coordinates = DockLocalEvaluationCoordinatesV1 {
            occurrence: committed.occurrence().clone(),
            basis: committed.basis().unwrap().clone(),
            selection: committed.selection().clone(),
            authorization: committed.authorization_binding().unwrap(),
            capability_observation: committed
                .readiness_bases
                .last()
                .unwrap()
                .capability
                .binding()
                .unwrap(),
        };
        let dock = DockLocalEvaluationOwnerV1::open_existing(fixture.root.path()).unwrap();
        let attempts = fixture
            .root
            .path()
            .join(format!("{DOCK_LOCAL_EVALUATION_ROOT_V1}/attempts"));
        let before = std::fs::read_dir(&attempts).unwrap().count();
        let outcome = NativeLocalEvaluationAuthorityV1::with_verification(
            fixture.root.path(),
            &fixture.authority,
            &formed.issuance,
            &formed.request,
            LocalEvaluationCheckpointV1::Launch,
            Some(coordinates.basis.fingerprint().as_str()),
            &formed.request.current_basis,
            &engine_reach(),
            |admission| {
                Ok(dock.execute_once(
                    admission,
                    &coordinates,
                    &request,
                    &run,
                    &dock_reach(),
                    SemanticTimeV1::parse(HORIZON).unwrap(),
                    &DockCancellationV1::new(),
                ))
            },
        )
        .unwrap();
        assert_eq!(
            outcome.unwrap_err().kind(),
            DockLocalEvaluationErrorKindV1::AuthorityRefused
        );
        assert_eq!(std::fs::read_dir(attempts).unwrap().count(), before);
        assert!(fixture
            .sdk
            .recover_local_evaluation(&formed.snapshot.locator)
            .unwrap()
            .attempt
            .is_none());
    });
}

/// Continues only the unexecuted acceptance suffix on the original physical
/// fixture. Earlier target failures and immutable stale results remain evidence;
/// this target neither reforms their requests nor repeats their producer effects.
#[test]
#[ignore = "requires the original retained Packet 3 public handoff and current checkpoint worker"]
fn native_retained_packet3_acceptance_continuation() {
    on_stack(|| {
        use sha2::{Digest, Sha256};
        let handoff = std::env::var_os("HANDBOOK_NATIVE_RETAINED_ACCEPTANCE_HANDOFF")
            .expect("supply the original .tmpVoEuRH/native-fixture-handoff.json");
        let mut fixture = Fixture::from_public_handoff(std::path::Path::new(&handoff));
        let load = |key: &str| -> Formed {
            let bytes = std::fs::read(
                fixture
                    .checkpoint_artifacts
                    .path()
                    .join(format!("formed-{key}.json")),
            )
            .unwrap();
            let value: Value = serde_json::from_slice(&bytes).unwrap();
            assert_eq!(serde_json_canonicalizer::to_vec(&value).unwrap(), bytes);
            serde_json::from_value(value).unwrap()
        };
        let first = load("packet3-native-pass-correct-native-case-02");
        let second = load("packet3-native-violation-01");

        // The handoff can predate the corrected native case. New occurrences
        // use the genuinely retained, reviewed definition of the successful run.
        fixture.definition = first.snapshot.definition.clone();
        assert_eq!(
            fixture
                .sdk
                .read_local_evaluation_definition(&fixture.definition.binding())
                .unwrap()
                .definition(),
            &fixture.definition,
        );
        let active = fixture
            .sdk
            .current_local_evaluation_lifecycle(&fixture.definition.binding())
            .unwrap();
        assert_eq!(active.state(), LifecycleStateV1::Active);
        assert!(active.approving_review().is_some());
        assert_eq!(
            active.lifecycle_fingerprint(),
            &first.snapshot.original_input.expected_lifecycle_fingerprint
        );
        assert_eq!(
            serde_json::to_value(&first.snapshot.original_input.case).unwrap()["case_id"],
            "case.schema-conformance"
        );
        assert_eq!(second.snapshot.definition, first.snapshot.definition);

        let original_positive = fixture
            .sdk
            .read_local_evaluation_history(&first.snapshot.locator)
            .unwrap();
        assert_eq!(original_positive.result.gate, LocalEvaluationGateV3::Pass);
        assert_eq!(original_positive.result.evidence_set.members.len(), 1);
        assert!(original_positive.exact_replay_available);
        let original_negative = fixture
            .sdk
            .read_local_evaluation_history(&second.snapshot.locator)
            .unwrap();
        assert_eq!(
            original_negative.result.gate,
            LocalEvaluationGateV3::Indeterminate
        );
        assert_eq!(original_negative.result.evidence_set.members.len(), 1);
        assert_eq!(
            original_negative.result.evaluated_at.as_str(),
            "2026-10-07T01:28:06Z"
        );
        assert_eq!(
            original_negative.result.purpose.formed_at_utc,
            "2026-10-07T00:22:45Z"
        );
        let claim = serde_json::to_value(&original_negative.result.claim).unwrap();
        assert_eq!(claim["currentness"], "stale");
        assert_eq!(claim["finding"], "unsatisfied");
        assert_eq!(claim["violations"][0]["instance_pointer"], "/amount");
        assert!(original_negative.exact_replay_available);

        // The original process's in-memory set source was lost. Rehydrate the
        // public DTO from the exact immutable SetClosed record, never the later
        // head or a rewritten/copy-seeded owner record. Engine rereads and pins it.
        let relative_path = ".handbook/state/contracts/local-evaluation-v3/records/aca47a83e4b228424d638b2e1a8413758f10e0fa5744013c831e7e6054cb6205.json";
        let bytes = std::fs::read(fixture.root.path().join(relative_path)).unwrap();
        let value: Value = serde_json::from_slice(&bytes).unwrap();
        assert_eq!(serde_json_canonicalizer::to_vec(&value).unwrap(), bytes);
        assert_eq!(
            format!("sha256:{:x}", Sha256::digest(&bytes)),
            "sha256:eaa8f473d51c532114561417a360ab10a7dae308fb1ccf5ebe2ae120b9efdbb7"
        );
        let fixed: LocalEvaluationSnapshotV3 = serde_json::from_value(value).unwrap();
        assert_eq!(fixed.stage, LocalEvaluationStageV3::SetClosed);
        assert_eq!(fixed.sequence, 6);
        assert_eq!(fixed.locator, first.snapshot.locator);
        assert_eq!(fixed.purpose, first.snapshot.purpose);
        assert_eq!(fixed.plan, first.snapshot.plan);
        assert_eq!(fixed.run, first.snapshot.run);
        assert_eq!(
            fixed.evidence_set.as_ref(),
            Some(&original_positive.result.evidence_set)
        );
        fixture.set_sources.borrow_mut().insert(
            first.snapshot.locator.request_key.clone(),
            LocalEvaluationFinalSetSourceV1 {
                relative_path: relative_path.into(),
                record_fingerprint: format!("sha256:{:x}", Sha256::digest(&bytes)),
                set_fingerprint: fixed
                    .evidence_set
                    .as_ref()
                    .unwrap()
                    .identity
                    .fingerprint()
                    .as_str()
                    .into(),
                canonical_record_bytes: bytes,
            },
        );
        eprintln!("native retained continuation: original Pass and stale violation verified, exact SetClosed source hydrated");

        // Stage/reuse only the independently verified frozen producer beside the
        // current test host; this does not initialize or copy the retained root.
        let _package = InstalledProductionPackage::install();
        let timed_out = fixture.form(
            "packet3-native-timeout-01",
            supplied_validation_content(),
            production_manifest().resolution_ceiling().clone(),
            LocalEvaluationUseIntentV1::DecisionSeeking,
        );
        fixture.assess(&timed_out);
        let session = fixture.session(&timed_out);
        fixture
            .sdk
            .commit_local_evaluation_collection(&session)
            .unwrap();
        let original = timed_out.process_request.as_ref().unwrap();
        let limits = original.effective_limits();
        let bounded = DockProcessRequestV1::new(
            &production_manifest(),
            DockProcessRequestInputV1 {
                run: original.run().clone(),
                requested_evidence_kinds: original.requested_evidence_kinds().to_vec(),
                inputs: original.inputs().to_vec(),
                effective_limits: DockProtocolLimitsV1::new(DockProtocolLimitInputV1 {
                    request_bytes: limits.request_bytes(),
                    result_bytes: limits.result_bytes(),
                    stderr_bytes: limits.stderr_bytes(),
                    diagnostic_bytes: limits.diagnostic_bytes(),
                    candidate_bytes: limits.candidate_bytes(),
                    input_count: limits.input_count(),
                    candidate_count: limits.candidate_count(),
                    diagnostic_count: limits.diagnostic_count(),
                    wall_timeout_ms: 1,
                    cancellation_grace_ms: limits.cancellation_grace_ms(),
                })
                .unwrap(),
            },
        )
        .unwrap();
        match fixture
            .sdk
            .execute_local_evaluation_once(
                &session,
                &bounded,
                &dock_reach(),
                &DockCancellationV1::new(),
            )
            .unwrap()
        {
            DockLocalEvaluationAttemptOutcomeV1::Terminal {
                terminal,
                replayed: false,
                ..
            } => {
                assert_eq!(terminal.kind, "timed_out");
                assert!(terminal.execution.is_some());
                assert!(terminal.candidates.is_empty());
            }
            other => panic!("{other:?}"),
        }
        let timed_out_result = finish(&fixture, &timed_out);
        assert_eq!(timed_out_result.gate, LocalEvaluationGateV3::Indeterminate);
        assert!(timed_out_result.evidence_set.members.is_empty());
        eprintln!(
            "native retained continuation: genuine timeout and empty current-source result closed"
        );

        // Summary (rank 1) exceeds this producer's identifier-only ceiling
        // (rank 0): available capability is insufficient and DecisionSeeking holds.
        let mut requested =
            serde_json::to_value(production_manifest().resolution_ceiling()).unwrap();
        requested["detail_resolution"] = json!("resolution.detail.summary");
        let limited = fixture.form(
            "packet3-native-limited-01",
            supplied_validation_content(),
            serde_json::from_value(requested).unwrap(),
            LocalEvaluationUseIntentV1::DecisionSeeking,
        );
        let held = fixture.assess(&limited);
        assert_eq!(held.stage, LocalEvaluationStageV3::Held);
        assert_eq!(
            held.readiness_bases.last().unwrap().readiness,
            LocalEvaluationReadinessV3::KnownLimited
        );
        assert_eq!(
            held.readiness_bases.last().unwrap().capability.availability,
            "available"
        );
        assert!(held.attempt.is_none());
        assert!(fixture
            .sdk
            .commit_local_evaluation_collection(&fixture.session(&limited))
            .is_err());
        eprintln!("native retained continuation: genuine available-but-insufficient DecisionSeeking held without attempt");

        let handoffs = checkpoint_handoffs(&fixture, &first, "retained-before-g");
        exercise_native_checkpoint_substitutions();
        eprintln!(
            "native retained continuation: 13 actual checkpoint substitution outcomes verified"
        );
        let before_g = arm_checkpoints(&handoffs, "retained-actual-g");
        use handbook_contracts::project_admission_governance::{
            ExpectedDeferredBasisV2, ProjectAdmissionScopeBasisV3,
        };
        use handbook_sdk::admission_governance::{
            DeferredEstablishmentExecutionV3, DeferredEstablishmentRequestV3,
        };
        let ProjectAdmissionScopeBasisV3::DeferredV2(d) = first
            .snapshot
            .requirements
            .applicability
            .governance_basis
            .clone()
        else {
            panic!("the original retained fixture requires actual D")
        };
        let establishment = fixture.sdk.prepare_deferred_project_admission_governance(&DeferredEstablishmentRequestV3::human_approved(
            d.project, ExpectedDeferredBasisV2 { outcome_fingerprint: d.outcome_fingerprint, relation_fingerprint: d.relation_fingerprint },
            3600, "native-full-establish-g", "Illustrative supplied human approval of this exact same-project G freshness scope.",
        ).unwrap()).unwrap();
        assert!(matches!(
            fixture
                .sdk
                .establish_deferred_project_admission_governance(establishment),
            DeferredEstablishmentExecutionV3::Committed { .. }
        ));
        finish_refused_checkpoints(before_g, "governance_currentness");
        assert!(fixture
            .sdk
            .commit_local_evaluation_collection(&fixture.session(&limited))
            .is_err());
        assert_eq!(
            fixture
                .sdk
                .read_local_evaluation_history(&first.snapshot.locator)
                .unwrap()
                .result,
            original_positive.result
        );
        assert_eq!(
            fixture
                .sdk
                .read_local_evaluation_history(&second.snapshot.locator)
                .unwrap()
                .result,
            original_negative.result
        );
        assert_eq!(
            fixture
                .sdk
                .close_local_evaluation_result(&fixture.session(&first))
                .unwrap()
                .result,
            Some(original_positive.result.clone())
        );
        eprintln!("native retained continuation: four actual G refusals and original immutable histories verified");

        let fresh_g = fixture.form(
            "packet3-native-fresh-g-01",
            supplied_validation_content(),
            production_manifest().resolution_ceiling().clone(),
            LocalEvaluationUseIntentV1::DecisionSeeking,
        );
        assert_ne!(fresh_g.request.current_basis, first.request.current_basis);
        assert_eq!(fresh_g.snapshot.definition, first.snapshot.definition);
        fixture.assess(&fresh_g);
        let session = fixture.session(&fresh_g);
        fixture
            .sdk
            .commit_local_evaluation_collection(&session)
            .unwrap();
        fixture
            .sdk
            .execute_local_evaluation_once(
                &session,
                fresh_g.process_request.as_ref().unwrap(),
                &dock_reach(),
                &DockCancellationV1::new(),
            )
            .unwrap();
        let fresh_g_result = finish(&fixture, &fresh_g);
        assert_eq!(fresh_g_result.gate, LocalEvaluationGateV3::Pass);
        eprintln!(
            "native retained continuation: genuinely new same-project G occurrence closed Pass"
        );
        let revoke_handoffs = checkpoint_handoffs(&fixture, &fresh_g, "retained-before-revoke");
        let before_revoke = arm_checkpoints(&revoke_handoffs, "retained-actual-revoke");
        let current = fixture
            .sdk
            .read_local_evaluation_terms_history(&fixture.authority)
            .outcome
            .unwrap();
        NativeLocalEvaluationAuthorityV1::revoke(
            fixture.root.path(),
            &fixture.authority,
            &LocalRevokeRequestV1 {
                request_key: "native-evaluation-revoke".into(),
                expected_current_fingerprint: current.outcome_fingerprint,
                principal: fixture.authority.terms.revocation_principal.clone(),
                source_ref: "decision.packet3.illustrative-revoke".into(),
                instruction:
                    "Revoke the exact retained evaluation permission after these supplied tests."
                        .into(),
            },
            &engine_reach(),
        )
        .unwrap();
        finish_refused_checkpoints(before_revoke, "evaluation_not_current");
        assert_eq!(
            fixture
                .sdk
                .close_local_evaluation_result(&fixture.session(&fresh_g))
                .unwrap()
                .result,
            Some(fresh_g_result.clone())
        );
        assert_eq!(
            fixture
                .sdk
                .read_local_evaluation_history(&fresh_g.snapshot.locator)
                .unwrap()
                .result,
            fresh_g_result
        );
        assert_eq!(
            fixture
                .sdk
                .read_local_evaluation_history(&first.snapshot.locator)
                .unwrap()
                .result,
            original_positive.result
        );
        assert_eq!(
            fixture
                .sdk
                .read_local_evaluation_history(&second.snapshot.locator)
                .unwrap()
                .result,
            original_negative.result
        );
        eprintln!("native retained continuation: four actual revoke refusals and exact immutable histories verified");
    });
}

#[test]
#[ignore = "requires the current Engine checkpoint worker and frozen native producer"]
fn native_governance_and_revocation_use_untouched_operating_generation() {
    on_stack(|| {
        use handbook_contracts::project_admission_governance::{
            ExpectedDeferredBasisV2, ProjectAdmissionScopeBasisV3,
        };
        use handbook_sdk::admission_governance::{
            DeferredEstablishmentExecutionV3, DeferredEstablishmentRequestV3,
        };

        let _package = InstalledProductionPackage::install();
        let fixture = Fixture::new();
        let complete = |key: &str, content: Value| {
            let formed = fixture.form(
                key,
                content,
                production_manifest().resolution_ceiling().clone(),
                LocalEvaluationUseIntentV1::DecisionSeeking,
            );
            fixture.assess(&formed);
            let session = fixture.session(&formed);
            fixture
                .sdk
                .commit_local_evaluation_collection(&session)
                .unwrap();
            fixture
                .sdk
                .execute_local_evaluation_once(
                    &session,
                    formed.process_request.as_ref().unwrap(),
                    &dock_reach(),
                    &DockCancellationV1::new(),
                )
                .unwrap();
            let result = finish(&fixture, &formed);
            (formed, result)
        };
        let (first, positive) = complete(
            "packet3-governance-positive-01",
            supplied_validation_content(),
        );
        assert_eq!(positive.gate, LocalEvaluationGateV3::Pass);
        let mut invalid = supplied_validation_content();
        invalid["instance"]["value"]["amount"] = json!(-1);
        invalid["instance"]["instance_fingerprint"] =
            json!(value_fingerprint(&invalid["instance"]["value"]));
        let (second, negative) = complete("packet3-governance-negative-01", invalid);
        assert_eq!(negative.gate, LocalEvaluationGateV3::Fail);

        exercise_native_checkpoint_substitutions();
        assert_eq!(
            handbook_engine::repository_durability::RetainedRepositoryRootV1::open_exact(
                fixture.root.path()
            )
            .unwrap()
            .existing_lock_physical_fingerprint_v1(".handbook/state/locks/local-operating.lock")
            .unwrap(),
            fixture.authority.operating_lock_fingerprint,
            "the destructive matrix must leave this governance subject's generation untouched"
        );
        eprintln!("isolated native governance: 13 checkpoint cases passed with the governing lock generation unchanged");

        let handoffs = checkpoint_handoffs(&fixture, &first, "generation-before-g");
        let before_g = arm_checkpoints(&handoffs, "generation-actual-g");
        let ProjectAdmissionScopeBasisV3::DeferredV2(d) = first
            .snapshot
            .requirements
            .applicability
            .governance_basis
            .clone()
        else {
            panic!("the untouched fixture requires actual D")
        };
        let establishment = fixture
            .sdk
            .prepare_deferred_project_admission_governance(
                &DeferredEstablishmentRequestV3::human_approved(
                    d.project,
                    ExpectedDeferredBasisV2 {
                        outcome_fingerprint: d.outcome_fingerprint,
                        relation_fingerprint: d.relation_fingerprint,
                    },
                    3600,
                    "native-generation-establish-g",
                    "Illustrative supplied approval of this exact same-project G freshness scope.",
                )
                .unwrap(),
            )
            .unwrap();
        assert!(matches!(
            fixture
                .sdk
                .establish_deferred_project_admission_governance(establishment),
            DeferredEstablishmentExecutionV3::Committed { .. }
        ));
        finish_refused_checkpoints(before_g, "governance_currentness");
        let verify_d_histories = || {
            for (formed, expected) in [(&first, &positive), (&second, &negative)] {
                let history = fixture
                    .sdk
                    .read_local_evaluation_history(&formed.snapshot.locator)
                    .unwrap();
                assert!(history.exact_replay_available);
                assert_eq!(&history.result, expected);
                assert_eq!(
                    fixture
                        .sdk
                        .close_local_evaluation_result(&fixture.session(formed))
                        .unwrap()
                        .result
                        .as_ref(),
                    Some(expected)
                );
            }
        };
        verify_d_histories();
        eprintln!("isolated native governance: four actual G refusals and exact D Pass/Fail histories verified");

        let (fresh_g, fresh_g_result) = complete(
            "packet3-governance-fresh-g-01",
            supplied_validation_content(),
        );
        assert_ne!(fresh_g.request.current_basis, first.request.current_basis);
        assert_eq!(fresh_g.snapshot.definition, first.snapshot.definition);
        assert_eq!(fresh_g_result.gate, LocalEvaluationGateV3::Pass);
        eprintln!("isolated native governance: fresh supported same-project G closed Pass");

        let handoffs = checkpoint_handoffs(&fixture, &fresh_g, "generation-before-revoke");
        let before_revoke = arm_checkpoints(&handoffs, "generation-actual-revoke");
        let current = fixture
            .sdk
            .read_local_evaluation_terms_history(&fixture.authority)
            .outcome
            .unwrap();
        NativeLocalEvaluationAuthorityV1::revoke(
            fixture.root.path(),
            &fixture.authority,
            &LocalRevokeRequestV1 {
                request_key: "native-generation-revoke".into(),
                expected_current_fingerprint: current.outcome_fingerprint,
                principal: fixture.authority.terms.revocation_principal.clone(),
                source_ref: "decision.packet3.illustrative-revoke".into(),
                instruction: "Revoke this exact independently retained evaluation permission."
                    .into(),
            },
            &engine_reach(),
        )
        .unwrap();
        finish_refused_checkpoints(before_revoke, "evaluation_not_current");
        verify_d_histories();
        let history = fixture
            .sdk
            .read_local_evaluation_history(&fresh_g.snapshot.locator)
            .unwrap();
        assert!(history.exact_replay_available);
        assert_eq!(history.result, fresh_g_result);
        assert_eq!(
            fixture
                .sdk
                .close_local_evaluation_result(&fixture.session(&fresh_g))
                .unwrap()
                .result,
            Some(fresh_g_result)
        );
        assert_eq!(
            std::fs::read_dir(
                fixture
                    .root
                    .path()
                    .join(".handbook/state/dock/v1/executions")
            )
            .unwrap()
            .count(),
            3,
            "historical delivery must not execute another producer"
        );
        eprintln!("isolated native governance: four actual revoke refusals and exact D/G histories verified without relaunch");
    });
}

#[path = "local_evaluation_native/owner_acceptance.rs"]
mod owner_acceptance;

#[path = "local_evaluation_native/correction_acceptance.rs"]
mod correction_acceptance;
