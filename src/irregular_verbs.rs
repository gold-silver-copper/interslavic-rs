pub const PREFIXES: &[&str] = &[
    "", "v", "u", "s", "o", "za", "vȯ", "vy", "vo", "sȯ", "po", "od", "ob", "ne", "na", "iz", "do",
    "vȯz", "voz", "råz", "raz", "prě", "pro", "pri", "pre", "pod", "nad", "odpo", "izpo", "råzpro",
    "razpro", "prědpo",
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
