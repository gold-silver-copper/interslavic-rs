# Inflector for the Interslavic language

[![Crates.io](https://img.shields.io/crates/v/interslavic.svg)](https://crates.io/crates/interslavic)
[![Documentation](https://docs.rs/interslavic/badge.svg)](https://docs.rs/interslavic/latest/)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/bevyengine/bevy/blob/master/LICENSE)
[![Downloads](https://img.shields.io/crates/d/interslavic.svg)](https://crates.io/crates/interslavic)

Interslavic inflection in four crates:

- `interslavic-core`: dependency-free morphology rules.
- `interslavic`: small public API with generated dictionary metadata.
- `interslavic-extractor`: offline metadata generator.
- `xtask`: workspace commands for refreshing/checking generated data.

Dictionary source sheet: https://docs.google.com/spreadsheets/d/1N79e_yVHDo-d026HljueuKJlAAdeELAiPzdFzdBuKbY/edit?gid=1987833874#gid=1987833874

## Example

```rust
use interslavic::*;

fn main() {
    // One dictionary-backed noun form.
    assert_eq!(ISV::noun("adept", Case::Acc, Number::Singular), "adepta");

    // Alternatives are returned as one slash-separated string.
    assert_eq!(ISV::noun("oko", Case::Nom, Number::Plural), "oči / očesa");

    // Ambiguous or context-sensitive nouns can be overridden directly.
    assert_eq!(
        ISV::noun_with(
            "luč",
            Case::Gen,
            Number::Singular,
            NounGender::Feminine,
            Animacy::Inanimate,
        ),
        "luči"
    );
    assert_eq!(
        ISV::noun_with(
            "mųž",
            Case::Acc,
            Number::Singular,
            NounGender::Masculine,
            Animacy::Animate,
        ),
        "mųža"
    );

    // Adjectives and verbs are also single-form APIs. Adjective phrases with
    // particles/complements should be modeled by the caller, not declined as a unit.
    assert_eq!(
        ISV::adj(
            "dobry",
            Case::Gen,
            Number::Singular,
            Gender::Masculine,
            Animacy::Animate,
        ),
        "dobrogo"
    );
    assert_eq!(
        ISV::verb(
            "učiti",
            Person::First,
            Number::Singular,
            Gender::Feminine,
            Tense::Present,
        ),
        "učų"
    );

    // Full verb paradigms include simple, compound, imperative, participial,
    // and gerund forms.
    let pisati = ISV::verb_forms("pisati");
    assert_eq!(pisati.future[0], "bųdų pisatì");
    assert_eq!(pisati.perfect[0], "jesm pisal(a)");
    assert_eq!(pisati.imperative, vec!["piši", "pišimo", "pišite"]);
    assert_eq!(pisati.gerund, "pisańje");

    // Dictionary integrations that need a specific present-stem hint can use the
    // explicit typed helper instead of passing a phrase string.
    assert_eq!(
        ISV::verb_with_present_hint(
            "bolěti",
            "(boli)",
            Person::First,
            Number::Singular,
            Gender::Masculine,
            Tense::Present,
        ),
        "boljų"
    );
}
```

## Data generation

The runtime crate does not parse dictionary TSV data. Generated Rust metadata is
committed under `crates/interslavic/generated` and included at compile time.

Refresh and check it with:

```bash
cargo xtask refresh-data
cargo xtask check-registry
```

## sonic16x parity and accuracy

This crate is tested against [`sonic16x/interslavic`](https://github.com/sonic16x/interslavic), the JavaScript implementation used by the Interslavic dictionary. The comparison script fetches the latest upstream `master` branch, generates reference forms with `@interslavic/utils`, compares the Rust output, and writes reports under `target/infl-comparison`.

```bash
node tools/compare-latest-sonic.js
```

Latest measured compatible accuracy against sonic16x commit `0fab0c5b4463118d46b1cdcd506926d8848052c9` / `@interslavic/utils@3.4.0`:

| Scope | Paradigms | Forms | Compatible accuracy | Mismatches |
|---|---:|---:|---:|---:|
| nouns | 8,851 | 99,060 | 99.9919% | 8 |
| adjectives | 3,261 | 156,528 | 100.0000% | 0 |
| verbs: present, imperfect, perfect, pluperfect, future, conditional, imperative, participles, gerund | 4,345 | 216,339 | 100.0000% | 0 |
| total core comparable | 16,457 | 471,927 | 99.9983% | 8 |

“Compatible” means the Rust form matched one of sonic16x's accepted alternatives when sonic returns comma/slash/parenthetical variants. Phrase strings from dictionary rows are reported separately because this crate's core APIs accept typed lemmas/metadata rather than arbitrary phrases; the latest run skipped 1,488 phrase rows and 8 Sonic-null/reference rows.

## License

Dual licensed under MIT and Apache-2.0.
