use crate::{doctor_rendering, exit_policy, SetupArgs, SetupCommand};
use handbook_sdk::contract_membrane::{
    ContractLimits, InvocationResponseV1, TransportFailureKind, TransportFailureV1,
};
use std::fmt::Write;
use std::process::ExitCode;

pub(crate) fn run(args: SetupArgs, machine_json: bool) -> ExitCode {
    let limits = ContractLimits::packet0();
    let cwd = match std::env::current_dir() {
        Ok(dir) => dir,
        Err(_) => {
            if machine_json {
                return emit_transport_failure(
                    TransportFailureKind::Repository,
                    "transport.repository_root_unavailable",
                    &limits,
                );
            }
            return exit_policy::emit_stdout(
                b"OUTCOME: ERROR\nCATEGORY: repository_root_unavailable\n",
                exit_policy::failure(),
            );
        }
    };
    let repo_root = crate::shell_shared::discover_managed_repo_root(&cwd);
    let request = match args.command {
        None => handbook_sdk::SetupRequest::default(),
        Some(SetupCommand::Init) => handbook_sdk::SetupRequest {
            mode: handbook_sdk::SetupMode::Init,
            ..handbook_sdk::SetupRequest::default()
        },
        Some(SetupCommand::Refresh(refresh)) => handbook_sdk::SetupRequest {
            mode: handbook_sdk::SetupMode::Refresh,
            rewrite: refresh.rewrite,
            reset_state: refresh.reset_state,
        },
    };

    let use_case = match handbook_sdk::HandbookSdkV1::open(&repo_root)
        .run_setup_use_case(&request, &limits)
    {
        Ok(use_case) => use_case,
        Err(_) if machine_json => {
            return emit_transport_failure(
                TransportFailureKind::Serialization,
                "transport.invocation_construction_failed",
                &limits,
            );
        }
        Err(_) => {
            return exit_policy::emit_stdout(
                b"OUTCOME: ERROR\nCATEGORY: serialization\nREASON: failed to construct setup response\n",
                exit_policy::failure(),
            );
        }
    };
    let invocation = match InvocationResponseV1::use_case(use_case, &limits) {
        Ok(invocation) => invocation,
        Err(_) if machine_json => {
            return emit_transport_failure(
                TransportFailureKind::Serialization,
                "transport.invocation_construction_failed",
                &limits,
            );
        }
        Err(_) => {
            return exit_policy::emit_stdout(
                b"OUTCOME: ERROR\nCATEGORY: serialization\nREASON: failed to construct setup response\n",
                exit_policy::failure(),
            );
        }
    };
    let semantic_exit = exit_policy::terminal(invocation.terminal_status());

    if machine_json {
        return match doctor_rendering::render_invocation(&invocation) {
            Ok(bytes) => exit_policy::emit_stdout(&bytes, semantic_exit),
            Err(_) => emit_transport_failure(
                TransportFailureKind::Serialization,
                "transport.serialization_failed",
                &limits,
            ),
        };
    }

    let outcome = match invocation.repository_setup_outcome(&limits) {
        Ok(Some(outcome)) => outcome,
        Ok(None) | Err(_) => {
            return exit_policy::emit_stdout(
                b"OUTCOME: ERROR\nCATEGORY: serialization\nREASON: failed to render setup response\n",
                exit_policy::failure(),
            );
        }
    };
    let output = match outcome {
        Ok(outcome) => render_setup_outcome(&outcome),
        Err(error) => render_setup_error(&error),
    };
    exit_policy::emit_stdout(output.as_bytes(), semantic_exit)
}

fn emit_transport_failure(
    kind: TransportFailureKind,
    reason_code: &str,
    limits: &ContractLimits,
) -> ExitCode {
    let bytes = TransportFailureV1::adapter_error(kind, reason_code, limits)
        .and_then(|failure| InvocationResponseV1::transport_failure(failure, limits))
        .map_err(|_| ())
        .and_then(|invocation| doctor_rendering::render_invocation(&invocation).map_err(|_| ()));
    match bytes {
        Ok(bytes) => exit_policy::emit_stdout(&bytes, exit_policy::failure()),
        Err(()) => {
            eprintln!("handbook: failed to prepare transport-failure response");
            exit_policy::failure()
        }
    }
}

fn render_setup_outcome(outcome: &handbook_sdk::SetupOutcome) -> String {
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

fn render_setup_error(error: &handbook_sdk::SetupError) -> String {
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

fn readiness_name(status: handbook_sdk::RepositoryReadinessStatus) -> &'static str {
    match status {
        handbook_sdk::RepositoryReadinessStatus::Ready => "READY",
        handbook_sdk::RepositoryReadinessStatus::ActionRequired => "ACTION_REQUIRED",
        handbook_sdk::RepositoryReadinessStatus::Indeterminate => "INDETERMINATE",
        handbook_sdk::RepositoryReadinessStatus::Invalid => "INVALID",
    }
}

fn setup_mode_name(mode: handbook_sdk::SetupMode) -> &'static str {
    match mode {
        handbook_sdk::SetupMode::Auto => "auto",
        handbook_sdk::SetupMode::Init => "init",
        handbook_sdk::SetupMode::Refresh => "refresh",
    }
}

fn root_action_name(action: handbook_sdk::SetupRootAction) -> &'static str {
    match action {
        handbook_sdk::SetupRootAction::Preserve => "preserve",
        handbook_sdk::SetupRootAction::Create => "create",
    }
}

fn artifact_action_name(action: handbook_sdk::SetupArtifactActionKind) -> &'static str {
    match action {
        handbook_sdk::SetupArtifactActionKind::Preserve => "preserve",
        handbook_sdk::SetupArtifactActionKind::AuthorRequired => "author_required",
        handbook_sdk::SetupArtifactActionKind::OptionalAbsent => "optional_absent",
        handbook_sdk::SetupArtifactActionKind::ConditionIndeterminate => "condition_indeterminate",
        handbook_sdk::SetupArtifactActionKind::Invalid => "invalid",
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

fn setup_error_kind_name(kind: handbook_sdk::SetupErrorKind) -> &'static str {
    match kind {
        handbook_sdk::SetupErrorKind::ProfileResolution => "profile_resolution",
        handbook_sdk::SetupErrorKind::ProfileDecision => "profile_decision",
        handbook_sdk::SetupErrorKind::AlreadyInitialized => "already_initialized",
        handbook_sdk::SetupErrorKind::MissingCanonicalRoot => "missing_canonical_root",
        handbook_sdk::SetupErrorKind::InvalidCanonicalRoot => "invalid_canonical_root",
        handbook_sdk::SetupErrorKind::InvalidRequest => "invalid_request",
        handbook_sdk::SetupErrorKind::MaterializerUnavailable => "materializer_unavailable",
        handbook_sdk::SetupErrorKind::RuntimeStatePlan => "runtime_state_plan",
        handbook_sdk::SetupErrorKind::RuntimeStateApply => "runtime_state_apply",
        handbook_sdk::SetupErrorKind::RepositoryIdentity => "repository_identity",
    }
}

fn setup_error_reason_name(reason: handbook_sdk::SetupErrorReasonCode) -> &'static str {
    match reason {
        handbook_sdk::SetupErrorReasonCode::ShippedProfileUnavailable => {
            "shipped_profile_unavailable"
        }
        handbook_sdk::SetupErrorReasonCode::SelectedProfileDecisionInvalid => {
            "selected_profile_decision_invalid"
        }
        handbook_sdk::SetupErrorReasonCode::UnresolvedMode => "unresolved_mode",
        handbook_sdk::SetupErrorReasonCode::InitRejectsRefreshFlags => "init_rejects_refresh_flags",
        handbook_sdk::SetupErrorReasonCode::RootAlreadyInitialized => "root_already_initialized",
        handbook_sdk::SetupErrorReasonCode::RefreshRootMissing => "refresh_root_missing",
        handbook_sdk::SetupErrorReasonCode::RootNotDirectory => "root_not_directory",
        handbook_sdk::SetupErrorReasonCode::RootSymlinkRefused => "root_symlink_refused",
        handbook_sdk::SetupErrorReasonCode::CanonicalRootInspectFailed => {
            "canonical_root_inspect_failed"
        }
        handbook_sdk::SetupErrorReasonCode::CanonicalRootCreateFailed => {
            "canonical_root_create_failed"
        }
        handbook_sdk::SetupErrorReasonCode::RewriteHasNoMaterializer => {
            "rewrite_has_no_materializer"
        }
        handbook_sdk::SetupErrorReasonCode::RuntimeStateTargetUnsafe => {
            "runtime_state_target_unsafe"
        }
        handbook_sdk::SetupErrorReasonCode::RuntimeStateResetPlanTooLarge => {
            "runtime_state_reset_plan_too_large"
        }
        handbook_sdk::SetupErrorReasonCode::RuntimeStateResetProjectionTooLarge => {
            "runtime_state_reset_projection_too_large"
        }
        handbook_sdk::SetupErrorReasonCode::RuntimeStateMutationFailed => {
            "runtime_state_mutation_failed"
        }
        handbook_sdk::SetupErrorReasonCode::RepositoryAuthorityRecoveryBlocked => {
            "repository_authority_recovery_blocked"
        }
        handbook_sdk::SetupErrorReasonCode::RepositoryIdentityUnsafe => {
            "repository_identity_unsafe"
        }
        handbook_sdk::SetupErrorReasonCode::RepositoryIdentityMismatch => {
            "repository_identity_mismatch"
        }
        handbook_sdk::SetupErrorReasonCode::RepositoryIdentityEntropyUnavailable => {
            "repository_identity_entropy_unavailable"
        }
        handbook_sdk::SetupErrorReasonCode::RepositoryIdentityPersistenceFailed => {
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
            setup_error_kind_name(handbook_sdk::SetupErrorKind::RepositoryIdentity),
            "repository_identity"
        );
        let cases = [
            (
                handbook_sdk::SetupErrorReasonCode::RepositoryAuthorityRecoveryBlocked,
                "repository_authority_recovery_blocked",
            ),
            (
                handbook_sdk::SetupErrorReasonCode::RepositoryIdentityUnsafe,
                "repository_identity_unsafe",
            ),
            (
                handbook_sdk::SetupErrorReasonCode::RepositoryIdentityMismatch,
                "repository_identity_mismatch",
            ),
            (
                handbook_sdk::SetupErrorReasonCode::RepositoryIdentityEntropyUnavailable,
                "repository_identity_entropy_unavailable",
            ),
            (
                handbook_sdk::SetupErrorReasonCode::RepositoryIdentityPersistenceFailed,
                "repository_identity_persistence_failed",
            ),
        ];
        for (reason, expected) in cases {
            assert_eq!(setup_error_reason_name(reason), expected);
        }
    }
}
