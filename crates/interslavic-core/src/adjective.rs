use crate::grammar::{Animacy, Case, Gender, Number};
use crate::utils::{char_at, char_len, slice_chars};
use crate::InterslavicCore;
use regex::Regex;
use std::sync::LazyLock;

pub type Adjective = String;

impl InterslavicCore {
    pub fn adjective(
        word: &str,
        case: Case,
        number: Number,
        gender: Gender,
        animacy: Animacy,
    ) -> Adjective {
        adjective_form(word, case, number, gender, animacy)
    }

    pub fn stem_of_adj_is_soft(word: &str) -> bool {
        establish_root(word).contains('^') || word.ends_with('i')
    }

    pub fn get_adj_stem(word: &str) -> String {
        establish_root(word).replace(['|', '^'], "")
    }
}

fn adjective_form(
    adj: &str,
    case: Case,
    number: Number,
    gender: Gender,
    animacy: Animacy,
) -> String {
    let mut adj = adj.to_lowercase();
    let mut postfix = String::new();
    if adj.contains(' ') {
        let words: Vec<&str> = adj.split(' ').collect();
        if let Some(index) = words.iter().position(|word| looks_like_adjective(word)) {
            if index < words.len() - 1 {
                postfix = format!(" {}", words[index + 1..].join(" "));
                adj = words[..=index].join(" ");
            }
        }
    }

    let root = establish_root(&adj);
    let base = match number {
        Number::Singular => match case {
            Case::Nom | Case::Voc => match gender {
                Gender::Masculine => m_nominative_sg(&adj, &root),
                Gender::Neuter => n_nominative_sg(&root),
                Gender::Feminine => f_nominative_sg(&root),
            },
            Case::Acc => match gender {
                Gender::Masculine => {
                    split_choice(&m_accusative_sg(&adj, &root), animacy == Animacy::Animate)
                }
                Gender::Neuter => n_nominative_sg(&root),
                Gender::Feminine => f_accusative_sg(&root),
            },
            Case::Gen => match gender {
                Gender::Feminine => f_gendatloc_sg(&root),
                Gender::Masculine | Gender::Neuter => mn_genitive_sg(&root),
            },
            Case::Loc => match gender {
                Gender::Feminine => f_gendatloc_sg(&root),
                Gender::Masculine | Gender::Neuter => mn_locative_sg(&root),
            },
            Case::Dat => match gender {
                Gender::Feminine => f_gendatloc_sg(&root),
                Gender::Masculine | Gender::Neuter => mn_dative_sg(&root),
            },
            Case::Ins => match gender {
                Gender::Feminine => f_instrumental_sg(&root),
                Gender::Masculine | Gender::Neuter => mn_instrumental_sg(&root),
            },
        },
        Number::Plural => match case {
            Case::Nom | Case::Voc => match gender {
                Gender::Masculine => {
                    split_choice(&m_nominative_pl(&root), animacy == Animacy::Animate)
                }
                Gender::Feminine | Gender::Neuter => fn_nominative_pl(&root),
            },
            Case::Acc => match gender {
                Gender::Masculine => {
                    split_choice(&m_accusative_pl(&root), animacy == Animacy::Animate)
                }
                Gender::Feminine | Gender::Neuter => fn_nominative_pl(&root),
            },
            Case::Gen | Case::Loc => genloc_pl(&root),
            Case::Dat => dative_pl(&root),
            Case::Ins => instrumental_pl(&root),
        },
    };

    rules(&base).replace('$', &postfix)
}

fn looks_like_adjective(word: &str) -> bool {
    word.ends_with(['y', 'i', 'j'])
}

fn split_choice(value: &str, first: bool) -> String {
    let mut parts = value.split('/').map(str::trim);
    if first {
        parts.next().unwrap_or(value).to_string()
    } else {
        parts.nth(1).unwrap_or(value).to_string()
    }
}

fn establish_root(adj: &str) -> String {
    let rules: &[(fn(&str) -> Option<String>, &str)] = &[
        (rule_sej, ""),
        (rule_ves, ""),
        (rule_ovoj, ""),
        (rule_jedin, ""),
        (rule_oj, ""),
        (rule_fleeting, ""),
        (rule_en, ""),
        (rule_rad_in_ev, ""),
        (rule_toj, ""),
        (rule_soft_possessive, ""),
        (rule_i, ""),
        (rule_y, ""),
    ];
    for (rule, _) in rules {
        if let Some(result) = rule(adj) {
            return kgh_caret(&result);
        }
    }
    String::new()
}

fn rule_sej(adj: &str) -> Option<String> {
    if matches!(adj, "sej" | "sėj" | "sjej") {
        Some("s|^".into())
    } else {
        None
    }
}
fn rule_ves(adj: &str) -> Option<String> {
    if matches!(adj, "ves" | "veś" | "vės" | "vėś") {
        Some("vs|^".into())
    } else {
        None
    }
}
fn rule_ovoj(adj: &str) -> Option<String> {
    static RE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"^(o[vn])(?:oj)?$").unwrap());
    RE.captures(adj).map(|c| format!("{}|", &c[1]))
}
fn rule_jedin(adj: &str) -> Option<String> {
    adj.strip_suffix("jedin").map(|p| format!("{p}jedn|"))
}
fn rule_oj(adj: &str) -> Option<String> {
    adj.strip_suffix('j')
        .and_then(|p| p.strip_suffix(['ė', 'ȯ']))
        .map(|p| format!("{p}|"))
}
fn rule_fleeting(adj: &str) -> Option<String> {
    let chars: Vec<char> = adj.chars().collect();
    if chars.len() >= 2 {
        let prev = chars[chars.len() - 2];
        if matches!(prev, 'ė' | 'ȯ') {
            let tail: String = chars[chars.len() - 1..].iter().collect();
            return Some(format!("{}{}|", slice_chars(adj, 0, chars.len() - 2), tail));
        }
    }
    if adj.ends_with("ėlj") || adj.ends_with("ȯlj") || adj.ends_with("ėnj") || adj.ends_with("ȯnj")
    {
        let len = char_len(adj);
        return Some(format!(
            "{}{}|",
            slice_chars(adj, 0, len - 3),
            slice_chars(adj, len - 2, len)
        ));
    }
    None
}
fn rule_en(adj: &str) -> Option<String> {
    adj.strip_suffix("en").map(|p| format!("{p}n|"))
}
fn rule_rad_in_ev(adj: &str) -> Option<String> {
    if adj.ends_with("rad") || adj.ends_with("in") || adj.ends_with("ev") || adj.ends_with("ov") {
        Some(format!("{adj}|"))
    } else {
        None
    }
}
fn rule_toj(adj: &str) -> Option<String> {
    if adj.ends_with("toj") || adj.ends_with("tȯj") {
        Some(format!("{}|", slice_chars(adj, 0, char_len(adj) - 2)))
    } else {
        None
    }
}
fn rule_soft_possessive(adj: &str) -> Option<String> {
    if adj == "naš" || adj == "vaš" || adj.ends_with("čij") || adj.ends_with("oj") {
        Some(format!("{adj}|^"))
    } else {
        None
    }
}
fn rule_i(adj: &str) -> Option<String> {
    if adj.ends_with('i') || adj.ends_with("ij") {
        let stem = if adj.ends_with("ij") {
            slice_chars(adj, 0, char_len(adj) - 2)
        } else {
            slice_chars(adj, 0, char_len(adj) - 1)
        };
        Some(format!("{stem}^"))
    } else {
        None
    }
}
fn rule_y(adj: &str) -> Option<String> {
    if adj.ends_with('y') || adj.ends_with("yj") {
        let stem = if adj.ends_with("yj") {
            slice_chars(adj, 0, char_len(adj) - 2)
        } else {
            slice_chars(adj, 0, char_len(adj) - 1)
        };
        Some(stem)
    } else {
        None
    }
}

fn kgh_caret(root: &str) -> String {
    root.replace("k^", "k")
        .replace("g^", "g")
        .replace("h^", "h")
}

fn m_nominative_sg(adj: &str, root: &str) -> String {
    if root.contains('|') {
        adj.to_string()
    } else {
        format!("{root}y")
    }
}
fn f_nominative_sg(root: &str) -> String {
    format!("{root}a")
}
fn n_nominative_sg(root: &str) -> String {
    format!("{root}o")
}
fn mn_genitive_sg(root: &str) -> String {
    format!("{root}ogo")
}
fn mn_dative_sg(root: &str) -> String {
    format!("{root}omu")
}
fn m_accusative_sg(adj: &str, root: &str) -> String {
    format!("{root}ogo / {adj}")
}
fn mn_instrumental_sg(root: &str) -> String {
    format!("{root}ym")
}
fn mn_locative_sg(root: &str) -> String {
    format!("{root}om")
}
fn f_accusative_sg(root: &str) -> String {
    format!("{root}ų")
}
fn f_gendatloc_sg(root: &str) -> String {
    format!("{root}oj")
}
fn f_instrumental_sg(root: &str) -> String {
    format!("{root}ojų")
}
fn m_nominative_pl(root: &str) -> String {
    format!("{root}i / {root}e")
}
fn m_accusative_pl(root: &str) -> String {
    format!("{root}yh / {root}e")
}
fn fn_nominative_pl(root: &str) -> String {
    format!("{root}e")
}
fn genloc_pl(root: &str) -> String {
    format!("{root}yh")
}
fn dative_pl(root: &str) -> String {
    format!("{root}ym")
}
fn instrumental_pl(root: &str) -> String {
    format!("{root}ymi")
}

fn rules(word: &str) -> String {
    word.replace("^o", "^e")
        .replace("^y", "^i")
        .replace("s|^e", "se")
        .replace("s|^i", "si")
        .replace('|', "")
        .replace("l^", "lj")
        .replace("n^", "ń")
        .replace("r^", "ŕ")
        .replace("j^", "j")
        .replace("t^", "ť")
        .replace("d^", "ď")
        .replace("s^", "ś")
        .replace("z^", "ź")
        .replace('^', "")
        .replace("jy", "ji")
        .replace("cy", "ci")
}

#[allow(dead_code)]
fn _last_char(s: &str) -> Option<char> {
    char_at(s, char_len(s).saturating_sub(1))
}
