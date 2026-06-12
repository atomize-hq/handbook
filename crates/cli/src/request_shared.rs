use super::{RequestArgs, PACKET_EXECUTION_DEMO_ID, PACKET_EXECUTION_LIVE_ID, PACKET_PLANNING_ID};
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum PacketId {
    Planning,
    ExecutionDemo,
    ExecutionLive,
}

impl PacketId {
    pub(super) fn as_str(self) -> &'static str {
        match self {
            PacketId::Planning => PACKET_PLANNING_ID,
            PacketId::ExecutionDemo => PACKET_EXECUTION_DEMO_ID,
            PacketId::ExecutionLive => PACKET_EXECUTION_LIVE_ID,
        }
    }
}

pub(super) struct PreparedRequest {
    pub(super) packet_id: PacketId,
    pub(super) compiler_root: PathBuf,
}

pub(super) fn prepare_request(
    args: &RequestArgs,
    repo_root: &Path,
) -> Result<PreparedRequest, String> {
    let packet_id = parse_packet_id(&args.packet)?;
    let compiler_root = match packet_id {
        PacketId::Planning | PacketId::ExecutionLive => repo_root.to_path_buf(),
        PacketId::ExecutionDemo => {
            let fixture_set_id = match args.fixture_set.as_deref() {
                Some(id) => id.trim(),
                None => {
                    return Err(format!(
                        "--fixture-set is required when --packet {PACKET_EXECUTION_DEMO_ID}"
                    ));
                }
            };
            validate_fixture_set_id(fixture_set_id)?;

            let fixture_set_dir = execution_demo_fixture_set_dir(repo_root, fixture_set_id);
            ensure_dir(&fixture_set_dir, "fixture set directory")?;
            let basis_root = fixture_set_dir.join(".handbook");
            ensure_dir(&basis_root, "fixture basis root")?;
            fixture_set_dir
        }
    };

    Ok(PreparedRequest {
        packet_id,
        compiler_root,
    })
}

pub(super) fn execution_demo_fixture_set_dir(repo_root: &Path, fixture_set_id: &str) -> PathBuf {
    repo_root
        .join("tests/fixtures/execution_demo")
        .join(fixture_set_id)
}

fn parse_packet_id(packet: &str) -> Result<PacketId, String> {
    let packet = packet.trim();
    match packet {
        PACKET_PLANNING_ID => Ok(PacketId::Planning),
        PACKET_EXECUTION_DEMO_ID => Ok(PacketId::ExecutionDemo),
        PACKET_EXECUTION_LIVE_ID => Ok(PacketId::ExecutionLive),
        _ => Err(format!(
            "unsupported --packet {packet:?} (allowed: {PACKET_PLANNING_ID:?}, {PACKET_EXECUTION_DEMO_ID:?}, {PACKET_EXECUTION_LIVE_ID:?})"
        )),
    }
}

fn validate_fixture_set_id(value: &str) -> Result<(), String> {
    let value = value.trim();
    if value.is_empty() {
        return Err("fixture_set_id must not be empty".to_string());
    }
    if value == "." || value == ".." {
        return Err("fixture_set_id must not be '.' or '..'".to_string());
    }
    if value
        .chars()
        .any(|c| !(c.is_ascii_alphanumeric() || c == '-' || c == '_'))
    {
        return Err("fixture_set_id must be ASCII [A-Za-z0-9_-] only".to_string());
    }
    Ok(())
}

fn ensure_dir(path: &Path, what: &str) -> Result<(), String> {
    match std::fs::metadata(path) {
        Ok(meta) if meta.is_dir() => Ok(()),
        Ok(_) => Err(format!("{what} is not a directory: {}", path.display())),
        Err(err) => Err(format!("{what} is missing: {} ({err})", path.display())),
    }
}
