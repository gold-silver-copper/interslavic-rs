use interslavic::*;

// Issue #10: the verb APIs must be total (never panic). Previously
// ISV::verb / ISV::verb_forms panicked ("IMPROPER PRESENT TENSE STEM") on any
// word whose infinitive stem could not be derived; ISV::verb_with_present_hint
// could panic on a char boundary when the prefix contained a multibyte char.

#[test]
fn verb_apis_do_not_panic_on_non_verbs() {
    // Reaching the end of the loop without unwinding IS the assertion.
    for w in ["", "  ", "xyz", "voda", "123", "ž", "hello world", "a", "ť"] {
        let _ = ISV::verb(
            w,
            Person::First,
            Number::Singular,
            Gender::Masculine,
            Tense::Present,
        );
        let _ = ISV::verb_forms(w);
    }
}

#[test]
fn verb_with_present_hint_does_not_panic() {
    // The hint path used to byte-slice at prefix.len(); a multibyte prefix
    // occurring mid-hint could land off a char boundary and panic.
    for (w, hint) in [
        ("prědati", "aprě"),
        ("razbiti", "raz"),
        ("iti", "prě jde"),
        ("byti", "ěěě"),
        ("dělati", "sę dělaje"),
    ] {
        let _ = ISV::verb_with_present_hint(
            w,
            hint,
            Person::Third,
            Number::Singular,
            Gender::Masculine,
            Tense::Present,
        );
    }
}

#[test]
fn try_variants_distinguish_verbs_from_garbage() {
    assert!(ISV::try_verb_forms("pisati").is_some());
    assert!(ISV::try_verb_forms("učiti").is_some());
    assert!(ISV::try_verb_forms("xyz").is_none());
    assert!(ISV::try_verb_forms("").is_none());
    assert!(ISV::try_verb_forms("voda").is_none());

    assert!(
        ISV::try_verb(
            "pisati",
            Person::First,
            Number::Singular,
            Gender::Masculine,
            Tense::Present
        )
        .is_some()
    );
    assert!(
        ISV::try_verb(
            "xyz",
            Person::First,
            Number::Singular,
            Gender::Masculine,
            Tense::Present
        )
        .is_none()
    );
}

#[test]
fn checked_and_total_agree_for_real_verbs() {
    // The checked path is the same algorithm; for a real verb it must produce
    // exactly the total paradigm.
    for w in ["pisati", "učiti", "dělati", "byti"] {
        assert_eq!(ISV::try_verb_forms(w), Some(ISV::verb_forms(w)), "{w}");
    }
}
