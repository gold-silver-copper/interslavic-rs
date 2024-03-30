use crate::basic::*;
use crate::enums::*;

#[derive(Debug, PartialEq, Clone)]
pub struct Adjective {
    pub base_form: String,
}

impl Adjective {
    pub fn new(record: &ISVEntry) -> Self {
        let word = &record.isv;
        let boop = if word.ends_with("o") {
            replace_last_occurence(word, "o", "y")
        } else if word.ends_with("e") {
            replace_last_occurence(word, "e", "i")
        } else {
            word.to_string()
        };

        Adjective { base_form: boop }
    }

    pub fn basic_declension(word: &str, g: &Gender, c: &Case, n: &Number, animate: bool) -> String {
        let adj_stem = slice_without_last(word);
        let stem_is_soft = ends_with_soft_consonant(&adj_stem);

        let yi = if stem_is_soft { "i" } else { "y" };
        let oe = if stem_is_soft { "e" } else { "o" };

        let ending = match g {
            Gender::Feminine => match n {
                Number::Sing => match c {
                    Case::Nom => format!("a"),
                    Case::Acc => format!("u"),
                    Case::Gen => format!("{oe}j"),
                    Case::Dat => format!("{oe}j"),
                    Case::Ins => format!("{oe}ju"),
                    Case::Loc => format!("{oe}j"),
                    Case::Voc => format!("a"),
                },
                Number::Plur => match c {
                    Case::Nom => format!("e"),
                    Case::Acc => format!("e"),
                    Case::Gen => format!("{yi}h"),
                    Case::Dat => format!("{yi}m"),
                    Case::Ins => format!("{yi}mi"),
                    Case::Loc => format!("{yi}h"),
                    Case::Voc => format!("e"),
                },
            },
            Gender::Masculine => match n {
                Number::Sing => match c {
                    Case::Nom => format!("{yi}"),
                    Case::Acc => {
                        if animate {
                            format!("{oe}go")
                        } else {
                            format!("{yi}")
                        }
                    }
                    Case::Gen => format!("{oe}go"),
                    Case::Dat => format!("{oe}mu"),
                    Case::Ins => format!("{yi}m"),
                    Case::Loc => format!("{oe}m"),
                    Case::Voc => format!("{yi}"),
                },
                Number::Plur => match c {
                    Case::Nom => {
                        if animate {
                            format!("i")
                        } else {
                            format!("e")
                        }
                    }
                    Case::Acc => {
                        if animate {
                            format!("{yi}h")
                        } else {
                            format!("e")
                        }
                    }
                    Case::Gen => format!("{yi}h"),
                    Case::Dat => format!("{yi}m"),
                    Case::Ins => format!("{yi}mi"),
                    Case::Loc => format!("{yi}h"),
                    Case::Voc => {
                        if animate {
                            format!("i")
                        } else {
                            format!("e")
                        }
                    }
                },
            },
            Gender::Neuter => match n {
                Number::Sing => match c {
                    Case::Nom => format!("{oe}"),
                    Case::Acc => format!("{oe}"),
                    Case::Gen => format!("{oe}go"),
                    Case::Dat => format!("{oe}mu"),
                    Case::Ins => format!("{yi}m"),
                    Case::Loc => format!("{oe}m"),
                    Case::Voc => format!("{oe}"),
                },
                Number::Plur => match c {
                    Case::Nom => {
                        format!("e")
                    }
                    Case::Acc => {
                        format!("e")
                    }
                    Case::Gen => format!("{yi}h"),
                    Case::Dat => format!("{yi}m"),
                    Case::Ins => format!("{yi}mi"),
                    Case::Loc => format!("{yi}h"),
                    Case::Voc => {
                        format!("e")
                    }
                },
            },
        };

        format!("{adj_stem}{ending}")
    }

    pub fn decline(&self, g: &Gender, c: &Case, n: &Number, animate: bool) -> String {
        Adjective::basic_declension(&self.base_form, g, c, n, animate)
    }

    pub fn adverb(&self) -> String {
        Adjective::basic_declension(
            &self.base_form,
            &Gender::Neuter,
            &Case::Nom,
            &Number::Sing,
            true,
        )
    }
    pub fn basic_stem(&self) -> String {
        slice_without_last(&self.base_form)
    }

    pub fn synthetic_comparative(&self) -> String {
        let mut adj_stem = self.basic_stem();
        let is_soft = ends_with_soft_consonant(&adj_stem);
        let ending = if adj_stem.ends_with("ek") || adj_stem.ends_with("ok") {
            adj_stem = slice_without_last(&slice_without_last(&adj_stem));
            "ši"
        } else if adj_stem.ends_with("k")
            && is_consonant(last_in_slice(&slice_without_last(&adj_stem)))
        {
            adj_stem = slice_without_last(&adj_stem);
            "ši"
        } else if ends_with_soft_consonant(&self.basic_stem()) {
            "ejši"
        } else {
            "ějši"
        };

        format!("{adj_stem}{ending}")
    }

    pub fn synthetic_comparative_adverb(&self) -> String {
        let mut adj_stem = self.basic_stem();

        let sc = self.synthetic_comparative();

        let is_soft = ends_with_soft_consonant(&adj_stem);
        let ending = if adj_stem.ends_with("ek") || adj_stem.ends_with("ok") {
            adj_stem = slice_without_last(&slice_without_last(&adj_stem));
            "je"
        } else if adj_stem.ends_with("k")
            && is_consonant(last_in_slice(&slice_without_last(&adj_stem)))
        {
            adj_stem = slice_without_last(&adj_stem);
            "je"
        } else {
            "e"
        };

        iotation_merge(&adj_stem, ending)
    }

    pub fn superlative(&self) -> String {


        format!("naj{}",self.synthetic_comparative())
    }
    pub fn simple_superlative(&self) -> String {


        format!("naj{}",&self.base_form)
    }
    pub fn negative(&self) -> String {


        format!("ne{}",&self.base_form)
    }
}
