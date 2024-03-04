mod cli;
mod config;
mod core;

use cli::CLI;
use config::CONFIG;
use die_exit::*;
use scraper::{Html, Selector};

fn main() {
    let r = reqwest::blocking::get(format!("https://www.weblio.jp/content/{}", CLI.words[0]))
        .die("network err");
    let h = Html::parse_document(&r.text().die("parse err"));
    let s = Selector::parse(".kijiWrp").die("selector err");
    for e in h.select(&s) {
        println!("{}", e.text().collect::<Vec<_>>().join(" "));
    }
}
