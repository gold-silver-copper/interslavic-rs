use interslavic::*;

#[test]
fn adjective_phrases_are_not_declined_as_a_unit() {
    assert_eq!(
        ISV::adj(
            "osnovany na",
            Case::Gen,
            Number::Singular,
            Gender::Masculine,
            Animacy::Animate,
        ),
        "osnovany na"
    );
    assert_eq!(
        ISV::adj(
            "pȯlny naděje",
            Case::Nom,
            Number::Plural,
            Gender::Masculine,
            Animacy::Animate,
        ),
        "pȯlny naděje"
    );
}

#[test]
fn noun_with_overrides_gender_and_animacy_directly() {
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
            "luč",
            Case::Gen,
            Number::Singular,
            NounGender::Masculine,
            Animacy::Inanimate,
        ),
        "luča"
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
        ISV::noun_with(
            "mųž",
            Case::Acc,
            Number::Singular,
            NounGender::Masculine,
            Animacy::Inanimate,
        ),
        "mųž"
    );
}

#[test]
fn neuter_je_genitive_plural_distinguishes_ij_and_ej_classes() {
    assert_eq!(ISV::noun("kopje", Case::Gen, Number::Plural), "kopij");
    assert_eq!(ISV::noun("obdobje", Case::Gen, Number::Plural), "obdobij");
    assert_eq!(ISV::noun("morje", Case::Gen, Number::Plural), "morej");
    assert_eq!(ISV::noun("polje", Case::Gen, Number::Plural), "polej");
}

#[test]
fn feminine_fluent_vowel_v_and_sibilant_nouns_preserve_vowel() {
    assert_eq!(ISV::noun("brȯv", Case::Ins, Number::Singular), "brȯvjų");
    assert_eq!(ISV::noun("krȯv", Case::Ins, Number::Singular), "krȯvjų");
    assert_eq!(ISV::noun("lȯž", Case::Nom, Number::Singular), "lȯž");
}

#[test]
fn seksi_matches_reference_softening() {
    assert_eq!(
        ISV::adj(
            "seksi",
            Case::Nom,
            Number::Singular,
            Gender::Masculine,
            Animacy::Inanimate
        ),
        "sekśi"
    );
    assert_eq!(
        ISV::adj(
            "seksi",
            Case::Acc,
            Number::Singular,
            Gender::Masculine,
            Animacy::Inanimate
        ),
        "seksi"
    );
    assert_eq!(
        ISV::adj(
            "seksi",
            Case::Gen,
            Number::Plural,
            Gender::Feminine,
            Animacy::Inanimate
        ),
        "sekśih"
    );
}
