use interslavic::{Gender, Number, Person, VerbTense, WordCore};

use std::fmt;

#[derive(Debug)]
pub enum MonsterType {
    Žuk,
    Dragon(i64),
    Unicorn,
    // Add other variants...
}

impl fmt::Display for MonsterType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Using `format!("{:?}", self)` to get the Debug representation of the enum variant
        let debug_str = format!("{:?}", self);

        // Find the position of the opening parenthesis (if any) and truncate the string there
        let end_pos = debug_str.find('(').unwrap_or(debug_str.len());

        // Extract the substring from the beginning up to the opening parenthesis
        let trimmed_str = &debug_str[..end_pos];

        // Convert the trimmed string to lowercase
        let lowercase_str = trimmed_str.to_lowercase();

        // Write the lowercase string to the formatter
        write!(f, "{}", lowercase_str)
    }
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
