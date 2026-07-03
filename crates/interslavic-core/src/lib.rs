#![doc = "Rule-based Interslavic morphology without bundled dictionary data."]

mod adjective;
mod case_endings;
pub mod grammar;
mod irregular_verbs;
mod noun;
pub mod utils;
mod verb;
mod verb_endings;

pub use crate::adjective::Adjective;
pub use crate::grammar::*;
pub use crate::noun::{Noun, NounMeta};
pub use crate::verb::Verb;

/// Pure rule engine for Interslavic inflection.
///
/// This crate deliberately contains no dictionary data. Use the `interslavic`
/// crate for dictionary-backed metadata and generated lookup tables.
pub struct InterslavicCore;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn regular_noun_declension() {
        assert_eq!(
            InterslavicCore::noun("mųž", Case::Gen, Number::Singular, None),
            "mųža"
        );
    }

    #[test]
    fn regular_adjective_declension() {
        assert_eq!(
            InterslavicCore::adjective(
                "samy",
                Case::Gen,
                Number::Singular,
                Gender::Masculine,
                Animacy::Animate,
            ),
            "samogo"
        );
    }

    #[test]
    fn present_verb_conjugation() {
        assert_eq!(
            InterslavicCore::verb("učiti", Person::First, Number::Singular, Tense::Present),
            "učų"
        );
    }

    #[test]
    fn l_participle() {
        assert_eq!(
            InterslavicCore::l_participle("buditi", Gender::Feminine, Number::Singular),
            "budila"
        );
    }
}
