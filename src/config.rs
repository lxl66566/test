use serde::{Deserialize, Serialize};

use crate::core::{selector::Selector, word::Word};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Config {
    default_language: String,
    color: Word,
    en: Vec<Selector>,
    jp: Vec<Selector>,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            default_language: "en".to_string(),
            color: Word::all("white"),
            en: vec![],
            jp: vec![],
        }
    }
}
