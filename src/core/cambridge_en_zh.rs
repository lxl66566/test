// https://crates.io/crates/scraper

// use super::*;
use html5ever::tree_builder::TreeSink;
use scraper::{Html, Selector};
trait Process {
    fn process(content: String) -> anyhow::Result<String>;
    fn url(&self) -> String;
}
impl Process for Item {
    fn process(content: String) -> anyhow::Result<String> {
        let mut document = Html::parse_document(&content);
        let exclude = Selector::parse(".pr.phrase-block.dphrase-block").unwrap();
        let node_ids: Vec<_> = document.select(&exclude).map(|x| x.id()).collect();
        for id in node_ids {
            document.remove_from_parent(&id);
        }
        Ok("".to_string())
    }
    fn url(&self) -> String {
        format!(
            "https://dictionary.cambridge.org/dictionary/english-chinese-simplified/{}",
            self.word
        )
    }
}
