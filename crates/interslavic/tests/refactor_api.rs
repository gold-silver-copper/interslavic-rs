use interslavic::{Animacy, Case, Gender, Number, Person, Tense, derivation::Pos};

#[test]
fn crate_root_single_form_api_remains_public_and_unchanged() {
    assert_eq!(
        interslavic::noun("adept", Case::Acc, Number::Singular),
        "adepta"
    );
    assert_eq!(
        interslavic::noun_with(
            "luč",
            Case::Gen,
            Number::Singular,
            Gender::Feminine,
            Animacy::Inanimate,
        ),
        "luči"
    );
    assert_eq!(
        interslavic::adj(
            "dobry",
            Case::Gen,
            Number::Singular,
            Gender::Masculine,
            Animacy::Animate,
        ),
        "dobrogo"
    );
    assert_eq!(
        interslavic::comparative("novy"),
        Some(("novějši".into(), "nověje".into()))
    );
    assert_eq!(
        interslavic::superlative("dobry"),
        Some(("najlěpši".into(), "najlěpje".into()))
    );
    assert_eq!(
        interslavic::pronoun(
            "toj",
            Case::Gen,
            Number::Singular,
            Gender::Masculine,
            Animacy::Inanimate,
        ),
        Some("togo".into())
    );
    assert_eq!(
        interslavic::numeral(
            "pęť",
            Case::Gen,
            Number::Plural,
            Gender::Masculine,
            Animacy::Inanimate,
        ),
        Some("pęti".into())
    );
    assert_eq!(
        interslavic::preposition_cases("na"),
        Some(&[Case::Acc, Case::Loc][..])
    );
    assert_eq!(
        interslavic::verb(
            "pisati",
            Person::First,
            Number::Singular,
            Gender::Feminine,
            Tense::Present,
        ),
        "pišų"
    );
    assert_eq!(
        interslavic::try_verb(
            "xyz",
            Person::First,
            Number::Singular,
            Gender::Feminine,
            Tense::Present,
        ),
        None
    );
    assert_eq!(
        interslavic::verb_with_present_hint(
            "bolěti",
            "(boli)",
            Person::First,
            Number::Singular,
            Gender::Masculine,
            Tense::Present,
        ),
        "boljų"
    );
}

#[test]
fn crate_root_paradigm_api_remains_public_and_unchanged() {
    let noun = interslavic::noun_forms("žena");
    assert_eq!(noun.get(Case::Gen, Number::Singular), "ženy");
    assert_eq!(noun.get(Case::Ins, Number::Singular), "ženojų");

    let noun_with = interslavic::noun_forms_with("kosť", Gender::Feminine, Animacy::Inanimate);
    assert_eq!(noun_with.gender, Gender::Feminine);
    assert_eq!(noun_with.get(Case::Gen, Number::Singular), "kosti");

    let adj = interslavic::adj_forms("dobry");
    assert_eq!(
        adj.get(
            Case::Gen,
            Number::Singular,
            Gender::Masculine,
            Animacy::Animate,
        ),
        "dobrogo"
    );

    let derived = interslavic::derive("kniga", Pos::Noun);
    assert!(derived.iter().any(|d| d.form == "knižny"));

    let verb = interslavic::verb_forms("pisati");
    assert_eq!(verb.future[0], "bųdų pisatì");
    assert_eq!(verb.imperative, vec!["piši", "pišimo", "pišite"]);

    assert_eq!(interslavic::try_verb_forms("xyz"), None);
    assert!(interslavic::try_verb_forms("pisati").is_some());

    let verb_with_metadata = interslavic::verb_forms_with_metadata("pisati", "(piše)", true, true);
    assert_eq!(verb_with_metadata.gerund, "pisańje");
    assert_eq!(
        verb_with_metadata.prap.as_deref(),
        Some("pišųćí (pišųćá, pišųćé)")
    );
}

#[test]
fn crate_root_functions_coexist_with_low_level_modules() {
    assert_eq!(
        interslavic::verb(
            "učiti",
            Person::First,
            Number::Singular,
            Gender::Feminine,
            Tense::Present,
        ),
        "učų"
    );
    assert_eq!(
        interslavic::verb::conjugate_verb(
            "učiti",
            Person::First,
            Number::Singular,
            Gender::Feminine,
            Tense::Present,
        ),
        "učų"
    );
    assert_eq!(
        interslavic::noun::decline_noun("suma", Case::Acc, Number::Singular),
        "sumų"
    );
}
