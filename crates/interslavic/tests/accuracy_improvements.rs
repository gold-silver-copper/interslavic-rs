use interslavic::*;

#[test]
fn adjective_phrases_are_not_declined_as_a_unit() {
    assert_eq!(
        interslavic::adj(
            "osnovany na",
            Case::Gen,
            Number::Singular,
            Gender::Masculine,
            Animacy::Animate,
        ),
        "osnovany na"
    );
    assert_eq!(
        interslavic::adj(
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
            "luč",
            Case::Gen,
            Number::Singular,
            Gender::Masculine,
            Animacy::Inanimate,
        ),
        "luča"
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
        interslavic::noun_with(
            "mųž",
            Case::Acc,
            Number::Singular,
            Gender::Masculine,
            Animacy::Inanimate,
        ),
        "mųž"
    );
}

#[test]
fn neuter_je_genitive_plural_distinguishes_ij_and_ej_classes() {
    assert_eq!(
        interslavic::noun("kopje", Case::Gen, Number::Plural),
        "kopij"
    );
    assert_eq!(
        interslavic::noun("obdobje", Case::Gen, Number::Plural),
        "obdobij"
    );
    assert_eq!(
        interslavic::noun("morje", Case::Gen, Number::Plural),
        "morej"
    );
    assert_eq!(
        interslavic::noun("polje", Case::Gen, Number::Plural),
        "polej"
    );
}

#[test]
fn feminine_fluent_vowel_v_and_sibilant_nouns_preserve_vowel() {
    assert_eq!(
        interslavic::noun("brȯv", Case::Ins, Number::Singular),
        "brȯvjų"
    );
    assert_eq!(
        interslavic::noun("krȯv", Case::Ins, Number::Singular),
        "krȯvjų"
    );
    assert_eq!(interslavic::noun("lȯž", Case::Nom, Number::Singular), "lȯž");
}

#[test]
fn seksi_matches_reference_softening() {
    assert_eq!(
        interslavic::adj(
            "seksi",
            Case::Nom,
            Number::Singular,
            Gender::Masculine,
            Animacy::Inanimate
        ),
        "sekśi"
    );
    assert_eq!(
        interslavic::adj(
            "seksi",
            Case::Acc,
            Number::Singular,
            Gender::Masculine,
            Animacy::Inanimate
        ),
        "seksi"
    );
    assert_eq!(
        interslavic::adj(
            "seksi",
            Case::Gen,
            Number::Plural,
            Gender::Feminine,
            Animacy::Inanimate
        ),
        "sekśih"
    );
}
