//! Personal and reflexive pronoun paradigms.
//!
//! These are closed-class, suppletive paradigms, so they live here as
//! explicit tables rather than derived rules. The cell data follows, in
//! order of authority:
//!
//! 1. `@interslavic/utils` 3.4.0, `src/pronoun/declensionPronoun.ts`
//!    (vendored at `.external/npm/package/src/pronoun/`) — the same parity
//!    target the README accuracy table measures against;
//! 2. Jan van Steenbergen's grammar tables (steen.free.fr, "Pronouns").
//!
//! Where the two disagree, the JS reference wins (it is the repo's
//! established parity standard). Known discrepancies, kept deliberately:
//!
//! - **Reflexive accusative full form**: the JS reference has `sebę (sę)`;
//!   the steen table has `sebe (se)`. We follow the JS `sebę`.
//! - **3sg masculine/neuter locative**: the JS reference has `(n)jem`; the
//!   steen legacy table has `jim`. We follow the JS `jem`/`njem` (which
//!   also matches the current interslavic.fun grammar). As steen notes,
//!   the locative always follows a preposition, so only the n-form `njem`
//!   occurs in real text.
//! - **Clitic inventory**: exactly the cells the JS reference attests —
//!   `mę`/`mi`, `tę`/`ti`, `go`/`mu`, `sę`/`si` (accusative/dative). The
//!   steen table shows the same set (in its plain orthography `me`, `te`,
//!   `se`). No genitive, plural, or feminine clitics exist in either
//!   source.
//!
//! The three form series the standard distinguishes are explicit in the
//! API via [`PronounStyle`]: full forms, clitic (short) forms, and the
//! prepositional n- forms of the third person (`od njego`, `s njim`).

use crate::{Case, Gender, Number, Person, PronounStyle};

/// One paradigm cell: the full form, the clitic if one is attested, and
/// whether the prepositional n- variant exists (3rd-person oblique cells
/// only; the n- form is always `n` + the full form).
struct Cell {
    full: &'static str,
    clitic: Option<&'static str>,
    n_form: bool,
}

const fn cell(full: &'static str) -> Cell {
    Cell {
        full,
        clitic: None,
        n_form: false,
    }
}

const fn cell_cl(full: &'static str, clitic: &'static str) -> Cell {
    Cell {
        full,
        clitic: Some(clitic),
        n_form: false,
    }
}

const fn cell_n(full: &'static str) -> Cell {
    Cell {
        full,
        clitic: None,
        n_form: true,
    }
}

const fn cell_n_cl(full: &'static str, clitic: &'static str) -> Cell {
    Cell {
        full,
        clitic: Some(clitic),
        n_form: true,
    }
}

/// Cell lookup for one person/number/gender column, in [`crate::CASE_ORDER`]
/// (nom, acc, gen, loc, dat, ins). Gender distinguishes cells only in the
/// third person: the feminine singular column, and `oni` (masculine) vs
/// `one` (feminine/neuter) with `jih` vs `je` in the plural
/// nominative/accusative.
fn personal_cell(person: Person, number: Number, gender: Gender, case: Case) -> Cell {
    use Case::*;
    match (person, number) {
        (Person::First, Number::Singular) => match case {
            Nom => cell("ja"),
            Acc => cell_cl("mene", "mę"),
            Gen => cell("mene"),
            Loc => cell("mně"),
            Dat => cell_cl("mně", "mi"),
            Ins => cell("mnojų"),
        },
        (Person::First, Number::Plural) => match case {
            Nom => cell("my"),
            Acc | Gen | Loc => cell("nas"),
            Dat => cell("nam"),
            Ins => cell("nami"),
        },
        (Person::Second, Number::Singular) => match case {
            Nom => cell("ty"),
            Acc => cell_cl("tebe", "tę"),
            Gen => cell("tebe"),
            Loc => cell("tobě"),
            Dat => cell_cl("tobě", "ti"),
            Ins => cell("tobojų"),
        },
        (Person::Second, Number::Plural) => match case {
            Nom => cell("vy"),
            Acc | Gen | Loc => cell("vas"),
            Dat => cell("vam"),
            Ins => cell("vami"),
        },
        (Person::Third, Number::Singular) => match (gender, case) {
            (Gender::Masculine, Nom) => cell("on"),
            (Gender::Neuter, Nom) => cell("ono"),
            (Gender::Feminine, Nom) => cell("ona"),
            (Gender::Masculine | Gender::Neuter, Acc) => cell_n_cl("jego", "go"),
            (Gender::Masculine | Gender::Neuter, Gen) => cell_n("jego"),
            (Gender::Masculine | Gender::Neuter, Loc) => cell_n("jem"),
            (Gender::Masculine | Gender::Neuter, Dat) => cell_n_cl("jemu", "mu"),
            (Gender::Masculine | Gender::Neuter, Ins) => cell_n("jim"),
            (Gender::Feminine, Acc) => cell_n("jų"),
            (Gender::Feminine, Gen | Loc | Dat) => cell_n("jej"),
            (Gender::Feminine, Ins) => cell_n("jejų"),
        },
        (Person::Third, Number::Plural) => match (gender, case) {
            (Gender::Masculine, Nom) => cell("oni"),
            (Gender::Feminine | Gender::Neuter, Nom) => cell("one"),
            (Gender::Masculine, Acc) => cell_n("jih"),
            (Gender::Feminine | Gender::Neuter, Acc) => cell_n("je"),
            (_, Gen | Loc) => cell_n("jih"),
            (_, Dat) => cell_n("jim"),
            (_, Ins) => cell_n("jimi"),
        },
    }
}

/// Cell lookup for the reflexive `sebe`, which has no nominative.
fn reflexive_cell(case: Case) -> Option<Cell> {
    Some(match case {
        Case::Nom => return None,
        // JS reference: `sebę (sę)`; the steen table has `sebe` — see the
        // module docs for why the JS form wins.
        Case::Acc => cell_cl("sebę", "sę"),
        Case::Gen => cell("sebe"),
        Case::Loc => cell("sobě"),
        Case::Dat => cell_cl("sobě", "si"),
        Case::Ins => cell("sobojų"),
    })
}

fn styled(cell: Cell, style: PronounStyle) -> Option<String> {
    match style {
        PronounStyle::Full => Some(cell.full.to_string()),
        PronounStyle::Clitic => cell.clitic.map(str::to_string),
        PronounStyle::AfterPreposition => Some(if cell.n_form {
            format!("n{}", cell.full)
        } else {
            cell.full.to_string()
        }),
    }
}

/// One personal-pronoun form, or `None` when the requested cell does not
/// exist — which happens only for [`PronounStyle::Clitic`] where no clitic
/// is attested (clitics exist solely in the accusative and dative of `ja`,
/// `ty`, the 3sg masculine/neuter, and the reflexive).
///
/// `gender` distinguishes forms only in the third person; the first and
/// second persons ignore it. In the plural, masculine selects `oni`/`jih`
/// and feminine/neuter `one`/`je` (the masculine-inanimate reading of
/// `one`/`je` is not separately representable — there is no animacy
/// parameter); the oblique plural cells are gender-invariant.
///
/// [`PronounStyle::AfterPreposition`] always returns a form that is
/// correct after a preposition: the prepositional n- variant where the
/// paradigm has one (3rd-person oblique cells: `njego`, `njemu`, `njim`,
/// `njej`, `njih`, …), and the plain full form everywhere else.
///
/// ```
/// use interslavic_core::{pronoun::personal_pronoun, Case, Gender, Number, Person, PronounStyle};
/// use PronounStyle::*;
///
/// // Nominatives and full genitive/accusative of every person/number.
/// let m = Gender::Masculine;
/// assert_eq!(personal_pronoun(Person::First, Number::Singular, m, Case::Nom, Full), Some("ja".into()));
/// assert_eq!(personal_pronoun(Person::First, Number::Singular, m, Case::Gen, Full), Some("mene".into()));
/// assert_eq!(personal_pronoun(Person::First, Number::Singular, m, Case::Acc, Full), Some("mene".into()));
/// assert_eq!(personal_pronoun(Person::Second, Number::Singular, m, Case::Nom, Full), Some("ty".into()));
/// assert_eq!(personal_pronoun(Person::Second, Number::Singular, m, Case::Gen, Full), Some("tebe".into()));
/// assert_eq!(personal_pronoun(Person::Second, Number::Singular, m, Case::Acc, Full), Some("tebe".into()));
/// assert_eq!(personal_pronoun(Person::Third, Number::Singular, m, Case::Nom, Full), Some("on".into()));
/// assert_eq!(personal_pronoun(Person::Third, Number::Singular, m, Case::Gen, Full), Some("jego".into()));
/// assert_eq!(personal_pronoun(Person::Third, Number::Singular, Gender::Neuter, Case::Nom, Full), Some("ono".into()));
/// assert_eq!(personal_pronoun(Person::Third, Number::Singular, Gender::Feminine, Case::Nom, Full), Some("ona".into()));
/// assert_eq!(personal_pronoun(Person::Third, Number::Singular, Gender::Feminine, Case::Acc, Full), Some("jų".into()));
/// assert_eq!(personal_pronoun(Person::Third, Number::Singular, Gender::Feminine, Case::Gen, Full), Some("jej".into()));
/// assert_eq!(personal_pronoun(Person::First, Number::Plural, m, Case::Nom, Full), Some("my".into()));
/// assert_eq!(personal_pronoun(Person::First, Number::Plural, m, Case::Gen, Full), Some("nas".into()));
/// assert_eq!(personal_pronoun(Person::First, Number::Plural, m, Case::Acc, Full), Some("nas".into()));
/// assert_eq!(personal_pronoun(Person::Second, Number::Plural, m, Case::Nom, Full), Some("vy".into()));
/// assert_eq!(personal_pronoun(Person::Second, Number::Plural, m, Case::Gen, Full), Some("vas".into()));
/// assert_eq!(personal_pronoun(Person::Third, Number::Plural, m, Case::Nom, Full), Some("oni".into()));
/// assert_eq!(personal_pronoun(Person::Third, Number::Plural, Gender::Feminine, Case::Nom, Full), Some("one".into()));
/// assert_eq!(personal_pronoun(Person::Third, Number::Plural, m, Case::Acc, Full), Some("jih".into()));
/// assert_eq!(personal_pronoun(Person::Third, Number::Plural, Gender::Feminine, Case::Acc, Full), Some("je".into()));
/// assert_eq!(personal_pronoun(Person::Third, Number::Plural, m, Case::Gen, Full), Some("jih".into()));
///
/// // Other full obliques downstream templates lean on.
/// assert_eq!(personal_pronoun(Person::First, Number::Singular, m, Case::Dat, Full), Some("mně".into()));
/// assert_eq!(personal_pronoun(Person::First, Number::Singular, m, Case::Ins, Full), Some("mnojų".into()));
/// assert_eq!(personal_pronoun(Person::Second, Number::Singular, m, Case::Ins, Full), Some("tobojų".into()));
/// assert_eq!(personal_pronoun(Person::Third, Number::Singular, m, Case::Dat, Full), Some("jemu".into()));
/// assert_eq!(personal_pronoun(Person::Second, Number::Plural, m, Case::Dat, Full), Some("vam".into()));
///
/// // One clitic per attested series, and a None where no clitic exists.
/// assert_eq!(personal_pronoun(Person::First, Number::Singular, m, Case::Acc, Clitic), Some("mę".into()));
/// assert_eq!(personal_pronoun(Person::First, Number::Singular, m, Case::Dat, Clitic), Some("mi".into()));
/// assert_eq!(personal_pronoun(Person::Second, Number::Singular, m, Case::Acc, Clitic), Some("tę".into()));
/// assert_eq!(personal_pronoun(Person::Second, Number::Singular, m, Case::Dat, Clitic), Some("ti".into()));
/// assert_eq!(personal_pronoun(Person::Third, Number::Singular, m, Case::Acc, Clitic), Some("go".into()));
/// assert_eq!(personal_pronoun(Person::Third, Number::Singular, m, Case::Dat, Clitic), Some("mu".into()));
/// assert_eq!(personal_pronoun(Person::Third, Number::Singular, Gender::Feminine, Case::Acc, Clitic), None);
/// assert_eq!(personal_pronoun(Person::First, Number::Singular, m, Case::Nom, Clitic), None);
/// assert_eq!(personal_pronoun(Person::First, Number::Plural, m, Case::Acc, Clitic), None);
///
/// // Every 3rd-person n- form series ("od njego", "s njim", …).
/// assert_eq!(personal_pronoun(Person::Third, Number::Singular, m, Case::Gen, AfterPreposition), Some("njego".into()));
/// assert_eq!(personal_pronoun(Person::Third, Number::Singular, m, Case::Dat, AfterPreposition), Some("njemu".into()));
/// assert_eq!(personal_pronoun(Person::Third, Number::Singular, m, Case::Loc, AfterPreposition), Some("njem".into()));
/// assert_eq!(personal_pronoun(Person::Third, Number::Singular, m, Case::Ins, AfterPreposition), Some("njim".into()));
/// assert_eq!(personal_pronoun(Person::Third, Number::Singular, Gender::Feminine, Case::Acc, AfterPreposition), Some("njų".into()));
/// assert_eq!(personal_pronoun(Person::Third, Number::Singular, Gender::Feminine, Case::Gen, AfterPreposition), Some("njej".into()));
/// assert_eq!(personal_pronoun(Person::Third, Number::Singular, Gender::Feminine, Case::Ins, AfterPreposition), Some("njejų".into()));
/// assert_eq!(personal_pronoun(Person::Third, Number::Plural, m, Case::Gen, AfterPreposition), Some("njih".into()));
/// assert_eq!(personal_pronoun(Person::Third, Number::Plural, m, Case::Dat, AfterPreposition), Some("njim".into()));
/// assert_eq!(personal_pronoun(Person::Third, Number::Plural, m, Case::Ins, AfterPreposition), Some("njimi".into()));
///
/// // Non-3rd persons have no n- forms; AfterPreposition is the full form.
/// assert_eq!(personal_pronoun(Person::First, Number::Singular, m, Case::Loc, AfterPreposition), Some("mně".into()));
/// assert_eq!(personal_pronoun(Person::Second, Number::Singular, m, Case::Acc, AfterPreposition), Some("tebe".into()));
/// ```
pub fn personal_pronoun(
    person: Person,
    number: Number,
    gender: Gender,
    case: Case,
    style: PronounStyle,
) -> Option<String> {
    styled(personal_cell(person, number, gender, case), style)
}

/// One form of the reflexive pronoun `sebe`, or `None` for a cell that
/// does not exist: the nominative (any style) and the clitic style outside
/// the accusative (`sę`) and dative (`si`). The reflexive has no
/// prepositional n- variant, so [`PronounStyle::AfterPreposition`] returns
/// the full form (`za sebe`, `o sobě`).
///
/// The accusative full form follows the JS parity reference (`sebę`); see
/// the module docs for the discrepancy with the steen table's `sebe`.
///
/// ```
/// use interslavic_core::{pronoun::reflexive_pronoun, Case, PronounStyle};
/// use PronounStyle::*;
///
/// assert_eq!(reflexive_pronoun(Case::Nom, Full), None);
/// assert_eq!(reflexive_pronoun(Case::Acc, Full), Some("sebę".into()));
/// assert_eq!(reflexive_pronoun(Case::Acc, Clitic), Some("sę".into()));
/// assert_eq!(reflexive_pronoun(Case::Gen, Full), Some("sebe".into()));
/// assert_eq!(reflexive_pronoun(Case::Dat, Full), Some("sobě".into()));
/// assert_eq!(reflexive_pronoun(Case::Dat, Clitic), Some("si".into()));
/// assert_eq!(reflexive_pronoun(Case::Loc, Full), Some("sobě".into()));
/// assert_eq!(reflexive_pronoun(Case::Ins, Full), Some("sobojų".into()));
/// assert_eq!(reflexive_pronoun(Case::Gen, Clitic), None);
/// assert_eq!(reflexive_pronoun(Case::Loc, AfterPreposition), Some("sobě".into()));
/// ```
pub fn reflexive_pronoun(case: Case, style: PronounStyle) -> Option<String> {
    styled(reflexive_cell(case)?, style)
}

/// The full form for a bare personal/reflexive lemma, used by
/// [`crate::adjective::decline_pronoun`] to keep the closed-class lookup
/// story uniform (`pronoun("ty", Case::Gen, …)` → `tebe`). The lemma fixes
/// the person (and, for the third person, the gender); `number` selects
/// the column, so the singular and plural lemmas of one person are
/// interchangeable (`ja`/`my`, `ty`/`vy`, `on`/`oni`).
pub(crate) fn personal_lemma_full(lemma: &str, case: Case, number: Number) -> Option<String> {
    let (person, gender) = match lemma {
        "ja" | "my" => (Person::First, Gender::Masculine),
        "ty" | "vy" => (Person::Second, Gender::Masculine),
        "on" | "oni" => (Person::Third, Gender::Masculine),
        "ona" | "one" => (Person::Third, Gender::Feminine),
        "ono" => (Person::Third, Gender::Neuter),
        "sebe" => return reflexive_pronoun(case, PronounStyle::Full),
        _ => return None,
    };
    personal_pronoun(person, number, gender, case, PronounStyle::Full)
}
