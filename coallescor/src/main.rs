use csv::ReaderBuilder;
use serde::Deserialize;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fs::File;

use std::io::BufReader;
use std::io::Write;
fn main() {
    initialize_dictionary("isv_words.csv");
    println!("Hello, world!");
    //  let interslavic_text = transliterate_russian_to_interslavic(russian_text);
    // println!("Interslavic: {}", &interslavic_text);
    //let mut file = File::create("interslavic_output.txt").expect("Could not create file");

    //writeln!(file, "Interslavic: {}", interslavic_text).expect("Could not write to file");
}
/* //if you do not initialize the dictionary, animate nouns will not be inflected correctly, nor will words with irregular stems
inflector.initialize_dictionary("isv_words.csv"); */

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
    let mut m_anim_words: HashSet<String> = HashSet::new();
    let mut m_nonanim_words: HashSet<String> = HashSet::new();
    let mut f_words: HashSet<String> = HashSet::new();
    let mut n_words: HashSet<String> = HashSet::new();

    let mut irregular_verbs: HashSet<String> = HashSet::new();

    // Iterate through the records
    for result in csv_reader.deserialize() {
        let record: WordEntry = result.unwrap();

        if record.part_of_speech.contains("v.") && !record.part_of_speech.contains("adv.") {
            let boop = irregular_verbs.insert(record.isv.to_lowercase());
            if record.addition.contains("(") && boop {
                println!("{:#?} {:#?}", record.isv, record.addition);
            }
        }

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
                m_anim_words.insert(record.isv.to_lowercase());
            } else if record.part_of_speech.contains("m.") {
                m_nonanim_words.insert(record.isv.to_lowercase());
            } else if record.part_of_speech.contains("f.") {
                f_words.insert(record.isv.to_lowercase());
            } else if record.part_of_speech.contains("n.") {
                n_words.insert(record.isv.to_lowercase());
            }
            if record.addition.contains("(") {
                //      println!("{:#?} {:#?}", record.isv, record.addition);
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

/// Transliterates a string from Russian Cyrillic to Interslavic.
fn transliterate_russian_to_interslavic(input: &str) -> String {
    let mut cyrillic_to_interslavic = HashMap::new();

    // Mapping of Russian Cyrillic to Interslavic strings
    cyrillic_to_interslavic.insert('А', "A");
    cyrillic_to_interslavic.insert('Б', "B");
    cyrillic_to_interslavic.insert('В', "V");
    cyrillic_to_interslavic.insert('Г', "G");
    cyrillic_to_interslavic.insert('Д', "D");
    cyrillic_to_interslavic.insert('Е', "Je");
    cyrillic_to_interslavic.insert('Ё', "Jo");
    cyrillic_to_interslavic.insert('Ж', "Ž");
    cyrillic_to_interslavic.insert('З', "Z");
    cyrillic_to_interslavic.insert('И', "I");
    cyrillic_to_interslavic.insert('Й', "J");
    cyrillic_to_interslavic.insert('К', "K");
    cyrillic_to_interslavic.insert('Л', "L");
    cyrillic_to_interslavic.insert('М', "M");
    cyrillic_to_interslavic.insert('Н', "N");
    cyrillic_to_interslavic.insert('О', "O");
    cyrillic_to_interslavic.insert('П', "P");
    cyrillic_to_interslavic.insert('Р', "R");
    cyrillic_to_interslavic.insert('С', "S");
    cyrillic_to_interslavic.insert('Т', "T");
    cyrillic_to_interslavic.insert('У', "U");
    cyrillic_to_interslavic.insert('Ф', "F");
    cyrillic_to_interslavic.insert('Х', "H");
    cyrillic_to_interslavic.insert('Ц', "C");
    cyrillic_to_interslavic.insert('Ч', "Č");
    cyrillic_to_interslavic.insert('Ш', "Š");
    cyrillic_to_interslavic.insert('Щ', "Šč");
    cyrillic_to_interslavic.insert('Ъ', "");
    cyrillic_to_interslavic.insert('Ы', "Y");
    cyrillic_to_interslavic.insert('Ь', "");
    cyrillic_to_interslavic.insert(' ', " ");
    cyrillic_to_interslavic.insert('Э', "E");
    cyrillic_to_interslavic.insert('Ю', "Ju");
    cyrillic_to_interslavic.insert('Я', "Ja");
    cyrillic_to_interslavic.insert('=', "=");
    cyrillic_to_interslavic.insert('0', "0");
    cyrillic_to_interslavic.insert('1', "1");
    cyrillic_to_interslavic.insert('2', "2");
    cyrillic_to_interslavic.insert('3', "3");
    cyrillic_to_interslavic.insert('4', "4");
    cyrillic_to_interslavic.insert('5', "5");
    cyrillic_to_interslavic.insert('6', "6");
    cyrillic_to_interslavic.insert('7', "7");
    cyrillic_to_interslavic.insert('8', "8");
    cyrillic_to_interslavic.insert('9', "9");
    cyrillic_to_interslavic.insert('\n', "\n");

    cyrillic_to_interslavic.insert('а', "a");
    cyrillic_to_interslavic.insert('б', "b");
    cyrillic_to_interslavic.insert('в', "v");
    cyrillic_to_interslavic.insert('г', "g");
    cyrillic_to_interslavic.insert('д', "d");
    cyrillic_to_interslavic.insert('е', "e");
    cyrillic_to_interslavic.insert('ё', "e");
    cyrillic_to_interslavic.insert('ж', "ž");
    cyrillic_to_interslavic.insert('з', "z");
    cyrillic_to_interslavic.insert('и', "i");
    cyrillic_to_interslavic.insert('й', "j");
    cyrillic_to_interslavic.insert('к', "k");
    cyrillic_to_interslavic.insert('л', "l");
    cyrillic_to_interslavic.insert('м', "m");
    cyrillic_to_interslavic.insert('н', "n");
    cyrillic_to_interslavic.insert('о', "o");
    cyrillic_to_interslavic.insert('п', "p");
    cyrillic_to_interslavic.insert('р', "r");
    cyrillic_to_interslavic.insert('с', "s");
    cyrillic_to_interslavic.insert('т', "t");
    cyrillic_to_interslavic.insert('у', "u");
    cyrillic_to_interslavic.insert('ф', "f");
    cyrillic_to_interslavic.insert('х', "h");
    cyrillic_to_interslavic.insert('ц', "c");
    cyrillic_to_interslavic.insert('ч', "č");
    cyrillic_to_interslavic.insert('ш', "š");
    cyrillic_to_interslavic.insert('щ', "šč");
    cyrillic_to_interslavic.insert('ъ', "");
    cyrillic_to_interslavic.insert('ы', "y");
    cyrillic_to_interslavic.insert('ь', "j");
    cyrillic_to_interslavic.insert('э', "e");
    cyrillic_to_interslavic.insert('ю', "ju");
    cyrillic_to_interslavic.insert('я', "ja");

    // Transliterating the input
    input
        .chars()
        .map(|c| cyrillic_to_interslavic.get(&c).unwrap_or(&"").to_string())
        .collect::<String>()
}
