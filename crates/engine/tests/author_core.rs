use handbook_engine::{
    render_charter_markdown, validate_charter_structured_input, CharterAudience,
    CharterBackwardCompatibility, CharterDebtTrackingInput, CharterDecisionRecordsInput,
    CharterDefaultImplicationsInput, CharterDeprecationPolicy, CharterDimensionInput,
    CharterDimensionName, CharterDomainInput, CharterExceptionsInput, CharterExpectedLifetime,
    CharterObservabilityThreshold, CharterOperationalRealityInput, CharterPostureInput,
    CharterProjectClassification, CharterProjectConstraintsInput, CharterProjectInput,
    CharterRequiredness, CharterRolloutControls, CharterRuntimeEnvironment, CharterStructuredInput,
    CharterSurface, DEFAULT_EXCEPTION_RECORD_LOCATION,
};

fn valid_charter_input() -> CharterStructuredInput {
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
            charter_dimension(CharterDimensionName::SpeedVsQuality),
            charter_dimension(CharterDimensionName::TypeSafetyStaticAnalysis),
            charter_dimension(CharterDimensionName::TestingRigor),
            charter_dimension(CharterDimensionName::ScalabilityPerformance),
            charter_dimension(CharterDimensionName::ReliabilityOperability),
            charter_dimension(CharterDimensionName::SecurityPrivacy),
            charter_dimension(CharterDimensionName::Observability),
            charter_dimension(CharterDimensionName::DxToolingAutomation),
            charter_dimension(CharterDimensionName::UxPolishApiUsability),
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

fn charter_dimension(name: CharterDimensionName) -> CharterDimensionInput {
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

#[test]
fn charter_core_renders_required_sections_from_engine() {
    let input = valid_charter_input();
    validate_charter_structured_input(&input).expect("valid charter input");

    let markdown = render_charter_markdown(&input).expect("render charter markdown");
    assert!(markdown.starts_with("# Engineering Charter — Handbook"));
    assert!(markdown.contains("## Dimensions (details + guardrails)"));
    assert!(markdown.contains("## Review & updates"));
}

#[test]
fn handbook_product_exception_record_default_remains_explicitly_bounded() {
    assert_eq!(
        DEFAULT_EXCEPTION_RECORD_LOCATION,
        ".handbook/charter/CHARTER.md#exceptions"
    );
}
