use interslavic::*;

fn noun(word: &str, case: Case, number: Number) -> String {
    ISV::noun_form(word, case, number).unwrap().text()
}

#[test]
fn dictionary_metadata_preserves_case_and_animacy() {
    assert_eq!(noun("Abhaz", Case::Nom, Number::Plural), "Abhazi");
    assert_eq!(noun("Abhaz", Case::Acc, Number::Singular), "Abhaza");
    assert_eq!(noun("Abhaz", Case::Acc, Number::Plural), "Abhazov");
    assert_eq!(noun("adept", Case::Nom, Number::Plural), "adepti");
    assert_eq!(noun("adept", Case::Acc, Number::Singular), "adepta");
    assert_eq!(noun("adept", Case::Acc, Number::Plural), "adeptov");
}

#[test]
fn dictionary_metadata_handles_feminine_soft_classes() {
    assert_eq!(
        noun("absolutnosť", Case::Gen, Number::Singular),
        "absolutnosti"
    );
    assert_eq!(
        noun("absolutnosť", Case::Nom, Number::Plural),
        "absolutnosti"
    );
    assert_eq!(
        noun("absolutnosť", Case::Ins, Number::Singular),
        "absolutnosťjų"
    );
}

#[test]
fn dictionary_metadata_uses_addition_for_fleeting_vowels() {
    assert_eq!(noun("Afrikanėc", Case::Gen, Number::Singular), "Afrikanca");
    assert_eq!(noun("Afrikanėc", Case::Nom, Number::Plural), "Afrikanci");
}

#[test]
fn dictionary_metadata_handles_indeclinable_entries() {
    assert_eq!(noun("cunami", Case::Gen, Number::Singular), "cunami");
    assert_eq!(noun("cunami", Case::Ins, Number::Plural), "cunami");
}

#[test]
fn dictionary_id_api_disambiguates_homonyms() {
    assert_eq!(
        ISV::noun_id("640")
            .unwrap()
            .get(Case::Acc, Number::Singular)
            .unwrap()
            .text(),
        "člena"
    );
    assert_eq!(
        ISV::noun_id("25028")
            .unwrap()
            .get(Case::Acc, Number::Singular)
            .unwrap()
            .text(),
        "člen"
    );
}
