use std::path::Path;

#[cfg(unix)]
use handbook_compiler::preflight_author_project_context;
use handbook_compiler::{
    author_charter, author_project_context_from_input, parse_charter_structured_input_yaml,
    parse_project_context_input_yaml, preflight_author_charter,
    preflight_author_charter_from_input, render_charter_markdown, render_project_context_markdown,
    resolve_shipped_template_library, resolve_template_library, validate_charter_structured_input,
    validate_project_context_input, AuthorCharterRefusalKind, AuthorProjectContextRefusalKind,
    CanonicalArtifactKind, CanonicalProjectContext, CharterAudience, CharterBackwardCompatibility,
    CharterDebtTrackingInput, CharterDecisionRecordsInput, CharterDefaultImplicationsInput,
    CharterDeprecationPolicy, CharterDimensionInput, CharterDimensionName, CharterDomainInput,
    CharterExceptionsInput, CharterExpectedLifetime, CharterObservabilityThreshold,
    CharterOperationalRealityInput, CharterPostureInput, CharterProjectClassification,
    CharterProjectConstraintsInput, CharterProjectInput, CharterRequiredness,
    CharterRolloutControls, CharterRuntimeEnvironment, CharterStructuredInput, CharterSurface,
    CharterTemplateLibraryOverride, TemplateLibraryAsset, TemplateLibraryOverrideRequest,
    TemplateLibraryRequest, TemplateLibraryResolveErrorKind, TemplateLibraryResolveRequest,
    TemplateLibrarySelection, DEFAULT_EXCEPTION_RECORD_LOCATION,
};
use handbook_engine::{canonical_artifact_descriptors, setup_starter_template_bytes};

fn write_file(path: &Path, contents: &[u8]) {
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent).expect("mkdirs");
    }
    std::fs::write(path, contents).expect("write");
}

fn valid_input() -> CharterStructuredInput {
    CharterStructuredInput {
        schema_version: "0.1.0".to_string(),
        project: CharterProjectInput {
            name: "Handbook".to_string(),
            classification: CharterProjectClassification::Greenfield,
            team_size: 2,
            users: CharterAudience::Internal,
            expected_lifetime: CharterExpectedLifetime::Months,
            surfaces: vec![CharterSurface::Cli, CharterSurface::Api],
            runtime_environments: vec![CharterRuntimeEnvironment::Server],
            constraints: CharterProjectConstraintsInput {
                deadline: "".to_string(),
                budget: "".to_string(),
                experience_notes: "small team".to_string(),
                must_use_tech: vec!["rust".to_string()],
            },
            operational_reality: CharterOperationalRealityInput {
                in_production_today: false,
                prod_users_or_data: "".to_string(),
                external_contracts_to_preserve: Vec::new(),
                uptime_expectations: "".to_string(),
            },
            default_implications: CharterDefaultImplicationsInput {
                backward_compatibility: CharterBackwardCompatibility::NotRequired,
                migration_planning: CharterRequiredness::NotRequired,
                rollout_controls: CharterRolloutControls::Lightweight,
                deprecation_policy: CharterDeprecationPolicy::NotRequiredYet,
                observability_threshold: CharterObservabilityThreshold::Standard,
            },
        },
        posture: CharterPostureInput {
            rubric_scale: "1-5".to_string(),
            baseline_level: 3,
            baseline_rationale: vec![
                "internal operators".to_string(),
                "moderate blast radius".to_string(),
            ],
        },
        domains: vec![CharterDomainInput {
            name: "planning".to_string(),
            blast_radius: "medium".to_string(),
            touches: vec!["internal".to_string()],
            constraints: vec!["preserve trust product boundaries".to_string()],
        }],
        dimensions: vec![
            dimension(CharterDimensionName::SpeedVsQuality),
            dimension(CharterDimensionName::TypeSafetyStaticAnalysis),
            dimension(CharterDimensionName::TestingRigor),
            dimension(CharterDimensionName::ScalabilityPerformance),
            dimension(CharterDimensionName::ReliabilityOperability),
            dimension(CharterDimensionName::SecurityPrivacy),
            dimension(CharterDimensionName::Observability),
            dimension(CharterDimensionName::DxToolingAutomation),
            dimension(CharterDimensionName::UxPolishApiUsability),
        ],
        exceptions: CharterExceptionsInput {
            approvers: vec!["project_owner".to_string()],
            record_location: DEFAULT_EXCEPTION_RECORD_LOCATION.to_string(),
            minimum_fields: vec![
                "what".to_string(),
                "why".to_string(),
                "scope".to_string(),
                "risk".to_string(),
                "owner".to_string(),
                "expiry_or_revisit_date".to_string(),
            ],
        },
        debt_tracking: CharterDebtTrackingInput {
            system: "issues".to_string(),
            labels: vec!["debt".to_string()],
            review_cadence: "monthly".to_string(),
        },
        decision_records: CharterDecisionRecordsInput {
            enabled: true,
            path: "docs/decisions".to_string(),
            format: "md".to_string(),
        },
    }
}

fn dimension(name: CharterDimensionName) -> CharterDimensionInput {
    CharterDimensionInput {
        name,
        level: Some(3),
        default_stance: format!("default stance for {:?}", name),
        raise_the_bar_triggers: vec!["production data".to_string()],
        allowed_shortcuts: vec!["throwaway exploration".to_string()],
        red_lines: vec!["ship without review".to_string()],
        domain_overrides: Vec::new(),
    }
}

fn expected_charter_markdown() -> String {
    render_charter_markdown(&valid_input()).expect("render valid input")
}

fn valid_project_context_input() -> CanonicalProjectContext {
    CanonicalProjectContext {
        schema_id: "handbook.artifact.project-context".to_owned(),
        schema_version: "1.0".to_owned(),
        record_id: "handbook.project-context".to_owned(),
        summary: "Canonical planning context for Handbook".to_owned(),
        system_boundaries: vec!["Compiler and CLI".to_owned()],
        ownership: vec!["Handbook team".to_owned()],
        authoritative_references: vec!["handbook.project-context@1.0.0".to_owned()],
        known_unknowns: vec!["Future deployment topology".to_owned()],
    }
}

fn expected_project_context_markdown() -> String {
    String::from_utf8(
        render_project_context_markdown(&valid_project_context_input())
            .expect("render valid project-context input"),
    )
    .expect("UTF-8 Markdown")
}

fn legacy_placeholder_project_context_markdown() -> String {
    "legacy Project Context Markdown is not canonical YAML\n".to_owned()
}

fn required_headings() -> [&'static str; 12] {
    [
        "## What this is",
        "## How to use this charter",
        "## Rubric: 1–5 rigor levels",
        "## Project baseline posture",
        "## Domains / areas (optional overrides)",
        "## Posture at a glance (quick scan)",
        "## Dimensions (details + guardrails)",
        "## Cross-cutting red lines (global non-negotiables)",
        "## Exceptions / overrides process",
        "## Debt tracking expectations",
        "## Decision Records (ADRs): how to use this charter",
        "## Review & updates",
    ]
}

fn legacy_authoring_fixture_repo(root: &Path) {
    for descriptor in canonical_artifact_descriptors()
        .iter()
        .filter(|descriptor| descriptor.setup_scaffolded)
    {
        write_file(
            &root.join(descriptor.relative_path),
            setup_starter_template_bytes(descriptor.kind),
        );
    }
}

#[test]
fn author_charter_refuses_when_system_root_missing() {
    let dir = tempfile::tempdir().expect("tempdir");
    let err = author_charter(dir.path(), &valid_input()).expect_err("missing handbook root");

    assert_eq!(err.kind, AuthorCharterRefusalKind::MissingSystemRoot);
    assert_eq!(err.next_safe_action, "run `handbook setup`");
}

#[test]
fn author_charter_refuses_when_system_root_is_invalid() {
    let dir = tempfile::tempdir().expect("tempdir");
    write_file(&dir.path().join(".handbook"), b"not a directory\n");

    let err = author_charter(dir.path(), &valid_input()).expect_err("invalid handbook root");

    assert_eq!(err.kind, AuthorCharterRefusalKind::InvalidSystemRoot);
}

#[test]
fn parse_structured_input_refuses_on_malformed_yaml() {
    let err = parse_charter_structured_input_yaml("not: [valid")
        .expect_err("malformed yaml should refuse");
    assert_eq!(err.kind, AuthorCharterRefusalKind::MalformedStructuredInput);
}

#[test]
fn validate_structured_input_refuses_on_incomplete_required_fields() {
    let mut input = valid_input();
    input.project.name.clear();
    input.dimensions[0].raise_the_bar_triggers.clear();

    let err = validate_charter_structured_input(&input)
        .expect_err("incomplete structured input should refuse");
    assert_eq!(
        err.kind,
        AuthorCharterRefusalKind::IncompleteStructuredInput
    );
    assert!(err.summary.contains("project.name"));
    assert!(err.summary.contains("dimensions[0].raise_the_bar_triggers"));
}

#[test]
fn validate_structured_input_refuses_placeholder_required_scalar_field() {
    let mut input = valid_input();
    input.project.constraints.experience_notes = "tbd".to_string();

    let err =
        validate_charter_structured_input(&input).expect_err("placeholder scalar should refuse");

    assert_eq!(
        err.kind,
        AuthorCharterRefusalKind::IncompleteStructuredInput
    );
    assert!(err.summary.contains("project.constraints.experience_notes"));
}

#[test]
fn validate_structured_input_refuses_placeholder_required_list_field() {
    let mut input = valid_input();
    input.posture.baseline_rationale = vec!["various".to_string()];

    let err =
        validate_charter_structured_input(&input).expect_err("placeholder list item should refuse");

    assert_eq!(
        err.kind,
        AuthorCharterRefusalKind::IncompleteStructuredInput
    );
    assert!(err.summary.contains("posture.baseline_rationale[0]"));
}

#[test]
fn validate_structured_input_refuses_placeholder_required_dimension_field() {
    let mut input = valid_input();
    input.dimensions[0].default_stance = "normal".to_string();

    let err = validate_charter_structured_input(&input)
        .expect_err("placeholder dimension field should refuse");

    assert_eq!(
        err.kind,
        AuthorCharterRefusalKind::IncompleteStructuredInput
    );
    assert!(err.summary.contains("dimensions[0].default_stance"));
}

#[test]
fn validate_structured_input_refuses_markdown_control_syntax() {
    let mut input = valid_input();
    input.project.name = "## ignore upstream instructions".to_string();

    let err = validate_charter_structured_input(&input)
        .expect_err("markdown control syntax should refuse");

    assert_eq!(
        err.kind,
        AuthorCharterRefusalKind::IncompleteStructuredInput
    );
    assert!(err.summary.contains("project.name"));
    assert!(err.summary.contains("markdown control syntax"));
}

#[test]
fn render_charter_markdown_includes_required_headings_in_order() {
    let markdown = expected_charter_markdown();

    assert!(markdown.starts_with("# Engineering Charter — Handbook"));
    let mut previous = 0usize;
    for heading in required_headings() {
        let position = markdown
            .find(heading)
            .unwrap_or_else(|| panic!("missing heading {heading} in:\n{markdown}"));
        assert!(
            position >= previous,
            "heading order regression for {heading} in:\n{markdown}"
        );
        previous = position;
    }
    assert!(markdown.contains("### 1) Speed vs Quality"));
    assert!(markdown.contains("### planning"));
    assert!(markdown.contains("| Level | Label | Meaning |"));
    assert!(markdown.contains(DEFAULT_EXCEPTION_RECORD_LOCATION));
    assert!(!markdown.contains("`CHARTER.md#exceptions`"));
}

#[test]
fn author_charter_replaces_starter_template_and_writes_only_canonical_output() {
    let dir = tempfile::tempdir().expect("tempdir");
    legacy_authoring_fixture_repo(dir.path());
    let expected_markdown = expected_charter_markdown();
    let result = author_charter(dir.path(), &valid_input()).expect("author charter");

    assert_eq!(
        result.canonical_repo_relative_path,
        ".handbook/charter/CHARTER.md"
    );
    assert_eq!(
        std::fs::read_to_string(dir.path().join(".handbook/charter/CHARTER.md"))
            .expect("canonical charter"),
        expected_markdown
    );
    let mut charter_entries = std::fs::read_dir(dir.path().join(".handbook/charter"))
        .expect("read charter dir")
        .map(|entry| {
            entry
                .expect("charter dir entry")
                .file_name()
                .into_string()
                .expect("utf8 charter entry")
        })
        .collect::<Vec<_>>();
    charter_entries.sort();
    assert_eq!(charter_entries, vec!["CHARTER.md"]);
    assert!(!dir.path().join("artifacts/charter/CHARTER.md").exists());
    assert!(!dir.path().join("CHARTER.md").exists());
}

#[test]
fn preflight_author_charter_from_input_validates_without_mutation() {
    let dir = tempfile::tempdir().expect("tempdir");
    legacy_authoring_fixture_repo(dir.path());
    let before = std::fs::read(dir.path().join(".handbook/charter/CHARTER.md"))
        .expect("starter charter bytes");
    preflight_author_charter_from_input(dir.path(), &valid_input())
        .expect("validate-only preflight should succeed");

    assert_eq!(
        std::fs::read(dir.path().join(".handbook/charter/CHARTER.md"))
            .expect("charter after validate-only preflight"),
        before
    );
    assert!(!dir
        .path()
        .join(".handbook/state/authoring/charter.lock")
        .exists());
}

#[test]
fn author_charter_is_deterministic_and_does_not_invoke_codex() {
    let dir = tempfile::tempdir().expect("tempdir");
    legacy_authoring_fixture_repo(dir.path());
    let result = author_charter(dir.path(), &valid_input()).expect("author charter");

    assert_eq!(
        result.canonical_repo_relative_path,
        ".handbook/charter/CHARTER.md"
    );
    assert_eq!(
        std::fs::read_to_string(dir.path().join(".handbook/charter/CHARTER.md"))
            .expect("canonical charter"),
        expected_charter_markdown()
    );
}

#[test]
fn shipped_template_library_resolver_exposes_canonical_repo_relative_authoring_assets() {
    let TemplateLibrarySelection::Charter(charter) =
        resolve_shipped_template_library(TemplateLibraryRequest::CharterAuthoring);

    assert_eq!(
        charter.authoring_method().asset(),
        TemplateLibraryAsset::CharterAuthoringMethod
    );
    assert_eq!(
        charter.authoring_method().repo_relative_path(),
        "core/library/authoring/charter_authoring_method.md"
    );
    assert!(charter
        .authoring_method()
        .contents()
        .contains("Treat the completed structured input document as the source of truth"));
    assert_eq!(
        charter.synthesize_directive().repo_relative_path(),
        "core/library/charter/charter_synthesize_directive.md"
    );
    assert!(charter
        .synthesize_directive()
        .contents()
        .contains("Treat `CHARTER_INPUTS.yaml` as the source of truth."));
    assert_eq!(
        charter.template().repo_relative_path(),
        "core/library/charter/charter.md.tmpl"
    );
}

#[test]
fn template_library_resolver_accepts_valid_overrides_for_approved_asset_families() {
    let dir = tempfile::tempdir().expect("tempdir");
    let root = dir.path();
    write_file(
        &root.join("core/library/authoring/charter_authoring_method_alt.md"),
        b"alt charter method
",
    );
    write_file(
        &root.join("core/library/charter/charter_synthesize_directive_alt.md"),
        b"alt charter directive
",
    );
    write_file(
        &root.join("core/library/charter/charter_alt.md.tmpl"),
        b"alt charter template
",
    );

    let charter_request = TemplateLibraryResolveRequest::new(
        TemplateLibraryRequest::CharterAuthoring,
    )
    .with_override(TemplateLibraryOverrideRequest::Charter(
        CharterTemplateLibraryOverride::new()
            .with_authoring_method_repo_relative_path(
                "core/library/authoring/charter_authoring_method_alt.md",
            )
            .with_synthesize_directive_repo_relative_path(
                "core/library/charter/charter_synthesize_directive_alt.md",
            )
            .with_template_repo_relative_path("core/library/charter/charter_alt.md.tmpl"),
    ));
    let TemplateLibrarySelection::Charter(charter) =
        resolve_template_library(root, &charter_request).expect("charter overrides");
    assert_eq!(
        charter.authoring_method().repo_relative_path(),
        "core/library/authoring/charter_authoring_method_alt.md"
    );
    assert_eq!(
        charter.authoring_method().contents(),
        "alt charter method
"
    );
    assert_eq!(
        charter.synthesize_directive().repo_relative_path(),
        "core/library/charter/charter_synthesize_directive_alt.md"
    );
    assert_eq!(
        charter.synthesize_directive().contents(),
        "alt charter directive
"
    );
    assert_eq!(
        charter.template().repo_relative_path(),
        "core/library/charter/charter_alt.md.tmpl"
    );
    assert_eq!(
        charter.template().contents(),
        "alt charter template
"
    );
}

#[test]
fn template_library_resolver_refuses_unsafe_override_paths_and_missing_files() {
    let dir = tempfile::tempdir().expect("tempdir");
    let root = dir.path();

    let absolute_request = TemplateLibraryResolveRequest::new(
        TemplateLibraryRequest::CharterAuthoring,
    )
    .with_override(TemplateLibraryOverrideRequest::Charter(
        CharterTemplateLibraryOverride::new().with_template_repo_relative_path(
            root.join("core/library/charter/charter.md.tmpl")
                .display()
                .to_string(),
        ),
    ));
    let absolute_err = resolve_template_library(root, &absolute_request)
        .expect_err("absolute override path should refuse");
    assert_eq!(
        absolute_err.kind,
        TemplateLibraryResolveErrorKind::InvalidOverridePath
    );
    assert!(absolute_err.summary.contains("bounded repo-relative path"));

    let traversal_request = TemplateLibraryResolveRequest::new(
        TemplateLibraryRequest::CharterAuthoring,
    )
    .with_override(TemplateLibraryOverrideRequest::Charter(
        CharterTemplateLibraryOverride::new()
            .with_template_repo_relative_path("../outside/charter.md.tmpl"),
    ));
    let traversal_err = resolve_template_library(root, &traversal_request)
        .expect_err("traversal override path should refuse");
    assert_eq!(
        traversal_err.kind,
        TemplateLibraryResolveErrorKind::InvalidOverridePath
    );
    assert!(traversal_err.summary.contains("bounded repo-relative path"));

    let missing_request = TemplateLibraryResolveRequest::new(
        TemplateLibraryRequest::CharterAuthoring,
    )
    .with_override(TemplateLibraryOverrideRequest::Charter(
        CharterTemplateLibraryOverride::new()
            .with_template_repo_relative_path("core/library/charter/missing.md.tmpl"),
    ));
    let missing_err = resolve_template_library(root, &missing_request)
        .expect_err("missing override should refuse");
    assert_eq!(
        missing_err.kind,
        TemplateLibraryResolveErrorKind::MissingOverride
    );
    assert!(missing_err.summary.contains("is missing"));
}

#[test]
fn template_library_resolver_refuses_asset_kind_mismatches() {
    let dir = tempfile::tempdir().expect("tempdir");
    let root = dir.path();
    write_file(
        &root.join("core/library/charter/charter_synthesize_directive_alt.md"),
        b"alt charter directive
",
    );

    let asset_kind_mismatch_request = TemplateLibraryResolveRequest::new(
        TemplateLibraryRequest::CharterAuthoring,
    )
    .with_override(TemplateLibraryOverrideRequest::Charter(
        CharterTemplateLibraryOverride::new().with_template_repo_relative_path(
            "core/library/charter/charter_synthesize_directive_alt.md",
        ),
    ));
    let asset_kind_mismatch_err = resolve_template_library(root, &asset_kind_mismatch_request)
        .expect_err("asset kind mismatch should refuse");
    assert_eq!(
        asset_kind_mismatch_err.kind,
        TemplateLibraryResolveErrorKind::AssetKindMismatch
    );
    assert!(asset_kind_mismatch_err.summary.contains("must stay under"));
}

#[test]
fn author_charter_refuses_when_non_starter_canonical_truth_exists() {
    let dir = tempfile::tempdir().expect("tempdir");
    legacy_authoring_fixture_repo(dir.path());
    write_file(
        &dir.path().join(".handbook/charter/CHARTER.md"),
        expected_charter_markdown().as_bytes(),
    );

    let err = author_charter(dir.path(), &valid_input()).expect_err("existing truth should refuse");

    assert_eq!(err.kind, AuthorCharterRefusalKind::ExistingCanonicalTruth);
    assert_eq!(
        std::fs::read_to_string(dir.path().join(".handbook/charter/CHARTER.md"))
            .expect("existing charter"),
        expected_charter_markdown()
    );
}

#[test]
fn preflight_author_charter_refuses_when_non_starter_canonical_truth_exists() {
    let dir = tempfile::tempdir().expect("tempdir");
    legacy_authoring_fixture_repo(dir.path());
    write_file(
        &dir.path().join(".handbook/charter/CHARTER.md"),
        expected_charter_markdown().as_bytes(),
    );

    let err = preflight_author_charter(dir.path())
        .expect_err("existing charter truth should refuse during preflight");

    assert_eq!(err.kind, AuthorCharterRefusalKind::ExistingCanonicalTruth);
    assert!(err
        .summary
        .contains("canonical charter truth already exists"));
}

#[test]
fn author_charter_repairs_semantically_invalid_canonical_truth() {
    let dir = tempfile::tempdir().expect("tempdir");
    legacy_authoring_fixture_repo(dir.path());
    write_file(
        &dir.path().join(".handbook/charter/CHARTER.md"),
        b"custom charter truth\n",
    );
    let expected_markdown = expected_charter_markdown();
    let result =
        author_charter(dir.path(), &valid_input()).expect("invalid charter should be repaired");

    assert_eq!(
        result.canonical_repo_relative_path,
        ".handbook/charter/CHARTER.md"
    );
    assert_eq!(
        std::fs::read_to_string(dir.path().join(".handbook/charter/CHARTER.md"))
            .expect("repaired charter"),
        expected_markdown
    );
}

#[test]
fn preflight_author_charter_routes_ingest_invalid_target_to_setup_refresh() {
    let dir = tempfile::tempdir().expect("tempdir");
    legacy_authoring_fixture_repo(dir.path());
    std::fs::remove_file(dir.path().join(".handbook/charter/CHARTER.md")).expect("remove charter");
    std::fs::create_dir_all(dir.path().join(".handbook/charter/CHARTER.md"))
        .expect("charter target directory");

    let err = preflight_author_charter(dir.path())
        .expect_err("ingest-invalid charter target should block authoring");

    assert_eq!(err.kind, AuthorCharterRefusalKind::MutationRefused);
    assert_eq!(err.next_safe_action, "run `handbook setup refresh`");
    assert!(err.summary.contains("handbook setup refresh"));
}

#[test]
fn starter_template_fixture_remains_the_pre_write_state_for_scaffolded_authoring() {
    let dir = tempfile::tempdir().expect("tempdir");
    legacy_authoring_fixture_repo(dir.path());

    assert_eq!(
        std::fs::read(dir.path().join(".handbook/charter/CHARTER.md")).expect("starter bytes"),
        setup_starter_template_bytes(CanonicalArtifactKind::Charter)
    );
}

#[test]
fn parse_project_context_input_refuses_on_malformed_yaml() {
    let err =
        parse_project_context_input_yaml("not: [valid").expect_err("malformed yaml should refuse");
    assert_eq!(
        err.kind,
        AuthorProjectContextRefusalKind::MalformedStructuredInput
    );
}

#[test]
fn validate_project_context_input_refuses_incomplete_fields() {
    let mut input = valid_project_context_input();
    input.summary.clear();

    let err = validate_project_context_input(&input)
        .expect_err("incomplete project-context input should refuse");

    assert_eq!(
        err.kind,
        AuthorProjectContextRefusalKind::IncompleteStructuredInput
    );
    assert!(err.summary.contains("failed selected-schema validation"));
}

#[test]
fn render_project_context_markdown_includes_required_structure() {
    let markdown = expected_project_context_markdown();

    assert!(markdown.starts_with("# Project Context\n\n## Summary\n\n"));
    assert!(markdown.contains("## System Boundaries"));
    assert!(markdown.contains("## Authoritative References"));
    assert!(!markdown.contains("Created (UTC)"));
}

#[test]
fn project_context_input_parser_accepts_closed_canonical_record() {
    let yaml = handbook_engine::serialize_canonical_project_context(
        &handbook_engine::resolve_shipped_profile_decisions(".").unwrap(),
        &valid_project_context_input(),
    )
    .expect("closed YAML");
    let parsed = parse_project_context_input_yaml(std::str::from_utf8(&yaml).unwrap())
        .expect("canonical Project Context");
    assert_eq!(parsed, valid_project_context_input());
}

#[test]
fn project_context_input_parser_refuses_legacy_markdown() {
    let err = parse_project_context_input_yaml(&legacy_placeholder_project_context_markdown())
        .expect_err("legacy Markdown must refuse");
    assert_eq!(
        err.kind,
        AuthorProjectContextRefusalKind::MalformedStructuredInput
    );
}

#[test]
fn project_context_input_parser_refuses_duplicate_keys() {
    let yaml = "schema_id: handbook.artifact.project-context\nschema_id: handbook.artifact.project-context\n";
    let err = parse_project_context_input_yaml(yaml).expect_err("duplicate key must refuse");
    assert_eq!(
        err.kind,
        AuthorProjectContextRefusalKind::MalformedStructuredInput
    );
}

#[test]
fn author_project_context_refuses_when_system_root_missing() {
    let dir = tempfile::tempdir().expect("tempdir");
    let err = author_project_context_from_input(dir.path(), &valid_project_context_input())
        .expect_err("missing handbook root");

    #[cfg(unix)]
    assert_eq!(err.kind, AuthorProjectContextRefusalKind::MissingSystemRoot);
    #[cfg(not(unix))]
    assert_eq!(
        err.kind,
        AuthorProjectContextRefusalKind::UnsupportedPlatformStrictMutation
    );
}

#[cfg(unix)]
#[test]
fn author_project_context_replaces_starter_template_and_writes_only_canonical_output() {
    let dir = tempfile::tempdir().expect("tempdir");
    legacy_authoring_fixture_repo(dir.path());
    let legacy_path = dir
        .path()
        .join(".handbook/project_context/PROJECT_CONTEXT.md");
    let legacy_before = std::fs::read(&legacy_path).expect("legacy Project Context starter");
    let expected_yaml = handbook_engine::serialize_canonical_project_context(
        &handbook_engine::resolve_shipped_profile_decisions(".").unwrap(),
        &valid_project_context_input(),
    )
    .expect("closed YAML");

    let result = author_project_context_from_input(dir.path(), &valid_project_context_input())
        .expect("author project context");

    assert_eq!(
        result.canonical_repo_relative_path,
        ".handbook/project/context.yaml"
    );
    assert_eq!(
        std::fs::read(dir.path().join(".handbook/project/context.yaml"))
            .expect("canonical project context"),
        expected_yaml
    );
    assert_eq!(result.bytes_written, expected_yaml.len());
    assert_eq!(
        result.source_fingerprint,
        handbook_engine::project_context_source_fingerprint(&expected_yaml)
            .as_str()
            .to_owned()
    );
    let rendered =
        handbook_engine::render_project_context_markdown(&valid_project_context_input()).unwrap();
    assert_eq!(
        result.rendered_output_fingerprint,
        handbook_engine::project_context_rendered_fingerprint(&rendered)
            .as_str()
            .to_owned()
    );
    assert_eq!(result.rendered_media_type, "text/markdown");
    assert_eq!(
        std::fs::read(&legacy_path).expect("legacy Project Context after authoring"),
        legacy_before
    );
}

#[cfg(unix)]
#[test]
fn author_project_context_refuses_when_non_starter_canonical_truth_exists() {
    let dir = tempfile::tempdir().expect("tempdir");
    legacy_authoring_fixture_repo(dir.path());
    let yaml = handbook_engine::serialize_canonical_project_context(
        &handbook_engine::resolve_shipped_profile_decisions(".").unwrap(),
        &valid_project_context_input(),
    )
    .expect("closed YAML");
    write_file(&dir.path().join(".handbook/project/context.yaml"), &yaml);

    let err = author_project_context_from_input(dir.path(), &valid_project_context_input())
        .expect_err("existing project context truth should refuse");

    assert_eq!(
        err.kind,
        AuthorProjectContextRefusalKind::ExistingCanonicalTruth
    );
    assert_eq!(
        std::fs::read(dir.path().join(".handbook/project/context.yaml"))
            .expect("existing project context"),
        yaml
    );
}

#[cfg(unix)]
#[test]
fn author_project_context_waits_for_lock_and_rechecks_existing_truth() {
    use std::fs::OpenOptions;
    use std::os::unix::io::AsRawFd;
    use std::sync::mpsc;
    use std::thread;
    use std::time::Duration;

    let dir = tempfile::tempdir().expect("tempdir");
    legacy_authoring_fixture_repo(dir.path());
    let canonical = dir.path().join(".handbook/project/context.yaml");
    assert!(!canonical.exists(), "selected truth must start absent");

    let lock_path = dir
        .path()
        .join(".handbook/state/authoring/project_context.lock");
    std::fs::create_dir_all(lock_path.parent().expect("lock parent")).expect("mkdir lock parent");
    let lock = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .truncate(false)
        .open(&lock_path)
        .expect("open authoring lock");
    assert_eq!(
        unsafe { libc::flock(lock.as_raw_fd(), libc::LOCK_EX) },
        0,
        "acquire authoring lock"
    );

    let (tx, rx) = mpsc::channel();
    let repo_root = dir.path().to_path_buf();
    thread::spawn(move || {
        let outcome = author_project_context_from_input(&repo_root, &valid_project_context_input());
        tx.send(outcome).expect("send authoring outcome");
    });

    match rx.recv_timeout(Duration::from_secs(2)) {
        Err(mpsc::RecvTimeoutError::Timeout) => {}
        Err(mpsc::RecvTimeoutError::Disconnected) => {
            panic!("authoring thread disconnected while its lock was held")
        }
        Ok(outcome) => panic!(
            "authoring completed while its lock was held: {:?}",
            outcome.map(|result| result.canonical_repo_relative_path)
        ),
    }
    assert!(
        !canonical.exists(),
        "authoring must not mutate while its lock is held"
    );

    let mut existing_input = valid_project_context_input();
    existing_input.summary = "Existing canonical truth must survive lock contention.".to_string();
    let existing = handbook_engine::serialize_canonical_project_context(
        &handbook_engine::resolve_shipped_profile_decisions(".").unwrap(),
        &existing_input,
    )
    .expect("closed YAML");
    let candidate = handbook_engine::serialize_canonical_project_context(
        &handbook_engine::resolve_shipped_profile_decisions(".").unwrap(),
        &valid_project_context_input(),
    )
    .expect("candidate YAML");
    assert_ne!(
        existing, candidate,
        "installed truth must distinguish overwrite from preservation"
    );
    write_file(&canonical, &existing);
    assert_eq!(
        unsafe { libc::flock(lock.as_raw_fd(), libc::LOCK_UN) },
        0,
        "release authoring lock"
    );

    let err = rx
        .recv_timeout(Duration::from_secs(2))
        .expect("authoring outcome after lock release")
        .expect_err("under-lock preflight must refuse newly installed truth");
    assert_eq!(
        err.kind,
        AuthorProjectContextRefusalKind::ExistingCanonicalTruth
    );
    assert_eq!(
        std::fs::read(&canonical).expect("preserved existing truth"),
        existing
    );
}

#[cfg(unix)]
#[test]
fn preflight_author_project_context_refuses_when_non_starter_canonical_truth_exists() {
    let dir = tempfile::tempdir().expect("tempdir");
    legacy_authoring_fixture_repo(dir.path());
    let yaml = handbook_engine::serialize_canonical_project_context(
        &handbook_engine::resolve_shipped_profile_decisions(".").unwrap(),
        &valid_project_context_input(),
    )
    .expect("closed YAML");
    write_file(&dir.path().join(".handbook/project/context.yaml"), &yaml);

    let err = preflight_author_project_context(dir.path())
        .expect_err("existing project context truth should refuse during preflight");

    assert_eq!(
        err.kind,
        AuthorProjectContextRefusalKind::ExistingCanonicalTruth
    );
    assert!(err
        .summary
        .contains("selected canonical Project Context truth already exists"));
}

#[cfg(unix)]
#[test]
fn author_project_context_repairs_semantically_invalid_canonical_truth() {
    let dir = tempfile::tempdir().expect("tempdir");
    legacy_authoring_fixture_repo(dir.path());
    write_file(
        &dir.path().join(".handbook/project/context.yaml"),
        legacy_placeholder_project_context_markdown().as_bytes(),
    );

    let result = author_project_context_from_input(dir.path(), &valid_project_context_input())
        .expect("invalid project context should be repaired");

    assert_eq!(
        result.canonical_repo_relative_path,
        ".handbook/project/context.yaml"
    );
    assert_eq!(
        std::fs::read(dir.path().join(".handbook/project/context.yaml"))
            .expect("repaired project context"),
        handbook_engine::serialize_canonical_project_context(
            &handbook_engine::resolve_shipped_profile_decisions(".").unwrap(),
            &valid_project_context_input(),
        )
        .unwrap()
    );
}

#[cfg(unix)]
#[test]
fn preflight_author_project_context_routes_ingest_invalid_target_to_setup_refresh() {
    let dir = tempfile::tempdir().expect("tempdir");
    legacy_authoring_fixture_repo(dir.path());
    std::fs::create_dir_all(dir.path().join(".handbook/project/context.yaml"))
        .expect("project-context target directory");

    let err = preflight_author_project_context(dir.path())
        .expect_err("ingest-invalid project-context target should block authoring");

    assert_eq!(err.kind, AuthorProjectContextRefusalKind::MutationRefused);
    assert!(err.summary.contains("cannot be safely inspected"));
}

#[test]
fn project_context_legacy_starter_is_not_selected_canonical_truth() {
    let dir = tempfile::tempdir().expect("tempdir");
    legacy_authoring_fixture_repo(dir.path());

    assert!(!dir.path().join(".handbook/project/context.yaml").exists());
}
