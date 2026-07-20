use std::fmt::Write;

pub(crate) fn render_json(report: &handbook_compiler::DoctorReport) -> Result<String, String> {
    serde_json::to_string_pretty(report)
        .map(|mut output| {
            output.push('\n');
            output
        })
        .map_err(|_| "failed to serialize doctor report".to_owned())
}

pub(crate) fn render_text(report: &handbook_compiler::DoctorReport) -> String {
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
    status: handbook_compiler::DoctorCharterDefinitionClosureStatus,
) -> &'static str {
    match status {
        handbook_compiler::DoctorCharterDefinitionClosureStatus::Resolved => "resolved",
        handbook_compiler::DoctorCharterDefinitionClosureStatus::Unavailable => "unavailable",
    }
}

fn charter_lifecycle_name(state: handbook_compiler::DoctorCharterLifecycleState) -> &'static str {
    match state {
        handbook_compiler::DoctorCharterLifecycleState::Unobserved => "unobserved",
    }
}

fn charter_next_action_name(action: handbook_compiler::DoctorCharterNextAction) -> &'static str {
    match action {
        handbook_compiler::DoctorCharterNextAction::RunCharterAuthor => "run_charter_author",
        handbook_compiler::DoctorCharterNextAction::RepairCanonicalCharter => {
            "repair_canonical_charter"
        }
        handbook_compiler::DoctorCharterNextAction::RepairDefinitionClosure => {
            "repair_definition_closure"
        }
        handbook_compiler::DoctorCharterNextAction::ObserveLifecycle => "observe_lifecycle",
    }
}

fn applicability_name(applicability: handbook_engine::ArtifactApplicability) -> &'static str {
    match applicability {
        handbook_engine::ArtifactApplicability::Required => "required",
        handbook_engine::ArtifactApplicability::Optional => "optional",
        handbook_engine::ArtifactApplicability::Indeterminate => "indeterminate",
    }
}

fn inspection_status_name(status: handbook_engine::ArtifactInspectionStatus) -> &'static str {
    match status {
        handbook_engine::ArtifactInspectionStatus::Missing => "missing",
        handbook_engine::ArtifactInspectionStatus::StructurallyValid => "structurally_valid",
        handbook_engine::ArtifactInspectionStatus::StructurallyInvalid => "structurally_invalid",
        handbook_engine::ArtifactInspectionStatus::UnsafePath => "unsafe_path",
        handbook_engine::ArtifactInspectionStatus::Unreadable => "unreadable",
        handbook_engine::ArtifactInspectionStatus::NotInspected => "not_inspected",
    }
}

fn inspection_reason_name(reason: handbook_engine::ArtifactInspectionReason) -> &'static str {
    match reason {
        handbook_engine::ArtifactInspectionReason::PresentAndStructurallyValid => {
            "present_and_structurally_valid"
        }
        handbook_engine::ArtifactInspectionReason::RequiredPathMissing => "required_path_missing",
        handbook_engine::ArtifactInspectionReason::OptionalPathMissing => "optional_path_missing",
        handbook_engine::ArtifactInspectionReason::ConditionalEvidenceUnavailablePathMissing => {
            "conditional_evidence_unavailable_path_missing"
        }
        handbook_engine::ArtifactInspectionReason::ConditionalEvidenceUnavailablePathPresent => {
            "conditional_evidence_unavailable_path_present"
        }
        handbook_engine::ArtifactInspectionReason::YamlSyntaxInvalid => "yaml_syntax_invalid",
        handbook_engine::ArtifactInspectionReason::DuplicateYamlKey => "duplicate_yaml_key",
        handbook_engine::ArtifactInspectionReason::DocumentNotObject => "document_not_object",
        handbook_engine::ArtifactInspectionReason::StructuralValidationFailed => {
            "structural_validation_failed"
        }
        handbook_engine::ArtifactInspectionReason::DocumentLimitExceeded => {
            "document_limit_exceeded"
        }
        handbook_engine::ArtifactInspectionReason::AggregateReadLimitExceeded => {
            "aggregate_read_limit_exceeded"
        }
        handbook_engine::ArtifactInspectionReason::SymlinkRefused => "symlink_refused",
        handbook_engine::ArtifactInspectionReason::NonRegularFileRefused => {
            "non_regular_file_refused"
        }
        handbook_engine::ArtifactInspectionReason::UnsafeRepositoryPath => "unsafe_repository_path",
        handbook_engine::ArtifactInspectionReason::UnsupportedPlatformStrictRead => {
            "unsupported_platform_strict_read"
        }
        handbook_engine::ArtifactInspectionReason::RepositoryReadFailed => "repository_read_failed",
        handbook_engine::ArtifactInspectionReason::TypedDecodeFailed => "typed_decode_failed",
        handbook_engine::ArtifactInspectionReason::RenderedViewRefused => "rendered_view_refused",
        handbook_engine::ArtifactInspectionReason::ObservationChangedDuringInspection => {
            "observation_changed_during_inspection"
        }
    }
}

fn status_name(status: handbook_compiler::RepositoryReadinessStatus) -> &'static str {
    match status {
        handbook_compiler::RepositoryReadinessStatus::Ready => "READY",
        handbook_compiler::RepositoryReadinessStatus::ActionRequired => "ACTION_REQUIRED",
        handbook_compiler::RepositoryReadinessStatus::Indeterminate => "INDETERMINATE",
        handbook_compiler::RepositoryReadinessStatus::Invalid => "INVALID",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ready_report_text_and_json_use_exact_status_and_single_lf() {
        let report = handbook_compiler::DoctorReport {
            schema_id: handbook_compiler::DOCTOR_REPORT_SCHEMA_ID.to_owned(),
            schema_version: handbook_compiler::DOCTOR_REPORT_SCHEMA_VERSION.to_owned(),
            profile_ref: "example.profile.ready@1.0.0".to_owned(),
            profile_fingerprint: "sha256:ready".to_owned(),
            stable_role_registry_ref: "handbook.roles.core@1.1.0".to_owned(),
            stable_role_registry_fingerprint: "sha256:roles".to_owned(),
            conditions: vec![],
            capabilities: vec![],
            artifacts: vec![],
            project_context: None,
            charter: None,
            status: handbook_compiler::RepositoryReadinessStatus::Ready,
        };

        let text = render_text(&report);
        assert!(text.starts_with("OUTCOME: READY\n"), "{text}");
        let json = render_json(&report).unwrap();
        assert!(json.ends_with('\n'));
        assert!(!json.ends_with("\n\n"));
        let value: serde_json::Value = serde_json::from_str(&json).unwrap();
        assert_eq!(value["status"], "ready");
    }
}
