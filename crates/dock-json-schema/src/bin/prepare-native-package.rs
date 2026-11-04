#[path = "../../package-support/production.rs"]
mod production;
#[path = "../../package-support/staging.rs"]
#[allow(dead_code)]
mod staging;

use std::path::Path;

fn main() {
    let args: Vec<_> = std::env::args_os().collect();
    if args.len() != 4 {
        eprintln!("usage: prepare-native-package build WORKSPACE EMPTY_TARGET_DIR | stage EXECUTABLE HOST_DIR");
        std::process::exit(2);
    }
    let result = (|| {
        if args[1] == "build" {
            let package =
                production::produce_current_package_v1(Path::new(&args[2]), Path::new(&args[3]))?;
            println!("executable={}", package.executable_path.display());
        } else if args[1] == "stage" {
            let staged = staging::stage_native_package_v1(
                Path::new(&args[2]),
                Path::new(&args[3]),
                production::SUPPORTED_TARGET,
                std::env::consts::EXE_SUFFIX,
            )?;
            println!(
                "manifest={}",
                staged.package_root.join("manifest.json").display()
            );
        } else {
            return Err("unknown command".to_owned());
        }
        Ok::<(), String>(())
    })();
    if let Err(error) = result {
        eprintln!("{error}");
        std::process::exit(1);
    }
}
