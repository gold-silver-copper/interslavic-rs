use interslavic::{Animacy, Case, Gender, Number, Person, Tense, ISV};

#[test]
fn dictionary_backed_facade_remains_public_and_unchanged() {
    assert_eq!(ISV::noun("adept", Case::Acc, Number::Singular), "adepta");
    assert_eq!(
        ISV::adj(
            "dobry",
            Case::Gen,
            Number::Singular,
            Gender::Masculine,
            Animacy::Animate,
        ),
        "dobrogo"
    );
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
}
