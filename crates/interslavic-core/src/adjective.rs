use crate::case_endings::*;
use crate::grammar::{Animacy, Case, Gender, Number};
use crate::utils::without_last_n;
use crate::InterslavicCore;

pub type Adjective = String;

impl InterslavicCore {
    pub fn adjective(
        word: &str,
        case: Case,
        number: Number,
        gender: Gender,
        animacy: Animacy,
    ) -> Adjective {
        let word = word.to_lowercase();
        let stem_is_soft = Self::stem_of_adj_is_soft(&word);
        let adj_stem = Self::get_adj_stem(&word);

        let endings = match gender {
            Gender::Masculine => match (animacy, stem_is_soft) {
                (Animacy::Animate, true) => &ADJ_ANIMATE_SOFT_ENDINGS,
                (Animacy::Animate, false) => &ADJ_ANIMATE_HARD_ENDINGS,
                (Animacy::Inanimate, true) => &ADJ_INANIMATE_SOFT_ENDINGS,
                (Animacy::Inanimate, false) => &ADJ_INANIMATE_HARD_ENDINGS,
            },
            Gender::Feminine => {
                if stem_is_soft {
                    &ADJ_FEMININE_SOFT_ENDINGS
                } else {
                    &ADJ_FEMININE_HARD_ENDINGS
                }
            }
            Gender::Neuter => {
                if stem_is_soft {
                    &ADJ_NEUTER_SOFT_ENDINGS
                } else {
                    &ADJ_NEUTER_HARD_ENDINGS
                }
            }
        };

        format!("{}{}", adj_stem, endings.ending(case, number))
    }

    pub fn stem_of_adj_is_soft(word: &str) -> bool {
        word.ends_with('i')
    }

    pub fn get_adj_stem(word: &str) -> String {
        without_last_n(word, 1)
    }
}
