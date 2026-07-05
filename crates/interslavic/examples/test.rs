use interslavic::*;

fn main() {
    println!(
        "mųž gen sg: {}",
        ISV::noun("mųž", Case::Gen, Number::Singular)
    );
    println!(
        "Abhaz nom pl: {}",
        ISV::noun("Abhaz", Case::Nom, Number::Plural)
    );
    println!(
        "člen default acc sg: {}",
        ISV::noun("člen", Case::Acc, Number::Singular)
    );
    println!(
        "člen inanimate acc sg: {}",
        ISV::noun_with(
            "člen",
            Case::Acc,
            Number::Singular,
            NounGender::Masculine,
            Animacy::Inanimate,
        )
    );
    println!(
        "luč feminine gen sg: {}",
        ISV::noun_with(
            "luč",
            Case::Gen,
            Number::Singular,
            NounGender::Feminine,
            Animacy::Inanimate,
        )
    );

    println!(
        "osnovany na gen sg: {}",
        ISV::adj(
            "osnovany na",
            Case::Gen,
            Number::Singular,
            Gender::Masculine,
            Animacy::Animate
        )
    );

    println!(
        "učiti 1sg present: {}",
        ISV::verb(
            "učiti",
            Person::First,
            Number::Singular,
            Gender::Feminine,
            Tense::Present,
        )
    );
}
