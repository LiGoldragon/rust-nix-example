//! A sentence: words joined by spaces.

use anyhow::Context;
use example_number::Count;
use example_signal::Message;
use example_word::Word;

pub struct Sentence {
    words: Vec<Word>,
}

impl Sentence {
    pub fn parse(text: &str) -> anyhow::Result<Self> {
        let words = text
            .split_whitespace()
            .map(|piece| Word::new(piece).with_context(|| format!("in {piece:?}")))
            .collect::<anyhow::Result<Vec<_>>>()?;
        Ok(Self { words })
    }

    pub fn count(&self) -> Count {
        Count(self.words.len())
    }

    pub fn messages(&self) -> Vec<Message> {
        self.words.iter().cloned().map(Message::new).collect()
    }

    pub fn describe(&self) -> String {
        let count = self.count();
        let noun = if count.plural() { "words" } else { "word" };
        let json: Vec<String> = self.messages().iter().map(Message::to_json).collect();
        format!("{} {noun}: [{}]", count.0, json.join(","))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn two_words_are_counted() {
        assert_eq!(Sentence::parse("quiet river").unwrap().count(), Count(2));
    }
}
