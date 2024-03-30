use crate::basic::*;
use crate::enums::*;

#[derive(Debug, PartialEq, Clone)]
pub struct Verb {
    pub infinitive: String,
    pub infinitive_stem: String,
    pub present_tense_stem: String,
    pub perfect: bool,
    pub transitive: bool,
    pub conjugation: Conjugation,
}

impl Verb {
    pub fn new(record: &ISVEntry) -> Self {
        let trans = record.is_transitive();
        let intrans = record.is_intransitive();
        let perfect = record.is_perfect();
        let imperfect = record.is_imperfect();

        let optional_stem = record.get_addition_verb_stem();

        let (present_stem, conj) =
            Verb::get_present_stem_and_conjugation(&record.isv, optional_stem);
        let v = Verb {
            infinitive: record.isv.clone(),
            infinitive_stem: slice_without_last(&slice_without_last(&record.isv)),
            present_tense_stem: present_stem,
            perfect: perfect,
            transitive: trans,
            conjugation: conj,
        };
        let fem_g = Gender::Feminine;
        println!("{:#?}", &v.present(&Person::Second, &Number::Sing));
        println!("{:#?}", &v.l_participle(&fem_g, &Number::Sing));
        println!("{:#?}", &v.perfect(&fem_g, &Person::Third, &Number::Plur));
        println!("{:#?}", Verb::byti(VerbTense::LParticiple(Gender::Feminine,Number::Plur)));
        v
    }

    pub fn get_present_stem_and_conjugation(
        infinitive: &str,
        optional_stem: Option<String>,
    ) -> (String, Conjugation) {
        let mut ti_removed = slice_without_last(&slice_without_last(infinitive));

        if let Some(opt_stem) = optional_stem {
            if opt_stem.ends_with("me") {
                return (
                    replace_last_occurence(&opt_stem, "me", "m"),
                    Conjugation::First,
                );
            }
            if opt_stem.ends_with("ne") {
                return (
                    replace_last_occurence(&opt_stem, "ne", "n"),
                    Conjugation::First,
                );
            }

            if opt_stem.ends_with("je") {
                return (
                    replace_last_occurence(&opt_stem, "je", "j"),
                    Conjugation::First,
                );
            }
            if opt_stem.ends_with("ju") {
                return (
                    replace_last_occurence(&opt_stem, "ju", "j"),
                    Conjugation::First,
                );
            }
            if opt_stem.ends_with("e") {
                return (
                    replace_last_occurence(&opt_stem, "e", ""),
                    Conjugation::First,
                );
            }
            if opt_stem.ends_with("i") {
                return (opt_stem, Conjugation::Second);
            }
        }

        if is_consonant(last_in_slice(&ti_removed)) {
            (ti_removed, Conjugation::First)
        } else if infinitive.ends_with("ovati") {
            (infinitive.replace("ovati", "uj"), Conjugation::First)
        } else if infinitive.ends_with("nųti") {
            (infinitive.replace("nųti", "n"), Conjugation::First)
        } else if infinitive.ends_with("ati") {
            (
                replace_last_occurence(infinitive, "ati", "aj"),
                Conjugation::First,
            )
        } else if infinitive.ends_with("eti") {
            (
                replace_last_occurence(infinitive, "eti", "ej"),
                Conjugation::First,
            )
        } else if infinitive.ends_with("ęti") {
            (
                replace_last_occurence(infinitive, "ęti", "n"),
                Conjugation::First,
            )
        } else if infinitive.ends_with("yti") {
            (
                replace_last_occurence(infinitive, "yti", "yj"),
                Conjugation::First,
            )
        } else if infinitive.ends_with("uti") {
            (
                replace_last_occurence(infinitive, "uti", "uj"),
                Conjugation::First,
            )
        } else if infinitive.ends_with("iti") {
            (
                replace_last_occurence(infinitive, "iti", "i"),
                Conjugation::Second,
            )
        } else if infinitive.ends_with("ěti") {
            (
                replace_last_occurence(infinitive, "ěti", "i"),
                Conjugation::Second,
            )
        } else {
            panic!("IMPROPER PERSENT TENSE STEM: {}", infinitive);
        }
    }

    pub fn derive_verb(&self, tense: &VerbTense) -> String {
        match tense {
            VerbTense::Infinitive => self.infinitive(),
            VerbTense::Imperative(p,n) => self.infinitive(),
            VerbTense::Present(p, n) => self.infinitive(),
            VerbTense::Perfect(g, p, n) => self.infinitive(),
            _=>todo!()
        }
    }

    pub fn infinitive(&self) -> String {
        self.infinitive.clone()
    }

    pub fn l_participle(&self, g: &Gender, n: &Number) -> String {
        let stem = &self.infinitive_stem;

        match n {
            Number::Sing => match g {
                Gender::Feminine => format!("{stem}la"),
                Gender::Masculine => format!("{stem}l"),
                Gender::Neuter => format!("{stem}lo"),
            },
            Number::Plur => {
                format!("{stem}li")
            }
        }
    }

    pub fn perfect(&self, g: &Gender, p: &Person, n: &Number) -> String {
        let l_part = self.l_participle(&g, &n);
        let be = Verb::byti(VerbTense::Present(p.clone(), n.clone()));
        format!("{be} {l_part}")
    }
    pub fn future(&self, p: &Person, n: &Number) -> String {
        let infi = self.infinitive();
        let be = Verb::byti(VerbTense::Future(p.clone(), n.clone()));
        format!("{be} {infi}")
    }
    pub fn pluperfect(&self, g: &Gender, p: &Person, n: &Number) -> String {
        let l_part = self.l_participle(&g, &n);
        let be = Verb::byti(VerbTense::Imperfect(p.clone(), n.clone()));
        format!("{be} {l_part}")
    }
    pub fn conditional(&self, g: &Gender, p: &Person, n: &Number) -> String {
        let l_part = self.l_participle(&g, &n);
        let be = Verb::byti(VerbTense::Conditional(g.clone(),p.clone(), n.clone()));
        format!("{be} {l_part}")
    }
  

    pub fn byti(vt: VerbTense) -> String {
        match &vt {
            VerbTense::Infinitive => "byti".to_string(),
            VerbTense::Present(p, n) => match n {
                Number::Sing => match p {
                    Person::First => "jesm".to_string(),
                    Person::Second => "jesi".to_string(),
                    Person::Third => "je".to_string(),
                },
                Number::Plur => match p {
                    Person::First => "jesmo".to_string(),
                    Person::Second => "jeste".to_string(),
                    Person::Third => "sųt".to_string(),
                },
            },
            VerbTense::Imperfect(p,n, )=>  match n {
                Number::Sing => match p {
                    Person::First => "běh".to_string(),
                    Person::Second => "běše".to_string(),
                    Person::Third => "běše".to_string(),
                },
                Number::Plur => match p {
                    Person::First => "běhmo".to_string(),
                    Person::Second => "běste".to_string(),
                    Person::Third => "běhų".to_string(),
                },
            },
            VerbTense::Future(p,n, )=>  match n {
                Number::Sing => match p {
                    Person::First => "bųdų".to_string(),
                    Person::Second => "bųdeš".to_string(),
                    Person::Third => "bųde".to_string(),
                },
                Number::Plur => match p {
                    Person::First => "bųdemo".to_string(),
                    Person::Second => "bųdete".to_string(),
                    Person::Third => "bųdųt".to_string(),
                },
            },
            VerbTense::Conditional(g, p,n) => match n {
                Number::Sing => match p {
                    Person::First => "byh".to_string(),
                    Person::Second => "byš".to_string(),
                    Person::Third => "by".to_string(),
                },
                Number::Plur => match p {
                    Person::First => "byhmo".to_string(),
                    Person::Second => "byste".to_string(),
                    Person::Third => "by".to_string(),
                },
            },
            VerbTense::Imperative(p, n) => match n {
                Number::Sing => match p {
                    Person::First => "bųd́".to_string(),
                    Person::Second => "bųd́".to_string(),
                    Person::Third => "nehaj bųd́".to_string(),
                },
                Number::Plur => match p {
                    Person::First => " bųd́mo".to_string(),
                    Person::Second => "bųd́te".to_string(),
                    Person::Third => "bųd́te".to_string(),
                },
            },
            VerbTense::LParticiple(g,n)=>   match n {
                Number::Sing => match g {
                    Gender::Feminine => format!("byla"),
                    Gender::Masculine => format!("byl"),
                    Gender::Neuter => format!("bylo"),
                },
                Number::Plur => {
                    format!("byli")
                }
            }, 
            VerbTense::Perfect(g, p, n)=> format!("{} {}",Verb::byti(VerbTense::Present(p.clone(), n.clone())), Verb::byti(VerbTense::LParticiple(g.clone(), n.clone()))),
            VerbTense::PluPerfect(g, p, n)=> format!("{} {}",Verb::byti(VerbTense::Imperfect(p.clone(), n.clone())), Verb::byti(VerbTense::LParticiple(g.clone(), n.clone()))),

           
            _ => panic!("THIS BYTI NOT IMPLEMENTED {:#?}",vt),
        }
    }

    pub fn imperfect(&self, p: &Person, n: &Number) -> String {
        let stem = &self.infinitive_stem;
        let ends_with_vowel = is_vowel(last_in_slice(stem));

        let mut ending = if ends_with_vowel {
            match n {
                Number::Sing => match p {
                    Person::First => "h",
                    Person::Second => "še",
                    Person::Third => "še",
                },
                Number::Plur => match p {
                    Person::First => "hmo",
                    Person::Second => "ste",
                    Person::Third => "hu",
                },
            }
        } else {
            match n {
                Number::Sing => match p {
                    Person::First => "eh",
                    Person::Second => "eše",
                    Person::Third => "eše",
                },
                Number::Plur => match p {
                    Person::First => "ehmo",
                    Person::Second => "este",
                    Person::Third => "ehu",
                },
            }
        };

        format!("{}{}", stem, ending)
    }

    pub fn present_active_participle(&self) -> String {

       let pluralthree =  self.present(&Person::Third, &Number::Plur);
       replace_last_occurence(&pluralthree, "t", "či")

    }
    pub fn past_active_participle(&self) -> String {

        if is_vowel(last_in_slice(&self.infinitive_stem)) {
            format!("{}vši",&self.infinitive_stem)

        }
        else {format!("{}ši",&self.infinitive_stem)}



        
 
     }

     pub fn present_passive_participle(&self) -> String {

        let stemmy =  &self.present_tense_stem;

        let ending = match self.conjugation {

            Conjugation::Second=>"my",
            Conjugation::First=> {
                if is_hard_consonant(last_in_slice(stemmy)) {"omy"}
                else {"emy"}
            }

        };

       format!("{}{}",stemmy,ending)
 
     }

     pub fn past_passive_participle(&self) -> String {

        if self.infinitive.ends_with("iti") {
            j_merge_stem_past_passive(&self.infinitive_stem)

        }

        else if self.infinitive.ends_with("nuti") {
            format!("{}eny",slice_without_last(&self.infinitive_stem))

        }
        else if  last_n_chars(&self.present_tense_stem, 3).contains("j")  {
            format!("{}ty",&self.infinitive_stem)
        }

        else if is_vowel(last_in_slice(&self.infinitive_stem)) {
            format!("{}ny",&self.infinitive_stem)

        }
        else {format!("{}eny",&self.infinitive_stem)}



        
 
     }

     pub fn verbal_noun(&self) -> String {

        replace_last_occurence(&self.past_passive_participle(), "y", "je")
     }

    pub fn present(&self, p: &Person, n: &Number) -> String {
        let mut stem = self.present_tense_stem.clone();

        let mut ending = match self.conjugation {
            Conjugation::First => match n {
                Number::Sing => match p {
                    Person::First => "ų",
                    Person::Second => "eš",
                    Person::Third => "e",
                },
                Number::Plur => match p {
                    Person::First => "emo",
                    Person::Second => "ete",
                    Person::Third => "ųt",
                },
            },
            Conjugation::Second => match n {
                Number::Sing => match p {
                    Person::First => "jų",
                    Person::Second => "iš",
                    Person::Third => "i",
                },
                Number::Plur => match p {
                    Person::First => "imo",
                    Person::Second => "ite",
                    Person::Third => "et",
                },
            },
        };

        if stem.ends_with("k") && (ending.chars().nth(0) == Some('e')) {
            stem = replace_last_occurence(&stem, "k", "č");
        } else if stem.ends_with("g") && (ending.chars().nth(0) == Some('e')) {
            stem = replace_last_occurence(&stem, "g", "ž");
        } else if self.conjugation == Conjugation::Second && (ending.chars().nth(0) == Some('j')) {
            return j_merge_stem_second_present(&stem);
        }

        match self.conjugation {
            Conjugation::First => {
                format!("{}{}", stem, ending)
            }
            Conjugation::Second => replace_last_occurence(&stem, "i", ending),
        }
    }

    pub fn imperative(&self, p: &Person, n: &Number) -> String {
        let mut stem = self.present_tense_stem.clone();

        let prefix =  match p {
                Person::First => "",
                Person::Second => "",
                Person::Third => "nehaj ",
            }
        ;

        let ending =  match n {
                Number::Sing => match p {
                    _ => "",
                   
                },
                Number::Plur => match p {
                    Person::First => "mo",
                    Person::Second => "te",
                    Person::Third => "te",
                },
            };

        if is_consonant(last_in_slice(&stem))  {
            stem = format!("{stem}i");
        } 

        
                format!("{}{}{}",prefix, stem, ending)
     
    }
}
