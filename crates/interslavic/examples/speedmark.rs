use interslavic::*;
use std::hint::black_box;
use std::time::Instant;

fn main() {
    benchmark_noun();
    benchmark_adjective();
    benchmark_verb();
    benchmark_paradigm();
}

fn benchmark_noun() {
    let words = ["adept", "oko", "zzzzzzzzzzzzzzzzz"];
    run_benchmark("noun", &words, |word| {
        ISV::noun(word, Case::Gen, Number::Plural)
    });
}

fn benchmark_adjective() {
    let words = ["dobry", "sinji", "zzzzzzzzzzzzzzzzz"];
    run_benchmark("adjective", &words, |word| {
        ISV::adj(
            word,
            Case::Gen,
            Number::Singular,
            Gender::Masculine,
            Animacy::Animate,
        )
    });
}

fn benchmark_verb() {
    let words = ["pisati", "učiti", "zzzzzzzzzzzzzzzzzti"];
    run_benchmark("verb", &words, |word| {
        ISV::verb(
            word,
            Person::Third,
            Number::Singular,
            Gender::Masculine,
            Tense::Present,
        )
    });
}

fn benchmark_paradigm() {
    let words = ["pisati", "učiti"];
    run_benchmark("verb_paradigm", &words, |word| ISV::verb_forms(word).gerund);
}

fn run_benchmark<F>(label: &str, words: &[&str], mut f: F)
where
    F: FnMut(&str) -> String,
{
    let iterations = 50_000;
    let total_calls = iterations * words.len();
    let start = Instant::now();
    let mut last_result = String::new();

    for _ in 0..iterations {
        for &word in words {
            last_result = black_box(f(black_box(word)));
        }
    }

    let duration = start.elapsed();
    let nanos = duration.as_nanos() as f64;
    let calls_per_sec = total_calls as f64 / (nanos / 1e9);
    let nanos_per_call = nanos / total_calls as f64;

    println!("[{label}] last result: {last_result}");
    println!("[{label}] completed in {duration:?} → {total_calls} calls");
    println!(
        "[{label}] throughput: {calls_per_sec:.2} calls/sec | time per call: {nanos_per_call:.2} ns"
    );
}
