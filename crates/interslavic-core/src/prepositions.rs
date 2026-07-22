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

/// Per-case English glosses for every preposition in [`PREPOSITIONS`] —
/// the case selects the meaning, and the meaning is what a
/// government-linting or glossing consumer needs (s+Gen "off, down from"
/// is a different word than s+Ins "with").
///
/// Gloss sources, marked per entry: `steen` = the prepositions section of
/// Jan van Steenbergen's grammar (steen.free.fr/interslavic/
/// prepositions.html, "With the genitive/dative/accusative/instrumental/
/// locative" lists, incl. the stable-location-vs-motion rule for the
/// Acc/Ins and Acc/Loc pairs); `dict` = the community dictionary's
/// English translation for entries steen does not list (its government
/// annotation is where the case set came from). A unit test asserts this
/// table and [`PREPOSITIONS`] agree key-for-key and case-for-case, so the
/// two can never drift.
pub const PREPOSITION_SENSES: &[(&str, &[(Case, &str)])] = &[
    // Single-letter core prepositions and their spelling variants.
    ("v", &[(Case::Acc, "into"), (Case::Loc, "in, within")]), // steen
    ("vȯ", &[(Case::Acc, "into"), (Case::Loc, "in, within")]), // steen
    // s+Gen is absent from steen's lists; the gloss is the dictionary's
    // (+2) sense. s+Ins is steen.
    (
        "s",
        &[
            (Case::Gen, "off, down from"),                          // dict
            (Case::Ins, "with, together with; by means of, using"), // steen
        ],
    ),
    (
        "so",
        &[
            (Case::Gen, "off, down from"),
            (Case::Ins, "with, together with; by means of, using"),
        ],
    ),
    (
        "sȯ",
        &[
            (Case::Gen, "off, down from"),
            (Case::Ins, "with, together with; by means of, using"),
        ],
    ),
    ("k", &[(Case::Dat, "to, towards (direction)")]), // steen
    ("ko", &[(Case::Dat, "to, towards (direction)")]), // steen
    ("kȯ", &[(Case::Dat, "to, towards (direction)")]), // steen
    // o/ob+Acc is the "against, on (impact)" reading the case table
    // already documents as the fuller dictionary reading; +Loc is steen.
    (
        "o",
        &[
            (Case::Acc, "against, on (impact)"),         // dict
            (Case::Loc, "about; concerning, regarding"), // steen
        ],
    ),
    (
        "ob",
        &[
            (Case::Acc, "against, on (impact)"),
            (Case::Loc, "about; concerning, regarding"),
        ],
    ),
    // The rest, in the same order as PREPOSITIONS.
    ("bez", &[(Case::Gen, "without")]),            // steen
    ("blizko", &[(Case::Gen, "near, close to")]),  // steen
    ("blågodarę", &[(Case::Dat, "thanks to")]),    // steen
    ("dlja", &[(Case::Gen, "for")]),               // steen
    ("do", &[(Case::Gen, "to, towards, till")]),   // steen
    ("dovnųtra", &[(Case::Gen, "inside (goal)")]), // dict
    ("dękujųći", &[(Case::Dat, "thanks to")]),     // steen
    ("hvala", &[(Case::Dat, "thanks to")]),        // steen
    ("iz", &[(Case::Gen, "from, out of")]),        // steen
    ("iz-među", &[(Case::Gen, "from between")]),   // steen
    ("iz-nad", &[(Case::Gen, "from above")]),      // steen
    ("iz-pod", &[(Case::Gen, "from under")]),      // steen
    ("iz-prěd", &[(Case::Gen, "from before")]),    // steen
    ("iz-srěd", &[(Case::Gen, "from among")]),     // steen
    ("iz-za", &[(Case::Gen, "from behind")]),      // steen
    ("izključajųći", &[(Case::Acc, "excluding")]), // dict
    ("kolo", &[(Case::Gen, "next to, around")]),   // steen
    ("kromě", &[(Case::Gen, "except")]),           // steen
    (
        "među",
        &[
            (Case::Acc, "between (direction)"),    // steen (motion)
            (Case::Ins, "between (place & time)"), // steen (stable)
        ],
    ),
    ("mimo", &[(Case::Gen, "past, by")]), // steen
    (
        "na",
        &[(Case::Acc, "on(to), unto"), (Case::Loc, "on, at")], // steen
    ),
    (
        "nad",
        &[
            (Case::Acc, "above, over (direction)"), // steen (motion)
            (Case::Ins, "above, over, beyond"),     // steen (stable)
        ],
    ),
    ("naprotiv", &[(Case::Gen, "opposite")]), // steen
    (
        "od",
        &[(Case::Gen, "of, from (away from); since; by (agent)")],
    ), // steen
    ("odnosno", &[(Case::Gen, "concerning, about")]), // steen
    ("okolo", &[(Case::Gen, "around; about, approximately")]), // steen
    (
        "po",
        &[
            (
                Case::Dat,
                "according to, per, by, in the manner of; for … each", // steen
            ),
            (
                Case::Acc,
                "for (in pursuit of); for (the Xth time); up to", // steen
            ),
            (
                Case::Loc,
                "after, following; along(side); throughout, all over", // steen
            ),
        ],
    ),
    (
        "pod",
        &[
            (Case::Acc, "under (direction)"), // steen (motion)
            (Case::Ins, "under"),             // steen (stable)
        ],
    ),
    ("podle", &[(Case::Gen, "near, beside")]), // steen
    ("podčas", &[(Case::Gen, "during, so long as")]), // steen
    ("podȯlg", &[(Case::Gen, "according to")]), // dict
    ("polěv", &[(Case::Gen, "to the left of")]), // steen
    ("pomimo", &[(Case::Gen, "in spite of")]), // steen
    ("ponad", &[(Case::Ins, "above, beyond")]), // steen
    ("poniž", &[(Case::Gen, "below")]),        // steen
    ("poniže", &[(Case::Gen, "below")]),       // dict (steen lists poniž)
    ("poprav", &[(Case::Gen, "to the right of")]), // steen
    ("poprěk", &[(Case::Gen, "across")]),      // steen
    ("poręd", &[(Case::Gen, "next to")]),      // steen
    ("poslě", &[(Case::Gen, "after")]),        // steen
    ("posrěd", &[(Case::Gen, "amidst, in the middle of, among")]), // steen
    ("posrědstvom", &[(Case::Gen, "by means of, using")]), // steen
    ("povodom", &[(Case::Gen, "on the occasion of")]), // dict
    ("povyše", &[(Case::Gen, "above")]),       // dict (steen lists poviž)
    ("povŕh", &[(Case::Gen, "to the upper side of, atop")]), // steen
    ("pri", &[(Case::Loc, "near; during, in the presence of")]), // steen
    ("protiv", &[(Case::Dat, "against")]),     // steen
    (
        "prěd",
        &[
            (Case::Acc, "before, in front of (direction)"), // steen (motion)
            (
                Case::Ins,
                "before, in front of, ahead of (place & time); prior to, ago", // steen
            ),
        ],
    ),
    ("prěz", &[(Case::Acc, "through, across, via")]), // steen
    ("radi", &[(Case::Gen, "for the sake of, on account of")]), // dict
    ("srěd", &[(Case::Gen, "among, amid")]),          // steen
    ("sųprotiv", &[(Case::Gen, "contrary to")]),      // steen
    ("sȯglåsno", &[(Case::Dat, "according to")]),     // steen
    ("sųglåsno", &[(Case::Dat, "according to")]),     // steen
    ("u", &[(Case::Gen, "at, at the place of")]),     // steen
    ("vizavi", &[(Case::Gen, "vis-à-vis")]),          // dict
    ("vključajųći", &[(Case::Acc, "including")]),     // dict
    ("vměsto", &[(Case::Gen, "instead of")]),         // steen
    ("vně", &[(Case::Gen, "outside of")]),            // steen
    ("vnųtri", &[(Case::Gen, "inside, within")]),     // steen
    (
        "vslěd",
        &[(Case::Gen, "following, because of, as a result of")],
    ), // steen
    ("vsrěd", &[(Case::Gen, "amid, among")]),         // dict
    ("vȯdle", &[(Case::Gen, "beside, near")]),        // dict
    ("vȯzdȯlž", &[(Case::Gen, "along")]),             // steen (vdolž)
    (
        "za",
        &[
            (Case::Gen, "during, in times of"), // steen
            (
                Case::Acc,
                "for, because of, in exchange for, in favour of; behind (direction)", // steen
            ),
            (Case::Ins, "behind (place), after (time)"), // steen
        ],
    ),
    ("zaměsto", &[(Case::Gen, "instead of")]), // dict
    ("zaradi", &[(Case::Gen, "for the sake of, on account of")]), // dict
    ("črěz", &[(Case::Acc, "through, across, via")]), // steen
];

/// The per-case senses of `prep` — each governed case with its English
/// gloss — or `None` if it is not a recognized single-word preposition.
/// Backed by [`PREPOSITION_SENSES`]; agrees with [`preposition_cases`]
/// case-for-case by construction (unit-tested).
///
/// ```
/// use interslavic_core::{prepositions::preposition_senses, Case};
/// assert_eq!(
///     preposition_senses("s"),
///     Some(&[
///         (Case::Gen, "off, down from"),
///         (Case::Ins, "with, together with; by means of, using"),
///     ][..])
/// );
/// assert_eq!(preposition_senses("bez"), Some(&[(Case::Gen, "without")][..]));
/// assert_eq!(preposition_senses("žaba"), None);
/// ```
pub fn preposition_senses(prep: &str) -> Option<&'static [(Case, &'static str)]> {
    PREPOSITION_SENSES
        .iter()
        .find(|(p, _)| *p == prep)
        .map(|(_, senses)| *senses)
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
    fn senses_and_cases_tables_agree() {
        // The two tables must cover the same prepositions with the same
        // case sets in the same order — glosses only ever ADD information.
        assert_eq!(PREPOSITIONS.len(), PREPOSITION_SENSES.len());
        for (prep, cases) in PREPOSITIONS {
            let senses = preposition_senses(prep)
                .unwrap_or_else(|| panic!("{prep} missing from PREPOSITION_SENSES"));
            let sense_cases: Vec<Case> = senses.iter().map(|(c, _)| *c).collect();
            assert_eq!(&sense_cases[..], *cases, "{prep} case sets differ");
            for (case, gloss) in senses {
                assert!(!gloss.trim().is_empty(), "{prep} +{case:?} gloss empty");
            }
        }
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
