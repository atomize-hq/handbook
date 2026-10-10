//! Ordinary consumer of native work state through normal SDK dependencies.
//! All project/approval inputs are supplied; retained locators grant no authority.
use handbook_sdk::local_check::LocalCheckLocatorV1;
use handbook_sdk::local_setup::LocalInitialSetupLocatorV1;
use handbook_sdk::work_state::*;
use handbook_sdk::HandbookSdkV1;
use std::fs::{self, OpenOptions};
use std::io::Write;
use std::path::Path;

fn main() {
    let result = std::thread::Builder::new()
        .name("work-state-consumer".into())
        .stack_size(8 * 1024 * 1024)
        .spawn(run)
        .expect("start work consumer")
        .join()
        .expect("work consumer completed");
    if let Err(error) = result {
        eprintln!("{error}");
        std::process::exit(2);
    }
}

fn run() -> Result<(), String> {
    let args = std::env::args().skip(1).collect::<Vec<_>>();
    if args.len() < 3 {
        return Err("usage: work_state establish-kind ROOT INITIAL CHECK DEFINITION APPROVAL SAVE | current-kind ROOT BINDING | kind-history/recover-kind ROOT KIND_REQUEST | activate ROOT INITIAL CHECK TERMS SAVE | change-scope/establish-from-deferred ROOT INITIAL CHECK TERMS SAVE PREDECESSOR | register ROOT INITIAL GRANT INPUT SAVE | current ROOT GRANT ITEM_KEY | current-terms ROOT INITIAL GRANT | terms-history/recover-terms ROOT GRANT | history/recover/completion ROOT REQUEST | revoke ROOT GRANT KEY PRINCIPAL SOURCE INSTRUCTION SAVE | revoke-history/recover-revoke ROOT REVOCATION".into());
    }
    let sdk = HandbookSdkV1::open(&args[1]);
    match args[0].as_str() {
        "establish-kind" if args.len() == 7 => {
            let initial =
                LocalInitialSetupLocatorV1::from_json_bytes(&read(&args[2])?).map_err(debug)?;
            let check = LocalCheckLocatorV1::from_json_bytes(&read(&args[3])?).map_err(debug)?;
            let definition =
                WorkKindDefinitionV1::from_json_bytes(&read(&args[4])?).map_err(debug)?;
            let approval = WorkKindApprovalV1::from_json_bytes(&read(&args[5])?).map_err(debug)?;
            let prepared = sdk
                .prepare_work_kind_establishment(&initial, &check, definition, approval)
                .map_err(debug)?;
            save_new(
                Path::new(&args[6]),
                &prepared.locator().canonical_json_bytes().map_err(debug)?,
            )?;
            print_json(&sdk.establish_work_kind(prepared))
        }
        "current-kind" if args.len() == 3 => {
            let binding = WorkKindBindingV1::from_json_bytes(&read(&args[2])?).map_err(debug)?;
            print_json(&sdk.resolve_work_kind(&binding).map_err(debug)?)
        }
        "kind-history" | "recover-kind" if args.len() == 3 => {
            let original = WorkKindLocatorV1::from_json_bytes(&read(&args[2])?).map_err(debug)?;
            print_json(&if args[0] == "kind-history" {
                sdk.resolve_work_kind_request(&original)
            } else {
                sdk.recover_work_kind_request(&original)
            })
        }
        "activate" | "change-scope" | "establish-from-deferred"
            if (args[0] == "activate" && args.len() == 6)
                || (args[0] != "activate" && args.len() == 7) =>
        {
            let initial =
                LocalInitialSetupLocatorV1::from_json_bytes(&read(&args[2])?).map_err(debug)?;
            let check = LocalCheckLocatorV1::from_json_bytes(&read(&args[3])?).map_err(debug)?;
            let terms =
                WorkRegistrationTermsV1::from_json_bytes(&read(&args[4])?).map_err(debug)?;
            let predecessor = if args.len() == 7 {
                Some(WorkGrantLocatorV1::from_json_bytes(&read(&args[6])?).map_err(debug)?)
            } else {
                None
            };
            let transition = match args[0].as_str() {
                "change-scope" => WorkTermsTransitionV1::ScopeChange(predecessor.as_ref().unwrap()),
                "establish-from-deferred" => {
                    WorkTermsTransitionV1::EstablishFromDeferred(predecessor.as_ref().unwrap())
                }
                _ => WorkTermsTransitionV1::Initial,
            };
            let prepared = sdk
                .prepare_work_registration_terms(&initial, &check, terms, transition)
                .map_err(debug)?;
            save_new(
                Path::new(&args[5]),
                &prepared.locator().canonical_json_bytes().map_err(debug)?,
            )?;
            print_json(&sdk.activate_work_registration_terms(prepared))
        }
        "register" if args.len() == 6 => {
            let initial =
                LocalInitialSetupLocatorV1::from_json_bytes(&read(&args[2])?).map_err(debug)?;
            let grant = WorkGrantLocatorV1::from_json_bytes(&read(&args[3])?).map_err(debug)?;
            let input =
                WorkRegistrationInputV1::from_json_bytes(&read(&args[4])?).map_err(debug)?;
            let prepared = sdk
                .prepare_work_registration(&initial, &grant, input)
                .map_err(debug)?;
            save_new(
                Path::new(&args[5]),
                &prepared.locator().canonical_json_bytes().map_err(debug)?,
            )?;
            print_json(&sdk.register_work_item(prepared))
        }
        "current" if args.len() == 4 => {
            let grant = WorkGrantLocatorV1::from_json_bytes(&read(&args[2])?).map_err(debug)?;
            print_json(
                &sdk.resolve_current_work_item(&grant, &args[3])
                    .map_err(debug)?,
            )
        }
        "current-terms" if args.len() == 4 => {
            let initial =
                LocalInitialSetupLocatorV1::from_json_bytes(&read(&args[2])?).map_err(debug)?;
            let grant = WorkGrantLocatorV1::from_json_bytes(&read(&args[3])?).map_err(debug)?;
            print_json(
                &sdk.resolve_current_work_registration_terms(&initial, &grant)
                    .map_err(debug)?,
            )
        }
        "terms-history" | "recover-terms" if args.len() == 3 => {
            let grant = WorkGrantLocatorV1::from_json_bytes(&read(&args[2])?).map_err(debug)?;
            print_json(&if args[0] == "terms-history" {
                sdk.resolve_work_registration_terms_request(&grant)
            } else {
                sdk.recover_work_registration_terms_request(&grant)
            })
        }
        "history" | "recover" | "completion" if args.len() == 3 => {
            let locator = WorkRequestLocatorV1::from_json_bytes(&read(&args[2])?).map_err(debug)?;
            if args[0] == "completion" {
                print_json(
                    &sdk.inspect_work_completion_support(&locator)
                        .map_err(debug)?,
                )
            } else {
                print_json(&if args[0] == "history" {
                    sdk.resolve_work_registration_request(&locator)
                } else {
                    sdk.recover_work_registration_request(&locator)
                })
            }
        }
        "revoke" if args.len() == 8 => {
            let grant = WorkGrantLocatorV1::from_json_bytes(&read(&args[2])?).map_err(debug)?;
            let locator = sdk
                .prepare_work_registration_revocation(
                    &grant, &args[3], &args[4], &args[5], &args[6],
                )
                .map_err(debug)?;
            save_new(
                Path::new(&args[7]),
                &locator.canonical_json_bytes().map_err(debug)?,
            )?;
            print_json(&sdk.revoke_work_registration_terms(&locator))
        }
        "revoke-history" | "recover-revoke" if args.len() == 3 => {
            let locator = WorkRevokeLocatorV1::from_json_bytes(&read(&args[2])?).map_err(debug)?;
            print_json(&if args[0] == "revoke-history" {
                sdk.resolve_work_registration_revocation_request(&locator)
            } else {
                sdk.recover_work_registration_revocation_request(&locator)
            })
        }
        _ => Err("invalid consumer command or argument count".into()),
    }
}

fn debug(error: impl std::fmt::Debug) -> String {
    format!("{error:?}")
}

fn read(path: &str) -> Result<Vec<u8>, String> {
    let metadata = fs::symlink_metadata(path).map_err(debug)?;
    if !metadata.is_file() || metadata.file_type().is_symlink() || metadata.len() > 1_048_576 {
        return Err("input must be a regular bounded file".into());
    }
    fs::read(path).map_err(debug)
}

fn save_new(path: &Path, bytes: &[u8]) -> Result<(), String> {
    let mut file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(path)
        .map_err(debug)?;
    file.write_all(bytes).map_err(debug)?;
    file.sync_all().map_err(debug)
}

fn print_json(value: &impl serde::Serialize) -> Result<(), String> {
    println!("{}", serde_json::to_string(value).map_err(debug)?);
    Ok(())
}
