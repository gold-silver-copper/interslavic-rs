use std::fs::File;
use serde_derive::Deserialize;

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
    Second
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
    Noun(Gender,Case,Animacy,Number, Declension),
    Verb(Gender, Tense, Person , Number , Aspect , Mood , Conjugation ),
    Adj(Gender,Case,Animacy,Number),
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


pub fn masc_decline(word: &str ,case:Case,animacy:Animacy,number:Number,declension:Declension) {

    match animacy {

    }





}


pub fn derive_noun(word: &str ,gender:Gender ,case:Case,animacy:Animacy,number:Number,declension:Declension) {

    match gender {


        Gender::Masculine => {masc_decline(word,case,animacy,number,declension)},
        Gender::Feminine => {femn_decline(word,case,number,declension)},
        Gender::Neuter => {neut_decline(word,case,number,declension)},
    }





}

pub fn derive_every_pos(word: &str , markers: PartOfSpeech) {

    match markers {

        PartOfSpeech::Noun(gender ,case,animacy,number,declension) => {derive_noun(word,gender ,case,animacy,number,declension)},
        _ => panic!("pos not implemented")
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