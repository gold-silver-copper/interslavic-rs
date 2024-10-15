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
    let russian_text = "== Бытие ==

    === 1 ===

    1 В начале сотвори Бог небо и землю.
    2 Земля же бе невидима и неустроена, и тма верху бездны, и Дух Божий ношашеся верху воды.
    3 И рече Бог: да будет свет. И бысть свет.
    4 И виде Бог свет, яко добро, и разлучи Бог между светом и между тмою.
    5 И нарече Бог свет день, а тму нарече нощь. И бысть вечер, и бысть утро, день един.
    6 И рече Бог: да будет твердь посреде воды, и да будет разлучающи посреде воды и воды. И бысть тако.
    7 И сотвори Бог твердь, и разлучи Бог между водою, яже бе под твердию, и между водою, яже бе над твердию.
    8 И нарече Бог твердь небо. И виде Бог, яко добро. И бысть вечер, и бысть утро, день вторый.
    9 И рече Бог: да соберется вода, яже под небесем, в собрание едино, и да явится суша. И бысть тако. И собрася вода, яже под небесем, в собрания своя, и явися суша.
    10 И нарече Бог сушу землю, и собрания вод нарече моря. И виде Бог, яко добро.
    11 И рече Бог: да прорастит земля былие травное, сеющее семя по роду и по подобию, и древо плодовитое творящее плод, емуже семя его в нем, по роду на земли. И бысть тако.
    12 И изнесе земля былие травное, сеющее семя по роду и по подобию, и древо плодовитое творящее плод, емуже семя его в нем, по роду на земли. И виде Бог, яко добро.
    13 И бысть вечер, и бысть утро, день третий.
    14 И рече Бог: да будут светила на тверди небесней, освещати землю и разлучати между днем и между нощию: и да будут в знамения и во времена, и во дни и в лета,
    15 и да будут в просвещение на тверди небесней, яко светити по земли. И бысть тако.
    16 И сотвори Бог два светила великая: светило великое в начала дне, и светило меншее в начала нощи, и звезды:
    17 и положи я Бог на тверди небесней, яко светити на землю,
    18 и владети днем и нощию, и разлучати между светом и между тмою. И виде Бог, яко добро.
    19 И бысть вечер, и бысть утро, день четвертый.
    20 И рече Бог: да изведут воды гады душ живых, и птицы летающыя по земли, по тверди небесней. И бысть тако.
    21 И сотвори Бог киты великия, и всяку душу животных гадов, яже изведоша воды по родом их, и всяку птицу пернату по роду. И виде Бог, яко добра.
    22 И благослови я Бог, глаголя: раститеся и множитеся, и наполните воды, яже в морях, и птицы да умножатся на земли.
    23 И бысть вечер, и бысть утро, день пятый.
    24 И рече Бог: да изведет земля душу живу по роду, четвероногая и гады, и звери земли по роду. И бысть тако.
    25 И сотвори Бог звери земли по роду, и скоты по роду их, и вся гады земли по роду их. И виде Бог, яко добра.
    26 И рече Бог: сотворим человека по образу Нашему и по подобию, и да обладает рыбами морскими, и птицами небесными, (и зверми) и скотами, и всею землею, и всеми гады пресмыкающимися по земли.
    27 И сотвори Бог человека, по образу Божию сотвори его: мужа и жену сотвори их.
    28 И благослови их Бог, глаголя: раститеся и множитеся, и наполните землю, и господствуйте ею, и обладайте рыбами морскими, (и зверми) и птицами небесными, и всеми скотами, и всею землею, и всеми гадами пресмыкающимися по земли.
    29 И рече Бог: се, дах вам всякую траву семенную сеющую семя, еже есть верху земли всея: и всякое древо, еже имать в себе плод семене семеннаго, вам будет в снедь:
    30 и всем зверем земным, и всем птицам небесным, и всякому гаду пресмыкающемуся по земли, иже имать в себе душу живота, и всяку траву зелену в снедь. И бысть тако.
    31 И виде Бог вся, елика сотвори: и се добра зело. И бысть вечер, и бысть утро, день шестый.";
    let interslavic_text = transliterate_russian_to_interslavic(russian_text);
    println!("Interslavic: {}", &interslavic_text);
    let mut file = File::create("interslavic_output.txt").expect("Could not create file");

    writeln!(file, "Interslavic: {}", interslavic_text).expect("Could not write to file");
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
