use std::collections::BTreeMap;
use std::env;
use std::error::Error;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

const DEFAULT_URL: &str = "https://docs.google.com/spreadsheets/d/1N79e_yVHDo-d026HljueuKJlAAdeELAiPzdFzdBuKbY/export?format=tsv&gid=1987833874";

#[derive(Debug, Default)]
struct Args {
    input: Option<PathBuf>,
    url: Option<String>,
    generated_dir: PathBuf,
    artifacts_dir: PathBuf,
    run_checks: bool,
}

#[derive(Debug, Clone)]
struct Row {
    isv: String,
    addition: String,
    pos: String,
}

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
struct NounEntry {
    word: String,
    gender: &'static str,
    animacy: &'static str,
    indeclinable: bool,
    singular_only: bool,
    plural_only: bool,
}

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
struct WordEntry {
    word: String,
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = parse_args()?;
    let raw = load_data(&args)?;

    fs::create_dir_all(&args.generated_dir)?;
    fs::create_dir_all(&args.artifacts_dir)?;
    fs::write(args.artifacts_dir.join("dictionary.tsv"), &raw)?;

    let rows = parse_tsv(&raw);
    let (nouns, adjectives, verbs) = classify(&rows);

    let generated = generate_dictionary(&nouns, &adjectives, &verbs);
    fs::write(args.generated_dir.join("dictionary.rs"), generated)?;

    if args.run_checks {
        run_checks(&nouns, &adjectives, &verbs)?;
    }

    println!(
        "generated {} nouns, {} adjectives, {} verbs into {}",
        nouns.len(),
        adjectives.len(),
        verbs.len(),
        args.generated_dir.display()
    );

    Ok(())
}

fn parse_args() -> Result<Args, Box<dyn Error>> {
    let workspace_root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..");
    let mut args = Args {
        generated_dir: workspace_root.join("crates/interslavic/src/generated"),
        artifacts_dir: workspace_root.join("data/intermediate"),
        ..Args::default()
    };

    let mut iter = env::args().skip(1);
    while let Some(arg) = iter.next() {
        match arg.as_str() {
            "--input" => {
                args.input = Some(PathBuf::from(
                    iter.next().ok_or("missing value after --input")?,
                ))
            }
            "--url" => args.url = Some(iter.next().ok_or("missing value after --url")?),
            "--generated-dir" => {
                args.generated_dir =
                    PathBuf::from(iter.next().ok_or("missing value after --generated-dir")?)
            }
            "--artifacts-dir" => {
                args.artifacts_dir =
                    PathBuf::from(iter.next().ok_or("missing value after --artifacts-dir")?)
            }
            "--run-checks" | "--with-checks" => args.run_checks = true,
            "-h" | "--help" => {
                print_usage();
                std::process::exit(0);
            }
            other => return Err(format!("unknown argument: {other}").into()),
        }
    }

    Ok(args)
}

fn print_usage() {
    eprintln!("Usage: extractor [--input dictionary.tsv | --url URL] [--generated-dir DIR] [--artifacts-dir DIR] [--run-checks]");
}

fn load_data(args: &Args) -> Result<String, Box<dyn Error>> {
    if let Some(input) = &args.input {
        return Ok(fs::read_to_string(input)?);
    }

    let url = args.url.as_deref().unwrap_or(DEFAULT_URL);
    let output = Command::new("curl").args(["-L", "-sS", url]).output()?;
    if !output.status.success() {
        return Err(format!(
            "curl failed for {url}: {}",
            String::from_utf8_lossy(&output.stderr)
        )
        .into());
    }
    Ok(String::from_utf8(output.stdout)?)
}

fn parse_tsv(raw: &str) -> Vec<Row> {
    let mut lines = raw.lines();
    let Some(header_line) = lines.next() else {
        return Vec::new();
    };
    let header: Vec<&str> = header_line.trim_end_matches('\r').split('\t').collect();
    let idx = |name: &str| {
        header
            .iter()
            .position(|field| *field == name)
            .unwrap_or(usize::MAX)
    };
    let isv_idx = idx("isv");
    let addition_idx = idx("addition");
    let pos_idx = idx("partOfSpeech");

    lines
        .filter_map(|line| {
            let cols: Vec<&str> = line.trim_end_matches('\r').split('\t').collect();
            let get = |i: usize| {
                if i == usize::MAX {
                    ""
                } else {
                    cols.get(i).copied().unwrap_or("")
                }
            };
            let isv = get(isv_idx).trim().to_string();
            if isv.is_empty() {
                return None;
            }
            Some(Row {
                isv,
                addition: get(addition_idx).trim().to_string(),
                pos: get(pos_idx).trim().to_string(),
            })
        })
        .collect()
}

fn classify(rows: &[Row]) -> (Vec<NounEntry>, Vec<WordEntry>, Vec<WordEntry>) {
    let mut nouns = BTreeMap::<String, NounEntry>::new();
    let mut adjectives = BTreeMap::<String, WordEntry>::new();
    let mut verbs = BTreeMap::<String, WordEntry>::new();

    for row in rows {
        for word in split_words(&row.isv) {
            if word.is_empty() || word.contains(' ') {
                continue;
            }
            let pos = row.pos.as_str();
            if is_noun(pos) {
                let entry = NounEntry {
                    word: word.clone(),
                    gender: gender(pos),
                    animacy: if pos.contains("anim") {
                        "Animacy::Animate"
                    } else {
                        "Animacy::Inanimate"
                    },
                    indeclinable: pos.contains("indecl") || row.addition.contains("indecl"),
                    singular_only: pos.contains("sg"),
                    plural_only: pos.contains("pl"),
                };
                nouns.entry(word).or_insert(entry);
            } else if pos.contains("adj") {
                adjectives.entry(word.clone()).or_insert(WordEntry { word });
            } else if pos.starts_with("v.") || pos.contains(" v.") {
                verbs.entry(word.clone()).or_insert(WordEntry { word });
            }
        }
    }

    (
        nouns.into_values().collect(),
        adjectives.into_values().collect(),
        verbs.into_values().collect(),
    )
}

fn split_words(isv: &str) -> Vec<String> {
    isv.split(',')
        .map(|word| {
            word.trim()
                .trim_start_matches('!')
                .trim_matches(|c| matches!(c, '[' | ']' | '(' | ')'))
                .trim()
                .to_lowercase()
        })
        .filter(|word| !word.is_empty())
        .collect()
}

fn is_noun(pos: &str) -> bool {
    let pos = pos.trim_start();
    starts_with_pos(pos, "m") || starts_with_pos(pos, "f") || starts_with_pos(pos, "n")
}

fn starts_with_pos(pos: &str, tag: &str) -> bool {
    pos == tag
        || pos.starts_with(&format!("{tag}."))
        || pos.starts_with(&format!("{tag}/"))
        || pos.starts_with(&format!("{tag}./"))
}

fn gender(pos: &str) -> &'static str {
    let pos = pos.trim_start();
    if starts_with_pos(pos, "f") || pos.starts_with("m./f") {
        "Gender::Feminine"
    } else if starts_with_pos(pos, "n") {
        "Gender::Neuter"
    } else {
        "Gender::Masculine"
    }
}

fn generate_dictionary(
    nouns: &[NounEntry],
    adjectives: &[WordEntry],
    verbs: &[WordEntry],
) -> String {
    let mut out = String::new();
    out.push_str("// @generated by crates/extractor. Do not edit by hand.\n");
    out.push_str("use interslavic_core::{Animacy, Gender};\n\n");
    out.push_str("#[derive(Debug, Clone, Copy, PartialEq, Eq)]\npub(crate) struct NounEntry {\n    pub word: &'static str,\n    pub gender: Gender,\n    pub animacy: Animacy,\n    pub indeclinable: bool,\n    pub singular_only: bool,\n    pub plural_only: bool,\n}\n\n");
    out.push_str("#[derive(Debug, Clone, Copy, PartialEq, Eq)]\npub(crate) struct VerbEntry { pub word: &'static str }\n\n");
    out.push_str("#[derive(Debug, Clone, Copy, PartialEq, Eq)]\npub(crate) struct AdjectiveEntry { pub word: &'static str }\n\n");

    out.push_str("pub(crate) static NOUNS: &[NounEntry] = &[\n");
    for n in nouns {
        out.push_str(&format!(
            "    NounEntry {{ word: \"{}\", gender: {}, animacy: {}, indeclinable: {}, singular_only: {}, plural_only: {} }},\n",
            escape(&n.word), n.gender, n.animacy, n.indeclinable, n.singular_only, n.plural_only
        ));
    }
    out.push_str("];\n\n");

    out.push_str("pub(crate) static VERBS: &[VerbEntry] = &[\n");
    for v in verbs {
        out.push_str(&format!(
            "    VerbEntry {{ word: \"{}\" }},\n",
            escape(&v.word)
        ));
    }
    out.push_str("];\n\n");

    out.push_str("pub(crate) static ADJECTIVES: &[AdjectiveEntry] = &[\n");
    for a in adjectives {
        out.push_str(&format!(
            "    AdjectiveEntry {{ word: \"{}\" }},\n",
            escape(&a.word)
        ));
    }
    out.push_str("];\n");
    out
}

fn escape(s: &str) -> String {
    s.replace('\\', "\\\\").replace('"', "\\\"")
}

fn run_checks(
    nouns: &[NounEntry],
    adjectives: &[WordEntry],
    verbs: &[WordEntry],
) -> Result<(), Box<dyn Error>> {
    if nouns.is_empty() && adjectives.is_empty() && verbs.is_empty() {
        return Err("generated dictionary is empty".into());
    }
    if has_duplicates(nouns.iter().map(|n| n.word.as_str())) {
        return Err("duplicate noun entries after normalization".into());
    }
    if has_duplicates(adjectives.iter().map(|n| n.word.as_str())) {
        return Err("duplicate adjective entries after normalization".into());
    }
    if has_duplicates(verbs.iter().map(|n| n.word.as_str())) {
        return Err("duplicate verb entries after normalization".into());
    }
    Ok(())
}

fn has_duplicates<'a>(items: impl Iterator<Item = &'a str>) -> bool {
    let mut previous = None;
    for item in items {
        if previous == Some(item) {
            return true;
        }
        previous = Some(item);
    }
    false
}

#[allow(dead_code)]
fn _is_file(path: &Path) -> bool {
    path.is_file()
}
