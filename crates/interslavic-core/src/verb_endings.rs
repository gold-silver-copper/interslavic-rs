use crate::grammar::{Number, Person};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct VerbEndings {
    pub first_singular: &'static str,
    pub second_singular: &'static str,
    pub third_singular: &'static str,
    pub first_plural: &'static str,
    pub second_plural: &'static str,
    pub third_plural: &'static str,
}

impl VerbEndings {
    pub fn ending(&self, person: Person, number: Number) -> &'static str {
        match (person, number) {
            (Person::First, Number::Singular) => self.first_singular,
            (Person::Second, Number::Singular) => self.second_singular,
            (Person::Third, Number::Singular) => self.third_singular,
            (Person::First, Number::Plural) => self.first_plural,
            (Person::Second, Number::Plural) => self.second_plural,
            (Person::Third, Number::Plural) => self.third_plural,
        }
    }
}

pub const FIRST_CONJUGATION: VerbEndings = VerbEndings {
    first_singular: "ų",
    second_singular: "eš",
    third_singular: "e",
    first_plural: "emo",
    second_plural: "ete",
    third_plural: "ųt",
};

pub const SECOND_CONJUGATION: VerbEndings = VerbEndings {
    first_singular: "ų",
    second_singular: "iš",
    third_singular: "i",
    first_plural: "imo",
    second_plural: "ite",
    third_plural: "ęt",
};
