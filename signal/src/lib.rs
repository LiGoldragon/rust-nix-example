//! A message carrying one word.

use example_word::Word;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Message {
    pub word: Word,
}

impl Message {
    pub fn new(word: Word) -> Self {
        Self { word }
    }

    #[cfg(feature = "json")]
    pub fn to_json(&self) -> String {
        format!("{{\"word\":{}}}", self.word.to_json())
    }
}
