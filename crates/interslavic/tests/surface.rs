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
