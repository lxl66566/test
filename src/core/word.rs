use colored::Colorize;
use serde::{Deserialize, Serialize};

use super::selector::RealSelectorString;
use crate::config::CONFIG;

/// the delimiter of metadata and other Word members.
pub type Delimiter = RealSelectorString;

impl Default for Delimiter {
    fn default() -> Self {
        Delimiter::new(" ", " ", "; ", "\n")
    }
}

/// A word and its explanations.
/// It can also be used in Color.
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
    /// preprocessing the given Strings, remove redundent space
    fn preprocessing(mut self) -> Word {
        let p = |s: String| {
            s.lines()
                .map(|line| line.trim())
                .filter(|line| !line.is_empty())
                .collect::<Vec<&str>>()
                .join("\n")
        };
        self.it = p(self.it);
        self.metadata = p(self.metadata);
        self.definition = p(self.definition);
        self.example = p(self.example);
        self
    }

    pub fn new<S1, S2, S3, S4>(it: S1, metadata: S2, definition: S3, example: S4) -> Self
    where
        S1: Into<String>,
        S2: Into<String>,
        S3: Into<String>,
        S4: Into<String>,
    {
        Word {
            it: it.into(),
            metadata: metadata.into(),
            definition: definition.into(),
            example: example.into(),
        }
        .preprocessing()
    }

    pub fn is_empty(&self) -> bool {
        self.definition.trim().is_empty()
    }

    pub fn print(&self) -> bool {
        if self.is_empty() {
            print!("Definition not found for word `{}`", self.it);
            return false;
        }
        println!("{}", self);
        true
    }
}

impl PartialEq for Word {
    fn eq(&self, other: &Self) -> bool {
        self.it == other.it
    }
}

impl Eq for Word {}

impl std::fmt::Display for Word {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let d = &CONFIG.delimiter_between_paragraphs;
        write!(
            f,
            "{}{}{}{}{}{}{}",
            self.it.as_str().color(CONFIG.color.it.as_ref()),
            d,
            self.metadata.as_str().color(CONFIG.color.metadata.as_ref()),
            d,
            self.definition
                .as_str()
                .color(CONFIG.color.definition.as_ref()),
            d,
            self.example
                .as_str()
                .trim_end()
                .color(CONFIG.color.example.as_ref())
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_print() {
        let w = Word::new("test", "123", "456", "789");
        let mut color = Word::new("red", "green", "blue", "yellow");
        w.print();
        assert!(!w.is_empty());
        color.definition = "  \n  \t ".into();
        assert!(color.is_empty());
    }
}
