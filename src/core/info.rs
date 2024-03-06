use super::selector::Selector;
use super::word::Word;
use crate::cli::CLI;
use crate::config::CONFIG;
use anyhow::Result;
use futures::stream::FuturesOrdered;
use futures::StreamExt;
use scraper::Html;
use wana_kana::ConvertJapanese;

/// get a word's info from a selector
async fn get_word_info<'a, T>(selector: &Selector, word: T) -> Result<(Word, String)>
where
    T: Into<String>,
{
    let mut word: String = word.into();
    if CLI.japanese {
        word = word.to_kana();
    }
    let url = selector.url.replace("{}", word.as_str());
    let response = reqwest::get(&url).await?.text().await?;
    let document = Html::parse_document(&response);
    Ok((selector.select_one(&document, word.as_str()), url))
}

/// info words
pub async fn info<'a, T>(words: T)
where
    T: IntoIterator<Item = String>,
{
    let selector = CONFIG.get_selector_by_name(CLI.get_cli_language(), CLI.selector.clone());
    let mut queue = FuturesOrdered::new();
    for word in words.into_iter() {
        queue.push_back(get_word_info(selector, word));
    }
    while let Some(result) = queue.next().await {
        match result {
            Ok((word, url)) => {
                if word.print() && !CLI.no_url {
                    print!("more infomation: {}", url);
                }
                print!("{}", CONFIG.delimiter_between_words);
            }
            Err(e) => println!("{}", e),
        };
    }
}
