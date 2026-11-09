//! A count of words.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Count(pub usize);

impl Count {
    pub fn plural(self) -> bool {
        self.0 != 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_is_singular() {
        assert!(!Count(1).plural());
    }
}
