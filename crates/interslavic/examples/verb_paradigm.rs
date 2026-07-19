fn main() {
    let forms = interslavic::verb_forms("pisati");

    println!("infinitive: {}", forms.infinitive);
    println!("present: {}", forms.present.join(", "));
    println!("future: {}", forms.future.join(", "));
    println!("perfect: {}", forms.perfect.join(", "));
    println!("imperative: {}", forms.imperative.join(", "));
    println!(
        "present active participle: {}",
        forms.prap.as_deref().unwrap_or("—")
    );
    println!(
        "past passive participle: {}",
        forms.pfpp.as_deref().unwrap_or("—")
    );
    println!("gerund: {}", forms.gerund);

    assert_eq!(forms.future[0], "bųdų pisatì");
    assert_eq!(forms.perfect[0], "jesm pisal(a)");
    assert_eq!(forms.imperative, vec!["piši", "pišimo", "pišite"]);
    assert_eq!(forms.gerund, "pisańje");
}
