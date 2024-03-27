use crate::Noun;
use serde_derive::Deserialize;
use std::fs::File;

pub enum Gender {
    Masculine,
    Feminine,
    Neuter,
}

pub enum Case {
    Nom,
    Acc,
    Gen,
    Dat,
    Ins,
    Loc,
    Voc,
}

pub enum Animacy {
    Animate,
    Inanimate,
}

pub enum Number {
    Sing,
    Dual,
    Plur,
}

pub enum Declension {
    First,
    Second,
    Third,
    Athematic,
}

pub enum Conjugation {
    First,
    Second,
}

pub enum Person {
    First,
    Second,
    Third,
}

pub enum Tense {
    Present,
    Past,
    Future,
}

pub enum Aspect {
    Perfect,
    Imperfect,
    Pluperfect,
}

pub enum Mood {
    Indicative,
    Conditional,
    Imperative,
}

pub enum PartOfSpeech {
    Noun(Gender, Case, Animacy, Number),
    Verb(Gender, Tense, Person, Number, Aspect, Mood, Conjugation),
    Adj(Gender, Case, Animacy, Number),
    Adv,
    Part,
}
//id,isv,addition,partOfSpeech,type,en,sameInLanguages,genesis,ru,be,uk,pl,cs,sk,bg,mk,sr,hr,sl,cu,de,nl,eo,frequency,intelligibility,using_example
#[derive(Debug, Deserialize)]
pub struct WordISV {
    id: i32,
    isv: String,
    addition: Option<String>,
    #[serde(rename = "partOfSpeech")]
    pos: String,
    #[serde(rename = "type")]
    isv_type: String, // Renamed from "type" to "isv_type" using serde's rename attribute
    en: String,
    ru: String,
    be: String,
    uk: String,
    pl: String,
    cs: String,
    sk: String,
    bg: String,
    mk: String,
    sr: String,
    hr: String,
    sl: String,
    cu: String,
    de: String,
    nl: String,
    eo: String,
}

pub fn masc_decline(word: &str,  animacy: Animacy)-> Noun {
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

pub fn femn_decline(word: &str)-> Noun{
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

pub fn derive_noun(word: &str, gender: Gender, case: Case, animacy: Animacy, number: Number) -> Noun {
    match gender {
        Gender::Masculine => masc_decline(word, animacy),
        Gender::Feminine => femn_decline(word),
        Gender::Neuter => neut_decline(word),
    }
}

pub fn derive_from_pos(word: &str, markers: PartOfSpeech) {
    match markers {
        PartOfSpeech::Noun(gender, case, animacy, number) => {
            derive_noun(word, gender, case, animacy, number);
        }
        _ => panic!("pos not implemented"),
    }
}

pub fn load_word_csv() {
    let file_path = "assets/interslavic_words.csv";
    let file = File::open(file_path).unwrap();
    let mut rdr = csv::Reader::from_reader(file);

    for result in rdr.deserialize() {
        let record: WordISV = result.unwrap();
        println!("{:?}", record);
    }
}
