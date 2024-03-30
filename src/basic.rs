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
    !is_hard_consonant(c.clone()) && !is_vowel(c.clone())
}

pub fn ends_with_soft_consonant(word: &str) -> bool {
    is_soft_consonant(last_in_slice(word))
}

pub fn replace_last_occurence(input: &str, pattern: &str, replacement: &str) -> String {
    if let Some(last_index) = input.rfind(pattern) {
        let (before_last, _after_last) = input.split_at(last_index);
        format!("{}{}", before_last, replacement)
    } else {
        input.into()
    }
}

pub const J_MERGE_CHARS: [&str; 11] = ["st", "zd", "sk", "zg", "s", "z", "t", "d", "k", "g", "h"];

pub fn iotation_merge(root: &str, suffix: &str) -> String {
    if suffix.chars().nth(0) == Some('j') {
        for entry in J_MERGE_CHARS {
            if root.ends_with(entry) {
                let new_root = match entry {
                    "st" => replace_last_occurence(root, entry, "šć"),
                    "zd" => replace_last_occurence(root, entry, "ždž"),
                    "sk" => replace_last_occurence(root, entry, "šč"),
                    "zg" => replace_last_occurence(root, entry, "žž"),
                    "s" => replace_last_occurence(root, entry, "š"),
                    "z" => replace_last_occurence(root, entry, "ž"),
                    "t" => replace_last_occurence(root, entry, "ć"),
                    "d" => replace_last_occurence(root, entry, "dž"),
                    "k" => replace_last_occurence(root, entry, "č"),
                    "g" => replace_last_occurence(root, entry, "ž"),
                    "h" => replace_last_occurence(root, entry, "š"),
                    _ => root.to_string(),
                };
                let new_suffix = suffix.get(1..).unwrap();
                return format!("{new_root}{new_suffix}");
            }
        }

        format!("{root}{suffix}")
    } else {
        format!("{root}{suffix}")
    }
}

pub fn j_merge_stem_second_present(stem_in: &str) -> String {
    let stem = &slice_without_last(stem_in);

    iotation_merge(stem, "jų")
}

pub fn j_merge_stem_past_passive(stem_in: &str) -> String {
    let stem = &slice_without_last(stem_in);

    iotation_merge(stem, "jeny")
}

pub fn last_n_chars(word: &str, n: usize) -> String {
    if n <= word.len() {
        word[word.len() - n..].to_string()
    } else {
        word.to_string()
    }
}
