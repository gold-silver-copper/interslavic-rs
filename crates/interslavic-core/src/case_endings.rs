use crate::grammar::{Case, Number};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct CaseEndings {
    pub nom_sg: &'static str,
    pub acc_sg: &'static str,
    pub gen_sg: &'static str,
    pub loc_sg: &'static str,
    pub dat_sg: &'static str,
    pub ins_sg: &'static str,
    pub voc_sg: &'static str,
    pub nom_pl: &'static str,
    pub acc_pl: &'static str,
    pub gen_pl: &'static str,
    pub loc_pl: &'static str,
    pub dat_pl: &'static str,
    pub ins_pl: &'static str,
    pub voc_pl: &'static str,
}

impl CaseEndings {
    pub fn ending(&self, case: Case, number: Number) -> &'static str {
        match number {
            Number::Singular => match case {
                Case::Nom => self.nom_sg,
                Case::Acc => self.acc_sg,
                Case::Gen => self.gen_sg,
                Case::Loc => self.loc_sg,
                Case::Dat => self.dat_sg,
                Case::Ins => self.ins_sg,
                Case::Voc => self.voc_sg,
            },
            Number::Plural => match case {
                Case::Nom => self.nom_pl,
                Case::Acc => self.acc_pl,
                Case::Gen => self.gen_pl,
                Case::Loc => self.loc_pl,
                Case::Dat => self.dat_pl,
                Case::Ins => self.ins_pl,
                Case::Voc => self.voc_pl,
            },
        }
    }
}

macro_rules! endings {
    ($nom_sg:expr, $acc_sg:expr, $gen_sg:expr, $loc_sg:expr, $dat_sg:expr, $ins_sg:expr,
     $nom_pl:expr, $acc_pl:expr, $gen_pl:expr, $loc_pl:expr, $dat_pl:expr, $ins_pl:expr) => {
        CaseEndings {
            nom_sg: $nom_sg,
            acc_sg: $acc_sg,
            gen_sg: $gen_sg,
            loc_sg: $loc_sg,
            dat_sg: $dat_sg,
            ins_sg: $ins_sg,
            voc_sg: $nom_sg,
            nom_pl: $nom_pl,
            acc_pl: $acc_pl,
            gen_pl: $gen_pl,
            loc_pl: $loc_pl,
            dat_pl: $dat_pl,
            ins_pl: $ins_pl,
            voc_pl: $nom_pl,
        }
    };
}

pub const ANIMATE_HARD_ENDINGS: CaseEndings =
    endings!("", "a", "a", "u", "u", "om", "i", "ov", "ov", "ah", "am", "ami");
pub const ANIMATE_SOFT_ENDINGS: CaseEndings =
    endings!("", "a", "a", "u", "u", "em", "i", "ev", "ev", "ah", "am", "ami");
pub const INANIMATE_HARD_ENDINGS: CaseEndings =
    endings!("", "", "a", "u", "u", "om", "y", "y", "ov", "ah", "am", "ami");
pub const INANIMATE_SOFT_ENDINGS: CaseEndings =
    endings!("", "", "a", "u", "u", "em", "e", "e", "ev", "ah", "am", "ami");
pub const NEUTER_HARD_ENDINGS: CaseEndings =
    endings!("o", "o", "a", "u", "u", "om", "a", "a", "", "ah", "am", "ami");
pub const NEUTER_SOFT_ENDINGS: CaseEndings =
    endings!("e", "e", "a", "u", "u", "em", "a", "a", "", "ah", "am", "ami");
pub const FEMININE_HARD_ENDINGS: CaseEndings =
    endings!("a", "u", "y", "ě", "ě", "oju", "y", "y", "", "ah", "am", "ami");
pub const FEMININE_SOFT_ENDINGS: CaseEndings =
    endings!("a", "u", "e", "i", "i", "eju", "e", "e", "", "ah", "am", "ami");
pub const OST_ENDINGS: CaseEndings =
    endings!("", "", "i", "i", "i", "ju", "i", "i", "ij", "ah", "am", "ami");

pub const ADJ_ANIMATE_HARD_ENDINGS: CaseEndings =
    endings!("y", "ogo", "ogo", "om", "omu", "ym", "i", "yh", "yh", "yh", "ym", "ymi");
pub const ADJ_INANIMATE_HARD_ENDINGS: CaseEndings =
    endings!("y", "y", "ogo", "om", "omu", "ym", "e", "e", "yh", "yh", "ym", "ymi");
pub const ADJ_NEUTER_HARD_ENDINGS: CaseEndings =
    endings!("o", "o", "ogo", "om", "omu", "ym", "e", "e", "yh", "yh", "ym", "ymi");
pub const ADJ_FEMININE_HARD_ENDINGS: CaseEndings =
    endings!("a", "u", "oj", "oj", "oj", "oju", "e", "e", "yh", "yh", "ym", "ymi");
pub const ADJ_ANIMATE_SOFT_ENDINGS: CaseEndings =
    endings!("i", "ego", "ego", "em", "emu", "im", "i", "ih", "ih", "ih", "im", "imi");
pub const ADJ_INANIMATE_SOFT_ENDINGS: CaseEndings =
    endings!("i", "i", "ego", "em", "emu", "im", "e", "e", "ih", "ih", "im", "imi");
pub const ADJ_NEUTER_SOFT_ENDINGS: CaseEndings =
    endings!("e", "e", "ego", "em", "emu", "im", "e", "e", "ih", "ih", "im", "imi");
pub const ADJ_FEMININE_SOFT_ENDINGS: CaseEndings =
    endings!("a", "u", "ej", "ej", "ej", "eju", "e", "e", "ih", "ih", "im", "imi");
