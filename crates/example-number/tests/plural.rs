use example_number::Count;

#[test]
fn zero_is_plural() {
    assert!(Count(0).plural());
}
