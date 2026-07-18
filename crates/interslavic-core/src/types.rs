//! Shared enums and basic public data types.

/// A noun phrase represented by its head and adjective modifiers.
#[derive(Debug, PartialEq, Clone)]
pub struct ComplexNoun {
    pub head_noun: String,
    pub adjective: Vec<String>,
}

impl Default for ComplexNoun {
    fn default() -> Self {
        Self {
            head_noun: "exemplum".into(),
            adjective: Vec::new(),
        }
    }
}

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
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Gender {
    Masculine,
    Feminine,
    Neuter,
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
    Pluperfect,
    Conditional,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Animacy {
    Animate,
    Inanimate,
}
