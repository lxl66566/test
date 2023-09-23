// pub mod cambridge_en_zh;

pub struct Item {
    word: String,
    lang: Option<&'static str>,
    meaning: Option<String>,
}
pub trait Process {
    fn process(content: String) -> anyhow::Result<String>;
    fn url(&self) -> String;
}

impl Item {
    pub fn new(word: String, lang: Option<&'static str>, meaning: Option<String>) -> Self {
        Self {
            word,
            lang,
            meaning,
        }
    }
    fn exec() -> Option<String> {
        // TODO
        Some("".to_string())
    }
}
