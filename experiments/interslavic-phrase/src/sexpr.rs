//! The S-expression surface: a hand-rolled reader (no dependencies), a
//! canonical printer, and the compiler from generic S-expressions to the
//! typed AST. This is the *data* surface — templates, goldens, downstream
//! markers; the typed builders in [`crate::ast`] are the *code* surface.
//! One AST underneath, so the two cannot drift.
//!
//! # Vocabulary (the format spec)
//!
//! ```text
//! CLAUSE := (clause SUBJ VP KEY*)
//!           KEY: :tense present|past|future  :neg  :prodrop
//!                :force li|či|intonation
//! SUBJ   := NP | PRON
//! NP     := (np [:case CASE] [(det L)] [(num N)] (adj L)* (n L))
//! PRON   := (pron :1|:2|:3 :sg|:pl :m|:f|:n)
//! VP     := (vp (v L…) [OBJ: NP|PRON] PP*)      ; "v myti sę" = reflexive
//! PP     := (pp (prep L) [:case CASE] OBJ)
//! CASE   := nom|acc|gen|loc|dat|ins
//! ```
//!
//! Leaves are flavored citation forms; integers are counts rendered as
//! digits.

use crate::ast::*;
use interslavic::{Case, Gender, Number, Person};
use std::fmt;

/// A parse or compile error with the byte offset it refers to.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SexprError {
    pub at: usize,
    pub msg: String,
}

impl fmt::Display for SexprError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "at byte {}: {}", self.at, self.msg)
    }
}
impl std::error::Error for SexprError {}

fn err<T>(at: usize, msg: impl Into<String>) -> Result<T, SexprError> {
    Err(SexprError {
        at,
        msg: msg.into(),
    })
}

/// Generic S-expression values, spans attached.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Value {
    Sym(String, usize),
    Int(u64, usize),
    Key(String, usize),
    List(Vec<Value>, usize),
}

impl Value {
    fn at(&self) -> usize {
        match self {
            Value::Sym(_, at) | Value::Int(_, at) | Value::Key(_, at) | Value::List(_, at) => *at,
        }
    }
}

/// Parse exactly one toplevel form. Never panics; malformed input returns
/// a spanned error.
pub fn parse(input: &str) -> Result<Value, SexprError> {
    let mut tokens = tokenize(input)?;
    tokens.reverse(); // pop() from the front
    let value = parse_value(&mut tokens)?;
    if let Some(extra) = tokens.pop() {
        return err(extra.at(), "trailing input after the toplevel form");
    }
    Ok(value)
}

fn tokenize(input: &str) -> Result<Vec<Value>, SexprError> {
    // Tokens reuse Value: Sym("(") / Sym(")") sentinels never escape.
    let mut out = Vec::new();
    let mut chars = input.char_indices().peekable();
    while let Some(&(at, c)) = chars.peek() {
        if c.is_whitespace() {
            chars.next();
        } else if c == '(' || c == ')' {
            chars.next();
            out.push(Value::Sym(c.to_string(), at));
        } else {
            let mut atom = String::new();
            while let Some(&(_, c)) = chars.peek() {
                if c.is_whitespace() || c == '(' || c == ')' {
                    break;
                }
                atom.push(c);
                chars.next();
            }
            out.push(if let Some(key) = atom.strip_prefix(':') {
                if key.is_empty() {
                    return err(at, "bare `:` is not a keyword");
                }
                Value::Key(key.to_string(), at)
            } else if atom.chars().all(|c| c.is_ascii_digit()) {
                Value::Int(
                    atom.parse().map_err(|_| SexprError {
                        at,
                        msg: format!("integer out of range: {atom}"),
                    })?,
                    at,
                )
            } else {
                Value::Sym(atom, at)
            });
        }
    }
    Ok(out)
}

fn parse_value(tokens: &mut Vec<Value>) -> Result<Value, SexprError> {
    match tokens.pop() {
        None => err(0, "unexpected end of input"),
        Some(Value::Sym(s, at)) if s == "(" => {
            let mut items = Vec::new();
            loop {
                match tokens.last() {
                    None => return err(at, "unclosed `(`"),
                    Some(Value::Sym(s, _)) if s == ")" => {
                        tokens.pop();
                        return Ok(Value::List(items, at));
                    }
                    _ => items.push(parse_value(tokens)?),
                }
            }
        }
        Some(Value::Sym(s, at)) if s == ")" => err(at, "unexpected `)`"),
        Some(token) => Ok(token),
    }
}

// ---------------------------------------------------------------------------
// Compiling generic values into the typed AST.
// ---------------------------------------------------------------------------

fn case_of(name: &str, at: usize) -> Result<Case, SexprError> {
    Ok(match name {
        "nom" => Case::Nom,
        "acc" => Case::Acc,
        "gen" => Case::Gen,
        "loc" => Case::Loc,
        "dat" => Case::Dat,
        "ins" => Case::Ins,
        _ => return err(at, format!("unknown case `{name}`")),
    })
}

fn case_name(case: Case) -> &'static str {
    match case {
        Case::Nom => "nom",
        Case::Acc => "acc",
        Case::Gen => "gen",
        Case::Loc => "loc",
        Case::Dat => "dat",
        Case::Ins => "ins",
    }
}

fn head_of(list: &[Value], at: usize) -> Result<(&str, usize), SexprError> {
    match list.first() {
        Some(Value::Sym(s, at)) => Ok((s.as_str(), *at)),
        _ => err(at, "expected a list starting with a symbol"),
    }
}

fn sym_arg(list: &[Value], head: &str, at: usize) -> Result<String, SexprError> {
    match list {
        [_, Value::Sym(s, _)] => Ok(s.clone()),
        _ => err(at, format!("`({head} …)` takes exactly one symbol")),
    }
}

pub fn compile_clause(value: &Value) -> Result<Clause, SexprError> {
    let Value::List(items, at) = value else {
        return err(value.at(), "expected `(clause …)`");
    };
    let (head, _) = head_of(items, *at)?;
    if head != "clause" {
        return err(*at, format!("expected `clause`, found `{head}`"));
    }
    let mut positional = Vec::new();
    let mut clause_tense = TenseSpec::Present;
    let mut polarity = Polarity::Affirmative;
    let mut force = Force::Declarative;
    let mut prodrop = false;
    let mut rest = items[1..].iter().peekable();
    while let Some(item) = rest.next() {
        match item {
            Value::Key(key, at) => match key.as_str() {
                "tense" => {
                    clause_tense = match rest.next() {
                        Some(Value::Sym(s, at)) => match s.as_str() {
                            "present" => TenseSpec::Present,
                            "past" => TenseSpec::Past,
                            "future" => TenseSpec::Future,
                            other => return err(*at, format!("unknown tense `{other}`")),
                        },
                        _ => return err(*at, "`:tense` needs present|past|future"),
                    }
                }
                "force" => {
                    force = match rest.next() {
                        Some(Value::Sym(s, at)) => match s.as_str() {
                            "li" => Force::LiQuestion,
                            "či" => Force::CiQuestion,
                            "intonation" => Force::IntonationQuestion,
                            other => return err(*at, format!("unknown force `{other}`")),
                        },
                        _ => return err(*at, "`:force` needs li|či|intonation"),
                    }
                }
                "neg" => polarity = Polarity::Negative,
                "prodrop" => prodrop = true,
                other => return err(*at, format!("unknown clause key `:{other}`")),
            },
            other => positional.push(other),
        }
    }
    let [subject, verb_phrase] = positional.as_slice() else {
        return err(*at, "`clause` takes exactly a subject and a vp");
    };
    Ok(Clause {
        subject: compile_nominal(subject)?,
        vp: compile_vp(verb_phrase)?,
        tense: clause_tense,
        polarity,
        force,
        prodrop,
    })
}

fn compile_nominal(value: &Value) -> Result<Nominal, SexprError> {
    let Value::List(items, at) = value else {
        return err(value.at(), "expected `(np …)` or `(pron …)`");
    };
    match head_of(items, *at)? {
        ("np", _) => compile_np(items, *at).map(Nominal::Np),
        ("pron", _) => compile_pron(items, *at),
        (other, at) => err(at, format!("expected np or pron, found `{other}`")),
    }
}

fn compile_np(items: &[Value], at: usize) -> Result<NounPhrase, SexprError> {
    let mut np = NounPhrase::new("");
    let mut head_seen = false;
    let mut rest = items[1..].iter().peekable();
    while let Some(item) = rest.next() {
        match item {
            Value::Key(key, kat) if key == "case" => match rest.next() {
                Some(Value::Sym(s, sat)) => np.case_override = Some(case_of(s, *sat)?),
                _ => return err(*kat, "`:case` needs a case name"),
            },
            Value::Key(key, kat) => return err(*kat, format!("unknown np key `:{key}`")),
            Value::List(child, cat) => match head_of(child, *cat)? {
                ("det", _) => np.determiner = Some(sym_arg(child, "det", *cat)?),
                ("adj", _) => np.adjectives.push(sym_arg(child, "adj", *cat)?),
                ("n", _) => {
                    np.head = sym_arg(child, "n", *cat)?;
                    head_seen = true;
                }
                ("num", _) => match child.as_slice() {
                    [_, Value::Int(n, _)] => np.count = Some(*n),
                    _ => return err(*cat, "`(num …)` takes one integer"),
                },
                (other, cat) => return err(cat, format!("unknown np child `{other}`")),
            },
            other => return err(other.at(), "unexpected atom inside `(np …)`"),
        }
    }
    if !head_seen {
        return err(at, "`(np …)` needs a `(n …)` head");
    }
    Ok(np)
}

fn compile_pron(items: &[Value], at: usize) -> Result<Nominal, SexprError> {
    let (mut person, mut number, mut gender) = (None, None, None);
    for item in &items[1..] {
        let Value::Key(key, kat) = item else {
            return err(item.at(), "`(pron …)` takes only keywords");
        };
        match key.as_str() {
            "1" => person = Some(Person::First),
            "2" => person = Some(Person::Second),
            "3" => person = Some(Person::Third),
            "sg" => number = Some(Number::Singular),
            "pl" => number = Some(Number::Plural),
            "m" => gender = Some(Gender::Masculine),
            "f" => gender = Some(Gender::Feminine),
            "n" => gender = Some(Gender::Neuter),
            other => return err(*kat, format!("unknown pron key `:{other}`")),
        }
    }
    match (person, number) {
        (Some(person), Some(number)) => Ok(Nominal::Pron {
            person,
            number,
            // Gender matters only in the 3rd person; default masculine.
            gender: gender.unwrap_or(Gender::Masculine),
        }),
        _ => err(
            at,
            "`(pron …)` needs a person (:1/:2/:3) and number (:sg/:pl)",
        ),
    }
}

fn compile_vp(value: &Value) -> Result<VerbPhrase, SexprError> {
    let Value::List(items, at) = value else {
        return err(value.at(), "expected `(vp …)`");
    };
    let (head, _) = head_of(items, *at)?;
    if head != "vp" {
        return err(*at, format!("expected `vp`, found `{head}`"));
    }
    let mut verb = None;
    let mut object = None;
    let mut pps = Vec::new();
    for item in &items[1..] {
        let Value::List(child, cat) = item else {
            return err(item.at(), "unexpected atom inside `(vp …)`");
        };
        match head_of(child, *cat)? {
            ("v", _) => {
                // One or more symbols: "myti" or "myti sę".
                let words: Vec<&str> = child[1..]
                    .iter()
                    .map(|value| match value {
                        Value::Sym(s, _) => Ok(s.as_str()),
                        other => err(other.at(), "`(v …)` takes symbols"),
                    })
                    .collect::<Result<_, _>>()?;
                if words.is_empty() {
                    return err(*cat, "`(v …)` needs a verb");
                }
                verb = Some(words.join(" "));
            }
            ("np" | "pron", _) => {
                if object.is_some() {
                    return err(*cat, "`(vp …)` takes at most one object");
                }
                object = Some(compile_nominal(item)?);
            }
            ("pp", _) => pps.push(compile_pp(child, *cat)?),
            (other, cat) => return err(cat, format!("unknown vp child `{other}`")),
        }
    }
    let Some(verb) = verb else {
        return err(*at, "`(vp …)` needs a `(v …)`");
    };
    Ok(VerbPhrase { verb, object, pps })
}

fn compile_pp(items: &[Value], at: usize) -> Result<PrepPhrase, SexprError> {
    let mut preposition = None;
    let mut case = None;
    let mut object = None;
    let mut rest = items[1..].iter().peekable();
    while let Some(item) = rest.next() {
        match item {
            Value::Key(key, kat) if key == "case" => match rest.next() {
                Some(Value::Sym(s, sat)) => case = Some(case_of(s, *sat)?),
                _ => return err(*kat, "`:case` needs a case name"),
            },
            Value::Key(key, kat) => return err(*kat, format!("unknown pp key `:{key}`")),
            Value::List(child, cat) => match head_of(child, *cat)? {
                ("prep", _) => preposition = Some(sym_arg(child, "prep", *cat)?),
                ("np" | "pron", _) => object = Some(compile_nominal(item)?),
                (other, cat) => return err(cat, format!("unknown pp child `{other}`")),
            },
            other => return err(other.at(), "unexpected atom inside `(pp …)`"),
        }
    }
    match (preposition, object) {
        (Some(preposition), Some(object)) => Ok(PrepPhrase {
            preposition,
            case,
            object,
        }),
        _ => err(at, "`(pp …)` needs a `(prep …)` and an object"),
    }
}

// ---------------------------------------------------------------------------
// Canonical printer.
// ---------------------------------------------------------------------------

/// Print a clause in canonical form: children in fixed order, only
/// non-default keys emitted. `compile_clause(parse(print(c)))? == *c`.
pub fn print(clause: &Clause) -> String {
    let mut out = String::from("(clause ");
    print_nominal(&clause.subject, &mut out);
    out.push(' ');
    print_vp(&clause.vp, &mut out);
    match clause.tense {
        TenseSpec::Present => {}
        TenseSpec::Past => out.push_str(" :tense past"),
        TenseSpec::Future => out.push_str(" :tense future"),
    }
    if clause.polarity == Polarity::Negative {
        out.push_str(" :neg");
    }
    match clause.force {
        Force::Declarative => {}
        Force::LiQuestion => out.push_str(" :force li"),
        Force::CiQuestion => out.push_str(" :force či"),
        Force::IntonationQuestion => out.push_str(" :force intonation"),
    }
    if clause.prodrop {
        out.push_str(" :prodrop");
    }
    out.push(')');
    out
}

fn print_nominal(nominal: &Nominal, out: &mut String) {
    match nominal {
        Nominal::Np(np) => {
            out.push_str("(np");
            if let Some(case) = np.case_override {
                out.push_str(" :case ");
                out.push_str(case_name(case));
            }
            if let Some(det) = &np.determiner {
                out.push_str(&format!(" (det {det})"));
            }
            if let Some(n) = np.count {
                out.push_str(&format!(" (num {n})"));
            }
            for a in &np.adjectives {
                out.push_str(&format!(" (adj {a})"));
            }
            out.push_str(&format!(" (n {}))", np.head));
        }
        Nominal::Pron {
            person,
            number,
            gender,
        } => {
            let p = match person {
                Person::First => "1",
                Person::Second => "2",
                Person::Third => "3",
            };
            let n = match number {
                Number::Singular => "sg",
                Number::Plural => "pl",
            };
            let g = match gender {
                Gender::Masculine => "m",
                Gender::Feminine => "f",
                Gender::Neuter => "n",
            };
            out.push_str(&format!("(pron :{p} :{n} :{g})"));
        }
    }
}

fn print_vp(vp: &VerbPhrase, out: &mut String) {
    out.push_str(&format!("(vp (v {})", vp.verb));
    if let Some(object) = &vp.object {
        out.push(' ');
        print_nominal(object, out);
    }
    for pp in &vp.pps {
        out.push_str(&format!(" (pp (prep {})", pp.preposition));
        if let Some(case) = pp.case {
            out.push_str(" :case ");
            out.push_str(case_name(case));
        }
        out.push(' ');
        print_nominal(&pp.object, out);
        out.push(')');
    }
    out.push(')');
}

/// Parse + compile in one step — the template entry point.
pub fn clause_from_str(input: &str) -> Result<Clause, SexprError> {
    compile_clause(&parse(input)?)
}
