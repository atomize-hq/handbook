use std::ffi::OsStr;
use std::fs;
use std::path::{Path, PathBuf};

const RUNTIME_STATE_ROOT_RELATIVE: &str = ".handbook/state";

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct RuntimeStateResetPlan {
    entries: Vec<RuntimeStateResetEntry>,
    paths: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum RuntimeStateResetPlanError {
    Unsafe(String),
    TooLarge { observed_count: usize },
}

impl std::fmt::Display for RuntimeStateResetPlanError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Unsafe(message) => formatter.write_str(message),
            Self::TooLarge { observed_count } => write!(
                formatter,
                "runtime state reset plan has {observed_count} entries; limit is 128"
            ),
        }
    }
}

impl std::error::Error for RuntimeStateResetPlanError {}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct RuntimeStateResetApplied {
    removed_paths: Vec<String>,
}

impl RuntimeStateResetApplied {
    pub(crate) fn removed_paths(&self) -> &[String] {
        &self.removed_paths
    }
}

#[derive(Debug)]
pub(crate) struct RuntimeStateResetApplyError {
    message: String,
    removed_paths: Vec<String>,
}

impl RuntimeStateResetApplyError {
    pub(crate) fn removed_paths(&self) -> &[String] {
        &self.removed_paths
    }
}

impl std::fmt::Display for RuntimeStateResetApplyError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(&self.message)
    }
}

impl std::error::Error for RuntimeStateResetApplyError {}

impl RuntimeStateResetPlan {
    pub(crate) fn paths(&self) -> &[String] {
        &self.paths
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct RuntimeStateResetEntry {
    path: PathBuf,
    display_path: String,
    kind: RuntimeStateResetEntryKind,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum RuntimeStateResetEntryKind {
    File,
    Dir,
}

pub(crate) fn plan_runtime_state_reset(
    repo_root: &Path,
) -> Result<RuntimeStateResetPlan, RuntimeStateResetPlanError> {
    let state_root_path = repo_root.join(RUNTIME_STATE_ROOT_RELATIVE);
    let state_root_metadata = match fs::symlink_metadata(&state_root_path) {
        Ok(metadata) => metadata,
        Err(source) if source.kind() == std::io::ErrorKind::NotFound => {
            return Ok(RuntimeStateResetPlan {
                entries: Vec::new(),
                paths: Vec::new(),
            });
        }
        Err(source) => {
            return Err(RuntimeStateResetPlanError::Unsafe(format!(
                "failed to inspect runtime state root `{RUNTIME_STATE_ROOT_RELATIVE}` at {}: {source}",
                state_root_path.display()
            )));
        }
    };

    if state_root_metadata.file_type().is_symlink() {
        return Err(RuntimeStateResetPlanError::Unsafe(format!(
            "runtime state root `{RUNTIME_STATE_ROOT_RELATIVE}` cannot be reset through symlink {}",
            state_root_path.display()
        )));
    }

    if !state_root_metadata.is_dir() {
        return Err(RuntimeStateResetPlanError::Unsafe(format!(
            "runtime state root `{RUNTIME_STATE_ROOT_RELATIVE}` is not a directory at {}",
            state_root_path.display()
        )));
    }

    let mut children = Vec::new();
    for child in fs::read_dir(&state_root_path).map_err(|source| {
        RuntimeStateResetPlanError::Unsafe(format!(
            "failed to read runtime state root `{RUNTIME_STATE_ROOT_RELATIVE}` at {}: {source}",
            state_root_path.display()
        ))
    })? {
        let child = child.map_err(|source| {
            RuntimeStateResetPlanError::Unsafe(format!(
                "failed to enumerate runtime state root `{RUNTIME_STATE_ROOT_RELATIVE}` at {}: {source}",
                state_root_path.display()
            ))
        })?;
        if children.len() == 128 {
            return Err(RuntimeStateResetPlanError::TooLarge {
                observed_count: 129,
            });
        }
        children.push(child);
    }
    children.sort_by_key(|child| child.file_name());

    let mut reset_entries = Vec::new();
    let mut observed_count = 0;
    for child in children {
        let child_file_name = child.file_name();
        let child_display_path =
            public_reset_display_path(RUNTIME_STATE_ROOT_RELATIVE, &child_file_name)?;
        collect_runtime_state_reset_entries(
            &child.path(),
            &child_display_path,
            &mut reset_entries,
            &mut observed_count,
        )?;
    }

    let mut reset_paths = reset_entries
        .iter()
        .map(|entry| entry.display_path.clone())
        .collect::<Vec<_>>();
    reset_paths.sort();

    Ok(RuntimeStateResetPlan {
        entries: reset_entries,
        paths: reset_paths,
    })
}

pub(crate) fn apply_runtime_state_reset(
    plan: &RuntimeStateResetPlan,
) -> Result<RuntimeStateResetApplied, RuntimeStateResetApplyError> {
    let mut removed_paths = Vec::with_capacity(plan.entries.len());
    for entry in &plan.entries {
        let removal = match entry.kind {
            RuntimeStateResetEntryKind::File => fs::remove_file(&entry.path).map_err(|source| {
                format!(
                    "failed to remove runtime state file `{}` at {}: {source}",
                    entry.display_path,
                    entry.path.display()
                )
            }),
            RuntimeStateResetEntryKind::Dir => fs::remove_dir(&entry.path).map_err(|source| {
                format!(
                    "failed to remove runtime state directory `{}` at {}: {source}",
                    entry.display_path,
                    entry.path.display()
                )
            }),
        };
        if let Err(message) = removal {
            return Err(RuntimeStateResetApplyError {
                message,
                removed_paths,
            });
        }
        removed_paths.push(entry.display_path.clone());
    }

    Ok(RuntimeStateResetApplied { removed_paths })
}

fn collect_runtime_state_reset_entries(
    path: &Path,
    display_path: &str,
    reset_entries: &mut Vec<RuntimeStateResetEntry>,
    observed_count: &mut usize,
) -> Result<(), RuntimeStateResetPlanError> {
    *observed_count += 1;
    if *observed_count > 128 {
        return Err(RuntimeStateResetPlanError::TooLarge {
            observed_count: *observed_count,
        });
    }
    let metadata = fs::symlink_metadata(path).map_err(|source| {
        RuntimeStateResetPlanError::Unsafe(format!(
            "failed to inspect runtime state path `{display_path}` at {}: {source}",
            path.display()
        ))
    })?;

    if metadata.file_type().is_symlink() {
        return Err(RuntimeStateResetPlanError::Unsafe(format!(
            "runtime state path `{display_path}` cannot be reset through symlink {}",
            path.display()
        )));
    }

    if metadata.is_dir() {
        let mut children = Vec::new();
        for child in fs::read_dir(path).map_err(|source| {
            RuntimeStateResetPlanError::Unsafe(format!(
                "failed to read runtime state directory `{display_path}` at {}: {source}",
                path.display()
            ))
        })? {
            let child = child.map_err(|source| {
                RuntimeStateResetPlanError::Unsafe(format!(
                    "failed to enumerate runtime state directory `{display_path}` at {}: {source}",
                    path.display()
                ))
            })?;
            let prospective_count = *observed_count + children.len() + 1;
            if prospective_count > 128 {
                return Err(RuntimeStateResetPlanError::TooLarge {
                    observed_count: 129,
                });
            }
            children.push(child);
        }
        children.sort_by_key(|child| child.file_name());

        for child in children {
            let child_file_name = child.file_name();
            let child_display_path = public_reset_display_path(display_path, &child_file_name)?;
            collect_runtime_state_reset_entries(
                &child.path(),
                &child_display_path,
                reset_entries,
                observed_count,
            )?;
        }

        reset_entries.push(RuntimeStateResetEntry {
            path: path.to_path_buf(),
            display_path: display_path.to_string(),
            kind: RuntimeStateResetEntryKind::Dir,
        });
        return Ok(());
    }

    if metadata.is_file() {
        reset_entries.push(RuntimeStateResetEntry {
            path: path.to_path_buf(),
            display_path: display_path.to_string(),
            kind: RuntimeStateResetEntryKind::File,
        });
        return Ok(());
    }

    Err(RuntimeStateResetPlanError::Unsafe(format!(
        "runtime state path `{display_path}` is not a regular file or directory"
    )))
}

fn public_reset_display_path(
    parent: &str,
    child_name: &OsStr,
) -> Result<String, RuntimeStateResetPlanError> {
    let child_name = child_name.to_str().ok_or_else(|| {
        RuntimeStateResetPlanError::Unsafe(
            "runtime state path cannot be represented exactly as UTF-8".to_owned(),
        )
    })?;
    if child_name.is_empty()
        || matches!(child_name, "." | "..")
        || child_name.contains(['/', '\\'])
        || child_name.chars().any(char::is_control)
    {
        return Err(RuntimeStateResetPlanError::Unsafe(
            "runtime state path is not a normalized public repository path".to_owned(),
        ));
    }
    let display_path = format!("{parent}/{child_name}");
    if display_path.len() > 4096 {
        return Err(RuntimeStateResetPlanError::Unsafe(
            "runtime state path exceeds the 4096-byte public path bound".to_owned(),
        ));
    }
    Ok(display_path)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::ffi::OsString;
    use tempfile::tempdir;

    #[cfg(unix)]
    fn non_utf_name() -> OsString {
        use std::os::unix::ffi::OsStringExt;
        OsString::from_vec(vec![0xff])
    }

    #[cfg(windows)]
    fn non_utf_name() -> OsString {
        use std::os::windows::ffi::OsStringExt;
        OsString::from_wide(&[0xd800])
    }

    #[test]
    fn public_reset_paths_reject_inexact_non_normalized_and_oversized_names() {
        assert!(matches!(
            public_reset_display_path(".handbook/state", &non_utf_name()),
            Err(RuntimeStateResetPlanError::Unsafe(_))
        ));
        assert!(matches!(
            public_reset_display_path(".handbook/state", std::ffi::OsStr::new("a\\b")),
            Err(RuntimeStateResetPlanError::Unsafe(_))
        ));
        let parent = format!(".handbook/state/{}", "a".repeat(4096));
        assert!(matches!(
            public_reset_display_path(&parent, std::ffi::OsStr::new("b")),
            Err(RuntimeStateResetPlanError::Unsafe(_))
        ));
    }

    #[test]
    fn planner_aborts_with_exact_count_at_entry_129() {
        let repo = tempdir().unwrap();
        let state = repo.path().join(RUNTIME_STATE_ROOT_RELATIVE);
        fs::create_dir_all(&state).unwrap();
        for index in 0..129 {
            fs::write(state.join(format!("entry-{index:03}.json")), b"{}").unwrap();
        }

        assert!(matches!(
            plan_runtime_state_reset(repo.path()),
            Err(RuntimeStateResetPlanError::TooLarge {
                observed_count: 129
            })
        ));
    }

    #[test]
    fn apply_reports_every_removed_path_on_success() {
        let repo = tempdir().unwrap();
        let first = repo.path().join("first");
        let second = repo.path().join("second");
        fs::write(&first, b"first").unwrap();
        fs::write(&second, b"second").unwrap();
        let plan = RuntimeStateResetPlan {
            entries: vec![
                RuntimeStateResetEntry {
                    path: first,
                    display_path: ".handbook/state/first".to_owned(),
                    kind: RuntimeStateResetEntryKind::File,
                },
                RuntimeStateResetEntry {
                    path: second,
                    display_path: ".handbook/state/second".to_owned(),
                    kind: RuntimeStateResetEntryKind::File,
                },
            ],
            paths: vec![
                ".handbook/state/first".to_owned(),
                ".handbook/state/second".to_owned(),
            ],
        };

        let applied = apply_runtime_state_reset(&plan).unwrap();

        assert_eq!(
            applied.removed_paths(),
            &[".handbook/state/first", ".handbook/state/second"]
        );
    }

    #[test]
    fn apply_error_reports_only_paths_removed_before_failure() {
        let repo = tempdir().unwrap();
        let removed = repo.path().join("removed");
        fs::write(&removed, b"removed").unwrap();
        let missing = repo.path().join("missing");
        let plan = RuntimeStateResetPlan {
            entries: vec![
                RuntimeStateResetEntry {
                    path: removed.clone(),
                    display_path: ".handbook/state/removed".to_owned(),
                    kind: RuntimeStateResetEntryKind::File,
                },
                RuntimeStateResetEntry {
                    path: missing,
                    display_path: ".handbook/state/missing".to_owned(),
                    kind: RuntimeStateResetEntryKind::File,
                },
            ],
            paths: vec![
                ".handbook/state/missing".to_owned(),
                ".handbook/state/removed".to_owned(),
            ],
        };

        let error = apply_runtime_state_reset(&plan).unwrap_err();

        assert_eq!(error.removed_paths(), &[".handbook/state/removed"]);
        assert!(!removed.exists());
    }
}
