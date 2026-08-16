use handbook_sdk::contract_membrane::schema_generation::{run, GenerationMode};

fn main() {
    let mut arguments = std::env::args_os().skip(1);
    let mode = match (arguments.next().as_deref(), arguments.next()) {
        (Some(value), None) if value == "--check" => GenerationMode::Check,
        (Some(value), None) if value == "--write" => GenerationMode::Write,
        _ => {
            eprintln!("usage: generate-contract-schemas <--check|--write>");
            std::process::exit(2);
        }
    };

    if let Err(error) = run(mode) {
        eprintln!("contract schema generation failed: {error}");
        std::process::exit(1);
    }
}
