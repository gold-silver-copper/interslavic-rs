//! Print every golden sentence, one per line — the input for
//! `cargo xtask phrase-check`, which runs them through slovowiki's
//! independent agreement checker.

use interslavic::{Case, Gender, Number, Person};
use interslavic_phrase::*;

fn main() {
    let trees: Vec<Clause> = vec![
        clause(np("otėc"), vp("kupiti").object(np("kniga"))).past(),
        clause(
            pron(Person::First, Number::Singular, Gender::Masculine),
            vp("myti sę"),
        ),
        clause(np("otėc"), vp("kupiti").object(np("kniga")))
            .past()
            .force(Force::CiQuestion),
        clause(np("otėc"), vp("kupiti").object(np("kniga")))
            .past()
            .force(Force::LiQuestion),
        clause(
            np("kot"),
            vp("spati").pp(pp("pod", np("stol")).case(Case::Ins)),
        ),
        clause(
            np("kot"),
            vp("poběgti").pp(pp("pod", np("stol")).case(Case::Acc)),
        )
        .past(),
        clause(
            np("krålj").det("toj").adj("dobry"),
            vp("ukrasti").object(np("moneta").count(5).adj("zlåty")),
        )
        .past(),
        clause(np("otėc"), vp("kupiti").object(np("kniga")))
            .past()
            .negated(),
        clause(np("žena"), vp("kupiti").object(np("kniga"))).past(),
        clause(
            np("otėc"),
            vp("kupiti").object(np("kniga")).pp(pp(
                "za",
                pron(Person::Third, Number::Singular, Gender::Masculine),
            )
            .case(Case::Acc)),
        )
        .past(),
    ];
    for tree in &trees {
        println!("{}", realize(tree, RealizeOpts::sentence()).unwrap());
    }
}
