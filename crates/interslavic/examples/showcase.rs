//! # `interslavic-rs` — complete API showcase
//!
//! A single runnable tour of **every public API** in the workspace: the
//! dictionary-backed `interslavic` facade (`ISV`), the shared type vocabulary,
//! the `phono` morphophonemics module, and the dependency-free `interslavic-core`
//! rule engine (`ISVCore` + `ISVUTILS`).
//!
//! Run it with:
//!
//! ```text
//! cargo run --example showcase -p interslavic
//! ```
//!
//! Each line prints an input and the form the library produces, so the output
//! doubles as living documentation of what every entry point does.

// Everything comes from the `interslavic` facade: the dictionary-backed `ISV`,
// the shared vocabulary (Case, Number, Gender, …), the `phono` module, and the
// re-exported dependency-free engine (`ISVCore`, `ISVUTILS`) and its constants.
use interslavic::*;

fn main() {
    banner("interslavic-rs — complete API showcase");

    part(
        "1",
        "THE FACADE — `ISV`, the dictionary-backed single-form API",
    );
    facade_nouns();
    facade_adjectives();
    facade_degrees_of_comparison();
    facade_pronouns();
    facade_numerals();
    facade_finite_verbs();
    facade_verb_paradigms();

    part("2", "THE SHARED TYPES — every enum variant");
    shared_types();

    part(
        "3",
        "`phono` — morphophonemics (palatalization, iotation, softness)",
    );
    phono_module();
    orthography_module();
    prepositions_module();
    cells_module();
    derivation_module();

    part("4", "THE CORE ENGINE — `ISVCore`, dictionary-free rules");
    core_nouns();
    core_adjectives_and_degrees();
    core_pronouns_and_numerals();
    core_verbs();

    part(
        "5",
        "CORE UTILITIES — `ISVUTILS` and the exported constants",
    );
    core_utilities();
    misc_types_and_constants();

    println!("\nDone — every public entry point above was exercised.\n");
}

// ---------------------------------------------------------------------------
// 1. The facade: `ISV`
// ---------------------------------------------------------------------------

/// `ISV::noun` and `ISV::noun_with` — dictionary-backed noun declension.
fn facade_nouns() {
    heading("ISV::noun — declension with automatic gender/animacy lookup");
    // A hard feminine a-stem across all six cases, singular and plural.
    for number in [Number::Singular, Number::Plural] {
        let forms: Vec<String> = [
            Case::Nom,
            Case::Acc,
            Case::Gen,
            Case::Dat,
            Case::Loc,
            Case::Ins,
        ]
        .iter()
        .map(|&c| ISV::noun("žena", c, number))
        .collect();
        row(&format!("žena ({number:?})"), &forms.join(", "));
    }
    // Suppletive / irregular plurals come back as a single slash-joined string.
    row("oko (Nom pl)", &ISV::noun("oko", Case::Nom, Number::Plural));
    row(
        "mųž (Ins sg)",
        &ISV::noun("mųž", Case::Ins, Number::Singular),
    );

    heading("ISV::noun_with — caller-supplied gender/animacy override");
    // For an out-of-dictionary word the caller can pin the paradigm class; here
    // an invented -osť abstract noun is forced feminine (an i-stem).
    row(
        "zzzukavosť as f (Gen sg)",
        &ISV::noun_with(
            "zzzukavosť",
            Case::Gen,
            Number::Singular,
            NounGender::Feminine,
            Animacy::Inanimate,
        ),
    );
}

/// `ISV::adj` — adjective declension (gender + animacy columns).
fn facade_adjectives() {
    heading("ISV::adj — hard (dobry) vs soft (svěži) adjective");
    for word in ["dobry", "svěži"] {
        let m = ISV::adj(
            word,
            Case::Gen,
            Number::Singular,
            Gender::Masculine,
            Animacy::Animate,
        );
        let f = ISV::adj(
            word,
            Case::Gen,
            Number::Singular,
            Gender::Feminine,
            Animacy::Inanimate,
        );
        let n = ISV::adj(
            word,
            Case::Nom,
            Number::Singular,
            Gender::Neuter,
            Animacy::Inanimate,
        );
        row(word, &format!("gen.sg.m={m}, gen.sg.f={f}, nom.sg.n={n}"));
    }
    // Animacy only affects the masculine accusative singular and nom/acc plural.
    let anim = ISV::adj(
        "dobry",
        Case::Acc,
        Number::Singular,
        Gender::Masculine,
        Animacy::Animate,
    );
    let inan = ISV::adj(
        "dobry",
        Case::Acc,
        Number::Singular,
        Gender::Masculine,
        Animacy::Inanimate,
    );
    row(
        "dobry acc.sg.m",
        &format!("animate={anim}, inanimate={inan}"),
    );
}

/// `ISV::comparative` and `ISV::superlative`.
fn facade_degrees_of_comparison() {
    heading("ISV::comparative / ISV::superlative — (adjective, adverb) pairs");
    for adj in ["novy", "dobry", "vysoky", "uzky", "svěži"] {
        match ISV::comparative(adj) {
            Some((comp, adv)) => row(adj, &format!("komp. {comp} / {adv}")),
            None => row(adj, "does not gradate synthetically"),
        }
    }
    if let Some((sup, adv)) = ISV::superlative("novy") {
        row("novy (superlative)", &format!("{sup} / {adv}"));
    }
    // Relational adjectives use the analytic comparative instead (None here).
    row("russky", &format!("{:?}", ISV::comparative("russky")));
}

/// `ISV::pronoun` — closed-class pronoun declension.
fn facade_pronouns() {
    heading("ISV::pronoun — toj / moj / kto / veś / ktory");
    let cases = [
        Case::Nom,
        Case::Gen,
        Case::Dat,
        Case::Acc,
        Case::Loc,
        Case::Ins,
    ];
    for lemma in ["toj", "moj", "veś", "ktory"] {
        let forms: Vec<String> = cases
            .iter()
            .map(|&c| {
                ISV::pronoun(
                    lemma,
                    c,
                    Number::Singular,
                    Gender::Masculine,
                    Animacy::Inanimate,
                )
                .unwrap_or_else(|| "—".into())
            })
            .collect();
        row(&format!("{lemma} (m.sg)"), &forms.join(", "));
    }
    // kto is animate: its accusative equals the genitive; -koli inflects inside.
    row(
        "kto (Gen) / ktokoli (Gen)",
        &format!(
            "{} / {}",
            ISV::pronoun(
                "kto",
                Case::Gen,
                Number::Singular,
                Gender::Masculine,
                Animacy::Animate
            )
            .unwrap(),
            ISV::pronoun(
                "ktokoli",
                Case::Gen,
                Number::Singular,
                Gender::Masculine,
                Animacy::Animate
            )
            .unwrap(),
        ),
    );
    // A non-pronoun returns None.
    row(
        "stol (not a pronoun)",
        &format!(
            "{:?}",
            ISV::pronoun(
                "stol",
                Case::Gen,
                Number::Singular,
                Gender::Masculine,
                Animacy::Inanimate
            )
        ),
    );
}

/// `ISV::numeral` — cardinals, ordinals and the dual-remnant numbers.
fn facade_numerals() {
    heading("ISV::numeral — jedin / dva / pęť / pŕvy");
    let n = |l: &str, c: Case, g: Gender| {
        ISV::numeral(l, c, Number::Plural, g, Animacy::Inanimate).unwrap_or_else(|| "—".into())
    };
    row(
        "jedin",
        &format!(
            "nom.m={}, gen.m={}, nom.f={}",
            ISV::numeral(
                "jedin",
                Case::Nom,
                Number::Singular,
                Gender::Masculine,
                Animacy::Inanimate
            )
            .unwrap(),
            ISV::numeral(
                "jedin",
                Case::Gen,
                Number::Singular,
                Gender::Masculine,
                Animacy::Inanimate
            )
            .unwrap(),
            ISV::numeral(
                "jedin",
                Case::Nom,
                Number::Singular,
                Gender::Feminine,
                Animacy::Inanimate
            )
            .unwrap(),
        ),
    );
    row(
        "dva",
        &format!(
            "nom.m={}, nom.f={}, gen={}, ins={}",
            n("dva", Case::Nom, Gender::Masculine),
            n("dva", Case::Nom, Gender::Feminine),
            n("dva", Case::Gen, Gender::Masculine),
            n("dva", Case::Ins, Gender::Masculine),
        ),
    );
    row(
        "tri / pęť (Gen)",
        &format!(
            "{} / {}",
            n("tri", Case::Gen, Gender::Masculine),
            n("pęť", Case::Gen, Gender::Masculine),
        ),
    );
    row(
        "pŕvy (ordinal, Gen sg m)",
        &ISV::numeral(
            "pŕvy",
            Case::Gen,
            Number::Singular,
            Gender::Masculine,
            Animacy::Inanimate,
        )
        .unwrap(),
    );
}

/// `ISV::verb`, `ISV::try_verb`, `ISV::verb_with_present_hint`.
fn facade_finite_verbs() {
    heading("ISV::verb — one finite form per (person, number, gender, tense)");
    let tenses = [
        Tense::Present,
        Tense::Imperfect,
        Tense::Future,
        Tense::Perfect,
        Tense::PluPerfect,
        Tense::Conditional,
    ];
    for tense in tenses {
        row(
            &format!("pisati (1sg, {tense:?})"),
            &ISV::verb(
                "pisati",
                Person::First,
                Number::Singular,
                Gender::Masculine,
                tense,
            ),
        );
    }
    // Gender is load-bearing for the compound past.
    row(
        "pisati (perfect, 3sg)",
        &format!(
            "m={}, f={}",
            ISV::verb(
                "pisati",
                Person::Third,
                Number::Singular,
                Gender::Masculine,
                Tense::Perfect
            ),
            ISV::verb(
                "pisati",
                Person::Third,
                Number::Singular,
                Gender::Feminine,
                Tense::Perfect
            ),
        ),
    );

    heading("ISV::try_verb — None for non-verbs (no catch_unwind needed)");
    row(
        "pisati",
        &format!(
            "{:?}",
            ISV::try_verb(
                "pisati",
                Person::First,
                Number::Singular,
                Gender::Masculine,
                Tense::Present
            )
        ),
    );
    row(
        "xyz",
        &format!(
            "{:?}",
            ISV::try_verb(
                "xyz",
                Person::First,
                Number::Singular,
                Gender::Masculine,
                Tense::Present
            )
        ),
    );

    heading("ISV::verb_with_present_hint — disambiguate a typed dictionary row");
    row(
        "bolěti + hint (boli), 3sg present",
        &ISV::verb_with_present_hint(
            "bolěti",
            "(boli)",
            Person::Third,
            Number::Singular,
            Gender::Masculine,
            Tense::Present,
        ),
    );
}

/// `ISV::verb_forms`, `ISV::try_verb_forms`, `ISV::verb_forms_with_metadata`.
fn facade_verb_paradigms() {
    heading("ISV::noun_forms / adj_forms — full paradigm structs (like verb_forms)");
    let np = ISV::noun_forms("žena");
    row(
        &format!("noun_forms(\"žena\") [{:?}]", np.gender),
        &format!("sg={:?}", np.singular),
    );
    let ap = ISV::adj_forms("dobry");
    row(
        "adj_forms(\"dobry\").feminine[sg]",
        &format!("{:?}", ap.feminine[0]),
    );
    row(
        "  .get(Gen, Sg, Masc, Anim)",
        ap.get(
            Case::Gen,
            Number::Singular,
            Gender::Masculine,
            Animacy::Animate,
        ),
    );

    heading("ISV::verb_forms — the whole paradigm as a VerbParadigm struct");
    print_paradigm("učiti", &ISV::verb_forms("učiti"));

    heading("ISV::try_verb_forms — checked variant");
    row(
        "učiti",
        &format!("is_some = {}", ISV::try_verb_forms("učiti").is_some()),
    );
    row("xyz", &format!("{:?}", ISV::try_verb_forms("xyz")));

    heading("ISV::verb_forms_with_metadata — explicit aspect/transitivity");
    // A perfective, intransitive reading suppresses the present-active and
    // present-passive participles (prap/prpp become None).
    let p = ISV::verb_forms_with_metadata("napisati", "", false, false);
    row(
        "napisati (perfective, intransitive)",
        &format!(
            "infinitive={}, prap={:?}, prpp={:?}, pfap={}",
            p.infinitive, p.prap, p.prpp, p.pfap
        ),
    );
}

// ---------------------------------------------------------------------------
// 2. Shared types
// ---------------------------------------------------------------------------

/// Every variant of every re-exported enum.
fn shared_types() {
    heading("Enums (all variants)");
    row("Number", "Singular, Plural");
    row("Case", "Nom, Acc, Gen, Loc, Dat, Ins");
    row("Gender", "Masculine, Feminine, Neuter");
    row(
        "NounGender",
        "Masculine, Feminine, Neuter (noun-facing gender)",
    );
    row("Animacy", "Animate, Inanimate");
    row("Person", "First, Second, Third");
    row(
        "Tense",
        "Present, Imperfect, Future, Perfect, PluPerfect, Conditional",
    );
    row(
        "Conjugation",
        "First, Second (returned by get_present_tense_stem)",
    );
    // Enums derive Debug/Copy/Eq, so they compose freely.
    let all_cases = [
        Case::Nom,
        Case::Acc,
        Case::Gen,
        Case::Loc,
        Case::Dat,
        Case::Ins,
    ];
    row("iterating Case", &format!("{all_cases:?}"));
}

// ---------------------------------------------------------------------------
// 3. phono
// ---------------------------------------------------------------------------

/// The `phono` morphophonemics module (re-exported as `interslavic::phono`).
fn phono_module() {
    heading("Correspondence tables (phono::PALATALIZATION / phono::IOTATION)");
    row("PALATALIZATION", &format!("{:?}", phono::PALATALIZATION));
    row("IOTATION", &format!("{:?}", phono::IOTATION));

    heading("phono::is_soft — stem softness (drives -ejši vs -ějši, O⇒E)");
    for stem in ["nov", "svěž", "końń", "rad"] {
        row(stem, &format!("{}", phono::is_soft(stem)));
    }

    heading("phono::palatalize_final / phono::iotate_final — apply an alternation");
    for stem in ["ruk", "bog", "muh", "krat"] {
        row(
            stem,
            &format!(
                "palatalize={}, iotate={}",
                phono::palatalize_final(stem),
                phono::iotate_final(stem),
            ),
        );
    }

    heading("phono::inverse_palatalization / phono::inverse_iotation — candidate sources");
    row(
        "inverse_palatalization(ruč)",
        &format!("{:?}", phono::inverse_palatalization("ruč")),
    );
    row(
        "inverse_iotation(voš)",
        &format!("{:?}", phono::inverse_iotation("voš")),
    );
}

/// The `orthography` module — flavored → standard alphabet folding.
fn orthography_module() {
    heading("orthography::to_standard — fold the flavored alphabet to standard");
    for word in ["dělajųt", "pomoćnȯgo", "međa", "Ěľo", "čas"] {
        row(word, &orthography::to_standard(word));
    }

    heading("orthography::FOLD_PAIRS — the correspondence table (for mirrors)");
    row("FOLD_PAIRS", &format!("{:?}", orthography::FOLD_PAIRS));
}

/// The `prepositions` module — preposition government (which cases each takes).
fn prepositions_module() {
    heading("ISV::preposition_cases / prepositions::preposition_cases");
    for prep in ["bez", "k", "s", "na", "pod", "za", "žaba"] {
        row(prep, &format!("{:?}", ISV::preposition_cases(prep)));
    }
    heading("prepositions::PREPOSITIONS — the full curated table");
    row(
        "count",
        &format!(
            "{} single-word prepositions",
            prepositions::PREPOSITIONS.len()
        ),
    );
}

/// The `cells` module — normalize a raw paradigm cell into clean form variants.
fn cells_module() {
    heading("cells::variants — flatten a raw cell's conventions into plain forms");
    // Real participle cells straight out of verb_forms carry the conventions.
    let p = ISV::verb_forms("dělati");
    let pfpp = p.pfpp.clone().unwrap_or_default();
    row(
        &format!("pfpp raw: {pfpp}"),
        &format!("{:?}", cells::variants(&pfpp)),
    );
    for raw in ["generoval(a)", "dělajemy (-a, -o)", "dělaĵųći"] {
        row(raw, &format!("{:?}", cells::variants(raw)));
    }
}

/// The `derivation` module — a lemma's regular word family.
fn derivation_module() {
    heading("ISV::derive / derivation::derive — the regular derivational family");
    for (base, pos) in [
        ("dobry", derivation::Pos::Adjective),
        ("učiti", derivation::Pos::Verb),
        ("kniga", derivation::Pos::Noun),
    ] {
        let fam: Vec<String> = ISV::derive(base, pos)
            .iter()
            .map(|d| format!("{}({})", d.form, d.pattern))
            .collect();
        row(&format!("{base} ({pos:?})"), &fam.join(", "));
    }
}

// ---------------------------------------------------------------------------
// 4. Core engine: ISVCore
// ---------------------------------------------------------------------------

/// Core noun declension (no dictionary): guessed and explicit.
fn core_nouns() {
    heading("ISVCore::decline_noun — rule-engine declension, gender guessed");
    row(
        "selo (Gen sg)",
        &ISVCore::decline_noun("selo", &Case::Gen, &Number::Singular),
    );

    heading("ISVCore::decline_noun_explicit — every metadata knob");
    // plural_only pluralia tantum, with an explicit oblique-stem addition.
    row(
        "dveri (Gen pl, f, plural_only)",
        &ISVCore::decline_noun_explicit(
            "dveri",
            &Case::Gen,
            &Number::Plural,
            NounGender::Feminine,
            Animacy::Inanimate,
            /* plural_only  */ true,
            /* singular_only*/ false,
            /* indeclinable */ false,
            /* addition     */ None,
        ),
    );
    // An indeclinable loanword echoes its citation form in every cell.
    row(
        "taksi (Ins sg, indeclinable)",
        &ISVCore::decline_noun_explicit(
            "taksi",
            &Case::Ins,
            &Number::Singular,
            NounGender::Neuter,
            Animacy::Inanimate,
            false,
            false,
            true,
            None,
        ),
    );

    heading("ISVCore::decline_noun_simple — the common explicit case");
    row(
        "kosť (Gen sg, f)",
        &ISVCore::decline_noun_simple(
            "kosť",
            &Case::Gen,
            &Number::Singular,
            NounGender::Feminine,
            Animacy::Inanimate,
        ),
    );

    heading("ISVCore noun helpers");
    row(
        "guess_gender(\"žena\")",
        &format!("{:?}", ISVCore::guess_gender("žena")),
    );
    row(
        "noun_is_animate(\"mųž\")",
        &format!("{}", ISVCore::noun_is_animate("mųž")),
    );
    row(
        "is_ost_class(\"radosť\")",
        &format!("{}", ISVCore::is_ost_class("radosť")),
    );
    row(
        "get_noun_stem(\"žena\", sg)",
        &ISVCore::get_noun_stem("žena", &Number::Singular),
    );
    row(
        "stem_of_noun_is_soft(\"konj\")",
        &format!("{}", ISVCore::stem_of_noun_is_soft("konj")),
    );
    row(
        "last_n_chars(\"pisati\", 2)",
        &ISVCore::last_n_chars("pisati", 2),
    );
}

/// Core adjective declension, stem helpers and degrees of comparison.
fn core_adjectives_and_degrees() {
    heading("ISVCore::decline_adj + adjective stem helpers");
    row(
        "decline_adj(\"dobry\", Gen sg m anim)",
        &ISVCore::decline_adj(
            "dobry",
            &Case::Gen,
            &Number::Singular,
            &Gender::Masculine,
            Animacy::Animate,
        ),
    );
    row("get_adj_stem(\"dobry\")", &ISVCore::get_adj_stem("dobry"));
    row(
        "stem_of_adj_is_soft(\"svěži\")",
        &format!("{}", ISVCore::stem_of_adj_is_soft("svěži")),
    );

    heading("ISVCore::comparative / ISVCore::superlative (the facade delegates here)");
    row(
        "comparative(\"vysoky\")",
        &format!("{:?}", ISVCore::comparative("vysoky")),
    );
    row(
        "superlative(\"dobry\")",
        &format!("{:?}", ISVCore::superlative("dobry")),
    );
}

/// Core pronoun and numeral declension (the Option-returning primitives).
fn core_pronouns_and_numerals() {
    heading("ISVCore::decline_pronoun / ISVCore::decline_numeral");
    row(
        "decline_pronoun(\"moj\", Dat sg n)",
        &format!(
            "{:?}",
            ISVCore::decline_pronoun(
                "moj",
                &Case::Dat,
                &Number::Singular,
                &Gender::Neuter,
                Animacy::Inanimate
            )
        ),
    );
    row(
        "decline_numeral(\"pęť\", Ins pl)",
        &format!(
            "{:?}",
            ISVCore::decline_numeral(
                "pęť",
                &Case::Ins,
                &Number::Plural,
                &Gender::Masculine,
                Animacy::Inanimate
            )
        ),
    );
}

/// Core verb conjugation — total and checked, single forms and full paradigms.
fn core_verbs() {
    heading("ISVCore::conjugate_verb family");
    row(
        "conjugate_verb(\"dělati\", 3pl present)",
        &ISVCore::conjugate_verb(
            "dělati",
            &Person::Third,
            &Number::Plural,
            &Gender::Masculine,
            &Tense::Present,
        ),
    );
    row(
        "conjugate_verb_with_present_hint(\"bolěti\", \"(boli)\", 3sg)",
        &ISVCore::conjugate_verb_with_present_hint(
            "bolěti",
            "(boli)",
            &Person::Third,
            &Number::Singular,
            &Gender::Masculine,
            &Tense::Present,
        ),
    );
    row(
        "conjugate_verb_with_options(\"napisati\", perfective/transitive, perfect 3sg f)",
        &ISVCore::conjugate_verb_with_options(
            "napisati",
            "",
            &Person::Third,
            &Number::Singular,
            &Gender::Feminine,
            &Tense::Perfect,
            true,
            false,
        ),
    );
    row(
        "conjugate_verb_checked(\"xyz\")",
        &format!(
            "{:?}",
            ISVCore::conjugate_verb_checked(
                "xyz",
                "",
                &Person::First,
                &Number::Singular,
                &Gender::Masculine,
                &Tense::Present,
                true,
                true
            )
        ),
    );

    heading("ISVCore::verb_paradigm_with_options / verb_paradigm_checked");
    let paradigm = ISVCore::verb_paradigm_with_options("pisati", "", true, true);
    row(
        "verb_paradigm_with_options(\"pisati\").present",
        &format!("{:?}", paradigm.present),
    );
    row(
        "verb_paradigm_checked(\"pisati\").is_some()",
        &format!(
            "{}",
            ISVCore::verb_paradigm_checked("pisati", "", true, true).is_some()
        ),
    );
    row(
        "verb_paradigm_checked(\"voda\")",
        &format!(
            "{:?}",
            ISVCore::verb_paradigm_checked("voda", "", true, true).map(|_| "…")
        ),
    );

    heading("ISVCore verb stem helpers");
    let (stem, conj): (String, Conjugation) = ISVCore::get_present_tense_stem("pisati");
    row(
        "get_present_tense_stem(\"pisati\")",
        &format!("stem={stem}, conjugation={conj:?}"),
    );
    row(
        "get_infinitive_stem(\"pisati\")",
        &ISVCore::get_infinitive_stem("pisati"),
    );
    row(
        "l_participle(\"pisati\", f, sg)",
        &ISVCore::l_participle("pisati", &Gender::Feminine, &Number::Singular),
    );
}

// ---------------------------------------------------------------------------
// 5. Core utilities + constants
// ---------------------------------------------------------------------------

/// `ISVUTILS` — the low-level string/phonology helpers.
fn core_utilities() {
    heading("ISVUTILS::iotation_merge — fuse a stem with a j-initial suffix");
    row(
        "iotation_merge(\"nos\", \"jenьje\")",
        &ISVUTILS::iotation_merge("nos", "jenьje"),
    );
    row(
        "iotation_merge(\"hod\", \"ju\")",
        &ISVUTILS::iotation_merge("hod", "ju"),
    );

    heading("ISVUTILS character predicates");
    for c in ['a', 'k', 'č', 'j'] {
        row(
            &format!("'{c}'"),
            &format!(
                "vowel={}, consonant={}, hard={}, soft={}",
                ISVUTILS::is_vowel(&c),
                ISVUTILS::is_consonant(&c),
                ISVUTILS::is_hard_consonant(&c),
                ISVUTILS::is_soft_consonant(&c),
            ),
        );
    }
    row(
        "ends_with_soft_consonant(\"konj\")",
        &format!("{}", ISVUTILS::ends_with_soft_consonant("konj")),
    );
    row(
        "last_in_stringslice(\"voda\")",
        &format!("'{}'", ISVUTILS::last_in_stringslice("voda")),
    );

    heading("ISVUTILS string helpers");
    row(
        "string_without_last_n(\"pisati\", 2)",
        &ISVUTILS::string_without_last_n("pisati", 2),
    );
    row(
        "replace_last_occurence(\"a-b-c\", \"-\", \"/\")",
        &ISVUTILS::replace_last_occurence("a-b-c", "-", "/"),
    );
}

/// The exported constants and the remaining public types.
fn misc_types_and_constants() {
    heading("Exported constants");
    row("VOWELS", &format!("{VOWELS:?}"));
    row("HARD_CONSONANTS", &format!("{HARD_CONSONANTS:?}"));
    row("J_MERGE_CHARS", &format!("{J_MERGE_CHARS:?}"));

    heading("ComplexNoun (head noun + qualifying adjectives)");
    let default = ComplexNoun::default();
    row("ComplexNoun::default()", &format!("{default:?}"));
    let phrase = ComplexNoun {
        head_noun: "grad".into(),
        adjective: vec!["stary".into(), "veliky".into()],
    };
    row("built ComplexNoun", &format!("{phrase:?}"));
}

// ---------------------------------------------------------------------------
// Small printing helpers (no library APIs below this line).
// ---------------------------------------------------------------------------

fn banner(title: &str) {
    let bar = "═".repeat(title.chars().count() + 4);
    println!("\n╔{bar}╗");
    println!("║  {title}  ║");
    println!("╚{bar}╝");
}

fn part(number: &str, title: &str) {
    println!("\n\n■■■ PART {number}: {title} ■■■");
}

fn heading(title: &str) {
    println!("\n── {title}");
}

fn row(label: &str, value: &str) {
    println!("   {label:<44} → {value}");
}

fn print_paradigm(lemma: &str, p: &VerbParadigm) {
    println!("   {lemma}:");
    println!("     infinitive : {}", p.infinitive);
    println!("     present    : {:?}", p.present);
    println!("     imperfect  : {:?}", p.imperfect);
    println!("     future     : {:?}", p.future);
    println!("     perfect    : {:?}", p.perfect);
    println!("     pluperfect : {:?}", p.pluperfect);
    println!("     conditional: {:?}", p.conditional);
    println!("     imperative : {:?}", p.imperative);
    println!("     prap (pres.act.part.) : {:?}", p.prap);
    println!("     prpp (pres.pas.part.) : {:?}", p.prpp);
    println!("     pfap (past.act.part.) : {}", p.pfap);
    println!("     pfpp (past.pas.part.) : {:?}", p.pfpp);
    println!("     gerund (verbal noun)  : {}", p.gerund);
}
