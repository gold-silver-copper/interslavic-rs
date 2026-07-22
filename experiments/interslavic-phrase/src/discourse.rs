//! Microplanning: from a sequence of clauses to connected prose.
//!
//! The step above realization in the classical NLG pipeline (Reiter &
//! Dale 2000). Three components, each usable alone, all TREE-TO-TREE —
//! the string is produced exactly once, by [`crate::realize`], so the
//! never-post-process contract extends upward:
//!
//! - **Referring expressions** (frame: Krahmer & van Deemter 2012):
//!   entity-tagged NPs render in full on first mention and pronominalize
//!   afterwards; the full NP returns after an interfering same-featured
//!   entity. The salience model is deliberately just recency +
//!   interference — no scoring.
//! - **Aggregation**: adjacent clauses sharing subject entity, tense,
//!   polarity, mood, voice, and declarative force merge into one clause
//!   with VP coordination ("Krålj kupil knigų i pročital jų"-shaped).
//! - **Connectives**: a small curated, dictionary-attested table
//!   (`potom` "then", `ale` "but", `zato` "therefore", `tomu` "hence",
//!   `i` "and"), rendered sentence-initially.

use crate::ast::*;
use crate::realize::{PhraseError, RealizeOpts, realize};
use interslavic::{Gender, Number, Person, noun_info};
use std::collections::HashMap;

/// Sentence-initial discourse connectives — a closed table.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Connective {
    /// "then, afterwards"
    Potom,
    /// "but"
    Ale,
    /// "therefore, for that"
    Zato,
    /// "hence, so"
    Tomu,
    /// "and"
    I,
}

impl Connective {
    pub fn word(self) -> &'static str {
        match self {
            Connective::Potom => "potom",
            Connective::Ale => "ale",
            Connective::Zato => "zato",
            Connective::Tomu => "tomu",
            Connective::I => "i",
        }
    }
}

/// One sentence of a narrative: an optional connective plus a clause.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DiscourseSentence {
    pub connective: Option<Connective>,
    pub clause: Clause,
}

impl DiscourseSentence {
    pub fn new(clause: Clause) -> Self {
        Self {
            connective: None,
            clause,
        }
    }
    pub fn connective(mut self, connective: Connective) -> Self {
        self.connective = Some(connective);
        self
    }
}

#[derive(Clone, Copy)]
struct EntityFeatures {
    gender: Gender,
    number: Number,
}

/// State of the referring-expression pass.
#[derive(Default)]
struct Mentions {
    /// entity id → (features, sequence number of the last mention).
    seen: HashMap<String, (EntityFeatures, usize)>,
    counter: usize,
}

impl Mentions {
    /// Decide whether this mention pronominalizes, and record it.
    fn mention(&mut self, id: &str, features: EntityFeatures) -> bool {
        self.counter += 1;
        let now = self.counter;
        let pronominalize = match self.seen.get(id) {
            None => false,
            Some((_, last)) => {
                // Interference: some OTHER entity with the same
                // gender+number mentioned since our last mention forces
                // the full NP again.
                let interfered = self.seen.iter().any(|(other, (other_features, at))| {
                    other != id
                        && other_features.gender == features.gender
                        && other_features.number == features.number
                        && at > last
                });
                !interfered
            }
        };
        self.seen.insert(id.to_string(), (features, now));
        pronominalize
    }
}

fn np_features(np: &NounPhrase) -> EntityFeatures {
    let info = noun_info(&np.head);
    EntityFeatures {
        gender: info.gender,
        number: if info.plural_only {
            Number::Plural
        } else {
            Number::Singular
        },
    }
}

/// Replace later mentions of entity-tagged NPs with pronouns, in place.
fn pronominalize_nominal(nominal: &mut Nominal, mentions: &mut Mentions) {
    match nominal {
        Nominal::Np(np) => {
            if let Some(id) = np.entity.clone() {
                let features = np_features(np);
                if mentions.mention(&id, features) {
                    *nominal = Nominal::Pron {
                        person: Person::Third,
                        number: features.number,
                        gender: features.gender,
                        clitic: false,
                    };
                }
            }
        }
        Nominal::Coord(coordination) => {
            for item in &mut coordination.items {
                pronominalize_nominal(item, mentions);
            }
        }
        _ => {}
    }
}

fn pronominalize_clause(clause: &mut Clause, mentions: &mut Mentions) {
    pronominalize_nominal(&mut clause.subject, mentions);
    match &mut clause.core {
        ClauseCore::Verbal(coordination) => {
            for verb_phrase in &mut coordination.items {
                if let Some(object) = &mut verb_phrase.object {
                    pronominalize_nominal(object, mentions);
                }
                for pp in &mut verb_phrase.pps {
                    pronominalize_nominal(&mut pp.object, mentions);
                }
            }
        }
        ClauseCore::Copular { predicate, .. } => {
            if let Predicate::Nominal(np) = predicate {
                let mut wrapped = Nominal::Np(np.clone());
                pronominalize_nominal(&mut wrapped, mentions);
                if let Nominal::Np(new_np) = wrapped {
                    *np = new_np;
                }
                // A pronominalized copular predicate stays a full NP:
                // "on jest on" is not a sentence. Mention tracking still
                // recorded it above.
            }
        }
    }
}

/// Do two subjects refer to the same thing — identical trees, or NPs
/// tagged with the same entity id (the surface NPs may differ: first
/// mention "toj krålj", second bare)?
fn same_subject(first: &Nominal, second: &Nominal) -> bool {
    if first == second {
        return true;
    }
    match (first, second) {
        (Nominal::Np(a), Nominal::Np(b)) => a.entity.is_some() && a.entity == b.entity,
        _ => false,
    }
}

/// Can `second` be aggregated into `first` as VP coordination?
fn can_aggregate(first: &Clause, second: &DiscourseSentence) -> bool {
    second.connective.is_none()
        && same_subject(&first.subject, &second.clause.subject)
        && first.tense == second.clause.tense
        && first.polarity == second.clause.polarity
        && first.mood == second.clause.mood
        && first.voice == second.clause.voice
        && first.force == Force::Declarative
        && second.clause.force == Force::Declarative
        && first.topic.is_none()
        && first.focus.is_none()
        && second.clause.topic.is_none()
        && second.clause.focus.is_none()
        && matches!(first.core, ClauseCore::Verbal(_))
        && matches!(second.clause.core, ClauseCore::Verbal(_))
}

/// Realize a narrative: aggregation first (tree-to-tree), then the
/// referring-expression pass (tree-to-tree), then one realization per
/// sentence.
///
/// ```
/// use interslavic_phrase::*;
/// use interslavic_phrase::discourse::*;
///
/// let story = vec![
///     DiscourseSentence::new(
///         clause(
///             np("krålj").entity("k").det("toj"),
///             vp("kupiti").object(np("kniga").entity("b")),
///         )
///         .past(),
///     ),
///     DiscourseSentence::new(
///         clause(
///             np("krålj").entity("k"),
///             vp("pročitati").object(np("kniga").entity("b")),
///         )
///         .past(),
///     )
///     .connective(Connective::Potom),
/// ];
/// assert_eq!(
///     narrate(&story, RealizeOpts::sentence()).unwrap(),
///     "Toj krålj kupil knigų. Potom on pročital jų."
/// );
/// ```
pub fn narrate(sentences: &[DiscourseSentence], opts: RealizeOpts) -> Result<String, PhraseError> {
    // Aggregation pass (tree-to-tree).
    let mut aggregated: Vec<DiscourseSentence> = Vec::new();
    for sentence in sentences {
        if let Some(previous) = aggregated.last_mut() {
            if can_aggregate(&previous.clause, sentence) {
                let (ClauseCore::Verbal(target), ClauseCore::Verbal(source)) =
                    (&mut previous.clause.core, &sentence.clause.core)
                else {
                    unreachable!("can_aggregate checked verbal cores");
                };
                target.items.extend(source.items.iter().cloned());
                continue;
            }
        }
        aggregated.push(sentence.clone());
    }

    // Referring-expression pass (tree-to-tree).
    let mut mentions = Mentions::default();
    for sentence in &mut aggregated {
        pronominalize_clause(&mut sentence.clause, &mut mentions);
    }

    // Realization, one string per sentence.
    let mut out = String::new();
    for sentence in &aggregated {
        if !out.is_empty() {
            out.push(' ');
        }
        match sentence.connective {
            Some(connective) => {
                // The connective takes the capital; the clause itself is
                // realized without sentence casing, then punctuated.
                let mut body = realize(&sentence.clause, RealizeOpts::plain())?;
                let word = connective.word();
                let mut first = word.chars();
                let capitalized = match first.next() {
                    Some(c) => c.to_uppercase().collect::<String>() + first.as_str(),
                    None => String::new(),
                };
                body = format!("{capitalized} {body}");
                if opts.sentence {
                    body.push(match sentence.clause.force {
                        Force::Declarative => '.',
                        Force::Imperative(_) => '!',
                        _ => '?',
                    });
                }
                out.push_str(&body);
            }
            None => {
                out.push_str(&realize(&sentence.clause, opts)?);
            }
        }
    }
    Ok(out)
}
