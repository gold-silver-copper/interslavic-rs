//! Print the whole-dictionary paradigm dump (`lemma\tcell\tform`) to
//! stdout — the stream the fingerprint test hashes. Run via
//! `cargo xtask dump-paradigms`, which captures it to
//! `target/paradigm-fingerprint/`.

fn main() {
    let tsv = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/data/dictionary_metadata.tsv"
    ));
    print!("{}", interslavic::fingerprint::dump_string(tsv));
}
