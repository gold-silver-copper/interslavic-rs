pub const PREFIXES: &[&str] = &[
    "prědpo", "razpro", "råzpro", "izpo", "odpo", "nad", "pod", "pre", "pri", "pro", "prě", "raz",
    "råz", "voz", "vȯz", "do", "iz", "na", "ne", "ob", "od", "po", "sȯ", "vo", "vy", "vȯ", "za",
    "o", "s", "u", "v", "",
];

pub const IRREGULAR_STEMS: &[(&str, &str)] = &[
    ("briti", "brije"),
    ("biti", "bije"),
    ("brati", "bere"),
    ("idti", "ide"),
    ("oděti", "oděne "),
    ("jesti", "je "),
];

pub fn irregular_present_stem(infinitive: &str) -> String {
    for (inf, third) in IRREGULAR_STEMS {
        for prefix in PREFIXES {
            let combined = format!("{}{}", prefix, inf);
            if combined == infinitive {
                return format!("{}{}", prefix, third);
            }
        }
    }
    "".into()
}
