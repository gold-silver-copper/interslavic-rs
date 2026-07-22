//! Abstract syntax: typed nodes over citation-form leaves.
//!
//! Leaves carry the flavored citation forms the `interslavic` facade
//! expects (Nom-sg nouns, masc-Nom-sg adjectives, infinitives). Case is
//! assigned by the *slot* a nominal occupies (subject → Nom, object →
//! Acc, PP → the preposition's government), never stored on the leaf;
//! [`NounPhrase::case_override`] exists only for marked constructions.

use interslavic::{Case, Gender, Number, Person};

/// A noun phrase: optional determiner and count, adjective modifiers, and
/// the head noun that fixes gender/animacy for the whole phrase.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NounPhrase {
    pub determiner: Option<String>,
    pub count: Option<u64>,
    pub adjectives: Vec<String>,
    pub head: String,
    pub case_override: Option<Case>,
}

impl NounPhrase {
    pub fn new(head: &str) -> Self {
        Self {
            determiner: None,
            count: None,
            adjectives: Vec::new(),
            head: head.trim().to_string(),
            case_override: None,
        }
    }
    pub fn det(mut self, determiner: &str) -> Self {
        self.determiner = Some(determiner.trim().to_string());
        self
    }
    pub fn count(mut self, n: u64) -> Self {
        self.count = Some(n);
        self
    }
    pub fn adj(mut self, adjective: &str) -> Self {
        self.adjectives.push(adjective.trim().to_string());
        self
    }
    pub fn case(mut self, case: Case) -> Self {
        self.case_override = Some(case);
        self
    }
}

/// Anything that can fill a nominal slot: a full noun phrase or a
/// personal pronoun (whose form series — full vs prepositional n- form —
/// is chosen by the slot at realization time).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Nominal {
    Np(NounPhrase),
    Pron {
        person: Person,
        number: Number,
        gender: Gender,
    },
}

impl From<NounPhrase> for Nominal {
    fn from(np: NounPhrase) -> Self {
        Nominal::Np(np)
    }
}

/// A prepositional phrase. `case` may be omitted only for prepositions
/// that govern a single case; multi-sense prepositions require it (the
/// diagnostic lists the senses).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PrepPhrase {
    pub preposition: String,
    pub case: Option<Case>,
    pub object: Nominal,
}

/// A verb phrase: the verb (an infinitive; a trailing ` sę` marks it
/// reflexive, as does the dictionary's own `v.refl.` metadata), an
/// optional direct object, and prepositional adjuncts.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VerbPhrase {
    pub verb: String,
    pub object: Option<Nominal>,
    pub pps: Vec<PrepPhrase>,
}

impl VerbPhrase {
    pub fn new(verb: &str) -> Self {
        Self {
            verb: verb.trim().to_string(),
            object: None,
            pps: Vec::new(),
        }
    }
    pub fn object(mut self, object: impl Into<Nominal>) -> Self {
        self.object = Some(object.into());
        self
    }
    pub fn pp(mut self, pp: PrepPhrase) -> Self {
        self.pps.push(pp);
        self
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TenseSpec {
    Present,
    Past,
    Future,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Polarity {
    Affirmative,
    Negative,
}

/// Clause force. The three overt yes/no question strategies are steen's
/// (syntax page): intonation only, fronted `či`, or the particle `li`
/// "placed right after the focus point of the question, usually the
/// verb" — phase 1 always focuses the finite verb.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Force {
    Declarative,
    IntonationQuestion,
    CiQuestion,
    LiQuestion,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Clause {
    pub subject: Nominal,
    pub vp: VerbPhrase,
    pub tense: TenseSpec,
    pub polarity: Polarity,
    pub force: Force,
    /// Pro-drop is OFF by default: steen (pronouns page) recommends
    /// keeping the subject pronoun ("ja čitaju" over "čitaju").
    pub prodrop: bool,
}

impl Clause {
    pub fn new(subject: impl Into<Nominal>, vp: VerbPhrase) -> Self {
        Self {
            subject: subject.into(),
            vp,
            tense: TenseSpec::Present,
            polarity: Polarity::Affirmative,
            force: Force::Declarative,
            prodrop: false,
        }
    }
    pub fn tense(mut self, tense: TenseSpec) -> Self {
        self.tense = tense;
        self
    }
    pub fn past(self) -> Self {
        self.tense(TenseSpec::Past)
    }
    pub fn future(self) -> Self {
        self.tense(TenseSpec::Future)
    }
    pub fn negated(mut self) -> Self {
        self.polarity = Polarity::Negative;
        self
    }
    pub fn force(mut self, force: Force) -> Self {
        self.force = force;
        self
    }
    pub fn prodrop(mut self) -> Self {
        self.prodrop = true;
        self
    }
}

/// Builder shorthands mirroring `english-phrase`'s lowercase entry points.
pub fn np(head: &str) -> NounPhrase {
    NounPhrase::new(head)
}
pub fn vp(verb: &str) -> VerbPhrase {
    VerbPhrase::new(verb)
}
pub fn pp(preposition: &str, object: impl Into<Nominal>) -> PrepPhrase {
    PrepPhrase {
        preposition: preposition.trim().to_string(),
        case: None,
        object: object.into(),
    }
}
impl PrepPhrase {
    pub fn case(mut self, case: Case) -> Self {
        self.case = Some(case);
        self
    }
}
pub fn pron(person: Person, number: Number, gender: Gender) -> Nominal {
    Nominal::Pron {
        person,
        number,
        gender,
    }
}
pub fn clause(subject: impl Into<Nominal>, vp: VerbPhrase) -> Clause {
    Clause::new(subject, vp)
}
