use crate::{has_more_than_one_word, ConjugatedNoun, Verb};
use serde_derive::Deserialize;
use std::{collections::HashMap, fs::File};

#[derive(PartialEq)]
pub enum Gender {
    Masculine,
    Feminine,
    Neuter,
    Error,
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



pub enum Number {
    Sing,
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

pub enum VerbTense {
    Infinitive,
    Imperative(Gender),
    Present(Person, Number),
    Perfect(Gender, Person, Number),
}





type ISVID = i32;

pub enum PartOfSpeech {
    ConjugatedNoun(ConjugatedNoun),
    Verb(Verb),
    Adj,
    Adv,
    Part,
}
//id,isv,addition,partOfSpeech,type,en,sameInLanguages,genesis,ru,be,uk,pl,cs,sk,bg,mk,sr,hr,sl,cu,de,nl,eo,frequency,intelligibility,using_example
#[derive(Debug, Deserialize)]
pub struct ISVEntry {
    id: ISVID,
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

impl ISVEntry {

    pub fn is_animate_from_poss(&self) -> bool {
        let boop: Vec<&str> = self.pos.split('.').collect();
    
        let without_whitespace: Vec<String> = boop
            .iter()
            .map(|s| s.chars().filter(|c| !c.is_whitespace()).collect())
            .collect();
    
        if without_whitespace.contains(&String::from("anim")) {
            true
        } else {
            false
        }
    }
    
    pub fn gender_from_poss(&self) -> Gender {
        let boop: Vec<&str> = self.pos.split('.').collect();
    
        let without_whitespace: Vec<String> = boop
            .iter()
            .map(|s| s.chars().filter(|c| !c.is_whitespace()).collect())
            .collect();
    
        if without_whitespace.contains(&String::from("m")) {
            Gender::Masculine
        } else if without_whitespace.contains(&String::from("f")) {
            Gender::Feminine
        } else if without_whitespace.contains(&String::from("n")) {
            Gender::Neuter
        } else {
            Gender::Error
        }
    }
    
    pub fn is_declineable_from_poss(&self) -> bool {
        let boop: Vec<&str> = self.pos.split('.').collect();
    
        let without_whitespace: Vec<String> = boop
            .iter()
            .map(|s| s.chars().filter(|c| !c.is_whitespace()).collect())
            .collect();
    
        if without_whitespace.contains(&String::from("indecl")) {
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
        }
    }




}

type HomographMap = HashMap<ISVID, ISVEntry>;

type ISVWordMap = HashMap<String, HomographMap>;

pub struct WordCore {
    word_map: ISVWordMap,
}

impl WordCore {
    pub fn new() -> Self {
        WordCore {
            word_map: load_word_csv(),
        }
    }
}



pub fn noun_from_csv(record: &ISVEntry) -> Option<ConjugatedNoun> {
    let gender = record.gender_from_poss();
    let declineable = record.is_declineable_from_poss();
    let animacy = record.is_animate_from_poss();

    if gender != Gender::Error {
        let mecz = if declineable && !has_more_than_one_word(&record.isv) {
            ConjugatedNoun::derive_noun(&record.isv, &gender, animacy)
        } else {
            ConjugatedNoun::indeclineable(&record.isv)
        };
        println!("{:?}", &mecz.pl.gen);
        Some(mecz)
    } else {
        None
    }
}



pub fn load_word_csv() -> ISVWordMap {
    let file_path = "assets/interslavic_words.csv";
    let file = File::open(file_path).unwrap();
    let mut rdr = csv::Reader::from_reader(file);

    let mut wordbase = ISVWordMap::new();

    for result in rdr.deserialize() {
        let mut record: ISVEntry = result.unwrap();
        record.isv = record.isv.trim().to_string();

        noun_from_csv(&record);
      
        let record_id = record.id.clone();
        let record_string = record.isv.clone();

        if wordbase.contains_key(&record_string) {
            let hmap = wordbase.get_mut(&record_string).unwrap();
            hmap.insert(record_id, record);
        } else {
            let mut hmap = HomographMap::new();
            hmap.insert(record_id, record);
            wordbase.insert(record_string, hmap);
        }

        //println!("RECORD ISSSS    {:?}", &record);
    }

    wordbase
}
