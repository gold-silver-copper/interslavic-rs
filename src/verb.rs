use crate::basic::*;
use crate::enums::*;

#[derive(Debug)]
pub struct Verb {
    pub infinitive: String,
    pub verb_stem: String,
    pub perfect: bool,
    pub transitive: bool,
    //  pub conjugation: Conjugation,
}

impl Verb {
    pub fn new(word: &str, perf: bool, trans: bool) -> Self {
        Verb {
            infinitive: word.into(),
            verb_stem: slice_without_last(&slice_without_last(word)),
            perfect: perf,
            transitive: trans,
            // conjugation: Conjugation::First,
        }
    }

    pub fn get_verb_stem_and_conjugation(infinitive: &str) -> (String, Conjugation) {
        let mut ti_removed = slice_without_last(&slice_without_last(infinitive));

        if is_consonant(last_in_slice(&ti_removed)) {
            (ti_removed, Conjugation::First)
        } else if ti_removed.ends_with("ovati") {
            (ti_removed.replace("ovati", "uj"), Conjugation::First)
        } else {
            (ti_removed, Conjugation::First)
        }
    }

    pub fn derive_verb(&self, tense: VerbTense) -> String {
        match tense {
            VerbTense::Infinitive => self.infinitive(),
            VerbTense::Imperative(g) => self.infinitive(),
            VerbTense::Present(p, n) => self.infinitive(),
            VerbTense::Perfect(g, p, n) => self.infinitive(),
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
