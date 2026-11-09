use example_word::Word;

#[test]
fn word_round_trips_through_json() {
    let word = Word::new("river").unwrap();
    let text = serde_json::to_string(&word).unwrap();
    assert_eq!(text, "\"river\"");
    assert_eq!(serde_json::from_str::<Word>(&text).unwrap(), word);
}
