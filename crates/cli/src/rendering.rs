pub(crate) struct PreparedFlowOutput {
    ready: bool,
    model: handbook_compiler::RenderOutputModel,
}

impl PreparedFlowOutput {
    pub(crate) fn render_markdown(&self) -> String {
        let rendered = render_markdown_output(&self.model);
        if self.ready {
            return rendered;
        }

        let Some(context) = self.model.packet_result.fixture_context.as_ref() else {
            return rendered;
        };

        inject_after_first_three_lines(&rendered, &render_fixture_section_for_demo(context))
    }

    pub(crate) fn render_inspect(&self) -> String {
        let rendered = render_inspect_output(&self.model);
        if self.ready {
            return rendered;
        }

        let Some(context) = self.model.packet_result.fixture_context.as_ref() else {
            return rendered;
        };

        inject_after_first_three_lines(&rendered, &render_fixture_section_for_demo(context))
    }

    pub(crate) fn is_ready(&self) -> bool {
        self.ready
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

fn render_markdown_output(model: &handbook_compiler::RenderOutputModel) -> String {
    let mut output = String::new();

    push_line(&mut output, format!("OUTCOME: {}", render_outcome(model)));
    push_line(&mut output, format!("OBJECT: {}", model.packet_id));
    push_line(
        &mut output,
        format!("NEXT SAFE ACTION: {}", render_next_safe_action(model)),
    );

    match model.refusal.as_ref() {
        Some(refusal) => {
            output.push('\n');
            push_line(&mut output, "## REFUSAL");
            push_line(
                &mut output,
                format!("CATEGORY: {}", render_refusal_category(refusal.category)),
            );
            push_line(&mut output, format!("SUMMARY: {}", refusal.summary));
            push_line(
                &mut output,
                format!(
                    "BROKEN SUBJECT: {}",
                    render_subject_ref(&refusal.broken_subject)
                ),
            );
            push_line(
                &mut output,
                format!(
                    "NEXT SAFE ACTION: {}",
                    render_next_safe_action_value(&refusal.next_safe_action)
                ),
            );
        }
        None if !model.blockers.is_empty() => {
            output.push('\n');
            push_line(&mut output, "## BLOCKERS");
            for (index, blocker) in model.blockers.iter().enumerate() {
                if index > 0 {
                    output.push('\n');
                }
                render_blocker(&mut output, blocker);
            }
        }
        None => {
            if model.packet_result.is_ready() {
                output.push('\n');
                render_markdown_body(&mut output, model);
            }
        }
    }

    output
}

fn render_inspect_output(model: &handbook_compiler::RenderOutputModel) -> String {
    let inspect_model = inspect_model(model);
    let mut output = String::new();

    push_line(
        &mut output,
        format!(
            "OUTCOME: {}",
            render_outcome_from_status(
                inspect_model.packet_status,
                inspect_model.refusal.is_some()
            )
        ),
    );
    push_line(&mut output, format!("OBJECT: {}", inspect_model.packet_id));
    push_line(
        &mut output,
        format!(
            "NEXT SAFE ACTION: {}",
            render_next_safe_action_from_model(
                &inspect_model.packet_result,
                inspect_model.refusal.as_ref(),
                &inspect_model.blockers
            )
        ),
    );

    output.push('\n');
    push_line(&mut output, "## DECISION LOG");
    for (index, entry) in inspect_model.decision_log_entries.iter().enumerate() {
        push_line(&mut output, format!("{}. {}", index + 1, entry));
    }

    output.push('\n');
    push_line(&mut output, "## BUDGET OUTCOME");
    push_line(
        &mut output,
        format!(
            "DISPOSITION: {}",
            render_budget_disposition(inspect_model.budget_outcome.disposition)
        ),
    );
    push_line(
        &mut output,
        format!(
            "REASON: {}",
            render_budget_reason(&inspect_model.budget_outcome.reason)
        ),
    );
    if inspect_model.budget_outcome.targets.is_empty() {
        push_line(&mut output, "TARGETS: NONE");
    } else {
        for (index, target) in inspect_model.budget_outcome.targets.iter().enumerate() {
            push_line(
                &mut output,
                format!(
                    "TARGET {}: {} ({} bytes)",
                    index + 1,
                    target.canonical_repo_relative_path,
                    target.byte_len
                ),
            );
        }
    }
    match render_budget_next_safe_action(inspect_model.budget_outcome.next_safe_action.as_ref()) {
        Some(next_safe_action) => {
            push_line(&mut output, format!("NEXT SAFE ACTION: {next_safe_action}"));
        }
        None => push_line(&mut output, "NEXT SAFE ACTION: NONE"),
    }

    output.push('\n');
    push_line(&mut output, "## REFUSAL");
    match inspect_model.refusal.as_ref() {
        Some(refusal) => {
            push_line(
                &mut output,
                format!("CATEGORY: {}", render_refusal_category(refusal.category)),
            );
            push_line(&mut output, format!("SUMMARY: {}", refusal.summary));
            push_line(
                &mut output,
                format!(
                    "BROKEN SUBJECT: {}",
                    render_subject_ref(&refusal.broken_subject)
                ),
            );
            push_line(
                &mut output,
                format!(
                    "NEXT SAFE ACTION: {}",
                    render_next_safe_action_value(&refusal.next_safe_action)
                ),
            );
        }
        None => push_line(&mut output, "NONE"),
    }

    output.push('\n');
    push_line(&mut output, "## BLOCKERS");
    if inspect_model.blockers.is_empty() {
        push_line(&mut output, "NONE");
    } else {
        for (index, blocker) in inspect_model.blockers.iter().enumerate() {
            if index > 0 {
                output.push('\n');
            }
            push_line(
                &mut output,
                format!("CATEGORY: {}", render_blocker_category(blocker.category)),
            );
            push_line(&mut output, format!("SUMMARY: {}", blocker.summary));
            push_line(
                &mut output,
                format!("SUBJECT: {}", render_subject_ref(&blocker.subject)),
            );
            push_line(
                &mut output,
                format!(
                    "NEXT SAFE ACTION: {}",
                    render_next_safe_action_value(&blocker.next_safe_action)
                ),
            );
        }
    }

    if inspect_model.packet_result.is_ready() {
        output.push('\n');
        push_line(&mut output, "## PACKET OVERVIEW");
        push_line(
            &mut output,
            format!(
                "PACKET VARIANT: {}",
                render_packet_variant(inspect_model.packet_result.variant)
            ),
        );
        push_line(
            &mut output,
            format!(
                "SUMMARY: {}",
                inspect_model.packet_result.decision_summary.summary_line
            ),
        );
        output.push('\n');
        render_packet_body(&mut output, &inspect_model.packet_result);
    }

    output.push('\n');
    push_line(&mut output, "## JSON FALLBACK");
    push_line(
        &mut output,
        handbook_compiler::render_json(&inspect_model).trim_end(),
    );

    output
}

fn inspect_model(
    model: &handbook_compiler::RenderOutputModel,
) -> handbook_compiler::RenderOutputModel {
    let mut inspect_model = model.clone();
    if inspect_model.packet_result.is_ready()
        && inspect_model.refusal.is_none()
        && inspect_model.blockers.is_empty()
    {
        inspect_model
            .packet_result
            .decision_summary
            .ready_next_safe_action = inspect_ready_next_safe_action(&inspect_model);
    }
    inspect_model
}

fn inspect_ready_next_safe_action(
    _model: &handbook_compiler::RenderOutputModel,
) -> handbook_flow::ReadyPacketNextSafeAction {
    handbook_flow::ReadyPacketNextSafeAction::Generate
}

fn render_markdown_body(output: &mut String, model: &handbook_compiler::RenderOutputModel) {
    push_line(output, "## PACKET OVERVIEW");
    push_line(
        output,
        format!(
            "PACKET VARIANT: {}",
            render_packet_variant(model.packet_result.variant)
        ),
    );
    push_line(
        output,
        format!(
            "SUMMARY: {}",
            model.packet_result.decision_summary.summary_line
        ),
    );

    output.push('\n');
    render_packet_body(output, &model.packet_result);
}

fn render_packet_body(output: &mut String, packet: &handbook_flow::PacketResult) {
    if let Some(context) = packet.fixture_context.as_ref() {
        output.push_str(&render_packet_fixture_context(context));
        output.push('\n');
    }

    push_line(output, "## INCLUDED SOURCES");
    if packet.included_sources.is_empty() {
        push_line(output, "NONE");
    } else {
        for (index, source) in packet.included_sources.iter().enumerate() {
            push_line(
                output,
                format!("{}. {}", index + 1, render_packet_source_summary(source)),
            );
        }
    }

    output.push('\n');
    push_line(output, "## OMISSIONS AND BUDGET");
    if packet.notes.is_empty() {
        push_line(output, "NONE");
    } else {
        for (index, note) in packet.notes.iter().enumerate() {
            push_line(
                output,
                format!("{}. {}", index + 1, render_packet_note(note)),
            );
        }
    }

    output.push('\n');
    push_line(output, "## DECISION SUMMARY");
    push_line(
        output,
        format!(
            "STATUS: {}",
            render_packet_status(packet.decision_summary.packet_status)
        ),
    );
    push_line(
        output,
        format!(
            "BUDGET: {}/{}",
            render_budget_disposition(packet.decision_summary.budget_disposition),
            render_budget_reason(&packet.decision_summary.budget_reason)
        ),
    );
    push_line(
        output,
        format!(
            "DECISION LOG ENTRIES: {}",
            packet.decision_summary.decision_log_entries
        ),
    );
    push_line(
        output,
        format!("SUMMARY: {}", packet.decision_summary.summary_line),
    );

    output.push('\n');
    push_line(output, "## PACKET BODY");
    if packet.sections.is_empty() {
        push_line(output, "NONE");
    } else {
        for section in &packet.sections {
            if !output.ends_with('\n') {
                output.push('\n');
            }
            render_packet_section(output, section);
            output.push('\n');
        }
    }
}

fn render_packet_fixture_context(context: &handbook_flow::PacketFixtureContext) -> String {
    let mut output = String::new();
    push_line(&mut output, "## FIXTURE DEMO");
    push_line(&mut output, "MODE: fixture-backed execution demo");
    push_line(
        &mut output,
        format!("FIXTURE SET: {}", context.fixture_set_id),
    );
    push_line(
        &mut output,
        format!("FIXTURE BASIS ROOT: {}", context.fixture_basis_root),
    );
    push_line(&mut output, "FIXTURE LINEAGE:");
    if context.fixture_lineage.is_empty() {
        push_line(&mut output, "NONE");
    } else {
        for (index, source) in context.fixture_lineage.iter().enumerate() {
            push_line(
                &mut output,
                format!("{}. {}", index + 1, render_packet_source_summary(source)),
            );
        }
    }
    output
}

fn render_packet_note(note: &handbook_flow::PacketBodyNote) -> String {
    let kind = match note.kind {
        handbook_flow::PacketBodyNoteKind::Omission => "OMISSION",
        handbook_flow::PacketBodyNoteKind::Budget => "BUDGET",
        handbook_flow::PacketBodyNoteKind::InheritedDependency => "INHERITED DEPENDENCY",
    };

    format!("{kind}: {}", note.text)
}

fn render_packet_section(output: &mut String, section: &handbook_flow::PacketSection) {
    push_line(
        output,
        format!(
            "### {} ({})",
            section.title, section.canonical_repo_relative_path
        ),
    );
    if section.mode == handbook_flow::PacketSectionMode::Summary {
        push_line(output, "MODE: summarized due to budget");
    }
    output.push_str("```text\n");
    output.push_str(&section.contents);
    if !section.contents.ends_with('\n') {
        output.push('\n');
    }
    output.push_str("```\n");
}

fn render_outcome(model: &handbook_compiler::RenderOutputModel) -> &'static str {
    render_outcome_from_status(model.packet_status, model.refusal.is_some())
}

fn render_outcome_from_status(
    packet_status: handbook_flow::PacketSelectionStatus,
    refusal_present: bool,
) -> &'static str {
    if refusal_present {
        return "REFUSED";
    }

    match packet_status {
        handbook_flow::PacketSelectionStatus::Selected => "READY",
        handbook_flow::PacketSelectionStatus::Blocked => "BLOCKED",
    }
}

fn render_next_safe_action(model: &handbook_compiler::RenderOutputModel) -> String {
    render_next_safe_action_from_model(
        &model.packet_result,
        model.refusal.as_ref(),
        &model.blockers,
    )
}

fn render_next_safe_action_from_model(
    packet: &handbook_flow::PacketResult,
    refusal: Option<&handbook_compiler::Refusal>,
    blockers: &[handbook_compiler::Blocker],
) -> String {
    if let Some(refusal) = refusal {
        return render_next_safe_action_value(&refusal.next_safe_action);
    }

    if let Some(blocker) = blockers.first() {
        return render_next_safe_action_value(&blocker.next_safe_action);
    }

    if packet.is_ready() {
        return render_ready_packet_next_safe_action(packet);
    }

    "run `doctor`".to_string()
}

fn render_ready_packet_next_safe_action(packet: &handbook_flow::PacketResult) -> String {
    match packet.decision_summary.ready_next_safe_action {
        handbook_flow::ReadyPacketNextSafeAction::InspectProof => {
            if let Some(context) = packet.fixture_context.as_ref() {
                format!(
                    "run `handbook inspect --packet {} --fixture-set {}` for proof",
                    packet.packet_id, context.fixture_set_id
                )
            } else {
                format!(
                    "run `handbook inspect --packet {}` for proof",
                    packet.packet_id
                )
            }
        }
        handbook_flow::ReadyPacketNextSafeAction::Generate => {
            if let Some(context) = packet.fixture_context.as_ref() {
                format!(
                    "run `handbook generate --packet {} --fixture-set {}`",
                    packet.packet_id, context.fixture_set_id
                )
            } else {
                format!("run `handbook generate --packet {}`", packet.packet_id)
            }
        }
        handbook_flow::ReadyPacketNextSafeAction::RunDoctor => "run `doctor`".to_string(),
    }
}

fn render_next_safe_action_value(action: &handbook_compiler::NextSafeAction) -> String {
    match action {
        handbook_compiler::NextSafeAction::RunSetup => "run `handbook setup`".to_string(),
        handbook_compiler::NextSafeAction::RunSetupInit => "run `handbook setup init`".to_string(),
        handbook_compiler::NextSafeAction::RunSetupRefresh => {
            "run `handbook setup refresh`".to_string()
        }
        handbook_compiler::NextSafeAction::RunAuthorCharter => {
            "run `handbook author charter --from-inputs <path|->`".to_string()
        }
        handbook_compiler::NextSafeAction::RunAuthorProjectContext => {
            "run `handbook author project-context --from-inputs <path|->`".to_string()
        }
        handbook_compiler::NextSafeAction::RunAuthorEnvironmentInventory => {
            "run `handbook author environment-inventory --from-inputs <path|->`".to_string()
        }
        handbook_compiler::NextSafeAction::CreateSystemRoot {
            canonical_repo_relative_path,
        } => format!("create canonical .handbook root at {canonical_repo_relative_path}"),
        handbook_compiler::NextSafeAction::EnsureSystemRootIsDirectory {
            canonical_repo_relative_path,
        } => format!(
            "ensure canonical .handbook root is a directory at {canonical_repo_relative_path}"
        ),
        handbook_compiler::NextSafeAction::RemoveSystemRootSymlink {
            canonical_repo_relative_path,
        } => format!("remove canonical .handbook symlink at {canonical_repo_relative_path}"),
        handbook_compiler::NextSafeAction::CreateCanonicalArtifact {
            canonical_repo_relative_path,
        } => format!("create canonical artifact at {canonical_repo_relative_path}"),
        handbook_compiler::NextSafeAction::FillCanonicalArtifact {
            canonical_repo_relative_path,
        } => format!("fill canonical artifact at {canonical_repo_relative_path}"),
        handbook_compiler::NextSafeAction::ReduceCanonicalArtifactSize {
            canonical_repo_relative_path,
        } => format!("reduce canonical artifact size at {canonical_repo_relative_path}"),
        handbook_compiler::NextSafeAction::RunGenerate { packet_id } => {
            format!("run `handbook generate --packet {packet_id}`")
        }
        handbook_compiler::NextSafeAction::RunDoctor => "run `handbook doctor`".to_string(),
    }
}

fn render_packet_variant(variant: handbook_flow::PacketVariant) -> &'static str {
    variant.as_str()
}

fn render_packet_status(status: handbook_flow::PacketSelectionStatus) -> &'static str {
    match status {
        handbook_flow::PacketSelectionStatus::Selected => "Selected",
        handbook_flow::PacketSelectionStatus::Blocked => "Blocked",
    }
}

fn render_budget_disposition(disposition: handbook_flow::BudgetDisposition) -> &'static str {
    match disposition {
        handbook_flow::BudgetDisposition::Keep => "Keep",
        handbook_flow::BudgetDisposition::Summarize => "Summarize",
        handbook_flow::BudgetDisposition::Exclude => "Exclude",
        handbook_flow::BudgetDisposition::Refuse => "Refuse",
    }
}

fn render_budget_reason(reason: &handbook_flow::BudgetReason) -> &'static str {
    match reason {
        handbook_flow::BudgetReason::WithinBudget => "WithinBudget",
        handbook_flow::BudgetReason::OptionalArtifactTooLarge => "OptionalArtifactTooLarge",
        handbook_flow::BudgetReason::TotalBytesExceeded => "TotalBytesExceeded",
        handbook_flow::BudgetReason::RequiredArtifactTooLarge => "RequiredArtifactTooLarge",
    }
}

fn render_budget_next_safe_action(
    action: Option<&handbook_flow::NextSafeAction>,
) -> Option<String> {
    action.map(|action| match action {
        handbook_flow::NextSafeAction::ReduceCanonicalArtifactSize {
            canonical_repo_relative_path,
        } => format!("reduce canonical artifact size at {canonical_repo_relative_path}"),
    })
}

fn render_refusal_category(category: handbook_compiler::RefusalCategory) -> &'static str {
    match category {
        handbook_compiler::RefusalCategory::NonCanonicalInputAttempt => "NonCanonicalInputAttempt",
        handbook_compiler::RefusalCategory::SystemRootMissing => "SystemRootMissing",
        handbook_compiler::RefusalCategory::SystemRootNotDir => "SystemRootNotDir",
        handbook_compiler::RefusalCategory::SystemRootSymlinkNotAllowed => {
            "SystemRootSymlinkNotAllowed"
        }
        handbook_compiler::RefusalCategory::RequiredArtifactMissing => "RequiredArtifactMissing",
        handbook_compiler::RefusalCategory::RequiredArtifactEmpty => "RequiredArtifactEmpty",
        handbook_compiler::RefusalCategory::RequiredArtifactStarterTemplate => {
            "RequiredArtifactStarterTemplate"
        }
        handbook_compiler::RefusalCategory::RequiredArtifactInvalid => "RequiredArtifactInvalid",
        handbook_compiler::RefusalCategory::ArtifactReadError => "ArtifactReadError",
        handbook_compiler::RefusalCategory::FreshnessInvalid => "FreshnessInvalid",
        handbook_compiler::RefusalCategory::BudgetRefused => "BudgetRefused",
        handbook_compiler::RefusalCategory::UnsupportedRequest => "UnsupportedRequest",
    }
}

fn render_blocker(output: &mut String, blocker: &handbook_compiler::Blocker) {
    push_line(
        output,
        format!("CATEGORY: {}", render_blocker_category(blocker.category)),
    );
    push_line(output, format!("SUMMARY: {}", blocker.summary));
    push_line(
        output,
        format!("SUBJECT: {}", render_subject_ref(&blocker.subject)),
    );
    push_line(
        output,
        format!(
            "NEXT SAFE ACTION: {}",
            render_next_safe_action_value(&blocker.next_safe_action)
        ),
    );
}

fn render_blocker_category(category: handbook_compiler::BlockerCategory) -> &'static str {
    match category {
        handbook_compiler::BlockerCategory::SystemRootMissing => "SystemRootMissing",
        handbook_compiler::BlockerCategory::SystemRootNotDir => "SystemRootNotDir",
        handbook_compiler::BlockerCategory::SystemRootSymlinkNotAllowed => {
            "SystemRootSymlinkNotAllowed"
        }
        handbook_compiler::BlockerCategory::RequiredArtifactMissing => "RequiredArtifactMissing",
        handbook_compiler::BlockerCategory::RequiredArtifactEmpty => "RequiredArtifactEmpty",
        handbook_compiler::BlockerCategory::RequiredArtifactStarterTemplate => {
            "RequiredArtifactStarterTemplate"
        }
        handbook_compiler::BlockerCategory::RequiredArtifactInvalid => "RequiredArtifactInvalid",
        handbook_compiler::BlockerCategory::ArtifactReadError => "ArtifactReadError",
        handbook_compiler::BlockerCategory::FreshnessInvalid => "FreshnessInvalid",
        handbook_compiler::BlockerCategory::BudgetRefused => "BudgetRefused",
        handbook_compiler::BlockerCategory::UnsupportedRequest => "UnsupportedRequest",
    }
}

fn render_subject_ref(subject: &handbook_compiler::SubjectRef) -> String {
    match subject {
        handbook_compiler::SubjectRef::CanonicalArtifact {
            kind,
            canonical_repo_relative_path,
        } => format!(
            "canonical artifact {} at {}",
            render_canonical_artifact_kind(*kind),
            canonical_repo_relative_path
        ),
        handbook_compiler::SubjectRef::InheritedDependency {
            dependency_id,
            version,
        } => match version {
            Some(version) => format!("inherited dependency {dependency_id}@{version}"),
            None => format!("inherited dependency {dependency_id}"),
        },
        handbook_compiler::SubjectRef::Policy { policy_id } => format!("policy {policy_id}"),
    }
}

fn push_line(output: &mut String, line: impl AsRef<str>) {
    output.push_str(line.as_ref());
    output.push('\n');
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
