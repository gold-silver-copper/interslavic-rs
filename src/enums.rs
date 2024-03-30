use crate::{has_more_than_one_word, ConjugatedNoun, Noun, Verb};
use serde_derive::Deserialize;
use std::{collections::HashMap, fs::File};
#[derive(Debug, PartialEq, Clone)]
pub enum Gender {
    Masculine,
    Feminine,
    Neuter,
}
#[derive(Debug, PartialEq, Clone)]
pub enum Case {
    Nom,
    Acc,
    Gen,
    Dat,
    Ins,
    Loc,
    Voc,
}
#[derive(Debug, PartialEq, Clone)]
pub enum Number {
    Sing,
    Plur,
}
#[derive(Debug, PartialEq, Clone)]
pub enum Declension {
    First,
    Second,
    Third,
    Athematic,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Conjugation {
    First,
    Second,
}
#[derive(Debug, PartialEq, Clone)]
pub enum Person {
    First,
    Second,
    Third,
}
#[derive(Debug, PartialEq, Clone)]
pub enum VerbTense {
    Infinitive,
    Imperative(Person,Number),
    Present(Person, Number),
    Perfect(Gender, Person, Number),
    Imperfect(Person,Number),
    Future(Person,Number),
    PluPerfect(Gender, Person, Number),
    Conditional(Gender, Person, Number),
    PresentActiveParticiple(Gender),
    PresentPassiveParticiple(Gender),
    PastActiveParticiple(Gender),
    PastPassiveParticiple(Gender),
    LParticiple(Gender,Number),
    Noun,
}

type ISVID = i32;
#[derive(Debug, PartialEq, Clone)]
pub enum Word {
    Noun(Noun),
    Verb(Verb),
    Adj,
    Adv,
    Part,
    Error,
}
//id,isv,addition,partOfSpeech,type,en,sameInLanguages,genesis,ru,be,uk,pl,cs,sk,bg,mk,sr,hr,sl,cu,de,nl,eo,frequency,intelligibility,using_example
#[derive(Debug, Deserialize)]
pub struct ISVEntry {
    pub id: ISVID,
    pub isv: String,
    pub addition: String,

    #[serde(rename = "partOfSpeech")]
    pub pos: String,

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
    pub fn check_poss_for_string(&self, compare_str: &str) -> bool {
        let mut boop = self.pos.replace(".", " ");
        boop = boop.replace("/", " ");
        boop = boop.replace("#", " ");

        let wee: Vec<&str> = boop.split(' ').collect();

        let without_whitespace: Vec<String> = wee
            .iter()
            .map(|s| s.chars().filter(|c| !c.is_whitespace()).collect())
            .collect();

        if without_whitespace.contains(&String::from(compare_str)) {
            true
        } else {
            false
        }
    }

    pub fn get_addition_verb_stem(&self) -> Option<String> {
        let mut boop = self.addition.replace("(", " ");
        boop = boop.replace(")", " ");
        boop = boop.replace("/", " ");
        boop = boop.replace(";", " ");
        boop = boop.replace("#", " ");

        let wee: Vec<&str> = boop.split(' ').collect();

        let mut result = None;

        for meow in wee {
            if (meow.chars().nth(0) == self.isv.chars().nth(0)) {
                //    panic!("matched an addition stem :D {}", meow);

                result = Some(meow.into())
            }
        }

        result
    }

    pub fn is_animate(&self) -> bool {
        self.check_poss_for_string("anim")
    }
    pub fn is_verb(&self) -> bool {
        self.check_poss_for_string("v")
    }

    pub fn is_perfect(&self) -> bool {
        self.check_poss_for_string("pf")
    }
    pub fn is_imperfect(&self) -> bool {
        self.check_poss_for_string("ipf")
    }
    pub fn is_transitive(&self) -> bool {
        self.check_poss_for_string("tr")
    }
    pub fn is_intransitive(&self) -> bool {
        self.check_poss_for_string("intr")
    }

    pub fn get_gender(&self) -> Option<Gender> {
        if self.check_poss_for_string("m") {
            Some(Gender::Masculine)
        } else if self.check_poss_for_string("f") {
            Some(Gender::Feminine)
        } else if self.check_poss_for_string("n") {
            Some(Gender::Neuter)
        } else {
            None
        }
    }

    pub fn is_declineable(&self) -> bool {
        if self.check_poss_for_string("indecl") {
            false
        } else if self.check_poss_for_string("pl") {
            false
        }
        //should fix this idk how it affects declineability
        else if self.check_poss_for_string("sg") {
            false
        }
        //should fix this idk how it affects declineability
        else {
            true
        }
    }
}

type HomographMap = HashMap<ISVID, Word>;

type ISVWordMap = HashMap<String, HomographMap>;

pub struct WordCore {
    word_map: ISVWordMap,
}

impl WordCore {
    pub fn new() -> Self {
        WordCore {
            word_map: WordCore::load_word_csv(),
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
            record.isv = record.isv.replace("#", "");

            let record_id = record.id.clone();
            let record_string = record.isv.clone();

            if !has_more_than_one_word(&record_string) {
                let word_to_insert = if record.get_gender() != None {
                    Word::Noun(Noun::new(&record))
                } else if record.is_verb() {
                    Word::Verb(Verb::new(&record))
                } else {
                    Word::Error
                };

                if wordbase.contains_key(&record_string) {
                    let hmap = wordbase.get_mut(&record_string).unwrap();
                    hmap.insert(record_id, word_to_insert);
                } else {
                    let mut hmap = HomographMap::new();
                    hmap.insert(record_id, word_to_insert);
                    wordbase.insert(record_string, hmap);
                }
            }

            //println!("RECORD ISSSS    {:?}", &record);
        }

        wordbase
    }
}
