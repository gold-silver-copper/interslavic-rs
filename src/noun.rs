use crate::basic::*;
use crate::Animacy;
pub struct Noun {
    sg: CaseForms, // Singular forms of the noun
    pl: CaseForms, // Plural forms of the noun
}
pub struct CaseForms {
    nom: String, // Nominative
    gen: String, // Genitive
    dat: String, // Dative
    acc: String, // Accusative
    ins: String, // Instrumental
    loc: String, // Locative
    voc: String, // Vocative
}

impl CaseForms {
    pub fn neuter_hard_singular(word_stem: &str) -> Self {
        CaseForms {
            nom: format!("{word_stem}o"),
            acc: format!("{word_stem}o"),
            gen: format!("{word_stem}a"),
            dat: format!("{word_stem}u"),

            ins: format!("{word_stem}om"),
            loc: format!("{word_stem}u"),
            voc: format!("{word_stem}o"),
        }
    }
    pub fn neuter_hard_plural(word_stem: &str) -> Self {
        CaseForms {
            nom: format!("{word_stem}a"),
            acc: format!("{word_stem}a"),
            gen: format!("{word_stem}"),
            dat: format!("{word_stem}am"),

            ins: format!("{word_stem}ami"),
            loc: format!("{word_stem}ah"),
            voc: format!("{word_stem}a"),
        }
    }
    pub fn neuter_soft_singular(word_stem: &str) -> Self {
        CaseForms {
            nom: format!("{word_stem}e"),
            acc: format!("{word_stem}e"),
            gen: format!("{word_stem}ja"),
            dat: format!("{word_stem}u"),

            ins: format!("{word_stem}em"),
            loc: format!("{word_stem}u"),
            voc: format!("{word_stem}e"),
        }
    }
    pub fn neuter_soft_plural(word_stem: &str) -> Self {
        CaseForms {
            nom: format!("{word_stem}a"),
            acc: format!("{word_stem}a"),
            gen: format!("{word_stem}ej"),
            dat: format!("{word_stem}am"),

            ins: format!("{word_stem}ami"),
            loc: format!("{word_stem}ah"),
            voc: format!("{word_stem}a"),
        }
    }
    pub fn neuter_en_singular(word_stem: &str) -> Self {
        CaseForms {
            nom: format!("{word_stem}ę"),
            acc: format!("{word_stem}ę"),
            gen: format!("{word_stem}ene"),
            dat: format!("{word_stem}eni"),

            ins: format!("{word_stem}enem"),
            loc: format!("{word_stem}eni"),
            voc: format!("{word_stem}ę"),
        }
    }
    pub fn neuter_en_plural(word_stem: &str) -> Self {
        CaseForms {
            nom: format!("{word_stem}ena"),
            acc: format!("{word_stem}ena"),
            gen: format!("{word_stem}en"),
            dat: format!("{word_stem}enam"),

            ins: format!("{word_stem}enami"),
            loc: format!("{word_stem}enah"),
            voc: format!("{word_stem}ena"),
        }
    }


    pub fn feminine_hard_singular(word_stem: &str) -> Self {
        CaseForms {
            nom: format!("{word_stem}a"),
            acc: format!("{word_stem}u"),
            gen: format!("{word_stem}y"),
            dat: format!("{word_stem}ě"),

            ins: format!("{word_stem}oju"),
            loc: format!("{word_stem}ě"),
            voc: format!("{word_stem}o"),
        }
    }
    pub fn feminine_hard_plural(word_stem: &str) -> Self {
        CaseForms {
            nom: format!("{word_stem}y"),
            acc: format!("{word_stem}y"),
            gen: format!("{word_stem}"),
            dat: format!("{word_stem}am"),

            ins: format!("{word_stem}ami"),
            loc: format!("{word_stem}ah"),
            voc: format!("{word_stem}y"),
        }
    }

    pub fn feminine_soft_singular(word_stem: &str) -> Self {
        CaseForms {
            nom: format!("{word_stem}a"),
            acc: format!("{word_stem}u"),
            gen: format!("{word_stem}je"),
            dat: format!("{word_stem}i"),

            ins: format!("{word_stem}eju"),
            loc: format!("{word_stem}i"),
            voc: format!("{word_stem}o"),
        }
    }
    pub fn feminine_soft_plural(word_stem: &str) -> Self {
        CaseForms {
            nom: format!("{word_stem}e"),
            acc: format!("{word_stem}e"),
            gen: format!("{word_stem}ej"),
            dat: format!("{word_stem}am"),

            ins: format!("{word_stem}ami"),
            loc: format!("{word_stem}ah"),
            voc: format!("{word_stem}e"),
        }
    }

    pub fn feminine_cons_singular(word_stem: &str) -> Self {
        CaseForms {
            nom: format!("{word_stem}"),
            acc: format!("{word_stem}"),
            gen: format!("{word_stem}i"),
            dat: format!("{word_stem}i"),

            ins: format!("{word_stem}ju"),
            loc: format!("{word_stem}i"),
            voc: format!("{word_stem}i"),
        }
    }
    pub fn feminine_cons_plural(word_stem: &str) -> Self {
        CaseForms {
            nom: format!("{word_stem}i"),
            acc: format!("{word_stem}i"),
            gen: format!("{word_stem}ij"),
            dat: format!("{word_stem}am"),

            ins: format!("{word_stem}ami"),
            loc: format!("{word_stem}ah"),
            voc: format!("{word_stem}i"),
        }
    }

}

fn main() {
    let char = 'a';
    println!("Is '{}' a vowel? {}", char, is_vowel(char));
    println!("Is '{}' a consonant? {}", char, is_consonant(char));
}

impl Noun {
    pub fn masc_decline(word: &str, animacy: Animacy) -> Noun {
        if word.ends_with("o") {
            Noun::neuter_hard(word)
        } else if word.ends_with("e") {
            Noun::neuter_soft(word)
        } else if word.ends_with("ę") {
            Noun::neuter_en(word)
        } else {
            panic!("masc word has wrong ending: {}", word)
        }
    }

    pub fn femn_decline(word: &str) -> Noun {
        let word_stem = slice_without_last(word);
        if is_consonant(last_in_slice(word)) {
            Noun::fem_cons(word)
        } else if is_soft_consonant(last_in_slice(&word_stem)) {
            Noun::fem_soft(word)
        } else if is_hard_consonant(last_in_slice(&word_stem)) {
            Noun::fem_hard(word)
        } else {
            panic!("fem word has wrong ending: {}", word)
        }
    }

    pub fn neut_decline(word: &str) -> Noun {
        if word.ends_with("o") {
            Noun::neuter_hard(word)
        } else if word.ends_with("e") {
            Noun::neuter_soft(word)
        } else if word.ends_with("ę") {
            Noun::neuter_en(word)
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
}
