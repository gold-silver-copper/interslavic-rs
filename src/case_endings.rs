#[derive(Debug, PartialEq, Clone)]
pub struct CaseEndings {
    pub nom_sg: &'static str,
    pub acc_sg: &'static str,
    pub gen_sg: &'static str,
    pub loc_sg: &'static str,
    pub dat_sg: &'static str,
    pub ins_sg: &'static str,

    pub nom_pl: &'static str,
    pub acc_pl: &'static str,
    pub gen_pl: &'static str,
    pub loc_pl: &'static str,
    pub dat_pl: &'static str,
    pub ins_pl: &'static str,
}

pub const ANIMATE_HARD_ENDINGS: CaseEndings = CaseEndings {
    nom_sg: "",   // Nominative Singular
    acc_sg: "a",  // Accusative Singular
    gen_sg: "a",  // Genitive Singular
    dat_sg: "u",  // Dative Singular
    ins_sg: "om", // Instrumental Singular
    loc_sg: "u",  // Locative Singular

    nom_pl: "i",   // Nominative Plural
    acc_pl: "ov",  // Accusative Plural
    gen_pl: "ov",  // Genitive Plural
    dat_pl: "am",  // Dative Plural
    ins_pl: "ami", // Instrumental Plural
    loc_pl: "ah",  // Locative Plural
};

pub const ANIMATE_SOFT_ENDINGS: CaseEndings = CaseEndings {
    nom_sg: "",   // Nominative Singular
    acc_sg: "a",  // Accusative Singular
    gen_sg: "a",  // Genitive Singular
    dat_sg: "u",  // Dative Singular
    ins_sg: "em", // Instrumental Singular
    loc_sg: "u",  // Locative Singular

    nom_pl: "i",   // Nominative Plural
    acc_pl: "ev",  // Accusative Plural
    gen_pl: "ev",  // Genitive Plural
    dat_pl: "am",  // Dative Plural
    ins_pl: "ami", // Instrumental Plural
    loc_pl: "ah",  // Locative Plural
};
pub const INANIMATE_HARD_ENDINGS: CaseEndings = CaseEndings {
    nom_sg: "",   // Nominative Singular
    acc_sg: "",   // Accusative Singular
    gen_sg: "a",  // Genitive Singular
    dat_sg: "u",  // Dative Singular
    ins_sg: "om", // Instrumental Singular
    loc_sg: "u",  // Locative Singular

    nom_pl: "y",   // Nominative Plural
    acc_pl: "y",   // Accusative Plural
    gen_pl: "ov",  // Genitive Plural
    dat_pl: "am",  // Dative Plural
    ins_pl: "ami", // Instrumental Plural
    loc_pl: "ah",  // Locative Plural
};
pub const INANIMATE_SOFT_ENDINGS: CaseEndings = CaseEndings {
    nom_sg: "",   // Nominative Singular
    acc_sg: "",   // Accusative Singular
    gen_sg: "a",  // Genitive Singular
    dat_sg: "u",  // Dative Singular
    ins_sg: "em", // Instrumental Singular
    loc_sg: "u",  // Locative Singular

    nom_pl: "e",   // Nominative Plural
    acc_pl: "e",   // Accusative Plural
    gen_pl: "ev",  // Genitive Plural
    dat_pl: "am",  // Dative Plural
    ins_pl: "ami", // Instrumental Plural
    loc_pl: "ah",  // Locative Plural
};
pub const NEUTER_HARD_ENDINGS: CaseEndings = CaseEndings {
    nom_sg: "o",  // Nominative Singular
    acc_sg: "o",  // Accusative Singular
    gen_sg: "a",  // Genitive Singular
    dat_sg: "u",  // Dative Singular
    ins_sg: "om", // Instrumental Singular
    loc_sg: "u",  // Locative Singular

    nom_pl: "a",   // Nominative Plural
    acc_pl: "a",   // Accusative Plural
    gen_pl: "",    // Genitive Plural
    dat_pl: "am",  // Dative Plural
    ins_pl: "ami", // Instrumental Plural
    loc_pl: "ah",  // Locative Plural
};
pub const NEUTER_SOFT_ENDINGS: CaseEndings = CaseEndings {
    nom_sg: "e",  // Nominative Singular
    acc_sg: "e",  // Accusative Singular
    gen_sg: "a",  // Genitive Singular
    dat_sg: "u",  // Dative Singular
    ins_sg: "em", // Instrumental Singular
    loc_sg: "u",  // Locative Singular

    nom_pl: "a",   // Nominative Plural
    acc_pl: "a",   // Accusative Plural
    gen_pl: "",    // Genitive Plural
    dat_pl: "am",  // Dative Plural
    ins_pl: "ami", // Instrumental Plural
    loc_pl: "ah",  // Locative Plural
};
pub const FEMININE_HARD_ENDINGS: CaseEndings = CaseEndings {
    nom_sg: "a",   // Nominative Singular
    acc_sg: "u",   // Accusative Singular
    gen_sg: "y",   // Genitive Singular
    dat_sg: "ě",   // Dative Singular
    ins_sg: "oju", // Instrumental Singular
    loc_sg: "ě",   // Locative Singular

    nom_pl: "y",   // Nominative Plural
    acc_pl: "y",   // Accusative Plural
    gen_pl: "",    // Genitive Plural
    dat_pl: "am",  // Dative Plural
    ins_pl: "ami", // Instrumental Plural
    loc_pl: "ah",  // Locative Plural
};
pub const FEMININE_SOFT_ENDINGS: CaseEndings = CaseEndings {
    nom_sg: "a",   // Nominative Singular
    acc_sg: "u",   // Accusative Singular
    gen_sg: "e",   // Genitive Singular
    dat_sg: "i",   // Dative Singular
    ins_sg: "eju", // Instrumental Singular
    loc_sg: "i",   // Locative Singular

    nom_pl: "e",   // Nominative Plural
    acc_pl: "e",   // Accusative Plural
    gen_pl: "",    // Genitive Plural
    dat_pl: "am",  // Dative Plural
    ins_pl: "ami", // Instrumental Plural
    loc_pl: "ah",  // Locative Plural
};

pub const OST_ENDINGS: CaseEndings = CaseEndings {
    nom_sg: "",   // Nominative Singular
    acc_sg: "",   // Accusative Singular
    gen_sg: "i",  // Genitive Singular
    dat_sg: "i",  // Dative Singular
    ins_sg: "ju", // Instrumental Singular
    loc_sg: "i",  // Locative Singular

    nom_pl: "i",   // Nominative Plural
    acc_pl: "i",   // Accusative Plural
    gen_pl: "ij",  // Genitive Plural
    dat_pl: "am",  // Dative Plural
    ins_pl: "ami", // Instrumental Plural
    loc_pl: "ah",  // Locative Plural
};

pub const ADJ_ANIMATE_HARD_ENDINGS: CaseEndings = CaseEndings {
    nom_sg: "y",   // Nominative Singular
    acc_sg: "ogo", // Accusative Singular
    gen_sg: "ogo", // Genitive Singular
    dat_sg: "omu", // Dative Singular
    ins_sg: "ym",  // Instrumental Singular
    loc_sg: "om",  // Locative Singular

    nom_pl: "i",   // Nominative Plural
    acc_pl: "yh",  // Accusative Plural
    gen_pl: "yh",  // Genitive Plural
    dat_pl: "ym",  // Dative Plural
    ins_pl: "ymi", // Instrumental Plural
    loc_pl: "yh",  // Locative Plural
};

pub const ADJ_INANIMATE_HARD_ENDINGS: CaseEndings = CaseEndings {
    nom_sg: "y",   // Nominative Singular
    acc_sg: "y",   // Accusative Singular
    gen_sg: "ogo", // Genitive Singular
    dat_sg: "omu", // Dative Singular
    ins_sg: "ym",  // Instrumental Singular
    loc_sg: "om",  // Locative Singular

    nom_pl: "e",   // Nominative Plural
    acc_pl: "e",   // Accusative Plural
    gen_pl: "yh",  // Genitive Plural
    dat_pl: "ym",  // Dative Plural
    ins_pl: "ymi", // Instrumental Plural
    loc_pl: "yh",  // Locative Plural
};
pub const ADJ_NEUTER_HARD_ENDINGS: CaseEndings = CaseEndings {
    nom_sg: "o",   // Nominative Singular
    acc_sg: "o",   // Accusative Singular
    gen_sg: "ogo", // Genitive Singular
    dat_sg: "omu", // Dative Singular
    ins_sg: "ym",  // Instrumental Singular
    loc_sg: "om",  // Locative Singular

    nom_pl: "e",   // Nominative Plural
    acc_pl: "e",   // Accusative Plural
    gen_pl: "yh",  // Genitive Plural
    dat_pl: "ym",  // Dative Plural
    ins_pl: "ymi", // Instrumental Plural
    loc_pl: "yh",  // Locative Plural
};
pub const ADJ_FEMININE_HARD_ENDINGS: CaseEndings = CaseEndings {
    nom_sg: "a",   // Nominative Singular
    acc_sg: "u",   // Accusative Singular
    gen_sg: "oj",  // Genitive Singular
    dat_sg: "oj",  // Dative Singular
    ins_sg: "oju", // Instrumental Singular
    loc_sg: "oj",  // Locative Singular

    nom_pl: "e",   // Nominative Plural
    acc_pl: "e",   // Accusative Plural
    gen_pl: "yh",  // Genitive Plural
    dat_pl: "ym",  // Dative Plural
    ins_pl: "ymi", // Instrumental Plural
    loc_pl: "yh",  // Locative Plural
};
pub const ADJ_ANIMATE_SOFT_ENDINGS: CaseEndings = CaseEndings {
    nom_sg: "i",   // Nominative Singular
    acc_sg: "ego", // Accusative Singular
    gen_sg: "ego", // Genitive Singular
    dat_sg: "emu", // Dative Singular
    ins_sg: "im",  // Instrumental Singular
    loc_sg: "em",  // Locative Singular

    nom_pl: "i",   // Nominative Plural
    acc_pl: "ih",  // Accusative Plural
    gen_pl: "ih",  // Genitive Plural
    dat_pl: "im",  // Dative Plural
    ins_pl: "imi", // Instrumental Plural
    loc_pl: "ih",  // Locative Plural
};
pub const ADJ_INANIMATE_SOFT_ENDINGS: CaseEndings = CaseEndings {
    nom_sg: "i",   // Nominative Singular
    acc_sg: "i",   // Accusative Singular
    gen_sg: "ego", // Genitive Singular
    dat_sg: "emu", // Dative Singular
    ins_sg: "im",  // Instrumental Singular
    loc_sg: "em",  // Locative Singular

    nom_pl: "e",   // Nominative Plural
    acc_pl: "e",   // Accusative Plural
    gen_pl: "ih",  // Genitive Plural
    dat_pl: "im",  // Dative Plural
    ins_pl: "imi", // Instrumental Plural
    loc_pl: "ih",  // Locative Plural
};
pub const ADJ_NEUTER_SOFT_ENDINGS: CaseEndings = CaseEndings {
    nom_sg: "e",   // Nominative Singular
    acc_sg: "e",   // Accusative Singular
    gen_sg: "ego", // Genitive Singular
    dat_sg: "emu", // Dative Singular
    ins_sg: "im",  // Instrumental Singular
    loc_sg: "em",  // Locative Singular

    nom_pl: "e",   // Nominative Plural
    acc_pl: "e",   // Accusative Plural
    gen_pl: "ih",  // Genitive Plural
    dat_pl: "im",  // Dative Plural
    ins_pl: "imi", // Instrumental Plural
    loc_pl: "ih",  // Locative Plural
};
pub const ADJ_FEMININE_SOFT_ENDINGS: CaseEndings = CaseEndings {
    nom_sg: "a",   // Nominative Singular
    acc_sg: "u",   // Accusative Singular
    gen_sg: "ej",  // Genitive Singular
    dat_sg: "ej",  // Dative Singular
    ins_sg: "eju", // Instrumental Singular
    loc_sg: "ej",  // Locative Singular

    nom_pl: "e",   // Nominative Plural
    acc_pl: "e",   // Accusative Plural
    gen_pl: "ih",  // Genitive Plural
    dat_pl: "im",  // Dative Plural
    ins_pl: "imi", // Instrumental Plural
    loc_pl: "ih",  // Locative Plural
};
