use super::model::RenderOutputModel;
use super::shared::{
    json_string, render_blocker_category, render_budget_disposition,
    render_budget_next_safe_action, render_budget_reason, render_packet_status,
    render_packet_variant, render_ready_packet_next_safe_action, render_refusal_category,
};
use handbook_sdk::{ArtifactPresence, Blocker, Refusal, SubjectRef};
use handbook_sdk::{
    PacketBodyNote, PacketBodyNoteKind, PacketFixtureContext, PacketResult, PacketSection,
    PacketSectionMode, PacketSourceSummary,
};
use std::fmt::Write;

pub fn render_json(model: &RenderOutputModel) -> String {
    let mut output = String::new();
    output.push_str("{\n");
    write_line(
        &mut output,
        1,
        "\"c04_result_version\":",
        &json_string(&model.c04_result_version),
        true,
    );
    write_line(
        &mut output,
        1,
        "\"c03_schema_version\":",
        &json_string(&model.c03_schema_version),
        true,
    );
    write_line(
        &mut output,
        1,
        "\"c03_manifest_generation_version\":",
        &model.c03_manifest_generation_version.to_string(),
        true,
    );
    write_line(
        &mut output,
        1,
        "\"c03_fingerprint_sha256\":",
        &json_string(&model.c03_fingerprint_sha256),
        true,
    );
    write_line(
        &mut output,
        1,
        "\"packet_id\":",
        &json_string(&model.packet_id),
        true,
    );
    write_line(
        &mut output,
        1,
        "\"packet_status\":",
        &json_string(render_packet_status(model.packet_status)),
        true,
    );
    output.push_str("  \"packet_result\": ");
    output.push_str(&render_packet_result_json(&model.packet_result));
    output.push_str(",\n");
    output.push_str("  \"budget_outcome\": ");
    output.push_str(&render_budget_outcome_json(model));
    output.push_str(",\n");
    output.push_str("  \"decision_log_entries\": [\n");
    for (index, entry) in model.decision_log_entries.iter().enumerate() {
        writeln!(
            &mut output,
            "    {}{}",
            json_string(entry),
            if index + 1 == model.decision_log_entries.len() {
                ""
            } else {
                ","
            }
        )
        .expect("decision log json");
    }
    output.push_str("  ],\n");
    output.push_str("  \"refusal\": ");
    output.push_str(&render_refusal_json(model.refusal.as_ref()));
    output.push_str(",\n");
    output.push_str("  \"blockers\": ");
    output.push_str(&render_blockers_json(&model.blockers));
    output.push_str("\n}\n");
    output
}

fn render_packet_result_json(packet: &PacketResult) -> String {
    let mut output = String::from("{\n");
    write_line(
        &mut output,
        2,
        "\"packet_id\":",
        &json_string(&packet.packet_id),
        true,
    );
    write_line(
        &mut output,
        2,
        "\"variant\":",
        &json_string(render_packet_variant(packet.variant)),
        true,
    );
    output.push_str("    \"fixture_context\": ");
    match packet.fixture_context.as_ref() {
        Some(context) => output.push_str(&render_packet_fixture_context_json(context)),
        None => output.push_str("null"),
    }
    output.push_str(",\n");
    output.push_str("    \"included_sources\": ");
    output.push_str(&render_packet_sources_json(&packet.included_sources));
    output.push_str(",\n");
    output.push_str("    \"notes\": ");
    output.push_str(&render_packet_notes_json(&packet.notes));
    output.push_str(",\n");
    output.push_str("    \"decision_summary\": ");
    output.push_str(&render_packet_decision_summary_json(packet));
    output.push_str(",\n");
    output.push_str("    \"sections\": ");
    output.push_str(&render_packet_sections_json(&packet.sections));
    output.push_str("\n  }");
    output
}

fn render_packet_fixture_context_json(context: &PacketFixtureContext) -> String {
    let mut output = String::from("{\n");
    write_line(
        &mut output,
        3,
        "\"fixture_set_id\":",
        &json_string(&context.fixture_set_id),
        true,
    );
    write_line(
        &mut output,
        3,
        "\"fixture_basis_root\":",
        &json_string(&context.fixture_basis_root),
        true,
    );
    output.push_str("      \"fixture_lineage\": ");
    output.push_str(&render_packet_sources_json(&context.fixture_lineage));
    output.push_str("\n    }");
    output
}

fn render_packet_sources_json(sources: &[PacketSourceSummary]) -> String {
    let mut output = String::from("[\n");
    for (index, source) in sources.iter().enumerate() {
        write!(
            &mut output,
            "      {{\n        \"kind\": {},\n        \"canonical_repo_relative_path\": {},\n        \"required\": {},\n        \"presence\": {},\n        \"byte_len\": {},\n        \"content_sha256\": {},\n        \"rendered_output_byte_len\": {},\n        \"rendered_output_sha256\": {},\n        \"rendered_media_type\": {}\n      }}{}\n",
            json_string(&source.label.replace(' ', "")),
            json_string(&source.canonical_repo_relative_path),
            source.required,
            json_string(match source.presence {
                ArtifactPresence::Missing => "Missing",
                ArtifactPresence::PresentEmpty => "PresentEmpty",
                ArtifactPresence::PresentNonEmpty => "PresentNonEmpty",
            }),
            match source.byte_len {
                Some(len) => len.to_string(),
                None => "null".to_string(),
            },
            match source.content_sha256.as_ref() {
                Some(value) => json_string(value),
                None => "null".to_string(),
            },
            match source.rendered_output_byte_len {
                Some(value) => value.to_string(),
                None => "null".to_string(),
            },
            match source.rendered_output_sha256.as_ref() {
                Some(value) => json_string(value),
                None => "null".to_string(),
            },
            match source.rendered_media_type.as_ref() {
                Some(value) => json_string(value),
                None => "null".to_string(),
            },
            if index + 1 == sources.len() { "" } else { "," }
        )
        .expect("packet sources json");
    }
    output.push_str("    ]");
    output
}

fn render_packet_notes_json(notes: &[PacketBodyNote]) -> String {
    let mut output = String::from("[\n");
    for (index, note) in notes.iter().enumerate() {
        write!(
            &mut output,
            "      {{\n        \"kind\": {},\n        \"text\": {}\n      }}{}\n",
            json_string(match note.kind {
                PacketBodyNoteKind::Omission => "Omission",
                PacketBodyNoteKind::Budget => "Budget",
                PacketBodyNoteKind::InheritedDependency => "InheritedDependency",
            }),
            json_string(&note.text),
            if index + 1 == notes.len() { "" } else { "," }
        )
        .expect("packet notes json");
    }
    output.push_str("    ]");
    output
}

fn render_packet_decision_summary_json(packet: &PacketResult) -> String {
    let summary = &packet.decision_summary;
    let mut output = String::from("{\n");
    write_line(
        &mut output,
        3,
        "\"packet_status\":",
        &json_string(render_packet_status(summary.packet_status)),
        true,
    );
    write_line(
        &mut output,
        3,
        "\"budget_disposition\":",
        &json_string(render_budget_disposition(summary.budget_disposition)),
        true,
    );
    write_line(
        &mut output,
        3,
        "\"budget_reason\":",
        &json_string(render_budget_reason(&summary.budget_reason)),
        true,
    );
    write_line(
        &mut output,
        3,
        "\"decision_log_entries\":",
        &summary.decision_log_entries.to_string(),
        true,
    );
    write_line(
        &mut output,
        3,
        "\"summary_line\":",
        &json_string(&summary.summary_line),
        true,
    );
    write_line(
        &mut output,
        3,
        "\"ready_next_safe_action\":",
        &json_string(&render_ready_packet_next_safe_action(packet)),
        false,
    );
    output.push_str("    }");
    output
}

fn render_packet_sections_json(sections: &[PacketSection]) -> String {
    let mut output = String::from("[\n");
    for (index, section) in sections.iter().enumerate() {
        write!(
            &mut output,
            "      {{\n        \"kind\": {},\n        \"canonical_repo_relative_path\": {},\n        \"title\": {},\n        \"mode\": {},\n        \"contents\": {},\n        \"source_content_sha256\": {},\n        \"rendered_output_sha256\": {}\n      }}{}\n",
            json_string(&section.label.replace(' ', "")),
            json_string(&section.canonical_repo_relative_path),
            json_string(&section.title),
            json_string(match section.mode {
                PacketSectionMode::Verbatim => "Verbatim",
                PacketSectionMode::Summary => "Summary",
                PacketSectionMode::Rendered => "Rendered",
            }),
            json_string(&section.contents),
            match section.source_content_sha256.as_ref() {
                Some(value) => json_string(value),
                None => "null".to_string(),
            },
            match section.rendered_output_sha256.as_ref() {
                Some(value) => json_string(value),
                None => "null".to_string(),
            },
            if index + 1 == sections.len() { "" } else { "," }
        )
        .expect("packet sections json");
    }
    output.push_str("    ]");
    output
}

fn render_budget_outcome_json(model: &RenderOutputModel) -> String {
    let mut output = String::from("{\n");
    write_line(
        &mut output,
        2,
        "\"disposition\":",
        &json_string(render_budget_disposition(model.budget_outcome.disposition)),
        true,
    );
    write_line(
        &mut output,
        2,
        "\"reason\":",
        &json_string(render_budget_reason(&model.budget_outcome.reason)),
        true,
    );
    output.push_str("    \"targets\": [\n");
    for (index, target) in model.budget_outcome.targets.iter().enumerate() {
        write!(
            &mut output,
            "      {{\n        \"canonical_repo_relative_path\": {},\n        \"byte_len\": {},\n        \"byte_domain\": {}\n      }}{}\n",
            json_string(&target.canonical_repo_relative_path),
            target.byte_len,
            json_string(match target.byte_domain {
                handbook_sdk::BudgetByteDomain::Source => "Source",
                handbook_sdk::BudgetByteDomain::RenderedOutput => "RenderedOutput",
            }),
            if index + 1 == model.budget_outcome.targets.len() {
                ""
            } else {
                ","
            }
        )
        .expect("budget target json");
    }
    output.push_str("    ],\n");
    output.push_str("    \"next_safe_action\": ");
    match render_budget_next_safe_action(model.budget_outcome.next_safe_action.as_ref()) {
        Some(action) => output.push_str(&json_string(&action)),
        None => output.push_str("null"),
    }
    output.push_str("\n  }");
    output
}

fn render_refusal_json(refusal: Option<&Refusal>) -> String {
    match refusal {
        Some(refusal) => {
            let mut output = String::from("{\n");
            write_line(
                &mut output,
                2,
                "\"category\":",
                &json_string(render_refusal_category(refusal.category)),
                true,
            );
            write_line(
                &mut output,
                2,
                "\"summary\":",
                &json_string(&refusal.summary),
                true,
            );
            output.push_str("    \"broken_subject\": ");
            output.push_str(&render_subject_json(&refusal.broken_subject));
            output.push_str(",\n");
            output.push_str("    \"next_safe_action\": ");
            output.push_str(&json_string(&super::shared::render_next_safe_action_value(
                &refusal.next_safe_action,
            )));
            output.push_str("\n  }");
            output
        }
        None => "null".to_string(),
    }
}

fn render_blockers_json(blockers: &[Blocker]) -> String {
    let mut output = String::from("[\n");
    for (index, blocker) in blockers.iter().enumerate() {
        write!(
            &mut output,
            "    {{\n      \"category\": {},\n      \"summary\": {},\n      \"subject\": {},\n      \"next_safe_action\": {}\n    }}{}\n",
            json_string(render_blocker_category(blocker.category)),
            json_string(&blocker.summary),
            render_subject_json(&blocker.subject),
            json_string(&super::shared::render_next_safe_action_value(&blocker.next_safe_action)),
            if index + 1 == blockers.len() { "" } else { "," }
        )
        .expect("blockers json");
    }
    output.push_str("  ]");
    output
}

fn render_subject_json(subject: &SubjectRef) -> String {
    match subject {
        SubjectRef::CanonicalArtifact {
            label,
            canonical_repo_relative_path,
            ..
        } => format!(
            "{{\n      \"kind\": {},\n      \"canonical_repo_relative_path\": {}\n    }}",
            json_string(&label.replace(' ', "")),
            json_string(canonical_repo_relative_path)
        ),
        SubjectRef::InheritedDependency {
            dependency_id,
            version,
        } => format!(
            "{{\n      \"kind\": \"InheritedDependency\",\n      \"dependency_id\": {},\n      \"version\": {}\n    }}",
            json_string(dependency_id),
            match version {
                Some(version) => json_string(version),
                None => "null".to_string(),
            }
        ),
        SubjectRef::Policy { policy_id } => format!(
            "{{\n      \"kind\": \"Policy\",\n      \"policy_id\": {}\n    }}",
            json_string(policy_id)
        ),
    }
}

fn write_line(output: &mut String, indent: usize, key: &str, value: &str, trailing_comma: bool) {
    for _ in 0..indent {
        output.push_str("  ");
    }
    output.push_str(key);
    output.push(' ');
    output.push_str(value);
    if trailing_comma {
        output.push(',');
    }
    output.push('\n');
}
