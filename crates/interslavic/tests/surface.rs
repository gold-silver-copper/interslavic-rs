use interslavic::*;

#[test]
fn dictionary_backed_surface_api() {
    assert_eq!(
        Interslavic::noun("mųž", Case::Gen, Number::Singular),
        "mųža"
    );
    assert_eq!(
        Interslavic::decline_noun("mųž", &Case::Gen, &Number::Singular).1,
        Gender::Masculine
    );
    assert!(Interslavic::noun_is_animate("mųž"));
}

#[test]
fn sonic_utils_noun_samples() {
    assert_eq!(Interslavic::noun("mųž", Case::Voc, Number::Singular), "mųžu");
    assert_eq!(Interslavic::noun("žena", Case::Voc, Number::Singular), "ženo");
    assert_eq!(Interslavic::noun("slovo", Case::Gen, Number::Singular), "slova / slovese");
    assert_eq!(Interslavic::noun("oko", Case::Nom, Number::Plural), "oči / očesa");
    assert_eq!(Interslavic::noun("člověk", Case::Nom, Number::Plural), "ljudi");
    assert_eq!(Interslavic::noun("dėń", Case::Gen, Number::Singular), "dne / dnja");
}

#[test]
fn sonic_utils_adjective_and_verb_samples() {
    assert_eq!(
        Interslavic::adjective(
            "naš",
            Case::Gen,
            Number::Singular,
            Gender::Masculine,
            Animacy::Animate,
        ),
        "našego"
    );
    assert_eq!(
        Interslavic::adjective(
            "dobry",
            Case::Acc,
            Number::Singular,
            Gender::Feminine,
            Animacy::Inanimate,
        ),
        "dobrų"
    );
    assert_eq!(
        Interslavic::verb("byti", Person::First, Number::Singular, Tense::Present),
        "jesm"
    );
    assert_eq!(
        Interslavic::verb("jesti", Person::Third, Number::Plural, Tense::Present),
        "jedųt"
    );
    assert_eq!(
        Interslavic::verb("dati", Person::Second, Number::Singular, Tense::Present),
        "daš"
    );
    assert_eq!(
        Interslavic::verb("buditi", Person::First, Number::Singular, Tense::Present),
        "buđų"
    );
}

#[test]
fn readme_forms() {
    assert_eq!(
        Interslavic::decline_adj(
            "samy",
            &Case::Gen,
            &Number::Singular,
            &Gender::Masculine,
            true,
        ),
        "samogo"
    );
    assert_eq!(
        Interslavic::conjugate_verb(
            "učiti",
            &Person::First,
            &Number::Singular,
            &Gender::Feminine,
            &Tense::Present,
        ),
        "učų"
    );
    assert_eq!(
        Interslavic::l_participle("buditi", &Gender::Feminine, &Number::Singular),
        "budila"
    );
}
