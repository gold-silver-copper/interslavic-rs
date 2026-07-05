mod case_endings;
use case_endings::*;
mod verb_endings;
use verb_endings::*;

mod irregular_verbs;

use irregular_verbs::*;
mod known_nouns;

use known_nouns::*;

#[derive(Debug, Clone, Default)]
pub struct ISVCore {}

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

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Number {
    Singular,
    Plural,
}
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Case {
    Nom,
    Acc,
    Gen,
    Loc,
    Dat,
    Ins,
    //vocative will be handle seperately
}
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Gender {
    Masculine,
    Feminine,
    Neuter,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Person {
    First,
    Second,
    Third,
}
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Conjugation {
    First,
    Second,
}
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
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

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum NounGender {
    Masculine,
    Feminine,
    Neuter,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Animacy {
    Animate,
    Inanimate,
}

pub const VOWELS: &[char] = &[
    'a', 'e', 'i', 'í', 'ó', 'o', 'u', 'å', 'ą', 'ę', 'ė', 'é', 'ȯ', 'ų', 'ů', 'ú', 'ý', 'y', 'ě',
    'A', 'E', 'I', 'O', 'U', 'á',
];

pub const HARD_CONSONANTS: &[char] = &[
    'p', 'b', 'f', 'v', 'm', 's', 'z', 't', 'd', 'r', 'n', 'l', 'k', 'g', 'h',
];

pub const J_MERGE_CHARS: &[&str] = &[
    "st", "zd", "sk", "zg", "s", "z", "t", "d", "k", "g", "h", "č", "š", "ž", "ć", "đ", "j",
];

//VERB STUFF
impl ISVCore {
    pub fn get_present_tense_stem(infinitive: &str) -> (String, Conjugation) {
        let infinitive_stem = ISVCore::get_infinitive_stem(infinitive);
        let irregular = irregular_present_stem(infinitive);

        if !irregular.is_empty() {
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
        word: &str,
        person: &Person,
        number: &Number,
        _gender: &Gender,
        tense: &Tense,
    ) -> String {
        let word = word.trim().to_string();
        let (present_stem, conjugation) = ISVCore::get_present_tense_stem(&word);

        let endings = match conjugation {
            Conjugation::First => &FIRST_CONJUGATION,
            Conjugation::Second => &SECOND_CONJUGATION,
        };

        match tense {
            Tense::Present => {
                let ending = endings.ending(person, number);
                ISVUTILS::iotation_merge(&present_stem, ending)
            }

            _ => panic!("TENSE NOT IMPLEMENTED"),
        }
    }
    pub fn l_participle(word: &str, gender: &Gender, number: &Number) -> String {
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
            let infinitive_stem = ISVCore::get_infinitive_stem(word);
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
impl ISVCore {
    pub fn decline_adj(
        word: &str,
        case: &Case,
        number: &Number,
        gender: &Gender,
        animacy: Animacy,
    ) -> String {
        let original_word = word.trim().to_string();
        let mut word = original_word.clone();
        let mut postfix = String::new();
        if word.contains(' ') {
            let words: Vec<&str> = word.split_whitespace().collect();
            if let Some(index) = words
                .iter()
                .position(|part| part.ends_with('y') || part.ends_with('i') || part.ends_with('j'))
            {
                if index < words.len() - 1 {
                    postfix = format!(" {}", words[index + 1..].join(" "));
                    word = words[..=index].join(" ");
                }
            }
        }
        if word == "seksi" {
            word = "sekśi".into();
        }
        if original_word == "seksi"
            && gender == &Gender::Masculine
            && animacy != Animacy::Animate
            && case == &Case::Acc
            && number == &Number::Singular
        {
            return "seksi".into();
        }
        let stem_is_soft = ISVCore::stem_of_adj_is_soft(&word);
        let adj_stem = ISVCore::get_adj_stem(&word);
        let has_long_form_ending =
            word.ends_with('y') || word.ends_with('i') || word.ends_with('j');

        if !has_long_form_ending
            && gender == &Gender::Masculine
            && number == &Number::Singular
            && (case == &Case::Nom || (animacy != Animacy::Animate && case == &Case::Acc))
        {
            return word;
        }

        let endings = match gender {
            Gender::Masculine => {
                if animacy == Animacy::Animate {
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
        ISVCore::append_postfix_to_alternatives(&format!("{}{}", adj_stem, ending), &postfix)
    }

    fn append_postfix_to_alternatives(form: &str, postfix: &str) -> String {
        if postfix.is_empty() {
            form.into()
        } else {
            form.split('/')
                .map(|part| format!("{}{}", part.trim(), postfix))
                .collect::<Vec<_>>()
                .join(" / ")
        }
    }

    pub fn stem_of_adj_is_soft(word: &str) -> bool {
        word.ends_with("i")
    }
    pub fn get_adj_stem(word: &str) -> String {
        if word.ends_with('y') || word.ends_with('i') || word.ends_with('j') {
            let mut adj_stem = word.to_string();
            adj_stem.pop();
            adj_stem
        } else {
            ISVCore::infer_fluent_vowel(word)
                .replace("(e)", "")
                .replace("(o)", "")
        }
    }
}

//NOUN STUFF
impl ISVCore {
    #[allow(clippy::too_many_arguments)]
    pub fn decline_noun_explicit(
        lemma: &str,
        case: &Case,
        number: &Number,
        gender: NounGender,
        animacy: Animacy,
        plural_only: bool,
        singular_only: bool,
        indeclinable: bool,
        addition: Option<&str>,
    ) -> String {
        let gender = ISVCore::api_gender_to_gender(gender);
        ISVCore::decline_noun_steen(
            lemma,
            addition.unwrap_or(""),
            &gender,
            animacy == Animacy::Animate,
            plural_only,
            singular_only,
            indeclinable,
            case,
            number,
        )
    }

    pub fn decline_noun_simple(
        lemma: &str,
        case: &Case,
        number: &Number,
        gender: NounGender,
        animacy: Animacy,
    ) -> String {
        ISVCore::decline_noun_explicit(
            lemma, case, number, gender, animacy, false, false, false, None,
        )
    }

    fn api_gender_to_gender(gender: NounGender) -> Gender {
        match gender {
            NounGender::Masculine => Gender::Masculine,
            NounGender::Feminine => Gender::Feminine,
            NounGender::Neuter => Gender::Neuter,
        }
    }

    pub fn decline_noun(word: &str, case: &Case, number: &Number) -> String {
        let word = word.trim();
        let gender = ISVCore::guess_gender(word);
        let word_is_animate = ISVCore::noun_is_animate(word);
        ISVCore::decline_noun_steen(
            word,
            "",
            &gender,
            word_is_animate,
            false,
            false,
            false,
            case,
            number,
        )
    }

    #[allow(clippy::too_many_arguments)]
    fn decline_noun_steen(
        word: &str,
        add: &str,
        origin_gender: &Gender,
        animate: bool,
        plural_only: bool,
        singular_only: bool,
        indeclinable: bool,
        case: &Case,
        number: &Number,
    ) -> String {
        let noun = ISVCore::remove_bracketed_text(word.trim(), '[', ']');
        if indeclinable {
            return noun;
        }
        if plural_only {
            return ISVCore::decline_plural_only_noun(&noun, add, origin_gender, case, number)
                .unwrap_or(noun);
        }
        if singular_only && number == &Number::Plural {
            return noun;
        }
        if let Some(form) = ISVCore::decline_substantivized_adjective(
            &noun,
            add,
            origin_gender,
            animate,
            case,
            number,
        ) {
            return form;
        }

        let noun = ISVCore::mark_or_infer_fluent_vowel(&noun, add);
        let marked_noun =
            ISVCore::mark_final_soft_noun_consonants(&noun.replace("(e)", "ė").replace("(o)", "ȯ"));
        let noun_without_fluent =
            ISVCore::mark_final_soft_noun_consonants(&noun.replace("(e)", "").replace("(o)", ""));
        let raw_gender = ISVCore::prepare_noun_gender(origin_gender, animate);
        let gender = ISVCore::establish_noun_gender(&marked_noun, &raw_gender);
        let root = ISVCore::establish_noun_root(&noun_without_fluent, &gender);
        let plural_root = ISVCore::establish_plural_noun_root(&root);
        let plural_gender =
            ISVCore::establish_plural_noun_gender(&root, &plural_root, &gender, &raw_gender);

        match number {
            Number::Singular => match case {
                Case::Nom => ISVCore::noun_nominative_sg(&marked_noun, &root, &gender),
                Case::Acc => {
                    let nominative_sg = ISVCore::noun_nominative_sg(&marked_noun, &root, &gender);
                    ISVCore::noun_accusative_sg(&nominative_sg, &root, &gender)
                }
                Case::Gen => ISVCore::noun_genitive_sg(&root, &gender),
                Case::Loc => ISVCore::noun_locative_sg(&root, &gender),
                Case::Dat => ISVCore::noun_dative_sg(&root, &gender),
                Case::Ins => ISVCore::noun_instrumental_sg(&root, &gender),
            },
            Number::Plural => match case {
                Case::Nom => ISVCore::noun_nominative_pl(&plural_root, &plural_gender),
                Case::Acc => {
                    let nom = ISVCore::noun_nominative_pl(&plural_root, &plural_gender);
                    let gen = ISVCore::noun_genitive_pl(&plural_root, &plural_gender);
                    if plural_gender == "m1" {
                        gen
                    } else {
                        nom
                    }
                }
                Case::Gen => ISVCore::noun_genitive_pl(&plural_root, &plural_gender),
                Case::Loc => ISVCore::noun_locative_pl(&plural_root, &gender),
                Case::Dat => ISVCore::noun_dative_pl(&plural_root, &gender),
                Case::Ins => ISVCore::noun_instrumental_pl(&plural_root, &gender),
            },
        }
    }

    fn remove_bracketed_text(input: &str, open: char, close: char) -> String {
        let mut result = String::new();
        let mut depth = 0usize;
        for c in input.chars() {
            if c == open {
                depth += 1;
            } else if c == close && depth > 0 {
                depth -= 1;
            } else if depth == 0 {
                result.push(c);
            }
        }
        result.trim().to_string()
    }

    fn mark_or_infer_fluent_vowel(word: &str, add: &str) -> String {
        let add = add.trim().trim_matches(['(', ')']);
        if !add.is_empty() && word != add {
            ISVCore::mark_fluent_vowel(word, add)
        } else {
            ISVCore::infer_fluent_vowel(word)
        }
    }

    fn mark_fluent_vowel(word: &str, add: &str) -> String {
        let word_chars: Vec<char> = word.chars().collect();
        let add_chars: Vec<char> = add.chars().collect();
        let limit = word_chars.len().saturating_sub(1).min(add_chars.len());
        let mut idx = 0usize;
        while idx < limit && word_chars[idx] == add_chars[idx] {
            idx += 1;
        }
        if idx + 1 < word_chars.len()
            && idx < add_chars.len()
            && word_chars[idx + 1] == add_chars[idx]
        {
            ISVCore::replace_fluent_vowel(word, idx)
        } else {
            word.to_string()
        }
    }

    fn infer_fluent_vowel(word: &str) -> String {
        let chars: Vec<char> = word.chars().collect();
        let mut result = word.to_string();
        let mut replaced = false;
        let mut end = chars.len();

        for idx in (0..chars.len()).rev() {
            let c = chars[idx];
            if !ISVCore::is_dictionary_letter(c) {
                end = idx;
                replaced = false;
            }
            if !replaced
                && matches!(c, 'è' | 'ė' | 'ȯ' | 'ò')
                && ISVCore::is_last_fluent_syllable(&chars, idx, end)
            {
                result = ISVCore::replace_fluent_vowel(&result, idx);
                replaced = true;
            }
        }
        result
    }

    fn replace_fluent_vowel(word: &str, index: usize) -> String {
        let mut result = String::new();
        for (idx, c) in word.chars().enumerate() {
            if idx == index {
                let plain = match c {
                    'ė' | 'è' => 'e',
                    'ȯ' | 'ò' => 'o',
                    _ => c,
                };
                result.push('(');
                result.push(plain);
                result.push(')');
            } else {
                result.push(c);
            }
        }
        result
    }

    fn is_last_fluent_syllable(chars: &[char], idx: usize, end: usize) -> bool {
        if idx + 2 == end {
            chars.get(idx + 1).is_some_and(ISVUTILS::is_consonant)
        } else if idx + 3 == end {
            chars.get(idx + 1) == Some(&'n') && chars.get(idx + 2) == Some(&'j')
        } else {
            false
        }
    }

    fn is_dictionary_letter(c: char) -> bool {
        c.is_alphabetic()
            || matches!(
                c,
                'å' | 'ę'
                    | 'ė'
                    | 'ȯ'
                    | 'ų'
                    | 'ě'
                    | 'ć'
                    | 'č'
                    | 'ď'
                    | 'đ'
                    | 'ĺ'
                    | 'ľ'
                    | 'ń'
                    | 'ŕ'
                    | 'ś'
                    | 'š'
                    | 'ť'
                    | 'ź'
                    | 'ž'
            )
    }

    fn decline_plural_only_noun(
        word: &str,
        add: &str,
        gender: &Gender,
        case: &Case,
        number: &Number,
    ) -> Option<String> {
        if number == &Number::Singular {
            return None;
        }

        let word_without_last = ISVUTILS::string_without_last_n(word, 1);
        let add = add.trim().trim_matches(['(', ')']);
        if add.ends_with("yh") || add.ends_with("ih") {
            let i_or_y = if add.ends_with("yh") { "y" } else { "i" };
            return Some(match case {
                Case::Nom | Case::Acc => word.to_string(),
                Case::Gen | Case::Loc => format!("{}{}h", word_without_last, i_or_y),
                Case::Dat => format!("{}{}m", word_without_last, i_or_y),
                Case::Ins => format!("{}{}mi", word_without_last, i_or_y),
            });
        }
        if !add.is_empty() {
            return None;
        }

        match gender {
            Gender::Masculine if word.ends_with(['i', 'y', 'e']) => Some(match case {
                Case::Nom | Case::Acc => word.to_string(),
                Case::Gen => format!("{}ov", word_without_last),
                Case::Loc => format!("{}ah", word_without_last),
                Case::Dat => format!("{}am", word_without_last),
                Case::Ins => format!("{}ami", word_without_last),
            }),
            Gender::Feminine if word.ends_with(['y', 'e']) => Some(match case {
                Case::Nom | Case::Acc => word.to_string(),
                Case::Gen => ISVCore::noun_rules(&ISVCore::plural_gen_ending(
                    &(word_without_last.clone() + "%"),
                    true,
                )),
                Case::Loc => format!("{}ah", word_without_last),
                Case::Dat => format!("{}am", word_without_last),
                Case::Ins => format!("{}ami", word_without_last),
            }),
            Gender::Neuter if word.ends_with('a') => Some(match case {
                Case::Nom | Case::Acc => word.to_string(),
                Case::Gen => ISVCore::noun_rules(&ISVCore::plural_gen_ending(
                    &(word_without_last.clone() + "%"),
                    true,
                )),
                Case::Loc => format!("{}ah", word_without_last),
                Case::Dat => format!("{}am", word_without_last),
                Case::Ins => format!("{}ami", word_without_last),
            }),
            Gender::Feminine if word.ends_with('i') => Some(match case {
                Case::Nom | Case::Acc => word.to_string(),
                Case::Gen => format!("{}ij", word_without_last),
                Case::Loc => format!("{}ah", word_without_last),
                Case::Dat => format!("{}am", word_without_last),
                Case::Ins => format!("{}ami", word_without_last),
            }),
            _ => None,
        }
    }

    fn decline_substantivized_adjective(
        word: &str,
        add: &str,
        gender: &Gender,
        animate: bool,
        case: &Case,
        number: &Number,
    ) -> Option<String> {
        let add = add.trim().trim_matches(['(', ')']);
        let stem = ISVUTILS::string_without_last_n(word, 1);
        let suffix = if ["-ogo", "-ego", "-oj", "-ej"].contains(&add) {
            Some(add.to_string())
        } else if !add.is_empty() && add.starts_with(&stem) {
            Some(add.replacen(&stem, "-", 1))
        } else {
            None
        }?;

        if !["-ogo", "-ego", "-oj", "-ej"].contains(&suffix.as_str()) {
            return None;
        }

        let adjective = match gender {
            Gender::Masculine | Gender::Neuter => stem + if suffix == "-ogo" { "y" } else { "i" },
            Gender::Feminine => stem + if suffix == "-oj" { "y" } else { "i" },
        };

        Some(ISVCore::decline_adj(
            &adjective,
            case,
            number,
            gender,
            if animate {
                Animacy::Animate
            } else {
                Animacy::Inanimate
            },
        ))
    }

    fn prepare_noun_gender(gender: &Gender, animate: bool) -> String {
        match gender {
            Gender::Feminine => "f".into(),
            Gender::Neuter => "n".into(),
            Gender::Masculine => {
                if animate {
                    "m1".into()
                } else {
                    "m2".into()
                }
            }
        }
    }

    fn mark_final_soft_noun_consonants(word: &str) -> String {
        let mut normalized = word.to_string();
        for (from, to) in [("ń", "nj"), ("ň", "nj"), ("ľ", "lj"), ("ĺ", "lj")] {
            if normalized.ends_with(from) {
                normalized = ISVUTILS::replace_last_occurence(&normalized, from, to);
            }
        }

        let chars: Vec<char> = normalized.chars().collect();
        let mark_from = chars.len().saturating_sub(2);
        let mut result = String::new();
        for (idx, c) in chars.iter().enumerate() {
            result.push(*c);
            if idx >= mark_from && "cšžčćńľŕťďśźđj".contains(*c) {
                result.push('ь');
            }
        }
        result
    }

    fn establish_noun_gender(noun: &str, raw_gender: &str) -> String {
        let last_char = ISVUTILS::last_in_stringslice(noun);
        let before_last_char = ISVCore::nth_char_from_end(noun, 2).unwrap_or(' ');
        let last_two = ISVCore::last_n_chars(noun, 2);

        if noun == "den" || noun == "dėn" || noun == "denjь" || noun == "dėnjь" {
            return "m3".into();
        }
        if raw_gender.starts_with('m')
            && (last_two == "en" || noun.ends_with("enjь"))
            && (noun.starts_with("kamen")
                || noun.starts_with("jelen")
                || noun.starts_with("jęčmen")
                || noun.starts_with("ječmen")
                || noun.starts_with("koren")
                || noun.starts_with("kremen")
                || noun.starts_with("plåmen")
                || noun.starts_with("plamen")
                || noun.starts_with("pŕsten")
                || noun.starts_with("prsten")
                || noun.starts_with("strumen")
                || noun.starts_with("greben")
                || noun.starts_with("stępen")
                || noun.starts_with("stepen")
                || noun.starts_with("stųpen")
                || noun.starts_with("stupen")
                || noun.starts_with("šršen")
                || noun.starts_with("šŕšen")
                || noun.starts_with("sršen")
                || noun.starts_with("sŕšen")
                || noun.starts_with("šeršen"))
        {
            return "m3".into();
        }
        if raw_gender.starts_with('n')
            && [
                "čudo", "dělo", "divo", "drěvo", "igo", "kolo", "licьe", "nebo", "ojьe", "oko",
                "slovo", "tělo", "uho",
            ]
            .contains(&noun)
        {
            return "n3".into();
        }
        if raw_gender == "f" && last_char == 'v' {
            return "f3".into();
        }
        if noun == "mati" || noun == "dočьi" || noun == "doćьi" || noun == "doči" {
            return "f3".into();
        }
        if last_char == 'a' || last_char == 'i' {
            return "f1".into();
        }
        if last_char == 'ę' {
            return "n2".into();
        }
        if before_last_char != 'ь' && last_char == 'e' {
            return "n2".into();
        }
        if last_char == 'o' || last_char == 'e' {
            return "n1".into();
        }
        if raw_gender == "m1" {
            return "m1".into();
        }
        if raw_gender == "f" {
            return "f2".into();
        }
        "m2".into()
    }

    fn establish_noun_root(noun: &str, gender: &str) -> String {
        let has_vowel_ending = matches!(ISVUTILS::last_in_stringslice(noun), 'a' | 'e' | 'ę' | 'o');
        let mut result = if noun == "lėv" || noun == "lev" {
            "ljv".into()
        } else if noun == "Lėv" || noun == "Lev" {
            "Ljv".into()
        } else if gender.starts_with('m')
            && (noun.ends_with("ecь") || noun.ends_with("ėcь") || noun.ends_with("ècь"))
            && ISVCore::masculine_ec_keeps_c(noun)
        {
            ISVUTILS::string_without_last_n(noun, 3) + "cь"
        } else if gender == "m3" {
            noun.trim_end_matches("jь").to_string()
        } else if noun == "mati" || noun == "dočьi" || noun == "doćьi" || noun == "doči" {
            ISVUTILS::string_without_last_n(noun, 1) + "er"
        } else if gender == "f3" && noun.ends_with("ov") {
            ISVUTILS::string_without_last_n(noun, 2) + "v"
        } else if gender == "f3" {
            noun.into()
        } else if gender == "n2" && ISVCore::nth_char_from_end(noun, 2) == Some('m') {
            ISVUTILS::string_without_last_n(noun, 1) + "en"
        } else if gender == "n2" {
            ISVUTILS::string_without_last_n(noun, 1) + "ęt"
        } else if gender == "f1" && (noun == "pani" || noun.ends_with("yni")) {
            ISVUTILS::string_without_last_n(noun, 1) + "jь"
        } else if noun.ends_with('i') {
            ISVUTILS::string_without_last_n(noun, 1) + "ь"
        } else if has_vowel_ending {
            ISVUTILS::string_without_last_n(noun, 1)
        } else {
            noun.into()
        };

        if !has_vowel_ending {
            if let Some(index) = ISVCore::last_fluent_vowel_index(noun) {
                let result_len = result.chars().count();
                if index > result_len.saturating_sub(3) {
                    result = ISVCore::remove_char_at(&result, index);
                }
            }
        }
        result
    }

    fn masculine_ec_keeps_c(noun: &str) -> bool {
        let chars: Vec<char> = noun.chars().collect();
        if chars.len() < 5 {
            return false;
        }
        let before = chars[chars.len() - 5];
        let near = chars[chars.len() - 4];
        "aeiouyęųåėěȯrŕ".contains(before) || "jdtc".contains(near)
    }

    fn establish_plural_noun_root(root: &str) -> String {
        match root {
            "dětęt" | "detet" | "dětet" | "detęt" => "dětь".into(),
            "človek" | "člověk" => "ljudь".into(),
            "ok" => "očь".into(),
            "uh" => "ušь".into(),
            _ if root.ends_with("anin") => ISVUTILS::string_without_last_n(root, 2),
            _ => root.into(),
        }
    }

    fn establish_plural_noun_gender(
        root: &str,
        plural_root: &str,
        gender: &str,
        raw_gender: &str,
    ) -> String {
        if root != plural_root && !plural_root.contains('n') {
            return "f2".into();
        }
        if gender == "f1" && raw_gender == "m1" {
            return "m1".into();
        }
        gender.into()
    }

    fn noun_nominative_sg(noun: &str, root: &str, gender: &str) -> String {
        let result = if gender == "f2" && (noun.contains('ȯ') || noun.contains('ė')) {
            noun.into()
        } else if gender == "f2" {
            root.into()
        } else if gender == "f3" {
            noun.into()
        } else if gender == "m3" && root == "dn" {
            "den / denj".into()
        } else if gender == "m3" {
            format!("{} / {}j", root, root)
        } else {
            noun.into()
        };
        ISVCore::noun_rules(&result)
    }

    fn noun_accusative_sg(noun: &str, root: &str, gender: &str) -> String {
        let result = if gender == "m1" {
            format!("{}a", root)
        } else if gender == "f1" {
            format!("{}ų", root)
        } else {
            noun.into()
        };
        ISVCore::noun_rules(&result)
    }

    fn noun_genitive_sg(root: &str, gender: &str) -> String {
        let result = match gender {
            "m1" | "m2" | "n1" => format!("{}a", root),
            "f1" => format!("{}y", root),
            "f2" => format!("{}i", root),
            "f3" => format!("{}e / {}i", root, root),
            "m3" => format!("{}e / {}ja", root, root),
            "n2" => format!("{}e / {}a", root, root),
            "n3" => format!("{}a / {}ese", root, ISVCore::palatalization_ending(root)),
            _ => root.into(),
        };
        ISVCore::noun_rules(&result)
    }

    fn noun_dative_sg(root: &str, gender: &str) -> String {
        let result = match gender {
            "m1" | "m2" | "n1" => format!("{}u", root),
            "f1" => format!("{}ě", root),
            "f2" | "f3" => format!("{}i", root),
            "m3" => format!("{}i / {}ju", root, root),
            "n2" => format!("{}i / {}u", root, root),
            "n3" => format!("{}u / {}esi", root, ISVCore::palatalization_ending(root)),
            _ => root.into(),
        };
        ISVCore::noun_rules(&result)
    }

    fn noun_locative_sg(root: &str, gender: &str) -> String {
        ISVCore::noun_dative_sg(root, gender)
    }

    fn noun_instrumental_sg(root: &str, gender: &str) -> String {
        let result = match gender {
            "m1" | "m2" | "n1" => format!("{}om", root),
            "f1" => format!("{}ojų", root),
            "f3" if ISVCore::is_feminine_v_stem_with_restored_o(root) => {
                format!("{}ȯvjų", ISVUTILS::string_without_last_n(root, 1))
            }
            "f2" | "f3" => format!("{}jų", root),
            "m3" => format!("{}em / {}jem", root, root),
            "n2" => format!("{}em / {}om", root, root),
            "n3" => format!("{}om / {}esem", root, ISVCore::palatalization_ending(root)),
            _ => root.into(),
        };
        ISVCore::noun_rules(&result)
    }

    fn is_feminine_v_stem_with_restored_o(root: &str) -> bool {
        let chars: Vec<char> = root.chars().collect();
        if chars.len() < 2 || chars.last() != Some(&'v') {
            return false;
        }
        let before_v = chars[chars.len() - 2];
        let vowels = "aåeęěėioȯuųy";
        !vowels.contains(before_v)
            || (chars.len() >= 3
                && before_v == 'l'
                && chars[chars.len() - 3] != 'o'
                && chars[chars.len() - 3] != 'ȯ')
    }

    fn noun_nominative_pl(root: &str, gender: &str) -> String {
        let result = if gender == "n3" {
            format!("{}a / {}esa", root, ISVCore::palatalization_ending(root))
        } else if root == "očь" || root == "ušь" {
            format!("{}i / {}esa", root, root)
        } else if gender.starts_with('n') {
            format!("{}a", root)
        } else if gender == "m1" {
            format!("{}i", root)
        } else if gender == "f1" || gender == "m2" {
            format!("{}y", root)
        } else if gender == "m3" {
            format!("{}i / {}je", root, root)
        } else {
            format!("{}i", root)
        };
        ISVCore::noun_rules(&result)
    }

    fn noun_genitive_pl(root: &str, gender: &str) -> String {
        let (result, use_ej_for_j_percent) = if gender == "f1" || root == "morjь" || root == "poljь"
        {
            (root.replacen('ь', "%", 1) + "%", true)
        } else if root == "st" {
            ("sȯt".into(), true)
        } else if gender.starts_with('n') {
            let mut result = root.replacen('ь', "%", 1) + "%";
            if gender == "n3" {
                result = format!("{} / {}es", result, ISVCore::palatalization_ending(root));
            }
            (result, false)
        } else if gender == "m3" {
            (format!("{}ev / {}jev", root, root), true)
        } else if gender.starts_with('m') {
            (format!("{}ov", root), true)
        } else if root == "očь" || root == "ušь" {
            (format!("{}ij / {}es", root, root), true)
        } else {
            (format!("{}ij", root), true)
        };
        ISVCore::noun_rules(&ISVCore::plural_gen_ending(&result, use_ej_for_j_percent))
    }

    fn noun_dative_pl(root: &str, gender: &str) -> String {
        let result = if gender == "m3" {
            format!("{}am / {}jam", root, root)
        } else if gender == "n3" {
            format!("{}am / {}esam", root, ISVCore::palatalization_ending(root))
        } else {
            format!("{}am", root)
        };
        ISVCore::noun_rules(&result)
    }

    fn noun_instrumental_pl(root: &str, gender: &str) -> String {
        let result = if gender == "m3" {
            format!("{}ami / {}jami", root, root)
        } else if gender == "n3" {
            format!(
                "{}ami / {}esami",
                root,
                ISVCore::palatalization_ending(root)
            )
        } else {
            format!("{}ami", root)
        };
        ISVCore::noun_rules(&result)
    }

    fn noun_locative_pl(root: &str, gender: &str) -> String {
        let result = if gender == "m3" {
            format!("{}ah / {}jah", root, root)
        } else if gender == "n3" {
            format!("{}ah / {}esah", root, ISVCore::palatalization_ending(root))
        } else {
            format!("{}ah", root)
        };
        ISVCore::noun_rules(&result)
    }

    fn noun_rules(word: &str) -> String {
        word.replace("ьo", "ьe")
            .replace("ьy", "ьe")
            .replace("ьě", "i")
            .replace('#', "")
            .replace("tь", "ť")
            .replace("dь", "ď")
            .replace("sь", "ś")
            .replace("zь", "ź")
            .replace('ь', "")
            .replace("ťi", "ti")
            .replace("ďi", "di")
            .replace("śi", "si")
            .replace("źi", "zi")
            .replace("ľi", "li")
            .replace("ńi", "ni")
            .replace("ŕi", "ri")
            .replace("jy", "ji")
            .replace("cy", "ci")
            .replace("ljj", "ľj")
            .replace("njj", "ńj")
    }

    fn plural_gen_ending(word: &str, use_ej_for_j_percent: bool) -> String {
        let mut word = word
            .replace("jsk%", "jsk")
            .replace("mš%", "meš")
            .replace("zl%", "zȯl")
            .replace("tl%", "tȯl")
            .replace("mgl%", "mgȯl")
            .replace("ńj%", "nij");

        if use_ej_for_j_percent {
            word = ISVCore::replace_j_percent(&word, "pbvfmlnr", "ej");
        }
        word = ISVCore::replace_j_percent(&word, "pbvfmlnrszńľŕťďśźščžđ", "ij");
        word = ISVCore::replace_final_pair_percent(&word, "jśźďťľŕńčšžćđc", 'k', "e");
        word = ISVCore::replace_final_pair_percent(&word, "pbfvmlnrtdszkgh", 'k', "ȯ");
        word = ISVCore::replace_final_pair_percent(&word, "vmpzšžt", 'n', "e");
        word = ISVCore::replace_first_second_percent(&word, 'k', "nl", "ȯ");
        word = ISVCore::replace_first_second_percent(&word, 's', "nl", "e");

        if word.starts_with("dn%") {
            word = word.replacen("dn%", "dȯn", 1);
        }
        if word.contains("pismo%") {
            word = word.replace("pismo%", "pisem");
        }
        if word.starts_with("ťm%") {
            word = word.replacen("ťm%", "tem", 1);
        }
        if word.starts_with("sto%") {
            word = word.replacen("sto%", "sȯt", 1);
        }
        word.replace('%', "")
    }

    fn replace_j_percent(word: &str, preceding: &str, replacement: &str) -> String {
        let chars: Vec<char> = word.chars().collect();
        for idx in 2..chars.len() {
            if chars[idx - 1] == 'j' && chars[idx] == '%' && preceding.contains(chars[idx - 2]) {
                let mut result: String = chars[..idx - 1].iter().collect();
                result.push_str(replacement);
                result.extend(chars[idx + 1..].iter());
                return result;
            }
        }
        word.into()
    }

    fn replace_final_pair_percent(
        word: &str,
        first_set: &str,
        second: char,
        insert: &str,
    ) -> String {
        let chars: Vec<char> = word.chars().collect();
        for idx in 2..chars.len() {
            if chars[idx] == '%' && chars[idx - 1] == second && first_set.contains(chars[idx - 2]) {
                let mut result: String = chars[..idx - 1].iter().collect();
                result.push_str(insert);
                result.push(second);
                result.extend(chars[idx + 1..].iter());
                return result;
            }
        }
        word.into()
    }

    fn replace_first_second_percent(
        word: &str,
        first: char,
        second_set: &str,
        insert: &str,
    ) -> String {
        let chars: Vec<char> = word.chars().collect();
        for idx in 2..chars.len() {
            if chars[idx] == '%' && second_set.contains(chars[idx - 1]) && chars[idx - 2] == first {
                let mut result: String = chars[..idx - 1].iter().collect();
                result.push_str(insert);
                result.push(chars[idx - 1]);
                result.extend(chars[idx + 1..].iter());
                return result;
            }
        }
        word.into()
    }

    fn palatalization_ending(root: &str) -> String {
        if root.ends_with('g') {
            ISVUTILS::string_without_last_n(root, 1) + "žь"
        } else if root.ends_with('h') {
            ISVUTILS::string_without_last_n(root, 1) + "šь"
        } else if root.ends_with('k') {
            ISVUTILS::string_without_last_n(root, 1) + "čь"
        } else if root.ends_with("cь") {
            ISVUTILS::string_without_last_n(root, 2) + "čь"
        } else {
            root.into()
        }
    }

    fn nth_char_from_end(s: &str, n: usize) -> Option<char> {
        s.chars().rev().nth(n - 1)
    }

    fn last_fluent_vowel_index(s: &str) -> Option<usize> {
        s.chars()
            .enumerate()
            .filter(|(_, c)| *c == 'ė' || *c == 'ȯ')
            .map(|(idx, _)| idx)
            .last()
    }

    fn remove_char_at(s: &str, index: usize) -> String {
        s.chars()
            .enumerate()
            .filter_map(|(idx, c)| if idx == index { None } else { Some(c) })
            .collect()
    }
    pub fn noun_is_animate(word: &str) -> bool {
        ANIMATE_MASCULINE_NOUNS.contains(&word)
    }

    pub fn guess_gender(word: &str) -> Gender {
        if ANIMATE_MASCULINE_NOUNS.contains(&word) || INANIMATE_MASCULINE_NOUNS.contains(&word) {
            Gender::Masculine
        } else if FEMININE_NOUNS.contains(&word) {
            Gender::Feminine
        } else if NEUTER_NOUNS.contains(&word) {
            Gender::Neuter
        } else {
            let last_one = ISVCore::last_n_chars(word, 1);

            if ISVCore::is_ost_class(word) || (last_one == "a") || (last_one == "i") {
                Gender::Feminine
            } else if (last_one == "o") || (last_one == "e") {
                Gender::Neuter
            } else {
                Gender::Masculine
            }
        }
    }

    pub fn last_n_chars(word: &str, n: usize) -> String {
        let split_pos = word.char_indices().nth_back(n - 1).unwrap_or((0, 'a')).0;
        word[split_pos..].into()
    }
    pub fn is_ost_class(word: &str) -> bool {
        word.ends_with("ost")
            || word.ends_with("ost́")
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
            ISVUTILS::string_without_last_n(word, 1)
        } else {
            String::from(word)
        }
    }
    pub fn stem_of_noun_is_soft(word: &str) -> bool {
        ISVUTILS::ends_with_soft_consonant(&ISVCore::get_noun_stem(word, &Number::Singular))
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
                        "č" | "š" | "ž" | "ć" | "đ" | "j" => root.to_string(),
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
