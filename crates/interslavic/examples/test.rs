use interslavic::*;

fn main() {
    assert_eq!(
        Interslavic::noun("mųž", Case::Gen, Number::Singular),
        "mųža"
    );
    assert_eq!(
        Interslavic::adjective(
            "samy",
            Case::Gen,
            Number::Singular,
            Gender::Masculine,
            Animacy::Animate,
        ),
        "samogo"
    );
    assert_eq!(
        Interslavic::verb("učiti", Person::First, Number::Singular, Tense::Present),
        "učų"
    );
    assert_eq!(
        Interslavic::l_participle("buditi", &Gender::Feminine, &Number::Singular),
        "budila"
    );
}
