use crate::{has_more_than_one_word, Noun};
use serde_derive::Deserialize;
use std::fs::File;

pub enum Gender {
    Masculine,
    Feminine,
    Neuter,
}

pub type Declineable = bool;

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

    #[serde(rename = "partOfSpeech")]
    pos: String,

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

pub fn derive_noun(word: &str, gender: Gender, animacy: Animacy) -> Noun {
    
        match gender {
            Gender::Masculine => Noun::masculine_decline(word, animacy),
            Gender::Feminine => Noun::feminine_decline(word),
            Gender::Neuter => Noun::neuter_decline(word),
        }
    
}

pub fn derive_from_pos(word: &str, markers: PartOfSpeech) {
    match markers {
        PartOfSpeech::Noun(gender, case, animacy, number) => {
            derive_noun(word, gender, animacy);
        }
        _ => panic!("pos not implemented"),
    }
}

pub fn gender_and_animacy_from_pos_string(poss: &str) -> (Option<Gender>, Animacy, Declineable) {
    let boop: Vec<&str> = poss.split('.').collect();

    let without_whitespace: Vec<String> = boop
        .iter()
        .map(|s| s.chars().filter(|c| !c.is_whitespace()).collect())
        .collect();

    let animacy = if without_whitespace.contains(&String::from("anim")) {
        Animacy::Animate
    } else {
        Animacy::Inanimate
    };
    let gender = if without_whitespace.contains(&String::from("m")) {
        Some(Gender::Masculine)
    } else if without_whitespace.contains(&String::from("f")) {
        Some(Gender::Feminine)
    } else if without_whitespace.contains(&String::from("n")) {
        Some(Gender::Neuter)
    } else {
        None
    };

    let declineable = if without_whitespace.contains(&String::from("indecl")) {
        false
    } else if without_whitespace.contains(&String::from("pl")) {
        false
    }
    //should fix this idk how it affects declineability
    else if without_whitespace.contains(&String::from("sg")) {
        false
    }
    //should fix this idk how it affects declineability
    else {
        true
    };

    (gender, animacy, declineable)
}

pub fn noun_from_csv(word : &str , poss : &str) -> Option<Noun>{

    let (gender, animacy, declineable) = gender_and_animacy_from_pos_string(poss);

    if let Some(gend) = gender {
        let mecz =
        if declineable && !has_more_than_one_word(word) {
            derive_noun(word, gend, animacy) 
            
        } else {
             Noun::indeclineable(word) 
           
        };
        println!("{:?}", &mecz.pl.gen);
        Some(mecz)
    }
    else {None}
   



}

pub fn load_word_csv() {
    let file_path = "assets/interslavic_words.csv";
    let file = File::open(file_path).unwrap();
    let mut rdr = csv::Reader::from_reader(file);

    for result in rdr.deserialize() {
        let record: WordISV = result.unwrap();
        let boop = record.isv.trim();
        let poss = &record.pos;
        //println!("RECORD ISSSS    {:?}", &record);
      
      noun_from_csv(boop, poss);
    }
}
