use super::selector::RealSelectorString;
use colored::Colorize;
use serde::{Deserialize, Serialize};

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
    }

    pub fn is_empty(&self) -> bool {
        self.definition.trim().is_empty()
    }

    pub fn print(&self, color: &Word) {
        if self.is_empty() {
            println!("Definition not found for word `{}`", self.it);
            return;
        }
        println!(
            "{}\n{}\n{}\n{}",
            self.it.as_str().color(color.it.as_ref()),
            self.metadata.as_str().color(color.metadata.as_ref()),
            self.definition.as_str().color(color.definition.as_ref()),
            self.example.as_str().color(color.example.as_ref())
        )
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
        let w = Word::new("test", "123", "456", "789");
        let mut color = Word::new("red", "green", "blue", "yellow");
        w.print(&color);
        assert!(!w.is_empty());
        color.definition = "  \n  \t ".into();
        assert!(color.is_empty());
    }
}
