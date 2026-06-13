use std::fs;
use std::path::PathBuf;
use std::process::Command;

fn normalize_newlines(value: &str) -> String {
    value.replace("\r\n", "\n")
}

fn help_snapshot_path(filename: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("snapshots")
        .join(filename)
}

fn run_help(args: &[&str], command_name: &str) -> String {
    let exe = env!("CARGO_BIN_EXE_handbook");

    let output = Command::new(exe)
        .args(args)
        .env("CLAP_TERM_WIDTH", "120")
        .env("NO_COLOR", "1")
        .env("CLICOLOR", "0")
        .output()
        .unwrap_or_else(|err| panic!("run `{command_name}`: {err}"));

    assert!(
        output.status.success(),
        "`{command_name}` failed (status: {})",
        output.status
    );

    normalize_newlines(
        std::str::from_utf8(&output.stdout)
            .unwrap_or_else(|err| panic!("`{command_name}` stdout must be UTF-8: {err}")),
    )
}

fn read_help_snapshot(filename: &str) -> String {
    let snapshot_path = help_snapshot_path(filename);
    normalize_newlines(
        &fs::read_to_string(&snapshot_path)
            .unwrap_or_else(|err| panic!("read {}: {}", snapshot_path.display(), err)),
    )
}

fn markdown_h2_section<'a>(text: &'a str, heading: &str) -> &'a str {
    let start = text
        .find(heading)
        .unwrap_or_else(|| panic!("missing section heading `{heading}`"));
    let rest = &text[start..];
    let next = rest[heading.len()..]
        .find("\n## `")
        .map(|offset| heading.len() + offset);
    match next {
        Some(end) => &rest[..end],
        None => rest,
    }
}

fn assert_help_matches_snapshot(args: &[&str], snapshot_filename: &str, command_name: &str) {
    let actual = run_help(args, command_name);
    let snapshot_path = help_snapshot_path(snapshot_filename);
    let expected = normalize_newlines(
        &fs::read_to_string(&snapshot_path)
            .unwrap_or_else(|err| panic!("read {}: {}", snapshot_path.display(), err)),
    );

    assert_eq!(
        actual,
        expected,
        "{command_name} output drifted. If intentional, update the snapshot at {}.",
        snapshot_path.display()
    );
}

#[test]
fn handbook_help_matches_snapshot() {
    assert_help_matches_snapshot(&["--help"], "handbook-help.txt", "handbook --help");
}

#[test]
fn handbook_setup_help_matches_snapshot() {
    assert_help_matches_snapshot(
        &["setup", "--help"],
        "handbook-setup-help.txt",
        "handbook setup --help",
    );
}

#[test]
fn handbook_setup_init_help_matches_snapshot() {
    assert_help_matches_snapshot(
        &["setup", "init", "--help"],
        "handbook-setup-init-help.txt",
        "handbook setup init --help",
    );
}

#[test]
fn handbook_setup_refresh_help_matches_snapshot() {
    assert_help_matches_snapshot(
        &["setup", "refresh", "--help"],
        "handbook-setup-refresh-help.txt",
        "handbook setup refresh --help",
    );
}

#[test]
fn handbook_author_help_matches_snapshot() {
    assert_help_matches_snapshot(
        &["author", "--help"],
        "handbook-author-help.txt",
        "handbook author --help",
    );
}

#[test]
fn handbook_author_charter_help_matches_snapshot() {
    assert_help_matches_snapshot(
        &["author", "charter", "--help"],
        "handbook-author-charter-help.txt",
        "handbook author charter --help",
    );
}

#[test]
fn handbook_author_project_context_help_matches_snapshot() {
    assert_help_matches_snapshot(
        &["author", "project-context", "--help"],
        "handbook-author-project-context-help.txt",
        "handbook author project-context --help",
    );
}

#[test]
fn handbook_author_environment_inventory_help_matches_snapshot() {
    assert_help_matches_snapshot(
        &["author", "environment-inventory", "--help"],
        "handbook-author-environment-inventory-help.txt",
        "handbook author environment-inventory --help",
    );
}

#[test]
fn handbook_generate_help_matches_snapshot() {
    assert_help_matches_snapshot(
        &["generate", "--help"],
        "handbook-generate-help.txt",
        "handbook generate --help",
    );
}

#[test]
fn handbook_inspect_help_matches_snapshot() {
    assert_help_matches_snapshot(
        &["inspect", "--help"],
        "handbook-inspect-help.txt",
        "handbook inspect --help",
    );
}

#[test]
fn handbook_doctor_help_matches_snapshot() {
    assert_help_matches_snapshot(
        &["doctor", "--help"],
        "handbook-doctor-help.txt",
        "handbook doctor --help",
    );
}

#[test]
fn handbook_pipeline_help_matches_snapshot() {
    assert_help_matches_snapshot(
        &["pipeline", "--help"],
        "handbook-pipeline-help.txt",
        "handbook pipeline --help",
    );
}

#[test]
fn handbook_pipeline_state_help_matches_snapshot() {
    assert_help_matches_snapshot(
        &["pipeline", "state", "--help"],
        "handbook-pipeline-state-help.txt",
        "handbook pipeline state --help",
    );
}

#[test]
fn handbook_pipeline_compile_help_matches_snapshot() {
    assert_help_matches_snapshot(
        &["pipeline", "compile", "--help"],
        "handbook-pipeline-compile-help.txt",
        "handbook pipeline compile --help",
    );
}

#[test]
fn handbook_pipeline_capture_help_matches_snapshot() {
    assert_help_matches_snapshot(
        &["pipeline", "capture", "--help"],
        "handbook-pipeline-capture-help.txt",
        "handbook pipeline capture --help",
    );
}

#[test]
fn handbook_pipeline_capture_apply_help_matches_snapshot() {
    assert_help_matches_snapshot(
        &["pipeline", "capture", "apply", "--help"],
        "handbook-pipeline-capture-apply-help.txt",
        "handbook pipeline capture apply --help",
    );
}

#[test]
fn handbook_pipeline_handoff_help_matches_snapshot() {
    assert_help_matches_snapshot(
        &["pipeline", "handoff", "--help"],
        "handbook-pipeline-handoff-help.txt",
        "handbook pipeline handoff --help",
    );
}

#[test]
fn handbook_pipeline_handoff_emit_help_matches_snapshot() {
    assert_help_matches_snapshot(
        &["pipeline", "handoff", "emit", "--help"],
        "handbook-pipeline-handoff-emit-help.txt",
        "handbook pipeline handoff emit --help",
    );
}

#[test]
fn support_story_docs_match_help_snapshots() {
    let root = workspace_root();
    let root_readme_path = root.join("README.md");
    let root_readme_text = fs::read_to_string(&root_readme_path)
        .unwrap_or_else(|err| panic!("read {}: {}", root_readme_path.display(), err));
    let docs = [
        root_readme_path.clone(),
        root.join("docs/README.md"),
        root.join("docs/START_HERE.md"),
        root.join("docs/SUPPORTED_COMMANDS.md"),
        root.join("docs/contracts/C-02-rust-workspace-and-cli-command-surface.md"),
    ];
    let docs_text = docs
        .iter()
        .map(|path| {
            fs::read_to_string(path)
                .unwrap_or_else(|err| panic!("read {}: {}", path.display(), err))
        })
        .collect::<Vec<_>>()
        .join("\n");

    let top_help_text = read_help_snapshot("handbook-help.txt");
    let setup_help_text = read_help_snapshot("handbook-setup-help.txt");
    let setup_init_help_text = read_help_snapshot("handbook-setup-init-help.txt");
    let setup_refresh_help_text = read_help_snapshot("handbook-setup-refresh-help.txt");
    let author_help_text = read_help_snapshot("handbook-author-help.txt");
    let author_charter_help_text = read_help_snapshot("handbook-author-charter-help.txt");
    let author_project_context_help_text =
        read_help_snapshot("handbook-author-project-context-help.txt");
    let author_environment_inventory_help_text =
        read_help_snapshot("handbook-author-environment-inventory-help.txt");
    let generate_help_text = read_help_snapshot("handbook-generate-help.txt");
    let inspect_help_text = read_help_snapshot("handbook-inspect-help.txt");
    let pipeline_help_text = read_help_snapshot("handbook-pipeline-help.txt");
    let compile_help_text = read_help_snapshot("handbook-pipeline-compile-help.txt");
    let capture_help_text = read_help_snapshot("handbook-pipeline-capture-help.txt");
    let capture_apply_help_text = read_help_snapshot("handbook-pipeline-capture-apply-help.txt");

    let top_level_required_phrases = [
        "planning packet generation",
        "canonical repo-local `.handbook/`",
        "fixture-backed execution demo",
        "execution.demo.packet",
        "live execution is explicitly refused",
        "`inspect` is the packet proof surface",
        "`doctor` is the recovery surface",
        "explicit stage compilation",
        "explicit stage-output capture",
    ];
    let setup_story_required_doc_phrases = [
        "The public setup family is `handbook setup`, `handbook setup init`, and `handbook setup refresh`.",
        "Bare `handbook setup` routes to `setup init` when canonical `.handbook/` truth is absent or invalid; otherwise it routes to `setup refresh`.",
        "`setup refresh` preserves canonical files by default",
        "`setup refresh --rewrite` rewrites only setup-owned starter files",
        "`setup refresh --reset-state` resets only `.handbook/state/**`",
        "The shipped starter templates are scaffolding only.",
        "Setup hands off to `handbook doctor`, which renders an ordered checklist",
    ];
    let ownership_required_doc_phrases = [
        "direct owner crates for extracted logic",
        "narrow compatibility/support seam",
        "default owner surfaces for extracted logic rather than routing that logic through `crates/compiler`",
    ];
    let root_readme_required_phrases = [
        "pipeline capture --preview",
        "pipeline capture apply --capture-id <capture-id>",
        "stage.04_charter_inputs",
        "stage.05_charter_synthesize",
        "stage.06_project_context_interview",
        "stage.07_foundation_pack",
        "stage.10_feature_spec",
        "only supported stage-output writer surface",
        "payload-only",
        "compile -> external model output -> capture",
        "`handbook`-coordinated single-writer flows",
    ];
    let author_required_doc_phrases = [
        "The public baseline authoring family is `handbook author charter`, `handbook author project-context`, and `handbook author environment-inventory`.",
        "`author` is the baseline authoring surface.",
        "`handbook author charter` is the human-guided surface.",
        "`handbook author charter --from-inputs <path|->` is the agent and automation surface.",
        "`handbook author project-context`",
        "`handbook author environment-inventory`",
    ];
    let stage_10_required_doc_phrases = [
        "external model output",
        "completed `FEATURE_SPEC.md`",
        "compile emits model input payload",
        "raw `pipeline compile` payload is refused as `invalid_capture_input`",
    ];
    let stage_10_banned_doc_phrases = [
        "compile-to-capture handoff",
        "compile | capture",
        "payload stdout piped into capture",
    ];

    for phrase in root_readme_required_phrases {
        assert!(
            root_readme_text.contains(phrase),
            "root README missing capture boundary phrase `{phrase}`"
        );
    }

    let phrase = "Repo-specific note:";
    assert!(
        !root_readme_text.contains(phrase),
        "root README must not keep stale checkout-specific setup note `{phrase}`"
    );

    for phrase in stage_10_required_doc_phrases {
        assert!(
            docs_text.contains(phrase),
            "docs missing M4 stage-10 boundary phrase `{phrase}`"
        );
    }

    for phrase in stage_10_banned_doc_phrases {
        assert!(
            !docs_text.contains(phrase),
            "docs must not reintroduce invalid M4 stage-10 wording `{phrase}`"
        );
    }

    for phrase in top_level_required_phrases {
        assert!(
            docs_text.contains(phrase),
            "docs missing supported-story phrase `{phrase}`"
        );
        assert!(
            top_help_text.contains(phrase),
            "top-level help snapshot missing supported-story phrase `{phrase}`"
        );
    }

    for phrase in setup_story_required_doc_phrases {
        assert!(
            docs_text.contains(phrase),
            "docs missing setup-family phrase `{phrase}`"
        );
    }

    for phrase in author_required_doc_phrases {
        assert!(
            docs_text.contains(phrase),
            "docs missing author-surface phrase `{phrase}`"
        );
    }

    for phrase in ownership_required_doc_phrases {
        assert!(
            docs_text.contains(phrase),
            "docs missing ownership-boundary phrase `{phrase}`"
        );
    }

    for phrase in [
        "Initialize or refresh canonical repo-local `.handbook/` inputs",
        "init",
        "refresh",
    ] {
        assert!(
            setup_help_text.contains(phrase),
            "setup help snapshot missing phrase `{phrase}`"
        );
    }

    assert!(
        top_help_text.contains("Initialize or refresh canonical repo-local `.handbook/` inputs"),
        "top-level help snapshot missing setup-family description"
    );
    assert!(
        author_help_text.contains("Human-guided and deterministic baseline authoring surfaces"),
        "author help snapshot missing author-family description"
    );
    for phrase in ["project-context", "environment-inventory"] {
        assert!(
            author_help_text.contains(phrase),
            "author help snapshot missing phrase `{phrase}`"
        );
    }
    assert!(
        author_charter_help_text.contains("--from-inputs <path|->"),
        "author charter help snapshot missing deterministic input flag"
    );
    assert!(
        author_project_context_help_text.contains(".handbook/project_context/PROJECT_CONTEXT.md"),
        "author project-context help snapshot missing canonical path"
    );
    assert!(
        author_project_context_help_text.contains("--from-inputs <path|->"),
        "author project-context help snapshot missing structured input flag"
    );
    assert!(
        author_environment_inventory_help_text
            .contains(".handbook/environment_inventory/ENVIRONMENT_INVENTORY.md"),
        "author environment-inventory help snapshot missing canonical path"
    );
    assert!(
        setup_init_help_text.contains(
            "Create canonical `.handbook/` scaffold and starter files for first-run setup"
        ),
        "setup init help snapshot missing first-run description"
    );
    for phrase in [
        "Preserve canonical files by default",
        "--rewrite",
        "--reset-state",
    ] {
        assert!(
            setup_refresh_help_text.contains(phrase),
            "setup refresh help snapshot missing phrase `{phrase}`"
        );
    }

    let generate_required_phrases = ["execution.demo.packet"];
    for phrase in generate_required_phrases {
        assert!(
            docs_text.contains(phrase),
            "docs missing generate-help phrase `{phrase}`"
        );
        assert!(
            generate_help_text.contains(phrase),
            "generate help snapshot missing phrase `{phrase}`"
        );
    }

    let inspect_required_phrases = [
        "execution.demo.packet",
        "packet composition and decision evidence",
    ];
    for phrase in inspect_required_phrases {
        assert!(
            docs_text.contains(phrase),
            "docs missing inspect-help phrase `{phrase}`"
        );
        assert!(
            inspect_help_text.contains(phrase),
            "inspect help snapshot missing phrase `{phrase}`"
        );
    }

    let pipeline_required_phrases = [
        "`pipeline`",
        "pipeline resolve",
        "pipeline compile",
        "pipeline capture",
        "pipeline capture --preview",
        "pipeline capture apply --capture-id",
        "pipeline state set",
        "pipeline compile --explain",
        "payload-only stdout",
        "proof-only stdout",
        "re-run `pipeline resolve`",
    ];
    assert!(
        top_help_text.contains("pipeline"),
        "top-level help snapshot missing pipeline entry"
    );
    for phrase in pipeline_required_phrases {
        assert!(
            docs_text.contains(phrase),
            "docs missing pipeline phrase `{phrase}`"
        );
    }

    assert!(
        pipeline_help_text.contains("compile"),
        "pipeline help snapshot missing compile entry"
    );
    assert!(
        pipeline_help_text.contains("capture"),
        "pipeline help snapshot missing capture entry"
    );
    for phrase in ["--stage", "--explain"] {
        assert!(
            compile_help_text.contains(phrase),
            "compile help snapshot missing phrase `{phrase}`"
        );
    }
    for phrase in ["--id", "--stage", "--preview"] {
        assert!(
            capture_help_text.contains(phrase),
            "capture help snapshot missing phrase `{phrase}`"
        );
    }
    for phrase in [
        "stage.04_charter_inputs",
        "stage.06_project_context_interview",
        "stage.10_feature_spec",
    ] {
        assert!(
            docs_text.contains(phrase),
            "docs missing capture target phrase `{phrase}`"
        );
        assert!(
            capture_help_text.contains(phrase),
            "capture help snapshot missing capture target phrase `{phrase}`"
        );
    }
    for phrase in ["--capture-id", "Deterministic capture id"] {
        assert!(
            capture_apply_help_text.contains(phrase),
            "capture apply help snapshot missing phrase `{phrase}`"
        );
    }
    assert!(
        !inspect_help_text.contains("compile"),
        "inspect help snapshot must remain packet-proof only"
    );
    assert!(
        !inspect_help_text.contains("capture"),
        "inspect help snapshot must remain packet-proof only"
    );
}

fn assert_doc_matches_m4_stage_10_boundary(
    path: &std::path::Path,
    required_phrases: &[&str],
    banned_phrases: &[&str],
) {
    let text =
        fs::read_to_string(path).unwrap_or_else(|err| panic!("read {}: {}", path.display(), err));

    for phrase in required_phrases {
        assert!(
            text.contains(phrase),
            "{} missing M4 stage-10 boundary phrase `{}`",
            path.display(),
            phrase
        );
    }

    for phrase in banned_phrases {
        assert!(
            !text.contains(phrase),
            "{} must not reintroduce invalid M4 stage-10 wording `{}`",
            path.display(),
            phrase
        );
    }
}

#[test]
fn m4_stage_10_boundary_docs_remain_truthful() {
    let root = workspace_root();

    assert_doc_matches_m4_stage_10_boundary(
        &root.join("docs/CLI_OPERATOR_JOURNEY.md"),
        &[
            "compile -> external model output -> capture",
            "payload-only",
            "completed `FEATURE_SPEC.md`",
            "raw `pipeline compile` payload is refused as `invalid_capture_input`",
        ],
        &[
            "compile-to-capture handoff",
            "compile | capture",
            "payload stdout piped into capture",
        ],
    );
    assert_doc_matches_m4_stage_10_boundary(
        &root.join("docs/contracts/pipeline-proof-corpus-and-docs-cutover.md"),
        &[
            "compile -> external model output -> capture",
            "payload-only stdout that becomes model input for an external operator or model runner",
            "materializing the completed `FEATURE_SPEC.md` body",
            "raw `pipeline compile` payload is refused as `invalid_capture_input`",
            "MUST NOT imply a direct compile write mode or direct raw `compile | capture` piping as the valid stage-10 path",
        ],
        &["compile-to-capture handoff", "payload stdout piped into capture"],
    );
    assert_doc_matches_m4_stage_10_boundary(
        &root.join("docs/contracts/C-02-rust-workspace-and-cli-command-surface.md"),
        &[
            "emits model input payload",
            "completed `FEATURE_SPEC.md`",
            "refuses raw `pipeline compile` payload as `invalid_capture_input`",
            "materializes that completed body",
        ],
        &[
            "compile-to-capture handoff",
            "compile | capture",
            "payload stdout piped into capture",
        ],
    );
}

#[test]
fn m1_activation_contract_docs_remain_boolean_only() {
    let root = workspace_root();
    let plan_path = root.join("PLAN.md");
    let todos_path = root.join("TODOS.md");
    let legacy_model_path = root.join("docs/legacy/SYSTEM_MODEL.md");
    let contract_path = root.join("docs/contracts/pipeline-route-and-state-core.md");

    let plan_text = fs::read_to_string(&plan_path)
        .unwrap_or_else(|err| panic!("read {}: {}", plan_path.display(), err));
    let todos_text = fs::read_to_string(&todos_path)
        .unwrap_or_else(|err| panic!("read {}: {}", todos_path.display(), err));
    let system_model_text = fs::read_to_string(&legacy_model_path)
        .unwrap_or_else(|err| panic!("read {}: {}", legacy_model_path.display(), err));
    let contract_text = fs::read_to_string(&contract_path)
        .unwrap_or_else(|err| panic!("read {}: {}", contract_path.display(), err));

    assert!(
        plan_text.contains("variables.<name> == true|false"),
        "PLAN must describe the boolean-only M1 activation clause shape"
    );
    assert!(
        !plan_text.contains("     - quoted strings\n     - numbers"),
        "PLAN must not advertise quoted-string or numeric activation support for shipped M1"
    );

    assert!(
        todos_text.contains("variable-path equality against boolean literals"),
        "TODOS must describe M1 activation as boolean-only"
    );
    assert!(
        todos_text.contains("future pipelines may eventually need string or numeric equality"),
        "TODOS must treat string and numeric activation as future work"
    );

    assert!(
        system_model_text.contains(
            "only boolean equality in the form `variables.<name> == true|false` is supported"
        ),
        "legacy system model must call out the boolean-only reduced-v1 boundary"
    );
    assert!(
        system_model_text.contains("legacy harness reference supported simple equality checks"),
        "legacy system model must preserve the legacy-harness note"
    );

    assert!(
        contract_text.contains("Activation values MUST be boolean literals only."),
        "route/state contract must remain authoritative for the boolean-only M1 activation subset"
    );
}

#[test]
fn cli_product_vocabulary_doc_locks_core_terms() {
    let root = workspace_root();
    let vocab_path = root.join("docs/CLI_PRODUCT_VOCABULARY.md");
    let vocab_text = fs::read_to_string(&vocab_path)
        .unwrap_or_else(|err| panic!("read {}: {}", vocab_path.display(), err));

    let required_phrases = [
        "planning packet generation",
        "canonical repo-local `.handbook/` inputs",
        "`inspect` is the packet proof surface",
        "`doctor` is the recovery and baseline-readiness surface",
        "`setup` is the durable setup term",
        "`setup init` is the concrete first-run subcommand",
        "`setup refresh` preserves canonical files by default",
        "scaffolding only",
        "`author` is the canonical authoring surface",
        "`handbook author project-context`",
        "`handbook author environment-inventory`",
        "`pipeline compile --explain`",
        "next safe action",
        "bootstrap",
        "hydrate",
    ];

    for phrase in required_phrases {
        assert!(
            vocab_text.contains(phrase),
            "CLI vocabulary doc missing phrase `{phrase}`"
        );
    }
}

#[test]
fn cli_command_hierarchy_doc_locks_front_door_rules() {
    let root = workspace_root();
    let hierarchy_path = root.join("docs/CLI_COMMAND_HIERARCHY.md");
    let hierarchy_text = fs::read_to_string(&hierarchy_path)
        .unwrap_or_else(|err| panic!("read {}: {}", hierarchy_path.display(), err));

    let required_phrases = [
        "The front door is the `setup` family.",
        "The stable operation name remains `setup`.",
        "Bare `handbook setup` routes to `setup init` when canonical `.handbook/` truth is absent or invalid; otherwise it routes to `setup refresh`.",
        "`setup refresh` preserves canonical files by default.",
        "The shipped starter templates are scaffolding only.",
        "`doctor` is the recovery and baseline-readiness surface",
        "The shipped baseline authoring surface includes `handbook author charter`, `handbook author project-context`, and `handbook author environment-inventory`.",
        "`generate` is the default ready-path command.",
        "`pipeline compile --id <pipeline-id> --stage <stage-id>`",
        "Commands anchor to the enclosing git root when one exists.",
        "A nested git repo boundary wins over a parent managed repo.",
        "`doctor` is the recovery and baseline-readiness command",
    ];

    for phrase in required_phrases {
        assert!(
            hierarchy_text.contains(phrase),
            "CLI command hierarchy doc missing phrase `{phrase}`"
        );
    }
}

#[test]
fn cli_tone_rules_doc_locks_core_tone() {
    let root = workspace_root();
    let tone_path = root.join("docs/CLI_TONE_RULES.md");
    let tone_text = fs::read_to_string(&tone_path)
        .unwrap_or_else(|err| panic!("read {}: {}", tone_path.display(), err));

    let required_phrases = [
        "The default tone is **strict but guided**.",
        "Do not use:",
        "something went wrong",
        "unable to process request",
        "Do not celebrate normal success.",
        "Plain `pipeline compile` success should stay payload-only.",
        "`pipeline compile --explain` should stay proof-only.",
        "Refusal output should:",
        "Proof output should:",
        "Recovery output should:",
    ];

    for phrase in required_phrases {
        assert!(
            tone_text.contains(phrase),
            "CLI tone rules doc missing phrase `{phrase}`"
        );
    }
}

#[test]
fn cli_output_anatomy_doc_locks_section_order_rules() {
    let root = workspace_root();
    let anatomy_path = root.join("docs/CLI_OUTPUT_ANATOMY.md");
    let anatomy_text = fs::read_to_string(&anatomy_path)
        .unwrap_or_else(|err| panic!("read {}: {}", anatomy_path.display(), err));

    let required_phrases = [
        "`generate` and `inspect` start with the same three-line trust header:",
        "## `generate` Anatomy",
        "## `inspect` Anatomy",
        "## `pipeline compile` Anatomy",
        "plain `pipeline compile` success is payload-only stdout",
        "`pipeline compile --explain` success is proof-only stdout",
        "## `doctor` Anatomy",
        "`setup` is a special M6 case: the setup family (`setup`, `setup init`, `setup refresh`) uses setup-family anatomy rather than packet anatomy.",
        "`OBJECT: setup init` or `OBJECT: setup refresh`",
        "exact author command",
        "bare `handbook setup` must reveal which routed subcommand it selected",
        "the shipped starter templates are scaffolding only",
        "setup success must point to `handbook doctor` for baseline classification rather than guessing which authoring command comes next",
        "`setup refresh` preserves canonical files by default",
        "## Presentation Failure And Parse-Validation Output",
    ];

    for phrase in required_phrases {
        assert!(
            anatomy_text.contains(phrase),
            "CLI output anatomy doc missing phrase `{phrase}`"
        );
    }

    let doctor_section = markdown_h2_section(&anatomy_text, "## `doctor` Anatomy");
    let required_doctor_phrases = [
        "`SCAFFOLDED` or `PARTIAL_BASELINE` or `INVALID_BASELINE` or `BASELINE_COMPLETE`",
        "`ROOT STATUS: <status>`",
        "`NEXT SAFE ACTION: <exact recovery action>` or `NEXT SAFE ACTION: <none>`",
        "`## BASELINE CHECKLIST`",
    ];

    for phrase in required_doctor_phrases {
        assert!(
            doctor_section.contains(phrase),
            "doctor anatomy section missing phrase `{phrase}`"
        );
    }

    let forbidden_doctor_phrases = [
        "OBJECT: baseline readiness",
        "## CHECKLIST",
        "## BLOCKERS",
        "## GUIDANCE",
    ];

    for phrase in forbidden_doctor_phrases {
        assert!(
            !doctor_section.contains(phrase),
            "doctor anatomy section must not contain `{phrase}`"
        );
    }
}

#[test]
fn root_readme_locks_setup_story_individually() {
    let root = workspace_root();
    let readme_path = root.join("README.md");
    let readme_text = fs::read_to_string(&readme_path)
        .unwrap_or_else(|err| panic!("read {}: {}", readme_path.display(), err));

    let required_phrases = [
        "The shipped starter templates are scaffolding only.",
        "Setup hands off to `handbook doctor`, which renders the ordered baseline checklist and the next exact authoring command.",
        "This repository does not ship completed canonical `.handbook/` truth at repo root.",
    ];

    for phrase in required_phrases {
        assert!(
            readme_text.contains(phrase),
            "README.md missing phrase `{phrase}`"
        );
    }
}

#[test]
fn design_doc_locks_cli_interaction_contract() {
    let root = workspace_root();
    let design_path = root.join("DESIGN.md");
    let design_text = fs::read_to_string(&design_path)
        .unwrap_or_else(|err| panic!("read {}: {}", design_path.display(), err));

    let required_phrases = [
        "This file is the canonical interaction contract for the reduced-v1 CLI product.",
        "the packet is the product",
        "`doctor` is the only canonical recovery verb",
        "the public setup family is `handbook setup`, `handbook setup init`, and `handbook setup refresh`",
        "bare `handbook setup` routes to `setup init` when canonical `.handbook/` truth is absent or invalid; otherwise it routes to `setup refresh`",
        "`setup` should stay one durable family name even when it routes between `setup init` and `setup refresh`",
        "setup hands off to `handbook doctor`, which renders baseline readiness and the next exact authoring action",
        "this repository does not ship completed canonical `.handbook/` truth at repo root; a fresh clone starts with `handbook setup`",
        "the baseline authoring family is `handbook author charter`, `handbook author project-context`, and `handbook author environment-inventory`",
        "`inspect` currently emits a self-referential ready-path next action",
        "update the relevant D1-D4 source document",
    ];

    for phrase in required_phrases {
        assert!(
            design_text.contains(phrase),
            "DESIGN.md missing phrase `{phrase}`"
        );
    }
}

#[test]
fn cli_operator_journey_doc_locks_revision_findings() {
    let root = workspace_root();
    let journey_path = root.join("docs/CLI_OPERATOR_JOURNEY.md");
    let journey_text = fs::read_to_string(&journey_path)
        .unwrap_or_else(|err| panic!("read {}: {}", journey_path.display(), err));

    let required_phrases = [
        "This M4/M5 journey now sits downstream of the M6 setup family.",
        "`handbook setup` is the durable front door.",
        "Bare `handbook setup` routes to `setup init` when canonical `.handbook/` truth is absent or invalid; otherwise it routes to `setup refresh`.",
        "The shipped starter templates are scaffolding only.",
        "`doctor` is the baseline-readiness surface and reports `SCAFFOLDED`, `PARTIAL_BASELINE`, `INVALID_BASELINE`, or `BASELINE_COMPLETE` before packet work continues.",
        "Does the shipped reduced-v1 product actually produce the confidence -> momentum -> controlled caution arc",
        "The command is functionally correct and productically wrong.",
        "## Revision Backlog",
        "R1, Align `doctor` to the interaction contract",
        "R2, Fix `inspect` ready-path next-action semantics",
    ];

    for phrase in required_phrases {
        assert!(
            journey_text.contains(phrase),
            "CLI operator journey doc missing phrase `{phrase}`"
        );
    }
}

#[test]
fn setup_family_contract_docs_lock_m6_story() {
    let root = workspace_root();

    let approved_surface_path = root.join("docs/contracts/C-01-approved-repo-surface.md");
    let approved_surface = fs::read_to_string(&approved_surface_path)
        .unwrap_or_else(|err| panic!("read {}: {}", approved_surface_path.display(), err));
    for phrase in [
        "The supported setup story is the Rust CLI setup family: `handbook setup`, `handbook setup init`, and `handbook setup refresh`.",
        "Bare `handbook setup` is the durable front door and routes to `setup init` when canonical `.handbook/` truth is absent or invalid; otherwise it routes to `setup refresh`.",
        "The shipped setup starter templates are scaffolding only.",
        "Historical guided-setup or legacy-harness wording MAY remain only as explicit historical reference material. It MUST NOT read like active product authority.",
    ] {
        assert!(
            approved_surface.contains(phrase),
            "approved repo surface contract missing phrase `{phrase}`"
        );
    }

    let command_surface_path =
        root.join("docs/contracts/C-02-rust-workspace-and-cli-command-surface.md");
    let command_surface = fs::read_to_string(&command_surface_path)
        .unwrap_or_else(|err| panic!("read {}: {}", command_surface_path.display(), err));
    for phrase in [
        "Help text and docs MUST make clear that the public setup family is `handbook setup`, `handbook setup init`, and `handbook setup refresh`.",
        "Bare `handbook setup` MUST route to `setup init` when canonical `.handbook/` truth is absent or invalid; otherwise it MUST route to `setup refresh`.",
        "`setup refresh` MUST preserve canonical files by default.",
        "The shipped setup starter templates MUST be treated as scaffolding only.",
        "`doctor` MUST classify baseline readiness using exactly:",
        "The shipped baseline authoring surface MUST expose exactly:",
    ] {
        assert!(
            command_surface.contains(phrase),
            "CLI command surface contract missing phrase `{phrase}`"
        );
    }

    let blocker_contract_path =
        root.join("docs/contracts/C-04-resolver-result-and-doctor-blockers.md");
    let blocker_contract = fs::read_to_string(&blocker_contract_path)
        .unwrap_or_else(|err| panic!("read {}: {}", blocker_contract_path.display(), err));
    let blocker_phrase =
        "Renderer-facing wording for missing-root and invalid-root blockers SHOULD route the operator toward the setup family.";
    assert!(
        blocker_contract.contains(blocker_phrase),
        "resolver blocker contract missing setup-family next-safe-action guidance"
    );
    let charter_blocker_phrase =
        "Renderer-facing wording for artifact-specific blockers SHOULD route the operator toward the exact `handbook author ...` command for that artifact.";
    assert!(
        blocker_contract.contains(charter_blocker_phrase),
        "resolver blocker contract missing author-surface next-safe-action guidance"
    );
}

fn workspace_root() -> PathBuf {
    let start = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    for ancestor in start.ancestors() {
        let cargo_toml = ancestor.join("Cargo.toml");
        if !cargo_toml.is_file() {
            continue;
        }

        let Ok(contents) = fs::read_to_string(&cargo_toml) else {
            continue;
        };
        if contents.contains("[workspace]") {
            return ancestor.to_path_buf();
        }
    }

    panic!(
        "failed to locate workspace root from CARGO_MANIFEST_DIR={}",
        env!("CARGO_MANIFEST_DIR")
    );
}
