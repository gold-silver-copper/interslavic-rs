//! Stateless string and character helpers shared by the morphology modules.

use crate::{HARD_CONSONANTS, J_MERGE_CHARS, VOWELS};

pub fn replace_last_occurence(input: &str, pattern: &str, replacement: &str) -> String {
    if let Some(last_index) = input.rfind(pattern) {
        let (before_last, _after_last) = input.split_at(last_index);
        format!("{}{}", before_last, replacement)
    } else {
        input.into()
    }
}
pub fn iotation_merge(root: &str, suffix: &str) -> String {
    if suffix.starts_with('j') {
        for entry in J_MERGE_CHARS {
            if root.ends_with(entry) {
                let new_root = match *entry {
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
                    "č" | "š" | "ž" | "ć" | "đ" | "j" => root.to_string(),
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

pub fn is_vowel(c: &char) -> bool {
    VOWELS.contains(c)
}

pub fn ends_with_soft_consonant(word: &str) -> bool {
    last_char(word).is_some_and(|c| is_soft_consonant(&c))
}

pub fn is_hard_consonant(c: &char) -> bool {
    HARD_CONSONANTS.contains(c)
}

pub fn is_soft_consonant(c: &char) -> bool {
    !is_hard_consonant(c) && !is_vowel(c)
}
/// Return the final Unicode scalar value, or `None` for empty input.
pub fn last_char(s: &str) -> Option<char> {
    s.chars().last()
}

/// Return the final `n` Unicode scalar values without panicking.
pub fn last_n_chars(s: &str, n: usize) -> String {
    if n == 0 {
        return String::new();
    }
    let character_count = s.chars().count();
    if n >= character_count {
        return s.to_string();
    }
    s.chars().skip(character_count - n).collect()
}
pub fn is_consonant(c: &char) -> bool {
    !is_vowel(c)
}
pub fn string_without_last_n(s: &str, n: i64) -> String {
    let mut stringik = s.to_string();
    for _ in 0..n {
        stringik.pop();
    }

    stringik
}
