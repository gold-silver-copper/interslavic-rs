use crate::case_endings::*;
use crate::grammar::{Animacy, Case, Gender, Number};
use crate::utils::{ends_with_soft_consonant, is_vowel, last_n_chars, without_last_n};
use crate::InterslavicCore;

pub type Noun = (String, Gender);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct NounMeta {
    pub gender: Gender,
    pub animacy: Animacy,
    pub indeclinable: bool,
    pub singular_only: bool,
    pub plural_only: bool,
}

impl NounMeta {
    pub const fn new(gender: Gender, animacy: Animacy) -> Self {
        Self {
            gender,
            animacy,
            indeclinable: false,
            singular_only: false,
            plural_only: false,
        }
    }
}

impl InterslavicCore {
    pub fn noun(word: &str, case: Case, number: Number, meta: Option<NounMeta>) -> String {
        Self::decline_noun_with_meta(word, case, number, meta).0
    }

    pub fn decline_noun_with_meta(
        word: &str,
        case: Case,
        number: Number,
        meta: Option<NounMeta>,
    ) -> Noun {
        let word = word.to_lowercase();
        if meta.is_some_and(|m| m.indeclinable) {
            return (word, meta.unwrap().gender);
        }

        let gender = meta.map_or_else(|| Self::guess_gender(&word), |m| m.gender);
        let animacy = meta.map_or_else(
            || {
                if Self::noun_is_animate(&word) {
                    Animacy::Animate
                } else {
                    Animacy::Inanimate
                }
            },
            |m| m.animacy,
        );
        let word_stem_is_soft = Self::stem_of_noun_is_soft(&word);
        let word_stem = Self::get_noun_stem(&word, number);

        let endings = if Self::is_ost_class(&word) {
            &OST_ENDINGS
        } else {
            match gender {
                Gender::Masculine => match (animacy, word_stem_is_soft) {
                    (Animacy::Animate, true) => &ANIMATE_SOFT_ENDINGS,
                    (Animacy::Animate, false) => &ANIMATE_HARD_ENDINGS,
                    (Animacy::Inanimate, true) => &INANIMATE_SOFT_ENDINGS,
                    (Animacy::Inanimate, false) => &INANIMATE_HARD_ENDINGS,
                },
                Gender::Feminine => {
                    if word_stem_is_soft {
                        &FEMININE_SOFT_ENDINGS
                    } else {
                        &FEMININE_HARD_ENDINGS
                    }
                }
                Gender::Neuter => {
                    if word_stem_is_soft {
                        &NEUTER_SOFT_ENDINGS
                    } else {
                        &NEUTER_HARD_ENDINGS
                    }
                }
            }
        };

        (
            format!("{}{}", word_stem, endings.ending(case, number)),
            gender,
        )
    }

    pub fn noun_is_animate(word: &str) -> bool {
        matches!(word, "konj" | "mųž" | "člověk" | "brat" | "syn" | "otec")
    }

    pub fn guess_gender(word: &str) -> Gender {
        let last_one = last_n_chars(word, 1);

        if Self::is_ost_class(word) || last_one == "a" || last_one == "i" {
            Gender::Feminine
        } else if last_one == "o" || last_one == "e" {
            Gender::Neuter
        } else {
            Gender::Masculine
        }
    }

    pub fn is_ost_class(word: &str) -> bool {
        word.ends_with("ost́")
            || word.ends_with("osť")
            || word.ends_with("ěć")
            || word.ends_with("ěč")
            || word.ends_with("eć")
            || word.ends_with("at́")
    }

    pub fn get_noun_stem(word: &str, number: Number) -> String {
        if word.ends_with("anin") && number == Number::Plural {
            return without_last_n(word, 2);
        }
        if word.ends_with("anina") && number == Number::Plural {
            return without_last_n(word, 3);
        }

        if word.chars().last().is_some_and(is_vowel) {
            without_last_n(word, 1)
        } else {
            word.to_string()
        }
    }

    pub fn stem_of_noun_is_soft(word: &str) -> bool {
        ends_with_soft_consonant(&Self::get_noun_stem(word, Number::Singular))
    }
}
