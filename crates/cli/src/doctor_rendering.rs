use serde::Serialize;
use std::fmt::Write;

const DOCTOR_REPORT_SCHEMA_ID: &str = "handbook.repository-doctor-report";
const DOCTOR_REPORT_SCHEMA_VERSION: &str = "1.2.0";

pub(crate) fn render_json(report: &handbook_sdk::DoctorReport) -> Result<String, String> {
    serde_json::to_string_pretty(&DoctorJsonReport::from(report))
        .map(|mut output| {
            output.push('\n');
            output
        })
        .map_err(|_| "failed to serialize doctor report".to_owned())
}

#[derive(Serialize)]
struct DoctorJsonReport<'a> {
    schema_id: &'static str,
    schema_version: &'static str,
    profile_ref: &'a str,
    profile_fingerprint: &'a str,
    stable_role_registry_ref: &'a str,
    stable_role_registry_fingerprint: &'a str,
    conditions: Vec<DoctorJsonCondition<'a>>,
    capabilities: Vec<DoctorJsonCapability<'a>>,
    artifacts: Vec<DoctorJsonArtifact<'a>>,
    project_context: Option<DoctorJsonProjectContext<'a>>,
    charter: Option<DoctorJsonCharter<'a>>,
    status: &'static str,
}

impl<'a> From<&'a handbook_sdk::DoctorReport> for DoctorJsonReport<'a> {
    fn from(report: &'a handbook_sdk::DoctorReport) -> Self {
        Self {
            schema_id: DOCTOR_REPORT_SCHEMA_ID,
            schema_version: DOCTOR_REPORT_SCHEMA_VERSION,
            profile_ref: &report.profile_ref,
            profile_fingerprint: &report.profile_fingerprint,
            stable_role_registry_ref: &report.stable_role_registry_ref,
            stable_role_registry_fingerprint: &report.stable_role_registry_fingerprint,
            conditions: report
                .conditions
                .iter()
                .map(DoctorJsonCondition::from)
                .collect(),
            capabilities: report
                .capabilities
                .iter()
                .map(DoctorJsonCapability::from)
                .collect(),
            artifacts: report
                .artifacts
                .iter()
                .map(DoctorJsonArtifact::from)
                .collect(),
            project_context: report
                .project_context
                .as_ref()
                .map(DoctorJsonProjectContext::from),
            charter: report.charter.as_ref().map(DoctorJsonCharter::from),
            status: status_json_name(report.status),
        }
    }
}

#[derive(Serialize)]
struct DoctorJsonCondition<'a> {
    condition_ref: &'a str,
    condition_definition_fingerprint: &'a str,
    outcome: &'static str,
    reason: &'static str,
    evidence_closure_fingerprint: Option<&'a str>,
}

impl<'a> From<&'a handbook_sdk::ProfileConditionRow> for DoctorJsonCondition<'a> {
    fn from(row: &'a handbook_sdk::ProfileConditionRow) -> Self {
        Self {
            condition_ref: &row.condition_ref,
            condition_definition_fingerprint: &row.condition_definition_fingerprint,
            outcome: condition_outcome_name(row.outcome),
            reason: condition_reason_name(row.reason),
            evidence_closure_fingerprint: row.evidence_closure_fingerprint.as_deref(),
        }
    }
}

#[derive(Serialize)]
struct DoctorJsonCapability<'a> {
    instance_id: &'a str,
    capability_id: &'a str,
    contract_ref: &'a str,
    contract_fingerprint: &'a str,
}

impl<'a> From<&'a handbook_sdk::ProfileCapabilityRow> for DoctorJsonCapability<'a> {
    fn from(row: &'a handbook_sdk::ProfileCapabilityRow) -> Self {
        Self {
            instance_id: &row.instance_id,
            capability_id: &row.capability_id,
            contract_ref: &row.contract_ref,
            contract_fingerprint: &row.contract_fingerprint,
        }
    }
}

#[derive(Serialize)]
struct DoctorJsonArtifact<'a> {
    instance_id: &'a str,
    kind_ref: &'a str,
    role_id: Option<&'a str>,
    capability_ids: &'a [String],
    canonical_path: &'a str,
    requiredness: &'static str,
    condition_ref: Option<&'a str>,
    condition_outcome: Option<&'static str>,
    condition_reason: Option<&'static str>,
    evidence_closure_fingerprint: Option<&'a str>,
    applicability: &'static str,
    inspection_status: &'static str,
    inspection_reason: &'static str,
}

impl<'a> From<&'a handbook_sdk::ProfileArtifactRow> for DoctorJsonArtifact<'a> {
    fn from(row: &'a handbook_sdk::ProfileArtifactRow) -> Self {
        Self {
            instance_id: &row.instance_id,
            kind_ref: &row.kind_ref,
            role_id: row.role_id.as_deref(),
            capability_ids: &row.capability_ids,
            canonical_path: &row.canonical_path,
            requiredness: requiredness_name(row.requiredness),
            condition_ref: row.condition_ref.as_deref(),
            condition_outcome: row.condition_outcome.map(condition_outcome_name),
            condition_reason: row.condition_reason.map(condition_reason_name),
            evidence_closure_fingerprint: row.evidence_closure_fingerprint.as_deref(),
            applicability: applicability_name(row.applicability),
            inspection_status: inspection_status_name(row.inspection_status),
            inspection_reason: inspection_reason_name(row.inspection_reason),
        }
    }
}

#[derive(Serialize)]
struct DoctorJsonProjectContext<'a> {
    instance_id: &'a str,
    kind_ref: &'a str,
    canonical_path: &'a str,
    source_fingerprint: &'a str,
    rendered_output_fingerprint: &'a str,
    rendered_media_type: &'a str,
}

impl<'a> From<&'a handbook_sdk::DoctorProjectContextRow> for DoctorJsonProjectContext<'a> {
    fn from(row: &'a handbook_sdk::DoctorProjectContextRow) -> Self {
        Self {
            instance_id: &row.instance_id,
            kind_ref: &row.kind_ref,
            canonical_path: &row.canonical_path,
            source_fingerprint: &row.source_fingerprint,
            rendered_output_fingerprint: &row.rendered_output_fingerprint,
            rendered_media_type: &row.rendered_media_type,
        }
    }
}

#[derive(Serialize)]
struct DoctorJsonCharterDefinition<'a> {
    exact_ref: &'a str,
    definition_fingerprint: &'a str,
    package_path: &'a str,
}

impl<'a> From<&'a handbook_sdk::DoctorCharterDefinitionRow> for DoctorJsonCharterDefinition<'a> {
    fn from(row: &'a handbook_sdk::DoctorCharterDefinitionRow) -> Self {
        Self {
            exact_ref: &row.exact_ref,
            definition_fingerprint: &row.definition_fingerprint,
            package_path: &row.package_path,
        }
    }
}

#[derive(Serialize)]
struct DoctorJsonCharter<'a> {
    instance_id: &'a str,
    kind_ref: &'a str,
    canonical_path: &'a str,
    definition_closure_status: &'static str,
    definition_closure: Vec<DoctorJsonCharterDefinition<'a>>,
    canonical_status: &'static str,
    canonical_reason: &'static str,
    lifecycle_state: &'static str,
    source_fingerprint: Option<&'a str>,
    rendered_output_fingerprint: Option<&'a str>,
    rendered_media_type: Option<&'a str>,
    next_actions: Vec<&'static str>,
}

impl<'a> From<&'a handbook_sdk::DoctorCharterRow> for DoctorJsonCharter<'a> {
    fn from(row: &'a handbook_sdk::DoctorCharterRow) -> Self {
        Self {
            instance_id: &row.instance_id,
            kind_ref: &row.kind_ref,
            canonical_path: &row.canonical_path,
            definition_closure_status: charter_definition_closure_name(
                row.definition_closure_status,
            ),
            definition_closure: row
                .definition_closure
                .iter()
                .map(DoctorJsonCharterDefinition::from)
                .collect(),
            canonical_status: inspection_status_name(row.canonical_status),
            canonical_reason: inspection_reason_name(row.canonical_reason),
            lifecycle_state: charter_lifecycle_name(row.lifecycle_state),
            source_fingerprint: row.source_fingerprint.as_deref(),
            rendered_output_fingerprint: row.rendered_output_fingerprint.as_deref(),
            rendered_media_type: row.rendered_media_type.as_deref(),
            next_actions: row
                .next_actions
                .iter()
                .copied()
                .map(charter_next_action_name)
                .collect(),
        }
    }
}

pub(crate) fn render_text(report: &handbook_sdk::DoctorReport) -> String {
    let mut output = String::new();
    writeln!(&mut output, "OUTCOME: {}", status_name(report.status)).expect("string write");
    writeln!(&mut output, "PROFILE: {}", report.profile_ref).expect("string write");
    writeln!(&mut output, "## PROFILE ARTIFACTS").expect("string write");
    for artifact in &report.artifacts {
        writeln!(
            &mut output,
            "{} [{}] APPLICABILITY: {} STATUS: {} REASON: {}",
            artifact.instance_id,
            artifact.canonical_path,
            applicability_name(artifact.applicability),
            inspection_status_name(artifact.inspection_status),
            inspection_reason_name(artifact.inspection_reason),
        )
        .expect("string write");
    }
    if let Some(project_context) = &report.project_context {
        writeln!(&mut output, "## PROJECT CONTEXT").expect("string write");
        writeln!(
            &mut output,
            "PATH: {} SOURCE FINGERPRINT: {} RENDERED OUTPUT FINGERPRINT: {} MEDIA TYPE: {}",
            project_context.canonical_path,
            project_context.source_fingerprint,
            project_context.rendered_output_fingerprint,
            project_context.rendered_media_type,
        )
        .expect("string write");
    }
    if let Some(charter) = &report.charter {
        writeln!(&mut output, "## CHARTER").expect("string write");
        writeln!(
            &mut output,
            "PATH: {} DEFINITION CLOSURE: {} STATUS: {} REASON: {} LIFECYCLE: {}",
            charter.canonical_path,
            charter_definition_closure_name(charter.definition_closure_status),
            inspection_status_name(charter.canonical_status),
            inspection_reason_name(charter.canonical_reason),
            charter_lifecycle_name(charter.lifecycle_state),
        )
        .expect("string write");
        if let (Some(source), Some(rendered), Some(media_type)) = (
            &charter.source_fingerprint,
            &charter.rendered_output_fingerprint,
            &charter.rendered_media_type,
        ) {
            writeln!(
                &mut output,
                "SOURCE FINGERPRINT: {source} RENDERED OUTPUT FINGERPRINT: {rendered} MEDIA TYPE: {media_type}"
            )
            .expect("string write");
        }
        for action in &charter.next_actions {
            writeln!(
                &mut output,
                "NEXT ACTION: {}",
                charter_next_action_name(*action)
            )
            .expect("string write");
        }
    }
    output
}

fn charter_definition_closure_name(
    status: handbook_sdk::DoctorCharterDefinitionClosureStatus,
) -> &'static str {
    match status {
        handbook_sdk::DoctorCharterDefinitionClosureStatus::Resolved => "resolved",
        handbook_sdk::DoctorCharterDefinitionClosureStatus::Unavailable => "unavailable",
    }
}

fn charter_lifecycle_name(state: handbook_sdk::DoctorCharterLifecycleState) -> &'static str {
    match state {
        handbook_sdk::DoctorCharterLifecycleState::Unobserved => "unobserved",
    }
}

fn charter_next_action_name(action: handbook_sdk::DoctorCharterNextAction) -> &'static str {
    match action {
        handbook_sdk::DoctorCharterNextAction::RunCharterAuthor => "run_charter_author",
        handbook_sdk::DoctorCharterNextAction::RepairCanonicalCharter => "repair_canonical_charter",
        handbook_sdk::DoctorCharterNextAction::RepairDefinitionClosure => {
            "repair_definition_closure"
        }
        handbook_sdk::DoctorCharterNextAction::ObserveLifecycle => "observe_lifecycle",
    }
}

fn applicability_name(applicability: handbook_sdk::ArtifactApplicability) -> &'static str {
    match applicability {
        handbook_sdk::ArtifactApplicability::Required => "required",
        handbook_sdk::ArtifactApplicability::Optional => "optional",
        handbook_sdk::ArtifactApplicability::Indeterminate => "indeterminate",
    }
}

fn inspection_status_name(status: handbook_sdk::ArtifactInspectionStatus) -> &'static str {
    match status {
        handbook_sdk::ArtifactInspectionStatus::Missing => "missing",
        handbook_sdk::ArtifactInspectionStatus::StructurallyValid => "structurally_valid",
        handbook_sdk::ArtifactInspectionStatus::StructurallyInvalid => "structurally_invalid",
        handbook_sdk::ArtifactInspectionStatus::UnsafePath => "unsafe_path",
        handbook_sdk::ArtifactInspectionStatus::Unreadable => "unreadable",
        handbook_sdk::ArtifactInspectionStatus::NotInspected => "not_inspected",
    }
}

fn inspection_reason_name(reason: handbook_sdk::ArtifactInspectionReason) -> &'static str {
    match reason {
        handbook_sdk::ArtifactInspectionReason::PresentAndStructurallyValid => {
            "present_and_structurally_valid"
        }
        handbook_sdk::ArtifactInspectionReason::RequiredPathMissing => "required_path_missing",
        handbook_sdk::ArtifactInspectionReason::OptionalPathMissing => "optional_path_missing",
        handbook_sdk::ArtifactInspectionReason::ConditionalEvidenceUnavailablePathMissing => {
            "conditional_evidence_unavailable_path_missing"
        }
        handbook_sdk::ArtifactInspectionReason::ConditionalEvidenceUnavailablePathPresent => {
            "conditional_evidence_unavailable_path_present"
        }
        handbook_sdk::ArtifactInspectionReason::YamlSyntaxInvalid => "yaml_syntax_invalid",
        handbook_sdk::ArtifactInspectionReason::DuplicateYamlKey => "duplicate_yaml_key",
        handbook_sdk::ArtifactInspectionReason::DocumentNotObject => "document_not_object",
        handbook_sdk::ArtifactInspectionReason::StructuralValidationFailed => {
            "structural_validation_failed"
        }
        handbook_sdk::ArtifactInspectionReason::DocumentLimitExceeded => "document_limit_exceeded",
        handbook_sdk::ArtifactInspectionReason::AggregateReadLimitExceeded => {
            "aggregate_read_limit_exceeded"
        }
        handbook_sdk::ArtifactInspectionReason::SymlinkRefused => "symlink_refused",
        handbook_sdk::ArtifactInspectionReason::NonRegularFileRefused => "non_regular_file_refused",
        handbook_sdk::ArtifactInspectionReason::UnsafeRepositoryPath => "unsafe_repository_path",
        handbook_sdk::ArtifactInspectionReason::UnsupportedPlatformStrictRead => {
            "unsupported_platform_strict_read"
        }
        handbook_sdk::ArtifactInspectionReason::RepositoryReadFailed => "repository_read_failed",
        handbook_sdk::ArtifactInspectionReason::TypedDecodeFailed => "typed_decode_failed",
        handbook_sdk::ArtifactInspectionReason::RenderedViewRefused => "rendered_view_refused",
        handbook_sdk::ArtifactInspectionReason::ObservationChangedDuringInspection => {
            "observation_changed_during_inspection"
        }
    }
}

fn requiredness_name(requiredness: handbook_sdk::ProfileRequirednessMode) -> &'static str {
    match requiredness {
        handbook_sdk::ProfileRequirednessMode::Always => "always",
        handbook_sdk::ProfileRequirednessMode::Conditional => "conditional",
        handbook_sdk::ProfileRequirednessMode::Optional => "optional",
    }
}

fn condition_outcome_name(outcome: handbook_sdk::ProfileConditionOutcome) -> &'static str {
    match outcome {
        handbook_sdk::ProfileConditionOutcome::True => "true",
        handbook_sdk::ProfileConditionOutcome::False => "false",
        handbook_sdk::ProfileConditionOutcome::Unknown => "unknown",
        handbook_sdk::ProfileConditionOutcome::Unresolved => "unresolved",
        handbook_sdk::ProfileConditionOutcome::Stale => "stale",
        handbook_sdk::ProfileConditionOutcome::Refused => "refused",
    }
}

fn condition_reason_name(reason: handbook_sdk::ProfileConditionReason) -> &'static str {
    match reason {
        handbook_sdk::ProfileConditionReason::EvidenceContractUnavailable => {
            "evidence_contract_unavailable"
        }
    }
}

fn status_name(status: handbook_sdk::RepositoryReadinessStatus) -> &'static str {
    match status {
        handbook_sdk::RepositoryReadinessStatus::Ready => "READY",
        handbook_sdk::RepositoryReadinessStatus::ActionRequired => "ACTION_REQUIRED",
        handbook_sdk::RepositoryReadinessStatus::Indeterminate => "INDETERMINATE",
        handbook_sdk::RepositoryReadinessStatus::Invalid => "INVALID",
    }
}

fn status_json_name(status: handbook_sdk::RepositoryReadinessStatus) -> &'static str {
    match status {
        handbook_sdk::RepositoryReadinessStatus::Ready => "ready",
        handbook_sdk::RepositoryReadinessStatus::ActionRequired => "action_required",
        handbook_sdk::RepositoryReadinessStatus::Indeterminate => "indeterminate",
        handbook_sdk::RepositoryReadinessStatus::Invalid => "invalid",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ready_report_text_and_json_use_exact_status_and_single_lf() {
        let report = handbook_sdk::DoctorReport {
            profile_ref: "example.profile.ready@1.0.0".to_owned(),
            profile_fingerprint: "sha256:ready".to_owned(),
            stable_role_registry_ref: "handbook.roles.core@1.1.0".to_owned(),
            stable_role_registry_fingerprint: "sha256:roles".to_owned(),
            conditions: vec![],
            capabilities: vec![],
            artifacts: vec![],
            project_context: None,
            charter: None,
            status: handbook_sdk::RepositoryReadinessStatus::Ready,
        };

        let text = render_text(&report);
        assert!(text.starts_with("OUTCOME: READY\n"), "{text}");
        let json = render_json(&report).unwrap();
        assert!(json.ends_with('\n'));
        assert!(!json.ends_with("\n\n"));
        let value: serde_json::Value = serde_json::from_str(&json).unwrap();
        assert_eq!(value["status"], "ready");
    }

    #[test]
    fn full_doctor_shape_preserves_the_legacy_json_and_text_contract() {
        let report = handbook_sdk::DoctorReport {
            profile_ref: "example.profile@1.0.0".to_owned(),
            profile_fingerprint: "sha256:profile".to_owned(),
            stable_role_registry_ref: "example.roles@1.0.0".to_owned(),
            stable_role_registry_fingerprint: "sha256:roles".to_owned(),
            conditions: vec![handbook_sdk::ProfileConditionRow {
                condition_ref: "example.condition@1.0.0".to_owned(),
                condition_definition_fingerprint: "sha256:condition".to_owned(),
                outcome: handbook_sdk::ProfileConditionOutcome::Refused,
                reason: handbook_sdk::ProfileConditionReason::EvidenceContractUnavailable,
                evidence_closure_fingerprint: Some("sha256:evidence".to_owned()),
            }],
            capabilities: vec![handbook_sdk::ProfileCapabilityRow {
                instance_id: "project_authority".to_owned(),
                capability_id: "example.capability".to_owned(),
                contract_ref: "example.contract@1.0.0".to_owned(),
                contract_fingerprint: "sha256:contract".to_owned(),
            }],
            artifacts: vec![handbook_sdk::ProfileArtifactRow {
                instance_id: "project_authority".to_owned(),
                kind_ref: "example.kind@1.0.0".to_owned(),
                role_id: Some("project_authority".to_owned()),
                capability_ids: vec!["example.capability".to_owned()],
                canonical_path: ".handbook/project/charter.yaml".to_owned(),
                requiredness: handbook_sdk::ProfileRequirednessMode::Conditional,
                condition_ref: Some("example.condition@1.0.0".to_owned()),
                condition_outcome: Some(handbook_sdk::ProfileConditionOutcome::Refused),
                condition_reason: Some(
                    handbook_sdk::ProfileConditionReason::EvidenceContractUnavailable,
                ),
                evidence_closure_fingerprint: Some("sha256:evidence".to_owned()),
                applicability: handbook_sdk::ArtifactApplicability::Indeterminate,
                inspection_status: handbook_sdk::ArtifactInspectionStatus::Unreadable,
                inspection_reason:
                    handbook_sdk::ArtifactInspectionReason::ObservationChangedDuringInspection,
            }],
            project_context: Some(handbook_sdk::DoctorProjectContextRow {
                instance_id: "project_context".to_owned(),
                kind_ref: "example.context@1.0.0".to_owned(),
                canonical_path: ".handbook/project/context.yaml".to_owned(),
                source_fingerprint: "sha256:source".to_owned(),
                rendered_output_fingerprint: "sha256:rendered".to_owned(),
                rendered_media_type: "text/markdown".to_owned(),
            }),
            charter: Some(handbook_sdk::DoctorCharterRow {
                instance_id: "project_authority".to_owned(),
                kind_ref: "example.kind@1.0.0".to_owned(),
                canonical_path: ".handbook/project/charter.yaml".to_owned(),
                definition_closure_status:
                    handbook_sdk::DoctorCharterDefinitionClosureStatus::Resolved,
                definition_closure: vec![handbook_sdk::DoctorCharterDefinitionRow {
                    exact_ref: "example.charter@1.0.0".to_owned(),
                    definition_fingerprint: "sha256:definition".to_owned(),
                    package_path: "definitions/example.yaml".to_owned(),
                }],
                canonical_status: handbook_sdk::ArtifactInspectionStatus::StructurallyValid,
                canonical_reason:
                    handbook_sdk::ArtifactInspectionReason::PresentAndStructurallyValid,
                lifecycle_state: handbook_sdk::DoctorCharterLifecycleState::Unobserved,
                source_fingerprint: Some("sha256:charter-source".to_owned()),
                rendered_output_fingerprint: Some("sha256:charter-rendered".to_owned()),
                rendered_media_type: Some("text/markdown".to_owned()),
                next_actions: vec![
                    handbook_sdk::DoctorCharterNextAction::RepairDefinitionClosure,
                    handbook_sdk::DoctorCharterNextAction::ObserveLifecycle,
                ],
            }),
            status: handbook_sdk::RepositoryReadinessStatus::Indeterminate,
        };

        let json = render_json(&report).unwrap();
        let value: serde_json::Value = serde_json::from_str(&json).unwrap();
        assert_eq!(value["schema_id"], DOCTOR_REPORT_SCHEMA_ID);
        assert_eq!(value["schema_version"], DOCTOR_REPORT_SCHEMA_VERSION);
        assert_eq!(value["conditions"][0]["outcome"], "refused");
        assert_eq!(
            value["artifacts"][0]["inspection_reason"],
            "observation_changed_during_inspection"
        );
        assert_eq!(
            value["charter"]["definition_closure"][0]["package_path"],
            "definitions/example.yaml"
        );
        assert_eq!(
            value["charter"]["next_actions"],
            serde_json::json!(["repair_definition_closure", "observe_lifecycle"])
        );
        assert_eq!(value["status"], "indeterminate");

        let text = render_text(&report);
        assert_eq!(
            text,
            concat!(
                "OUTCOME: INDETERMINATE\n",
                "PROFILE: example.profile@1.0.0\n",
                "## PROFILE ARTIFACTS\n",
                "project_authority [.handbook/project/charter.yaml] APPLICABILITY: indeterminate STATUS: unreadable REASON: observation_changed_during_inspection\n",
                "## PROJECT CONTEXT\n",
                "PATH: .handbook/project/context.yaml SOURCE FINGERPRINT: sha256:source RENDERED OUTPUT FINGERPRINT: sha256:rendered MEDIA TYPE: text/markdown\n",
                "## CHARTER\n",
                "PATH: .handbook/project/charter.yaml DEFINITION CLOSURE: resolved STATUS: structurally_valid REASON: present_and_structurally_valid LIFECYCLE: unobserved\n",
                "SOURCE FINGERPRINT: sha256:charter-source RENDERED OUTPUT FINGERPRINT: sha256:charter-rendered MEDIA TYPE: text/markdown\n",
                "NEXT ACTION: repair_definition_closure\n",
                "NEXT ACTION: observe_lifecycle\n",
            )
        );
    }
}
