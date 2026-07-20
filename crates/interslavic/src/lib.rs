//! Interslavic inflection with embedded dictionary metadata.
//!
//! `interslavic-core` contains the dependency-free morphology rules. This crate
//! adds generated dictionary metadata for noun and verb lookup and keeps the public
//! API focused on single-form inflection.
//!
//! # Examples
//!
//! ```
//! use interslavic::*;
//!
//! assert_eq!(interslavic::noun("adept", Case::Acc, Number::Singular), "adepta");
//! assert_eq!(interslavic::noun("oko", Case::Nom, Number::Plural), "oči / očesa");
//! assert_eq!(
//!     interslavic::adj(
//!         "dobry",
//!         Case::Gen,
//!         Number::Singular,
//!         Gender::Masculine,
//!         Animacy::Animate,
//!     ),
//!     "dobrogo"
//! );
//! assert_eq!(
//!     interslavic::verb(
//!         "učiti",
//!         Person::First,
//!         Number::Singular,
//!         Gender::Feminine,
//!         Tense::Present,
//!     ),
//!     "učų"
//! );
//! ```

pub use interslavic_core::{
    AdjParadigm, Animacy, CASE_ORDER, Case, Gender, NounParadigm, Number, Person, PronounStyle,
    Tense, VerbParadigm, adjective, cells, derivation, noun, orthography, paradigm, phono,
    prepositions, pronoun, types, utils, verb,
};
// The dependency-free rule engine is also re-exported, so consumers can reach
// the lower-level dictionary-less API (and the shared morphophonemics helpers)
// through this crate alone, without a separate `interslavic-core` dependency.
pub use interslavic_core::{ComplexNoun, Conjugation, HARD_CONSONANTS, J_MERGE_CHARS, VOWELS};

mod dictionary;
use dictionary::*;

/// Return one dictionary-backed noun form. Unknown words fall back to the
/// core rule engine's gender and animacy inference.
pub fn noun(lemma: &str, case: Case, number: Number) -> String {
    let entries = lookup_nouns_by_lemma(lemma);
    if let Some(entry) = entries.first() {
        return noun_from_entry(entry, case, number, None, None);
    }

    noun::decline_noun(lemma, case, number)
}

/// Return one noun form with explicit gender and animacy.
///
/// If the noun exists in the generated dictionary, dictionary-only metadata
/// such as fleeting-vowel additions, number restrictions, and indeclinability
/// is still used; `gender` and `animacy` override the dictionary row.
pub fn noun_with(
    lemma: &str,
    case: Case,
    number: Number,
    gender: Gender,
    animacy: Animacy,
) -> String {
    let entries = lookup_nouns_by_lemma(lemma);
    if let Some(entry) =
        select_noun_entry(lemma, entries, case, number, Some(gender), Some(animacy))
    {
        return noun_from_entry(entry, case, number, Some(gender), Some(animacy));
    }

    noun::decline_noun_explicit(
        lemma, case, number, gender, animacy, false, false, false, None,
    )
}

/// One adjective form. Adjective phrases are not declined as a unit; callers
/// should model particles/complements separately and pass only the adjective.
pub fn adj(word: &str, case: Case, number: Number, gender: Gender, animacy: Animacy) -> String {
    adjective::decline_adj(word, case, number, gender, animacy)
}

/// The whole noun paradigm — every case in both numbers — with gender and
/// animacy inferred from the dictionary (falling back to the rule engine's
/// guess for out-of-lexicon words), the counterpart of [`verb_forms()`].
/// Cells equal the corresponding [`noun()`] calls; index them with
/// [`NounParadigm::get`].
///
/// ```
/// use interslavic::{Case, Number};
/// let p = interslavic::noun_forms("žena");
/// assert_eq!(p.get(Case::Gen, Number::Singular), "ženy");
/// assert_eq!(p.get(Case::Ins, Number::Singular), "ženojų");
/// ```
pub fn noun_forms(lemma: &str) -> NounParadigm {
    let trimmed = lemma.trim();
    let (gender, animacy) = match lookup_nouns_by_lemma(trimmed).first() {
        Some(entry) => (
            dictionary_gender_to_api(entry.gender),
            if entry.animate {
                Animacy::Animate
            } else {
                Animacy::Inanimate
            },
        ),
        None => (
            noun::guess_gender(trimmed),
            if noun::noun_is_animate(trimmed) {
                Animacy::Animate
            } else {
                Animacy::Inanimate
            },
        ),
    };
    NounParadigm {
        lemma: trimmed.to_string(),
        gender,
        animacy,
        singular: CASE_ORDER
            .iter()
            .map(|&c| noun(trimmed, c, Number::Singular))
            .collect(),
        plural: CASE_ORDER
            .iter()
            .map(|&c| noun(trimmed, c, Number::Plural))
            .collect(),
    }
}

/// The whole noun paradigm with caller-supplied gender and animacy — the
/// paradigm counterpart of [`noun_with()`]. Dictionary metadata such as
/// fleeting-vowel additions and number restrictions still applies;
/// `gender`/`animacy` override the dictionary row.
pub fn noun_forms_with(lemma: &str, gender: Gender, animacy: Animacy) -> NounParadigm {
    let trimmed = lemma.trim();
    NounParadigm {
        lemma: trimmed.to_string(),
        gender,
        animacy,
        singular: CASE_ORDER
            .iter()
            .map(|&c| noun_with(trimmed, c, Number::Singular, gender, animacy))
            .collect(),
        plural: CASE_ORDER
            .iter()
            .map(|&c| noun_with(trimmed, c, Number::Plural, gender, animacy))
            .collect(),
    }
}

/// The whole adjective paradigm — every case × number × gender/animacy
/// column. Purely rule-driven (no dictionary), like [`adj()`]; cells
/// equal the corresponding `interslavic::adj` calls. Index with [`AdjParadigm::get`].
///
/// ```
/// use interslavic::{Animacy, Case, Gender, Number};
/// let p = interslavic::adj_forms("dobry");
/// assert_eq!(p.get(Case::Gen, Number::Singular, Gender::Masculine, Animacy::Animate), "dobrogo");
/// assert_eq!(p.get(Case::Nom, Number::Singular, Gender::Feminine, Animacy::Inanimate), "dobra");
/// ```
pub fn adj_forms(word: &str) -> AdjParadigm {
    let w = word.trim();
    let column = |gender: Gender, animacy: Animacy| -> [Vec<String>; 2] {
        [Number::Singular, Number::Plural].map(|number| {
            CASE_ORDER
                .iter()
                .map(|&c| adj(w, c, number, gender, animacy))
                .collect()
        })
    };
    AdjParadigm {
        lemma: w.to_string(),
        masculine_animate: column(Gender::Masculine, Animacy::Animate),
        masculine_inanimate: column(Gender::Masculine, Animacy::Inanimate),
        feminine: column(Gender::Feminine, Animacy::Inanimate),
        neuter: column(Gender::Neuter, Animacy::Inanimate),
    }
}

/// The regular derivational family of a lemma — its abstract noun, adverb,
/// verbal noun, agentive, denominal adjectives, diminutive, negation, and
/// so on, seam morphophonemics applied. Backed by the [`derivation`]
/// module; the caller filters against attestation for real words only.
///
/// ```
/// use interslavic::derivation::Pos;
/// let fam = interslavic::derive("kniga", Pos::Noun);
/// assert!(fam.iter().any(|d| d.form == "knižny")); // denominal adjective
/// assert!(fam.iter().any(|d| d.form == "knižka")); // diminutive
/// ```
pub fn derive(base: &str, pos: derivation::Pos) -> Vec<derivation::Derived> {
    derivation::derive(base, pos)
}

/// The synthetic comparative of an adjective, as `(comparative adjective,
/// comparative adverb)`. `None` for adjectives that do not gradate
/// synthetically (relational `-sky`/`-cky`, already-comparative `-ši`/`-ći`,
/// soft `-ji` possessives) — use the analytic comparative (`vyše`/`bolje`
/// followed by the positive) there. The comparative is a soft adjective,
/// so its paradigm is `interslavic::adj(comparative, …)`. Expects a positive-degree
/// qualitative adjective in flavored orthography; other input (verbs,
/// determiners) is unspecified.
///
/// ```
/// assert_eq!(interslavic::comparative("novy"), Some(("novějši".into(), "nověje".into())));
/// assert_eq!(interslavic::comparative("dobry"), Some(("lěpši".into(), "lěpje".into())));
/// assert_eq!(interslavic::comparative("russky"), None);
/// ```
pub fn comparative(adj: &str) -> Option<(String, String)> {
    adjective::comparative(adj.trim())
}

/// The synthetic superlative of an adjective, as `(superlative adjective,
/// superlative adverb)` — the comparative with the `naj-` prefix. `None`
/// when the adjective does not gradate synthetically.
///
/// ```
/// assert_eq!(interslavic::superlative("novy"), Some(("najnovějši".into(), "najnověje".into())));
/// ```
pub fn superlative(adj: &str) -> Option<(String, String)> {
    adjective::superlative(adj.trim())
}

/// One pronoun form, or `None` if the lemma is not a recognized pronoun.
/// Covers the personal pronouns and reflexive `sebe` (full forms; the
/// clitic and prepositional n- series are explicit in
/// [`personal_pronoun()`]/[`reflexive_pronoun()`]), the `toj`-class
/// demonstratives, the `moj`-class possessives and interrogatives (incl.
/// `naš`/`vaš`/`čij`), `kto`/`čto` and derivatives, the `-koli`
/// indefinites, `veś`, and the adjectivally-declined determiners
/// (`ktory`, `kaky`, `samy`, …).
///
/// A personal lemma fixes the person and (3rd person) gender, overriding
/// the `gender`/`animacy` arguments; `number` still selects the column.
///
/// ```
/// use interslavic::*;
/// assert_eq!(interslavic::pronoun("toj", Case::Gen, Number::Singular, Gender::Masculine, Animacy::Inanimate), Some("togo".into()));
/// assert_eq!(interslavic::pronoun("moj", Case::Dat, Number::Singular, Gender::Neuter, Animacy::Inanimate), Some("mojemu".into()));
/// assert_eq!(interslavic::pronoun("kto", Case::Gen, Number::Singular, Gender::Masculine, Animacy::Animate), Some("kogo".into()));
/// assert_eq!(interslavic::pronoun("ty", Case::Gen, Number::Singular, Gender::Masculine, Animacy::Animate), Some("tebe".into()));
/// assert_eq!(interslavic::pronoun("on", Case::Dat, Number::Singular, Gender::Masculine, Animacy::Animate), Some("jemu".into()));
/// assert_eq!(interslavic::pronoun("my", Case::Gen, Number::Plural, Gender::Masculine, Animacy::Animate), Some("nas".into()));
/// assert_eq!(interslavic::pronoun("sebe", Case::Gen, Number::Singular, Gender::Masculine, Animacy::Animate), Some("sebe".into()));
/// assert_eq!(interslavic::pronoun("stol", Case::Gen, Number::Singular, Gender::Masculine, Animacy::Inanimate), None);
/// ```
pub fn pronoun(
    lemma: &str,
    case: Case,
    number: Number,
    gender: Gender,
    animacy: Animacy,
) -> Option<String> {
    adjective::decline_pronoun(lemma.trim(), case, number, gender, animacy)
}

/// One personal-pronoun form (`ja`/`ty`/`on`/`ona`/`ono`/`my`/`vy`/`oni`/
/// `one`), or `None` when the requested cell does not exist — only for
/// [`PronounStyle::Clitic`] where no clitic is attested. `gender`
/// distinguishes forms in the third person only.
///
/// The three form series the standard distinguishes are selected by
/// `style`: full forms (`mene`, `jego`), clitics (`mę`, `go`), and the
/// prepositional n- forms of the third person (`od njego`, `s njim`;
/// non-3rd-person cells have no n- variant, so `AfterPreposition` returns
/// the full form there). Backed by the explicit tables in [`pronoun`]
/// (`interslavic_core::pronoun`), which follow the `@interslavic/utils`
/// parity reference; see that module's docs for sourcing.
///
/// ```
/// use interslavic::*;
/// use PronounStyle::*;
/// let m = Gender::Masculine;
/// assert_eq!(personal_pronoun(Person::Second, Number::Singular, m, Case::Gen, Full), Some("tebe".into()));
/// assert_eq!(personal_pronoun(Person::Second, Number::Singular, m, Case::Acc, Clitic), Some("tę".into()));
/// assert_eq!(personal_pronoun(Person::Second, Number::Singular, m, Case::Ins, Full), Some("tobojų".into()));
/// assert_eq!(personal_pronoun(Person::Third, Number::Singular, m, Case::Gen, AfterPreposition), Some("njego".into()));
/// assert_eq!(personal_pronoun(Person::Third, Number::Singular, m, Case::Nom, Clitic), None);
/// ```
pub fn personal_pronoun(
    person: Person,
    number: Number,
    gender: Gender,
    case: Case,
    style: PronounStyle,
) -> Option<String> {
    pronoun::personal_pronoun(person, number, gender, case, style)
}

/// One form of the reflexive pronoun `sebe`/`sę`/`si`, or `None` for a
/// nonexistent cell (the nominative, and `Clitic` outside the accusative
/// and dative). `AfterPreposition` returns the full form — the reflexive
/// has no n- variant (`za sebe`, `o sobě`).
///
/// ```
/// use interslavic::*;
/// use PronounStyle::*;
/// assert_eq!(reflexive_pronoun(Case::Acc, Clitic), Some("sę".into()));
/// assert_eq!(reflexive_pronoun(Case::Gen, Full), Some("sebe".into()));
/// assert_eq!(reflexive_pronoun(Case::Dat, Clitic), Some("si".into()));
/// assert_eq!(reflexive_pronoun(Case::Nom, Full), None);
/// ```
pub fn reflexive_pronoun(case: Case, style: PronounStyle) -> Option<String> {
    pronoun::reflexive_pronoun(case, style)
}

/// One numeral form, or `None` if the lemma is not a recognized numeral.
/// Covers `jedin`, the dual-remnant `dva`/`oba`/`obydva` and `tri`/`četyri`,
/// the i-stem numerals `pęť`…`desęť`, and the adjectivally-declined ordinals
/// (`pŕvy`, `drugy`, …). Cardinals return their citation form for the
/// nominative and accusative.
///
/// ```
/// use interslavic::*;
/// assert_eq!(interslavic::numeral("pęť", Case::Gen, Number::Plural, Gender::Masculine, Animacy::Inanimate), Some("pęti".into()));
/// assert_eq!(interslavic::numeral("tri", Case::Gen, Number::Plural, Gender::Masculine, Animacy::Inanimate), Some("trěh".into()));
/// assert_eq!(interslavic::numeral("pŕvy", Case::Gen, Number::Singular, Gender::Masculine, Animacy::Inanimate), Some("pŕvogo".into()));
/// ```
pub fn numeral(
    lemma: &str,
    case: Case,
    number: Number,
    gender: Gender,
    animacy: Animacy,
) -> Option<String> {
    adjective::decline_numeral(lemma.trim(), case, number, gender, animacy)
}

/// The case(s) a preposition governs, or `None` if `prep` is not a
/// recognized single-word preposition. `prep` is the flavored citation
/// form; a preposition may govern several cases (the case selects the
/// meaning). Backed by the curated [`prepositions`] table.
///
/// ```
/// use interslavic::Case;
/// assert_eq!(interslavic::preposition_cases("bez"), Some(&[Case::Gen][..]));
/// assert_eq!(interslavic::preposition_cases("na"), Some(&[Case::Acc, Case::Loc][..]));
/// assert_eq!(interslavic::preposition_cases("žaba"), None);
/// ```
pub fn preposition_cases(prep: &str) -> Option<&'static [Case]> {
    prepositions::preposition_cases(prep.trim())
}

/// One finite verb form. Present, imperfect, future, perfect, pluperfect,
/// and conditional are supported; imperative and participial/gerund forms
/// are available through `verb_forms`.
pub fn verb(word: &str, person: Person, number: Number, gender: Gender, tense: Tense) -> String {
    let trimmed = word.trim();
    let entries = lookup_verbs_by_lemma(trimmed);
    if let Some(entry) = entries.first() {
        return verb::conjugate_verb_with_options(
            entry.lemma,
            entry.addition,
            person,
            number,
            gender,
            tense,
            entry.transitive,
            entry.imperfective,
        );
    }

    verb::conjugate_verb(trimmed, person, number, gender, tense)
}

/// One finite verb form, or `None` when an infinitive stem cannot be
/// derived from `word` (roughly, it does not end in `-ti`/`-t`/`-ť`). The
/// checked counterpart of [`verb()`]: it reports clearly non-verb input
/// as `None` instead of degrading to a best-effort string, so callers need
/// no `catch_unwind` guard. The check is mechanical, not lexical — a `-t`
/// noun shaped like an infinitive still yields `Some`.
///
/// ```
/// use interslavic::*;
/// assert!(interslavic::try_verb("pisati", Person::First, Number::Singular, Gender::Masculine, Tense::Present).is_some());
/// assert_eq!(interslavic::try_verb("xyz", Person::First, Number::Singular, Gender::Masculine, Tense::Present), None);
/// ```
pub fn try_verb(
    word: &str,
    person: Person,
    number: Number,
    gender: Gender,
    tense: Tense,
) -> Option<String> {
    let trimmed = word.trim();
    let entries = lookup_verbs_by_lemma(trimmed);
    if let Some(entry) = entries.first() {
        return verb::conjugate_verb_checked(
            entry.lemma,
            entry.addition,
            person,
            number,
            gender,
            tense,
            entry.transitive,
            entry.imperfective,
        );
    }
    verb::conjugate_verb_checked(trimmed, "", person, number, gender, tense, true, true)
}

/// One finite verb form with an explicit dictionary present-stem hint.
///
/// This is intended for typed dictionary rows that have multiple entries for
/// the same lemma. It does not parse arbitrary phrase strings.
pub fn verb_with_present_hint(
    word: &str,
    present_hint: &str,
    person: Person,
    number: Number,
    gender: Gender,
    tense: Tense,
) -> String {
    verb::conjugate_verb_with_present_hint(word.trim(), present_hint, person, number, gender, tense)
}

/// The l-participle of an infinitive — the gender/number-marked past
/// active form the compound tenses are built on, useful on its own for
/// fixed-gender past subjects ("strěla tę ubila"). Suppletion is handled
/// (`idti` → `šėl`/`šla`/…), and the form always agrees with the
/// participle inside [`verb_forms()`]'s perfect/pluperfect/conditional.
///
/// ```
/// use interslavic::*;
/// // A regular verb.
/// assert_eq!(l_participle("pisati", Gender::Masculine, Number::Singular), "pisal");
/// assert_eq!(l_participle("pisati", Gender::Feminine, Number::Singular), "pisala");
/// assert_eq!(l_participle("pisati", Gender::Neuter, Number::Singular), "pisalo");
/// assert_eq!(l_participle("pisati", Gender::Masculine, Number::Plural), "pisali");
/// // The idti suppletion.
/// assert_eq!(l_participle("idti", Gender::Masculine, Number::Singular), "šėl");
/// assert_eq!(l_participle("idti", Gender::Feminine, Number::Singular), "šla");
/// assert_eq!(l_participle("idti", Gender::Neuter, Number::Singular), "šlo");
/// assert_eq!(l_participle("idti", Gender::Masculine, Number::Plural), "šli");
/// // A prefixed perfective.
/// assert_eq!(l_participle("ubiti", Gender::Masculine, Number::Singular), "ubil");
/// assert_eq!(l_participle("ubiti", Gender::Feminine, Number::Singular), "ubila");
/// ```
pub fn l_participle(infinitive: &str, gender: Gender, number: Number) -> String {
    verb::l_participle(infinitive, gender, number)
}

/// Full verb paradigm with dictionary metadata when available.
pub fn verb_forms(word: &str) -> VerbParadigm {
    let trimmed = word.trim();
    let entries = lookup_verbs_by_lemma(trimmed);
    if let Some(entry) = entries.first() {
        return verb::verb_paradigm_with_options(
            entry.lemma,
            entry.addition,
            entry.transitive,
            entry.imperfective,
        );
    }
    verb::verb_paradigm_with_options(trimmed, "", true, true)
}

/// The full verb paradigm, or `None` when an infinitive stem cannot be
/// derived from `word` — the checked counterpart of [`verb_forms()`].
/// The check is mechanical (infinitive shape), not a lexical verb lookup.
///
/// ```
/// use interslavic::*;
/// assert!(interslavic::try_verb_forms("pisati").is_some());
/// assert_eq!(interslavic::try_verb_forms("xyz"), None);
/// ```
pub fn try_verb_forms(word: &str) -> Option<VerbParadigm> {
    let trimmed = word.trim();
    let entries = lookup_verbs_by_lemma(trimmed);
    if let Some(entry) = entries.first() {
        return verb::verb_paradigm_checked(
            entry.lemma,
            entry.addition,
            entry.transitive,
            entry.imperfective,
        );
    }
    verb::verb_paradigm_checked(trimmed, "", true, true)
}

/// Full verb paradigm with explicit dictionary metadata.
pub fn verb_forms_with_metadata(
    word: &str,
    present_hint: &str,
    transitive: bool,
    imperfective: bool,
) -> VerbParadigm {
    verb::verb_paradigm_with_options(word.trim(), present_hint, transitive, imperfective)
}

fn select_noun_entry<'a>(
    lemma: &str,
    entries: &'a [DictionaryEntry],
    case: Case,
    number: Number,
    gender: Option<Gender>,
    animacy: Option<Animacy>,
) -> Option<&'a DictionaryEntry> {
    entries.iter().max_by_key(|entry| {
        let mut score = 0;
        if entry.lemma == lemma {
            score += 16;
        }
        if gender.is_some_and(|wanted| dictionary_gender_to_api(entry.gender) == wanted) {
            score += 8;
        }
        if animacy.is_some_and(|wanted| {
            let entry_animacy = if entry.animate {
                Animacy::Animate
            } else {
                Animacy::Inanimate
            };
            entry_animacy == wanted
        }) {
            score += 4;
        }
        if number == Number::Singular && !entry.plural_only {
            score += 2;
        }
        if number == Number::Plural && !entry.singular_only {
            score += 2;
        }
        if case == Case::Nom {
            score += 1;
        }
        if !entry.indeclinable {
            score += 1;
        }
        score
    })
}

fn noun_from_entry(
    entry: &DictionaryEntry,
    case: Case,
    number: Number,
    gender_override: Option<Gender>,
    animacy_override: Option<Animacy>,
) -> String {
    let gender = gender_override.unwrap_or_else(|| dictionary_gender_to_api(entry.gender));
    let animacy = animacy_override.unwrap_or(if entry.animate {
        Animacy::Animate
    } else {
        Animacy::Inanimate
    });

    noun::decline_noun_explicit(
        entry.lemma,
        case,
        number,
        gender,
        animacy,
        entry.plural_only,
        entry.singular_only,
        entry.indeclinable,
        (!entry.addition.is_empty()).then_some(entry.addition),
    )
}

fn dictionary_gender_to_api(gender: DictionaryGender) -> Gender {
    match gender {
        DictionaryGender::Masculine | DictionaryGender::MasculineFeminine => Gender::Masculine,
        DictionaryGender::Feminine => Gender::Feminine,
        DictionaryGender::Neuter => Gender::Neuter,
    }
}
