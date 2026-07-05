//! Interslavic inflection with embedded dictionary metadata.
//!
//! `interslavic-core` contains the dependency-free morphology rules. This crate
//! adds generated dictionary metadata for noun lookup and keeps the public API
//! focused on single-form inflection.

use interslavic_core::ISVCore;
pub use interslavic_core::{Animacy, Case, Gender, NounGender, Number, Person, Tense};

mod dictionary;
use dictionary::*;

/// Short dictionary-backed API.
pub struct ISV;

impl ISV {
    /// Return one dictionary-backed noun form. Unknown words fall back to the
    /// core rule engine's gender and animacy inference.
    pub fn noun(lemma: &str, case: Case, number: Number) -> String {
        let entries = lookup_nouns_by_lemma(lemma);
        if let Some(entry) = entries.first() {
            return Self::noun_from_entry(entry, case, number, None, None);
        }

        ISVCore::decline_noun(lemma, &case, &number)
    }

    /// Return one noun form with explicit gender and animacy.
    ///
    /// If the noun exists in the generated dictionary, dictionary-only metadata
    /// such as fleeting-vowel additions, number restrictions, and indeclinability
    /// is still used; `gender` and `animacy` override the dictionary row.
    pub fn noun_with(
        lemma: &str,
        case: Case,
        number: Number,
        gender: NounGender,
        animacy: Animacy,
    ) -> String {
        let entries = lookup_nouns_by_lemma(lemma);
        if let Some(entry) =
            Self::select_noun_entry(lemma, entries, case, number, Some(gender), Some(animacy))
        {
            return Self::noun_from_entry(entry, case, number, Some(gender), Some(animacy));
        }

        ISVCore::decline_noun_explicit(
            lemma, &case, &number, gender, animacy, false, false, false, None,
        )
    }

    /// One adjective form. Adjective phrases are not declined as a unit; callers
    /// should model particles/complements separately and pass only the adjective.
    pub fn adj(word: &str, case: Case, number: Number, gender: Gender, animacy: Animacy) -> String {
        ISVCore::decline_adj(word, &case, &number, &gender, animacy)
    }

    /// One present-tense verb form.
    pub fn verb(
        word: &str,
        person: Person,
        number: Number,
        gender: Gender,
        tense: Tense,
    ) -> String {
        let trimmed = word.trim();
        let entries = lookup_verbs_by_lemma(trimmed);
        if let Some(entry) = entries.first() {
            return ISVCore::conjugate_verb_with_present_hint(
                entry.lemma,
                entry.addition,
                &person,
                &number,
                &tense,
            );
        }

        ISVCore::conjugate_verb(trimmed, &person, &number, &gender, &tense)
    }

    /// One present-tense verb form with an explicit dictionary present-stem hint.
    ///
    /// This is intended for typed dictionary rows that have multiple entries for
    /// the same lemma. It does not parse arbitrary phrase strings.
    pub fn verb_with_present_hint(
        word: &str,
        present_hint: &str,
        person: Person,
        number: Number,
        tense: Tense,
    ) -> String {
        ISVCore::conjugate_verb_with_present_hint(
            word.trim(),
            present_hint,
            &person,
            &number,
            &tense,
        )
    }

    fn select_noun_entry<'a>(
        lemma: &str,
        entries: &'a [DictionaryEntry],
        case: Case,
        number: Number,
        gender: Option<NounGender>,
        animacy: Option<Animacy>,
    ) -> Option<&'a DictionaryEntry> {
        entries.iter().max_by_key(|entry| {
            let mut score = 0;
            if entry.lemma == lemma {
                score += 16;
            }
            if gender.is_some_and(|wanted| dictionary_gender_to_api(entry.gender) == wanted) {
                score += 8;
            }
            if animacy.is_some_and(|wanted| {
                let entry_animacy = if entry.animate {
                    Animacy::Animate
                } else {
                    Animacy::Inanimate
                };
                entry_animacy == wanted
            }) {
                score += 4;
            }
            if number == Number::Singular && !entry.plural_only {
                score += 2;
            }
            if number == Number::Plural && !entry.singular_only {
                score += 2;
            }
            if case == Case::Nom {
                score += 1;
            }
            if !entry.indeclinable {
                score += 1;
            }
            score
        })
    }

    fn noun_from_entry(
        entry: &DictionaryEntry,
        case: Case,
        number: Number,
        gender_override: Option<NounGender>,
        animacy_override: Option<Animacy>,
    ) -> String {
        let gender = gender_override.unwrap_or_else(|| dictionary_gender_to_api(entry.gender));
        let animacy = animacy_override.unwrap_or(if entry.animate {
            Animacy::Animate
        } else {
            Animacy::Inanimate
        });

        ISVCore::decline_noun_explicit(
            entry.lemma,
            &case,
            &number,
            gender,
            animacy,
            entry.plural_only,
            entry.singular_only,
            entry.indeclinable,
            (!entry.addition.is_empty()).then_some(entry.addition),
        )
    }
}

fn dictionary_gender_to_api(gender: DictionaryGender) -> NounGender {
    match gender {
        DictionaryGender::Masculine | DictionaryGender::MasculineFeminine => NounGender::Masculine,
        DictionaryGender::Feminine => NounGender::Feminine,
        DictionaryGender::Neuter => NounGender::Neuter,
    }
}
