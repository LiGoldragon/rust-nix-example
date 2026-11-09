fn main() -> anyhow::Result<()> {
    let text = std::env::args().skip(1).collect::<Vec<_>>().join(" ");
    println!("{}", example_sentence::Sentence::parse(&text)?.describe());
    Ok(())
}
