use std::collections::HashMap;
use std::sync::OnceLock;

const DICTIONARY_METADATA_TSV: &str = include_str!("../data/dictionary_metadata.tsv");

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum DictionaryPartOfSpeech {
    Noun,
    Adjective,
    Other,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum DictionaryGender {
    Masculine,
    Feminine,
    Neuter,
    MasculineOrFeminine,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub(crate) struct DictionaryEntry {
    pub id: String,
    pub lemma: String,
    pub addition: String,
    pub part_of_speech: String,
    pub pos: DictionaryPartOfSpeech,
    pub gender: Option<DictionaryGender>,
    pub animate: bool,
    pub plural_only: bool,
    pub singular_only: bool,
    pub indeclinable: bool,
}

#[derive(Debug)]
struct DictionaryIndex {
    entries: Vec<DictionaryEntry>,
    exact: HashMap<String, Vec<usize>>,
    lowercase: HashMap<String, Vec<usize>>,
}

static DICTIONARY: OnceLock<DictionaryIndex> = OnceLock::new();

#[allow(dead_code)]
pub(crate) fn lookup_noun(word: &str) -> Option<&'static DictionaryEntry> {
    lookup(word, |entry| entry.pos == DictionaryPartOfSpeech::Noun)
}

pub(crate) fn lookup_noun_for_number(
    word: &str,
    wants_plural: bool,
) -> Option<&'static DictionaryEntry> {
    choose_for_number(
        lookup_all(word, |entry| entry.pos == DictionaryPartOfSpeech::Noun),
        wants_plural,
    )
}

pub(crate) fn lookup_noun_by_id_for_number(
    id: &str,
    word: &str,
    wants_plural: bool,
) -> Option<&'static DictionaryEntry> {
    choose_for_number(
        lookup_nouns_by_id(id)
            .into_iter()
            .filter(|entry| {
                let lower = word.to_lowercase();
                entry.lemma == word || entry.lemma.to_lowercase() == lower
            })
            .collect(),
        wants_plural,
    )
}

pub(crate) fn lookup_nouns_by_id(id: &str) -> Vec<&'static DictionaryEntry> {
    dictionary()
        .entries
        .iter()
        .filter(|entry| entry.id == id && entry.pos == DictionaryPartOfSpeech::Noun)
        .collect()
}

pub(crate) fn lookup_nouns_by_lemma(word: &str) -> Vec<&'static DictionaryEntry> {
    lookup_all(word, |entry| entry.pos == DictionaryPartOfSpeech::Noun)
}

fn choose_for_number(
    candidates: Vec<&'static DictionaryEntry>,
    wants_plural: bool,
) -> Option<&'static DictionaryEntry> {
    candidates
        .iter()
        .copied()
        .find(|entry| {
            if wants_plural {
                !entry.singular_only
            } else {
                !entry.plural_only
            }
        })
        .or_else(|| candidates.first().copied())
}

#[allow(dead_code)]
pub(crate) fn lookup_adjective(word: &str) -> Option<&'static DictionaryEntry> {
    lookup(word, |entry| entry.pos == DictionaryPartOfSpeech::Adjective)
}

fn lookup(
    word: &str,
    predicate: impl Fn(&DictionaryEntry) -> bool,
) -> Option<&'static DictionaryEntry> {
    lookup_all(word, predicate).first().copied()
}

fn lookup_all(
    word: &str,
    predicate: impl Fn(&DictionaryEntry) -> bool,
) -> Vec<&'static DictionaryEntry> {
    let dictionary = dictionary();
    if let Some(indices) = dictionary.exact.get(word) {
        let entries: Vec<&DictionaryEntry> = indices
            .iter()
            .map(|idx| &dictionary.entries[*idx])
            .filter(|entry| predicate(entry))
            .collect();
        if !entries.is_empty() {
            return entries;
        }
    }

    let lower = word.to_lowercase();
    dictionary
        .lowercase
        .get(&lower)
        .map(|indices| {
            indices
                .iter()
                .map(|idx| &dictionary.entries[*idx])
                .filter(|entry| predicate(entry))
                .collect()
        })
        .unwrap_or_default()
}

fn dictionary() -> &'static DictionaryIndex {
    DICTIONARY.get_or_init(parse_dictionary)
}

fn parse_dictionary() -> DictionaryIndex {
    let mut entries = Vec::new();
    let mut exact: HashMap<String, Vec<usize>> = HashMap::new();
    let mut lowercase: HashMap<String, Vec<usize>> = HashMap::new();

    for line in DICTIONARY_METADATA_TSV.lines().skip(1) {
        let columns: Vec<&str> = line.split('\t').collect();
        if columns.len() < 4 {
            continue;
        }

        let id = columns[0].to_string();
        let isv = columns[1];
        let addition = columns[2].to_string();
        let part_of_speech = columns[3].to_string();
        let metadata = parse_part_of_speech(&part_of_speech);

        for lemma in isv
            .split(',')
            .map(normalize_lemma)
            .filter(|lemma| !lemma.is_empty())
        {
            let entry = DictionaryEntry {
                id: id.clone(),
                lemma,
                addition: addition.clone(),
                part_of_speech: part_of_speech.clone(),
                pos: metadata.pos.clone(),
                gender: metadata.gender.clone(),
                animate: metadata.animate,
                plural_only: metadata.plural_only,
                singular_only: metadata.singular_only,
                indeclinable: metadata.indeclinable,
            };
            let index = entries.len();
            exact.entry(entry.lemma.clone()).or_default().push(index);
            lowercase
                .entry(entry.lemma.to_lowercase())
                .or_default()
                .push(index);
            entries.push(entry);
        }
    }

    DictionaryIndex {
        entries,
        exact,
        lowercase,
    }
}

#[derive(Debug, Clone)]
struct ParsedPartOfSpeech {
    pos: DictionaryPartOfSpeech,
    gender: Option<DictionaryGender>,
    animate: bool,
    plural_only: bool,
    singular_only: bool,
    indeclinable: bool,
}

fn parse_part_of_speech(details: &str) -> ParsedPartOfSpeech {
    let normalized = details.replace("./", "/").replace(' ', "");
    let parts: Vec<&str> = normalized
        .split('.')
        .filter(|part| !part.is_empty())
        .collect();

    let has = |needle: &str| parts.contains(&needle);
    let pos = if has("adj") {
        DictionaryPartOfSpeech::Adjective
    } else if has("f") || has("n") || has("m") || has("m/f") {
        DictionaryPartOfSpeech::Noun
    } else {
        DictionaryPartOfSpeech::Other
    };

    let gender = if has("m/f") {
        Some(DictionaryGender::MasculineOrFeminine)
    } else if has("m") {
        Some(DictionaryGender::Masculine)
    } else if has("f") {
        Some(DictionaryGender::Feminine)
    } else if has("n") {
        Some(DictionaryGender::Neuter)
    } else {
        None
    };

    ParsedPartOfSpeech {
        pos,
        gender,
        animate: has("anim"),
        plural_only: has("pl"),
        singular_only: has("sg"),
        indeclinable: has("indecl"),
    }
}

fn normalize_lemma(lemma: &str) -> String {
    lemma
        .trim()
        .replace('!', "")
        .trim_matches('"')
        .trim()
        .to_string()
}
