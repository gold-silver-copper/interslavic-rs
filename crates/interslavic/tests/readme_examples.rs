use interslavic::*;

#[test]
fn readme_noun_adjective_and_verb_examples_stay_current() {
    assert_eq!(
        interslavic::noun("adept", Case::Acc, Number::Singular),
        "adepta"
    );
    assert_eq!(
        interslavic::noun("oko", Case::Nom, Number::Plural),
        "oči / očesa"
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
        interslavic::noun_with(
            "mųž",
            Case::Acc,
            Number::Singular,
            Gender::Masculine,
            Animacy::Animate,
        ),
        "mųža"
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
        interslavic::verb(
            "učiti",
            Person::First,
            Number::Singular,
            Gender::Feminine,
            Tense::Present,
        ),
        "učų"
    );
}

#[test]
fn readme_verb_paradigm_examples_stay_current() {
    let pisati = interslavic::verb_forms("pisati");
    assert_eq!(pisati.future[0], "bųdų pisatì");
    assert_eq!(pisati.perfect[0], "jesm pisal(a)");
    assert_eq!(pisati.imperative, vec!["piši", "pišimo", "pišite"]);
    assert_eq!(pisati.gerund, "pisańje");

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
