//! Adjective declension, comparison, pronouns, and numerals.

use crate::case_endings::*;
use crate::{Animacy, Case, Gender, Number, noun};

pub fn decline_adj(
    word: &str,
    case: Case,
    number: Number,
    gender: Gender,
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
        && gender == Gender::Masculine
        && animacy != Animacy::Animate
        && case == Case::Acc
        && number == Number::Singular
    {
        return "seksi".into();
    }
    let stem_is_soft = stem_of_adj_is_soft(&word);
    let adj_stem = get_adj_stem(&word);
    let has_long_form_ending = word.ends_with('y') || word.ends_with('i') || word.ends_with('j');

    if !has_long_form_ending
        && gender == Gender::Masculine
        && number == Number::Singular
        && (case == Case::Nom || (animacy != Animacy::Animate && case == Case::Acc))
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
        noun::infer_fluent_vowel(word)
            .replace("(e)", "")
            .replace("(o)", "")
    }
}

/// The synthetic comparative of an adjective, as `(comparative adjective,
/// comparative adverb)`.
///
/// Returns `None` for adjectives that do not gradate synthetically — the
/// relational `-sky`/`-cky` type, forms that already are comparatives or
/// participial adjectives (`-ši`/`-ći`), and the soft `-ji` possessives —
/// for which the analytic comparative (`vyše`/`bolje` + the positive) is
/// used instead. The returned comparative is itself a soft adjective, so
/// its full paradigm is `decline_adj(comparative, …)`; the superlative is
/// `naj-` prefixed (see [`superlative`]).
///
/// Rules: seven lexical irregulars (dobry→lěpši, zly→gorši, …); the
/// `-ky`/`-eky`/`-oky` class takes `-ši` on the truncated root with an
/// iotated adverb (daleky→dalši/dalje, vysoky→vysši/vyše); otherwise the
/// seam is palatalized and the softness of the result picks `-ějši`/`-ěje`
/// (hard) vs `-ejši`/`-eje` (soft), e.g. novy→novějši, blagy stays
/// irregular, ryđi→ryđejši.
///
/// Precondition: `adj` is a positive-degree qualitative adjective written
/// in the flavored (etymological) orthography; the result is unspecified
/// for other input (a verb infinitive also ends in `-i` and would gradate
/// spuriously). Determiners and other non-gradable words should not be
/// passed here.
pub fn comparative(adj: &str) -> Option<(String, String)> {
    // Non-gradable: relational -sky/-cky, already-comparative/participial
    // -ši/-ći, and soft -ji possessives (ji+ejši would be malformed).
    if adj.ends_with("sky")
        || adj.ends_with("cky")
        || adj.ends_with("ši")
        || adj.ends_with("ći")
        || adj.ends_with("ji")
    {
        return None;
    }
    // Seven lexical irregulars, matched on the full lemma.
    for (base, comp, adv) in [
        ("dobry", "lěpši", "lěpje"),
        ("zly", "gorši", "gorje"),
        ("veliky", "večši", "veče"),
        ("maly", "menši", "menje"),
        ("blagy", "unši", "unje"),
        ("legky", "legši", "legše"),
        ("mękky", "mękši", "mękše"),
    ] {
        if adj == base {
            return Some((comp.to_string(), adv.to_string()));
        }
    }
    let stem = adj.strip_suffix(['y', 'i'])?;
    if stem.chars().count() < 2 {
        return None;
    }
    // -ky / -eky / -oky class: -ši on the truncated root (kratky→kratši,
    // vysoky→vysši, uzky→uzši), adverb by iotation of the root
    // (kratky→kraće, vysoky→vyše, uzky→uže). A few adjectives end in -ky
    // with the k belonging to the ROOT rather than the -ky suffix; they
    // palatalize regularly instead (diky→dičejši, not *diši), so they are
    // excluded here and fall through to the regular rule below. (Root
    // length cannot tell them apart — both diky and uzky leave a 2-char
    // root — so the distinction is lexical.)
    const ROOT_FINAL_K: &[&str] = &["diky"];
    if !ROOT_FINAL_K.contains(&adj) {
        for suf in ["ok", "ek", "k"] {
            if let Some(root) = stem.strip_suffix(suf) {
                if root.chars().count() >= 2 {
                    let comp = format!("{root}ši");
                    let adv = format!("{}e", crate::phono::iotate_final(root));
                    return Some((comp, adv));
                }
                break;
            }
        }
    }
    // Regular: palatalize the seam, then the full softness predicate picks
    // the soft (-ejši) vs hard (-ějši) ending.
    let pal = crate::phono::palatalize_final(stem);
    let (adj_suf, adv_suf) = if crate::phono::is_soft(&pal) {
        ("ejši", "eje")
    } else {
        ("ějši", "ěje")
    };
    Some((format!("{pal}{adj_suf}"), format!("{pal}{adv_suf}")))
}

/// The synthetic superlative of an adjective, as `(superlative adjective,
/// superlative adverb)` — the [`comparative`] with the `naj-`
/// prefix. `None` when the adjective does not gradate synthetically.
pub fn superlative(adj: &str) -> Option<(String, String)> {
    comparative(adj).map(|(c, a)| (format!("naj{c}"), format!("naj{a}")))
}

/// The pronominal (hard `toj` / soft `moj`) declension of a synthetic
/// adjective lemma, with the masculine nominative singular — and the
/// masculine inanimate accusative singular, which is syncretic with it —
/// overridden to the pronoun's actual nominative.
///
/// The oblique cells of the pronominal declension are identical to the
/// adjectival ones (togo≡the gen.sg of hard `ty`, mojego≡the gen.sg of
/// soft `moji`), so `decline_adj` on a synthetic `stem+y`/`stem+i` lemma
/// produces them; only the two nominative-syncretic masculine cells
/// differ (a pronoun has `toj`, not `*ty`).
fn pronominal_via_adj(
    synthetic: &str,
    nom_masc: &str,
    case: Case,
    number: Number,
    gender: Gender,
    animacy: Animacy,
) -> String {
    let is_masc_nom_sg = gender == Gender::Masculine
        && number == Number::Singular
        && (case == Case::Nom || (case == Case::Acc && animacy == Animacy::Inanimate));
    if is_masc_nom_sg {
        nom_masc.to_string()
    } else {
        decline_adj(synthetic, case, number, gender, animacy)
    }
}

/// One pronoun form, or `None` if `lemma` is not a recognized pronoun
/// paradigm. Covers the `toj`-class demonstratives, the `moj`-class
/// possessives/interrogatives (incl. `naš`/`vaš`/`čij`), `kto`/`čto` and
/// their derivatives, the internally-inflecting `-koli` indefinites,
/// `veś`, and the adjectivally-declined determiners (`ktory`, `kaky`,
/// `samy`, …).
///
/// Recognition is by word SHAPE, not a closed lexicon: the last-resort
/// branch declines any `-y`/`-i` word as an adjective, so a same-shaped
/// non-pronoun yields a (correctly-declined) `Some` rather than `None`.
/// Lemmas are the flavored (etymological) citation forms.
pub fn decline_pronoun(
    lemma: &str,
    case: Case,
    number: Number,
    gender: Gender,
    animacy: Animacy,
) -> Option<String> {
    let l = lemma.trim();
    if l.is_empty() || l.contains(' ') {
        return None;
    }
    // -koli indefinites inflect internally on the head, then re-append the
    // particle. Strip iteratively (not recursively) so a pathological
    // "kolikoli…" input cannot overflow the stack.
    let mut head = l;
    let mut koli = 0usize;
    while let Some(rest) = head.strip_suffix("koli") {
        if rest.is_empty() {
            break;
        }
        head = rest;
        koli += 1;
    }
    if koli > 0 {
        return decline_pronoun(head, case, number, gender, animacy)
            .map(|f| format!("{f}{}", "koli".repeat(koli)));
    }
    // toj-class demonstratives: hard pronominal declension.
    if matches!(l, "toj" | "tutoj" | "tamtoj" | "onoj" | "ov") {
        let stem = l.strip_suffix("oj").unwrap_or(l);
        let synthetic = format!("{stem}y");
        return Some(pronominal_via_adj(
            &synthetic, l, case, number, gender, animacy,
        ));
    }
    // kto and its derivatives (nikto, něstokto, vsekto, …).
    if let Some(head) = l.strip_suffix("kto") {
        return Some(match case {
            Case::Nom => l.to_string(),
            Case::Gen | Case::Acc => format!("{head}kogo"),
            Case::Dat => format!("{head}komu"),
            Case::Ins => format!("{head}kym"),
            Case::Loc => format!("{head}kom"),
        });
    }
    // čto / što and derivatives (inanimate: accusative = nominative).
    if let Some(head) = l.strip_suffix("čto").or_else(|| l.strip_suffix("što")) {
        return Some(match case {
            Case::Nom | Case::Acc => l.to_string(),
            Case::Gen => format!("{head}čego"),
            Case::Dat => format!("{head}čemu"),
            Case::Ins => format!("{head}čim"),
            Case::Loc => format!("{head}čem"),
        });
    }
    // veś "all, whole": soft pronominal declension over the vs- stem.
    if l == "veś" || l == "ves" {
        return Some(pronominal_via_adj("vsi", l, case, number, gender, animacy));
    }
    // moj-class possessives/interrogatives (moj, tvoj, svoj, koj, čij, naš,
    // vaš, and their ni-/ně-/vse- prefixed derivatives): soft pronominal
    // declension over the full lemma. The length guard excludes the bare
    // conjunction "oj".
    if (l.ends_with("oj") && l.chars().count() >= 3)
        || l.ends_with("čij")
        || l == "naš"
        || l == "vaš"
    {
        let synthetic = format!("{l}i");
        return Some(pronominal_via_adj(
            &synthetic, l, case, number, gender, animacy,
        ));
    }
    // Adjectivally-declined determiners (ktory, kaky, samy, vsaky, iny…).
    if l.ends_with(['y', 'i']) && l.chars().count() >= 3 {
        return Some(decline_adj(l, case, number, gender, animacy));
    }
    None
}

/// One numeral form, or `None` if `lemma` is not a recognized numeral.
/// Covers `jedin` (adjectival, with the irregular masculine nominative),
/// the dual-remnant `dva`/`oba`/`obydva` and `tri`/`četyri`, the i-stem
/// numerals `pęť`…`desęť` (declining like `kosť`), and the adjectivally-
/// declined ordinals (`pŕvy`, `drugy`, …). Cardinals return their citation
/// form for nominative/accusative.
///
/// Recognition is by word SHAPE (an i-stem lemma is detected by its final
/// `-ť`, an ordinal by its `-y`/`-i`), not a closed lexicon, so a same-
/// shaped non-numeral — a feminine i-stem noun like `kosť` — yields a
/// (correctly-declined) `Some`. Lemmas are the flavored citation forms
/// (`desęť`, not `deset`).
pub fn decline_numeral(
    lemma: &str,
    case: Case,
    number: Number,
    gender: Gender,
    animacy: Animacy,
) -> Option<String> {
    let l = lemma.trim();
    if l.is_empty() || l.contains(' ') {
        return None;
    }
    // jedin: declines like the hard adjective *jedny, except the masculine
    // nominative singular is the irregular citation form jedin.
    if l == "jedin" {
        return Some(pronominal_via_adj(
            "jedny", "jedin", case, number, gender, animacy,
        ));
    }
    // Dual remnants 2 (gender only in nom/acc), oba/obydva likewise.
    for (base, stem) in [("dva", "dv"), ("oba", "ob"), ("obydva", "obydv")] {
        if l == base {
            return Some(match case {
                Case::Nom | Case::Acc => {
                    if gender == Gender::Feminine {
                        format!("{stem}ě")
                    } else {
                        l.to_string()
                    }
                }
                Case::Gen | Case::Loc => format!("{stem}oh"),
                Case::Dat => format!("{stem}om"),
                Case::Ins => format!("{stem}oma"),
            });
        }
    }
    // 3 and 4: dual-remnant declension, no gender.
    if l == "tri" || l == "četyri" {
        let stem = l.strip_suffix('i').unwrap_or(l);
        return Some(match case {
            Case::Nom | Case::Acc => l.to_string(),
            Case::Gen | Case::Loc => format!("{stem}ěh"),
            Case::Dat => format!("{stem}ěm"),
            Case::Ins => format!("{stem}ěmi"),
        });
    }
    // 5 and up (pęť…desęť and the -nadsęť/-deset series): the i-stem
    // (kosť-class) declension.
    if let Some(stem) = l.strip_suffix('ť')
        && stem.chars().count() >= 2
    {
        return Some(match case {
            Case::Nom | Case::Acc => l.to_string(),
            Case::Gen | Case::Dat | Case::Loc => format!("{stem}ti"),
            Case::Ins => format!("{l}jų"),
        });
    }
    // Ordinals and other adjectivally-shaped numerals (pŕvy, drugy…).
    if l.ends_with(['y', 'i']) && l.chars().count() >= 3 {
        return Some(decline_adj(l, case, number, gender, animacy));
    }
    None
}
