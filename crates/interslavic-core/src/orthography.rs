//! Interslavic orthography folding: the flavored (etymological) alphabet → the
//! standard alphabet.
//!
//! Interslavic has two written standards. The flavored/scientific alphabet
//! preserves etymological distinctions — jat `ě`, the nasals `ę`/`ų`, the
//! liquid-diphthong vowel `å`, the yer reflexes `ȯ`/`ė`, the soft consonants
//! `ĺ ń ŕ ť ď ś ź`, and `ć`/`đ`. The standard alphabet folds those away, so two
//! spellings that differ only in etymological flavor collapse to one string.
//! That makes the fold the natural key for matching user input (typed in either
//! alphabet) against generated or dictionary forms.
//!
//! # Stability
//!
//! This fold is provided on a **best-effort** basis and may change between
//! releases as edge cases are refined — the crate does **not** promise the
//! mapping is frozen across versions. That is fine for folding at the point of
//! use (matching, search, display). If instead you *persist* its output — a
//! cache key, a shard or route derived from it, a URL — pin an exact crate
//! version and keep your own frozen-value test, so a later refinement cannot
//! silently repartition your data.

/// The flavored → standard character correspondences (lowercase). Exposed so a
/// downstream tool can regenerate a matching fold in another language — e.g. a
/// client-side JavaScript mirror — from one authoritative table instead of
/// transcribing it by hand. `đ` is the only many-to-one entry (it expands to
/// the digraph `dž`).
///
/// [`to_standard`] is the single source for the lowercase fold; a test asserts
/// it reproduces every pair here exactly.
pub const FOLD_PAIRS: &[(char, &str)] = &[
    ('ě', "e"),
    ('ę', "e"),
    ('ų', "u"),
    ('å', "a"),
    ('ȯ', "o"),
    ('ė', "e"),
    ('ĺ', "l"),
    ('ľ', "l"),
    ('ń', "n"),
    ('ŕ', "r"),
    ('ť', "t"),
    ('ď', "d"),
    ('ś', "s"),
    ('ź', "z"),
    ('ć', "č"),
    ('đ', "dž"),
];

/// Fold a word from the flavored/scientific alphabet down to the standard
/// alphabet. Characters that are not flavored — plain ASCII and the standard
/// letters `č`, `š`, `ž` — pass through unchanged, and case is preserved.
///
/// ```
/// use interslavic_core::orthography::to_standard;
/// assert_eq!(to_standard("dělajųt"), "delajut");
/// assert_eq!(to_standard("pomoćnȯgo"), "pomočnogo");
/// assert_eq!(to_standard("međa"), "medža"); // đ expands to the digraph dž
/// assert_eq!(to_standard("čas"), "čas"); // already standard, unchanged
/// ```
pub fn to_standard(word: &str) -> String {
    let mut out = String::with_capacity(word.len());
    for ch in word.chars() {
        match ch {
            'ě' | 'ę' | 'ė' => out.push('e'),
            'ų' => out.push('u'),
            'å' => out.push('a'),
            'ȯ' => out.push('o'),
            'ĺ' | 'ľ' => out.push('l'),
            'ń' => out.push('n'),
            'ŕ' => out.push('r'),
            'ť' => out.push('t'),
            'ď' => out.push('d'),
            'ś' => out.push('s'),
            'ź' => out.push('z'),
            'ć' => out.push('č'),
            'đ' => out.push_str("dž"),
            // Uppercase flavored letters fold to their standard uppercase.
            'Ě' | 'Ę' | 'Ė' => out.push('E'),
            'Ų' => out.push('U'),
            'Å' => out.push('A'),
            'Ȯ' => out.push('O'),
            'Ĺ' | 'Ľ' => out.push('L'),
            'Ń' => out.push('N'),
            'Ŕ' => out.push('R'),
            'Ť' => out.push('T'),
            'Ď' => out.push('D'),
            'Ś' => out.push('S'),
            'Ź' => out.push('Z'),
            'Ć' => out.push('Č'),
            'Đ' => out.push_str("Dž"),
            other => out.push(other),
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn folds_flavored_to_standard() {
        assert_eq!(to_standard("dělajųt"), "delajut");
        assert_eq!(to_standard("pomoćnȯgo"), "pomočnogo");
        assert_eq!(to_standard("veśměśnȯ"), "vesmesno");
        assert_eq!(to_standard("međa"), "medža");
        // Standard letters and ASCII pass through; case is preserved.
        assert_eq!(to_standard("čas"), "čas");
        assert_eq!(to_standard("Ěľo"), "Elo");
    }

    #[test]
    fn fold_pairs_agree_with_to_standard() {
        // FOLD_PAIRS is the source consumers mirror; to_standard must reproduce
        // every pair exactly, or a JS/other-language mirror would drift.
        for (from, std) in FOLD_PAIRS {
            assert_eq!(&to_standard(&from.to_string()), std, "pair {from} drifted");
        }
    }
}
