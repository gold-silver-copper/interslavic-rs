use std::env;
use std::error::Error;
use std::path::PathBuf;
use std::process::{self, Command};

fn main() -> Result<(), Box<dyn Error>> {
    let mut args = env::args().skip(1);
    match args.next().as_deref() {
        Some("refresh-data") => refresh_data(args.collect()),
        Some("-h") | Some("--help") | None => {
            print_usage();
            Ok(())
        }
        Some(command) => Err(format!("unknown xtask command: {command}").into()),
    }
}

fn refresh_data(args: Vec<String>) -> Result<(), Box<dyn Error>> {
    let cargo = env::var("CARGO").unwrap_or_else(|_| "cargo".to_string());
    let workspace_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .canonicalize()?;

    let mut command = Command::new(cargo);
    command.current_dir(&workspace_root);
    command.arg("run").arg("-p").arg("extractor").arg("--");

    let mut iter = args.into_iter();
    while let Some(arg) = iter.next() {
        match arg.as_str() {
            "--input" | "--url" | "--generated-dir" | "--artifacts-dir" => {
                let value = iter
                    .next()
                    .ok_or_else(|| format!("expected value after {arg}"))?;
                command.arg(arg).arg(value);
            }
            "--with-checks" => {
                command.arg("--run-checks");
            }
            "--run-checks" => {
                command.arg("--run-checks");
            }
            "-h" | "--help" => {
                print_refresh_data_usage();
                return Ok(());
            }
            other => return Err(format!("unknown refresh-data flag: {other}").into()),
        }
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
    eprintln!("  refresh-data    Regenerate dictionary tables from official Interslavic TSV data");
}

fn print_refresh_data_usage() {
    eprintln!("Usage:");
    eprintln!("  cargo xtask refresh-data [--with-checks]");
    eprintln!("  cargo xtask refresh-data --url <tsv-url> [--with-checks]");
    eprintln!("  cargo xtask refresh-data --input data/raw/dictionary.tsv [--with-checks]");
}
