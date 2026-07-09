use interslavic::*;

fn pron(l: &str, c: Case, n: Number, g: Gender, a: Animacy) -> Option<String> {
    ISV::pronoun(l, c, n, g, a)
}
fn num(l: &str, c: Case, n: Number, g: Gender, a: Animacy) -> Option<String> {
    ISV::numeral(l, c, n, g, a)
}

// Shorthands for the common masc-inanimate singular query.
fn ms(l: &str, c: Case) -> Option<String> {
    ISV::pronoun(
        l,
        c,
        Number::Singular,
        Gender::Masculine,
        Animacy::Inanimate,
    )
}

#[test]
fn toj_class_hard_pronominal() {
    // Masculine singular: the citation form is the nominative (and the
    // syncretic inanimate accusative); obliques are the hard pronominal set.
    assert_eq!(ms("toj", Case::Nom), Some("toj".into()));
    assert_eq!(ms("toj", Case::Acc), Some("toj".into())); // inanimate acc = nom
    assert_eq!(ms("toj", Case::Gen), Some("togo".into()));
    assert_eq!(ms("toj", Case::Dat), Some("tomu".into()));
    assert_eq!(ms("toj", Case::Loc), Some("tom".into()));
    assert_eq!(ms("toj", Case::Ins), Some("tym".into()));
    // Masculine animate accusative = genitive.
    assert_eq!(
        pron(
            "toj",
            Case::Acc,
            Number::Singular,
            Gender::Masculine,
            Animacy::Animate
        ),
        Some("togo".into())
    );
    // Feminine / neuter singular.
    assert_eq!(
        pron(
            "toj",
            Case::Nom,
            Number::Singular,
            Gender::Feminine,
            Animacy::Inanimate
        ),
        Some("ta".into())
    );
    assert_eq!(
        pron(
            "toj",
            Case::Acc,
            Number::Singular,
            Gender::Feminine,
            Animacy::Inanimate
        ),
        Some("tų".into())
    );
    assert_eq!(
        pron(
            "toj",
            Case::Ins,
            Number::Singular,
            Gender::Feminine,
            Animacy::Inanimate
        ),
        Some("tojų".into())
    );
    assert_eq!(
        pron(
            "toj",
            Case::Nom,
            Number::Singular,
            Gender::Neuter,
            Animacy::Inanimate
        ),
        Some("to".into())
    );
    // Plural.
    assert_eq!(
        pron(
            "toj",
            Case::Nom,
            Number::Plural,
            Gender::Masculine,
            Animacy::Animate
        ),
        Some("ti".into())
    );
    assert_eq!(
        pron(
            "toj",
            Case::Gen,
            Number::Plural,
            Gender::Masculine,
            Animacy::Inanimate
        ),
        Some("tyh".into())
    );
    assert_eq!(
        pron(
            "toj",
            Case::Ins,
            Number::Plural,
            Gender::Masculine,
            Animacy::Inanimate
        ),
        Some("tymi".into())
    );
    // ov declines like toj (stem = whole lemma).
    assert_eq!(ms("ov", Case::Gen), Some("ovogo".into()));
    assert_eq!(ms("ov", Case::Nom), Some("ov".into()));
}

#[test]
fn moj_class_soft_pronominal() {
    assert_eq!(ms("moj", Case::Nom), Some("moj".into()));
    assert_eq!(ms("moj", Case::Gen), Some("mojego".into()));
    assert_eq!(ms("moj", Case::Dat), Some("mojemu".into()));
    assert_eq!(ms("moj", Case::Ins), Some("mojim".into()));
    assert_eq!(
        pron(
            "moj",
            Case::Acc,
            Number::Singular,
            Gender::Feminine,
            Animacy::Inanimate
        ),
        Some("mojų".into())
    );
    assert_eq!(
        pron(
            "moj",
            Case::Ins,
            Number::Singular,
            Gender::Feminine,
            Animacy::Inanimate
        ),
        Some("mojejų".into())
    );
    // naš / vaš / čij follow the same soft pattern.
    assert_eq!(ms("naš", Case::Gen), Some("našego".into()));
    assert_eq!(ms("naš", Case::Nom), Some("naš".into()));
    assert_eq!(ms("čij", Case::Gen), Some("čijego".into()));
}

#[test]
fn ves_soft_pronominal() {
    assert_eq!(ms("veś", Case::Nom), Some("veś".into()));
    assert_eq!(ms("veś", Case::Gen), Some("vsego".into()));
    assert_eq!(
        pron(
            "veś",
            Case::Nom,
            Number::Singular,
            Gender::Feminine,
            Animacy::Inanimate
        ),
        Some("vsa".into())
    );
    assert_eq!(
        pron(
            "veś",
            Case::Nom,
            Number::Plural,
            Gender::Masculine,
            Animacy::Animate
        ),
        Some("vsi".into())
    );
}

#[test]
fn kto_cto_and_koli() {
    // kto (animate): accusative = genitive = kogo.
    assert_eq!(ms("kto", Case::Nom), Some("kto".into()));
    assert_eq!(ms("kto", Case::Gen), Some("kogo".into()));
    assert_eq!(ms("kto", Case::Acc), Some("kogo".into()));
    assert_eq!(ms("kto", Case::Dat), Some("komu".into()));
    assert_eq!(ms("kto", Case::Ins), Some("kym".into()));
    assert_eq!(ms("kto", Case::Loc), Some("kom".into()));
    // čto (inanimate): accusative = nominative = čto.
    assert_eq!(ms("čto", Case::Nom), Some("čto".into()));
    assert_eq!(ms("čto", Case::Acc), Some("čto".into()));
    assert_eq!(ms("čto", Case::Gen), Some("čego".into()));
    // Derivatives inflect on the kto/čto tail.
    assert_eq!(ms("nikto", Case::Gen), Some("nikogo".into()));
    // -koli indefinites inflect internally.
    assert_eq!(ms("ktokoli", Case::Gen), Some("kogokoli".into()));
    assert_eq!(ms("ktokoli", Case::Nom), Some("ktokoli".into()));
}

#[test]
fn adjectival_determiners() {
    // ktory / kaky decline as ordinary adjectives (masc nom = citation).
    assert_eq!(ms("ktory", Case::Nom), Some("ktory".into()));
    assert_eq!(ms("ktory", Case::Gen), Some("ktorogo".into()));
    assert_eq!(ms("kaky", Case::Gen), Some("kakogo".into()));
}

#[test]
fn jedin_and_cardinals() {
    // jedin: adjectival, irregular masculine nominative.
    assert_eq!(
        num(
            "jedin",
            Case::Nom,
            Number::Singular,
            Gender::Masculine,
            Animacy::Inanimate
        ),
        Some("jedin".into())
    );
    assert_eq!(
        num(
            "jedin",
            Case::Gen,
            Number::Singular,
            Gender::Masculine,
            Animacy::Inanimate
        ),
        Some("jednogo".into())
    );
    assert_eq!(
        num(
            "jedin",
            Case::Nom,
            Number::Singular,
            Gender::Feminine,
            Animacy::Inanimate
        ),
        Some("jedna".into())
    );
    // dva: gender only in nom/acc.
    assert_eq!(
        num(
            "dva",
            Case::Nom,
            Number::Plural,
            Gender::Masculine,
            Animacy::Inanimate
        ),
        Some("dva".into())
    );
    assert_eq!(
        num(
            "dva",
            Case::Nom,
            Number::Plural,
            Gender::Feminine,
            Animacy::Inanimate
        ),
        Some("dvě".into())
    );
    assert_eq!(
        num(
            "dva",
            Case::Gen,
            Number::Plural,
            Gender::Masculine,
            Animacy::Inanimate
        ),
        Some("dvoh".into())
    );
    assert_eq!(
        num(
            "dva",
            Case::Ins,
            Number::Plural,
            Gender::Masculine,
            Animacy::Inanimate
        ),
        Some("dvoma".into())
    );
    // tri / četyri.
    assert_eq!(
        num(
            "tri",
            Case::Gen,
            Number::Plural,
            Gender::Masculine,
            Animacy::Inanimate
        ),
        Some("trěh".into())
    );
    assert_eq!(
        num(
            "tri",
            Case::Dat,
            Number::Plural,
            Gender::Masculine,
            Animacy::Inanimate
        ),
        Some("trěm".into())
    );
    assert_eq!(
        num(
            "tri",
            Case::Ins,
            Number::Plural,
            Gender::Masculine,
            Animacy::Inanimate
        ),
        Some("trěmi".into())
    );
}

#[test]
fn i_stem_numerals_and_ordinals() {
    // pęť declines like kosť.
    assert_eq!(
        num(
            "pęť",
            Case::Nom,
            Number::Plural,
            Gender::Masculine,
            Animacy::Inanimate
        ),
        Some("pęť".into())
    );
    assert_eq!(
        num(
            "pęť",
            Case::Gen,
            Number::Plural,
            Gender::Masculine,
            Animacy::Inanimate
        ),
        Some("pęti".into())
    );
    assert_eq!(
        num(
            "pęť",
            Case::Ins,
            Number::Plural,
            Gender::Masculine,
            Animacy::Inanimate
        ),
        Some("pęťjų".into())
    );
    // Ordinals decline as adjectives.
    assert_eq!(
        num(
            "pŕvy",
            Case::Gen,
            Number::Singular,
            Gender::Masculine,
            Animacy::Inanimate
        ),
        Some("pŕvogo".into())
    );
    assert_eq!(
        num(
            "drugy",
            Case::Dat,
            Number::Singular,
            Gender::Masculine,
            Animacy::Inanimate
        ),
        Some("drugomu".into())
    );
}

#[test]
fn unrecognized_lemmas_return_none() {
    assert_eq!(ms("stol", Case::Gen), None);
    assert_eq!(ms("", Case::Gen), None);
    assert_eq!(ms("oj", Case::Gen), None); // bare conjunction, below the -oj guard
    assert_eq!(
        num(
            "stol",
            Case::Gen,
            Number::Singular,
            Gender::Masculine,
            Animacy::Inanimate
        ),
        None
    );
}
