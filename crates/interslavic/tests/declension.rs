use interslavic::*;

fn noun(word: &str, case: Case, number: Number) -> String {
    interslavic::noun(word, case, number)
}

#[test]
fn noun_declension_matches_sonic16x_common_paradigms() {
    assert_eq!(noun("suma", Case::Acc, Number::Singular), "sumų");
    assert_eq!(noun("suma", Case::Gen, Number::Plural), "sum");
    assert_eq!(noun("maj", Case::Ins, Number::Singular), "majem");
    assert_eq!(noun("maj", Case::Nom, Number::Plural), "maje");
    assert_eq!(noun("mųž", Case::Ins, Number::Singular), "mųžem");
    assert_eq!(noun("mųž", Case::Gen, Number::Plural), "mųžev");
    assert_eq!(noun("morje", Case::Gen, Number::Plural), "morej");
    assert_eq!(noun("pismo", Case::Gen, Number::Plural), "pism");
}

#[test]
fn noun_declension_handles_irregular_and_soft_classes() {
    assert_eq!(noun("oko", Case::Nom, Number::Plural), "oči / očesa");
    assert_eq!(noun("oko", Case::Gen, Number::Singular), "oka / očese");
    assert_eq!(noun("oko", Case::Dat, Number::Plural), "očam / očesam");
    assert_eq!(noun("oko", Case::Ins, Number::Plural), "očami / očesami");
    assert_eq!(noun("oko", Case::Loc, Number::Plural), "očah / očesah");
    assert_eq!(noun("kamen", Case::Acc, Number::Singular), "kamen / kamenj");
    assert_eq!(
        noun("kamen", Case::Gen, Number::Singular),
        "kamene / kamenja"
    );
    assert_eq!(noun("den", Case::Acc, Number::Singular), "den / denj");
    assert_eq!(noun("den", Case::Gen, Number::Singular), "dene / denja");
    assert_eq!(noun("kost", Case::Ins, Number::Singular), "kostjų");
    assert_eq!(noun("kost", Case::Gen, Number::Plural), "kostij");
    assert_eq!(noun("zemja", Case::Gen, Number::Plural), "zemej");
    assert_eq!(noun("slovo", Case::Gen, Number::Plural), "slov / sloves");
    assert_eq!(noun("okno", Case::Gen, Number::Plural), "okȯn");
    assert_eq!(noun("matka", Case::Gen, Number::Plural), "matȯk");
    assert_eq!(noun("slåvėj", Case::Gen, Number::Singular), "slåvėja");
    assert_eq!(noun("žȯlv", Case::Ins, Number::Singular), "žȯlvjų");
}

#[test]
fn adjective_declension_uses_sonic16x_nasal_endings() {
    assert_eq!(
        interslavic::adj(
            "babin",
            Case::Nom,
            Number::Singular,
            Gender::Masculine,
            Animacy::Inanimate
        ),
        "babin"
    );
    assert_eq!(
        interslavic::adj(
            "babin",
            Case::Gen,
            Number::Singular,
            Gender::Masculine,
            Animacy::Animate
        ),
        "babinogo"
    );
    assert_eq!(
        interslavic::adj(
            "dȯlžėn",
            Case::Gen,
            Number::Singular,
            Gender::Masculine,
            Animacy::Animate
        ),
        "dȯlžnogo"
    );
    assert_eq!(
        interslavic::adj(
            "samy",
            Case::Acc,
            Number::Singular,
            Gender::Feminine,
            Animacy::Inanimate
        ),
        "samų"
    );
    assert_eq!(
        interslavic::adj(
            "samy",
            Case::Ins,
            Number::Singular,
            Gender::Feminine,
            Animacy::Inanimate
        ),
        "samojų"
    );
    assert_eq!(
        interslavic::adj(
            "sinji",
            Case::Ins,
            Number::Singular,
            Gender::Feminine,
            Animacy::Inanimate
        ),
        "sinjejų"
    );
}
