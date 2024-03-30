#[derive(Debug, PartialEq, Clone)]
pub struct CaseForms {
    pub nom: String, // Nominative
    pub gen: String, // Genitive
    pub dat: String, // Dative
    pub acc: String, // Accusative
    pub ins: String, // Instrumental
    pub loc: String, // Locative
    pub voc: String, // Vocative
}

impl CaseForms {
    pub fn neuter_hard_singular(word_stem: &str) -> Self {
        CaseForms {
            nom: format!("{word_stem}o"),
            acc: format!("{word_stem}o"),
            gen: format!("{word_stem}a"),
            dat: format!("{word_stem}u"),

            ins: format!("{word_stem}om"),
            loc: format!("{word_stem}u"),
            voc: format!("{word_stem}o"),
        }
    }
    pub fn neuter_hard_plural(word_stem: &str) -> Self {
        CaseForms {
            nom: format!("{word_stem}a"),
            acc: format!("{word_stem}a"),
            gen: format!("{word_stem}"),
            dat: format!("{word_stem}am"),

            ins: format!("{word_stem}ami"),
            loc: format!("{word_stem}ah"),
            voc: format!("{word_stem}a"),
        }
    }
    pub fn neuter_soft_singular(word_stem: &str) -> Self {
        CaseForms {
            nom: format!("{word_stem}e"),
            acc: format!("{word_stem}e"),
            gen: format!("{word_stem}ja"),
            dat: format!("{word_stem}u"),

            ins: format!("{word_stem}em"),
            loc: format!("{word_stem}u"),
            voc: format!("{word_stem}e"),
        }
    }
    pub fn neuter_soft_plural(word_stem: &str) -> Self {
        CaseForms {
            nom: format!("{word_stem}a"),
            acc: format!("{word_stem}a"),
            gen: format!("{word_stem}ej"),
            dat: format!("{word_stem}am"),

            ins: format!("{word_stem}ami"),
            loc: format!("{word_stem}ah"),
            voc: format!("{word_stem}a"),
        }
    }
    pub fn neuter_en_singular(word_stem: &str) -> Self {
        CaseForms {
            nom: format!("{word_stem}ę"),
            acc: format!("{word_stem}ę"),
            gen: format!("{word_stem}ene"),
            dat: format!("{word_stem}eni"),

            ins: format!("{word_stem}enem"),
            loc: format!("{word_stem}eni"),
            voc: format!("{word_stem}ę"),
        }
    }
    pub fn neuter_en_plural(word_stem: &str) -> Self {
        CaseForms {
            nom: format!("{word_stem}ena"),
            acc: format!("{word_stem}ena"),
            gen: format!("{word_stem}en"),
            dat: format!("{word_stem}enam"),

            ins: format!("{word_stem}enami"),
            loc: format!("{word_stem}enah"),
            voc: format!("{word_stem}ena"),
        }
    }

    pub fn feminine_hard_singular(word_stem: &str) -> Self {
        CaseForms {
            nom: format!("{word_stem}a"),
            acc: format!("{word_stem}u"),
            gen: format!("{word_stem}y"),
            dat: format!("{word_stem}ě"),

            ins: format!("{word_stem}oju"),
            loc: format!("{word_stem}ě"),
            voc: format!("{word_stem}o"),
        }
    }
    pub fn feminine_hard_plural(word_stem: &str) -> Self {
        CaseForms {
            nom: format!("{word_stem}y"),
            acc: format!("{word_stem}y"),
            gen: format!("{word_stem}"),
            dat: format!("{word_stem}am"),

            ins: format!("{word_stem}ami"),
            loc: format!("{word_stem}ah"),
            voc: format!("{word_stem}y"),
        }
    }

    pub fn feminine_soft_singular(word_stem: &str) -> Self {
        CaseForms {
            nom: format!("{word_stem}a"),
            acc: format!("{word_stem}u"),
            gen: format!("{word_stem}je"),
            dat: format!("{word_stem}i"),

            ins: format!("{word_stem}eju"),
            loc: format!("{word_stem}i"),
            voc: format!("{word_stem}o"),
        }
    }
    pub fn feminine_soft_plural(word_stem: &str) -> Self {
        CaseForms {
            nom: format!("{word_stem}e"),
            acc: format!("{word_stem}e"),
            gen: format!("{word_stem}ej"),
            dat: format!("{word_stem}am"),

            ins: format!("{word_stem}ami"),
            loc: format!("{word_stem}ah"),
            voc: format!("{word_stem}e"),
        }
    }

    pub fn feminine_cons_singular(word_stem: &str) -> Self {
        CaseForms {
            nom: format!("{word_stem}"),
            acc: format!("{word_stem}"),
            gen: format!("{word_stem}i"),
            dat: format!("{word_stem}i"),

            ins: format!("{word_stem}ju"),
            loc: format!("{word_stem}i"),
            voc: format!("{word_stem}i"),
        }
    }
    pub fn feminine_cons_plural(word_stem: &str) -> Self {
        CaseForms {
            nom: format!("{word_stem}i"),
            acc: format!("{word_stem}i"),
            gen: format!("{word_stem}ij"),
            dat: format!("{word_stem}am"),

            ins: format!("{word_stem}ami"),
            loc: format!("{word_stem}ah"),
            voc: format!("{word_stem}i"),
        }
    }

    pub fn masculine_anim_hard_singular(word_stem: &str) -> Self {
        CaseForms {
            nom: format!("{word_stem}"),
            acc: format!("{word_stem}a"),
            gen: format!("{word_stem}a"),
            dat: format!("{word_stem}u"),

            ins: format!("{word_stem}om"),
            loc: format!("{word_stem}u"),
            voc: format!("{word_stem}e"),
        }
    }
    pub fn masculine_anim_hard_plural(word_stem: &str) -> Self {
        CaseForms {
            nom: format!("{word_stem}i"),
            acc: format!("{word_stem}ov"),
            gen: format!("{word_stem}ov"),
            dat: format!("{word_stem}am"),

            ins: format!("{word_stem}ami"),
            loc: format!("{word_stem}ah"),
            voc: format!("{word_stem}i"),
        }
    }

    pub fn masculine_anim_soft_singular(word_stem: &str) -> Self {
        CaseForms {
            nom: format!("{word_stem}"),
            acc: format!("{word_stem}a"),
            gen: format!("{word_stem}a"),
            dat: format!("{word_stem}u"),

            ins: format!("{word_stem}em"),
            loc: format!("{word_stem}u"),
            voc: format!("{word_stem}u"),
        }
    }
    pub fn masculine_anim_soft_plural(word_stem: &str) -> Self {
        CaseForms {
            nom: format!("{word_stem}i"),
            acc: format!("{word_stem}ev"),
            gen: format!("{word_stem}ev"),
            dat: format!("{word_stem}am"),

            ins: format!("{word_stem}ami"),
            loc: format!("{word_stem}ah"),
            voc: format!("{word_stem}i"),
        }
    }

    pub fn masculine_inanim_hard_singular(word_stem: &str) -> Self {
        CaseForms {
            nom: format!("{word_stem}"),
            acc: format!("{word_stem}"),
            gen: format!("{word_stem}a"),
            dat: format!("{word_stem}u"),

            ins: format!("{word_stem}om"),
            loc: format!("{word_stem}u"),
            voc: format!("{word_stem}e"),
        }
    }
    pub fn masculine_inanim_hard_plural(word_stem: &str) -> Self {
        CaseForms {
            nom: format!("{word_stem}y"),
            acc: format!("{word_stem}y"),
            gen: format!("{word_stem}ov"),
            dat: format!("{word_stem}am"),

            ins: format!("{word_stem}ami"),
            loc: format!("{word_stem}ah"),
            voc: format!("{word_stem}y"),
        }
    }

    pub fn masculine_inanim_soft_singular(word_stem: &str) -> Self {
        CaseForms {
            nom: format!("{word_stem}"),
            acc: format!("{word_stem}"),
            gen: format!("{word_stem}a"),
            dat: format!("{word_stem}u"),

            ins: format!("{word_stem}em"),
            loc: format!("{word_stem}u"),
            voc: format!("{word_stem}u"),
        }
    }
    pub fn masculine_inanim_soft_plural(word_stem: &str) -> Self {
        CaseForms {
            nom: format!("{word_stem}e"),
            acc: format!("{word_stem}e"),
            gen: format!("{word_stem}ev"),
            dat: format!("{word_stem}am"),

            ins: format!("{word_stem}ami"),
            loc: format!("{word_stem}ah"),
            voc: format!("{word_stem}e"),
        }
    }

    pub fn indeclineable(word_stem: &str) -> Self {
        CaseForms {
            nom: format!("{word_stem}"),
            acc: format!("{word_stem}"),
            gen: format!("{word_stem}"),
            dat: format!("{word_stem}"),

            ins: format!("{word_stem}"),
            loc: format!("{word_stem}"),
            voc: format!("{word_stem}"),
        }
    }
}
