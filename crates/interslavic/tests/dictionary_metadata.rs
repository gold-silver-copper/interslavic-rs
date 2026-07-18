use interslavic::*;

fn noun(word: &str, case: Case, number: Number) -> String {
    ISV::noun(word, case, number)
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
fn generated_lookup_strips_bracketed_dictionary_annotations() {
    assert_eq!(noun("hektar", Case::Gen, Number::Singular), "hektara");
    assert_eq!(noun("uško", Case::Gen, Number::Plural), "ušek");
}

#[test]
fn ambiguous_lemmas_default_to_first_dictionary_row_and_allow_overrides() {
    assert_eq!(noun("člen", Case::Acc, Number::Singular), "člena");
    assert_eq!(
        ISV::noun_with(
            "člen",
            Case::Acc,
            Number::Singular,
            Gender::Masculine,
            Animacy::Inanimate,
        ),
        "člen"
    );
}

#[test]
fn noun_with_prefers_number_compatible_duplicate_metadata() {
    assert_eq!(
        ISV::noun_with(
            "anestezija",
            Case::Nom,
            Number::Plural,
            Gender::Feminine,
            Animacy::Inanimate,
        ),
        "anestezije"
    );
    assert_eq!(
        ISV::noun_with(
            "bor",
            Case::Gen,
            Number::Plural,
            Gender::Masculine,
            Animacy::Inanimate,
        ),
        "borov"
    );
}
