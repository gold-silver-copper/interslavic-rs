#[derive(Debug, PartialEq, Clone)]
pub struct VerbEndings {
    pub first_singular: &'static str,
    pub second_singular: &'static str,
    pub third_singular: &'static str,
    pub first_plural: &'static str,
    pub second_plural: &'static str,
    pub third_plural: &'static str,
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
    first_singular: "jų",
    second_singular: "iš",
    third_singular: "i",
    first_plural: "imo",
    second_plural: "ite",
    third_plural: "ęt",
};
