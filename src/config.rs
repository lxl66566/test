use crate::core::{selector::Selector, word::Word};
use anyhow::Result;
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
        fs::create_dir(&path).expect("Failed to create `.config` directory");
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
        let toml_string = toml::to_string_pretty(self)?;
        fs::write(config_path, toml_string)?;
        Ok(())
    }

    pub fn get_selectors_by_language(&self, language: Option<&'static str>) -> &Vec<Selector> {
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
                .die(&format!("no selector found in {}", self.default_language))
        };
        if selector_name.is_none() {
            return first_selector();
        }
        for selector in selectors.iter() {
            if selector_name.as_ref().unwrap() == &selector.name {
                return selector;
            }
        }
        first_selector()
    }
}

impl Default for Config {
    fn default() -> Self {
        Config {
            version: std::env!("CARGO_PKG_VERSION").to_string(),
            default_language: "en".to_string(),
            color: Word::new("red", "blue", "green", "yellow"),
            en: vec![], // Selector::new("weblio", Word ,url, field)
            jp: vec![],
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
    }
}
