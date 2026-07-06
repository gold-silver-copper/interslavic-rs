use interslavic::*;

#[test]
fn readme_noun_adjective_and_verb_examples_stay_current() {
    assert_eq!(ISV::noun("adept", Case::Acc, Number::Singular), "adepta");
    assert_eq!(ISV::noun("oko", Case::Nom, Number::Plural), "oči / očesa");

    assert_eq!(
        ISV::noun_with(
            "luč",
            Case::Gen,
            Number::Singular,
            NounGender::Feminine,
            Animacy::Inanimate,
        ),
        "luči"
    );
    assert_eq!(
        ISV::noun_with(
            "mųž",
            Case::Acc,
            Number::Singular,
            NounGender::Masculine,
            Animacy::Animate,
        ),
        "mųža"
    );

    assert_eq!(
        ISV::adj(
            "dobry",
            Case::Gen,
            Number::Singular,
            Gender::Masculine,
            Animacy::Animate,
        ),
        "dobrogo"
    );
    assert_eq!(
        ISV::verb(
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
    let pisati = ISV::verb_forms("pisati");
    assert_eq!(pisati.future[0], "bųdų pisatì");
    assert_eq!(pisati.perfect[0], "jesm pisal(a)");
    assert_eq!(pisati.imperative, vec!["piši", "pišimo", "pišite"]);
    assert_eq!(pisati.gerund, "pisańje");

    assert_eq!(
        ISV::verb_with_present_hint(
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
