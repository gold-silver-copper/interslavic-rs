# interslavic-core

Dependency-free Interslavic morphology rules.

`interslavic-core` is the pure rule engine underneath the table-backed [`interslavic`](https://crates.io/crates/interslavic) crate. It contains no generated dictionary tables and has no third-party dependencies.

## When to use it

Use this crate when you want a tiny morphology engine for known lemmas or controlled inputs. For general application code, prefer `interslavic`, which consults generated dictionary metadata first and falls back to this rule engine.

## Usage

```rust
use interslavic_core::*;

assert_eq!(
    noun::decline_noun("suma", Case::Acc, Number::Singular),
    "sumų"
);

assert_eq!(
    adjective::decline_adj(
        "dobry",
        Case::Gen,
        Number::Singular,
        Gender::Masculine,
        Animacy::Animate,
    ),
    "dobrogo"
);

assert_eq!(
    verb::conjugate_verb(
        "učiti",
        Person::First,
        Number::Singular,
        Gender::Feminine,
        Tense::Present,
    ),
    "učų"
);
```

## Scope

- Encodes productive noun, adjective, and verb morphology.
- Provides core enums and `VerbParadigm` shared by the public facade.
- Does not read dictionary TSV data.
- Does not contain PHF lookup tables or other generated metadata.

## License

Dual licensed under MIT and Apache-2.0.
