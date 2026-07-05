use std::env;
use std::error::Error;
use std::path::PathBuf;
use std::process::{self, Command};

fn workspace_root() -> Result<PathBuf, Box<dyn Error>> {
    Ok(PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .canonicalize()?)
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut args = env::args().skip(1);
    match args.next().as_deref() {
        Some("refresh-data") => run_extractor(false),
        Some("check-registry") => run_extractor(true),
        Some("-h") | Some("--help") | None => {
            print_usage();
            Ok(())
        }
        Some(command) => Err(format!("unknown xtask command: {command}").into()),
    }
}

fn run_extractor(check_only: bool) -> Result<(), Box<dyn Error>> {
    let root = workspace_root()?;
    let cargo = env::var("CARGO").unwrap_or_else(|_| "cargo".to_string());
    let mut command = Command::new(cargo);
    command
        .current_dir(&root)
        .args(["run", "-p", "interslavic-extractor", "--"])
        .arg("--input")
        .arg(root.join("crates/interslavic/data/dictionary_metadata.tsv"))
        .arg("--output-dir")
        .arg(root.join("crates/interslavic/generated"));
    if check_only {
        command.arg("--check-only");
    }

    let status = command.status()?;
    if status.success() {
        Ok(())
    } else {
        process::exit(status.code().unwrap_or(1));
    }
}

fn print_usage() {
    eprintln!("Usage: cargo xtask <command>");
    eprintln!();
    eprintln!("Commands:");
    eprintln!("  refresh-data     Regenerate static dictionary metadata");
    eprintln!("  check-registry   Verify generated dictionary metadata is current");
}
