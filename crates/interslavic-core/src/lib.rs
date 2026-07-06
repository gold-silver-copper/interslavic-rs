mod case_endings;
use case_endings::*;
mod known_nouns;

use known_nouns::*;

#[derive(Debug, Clone, Default)]
pub struct ISVCore {}

pub struct ISVUTILS {}

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
    //vocative will be handle seperately
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
    PluPerfect,
    Conditional,
}

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

impl CaseEndings {
    pub fn ending(&self, case: &Case, number: &Number) -> &str {
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

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum NounGender {
    Masculine,
    Feminine,
    Neuter,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Animacy {
    Animate,
    Inanimate,
}

pub const VOWELS: &[char] = &[
    'a', 'e', 'i', 'í', 'ó', 'o', 'u', 'å', 'ą', 'ę', 'ė', 'é', 'ȯ', 'ų', 'ů', 'ú', 'ý', 'y', 'ě',
    'A', 'E', 'I', 'O', 'U', 'á',
];

pub const HARD_CONSONANTS: &[char] = &[
    'p', 'b', 'f', 'v', 'm', 's', 'z', 't', 'd', 'r', 'n', 'l', 'k', 'g', 'h',
];

pub const J_MERGE_CHARS: &[&str] = &[
    "st", "zd", "sk", "zg", "s", "z", "t", "d", "k", "g", "h", "č", "š", "ž", "ć", "đ", "j",
];

//VERB STUFF
impl ISVCore {
    pub fn get_present_tense_stem(infinitive: &str) -> (String, Conjugation) {
        let ps = ISVCore::sonic_present_tense_stem(infinitive, "");
        let conjugation = if ps.ends_with('i') {
            Conjugation::Second
        } else {
            Conjugation::First
        };
        (ps, conjugation)
    }

    pub fn get_infinitive_stem(word: &str) -> String {
        ISVCore::sonic_infinitive_stem(&ISVCore::sonic_prefix(word), word)
            .unwrap_or_else(|| ISVUTILS::string_without_last_n(word, 2))
    }

    pub fn conjugate_verb(
        word: &str,
        person: &Person,
        number: &Number,
        _gender: &Gender,
        tense: &Tense,
    ) -> String {
        ISVCore::conjugate_verb_with_present_hint(word, "", person, number, tense)
    }

    pub fn conjugate_verb_with_present_hint(
        word: &str,
        present_hint: &str,
        person: &Person,
        number: &Number,
        tense: &Tense,
    ) -> String {
        let paradigm = ISVCore::verb_paradigm_with_options(word, present_hint, true, true);
        match tense {
            Tense::Present => ISVCore::finite_slot(&paradigm.present, person, number),
            Tense::Imperfect => ISVCore::finite_slot(&paradigm.imperfect, person, number),
            Tense::Future => ISVCore::finite_slot(&paradigm.future, person, number),
            Tense::Perfect => ISVCore::gendered_compound_slot(
                &paradigm.perfect,
                person,
                number,
                &Gender::Masculine,
            ),
            Tense::PluPerfect => ISVCore::gendered_compound_slot(
                &paradigm.pluperfect,
                person,
                number,
                &Gender::Masculine,
            ),
            Tense::Conditional => ISVCore::gendered_compound_slot(
                &paradigm.conditional,
                person,
                number,
                &Gender::Masculine,
            ),
        }
    }

    pub fn conjugate_verb_with_options(
        word: &str,
        present_hint: &str,
        person: &Person,
        number: &Number,
        gender: &Gender,
        tense: &Tense,
        transitive: bool,
        imperfective: bool,
    ) -> String {
        let paradigm =
            ISVCore::verb_paradigm_with_options(word, present_hint, transitive, imperfective);
        match tense {
            Tense::Present => ISVCore::finite_slot(&paradigm.present, person, number),
            Tense::Imperfect => ISVCore::finite_slot(&paradigm.imperfect, person, number),
            Tense::Future => ISVCore::finite_slot(&paradigm.future, person, number),
            Tense::Perfect => {
                ISVCore::gendered_compound_slot(&paradigm.perfect, person, number, gender)
            }
            Tense::PluPerfect => {
                ISVCore::gendered_compound_slot(&paradigm.pluperfect, person, number, gender)
            }
            Tense::Conditional => {
                ISVCore::gendered_compound_slot(&paradigm.conditional, person, number, gender)
            }
        }
    }

    pub fn verb_paradigm_with_options(
        word: &str,
        present_hint: &str,
        transitive: bool,
        imperfective: bool,
    ) -> VerbParadigm {
        let word = word.trim();
        if word.split_whitespace().nth(1).is_some() {
            return ISVCore::empty_phrase_verb_paradigm(word);
        }

        let normalized = match word {
            "sųt" | "je" | "jest" => "byti",
            other => other,
        };
        let pref = ISVCore::sonic_prefix(normalized);
        let clean_hint = ISVCore::clean_present_hint(present_hint);
        let Some(infinitive_stem) =
            ISVCore::sonic_infinitive_stem_with_hint(&pref, normalized, &clean_hint)
        else {
            panic!("IMPROPER PRESENT TENSE STEM: {}", word);
        };
        let present_stem =
            ISVCore::sonic_present_tense_stem_from_parts(&pref, &clean_hint, &infinitive_stem);
        let secondary_stem = ISVCore::secondary_present_tense_stem(&present_stem);
        let lpa = ISVCore::sonic_l_participle(&pref, &infinitive_stem);
        let infinitive = ISVCore::build_sonic_infinitive(&pref, &infinitive_stem);
        let present = ISVCore::build_present_vec(&pref, &present_stem, &secondary_stem);
        let imperfect = ISVCore::build_imperfect_vec(&pref, &infinitive_stem);
        let perfect = ISVCore::build_perfect_vec(&lpa);
        let pluperfect = ISVCore::build_pluperfect_vec(&lpa);
        let future = ISVCore::build_future_vec(&infinitive, &present_stem);
        let conditional = ISVCore::build_conditional_vec(&lpa);
        let imperative = ISVCore::build_imperative_vec(&pref, &secondary_stem);
        let prap = imperfective.then(|| ISVCore::build_prap(&pref, &present_stem));
        let prpp = (imperfective && transitive)
            .then(|| ISVCore::build_prpp(&pref, &present_stem, &secondary_stem));
        let pfap = ISVCore::build_pfap(&lpa);
        let computed_pfpp = ISVCore::build_pfpp(&pref, &infinitive_stem, &secondary_stem);
        let pfpp = transitive.then(|| computed_pfpp.clone());
        let gerund = ISVCore::build_gerund(&computed_pfpp);

        VerbParadigm {
            infinitive,
            present,
            imperfect,
            perfect,
            pluperfect,
            future,
            conditional,
            imperative,
            prap,
            prpp,
            pfap,
            pfpp,
            gerund,
        }
    }

    fn empty_phrase_verb_paradigm(word: &str) -> VerbParadigm {
        VerbParadigm {
            infinitive: word.to_string(),
            present: vec![word.to_string(); 6],
            imperfect: vec![word.to_string(); 6],
            perfect: vec![word.to_string(); 8],
            pluperfect: vec![word.to_string(); 8],
            future: vec![word.to_string(); 6],
            conditional: vec![word.to_string(); 8],
            imperative: vec![word.to_string(); 3],
            prap: None,
            prpp: None,
            pfap: word.to_string(),
            pfpp: None,
            gerund: word.to_string(),
        }
    }

    fn finite_slot(forms: &[String], person: &Person, number: &Number) -> String {
        let slot = match (person, number) {
            (Person::First, Number::Singular) => 0,
            (Person::Second, Number::Singular) => 1,
            (Person::Third, Number::Singular) => 2,
            (Person::First, Number::Plural) => 3,
            (Person::Second, Number::Plural) => 4,
            (Person::Third, Number::Plural) => 5,
        };
        forms.get(slot).cloned().unwrap_or_default()
    }

    fn gendered_compound_slot(
        forms: &[String],
        person: &Person,
        number: &Number,
        gender: &Gender,
    ) -> String {
        let slot = match (person, number, gender) {
            (Person::First, Number::Singular, _) => 0,
            (Person::Second, Number::Singular, _) => 1,
            (Person::Third, Number::Singular, Gender::Masculine) => 2,
            (Person::Third, Number::Singular, Gender::Feminine) => 3,
            (Person::Third, Number::Singular, Gender::Neuter) => 4,
            (Person::First, Number::Plural, _) => 5,
            (Person::Second, Number::Plural, _) => 6,
            (Person::Third, Number::Plural, _) => 7,
        };
        forms.get(slot).cloned().unwrap_or_default()
    }

    fn sonic_prefix(infinitive: &str) -> String {
        const PREFIXES: &[&str] = &[
            "prědpo", "razpro", "råzpro", "izpo", "odpo", "nad", "pod", "pre", "pri", "pro", "prě",
            "raz", "råz", "voz", "vȯz", "do", "iz", "na", "ne", "ob", "od", "po", "sȯ", "vo", "vy",
            "vȯ", "za", "o", "s", "u", "v",
        ];
        const NON_REGULAR: &[&str] = &[
            "vedeti", "věděti", "jesti", "jěsti", "dati", "dųti", "byti", "žegti",
        ];
        for verb in NON_REGULAR {
            if infinitive.ends_with(verb) {
                let maybe_prefix = &infinitive[..infinitive.len() - verb.len()];
                if PREFIXES.contains(&maybe_prefix) {
                    return maybe_prefix.to_string();
                }
            }
        }
        if let Some(index) = infinitive.find('-') {
            return infinitive[..=index].to_string();
        }
        if infinitive.starts_with("ne ") {
            return "ne ".to_string();
        }
        String::new()
    }

    fn sonic_infinitive_stem(prefix: &str, infinitive: &str) -> Option<String> {
        ISVCore::sonic_infinitive_stem_with_hint(prefix, infinitive, "")
    }

    fn sonic_infinitive_stem_with_hint(
        prefix: &str,
        infinitive: &str,
        present_hint: &str,
    ) -> Option<String> {
        let trunc = infinitive.strip_prefix(prefix).unwrap_or(infinitive);
        if trunc.is_empty() {
            return None;
        }
        let mut result = None;
        for ending in ["ti", "tì", "t", "ť"] {
            if let Some(stem) = trunc.strip_suffix(ending) {
                result = Some(stem.to_string());
                break;
            }
        }
        let mut result = result?;
        if result.ends_with('s') {
            if result == "jes" {
                result = "jed".to_string();
            } else if !present_hint.is_empty() {
                result = ISVUTILS::string_without_last_n(present_hint, 1);
            }
        }
        Some(result)
    }

    fn clean_present_hint(raw: &str) -> String {
        let mut pts = raw.trim().replace(") (", ")(").replace(['(', ')'], "");
        if let Some((head, _)) = pts.split_once([';', ',', '/']) {
            pts = head.to_string();
        }
        if let Some(index) = pts.find('+') {
            if pts[index + 1..]
                .chars()
                .next()
                .is_some_and(|c| c.is_ascii_digit())
            {
                pts.replace_range(index..=index + 1, "");
            }
        }
        pts.trim().to_string()
    }

    fn sonic_present_tense_stem(infinitive: &str, present_hint: &str) -> String {
        let pref = ISVCore::sonic_prefix(infinitive);
        let stem = ISVCore::sonic_infinitive_stem(&pref, infinitive)
            .unwrap_or_else(|| ISVUTILS::string_without_last_n(infinitive, 2));
        ISVCore::sonic_present_tense_stem_from_parts(
            &pref,
            &ISVCore::clean_present_hint(present_hint),
            &stem,
        )
    }

    fn sonic_present_tense_stem_from_parts(
        prefix: &str,
        present_hint: &str,
        infinitive_stem: &str,
    ) -> String {
        let mut result = if present_hint.is_empty() {
            ISVCore::derive_sonic_present_tense_stem(infinitive_stem)
        } else {
            let mut pts = present_hint.to_string();
            if pts.ends_with(" se") || pts.ends_with(" sę") {
                pts.truncate(pts.len() - 3);
            } else if pts.starts_with("se ") || pts.starts_with("sę ") {
                pts = pts[3..].to_string();
            }
            if !prefix.is_empty() {
                if pts.contains(prefix) {
                    pts = pts[prefix.len()..].to_string();
                } else if prefix.len() > 0 && pts.len() >= prefix.len() - 1 {
                    pts = pts[prefix.len() - 1..].to_string();
                }
            }
            if pts.ends_with(['m', 'e', 'u', 'ų', '-']) {
                pts[..pts.len() - pts.chars().last().unwrap().len_utf8()].to_string()
            } else {
                pts
            }
        };
        result = ISVCore::process_present_tense_stem_exceptions(prefix, infinitive_stem, &result);
        result
    }

    fn derive_sonic_present_tense_stem(infinitive_stem: &str) -> String {
        let result = infinitive_stem;
        if result == "vzę" {
            "vȯzm".to_string()
        } else if result == "umě" {
            "uměĵ".to_string()
        } else if result == "hova" {
            "hovaĵ".to_string()
        } else if result.ends_with("eva") || result.ends_with("ova") {
            format!("{}uj", &result[..result.len() - 3])
        } else if result.chars().count() >= 3 && (result.ends_with("nu") || result.ends_with("nų"))
        {
            result[..result.len() - result.chars().last().unwrap().len_utf8()].to_string()
        } else if ISVCore::is_short_oueě_stem(result) {
            format!("{result}j")
        } else if ["bję", "dję", "sję", "zję"]
            .iter()
            .any(|suffix| result.ends_with(suffix))
        {
            format!("{}ȯjm", &result[..result.len() - "ję".len()])
        } else if result.ends_with("ję") {
            format!("{}m", &result[..result.len() - "ę".len()])
        } else if result.ends_with('ę') {
            format!("{}n", &result[..result.len() - 'ę'.len_utf8()])
        } else if result.ends_with('ų') {
            result[..result.len() - 'ų'.len_utf8()].to_string()
        } else if result.ends_with('y') {
            format!("{result}j")
        } else if result.ends_with(['a', 'e', 'ě']) {
            format!("{result}ĵ")
        } else {
            result.to_string()
        }
    }

    fn is_short_oueě_stem(stem: &str) -> bool {
        let chars: Vec<char> = stem.chars().collect();
        (2..=3).contains(&chars.len())
            && chars
                .last()
                .is_some_and(|c| matches!(c, 'e' | 'ě' | 'o' | 'u'))
    }

    fn process_present_tense_stem_exceptions(
        prefix: &str,
        infinitive_stem: &str,
        present_stem: &str,
    ) -> String {
        let mut result = present_stem.to_string();
        if (infinitive_stem == "by" && prefix.is_empty())
            || ((result == "je" || result == "j") && infinitive_stem == "bi")
        {
            result = "jes".to_string();
        } else if result == "věděĵ" {
            result = "vě".to_string();
        } else if result == "jed" || (result == "j" && infinitive_stem == "jed") {
            result = "je".to_string();
        } else if result == "jěd" || (result == "j" && infinitive_stem == "jěd") {
            result = "jě".to_string();
        } else if result == "jad" || (result == "j" && infinitive_stem == "jad") {
            result = "ja".to_string();
        } else if result == "daĵ" {
            result = "da".to_string();
        } else if result == "žeg" || result == "žž" {
            result = "žg".to_string();
        } else if result.ends_with("maj") {
            result = format!("{}ĵ", &result[..result.len() - 1]);
        }
        if result == "jěhaĵ" || (result == "jě" && infinitive_stem == "jěha") {
            result = "jěd".to_string();
        }
        if result == "jehaĵ" || (result == "je" && infinitive_stem == "jeha") {
            result = "jed".to_string();
        }
        if result == "jahaĵ" || (result == "ja" && infinitive_stem == "jaha") {
            result = "jad".to_string();
        }
        result
    }

    fn secondary_present_tense_stem(present_stem: &str) -> String {
        if let Some(stem) = present_stem.strip_suffix('g') {
            format!("{stem}ž")
        } else if let Some(stem) = present_stem.strip_suffix('k') {
            format!("{stem}č")
        } else {
            present_stem.to_string()
        }
    }

    fn sonic_l_participle(prefix: &str, infinitive_stem: &str) -> String {
        if infinitive_stem == "vojd" || infinitive_stem == "vȯjd" {
            "všėl".to_string()
        } else if infinitive_stem == "id" || infinitive_stem == "jd" {
            format!("{prefix}šėl")
        } else if infinitive_stem.ends_with("id") || infinitive_stem.ends_with("jd") {
            format!(
                "{}{}šėl",
                prefix,
                &infinitive_stem[..infinitive_stem.len() - 2]
            )
        } else {
            format!("{prefix}{infinitive_stem}l")
        }
    }

    fn build_sonic_infinitive(prefix: &str, infinitive_stem: &str) -> String {
        let mut stem = infinitive_stem.to_string();
        if stem.ends_with("st") {
            stem.truncate(stem.len() - 1);
        } else if stem.ends_with('t')
            || (stem.ends_with('d') && !stem.ends_with("id") && !stem.ends_with("jd"))
        {
            stem.truncate(stem.len() - 1);
            stem.push('s');
        }
        ISVCore::transliterate_sonic_back(&format!("{prefix}{stem}tì"))
    }

    fn build_present_vec(prefix: &str, ps: &str, psi: &str) -> Vec<String> {
        (0..6)
            .map(|slot| ISVCore::sonic_present_form_by_slot(prefix, ps, psi, slot))
            .collect()
    }

    fn sonic_present_form_by_slot(prefix: &str, ps: &str, psi: &str, slot: usize) -> String {
        let raw = match ps {
            "jes" => ["jesm", "jesi", "jest", "jesmo", "jeste", "sųt"][slot].to_string(),
            "da" => format!(
                "{}{}",
                prefix,
                ["dam", "daš", "da", "damo", "date", "dadųt"][slot]
            ),
            "vě" => format!(
                "{}{}",
                prefix,
                ["věm", "věš", "vě", "věmo", "věte", "vědųt"][slot]
            ),
            "jě" => format!(
                "{}{}",
                prefix,
                ["jěm", "jěš", "jě", "jěmo", "jěte", "jědųt"][slot]
            ),
            "je" => format!(
                "{}{}",
                prefix,
                ["jem", "ješ", "je", "jemo", "jete", "jedųt"][slot]
            ),
            "ja" => format!(
                "{}{}",
                prefix,
                ["jam", "jaš", "ja", "jamo", "jate", "jadųt"][slot]
            ),
            _ if ps.ends_with('ĵ') => {
                let cut = &ps[..ps.len() - 'ĵ'.len_utf8()];
                let pps = format!("{cut}j");
                match slot {
                    0 => format!("{prefix}{pps}ų"),
                    1 => format!("{prefix}{pps}eš"),
                    2 => format!("{prefix}{pps}e"),
                    3 => format!("{prefix}{pps}emo"),
                    4 => format!("{prefix}{pps}ete"),
                    _ => format!("{prefix}{pps}ųt"),
                }
            }
            _ if ps.ends_with('i') => {
                let cut = &ps[..ps.len() - 'i'.len_utf8()];
                match slot {
                    0 => format!("{prefix}{cut}xų"),
                    1 => format!("{prefix}{ps}š"),
                    2 => format!("{prefix}{ps}"),
                    3 => format!("{prefix}{ps}mo"),
                    4 => format!("{prefix}{ps}te"),
                    _ => format!("{prefix}{cut}ęt"),
                }
            }
            _ => match slot {
                0 => format!("{prefix}{ps}ų"),
                1 => format!("{prefix}{psi}eš"),
                2 => format!("{prefix}{psi}e"),
                3 => format!("{prefix}{psi}emo"),
                4 => format!("{prefix}{psi}ete"),
                _ => format!("{prefix}{ps}ųt"),
            },
        };
        ISVCore::transliterate_sonic_back(&raw)
    }

    fn build_imperfect_vec(prefix: &str, infinitive_stem: &str) -> Vec<String> {
        let last = infinitive_stem.chars().last().unwrap_or('\0');
        let impst = if !matches!(
            last,
            'a' | 'e' | 'i' | 'o' | 'u' | 'y' | 'ę' | 'ų' | 'å' | 'ě' | 'ė' | 'ȯ' | ')'
        ) {
            if last == 'k' {
                format!("{}če", ISVUTILS::string_without_last_n(infinitive_stem, 1))
            } else if infinitive_stem.ends_with("žeg") {
                "žže".to_string()
            } else if last == 'g' {
                format!("{}že", ISVUTILS::string_without_last_n(infinitive_stem, 1))
            } else {
                format!("{infinitive_stem}e")
            }
        } else if infinitive_stem == "by" && prefix.is_empty() {
            "bě".to_string()
        } else {
            infinitive_stem.to_string()
        };
        ["h", "še", "še", "hmo", "ste", "hų"]
            .iter()
            .map(|ending| ISVCore::transliterate_sonic_back(&format!("{prefix}{impst}{ending}")))
            .collect()
    }

    fn build_future_vec(infinitive: &str, ps: &str) -> Vec<String> {
        let verb = if ((infinitive == "biti" || infinitive == "бити")
            && (ps == "j" || ps == "je" || ps == "jes"))
            || infinitive == "byti"
            || infinitive == "bytì"
            || infinitive == "быти"
        {
            "".to_string()
        } else {
            ISVCore::transliterate_sonic_back(infinitive)
        };
        ["bųdų", "bųdeš", "bųde", "bųdemo", "bųdete", "bųdųt"]
            .iter()
            .map(|aux| format!("{aux} {verb}"))
            .collect()
    }

    fn build_perfect_vec(lpa: &str) -> Vec<String> {
        [
            format!("jesm {lpa}(a)"),
            format!("jesi {lpa}(a)"),
            format!("(je) {lpa}"),
            format!("(je) {lpa}a"),
            format!("(je) {lpa}o"),
            format!("jesmo {lpa}i"),
            format!("jeste {lpa}i"),
            format!("(sųt) {lpa}i"),
        ]
        .into_iter()
        .map(|line| ISVCore::postprocess_lpa_line(&line))
        .collect()
    }

    fn build_pluperfect_vec(lpa: &str) -> Vec<String> {
        [
            format!("běh {lpa}(a)"),
            format!("běše {lpa}(a)"),
            format!("běše {lpa}"),
            format!("běše {lpa}a"),
            format!("běše {lpa}o"),
            format!("běhmo {lpa}i"),
            format!("běste {lpa}i"),
            format!("běhų {lpa}i"),
        ]
        .into_iter()
        .map(|line| ISVCore::postprocess_lpa_line(&line))
        .collect()
    }

    fn build_conditional_vec(lpa: &str) -> Vec<String> {
        [
            format!("byh {lpa}(a)"),
            format!("bys {lpa}(a)"),
            format!("by {lpa}"),
            format!("by {lpa}a"),
            format!("by {lpa}o"),
            format!("byhmo {lpa}i"),
            format!("byste {lpa}i"),
            format!("by {lpa}i"),
        ]
        .into_iter()
        .map(|line| ISVCore::postprocess_lpa_line(&line))
        .collect()
    }

    fn postprocess_lpa_line(line: &str) -> String {
        let mut res = if line.contains("šėl") {
            ISVCore::idti(line)
        } else {
            line.to_string()
        };
        if res.contains("žegl") {
            res = ISVCore::zegti(&res);
        }
        ISVCore::transliterate_sonic_back(&res)
    }

    fn build_imperative_vec(prefix: &str, ps: &str) -> Vec<String> {
        let chars: Vec<char> = ps.chars().collect();
        let last = chars.last().copied().unwrap_or('\0');
        let penultimate = chars.iter().rev().nth(1).copied().unwrap_or('\0');
        let mut p2s = if ps == "jes" {
            "bųď".to_string()
        } else if ps == "da" {
            format!("{prefix}{ps}j")
        } else if ISVCore::is_irregular_stem(ps) {
            format!("{prefix}{ps}ď")
        } else if (last == 'ĵ' || last == 'j') && !(penultimate == 'l' || penultimate == 'n') {
            format!("{prefix}{ps}")
        } else if last == 'a' || last == 'e' || last == 'ě' {
            format!("{prefix}{ps}j")
        } else if last == 'i' {
            format!("{prefix}{ps}")
        } else {
            format!("{prefix}{ps}i")
        };
        p2s = p2s.replace("jij", "j").replace("ĵij", "ĵ");
        [p2s.clone(), format!("{p2s}mo"), format!("{p2s}te")]
            .into_iter()
            .map(|form| ISVCore::transliterate_sonic_back(&form))
            .collect()
    }

    fn build_prap(prefix: &str, ps: &str) -> String {
        let last = ps.chars().last().unwrap_or('\0');
        let base = if ps == "jes" {
            format!("{prefix}sų")
        } else if ISVCore::is_irregular_stem(ps) {
            format!("{prefix}{ps}dų")
        } else if last == 'a' || last == 'e' || last == 'ě' {
            format!("{prefix}{ps}jų")
        } else if last == 'i' {
            format!("{}{}ę", prefix, ISVUTILS::string_without_last_n(ps, 1))
        } else {
            format!("{prefix}{ps}ų")
        };
        ISVCore::transliterate_sonic_back(&format!("{base}ćí ({base}ćá, {base}ćé)"))
    }

    fn build_prpp(prefix: &str, ps: &str, psi: &str) -> String {
        let mut result = String::new();
        let mut ps = ps.to_string();
        if ps == "jes" {
            result = "—".to_string();
        } else if ISVCore::is_irregular_stem(&ps) {
            ps = format!("{ps}do");
        }
        let last = ps.chars().last().unwrap_or('\0');
        if last == 'ĵ' {
            let cut = ISVUTILS::string_without_last_n(&ps, 1);
            let psj = format!("{cut}j");
            result = format!("{prefix}{psj}emý (—á, —œ), {prefix}{cut}mý (—á, —œ)");
        } else if last == 'j' {
            result = format!("{prefix}{psi}emý ({prefix}{psi}emá, {prefix}{psi}emœ)");
        } else if matches!(last, 's' | 'z' | 't' | 'd' | 'l') {
            result = format!("{prefix}{ps}omý ({prefix}{ps}omá, {prefix}{ps}omœ)");
        } else if last == 'i' || last == 'o' {
            result = format!("{prefix}{ps}mý ({prefix}{ps}má, {prefix}{ps}mœ)");
        } else if result != "—" {
            result = format!("{prefix}{psi}emý ({prefix}{psi}emá, {prefix}{psi}emœ)");
        }
        ISVCore::transliterate_sonic_back(&result)
    }

    fn build_pfap(lpa: &str) -> String {
        let before_l = lpa.chars().rev().nth(1).unwrap_or('\0');
        let mut result = if matches!(
            before_l,
            'a' | 'e' | 'i' | 'o' | 'u' | 'y' | 'ę' | 'ų' | 'å' | 'ě' | 'ė' | 'ȯ' | ')'
        ) {
            format!("{}vši", ISVUTILS::string_without_last_n(lpa, 1))
        } else {
            format!("{}ši", ISVUTILS::string_without_last_n(lpa, 1))
        };
        if result.contains("šėv") {
            result = ISVCore::idti(&result);
        }
        let fem_neut_stem = ISVUTILS::string_without_last_n(&result, 1);
        result = format!("{result} ({fem_neut_stem}á, {fem_neut_stem}é)");
        ISVCore::transliterate_sonic_back(&result)
    }

    fn build_pfpp(prefix: &str, infinitive_stem: &str, psi: &str) -> String {
        let is = infinitive_stem;
        let last = is.chars().last().unwrap_or('\0');
        let ppps = if (matches!(last, 'i' | 'y' | 'u' | 'ě')
            && psi.ends_with(['j', 'v', 'n'])
            && psi != "imaj")
            || matches!(last, 'ę' | 'u' | 'ų' | 'å')
            || is == "by"
            || (is.ends_with("lě") && psi.ends_with("lj"))
        {
            format!("{prefix}{is}t")
        } else if last == 'a' || last == 'e' || last == 'ě' {
            format!("{prefix}{is}n")
        } else if last == 'i' {
            let mut ppps = format!("{prefix}{is}Xen");
            for (from, to) in [
                ("stiX", "šćX"),
                ("zdiX", "žđX"),
                ("siX", "šX"),
                ("ziX", "žX"),
                ("tiX", "ćX"),
                ("diX", "đX"),
                ("jiX", "jX"),
                ("šiX", "šX"),
                ("žiX", "žX"),
                ("čiX", "čX"),
                ("iX", "jX"),
                ("X", ""),
            ] {
                ppps = ppps.replace(from, to);
            }
            ppps
        } else if last == 'k' || last == 'g' {
            if psi.ends_with('i') {
                format!("{}{}en", prefix, ISVUTILS::string_without_last_n(psi, 1))
            } else {
                format!("{prefix}{psi}en")
            }
        } else {
            format!("{prefix}{is}en")
        };
        ISVCore::transliterate_sonic_back(&format!("{ppps}ý ({ppps}á, {ppps}ó)"))
    }

    fn build_gerund(pfpp: &str) -> String {
        let stem = pfpp.split('ý').next().unwrap_or(pfpp).trim_end();
        ISVCore::transliterate_sonic_back(&format!("{stem}ıje")).replace("ne ", "ne")
    }

    fn is_irregular_stem(ps: &str) -> bool {
        matches!(ps, "da" | "je" | "jě" | "ja" | "vě")
    }

    fn idti(input: &str) -> String {
        input
            .replace("šėl(a)", "šėl/šla")
            .replace("šėl(a)", "šėl/šla")
            .replace("všėl/šla", "všėl/vȯšla")
            .replace("všėl/šla", "všėl/vȯšla")
            .replace("šėla", "šla")
            .replace("šėlo", "šlo")
            .replace("šėli", "šli")
            .replace("všl", "vȯšl")
            .replace("izoš", "izš")
            .replace("izȯš", "izš")
            .replace("obȯš", "obš")
            .replace("oboš", "obš")
            .replace("odȯš", "odš")
            .replace("odoš", "odš")
            .replace("podȯš", "podš")
            .replace("podoš", "podš")
            .replace("nadȯš", "nadš")
            .replace("nadoš", "nadš")
    }

    fn zegti(input: &str) -> String {
        if input.ends_with("žegl(a)") {
            input.replace("žegl(a)", "žegl/žgla")
        } else {
            input
                .replace("žegla", "žgla")
                .replace("žeglo", "žglo")
                .replace("žegli", "žgli")
        }
    }

    fn transliterate_sonic_back(input: &str) -> String {
        input
            .replace("stx", "šć")
            .replace("zdx", "žđ")
            .replace("sx", "š")
            .replace("šx", "š")
            .replace("zx", "ž")
            .replace("žx", "ž")
            .replace("cx", "č")
            .replace("čx", "č")
            .replace("tx", "ć")
            .replace("dx", "đ")
            .replace("jx", "j")
            .replace('x', "j")
            .replace('-', "")
            .replace('—', "-")
            .replace("lı", "ľ")
            .replace("nı", "ń")
            .replace("rı", "ŕ")
            .replace("tı", "ť")
            .replace("dı", "ď")
            .replace("sı", "ś")
            .replace("zı", "ź")
            .replace('ı', "")
    }

    pub fn l_participle(word: &str, gender: &Gender, number: &Number) -> String {
        if word == "idti" {
            match number {
                Number::Singular => String::from("šli"),
                Number::Plural => match gender {
                    Gender::Masculine => String::from("šėl"),
                    Gender::Feminine => String::from("šla"),
                    Gender::Neuter => String::from("šlo"),
                },
            }
        } else {
            let infinitive_stem = ISVCore::get_infinitive_stem(word);
            match number {
                Number::Plural => {
                    format!("{}{}", infinitive_stem, "li")
                }
                Number::Singular => match gender {
                    Gender::Masculine => {
                        format!("{}{}", infinitive_stem, "l")
                    }
                    Gender::Feminine => {
                        format!("{}{}", infinitive_stem, "la")
                    }
                    Gender::Neuter => {
                        format!("{}{}", infinitive_stem, "lo")
                    }
                },
            }
        }
    }
}

// ADJECTIVE STUFF
impl ISVCore {
    pub fn decline_adj(
        word: &str,
        case: &Case,
        number: &Number,
        gender: &Gender,
        animacy: Animacy,
    ) -> String {
        let original_word = word.trim().to_string();
        if original_word.split_whitespace().nth(1).is_some() {
            return original_word;
        }

        let mut word = original_word.clone();
        if word == "seksi" {
            word = "sekśi".into();
        }
        if original_word == "seksi"
            && gender == &Gender::Masculine
            && animacy != Animacy::Animate
            && case == &Case::Acc
            && number == &Number::Singular
        {
            return "seksi".into();
        }
        let stem_is_soft = ISVCore::stem_of_adj_is_soft(&word);
        let adj_stem = ISVCore::get_adj_stem(&word);
        let has_long_form_ending =
            word.ends_with('y') || word.ends_with('i') || word.ends_with('j');

        if !has_long_form_ending
            && gender == &Gender::Masculine
            && number == &Number::Singular
            && (case == &Case::Nom || (animacy != Animacy::Animate && case == &Case::Acc))
        {
            return word;
        }

        let endings = match gender {
            Gender::Masculine => {
                if animacy == Animacy::Animate {
                    if stem_is_soft {
                        &ADJ_ANIMATE_SOFT_ENDINGS
                    } else {
                        &ADJ_ANIMATE_HARD_ENDINGS
                    }
                } else {
                    if stem_is_soft {
                        &ADJ_INANIMATE_SOFT_ENDINGS
                    } else {
                        &ADJ_INANIMATE_HARD_ENDINGS
                    }
                }
            }
            Gender::Feminine => {
                if stem_is_soft {
                    &ADJ_FEMININE_SOFT_ENDINGS
                } else {
                    &ADJ_FEMININE_HARD_ENDINGS
                }
            }
            Gender::Neuter => {
                if stem_is_soft {
                    &ADJ_NEUTER_SOFT_ENDINGS
                } else {
                    &ADJ_NEUTER_HARD_ENDINGS
                }
            }
        };
        let ending = endings.ending(case, number);
        format!("{}{}", adj_stem, ending)
    }

    pub fn stem_of_adj_is_soft(word: &str) -> bool {
        word.ends_with("i")
    }
    pub fn get_adj_stem(word: &str) -> String {
        if word.ends_with('y') || word.ends_with('i') || word.ends_with('j') {
            let mut adj_stem = word.to_string();
            adj_stem.pop();
            adj_stem
        } else {
            ISVCore::infer_fluent_vowel(word)
                .replace("(e)", "")
                .replace("(o)", "")
        }
    }
}

//NOUN STUFF
impl ISVCore {
    #[allow(clippy::too_many_arguments)]
    pub fn decline_noun_explicit(
        lemma: &str,
        case: &Case,
        number: &Number,
        gender: NounGender,
        animacy: Animacy,
        plural_only: bool,
        singular_only: bool,
        indeclinable: bool,
        addition: Option<&str>,
    ) -> String {
        let gender = ISVCore::api_gender_to_gender(gender);
        ISVCore::decline_noun_steen(
            lemma,
            addition.unwrap_or(""),
            &gender,
            animacy == Animacy::Animate,
            plural_only,
            singular_only,
            indeclinable,
            case,
            number,
        )
    }

    pub fn decline_noun_simple(
        lemma: &str,
        case: &Case,
        number: &Number,
        gender: NounGender,
        animacy: Animacy,
    ) -> String {
        ISVCore::decline_noun_explicit(
            lemma, case, number, gender, animacy, false, false, false, None,
        )
    }

    fn api_gender_to_gender(gender: NounGender) -> Gender {
        match gender {
            NounGender::Masculine => Gender::Masculine,
            NounGender::Feminine => Gender::Feminine,
            NounGender::Neuter => Gender::Neuter,
        }
    }

    pub fn decline_noun(word: &str, case: &Case, number: &Number) -> String {
        let word = word.trim();
        let gender = ISVCore::guess_gender(word);
        let word_is_animate = ISVCore::noun_is_animate(word);
        ISVCore::decline_noun_steen(
            word,
            "",
            &gender,
            word_is_animate,
            false,
            false,
            false,
            case,
            number,
        )
    }

    #[allow(clippy::too_many_arguments)]
    fn decline_noun_steen(
        word: &str,
        add: &str,
        origin_gender: &Gender,
        animate: bool,
        plural_only: bool,
        singular_only: bool,
        indeclinable: bool,
        case: &Case,
        number: &Number,
    ) -> String {
        let noun = ISVCore::remove_bracketed_text(word.trim(), '[', ']');
        if indeclinable {
            return noun;
        }
        if plural_only {
            return ISVCore::decline_plural_only_noun(&noun, add, origin_gender, case, number)
                .unwrap_or(noun);
        }
        if singular_only && number == &Number::Plural {
            return noun;
        }
        if let Some(form) = ISVCore::decline_substantivized_adjective(
            &noun,
            add,
            origin_gender,
            animate,
            case,
            number,
        ) {
            return form;
        }

        let noun = ISVCore::mark_or_infer_fluent_vowel(&noun, add);
        let marked_noun =
            ISVCore::mark_final_soft_noun_consonants(&noun.replace("(e)", "ė").replace("(o)", "ȯ"));
        let noun_without_fluent =
            ISVCore::mark_final_soft_noun_consonants(&noun.replace("(e)", "").replace("(o)", ""));
        let raw_gender = ISVCore::prepare_noun_gender(origin_gender, animate);
        let gender = ISVCore::establish_noun_gender(&marked_noun, &raw_gender);
        let root = ISVCore::establish_noun_root(&noun_without_fluent, &gender);
        let plural_root = ISVCore::establish_plural_noun_root(&root);
        let plural_gender =
            ISVCore::establish_plural_noun_gender(&root, &plural_root, &gender, &raw_gender);

        match number {
            Number::Singular => match case {
                Case::Nom => ISVCore::noun_nominative_sg(&marked_noun, &root, &gender),
                Case::Acc => {
                    let nominative_sg = ISVCore::noun_nominative_sg(&marked_noun, &root, &gender);
                    ISVCore::noun_accusative_sg(&nominative_sg, &root, &gender)
                }
                Case::Gen => ISVCore::noun_genitive_sg(&root, &gender),
                Case::Loc => ISVCore::noun_locative_sg(&root, &gender),
                Case::Dat => ISVCore::noun_dative_sg(&root, &gender),
                Case::Ins => ISVCore::noun_instrumental_sg(&root, &gender),
            },
            Number::Plural => match case {
                Case::Nom => ISVCore::noun_nominative_pl(&plural_root, &plural_gender),
                Case::Acc => {
                    let nom = ISVCore::noun_nominative_pl(&plural_root, &plural_gender);
                    let gen = ISVCore::noun_genitive_pl(&plural_root, &plural_gender);
                    if plural_gender == "m1" {
                        gen
                    } else {
                        nom
                    }
                }
                Case::Gen => ISVCore::noun_genitive_pl(&plural_root, &plural_gender),
                Case::Loc => ISVCore::noun_locative_pl(&plural_root, &gender),
                Case::Dat => ISVCore::noun_dative_pl(&plural_root, &gender),
                Case::Ins => ISVCore::noun_instrumental_pl(&plural_root, &gender),
            },
        }
    }

    fn remove_bracketed_text(input: &str, open: char, close: char) -> String {
        let mut result = String::new();
        let mut depth = 0usize;
        for c in input.chars() {
            if c == open {
                depth += 1;
            } else if c == close && depth > 0 {
                depth -= 1;
            } else if depth == 0 {
                result.push(c);
            }
        }
        result.trim().to_string()
    }

    fn mark_or_infer_fluent_vowel(word: &str, add: &str) -> String {
        let add = add.trim().trim_matches(['(', ')']);
        if !add.is_empty() && word != add {
            ISVCore::mark_fluent_vowel(word, add)
        } else {
            ISVCore::infer_fluent_vowel(word)
        }
    }

    fn mark_fluent_vowel(word: &str, add: &str) -> String {
        let word_chars: Vec<char> = word.chars().collect();
        let add_chars: Vec<char> = add.chars().collect();
        let limit = word_chars.len().saturating_sub(1).min(add_chars.len());
        let mut idx = 0usize;
        while idx < limit && word_chars[idx] == add_chars[idx] {
            idx += 1;
        }
        if idx + 1 < word_chars.len()
            && idx < add_chars.len()
            && word_chars[idx + 1] == add_chars[idx]
        {
            ISVCore::replace_fluent_vowel(word, idx)
        } else {
            word.to_string()
        }
    }

    fn infer_fluent_vowel(word: &str) -> String {
        if word.ends_with("lv") || word.ends_with("ėj") {
            return word.to_string();
        }
        let chars: Vec<char> = word.chars().collect();
        let mut result = word.to_string();
        let mut replaced = false;
        let mut end = chars.len();

        for idx in (0..chars.len()).rev() {
            let c = chars[idx];
            if !ISVCore::is_dictionary_letter(c) {
                end = idx;
                replaced = false;
            }
            if !replaced
                && matches!(c, 'è' | 'ė' | 'ȯ' | 'ò')
                && ISVCore::is_last_fluent_syllable(&chars, idx, end)
            {
                result = ISVCore::replace_fluent_vowel(&result, idx);
                replaced = true;
            }
        }
        result
    }

    fn replace_fluent_vowel(word: &str, index: usize) -> String {
        let mut result = String::new();
        for (idx, c) in word.chars().enumerate() {
            if idx == index {
                let plain = match c {
                    'ė' | 'è' => 'e',
                    'ȯ' | 'ò' => 'o',
                    _ => c,
                };
                result.push('(');
                result.push(plain);
                result.push(')');
            } else {
                result.push(c);
            }
        }
        result
    }

    fn is_last_fluent_syllable(chars: &[char], idx: usize, end: usize) -> bool {
        if idx + 2 == end {
            chars.get(idx + 1).is_some_and(ISVUTILS::is_consonant)
        } else if idx + 3 == end {
            chars.get(idx + 1) == Some(&'n') && chars.get(idx + 2) == Some(&'j')
        } else {
            false
        }
    }

    fn is_dictionary_letter(c: char) -> bool {
        c.is_alphabetic()
            || matches!(
                c,
                'å' | 'ę'
                    | 'ė'
                    | 'ȯ'
                    | 'ų'
                    | 'ě'
                    | 'ć'
                    | 'č'
                    | 'ď'
                    | 'đ'
                    | 'ĺ'
                    | 'ľ'
                    | 'ń'
                    | 'ŕ'
                    | 'ś'
                    | 'š'
                    | 'ť'
                    | 'ź'
                    | 'ž'
            )
    }

    fn decline_plural_only_noun(
        word: &str,
        add: &str,
        gender: &Gender,
        case: &Case,
        number: &Number,
    ) -> Option<String> {
        if number == &Number::Singular {
            return None;
        }

        let word_without_last = ISVUTILS::string_without_last_n(word, 1);
        let add = add.trim().trim_matches(['(', ')']);
        if add.ends_with("yh") || add.ends_with("ih") {
            let i_or_y = if add.ends_with("yh") { "y" } else { "i" };
            return Some(match case {
                Case::Nom | Case::Acc => word.to_string(),
                Case::Gen | Case::Loc => format!("{}{}h", word_without_last, i_or_y),
                Case::Dat => format!("{}{}m", word_without_last, i_or_y),
                Case::Ins => format!("{}{}mi", word_without_last, i_or_y),
            });
        }
        if !add.is_empty() {
            return None;
        }

        match gender {
            Gender::Masculine if word.ends_with(['i', 'y', 'e']) => Some(match case {
                Case::Nom | Case::Acc => word.to_string(),
                Case::Gen => format!("{}ov", word_without_last),
                Case::Loc => format!("{}ah", word_without_last),
                Case::Dat => format!("{}am", word_without_last),
                Case::Ins => format!("{}ami", word_without_last),
            }),
            Gender::Feminine if word.ends_with(['y', 'e']) => Some(match case {
                Case::Nom | Case::Acc => word.to_string(),
                Case::Gen if word.ends_with("je") => word_without_last.clone(),
                Case::Gen => ISVCore::noun_rules(&ISVCore::plural_gen_ending(
                    &(word_without_last.clone() + "%"),
                    true,
                )),
                Case::Loc => format!("{}ah", word_without_last),
                Case::Dat => format!("{}am", word_without_last),
                Case::Ins => format!("{}ami", word_without_last),
            }),
            Gender::Neuter if word.ends_with('a') => Some(match case {
                Case::Nom | Case::Acc => word.to_string(),
                Case::Gen => ISVCore::noun_rules(&ISVCore::plural_gen_ending(
                    &(word_without_last.clone() + "%"),
                    true,
                )),
                Case::Loc => format!("{}ah", word_without_last),
                Case::Dat => format!("{}am", word_without_last),
                Case::Ins => format!("{}ami", word_without_last),
            }),
            Gender::Feminine if word.ends_with('i') => Some(match case {
                Case::Nom | Case::Acc => word.to_string(),
                Case::Gen => format!("{}ij", word_without_last),
                Case::Loc => format!("{}ah", word_without_last),
                Case::Dat => format!("{}am", word_without_last),
                Case::Ins => format!("{}ami", word_without_last),
            }),
            _ => None,
        }
    }

    fn decline_substantivized_adjective(
        word: &str,
        add: &str,
        gender: &Gender,
        animate: bool,
        case: &Case,
        number: &Number,
    ) -> Option<String> {
        let add = add.trim().trim_matches(['(', ')']);
        let stem = ISVUTILS::string_without_last_n(word, 1);
        let suffix = if ["-ogo", "-ego", "-oj", "-ej"].contains(&add) {
            Some(add.to_string())
        } else if !add.is_empty() && add.starts_with(&stem) {
            Some(add.replacen(&stem, "-", 1))
        } else {
            None
        }?;

        if !["-ogo", "-ego", "-oj", "-ej"].contains(&suffix.as_str()) {
            return None;
        }

        let adjective = match gender {
            Gender::Masculine | Gender::Neuter => stem + if suffix == "-ogo" { "y" } else { "i" },
            Gender::Feminine => stem + if suffix == "-oj" { "y" } else { "i" },
        };

        Some(ISVCore::decline_adj(
            &adjective,
            case,
            number,
            gender,
            if animate {
                Animacy::Animate
            } else {
                Animacy::Inanimate
            },
        ))
    }

    fn prepare_noun_gender(gender: &Gender, animate: bool) -> String {
        match gender {
            Gender::Feminine => "f".into(),
            Gender::Neuter => "n".into(),
            Gender::Masculine => {
                if animate {
                    "m1".into()
                } else {
                    "m2".into()
                }
            }
        }
    }

    fn mark_final_soft_noun_consonants(word: &str) -> String {
        let mut normalized = word.to_string();
        for (from, to) in [("ń", "nj"), ("ň", "nj"), ("ľ", "lj"), ("ĺ", "lj")] {
            if normalized.ends_with(from) {
                normalized = ISVUTILS::replace_last_occurence(&normalized, from, to);
            }
        }

        let chars: Vec<char> = normalized.chars().collect();
        let mark_from = chars.len().saturating_sub(2);
        let mut result = String::new();
        for (idx, c) in chars.iter().enumerate() {
            result.push(*c);
            if idx >= mark_from && "cšžčćńľŕťďśźđj".contains(*c) {
                result.push('ь');
            }
        }
        result
    }

    fn establish_noun_gender(noun: &str, raw_gender: &str) -> String {
        let last_char = ISVUTILS::last_in_stringslice(noun);
        let before_last_char = ISVCore::nth_char_from_end(noun, 2).unwrap_or(' ');
        let last_two = ISVCore::last_n_chars(noun, 2);

        if noun == "den" || noun == "dėn" || noun == "denjь" || noun == "dėnjь" {
            return "m3".into();
        }
        if raw_gender.starts_with('m')
            && (last_two == "en" || noun.ends_with("enjь"))
            && (noun.starts_with("kamen")
                || noun.starts_with("jelen")
                || noun.starts_with("jęčmen")
                || noun.starts_with("ječmen")
                || noun.starts_with("koren")
                || noun.starts_with("kremen")
                || noun.starts_with("plåmen")
                || noun.starts_with("plamen")
                || noun.starts_with("pŕsten")
                || noun.starts_with("prsten")
                || noun.starts_with("strumen")
                || noun.starts_with("greben")
                || noun.starts_with("stępen")
                || noun.starts_with("stepen")
                || noun.starts_with("stųpen")
                || noun.starts_with("stupen")
                || noun.starts_with("šršen")
                || noun.starts_with("šŕšen")
                || noun.starts_with("sršen")
                || noun.starts_with("sŕšen")
                || noun.starts_with("šeršen"))
        {
            return "m3".into();
        }
        if raw_gender.starts_with('n')
            && [
                "čudo", "dělo", "divo", "drěvo", "igo", "kolo", "licьe", "nebo", "ojьe", "oko",
                "slovo", "tělo", "uho",
            ]
            .contains(&noun)
        {
            return "n3".into();
        }
        if raw_gender == "f" && last_char == 'v' {
            return "f3".into();
        }
        if noun == "mati" || noun == "dočьi" || noun == "doćьi" || noun == "doči" {
            return "f3".into();
        }
        if last_char == 'a' || last_char == 'i' {
            return "f1".into();
        }
        if last_char == 'ę' {
            return "n2".into();
        }
        if before_last_char != 'ь' && last_char == 'e' {
            return "n2".into();
        }
        if last_char == 'o' || last_char == 'e' {
            return "n1".into();
        }
        if raw_gender == "m1" {
            return "m1".into();
        }
        if raw_gender == "f" {
            return "f2".into();
        }
        "m2".into()
    }

    fn establish_noun_root(noun: &str, gender: &str) -> String {
        let has_vowel_ending = matches!(ISVUTILS::last_in_stringslice(noun), 'a' | 'e' | 'ę' | 'o');
        let mut result = if noun == "lėv" || noun == "lev" {
            "ljv".into()
        } else if noun == "Lėv" || noun == "Lev" {
            "Ljv".into()
        } else if gender.starts_with('m')
            && (noun.ends_with("ecь") || noun.ends_with("ėcь") || noun.ends_with("ècь"))
            && ISVCore::masculine_ec_keeps_c(noun)
        {
            ISVUTILS::string_without_last_n(noun, 3) + "cь"
        } else if gender == "m3" {
            noun.trim_end_matches("jь").to_string()
        } else if noun == "mati" || noun == "dočьi" || noun == "doćьi" || noun == "doči" {
            ISVUTILS::string_without_last_n(noun, 1) + "er"
        } else if gender == "f3" && noun.ends_with("ov") {
            ISVUTILS::string_without_last_n(noun, 2) + "v"
        } else if gender == "f3" {
            noun.into()
        } else if gender == "n2" && ISVCore::nth_char_from_end(noun, 2) == Some('m') {
            ISVUTILS::string_without_last_n(noun, 1) + "en"
        } else if gender == "n2" {
            ISVUTILS::string_without_last_n(noun, 1) + "ęt"
        } else if gender == "f1" && (noun == "pani" || noun.ends_with("yni")) {
            ISVUTILS::string_without_last_n(noun, 1) + "jь"
        } else if noun.ends_with('i') {
            ISVUTILS::string_without_last_n(noun, 1) + "ь"
        } else if has_vowel_ending {
            ISVUTILS::string_without_last_n(noun, 1)
        } else {
            noun.into()
        };

        if !has_vowel_ending {
            if let Some(index) = ISVCore::last_fluent_vowel_index(noun) {
                let result_len = result.chars().count();
                if index > result_len.saturating_sub(3) {
                    result = ISVCore::remove_char_at(&result, index);
                }
            }
        }
        result
    }

    fn masculine_ec_keeps_c(noun: &str) -> bool {
        let chars: Vec<char> = noun.chars().collect();
        if chars.len() < 5 {
            return false;
        }
        let before = chars[chars.len() - 5];
        let near = chars[chars.len() - 4];
        "aeiouyęųåėěȯrŕ".contains(before) || "jdtc".contains(near)
    }

    fn establish_plural_noun_root(root: &str) -> String {
        match root {
            "dětęt" | "detet" | "dětet" | "detęt" => "dětь".into(),
            "človek" | "člověk" => "ljudь".into(),
            "ok" => "očь".into(),
            "uh" => "ušь".into(),
            _ if root.ends_with("anin") => ISVUTILS::string_without_last_n(root, 2),
            _ => root.into(),
        }
    }

    fn establish_plural_noun_gender(
        root: &str,
        plural_root: &str,
        gender: &str,
        raw_gender: &str,
    ) -> String {
        if root != plural_root && !plural_root.contains('n') {
            return "f2".into();
        }
        if gender == "f1" && raw_gender == "m1" {
            return "m1".into();
        }
        gender.into()
    }

    fn noun_nominative_sg(noun: &str, root: &str, gender: &str) -> String {
        let result = if gender == "f2" && (noun.contains('ȯ') || noun.contains('ė')) {
            noun.into()
        } else if gender == "f2" {
            root.into()
        } else if gender == "f3" {
            noun.into()
        } else if gender == "m3" && root == "dn" {
            "den / denj".into()
        } else if gender == "m3" {
            format!("{} / {}j", root, root)
        } else {
            noun.into()
        };
        ISVCore::noun_rules(&result)
    }

    fn noun_accusative_sg(noun: &str, root: &str, gender: &str) -> String {
        let result = if gender == "m1" {
            format!("{}a", root)
        } else if gender == "f1" {
            format!("{}ų", root)
        } else {
            noun.into()
        };
        ISVCore::noun_rules(&result)
    }

    fn noun_genitive_sg(root: &str, gender: &str) -> String {
        let result = match gender {
            "m1" | "m2" | "n1" => format!("{}a", root),
            "f1" => format!("{}y", root),
            "f2" => format!("{}i", root),
            "f3" => format!("{}e / {}i", root, root),
            "m3" => format!("{}e / {}ja", root, root),
            "n2" => format!("{}e / {}a", root, root),
            "n3" => format!("{}a / {}ese", root, ISVCore::palatalization_ending(root)),
            _ => root.into(),
        };
        ISVCore::noun_rules(&result)
    }

    fn noun_dative_sg(root: &str, gender: &str) -> String {
        let result = match gender {
            "m1" | "m2" | "n1" => format!("{}u", root),
            "f1" => format!("{}ě", root),
            "f2" | "f3" => format!("{}i", root),
            "m3" => format!("{}i / {}ju", root, root),
            "n2" => format!("{}i / {}u", root, root),
            "n3" => format!("{}u / {}esi", root, ISVCore::palatalization_ending(root)),
            _ => root.into(),
        };
        ISVCore::noun_rules(&result)
    }

    fn noun_locative_sg(root: &str, gender: &str) -> String {
        ISVCore::noun_dative_sg(root, gender)
    }

    fn noun_instrumental_sg(root: &str, gender: &str) -> String {
        let result = match gender {
            "m1" | "m2" | "n1" => format!("{}om", root),
            "f1" => format!("{}ojų", root),
            "f3" if ISVCore::is_feminine_v_stem_with_restored_o(root) => {
                format!("{}ȯvjų", ISVUTILS::string_without_last_n(root, 1))
            }
            "f2" | "f3" => format!("{}jų", root),
            "m3" => format!("{}em / {}jem", root, root),
            "n2" => format!("{}em / {}om", root, root),
            "n3" => format!("{}om / {}esem", root, ISVCore::palatalization_ending(root)),
            _ => root.into(),
        };
        ISVCore::noun_rules(&result)
    }

    fn is_feminine_v_stem_with_restored_o(root: &str) -> bool {
        let chars: Vec<char> = root.chars().collect();
        if chars.len() < 2 || chars.last() != Some(&'v') {
            return false;
        }
        let before_v = chars[chars.len() - 2];
        let vowels = "aåeęěėioȯuųy";
        if before_v == 'l' && chars.len() >= 3 {
            return chars[chars.len() - 3] != 'o' && chars[chars.len() - 3] != 'ȯ';
        }
        !vowels.contains(before_v)
    }

    fn noun_nominative_pl(root: &str, gender: &str) -> String {
        let result = if gender == "n3" {
            format!("{}a / {}esa", root, ISVCore::palatalization_ending(root))
        } else if root == "očь" || root == "ušь" {
            format!("{}i / {}esa", root, root)
        } else if gender.starts_with('n') {
            format!("{}a", root)
        } else if gender == "m1" {
            format!("{}i", root)
        } else if gender == "f1" || gender == "m2" {
            format!("{}y", root)
        } else if gender == "m3" {
            format!("{}i / {}je", root, root)
        } else {
            format!("{}i", root)
        };
        ISVCore::noun_rules(&result)
    }

    fn noun_genitive_pl(root: &str, gender: &str) -> String {
        let (result, use_ej_for_j_percent) = if gender == "f1" || root == "morjь" || root == "poljь"
        {
            (root.replacen('ь', "%", 1) + "%", true)
        } else if root == "st" {
            ("sȯt".into(), true)
        } else if gender.starts_with('n') {
            let mut result = root.replacen('ь', "%", 1) + "%";
            if gender == "n3" {
                result = format!("{} / {}es", result, ISVCore::palatalization_ending(root));
            }
            (result, false)
        } else if gender == "m3" {
            (format!("{}ev / {}jev", root, root), true)
        } else if gender.starts_with('m') {
            (format!("{}ov", root), true)
        } else if root == "očь" || root == "ušь" {
            (format!("{}ij / {}es", root, root), true)
        } else {
            (format!("{}ij", root), true)
        };
        ISVCore::noun_rules(&ISVCore::plural_gen_ending(&result, use_ej_for_j_percent))
    }

    fn noun_dative_pl(root: &str, gender: &str) -> String {
        let result = if gender == "m3" {
            format!("{}am / {}jam", root, root)
        } else if gender == "n3" {
            format!("{}am / {}esam", root, ISVCore::palatalization_ending(root))
        } else {
            format!("{}am", root)
        };
        ISVCore::noun_rules(&result)
    }

    fn noun_instrumental_pl(root: &str, gender: &str) -> String {
        let result = if gender == "m3" {
            format!("{}ami / {}jami", root, root)
        } else if gender == "n3" {
            format!(
                "{}ami / {}esami",
                root,
                ISVCore::palatalization_ending(root)
            )
        } else {
            format!("{}ami", root)
        };
        ISVCore::noun_rules(&result)
    }

    fn noun_locative_pl(root: &str, gender: &str) -> String {
        let result = if gender == "m3" {
            format!("{}ah / {}jah", root, root)
        } else if gender == "n3" {
            format!("{}ah / {}esah", root, ISVCore::palatalization_ending(root))
        } else {
            format!("{}ah", root)
        };
        ISVCore::noun_rules(&result)
    }

    fn noun_rules(word: &str) -> String {
        word.replace("ьo", "ьe")
            .replace("ьy", "ьe")
            .replace("ьě", "i")
            .replace('#', "")
            .replace("tь", "ť")
            .replace("dь", "ď")
            .replace("sь", "ś")
            .replace("zь", "ź")
            .replace('ь', "")
            .replace("ťi", "ti")
            .replace("ďi", "di")
            .replace("śi", "si")
            .replace("źi", "zi")
            .replace("ľi", "li")
            .replace("ńi", "ni")
            .replace("ŕi", "ri")
            .replace("jy", "ji")
            .replace("cy", "ci")
            .replace("ljj", "ľj")
            .replace("njj", "ńj")
    }

    fn plural_gen_ending(word: &str, use_ej_for_j_percent: bool) -> String {
        let mut word = word
            .replace("jsk%", "jsk")
            .replace("mš%", "meš")
            .replace("zl%", "zȯl")
            .replace("tl%", "tȯl")
            .replace("mgl%", "mgȯl")
            .replace("ńj%", "nij");

        if use_ej_for_j_percent {
            word = ISVCore::replace_j_percent(&word, "pbvfmlnr", "ej");
        }
        word = ISVCore::replace_j_percent(&word, "pbvfmlnrszńľŕťďśźščžđ", "ij");
        word = ISVCore::replace_final_pair_percent(&word, "jśźďťľŕńčšžćđc", 'k', "e");
        word = ISVCore::replace_final_pair_percent(&word, "pbfvmlnrtdszkgh", 'k', "ȯ");
        word = ISVCore::replace_final_pair_percent(&word, "vmpzšžt", 'n', "e");
        word = ISVCore::replace_first_second_percent(&word, 'k', "nl", "ȯ");
        word = ISVCore::replace_first_second_percent(&word, 's', "nl", "e");

        if word.starts_with("dn%") {
            word = word.replacen("dn%", "dȯn", 1);
        }
        if word.contains("pismo%") {
            word = word.replace("pismo%", "pisem");
        }
        if word.starts_with("ťm%") {
            word = word.replacen("ťm%", "tem", 1);
        }
        if word.starts_with("sto%") {
            word = word.replacen("sto%", "sȯt", 1);
        }
        word.replace('%', "")
    }

    fn replace_j_percent(word: &str, preceding: &str, replacement: &str) -> String {
        let chars: Vec<char> = word.chars().collect();
        for idx in 2..chars.len() {
            if chars[idx - 1] == 'j' && chars[idx] == '%' && preceding.contains(chars[idx - 2]) {
                let mut result: String = chars[..idx - 1].iter().collect();
                result.push_str(replacement);
                result.extend(chars[idx + 1..].iter());
                return result;
            }
        }
        word.into()
    }

    fn replace_final_pair_percent(
        word: &str,
        first_set: &str,
        second: char,
        insert: &str,
    ) -> String {
        let chars: Vec<char> = word.chars().collect();
        for idx in 2..chars.len() {
            if chars[idx] == '%' && chars[idx - 1] == second && first_set.contains(chars[idx - 2]) {
                let mut result: String = chars[..idx - 1].iter().collect();
                result.push_str(insert);
                result.push(second);
                result.extend(chars[idx + 1..].iter());
                return result;
            }
        }
        word.into()
    }

    fn replace_first_second_percent(
        word: &str,
        first: char,
        second_set: &str,
        insert: &str,
    ) -> String {
        let chars: Vec<char> = word.chars().collect();
        for idx in 2..chars.len() {
            if chars[idx] == '%' && second_set.contains(chars[idx - 1]) && chars[idx - 2] == first {
                let mut result: String = chars[..idx - 1].iter().collect();
                result.push_str(insert);
                result.push(chars[idx - 1]);
                result.extend(chars[idx + 1..].iter());
                return result;
            }
        }
        word.into()
    }

    fn palatalization_ending(root: &str) -> String {
        if root.ends_with('g') {
            ISVUTILS::string_without_last_n(root, 1) + "žь"
        } else if root.ends_with('h') {
            ISVUTILS::string_without_last_n(root, 1) + "šь"
        } else if root.ends_with('k') {
            ISVUTILS::string_without_last_n(root, 1) + "čь"
        } else if root.ends_with("cь") {
            ISVUTILS::string_without_last_n(root, 2) + "čь"
        } else {
            root.into()
        }
    }

    fn nth_char_from_end(s: &str, n: usize) -> Option<char> {
        s.chars().rev().nth(n - 1)
    }

    fn last_fluent_vowel_index(s: &str) -> Option<usize> {
        s.chars()
            .enumerate()
            .filter(|(_, c)| *c == 'ė' || *c == 'ȯ')
            .map(|(idx, _)| idx)
            .last()
    }

    fn remove_char_at(s: &str, index: usize) -> String {
        s.chars()
            .enumerate()
            .filter_map(|(idx, c)| if idx == index { None } else { Some(c) })
            .collect()
    }
    pub fn noun_is_animate(word: &str) -> bool {
        ANIMATE_MASCULINE_NOUNS.contains(&word)
    }

    pub fn guess_gender(word: &str) -> Gender {
        if ANIMATE_MASCULINE_NOUNS.contains(&word) || INANIMATE_MASCULINE_NOUNS.contains(&word) {
            Gender::Masculine
        } else if FEMININE_NOUNS.contains(&word) {
            Gender::Feminine
        } else if NEUTER_NOUNS.contains(&word) {
            Gender::Neuter
        } else {
            let last_one = ISVCore::last_n_chars(word, 1);

            if ISVCore::is_ost_class(word) || (last_one == "a") || (last_one == "i") {
                Gender::Feminine
            } else if (last_one == "o") || (last_one == "e") {
                Gender::Neuter
            } else {
                Gender::Masculine
            }
        }
    }

    pub fn last_n_chars(word: &str, n: usize) -> String {
        let split_pos = word.char_indices().nth_back(n - 1).unwrap_or((0, 'a')).0;
        word[split_pos..].into()
    }
    pub fn is_ost_class(word: &str) -> bool {
        word.ends_with("ost")
            || word.ends_with("ost́")
            || word.ends_with("ěć")
            || word.ends_with("ěč")
            || word.ends_with("eć")
            || word.ends_with("at́")
    }

    pub fn get_noun_stem(word: &str, number: &Number) -> String {
        if word.ends_with("anin") && number == &Number::Plural {
            return ISVUTILS::string_without_last_n(word, 2);
        }
        if word.ends_with("anina") && number == &Number::Plural {
            return ISVUTILS::string_without_last_n(word, 3);
        }

        if ISVUTILS::is_vowel(&ISVUTILS::last_in_stringslice(word)) {
            ISVUTILS::string_without_last_n(word, 1)
        } else {
            String::from(word)
        }
    }
    pub fn stem_of_noun_is_soft(word: &str) -> bool {
        ISVUTILS::ends_with_soft_consonant(&ISVCore::get_noun_stem(word, &Number::Singular))
    }
}

impl ISVUTILS {
    pub fn replace_last_occurence(input: &str, pattern: &str, replacement: &str) -> String {
        if let Some(last_index) = input.rfind(pattern) {
            let (before_last, _after_last) = input.split_at(last_index);
            format!("{}{}", before_last, replacement)
        } else {
            input.into()
        }
    }
    pub fn iotation_merge(root: &str, suffix: &str) -> String {
        if suffix.chars().nth(0) == Some('j') {
            for entry in J_MERGE_CHARS {
                if root.ends_with(entry) {
                    let new_root = match *entry {
                        "st" => ISVUTILS::replace_last_occurence(root, entry, "šć"),
                        "zd" => ISVUTILS::replace_last_occurence(root, entry, "ždž"),
                        "sk" => ISVUTILS::replace_last_occurence(root, entry, "šč"),
                        "zg" => ISVUTILS::replace_last_occurence(root, entry, "žž"),
                        "s" => ISVUTILS::replace_last_occurence(root, entry, "š"),
                        "z" => ISVUTILS::replace_last_occurence(root, entry, "ž"),
                        "t" => ISVUTILS::replace_last_occurence(root, entry, "ć"),
                        "d" => ISVUTILS::replace_last_occurence(root, entry, "dž"),
                        "k" => ISVUTILS::replace_last_occurence(root, entry, "č"),
                        "g" => ISVUTILS::replace_last_occurence(root, entry, "ž"),
                        "h" => ISVUTILS::replace_last_occurence(root, entry, "š"),
                        "č" | "š" | "ž" | "ć" | "đ" | "j" => root.to_string(),
                        _ => root.to_string(),
                    };
                    let new_suffix = suffix.get(1..).unwrap();
                    return format!("{new_root}{new_suffix}");
                }
            }

            format!("{root}{suffix}")
        } else {
            format!("{root}{suffix}")
        }
    }

    pub fn is_vowel(c: &char) -> bool {
        VOWELS.contains(c)
    }

    pub fn ends_with_soft_consonant(word: &str) -> bool {
        ISVUTILS::is_soft_consonant(&ISVUTILS::last_in_stringslice(word))
    }

    pub fn is_hard_consonant(c: &char) -> bool {
        HARD_CONSONANTS.contains(c)
    }

    pub fn is_soft_consonant(c: &char) -> bool {
        !ISVUTILS::is_hard_consonant(c) && !ISVUTILS::is_vowel(c)
    }
    pub fn last_in_stringslice(s: &str) -> char {
        s.to_string().pop().unwrap_or(' ')
    }
    pub fn is_consonant(c: &char) -> bool {
        !ISVUTILS::is_vowel(c)
    }
    pub fn string_without_last_n(s: &str, n: i64) -> String {
        let mut stringik = s.to_string();
        for _ in 0..n {
            stringik.pop();
        }

        stringik
    }
}
