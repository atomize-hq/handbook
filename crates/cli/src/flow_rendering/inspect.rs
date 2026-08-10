use super::json::render_json;
use super::model::RenderOutputModel;
use super::shared::{
    push_line, render_blocker_category, render_budget_disposition, render_budget_next_safe_action,
    render_budget_reason, render_next_safe_action_from_model, render_outcome,
    render_refusal_category, render_subject_ref,
};
use handbook_sdk::ReadyPacketNextSafeAction;

pub fn render_inspect(model: &RenderOutputModel) -> String {
    let inspect_model = inspect_model(model);
    let mut output = String::new();

    push_line(
        &mut output,
        format!(
            "OUTCOME: {}",
            render_outcome(inspect_model.packet_status, inspect_model.refusal.is_some())
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
        Some(next_safe_action) => push_line(
            &mut output,
            format!("NEXT SAFE ACTION: {}", next_safe_action),
        ),
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
                    super::shared::render_next_safe_action_value(&refusal.next_safe_action)
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
                    super::shared::render_next_safe_action_value(&blocker.next_safe_action)
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
                super::shared::render_packet_variant(inspect_model.packet_result.variant)
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
        super::shared::render_packet_body(&mut output, &inspect_model.packet_result);
    }

    output.push('\n');
    push_line(&mut output, "## JSON FALLBACK");
    push_line(&mut output, render_json(&inspect_model).trim_end());

    output
}

fn inspect_model(model: &RenderOutputModel) -> RenderOutputModel {
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

fn inspect_ready_next_safe_action(_model: &RenderOutputModel) -> ReadyPacketNextSafeAction {
    ReadyPacketNextSafeAction::Generate
}
