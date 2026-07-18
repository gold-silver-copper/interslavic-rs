//! Regular Interslavic word formation: generate a lemma's derivational family.
//!
//! Interslavic derivation is regular and documented. From one citation form
//! [`derive`] produces the regular derivatives — the abstract noun `-osť`, the
//! adverb, the verbal noun `-ńje`, the agentive `-telj` (and its `-teljstvo`/
//! `-teljka` family), the denominal adjectives `-ny`/`-sky`, the feminine
//! diminutive `-ka`/`-ica`, and negation `ne-` — applying the seam
//! morphophonemics from [`crate::phono`]: first palatalization before the
//! adjective/diminutive suffixes, iotation before `-jeńje`, and the O⇒E adverb
//! alternation after a soft stem.
//!
//! Only patterns whose preconditions hold fire; the caller filters the result
//! against attestation (a dictionary, a corpus) if it wants only real words.

use crate::phono::{iotate_final, is_soft, palatalize_final};

/// Part of speech, as consumed and produced by [`derive`]. Deriving from a
/// part of speech other than a noun, adjective, or verb yields nothing.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Pos {
    Noun,
    Adjective,
    Verb,
    Adverb,
}

/// One derived family member.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Derived {
    /// The derived form (flavored orthography).
    pub form: String,
    /// The part of speech of the derived form.
    pub pos: Pos,
    /// Stable pattern id: `"ost"`, `"adv"`, `"vnoun"`, `"telj"`, `"ny"`, `"sky"`,
    /// `"dimka"`, `"ica"`, `"ne"`, `"teljstvo"`, `"teljka"`.
    pub pattern: &'static str,
    /// The Interslavic name of the pattern (e.g. `odvlečeny imennik`).
    pub label: &'static str,
}

/// A stem counts as soft for the O⇒E adverb alternation (a final `rj` counts
/// too, beyond [`crate::phono::is_soft`]).
fn ends_soft(stem: &str) -> bool {
    is_soft(stem) || stem.ends_with("rj")
}

fn strip_final_vowel(w: &str) -> &str {
    match w.chars().last() {
        Some('a' | 'o' | 'e' | 'y' | 'i') => &w[..w.len() - w.chars().last().unwrap().len_utf8()],
        _ => w,
    }
}

/// The regular derivational family of a lemma, seam-aware. `base` is the
/// citation form and `pos` its part of speech.
///
/// ```
/// use interslavic_core::derivation::{derive, Pos};
/// let fam = derive("dobry", Pos::Adjective);
/// assert!(fam.iter().any(|d| d.form == "dobrosť")); // abstract noun
/// assert!(fam.iter().any(|d| d.form == "dobro"));   // adverb
/// assert!(fam.iter().any(|d| d.form == "nedobry")); // negation
///
/// let verb = derive("učiti", Pos::Verb);
/// assert!(verb.iter().any(|d| d.form == "učeńje"));  // verbal noun (iotated)
/// assert!(verb.iter().any(|d| d.form == "učitelj")); // agentive
///
/// let noun = derive("kniga", Pos::Noun);
/// assert!(noun.iter().any(|d| d.form == "knižny"));  // denominal adj (palatalized)
/// assert!(noun.iter().any(|d| d.form == "knižka"));  // diminutive
/// ```
pub fn derive(base: &str, pos: Pos) -> Vec<Derived> {
    let mut out = Vec::new();
    let b = base.trim();
    if b.is_empty() || b.contains(' ') {
        return out;
    }
    let push = |out: &mut Vec<Derived>, form: String, pos, pattern, label| {
        if !form.is_empty() && form != b {
            out.push(Derived {
                form,
                pos,
                pattern,
                label,
            });
        }
    };

    match pos {
        Pos::Adjective => {
            let stem = strip_final_vowel(b).to_string();
            if stem.chars().count() >= 2 {
                // Abstract noun: dobry → dobrosť.
                push(
                    &mut out,
                    format!("{stem}osť"),
                    Pos::Noun,
                    "ost",
                    "odvlečeny imennik",
                );
                // Adverb: neut.sg -o, -e after a soft stem (svěži → svěže).
                let adv_end = if ends_soft(&stem) { "e" } else { "o" };
                push(
                    &mut out,
                    format!("{stem}{adv_end}"),
                    Pos::Adverb,
                    "adv",
                    "prislovnik",
                );
                // Negation: dobry → nedobry.
                if !b.starts_with("ne") {
                    push(&mut out, format!("ne{b}"), Pos::Adjective, "ne", "negacija");
                }
            }
        }
        Pos::Verb => {
            if let Some(stem) = b.strip_suffix("ti") {
                // Verbal noun: -ati→-ańje, -ěti→-ěńje; -iti → iotated stem +
                // -jeńje (prositi→prošeńje, roditi→rođeńje, loviti→lovjeńje).
                if stem.ends_with('a') || stem.ends_with('ě') {
                    push(
                        &mut out,
                        format!("{stem}ńje"),
                        Pos::Noun,
                        "vnoun",
                        "odglagolny imennik",
                    );
                } else if let Some(istem) = stem.strip_suffix('i') {
                    // Root i-verbs (piti, biti, žiti) take -ťje, not the iotated
                    // -jeńje; only derive suffixal -iti stems (≥2 chars).
                    if istem.chars().count() >= 2 {
                        push(
                            &mut out,
                            format!("{}eńje", iotate_final(istem)),
                            Pos::Noun,
                            "vnoun",
                            "odglagolny imennik",
                        );
                    }
                }
                // Agentive: učiti → učitelj, izdavati → izdavatelj.
                if stem.chars().count() >= 2 && !stem.ends_with('n') {
                    push(
                        &mut out,
                        format!("{stem}telj"),
                        Pos::Noun,
                        "telj",
                        "dějatelj",
                    );
                }
            }
        }
        Pos::Noun => {
            if b.ends_with("telj") {
                // Agent-noun family: -teljstvo, -teljka.
                push(
                    &mut out,
                    format!("{b}stvo"),
                    Pos::Noun,
                    "teljstvo",
                    "odvlečeny imennik",
                );
                push(
                    &mut out,
                    format!("{b}ka"),
                    Pos::Noun,
                    "teljka",
                    "žensky dějatelj",
                );
            }
            let stem = strip_final_vowel(b).to_string();
            if stem.chars().count() >= 2 {
                // Denominal adjectives with first palatalization at the seam:
                // kniga → knižny, Grek → grečsky.
                push(
                    &mut out,
                    format!("{}ny", palatalize_final(&stem)),
                    Pos::Adjective,
                    "ny",
                    "pridavnik",
                );
                push(
                    &mut out,
                    format!("{}sky", palatalize_final(&stem)),
                    Pos::Adjective,
                    "sky",
                    "pridavnik",
                );
            }
            if let Some(astem) = b
                .strip_suffix('a')
                .filter(|astem| astem.chars().count() >= 2)
            {
                // Feminine diminutive: kniga → knižka, ruka → ručka.
                push(
                    &mut out,
                    format!("{}ka", palatalize_final(astem)),
                    Pos::Noun,
                    "dimka",
                    "umenšeny imennik",
                );
                // -ica: voda → vodica, ruka → ručica.
                push(
                    &mut out,
                    format!("{}ica", palatalize_final(astem)),
                    Pos::Noun,
                    "ica",
                    "umenšeny/žensky imennik",
                );
            }
        }
        Pos::Adverb => {}
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    fn forms(base: &str, pos: Pos) -> Vec<String> {
        derive(base, pos).into_iter().map(|d| d.form).collect()
    }

    #[test]
    fn adjective_family() {
        let f = forms("dobry", Pos::Adjective);
        assert!(f.contains(&"dobrosť".to_string()));
        assert!(f.contains(&"dobro".to_string())); // hard-stem adverb -o
        assert!(f.contains(&"nedobry".to_string()));
        // Soft stem takes the -e adverb.
        assert!(forms("svěži", Pos::Adjective).contains(&"svěže".to_string()));
        // Already-negated adjectives are not doubly negated.
        assert!(
            !forms("nemožny", Pos::Adjective)
                .iter()
                .any(|f| f.starts_with("nene"))
        );
    }

    #[test]
    fn verb_family_iotates_the_verbal_noun() {
        assert!(forms("dělati", Pos::Verb).contains(&"dělańje".to_string()));
        assert!(forms("prositi", Pos::Verb).contains(&"prošeńje".to_string())); // s→š
        assert!(forms("loviti", Pos::Verb).contains(&"lovjeńje".to_string())); // v→vj
        assert!(forms("učiti", Pos::Verb).contains(&"učeńje".to_string())); // č passthrough
        assert!(forms("učiti", Pos::Verb).contains(&"učitelj".to_string()));
    }

    #[test]
    fn noun_family_palatalizes_the_seam() {
        let f = forms("kniga", Pos::Noun);
        assert!(f.contains(&"knižny".to_string())); // g→ž
        assert!(f.contains(&"knižsky".to_string()));
        assert!(f.contains(&"knižka".to_string()));
        assert!(f.contains(&"knižica".to_string()));
        // Agent-noun family only for -telj nouns.
        assert!(forms("učitelj", Pos::Noun).contains(&"učiteljstvo".to_string()));
    }

    #[test]
    fn empty_and_multiword_and_adverb_derive_nothing() {
        assert!(derive("", Pos::Adjective).is_empty());
        assert!(derive("bez obzira", Pos::Adjective).is_empty());
        assert!(derive("dobro", Pos::Adverb).is_empty());
    }
}
