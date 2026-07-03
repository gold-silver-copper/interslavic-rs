#![doc = "Dictionary-backed Interslavic morphology."]

pub use interslavic_core::grammar::*;
pub use interslavic_core::utils::{HARD_CONSONANTS, J_MERGE_CHARS, VOWELS};
pub use interslavic_core::{Adjective, InterslavicCore, Noun, NounMeta, Verb};

mod generated;

use generated::dictionary::{NounEntry, ADJECTIVES, NOUNS, VERBS};

/// Public entry point for Interslavic morphology.
///
/// `Interslavic` combines generated dictionary metadata with the pure rule
/// engine from `interslavic-core`. Dictionary data is generated ahead of time;
/// no CSV file or network access is needed at runtime.
///
/// # Examples
/// ```
/// use interslavic::*;
///
/// assert_eq!(Interslavic::noun("mųž", Case::Gen, Number::Singular), "mųža");
/// assert_eq!(Interslavic::verb("učiti", Person::First, Number::Singular, Tense::Present), "učų");
/// ```
#[derive(Debug, Clone, Default)]
pub struct Interslavic;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct ComplexNoun {
    pub head_noun: String,
    pub adjective: Vec<String>,
}

impl Default for ComplexNoun {
    fn default() -> Self {
        Self {
            head_noun: "exemplum".into(),
            adjective: Vec::new(),
        }
    }
}

/// Compatibility namespace for utility helpers exposed by older releases.
pub struct ISVUTILS;

/// Backwards-compatible alias for older versions of the crate.
pub type ISV = Interslavic;

impl Interslavic {
    /// No-op retained for compatibility with older versions that loaded CSVs at runtime.
    pub fn initialize_dictionary(&mut self, _path: &str) {}

    /// Decline a noun using generated dictionary metadata when available.
    pub fn noun(word: &str, case: Case, number: Number) -> String {
        Self::decline_noun(word, &case, &number).0
    }

    /// Decline a noun and return the inferred/dictionary gender.
    pub fn decline_noun(word: &str, case: &Case, number: &Number) -> Noun {
        let meta = get_noun(word).map(noun_meta);
        InterslavicCore::decline_noun_with_meta(word, *case, *number, meta)
    }

    /// Decline an adjective.
    pub fn adjective(
        word: &str,
        case: Case,
        number: Number,
        gender: Gender,
        animacy: Animacy,
    ) -> String {
        let _ = get_adjective(word);
        InterslavicCore::adjective(word, case, number, gender, animacy)
    }

    /// Backwards-compatible adjective API accepting `animate: bool`.
    pub fn decline_adj(
        word: &str,
        case: &Case,
        number: &Number,
        gender: &Gender,
        animate: bool,
    ) -> String {
        Self::adjective(word, *case, *number, *gender, Animacy::from(animate))
    }

    /// Conjugate a verb.
    pub fn verb(word: &str, person: Person, number: Number, tense: Tense) -> String {
        let _ = get_verb(word);
        InterslavicCore::verb(word, person, number, tense)
    }

    /// Backwards-compatible verb API. `gender` is currently relevant for participles,
    /// not present-tense finite forms.
    pub fn conjugate_verb(
        word: &str,
        person: &Person,
        number: &Number,
        _gender: &Gender,
        tense: &Tense,
    ) -> String {
        Self::verb(word, *person, *number, *tense)
    }

    pub fn l_participle(word: &str, gender: &Gender, number: &Number) -> String {
        InterslavicCore::l_participle(word, *gender, *number)
    }

    pub fn get_present_tense_stem(infinitive: &str) -> (String, Conjugation) {
        InterslavicCore::get_present_tense_stem(infinitive)
    }

    pub fn get_infinitive_stem(word: &str) -> String {
        InterslavicCore::get_infinitive_stem(word)
    }

    pub fn guess_gender(word: &str) -> Gender {
        get_noun(word).map_or_else(|| InterslavicCore::guess_gender(word), |entry| entry.gender)
    }

    pub fn noun_is_animate(word: &str) -> bool {
        get_noun(word).map_or_else(
            || InterslavicCore::noun_is_animate(word),
            |entry| entry.animacy == Animacy::Animate,
        )
    }

    pub fn last_n_chars(word: &str, n: usize) -> String {
        interslavic_core::utils::last_n_chars(word, n)
    }

    pub fn is_ost_class(word: &str) -> bool {
        InterslavicCore::is_ost_class(word)
    }

    pub fn get_noun_stem(word: &str, number: &Number) -> String {
        InterslavicCore::get_noun_stem(word, *number)
    }

    pub fn stem_of_noun_is_soft(word: &str) -> bool {
        InterslavicCore::stem_of_noun_is_soft(word)
    }

    pub fn stem_of_adj_is_soft(word: &str) -> bool {
        InterslavicCore::stem_of_adj_is_soft(word)
    }

    pub fn get_adj_stem(word: &str) -> String {
        InterslavicCore::get_adj_stem(word)
    }
}

impl ISVUTILS {
    pub fn replace_last_occurence(input: &str, pattern: &str, replacement: &str) -> String {
        interslavic_core::utils::replace_last_occurrence(input, pattern, replacement)
    }

    pub fn replace_last_occurrence(input: &str, pattern: &str, replacement: &str) -> String {
        interslavic_core::utils::replace_last_occurrence(input, pattern, replacement)
    }

    pub fn iotation_merge(root: &str, suffix: &str) -> String {
        interslavic_core::utils::iotation_merge(root, suffix)
    }

    pub fn is_vowel(c: &char) -> bool {
        interslavic_core::utils::is_vowel(*c)
    }

    pub fn ends_with_soft_consonant(word: &str) -> bool {
        interslavic_core::utils::ends_with_soft_consonant(word)
    }

    pub fn is_hard_consonant(c: &char) -> bool {
        interslavic_core::utils::is_hard_consonant(*c)
    }

    pub fn is_soft_consonant(c: &char) -> bool {
        interslavic_core::utils::is_soft_consonant(*c)
    }

    pub fn last_in_stringslice(s: &str) -> char {
        s.chars().last().unwrap_or(' ')
    }

    pub fn is_consonant(c: &char) -> bool {
        interslavic_core::utils::is_consonant(*c)
    }

    pub fn string_without_last_n(s: &str, n: i64) -> String {
        interslavic_core::utils::without_last_n(s, n.max(0) as usize)
    }
}

fn noun_meta(entry: NounEntry) -> NounMeta {
    NounMeta {
        gender: entry.gender,
        animacy: entry.animacy,
        indeclinable: entry.indeclinable,
        singular_only: entry.singular_only,
        plural_only: entry.plural_only,
    }
}

fn normalize_lookup(word: &str) -> String {
    word.trim()
        .trim_start_matches('!')
        .trim_matches(|c| c == '[' || c == ']')
        .to_lowercase()
}

fn get_noun(word: &str) -> Option<NounEntry> {
    let needle = normalize_lookup(word);
    NOUNS
        .binary_search_by(|entry| entry.word.cmp(needle.as_str()))
        .ok()
        .map(|idx| NOUNS[idx])
}

fn get_verb(word: &str) -> Option<&'static str> {
    let needle = normalize_lookup(word);
    VERBS
        .binary_search_by(|entry| entry.word.cmp(needle.as_str()))
        .ok()
        .map(|idx| VERBS[idx].word)
}

fn get_adjective(word: &str) -> Option<&'static str> {
    let needle = normalize_lookup(word);
    ADJECTIVES
        .binary_search_by(|entry| entry.word.cmp(needle.as_str()))
        .ok()
        .map(|idx| ADJECTIVES[idx].word)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn readme_regressions() {
        assert_eq!(
            Interslavic::noun("mųž", Case::Gen, Number::Singular),
            "mųža"
        );
        assert_eq!(
            Interslavic::adjective(
                "samy",
                Case::Gen,
                Number::Singular,
                Gender::Masculine,
                Animacy::Animate,
            ),
            "samogo"
        );
        assert_eq!(
            Interslavic::verb("učiti", Person::First, Number::Singular, Tense::Present),
            "učų"
        );
        assert_eq!(
            Interslavic::l_participle("buditi", &Gender::Feminine, &Number::Singular),
            "budila"
        );
    }
}
