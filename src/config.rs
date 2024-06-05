use std::{collections::HashMap, fs};

use anyhow::{Error, Result};
use colored::Colorize;
use die_exit::*;
use home::home_dir;
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};

use crate::{
    cli::Format,
    core::{
        selector::{RealSelectorString, Selector},
        word::{Delimiter, Word},
    },
};

lazy_static! {
    pub static ref CONFIG: Config = Config::load();
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Config {
    pub version: String,
    pub default_language: String,
    pub color: Word,
    pub delimiter_between_words: String,
    pub delimiter_between_paragraphs: String,
    pub en: Vec<Selector>,
    pub jp: Vec<Selector>,
    pub other_languages: HashMap<String, Vec<Selector>>,
}

impl Config {
    /// get wordinfo's config file path
    pub fn config_path() -> std::path::PathBuf {
        let mut path = home_dir().die("Failed to get home directory");
        path.push(".config");
        path.push("wordinfo");
        path
    }

    /// get config, otherwise use default value
    pub fn load() -> Config {
        fn raise() -> Result<Config> {
            if let Ok(content) = fs::read_to_string(Config::config_path().with_extension("json")) {
                return serde_json::from_str(&content).map_err(Error::from);
            }
            if let Ok(content) = fs::read_to_string(Config::config_path().with_extension("toml")) {
                return toml::from_str(&content).map_err(Error::from);
            }
            if let Ok(content) = fs::read_to_string(Config::config_path().with_extension("yaml")) {
                return serde_yaml::from_str(&content).map_err(Error::from);
            }
            Ok(Config::default())
        }
        raise().unwrap_or_else(|_| {
            println!(
                "{}",
                "WARNING: Failed to parse config file, using default config.".yellow()
            );
            Config::default()
        })
    }

    /// save config to config_path
    pub fn save(&self, format: &Format) -> Result<()> {
        let config_path = Config::config_path();
        fs::create_dir_all(
            config_path
                .parent()
                .expect("can not find parent dir in config."),
        )
        .die("Failed to create `.config` directory");
        match format {
            Format::Json => {
                let json_string =
                    serde_json::to_string_pretty(self).expect("Failed to serialize config.");
                fs::write(config_path.with_extension("json"), json_string)?;
            }
            Format::Toml => {
                let toml_string =
                    toml::to_string_pretty(self).expect("Failed to serialize config.");
                fs::write(config_path.with_extension("toml"), toml_string)?;
            }
            Format::Yaml => {
                let yaml_string = serde_yaml::to_string(self).expect("Failed to serialize config.");
                fs::write(config_path.with_extension("yaml"), yaml_string)?;
            }
        }
        Ok(())
    }

    fn get_selectors_by_language(&self, language: Option<&'static str>) -> &Vec<Selector> {
        match language.unwrap_or(self.default_language.as_str()) {
            "en" => &self.en,
            "jp" => &self.jp,
            lang => self
                .other_languages
                .get(lang)
                .die(&format!("Selector not found with name `{}`", lang)),
        }
    }

    /// get selectors by its name
    pub fn get_selector_by_name(
        &self,
        language: Option<&'static str>,
        selector_name: Option<String>,
    ) -> &Selector {
        let selectors = self.get_selectors_by_language(language);
        let first_selector = || {
            selectors
                .iter()
                .next()
                .die(&format!("no selector found in `{}`", self.default_language))
        };
        if selector_name.is_none() {
            return first_selector();
        }
        for selector in selectors.iter() {
            if selector.name.starts_with(selector_name.as_ref().unwrap()) {
                return selector;
            }
        }
        eprintln!(
            "Selector `{}` not found, use default selector...",
            selector_name.unwrap()
        );
        print!(
            "Hint: all available selectors in language `{}`: ",
            language.unwrap_or(self.default_language.as_str())
        );
        for selector in selectors.iter() {
            print!("`{}` ", selector.name.green());
        }
        println!();
        first_selector()
    }

    pub fn show_all_selectors(&self) {
        let show_selectors = |selectors: &Vec<Selector>| {
            for selector in selectors.iter() {
                print!("`{}` ", selector.name.green());
            }
            println!();
        };
        println!("All Available selectors:");
        print!("en: ");
        show_selectors(&self.en);
        print!("jp: ");
        show_selectors(&self.jp);
        for pair in self.other_languages.iter() {
            print!("{}: ", pair.0);
            show_selectors(pair.1);
        }
    }
}

impl Default for Config {
    fn default() -> Self {
        Config {
            version: std::env!("CARGO_PKG_VERSION").to_string(),
            default_language: "en".to_string(),
            color: Word::new("red", "blue", "green", "yellow"),
            en: vec![
                Selector::new(
                    "cambridge-zh",
                    RealSelectorString::new(
                        ".pr.entry-body__el:nth-of-type(1) .di-title",
                        ".pr.entry-body__el:nth-of-type(1) .pos-header.dpos-h .region.dreg,.pr.entry-body__el:nth-of-type(1) .pos-header.dpos-h .pron.dpron",
                        ".def.ddef_d.db, .def-body.ddef_b > .trans.dtrans.dtrans-se.break-cj",
                        ".examp.dexamp",
                    ),
                    "https://dictionary.cambridge.org/dictionary/english-chinese-simplified/{}",
                    Delimiter {
                        definition: "\n".into(),
                        ..Default::default()
                    },
                ),
                Selector::new(
                    "oxford",
                    RealSelectorString::new(
                        ".webtop .headword",
                        ".phons_n_am .phon",
                        ".def",
                        ".examples",
                    ),
                    "https://www.oxfordlearnersdictionaries.com/search/english/?q={}",
                    Default::default(),
                ),
                Selector::new(
                    "bing-cn",
                    RealSelectorString::new(
                        "#headword",
                        ".hd_prUS.b_primtxt, .hd_pr.b_primtxt",
                        ".qdef > ul > li > span",
                        ".se_li1 > .sen_en, .se_li1 > .sen_cn",
                    ),
                    "https://cn.bing.com/dict/search?q={}",
                    Delimiter {
                        definition: " ".into(),
                        ..Default::default()
                    },
                ),
            ],
            jp: vec![
                Selector::new(
                    "weblio",
                    RealSelectorString::new(
                        "NULL",
                        ".Sgkdj > p:nth-of-type(1)",
                        ".Sgkdj > p:nth-of-type(2)",
                        ".Wnryj > ul > li",
                    ),
                    "https://www.weblio.jp/content/{}",
                    Delimiter {
                        definition: "\n".into(),
                        ..Default::default()
                    },
                ),
                Selector::new(
                    "cambridge-en",
                    RealSelectorString::new(
                        ".pr.dictionary .tw-bw.dhw.dpos-h_hw.di-title",
                        ".pr.dictionary .var.dvar .v.dv.lmr-0",
                        ".pr.dictionary .def.ddef_d.db, .def-body.ddef_b.ddef_b-t > .trans.dtrans",
                        ".pr.dictionary .examp.dexamp",
                    ),
                    "https://dictionary.cambridge.org/dictionary/japanese-english/{}",
                    Default::default(),
                ),
                Selector::new(
                    "dict-asia",
                    RealSelectorString::new(
                        "#jp_Resunt_panel #jp_comment:nth-child(1) .jpword",
                        "#jp_Resunt_panel #jp_comment:nth-child(1) #kana_0",
                        "#jp_Resunt_panel #jp_comment:nth-child(1) #comment_0",
                        "NULL", // This dict is not so good to seperate definition and example.
                    ),
                    "https://dict.asia/jc/{}",
                    Default::default(),
                ),
            ],
            other_languages: HashMap::default(),
            delimiter_between_paragraphs: "\n".into(),
            delimiter_between_words: "\n\n".into()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_get_selectors_by_language() {
        let config = Config::default();
        let selectors = config.get_selectors_by_language(None);
        assert!(selectors.iter().any(|x| x.name == "oxford"));
    }

    #[test]
    fn test_get_selector_by_name() {
        let config = Config::default();
        let selector = config.get_selector_by_name(None, Some("cam".into()));
        assert_eq!(selector.name, "cambridge-zh");
        let selector = config.get_selector_by_name(Some("jp"), Some("cam".into()));
        assert_eq!(selector.name, "cambridge-en");
    }
}
