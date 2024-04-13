use interslavic::WordCore;

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
}
