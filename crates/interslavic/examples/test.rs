use interslavic::*;

fn main() {
    println!(
        "mųž gen sg: {}",
        interslavic::noun("mųž", Case::Gen, Number::Singular)
    );
    println!(
        "Abhaz nom pl: {}",
        interslavic::noun("Abhaz", Case::Nom, Number::Plural)
    );
    println!(
        "člen default acc sg: {}",
        interslavic::noun("člen", Case::Acc, Number::Singular)
    );
    println!(
        "člen inanimate acc sg: {}",
        interslavic::noun_with(
            "člen",
            Case::Acc,
            Number::Singular,
            Gender::Masculine,
            Animacy::Inanimate,
        )
    );
    println!(
        "luč feminine gen sg: {}",
        interslavic::noun_with(
            "luč",
            Case::Gen,
            Number::Singular,
            Gender::Feminine,
            Animacy::Inanimate,
        )
    );

    println!(
        "dobry gen sg: {}",
        interslavic::adj(
            "dobry",
            Case::Gen,
            Number::Singular,
            Gender::Masculine,
            Animacy::Animate
        )
    );

    println!(
        "učiti 1sg present: {}",
        interslavic::verb(
            "učiti",
            Person::First,
            Number::Singular,
            Gender::Feminine,
            Tense::Present,
        )
    );
}
