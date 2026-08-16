use std::collections::BTreeSet;

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub(crate) enum ResultClass {
    Operation(&'static str),
    UseCase(&'static str),
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct CoverageRow {
    pub(crate) leaf: &'static str,
    pub(crate) result_class: ResultClass,
    pub(crate) machine_json: bool,
    pub(crate) legacy_json: bool,
}

const fn operation(leaf: &'static str, operation: &'static str, legacy_json: bool) -> CoverageRow {
    CoverageRow {
        leaf,
        result_class: ResultClass::Operation(operation),
        machine_json: true,
        legacy_json,
    }
}

const fn use_case(leaf: &'static str, use_case: &'static str, legacy_json: bool) -> CoverageRow {
    CoverageRow {
        leaf,
        result_class: ResultClass::UseCase(use_case),
        machine_json: true,
        legacy_json,
    }
}

static COVERAGE: [CoverageRow; 36] = [
    use_case(
        "setup.auto",
        "handbook.use-case.repository-setup@1.0.0",
        false,
    ),
    use_case(
        "setup.init",
        "handbook.use-case.repository-setup@1.0.0",
        false,
    ),
    use_case(
        "setup.refresh",
        "handbook.use-case.repository-setup@1.0.0",
        false,
    ),
    use_case(
        "author.charter.author",
        "handbook.use-case.author-charter@1.0.0",
        true,
    ),
    use_case(
        "author.charter.approve",
        "handbook.use-case.author-charter@1.0.0",
        true,
    ),
    use_case(
        "author.charter.promote",
        "handbook.use-case.author-charter@1.0.0",
        true,
    ),
    use_case(
        "author.charter.validate",
        "handbook.use-case.author-charter@1.0.0",
        true,
    ),
    use_case(
        "author.project_context.author",
        "handbook.use-case.author-project-context@1.0.0",
        false,
    ),
    use_case(
        "author.project_context.validate",
        "handbook.use-case.author-project-context@1.0.0",
        false,
    ),
    use_case(
        "approvers.bootstrap",
        "handbook.use-case.approver-administration@1.0.0",
        true,
    ),
    use_case(
        "approvers.add_credential",
        "handbook.use-case.approver-administration@1.0.0",
        true,
    ),
    use_case(
        "approvers.revoke_credential",
        "handbook.use-case.approver-administration@1.0.0",
        true,
    ),
    use_case(
        "approvers.update_mapping",
        "handbook.use-case.approver-administration@1.0.0",
        true,
    ),
    operation("artifact.list_kinds", "artifact.kind.list@1.0.0", true),
    operation(
        "artifact.list_instances",
        "artifact.instance.list@1.0.0",
        true,
    ),
    operation("artifact.read", "artifact.read@1.0.0", true),
    operation("artifact.validate", "artifact.validate@1.0.0", true),
    operation(
        "artifact.intake_definition_read",
        "intake.definition.read@1.0.0",
        true,
    ),
    operation(
        "artifact.intake_evaluate",
        "intake.coverage.evaluate@1.0.0",
        true,
    ),
    operation("artifact.intake_append", "intake.record.append@1.0.0", true),
    operation(
        "artifact.candidate_validate",
        "artifact.candidate.validate@1.0.0",
        true,
    ),
    operation(
        "artifact.candidate_append",
        "artifact.candidate.append@1.0.0",
        true,
    ),
    operation("artifact.promote", "artifact.candidate.promote@1.0.0", true),
    operation("pipeline.list", "pipeline.catalog.list@1.0.0", false),
    operation("pipeline.show", "pipeline.catalog.read@1.0.0", false),
    use_case(
        "pipeline.resolve",
        "handbook.use-case.pipeline-route-resolve-and-persist@1.0.0",
        false,
    ),
    operation("pipeline.compile_payload", "pipeline.compile@1.0.0", false),
    operation("pipeline.compile_explain", "pipeline.compile@1.0.0", false),
    operation(
        "pipeline.capture_preview",
        "pipeline.capture.plan@1.0.0",
        false,
    ),
    operation(
        "pipeline.capture_direct",
        "pipeline.capture.apply@1.0.0",
        false,
    ),
    operation(
        "pipeline.capture_apply",
        "pipeline.capture.apply@1.0.0",
        false,
    ),
    operation(
        "pipeline.handoff_emit",
        "pipeline.handoff.emit@1.0.0",
        false,
    ),
    operation("pipeline.state_set", "pipeline.state.apply@1.0.0", false),
    operation("flow.generate", "flow.resolve@1.0.0", false),
    operation("flow.inspect", "flow.resolve@1.0.0", false),
    use_case("doctor", "handbook.use-case.repository-doctor@1.0.0", true),
];

fn authority_class(leaf: &str) -> Option<ResultClass> {
    match leaf {
        "setup.auto" | "setup.init" | "setup.refresh" => Some(ResultClass::UseCase(
            "handbook.use-case.repository-setup@1.0.0",
        )),
        "author.charter.author"
        | "author.charter.approve"
        | "author.charter.promote"
        | "author.charter.validate" => Some(ResultClass::UseCase(
            "handbook.use-case.author-charter@1.0.0",
        )),
        "author.project_context.author" | "author.project_context.validate" => Some(
            ResultClass::UseCase("handbook.use-case.author-project-context@1.0.0"),
        ),
        "approvers.bootstrap"
        | "approvers.add_credential"
        | "approvers.revoke_credential"
        | "approvers.update_mapping" => Some(ResultClass::UseCase(
            "handbook.use-case.approver-administration@1.0.0",
        )),
        "pipeline.resolve" => Some(ResultClass::UseCase(
            "handbook.use-case.pipeline-route-resolve-and-persist@1.0.0",
        )),
        "doctor" => Some(ResultClass::UseCase(
            "handbook.use-case.repository-doctor@1.0.0",
        )),
        "artifact.list_kinds" => Some(ResultClass::Operation("artifact.kind.list@1.0.0")),
        "artifact.list_instances" => Some(ResultClass::Operation("artifact.instance.list@1.0.0")),
        "artifact.read" => Some(ResultClass::Operation("artifact.read@1.0.0")),
        "artifact.validate" => Some(ResultClass::Operation("artifact.validate@1.0.0")),
        "artifact.intake_definition_read" => {
            Some(ResultClass::Operation("intake.definition.read@1.0.0"))
        }
        "artifact.intake_evaluate" => {
            Some(ResultClass::Operation("intake.coverage.evaluate@1.0.0"))
        }
        "artifact.intake_append" => Some(ResultClass::Operation("intake.record.append@1.0.0")),
        "artifact.candidate_validate" => {
            Some(ResultClass::Operation("artifact.candidate.validate@1.0.0"))
        }
        "artifact.candidate_append" => {
            Some(ResultClass::Operation("artifact.candidate.append@1.0.0"))
        }
        "artifact.promote" => Some(ResultClass::Operation("artifact.candidate.promote@1.0.0")),
        "pipeline.list" => Some(ResultClass::Operation("pipeline.catalog.list@1.0.0")),
        "pipeline.show" => Some(ResultClass::Operation("pipeline.catalog.read@1.0.0")),
        "pipeline.compile_payload" | "pipeline.compile_explain" => {
            Some(ResultClass::Operation("pipeline.compile@1.0.0"))
        }
        "pipeline.capture_preview" => Some(ResultClass::Operation("pipeline.capture.plan@1.0.0")),
        "pipeline.capture_direct" | "pipeline.capture_apply" => {
            Some(ResultClass::Operation("pipeline.capture.apply@1.0.0"))
        }
        "pipeline.handoff_emit" => Some(ResultClass::Operation("pipeline.handoff.emit@1.0.0")),
        "pipeline.state_set" => Some(ResultClass::Operation("pipeline.state.apply@1.0.0")),
        "flow.generate" | "flow.inspect" => Some(ResultClass::Operation("flow.resolve@1.0.0")),
        _ => None,
    }
}

pub(crate) fn rows() -> &'static [CoverageRow] {
    &COVERAGE
}

#[allow(dead_code)]
pub(crate) fn validate() -> Result<(), &'static str> {
    let rows = rows();
    if rows.len() != 36 {
        return Err("HCM-4.3 coverage must contain exactly 36 leaf/modes");
    }
    if !rows.iter().all(|row| row.machine_json) {
        return Err("every HCM-4.3 leaf/mode must admit machine JSON selection");
    }
    let unique = rows.iter().map(|row| row.leaf).collect::<BTreeSet<_>>();
    if unique.len() != rows.len() {
        return Err("HCM-4.3 coverage contains a duplicate leaf/mode");
    }
    let use_cases = rows
        .iter()
        .filter(|row| matches!(row.result_class, ResultClass::UseCase(_)))
        .count();
    let operations = rows
        .iter()
        .filter(|row| matches!(row.result_class, ResultClass::Operation(_)))
        .count();
    if use_cases != 15 || operations != 21 {
        return Err("HCM-4.3 coverage must partition into 15 use cases and 21 operations");
    }
    if rows.iter().filter(|row| row.legacy_json).count() != 19 {
        return Err("HCM-4.3 legacy JSON coverage must remain exactly 19 leaf/modes");
    }
    if rows
        .iter()
        .any(|row| authority_class(row.leaf) != Some(row.result_class))
    {
        return Err("HCM-4.3 leaf/mode is missing or mapped to the wrong exact result authority");
    }
    Ok(())
}
