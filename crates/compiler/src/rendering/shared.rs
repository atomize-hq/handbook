use crate::{
    blocker::Blocker,
    refusal::{NextSafeAction, RefusalCategory, SubjectRef},
    BlockerCategory, CanonicalArtifactKind,
};
use handbook_flow::{
    BudgetDisposition, BudgetReason, NextSafeAction as BudgetNextSafeAction, PacketBodyNote,
    PacketBodyNoteKind, PacketFixtureContext, PacketResult, PacketSection, PacketSectionMode,
    PacketSelectionStatus, PacketSourceSummary, PacketVariant, ReadyPacketNextSafeAction,
};

pub fn push_line(output: &mut String, line: impl AsRef<str>) {
    output.push_str(line.as_ref());
    output.push('\n');
}

pub fn render_outcome(packet_status: PacketSelectionStatus, refusal_present: bool) -> &'static str {
    if refusal_present {
        return "REFUSED";
    }

    match packet_status {
        PacketSelectionStatus::Selected => "READY",
        PacketSelectionStatus::Blocked => "BLOCKED",
    }
}

pub fn render_next_safe_action_from_model(
    packet: &PacketResult,
    refusal: Option<&crate::Refusal>,
    blockers: &[Blocker],
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

pub fn render_ready_packet_next_safe_action(packet: &PacketResult) -> String {
    match packet.decision_summary.ready_next_safe_action {
        ReadyPacketNextSafeAction::InspectProof => {
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
        ReadyPacketNextSafeAction::Generate => {
            if let Some(context) = packet.fixture_context.as_ref() {
                format!(
                    "run `handbook generate --packet {} --fixture-set {}`",
                    packet.packet_id, context.fixture_set_id
                )
            } else {
                format!("run `handbook generate --packet {}`", packet.packet_id)
            }
        }
        ReadyPacketNextSafeAction::RunDoctor => "run `doctor`".to_string(),
    }
}

pub fn render_packet_variant(variant: PacketVariant) -> &'static str {
    variant.as_str()
}

pub fn render_packet_source_summary(summary: &PacketSourceSummary) -> String {
    let presence = match summary.presence {
        crate::ArtifactPresence::Missing => "missing",
        crate::ArtifactPresence::PresentEmpty => "empty",
        crate::ArtifactPresence::PresentNonEmpty => "present",
    };

    let bytes = match summary.byte_len {
        Some(len) => format!("{len} bytes"),
        None => "byte length unavailable".to_string(),
    };

    let hash = summary
        .content_sha256
        .as_ref()
        .map(|value| format!(", sha256={value}"))
        .unwrap_or_default();

    format!(
        "{} [{}] ({presence}, {bytes}{hash})",
        render_canonical_artifact_kind(summary.kind),
        summary.canonical_repo_relative_path
    )
}

pub fn render_packet_note(note: &PacketBodyNote) -> String {
    let kind = match note.kind {
        PacketBodyNoteKind::Omission => "OMISSION",
        PacketBodyNoteKind::Budget => "BUDGET",
        PacketBodyNoteKind::InheritedDependency => "INHERITED DEPENDENCY",
    };

    format!("{kind}: {}", note.text)
}

pub fn render_packet_fixture_context(context: &PacketFixtureContext) -> String {
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

pub fn render_packet_section(output: &mut String, section: &PacketSection) {
    push_line(
        output,
        format!(
            "### {} ({})",
            section.title, section.canonical_repo_relative_path
        ),
    );
    if section.mode == PacketSectionMode::Summary {
        push_line(output, "MODE: summarized due to budget");
    }
    output.push_str("```text\n");
    output.push_str(&section.contents);
    if !section.contents.ends_with('\n') {
        output.push('\n');
    }
    output.push_str("```\n");
}

pub fn render_packet_body(output: &mut String, packet: &PacketResult) {
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

pub fn render_next_safe_action_value(action: &NextSafeAction) -> String {
    recovery_shell::render_next_safe_action_value(action)
}

pub fn render_refusal_category(category: RefusalCategory) -> &'static str {
    match category {
        RefusalCategory::NonCanonicalInputAttempt => "NonCanonicalInputAttempt",
        RefusalCategory::SystemRootMissing => "SystemRootMissing",
        RefusalCategory::SystemRootNotDir => "SystemRootNotDir",
        RefusalCategory::SystemRootSymlinkNotAllowed => "SystemRootSymlinkNotAllowed",
        RefusalCategory::RequiredArtifactMissing => "RequiredArtifactMissing",
        RefusalCategory::RequiredArtifactEmpty => "RequiredArtifactEmpty",
        RefusalCategory::RequiredArtifactStarterTemplate => "RequiredArtifactStarterTemplate",
        RefusalCategory::RequiredArtifactInvalid => "RequiredArtifactInvalid",
        RefusalCategory::ArtifactReadError => "ArtifactReadError",
        RefusalCategory::FreshnessInvalid => "FreshnessInvalid",
        RefusalCategory::BudgetRefused => "BudgetRefused",
        RefusalCategory::UnsupportedRequest => "UnsupportedRequest",
    }
}

pub fn render_blocker_category(category: BlockerCategory) -> &'static str {
    match category {
        BlockerCategory::SystemRootMissing => "SystemRootMissing",
        BlockerCategory::SystemRootNotDir => "SystemRootNotDir",
        BlockerCategory::SystemRootSymlinkNotAllowed => "SystemRootSymlinkNotAllowed",
        BlockerCategory::RequiredArtifactMissing => "RequiredArtifactMissing",
        BlockerCategory::RequiredArtifactEmpty => "RequiredArtifactEmpty",
        BlockerCategory::RequiredArtifactStarterTemplate => "RequiredArtifactStarterTemplate",
        BlockerCategory::RequiredArtifactInvalid => "RequiredArtifactInvalid",
        BlockerCategory::ArtifactReadError => "ArtifactReadError",
        BlockerCategory::FreshnessInvalid => "FreshnessInvalid",
        BlockerCategory::BudgetRefused => "BudgetRefused",
        BlockerCategory::UnsupportedRequest => "UnsupportedRequest",
    }
}

pub fn render_subject_ref(subject: &SubjectRef) -> String {
    recovery_shell::render_subject_ref(subject)
}

pub fn render_canonical_artifact_kind(kind: CanonicalArtifactKind) -> &'static str {
    match kind {
        CanonicalArtifactKind::Charter => "Charter",
        CanonicalArtifactKind::ProjectContext => "ProjectContext",
        CanonicalArtifactKind::EnvironmentInventory => "EnvironmentInventory",
        CanonicalArtifactKind::FeatureSpec => "FeatureSpec",
    }
}

pub fn render_budget_disposition(disposition: BudgetDisposition) -> &'static str {
    match disposition {
        BudgetDisposition::Keep => "Keep",
        BudgetDisposition::Summarize => "Summarize",
        BudgetDisposition::Exclude => "Exclude",
        BudgetDisposition::Refuse => "Refuse",
    }
}

pub fn render_budget_reason(reason: &BudgetReason) -> &'static str {
    match reason {
        BudgetReason::WithinBudget => "WithinBudget",
        BudgetReason::OptionalArtifactTooLarge => "OptionalArtifactTooLarge",
        BudgetReason::TotalBytesExceeded => "TotalBytesExceeded",
        BudgetReason::RequiredArtifactTooLarge => "RequiredArtifactTooLarge",
    }
}

pub fn render_budget_next_safe_action(action: Option<&BudgetNextSafeAction>) -> Option<String> {
    action.map(|action| match action {
        BudgetNextSafeAction::ReduceCanonicalArtifactSize {
            canonical_repo_relative_path,
        } => format!("reduce canonical artifact size at {canonical_repo_relative_path}"),
    })
}

pub fn render_packet_status(status: PacketSelectionStatus) -> &'static str {
    match status {
        PacketSelectionStatus::Selected => "Selected",
        PacketSelectionStatus::Blocked => "Blocked",
    }
}

pub fn json_escape(input: &str) -> String {
    let mut escaped = String::with_capacity(input.len() + 8);
    for character in input.chars() {
        match character {
            '"' => escaped.push_str("\\\""),
            '\\' => escaped.push_str("\\\\"),
            '\n' => escaped.push_str("\\n"),
            '\r' => escaped.push_str("\\r"),
            '\t' => escaped.push_str("\\t"),
            '\u{08}' => escaped.push_str("\\b"),
            '\u{0c}' => escaped.push_str("\\f"),
            c if c.is_control() => {
                use std::fmt::Write;
                write!(&mut escaped, "\\u{:04x}", c as u32).expect("json escape");
            }
            other => escaped.push(other),
        }
    }
    escaped
}

pub fn json_string(input: &str) -> String {
    format!("\"{}\"", json_escape(input))
}

mod recovery_shell {
    use crate::{
        refusal::{NextSafeAction, SubjectRef},
        CanonicalArtifactKind,
    };

    pub(super) fn render_next_safe_action_value(action: &NextSafeAction) -> String {
        match action {
            NextSafeAction::RunSetup => "run `handbook setup`".to_string(),
            NextSafeAction::RunSetupInit => "run `handbook setup init`".to_string(),
            NextSafeAction::RunSetupRefresh => "run `handbook setup refresh`".to_string(),
            NextSafeAction::RunAuthorCharter => {
                "run `handbook author charter --from-inputs <path|->`".to_string()
            }
            NextSafeAction::RunAuthorProjectContext => {
                "run `handbook author project-context --from-inputs <path|->`".to_string()
            }
            NextSafeAction::RunAuthorEnvironmentInventory => {
                "run `handbook author environment-inventory --from-inputs <path|->`".to_string()
            }
            NextSafeAction::CreateSystemRoot {
                canonical_repo_relative_path,
            } => format!("create canonical .handbook root at {canonical_repo_relative_path}"),
            NextSafeAction::EnsureSystemRootIsDirectory {
                canonical_repo_relative_path,
            } => format!(
                "ensure canonical .handbook root is a directory at {canonical_repo_relative_path}"
            ),
            NextSafeAction::RemoveSystemRootSymlink {
                canonical_repo_relative_path,
            } => format!("remove canonical .handbook symlink at {canonical_repo_relative_path}"),
            NextSafeAction::CreateCanonicalArtifact {
                canonical_repo_relative_path,
            } => format!("create canonical artifact at {canonical_repo_relative_path}"),
            NextSafeAction::FillCanonicalArtifact {
                canonical_repo_relative_path,
            } => format!("fill canonical artifact at {canonical_repo_relative_path}"),
            NextSafeAction::ReduceCanonicalArtifactSize {
                canonical_repo_relative_path,
            } => format!("reduce canonical artifact size at {canonical_repo_relative_path}"),
            NextSafeAction::RunGenerate { packet_id } => {
                format!("run `handbook generate --packet {packet_id}`")
            }
            NextSafeAction::RunDoctor => "run `handbook doctor`".to_string(),
        }
    }

    pub(super) fn render_subject_ref(subject: &SubjectRef) -> String {
        match subject {
            SubjectRef::CanonicalArtifact {
                kind,
                canonical_repo_relative_path,
            } => format!(
                "canonical artifact {} at {}",
                render_canonical_artifact_kind(*kind),
                canonical_repo_relative_path
            ),
            SubjectRef::InheritedDependency {
                dependency_id,
                version,
            } => match version {
                Some(version) => format!("inherited dependency {dependency_id}@{version}"),
                None => format!("inherited dependency {dependency_id}"),
            },
            SubjectRef::Policy { policy_id } => format!("policy {policy_id}"),
        }
    }

    fn render_canonical_artifact_kind(kind: CanonicalArtifactKind) -> &'static str {
        match kind {
            CanonicalArtifactKind::Charter => "Charter",
            CanonicalArtifactKind::ProjectContext => "ProjectContext",
            CanonicalArtifactKind::EnvironmentInventory => "EnvironmentInventory",
            CanonicalArtifactKind::FeatureSpec => "FeatureSpec",
        }
    }
}
