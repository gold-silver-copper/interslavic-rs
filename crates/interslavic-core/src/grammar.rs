#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Number {
    Singular,
    Plural,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Case {
    Nom,
    Acc,
    Gen,
    Loc,
    Dat,
    Ins,
    Voc,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Gender {
    Masculine,
    Feminine,
    Neuter,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Animacy {
    Animate,
    Inanimate,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Person {
    First,
    Second,
    Third,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Conjugation {
    First,
    Second,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Tense {
    Present,
    Imperfect,
    Future,
    Perfect,
    PluPerfect,
    Conditional,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Aspect {
    Imperfective,
    Perfective,
    Biaspectual,
    Unknown,
}

impl From<bool> for Animacy {
    fn from(value: bool) -> Self {
        if value {
            Self::Animate
        } else {
            Self::Inanimate
        }
    }
}
