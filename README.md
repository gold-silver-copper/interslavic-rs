# Inflector for the Interslavic language

[![Crates.io](https://img.shields.io/crates/v/interslavic.svg)](https://crates.io/crates/interslavic)
[![Documentation](https://docs.rs/interslavic/badge.svg)](https://docs.rs/interslavic/latest/)
![License](https://img.shields.io/crates/l/interslavic)
[![Downloads](https://img.shields.io/crates/d/interslavic.svg)](https://crates.io/crates/interslavic)

**interslavic** is a fast Rust inflection library for Interslavic. It combines a dependency-free morphology core with generated dictionary metadata, making the common API ergonomic while keeping runtime data loading out of the crate.

The project is organized as four crates:

- `interslavic-core`: dependency-free morphology rules.
- `interslavic`: public API with embedded generated dictionary metadata.
- `interslavic-extractor`: offline metadata generator.
- `xtask`: workspace commands for refreshing, checking, benchmarking, and parity testing.

Dictionary source sheet: <https://docs.google.com/spreadsheets/d/1N79e_yVHDo-d026HljueuKJlAAdeELAiPzdFzdBuKbY/edit?gid=1987833874#gid=1987833874>

Integrating this crate into a text pipeline? Read **[INTEGRATION.md](INTEGRATION.md)**
— citation-form conventions, the byform contract, clean vs raw paradigms,
pronoun styles, counting, and a worked runtime-assembly example.

## Installation

```bash
cargo add interslavic
```

## Example

```rust
use interslavic::*;

fn main() {
    // One dictionary-backed noun form.
    assert_eq!(interslavic::noun("adept", Case::Acc, Number::Singular), "adepta");

    // Alternatives are returned as one slash-separated string.
    assert_eq!(interslavic::noun("oko", Case::Nom, Number::Plural), "oči / očesa");

    // Ambiguous or context-sensitive nouns can be overridden directly.
    assert_eq!(
        interslavic::noun_with(
            "luč",
            Case::Gen,
            Number::Singular,
            Gender::Feminine,
            Animacy::Inanimate,
        ),
        "luči"
    );
    assert_eq!(
        interslavic::noun_with(
            "mųž",
            Case::Acc,
            Number::Singular,
            Gender::Masculine,
            Animacy::Animate,
        ),
        "mųža"
    );

    // Adjectives and verbs are single-form APIs. Adjective phrases with
    // particles/complements should be modeled by the caller, not declined as a unit.
    assert_eq!(
        interslavic::adj(
            "dobry",
            Case::Gen,
            Number::Singular,
            Gender::Masculine,
            Animacy::Animate,
        ),
        "dobrogo"
    );
    assert_eq!(
        interslavic::verb(
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
    let pisati = interslavic::verb_forms("pisati");
    assert_eq!(pisati.future[0], "bųdų pisatì");
    assert_eq!(pisati.perfect[0], "jesm pisal(a)");
    assert_eq!(pisati.imperative, vec!["piši", "pišimo", "pišite"]);
    assert_eq!(pisati.gerund, "pisańje");

    // Personal pronouns carry all three form series: full, clitic, and the
    // prepositional n- forms of the third person.
    use PronounStyle::*;
    assert_eq!(
        interslavic::personal_pronoun(Person::Second, Number::Singular, Gender::Masculine, Case::Gen, Full),
        Some("tebe".to_string())
    );
    assert_eq!(
        interslavic::personal_pronoun(Person::Third, Number::Singular, Gender::Masculine, Case::Gen, AfterPreposition),
        Some("njego".to_string())
    );
    assert_eq!(interslavic::reflexive_pronoun(Case::Acc, Clitic), Some("sę".to_string()));

    // The l-participle and the declined participles are exposed directly.
    assert_eq!(interslavic::l_participle("idti", Gender::Feminine, Number::Singular), "šla");
    assert_eq!(
        interslavic::passive_participle("osvětliti", Case::Nom, Number::Singular, Gender::Feminine, Animacy::Inanimate),
        Some("osvětljena".to_string())
    );

    // Dictionary integrations that need a specific present-stem hint can use the
    // explicit typed helper instead of passing a phrase string.
    assert_eq!(
        interslavic::verb_with_present_hint(
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

More runnable examples:

```bash
cargo run -p interslavic --example basic
cargo run -p interslavic --example verb_paradigm
cargo run -p interslavic --example sentence
cargo run -p interslavic --example speedmark --release
```

## Crate overview

### `interslavic`

The public facade for application code. It looks up generated dictionary metadata first and falls back to `interslavic-core` rules for unknown words. Use this crate when you want dictionary-aware noun metadata, verb present-stem hints, transitivity/imperfectivity flags, and crate-root free functions.

### `interslavic-core`

The dependency-free rule engine. It does not know about the dictionary TSV or generated PHF tables. Use it directly only when a tiny dependency-free morphology layer is more important than dictionary-backed metadata.

### `interslavic-extractor`

Offline generator for committed Rust metadata under `crates/interslavic/generated`. It parses the dictionary TSV and emits deterministic PHF tables. Runtime crates do not parse TSV data.

### `xtask`

Developer workflow wrapper for repeatable commands.

## Data generation

Generated Rust metadata is committed under `crates/interslavic/generated` and included at compile time. Refresh and check it with:

```bash
cargo xtask refresh-data
cargo xtask check-registry
```

`check-registry` is intentionally dump-free: it verifies that committed generated metadata is deterministic for the committed TSV. It does not contact the upstream dictionary sheet.

## sonic16x parity and accuracy

This crate is tested against [`sonic16x/interslavic`](https://github.com/sonic16x/interslavic), the JavaScript implementation used by the Interslavic dictionary. The comparison script fetches the latest upstream `master` branch, generates reference forms with `@interslavic/utils`, compares the Rust output, and writes reports under `target/infl-comparison`.

```bash
cargo xtask accuracy
# equivalent to: node tools/compare-latest-sonic.js
```

Latest measured compatible accuracy against sonic16x commit `0fab0c5b4463118d46b1cdcd506926d8848052c9` / `@interslavic/utils@3.4.0`:

| Scope | Paradigms | Forms | Compatible accuracy | Mismatches |
|---|---:|---:|---:|---:|
| nouns | 8,851 | 99,060 | 99.9828% | 17 |
| adjectives | 3,261 | 156,528 | 100.0000% | 0 |
| verbs: present, imperfect, perfect, pluperfect, future, conditional, imperative, participles, gerund | 4,345 | 216,339 | 100.0000% | 0 |
| personal/reflexive pronouns: full, clitic, and prepositional n- series | 4 | 198 | 100.0000% | 0 |
| out-of-vocabulary verbs (fixed sample: -jati, -nųti, -ovati, prefixed -sti) | 12 | 612 | 100.0000% | 0 |
| numerals (dictionary rows: cardinal, ordinal, collective, fractional, …) | 129 | 7,308 | 100.0000% | 0 |
| total core comparable | 16,602 | 480,045 | 99.9965% | 17 |

The pronoun scope compares every cell of the personal and reflexive
paradigms (`personal_pronoun`/`reflexive_pronoun`) against
`declensionPronoun`, including the nonexistence of cells (unattested
clitics, the reflexive nominative). The out-of-vocabulary scope tracks the
hintless rule-engine path on a fixed sample of plausible non-dictionary
lemmas (membership re-checked against the fetched dictionary each run), so
OOV behavior is parity-verified too — e.g. the JS reference does **not**
contract OOV `-jati` presents (`stajati` → `stajaje, staja`, never
`staje`), and this crate follows it.

When cross-checking forms against the slovowiki form API, read a hit's
`lemmas`/analyses fields before crediting it to a lemma: keys are folded
surfaces, so a homograph hit can belong to an unrelated word (`staje` is
the noun `staja`'s gen.sg/nom.pl/acc.pl, not a verb form). The noun count moved from 8 to 17
mismatches between measurements because the live dictionary sheet edited a
handful of rows (soft-o loans like *adadžo*, substantivized adjectives);
the change reproduces identically on earlier releases, i.e. it is upstream
data drift, not an engine change.

“Compatible” means the Rust form matched one of sonic16x's accepted alternatives when sonic returns comma/slash/parenthetical variants. Phrase strings from dictionary rows are reported separately because this crate's core APIs accept typed lemmas/metadata rather than arbitrary phrases.

## Developer commands

```bash
cargo xtask check-all       # fmt check, workspace tests, generated metadata check
cargo xtask examples        # compile/run the lightweight examples
cargo xtask speed           # run the speedmark example in release mode
cargo xtask accuracy        # run sonic16x parity comparison
cargo xtask refresh-data    # regenerate committed dictionary metadata
cargo xtask check-registry  # verify generated metadata is current
```

## API philosophy and limitations

- Public functions return a single `String`; accepted alternatives are represented as slash-separated strings when the morphology engine has multiple compatible forms.
- `interslavic::noun_with` is the escape hatch for explicit gender/animacy when a lemma is ambiguous or context-sensitive.
- Verb phrase strings are not parsed as arbitrary syntax. Use typed helpers such as `verb_with_present_hint` or `verb_forms_with_metadata` for dictionary rows with extra metadata.
- `interslavic-core` stays dependency-free; dictionary-backed behavior belongs in `interslavic`.

## License

Dual licensed under MIT and Apache-2.0.
