use interslavic::*;

fn main() {
    println!("Nouns");
    println!(
        "  adept acc.sg = {}",
        interslavic::noun("adept", Case::Acc, Number::Singular)
    );
    println!(
        "  oko nom.pl = {}",
        interslavic::noun("oko", Case::Nom, Number::Plural)
    );

    assert_eq!(
        interslavic::noun("adept", Case::Acc, Number::Singular),
        "adepta"
    );
    assert_eq!(
        interslavic::noun("oko", Case::Nom, Number::Plural),
        "oči / očesa"
    );

    let adjective = interslavic::adj(
        "dobry",
        Case::Gen,
        Number::Singular,
        Gender::Masculine,
        Animacy::Animate,
    );
    println!("Adjective");
    println!("  dobry gen.sg masculine animate = {adjective}");
    assert_eq!(adjective, "dobrogo");

    let verb = interslavic::verb(
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
