pub struct Noun {
    // Singular forms
    nom_sg: String, // Nominative Singular
    gen_sg: String, // Genitive Singular
    dat_sg: String, // Dative Singular
    acc_sg: String, // Accusative Singular
    ins_sg: String, // Instrumental Singular
    loc_sg: String, // Locative Singular
    voc_sg: String, // Vocative Singular

    // Plural forms
    nom_pl: String, // Nominative Plural
    gen_pl: String, // Genitive Plural
    dat_pl: String, // Dative Plural
    acc_pl: String, // Accusative Plural
    ins_pl: String, // Instrumental Plural
    loc_pl: String, // Locative Plural
    voc_pl: String, // Vocative Plural
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
            nom_sg: format!("{word_stem}o"),
            acc_sg: format!("{word_stem}o"),
            gen_sg: format!("{word_stem}a"),
            dat_sg: format!("{word_stem}u"),
            ins_sg: format!("{word_stem}om"),
            loc_sg: format!("{word_stem}u"),
            voc_sg: format!("{word_stem}o"),
            nom_pl: format!("{word_stem}a"),
            acc_pl: format!("{word_stem}a"),
            gen_pl: format!("{word_stem}"),
            dat_pl: format!("{word_stem}am"),
            ins_pl: format!("{word_stem}ami"),
            loc_pl: format!("{word_stem}ah"),
            voc_pl: format!("{word_stem}a"),
        }
    }
}
