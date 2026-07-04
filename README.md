# Inflector for the Interslavic language

[![Crates.io](https://img.shields.io/crates/v/interslavic.svg)](https://crates.io/crates/interslavic)
[![Documentation](https://docs.rs/interslavic/badge.svg)](https://docs.rs/interslavic/latest/)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/bevyengine/bevy/blob/master/LICENSE)
[![Downloads](https://img.shields.io/crates/d/interslavic.svg)](https://crates.io/crates/interslavic)

Interslavic inflection in two crates:

- `interslavic-core`: dependency-free morphology rules.
- `interslavic`: dictionary-backed wrapper with embedded metadata generated from the official dictionary at https://interslavic-dictionary.com/.

Dictionary source sheet: https://docs.google.com/spreadsheets/d/1N79e_yVHDo-d026HljueuKJlAAdeELAiPzdFzdBuKbY/edit?gid=1987833874#gid=1987833874

## Example

```rust
use interslavic::*;

fn main() -> Result<(), InflectionError> {
    // One form, dictionary-backed.
    println!("{}", ISV::noun_form("adept", Case::Acc, Number::Singular)?.text());
    // adepta

    // Full paradigm by dictionary id.
    let člen = ISV::noun_id("25028")?;
    println!("{}", člen.get(Case::Acc, Number::Singular).unwrap().text());
    // člen

    // m./f. nouns require explicit gender.
    let luč = ISV::noun_id_as("339", NounGender::Feminine)?;
    println!("{}", luč.get(Case::Gen, Number::Singular).unwrap().text());
    // luči

    // All dictionary paradigms for a lemma, including homonyms and m./f. variants.
    for paradigm in ISV::noun("luč")? {
        println!("{:?}: {}", paradigm.gender, paradigm.get(Case::Gen, Number::Singular).unwrap().text());
    }

    // Pure explicit morphology, no dictionary lookup.
    let mųž = ISV::noun_as(NounDeclensionRequest {
        lemma: "mųž",
        gender: NounGender::Masculine,
        animacy: Animacy::Animate,
        number_restriction: NumberRestriction::Countable,
        indeclinable: false,
        addition: None,
        gender_override: None,
    })?;
    println!("{}", mųž.get(Case::Gen, Number::Singular).unwrap().text());
    // mųža

    // Adjectives and verbs.
    println!("{}", ISV::adj("osnovany na", Case::Gen, Number::Singular, Gender::Masculine, Animacy::Animate));
    // osnovanogo na
    println!("{}", ISV::verb("učiti", Person::First, Number::Singular, Gender::Feminine, Tense::Present));
    // učų

    Ok(())
}
```
