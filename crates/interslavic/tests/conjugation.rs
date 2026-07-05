use interslavic::*;

#[test]
fn second_conjugation_drops_extra_j_after_soft_consonants() {
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
fn dictionary_present_hints_match_sonic_reference() {
    assert_eq!(
        ISV::verb(
            "pisati",
            Person::First,
            Number::Singular,
            Gender::Feminine,
            Tense::Present,
        ),
        "pišų"
    );
    assert_eq!(
        ISV::verb(
            "dělati",
            Person::Third,
            Number::Singular,
            Gender::Feminine,
            Tense::Present,
        ),
        "dělaje"
    );
    assert_eq!(
        ISV::verb(
            "byti",
            Person::Third,
            Number::Plural,
            Gender::Feminine,
            Tense::Present,
        ),
        "sųt"
    );
}

#[test]
fn ovati_and_evati_verbs_use_uj_present_stem() {
    assert_eq!(
        ISV::verb(
            "aranževati",
            Person::First,
            Number::Singular,
            Gender::Feminine,
            Tense::Present,
        ),
        "aranžujų"
    );
    assert_eq!(
        ISV::verb(
            "abdikovati",
            Person::Second,
            Number::Plural,
            Gender::Feminine,
            Tense::Present,
        ),
        "abdikujete"
    );
}

#[test]
fn raw_phrase_strings_are_not_conjugated_as_core_lemmas() {
    assert_eq!(
        ISV::verb(
            "bazovati na",
            Person::First,
            Number::Singular,
            Gender::Feminine,
            Tense::Present,
        ),
        "bazovati na"
    );
}
