//! Interslavic morphophonemics: the canonical home for the first-palatalization
//! and iotation correspondence tables and the stem-softness predicate.
//!
//! These alternations are shared by every morphological operation that builds
//! a form across a suffix seam — declension, degrees of comparison, and word
//! derivation. Exposing one authoritative table set here keeps consumers from
//! each re-deriving (and diverging on) the correspondences.
//!
//! Orthography: the tables use the etymological Interslavic letters (`ć`, `đ`,
//! `šć`, `žđ`), matching the standard "flavored" alphabet.
//!
//! Note: the crate still carries a few older, narrower alternation sites that
//! predate this module — the yer-marking palatalization used by the `-es-`
//! neuter nouns (same k/g/h/c sources as [`PALATALIZATION`], but it appends a
//! soft-sign and is specific to that stem class), and the currently-unused
//! `iotation_merge`, which writes the `dž` digraph rather than `đ`. Reconciling
//! those onto this module is deliberately left as follow-up so the noun/verb
//! declension goldens are not perturbed; new functionality builds on `phono`
//! directly.

/// First palatalization at a suffix seam (live before `-ny`, `-ka`/`-ko`/`-ok`,
/// `-sky`, `-stvo`, `-ec`, `-ica`, `-ina`, `-išče`, `-nik`, and the comparative
/// `-ejši`).
pub const PALATALIZATION: &[(char, char)] = &[('k', 'č'), ('g', 'ž'), ('h', 'š'), ('c', 'č')];

/// Iotation of a stem-final consonant before a `-je-` suffix: s→š, z→ž, t→ć,
/// d→đ, st→šć, zd→žđ, k→č, g→ž, h→š, labials take bare `j` (lovjeńje),
/// sonorants soften (děljeńje). Ordered longest-match-first (`st` before `s`,
/// `zd` before `z`); the order is load-bearing.
pub const IOTATION: &[(&str, &str)] = &[
    ("st", "šć"),
    ("zd", "žđ"),
    ("s", "š"),
    ("z", "ž"),
    ("t", "ć"),
    ("d", "đ"),
    ("k", "č"),
    ("g", "ž"),
    ("h", "š"),
    ("l", "lj"),
    ("n", "nj"),
    ("r", "rj"),
    ("p", "pj"),
    ("b", "bj"),
    ("v", "vj"),
    ("m", "mj"),
];

/// Whether a stem counts as soft, driving the O⇒E ending alternation and the
/// comparative `-ejši`/`-ějši` choice. True when the last character is one of
/// `š ž č c j ć đ ń ľ ŕ` — which also covers the palatal digraphs `lj`, `nj`
/// and `dž`, whose final letter (`j`/`j`/`ž`) is already in the set.
pub fn is_soft(stem: &str) -> bool {
    matches!(
        stem.chars().last().unwrap_or(' '),
        'š' | 'ž' | 'č' | 'c' | 'j' | 'ć' | 'đ' | 'ń' | 'ľ' | 'ŕ'
    )
}

/// Apply first palatalization to a stem-final consonant (no-op if the final
/// consonant is not one of the palatalization sources).
pub fn palatalize_final(stem: &str) -> String {
    let mut s = stem.to_string();
    if let Some((_, soft)) = s
        .chars()
        .last()
        .and_then(|last| PALATALIZATION.iter().find(|(hard, _)| *hard == last))
    {
        s.pop();
        s.push(*soft);
    }
    s
}

/// Apply iotation to a stem-final consonant (longest match first); returns the
/// stem unchanged if nothing matches.
pub fn iotate_final(stem: &str) -> String {
    for (suf, rep) in IOTATION {
        if let Some(head) = stem.strip_suffix(suf) {
            return format!("{head}{rep}");
        }
    }
    stem.to_string()
}

/// All possible un-palatalized sources of a stem (always includes the stem
/// itself, since a hushing-final stem may be original rather than derived).
pub fn inverse_palatalization(stem: &str) -> Vec<String> {
    let mut v = vec![stem.to_string()];
    for (hard, soft) in PALATALIZATION {
        if let Some(head) = stem.strip_suffix(*soft) {
            v.push(format!("{head}{hard}"));
        }
    }
    v
}

/// All possible un-iotated sources of a form (always includes the form itself
/// so hushing-final stems — učiti → uč- — resolve too). Covers every hard
/// source of an ambiguous soft outcome (š ← s or h, ž ← z or g, č ← k …).
pub fn inverse_iotation(t: &str) -> Vec<String> {
    let mut v = vec![t.to_string()];
    for (hard, soft) in IOTATION {
        if let Some(head) = t.strip_suffix(soft) {
            v.push(format!("{head}{hard}"));
        }
    }
    v
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tables_round_trip() {
        // Every iotation outcome inverts back to its source.
        for (hard, soft) in IOTATION {
            let stem = format!("xx{hard}");
            let iotated = iotate_final(&stem);
            // Longest-match: a single-char source that lives inside a longer
            // match (s inside st) iotates as the longer pair; skip those.
            if iotated == format!("xx{soft}") {
                assert!(
                    inverse_iotation(&iotated).contains(&stem),
                    "{stem} → {iotated} does not invert"
                );
            }
        }
        for (hard, _) in PALATALIZATION {
            let stem = format!("xx{hard}");
            assert!(inverse_palatalization(&palatalize_final(&stem)).contains(&stem));
        }
    }

    #[test]
    fn softness_covers_all_palatal_outcomes() {
        // Any stem produced by palatalization or iotation must count as soft —
        // the invariant that keeps the comparative -ejši/-ějši choice correct.
        for (hard, _) in PALATALIZATION {
            assert!(is_soft(&palatalize_final(&format!("xx{hard}"))));
        }
        for (hard, _) in IOTATION {
            assert!(is_soft(&iotate_final(&format!("xx{hard}"))));
        }
        for s in [
            "końń", "xxń", "xxľ", "xxŕ", "xxć", "xxđ", "xxlj", "xxnj", "xxdž",
        ] {
            assert!(is_soft(s), "{s} must be soft");
        }
    }
}
