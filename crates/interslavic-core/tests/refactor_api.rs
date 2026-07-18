use interslavic_core::{
    Animacy, Case, Gender, Number, Person, Tense, adjective, noun, utils, verb,
};

#[test]
fn character_helpers_are_optional_and_unicode_safe() {
    assert_eq!(utils::last_char(""), None);
    assert_eq!(utils::last_char("plain"), Some('n'));
    assert_eq!(utils::last_char("dobry🙂"), Some('🙂'));

    let value = "až🙂";
    assert_eq!(utils::last_n_chars(value, 0), "");
    assert_eq!(utils::last_n_chars(value, 1), "🙂");
    assert_eq!(utils::last_n_chars(value, 2), "ž🙂");
    assert_eq!(utils::last_n_chars(value, 3), value);
    assert_eq!(utils::last_n_chars(value, usize::MAX), value);
}

#[test]
fn focused_modules_preserve_representative_forms() {
    assert_eq!(
        noun::decline_noun("suma", Case::Acc, Number::Singular),
        "sumų"
    );
    assert_eq!(
        adjective::decline_adj(
            "dobry",
            Case::Gen,
            Number::Singular,
            Gender::Masculine,
            Animacy::Animate,
        ),
        "dobrogo"
    );
    assert_eq!(
        verb::conjugate_verb(
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
fn pluperfect_variant_selects_the_pluperfect_paradigm() {
    assert_eq!(
        verb::conjugate_verb(
            "pisati",
            Person::Third,
            Number::Singular,
            Gender::Feminine,
            Tense::Pluperfect,
        ),
        "běše pisala"
    );
}
