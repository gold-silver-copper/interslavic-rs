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

/// Which of the three personal-pronoun form series to return: the full
/// (stressed) form, the clitic (short, unstressed) form where one is
/// attested, or the form to use after a preposition (the 3rd-person
/// prepositional n- variant — `njego`, `njim` — falling back to the full
/// form where no n- variant exists).
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum PronounStyle {
    Full,
    Clitic,
    AfterPreposition,
}
