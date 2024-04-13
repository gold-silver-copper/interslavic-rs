use crate::basic::*;
use crate::enums::*;
use crate::CaseForms;

#[derive(Debug, PartialEq, Clone)]
pub struct Noun {
    pub nom_sg: String,
    pub conjugated_noun: ConjugatedNoun,
    pub animate: bool,
    pub gender: Gender,
    pub declineable: bool,
}

impl Noun {
    pub fn new(record: &ISVEntry) -> Self {
        let gender = record.get_gender();

        let boop = if gender == None {
            Gender::Neuter
        } else {
            gender.unwrap()
        };

        let declineable = record.is_declineable();
        let animate = record.is_animate();

        let word = &record.isv;
        let cn: ConjugatedNoun = if declineable {
            ConjugatedNoun::derive_noun(word, &boop, animate)
        } else {
            ConjugatedNoun::indeclineable(word)
        };

        Noun {
            nom_sg: word.into(),
            conjugated_noun: cn,
            animate: animate,
            gender: boop,
            declineable: declineable,
        }
    }

    pub fn nom_sg(&self) -> String {
        self.nom_sg.to_string()
    }

    pub fn nom_pl(&self) -> String {
        self.conjugated_noun.pl.nom.clone()
    }

    pub fn acc_sg(&self) -> String {
        self.conjugated_noun.sg.acc.clone()
    }

    pub fn acc_pl(&self) -> String {
        self.conjugated_noun.pl.acc.clone()
    }
    pub fn dat_sg(&self) -> String {
        self.conjugated_noun.sg.dat.clone()
    }

    pub fn dat_pl(&self) -> String {
        self.conjugated_noun.pl.dat.clone()
    }
    pub fn gen_sg(&self) -> String {
        self.conjugated_noun.sg.gen.clone()
    }

    pub fn gen_pl(&self) -> String {
        self.conjugated_noun.pl.gen.clone()
    }
    pub fn ins_sg(&self) -> String {
        self.conjugated_noun.sg.ins.clone()
    }

    pub fn ins_pl(&self) -> String {
        self.conjugated_noun.pl.ins.clone()
    }
    pub fn loc_sg(&self) -> String {
        self.conjugated_noun.sg.loc.clone()
    }

    pub fn loc_pl(&self) -> String {
        self.conjugated_noun.pl.loc.clone()
    }
    pub fn voc_sg(&self) -> String {
        self.conjugated_noun.sg.voc.clone()
    }

    pub fn voc_pl(&self) -> String {
        self.conjugated_noun.pl.voc.clone()
    }
}
#[derive(Debug, PartialEq, Clone)]
pub struct ConjugatedNoun {
    pub sg: CaseForms, // Singular forms of the noun
    pub pl: CaseForms, // Plural forms of the noun
}

impl ConjugatedNoun {
    pub fn derive_noun(word: &str, gender: &Gender, animacy: bool) -> ConjugatedNoun {
        match gender {
            Gender::Masculine => ConjugatedNoun::masculine_decline(word, animacy),
            Gender::Feminine => ConjugatedNoun::feminine_decline(word),
            Gender::Neuter => ConjugatedNoun::neuter_decline(word),
            _ => panic!("GENDER IS ERROR"),
        }
    }
    pub fn masculine_decline(word: &str, animate: bool) -> ConjugatedNoun {
        if word.ends_with("a") {
            let word_stem = slice_without_last(word);

            if is_hard_consonant(last_in_slice(&word_stem)) {
                ConjugatedNoun::masculine_a_hard(&word_stem)
            } else if is_soft_consonant(last_in_slice(&word_stem)) {
                ConjugatedNoun::masculine_a_soft(&word_stem)
            } else {
                panic!("masc word has wrong ending: {}", &word_stem)
            }
        } else if word.ends_with("y") || word.ends_with("i") {
            //členistonogy (-ogo)

            let word_stem = slice_without_last(word);

            if is_hard_consonant(last_in_slice(&word_stem)) {
                ConjugatedNoun::masculine_anim_hard(&word_stem)
            } else if is_soft_consonant(last_in_slice(&word_stem)) {
                ConjugatedNoun::masculine_anim_soft(&word_stem)
            } else {
                panic!("masc word has wrong ending: {}", &word_stem)
            }
        } else if word.ends_with("u") {
            //kenguru

            let word_stem = slice_without_last(word);

            if is_hard_consonant(last_in_slice(&word_stem)) {
                ConjugatedNoun::masculine_anim_hard(word)
            } else if is_soft_consonant(last_in_slice(&word_stem)) {
                ConjugatedNoun::masculine_anim_soft(word)
            } else {
                panic!("masc word has wrong ending: {}", &word_stem)
            }
        } else if word.ends_with("o") {
            ConjugatedNoun::neuter_hard(word)
        } else {
            if animate {
                if is_hard_consonant(last_in_slice(word)) {
                    ConjugatedNoun::masculine_anim_hard(word)
                } else if is_soft_consonant(last_in_slice(word)) {
                    ConjugatedNoun::masculine_anim_soft(word)
                } else {
                    panic!("masc word has wrong ending: {}", word)
                }
            } else {
                if is_hard_consonant(last_in_slice(word)) {
                    ConjugatedNoun::masculine_inanim_hard(word)
                } else if is_soft_consonant(last_in_slice(word)) {
                    ConjugatedNoun::masculine_inanim_soft(word)
                } else {
                    panic!("masc word has wrong ending: {}", word)
                }
            }
        }
    }

    pub fn feminine_decline(word: &str) -> ConjugatedNoun {
        let word_stem = slice_without_last(word);
        if is_consonant(last_in_slice(word)) {
            ConjugatedNoun::fem_cons(word)
        } else if is_soft_consonant(last_in_slice(&word_stem)) {
            ConjugatedNoun::fem_soft(word)
        } else if is_hard_consonant(last_in_slice(&word_stem)) {
            ConjugatedNoun::fem_hard(word)
        } else if is_vowel(last_in_slice(&word_stem)) {
            ConjugatedNoun::fem_hard(word)
        } else {
            panic!("fem word has wrong ending: {}", word)
        }
    }

    pub fn neuter_decline(word: &str) -> ConjugatedNoun {
        if word.ends_with("o") {
            ConjugatedNoun::neuter_hard(word)
        } else if word.ends_with("e") {
            ConjugatedNoun::neuter_soft(word)
        } else if word.ends_with("ę") {
            ConjugatedNoun::neuter_en(word)
        } else if word.ends_with("a") {
            ConjugatedNoun::feminine_decline(word)
        } else {
            panic!("neuter word has wrong ending: {}", word)
        }
    }

    pub fn neuter_hard(word: &str) -> Self {
        let word_stem = slice_without_last(word);

        ConjugatedNoun {
            sg: CaseForms::neuter_hard_singular(&word_stem),
            pl: CaseForms::neuter_hard_plural(&word_stem),
        }
    }
    pub fn neuter_soft(word: &str) -> Self {
        let word_stem = slice_without_last(word);

        ConjugatedNoun {
            sg: CaseForms::neuter_soft_singular(&word_stem),
            pl: CaseForms::neuter_soft_plural(&word_stem),
        }
    }
    pub fn neuter_en(word: &str) -> Self {
        let word_stem = slice_without_last(word);

        ConjugatedNoun {
            sg: CaseForms::neuter_en_singular(&word_stem),
            pl: CaseForms::neuter_en_plural(&word_stem),
        }
    }

    pub fn fem_hard(word: &str) -> Self {
        let word_stem = slice_without_last(word);

        ConjugatedNoun {
            sg: CaseForms::feminine_hard_singular(&word_stem),
            pl: CaseForms::feminine_hard_plural(&word_stem),
        }
    }
    pub fn fem_soft(word: &str) -> Self {
        let word_stem = slice_without_last(word);

        ConjugatedNoun {
            sg: CaseForms::feminine_soft_singular(&word_stem),
            pl: CaseForms::feminine_soft_plural(&word_stem),
        }
    }
    pub fn fem_cons(word: &str) -> Self {
        ConjugatedNoun {
            sg: CaseForms::feminine_cons_singular(word),
            pl: CaseForms::feminine_cons_plural(word),
        }
    }

    pub fn masculine_anim_hard(word: &str) -> Self {
        ConjugatedNoun {
            sg: CaseForms::masculine_anim_hard_singular(word),
            pl: CaseForms::masculine_anim_hard_plural(word),
        }
    }

    pub fn masculine_anim_soft(word: &str) -> Self {
        ConjugatedNoun {
            sg: CaseForms::masculine_anim_soft_singular(word),
            pl: CaseForms::masculine_anim_soft_plural(word),
        }
    }

    pub fn masculine_inanim_hard(word: &str) -> Self {
        ConjugatedNoun {
            sg: CaseForms::masculine_inanim_hard_singular(word),
            pl: CaseForms::masculine_inanim_hard_plural(word),
        }
    }

    pub fn masculine_inanim_soft(word: &str) -> Self {
        ConjugatedNoun {
            sg: CaseForms::masculine_inanim_soft_singular(word),
            pl: CaseForms::masculine_inanim_soft_plural(word),
        }
    }
    pub fn masculine_a_hard(word: &str) -> Self {
        ConjugatedNoun {
            sg: CaseForms::feminine_hard_singular(word),
            pl: CaseForms::masculine_anim_hard_plural(word),
        }
    }
    pub fn masculine_a_soft(word: &str) -> Self {
        ConjugatedNoun {
            sg: CaseForms::feminine_soft_singular(word),
            pl: CaseForms::masculine_anim_soft_plural(word),
        }
    }
    pub fn indeclineable(word: &str) -> Self {
        ConjugatedNoun {
            sg: CaseForms::indeclineable(word),
            pl: CaseForms::indeclineable(word),
        }
    }
}
