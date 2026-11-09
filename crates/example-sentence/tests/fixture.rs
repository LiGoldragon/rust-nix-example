//! Reads a fixture beside the crate, at run time, by the manifest directory.

use example_sentence::Sentence;

#[test]
fn fixture_sentence_parses() {
    let path = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/fixtures/sentence.txt");
    let text = std::fs::read_to_string(path).unwrap();
    assert_eq!(Sentence::parse(&text).unwrap().count().0, 3);
}
