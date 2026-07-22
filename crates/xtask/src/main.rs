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
        Some("dump-paradigms") => dump_paradigms(&mut args),
        Some("diff-fingerprint") => diff_fingerprint(&mut args),
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

/// Write the whole-dictionary paradigm dump (`lemma\tcell\tform`) to
/// `target/paradigm-fingerprint/<name>.tsv` (default `dump.tsv`; pass a
/// name to keep an old dump around for diffing).
fn dump_paradigms(args: &mut impl Iterator<Item = String>) -> Result<(), Box<dyn Error>> {
    let root = workspace_root()?;
    let name = args.next().unwrap_or_else(|| "dump".to_string());
    let cargo = env::var("CARGO").unwrap_or_else(|_| "cargo".to_string());
    let output = Command::new(cargo)
        .current_dir(&root)
        .args([
            "run",
            "-q",
            "-p",
            "interslavic",
            "--example",
            "dump_paradigms",
        ])
        .output()?;
    exit_if_failed(output.status.code(), output.status.success());
    let dir = root.join("target/paradigm-fingerprint");
    std::fs::create_dir_all(&dir)?;
    let path = dir.join(format!("{name}.tsv"));
    std::fs::write(&path, &output.stdout)?;
    let hash = fnv1a(&output.stdout);
    eprintln!(
        "wrote {} ({} cells, fingerprint {hash:#018x})",
        path.display(),
        output.stdout.iter().filter(|&&b| b == b'\n').count()
    );
    Ok(())
}

fn fnv1a(bytes: &[u8]) -> u64 {
    let mut hash: u64 = 0xcbf2_9ce4_8422_2325;
    for &byte in bytes {
        hash ^= u64::from(byte);
        hash = hash.wrapping_mul(0x0000_0100_0000_01b3);
    }
    hash
}

/// Print the cell-level delta between two paradigm dumps, so the
/// changelog enumeration for a fingerprint change is mechanical.
fn diff_fingerprint(args: &mut impl Iterator<Item = String>) -> Result<(), Box<dyn Error>> {
    let (Some(old_path), Some(new_path)) = (args.next(), args.next()) else {
        return Err("usage: cargo xtask diff-fingerprint <old.tsv> <new.tsv>".into());
    };
    let load = |path: &str| -> Result<std::collections::BTreeMap<String, String>, Box<dyn Error>> {
        let text = std::fs::read_to_string(path)?;
        Ok(text
            .lines()
            .filter_map(|line| {
                let mut parts = line.splitn(3, '\t');
                let lemma = parts.next()?;
                let cell = parts.next()?;
                let form = parts.next()?;
                Some((format!("{lemma}\t{cell}"), form.to_string()))
            })
            .collect())
    };
    let old = load(&old_path)?;
    let new = load(&new_path)?;
    let mut changed = 0usize;
    for (key, old_form) in &old {
        match new.get(key) {
            Some(new_form) if new_form != old_form => {
                println!("changed\t{key}\t{old_form}\t->\t{new_form}");
                changed += 1;
            }
            None => {
                println!("removed\t{key}\t{old_form}");
                changed += 1;
            }
            _ => {}
        }
    }
    for (key, new_form) in &new {
        if !old.contains_key(key) {
            println!("added\t{key}\t{new_form}");
            changed += 1;
        }
    }
    eprintln!(
        "{changed} cell(s) differ ({} -> {} cells)",
        old.len(),
        new.len()
    );
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
    eprintln!("  dump-paradigms   Write the whole-dictionary paradigm dump [name]");
    eprintln!("  diff-fingerprint Print the cell delta between two dumps <old> <new>");
}
