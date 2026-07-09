//! Preposition government: which grammatical case(s) each Interslavic
//! preposition takes.
//!
//! This is closed-class *lexical* data (curated from the community dictionary's
//! `(+N)` government annotations), not a rule derived from morphology — so it
//! is provided here as a plain table for the tools that need it (grammar
//! checking, sentence generation, learning aids). A preposition may govern more
//! than one case, with the case selecting the meaning (e.g. `na` + accusative
//! "onto" vs `na` + locative "on").
//!
//! Keys are the flavored (etymological) citation forms, matching the rest of
//! the crate. The table covers single-word prepositions; multi-word
//! prepositional phrases (`bez obzira na`, …) and the bare comparative
//! particles (`ako`, `kak`) are omitted. It may be extended in future releases.

use crate::Case;

/// The single-word Interslavic prepositions and the case(s) each governs.
/// Exposed so consumers can inspect or iterate the whole table (e.g. to build
/// a folded-key index); [`preposition_cases`] is the point lookup.
pub const PREPOSITIONS: &[(&str, &[Case])] = &[
    // Single-letter core prepositions and their spelling variants. `o`/`ob`
    // govern the accusative ("against") as well as the locative ("about"); the
    // dictionary annotates only the locative, so this is the fuller reading.
    ("v", &[Case::Acc, Case::Loc]),
    ("vȯ", &[Case::Acc, Case::Loc]),
    ("s", &[Case::Gen, Case::Ins]),
    ("so", &[Case::Gen, Case::Ins]),
    ("sȯ", &[Case::Gen, Case::Ins]),
    ("k", &[Case::Dat]),
    ("ko", &[Case::Dat]),
    ("kȯ", &[Case::Dat]),
    ("o", &[Case::Acc, Case::Loc]),
    ("ob", &[Case::Acc, Case::Loc]),
    // The rest, aggregated from the dictionary's (+N) annotations.
    ("bez", &[Case::Gen]),
    ("blizko", &[Case::Gen]),
    ("blågodarę", &[Case::Dat]),
    ("dlja", &[Case::Gen]),
    ("do", &[Case::Gen]),
    ("dovnųtra", &[Case::Gen]),
    ("dękujųći", &[Case::Dat]),
    ("hvala", &[Case::Dat]),
    ("iz", &[Case::Gen]),
    ("iz-među", &[Case::Gen]),
    ("iz-nad", &[Case::Gen]),
    ("iz-pod", &[Case::Gen]),
    ("iz-prěd", &[Case::Gen]),
    ("iz-srěd", &[Case::Gen]),
    ("iz-za", &[Case::Gen]),
    ("izključajųći", &[Case::Acc]),
    ("kolo", &[Case::Gen]),
    ("kromě", &[Case::Gen]),
    ("među", &[Case::Acc, Case::Ins]),
    ("mimo", &[Case::Gen]),
    ("na", &[Case::Acc, Case::Loc]),
    ("nad", &[Case::Acc, Case::Ins]),
    ("naprotiv", &[Case::Gen]),
    ("od", &[Case::Gen]),
    ("odnosno", &[Case::Gen]),
    ("okolo", &[Case::Gen]),
    ("po", &[Case::Dat, Case::Acc, Case::Loc]),
    ("pod", &[Case::Acc, Case::Ins]),
    ("podle", &[Case::Gen]),
    ("podčas", &[Case::Gen]),
    ("podȯlg", &[Case::Gen]),
    ("polěv", &[Case::Gen]),
    ("pomimo", &[Case::Gen]),
    ("ponad", &[Case::Ins]),
    ("poniž", &[Case::Gen]),
    ("poniže", &[Case::Gen]),
    ("poprav", &[Case::Gen]),
    ("poprěk", &[Case::Gen]),
    ("poręd", &[Case::Gen]),
    ("poslě", &[Case::Gen]),
    ("posrěd", &[Case::Gen]),
    ("posrědstvom", &[Case::Gen]),
    ("povodom", &[Case::Gen]),
    ("povyše", &[Case::Gen]),
    ("povŕh", &[Case::Gen]),
    ("pri", &[Case::Loc]),
    ("protiv", &[Case::Dat]),
    ("prěd", &[Case::Acc, Case::Ins]),
    ("prěz", &[Case::Acc]),
    ("radi", &[Case::Gen]),
    ("srěd", &[Case::Gen]),
    ("sųprotiv", &[Case::Gen]),
    ("sȯglåsno", &[Case::Dat]),
    ("sųglåsno", &[Case::Dat]),
    ("u", &[Case::Gen]),
    ("vizavi", &[Case::Gen]),
    ("vključajųći", &[Case::Acc]),
    ("vměsto", &[Case::Gen]),
    ("vně", &[Case::Gen]),
    ("vnųtri", &[Case::Gen]),
    ("vslěd", &[Case::Gen]),
    ("vsrěd", &[Case::Gen]),
    ("vȯdle", &[Case::Gen]),
    ("vȯzdȯlž", &[Case::Gen]),
    ("za", &[Case::Gen, Case::Acc, Case::Ins]),
    ("zaměsto", &[Case::Gen]),
    ("zaradi", &[Case::Gen]),
    ("črěz", &[Case::Acc]),
];

/// The case(s) `prep` governs, or `None` if it is not a recognized
/// single-word preposition. `prep` is the flavored citation form.
///
/// ```
/// use interslavic_core::{prepositions::preposition_cases, Case};
/// assert_eq!(preposition_cases("bez"), Some(&[Case::Gen][..]));
/// assert_eq!(preposition_cases("na"), Some(&[Case::Acc, Case::Loc][..]));
/// assert_eq!(preposition_cases("pri"), Some(&[Case::Loc][..])); // instrumental is (+5), never (+7)
/// assert_eq!(preposition_cases("žaba"), None);
/// ```
pub fn preposition_cases(prep: &str) -> Option<&'static [Case]> {
    PREPOSITIONS
        .iter()
        .find(|(p, _)| *p == prep)
        .map(|(_, cases)| *cases)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn governs_expected_cases() {
        assert_eq!(preposition_cases("bez"), Some(&[Case::Gen][..]));
        assert_eq!(preposition_cases("k"), Some(&[Case::Dat][..]));
        assert_eq!(preposition_cases("s"), Some(&[Case::Gen, Case::Ins][..]));
        assert_eq!(preposition_cases("na"), Some(&[Case::Acc, Case::Loc][..]));
        // The classic pitfall: instrumental prepositions must include Ins, and
        // never a phantom seventh case.
        assert_eq!(preposition_cases("pod"), Some(&[Case::Acc, Case::Ins][..]));
        assert_eq!(preposition_cases("nad"), Some(&[Case::Acc, Case::Ins][..]));
        assert_eq!(
            preposition_cases("za"),
            Some(&[Case::Gen, Case::Acc, Case::Ins][..])
        );
        // Non-prepositions and unlisted words.
        assert_eq!(preposition_cases("žaba"), None);
        assert_eq!(preposition_cases(""), None);
    }

    #[test]
    fn table_is_well_formed() {
        for (prep, cases) in PREPOSITIONS {
            assert!(!prep.is_empty(), "empty preposition key");
            assert!(!cases.is_empty(), "{prep} governs no case");
            // No case listed twice for one preposition.
            for (i, c) in cases.iter().enumerate() {
                assert!(!cases[i + 1..].contains(c), "{prep} lists {c:?} twice");
            }
        }
        // Every key is unique.
        for (i, (prep, _)) in PREPOSITIONS.iter().enumerate() {
            assert!(
                !PREPOSITIONS[i + 1..].iter().any(|(q, _)| q == prep),
                "duplicate key {prep}"
            );
        }
    }
}
