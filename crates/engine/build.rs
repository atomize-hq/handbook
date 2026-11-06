fn main() {
    let target = std::env::var("TARGET").expect("Cargo supplies the compile target");
    println!("cargo:rustc-env=HANDBOOK_ENGINE_COMPILE_TARGET_V1={target}");
}
