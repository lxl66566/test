use super::word::Word;
use super::wordselector::WordSelector;
use super::{selector as myselector, wordselector};
use crate::cli::CLI;
use crate::config::CONFIG;
use die_exit::*;
use scraper::{Html, Selector};

pub fn select_one(html: &Html, selector: &Word) -> Word {
    let wordselector = WordSelector::try_from(selector)
        .die_with(|err| format!("selector construction error: {}", err));
    // TODO: Word::new(html.select(&wordselector.it), metadata, definition, example)
    Word::default()
}

pub fn info_one(word: String) {
    let selector = CONFIG.get_selector_by_name(CLI.get_cli_language(), CLI.selector.clone());
}
