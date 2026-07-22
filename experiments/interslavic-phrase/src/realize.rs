//! Agreement/government resolution and linearization.
//!
//! Every inflected form comes from the `interslavic` facade — this module
//! never post-processes forms (the INTEGRATION.md contract; punctuation
//! attachment is the one string operation, and it touches only commas and
//! terminal marks, never word forms). Linearization defaults are steen's
//! (syntax page): S–V–O neutral; "modifiers usually precede the noun";
//! postverbal clitics (steen's own examples); `li` "right after the focus
//! point of the question, usually the verb"; non-SVO orders are sanctioned
//! "when special emphasis is needed", with steen's own clarity caveat
//! surfaced as a warning when Nom/Acc syncretism would garden-path.
//! Adverb position (before the verb) is POLICY — the sources are silent.

use crate::ast::*;
use interslavic::{
    Animacy, Case, Gender, Number, Person, PronounStyle, Provenance, Tense, adj, cells, noun_with,
    passive_participle, perfect_parts, personal_pronoun, preposition_cases, preposition_senses,
    pronoun, quantified_parts_with_info, verb, verb_forms, verb_info,
};
use std::fmt;

/// Where the clause's clitic cluster lands.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum CliticStyle {
    /// Immediately after the verb complex — steen's own examples
    /// ("Ja myju se"). The default.
    #[default]
    Postverbal,
    /// Wackernagel second position: after the first constituent.
    SecondPosition,
}

/// Realization options, in the `english-phrase` style.
#[derive(Debug, Clone, Copy, Default)]
pub struct RealizeOpts {
    pub sentence: bool,
    /// Treat a `Provenance::Guessed` NP head as an error instead of
    /// trusting the rule engine's gender/animacy guess.
    pub strict_guessed: bool,
    pub clitic_style: CliticStyle,
}

impl RealizeOpts {
    pub fn plain() -> Self {
        Self::default()
    }
    pub fn sentence() -> Self {
        Self {
            sentence: true,
            ..Self::default()
        }
    }
    pub fn strict(mut self) -> Self {
        self.strict_guessed = true;
        self
    }
    pub fn clitics(mut self, style: CliticStyle) -> Self {
        self.clitic_style = style;
        self
    }
}

/// Diagnostics are values; realization of a well-formed tree is total.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PhraseError {
    UnknownPreposition(String),
    AmbiguousPreposition {
        preposition: String,
        senses: Vec<(Case, &'static str)>,
    },
    InvalidPrepositionCase {
        preposition: String,
        case: Case,
        allowed: Vec<Case>,
    },
    ObjectOfIntransitive {
        verb: String,
    },
    /// A passive clause whose verb phrase still carries an object — in
    /// the passive the patient IS the subject.
    ObjectInPassive {
        verb: String,
    },
    GuessedHead {
        lemma: String,
    },
    /// Coordination with no items, or another structurally empty node.
    EmptyCoordination,
    Unsupported(&'static str),
}

impl fmt::Display for PhraseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PhraseError::UnknownPreposition(p) => write!(f, "unknown preposition `{p}`"),
            PhraseError::AmbiguousPreposition {
                preposition,
                senses,
            } => {
                write!(
                    f,
                    "preposition `{preposition}` governs several cases; pick one:"
                )?;
                for (case, gloss) in senses {
                    write!(f, " {case:?} = \"{gloss}\";")?;
                }
                Ok(())
            }
            PhraseError::InvalidPrepositionCase {
                preposition,
                case,
                allowed,
            } => write!(
                f,
                "`{preposition}` does not govern {case:?} (allowed: {allowed:?})"
            ),
            PhraseError::ObjectOfIntransitive { verb } => {
                write!(
                    f,
                    "`{verb}` is intransitive in the dictionary but has a direct object"
                )
            }
            PhraseError::ObjectInPassive { verb } => {
                write!(
                    f,
                    "passive clause of `{verb}` carries an object; the patient is the subject"
                )
            }
            PhraseError::GuessedHead { lemma } => {
                write!(
                    f,
                    "`{lemma}` is not a dictionary noun (gender/animacy would be guessed)"
                )
            }
            PhraseError::EmptyCoordination => write!(f, "coordination with no items"),
            PhraseError::Unsupported(what) => write!(f, "unsupported: {what}"),
        }
    }
}

impl std::error::Error for PhraseError {}

/// Warnings never change output; they surface steen's caveats and the
/// dictionary's opinions.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PhraseWarning {
    /// A perfective verb in the present tense reads as future/completed
    /// in Slavic — grammatical, but probably not "present".
    PerfectivePresent { verb: String },
    /// An explicit object case contradicts the dictionary's government
    /// annotation.
    GovernsConflict {
        verb: String,
        dictionary: Case,
        used: Case,
    },
    /// A non-SVO order where neither subject nor object morphologically
    /// distinguishes Nom from Acc — steen's own clarity caveat.
    AmbiguousOrder,
}

/// The result of checked realization: the text plus any warnings.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Realized {
    pub text: String,
    pub warnings: Vec<PhraseWarning>,
}

/// One clean surface form: byform cells resolve to the first variant and
/// citation accents are stripped — via `cells::variants`, the documented
/// normalization step.
fn surface(form: &str) -> String {
    cells::variants(form).into_iter().next().unwrap_or_default()
}

/// A rendered nominal plus the agreement features it projects.
struct Rendered {
    words: Vec<String>,
    person: Person,
    number: Number,
    gender: Gender,
    animacy: Animacy,
    /// A clitic form to route into the clause cluster instead of the
    /// in-place words (pronominal clitic objects).
    clitic: Option<(Case, String)>,
}

struct Ctx<'o> {
    opts: &'o RealizeOpts,
    warnings: Vec<PhraseWarning>,
}

/// Marker prefix for a comma that attaches to the PREVIOUS word (used by
/// relative clauses); dropped against terminal punctuation.
const ATTACH_COMMA: char = '\u{1}';

fn render_nominal(
    nominal: &Nominal,
    slot_case: Case,
    style: PronounStyle,
    ctx: &mut Ctx,
) -> Result<Rendered, PhraseError> {
    match nominal {
        Nominal::Pron {
            person,
            number,
            gender,
            clitic,
        } => {
            let full = personal_pronoun(*person, *number, *gender, slot_case, style)
                .or_else(|| {
                    personal_pronoun(*person, *number, *gender, slot_case, PronounStyle::Full)
                })
                .expect("full personal-pronoun cells are total");
            // A clitic is routed to the cluster only when the cell has
            // one and the slot is not prepositional (steen: full forms
            // after prepositions).
            let clitic_form = if *clitic && style != PronounStyle::AfterPreposition {
                personal_pronoun(*person, *number, *gender, slot_case, PronounStyle::Clitic)
            } else {
                None
            };
            Ok(Rendered {
                words: if clitic_form.is_some() {
                    Vec::new()
                } else {
                    vec![full]
                },
                person: *person,
                number: *number,
                gender: *gender,
                animacy: Animacy::Animate,
                clitic: clitic_form.map(|form| (slot_case, form)),
            })
        }
        Nominal::Name {
            text,
            gender,
            indeclinable,
        } => {
            let word = if *indeclinable {
                text.clone()
            } else {
                surface(&noun_with(
                    text,
                    slot_case,
                    Number::Singular,
                    *gender,
                    Animacy::Animate,
                ))
            };
            Ok(Rendered {
                words: vec![word],
                person: Person::Third,
                number: Number::Singular,
                gender: *gender,
                animacy: Animacy::Animate,
                clitic: None,
            })
        }
        Nominal::Np(np) => render_np(np, slot_case, ctx),
        Nominal::Coord(coordination) => {
            if coordination.items.is_empty() {
                return Err(PhraseError::EmptyCoordination);
            }
            let mut rendered = Vec::new();
            for item in &coordination.items {
                rendered.push(render_nominal(item, slot_case, style, ctx)?);
            }
            let mut words: Vec<String> = Vec::new();
            let last = rendered.len() - 1;
            for (index, item) in rendered.iter().enumerate() {
                if index > 0 {
                    if index == last {
                        words.push(coordination.conjunction.word().to_string());
                    } else if let Some(previous) = words.last_mut() {
                        // Serial style: comma between non-final items,
                        // conjunction only before the last; no comma
                        // before `i` (POLICY, Slavic convention).
                        previous.push(',');
                    }
                }
                words.extend(item.words.iter().cloned());
            }
            // Coordinated-subject agreement (POLICY, pan-Slavic
            // defaults): >1 conjunct → plural; mixed gender → masculine;
            // person resolves 1st > 2nd > 3rd.
            let number =
                if rendered.len() > 1 || rendered.iter().any(|r| r.number == Number::Plural) {
                    Number::Plural
                } else {
                    Number::Singular
                };
            let gender = if rendered.iter().all(|r| r.gender == rendered[0].gender) {
                rendered[0].gender
            } else {
                Gender::Masculine
            };
            let person = rendered
                .iter()
                .map(|r| r.person)
                .min_by_key(|p| match p {
                    Person::First => 0,
                    Person::Second => 1,
                    Person::Third => 2,
                })
                .unwrap_or(Person::Third);
            Ok(Rendered {
                words,
                person,
                number,
                gender,
                animacy: Animacy::Animate,
                clitic: None,
            })
        }
    }
}

fn render_np(np: &NounPhrase, slot_case: Case, ctx: &mut Ctx) -> Result<Rendered, PhraseError> {
    let slot_case = np.case_override.unwrap_or(slot_case);
    let info = interslavic::noun_info(&np.head);
    if ctx.opts.strict_guessed && info.provenance == Provenance::Guessed {
        return Err(PhraseError::GuessedHead {
            lemma: np.head.clone(),
        });
    }
    let (gender, animacy) = (info.gender, info.animacy);

    let modifier = |lemma: &str, case: Case, number: Number| -> String {
        pronoun(lemma, case, number, gender, animacy)
            .unwrap_or_else(|| adj(lemma, case, number, gender, animacy))
    };

    let mut words = Vec::new();
    let (number, agr_gender);
    if let Some(n) = np.count {
        let parts =
            quantified_parts_with_info(n, &np.head, slot_case, gender, animacy, info.plural_only);
        if let Some(det) = &np.determiner {
            words.push(surface(&modifier(det, parts.case, parts.number)));
        }
        words.push(n.to_string());
        for adjective in &np.adjectives {
            words.push(surface(&modifier(adjective, parts.case, parts.number)));
        }
        words.push(surface(&parts.noun));
        // Verb agreement with a counted subject: steen states 3sg-neuter
        // for collective subjects; extending it to all gen-pl quantified
        // subjects is POLICY.
        (number, agr_gender) = match n {
            1 => (Number::Singular, gender),
            2..=4 => (Number::Plural, gender),
            _ => (Number::Singular, Gender::Neuter),
        };
    } else {
        let noun_number = if info.plural_only {
            Number::Plural
        } else {
            Number::Singular
        };
        if let Some(det) = &np.determiner {
            words.push(surface(&modifier(det, slot_case, noun_number)));
        }
        for adjective in &np.adjectives {
            words.push(surface(&modifier(adjective, slot_case, noun_number)));
        }
        words.push(surface(&noun_with(
            &np.head,
            slot_case,
            noun_number,
            gender,
            animacy,
        )));
        (number, agr_gender) = (noun_number, gender);
    }

    if let Some(rel) = &np.relative {
        words.extend(render_relative(rel, number, agr_gender, animacy, ctx)?);
    }

    Ok(Rendered {
        words,
        person: Person::Third,
        number,
        gender: agr_gender,
        animacy,
        clitic: None,
    })
}

/// The relative clause: comma, (preposition,) relativizer agreeing with
/// the head in gender/number and taking its case from the gap role, then
/// the clause body, then a trailing comma (dropped against sentence
/// punctuation). Comma-delimited relatives are POLICY (pan-Slavic
/// convention; steen's pages show no relative punctuation examples).
fn render_relative(
    rel: &RelClause,
    head_number: Number,
    head_gender: Gender,
    head_animacy: Animacy,
    ctx: &mut Ctx,
) -> Result<Vec<String>, PhraseError> {
    if rel.relativizer == Relativizer::Iže {
        // The facade has no iže paradigm — a declared gap and a facade
        // finding, not silent wrong output.
        return Err(PhraseError::Unsupported(
            "the iže relativizer (no facade paradigm)",
        ));
    }
    let (bare, lemma_reflexive) = strip_reflexive(&rel.vp.verb);
    let reflexive = lemma_reflexive
        || verb_info(bare)
            .as_ref()
            .is_some_and(|entry| entry.reflexive);
    let rel_case = match &rel.gap {
        GapRole::Subject => Case::Nom,
        GapRole::Object => verb_info(bare)
            .and_then(|info| info.governs)
            .unwrap_or(Case::Acc),
        GapRole::PpObject { case, .. } => *case,
    };
    let relativizer = pronoun("ktory", rel_case, head_number, head_gender, head_animacy)
        .expect("ktory declines for every cell");

    let mut words = vec![format!("{ATTACH_COMMA}")];
    if let GapRole::PpObject { preposition, .. } = &rel.gap {
        words.push(preposition.clone());
    }
    words.push(surface(&relativizer));

    // Agreement inside the relative clause: a subject gap agrees with
    // the head (3rd person); otherwise with the overt subject.
    let (person, number, gender, subject_words) = match (&rel.gap, &rel.subject) {
        (GapRole::Subject, _) => (Person::Third, head_number, head_gender, Vec::new()),
        (_, Some(subject)) => {
            let rendered = render_nominal(subject, Case::Nom, PronounStyle::Full, ctx)?;
            (
                rendered.person,
                rendered.number,
                rendered.gender,
                rendered.words,
            )
        }
        (_, None) => {
            return Err(PhraseError::Unsupported(
                "a non-subject gap needs an overt subject",
            ));
        }
    };
    words.extend(subject_words);

    let mut verb_words = render_verb_complex(
        bare,
        rel.tense,
        Mood::Indicative,
        rel.polarity,
        person,
        number,
        gender,
        &mut ctx.warnings,
    );
    words.append(&mut verb_words);
    if reflexive {
        words.push("sę".to_string());
    }

    if rel.gap != GapRole::Object {
        if let Some(object) = &rel.vp.object {
            let object_case = verb_info(bare)
                .and_then(|info| info.governs)
                .unwrap_or(Case::Acc);
            let rendered = render_nominal(object, object_case, PronounStyle::Full, ctx)?;
            words.extend(rendered.words);
        }
    }
    for adjunct in &rel.vp.pps {
        words.extend(render_pp(adjunct, ctx)?);
    }
    words.push(format!("{ATTACH_COMMA}"));
    Ok(words)
}

fn render_pp(pp: &PrepPhrase, ctx: &mut Ctx) -> Result<Vec<String>, PhraseError> {
    let allowed = preposition_cases(&pp.preposition)
        .ok_or_else(|| PhraseError::UnknownPreposition(pp.preposition.clone()))?;
    let case = match pp.case {
        Some(case) if allowed.contains(&case) => case,
        Some(case) => {
            return Err(PhraseError::InvalidPrepositionCase {
                preposition: pp.preposition.clone(),
                case,
                allowed: allowed.to_vec(),
            });
        }
        None if allowed.len() == 1 => allowed[0],
        None => {
            return Err(PhraseError::AmbiguousPreposition {
                preposition: pp.preposition.clone(),
                senses: preposition_senses(&pp.preposition)
                    .expect("senses table agrees with cases table")
                    .to_vec(),
            });
        }
    };
    let object = render_nominal(&pp.object, case, PronounStyle::AfterPreposition, ctx)?;
    let mut words = vec![pp.preposition.clone()];
    words.extend(object.words);
    Ok(words)
}

fn strip_reflexive(verb_lemma: &str) -> (&str, bool) {
    match verb_lemma.strip_suffix(" sę") {
        Some(bare) => (bare, true),
        None => (verb_lemma, false),
    }
}

/// The finite verb complex WITHOUT the reflexive clitic (that joins the
/// clause cluster in main clauses): negation, then tense/mood forms.
/// Conditional: the person-marked auxiliary is the first word of the
/// facade's own conditional paradigm cell (byh/bys/by/…), the participle
/// from `perfect_parts` — one stem implementation, no local table.
#[allow(clippy::too_many_arguments)]
fn render_verb_complex(
    verb_lemma: &str,
    tense: TenseSpec,
    mood: Mood,
    polarity: Polarity,
    person: Person,
    number: Number,
    gender: Gender,
    warnings: &mut Vec<PhraseWarning>,
) -> Vec<String> {
    let mut words = Vec::new();
    if polarity == Polarity::Negative {
        words.push("ne".to_string());
    }
    match mood {
        Mood::Conditional => {
            let slot = compound_slot(person, number, gender);
            let cell = verb_forms(verb_lemma).conditional[slot].clone();
            let auxiliary = surface(&cell)
                .split_whitespace()
                .next()
                .unwrap_or("by")
                .to_string();
            words.push(auxiliary);
            words.push(perfect_parts(verb_lemma, person, number, gender).participle);
        }
        Mood::Indicative => match tense {
            TenseSpec::Present => {
                if verb_info(verb_lemma).and_then(|info| info.aspect)
                    == Some(interslavic::Aspect::Pf)
                {
                    warnings.push(PhraseWarning::PerfectivePresent {
                        verb: verb_lemma.to_string(),
                    });
                }
                words.push(surface(&verb(
                    verb_lemma,
                    person,
                    number,
                    gender,
                    Tense::Present,
                )));
            }
            TenseSpec::Future => {
                let complex = surface(&verb(verb_lemma, person, number, gender, Tense::Future));
                words.extend(complex.split_whitespace().map(str::to_string));
            }
            TenseSpec::Past => {
                let parts = perfect_parts(verb_lemma, person, number, gender);
                if let Some(auxiliary) = parts.auxiliary {
                    words.push(auxiliary);
                }
                words.push(parts.participle);
            }
        },
    }
    words
}

fn compound_slot(person: Person, number: Number, gender: Gender) -> usize {
    match (person, number, gender) {
        (Person::First, Number::Singular, _) => 0,
        (Person::Second, Number::Singular, _) => 1,
        (Person::Third, Number::Singular, Gender::Masculine) => 2,
        (Person::Third, Number::Singular, Gender::Feminine) => 3,
        (Person::Third, Number::Singular, Gender::Neuter) => 4,
        (Person::First, Number::Plural, _) => 5,
        (Person::Second, Number::Plural, _) => 6,
        (Person::Third, Number::Plural, _) => 7,
    }
}

/// Slot labels for information-structure movement.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Slot {
    Subject,
    Verb,
    Object,
}

/// Realize a clause to text + warnings — the checked entry point.
pub fn realize_checked(clause: &Clause, opts: RealizeOpts) -> Result<Realized, PhraseError> {
    let mut ctx = Ctx {
        opts: &opts,
        warnings: Vec::new(),
    };

    let imperative = matches!(clause.force, Force::Imperative(_));

    let subject = render_nominal(&clause.subject, Case::Nom, PronounStyle::Full, &mut ctx)?;
    let (person, number, gender) = if let Force::Imperative(addressee) = clause.force {
        // The imperative fixes its own agreement; the subject is omitted
        // by default (the one construction where pro-drop is the norm).
        match addressee {
            Addressee::You => (Person::Second, Number::Singular, Gender::Masculine),
            Addressee::We => (Person::First, Number::Plural, Gender::Masculine),
            Addressee::YouAll => (Person::Second, Number::Plural, Gender::Masculine),
        }
    } else {
        (subject.person, subject.number, subject.gender)
    };

    // Clitic cluster parts, in the source-cited order (Franks & King
    // 2000, A Handbook of Slavic Clitics): li > dative > accusative >
    // reflexive sę. Steen has no cluster table (verified); the order is
    // the pan-Slavic generalization, cited not invented.
    let mut cluster_dat: Option<String> = None;
    let mut cluster_acc: Option<String> = None;
    let mut cluster_se = false;

    let mut verb_words: Vec<String> = Vec::new();
    let mut object_words: Vec<String> = Vec::new();
    let mut rest_words: Vec<String> = Vec::new();
    let mut object_syncretic: Option<bool> = None;

    match &clause.core {
        ClauseCore::Copular {
            predicate,
            pred_case,
        } => {
            verb_words = if imperative {
                let mut words = Vec::new();
                if clause.polarity == Polarity::Negative {
                    words.push("ne".to_string());
                }
                words.push(surface(
                    &verb_forms("byti").imperative[imperative_slot(clause.force)],
                ));
                words
            } else {
                render_verb_complex(
                    "byti",
                    clause.tense,
                    clause.mood,
                    clause.polarity,
                    person,
                    number,
                    gender,
                    &mut ctx.warnings,
                )
            };
            let case = match pred_case {
                PredCase::Nominative => Case::Nom,
                PredCase::Instrumental => Case::Ins,
            };
            object_words = match predicate {
                Predicate::Nominal(np) => render_np(np, case, &mut ctx)?.words,
                Predicate::Adjectival(adjective) => {
                    vec![surface(&adj(
                        adjective,
                        case,
                        number,
                        gender,
                        subject.animacy,
                    ))]
                }
                Predicate::Participial(infinitive) => {
                    let participle =
                        passive_participle(infinitive, case, number, gender, subject.animacy)
                            .ok_or(PhraseError::Unsupported(
                                "no passive participle (intransitive verb?)",
                            ))?;
                    vec![surface(&participle)]
                }
            };
        }
        ClauseCore::Verbal(coordination) => {
            if coordination.items.is_empty() {
                return Err(PhraseError::EmptyCoordination);
            }
            for (index, verb_phrase) in coordination.items.iter().enumerate() {
                let first = index == 0;
                let sink: &mut Vec<String> = if first {
                    &mut verb_words
                } else {
                    &mut rest_words
                };
                let (bare, lemma_reflexive) = strip_reflexive(&verb_phrase.verb);
                let info = verb_info(bare);
                let reflexive =
                    lemma_reflexive || info.as_ref().is_some_and(|entry| entry.reflexive);

                if verb_phrase.object.is_some() {
                    if clause.voice == Voice::Passive {
                        return Err(PhraseError::ObjectInPassive {
                            verb: bare.to_string(),
                        });
                    }
                    // "Intransitive" in the dictionary means no
                    // ACCUSATIVE object; a (+N) government annotation
                    // licenses an oblique object (dękovati is v.intr.
                    // yet takes the dative).
                    if info.as_ref().and_then(|entry| entry.transitive) == Some(false)
                        && info.as_ref().and_then(|entry| entry.governs).is_none()
                    {
                        return Err(PhraseError::ObjectOfIntransitive {
                            verb: bare.to_string(),
                        });
                    }
                }

                if !first {
                    sink.push(if index == coordination.items.len() - 1 {
                        coordination.conjunction.word().to_string()
                    } else {
                        ",".to_string()
                    });
                }

                // Adverbs precede the verb complex (POLICY; the sources
                // are silent on neutral adverb position).
                sink.extend(verb_phrase.adverbs.iter().cloned());

                if imperative {
                    if clause.polarity == Polarity::Negative {
                        sink.push("ne".to_string());
                    }
                    sink.push(surface(
                        &verb_forms(bare).imperative[imperative_slot(clause.force)],
                    ));
                } else if clause.voice == Voice::Passive {
                    let mut copula_words = render_verb_complex(
                        "byti",
                        clause.tense,
                        clause.mood,
                        clause.polarity,
                        person,
                        number,
                        gender,
                        &mut ctx.warnings,
                    );
                    sink.append(&mut copula_words);
                    let participle =
                        passive_participle(bare, Case::Nom, number, gender, subject.animacy)
                            .ok_or(PhraseError::Unsupported(
                                "no passive participle (intransitive verb?)",
                            ))?;
                    sink.push(surface(&participle));
                } else {
                    let mut complex = render_verb_complex(
                        bare,
                        clause.tense,
                        clause.mood,
                        clause.polarity,
                        person,
                        number,
                        gender,
                        &mut ctx.warnings,
                    );
                    sink.append(&mut complex);
                }
                if reflexive {
                    if first {
                        cluster_se = true;
                    } else {
                        sink.push("sę".to_string());
                    }
                }

                if let Some(object) = &verb_phrase.object {
                    let dictionary = info.as_ref().and_then(|entry| entry.governs);
                    let explicit = match object {
                        Nominal::Np(np) => np.case_override,
                        _ => None,
                    };
                    let object_case = match (explicit, dictionary) {
                        (Some(used), Some(marked)) if used != marked => {
                            ctx.warnings.push(PhraseWarning::GovernsConflict {
                                verb: bare.to_string(),
                                dictionary: marked,
                                used,
                            });
                            used
                        }
                        (Some(used), _) => used,
                        (None, Some(marked)) => marked,
                        (None, None) => Case::Acc,
                    };
                    let rendered =
                        render_nominal(object, object_case, PronounStyle::Full, &mut ctx)?;
                    if let Some((case, form)) = rendered.clitic {
                        match case {
                            Case::Dat => cluster_dat = Some(form),
                            _ => cluster_acc = Some(form),
                        }
                    } else if first {
                        if object_case == Case::Acc {
                            object_syncretic = Some(nominal_nom_acc_syncretic(object, &mut ctx)?);
                        }
                        object_words = rendered.words;
                    } else {
                        rest_words.extend(rendered.words);
                    }
                }
                for adjunct in &verb_phrase.pps {
                    let pp_words = render_pp(adjunct, &mut ctx)?;
                    rest_words.extend(pp_words);
                }
            }
        }
    }

    // Non-SVO clarity guard (steen's caveat, cited): only when an order
    // change is requested and both subject and object are syncretic.
    let order_marked = clause.topic.is_some()
        || (clause.focus.is_some() && clause.focus != Some(SlotRef::Object))
        || (matches!(clause.force, Force::LiQuestion) && clause.focus.is_some())
        || clause.focus.is_some();
    if order_marked
        && object_syncretic == Some(true)
        && !imperative
        && nominal_nom_acc_syncretic(&clause.subject, &mut ctx)?
    {
        ctx.warnings.push(PhraseWarning::AmbiguousOrder);
    }

    // --- Linearization ---------------------------------------------------
    let subject_words = if imperative || clause.prodrop {
        Vec::new()
    } else {
        subject.words
    };
    let slot_words = |slot: Slot| -> &[String] {
        match slot {
            Slot::Subject => &subject_words,
            Slot::Verb => &verb_words,
            Slot::Object => &object_words,
        }
    };

    let mut words: Vec<String> = Vec::new();
    match clause.force {
        Force::LiQuestion => {
            // The focused constituent fronts with li right after it
            // (steen); default focus is the verb. The rest follows in
            // verb–subject–object order (steen's own example).
            let focus_slot = clause.focus.map(slot_of).unwrap_or(Slot::Verb);
            words.extend(slot_words(focus_slot).iter().cloned());
            words.push("li".to_string());
            for slot in [Slot::Verb, Slot::Subject, Slot::Object] {
                if slot != focus_slot {
                    words.extend(slot_words(slot).iter().cloned());
                }
            }
        }
        _ => {
            if clause.force == Force::CiQuestion {
                words.push("či".to_string());
            }
            // Default S V O; topic fronts, focus moves last (theme-first,
            // rheme-last — functional sentence perspective).
            let mut order = vec![Slot::Subject, Slot::Verb, Slot::Object];
            if let Some(topic) = clause.topic {
                let slot = slot_of(topic);
                order.retain(|s| *s != slot);
                order.insert(0, slot);
            }
            if let Some(focus) = clause.focus {
                let slot = slot_of(focus);
                order.retain(|s| *s != slot);
                order.push(slot);
            }
            for slot in order {
                words.extend(slot_words(slot).iter().cloned());
            }
        }
    }
    words.extend(rest_words);

    // Clitic cluster placement.
    let mut cluster: Vec<String> = Vec::new();
    if let Some(dative) = cluster_dat {
        cluster.push(dative);
    }
    if let Some(accusative) = cluster_acc {
        cluster.push(accusative);
    }
    if cluster_se {
        cluster.push("sę".to_string());
    }
    if !cluster.is_empty() {
        let position = match opts.clitic_style {
            CliticStyle::Postverbal => postverbal_position(&words, &verb_words),
            CliticStyle::SecondPosition => second_position(&words),
        };
        for (offset, clitic) in cluster.into_iter().enumerate() {
            words.insert(position + offset, clitic);
        }
    }

    let mut text = join_words(&words);
    if opts.sentence {
        let mut chars = text.chars();
        if let Some(first) = chars.next() {
            text = first.to_uppercase().collect::<String>() + chars.as_str();
        }
        text.push(match clause.force {
            Force::Declarative => '.',
            Force::Imperative(_) => '!',
            _ => '?',
        });
        text = fix_terminal_punctuation(text);
    } else {
        text = text
            .trim_end_matches(|c: char| c == ',' || c == ' ')
            .to_string();
    }
    Ok(Realized {
        text,
        warnings: ctx.warnings,
    })
}

/// Realize a clause — the warnings-discarding convenience over
/// [`realize_checked`].
///
/// ```
/// use interslavic_phrase::*;
///
/// // The flagship: count government decides the whole phrase once.
/// let tree = clause(
///     np("krålj").det("toj").adj("dobry"),
///     vp("ukrasti").object(np("moneta").count(5).adj("zlåty")),
/// )
/// .past();
/// assert_eq!(
///     realize(&tree, RealizeOpts::sentence()).unwrap(),
///     "Toj dobry krålj ukradl 5 zlåtyh monet."
/// );
///
/// // A gen-pl quantified SUBJECT takes a 3sg-neuter verb (steen's rule
/// // for collective subjects, extended to numeral subjects as policy).
/// let tree = clause(np("kot").count(5), vp("spati"));
/// assert_eq!(realize(&tree, RealizeOpts::sentence()).unwrap(), "5 kotov spi.");
///
/// // Ambiguous prepositions demand a case, and the error teaches:
/// let tree = clause(np("kot"), vp("spati").pp(pp("pod", np("stol"))));
/// let err = realize(&tree, RealizeOpts::sentence()).unwrap_err();
/// assert!(matches!(err, PhraseError::AmbiguousPreposition { .. }));
/// ```
pub fn realize(clause: &Clause, opts: RealizeOpts) -> Result<String, PhraseError> {
    realize_checked(clause, opts).map(|realized| realized.text)
}

fn imperative_slot(force: Force) -> usize {
    match force {
        Force::Imperative(Addressee::You) => 0,
        Force::Imperative(Addressee::We) => 1,
        Force::Imperative(Addressee::YouAll) => 2,
        _ => 0,
    }
}

/// Does this nominal fail to distinguish Nom from Acc on the surface?
fn nominal_nom_acc_syncretic(nominal: &Nominal, ctx: &mut Ctx) -> Result<bool, PhraseError> {
    Ok(match nominal {
        Nominal::Pron { .. } => false,
        _ => {
            let nom = render_nominal(nominal, Case::Nom, PronounStyle::Full, ctx)?.words;
            let acc = render_nominal(nominal, Case::Acc, PronounStyle::Full, ctx)?.words;
            nom == acc
        }
    })
}

fn slot_of(slot: SlotRef) -> Slot {
    match slot {
        SlotRef::Subject => Slot::Subject,
        SlotRef::Object => Slot::Object,
    }
}

/// Where the postverbal clitic cluster lands: immediately after the last
/// word of the (first) verb complex, or after `li` when the verb fronts
/// in a li-question.
fn postverbal_position(words: &[String], verb_words: &[String]) -> usize {
    if verb_words.is_empty() {
        return words.len();
    }
    // Find the verb complex as a contiguous run and return the index
    // after it (after `li` if li follows it directly).
    let end = verb_words.len();
    for start in 0..words.len().saturating_sub(end - 1) {
        if words[start..start + end] == *verb_words {
            let mut position = start + end;
            if words.get(position).map(String::as_str) == Some("li") {
                position += 1;
            }
            return position;
        }
    }
    words.len()
}

/// Second position: after the first constituent word (skipping the
/// fronted `či` particle).
fn second_position(words: &[String]) -> usize {
    let mut index = 0;
    if words.get(index).map(String::as_str) == Some("či") {
        index += 1;
    }
    let mut position = (index + 1).min(words.len());
    // li is first in the cluster (Franks & King order): when the force
    // rules already placed it at second position, the rest of the
    // cluster follows it.
    if words.get(position).map(String::as_str) == Some("li") {
        position += 1;
    }
    position
}

/// Join words, attaching comma tokens to the preceding word.
fn join_words(words: &[String]) -> String {
    let mut out = String::new();
    for word in words {
        if word.starts_with(ATTACH_COMMA) || word == "," {
            out.push(',');
        } else {
            if !out.is_empty() {
                out.push(' ');
            }
            out.push_str(word);
        }
    }
    out
}

/// Drop a trailing comma that collides with terminal punctuation.
fn fix_terminal_punctuation(text: String) -> String {
    text.replace(",.", ".")
        .replace(",?", "?")
        .replace(",!", "!")
}
