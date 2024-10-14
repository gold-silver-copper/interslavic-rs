mod case_endings;
use case_endings::*;
mod verb_endings;
use verb_endings::*;
mod dictionary_initialization;
mod irregular_verbs;
use dictionary_initialization::*;
use irregular_verbs::*;

#[derive(Debug, Clone, Default)]
pub struct ISV {
    pub animate_nouns: Vec<String>,
    pub nonanimate_nouns: Vec<String>,
    pub feminine_nouns: Vec<String>,
    pub neuter_nouns: Vec<String>,
    pub irregular_noun_stems: Vec<String>,
}

pub struct ISVUTILS {}

#[derive(Debug, PartialEq, Clone)]
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

#[derive(Debug, PartialEq, Clone)]
pub enum Number {
    Singular,
    Plural,
}
#[derive(Debug, PartialEq, Clone)]
pub enum Case {
    Nom,
    Acc,
    Gen,
    Loc,
    Dat,
    Ins,
    //vocative will be handle seperately
}
#[derive(Debug, PartialEq, Clone)]
pub enum Gender {
    Masculine,
    Feminine,
    Neuter,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Person {
    First,
    Second,
    Third,
}
#[derive(Debug, PartialEq, Clone)]
pub enum Conjugation {
    First,
    Second,
}
#[derive(Debug, PartialEq, Clone)]
pub enum Tense {
    Present,
    Imperfect,
    Future,
    Perfect,
    PluPerfect,
    Conditional,
}

impl CaseEndings {
    pub fn ending(&self, case: &Case, number: &Number) -> &str {
        match number {
            Number::Singular => match case {
                Case::Nom => self.nom_sg,
                Case::Acc => self.acc_sg,
                Case::Gen => self.gen_sg,
                Case::Loc => self.loc_sg,
                Case::Dat => self.dat_sg,
                Case::Ins => self.ins_sg,
            },
            Number::Plural => match case {
                Case::Nom => self.nom_pl,
                Case::Acc => self.acc_pl,
                Case::Gen => self.gen_pl,
                Case::Loc => self.loc_pl,
                Case::Dat => self.dat_pl,
                Case::Ins => self.ins_pl,
            },
        }
    }
}
impl VerbEndings {
    pub fn ending(&self, person: &Person, number: &Number) -> &str {
        match (person, number) {
            (Person::First, Number::Singular) => self.first_singular,
            (Person::Second, Number::Singular) => self.second_singular,
            (Person::Third, Number::Singular) => self.third_singular,
            (Person::First, Number::Plural) => self.first_plural,
            (Person::Second, Number::Plural) => self.second_plural,
            (Person::Third, Number::Plural) => self.third_plural,
        }
    }
}

pub type Noun = (String, Gender);
pub type Adjective = String;
pub type Verb = String;

pub const VOWELS: &[char] = &[
    'a', 'e', 'i', 'í', 'ó', 'o', 'u', 'å', 'ą', 'ę', 'ė', 'é', 'ȯ', 'ų', 'ů', 'ú', 'ý', 'y', 'ě',
    'A', 'E', 'I', 'O', 'U', 'á',
];

pub const HARD_CONSONANTS: &[char] = &[
    'p', 'b', 'f', 'v', 'm', 's', 'z', 't', 'd', 'r', 'n', 'l', 'k', 'g', 'h',
];

pub const J_MERGE_CHARS: &[&str] = &["st", "zd", "sk", "zg", "s", "z", "t", "d", "k", "g", "h"];

//VERB STUFF
impl ISV {
    pub fn get_present_tense_stem(infinitive: &str) -> (String, Conjugation) {
        let infinitive_stem = ISV::get_infinitive_stem(infinitive);
        let irregular = irregular_present_stem(infinitive);

        if irregular != "" {
            if irregular.ends_with("me") {
                return (
                    ISVUTILS::replace_last_occurence(&irregular, "me", "m"),
                    Conjugation::First,
                );
            }
            if irregular.ends_with("ne") {
                return (
                    ISVUTILS::replace_last_occurence(&irregular, "ne", "n"),
                    Conjugation::First,
                );
            }

            if irregular.ends_with("je") {
                return (
                    ISVUTILS::replace_last_occurence(&irregular, "je", "j"),
                    Conjugation::First,
                );
            }

            if irregular.ends_with("e") {
                return (
                    ISVUTILS::replace_last_occurence(&irregular, "e", ""),
                    Conjugation::First,
                );
            }
            if irregular.ends_with("i") {
                return (
                    ISVUTILS::replace_last_occurence(&irregular, "i", ""),
                    Conjugation::Second,
                );
            }
        }

        if ISVUTILS::is_consonant(&ISVUTILS::last_in_stringslice(&infinitive_stem)) {
            (infinitive_stem, Conjugation::First)
        } else if infinitive.ends_with("ovati") {
            (infinitive.replace("ovati", "uj"), Conjugation::First)
        } else if infinitive.ends_with("nųti") {
            (infinitive.replace("nųti", "n"), Conjugation::First)
        } else if infinitive.ends_with("ati") {
            (
                ISVUTILS::replace_last_occurence(infinitive, "ati", "aj"),
                Conjugation::First,
            )
        } else if infinitive.ends_with("eti") {
            (
                ISVUTILS::replace_last_occurence(infinitive, "eti", "ej"),
                Conjugation::First,
            )
        } else if infinitive.ends_with("ęti") {
            (
                ISVUTILS::replace_last_occurence(infinitive, "ęti", "n"),
                Conjugation::First,
            )
        } else if infinitive.ends_with("yti") {
            (
                ISVUTILS::replace_last_occurence(infinitive, "yti", "yj"),
                Conjugation::First,
            )
        } else if infinitive.ends_with("uti") {
            (
                ISVUTILS::replace_last_occurence(infinitive, "uti", "uj"),
                Conjugation::First,
            )
        } else if infinitive.ends_with("iti") {
            (
                ISVUTILS::replace_last_occurence(infinitive, "iti", ""),
                Conjugation::Second,
            )
        } else if infinitive.ends_with("ěti") {
            (
                ISVUTILS::replace_last_occurence(infinitive, "ěti", ""),
                Conjugation::Second,
            )
        } else {
            panic!("IMPROPER PERSENT TENSE STEM: {}", infinitive);
        }
    }
    pub fn get_infinitive_stem(word: &str) -> String {
        ISVUTILS::string_without_last_n(word, 2)
    }

    pub fn conjugate_verb(
        &self,
        word: &str,
        person: &Person,
        number: &Number,
        gender: &Gender,
        tense: &Tense,
    ) -> Verb {
        let word = word.to_lowercase();
        let (present_stem, conjugation) = ISV::get_present_tense_stem(&word);
        let infinitive_stem = ISV::get_infinitive_stem(&word);

        let endings = match conjugation {
            Conjugation::First => &FIRST_CONJUGATION,
            Conjugation::Second => &SECOND_CONJUGATION,
        };

        match tense {
            Tense::Present => {
                let ending = endings.ending(person, number);
                let merged = ISVUTILS::iotation_merge(&present_stem, ending);
                merged
            }

            _ => panic!("TENSE NOT IMPLEMENTED"),
        }
    }
    pub fn l_participle(&self, word: &str, gender: &Gender, number: &Number) -> Verb {
        if word == "idti" {
            match number {
                Number::Singular => String::from("šli"),
                Number::Plural => match gender {
                    Gender::Masculine => String::from("šėl"),
                    Gender::Feminine => String::from("šla"),
                    Gender::Neuter => String::from("šlo"),
                },
            }
        } else {
            let infinitive_stem = ISV::get_infinitive_stem(word);
            match number {
                Number::Plural => {
                    format!("{}{}", infinitive_stem, "li")
                }
                Number::Singular => match gender {
                    Gender::Masculine => {
                        format!("{}{}", infinitive_stem, "l")
                    }
                    Gender::Feminine => {
                        format!("{}{}", infinitive_stem, "la")
                    }
                    Gender::Neuter => {
                        format!("{}{}", infinitive_stem, "lo")
                    }
                },
            }
        }
    }
}

// ADJECTIVE STUFF
impl ISV {
    pub fn decline_adj(
        &self,
        word: &str,
        case: &Case,
        number: &Number,
        gender: &Gender,
        animate: bool,
    ) -> Adjective {
        let word = word.to_lowercase();
        let stem_is_soft = ISV::stem_of_adj_is_soft(&word);
        let adj_stem = ISV::get_adj_stem(&word);

        let endings = match gender {
            Gender::Masculine => {
                if animate {
                    if stem_is_soft {
                        &ADJ_ANIMATE_SOFT_ENDINGS
                    } else {
                        &ADJ_ANIMATE_HARD_ENDINGS
                    }
                } else {
                    if stem_is_soft {
                        &ADJ_INANIMATE_SOFT_ENDINGS
                    } else {
                        &ADJ_INANIMATE_HARD_ENDINGS
                    }
                }
            }
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
        let ending = endings.ending(case, number);
        let merged = format!("{}{}", adj_stem, ending);
        return merged;
    }

    pub fn stem_of_adj_is_soft(word: &str) -> bool {
        if word.ends_with("i") {
            true
        } else {
            false
        }
    }
    pub fn get_adj_stem(word: &str) -> String {
        let mut adj_stem = word.to_string();
        adj_stem.pop();
        adj_stem
    }
}

//NOUN STUFF
impl ISV {
    pub fn decline_noun(&self, word: &str, case: &Case, number: &Number) -> Noun {
        let word = word.to_lowercase();
        let gender = self.guess_gender(&word);
        let word_is_animate = self.noun_is_animate(&word);
        let word_stem_is_soft = ISV::stem_of_noun_is_soft(&word);
        let word_stem = ISV::get_noun_stem(&word, number);

        let endings = if ISV::is_ost_class(&word) {
            &OST_ENDINGS
        } else {
            match gender {
                Gender::Masculine => {
                    if word_is_animate {
                        if word_stem_is_soft {
                            &ANIMATE_SOFT_ENDINGS
                        } else {
                            &ANIMATE_HARD_ENDINGS
                        }
                    } else {
                        if word_stem_is_soft {
                            &INANIMATE_SOFT_ENDINGS
                        } else {
                            &INANIMATE_HARD_ENDINGS
                        }
                    }
                }
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

        let ending = endings.ending(case, number);
        let merged = format!("{}{}", word_stem, ending);
        return (merged, gender.clone());
    }
    pub fn noun_is_animate(&self, word: &str) -> bool {
        self.animate_nouns.contains(&word.to_string())
    }

    pub fn guess_gender(&self, word: &str) -> Gender {
        let word_string = word.to_string();
        if self.animate_nouns.contains(&word_string) || self.nonanimate_nouns.contains(&word_string)
        {
            return Gender::Masculine;
        } else if self.feminine_nouns.contains(&word_string) {
            return Gender::Feminine;
        } else if self.neuter_nouns.contains(&word_string) {
            return Gender::Neuter;
        }
        let last_one = ISV::last_n_chars(word, 1);

        if ISV::is_ost_class(word) || (last_one == "a") || (last_one == "i") {
            return Gender::Feminine;
        } else if (last_one == "o") || (last_one == "e") {
            return Gender::Neuter;
        } else {
            return Gender::Masculine;
        }
    }

    pub fn last_n_chars(word: &str, n: usize) -> String {
        let split_pos = word.char_indices().nth_back(n - 1).unwrap_or((0, 'a')).0;
        word[split_pos..].into()
    }
    pub fn is_ost_class(word: &str) -> bool {
        word.ends_with("ost́")
            || word.ends_with("ěć")
            || word.ends_with("ěč")
            || word.ends_with("eć")
            || word.ends_with("at́")
    }

    pub fn get_noun_stem(word: &str, number: &Number) -> String {
        if word.ends_with("anin") && number == &Number::Plural {
            return ISVUTILS::string_without_last_n(word, 2);
        }
        if word.ends_with("anina") && number == &Number::Plural {
            return ISVUTILS::string_without_last_n(word, 3);
        }

        if ISVUTILS::is_vowel(&ISVUTILS::last_in_stringslice(word)) {
            return ISVUTILS::string_without_last_n(word, 1);
        } else {
            return String::from(word);
        }
    }
    pub fn stem_of_noun_is_soft(word: &str) -> bool {
        ISVUTILS::ends_with_soft_consonant(&ISV::get_noun_stem(word, &Number::Singular))
    }
}

impl ISVUTILS {
    pub fn replace_last_occurence(input: &str, pattern: &str, replacement: &str) -> String {
        if let Some(last_index) = input.rfind(pattern) {
            let (before_last, _after_last) = input.split_at(last_index);
            format!("{}{}", before_last, replacement)
        } else {
            input.into()
        }
    }
    pub fn iotation_merge(root: &str, suffix: &str) -> String {
        if suffix.chars().nth(0) == Some('j') {
            for entry in J_MERGE_CHARS {
                if root.ends_with(entry) {
                    let new_root = match *entry {
                        "st" => ISVUTILS::replace_last_occurence(root, entry, "šć"),
                        "zd" => ISVUTILS::replace_last_occurence(root, entry, "ždž"),
                        "sk" => ISVUTILS::replace_last_occurence(root, entry, "šč"),
                        "zg" => ISVUTILS::replace_last_occurence(root, entry, "žž"),
                        "s" => ISVUTILS::replace_last_occurence(root, entry, "š"),
                        "z" => ISVUTILS::replace_last_occurence(root, entry, "ž"),
                        "t" => ISVUTILS::replace_last_occurence(root, entry, "ć"),
                        "d" => ISVUTILS::replace_last_occurence(root, entry, "dž"),
                        "k" => ISVUTILS::replace_last_occurence(root, entry, "č"),
                        "g" => ISVUTILS::replace_last_occurence(root, entry, "ž"),
                        "h" => ISVUTILS::replace_last_occurence(root, entry, "š"),
                        _ => root.to_string(),
                    };
                    let new_suffix = suffix.get(1..).unwrap();
                    return format!("{new_root}{new_suffix}");
                }
            }

            format!("{root}{suffix}")
        } else {
            format!("{root}{suffix}")
        }
    }

    pub fn is_vowel(c: &char) -> bool {
        VOWELS.contains(c)
    }

    pub fn ends_with_soft_consonant(word: &str) -> bool {
        ISVUTILS::is_soft_consonant(&ISVUTILS::last_in_stringslice(word))
    }

    pub fn is_hard_consonant(c: &char) -> bool {
        HARD_CONSONANTS.contains(c)
    }

    pub fn is_soft_consonant(c: &char) -> bool {
        !ISVUTILS::is_hard_consonant(c) && !ISVUTILS::is_vowel(c)
    }
    pub fn last_in_stringslice(s: &str) -> char {
        s.to_string().pop().unwrap_or(' ')
    }
    pub fn is_consonant(c: &char) -> bool {
        !ISVUTILS::is_vowel(c)
    }
    pub fn string_without_last_n(s: &str, n: i64) -> String {
        let mut stringik = s.to_string();
        for _ in 0..n {
            stringik.pop();
        }

        stringik
    }
}
