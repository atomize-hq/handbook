// Frozen inventory gate from deferral.rs::inspect at 65a3328972faf08d5cd08d09231a964955f845f4.
// This bounded predecessor reader executes the retained-root inventory gate that
// every predecessor current/replay resolution passed before dependent use. It
// intentionally does not import any new owner type or transition implementation.
use handbook_engine::{RepositoryDirectoryEntryKindV1, RetainedRepositoryRootV1};
pub fn predecessor_inventory_gate(root: &RetainedRepositoryRootV1) -> Result<(), String> {
    let versions = root
        .list_directory_bounded(".handbook/admission-governance", 1)
        .map_err(|error| error.to_string())?;
    if versions.iter().any(|entry| {
        entry.name() != "v2" || entry.kind() != RepositoryDirectoryEntryKindV1::Directory
    }) {
        return Err("Unsupported owner inventory version".into());
    }
    let root_names = root
        .list_directory_bounded(".handbook/admission-governance/v2", 3)
        .map_err(|error| error.to_string())?;
    for entry in &root_names {
        let expected_kind = if entry.name() == "requests" {
            RepositoryDirectoryEntryKindV1::Directory
        } else {
            RepositoryDirectoryEntryKindV1::RegularFile
        };
        if !matches!(
            entry.name(),
            "registration.json" | "requests" | "scope-current.json"
        ) || entry.kind() != expected_kind
        {
            return Err("Unsupported transition or unsafe v2 root inventory".into());
        }
    }
    Ok(())
}
