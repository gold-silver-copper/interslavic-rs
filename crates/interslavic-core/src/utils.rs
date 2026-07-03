pub const VOWELS: &[char] = &[
    'a', 'e', 'i', 'í', 'ó', 'o', 'u', 'å', 'ą', 'ę', 'ė', 'é', 'ȯ', 'ų', 'ů', 'ú', 'ý', 'y', 'ě',
    'A', 'E', 'I', 'O', 'U', 'á',
];

pub const HARD_CONSONANTS: &[char] = &[
    'p', 'b', 'f', 'v', 'm', 's', 'z', 't', 'd', 'r', 'n', 'l', 'k', 'g', 'h',
];

pub const ALL_CONSONANTS: &[char] = &[
    'b', 'c', 'ć', 'ç', 'č', 'd', 'ď', 'đ', 'ḓ', 'f', 'g', 'h', 'k', 'l', 'ĺ', 'ľ', 'ļ', 'ł', 'ŀ',
    'ǉ', 'm', 'n', 'ń', 'ň', 'ñ', 'ņ', 'ǌ', 'p', 'q', 'r', 'ŕ', 'ṙ', 'ř', 's', 'ś', 'š', 'Š', 't',
    'ť', 'ṱ', 'v', 'w', 'x', 'z', 'ź', 'ż', 'ž',
];

pub const ALL_LETTERS: &[char] = &[
    'a', 'á', 'à', 'ă', 'â', 'å', 'ą', 'ā', 'æ', 'b', 'c', 'ć', 'ç', 'č', 'd', 'ď', 'đ', 'ḓ', 'e',
    'é', 'è', 'ĕ', 'ê', 'ě', 'ë', 'ė', 'ę', 'ē', 'ǝ', 'f', 'g', 'h', 'i', 'í', 'ì', 'ĭ', 'î', 'ī',
    'ı', 'j', 'ĵ', 'k', 'l', 'ĺ', 'ľ', 'ļ', 'ł', 'ŀ', 'ǉ', 'm', 'n', 'ń', 'ň', 'ñ', 'ņ', 'ǌ', 'o',
    'ó', 'ò', 'ŏ', 'ô', 'ö', 'ȯ', 'ǫ', 'œ', 'p', 'q', 'r', 'ŕ', 'ṙ', 'ř', 's', 'ś', 'š', 'Š', 't',
    'ť', 'ṱ', 'u', 'ú', 'ù', 'ŭ', 'û', 'ů', 'ũ', 'ų', 'ū', 'v', 'w', 'x', 'y', 'ý', 'z', 'ź', 'ż',
    'ž',
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

pub fn char_len(s: &str) -> usize {
    s.chars().count()
}

pub fn char_at(s: &str, idx: usize) -> Option<char> {
    s.chars().nth(idx)
}

pub fn slice_chars(s: &str, start: usize, end: usize) -> String {
    s.chars()
        .skip(start)
        .take(end.saturating_sub(start))
        .collect()
}

pub fn replace_at_char(s: &str, idx: usize, replacement: &str) -> String {
    let mut out = String::new();
    for (i, ch) in s.chars().enumerate() {
        if i == idx {
            out.push_str(replacement);
        } else {
            out.push(ch);
        }
    }
    out
}

pub fn remove_brackets(raw: &str, open: char, close: char) -> String {
    let mut depth = 0usize;
    let mut out = String::new();
    for ch in raw.chars() {
        if ch == open {
            depth += 1;
        } else if ch == close {
            depth = depth.saturating_sub(1);
        } else if depth == 0 {
            out.push(ch);
        }
    }
    out
}

pub fn mark_fluent_vowel(word: &str, add: &str) -> String {
    let word_chars: Vec<char> = word.chars().collect();
    let add_chars: Vec<char> = add.chars().collect();
    let mut i = 0usize;
    let limit = word_chars.len().saturating_sub(1).min(add_chars.len());
    while i < limit && word_chars.get(i) == add_chars.get(i) {
        i += 1;
    }
    if word_chars.get(i) != add_chars.get(i) && word_chars.get(i + 1) == add_chars.get(i) {
        replace_fluent_vowel(word, i)
    } else {
        word.to_string()
    }
}

pub fn infer_fluent_vowel(word: &str) -> String {
    let chars: Vec<char> = word.chars().collect();
    let mut end = chars.len();
    let mut replaced = false;
    let mut result = word.to_string();
    for i in (1..chars.len()).rev() {
        let ch = chars[i];
        if !ALL_LETTERS.contains(&ch) {
            end = i;
            replaced = false;
        }
        if !replaced && is_fleeting_vowel(ch) && is_last_syllable(&chars, i, end) {
            result = replace_fluent_vowel(&result, i);
            replaced = true;
        }
    }
    result
}

fn is_fleeting_vowel(ch: char) -> bool {
    matches!(ch, 'è' | 'ė' | 'ȯ' | 'ò')
}

fn replace_fluent_vowel(word: &str, idx: usize) -> String {
    let chars: Vec<char> = word.chars().collect();
    let Some(&ch) = chars.get(idx) else {
        return word.to_string();
    };
    let base = match ch {
        'è' | 'ė' => 'e',
        'ò' | 'ȯ' => 'o',
        c => c,
    };
    let mut out = String::new();
    for (i, c) in chars.iter().enumerate() {
        if i == idx {
            out.push('(');
            out.push(base);
            out.push(')');
        } else {
            out.push(*c);
        }
    }
    out
}

fn is_last_syllable(chars: &[char], i: usize, end: usize) -> bool {
    if i == end.saturating_sub(2) {
        chars.get(i + 1).is_some_and(|c| ALL_CONSONANTS.contains(c))
    } else if i == end.saturating_sub(3) {
        chars.get(i + 1) == Some(&'n') && chars.get(i + 2) == Some(&'j')
    } else {
        false
    }
}

pub fn contains_vowel(s: &str) -> bool {
    s.chars().any(is_vowel)
}

pub fn strip_one_of_suffixes<'a>(s: &'a str, suffixes: &[&str]) -> Option<&'a str> {
    suffixes.iter().find_map(|suffix| s.strip_suffix(suffix))
}
