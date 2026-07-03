use crate::grammar::{Animacy, Case, Gender, Number};
use crate::utils::{
    char_at, char_len, infer_fluent_vowel, mark_fluent_vowel, remove_brackets, replace_at_char,
    slice_chars,
};
use crate::InterslavicCore;
use regex::{Captures, Regex};
use std::sync::LazyLock;

pub type Noun = (String, Gender);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct NounMeta {
    pub gender: Gender,
    pub animacy: Animacy,
    pub indeclinable: bool,
    pub singular_only: bool,
    pub plural_only: bool,
}

impl NounMeta {
    pub const fn new(gender: Gender, animacy: Animacy) -> Self {
        Self {
            gender,
            animacy,
            indeclinable: false,
            singular_only: false,
            plural_only: false,
        }
    }
}

type Paradigm = [[Option<String>; 2]; 7];

const AEEO: &[char] = &['a', 'e', 'ę', 'o'];
const VOWELS: &[char] = &[
    'a', 'e', 'i', 'o', 'u', 'y', 'ę', 'ų', 'å', 'ė', 'ě', 'ȯ', 'r', 'ŕ',
];
const SOFTEN_WITH_YER: &[char] = &[
    'c', 'š', 'ž', 'č', 'ć', 'ń', 'ľ', 'ŕ', 'ť', 'ď', 'ś', 'ź', 'đ', 'j',
];

impl InterslavicCore {
    pub fn noun(word: &str, case: Case, number: Number, meta: Option<NounMeta>) -> String {
        Self::noun_with_addition(word, "", case, number, meta)
    }

    pub fn noun_with_addition(
        word: &str,
        raw_add: &str,
        case: Case,
        number: Number,
        meta: Option<NounMeta>,
    ) -> String {
        Self::decline_noun_with_addition(word, raw_add, case, number, meta).0
    }

    pub fn decline_noun_with_meta(
        word: &str,
        case: Case,
        number: Number,
        meta: Option<NounMeta>,
    ) -> Noun {
        Self::decline_noun_with_addition(word, "", case, number, meta)
    }

    pub fn decline_noun_with_addition(
        word: &str,
        raw_add: &str,
        case: Case,
        number: Number,
        meta: Option<NounMeta>,
    ) -> Noun {
        let fallback_gender = meta.map_or_else(|| Self::guess_gender(word), |m| m.gender);
        let paradigm = declension_noun(word, raw_add, fallback_gender, meta);
        let case_idx = case_index(case);
        let number_idx = number_index(number);
        let selected = paradigm
            .as_ref()
            .and_then(|p| p[case_idx][number_idx].clone())
            .or_else(|| paradigm.as_ref().and_then(|p| p[case_idx][0].clone()))
            .unwrap_or_else(|| word.to_lowercase());
        (selected, fallback_gender)
    }

    pub fn noun_is_animate(word: &str) -> bool {
        matches!(word, "konj" | "mųž" | "člověk" | "brat" | "syn" | "otec")
    }

    pub fn guess_gender(word: &str) -> Gender {
        let last = word.chars().last();
        if Self::is_ost_class(word) || matches!(last, Some('a' | 'i')) {
            Gender::Feminine
        } else if matches!(last, Some('o' | 'e' | 'ę')) {
            Gender::Neuter
        } else {
            Gender::Masculine
        }
    }

    pub fn is_ost_class(word: &str) -> bool {
        word.ends_with("ost́")
            || word.ends_with("osť")
            || word.ends_with("ěć")
            || word.ends_with("ěč")
            || word.ends_with("eć")
            || word.ends_with("at́")
    }

    pub fn get_noun_stem(word: &str, number: Number) -> String {
        let gender = establish_gender(word, "m2");
        let root = establish_root(word, &gender);
        if number == Number::Plural {
            establish_plural_root(&root)
        } else {
            root
        }
    }

    pub fn stem_of_noun_is_soft(word: &str) -> bool {
        let stem = Self::get_noun_stem(word, Number::Singular);
        stem.chars().last().is_some_and(|c| {
            matches!(
                c,
                'j' | 'c' | 'ć' | 'č' | 'š' | 'ž' | 'ŕ' | 'ĺ' | 'ľ' | 'ť' | 'ś' | 'ď' | 'ń'
            )
        })
    }
}

fn case_index(case: Case) -> usize {
    match case {
        Case::Nom => 0,
        Case::Acc => 1,
        Case::Gen => 2,
        Case::Loc => 3,
        Case::Dat => 4,
        Case::Ins => 5,
        Case::Voc => 6,
    }
}

fn number_index(number: Number) -> usize {
    match number {
        Number::Singular => 0,
        Number::Plural => 1,
    }
}

fn empty_paradigm() -> Paradigm {
    std::array::from_fn(|_| std::array::from_fn(|_| None))
}

fn both(word: &str) -> [Option<String>; 2] {
    [Some(word.to_string()), Some(word.to_string())]
}

fn sg_pl(sg: impl Into<String>, pl: impl Into<String>) -> [Option<String>; 2] {
    [Some(sg.into()), Some(pl.into())]
}

fn declension_noun(
    raw_noun: &str,
    raw_add: &str,
    origin_gender: Gender,
    meta: Option<NounMeta>,
) -> Option<Paradigm> {
    let mut noun = remove_brackets(raw_noun, '[', ']').to_lowercase();
    if noun.contains(' ') {
        return None;
    }

    if meta.is_some_and(|m| m.indeclinable) {
        return Some([
            both(&noun),
            both(&noun),
            both(&noun),
            both(&noun),
            both(&noun),
            both(&noun),
            both(&noun),
        ]);
    }

    let add = raw_add.replace(['(', ')'], "");
    if meta.is_some_and(|m| m.plural_only) {
        return declension_plural_noun(&noun, &add, origin_gender);
    }

    if let Some(subst) = maybe_declension_subst_adj(&noun, &add, origin_gender, meta) {
        return Some(subst);
    }

    if !add.is_empty() && noun != add {
        noun = mark_fluent_vowel(&noun, &add);
    } else {
        noun = infer_fluent_vowel(&noun);
    }

    let raw_gender = prepare_gender(
        origin_gender,
        meta.map_or(false, |m| m.animacy == Animacy::Animate),
    );
    noun = noun.replace('ń', "nj").replace('ň', "nj");
    noun = noun.replace('ľ', "lj").replace('ĺ', "lj");
    noun = mark_final_soft_yer(&noun);

    let noun_without_fluent = noun.replace("(e)", "").replace("(o)", "");
    noun = noun.replace("(e)", "ė").replace("(o)", "ȯ");

    let gender = establish_gender(&noun, &raw_gender);
    let root = establish_root(&noun_without_fluent, &gender);
    let plroot = establish_plural_root(&root);
    let plgen = establish_plural_gender(&root, &plroot, &gender, &raw_gender);

    let nom_sg = nominative_sg(&noun, &root, &gender);
    let gen_sg = genitive_sg(&root, &gender);
    let dat_sg = dative_sg(&root, &gender);
    let acc_sg = accusative_sg(&nom_sg, &root, &gender);
    let ins_sg = instrumental_sg(&root, &gender);
    let loc_sg = locative_sg(&root, &gender);
    let voc_sg = vocative_sg(&nom_sg, &root, &gender);

    if meta.is_some_and(|m| m.singular_only) {
        let mut p = empty_paradigm();
        p[0] = [Some(nom_sg), None];
        p[1] = [Some(acc_sg), None];
        p[2] = [Some(gen_sg), None];
        p[3] = [Some(loc_sg), None];
        p[4] = [Some(dat_sg), None];
        p[5] = [Some(ins_sg), None];
        p[6] = [Some(voc_sg), None];
        return Some(p);
    }

    let nom_pl = nominative_pl(&plroot, &plgen);
    let gen_pl = genitive_pl(&plroot, &plgen);
    let dat_pl = dative_pl(&plroot, &gender);
    let acc_pl = accusative_pl(&nom_pl, &gen_pl, &plgen);
    let ins_pl = instrumental_pl(&plroot, &gender);
    let loc_pl = locative_pl(&plroot, &gender);

    Some([
        sg_pl(nom_sg, nom_pl.clone()),
        sg_pl(acc_sg, acc_pl),
        sg_pl(gen_sg, gen_pl),
        sg_pl(loc_sg, loc_pl),
        sg_pl(dat_sg, dat_pl),
        sg_pl(ins_sg, ins_pl),
        sg_pl(voc_sg, nom_pl),
    ])
}

fn prepare_gender(gender: Gender, animated: bool) -> String {
    match gender {
        Gender::Feminine => "f".to_string(),
        Gender::Neuter => "n".to_string(),
        Gender::Masculine => if animated { "m1" } else { "m2" }.to_string(),
    }
}

fn mark_final_soft_yer(noun: &str) -> String {
    let len = char_len(noun);
    if len < 2 {
        return noun.to_string();
    }
    let mut chars: Vec<char> = noun.chars().collect();
    for i in len.saturating_sub(2)..len {
        if SOFTEN_WITH_YER.contains(&chars[i]) {
            chars[i] = match chars[i] {
                'c' => 'c',
                c => c,
            };
            let mut out = String::new();
            for (idx, ch) in chars.iter().enumerate() {
                out.push(*ch);
                if idx == i {
                    out.push('ь');
                }
            }
            chars = out.chars().collect();
        }
    }
    chars.into_iter().collect()
}

fn establish_root(noun: &str, gender: &str) -> String {
    let fluent_idx = noun.rfind('ė').into_iter().chain(noun.rfind('ȯ')).max();
    let has_vowel_ending = noun.chars().last().is_some_and(|c| AEEO.contains(&c));

    let mut result = if noun == "lėv" || noun == "lev" {
        "ljv".to_string()
    } else if noun == "Lėv" || noun == "Lev" {
        "Ljv".to_string()
    } else if gender.starts_with('m') && (noun.ends_with("ecь") || noun.ends_with("ėcь")) {
        let before = char_at(noun, char_len(noun).saturating_sub(5));
        let before2 = char_at(noun, char_len(noun).saturating_sub(4));
        if before.is_some_and(|c| VOWELS.contains(&c))
            || before2.is_some_and(|c| matches!(c, 'j' | 'd' | 't' | 'c'))
        {
            format!(
                "{}cь",
                slice_chars(noun, 0, char_len(noun).saturating_sub(3))
            )
        } else {
            noun.to_string()
        }
    } else if gender == "m3" {
        noun.strip_suffix("jь").unwrap_or(noun).to_string()
    } else if matches!(noun, "mati" | "dočьi" | "doćьi") {
        format!(
            "{}er",
            slice_chars(noun, 0, char_len(noun).saturating_sub(1))
        )
    } else if gender == "f3" && noun.ends_with("ov") {
        format!(
            "{}v",
            slice_chars(noun, 0, char_len(noun).saturating_sub(2))
        )
    } else if gender == "f3" {
        noun.to_string()
    } else if gender == "n2"
        && slice_chars(
            noun,
            char_len(noun).saturating_sub(2),
            char_len(noun).saturating_sub(1),
        ) == "m"
    {
        format!(
            "{}en",
            slice_chars(noun, 0, char_len(noun).saturating_sub(1))
        )
    } else if gender == "n2" {
        format!(
            "{}ęt",
            slice_chars(noun, 0, char_len(noun).saturating_sub(1))
        )
    } else if gender == "f1" && (noun == "pani" || noun.ends_with("yni")) {
        format!(
            "{}jь",
            slice_chars(noun, 0, char_len(noun).saturating_sub(1))
        )
    } else if noun.ends_with('i') {
        format!(
            "{}ь",
            slice_chars(noun, 0, char_len(noun).saturating_sub(1))
        )
    } else if has_vowel_ending {
        slice_chars(noun, 0, char_len(noun).saturating_sub(1))
    } else {
        noun.to_string()
    };

    if !has_vowel_ending {
        if let Some(byte_idx) = fluent_idx {
            let char_idx = noun[..byte_idx].chars().count();
            if char_idx > char_len(&result).saturating_sub(3) {
                result = replace_at_char(&result, char_idx, "");
            }
        }
    }
    result
}

fn establish_plural_root(root: &str) -> String {
    if matches!(root, "dětęt" | "detet" | "dětet" | "detęt") {
        "dětь".to_string()
    } else if matches!(root, "človek" | "člověk") {
        "ljudь".to_string()
    } else if root == "ok" {
        "očь".to_string()
    } else if root == "uh" {
        "ušь".to_string()
    } else if root.ends_with("anin") {
        slice_chars(root, 0, char_len(root).saturating_sub(2))
    } else {
        root.to_string()
    }
}

fn establish_plural_gender(root: &str, plroot: &str, gender: &str, raw_gender: &str) -> String {
    if root != plroot && !plroot.contains('n') {
        "f2".to_string()
    } else if gender == "f1" && raw_gender == "m1" {
        "m1".to_string()
    } else {
        gender.to_string()
    }
}

fn establish_gender(noun: &str, gender: &str) -> String {
    let last = noun.chars().last().unwrap_or('\0');
    let before_last = char_at(noun, char_len(noun).saturating_sub(2)).unwrap_or('\0');
    if matches!(noun, "den" | "dėn" | "denjь" | "dėnjь") {
        return "m3".to_string();
    }
    if gender.starts_with('m') && (noun.ends_with("en") || noun.ends_with("enjь")) {
        let exceptions = [
            "kamen", "jelen", "jęčmen", "ječmen", "koren", "kremen", "plåmen", "plamen", "pŕsten",
            "prsten", "strumen", "greben", "stępen", "stepen", "stųpen", "stupen", "šršen",
            "šŕšen", "sršen", "sŕšen", "šeršen",
        ];
        if exceptions.iter().any(|prefix| noun.starts_with(prefix)) {
            return "m3".to_string();
        }
    }
    if gender.starts_with('n')
        && [
            "čudo", "dělo", "divo", "drěvo", "igo", "kolo", "licьe", "nebo", "ojьe", "oko",
            "slovo", "tělo", "uho",
        ]
        .contains(&noun)
    {
        return "n3".to_string();
    }
    if gender == "f" && last == 'v' {
        return "f3".to_string();
    }
    if matches!(noun, "mati" | "dočьi" | "doćьi") {
        return "f3".to_string();
    }
    if matches!(last, 'a' | 'i') {
        return "f1".to_string();
    }
    if last == 'ę' || (before_last != 'ь' && last == 'e') {
        return "n2".to_string();
    }
    if matches!(last, 'o' | 'e') {
        return "n1".to_string();
    }
    if gender == "m1" {
        return "m1".to_string();
    }
    if gender == "f" {
        return "f2".to_string();
    }
    "m2".to_string()
}

fn nominative_sg(noun: &str, root: &str, gender: &str) -> String {
    let result = if gender == "f2" {
        root.to_string()
    } else if gender == "f3" && xv_ending(root) {
        format!(
            "{}ȯv",
            slice_chars(root, 0, char_len(root).saturating_sub(1))
        )
    } else if gender == "f3" {
        noun.to_string()
    } else if gender == "m3" && root == "dn" {
        "den / denj".to_string()
    } else if gender == "m3" {
        format!("{root} / {root}j")
    } else {
        noun.to_string()
    };
    noun_rules(&result)
}

fn accusative_sg(noun: &str, root: &str, gender: &str) -> String {
    let result = if gender == "m1" {
        format!("{root}a")
    } else if gender == "f1" {
        format!("{root}ų")
    } else {
        noun.to_string()
    };
    noun_rules(&result)
}

fn genitive_sg(root: &str, gender: &str) -> String {
    let result = match gender {
        "m1" | "m2" | "n1" => format!("{root}a"),
        "f1" => format!("{root}y"),
        "f2" => format!("{root}i"),
        "f3" => format!("{root}e / {root}i"),
        "m3" => format!("{root}e / {root}ja"),
        "n2" => format!("{root}e / {root}a"),
        "n3" => format!("{root}a / {}ese", palatalization_ending(root)),
        _ => root.to_string(),
    };
    noun_rules(&result)
}

fn dative_sg(root: &str, gender: &str) -> String {
    let result = match gender {
        "m1" | "m2" | "n1" => format!("{root}u"),
        "f1" => format!("{root}ě"),
        "f2" | "f3" => format!("{root}i"),
        "m3" => format!("{root}i / {root}ju"),
        "n2" => format!("{root}i / {root}u"),
        "n3" => format!("{root}u / {}esi", palatalization_ending(root)),
        _ => root.to_string(),
    };
    noun_rules(&result)
}

fn instrumental_sg(root: &str, gender: &str) -> String {
    let result = match gender {
        "m1" | "m2" | "n1" => format!("{root}om"),
        "f1" => format!("{root}ojų"),
        "f2" => format!("{root}jų"),
        "f3" if xv_ending(root) => format!(
            "{}ȯvjų",
            slice_chars(root, 0, char_len(root).saturating_sub(1))
        ),
        "f3" => format!("{root}jų"),
        "m3" => format!("{root}em / {root}jem"),
        "n2" => format!("{root}em / {root}om"),
        "n3" => format!("{root}om / {}esem", palatalization_ending(root)),
        _ => root.to_string(),
    };
    noun_rules(&result)
}

fn locative_sg(root: &str, gender: &str) -> String {
    dative_sg(root, gender)
}

fn vocative_sg(nom_sg: &str, root: &str, gender: &str) -> String {
    let result = if gender == "m1" || gender == "m2" {
        if ec_ending(nom_sg) {
            format!(
                "{}če",
                slice_chars(root, 0, char_len(root).saturating_sub(2))
            )
        } else if root.ends_with('ь') || root.ends_with("čk") {
            format!("{root}u")
        } else if root.ends_with('k') {
            format!(
                "{}če",
                slice_chars(root, 0, char_len(root).saturating_sub(1))
            )
        } else if root.ends_with('g') {
            format!(
                "{}že",
                slice_chars(root, 0, char_len(root).saturating_sub(1))
            )
        } else if root.ends_with('h') {
            format!(
                "{}še",
                slice_chars(root, 0, char_len(root).saturating_sub(1))
            )
        } else {
            format!("{root}e")
        }
    } else if gender == "f1" {
        format!("{root}#o")
    } else if gender == "f2" {
        format!("{root}#i")
    } else if root == "dn" {
        "den / dnju".to_string()
    } else if gender == "m3" {
        format!("{root} / {root}ju")
    } else {
        return nom_sg.to_string();
    };
    noun_rules(&result)
}

fn nominative_pl(root: &str, gender: &str) -> String {
    let result = if gender == "n3" {
        format!("{root}a / {}esa", palatalization_ending(root))
    } else if root == "očь" || root == "ušь" {
        format!("{root}i / {root}esa")
    } else if gender.starts_with('n') {
        format!("{root}a")
    } else if gender == "m1" {
        format!("{root}i")
    } else if gender == "f1" || gender == "m2" {
        format!("{root}y")
    } else if gender == "m3" {
        format!("{root}i / {root}je")
    } else {
        format!("{root}i")
    };
    noun_rules(&result)
}

fn accusative_pl(nom_pl: &str, gen_pl: &str, gender: &str) -> String {
    if gender == "m1" {
        gen_pl.to_string()
    } else {
        nom_pl.to_string()
    }
}

fn genitive_pl(root: &str, gender: &str) -> String {
    let mut result = if gender == "f1" || root == "morjь" || root == "poljь" {
        format!("{}%", root.replace('ь', "%"))
    } else if root == "st" {
        "sȯt".to_string()
    } else if gender.starts_with('n') {
        let mut r = re_j_percent()
            .replace_all(&root.replace('ь', "%"), "$1ij")
            .to_string();
        r.push('%');
        if gender == "n3" {
            r.push_str(&format!(" / {}es", palatalization_ending(root)));
        }
        r
    } else if gender == "m3" {
        format!("{root}ev / {root}jev")
    } else if gender.starts_with('m') {
        format!("{root}ov")
    } else if root == "očь" || root == "ušь" {
        format!("{root}ij / {root}es")
    } else {
        format!("{root}ij")
    };
    result = plural_gen_ending(&result);
    noun_rules(&result)
}

fn dative_pl(root: &str, gender: &str) -> String {
    let result = match gender {
        "m3" => format!("{root}am / {root}jam"),
        "n3" => format!("{root}am / {}esam", palatalization_ending(root)),
        _ => format!("{root}am"),
    };
    noun_rules(&result)
}

fn instrumental_pl(root: &str, gender: &str) -> String {
    let result = match gender {
        "m3" => format!("{root}ami / {root}jami"),
        "n3" => format!("{root}ami / {}esami", palatalization_ending(root)),
        _ => format!("{root}ami"),
    };
    noun_rules(&result)
}

fn locative_pl(root: &str, gender: &str) -> String {
    let result = match gender {
        "m3" => format!("{root}ah / {root}jah"),
        "n3" => format!("{root}ah / {}esah", palatalization_ending(root)),
        _ => format!("{root}ah"),
    };
    noun_rules(&result)
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

fn declension_plural_noun(word: &str, add: &str, gender: Gender) -> Option<Paradigm> {
    let word_without_last = slice_chars(word, 0, char_len(word).saturating_sub(1));
    let mut p = empty_paradigm();
    if add.ends_with("yh") || add.ends_with("ih") {
        let iy = if add.ends_with("yh") { "y" } else { "i" };
        p[0] = [None, Some(word.to_string())];
        p[1] = [None, Some(word.to_string())];
        p[2] = [None, Some(format!("{word_without_last}{iy}h"))];
        p[3] = [None, Some(format!("{word_without_last}{iy}h"))];
        p[4] = [None, Some(format!("{word_without_last}{iy}m"))];
        p[5] = [None, Some(format!("{word_without_last}{iy}mi"))];
        p[6] = [None, Some(word.to_string())];
        Some(p)
    } else if !add.is_empty() {
        None
    } else if gender == Gender::Masculine && word.ends_with(['i', 'y', 'e']) {
        p[0] = [None, Some(word.to_string())];
        p[1] = [None, Some(word.to_string())];
        p[2] = [None, Some(format!("{word_without_last}ov"))];
        p[3] = [None, Some(format!("{word_without_last}ah"))];
        p[4] = [None, Some(format!("{word_without_last}am"))];
        p[5] = [None, Some(format!("{word_without_last}ami"))];
        p[6] = [None, Some(word.to_string())];
        Some(p)
    } else if (gender == Gender::Feminine && word.ends_with(['y', 'e']))
        || (gender == Gender::Neuter && word.ends_with('a'))
    {
        p[0] = [None, Some(word.to_string())];
        p[1] = [None, Some(word.to_string())];
        p[2] = [
            None,
            Some(noun_rules(&plural_gen_ending(&format!(
                "{word_without_last}%"
            )))),
        ];
        p[3] = [None, Some(format!("{word_without_last}ah"))];
        p[4] = [None, Some(format!("{word_without_last}am"))];
        p[5] = [None, Some(format!("{word_without_last}ami"))];
        p[6] = [None, Some(word.to_string())];
        Some(p)
    } else if gender == Gender::Feminine && word.ends_with('i') {
        p[0] = [None, Some(word.to_string())];
        p[1] = [None, Some(word.to_string())];
        p[2] = [None, Some(format!("{word_without_last}ij"))];
        p[3] = [None, Some(format!("{word_without_last}ah"))];
        p[4] = [None, Some(format!("{word_without_last}am"))];
        p[5] = [None, Some(format!("{word_without_last}ami"))];
        p[6] = [None, Some(word.to_string())];
        Some(p)
    } else {
        None
    }
}

fn maybe_declension_subst_adj(
    word: &str,
    add: &str,
    gender: Gender,
    meta: Option<NounMeta>,
) -> Option<Paradigm> {
    if add.is_empty() {
        return None;
    }
    let stem = slice_chars(word, 0, char_len(word).saturating_sub(1));
    let normalized = if ["-ogo", "-ego", "-oj", "-ej"].contains(&add) {
        add.to_string()
    } else {
        add.strip_prefix(&stem).map(|suffix| format!("-{suffix}"))?
    };
    if !["-ogo", "-ego", "-oj", "-ej"].contains(&normalized.as_str()) {
        return None;
    }

    let animacy = meta.map_or(Animacy::Inanimate, |m| m.animacy);
    let adj = if gender == Gender::Feminine {
        format!("{}{}", stem, if normalized == "-oj" { "y" } else { "i" })
    } else {
        format!("{}{}", stem, if normalized == "-ogo" { "y" } else { "i" })
    };
    let mut p = empty_paradigm();
    for case in [
        Case::Nom,
        Case::Acc,
        Case::Gen,
        Case::Loc,
        Case::Dat,
        Case::Ins,
        Case::Voc,
    ] {
        let idx = case_index(case);
        p[idx][0] = Some(if case == Case::Nom || case == Case::Voc {
            word.to_string()
        } else {
            InterslavicCore::adjective(&adj, case, Number::Singular, gender, animacy)
        });
        p[idx][1] = Some(InterslavicCore::adjective(
            &adj,
            case,
            Number::Plural,
            gender,
            animacy,
        ));
    }
    Some(p)
}

fn plural_gen_ending(word: &str) -> String {
    let mut out = word
        .replace("jsk%", "jsk")
        .replace("mš%", "meš")
        .replace("zl%", "zȯl")
        .replace("tl%", "tȯl")
        .replace("mgl%", "mgȯl")
        .replace("ńj%", "nij");
    out = re_soft_k_percent().replace_all(&out, "$1e$2").to_string();
    out = re_hard_k_percent().replace_all(&out, "$1ȯ$2").to_string();
    out = re_vmpzn_percent().replace_all(&out, "$1e$2").to_string();
    out = re_k_nl_percent().replace_all(&out, "$1ȯ$2").to_string();
    out = re_s_nl_percent().replace_all(&out, "$1e$2").to_string();
    out.replace("dn%", "dȯn")
        .replace("pismo%", "pisem")
        .replace("ťm%", "tem")
        .replace("sto%", "sȯt")
        .replace('%', "")
}

fn palatalization_ending(root: &str) -> String {
    if let Some(prefix) = root.strip_suffix('g') {
        format!("{prefix}žь")
    } else if let Some(prefix) = root.strip_suffix('h') {
        format!("{prefix}šь")
    } else if let Some(prefix) = root.strip_suffix('k') {
        format!("{prefix}čь")
    } else if let Some(prefix) = root.strip_suffix("cь") {
        format!("{prefix}čь")
    } else {
        root.to_string()
    }
}

fn xv_ending(root: &str) -> bool {
    static XV: LazyLock<Regex> =
        LazyLock::new(|| Regex::new(r"(?:[^aåeęěėioȯuųyl]|[^oȯ]l)v$").unwrap());
    XV.is_match(root)
}

fn ec_ending(s: &str) -> bool {
    s.ends_with("ec") || s.ends_with("ėc") || s.ends_with("èc")
}

fn re_j_percent() -> &'static Regex {
    static RE: LazyLock<Regex> =
        LazyLock::new(|| Regex::new(r"([pbvfmlnrszńľŕťďśźščžđ])j%").unwrap());
    &RE
}
fn re_soft_k_percent() -> &'static Regex {
    static RE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"([jśźďťľŕńčšžćđc])(k)%").unwrap());
    &RE
}
fn re_hard_k_percent() -> &'static Regex {
    static RE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"([pbfvmlnrtdszkgh])(k)%").unwrap());
    &RE
}
fn re_vmpzn_percent() -> &'static Regex {
    static RE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"([vmpzšžt])(n)%").unwrap());
    &RE
}
fn re_k_nl_percent() -> &'static Regex {
    static RE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"(k)([nl])%").unwrap());
    &RE
}
fn re_s_nl_percent() -> &'static Regex {
    static RE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"(s)([nl])%").unwrap());
    &RE
}

#[allow(dead_code)]
fn cap<'a>(caps: &'a Captures<'a>, idx: usize) -> &'a str {
    caps.get(idx).map_or("", |m| m.as_str())
}
