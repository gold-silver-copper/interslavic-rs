//! Interslavic inflection with embedded dictionary metadata.
//!
//! `interslavic-core` contains the dependency-free morphology rules. This crate
//! adds dictionary lookup, homonym handling, and metadata-driven noun paradigms.

pub use interslavic_core::{
    Animacy, Case, Gender, ISVCore, InflectionAlternatives, InflectionError, NounDeclensionRequest,
    NounGender, NounParadigm, Number, NumberRestriction, Person, Tense,
};

mod dictionary;
use dictionary::*;

/// Short dictionary-backed API.
pub struct ISV;

impl ISV {
    /// Return every dictionary-backed noun paradigm for a lemma.
    ///
    /// Homonyms produce multiple paradigms. `m./f.` entries produce both a
    /// masculine and feminine paradigm.
    pub fn noun(lemma: &str) -> Result<Vec<NounParadigm>, InflectionError> {
        let entries = lookup_nouns_by_lemma(lemma);
        if entries.is_empty() {
            return Err(InflectionError::DictionaryEntryNotFound);
        }

        let mut out = Vec::new();
        for entry in entries {
            if entry.gender == Some(DictionaryGender::MasculineOrFeminine) {
                out.push(Self::paradigm(entry, Some(NounGender::Masculine))?);
                out.push(Self::paradigm(entry, Some(NounGender::Feminine))?);
            } else {
                out.push(Self::paradigm(entry, None)?);
            }
        }
        Ok(out)
    }

    /// Return one dictionary-backed noun paradigm by dictionary id.
    ///
    /// `m./f.` entries require [`ISV::noun_id_as`].
    pub fn noun_id(id: &str) -> Result<NounParadigm, InflectionError> {
        let entry = lookup_nouns_by_id(id)
            .into_iter()
            .next()
            .ok_or(InflectionError::DictionaryEntryNotFound)?;
        if entry.gender == Some(DictionaryGender::MasculineOrFeminine) {
            return Err(InflectionError::MissingGenderOverride);
        }
        Self::paradigm(entry, None)
    }

    /// Return one dictionary-backed noun paradigm by id with explicit gender.
    ///
    /// This is the correct path for `m./f.` entries such as `luč`.
    pub fn noun_id_as(id: &str, gender: NounGender) -> Result<NounParadigm, InflectionError> {
        let entry = lookup_nouns_by_id(id)
            .into_iter()
            .next()
            .ok_or(InflectionError::DictionaryEntryNotFound)?;
        Self::paradigm(entry, Some(gender))
    }

    /// Decline a noun from explicit caller-provided morphology, without using
    /// dictionary lookup.
    pub fn noun_as(request: NounDeclensionRequest<'_>) -> Result<NounParadigm, InflectionError> {
        ISVCore::decline_noun_explicit(request)
    }

    /// Convenience one-form lookup. If a lemma is ambiguous, this returns the
    /// first dictionary row; use [`ISV::noun`] or [`ISV::noun_id`] for exactness.
    pub fn noun_form(
        lemma: &str,
        case: Case,
        number: Number,
    ) -> Result<InflectionAlternatives, InflectionError> {
        match Self::noun(lemma) {
            Ok(paradigms) => paradigms
                .into_iter()
                .next()
                .and_then(|paradigm| paradigm.get(case, number).cloned())
                .ok_or(InflectionError::DictionaryEntryNotFound),
            Err(InflectionError::DictionaryEntryNotFound) => {
                let form = ISVCore::decline_noun(lemma, &case, &number).0;
                Ok(InflectionAlternatives {
                    alternatives: form.split('/').map(str::trim).map(str::to_string).collect(),
                })
            }
            Err(err) => Err(err),
        }
    }

    /// One adjective form. Compound adjective phrases are handled by the core
    /// rules (`osnovany na` -> `osnovanogo na`).
    pub fn adj(word: &str, case: Case, number: Number, gender: Gender, animate: bool) -> String {
        ISVCore::decline_adj(word, &case, &number, &gender, animate)
    }

    /// One present-tense verb form.
    pub fn verb(
        word: &str,
        person: Person,
        number: Number,
        gender: Gender,
        tense: Tense,
    ) -> String {
        ISVCore::conjugate_verb(word, &person, &number, &gender, &tense)
    }

    fn paradigm(
        entry: &DictionaryEntry,
        gender_override: Option<NounGender>,
    ) -> Result<NounParadigm, InflectionError> {
        let gender = dictionary_gender_to_api(
            entry
                .gender
                .as_ref()
                .unwrap_or(&DictionaryGender::Masculine),
        );
        ISVCore::decline_noun_explicit(NounDeclensionRequest {
            lemma: &entry.lemma,
            gender,
            animacy: if entry.animate {
                Animacy::Animate
            } else {
                Animacy::Inanimate
            },
            number_restriction: if entry.plural_only {
                NumberRestriction::PluralOnly
            } else if entry.singular_only {
                NumberRestriction::SingularOnly
            } else {
                NumberRestriction::Countable
            },
            indeclinable: entry.indeclinable,
            addition: (!entry.addition.is_empty()).then_some(entry.addition.as_str()),
            gender_override,
        })
    }
}

fn dictionary_gender_to_api(gender: &DictionaryGender) -> NounGender {
    match gender {
        DictionaryGender::Masculine => NounGender::Masculine,
        DictionaryGender::Feminine => NounGender::Feminine,
        DictionaryGender::Neuter => NounGender::Neuter,
        DictionaryGender::MasculineOrFeminine => NounGender::MasculineOrFeminine,
    }
}
