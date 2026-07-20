use handbook_engine::{
    load_shipped_charter_definition_registry, resolve_shipped_profile_decisions, ExactDefinitionRef,
};

fn exact(value: &str) -> ExactDefinitionRef {
    ExactDefinitionRef::parse(value).unwrap()
}

#[test]
fn shipped_charter_subordinate_definition_registry_is_exact_and_selected() {
    let registry = load_shipped_charter_definition_registry().unwrap();
    assert_eq!(registry.refs().len(), 8);
    for (reference, fingerprint) in [
        (
            "handbook.approval.constitutional-candidate@1.0.0",
            "sha256:67466e0c26e48e7e25705b6a601e487b74923371d886d759fcd9face2e793c29",
        ),
        (
            "handbook.waiver.constitutional-intake@1.0.0",
            "sha256:704acdfe61b4d76ece2ceac72eb99df65dff2d2998b876990f24c3410fc317e3",
        ),
        (
            "handbook.intake-trigger.production-posture-changed@1.0.0",
            "sha256:35df6e191f201517abd07f428b87b9e3970117fb70110b801947116cb04c69f6",
        ),
        (
            "handbook.intake-trigger.trust-boundary-changed@1.0.0",
            "sha256:12950ab5172acce8b0b944f630791a4ffda4960095248fa3b5bea833f7fa4766",
        ),
        (
            "handbook.lifecycle-trigger.charter-amendment-proposed@1.0.0",
            "sha256:9672246337ff266fc07f67053ca053736cb0650d2ad5e053e4a19304acd48ed8",
        ),
        (
            "handbook.renderer.charter-review-markdown@1.0.0",
            "sha256:68f6fceedaab6133364d18a2b474694d1b44b91d73c05d0af65db1aa58e80fa7",
        ),
        (
            "handbook.lifecycle.constitutional-review-lock@1.0.0",
            "sha256:88caafb9caaf137647c42a91cd2762ac0871e0a20e2a1844c2c0076d5fb43cc3",
        ),
        (
            "handbook.intake.charter@1.0.0",
            "sha256:a92229722f25119c7d91137e1feef4ce51b88ae766ce308b585d37f39eb52d1c",
        ),
    ] {
        assert_eq!(
            registry
                .record(&exact(reference))
                .unwrap()
                .definition_fingerprint()
                .as_str(),
            fingerprint
        );
    }

    let decisions = resolve_shipped_profile_decisions(env!("CARGO_MANIFEST_DIR")).unwrap();
    registry.validate_selected_decisions(&decisions).unwrap();
}
