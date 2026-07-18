use interslavic::*;

fn comp(adj: &str) -> Option<(String, String)> {
    ISV::comparative(adj)
}

#[test]
fn comparatives_follow_steen() {
    // Regular hard stem.
    assert_eq!(comp("novy"), Some(("novějši".into(), "nověje".into())));
    // Lexical irregulars.
    assert_eq!(comp("blagy"), Some(("unši".into(), "unje".into())));
    assert_eq!(comp("dobry"), Some(("lěpši".into(), "lěpje".into())));
    // -ky class: -ši on the truncated root, adverb by iotation.
    assert_eq!(comp("kratky"), Some(("kratši".into(), "kraće".into())));
    assert_eq!(comp("vysoky"), Some(("vysši".into(), "vyše".into())));
    assert_eq!(comp("glųboky"), Some(("glųbši".into(), "glųbje".into()))); // labial → j
    assert_eq!(comp("krěhky"), Some(("krěhši".into(), "krěše".into()))); // h → š
    assert_eq!(comp("žestoky"), Some(("žestši".into(), "žešće".into()))); // st → šć
    // Soft đ-final stem takes -ejši, not hard -ějši.
    assert_eq!(comp("ryđi"), Some(("ryđejši".into(), "ryđeje".into())));
    // Short-root -ky truncators (2-char root) still truncate, not palatalize:
    // the -ky is the suffix here, so drop it (uz-, niz-) rather than uzč-.
    assert_eq!(comp("uzky"), Some(("uzši".into(), "uže".into())));
    assert_eq!(comp("nizky"), Some(("nizši".into(), "niže".into())));
    // But diky's k is root-final, so it palatalizes regularly (not *diši).
    assert_eq!(comp("diky"), Some(("dičejši".into(), "dičeje".into())));
}

#[test]
fn non_gradable_adjectives_return_none() {
    for a in [
        "russky",    // relational -sky
        "gręcky",    // relational -cky
        "bogatši",   // already a comparative
        "bųdųći",    // participial -ći
        "boljši",    // already a comparative
        "božji",     // soft -ji possessive
        "poslědnji", // soft -nji
    ] {
        assert_eq!(comp(a), None, "{a} must not gradate synthetically");
    }
}

#[test]
fn superlative_prefixes_naj() {
    assert_eq!(
        ISV::superlative("novy"),
        Some(("najnovějši".into(), "najnověje".into()))
    );
    assert_eq!(
        ISV::superlative("dobry"),
        Some(("najlěpši".into(), "najlěpje".into()))
    );
    assert_eq!(ISV::superlative("russky"), None);
}
