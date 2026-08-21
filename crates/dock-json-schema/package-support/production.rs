use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};

pub const SUPPORTED_TARGET: &str = "x86_64-pc-windows-msvc";
pub const CARGO_ARGUMENTS: [&str; 10] = [
    "rustc",
    "--offline",
    "--locked",
    "-p",
    "handbook-dock-json-schema",
    "--bin",
    "handbook-dock-json-schema",
    "--",
    "-C",
    "link-arg=/Brepro",
];

const CARGO_RELEASE: &str = "release: 1.89.0";
const RUSTC_RELEASE: &str = "release: 1.89.0";
const LLVM_RELEASE: &str = "LLVM version: 20.1.7";
const HOST: &str = "host: x86_64-pc-windows-msvc";
const LINKER_RELEASE: &str = "14.43.34810.0";
const LINK_TRACE: &str = "rustc_codegen_ssa::back::link=info";

pub struct ProductionPackageBuildV1 {
    pub executable_path: PathBuf,
}

pub fn produce_current_package_v1(
    workspace_root: &Path,
    target_directory: &Path,
) -> Result<ProductionPackageBuildV1, String> {
    validate_inputs(workspace_root, target_directory)?;
    let cargo = Path::new(env!("CARGO"));
    let rustc = cargo.with_file_name(format!("rustc{}", std::env::consts::EXE_SUFFIX));
    for (name, program) in [("Cargo", cargo), ("Rust", rustc.as_path())] {
        let metadata = fs::symlink_metadata(program)
            .map_err(|_| format!("inspect admitted {name} program failed"))?;
        if !program.is_absolute()
            || !metadata.file_type().is_file()
            || metadata.file_type().is_symlink()
        {
            return Err(format!("admitted {name} program was not one explicit file"));
        }
    }
    require_version(
        cargo,
        &["-Vv"],
        &["cargo 1.89.0", CARGO_RELEASE, HOST],
        "Cargo",
    )?;
    require_version(
        &rustc,
        &["-Vv"],
        &["rustc 1.89.0", RUSTC_RELEASE, LLVM_RELEASE, HOST],
        "Rust",
    )?;

    let output = Command::new(cargo)
        .current_dir(workspace_root)
        .env("CARGO_TARGET_DIR", target_directory)
        .env("RUSTC_LOG", LINK_TRACE)
        .env_remove("RUSTFLAGS")
        .env_remove("CARGO_ENCODED_RUSTFLAGS")
        .env_remove("CARGO_BUILD_RUSTFLAGS")
        .env_remove("CARGO_TARGET_X86_64_PC_WINDOWS_MSVC_RUSTFLAGS")
        .args(CARGO_ARGUMENTS)
        .output()
        .map_err(|_| "start exact production Cargo build failed".to_owned())?;
    if !output.status.success() {
        return Err(format!(
            "exact production Cargo build failed: {}",
            bounded_stderr(&output)
        ));
    }
    let stderr = String::from_utf8(output.stderr)
        .map_err(|_| "Cargo build diagnostics were not UTF-8".to_owned())?;
    if !stderr.contains("`dev` profile [unoptimized + debuginfo]") {
        return Err("Cargo did not report the selected dev profile".to_owned());
    }
    if !stderr.contains("/Brepro") {
        return Err("final linker diagnostics did not contain /Brepro".to_owned());
    }
    verify_linker(&stderr)?;

    let executable_path = target_directory
        .join("debug")
        .join("handbook-dock-json-schema.exe");
    let metadata = fs::symlink_metadata(&executable_path)
        .map_err(|_| "inspect exact production executable failed".to_owned())?;
    if !metadata.file_type().is_file() || metadata.file_type().is_symlink() {
        return Err("exact production executable was not one regular file".to_owned());
    }
    Ok(ProductionPackageBuildV1 { executable_path })
}

fn validate_inputs(workspace_root: &Path, target_directory: &Path) -> Result<(), String> {
    if env!("HANDBOOK_DOCK_JSON_SCHEMA_TARGET") != SUPPORTED_TARGET {
        return Err("production package target is not admitted".to_owned());
    }
    if !workspace_root.join("Cargo.lock").is_file()
        || !workspace_root
            .join("crates/dock-json-schema/src/main.rs")
            .is_file()
    {
        return Err("production workspace source or Cargo.lock is absent".to_owned());
    }
    let metadata = fs::symlink_metadata(target_directory)
        .map_err(|_| "inspect isolated Cargo target directory failed".to_owned())?;
    if !metadata.file_type().is_dir() || metadata.file_type().is_symlink() {
        return Err("Cargo target directory must be one real directory".to_owned());
    }
    if fs::read_dir(target_directory)
        .map_err(|_| "read isolated Cargo target directory failed".to_owned())?
        .next()
        .is_some()
    {
        return Err("Cargo target directory must start empty".to_owned());
    }
    Ok(())
}

fn require_version(
    program: &Path,
    arguments: &[&str],
    required: &[&str],
    name: &str,
) -> Result<(), String> {
    let output = Command::new(program)
        .args(arguments)
        .output()
        .map_err(|_| format!("start admitted {name} failed"))?;
    if !output.status.success() {
        return Err(format!("admitted {name} version check failed"));
    }
    let stdout = String::from_utf8(output.stdout)
        .map_err(|_| format!("admitted {name} version output was not UTF-8"))?;
    if required.iter().all(|marker| stdout.contains(marker)) {
        Ok(())
    } else {
        Err(format!("admitted {name} profile did not match"))
    }
}

fn verify_linker(build_stderr: &str) -> Result<(), String> {
    let marker = "invoking linker \"";
    let start = build_stderr
        .rfind(marker)
        .map(|index| index + marker.len())
        .ok_or_else(|| "final linker path was not reported by rustc".to_owned())?;
    let end = build_stderr[start..]
        .find('"')
        .map(|index| start + index)
        .ok_or_else(|| "final linker path was malformed".to_owned())?;
    let linker = Path::new(&build_stderr[start..end]);
    if !linker.is_absolute()
        || linker.file_name().and_then(|name| name.to_str()) != Some("link.exe")
        || !linker.is_file()
    {
        return Err("rustc did not select one explicit MSVC link.exe".to_owned());
    }
    let output = Command::new(linker)
        .arg("/?")
        .output()
        .map_err(|_| "start selected MSVC linker version check failed".to_owned())?;
    let banner = format!(
        "{}{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
    if banner.contains(LINKER_RELEASE) {
        Ok(())
    } else {
        Err("selected MSVC linker profile did not match".to_owned())
    }
}

fn bounded_stderr(output: &Output) -> String {
    let stderr = String::from_utf8_lossy(&output.stderr);
    stderr.chars().take(2_048).collect()
}
