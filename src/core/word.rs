use colored::Colorize;
use serde::{Deserialize, Serialize};

/// A word and its explanations.
/// The Strings of it can be also used in Selector and Color.
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Word {
    /// word itself.
    pub it: String,
    /// phonetic notation, hiragana and katagana in Japanese, or other things.
    pub metadata: String,
    pub definition: String,
    pub example: String,
}

impl Word {
    pub fn all(s: &str) -> Self {
        Self {
            it: s.to_string(),
            metadata: s.to_string(),
            definition: s.to_string(),
            example: s.to_string(),
        }
    }
    pub fn print(&self, color: &Word) {
        println!("{}", self.it.as_str().color(color.it.as_ref()));
        println!("{}", self.metadata.as_str().color(color.metadata.as_ref()));
        println!(
            "{}",
            self.definition.as_str().color(color.definition.as_ref())
        );
        println!("{}", self.example.as_str().color(color.example.as_ref()));
    }
}

impl PartialEq for Word {
    fn eq(&self, other: &Self) -> bool {
        self.it == other.it
    }
}

impl Eq for Word {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_print() {
        let w = Word {
            it: "test".to_string(),
            metadata: "123".to_string(),
            definition: "456".to_string(),
            example: "789".to_string(),
        };
        let color = Word {
            it: "red".to_string(),
            metadata: "green".to_string(),
            definition: "blue".to_string(),
            example: "yellow".to_string(),
        };
        w.print(&color);
    }
}
