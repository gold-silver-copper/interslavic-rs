//! Agreement/government resolution and linearization.
//!
//! Every inflected form comes from the `interslavic` facade — this module
//! never post-processes forms (the INTEGRATION.md contract). Linearization
//! defaults are steen's (syntax page): S–V–O is the "clearest and
//! stylistically most neutral" order; "modifiers usually precede the
//! noun"; the reflexive clitic follows the verb (steen's own examples:
//! "Ja myju se"); `li` sits right after the fronted focus, in phase 1
//! always the finite verb.

use crate::ast::*;
use interslavic::{
    Case, Gender, Number, Person, PronounStyle, Provenance, Tense, adj, cells, noun_with,
    perfect_parts, personal_pronoun, preposition_cases, preposition_senses, pronoun,
    quantified_parts_with_info, verb, verb_info,
};
use std::fmt;

/// Realization options, in the `english-phrase` style: `plain()` gives the
/// bare word sequence, `sentence()` capitalizes and punctuates.
#[derive(Debug, Clone, Copy, Default)]
pub struct RealizeOpts {
    pub sentence: bool,
    /// Treat a `Provenance::Guessed` NP head as an error instead of
    /// trusting the rule engine's gender/animacy guess.
    pub strict_guessed: bool,
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
}

/// Diagnostics are values; realization of a well-formed tree is total.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PhraseError {
    /// Not in the curated preposition table.
    UnknownPreposition(String),
    /// The preposition governs several cases and the tree picked none;
    /// the senses show the author what each case means.
    AmbiguousPreposition {
        preposition: String,
        senses: Vec<(Case, &'static str)>,
    },
    /// An explicit case the preposition does not govern.
    InvalidPrepositionCase {
        preposition: String,
        case: Case,
        allowed: Vec<Case>,
    },
    /// A direct object under a dictionary-intransitive verb.
    ObjectOfIntransitive { verb: String },
    /// Strict mode only: the NP head is not a dictionary noun and the
    /// engine would have to guess its gender/animacy.
    GuessedHead { lemma: String },
    /// A declared-but-unimplemented option (e.g. second-position
    /// clitics) — declared, not silently wrong.
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
            PhraseError::GuessedHead { lemma } => {
                write!(
                    f,
                    "`{lemma}` is not a dictionary noun (gender/animacy would be guessed)"
                )
            }
            PhraseError::Unsupported(what) => write!(f, "unsupported: {what}"),
        }
    }
}

impl std::error::Error for PhraseError {}

/// A rendered nominal plus the agreement features it projects.
struct Rendered {
    words: Vec<String>,
    person: Person,
    number: Number,
    gender: Gender,
}

/// One clean surface form: byform cells resolve to the first variant and
/// citation accents are stripped — both via `cells::variants`, the
/// documented normalization step (never ad-hoc string surgery).
fn surface(form: &str) -> String {
    cells::variants(form).into_iter().next().unwrap_or_default()
}

fn render_nominal(
    nominal: &Nominal,
    slot_case: Case,
    style: PronounStyle,
    opts: &RealizeOpts,
) -> Result<Rendered, PhraseError> {
    match nominal {
        Nominal::Pron {
            person,
            number,
            gender,
        } => {
            let form = personal_pronoun(*person, *number, *gender, slot_case, style)
                .or_else(|| {
                    // No such cell in the requested series (e.g. a clitic
                    // gap) — the full form always exists.
                    personal_pronoun(*person, *number, *gender, slot_case, PronounStyle::Full)
                })
                .expect("full personal-pronoun cells are total");
            Ok(Rendered {
                words: vec![form],
                person: *person,
                number: *number,
                gender: *gender,
            })
        }
        Nominal::Np(np) => render_np(np, slot_case, opts),
    }
}

fn render_np(
    np: &NounPhrase,
    slot_case: Case,
    opts: &RealizeOpts,
) -> Result<Rendered, PhraseError> {
    let slot_case = np.case_override.unwrap_or(slot_case);
    let info = interslavic::noun_info(&np.head);
    if opts.strict_guessed && info.provenance == Provenance::Guessed {
        return Err(PhraseError::GuessedHead {
            lemma: np.head.clone(),
        });
    }
    let (gender, animacy) = (info.gender, info.animacy);

    // A modifier is a determiner-class word first (toj/moj/ktory…, via the
    // pronoun paradigms) and an adjective otherwise.
    let modifier = |lemma: &str, case: Case, number: Number| -> String {
        pronoun(lemma, case, number, gender, animacy)
            .unwrap_or_else(|| adj(lemma, case, number, gender, animacy))
    };

    if let Some(n) = np.count {
        // Counted phrase: quantified_parts makes the government decision
        // once; every agreeing word declines with parts.case/parts.number.
        let parts =
            quantified_parts_with_info(n, &np.head, slot_case, gender, animacy, info.plural_only);
        let mut words = Vec::new();
        if let Some(det) = &np.determiner {
            words.push(surface(&modifier(det, parts.case, parts.number)));
        }
        words.push(n.to_string()); // digits, per the 0.12.0 decision
        for a in &np.adjectives {
            words.push(surface(&modifier(a, parts.case, parts.number)));
        }
        words.push(surface(&parts.noun));
        // Verb agreement with a counted subject: 1 agrees like the bare
        // noun; 2–4 are plural; the gen-pl quantities (0, 5+) take a
        // 3sg-NEUTER verb. Steen states the neuter-singular rule for
        // collective subjects; extending it to all gen-pl quantified
        // subjects is POLICY (the pan-Slavic pattern; steen is silent on
        // plain numeral subjects).
        let (number, agr_gender) = match n {
            1 => (Number::Singular, gender),
            2..=4 => (Number::Plural, gender),
            _ => (Number::Singular, Gender::Neuter),
        };
        return Ok(Rendered {
            words,
            person: Person::Third,
            number,
            gender: agr_gender,
        });
    }

    let number = if info.plural_only {
        Number::Plural
    } else {
        Number::Singular
    };
    let mut words = Vec::new();
    if let Some(det) = &np.determiner {
        words.push(surface(&modifier(det, slot_case, number)));
    }
    for a in &np.adjectives {
        words.push(surface(&modifier(a, slot_case, number)));
    }
    words.push(surface(&noun_with(
        &np.head, slot_case, number, gender, animacy,
    )));
    Ok(Rendered {
        words,
        person: Person::Third,
        number,
        gender,
    })
}

fn render_pp(pp: &PrepPhrase, opts: &RealizeOpts) -> Result<Vec<String>, PhraseError> {
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
    // Inside a PP, pronouns take the prepositional series automatically
    // (3rd-person n- forms: "od njego"; full forms elsewhere).
    let object = render_nominal(&pp.object, case, PronounStyle::AfterPreposition, opts)?;
    let mut words = vec![pp.preposition.clone()];
    words.extend(object.words);
    Ok(words)
}

/// The finite verb complex, in order: negation, auxiliary (past
/// 1st/2nd person only — `perfect_parts` drops it in the 3rd), the verb
/// form, the reflexive clitic.
fn render_verb(
    verb_lemma: &str,
    reflexive: bool,
    tense: TenseSpec,
    polarity: Polarity,
    person: Person,
    number: Number,
    gender: Gender,
) -> Vec<String> {
    let mut words = Vec::new();
    if polarity == Polarity::Negative {
        words.push("ne".to_string());
    }
    match tense {
        TenseSpec::Present => {
            words.push(surface(&verb(
                verb_lemma,
                person,
                number,
                gender,
                Tense::Present,
            )));
        }
        TenseSpec::Future => {
            // "bųdų pisati" comes back as one cell; keep it as words.
            let complex = surface(&verb(verb_lemma, person, number, gender, Tense::Future));
            words.extend(complex.split_whitespace().map(str::to_string));
        }
        TenseSpec::Past => {
            let parts = perfect_parts(verb_lemma, person, number, gender);
            if let Some(aux) = parts.auxiliary {
                words.push(aux);
            }
            words.push(parts.participle);
        }
    }
    if reflexive {
        // Postverbal per steen's examples ("Ja myju se"); second-position
        // placement is a declared, unimplemented option.
        words.push("sę".to_string());
    }
    words
}

/// Realize a clause. The one entry point: resolves agreement and
/// government (returning a [`PhraseError`] for ill-formed trees), then
/// linearizes with the steen-cited defaults.
///
/// ```
/// use interslavic_phrase::*;
/// use interslavic::Case;
///
/// // The flagship: count government decides the whole phrase once —
/// // determiner and adjective agree via quantified_parts.
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
    // The verb: a trailing " sę" on the lemma marks reflexivity, as does
    // the dictionary's own v.refl. metadata.
    let (verb_lemma, lemma_refl) = match clause.vp.verb.strip_suffix(" sę") {
        Some(bare) => (bare.to_string(), true),
        None => (clause.vp.verb.clone(), false),
    };
    let info = verb_info(&verb_lemma);
    let reflexive = lemma_refl || info.as_ref().is_some_and(|i| i.reflexive);

    // Valence: an object under a dictionary-intransitive verb is an error.
    if clause.vp.object.is_some() && info.as_ref().and_then(|i| i.transitive) == Some(false) {
        return Err(PhraseError::ObjectOfIntransitive { verb: verb_lemma });
    }

    let subject = render_nominal(&clause.subject, Case::Nom, PronounStyle::Full, &opts)?;
    let verb_words = render_verb(
        &verb_lemma,
        reflexive,
        clause.tense,
        clause.polarity,
        subject.person,
        subject.number,
        subject.gender,
    );
    let object_words = match &clause.vp.object {
        Some(object) => render_nominal(object, Case::Acc, PronounStyle::Full, &opts)?.words,
        None => Vec::new(),
    };
    let mut pp_words = Vec::new();
    for pp in &clause.vp.pps {
        pp_words.extend(render_pp(pp, &opts)?);
    }

    // Linearization. SVO (steen: neutral); `li` fronts the verb complex
    // and follows it (steen: "right after the focus point ... usually
    // the verb"); `či` fronts the whole clause.
    let mut words: Vec<String> = Vec::new();
    match clause.force {
        Force::LiQuestion => {
            words.extend(verb_words);
            words.push("li".to_string());
            if !clause.prodrop {
                words.extend(subject.words);
            }
            words.extend(object_words);
            words.extend(pp_words);
        }
        _ => {
            if clause.force == Force::CiQuestion {
                words.push("či".to_string());
            }
            if !clause.prodrop {
                words.extend(subject.words);
            }
            words.extend(verb_words);
            words.extend(object_words);
            words.extend(pp_words);
        }
    }

    let mut text = words.join(" ");
    if opts.sentence {
        let mut chars = text.chars();
        if let Some(first) = chars.next() {
            text = first.to_uppercase().collect::<String>() + chars.as_str();
        }
        text.push(match clause.force {
            Force::Declarative => '.',
            _ => '?',
        });
    }
    Ok(text)
}
