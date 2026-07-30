#[cfg(unix)]
use handbook_engine::{
    inspect_profile_repository_with_stability_hook, load_selected_project_context,
    ArtifactInspectionReason, ArtifactInspectionStatus,
};
use handbook_engine::{
    parse_canonical_project_context, render_project_context_markdown,
    resolve_shipped_profile_decisions, serialize_canonical_project_context, ArtifactPresence,
    CanonicalArtifacts, CanonicalProjectContext, DefinitionFingerprint,
    ProjectContextArtifactErrorKind, SystemRootStatus, MAX_SOURCE_DOCUMENT_BYTES,
};
use std::path::Path;
#[cfg(unix)]
use std::path::PathBuf;

const BOUNDARY_YAML: &str = concat!(
    "schema_id: \"handbook.artifact.project-context\"\n",
    "schema_version: \"1.0\"\n",
    "record_id: \"handbook.project-context\"\n",
    "summary: \"Coordinates \\\"alpha\\\"\\nboundary #1.\"\n",
    "system_boundaries:\n",
    "  - \"API -> worker\"\n",
    "  - \"No <external> writes\"\n",
    "ownership:\n",
    "  - \"Platform/Ops\"\n",
    "authoritative_references:\n",
    "  - \"handbook.project-context@1.0.0\"\n",
    "known_unknowns:\n",
    "  - \"Which region?\\tOwner TBD\"\n",
);

const BOUNDARY_MARKDOWN: &str = concat!(
    "# Project Context\n\n",
    "## Summary\n\n",
    "Coordinates \\\"alpha\\\" boundary \\#1\\.\n\n",
    "## System Boundaries\n\n",
    "- API \\-\\> worker\n",
    "- No \\<external\\> writes\n\n",
    "## Ownership\n\n",
    "- Platform\\/Ops\n\n",
    "## Authoritative References\n\n",
    "- handbook\\.project\\-context\\@1\\.0\\.0\n\n",
    "## Known Unknowns\n\n",
    "- Which region\\? Owner TBD\n",
);

fn decisions() -> handbook_engine::ResolvedProfileDecisions {
    resolve_shipped_profile_decisions(Path::new(env!("CARGO_MANIFEST_DIR")))
        .expect("shipped decisions")
}

fn write(path: &Path, bytes: &[u8]) {
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent).expect("create parent");
    }
    std::fs::write(path, bytes).expect("write fixture");
}

#[cfg(unix)]
fn selected_path(root: &Path) -> PathBuf {
    root.join(".handbook/project/context.yaml")
}

#[cfg(unix)]
fn project_context_row(
    report: &handbook_engine::ProfileInspectionReport,
) -> &handbook_engine::ArtifactInspection {
    report
        .artifacts()
        .iter()
        .find(|artifact| artifact.instance_id().as_str() == "project_context")
        .expect("Project Context row")
}

fn record_with_summary(summary: &str) -> CanonicalProjectContext {
    CanonicalProjectContext {
        schema_id: "handbook.artifact.project-context".to_owned(),
        schema_version: "1.0".to_owned(),
        record_id: "handbook.project-context".to_owned(),
        summary: summary.to_owned(),
        system_boundaries: vec!["Boundary".to_owned()],
        ownership: vec!["Owner".to_owned()],
        authoritative_references: vec![],
        known_unknowns: vec![],
    }
}

fn rendered_summary(summary: &str) -> Result<String, ProjectContextArtifactErrorKind> {
    let rendered = render_project_context_markdown(&record_with_summary(summary))
        .map_err(|error| error.kind())?;
    let rendered = String::from_utf8(rendered).expect("UTF-8 Markdown");
    Ok(rendered
        .split("## Summary\n\n")
        .nth(1)
        .expect("summary section")
        .split("\n\n## System Boundaries")
        .next()
        .expect("summary paragraph")
        .to_owned())
}

#[test]
fn canonical_project_context_roundtrips_exact_boundary_bytes() {
    let record = parse_canonical_project_context(&decisions(), BOUNDARY_YAML.as_bytes())
        .expect("canonical Project Context");

    assert_eq!(record.schema_id, "handbook.artifact.project-context");
    assert_eq!(record.schema_version, "1.0");
    assert_eq!(record.record_id, "handbook.project-context");
    assert_eq!(record.summary, "Coordinates \"alpha\"\nboundary #1.");
    assert_eq!(
        record.system_boundaries,
        ["API -> worker", "No <external> writes"]
    );
    assert_eq!(record.ownership, ["Platform/Ops"]);
    assert_eq!(
        record.authoritative_references,
        ["handbook.project-context@1.0.0"]
    );
    assert_eq!(record.known_unknowns, ["Which region?\tOwner TBD"]);
    assert_eq!(
        serialize_canonical_project_context(&decisions(), &record).expect("closed YAML"),
        BOUNDARY_YAML.as_bytes()
    );
    assert_eq!(
        render_project_context_markdown(&record).expect("fixed Markdown view"),
        BOUNDARY_MARKDOWN.as_bytes()
    );
}

#[test]
fn empty_optional_lists_have_fixed_yaml_and_markdown_forms() {
    let record = CanonicalProjectContext {
        schema_id: "handbook.artifact.project-context".to_owned(),
        schema_version: "1.0".to_owned(),
        record_id: "handbook.project-context".to_owned(),
        summary: "Owned by Platform".to_owned(),
        system_boundaries: vec!["API".to_owned()],
        ownership: vec!["Platform".to_owned()],
        authoritative_references: vec![],
        known_unknowns: vec![],
    };

    let yaml = serialize_canonical_project_context(&decisions(), &record).expect("closed YAML");
    assert!(String::from_utf8(yaml.clone())
        .unwrap()
        .contains("authoritative_references: []\nknown_unknowns: []\n"));
    let reparsed = parse_canonical_project_context(&decisions(), &yaml).expect("reparse");
    let markdown = render_project_context_markdown(&reparsed).expect("fixed view");
    assert!(String::from_utf8(markdown)
        .unwrap()
        .contains("## Authoritative References\n\n- None recorded.\n\n## Known Unknowns\n\n- None recorded.\n"));
}

#[test]
fn source_and_rendered_fingerprint_domains_remain_distinct() {
    let equivalent = BOUNDARY_YAML.replacen(
        "schema_id: \"handbook.artifact.project-context\"",
        "schema_id: handbook.artifact.project-context",
        1,
    );
    let canonical = parse_canonical_project_context(&decisions(), BOUNDARY_YAML.as_bytes())
        .expect("canonical source");
    let alternate = parse_canonical_project_context(&decisions(), equivalent.as_bytes())
        .expect("equivalent source");

    assert_ne!(
        DefinitionFingerprint::from_bytes(BOUNDARY_YAML.as_bytes()),
        DefinitionFingerprint::from_bytes(equivalent.as_bytes())
    );
    assert_eq!(
        DefinitionFingerprint::from_bytes(
            &render_project_context_markdown(&canonical).expect("canonical view")
        ),
        DefinitionFingerprint::from_bytes(
            &render_project_context_markdown(&alternate).expect("alternate view")
        )
    );
}

#[test]
fn parser_refuses_duplicate_keys_non_objects_and_unknown_fields() {
    let duplicate = BOUNDARY_YAML.replacen(
        "schema_version: \"1.0\"",
        "schema_version: \"1.0\"\nschema_version: \"1.0\"",
        1,
    );
    assert_eq!(
        parse_canonical_project_context(&decisions(), duplicate.as_bytes())
            .unwrap_err()
            .kind(),
        ProjectContextArtifactErrorKind::DuplicateKey
    );
    assert_eq!(
        parse_canonical_project_context(&decisions(), b"- not\n- an\n- object\n")
            .unwrap_err()
            .kind(),
        ProjectContextArtifactErrorKind::NonObjectRoot
    );
    let unknown = BOUNDARY_YAML.replace(
        "known_unknowns:\n",
        "unexpected: \"closed\"\nknown_unknowns:\n",
    );
    assert_eq!(
        parse_canonical_project_context(&decisions(), unknown.as_bytes())
            .unwrap_err()
            .kind(),
        ProjectContextArtifactErrorKind::StructuralValidationFailed
    );
}

#[test]
fn renderer_refuses_forbidden_controls() {
    let mut record = parse_canonical_project_context(&decisions(), BOUNDARY_YAML.as_bytes())
        .expect("canonical Project Context");
    record.summary = "control\u{7f}character".to_owned();

    assert_eq!(
        render_project_context_markdown(&record).unwrap_err().kind(),
        ProjectContextArtifactErrorKind::RenderedViewRefused
    );
}

#[test]
fn parser_refuses_nested_duplicates_multiple_documents_and_closed_shape_failures() {
    let nested_duplicate = BOUNDARY_YAML.replace(
        "ownership:\n  - \"Platform/Ops\"",
        "ownership:\n  - nested: \"first\"\n    nested: \"second\"",
    );
    assert_eq!(
        parse_canonical_project_context(&decisions(), nested_duplicate.as_bytes())
            .unwrap_err()
            .kind(),
        ProjectContextArtifactErrorKind::DuplicateKey
    );

    let multiple_documents = format!("{BOUNDARY_YAML}---\n{BOUNDARY_YAML}");
    assert_eq!(
        parse_canonical_project_context(&decisions(), multiple_documents.as_bytes())
            .unwrap_err()
            .kind(),
        ProjectContextArtifactErrorKind::SyntaxError
    );

    for invalid in [
        BOUNDARY_YAML.replacen("schema_id: \"handbook.artifact.project-context\"\n", "", 1),
        BOUNDARY_YAML.replacen(
            "schema_id: \"handbook.artifact.project-context\"",
            "schema_id: \"handbook.artifact.other\"",
            1,
        ),
        BOUNDARY_YAML.replacen("summary: \"Coordinates", "summary: 42 # Coordinates", 1),
        BOUNDARY_YAML.replacen("handbook.project-context@1.0.0", "Not A Stable Ref", 1),
    ] {
        assert_eq!(
            parse_canonical_project_context(&decisions(), invalid.as_bytes())
                .unwrap_err()
                .kind(),
            ProjectContextArtifactErrorKind::StructuralValidationFailed
        );
    }
}

#[test]
fn parser_refuses_schema_bounds_and_source_limit() {
    let long_summary = BOUNDARY_YAML.replacen(
        "Coordinates \\\"alpha\\\"\\nboundary #1.",
        &"s".repeat(8193),
        1,
    );
    let long_short_text = BOUNDARY_YAML.replacen("API -> worker", &"b".repeat(257), 1);
    let too_many_items = BOUNDARY_YAML.replacen(
        "  - \"Platform/Ops\"",
        &(0..65)
            .map(|index| format!("  - \"owner-{index}\""))
            .collect::<Vec<_>>()
            .join("\n"),
        1,
    );
    for invalid in [long_summary, long_short_text, too_many_items] {
        assert_eq!(
            parse_canonical_project_context(&decisions(), invalid.as_bytes())
                .unwrap_err()
                .kind(),
            ProjectContextArtifactErrorKind::StructuralValidationFailed
        );
    }

    let oversized = vec![b'a'; MAX_SOURCE_DOCUMENT_BYTES + 1];
    assert_eq!(
        parse_canonical_project_context(&decisions(), &oversized)
            .unwrap_err()
            .kind(),
        ProjectContextArtifactErrorKind::SourceLimitExceeded
    );
}

#[test]
fn public_serializer_covers_every_valid_optional_list_shape_and_json_string_escape() {
    for mask in [3_u8, 7, 11, 15] {
        let mut record = record_with_summary(
            "quote \" slash / backslash \\ controls \u{8}\u{c}\n\r\t\u{1} unicode é",
        );
        record.system_boundaries = ((mask & 1) != 0)
            .then(|| "Boundary".to_owned())
            .into_iter()
            .collect();
        record.ownership = ((mask & 2) != 0)
            .then(|| "Owner".to_owned())
            .into_iter()
            .collect();
        record.authoritative_references = ((mask & 4) != 0)
            .then(|| "handbook.reference@1.0.0".to_owned())
            .into_iter()
            .collect();
        record.known_unknowns = ((mask & 8) != 0)
            .then(|| "Unknown".to_owned())
            .into_iter()
            .collect();

        let bytes =
            serialize_canonical_project_context(&decisions(), &record).expect("closed YAML");
        let text = String::from_utf8(bytes.clone()).expect("UTF-8 YAML");
        assert!(text.contains(
            "summary: \"quote \\\" slash / backslash \\\\ controls \\b\\f\\n\\r\\t\\u0001 unicode é\"\n"
        ));
        for (bit, key) in [
            (1, "system_boundaries"),
            (2, "ownership"),
            (4, "authoritative_references"),
            (8, "known_unknowns"),
        ] {
            if mask & bit == 0 {
                assert!(text.contains(&format!("{key}: []\n")), "{key}: {text}");
            } else {
                assert!(text.contains(&format!("{key}:\n  - \"")), "{key}: {text}");
            }
        }
        assert!(bytes.ends_with(b"\n"));
        assert!(!bytes.ends_with(b"\n\n"));
        assert!(!bytes.contains(&b'\r'));
    }
}

#[test]
fn public_serializer_refuses_typed_records_outside_the_selected_schema() {
    let mut record = record_with_summary("Selected-schema validation.");
    for mutation in [
        |record: &mut CanonicalProjectContext| record.system_boundaries.clear(),
        |record: &mut CanonicalProjectContext| record.ownership.clear(),
        |record: &mut CanonicalProjectContext| record.schema_version = "0.1".to_owned(),
    ] {
        mutation(&mut record);
        assert_eq!(
            serialize_canonical_project_context(&decisions(), &record)
                .unwrap_err()
                .kind(),
            ProjectContextArtifactErrorKind::StructuralValidationFailed
        );
        record = record_with_summary("Selected-schema validation.");
    }
}

#[test]
fn markdown_plain_text_transform_table_is_exhaustive() {
    assert_eq!(
        rendered_summary("  alpha \r\n \t beta   gamma  ").unwrap(),
        "alpha beta gamma"
    );

    let punctuation = "!\"#$%&'()*+,-./:;<=>?@[\\]^_`{|}~";
    let expected = punctuation
        .chars()
        .flat_map(|character| ['\\', character])
        .collect::<String>();
    assert_eq!(rendered_summary(punctuation).unwrap(), expected);
    assert_eq!(rendered_summary("\\").unwrap(), "\\\\");

    for control in (0x01..=0x08)
        .chain(0x0b..=0x0c)
        .chain(0x0e..=0x1f)
        .chain(std::iter::once(0x7f))
    {
        let value = char::from_u32(control).unwrap().to_string();
        assert_eq!(
            rendered_summary(&value).unwrap_err(),
            ProjectContextArtifactErrorKind::RenderedViewRefused
        );
    }
    assert_eq!(
        rendered_summary(" \r\n\t ").unwrap_err(),
        ProjectContextArtifactErrorKind::RenderedViewRefused
    );
    assert_eq!(rendered_summary("é e\u{301} 😀").unwrap(), "é e\u{301} 😀");
    assert_eq!(
        rendered_summary("# heading\n- item").unwrap(),
        "\\# heading \\- item"
    );

    let rendered = render_project_context_markdown(&record_with_summary("stable"))
        .expect("deterministic Markdown");
    assert!(rendered.ends_with(b"\n"));
    assert!(!rendered.ends_with(b"\n\n"));
    assert_eq!(
        rendered,
        render_project_context_markdown(&record_with_summary("stable")).unwrap()
    );
}

#[cfg(unix)]
#[test]
fn selected_loader_covers_missing_symlink_non_regular_oversize_and_render_refusal() {
    use std::os::unix::fs::symlink;

    let cases = [
        (
            "missing",
            ArtifactInspectionStatus::Missing,
            ArtifactInspectionReason::RequiredPathMissing,
        ),
        (
            "symlink",
            ArtifactInspectionStatus::UnsafePath,
            ArtifactInspectionReason::SymlinkRefused,
        ),
        (
            "directory",
            ArtifactInspectionStatus::UnsafePath,
            ArtifactInspectionReason::NonRegularFileRefused,
        ),
        (
            "oversize",
            ArtifactInspectionStatus::Unreadable,
            ArtifactInspectionReason::DocumentLimitExceeded,
        ),
        (
            "render",
            ArtifactInspectionStatus::StructurallyInvalid,
            ArtifactInspectionReason::RenderedViewRefused,
        ),
    ];

    for (case, expected_status, expected_reason) in cases {
        let repo = tempfile::tempdir().unwrap();
        let path = selected_path(repo.path());
        std::fs::create_dir_all(path.parent().unwrap()).unwrap();
        match case {
            "missing" => {}
            "symlink" => {
                write(&repo.path().join("outside.yaml"), BOUNDARY_YAML.as_bytes());
                symlink(repo.path().join("outside.yaml"), &path).unwrap();
            }
            "directory" => std::fs::create_dir(&path).unwrap(),
            "oversize" => write(&path, &vec![b'a'; MAX_SOURCE_DOCUMENT_BYTES + 1]),
            "render" => write(
                &path,
                BOUNDARY_YAML
                    .replace(
                        "Coordinates \\\"alpha\\\"\\nboundary #1.",
                        "refuse\\u007fview",
                    )
                    .as_bytes(),
            ),
            _ => unreachable!(),
        }

        let report =
            inspect_profile_repository_with_stability_hook(repo.path(), &decisions(), || {});
        let row = project_context_row(&report);
        assert_eq!(
            (row.status(), row.reason()),
            (expected_status, expected_reason)
        );
        assert!(row.project_context_projection().is_none());
    }
}

#[cfg(unix)]
#[test]
fn selected_loader_maps_permission_denied_read_without_exposing_bytes() {
    use std::os::unix::fs::PermissionsExt;

    let repo = tempfile::tempdir().unwrap();
    let path = selected_path(repo.path());
    write(&path, BOUNDARY_YAML.as_bytes());
    std::fs::set_permissions(&path, std::fs::Permissions::from_mode(0o000)).unwrap();

    let report = inspect_profile_repository_with_stability_hook(repo.path(), &decisions(), || {});
    std::fs::set_permissions(&path, std::fs::Permissions::from_mode(0o600)).unwrap();
    let row = project_context_row(&report);
    assert_eq!(row.status(), ArtifactInspectionStatus::Unreadable);
    assert_eq!(row.reason(), ArtifactInspectionReason::RepositoryReadFailed);
    assert!(row.project_context_projection().is_none());
}

#[cfg(unix)]
#[test]
fn selected_loader_refuses_intermediate_symlink_without_following_it() {
    use std::os::unix::fs::symlink;

    let repo = tempfile::tempdir().unwrap();
    let outside = tempfile::tempdir().unwrap();
    write(
        &outside.path().join("project/context.yaml"),
        BOUNDARY_YAML.as_bytes(),
    );
    symlink(outside.path(), repo.path().join(".handbook")).unwrap();

    let report = inspect_profile_repository_with_stability_hook(repo.path(), &decisions(), || {});
    let row = project_context_row(&report);
    assert_eq!(row.status(), ArtifactInspectionStatus::UnsafePath);
    assert_eq!(row.reason(), ArtifactInspectionReason::SymlinkRefused);
    assert!(row.project_context_projection().is_none());
}

#[cfg(unix)]
#[test]
fn final_stability_rejects_same_inode_content_change_and_identical_byte_aba() {
    for identical_replacement in [false, true] {
        let repo = tempfile::tempdir().unwrap();
        let path = selected_path(repo.path());
        write(&path, BOUNDARY_YAML.as_bytes());
        let replacement = if identical_replacement {
            BOUNDARY_YAML.to_owned()
        } else {
            BOUNDARY_YAML.replace("boundary #1.", "changed boundary.")
        };
        let backup = repo.path().join("retained-original.yaml");

        let report =
            inspect_profile_repository_with_stability_hook(repo.path(), &decisions(), || {
                if identical_replacement {
                    std::fs::rename(&path, &backup).unwrap();
                    write(&path, replacement.as_bytes());
                } else {
                    std::fs::write(&path, replacement.as_bytes()).unwrap();
                }
            });
        let row = project_context_row(&report);
        assert_eq!(row.status(), ArtifactInspectionStatus::Unreadable);
        assert_eq!(
            row.reason(),
            ArtifactInspectionReason::ObservationChangedDuringInspection
        );
        assert!(row.project_context_projection().is_none());
    }
}

#[cfg(unix)]
#[test]
fn public_projection_excludes_retained_source_and_file_identity() {
    let repo = tempfile::tempdir().unwrap();
    let selected = selected_path(repo.path());
    write(&selected, BOUNDARY_YAML.as_bytes());
    let projection = load_selected_project_context(repo.path(), &decisions()).unwrap();
    let debug = format!("{projection:?}");

    std::fs::remove_file(&selected).unwrap();
    write(&selected, BOUNDARY_YAML.as_bytes());
    let replacement_projection = load_selected_project_context(repo.path(), &decisions()).unwrap();

    assert_eq!(
        projection.canonical_path(),
        ".handbook/project/context.yaml"
    );
    assert_eq!(projection.source_byte_length(), BOUNDARY_YAML.len());
    assert_eq!(projection, replacement_projection);
    assert!(!debug.contains("source_bytes"));
    assert!(!debug.contains("file_identity"));
    assert!(debug.contains("source_byte_length"));
}

#[test]
fn descriptor_selected_collection_never_ingests_retired_project_context_member() {
    let repo = tempfile::tempdir().unwrap();
    write(
        &repo.path().join(".handbook/project/context.yaml"),
        BOUNDARY_YAML.as_bytes(),
    );
    std::fs::create_dir_all(
        repo.path()
            .join(".handbook/project_context/PROJECT_CONTEXT.md"),
    )
    .unwrap();

    let artifacts = CanonicalArtifacts::load(repo.path()).unwrap();
    let selected = artifacts
        .artifacts
        .iter()
        .find(|artifact| artifact.identity.instance_id == "project_context")
        .expect("selected Project Context");
    assert_eq!(selected.bytes.as_deref(), Some(BOUNDARY_YAML.as_bytes()));
    assert!(artifacts.ingest_issues.iter().all(|issue| {
        issue.canonical_repo_relative_path != ".handbook/project_context/PROJECT_CONTEXT.md"
    }));
    assert!(artifacts.artifacts.iter().all(|artifact| {
        artifact.identity.relative_path != ".handbook/project_context/PROJECT_CONTEXT.md"
    }));
}

#[test]
fn descriptor_selected_collection_recognizes_selected_project_context_namespace_as_root_scaffold() {
    let repo = tempfile::tempdir().unwrap();
    write(
        &repo.path().join(".handbook/project/context.yaml"),
        BOUNDARY_YAML.as_bytes(),
    );

    let artifacts = CanonicalArtifacts::load(repo.path()).unwrap();

    assert_eq!(artifacts.system_root_status, SystemRootStatus::Ok);
    assert_eq!(
        artifacts
            .artifacts
            .iter()
            .find(|artifact| artifact.identity.instance_id == "project_authority")
            .expect("selected Charter")
            .identity
            .presence,
        ArtifactPresence::Missing
    );
    assert_eq!(
        artifacts
            .artifacts
            .iter()
            .find(|artifact| artifact.identity.instance_id == "project_context")
            .expect("selected Project Context")
            .identity
            .presence,
        ArtifactPresence::PresentNonEmpty
    );
    assert!(artifacts.ingest_issues.iter().all(|issue| {
        issue.canonical_repo_relative_path != ".handbook/project_context/PROJECT_CONTEXT.md"
    }));
}
