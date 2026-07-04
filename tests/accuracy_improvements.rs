use interslavic::*;

#[test]
fn compound_adjective_phrases_decline_head_and_append_postfix() {
    assert_eq!(
        ISV::decline_adj(
            "osnovany na",
            &Case::Gen,
            &Number::Singular,
            &Gender::Masculine,
            true,
        ),
        "osnovanogo na"
    );
    assert_eq!(
        ISV::decline_adj(
            "pȯlny naděje",
            &Case::Nom,
            &Number::Plural,
            &Gender::Masculine,
            true,
        ),
        "pȯlni naděje"
    );
}

#[test]
fn explicit_masc_fem_nouns_require_gender_override() {
    let missing = ISV::decline_noun_explicit(NounDeclensionRequest {
        lemma: "luč",
        gender: NounGender::MasculineOrFeminine,
        animacy: Animacy::Inanimate,
        number_restriction: NumberRestriction::Countable,
        indeclinable: false,
        addition: None,
        dictionary_id: None,
        gender_override: None,
    });
    assert_eq!(missing.unwrap_err(), InflectionError::MissingGenderOverride);

    let invalid = ISV::decline_noun_explicit(NounDeclensionRequest {
        lemma: "luč",
        gender: NounGender::MasculineOrFeminine,
        animacy: Animacy::Inanimate,
        number_restriction: NumberRestriction::Countable,
        indeclinable: false,
        addition: None,
        dictionary_id: None,
        gender_override: Some(NounGender::Neuter),
    });
    assert_eq!(invalid.unwrap_err(), InflectionError::InvalidGenderOverride);

    let feminine = ISV::decline_noun_explicit(NounDeclensionRequest {
        lemma: "luč",
        gender: NounGender::MasculineOrFeminine,
        animacy: Animacy::Inanimate,
        number_restriction: NumberRestriction::Countable,
        indeclinable: false,
        addition: None,
        dictionary_id: None,
        gender_override: Some(NounGender::Feminine),
    })
    .unwrap();
    assert_eq!(
        feminine.genitive_singular.unwrap().alternatives,
        vec!["luči"]
    );

    let all_luč = ISV::decline_noun_all("luč").unwrap();
    assert!(all_luč.iter().any(|p| p.gender == NounGender::Masculine));
    assert!(all_luč.iter().any(|p| p.gender == NounGender::Feminine));
    assert_eq!(
        ISV::decline_noun_by_id("339").unwrap_err(),
        InflectionError::MissingGenderOverride
    );
    assert_eq!(
        ISV::decline_noun_by_id_with_gender_override("339", NounGender::Feminine)
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
        ISV::decline_noun("kopje", &Case::Gen, &Number::Plural).0,
        "kopij"
    );
    assert_eq!(
        ISV::decline_noun("obdobje", &Case::Gen, &Number::Plural).0,
        "obdobij"
    );
    assert_eq!(
        ISV::decline_noun("morje", &Case::Gen, &Number::Plural).0,
        "morej"
    );
    assert_eq!(
        ISV::decline_noun("polje", &Case::Gen, &Number::Plural).0,
        "polej"
    );
}

#[test]
fn feminine_fluent_vowel_v_and_sibilant_nouns_preserve_vowel() {
    assert_eq!(
        ISV::decline_noun("brȯv", &Case::Ins, &Number::Singular).0,
        "brȯvjų"
    );
    assert_eq!(
        ISV::decline_noun("krȯv", &Case::Ins, &Number::Singular).0,
        "krȯvjų"
    );
    assert_eq!(
        ISV::decline_noun("lȯž", &Case::Nom, &Number::Singular).0,
        "lȯž"
    );
}

#[test]
fn seksi_matches_reference_softening() {
    assert_eq!(
        ISV::decline_adj(
            "seksi",
            &Case::Nom,
            &Number::Singular,
            &Gender::Masculine,
            false
        ),
        "sekśi"
    );
    assert_eq!(
        ISV::decline_adj(
            "seksi",
            &Case::Acc,
            &Number::Singular,
            &Gender::Masculine,
            false
        ),
        "seksi"
    );
    assert_eq!(
        ISV::decline_adj(
            "seksi",
            &Case::Gen,
            &Number::Plural,
            &Gender::Feminine,
            false
        ),
        "sekśih"
    );
}
