use std::process::ExitCode;

pub(crate) struct PreparedFlowOutput {
    ready: bool,
    model: handbook_compiler::RenderOutputModel,
}

impl PreparedFlowOutput {
    pub(crate) fn render_markdown(&self) -> String {
        handbook_compiler::render_markdown(&self.model)
    }

    pub(crate) fn render_inspect(&self) -> String {
        let rendered = handbook_compiler::render_inspect(&self.model);
        if self.ready {
            return rendered;
        }

        let Some(context) = self.model.packet_result.fixture_context.as_ref() else {
            return rendered;
        };

        inject_after_first_three_lines(&rendered, &render_fixture_section_for_demo(context))
    }

    pub(crate) fn exit_code(&self) -> ExitCode {
        if self.ready {
            ExitCode::SUCCESS
        } else {
            ExitCode::from(1)
        }
    }
}

fn render_fixture_section_for_demo(context: &handbook_flow::PacketFixtureContext) -> String {
    let mut out = String::new();
    out.push_str("MODE: fixture-backed execution demo\n");
    out.push_str("## FIXTURE DEMO\n");
    out.push_str(&format!("FIXTURE SET: {}\n", context.fixture_set_id));
    out.push_str(&format!(
        "FIXTURE BASIS ROOT: {}\n",
        context.fixture_basis_root
    ));
    out.push_str("FIXTURE LINEAGE:\n");
    if context.fixture_lineage.is_empty() {
        out.push_str("NONE\n");
    } else {
        for (index, item) in context.fixture_lineage.iter().enumerate() {
            out.push_str(&format!(
                "{}. {}\n",
                index + 1,
                render_packet_source_summary(item)
            ));
        }
    }
    out
}

fn render_packet_source_summary(source: &handbook_flow::PacketSourceSummary) -> String {
    let presence = match source.presence {
        handbook_engine::ArtifactPresence::Missing => "missing",
        handbook_engine::ArtifactPresence::PresentEmpty => "empty",
        handbook_engine::ArtifactPresence::PresentNonEmpty => "present",
    };

    let bytes = match source.byte_len {
        Some(len) => format!("{len} bytes"),
        None => "byte length unavailable".to_string(),
    };

    let hash = source
        .content_sha256
        .as_ref()
        .map(|value| format!(", sha256={value}"))
        .unwrap_or_default();

    format!(
        "{} [{}] ({presence}, {bytes}{hash})",
        render_canonical_artifact_kind(source.kind),
        source.canonical_repo_relative_path
    )
}

fn render_canonical_artifact_kind(kind: handbook_engine::CanonicalArtifactKind) -> &'static str {
    match kind {
        handbook_engine::CanonicalArtifactKind::Charter => "Charter",
        handbook_engine::CanonicalArtifactKind::ProjectContext => "ProjectContext",
        handbook_engine::CanonicalArtifactKind::EnvironmentInventory => "EnvironmentInventory",
        handbook_engine::CanonicalArtifactKind::FeatureSpec => "FeatureSpec",
    }
}

fn inject_after_first_three_lines(rendered: &str, injection: &str) -> String {
    let mut lines: Vec<&str> = rendered.split('\n').collect();
    let insert_at = 3.min(lines.len());
    lines.insert(insert_at, injection.trim_end_matches('\n'));
    lines.join("\n")
}

pub(crate) fn prepare_flow_output(
    result: handbook_flow::ResolverResult,
) -> Result<PreparedFlowOutput, String> {
    let ready = result.selection.status == handbook_flow::PacketSelectionStatus::Selected
        && result.refusal.is_none()
        && result.blockers.is_empty();

    let compiler_result = flow_result_for_rendering(result);
    let model =
        handbook_compiler::build_output_model(&compiler_result).map_err(|err| format!("{err}"))?;

    Ok(PreparedFlowOutput { ready, model })
}

fn flow_result_for_rendering(
    result: handbook_flow::ResolverResult,
) -> handbook_compiler::ResolverResult {
    handbook_compiler::ResolverResult {
        c04_result_version: result.c04_result_version,
        c03_schema_version: result.c03_schema_version,
        c03_manifest_generation_version: result.c03_manifest_generation_version,
        c03_fingerprint_sha256: result.c03_fingerprint_sha256,
        packet_result: result.packet_result,
        decision_log: handbook_compiler::DecisionLog {
            entries: result.decision_log_entries,
        },
        budget_outcome: result.budget_outcome,
        selection: result.selection,
        refusal: result.refusal.map(flow_refusal_for_rendering),
        blockers: result
            .blockers
            .into_iter()
            .map(flow_blocker_for_rendering)
            .collect(),
    }
}

fn flow_refusal_for_rendering(
    refusal: handbook_flow::ResolverRefusal,
) -> handbook_compiler::Refusal {
    handbook_compiler::Refusal {
        category: flow_refusal_category_for_rendering(refusal.category),
        summary: refusal.summary,
        broken_subject: flow_subject_ref_for_rendering(refusal.broken_subject),
        next_safe_action: flow_next_safe_action_for_rendering(refusal.next_safe_action),
    }
}

fn flow_blocker_for_rendering(
    blocker: handbook_flow::ResolverBlocker,
) -> handbook_compiler::Blocker {
    handbook_compiler::Blocker {
        category: flow_blocker_category_for_rendering(blocker.category),
        subject: flow_subject_ref_for_rendering(blocker.subject),
        summary: blocker.summary,
        next_safe_action: flow_next_safe_action_for_rendering(blocker.next_safe_action),
    }
}

fn flow_refusal_category_for_rendering(
    category: handbook_flow::ResolverRefusalCategory,
) -> handbook_compiler::RefusalCategory {
    match category {
        handbook_flow::ResolverRefusalCategory::NonCanonicalInputAttempt => {
            handbook_compiler::RefusalCategory::NonCanonicalInputAttempt
        }
        handbook_flow::ResolverRefusalCategory::SystemRootMissing => {
            handbook_compiler::RefusalCategory::SystemRootMissing
        }
        handbook_flow::ResolverRefusalCategory::SystemRootNotDir => {
            handbook_compiler::RefusalCategory::SystemRootNotDir
        }
        handbook_flow::ResolverRefusalCategory::SystemRootSymlinkNotAllowed => {
            handbook_compiler::RefusalCategory::SystemRootSymlinkNotAllowed
        }
        handbook_flow::ResolverRefusalCategory::RequiredArtifactMissing => {
            handbook_compiler::RefusalCategory::RequiredArtifactMissing
        }
        handbook_flow::ResolverRefusalCategory::RequiredArtifactEmpty => {
            handbook_compiler::RefusalCategory::RequiredArtifactEmpty
        }
        handbook_flow::ResolverRefusalCategory::RequiredArtifactStarterTemplate => {
            handbook_compiler::RefusalCategory::RequiredArtifactStarterTemplate
        }
        handbook_flow::ResolverRefusalCategory::RequiredArtifactInvalid => {
            handbook_compiler::RefusalCategory::RequiredArtifactInvalid
        }
        handbook_flow::ResolverRefusalCategory::ArtifactReadError => {
            handbook_compiler::RefusalCategory::ArtifactReadError
        }
        handbook_flow::ResolverRefusalCategory::FreshnessInvalid => {
            handbook_compiler::RefusalCategory::FreshnessInvalid
        }
        handbook_flow::ResolverRefusalCategory::BudgetRefused => {
            handbook_compiler::RefusalCategory::BudgetRefused
        }
        handbook_flow::ResolverRefusalCategory::UnsupportedRequest => {
            handbook_compiler::RefusalCategory::UnsupportedRequest
        }
    }
}

fn flow_blocker_category_for_rendering(
    category: handbook_flow::ResolverBlockerCategory,
) -> handbook_compiler::BlockerCategory {
    match category {
        handbook_flow::ResolverBlockerCategory::SystemRootMissing => {
            handbook_compiler::BlockerCategory::SystemRootMissing
        }
        handbook_flow::ResolverBlockerCategory::SystemRootNotDir => {
            handbook_compiler::BlockerCategory::SystemRootNotDir
        }
        handbook_flow::ResolverBlockerCategory::SystemRootSymlinkNotAllowed => {
            handbook_compiler::BlockerCategory::SystemRootSymlinkNotAllowed
        }
        handbook_flow::ResolverBlockerCategory::RequiredArtifactMissing => {
            handbook_compiler::BlockerCategory::RequiredArtifactMissing
        }
        handbook_flow::ResolverBlockerCategory::RequiredArtifactEmpty => {
            handbook_compiler::BlockerCategory::RequiredArtifactEmpty
        }
        handbook_flow::ResolverBlockerCategory::RequiredArtifactStarterTemplate => {
            handbook_compiler::BlockerCategory::RequiredArtifactStarterTemplate
        }
        handbook_flow::ResolverBlockerCategory::RequiredArtifactInvalid => {
            handbook_compiler::BlockerCategory::RequiredArtifactInvalid
        }
        handbook_flow::ResolverBlockerCategory::ArtifactReadError => {
            handbook_compiler::BlockerCategory::ArtifactReadError
        }
        handbook_flow::ResolverBlockerCategory::FreshnessInvalid => {
            handbook_compiler::BlockerCategory::FreshnessInvalid
        }
        handbook_flow::ResolverBlockerCategory::BudgetRefused => {
            handbook_compiler::BlockerCategory::BudgetRefused
        }
        handbook_flow::ResolverBlockerCategory::UnsupportedRequest => {
            handbook_compiler::BlockerCategory::UnsupportedRequest
        }
    }
}

fn flow_subject_ref_for_rendering(
    subject: handbook_flow::ResolverSubjectRef,
) -> handbook_compiler::SubjectRef {
    match subject {
        handbook_flow::ResolverSubjectRef::CanonicalArtifact {
            kind,
            canonical_repo_relative_path,
        } => handbook_compiler::SubjectRef::CanonicalArtifact {
            kind,
            canonical_repo_relative_path,
        },
        handbook_flow::ResolverSubjectRef::InheritedDependency {
            dependency_id,
            version,
        } => handbook_compiler::SubjectRef::InheritedDependency {
            dependency_id,
            version,
        },
        handbook_flow::ResolverSubjectRef::Policy { policy_id } => {
            handbook_compiler::SubjectRef::Policy { policy_id }
        }
    }
}

fn flow_next_safe_action_for_rendering(
    action: handbook_flow::ResolverNextSafeAction,
) -> handbook_compiler::NextSafeAction {
    match action {
        handbook_flow::ResolverNextSafeAction::RunSetup => {
            handbook_compiler::NextSafeAction::RunSetup
        }
        handbook_flow::ResolverNextSafeAction::RunSetupInit => {
            handbook_compiler::NextSafeAction::RunSetupInit
        }
        handbook_flow::ResolverNextSafeAction::RunSetupRefresh => {
            handbook_compiler::NextSafeAction::RunSetupRefresh
        }
        handbook_flow::ResolverNextSafeAction::RunAuthorCharter => {
            handbook_compiler::NextSafeAction::RunAuthorCharter
        }
        handbook_flow::ResolverNextSafeAction::RunAuthorProjectContext => {
            handbook_compiler::NextSafeAction::RunAuthorProjectContext
        }
        handbook_flow::ResolverNextSafeAction::RunAuthorEnvironmentInventory => {
            handbook_compiler::NextSafeAction::RunAuthorEnvironmentInventory
        }
        handbook_flow::ResolverNextSafeAction::CreateSystemRoot {
            canonical_repo_relative_path,
        } => handbook_compiler::NextSafeAction::CreateSystemRoot {
            canonical_repo_relative_path,
        },
        handbook_flow::ResolverNextSafeAction::EnsureSystemRootIsDirectory {
            canonical_repo_relative_path,
        } => handbook_compiler::NextSafeAction::EnsureSystemRootIsDirectory {
            canonical_repo_relative_path,
        },
        handbook_flow::ResolverNextSafeAction::RemoveSystemRootSymlink {
            canonical_repo_relative_path,
        } => handbook_compiler::NextSafeAction::RemoveSystemRootSymlink {
            canonical_repo_relative_path,
        },
        handbook_flow::ResolverNextSafeAction::CreateCanonicalArtifact {
            canonical_repo_relative_path,
        } => handbook_compiler::NextSafeAction::CreateCanonicalArtifact {
            canonical_repo_relative_path,
        },
        handbook_flow::ResolverNextSafeAction::FillCanonicalArtifact {
            canonical_repo_relative_path,
        } => handbook_compiler::NextSafeAction::FillCanonicalArtifact {
            canonical_repo_relative_path,
        },
        handbook_flow::ResolverNextSafeAction::ReduceCanonicalArtifactSize {
            canonical_repo_relative_path,
        } => handbook_compiler::NextSafeAction::ReduceCanonicalArtifactSize {
            canonical_repo_relative_path,
        },
        handbook_flow::ResolverNextSafeAction::RunGenerate { packet_id } => {
            handbook_compiler::NextSafeAction::RunGenerate { packet_id }
        }
        handbook_flow::ResolverNextSafeAction::RunDoctor => {
            handbook_compiler::NextSafeAction::RunDoctor
        }
    }
}
