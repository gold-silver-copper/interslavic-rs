//! Workspace task runner (`cargo xtask <command>`).
//!
//! The runtime crates never parse dictionary TSV data. `refresh-data` delegates
//! to `interslavic-extractor` to regenerate committed Rust metadata, while
//! `check-registry` verifies that the generated files are current.

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
        Some("check-all") => check_all(),
        Some("examples") => run_examples(),
        Some("speed") => run_speedmark(),
        Some("accuracy") => run_accuracy(),
        Some("-h") | Some("--help") | Some("help") | None => {
            print_usage();
            Ok(())
        }
        Some(command) => Err(format!("unknown xtask command: {command}").into()),
    }
}

fn check_all() -> Result<(), Box<dyn Error>> {
    run_cargo(&["fmt", "--all", "--", "--check"])?;
    run_cargo(&["test", "--workspace"])?;
    run_extractor(true)
}

fn run_examples() -> Result<(), Box<dyn Error>> {
    for example in ["basic", "verb_paradigm", "sentence"] {
        run_cargo(&["run", "-p", "interslavic", "--example", example])?;
    }
    Ok(())
}

fn run_speedmark() -> Result<(), Box<dyn Error>> {
    run_cargo(&[
        "run",
        "-p",
        "interslavic",
        "--example",
        "speedmark",
        "--release",
    ])
}

fn run_accuracy() -> Result<(), Box<dyn Error>> {
    let root = workspace_root()?;
    let status = Command::new("node")
        .current_dir(&root)
        .arg("tools/compare-latest-sonic.js")
        .status()?;
    exit_if_failed(status.code(), status.success());
    Ok(())
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
    exit_if_failed(status.code(), status.success());
    Ok(())
}

fn run_cargo(args: &[&str]) -> Result<(), Box<dyn Error>> {
    let root = workspace_root()?;
    let cargo = env::var("CARGO").unwrap_or_else(|_| "cargo".to_string());
    let status = Command::new(cargo).current_dir(root).args(args).status()?;
    exit_if_failed(status.code(), status.success());
    Ok(())
}

fn exit_if_failed(code: Option<i32>, success: bool) {
    if !success {
        process::exit(code.unwrap_or(1));
    }
}

fn print_usage() {
    eprintln!("Usage: cargo xtask <command>");
    eprintln!();
    eprintln!("Commands:");
    eprintln!("  refresh-data     Regenerate static dictionary metadata");
    eprintln!("  check-registry   Verify generated dictionary metadata is current");
    eprintln!("  check-all        Run fmt check, workspace tests, and check-registry");
    eprintln!("  examples         Run lightweight public API examples");
    eprintln!("  speed            Run the release speedmark example");
    eprintln!("  accuracy         Run sonic16x parity comparison");
}
