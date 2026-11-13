use crate::definition_identity::ExactDefinitionRef;

pub(crate) struct BuiltInSource {
    pub(crate) package_path: &'static str,
    pub(crate) bytes: &'static [u8],
}

macro_rules! source {
    ($path:literal) => {
        BuiltInSource {
            package_path: concat!("definitions/", $path),
            bytes: include_bytes!(concat!("../definitions/", $path)),
        }
    };
}

pub(crate) fn definition(reference: &ExactDefinitionRef) -> Option<BuiltInSource> {
    Some(match reference.as_str() {
        "handbook.profile.shipped-root@1.0.0" => {
            source!("profiles/handbook.profile.shipped-root/1.0.0.yaml")
        }
        "handbook.profile.shipped-root@1.1.0" => {
            source!("profiles/handbook.profile.shipped-root/1.1.0.yaml")
        }
        "handbook.profile.shipped-root@1.2.0" => {
            source!("profiles/handbook.profile.shipped-root/1.2.0.yaml")
        }
        "handbook.roles.core@1.0.0" => {
            source!("stable-roles/handbook.roles.core/1.0.0.yaml")
        }
        "handbook.roles.core@1.1.0" => {
            source!("stable-roles/handbook.roles.core/1.1.0.yaml")
        }
        "handbook.schemas.artifacts.project-authority@1.0.0" => {
            source!("schemas/handbook.schemas.artifacts.project-authority/1.0.0.entry.yaml")
        }
        "handbook.schemas.artifacts.project-authority@1.1.0" => {
            source!("schemas/handbook.schemas.artifacts.project-authority/1.1.0.entry.yaml")
        }
        "handbook.schemas.lifecycle.trigger-evidence@1.0.0" => {
            source!("schemas/handbook.schemas.lifecycle.trigger-evidence/1.0.0.entry.yaml")
        }
        "handbook.schemas.security.approver-registry@1.0.0" => {
            source!("schemas/handbook.schemas.security.approver-registry/1.0.0.entry.yaml")
        }
        "handbook.schemas.security.approver-registry-transition@1.0.0" => source!(
            "schemas/handbook.schemas.security.approver-registry-transition/1.0.0.entry.yaml"
        ),
        "handbook.schemas.security.authenticator-registration-request@1.0.0" => source!(
            "schemas/handbook.schemas.security.authenticator-registration-request/1.0.0.entry.yaml"
        ),
        "handbook.schemas.security.authenticator-challenge@1.0.0" => {
            source!("schemas/handbook.schemas.security.authenticator-challenge/1.0.0.entry.yaml")
        }
        "handbook.schemas.security.authenticator-make-credential-response@1.0.0" => source!(
            "schemas/handbook.schemas.security.authenticator-make-credential-response/1.0.0.entry.yaml"
        ),
        "handbook.schemas.security.authenticator-registration@1.0.0" => source!(
            "schemas/handbook.schemas.security.authenticator-registration/1.0.0.entry.yaml"
        ),
        "handbook.schemas.security.authenticator-get-assertion-response@1.0.0" => source!(
            "schemas/handbook.schemas.security.authenticator-get-assertion-response/1.0.0.entry.yaml"
        ),
        "handbook.schemas.security.authenticator-assertion@1.0.0" => {
            source!("schemas/handbook.schemas.security.authenticator-assertion/1.0.0.entry.yaml")
        }
        "handbook.schemas.security.approver-admin-api@1.0.0" => {
            source!("schemas/handbook.schemas.security.approver-admin-api/1.0.0.entry.yaml")
        }
        "handbook.schemas.artifacts.project-context@1.0.0" => {
            source!("schemas/handbook.schemas.artifacts.project-context/1.0.0.entry.yaml")
        }
        "handbook.schemas.artifacts.environment-context@1.0.0" => {
            source!("schemas/handbook.schemas.artifacts.environment-context/1.0.0.entry.yaml")
        }
        "handbook.schemas.artifacts.environment-context@1.1.0" => {
            source!("schemas/handbook.schemas.artifacts.environment-context/1.1.0.entry.yaml")
        }
        "handbook.schemas.artifacts.work-specification@1.0.0" => {
            source!("schemas/handbook.schemas.artifacts.work-specification/1.0.0.entry.yaml")
        }
        "handbook.schemas.artifacts.decision-record@1.0.0" => {
            source!("schemas/handbook.schemas.artifacts.decision-record/1.0.0.entry.yaml")
        }
        "handbook.schemas.artifacts.risk-record@1.0.0" => {
            source!("schemas/handbook.schemas.artifacts.risk-record/1.0.0.entry.yaml")
        }
        "handbook.artifact-kind.project-authority@1.0.0" => {
            source!("artifact-kinds/handbook.artifact-kind.project-authority/1.0.0.yaml")
        }
        "handbook.artifact-kind.project-authority@1.1.0" => {
            source!("artifact-kinds/handbook.artifact-kind.project-authority/1.1.0.yaml")
        }
        "handbook.artifact-kind.project-context@1.0.0" => {
            source!("artifact-kinds/handbook.artifact-kind.project-context/1.0.0.yaml")
        }
        "handbook.artifact-kind.project-context@1.1.0" => {
            source!("artifact-kinds/handbook.artifact-kind.project-context/1.1.0.yaml")
        }
        "handbook.artifact-kind.environment-context@1.0.0" => {
            source!("artifact-kinds/handbook.artifact-kind.environment-context/1.0.0.yaml")
        }
        "handbook.artifact-kind.environment-context@1.1.0" => {
            source!("artifact-kinds/handbook.artifact-kind.environment-context/1.1.0.yaml")
        }
        "handbook.artifact-kind.work-specification@1.0.0" => {
            source!("artifact-kinds/handbook.artifact-kind.work-specification/1.0.0.yaml")
        }
        "handbook.artifact-kind.work-specification@1.1.0" => {
            source!("artifact-kinds/handbook.artifact-kind.work-specification/1.1.0.yaml")
        }
        "handbook.artifact-kind.decision-record@1.0.0" => {
            source!("artifact-kinds/handbook.artifact-kind.decision-record/1.0.0.yaml")
        }
        "handbook.artifact-kind.decision-record@1.1.0" => {
            source!("artifact-kinds/handbook.artifact-kind.decision-record/1.1.0.yaml")
        }
        "handbook.artifact-kind.risk-record@1.0.0" => {
            source!("artifact-kinds/handbook.artifact-kind.risk-record/1.0.0.yaml")
        }
        "handbook.artifact-kind.risk-record@1.1.0" => {
            source!("artifact-kinds/handbook.artifact-kind.risk-record/1.1.0.yaml")
        }
        "handbook.capabilities.constitutional-root@1.0.0" => {
            source!("semantic-capabilities/handbook.capabilities.constitutional-root/1.0.0.yaml")
        }
        "handbook.semantic-validation.constitutional-root@1.0.0" => source!(
            "semantic-validators/handbook.semantic-validation.constitutional-root/1.0.0.yaml"
        ),
        "handbook.semantic-validation.constitutional-root@1.1.0" => source!(
            "semantic-validators/handbook.semantic-validation.constitutional-root/1.1.0.yaml"
        ),
        "handbook.approval.constitutional-candidate@1.0.0" => source!(
            "approval-policies/handbook.approval.constitutional-candidate/1.0.0.yaml"
        ),
        "handbook.waiver.constitutional-intake@1.0.0" => source!(
            "waiver-policies/handbook.waiver.constitutional-intake/1.0.0.yaml"
        ),
        "handbook.intake-trigger.production-posture-changed@1.0.0" => source!(
            "triggers/handbook.intake-trigger.production-posture-changed/1.0.0.yaml"
        ),
        "handbook.intake-trigger.trust-boundary-changed@1.0.0" => source!(
            "triggers/handbook.intake-trigger.trust-boundary-changed/1.0.0.yaml"
        ),
        "handbook.lifecycle-trigger.charter-amendment-proposed@1.0.0" => source!(
            "triggers/handbook.lifecycle-trigger.charter-amendment-proposed/1.0.0.yaml"
        ),
        "handbook.renderer.charter-review-markdown@1.0.0" => source!(
            "renderers/handbook.renderer.charter-review-markdown/1.0.0.yaml"
        ),
        "handbook.renderer.project-context-review-markdown@1.0.0" => source!(
            "renderers/handbook.renderer.project-context-review-markdown/1.0.0.yaml"
        ),
        "handbook.renderer.environment-context-review-markdown@1.0.0" => source!(
            "renderers/handbook.renderer.environment-context-review-markdown/1.0.0.yaml"
        ),
        "handbook.renderer.work-specification-review-markdown@1.0.0" => source!(
            "renderers/handbook.renderer.work-specification-review-markdown/1.0.0.yaml"
        ),
        "handbook.renderer.decision-record-review-markdown@1.0.0" => source!(
            "renderers/handbook.renderer.decision-record-review-markdown/1.0.0.yaml"
        ),
        "handbook.renderer.risk-record-review-markdown@1.0.0" => source!(
            "renderers/handbook.renderer.risk-record-review-markdown/1.0.0.yaml"
        ),
        "handbook.lifecycle.constitutional-review-lock@1.0.0" => source!(
            "lifecycle-policies/handbook.lifecycle.constitutional-review-lock/1.0.0.yaml"
        ),
        "handbook.intake.charter@1.0.0" => {
            source!("intakes/handbook.intake.charter/1.0.0.yaml")
        }
        "handbook.intake.project-context@1.0.0" => {
            source!("intakes/handbook.intake.project-context/1.0.0.yaml")
        }
        "handbook.intake.environment-context@1.0.0" => {
            source!("intakes/handbook.intake.environment-context/1.0.0.yaml")
        }
        "handbook.intake.work-specification@1.0.0" => {
            source!("intakes/handbook.intake.work-specification/1.0.0.yaml")
        }
        "handbook.intake.decision-record@1.0.0" => {
            source!("intakes/handbook.intake.decision-record/1.0.0.yaml")
        }
        "handbook.intake.risk-record@1.0.0" => {
            source!("intakes/handbook.intake.risk-record/1.0.0.yaml")
        }
        "handbook.condition.project.managed-operational-surface@1.0.0" => source!(
            "project-conditions/handbook.condition.project.managed-operational-surface/1.0.0.yaml"
        ),
        "handbook.vocabulary.shipped-root@1.0.0" => {
            source!("vocabularies/handbook.vocabulary.shipped-root/1.0.0.yaml")
        }
        "handbook.context-resolution.shipped-root@1.0.0" => {
            source!("context-resolution/handbook.context-resolution.shipped-root/1.0.0.yaml")
        }
        "handbook.mutation-matcher.core@1.0.0" => {
            source!("context-resolution-policies/handbook.mutation-matcher.core/1.0.0.yaml")
        }
        "handbook.resolution-escalation.core@1.0.0" => {
            source!("context-resolution-policies/handbook.resolution-escalation.core/1.0.0.yaml")
        }
        "handbook.memory-promotion.core@1.0.0" => {
            source!("context-resolution-policies/handbook.memory-promotion.core/1.0.0.yaml")
        }
        _ => return None,
    })
}

pub(crate) fn schema_document(package_path: &str) -> Option<&'static [u8]> {
    Some(match package_path {
        "definitions/schemas/handbook.schemas.artifacts.project-authority/1.0.0.schema.json" => include_bytes!(
            "../definitions/schemas/handbook.schemas.artifacts.project-authority/1.0.0.schema.json"
        ),
        "definitions/schemas/handbook.schemas.artifacts.project-authority/1.1.0.schema.json" => include_bytes!(
            "../definitions/schemas/handbook.schemas.artifacts.project-authority/1.1.0.schema.json"
        ),
        "definitions/schemas/handbook.schemas.lifecycle.trigger-evidence/1.0.0.schema.json" => include_bytes!(
            "../definitions/schemas/handbook.schemas.lifecycle.trigger-evidence/1.0.0.schema.json"
        ),
        "definitions/schemas/handbook.schemas.security.approver-registry/1.0.0.schema.json" => include_bytes!(
            "../definitions/schemas/handbook.schemas.security.approver-registry/1.0.0.schema.json"
        ),
        "definitions/schemas/handbook.schemas.security.approver-registry-transition/1.0.0.schema.json" => include_bytes!(
            "../definitions/schemas/handbook.schemas.security.approver-registry-transition/1.0.0.schema.json"
        ),
        "definitions/schemas/handbook.schemas.security.authenticator-registration-request/1.0.0.schema.json" => include_bytes!(
            "../definitions/schemas/handbook.schemas.security.authenticator-registration-request/1.0.0.schema.json"
        ),
        "definitions/schemas/handbook.schemas.security.authenticator-challenge/1.0.0.schema.json" => include_bytes!(
            "../definitions/schemas/handbook.schemas.security.authenticator-challenge/1.0.0.schema.json"
        ),
        "definitions/schemas/handbook.schemas.security.authenticator-make-credential-response/1.0.0.schema.json" => include_bytes!(
            "../definitions/schemas/handbook.schemas.security.authenticator-make-credential-response/1.0.0.schema.json"
        ),
        "definitions/schemas/handbook.schemas.security.authenticator-registration/1.0.0.schema.json" => include_bytes!(
            "../definitions/schemas/handbook.schemas.security.authenticator-registration/1.0.0.schema.json"
        ),
        "definitions/schemas/handbook.schemas.security.authenticator-get-assertion-response/1.0.0.schema.json" => include_bytes!(
            "../definitions/schemas/handbook.schemas.security.authenticator-get-assertion-response/1.0.0.schema.json"
        ),
        "definitions/schemas/handbook.schemas.security.authenticator-assertion/1.0.0.schema.json" => include_bytes!(
            "../definitions/schemas/handbook.schemas.security.authenticator-assertion/1.0.0.schema.json"
        ),
        "definitions/schemas/handbook.schemas.security.approver-admin-api/1.0.0.schema.json" => include_bytes!(
            "../definitions/schemas/handbook.schemas.security.approver-admin-api/1.0.0.schema.json"
        ),
        "definitions/schemas/handbook.schemas.artifacts.project-context/1.0.0.schema.json" => include_bytes!(
            "../definitions/schemas/handbook.schemas.artifacts.project-context/1.0.0.schema.json"
        ),
        "definitions/schemas/handbook.schemas.artifacts.environment-context/1.0.0.schema.json" => include_bytes!(
            "../definitions/schemas/handbook.schemas.artifacts.environment-context/1.0.0.schema.json"
        ),
        "definitions/schemas/handbook.schemas.artifacts.environment-context/1.1.0.schema.json" => include_bytes!(
            "../definitions/schemas/handbook.schemas.artifacts.environment-context/1.1.0.schema.json"
        ),
        "definitions/schemas/handbook.schemas.artifacts.work-specification/1.0.0.schema.json" => include_bytes!(
            "../definitions/schemas/handbook.schemas.artifacts.work-specification/1.0.0.schema.json"
        ),
        "definitions/schemas/handbook.schemas.artifacts.decision-record/1.0.0.schema.json" => include_bytes!(
            "../definitions/schemas/handbook.schemas.artifacts.decision-record/1.0.0.schema.json"
        ),
        "definitions/schemas/handbook.schemas.artifacts.risk-record/1.0.0.schema.json" => include_bytes!(
            "../definitions/schemas/handbook.schemas.artifacts.risk-record/1.0.0.schema.json"
        ),
        _ => return None,
    })
}
