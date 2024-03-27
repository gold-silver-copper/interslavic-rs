use crate::basic::*;
use crate::Animacy;
use crate::CaseForms;
pub struct Noun {
    pub sg: CaseForms, // Singular forms of the noun
    pub pl: CaseForms, // Plural forms of the noun
}

impl Noun {
    pub fn masculine_decline(word: &str, animacy: Animacy) -> Noun {
        if word.ends_with("a") {
            let word_stem = slice_without_last(word);

            if is_hard_consonant(last_in_slice(&word_stem)) {
                Noun::masculine_a_hard(&word_stem)
            } else if is_soft_consonant(last_in_slice(&word_stem)) {
                Noun::masculine_a_soft(&word_stem)
            } else {
                panic!("masc word has wrong ending: {}", &word_stem)
            }
        } else if word.ends_with("y") || word.ends_with("i") {
            //členistonogy (-ogo)

            let word_stem = slice_without_last(word);

            if is_hard_consonant(last_in_slice(&word_stem)) {
                Noun::masculine_anim_hard(&word_stem)
            } else if is_soft_consonant(last_in_slice(&word_stem)) {
                Noun::masculine_anim_soft(&word_stem)
            } else {
                panic!("masc word has wrong ending: {}", &word_stem)
            }
        }
        else if word.ends_with("u") {
            //členistonogy (-ogo)

            let word_stem = slice_without_last(word);

            if is_hard_consonant(last_in_slice(&word_stem)) {
                Noun::masculine_anim_hard(word)
            } else if is_soft_consonant(last_in_slice(&word_stem)) {
                Noun::masculine_anim_soft(word)
            } else {
                panic!("masc word has wrong ending: {}", &word_stem)
            }
        } else if word.ends_with("o") {
            Noun::neuter_hard(word)
        } else {
            match animacy {
                Animacy::Animate => {
                    if is_hard_consonant(last_in_slice(word)) {
                        Noun::masculine_anim_hard(word)
                    } else if is_soft_consonant(last_in_slice(word)) {
                        Noun::masculine_anim_soft(word)
                    } else {
                        panic!("masc word has wrong ending: {}", word)
                    }
                }
                Animacy::Inanimate => {
                    if is_hard_consonant(last_in_slice(word)) {
                        Noun::masculine_inanim_hard(word)
                    } else if is_soft_consonant(last_in_slice(word)) {
                        Noun::masculine_inanim_soft(word)
                    } else {
                        panic!("masc word has wrong ending: {}", word)
                    }
                }
            }
        }
    }

    pub fn feminine_decline(word: &str) -> Noun {
        let word_stem = slice_without_last(word);
        if is_consonant(last_in_slice(word)) {
            Noun::fem_cons(word)
        } else if is_soft_consonant(last_in_slice(&word_stem)) {
            Noun::fem_soft(word)
        } else if is_hard_consonant(last_in_slice(&word_stem)) {
            Noun::fem_hard(word)
        } else if is_vowel(last_in_slice(&word_stem)) {
            Noun::fem_hard(word)
        } else {
            panic!("fem word has wrong ending: {}", word)
        }
    }

    pub fn neuter_decline(word: &str) -> Noun {
        if word.ends_with("o") {
            Noun::neuter_hard(word)
        } else if word.ends_with("e") {
            Noun::neuter_soft(word)
        } else if word.ends_with("ę") {
            Noun::neuter_en(word)
        } else if word.ends_with("a") {
            Noun::feminine_decline(word)
        } else {
            panic!("neuter word has wrong ending: {}", word)
        }
    }

    pub fn neuter_hard(word: &str) -> Self {
        let word_stem = slice_without_last(word);

        Noun {
            sg: CaseForms::neuter_hard_singular(&word_stem),
            pl: CaseForms::neuter_hard_plural(&word_stem),
        }
    }
    pub fn neuter_soft(word: &str) -> Self {
        let word_stem = slice_without_last(word);

        Noun {
            sg: CaseForms::neuter_soft_singular(&word_stem),
            pl: CaseForms::neuter_soft_plural(&word_stem),
        }
    }
    pub fn neuter_en(word: &str) -> Self {
        let word_stem = slice_without_last(word);

        Noun {
            sg: CaseForms::neuter_en_singular(&word_stem),
            pl: CaseForms::neuter_en_plural(&word_stem),
        }
    }

    pub fn fem_hard(word: &str) -> Self {
        let word_stem = slice_without_last(word);

        Noun {
            sg: CaseForms::feminine_hard_singular(&word_stem),
            pl: CaseForms::feminine_hard_plural(&word_stem),
        }
    }
    pub fn fem_soft(word: &str) -> Self {
        let word_stem = slice_without_last(word);

        Noun {
            sg: CaseForms::feminine_soft_singular(&word_stem),
            pl: CaseForms::feminine_soft_plural(&word_stem),
        }
    }
    pub fn fem_cons(word: &str) -> Self {
        Noun {
            sg: CaseForms::feminine_cons_singular(word),
            pl: CaseForms::feminine_cons_plural(word),
        }
    }

    pub fn masculine_anim_hard(word: &str) -> Self {
        Noun {
            sg: CaseForms::masculine_anim_hard_singular(word),
            pl: CaseForms::masculine_anim_hard_plural(word),
        }
    }

    pub fn masculine_anim_soft(word: &str) -> Self {
        Noun {
            sg: CaseForms::masculine_anim_soft_singular(word),
            pl: CaseForms::masculine_anim_soft_plural(word),
        }
    }

    pub fn masculine_inanim_hard(word: &str) -> Self {
        Noun {
            sg: CaseForms::masculine_inanim_hard_singular(word),
            pl: CaseForms::masculine_inanim_hard_plural(word),
        }
    }

    pub fn masculine_inanim_soft(word: &str) -> Self {
        Noun {
            sg: CaseForms::masculine_inanim_soft_singular(word),
            pl: CaseForms::masculine_inanim_soft_plural(word),
        }
    }
    pub fn masculine_a_hard(word: &str) -> Self {
        Noun {
            sg: CaseForms::feminine_hard_singular(word),
            pl: CaseForms::masculine_anim_hard_plural(word),
        }
    }
    pub fn masculine_a_soft(word: &str) -> Self {
        Noun {
            sg: CaseForms::feminine_soft_singular(word),
            pl: CaseForms::masculine_anim_soft_plural(word),
        }
    }
    pub fn indeclineable(word: &str) -> Self {
        Noun {
            sg: CaseForms::indeclineable(word),
            pl: CaseForms::indeclineable(word),
        }
    }
}
