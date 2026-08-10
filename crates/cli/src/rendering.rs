use crate::flow_rendering;

pub(crate) struct PreparedFlowOutput {
    ready: bool,
    model: flow_rendering::RenderOutputModel,
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

fn render_fixture_section_for_demo(context: &handbook_sdk::PacketFixtureContext) -> String {
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

fn render_packet_source_summary(source: &handbook_sdk::PacketSourceSummary) -> String {
    let presence = match source.presence {
        handbook_sdk::ArtifactPresence::Missing => "missing",
        handbook_sdk::ArtifactPresence::PresentEmpty => "empty",
        handbook_sdk::ArtifactPresence::PresentNonEmpty => "present",
    };

    if let (
        Some(source_byte_len),
        Some(source_sha256),
        Some(rendered_byte_len),
        Some(rendered_sha256),
        Some(media_type),
    ) = (
        source.byte_len,
        source.content_sha256.as_deref(),
        source.rendered_output_byte_len,
        source.rendered_output_sha256.as_deref(),
        source.rendered_media_type.as_deref(),
    ) {
        return format!(
            "{} [{}] ({presence}, {source_byte_len} source bytes, source_sha256=sha256:{source_sha256}, {rendered_byte_len} rendered bytes, rendered_sha256={rendered_sha256}, media_type={media_type})",
            source.label,
            source.canonical_repo_relative_path
        );
    }

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
        source.label, source.canonical_repo_relative_path
    )
}

fn inject_after_first_three_lines(rendered: &str, injection: &str) -> String {
    let mut lines: Vec<&str> = rendered.split('\n').collect();
    let insert_at = 3.min(lines.len());
    lines.insert(insert_at, injection.trim_end_matches('\n'));
    lines.join("\n")
}

fn render_markdown_output(model: &flow_rendering::RenderOutputModel) -> String {
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

fn render_inspect_output(model: &flow_rendering::RenderOutputModel) -> String {
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
                    "TARGET {}: {} ({} bytes [{}])",
                    index + 1,
                    target.canonical_repo_relative_path,
                    target.byte_len,
                    match target.byte_domain {
                        handbook_sdk::BudgetByteDomain::Source => "source",
                        handbook_sdk::BudgetByteDomain::RenderedOutput => "rendered_output",
                    }
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
        flow_rendering::render_json(&inspect_model).trim_end(),
    );

    output
}

fn inspect_model(model: &flow_rendering::RenderOutputModel) -> flow_rendering::RenderOutputModel {
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
    _model: &flow_rendering::RenderOutputModel,
) -> handbook_sdk::ReadyPacketNextSafeAction {
    handbook_sdk::ReadyPacketNextSafeAction::Generate
}

fn render_markdown_body(output: &mut String, model: &flow_rendering::RenderOutputModel) {
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

fn render_packet_body(output: &mut String, packet: &handbook_sdk::PacketResult) {
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

fn render_packet_fixture_context(context: &handbook_sdk::PacketFixtureContext) -> String {
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

fn render_packet_note(note: &handbook_sdk::PacketBodyNote) -> String {
    let kind = match note.kind {
        handbook_sdk::PacketBodyNoteKind::Omission => "OMISSION",
        handbook_sdk::PacketBodyNoteKind::Budget => "BUDGET",
        handbook_sdk::PacketBodyNoteKind::InheritedDependency => "INHERITED DEPENDENCY",
    };

    format!("{kind}: {}", note.text)
}

fn render_packet_section(output: &mut String, section: &handbook_sdk::PacketSection) {
    push_line(
        output,
        format!(
            "### {} ({})",
            section.title, section.canonical_repo_relative_path
        ),
    );
    if section.mode == handbook_sdk::PacketSectionMode::Summary {
        push_line(output, "MODE: summarized due to budget");
    } else if section.mode == handbook_sdk::PacketSectionMode::Rendered {
        push_line(output, "MODE: rendered from selected canonical YAML");
        push_line(
            output,
            format!(
                "SOURCE SHA256: {}",
                section
                    .source_content_sha256
                    .as_deref()
                    .expect("rendered section retains source fingerprint")
            ),
        );
        push_line(
            output,
            format!(
                "RENDERED SHA256: {}",
                section
                    .rendered_output_sha256
                    .as_deref()
                    .expect("rendered section retains output fingerprint")
            ),
        );
    }
    output.push_str("```text\n");
    output.push_str(&section.contents);
    if !section.contents.ends_with('\n') {
        output.push('\n');
    }
    output.push_str("```\n");
}

fn render_outcome(model: &flow_rendering::RenderOutputModel) -> &'static str {
    render_outcome_from_status(model.packet_status, model.refusal.is_some())
}

fn render_outcome_from_status(
    packet_status: handbook_sdk::PacketSelectionStatus,
    refusal_present: bool,
) -> &'static str {
    if refusal_present {
        return "REFUSED";
    }

    match packet_status {
        handbook_sdk::PacketSelectionStatus::Selected => "READY",
        handbook_sdk::PacketSelectionStatus::Blocked => "BLOCKED",
    }
}

fn render_next_safe_action(model: &flow_rendering::RenderOutputModel) -> String {
    render_next_safe_action_from_model(
        &model.packet_result,
        model.refusal.as_ref(),
        &model.blockers,
    )
}

fn render_next_safe_action_from_model(
    packet: &handbook_sdk::PacketResult,
    refusal: Option<&handbook_sdk::Refusal>,
    blockers: &[handbook_sdk::Blocker],
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

fn render_ready_packet_next_safe_action(packet: &handbook_sdk::PacketResult) -> String {
    match packet.decision_summary.ready_next_safe_action {
        handbook_sdk::ReadyPacketNextSafeAction::InspectProof => {
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
        handbook_sdk::ReadyPacketNextSafeAction::Generate => {
            if let Some(context) = packet.fixture_context.as_ref() {
                format!(
                    "run `handbook generate --packet {} --fixture-set {}`",
                    packet.packet_id, context.fixture_set_id
                )
            } else {
                format!("run `handbook generate --packet {}`", packet.packet_id)
            }
        }
        handbook_sdk::ReadyPacketNextSafeAction::RunDoctor => "run `doctor`".to_string(),
    }
}

fn render_next_safe_action_value(action: &handbook_sdk::NextSafeAction) -> String {
    match action {
        handbook_sdk::NextSafeAction::RunSetup => "run `handbook setup`".to_string(),
        handbook_sdk::NextSafeAction::RunSetupInit => "run `handbook setup init`".to_string(),
        handbook_sdk::NextSafeAction::RunSetupRefresh => "run `handbook setup refresh`".to_string(),
        handbook_sdk::NextSafeAction::RunAuthorCharter => {
            "run `handbook author charter --from-inputs <path|->`".to_string()
        }
        handbook_sdk::NextSafeAction::RunAuthorProjectContext => {
            "run `handbook author project-context --from-inputs <path|->`".to_string()
        }
        handbook_sdk::NextSafeAction::CreateSystemRoot {
            canonical_repo_relative_path,
        } => format!("create canonical .handbook root at {canonical_repo_relative_path}"),
        handbook_sdk::NextSafeAction::EnsureSystemRootIsDirectory {
            canonical_repo_relative_path,
        } => format!(
            "ensure canonical .handbook root is a directory at {canonical_repo_relative_path}"
        ),
        handbook_sdk::NextSafeAction::RemoveSystemRootSymlink {
            canonical_repo_relative_path,
        } => format!("remove canonical .handbook symlink at {canonical_repo_relative_path}"),
        handbook_sdk::NextSafeAction::CreateCanonicalArtifact {
            canonical_repo_relative_path,
        } => format!("create canonical artifact at {canonical_repo_relative_path}"),
        handbook_sdk::NextSafeAction::FillCanonicalArtifact {
            canonical_repo_relative_path,
        } => format!("fill canonical artifact at {canonical_repo_relative_path}"),
        handbook_sdk::NextSafeAction::ReduceCanonicalArtifactSize {
            canonical_repo_relative_path,
        } => format!("reduce canonical artifact size at {canonical_repo_relative_path}"),
        handbook_sdk::NextSafeAction::RunGenerate { packet_id } => {
            format!("run `handbook generate --packet {packet_id}`")
        }
        handbook_sdk::NextSafeAction::RunDoctor => "run `handbook doctor`".to_string(),
    }
}

fn render_packet_variant(variant: handbook_sdk::PacketVariant) -> &'static str {
    variant.as_str()
}

fn render_packet_status(status: handbook_sdk::PacketSelectionStatus) -> &'static str {
    match status {
        handbook_sdk::PacketSelectionStatus::Selected => "Selected",
        handbook_sdk::PacketSelectionStatus::Blocked => "Blocked",
    }
}

fn render_budget_disposition(disposition: handbook_sdk::BudgetDisposition) -> &'static str {
    match disposition {
        handbook_sdk::BudgetDisposition::Keep => "Keep",
        handbook_sdk::BudgetDisposition::Summarize => "Summarize",
        handbook_sdk::BudgetDisposition::Exclude => "Exclude",
        handbook_sdk::BudgetDisposition::Refuse => "Refuse",
    }
}

fn render_budget_reason(reason: &handbook_sdk::BudgetReason) -> &'static str {
    match reason {
        handbook_sdk::BudgetReason::WithinBudget => "WithinBudget",
        handbook_sdk::BudgetReason::OptionalArtifactTooLarge => "OptionalArtifactTooLarge",
        handbook_sdk::BudgetReason::TotalBytesExceeded => "TotalBytesExceeded",
        handbook_sdk::BudgetReason::RequiredArtifactTooLarge => "RequiredArtifactTooLarge",
    }
}

fn render_budget_next_safe_action(
    action: Option<&handbook_sdk::BudgetNextSafeAction>,
) -> Option<String> {
    action.map(|action| match action {
        handbook_sdk::BudgetNextSafeAction::ReduceCanonicalArtifactSize {
            canonical_repo_relative_path,
        } => format!("reduce canonical artifact size at {canonical_repo_relative_path}"),
    })
}

fn render_refusal_category(category: handbook_sdk::RefusalCategory) -> &'static str {
    match category {
        handbook_sdk::RefusalCategory::NonCanonicalInputAttempt => "NonCanonicalInputAttempt",
        handbook_sdk::RefusalCategory::SystemRootMissing => "SystemRootMissing",
        handbook_sdk::RefusalCategory::SystemRootNotDir => "SystemRootNotDir",
        handbook_sdk::RefusalCategory::SystemRootSymlinkNotAllowed => "SystemRootSymlinkNotAllowed",
        handbook_sdk::RefusalCategory::RequiredArtifactMissing => "RequiredArtifactMissing",
        handbook_sdk::RefusalCategory::RequiredArtifactEmpty => "RequiredArtifactEmpty",
        handbook_sdk::RefusalCategory::RequiredArtifactStarterTemplate => {
            "RequiredArtifactStarterTemplate"
        }
        handbook_sdk::RefusalCategory::RequiredArtifactInvalid => "RequiredArtifactInvalid",
        handbook_sdk::RefusalCategory::ArtifactReadError => "ArtifactReadError",
        handbook_sdk::RefusalCategory::FreshnessInvalid => "FreshnessInvalid",
        handbook_sdk::RefusalCategory::BudgetRefused => "BudgetRefused",
        handbook_sdk::RefusalCategory::UnsupportedRequest => "UnsupportedRequest",
    }
}

fn render_blocker(output: &mut String, blocker: &handbook_sdk::Blocker) {
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

fn render_blocker_category(category: handbook_sdk::BlockerCategory) -> &'static str {
    match category {
        handbook_sdk::BlockerCategory::SystemRootMissing => "SystemRootMissing",
        handbook_sdk::BlockerCategory::SystemRootNotDir => "SystemRootNotDir",
        handbook_sdk::BlockerCategory::SystemRootSymlinkNotAllowed => "SystemRootSymlinkNotAllowed",
        handbook_sdk::BlockerCategory::RequiredArtifactMissing => "RequiredArtifactMissing",
        handbook_sdk::BlockerCategory::RequiredArtifactEmpty => "RequiredArtifactEmpty",
        handbook_sdk::BlockerCategory::RequiredArtifactStarterTemplate => {
            "RequiredArtifactStarterTemplate"
        }
        handbook_sdk::BlockerCategory::RequiredArtifactInvalid => "RequiredArtifactInvalid",
        handbook_sdk::BlockerCategory::ArtifactReadError => "ArtifactReadError",
        handbook_sdk::BlockerCategory::FreshnessInvalid => "FreshnessInvalid",
        handbook_sdk::BlockerCategory::BudgetRefused => "BudgetRefused",
        handbook_sdk::BlockerCategory::UnsupportedRequest => "UnsupportedRequest",
    }
}

fn render_subject_ref(subject: &handbook_sdk::SubjectRef) -> String {
    match subject {
        handbook_sdk::SubjectRef::CanonicalArtifact {
            label,
            canonical_repo_relative_path,
            ..
        } => format!(
            "canonical artifact {} at {}",
            label, canonical_repo_relative_path
        ),
        handbook_sdk::SubjectRef::InheritedDependency {
            dependency_id,
            version,
        } => match version {
            Some(version) => format!("inherited dependency {dependency_id}@{version}"),
            None => format!("inherited dependency {dependency_id}"),
        },
        handbook_sdk::SubjectRef::Policy { policy_id } => format!("policy {policy_id}"),
    }
}

fn push_line(output: &mut String, line: impl AsRef<str>) {
    output.push_str(line.as_ref());
    output.push('\n');
}

pub(crate) fn prepare_flow_output(
    result: handbook_sdk::flow_api::PacketResolutionView,
) -> Result<PreparedFlowOutput, String> {
    let ready = result.selection.status == handbook_sdk::PacketSelectionStatus::Selected
        && result.refusal.is_none()
        && result.blockers.is_empty();

    let model = flow_rendering::build_output_model(&result).map_err(|err| format!("{err}"))?;

    Ok(PreparedFlowOutput { ready, model })
}
