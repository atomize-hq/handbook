fn main() {
    let target = std::env::var("TARGET").expect("Cargo supplies TARGET to build scripts");
    let manifest = std::path::Path::new(
        &std::env::var("CARGO_MANIFEST_DIR").expect("Cargo supplies CARGO_MANIFEST_DIR"),
    )
    .join("production-manifests")
    .join(&target)
    .join("manifest.json");
    assert!(
        manifest.is_file(),
        "no frozen production Dock manifest is available for target {target}"
    );
    println!("cargo:rerun-if-changed={}", manifest.display());
    println!("cargo:rustc-env=HANDBOOK_DOCK_TARGET={target}");
    println!(
        "cargo:rustc-env=HANDBOOK_DOCK_PRODUCTION_MANIFEST={}",
        manifest.display()
    );
}
