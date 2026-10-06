//! Supplied Packet 2 test inputs. These are illustrative choices, not project defaults.
use handbook_contracts::*;
use handbook_engine::local_authority::local_check::*;
use handbook_engine::{
    CharterAcquisitionMode, CharterCoverageSubmission, CharterIntakeConsumer,
    CharterIntakeEnvelope, CharterIntakeSourceKind,
};
use handbook_sdk::admission_governance::DeferralRequestInputV2;
use handbook_sdk::local_setup::*;
use handbook_sdk::HandbookSdkV1;
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::fs;
use std::path::{Path, PathBuf};

const CHARTER: &[u8] = include_bytes!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../docs/specs/handbook-contract-membrane/slices/HCM-2.2/contracts/canonical-charter-boundary-v1.1.yaml"
));

pub fn supplied_initial_request() -> LocalInitialSetupRequestV1 {
    let content = handbook_engine::parse_definition_yaml(CHARTER).unwrap();
    let mut terms = LocalOperatingTermsV1 {
        request_key: "packet2-initial-terms".into(),
        mode: "local_advisory".into(),
        rule_ref: handbook_engine::local_authority::LOCAL_OPERATING_RULE_V1.into(),
        instruction: "Validate the supplied captured order in the bounded local owner reach."
            .into(),
        principal: "packet2.human".into(),
        agent: "packet2.author".into(),
        delegation: "bounded local order schema validation".into(),
        approval_source_ref: "decision.packet2.initial-terms".into(),
        human_approval_asserted: true,
        approved_content_fingerprint: String::new(),
        ceiling: LocalEnvelopeCeilingV1 {
            active_level_id: "execution".into(),
            dimensions: [
                "assigned_unit",
                "normal",
                "immediate",
                "local_write",
                "execution",
                "unit_closeout",
            ]
            .map(str::to_owned),
        },
        constraint_inputs: vec![],
        allowed_input_paths: vec!["inputs/*".into()],
        allowed_paths: vec![
            ".handbook/state/contracts/**".into(),
            ".handbook/state/dock/**".into(),
        ],
        operation_classes: vec![
            LocalOperationClassV1::LocalValidation,
            LocalOperationClassV1::OwnerWrite,
        ],
        reserved_decisions: vec![
            LocalDecisionClassV1::PathExpansion,
            LocalDecisionClassV1::CharterReplacement,
            LocalDecisionClassV1::TermsReplacement,
        ],
        delegated_decisions: vec![LocalDecisionClassV1::InputCorrection],
        revocation_principal: "packet2.human".into(),
    };
    terms.approved_content_fingerprint = terms.fingerprint_approved_content().unwrap();
    let coverage = [
        "project_shape.definition",
        "delivery.constraints",
        "delivery.default_implications",
        "operational_reality.production_state",
        "risk.domains",
        "engineering_posture.baseline",
        "policy.authority_and_revision",
        "governance.decision_authority",
        "governance.required_approvals",
        "governance.exception_policy",
        "engineering_posture.dimensions",
        "engineering_posture.red_lines",
        "governance.review_triggers",
        "governance.reassessment_triggers",
        "debt.register",
        "decisions.records",
    ]
    .into_iter()
    .map(|coverage_id| CharterCoverageSubmission {
        coverage_id: coverage_id.into(),
        source_kind: CharterIntakeSourceKind::UserDeclaration,
        value_ref: format!("intake-values/{coverage_id}.json"),
        evidence_refs: vec![],
        confidence: "high".into(),
        freshness: None,
        sensitivity: "internal".into(),
        contradiction_refs: vec![],
        waiver_ref: None,
    })
    .collect();
    LocalInitialSetupRequestV1 {
        deferral: DeferralRequestInputV2::human_approved(
            "packet2-deferral",
            "The supplied test human approved this exact scoped deferral.",
        ),
        charter: NativeLocalCharterRequestV1 {
            request_key: "packet2-charter".into(),
            intake: CharterIntakeEnvelope {
                mode: CharterAcquisitionMode::Express,
                content,
                coverage,
                consumer: CharterIntakeConsumer {
                    kind: "sdk_consumer".into(),
                    id: "packet2-test".into(),
                    version: "1.0.0".into(),
                },
                prompt_event_refs: vec!["decision.packet2.charter".into()],
                finalized_at_utc: "2026-10-05T12:00:00Z".into(),
                expected_current_fingerprint: None,
            },
            human_approval: LocalCharterHumanApprovalV1 {
                accepted: true,
                principal_claim: "packet2.human".into(),
                agent_claim: "packet2.author".into(),
                delegation_claim: "initial local Charter establishment".into(),
                source_ref: "decision.packet2.charter".into(),
                approved_content_fingerprint: handbook_engine::charter_source_fingerprint(CHARTER)
                    .to_string(),
            },
        },
        terms,
    }
}

pub fn establish_initial(sdk: &HandbookSdkV1) -> LocalInitialSetupLocatorV1 {
    let request = supplied_initial_request();
    let prepared = sdk.prepare_local_initial_setup(&request).unwrap();
    let locator = prepared.locator().clone();
    let outcome = sdk.establish_local_initial_setup(prepared);
    assert!(outcome.stopped.is_none(), "{outcome:?}");
    assert_eq!(
        outcome.charter.unwrap().disposition,
        LocalCharterDispositionV1::Committed
    );
    assert_eq!(
        outcome.terms.unwrap().disposition,
        LocalAuthorityDispositionV1::Committed
    );
    locator
}

pub fn value_fingerprint(value: &Value) -> String {
    let bytes = serde_json_canonicalizer::to_vec(value).unwrap();
    format!("sha256:{:x}", Sha256::digest(bytes))
}

pub fn supplied_validation_content() -> Value {
    let schema = json!({
        "$schema": "https://json-schema.org/draft/2020-12/schema", "type": "object",
        "required": ["order_id", "amount"], "additionalProperties": false,
        "properties": {"order_id": {"type": "string", "minLength": 1}, "amount": {"type": "integer", "minimum": 0}}
    });
    let instance = json!({"order_id": "order-42", "amount": 125});
    let schema_fingerprint = value_fingerprint(&schema);
    let members = json!([{"path": "definitions/schemas/order/1.0.0.schema.json", "document_fingerprint": schema_fingerprint}]);
    json!({
        "root_schema": {"ref": "handbook.schema.order@1.0.0", "fingerprint": schema_fingerprint},
        "root_document": "definitions/schemas/order/1.0.0.schema.json",
        "root_document_fingerprint": schema_fingerprint,
        "closure_fingerprint": value_fingerprint(&members),
        "documents": [{"path": "definitions/schemas/order/1.0.0.schema.json", "schema": schema, "document_fingerprint": schema_fingerprint}],
        "instance": {"value": instance, "instance_fingerprint": value_fingerprint(&instance)}
    })
}

pub fn production_manifest() -> handbook_dock::DockManifestV1 {
    let registry = handbook_dock::DockManifestRegistryV1::production();
    let page = registry
        .list(handbook_dock::DockManifestListRequestV1::first(1).unwrap())
        .unwrap();
    match registry
        .read(
            page.entries()[0].manifest_ref(),
            page.entries()[0].manifest_fingerprint(),
        )
        .unwrap()
    {
        handbook_dock::DockManifestReadOutcomeV1::Found(manifest) => *manifest,
        handbook_dock::DockManifestReadOutcomeV1::Absent => {
            panic!("native production manifest is absent")
        }
    }
}

pub struct InstalledProductionPackage {
    package_root: PathBuf,
    host_directory: PathBuf,
    created: bool,
}

impl InstalledProductionPackage {
    pub fn install() -> Self {
        let host_executable = std::env::current_exe().unwrap();
        let host_directory = host_executable.parent().unwrap().to_owned();
        let manifest = production_manifest();
        let executable = std::env::var_os("HANDBOOK_PACKET2_VALIDATOR_EXE")
            .map(PathBuf::from)
            .unwrap_or_else(|| {
                Path::new(env!("CARGO_MANIFEST_DIR"))
                    .join("../../target/debug")
                    .join(format!(
                        "handbook-dock-json-schema{}",
                        std::env::consts::EXE_SUFFIX
                    ))
            });
        let bytes = fs::read(&executable).expect("build the exact native validator with the selected /Brepro argument before this source test");
        let entry = &manifest.implementation_bundle().files()[0];
        assert_eq!(bytes.len() as u64, entry.length());
        assert_eq!(
            format!("sha256:{:x}", Sha256::digest(&bytes)),
            entry.sha256().as_str()
        );
        let package_root = host_directory
            .join("handbook-docks/handbook.dock.json-schema/1.0.0")
            .join(handbook_dock::host_target_tuple_v1());
        if package_root.exists() {
            // Reuse only the exact owner-resolved production package. It may
            // already be staged by a prior interrupted proof; never overwrite
            // or remove a package this invocation did not create.
            handbook_dock::DockResolvedFirstPartyBundleV1::resolve_production().unwrap();
            return Self {
                package_root,
                host_directory,
                created: false,
            };
        }
        let staged_executable = package_root
            .join("bundle")
            .join(manifest.implementation_bundle().entrypoint().as_str());
        fs::create_dir_all(staged_executable.parent().unwrap()).unwrap();
        fs::write(&staged_executable, bytes).unwrap();
        fs::write(
            package_root.join("manifest.json"),
            manifest.canonical_json_bytes().unwrap(),
        )
        .unwrap();
        handbook_dock::DockResolvedFirstPartyBundleV1::resolve_production().unwrap();
        Self {
            package_root,
            host_directory,
            created: true,
        }
    }
}

impl Drop for InstalledProductionPackage {
    fn drop(&mut self) {
        assert!(self
            .package_root
            .starts_with(self.host_directory.join("handbook-docks")));
        if self.created && self.package_root.exists() {
            fs::remove_dir_all(&self.package_root).unwrap();
        }
    }
}

pub fn id(value: &str) -> StableIdV1 {
    StableIdV1::parse(value).unwrap()
}

pub fn schema_binding(reference: &str) -> ExactBindingV1 {
    static WITNESS: std::sync::OnceLock<handbook_sdk::contract_membrane::ProductionWitness> =
        std::sync::OnceLock::new();
    let witness = WITNESS.get_or_init(|| {
        handbook_sdk::contract_membrane::ProductionWitness::from_current(
            handbook_sdk::contract_membrane::UtcTimestamp::parse("9999-12-31T23:59:59Z").unwrap(),
        )
        .unwrap()
    });
    let schema = witness
        .schema_manifest()
        .iter()
        .find(|s| s.schema().reference().as_str() == reference)
        .unwrap()
        .schema();
    ExactBindingV1::new(
        ExactRefV1::parse(reference).unwrap(),
        FingerprintV1::parse(schema.fingerprint().as_str()).unwrap(),
    )
}

pub fn supplied_definition(
    permitted_use: LocalPermittedUseV2,
    age_seconds: u64,
    author_authority: ExactBindingV1,
) -> (
    LocalContractDefinitionV2,
    handbook_engine::ContextResolutionStackDefinition,
) {
    supplied_definition_with_gate_effect(
        permitted_use,
        age_seconds,
        author_authority,
        GateEffectV1::Required,
    )
}

pub fn supplied_definition_with_gate_effect(
    permitted_use: LocalPermittedUseV2,
    age_seconds: u64,
    author_authority: ExactBindingV1,
    gate_effect: GateEffectV1,
) -> (
    LocalContractDefinitionV2,
    handbook_engine::ContextResolutionStackDefinition,
) {
    supplied_definition_with_version(
        permitted_use,
        age_seconds,
        author_authority,
        gate_effect,
        "1.0.0",
    )
}

pub fn supplied_definition_with_version(
    permitted_use: LocalPermittedUseV2,
    age_seconds: u64,
    author_authority: ExactBindingV1,
    gate_effect: GateEffectV1,
    version: &str,
) -> (
    LocalContractDefinitionV2,
    handbook_engine::ContextResolutionStackDefinition,
) {
    let profile = handbook_engine::local_authority::resolved_local_operating_profile().unwrap();
    let stack = profile.context_resolution().clone();
    let manifest = production_manifest();
    let minimum = manifest.resolution_ceiling();
    let minimum_values = serde_json::to_value(minimum).unwrap();
    let freshness = EvidenceFreshnessPolicyV1::new(
        EvidenceFreshnessPolicyInputV1::new(
            ContractIdV1::parse("handbook.freshness.order-capture").unwrap(),
            VersionV1::parse("1.0.0").unwrap(),
            age_seconds,
            true,
            0,
        )
        .unwrap(),
    )
    .unwrap();
    let gate = GatePolicyV1::new(
        GatePolicyInputV1::new(
            ContractIdV1::parse("handbook.gate.order-capture").unwrap(),
            VersionV1::parse("1.0.0").unwrap(),
            GatePolicyAlgorithmV1::ClosedAccounting,
            None,
            local_closeout_policy_binding_v1().unwrap(),
        )
        .unwrap(),
    )
    .unwrap();
    let selector = SelectorV1::new(
        MatcherBindingV1::exact_fields_v1(),
        vec![SelectorCriterionV1::new(
            id("dock.input.role"),
            "handbook.input.json-schema-validation",
        )
        .unwrap()],
    )
    .unwrap();
    let semantic_definition = ContractDefinitionV1::new(ContractDefinitionInputV1 {
        definition_schema: schema_binding("handbook.contract-definition@1.0.0"),
        contract_id: ContractIdV1::parse("handbook.contract.order-capture").unwrap(),
        contract_version: VersionV1::parse(version).unwrap(),
        contract_kind: ContractKindV1::ClaimSet,
        claims: vec![ContractClaimV1::new(ContractClaimInputV1 {
            claim_id: id("payload.schema-conformant"),
            statement:
                "The exact captured order conforms to its exact supplied offline schema closure."
                    .into(),
            subject_selector: selector.clone(),
            case_selector: selector,
            applicability_rule: ApplicabilityRuleV1::SubjectAndCaseMatch,
            gate_effect,
            evidence_requirements: vec![EvidenceRequirementV1::new(
                id("handbook.evidence.schema-conformance"),
                EvidenceCardinalityV1::ExactlyOne,
                EvidenceCaseSelectionV1::ClaimCaseSelector,
                evidence_stability_policy_binding_v1().unwrap(),
            )
            .unwrap()],
            freshness_policy: freshness.exact_binding(),
            minimum_resolution: ResolutionMinimumV1::new(
                minimum.resolution_stack().clone(),
                minimum.scope_horizon().clone(),
                id(minimum_values["detail_resolution"].as_str().unwrap()),
                id(minimum_values["temporal_horizon"].as_str().unwrap()),
                id(minimum_values["authority_horizon"].as_str().unwrap()),
                id(minimum_values["memory_horizon"].as_str().unwrap()),
                id(minimum_values["validation_horizon"].as_str().unwrap()),
            ),
            score_weight: None,
        })
        .unwrap()],
        definition_author_ref: id("packet2.author"),
        definition_authority: author_authority,
        lifecycle_policy: local_lifecycle_policy_binding_v2().unwrap(),
        matcher_bindings: vec![MatcherBindingV1::exact_fields_v1()],
        gate_policy: gate.exact_binding(),
        extensions_policy: local_extensions_policy_binding_v2().unwrap(),
    })
    .unwrap();
    let content = supplied_validation_content();
    let document = LocalSchemaDocumentV2::new(
        content["root_document"].as_str().unwrap().into(),
        content["documents"][0]["schema"].clone(),
    )
    .unwrap();
    let root_schema = ExactBindingV1::new(
        ExactRefV1::parse(content["root_schema"]["ref"].as_str().unwrap()).unwrap(),
        document.document_fingerprint.clone(),
    );
    let schema_closure =
        LocalSchemaClosureV2::new(root_schema, document.path.clone(), vec![document]).unwrap();
    let fact_schema = LocalExactContentV2::from_source_bytes(
        ExactRefV1::parse("handbook.fact.schema-conformance@1.0.0").unwrap(),
        include_bytes!("../../schemas/2020-12/handbook.fact.schema-conformance/1.0.0.schema.json"),
    )
    .unwrap();
    let dependencies = LocalDefinitionDependenciesV2 {
        definition_schema: LocalExactContentV2::from_source_bytes(
            ExactRefV1::parse("handbook.contract-definition@1.0.0").unwrap(),
            include_bytes!("../../schemas/2020-12/handbook.contract-definition/1.0.0.schema.json"),
        )
        .unwrap(),
        extensions_policy: local_extensions_policy_binding_v2().unwrap(),
        freshness_policies: vec![freshness],
        gate_policy: gate,
        stability_policy: evidence_stability_policy_binding_v1().unwrap(),
        evidence_admission_policy: local_evidence_admission_policy_binding_v2().unwrap(),
        lifecycle_policy: local_lifecycle_policy_binding_v2().unwrap(),
        fact_schema,
        validator: LocalValidatorDependencyV2::from_manifest_json(
            &manifest.canonical_json_bytes().unwrap(),
        )
        .unwrap(),
        schema_closure,
        required_governance_scopes: vec![],
    };
    (
        LocalContractDefinitionV2::new(
            LocalContractDefinitionInputV2 {
                semantic_definition,
                permitted_use,
                dependencies,
            },
            &stack,
        )
        .unwrap(),
        stack,
    )
}

pub fn supplied_successor_terms(
    initial: &LocalInitialSetupLocatorV1,
    predecessor_fingerprint: &str,
) -> LocalCheckTermsV1 {
    let original = &initial.terms.terms;
    let mut terms = LocalCheckTermsV1 {
        request_key: "packet2-compatible-local-check".into(), mode: "local_advisory".into(),
        rule_ref: LOCAL_CHECK_RULE_V1.into(),
        instruction: "Author this complete exact local order contract, have the selected reviewer review it, and resolve its captured native validator sources.".into(),
        principal: original.principal.clone(), agent: original.agent.clone(),
        delegation: "bounded author, actual review, lifecycle and captured source resolution".into(),
        approval_source_ref: "decision.packet2.compatible-successor-terms".into(),
        human_approval_asserted: true, approved_content_fingerprint: String::new(),
        expected_predecessor_fingerprint: predecessor_fingerprint.into(),
        expected_charter_fingerprint: initial.charter.canonical_fingerprint.clone(),
        ceiling: original.ceiling.clone(), constraint_inputs: original.constraint_inputs.clone(),
        allowed_paths: vec![".handbook/state/contracts/**".into(), ".handbook/state/dock/**".into()],
        allowed_input_paths: vec![".handbook/state/contracts/**".into(), ".handbook/state/dock/**".into(), ".handbook/state/locks/contract-owner.lock".into(), ".handbook/state/locks/dock-operational-store.lock".into(), "inputs/*".into()],
        action_classes: vec![LocalCheckActionV1::DefinitionAuthor, LocalCheckActionV1::Review, LocalCheckActionV1::Lifecycle, LocalCheckActionV1::EvidenceSource],
        author_participant_ref: "packet2.author".into(), review_participant_refs: vec!["packet2.reviewer".into()],
        reserved_decisions: original.reserved_decisions.clone(), delegated_decisions: original.delegated_decisions.clone(),
        revocation_principal: original.revocation_principal.clone(),
    };
    terms.approved_content_fingerprint = terms.fingerprint_approved_content().unwrap();
    terms
}

pub fn adopt_check(
    sdk: &HandbookSdkV1,
    _root: &Path,
    initial: &LocalInitialSetupLocatorV1,
) -> (LocalCheckLocatorV1, LocalCheckAuthorityOutcomeV1) {
    let original = sdk
        .resolve_local_initial_setup_request(initial)
        .unwrap()
        .terms
        .unwrap()
        .outcome
        .unwrap();
    let prepared = sdk
        .prepare_local_check_terms(
            initial,
            supplied_successor_terms(initial, &original.outcome_fingerprint),
        )
        .unwrap();
    let locator = prepared.locator().clone();
    let execution = sdk.adopt_local_check_terms(prepared);
    assert_eq!(
        execution.disposition,
        LocalAuthorityDispositionV1::Committed,
        "{execution:?}"
    );
    (locator, execution.outcome.unwrap())
}

pub fn author_authority(outcome: &LocalCheckAuthorityOutcomeV1) -> ExactBindingV1 {
    ExactBindingV1::new(
        ExactRefV1::parse(LOCAL_CHECK_RULE_V1).unwrap(),
        FingerprintV1::parse(&outcome.outcome_fingerprint).unwrap(),
    )
}

pub fn supplied_review_outcome(definition: &LocalContractDefinitionV2) -> LocalReviewInputV2 {
    // This supplied outcome represents the selected reviewer having inspected
    // this exact closure. It asserts advisory attribution, not authenticated
    // independence, and it is not obtained by relabeling the author.
    assert_eq!(definition.semantic_definition().claims().len(), 1);
    assert_eq!(
        definition
            .dependencies()
            .schema_closure
            .root_schema
            .fingerprint(),
        &definition.dependencies().schema_closure.documents[0].document_fingerprint
    );
    LocalReviewInputV2 {
        definition: definition.binding(), closure_fingerprint: definition.closure_fingerprint().clone(),
        permitted_use: definition.permitted_use(), author_participant_ref: "packet2.author".into(),
        reviewer_participant_ref: "packet2.reviewer".into(), reviewer_actor_ref: "packet2.reviewer".into(),
        decision: LocalReviewDecisionV2::Approve, source_ref: "review.packet2.exact-order-contract".into(),
        outcome_text: "Approve after reviewing the exact supplied order conformance claim, captured role selectors, required single native schema-conformance fact, immutable permitted use, exact matcher and Resolution minimum, selected freshness/stability/gate rules, local lifecycle/evidence rules, native validator and complete offline schema dependency closure. This is the declared distinct reviewer's supplied local advisory outcome.".into(),
        semantic_time: SemanticTimeV1::parse("2026-10-05T12:00:00Z").unwrap(),
    }
}

pub fn project(
    initial: &LocalInitialSetupLocatorV1,
) -> handbook_contracts::project_admission_governance::ProjectAdmissionBindingV3 {
    handbook_contracts::project_admission_governance::ProjectAdmissionBindingV3::ScopedChoiceV2(
        initial.deferral.project.clone(),
    )
}

pub fn append_definition(
    root: &Path,
    _owner: &ContractOwnerRepositoryV1,
    initial: &LocalInitialSetupLocatorV1,
    locator: &LocalCheckLocatorV1,
    definition: LocalContractDefinitionV2,
    stack: &handbook_engine::ContextResolutionStackDefinition,
    key: &str,
) -> LocalOwnerMutationReceiptV2 {
    let sdk = HandbookSdkV1::open(root);
    let prepared = sdk
        .prepare_local_contract_definition(initial, definition, stack)
        .unwrap();
    sdk.append_local_contract_definition(
        locator,
        prepared,
        SemanticTimeV1::parse("2026-10-05T12:00:00Z").unwrap(),
        key,
    )
    .unwrap()
}

pub fn transition(
    root: &Path,
    owner: &ContractOwnerRepositoryV1,
    initial: &LocalInitialSetupLocatorV1,
    locator: &LocalCheckLocatorV1,
    definition: &LocalContractDefinitionV2,
    state: LifecycleStateV1,
    review: Option<LocalReviewInputV2>,
    key: &str,
) -> LocalOwnerMutationReceiptV2 {
    let head = owner
        .current_local_lifecycle_head_v2(&definition.binding())
        .unwrap();
    let sdk = HandbookSdkV1::open(root);
    let prepared = sdk
        .prepare_local_contract_lifecycle(
            initial,
            &definition.binding(),
            LocalLifecycleTransitionInputV2 {
                expected_lifecycle_fingerprint: head.lifecycle_fingerprint().clone(),
                to_state: state,
                review: review.clone(),
                reason: format!("The supplied authority requests this exact {state:?} edge."),
            },
        )
        .unwrap();
    let (actor, participant, time) = match review {
        Some(r) => (
            r.reviewer_actor_ref,
            r.reviewer_participant_ref,
            r.semantic_time,
        ),
        None => (
            "packet2.author".into(),
            "packet2.author".into(),
            SemanticTimeV1::parse("2026-10-05T12:00:00Z").unwrap(),
        ),
    };
    sdk.transition_local_contract_lifecycle(locator, prepared, actor, participant, time, key)
        .unwrap()
}

pub fn activate_definition(
    root: &Path,
    owner: &ContractOwnerRepositoryV1,
    initial: &LocalInitialSetupLocatorV1,
    locator: &LocalCheckLocatorV1,
    definition: &LocalContractDefinitionV2,
) -> LocalLifecycleHeadV2 {
    append_definition(
        root,
        owner,
        initial,
        locator,
        definition.clone(),
        &handbook_engine::local_authority::resolved_local_operating_profile()
            .unwrap()
            .context_resolution()
            .clone(),
        "packet2-definition-author",
    );
    transition(
        root,
        owner,
        initial,
        locator,
        definition,
        LifecycleStateV1::ReviewReady,
        None,
        "packet2-review-ready",
    );
    transition(
        root,
        owner,
        initial,
        locator,
        definition,
        LifecycleStateV1::Locked,
        Some(supplied_review_outcome(definition)),
        "packet2-actual-review",
    );
    let active = transition(
        root,
        owner,
        initial,
        locator,
        definition,
        LifecycleStateV1::Active,
        None,
        "packet2-activate-reviewed",
    );
    assert_eq!(active.lifecycle_head.state(), LifecycleStateV1::Active);
    active.lifecycle_head
}
