pub const VOWELS: &[char] = &[
    'a', 'e', 'i', 'í', 'ó', 'o', 'u', 'å', 'ą', 'ę', 'ė', 'é', 'ȯ', 'ų', 'ů', 'ú', 'ý', 'y', 'ě',
    'A', 'E', 'I', 'O', 'U', 'á',
];

pub const HARD_CONSONANTS: &[char] = &[
    'p', 'b', 'f', 'v', 'm', 's', 'z', 't', 'd', 'r', 'n', 'l', 'k', 'g', 'h',
];

pub const J_MERGE_CHARS: &[&str] = &["st", "zd", "sk", "zg", "s", "z", "t", "d", "k", "g", "h"];

pub fn replace_last_occurrence(input: &str, pattern: &str, replacement: &str) -> String {
    if let Some(last_index) = input.rfind(pattern) {
        let (before_last, _) = input.split_at(last_index);
        format!("{}{}", before_last, replacement)
    } else {
        input.into()
    }
}

#[allow(clippy::module_name_repetitions)]
pub fn iotation_merge(root: &str, suffix: &str) -> String {
    if suffix.starts_with('j') {
        for entry in J_MERGE_CHARS {
            if root.ends_with(entry) {
                let new_root = match *entry {
                    "st" => replace_last_occurrence(root, entry, "šć"),
                    "zd" => replace_last_occurrence(root, entry, "ždž"),
                    "sk" => replace_last_occurrence(root, entry, "šč"),
                    "zg" => replace_last_occurrence(root, entry, "žž"),
                    "s" => replace_last_occurrence(root, entry, "š"),
                    "z" => replace_last_occurrence(root, entry, "ž"),
                    "t" => replace_last_occurrence(root, entry, "ć"),
                    "d" => replace_last_occurrence(root, entry, "dž"),
                    "k" => replace_last_occurrence(root, entry, "č"),
                    "g" => replace_last_occurrence(root, entry, "ž"),
                    "h" => replace_last_occurrence(root, entry, "š"),
                    _ => root.to_string(),
                };
                let new_suffix = suffix.get(1..).unwrap_or_default();
                return format!("{new_root}{new_suffix}");
            }
        }
    }

    format!("{root}{suffix}")
}

pub fn is_vowel(c: char) -> bool {
    VOWELS.contains(&c)
}

pub fn is_hard_consonant(c: char) -> bool {
    HARD_CONSONANTS.contains(&c)
}

pub fn is_soft_consonant(c: char) -> bool {
    !is_hard_consonant(c) && !is_vowel(c)
}

pub fn ends_with_soft_consonant(word: &str) -> bool {
    word.chars().last().is_some_and(is_soft_consonant)
}

pub fn is_consonant(c: char) -> bool {
    !is_vowel(c)
}

pub fn without_last_n(s: &str, n: usize) -> String {
    let keep = s.chars().count().saturating_sub(n);
    s.chars().take(keep).collect()
}

pub fn last_n_chars(word: &str, n: usize) -> String {
    let count = word.chars().count();
    word.chars().skip(count.saturating_sub(n)).collect()
}
