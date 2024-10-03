# Inflector for the Interslavic language

[![Crates.io](https://img.shields.io/crates/v/interslavic.svg)](https://crates.io/crates/interslavic)
[![Documentation](https://docs.rs/interslavic/badge.svg)](https://docs.rs/interslavic/latest/)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/bevyengine/bevy/blob/master/LICENSE)
[![Downloads](https://img.shields.io/crates/d/interslavic.svg)](https://crates.io/crates/interslavic)


This crate provides a decliner/conjugator/inflector for Interslavic.

It uses data from the official dictionary at  https://interslavic-dictionary.com/


Sample usage:

```rust
use interslavic::*;
fn main() {
    let mut inflector = ISV::default();
    //if you do not initialize the dictionary, animate nouns will not be inflected correctly, nor will words with irregular stems
    inflector.initialize_dictionary("isv_words.csv");

    let noun = inflector.decline_noun("mųž", &Case::Gen, &Number::Singular);
    //the output is a tuple of a string and gender
    println!("{:#?}", noun.0);
    //output: mųža

    let adj = inflector.decline_adj(
        "samy",
        &Case::Gen,
        &Number::Singular,
        &Gender::Masculine,
        true,
    );
       //the output is just a string
    println!("{:#?}", adj);
    //output: samogo


    let verbik = "učiti";


    let verb = inflector.conjugate_verb(
            verbik,
            &Person::First,
            &Number::Singular,
            &Gender::Feminine,
            &Tense::Present,
    );
    println!("{:#?}", verb);
    //output: učų



    let lik = inflector.l_participle("buditi", &Gender::Feminine, &Number::Singular);
    println!("{:#?}", lik);
    //output: budila

    let guessed_noun = inflector.decline_noun("sluga", &Case::Ins, &Number::Singular);
    println!("{:#?}", guessed_noun.0);
}

```
