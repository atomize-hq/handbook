use super::model::RenderOutputModel;
use crate::{
    blocker::Blocker,
    refusal::{NextSafeAction, RefusalCategory, SubjectRef},
    BlockerCategory,
};
use handbook_flow::PacketSelectionStatus;

pub fn render_markdown(model: &RenderOutputModel) -> String {
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
            push_line(&mut output, "## REFUSAL".to_string());
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
            push_line(&mut output, "## BLOCKERS".to_string());
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
                render_body(&mut output, model);
            }
        }
    }

    output
}

fn render_outcome(model: &RenderOutputModel) -> &'static str {
    if model.refusal.is_some() {
        return "REFUSED";
    }

    match model.packet_status {
        PacketSelectionStatus::Selected => "READY",
        PacketSelectionStatus::Blocked => "BLOCKED",
    }
}

fn render_next_safe_action(model: &RenderOutputModel) -> String {
    if let Some(refusal) = model.refusal.as_ref() {
        return render_next_safe_action_value(&refusal.next_safe_action);
    }

    if let Some(blocker) = model.blockers.first() {
        return render_next_safe_action_value(&blocker.next_safe_action);
    }

    super::shared::render_next_safe_action_from_model(&model.packet_result, None, &[])
}

fn render_body(output: &mut String, model: &RenderOutputModel) {
    push_line(output, "## PACKET OVERVIEW".to_string());
    push_line(
        output,
        format!(
            "PACKET VARIANT: {}",
            super::shared::render_packet_variant(model.packet_result.variant)
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
    super::shared::render_packet_body(output, &model.packet_result);
}

fn render_refusal_category(category: RefusalCategory) -> &'static str {
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

fn render_blocker(output: &mut String, blocker: &Blocker) {
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

fn render_blocker_category(category: BlockerCategory) -> &'static str {
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

fn render_next_safe_action_value(action: &NextSafeAction) -> String {
    super::shared::render_next_safe_action_value(action)
}

fn render_subject_ref(subject: &SubjectRef) -> String {
    match subject {
        SubjectRef::CanonicalArtifact {
            label,
            canonical_repo_relative_path,
            ..
        } => format!(
            "canonical artifact {} at {}",
            label, canonical_repo_relative_path
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

fn push_line(output: &mut String, line: String) {
    output.push_str(&line);
    output.push('\n');
}
