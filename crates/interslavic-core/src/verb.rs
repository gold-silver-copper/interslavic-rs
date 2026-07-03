use crate::grammar::{Conjugation, Gender, Number, Person, Tense};
use crate::irregular_verbs::irregular_present_stem;
use crate::utils::{iotation_merge, is_consonant, replace_last_occurrence, without_last_n};
use crate::verb_endings::{FIRST_CONJUGATION, SECOND_CONJUGATION};
use crate::InterslavicCore;

pub type Verb = String;

impl InterslavicCore {
    pub fn verb(word: &str, person: Person, number: Number, tense: Tense) -> Verb {
        match tense {
            Tense::Present => Self::conjugate_present(word, person, number),
            _ => word.to_string(),
        }
    }

    pub fn conjugate_present(word: &str, person: Person, number: Number) -> Verb {
        let word = word.to_lowercase();
        let (present_stem, conjugation) = Self::get_present_tense_stem(&word);
        let endings = match conjugation {
            Conjugation::First => &FIRST_CONJUGATION,
            Conjugation::Second => &SECOND_CONJUGATION,
        };
        iotation_merge(&present_stem, endings.ending(person, number))
    }

    pub fn get_present_tense_stem(infinitive: &str) -> (String, Conjugation) {
        let infinitive_stem = Self::get_infinitive_stem(infinitive);

        if let Some(irregular) = irregular_present_stem(infinitive) {
            if irregular.ends_with("me") {
                return (
                    replace_last_occurrence(&irregular, "me", "m"),
                    Conjugation::First,
                );
            }
            if irregular.ends_with("ne") {
                return (
                    replace_last_occurrence(&irregular, "ne", "n"),
                    Conjugation::First,
                );
            }
            if irregular.ends_with("je") {
                return (
                    replace_last_occurrence(&irregular, "je", "j"),
                    Conjugation::First,
                );
            }
            if irregular.ends_with('e') {
                return (
                    replace_last_occurrence(&irregular, "e", ""),
                    Conjugation::First,
                );
            }
            if irregular.ends_with('i') {
                return (
                    replace_last_occurrence(&irregular, "i", ""),
                    Conjugation::Second,
                );
            }
        }

        if infinitive_stem.chars().last().is_some_and(is_consonant) {
            (infinitive_stem, Conjugation::First)
        } else if infinitive.ends_with("ovati") {
            (infinitive.replace("ovati", "uj"), Conjugation::First)
        } else if infinitive.ends_with("nųti") {
            (infinitive.replace("nųti", "n"), Conjugation::First)
        } else if infinitive.ends_with("ati") {
            (
                replace_last_occurrence(infinitive, "ati", "aj"),
                Conjugation::First,
            )
        } else if infinitive.ends_with("eti") {
            (
                replace_last_occurrence(infinitive, "eti", "ej"),
                Conjugation::First,
            )
        } else if infinitive.ends_with("ęti") {
            (
                replace_last_occurrence(infinitive, "ęti", "n"),
                Conjugation::First,
            )
        } else if infinitive.ends_with("yti") {
            (
                replace_last_occurrence(infinitive, "yti", "yj"),
                Conjugation::First,
            )
        } else if infinitive.ends_with("uti") {
            (
                replace_last_occurrence(infinitive, "uti", "uj"),
                Conjugation::First,
            )
        } else if infinitive.ends_with("iti") {
            (
                replace_last_occurrence(infinitive, "iti", ""),
                Conjugation::Second,
            )
        } else if infinitive.ends_with("ěti") {
            (
                replace_last_occurrence(infinitive, "ěti", ""),
                Conjugation::Second,
            )
        } else {
            (infinitive_stem, Conjugation::First)
        }
    }

    pub fn get_infinitive_stem(word: &str) -> String {
        without_last_n(word, 2)
    }

    pub fn l_participle(word: &str, gender: Gender, number: Number) -> Verb {
        if word == "idti" {
            match number {
                Number::Plural => String::from("šli"),
                Number::Singular => match gender {
                    Gender::Masculine => String::from("šėl"),
                    Gender::Feminine => String::from("šla"),
                    Gender::Neuter => String::from("šlo"),
                },
            }
        } else {
            let infinitive_stem = Self::get_infinitive_stem(word);
            match number {
                Number::Plural => format!("{}li", infinitive_stem),
                Number::Singular => match gender {
                    Gender::Masculine => format!("{}l", infinitive_stem),
                    Gender::Feminine => format!("{}la", infinitive_stem),
                    Gender::Neuter => format!("{}lo", infinitive_stem),
                },
            }
        }
    }
}
