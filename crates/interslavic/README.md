# interslavic

Dictionary-backed Interslavic inflection for Rust.

This crate is the ergonomic public facade for the workspace. It combines generated dictionary metadata with the dependency-free rules from `interslavic-core`:

- noun declension with dictionary gender, animacy, number restrictions, indeclinability, and fleeting-vowel metadata;
- adjective agreement by case, number, gender, and animacy;
- finite verb forms for present, imperfect, future, perfect, pluperfect, and conditional;
- full verb paradigms including imperative, participles, and gerund forms;
- explicit helpers for dictionary present-stem hints.

## Usage

```rust
use interslavic::*;

assert_eq!(interslavic::noun("adept", Case::Acc, Number::Singular), "adepta");
assert_eq!(interslavic::noun("oko", Case::Nom, Number::Plural), "oči / očesa");

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
```

## Dictionary metadata

The crate does not parse TSV files at runtime. Metadata is generated offline by `interslavic-extractor`, committed under `generated/`, and included at compile time.

Useful workspace commands:

```bash
cargo xtask refresh-data
cargo xtask check-registry
cargo xtask accuracy
```

## When to use `interslavic-core`

Use `interslavic-core` directly only when you need the smallest dependency-free morphology layer and can live without dictionary-backed metadata. Application code usually wants this `interslavic` crate.

## License

Dual licensed under MIT and Apache-2.0.
