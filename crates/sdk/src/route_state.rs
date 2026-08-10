use std::fs;
use std::path::{Path, PathBuf};

const RUNTIME_STATE_ROOT_RELATIVE: &str = ".handbook/state";

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct RuntimeStateResetPlan {
    entries: Vec<RuntimeStateResetEntry>,
    paths: Vec<String>,
}

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

pub(crate) fn plan_runtime_state_reset(repo_root: &Path) -> Result<RuntimeStateResetPlan, String> {
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
            return Err(format!(
                "failed to inspect runtime state root `{RUNTIME_STATE_ROOT_RELATIVE}` at {}: {source}",
                state_root_path.display()
            ));
        }
    };

    if state_root_metadata.file_type().is_symlink() {
        return Err(format!(
            "runtime state root `{RUNTIME_STATE_ROOT_RELATIVE}` cannot be reset through symlink {}",
            state_root_path.display()
        ));
    }

    if !state_root_metadata.is_dir() {
        return Err(format!(
            "runtime state root `{RUNTIME_STATE_ROOT_RELATIVE}` is not a directory at {}",
            state_root_path.display()
        ));
    }

    let mut children = fs::read_dir(&state_root_path)
        .map_err(|source| {
            format!(
                "failed to read runtime state root `{RUNTIME_STATE_ROOT_RELATIVE}` at {}: {source}",
                state_root_path.display()
            )
        })?
        .collect::<Result<Vec<_>, _>>()
        .map_err(|source| {
            format!(
                "failed to enumerate runtime state root `{RUNTIME_STATE_ROOT_RELATIVE}` at {}: {source}",
                state_root_path.display()
            )
        })?;
    children.sort_by_key(|child| child.file_name());

    let mut reset_entries = Vec::new();
    for child in children {
        let child_file_name = child.file_name();
        let child_name = child_file_name.to_string_lossy();
        let child_display_path = format!("{RUNTIME_STATE_ROOT_RELATIVE}/{child_name}");
        collect_runtime_state_reset_entries(
            &child.path(),
            &child_display_path,
            &mut reset_entries,
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

pub(crate) fn apply_runtime_state_reset(plan: &RuntimeStateResetPlan) -> Result<(), String> {
    for entry in &plan.entries {
        match entry.kind {
            RuntimeStateResetEntryKind::File => {
                fs::remove_file(&entry.path).map_err(|source| {
                    format!(
                        "failed to remove runtime state file `{}` at {}: {source}",
                        entry.display_path,
                        entry.path.display()
                    )
                })?;
            }
            RuntimeStateResetEntryKind::Dir => {
                fs::remove_dir(&entry.path).map_err(|source| {
                    format!(
                        "failed to remove runtime state directory `{}` at {}: {source}",
                        entry.display_path,
                        entry.path.display()
                    )
                })?;
            }
        }
    }

    Ok(())
}

fn collect_runtime_state_reset_entries(
    path: &Path,
    display_path: &str,
    reset_entries: &mut Vec<RuntimeStateResetEntry>,
) -> Result<(), String> {
    let metadata = fs::symlink_metadata(path).map_err(|source| {
        format!(
            "failed to inspect runtime state path `{display_path}` at {}: {source}",
            path.display()
        )
    })?;

    if metadata.file_type().is_symlink() {
        return Err(format!(
            "runtime state path `{display_path}` cannot be reset through symlink {}",
            path.display()
        ));
    }

    if metadata.is_dir() {
        let mut children = fs::read_dir(path)
            .map_err(|source| {
                format!(
                    "failed to read runtime state directory `{display_path}` at {}: {source}",
                    path.display()
                )
            })?
            .collect::<Result<Vec<_>, _>>()
            .map_err(|source| {
                format!(
                    "failed to enumerate runtime state directory `{display_path}` at {}: {source}",
                    path.display()
                )
            })?;
        children.sort_by_key(|child| child.file_name());

        for child in children {
            let child_file_name = child.file_name();
            let child_name = child_file_name.to_string_lossy();
            let child_display_path = format!("{display_path}/{child_name}");
            collect_runtime_state_reset_entries(&child.path(), &child_display_path, reset_entries)?;
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

    Err(format!(
        "runtime state path `{display_path}` is not a regular file or directory"
    ))
}
