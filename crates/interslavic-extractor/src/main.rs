use std::collections::BTreeMap;
use std::env;
use std::error::Error;
use std::fs::{self, File};
use std::io::Write;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, PartialEq, Eq)]
enum Gender {
    Masculine,
    Feminine,
    Neuter,
    MasculineFeminine,
}

#[derive(Debug, Clone)]
struct Entry {
    lemma: String,
    addition: String,
    gender: Gender,
    animate: bool,
    plural_only: bool,
    singular_only: bool,
    indeclinable: bool,
}

#[derive(Debug)]
struct ParsedPartOfSpeech {
    is_noun: bool,
    gender: Option<Gender>,
    animate: bool,
    plural_only: bool,
    singular_only: bool,
    indeclinable: bool,
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut input = PathBuf::from("crates/interslavic/data/dictionary_metadata.tsv");
    let mut output_dir = PathBuf::from("crates/interslavic/generated");
    let mut check_only = false;

    let mut args = env::args().skip(1);
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--input" => input = PathBuf::from(require_value(&mut args, "--input")?),
            "--output-dir" => output_dir = PathBuf::from(require_value(&mut args, "--output-dir")?),
            "--check-only" => check_only = true,
            "-h" | "--help" => {
                print_usage();
                return Ok(());
            }
            other => return Err(format!("unknown argument: {other}").into()),
        }
    }

    let generated = generate(&input)?;
    let output_path = output_dir.join("noun_metadata_phf.rs");

    if check_only {
        let committed = fs::read_to_string(&output_path)?;
        if committed != generated {
            return Err(format!(
                "generated noun metadata is stale: run `cargo xtask refresh-data` and commit {}",
                output_path.display()
            )
            .into());
        }
        println!("check-registry: OK — generated noun metadata is deterministic and current.");
        return Ok(());
    }

    fs::create_dir_all(&output_dir)?;
    let mut file = File::create(&output_path)?;
    file.write_all(generated.as_bytes())?;
    println!("wrote {}", output_path.display());
    Ok(())
}

fn require_value(
    args: &mut impl Iterator<Item = String>,
    flag: &str,
) -> Result<String, Box<dyn Error>> {
    args.next()
        .ok_or_else(|| format!("expected a value after {flag}").into())
}

fn print_usage() {
    eprintln!("Usage: interslavic-extractor [--input PATH] [--output-dir DIR] [--check-only]");
}

fn generate(input: &Path) -> Result<String, Box<dyn Error>> {
    let text = fs::read_to_string(input)?;
    let mut map: BTreeMap<String, Vec<Entry>> = BTreeMap::new();

    for line in text.lines().skip(1) {
        let columns: Vec<&str> = line.split('\t').collect();
        if columns.len() < 4 {
            continue;
        }

        let isv = columns[1];
        let addition = columns[2].to_string();
        let metadata = parse_part_of_speech(columns[3]);
        if !metadata.is_noun {
            continue;
        }
        let Some(gender) = metadata.gender else {
            continue;
        };

        for lemma in isv
            .split(',')
            .map(normalize_lemma)
            .filter(|lemma| !lemma.is_empty())
        {
            let entry = Entry {
                lemma: lemma.clone(),
                addition: addition.clone(),
                gender: gender.clone(),
                animate: metadata.animate,
                plural_only: metadata.plural_only,
                singular_only: metadata.singular_only,
                indeclinable: metadata.indeclinable,
            };
            insert_entry(&mut map, lemma.clone(), entry.clone());
            let lower = lemma.to_lowercase();
            if lower != lemma {
                insert_entry(&mut map, lower, entry);
            }
        }
    }

    Ok(write_phf(&map))
}

fn insert_entry(map: &mut BTreeMap<String, Vec<Entry>>, key: String, entry: Entry) {
    let entries = map.entry(key).or_default();
    if !entries.iter().any(|existing| same_entry(existing, &entry)) {
        entries.push(entry);
    }
}

fn same_entry(a: &Entry, b: &Entry) -> bool {
    a.lemma == b.lemma
        && a.addition == b.addition
        && a.gender == b.gender
        && a.animate == b.animate
        && a.plural_only == b.plural_only
        && a.singular_only == b.singular_only
        && a.indeclinable == b.indeclinable
}

fn write_phf(map: &BTreeMap<String, Vec<Entry>>) -> String {
    let mut out = String::new();
    out.push_str("use phf::phf_map;\n");
    out.push_str("use super::{DictionaryEntry, DictionaryGender};\n\n");
    out.push_str("pub(crate) static NOUN_METADATA: phf::Map<&'static str, &'static [DictionaryEntry]> = phf_map! {\n");
    for (key, entries) in map {
        out.push_str(&format!("    {:?} => &[\n", key));
        for entry in entries {
            out.push_str("        DictionaryEntry { ");
            out.push_str(&format!("lemma: {:?}, ", entry.lemma));
            out.push_str(&format!("addition: {:?}, ", entry.addition));
            out.push_str(&format!(
                "gender: DictionaryGender::{}, ",
                gender_name(&entry.gender)
            ));
            out.push_str(&format!("animate: {}, ", entry.animate));
            out.push_str(&format!("plural_only: {}, ", entry.plural_only));
            out.push_str(&format!("singular_only: {}, ", entry.singular_only));
            out.push_str(&format!("indeclinable: {} ", entry.indeclinable));
            out.push_str("},\n");
        }
        out.push_str("    ],\n");
    }
    out.push_str("};\n\n");
    out.push_str("pub(crate) fn get_nouns(word: &str) -> Option<&'static [DictionaryEntry]> {\n");
    out.push_str("    NOUN_METADATA.get(word).copied()\n");
    out.push_str("}\n");
    out
}

fn gender_name(gender: &Gender) -> &'static str {
    match gender {
        Gender::Masculine => "Masculine",
        Gender::Feminine => "Feminine",
        Gender::Neuter => "Neuter",
        Gender::MasculineFeminine => "MasculineFeminine",
    }
}

fn parse_part_of_speech(details: &str) -> ParsedPartOfSpeech {
    let normalized = details.replace("./", "/").replace(' ', "");
    let parts: Vec<&str> = normalized
        .split('.')
        .filter(|part| !part.is_empty())
        .collect();

    let has = |needle: &str| parts.contains(&needle);
    let is_noun = has("f") || has("n") || has("m") || has("m/f");

    let gender = if has("m/f") {
        Some(Gender::MasculineFeminine)
    } else if has("m") {
        Some(Gender::Masculine)
    } else if has("f") {
        Some(Gender::Feminine)
    } else if has("n") {
        Some(Gender::Neuter)
    } else {
        None
    };

    ParsedPartOfSpeech {
        is_noun,
        gender,
        animate: has("anim"),
        plural_only: has("pl"),
        singular_only: has("sg"),
        indeclinable: has("indecl"),
    }
}

fn normalize_lemma(lemma: &str) -> String {
    let cleaned = lemma.trim().replace('!', "");
    remove_bracketed_text(cleaned.trim_matches('"').trim(), '[', ']')
}

fn remove_bracketed_text(input: &str, open: char, close: char) -> String {
    let mut result = String::new();
    let mut depth = 0usize;
    for c in input.chars() {
        if c == open {
            depth += 1;
        } else if c == close && depth > 0 {
            depth -= 1;
        } else if depth == 0 {
            result.push(c);
        }
    }
    result.trim().to_string()
}
