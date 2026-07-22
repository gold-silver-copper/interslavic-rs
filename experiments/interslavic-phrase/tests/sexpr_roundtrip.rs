//! The S-expression contract: `compile(parse(print(tree))) == tree` for
//! every tree shape, canonical re-print stability for every golden
//! string, and no panics on garbage input.

use interslavic::{Case, Gender, Number, Person};
use interslavic_phrase::*;

fn trees() -> Vec<Clause> {
    vec![
        clause(np("otėc"), vp("kupiti").object(np("kniga"))).past(),
        clause(
            np("krålj").det("toj").adj("dobry"),
            vp("ukrasti").object(np("moneta").count(5).adj("zlåty")),
        )
        .past(),
        clause(
            pron(Person::First, Number::Singular, Gender::Masculine),
            vp("myti sę"),
        ),
        clause(
            np("kot"),
            vp("spati").pp(pp("pod", np("stol")).case(Case::Ins)),
        ),
        clause(np("otėc"), vp("kupiti").object(np("kniga")))
            .past()
            .negated()
            .force(Force::LiQuestion),
        clause(
            pron(Person::Third, Number::Plural, Gender::Feminine),
            vp("spati"),
        )
        .future()
        .prodrop(),
        clause(
            np("otėc"),
            vp("kupiti").object(np("kniga").case(Case::Gen)).pp(pp(
                "za",
                pron(Person::Third, Number::Singular, Gender::Neuter),
            )
            .case(Case::Acc)),
        )
        .force(Force::CiQuestion),
    ]
}

#[test]
fn print_parse_roundtrip_is_identity() {
    for tree in trees() {
        let printed = print(&tree);
        let reparsed = clause_from_str(&printed)
            .unwrap_or_else(|e| panic!("reparse of `{printed}` failed: {e}"));
        assert_eq!(
            reparsed, tree,
            "round-trip changed the tree for `{printed}`"
        );
        // Canonical: printing the reparse reproduces the same string.
        assert_eq!(print(&reparsed), printed);
    }
}

#[test]
fn reader_and_typed_builders_agree() {
    let from_text = clause_from_str(
        "(clause (np (det toj) (adj dobry) (n krålj)) \
                 (vp (v ukrasti) (np (num 5) (adj zlåty) (n moneta))) \
                 :tense past)",
    )
    .unwrap();
    let from_code = clause(
        np("krålj").det("toj").adj("dobry"),
        vp("ukrasti").object(np("moneta").count(5).adj("zlåty")),
    )
    .past();
    assert_eq!(from_text, from_code);
}

#[test]
fn garbage_input_never_panics() {
    for garbage in [
        "",
        "(",
        ")",
        "()",
        "(clause)",
        "(clause (np) (vp))",
        "(clause (np (n dom)) (vp (v spati)) :tense yesterday)",
        "(clause (np (n dom)) (vp (v spati)) :frobnicate)",
        "((()))",
        "(clause (np (n dom)) (vp (v spati))) extra",
        "(pron :sg)",
        "(clause (pron :9 :sg) (vp (v spati)))",
        ":::",
        "(clause (np (n dom)) (vp (v spati)) :tense",
        "(clause (np (n dom) (num x)) (vp (v spati)))",
        "žčš đćę (ų)",
    ] {
        // Must return Err (or Ok for well-formed input) — never panic.
        let _ = clause_from_str(garbage);
    }
}

#[test]
fn spans_point_into_the_input() {
    let input = "(clause (np (n dom)) (vp (v spati)) :tense yesterday)";
    let err = clause_from_str(input).unwrap_err();
    assert!(err.at < input.len());
    assert!(err.msg.contains("yesterday"));
}
