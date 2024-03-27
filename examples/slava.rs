use interslavic::*;

fn main() {
    load_word_csv();

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
}
