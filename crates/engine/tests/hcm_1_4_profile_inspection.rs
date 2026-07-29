#[cfg(not(windows))]
use handbook_engine::{
    inspect_profile_repository, resolve_shipped_profile_decisions, ArtifactInspectionReason,
    ArtifactInspectionStatus,
};
#[cfg(not(windows))]
use tempfile::tempdir;

#[cfg(all(not(unix), not(windows)))]
#[test]
fn non_unix_repository_inspection_refuses_before_read() {
    let repo = tempdir().expect("temporary repository");
    let decisions = resolve_shipped_profile_decisions(repo.path()).expect("shipped decisions");
    let report = inspect_profile_repository(repo.path(), &decisions);

    assert_eq!(
        report.artifacts().len(),
        decisions.artifact_decisions().len()
    );
    assert!(report.artifacts().iter().all(|artifact| {
        artifact.status() == ArtifactInspectionStatus::UnsafePath
            && artifact.reason() == ArtifactInspectionReason::UnsupportedPlatformStrictRead
    }));
}

#[cfg(unix)]
mod unix {
    use super::*;
    use handbook_engine::{
        load_artifact_kind_registry, resolve_profile_selection,
        shipped_root_artifact_instance_values, ArtifactInstanceRegistry,
        ArtifactKindRegistryLoadRequest, DefinitionFingerprint, DefinitionSource,
        DefinitionSourceBinding, ExactDefinitionRef, ProfileSelectionRequest, RequirednessMode,
        ResolvedProfileDecisions, SchemaRegistry, MAX_SOURCE_DOCUMENT_BYTES,
        MAX_TOTAL_SOURCE_BYTES,
    };
    use serde_json::{json, Value};
    use std::fs;
    use std::os::unix::fs::symlink;
    use std::os::unix::fs::PermissionsExt;
    use std::path::Path;

    fn write_yaml(repo: &std::path::Path, relative: &str, value: serde_json::Value) {
        let path = repo.join(relative);
        fs::create_dir_all(path.parent().unwrap()).unwrap();
        fs::write(path, serde_yaml_bw::to_string(&value).unwrap()).unwrap();
    }

    fn authority() -> serde_json::Value {
        serde_yaml_bw::from_slice(include_bytes!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../docs/specs/handbook-contract-membrane/slices/HCM-2.2/contracts/canonical-charter-boundary-v1.1.yaml"
        )))
        .expect("canonical Charter 1.1 fixture")
    }

    fn authority_v1_0() -> serde_json::Value {
        json!({
            "schema_id": "handbook.artifact.project-authority",
            "schema_version": "1.0",
            "record_id": "example.record.project-authority",
            "policy": {"revision": "1", "authority_statement": "The project authority is explicit."},
            "governance": {
                "decision_authority": ["Owner"], "required_approvals": ["Owner"],
                "exception_policy": "Exceptions require explicit approval.",
                "review_triggers": ["Authority changes"], "reassessment_triggers": ["Scope changes"]
            },
            "engineering_posture": {"dimensions": ["Reliability"], "red_lines": ["No silent authority mutation"]}
        })
    }

    fn project_context() -> serde_json::Value {
        json!({
            "schema_id":"handbook.artifact.project-context", "schema_version":"1.0",
            "record_id":"example.record.project-context", "summary":"Summary",
            "system_boundaries":["API"], "ownership":["Team"],
            "authoritative_references":[], "known_unknowns":[]
        })
    }

    fn environment_context() -> serde_json::Value {
        json!({
            "schema_id":"handbook.artifact.environment-context", "schema_version":"1.1",
            "record_id":"example.record.environment-context",
            "environments":[{
                "environment_id":"local-dev",
                "description":"Local development and focused tests.",
                "capabilities":["rust.stable", "filesystem.workspace-write"]
            }],
            "authoritative_references":[], "known_unknowns":[]
        })
    }

    fn resolved_decisions(repo: &std::path::Path) -> handbook_engine::ResolvedProfileDecisions {
        resolve_shipped_profile_decisions(repo).unwrap()
    }

    fn row<'a>(
        report: &'a handbook_engine::ProfileInspectionReport,
        id: &str,
    ) -> &'a handbook_engine::ArtifactInspection {
        report
            .artifacts()
            .iter()
            .find(|artifact| artifact.instance_id().as_str() == id)
            .unwrap()
    }

    fn assert_shipped_charter_bytes(
        bytes: &[u8],
        expected_status: ArtifactInspectionStatus,
        expected_reason: ArtifactInspectionReason,
    ) {
        let repo = tempdir().unwrap();
        let decisions = resolved_decisions(repo.path());
        write(&repo.path().join(".handbook/project/charter.yaml"), bytes);
        let report = inspect_profile_repository(repo.path(), &decisions);
        assert_eq!(row(&report, "project_authority").status(), expected_status);
        assert_eq!(row(&report, "project_authority").reason(), expected_reason);
    }

    #[test]
    fn required_path_missing_row_is_exact() {
        let repo = tempdir().unwrap();
        let report = inspect_profile_repository(repo.path(), &resolved_decisions(repo.path()));
        assert_eq!(
            (
                row(&report, "project_authority").status(),
                row(&report, "project_authority").reason(),
            ),
            (
                ArtifactInspectionStatus::Missing,
                ArtifactInspectionReason::RequiredPathMissing,
            )
        );
    }

    #[test]
    fn optional_path_missing_row_is_exact() {
        let repo = tempdir().unwrap();
        let decisions = custom_decisions(repo.path(), 1, RequirednessMode::Optional);
        let report = inspect_profile_repository(repo.path(), &decisions);
        assert_eq!(
            (
                report.artifacts()[0].status(),
                report.artifacts()[0].reason()
            ),
            (
                ArtifactInspectionStatus::Missing,
                ArtifactInspectionReason::OptionalPathMissing,
            )
        );
    }

    #[test]
    fn shipped_optional_environment_path_missing_row_is_exact() {
        let repo = tempdir().unwrap();
        let report = inspect_profile_repository(repo.path(), &resolved_decisions(repo.path()));
        assert_eq!(
            (
                row(&report, "environment_context").status(),
                row(&report, "environment_context").reason(),
            ),
            (
                ArtifactInspectionStatus::Missing,
                ArtifactInspectionReason::OptionalPathMissing,
            )
        );
    }

    #[test]
    fn required_structurally_valid_row_is_exact() {
        let repo = tempdir().unwrap();
        let decisions = resolved_decisions(repo.path());
        write_yaml(repo.path(), ".handbook/project/charter.yaml", authority());
        let report = inspect_profile_repository(repo.path(), &decisions);
        assert_eq!(
            (
                row(&report, "project_authority").status(),
                row(&report, "project_authority").reason(),
            ),
            (
                ArtifactInspectionStatus::StructurallyValid,
                ArtifactInspectionReason::PresentAndStructurallyValid,
            )
        );
    }

    #[test]
    fn optional_structurally_valid_row_is_exact() {
        let repo = tempdir().unwrap();
        let decisions = custom_decisions(repo.path(), 1, RequirednessMode::Optional);
        write_yaml(
            repo.path(),
            decisions.artifact_decisions()[0].canonical_path(),
            authority_v1_0(),
        );
        let report = inspect_profile_repository(repo.path(), &decisions);
        assert_eq!(
            (
                report.artifacts()[0].status(),
                report.artifacts()[0].reason()
            ),
            (
                ArtifactInspectionStatus::StructurallyValid,
                ArtifactInspectionReason::PresentAndStructurallyValid,
            )
        );
    }

    #[test]
    fn shipped_optional_environment_structurally_valid_row_is_exact() {
        let repo = tempdir().unwrap();
        let decisions = resolved_decisions(repo.path());
        write_yaml(
            repo.path(),
            ".handbook/project/environment.yaml",
            environment_context(),
        );
        let report = inspect_profile_repository(repo.path(), &decisions);
        assert_eq!(
            (
                row(&report, "environment_context").status(),
                row(&report, "environment_context").reason(),
            ),
            (
                ArtifactInspectionStatus::StructurallyValid,
                ArtifactInspectionReason::PresentAndStructurallyValid,
            )
        );
    }

    #[test]
    fn yaml_syntax_invalid_row_is_exact() {
        assert_shipped_charter_bytes(
            b"schema_id: [\n",
            ArtifactInspectionStatus::StructurallyInvalid,
            ArtifactInspectionReason::YamlSyntaxInvalid,
        );
    }

    #[test]
    fn duplicate_yaml_key_row_is_exact() {
        assert_shipped_charter_bytes(
            b"schema_id: one\nschema_id: two\n",
            ArtifactInspectionStatus::StructurallyInvalid,
            ArtifactInspectionReason::DuplicateYamlKey,
        );
    }

    #[test]
    fn document_not_object_row_is_exact() {
        assert_shipped_charter_bytes(
            b"- one\n- two\n",
            ArtifactInspectionStatus::StructurallyInvalid,
            ArtifactInspectionReason::DocumentNotObject,
        );
    }

    #[test]
    fn structural_validation_failed_row_is_exact() {
        assert_shipped_charter_bytes(
            b"schema_id: wrong\n",
            ArtifactInspectionStatus::StructurallyInvalid,
            ArtifactInspectionReason::StructuralValidationFailed,
        );
    }

    #[test]
    fn document_limit_exceeded_row_is_exact() {
        let bytes = vec![b'x'; MAX_SOURCE_DOCUMENT_BYTES + 1];
        assert_shipped_charter_bytes(
            &bytes,
            ArtifactInspectionStatus::Unreadable,
            ArtifactInspectionReason::DocumentLimitExceeded,
        );
    }

    #[test]
    fn aggregate_limit_marks_current_and_every_later_row_without_opening() {
        let repo = tempdir().unwrap();
        let decisions = custom_decisions(repo.path(), 10, RequirednessMode::Always);
        for (index, decision) in decisions.artifact_decisions().iter().enumerate() {
            let size = match index {
                0..=6 => MAX_SOURCE_DOCUMENT_BYTES,
                7 => MAX_SOURCE_DOCUMENT_BYTES - 1,
                8 => 2,
                _ => continue,
            };
            write(
                &repo.path().join(decision.canonical_path()),
                vec![b'x'; size],
            );
        }
        let report = inspect_profile_repository(repo.path(), &decisions);
        assert!(report.artifacts()[8..].iter().all(|artifact| {
            artifact.status() == ArtifactInspectionStatus::Unreadable
                && artifact.reason() == ArtifactInspectionReason::AggregateReadLimitExceeded
        }));
    }

    #[test]
    fn final_symlink_refusal_row_is_exact() {
        let repo = tempdir().unwrap();
        let decisions = resolved_decisions(repo.path());
        write(&repo.path().join("target.yaml"), b"{}\n");
        fs::create_dir_all(repo.path().join(".handbook/project")).unwrap();
        symlink(
            repo.path().join("target.yaml"),
            repo.path().join(".handbook/project/charter.yaml"),
        )
        .unwrap();
        let report = inspect_profile_repository(repo.path(), &decisions);
        assert_eq!(
            (
                row(&report, "project_authority").status(),
                row(&report, "project_authority").reason(),
            ),
            (
                ArtifactInspectionStatus::UnsafePath,
                ArtifactInspectionReason::SymlinkRefused,
            )
        );
    }

    #[test]
    fn non_regular_file_refusal_row_is_exact() {
        let repo = tempdir().unwrap();
        let decisions = resolved_decisions(repo.path());
        fs::create_dir_all(repo.path().join(".handbook/project/charter.yaml")).unwrap();
        let report = inspect_profile_repository(repo.path(), &decisions);
        assert_eq!(
            (
                row(&report, "project_authority").status(),
                row(&report, "project_authority").reason(),
            ),
            (
                ArtifactInspectionStatus::UnsafePath,
                ArtifactInspectionReason::NonRegularFileRefused,
            )
        );
    }

    #[test]
    fn repository_read_failure_row_is_exact() {
        let repo = tempdir().unwrap();
        let decisions = resolved_decisions(repo.path());
        let path = repo.path().join(".handbook/project/charter.yaml");
        write(&path, b"{}\n");
        fs::set_permissions(&path, fs::Permissions::from_mode(0o000)).unwrap();
        let report = inspect_profile_repository(repo.path(), &decisions);
        fs::set_permissions(&path, fs::Permissions::from_mode(0o600)).unwrap();
        assert_eq!(
            (
                row(&report, "project_authority").status(),
                row(&report, "project_authority").reason(),
            ),
            (
                ArtifactInspectionStatus::Unreadable,
                ArtifactInspectionReason::RepositoryReadFailed,
            )
        );
    }

    #[test]
    fn optional_requiredness_maps_to_optional_without_condition_truth() {
        let repo = tempdir().unwrap();
        let decisions = custom_decisions(repo.path(), 1, RequirednessMode::Optional);
        let decision = &decisions.artifact_decisions()[0];
        assert_eq!(decision.requiredness_mode(), RequirednessMode::Optional);
        assert_eq!(
            decision.applicability(),
            handbook_engine::ArtifactApplicability::Optional
        );
        assert!(decision.condition_ref().is_none());
        assert!(decision.condition_outcome().is_none());
        assert!(decision.condition_reason().is_none());
        assert!(decision.evidence_closure_fingerprint().is_none());
    }

    #[test]
    fn missing_and_structurally_valid_rows_follow_applicability() {
        let repo = tempdir().unwrap();
        let decisions = resolved_decisions(repo.path());
        let missing = inspect_profile_repository(repo.path(), &decisions);
        assert_eq!(
            (
                row(&missing, "project_authority").status(),
                row(&missing, "project_authority").reason()
            ),
            (
                ArtifactInspectionStatus::Missing,
                ArtifactInspectionReason::RequiredPathMissing
            )
        );
        assert_eq!(
            (
                row(&missing, "environment_context").status(),
                row(&missing, "environment_context").reason()
            ),
            (
                ArtifactInspectionStatus::Missing,
                ArtifactInspectionReason::OptionalPathMissing
            )
        );

        write_yaml(repo.path(), ".handbook/project/charter.yaml", authority());
        write_yaml(
            repo.path(),
            ".handbook/project/context.yaml",
            project_context(),
        );
        write_yaml(
            repo.path(),
            ".handbook/project/environment.yaml",
            environment_context(),
        );
        let present = inspect_profile_repository(repo.path(), &decisions);
        assert_eq!(
            (
                row(&present, "project_authority").status(),
                row(&present, "project_authority").reason()
            ),
            (
                ArtifactInspectionStatus::StructurallyValid,
                ArtifactInspectionReason::PresentAndStructurallyValid
            )
        );
        assert_eq!(
            (
                row(&present, "environment_context").status(),
                row(&present, "environment_context").reason()
            ),
            (
                ArtifactInspectionStatus::StructurallyValid,
                ArtifactInspectionReason::PresentAndStructurallyValid
            )
        );
    }

    #[test]
    fn parser_and_structural_failures_remain_distinct() {
        let cases: &[(&[u8], ArtifactInspectionReason)] = &[
            (
                b"schema_id: [\n",
                ArtifactInspectionReason::YamlSyntaxInvalid,
            ),
            (
                b"schema_id: one\nschema_id: two\n",
                ArtifactInspectionReason::DuplicateYamlKey,
            ),
            (
                b"- not\n- object\n",
                ArtifactInspectionReason::DocumentNotObject,
            ),
            (
                b"schema_id: wrong\n",
                ArtifactInspectionReason::StructuralValidationFailed,
            ),
        ];
        for (bytes, expected) in cases {
            let repo = tempdir().unwrap();
            let decisions = resolved_decisions(repo.path());
            let path = repo.path().join(".handbook/project/charter.yaml");
            fs::create_dir_all(path.parent().unwrap()).unwrap();
            fs::write(path, bytes).unwrap();
            let report = inspect_profile_repository(repo.path(), &decisions);
            assert_eq!(row(&report, "project_authority").reason(), *expected);
            assert_eq!(
                row(&report, "project_authority").status(),
                ArtifactInspectionStatus::StructurallyInvalid
            );
        }
    }

    #[test]
    fn size_directory_and_symlink_refusals_are_typed() {
        let oversized = tempdir().unwrap();
        let decisions = resolved_decisions(oversized.path());
        let path = oversized.path().join(".handbook/project/charter.yaml");
        fs::create_dir_all(path.parent().unwrap()).unwrap();
        fs::write(&path, vec![b'x'; MAX_SOURCE_DOCUMENT_BYTES + 1]).unwrap();
        let report = inspect_profile_repository(oversized.path(), &decisions);
        assert_eq!(
            row(&report, "project_authority").reason(),
            ArtifactInspectionReason::DocumentLimitExceeded
        );

        let directory = tempdir().unwrap();
        let decisions = resolved_decisions(directory.path());
        fs::create_dir_all(directory.path().join(".handbook/project/charter.yaml")).unwrap();
        let report = inspect_profile_repository(directory.path(), &decisions);
        assert_eq!(
            row(&report, "project_authority").reason(),
            ArtifactInspectionReason::NonRegularFileRefused
        );

        let linked = tempdir().unwrap();
        let decisions = resolved_decisions(linked.path());
        fs::create_dir_all(linked.path().join(".handbook/project")).unwrap();
        fs::write(linked.path().join("target.yaml"), b"{}\n").unwrap();
        symlink(
            linked.path().join("target.yaml"),
            linked.path().join(".handbook/project/charter.yaml"),
        )
        .unwrap();
        let report = inspect_profile_repository(linked.path(), &decisions);
        assert_eq!(
            row(&report, "project_authority").reason(),
            ArtifactInspectionReason::SymlinkRefused
        );
    }

    #[test]
    fn custom_profile_kind_and_instance_are_inspected_without_fixed_variants() {
        let repo = tempdir().unwrap();
        let decisions = custom_decisions(repo.path(), 1, RequirednessMode::Always);
        let decision = &decisions.artifact_decisions()[0];
        assert_eq!(decision.instance_id().as_str(), "bulk_00");
        assert_eq!(
            decision.kind_ref().as_str(),
            "example.artifact-kind.bulk-record@1.0.0"
        );
        write_yaml(repo.path(), decision.canonical_path(), authority_v1_0());

        let report = inspect_profile_repository(repo.path(), &decisions);
        assert_eq!(report.artifacts().len(), 2);
        assert_eq!(
            (
                report.artifacts()[0].status(),
                report.artifacts()[0].reason()
            ),
            (
                ArtifactInspectionStatus::StructurallyValid,
                ArtifactInspectionReason::PresentAndStructurallyValid,
            )
        );
    }

    #[test]
    fn document_and_aggregate_byte_boundaries_are_exact() {
        for (size, expected_reason) in [
            (0, ArtifactInspectionReason::DocumentNotObject),
            (1, ArtifactInspectionReason::DocumentNotObject),
            (
                MAX_SOURCE_DOCUMENT_BYTES,
                ArtifactInspectionReason::DocumentNotObject,
            ),
            (
                MAX_SOURCE_DOCUMENT_BYTES + 1,
                ArtifactInspectionReason::DocumentLimitExceeded,
            ),
        ] {
            let repo = tempdir().unwrap();
            let decisions = custom_decisions(repo.path(), 1, RequirednessMode::Always);
            write(
                &repo
                    .path()
                    .join(decisions.artifact_decisions()[0].canonical_path()),
                vec![b'x'; size],
            );
            let report = inspect_profile_repository(repo.path(), &decisions);
            assert_eq!(
                report.artifacts()[0].reason(),
                expected_reason,
                "size={size}"
            );
        }

        let exact = tempdir().unwrap();
        let exact_decisions = custom_decisions(exact.path(), 7, RequirednessMode::Always);
        for decision in exact_decisions.artifact_decisions() {
            write(
                &exact.path().join(decision.canonical_path()),
                vec![b'x'; MAX_SOURCE_DOCUMENT_BYTES],
            );
        }
        let exact_report = inspect_profile_repository(exact.path(), &exact_decisions);
        assert_eq!(
            exact_decisions.artifact_decisions().len() * MAX_SOURCE_DOCUMENT_BYTES,
            MAX_TOTAL_SOURCE_BYTES
        );
        let exact_reasons = exact_report
            .artifacts()
            .iter()
            .map(|artifact| (artifact.instance_id().as_str(), artifact.reason()))
            .collect::<Vec<_>>();
        assert_eq!(
            exact_reasons,
            [
                ("bulk_00", ArtifactInspectionReason::DocumentNotObject),
                ("bulk_01", ArtifactInspectionReason::DocumentNotObject),
                ("bulk_02", ArtifactInspectionReason::DocumentNotObject),
                ("bulk_03", ArtifactInspectionReason::DocumentNotObject),
                ("bulk_04", ArtifactInspectionReason::DocumentNotObject),
                ("bulk_05", ArtifactInspectionReason::DocumentNotObject),
                ("bulk_06", ArtifactInspectionReason::DocumentNotObject),
                (
                    "project_authority",
                    ArtifactInspectionReason::StructuralValidationFailed,
                ),
            ]
        );

        let exceeded = tempdir().unwrap();
        let exceeded_decisions = custom_decisions(exceeded.path(), 9, RequirednessMode::Always);
        for (index, decision) in exceeded_decisions.artifact_decisions().iter().enumerate() {
            let size = match index {
                0..=6 => MAX_SOURCE_DOCUMENT_BYTES,
                7 => MAX_SOURCE_DOCUMENT_BYTES - 1,
                _ => 2,
            };
            write(
                &exceeded.path().join(decision.canonical_path()),
                vec![b'x'; size],
            );
        }
        let exceeded_report = inspect_profile_repository(exceeded.path(), &exceeded_decisions);
        assert!(
            exceeded_report.artifacts()[8..].iter().all(|artifact| {
                artifact.reason() == ArtifactInspectionReason::AggregateReadLimitExceeded
            }),
            "aggregate-exceeded rows: {:?}",
            &exceeded_report.artifacts()[8..]
        );
    }

    #[test]
    fn failed_project_context_reads_charge_aggregate_budget_before_later_rows() {
        for (project_context_bytes, expected_reason) in [
            (
                vec![b'x'; MAX_SOURCE_DOCUMENT_BYTES],
                ArtifactInspectionReason::DocumentNotObject,
            ),
            (
                vec![b'x'; MAX_SOURCE_DOCUMENT_BYTES + 1],
                ArtifactInspectionReason::DocumentLimitExceeded,
            ),
        ] {
            let repo = tempdir().unwrap();
            let decisions =
                custom_decisions_with_project_context(repo.path(), 8, RequirednessMode::Always);
            assert_eq!(
                decisions.artifact_decisions()[0].instance_id().as_str(),
                "project_context"
            );
            write(
                &repo.path().join(".handbook/project/context.yaml"),
                project_context_bytes,
            );
            for decision in decisions
                .artifact_decisions()
                .iter()
                .filter(|decision| decision.instance_id().as_str().starts_with("zz_bulk_"))
            {
                write(
                    &repo.path().join(decision.canonical_path()),
                    vec![b'x'; MAX_SOURCE_DOCUMENT_BYTES],
                );
            }

            let report = inspect_profile_repository(repo.path(), &decisions);
            assert_eq!(row(&report, "project_context").reason(), expected_reason);
            assert_eq!(
                row(&report, "zz_bulk_06").reason(),
                ArtifactInspectionReason::DocumentNotObject
            );
            assert_eq!(
                row(&report, "zz_bulk_07").reason(),
                ArtifactInspectionReason::AggregateReadLimitExceeded
            );
            assert_eq!(
                row(&report, "zz_project_authority").reason(),
                ArtifactInspectionReason::AggregateReadLimitExceeded
            );
        }
    }

    #[test]
    fn optional_unreadable_and_intermediate_symlink_rows_are_typed() {
        let optional = tempdir().unwrap();
        let optional_decisions = custom_decisions(optional.path(), 1, RequirednessMode::Optional);
        let optional_report = inspect_profile_repository(optional.path(), &optional_decisions);
        assert_eq!(
            (
                optional_report.artifacts()[0].status(),
                optional_report.artifacts()[0].reason()
            ),
            (
                ArtifactInspectionStatus::Missing,
                ArtifactInspectionReason::OptionalPathMissing,
            )
        );

        let unreadable = tempdir().unwrap();
        let unreadable_decisions = custom_decisions(unreadable.path(), 1, RequirednessMode::Always);
        let unreadable_path = unreadable
            .path()
            .join(unreadable_decisions.artifact_decisions()[0].canonical_path());
        write(&unreadable_path, b"{}\n");
        fs::set_permissions(&unreadable_path, fs::Permissions::from_mode(0o000)).unwrap();
        let unreadable_report =
            inspect_profile_repository(unreadable.path(), &unreadable_decisions);
        fs::set_permissions(&unreadable_path, fs::Permissions::from_mode(0o600)).unwrap();
        assert_eq!(
            (
                unreadable_report.artifacts()[0].status(),
                unreadable_report.artifacts()[0].reason()
            ),
            (
                ArtifactInspectionStatus::Unreadable,
                ArtifactInspectionReason::RepositoryReadFailed,
            )
        );

        let linked = tempdir().unwrap();
        let linked_decisions = custom_decisions(linked.path(), 1, RequirednessMode::Always);
        let outside = linked.path().join("outside");
        fs::create_dir_all(&outside).unwrap();
        write(&outside.join("bulk-00.yaml"), b"{}\n");
        fs::create_dir_all(linked.path().join(".handbook")).unwrap();
        symlink(&outside, linked.path().join(".handbook/project")).unwrap();
        let linked_report = inspect_profile_repository(linked.path(), &linked_decisions);
        assert_eq!(
            (
                linked_report.artifacts()[0].status(),
                linked_report.artifacts()[0].reason()
            ),
            (
                ArtifactInspectionStatus::UnsafePath,
                ArtifactInspectionReason::SymlinkRefused,
            )
        );
    }

    const ROLE_REGISTRY_REF: &str = "handbook.roles.core@1.1.0";
    const ROLE_REGISTRY_FINGERPRINT: &str =
        "sha256:0c85b1b53786e7980c4fd0d7975cd9cde1a3eae2bc8daceb23be1a1731263029";

    fn exact(value: &str) -> ExactDefinitionRef {
        ExactDefinitionRef::parse(value).unwrap()
    }

    fn builtin(value: &str) -> DefinitionSourceBinding {
        let definition_ref = exact(value);
        DefinitionSourceBinding {
            definition_ref: definition_ref.clone(),
            source: DefinitionSource::BuiltIn(definition_ref),
        }
    }

    fn repository(value: &str, path: &str) -> DefinitionSourceBinding {
        DefinitionSourceBinding {
            definition_ref: exact(value),
            source: DefinitionSource::RepositoryPath(path.to_owned()),
        }
    }

    fn write(path: &Path, bytes: impl AsRef<[u8]>) {
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).unwrap();
        }
        fs::write(path, bytes).unwrap();
    }

    fn copy_crate_file(repo: &Path, relative: &str) {
        write(
            &repo.join(relative),
            fs::read(Path::new(env!("CARGO_MANIFEST_DIR")).join(relative)).unwrap(),
        );
    }

    fn fingerprint(value: &Value) -> String {
        DefinitionFingerprint::from_json_value(value)
            .unwrap()
            .to_string()
    }

    fn custom_decisions(
        repo: &Path,
        count: usize,
        requiredness: RequirednessMode,
    ) -> ResolvedProfileDecisions {
        custom_decisions_inner(repo, count, requiredness, false)
    }

    fn custom_decisions_with_project_context(
        repo: &Path,
        count: usize,
        requiredness: RequirednessMode,
    ) -> ResolvedProfileDecisions {
        custom_decisions_inner(repo, count, requiredness, true)
    }

    fn custom_decisions_inner(
        repo: &Path,
        count: usize,
        requiredness: RequirednessMode,
        include_project_context: bool,
    ) -> ResolvedProfileDecisions {
        let schema_entry_path =
            "definitions/schemas/handbook.schemas.artifacts.project-authority/1.0.0.entry.yaml";
        let schema_document_path =
            "definitions/schemas/handbook.schemas.artifacts.project-authority/1.0.0.schema.json";
        let project_context_schema_entry_path =
            "definitions/schemas/handbook.schemas.artifacts.project-context/1.0.0.entry.yaml";
        let project_context_schema_document_path =
            "definitions/schemas/handbook.schemas.artifacts.project-context/1.0.0.schema.json";
        let authority_kind_path =
            "definitions/artifact-kinds/handbook.artifact-kind.project-authority/1.0.0.yaml";
        let project_context_kind_path =
            "definitions/artifact-kinds/handbook.artifact-kind.project-context/1.0.0.yaml";
        let capability_path = "definitions/semantic-capabilities/handbook.capabilities.constitutional-root/1.0.0.yaml";
        let validator_path = "definitions/semantic-validators/handbook.semantic-validation.constitutional-root/1.0.0.yaml";
        copy_crate_file(repo, schema_entry_path);
        copy_crate_file(repo, schema_document_path);
        copy_crate_file(repo, authority_kind_path);
        copy_crate_file(repo, capability_path);
        copy_crate_file(repo, validator_path);
        if include_project_context {
            copy_crate_file(repo, project_context_schema_entry_path);
            copy_crate_file(repo, project_context_schema_document_path);
            copy_crate_file(repo, project_context_kind_path);
        }

        let baseline = shipped_selection();
        let schema_ref = exact("handbook.schemas.artifacts.project-authority@1.0.0");
        let project_context_schema_ref = exact("handbook.schemas.artifacts.project-context@1.0.0");
        let mut schema_entry_paths = vec![schema_entry_path.to_owned()];
        if include_project_context {
            schema_entry_paths.push(project_context_schema_entry_path.to_owned());
        }
        let schema_registry = SchemaRegistry::load(
            repo,
            &schema_entry_paths,
            &["definitions/schemas".to_owned()],
        )
        .unwrap();
        let schema_entry = schema_registry.entry(&schema_ref).unwrap();

        let kind_ref = "example.artifact-kind.bulk-record@1.0.0";
        let kind_path = "sources/bulk-record.kind.yaml";
        let mut kind = json!({
            "schema_id": "handbook.artifact-kind-definition",
            "schema_version": "1.0",
            "kind_id": "example.artifact-kind.bulk-record",
            "kind_version": "1.0.0",
            "compatibility": "exact",
            "stable_role_registry": {
                "ref": ROLE_REGISTRY_REF,
                "fingerprint": ROLE_REGISTRY_FINGERPRINT,
            },
            "canonical_schema_ref": schema_ref.as_str(),
            "supported_role_refs": ["project_context"],
            "semantic_capabilities": [],
            "structural_validation_profile_ref": "json-schema.draft-2020-12",
            "semantic_validation_profile_refs": [],
            "renderer_definition_refs": [],
            "projection_definition_refs": [],
            "lifecycle_policy_ref": null,
            "review_triggers": [],
            "required_capabilities": [],
            "extensions": {},
        });
        let kind_fingerprint = fingerprint(&json!({
            "definition": kind,
            "stable_role_registry_fingerprint": ROLE_REGISTRY_FINGERPRINT,
            "schema_entry_fingerprint": schema_entry.entry_fingerprint().as_str(),
            "schema_closure_fingerprint": schema_entry.closure_fingerprint().as_str(),
        }));
        kind["definition_fingerprint"] = Value::String(kind_fingerprint.clone());
        write(
            &kind_path_at(repo),
            serde_yaml_bw::to_string(&kind).unwrap(),
        );

        let mut artifact_kind_paths = vec![authority_kind_path.to_owned(), kind_path.to_owned()];
        if include_project_context {
            artifact_kind_paths.push(project_context_kind_path.to_owned());
        }
        let kind_registry = load_artifact_kind_registry(
            repo,
            ArtifactKindRegistryLoadRequest::new(
                exact(ROLE_REGISTRY_REF),
                schema_entry_paths,
                vec!["definitions/schemas".to_owned()],
                artifact_kind_paths,
            )
            .with_semantic_sources(
                vec![capability_path.to_owned()],
                vec![validator_path.to_owned()],
            ),
        )
        .unwrap();

        let requiredness = match requiredness {
            RequirednessMode::Always => json!({"mode": "always", "condition_ref": null}),
            RequirednessMode::Optional => json!({"mode": "optional", "condition_ref": null}),
            RequirednessMode::Conditional => unreachable!("custom helper has no condition source"),
        };
        let mut instances = (0..count)
            .map(|index| {
                let instance_id = if include_project_context {
                    format!("zz_bulk_{index:02}")
                } else {
                    format!("bulk_{index:02}")
                };
                json!({
                    "schema_id": "handbook.artifact-instance-descriptor",
                    "schema_version": "1.0",
                    "id": instance_id,
                    "kind_ref": kind_ref,
                    "role_ref": "project_context",
                    "capability_refs": [],
                    "label": format!("Bulk Record {index:02}"),
                    "canonical_path": format!(".handbook/project/bulk-{index:02}.yaml"),
                    "requiredness": requiredness,
                    "depends_on": if include_project_context {
                        vec![json!({
                            "target_kind": "instance",
                            "target_ref": "project_context",
                            "target_contract_ref": null,
                            "cardinality": "exactly_one"
                        })]
                    } else {
                        Vec::<Value>::new()
                    },
                    "lifecycle_policy_ref": null,
                    "intake_definition_ref": null,
                    "renderer_definition_refs": [],
                    "projection_definition_refs": [],
                    "validation_overlay_refs": [],
                    "extensions": {},
                })
            })
            .collect::<Vec<_>>();
        if include_project_context {
            instances.insert(
                0,
                shipped_root_artifact_instance_values()
                    .into_iter()
                    .find(|value| value["id"] == json!("project_context"))
                    .unwrap(),
            );
        }
        let mut authority_instance = shipped_root_artifact_instance_values()
            .into_iter()
            .find(|value| value["id"] == json!("project_authority"))
            .unwrap();
        if include_project_context {
            authority_instance["id"] = json!("zz_project_authority");
            authority_instance["canonical_path"] =
                json!(".handbook/project/zz-project-authority.yaml");
        }
        instances.push(authority_instance);
        let instance_registry =
            ArtifactInstanceRegistry::resolve(&instances, &kind_registry, &[]).unwrap();

        let mut profile_schema_refs = vec![schema_ref.as_str().to_owned()];
        let mut profile_kind_refs = vec![
            "handbook.artifact-kind.project-authority@1.0.0".to_owned(),
            kind_ref.to_owned(),
        ];
        if include_project_context {
            profile_schema_refs.push(project_context_schema_ref.as_str().to_owned());
            profile_kind_refs.push("handbook.artifact-kind.project-context@1.0.0".to_owned());
        }
        let mut profile = json!({
            "schema_id": "handbook.instance-profile",
            "schema_version": "1.0",
            "profile_id": "example.profile.bulk-root",
            "profile_version": "1.0.0",
            "profile_scope": "shipped",
            "extends_profile_ref": null,
            "stable_role_registry": {
                "ref": ROLE_REGISTRY_REF,
                "fingerprint": ROLE_REGISTRY_FINGERPRINT,
            },
            "schema_registry_sources": profile_schema_refs,
            "artifact_kind_sources": profile_kind_refs,
            "artifact_instances": instances,
            "vocabulary_ref": baseline.vocabulary().exact_ref().as_str(),
            "context_resolution_ref": baseline.context_resolution().exact_ref().as_str(),
            "projection_catalog_refs": [],
            "posture_evaluation_policy": null,
            "dock_requirement_refs": [],
            "adapter_overlay_refs": [],
            "extensions": {},
            "profile_fingerprint": "sha256:0000000000000000000000000000000000000000000000000000000000000000",
        });
        let mut dependencies = vec![
            json!({
                "definition_class": "stable_role_registry",
                "reference": ROLE_REGISTRY_REF,
                "fingerprint": ROLE_REGISTRY_FINGERPRINT,
            }),
            json!({
                "definition_class": "schema_entry",
                "reference": schema_ref.as_str(),
                "fingerprint": schema_entry.entry_fingerprint().as_str(),
            }),
            json!({
                "definition_class": "artifact_kind",
                "reference": kind_ref,
                "fingerprint": kind_fingerprint,
            }),
            json!({
                "definition_class": "artifact_kind",
                "reference": "handbook.artifact-kind.project-authority@1.0.0",
                "fingerprint": baseline
                    .artifact_kind_registry()
                    .kind(&exact("handbook.artifact-kind.project-authority@1.0.0"))
                    .unwrap()
                    .definition_fingerprint()
                    .as_str(),
            }),
            json!({
                "definition_class": "artifact_instance_registry",
                "reference": "example.profile.bulk-root@1.0.0",
                "fingerprint": instance_registry.fingerprint().as_str(),
            }),
            json!({
                "definition_class": "vocabulary",
                "reference": baseline.vocabulary().exact_ref().as_str(),
                "fingerprint": baseline.vocabulary().vocabulary_fingerprint().as_str(),
            }),
            json!({
                "definition_class": "context_resolution",
                "reference": baseline.context_resolution().exact_ref().as_str(),
                "fingerprint": baseline.context_resolution().definition_fingerprint().as_str(),
            }),
        ];
        if include_project_context {
            let project_context_schema_entry = schema_registry
                .entry(&project_context_schema_ref)
                .expect("Project Context schema entry");
            dependencies.push(json!({
                "definition_class": "schema_entry",
                "reference": project_context_schema_ref.as_str(),
                "fingerprint": project_context_schema_entry.entry_fingerprint().as_str(),
            }));
            dependencies.push(json!({
                "definition_class": "artifact_kind",
                "reference": "handbook.artifact-kind.project-context@1.0.0",
                "fingerprint": baseline
                    .artifact_kind_registry()
                    .kind(&exact("handbook.artifact-kind.project-context@1.0.0"))
                    .unwrap()
                    .definition_fingerprint()
                    .as_str(),
            }));
        }
        dependencies.sort_by(|left, right| {
            (
                left["definition_class"].as_str(),
                left["reference"].as_str(),
            )
                .cmp(&(
                    right["definition_class"].as_str(),
                    right["reference"].as_str(),
                ))
        });
        let mut definition = profile.clone();
        definition
            .as_object_mut()
            .unwrap()
            .remove("profile_fingerprint");
        profile["profile_fingerprint"] = Value::String(fingerprint(&json!({
            "definition": definition,
            "dependencies": dependencies,
        })));
        let profile_path = "sources/bulk-root.profile.yaml";
        write(
            &repo.join(profile_path),
            serde_yaml_bw::to_string(&profile).unwrap(),
        );

        let mut selected_schema_sources = vec![builtin(schema_ref.as_str())];
        let mut selected_kind_sources = vec![
            builtin("handbook.artifact-kind.project-authority@1.0.0"),
            repository(kind_ref, kind_path),
        ];
        if include_project_context {
            selected_schema_sources.push(builtin(project_context_schema_ref.as_str()));
            selected_kind_sources.push(builtin("handbook.artifact-kind.project-context@1.0.0"));
        }
        let selected = resolve_profile_selection(
            repo,
            ProfileSelectionRequest {
                selected_profile_ref: exact("example.profile.bulk-root@1.0.0"),
                profile_sources: vec![repository("example.profile.bulk-root@1.0.0", profile_path)],
                stable_role_registry_sources: vec![builtin(ROLE_REGISTRY_REF)],
                schema_entry_sources: selected_schema_sources,
                artifact_kind_sources: selected_kind_sources,
                semantic_capability_sources: vec![builtin(
                    "handbook.capabilities.constitutional-root@1.0.0",
                )],
                semantic_validator_sources: vec![builtin(
                    "handbook.semantic-validation.constitutional-root@1.0.0",
                )],
                project_condition_sources: vec![],
                vocabulary_sources: vec![builtin("handbook.vocabulary.shipped-root@1.0.0")],
                context_resolution_sources: vec![builtin(
                    "handbook.context-resolution.shipped-root@1.0.0",
                )],
                context_resolution_policy_sources: vec![
                    builtin("handbook.mutation-matcher.core@1.0.0"),
                    builtin("handbook.resolution-escalation.core@1.0.0"),
                    builtin("handbook.memory-promotion.core@1.0.0"),
                ],
                intake_definition_sources: vec![],
                allowed_schema_roots: vec!["definitions/schemas".to_owned()],
            },
        )
        .unwrap();
        ResolvedProfileDecisions::from_profile(&selected).unwrap()
    }

    fn shipped_selection() -> handbook_engine::ResolvedInstanceProfile {
        let artifact_names = [
            "project-authority",
            "project-context",
            "environment-context",
            "work-specification",
            "decision-record",
            "risk-record",
        ];
        resolve_profile_selection(
            Path::new(env!("CARGO_MANIFEST_DIR")),
            ProfileSelectionRequest {
                selected_profile_ref: exact("handbook.profile.shipped-root@1.0.0"),
                profile_sources: vec![builtin("handbook.profile.shipped-root@1.0.0")],
                stable_role_registry_sources: vec![builtin(ROLE_REGISTRY_REF)],
                schema_entry_sources: artifact_names
                    .iter()
                    .map(|name| builtin(&format!("handbook.schemas.artifacts.{name}@1.0.0")))
                    .collect(),
                artifact_kind_sources: artifact_names
                    .iter()
                    .map(|name| builtin(&format!("handbook.artifact-kind.{name}@1.0.0")))
                    .collect(),
                semantic_capability_sources: vec![builtin(
                    "handbook.capabilities.constitutional-root@1.0.0",
                )],
                semantic_validator_sources: vec![builtin(
                    "handbook.semantic-validation.constitutional-root@1.0.0",
                )],
                project_condition_sources: vec![builtin(
                    "handbook.condition.project.managed-operational-surface@1.0.0",
                )],
                vocabulary_sources: vec![builtin("handbook.vocabulary.shipped-root@1.0.0")],
                context_resolution_sources: vec![builtin(
                    "handbook.context-resolution.shipped-root@1.0.0",
                )],
                context_resolution_policy_sources: vec![
                    builtin("handbook.mutation-matcher.core@1.0.0"),
                    builtin("handbook.resolution-escalation.core@1.0.0"),
                    builtin("handbook.memory-promotion.core@1.0.0"),
                ],
                intake_definition_sources: vec![],
                allowed_schema_roots: vec!["definitions/schemas".to_owned()],
            },
        )
        .unwrap()
    }

    fn kind_path_at(repo: &Path) -> std::path::PathBuf {
        repo.join("sources/bulk-record.kind.yaml")
    }
}
