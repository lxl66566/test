use anyhow::Result;
// https://crates.io/crates/scraper
use html5ever::tree_builder::TreeSink;
use scraper::{Html, Selector};
pub fn process(content: String) -> Result<String> {
    let document = Html::parse_document(&content);
    let exclude = Selector::parse(".pr.phrase-block.dphrase-block").unwrap();
    let node_ids: Vec<_> = document.select(&exclude).map(|x| x.id()).collect();
    for id in node_ids {
        document.remove_from_parent(&id);
    }
}
