fn main() {
    initialize_dictionary("isv_words.csv");
    println!("Hello, world!");
}
/* //if you do not initialize the dictionary, animate nouns will not be inflected correctly, nor will words with irregular stems
inflector.initialize_dictionary("isv_words.csv"); */
use csv::ReaderBuilder;
use serde::Deserialize;

use std::fs::File;
use std::io::BufReader;

#[derive(Debug, Default, Deserialize)]
struct WordEntry {
    id: i64,
    isv: String,
    addition: String,
    #[serde(rename = "partOfSpeech")]
    part_of_speech: String,

    en: String,

    ru: String,
    be: String,
    uk: String,
    pl: String,
    cs: String,
    sk: String,
    sl: String,
    hr: String,
    sr: String,
    mk: String,
    bg: String,
    cu: String,
    de: String,
    nl: String,
    eo: String,
}

pub fn initialize_dictionary(isv_words: &str) {
    // Open the CSV file
    let file = File::open(isv_words).unwrap();
    let reader = BufReader::new(file);

    // Create a CSV reader
    let mut csv_reader = ReaderBuilder::new().has_headers(true).from_reader(reader);

    // Create a vector to hold words that are m.anim
    let mut m_anim_words: Vec<String> = Vec::new();
    let mut m_nonanim_words: Vec<String> = Vec::new();
    let mut f_words: Vec<String> = Vec::new();
    let mut n_words: Vec<String> = Vec::new();

    // Iterate through the records
    for result in csv_reader.deserialize() {
        let record: WordEntry = result.unwrap();

        //make sure its a noun
        if !record.part_of_speech.contains("v.")
            && !record.part_of_speech.contains("adj.")
            && !record.part_of_speech.contains("conj.")
            && !record.part_of_speech.contains("adv.")
            && !record.part_of_speech.contains("particle")
            && !record.part_of_speech.contains("prefix")
            && !record.part_of_speech.contains("prep.")
            && !record.part_of_speech.contains("pron.")
            && !record.part_of_speech.contains("num.")
        {
            // Check if the partOfSpeech is "m.anim"
            if record.part_of_speech.contains("m.anim.") {
                m_anim_words.push(record.isv.to_lowercase());
            } else if record.part_of_speech.contains("m.") {
                m_nonanim_words.push(record.isv.to_lowercase());
            } else if record.part_of_speech.contains("f.") {
                f_words.push(record.isv.to_lowercase());
            } else if record.part_of_speech.contains("n.") {
                n_words.push(record.isv.to_lowercase());
            }
            if record.addition.contains("(") {
                println!("{:#?} {:#?}", record.isv, record.addition);
            }
        }
    }

    /*
    self.animate_nouns = m_anim_words;
    self.nonanimate_nouns = m_nonanim_words;
    self.feminine_nouns = f_words;
    self.neuter_nouns = n_words;
    */
}
