use crate::{SetupArgs, SetupCommand};
use std::process::ExitCode;

pub(crate) fn run(args: SetupArgs) -> ExitCode {
    let cwd = match std::env::current_dir() {
        Ok(dir) => dir,
        Err(err) => {
            println!("REFUSED: failed to determine repo root: {err}");
            return ExitCode::from(1);
        }
    };
    let repo_root = crate::shell_shared::discover_managed_repo_root(&cwd);

    let (request, routed_from_auto) = match args.command {
        None => (
            handbook_compiler::SetupRequest {
                mode: handbook_compiler::SetupMode::Auto,
                ..handbook_compiler::SetupRequest::default()
            },
            true,
        ),
        Some(SetupCommand::Init) => (
            handbook_compiler::SetupRequest {
                mode: handbook_compiler::SetupMode::Init,
                ..handbook_compiler::SetupRequest::default()
            },
            false,
        ),
        Some(SetupCommand::Refresh(refresh)) => (
            handbook_compiler::SetupRequest {
                mode: handbook_compiler::SetupMode::Refresh,
                rewrite: refresh.rewrite,
                reset_state: refresh.reset_state,
            },
            false,
        ),
    };

    match handbook_compiler::run_setup(&repo_root, &request) {
        Ok(outcome) => {
            println!("{}", render_setup_success(&outcome, routed_from_auto));
            ExitCode::SUCCESS
        }
        Err(refusal) => {
            println!("{}", render_setup_refusal(&refusal));
            ExitCode::from(1)
        }
    }
}

fn render_setup_success(
    outcome: &handbook_compiler::SetupOutcome,
    routed_from_auto: bool,
) -> String {
    let mut out = String::new();
    let starter_actions = outcome
        .plan
        .actions
        .iter()
        .filter(|action| action.label != handbook_compiler::SetupActionLabel::Reset)
        .collect::<Vec<_>>();
    let state_updates = outcome
        .plan
        .actions
        .iter()
        .filter(|action| action.label == handbook_compiler::SetupActionLabel::Reset)
        .collect::<Vec<_>>();
    out.push_str(&format!(
        "OUTCOME: {}\n",
        setup_success_outcome_name(outcome.disposition)
    ));
    out.push_str(&format!(
        "OBJECT: {}\n",
        setup_object_name(outcome.plan.resolved_mode)
    ));
    out.push_str(&format!("NEXT SAFE ACTION: {}\n", outcome.next_safe_action));
    out.push_str("## CANONICAL ROOT\n");
    out.push_str(match outcome.plan.resolved_mode {
        handbook_compiler::SetupMode::Init => "STATUS: established canonical `.handbook/` root\n",
        handbook_compiler::SetupMode::Refresh => "STATUS: reused canonical `.handbook/` root\n",
        handbook_compiler::SetupMode::Auto => {
            unreachable!("setup mode should resolve before render")
        }
    });
    out.push_str("## STARTER FILES\n");
    for action in starter_actions {
        out.push_str(&format!(
            "{} {}\n",
            setup_action_label_name(action.label),
            setup_action_path(action)
        ));
    }
    out.push_str("## STATE UPDATES\n");
    if state_updates.is_empty() {
        out.push_str("<none>\n");
    } else {
        for action in state_updates {
            out.push_str(&format!(
                "{} {}\n",
                setup_action_label_name(action.label),
                action.path
            ));
        }
    }
    out.push_str("## MODE NOTES\n");
    if routed_from_auto {
        out.push_str("ROUTED FROM: handbook setup -> ");
        out.push_str(setup_command_name(outcome.plan.resolved_mode));
        out.push('\n');
    }
    if outcome.disposition == handbook_compiler::SetupDisposition::Scaffolded {
        out.push_str(
            "Required starter files still contain shipped scaffold text; replace canonical truth before running `handbook doctor` or packet work.\n",
        );
    }

    out.trim_end().to_string()
}

fn setup_success_outcome_name(disposition: handbook_compiler::SetupDisposition) -> &'static str {
    match disposition {
        handbook_compiler::SetupDisposition::Ready => "READY",
        handbook_compiler::SetupDisposition::Scaffolded => "SCAFFOLDED",
    }
}

fn render_setup_refusal(refusal: &handbook_compiler::SetupRefusal) -> String {
    let mut out = String::new();
    let next_safe_action = refusal.next_safe_action.trim();
    out.push_str(&format!(
        "OUTCOME: {}\n",
        setup_refusal_outcome_name(refusal.kind)
    ));
    out.push_str("OBJECT: setup\n");
    out.push_str(&format!("NEXT SAFE ACTION: {next_safe_action}\n"));
    out.push_str("## REFUSAL\n");
    out.push_str(&format!(
        "CATEGORY: {}\n",
        setup_refusal_kind_name(refusal.kind)
    ));
    out.push_str(&format!("SUMMARY: {}\n", refusal.summary.trim()));
    out.push_str(&format!(
        "BROKEN SUBJECT: {}\n",
        refusal.broken_subject.trim()
    ));
    out.push_str(&format!("NEXT SAFE ACTION: {next_safe_action}\n"));
    out.trim_end().to_string()
}

fn setup_command_name(mode: handbook_compiler::SetupMode) -> &'static str {
    match mode {
        handbook_compiler::SetupMode::Auto => "handbook setup",
        handbook_compiler::SetupMode::Init => "handbook setup init",
        handbook_compiler::SetupMode::Refresh => "handbook setup refresh",
    }
}

fn setup_object_name(mode: handbook_compiler::SetupMode) -> &'static str {
    match mode {
        handbook_compiler::SetupMode::Auto => "setup",
        handbook_compiler::SetupMode::Init => "setup init",
        handbook_compiler::SetupMode::Refresh => "setup refresh",
    }
}

fn setup_action_label_name(label: handbook_compiler::SetupActionLabel) -> &'static str {
    match label {
        handbook_compiler::SetupActionLabel::Created => "created",
        handbook_compiler::SetupActionLabel::Preserved => "preserved",
        handbook_compiler::SetupActionLabel::Rewritten => "rewritten",
        handbook_compiler::SetupActionLabel::Reset => "reset",
    }
}

fn setup_action_path(action: &handbook_compiler::SetupAction) -> String {
    action.path.clone()
}

fn setup_refusal_outcome_name(kind: handbook_compiler::SetupRefusalKind) -> &'static str {
    match kind {
        handbook_compiler::SetupRefusalKind::AlreadyInitialized
        | handbook_compiler::SetupRefusalKind::InvalidRequest => "REFUSED",
        handbook_compiler::SetupRefusalKind::MissingCanonicalRoot
        | handbook_compiler::SetupRefusalKind::InvalidCanonicalRoot
        | handbook_compiler::SetupRefusalKind::MutationRefused => "BLOCKED",
    }
}

fn setup_refusal_kind_name(kind: handbook_compiler::SetupRefusalKind) -> &'static str {
    match kind {
        handbook_compiler::SetupRefusalKind::AlreadyInitialized => "AlreadyInitialized",
        handbook_compiler::SetupRefusalKind::MissingCanonicalRoot => "MissingCanonicalRoot",
        handbook_compiler::SetupRefusalKind::InvalidCanonicalRoot => "InvalidCanonicalRoot",
        handbook_compiler::SetupRefusalKind::InvalidRequest => "InvalidRequest",
        handbook_compiler::SetupRefusalKind::MutationRefused => "MutationRefused",
    }
}
