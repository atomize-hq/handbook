fn main() {
    let target = std::env::var("TARGET").expect("Cargo supplies TARGET to build scripts");
    println!("cargo:rustc-env=HANDBOOK_DOCK_JSON_SCHEMA_TARGET={target}");
}
