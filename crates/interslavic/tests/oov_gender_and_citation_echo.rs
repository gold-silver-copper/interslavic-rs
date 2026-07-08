//! Regression tests for gold-silver-copper/interslavic-rs#5:
//! out-of-lexicon gender guessing and citation-form echo.

use interslavic::{Case, Number, ISV};

/// An out-of-lexicon abstract noun in -osť must decline as a feminine i-stem
/// (like kosť), not be guessed masculine (točnosťa/točnosťem was the bug).
#[test]
fn oov_ost_nouns_are_feminine_i_stems() {
    // A nonsense -osť word guaranteed to be outside the dictionary.
    for w in ["zzzukavosť", "točnosť"] {
        let gen = ISV::noun(w, Case::Gen, Number::Singular);
        assert!(
            gen.ends_with("osti"),
            "{w}: gen.sg must be the i-stem -osti, got {gen}"
        );
        let nom = ISV::noun(w, Case::Nom, Number::Singular);
        assert_eq!(nom, w, "{w}: nom.sg must echo the citation form");
        let nom_pl = ISV::noun(w, Case::Nom, Number::Plural);
        assert!(
            nom_pl.ends_with("osti"),
            "{w}: nom.pl must be the i-stem -osti, got {nom_pl}"
        );
    }
}

/// The 4-char lexical words stay with their dictionary genders: kosť is
/// feminine, gosť is MASCULINE — the length guard keeps the heuristic away.
#[test]
fn short_ost_words_keep_dictionary_gender() {
    assert!(
        ISV::noun("kosť", Case::Gen, Number::Singular).contains("kosti"),
        "kosť stays feminine"
    );
    let gost_gen = ISV::noun("gosť", Case::Gen, Number::Singular);
    assert!(
        !gost_gen.contains("gosti"),
        "gosť must NOT flip to a feminine i-stem: got {gost_gen}"
    );
}

/// Loan lemmas in soft consonant + -o must echo their citation form in the
/// nominative (adadžo → adadže was the bug); native soft neuters in -e are
/// untouched.
#[test]
fn soft_o_loans_echo_citation_form() {
    for w in ["adadžo", "bandžo"] {
        assert_eq!(ISV::noun(w, Case::Nom, Number::Singular), w);
        // Accusative of an inanimate neuter echoes the nominative.
        assert_eq!(ISV::noun(w, Case::Acc, Number::Singular), w);
    }
    // Native soft neuter unchanged by the fix.
    assert_eq!(ISV::noun("morje", Case::Nom, Number::Singular), "morje");
    assert_eq!(ISV::noun("polje", Case::Nom, Number::Singular), "polje");
}
