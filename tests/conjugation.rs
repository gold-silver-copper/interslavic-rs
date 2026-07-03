use interslavic::*;

#[test]
fn second_conjugation_drops_extra_j_after_soft_consonants() {
    assert_eq!(
        ISV::conjugate_verb(
            "učiti",
            &Person::First,
            &Number::Singular,
            &Gender::Feminine,
            &Tense::Present,
        ),
        "učų"
    );
}
