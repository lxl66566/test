use super::*;
use html5ever::tree_builder::TreeSink;
use scraper::{Html, Selector};
impl Process for Item {
    fn process(content: String) -> anyhow::Result<String> {
        Ok("".to_string())
    }
    fn url(&self) -> String {
        format!(
            "https://dictionary.cambridge.org/dictionary/english-chinese-simplified/{}",
            self.word
        )
    }
}
