//! Ordinary typed consumer. Inputs contain supplied project choices and approval
//! assertions; this executable invents no Charter, bounds, or credentials.
use handbook_sdk::local_setup::{
    LocalEnvelopeRequestV1, LocalInitialSetupLocatorV1, LocalInitialSetupRequestV1,
    LocalRevokeRequestV1,
};
use handbook_sdk::HandbookSdkV1;
use std::fs::{self, OpenOptions};
use std::io::Write;
use std::path::Path;

fn main() {
    if let Err(error) = run() {
        eprintln!("{error}");
        std::process::exit(2);
    }
}

fn run() -> Result<(), String> {
    let args = std::env::args().skip(1).collect::<Vec<_>>();
    if args.len() < 3 {
        return Err("usage: local_initial_setup establish ROOT INPUT_JSON RECOVERY_JSON | recover/current ROOT RECOVERY_JSON | admit ROOT RECOVERY_JSON ENVELOPE_JSON HORIZON OWNER_WRITE_PATH | revoke ROOT RECOVERY_JSON REQUEST_JSON".into());
    }
    let sdk = HandbookSdkV1::open(&args[1]);
    if args[0] == "establish" && args.len() == 4 {
        let request = LocalInitialSetupRequestV1::from_json_bytes(&read(&args[2])?)
            .map_err(|error| format!("{error:?}"))?;
        let prepared = sdk
            .prepare_local_initial_setup(&request)
            .map_err(|error| format!("{error:?}"))?;
        let locator = prepared
            .locator()
            .canonical_json_bytes()
            .map_err(|error| format!("{error:?}"))?;
        // Retain the complete locator before any native setup effect. Never
        // overwrite another original request's recovery file.
        save_new(Path::new(&args[3]), &locator)?;
        print_json(&sdk.establish_local_initial_setup(prepared))?;
        return Ok(());
    }
    let locator = LocalInitialSetupLocatorV1::from_json_bytes(&read(&args[2])?)
        .map_err(|error| format!("{error:?}"))?;
    match args[0].as_str() {
        "recover" if args.len() == 3 => print_json(
            &sdk.resume_local_initial_setup_request(&locator)
                .map_err(|error| format!("{error:?}"))?,
        ),
        "current" if args.len() == 3 => print_json(
            &sdk.resolve_current_local_initial_setup(&locator)
                .map_err(|error| format!("{error:?}"))?,
        ),
        "admit" if args.len() == 6 => {
            let request = LocalEnvelopeRequestV1::from_json_bytes(&read(&args[3])?)
                .map_err(|error| format!("{error:?}"))?;
            let envelope = sdk
                .admit_local_envelope_request(&locator, &request)
                .map_err(|error| format!("{error:?}"))?;
            let validation = envelope
                .authorize_validation(&args[4])
                .map_err(|error| error.detail().to_owned())?;
            let owner_write = envelope.evaluate_mutation("repository_path", &args[5]);
            println!("validation={validation:?}; owner_write={owner_write:?}");
            Ok(())
        }
        "revoke" if args.len() == 4 => {
            let value = handbook_engine::parse_schema_json(&read(&args[3])?)
                .map_err(|error| error.to_string())?;
            let request: LocalRevokeRequestV1 =
                serde_json::from_value(value).map_err(|error| error.to_string())?;
            print_json(
                &sdk.revoke_local_operating_authority(&locator, request)
                    .map_err(|error| format!("{error:?}"))?,
            )
        }
        _ => Err("invalid consumer command or argument count".into()),
    }
}

fn read(path: &str) -> Result<Vec<u8>, String> {
    let metadata = fs::symlink_metadata(path).map_err(|error| error.to_string())?;
    if !metadata.is_file() || metadata.file_type().is_symlink() || metadata.len() > 1_048_576 {
        return Err("input must be a regular bounded file".into());
    }
    fs::read(path).map_err(|error| error.to_string())
}

fn save_new(path: &Path, bytes: &[u8]) -> Result<(), String> {
    let mut file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(path)
        .map_err(|error| error.to_string())?;
    file.write_all(bytes).map_err(|error| error.to_string())?;
    file.sync_all().map_err(|error| error.to_string())
}

fn print_json(value: &impl serde::Serialize) -> Result<(), String> {
    println!(
        "{}",
        serde_json::to_string(value).map_err(|error| error.to_string())?
    );
    Ok(())
}
