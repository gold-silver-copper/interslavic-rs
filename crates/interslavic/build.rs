use std::env;
use std::path::PathBuf;

fn main() {
    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").expect("missing manifest dir"));
    let generated = manifest_dir.join("generated/noun_metadata_phf.rs");
    println!("cargo:rerun-if-changed={}", generated.display());
    assert!(
        generated.exists(),
        "missing generated data file: {}. Run `cargo xtask refresh-data`.",
        generated.display()
    );
}
