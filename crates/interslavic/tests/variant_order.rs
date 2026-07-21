//! Variant-order compatibility fence.
//!
//! Multi-byform cells are returned as one `" / "`-joined string, and the
//! ORDER of the byforms is observable API behavior: downstream consumers
//! bless first-variant outputs into their test expectations. Reordering
//! variants is therefore a BREAKING change and must be called out
//! explicitly in the changelog (see the policy note in CHANGELOG.md).
//! This test pins the current order for a representative cell per
//! declension class; it must only ever be updated deliberately, together
//! with a changelog entry.

use interslavic::*;

#[test]
fn multi_byform_cell_order_is_stable() {
    let sg = Number::Singular;
    let pl = Number::Plural;
    for (lemma, case, number, expected) in [
        // Masculine -enj dual-stem class: short stem first.
        ("dėnj", Case::Nom, sg, "den / denj"),
        ("dėnj", Case::Gen, sg, "dne / dnja"),
        ("dėnj", Case::Nom, pl, "dni / dnje"),
        ("grebenj", Case::Gen, sg, "grebene / grebenja"),
        ("stųpenj", Case::Nom, pl, "stųpeni / stųpenje"),
        // Animate -enj.
        ("jelenj", Case::Acc, pl, "jeleni / jelenje"),
        // Feminine -ȯv class: -e genitive first.
        ("cŕkȯv", Case::Gen, sg, "cŕkve / cŕkvi"),
        ("brȯv", Case::Gen, sg, "brve / brvi"),
        // Neuter es-stem class: plain stem first.
        ("tělo", Case::Loc, sg, "tělu / tělesi"),
        ("čudo", Case::Nom, pl, "čuda / čudesa"),
        ("dělo", Case::Gen, sg, "děla / dělese"),
        // Neuter ę-stem (n-stem) class.
        ("brěmę", Case::Gen, sg, "brěmene / brěmena"),
        // The suppletive oko family: i-stem byforms first.
        ("oko", Case::Nom, pl, "oči / očesa"),
        ("oko", Case::Dat, pl, "očam / očesam"),
        ("oko", Case::Ins, pl, "očami / očesami"),
        ("oko", Case::Loc, pl, "očah / očesah"),
    ] {
        assert_eq!(
            noun(lemma, case, number),
            expected,
            "{lemma} {case:?} {number:?}"
        );
    }
}
