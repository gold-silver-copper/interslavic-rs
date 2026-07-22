//! Golden sentences: steen's own examples (syntax and pronoun pages),
//! re-spelled to the flavored orthography this workspace uses and pinned
//! against actual facade output. Where steen's legacy plain orthography
//! and the facade differ (knigu → knigų, se → sę, otec → otėc), the
//! facade wins — the parity-standard rule — and the divergence is noted
//! inline. These strings are the linearization contract; `cargo xtask
//! phrase-check` additionally runs every golden through slovowiki's
//! independent agreement checker.

use interslavic::{Case, Gender, Number, Person};
use interslavic_phrase::*;

fn sentence(tree: &Clause) -> String {
    realize(tree, RealizeOpts::sentence()).unwrap()
}

#[test]
fn steen_declaratives() {
    // steen: "Otec kupil knigu." (flavored: otėc, knigų)
    let tree = clause(np("otėc"), vp("kupiti").object(np("kniga"))).past();
    assert_eq!(sentence(&tree), "Otėc kupil knigų.");

    // steen (pronouns page): "Ja myju se" — flavored myjų, sę; the
    // reflexive comes from the lemma spelling "myti sę".
    let tree = clause(
        pron(Person::First, Number::Singular, Gender::Masculine),
        vp("myti sę"),
    );
    assert_eq!(sentence(&tree), "Ja myjų sę.");
}

#[test]
fn steen_questions() {
    let declarative = || clause(np("otėc"), vp("kupiti").object(np("kniga"))).past();
    assert_eq!(
        sentence(&declarative().force(Force::IntonationQuestion)),
        "Otėc kupil knigų?"
    );
    assert_eq!(
        sentence(&declarative().force(Force::CiQuestion)),
        "Či otėc kupil knigų?"
    );
    assert_eq!(
        sentence(&declarative().force(Force::LiQuestion)),
        "Kupil li otėc knigų?"
    );
}

#[test]
fn steen_stable_vs_motion_pp() {
    // steen: "Kot spi pod stolom" / "Kot poběgl pod stol" — the Acc/Ins
    // pair that motivates explicit sense selection on `pod`.
    let stable = clause(
        np("kot"),
        vp("spati").pp(pp("pod", np("stol")).case(Case::Ins)),
    );
    assert_eq!(sentence(&stable), "Kot spi pod stolom.");

    let motion = clause(
        np("kot"),
        vp("poběgti").pp(pp("pod", np("stol")).case(Case::Acc)),
    )
    .past();
    assert_eq!(sentence(&motion), "Kot poběgl pod stol.");
}

#[test]
fn flagship_quantified_agreement() {
    let tree = clause(
        np("krålj").det("toj").adj("dobry"),
        vp("ukrasti").object(np("moneta").count(5).adj("zlåty")),
    )
    .past();
    assert_eq!(sentence(&tree), "Toj dobry krålj ukradl 5 zlåtyh monet.");
}

#[test]
fn negation_feminine_past_and_pronoun_pp() {
    // Negation before the finite complex.
    let tree = clause(np("otėc"), vp("kupiti").object(np("kniga")))
        .past()
        .negated();
    assert_eq!(sentence(&tree), "Otėc ne kupil knigų.");

    // Feminine subject gender flows into perfect_parts.
    let tree = clause(np("žena"), vp("kupiti").object(np("kniga"))).past();
    assert_eq!(sentence(&tree), "Žena kupila knigų.");

    // A pronoun inside a PP takes the prepositional n- form automatically.
    let tree = clause(
        np("otėc"),
        vp("kupiti").object(np("kniga")).pp(pp(
            "za",
            pron(Person::Third, Number::Singular, Gender::Masculine),
        )
        .case(Case::Acc)),
    )
    .past();
    assert_eq!(sentence(&tree), "Otėc kupil knigų za njego.");
}

#[test]
fn diagnostics_are_values() {
    // Ambiguous preposition without a case: the error lists the senses.
    let tree = clause(np("kot"), vp("spati").pp(pp("pod", np("stol"))));
    match realize(&tree, RealizeOpts::sentence()) {
        Err(PhraseError::AmbiguousPreposition {
            preposition,
            senses,
        }) => {
            assert_eq!(preposition, "pod");
            assert_eq!(senses.len(), 2);
        }
        other => panic!("expected AmbiguousPreposition, got {other:?}"),
    }

    // Object under a dictionary-intransitive verb.
    let tree = clause(np("otėc"), vp("spati").object(np("kniga")));
    assert!(matches!(
        realize(&tree, RealizeOpts::sentence()),
        Err(PhraseError::ObjectOfIntransitive { .. })
    ));

    // Strict mode rejects guessed heads; default mode trusts the guess.
    let tree = clause(np("glorbina"), vp("spati"));
    assert!(matches!(
        realize(&tree, RealizeOpts::sentence().strict()),
        Err(PhraseError::GuessedHead { .. })
    ));
    assert!(realize(&tree, RealizeOpts::sentence()).is_ok());
}
