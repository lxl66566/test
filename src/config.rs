use crate::core::{selector::Selector, word::Word};
use anyhow::Result;
use die_exit::*;
use home::home_dir;
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use std::fs;

lazy_static! {
    static ref CONFIG: Config = Config::load();
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Config {
    default_language: String,
    color: Word,
    en: Vec<Selector>,
    jp: Vec<Selector>,
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

    // save config to config_path
    pub fn save(&self) -> Result<()> {
        let config_path = Config::config_path();
        let toml_string = toml::to_string_pretty(self)?;
        fs::write(&config_path, toml_string)?;
        Ok(())
    }
}

impl Default for Config {
    fn default() -> Self {
        Config {
            default_language: "en".to_string(),
            color: Word::new("red", "blue", "green", "yellow"),
            en: vec![], // Selector::new("weblio", Word ,url, field)
            jp: vec![],
        }
    }
}
