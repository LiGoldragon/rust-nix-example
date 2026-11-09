//! A word: a non-empty run of letters.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Word(String);

#[derive(Debug, thiserror::Error, PartialEq, Eq)]
pub enum WordError {
    #[error("a word is never empty")]
    Empty,
    #[error("a word holds only letters, not {0:?}")]
    NotLetter(char),
}

impl Word {
    pub fn new(text: &str) -> Result<Self, WordError> {
        if text.is_empty() {
            return Err(WordError::Empty);
        }
        match text.chars().find(|character| !character.is_alphabetic()) {
            Some(character) => Err(WordError::NotLetter(character)),
            None => Ok(Self(text.to_owned())),
        }
    }

    pub fn text(&self) -> &str {
        &self.0
    }

    #[cfg(feature = "json")]
    pub fn to_json(&self) -> String {
        serde_json::to_string(self).expect("a word always serializes")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_text_is_no_word() {
        assert_eq!(Word::new(""), Err(WordError::Empty));
    }

    #[test]
    fn digit_is_no_letter() {
        assert_eq!(Word::new("a1"), Err(WordError::NotLetter('1')));
    }
}
