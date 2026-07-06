use interslavic::*;

fn main() {
    println!("Nouns");
    println!(
        "  adept acc.sg = {}",
        ISV::noun("adept", Case::Acc, Number::Singular)
    );
    println!(
        "  oko nom.pl = {}",
        ISV::noun("oko", Case::Nom, Number::Plural)
    );

    assert_eq!(ISV::noun("adept", Case::Acc, Number::Singular), "adepta");
    assert_eq!(ISV::noun("oko", Case::Nom, Number::Plural), "oči / očesa");

    let adjective = ISV::adj(
        "dobry",
        Case::Gen,
        Number::Singular,
        Gender::Masculine,
        Animacy::Animate,
    );
    println!("Adjective");
    println!("  dobry gen.sg masculine animate = {adjective}");
    assert_eq!(adjective, "dobrogo");

    let verb = ISV::verb(
        "učiti",
        Person::First,
        Number::Singular,
        Gender::Feminine,
        Tense::Present,
    );
    println!("Verb");
    println!("  učiti 1sg present = {verb}");
    assert_eq!(verb, "učų");
}
