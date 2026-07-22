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

#[test]
fn integration_md_assembly_example_stays_current() {
    // Mirrors the worked runtime-assembly example in INTEGRATION.md.
    let count = 5u64;
    let coin = interslavic::quantified(
        count,
        "zlåtnik",
        Case::Acc,
        Gender::Masculine,
        Animacy::Inanimate,
    );
    let stole = interslavic::perfect_parts(
        "ukrasti",
        Person::Third,
        Number::Singular,
        Gender::Masculine,
    );
    let sentence = format!("Straž {} {} {}.", stole.participle, count, coin);
    assert_eq!(sentence, "Straž ukradl 5 zlåtnikov.");
}

#[test]
fn readme_pronoun_and_participle_examples_stay_current() {
    use PronounStyle::*;
    assert_eq!(
        interslavic::personal_pronoun(
            Person::Second,
            Number::Singular,
            Gender::Masculine,
            Case::Gen,
            Full
        ),
        Some("tebe".to_string())
    );
    assert_eq!(
        interslavic::personal_pronoun(
            Person::Third,
            Number::Singular,
            Gender::Masculine,
            Case::Gen,
            AfterPreposition
        ),
        Some("njego".to_string())
    );
    assert_eq!(
        interslavic::reflexive_pronoun(Case::Acc, Clitic),
        Some("sę".to_string())
    );

    assert_eq!(
        interslavic::l_participle("idti", Gender::Feminine, Number::Singular),
        "šla"
    );
    assert_eq!(
        interslavic::passive_participle(
            "osvětliti",
            Case::Nom,
            Number::Singular,
            Gender::Feminine,
            Animacy::Inanimate
        ),
        Some("osvětljena".to_string())
    );
}
