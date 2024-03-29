use crate::basic::*;
use crate::enums::*;

#[derive(Debug)]
pub struct Verb {
    pub infinitive: String,
    pub verb_stem: String,
    pub perfect: bool,
    pub transitive: bool,
}

impl Verb {
    pub fn new(word: &str, perf: bool, trans: bool) -> Self {
        Verb {
            infinitive: word.into(),
            verb_stem: slice_without_last(&slice_without_last(word)),
            perfect: perf,
            transitive: trans,
        }
    }

    pub fn derive_verb(word: &str, tense: VerbTense) -> String {
        match tense {
            VerbTense::Infinitive => word.into(),
            VerbTense::Imperative(g) => word.into(),
            VerbTense::Present(p, n) => word.into(),
            VerbTense::Perfect(g, p, n) => word.into(),
        }
    }

    pub fn infinitive(&self) -> String {
        self.infinitive.clone()
    }

    pub fn imperative(&self, g: Gender) {}
    pub fn present(&self, p: Person, n: Number) {}
    pub fn perfect(&self, g: Gender, p: Person, n: Number) {}
}

pub struct TenseForms {
    pub sg_1: String,
    pub sg_2: String,
}
