use interslavic::*;

fn main() -> Result<(), InflectionError> {
    println!(
        "mųž gen sg: {}",
        ISV::noun_form("mųž", Case::Gen, Number::Singular)?.text()
    );
    println!(
        "Abhaz nom pl: {}",
        ISV::noun_form("Abhaz", Case::Nom, Number::Plural)?.text()
    );

    let člen_anim = ISV::noun_id("640")?;
    println!(
        "člen animate acc sg: {}",
        člen_anim.get(Case::Acc, Number::Singular).unwrap().text()
    );

    let luč_f = ISV::noun_id_as("339", NounGender::Feminine)?;
    println!(
        "luč feminine gen sg: {}",
        luč_f.get(Case::Gen, Number::Singular).unwrap().text()
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

    Ok(())
}
