use interslavic::*;

#[test]
fn compound_adjective_phrases_decline_head_and_append_postfix() {
    assert_eq!(
        ISV::adj(
            "osnovany na",
            Case::Gen,
            Number::Singular,
            Gender::Masculine,
            true,
        ),
        "osnovanogo na"
    );
    assert_eq!(
        ISV::adj(
            "pȯlny naděje",
            Case::Nom,
            Number::Plural,
            Gender::Masculine,
            true,
        ),
        "pȯlni naděje"
    );
}

#[test]
fn explicit_masc_fem_nouns_require_gender_override() {
    let missing = ISV::noun_as(NounDeclensionRequest {
        lemma: "luč",
        gender: NounGender::MasculineOrFeminine,
        animacy: Animacy::Inanimate,
        number_restriction: NumberRestriction::Countable,
        indeclinable: false,
        addition: None,
        gender_override: None,
    });
    assert_eq!(missing.unwrap_err(), InflectionError::MissingGenderOverride);

    let invalid = ISV::noun_as(NounDeclensionRequest {
        lemma: "luč",
        gender: NounGender::MasculineOrFeminine,
        animacy: Animacy::Inanimate,
        number_restriction: NumberRestriction::Countable,
        indeclinable: false,
        addition: None,
        gender_override: Some(NounGender::Neuter),
    });
    assert_eq!(invalid.unwrap_err(), InflectionError::InvalidGenderOverride);

    let feminine = ISV::noun_as(NounDeclensionRequest {
        lemma: "luč",
        gender: NounGender::MasculineOrFeminine,
        animacy: Animacy::Inanimate,
        number_restriction: NumberRestriction::Countable,
        indeclinable: false,
        addition: None,
        gender_override: Some(NounGender::Feminine),
    })
    .unwrap();
    assert_eq!(
        feminine.genitive_singular.unwrap().alternatives,
        vec!["luči"]
    );

    let all_luč = ISV::noun("luč").unwrap();
    assert!(all_luč.iter().any(|p| p.gender == NounGender::Masculine));
    assert!(all_luč.iter().any(|p| p.gender == NounGender::Feminine));
    assert_eq!(
        ISV::noun_id("339").unwrap_err(),
        InflectionError::MissingGenderOverride
    );
    assert_eq!(
        ISV::noun_id_as("339", NounGender::Feminine)
            .unwrap()
            .genitive_singular
            .unwrap()
            .alternatives,
        vec!["luči"]
    );
}

#[test]
fn neuter_je_genitive_plural_distinguishes_ij_and_ej_classes() {
    assert_eq!(
        ISV::noun_form("kopje", Case::Gen, Number::Plural)
            .unwrap()
            .text(),
        "kopij"
    );
    assert_eq!(
        ISV::noun_form("obdobje", Case::Gen, Number::Plural)
            .unwrap()
            .text(),
        "obdobij"
    );
    assert_eq!(
        ISV::noun_form("morje", Case::Gen, Number::Plural)
            .unwrap()
            .text(),
        "morej"
    );
    assert_eq!(
        ISV::noun_form("polje", Case::Gen, Number::Plural)
            .unwrap()
            .text(),
        "polej"
    );
}

#[test]
fn feminine_fluent_vowel_v_and_sibilant_nouns_preserve_vowel() {
    assert_eq!(
        ISV::noun_form("brȯv", Case::Ins, Number::Singular)
            .unwrap()
            .text(),
        "brȯvjų"
    );
    assert_eq!(
        ISV::noun_form("krȯv", Case::Ins, Number::Singular)
            .unwrap()
            .text(),
        "krȯvjų"
    );
    assert_eq!(
        ISV::noun_form("lȯž", Case::Nom, Number::Singular)
            .unwrap()
            .text(),
        "lȯž"
    );
}

#[test]
fn seksi_matches_reference_softening() {
    assert_eq!(
        ISV::adj(
            "seksi",
            Case::Nom,
            Number::Singular,
            Gender::Masculine,
            false
        ),
        "sekśi"
    );
    assert_eq!(
        ISV::adj(
            "seksi",
            Case::Acc,
            Number::Singular,
            Gender::Masculine,
            false
        ),
        "seksi"
    );
    assert_eq!(
        ISV::adj("seksi", Case::Gen, Number::Plural, Gender::Feminine, false),
        "sekśih"
    );
}
