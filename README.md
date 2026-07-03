# Interslavic morphology for Rust

[![Crates.io](https://img.shields.io/crates/v/interslavic.svg)](https://crates.io/crates/interslavic)
[![Documentation](https://docs.rs/interslavic/badge.svg)](https://docs.rs/interslavic/latest/)

`interslavic` provides fast noun declension, adjective declension, and verb conjugation for Interslavic.

The morphology engine is now a Rust-oriented port of the noun, adjective, and verb rules used by [`@interslavic/utils`](https://www.npmjs.com/package/@interslavic/utils) in [`sonic16x/interslavic`](https://github.com/sonic16x/interslavic), including dictionary `addition` metadata for irregular stems and common alternative forms.

The crate is structured like [`gold-silver-copper/english`](https://github.com/gold-silver-copper/english):

- `crates/interslavic-core` — Rust port of the core `@interslavic/utils` morphology rules, no bundled dictionary data.
- `crates/interslavic` — public API plus generated dictionary metadata and rule fallback.
- `crates/extractor` — parses official Interslavic dictionary TSV data and generates Rust tables, including additions/part-of-speech metadata.
- `crates/xtask` — reproducible developer command for refreshing generated data.

Dictionary metadata is generated ahead of time from the official Interslavic dictionary data used by [`sonic16x/interslavic`](https://github.com/sonic16x/interslavic), especially the Google Sheets TSV export:

```text
https://docs.google.com/spreadsheets/d/1N79e_yVHDo-d026HljueuKJlAAdeELAiPzdFzdBuKbY/export?format=tsv&gid=1987833874
```

This crate does **not** use Wiktionary data and does **not** require loading `isv_words.csv` at runtime.

## Usage

```rust
use interslavic::*;

fn main() {
    assert_eq!(
        Interslavic::noun("mųž", Case::Gen, Number::Singular),
        "mųža"
    );

    assert_eq!(
        Interslavic::adjective(
            "samy",
            Case::Gen,
            Number::Singular,
            Gender::Masculine,
            Animacy::Animate,
        ),
        "samogo"
    );

    assert_eq!(
        Interslavic::verb("učiti", Person::First, Number::Singular, Tense::Present),
        "učų"
    );

    assert_eq!(
        Interslavic::l_participle("buditi", &Gender::Feminine, &Number::Singular),
        "budila"
    );
}
```

Older names such as `ISV::decline_noun`, `ISV::decline_adj`, and `ISV::conjugate_verb` remain available for compatibility. `initialize_dictionary` is now a no-op because generated data is bundled at compile time.

## Refreshing dictionary data

Regenerate bundled data from the official TSV export:

```bash
cargo xtask refresh-data --with-checks
```

Use a local TSV instead:

```bash
cargo xtask refresh-data --input data/raw/dictionary.tsv --with-checks
```

Generated Rust tables are written to:

```text
crates/interslavic/src/generated/dictionary.rs
```

Intermediate artifacts are written to:

```text
data/intermediate/
```

## Development

```bash
cargo test
cargo run -p interslavic --example test
```
