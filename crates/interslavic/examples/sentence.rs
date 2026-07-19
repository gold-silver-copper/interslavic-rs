use interslavic::*;

#[derive(Clone, Copy)]
struct Subject {
    lemma: &'static str,
    number: Number,
    gender: Gender,
    animacy: Animacy,
}

fn subject_phrase(adjective: &str, subject: Subject) -> String {
    let adjective = interslavic::adj(
        adjective,
        Case::Nom,
        subject.number,
        subject.gender,
        subject.animacy,
    );
    let noun = interslavic::noun(subject.lemma, Case::Nom, subject.number);
    format!("{adjective} {noun}")
}

fn predicate(verb: &str, subject: Subject) -> String {
    interslavic::verb(
        verb,
        Person::Third,
        subject.number,
        subject.gender,
        Tense::Present,
    )
}

fn main() {
    let teacher = Subject {
        lemma: "učitelj",
        number: Number::Singular,
        gender: Gender::Masculine,
        animacy: Animacy::Animate,
    };

    let sentence = format!(
        "{} {}.",
        subject_phrase("dobry", teacher),
        predicate("pisati", teacher)
    );
    println!("{sentence}");

    assert_eq!(subject_phrase("dobry", teacher), "dobry učitelj");
    assert_eq!(predicate("pisati", teacher), "piše");
    assert_eq!(sentence, "dobry učitelj piše.");
}
