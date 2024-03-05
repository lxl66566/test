use crate::core::{
    selector::{RealSelectorString, Selector},
    word::{Delimiter, Word},
};
use anyhow::Result;
use colored::Colorize;
use die_exit::*;
use home::home_dir;
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use std::fs;

lazy_static! {
    pub static ref CONFIG: Config = Config::load();
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Config {
    pub version: String,
    pub default_language: String,
    pub color: Word,
    pub en: Vec<Selector>,
    pub jp: Vec<Selector>,
}

impl Config {
    /// get wordinfo's config file path
    pub fn config_path() -> std::path::PathBuf {
        let mut path = home_dir().die("Failed to get home directory");
        path.push(".config");
        path.push("wordinfo.toml");
        path
    }

    /// get config, otherwise use default value
    pub fn load() -> Config {
        fn raise() -> Result<Config> {
            let content = fs::read_to_string(Config::config_path())?;
            Ok(toml::from_str(&content)?)
        }
        raise().unwrap_or_default()
    }

    /// save config to config_path
    pub fn save(&self) -> Result<()> {
        let config_path = Config::config_path();
        fs::create_dir_all(
            config_path
                .parent()
                .expect("can not find parent dir in config."),
        )
        .die("Failed to create `.config` directory");
        let toml_string = toml::to_string_pretty(self).expect("Failed to serialize config.");
        fs::write(config_path, toml_string)?;
        Ok(())
    }

    fn get_selectors_by_language(&self, language: Option<&'static str>) -> &Vec<Selector> {
        match language.unwrap_or(self.default_language.as_str()) {
            "en" => &self.en,
            "jp" => &self.jp,
            _ => &self.en,
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
            ],
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
