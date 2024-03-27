pub struct Noun {
    sg: CaseForms, // Singular forms of the noun
    pl: CaseForms, // Plural forms of the noun
}
pub struct CaseForms {
    nom: String, // Nominative
    gen: String, // Genitive
    dat: String, // Dative
    acc: String, // Accusative
    ins: String, // Instrumental
    loc: String, // Locative
    voc: String, // Vocative
}

impl CaseForms {

    pub fn neuter_hard_singular(word_stem: &str) -> Self {
        CaseForms {
            nom: format!("{word_stem}o"),
            gen: format!("{word_stem}o"),
            dat: format!("{word_stem}a"),
            acc: format!("{word_stem}u"),
            ins: format!("{word_stem}om"),
            loc: format!("{word_stem}u"),
            voc: format!("{word_stem}o"),
        }
    }
    pub fn neuter_hard_plural(word_stem: &str) -> Self {
        CaseForms {
            nom: format!("{word_stem}a"),
            gen: format!("{word_stem}a"),
            dat: format!("{word_stem}"),
            acc: format!("{word_stem}am"),
            ins: format!("{word_stem}ami"),
            loc: format!("{word_stem}ah"),
            voc: format!("{word_stem}a"),
        }
    }
    
}

fn slice_without_last(s: &str) -> String {
    s.char_indices()
        .last() // Get the last character's (index, char)
        .map(|(idx, _)| &s[0..idx]) // Map to a slice without the last character
        .unwrap_or(s) // If the string is empty, return it as is
        .into()
}

impl Noun {
    pub fn neuter_hard(word: &str) -> Self {
        let word_stem = slice_without_last(word);

      

        Noun {


            sg: CaseForms::neuter_hard_singular(&word_stem),
            pl: CaseForms::neuter_hard_plural(&word_stem)
        }
    }
}
