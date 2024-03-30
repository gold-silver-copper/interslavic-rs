pub fn slice_without_last(s: &str) -> String {
    s.char_indices()
        .last() // Get the last character's (index, char)
        .map(|(idx, _)| &s[0..idx]) // Map to a slice without the last character
        .unwrap_or(s) // If the string is empty, return it as is
        .into()
}

pub fn last_in_slice(s: &str) -> char {
    s.chars()
        .last() // Get the last character's ( char)
        .unwrap_or(' ') // If the string is empty, return it as is
        .into()
}

pub fn has_more_than_one_word(s: &str) -> bool {
    s.split_whitespace().count() > 1
}

pub fn is_vowel(c: char) -> bool {
    matches!(
        c,
        'a' | 'e'
            | 'i'
            | 'o'
            | 'u'
            | 'å'
            | 'ę'
            | 'ė'
            | 'ȯ'
            | 'ų'
            | 'y'
            | 'ě'
            | 'A'
            | 'E'
            | 'I'
            | 'O'
            | 'U'
    )
}

pub fn is_consonant(c: char) -> bool {
    !is_vowel(c)
}

pub fn is_hard_consonant(c: char) -> bool {
    matches!(
        c,
        'p' | 'b' | 'f' | 'v' | 'm' | 's' | 'z' | 't' | 'd' | 'r' | 'n' | 'l' | 'k' | 'g' | 'h'
    )
}

pub fn is_soft_consonant(c: char) -> bool {
    matches!(
        c,
        'č' | 'š'
            | 'ž'
            | 'j'
            | 'ć'
            | 'ď'
            | 'ť'
            | 'ň'
            | 'ľ'
            | '\u{301}'
            | '\u{300}'
            | '\u{302}'
            | '\u{303}'
            | '\u{304}'
            | '\u{305}'
            | '\u{306}'
            | 'đ'
            | 'ż'
            | 'ń'
            | 'c'
            | 'ŕ'
            | 'ś'
            | 'ź'
    )
}

pub fn replace_last_occurence(input: &str, pattern: &str, replacement: &str) -> String {
    if let Some(last_index) = input.rfind(pattern) {
        let (before_last, _after_last) = input.split_at(last_index);
        format!("{}{}", before_last, replacement)
    } else {
        input.into()
    }
}

pub const J_MERGE_CHARS: [&str; 6] = ["st", "zd", "s", "z", "t", "d"];

pub fn j_merge_stem_second_present(stem_in: &str) -> String {
    let stem = &slice_without_last(stem_in);

    for entry in J_MERGE_CHARS {
        if stem.ends_with(entry) {
            match entry {
                "st" => return replace_last_occurence(stem, entry, "ščų"),
                "zd" => return replace_last_occurence(stem, entry, "ždžų"),
                "s" => return replace_last_occurence(stem, entry, "šų"),
                "z" => return replace_last_occurence(stem, entry, "žų"),
                "t" => return replace_last_occurence(stem, entry, "čų"),
                "d" => return replace_last_occurence(stem, entry, "džų"),
                _ => (),
            }
        }
    }
    format!("{}{}", stem, "jų")
}
