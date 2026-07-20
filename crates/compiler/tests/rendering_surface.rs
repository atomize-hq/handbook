#[cfg(unix)]
use handbook_compiler::render_json;
#[cfg(unix)]
use handbook_compiler::rendering::render_inspect;
use handbook_compiler::rendering::render_markdown;
use handbook_compiler::{build_output_model, resolve};
#[cfg(unix)]
use handbook_engine::{
    parse_canonical_project_context, render_project_context_markdown,
    resolve_shipped_profile_decisions,
};
#[cfg(unix)]
use handbook_engine::{setup_starter_template_bytes, CanonicalArtifactKind};
#[cfg(unix)]
use handbook_flow::BudgetPolicy;
use handbook_flow::ResolveRequest;

#[cfg(unix)]
fn write_file(path: &std::path::Path, contents: &[u8]) {
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent).expect("mkdirs");
    }
    std::fs::write(path, contents).expect("write");
}

#[cfg(unix)]
fn valid_charter_markdown() -> &'static str {
    "# Engineering Charter — Handbook

## What this is
Body.

## How to use this charter
Use it.

## Rubric: 1–5 rigor levels
Levels.

## Project baseline posture
Baseline.

## Domains / areas (optional overrides)
None.

## Posture at a glance (quick scan)
Snapshot.

## Dimensions (details + guardrails)
Details.

## Cross-cutting red lines (global non-negotiables)
- Keep trust boundaries intact.

## Exceptions / overrides process
- **Approvers:** project_owner
- **Record location:** docs/exceptions.md
- **Minimum required fields:**
  - what
  - why
  - scope
  - risk
  - owner
  - expiry_or_revisit_date

## Debt tracking expectations
Tracked in issues.

## Decision Records (ADRs): how to use this charter
Use ADRs.

## Review & updates
Review monthly.
"
}

#[cfg(unix)]
fn valid_project_context_markdown() -> &'static str {
    concat!(
        "schema_id: \"handbook.artifact.project-context\"\n",
        "schema_version: \"1.0\"\n",
        "record_id: \"handbook.project-context\"\n",
        "summary: \"Project reality.\"\n",
        "system_boundaries:\n",
        "  - \"Canonical handbook truth\"\n",
        "ownership:\n",
        "  - \"handbook-team\"\n",
        "authoritative_references:\n",
        "  - \"handbook.charter@1.0.0\"\n",
        "known_unknowns:\n",
        "  - \"None\"\n",
    )
}

#[cfg(unix)]
fn valid_environment_inventory_markdown() -> &'static str {
    "# Environment Inventory

> **Canonical File:** `.handbook/environment_inventory/ENVIRONMENT_INVENTORY.md`
> **Project Context Ref:** `.handbook/project/context.yaml`

## What this is
Canonical environment and runtime inventory.

## How to use
- Update this file when runtime assumptions change.

## 1) Environment Variables (Inventory)
- None yet.

## 2) External Services / Infrastructure Dependencies
- None yet.

## 3) Runtime Assumptions (Ports, Paths, Storage, Limits)
- None yet.

## 4) Local Development Requirements
- None yet.

## 5) CI Requirements
- None yet.

## 6) Production / Deployment Requirements (even if not live yet)
- None yet.

## 7) Dependency & Tooling Inventory (project-specific)
- None yet.

## 8) Update Contract (non-negotiable)
- Update `.handbook/environment_inventory/ENVIRONMENT_INVENTORY.md` in the same change.

## 9) Known Unknowns
- None yet.
"
}

#[cfg(unix)]
fn assert_in_order(haystack: &str, needles: &[&str]) {
    let mut last = 0;
    for needle in needles {
        let offset = haystack[last..]
            .find(needle)
            .unwrap_or_else(|| panic!("missing `{needle}` in output:\n{haystack}"));
        last += offset + needle.len();
    }
}

#[cfg(unix)]
fn write_valid_project_context(root: &std::path::Path) {
    write_file(
        &root.join(".handbook/project/context.yaml"),
        valid_project_context_markdown().as_bytes(),
    );
}

#[cfg(unix)]
#[test]
fn render_markdown_keeps_trust_header_first_for_ready_result() {
    let dir = tempfile::tempdir().expect("tempdir");
    let root = dir.path();

    write_file(
        &root.join(".handbook/charter/CHARTER.md"),
        valid_charter_markdown().as_bytes(),
    );
    write_file(
        &root.join(".handbook/feature_spec/FEATURE_SPEC.md"),
        b"feature",
    );
    write_valid_project_context(root);

    let result = resolve(root, ResolveRequest::default()).expect("resolve");
    let model = build_output_model(&result).expect("model");

    let rendered = render_markdown(&model);
    let lines: Vec<&str> = rendered.lines().take(3).collect();

    assert_eq!(
        lines,
        [
            "OUTCOME: READY",
            "OBJECT: planning.packet",
            "NEXT SAFE ACTION: run `handbook inspect --packet planning.packet` for proof",
        ]
    );
    assert_eq!(render_markdown(&model), rendered);
    assert!(rendered.contains("## PACKET OVERVIEW"));
    assert!(rendered.contains("PACKET VARIANT: planning.packet"));
    assert!(rendered.contains("## INCLUDED SOURCES"));
    assert!(rendered.contains("Charter [.handbook/charter/CHARTER.md]"));
    assert!(rendered.contains("FeatureSpec [.handbook/feature_spec/FEATURE_SPEC.md]"));
    assert!(rendered.contains("## OMISSIONS AND BUDGET"));
    assert!(rendered.contains("## DECISION SUMMARY"));
    assert!(rendered.contains("## PACKET BODY"));
    assert!(rendered.contains("### CHARTER (.handbook/charter/CHARTER.md)"));
    assert!(rendered.contains("### FEATURE_SPEC (.handbook/feature_spec/FEATURE_SPEC.md)"));
    assert!(
        !rendered.contains("render packet body once implemented"),
        "placeholder body text should be gone: {rendered}"
    );
}

#[cfg(unix)]
#[test]
fn render_markdown_keeps_optional_project_context_in_order_when_present() {
    let dir = tempfile::tempdir().expect("tempdir");
    let root = dir.path();

    write_file(
        &root.join(".handbook/charter/CHARTER.md"),
        valid_charter_markdown().as_bytes(),
    );
    write_file(
        &root.join(".handbook/project/context.yaml"),
        valid_project_context_markdown().as_bytes(),
    );
    write_file(
        &root.join(".handbook/feature_spec/FEATURE_SPEC.md"),
        b"feature",
    );

    let result = resolve(root, ResolveRequest::default()).expect("resolve");
    let model = build_output_model(&result).expect("model");

    let rendered = render_markdown(&model);
    let pos_charter = rendered
        .find("### CHARTER (.handbook/charter/CHARTER.md)")
        .expect("charter section");
    let pos_context = rendered
        .find("### PROJECT_CONTEXT (.handbook/project/context.yaml)")
        .expect("project context section");
    let pos_feature = rendered
        .find("### FEATURE_SPEC (.handbook/feature_spec/FEATURE_SPEC.md)")
        .expect("feature spec section");

    assert!(
        pos_charter < pos_context && pos_context < pos_feature,
        "expected section order charter -> project_context -> feature_spec: {rendered}"
    );
    assert!(rendered.contains("ProjectContext [.handbook/project/context.yaml]"));
    assert!(rendered.contains("MODE: rendered from selected canonical YAML"));
    assert!(rendered.contains("SOURCE SHA256: sha256:"));
    assert!(rendered.contains("RENDERED SHA256: sha256:"));
}

#[cfg(unix)]
#[test]
fn render_markdown_omits_optional_feature_spec_starter_template_from_ready_packet() {
    let dir = tempfile::tempdir().expect("tempdir");
    let root = dir.path();

    write_file(
        &root.join(".handbook/charter/CHARTER.md"),
        valid_charter_markdown().as_bytes(),
    );
    write_file(
        &root.join(".handbook/feature_spec/FEATURE_SPEC.md"),
        setup_starter_template_bytes(CanonicalArtifactKind::FeatureSpec),
    );
    write_valid_project_context(root);

    let result = resolve(root, ResolveRequest::default()).expect("resolve");
    let model = build_output_model(&result).expect("model");
    let rendered = render_markdown(&model);

    assert!(rendered.contains("OUTCOME: READY"));
    assert!(rendered.contains("Charter [.handbook/charter/CHARTER.md]"));
    assert!(!rendered.contains("FeatureSpec [.handbook/feature_spec/FEATURE_SPEC.md]"));
    assert!(!rendered.contains("### FEATURE_SPEC (.handbook/feature_spec/FEATURE_SPEC.md)"));
    assert!(rendered.contains(
        "optional source omitted: .handbook/feature_spec/FEATURE_SPEC.md (shipped starter template)"
    ));
}

#[test]
fn render_markdown_keeps_trust_header_first_for_refusal_result() {
    let dir = tempfile::tempdir().expect("tempdir");
    let root = dir.path();

    let result = resolve(root, ResolveRequest::default()).expect("resolve");
    let model = build_output_model(&result).expect("model");

    let rendered = render_markdown(&model);
    let lines: Vec<&str> = rendered.lines().take(3).collect();

    assert_eq!(
        lines,
        [
            "OUTCOME: REFUSED",
            "OBJECT: planning.packet",
            "NEXT SAFE ACTION: run `handbook setup`",
        ]
    );
    assert!(rendered.contains("## REFUSAL"));
    assert!(rendered.contains("CATEGORY: SystemRootMissing"));
    assert!(rendered.contains("BROKEN SUBJECT: policy system_root"));
    assert!(rendered.contains("NEXT SAFE ACTION: run `handbook setup`"));
    assert!(
        !rendered.contains("## PACKET BODY"),
        "refusal output should stay compact: {rendered}"
    );
}

#[cfg(unix)]
#[test]
fn render_json_is_deterministic_for_identical_models() {
    let dir = tempfile::tempdir().expect("tempdir");
    let root = dir.path();

    write_file(
        &root.join(".handbook/charter/CHARTER.md"),
        valid_charter_markdown().as_bytes(),
    );
    write_file(
        &root.join(".handbook/feature_spec/FEATURE_SPEC.md"),
        b"feature",
    );
    write_valid_project_context(root);

    let result = resolve(root, ResolveRequest::default()).expect("resolve");
    let model = build_output_model(&result).expect("model");

    let first = render_json(&model);
    let second = render_json(&model);

    assert_eq!(first, second);
    assert!(first.contains("\"budget_outcome\""));
    assert!(first.contains("\"decision_log_entries\""));
    assert!(first.contains("\"packet_result\""));
    assert!(first.contains("\"ready_next_safe_action\""));
    assert!(first.contains("\"sections\""));
    assert!(first.contains("\"rendered_output_byte_len\""));
    assert!(first.contains("\"source_content_sha256\""));
}

#[cfg(unix)]
#[test]
fn render_json_redacts_packet_body_for_refused_live_execution_requests() {
    let dir = tempfile::tempdir().expect("tempdir");
    let root = dir.path();

    write_file(
        &root.join(".handbook/charter/CHARTER.md"),
        valid_charter_markdown().as_bytes(),
    );
    write_file(
        &root.join(".handbook/feature_spec/FEATURE_SPEC.md"),
        b"feature-body",
    );
    write_valid_project_context(root);

    let result = resolve(
        root,
        ResolveRequest {
            packet_id: "execution.live.packet",
            ..Default::default()
        },
    )
    .expect("resolve");
    let model = build_output_model(&result).expect("model");

    let rendered = render_json(&model);

    assert!(rendered.contains("\"packet_result\""));
    assert!(rendered.contains("\"sections\": [\n    ]"));
    assert!(rendered.contains("\"packet body omitted because request is not ready\""));
    assert!(!rendered.contains(valid_charter_markdown()));
    assert!(!rendered.contains("\"contents\": \"feature-body\""));
}

#[cfg(unix)]
#[test]
fn render_json_does_not_mislabel_optional_read_error_as_omission() {
    let dir = tempfile::tempdir().expect("tempdir");
    let root = dir.path();

    write_file(
        &root.join(".handbook/charter/CHARTER.md"),
        valid_charter_markdown().as_bytes(),
    );
    write_file(
        &root.join(".handbook/feature_spec/FEATURE_SPEC.md"),
        b"feature-body",
    );
    write_valid_project_context(root);
    std::fs::create_dir_all(root.join(".handbook/project_context/PROJECT_CONTEXT.md"))
        .expect("project_context dir");

    let result = resolve(root, ResolveRequest::default()).expect("resolve");
    let model = build_output_model(&result).expect("model");

    let rendered = render_json(&model);

    assert!(rendered.contains("\"packet_result\""));
    assert!(rendered.contains("\"sections\": ["));
    assert!(!rendered.contains("\"category\": \"ArtifactReadError\""));
    assert!(!rendered.contains(
        "\"text\": \"optional source omitted: .handbook/project_context/PROJECT_CONTEXT.md\""
    ));
}

#[cfg(unix)]
#[test]
fn render_inspect_is_deterministic_and_includes_json_fallback() {
    let dir = tempfile::tempdir().expect("tempdir");
    let root = dir.path();

    write_file(
        &root.join(".handbook/charter/CHARTER.md"),
        valid_charter_markdown().as_bytes(),
    );
    write_file(
        &root.join(".handbook/feature_spec/FEATURE_SPEC.md"),
        b"feature",
    );
    write_valid_project_context(root);

    let result = resolve(root, ResolveRequest::default()).expect("resolve");
    let model = build_output_model(&result).expect("model");

    let first = render_inspect(&model);
    let second = render_inspect(&model);

    assert_eq!(first, second);
    assert_eq!(
        first.lines().take(3).collect::<Vec<_>>(),
        vec![
            "OUTCOME: READY",
            "OBJECT: planning.packet",
            "NEXT SAFE ACTION: run `handbook generate --packet planning.packet`",
        ]
    );
    assert!(first.contains("## JSON FALLBACK"));
    assert!(first.contains("\"packet_id\": \"planning.packet\""));
    assert!(
        !first.contains("run `handbook inspect --packet planning.packet` for proof"),
        "inspect ready path should not loop back into inspect: {first}"
    );
    assert!(first.contains("## PACKET OVERVIEW"));
    assert!(first.contains("## PACKET BODY"));
    assert!(first.contains("### CHARTER (.handbook/charter/CHARTER.md)"));
}

#[cfg(unix)]
#[test]
fn render_markdown_includes_execution_demo_fixture_context_and_ready_next_action() {
    let dir = tempfile::tempdir().expect("tempdir");
    let root = dir.path().join("tests/fixtures/execution_demo/basic");
    write_file(
        &root.join(".handbook/charter/CHARTER.md"),
        valid_charter_markdown().as_bytes(),
    );
    write_file(
        &root.join(".handbook/project/context.yaml"),
        valid_project_context_markdown().as_bytes(),
    );
    write_file(
        &root.join(".handbook/environment_inventory/ENVIRONMENT_INVENTORY.md"),
        valid_environment_inventory_markdown().as_bytes(),
    );
    write_file(
        &root.join(".handbook/feature_spec/FEATURE_SPEC.md"),
        b"demo feature",
    );

    let request = ResolveRequest {
        packet_id: "execution.demo.packet",
        ..Default::default()
    };
    let result = resolve(&root, request).expect("resolve");
    let model = build_output_model(&result).expect("model");

    let rendered = render_markdown(&model);
    assert_eq!(
        rendered.lines().take(3).collect::<Vec<_>>(),
        vec![
            "OUTCOME: READY",
            "OBJECT: execution.demo.packet",
            "NEXT SAFE ACTION: run `handbook inspect --packet execution.demo.packet --fixture-set basic` for proof",
        ]
    );
    assert!(rendered.contains("## FIXTURE DEMO"));
    assert!(rendered.contains("MODE: fixture-backed execution demo"));
    assert!(rendered.contains("FIXTURE SET: basic"));
    assert!(rendered.contains("FIXTURE BASIS ROOT: tests/fixtures/execution_demo/basic/.handbook/"));
    assert!(rendered.contains("FIXTURE LINEAGE:"));
    assert_in_order(
        &rendered,
        &[
            "1. Charter [.handbook/charter/CHARTER.md]",
            "2. ProjectContext [.handbook/project/context.yaml]",
            "3. EnvironmentInventory [.handbook/environment_inventory/ENVIRONMENT_INVENTORY.md]",
            "4. FeatureSpec [.handbook/feature_spec/FEATURE_SPEC.md]",
        ],
    );
    assert!(rendered.contains("### CHARTER (.handbook/charter/CHARTER.md)"));
    assert!(rendered.contains("# Engineering Charter — Handbook"));
    assert!(rendered.contains("demo feature"));
}

#[cfg(unix)]
#[test]
fn render_json_preserves_execution_demo_fixture_lineage_order() {
    let dir = tempfile::tempdir().expect("tempdir");
    let root = dir.path().join("tests/fixtures/execution_demo/basic");
    write_file(
        &root.join(".handbook/charter/CHARTER.md"),
        valid_charter_markdown().as_bytes(),
    );
    write_file(
        &root.join(".handbook/project/context.yaml"),
        valid_project_context_markdown().as_bytes(),
    );
    write_file(
        &root.join(".handbook/environment_inventory/ENVIRONMENT_INVENTORY.md"),
        valid_environment_inventory_markdown().as_bytes(),
    );
    write_file(
        &root.join(".handbook/feature_spec/FEATURE_SPEC.md"),
        b"demo feature",
    );

    let request = ResolveRequest {
        packet_id: "execution.demo.packet",
        ..Default::default()
    };
    let result = resolve(&root, request).expect("resolve");
    let model = build_output_model(&result).expect("model");

    let rendered = render_json(&model);
    assert_in_order(
        &rendered,
        &[
            "\"canonical_repo_relative_path\": \".handbook/charter/CHARTER.md\"",
            "\"canonical_repo_relative_path\": \".handbook/project/context.yaml\"",
            "\"canonical_repo_relative_path\": \".handbook/environment_inventory/ENVIRONMENT_INVENTORY.md\"",
            "\"canonical_repo_relative_path\": \".handbook/feature_spec/FEATURE_SPEC.md\"",
        ],
    );
}

#[cfg(unix)]
#[test]
fn render_markdown_marks_budget_summarized_sections_without_leaking_body() {
    let dir = tempfile::tempdir().expect("tempdir");
    let root = dir.path();

    write_file(
        &root.join(".handbook/charter/CHARTER.md"),
        valid_charter_markdown().as_bytes(),
    );
    write_file(
        &root.join(".handbook/feature_spec/FEATURE_SPEC.md"),
        "feature".repeat(1024).as_bytes(),
    );
    write_file(
        &root.join(".handbook/project/context.yaml"),
        valid_project_context_markdown().as_bytes(),
    );

    let result = resolve(
        root,
        ResolveRequest {
            budget_policy: BudgetPolicy {
                max_total_bytes: None,
                max_per_artifact_bytes: Some(valid_charter_markdown().len() as u64),
            },
            ..ResolveRequest::default()
        },
    )
    .expect("resolve");
    let model = build_output_model(&result).expect("model");

    let rendered = render_markdown(&model);
    assert!(rendered.contains("### FEATURE_SPEC (.handbook/feature_spec/FEATURE_SPEC.md)"));
    assert!(rendered.contains("MODE: summarized due to budget"));
    assert!(rendered.contains("budget summary: full contents omitted"));
    assert!(!rendered.contains("featurefeaturefeature"));
}

#[cfg(unix)]
#[test]
fn render_markdown_omits_budget_excluded_optional_sections() {
    let dir = tempfile::tempdir().expect("tempdir");
    let root = dir.path();

    write_file(
        &root.join(".handbook/charter/CHARTER.md"),
        valid_charter_markdown().as_bytes(),
    );
    write_file(
        &root.join(".handbook/feature_spec/FEATURE_SPEC.md"),
        b"feature",
    );
    write_file(
        &root.join(".handbook/project/context.yaml"),
        valid_project_context_markdown().as_bytes(),
    );

    let decisions = resolve_shipped_profile_decisions(root).expect("shipped decisions");
    let project_context =
        parse_canonical_project_context(&decisions, valid_project_context_markdown().as_bytes())
            .expect("canonical Project Context");
    let required_total = valid_charter_markdown().len()
        + render_project_context_markdown(&project_context)
            .expect("rendered Project Context")
            .len();

    let result = resolve(
        root,
        ResolveRequest {
            budget_policy: BudgetPolicy {
                max_total_bytes: Some(required_total as u64),
                max_per_artifact_bytes: None,
            },
            ..ResolveRequest::default()
        },
    )
    .expect("resolve");
    let model = build_output_model(&result).expect("model");

    let rendered = render_markdown(&model);
    assert!(!rendered.contains("FeatureSpec [.handbook/feature_spec/FEATURE_SPEC.md]"));
    assert!(!rendered.contains("### FEATURE_SPEC (.handbook/feature_spec/FEATURE_SPEC.md)"));
    assert!(rendered.contains(
        "optional source excluded due to budget: .handbook/feature_spec/FEATURE_SPEC.md"
    ));
}
