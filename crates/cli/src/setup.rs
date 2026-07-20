use crate::{exit_policy, SetupArgs, SetupCommand};
use std::fmt::Write;
use std::process::ExitCode;

pub(crate) fn run(args: SetupArgs) -> ExitCode {
    let cwd = match std::env::current_dir() {
        Ok(dir) => dir,
        Err(_) => {
            println!("OUTCOME: ERROR\nCATEGORY: repository_root_unavailable");
            return exit_policy::failure();
        }
    };
    let repo_root = crate::shell_shared::discover_managed_repo_root(&cwd);
    let request = match args.command {
        None => handbook_compiler::SetupRequest::default(),
        Some(SetupCommand::Init) => handbook_compiler::SetupRequest {
            mode: handbook_compiler::SetupMode::Init,
            ..handbook_compiler::SetupRequest::default()
        },
        Some(SetupCommand::Refresh(refresh)) => handbook_compiler::SetupRequest {
            mode: handbook_compiler::SetupMode::Refresh,
            rewrite: refresh.rewrite,
            reset_state: refresh.reset_state,
        },
    };

    match handbook_compiler::run_setup(&repo_root, &request) {
        Ok(outcome) => {
            print!("{}", render_setup_outcome(&outcome));
            exit_policy::repository_status(outcome.status)
        }
        Err(error) => {
            print!("{}", render_setup_error(&error));
            exit_policy::failure()
        }
    }
}

fn render_setup_outcome(outcome: &handbook_compiler::SetupOutcome) -> String {
    let mut output = String::new();
    writeln!(&mut output, "OUTCOME: {}", readiness_name(outcome.status)).expect("string write");
    writeln!(&mut output, "PROFILE: {}", outcome.plan.profile_ref).expect("string write");
    writeln!(
        &mut output,
        "MODE: {}",
        setup_mode_name(outcome.plan.resolved_mode)
    )
    .expect("string write");
    writeln!(
        &mut output,
        "ROOT ACTION: {}",
        root_action_name(outcome.plan.root_action)
    )
    .expect("string write");
    writeln!(&mut output, "## PROFILE ARTIFACTS").expect("string write");
    for action in &outcome.plan.artifacts {
        writeln!(
            &mut output,
            "{} [{}] ACTION: {} STATUS: {} REASON: {}",
            action.artifact.instance_id,
            action.artifact.canonical_path,
            artifact_action_name(action.action),
            inspection_status_name(action.artifact.inspection_status),
            inspection_reason_name(action.artifact.inspection_reason),
        )
        .expect("string write");
    }
    writeln!(
        &mut output,
        "RESET APPLIED: {}",
        if outcome.reset_applied { "yes" } else { "no" }
    )
    .expect("string write");
    output
}

fn render_setup_error(error: &handbook_compiler::SetupError) -> String {
    let mut output = String::new();
    writeln!(&mut output, "OUTCOME: ERROR").expect("string write");
    writeln!(
        &mut output,
        "CATEGORY: {}",
        setup_error_kind_name(error.kind())
    )
    .expect("string write");
    writeln!(
        &mut output,
        "REASON: {}",
        setup_error_reason_name(error.reason_code())
    )
    .expect("string write");
    if let Some(path) = error.repo_relative_path() {
        writeln!(&mut output, "SUBJECT: {path}").expect("string write");
    }
    output
}

fn readiness_name(status: handbook_compiler::RepositoryReadinessStatus) -> &'static str {
    match status {
        handbook_compiler::RepositoryReadinessStatus::Ready => "READY",
        handbook_compiler::RepositoryReadinessStatus::ActionRequired => "ACTION_REQUIRED",
        handbook_compiler::RepositoryReadinessStatus::Indeterminate => "INDETERMINATE",
        handbook_compiler::RepositoryReadinessStatus::Invalid => "INVALID",
    }
}

fn setup_mode_name(mode: handbook_compiler::SetupMode) -> &'static str {
    match mode {
        handbook_compiler::SetupMode::Auto => "auto",
        handbook_compiler::SetupMode::Init => "init",
        handbook_compiler::SetupMode::Refresh => "refresh",
    }
}

fn root_action_name(action: handbook_compiler::SetupRootAction) -> &'static str {
    match action {
        handbook_compiler::SetupRootAction::Preserve => "preserve",
        handbook_compiler::SetupRootAction::Create => "create",
    }
}

fn artifact_action_name(action: handbook_compiler::SetupArtifactActionKind) -> &'static str {
    match action {
        handbook_compiler::SetupArtifactActionKind::Preserve => "preserve",
        handbook_compiler::SetupArtifactActionKind::AuthorRequired => "author_required",
        handbook_compiler::SetupArtifactActionKind::OptionalAbsent => "optional_absent",
        handbook_compiler::SetupArtifactActionKind::ConditionIndeterminate => {
            "condition_indeterminate"
        }
        handbook_compiler::SetupArtifactActionKind::Invalid => "invalid",
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

fn setup_error_kind_name(kind: handbook_compiler::SetupErrorKind) -> &'static str {
    match kind {
        handbook_compiler::SetupErrorKind::ProfileResolution => "profile_resolution",
        handbook_compiler::SetupErrorKind::ProfileDecision => "profile_decision",
        handbook_compiler::SetupErrorKind::AlreadyInitialized => "already_initialized",
        handbook_compiler::SetupErrorKind::MissingCanonicalRoot => "missing_canonical_root",
        handbook_compiler::SetupErrorKind::InvalidCanonicalRoot => "invalid_canonical_root",
        handbook_compiler::SetupErrorKind::InvalidRequest => "invalid_request",
        handbook_compiler::SetupErrorKind::MaterializerUnavailable => "materializer_unavailable",
        handbook_compiler::SetupErrorKind::RuntimeStatePlan => "runtime_state_plan",
        handbook_compiler::SetupErrorKind::RuntimeStateApply => "runtime_state_apply",
        handbook_compiler::SetupErrorKind::RepositoryIdentity => "repository_identity",
    }
}

fn setup_error_reason_name(reason: handbook_compiler::SetupErrorReasonCode) -> &'static str {
    match reason {
        handbook_compiler::SetupErrorReasonCode::ShippedProfileUnavailable => {
            "shipped_profile_unavailable"
        }
        handbook_compiler::SetupErrorReasonCode::SelectedProfileDecisionInvalid => {
            "selected_profile_decision_invalid"
        }
        handbook_compiler::SetupErrorReasonCode::UnresolvedMode => "unresolved_mode",
        handbook_compiler::SetupErrorReasonCode::InitRejectsRefreshFlags => {
            "init_rejects_refresh_flags"
        }
        handbook_compiler::SetupErrorReasonCode::RootAlreadyInitialized => {
            "root_already_initialized"
        }
        handbook_compiler::SetupErrorReasonCode::RefreshRootMissing => "refresh_root_missing",
        handbook_compiler::SetupErrorReasonCode::RootNotDirectory => "root_not_directory",
        handbook_compiler::SetupErrorReasonCode::RootSymlinkRefused => "root_symlink_refused",
        handbook_compiler::SetupErrorReasonCode::CanonicalRootInspectFailed => {
            "canonical_root_inspect_failed"
        }
        handbook_compiler::SetupErrorReasonCode::CanonicalRootCreateFailed => {
            "canonical_root_create_failed"
        }
        handbook_compiler::SetupErrorReasonCode::RewriteHasNoMaterializer => {
            "rewrite_has_no_materializer"
        }
        handbook_compiler::SetupErrorReasonCode::RuntimeStateTargetUnsafe => {
            "runtime_state_target_unsafe"
        }
        handbook_compiler::SetupErrorReasonCode::RuntimeStateMutationFailed => {
            "runtime_state_mutation_failed"
        }
        handbook_compiler::SetupErrorReasonCode::RepositoryAuthorityRecoveryBlocked => {
            "repository_authority_recovery_blocked"
        }
        handbook_compiler::SetupErrorReasonCode::RepositoryIdentityUnsafe => {
            "repository_identity_unsafe"
        }
        handbook_compiler::SetupErrorReasonCode::RepositoryIdentityMismatch => {
            "repository_identity_mismatch"
        }
        handbook_compiler::SetupErrorReasonCode::RepositoryIdentityEntropyUnavailable => {
            "repository_identity_entropy_unavailable"
        }
        handbook_compiler::SetupErrorReasonCode::RepositoryIdentityPersistenceFailed => {
            "repository_identity_persistence_failed"
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{setup_error_kind_name, setup_error_reason_name};

    #[test]
    fn repository_identity_setup_error_projection_is_exact() {
        assert_eq!(
            setup_error_kind_name(handbook_compiler::SetupErrorKind::RepositoryIdentity),
            "repository_identity"
        );
        let cases = [
            (
                handbook_compiler::SetupErrorReasonCode::RepositoryAuthorityRecoveryBlocked,
                "repository_authority_recovery_blocked",
            ),
            (
                handbook_compiler::SetupErrorReasonCode::RepositoryIdentityUnsafe,
                "repository_identity_unsafe",
            ),
            (
                handbook_compiler::SetupErrorReasonCode::RepositoryIdentityMismatch,
                "repository_identity_mismatch",
            ),
            (
                handbook_compiler::SetupErrorReasonCode::RepositoryIdentityEntropyUnavailable,
                "repository_identity_entropy_unavailable",
            ),
            (
                handbook_compiler::SetupErrorReasonCode::RepositoryIdentityPersistenceFailed,
                "repository_identity_persistence_failed",
            ),
        ];
        for (reason, expected) in cases {
            assert_eq!(setup_error_reason_name(reason), expected);
        }
    }
}
