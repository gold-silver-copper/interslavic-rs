use crate::grammar::{Conjugation, Gender, Number, Person, Tense};
use crate::utils::{char_at, char_len, slice_chars};
use crate::InterslavicCore;
use regex::Regex;
use std::sync::LazyLock;

pub type Verb = String;

const PREFIXES: &[&str] = &[
    "prědpo", "razpro", "råzpro", "izpo", "odpo", "nad", "pod", "pre", "pri", "pro", "prě", "raz",
    "råz", "voz", "vȯz", "do", "iz", "na", "ne", "ob", "od", "po", "sȯ", "vo", "vy", "vȯ", "za",
    "o", "s", "u", "v",
];

impl InterslavicCore {
    pub fn verb(word: &str, person: Person, number: Number, tense: Tense) -> Verb {
        Self::verb_with_addition(word, "", "v.tr. ipf./pf.", person, number, tense)
    }

    pub fn verb_with_addition(
        word: &str,
        raw_pts: &str,
        part_of_speech: &str,
        person: Person,
        number: Number,
        tense: Tense,
    ) -> Verb {
        match tense {
            Tense::Present => Self::conjugate_present_with_addition(word, raw_pts, person, number),
            Tense::Imperfect => {
                select_person_number(&build_imperfect_for(word, raw_pts), person, number)
            }
            Tense::Future => select_person_number(&build_future_for(word, raw_pts), person, number),
            Tense::Perfect | Tense::PluPerfect | Tense::Conditional => {
                let _ = part_of_speech;
                word.to_string()
            }
        }
    }

    pub fn conjugate_present(word: &str, person: Person, number: Number) -> Verb {
        Self::conjugate_present_with_addition(word, "", person, number)
    }

    pub fn conjugate_present_with_addition(
        word: &str,
        raw_pts: &str,
        person: Person,
        number: Number,
    ) -> Verb {
        let (inf, refl) = split_reflexive(&word.to_lowercase());
        let inf = normalize_biti(&inf);
        let pref = prefix(&inf);
        let pts = clean_pts(raw_pts);
        let is = infinitive_stem(&pref, &inf, &pts);
        let ps = present_tense_stem(&pref, &pts, &is);
        let psi = secondary_present_tense_stem(&ps);
        let present = build_present(&pref, &ps, &psi, &refl);
        first_alternative(&select_person_number(&present, person, number))
    }

    pub fn get_present_tense_stem(infinitive: &str) -> (String, Conjugation) {
        let (inf, _) = split_reflexive(&infinitive.to_lowercase());
        let inf = normalize_biti(&inf);
        let pref = prefix(&inf);
        let is = infinitive_stem(&pref, &inf, "");
        let ps = present_tense_stem(&pref, "", &is);
        let conjugation = if ps.ends_with('i') {
            Conjugation::Second
        } else {
            Conjugation::First
        };
        (format!("{pref}{ps}"), conjugation)
    }

    pub fn get_infinitive_stem(word: &str) -> String {
        let (inf, _) = split_reflexive(word);
        let pref = prefix(&inf);
        infinitive_stem(&pref, &inf, "")
    }

    pub fn l_participle(word: &str, gender: Gender, number: Number) -> Verb {
        let (inf, _) = split_reflexive(word);
        let pref = prefix(&inf);
        let is = infinitive_stem(&pref, &inf, "");
        let mut base = l_participle_base(&pref, &is);
        base = idti(&base);
        base = zegti(&base);
        match number {
            Number::Plural => transliterate_back(&format!("{base}i")),
            Number::Singular => match gender {
                Gender::Masculine => transliterate_back(&base),
                Gender::Feminine => transliterate_back(&format!("{base}a")),
                Gender::Neuter => transliterate_back(&format!("{base}o")),
            },
        }
    }
}

fn split_reflexive(inf: &str) -> (String, String) {
    let has_ne = inf.starts_with("ne ");
    let start = if has_ne { 3 } else { 0 };
    if let Some(space_idx) = inf[start..].find(' ').map(|i| i + start) {
        let maybe = inf.get(space_idx + 1..space_idx + 3).unwrap_or("");
        let se = if maybe == "se" || maybe == "sę" {
            " sę"
        } else {
            ""
        };
        (inf[..space_idx].to_string(), se.to_string())
    } else {
        (inf.to_string(), String::new())
    }
}

fn normalize_biti(inf: &str) -> String {
    if matches!(inf, "sųt" | "je" | "jest") {
        "byti".to_string()
    } else {
        inf.to_string()
    }
}

fn prefix(inf: &str) -> String {
    if non_regular_verbs().is_match(inf) {
        if let Some(m) = non_regular_verbs().find(inf) {
            let maybe = &inf[..m.start()];
            if PREFIXES.contains(&maybe) {
                return maybe.to_string();
            }
        }
    }
    if let Some(i) = inf.find('-') {
        return inf[..=i].to_string();
    }
    if inf.starts_with("ne ") {
        return "ne ".to_string();
    }
    String::new()
}

fn clean_pts(raw: &str) -> String {
    raw.replace(") (", ")(")
        .replace(['(', ')'], "")
        .split([';', ',', '/'])
        .next()
        .unwrap_or("")
        .replace("+1", "")
        .replace("+2", "")
        .replace("+3", "")
        .replace("+4", "")
        .replace("+5", "")
        .replace("+6", "")
        .replace("+7", "")
        .replace("+8", "")
        .replace("+9", "")
        .trim()
        .to_string()
}

fn infinitive_stem(pref: &str, inf: &str, pts: &str) -> String {
    let trunc = inf.strip_prefix(pref).unwrap_or(inf);
    let Some(result) = strip_infinitive_ending(trunc) else {
        return "ERROR-2".into();
    };
    if result.ends_with('s') {
        if result == "jes" {
            "jed".to_string()
        } else if !pts.is_empty() {
            slice_chars(pts, 0, char_len(pts).saturating_sub(1))
        } else {
            result
        }
    } else {
        result
    }
}

fn strip_infinitive_ending(trunc: &str) -> Option<String> {
    for ending in ["ti", "tì", "t", "ť"] {
        if let Some(stem) = trunc.strip_suffix(ending) {
            return Some(stem.to_string());
        }
    }
    None
}

fn derive_present_tense_stem(is: &str) -> String {
    let mut result = is.to_string();
    if result == "vzę" {
        result = "vȯzm".into();
    } else if result == "umě" {
        result = "uměĵ".into();
    } else if result == "hova" {
        result = "hovaĵ".into();
    } else if eva_ova().is_match(&result) {
        result = format!("{}uj", slice_chars(&result, 0, char_len(&result) - 3));
    } else if nuu().is_match(&result) {
        result = slice_chars(&result, 0, char_len(&result) - 1);
    } else if ouee().is_match(&result) {
        result.push('j');
    } else if bdsze().is_match(&result) {
        result = format!("{}ȯjm", slice_chars(&result, 0, char_len(&result) - 2));
    } else if result.ends_with("ję") {
        result = format!("{}m", slice_chars(&result, 0, char_len(&result) - 1));
    } else if result.ends_with('ę') {
        result = format!("{}n", slice_chars(&result, 0, char_len(&result) - 1));
    } else if result.ends_with('ų') {
        result = slice_chars(&result, 0, char_len(&result) - 1);
    } else if result.ends_with('y') {
        result.push('j');
    } else if result.ends_with(['a', 'e', 'ě']) {
        result.push('ĵ');
    }
    result
}

fn present_tense_stem(pref: &str, pts: &str, is: &str) -> String {
    let mut result;
    if pts.is_empty() {
        result = derive_present_tense_stem(is);
    } else {
        let mut pts = pts.to_string();
        if pts.ends_with(" se") || pts.ends_with(" sę") {
            pts = slice_chars(&pts, 0, char_len(&pts) - 3);
        } else if pts.starts_with("se ") || pts.starts_with("sę ") {
            pts = pts[3..].to_string();
        }
        if !pref.is_empty() {
            if pts.contains(pref) {
                pts = pts[pref.len()..].to_string();
            } else {
                let byte = pref
                    .char_indices()
                    .nth(char_len(pref).saturating_sub(1))
                    .map_or(0, |(i, _)| i);
                pts = pts.get(byte..).unwrap_or("").to_string();
            }
        }
        if pts
            .chars()
            .last()
            .is_some_and(|c| matches!(c, 'm' | 'e' | 'u' | 'ų' | '-'))
        {
            result = slice_chars(&pts, 0, char_len(&pts).saturating_sub(1));
        } else {
            result = pts;
        }
    }
    process_present_tense_stem_exceptions(pref, is, &mut result);
    result
}

fn process_present_tense_stem_exceptions(pref: &str, is: &str, result: &mut String) {
    if (is == "by" && pref.is_empty()) || ((result == "je" || result == "j") && is == "bi") {
        *result = "jes".into();
    } else if result == "věděĵ" {
        *result = "vě".into();
    } else if result == "jed" || (result == "j" && is == "jed") {
        *result = "je".into();
    } else if result == "jěd" || (result == "j" && is == "jěd") {
        *result = "jě".into();
    } else if result == "jad" || (result == "j" && is == "jad") {
        *result = "ja".into();
    } else if result == "daĵ" {
        *result = "da".into();
    } else if result == "žeg" || result == "žž" {
        *result = "žg".into();
    } else if result.ends_with("maj") {
        *result = format!("{}ĵ", slice_chars(result, 0, char_len(result) - 1));
    }
    if result == "jěhaĵ" || (result == "jě" && is == "jěha") {
        *result = "jěd".into();
    }
    if result == "jehaĵ" || (result == "je" && is == "jeha") {
        *result = "jed".into();
    }
    if result == "jahaĵ" || (result == "ja" && is == "jaha") {
        *result = "jad".into();
    }
}

fn secondary_present_tense_stem(ps: &str) -> String {
    ps.strip_suffix('g')
        .map(|s| format!("{s}ž"))
        .or_else(|| ps.strip_suffix('k').map(|s| format!("{s}č")))
        .unwrap_or_else(|| ps.to_string())
}

fn l_participle_base(pref: &str, is: &str) -> String {
    if is == "vojd" || is == "vȯjd" {
        "všėl".to_string()
    } else if is == "id" || is == "jd" {
        format!("{pref}šėl")
    } else if is.ends_with("id") || is.ends_with("jd") {
        format!("{}{}šėl", pref, slice_chars(is, 0, char_len(is) - 2))
    } else {
        format!("{pref}{is}l")
    }
}

fn build_present(pref: &str, ps: &str, psi: &str, refl: &str) -> Vec<String> {
    match ps {
        "jes" => {
            return ["jesm", "jesi", "jest (je)", "jesmo", "jeste", "sųt"]
                .iter()
                .map(|s| transliterate_back(s))
                .collect()
        }
        "da" => {
            return ["dam", "daš", "da", "damo", "date", "dadųt"]
                .iter()
                .map(|s| transliterate_back(&format!("{pref}{s}{refl}")))
                .collect()
        }
        "vě" => {
            return ["věm", "věš", "vě", "věmo", "věte", "vědųt"]
                .iter()
                .map(|s| transliterate_back(&format!("{pref}{s}{refl}")))
                .collect()
        }
        "jě" => {
            return ["jěm", "jěš", "jě", "jěmo", "jěte", "jědųt"]
                .iter()
                .map(|s| transliterate_back(&format!("{pref}{s}{refl}")))
                .collect()
        }
        "je" => {
            return ["jem", "ješ", "je", "jemo", "jete", "jedųt"]
                .iter()
                .map(|s| transliterate_back(&format!("{pref}{s}{refl}")))
                .collect()
        }
        "ja" => {
            return ["jam", "jaš", "ja", "jamo", "jate", "jadųt"]
                .iter()
                .map(|s| transliterate_back(&format!("{pref}{s}{refl}")))
                .collect()
        }
        _ => {}
    }

    match ps.chars().last() {
        Some('ĵ') => {
            let cut = slice_chars(ps, 0, char_len(ps) - 1);
            let pps = format!("{cut}j");
            vec![
                format!("{pref}{pps}ų{refl}, {pref}{cut}m{refl}"),
                format!("{pref}{pps}eš{refl}, {pref}{cut}š{refl}"),
                format!("{pref}{pps}e{refl}, {pref}{cut}{refl}"),
                format!("{pref}{pps}emo{refl}, {pref}{cut}mo{refl}"),
                format!("{pref}{pps}ete{refl}, {pref}{cut}te{refl}"),
                format!("{pref}{pps}ųt{refl}"),
            ]
        }
        Some('i') => {
            let cut = slice_chars(ps, 0, char_len(ps) - 1);
            vec![
                format!("{pref}{cut}xų{refl}, {pref}{ps}m{refl}"),
                format!("{pref}{ps}š{refl}"),
                format!("{pref}{ps}{refl}"),
                format!("{pref}{ps}mo{refl}"),
                format!("{pref}{ps}te{refl}"),
                format!("{pref}{cut}ęt{refl}"),
            ]
        }
        _ => vec![
            format!("{pref}{ps}ų{refl}, {pref}{psi}em{refl}"),
            format!("{pref}{psi}eš{refl}"),
            format!("{pref}{psi}e{refl}"),
            format!("{pref}{psi}emo{refl}"),
            format!("{pref}{psi}ete{refl}"),
            format!("{pref}{ps}ųt{refl}"),
        ],
    }
    .into_iter()
    .map(|s| transliterate_back(&s))
    .collect()
}

fn build_imperfect_for(inf: &str, raw_pts: &str) -> Vec<String> {
    let (inf, refl) = split_reflexive(&inf.to_lowercase());
    let pref = prefix(&inf);
    let pts = clean_pts(raw_pts);
    let is = infinitive_stem(&pref, &inf, &pts);
    let i = char_len(&is).saturating_sub(1);
    let last = char_at(&is, i).unwrap_or('\0');
    let impst = if !matches!(
        last,
        'a' | 'e' | 'i' | 'o' | 'u' | 'y' | 'ę' | 'ų' | 'å' | 'ě' | 'ė' | 'ȯ' | ')'
    ) {
        if last == 'k' {
            format!("{}če", slice_chars(&is, 0, i))
        } else if is.ends_with("žeg") {
            "žže".to_string()
        } else if last == 'g' {
            format!("{}že", slice_chars(&is, 0, i))
        } else {
            format!("{is}e")
        }
    } else if is == "by" && pref.is_empty() {
        "bě".to_string()
    } else {
        is
    };
    ["h", "še", "še", "hmo", "ste", "hų"]
        .iter()
        .map(|end| transliterate_back(&format!("{pref}{impst}{end}{refl}")))
        .collect()
}

fn build_future_for(inf: &str, raw_pts: &str) -> Vec<String> {
    let (inf0, refl) = split_reflexive(&inf.to_lowercase());
    let inf0 = normalize_biti(&inf0);
    let pref = prefix(&inf0);
    let pts = clean_pts(raw_pts);
    let is = infinitive_stem(&pref, &inf0, &pts);
    let ps = present_tense_stem(&pref, &pts, &is);
    let mut infinitive = build_infinitive(&pref, &is, &refl);
    if ((infinitive == "biti" || infinitive == "бити") && (ps == "j" || ps == "je" || ps == "jes"))
        || matches!(infinitive.as_str(), "byti" | "bytì" | "быти")
    {
        infinitive.clear();
    }
    ["bųdų", "bųdeš", "bųde", "bųdemo", "bųdete", "bųdųt"]
        .iter()
        .map(|aux| {
            format!("{aux} {}", transliterate_back(&infinitive))
                .trim()
                .to_string()
        })
        .collect()
}

fn build_infinitive(pref: &str, is: &str, refl: &str) -> String {
    let mut stem = is.to_string();
    if stem.ends_with("st") {
        stem = slice_chars(&stem, 0, char_len(&stem) - 1);
    } else if stem.ends_with('t')
        || (stem.ends_with('d') && !stem.ends_with("id") && !stem.ends_with("jd"))
    {
        stem = format!("{}s", slice_chars(&stem, 0, char_len(&stem) - 1));
    }
    transliterate_back(&format!("{pref}{stem}tì{refl}"))
}

fn select_person_number(forms: &[String], person: Person, number: Number) -> String {
    let idx = match (person, number) {
        (Person::First, Number::Singular) => 0,
        (Person::Second, Number::Singular) => 1,
        (Person::Third, Number::Singular) => 2,
        (Person::First, Number::Plural) => 3,
        (Person::Second, Number::Plural) => 4,
        (Person::Third, Number::Plural) => 5,
    };
    forms.get(idx).cloned().unwrap_or_default()
}

fn first_alternative(s: &str) -> String {
    s.split(',').next().unwrap_or(s).trim().to_string()
}

fn idti(sel: &str) -> String {
    sel.replace("šėl(a)", "šėl/šla")
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

fn zegti(zeg: &str) -> String {
    zeg.replace("žegla", "žgla")
        .replace("žeglo", "žglo")
        .replace("žegli", "žgli")
}

fn transliterate_back(iw: &str) -> String {
    iw.replace("stx", "šć")
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

fn non_regular_verbs() -> &'static Regex {
    static RE: LazyLock<Regex> =
        LazyLock::new(|| Regex::new(r"(v[eě]d[eě]ti|j[eě]sti|d[aų]ti|byti|žegti)$").unwrap());
    &RE
}
fn eva_ova() -> &'static Regex {
    static RE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"[eo]va$").unwrap());
    &RE
}
fn nuu() -> &'static Regex {
    static RE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"..n[uų]$").unwrap());
    &RE
}
fn ouee() -> &'static Regex {
    static RE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"^..?[eěou]$").unwrap());
    &RE
}
fn bdsze() -> &'static Regex {
    static RE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"[bdsz]ję$").unwrap());
    &RE
}
