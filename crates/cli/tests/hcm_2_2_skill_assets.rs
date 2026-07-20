const ROOT_SKILL: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../install/handbook-home/SKILL.md.tmpl"
));
const CHARTER_SKILL: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../install/handbook-home/charter-intake/SKILL.md.tmpl"
));
const CORE_METHOD: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../core/library/authoring/charter_authoring_method.md"
));

const REQUIRED_COMMANDS: [&str; 6] = [
    "handbook author charter --mode guided-adaptive --from-inputs <path|->",
    "handbook author charter --mode express --from-inputs <path|->",
    "handbook author charter --mode agent-assisted --from-inputs <path|->",
    "handbook author charter --approve-candidate <candidate-ref> --approval-class <class> --authority-ref <authority-ref>",
    "handbook author charter --promote-candidate <candidate-ref> --approval-ref <approval-ref>",
    "handbook author charter --validate",
];

#[test]
fn installed_and_core_charter_assets_publish_the_same_frozen_commands() {
    for command in REQUIRED_COMMANDS {
        assert!(
            CHARTER_SKILL.contains(command),
            "leaf skill missing {command}"
        );
        assert!(
            CORE_METHOD.contains(command),
            "core method missing {command}"
        );
    }
    assert!(ROOT_SKILL.contains("--mode guided-adaptive"));
    assert!(ROOT_SKILL.contains("handbook author charter --validate"));
}

#[test]
fn skill_assets_do_not_author_canonical_truth_or_drive_a_nested_conversation() {
    for asset in [ROOT_SKILL, CHARTER_SKILL, CORE_METHOD] {
        assert!(
            !asset.contains("author charter --validate --from-inputs"),
            "{asset}"
        );
        assert!(!asset.contains("writes the canonical charter"), "{asset}");
        assert!(
            !asset.contains("write `.handbook/project/charter.yaml`"),
            "{asset}"
        );
        assert!(!asset.contains("run a human interview"), "{asset}");
        assert!(!asset.contains("nested model"), "{asset}");
    }
    assert!(CHARTER_SKILL.contains("immutable intake and candidate"));
    assert!(CHARTER_SKILL.contains("human approval"));
    assert!(CHARTER_SKILL.contains("Never edit canonical Charter YAML"));
}
