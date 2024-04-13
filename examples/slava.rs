use interslavic::{Gender, Number, Person, VerbTense, WordCore};
use my_display_macro::SimpleDisplay;

use std::fmt;

#[derive(Debug, SimpleDisplay)]
pub enum MonsterType {
    Žuk,
    Dragon(i64),
    Unicorn,
    // Add other variants...
}

fn main() {
    let boop = WordCore::new();

    let word = "pęť";

    let count = word.chars().count();
    println!("char count is {}", count);

    let mut chars = word.chars();

    while let Some(letter) = chars.next() {
        println!("letter  is {}", letter);
    }

    if word.ends_with("ť") {
        println!("wooooo");
    }

    let asd = boop.get_noun("kråva");
    match asd {
        Some(cn) => println!("{:#?}", &cn.ins_pl()),
        _ => println!("OH NO"),
    }

    let ccc = VerbTense::Perfect(Gender::Feminine, Person::Third, Number::Sing);
    let asd = boop.get_verb("udariti");
    match asd {
        Some(cn) => println!("{:#?}", &cn.derive_verb(&ccc)),
        _ => println!("OH NO"),
    };

    let zuk = MonsterType::Žuk;
    let dragon = MonsterType::Dragon(32);
    let unicorn = MonsterType::Unicorn;

    println!("{}", zuk); // Prints "žuk"
    println!("{}", dragon); // Prints "dragon"
    println!("{}", unicorn); // Prints "unicorn"
    println!("Žuk");
}
