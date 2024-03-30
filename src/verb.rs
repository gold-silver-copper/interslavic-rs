use crate::basic::*;
use crate::enums::*;

#[derive(Debug)]
pub struct Verb {
    pub infinitive: String,
    pub present_tense_stem: String,
    pub perfect: bool,
    pub transitive: bool,
     pub conjugation: Conjugation,
}

impl Verb {
    pub fn new(word: &str, perf: bool, trans: bool, optional_stem : Option<String>) -> Self {

        let (stem,conj) = Verb::get_present_stem_and_conjugation(word, optional_stem);
        Verb {
            infinitive: word.into(),
            present_tense_stem: stem,
            perfect: perf,
            transitive: trans,
             conjugation: conj,
        }
    }

    pub fn get_present_stem_and_conjugation(infinitive: &str , optional_stem : Option<String>) -> (String, Conjugation) {
        let mut ti_removed = slice_without_last(&slice_without_last(infinitive));


        if let Some(opt_stem) = optional_stem {
           
            if opt_stem.ends_with("me") {return (replace_last_occurence(&opt_stem, "me", "m"), Conjugation::First)}
            if opt_stem.ends_with("ne") {return (replace_last_occurence(&opt_stem, "ne", "n"), Conjugation::First)}

            if opt_stem.ends_with("je") {return (replace_last_occurence(&opt_stem, "je", "j"), Conjugation::First)}
            if opt_stem.ends_with("ju") {return (replace_last_occurence(&opt_stem, "ju", "j"), Conjugation::First)}
            if opt_stem.ends_with("e") {return (replace_last_occurence(&opt_stem, "e", ""), Conjugation::First)}
            if opt_stem.ends_with("i") {return (opt_stem, Conjugation::Second)}
        }

        if is_consonant(last_in_slice(&ti_removed)) {
            (ti_removed, Conjugation::First)
        } else if infinitive.ends_with("ovati") {
            (infinitive.replace("ovati", "uj"), Conjugation::First)
        } else if infinitive.ends_with("nųti") {
            (infinitive.replace("nųti", "n"), Conjugation::First)
        } else if infinitive.ends_with("ati") {
            (replace_last_occurence(infinitive, "ati", "aj"), Conjugation::First)
        } else if infinitive.ends_with("eti") {
            (replace_last_occurence(infinitive, "eti", "ej"), Conjugation::First)
        } 
        else if infinitive.ends_with("ęti") {
            (replace_last_occurence(infinitive, "ęti", "n"), Conjugation::First)
        } 
        else if infinitive.ends_with("yti") {
            (replace_last_occurence(infinitive, "yti", "yj"), Conjugation::First)
        }
        else if infinitive.ends_with("uti") {
            (replace_last_occurence(infinitive, "uti", "uj"), Conjugation::First)
        }
        else if infinitive.ends_with("iti") {
            (replace_last_occurence(infinitive, "iti", "i"), Conjugation::Second)
        }
        else if infinitive.ends_with("ěti") {
            (replace_last_occurence(infinitive, "ěti", "i"), Conjugation::Second)
        }
       
        else {
            panic!("IMPROPER PERSENT TENSE STEM: {}", infinitive);
            
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
