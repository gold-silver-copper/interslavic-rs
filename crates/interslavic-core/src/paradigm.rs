//! Noun, adjective, and verb paradigm data structures and accessors.

use crate::case_endings::CaseEndings;
use crate::{Animacy, Case, Gender, Number};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VerbParadigm {
    pub infinitive: String,
    pub present: Vec<String>,
    pub imperfect: Vec<String>,
    pub perfect: Vec<String>,
    pub pluperfect: Vec<String>,
    pub future: Vec<String>,
    pub conditional: Vec<String>,
    pub imperative: Vec<String>,
    pub prap: Option<String>,
    pub prpp: Option<String>,
    pub pfap: String,
    pub pfpp: Option<String>,
    pub gerund: String,
}

/// The case order the paradigm structs store their per-number `Vec`s in.
pub const CASE_ORDER: [Case; 6] = [
    Case::Nom,
    Case::Acc,
    Case::Gen,
    Case::Loc,
    Case::Dat,
    Case::Ins,
];

fn case_index(case: Case) -> usize {
    match case {
        Case::Nom => 0,
        Case::Acc => 1,
        Case::Gen => 2,
        Case::Loc => 3,
        Case::Dat => 4,
        Case::Ins => 5,
    }
}

/// A full noun paradigm — the six cases in each number — for one lemma with a
/// fixed gender and animacy (the counterpart of [`VerbParadigm`] for nouns).
/// `singular` and `plural` each hold six forms in [`CASE_ORDER`]; index them
/// with [`NounParadigm::get`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NounParadigm {
    pub lemma: String,
    pub gender: Gender,
    pub animacy: Animacy,
    pub singular: Vec<String>,
    pub plural: Vec<String>,
}

impl NounParadigm {
    /// Return the form for one `(case, number)` cell.
    pub fn get(&self, case: Case, number: Number) -> &str {
        let cells = match number {
            Number::Singular => &self.singular,
            Number::Plural => &self.plural,
        };
        &cells[case_index(case)]
    }
}

/// A full adjective paradigm — six cases × two numbers × the four
/// gender/animacy columns (masculine animate, masculine inanimate, feminine,
/// neuter; feminine and neuter are animacy-invariant). Each column is
/// `[singular, plural]`, each a six-form `Vec` in [`CASE_ORDER`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AdjParadigm {
    pub lemma: String,
    pub masculine_animate: [Vec<String>; 2],
    pub masculine_inanimate: [Vec<String>; 2],
    pub feminine: [Vec<String>; 2],
    pub neuter: [Vec<String>; 2],
}

impl AdjParadigm {
    /// Return the form for one `(case, number, gender, animacy)` cell.
    /// Feminine and neuter ignore `animacy`.
    pub fn get(&self, case: Case, number: Number, gender: Gender, animacy: Animacy) -> &str {
        let column = match (gender, animacy) {
            (Gender::Masculine, Animacy::Animate) => &self.masculine_animate,
            (Gender::Masculine, Animacy::Inanimate) => &self.masculine_inanimate,
            (Gender::Feminine, _) => &self.feminine,
            (Gender::Neuter, _) => &self.neuter,
        };
        let number_index = match number {
            Number::Singular => 0,
            Number::Plural => 1,
        };
        &column[number_index][case_index(case)]
    }
}

impl CaseEndings {
    pub(crate) fn ending(&self, case: Case, number: Number) -> &str {
        match number {
            Number::Singular => match case {
                Case::Nom => self.nom_sg,
                Case::Acc => self.acc_sg,
                Case::Gen => self.gen_sg,
                Case::Loc => self.loc_sg,
                Case::Dat => self.dat_sg,
                Case::Ins => self.ins_sg,
            },
            Number::Plural => match case {
                Case::Nom => self.nom_pl,
                Case::Acc => self.acc_pl,
                Case::Gen => self.gen_pl,
                Case::Loc => self.loc_pl,
                Case::Dat => self.dat_pl,
                Case::Ins => self.ins_pl,
            },
        }
    }
}
